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
  The pages an agent most often needs:
  [`defined_behaviour.md`](docs/kernel/defined_behaviour.md), the
  diagnostics, orderings and load refusals the engine defines normatively
  (§ 4 has one section per refusal);
  [`inference/solution_semantics.md`](docs/kernel/inference/solution_semantics.md),
  the answer as a region R;
  [`inference/read_out.md`](docs/kernel/inference/read_out.md), every row
  `ein solve` prints and what it counts;
  [`configuration.md`](docs/kernel/configuration.md), the `(config …)` flags,
  the CLI options and the `EIN_*` names, each with a *does it change the
  answer* column;
  [`ir/03-ein-lang/01_grammar.md`](docs/kernel/ir/03-ein-lang/01_grammar.md)
  and `00_ebnf.md`, the language;
  [`ir/03-ein-lang/09_natural_language.md`](docs/kernel/ir/03-ein-lang/09_natural_language.md),
  `:why` texts and how a fact reads as English; and
  `inference/implementation.md` with `ir/02-data-model/03_implementation.md`,
  the module maps. What is true of the language and the answer today is
  [§ Standing rules](#standing-rules) below; how each rule came to be is in
  the stage records under `plans/` and in `git log`, not here.
  **This tree is now the only statement of intent that is
  not also the implementation**, so it is load-bearing: a claim here is
  checked by `cargo test --workspace` and by nothing else — though **not every
  sentence on a page is such a claim**, and since M1e
  [Q-M1e.4](docs/history/m1e_review_processing/open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)
  a reader can tell which are: an exact number carries the **command** that
  re-takes it, the **date** it was taken, or the **test** that pins it, and a
  number that can carry none is not written down.
  Since M1e T1e.1.1.1 what it takes to call a claim *settled* is
  [`standard_of_proof.md`](docs/kernel/standard_of_proof.md): a behaviour is
  refuted only by a banked probe, an absence by naming the thing that checks
  it, a **risk not by argument at all**, and an argument for leaving something
  alone holds only while its premise is enforced by something that fails when
  it stops being true.
  **Since M1e S1e.2.2 every one of the pages — 46 since M1e2 S1e2.1.2 added
  `inference/answer_dump.md`, S1e2.1a.1 `inference/region_dump.md`, S1e2.1b.6
  `inference/monotonicity.md`, M1f S1f.5.16 `inference/read_out.md` and its
  examples page, and M1f S1f.5.14 `ir/03-ein-lang/09_natural_language.md`
  (`git ls-files 'docs/kernel/*.md' | wc -l`, 2026-11-10) — is in
  exactly one of three
  declared states, visible from the page rather than from a plan** —
  [README § Which pages to trust](docs/kernel/README.md): *current* (43, no
  banner), *superseded with a banner* (**3**, all of them M1 P1.5b's design —
  `inference/algorithm_layer_n.md`, `inference/lattice_diagrams.md`,
  `inference/parity_baselines.md`), and *moved to `docs/history/`* (**0**).
  Zero, because of the rule the stage took for
  [Q-M1e.3](docs/history/m1e_review_processing/open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted):
  **a page is moved *into* an existing milestone record, never made into
  one** — and M1 is the one shipped milestone with no `docs/history/` entry,
  by the decision `docs/history/README.md` records. **Do not rewrite a
  superseded page to match today's engine**: it is neither a record nor a
  specification afterwards, and `docs/history/m1a_rust/design/07` cites the
  first of the three as the search layer's contract. The pass that
  keeps the other 43 honest is [README § Keeping this
  true](docs/kernel/README.md) — six checks, five of them `utils/doc_audit.py`
  and the sixth a shell, and the sixth is the one that found the most.
- **`docs/api/`** — **how to drive Ein as a library.** Since M1a S1a.9.4 its
  subject is [`rust.md`](docs/api/rust.md): the **crates** — `ein-ir` to load,
  `ein-infer` to solve, `ein-render` to explain, `ein-einb` to cache — which
  is what [M20](plans/m20_gui/README.md) binds against and what nothing
  documented before. **Its worked example is a test.** The page's `rust` block
  is the marked region of
  [`ein-cli/tests/embedding.rs`](ein.rs/crates/ein-cli/tests/embedding.rs) and
  a test in that file diffs the two, so `cargo test --workspace` keeps the
  page true — *edit the test, run it, paste; never edit the block by hand*.
  Distinct from `docs/kernel/` (the IR *language*) and from the
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
- **`docs/history/`** — **shipped milestones, kept as record.** Four entries.
  [`m1e_review_processing/`](docs/history/m1e_review_processing/README.md) is
  the newest (2026-08-27 → 2026-09-01, **closed 2026-10-14**): the full-tree
  review of 2026-08-27 processed finding by finding. Five phases and 28 stages
  as one README carrying the **63-finding ledger**, plus the review's reports
  verbatim (`review/`), Q-M1e.1–22 (`open_questions.md`, fourteen open at the
  close, each row naming its owner or why it has none) and S1e.1.1's soundness
  probes (`search_soundness_probes/`, D1–D9 and the golden audit), kept whole
  because they are the evidence behind M1e2's cases. **Read its § What this
  milestone may claim before citing it**: it swept one of four unswept
  surfaces, and it did not check the engine's answers, which is why M1e2
  exists. **`plans/m1e_review_processing/` is gone**, deleted 2026-10-14. Its 31
  phase and stage files, 8 008 lines, are in
  `git log --diff-filter=D -- plans/m1e_review_processing`, and the other 37
  files moved into the record.
  [`m1d_satisfiability/`](docs/history/m1d_satisfiability/README.md) is the
  one before (2026-08-21 → 2026-08-27): **from saturation to satisfiability** — four
  phases and eighteen stages as one README, plus **fifteen** documents that are
  still read. What it shipped: a program can state a requirement (`(open ?R)`
  asserted by a rule, form G of
  [`obligation_forms.md`](docs/history/m1d_satisfiability/obligation_forms.md)'s
  menu A–G), a state can say what it **owes**, the search branches on it, the
  verdict word `Open` reports it, `--models key` prints a model set compactly,
  every verdict states whether its count is certified, and `EIN_TRAVERSAL=tree`
  reaches the same 32 models in **86 enterings** where the lattice needs
  **17 204 592**. Four of the documents are **re-takable censuses** with a
  `utils/` script apiece — openness, model sets, closure, layers — and
  [`ideas.md`](docs/history/m1d_satisfiability/ideas.md) is authoritative on
  intent, as `plans/ideas/*` is. **P1d.10 was closed as it stood**, three of six
  stages shipped, and its § Eight measurements with no owner is where the
  findings it did not act on are kept — read it before re-opening any of them.
  **`plans/m1d_satisfiability/` is gone** — deleted 2026-08-27, 23 files of
  phase and stage documents that were intent rather than record
  (`git log --diff-filter=D -- plans/m1d_satisfiability`).
  [`m1c_external_validation/`](docs/history/m1c_external_validation/README.md)
  is the newer and smaller (2026-08-23 → 2026-08-24): one phase and five stages
  as one README — `:expect` on `query`, `ein test`, 45 programs in
  `tests/stdlib/`, the coverage number in `cargo test` — plus
  [`stdlib_census.md`](docs/history/m1c_external_validation/stdlib_census.md),
  which is the evidence and is **still re-takable**, and seven `Q-M1c.<n>`
  questions of which two are open on purpose.
  **`plans/m1c_external_validation/` is gone** — deleted 2026-08-24, the day it
  shipped (`git log --diff-filter=D -- plans/m1c_external_validation`).
  The first and larger is [`m1a_rust/`](docs/history/m1a_rust/README.md), the Rust engine
  (2026-08-17 → 2026-08-23): one README carrying all eleven phases and 53
  stages, plus what is still *read* rather than merely intended — the
  [`design/`](docs/history/m1a_rust/design/README.md) contracts the crates cite
  as their specification, six
  [`measurements/`](docs/history/m1a_rust/measurements/) documents, and the
  [questions](docs/history/m1a_rust/open_questions.md).
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
  per file / sub-dir. **Four of the zebra files are generated**, by
  `examples/gen_zebra2_variants.py` from `zebra2.ein`, and `--check` is in the
  gate: `zebra2-minus-15` (clue dropped), `ein-bugs/zebra2-bad` (clue added)
  and — since M1d S1d.2.5 —
  [`zebra2-obligations.ein`](examples/zebra2-obligations.ein) and its
  `-minus-15` twin, which are `zebra2` with the `(hrule guess …)` and the
  `(query … :hrules …)` clause **deleted and nothing else**. They are what
  exercises the obligations rung, and they solve to the same models in the same
  number of enterings as the hrule path they dropped.
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
  → the checkout → the embedded copy. **What tests it is `tests/stdlib/`**
  (below), and adding a rule here means adding a program there. Since M1d
  S1d.2.4 four of its rules are **obligations** — `total-owed` /
  `surjective-owed` in `std.algebra`, `slot-owed-room` / `slot-owed-fill` in
  `std.slots` — which assert `(open ?R)`, derive nothing, are not in the
  saturation agenda, and are read by one pass over the quiescent KB. No puzzle
  changes a line: the two setup fan-outs pick them up, at a cost of two stored
  activator facts per declaration (50 across the corpus's 13 such entries).
  **Since S1d.2.5 those four rules are also a hypothesis generator.**
  Generation is a *ladder* — the puzzle's `(hrule …)` if it declares one, else
  the facts that would discharge what the state owes
  ([`oblgen.rs`](ein.rs/crates/ein-infer/src/oblgen.rs)), else the blind
  combinatorial enumerator — so `(bijective color-loc)` now tells the search
  what to guess and `:hrules` is an override rather than the only way in. A
  program declaring no obligation rule never consults the middle rung and its
  streams are byte-identical to before.
  **Since M1f P1f.6 three modules are the calculator's** (2026-11-11):
  `std.digits`, a library of *facts*, the digit tables, generated by
  `utils/gen_arith_tables.py` (never edit its rows); `std.arith`, school
  arithmetic over them on numbers of any length, computed by saturation
  alone and imported by one name, `(import std.arith :symbols (arith))`;
  and, since S1f.6.13, `std.num9`, the same on nine digits in one fact by
  wide rules, generated by `utils/gen_num9.py` (never edit the file) and
  imported as `(import std.num9 :symbols (arith9))`. The rule of both
  theories is that **no rule joins two facts on a number**, and a relation
  or a rule added to `std.arith` goes into its closing rule's lists. **Since
  S1f.6.7 `std.int` extends it** (2026-11-12): integers, `(int P N)`, by
  rules of one premise each over `std.arith`'s walks, imported as `(import
  std.int :symbols (int-arith))`, its division under three conventions since
  S1f.6.22, `int-div-euclid`, `int-div-trunc` and `int-div-floor`; a module over a module names the lower
  one's entry in its own closing rule. **Since S1f.6.8 `std.rat` extends
  that**: rationals in lowest terms, `(rat Z D)`, imported as `(import
  std.rat :symbols (rat-arith))`; a zero denominator is `std.arith`'s
  division by zero;
  [`stdlib/README.md`](stdlib/README.md) has a section for each.
- **`tests/`** — **the ein-lang test suites**, and since M1c S1c.1.4 a third
  corpus root beside `examples/` and `stdlib/`: a `.ein` here with no
  `corpus.toml` entry fails the same completeness check. Its one subject is
  [`tests/stdlib/`](tests/README.md) — **one program per stdlib rule or tight
  family** (`find tests/stdlib -name '*.ein' | wc -l`) plus one pair that is a
  *corner* rather than a rule
  (`closure/02_closed_and_satisfied` and `03_closed_and_owing`, M1d S1d.2.2:
  the same program with one fact deleted, banked so the stage that fixes it has
  to move the golden — **cashed at S1d.2.6**, where both grew
  `(total-owed r is-a)` and the pair went from reporting one word for two
  states to `Solution` against `Open — owes 1`. What moved it is that one
  declaration and not the verdict change: the read-out judges by discharge only
  where a program *states* an obligation), each carrying an `:expect` so `ein test tests/` is the
  whole suite in 0.04 s. They are deliberately *not* under `examples/`: that
  directory is things to read, and these are three declarations and two facts
  apiece that exist to break. Three idioms are worth knowing before writing a
  new one — **naming a relation closes it** (which is how a rule with no guard
  gets a negative case at all), **`(unknown …)` as a probe** at priority 500
  (the only way to say a rule did *not* derive a negative, since stored
  negatives are not closed), **a refutation rule gets two files** — one where
  it fires and one where it is loaded, activated and satisfied — and **where
  two rules reach one verdict, separate them by activation**, because an
  expectation made of facts cannot say which one got there. **M1d S1d.2.4
  added a fifth**: a rule that asserts the verdict atom `open` derives nothing
  and is never in the saturation agenda, so `:expect` — three forms, all of
  them assertions about *facts* — cannot state what it reports, and the eight
  obligation fixtures carry an ordinary `(model …)` claim while their owe
  counts are asserted in `ein-infer/tests/obligation_reports.rs`. Since M1e2's
  switch a claim can say that a state owes, though not how much:
  `(solve … :expect (region (owing (model …))))`, which is what killed eight
  banked mutation survivors (`tests/README.md`, M1e2 S1e2.3.6). **Since
  S1d.2.6 twelve of these programs report `Open` rather than `Solution`** and
  every one of their `(model …)` claims still holds unchanged, which is the
  same rule read from the other end: an expectation is about a fact set, and
  the facts an open state reached are the facts it reached. The coverage claim
  is measured by `utils/stdlib_census.py`, never by reading the directory, and
  the suite's sensitivity by a 51-mutant sweep it catches 50 of. **Since M1c
  S1c.1.5 two claims about this directory are in `cargo test`**
  (`ein-infer/tests/stdlib_coverage.rs`, 0.04 s then and half a second with
  the calculator's programs): every stdlib rule is
  activated by a program *here* — **77 of 77**, 96 of 96 since M1f
  S1f.6.3's `std.arith`, 114 of 114 since S1f.6.4's subtraction and
  comparison, 130 of 130 since S1f.6.5's multiplication, 166 of 166
  since S1f.6.6's division, 213 of 213 since S1f.6.13's `std.num9` and
  250 of 250 since S1f.6.7's `std.int`, 307 of 307 since S1f.6.8's
  `std.rat` and 341 of 341 since S1f.6.22's division of integers (`cat stdlib/*.ein | grep -c '^(rule '`, 2026-11-12), with no `examples/`
  entry contributing — and every program here states an expectation. **Activation is
  two events since S1d.2.4**: `fire` for a saturation rule and `owe` for an
  obligation one, which can never produce a `fire`. Scoping the
  first to the suite is what found `transitive`, whose fixture was a two-cycle
  the `(neq ?a ?c)` guard refuses every match of; it grew a three-chain.
- **`corpus/`** — the **corpus**: `corpus.toml` (one entry per `.ein`, with
  the runs it is exercised under) plus `fuzz_findings/`. **Since M1e2
  S1e2.1.1 also `wrong_answers.toml`**, the register of every known wrong
  answer. It has one row per witness, with today's answer and the right one as
  data, and `ein-cli/tests/wrong_answers.rs` holds an `open` row to its wrong
  answer and a `fixed` one to its right one. A fix flips its rows in the same
  commit as any `:expect` that stated the wrong answer. A file under
  `examples/`, `stdlib/` or `tests/` with no entry fails a completeness check. What
  runs it is `cargo test`, and [`corpus/README.md`](corpus/README.md) has the
  table of readers. It was `conformance/` until M1a S1a.10.3, and the
  `--events` protocol it also held is now
  [`docs/kernel/inference/events.md`](docs/kernel/inference/events.md).
  Since [S1a.9.0](docs/history/m1a_rust/README.md#s1a90--the-slow-corpus-re-priced)
  **`slow = true` is a measured claim** — an entry whose declared runs cost
  1 s or more together, recorded in `cost_ms` and checked in both directions
  ([corpus_cost.md](docs/history/m1a_rust/measurements/corpus_cost.md) is the
  measurement, `utils/corpus_cost.py` re-takes it). **Fifteen** entries are
  slow, fourteen of them since M1e2 S1e2.1c.2 (re-taken 2026-11-03,
  `utils/bench_env.sh --cores PT python3 utils/corpus_cost.py`). Describe takes the state route since
  that stage, and the default `--region-budget` is 2²⁰ facts, which the user
  ruled after 2²⁴ ran the gate's sweeps out of memory. The six `square-*` cubes
  joined, their `render lattice` counting states to the budget (1.6 s each).
  `features/02` and `zebra2-hints` left, which the state route describes in
  milliseconds. `zebra2` and `zebra2-obligations` dropped to 18.6 s apiece,
  9.4 s of it `solve -n 3`, which was still route 1 with a cut, and the
  `zebra2-minus-15` pair rose to 7.9 s. M1e2 S1e2.1c.4 routed the modes over
  states, and the re-take of the same day kept the same fourteen: `zebra2`
  and `zebra2-obligations` fell to 6.1 s and 6.0 s, their `solve -n 3` to
  114 ms ([corpus_cost.md
  §10](docs/history/m1a_rust/measurements/corpus_cost.md#10-the-fourth-re-take--2026-11-03-m1e2-s1e21c4s-routes-for-the-modes)).
  It was ten at M1e2 S1e2.1a.5's
  switch (2026-10-18), when `features/04_open` stopped being slow because no
  cell runs the walk, three before that, and
  seventeen before M1a T1a.7.2.0 made `branching/07_lookahead_off` 2.8×
  cheaper and the re-take took its flag off — the mechanism working both ways
  ([corpus_cost.md
  §7](docs/history/m1a_rust/measurements/corpus_cost.md#7-the-first-re-take--2026-08-22-and-it-moved-an-entry));
  and **a run is dropped from a `runs` column only when it does not ask the
  fixture's question**, never for costing too much.
- **`ein.rs/`** — the engine ([M1a](docs/history/m1a_rust/README.md)). Two of
  its eight crates are dev-only: `ein-corpus` (the manifest, the fixture
  helpers, the bench set) and `ein-narration` (the one definition of what
  counts as a derivation's *narration* rather than its content: a fork resumes
  root's saturation, so two runs can narrate different amounts of one
  derivation and reach the same answer; `EIN_NARRATION_STRICT=1` turns the cut
  off). The six that ship are `ein-core`
  (interning, `Value`/`FactId`, the layered COW KB, provenance), `ein-ir`
  (the parser grmtools generates from `src/ein.l` and `ein.y` → macros →
  imports → load; the hand-written `lex.rs` and `parse.rs` until M1f S1f.5.5's
  swap), `ein-infer` (compile → match →
  saturate → the NAF boundary → the routes over R, `explore` (the walk's
  hypothesis loop until M1e2 S1e2.1c.5 deleted it); and since M1e S1e.3.3
  `invariant.rs`, the **M1 alive-set invariant** — *no rule asserts a name
  the program lacks* — whose list of the program's names seeds the blind
  enumerator's objects: a rule's `:assert` constants since M1e2 S1e2.2.4,
  because a rule's constant is a name of the program (Q-M1e.21 (b)), and
  every constant a fact names at any depth or a rule's `:match` names since
  S1e2.2.14 (Q-M1e2.9 (a); not one only a query names).
  [`examples/ein-bugs/alive-set-fresh-name.ein`](examples/ein-bugs/alive-set-fresh-name.ein)
  is the eleven lines on which a rule's constant missing from the candidates
  cost a model, W4), **`ein-einb`** (the
  `.einb` binary KB container — [P1a.8](docs/history/m1a_rust/README.md#p1a8--binary-kb-container),
  and the **only library crate that is not `#![forbid(unsafe_code)]`**: its
  `cast.rs` is the one audited module design/12 §2 permits `unsafe` in, which
  is why it is a crate at all; the other audited `unsafe` is the binary's,
  `ein-cli/src/main.rs`'s one `libc::signal(SIGPIPE, SIG_DFL)` as `main`'s
  first statement, since M1f S1f.5.25, so a closed stdout ends `ein` by
  `SIGPIPE` as it ends `cat`, where every `print!` panicked, exit 101), `ein-render` (DOT views, the read-out, the region trace,
  the answer and lattice dumps, the JSON summary) and `ein-cli`. They stack linearly up
  to `ein-infer` and fork there — `ein-einb` and `ein-render` are siblings
  above it, and `ein-cli` depends on both. `ein-cli` fronts **five**
  subcommands — `ein {oracle,render,saturate,solve,test}`, `oracle` since M1e2
  S1e2.1a.9 and `test` since M1c S1c.1.3 — plus `ein kb` under the `einb`
  feature.

  **`.einb` is a private cache format, never an interchange one.** `ein kb save
  <file.ein> <out.einb>` writes one and every command that takes a `.ein` path
  takes a `.einb` too, dispatching on the magic bytes rather than the
  extension; `ein solve x.einb` is byte-identical to `ein solve x.ein` apart
  from the path it echoes. Anything crossing a tool boundary is still `.ein`
  text or the event protocol's JSON.
- **`utils/`** — **thirty-seven scripts** (`ls utils/*.py utils/*.sh | wc -l`,
  2026-11-12; the thirty-second, `nl_census.py`, is M1f S1f.5.14's: how much of
  what Ein answers reads as English, the thirty-third,
  `gen_arith_tables.py`, M1f S1f.6.2's: it writes `stdlib/digits.ein` and runs
  no engine, two M1f S1f.6.15's: `gen_arith_examples.py` writes three
  example programs, and `arith_cost.py` measures where an arithmetic
  program's time goes, one M1f S1f.6.13's, `gen_num9.py`, which
  writes `stdlib/num9.ein`, and the last M12's, `rule_census.py`, the census
  of rules, which reads the corpus as text), **all but five driving `ein.rs`**.
  Every script that runs the engine names the binary — **`$EIN_BIN`** or
  `--bin` — defaulting to `ein.rs/target/release/ein`, except the one that
  wants a build of its own (`profile_ein_rs.py` → `--profile profiling`,
  which it builds). `fork_delta_verify.py` and `spec_audit.py`, which wanted
  the walk's `fork-delta` and `spec-audit` builds, went with the walk at M1e2
  S1e2.1c.5.
  [`utils/README.md`](utils/README.md) is the catalogue — one line per script
  in three groups (*renderers*, *checks*, **the M1a measurement set**). One
  thing worth knowing without opening it: run every measurement through
  **`bench_env.sh`**, which prints the machine state the numbers were taken
  under. The nineteenth,
  **`stdlib_census.py`**, is [M1c](docs/history/m1c_external_validation/README.md)'s
  and the first check aimed at the *stdlib* rather than the engine: 77 rules
  parsed out of `stdlib/*.ein`, then every corpus entry × every declared
  `solve` / `saturate` / `test` run under `--events`, `fire` counted by rule
  — plus `owe` since M1d S1d.2.4, which is the same claim for the four rules
  that assert the verdict atom `open` and therefore never reach the firing
  stream.
  Its first answer, 2026-08-23 — [**38 of 73 rules never
  fire**](docs/history/m1c_external_validation/stdlib_census.md),
  and `examples/zebra.ein` the sole activator of 20 more — is what M1c existed
  to close, and S1c.1.4 closed it: **0 of 73** on the re-take of 2026-08-24
  ([§11](docs/history/m1c_external_validation/stdlib_census.md#11-the-re-take--2026-08-24-and-the-zero-set-is-empty)),
  and 77 of 77 over 217 entries in 38 s when M1e S1e.4.5 re-took it on
  2026-09-01. `--check` exits 1 while any rule is at zero, or while a declared
  run exits 0 and narrates nothing, and is **not** the gate: S1c.1.5 made that
  a cargo test, in-process and scoped to `tests/stdlib/`. It **ran nightly**
  from S1e.4.5, beside `stdlib_mutants.py --check`, and runs on the dev host
  since M1f S1f.5.27 (`./run_tests.sh --dev-host`), a measurement over 5 min
  on the runner, where its sibling stays nightly. What stays here is the
  measurement the gate is a yes/no of.
  The twentieth, **`layer_census.py`**, is
  [M1d](docs/history/m1d_satisfiability/README.md) S1d.10.1's and asks what a *layer of
  the search* kills and what the killing is worth: every entry under `solve -e`
  twice — bare for the wall and the RSS, narrated for the new **`layer`** event's
  sixteen counters, of which `dropped_nogood` (what the learned clauses removed
  from the next layer's join) is the one nothing reported before. Its answer,
  2026-08-24: of 2 232 330 joined candidates corpus-wide **0** were dropped for a
  dead element and **31 303 — 1.4 %** by a clause, and for **25 of the 49**
  entries that search at all the enterings are *exactly* `Σₖ C(alive, k)`
  ([layer_census.md](docs/history/m1d_satisfiability/layer_census.md)).
  It writes `--events` to a **FIFO**, because the run it exists to measure
  narrates 72.6 M events.
  The twenty-first, **`openness_census.py`**, is M1d S1d.2.6's and asks a
  question about *programs* rather than about the search: what does each entry
  **owe**, and is it judged by discharge or by exhaustion? Its third number is
  the one that did not exist — **`declared`**, how many obligation rules a
  program *states* — because `owes = 0` is equally true of a debt paid and of a
  debt never stated, and only the first may be called satisfied. Its answer,
  2026-08-25: of the 121 entries that reach a fixpoint **92 state no
  obligation** and keep exactly the verdict they had, 12 are discharged, 17 owe,
  and **12 moved to `Open`**
  ([openness_census.md](docs/history/m1d_satisfiability/openness_census.md)).
  The twenty-second, **`model_set_census.py`**, is M1d S1d.3.1's and is the
  first whose subject is the **answer**: what is a model set made of, and does
  it *factor*? It turns `--json-summary`'s `verdict.solutions` into decision
  variables — every varying positive atom a Boolean, and only a `functional` /
  `bijective` declaration read off the models' own facts licensing the collapse
  of `(R a ·)` into one multi-valued variable — then asks independence at three
  granularities plus a fourth nothing had asked: is the set a **free grid** over
  a small determining basis? Its answer, 2026-08-25: **13** corpus entries have
  a model set (four more than a `-m 2` count sees), **2** of them partition and
  both are two-object demos where the three-object sibling does not, 5 are a
  free grid and every one has `k ≤ 4`, and `examples/zebra2-minus-15.ein`'s 23
  varying variables are **one** coupling component whose graph is K₂₃ minus five
  edges with a minimum vertex separator of **17**
  ([model_set_census.md](docs/history/m1d_satisfiability/model_set_census.md)).
  It also carries the probe P1d.2 declined — `EIN_LEFTOVER=1`, above — and its
  number: `zebra2`'s **unique** model leaves **3 678** facts the blind
  enumerator would still propose, none of them an attribute arrow. That was
  the count through the walk's lookahead filter, until 2026-11-03: config-free,
  as H is since M1e2 S1e2.3.1, it is **8 203**, and 4 525 of them are
  attribute arrows, 905 per `*-loc` relation, which the filter dropped
  (`EIN_LEFTOVER=1 ein solve --maximal examples/zebra2.ein --json-summary J`).
  Since S1d.3.2 it has a second half, **`--form {envelope,key,list,diagram}`**,
  which renders a model set as one of the candidate *representations* rather
  than as a census row — because a representation argued about in prose and
  never printed is one nobody has read. Its answer, priced on four columns
  (produce · size · exact · read) in
  [representations.md](docs/history/m1d_satisfiability/representations.md):
  the **determining key** wins at 2 506 bytes and is *verified* exact — all 32
  key rows reconstruct their model to the fact, 30 of them without entering a
  commitment — while the certain core, the form the stage expected to win,
  cannot say how many models there are and invites arithmetic that over-states
  by 3.11 × 10¹².
  The twenty-third, **`closure_census.py`**, is M1d S1d.4.1's and asks about
  the **claim** rather than about the search, the program or the answer: who
  states that a model set is *exactly* these k, and what stating one would
  cost. Its transport is **`ein test --json-report`**, a read-out the same
  stage added, and the reason it had to exist is the reason the stage insisted
  the census be *parsed*: the reconnaissance grepped `:expect (or`, found two
  users, and one of them was a **header comment documenting the form**. Parsed,
  the corpus states **59 claims over 124 queries and exactly one about a set**,
  all 59 hold, and all 59 exhausted. Its two new numbers are the **write** cost
  — `k × |goal extent| / |file|` under *naming a relation closes it*, worst on
  a 95-line feature demo at **4.28×** and not on the puzzle at 0.96× — and the
  **counterfactual `NOT CHECKED` set**, which is what an empty outcome column
  means: **10 of the 121** entries that reach a fixpoint do not exhaust at
  `ein test`'s depth, so a claim written on any of them could not be checked
  ([closure_census.md](docs/history/m1d_satisfiability/closure_census.md)).
  The twenty-fifth, **`stdlib_mutants.py`**, is M1e S1e.3.6's and asks whether
  `tests/stdlib/` would **notice a defect** in the rule it tests: four
  mechanical mutation families over every `(rule …)` in `stdlib/*.ein`, each
  spliced into a copy the run points `$EIN_STDLIB` at, killed when `ein test
  tests/` exits non-zero. `tests/README.md` had carried a **hand-taken** 50 of
  51 since M1c with one named survivor; the script is exhaustive where that was
  one-per-family — **157 of 217** on 2026-08-31, **165** on 2026-11-03,
  when M1e2's region claim had killed eight obligation-rule mutants, and
  **167** on 2026-11-08, when M1f S1f.5.17 found two more that *forbid*'s
  refusal had killed at `597b2b7e` — and banks
  the survivors as a *set* in `tests/mutation_survivors.txt`, because a score
  decays while the thing it guards grows. **42 of the 50 are in `slots.ein`**
  (2026-11-08). The recorded survivor is
  dead: `tests/stdlib/slots/13_adjacent_bwd_neg_direction.ein`.
  The twenty-fourth, **`doc_audit.py`**, is M1e S1e.2.2's and the only one
  whose subject is **the documentation** rather than the engine: does a page in
  `docs/kernel/` still describe the engine that ships? Three questions, none of
  which needs to know what a milestone changed — identifiers resolved against
  `ein.rs/crates/**` (a report, not a gate: `Human` and DOT node ids are not
  identifiers), links and anchors, and which pages carry a banner. The fourth
  is the one it exists for: a **prose `§x.y`** after a link is not part of the
  link, so no anchor checker sees one, and the stage found six such citations
  naming four sections `algorithm_layer_n.md` has never had. `--check` exits 1
  on the link half. Its fifth check has no instrument and found the most —
  *run the commands a page shows*
  ([`docs/kernel/README.md` § Keeping this true](docs/kernel/README.md)).
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
- **`nl2smt/`** — **the Zebra puzzle's sentences through link grammar to
  SMT**, a proof of concept ([M1j](plans/m1j_nlp_smt_poc/README.md); `nlp/`
  and `smt/` until then), wired into nothing: no crate depends on it and the
  gate does not run it. `python3 nl2smt/nl2smt.py nl2smt/puzzles/zebra.txt`
  takes English lines over a written vocabulary (`vocabulary.rktd`) through
  `link-parser`, a Racket converter (`lg2smt.rkt`) and an SMT solver, one
  direction; `python3 nl2smt/test_pipeline.py` is its tests, which skip where
  `link-parser`, `racket` or every solver is missing.
  [`nl2smt/README.md`](nl2smt/README.md) is the page. `nl2smt/smt/` holds three
  hand-written `.smt` encodings of the Zebra puzzle and 4-queens, which
  [M10](plans/m10_external_benchmarks/README.md)
  counts as part of its benchmark corpus, so their content does not change;
  `nl2smt/prototype/` holds two throwaway dependency-parsing scripts that
  [M2 S2.6.4](plans/m2_nl_to_ir/p2.6_ablations/s2.6.4_representation_ablations.md)
  starts from — the link-grammar A/B, one arm of M2's representation
  ablation since the 2026-08-23 reshape. **The two submodules the old
  directories carried** (`opencog/link-grammar`, `CVC4/CVC4`) were
  deinitialised at M1a S1a.10.5 — never checked out here, and a cost on every
  recursive clone. The page has the one `git submodule add` that restores
  each.

## Where the work is

**This file holds standing rules, locations and commands. It is not a
log.** A stage writes what it did to its own stage file under `plans/` and to
its commit message. Edit this file only when a rule, a location, a command or
the pointer below changes, and then in place: no dated *"S… ran to its end"*
paragraph, no register count, no measurement (the user, 2026-11-13).

- **Current milestone: M1f**,
  [`plans/m1f_hypothesis_and_documentation/`](plans/m1f_hypothesis_and_documentation/README.md).
  P1f.5 and P1f.7 are closed. **P1f.6, the calculator, is running**: its
  [README](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/README.md)
  has the stage table, and
  [`replies_2026-11-12.md`](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/replies_2026-11-12.md)
  the order the remaining stages (S1f.6.24 to S1f.6.33) run in and which
  reply or default each rests on.
- **What waits for the user** in P1f.6:
  [`questions.md`](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md)
  and `questions_round2.md`; the phase keeps what it finds in its own
  [`findings.md`](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)
  (E-n) and shows it. Milestone-wide questions, with every default taken, are
  [`open_questions.md`](plans/m1f_hypothesis_and_documentation/open_questions.md)
  (Q-M1f.n); a default stands until the user revises it.
- **The wrong-answer register** is
  [`corpus/wrong_answers.toml`](corpus/wrong_answers.toml); its counts are
  `grep '^status' corpus/wrong_answers.toml | sort | uniq -c`. Every open row
  is owned by a followup ([F21](plans/followups/f21_non_monotone_programs_decided.md),
  [F22](plans/followups/f22_exact_core_past_the_cap.md),
  [F24](plans/followups/f24_written_provenance.md)) or by M1f's S1f.10.6.
- **After M1f**: [M1g](plans/m1g_logic/README.md), logic in Ein (school
  algebra and geometry are its P1g.10 and P1g.11);
  [M1h](plans/m1h_beyond_word_problems/README.md);
  [M12](plans/m12_rules_as_data/README.md), rules as data (its
  `rule_census.md` tables are written by `utils/rule_census.py --page`);
  [M15](plans/m15_mcp_server/README.md).
  M1e2 closed on 2026-11-08 and its plans stay in `plans/m1e2_correctness/`.

## Standing rules

### The language

Each of these is a refusal at load or at the parse, on every command, stated
normatively in [`defined_behaviour.md`](docs/kernel/defined_behaviour.md) § 4
or in the grammar's table (`ein-ir/tests/grammar_pages.rs`).

- **Ein has no numbers, no ranges and no int kind**, and none is planned.
  `1` and `01` are two names. Arithmetic is a theory in the stdlib, computed
  by saturation.
- **A value is a name, or a fact a state could hold**: an atom of an ordinary
  relation or its negation. `eq`, `neq`, `absent`, `and`, `or` and `false`
  with an argument are never a nested head; `not` of a name is the negation
  of no fact.
- **A string is natural-language text**, written only after `:why`, `:source`
  or `:goal-text`, never a value. Names are ASCII.
- `_` and `?_` are refused everywhere. Keywords are closed per form.
  `:priority` is gone: the agenda is FIFO in written order.
- **`:why` is required** on every relation, rule, hrule and macro;
  `--no-require-why` loads a program without.
- `__closed__` and `__symmetric__` are reserved and never bound; names the
  loader makes up start with `#` (`ein_core::text::INTERNAL_NAMES`).
- `eq` and `neq` are computed relations, never stored. A conclusion's
  variable must be bound by every way of satisfying the `:match`. An empty
  `:match` is true and fires once.
- A query's `:goal` is held to what a rule's `:match` is held to. Every name
  in a `(solve …)` list names something the program has. A macro's call is no
  fact.
- **A form headed by a declared relation has as many arguments as its
  declaration has types.** It is no type check. A fixture that wants a
  relation at several counts leaves it undeclared.
- **A flat import brings a module's facts**; a qualified one renames
  declarations only, so a library of facts is imported flat. **No import
  brings a module's `(query …)`, `(solve …)` or `(config …)`**: those are a
  program's. Where a selection brings a relation's declaration through its
  closure and no rule that derives it, the loader says so in one `note:`
  line on stderr and refuses nothing (`ein-cli/tests/import_left_behind.rs`).
- *Forbid*: a program whose refutation could rest on an `absent` over a
  relation a commitment can still extend is refused (`ein_infer::forbid`).

### The answer

- **The answer is a region R**, the commitments whose state is alive and
  discharged. A bare `ein solve` describes R; `--minimal`, `--maximal`,
  `--single`, `-n N` and `--count` each do the least work their question
  needs; **`ein oracle` is the reference every mode is held to**
  (`ein-infer/tests/explore.rs`). No two commands may disagree.
- **Every answer exits 0.** Only a refuted claim exits 1; a load error takes
  each command's load-error exit (2 under `ein test`). A count of commitments
  is exact in the engine and reported up to 2^63 − 1, which every surface
  reads as *this many or more* (`ein_infer::count`).
- **No config option, lever or flag changes R**, held per commit by
  `ein-cli/tests/lever_invariance.rs`. The one exemption:
  `:record-alternative-justifications false` may change a dead node's core.
- **A core is an explanation**: it may vary with rule order, and F22 makes it
  canonical. A node that is not dead has no core (`null`); `[]` is the empty
  core of a dead node. A core is taken only where a surface prints one.
- `ein test` asks a file **the least question that can decide the claims
  it states**, read off their shapes: `--single` or `-n k+1` for queries'
  claims all over one part of R, describe for a `(solve … :expect …)` or
  for claims over more than one part; what that leaves undecided it asks the
  describing question, of the program loaded again. No option chooses the
  question. A claim is held only by an answer that lists all of what it is
  about, and under `ein test` alone a state found whose class the claim does
  not list refutes it (`ein-infer/tests/claim_routes.rs` holds each question
  to the description and to the oracle). It never solves a query with no
  `:expect`, and reads every claim of a file off those one or two runs.
- Every command reads a program by one path (`ein-cli`'s
  `common::read_program`), `.ein` or `.einb` alike.

### Edits that go together

- **Generated, never edited by hand** (edit the generator and re-run):
  `00_ebnf.md`'s two grammar blocks (from `ein.l` and `ein.y`, `EIN_BLESS=1`);
  `stdlib/digits.ein` (`utils/gen_arith_tables.py`, then
  `utils/stdlib_manifest.py --write`); `stdlib/num9.ein` (`utils/gen_num9.py`);
  three `examples/arith/` programs (`utils/gen_arith_examples.py`); the four
  zebra2 variants (`examples/gen_zebra2_variants.py`);
  `read_out_examples.md` (`utils/read_out_examples.py`); `configuration.md`'s
  defaults block and `docs/api/rust.md`'s `rust` block. All of those have a
  `--check` or a test in the gate. **Not in the gate, run by hand**:
  `examples/binary_arithmetic/gen_binary.py` (after any edit to
  `stdlib/arith.ein`) and `examples/binary_logic/gen_gates.py`.
- **`ein.rs/vendor/lrpar/` is outside the workspace**: edit it only through a
  change to `ein.patch`. Nothing goes upstream.
- A change to what `ein solve` prints changes `read_out.md` in the same
  commit. A shape the grammar newly refuses gets a row in the grammar page's
  table.
- **A wrong answer** is filed as an `open` row of the register, with a
  witness under `examples/ein-bugs/wrong-answers/` and a corpus entry, in a
  commit of its own; the fix flips the row, and any `:expect` that stated the
  wrong answer, in the fixing commit.
- **A stdlib rule needs a program under `tests/stdlib/` that activates it**,
  or `stdlib_coverage.rs` fails. A rule or relation added to `std.arith`
  (or to a module over it) goes into its closing rule's two lists
  (`arith_theory.rs`); a new operation is a row of `OPS`, and joins
  `arith_search_free.rs` by its lists. **No rule of the calculator joins two
  facts on a number.** `std.arith` reads **one `absent`, over `let`**, which
  no rule may derive, and writes guards in three named rules
  (`arith_theory.rs`); a program it flags, `(malformed …)` or
  `(undeclared …)`, is dead and claims only `(solve :expect (false))`.
- **A stage that changes a rule of the calculator runs the batch by hand**:
  `cargo test -p ein-infer --features counters --test arith_batch`. No tier
  runs it.
- `dot_wellformed.rs` skips a view of more than 1 000 edges; a program with
  such a view in a directory not named in its `SKIPPING` fails the sweep.
- **A corpus program in which a hypothesis leads to a request to `std.arith`
  through a rule with an unbound operand may hang `region.rs`'s monotonicity
  test** (E-33); run it alone before a gate.
- A file-relative import cannot reach a sibling directory (E-32).

## Running the gate

`./build.sh` builds everything first (see above); this runs it.

```sh
./run_tests.sh                                               # the whole per-commit tier
./run_tests.sh --tests-only                                  # skip the static checks
./run_tests.sh --nightly                                     # M1e2 S1e2.3.6: instead of the gate, every step of nightly.yml, its three jobs in order
./run_tests.sh --dev-host                                    # M1f S1f.5.27: instead of the gate, the nightly at full budget and the steps that left it
EIN_FUZZ_ITERS=20000 EIN_FUZZ_SEED=1 cargo test … -p ein-ir --release --test fuzz_properties     # the nightly's fuzz job, once per seed (1, 987654321); the dev host runs 1000000
cargo bench … --bench engine -- --test                       # the nightly's bench job, and the gate's last step
cargo test … -p ein-infer --features counters --test arith_batch   # M1f S1f.6.3: the calculator's batch, by the matcher's count; by hand since S1f.6.21, in no tier
cargo test --manifest-path ein.rs/Cargo.toml --workspace     # the tests alone
EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test corpus_cli  # + the slow entries, which the_slow_set_is_exactly_these_fifteen names
EIN_ID_SEEDS=8    cargo test … -p ein-render --test id_order_invariance   # the dev host's, since M1f S1f.5.27
EIN_JOBS_SWEEP=2,4,8,16 cargo test … -p ein-render --test jobs_invariance
EIN_BLESS=1       cargo test … --workspace                   # re-bank the goldens
ein solve … --maximal --models key|grid                       # the M1d S1d.3.3 forms, over the states listed
EIN_LEFTOVER=1    ein solve … --json-summary out.json        # the M1d S1d.3.1 probe
ein test examples tests stdlib --json-report r.json          # the M1d S1d.4.1 read-out
ein solve … --dump-answer DIR                                # M1e2: the answer as files, each state loadable
ein solve … --maximal | --minimal | --single | -n N --region-budget F   # M1e2: a question about R, the least work it needs
ein solve … --count                                          # M1e2: how many, R's two counts by describe's route, no state read
ein oracle FILE [--dump DIR]                                 # M1e2: R by brute force, 2ⁿ saturations — the reference every mode is held to
EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test lever_invariance   # M1e2: every mode under every lever, + the slow pair's cells
EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test answer_round_trip  # M1f S1f.5.13: every dump reloaded, + the slow and heavy_dump entries'
ein render region DIR [--visited D] [--answer D] [--view …]  # M1e2: an oracle dump drawn, a solve run on it
ein solve … --dump-lattice DIR                               # M1e2: every node the run visited (ein render lattice draws them)
cargo run --release -p ein-infer --example route1_reach      # M1e2: route 1, R from the alive down-set, over the corpus
ein render program|kb FILE                                   # M1e2: the program as written; a KB (or a dumped model)
python3 utils/review_bundle.py                               # M1e2: build/review/, one page per register row
python3 utils/review_bundle.py --regions                     # M1e2: build/review/regions/, every corpus region surveyed
python3 utils/region_synth.py --example three_components     # M1e2: a program written for a chosen region (--check, --random)
python3 utils/read_out_examples.py                           # M1f S1f.5.16: the read-out reference's examples page, written from the binary
```

**Since M1e2 S1e2.1a.5 (2026-10-18) the walk has no CLI path, and since
S1e2.1c.5 (2026-11-03) no code**, so several of the paragraphs below describe
it as M1d and M1e left it — `EIN_OBLIGATION_CHOICE`, `EIN_TRAVERSAL`, the stop
policy, `Ambiguity`'s qualifier and the walk's `:expect` wording are records
of a library that was deleted. `EIN_OBLIGATION_CHOICE` was read by the
generator the summary's `root` block called until M1e2 S1e2.3.1 retired it;
`EIN_TRAVERSAL` went with the walk. What
carried over: `--models list|key|grid`, now over the states a run lists;
`EIN_LEFTOVER`'s block in `ein-summary/2`; and `ein test --json-report`, now
`ein-test-report/5`, a query row carrying its classes, since M1e2
S1e2.3.10 every row what the universe leaves out, and since M1f S1f.6.27
the question its claim was read off. The read-out's
vocabulary is, since M1f S1f.5.16, the kernel page
[`docs/kernel/inference/read_out.md`](docs/kernel/inference/read_out.md),
every row `ein solve` prints and what it counts, and its golden page, written
by the binary, is
[`read_out_examples.md`](docs/kernel/inference/read_out_examples.md), both
held to the printer by `ein-cli/tests/read_out_reference.rs`; M1e2's two pages
under `plans/` are the record of the rulings of 2026-10-16.

**The `EIN_*` names in that block are the ones an agent needs, not the
set.** The shipped binary reads **two** since M1e2 S1e2.3.1 (three after
S1e2.1c.5, eight before), the test harness reads ten more, `utils/` and the shell two — and six
greppable `EIN_*` names are not environment variables at all, `EIN_RS` and
`EIN_RENDER_LEVI` among them. The whole census, classified, with the 17
`(config …)` fields and the 82 CLI options beside it and a *does it change the
answer* column on every flag row:
[`docs/kernel/configuration.md`](docs/kernel/configuration.md) (M1e S1e.5.1),
pinned by `ein-cli/tests/config_reference.rs`.

**`EIN_OBLIGATION_CHOICE`** was the obligations rung's measurement lever
(default `rule-order`): `fail-first` walked the owed instances smallest-set
first, and **`off` declined the rung altogether**, which is the pre-S1d.2.5
engine and the control arm every number in
[`hypotheses_from_obligations.md`](docs/history/m1d_satisfiability/hypotheses_from_obligations.md)
is measured against. It was deliberately not a `(config …)` field: `SolverConfig`
is rendered into the KB-shape digest, so a knob whose settings are being
compared would re-bless every shape golden in the corpus. **The user retired
it on 2026-11-03** (M1e2 S1e2.3.1): no route walks the ladder, the universe is
taken with the rung in report order, and once the summary's `root` block, its
last reader, became root's fixpoint and H, it changed nothing the binary
printed. The binary no longer reads it, and the rung's order stays pinned.

**`EIN_TRAVERSAL=tree`** was M1d
[S1d.10.6](docs/history/m1d_satisfiability/README.md#s1d106--the-traversal)'s
and a **second traversal beside the lattice**, off by default; it was deleted
with the walk at M1e2 S1e2.1c.5, and this paragraph and the next two are its
record. It branches on
**one owed instance's alternatives** — jointly exhaustive by the obligation's
meaning, so committing to one excludes its siblings with nothing to refute —
where the lattice enumerates subsets of a fixed `alive` and prunes only through
death. On `examples/zebra2-minus-15-obligations.ein` that is **86 enterings and
0.083 s** against the lattice's **17 204 592 and 1 496 s** for the same 32
models, verified fact for fact. It **declines** on any other rung, narrating a
`traversal` event: an hrule's candidates are not an owed instance's
alternatives, and branching on them is the `d!`-per-set depth-first solver P1.5b
deleted — 7 877 enterings against 101 on `zebra2.ein`, measured before the guard
existed. It reports `exhausted = false` on purpose (a tree terminates by
*discharge*, and the sentence saying what that licenses is T1d.10.5.1's), and it
is an environment variable rather than a flag because
[T1d.10.6.4](docs/history/m1d_satisfiability/README.md#s1d106--the-traversal)
has not decided what a tree reports where a lattice reports layers.

**What it does with the stop policy is settled since M1e S1e.2.1** ([CO-H3](docs/history/m1e_review_processing/review/correctness/high.md)),
and the two flags get different answers because they are different questions.
**`-n` is honoured**: `ein solve` defaults to `-n 1` and the tree used to
explore and record the *entire* tree while being asked for one model — 32 where
the lattice, same file, same flags, said 1. **`-m` is refused**, exit 2 with a
stderr `error:`, because `--max-set-size` bounds the size of the commitment
*sets* the lattice enumerates and a tree enumerates none: its depth is what the
program owes. Not a stylistic call — 6 of `zebra2-minus-15-obligations`'s 32
models sit at commitment size **6**, one past that flag's own default, so
reading it as a depth cap would have deleted a fifth of the headline at stock
settings. The same stage made a dead branch **record** its refutation (without
learning from it, so the search is byte-identical and the 86 holds) where the
`Contradiction` arm used to print *refuted so far (0 facts)* for a run that had
refuted real commitments, and made `tree_node` **re-read the rung mode at every
node** rather than trusting root's probe — the user's ruling of 2026-08-28,
whose regression test is owed to
[S1f.10.6](plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md)
because no corpus program can flip the mode. The headline was re-measured on
the way out: **86 enterings, 32 models, identical fact for fact** to the
lattice's 48 745.

**What it *reports* is written down since M1e S1e.3.2** (`SE-M3`), independent
of `T1d.10.6.4` and in two places: the shipped subset is
[`configuration.md` § What `EIN_TRAVERSAL=tree` reports](docs/kernel/configuration.md)
and the stream half is
[`events.md` § `traversal`](docs/kernel/inference/events.md). Three things an
agent should know before running one. **`exhausted` is `false` on every tree
run**, so `ein test` can never mark a claim `held` on a program the tree
accepts — every `:expect` comes back `NOT CHECKED` and the runner exits 1,
where the lattice holds the same claim. **`layers_explored` carries the deepest
node**, not a layer count. And **four event kinds are not emitted at all** —
`enter`, `layer`, `nogood`, `writeback` — so the enterings are invisible in a
stream that still counts them, and `Σ entered = enterings_total` holds for a
lattice run only.

**`--models {list,key}`** is M1d
[S1d.3.3](docs/history/m1d_satisfiability/README.md#s1d33--the-verdict)'s
and the only *lever* in this block that is a flag, because it is presentation
rather than measurement. `key` prints a model **set** as its determining key —
the smallest set of slots that tells the models apart, and the table of
combinations that occur — instead of as *k* blocks: on
`examples/zebra2-minus-15.ein -e -m 3` that is **49 lines against 516**, the
same 4 columns and 32 rows `utils/model_set_census.py --form key` prints. It is
read by the `Ambiguity` arm alone and reaches nothing recorded, and where a key
is unaffordable — `examples/branching/06_lookahead_on.ein` needed 8 of 42
slots, `C(42, 8) = 118 030 185`, untyped as it was until M1e2 S1e2.2.10
(`model_set_report.rs` keeps that program) — it says so and **prints the
models**, which is the form the phase priced as (e). `list` stays the default
because it is what `ein solve` has always printed, not because the key is
expensive — 42 ms to decline on that program, under 1 % of the solve on the
zebra family.

Beside it, and not a flag: **an `Ambiguity` now qualifies its own count.**
`exhausted = true` prints `solutions (k) 9`; `exhausted = false` prints
`solutions (k) 5   (a lower bound — the search did not exhaust)` and
*"distinct complete models **found**"*. It matters because the corpus was full
of the case — `ein solve -e examples/saturation/type-exclusivity/colors.ein`
printed **5** for a file with **9** models, and 5 of the 10 entries that answer
`Ambiguity` under their declared runs do it unexhausted. `Contradiction` was
left alone by that stage and changed by another. Since M1d T1d.10.5.2b a
`k = 0` run that did not exhaust says *No model found — the search did not
exhaust the lattice*, not *the constraints are contradictory*.
`saturation/type-exclusivity/pets.ein` is the fixture: `k = 0` at
`-m 5`…`-m 8`, **35 models** at `-m 10`, and **39**, exhausted, at `-m 12`
(re-taken 2026-10-14). Two surfaces said the old word, the `--trace`
headline and `--json-summary`'s `verdict.type`, until M1e2 S1e2.1a.5's switch
(2026-10-18) removed both: they were S2 and S3 of
[P1e2.3](plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/README.md),
which found them gone at its re-take and left no surface live (2026-11-03).
And whether a
truncated refutation may be called anything stronger is still
[Q-M1d.1](docs/history/m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)'s
question about a *word*.

**`EIN_LEFTOVER=1`** is its neighbour and M1d S1d.3.1's probe: it fills
`--json-summary`'s **`leftover`** block with what the **blind** enumerator
would still propose at each recorded model or open state. `complete` means
*the active rung proposes nothing*, so a node the hrule or obligations rung
called complete can still have facts a guess could be about — and their count
is what separates *one model* from *2ⁿ models* when the reading is open-world
(`zebra2`'s **unique** model leaves **8 203**, config-free since M1e2
S1e2.3.1; the **3 678** M1d measured was the count through the walk's
lookahead filter, until 2026-11-03). It runs on a discarded fork,
which is what makes it a read where
[P1d.2 declined one](docs/history/m1d_satisfiability/hypotheses_from_obligations.md):
with the lever on and off, every field of every summary outside that block is
identical on all 121 entries that reach a fixpoint. Off by default because it
costs a generation pass per listed state and every corpus `solve` in `cargo
test` writes a summary: ≈40 ms on the zebra family through the lookahead, and
≈3 ms on `zebra2` config-free (`ein solve --maximal examples/zebra2.ein`, best
of five with the variable and without, 99 against 102 ms, 2026-11-03).

**A failing `:expect` now says why on stderr too**, M1d
[S1d.4.3](docs/history/m1d_satisfiability/the_vocabulary.md),
and it is one line: `<file>: :expect NOT CHECKED — expected Ambiguity with k =
2, got Solution with k = 1`. stdout is **unchanged** — the report block stays
under the solution table, because a false claim is a *result* and not a refusal
of the input — but an exit 1 with an empty stderr is a run nobody can diagnose
from a pipeline, which is what
`corpus_cli::every_refusal_carries_a_diagnostic` forbids and what `ein solve`
was producing. It is why `examples/features/11_expect_ambiguity.ein` could not
declare plain `solve`; it declares it now, and that cell was the corpus's only
witness for `Outcome::NotChecked` until M1e S1e.3.1 gave
`examples/ein-bugs/complete-records-stale.ein` a claim a truncated search
cannot check. **P1d.4 closed with no keyword**: tests stay
exhaustive by default, `:expect` stays closed by default, and a claim too slow
to check at `ein test`'s `-m 5` stays out of the corpus — which needs no
mechanism, because `NOT CHECKED` takes a failing exit code inside `cargo test`.

**`ein test --json-report FILE.json`** is M1d
[S1d.4.1](docs/history/m1d_satisfiability/README.md#s1d41--what-closure-costs)'s
and the 50th CLI option: **one row per `(query …)` of the whole selection** —
the claim's shape (`model` / `or` / `false`), how many models it lists, the
relations its `:goal` closes, the outcome, and what the run found. It exists
because `:expect` had **no machine-readable surface at all**, so the only way
to ask the corpus what fraction of it claims a model set was a grep — and a
grep cannot tell a keyword from a comment about one, which is not hypothetical:
the stage's reconnaissance made exactly that mistake, on
`examples/features/10_expect.ein`. Additive in the strict sense (stdout,
stderr, exit code and *what is solved* identical with it and without, so a
query stating nothing is still never solved), and deliberately **not** under
the one-path rule that refuses `--json-summary` over several runs: a report has
no run to be more than one of, and one invocation over the three roots is the
whole census in 0.04 s. A file that did not load carries **no claim** —
`queries = 0`, `expect = null` — because a claim is a property of a *program*.

**`./run_tests.sh` runs every step of the per-commit CI tier**, in its order,
since M1c S1c.1.5 — **six** static checks (~5 s warm; `--tests-only` skips them
and the bench smoke), then `cargo test --workspace`, then the bench smoke test.
From M1f S1f.6.3 to S1f.6.21 one test was run a second time after the
workspace's, built with the work counters, which every other build compiles
out: `cargo test -p ein-infer --features counters --test arith_batch`, the
calculator's batch held to the matcher's own count of its work. The user
ruled it out of the gate on 2026-11-12 (*"we don't need this proportional
gate"*): the test and the feature stay, no tier runs them, and a stage that
changes a rule of the calculator runs the command by hand.
Since M1e2 S1e2.3.5 the gate checks its own scratch: the tests run with
`TMPDIR` set to a fresh directory under `build/`, and
`utils/check_test_scratch.py` fails the gate on anything they leave there,
naming the file that writes it. **`cargo test
--workspace` alone is not the gate**: it checks none of the six, and until
S1c.1.5 neither did this script, which is why CI was red for three commits on
findings the local run reported as a pass. A local gate that is a subset of the
remote one is a local gate that lies — keep the two lists the same.

| step | what it found the first time it ran |
|---|---|
| `utils/stdlib_manifest.py` | — (it has always run in CI) |
| `utils/check_hashmap_iteration.py` | one finding, and the finding was the *check*: only the line immediately above a statement counted as `determinism-ok:`, so the second line of a two-line reason silently un-annotated it. It reads the whole comment block now |
| `utils/doc_audit.py --links --check` | **264** — and 251 of them in `docs/history/m1d_satisfiability/`, a milestone record moved out of `plans/` with its links still aimed at the tree it left. Added M1e S1e.3.8 (`DO-M2`); the whole tree, not `docs/kernel/`, in 0.29 s |
| `cargo fmt --all --check` | **three** unformatted files, all from M1c's own `:expect` work |
| `cargo clippy --workspace --all-targets -D warnings` | a `for i in 0..n` indexing a slice, and **four** `&file` where `file` was already a `&str` — the four latent, because clippy stops at the first crate that fails and had never reached `ein-cli` |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | **twelve** unresolved intra-doc links and **seven** public items whose docs linked to a private one, plus a `<path>` read as an HTML tag. Nothing here had ever run rustdoc, whose default for all of it is a *warning* |

**`./run_tests.sh --nightly` runs the nightly**, since M1e2 S1e2.3.6: every
step of `.github/workflows/nightly.yml` instead of the gate's, its three jobs
one after another (`deep-corpus`, the fuzzer once per seed, the bench smoke),
so the nightly can be run before a push. Nothing local ran those steps before,
and the nightly was red from 2026-10-17 to 11-03 behind a green gate.
`ein-cli/tests/nightly_steps.rs` diffs its list against the workflow's, as
`gate_steps.rs` does the gate's against `per-commit.yml`, so a step added to
one and not the other fails `cargo test`. Its test steps run under a checked
`TMPDIR`, as the gate's do. It took 9 min 50 s here on 2026-11-03, half of
it `stdlib_census.py`; the runner's `deep-corpus` alone took 18 min that day.
Since M1f S1f.5.13 `deep-corpus` also runs `answer_round_trip` under
`EIN_CORPUS_SLOW=1`, which the per-commit tier does not set: every corpus
program's `--dump-answer` reloaded by value, the `slow` entries' and the five
`heavy_dump` ones' too, whose dumps write 40 000 state files or more
([`corpus/README.md` § `heavy_dump`](corpus/README.md#heavy_dump)); 121 s in
release on 2026-11-09. No CI tier had run that test's slow half before.

**The nightly holds only what guards an answer or a contract no per-commit
step guards**, each step within 5 min on the runner, since M1f S1f.5.27 (the
user's ruling of 2026-11-10, *"reduce nightly job, remove long running tests,
we will eventually run them on dev host, retain critical tests in CI"*;
[Q-M1f.102](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f102--what-the-ci-nightly-keeps-and-what-the-dev-host-runs)).
It keeps the slow corpus entries' cells, the round trip's, the levers' and the
cores' slow halves, `region`'s and `obligation_rung`'s slow halves (no tier
ran those two before,
[Q-M1f.51](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs)),
the mutation census and the bench smoke, and runs the fuzzer at 20 000 inputs
a seed, where a million had taken up to 4 410 s a seed on the runner since
the parser swap (run 42802597417, 2026-11-10;
[Q-M1f.103](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f103--the-fuzzers-budget-in-the-ci-nightly)).
**`./run_tests.sh --dev-host` is the tier nothing schedules**, which the user
runs here or in the runner's image (`utils/ci_container.sh --dev-host`):
every nightly step at full budget, the fuzzer at a million a seed, and the
steps that left or never ran in CI, the id space permuted eight ways,
`route_invariance`'s slow half, `stdlib_census.py --check` and the whole
review bundle with its link check. `nightly_steps.rs` holds its list to the
nightly's: every nightly step is in it, and every other is named in
`DEV_HOST_ONLY` with why, so a step that leaves the nightly and is not named
there fails `cargo test`. The run times are in
[S1f.5.27's records](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.27_the_nightly_reduced.md#t2-record-2026-11-10).

A reference that cannot be a link is still fine as `` `code` ``, and that is
the fix for most of the rustdoc ones: `ein-ir` cannot link into `ein-infer`
(the dependency runs the other way), a `#[cfg(test)]` module is not in the
documented crate, and a private item is a real name that public docs may cite
but not hyperlink.

Everything runs one engine. `cargo test --workspace` is the gate — the corpus
sweep through the CLI, the shape digests, the goldens, the manifest's own
invariants, and since M1c S1c.1.5 the **stdlib conformance** pair — and it
needs **Graphviz** on `PATH`, because `dot_wellformed.rs` is the only authority
the DOT views have on being well-formed and it fails rather than skips without
it.

**Which Graphviz build is part of what that test checks** (M1f S1f.5.17). The
CI runs on whatever `ubuntu-latest` names, and since 2026-11-06 that is Ubuntu
26.04, whose runner installs the **x86-64-v3 build** of every package. That
build of Graphviz 14.1.2 compiles `libpathplan`'s orientation test into a
fused multiply-add, and its spline router refused 20 corpus views that the
same version's baseline build, 2.42.2 and 16.0.0 all draw, until the overlay
and Levi digraphs set `ranksep=0.55` (`dot_util::POLYGON_RANKSEP`). A green
gate here says nothing about that build. **`utils/ci_container.sh`** runs
`./run_tests.sh`, or `--nightly`, in a container built like the runner, on
HEAD as committed, so an image move can be checked before a push. Both tiers
were green there at `76ede241`: the gate in 13 min 19 s and the nightly in
26 min 27 s (2026-11-08).

**When `utils/ci_container.sh` is part of done** (M1f S1f.5.29,
[Q-M1f.77](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f77--when-is-utilsci_containersh-part-of-done)).
A stage that changes a DOT writer, the review bundle or what CI installs is
not done until `utils/ci_container.sh` is green on its commit. Twice a
Graphviz refusal reached CI behind a green gate here: S1f.5.17's router, and
S1f.5.29's `fdp`, which 26.04's `graphviz` links to `dot` with no layout
plugin behind it, so every KB view was refused. The workflows and the image
install `libgvplugin-neato-layout8`, `run_tests.sh` exits 127 where `fdp`
lays out no graph, and a refused DOT fails the review bundle, the smoke's and
the nightly's alike, with Graphviz's message under its path.

**The stdlib conformance gate** is `ein-infer/tests/stdlib_coverage.rs`, and
what it costs is 0.04 s:

```sh
cargo test … -p ein-infer --test stdlib_coverage   # every rule, every program: fires + owes
ein test tests/                                    # the same suite, as a status code
```

It loads `stdlib/*.ein` for its rule heads, describes R of every
`tests/stdlib/` program that states a claim with `--events` on, and fails on
any rule no program activated — so **adding a rule to the stdlib without a test fails the
gate**, the way a `.ein` file with no corpus entry does. Its sibling fails on a
program under `tests/` that states no `:expect`. It is scoped to `tests/`
rather than to the corpus on purpose: a rule that fires only inside
`examples/zebra.ein` has no test, and 20 of them were in that state before
S1c.1.4. `utils/stdlib_census.py` is the same census with the *numbers*
(37 s, all 180 entries) and is not the gate.

The `ein` binary links `snmalloc` by default since S1a.6.2 (worth 8–16 % of a
solve), so the build needs **`cmake` and a C++ compiler**;
`cargo build --release -p ein-cli --no-default-features` builds against the
system allocator and needs neither.

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
