# Inference — the rule firing engine

> **Status (2026-06-16).** The engine has **shipped** — P1.3
> (saturation / rules), P1.4 (contradiction), P1.5–P1.5b (hypothesis
> loop + commitment-lattice search) are all in place and `zebra2`
> solves end-to-end. The **as-built** architecture reference is
> [`architecture_and_algorithms.md`](architecture_and_algorithms.md);
> this file holds the design principles, the M1 invariant, NAF
> semantics, and determinism. The "as-built layout" + "what's
> implemented" sections were reconciled by
> P1.20 S1.20.A0;
> the module-level walkthrough is
> [`implementation.md`](implementation.md); the "Determinism" and
> "Superseded tree-solver" sections lower down describe a solver that was
> removed, and say so in their own banners.

The inference engine is what takes a populated
[`KnowledgeBase`](../ir/02-data-model/02_store.md) and produces
**derived facts** by firing
[rules](../ir/01-ein-graph/02_rules.md). Everything else in the
kernel tree describes *what* the engine reads and writes; this
chapter describes *how* it does it.

> **Architecture + algorithms overview.** For the engine's *as-built*
> architecture and main steps, the abstract operations it performs, their
> analogs in other CS fields (Datalog / RETE / CDCL / ATMS / e-graphs), and
> the fast/optimal known algorithms for each, see
> [`architecture_and_algorithms.md`](architecture_and_algorithms.md) — the
> overview the planned chapters below sit under.

---

## As-built layout

The engine shipped across P1.3–P1.5b; the planned `01_matcher.md …
05_trace.md` split was never created — the directory instead grew
these reference docs:

```text
docs/kernel/inference/
├── README.md                      ← this file: design principles, the
│                                     M1 invariant, NAF semantics, determinism
├── architecture_and_algorithms.md ← the as-built architecture: the 9 core
│                                     operations (O1–O9), their CS analogs
│                                     (Datalog / RETE / CDCL / ATMS), fast algos
├── absent_semantics.md            ← normative `(absent P)` semantics: worlds,
│                                     boundary epistemic NAF, corollaries
│                                     C1–C7, non-guarantees (P1.21 R4,
│                                     re-grounded by S1.21.8)
├── domain_elim_vs_hypothesis.md   ← the domain-elimination vs guess duals
├── lattice_dump.md                ← the commitment-lattice dump format
├── events.md                      ← the `--events` protocol: one JSON object
│                                     per line, every step the engine took
│                                     (schema `ein-events/1`; M1a S1a.0.2,
│                                     re-homed here at S1a.10.3)
├── solution_semantics.md          ← normative: hypothesis / commitment /
│                                     entering / solution / model, and where
│                                     the engine's `complete()` differs from
│                                     the definition (M1e, 2026-08-28)
├── reserved_engine_strings.md     ← engine-internal reserved atoms
│                                     (__closed__, __symmetric__, false, …)
├── implementation.md              ← the module-by-module engine map
├── ../defined_behaviour.md        ← the behaviours the Python source used to
│                                     be the only statement of (M1a S1a.10.6)
└── features.md                    ← measured feature×config matrix: which
                                      SolverConfig knobs are load-bearing (S1.20.I)
```

Source for the engine lives under
[`ein-infer`](../../../ein.rs/crates/ein-infer/src/); the
module-by-module walkthrough is [`implementation.md`](implementation.md).

## What's implemented (M1)

The engine shipped end-to-end; `examples/zebra2.ein` solves with a full
derivation trace. Where each piece lives (`ein-infer/src/` unless noted):

| concept                         | M1 state                                                         |
|---------------------------------|------------------------------------------------------------------|
| Pattern matcher                 | **shipped** — `compile.rs` lowers each (rule, activator) to a `JoinPlan` whose steps are **purely positive** (S1.21.8: `split_naf` lifts every `(absent …)` into `plan.naf_guards`); `match_.rs` `_run_steps` executes it, `run_guarded` tags each match with its own disjunct's guards |
| Rule registry                   | `Rule` entity in [`02-data-model`](../ir/02-data-model/); puzzle rules authored inline or imported from the [stdlib](../../../stdlib/) |
| Property-fact activation        | KB indexes `_rule_apps_by_rule` / `_rule_apps_on_relation` built at load |
| Saturation loop                 | **shipped** — priority-banded, delta-driven `saturator.rs` (P1.3 S1.3.3; semi-naive in P1.8a); **two-phase since S1.21.8** — a purely positive closure runs to quiescence, then one boundary round admits at most one parked NAF-guarded candidate and the closure re-runs |
| Hypothesis branching            | **shipped** — `hypgen.rs` enumerates candidates; the commitment-lattice search is `solve.rs` (P1.5–P1.5b) |
| Contradiction detection         | **shipped** — `contradiction.rs` detects (`(X, ¬X)` pairs + `(false)`); `explain.rs` → `explain.rs` explains: smallest contradiction frontier — a minimum-cardinality AND/OR search over every recorded derivation (provenance-based, NAF-safe, budgeted); **not** a subset-minimal MUS |
| Multi-justification provenance  | **shipped** — provenance is per *derivation*: `Fact.provenance` is the primary justification, `kb.justifications(fact)` returns every recorded one (capped per fact, shortest kept), so a fact is an OR-node over AND-nodes. Gated by `SolverConfig.record_alternative_justifications` (default on) |
| Verdict                         | **shipped** — one `solve()`; `verdict.rs` reports `Solution` / `Ambiguity` / `Contradiction`, read off the model count `k`; the `k = 0` unsat core is that smallest explanation — of the root contradiction directly, or of each dead commitment, unioned across the deads |
| Trace generation                | **shipped** — `DerivationDAG.to_dot()` + the markdown trace builder under [`ein-render/trace/`](../../../ein.rs/crates/ein-render/src/trace/) (P1.6) |
| `(not P)` / `(absent P)` premises | S1.5.8c.1: `(not P)` in `:match` matches a STORED `(not P)` fact (uniform with all other patterns); `(absent P)` is the explicit NAF guard. The old NAF default on `(not P)` was dropped. |
| `(forall ?b (G) (B))` / `(unknown P)` | S1.5.8c.3a/b sugars (now in `std.macro`): `forall` ⇒ `(absent (and G (absent B)))`, `unknown` ⇒ `(and (absent P) (absent (not P)))`. Compile to the `AbsentGuard` machinery; a `forall`'s **nested** absent is not lifted — it stays part of the negative query and the boundary evaluates the whole guard as one unit. |
| `(absent …)` evaluated at the closure/world boundary | **shipped** S1.21.8 — guards are lifted out of the closure plan and judged against the positive fixpoint by [the boundary phase](../../../ein.rs/crates/ein-infer/src/saturator.rs), never mid-saturation. The fire-time re-check (`match.absents_still_pass`) and the absent-flip full-match split (`saturator._absent_relations`) are **deleted**, not bypassed; `Saturator.naf_dropped` is structurally 0. See § "NAF semantics" below. |
| Negative (NAF) provenance       | **shipped** S1.21.8 — [`Prov::absent`](../../../ein.rs/crates/ein-core/src/prov.rs) records the `(absent …)` queries that had to fail for a firing to be admitted (`World.negative_premises`), one `(relation, args)` pattern each with `None` where the query ranged free. This is the missing half of `Deps(Y)` = `PositiveDeps(Y)` ∪ `NegativeDeps(Y)`; it makes negative dependence **visible**, and nothing interprets it yet — `unsat_core` and the trace's "using" line still read positive premises only. |
| Hypothesis-branch order is deterministic | content-based sort, not hash-based (`PYTHONHASHSEED` does not reach iteration order); the score key is an M1 stub. See § "Determinism" below. |

The data substrate (KB, entities, layer views, fork, provenance,
derivation DAG) was complete at P1.2; the engine that *operates* on it
shipped across P1.3–P1.6.

## Design principles (already locked in M1)

1–5 are inherited from the graph + data-model docs and don't
change when the engine arrives; 6 is the one the engine itself
contributes, locked by S1.21.8:

1. **The graph is canonical, the engine is dynamic.**
   [`feedback_graph_canonical`](../../../README.md). The engine
   never replaces the KB; it only appends.
2. **Rules can be higher-order.** Three rule types
   ([`../ir/01-ein-graph/02_rules.md`](../ir/01-ein-graph/02_rules.md));
   the matcher must enumerate relation variables.
3. **Every firing leaves provenance.** Rule-kind provenance with
   `premises_raw` and `bindings` is mandatory; trace fidelity
   ([idea 08](../../../plans/ideas/08-human-style-deductive-trace.md)) is an
   M1 acceptance gate. Provenance is per **derivation**, not per fact:
   `Fact.provenance` is the primary justification and
   `kb.justifications(fact)` returns every recorded one, so a fact is an
   OR-node over AND-nodes — the proof structure is an AND/OR graph
   ([`implementation.md` § cross-cutting invariants](implementation.md), searched by
   [`explain.rs`](../../../ein.rs/crates/ein-infer/src/explain.rs)).
4. **Lazy branching.** Saturate first with all propagation rules;
   branch only when no rule fires and the puzzle is not yet solved.
   ([Q19 working answer](../../../plans/open_questions.md#q19--hypothesis-branching-strategy).)
5. **Encoding-agnostic.** The engine works over both `zebra.ein`
   (one generic `co-located` link, `instance` / `type` membership) and
   `zebra2.ein` (five typed `*-loc` relations, unified `is-a`) —
   *two ontologies for one puzzle*, and since S1.22.1a both solve to the
   same model, which is what makes the claim testable rather than
   aspirational (`ein-cli/tests/acceptance_cli.rs`). P1.7 resolved the
   encoding (`is-a` canonical) and S1.7.23 removed the kernel
   type/instance entity-view, so the engine treats every membership
   relation uniformly as facts — there is no
   `logical_types` / `logical_instances` bridge to consult.
6. **Negation sits on the closure/world boundary.** The closure is
   purely positive and monotone — it consults no negation at all —
   and `(absent P)` is asked once, of a *saturated* world
   ([the boundary phase](../../../ein.rs/crates/ein-infer/src/saturator.rs)),
   never of a half-built KB (S1.21.8; normative contract:
   [`absent_semantics.md`](absent_semantics.md), layering:
   [`architecture.md` §closure/worlds seam](../architecture.md)).
   The consequence rule authors feel: on a **stratified** rule set
   the priority bands order the work but no longer decide what is
   derivable — band discipline is advisory, not load-bearing.

## M1 invariant — alive-set soundness

The solver **recomputes** the alive-candidate set per-KB via
[`compute_alive`](../../../ein.rs/crates/ein-infer/src/solve.rs)
(`= open_hypotheses(kb)`) — the open hypotheses are a pure function of the
closed KB. The *set* is materialised only where a set is what the caller
needs (layer enumeration); the completeness test asks the same generator for
its first element and stops (`solution.complete`, S1.9.E16 — 8 of 9 zebra2
calls are answered by candidate #1, and the peak open set is ~38 fact-ids, so
there is no memory case for streaming the materialised half). (Historical: an *inherit-once* optimization — seed `kb.alive` at
root saturation, let forks inherit it via `kb.fork()`, run
`generate_hypotheses` once per `solve()` — was gated by a
`SolverConfig.enable_alive_inherit` flag, default on since S1.5.4 T1.5.4.8.
P1.7a switched to the per-KB recompute and the flag was **removed 2026-06-15**.)

That "alive is a pure function of the closed KB" property is sound iff three
pre-conditions hold across the puzzle's rule library — collectively the
**M1 invariant**:

1. **No new objects.** Rules don't `:assert` facts whose args
   introduce names that weren't already in the ontology /
   facts. (Q40 nested-Fact args are existing facts, not new
   names.)
2. **No new relations.** Rules don't `:assert (relation N S₀ S₁)`
   declarations — the relation registry is fixed by the ontology
   block.
3. **Hypotheses connect names only.** `_fill_slot` iterates
   `_instance_like_objects` and string-fills both slots; no
   nested-Fact hypothesis args.

Under these clauses, every admissible hypothesis is enumerable
from the current KB state; deeper branches **eliminate** candidates,
never extend the space. The same "alive ⇐ KB" argument licenses the
[`canon::state_key`](../../../ein.rs/crates/ein-infer/src/canon.rs)
KB-only dedup — two KBs with identical facts have identical futures
(P1.21 R1: the key IS the canonical fact set, compared exactly — a
hash of it is display-only, never identity).

**When the invariant breaks** (a rule library asserts new
`(relation …)`; F5 rules-as-data; a future puzzle's matcher
produces nested-Fact hypotheses), "alive is a pure function of the
closed KB" no longer holds, and both the per-KB recompute and the
state-key dedup lose their soundness warrant.

Tracked at
M1 Q-S1.5.4.D
as a long-term design seam; promote to a typed invariant check
when F5 lands.

## NAF semantics — the closure/world boundary (S1.21.8)

> **Normative definition:**
> [`absent_semantics.md`](absent_semantics.md) (P1.21 R4, re-grounded by
> S1.21.8). `absent(P)` is a **query** — "the current fork-local world, at
> the positive quiescence this firing was admitted at, holds no fact
> matching P" — **never a ground atom** that could be stored, cached, or
> carried between worlds. That page states the worlds model, the
> evaluation points (**one**: E1 at the boundary; E2 fire-time **retired**,
> E3 never-after unchanged), the corollaries C1–C7 the engine relies on
> (C2 re-grounded, C4/C5 retired), and what is explicitly *not* provided
> (stratification, stable models, retraction) — each pinned by
> [`naf_semantics.rs`](../../../ein.rs/crates/ein-infer/tests/naf_semantics.rs).
> This section is the operational how.

*Shipped, no longer a target picture:* the **closure/world seam** — a
purely positive closure with NAF on its boundary — was recorded by P1.21 R6
in [`../architecture.md` §closure/worlds seam](../architecture.md) and
implemented on 2026-08-17 by
P1.21 S1.21.8.

**The compile split.** `(absent P)` in a `:match` clause still compiles to
an [`NafGuard`](../../../ein.rs/crates/ein-infer/src/compile.rs) step, but
[`compile.rs`](../../../ein.rs/crates/ein-infer/src/compile.rs) then
lifts every *top-level* one out of the plan. What remains in
`JoinPlan.steps` — and in each S1.8.A13 `extra_match_plans` disjunct — is a
purely positive Scan/Join/Guard plan, the **closure plan**. The guards
become `NafGuard`s in `JoinPlan.naf_guards`, one tuple per disjunct, paired
back up by `JoinPlan.disjuncts()` (`JoinPlan.has_naf` says whether any
exist at all). Each `NafGuard` carries three things beyond the guard:

- **`scope`** — the variables bound by the positive premises that
  *preceded* it. Boundary evaluation projects the completed bindings back
  down to that set
  ([`NafGuard::scope_of`](../../../ein.rs/crates/ein-infer/src/plan.rs)), which is
  what makes lifting exactly as strong as evaluating in place:
  `(and (absent (P ?x)) (Q ?x))` still asks "is there no `P` at all?", and
  `(and (Q ?x) (absent (P ?x)))` still asks "no `P` for *this* `x`?".
- **`watched`** — every relation the negative query reads, nested guards
  included. The boundary's invalidation key (below).
- **`monotone`** — true iff the query contains no nested absent, hence is
  anti-monotone in the KB: once it finds a match it finds one forever.

A **nested** `AbsentGuard` — what `forall` desugars to,
`(absent (and G (absent B)))` — is *not* lifted: it is part of the negative
query, and the boundary evaluates the whole guard as one unit. That is why
`match._run_steps` keeps its `AbsentGuard` arm, now reachable only from
inside a negative query.

**The boundary type.**
[`saturator.rs`](../../../ein.rs/crates/ein-infer/src/saturator.rs) —
`World(kb, commitment=())` with `holds(steps, bindings)`,
`absent(guard, bindings)`, `admits(guards, bindings)`,
`first_failing(guards, bindings)` and `negative_premises(guards, bindings)`
(plus `project` and `root_world`). It is a **read-only view taken at a
quiescence point, not a snapshot** — saturating past that point invalidates
it, so the saturator builds a fresh one per round rather than mutating one.
Every NAF query in the engine goes through it, and nothing else does.

**Two-phase saturation.**
[`Saturator::step`](../../../ein.rs/crates/ein-infer/src/saturator.rs)
alternates:

1. **Closure** (`_closure_step`) — purely positive plans fire to
   quiescence, consulting no negation whatsoever. A match whose disjunct
   carries guards never enters the firing queue: `_enqueue_binding` routes
   it to `_parked` instead.
2. **Boundary** (`_admit_from_boundary`) — at quiescence a `World` is built
   over the stalled KB, the parked candidates are judged against that
   fixpoint in the engine's own (priority, FIFO) order, and the **first**
   whose guards all pass is admitted back into phase 1.

Repeat until a quiescence admits nothing. **One admission per round is the
design, not a throttle:** the queue is empty at quiescence, so the admitted
candidate fires immediately against exactly the world its guard was judged
against — no window, nothing can go stale. Admitting the whole batch would
re-create the race this stage removes, one layer up: on
`p ← absent q; q ← absent p` both guards pass against the world in which
neither holds, so a batch derives **both**, and `{p, q}` is not a model of
that program under any reading. On a *stratified* program the two policies
agree, whatever the rules' priorities.

A rejected candidate stays parked — a `forall`'s nested absent flips from
failing to *passing* as the KB grows — with two economies the naive loop
cannot do without (without them the same design ran ~80 % slower, at 233 k
guard queries per solve):

- a candidate whose *failing* guard is anti-monotone (`monotone`) is
  **retired** outright (`naf_retired`): the KB only grows, so its query
  keeps matching and it can never pass again — it is dead, not waiting;
- otherwise the round stores a `_watch_stamp`, the extent sizes of the
  guard's `watched` relations. An unchanged stamp means no watched relation
  grew, so the verdict cannot have moved and the query is not re-run
  (zebra2 root: 460 parked candidates over 40 rounds).

Observables on the saturator: `naf_rounds`, `naf_admitted`, `naf_retired`,
and `naf_dropped` — kept, and now **structurally 0**. `is_stalled()`
consults the boundary too: a parked candidate whose guards now pass is an
available firing, and answering from the positive queue alone would report
"stalled" one round before a `forall` admits. The queue-less
[`Engine::step`](../../../ein.rs/crates/ein-infer/src/engine.rs) implements
the same two phases directly — every positive match first, the boundary
only once none remain.

**The race, retired.** The old design evaluated guards *inside* the
closure, three times over: at enqueue-time matching, again at fire time to
close the enqueue/fire race (`match.absents_still_pass` →
`Saturator.naf_dropped`), and implicitly through the semi-naive enqueue,
which had to full-match any plan whose delta relation landed inside a guard
(`saturator._absent_relations` + its `_abs_index`). All three are
**deleted, not bypassed**. There is no enqueue/fire race left to close,
because a guard is asked once, of a fixpoint, at the moment its firing is
admitted; and no delta can force a full re-match, because guards do not
participate in matching at all. The `forall` false→true flip the
absent-flip split existed for is caught by re-judging parked candidates at
the boundary — cheaper *and* strictly more complete, since it also catches
a flip with no delta in the watched relation.

zebra2's priority discipline (the `(includes right-of next-to)` +
`(symmetric next-to)` chain at priority 100 draining before the
priority-200 cross-attr rules that NAF over `next-to`) used to be what
*structurally* prevented the race for that shape — and bands ≥ 200 NAF-ing
each other, or a branched saturation starting with a non-empty queue, had
no such protection. The property is now an **engine** property rather than
a ruleset one: the closure is complete before any guard is asked, whatever
the bands say.

**Termination.** Within a single `saturate()` run the fact base still grows
monotonically (no retractions). A `(plan, bindings)` candidate is
`_seen`-deduped when it is parked and recorded in `engine._fired` when it
fires, so it is parked once and admitted at most once; an anti-monotone
rejection retires it outright. The outer loop stops at the first quiescence
that admits nothing.

The retracting flow that *does* exist (hypothesis branching's `kb.fork()`)
takes a fresh saturator over the branch KB, so `_queue`, `_parked`, `_seen`
and the park stamps all start empty and no boundary verdict crosses a
world — corollary C6 made structural.

> **ein.rs does this differently, on purpose** (M1a
> [S1a.6.9](../../history/m1a_rust/README.md#s1a69--the-fork-entry-delta-the-resumed-saturator),
> ledgered as [D3](../../history/m1a_rust/divergences.md)). A fork there
> *resumes* the parent's saturation — the plan list, `fired`, `seen` and the
> parked set with their stamps are inherited, and the delta is the commitment
> — because a fresh saturator's first pass re-derives the parent's entire
> closure, 94.6 % of a fork's firings on `zebra -e`. C6 still holds: the
> parked set is carried *with* its watch stamps, and an inherited candidate is
> re-judged against the fork's own world before it can be admitted, so no
> boundary verdict crosses a world. What differs between the two engines is
> how much of the derivation each narrates, not what either derives — the
> verdict, `k`, the models, the unsat core and every published counter are
> compared exactly and do not move.

**Negative provenance.** A firing admitted through the boundary records
what had to *not* hold:
[`Prov::absent`](../../../ein.rs/crates/ein-core/src/prov.rs) is
a tuple of `(relation, args)` patterns — `None` marking a position the
query left free — built by `World.negative_premises` (nested guards
contribute their patterns too: the whole query is what had to fail) and
handed to [`fire`](../../../ein.rs/crates/ein-infer/src/firing.rs) as a
kwarg. This is the missing half of `Deps(Y)` = `PositiveDeps(Y)` ∪
`NegativeDeps(Y)` (REVIEW_M1-01 §2) — the object whose absence sank the
`unconditional_facts` extraction (below) and keeps deletion-based core
minimisation unsound. **It makes negative dependence visible; no walk
interprets it yet** — `unsat_core`,
[`explain.rs`](../../../ein.rs/crates/ein-infer/src/explain.rs) and the
trace's "using" line still read positive premises only, so
[`absent_semantics.md`](absent_semantics.md) C3 stands as written.

**The one guard evaluation outside the saturator.**
[`Lookahead::dies_immediately`](../../../ein.rs/crates/ein-infer/src/lookahead.rs)
judges a rule's guards in the world **with** the probed candidate `h`: the
guard must find no match in `kb` *and* `h` must not create one (divergence
D3, fixed with this stage). A guard containing a nested absent is
non-monotone and cannot be decided that cheaply, so the lookahead **skips
that disjunct** rather than guessing — which only loses a kill, keeping the
"never reports a live hypothesis as dead" contract.

**What changed semantically**, all pinned in
[`naf_semantics.rs`](../../../ein.rs/crates/ein-infer/tests/naf_semantics.rs):
a rule that used to fire because its watched fact had not been derived
*yet* no longer does (the guard is judged against the fixpoint, in which it
has); the result of a **stratified** program no longer depends on rule
priority at all, which demotes band discipline from load-bearing to
advisory; and a **non-stratified** program is still answered by operational
order — now boundary-admission order rather than priority-then-FIFO — with
the engine reporting one model where several exist and not saying so. A
static stratification checker is the proper remedy and remains future work.

**Measured** (S1.21.8): acceptance 17/17 with verdicts unchanged, 1342 unit
tests, **zero** xfails (D5's `xfail(strict=True)` now passes). Faster, not
slower — exhaustive `zebra2` solve ~10.4 s → ~8.5 s, acceptance gate 130 s
→ 91 s: removing the absent-flip full-match split more than pays for the
boundary evaluations.

**Static NAF dependency map (S1.7.4) — re-grounded, not retired.** The map
used to answer a *soundness* question: the fire-time re-eval made every
derived-NAF rule sound, and this told the author which of their rules
leaned on it (or on a strictly-lower deriving priority). That rationale
died with the re-check. What the derived-vs-declared split still buys is
**stratification** — NAF over a derived relation is exactly the shape that
can make a rule set non-stratified, which is the one thing that can still
go wrong (the engine then picks one model by boundary-admission order and
does not report that others exist), while NAF over a declared-only relation
cannot, its watched extent being fixed by the puzzle. So the map is now the
cheap static proxy for "could this rule set have more than one answer?" —
the input a real stratification checker would refine.
[`naf_deps`](../../../ein.rs/crates/ein-infer/src/naf_deps.rs) answers it
statically:
[`naf_deps::compute_naf_map`](../../../ein.rs/crates/ein-infer/src/engine.rs)
walks the compile cache and returns one `NafDep` per `(rule, activator)`
that carries a guard — since S1.21.8 via `plan.disjuncts()`, so an
or-disjunct's guards are visible to it too (they were not before) —
splitting the watched relations into
`derived` (some rule positively asserts it — or, for an
`(absent (not (R …)))` guard, some rule asserts `(not (R …))`) vs
`declared_only` (extension fixed by the loaded facts — no rule
produces it). The
classification reuses [`compile::asserted_relation`](../../../ein.rs/crates/ein-infer/src/compile.rs)
(the same test behind [`closed::producible_relations`](../../../ein.rs/crates/ein-infer/src/closed.rs))
and its `negated_relation` dual. Because the activator-bound head var
(`?S` in `adjacent-via-*`) is baked to a literal relation per activator,
the split is per-activator: zebra2's `adjacent-via-fwd` is derived-NAF on
its `next-to` activator but declared-only on `right-of` — the same
asymmetry the (now historical) priority-protection note above turned on.
**The map is only complete on a post-initial-saturation
cache** — most NAF-bearing rules (the spatial and
elimination families) are activated by *derived* facts absent at load —
so the warning is emitted once, after `_phase1_root`'s root saturation,
gated by `SolverConfig.warn_derived_naf` (a `DerivedNafWarning`). That
flag defaults **off**: the warning is advisory — a derived-NAF rule is
sound whatever the priorities say now that the guard is judged on the
boundary — and the suite runs under `filterwarnings=["error"]`; it
promotes to load-bearing under S1.7.7,
and stays the only diagnostic for non-stratifiability until a real
checker lands.

**Open follow-ups.**

- Q-S1.5a.1.B
  — caching per-(rule, binding) NAF results and invalidating on
  watched-fact arrival. **Half shipped** as S1.21.8's `_watch_stamp`: a
  parked candidate's guards are re-asked only when one of their `watched`
  relations grew. What is still open is a *shared* verdict cache across
  candidates. (The old companion entry — P1.9 E8, watched-fact rule
  applicability — was closed as superseded by the semi-naive saturation;
  [F9 ledger](../../../plans/followups/f9_e_catalog.md).)
- **Static stratification checking** — the engine accepts unstratifiable
  rule sets, answers them by boundary-admission order, and reports one
  model without saying that others exist
  ([`absent_semantics.md` §Explicitly not provided](absent_semantics.md)).
  `warn_derived_naf` is the advisory proxy in the meantime.
- S1.7.4
  — static NAF dependency map: **shipped** 2026-06-01 (see "Static
  NAF dependency map" above). `Engine.naf_dependency_map()` +
  the post-saturation `DerivedNafWarning` (default-off
  `warn_derived_naf`). Relocated to P1.7 on 2026-05-26 (formerly
  P1.5a S1.5a.8 / T1.5a.1.2).

## Hypgen pre-pruning — disjunctive-prune (S1.5a.2)

The hypothesis generator
([`hypgen::generate`](../../../ein.rs/crates/ein-infer/src/hypgen.rs))
emits one candidate `(?R ?A ?B)` per legal slot-fill at root
saturation; each candidate becomes a hypothesis the solver
might branch on. The generator's filter consults
[`Kb::negated`](../../../ein.rs/crates/ein-core/src/kb.rs)
to drop candidates whose negation is already known: a
candidate ``(color-loc Yellow House-3)`` is dropped if
``(not (color-loc Yellow House-3))`` is in `_negated_facts`.
The `--hyp-stats` output's `filtered.negated_fact` counter
measures this filter's hit-rate (62 of 125 raw candidates on
zebra2, leaving 56 emitted).

Pre-S1.5a.2 the only `(not …)` facts entering
`_negated_facts` at root saturation were the ones the puzzle
declared directly. Cross-attribute spatial constraints
(`adjacent-via`) didn't contribute negatives — the
`adjacent-via-{fwd,bwd}` rules only assert positives when the
spatial neighbour is unique. For non-corner houses no positive
fires and the candidate stays in the hypothesis space.

**The fix.** Two new rules ship with each `adjacent-via`
activator:

- ``disjunctive-prune-fwd ?S ?R1 ?V1 ?R2 ?V2`` — given
  ``(R1 V1 h1)``, for every ``h_other`` in the partner's
  type-domain where ``(?S h_other h1)`` is absent, assert
  ``(not (R2 V2 h_other))``.
- ``disjunctive-prune-bwd`` — symmetric, with the NAF operand
  order reversed for asymmetric ?S like ``right-of``.

These fire in BOTH unique and non-unique cases, so they
contribute negatives even when the positive can't be pinned.
The pair derives from a single ``(adjacent-via ?S V1 V2)``
activator via two meta-rules (`derive-disjunctive-prune-{fwd,bwd}`
at priority 200) — author writes one activator, gets both
pre-pruners. Priority 250 on the pruner itself ensures the
next-to derivations at priority 100 drain first, so the NAF
guard sees the closed adjacency graph.

The split into `-fwd` / `-bwd` matters for asymmetric ?S:
pre-S1.5a.2 there was a single rule whose `-bwd` direction
swapped the activator args but kept the `-fwd` NAF, asserting
spurious ``(not (color-loc Ivory House-4))`` from a known
``Green@House-5``. For symmetric ``next-to`` the two NAF
directions are equivalent and the bug was masked; the
S1.5a.11 dump on `zebra2-hints.ein` surfaced it in its first
realistic outing. The two-rule structure makes each
direction's NAF explicit in its own match clause.

## Determinism — content-based candidate ordering (S1.5a.1a)

> **Reconcile note.** The names in this section are the **removed tree
> solver's** (`_candidates_for` / `_candidate_sort_key`). The live ordering is
> [`apriori::order_candidates`](../../../ein.rs/crates/ein-infer/src/apriori.rs)
> (+ `_set_score`), applied by
> [`solve.rs`](../../../ein.rs/crates/ein-infer/src/solve.rs);
> [`hypgen::score_hypothesis`](../../../ein.rs/crates/ein-infer/src/hypgen.rs) is the
> score key. The *principle* below — sort candidates by a content key, never by
> `frozenset` / hash iteration order — is unchanged.

`solve()` visits hypothesis branches in the order `_candidates_for`
returns them. Pre-S1.5a.1a that list was the iteration order of a **hash
set** of facts, and every invocation explored branches in a different one.

> The *cause* was CPython's per-process string-hash randomisation; ein.rs's
> hash maps are seeded identically on every run, so the same code would look
> deterministic here and would still be wrong — the order would depend on
> insertion history and on which ids the interner happened to assign. That is
> exactly what
> [`id_order_invariance.rs`](../../../ein.rs/crates/ein-render/tests/id_order_invariance.rs)
> perturbs on purpose, and it is why the rule below is stated over *any* hash
> iteration rather than over a randomised one.

The fix sorts the result of `_candidates_for` by
`_candidate_sort_key`:

```python
(-score_hypothesis(fact, kb), fact.args, fact.relation_name)
```

All three components are content-derived; `hash(str)` never
reaches the tuple. With the M1 stub
[`score_hypothesis`](../../../ein.rs/crates/ein-infer/src/hypgen.rs)
returning `0` for every fact, the effective order is
``(args, relation_name)`` — alphabetic on first arg, then
second, then relation. The score primary key is the slot
S1.5a.7
fills in (fact-popularity sum, weighted relation/object
coefficients); when it lands, the solver doesn't move.

**Determinism rule for new code.** Any `set` / `frozenset`
whose iteration order influences user-visible output (branch
IDs, trace ordering, log lines, fixture-dependent test
assertions) must be sorted at the iteration boundary. Membership-only
structures — the fired set, the negated index, the seen set — do not need
sorting. The audit point is the read site, not the storage site. The rule is
mechanically checked by
[`check_hashmap_iteration.py`](../../../utils/check_hashmap_iteration.py),
which fails on any hash-map iteration at a site whose order could reach an
output unless it carries a `// determinism-ok:` reason.

The subprocess pin this paragraph used to cite,
`tests/inference/tree/test_branch_determinism.py` (two
`PYTHONHASHSEED`s, byte-identical solve output), went with the tree
solver in `8d77b02`. The surviving order-leak guard is
[`lattice_semantics.rs`](../../../ein.rs/crates/ein-infer/tests/lattice_semantics.rs),
which shuffles each layer's candidate order and asserts the resulting
`LatticeSnapshotV1` still compares equal to the unshuffled run's — a
stronger statement about the lattice, but no longer a hash-seed test.
Restoring an explicit `PYTHONHASHSEED` pin for the lattice engine is
open work.

## d=0 negative-completion (S1.5a.19)

The NL Zebra walkthrough closes at depth 0 — every "Therefore X"
in the trace is reachable from the puzzle's facts + ontology
without any hypothesis branching. Pre-S1.5a.19, the engine
needed branching to discover the same negatives: a known
``(color-loc Yellow House-1)`` did not derive
``(not (color-loc Yellow House-{2,3,4,5}))`` in the same
saturation pass, so the candidates lingered in hypgen's
output and the solver split into 568 nodes searching for a
contradiction that NL closes at d=0. After S1.5a.19 the tree
collapses to 32 nodes at `--max-depth 1` (S1.5a.19).

Six new rules ship in
[`examples/zebra2.ein`](../../../examples/zebra2.ein) (mirrored
in `zebra2-hints.ein`) to close the gap. Each derives a
``(not …)`` directly from positive evidence + an ontology
declaration, with no recourse to branching:

| rule                                              | premise pattern                                                                                                                 | derived negative                                |
|---------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------|
| `functional-negative ?R`                          | ``(R ?a ?b)`` ∧ ``functional R`` ∧ ``b' ≠ b``                                                                                  | ``(not (R a b'))``                              |
| `injective-negative ?R`                           | ``(R ?a ?b)`` ∧ ``injective R`` ∧ ``a' ≠ a``                                                                                   | ``(not (R a' b))``                              |
| `co-located-negative ?R1 ?V1 ?R2 ?V2`             | ``(co-located R1 V1 R2 V2)`` ∧ ``(not (R1 V1 h))``                                                                             | ``(not (R2 V2 h))``                             |
| `adjacent-via-endpoint-fwd ?S ?R1 ?V1 ?R2 ?V2`    | no ``h2`` with ``(?S h2 h1)``                                                                                                  | ``(not (R1 V1 h1))``                            |
| `adjacent-via-endpoint-bwd`                       | no ``h1`` with ``(?S h2 h1)``                                                                                                  | ``(not (R2 V2 h2))``                            |
| `adjacent-via-{fwd,bwd}-negative`                 | contrapositive of `adjacent-via-{fwd,bwd}` — ``(not (R2 V2 h2))`` + unique ?S-source ⟹ ``(not (R1 V1 h1))`` (and symmetric)    | ``(not (R1 V1 h1))`` resp. ``(not (R2 V2 h2))`` |

Each rule has a `derive-…` meta-rule (priority 100 or 200)
that lifts an ontology activator (`functional R`,
`co-located R1 V1 R2 V2`, `adjacent-via-{fwd,bwd} ?S …`) into
the target rule's own activator. Authors keep writing one
ontology-level declaration per constraint; the engine fans it
out into the negative-completion machinery automatically.

**Priority discipline** (lines 117-120 of `examples/zebra2.ein`):
the negative rules sit at priority 240 — AFTER propagation
(200) so the new positives are visible, BEFORE both
sibling-violation rules (250) and elimination rules (400), so
derived negatives reach `domain/range-elimination`'s `forall`
premises in the same pass.

The NL chain this closes (see
[`examples/README.md`](zebra_walkthrough.md)):
*Norwegian@H_1 ⟹ Englishman ≠ H_1 ⟹ Red ≠ H_1* — once
`functional-negative` produces the first negative,
`co-located-negative` propagates it across the equivalence,
and the cascade terminates at the corner-house exclusions from
`adjacent-via-endpoint-{fwd,bwd}`.

Naming convention: every rule name carries `-negative` so the
trace renderer (P1.6 S1.6.4) can group derivation events by
polarity. The `derive-…-negative` meta-rules are similarly
named after the target they enable.

These elimination rules are pathway **A** in the "domain-elimination
rule vs explicit hypothesis exploration" comparison: when their
premises are derivable they solve at root saturation and preempt the
hypothesis search entirely. The S1.5b.32 measurement
([`domain_elim_vs_hypothesis.md`](domain_elim_vs_hypothesis.md))
quantifies the trade-off — A leaves the exhaustive lattice at 0 sets;
without it the engine falls back to forced-positive promotion (1 set)
or, with every elimination path off, full branch-and-refute (7 sets,
6 nogoods on the fixture).

### The same inference over ONE generic relation (`std.slots`, S1.22.1a)

Everything above is keyed to a property of *one relation* —
`(bijective color-loc)` and its fan-out. `examples/zebra.ein` links every
attribute through a single `co-located` equivalence, which is not that
kind of relation: restricted to one ordered pair of types it is a
bijection, but `bijective` has nowhere to put a type pair.
[`std.slots`](../../../stdlib/slots.ein) supplies the same
inference from a property scoped by the type **family** —
`(slot-partition R isa sub Super Index)` — plus one
`(slot-spatial R S isa PositionType)` per spatial relation. The
correspondence, rule for rule:

| `std.bijection` / zebra2 | `std.slots` / zebra.ein | note |
|---|---|---|
| the `co-located` 4-ary propagation rule | `slot-locate` | index-anchored transitivity; here a clue is a *fact*, not an activator |
| `injective` (check) | `slot-exclusive` | all-different within a type, derived from the membership facts |
| `functional-negative` + `injective-negative` | `slot-occupied` | one rule, because R is symmetric: the two argument positions collapse |
| — | `slot-negative` | contrapositive of transitivity; carries a negative across an authored cross-attribute link |
| `domain-elimination` | `slot-elimination` | every slot but one excluded for a value |
| `range-elimination` | `slot-fill` | every member but one excluded for a slot |
| `total` / `surjective` (⊥) | `slot-no-room` / `slot-no-fill` | the encoding's main branch killers |
| `adjacent-via-{fwd,bwd}` | `slot-adjacent-{fwd,bwd}` | same unique-neighbour NAF gate |
| `adjacent-via-{fwd,bwd}-negative` | `slot-adjacent-{fwd,bwd}-neg` | |
| `disjunctive-prune-{fwd,bwd}` | `slot-prune-{fwd,bwd}` | |
| `adjacent-via-endpoint-{fwd,bwd}` | `slot-endpoint-{fwd,bwd}` | |

The priority bands are the same (100 setup, 200 propagation, 240 negative
completion, 250 pruning + violation checks, 400 elimination), so the two
libraries are interchangeable band-for-band.

Two asymmetries are worth knowing, because they are what the second
encoding exists to expose:

- **`slot-elimination` and `slot-fill` are both needed, and they are not
  each other's mirror.** R's symmetry makes the two *conclusions*
  interchangeable — `(R a i)` and `(R i a)` are the same edge — but the
  rules quantify over different domains, so they consume different
  negatives and fire at different times. The Zebra opening needs
  `slot-fill` ("House-1's colour seat has only Yellow left"); the endgame
  needs `slot-elimination` ("Zebra has only House-5 left").
- **A symmetric relation needs its negative mirror.** `std.algebra`'s
  `symmetric-negative` is decoration for a directed encoding and
  load-bearing here: most negatives are derived in one argument order
  only, and without the mirror `(not (co-located House-2 Green))` never
  reaches a rule matching `(not (co-located Green House-2))`.

Both libraries also make the same *closed-world* bet, in the same place:
the only NAF reads are on the position structure (the unique-neighbour,
pruning and endpoint guards), which is sound exactly when that structure
is saturation-determined — for Zebra, condition (1) fixes the row of
houses. The attribute side is never read under NAF, so nothing prunes a
branch the search still needs.

Measurements, including why `std.slots` anchors its conclusions at the
`Index` type instead of enumerating the equivalence closure, and why a
densely multi-justified proof graph makes
`:record-alternative-justifications` the most consequential config knob in
that file, are in
C2.

## Mid-sweep saturation + per-sibling apriori re-check (S1.5a.19)

> **Superseded — describes the removed tree solver** (tree engine removed in
> `8d77b02`; its last dead residue deleted in S1.9.E6a). The `_consume` loop,
> `try_branch`, `back_propagate`, and `is_unconditional_death` named below no
> longer exist, in either engine. The live engine is the
> set-indexed **lattice** (see *Set-indexed search — monotonic engine* below),
> which bakes the per-set saturate-from-root pattern in from the start. (The
> transitive "unconditional" walk that briefly survived the tree solver in
> `commitment.rs` was itself retired in P1.21 R2 — see the
> historical note *Unconditional facts — retired* below.)
> Kept for the algorithmic intuition: each commitment closes its consequences
> before the next decision.

The d=0 rules above are necessary but not sufficient on their
own — the solver's `_consume` loop must actually *use* the new
negatives. Pre-S1.5a.19's loop tested every sibling in the
parent's alive set via the full `try_branch` (fork + saturate
+ contradiction-detect), even if an earlier sibling's
back-prop had just made the next sibling apriori dead. The
cost was paid for the contradiction to re-surface inside the
fork.

S1.5a.19 fixes this with two cheap pre-fork checks plus a
mid-sweep saturator pass (in the removed tree solver; the listing below is
its Python, kept because the *shape* is what the section explains):

```python
for h in to_check:
    key = (h.relation_name, h.args)
    # (a) Apriori Tier-A re-check: earlier sibling's back-prop
    #     + in-sweep re-saturation may have made h dead.
    if key in kb._negated_facts:
        stats.apriori_dead_in_sweep += 1
        # mark dead, no try_branch
        continue
    # (b) Mid-sweep re-saturation may have derived h's positive
    #     directly (functional / adjacency closure).
    if kb._fact_by_id(h.relation_name, h.args) is not None:
        # mark alive, no try_branch
        continue
    result = try_branch(kb, h, branch_id=...)
    if result.is_alive():
        ...
    elif is_unconditional_death(result.kb, result.unsat_core, ...):
        back_propagate(kb, h, result.unsat_core)
        # Mid-sweep saturator: propagate (not h)'s transitive
        # consequences into kb so subsequent apriori re-checks
        # can skip more siblings.
        mid_sweep_firings.extend(
            Saturator(kb).saturate(max_steps=10_000))
    else:
        ...
```

Three pieces compose:

1. **Apriori Tier-A re-check** before `try_branch`: query
   `kb._negated_facts` directly; if the sibling's negation is
   now known, skip the fork and mark dead in one O(1) step.
   Counted in `stats.apriori_dead_in_sweep`.
2. **Positive-already-derived check**: between siblings the
   mid-sweep saturator may have derived h's positive directly
   (e.g. via `adjacent-via-bwd` from a recently-pinned
   ``(?R2 ?V2 h2)``); mark alive and skip the fork.
3. **Mid-sweep `Saturator(kb).saturate(...)`** after each
   `back_propagate`: runs the saturator on the parent KB with
   the freshly-bubbled ``(not h)`` so the d=0
   negative-completion rules can fire transitively before the
   next sibling is tested. `max_steps=10_000` caps the cost;
   on zebra2 the sweep terminates well below the cap.

Measured impact on zebra2 (depth 1): 28 of 31 dead leaves
(`apriori_dead_in_sweep=28`) skip `try_branch` entirely via
the Tier-A path; the three remaining dead siblings need the
full fork (cases the apriori check can't predict from
`_negated_facts` alone — e.g. a sibling whose conditional
contradiction depends on the candidate's own consequences).

Together the rules + mechanism implement at the engine level
what the NL trace does at the cognitive level: each commitment
unfolds its consequences fully before the next decision. The
result is the 568 → 32 node collapse measured at S1.5a.19.

**Future composition.** The mid-sweep saturator pass is the
engine's "go up" channel; pre-2026-05-26 it was the motivation
for the now-dropped S1.5a.20 branch-isolation re-architecture.
The
P1.5b
set-indexed engines (monotonic + lattice) bake the per-set
saturate-from-root pattern in from the start, so the mid-sweep
pass becomes the default control flow rather than an opt-in.
The tree-side `_consume` keeps the explicit mid-sweep until
P1.5b reaches parity; then the per-sibling re-check moves to
whichever engine inherits the responsibility.

## Unconditional facts — retired (S1.5.7 → P1.21 R2)

> **Historical note.** The mechanism this section used to document —
> classify each alive fork's newly-derived facts as *unconditional* and
> merge them into root mid-search — was **removed** in P1.21 R2
> (report):
> the classification is unsound under NAF (`absent`). Deleted with it:
> `CommitmentSetResult.unconditional_facts`, `commitment._is_unconditional`,
> `provenance.reaches`, both dumpers' `unconditional_count` fields, and the
> `unconditional_facts.jsonl` lattice-dump writer. This note records what
> was believed, the counterexample that falsified it, and the model that
> replaced it.

**What was believed (S1.5.7).** When a commitment's fork saturated, the
engine asked of each newly-derived fact whether it was *unconditional* —
its whole derivation chain grounding out at root facts, never touching a
committed hypothesis
(S1.5.7).
Such a fact was held "provably true at root given root + rules", so
`try_commitment_set` extracted it (a positive-edge provenance DFS over
`premises_raw` with a commitment-set terminal) for the engine to merge into
root, monotonically shrinking the alive set. The predicate even erred
carefully — an empty or unresolvable chain read as conditional, never
unconditional — but the care was misdirected: the walk was sound over the
*recorded* edges, and the recorded edges were not the dependencies.

**The NAF counterexample.** An `(absent P)` premise contributes **no
premise fact** to a firing
([`match_.rs`](../../../ein.rs/crates/ein-infer/src/match_.rs) — the guard
passes silently), so a fork fact derived through NAF carries no provenance
edge to the commitment whose absence licensed it. Concretely — rule
`(and (seed ?s) (absent (x a))) → (y ?s)`, a second rule making any `x`
clash with any `y`, and `(seed s)` at root:

- under an unrelated commitment `{g(b)}` the fork derives `(y s)`; its
  only recorded premise is the root fact `(seed s)`, so the walk
  classified it **unconditional**;
- yet the sibling commitment `{x(a)}` is a genuinely consistent world in
  which `(y s)` does **not** hold — refuting "true at root" directly (a
  root-true fact must hold in every consistent extension of root);
- performing the documented merge (root′ = root + `(y s)`) flips `{x(a)}`
  from alive to dead-post: a real model is refuted, the model count `k`
  is undercounted, and the verdict read off `k` degrades (Ambiguity →
  Solution, Solution → Contradiction).

A sound test needs `Deps(Y) = PositiveDeps(Y) ∪ NegativeDeps(Y)`, and
`premises_raw` carries only the positive half — no smarter walk over it
can recover dependence-through-absence.

**Since S1.21.8 the other half exists**, on the same record:
[`Prov::absent`](../../../ein.rs/crates/ein-core/src/prov.rs)
holds the `(absent …)` queries a firing was admitted through, so `Deps(Y)`
is finally representable. That is the *precondition* for reviving the
extraction, not the revival: nothing yet interprets those entries, so a
walk that ignored them would be exactly as unsound as before. A revival
would have to read them and refuse any fact whose chain passes through an
absence root cannot also assert.

**The model now (P1.7a — keep root stable).** Mid-search root writes are
limited to two *sound* mechanisms: the singleton-death `(not h)` writeback
and the forced-positive cascade (`_helpers._promote_forced_positives` — a
sole-surviving slot value must hold). Everything else is per-branch: each
commitment is evaluated independently against the post-Phase-1 root, and
no fork fact ever merges back. Pinned by
[`naf_semantics.rs`](../../../ein.rs/crates/ein-infer/tests/naf_semantics.rs).

**Resurrection path, if ever needed.** On a ruleset with **no** `absent`
guards — checkable as every compiled plan's
[`Plan::has_naf`](../../../ein.rs/crates/ein-infer/src/compile.rs) being False
(equivalently `naf_relation_refs` empty), surfaced as
`Engine.naf_dependency_map` — the extraction
theorem genuinely holds: `premises_raw` is then a complete dependency
record, and a chain grounding out at root facts replays at root by
monotonicity. Any revival must gate on that *checked* precondition, or
move to a dependency carrier that records negative support (ATMS-style
environments). Fork-internal NAF *ordering* is a distinct hole, covered by
the `absent`-semantics formalisation —
[`absent_semantics.md`](absent_semantics.md) (§C1 no-root-merge and §C2
negative-dependence are this section's two lessons, stated as corollaries
of the boundary epistemic definition — C2 having been re-grounded by
S1.21.8 from "positive provenance is not dependence" to "negative
dependence is now recorded, and not yet interpreted").

The negative dual — caching a forced `(not h)` — remains live and is
deliberately narrow: only the **one-step lookahead kill**
(`hypgen._write_negated`, gated by `enable_lookahead_kill_cache`) writes a
`(not h)` REASONING fact, and only when a single rule firing already
refutes `h` before any fork. (The former full-saturation "unconditional
death → `(not h)` into the parent" — `back_prop.is_unconditional_death` /
`reaches_hypothesis` — was tree-solver machinery: dead after the tree
solver's removal, deleted in S1.9.E6a.)

## Set-indexed search — monotonic engine (P1.5b S1.5b.0–.10)

The tree engine's depth-first ordering over hypothesis branches
prices in d! orderings of the same commitment set — for d=4 on
zebra2 that's 24× redundant work on each set. The **monotonic
engine** under
[`solve.rs`](../../../ein.rs/crates/ein-infer/src/solve.rs)
collapses this by indexing by commitment **set** rather than
path: layer N enumerates every size-N alive subset via
Apriori-style prefix-join and enters each via the common
[`commitment::try_commitment_set`](../../../ein.rs/crates/ein-infer/src/commitment.rs)
primitive; the root KB stays stable mid-search (P1.21 R2 — see
*Unconditional facts — retired* above). There is **one entry**,
[`solve`](../../../ein.rs/crates/ein-infer/src/solve.rs): it
records every solution node (`consistent ∧ complete`, `state_key`-deduped)
plus every refuted commitment, and
[`solve.rs`](../../../ein.rs/crates/ein-infer/src/solve.rs)
reads the verdict off the count `k` of distinct solution nodes —
`k = 0` → Contradiction (unsat core — each dead commitment's *smallest*
explanation, unioned across the deads), `k = 1` → Solution, `k > 1` →
Ambiguity (gaps). These are **three answers to one problem**, selected
by the input, not by which function was called (the unsound
`gaps_solve` / `contradictions_solve` split was removed 2026-06-16 —
Q1.5b.7). The orthogonal **stop policy** (`stop_after=1` single / `N` /
`None` exhaustive) only bounds how far the lattice is walked;
`store_lattice=True` attaches a sound
[`LatticeProof`](../../../ein.rs/crates/ein-infer/src/solve.rs)
carrying both the gaps view (`proof.solutions`) and the contradictions
view (`proof.dead_commitments` + `verdict.unsat_core`).

### Termination conditions, in order of precedence

1. **Solution at a fork.** `is_solved(result.kb, Mode.SOLVE)` on an
   alive entering — the fork's saturated kb carries the
   committed hypotheses + their derivations, which is the
   context the goal needs when it references hypothesis facts
   directly (e.g. `examples/branching/05_mini_zebra.ein`).
   Returns `Solution(kb=result.kb)` so the caller sees the
   hypothesis context the goal depended on. **Algorithm spec
   §3d.vii.**
2. **Solution at root.** The forced-positive cascade
   ([`promote_forced_positives`](../../../ein.rs/crates/ein-infer/src/solve.rs),
   the sound inter-layer prune) promotes each singleton-alive
   hypothesis to root, re-saturates, and repeats until the root
   itself is complete ∧ consistent. This is how
   `examples/zebra2.ein` solves — the layer-1 deaths (plus the
   chain of pre-emptive lookahead negatives) leave one survivor
   per slot, and the cascade starting at
   `(color-loc Green House-4)` completes the puzzle at root.
3. **Contradiction at Phase 1.** Root saturates to `(false)` —
   the puzzle is inconsistent before any hypothesis enters.
4. **Contradiction at Phase 3.** Every layer-1 singleton died;
   `_compute_alive` returns ∅; verdict is Contradiction.
5. **Ambiguity.** Layer cap reached with alive ≠ ∅ and no
   goal-satisfying commitment found.

### Learned no-goods (S1.5b.6)

Ein's search layer is an **ATMS-style environment search with Apriori
candidate generation and nogood learning**: commitment sets are
assumption environments explored breadth-first by cardinality, a dead
environment is learned whole as a no-good clause (kept
subsumption-minimal), and Apriori's downward-closure filter suppresses
its supersets. **CDCL is the SAT-world analog** (no-good ≈ conflict
clause), not the mechanism — and, as a direction, measured out: the
reorderer / consistency-pre-pass cluster was tried and rejected against a
complete cardinality-BFS
([F9 ledger](../../../plans/followups/f9_e_catalog.md)).

Every dead entering emits `frozenset(C)` into
`root_kb._nogoods` via `inference.nogoods.emit_nogood`
(min_size=1 so layer-1 singleton deaths land — Q1.5b.5.c).
The next layer's `generate_layer` filters supersets via the
existing `apriori.filter_candidate` subset check; the engine
never re-enters a strict superset of a known-dead set.
Singleton dead clauses additionally write `(not h)` into
`root_kb._negated_facts` (plus the symmetric mirror if
`(symmetric R)` is in the ontology) so subsequent
`_compute_alive` calls drop h from `alive`.

How this differs from CDCL, mechanically:

| CDCL | Ein lattice search |
|---|---|
| ordered **decision trail**, one variable per decision level | unordered **commitment set C** (an ATMS environment); whole layers by cardinality (Apriori prefix-join) |
| per-conflict **implication graph** + cut analysis | per-fact **provenance AND/OR graph** (ATMS justifications — OR over every recorded derivation); ATMS labels are computed on demand by `explain.rs` to *explain* a conflict, never to learn from one — no conflict-cut analysis |
| learned clause = **1UIP-minimised** asserting clause | learned clause = **the full dead environment** (`learned_clause == frozenset(C)`, contract-pinned); shrinking measured vacuous + NAF-unsound (ex-E7, [F9 ledger](../../../plans/followups/f9_e_catalog.md)) |
| asserting clause **propagates immediately** after backjump | clause only **filters future candidates** pre-fork (`filter_candidate`); size-1 clauses also write `(not h)` |
| **non-chronological backjump** | **no backjump** — the BFS layer loop just continues; superset suppression prunes descendants |
| VSIDS activity, restarts, watched literals | `lex`/`score-sum` candidate order; none of the rest — the one watched-literal-shaped win is taken at insertion instead: a fork's saturation stops at the firing that kills it (`enable_fail_fast_fork`, S1.9.E23) |

The genuine CDCL *direction* is closed out rather than pending. Both
recorded entries settled in 2026-08 ([F9
ledger](../../../plans/followups/f9_e_catalog.md)): the exhaustive-search
umbrella (ex-E23) shipped as **fail-fast fork saturation** — 2.4× on
exhaustive `zebra2`, uniqueness untouched — and not as any of its
branch-count candidates (learned-clause caching, goal-driven pruning, AC
pre-pass: each measured inert or unsound here); the cross-call conflict
cache (ex-E20 ≈ incremental SAT) was rejected as puzzle memoisation rather
than reasoning. What remains is the DPLL/CDCL re-architecture
lever in [`architecture_and_algorithms.md`
§7](architecture_and_algorithms.md#7-summary--where-the-bodies-are-and-the-levers).

### Diagnostics — `MonotonicDumper` (S1.5b.7)

Optional `dumper=MonotonicDumper(out_dir=…)` captures:

```
dump/<puzzle>-<ts>/
   00_root_initial.ein           ← root before any enterings
   00_timeline.jsonl             ← chronological event log
   layers/
       layer_NN_pre.ein          ← root.kb at layer N start
       layer_NN_post.ein         ← root.kb at layer N end
   summary.json                  ← final stats + verdict
```

Six lifecycle hooks (`root_initial`, `layer_start`, `entering`,
`layer_end`, `early_terminate`, `summary`) fire from the
backbone; `dumper=None` is a no-op for every hook site. The
[the `Dumper` hook](../../../ein.rs/crates/ein-render/src/dump/state.rs)
subclass streams the same events to stderr as live progress lines
(so a multi-minute exhaustive `solve` isn't a silent hang) without
needing an on-disk dump.

`MonotonicDumper` captures only the per-layer root snapshots —
on the default single-solution stop policy the engine
early-terminates, so most hypotheses are never reached and there's
nothing per-hypothesis to record. For a
**complete per-hypothesis record** — every commitment tested at
every layer, with the firings each one emitted, survivors and
casualties alike — run `solve` exhaustively (`stop_after=None`) with
a `LatticeDumper`:
see [`lattice_dump.md`](lattice_dump.md). That dump groups
`enterings/` and `kb_index/` by layer and writes
`outcome.txt` + `firings.jsonl` + `unsat_core.jsonl` per
commitment — the audit trail for debugging problem statements and
rules.

### Budget — `max_time` / `max_enterings`

`solve(..., max_time=N, max_enterings=K)` checks the
caps before every `try_commitment_set` call; on exhaust raises
`BudgetExceededError(reason, stats)` with the partial counters.
The dumper's timeline is flushed via `MonotonicDumper.close()`
on the abort path (no `summary.json` then — the events up to
the abort suffice for diagnostic).

### Measured performance

> **Frozen constants.** Both figures below were taken on the Python engine
> against the removed tree solver; neither instrument exists. What is live is
> the same solve on ein.rs — milliseconds, re-measurable through
> [`e2e_baseline.py`](../../../utils/e2e_baseline.py) and recorded in
> [`baseline.md`](../../history/m1a_rust/measurements/baseline.md).

- `examples/zebra2.ein`: Solution in ~1.9 s under PyPy (CPython ~2.8 s),
  1 alive entering, 0 nogoods — single-shot solve via fork-side
  `is_solved`. ~18× faster than tree on CPython; ~4× on PyPy.
- `examples/branching/*` (11 fixtures): all 11 reach the
  tree-side bindings; combined parity-test wall ~3.5 s. See
  [`parity_baselines.md`](parity_baselines.md).

### Two engines, two termination criteria

The monotonic and lattice engines are **not** interchangeable
implementations of one search — they terminate differently on purpose,
and the verdict each can report follows from that (M1 P1.5b Q1.5b.7,
resolved 2026-05-25; the user's framing: *"monotonic converges to first
met solution if any, lattice converges to full map of all solutions"*).

| engine | termination | modes it can serve |
|---|---|---|
| **monotonic** | **first goal-satisfaction** at root, or layer exhaustion, or the `max_set_size` cap | SOLVE only — the architecture stores no per-set state, so it can enumerate neither multiple solutions nor per-dead-set unsat cores |
| **lattice** | **exhaustive** to `max_set_size`; no early exit | SOLVE / GAPS / CONTRADICTIONS |

Monotonic's verdict is decided by *which* termination condition fired
(the five outcomes above). Lattice computes its verdict at end-of-search
from the accumulated alive/dead frontier plus per-set `is_solved`:
**Solution** iff exactly one SetNode satisfies the goal and nothing alive
remains unexplored above the cap; **Ambiguity** if several satisfy it, or
one is still alive at the cap without satisfying;
**Contradiction** otherwise. See
[`algorithm_layer_n.md`](algorithm_layer_n.md) §3d.vii.

### Cross-links

- Algorithm spec: [`algorithm_layer_n.md`](algorithm_layer_n.md) §3d.
- Diagrams: [`lattice_diagrams.md`](lattice_diagrams.md).
- Stage plan: M1 P1.5b S1.5b.0–.10 (plans removed at P1.22; see git
  history).

## Where the design lives today

The complete plan, including task breakdown and acceptance criteria:

- Plan phase P1.3 — Inference rules.
- Plan phase P1.5 — Hypothesis loop.
- Plan phase P1.6 — Rendering + trace.
- Idea: [`docs/ideas/06-inference-rules-completeness.md`](../../../plans/ideas/06-inference-rules-completeness.md).
- Idea: [`docs/ideas/08-human-style-deductive-trace.md`](../../../plans/ideas/08-human-style-deductive-trace.md).

When P1.3 work begins, this stub becomes a hub for the
implementation reality.
