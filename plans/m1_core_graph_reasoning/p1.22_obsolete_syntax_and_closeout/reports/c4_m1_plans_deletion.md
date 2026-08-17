# C4 — M1-plans preservation census

**Stage:** [S1.22.99](../s1.22.99_m1_plans_deletion.md), task T1.22.99.1
(read-only). **Date:** 2026-08-17. **Tree:** post-S1.22.4 (`b28b0f3`).

> **Naming note.** The stage brief calls this report `c4_m1_plans_deletion.md`
> while S1.22.4's decision memo is `c4_relation_kernel_word.md` — the `c<n>`
> sequence follows *execution* order and S1.22.4 executed first. Both files
> exist; the duplicate `c4` is a wart in the numbering scheme, not an error
> in either report. If it matters at deletion time, this one is the later
> `c4` and could be renumbered `c5`; nothing links to it by number.

---

## 0. Summary

| question | answer |
|---|---|
| Inbound refs outside the folder | **261** across **69 files** — 191 markdown links, 70 bare path citations (§1) |
| Biggest surprise | `docs/kernel/` does not merely *cite* the folder, it **delegates content to it** in ~20 places — algorithm spec, perf baselines, open questions, review §s (§2). Deleting without migrating leaves the kernel docs pointing at nothing for material they do not carry. |
| Is the brief's "known live" list accurate? | **Two of six items are wrong** — S1.21.7 / S1.21.8 are ✅ executed, not parked, and every P1.21 divergence (D3, D5, D-R5-1, D-S8-1…5) is closed (§3.1). |
| Content the brief missed | M1 `open_questions.md` is the *body* behind 14 index rows in `plans/open_questions.md` and 13 deep anchors in `docs/`; plus 10 diagram files, `algorithm_layer_n.md`, `lattice_diagrams.md`, `parity_baselines.md`, `REVIEW_M1-01.md`, `STATUS.md` (§2, §3.2). |
| Rehoming scheme | **`plans/followups/`**, with *directory-shaped* entries for the two unexecuted bodies — precedent already exists (`f8_FCA_RCA_odis_tptp/`). No new `plans/backlog/` (§4). |
| Folder size | 291 files (281 `.md`, 5 `.dot`, 5 `.svg`), 3.5 MB, 238 stage files |
| Safe to execute T1.22.99.2 as briefed? | **Not as briefed** — the brief's order (migrate → rewire → delete) is right, but its migration list is incomplete. §5 gives the corrected manifest. |

---

## 1. Inbound-link inventory

`grep -rn m1_core_graph_reasoning` over the tree, excluding `.git` and the
folder itself.

| area | refs | files | dominant form | disposition |
|---|---:|---:|---|---|
| `docs/kernel/**` | 109 | 24 | markdown links | **mixed** — most reword, ~20 need migration first (§2) |
| `plans/open_questions.md` | 19 | 1 | index rows | **retarget** — the body moves here (§3.2) |
| `ein.py/src/**` | 26 | 16 | bare paths in docstrings | **reword** to a bare stage id |
| `ein.py/tests/**` + `acceptance/` | 15 | 13 | bare paths in docstrings | **reword** |
| `plans/followups/**` | 11 | 4 | markdown links | **retarget** to the rehomed entries |
| `plans/m1a_rust/README.md` | 7 | 1 | links to PyPy-perf + Track B | **retarget** (§3.3 — these are *inputs* to the port) |
| `plans/ideas.md` | 7 | 1 | links to P1.3/P1.4 stages + Q-rows | reword; Q-links retarget |
| `plans/m1b_gui`, `m2`, `m2b`, `m3` READMEs + `m3/open_questions.md` | 10 | 5 | "depends on M1" links | **reword** to "M1 (shipped 2026-06-17)" |
| `plans/README.md` | 3 | 1 | roadmap row + tree diagram | **reword** (§5) |
| `utils/vscode-ein/README.md` | 3 | 1 | links to S1.7c.8 / S1.5.9 / S1.7c.4 | **reword** |
| `examples/README.md`, `examples/saturation/README.md` | 2 | 2 | links | **reword** |
| root `README.md` | 1 | 1 | link to P1.11 | **reword** |

**The 70 bare citations are uniform and mechanical** — docstring lines of
the form ``Designed in `plans/m1_core_graph_reasoning/p1.2_typed_hypergraph/`.``
in `ein/__init__.py`, `ir/__init__.py`, `kb/__init__.py`, `kb/store.py`,
`render/__init__.py`, `inference/{solution,primitives,hypgen}.py`,
`inference/monotonic/{solver,lattice,snapshot}.py`, `ir/grammar.lark`, and
12 lattice/trace test modules. All become "S1.2.1 (M1 P1.2; see git
history)". None is load-bearing.

---

## 2. The finding: `docs/kernel/` delegates content, it does not only cite

Of the 109 `docs/kernel/**` references, the large majority are **provenance
citations** — `[S1.7.23](…)` attached to a sentence that already states the
fact. Those reword safely: the stage id stays as text, the link goes.

But **~20 references are normative delegations** — the doc states a claim
and sends the reader into the plans folder *for the substance*. These are
the deletion's real risk, and the stage brief does not list them:

| doc site | delegates to | what would be lost |
|---|---|---|
| `inference/lattice_dump.md:267` · `inference/README.md:1010` | `p1.5b_lattice_search/algorithm_layer_n.md` (529 ln) | **the layer-N algorithm spec** — cited as "Algorithm spec", and by `monotonic/solver.py`, `monotonic/lattice.py`, `test_contradictions_backbone.py` |
| `inference/README.md:1004` | `p1.5b_lattice_search/parity_baselines.md` (110 ln) | the monotonic-vs-lattice parity baselines |
| `inference/README.md:1009` | `p1.5b_lattice_search/open_questions.md#q15b7` (972 ln) | **the equivalence claim** behind the two engines |
| `inference/README.md:546, 744` | `p1.5a_zebra_solution/STATUS.md` (152 ln) | the zebra solve status/perf record |
| `inference/absent_semantics.md:4` · `architecture.md:107` | `REVIEW_M1-01.md` §4, §6 (420 ln) | the external review's wording that drove P1.21 |
| `inference/README.md:763` | `p1.21_review_response/reports/r2_unconditional_facts.md` | the retirement analysis |
| `architecture.md` ×2 · `m3/open_questions.md` | `p1.21_review_response/{s1.21.8,reports/r6_seam.md}` | the closure/worlds seam |
| `inference/README.md:916, 942, 944` · `:412` | `p1.9_hypothesis_loop_followups/README.md` + E8/E20/E23 | the E-catalog — **self-declared authoritative**: *"until then this README is the authoritative spec"* |
| `inference/README.md:177, 406` | `s1.5.4_hypgen_improvements.md#open-questions`, `s1.5a.1_naf_semantic_rearch.md#open-questions` | two parked Q-rows quoted as working answers |
| `inference/README.md:659` | `p1.22…/reports/c2_zebra_ein_gap.md` | the zebra.ein gap analysis |
| 13 distinct `open_questions.md#q…` anchors across 8 docs | M1 `open_questions.md` (784 ln) | Q1, Q3, Q4, Q15, Q17, Q18 (×2), Q19, Q21 (×5), Q26, Q27, Q28, Q30 |

Note the last row: docs deep-link **resolved** questions (Q21 five times,
Q3, Q27) as the *record of the decision*, not just the open ones. The
brief's "Q26/Q28 still-open rows" framing under-scopes it by an order of
magnitude.

`examples/lattice/01_subset_pruned.ein` and `02_genuine_3set_death.ein`
also reference `lattice_diagrams.md` (297 ln) by name in their header
comments, and that file is backed by **10 diagram files**
(`p1.5b_lattice_search/diagrams/*.{dot,svg}`) the brief does not mention.

---

## 3. Live-content disposition

### 3.1 Corrections to the brief's "known live" list

| brief says | actual state |
|---|---|
| "P1.21 parked follow-up stages S1.21.7 and S1.21.8" | **Both ✅ executed 2026-08-17.** The files are history; only their *inbound links* (from `frontier.py`, `architecture.md`, `inference/README.md`, `test_frontier.py`, M3 Q30) need retargeting. |
| "P1.21 divergences — D3, D5 (…promote-to-fix candidate), the `_helpers.py` `landed` NameError" | **All closed.** P1.21 README: *"✅ PHASE COMPLETE 2026-08-17. Every divergence is closed"* — D3/D5 by S1.21.8, D-R5-1 fixed, and five more (D-S8-1…5) found and fixed by S1.21.7/.8 + S1.22.0. Each has a regression pin. Nothing to rehome as *open work*; the **lessons** are the only candidate. |
| "P1.9 E-catalog (E1–E24)" | Correct and live. README (231 ln, self-declared authoritative) + **25** stub stage files. |
| "P1.7c Track B (S1.7c.10–.32)" | Correct and live. **23** stage files, explicitly recommended *before the M1a Rust port*. |
| "M1 `open_questions.md` still-open rows (Q26, Q28)" | Under-scoped — see §2. The whole file is the referent of 14 `plans/open_questions.md` index rows and 13 doc anchors. |
| "P1.21 reports — default die with the folder" | Agreed for r1/r3/r5; **r2, r4, r6 are cited from `docs/kernel/`** and need their cited claims absorbed first. |

### 3.2 Disposition table

| item | size | disposition |
|---|---|---|
| **P1.9 E-catalog** | README + 25 stubs | **Relocate wholesale** → `plans/followups/f9_e_catalog/` (directory entry) |
| **P1.7c Track B** | 23 stage files | **Relocate wholesale** → `plans/followups/f10_m1_refactor_tail/`; keep the "do before the Rust port" note and retarget `m1a_rust/README.md` |
| **M1 `open_questions.md`** | 784 ln | **Merge into `plans/open_questions.md`** — the index absorbs the bodies; ids are already sticky by that file's own rule, so anchors are preservable |
| **P1.5b `open_questions.md`** | 972 ln | Merge the **cited** rows (Q1.5b.7 equivalence claim at minimum) into `docs/kernel/inference/README.md`; the rest dies |
| **`algorithm_layer_n.md`** + `diagrams/` | 529 ln + 10 files | **Move to `docs/kernel/inference/`** — it is kernel algorithm documentation that source docstrings cite; it was never really a plan |
| **`lattice_diagrams.md`** | 297 ln | Move to `docs/kernel/inference/` (referenced from `examples/lattice/*.ein`) |
| **`parity_baselines.md`** | 110 ln | Move to `docs/kernel/inference/` or inline into its README §; either way it must survive |
| **`p1.5a/STATUS.md`** | 152 ln | Absorb the two cited claims into `inference/README.md`; then die |
| **`REVIEW_M1-01.md`** | 420 ln | **Move to `docs/kernel/`** — it is the external review of the shipped engine, cited from two kernel docs, and is not a plan |
| **P1.21 reports r2 / r4 / r6** | — | Absorb cited claims into the citing docs; then die with the folder |
| **P1.21 divergences** | — | Closed. Fold the *lessons* into `docs/kernel/inference/absent_semantics.md §Known divergences` (which already carries some); no backlog entry |
| **P1.21 reports r1 / r3 / r5**, **P1.22 reports c0–c4**, all executed stage files, `TODO.md`, `p1.7a/*.md`, `p1.7b/findings.md` | ~230 files | **Die with the folder** — superseded by the docs they produced; history in git |

Note `p1.7b/findings.md` (the 40-finding review) dies *only* because Track
B carries its unexecuted remainder; the Track B stage files each quote
their finding, so relocating Track B preserves what is still actionable.

### 3.3 One forward dependency worth naming

`plans/m1a_rust/README.md` cites the M1 folder **7 times**, including
S1.5a.6 / S1.5a.13.1 PyPy perf measurements as *inputs* to the port and
P1.8 Theme B. Those are not history for M1a — they are its baseline. The
migration must land those numbers somewhere M1a can still read (simplest:
inline the figures into `m1a_rust/README.md` at rewire time).

---

## 4. Scheme: `plans/followups/`, not a new `plans/backlog/`

**Recommended: `plans/followups/`.** Justification:

1. It already exists with a stated working agreement, and both surviving
   bodies fit its definition — *"themes that are neither MVP-blocking nor
   on the M1-M2-M3 schedule"*.
2. A second parking directory would need a rule distinguishing it from
   followups. There isn't one; two homes for one purpose is precisely the
   "one definitive home" discipline this repo applies elsewhere.
3. `plans/README.md` and four milestone READMEs already point at
   `followups/` as the parking lot. No new wiring.

**The one tension, and how to resolve it.** Followups' working agreement
says each file is a *one-page placeholder* and *"if a followup starts to
acquire concrete tasks, promote it"*. The E-catalog (25 stubs) and Track B
(23 stages) are the opposite — detailed, already-written stage specs.
Flattening them to one page each would **destroy** the very content this
census exists to preserve.

Resolve it with **directory-shaped followups**, for which precedent
already exists: `f8_FCA_RCA_odis_tptp/` is a directory. So:

- `plans/followups/f9_e_catalog/` — `README.md` (the authoritative catalog,
  moved verbatim) + the 25 stubs.
- `plans/followups/f10_m1_refactor_tail/` — `README.md` (Track B section of
  the P1.7c README, extracted) + the 23 stage files.

and amend `followups/README.md`'s working agreement with one sentence: a
followup may be a directory when it carries already-written stage specs.

**Numbering.** F9 is free — S1.22.4 created and then deleted an `f9_…`
when the user reversed that stage's park decision, so no committed content
ever used it. F10 is next. (F8's row was missing from the index; S1.22.4
added it.)

---

## 5. Deletion manifest

**Created**

```
plans/followups/f9_e_catalog/              (26 files, moved)
plans/followups/f10_m1_refactor_tail/      (24 files, moved)
docs/kernel/inference/algorithm_layer_n.md (+ diagrams/, 10 files)
docs/kernel/inference/lattice_diagrams.md
docs/kernel/inference/parity_baselines.md
docs/kernel/REVIEW_M1-01.md
```

**Edited**

```
plans/open_questions.md          absorb the M1 question bodies; retarget 14 rows
plans/README.md                  M1 row + tree diagram (wording below)
plans/followups/README.md        F9/F10 rows + the directory-followup sentence
plans/m1a_rust/README.md         inline the PyPy figures; retarget Track B
plans/m1b_gui|m2|m2b|m3 READMEs  reword "depends on M1"
plans/m3_smt_integration/open_questions.md   Q30 seam link
plans/ideas.md                   reword 5, retarget 2 Q-links
docs/kernel/**                   24 files: ~150 rewords, ~20 retargets
ein.py/src/**                    16 files, docstring path citations
ein.py/tests/** + acceptance/    13 files, docstring path citations
examples/README.md, examples/saturation/README.md, examples/lattice/*.ein
utils/vscode-ein/README.md, README.md
```

**Deleted**

```
git rm -r plans/m1_core_graph_reasoning
```

**`plans/README.md` M1 row wording** (line 147):

> `| M1 | full (stages-as-files) | **shipped** — done 2026-06-17 (gate green); plans removed at P1.22, see git history | ~3 months |`

with the link dropped from line 19 and line 89's tree entry reworded to
`m1_core_graph_reasoning/   MVP — shipped; plans removed at P1.22`.

---

## 6. Order of operations for T1.22.99.2

No intermediate commit may have a dangling link, so **migrate before
rewire, rewire before delete**:

1. **Move surviving content** (`git mv`, so history follows): the two
   followup directories, the four `docs/kernel/` files + `diagrams/`.
   Commit. *Tree is still consistent — nothing deleted yet, new copies are
   additive.*
2. **Absorb** the cited claims that are not whole-file moves: M1
   `open_questions.md` bodies → `plans/open_questions.md`; Q1.5b.7 →
   `inference/README.md`; `STATUS.md` figures → `inference/README.md`;
   r2/r4/r6 claims → their citing docs; PyPy figures →
   `m1a_rust/README.md`. Commit.
3. **Rewire all 261 references** — retarget the ~20+14 delegations to the
   homes from steps 1–2, reword the rest to bare stage ids. Commit.
4. **Verify before deleting**: `grep -rn m1_core_graph_reasoning` returns
   hits *only* inside the folder. This is the gate — if anything outside
   still matches, step 3 is incomplete.
5. **`git rm -r plans/m1_core_graph_reasoning`** + the `plans/README.md`
   wording. Commit.
6. **Post-verify**: `grep -rn m1_core_graph_reasoning` (excluding `.git`)
   is empty; markdown link-resolution sweep over every edited file;
   `./run_tests.sh` + `ruff check .` green.

**Gate:** tests should be untouched by steps 1–5 except for docstring
comment edits; acceptance verdicts unchanged. Any test failure means a
migration moved something the engine reads, not just something a human
reads — stop and re-examine.

---

## 7. Recommendation

The deletion is sound and the user's ruling stands — but **step 1–2 are
not optional bookkeeping, they are the whole risk**. Roughly 34 references
in `docs/kernel/` and `plans/open_questions.md` are load-bearing: the docs
assert claims whose justification lives only in the folder. Deleting first
and fixing links afterwards would silently convert "documented" into
"asserted".

The single largest judgement call for the user: **`algorithm_layer_n.md`,
`lattice_diagrams.md`, `parity_baselines.md` and `REVIEW_M1-01.md` are not
plans.** They are kernel documentation that happened to be written inside a
plan folder. Moving them to `docs/kernel/` is the recommendation; the
alternative — letting them die and thinning the kernel docs' claims to
match — is a real option but a lossy one, and should be an explicit choice
rather than a side effect of `git rm -r`.
