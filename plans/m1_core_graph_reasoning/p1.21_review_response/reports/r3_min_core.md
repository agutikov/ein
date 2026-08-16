# R3 — `minimal_unsat_core` promises more than it guarantees

**Review point:** [REVIEW_M1-01 §3](../../REVIEW_M1-01.md) (P1).
**Stage:** [S1.21.3](../s1.21.3_min_core_naming.md), task T1.21.3.1 (investigation).
**Investigated:** 2026-08-16, read-only; probe scripts under scratch only.

## Verdict

**Confirmed** — on both sub-claims, plus one finding *beyond* the review.
(i) [`minimal_unsat_core`](../../../../ein.py/src/ein/inference/min_core.py)
is not a subset-minimal MUS — the module says so itself
([min_core.py:17-25](../../../../ein.py/src/ein/inference/min_core.py)) while
[`README.md:19`](../../../../README.md) and three kernel-doc rows promise
"minimal unsat core". (ii) The KB stores exactly one `Provenance` per
`(relation, args)` fact and **silently drops every re-derivation's
justification** at the dedup seam
([store.py:300-302](../../../../ein.py/src/ein/kb/store.py)); an executed probe
(§ Evidence E3) shows the reported "minimal" core flipping between `{C,Y}`
(2 facts) and `{A,B,Y}` (3 facts) on **rule priority alone** — the review's
"`{A,B}` reported while `{C}` exists" instance, reproduced live. (iii) Beyond
the review: `minimal_unsat_core` has **zero production callers** — only
[`tests/inference/test_min_core.py:7`](../../../../ein.py/tests/inference/test_min_core.py)
imports it. The `k = 0` verdict the README describes actually carries the
**union** frontier
([`_union_dead_cores`, _helpers.py:108-114](../../../../ein.py/src/ein/inference/monotonic/_helpers.py)),
so the "minimal unsat core" promise is doubly unbacked: the function that
exists is not minimal, and the shipped verdict does not even call it.

## Evidence

### E1 — single-justification provenance: the drop happens at the store dedup seam

- **Model:** `Fact.provenance` is a single `Provenance | None` field,
  excluded from identity
  ([entities.py:244](../../../../ein.py/src/ein/kb/entities.py); identity =
  `(relation_name, args)` per the class docstring, entities.py:217-221; the
  dataclass is `frozen=True`, entities.py:213). `Provenance` itself holds one
  `rule` + one `premises_raw` tuple
  ([provenance.py:59-77](../../../../ein.py/src/ein/kb/provenance.py)) — there
  is no list-of-justifications anywhere in the model.
- **Loader path:** `add_fact` scans `self.facts` and returns the existing
  object on a `(relation_name, args)` hit
  ([store.py:280-284](../../../../ein.py/src/ein/kb/store.py)); the docstring
  states "the *first* occurrence wins" (store.py:274-276). The incoming fact's
  provenance is never read.
- **Saturation hot path:** `add_and_index_fact` does the same via
  `_fact_by_id` — `if existing is not None: return existing`
  ([store.py:300-302](../../../../ein.py/src/ein/kb/store.py)). No merge, no
  alternative-justification record.
- **Firing side:** every rule application builds a fresh
  `Provenance.from_rule(rule, premises_raw, bindings)`
  ([firing.py:148-152](../../../../ein.py/src/ein/inference/firing.py)) and
  hands it to `add_and_index_fact`; the code comment acknowledges the drop —
  "an already-known conclusion is returned (and indexed) once"
  (firing.py:154-156). Same pattern for native symmetric mirrors
  ([saturator.py:366-374](../../../../ein.py/src/ein/inference/saturator.py)).
  **Decisive for option (b)'s feasibility:** the alternative derivation *is*
  materialised as a `Provenance` object at firing time and reaches the dedup
  seam — recording it would be a local change at store.py:300-302 — it is the
  *storage model and its consumers* that cannot hold it (see § Option (b)).
- **Consumers all assume one justification:** `walk_premises`
  ([provenance.py:323-369](../../../../ein.py/src/ein/kb/provenance.py)),
  `reaches` (provenance.py:282-320), `build_derivation_dag`
  (provenance.py:208-239), `detect_provenance_cycles` (242-276) all walk the
  single `prov.premises_raw`; `kb.unsat_core` is `walk_premises` with a
  frontier `keep` ([store.py:570-597](../../../../ein.py/src/ein/kb/store.py)).

### E2 — what `minimal_unsat_core` actually computes

[`min_core.py:36-54`](../../../../ein.py/src/ein/inference/min_core.py):
for each contradiction witness, take `kb.unsat_core([w])` — the
source/hypothesis leaves of that witness's **recorded** derivation — and return
the smallest such frontier by cardinality. So the guarantee is: *smallest
single-witness frontier over the derivations that provenance happened to
record*. Honest about NAF (the deletion-MUS is unsound here — min_core.py:17-25,
independently documented in
[S1.9.E19 § Implemented](../../p1.9_hypothesis_loop_followups/s1.9.e19_unsat_core_min.md));
silent about first-derivation-wins.

### E3 — order-dependent explanation, executed

Probe (scratch-only, `probe_r3.py`; ein-lang fixture inline): two derivations
of the same fact plus one clash —

```text
join:  (A a) ∧ (B a) → (X a)      chain: (C a) → (X a)      clash: (X a) ∧ (Y a) → (false)
```

all four leaves `:source`-annotated. Only the two rules' `:priority` values are
swapped between runs:

| priorities | `(X a).provenance` recorded | `minimal_unsat_core` | size |
|---|---|---|---|
| join=100, chain=50 | `chain`, premises `(C a)` | `{(C a), (Y a)}` | 2 |
| join=50, chain=100 | `join`, premises `(A a), (B a)` | `{(A a), (B a), (Y a)}` | **3** |

Same logical content, different reported "minimal" core; in the second run the
strictly smaller explanation `{C,Y}` **exists** but is unreachable because
`chain`'s justification for `(X a)` was dropped at store.py:301-302. This is
exactly [REVIEW_M1-01 §3](../../REVIEW_M1-01.md)'s `{A,B}`-vs-`{C}` diagram,
confirmed empirically. (Existing suite stays green:
`tests/inference/test_min_core.py` — 3 passed.)

### E4 — the shipped `k = 0` verdict is the *union* core, not this function

- `verdict_of` builds `Contradiction(unsat_core=_union_dead_cores(...))`
  ([_helpers.py:153](../../../../ein.py/src/ein/inference/monotonic/_helpers.py);
  union at _helpers.py:108-114; root-level case `_source_frontier_core`,
  _helpers.py:117-125, 208; lattice-side twin
  [_state.py:153](../../../../ein.py/src/ein/inference/monotonic/_state.py)).
- CLI/trace read `verdict.unsat_core` only:
  [cli/solve.py:172-177](../../../../ein.py/src/ein/cli/solve.py)
  (`"unsat-core facts (N facts):"`),
  [trace/answer.py:125-132](../../../../ein.py/src/ein/trace/answer.py)
  (`"(unsat core: …)"`), answer.py:231-241 (`"unsat core (N facts)"`),
  [trace/linearize.py:167-172](../../../../ein.py/src/ein/trace/linearize.py)
  (`"Contradiction — no model; unsat core: …"`). **None of these strings says
  "minimal"** — the runtime output is honest; only the *docs* over-promise.
- `grep -rn min_core ein.py/` outside the module itself hits exactly one file:
  `tests/inference/test_min_core.py`. The E19 use-site plan ("the
  contradictions answer calls `minimal_unsat_core(kb)` for the legible why",
  [s1.9.e19:39-42](../../p1.9_hypothesis_loop_followups/s1.9.e19_unsat_core_min.md))
  was never wired.

## 1. Single-justification provenance — verified

Confirmed at all three levels with the citations in § E1 (model:
entities.py:244 + provenance.py:59-77; store: store.py:280-284 and
store.py:300-302; firing: firing.py:148-162). The order-dependent instance is
not on-paper but executed (§ E3): report `{A,B,Y}` while `{C,Y}` exists, flipped
purely by `:priority`.

## 2. Promise census

Every occurrence of the words "minimal unsat core" / the symbol
`minimal_unsat_core` / adjacent "minimal"-claims about cores.
**Bold** = over-promising, must change. *Italic* = historical/plan record,
annotate or leave.

| site | text | class |
|---|---|---|
| [`README.md:18-19`](../../../../README.md) | `k = 0` "reported with a **minimal unsat core**" | **fix** — verdict actually carries the union frontier (E4) |
| [`docs/kernel/inference/README.md:73`](../../../../docs/kernel/inference/README.md) | "**minimal unsat core** via `min_core.py` + provenance" | **fix** |
| [`docs/kernel/inference/reserved_engine_strings.md:46`](../../../../docs/kernel/inference/reserved_engine_strings.md) | `k = 0` shape = "**minimal unsat core**" | **fix** |
| [`docs/kernel/inference/python_impl.md:60`](../../../../docs/kernel/inference/python_impl.md) | `min_core.py` = "**minimal unsat core** (sound, provenance-based)" | **fix** |
| [`docs/kernel/inference/architecture_and_algorithms.md:29`](../../../../docs/kernel/inference/architecture_and_algorithms.md) | contradictions = "a **minimal unsat core**" | **fix** (note: lines 311-312 already honestly flag "not a minimal MUS" — the doc contradicts itself) |
| [`docs/kernel/glossary.md:96-100`](../../../../docs/kernel/glossary.md) | "Unsat core — The **minimal** source-kind frontier" | **fix** (drop "minimal", add recorded-derivations caveat) |
| [`docs/kernel/ir/02-data-model/02_store.md:200-202`](../../../../docs/kernel/ir/02-data-model/02_store.md) | union is "the **minimal source-frontier**" | **fix** |
| [`ein.py/src/ein/kb/store.py:575`](../../../../ein.py/src/ein/kb/store.py) | `unsat_core` docstring: "the **minimal** set of facts that jointly derive the conflict" | **fix** (one word) |
| [`ein.py/src/ein/inference/min_core.py:1,3,15,36,57`](../../../../ein.py/src/ein/inference/min_core.py) | module title "Minimal unsat core", the symbol, `__all__` | **rename** |
| [`ein.py/tests/inference/test_min_core.py:1,7,43-64`](../../../../ein.py/tests/inference/test_min_core.py) | docstring, import, `TestMinimalUnsatCore` | **rename** (sole importer) |
| CLI/trace output strings (E4: solve.py:174, answer.py:132/241, linearize.py:172) | "unsat core", never "minimal" | already honest — keep |
| [`plans/…/p1.9…/README.md:138`](../../p1.9_hypothesis_loop_followups/README.md) + [`s1.9.e19_unsat_core_min.md:10,18,32,40`](../../p1.9_hypothesis_loop_followups/s1.9.e19_unsat_core_min.md) | E19 status rows naming the symbol | *annotate* — add rename addendum, keep history |
| [`plans/followups/f3…:38`](../../../followups/f3_three_task_classes_first_class.md) | "contradictions foregrounds the minimal unsat core" | **fix** (live follow-up plan) |
| [`plans/m3_smt_integration/README.md:44-46`](../../../m3_smt_integration/README.md) | SMT backend "returns a minimal unsat core" | *leave* — a real MUS is achievable on the SMT backend; not an M1 claim |
| [`plans/ideas/03-three-task-classes.md:53`](../../../ideas/03-three-task-classes.md) | "A minimal unsatisfiable subset is the right granularity" | *leave* — user's own intent doc (CLAUDE.md: keep framing intact); it states the *goal*, which is fine |
| [`docs/lib/02-solvers-csp-sat-smt.md:242`](../../../../docs/lib/02-solvers-csp-sat-smt.md), [`docs/lib/11…:90`](../../../../docs/lib/11-search-optimization-algorithms.md) | literature catalogue entries on MUS | *leave* — describe SOTA, not Ein |
| P1.21 phase docs ([README](../README.md), [stage](../s1.21.3_min_core_naming.md)), [REVIEW_M1-01](../../REVIEW_M1-01.md), this report | quote the old name by necessity | *leave* |

`grep -rn "minimal_unsat_core"` and `grep -rni "minimal unsat"` over the repo
confirm the table is exhaustive (2026-08-16).

## 3. Options weighed

**(a) Rename + precise contract.** Cost: one module + one test file (the
symbol's only importer — E4), plus the eight doc/docstring fix rows above.
Zero behaviour change, zero acceptance risk (no output string contains
"minimal"). Fixes the review's floor completely. Does *not* make explanations
order-independent — it says so instead.

**(b) Multi-justification OR/AND proof DAG + true minimal-explanation search.**
Feasibility: the alternative justification already reaches the dedup seam as a
built `Provenance` (E1, firing.py:148-162), so *recording* it is a local edit
at store.py:300-302. Everything after that is expensive:

- **Storage:** `Fact` is `frozen=True` with a single `provenance` field
  (entities.py:213, 244); alternatives need either un-freezing / object
  replacement (breaks the identity-sharing every index relies on —
  store.py:165-193) or a KB-level side table
  `dict[FactId, tuple[Provenance, ...]]` that must join the fork/snapshot copy
  contract ([store.py:708-731](../../../../ein.py/src/ein/kb/store.py)) and
  the S1.21.1 state-identity story.
- **Consumers:** every provenance walker assumes one justification — 4
  functions in provenance.py (E1) + their callers across `trace/answer.py`,
  `trace/linearize.py`, `render/slice.py`, `render/lattice_dag.py`,
  `inference/commitment.py`, `inference/monotonic/_helpers.py`/`_state.py`,
  nogood lifting. ≈ a dozen modules re-reasoned for OR-nodes.
- **Search:** minimum-cardinality source frontier over an AND/OR DAG is the
  minimum-axiom-set / ATMS-label problem — worst-case exponential (the reason
  ATMS labels blow up); "define/search real minimality" is a research-grade
  deliverable, not a fix.
- **Runtime cost — measured, and *lower* than the stage feared:** a fresh
  cProfile of `solve(zebra2.ein, stop_after=1)`
  (`utils/profile_solve.py`, 2026-08-16) puts 72 % of self-time in match/bind
  and the whole *saturate* subsystem at **1 %**; `add_and_index_fact` is not in
  the top-30 by cumtime. So appending to a justification table would be cycles-
  cheap — the blocker is model blast radius + search complexity + semantics
  (a justification recorded inside a hypothesis fork is not valid at root;
  NAF-stage-dependence), **not** hot-path cycles. The stage file's "P1.8a
  measured `add_and_index_fact` hot" concern should be recorded as *not
  currently reproduced* (no P1.8a doc names it either — the profile data
  supersedes).
- **Completeness caveat even then:** alternatives are only those firings the
  saturator actually attempts; binding-level dedup (saturator.py:293,
  `_binding_key`) is per `(rule, binding)`, so distinct proofs do fire — but
  "minimal over all derivations" would still be relative to the rule set and
  saturation strategy.

**(c) Staged: (a) now, park (b).** All of (a)'s payoff immediately; (b)
becomes an explicit P1.9 catalog entry (next free row: **E25**) cross-linked
from the renamed module's docstring, so the DAG idea survives with its
feasibility notes (record-at-dedup-seam is cheap; the cost is consumers +
search) instead of dying in this report.

## 4. Exact new contract wording (chosen path)

**Symbol:** `minimal_unsat_core` → `smallest_contradiction_frontier`
(the review's own suggestion). **Module:** `min_core.py` →
`frontier.py` (keeping a file whose *name* says "min core" would perpetuate
the claim; exactly one importer exists, so the move is free; alternative
`explanation.py` noted and not preferred — the function returns a frontier,
not prose).

**Canonical long form** (module + function docstring — the single normative
statement, everything else abbreviates it):

> `smallest_contradiction_frontier(kb)` returns the smallest **recorded**
> single-witness source frontier: over the contradiction witnesses the
> detector finds, the smallest set of given/hypothesis leaves of **one**
> witness's recorded derivation. Sound by construction and NAF-safe — it is a
> real derivation's leaves, never a re-saturated guess. It is **not** a
> subset-minimal MUS (no guarantee that every proper subset is satisfiable),
> and it is minimal only over *recorded* derivations: the KB keeps one
> justification per fact (first derivation wins — `store.add_and_index_fact`),
> so a shorter derivation that fired later is invisible and the result can
> depend on rule-firing order.

**One-line row form** (reused verbatim at every table site):

> smallest recorded contradiction frontier — one witness's derivation leaves
> (provenance-based, NAF-safe); **not** a subset-minimal MUS, minimal only
> over first-recorded derivations

**Per-site replacements:**

- `README.md:18-19`: "`k = 0` → **a contradiction** — an over-constrained KB,
  reported with its unsat core: the given facts on the recorded
  contradictions' derivation frontier (provenance-based; not a subset-minimal
  MUS)."
- `reserved_engine_strings.md:46` shape cell: "unsat core — recorded-derivation
  source frontier (not a subset-minimal MUS)".
- `docs/kernel/inference/README.md:73`: "…smallest recorded contradiction
  frontier via `frontier.py` + provenance".
- `python_impl.md:60`: "`frontier.py` | smallest recorded contradiction
  frontier (provenance-based, NAF-safe; not a subset-minimal MUS)".
- `architecture_and_algorithms.md:29`: "**contradictions** — the unsat core
  (recorded-derivation source frontier) of an over-constrained KB. (k = 0)".
- `store.py:575` docstring: "The union is the source-frontier set of facts
  that jointly derive the conflict *per the recorded derivations*…" (delete
  "minimal").
- `glossary.md § Unsat core`: "The source-kind frontier across a set of
  conflicting facts, per the recorded derivations — the 'given' premises that
  together derive the conflict. Not a subset-minimal MUS."
- `02_store.md:201`: "The union is the **recorded source-frontier** that
  derives the conflict".

## Recommendation

**Take (c): rename now with the § 4 contract, park (b) as P1.9 E25.**
Why: (a)'s entire benefit at near-zero risk — the symbol has one importer, no
output string changes, the review's floor ("the name must stop over-promising")
is met exactly; and the census shows the deeper fix the docs *actually* need is
not the function at all but the verdict-shape rows, since the shipped `k = 0`
payload is the union frontier and `minimal_unsat_core` is production-dead
(E4). (b) alone would leave the README wrong anyway (the verdict path doesn't
call the function) while costing a dozen-module model change plus an
exponential-worst-case search — disproportionate for an M1 truth-in-labelling
P1. The E25 stub preserves (b) with its feasibility map (record at
store.py:300-302 is cheap; consumers + minimality search are the cost) and a
note that E19's planned trace use-site ("foreground the legible why" —
[s1.9.e19:39-42](../../p1.9_hypothesis_loop_followups/s1.9.e19_unsat_core_min.md))
is still unwired — wiring `smallest_contradiction_frontier` into the
contradiction answer is a natural companion follow-up, deliberately **not**
part of T1.21.3.2 (it changes acceptance-visible output and would race the
other P1.21 improvement lanes).

Alternatives noted: (a)-only without the E25 stub loses the DAG design notes;
(b) now — rejected above; keeping the `min_core.py` filename — rejected (name
still says "min core"; the move costs one import line).

## Improvement inventory (T1.21.3.2)

Files (repo-relative; exhaustive — parallel-wave scheduling input):

1. `ein.py/src/ein/inference/min_core.py` — **delete** (git mv →
   `frontier.py`); rewrite module + function docstring per § 4; rename symbol +
   `__all__`; cross-link E25.
2. `ein.py/src/ein/inference/frontier.py` — **new** (the moved module).
3. `ein.py/tests/inference/test_min_core.py` — **delete** (git mv).
4. `ein.py/tests/inference/test_frontier.py` — **new**: renamed import/class +
   the new order-dependence pin (below).
5. `ein.py/src/ein/kb/store.py` — `unsat_core` docstring word at :575
   (⚠ overlaps S1.21.1's interest in `add_and_index_fact` — schedule after or
   merge trivially; the edit is one docstring line).
6. `README.md` — line 18-19 replacement (§ 4).
7. `docs/kernel/glossary.md` — § Unsat core (≈ lines 96-100).
8. `docs/kernel/inference/README.md` — line 73 row.
9. `docs/kernel/inference/reserved_engine_strings.md` — line 46 row.
10. `docs/kernel/inference/python_impl.md` — line 60 row.
11. `docs/kernel/inference/architecture_and_algorithms.md` — line 29; optional
    one-clause tighten at 311-312 to cite the new name.
12. `docs/kernel/ir/02-data-model/02_store.md` — § 7.2 (≈ line 201).
13. `plans/followups/f3_three_task_classes_first_class.md` — line 38 wording.
14. `plans/m1_core_graph_reasoning/p1.9_hypothesis_loop_followups/README.md` —
    E19 row: append rename note; add E25 row.
15. `plans/m1_core_graph_reasoning/p1.9_hypothesis_loop_followups/s1.9.e19_unsat_core_min.md`
    — addendum: renamed 2026-08 per R3; link this report.
16. `plans/m1_core_graph_reasoning/p1.9_hypothesis_loop_followups/s1.9.e25_multi_justification_provenance.md`
    — **new** E25 stub (OR/AND proof DAG; feasibility + cost notes from § 3(b);
    companion note: wire the frontier into the contradiction answer).
17. `plans/m1_core_graph_reasoning/p1.21_review_response/s1.21.3_min_core_naming.md`
    — status tick (T1.21.3.2 done).

Tests to add (in `ein.py/tests/inference/test_frontier.py`):

- `test_result_depends_on_recorded_derivation_order` — the § E3 fixture, both
  priority variants: assert each result is a sound frontier (⊆ union core),
  assert the two results **differ** — the executable form of the "minimal only
  over recorded derivations" caveat, so any future (b)-style fix flips this
  test deliberately.
- Keep the three existing tests under the new names (empty-on-consistent,
  subset-of-union, zebra2-bad culprit).

Gate: `./run_tests.sh` (full — pytest -q skips acceptance/) + `ruff check .`
green; acceptance output unchanged (no runtime string contains "minimal";
verified in E4). Grep-zero for `minimal_unsat_core` and for the phrase
"minimal unsat core" outside `plans/ideas/`, `plans/m3_smt_integration/`,
`docs/lib/`, `REVIEW_M1-01.md`, and P1.21's own record.

Risks: (low) parallel-wave overlap on `store.py` with S1.21.1 (one-line
docstring edit — trivial merge); (low) doc-row overlap with R5's positioning
edits in `architecture_and_algorithms.md` — R5 touches the CDCL/ATMS sections,
R3 touches line 29 + O6; coordinate if both land in one wave; (nil) runtime
behaviour — the renamed function still has no production caller.
