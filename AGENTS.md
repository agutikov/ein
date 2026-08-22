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
  Since M1a S1a.10.6 two pages are new or renamed and worth knowing:
  [`defined_behaviour.md`](docs/kernel/defined_behaviour.md) — the thirteen
  diagnostics, orderings and error strings whose only statement used to be a
  Python source file, now normative; and `inference/implementation.md` +
  `ir/02-data-model/03_implementation.md`, the module maps that were
  `python_impl.md`. **This tree is now the only statement of intent that is
  not also the implementation**, so it is load-bearing: a claim here is
  checked by `cargo test --workspace` and by nothing else.
- **`docs/api/`** — the **Python embedding API** reference (P1.20 Theme J):
  how to drive Ein *as a library* (`parse` → `KnowledgeBase` → `solve` →
  read verdict/trace). `ein.md` is the contract + worked example; per-module
  pages for `ir`/`kb`/`inference`/`trace`. Distinct from `docs/kernel/`
  (the IR *language*) and the engine internals. **Nothing implements it and
  nothing is scheduled to** — every page says so in a banner. The PyO3
  successor was deferred on 2026-08-21 for want of a consumer
  ([Q-M1a.23](plans/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
  holds the three trip-wires that revive it), so these pages are a **record
  kept in reserve**, not a specification anyone is building against. Do not
  "fix" them to match ein.rs's internals — they describe an engine that no
  longer exists. The embedding surface that *is* real is the **crates**, and
  [S1a.9.4](plans/m1a_rust/p1a.9_release/s1a.9.4_documentation.md) is the
  stage that documents it.
- **`docs/guide/`** — **the newcomer tutorial** ("Learn Ein by solving the
  Zebra puzzle"): objects/relations/facts → rules → the full solve, four
  chapters (P1.20 Theme K). User-facing; references `docs/kernel/` +
  `docs/api/`, never explains internals; complements
  `inference/zebra_walkthrough.md`.
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
- **`ein.rs/`** — the Rust port ([M1a](plans/m1a_rust/README.md)), a
  drop-in replacement for `ein`, and since
  [P1a.10](plans/m1a_rust/p1a.10_single_implementation/README.md) the only
  implementation. Two of its eight crates are dev-only: `ein-corpus` (the
  manifest, the fixture helpers, the bench set) and `ein-parity` (the one
  implementation of what counts as a derivation's *narration* rather than
  its content — [design/01
  §5](plans/m1a_rust/design/01_parity_contract.md#5-legitimate-divergences-the-normalisation-list);
  `EIN_PARITY_STRICT=1` turns it off). The six that ship are `ein-core`
  (interning, `Value`/`FactId`, the layered COW KB, provenance), `ein-ir`
  (lex → parse → macros → imports → load), `ein-infer` (compile → match →
  saturate → the NAF boundary → the hypothesis loop), **`ein-einb`** (the
  `.einb` binary KB container — [P1a.8](plans/m1a_rust/p1a.8_binary_container/README.md),
  and the **only crate that is not `#![forbid(unsafe_code)]`**: its `cast.rs`
  is the one audited module design/12 §2 permits `unsafe` in, which is why it
  is a crate at all), `ein-render` (DOT views, the markdown trace, the
  state/lattice dumps, the JSON summary) and `ein-cli`. They stack linearly up
  to `ein-infer` and fork there — `ein-einb` and `ein-render` are siblings
  above it, and `ein-cli` depends on both.

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
- **`utils/`** — **seventeen scripts, all of them driving `ein.rs`** since M1a
  [S1a.10.4](plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md),
  which deleted the eleven that compared two engines or measured the Python
  one. Every script that runs the engine names the binary — **`$EIN_BIN`** or
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
  [`baseline.md`](plans/m1a_rust/p1a.6_performance/baseline.md) /
  [`scaling.md`](plans/m1a_rust/p1a.7_parallelism/scaling.md) **the CPython
  and PyPy columns are frozen constants**, because the instruments that
  produced them left with the engine they measured.
- **`nlp/`, `smt/`** — scratch areas, 56 KB, wired into nothing. `smt/` holds
  three hand-written `.smt` encodings of the Zebra puzzle and 4-queens, which
  [M1c P1c.2](plans/m1c_external_validation/p1c.2_external_benchmarks/README.md)
  counts as part of its benchmark corpus; `nlp/` holds two throwaway
  dependency-parsing scripts that
  [M2 P2.5](plans/m2_nl_to_ir/p2.5_link_grammar_experiment/README.md) starts
  from. **The two submodules they used to carry** (`opencog/link-grammar`,
  `CVC4/CVC4`) were deinitialised at M1a S1a.10.5 — never checked out here,
  and a cost on every recursive clone. Each README has the one
  `git submodule add` that restores it.

## Running the gate

```sh
cargo test --manifest-path ein.rs/Cargo.toml --workspace     # the whole gate
EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test corpus_cli  # + the 17 slow entries
EIN_ID_SEEDS=8    cargo test … -p ein-render --test id_order_invariance
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
[the oracle ledger](plans/m1a_rust/p1a.10_single_implementation/oracle_ledger.md).

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
