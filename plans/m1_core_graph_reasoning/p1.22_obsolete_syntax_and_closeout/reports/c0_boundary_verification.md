# C0 — Boundary verification debt: completeness + state parity

**Stage:** [S1.22.0](../s1.22.0_boundary_verification.md), task T1.22.0.1
(attack) + T1.22.0.2 (fix + pin). **Date:** 2026-08-17.
**Method:** executed probes only. Nothing below is recorded as a finding
unless it was *reproduced* end-to-end; nothing is recorded as sound unless a
probe was run that would have shown it otherwise.

## Verdict summary

The three unfinished attack angles were run. **Angle A came back clean**
(four items, four sound-with-pin verdicts). **Angle B found three confirmed
defects, two of them soundness bugs.** **Angle C came back clean** on parity,
determinism and state leakage, and forced the one open decision
(`World.commitment`).

| item | verdict | pin |
|---|---|---|
| A1 monotonic growth | sound | (covered by A2/A4 pins + no-retraction) |
| A2 predicates in `watched` | **sound-with-pin** — hazard demonstrated | `test_watch_stamp_is_blind_to_predicate_guards` |
| A3 eq-classes | **sound-with-pin** | `test_unifier_does_not_resolve_eq_classes` |
| A4 nested-pattern args | **sound-with-pin** | `test_nested_pattern_guard_watches_the_outer_relation_only` |
| B1 nested-absent depth | sound — proved, not assumed | (B1 is what A/B pins rest on; proved by construction below) |
| B2 `(absent (or …))` | **CONFIRMED DEFECT — D-S8-4** | `test_nested_or_inside_an_absent_is_an_error` |
| B3 `_seen` × disjunct guards | **CONFIRMED DEFECT — D-S8-3** | `test_or_disjuncts_with_the_same_bindings_keep_separate_candidates` |
| B2c (found en route) | **CONFIRMED DEFECT — D-S8-5** | `test_arity_mismatched_activator_{is_an_error,does_not_activate_the_rule}` |
| C1 fork parity | sound-with-pin | `test_fork_parity_extends_to_the_boundary` |
| C2 determinism | sound-with-pin | `test_boundary_admission_is_not_hash_derived` (+ precondition test) |
| C3 state leakage | sound-with-pin | `test_boundary_state_does_not_leak_into_state_key` |
| C4 `World.commitment` | **decided: deliberately inert** | `test_the_live_engine_never_populates_world_commitment` |

The stage's own base-rate estimate ("one confirmed soundness bug per two
completed attack angles") held: three angles, three defects, two of them
unsound.

Checkpoint `3dd6d28`'s four in-flight probes were recovered first
(`_probe/fuzz_explain.py`, `_probe/p1_absent_alt.py`,
`_probe/p1b_absent_primary.py`, `_probes/p_a_scope_seed.py`). All four belong
to the two angles that had already *completed* (D-S8-1 soundness, D-S8-2
alternative-justification premises, the explain fuzz) and were closed by
`95b3d36`; they contained no unfinished work for A/B/C. The probes for this
report were written fresh.

---

## The common root

All three defects are the same mistake in three places: **a premise the
compiler cannot lower is silently dropped** (`return []`), and dropping is
never safe in either direction.

- Drop a **positive** conjunct → the match set grows → the rule fires on a
  premise that is false. **Unsound.**
- Drop a conjunct **inside `(absent …)`** → the negative query matches more
  often → the guard fails when it should pass. And a failing *monotone* guard
  is retired permanently by `_admit_from_boundary`, so the loss is not
  recoverable. **Incomplete, irreversibly.**
- Drop **every** premise → `steps=()` → `match._run_steps(())` yields one
  match → the rule fires **unconditionally**. **Unsound.**

The third is the sharpest: an empty plan is not "a plan that matches
nothing", it is a plan that matches *vacuously*. The same inversion drives
B2 — an empty guard sub-plan makes `World.holds` true, so the guard fails
against every possible KB.

---

## Angle A — `_watch_stamp` invalidation

`_watch_stamp` skips re-judging a parked candidate when no relation its
guards read has grown. Pure optimisation, so any equal-stamp/different-verdict
case is a lost firing.

### A1 — "the KB grows monotonically within a run" — **sound**

Probe: saturate `zebra2.ein` and, after every firing, compare each relation's
extent size against the previous step.

```
firings observed : 321      relations watched: 38      shrink events: 0
```

Audit of every write path backs it: `_index_fact` rebinds each key to
`(*existing, fact)` — an append, never a replacement — and every writer goes
through it (`firing.fire`, `saturator._next_mirror_firing`'s `__symmetric__`
mirror, `closed.emit_closed`, `hypgen`, `commitment`, `monotonic/sanity`, and
the monotonic **root writebacks** at `monotonic/_helpers.py:123,167`).
`rebuild_indexes` replaces the dicts wholesale but recomputes from
`self.facts`, which is itself append-only. There is no retraction API at all
(`contradiction.py:18`: "there's no way to retract a fact once asserted").

Root writebacks deserve the explicit note the stage asked for: they grow the
**root** KB while a root `Saturator` may hold `_park_stamp` entries. Growth
only ever makes a stale stamp *unequal*, which triggers a re-judge — the safe
direction. Shrinking would be the unsafe one and cannot happen.

### A2 — "every relation the guard reads is in `watched`" — **sound today, pinned**

`compile._watched_relations` walks `Scan`/`Join` and recurses into nested
`AbsentGuard`s. It does **not** account for a predicate `Guard`. That is
sound only because every registered predicate is stateless over the KB:

```
registry: ('eq', 'neq')
eq   reads KB: False    neq  reads KB: False
```

Both compute over `bindings` via `resolve_leaf`; neither touches the store.
`predicates.register` is public API for followups, so the hazard was
demonstrated rather than argued — a KB-reading predicate was registered under
an `(absent …)`, and the relation it reads was grown:

```
guard.watched  : {'blk'}      <-- 'grow' is NOT here
stamp before   : (1,)
stamp after add: (1,)         <-- UNCHANGED, though the verdict just flipped
```

Verdict: sound-with-pin. `test_watch_stamp_is_blind_to_predicate_guards`
pins the registry's KB-freedom, so a new predicate has to come with a
decision about `watched`.

This also underwrites `NafGuard.monotone`: "purely positive query ⇒
anti-monotone" is true only while predicates are KB-free. Same coupling, same
pin.

### A3 — eq-classes — **sound, pinned**

`match._candidates` documents that the participation index "does not apply
eq-class resolution — neither does the unifier", which is what makes
*extent size* equivalent to *match-set equality*. Probe: `union("A", "C")`,
then scan `(r C ?y)` against a KB holding only `(r A B)`:

```
classes after union(A,C): {'A': ['A', 'C']}
Scan (r C ?y) matches: []
```

`kb.classes` is an inert placeholder — `EqClasses` is never called by the
engine (grep: no `union`/`find`/`equivalent` caller outside `store.py`
itself and `fork`/`snapshot` copying). Pinned by
`test_unifier_does_not_resolve_eq_classes` so an F4 e-graph cannot land
without the stamp's argument being revisited.

Note for whoever does land it: the coupling lives in **two** layers — the
participation index *and* `_bind_arg`. Breaking only one leaves the engine
incoherent rather than tripping the pin; that is why the mutation used to
validate this pin had to change both.

### A4 — nested-pattern args — **sound, pinned**

A `Scan` on `not` carrying a `NestedPattern` arg is keyed on `not` alone.
Probe: hold `(not (r X B))`, then grow `r` by two facts:

```
|not| 1 -> 1     |r| 0 -> 2
matches before: 1     after growing `r`: 1
```

A stored `(not …)` fact holds its inner pattern as a frozen `Fact` arg, fixed
at creation, so growing `R` can neither add nor alter a `not` fact. `not` is
the right and complete watch key.

---

## Angle B — monotone retirement

### B1 — is the `monotone` test right? — **sound, proved**

`compile._has_nested_absent` checks only the **top level** of `sub_steps`.
The stage asked for this to be proved rather than assumed. Probe: compile
four shapes and measure the deepest level at which an `AbsentGuard` occurs
inside the outer guard's `sub_steps`.

| shape | sub_steps kinds | deepest | `monotone` |
|---|---|---|---|
| `(absent (and G (absent B)))` | Join, AbsentGuard | 1 | False |
| `(absent (and G (and H (absent B))))` | Join, Join, AbsentGuard | 1 | False |
| `(absent (absent B))` | AbsentGuard | 1 | False |
| `(absent (and (and (absent B) G) H))` | AbsentGuard, Join, Join | 1 | False |

`(and …)` flattens through `_compile_premise` (it `extend`s each child's
steps into one list), so nesting `and`s to any depth still deposits the inner
`AbsentGuard` at the top level of the enclosing guard's `sub_steps` — which
is exactly where `_has_nested_absent` looks. The top-level check is
sufficient, and `monotone` was False in every nested shape.

### B2 — `(absent (or …))` — **CONFIRMED DEFECT (D-S8-4)**

Reachable from the surface language: it parses and loads with no diagnostic.
`_compile_premise` returned `[]` for a nested `(or …)`, so:

```
1 bare (absent (or A B))
  guard.sub_steps : ()
  guard.monotone  : True      (True => retired on failure)
  (out X) derived : False     <-- neither p nor q is in the KB; it MUST fire
  naf_retired     : 1
```

An empty step tuple yields one match, so `World.holds` is True, so the guard
fails — against every possible KB — and `monotone=True` retires the candidate
permanently on its first judgement.

The second shape is worse because it is quieter. `(absent (and (p ?x) (or (q ?x) (z ?x))))`
compiles to a *non-empty* sub-plan with the disjunction silently deleted:

```
2 (absent (and (p ?x) (or …)))
  guard.sub_steps : (Join(relation='p', …),)     <-- the `or` is just gone
  (out X) derived : False
```

The guard is now judged on `(p ?x)` alone, which matches more often, so it
fails where it should pass. Same direction, no empty tuple to notice.

**Answer to the stage's question** ("is the silent always-fail acceptable, or
must it become a compile error?"): not acceptable — it is a wrong answer with
no diagnostic. It is now a `CompileError`.

### B2b — the same hole on the POSITIVE path — **UNSOUND**

Found by the same probe family, and worse than B2. `_match_disjuncts` splits
only a *top-level* `(or …)`; a nested one reaches `_compile_premise` and was
dropped there — on the positive path too:

```
plan.steps        : (Scan(relation='a', arg_slots=(Var(name='x'),)),)
(p X) in KB: False    (q X) in KB: False
(out X) derived: True
VERDICT: UNSOUND — nested (or …) silently dropped; rule fired on a false premise
```

`(and (a ?x) (or (p ?x) (q ?x)))` fired with neither disjunct true. Folded
into D-S8-4: one cause, one fix.

### B3 — `_seen` vs per-disjunct guards — **CONFIRMED DEFECT (D-S8-3)**

The stage's "sharpest hypothesis", and it was right. `_binding_key` is
`(rule_name, activator_args, bindings)` with **no disjunct index**, while
`naf_guards` is per disjunct. Two disjuncts producing the same bindings under
different guards collide in `_seen`; only the first is ever admitted.

```
disjunct 0: steps=(Scan('a', …),)  guards=[absent(block ?x)]   <-- FAILS
disjunct 1: steps=(Scan('a', …),)  guards=[absent(other ?x)]   <-- would pass

productive firings: []
naf_admitted: 0   naf_retired: 1   parked left: 0
(out X) derived: False
```

Disjunct 0 is parked, judged, retired (monotone). Disjunct 1 never gets a
candidate at all. Confirmed it is a lost firing and not intended semantics by
swapping the disjuncts — `(or …)` is commutative; the engine was not:

```
(or  block-guard  other-guard) -> (out X): False
(or  other-guard  block-guard) -> (out X): True
VERDICT: ORDER-DEPENDENT
```

Why the existing D5 tests miss it: both
`test_each_or_disjunct_keeps_its_own_guards` and
`test_or_disjunct_absent_is_evaluated_on_the_boundary` use *different*
trigger relations per disjunct (`t1`/`t2`), so their disjuncts never produce
the same bindings. The dedup predates S1.21.8 — what S1.21.8 changed is that
the two disjuncts now carry different guards, turning a harmless duplicate
into a lost firing, exactly as the stage predicted.

### B2c — vacuous empty plan — **CONFIRMED DEFECT (D-S8-5)**

Found while fixing B2. `compile_rule`'s arity-mismatch branch left `bindings`
empty and commented:

> Shape mismatch — leave bindings empty so the compiler produces a plan with
> unbound head vars (which the matcher will reject via the "unbound head var"
> branch).

**There is no such branch in `match._run_steps`.** The rejection was
`_compile_relation` returning `[]` — and for a single-premise rule that
leaves `steps=()`, which the matcher accepts as one vacuous match:

```
cache key: ('copier', ('p', 'q'))   steps: ()
(out …) facts: [('SENTINEL',)]
VERDICT: UNSOUND — empty plan matched vacuously, ground conclusion stored
```

It stayed invisible because the *usual* shape dies loudly: with a variable in
the `:assert`, `firing.build_fact` raises `KeyError: unbound var ?a in
:assert`. Only a **ground** conclusion survives to a stored fact.

And this is **live in `zebra2.ein`**, not a synthetic shape. zebra2 derives
the 1-ary property marker `(total color-loc)` while `std.algebra`'s `total`
rule takes two parameters `(?R ?isa)`:

```
failed after 42 firings
  RULE total      params ('R', 'isa')  activator args ('color-loc',)
  RULE surjective params ('R', 'isa')  activator args ('color-loc',)
```

Those two junk plans have been compiled on every fork of every zebra2 solve.
They never fired — by luck, not design: with `?R` unbound, `_slot` leaves the
`(not (?R ?a ?b))` sub-pattern as a raw `SForm`, and `match._bind_arg`'s
fallback compares an `SForm` against a stored `Fact` by `==`, which is never
true. One substitution rule away from asserting `(false)` into every branch.

---

## Angle C — fork parity, determinism, state

### C1 — fork parity at the boundary — **sound, pinned**

`test_saturator_fork_parity.py` pinned the fact set only. Extended to the
boundary observables on a `forall`-shaped fixture (nested absent — the one
guard shape that can flip fail → pass, so the one whose candidates park and
get re-judged), rather than zebra2's shape:

```
firings      direct=2  fork=2   OK
rounds       direct=3  fork=3   OK
admitted     direct=2  fork=2   OK
retired      direct=0  fork=0   OK
parked_left  direct=1  fork=1   OK
facts equal: True
```

The pin also asserts the fixture *exercises* the boundary (`naf_rounds > 1`,
`naf_admitted > 0`, `parked_left > 0`), so it cannot decay into a vacuous
comparison.

### C2 — determinism — **sound, pinned**

Nothing hash-derived reaches boundary admission. Across five
`PYTHONHASHSEED`s on an exhaustive-shaped `zebra2` root saturation, the
firing order, the canonical state and the boundary counters were byte-identical:

```
seed 0 / 1 / 42 / 12345 / 987654321
ORDER 342ccf3c9c8d34e078ea8da823c69a2a
STATE acf66c65b741ac693e873747cbc83423
OBS   rounds=40 admitted=39 retired=84 firings=321
```

The regression pin uses a sharper fixture than zebra2: the classic
unstratifiable program `p ← absent q; q ← absent p` at **one** priority band.
Both guards pass against the empty world and `_admit_from_boundary` admits
one candidate per round, so *which stable model the engine lands in* is
decided purely by enqueue order — the divergence
`absent_semantics.md` §Divergence documents. A companion test
(`test_admission_order_decides_the_model_on_a_non_stratified_program`) pins
that the fixture is order-*sensitive*, so the seed sweep cannot silently
become vacuous. zebra2's own rule set is stratified enough that a
hash-derived enqueue order does not change its answer, which is why the first
version of this pin failed to catch a deliberately hash-ordered mutation.

### C3 — snapshot/state leakage — **sound, pinned**

`state_key` is `(relation_name, args)`-only by S1.21.1, so this holds by
construction. Pinned anyway: mutate `naf_rounds` / `naf_admitted` /
`naf_retired` / `naf_dropped` / `_park_stamp` / `_alt_justifications` after a
solve and assert `state_key` is unmoved, plus that every entry is a 2-tuple.
Boundary admission order decides the answer on non-stratified programs, so
any of that state reaching lattice identity would stop two branches with the
same model from collapsing to one node.

### C4 — `World.commitment` — **DECIDED: deliberately inert**

Probe: trace every `World.__init__` through a full saturation.

```
World() constructions: 5      non-empty commitments: 0
```

Dead in the live engine, as the stage suspected. **Decision: document it as
inert, do not wire it** — recorded in `world.py`'s class docstring and pinned
by `test_the_live_engine_never_populates_world_commitment`.

Reasoning. The saturator building `World(self.kb)` is *correct*, not an
oversight: in a branch, `self.kb` **is** the fork, whose facts already
include the committed hypotheses, so every `holds` query is branch-relative
by construction. Wiring the tuple would add plumbing (solver → saturator)
that nothing reads. The one consumer that would genuinely want it is
branch-relative **negative provenance** — recording that an `absent_premises`
entry holds only under this commitment — and that needs a field on
`Provenance`, which `negative_premises` produces; populating
`World.commitment` alone would not reach it. Left as a followup, noted below.

---

## Fixes

All in `ein.py/src/ein/inference/`.

1. **`compile.CompileError`** (new) — every branch that silently dropped a
   premise now raises it:
   - nested `(or …)` in any premise position (D-S8-4, both polarities);
   - an `(absent …)` whose sub-plan compiled to no steps — the general form,
     which also catches an unbound relation head inside a guard;
   - an unbound relation head in a positive premise;
   - an activator whose arity cannot bind the rule's parameters (D-S8-5).
2. **`engine._activators_for`** filters arity-mismatched activator facts, and
   **`hrule.Hrules`** does the same for `:hrules` activators — a fact that
   cannot bind the parameters does not authorise the rule, so the pair is
   never constructed. This is what keeps zebra2's `(total color-loc)` from
   becoming a hard error while removing the junk plans.
3. **`saturator._enqueue_binding`** keys `_seen` on `(binding_key, guards)`
   (D-S8-3). Exact rather than a proxy for a disjunct index: same bindings
   *and* same guards really is the same candidate, so genuine duplicates
   still collapse. `engine._fired` stays guard-free — once the conclusion is
   derived, every other disjunct for those bindings is redundant, which is
   what it already meant.
4. **`world.World`** docstring records the C4 decision.

`NafGuard` tuples are hashable (every IR node and opcode is a frozen
dataclass), verified including nested-absent shapes, so (3) costs nothing.

## Pins

14 new tests. Every one was verified to **fail without the behaviour it
pins** — the six fix-pins by reverting each fix in turn, the seven
property-pins by mutating the property they assert (registering a KB-reading
predicate; making the unifier *and* the participation index eq-class-aware;
unwrapping `not` in `_watched_relations`; leaking `_parked` across Saturator
instances; sorting the enqueue pass by `hash`; appending
`len(_alt_justifications)` to `state_key`; populating `World.commitment`).

| test | file |
|---|---|
| `test_or_disjuncts_with_the_same_bindings_keep_separate_candidates` | `test_world_boundary.py` |
| `test_watch_stamp_is_blind_to_predicate_guards` | `test_world_boundary.py` |
| `test_unifier_does_not_resolve_eq_classes` | `test_world_boundary.py` |
| `test_nested_pattern_guard_watches_the_outer_relation_only` | `test_world_boundary.py` |
| `test_nested_or_in_a_positive_premise_is_an_error` | `test_compile.py` |
| `test_nested_or_inside_an_absent_is_an_error` | `test_compile.py` |
| `test_an_absent_whose_sub_plan_is_empty_is_an_error` | `test_compile.py` |
| `test_arity_mismatched_activator_is_an_error` | `test_compile.py` |
| `test_arity_mismatched_activator_does_not_activate_the_rule` | `test_compile.py` |
| `test_fork_parity_extends_to_the_boundary` | `test_saturator_fork_parity.py` |
| `test_boundary_state_does_not_leak_into_state_key` | `test_saturator_fork_parity.py` |
| `test_admission_order_decides_the_model_on_a_non_stratified_program` | `test_saturator_fork_parity.py` |
| `test_boundary_admission_is_not_hash_derived` | `test_saturator_fork_parity.py` |
| `test_the_live_engine_never_populates_world_commitment` | `test_saturator_fork_parity.py` |

## Gate

- `./run_tests.sh`: **1359 passed** (1345 + 14 new), **acceptance 17/17**,
  verdicts and bindings unchanged.
- `ruff check ein.py/`: clean. (The 5 `nlp/` findings are pre-existing in
  that unwired scratch area — confirmed by stashing this change.)
- Exhaustive `zebra2` (PyPy), `Solution`, k=1, exhausted, both before and
  after: **13.80 / 11.25 s before → 10.08 / 9.35 s after**. No regression;
  the gain is the two junk plans per fork that fix (2) removes. The
  acceptance phase moved 95 s → 68 s for the same reason.

## Followups (not this stage)

- **Branch-relative negative provenance.** The real consumer of a world's
  commitment: an `absent_premises` entry is only valid under the commitment
  its world assumed, and `Provenance` has nowhere to say so. Needed before
  negative dependencies are trusted across branches (REVIEW_M1-01 §2's
  `Deps(Y)`).
- **DNF expansion for nested `(or …)`.** Now a clear `CompileError` rather
  than a wrong answer; supporting it is a compiler feature.
- **A load-time home for `CompileError`.** Compilation is lazy, so these
  surface during `solve()` rather than at load. A `:match`-shape check in
  `kb.from_ir`'s validator would report them with source locations, next to
  the other load-time diagnostics.
