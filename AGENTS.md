# AGENTS.md

Guidance for AI coding agents working in this repo.

## What this project is

Ein is a graph-based reasoner for Zebra-style logic puzzles. The
2021 prototype is being modernised in light of neuro-symbolic /
constrained-reasoning research.

## Where things live

- **`docs/kernel/`** — **canonical M1 kernel documentation**: graph
  semantics (`ir/01-ein-graph/`), the data model (`ir/02-data-model/`),
  surface S-expression language (`ir/03-ein-lang/`, mostly what used
  to be `docs/ir.md`), inference engine (`inference/`). Start here for any
  "what does Ein reason about / how" question. See
  [`docs/kernel/README.md`](docs/kernel/README.md) for orientation.
  **Since M1c S1c.1.2 a `(query …)` can state its own answer** — `:expect
  (model …)` / `(or (model …) …)` / `(false)`, where *naming a relation closes
  it*, and `ein solve` exits 1 when the claim is false
  ([`ir/03-ein-lang/01_grammar.md` § Query](docs/kernel/ir/03-ein-lang/01_grammar.md#query)).
  A file may carry several `(query …)` blocks and each is run; the last one no
  longer silently wins, and an unrecognised query keyword is now a load error.
  **S1c.1.3 added the runner**: `ein test <file|dir>…` is the fourth
  subcommand, and it turns a directory of expectations into a status code.
  It **exhausts** (an expectation is a claim about the exhausted answer, so
  there is no `-n`), it **never solves a query that carries no `:expect`**, and
  its exit **1 means a claim is false** — which is why a load error there takes
  2, where `solve` gives it 1.
  Since M1a S1a.10.6 two pages are new or renamed and worth knowing:
  [`defined_behaviour.md`](docs/kernel/defined_behaviour.md) — the thirteen
  diagnostics, orderings and error strings whose only statement used to be a
  Python source file, now normative; and `inference/implementation.md` +
  `ir/02-data-model/03_implementation.md`, the module maps that were
  `python_impl.md`. **This tree is now the only statement of intent that is
  not also the implementation**, so it is load-bearing: a claim here is
  checked by `cargo test --workspace` and by nothing else.
- **`docs/api/`** — **how to drive Ein as a library.** Since M1a S1a.9.4 its
  subject is [`rust.md`](docs/api/rust.md): the **crates** — `ein-ir` to load,
  `ein-infer` to solve, `ein-render` to explain, `ein-einb` to cache — which
  is what [M20](plans/m20_gui/README.md) binds against and what nothing
  documented before. **Its worked example is a test.** The page's `rust` block
  is the marked region of
  [`ein-cli/tests/embedding.rs`](ein.rs/crates/ein-cli/tests/embedding.rs) and
  a test in that file diffs the two, so `cargo test --workspace` keeps the
  page true — *edit the test, run it, paste; never edit the block by hand*.
  The other five pages (`ein`/`ir`/`kb`/`inference`/`trace`, 1 019 lines) are
  the **Python** embedding contract, filed as **history** with a 🏛 banner:
  kept whole because a deferral is cheap to reverse only while the
  specification survives it
  ([Q-M1a.23](docs/history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
  holds the three trip-wires). Do not "fix" them to match ein.rs — a page
  rewritten to describe the current engine is neither history nor a
  specification. Distinct from `docs/kernel/` (the IR *language*) and from the
  engine internals.
- **`docs/install.md`** — the install page (M1a S1a.9.3): the two channels
  (a release binary, `cargo install --path`), what `ein --version`'s five
  lines mean, how to check a binary's stdlib against the checkout's manifest,
  and `$EIN_STDLIB`. **`pip install` is not a channel.**
- **`docs/guide/`** — **the newcomer tutorial** ("Learn Ein by solving the
  Zebra puzzle"): objects/relations/facts → rules → the full solve, four
  chapters (P1.20 Theme K). User-facing; references `docs/kernel/` +
  `docs/api/`, never explains internals; complements
  `inference/zebra_walkthrough.md`.
- **`docs/history/`** — **shipped milestones, kept as record.** Its first and
  only entry is [`m1a_rust/`](docs/history/m1a_rust/README.md), the Rust port
  (2026-08-17 → 2026-08-23): one README carrying all eleven phases and 53
  stages, plus what is still *read* rather than merely intended — the eleven
  [`design/`](docs/history/m1a_rust/design/README.md) contracts the crates cite
  as their specification, six
  [`measurements/`](docs/history/m1a_rust/measurements/) documents whose
  CPython and PyPy columns nothing can re-take, the
  [divergence ledger](docs/history/m1a_rust/divergences.md), twenty-three
  [questions](docs/history/m1a_rust/open_questions.md) (two still open on
  purpose), the [oracle ledger](docs/history/m1a_rust/oracle_ledger.md) and the
  [suite dispositions](docs/history/m1a_rust/suite_dispositions.md).
  **`plans/m1a_rust/` is gone** — deleted 2026-08-23, 65 files and 13 950 lines
  of milestone, phase and stage documents, and it is in git history
  (`git log --diff-filter=D -- plans/m1a_rust`). The rule that put a document here rather than leaving it in
  git: it is still read, as a specification, as evidence, or as the reason
  something is the way it is.
- **`docs/lib/`** — catalogue of external tech relevant to the rewrite
  (LLM constrained generation, CSP/SAT/SMT, theorem proving, category
  theory, graphs & rewrite systems, reasoning benchmarks, …). 12
  thematic files + a knowledge graph (`knowledge-graph.dot` → SVGs
  and a Cytoscape.js page).
- **`plans/ideas/`** — the user's *own* ideas (10 numbered files +
  README; moved here from `docs/ideas/`); each preserves user quotes and
  open questions. Authoritative on intent.
- **`examples/`** — encoded Zebra puzzles (`zebra.ein` classic,
  `zebra2.ein` unified-is-a / `*-loc`, `zebra2-hints.ein` partial-state
  fixture) plus focused per-feature fixtures (`features/`, `branching/`,
  `saturation/`, `lattice/`, `domain_elim/`, `broken/`).
  [`examples/README.md`](examples/README.md) is a **catalog** — one line
  per file / sub-dir.
- **`docs/kernel/inference/zebra_walkthrough.md`** — the Wikipedia human
  Zebra walkthrough annotated as ein inference (NL↔ein rule↔branch-depth
  table, hypotheses with their contradictions and no-good clauses; **moved
  here from `examples/README.md`**). The **M1 target trace** for the engine
  and the **M2 target** for the NL ⇄ IR round-trip (NL problem → facts →
  ontology+rules → solution → NL explanation).
- **`stdlib/`** — the ein-lang standard library (`std.*`), the single source
  of truth. `MANIFEST.sha256` is what identifies a directory as the stdlib and
  what CI checks for drift; `ein-ir` embeds a copy with `include_dir!`, so the
  manifest is a build dependency and cannot go stale. Resolution: `$EIN_STDLIB`
  → the checkout → the embedded copy.
- **`corpus/`** — the **corpus**: `corpus.toml` (one entry per `.ein`, with
  the runs it is exercised under) plus `fuzz_findings/`. A file under
  `examples/` or `stdlib/` with no entry fails a completeness check. What
  runs it is `cargo test`, and [`corpus/README.md`](corpus/README.md) has the
  table of readers. It was `conformance/` until M1a S1a.10.3, and the
  `--events` protocol it also held is now
  [`docs/kernel/inference/events.md`](docs/kernel/inference/events.md).
  Since [S1a.9.0](docs/history/m1a_rust/README.md#s1a90--the-slow-corpus-re-priced)
  **`slow = true` is a measured claim** — an entry whose declared runs cost
  1 s or more together, recorded in `cost_ms` and checked in both directions
  ([corpus_cost.md](docs/history/m1a_rust/measurements/corpus_cost.md) is the
  measurement, `utils/corpus_cost.py` re-takes it). **Two** entries are slow,
  where seventeen were — three until M1a T1a.7.2.0 made
  `branching/07_lookahead_off` 2.8× cheaper and the re-take took its flag off,
  which is the mechanism working ([corpus_cost.md
  §7](docs/history/m1a_rust/measurements/corpus_cost.md#7-the-first-re-take--2026-08-22-and-it-moved-an-entry));
  `EIN_CORPUS_SLOW=1` is 20 s of `cargo test` rather than four minutes;
  and **a run is dropped from a `runs` column only when it does not ask the
  fixture's question**, never for costing too much.
- **`ein.rs/`** — the Rust port ([M1a](docs/history/m1a_rust/README.md)), a
  drop-in replacement for `ein`, and since
  [P1a.10](docs/history/m1a_rust/README.md#p1a10--one-implementation) the only
  implementation. Two of its eight crates are dev-only: `ein-corpus` (the
  manifest, the fixture helpers, the bench set) and `ein-parity` (the one
  implementation of what counts as a derivation's *narration* rather than
  its content — [design/01
  §5](docs/history/m1a_rust/design/01_parity_contract.md#5-legitimate-divergences-the-normalisation-list);
  `EIN_PARITY_STRICT=1` turns it off). The six that ship are `ein-core`
  (interning, `Value`/`FactId`, the layered COW KB, provenance), `ein-ir`
  (lex → parse → macros → imports → load), `ein-infer` (compile → match →
  saturate → the NAF boundary → the hypothesis loop), **`ein-einb`** (the
  `.einb` binary KB container — [P1a.8](docs/history/m1a_rust/README.md#p1a8--binary-kb-container),
  and the **only crate that is not `#![forbid(unsafe_code)]`**: its `cast.rs`
  is the one audited module design/12 §2 permits `unsafe` in, which is why it
  is a crate at all), `ein-render` (DOT views, the markdown trace, the
  state/lattice dumps, the JSON summary) and `ein-cli`. They stack linearly up
  to `ein-infer` and fork there — `ein-einb` and `ein-render` are siblings
  above it, and `ein-cli` depends on both. `ein-cli` fronts **four**
  subcommands since M1c S1c.1.3 — `ein {render,saturate,solve,test}`, plus
  `ein kb` under the `einb` feature.

  **`.einb` is a private cache format, never an interchange one.** `ein kb save
  <file.ein> <out.einb>` writes one and every command that takes a `.ein` path
  takes a `.einb` too, dispatching on the magic bytes rather than the
  extension; `ein solve x.einb` is byte-identical to `ein solve x.ein` apart
  from the path it echoes. Anything crossing a tool boundary is still `.ein`
  text or the event protocol's JSON.

  **`ein.py/` was the oracle** until S1a.10.2 banked what only it proved, and
  was deleted at S1a.10.5; what remains of that argument is the ledger, the
  goldens under `tests/golden/from_ein_py/` — the last independent provenance
  in the repo — the divergence list, and
  [`docs/kernel/defined_behaviour.md`](docs/kernel/defined_behaviour.md), which
  states what "whatever ein.py did" used to define.
- **`utils/`** — **nineteen scripts, all of them driving `ein.rs`** since M1a
  [S1a.10.4](docs/history/m1a_rust/README.md#s1a104--utils-re-aimed-at-one-engine),
  which deleted the eleven that compared two engines or measured the Python
  one, plus `corpus_cost.py` from
  [S1a.9.0](docs/history/m1a_rust/README.md#s1a90--the-slow-corpus-re-priced).
  Every script that runs the engine names the binary — **`$EIN_BIN`** or
  `--bin` — defaulting to `ein.rs/target/release/ein`, except the three that
  want a build of their own (`fork_delta_verify.py` → `target-fd`,
  `spec_audit.py` → `target-sa`, `profile_ein_rs.py` → `--profile profiling`,
  which it builds). **None takes an `--impl`**: a flag with one value invites
  a reader to look for the operand that is gone.
  [`utils/README.md`](utils/README.md) is the catalogue — one line per script
  in three groups (*renderers*, *checks*, **the M1a measurement set**), plus
  the census of the eleven that went and what answers each one's question now.
  Two things worth knowing without opening it: run every measurement through
  **`bench_env.sh`**, which prints the machine state the numbers were taken
  under; and in
  [`baseline.md`](docs/history/m1a_rust/measurements/baseline.md) /
  [`scaling.md`](docs/history/m1a_rust/measurements/scaling.md) **the CPython
  and PyPy columns are frozen constants**, because the instruments that
  produced them left with the engine they measured. The nineteenth,
  **`stdlib_census.py`**, is [M1c](plans/m1c_external_validation/README.md)'s
  and the first check aimed at the *stdlib* rather than the engine: 73 rules
  parsed out of `stdlib/*.ein`, then 128 corpus entries × 400 inference runs
  under `--events`, `fire` counted by rule. Its answer —
  [**38 of 73 rules never fire**](plans/m1c_external_validation/p1c.1_stdlib_conformance/stdlib_census.md),
  and `examples/zebra.ein` is the sole activator of 20 more — is what M1c
  exists to close.
- **`build.sh`** — **everything this repo builds, in one command**: the Rust
  workspace (`--release` by default, into `ein.rs/target/`) and then the three
  C baselines in `c/` (into the gitignored `build/`). `--debug`,
  `--no-snmalloc` (the system allocator, for a machine without `cmake` and a
  C++ compiler), `--all-targets` (tests, benches and the measurement
  examples), `--engine` / `--c` for one of the two. It builds and does not
  run: `./run_tests.sh` is the gate.
- **`c/`** — **three plain-C Zebra baselines**, solving the puzzle
  `examples/zebra.ein` encodes with the same value names and the same answer,
  and with exactly one thing varying: **how much the search is told about the
  constraints**. [`c/README.md`](c/README.md) is the catalogue and carries the
  argument; the table is

  | | what the search knows | assignments | wall |
  |---|---|---:|---:|
  | `zebra_levels.c` | every clue, and the level at which each becomes testable | **6 840** | 0.003 s |
  | `zebra_oracles.c` | fourteen opaque yes/no functions, in the puzzle's order | 25 092 302 520 | 158 s |
  | `blackbox.c` + `zebra_module.c` | a grid size and one function pointer | 25 092 302 520 | 388 s |

  **3 668 465×**, and the difference is not an algorithm — it is one integer
  per clue, the level at which every attribute it names is bound. The third
  pair is two translation units on purpose: "the search knows nothing" is then
  checkable rather than stylistic, since `blackbox.c`'s object file holds no
  symbol from the puzzle beyond `PROBLEM`. § Circular dependencies between
  levels answers the question the level tag invites: the puzzle's constraint
  graph is K₅ minus two edges — four independent cycles — and the scheme does
  not notice, because a level is a `max` over an order chosen before any clue
  is read and because tests have no data flow between them to be circular in.
  What the cycles cost is the *schedule*: on a tree-shaped graph a DFS order
  would be optimal by construction, and it is only because of them that the
  level order had to be swept rather than derived.

  Nothing here is wired into anything — the gate does not run them and no
  crate depends on them. They earn their place by being what `ein solve
  examples/zebra.ein` is *not*, and `c/README.md` § What none of them do is
  where that is spelled out: no propagation, no domains, nothing learned from
  a dead subtree, and every condition compiled code that only answers this one
  puzzle.
- **`nlp/`, `smt/`** — scratch areas, 56 KB, wired into nothing. `smt/` holds
  three hand-written `.smt` encodings of the Zebra puzzle and 4-queens, which
  [M10](plans/m10_external_benchmarks/README.md)
  counts as part of its benchmark corpus; `nlp/` holds two throwaway
  dependency-parsing scripts that
  [M2 S2.6.4](plans/m2_nl_to_ir/p2.6_ablations/s2.6.4_representation_ablations.md)
  starts from — the link-grammar A/B, one arm of M2's representation
  ablation since the 2026-08-23 reshape. **The two submodules they used to carry** (`opencog/link-grammar`,
  `CVC4/CVC4`) were deinitialised at M1a S1a.10.5 — never checked out here,
  and a cost on every recursive clone. Each README has the one
  `git submodule add` that restores it.

## Running the gate

`./build.sh` builds everything first (see above); this runs it.

```sh
cargo test --manifest-path ein.rs/Cargo.toml --workspace     # the whole gate
EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test corpus_cli  # + the 2 slow entries
EIN_ID_SEEDS=8    cargo test … -p ein-render --test id_order_invariance
EIN_JOBS_SWEEP=2,4,8,16 cargo test … -p ein-render --test jobs_invariance
EIN_BLESS=1       cargo test … --workspace                   # re-bank the goldens
```

Everything runs one engine. `cargo test --workspace` is the gate — the corpus
sweep through the CLI, the shape digests, the goldens, the manifest's own
invariants — and it needs **Graphviz** on `PATH`, because
`dot_wellformed.rs` is the only authority the DOT views have on being
well-formed and it fails rather than skips without it.

The `ein` binary links `snmalloc` by default since S1a.6.2 (worth 8–16 % of a
solve), so the build needs **`cmake` and a C++ compiler**;
`cargo build --release -p ein-cli --no-default-features` builds against the
system allocator and needs neither.

**The parity harness is gone** (M1a S1a.10.3). `ein-conformance run --impl-a …
--impl-b … --tier T0…T3` ran two implementations over the corpus and diffed
them; there is no second operand. What each tier proved and who owns it now is
[the oracle ledger](docs/history/m1a_rust/oracle_ledger.md).

## Regenerating the knowledge graph

When `docs/lib/knowledge-graph.dot` changes, re-render both views:

```sh
utils/render_knowledge_graph.sh svg all     # 4 SVGs (dot/fdp/sfdp/osage)
python3 utils/render_knowledge_graph_cy.py  # elements.js + style.js + index.html
```

The Cytoscape view's `index.html` is a single self-contained file that
loads Cytoscape + fcose from unpkg CDN.

## Working priorities

The biggest unrealised idea is `plans/ideas/01-self-modifying-constraint-language.md`
(LLM ↔ harness loop on GBNF). The current-implementation-vs-target axis
is `05-zebra-puzzle-graph-reasoner` → `04-nlp-to-graph-to-solver-pipeline`
/ `06-inference-rules-completeness` / `08-human-style-deductive-trace`.

## Style

- The user is bilingual RU/EN but prefers EN in code and docs.
- Prefers dense, link-rich answers; few-but-substantive over many-but-thin.
- For plans/ideas/* extensions: keep the user's framing intact; do not
  cite "conversation-N msg M" (raw conversations were removed 2026-05-17).

`CLAUDE.md` is a symlink to this file — both AI tools see the same guidance.
