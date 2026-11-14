# AGENTS.md

Guidance for AI coding agents working in this repo.

## What this project is

Ein is a graph-based reasoner for Zebra-style logic puzzles. The
2021 prototype is being modernised in light of neuro-symbolic /
constrained-reasoning research.

## Where things live

- **`docs/kernel/`** — **canonical M1 kernel documentation**: graph
  semantics (`ir/01-ein-graph/`), the data model (`ir/02-data-model/`),
  surface S-expression language (`ir/03-ein-lang/`), inference engine
  (`inference/`). Start here for any "what does Ein reason about / how"
  question. See [`docs/kernel/README.md`](docs/kernel/README.md) for orientation.
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
  **This tree is the only statement of intent that is
  not also the implementation**, so it is load-bearing: a claim here is
  checked by `cargo test --workspace` and by nothing else — though **not every
  sentence on a page is such a claim**, and since M1e
  [Q-M1e.4](docs/history/m1e_review_processing/open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)
  a reader can tell which are: an exact number carries the **command** that
  re-takes it, the **date** it was taken, or the **test** that pins it, and a
  number that can carry none is not written down.
  What it takes to call a claim *settled* is
  [`standard_of_proof.md`](docs/kernel/standard_of_proof.md): a behaviour is
  refuted only by a banked probe, an absence by naming the thing that checks
  it, a **risk not by argument at all**, and an argument for leaving something
  alone holds only while its premise is enforced by something that fails when
  it stops being true.
  Every page is in exactly one of three declared states, visible from the
  page: *current*, *superseded with a banner*, or *moved to `docs/history/`*
  ([README § Which pages to trust](docs/kernel/README.md)). **A page is moved
  *into* an existing milestone record, never made into one.** **Do not
  rewrite a superseded page to match today's engine**: it is neither a record
  nor a specification afterwards, and `docs/history/m1a_rust/design/07` cites
  `inference/algorithm_layer_n.md` as the search layer's contract. [README §
  Keeping this true](docs/kernel/README.md) is the pass that keeps the
  current pages honest — six checks, five of them `utils/doc_audit.py` and
  the sixth a shell, and the sixth is the one that found the most.
- **`docs/api/`** — **how to drive Ein as a library**: [`rust.md`](docs/api/rust.md),
  the **crates** — `ein-ir` to load, `ein-infer` to solve, `ein-render` to
  explain, `ein-einb` to cache. **Its worked example is a test.** The page's
  `rust` block is the marked region of
  [`ein-cli/tests/embedding.rs`](ein.rs/crates/ein-cli/tests/embedding.rs) and
  a test in that file diffs the two, so `cargo test --workspace` keeps the
  page true — *edit the test, run it, paste; never edit the block by hand*.
- **`docs/install.md`** — the install page: the two channels (a release
  binary, `cargo install --path`), what `ein --version`'s five lines mean, how
  to check a binary's stdlib against the checkout's manifest, and
  `$EIN_STDLIB`. **`pip install` is not a channel.**
- **`docs/guide/`** — **the newcomer tutorial** ("Learn Ein by solving the
  Zebra puzzle"). User-facing; references `docs/kernel/` + `docs/api/`, never
  explains internals; complements `inference/zebra_walkthrough.md`.
- **`docs/history/`** — **shipped milestones, kept as record**
  ([`docs/history/README.md`](docs/history/README.md)): a document goes there,
  rather than staying only in git, when it is still read, as a specification,
  as evidence, or as the reason something is the way it is; a shipped
  milestone's plan folder is deleted and stays in
  `git log --diff-filter=D -- plans/<milestone>`. **Read
  [M1e's](docs/history/m1e_review_processing/README.md) § What this milestone
  may claim before citing it**, and M1d's P1d.10 § Eight measurements with no
  owner before re-opening any of them;
  [`m1d_satisfiability/ideas.md`](docs/history/m1d_satisfiability/ideas.md) is
  authoritative on intent, as `plans/ideas/*` is.
- **`docs/lib/`** — catalogue of external tech relevant to the rewrite
  (LLM constrained generation, CSP/SAT/SMT, theorem proving, category
  theory, graphs & rewrite systems, reasoning benchmarks, …), and a
  knowledge graph (`knowledge-graph.dot` → SVGs and a Cytoscape.js page).
- **`plans/ideas/`** — the user's *own* ideas; each preserves user quotes and
  open questions. Authoritative on intent.
- **`examples/`** — encoded Zebra puzzles (`zebra.ein` classic,
  `zebra2.ein` unified-is-a / `*-loc`, `zebra2-hints.ein` partial-state
  fixture) plus focused per-feature fixtures.
  [`examples/README.md`](examples/README.md) is a **catalog** — one line
  per file / sub-dir. **Four of the zebra files are generated**, by
  `examples/gen_zebra2_variants.py` from `zebra2.ein`, and `--check` is in the
  gate: `zebra2-minus-15` (clue dropped), `ein-bugs/zebra2-bad` (clue added),
  and [`zebra2-obligations.ein`](examples/zebra2-obligations.ein) and its
  `-minus-15` twin, which are `zebra2` with the `(hrule guess …)` and the
  `(solve … :hrules …)` clause **deleted and nothing else**.
- **`docs/kernel/inference/zebra_walkthrough.md`** — the Wikipedia human
  Zebra walkthrough annotated as ein inference: the **M1 target trace** for
  the engine and the **M2 target** for the NL ⇄ IR round-trip.
- **`stdlib/`** — the ein-lang standard library (`std.*`), the single source
  of truth. `MANIFEST.sha256` is what identifies a directory as the stdlib and
  what CI checks for drift; `ein-ir` embeds a copy with `include_dir!`, so the
  manifest is a build dependency and cannot go stale. Resolution: `$EIN_STDLIB`
  → the checkout → the embedded copy. **What tests it is `tests/stdlib/`**
  (below), and adding a rule here means adding a program there. Four of its
  rules are **obligations** — `total-owed` / `surjective-owed` in
  `std.algebra`, `slot-owed-room` / `slot-owed-fill` in `std.slots` — which
  assert `(open ?R)`, derive nothing, are not in the saturation agenda, and
  are read by one pass over the quiescent KB. Hypothesis generation is a
  *ladder* — the puzzle's `(hrule …)` if it declares one, else the facts that
  would discharge what the state owes
  ([`oblgen.rs`](ein.rs/crates/ein-infer/src/oblgen.rs)), else the blind
  combinatorial enumerator. **The calculator's modules are under
  `stdlib/math/`**, `std.math.x` in `math/x.ein` (a dot a directory, at any
  depth, from all three places), their programs under `tests/stdlib/math/`,
  each with an entry name a program may list beside the names it writes:
  `std.math.digits` (the digit tables, a library of *facts*, generated),
  `std.math.decimal-arith` (`(import std.math.decimal-arith :symbols
  (decimal-arith add n sum-of))`), `std.math.num9` (generated, entry
  `arith9`), `std.math.int` (`int-arith`) and `std.math.rat` (`rat-arith`); a
  module over a module names the lower one's entry in its own closing rule,
  and lists every name of the lower one it writes. **Boolean functions and
  circuits are under `stdlib/bool/`**, their programs under
  `tests/stdlib/bool/`, each module's entry its last name: `std.bool.gates`
  (the sixteen functions of two inputs, a table `(gate2 NAME A B OUT)` whose
  rows are generated and a relation each, `(And A B OUT)`), `std.bool.props`
  (thirty relations known of them, derived by rules), `std.bool.circuit` and
  `std.bool.inputs` (a circuit's inputs as hypotheses); a function's name is
  `std.bool.gates`', as a relation and as a value, so a program that writes
  one lists it. `std.logic` is left free for M1g.
  Their rules are in [§ Edits that go together](#edits-that-go-together), and
  [`stdlib/README.md`](stdlib/README.md) has a section for each.
- **`tests/`** — **the ein-lang test suites**, a third corpus root beside
  `examples/` and `stdlib/`: a `.ein` here with no `corpus.toml` entry fails
  the same completeness check. Its one subject is
  [`tests/stdlib/`](tests/README.md) — **one program per stdlib rule or tight
  family**, each carrying an `:expect`, so `ein test tests/` is the whole
  suite. They are deliberately *not* under `examples/`: that directory is
  things to read, and these exist to break. Idioms worth knowing before
  writing a new one — **naming a relation closes it** (which is how a rule
  with no guard gets a negative case at all), **`(unknown …)` as a probe**
  (the only way to say a rule did *not* derive a negative, since stored
  negatives are not closed), **a refutation rule gets two files** — one where
  it fires and one where it is loaded, activated and satisfied — **where two
  rules reach one verdict, separate them by activation**, because an
  expectation made of facts cannot say which one got there, and **an
  obligation rule derives nothing**, so its fixture carries an ordinary
  `(model …)` claim, its owe counts are asserted in
  `ein-infer/tests/obligation_reports.rs`, and `(solve … :expect (region
  (owing (model …))))` says that a state owes, though not how much.
  **Activation is two events**: `fire` for a saturation rule and `owe` for an
  obligation one. The coverage claim is measured by `utils/stdlib_census.py`,
  never by reading the directory, and the suite's sensitivity by
  `utils/stdlib_mutants.py`, whose survivors are banked as a set in
  `tests/mutation_survivors.txt`.
- **`corpus/`** — the **corpus**: `corpus.toml` (one entry per `.ein`, with
  the runs it is exercised under) plus `fuzz_findings/`, and
  `wrong_answers.toml`, the register of every known wrong answer. The
  register has one row per witness, with today's answer and the right one as
  data, and `ein-cli/tests/wrong_answers.rs` holds an `open` row to its wrong
  answer and a `fixed` one to its right one. A file under `examples/`,
  `stdlib/` or `tests/` with no entry fails a completeness check. What runs it
  is `cargo test`, and [`corpus/README.md`](corpus/README.md) has the table of
  readers; the `--events` protocol is
  [`docs/kernel/inference/events.md`](docs/kernel/inference/events.md).
  **`slow = true` is a measured claim** — an entry whose declared runs cost
  1 s or more together, recorded in `cost_ms` and checked in both directions
  (`utils/bench_env.sh --cores PT python3 utils/corpus_cost.py` re-takes it;
  `corpus_cli`'s `the_slow_set_is_exactly_these_fifteen` names the set). The
  default `--region-budget` is 2²⁰ facts, which the user ruled after 2²⁴ ran
  the gate's sweeps out of memory. **A run is dropped from a `runs` column
  only when it does not ask the fixture's question**, never for costing too
  much.
- **`ein.rs/`** — the engine. Two of its eight crates are dev-only:
  `ein-corpus` (the manifest, the fixture helpers, the bench set) and
  `ein-narration` (the one definition of what counts as a derivation's
  *narration* rather than its content: a fork resumes root's saturation, so
  two runs can narrate different amounts of one derivation and reach the same
  answer; `EIN_NARRATION_STRICT=1` turns the cut off). The six that ship are
  `ein-core` (interning, `Value`/`FactId`, the layered COW KB, provenance),
  `ein-ir` (the parser grmtools generates from `src/ein.l` and `ein.y` →
  macros → imports → load), `ein-infer` (compile → match → saturate → the NAF
  boundary → the routes over R; `invariant.rs`, the **alive-set invariant** —
  *no rule asserts a name the program lacks* — whose list of the program's
  names seeds the blind enumerator's objects), **`ein-einb`** (the `.einb`
  binary KB container, and the **only library crate that is not
  `#![forbid(unsafe_code)]`**: its `cast.rs` is the one audited module
  design/12 §2 permits `unsafe` in; the other audited `unsafe` is
  `ein-cli/src/main.rs`'s one `libc::signal(SIGPIPE, SIG_DFL)`, so a closed
  stdout ends `ein` as it ends `cat`), `ein-render` (DOT views, the read-out,
  the region trace, the answer and lattice dumps, the JSON summary) and
  `ein-cli`. They stack linearly up to `ein-infer` and fork there —
  `ein-einb` and `ein-render` are siblings above it, and `ein-cli` depends on
  both. `ein-cli` fronts **five** subcommands — `ein
  {oracle,render,saturate,solve,test}` — plus `ein kb` under the `einb`
  feature.

  **`.einb` is a private cache format, never an interchange one.** `ein kb save
  <file.ein> <out.einb>` writes one and every command that takes a `.ein` path
  takes a `.einb` too, dispatching on the magic bytes rather than the
  extension; `ein solve x.einb` is byte-identical to `ein solve x.ein` apart
  from the path it echoes. Anything crossing a tool boundary is still `.ein`
  text or the event protocol's JSON.
- **`utils/`** — the scripts; [`utils/README.md`](utils/README.md) is the
  catalogue — one line per script in three groups (*renderers*, *checks*,
  **the M1a measurement set**). Every script that runs the engine names the
  binary — **`$EIN_BIN`** or `--bin` — defaulting to
  `ein.rs/target/release/ein`, except the one that wants a build of its own
  (`profile_ein_rs.py` → `--profile profiling`, which it builds). Run every
  measurement through **`bench_env.sh`**, which prints the machine state the
  numbers were taken under.
- **`build.sh`** — **everything this repo builds, in one command**: the Rust
  workspace (`--release` by default, into `ein.rs/target/`) and then the three
  C baselines in `c/` (into the gitignored `build/`). `--debug`,
  `--no-snmalloc` (the system allocator, for a machine without `cmake` and a
  C++ compiler), `--all-targets` (tests, benches and the measurement
  examples), `--engine` / `--c` for one of the two. It builds and does not
  run: `./run_tests.sh` is the gate.
- **`c/`** — **three plain-C Zebra baselines**, the puzzle `examples/zebra.ein`
  encodes with exactly one thing varying, how much the search is told about
  the constraints. Wired into nothing: the gate does not run them and no crate
  depends on them. [`c/README.md`](c/README.md) is the catalogue and carries
  the argument.
- **`nl2smt/`** — **the Zebra puzzle's sentences through link grammar to
  SMT**, a proof of concept ([M1j](plans/m1j_nlp_smt_poc/README.md)), wired
  into nothing: no crate depends on it and the gate does not run it.
  [`nl2smt/README.md`](nl2smt/README.md) is the page, with the command and
  its tests, which skip where `link-parser`, `racket` or every solver is
  missing. `nl2smt/smt/` holds three hand-written `.smt` encodings that
  [M10](plans/m10_external_benchmarks/README.md) counts as part of its
  benchmark corpus, so their content does not change; `nl2smt/prototype/`
  holds the two scripts
  [M2 S2.6.4](plans/m2_nl_to_ir/p2.6_ablations/s2.6.4_representation_ablations.md)
  starts from.

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
- **A rule concludes a conjunction of literals** — facts and stored
  negations, with `(false)` and checks among them — or `()`, or an obligation
  as its whole `:assert`. An `(and …)` is read flat at any depth, by every
  reader through one function, `ein_ir::Ast::conjuncts`. No disjunction and
  no compound fact: *one of* is the hypothesis layer's. **An hrule's
  conclusion is one fact**; one of more is refused.
- A query's `:goal` is held to what a rule's `:match` is held to. Every name
  in a `(solve …)` list names something the program has. A macro's call is no
  fact.
- **A form headed by a declared relation has as many arguments as its
  declaration has types.** It is no type check. A fixture that wants a
  relation at several counts leaves it undeclared.
- **Every import brings its module whole**, with its own imports, one copy
  per program, every declared name under its **full name**,
  `std.math.decimal-arith.add`; the three shapes differ only in names: `:symbols (…)` lets the file write
  the listed names short, `:as a` writes `a.add`, a bare import writes full
  names. **A file lists every short name it writes** from the module that
  declares it (no re-export); one it writes and does not list is its own. One
  short name bound to two full names in one file is refused. A module's name
  is a path **from the program file's directory**, for a module's own imports
  too; `..` is refused. A state prints the main file's spelling, else the full
  name. **The answer does not depend on the import form**
  (`ein-cli/tests/import_invariance.rs`, every corpus program selective
  against whole against `:as`). **No import brings a module's `(query …)`,
  `(solve …)` or `(config …)`**: those are a program's.
- *Forbid*: a program whose refutation could rest on an `absent` over a
  relation a commitment can still extend is refused (`ein_infer::forbid`).
- **The hypothesis mechanisms do not contradict one another**: an hrule, an
  obligation, `:hypothesis-relations`, `:no-hypothesis` and `(__closed__ R)`,
  read on root's fixpoint. No active hrule proposes a relation a list or a
  closure takes out, no obligation owes one (a list only with no hrule), and no
  relation is in both lists or listed and closed; a list an hrule leaves
  filtering nothing, or a redundant name, is a `note:` (`ein_infer::mechanisms`).
  **`(solve :no-search)`**, a flag with no value, states that the lattice is
  root alone: nothing is proposed, so R is {∅} or empty, and it is how a program
  says *judge root alone*. It relaxes no conflict; beside an hrule or a list it
  is a `note:`, beside an obligation or a closure silent.

### The answer

- **The answer is a region R**, the commitments whose state is alive and
  discharged. A bare `ein solve` describes R; `--minimal`, `--maximal`,
  `--single`, `-n N` and `--count` each do the least work their question
  needs; **`ein oracle` is the reference every mode is held to**
  (`ein-infer/tests/explore.rs`). No two commands may disagree.
  **`--no-search`** (`ein solve`, `ein test`) declines to discover R: it
  reports root as the full run reaches it, states no R, and decides only the
  claims root decides.
- **Every answer exits 0.** Only a refuted claim exits 1; a load error takes
  each command's load-error exit (2 under `ein test`). A count of commitments
  is exact in the engine and reported up to 2^63 − 1, which every surface
  reads as *this many or more* (`ein_infer::count`).
- **No config option, lever or flag changes R**, held per commit by
  `ein-cli/tests/lever_invariance.rs`, which holds `--no-search` too: it
  declines to search, and reports no other R. The one exemption:
  `:record-alternative-justifications false` may change a dead node's core.
- **A core is an explanation**: it may vary with rule order, and F22 makes it
  canonical. A node that is not dead has no core (`null`); `[]` is the empty
  core of a dead node. A core is taken only where a surface prints one.
- `ein test` asks a file **the least question that can decide the claims
  it states**, read off their shapes: `--single` or `-n k+1` for queries'
  claims all over one part of R, describe for a `(solve … :expect …)` or
  for claims over more than one part; what that leaves undecided it asks the
  describing question, of the program loaded again. No option chooses the
  question; `--no-search` asks none about R, root alone, and what root does
  not decide is NOT CHECKED with no exit code. A claim is held only by an answer that lists all of what it is
  about, and under `ein test` alone a state found whose class the claim does
  not list refutes it (`ein-infer/tests/claim_routes.rs` holds each question
  to the description and to the oracle). It never solves a query with no
  `:expect`, and reads every claim of a file off those one or two runs.
- Every command reads a program by one path (`ein-cli`'s
  `common::read_program`), `.ein` or `.einb` alike.

### Edits that go together

- **Generated, never edited by hand** (edit the generator and re-run):
  `00_ebnf.md`'s two grammar blocks (from `ein.l` and `ein.y`, `EIN_BLESS=1`);
  `stdlib/math/digits.ein` (`utils/gen_arith_tables.py`, then
  `utils/stdlib_manifest.py --write`); `stdlib/math/num9.ein`
  (`utils/gen_num9.py`); `stdlib/bool/gates.ein`'s rows and the programs
  under `tests/stdlib/bool/gates/` and `props/` (`utils/gen_gates.py`, then
  `utils/stdlib_manifest.py --write`);
  three `examples/arith/` programs (`utils/gen_arith_examples.py`); the four
  zebra2 variants (`examples/gen_zebra2_variants.py`);
  `read_out_examples.md` (`utils/read_out_examples.py`); `configuration.md`'s
  defaults block and `docs/api/rust.md`'s `rust` block. All of those have a
  `--check` or a test in the gate. **Not in the gate, run by hand**:
  `examples/binary_arithmetic/gen_binary.py` (after any edit to
  `stdlib/math/decimal-arith.ein`).
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
  or `stdlib_coverage.rs` fails. A rule or relation added to
  `std.math.decimal-arith` (or to a module over it) goes into its closing
  rule's two lists (`arith_theory.rs`); a new operation is a row of `OPS`,
  and joins `arith_search_free.rs` by its lists. **No rule of the calculator
  joins two facts on a number.** `std.math.decimal-arith` reads **one
  `absent`, over `let`**, which no rule may derive, and writes guards in three named rules
  (`arith_theory.rs`); a program it flags, `(malformed …)` or
  `(undeclared …)`, is dead and claims only `(solve :expect (false))`.
- **A stage that changes a rule of the calculator runs the batch by hand**:
  `cargo test -p ein-infer --features counters --test arith_batch`. No tier
  runs it.
- `dot_wellformed.rs` skips a view of more than 1 000 edges; a program with
  such a view in a directory not named in its `SKIPPING` fails the sweep.
- **A flagged corpus program in which a hypothesis leads to a request to
  `std.math.decimal-arith` through a rule with an unbound operand fails
  `region.rs`'s fact-grain test**: the prototype's closure gives up past 10 000 shapes and
  the test names the program rather than read a narrower verdict (E-33,
  Q-M1f.225). An unflagged one costs it nothing: the closure runs only where
  a site reads it.
- A local module's name is a path from the **program** file's directory, for
  a module's own imports too: no import reaches a sibling directory (E-32),
  and a module that imports a neighbour is found from the program's
  directory, not its own.

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

The read-out's vocabulary is
[`docs/kernel/inference/read_out.md`](docs/kernel/inference/read_out.md),
every row `ein solve` prints and what it counts, and its golden page, written
by the binary, is
[`read_out_examples.md`](docs/kernel/inference/read_out_examples.md), both
held to the printer by `ein-cli/tests/read_out_reference.rs`. **The `EIN_*`
names in the block above are the ones an agent needs, not the set**:
[`docs/kernel/configuration.md`](docs/kernel/configuration.md) is the census
of every `EIN_*` name, `(config …)` field and CLI option, with a *does it
change the answer* column on every flag row, pinned by
`ein-cli/tests/config_reference.rs`. The walk — `-e`, `-m`,
`EIN_TRAVERSAL`, `EIN_OBLIGATION_CHOICE`, the lattice's verdict wording — was
deleted at M1e2 S1e2.1c.5; a page that describes it is a record.

**`./run_tests.sh` runs every step of the per-commit CI tier**, in its order:
six static checks (`--tests-only` skips them and the bench smoke), then
`cargo test --workspace`, then the bench smoke test. The tests run with
`TMPDIR` set to a fresh directory under `build/`, and
`utils/check_test_scratch.py` fails the gate on anything they leave there,
naming the file that writes it. **`cargo test --workspace` alone is not the
gate**: it checks none of the six. A local gate that is a subset of the
remote one is a local gate that lies — keep the two lists the same;
`ein-cli/tests/gate_steps.rs` diffs them.

**`./run_tests.sh --nightly` runs the nightly** instead of the gate: every
step of `.github/workflows/nightly.yml`, its three jobs one after another
(`deep-corpus`, the fuzzer once per seed, the bench smoke), so the nightly
can be run before a push; `ein-cli/tests/nightly_steps.rs` diffs its list
against the workflow's. **The nightly holds only what guards an answer or a
contract no per-commit step guards**, each step within 5 min on the runner
(the user's ruling of 2026-11-10;
[Q-M1f.102](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f102--what-the-ci-nightly-keeps-and-what-the-dev-host-runs)).
**`./run_tests.sh --dev-host` is the tier nothing schedules**, which the user
runs here or in the runner's image (`utils/ci_container.sh --dev-host`):
every nightly step at full budget and the steps that left or never ran in
CI. A step that leaves the nightly is named in `nightly_steps.rs`'s
`DEV_HOST_ONLY` with why, or `cargo test` fails.

A reference that cannot be a link is still fine as `` `code` ``, and that is
the fix for most of the rustdoc ones: `ein-ir` cannot link into `ein-infer`
(the dependency runs the other way), a `#[cfg(test)]` module is not in the
documented crate, and a private item is a real name that public docs may cite
but not hyperlink.

Everything runs one engine. `cargo test --workspace` is the gate — the corpus
sweep through the CLI, the shape digests, the goldens, the manifest's own
invariants, and the **stdlib conformance** pair — and it needs **Graphviz** on
`PATH`, because `dot_wellformed.rs` is the only authority the DOT views have
on being well-formed and it fails rather than skips without it.

**Which Graphviz build is part of what that test checks** (M1f S1f.5.17). The
CI runs on whatever `ubuntu-latest` names, and since 2026-11-06 that is Ubuntu
26.04, whose runner installs the **x86-64-v3 build** of every package. That
build of Graphviz 14.1.2 compiles `libpathplan`'s orientation test into a
fused multiply-add, and its spline router refused 20 corpus views that the
same version's baseline build, 2.42.2 and 16.0.0 all draw, until the overlay
and Levi digraphs set `ranksep=0.55` (`dot_util::POLYGON_RANKSEP`). A green
gate here says nothing about that build. **`utils/ci_container.sh`** runs
`./run_tests.sh`, or `--nightly`, in a container built like the runner, on
HEAD as committed, so an image move can be checked before a push.

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

**The stdlib conformance gate** is `ein-infer/tests/stdlib_coverage.rs`
(`cargo test … -p ein-infer --test stdlib_coverage`; `ein test tests/` is the
same suite as a status code). It describes R of every `tests/stdlib/` program
that states a claim with `--events` on, and fails on any stdlib rule no
program activated — so **adding a rule to the stdlib without a test fails the
gate**, the way a `.ein` file with no corpus entry does. Its sibling fails on
a program under `tests/` that states no `:expect`. It is scoped to `tests/`
rather than to the corpus on purpose: a rule that fires only inside
`examples/zebra.ein` has no test. `utils/stdlib_census.py` is the same census
with the *numbers*, and is not the gate.

The `ein` binary links `snmalloc` by default (worth 8–16 % of a solve), so
the build needs **`cmake` and a C++ compiler**;
`cargo build --release -p ein-cli --no-default-features` builds against the
system allocator and needs neither.

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
