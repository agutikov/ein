# AGENTS.md

Guidance for AI coding agents working in this repo.

## What this project is

Ein is a graph-based reasoner for Zebra-style logic puzzles. The
2021 prototype is being modernised in light of neuro-symbolic /
constrained-reasoning research.

## Where things live

- **`docs/ein/`** — **the documentation of Ein as it ships**, in three
  perspectives ([`docs/ein/README.md`](docs/ein/README.md) is the index):
  **`user/`**, the black box — the language, the command line, what the
  answer means and every row `ein solve` prints, the guide;
  **`reasoning/`**, what is computed and why it is right, true of any
  implementation (`static-ir/`, `kb-evolution/`, `rule-evolution/`); and
  **`ein.rs/`**, how this implementation computes it (below). Start here for
  any "what does Ein reason about / how" question. The pages an agent most
  often needs:
  [`diagnostics.md`](docs/ein/user/reference/diagnostics.md#2-errors-and-exit-codes),
  the parse and load diagnostics and the load refusals the engine defines
  normatively (§ 2 has one section per refusal);
  [`solution-semantics.md`](docs/ein/reasoning/kb-evolution/solution-semantics.md),
  the answer as a region R;
  [`read-out.md`](docs/ein/user/reference/read-out.md), every row
  `ein solve` prints and what it counts;
  [`hypothesis-space.md`](docs/ein/reasoning/kb-evolution/hypothesis-space.md),
  the hypothesis space and its structure, each object defined, and
  `h-space.json`;
  [`config.md`](docs/ein/user/reference/config.md), the `(config …)` flags,
  the CLI options and the `EIN_*` names, each with a *does it change the
  answer* column;
  [`syntax.md`](docs/ein/user/reference/syntax.md) and
  [`grammar.md`](docs/ein/user/reference/grammar.md), the language;
  [`natural-language.md`](docs/ein/user/syntax-and-semantics/natural-language.md),
  `:why` texts and how a fact reads as English; and
  [`architecture.md`](docs/ein/ein.rs/architecture.md), the crate and module
  map, with the design contracts the code cites under `docs/ein/ein.rs/design/`.
  What is true of the language and the answer today is
  [§ Standing rules](#standing-rules) below; how each rule came to be is in
  the stage records under `plans/` and in `git log`, not here.
  **This tree is the only statement of intent that is
  not also the implementation**, so it is load-bearing: a claim here is
  checked by `cargo test --workspace` and by nothing else — though **not every
  sentence on a page is such a claim**, and a reader can tell which are: an
  exact number carries the **command** that re-takes it or the **test** that
  pins it, and a number that can carry neither is not written down
  ([Q-M1e.4](docs/decisions/docs/Q-M1e.4-an-exact-number-in-a-page-carries-the-command.md)).
  **It states the current state and nothing else**
  ([Q-M1f.341](docs/decisions/docs/Q-M1f.341-the-project-documents-only-its-current-state.md)):
  a page that stops being true is rewritten or deleted, never kept with a
  banner; no page names a plan, a stage or a date, and a page that needs a
  reason cites the decision that carries it.
  What it takes to call a claim *settled* is
  [`docs/ein/standard-of-proof.md`](docs/ein/standard-of-proof.md): a behaviour is
  refuted only by a banked probe, an absence by naming the thing that checks
  it, a **risk not by argument at all**, and an argument for leaving something
  alone holds only while its premise is enforced by something that fails when
  it stops being true.
  **What keeps the pages true**: `utils/doc_audit.py --links --check
  --tracked`, a gate step (every link and anchor, every prose `§` number
  written after a link, every decision id the project cites); its
  `--identifiers` report (every backticked name that looks like code, looked
  up in `ein.rs/crates/`: a report, never a gate, since a page that rightly
  says a name is gone still names it); the release-voice check; and running
  the commands a page shows, which no script does and which finds the most.
- **`docs/decisions/`** — **every question about Ein that has an answer, as
  that answer** ([`docs/decisions/README.md`](docs/decisions/README.md)): one
  file a decision, `<subject>/<id>-<slug>.md`, the final answer only, under
  the id of the question it answers (`Q-M1e2.24`, `Q-M1f.313`, `Q23`). A page
  links a decision's file; a code, doc or `.ein` comment and a corpus note
  cite it by its bare id, which the link check resolves to exactly one file.
  A question still open is a work item and lives in `plans/`, cited by
  nothing in the project; a decision about the process stays in `plans/` too.
- **`docs/ein/ein.rs/`** — **the engine as built**: the crate and module map,
  the design contracts the crates cite (`design/`), the `--events` protocol and
  the other engine read-outs, and **how to drive Ein as a library**,
  [`api.md`](docs/ein/ein.rs/api.md): the **crates** — `ein-ir` to load, `ein-infer` to solve, `ein-render` to
  explain, `ein-einb` to cache. **Its worked example is a test.** The page's
  `rust` block is the marked region of
  [`ein-cli/tests/embedding.rs`](ein.rs/crates/ein-cli/tests/embedding.rs) and
  a test in that file diffs the two, so `cargo test --workspace` keeps the
  page true — *edit the test, run it, paste; never edit the block by hand*.
- **`docs/ein/user/install.md`** — the install page: the two channels (a release
  binary, `cargo install --path`), what `ein --version`'s five lines mean, how
  to check a binary's stdlib against the checkout's manifest, and
  `$EIN_STDLIB`. **`pip install` is not a channel.**
- **`docs/ein/user/guide/`** — **the newcomer tutorial** ("Learn Ein by solving the
  Zebra puzzle"). User-facing; links the reference and the concept pages
  under `docs/ein/user/`, never explains internals; its chapter 6 is
  `06_zebra_walkthrough.md`.
- **`docs/lib/`** — catalogue of external tech relevant to the rewrite
  (LLM constrained generation, CSP/SAT/SMT, theorem proving, category
  theory, graphs & rewrite systems, reasoning benchmarks, …), and a
  knowledge graph (`knowledge-graph.dot` → SVGs and a Cytoscape.js page).
- **`plans/ideas/`** — the user's *own* ideas; each preserves user quotes and
  open questions. Authoritative on intent; idea 11 is the note M1d's obligations
  were built on.
- **`examples/`** — encoded Zebra puzzles (`zebra.ein` classic,
  `zebra2.ein` unified-is-a / `*-loc`, `zebra2-hints.ein` partial-state
  fixture) plus focused per-feature fixtures.
  [`examples/README.md`](examples/README.md) is a **catalog** — one line
  per file / sub-dir. **Four of the zebra files are generated**, by
  `utils/gen_zebra2_variants.py` from `zebra2.ein`, and `--check` is in the
  gate: `zebra2-minus-15` (clue dropped), `ein-bugs/zebra2-bad` (clue added),
  and [`zebra2-obligations.ein`](examples/zebra2-obligations.ein) and its
  `-minus-15` twin, which are `zebra2` with the `(hrule guess …)` and the
  `(solve … :hrules …)` clause **deleted and nothing else**.
- **`docs/ein/user/guide/06_zebra_walkthrough.md`** — the Wikipedia human
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
  *ladder* — the puzzle's `(hrule …)` if it declares one, else, where an
  obligation is **activated** at root, the facts that would discharge what
  root owes, and what a rule can bring a commitment to owe, read off the rules
  with no saturation ([`oblgen.rs`](ein.rs/crates/ein-infer/src/oblgen.rs)),
  so a discharged root under it proposes nothing, else the blind
  combinatorial enumerator; an obligation rule a program loads (an import
  brings its module whole) and nothing activates chooses no rung, by one
  reading `hypgen.rs` and `mechanisms.rs` share. **The calculator's modules
  are under `stdlib/math/`**, `std.math.x` in `math/x.ein` (a dot a directory, at any
  depth, from all three places), their programs under `tests/stdlib/math/`,
  each with an entry name a program may list beside the names it writes:
  `std.math.digits` (the digit tables, a library of *facts*, generated),
  `std.math.arith` (`arith`: `z` and the division by zero's flag and rule,
  what every base shares), `std.math.decimal` (`decimal`: the numeral `n`,
  `named`, and the numeral's check over `written`), `std.math.decimal-arith`
  (`(import std.math.decimal-arith :symbols (decimal-arith add sum-of))`, with
  `(import std.math.decimal :symbols (n))` and `(import std.math.arith
  :symbols (z))` where a program writes a numeral), `std.math.num9`
  (generated, entry `arith9`), `std.math.int` (`int-arith`) and
  `std.math.rat` (`rat-arith`; an operation answers unreduced, a common
  factor left, and `rat-simplify` alone reduces, by `std.math.gcd`; equality is
  `rat-cmp`), and `std.math.gcd` (`gcd`, which is also its
  request: `(gcd X Y)`/`gcd-of` and `(lcm X Y)`/`lcm-of`, natural numbers only,
  and the tagged call `gcd-call`/`gcd-done`); binary's twins, `std.math.bits` (its tables,
  generated beside the digits'), `std.math.binary` (`binary`: the numeral
  `b2`) and `std.math.binary-arith` (`binary-arith`, every name `bin-`:
  `bin-add`, `bin-sum-of`, `bin-let`), and `std.math.convert` (`convert`:
  `to-bin`/`bin-of`, `to-dec`/`dec-of`). **The rules both bases share are
  `std.math.positional`'s**, every relation a parameter in capitals, and a
  base connects them by one call of its macro `positional` inside its closing
  rule's `(and …)`, never by naming them. A module over a module names the
  lower one's entry in its own closing rule, and lists every name of the lower
  one it writes. **A malformed term is its representation's**: a numeral's
  fault is `std.math.decimal`'s or `std.math.binary`'s, flagged where it is
  written; an operand's and a `let`'s are the module's that reads them. **Boolean functions and
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
  `regressions.toml`, one row per program and run once answered wrong, with
  today's answer and the right one as data, which `ein-cli/tests/regressions.rs`
  holds to the right one (the format is `ein_corpus::regressions`; a known
  wrong answer still open is process, [`plans/wrong_answers/`](plans/wrong_answers/README.md)).
  A file under `examples/`,
  `stdlib/` or `tests/` with no entry fails a completeness check. What runs it
  is `cargo test`, and [`corpus/README.md`](corpus/README.md) has the table of
  readers; the `--events` protocol is
  [`docs/ein/ein.rs/events.md`](docs/ein/ein.rs/events.md).
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
  the region trace, the answer and lattice dumps, the JSON summary, the
  structure's document and graphs) and
  `ein-cli`. They stack linearly up to `ein-infer` and fork there —
  `ein-einb` and `ein-render` are siblings above it, and `ein-cli` depends on
  both. `ein-cli` fronts **six** subcommands — `ein
  {oracle,render,saturate,show,solve,test}` — plus `ein kb` under the `einb`
  feature. **`ein render` draws and `ein show` lists**: `render` writes DOT,
  `show` writes Ein text a `.ein` file could hold and nothing else on stdout.

  **`.einb` is a private cache format, never an interchange one.** `ein kb save
  <file.ein> <out.einb>` writes one and every command that takes a `.ein` path
  takes a `.einb` too, dispatching on the magic bytes rather than the
  extension; `ein solve x.einb` is byte-identical to `ein solve x.ein` apart
  from the path it echoes. Anything crossing a tool boundary is still `.ein`
  text or the event protocol's JSON.
- **`utils/`** — the scripts; [`utils/README.md`](utils/README.md) is the
  catalogue — one line per script in its group (*renderers*, *checks*, **the
  measurement set**), and the **censuses**, each kept beside the tool that
  re-takes it (`utils/censuses/`). Every script that runs the engine names the
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
  SMT**, a proof of concept, wired
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
paragraph, no register count, no measurement (the user's rule).

- **Current milestone: M1f**,
  [`plans/m1f_hypothesis_and_documentation/`](plans/m1f_hypothesis_and_documentation/README.md).
  Its one phase left is **P1f.20, the documentation tree**
  ([its README](plans/m1f_hypothesis_and_documentation/p1f.20_docs_refactor/README.md)):
  every stage is done, and the phase stops for **the user's review of the new
  docs**; it is not closed, and its folder not deleted, before the user has
  read them (the user's ruling).
- **A finished plan is deleted.** A milestone or phase whose README says
  *closed* or *done*, and a followup that says *done*, goes with its folder once
  what still reads it is re-aimed and what it still owns has moved to its owner;
  it stays in `git log --diff-filter=D -- plans/<folder>`, and a link to it is
  its label text. What stays under `plans/` is unfinished work: open
  milestones, phases and followups, the open questions, the open wrong answers,
  and `plans/ideas/`.
- **What waits for the user**: milestone-wide questions, with every default
  taken, are
  [`open_questions.md`](plans/m1f_hypothesis_and_documentation/open_questions.md)
  (Q-M1f.n); a default stands until the user revises it. A phase keeps what
  it finds outside its scope in a findings page of its own and shows it, as
  P1f.20's [`findings.md`](plans/m1f_hypothesis_and_documentation/p1f.20_docs_refactor/findings.md)
  (L-n) does.
- **The wrong-answer register** is
  [`plans/wrong_answers/register.toml`](plans/wrong_answers/register.toml), the
  open wrong answers, with its rules in [its README](plans/wrong_answers/README.md);
  its count is `grep -c '^\[\[case\]\]' plans/wrong_answers/register.toml`,
  and the fixed ones are the project's regressions (`corpus/regressions.toml`).
  `python3 utils/wrong_answers.py`, a gate step, holds every row to what the
  binary answers today. Every open row is owned by a followup ([F21](plans/followups/f21_non_monotone_programs_decided.md),
  [F22](plans/followups/f22_exact_core_past_the_cap.md),
  [F24](plans/followups/f24_written_provenance.md)).
- **After M1f**: [M1g](plans/m1g_logic/README.md), logic in Ein (school
  algebra and geometry are its P1g.10 and P1g.11);
  [M1h](plans/m1h_beyond_word_problems/README.md);
  [M12](plans/m12_rules_as_data/README.md), rules as data (its
  `rule_census.md` tables are written by `utils/rule_census.py --page`);
  [M15](plans/m15_mcp_server/README.md).

## Standing rules

### The language

Each of these is a refusal at load or at the parse, on every command, stated
normatively in [`diagnostics.md`](docs/ein/user/reference/diagnostics.md#2-errors-and-exit-codes) § 2
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
  `:priority` is gone: **the agenda fires first what leads to `(false)`** —
  a rule concluding `(false)`, a stored negation, or a fact whose negation
  can be concluded — then the rest, FIFO in written order within each half
  (`saturator::AGENDA`, everywhere: root, every route's node, the oracle's,
  every reader; [Q-M1f.313](docs/decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)).
  On a monotone program it decides no answer (`ein-infer/tests/agenda.rs`);
  of a flagged one the engine claims none.
- **`:why` is required** on every relation, rule, hrule and macro;
  `--no-require-why` loads a program without.
- `__closed__` and `__symmetric__` are reserved and never bound, and never
  an object the blind enumerator proposes about, whether a rule derives the
  fact one heads or a program writes one as a value, which stays legal
  (`ein-infer/tests/dunder_objects.rs`); names the loader makes up start
  with `#` (`ein_core::text::INTERNAL_NAMES`).
- **`written` is the loader's**: it states `(written T)` for every fact
  nested in a fact a file writes, and for each one a rule's conclusion nests
  that holds no variable, derived from the fact it was read in. A rule reads
  it, which is how a representation module checks its own terms where they
  are written (`std.math.decimal`'s numeral); no program writes it, and no
  declarator binds it (`diagnostics.md` § 2.30).
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
  `grammar.md`'s two grammar blocks (from `ein.l` and `ein.y`, `EIN_BLESS=1`);
  `stdlib/math/digits.ein` and `stdlib/math/bits.ein`
  (`utils/gen_arith_tables.py`, then `utils/stdlib_manifest.py --write`); `stdlib/math/num9.ein`
  and its width-2 twin, the fixture `ein-infer/tests/golden/num2_theory.ein`
  (`utils/gen_num9.py`); the detector's two hundred random programs,
  `ein-infer/tests/golden/random_programs.txt` (`utils/gen_random_programs.py`);
  `stdlib/bool/gates.ein`'s rows and the programs
  under `tests/stdlib/bool/gates/` and `props/` (`utils/gen_gates.py`, then
  `utils/stdlib_manifest.py --write`);
  three `examples/arith/` programs (`utils/gen_arith_examples.py`); the four
  zebra2 variants (`utils/gen_zebra2_variants.py`);
  `examples/beyond_arith/send_more_money_written.ein`, `send_more_money.ein`
  with its letters declared in another order (`utils/gen_send_written.py`);
  `read-out-examples.md` (`utils/read_out_examples.py`); `config.md`'s
  defaults block and `docs/ein/ein.rs/api.md`'s `rust` block;
  `ein-core/src/printable.rs` (`utils/gen_unicode_printable.py`, with
  `ein-core/tests/golden/repr_escapes.txt` re-blessed). All of those have a
  `--check` or a test in the gate; `utils/check_generated.py` runs every
  generator's `--check`, and no crate test runs a generator. None names its
  generator; this list and `utils/README.md` are where the script is written
  down.
- **`ein.rs/vendor/lrpar/` is outside the workspace**: edit it only through a
  change to `ein.patch`. Nothing goes upstream.
- A change to what `ein solve` prints changes `read-out.md` in the same
  commit. A shape the grammar newly refuses gets a row in the grammar page's
  table.
- **A file that moves takes every reference to it along**, in the commit
  that moves it: `utils/doc_audit.py --links --check --tracked` reads each
  `.md`'s links, the links in a Rust comment and the bare paths in a `.ein`
  comment and in `corpus/corpus.toml`, and `--inbound --tracked --target
  PREFIX` counts them per target.
- **The release voice is a ratchet**: `utils/check_release_voice.py` fails
  where a file's count of lines carrying a milestone, phase, stage or task id,
  a path into `plans/` or a date rises above `utils/release_voice_baseline.txt`,
  or a file not listed there has one. A decision id passes. The commit that
  lowers a count re-banks the baseline (`--bless`).
- **The project and the process are two things, and the dependency runs one
  way.** The project is `README.md`, `ein.rs/` with its tests, `stdlib/`,
  `examples/`, `tests/`, `corpus/` and `docs/` but `docs/lib/`; the process is
  `plans/`, `utils/`, the gate and CI (`run_tests.sh`, `.github/workflows/`)
  and this file; `c/`, `nl2smt/` and `docs/lib/` are external to both. The
  process reads the project, which it sees in many states; nothing in the
  project reads, links or names the process. A decision about the process
  stays in `plans/`; a generated project file says it is generated and names
  no tool; a measurement lives beside the script that takes it.
- **A wrong answer** is filed and fixed by
  [`plans/wrong_answers/README.md`](plans/wrong_answers/README.md)'s rules: a
  row of the register with its witness, in a commit of its own; the fix moves
  the row into `corpus/regressions.toml` and the witness into the corpus, with
  any `:expect` that stated the wrong answer, in the fixing commit.
- **A stdlib rule needs a program under `tests/stdlib/` that activates it**,
  or `stdlib_coverage.rs` fails. A rule or relation added to
  a module of the calculator goes into its closing rule's two lists
  (`arith_theory.rs`, `int_theory.rs`, `rat_theory.rs`, `num9_theory.rs`,
  `gcd_theory.rs`); a
  rule added to `std.math.positional` goes into its macro's body, and a new
  parameter into the macro's list and every base's call
  (`arith_theory.rs`' `one_call_of_the_macro_connects_each_base`); a
  new operation is a row of `OPS` (`binary_theory.rs` has binary's), and
  joins `arith_search_free.rs` by its lists. **No rule of the calculator
  joins two facts on a number**, `std.math.positional`'s read as each base
  activates them. Each base reads **one `absent`, over its `let`**, which no
  rule may derive (`operand-undeclared`, `bin-operand-undeclared`), and
  writes guards in named rules (`arith_theory.rs`); a program they flag,
  `(malformed …)`, `(undeclared …)`, `(not-decimal …)` or binary's
  `(bin-malformed …)`, `(bin-undeclared …)`, `(not-binary …)`, is dead and
  claims only `(solve :expect (false))`.
- **A stage that changes a rule of the calculator runs the batch by hand**:
  `cargo test -p ein-infer --features counters --test arith_batch`. No tier
  runs it.
- `dot_wellformed.rs` skips a view of more than 1 000 edges; a program with
  such a view in a directory not named in its `SKIPPING` fails the sweep.
- **A flagged corpus program in which a hypothesis leads to a request to
  `std.math.decimal-arith` through a rule with an unbound operand fails
  `region.rs`'s fact-grain test**: the prototype's closure gives up past 10 000 shapes and
  the test names the program rather than read a narrower verdict
  (Q-M1f.225). An unflagged one costs it nothing: the closure runs only where
  a site reads it.
- A local module's name is a path from the **program** file's directory, for
  a module's own imports too: no import reaches a sibling directory,
  and a module that imports a neighbour is found from the program's
  directory, not its own.

## Running the gate

`./build.sh` builds everything first (see above); this runs it.

```sh
./run_tests.sh                                               # the whole per-commit tier
./run_tests.sh --tests-only                                  # skip the static checks
./run_tests.sh --nightly                                     # instead of the gate, every step of nightly.yml, its jobs in order
./run_tests.sh --dev-host                                    # instead of the gate, the nightly at full budget and the steps that left it
EIN_FUZZ_ITERS=20000 EIN_FUZZ_SEED=1 cargo test … -p ein-ir --profile gate --test fuzz_properties     # the nightly's fuzz job, once per seed (1, 987654321); the dev host runs 1000000
cargo bench … --bench engine -- --test                       # the nightly's bench job, and the gate's last step
cargo test … -p ein-infer --features counters --test arith_batch   # the calculator's batch, by the matcher's count; by hand, in no tier
cargo test --manifest-path ein.rs/Cargo.toml --profile gate --workspace   # the tests alone, as the gate builds them
EIN_TEST_JOBS=1   cargo test … --profile gate --workspace    # the corpus sweeps one unit at a time; the verdicts are the default's
EIN_TEST_UNITS=u.tsv EIN_TEST_JOBS=1 cargo test … --test id_order_invariance   # every unit's cost and peak memory, one line each
EIN_CORPUS_SLOW=1 EIN_TEST_SHARD=2/4 cargo test … --test corpus_cores   # one shard of a nightly step
python3 utils/gate_times.py run --out g.log -- ./run_tests.sh   # a gate run timed per step and per binary, memory and power sampled; `report g.log`
EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test corpus_cli  # + the slow entries, which the_slow_set_is_exactly_these_fifteen names
EIN_ID_SEEDS=8 EIN_CORPUS_SLOW=1 cargo test … -p ein-render --test id_order_invariance   # the dev host's, every program
EIN_JOBS_SWEEP=2,4,8,16 cargo test … -p ein-render --test jobs_invariance
EIN_BLESS=1       cargo test … --workspace                   # re-bank the goldens
EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test … -p ein-render --test id_order_invariance   # the shape golden's every line, tests/ too
ein solve … --maximal --models key|grid                       # the key and grid forms, over the states listed
EIN_LEFTOVER=1    ein solve … --json-summary out.json        # the leftover probe
ein test examples tests stdlib --json-report r.json          # every claim's verdict, as JSON
ein solve … --dump-answer DIR                                # the answer as files, each state loadable
ein solve … --maximal | --minimal | --single | -n N --region-budget F   # a question about R, the least work it needs
ein solve … --count                                          # how many, R's two counts by describe's route, no state read
ein solve … --describe-route tree|states                     # describe's and --count's route: the tree and the classes (default; no N, no empty R's owing frontier) or every alive state
ein oracle FILE [--dump DIR]                                 # R by brute force, 2ⁿ saturations — the reference every mode is held to
EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test lever_invariance   # every mode under every lever, + the slow pair's cells
EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test answer_round_trip  # every dump reloaded, + the slow and heavy_dump entries'
ein render region DIR [--visited D] [--answer D] [--view …]  # an oracle dump drawn, a solve run on it
ein solve … --dump-lattice DIR                               # every node the run visited (ein render lattice draws them)
cargo run --release -p ein-infer --example route1_reach      # route 1, R from the alive down-set, over the corpus
ein render program|kb FILE                                   # the program as written; a KB (or a dumped model)
ein show hyps FILE                                           # H as Ein facts, one candidate per line, uncut, nothing else on stdout: root's saturation and the generator, no search
ein render h-space FILE [--view …] [--dump DIR] [--supports]   # the structure of H: a read-out, a graph as DOT, or h-space.json; `ein solve` computes none of it
python3 utils/review_bundle.py                               # build/review/, one page per open wrong answer
python3 utils/review_bundle.py --regions                     # build/review/regions/, every corpus region surveyed
python3 utils/region_synth.py --example three_components     # a program written for a chosen region (--check, --random)
python3 utils/read_out_examples.py                           # the read-out reference's examples page, written from the binary
python3 utils/doc_audit.py --inbound --tracked --target docs/ein/      # per target file, the references a move rewrites, by referrer kind
python3 utils/check_release_voice.py --list [PATH …]         # every line the release-voice check counts; --bless re-banks its baseline
python3 utils/history_census.py                              # the history census: per tree, lines carrying an id, a plans/ reference, a date
```

The read-out's vocabulary is
[`docs/ein/user/reference/read-out.md`](docs/ein/user/reference/read-out.md),
every row `ein solve` prints and what it counts, and its golden page, written
by the binary, is
[`read-out-examples.md`](docs/ein/user/reference/read-out-examples.md), both
held to the printer by `ein-cli/tests/read_out_reference.rs`.
[`docs/ein/user/reference/config.md`](docs/ein/user/reference/config.md) is the census
of every `(config …)` field, CLI option and `EIN_*` name the binary reads, with a
*does it change the answer* column on every flag row, pinned by
`ein-cli/tests/config_reference.rs`. **The test harness's `EIN_*` names are the
process's**, and this table is their census: the test suite reads them, the binary
none, and setting one has no effect on any `ein` command. `grep -rhoE
'env::var(_os)?\(\s*"EIN_[A-Z0-9_]+' ein.rs/crates | grep -oE 'EIN_[A-Z0-9_]+' | sort -u`
re-takes the set, the binary's two among it.

| name | what it does |
|---|---|
| `EIN_BLESS` | `=1` rewrites a golden instead of comparing it, `config.md`'s generated block among them |
| `EIN_CORPUS_SLOW` | `=1` adds the corpus entries marked slow, the heavy answer dumps, and the slow half of every test that has one |
| `EIN_CORPUS_TIMEOUT` | the per-run wall budget of the corpus sweep through the binary, in seconds, default 300 |
| `EIN_ID_SEEDS` | how many permutations of the interned ids the id-order sweep runs, default 1 |
| `EIN_ID_FILES` | points that sweep at a directory instead of the corpus |
| `EIN_ID_REPORT` | prints that sweep's per-file report |
| `EIN_JOBS_SWEEP` | the job counts the job-count sweep compares, e.g. `2,4,8,16` |
| `EIN_TEST_JOBS` | the corpus sweeps' worker count, default 16, never more than the machine's threads. **No verdict depends on it**: the results are read back in the units' order, and a failure reads the same at any count |
| `EIN_TEST_SHARD` | `k/n`, `1 ≤ k ≤ n`: run shard `k` of `n` — a sharded sweep keeps the programs dealt to shard `k`, every program in exactly one, and a test that is one computation runs in one shard; unset, the whole run. A value that is no shard is a panic |
| `EIN_TEST_UNITS` | a file each pooled sweep appends one line per unit to: the sweep, the program, the output, milliseconds, and with one worker the unit's peak resident memory |
| `EIN_FUZZ_ITERS` | the parser fuzzer's budget, default 2 000 |
| `EIN_FUZZ_SEED` | moves that fuzzer's stream |
| `EIN_NARRATION_STRICT` | `=1` runs the id-order, job-count and corpus-shape comparisons with no narration cut: a node's derivation is compared byte for byte, not only what it concludes |
| `EIN_ROUND_TRIP_LIST` | prints each program's state files beside it in `answer_round_trip` |
| `EIN_REGION_PRINT` | turns on `ein-infer/tests/region.rs`' `print_for_inspection`, which prints and asserts nothing |
| `EIN_REGION_CENSUS` | turns on `region.rs`' `census_for_inspection`, a measurement and not a claim |
| `EIN_ROUTE1_TIMES` | prints each program's route-1 time in `region.rs`' route-1 sweep |

The walk, the search R replaced, is gone with its flags and words (`-e`,
`-m`, `EIN_TRAVERSAL`, `EIN_OBLIGATION_CHOICE`, the lattice's verdict wording);
only a plan or a commit still names them.

**`./run_tests.sh` runs every step of the per-commit CI tier**, in its order:
nine static checks (`--tests-only` skips them and the bench smoke), then
`cargo test --profile gate --workspace`, then the process's checks that need
the binary it built, then the bench smoke test. The tests run with
`TMPDIR` set to a fresh directory under `build/`, and
`utils/check_test_scratch.py` fails the gate on anything they leave there,
naming the file that writes it. **`cargo test --profile gate --workspace`
alone is not the gate**: it checks none of the nine, and none of what the
process holds itself to. **The tests read only the project**: what ties a
process file to the project is a `utils/` check and a gate step, never a crate
test — `check_gate_steps.py` (the step lists), `check_tool_mirrors.py` (the
editor grammar, `nl_census.py` and the review driver against the engine's
lists), `check_generated.py` (every generator's `--check`) and
`check_region_routes.py` (the oracle against `region.py` and
`region_synth.py`). A local gate that is a subset of the remote one is a local
gate that lies — keep the two lists the same; `utils/check_gate_steps.py`
diffs them.

**`./run_tests.sh --nightly` runs the nightly** instead of the gate: every
step of `.github/workflows/nightly.yml`, its jobs one after another
(`deep-corpus`, `deep-corpus-shards` and `mutants` once per shard, the fuzzer
once per seed, the bench smoke), so the nightly can be run before a push;
`utils/check_gate_steps.py` diffs its list against the workflow's.
**A nightly step over 5 min on the runner runs in shards**
([Q-M1f.293](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f293--the-nightlys-steps-against-five-minutes)):
a matrix job a shard, `EIN_TEST_SHARD=k/n`, `ein_corpus::shard` dealing every
program to exactly one shard and `utils/stdlib_mutants.py --shard` every
`n`-th mutant; each shard is measured on four P-core threads with
`EIN_TEST_JOBS=4`, the runner's count, on AC, and `check_gate_steps.py` holds a
sharded step to every one of its shards. **The nightly holds only what guards an answer or a
contract no per-commit step guards**, each step within 5 min on the runner
([Q-M1f.102](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f102--what-the-ci-nightly-keeps-and-what-the-dev-host-runs)).
**`./run_tests.sh --dev-host` is the tier nothing schedules**, which the user
runs here or in the runner's image (`utils/ci_container.sh --dev-host`):
every nightly step at full budget and the steps that left or never ran in
CI. A step that leaves the nightly is named in `check_gate_steps.py`'s
`DEV_HOST_ONLY` with why, or the gate fails.

A reference that cannot be a link is still fine as `` `code` ``, and that is
the fix for most of the rustdoc ones: `ein-ir` cannot link into `ein-infer`
(the dependency runs the other way), a `#[cfg(test)]` module is not in the
documented crate, and a private item is a real name that public docs may cite
but not hyperlink.

**What a commit reads and what the nightly does** (the user's rulings on
Q-M1f.287 to .294). Per commit:
`id_order_invariance` digests every rendering of every corpus program outside
`tests/` against `ein-render/tests/golden/corpus_shapes.md5` and permutes the
id space of those of them that **search**, whose describe saturates more than
root (`corpus_ops::searches`, computed, never listed); `jobs_invariance` reads
the same programs that search, `dot_wellformed` every program outside
`tests/`, `route_invariance` every program but the `slow` entries, and
`answer_dump` every run but the `heavy_dump` entries'. `EIN_CORPUS_SLOW=1`
reads every corpus program in each, which the nightly does; a per-commit
`EIN_BLESS=1` re-banks the golden's lines of the programs it reads and keeps
the others, and a corpus program with no line fails per commit. Each corpus
sweep over (program, op) pairs loads a program once and runs every op on a
copy (`corpus_ops::Loads`; `Ast`, `Terms` and `Kb` are `Clone`).

**The gate profile**: every test step of every tier runs
`cargo test --profile gate`, into `ein.rs/target/gate/` — release codegen
with `debug_assert!` and overflow checks on (`ein.rs/Cargo.toml`), which the
dev profile kept at `opt-level = 1` and `--release` dropped. Clippy, rustdoc
and the bench smoke are unchanged; the review-bundle smoke drives
`target/gate/ein`. **The worker pool**: a sweep over the corpus hands its
loop to `ein_corpus::pool::map`, whose units — a (program, output) pair, a
program, a CLI cell — run on `EIN_TEST_JOBS` workers (default 16, never more
than the machine's threads, shared by every pool of one test binary; a
program on `pool::HEAVY` runs in a lane of a quarter of them) and come back
in the units' order. **No verdict depends on the count**: a unit keeps no
state another reads, and the sweep folds the results as its serial loop did,
so a failure reads the same at 1 and at 16 (`pool.rs`' tests). A new corpus
sweep uses the pool. `utils/stdlib_mutants.py --jobs` is the same count for
the mutation census.

Everything runs one engine. `cargo test --profile gate --workspace` is the gate — the corpus
sweep through the CLI, the shape digests, the goldens, the manifest's own
invariants, and the **stdlib conformance** pair — and it needs **Graphviz** on
`PATH`, because `dot_wellformed.rs` is the only authority the DOT views have
on being well-formed and it fails rather than skips without it.

**Which Graphviz build is part of what that test checks.** The CI runs on
whatever `ubuntu-latest` names, Ubuntu 26.04, whose runner installs the
**x86-64-v3 build** of every package. That build of Graphviz 14.1.2 compiles
`libpathplan`'s orientation test into a fused multiply-add, and its spline
router refuses corpus views that the same version's baseline build, 2.42.2 and
16.0.0 all draw, unless the overlay and Levi digraphs set `ranksep=0.55`
(`dot_util::POLYGON_RANKSEP`), as they do. A green gate here says nothing
about that build. **`utils/ci_container.sh`** runs
`./run_tests.sh`, or `--nightly`, in a container built like the runner, on
HEAD as committed, so an image move can be checked before a push.

**When `utils/ci_container.sh` is part of done**
([Q-M1f.77](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f77--when-is-utilsci_containersh-part-of-done)).
A stage that changes a DOT writer, the review bundle or what CI installs is
not done until `utils/ci_container.sh` is green on its commit. A Graphviz
refusal can reach CI behind a green gate here: the router above, and `fdp`,
which 26.04's `graphviz` links to `dot` with no layout plugin behind it, so
that every KB view is refused without the plugin. The workflows and the image
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

The `ein` binary links `snmalloc` by default, so
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
