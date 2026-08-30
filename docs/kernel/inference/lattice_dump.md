# Lattice dump — per-hypothesis emission tracking

> **Purpose.** When you need to know *which hypotheses the engine
> tested at each layer* and *what every one of them derived —
> survivors and casualties alike* — run the exhaustive
> [`solve`](../../../ein.rs/crates/ein-infer/src/solve.rs) sweep
> with a
> [`LatticeDumper`](../../../ein.rs/crates/ein-render/src/dump/lattice.rs)
> attached. The on-disk dump is the audit trail for debugging
> **problem statements** (is the puzzle even consistent? which
> committed pair kills it?) and **rules** (did the rule I expected
> fire under this commitment? did it fire when it shouldn't?).

This page covers the **exhaustive** lattice sweep —
[`solve`](../../../ein.rs/crates/ein-infer/src/solve.rs) run
with `store_lattice=True` and an unbounded stop policy (`stop_after=None`)
— which tests *every* commitment set layer-by-layer and so produces a
complete per-hypothesis record. The default single-solution
[`solve`](README.md#set-indexed-search--monotonic-engine-p15b-s15b010)
(`stop_after=1`) early-terminates on the first solution and uses the lighter
[`MonotonicDumper`](../../../ein.rs/crates/ein-render/src/dump/state.rs)
(timeline + per-layer root snapshots only — no per-hypothesis folders, since
most hypotheses are never reached), which **is** on the CLI:
`ein solve --dump-states DIR`.

---

## How to run it

### From the CLI

[`ein solve`](../../../ein.rs/crates/ein-cli/src/solve.rs) runs the exhaustive
sweep under `--exhaustive` (the default stop policy is a single
solution); `--trace FILE` builds the `store_lattice` `LatticeProof` and
renders the reductio markdown (every refuted commitment, foldable, plus
the lattice DAG):

```sh
# Exhaustive sweep, both views written into the markdown trace:
ein solve examples/branching/04_two_levels.ein \
    --exhaustive --trace ./trace.md
```

That one sweep records both views at once — the satisfying commitments
and the dead ones with their refutations — because the proof carries
`proof.solutions` (gaps) and `proof.dead_commitments` + the verdict's
`unsat_core` (contradictions); there is no per-view command to choose
between.

The CLI does **not** surface the on-disk `LatticeDumper` tree
(`enterings/`, `proof_summary.json`, …): `--dump-states` builds a
`MonotonicDumper`, and no flag builds the other one. That per-hypothesis dump
is reachable from Rust only — see *Programmatically* below.

An exhaustive `zebra2` lattice sweep is large — bound it with
`--max-set-size N`, `--max-time S`, or `--max-enterings K` so the dump stays a
manageable size. (This paragraph used to recommend a PyPy interpreter to make
the sweep finish at all; the engine that needed one left at M1a S1a.10.5, and
the exhaustive solve is now seconds.)

### Programmatically

> **Superseded — this recipe was the Python engine's.** It imported
> `ein.inference.monotonic`, `ein.ir` and `ein.kb.store` from `ein.py`, which
> was deleted at M1a
> [S1a.10.5](../../history/m1a_rust/README.md#s1a105--the-removal);
> there is no module to import, and `pip install` is not a channel
> ([`docs/install.md`](../../install.md)). The **dump format below is not
> superseded** — it is what `LatticeDumper` writes today, banked by
> [`golden_dump.rs`](../../../ein.rs/crates/ein-render/tests/golden_dump.rs)
> (the timeline and the whole `enterings/` subtree) and byte-compared for the
> rest by `dump_parity.rs`. What changed is only how you ask for it.

The dumper is a Rust type,
[`ein_render::dump::LatticeDumper`](../../../ein.rs/crates/ein-render/src/dump/lattice.rs),
handed to [`ein_infer::solve::solve`](../../../ein.rs/crates/ein-infer/src/solve.rs)
as its `dumper` argument — the shape
[`golden_dump.rs`'s `run_dump`](../../../ein.rs/crates/ein-render/tests/golden_dump.rs)
uses. **No CLI flag reaches it**, and that is § *How to run it*'s subject.

`LatticeDumper::new(None)` makes every hook a no-op (the call sites stay
uniform, nothing hits disk) — useful for a wrapper that streams the
lifecycle events somewhere else.

---

## Layout

Grouped **by layer** throughout (S1.5b.30), so the dump reads in the
same order the engine explores — layer 1 singletons, then layer 2
pairs, and so on:

```text
dump/
├── 00_root_initial.ein          ← root KB after Phase-1 saturation, before any hypothesis
├── 00_timeline.jsonl            ← chronological event log (one JSON record per line)
├── layers/
│   └── layer_NN/
│       ├── pre.ein              ← root.kb at the start of layer NN
│       └── post.ein             ← root.kb at the end of layer NN (after the inter-layer
│                                  (not h) writebacks + forced-positive promotions)
├── enterings/                   ← ★ per-hypothesis emission tracking
│   └── layer_NN/
│       └── <C-slug>/            ← one commitment tested at layer NN
│           ├── commitment.json          ← the committed FactId list
│           ├── outcome.txt              ← alive | dead-pre | dead-post | solution
│           ├── firings.jsonl            ← every rule firing in this fork  (non dead-pre)
│           ├── kb.ein                   ← the fork's full saturated KB    (solution only)
│           ├── unsat_core.jsonl         ← smallest given-fact explanation (dead-* only)
│           └── learned_clause.json      ← the learned no-good emitted     (dead-* only)
├── proof_summary.json           ← top-level index (solutions, deads, alive_at_end, stats)
└── summary.json                 ← cumulative stats + verdict kind + wall time
```

**There is no `kb_index/`.** It is specified below anyway, because
`proof_summary.json` carries an (always empty) `kb_index` list and a reader of
that file needs to know why — § `kb_index/` — never materialises.

### `<C-slug>` — the commitment slug

A commitment is a *set* of [`FactId`](../../../ein.rs/crates/ein-core/src/facts.rs)s
`(relation_name, args)`. The slug joins each FactId as
`relation_arg0_arg1`, with multiple FactIds joined by `+`, and `_`
inside identifiers rewritten to `-` so the field separator stays
unambiguous (see `_commitment_slug`). Examples:

| commitment                                       | slug                                  |
|--------------------------------------------------|---------------------------------------|
| `{(co-located, [Blue, H3])}`                     | `co-located_blue_h3`                  |
| `{(co-located, [Blue, H2]), (co-located, [Green, H2])}` | `co-located_blue_h2+co-located_green_h2` |
| `{}` (root)                                      | `root`                                |

---

## Reading per-hypothesis emissions (positive **and** negative)

Each `enterings/layer_NN/<C-slug>/` folder is the complete record of
one hypothesis test. `outcome.txt` classifies it; the other files
are the emissions:

- **`alive`** — the fork saturated without contradiction and did not
  satisfy the goal. `firings.jsonl` is every rule that fired under
  this commitment (the **positive** emissions); the derived facts
  stay in the fork — nothing merges back into the shared root
  (P1.21 R2; see [the retired unconditional-facts
  note](README.md#unconditional-facts--retired-s157--p121-r2)).
- **`solution`** — same as alive plus the goal was satisfied;
  `kb.ein` is the fork's full saturated KB so you can read the
  solved state.
- **`dead-post`** — the fork saturated *to a contradiction*.
  `firings.jsonl` still records what fired on the way down (so you
  can see the derivation that led to the clash); `unsat_core.jsonl`
  is this dead commitment's **smallest explanation** — the smallest
  set of given facts (clues plus the committed hypotheses) from which
  one recorded contradiction follows, searched across every recorded
  derivation and so independent of firing order, though **not** a
  subset-minimal MUS
  ([`explain::smallest_contradiction_frontier`](../../../ein.rs/crates/ein-infer/src/explain.rs),
  the AND/OR search in
  [`explain.rs`](../../../ein.rs/crates/ein-infer/src/explain.rs)); and
  `learned_clause.json` is the `frozenset(C)` nogood emitted so no
  superset is re-entered
  ([learned no-goods](README.md#learned-no-goods-s15b6)).
- **`dead-pre`** — the commitment was rejected *before* saturation
  (apriori superset of a known nogood, or a `_negated_facts` hit), so
  there are no firings to record — only `unsat_core.jsonl` (same
  smallest-explanation shape, over the pre-saturation clash) +
  `learned_clause.json`.

`firings.jsonl` records, per line: the `rule` name, its `activator`
relation, the `bindings`, a `redundant` flag (the conclusion was
already present), the `derived` fact, and the `premises` it fired
from — the same shape used by the trace renderer. This is what makes
the dump a rule-debugging tool: you see *exactly* which rule fired,
on which bindings, in the context of each tested hypothesis.

> The "positive and negative" axis is **two** things at once, and the
> dump captures both: (1) every hypothesis the engine tested,
> surviving (`alive`/`solution`) or refuted (`dead-*`) — read off
> `outcome.txt` across the tree; and (2) within each fork, both the
> positive firings and the derived negatives (`(not …)` facts appear
> among `firings.jsonl`'s derived facts, since the [d=0
> negative-completion rules](README.md#d0-negative-completion-s15a19)
> emit them as ordinary derived facts). A *derived* negative is
> not a frontier fact, so it does not show up in `unsat_core.jsonl` —
> a `(not …)` line there was **given**, not derived.

---

## `kb_index/` — never materialises

> **This folder has never been written, by either engine.** The design is kept
> below because `proof_summary.json` carries a `kb_index` list, and an empty
> one is not a lost dump.
> [`dump/lattice.rs`](../../../ein.rs/crates/ein-render/src/dump/lattice.rs)
> emits `("kb_index", Json::Array(Vec::new()))` under the comment *"Empty by
> construction — see the module docs"*, and those docs give the reason: the
> per-`SetNode` DAG is built only by a builder that nothing on the shipping
> path calls — the same fact that makes `ein render lattice --view full`
> always take its fallback
> ([S1a.5.1](../../history/m1a_rust/README.md#s1a51--dot-renderers)). It is
> ein.py's behaviour too, not a port gap, and
> [`lattice_semantics.rs`](../../../ein.rs/crates/ein-infer/tests/lattice_semantics.rs)
> pins it: `kb_index` empty, `state_key_merges` 0.

**The dedup itself is real** — state-**key** dedup, P1.21 R1: exact
canonical-representation equality, so two commitments whose post-saturation
KBs are identical are one node. What is absent is only the *stored index of
those nodes*, which is what this folder would have held. The identity is
[`canon::state_key`](../../../ein.rs/crates/ein-infer/src/canon.rs); how often
it merged is **`proof_summary.json`'s `stats.state_key_merges`** (the field
`summary.json`'s cumulative stats do not carry — the two blocks are
`lattice_stats_json` and `stats_json`, and only the first is per-proof).

The design, for the record: per-layer ordered ids `kb_index/layer_NN/kb_<i>/`
rather than hash-named folders, nodes sorted within a layer and numbered
`kb_0 … kb_n`; `state_hash.txt` and `proof_summary.json`'s `state_hash_hex`
carrying a 16-hex **display digest** (`canon::state_digest` of the key) — an
eyeball id for nodes *within one dump*, and **never identity**, since distinct
states may share one and the digest is taken over interned fact ids, so it
moves whenever the ids do; `labels.json` listing every commitment that mapped
onto the node.

---

## `00_timeline.jsonl` — the chronological story

One JSON record per line, in firing order, each with a monotonic
`seq` and a `ts_ms` offset. Event types: `root_initial`,
`layer_start`, `entering` (one per commitment tested, carrying
`outcome`, `commitment`, `kind`, `firings` count, `unsat_core_size`,
nogood flags), `layer_end`, `proof_summary`, `summary`. Reading it
top-to-bottom replays the search; `jq` over it is the fastest way to
answer "how many commitments died at layer 2?":

```sh
jq -c 'select(.event=="entering" and .outcome=="dead-post") | .commitment' dump/00_timeline.jsonl
```

`proof_summary.json` is the post-hoc index: `solutions` and
`dead_commitments` each carry a `path` into the `enterings/` tree, so
it's the entry point for "show me every refutation" tooling.

---

## Debugging workflows

- **"Is my problem statement consistent?"** — run an exhaustive sweep
  with a `LatticeDumper` attached. If `00_root_initial.ein` already
  contains `(false)`, the puzzle is inconsistent before any hypothesis
  (Phase-1 contradiction) and the verdict is `Contradiction` (k=0).
  Else, scan `enterings/layer_01/*/outcome.txt`: a singleton that dies
  `dead-post` means that one fact is incompatible with the givens. (For
  the verdict alone, without the dump, `ein solve --exhaustive` suffices.)
- **"Why did commitment {A,B} get pruned?"** — find its
  `learned_clause.json`; the clause is the dead commitment set itself
  (subset-minimal among *explored* sets by BFS + Apriori construction,
  not a MUS). Its `unsat_core.jsonl` names the *givens* that force the
  clash — the smallest such set the recorded derivations support, often
  a single fact — so chase them forward through `firings.jsonl` to the
  firing that closed the contradiction.
- **"Did rule R fire where I expected?"** — `grep '"rule": "R"'`
  across `enterings/**/firings.jsonl`. Empty under a commitment where
  you expected it means a `:match` premise (often an
  [`(absent …)` NAF guard](README.md#naf-semantics--the-closureworld-boundary-s1218))
  didn't hold in that fork.
- **"Two commitments should reach the same state but don't"** — ground truth
  is the state itself: `diff` the two forks' `kb.ein` under `enterings/`. If
  they are identical the engine merged them, and `proof_summary.json`'s
  `stats.state_key_merges` counts it; if they differ where you expected, the
  rule set is non-confluent. There is **no per-node folder to compare** —
  `kb_index/` never materialises (above), so this workflow reads the two forks
  and the counter.

---

## Reachability — what can ask for this dump

Three questions, and the honest answers as of M1e S1e.2.2:

| | |
|---|---|
| **From the CLI?** | **No.** `--dump-states DIR` builds a `MonotonicDumper` — `00_root_initial.ein`, `00_timeline.jsonl`, `layers/layer_NN_{pre,post}.ein`, `summary.json`, and no `enterings/` or `proof_summary.json`. `make_dumper` in [`ein-cli/src/solve.rs`](../../../ein.rs/crates/ein-cli/src/solve.rs) chooses between `ProgressDumper`, a timing dumper, `MonotonicDumper` and none; `LatticeDumper` is not among them |
| **From Rust?** | **Yes.** `LatticeDumper::new(Some(dir))` passed as `solve`'s `dumper` argument, with `SolveOptions { stop_after: None, store_lattice: true, .. }` — both types are `pub` and the shape is [`golden_dump.rs`'s `run_dump`](../../../ein.rs/crates/ein-render/tests/golden_dump.rs). It is not in [`docs/api/rust.md`](../../api/rust.md), whose worked example stops at solve-and-render |
| **Is the format pinned?** | **Yes**, which is why this page is *current* rather than superseded: `golden_dump.rs` banks the timeline and every file under `enterings/`, and `dump_parity.rs` banked the rest byte for byte against ein.py |

So the artifact is real, produced and tested, and the only thing missing is a
**documented way to ask for it** — which is a decision about the shipping
surface, not one a doc pass takes.
[`ein_render::kb_dot`](../../../ein.rs/crates/ein-render/src/kb_dot.rs) is in
exactly the same position for exactly the same reason
([`utils/render_examples.sh`](../../../utils/render_examples.sh) says so at its
own head, and declines it too), so it is **one** question with two instances:
[Q-M1e.20](../../../plans/m1e_review_processing/open_questions.md#q-m1e20--two-renderers-are-produced-tested-and-unreachable).

---

## Cross-links

- Engine overview: [README § Set-indexed search](README.md#set-indexed-search--monotonic-engine-p15b-s15b010).
- Implementation: [`ein-render/dump/lattice.rs`](../../../ein.rs/crates/ein-render/src/dump/lattice.rs)
  (`LatticeDumper` — this page's subject) and
  [`ein-render/dump/state.rs`](../../../ein.rs/crates/ein-render/src/dump/state.rs)
  (`MonotonicDumper`, `ProgressDumper`).
- Goldens: [`golden_dump.rs`](../../../ein.rs/crates/ein-render/tests/golden_dump.rs)
  (the timeline's per-entering `firings` count and every file under
  `enterings/`) and `dump_parity.rs` (the rest of the tree, byte for byte).
- CLI: [`ein solve`](../../../ein.rs/crates/ein-cli/src/solve.rs)
  (`--exhaustive`; `--trace` for the reductio markdown; `--dump-states DIR`
  for the *lighter* `MonotonicDumper` tree). The on-disk `LatticeDumper` tree
  is reachable from Rust only (see *How to run it*).
- Tests: [`lattice_semantics.rs`](../../../ein.rs/crates/ein-infer/tests/lattice_semantics.rs).
- Algorithm spec: [`algorithm_layer_n.md`](algorithm_layer_n.md).
