# `absent` — formal semantics (NAF)

> **Status (2026-08-17).** Written for P1.21 R4
> (REVIEW_M1-01 §4;
> investigation:
> `r4_absent_semantics.md`),
> then **re-grounded by S1.21.8**, which moved NAF evaluation off the
> closure and onto an explicit closure/world boundary. This page is the
> **normative definition** of what `(absent P)` means; the operational how
> stays in the [inference README §NAF](README.md).
> Every claim here is pinned by
> [`naf_semantics.rs`](../../../ein.rs/crates/ein-infer/tests/naf_semantics.rs)
> — the doc is executable law, not prose.

`absent` is what splits Ein into *positive monotone deduction* +
*non-monotone observations of absence*. The one distinction that matters:
`absent(P)` is a **query** over a saturated world — evaluated, answered,
acted on, and gone — **never a ground logical atom** that could be stored,
cached, or carried between worlds. Of the three candidate readings the
review offered (closed-world / stratified NAF / branch-relative
epistemic), the engine implements the third: the worked micro-examples
below rule the other two out.

**What S1.21.8 changed, in one line.** The reading used to be sharpened to
*fire time* — a guard was evaluated against whatever the KB held at the
moment its candidate was dequeued, which is not a world at all, and the
answer moved with rule priority. It is now sharpened to the **positive
fixpoint**: the closure runs to quiescence consulting no negation, and
guards are judged there, against a world. `W ⊭ ∃x̄.Pθ` became literal
rather than approximated, and priority-band discipline dropped from
load-bearing to advisory.

## Worlds

A **world** `W` is one [`KnowledgeBase`](../ir/02-data-model/02_store.md)
instance under saturation: the root, or a fork
`KB_C = fork(root) ∪ {h : h ∈ C}` for a commitment set `C`
([`commitment::try_commitment_set`](../../../ein.rs/crates/ein-infer/src/commitment.rs)
— fork, write the hypothesis facts, hand the fork to a fresh
[`Saturator`](../../../ein.rs/crates/ein-infer/src/saturator.rs)).
Within one `saturate()` run, `W` is **append-only**; `W(t)` denotes its
fact set after `t` firings. Worlds are related only by fork: nothing
evaluated in `KB_C` is meaningful in root or in any sibling `KB_C′`
(corollary C6 below).

## Definition

**Boundary epistemic NAF.** For a rule firing considered in world `W` at a
**positive quiescence** — a point at which the purely positive closure has
run to a fixpoint — with bindings `θ`:

> `(absent P)` holds ⟺ there is **no** extension `θ′ ⊇ θ` such that
> `P θ′` matches a stored fact of `W` — i.e. `W ⊭ ∃x̄. P θ`,
> where `x̄` are `P`'s variables unbound by `θ` (micro-example P7) and
> "matches" is the matcher's raw unification
> ([`Matcher`](../../../ein.rs/crates/ein-infer/src/match_.rs)).

`θ` here is the guard's bindings **projected to its scope** — the
variables bound by the positive premises that preceded it in the rule
([`NafGuard::scope_of`](../../../ein.rs/crates/ein-infer/src/plan.rs)). That
projection is what makes lifting a guard to the boundary exactly as strong
as evaluating it in place: `(and (absent (P ?x)) (Q ?x))` still asks "is
there no `P` at all?", and `(and (Q ?x) (absent (P ?x)))` still asks "is
there no `P` for *this* `x`?".

Three consequences of the wording:

- **Membership, not derivability — but of a saturated store.** `absent`
  reads the store, not the theory. Since the store it reads is a positive
  fixpoint, everything positively derivable *is* there (P1), so the two
  coincide for the positive part of the program. What they still do not
  coincide on is what a *later* boundary round derives: a fact admitted
  through a negation in round `n` is not in the world round `n` judged.
- **Inner free vars are existential.** The guard is ¬∃ over the
  sub-pattern's unbound variables (P7); the `forall` macro's ∀ arises
  from its double negation
  (`(forall ?b G B)` ⇒ `(absent (and G (absent B)))` —
  [`std.macro`](../../../stdlib/macro.ein)).
- **One question, one world.** A guard is asked once per world and its
  answer is used immediately; there is no interval in which it can go
  stale (E2 below).

## Evaluation points

There is **one**, and it is the whole point of the S1.21.8 design.

- **E1 — at the closure/world boundary, once.** A `(absent …)` premise is
  lifted out of its plan at compile time
  ([`compile.rs`](../../../ein.rs/crates/ein-infer/src/compile.rs)), so
  the closure plan the matcher runs is purely positive and a match says
  nothing about negation. A candidate whose disjunct carries guards is
  **parked**. When the closure quiesces, the saturator builds a
  [the boundary phase](../../../ein.rs/crates/ein-infer/src/saturator.rs) over the stalled KB
  and asks it: `World.absent` runs the guard's sub-plan under the projected
  bindings, and the candidate is admitted iff every guard passes. **This is
  the decision, and there is no other.**
- **E2 — retired (was: fire-time re-check).** `match.absents_still_pass` is
  deleted, not bypassed. Exactly one candidate is admitted per boundary
  round, and the closure queue is empty at that moment, so the admitted
  candidate fires immediately against precisely the world its guard was
  judged against. There is no interval in which a verdict can go stale, so
  there is nothing to re-check: `Saturator.naf_dropped` is **structurally
  0**.
- **E3 — never after the firing commits.** Unchanged: no retraction, no
  truth maintenance. What changed is that "the watched fact arrives one
  step later" can no longer happen *within* the positive closure — the
  closure had already finished when the guard was asked.

Monotonicity within one run, and what each case costs:

- A **positive-only** guard is *anti-monotone*: `absent(P)` can flip
  true→false only, so once it fails it fails forever and the candidate is
  **retired** (`Saturator.naf_retired`) rather than re-asked.
- A **nested** absent (`forall`) can also flip false→true — adding a `B`
  makes the inner absent fail and the outer pass — so its candidate stays
  parked and is re-judged at every later quiescence. Re-judging is gated on
  [`NafGuard::watched`](../../../ein.rs/crates/ein-infer/src/plan.rs): if no
  relation the query reads has grown, the verdict cannot have moved and the
  query is not re-run.
- The **absent-index full-match** the old design needed for that flip
  (`saturator._absent_relations`) is gone. Guards do not participate in
  matching at all, so no delta can force a re-match (C5, retired).

The two phases are
[`Saturator::step`](../../../ein.rs/crates/ein-infer/src/saturator.rs), and it
is one loop rather than two: `closure_step` fires purely positive matches from
a priority-banded queue until it is empty, and only then does
`admit_from_boundary` speak. *This paragraph named a queue-less `Engine::step`
until 2026-08-31; that was `ein.py`'s second, simpler loop — `ein.rs`'s
`Engine` is the compile cache and has no `step` (M1e `CD-M1`).*

## Corollaries the engine relies on

Each is already enforced locally; this page is the shared reason.

- **C1 — no root-merge.** An alive fork's derived facts may depend on an
  absence that holds in `KB_C` but not in root's future; they are never
  merged mid-search
  ([`solve.rs`](../../../ein.rs/crates/ein-infer/src/solve.rs)
  "keep root STABLE"; history:
  [README §Unconditional facts — retired](README.md#unconditional-facts--retired-s157--p121-r2)).
- **C2 — negative dependence is now recorded** (was: "positive provenance
  is not dependence"). A firing admitted through the boundary writes the
  queries that had to fail into
  [`Prov::absent`](../../../ein.rs/crates/ein-core/src/prov.rs),
  so `Deps(Y)` — the union of `PositiveDeps(Y)` and `NegativeDeps(Y)` — is
  finally representable. Note what this does and does not buy: the
  dependence is *visible*, which is the precondition for C1 and C3 to be
  revisited, but no walk yet **interprets** it. `unsat_core` and the trace's
  "using" line still read positive premises only.
- **C3 — deletion-based MUS minimisation is still unsound.** Removing a
  fact can flip an absent and *fabricate* a contradiction the full KB never
  had; recorded single-witness frontiers are used instead
  ([`explain.rs`](../../../ein.rs/crates/ein-infer/src/explain.rs)). C2's
  negative premises make a *sound* deletion minimiser conceivable — a
  candidate subset would have to preserve every recorded
  `absent_premises` query as well as the positive ones — but nothing
  implements that, and until something does the caveat stands.
- **C4 — retired.** Fire-time re-eval was required *of a queued executor
  that evaluated guards during matching*. Neither half is true now: guards
  are off the match path, and the boundary admits one candidate at a time
  into an empty queue, so match-time, decision-time and fire-time coincide
  by construction.
- **C5 — retired.** Semi-naive seeding was incomplete for absent-watchers
  because a delta *inside* a guard had no positive premise to seed at. The
  closure no longer contains guards, so seeding is complete for what it now
  covers; the flip a `forall` needs is caught by re-judging parked
  candidates at the boundary, which is both cheaper than the old full-match
  and strictly more complete (it also catches a flip with no delta in the
  watched relation).
- **C6 — `absent` is world-relative.** Results must not be cached across
  worlds nor written back as facts (P5): the same ground query answers
  differently in root and in a fork.
- **C7 — the verdict inherits the semantics.**
  [`complete` / `open_hypotheses`](../../../ein.rs/crates/ein-infer/src/hypgen.rs)
  are defined via `generate_hypotheses` → filters → lookahead → matcher,
  so the solution-node predicate — hence the model count `k`, hence the
  verdict — is downstream of the same boundary epistemic queries.

## Explicitly not provided

- **Stratification checking.**
  [`naf_deps`](../../../ein.rs/crates/ein-infer/src/naf_deps.rs) is
  *advisory* (`warn_derived_naf` defaults off); unstratifiable programs
  are accepted (P3, P4).
- **Stable / well-founded model computation.** The fixpoint is
  *supported at the boundary*, not stable: P3's program has **no** stable
  model yet converges; P4's has two and the engine picks one — by
  boundary-admission order, which is why admission is one candidate per
  round (a batch admission would derive both and answer with a set that is
  not a model at all).
- **Retraction / truth maintenance.** E3. The non-monotonicity lives in
  the search layer — "this assumption led to ⊥, fork without it" — which
  makes the whole system ATMS-shaped, not stratified-Datalog
  ([architecture §O3](architecture_and_algorithms.md#o3--negation-as-failure)).
- **A sound search over a program that *refutes* under an `absent`.** This is
  the narrowed claim, and it is new at M1e
  [S1e.2.3](../../../plans/m1e_review_processing/p1e.2_high/s1e.2.3_naf_refutation_diagnostic.md).
  The search layer's `dead` is documented as **monotone** —
  [design/08 § The objects](../../history/m1a_rust/design/08_parallelism.md):
  *`X ⊆ Y ∧ dead(X) ⇒ dead(Y)`, because the KB is append-only and nothing
  retracts.* Append-only makes saturation **inflationary**; it does not make
  it monotone *in its input*, and `absent` is exactly what separates the two —
  which C3 above already says from the other direction. So the property fails
  on any program whose `(false)` or `(not …)` derivation passes an
  `(absent P)` over a relation the hypothesis generator can still propose:
  `{(p A)}` dies, `{(p A), (q A)}` would live, and three shipped mechanisms
  make sure the second is never reached — the lookahead kill cache, the
  singleton writeback, and the width-1 no-good clause. Reproduced
  ([Q-M1e.9](../../../plans/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent)),
  banked as
  [`examples/ein-bugs/naf-upward-closure.ein`](../../../examples/ein-bugs/naf-upward-closure.ein),
  and **diagnosed rather than fixed**: `warn_derived_naf` emits a
  `RefutationUnderAbsentWarning` naming the rule, the relation and the two
  replacements — a stored-negative scan in `total`'s style, or `(open ?R)` if
  the constraint is a requirement rather than a refutation. Making the three
  consumers world-aware is
  [F18](../../../plans/followups/f18_world_aware_negatives.md), starting from
  `Prov::absent`; whether the shape stays legal at all is
  [S1f.10.8](../../../plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md)'s.
  Note this is **not** the stratification hazard and the two do not overlap: a
  guard over a *rule-derived* relation can flip during saturation, and S1.21.8
  made that sound by judging it at a fixpoint. This one is a *commitment*
  discharging the guard in a world the search never enters.

On **stratified** inputs the result no longer depends on operational order
at all — that is what S1.21.8 bought, and P1/P2 pin it: the same program
with the two rules' priorities swapped now yields the same model. The
degenerate closed-world coincidence the old design needed a *ruleset*
property for (every producer of a watched relation at strictly lower
priority than every watcher — zebra2's priority discipline) is now an
*engine* property, holding for any rule set: the closure is complete
before any guard is asked.

On **non-stratified** inputs the result is still defined by operational
order — now boundary-admission order rather than priority-then-FIFO (P4).
The engine reports one model where several exist and does not say so;
`warn_derived_naf` is the advisory flag for the shape that can cause it,
and a real stratification checker remains future work.

## Worked micro-examples

The investigation's probes P1–P8, each pinned 1:1 by a test in
[`naf_semantics.rs`](../../../ein.rs/crates/ein-infer/tests/naf_semantics.rs).
Programs are sketched `head ← body @priority`; all facts are given at
load.

| probe | program | result | establishes |
|---|---|---|---|
| **P1** fixpoint judgement | `p ← seed ∧ absent q  @100`; `q ← t  @200` | `{q}` only; `naf_dropped=0` | E1 — the guard is judged against the positive fixpoint, in which `q` already holds |
| **P2** priority swap | same rules, `@200`/`@100` | identical to P1 (`{q}`, no `p`) | priority no longer decides what is derivable — band discipline is advisory |
| **P3** unstratified loop | `p ← seed ∧ absent q`; `q ← p` | converges to `{p, q}` | no stable-model discipline — this program has *no* stable model |
| **P4** mutual NAF | `p ← seed ∧ absent q`; `q ← seed ∧ absent p` (equal prio) | exactly `{p}` (first-declared wins) | no stratification check; one of two stable models picked by boundary-admission order — and why admission is one-at-a-time |
| **P5** root vs fork | `gated ← seed ∧ absent (r A B)`; commit `{(r A B)}` | root derives `gated`; the alive fork does **not** | C6 — same ground query, different world, different answer |
| **P6** lookahead world | `false ← (cand ?x) ∧ absent (cand ?x)`; probe `h = (cand A)` | `dies_immediately` returns **False** | D3 **fixed** — the guard is evaluated in the world that includes `h` |
| **P7** nested `(and …)` | `ok ← seed ?x ∧ absent (and (g ?x ?y) (h ?y))`; witness for `A` only | `ok(B)` only | Definition — inner free vars are existential under the guard |
| **P8** or-disjunct guards | `gate: (or (t1 ∧ absent r1) (t2 ∧ absent r2)) @200`; `r2 ← raw @100` | `gated` **not** derived | D5 **fixed** — guards are lifted per disjunct and paired with it by `match.run_guarded` |

## Divergences — both closed by S1.21.8

Both were surfaced by the R4 investigation and recorded in the
phase README
rather than fixed there (the documentation task ran under a
behaviour-unchanged gate). The closure/worlds split fixed both, and both
fixes are pinned.

- **D3 — lookahead world mismatch** (P6). ✅ **fixed 2026-08-17.**
  [`Lookahead::dies_immediately`](../../../ein.rs/crates/ein-infer/src/lookahead.rs)
  used to posit the candidate `h` into a positive premise while running the
  rule's guards against the KB *without* `h`, so a rule watching the
  candidate's own relation could kill a hypothesis it could never refute in
  any one world — and the default-on kill cache wrote `(not h)` back to the
  parent, making it verdict-affecting in principle. The guard is now
  evaluated in the world **with** `h`: it must find no match in `kb` *and*
  `h` must not create one. A guard whose verdict cannot be decided that
  cheaply — one containing a nested absent, which is non-monotone in the KB
  — makes the lookahead skip the disjunct rather than guess, which only
  loses a kill and so keeps the "never reports a live hypothesis as dead"
  contract.
- **D5 — or-disjunct guards skipped the re-check** (P8). ✅ **fixed
  2026-08-17.** `absents_still_pass` walked `plan.steps` only, while
  `match.run` also yielded from the S1.8.A13 `extra_match_plans`
  disjuncts — their guards got no protection at all, a confirmed unsound
  firing that had been pinned `xfail(strict=True)`. It is closed
  *structurally*, not by walking one more tuple: guards are lifted **per
  disjunct** into `plan.naf_guards`, and `match.run_guarded` yields every
  match together with its own disjunct's guards, so there is no longer a
  tuple a caller could forget. The strict xfail is retired.

## Divergence introduced by the fix

- **Non-stratified programs are answered by admission order.** P4's
  `p ← absent q; q ← absent p` has two stable models; the engine picks one
  and does not report that the other exists. This is not new — the old
  engine picked one by priority-then-FIFO — but the *mechanism* is, and it
  is the reason the boundary admits one candidate per round rather than the
  whole batch. Admitting the batch would judge both guards against the
  world in which neither `p` nor `q` holds, admit both, and answer `{p, q}`,
  which is not a model of that program under any reading. A static
  stratification checker is the proper remedy and remains future work; see
  `warn_derived_naf` for the advisory in the meantime.

## Cross-references

- Operational narrative (race, fix, termination, `naf_deps`):
  [README §NAF semantics](README.md#naf-semantics--the-closureworld-boundary-s1218).
- CS positioning (stratified Datalog / well-founded / stable models):
  [`architecture_and_algorithms.md` §O3](architecture_and_algorithms.md#o3--negation-as-failure).
- Surface syntax (`(absent P)` in `:match`; `forall`/`unknown` macros):
  [`01_grammar.md`](../ir/03-ein-lang/01_grammar.md#premise-forms-in-match),
  [`06_reserved_names.md`](../ir/03-ein-lang/06_reserved_names.md).
- Why root stays stable mid-search (C1/C2 in action):
  [README §Unconditional facts — retired](README.md#unconditional-facts--retired-s157--p121-r2).
- Glossary: [`absent` / `world`](../glossary.md).
