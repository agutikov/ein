# R5 — Lattice search positioning: ATMS-style environment search, CDCL as analog

**Review point:** [REVIEW_M1-01 §5](../../REVIEW_M1-01.md) (lines 314–363; also
the summary indictment at line 410 listing *CDCL* among the words that
"promise stricter properties than are actually guaranteed").
**Stage:** [s1.21.5_lattice_positioning.md](../s1.21.5_lattice_positioning.md)
(T1.21.5.1, investigation). **Status: investigation complete, read-only.**

## Verdict

**Confirmed.** The engine's learned object on a dead commitment is the **whole
dead environment** — `emit_nogood(ctx.root_kb, frozenset(c), min_size=1)` at
[`_helpers.py:430`](../../../../ein.py/src/ein/inference/monotonic/_helpers.py)
and `learned_clause=frozenset(c)` at
[`_helpers.py:441`](../../../../ein.py/src/ein/inference/monotonic/_helpers.py),
enforced *by contract*:
[`contract.py:102`](../../../../ein.py/src/ein/inference/monotonic/contract.py)
asserts `d.learned_clause == frozenset(d.commitment)` for every recorded dead.
There is no decision trail, no implication-graph conflict analysis, no
asserting clause, no backjump — the loop is exactly the review's
`set C → fork → saturate → dead → learn C → suppress supersets`
([`solver.py:283–428`](../../../../ein.py/src/ein/inference/monotonic/solver.py)).
The accurate positioning is the review's own formula — **ATMS-style
environment search with Apriori candidate generation and nogood learning** —
and the deeper docs
([`architecture_and_algorithms.md:327–361`](../../../../docs/kernel/inference/architecture_and_algorithms.md))
already say this honestly ("plain BFS backtracking, no backjump"; "Gap: no
backjumping … no watched-literals"). The drift is confined to ~20 shorthand
sites (headings, badges, comments, one summary sentence) that say "CDCL" *as
the mechanism name*; the fix is prose-only.

## Evidence

**What is learned on a dead commitment — the full set, never a core.**

- Emit site (the only one in the engine — `grep emit_nogood` over
  `ein.py/src` hits only here and the definition):
  [`_helpers.py:429–430`](../../../../ein.py/src/ein/inference/monotonic/_helpers.py)
  `if ctx.cfg.enable_path_nogoods: landed = emit_nogood(ctx.root_kb, frozenset(c), min_size=1)`.
- Recorded clause:
  [`_helpers.py:438–445`](../../../../ein.py/src/ein/inference/monotonic/_helpers.py)
  `DeadCommitment(commitment=c, unsat_core=result.unsat_core, learned_clause=frozenset(c), …)` —
  the `unsat_core` rides along as a *separate* field, never fed to the nogood.
- Contract pin:
  [`contract.py:56–59`](../../../../ein.py/src/ein/inference/monotonic/contract.py)
  (invariant 3) and the assert at
  [`contract.py:102–105`](../../../../ein.py/src/ein/inference/monotonic/contract.py).
- No minimisation path exists: `min_core.minimal_unsat_core`
  ([`min_core.py:36–51`](../../../../ein.py/src/ein/inference/min_core.py))
  is consumed by the verdict/trace only
  ([`cli/solve.py:172`](../../../../ein.py/src/ein/cli/solve.py)); nothing in
  [`nogoods.py`](../../../../ein.py/src/ein/inference/nogoods.py) or
  [`apriori.py`](../../../../ein.py/src/ein/inference/apriori.py) touches
  `unsat_core`.
- Consumption is a pre-fork subset filter, not propagation:
  [`apriori.py:88–94`](../../../../ein.py/src/ein/inference/apriori.py)
  (`filter_candidate` drops a candidate iff some clause ⊆ candidate), invoked
  per layer at [`solver.py:293–297`](../../../../ein.py/src/ein/inference/monotonic/solver.py).
- The store is kept **subsumption-minimal** (a real, honest property):
  [`nogoods.py:68–82`](../../../../ein.py/src/ein/inference/nogoods.py).
- The "learn a smaller clause" direction was *investigated and closed*:
  [P1.9 E7](../../p1.9_hypothesis_loop_followups/s1.9.e7_learned_clause.md)
  found the pruning half **measured vacuous** (all 49 zebra2 deaths are
  singletons — E7 §2, lines 33–45: cardinality BFS + Apriori downward closure
  means every emitted nogood is already subset-minimal among explored sets)
  **and unsound under NAF** (E7 §3, lines 47–53: a superset commitment can
  flip an `(absent P)` the ⊥ relied on; `Provenance.premises_raw` records
  positive premises only). So "learn full C" is a deliberate, measured
  decision — not an unfinished CDCL.
- Tests pin the behaviour and are green:
  [`test_monotonic_cdcl.py`](../../../../ein.py/tests/inference/monotonic/test_monotonic_cdcl.py)
  (6 passed, run 2026-08-16).

**Where the mechanism description is already honest.**
[`architecture_and_algorithms.md`](../../../../docs/kernel/inference/architecture_and_algorithms.md)
§6-O7 (lines 327–339: "literal Apriori … forgoes per-variable CDCL's strength
(VSIDS activity, 1UIP learning, non-chronological backjumping — Ein does
*plain BFS backtracking*, no backjump)") and §6-O8 (lines 350–361:
"CDCL-*flavoured* … **Gap:** no backjumping (plain BFS), no VSIDS-style
activity ordering … no watched-literals") already state exactly what the
review asks for. The one mechanism-overclaim in that file is the summary
reframing at lines 192–193: "the search layer **is** a CDCL/CSP solver with an
ATMS underneath".

## 1. Claim census

Adjacent terms swept: `CDCL`, `conflict clause`, `backjump`, `implication
graph`, `asserting clause`, `stratified`. Historical stage ledgers
(`plans/*/p1.5*`, `p1.7*`, `p1.20/s1.20.*`, `STATUS.md`, `open_questions.md`,
P1.9 stage bodies) are **excluded — they stay as history** per the stage spec.
Classification: **analog** = keep as-is (explicitly comparative), **mech** =
misleading-as-mechanism → reword, **mild** = borrowed vocabulary worth a
one-word fix.

### Active docs

| site | quote (abridged) | class |
|---|---|---|
| [`README.md:32`](../../../../README.md) | "(Datalog · CDCL/CSP · ATMS · Apriori) — is mapped in…" | **mech** — the badge reads as a mechanism list with CDCL second; reorder + demote CDCL to analog |
| [`docs/kernel/inference/README.md:26`](../../../../docs/kernel/inference/README.md) | "analogs in other CS fields (Datalog / RETE / CDCL / ATMS / e-graphs)" | **analog** — literally labelled "analogs"; keep |
| [`docs/kernel/inference/README.md:45`](../../../../docs/kernel/inference/README.md) | as-built tree: "…their CS analogs (Datalog / RETE / CDCL / ATMS), fast algos" | **analog** — keep |
| [`docs/kernel/inference/README.md:608`](../../../../docs/kernel/inference/README.md) | heading "### CDCL nogoods (S1.5b.6)" | **mech** — names the mechanism CDCL; retitle "Learned no-goods (S1.5b.6) — CDCL-style"; anchor `#cdcl-nogoods-s15b6` has exactly one inbound link ([`lattice_dump.md:151`](../../../../docs/kernel/inference/lattice_dump.md)) |
| [`docs/kernel/inference/lattice_dump.md:104`](../../../../docs/kernel/inference/lattice_dump.md) | "`learned_clause.json` ← the CDCL nogood emitted" | **mech** — "the learned no-good emitted (`frozenset(C)`)" |
| [`lattice_dump.md:149–151`](../../../../docs/kernel/inference/lattice_dump.md) | "the `frozenset(C)` nogood emitted … ([CDCL nogoods](README.md#cdcl-nogoods-s15b6))" | **mild** — content honest (`frozenset(C)`!); link text + anchor follow the README retitle |
| [`lattice_dump.md:221–222`](../../../../docs/kernel/inference/lattice_dump.md) | "the clause is the **minimal set** whose conjunction is unsat" | **mech** — false as stated: a `dead-pre` record's clause is the full candidate even when a strict subset (e.g. a `_negated_facts` singleton hit mid-layer) is the culprit ([`commitment.py:90–128`](../../../../ein.py/src/ein/inference/commitment.py)); say "the dead commitment set (subset-minimal among *explored* sets by BFS construction, not a MUS)" |
| [`domain_elim_vs_hypothesis.md:144`](../../../../docs/kernel/inference/domain_elim_vs_hypothesis.md) | "7 commitment sets entered, 6 contradicted, 6 CDCL nogoods learned" | **mild** — "6 no-goods learned" |
| [`architecture_and_algorithms.md:50`](../../../../docs/kernel/inference/architecture_and_algorithms.md) | paradigm row "CSP / SAT solver … DPLL/CDCL (MiniSat, Chaff)" | **analog** — "what Ein borrows" table; keep |
| [`architecture_and_algorithms.md:144`](../../../../docs/kernel/inference/architecture_and_algorithms.md) | "analog: CDCL conflict clause / CSP no-good" | **analog** — keep (this is the review's own "as analogy — fine" line) |
| [`architecture_and_algorithms.md:189`](../../../../docs/kernel/inference/architecture_and_algorithms.md) | O8 analog row "**CDCL**; conflict-directed backjumping; …" | **analog** — keep |
| [`architecture_and_algorithms.md:192–193`](../../../../docs/kernel/inference/architecture_and_algorithms.md) | "the search layer **is a CDCL/CSP solver** with an ATMS underneath" | **mech** — the single strongest overclaim; replace with the canonical sentence (§3) |
| [`architecture_and_algorithms.md:318–339, 343–361, 391–393`](../../../../docs/kernel/inference/architecture_and_algorithms.md) | "plain BFS backtracking, no backjump"; "CDCL-*flavoured*"; "Gap: no backjumping…"; "DPLL/CDCL re-architecture … deferred" | **analog/honest** — keep; this is the model text |
| [`zebra_walkthrough.md:24`](../../../../docs/kernel/inference/zebra_walkthrough.md) | "a contradiction at depth d **backjumps** to d−1 and asserts the negation" | **mild** — no jump happens (BFS layer just continues); "returns to d−1" / "kills the branch and learns the negation". Lines 179/196/249/280 already use the honest "learn no-good" phrasing |
| [`docs/api/ein.md:12`](../../../../docs/api/ein.md) | "never reaches into the matcher, compiler, hypothesis generator, or **CDCL machinery**" | **mild** — "no-good machinery" |
| [`docs/api/ein.md:216`](../../../../docs/api/ein.md) | "the contradiction detector, **the CDCL no-goods**, and the monotonic/ lattice driver" | **mild** — "the learned no-goods" |
| [`docs/api/inference.md:156`](../../../../docs/api/inference.md) | "`enable_path_nogoods` … **CDCL path-condition no-good emission**" | **mild** — "learned no-good emission (CDCL-style)" |
| [`docs/lib/11-search-optimization-algorithms.md:45`](../../../../docs/lib/11-search-optimization-algorithms.md) | Ein's loop "**Structurally identical** to CDCL + ATMS" | **mild** — "structurally akin"; the neighbouring "surprisingly close to CDCL" (line 165) is fine |
| `docs/lib/02:74–84`, `docs/lib/03:156,161`, `docs/lib/09:69`, `docs/lib/06:225,241`, `docs/lib/README.md:21,52` | CDCL described as external tech / comparative | **analog** — literature catalogue; keep |

### Code comments / docstrings (`ein.py/src`)

| site | quote | class |
|---|---|---|
| [`inference/nogoods.py:3`](../../../../ein.py/src/ein/inference/nogoods.py) | "**CDCL flavour** applied to the hypothesis search…" | **mild** — lead with "No-good clause learning (ATMS nogoods; CDCL conflict clauses are the SAT analog)" |
| [`inference/config.py:149`](../../../../ein.py/src/ein/inference/config.py) | "`enable_path_nogoods` — CDCL path-condition no-good emission" | **mild** |
| [`inference/monotonic/__init__.py:45`](../../../../ein.py/src/ein/inference/monotonic/__init__.py) | "**CDCL nogoods** — every dead entering emits `frozenset(C)`…" | **mild** — body already honest; retitle bullet "Learned no-goods" |
| [`inference/monotonic/solver.py:32`](../../../../ein.py/src/ein/inference/monotonic/solver.py) | docstring section "**CDCL (S1.5b.6)**" | **mech** — retitle "No-good learning (S1.5b.6, CDCL-style)" |
| [`inference/monotonic/lattice.py:84`](../../../../ein.py/src/ein/inference/monotonic/lattice.py) | "# S1.5b.6 — **CDCL counters**." | **mild** — "no-good counters" |
| [`kb/store.py:148`](../../../../ein.py/src/ein/kb/store.py) | "# Learned no-good clauses (**path-condition CDCL**)" | **mild** |
| [`cli/solve.py:380`](../../../../ein.py/src/ein/cli/solve.py) | `-L` help: "forces deaths through the **monotonic CDCL path**" | **mild** — user-facing; "through the no-good learning path" |

### Tests (docstrings/comments only; filenames stay)

| site | class |
|---|---|
| [`tests/inference/monotonic/test_monotonic_cdcl.py:1,27,51`](../../../../ein.py/tests/inference/monotonic/test_monotonic_cdcl.py) | **mild** — retitle docstring "no-good learning tests (CDCL-style)"; **keep the filename** (rename churn > honesty gain; the S1.5b.6 stage it mirrors is history) |
| [`tests/inference/monotonic/test_monotonic_dumper.py:37`](../../../../ein.py/tests/inference/monotonic/test_monotonic_dumper.py) | **mild** — same one-word fix |

### Active plans (kept)

[`p1.9 README:87`](../../p1.9_hypothesis_loop_followups/README.md) "Conflict-driven
learning (SAT/**CDCL-inspired**)" — hedged, and that catalog *is* the CDCL
direction: keep. [`plans/m1_core_graph_reasoning/README.md:110,173`](../../README.md)
("CDCL-inspired learning" / "CDCL no-good learning" as catalog topic labels) —
ledger rows describing P1.9; keep. [`plans/ideas/*`](../../../ideas/)
(05:148, 06:42, 08:56) — the user's own framing, authoritative on intent
(CLAUDE.md); keep untouched. This phase's own files quote the review; keep.

**Census total:** 4 **mech** sites (arch:192–193, kernel README:608 heading,
lattice_dump:104 + :221, README.md:32 badge), ~13 **mild** shorthand sites,
everything else legitimately analog or historical.

## 2. Mechanism check

What actually happens on a dead commitment (all paths traced):

1. `try_commitment_set` returns `kind="dead-pre"` (contradiction on write,
   before saturation) or `"dead-post"` (after saturation), each with an
   `unsat_core` = **source frontier of the contradiction witnesses**
   ([`commitment.py:127–128,143–144`](../../../../ein.py/src/ein/inference/commitment.py)
   via [`store.unsat_core`](../../../../ein.py/src/ein/kb/store.py)).
2. `_handle_dead` emits **`frozenset(C)`** — the full commitment set — into
   `root_kb._nogoods`
   ([`_helpers.py:430`](../../../../ein.py/src/ein/inference/monotonic/_helpers.py)),
   gated by `enable_path_nogoods`; size-1 deaths additionally write `(not h)`
   to `_negated_facts` (`:435–436`). The `unsat_core` is stored on the
   `DeadCommitment` **for the trace/verdict only** (`:440`) — **no
   minimisation of any kind feeds the nogood**, and
   [`contract.py:102`](../../../../ein.py/src/ein/inference/monotonic/contract.py)
   asserts it never will (invariant 3).
3. The store applies **subsumption on insert** — an existing subset clause
   rejects the newcomer; the newcomer evicts strict supersets
   ([`nogoods.py:71–81`](../../../../ein.py/src/ein/inference/nogoods.py)).
4. Consumption: `apriori.filter_candidate`'s subset test pre-fork
   ([`apriori.py:90–93`](../../../../ein.py/src/ein/inference/apriori.py)) —
   suppression of supersets, no propagation, no assertion, no jump.

Two nuances the reworded docs should state (they make the honest story
*stronger*, not weaker):

- **The full-C clause is subset-minimal among explored sets by construction.**
  Cardinality BFS + downward-closure filtering means a size-k candidate is
  only entered when no known nogood is a proper subset, so at emit time C has
  no *known-dead* proper subset — measured on zebra2: all 49 deaths are
  singletons, nogoods "already Apriori-minimal"
  ([E7 §2](../../p1.9_hypothesis_loop_followups/s1.9.e7_learned_clause.md)).
  This is precisely why CDCL-style clause shrinking is structurally vacuous
  here (no path-condition tail to strip). It is **not** a MUS guarantee —
  the `dead-pre` mid-layer case records a non-minimal `learned_clause` (the
  subsuming store then drops the emit), which is what makes
  [`lattice_dump.md:221`](../../../../docs/kernel/inference/lattice_dump.md)'s
  "minimal set" claim wrong as stated.
- **Core-based nogood generalisation is NAF-unsound** — a superset commitment
  can derive a fact flipping an `(absent P)` the core's ⊥ relied on
  ([E7 §3](../../p1.9_hypothesis_loop_followups/s1.9.e7_learned_clause.md)),
  and provenance does not record absent-premise usage. Learning the full
  environment is the *sound* choice, which is textbook **ATMS nogood**
  behaviour — exactly the review's point.

Observation (out of R5 scope, for the acceptance gate): with
`enable_path_nogoods=False` **and** a dumper attached, `_handle_dead` hits a
`NameError` — `landed` is assigned only inside the gate
([`_helpers.py:429–430`](../../../../ein.py/src/ein/inference/monotonic/_helpers.py))
but read at `:452`. One-line fix (`landed = False` default); behaviour change,
so not part of the docs-only T1.21.5.2.

## 3. Wording kit

**Canonical positioning sentence** (for `architecture_and_algorithms.md:192–193`,
the kernel README nogood section, and the top-level README):

> Ein's search layer is an **ATMS-style environment search with Apriori
> candidate generation and nogood learning**: commitment sets are assumption
> environments explored breadth-first by cardinality, a dead environment is
> learned whole as a no-good clause (kept subsumption-minimal), and Apriori's
> downward-closure filter suppresses its supersets. **CDCL is the SAT-world
> analog** (no-good ≈ conflict clause) **and an optimization direction**
> ([P1.9 E-catalog](../../p1.9_hypothesis_loop_followups/README.md)), not the
> mechanism.

**Differences-from-CDCL table** (add under the retitled "Learned no-goods
(S1.5b.6)" section of
[`docs/kernel/inference/README.md`](../../../../docs/kernel/inference/README.md)):

| CDCL | Ein lattice search |
|---|---|
| ordered **decision trail**, one variable per decision level | unordered **commitment set C** (an ATMS environment); whole layers by cardinality (Apriori prefix-join) |
| per-conflict **implication graph** + cut analysis | per-fact **provenance DAG** (ATMS justifications); no conflict-cut analysis |
| learned clause = **1UIP-minimised** asserting clause | learned clause = **the full dead environment** (`learned_clause == frozenset(C)`, contract-pinned); shrinking measured vacuous + NAF-unsound (E7) |
| asserting clause **propagates immediately** after backjump | clause only **filters future candidates** pre-fork (`filter_candidate`); size-1 clauses also write `(not h)` |
| **non-chronological backjump** | **no backjump** — BFS layer loop continues; superset suppression prunes descendants |
| VSIDS activity, restarts, watched literals | `lex`/`score-sum` candidate order; none of the rest |

**Per-site rewording** — see the census classification above; every **mech**
row carries its replacement inline, every **mild** row is a one-phrase
substitution ("CDCL X" → "learned no-good X" or "X (CDCL-style)"). Rule of
thumb for the improvement: *CDCL may appear (a) in an explicit analog/
comparison position, (b) as the P1.9/arch-§7 future direction — never as the
subject of a sentence describing what the engine does.*

**Artifact/identifier names — keep all, relabel prose only:**

- `learned_clause.json` / `DeadCommitment.learned_clause` — **keep**. "Learned
  clause" is solver-neutral (ATMS learns clauses too); only the *gloss* "the
  CDCL nogood emitted" (`lattice_dump.md:104`) changes. A rename would touch
  the dumper, renderer ([`render/slice.py:98`](../../../../ein.py/src/ein/render/slice.py),
  [`lattice_dag.py:15`](../../../../ein.py/src/ein/render/lattice_dag.py)),
  trace, contract, tests, and every dump consumer — pure churn.
- `enable_path_nogoods`, `nogoods.py`, `_nogoods` — **keep**; "nogood" is the
  ATMS-native term, already correct.
- `test_monotonic_cdcl.py` — **keep filename** (mirrors the historical S1.5b.6
  stage name); fix the docstring.
- Kernel README heading `### CDCL nogoods (S1.5b.6)` → `### Learned no-goods
  (S1.5b.6)` — heading *is* prose; update the single inbound anchor link
  (`lattice_dump.md:151`).
- Top-level README badge `(Datalog · CDCL/CSP · ATMS · Apriori)` →
  `(Datalog · ATMS · Apriori — CDCL/CSP as analogs)`.

## 4. P1.9 tie-in — the legitimate CDCL direction

Cross-ref these as *future work / recorded outcomes* instead of deleting the
aspiration (all in
[`p1.9 README §Conflict-driven learning + §Search heuristics`](../../p1.9_hypothesis_loop_followups/README.md)):

- **Open (📅) — the real forward pointers:**
  [E20 conflict-cache cross-call](../../p1.9_hypothesis_loop_followups/s1.9.e20_conflict_cache.md)
  (clause persistence ≈ incremental SAT) and
  [E23 exhaustive-search speedup](../../p1.9_hypothesis_loop_followups/s1.9.e23_prove_speedup.md)
  (the umbrella; its candidate list — learned-clause caching, goal-driven
  pruning, AC pre-pass — is the review's P2 line 418 verbatim: "propagation,
  conflict minimization, variable/value selection, backjump-like behaviour").
- **Closed with reasons — cite as "why not CDCL-minimisation":**
  [E7](../../p1.9_hypothesis_loop_followups/s1.9.e7_learned_clause.md)
  (clause-shrinking vacuous + NAF-unsound),
  [E19](../../p1.9_hypothesis_loop_followups/s1.9.e19_unsat_core_min.md)
  (core minimisation shipped as a *trace* win only; deletion-MUS NAF-unsound),
  [E8](../../p1.9_hypothesis_loop_followups/s1.9.e8_watched_fact.md)
  (watched literals — superseded by P1.8a semi-naive),
  E9/E12 (VSIDS-ish ordering — measured worse than lex),
  E14/E15 (AC-3/path-consistency — subsumed by rule saturation).
- The structural lever stays named where it already is:
  [`architecture_and_algorithms.md:391–393`](../../../../docs/kernel/inference/architecture_and_algorithms.md)
  "DPLL/CDCL re-architecture of O7/O8 … deferred because search is not the
  bottleneck (saturation is)."

## Recommendation

**Prose-only repositioning (the stage's expected shape), no renames.** Apply
the census reword table: fix the 4 **mech** sites (lead with the canonical
sentence at `architecture_and_algorithms.md:192–193`; retitle the kernel
README §608 heading + add the differences table there; fix
`lattice_dump.md:104/221` incl. the anchor link at :151; demote the top-level
README badge), sweep the ~13 **mild** shorthand sites in docs/API/src/tests,
and cross-link E20/E23 + arch-§7 as the explicit CDCL direction. Keep
`learned_clause.json`, `test_monotonic_cdcl.py`, `enable_path_nogoods` and all
identifiers.

*Alternatives considered:* (a) rename artifacts/files to purge "CDCL"/"clause"
— rejected: wide churn across dumper/renderer/tests for a vocabulary that is
solver-neutral anyway; (b) docs-only fix of just the 4 mech sites, leaving the
shorthand — rejected: the shorthand ("CDCL nogoods", "CDCL machinery", "CDCL
path") is exactly how the mechanism-drift propagates into new docs; the sweep
is cheap and mechanical; (c) also editing historical stage ledgers — rejected
per stage spec (history stays).

## Improvement inventory

Files T1.21.5.2 will touch (docs + comments/docstrings only, no behaviour):

| file | edits |
|---|---|
| `README.md` | :32 badge → analog phrasing |
| `docs/kernel/inference/README.md` | :608 heading retitle + canonical sentence + differences-from-CDCL table |
| `docs/kernel/inference/lattice_dump.md` | :104 gloss; :151 link text/anchor; :221 "minimal set" → honest formulation |
| `docs/kernel/inference/domain_elim_vs_hypothesis.md` | :144 "6 CDCL nogoods" → "6 no-goods" |
| `docs/kernel/inference/architecture_and_algorithms.md` | :192–193 canonical sentence (CDCL → analog position); optional cross-ref to E20/E23 near :391 |
| `docs/kernel/inference/zebra_walkthrough.md` | :24 "backjumps" → branch-death phrasing |
| `docs/api/ein.md` | :12, :216 "CDCL machinery/no-goods" → "no-good machinery / learned no-goods" |
| `docs/api/inference.md` | :156 `enable_path_nogoods` gloss |
| `docs/lib/11-search-optimization-algorithms.md` | :45 "identical" → "akin" |
| `ein.py/src/ein/inference/nogoods.py` | :1–3 docstring lead |
| `ein.py/src/ein/inference/config.py` | :149 comment |
| `ein.py/src/ein/inference/monotonic/__init__.py` | :45 bullet |
| `ein.py/src/ein/inference/monotonic/solver.py` | :32 docstring section title |
| `ein.py/src/ein/inference/monotonic/lattice.py` | :84 comment |
| `ein.py/src/ein/kb/store.py` | :148 comment |
| `ein.py/src/ein/cli/solve.py` | :380 `-L` help text |
| `ein.py/tests/inference/monotonic/test_monotonic_cdcl.py` | :1,27,51 docstring/comments (filename kept) |
| `ein.py/tests/inference/monotonic/test_monotonic_dumper.py` | :37 comment |

**Tests to add:** none (docs/comments only). Gate per stage: `./run_tests.sh`
+ `ruff check .` + link-check of edited docs (the retitled heading's anchor
has exactly one inbound link, `lattice_dump.md:151` — update in the same
commit).

**Risks:**

- *Anchor breakage* — `#cdcl-nogoods-s15b6` is referenced once
  (`lattice_dump.md:151`); grep for the anchor after the retitle.
- *Help-text greps* — no test asserts on the `-L` help string (verified by
  grep over `ein.py/tests`); still re-run the CLI tests.
- *Parallel-edit conflicts* — `architecture_and_algorithms.md` also carries
  R3 ("minimal"/MUS, :311–312, :351) and R4 ("stratified", :49/:184/:268–269)
  material, and `docs/kernel/inference/README.md` carries R2/R4 NAF sections;
  R5's edits are confined to the CDCL lines listed above, but the improvement
  wave scheduler must serialise R3/R4/R5 improvements touching these two
  files. `_helpers.py` is **not** in R5's change set (the `landed` NameError
  noted in §2 is flagged for the P1.21 acceptance gate, not this task).
- *Tone* — keep `architecture_and_algorithms.md`'s honest §6 text intact; the
  repositioning must not delete the CDCL *aspiration* (E20/E23, arch §7),
  only re-shelve it as direction.
