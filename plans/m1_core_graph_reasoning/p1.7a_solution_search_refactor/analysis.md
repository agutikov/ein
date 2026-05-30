# P1.7a — Analysis: have / missing / incorrect

The gap between [`target_design.md`](target_design.md) and the engine as it
stands at `9259ea2`. Every claim here is read off the code (path:line). The
read-only items that still need *measurement* (not just reading) are flagged
`→ S1.7a.1` and feed back into this doc.

## What we HAVE (reusable, correct foundations)

The P1.5b unification already built most of the substrate the target needs.

- **One shared loop.** `_explore_layers(root_kb, *, entry, …)`
  (`inference/monotonic/solver.py:612`) is the single per-candidate search;
  the three public entries are thin wrappers fixing `entry`
  (`solver.py:190` monotonic; `gaps_solve`/`contradictions_solve` below). The
  "one search" of the target is *already here* structurally.
- **The completeness predicate, for free.** `_compute_alive(kb)`
  (`solver.py:393`) = `generate_hypotheses` results, symmetric-canonicalised
  = the **open-hypothesis set** (Phase A I4). So `complete(kb)` is
  `not _compute_alive(kb)` — nothing to build.
- **The generator yields exactly the open set.** `generate_hypotheses`
  (`inference/hypgen.py:107`) filters out refuted (`_negated_facts`), present
  (`_already_a_fact`), and lookahead-doomed candidates (Phase A I1). So
  "no open hypothesis" is a faithful completeness test for zebra2's five
  `*-loc` grids.
- **The dedup key.** `state_hash` (`inference/canon.py:27`) — order-insensitive
  over propositional facts, excludes bookkeeping heads (`hypothesis` /
  `contradiction-under`), keeps `(layer, relation, args)` (Phase A I2).
- **Per-branch isolation.** `KnowledgeBase.snapshot()` + `SolutionRecord.kb`
  (`solver.py:961`) already store a stable per-commitment kb. The sound
  solution node is exactly this snapshot — *not* the shared root.
- **Dead-commitment capture.** `contradictions_solve` collects
  `DeadCommitment(commitment, unsat_core, learned_clause, …)`
  (`solver.py:888`) and unions cores in `_finalise_lattice_verdict`.
- **Resource-stop primitives.** `max_set_size` / `max_time` / `max_enterings`
  with `BudgetExceededError` are already plumbed through every entry
  (`solver.py:193,197,198`). These are the budget half of the target's stop
  policy — only `stop_after` is new.
- **`store_lattice` + `kb_index`.** Per-SetNode storage keyed by `state_hash`
  with the multilabel merge (`solver.py:915`, `_record_setnode`) — the dedup
  machinery the target's `S` needs already exists; under `contradictions` it
  merges, under `gaps` it is keyed by `hash(commitment)` (no merge).

## What's MISSING (target needs it; not present)

- **A "solution node" notion.** Nothing computes `consistent ∧ complete`.
  Solutions are recorded on **`is_solved` (goal match)** instead
  (`solver.py:908,940,994`). The complete-filter does not exist.
- **Dedup of the solution set.** `gaps_solve` appends one `SolutionRecord`
  **per satisfying commitment with no state dedup** (`solver.py:961`, Phase A
  I3) — hence the documented **15 records for 1 model** on zebra2
  ([P1.5b acceptance #4](../p1.5b_lattice_search/README.md#lattice-engine--gaps_solve)).
  The target counts **k = |dedup(S)|**; that dedup over solutions is absent
  (the `kb_index` merge is a *storage* feature, not applied to the
  verdict-bearing `solutions` list).
- **Result-type-from-`k`.** No `verdict_of(result)`. Verdict shape is fixed
  by `entry` at the call site: monotonic → trichotomy, gaps → always
  `Ambiguity`, contradictions → always `Contradiction`
  (`_finalise_lattice_verdict`). The count never *decides* the type.
- **`stop_after`.** Only "first goal-match" (monotonic) or "exhaust"
  (gaps/contradictions) exist. No "first N solution nodes". → S1.7a.4.
- **`exhausted` honesty.** The verdict carries no flag for whether the run
  was exhaustive. `contradictions_solve` returns `Contradiction` even if
  `max_set_size` truncated the search (Q-B) — a truncated `k=0` is
  indistinguishable from a real UNSAT. → S1.7a.1 settles the certification
  rule; S1.7a.3 carries the flag.
- **A sound SOLVE answer path.** The CLI `solve` command offers only
  `--mode {gaps,contradictions}` (`cli.py:377`) and dispatches
  `contradictions_solve`/`gaps_solve` (`cli.py:227`). **No CLI path runs a
  sound solve and prints the answer in words** (S1.7.3 T1.7.3.3, still open).
- **`:goal` as projection.** `:goal` is consumed by `is_solved` as a
  *termination* test (`verdict.py:116`); there is no end-of-run projection of
  goal bindings into an answer.

## What's INCORRECT (actively wrong; must change)

- **`is_solved` as the solution/termination signal — the SEVERE bug.**
  `is_solved(kb, mode)` (`verdict.py:116`) tests goal-*pattern* match, not
  model completeness. On zebra2 the house-only goal matches a partial,
  7-cells-wrong grid → `monotonic_solve` returns it as `Solution`
  (`solver.py:940`). **Goal match ≠ solution.**
- **Hardcoded `mode = Mode.SOLVE` inside the shared loop.**
  `_explore_layers` sets `mode = Mode.SOLVE` (`solver.py:673`) and feeds it to
  every `is_solved`. So the "mode" parameter is already vestigial inside the
  core — confirming the target's claim that mode is *not* a search input.
  The vestige needs to be removed cleanly, not left as dead state.
- **`monotonic_solve` is unsound for complete-model finding.**
  `_promote_forced_positives` (`solver.py:337`) commits forced-positives into
  the **shared, append-only root**; driven past first-goal-match it writes
  **wrong** values (Phase A I5: final root 12/25 correct + 7 wrong). Cannot be
  the answer source under full search. Either it stays a strictly-narrower
  "fast first complete node" preset built on per-branch nodes, or it is
  retired. → `refactoring_design.md` decides.
- **`gaps_solve` over-counts → false ambiguity risk.** Without the
  complete-filter + dedup, `len(proof.solutions)` is not `k`. A caller using
  the [P1.5b "check `len(solutions) == 1`"](../p1.5b_lattice_search/README.md#when-to-use-which-entry)
  recipe gets 15, not 1 — the recipe is unsound as written.
- **`Mode` conflates three axes.** `Mode{SOLVE,GAPS,CONTRADICTIONS}`
  (`verdict.py:59`) is simultaneously used as: a search selector (CLI), a
  termination semantics (`is_solved`: SOLVE=exactly-1, GAPS=≥1,
  CONTRADICTIONS=never), and a verdict label. The target splits these; `Mode`
  as it stands cannot survive unchanged.
- **The S1.7.3 completeness-gate experiment** (the
  `SolverConfig.solve_requires_complete_model` flag + the monotonic
  accept-point edits) is a dead-end patch that produced SAT→⊥; it must be
  reverted (S1.7.3a T4), keeping only the harmless pieces (goal_bindings,
  root unsat-core, any `trace/answer.py`, `demo/compare_engines.py`).
  → confirm the current on-disk state of these in S1.7a.1.

## Open questions the analysis cannot close by reading — `→ S1.7a.1`

1. **Open vs all-refuted dead-end (Phase A caveat).** Is zebra2's incomplete
   grid[0] excluded because its cells are **open** (`_compute_alive` non-empty
   ⇒ not complete) or are they **all-refuted** (a totality dead-end that must
   register as a *contradiction*, not silently dropped)? Must be checked on
   the real branch kbs before wiring `complete` in.
2. **Exhaustive `gaps` on all three variants — does it terminate, and what is
   `k`?** zebra2 is measured (15 raw → 2 states → 1 complete). **`zebra2-minus-15`
   drops a constraint** — the exhaustive search could blow up; need the
   commitment count, wall-clock (PyPy), and the distinct-complete-model count
   `k>1`. `zebra2-bad` need `k=0` + the core. This is measurement, not
   reading. (Use a committed script, per project convention — not inline
   one-liners.)
3. **Exhaustiveness-certification rule.** Precise definition of `exhausted`:
   does it require every branch to reach complete-or-dead within
   `max_set_size`, and how does the forced-positive collapse interact with the
   depth cap for a uniquely-solvable puzzle (does zebra2 certify unique
   *without* hitting `max_set_size`)?
4. **Current on-disk state of the S1.7.3 experiment** — what exactly exists to
   revert (grep `solve_requires_complete_model`, `trace/answer.py`,
   `compare_engines.py`).

These four are S1.7a.1's charter; their answers are appended back here as a
"Findings" section before any code in S1.7a.2+ is written.
