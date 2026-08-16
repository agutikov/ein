# R4 — `absent` (NAF) semantics: investigation report

**Stage:** [S1.21.4](../s1.21.4_absent_semantics.md) T1.21.4.1 · **Review:**
[REVIEW_M1-01 §4](../../REVIEW_M1-01.md) · **Priority P1** · 2026-08-16.
Probes referenced below (P1–P8) were run against the current `master` with
throwaway scripts (scratch space, not committed); every probe's fixture is
reproduced in §2 so the improvement task can turn each into a pinning test.

## Verdict

**Confirmed.** There is no formal semantics document for `absent`; what exists
is scattered *operational* prose — the fire-time-re-eval narrative in
[`docs/kernel/inference/README.md:153-254`](../../../../docs/kernel/inference/README.md),
one paragraph in
[`architecture_and_algorithms.md:251-271`](../../../../docs/kernel/inference/architecture_and_algorithms.md)
(§O3), a one-line registry entry
([`primitives.py:76-80`](../../../../ein.py/src/ein/inference/primitives.py)) —
plus one **actively wrong** section:
[`01_grammar.md:284-305`](../../../../docs/kernel/ir/03-ein-lang/01_grammar.md)
still teaches "`(absent P)` is evaluated at **enqueue time**, not at fire
time" as a "known limitation", contradicting the S1.5a.1 re-eval that shipped
([`saturator.py:535-541`](../../../../ein.py/src/ein/inference/saturator.py)).
Behaviourally the engine implements the review's candidate **(3)
branch-relative epistemic**: `absent(P)` is a *query* "the current fork-local
saturated world, at this firing's fire time, contains no fact matching P" —
never a ground atom, never revisited after firing (probes P1–P5, §2). It is
**not** closed-world over the final closure (P1: `p` and `q` coexist) and
**not** stratified NAF (P3/P4: unstratifiable programs are accepted and
resolved by queue order). The investigation also surfaced one **new soundness
gap**: the fire-time re-eval walks only `plan.steps`, skipping `AbsentGuard`s
inside the S1.8.A13 `extra_match_plans` or-disjuncts (P8, divergence D5).

## Evidence

The load-bearing sites, each verified by reading the file at the cited lines:

| claim | evidence |
|---|---|
| `(absent P)` compiles to an `AbsentGuard` opcode | [`compile.py:92-102`](../../../../ein.py/src/ein/inference/compile.py) (dataclass), [`compile.py:262-265`](../../../../ein.py/src/ein/inference/compile.py) (`_compile_premise`) |
| Match-time evaluation: parent continues iff sub-plan yields zero matches against the **current KB** | [`match.py:164-172`](../../../../ein.py/src/ein/inference/match.py) (`_run_steps`) |
| Fire-time re-evaluation of top-level guards (S1.5a.1) | [`match.py:248-281`](../../../../ein.py/src/ein/inference/match.py) (`absents_still_pass`), called from [`saturator.py:535-541`](../../../../ein.py/src/ein/inference/saturator.py) (`_apply`), counter `naf_dropped` at [`saturator.py:148`](../../../../ein.py/src/ein/inference/saturator.py) |
| Semi-naive triggering must full-match absent-watching plans (a `forall` can flip false→true) | [`saturator.py:86-115`](../../../../ein.py/src/ein/inference/saturator.py) (`_absent_relations`, incl. `extra_match_plans`), [`saturator.py:398-403,429-434`](../../../../ein.py/src/ein/inference/saturator.py) (`_enqueue_pass` delta rules), caveat on seeding at [`match.py:236-242`](../../../../ein.py/src/ein/inference/match.py) |
| The world a fork's guard queries = root + commitment hypotheses + saturation-so-far | [`commitment.py:104-134`](../../../../ein.py/src/ein/inference/commitment.py) (`fork` → write `h_i` → `Saturator(fork)`) |
| Fork facts never root-merged **because of NAF** | [`monotonic/solver.py:376-388`](../../../../ein.py/src/ein/inference/monotonic/solver.py) ("unconditional-fact extraction is UNSOUND under NAF … a fork fact derived via `absent X` looks unconditional by the provenance walk but actually depends on the commitment having suppressed X") |
| Provenance records **positive premises only** — the absence a firing depended on is invisible | [`firing.py:143-152`](../../../../ein.py/src/ein/inference/firing.py) (`premises_raw` from Scan/Join facts; `AbsentGuard` yields with unchanged `premises`, [`match.py:164-172`](../../../../ein.py/src/ein/inference/match.py)); the trace's "using" line inherits this ([`trace/linearize.py:77`](../../../../ein.py/src/ein/trace/linearize.py)) |
| Deletion-based MUS minimisation unsound under NAF | [`min_core.py:17-25`](../../../../ein.py/src/ein/inference/min_core.py) |
| Static NAF dependency map (advisory, default-off warning) | [`naf_deps.py:1-36,68-77,132-150`](../../../../ein.py/src/ein/inference/naf_deps.py); `warn_derived_naf` default `False` at [`config.py:105`](../../../../ein.py/src/ein/inference/config.py); emitted once post-root-saturation at [`monotonic/solver.py:227-233`](../../../../ein.py/src/ein/inference/monotonic/solver.py) |
| Stale doc contradicting the implementation | [`01_grammar.md:284-305`](../../../../docs/kernel/ir/03-ein-lang/01_grammar.md) ("NAF evaluation timing (known limitation)": enqueue-time-only, "Parked for engine-side resolution in P1.5a" — S1.5a.1 shipped long ago) |
| Existing operational (not formal) prose | [`inference/README.md:153-254`](../../../../docs/kernel/inference/README.md) (§"NAF semantics — fire-time re-evaluation"), [`architecture_and_algorithms.md:251-271`](../../../../docs/kernel/inference/architecture_and_algorithms.md) (§O3, incl. the honest "Gap: no well-founded/stable-model machinery — sound because the puzzle ruleset is effectively stratified"), [`06_reserved_names.md:102,130-131`](../../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md) |
| Existing tests: fire-time drop, per-binding NAF, zebra chain | [`tests/inference/test_saturator_naf.py`](../../../../ein.py/tests/inference/test_saturator_naf.py) (5 tests), plus `test_forall.py` / `test_open.py` / `test_naf_deps.py` / `test_match.py` — all 38 green on master (run 2026-08-16) |

## 1. Implementation inventory — where `absent` semantics is *decided*

Every site that gives `absent` meaning (not merely mentions it), in pipeline
order:

1. **Surface → macro layer.** `forall` / `open` are `std.macro` macros
   expanding at load time to nested absents —
   [`stdlib/macro.ein:16-21`](../../../../ein.py/src/ein/stdlib/macro.ein)
   (`forall ⇒ (absent (and ?G (absent ?B)))`, `open ⇒ (and (absent ?P)
   (absent (not ?P)))`). Stdlib consumers:
   [`elim.ein:43,50`](../../../../ein.py/src/ein/stdlib/elim.ein),
   [`bijection.ein:134,140`](../../../../ein.py/src/ein/stdlib/bijection.ein),
   [`algebra.ein:124,134`](../../../../ein.py/src/ein/stdlib/algebra.ein)
   (the latter's comment at `algebra.ein:174` already reasons about the
   fire-time re-check + priority ordering).
2. **Compiler.** `(absent P)` → `AbsentGuard(sub_steps)` at
   [`compile.py:262-265`](../../../../ein.py/src/ein/inference/compile.py);
   opcode defined at `compile.py:92-102`. Introspection walks:
   `naf_relation_refs` [`compile.py:470-503`](../../../../ein.py/src/ein/inference/compile.py)
   (feeds `naf_deps` + the saturator's absent index).
3. **Matcher — the decision itself.**
   [`match.py:164-172`](../../../../ein.py/src/ein/inference/match.py):
   NAF passes iff the sub-plan yields zero matches under the current bindings
   against the KB *given to the matcher* — the world argument. The guard
   consumes no premise fact (provenance-invisible, C2 below).
4. **Saturator — when the decision is (re)made.**
   Enqueue-time: every `match.run` in `_full_match` / `_seed_match`
   ([`saturator.py:462-473`](../../../../ein.py/src/ein/inference/saturator.py)).
   Delta-triggering: `_absent_relations`
   ([`saturator.py:86-115`](../../../../ein.py/src/ein/inference/saturator.py))
   + the `abs_index` full-match branch
   ([`saturator.py:429-434`](../../../../ein.py/src/ein/inference/saturator.py))
   handle the false→true flip of a *nested* absent (the D2 completeness case).
   Fire-time: `absents_still_pass`
   ([`saturator.py:535-541`](../../../../ein.py/src/ein/inference/saturator.py)
   → [`match.py:248-281`](../../../../ein.py/src/ein/inference/match.py))
   handles the true→false flip between enqueue and dequeue. **After the
   firing: never again** — no retraction, no TMS (P1, §2).
   (The queue-less [`engine.py:129-148`](../../../../ein.py/src/ein/inference/engine.py)
   `Engine.step()` needs no re-eval: it matches fresh per step, so match time
   *is* fire time.)
5. **Search layer — which world.**
   [`commitment.py:104-134`](../../../../ein.py/src/ein/inference/commitment.py):
   the fork's saturator queries `fork = root.fork() + hypothesis facts +
   facts-derived-so-far`. Consequences hard-coded downstream:
   no mid-search root merge
   ([`monotonic/solver.py:376-388`](../../../../ein.py/src/ein/inference/monotonic/solver.py));
   `_is_unconditional`'s positive-provenance walk
   ([`commitment.py:181-196`](../../../../ein.py/src/ein/inference/commitment.py))
   is exactly the walk solver.py declares insufficient under NAF (S1.21.2's
   subject).
6. **Hypgen / lookahead.**
   [`lookahead.py:100-118`](../../../../ein.py/src/ein/inference/lookahead.py):
   the one-step probe runs the rule's *other* premises — including
   `AbsentGuard`s — against `kb` **without** the candidate `h`, while `h` is
   posited into a positive premise: the NAF queries a *different world* than
   the probe hypothesises (divergence D3, probe P6). A kill writes `(not h)`
   via `_write_negated`
   ([`hypgen.py:214-222,320-334`](../../../../ein.py/src/ein/inference/hypgen.py),
   both flags default-on at
   [`config.py:95-96`](../../../../ein.py/src/ein/inference/config.py)).
7. **Verdict coupling.** `complete(kb)`
   ([`solution.py:46-53`](../../../../ein.py/src/ein/inference/solution.py))
   is defined via `generate_hypotheses` → filters → lookahead → matcher, so
   the *solution-node predicate itself* — hence `k`, hence the verdict —
   is downstream of the same epistemic queries.
8. **Contradiction detection interplay.** None directly — `contradiction.py`
   never consults `absent`; the interplay is indirect: absent-guarded rules
   (`typecheck-*`, `domain/range-elimination`) *derive* `(false)` /
   `(not …)` facts the detector then finds.
9. **Rendering.** `AbsentGuard` bodies render as `cluster_absent` (∄/∀)
   sub-graphs — [`render/rules.py:25-26,81,190-196,248-276`](../../../../ein.py/src/ein/render/rules.py).
   The *trace* of an absent-derived firing shows only positive premises
   ([`trace/linearize.py:77`](../../../../ein.py/src/ein/trace/linearize.py));
   the absence survives only in the rule's `:why` text.

## 2. Behavioural characterisation — which semantics is implemented

Probes run on master (2026-08-16), each a minimal `.ein` program through
`Saturator`/`try_commitment_set`/`Lookahead`; existing suites
(`test_saturator_naf.py` + forall/open/naf_deps/match, 38 tests) all green.

| probe | program | result | rules out / establishes |
|---|---|---|---|
| **P1** fire-then-arrive | `p ← seed ∧ absent q` (prio 100); `q ← t` (prio 200); both `seed`,`t` given | final KB has **both** `p` and `q`; `naf_dropped=0` | rules out **closed-world over final closure** (`q ∈ closure` yet `p` derived and kept); no retraction after firing |
| **P2** same program, priorities swapped | gate at 200, derive-q at 100 | `p` absent, `naf_dropped=1` | the *meaning of the program* depends on priority/queue order — semantics is operational, decided at fire time |
| **P3** unstratified loop | `p ← absent q; q ← p` | converges to `{p, q}` | rules out **stable-model** discipline: this program has *no* stable model; final KB is supported-at-fire-time, not stable |
| **P4** mutual NAF | `p ← absent q; q ← absent p` (equal prio) | exactly `{p}` (FIFO tiebreak) | rules out **stratified NAF** (program is unstratifiable, engine accepts it and deterministically picks one of the two stable models by queue order) |
| **P5** root vs fork | `gated ← seed ∧ absent (r X Y)`; commit `{(r X Y)}` via `try_commitment_set` | root: `(gated A)` derived; fork (`kind=alive`): **not** derived | **establishes branch-relative epistemic**: same ground query, different world, different answer — `absent` cannot be a ground atom |
| **P6** lookahead world | rule `false ← (cand ?x) ∧ absent (cand ?x)` (unsatisfiable in any real match); `dies_immediately(kb, (cand A))` | returns **True** | divergence **D3**: the probe's NAF world excludes `h` itself while `h` feeds a positive premise — a candidate can be killed by a rule that could never fire; with the default-on kill cache this writes `(not h)` to the parent |
| **P7** nested `(and …)` under absent | `ok ← seed ?x ∧ absent (and (g ?x ?y) (h ?y))`; witness exists for A only | `ok(B)` only | inner free vars are **existential under the guard**: `absent` = ¬∃ binding-extension; `forall`'s ∀ arises from the double negation |
| **P8** or-disjunct re-eval gap | `gate: (or (t1 ∧ absent r1) (t2 ∧ absent r2))` at 200; `r2 ← raw` at 100; `t2`,`raw` given | `(gated A)` **fires** with `(r2 A)` present, `naf_dropped=0` | divergence **D5** (new, soundness): `absents_still_pass` walks `plan.steps` only ([`match.py:277-281`](../../../../ein.py/src/ein/inference/match.py)) while `run` also yields from `extra_match_plans` ([`match.py:193-195`](../../../../ein.py/src/ein/inference/match.py)) — guards in A13 or-disjuncts get **no** fire-time protection |

**Conclusion.** The engine implements the review's candidate **(3)** with a
sharpening: `absent(P)` is a query evaluated against the fork-local KB **at
the moment the firing is committed**, decisive once, never revisited. It
degenerates to (1) closed-world only in the special case where every producer
of a watched relation runs at strictly lower priority than every watcher
(then fire-time == post-closure for that relation — zebra2's
priority-discipline case, [`inference/README.md:173-181`](../../../../docs/kernel/inference/README.md));
it is never (2) — no stratification exists or is checked.

**Divergences / underspecified corners** (the improvement documents these;
per the stage gate none is silently fixed there):

- **D1 (stale doc, must fix in T1.21.4.2):**
  [`01_grammar.md:284-305`](../../../../docs/kernel/ir/03-ein-lang/01_grammar.md)
  claims enqueue-time-only evaluation, "parked for P1.5a" — S1.5a.1 shipped.
  Also `01_grammar.md:272-274` calls `forall`/`open` "parser sugar"; they are
  `std.macro` load-time macros since S1.5.9
  ([`compile.py:267-272`](../../../../ein.py/src/ein/inference/compile.py)).
- **D2 (order-dependence, must document):** P1–P4 — on non-stratified rule
  sets the final KB is a function of priority bands + FIFO order, and the
  fixpoint is *supported at fire time*, not stable. The doc must state this as
  the contract (with the effectively-stratified-ruleset caveat the
  architecture doc already gestures at).
- **D3 (lookahead world mismatch, phase-README open question):** P6 — the
  one-step lookahead's `dies_immediately` violates its own "never reports a
  live hypothesis as dead" docstring
  ([`lookahead.py:36-39`](../../../../ein.py/src/ein/inference/lookahead.py))
  for rules whose `AbsentGuard` watches the candidate's own relation. M1's
  shipping rule library has no such rule (typecheck/elim guards watch `is-a`,
  candidates are domain relations), so no fixture misbehaves today — but it
  is verdict-affecting in principle via the `(not h)` write-back.
- **D5 (or-disjunct re-eval gap, phase-README open question + xfail pin):**
  P8 — a genuine unsound firing reachable with plain `(or …)` + `(absent …)`
  + priorities; fix is a 3-line extension of `absents_still_pass` to walk
  `plan.extra_match_plans`, but per the gate it is *reported*, pinned
  `xfail(strict=True)`, and fixed in its own follow-up (or explicitly pulled
  into T1.21.4.2 by the phase owner).

## 3. Formal statement draft

For the improvement task to transcribe into
`docs/kernel/inference/absent_semantics.md`:

> **Worlds.** A *world* `W` is one `KnowledgeBase` instance under saturation:
> the root, or a fork `KB_C = fork(root) ∪ {h : h ∈ C}` for a commitment set
> `C` ([`commitment.py:104-114`](../../../../ein.py/src/ein/inference/commitment.py)).
> Within one `saturate()` run, `W` is append-only; `W(t)` denotes its fact
> set after `t` firings. Worlds are related only by fork: nothing evaluated
> in `KB_C` is meaningful in root or in `KB_{C'}` (C6).
>
> **Definition (fire-time epistemic NAF).** For a rule firing considered in
> world `W` at time `t` with bindings `θ`:
>
> `(absent P)` holds ⟺ there is **no** extension `θ' ⊇ θ` such that `P θ'`
> matches a stored fact of `W(t)` — i.e. `W(t) ⊭ ∃x̄. P θ`, where `x̄` are
> `P`'s vars unbound by `θ` (P7) and "matches" is the matcher's raw
> unification ([`match.py:42-91`](../../../../ein.py/src/ein/inference/match.py)),
> **membership, not derivability**: a fact not yet derived at `t` counts as
> absent even if it is in the closure.
>
> **Evaluation points.** (E1) at every match — admission to the queue;
> (E2) decisively re-checked at fire time for the plan's top-level guards
> (`absents_still_pass`); (E3) **never after the firing commits** — no
> retraction, no truth maintenance. Monotonicity within one run: for a
> positive-only `P`, `absent(P)` can flip true→false only (handled by E2);
> for a *nested* absent (`forall`), the outer guard can also flip false→true
> (handled by the saturator's absent-index full-match, never by seeding).
>
> **Corollaries the engine relies on** (each already enforced locally):
> - **C1 — no root-merge:** an alive fork's derived facts may depend on an
>   absence that holds in `KB_C` but not in root's future; they are never
>   merged mid-search ([`solver.py:376-388`](../../../../ein.py/src/ein/inference/monotonic/solver.py)).
> - **C2 — positive provenance is not dependence:** `Provenance.premises_raw`
>   records Scan/Join facts only ([`firing.py:143-152`](../../../../ein.py/src/ein/inference/firing.py));
>   negative dependence is invisible to every provenance walk
>   (`reaches`, `unsat_core`, `_is_unconditional`).
> - **C3 — deletion-based MUS minimisation is unsound:** removing a fact can
>   flip an absent and fabricate a contradiction the full KB never had
>   ([`min_core.py:17-25`](../../../../ein.py/src/ein/inference/min_core.py));
>   single-witness provenance frontiers are used instead.
> - **C4 — fire-time re-eval is required** for any queued executor
>   ([`saturator.py:535-541`](../../../../ein.py/src/ein/inference/saturator.py));
>   a fresh-match executor (`Engine.step`) needs none.
> - **C5 — semi-naive seeding is incomplete for absent-watchers:** a delta
>   inside a guard has no positive premise to seed; such plans must
>   full-match ([`saturator.py:86-115,429-434`](../../../../ein.py/src/ein/inference/saturator.py),
>   [`match.py:236-242`](../../../../ein.py/src/ein/inference/match.py)).
> - **C6 — `absent` is world-relative:** results must not be cached across
>   worlds nor written back as facts (P5).
> - **C7 — the verdict inherits the semantics:** `complete` / `open_hypotheses`
>   ([`solution.py:30-53`](../../../../ein.py/src/ein/inference/solution.py))
>   and hypgen's filters are themselves fire-time epistemic queries.
>
> **Explicitly not provided:** stratification checking (`naf_deps` is
> advisory; `warn_derived_naf` default off), stable/well-founded model
> computation, retraction. On non-stratified inputs the result is
> **defined by operational order** (priority bands, FIFO tiebreak) — P2/P4.

## 4. Doc placement recommendation

**Home: `docs/kernel/inference/absent_semantics.md`** (new file). The
semantics is *engine* semantics — worlds, evaluation times, saturation and
search interplay — and every corollary cites `inference/` internals; the
surface layer only needs the query/atom distinction and a pointer. The
alternative (`docs/kernel/ir/03-ein-lang/`) would strand C1–C7 far from the
machinery they constrain and duplicate the inference README's NAF section a
directory away from it.

Cross-link set (all touched by T1.21.4.2):
[`inference/README.md`](../../../../docs/kernel/inference/README.md) (§NAF
becomes a summary + link),
[`architecture_and_algorithms.md`](../../../../docs/kernel/inference/architecture_and_algorithms.md)
(§O3 link), [`docs/kernel/README.md`](../../../../docs/kernel/README.md)
(index), [`docs/kernel/glossary.md`](../../../../docs/kernel/glossary.md)
(new `absent` / `NAF` / `world` entries — none exist today),
[`01_grammar.md`](../../../../docs/kernel/ir/03-ein-lang/01_grammar.md)
(**replace** the stale D1 section with a pointer),
[`06_reserved_names.md`](../../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
(`absent` row links out), plus `min_core.py` / `monotonic/solver.py` /
`match.py` docstrings pointing at the doc instead of re-arguing locally.
S1.21.2's rewritten unconditional-facts section should cite C1/C2 by anchor;
S1.21.6's seam doc should cite the **Worlds** paragraph.

**Pinning tests** (new `ein.py/tests/inference/test_absent_semantics.py`,
one per documented edge, each naming its doc section):

1. `test_fire_then_arrive_keeps_both` — P1 → §Evaluation points (E3).
2. `test_priority_swap_changes_outcome` — P2 → §not-provided/order-defined.
3. `test_unstratified_loop_converges` — P3 → §not-provided (no stable-model).
4. `test_mutual_naf_picks_queue_order` — P4 → §not-provided (no stratification).
5. `test_absent_is_branch_relative` — P5 → §Worlds / C6 (via `try_commitment_set`).
6. `test_lookahead_naf_world_excludes_candidate` — P6 → §D3 (pins *current*
   behaviour; assertion documents it as an open question, not an endorsement).
7. `test_absent_nested_and_is_existential` — P7 → §Definition.
8. `test_or_disjunct_absent_not_reevaluated_at_fire_time` — P8 → §D5,
   `xfail(strict=True)` so the eventual engine fix flips it loudly.

## Recommendation

**Chosen path:** write `docs/kernel/inference/absent_semantics.md` from §3
verbatim (definition → evaluation points → C1–C7 → non-guarantees → the P1–P8
programs as worked micro-examples), fix the stale D1 grammar section down to
a pointer, add the §4 cross-links and the 8 pinning tests, and file **D3 +
D5 as new open questions in the phase README** (D5 with its 3-line candidate
fix noted: extend `absents_still_pass` to walk `plan.extra_match_plans`).
This keeps T1.21.4.2 behaviour-preserving as its gate demands while making
the semantics executable law.

*Alternatives considered:* (a) grow the existing `inference/README.md` §NAF
in place — rejected: that section is already the operational how, the README
is 695 lines, and S1.21.2/S1.21.6 need a stable anchor to cite; (b) fix D5
inside T1.21.4.2 — rejected by the stage gate ("any divergence … is reported
…, not silently fixed here"), though the phase owner may promote it given it
is a confirmed unsound firing; (c) place the doc in `03-ein-lang/` —
rejected above (§4).

## Improvement inventory

Files T1.21.4.2 will create or modify (exhaustive):

| file | change |
|---|---|
| `docs/kernel/inference/absent_semantics.md` | **new** — the formal doc (§3 + §2 micro-examples) |
| `docs/kernel/inference/README.md` | §NAF (153-254) gains the link + query-vs-atom framing; layout tree at 39-53 gains the file |
| `docs/kernel/inference/architecture_and_algorithms.md` | §O3 (251-271) links the doc |
| `docs/kernel/README.md` | index entry |
| `docs/kernel/glossary.md` | `absent`/`NAF`/`world` entries |
| `docs/kernel/ir/03-ein-lang/01_grammar.md` | replace stale 284-305 with pointer; correct 272-274 sugar→macro wording |
| `docs/kernel/ir/03-ein-lang/06_reserved_names.md` | `absent` rows (35, 102, 130-131) link out |
| `ein.py/src/ein/inference/min_core.py` | docstring (17-25) cites doc §C3 |
| `ein.py/src/ein/inference/monotonic/solver.py` | comment (376-388) cites doc §C1 |
| `ein.py/src/ein/inference/match.py` | `absents_still_pass` docstring cites doc §C4 (and notes the D5 gap until fixed) |
| `ein.py/tests/inference/test_absent_semantics.py` | **new** — the 8 pinning tests (§4) |
| `plans/m1_core_graph_reasoning/p1.21_review_response/README.md` | D3 + D5 recorded as open questions |

Tests to add: the 8 listed in §4 (7 asserting current behaviour, 1 strict
xfail). Gate: `./run_tests.sh` + `ruff check .` green; no engine behaviour
change.

**Risks.** (1) Wording collision with S1.21.2 (C1/C2 are its subject) and
S1.21.6 (Worlds) — both stages must cite this doc's anchors, not restate;
coordinate before merging. (2) `docs/kernel/glossary.md` and
`docs/kernel/README.md` are likely touch-points of R5/R6 improvements —
schedule those waves apart or land this one first. (3) The D5 xfail must be
`strict=True`, else the eventual engine fix silently orphans the doc's D5
paragraph. (4) `01_grammar.md`'s stale section may be link-targeted elsewhere
— grep for `#naf-evaluation-timing` anchors before deleting (none found in
`docs/` today, but plans/ may cite it).
