# P1.6 — Rendering + markdown trace

**Estimate:** 1-2 weeks.
**Depends on:** P1.2 (graph + provenance), P1.3 (saturator emits
`Firing`s), **P1.5b** — the monotonic-lattice engine. The renderer's
input is now `Verdict` + `verdict.proof: LatticeProof`, validated by
`validate_proof_for_explanation`; the old P1.5 ordered search tree was
removed 2026-05-29.
**Blocks:** P1.7 (the Zebra acceptance test reads the markdown
trace and asserts the rule firings match the human walkthrough).

> **Re-planned 2026-05-29** in two passes. (1) tree→lattice shift
> (P1.5b): old S1.6.3 search-tree → lattice/DAG; S1.6.4 / S1.6.5
> re-aimed onto `LatticeProof`. (2) output-model decision: **all
> diagrams are inline fenced `dot` blocks in the markdown — no SVG, no
> separate diagram directory**; S1.6.2 re-scoped to per-hypothesis
> derivation slices (whole-KB snapshots kept behind a flag). See
> [§Output format](#output-format--inline-dot-no-svg),
> [§Input contract](#input-contract-p15b--what-the-renderer-consumes),
> [§Net-new prerequisites](#net-new-prerequisites-surfaced-by-the-p15b-audit).

## Goal

Render the engine's work as a *story* — DOT diagrams for state
snapshots and structures, plus a markdown narrative that threads
the diagrams together.

The acceptance criterion is set by
[`docs/ideas/08-human-style-deductive-trace.md`](../../ideas/08-human-style-deductive-trace.md):
"the solver should reproduce a deductive trace of the kind a
human would write". Every named reasoning move in the Zebra
walkthrough must surface as a named rule firing with its premises
and a one-sentence English explanation.

## Output format — inline DOT, no SVG

The trace is a **single self-contained markdown file**; every diagram
is an inline fenced `dot` block, rendered in place. There is no SVG
step and no separate diagram directory — `--trace=out.md` is the only
output path.

Two diagram families:

1. **Lattice / proof DAG** (S1.6.3) — a standalone `dot` block of the
   commitment lattice (the search structure), emitted once in the
   trace's final section.
2. **Per-hypothesis derivation slices** (S1.6.2) — one `dot` block per
   hypothesis/commitment section showing *only* that hypothesis's
   interaction with the KB: the hypothesis fact(s), the specific facts
   and rules they touch (a provenance cone, **not** the whole KB), and
   the firings, with the NL `:why` as labels. **Default on**; suppress
   with `--no-diagrams`.

Slice scope is the **surviving path**: every firing on the
alive/solution path is shown — including exclusion / elimination
derivations — while refuted-branch firings stay folded into their
reductio `<details>` (S1.6.4 T1.6.4.6), rendered only if expanded.

**Kept behind a flag:** whole-KB per-step snapshots (the old S1.6.2
view) via `--full-kb-snapshots`, plus a final full-KB solution grid.

**Viewer note.** Fenced `dot` blocks are *not* auto-rendered by GitHub
(it renders mermaid, not graphviz). Viewing needs a DOT-aware renderer
— `mdcat` + graphviz, the VS Code Graphviz preview, or a pandoc/`gvpr`
filter. Deliberate trade-off: a self-contained, diffable,
copy-pasteable source over GitHub's inline auto-render.

## Input contract (P1.5b) — what the renderer consumes

The renderer no longer walks a search tree. Its inputs are:

- **`Verdict`** (`inference/verdict.py`) — `Solution{kb, trace:
  tuple[Firing,...], proof}`, `Ambiguity{branches, proof}`, or
  `Contradiction{unsat_core, proof}`. The tree-side fields
  (`Solution.tree`, `Ambiguity.{tree,unresolved}`, `SearchNode`) are
  gone.
- **`verdict.proof: LatticeProof`** (`inference/monotonic/lattice.py`)
  — `solutions: tuple[SolutionRecord,...]`,
  `dead_commitments: tuple[DeadCommitment,...]`,
  `kb_index: dict[int, SetNode]` (only under `store_lattice=True`),
  `alive_at_end`, `learned_nogoods`, `stats: LatticeStats`. Populated
  by `gaps_solve` / `contradictions_solve`; `monotonic_solve` leaves
  it `None`.
  - `SolutionRecord = (commitment, kb, firings: tuple[Firing,...], layer)`
  - `DeadCommitment = (commitment, unsat_core, learned_clause, layer, kind)`
  - `SetNode = (state_hash, canonical_set, labels [multilabel], verdict, layer)`
- **`validate_proof_for_explanation(verdict, proof)`**
  (`inference/monotonic/contract.py`) — the handoff gate; asserts 6
  structural invariants the renderer relies on (proof identity,
  solutions goal-sat, dead well-formed, nogood subsumption, SetNode
  consistency, stats coherence).
- **`lattice_snapshot(verdict, root_kb) → LatticeSnapshotV1`**
  (`inference/monotonic/snapshot.py`) — permutation-invariant,
  `state_hash`-collapsed projection; the order-stable structural input
  for the lattice diagram (S1.6.3).

**The conceptual shift.** The human walkthrough
([idea-08](../../ideas/08-human-style-deductive-trace.md),
[`examples/README.md`](../../../docs/kernel/inference/zebra_walkthrough.md)) is a *linear
story keyed to branch-depth `d`*; the engine now produces an
*unordered commitment lattice* (`(layer, commitment-set)`). So
**S1.6.4's new core task is to linearize the lattice into a
depth-ordered story** — mapping `(layer, set-size)` onto the human
`(d, hypothesis)` framing. idea-08 §acceptance permits this: "the same
order or a recognisably equivalent one."

## Stages

| ID      | Title                                  | Duration |
|---------|----------------------------------------|----------|
| S1.6.0  | Compact rendering default + `--levi`; collapse `render_examples.sh` | 2-3 days |
| S1.6.1  | DOT — rules + constraints              | 2 days   |
| S1.6.2  | DOT — derivation slices + KB snapshots  | 2-3 days |
| S1.6.3  | DOT — commitment lattice / proof DAG   | 2-3 days |
| S1.6.4  | Markdown trace renderer                | 4-5 days |
| S1.6.5  | idea-08 trace acceptance               | 2-4 days |

**Re-plan deltas (2026-05-29).** Two passes — the P1.5b tree→lattice
shift and the inline-DOT output-model decision (§Output format):
- **S1.6.0** — new; compact rendering for readable inline DOT (only
  Levi exists today). No dedicated stage file yet.
- **S1.6.1** — output is now inline `dot` (no SVG); rules also surface
  as firing nodes in the S1.6.2 slices. Otherwise unchanged.
- **S1.6.2** — **re-scoped**: primary deliverable is the
  per-hypothesis derivation-slice renderer (provenance cone, not whole
  KB); whole-KB snapshot kept behind `--full-kb-snapshots`.
- **S1.6.3** (2-3 days) — **replaces** the obsolete "DOT — search tree"
  with the lattice/DAG, emitted as an inline `dot` block.
- **S1.6.4 / S1.6.5** — **re-aimed** onto `LatticeProof`; S1.6.4 embeds
  the S1.6.2 slices inline (default on).

## Net-new prerequisites surfaced by the P1.5b audit

Three code gaps block S1.6.4 from idea-08's bar; none were in the
original plan:

1. **`:why` never reaches the firing.** `Rule.why` (`kb/entities.py`)
   threads into `JoinPlan.why` (`inference/compile.py`) but `fire()`
   drops it — neither `Firing` (`inference/firing.py`) nor
   `Provenance` (`kb/provenance.py`) carries it. *Decision:* render at
   trace-time via `render_why(rule.why, firing.bindings)`
   (`inference/why.py`, looking the rule up by `firing.rule`) **vs.**
   add `why: str` to `Firing`. The former keeps `Firing` lean.
2. **`DeadCommitment` has no firing chain.** It carries `unsat_core`
   but not the firings that reached ⊥ (`lattice.py`); idea-08 wants
   "the chain that led to ⊥". The chain is dumped to disk
   (`enterings/<slug>/firings.jsonl`) but not held in memory.
   *Decision:* renderer reads the dump folder **vs.** add `firings` to
   `DeadCommitment`.
3. **`solve --trace` must run a proof-producing entry.**
   `monotonic_solve` returns `proof=None`; only `gaps_solve` /
   `contradictions_solve` populate `LatticeProof`. The trace CLI must
   invoke a lattice entry (likely `contradictions_solve`, or
   `gaps_solve --store-lattice`), not the fast path.

## Defaults — compact view, project-wide; Levi only by flag

User direction recorded 2026-05-27: the canonical Levi-bipartite
graph representation reflects ein-lang structure faithfully
(atoms/names are nodes with arrows *to*, relations as
list-nodes with arrows *from*), but it's not readable as a
default view. **Compact rendering is the default for every
phase that touches DOT output;** Levi-bipartite stays available
on request via a `--levi` flag (or `EIN_RENDER_LEVI=1` env).

**Status (P1.5b audit):** none of this exists in code yet —
`ir/to_dot.py` renders only Levi-bipartite; the `--levi` flag /
`EIN_RENDER_LEVI` env are plan-only. Compact mode is **S1.6.0**,
building on the entity-fused unified view already in
[`kb/render.py`](../../../ein.py/src/ein/kb/render.py)
(`to_dot(kb, …)`). Both `rule-mode` (a/c) and `trace-view=a` already
exist; `trace-view` b (aggregate) / c (DAG) are stubs that emit no DOT
— the DAG view is implemented in S1.6.3.

The mode is set per top-level form:

- **ontology / facts / reasoning** — compact (entity-style:
  instances + arrows, the abstract view).
- **rules** — `rule-mode=a` (Side-by-side LHS | RHS clusters)
  with `rankdir=TB`. The two existing modes ((a) clusters,
  (c) overlay) get folded into one default diagram per rule
  rather than the current cross-product; the `(c)` overlay
  variant moves behind a `--rule-mode=overlay` flag.
- **trace** — per-hypothesis derivation slices inline (S1.6.2),
  default on; the lattice/proof DAG (S1.6.3) in the final section;
  whole-KB snapshots behind `--full-kb-snapshots`. The aggregate view
  stays behind `--trace-view=aggregate`.

## render_examples.sh — collapse the matrix

[`utils/render_examples.sh`](../../../utils/render_examples.sh)
currently produces six variants per input file
(`rule-mode ∈ {a, c} × trace-view ∈ {a, b, c}`). Update it
to render **one variant per file** under the new defaults:
compact rule mode (a, LR), per-step trace mode (a). The other
modes are addressable via env vars / flags for the rare cases
that need them. Note: example puzzles under `examples/`
don't carry `(trace …)` blocks, so the trace dimension is a
no-op for most of them anyway — only the `(rule …)`
output changes.

**Output convention (S1.6.0, 2026-05-30).** The split is by *layer*,
not by feature: **the Python tools emit DOT only** — the `Ein` CLI
(`ir dot`, and the S1.6.1–S1.6.2 `render …` commands), and the S1.6.4
markdown trace (inline fenced `dot` blocks). None of them call
Graphviz. **Rasterising DOT → SVG is a shell-script job**:
`render_examples.sh` takes that DOT and renders it for an examples
overview — **`.dot` + SVG by default** (`--no-svg` / `--dot-only` for
dot-only), with rule diagrams in a **`rules/` subfolder** per example
and the other forms flat.

## VSCode ein syntax highlighting

**Moved (2026-06-02) to
[P1.7c S1.7c.8](../p1.7c_block_head_removal/s1.7c.8_vscode_syntax_highlighting.md).**
It highlights the IR *surface* syntax, which P1.7c flattened (the old
keyword list here still named the removed `facts` / `ontology` / `rules`
block wrappers), so the editor grammar belongs next to the surface-syntax
work, not the DOT renderer. The DOT renderers (S1.6.1–.4) stay in P1.6.

## Acceptance

- `ein solve zebra.ein --trace=out.md` writes a single
  self-contained markdown file (no SVG, no diagram directory). The
  `--trace` path must run a proof-producing entry (see §Net-new
  prerequisites, gap 3).
- `out.md` reads as a coherent narrative: each step names the rule,
  quotes the source condition, and carries an inline `dot` derivation
  slice (default on; `--no-diagrams` to suppress); the final section
  embeds the lattice/proof DAG `dot` block (S1.6.3) + solution grid.
- The trace's named rule firings match
  [the target walkthrough](../../ideas/08-human-style-deductive-trace.md#the-target-trace-paraphrased)
  one-to-one (P1.7 enforces this) — in the same order **or a
  recognisably equivalent one** (the lattice imposes no hypothesis
  order; the renderer linearizes by `(layer, lattice_order)`).
- Rendered through a DOT-aware viewer (`mdcat` + graphviz / VS Code
  Graphviz preview), every inline `dot` block produces a valid diagram.

## Connections

- [Idea 08](../../ideas/08-human-style-deductive-trace.md) —
  the whole phase is about delivering its acceptance criterion.
- [Idea 03 §The implicit fourth class](../../ideas/03-three-task-classes.md) —
  the *explanation* task class falls out of this rendering work.
- [P1.5b](../p1.5b_lattice_search/README.md) — the monotonic-lattice
  engine producing the `LatticeProof` this phase renders; S1.5b.29
  shipped the `validate_proof_for_explanation` handoff contract.
