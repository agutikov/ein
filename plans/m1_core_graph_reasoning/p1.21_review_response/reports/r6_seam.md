# R6 — The closure/worlds architectural seam; NAF at the boundary

**Task:** [T1.21.6.1](../s1.21.6_architecture_seam.md) ·
**Review point:** [REVIEW_M1-01 §6](../../REVIEW_M1-01.md) (lines 367–402) ·
**Priority:** P2 (architecture doc; no code change in T1.21.6.2).
**Sibling reports:** [r1 (state identity)](r1_state_identity.md),
[r2 (unconditional_facts)](r2_unconditional_facts.md),
[r4 (absent semantics)](r4_absent_semantics.md) — the boundary's contract,
[r5 (ATMS positioning)](r5_positioning.md) — the worlds side's name.

## Verdict

**Confirmed.** The review's target seam — `ein-lang → typed IR → KB`, then
`{monotone closure ∥ assumptions/worlds lattice}` → `complete model` →
`canonical key` → `models/refutations` — is not an aspiration but a
near-literal description of the as-built package layout: every seam node maps
onto one or two modules, and the two-layer split is already documented
([architecture_and_algorithms.md:53–64](../../../../docs/kernel/inference/architecture_and_algorithms.md)).
What is *not* recorded anywhere is (a) the **leak list** — the six places
where the layers interpenetrate, and (b) the review's normative point that
**NAF today sits inside the closure's matcher** (`AbsentGuard` is a
`JoinPlan` opcode, [compile.py:92](../../../../ein.py/src/ein/inference/compile.py),
evaluated in [`match._run_steps:164–172`](../../../../ein.py/src/ein/inference/match.py)
and re-evaluated at fire time,
[saturator.py:535–542](../../../../ein.py/src/ein/inference/saturator.py))
rather than at the closure/world boundary. Every one of the review's §2–§4
pathologies (unsound `unconditional_facts`, NAF-unsound deletion-MUS, the
fire-time re-eval race, the absent-flip semi-naive split) is a downstream
cost of exactly that placement. The seam therefore deserves a recorded home
in `docs/kernel/` plus a P1.9 engineering track and an M3 open question — the
improvement is docs/plans-only, as the stage file specifies.

## Evidence

Decisive cites (each verified by reading the file; layout `claim → file:line`):

- **Two-layer split exists and is stated**: deductive (monotone, append-only)
  vs search (non-monotone) module lists at
  [architecture_and_algorithms.md:53–64](../../../../docs/kernel/inference/architecture_and_algorithms.md);
  the §2 diagram (:70–102) already draws DEDUCTIVE and SEARCH boxes; the
  soundness story ("all non-monotonicity is quarantined in the search layer")
  at :397–402.
- **NAF is a closure opcode**: `(absent P)` compiles to `AbsentGuard`
  ([compile.py:92–103](../../../../ein.py/src/ein/inference/compile.py)) inside
  `JoinPlan.steps` (:106); the matcher evaluates it against the *current
  mid-saturation KB* ([match.py:164–172](../../../../ein.py/src/ein/inference/match.py));
  the saturator re-evaluates at fire time via `absents_still_pass`
  ([saturator.py:535–542](../../../../ein.py/src/ein/inference/saturator.py) →
  [match.py:248–281](../../../../ein.py/src/ein/inference/match.py)) and counts
  drops in `naf_dropped` (saturator.py:148); semi-naive enqueue must
  special-case relations inside AbsentGuards — full-match, never seed
  (saturator.py:86–115 `_absent_relations`, :429–434).
- **The worlds layer's state lives inside the KB (the ground-atom store)**:
  `_nogoods` is a `KnowledgeBase` field
  ([store.py:148–156](../../../../ein.py/src/ein/kb/store.py)) **shared by
  reference into every closure fork** (store.py:649–651; `snapshot` copies it,
  :700); `_negated_facts` (store.py:193) is simultaneously a closure index
  (contradiction detection, matcher) and, per its consumer's own docstring,
  "*IS the dead-hypothesis cache*"
  ([hypgen.py:345–355](../../../../ein.py/src/ein/inference/hypgen.py)).
- **Worlds → closure fact injections with synthetic provenance**: singleton
  nogood writeback `<monotonic-unconditional>`
  ([_helpers.py:105–123](../../../../ein.py/src/ein/inference/monotonic/_helpers.py)),
  lookahead kill-cache `<lookahead-dies-immediately>`
  ([hypgen.py:320–335](../../../../ein.py/src/ein/inference/hypgen.py)),
  forced-positive `<forced-positive>` — whose empty-premise rule provenance is
  *chosen so that* `commitment._is_unconditional`'s walk treats it as a
  non-hypothesis terminal (_helpers.py:141–147, 157–171).
- **The "complete model" node re-enters the worlds layer and can mutate the
  model under test**: `complete(kb)` ≡ "hypgen proposes nothing"
  ([solution.py:46–53](../../../../ein.py/src/ein/inference/solution.py));
  `generate_hypotheses` runs the one-step lookahead and, with the
  **default-on** `enable_lookahead_kill_cache`
  ([config.py:95–96](../../../../ein.py/src/ein/inference/config.py)), writes
  `(not h)` facts into the KB being checked (hypgen.py:214–222 → :320–335).
  The solver calls `complete(result.kb)` at
  [solver.py:337](../../../../ein.py/src/ein/inference/monotonic/solver.py)
  *before* `_record_node` hashes that same KB
  ([_helpers.py:331](../../../../ein.py/src/ein/inference/monotonic/_helpers.py)) —
  so the canonical key is computed over a fact set the completeness *query*
  may have just extended.
- **NAF's misplacement is what broke `unconditional_facts`**: the live solver
  refuses the merge because "extraction is UNSOUND under NAF (`absent`) — a
  fork fact derived via `absent X` looks unconditional by the provenance walk
  but actually depends on the commitment having suppressed X"
  ([solver.py:376–388](../../../../ein.py/src/ein/inference/monotonic/solver.py));
  same root cause blocks deletion-MUS
  ([min_core.py:17–25](../../../../ein.py/src/ein/inference/min_core.py)).
  Two docs still assert the pre-P1.7a merge —
  [architecture_and_algorithms.md:126–128, 355–358](../../../../docs/kernel/inference/architecture_and_algorithms.md)
  and [inference/README.md:520–553](../../../../docs/kernel/inference/README.md)
  — that is [S1.21.2](../s1.21.2_unconditional_facts.md)'s remit; the seam doc
  must link, not restate.
- **The review's own words**: "NAF должен сидеть **на границе closure/world**,
  а не выглядеть обычной разновидностью positive premise. Это сделает
  дальнейшую SMT-интеграцию значительно понятнее"
  ([REVIEW_M1-01.md:400–402](../../REVIEW_M1-01.md)).

## 1. Module mapping — `src/ein/` onto the seam diagram

| seam node | modules | clean / leaking |
|---|---|---|
| **ein-lang → typed IR** | [`ir/parser.py`](../../../../ein.py/src/ein/ir/parser.py), `ir/ast.py`, `ir/types.py`, `ir/macros.py` | **Clean.** `ir/` depends on nothing ([architecture.md:67](../../../../docs/kernel/architecture.md)). |
| **typed IR → KB (ground atoms)** | [`kb/from_ir.py`](../../../../ein.py/src/ein/kb/from_ir.py), [`kb/store.py`](../../../../ein.py/src/ein/kb/store.py), `kb/entities.py`, [`kb/provenance.py`](../../../../ein.py/src/ein/kb/provenance.py) | **Leaks L2** — the store carries worlds-layer state: `_nogoods` (store.py:148–156; fork-shared by reference :651), `_negated_facts` doubling as the dead-hypothesis cache (:193; hypgen.py:350–351), `config` (:140–146, read by the Saturator at saturator.py:172–175). |
| **monotone closure (Datalog-ish)** | [`inference/compile.py`](../../../../ein.py/src/ein/inference/compile.py) (JoinPlan opcodes :57–106), [`match.py`](../../../../ein.py/src/ein/inference/match.py), [`saturator.py`](../../../../ein.py/src/ein/inference/saturator.py), [`engine.py`](../../../../ein.py/src/ein/inference/engine.py) (`_fired` :43, `compile_all` :81), `firing.py`, [`contradiction.py`](../../../../ein.py/src/ein/inference/contradiction.py) (`detect` :126), `predicates.py`, `primitives.py`, `resolve.py` | **Leaks L1** — NAF inside the closure: `AbsentGuard` in `_run_steps` (match.py:164–172), fire-time re-eval (saturator.py:535–542), absent-flip full-match split (saturator.py:86–115, 429–434). The closure is *not* purely positive; its output on a branch depends on what the branch's world lacks. The `__symmetric__` native mirror (saturator.py:309–388) is closure-internal — fine. |
| **assumptions / worlds lattice** | [`monotonic/solver.py`](../../../../ein.py/src/ein/inference/monotonic/solver.py) + [`_helpers.py`](../../../../ein.py/src/ein/inference/monotonic/_helpers.py) + [`_state.py`](../../../../ein.py/src/ein/inference/monotonic/_state.py) + `lattice.py`, [`commitment.py`](../../../../ein.py/src/ein/inference/commitment.py), [`apriori.py`](../../../../ein.py/src/ein/inference/apriori.py), [`nogoods.py`](../../../../ein.py/src/ein/inference/nogoods.py), [`hypgen.py`](../../../../ein.py/src/ein/inference/hypgen.py), [`lookahead.py`](../../../../ein.py/src/ein/inference/lookahead.py), `hrule.py`, [`closed.py`](../../../../ein.py/src/ein/inference/closed.py), [`naf_deps.py`](../../../../ein.py/src/ein/inference/naf_deps.py) (static boundary analysis) | **Clean core, leaking rim.** Clean: `try_commitment_set` is a pure-with-fork world transition — assumptions enter the closure substrate as provenance-tagged facts (commitment.py:104–114), fresh `Saturator` per fork (:133–134); `apriori`/`nogoods` are pure set arithmetic, "no kb inspection" (apriori.py:8–11). Leaks **L3** (synthetic-provenance root writebacks, _helpers.py:105–123 + 157–171, hypgen.py:320–335), **L6** (`unconditional_facts` extraction, commitment.py:149–165 — a worlds-layer question asked of positive-only closure provenance; refused by its only would-be consumer, solver.py:376–388 — see [r2](r2_unconditional_facts.md)). |
| **complete model** | [`solution.py`](../../../../ein.py/src/ein/inference/solution.py) (`complete`/`open_hypotheses` :30–53, `is_solution_node` :61–63) | **Leaks L4** — completeness is defined *operationally through the worlds-layer generator*; evaluating it can mutate the KB (lookahead kill cache, hypgen.py:214–222) and consults the `__closed__` CWA markers (hypgen.py:358–370) which arrive from three producers: authored, the stdlib rule ([`stdlib/closure.ein:33`](../../../../ein.py/src/ein/stdlib/closure.ein)), or `emit_closed` — which the solve path never calls (only the CLI `--hyp-stats` preview, [cli/solve.py:250–255](../../../../ein.py/src/ein/cli/solve.py)). |
| **canonical key** | [`canon.py`](../../../../ein.py/src/ein/inference/canon.py) (`state_hash` :14–32) | Directionally clean (worlds computes it over a closed KB, _helpers.py:331) but **R1's** identity problem lives here (hash-as-identity; `f.layer.value` in the key, canon.py:30), and L4 taints its input (the hashed fact set may include completeness-check writebacks). See [r1](r1_state_identity.md) / [S1.21.1](../s1.21.1_state_identity.md). |
| **models / refutations** | [`verdict.py`](../../../../ein.py/src/ein/inference/verdict.py) (:47–118), [`_state.py:verdict_of`](../../../../ein.py/src/ein/inference/monotonic/_state.py) (:128–153), [`min_core.py`](../../../../ein.py/src/ein/inference/min_core.py), `trace/`, goal projection `goal_bindings` (verdict.py:124–149) | **Clean.** Verdict is read off `k`; the query `:goal` only projects over the model afterwards. |

**Leak list** (the seam doc's payload; L5 is a doc, not code, leak):

- **L1 — NAF inside the closure matcher** (match.py:164–172; saturator.py:535–542,
  86–115, 429–434). The headline; §2 below.
- **L2 — worlds state stored in the KB**: `_nogoods` field + fork-by-reference
  (store.py:148–156, 651); `_negated_facts` double duty (store.py:193;
  hypgen.py:350–351); `kb.config` (store.py:140–146).
- **L3 — worlds → root fact injections keyed by magic provenance strings**:
  `<monotonic-unconditional>` (_helpers.py:117–122),
  `<lookahead-dies-immediately>` (hypgen.py:334),
  `<forced-positive>` (_helpers.py:163–165, with the :141–147 comment
  admitting the string is chosen to steer `_is_unconditional`). Each is
  individually sound, but each is an unannounced world transition whose
  closure consequences are re-derived by ad-hoc re-saturations
  (solver.py:410–421; _helpers.py:171).
- **L4 — `complete()` re-enters the worlds layer and mutates during
  evaluation** (solution.py:46–53 → hypgen.py:214–222; default-on per
  config.py:95–96).
- **L5 — docs still claim the reverse merge** (architecture_and_algorithms.md:126–128,
  355–358; inference/README.md:520–553) — S1.21.2's overlap.
- **L6 — `unconditional_facts`**: positive-only provenance cannot express
  NAF dependence (commitment.py:149–165 vs solver.py:376–388) — the concrete
  price of L1 (no `NegativeDeps` object exists).

## 2. Where NAF actually sits, and what "NAF at the boundary" would take

**Today.** `(absent P)` is evaluated three times, all *inside* the closure and
all against a transient mid-saturation KB: (i) at enqueue-time matching
(match.py:164–172); (ii) re-evaluated at fire time to close the enqueue/fire
race (saturator.py:535–542 → `absents_still_pass`, match.py:248–281 — the
S1.5a.1 story at [inference/README.md:153–254](../../../../docs/kernel/inference/README.md));
(iii) implicitly by the semi-naive enqueue, which must full-match any plan
whose delta relation lands inside an AbsentGuard because an absent may *flip*
(saturator.py:86–115, 390–439). Soundness rests on the monotone-growth
argument (match.py:270–276) plus priority-band discipline
(inference/README.md:173–181), with `naf_deps.py` (:53–65) as a static map of
which rules depend on the fire-time re-eval. The world a guard queries is
therefore *"the KB as of this dequeue"* — not the saturated world `W` that
[S1.21.4](../s1.21.4_absent_semantics.md)'s candidate semantics (3)
(`KB_C ⊭ P`, branch-relative epistemic) names.

**Engineering steps for "NAF at the boundary"** (recorded, NOT executed —
this is the P1.9 track):

1. **Compile split** — lift `AbsentGuard`s out of `JoinPlan.steps`
   (compile.py:92–106) into a separate `plan.naf_guards`; the residue is a
   purely positive Scan/Join/Guard plan; `match._run_steps` drops its
   AbsentGuard arm.
2. **Alternating (two-phase) saturation** — inner loop fires only positive
   plans to quiescence; *at quiescence* evaluate every parked NAF-guarded
   binding against the stalled KB (= the world `W`); admissions re-enter the
   inner loop; outer loop to fixpoint. Retires the enqueue-time NAF eval, the
   fire-time re-check, and the absent-flip full-match split in one move;
   guards are only ever judged against a positive fixpoint, making the
   `KB_C ⊭ P` reading literal and demoting priority-band protection from
   load-bearing to advisory.
3. **A boundary contract type** — a `World` view (saturated fork KB + its
   commitment `C`) exposing `holds(pattern)` / `absent(pattern, bindings)`;
   guard evaluation goes through it exclusively; S1.21.4's semantics doc is
   this contract's spec.
4. **Negative provenance** — firings admitted through the boundary record
   their guards as negative premises (`Provenance` grows an
   `absent_premises` field: `Deps(Y) = PositiveDeps(Y) ∪ NegativeDeps(Y)`,
   the review §2 formula). This is the missing object that made
   `unconditional_facts` unsound (r2) and deletion-MUS unsound
   (min_core.py:17–25, r3's territory); with it both become revisitable.
5. **Re-eval points at world transitions** — each root writeback (`(not h)`
   singleton death, forced-positive promotion, lookahead kill-cache) is a
   world transition; the boundary re-evaluates parked guards there, replacing
   the ad-hoc re-saturations (solver.py:410–421; _helpers.py:171). Fork entry
   already re-evaluates trivially (fresh saturator per fork,
   commitment.py:133).
6. **Measurement gates** — `Saturator.naf_dropped` → 0 by construction;
   `naf_deps` derived-NAF warning retires; branch-determinism
   (`tests/inference/test_branch_determinism.py`) and the parity fixtures
   byte-identical before/after.

Sequencing: gated on S1.21.4's semantics doc (the boundary needs its contract
written before code moves); belongs in P1.9 as an engineering entry, not in
T1.21.6.2.

## 3. M3 implication sketch — seam edge → SMT counterpart

Feeding [`plans/m3_smt_integration/`](../../../m3_smt_integration/README.md)
(phases P3.1–P3.5, [open_questions Q25–Q29](../../../m3_smt_integration/open_questions.md)):

| seam edge | SMT counterpart | M3 phase |
|---|---|---|
| ein-lang → typed IR → KB | sort + relation declarations, ground-atom assertions (Q26: ints + bounds) | P3.1, P3.2 |
| monotone closure | rules as quantified Horn axioms (EUF + Datalog-style theories / E-matching), or engine-side pre-grounding; the closure fixpoint is exactly the fragment an SMT/ASP backend natively saturates | P3.1 |
| **NAF boundary** | **the reason the boundary must be explicit**: SMT has no NAF. Sound translation of `(absent P)` is `¬∃…P` **only under a Clark-completion axiom whose scope is exactly the boundary's world** `W`. `naf_deps`' `declared_only` vs `derived` split (naf_deps.py:53–65) tells which relations get a finite completion axiom vs which need stratified/ASP treatment (clingo as alternate backend). NAF buried inside closure plans makes the translation non-compositional — this is the review's "SMT-интеграция значительно понятнее" claim, and it checks out | P3.1/P3.2 |
| assumptions / worlds lattice | assumption literals + `check-sat-assuming`; `DeadCommitment.unsat_core` ↔ solver unsat cores over assumptions; `_nogoods` ↔ learned clauses; Apriori layer-BFS replaced by the solver's internal search or AllSAT enumeration with blocking clauses | P3.3, P3.4 |
| complete model | `(check-sat)` + `(get-model)`; `complete` (no open hypothesis) ↔ totality of the model over the declared cells | P3.3 |
| canonical key | model-equivalence / blocking-clause construction over the model's atoms — **requires r1's `StateKey`** (a canonical fact tuple), since a Python `int` hash cannot be turned into a blocking clause | P3.3 |
| models / refutations | k = 1 / >1 / 0 ↔ sat + unique / sat + enumeration / unsat + core; explanation recovery lifts cores back to the provenance source frontier | P3.3, P3.4 |

Recommended M3 artifact: one new open question (next free number, **Q30 —
"Seam ↔ SMT mapping: Clark completion at the NAF boundary; assumptions ↔
check-sat-assuming; StateKey ↔ blocking clauses"**) in
[`open_questions.md`](../../../m3_smt_integration/open_questions.md),
cross-linking this report and the future seam section.

## 4. Placement recommendation

**Recommended home: a new "§ The closure/worlds seam (target architecture)"
section in [`docs/kernel/architecture.md`](../../../../docs/kernel/architecture.md).**
The stage file's fork ("extend `inference/README.md` vs a *new*
`docs/kernel/architecture.md`") is resolved by the fact that
`architecture.md` **already exists** (created by P1.20 S1.20.F, commit
`cdce8c3`) as the structural where-do-I-look map for engine contributors —
exactly the audience and altitude of a seam + leak-list + target-picture
section. Reasons over the alternative:

- `inference/README.md` and `architecture_and_algorithms.md` are the two
  files S1.21.2 (unconditional-merge rewrite) and S1.21.5 (CDCL→ATMS
  repositioning) will edit heavily; putting R6's new section in
  `architecture.md` minimises the P2-wave collision surface (phase README
  [Scheduling](../README.md) already flags the shared-file overlap).
- `architecture_and_algorithms.md` §2 already carries the *as-built* two-box
  diagram; duplicating a second big diagram there would blur as-built vs
  *target*. The seam section is normative ("where NAF should sit"), which
  fits `architecture.md`'s milestone-boundaries framing (:77–101).
- Cross-refs (one line each, low-conflict): from
  `inference/README.md`'s NAF-semantics section and from
  `architecture_and_algorithms.md` §2 → the new seam section; the seam
  section links S1.21.4's `absent_semantics.md` (once it exists) as the
  boundary's contract, per the stage file.

**Follow-up entries:**

- **P1.9** — one new catalog row + stub
  (`s1.9.e25_boundary_naf.md`): *"Purely-positive closure + boundary NAF
  re-eval"* — the §2 step list above; 📅 parked; effort L; value H
  (correctness-architecture); activation = S1.21.4 doc shipped **and** a
  measured signal (`naf_dropped > 0` on a real puzzle, or a rule library the
  priority-band discipline can't protect). Precedent for a new
  section-with-📌/📅-rows: the "Deductive-layer perf" section
  ([p1.9 README:149–167](../../p1.9_hypothesis_loop_followups/README.md)).
- **M3** — the Q30 entry above (open_questions.md only; the M3 README's
  Open-questions pointer needs no edit).

## Recommendation

Execute T1.21.6.2 as **docs/plans-only**, in this shape: (1) add the seam
section to `docs/kernel/architecture.md` — the review's diagram, the §1
module-mapping table, the L1–L6 leak list, and an explicit statement that
NAF's *target* position is the closure/world boundary with S1.21.4's doc as
the boundary contract; (2) one-line cross-refs from
`docs/kernel/inference/README.md` and
`docs/kernel/inference/architecture_and_algorithms.md` §2; (3) the P1.9 E25
row + stub file; (4) the M3 Q30 entry. Do **not** fix L5's stale merge claims
(S1.21.2 owns those lines) and do not restate `absent` semantics (S1.21.4
owns the definition — link it). Alternatives considered: extending
`inference/README.md` (rejected: heaviest collision surface, wrong altitude);
extending `architecture_and_algorithms.md` §2 in place (rejected: conflates
as-built with target; S1.21.5 edits the same section); a brand-new
`docs/kernel/seam.md` (rejected: `architecture.md` is the established
structural home; a third architecture file fragments the story).

## Improvement inventory

Files T1.21.6.2 will create or modify (exhaustive; repo-relative):

| file | change |
|---|---|
| `docs/kernel/architecture.md` | new "closure/worlds seam" section: diagram + module map + L1–L6 + NAF target position + M3 pointer |
| `docs/kernel/inference/README.md` | one cross-ref line in the NAF-semantics section → the seam section (**shared with S1.21.2/.3/.5 — serialise P0→P1→P2 per phase README**) |
| `docs/kernel/inference/architecture_and_algorithms.md` | one cross-ref line in §2 → the seam section (**shared with S1.21.2/.5**) |
| `plans/m1_core_graph_reasoning/p1.9_hypothesis_loop_followups/README.md` | new catalog row E25 (boundary NAF) |
| `plans/m1_core_graph_reasoning/p1.9_hypothesis_loop_followups/s1.9.e25_boundary_naf.md` | **new** stub: the §2 step list, activation criteria, measurement gates |
| `plans/m3_smt_integration/open_questions.md` | new Q30 (index row + section) |
| `plans/m1_core_graph_reasoning/p1.21_review_response/s1.21.6_architecture_seam.md` | status note: investigation done → improvement executed |

**Tests to add:** none — no code changes, no behaviour change. Gate =
`./run_tests.sh` + `ruff check .` green (unchanged behaviour trivially) +
manual link-check of every edited doc.

**Risks:**

- *Edit collisions* on the two shared `docs/kernel/inference/` files with the
  S1.21.2/.3/.5 improvement waves — mitigated by keeping R6's touch there to
  one cross-ref line each and scheduling after the P0/P1 waves.
- *Wording drift vs S1.21.4*: the seam doc must reference the (not yet
  written) `absent_semantics.md`; if S1.21.4's improvement lands later, use a
  forward link to the stage file and let S1.21.4's wave retarget it.
- *Stale-claim adjacency*: the seam section sits near
  architecture_and_algorithms.md's L5 lines; if S1.21.2's wave has not yet
  landed, the new section must not silently contradict the old text — the
  cross-ref line should be neutral ("target seam: see architecture.md §…").
- *Over-promising*: the seam section describes a **target**; it must label
  the §2 steps as the parked P1.9/E25 track, not as shipped behaviour —
  otherwise P1.21 would recreate the very claims-vs-guarantees gap the
  review is about.
