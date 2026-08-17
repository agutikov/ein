# C3 — Knowledge-layer removal census

**Stage:** [S1.22.1b](../s1.22.1b_layer_removal.md), task T1.22.1b.1
(read-only). **Date:** 2026-08-17. **Tree:** post-S1.22.1 (`3de7af0`).
**Interpreter:** `.venv-pypy/bin/python` (PyPy 7.3.23 / 3.11.15).

> **Executed.** T1.22.1b.2/3 shipped on 2026-08-17; the results (and three
> corrections to the stage brief) are in
> [S1.22.1b §Outcome](../s1.22.1b_layer_removal.md#outcome-2026-08-17).
> The predictions below held: 2 goldens shifted and only those 2, no
> verdict moved, no serialised format needed a bump. Two test dispositions
> in §6.2 came out slightly wider than sketched — `kb/test_layers.py` lost
> its whole `TestLayerViews` class (not just the accessors) and
> `test_ir_ast.py`'s `:layer reasoning` round-trip case was dropped rather
> than re-pointed, since `:rule`/`:using` already had its own case.

---

## 0. Summary

| question | answer |
|---|---|
| Is the bug real and reachable? | **Yes** — reproduced; a flatly inconsistent KB reports **0** contradictions (§1). |
| Does the fix flip any acceptance verdict at the root? | **No** — 0 cross-layer pairs at the root of `zebra2` / `zebra2-minus-15` / `zebra2-hints` / `zebra.ein`; the two fixtures that do have them (`zebra2-bad`, `ein-bugs/zebra2-hints`) are already dead by `(false)` (§6). |
| Is `Layer` derivable from `Provenance`? | **Yes**, exactly — 3039 facts, 23 mismatches, all 23 are the 23 explicit `:layer` facts (§2). |
| `:layer` disposition | **Reject at load**, loudly (§4). |
| Layer views (`ontology()` / `fact_layer()` / `reasoning()` / `all_layers()` / `facts_in_layer()`) | **Zero production callers.** Drop the three layer-scoped ones + `facts_in_layer`; rename `all_layers()` → `all_facts()` (§3). |
| Goldens that shift | 2 predicted, both from `zebra.ein`'s four `:layer ontology` lines (§5). |
| Serialised formats needing a version bump | **None** (§5.3). |

---

## 1. The bug, reproduced

The plan's §Why reproducer, run on this tree:

```
stated : [(('A','S1'), 'fact'),      (('B','S1'), 'fact')]
derived: [(('B','S1'), 'reasoning'), (('A','S1'), 'reasoning')]
contradictions detected: 0
```

The KB simultaneously holds `(sits A S1)`, `(sits B S1)`, `(not (sits A S1))`
and `(not (sits B S1))`. `ContradictionDetector.detect()` returns `()` because
[`contradiction.py:183`](../../../../ein.py/src/ein/inference/contradiction.py)
skips every pair whose two facts differ in layer.

---

## 2. Site inventory, split by meaning

The word "layer" names two unrelated things. Counts are **lines matched**, from
`grep -rn "Layer\b|\.layer\b|:layer|layer=|layers\b|_layer\b|layer_"` over
`ein.py/src/ein`.

### 2.1 Knowledge layer — **in scope**

| file | lines | what |
|---|---|---|
| `kb/entities.py` | 55, 58–71, 213, 237, 389 | the `Layer` enum + `Fact.layer` field + `__all__` |
| `kb/from_ir.py` | 6, 44, 60, 63–92, 136, 156, 209, 366, 368, 421, 441, 498, 523, 550, 551 | `_layer_of`, `_LAYER_BY_NAME`, the `:layer` kwarg, the per-fact layer plumbed through pass 3 |
| `kb/store.py` | 35, 146, 308–311, 325, 331, 483, 602, 677–705, 749, 808, 815 | `facts_in_layer` + the four `FactView` accessors + docstrings |
| `kb/views.py` | 1, 4–6 | module docstring naming the four accessors |
| `kb/__init__.py` | 17, 24, 39 | re-export |
| `kb/render.py` | 27–32, 52, 180, 186, 199–201, 222, 245, 255, 292, 346–349, 353–354, 366, 368 | `layers=` filter, `colour_by="layer"`, `_pick_style`, `_label_extra` |
| `kb/provenance.py` | 52 | comment only ("both layers" = kb/inference strata) — **false positive** |
| `inference/contradiction.py` | 28–41, 58, 87, 130–132, 163, 183, 190 | the restriction, `Contradiction.layer`, `detect_layer()` |
| `inference/lookahead.py` | 29–39, 43, 178–181, 195–197, 199–203 | the mirror |
| `inference/firing.py` | 27, 93, 101, 117, 146, 167 | fact construction |
| `inference/saturator.py` | 46, 525 | fact construction |
| `inference/hypgen.py` | 40, 312, 333 | fact construction |
| `inference/commitment.py` | 37, 101 | hypothesis-fact construction |
| `inference/closed.py` | 29, 88 | `(__closed__ R)` construction |
| `inference/apriori.py` | 169, 181 | fact construction (**only** these two — the rest of the file is lattice) |
| `inference/monotonic/sanity.py` | 49, 150 | fact construction |
| `inference/monotonic/_helpers.py` | 51, 110, 117, 162 | fact construction (**only** these — 285/295/346/355/456 are lattice) |
| `inference/monotonic/_serialise.py` | 17, 39, 63–82, 90, 95, 104 | the flat `.ein` dumper's `:layer` emission + ONTOLOGY-first ordering |
| `inference/canon.py` | 46 | comment: layer excluded from `state_key` |
| `cli/saturate.py` | 53, 106–109, 407–420 | the layer histogram + the `--dump` grouping |
| `cli/_factdump.py` | 69, 78 | `--print-final-state/-positive` filters on REASONING |
| `render/slice.py` | 193, 199, 203–209 | `layer_filter` passthrough |
| `ir/to_dot.py` | 390–402, 442, 464 | `_flat_layer` — an **IR-form** grouper (see §2.3) |
| `render/constraints.py` | 57–70, 82 | `_flat_layer` — same |
| `ir/grammar.lark` | 21, 100–101 | comments only |

### 2.2 Lattice layer — **out of scope, do not touch**

Search depth in the monotonic solver. Named false positives:

- `inference/monotonic/solver.py` — **all** hits (`_explore_layers`,
  `_phase2_layers`, `a_layer`, `layer_start`/`layer_end`, `layer_1`).
- `inference/monotonic/_lattice_dump.py` — **all 9+** hits
  (`layers/layer_NN/`, `enterings/layer_NN/`, `node.layer`).
- `inference/monotonic/state_dump.py` — **all** hits
  (`layers/layer_NN_pre.ein`, `layer_start`/`layer_end` hooks).
- `render/lattice_dag.py` — **all 7** hits: `_Cell.layer: int = len(rep)`
  is the commitment-set size, i.e. lattice depth.
- `inference/apriori.py` — `generate_layer`, `layer_1`, the docstring.
- `inference/monotonic/_helpers.py` lines 215, 285, 295, 302, 346, 355, 456.
- `inference/config.py`, `inference/monotonic/{__init__,lattice,_state}.py`,
  `cli/solve.py` — all hits.

> **Correction to the plan's §C counts.** The stage brief listed
> `_lattice_dump.py` (9), `render/lattice_dag.py` (7) and `state_dump.py` (4)
> as presentation work and `_helpers.py` as 8 sites. In fact
> **`_lattice_dump.py`, `lattice_dag.py` and `state_dump.py` are 100 %
> lattice-layer** — zero work — and `_helpers.py` splits 4 knowledge / 5
> lattice. The presentation surface is materially smaller than budgeted:
> `kb/render.py`, `cli/saturate.py`, `_serialise.py`, `cli/_factdump.py`,
> `render/slice.py`.

### 2.3 A third thing: the IR-form render grouper

`ir/to_dot.py::_flat_layer` and `render/constraints.py::_flat_layer` are
*duplicated* helpers that classify a **parsed form** (not a `Fact`) into one of
the three DOT buckets `ontology` / `fact` / `reasoning`, mirroring `_layer_of`.
They never touch `Fact.layer` or the `Layer` enum. They survive the removal as
a **rendering grouping**, but their explicit-`:layer` branch must go in lockstep
with the loader — otherwise the renderer would honour an annotation the loader
rejects. Disposition: keep the grouping, delete the `:layer` branch, rename to
`_render_group` so the retired concept's name does not linger.

---

## 3. The replacement mapping

### 3.1 The predicate

```python
Layer.REASONING  ⟺  provenance is not None and provenance.kind in {"rule", "hypothesis", "rejected"}
Layer.FACT       ⟺  provenance is not None and provenance.kind == "source" and provenance.source is not None
Layer.ONTOLOGY   ⟺  otherwise  (provenance is None, or kind == "source" with source is None)
```

The plan's refinement is confirmed: the discriminator is
`provenance.source is not None`, **not** `kind` alone — `kind == "source"`
covers both the sourced clues (FACT) and the unsourced ontology facts (`is-a`
enumerations, `(relation …)` mirrors, property tags) whose `source` is `None`.

### 3.2 Exhaustive verification

Over **every** `examples/**/*.ein` (parse → load → `emit_closed` → saturate to
fixpoint), comparing the stored `Fact.layer` against the predicate:

```
facts checked: 3039     mismatches: 23
```

The 23, by file — **and there are exactly 23 `:layer` occurrences in
`examples/`**, a 1:1 correspondence:

| file | n | shape |
|---|---|---|
| `features/01_not_and_absent.ein` | 2 | `:layer fact`, unsourced → predicted ONTOLOGY |
| `features/03_forall.ein` | 6 | `:layer fact`, unsourced → predicted ONTOLOGY |
| `features/04_open.ein` | 2 | `:layer fact`, unsourced → predicted ONTOLOGY |
| `features/05_stdlib_domain_elim.ein` | 3 | `:layer fact`, unsourced → predicted ONTOLOGY |
| `saturation/hypothesis-contradiction/coloc-disproved.ein` | 2 | `:layer reasoning`, unannotated → predicted ONTOLOGY |
| `saturation/hypothesis-contradiction/next-to-disproved.ein` | 2 | ditto |
| `saturation/hypothesis-contradiction/right-of-disproved.ein` | 2 | ditto |
| `zebra.ein` | 4 | `:layer ontology` **with** `:source "condition (1)"` → predicted FACT |

Residue after `:layer` is removed: **zero**. `Layer` carries no information the
`Provenance` does not.

### 3.3 One provenance side-effect to know about

`from_ir._ingest_one_fact` builds provenance as

```python
elif source is not None or layer in (Layer.FACT, Layer.ONTOLOGY):
    provenance = Provenance.from_source(source=source, loc=child.loc)
else:
    provenance = None            # ← only reachable for an authored :layer reasoning
```

So the 6 `:layer reasoning` facts in `saturation/hypothesis-contradiction/*`
are the *only* facts in the corpus whose `provenance is None`. Dropping
`:layer` gives them `Provenance.from_source(source=None)` instead — and the
`else: provenance = None` arm becomes **dead code**, to be deleted with it.
Every consumer already treats `provenance is None` and `kind == "source"` with
no source identically (`DerivationDAG.sources`,
`provenance._fact_dot_label`, `walk_premises`'s frontier predicate), so this is
behaviour-neutral; it is recorded here because it is the one place the removal
changes a field other than `layer`.

---

## 4. Per-view disposition

Grepped over `ein.py/src`, `ein.py/tests`, `ein.py/acceptance`, `utils`:

| accessor | production callers | tests | disposition |
|---|---|---|---|
| `KnowledgeBase.facts_in_layer(layer)` | **0** | `kb/test_store.py:181` | **delete** |
| `KnowledgeBase.ontology()` | **0** | `kb/test_layers.py`, `integration/test_zebra_parse.py` | **delete** |
| `KnowledgeBase.fact_layer()` | **0** | same | **delete** |
| `KnowledgeBase.reasoning()` | **0** | same | **delete** |
| `KnowledgeBase.all_layers()` | **0** | `kb/test_layers.py:104,175` | **keep, renamed `all_facts()`** — it is layer-free ("every fact"); only its *name* refers to the removed concept |
| `FactView` + `.relation()` / `.about()` / `.by_source()` / `.by_rule()` / `.matching()` | via the accessors | `kb/test_layers.py` | **keep unchanged** — the filters are provenance- and relation-based, and `by_rule()` / `by_source()` already give back everything `reasoning()` / `fact_layer()` did |

The three layer-scoped views have **no production consumer at all** — they were
a P1.2 inspection API that the engine never adopted. Re-expressing them over
provenance would preserve a surface nobody calls; dropping them is the smaller
kernel. `kb/views.py` keeps its class and loses its docstring's layer framing.

Tests are re-pointed, not preserved: per the S1.22.1 rule, `kb/test_layers.py`'s
layer-partition class goes with the partition, while its `TestFactViewFilters`
class survives on `all_facts()`.

---

## 5. The `:layer` rejection decision (§B.6)

**Ruling: reject at load, with a named error.** Not the `:where` precedent.

```
(:layer …) — knowledge layers were removed in S1.22.1b; delete the
annotation (a fact's origin is its :source / :rule provenance)  at <loc>
```

Rationale, in the order that decided it:

1. **`:layer` was authoritative, `:where` was inert.** `_layer_of` gives the
   explicit keyword priority over the derivation — it *overrode* engine
   behaviour, and (§6) the layer it set could flip a contradiction from
   reported to ignored. Q32 dropped `:where` silently because `:where` never
   did anything; dropping an annotation that *did* something, silently, changes
   behaviour on existing files without saying so. That is precisely the failure
   mode S1.22.0 traced the surviving `(or …)` bugs to.
2. **The blast radius is nil.** 23 occurrences, 8 files, all in-repo, all
   deleted by this stage. Nobody outside the repo authors `.ein`.
3. **A loud reject is the migration note.** A stale file (an old `state_dump`
   snapshot, a hand-copied fixture) fails with a sentence naming the stage
   rather than silently re-classifying.

Scope of the reject: the **fact** path only (`_ingest_one_fact`), which is
where `:layer` was ever legal. The grammar is untouched — `:layer` is a
generic kw-pair, not a grammar production, so a `.ein` carrying it still
*parses* and is rejected one level up, at load. That is the right seam.

---

## 6. Blast radius

### 6.1 Goldens

| golden | shifts? | why |
|---|---|---|
| `tests/golden/zebra.golden` | **yes** | `dump_canonical(parse(examples/zebra.ein))`; the four `:layer ontology` lines leave `zebra.ein`, so lines 165/167/169/171 lose the kwarg |
| `tests/golden/kb_zebra_unified.dot` | **yes** | those same four `(right-of …)` facts move ONTOLOGY→FACT, so `_label_extra` starts emitting the `(1)` source badge on their edges |
| `tests/golden/zebra2.golden` | no | `zebra2.ein` has no `:layer` |
| `tests/golden/dot/*.dot` (15 emitters) | no | none of the fixed inputs carries `:layer`; `_REASONING_TEXT` uses `:rule`, `_FACT_TEXT`/`_NEG_TEXT` use `:source`, `_small_kb()` is unannotated |
| `tests/golden/trace_3step.md` | no | trace renderer does not read layer |

Both shifts are *predicted by the mapping* and are the mapping working: a
`:source`-carrying fact should render as a sourced fact.

### 6.2 Tests that pin layer semantics

| test | disposition |
|---|---|
| `inference/test_contradiction.py::test_cross_layer_non_conflict` | **delete** — pins the removed restriction (the stage brief and the S1.22.1 rule both say so) |
| `inference/test_contradiction.py::test_detect_layer_scopes_correctly` | **delete** — `detect_layer()` goes with `Contradiction.layer` |
| `inference/test_contradiction.py` — the rest (~15 cases) | re-point: drop the `layer=` kwargs, keep the assertions |
| `kb/test_entities.py::test_layer_enum_*` | delete |
| `kb/test_entities.py::test_nested_fact_layer_excluded_from_identity` | re-point onto another non-identity field (`provenance`) — the *property* survives |
| `kb/test_layers.py` — `TestLayerViews` | delete with the views |
| `kb/test_layers.py` — `TestFactViewFilters`, fork cases | re-point onto `all_facts()` |
| `kb/test_store.py::test_facts_in_layer_filter` | delete |
| `integration/test_zebra_parse.py` — `_has(..., layer=)`, `fact_layer()`/`ontology()` counts | re-point onto fact-set identity (which is what the assertions are really about) |
| `inference/test_state_key.py` — layer-free identity | re-point onto `provenance`; the *invariant* (identity excludes metadata) is the point and must survive |
| `inference/test_dies_immediately.py:111–117` | re-point: the case "an existing FACT-layer `g` is not a contradiction" **inverts** — it is now a kill. This is the acceptance-2 pin. |
| `test_ir_ast.py:277` (`:layer reasoning` round-trip case) | re-point onto a `:rule`-annotated form; `:layer` is no longer syntax the repo authors |
| ~20 files constructing `Fact(..., layer=Layer.X)` | mechanical: drop the kwarg |

### 6.3 Serialised / persisted formats

| format | carries layer? | version bump? |
|---|---|---|
| `monotonic/_serialise.py` flat `.ein` dump | **yes** — emits `:layer` for the residue, and orders ONTOLOGY-first | **no bump.** These are *debug dumps* written to a `--dump-dir`, never read back by the engine; there is no reader to version. The emission is deleted and the round-trip becomes exact by construction. |
| `LatticeSnapshotV1` | no — commitments, state keys, unsat cores, learned clauses; no `Fact.layer` | no |
| `_lattice_dump.py` JSON index | no — its `"layer"` keys are lattice depth | no |
| `state_dump.py` `.ein` snapshots | via `_serialise` only | no |
| `trace/` events | no | no |

---

## 7. Verdict-risk assessment

The fix makes the engine detect **more** contradictions. Three places consume a
contradiction, and only two can move a verdict:

| site | effect of an extra contradiction |
|---|---|
| `monotonic/solver.py:238` (root, post-saturation) | **verdict-affecting** — `_root_dead()`, k=0, immediate `Contradiction` |
| `monotonic/_helpers.py:174` (root, post forced-positive promotion) | same |
| `commitment.py:111/126` (fork pre-/post-saturation) | kills a branch. Timing-only *unless* the branch was a **solution node** — then k drops, and `Solution`→`Contradiction` or `Ambiguity`→`Solution` |
| `solution.py:58` (`is_consistent`) | a node holding `X ∧ ¬X` stops counting as a solution |
| `explain.py:426`, `_state.py:137` | witnesses for an already-decided kill |

### 7.1 Root measurement

Load → `emit_closed` → saturate → count `(X, ¬X)` pairs at the **root**:

| fixture | facts | `(false)` | same-layer pairs | **cross-layer pairs** | verdict risk |
|---|---:|---:|---:|---:|---|
| `examples/zebra2.ein` | 372 | 0 | 0 | **0** | none |
| `examples/zebra2-minus-15.ein` | 323 | 0 | 0 | **0** | none |
| `examples/ein-bugs/zebra2-bad.ein` | 554 | 1 | 122 | 3 | none — already dead 123× over |
| `examples/zebra2-hints.ein` | 380 | 0 | 0 | **0** | none |
| `examples/ein-bugs/zebra2-hints.ein` | 286 | 1 | 0 | 1 | none — already dead by `(false)` |
| `examples/zebra.ein` | 216 | 0 | 0 | **0** | none |

The three cross-layer pairs in `zebra2-bad` are exactly the shape the plan
predicted — a derived negation of a *given* clue:

```
+ (color-loc  Green     House-1)  :source "injected contradiction"
- (not …)                          rule adjacent-via-endpoint-bwd
+ (nation-loc Norwegian House-1)  :source "condition (10)"
- (not …)                          rule adjacent-via-fwd-negative
+ (drink-loc  Milk      House-3)  :source "condition (9)"
- (not …)                          rule injective-negative
```

Note the correction to the plan's §Why: these are `zebra2-**bad**`'s root, not
`zebra2`'s. The soundness point stands unchanged — the engine holds a
contradiction it does not report — and it is exactly the plan's "correctness
currently rests on a second mechanism": the `(false)` direct shape.

**Conclusion: no acceptance verdict can move at the root.** The two SAT
fixtures and the ambiguity fixture have a clean root under both rules.

### 7.2 Fork-level reasoning

Inside the search the fix can only ever kill *more*. Killing more is
verdict-affecting only if a killed branch would otherwise have been counted a
solution — and a branch holding both `X` and `¬X` is **not** a model. So any
verdict movement here is a false solution being withdrawn, i.e. the fix
working, not a regression. `zebra2-minus-15` (Ambiguity, k>1) is the fixture
where this would show as k dropping; `zebra2` (k=1) is where it would show as
`Solution`→`Contradiction`. Both are measured end-to-end in T1.22.1b.2's gate,
and the requirement is that neither moves.

### 7.3 The `gaps` view

`acceptance/test_mode_consistency.py` asserts `proof.solutions` (the gaps view)
agrees with the verdict. It reads the same solution-node set, so it moves iff
the verdict moves — it is not an independent risk, it is the cross-check that
catches a silent k-change.

### 7.4 The lookahead direction

`lookahead._is_contradiction` mirrors the detector to *predict* death, under a
one-way contract: it may miss a death (safe — the branch forks and dies
normally) but must never call a live hypothesis dead. Dropping its two
`existing.layer is Layer.REASONING` guards makes it kill more — and each extra
kill is a world that genuinely holds `g ∧ ¬g`, which the fixed detector *will*
flag. So the mirror stays exact; the risk is only in getting the two edits
out of step, which is why they land in one commit with the detector
(precedent: P1.21 D3).

---

## 8. Method

- Mapping (§3.2): `parse` → `from_ir.load` → `closed.emit_closed` →
  `Saturator(kb).saturate()` to fixpoint over all 57 `examples/**/*.ein`
  (4 `broken/` files are parse-error fixtures and excluded by their own
  failure).
- Root risk (§7.1): same pipeline, then a direct walk of
  `kb._facts_by_relation["not"]` resolving each inner against `_fact_by_id`,
  bucketed by `positive.layer is negative.layer`.
- Site inventory (§2): `grep -rn` over `ein.py/src/ein`, then per-file reading
  to assign each hit to a meaning. No hit was classified from the grep alone.
