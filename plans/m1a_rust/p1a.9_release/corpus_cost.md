# The corpus, priced — what a sweep costs on the engine that ships

**Produced by:** [S1a.9.0](s1a.9.0_slow_corpus.md), 2026-08-22
**Build:** `master` @ `b5661d8`, `cargo build --release -p ein-cli` — the
shipping default, `snmalloc` linked
**Machine:** Intel i9-14900HX, Linux 7.1.8, `powersave` governor, turbo on,
pinned to **cpu4** (a P-core sibling) by
[`utils/bench_env.sh`](../../../utils/bench_env.sh)
**Instrument:** [`utils/corpus_cost.py`](../../../utils/corpus_cost.py) — cold
processes, mean ± sd over *n* runs after a warm-up (discarded unless the cell
turns out to cost more than 2 s), `{out}` and `--json-summary` exactly as the
sweep issues them

```sh
utils/bench_env.sh python3 utils/corpus_cost.py --runs 5 --budget 20 --json cost.json
```

The process floor on this machine is **1.3 ms** — `render constraints`, which
parses the file and never loads a KB — so a 3.5 ms cell is about 2 ms of
engine and the corpus's median cell is mostly `execve`.

---

## Why there is a file here at all

`corpus/corpus.toml` marked **17 of 128** entries `slow = true`, and six
carried a note dropping `solve` / `solve -e` / `render lattice` for a reason
given verbatim as *"outlives a 150 s budget under CPython, and a run nobody
can finish is not coverage."*

The flag and the exclusions were priced in the T1a.0.1.1 probe of 2026-08-17,
against an engine that left the tree at
[S1a.10.5](../p1a.10_single_implementation/s1a.10.5_removal.md), on a budget
set for it. Nothing had re-taken them since, and nothing could: there was no
threshold to check the flag against and no instrument that would have.

This file is what re-takes them. Its tables are the first ein.rs-native
pricing of the corpus, and they outlive the decisions made on them: whatever
happens to the flag next, these are the numbers it moved from.

## 1. The seventeen, measured

`solve`, `solve -e`, `render lattice` and the entry's **total** — every
declared run summed, which is what the entry costs a sweep. `∅` is a run the
manifest does not declare; § 3 prices those separately.

| entry | `solve` | `solve -e` | `render lattice` | total | was | now |
|---|---:|---:|---:|---:|:-:|:-:|
| `branching/06_lookahead_on` | 8 ms | 245 ms | 114 ms | **377 ms** | slow | — |
| `branching/07_lookahead_off` | 925 ms | 916 ms | 265 ms | **2.12 s** | slow | **slow** |
| `features/01_not_and_absent` | 1.97 s | 1.97 s | 115 ms | **4.06 s** | slow | **slow** |
| `features/02_star_in_identifiers` | 214 ms | 265 ms | 32 ms | **522 ms** | slow | — ¹ |
| `features/04_open` | ∅ | ∅ | **10.20 s** | **10.21 s** | slow | **slow** |
| `features/05_stdlib_domain_elim` | ∅ | ∅ | 114 ms | **124 ms** | slow | — |
| `saturation/square-bwd/floors` | 315 ms | 315 ms | 32 ms | **676 ms** | slow | — ¹ |
| `saturation/square-bwd/houses` | 315 ms | 315 ms | 16 ms | **655 ms** | slow | — ¹ |
| `saturation/square-bwd/meetings` | 315 ms | 315 ms | 16 ms | **657 ms** | slow | — ¹ |
| `saturation/square-fwd/floors` | 315 ms | 315 ms | 16 ms | **661 ms** | slow | — ¹ |
| `saturation/square-fwd/houses` | 315 ms | 315 ms | 32 ms | **677 ms** | slow | — ¹ |
| `saturation/square-fwd/meetings` | 315 ms | 315 ms | 32 ms | **676 ms** | slow | — ¹ |
| `saturation/square-unique/corner-house` | ∅ | ∅ | **124.80 s** | **124.82 s** | slow | — ¹ |
| `saturation/square-unique/cul-de-sac` | ∅ | ∅ | **94.20 s** | **94.21 s** | slow | — ¹ |
| `zebra` | 16 ms | 64 ms | 64 ms | **166 ms** | slow | — |
| `zebra2-minus-15` | 32 ms | ∅ | ∅ | **54 ms** | slow | — |
| `zebra2` | 16 ms | 32 ms | 32 ms | **260 ms** ² | slow | — |
| `saturation/square-unique/terminus` ³ | ∅ | ∅ | ∅ | **10 ms** | — | — |

¹ the entry also **loses runs** — § 2B and § 3 say which and why. The `now`
column is the flag after that.
² sixteen cells — the manifest's widest column, which four other entries also
have — and still a quarter of a second.
³ not flagged, but it carried the same CPython note and is re-priced with its
two siblings.

**Spread.** Above 10 ms the measurement is quiet: of the 655 repeated cells,
only three that cost more than 10 ms read over 3 % relative sd —
`branching/06 :: solve -e` (11.2 % of 245 ms, and the largest real variance in
the corpus), `ein-bugs/zebra2-bad :: solve` (21.6 % of 14 ms) and
`features/04_open :: render lattice` (3.1 %, n = 2). Below 10 ms it is loud and
uninformative: 114 cells read over 3 %, every one of them a 1.7–2.6 ms cell
whose deviation is ±1 ms of process creation. Cells at n ≤ 2 are the ones the
20 s per-cell budget stopped repeating.

## 2. Three groups, and only one of them was what the flag claimed

**Twelve of the seventeen were never slow** by the 1 s rule, whatever is done
with their `runs` columns; two more (`square-unique/corner-house` and
`cul-de-sac`) stop being slow once a run that asks their fixture nothing is
dropped; three remain. The stage's own A/B/C grouping is finer than that, and
it is the one worth keeping, because the three groups are wrong in three
different ways.

### A — four were never slow, and one of them is the flagship

`zebra2.ein` solves in **16 ms** and its whole sixteen-cell matrix in 260 ms.
`zebra` is 166 ms, `zebra2-minus-15` 54 ms on the runs it declares,
`branching/06_lookahead_on` 377 ms. A flag that is false on the corpus's
flagship is a flag no reader trusts anywhere else, and all four lose it. Every
run they declare is kept: on these entries `solve -e` is the point of the
fixture.

### B — ten were slow because the fast path buys nothing

Ten entries cost the **same** exhaustively as they do at `stop_after = 1`:

| entry | `solve` | `solve -e` | ratio |
|---|---:|---:|---:|
| `branching/07_lookahead_off` | 925 ms | 916 ms | 0.99× |
| `features/01_not_and_absent` | 1.97 s | 1.97 s | 1.00× |
| `features/02_star_in_identifiers` | 214 ms | 265 ms | 1.24× |
| `saturation/square-{bwd,fwd}/*` (6) | 315 ms | 315 ms | 1.00× |
| `features/05_stdlib_domain_elim` ⁴ | 2.97 s | 2.92 s | 0.98× |

⁴ measured in § 3 — the manifest does not declare either run.

They return the same verdict, and it is the reason the two paths cost the
same: **no node is ever complete, so `stop_after = 1` has nothing to stop
early at.** The search runs out of commitment-set depth, reports
`Contradiction k=0` with `exhausted=False` and `layers_explored == -m`, and the
cost of these entries is the depth it walked before saying so.

That finding has two halves and they are different in kind.

**The manifest's half is this stage's.** `02_star_in_identifiers` is a demo of
`*` in an identifier tail: two rules, four `is-a` edges, a transitive closure.
There is no hypothesis structure in it to search, and its `runs` column
nevertheless asked for `solve` **and** `solve -e`, so the engine enumerated a
wide empty space to the cap and spent 265 ms establishing nothing, on a file
whose meaningful run — `saturate --dump`, which lists the ten `is-a*` edges the
header predicts — costs 3.7 ms. Six of the ten are per-rule saturation demos
in exactly that position ([`examples/saturation/README.md`](../../../examples/saturation/README.md):
*"the smallest IR file that exercises **one** of the seven M1-core rules"*).
Those seven entries lose `solve` and `solve -e`; the note on each says so.

**The engine's half is not.** `Contradiction` is `verdict.rs` reading `k == 0`,
and `exhausted = !truncated` is set two functions away; the word does not
consult it. The same file under `-T` or `-E` refuses to answer at all —
`** aborted: max-time (0.05s) exceeded **`, exit 2, and `Answer::Aborted`
through the library, which `verdict.rs` glosses as *"unexplored, not proven
unsatisfiable"*. Three budgets, two vocabularies, one situation. That is
[Q-M1d.6](../../m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false),
written into M1d with the `-m` sweep as its reproducer, and **this stage
changed no verdict word**: doing so would move checked-in fixtures across the
corpus and it is a semantic decision, not a release chore.

### C — the ones that cannot finish, and CPython was never why

Eight cells across four entries do not end in a verdict at the default `-m 5`.
They do not end in a timeout either. They end in the **OOM killer**:

```text
Out of memory: Killed process 1474231 (ein) total-vm:339880116kB,
anon-rss:14271536kB, … oom_score_adj:200
```

That is `ein solve examples/features/04_open.ein`, 78 s in, having reached
**14.3 GB** of anonymous memory. The exclusion note blamed a 150 s CPython
budget; the truth is that an unbounded hypothesis space is unbounded in
**memory** first, and it would have been true of any engine. The note's reason
was wrong even where its conclusion was right.

## 3. The runs the manifest does not declare, priced

Ceiling **300 s** — `EIN_CORPUS_TIMEOUT`'s default, i.e. the point at which the
sweep would kill the cell and record `-2`. Thirteen cells, across the six
entries whose notes dropped a run.

| entry :: run | result |
|---|---|
| `features/04_open :: solve` | **OOM-killed**, 78 s — 14.3 GB |
| `features/04_open :: solve -e` | **OOM-killed**, 103 s |
| `features/05_stdlib_domain_elim :: solve` | **2.97 s** ± 0.2 ms (n=3) |
| `features/05_stdlib_domain_elim :: solve -e` | **2.92 s** ± 0.6 ms (n=3) |
| `square-unique/corner-house :: solve` | **OOM-killed**, 63 s (n=2) |
| `square-unique/corner-house :: solve -e` | **OOM-killed**, 57 s (n=2) |
| `square-unique/cul-de-sac :: solve` | **OOM-killed**, 66 s |
| `square-unique/cul-de-sac :: solve -e` | **OOM-killed**, 66 s |
| `square-unique/terminus :: solve` | **OOM-killed**, 152 s |
| `square-unique/terminus :: solve -e` | **OOM-killed**, 151 s |
| `square-unique/terminus :: render lattice` | killed at the **300 s** ceiling |
| `zebra2-minus-15 :: solve -e` | killed at the **300 s** ceiling ⁴ |
| `zebra2-minus-15 :: render lattice` | **27.82 s** ± 29 ms (n=3) |

⁴ [P1d.1](../../m1d_satisfiability/p1d.1_exhaustive_search/README.md) has the
same cell **killed at 30 minutes**; the ceiling here is the sweep's, not the
run's.

**Three of the thirteen finish, and all three keep their exclusion.**
`features/05_stdlib_domain_elim` is a demo of the stdlib's elimination rules,
and three seconds of blind enumeration is not three seconds of coverage;
`zebra2-minus-15 :: render lattice` finishes because `-m 3` is where all 32 of
its models live (P1d.1) — and writes 11.6 MB of DOT to say so. **Affordable is
not the test.** Whether the run asks the fixture's question is, and
`corpus/README.md` § Dropped runs now says so in as many words.

The `square-unique` family prices something else on the way past. Its three
demos are the same rule over three tiny streets — 13, 17 and 20 forms — and
their blind searches are **63 s, 66 s and 152 s** to the OOM killer and
**125 s, 94 s and > 300 s** to a lattice, in an order that does not follow
either the file size or the object count. What a blind enumeration costs is a
property of the enumeration, not of the demo.

## 4. What the sweep costs, before and after

| | cells | engine time | `cargo test` wall clock |
|---|---:|---:|---:|
| default selection, before | 542 | 1.70 s | — |
| default selection, after | 622 | 2.97 s | **3.6 s** |
| `EIN_CORPUS_SLOW=1`, before | 660 | 242.6 s | **307.2 s**, green |
| `EIN_CORPUS_SLOW=1`, after | 641 | 19.4 s | **21.1 s**, green |
| the slow tier alone, before | 118 | 240.9 s | — |
| the slow tier alone, after | 19 | 16.4 s | — |

The two columns are two instruments and they do not measure the same binary.
**Engine time** is `corpus_cost.py` over the `release` build, summed per cell.
**Wall clock** is what `cargo test` reports, over the `dev` build
(`[profile.dev] opt-level = 1`) and including the runner's own polling — 1.27×
the engine time before, 1.09× after. The "before" figure was taken by stashing
this stage's four workspace files and re-running the sweep, so it is the same
machine on the same afternoon. That gap between profiles is one more reason
`corpus_cli`'s live check allows 4× rather than something tight.

Two numbers explain almost all of it. `square-unique/corner-house` and
`cul-de-sac` spent **219 s — 90 % of the whole sweep — rendering a
commitment lattice** for two demos of one rule firing. `ein render lattice` is
an exhaustive solve with `store_lattice` at `-m 3` (`render.rs::cmd_lattice`),
so on a fixture that closes no domain it is the blind enumerator with a DOT
writer attached. Their sibling `terminus` has run without that cell since it
was written, and no one has recorded a gap.

What those cells write is the other half of the argument:

| file | `render lattice` | the DOT it emits |
|---|---:|---:|
| `zebra2.ein` | 32 ms | **26.8 KB**, 210 lines |
| `zebra2-minus-15` ⁵ | 27.8 s | 11.6 MB, 88 170 lines |
| `features/04_open` | 10.2 s | 18.3 MB, 170 646 lines |
| `square-unique/cul-de-sac` | 94.2 s | **49.2 MB**, 480 934 lines |
| `square-unique/corner-house` | 124.8 s | **58.6 MB**, 533 838 lines |
| `square-unique/terminus` ⁵ | > 300 s | — |

⁵ not declared; measured in § 3.

The flagship's lattice is **26.8 KB and a reader opens it**. `cul-de-sac`'s is
1 800× that, and no one has ever looked at it — it is not a view, it is the
enumerator's shadow. So the rule this stage applies to `render lattice` is the
same one it applies to `solve`: keep it where the *fixture* is about the
search. **The corpus keeps exactly one entry that pays for an unbounded
lattice, and it is the one whose subject that is** — `features/04_open`, the
demo of an open domain, which
[Q-M1d.3](../../m1d_satisfiability/open_questions.md#q-m1d3--what-closes-a-domain)
already cites for exactly that. The `square-unique` demos are about one rule
firing once, and `saturate` shows it in 3.5 ms.

Sixteen cells left the corpus and eighty joined the default selection: it now
sweeps **fourteen entries that used to be nightly's**, `zebra2` and the six
`square-*` demos among them, while the nightly tier is 19 s rather than four
minutes.

## 5. The 10.2 s line

**The corpus's slowest surviving cell is `features/04_open :: render lattice`
at 10.2 s** — **640×** `zebra2`'s entire `solve`, on the same engine, and 5×
the next slowest cell in the corpus (`features/01 :: solve`, 1.97 s). It is a
demo of the `open` macro, and the cost is a property of an open domain rather
than a regression: nothing bounds the
hypothesis space, so `solve` measures the blind enumerator and `render lattice`
draws it.

The three numbers next to each other are the milestone's shape in one line:

| | |
|---|---:|
| `zebra2 :: solve` — the flagship | **16 ms** |
| `04_open :: render lattice` — the corpus's worst, at `-m 3` | **10.2 s** |
| `04_open :: solve` — the same puzzle at `-m 5` | **no answer**, 14.3 GB |

The depth cap is the only thing between the second row and the third.
[M1d](../../m1d_satisfiability/README.md) owns that sentence —
[Q-M1d.3](../../m1d_satisfiability/open_questions.md#q-m1d3--what-closes-a-domain)
already names this fixture as the corpus entry whose whole point is that an
open domain makes the search unbounded, and now it names the price too.

## 6. What holds these numbers up

The measurement is not the mechanism; a table in a plan document rots exactly
the way the flag did. Three things carry it forward:

- **`corpus/corpus.toml` carries `cost_ms`** — an entry's total, in
  milliseconds — on every `slow` entry and on every entry within 4× of the
  threshold. Nothing else: a number nobody checks is what went stale here.
- **`ein-corpus::manifest::slow_matches_the_recorded_cost`** holds the flag to
  `SLOW_MS` and `cost_ms`, exactly, without running the engine.
- **`corpus_cli::the_slow_flag_still_describes_the_sweep`** holds `cost_ms` to
  the wall clock of the sweep it has just run, at a 4× tolerance — wide enough
  for a slow machine, narrow enough that the 165× drift this stage found could
  never have hidden in it.

`utils/corpus_cost.py --check` is the same claim outside the gate, with an
exit code.

## Reproducing this file

```sh
cargo build --release -p ein-cli
utils/bench_env.sh python3 utils/corpus_cost.py --runs 5 --budget 20 --json cost.json

# § 3 — the runs the manifest does not declare
utils/bench_env.sh python3 utils/corpus_cost.py -k square-unique \
    --also 'solve,solve -e' -r solve -r 'solve -e' --runs 3 --timeout 300

# § 4 — the sweep itself
cargo test --manifest-path ein.rs/Cargo.toml -p ein-cli --test corpus_cli
EIN_CORPUS_SLOW=1 cargo test --manifest-path ein.rs/Cargo.toml -p ein-cli --test corpus_cli
```

An OOM-killed cell reads `exit -9` in the table and is a **machine-dependent**
reading: this box has 31 GB, of which ~9 GB were free. What is not machine
dependent is that the run has no fixed point to reach — the planning probe on
another machine recorded `04_open :: solve` at 136 s *without* the kill, and
that is the same finding with more RAM under it.
