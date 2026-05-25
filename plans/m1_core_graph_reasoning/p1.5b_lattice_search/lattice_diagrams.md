# P1.5b — Lattice / DAG state-search diagrams

Visualises the moving parts of the search-lattice machinery
across four fully-explored examples. Each diagram pairs a
DOT source + a pre-rendered SVG; the markdown here gives the
reading guide + the key concepts each one illustrates.

| diagram | hyps | feature shown |
|---|---|---|
| 1 | 3 | full powerset, no dedup, no deaths — baseline |
| 2 | 3 | state-hash dedup (cross-layer merge collapsing 3 commitment sets into 1 SetNode) |
| 3 | 3 | 2-clause contradiction (death emits nogood; layer 3 pruned at generation) |
| 4 | 4 | fully explored — dedup + death + cross-layer merge + layer-4 prune |

## Diagrams

### Diagram 1 — 3 hypotheses, no dedup

[`lattice_3hyps_no_dedup.dot`](diagrams/lattice_3hyps_no_dedup.dot) →
[`lattice_3hyps_no_dedup.svg`](diagrams/lattice_3hyps_no_dedup.svg)

The full powerset lattice over `{h₁, h₂, h₃}`. Every commitment
set is a distinct node; every saturation step is a distinct
arc. No state-hash collision occurs in this example, so the
lattice is the pure powerset DAG.

What to look for:

- **Two tiers per layer.** Rounded boxes (lightblue) hold the
  *raw* commitment set — the unordered hypothesis collection.
  Sticky-note shapes (palegreen) hold the *saturated* kb —
  the post-`Saturator(root ∪ C).saturate()` state.
- **Saturation arc.** Every commitment ⟶ kb edge labelled
  `sat` (dashed, gray). This is one full saturator run.
- **Add-hypothesis arc.** Every kb ⟶ next-layer-commit edge
  labelled `+h_i` (solid, black). Adding a hypothesis to a
  saturated parent produces a child *commitment* — which then
  saturates again.
- **Multi-parent intrinsic.** Layer-2 set `{h₁, h₂}` has TWO
  incoming `+h_i` edges — one from `kb({h₁})` adding `+h₂`,
  one from `kb({h₂})` adding `+h₁`. The lattice picks **any
  one** to fork from (saturation commutativity guarantees the
  outcome is identical); the other edge is still recorded in
  the `SetNode`'s `parents` list for audit + multi-parent
  bubble during integrate.
- **Layer 3 has THREE parents.** `{h₁, h₂, h₃}` is reachable
  from `{h₁, h₂}` via `+h₃`, from `{h₁, h₃}` via `+h₂`, and
  from `{h₂, h₃}` via `+h₁`.

### Diagram 2 — 3 hypotheses, state-hash dedup (cross-layer merge)

[`lattice_3hyps_dedup.dot`](diagrams/lattice_3hyps_dedup.dot) →
[`lattice_3hyps_dedup.svg`](diagrams/lattice_3hyps_dedup.svg)

Same powerset over `{h₁, h₂, h₃}`, but a rule under `h₂` makes
`h₁` and `h₃` derivable from each other when `h₂` is committed.
Consequence:

- `sat(root ∪ {h₁, h₂})` derives `h₃` → kb = `{…, h₁, h₂, h₃}`
- `sat(root ∪ {h₂, h₃})` derives `h₁` → same kb
- `sat(root ∪ {h₁, h₂, h₃})` is idempotent → same kb

All three commitment sets state-hash to the same saturated kb.
The search lattice stores them as **one** gold `SetNode` with
**three labels** spanning **two layers** (2 and 3).

What to look for:

- **Three darkorange `sat`-arcs converge** on the merged node
  — two from layer 2 (`{h₁,h₂}` and `{h₂,h₃}`) and one from
  layer 3 (`{h₁,h₂,h₃}`). The third arc is darker
  (`#cc5500`) to emphasise the cross-layer aspect.
- **The merged node's labels accumulate** across layers —
  `labels = ({h₁,h₂}, {h₂,h₃}, {h₁,h₂,h₃})`. The first
  arrival's `canonical_set` is the dict key; subsequent
  arrivals add their canonical-tuples as additional labels.
- **Parent union spans tiers.** The merged node's `parents`
  list pulls from layer-1 kbs (each label's
  `(k-1)`-subsets) AND from layer-2 alive `kb({h₁,h₃})`
  (the layer-3 label's third parent).
- **The "search lattice ≠ true semantic lattice" framing
  made concrete.** Two distinct commitment sets fall on the
  same kb because the underlying rule structure says they're
  semantically equivalent. The merge node *is* the evidence.

### Diagram 3 — 3 hypotheses, 2-clause contradiction

[`lattice_3hyps_contradiction.dot`](diagrams/lattice_3hyps_contradiction.dot) →
[`lattice_3hyps_contradiction.svg`](diagrams/lattice_3hyps_contradiction.svg)

Layer-1 enterings are all alive; at layer 2 the pair `{h₁, h₂}`
saturates to a contradiction. The learned clause
`frozenset({h₁, h₂})` lands in `root._nogoods` and the sole
layer-3 candidate `{h₁, h₂, h₃}` is pruned at generation time
(`matches_any_nogood: {h₁,h₂} ⊆ {h₁,h₂,h₃}`).

What to look for:

- **Salmon kb at `kb({h₁,h₂})`** — the contradicted node.
  Its `sat` arc is coloured red.
- **Dotted red `emit` arrow** to a mistyrose annotation box
  carrying the literal write `root._nogoods += frozenset({h₁, h₂})`.
- **Layer 3 is one dashed-grey box** — the would-be candidate
  `{h₁, h₂, h₃}` that never got forked. Its prospective
  lineage is drawn as dashed grey edges from the alive
  layer-2 parents (`kb({h₁,h₃})` via `+h₂`, `kb({h₂,h₃})`
  via `+h₁`); the dotted red `filter` arrow from the nogood
  box shows what stopped them.
- **Search terminates after layer 2** — verdict is computed
  from the cumulative state (Solved / Ambiguity /
  Contradiction depending on which alive layer-2 set
  satisfies the goal; the diagram doesn't commit to a
  specific verdict, just shows the termination shape).

### Diagram 4 — 4 hypotheses, fully explored

[`lattice_4hyps_dedup.dot`](diagrams/lattice_4hyps_dedup.dot) →
[`lattice_4hyps_dedup.svg`](diagrams/lattice_4hyps_dedup.svg)

Composes every feature into one example. Layers 0–4 of the
powerset over `{h₁, h₂, h₃, h₄}`. Events:

**Layer 2:**
- **Merge `{h₁, h₃}` ≡ `{h₁, h₄}`** — a rule under `h₁`
  derives `h₄` from `h₃` so the two commitment sets state-hash
  to the same kb. Gold node with two labels; multi-parent
  union.
- **Death `{h₂, h₃}`** — salmon node; emits
  `frozenset({h₂, h₃})` into `root._nogoods`.

**Layer 3** (Apriori-gen + filter): the 4 raw candidates are
- `{h₁, h₂, h₃}` — nogood match → **pruned**
- `{h₁, h₂, h₄}` — passes → **alive, new kb**
- `{h₁, h₃, h₄}` — passes → **cross-layer merge into the
  layer-2 merged node** (because saturating
  `kb({h₁,h₃,…,h₄})` is idempotent — `h₄` is already there
  from the layer-2 derivation)
- `{h₂, h₃, h₄}` — nogood match → **pruned**

The cross-layer merge is the key didactic moment: state-hash
dedup is **layer-independent**. The merged node's `labels`
list grows from `({h₁,h₃}, {h₁,h₄})` (layer 2) to
`({h₁,h₃}, {h₁,h₄}, {h₁,h₃,h₄})` (after layer 3).

**Layer 4:** only one possible size-4 set
`{h₁, h₂, h₃, h₄}` — pruned (matches the `{h₂,h₃}` nogood,
and Apriori rules it out because `{h₂,h₃} ∉ A_2`). Search
terminates.

What to look for:

- **Three different orange shades on `sat` arcs:** plain
  `darkorange` for layer-2 dedup arrivals; `#cc5500`
  (browner) for the cross-layer dedup at layer 3.
- **Dashed grey "would form" edges** to pruned candidates —
  these are the prospective lineages that the nogood
  filter intercepted. Drawn so the reader can see what
  *didn't* happen.
- **Layer 3's pruned candidates and layer 4's pruned
  candidate** all share the same dashed-rounded-grey
  shape — pruned-at-generation never forks a fresh kb.
- **The `nogood_emit` annotation box has three outgoing
  `filter` arrows** — one per pruned candidate across
  layers 3 and 4. The single learned clause does work at
  every subsequent layer.

## Reading the diagrams — vocabulary

| visual element            | meaning                                                            |
|--------------------------|--------------------------------------------------------------------|
| rounded lightblue box     | commitment **set** — unordered hypothesis collection, *not yet saturated* |
| palegreen note            | saturated kb — alive (no contradiction)                            |
| gold note                 | merged kb — same state-hash from two or more commitment sets       |
| salmon note               | dead kb — contradiction detected (pre- or post-saturation)         |
| mistyrose dotted note     | side effect — clause emitted to `root._nogoods`                    |
| solid black edge `+h_i`   | kb ⟶ child commitment by adding hypothesis `h_i`                  |
| dashed gray edge `sat`    | saturator run on a commitment                                       |
| dashed darkorange `sat`   | saturator run that triggered state-hash dedup                       |
| dashed red `sat`          | saturator run that detected contradiction                           |
| dotted red `emit`         | nogood-clause emission to root                                      |

## Key concepts illustrated

### True semantic lattice vs search lattice

The **true semantic lattice** is the abstract object defined
by the rule set + the puzzle's ground facts: every reachable
kb-state + every commitment set that produces it, related by
set-inclusion. It is infinite-by-construction (modulo the
finite-model property of the puzzle); we never materialise it.

The **search lattice** is the finite-state artefact we
*build* by exploring the powerset of `alive` and dedup-ing
where state-hash collisions surface. Two distinct
commitment sets that fall on the same kb are evidence that
the true lattice's nodes are coarser than the powerset of
hypotheses — and the merge node in Diagram 2 captures that
evidence directly.

Recorded in
[Q1.5b.4.a](open_questions.md#q15b4--set-equivalence-dedup--state-hash-dedup).

### Multi-parent intrinsic to BFS-by-size

Every layer-`k` set has exactly **`k` parents** in the
lattice DAG — the `k` distinct `(k-1)`-subsets, each of which
is in `A_{k-1}` by the Apriori invariant. The orchestrator
picks **any one** parent to fork from
(commutativity guarantees the saturated child is
parent-choice-independent), but stores all `k` parents in the
`SetNode.parents` list. Integrate bubbles upward through ALL
`k` parents — idempotent under subsumption, so the
multi-parent fan-out costs nothing extra in writes.

Documented in
[`algorithm_layer_n.md`](algorithm_layer_n.md) § 3a + § 3d.iv.

### Saturation arc as a first-class step

Every commitment ⟶ kb edge is one full
`Saturator(fork).saturate()` invocation. The diagrams render
it as a separate arc to make it visible: the search lattice's
*node count* is the powerset size, but its *saturation count*
is the same. Each saturation is one batch of forward chaining
under the rule set.

Under Q1.5b.8's bootstrap-incremental engine bridge, every
saturation runs `try_branch(parent_kb, h)` once — taking `k`
saturations per size-`k` set. The future set-batch primitive
`try_set(parent_kb, commitment_set)` would collapse those
`k` saturations into 1 by adding all hypotheses at once;
parked in F9 perf round (see
[`open_questions.md` § Q1.5b.8](open_questions.md#q15b8--engine-bridge)).

### Dedup as primary structure, not optimisation

State-hash dedup is *not* an optional perf knob — it is what
turns the powerset enumeration into a finite search lattice.
Without it, two semantically-equivalent commitment sets would
produce two `SetNode`s with the same kb, double-running the
integrate step + spawning two parallel subtrees with the
same content. The merge in Diagram 2 is the smallest
non-trivial case where this happens.

Set-equivalence dedup (primary) catches *re-arrival of the
same canonical set* — short-circuits before saturation.
State-hash dedup (secondary) catches *cross-set state
convergence* — short-circuits after saturation. Both are
mandatory; Diagram 2 shows the secondary one fire.

Documented in [`algorithm_layer_n.md`](algorithm_layer_n.md)
§ 3d.iii + § 3e.

## Rendering

```sh
cd diagrams
for f in lattice_3hyps_no_dedup lattice_3hyps_dedup \
         lattice_3hyps_contradiction lattice_4hyps_dedup; do
  dot -Tsvg "$f.dot" -o "$f.svg"
  # dot -Tpng "$f.dot" -o "$f.png"   # for PNG output
done
```

The companion algorithm flow chart is
[`algorithm_layer_n.dot`](diagrams/algorithm_layer_n.dot) /
[`algorithm_layer_n.svg`](diagrams/algorithm_layer_n.svg) — same
rendering tooling, same colour vocabulary.

## Cross-references

- [`README.md`](README.md) — phase intro, motivation, scope.
- [`open_questions.md`](open_questions.md) — all resolved
  design decisions; the diagrams here illustrate the
  Q1.5b.4 (multilabel + multi-parent), Q1.5b.7 (verdict
  trichotomy), and Q1.5b.5 (nogood emit) outcomes.
- [`algorithm_layer_n.md`](algorithm_layer_n.md) — per-layer
  algorithm spec; the diagrams here are its data-model
  companion.
- [S1.5a.20](../p1.5a_zebra_solution/s1.5a.20_branch_isolation_rearch.md)
  — the channel-isolation rewrite that makes per-set
  `BranchResult` + `integrate` the universal up/down channel
  the lattice orchestrator sits on.
