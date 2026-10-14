# 05 — The matcher: a register machine

**Settles:** the compiled plan representation and the join execution
model — §O1.
**Phase:** [P1a.3](../README.md#p1a3--deductive-core),
[P1a.6](../README.md#p1a6--performance) (beta-memories).

---

## 1. Three structural costs

A matcher over a dynamically typed binding set pays three costs that are
structural rather than incidental:

1. **A binding-set copy per bound variable** — an allocation + rehash of
   the whole binding set, per variable, per candidate fact, at every
   level of the join.
2. **Type dispatch at runtime.** A slot is `Var | Atom | Int |
   NestedPattern`; an arg is a symbol, an `int` or a `Fact`; every
   unification asks which.
3. **Recursion** — a frame and a fresh tuple of the remaining steps per
   step entered.

None of this is the language being slow; it is the data model
([03](03_data_model.md)) making the fast path impossible. Fix the model
and the matcher becomes a loop over `u32`s.

---

## 2. Compile: from `JoinPlan` to bytecode

The compiled plan has the right *shape* — `Scan` / `Join` / `Guard` /
`AbsentGuard`, with `(absent …)` lifted into per-disjunct `NafGuard`s by
`split_naf`, plus `assert_templates` and `extra_match_plans` for the A13
or/and lowering. The semantics stay exactly; the encoding changes.

```rust
pub struct PlanId(u32);

pub struct Plan {
    rule:        Symbol,
    activator:   Box<[Symbol]>,      // plan.activator_args (string args only!)
    n_regs:      u16,                // vars numbered at compile time
    seed:        Box<[(Reg, Value)]>,// bindings_seed from the activator
    disjuncts:   Box<[Disjunct]>,    // [0] is `steps`, rest are extra_match_plans
    asserts:     Box<[Template]>,
    why:         Symbol,
}

pub struct Disjunct { steps: Range<u32>, guards: Range<u32> }

pub enum Step {
    Scan  { rel: Symbol, slots: Range<u32>, probe: Option<Probe> },
    Join  { rel: Symbol, slots: Range<u32>, probe: Option<Probe> },
    Guard { pred: PredId, args: Range<u32> },
    Absent{ sub: Range<u32> },       // only ever *nested* inside a NafGuard
}

pub enum Slot {
    Reg(Reg),            // a variable: bind-if-free, else compare
    Const(Value),        // Atom / Int literal, or a var the activator bound
    Nested { rel: Symbol, slots: Range<u32> },
}
```

Two compile-time additions, both pure metadata (no semantic change):

- **Register numbering.** Every distinct `Var` name in a disjunct gets a
  `Reg`. The name is kept in a side table for provenance rendering. The
  same variable in two disjuncts may get different registers; that is
  fine because a match never spans disjuncts.
- **`Probe`** — the precomputed answer to "which arg slot does
  candidate selection narrow on?": the first slot with a known value.
  The *rule* is static except for one thing — whether a `Reg` slot is
  already bound at that point — and that is also static, because bind
  order is static. So `Probe` is `{ slot_index, source: Const(Value) |
  Reg(Reg) }`, resolved at compile time, and the runtime does one
  index-lookup instead of a scan. **The rule, exactly:** a nested-`Fact`
  binding is not keyed, keep scanning; a `NestedPattern` slot, keep
  scanning.

`bindings_seed` (the activator binding) is materialised into `seed`, so
the runtime never rebuilds a binding set. The `CompileError` conditions —
unbound relation head, empty `(absent …)` sub-plan, nested `(or …)`,
activator arity mismatch — have fixed messages; they are authoring errors
users see.

---

## 3. Execute: registers + a trail, no recursion

```rust
pub struct MatchCtx<'a> {
    kb:      &'a Kb,
    regs:    [Value; MAX_REGS],      // Value::UNBOUND = u32::MAX
    trail:   SmallVec<[Reg; 16]>,    // bind order — see below
    prems:   SmallVec<[FactId; 8]>,
    cursors: SmallVec<[Cursor; 8]>,  // one per Scan/Join step
}
```

The driver is an explicit loop over a step index with a per-step cursor:

```
i = 0
loop {
    match steps[i] {
        Scan|Join => advance cursor[i] through candidates;
                     on a candidate that unifies -> push trail, i += 1
                     on exhaustion -> unwind trail to step i-1's mark, i -= 1
        Guard     => evaluate; pass -> i += 1, fail -> backtrack
        Absent    => run the sub-plan to first match; none -> i += 1
    }
    if i == steps.len() { emit(); backtrack }
}
```

- **Binding** is `regs[r] = v` plus `trail.push(r)`; **unbinding** is
  popping the trail back to the step's mark and writing `UNBOUND`. No
  allocation anywhere in the inner loop.
- **Unification** of a `Slot` against a `Value` is: `Const` → `u32`
  equality; `Reg` → `UNBOUND ? bind : u32` equality; `Nested` → the arg
  must be `Value::fact(id)` and the nested pattern must unify against
  that fact's row (one level of recursion, bounded by pattern depth).
- **Emission** is a callback — `FnMut(&MatchCtx) -> ControlFlow<()>` —
  not a materialised `Vec`. The saturator's `enqueue_binding` runs
  *inside* it, so a match that is a duplicate costs nothing beyond the
  key hash.

### The trail is also the binding order

`Provenance.bindings` is in **the order the matcher first bound each
variable**, and it is printed in the trace
([02](02_determinism_and_order.md) §3a). The `trail` is exactly that
order, so rendering walks the trail rather than the register file. Seed
bindings come first, then body vars in bind order. Pinned by a golden.

### `premises` order

Premises are in step order, and a semi-naive seed puts the seeded fact
at *its* step's position — so that provenance from a seed is identical
to provenance from a full run. `prems` is written at
`prems[step_ordinal]`, which gives that without a splice.

---

## 4. Entry points

| entry | used by |
|---|---|
| `run(plan, kb, &mut f)` | the saturator's `step`, `goal_bindings` |
| `run_guarded` — yields the disjunct's `NafGuard`s with each match | the saturator's full match |
| `run_seeded` | semi-naive delta |
| `run_seeded_guarded` | the saturator's seeded match |
| `run_steps` | `holds` (guard queries) |

`run_seeded`'s contract is subtle: for each
top-level `Scan`/`Join` whose relation equals the new fact's relation,
bind that step to the fact and run *the remaining steps* — seeding at
**each** such step, since the fact may play either role in e.g.
`(R ?a ?b) ∧ (R ?b ?c)`.

`goal_bindings` builds a synthetic `JoinPlan` named `<query>` from the
query's `:goal` pattern and runs it.

---

## 5. Predicates and guards

The predicates are a registry with `eq` and `neq` — a *registry* because
ein-lang can, in principle, gain more: a `PredId`-indexed table with an
`is_predicate` / `names()` surface (`names()` is sorted and reaches
`--help`-adjacent output and the set of reserved non-object names).

One detail with teeth: **a `Guard`'s args are raw IR nodes, not
compiled slots.** A premise compiles to a guard holding its arguments as
written, with no slot compilation, so a guard inside an `(absent …)`
sees unsubstituted parameter vars — which is exactly why `split_naf`
takes `seed_vars` and starts every guard's scope from the rule's
parameters. The asymmetry is the rule: guard args resolve against the
*runtime* binding environment, including seeds, not against compile-time
substitution.

---

## 6. Optimisations, and which ones are legal

The order contract ([02](02_determinism_and_order.md)) splits candidate
optimisations cleanly.

### Order-preserving (allowed, and where the win is)

| change | removes |
|---|---|
| register bindings + trail | a binding-set copy per bind |
| `u32` equality | string compares and runtime type dispatch |
| explicit cursor loop | recursion + a tuple rebuild per step |
| compile-time `Probe` | a per-call slot scan |
| callback emission | materialising each match — the inner loop allocates nothing |
| `binding_key` as a fixed `[Value; n]` hashed with FxHash | hashing a set of pairs |
| plan-local relation `Vec<FactId>` pointer cached per Scan | a lookup per step |
| **a fully bound premise resolved by one interned lookup** ([T1a.6.12.3](../README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot)) | fetching the participation bucket and unifying all of it — measured: candidates **1 172 870 → 238 567** on `zebra -e`, −20.6 % end-to-end |

The last row is the one the order argument has to be stated for, because it
does not merely *narrow* the candidate stream — it replaces it. A premise every
one of whose slots is bound denotes exactly one argument tuple; `unify` accepts
a fact iff its arguments **are** that tuple; and the fact store interns
propositions, so no two facts share one. The scan therefore emits the same
one-or-none sequence with the same `FactId`, and a nested pattern naming a
proposition that was never interned emits nothing without a lookup at all. It
is the same argument the probe narrowing rests on (unification re-checks every
slot), taken to its limit: when the re-check is the *whole* test, the search
around it is redundant.

Its price is one slot inspection per premise on the path where nothing is
bound — `+0.9 %` on `branching/07 -e`, which has no ground premise anywhere,
and +4.0 % on the `match_hot` micro-bench.

`binding_key` deserves a note: the key is `(rule, the activator's symbol
args, the bindings)`, and its activator component holds the activator's
**symbol args only**, while the plan *cache* key covers **all** args. Two
activators differing only in an `int` arg therefore share a binding key.
Flagged as Q-M1a.8.

> **The last sentence of that inference is wrong**
> (2026-08-29, M1e
> [S1e.1.4](../../m1e_review_processing/README.md#s1e14--q-m1a8s-real-trigger-q3)).
> The key's *third* component carries the parameter bindings, and
> `compile_rule` binds a parameter for every argument that is a symbol or an
> `int` — so an `int` is in the key after all and two activators differing
> only there both fire. What reaches neither component is a nested `Fact`.
> The key above is unaffected. What changed is what the asymmetry costs, which
> is [`defined_behaviour.md` §3.2](../../../kernel/defined_behaviour.md#32-a-rule-applications-identity-ignores-nested-fact-activator-arguments)
> and
> [Q-M1e.16](../../m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one).

### Order-breaking (rejected here)

| change | why it breaks order |
|---|---|
| **join reordering** (cost-based or first-fail) | changes the *order* matches are produced → firing order → the trace. Legal only if the reorder is proven to preserve enumeration order, which reordering by definition does not. |
| **worst-case-optimal join** (Leapfrog Triejoin / Generic Join, [F11 D2](../../../../plans/followups/f11_deductive_layer_perf.md)) | same reason. Its trigger is *half* met — **not** none: `stdlib/slots.ein`'s `slot-adjacent-fwd` has the triangle `p1 — PT — p2 — p1`, so "ein rule bodies are acyclic chains/stars" was wrong (S1a.6.3's re-check, 2026-08-19). What is not met is the cost half: those relations hold 30 and 16 facts, and matching is 37.7 % of a 78 ms run. Revisit when a cyclic body is *hot*. |
| **deduplicating matches inside the matcher** | the saturator's `seen`/`fired` sets already do it, and their counters are observable. |
| **skipping the re-check of already-probed slots** | unification re-checks *every* slot even after candidate selection narrowed on one; the narrowing is documented as behaviour-preserving precisely because of that. Dropping the re-check would be equivalent only if the index and the unifier agreed on equality — they do — but the invariant is cheap to keep and expensive to lose. Measure before touching. |

---

## 7. Beta-memories (P1a.6, [F11](../../../../plans/followups/f11_deductive_layer_perf.md) D1)

The one *named* remaining lever, and M1a is its promotion trigger.

**What.** Persist partial joins: for a plan and a prefix of its steps,
materialise the binding tuples that satisfy the prefix, and extend them
incrementally as facts arrive — RETE's beta-memory, the rung above the
semi-naive seeded delta join (D5) the engine already has. The
participation index is already the alpha-memory.

**The objection F11 records:** "a beta-memory is per-KB state, and this
engine forks KBs constantly; a memory that must be copied per fork can
lose more than it saves."

**The answer the data model makes available.** [03](03_data_model.md)'s KB is
`Arc<KbCore> + Delta`. So the memory splits the same way:

- the **root** memory is built once during root saturation, lives in
  `KbCore`, and is shared read-only by every fork (`Arc`, no copy, no
  lock);
- a fork holds a **delta memory**: only the partial joins that involve at
  least one fork-local fact. On zebra2 a fork adds tens of facts to a
  base of ~380, so the delta memory is one to two orders of magnitude
  smaller than the root one;
- enumeration walks root-then-delta, which is the same order the layered
  extents give — so **match order is preserved**, which is what makes
  this order-safe at all;
- a fork is dropped wholesale, so there is no invalidation problem: the
  root memory is never invalidated within a solve (append-only), and the
  delta memory dies with the fork.

**Gate.** It ships only if it has (a) identical event traces and (b) is
measurably better on both zebra2 and zebra. If it is a wash, it
is reverted, exactly as P1.8a's D3 cross-fork carry was.

> **The gate ran on 2026-08-19 and the memory was not built**
> ([S1a.6.3](../README.md#s1a63--beta-memories-f11-d1)). Two cheaper
> changes removed what it was for, and both are in this section's own terms:
>
> - the **alpha**-memory was not narrowing anything. `index_fact` keyed only
>   non-nested arguments, so a `(not (R ?b ?i))` premise — most of what the
>   corpus scans — walked a 368-fact extent. Keying **one level in** took
>   `zebra -e`'s candidates from 25.16 M to **1.17 M** and the run from 349 to
>   78 ms. The intermediate a beta-memory would materialise went from **47.4
>   tuples per step entered to 2.21**;
> - the **root memory** shape was built and measured — as a flat per-relation
>   extent, cloned per fork ([T1a.6.2.5](../README.md#s1a62--memory-layout)) —
>   and cost **7.6 %** on the search while making the fork-free bench 8 %
>   faster. The `Arc`-shared half of the design above is not an optimisation
>   over the copied half; it is the only half that pays.
>
> The section stays as the design a memory would follow. What changed is the
> premise: it is no longer the largest lever (Q-M1a.10 → *no*), and its
> promotion now needs a workload whose per-step candidate count is large
> again.

> **What S1a.6.2 measured for this stage (2026-08-19).** Three numbers, and
> the third is a design constraint rather than a target:
>
> - **the join is 84.8 % of `solve zebra.ein -e`** — `unify` 49.3 %,
>   `try_candidate` 16.7 %, `walk` 9.4 % — after the allocator and the fact
>   store stopped being anything;
> - **25.16 M candidates, and 99.1 % of them come from a full extent scan**
>   of a 368-fact extent, at ~2 slot unifications each
>   ([baseline.md § 13](../measurements/baseline.md#t1a622-and-t1a626--the-candidate-loop-and-the-two-tasks-that-swapped-places)).
>   The alpha-memory is *not* doing the work this section assumes it is:
>   `index_fact` keys only non-nested arguments, so a `(not (R …))` premise —
>   most of what the corpus scans — never narrows at all. Keying inside a
>   nested argument is the cheapest form of this whole idea and should be
>   priced before the beta-memory proper;
> - **do not give a fork its own copy of anything it could read from root.**
>   T1a.6.2.5 built the flat per-relation extent this section's "root memory"
>   is shaped like, but *cloned per fork* instead of shared: 8 % faster on
>   `match_hot`, **7.6 % slower on the search**. The `Arc`-shared root memory
>   above is not merely convenient, it is the half that pays; a per-fork
>   materialisation of the same data is measurably negative.

**Ordering caveat to design against.** A memory that stores prefix
tuples in *discovery* order and appends on delta reproduces the
enumeration order of a full re-run only if the outer loop consumes the
memory in the same order the nested loops would have produced it. For a
left-deep plan with per-step append-ordered extents, it does. Prove it in
the stage doc with a small argument plus a randomised differential test,
rather than assuming it.

---

## 8. Acceptance for this design

- A microbenchmark harness (`criterion`) over `match::run` on the
  saturated zebra2 root, reported beside the work counters so the *work*
  is visible, not just the wall-clock.
- Zero heap allocations in the inner loop, asserted with a counting
  allocator in a test build.
- `MAX_REGS` overflow (a `:match` binding more than 256 distinct
  variables) is a clean `CompileError`, not a panic — with a fixture.

## Cross-links

- [02 — Determinism & order](02_determinism_and_order.md) §3a — the
  order obligations named above.
- [03 — Data model](03_data_model.md) — `Value`, `FactId`, the extents.
- [06 — Saturation](06_saturation.md) — the caller, and where the
  delta comes from.
- [`architecture_and_algorithms.md` §O1](../../../kernel/inference/architecture_and_algorithms.md)
  — RETE / TREAT / WCOJ context and the SOTA framing.
- [F11](../../../../plans/followups/f11_deductive_layer_perf.md) — D1/D2, absorbed by
  [P1a.6](../README.md#p1a6--performance).
