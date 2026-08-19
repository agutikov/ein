# 05 — The matcher: from dict-copying generators to a register machine

**Settles:** the compiled plan representation and the join execution
model — §O1.
**Phase:** [P1a.3](../p1a.3_deductive_core/README.md) (parity),
[P1a.6](../p1a.6_performance/README.md) (beta-memories).
**Replaces:** `ein/inference/{compile,match,firing,resolve,predicates}.py`.

---

## 1. The 46 %

From the exhaustive-zebra2 profile:

```
5 980 322 calls  match._bind_arg     4.12 s self   6.36 s cum
4 646 381 calls  match._bind_args    3.60 s self  10.69 s cum
31 947 782 calls builtins.isinstance 2.84 s self
1 015 900 calls  match._run_steps    1.28 s self  12.33 s cum
```

Those four functions are the bulk of the 46 % of self time cProfile
attributes to the match/bind subsystem, and most of the 31.9 M
`isinstance` calls are made from inside them. Three costs, all
structural:

1. **A dict copy per bound variable.** `_bind_arg` returns
   `{**bindings, slot.name: arg}` on every successful `Var` bind. That is
   an allocation + rehash of the whole binding set, per variable, per
   candidate fact, at every level of the join.
2. **Type dispatch by `isinstance`.** A slot is `Var | Atom | Int |
   NestedPattern`; an arg is `str | int | Fact`. Six `isinstance` calls
   is a *cheap* path through `_bind_arg`.
3. **Generator recursion.** `_run_steps` is a recursive generator; every
   step allocates a frame and a `rest = tuple(rest_list)` (a fresh tuple
   per call, from `step, *rest_list = steps`).

None of this is Python being slow; it is the data model
([03](03_data_model.md)) making the fast path impossible. Fix the model
and the matcher becomes a loop over `u32`s.

---

## 2. Compile: from `JoinPlan` to bytecode

`compile.py` already produces the right *shape* — `Scan` / `Join` /
`Guard` / `AbsentGuard`, with `(absent …)` lifted into per-disjunct
`NafGuard`s by `split_naf`, plus `assert_templates` and
`extra_match_plans` for the A13 or/and lowering. ein.rs keeps the
semantics exactly and changes the encoding.

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
  `_candidates` narrow on?". ein.py recomputes this per call by walking
  `step.arg_slots` looking for the first slot with a known value
  ([`match._candidates`](../../../ein.py/src/ein/inference/match.py)).
  The *rule* is static except for one thing — whether a `Reg` slot is
  already bound at that point — and that is also static, because bind
  order is static. So `Probe` is `{ slot_index, source: Const(Value) |
  Reg(Reg) }`, resolved at compile time, and the runtime does one
  index-lookup instead of a scan. **Parity note:** it must pick the same
  slot ein.py picks, including the "nested-`Fact` binding is not keyed,
  keep scanning" and "`NestedPattern` slot, keep scanning" rules.

`bindings_seed` (the activator binding) is materialised into `seed`, so
the runtime never rebuilds a dict. The `CompileError` conditions —
unbound relation head, empty `(absent …)` sub-plan, nested `(or …)`,
activator arity mismatch — port with byte-identical messages; they are
authoring errors users see.

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
  Compare that with `_bind_arg`'s six `isinstance` calls and a dict copy.
- **Emission** is a callback — `FnMut(&MatchCtx) -> ControlFlow<()>` —
  not a materialised `Vec`. The saturator's `_enqueue_binding` runs
  *inside* it, so a match that is a duplicate costs nothing beyond the
  key hash. (ein.py already effectively does this by consuming the
  generator lazily; the callback just removes the generator machinery.)

### The trail is also the binding order

`Provenance.bindings` is `tuple((k, str(v)) for k, v in
bindings.items())` — CPython dict insertion order, i.e. **the order the
matcher first bound each variable**, and it is printed in the trace
([02](02_determinism_and_order.md) §3a). The `trail` is exactly that
order, so rendering walks the trail rather than the register file. Seed
bindings come first (they are inserted first, by `dict(plan.bindings_seed)`),
then body vars in bind order. Pin it with a T3 fixture.

### `premises` order

ein.py appends each consumed fact in step order (`(*premises, fact)`),
and `_seed_steps` deliberately rebuilds the tuple so the seeded fact sits
at *its* step's position — so that provenance from a semi-naive seed is
identical to provenance from a full run. `prems` is written at
`prems[step_ordinal]`, which gives the same result without the splice.

---

## 4. Entry points

Four, mirroring `match.py` one-for-one:

| ein.py | ein.rs | used by |
|---|---|---|
| `run(plan, kb)` | `run(plan, kb, &mut f)` | `Engine.step`, `goal_bindings` |
| `run_guarded(plan, kb)` | `run_guarded` — yields the disjunct's `NafGuard`s with each match | `Saturator._full_match` |
| `run_seeded(plan, fact, kb)` | `run_seeded` | semi-naive delta |
| `run_seeded_guarded(plan, fact, kb)` | `run_seeded_guarded` | `Saturator._seed_match` |
| `run_steps(steps, bindings, …)` | `run_steps` | `World.holds` (guard queries) |

`run_seeded`'s contract is subtle and must be copied exactly: for each
top-level `Scan`/`Join` whose relation equals the new fact's relation,
bind that step to the fact and run *the remaining steps* — seeding at
**each** such step, since the fact may play either role in e.g.
`(R ?a ?b) ∧ (R ?b ?c)`.

`goal_bindings` builds a synthetic `JoinPlan` named `<query>` from the
query's `:goal` pattern and runs it; same in Rust.

---

## 5. Predicates and guards

`predicates.py` is a registry with `eq` and `neq`. It is a *registry*
because ein-lang can, in principle, gain more. ein.rs keeps a
`PredId`-indexed table with the same two entries and the same
`is_predicate` / `names()` surface (`names()` is sorted and reaches
`--help`-adjacent output and `primitives.non_object_names`).

One detail with teeth: **a `Guard`'s args are raw IR nodes, not
compiled slots.** `_compile_premise` emits `Guard(predicate, args=node.args)`
without running `_slot`, so a guard inside an `(absent …)` sees
unsubstituted parameter vars — which is exactly why `split_naf` takes
`seed_vars` and starts every guard's scope from the rule's parameters
(see the docstring in `compile.split_naf`). ein.rs must reproduce the
same asymmetry: guard args resolve against the *runtime* binding
environment, including seeds, not against compile-time substitution.

---

## 6. Optimisations, and which ones are legal

The parity contract splits candidate optimisations cleanly.

### Parity-preserving (allowed, and where the win is)

| change | replaces | expected |
|---|---|---|
| register bindings + trail | dict copy per bind | the bulk of the 46 % |
| `u32` equality | `str.__eq__` + `isinstance` | the 31.9 M `isinstance` |
| explicit cursor loop | recursive generator + tuple rebuild | ~1.3 s self |
| compile-time `Probe` | per-call `_candidates` slot scan | 0.26 s + better locality |
| callback emission | tuple/dict materialisation per match | allocation-free inner loop |
| `_binding_key` as a fixed `[Value; n]` hashed with FxHash | `frozenset((k, _hashable(v)) …)` | 2.7 s cum, 445 k calls |
| plan-local relation `Vec<FactId>` pointer cached per Scan | dict lookup per step | small, free |

`_binding_key` deserves a note: the Python key is
`(rule_name, plan.activator_args, frozenset(bindings.items()))`, and
`activator_args` is the activator's **string args only**
(`tuple(a for a in activator.args if isinstance(a, str))`) while the
*cache* key stringifies **all** args. Two activators differing only in an
`int` arg therefore share a binding key. That is almost certainly
unintended, but it is current behaviour, so ein.rs computes the key from
the same filtered tuple. Flagged as Q-M1a.8.

`engine._hashable` — a defensive shim for `list`/`dict` binding values —
has no reachable caller (bindings hold `str | int | Fact`). ein.rs drops
it; the conformance fuzzer would catch it if that were ever wrong.

### Parity-breaking (rejected here, revisit only with a ledger entry)

| change | why it breaks parity |
|---|---|
| **join reordering** (cost-based or first-fail) | changes the *order* matches are produced → firing order → the trace. Legal only if the reorder is proven to preserve enumeration order, which reordering by definition does not. |
| **worst-case-optimal join** (Leapfrog Triejoin / Generic Join, [F11 D2](../../followups/f11_deductive_layer_perf.md)) | same reason. Its trigger is *half* met — **not** none: `stdlib/slots.ein`'s `slot-adjacent-fwd` has the triangle `p1 — PT — p2 — p1`, so "ein rule bodies are acyclic chains/stars" was wrong (S1a.6.3's re-check, 2026-08-19). What is not met is the cost half: those relations hold 30 and 16 facts, and matching is 37.7 % of a 78 ms run. Revisit when a cyclic body is *hot*. |
| **deduplicating matches inside the matcher** | the saturator's `_seen`/`_fired` sets already do it, and their counters are observable. |
| **skipping the re-check of already-probed slots** | `_bind_args` re-checks *every* slot even after `_candidates` narrowed on one; the narrowing is documented as behaviour-preserving precisely because of that. Dropping the re-check would be equivalent only if the index and the unifier agreed on equality — they do — but the invariant is cheap to keep and expensive to lose. Measure before touching. |

---

## 7. Beta-memories (P1a.6, [F11](../../followups/f11_deductive_layer_perf.md) D1)

The one *named* remaining lever, and the port is its promotion trigger.

**What.** Persist partial joins: for a plan and a prefix of its steps,
materialise the binding tuples that satisfy the prefix, and extend them
incrementally as facts arrive — RETE's beta-memory, the rung above the
semi-naive seeded delta join (D5) the engine already has. The
participation index is already the alpha-memory.

**The objection F11 records:** "a beta-memory is per-KB state, and this
engine forks KBs constantly; a memory that must be copied per fork can
lose more than it saves."

**The answer this port makes available.** [03](03_data_model.md)'s KB is
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
  this parity-safe at all;
- a fork is dropped wholesale, so there is no invalidation problem: the
  root memory is never invalidated within a solve (append-only), and the
  delta memory dies with the fork.

**Gate.** It ships only if it is (a) T2-green — identical event traces —
and (b) measurably better on both zebra2 and zebra. If it is a wash, it
is reverted, exactly as P1.8a's D3 cross-fork carry was.

> **The gate ran on 2026-08-19 and the memory was not built**
> ([S1a.6.3](../p1a.6_performance/s1a.6.3_beta_memories.md)). Two cheaper
> changes removed what it was for, and both are in this section's own terms:
>
> - the **alpha**-memory was not narrowing anything. `index_fact` keyed only
>   non-nested arguments, so a `(not (R ?b ?i))` premise — most of what the
>   corpus scans — walked a 368-fact extent. Keying **one level in** took
>   `zebra -e`'s candidates from 25.16 M to **1.17 M** and the run from 349 to
>   78 ms. The intermediate a beta-memory would materialise went from **47.4
>   tuples per step entered to 2.21**;
> - the **root memory** shape was built and measured — as a flat per-relation
>   extent, cloned per fork ([T1a.6.2.5](../p1a.6_performance/s1a.6.2_memory_layout.md)) —
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
>   ([baseline.md § 13](../p1a.6_performance/baseline.md#t1a622-and-t1a626--the-candidate-loop-and-the-two-tasks-that-swapped-places)).
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

- T2 event-trace parity on every `examples/saturation/**`,
  `examples/features/**` fixture — the firing sequence is the matcher's
  signature.
- A microbenchmark harness (`criterion`) over `match::run` on the
  saturated zebra2 root, reported against the ein.py call counts
  (6.0 M `_bind_arg`, 4.6 M `_bind_args`) so the *work* is comparable,
  not just the wall-clock.
- Zero heap allocations in the inner loop, asserted with a counting
  allocator in a test build.
- `MAX_REGS` overflow (a rule with more than N vars) is a clean
  `CompileError`, not a panic — with a fixture.

## Cross-links

- [02 — Determinism & order](02_determinism_and_order.md) §3a — the
  order obligations named above.
- [03 — Data model](03_data_model.md) — `Value`, `FactId`, the extents.
- [06 — Saturation](06_saturation.md) — the caller, and where the
  delta comes from.
- [`architecture_and_algorithms.md` §O1](../../../docs/kernel/inference/architecture_and_algorithms.md)
  — RETE / TREAT / WCOJ context and the SOTA framing.
- [F11](../../followups/f11_deductive_layer_perf.md) — D1/D2, absorbed by
  [P1a.6](../p1a.6_performance/README.md).
