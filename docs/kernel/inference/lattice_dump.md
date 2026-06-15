# Lattice dump — per-hypothesis emission tracking

> **Purpose.** When you need to know *which hypotheses the engine
> tested at each layer* and *what every one of them derived —
> survivors and casualties alike* — run a lattice entry with a
> [`LatticeDumper`](../../../ein.py/src/ein/inference/monotonic/state_dump.py)
> attached. The on-disk dump is the audit trail for debugging
> **problem statements** (is the puzzle even consistent? which
> committed pair kills it?) and **rules** (did the rule I expected
> fire under this commitment? did it fire when it shouldn't?).

This page covers the two exhaustive lattice entries —
[`gaps_solve`](../../../ein.py/src/ein/inference/monotonic/solver.py)
and `contradictions_solve` — which test *every* commitment set
layer-by-layer and so produce a complete per-hypothesis record.
The solution-mode [`monotonic_solve`](README.md#set-indexed-search--monotonic-engine-p15b-s15b0-10)
early-terminates on the first solution and uses the lighter
[`MonotonicDumper`](../../../ein.py/src/ein/inference/monotonic/state_dump.py)
(timeline + per-layer root snapshots only — no per-hypothesis
folders, since most hypotheses are never reached).

---

## How to run it

### From the CLI

[`demo/bench_lattice.py`](../../../ein.py/demo/bench_lattice.py)
dispatches both lattice entries. Pass `--dump-states DIR` to attach
a `LatticeDumper`; add `--store-lattice` to also materialise the
per-SetNode `kb_index/`:

```sh
cd ein.py
# Every dead commitment + its refutation, dumped:
python3 demo/bench_lattice.py ../examples/branching/04_two_levels.ein \
    --contradictions --store-lattice --dump-states ./dump
# Every satisfying commitment, dumped:
python3 demo/bench_lattice.py ../examples/branching/04_two_levels.ein \
    --gaps --dump-states ./dump
tree ./dump
```

Full zebra2 is too slow on CPython for an exhaustive lattice sweep —
use [`./bench_solve_pypy.sh`](../../../ein.py/) / a PyPy interpreter,
and bound the sweep with `--max-set-size N`, `--max-time S`, or
`--max-enterings K` so the dump stays a manageable size.

### Programmatically

```python
from pathlib import Path
from ein.inference.monotonic import LatticeDumper, contradictions_solve
from ein.ir import parse
from ein.kb.store import KnowledgeBase

kb = KnowledgeBase.from_ir(parse(Path("examples/branching/04_two_levels.ein").read_text()))
dumper = LatticeDumper(out_dir=Path("./dump"))
verdict, stats = contradictions_solve(kb, max_set_size=3,
                                      store_lattice=True, dumper=dumper)
```

`out_dir=None` makes every hook a no-op (the call sites stay
uniform, nothing hits disk) — useful for subclasses that stream the
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
│       └── post.ein             ← root.kb at the end of layer NN (after merges)
├── enterings/                   ← ★ per-hypothesis emission tracking
│   └── layer_NN/
│       └── <C-slug>/            ← one commitment tested at layer NN
│           ├── commitment.json          ← the committed FactId list
│           ├── outcome.txt              ← alive | dead-pre | dead-post | solution
│           ├── firings.jsonl            ← every rule firing in this fork  (non dead-pre)
│           ├── unconditional_facts.jsonl← facts merged back to root       (non dead-pre)
│           ├── kb.ein                   ← the fork's full saturated KB    (solution only)
│           ├── unsat_core.jsonl         ← the contradiction witnesses     (dead-* only)
│           └── learned_clause.json      ← the CDCL nogood emitted          (dead-* only)
├── kb_index/                    ← only under store_lattice=True
│   └── layer_NN/
│       └── kb_<i>/              ← ordered ids (kb_0 … kb_n), sorted by state_hash within layer
│           ├── state_hash.txt           ← 16-hex of the post-saturation state hash
│           ├── canonical_set.json       ← the SetNode's canonical commitment
│           ├── labels.json              ← every commitment that mapped to this node
│           └── verdict.txt              ← alive | dead | solution
├── proof_summary.json           ← top-level index (solutions, deads, kb_index, stats)
└── summary.json                 ← cumulative stats + verdict kind + wall time
```

### `<C-slug>` — the commitment slug

A commitment is a *set* of [`FactId`](../../../ein.py/src/ein/inference/commitment.py)s
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
  this commitment (the **positive** emissions);
  `unconditional_facts.jsonl` is the subset that merged back into the
  shared root (facts true regardless of the hypothesis).
- **`solution`** — same as alive plus the goal was satisfied;
  `kb.ein` is the fork's full saturated KB so you can read the
  solved state.
- **`dead-post`** — the fork saturated *to a contradiction*.
  `firings.jsonl` still records what fired on the way down (so you
  can see the derivation that led to the clash); `unsat_core.jsonl`
  is the minimal witness set, and `learned_clause.json` is the
  `frozenset(C)` nogood emitted so no superset is re-entered
  ([CDCL nogoods](README.md#cdcl-nogoods-s15b6)).
- **`dead-pre`** — the commitment was rejected *before* saturation
  (apriori superset of a known nogood, or a `_negated_facts` hit), so
  there are no firings to record — only `unsat_core.jsonl` +
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
> positive firings (`firings.jsonl`) and the derived negatives
> (`(not …)` facts appear in `unconditional_facts.jsonl` /
> `unsat_core.jsonl`, since the [d=0 negative-completion
> rules](README.md#d0-negative-completion-s15a19) emit them as
> ordinary REASONING-layer facts).

---

## `kb_index/` — ordered ids, grouped by layer

Under `store_lattice=True` the engine keeps a per-SetNode index
(state-hash dedup; under `contradictions_solve` it drives the merge
where distinct dead commitments with identical post-saturation KBs
collapse). The dump folders use **per-layer ordered ids** —
`kb_index/layer_NN/kb_<i>/` — rather than hash-named folders: within
each layer the nodes are sorted by `state_hash` (deterministic) and
numbered `kb_0 … kb_n`. The raw hash is still available in
`state_hash.txt` and `proof_summary.json`'s `state_hash_hex` for
correlating a node across runs; `labels.json` lists every commitment
that mapped onto the node (>1 means a state-hash merge happened).

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

- **"Is my problem statement consistent?"** — run `--contradictions`.
  If `00_root_initial.ein` already contains `(false)`, the puzzle is
  inconsistent before any hypothesis (Phase-1 contradiction). Else,
  scan `enterings/layer_01/*/outcome.txt`: a singleton that dies
  `dead-post` means that one fact is incompatible with the givens.
- **"Why did commitment {A,B} get pruned?"** — find its
  `learned_clause.json`; the clause is the minimal set whose
  conjunction is unsat. Its `unsat_core.jsonl` names the facts that
  clashed — chase their provenance back through `firings.jsonl`.
- **"Did rule R fire where I expected?"** — `grep '"rule": "R"'`
  across `enterings/**/firings.jsonl`. Empty under a commitment where
  you expected it means a `:match` premise (often an
  [`(absent …)` NAF guard](README.md#naf-semantics--fire-time-re-evaluation-s15a1))
  didn't hold in that fork.
- **"Two commitments should reach the same state but don't"** — under
  `--store-lattice`, compare their `kb_index/layer_NN/kb_<i>/state_hash.txt`;
  different hashes with the same intended meaning point at a
  non-confluent rule set (see the `--lattice-sanity-check` flag).

---

## Cross-links

- Engine overview: [README § Set-indexed search](README.md#set-indexed-search--monotonic-engine-p15b-s15b0-10).
- Implementation: [`monotonic/state_dump.py`](../../../ein.py/src/ein/inference/monotonic/state_dump.py)
  (`LatticeDumper`, `MonotonicDumper`).
- CLI: [`demo/bench_lattice.py`](../../../ein.py/demo/bench_lattice.py).
- Tests: [`tests/inference/lattice/test_lattice_dumper.py`](../../../ein.py/tests/inference/lattice/test_lattice_dumper.py).
- Algorithm spec: [`algorithm_layer_n.md`](../../../plans/m1_core_graph_reasoning/p1.5b_lattice_search/algorithm_layer_n.md).
