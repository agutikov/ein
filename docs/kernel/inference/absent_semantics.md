# `absent` — formal semantics (NAF)

> **Status (2026-08-16).** Written for P1.21 R4
> ([REVIEW_M1-01 §4](../../../plans/m1_core_graph_reasoning/REVIEW_M1-01.md);
> investigation:
> [`r4_absent_semantics.md`](../../../plans/m1_core_graph_reasoning/p1.21_review_response/reports/r4_absent_semantics.md)).
> This page is the **normative definition** of what `(absent P)` means;
> the operational how (the enqueue-vs-fire race, `naf_dropped`, the
> static dependency map) stays in the
> [inference README §NAF](README.md#naf-semantics--fire-time-re-evaluation-s15a1).
> Every claim here is pinned by
> [`tests/inference/test_absent_semantics.py`](../../../ein.py/tests/inference/test_absent_semantics.py)
> — the doc is executable law, not prose.

`absent` is what splits Ein into *positive monotone deduction* +
*non-monotone observations of absence*. The one distinction that matters:
`absent(P)` is a **query** over the current saturated world — evaluated,
answered, acted on, and gone — **never a ground logical atom** that could
be stored, cached, or carried between worlds. Of the three candidate
readings the review offered (closed-world / stratified NAF /
branch-relative epistemic), the engine implements the third, sharpened to
fire-time: the worked micro-examples below rule the other two out.

## Worlds

A **world** `W` is one [`KnowledgeBase`](../ir/02-data-model/02_store.md)
instance under saturation: the root, or a fork
`KB_C = fork(root) ∪ {h : h ∈ C}` for a commitment set `C`
([`commitment.try_commitment_set`](../../../ein.py/src/ein/inference/commitment.py)
— fork, write the hypothesis facts, hand the fork to a fresh
[`Saturator`](../../../ein.py/src/ein/inference/saturator.py)).
Within one `saturate()` run, `W` is **append-only**; `W(t)` denotes its
fact set after `t` firings. Worlds are related only by fork: nothing
evaluated in `KB_C` is meaningful in root or in any sibling `KB_C′`
(corollary C6 below).

## Definition

**Fire-time epistemic NAF.** For a rule firing considered in world `W` at
time `t` with bindings `θ`:

> `(absent P)` holds ⟺ there is **no** extension `θ′ ⊇ θ` such that
> `P θ′` matches a stored fact of `W(t)` — i.e. `W(t) ⊭ ∃x̄. P θ`,
> where `x̄` are `P`'s variables unbound by `θ` (micro-example P7) and
> "matches" is the matcher's raw unification
> ([`match._bind_args`](../../../ein.py/src/ein/inference/match.py)).

Two consequences of the wording:

- **Membership, not derivability.** A fact not yet derived at `t` counts
  as absent even if it is in the closure (P1). `absent` reads the store,
  not the theory.
- **Inner free vars are existential.** The guard is ¬∃ over the
  sub-pattern's unbound variables (P7); the `forall` macro's ∀ arises
  from its double negation
  (`(forall ?b G B)` ⇒ `(absent (and G (absent B)))` —
  [`std.macro`](../../../ein.py/src/ein/stdlib/macro.ein)).

## Evaluation points

- **E1 — at every match.** Each `match.run` — admission of a candidate
  firing to the saturator's priority queue — evaluates the plan's
  [`AbsentGuard`](../../../ein.py/src/ein/inference/compile.py)s against
  the KB of that moment.
- **E2 — decisively re-checked at fire time.** For the plan's top-level
  guards,
  [`match.absents_still_pass`](../../../ein.py/src/ein/inference/match.py)
  re-runs each sub-plan against the *current* KB just before
  [`fire()`](../../../ein.py/src/ein/inference/firing.py); a firing whose
  guard flipped between enqueue and dequeue is dropped
  (`Saturator.naf_dropped`). **This is the decision** — the fire-time
  verdict is the one the semantics is named after.
- **E3 — never after the firing commits.** No retraction, no truth
  maintenance: a committed firing stands even when the watched fact
  arrives one step later (P1).

Monotonicity within one run: for a positive-only `P`, `absent(P)` can
flip **true→false only** (more facts) — handled by E2; for a *nested*
absent (`forall`), the outer guard can also flip **false→true** — handled
by the saturator's absent-index **full-match** on watched-relation deltas
([`_absent_relations`](../../../ein.py/src/ein/inference/saturator.py)),
never by semi-naive seeding (C5). The queue-less
[`Engine.step()`](../../../ein.py/src/ein/inference/engine.py) needs no
E2: it matches fresh per step, so match time *is* fire time.

## Corollaries the engine relies on

Each is already enforced locally; this page is the shared reason.

- **C1 — no root-merge.** An alive fork's derived facts may depend on an
  absence that holds in `KB_C` but not in root's future; they are never
  merged mid-search
  ([`monotonic/solver.py`](../../../ein.py/src/ein/inference/monotonic/solver.py)
  "keep root STABLE"; history:
  [README §Unconditional facts — retired](README.md#unconditional-facts--retired-s157--p121-r2)).
- **C2 — positive provenance is not dependence.** `Provenance.premises_raw`
  records Scan/Join facts only
  ([`firing.py`](../../../ein.py/src/ein/inference/firing.py)); an
  `AbsentGuard` consumes no premise fact, so negative dependence is
  invisible to every provenance walk (`unsat_core`, the trace's "using"
  line, the retired `_is_unconditional`).
- **C3 — deletion-based MUS minimisation is unsound.** Removing a fact
  can flip an absent and *fabricate* a contradiction the full KB never
  had; single-witness recorded frontiers are used instead
  ([`frontier.py`](../../../ein.py/src/ein/inference/frontier.py)).
- **C4 — fire-time re-eval is required** for any queued executor
  ([`Saturator._apply`](../../../ein.py/src/ein/inference/saturator.py) →
  `absents_still_pass`); a fresh-match executor (`Engine.step`) needs
  none.
- **C5 — semi-naive seeding is incomplete for absent-watchers.** A delta
  *inside* a guard has no positive premise to seed at; plans watching the
  delta's relation through an `AbsentGuard` must **full-match**
  ([`saturator._absent_relations`](../../../ein.py/src/ein/inference/saturator.py),
  [`match.run_seeded`](../../../ein.py/src/ein/inference/match.py)'s
  caveat).
- **C6 — `absent` is world-relative.** Results must not be cached across
  worlds nor written back as facts (P5): the same ground query answers
  differently in root and in a fork.
- **C7 — the verdict inherits the semantics.**
  [`complete` / `open_hypotheses`](../../../ein.py/src/ein/inference/solution.py)
  are defined via `generate_hypotheses` → filters → lookahead → matcher,
  so the solution-node predicate — hence the model count `k`, hence the
  verdict — is downstream of the same fire-time epistemic queries.

## Explicitly not provided

- **Stratification checking.**
  [`naf_deps`](../../../ein.py/src/ein/inference/naf_deps.py) is
  *advisory* (`warn_derived_naf` defaults off); unstratifiable programs
  are accepted (P3, P4).
- **Stable / well-founded model computation.** The fixpoint is
  *supported at fire time*, not stable: P3's program has **no** stable
  model yet converges; P4's has two and the engine picks one.
- **Retraction / truth maintenance.** E3. The non-monotonicity lives in
  the search layer — "this assumption led to ⊥, fork without it" — which
  makes the whole system ATMS-shaped, not stratified-Datalog
  ([architecture §O3](architecture_and_algorithms.md#o3--negation-as-failure)).

On non-stratified inputs the result is therefore **defined by operational
order**: priority bands, then FIFO tiebreak (P2, P4). The degenerate case
where `absent` *does* coincide with closed-world: every producer of a
watched relation runs at strictly lower priority than every watcher —
then fire-time = post-closure for that relation. That is zebra2's
priority discipline
([README §NAF](README.md#naf-semantics--fire-time-re-evaluation-s15a1)),
a property of the *ruleset*, not of the engine.

## Worked micro-examples

The investigation's probes P1–P8, each pinned 1:1 by a test in
[`test_absent_semantics.py`](../../../ein.py/tests/inference/test_absent_semantics.py).
Programs are sketched `head ← body @priority`; all facts are given at
load.

| probe | program | result | establishes |
|---|---|---|---|
| **P1** fire-then-arrive | `p ← seed ∧ absent q  @100`; `q ← t  @200` | final KB has **both** `p` and `q`; `naf_dropped=0` | E3 — no revisit after commit; not closed-world over the closure (`q ∈ closure`, yet `p` stands) |
| **P2** priority swap | same rules, `@200`/`@100` | `p` never derived; `naf_dropped=1` | E2 drops the stale firing; the program's *meaning* is order-defined |
| **P3** unstratified loop | `p ← seed ∧ absent q`; `q ← p` | converges to `{p, q}` | no stable-model discipline — this program has *no* stable model |
| **P4** mutual NAF | `p ← seed ∧ absent q`; `q ← seed ∧ absent p` (equal prio) | exactly `{p}` (FIFO: first-declared wins) | no stratification check; deterministic pick of one of two stable models by queue order |
| **P5** root vs fork | `gated ← seed ∧ absent (r A B)`; commit `{(r A B)}` | root derives `gated`; the alive fork does **not** | C6 — same ground query, different world, different answer |
| **P6** lookahead world | `false ← (cand ?x) ∧ absent (cand ?x)`; probe `h = (cand A)` | `dies_immediately` returns **True** | divergence **D3** (below) — the probe's NAF world excludes `h` while `h` feeds a positive premise |
| **P7** nested `(and …)` | `ok ← seed ?x ∧ absent (and (g ?x ?y) (h ?y))`; witness for `A` only | `ok(B)` only | Definition — inner free vars are existential under the guard |
| **P8** or-disjunct gap | `gate: (or (t1 ∧ absent r1) (t2 ∧ absent r2)) @200`; `r2 ← raw @100` | `gated` **fires** with `r2` present; `naf_dropped=0` | divergence **D5** (below) — no fire-time protection for or-disjunct guards |

## Known divergences (open questions, P1.21)

Both surfaced by the R4 investigation, recorded in the
[phase README](../../../plans/m1_core_graph_reasoning/p1.21_review_response/README.md#divergences-surfaced-by-investigation-2026-08-16),
deliberately **not** fixed by the documentation task (behaviour-unchanged
gate):

- **D3 — lookahead world mismatch** (P6, pinned as current behaviour).
  [`Lookahead.dies_immediately`](../../../ein.py/src/ein/inference/lookahead.py)
  posits the candidate `h` into a positive premise while running the
  rule's `AbsentGuard`s against the KB *without* `h` — a rule watching
  the candidate's own relation can kill a hypothesis it could never
  refute in any one world, and the default-on kill cache writes
  `(not h)` back to the parent. M1's shipping rule library has no such
  rule, so no fixture misbehaves today.
- **D5 — or-disjunct fire-time gap** (P8, pinned `xfail(strict=True)`).
  `absents_still_pass` walks `plan.steps` only, while `match.run` also
  yields from the S1.8.A13 `extra_match_plans` disjuncts — their
  `AbsentGuard`s get **no** E2 protection, a confirmed unsound firing.
  Candidate fix (3 lines): walk `plan.extra_match_plans` too. When it
  lands, the strict xfail flips and this paragraph retires.

## Cross-references

- Operational narrative (race, fix, termination, `naf_deps`):
  [README §NAF semantics](README.md#naf-semantics--fire-time-re-evaluation-s15a1).
- CS positioning (stratified Datalog / well-founded / stable models):
  [`architecture_and_algorithms.md` §O3](architecture_and_algorithms.md#o3--negation-as-failure).
- Surface syntax (`(absent P)` in `:match`; `forall`/`open` macros):
  [`01_grammar.md`](../ir/03-ein-lang/01_grammar.md#premise-forms-in-match),
  [`06_reserved_names.md`](../ir/03-ein-lang/06_reserved_names.md).
- Why root stays stable mid-search (C1/C2 in action):
  [README §Unconditional facts — retired](README.md#unconditional-facts--retired-s157--p121-r2).
- Glossary: [`absent` / `world`](../glossary.md).
