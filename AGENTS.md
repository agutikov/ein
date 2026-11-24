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
  implementation (`01-static-ir/`, `02-kb-evolution/`, `03-rule-evolution/`), in
  mathematics, graphs and words, with no code and no test named, and one
  notation, the table of [`reasoning/README.md`](docs/ein/reasoning/README.md#notation); and
  **`ein.rs/`**, how this implementation computes it (below), where
  [`08-reasoning-in-code/`](docs/ein/ein.rs/08-reasoning-in-code/README.md) is what
  holds the engine to the reasoning: for each of its pages, the code, the
  tests and the commands. Start here for
  any "what does Ein reason about / how" question. The pages an agent most
  often needs:
  [`07-diagnostics.md`](docs/ein/user/04-reference/07-diagnostics.md#2-errors-and-exit-codes),
  the parse and load diagnostics and the load refusals the engine defines
  normatively (§ 2 has one section per refusal);
  [`06-solution-semantics.md`](docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md),
  the answer as a region R;
  [`08-read-out.md`](docs/ein/user/04-reference/08-read-out.md), every row
  `ein solve` prints and what it counts;
  [`05-hypothesis-space.md`](docs/ein/reasoning/02-kb-evolution/05-hypothesis-space.md),
  the hypothesis space and its structure, each notion defined;
  [`03-hypotheses-and-the-answer.md`](docs/ein/ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md),
  `ein show hyps`, `ein render h-space` and `h-space.json`, key by key (its
  § 1.3, § 2.3 and § 2.4), and the routes' tests;
  [`06-config.md`](docs/ein/user/04-reference/06-config.md), the `(config …)` flags,
  the CLI options and the `EIN_*` names, each with a *does it change the
  answer* column;
  [`02-syntax.md`](docs/ein/user/04-reference/02-syntax.md) and
  [`01-grammar.md`](docs/ein/user/04-reference/01-grammar.md), the language;
  [`14-natural-language.md`](docs/ein/user/03-syntax-and-semantics/14-natural-language.md),
  `:why` texts and how a fact reads as English; and
  [`01-architecture.md`](docs/ein/ein.rs/01-architecture.md), the crate and module
  map, with the design contracts the code cites under `docs/ein/ein.rs/03-design/`
  and each algorithm told once, as a flow, in
  [`02-algorithms.md`](docs/ein/ein.rs/02-algorithms.md).
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
  What it takes to call a claim *settled* is two decisions: a behaviour is
  refuted only by a banked probe, an absence by naming the thing that checks
  it, a **risk not by argument at all**
  ([Q-M1e.1](docs/decisions/docs/Q-M1e.1-a-behaviour-is-refuted-by-a-banked-probe.md)),
  and an argument for leaving something alone holds only while its premise is
  enforced by something that fails when it stops being true
  ([Q-M1e.2](docs/decisions/docs/Q-M1e.2-an-argument-is-enough-only-when-its-premise-is.md);
  the premises the answer rests on, each with what enforces it, are
  [`03-design/README.md`](docs/ein/ein.rs/03-design/README.md#what-the-answer-rests-on)).
  **How a page is named and written**: every folder and page under `user/`,
  `reasoning/`, `ein.rs/` and `overview/` is `NN-kebab-name`, numbered in its
  folder's reading order from `01` with no gap, `README.md` and `diagrams/`
  aside ([Q-M1f.367](docs/decisions/docs/Q-M1f.367-the-folders-and-pages-of-the-documentation-are.md));
  a diagram is a file, `NN-name.dot` or `NN-name.puml` in the `diagrams/`
  folder beside its page, `NN` the page's number, and the page shows the
  picture drawn from it, an image of `diagrams/NN-name.svg` whose text says
  what it shows: no page holds a fenced `dot` or `plantuml` block, none is
  drawn in characters, and what `ein` prints is output, marked
  `<!-- output -->` where it could be read as a drawing
  ([Q-M1f.368](docs/decisions/docs/Q-M1f.368-a-diagram-is-a-file-and-a-page-shows-its-picture.md));
  a formula is TeX, between `$$` lines or inline `$`, never a `text` block
  ([Q-M1f.369](docs/decisions/docs/Q-M1f.369-a-formula-is-written-in-tex-displayed-or-inline.md));
  and each perspective has its languages
  ([Q-M1f.370](docs/decisions/docs/Q-M1f.370-each-perspective-of-the-documentation-is-written.md)):
  `reasoning/` is mathematics, graphs and words, with no Ein source, command,
  Rust, file path, test name or algorithm, and links only pages of `docs/ein/`
  and decisions; `user/` is Ein first, with graphs, words, and mathematics
  where it explains, a core notion in all four and special syntax not
  ([Q-M1f.371](docs/decisions/docs/Q-M1f.371-a-core-notion-is-shown-four-ways-in-the-users.md));
  `ein.rs/` is software documentation in whatever language it needs, and is
  where tests and algorithms are named.
  **What keeps the pages true**: `utils/doc_audit.py --links --check
  --tracked`, a gate step (every link and anchor, every prose `§` number
  written after a link, every decision id the project cites); its
  `--identifiers` report (every backticked name that looks like code, looked
  up in `ein.rs/crates/`: a report, never a gate, since a page that rightly
  says a name is gone still names it); the release-voice check;
  `utils/check_doc_blocks.py`, a gate step (a diagram is a file with its
  picture: no fenced `dot` or `plantuml` block, every source with the `.svg`
  drawn from it as it stands, by the stamp `utils/render_diagrams.py` writes,
  named for the page that shows it, and every `.dot` through the Graphviz at
  hand; the numbering, the languages of `reasoning/`, no formula in a `text`
  block and no diagram in characters; `--plantuml`, the dev host's, draws the
  `.puml` sources too); and running
  the commands a page shows, which no script does and which finds the most.
- **`docs/decisions/`** — **every question about Ein that has an answer, as
  that answer** ([`docs/decisions/README.md`](docs/decisions/README.md)): one
  file a decision, `<subject>/<id>-<slug>.md`, the final answer only, under
  the id of the question it answers (`Q-M1e2.24`, `Q-M1f.313`, `Q23`). A page
  links a decision's file; a code, doc or `.ein` comment and a corpus note
  cite it by its bare id, which the link check resolves to exactly one file.
  A question still open is a work item and lives in `plans/`, cited by
  nothing in the project; a decision about the process stays in `plans/` too.
  **An id is kept and never reused**: a ruling with no id takes the next free
  id of the milestone that ruled it, and a superseded id that is still cited
  is rewritten to the id that carries the decision, with no redirect file.
- **`docs/ein/ein.rs/`** — **the engine as built**: the crate and module map,
  the algorithms (`02-algorithms.md`), the design contracts the crates cite
  (`03-design/`), the `--events` protocol and the other engine read-outs, the
  reasoning's pages in this code (`08-reasoning-in-code/`), and **how to drive
  Ein as a library**,
  [`04-api.md`](docs/ein/ein.rs/04-api.md): the **crates** — `ein-ir` to load, `ein-infer` to solve, `ein-render` to
  explain, `ein-einb` to cache. **Its worked example is a test.** The page's
  `rust` block is the marked region of
  [`ein-cli/tests/embedding.rs`](ein.rs/crates/ein-cli/tests/embedding.rs) and
  a test in that file diffs the two, so `cargo test --workspace` keeps the
  page true — *edit the test, run it, paste; never edit the block by hand*.
- **`docs/ein/user/01-install.md`** — the install page: the two channels (a release
  binary, `cargo install --path`), what `ein --version`'s five lines mean, how
  to check a binary's stdlib against the checkout's manifest, and
  `$EIN_STDLIB`. **`pip install` is not a channel.**
- **`docs/ein/user/02-guide/`** — **the newcomer tutorial** ("Learn Ein by solving the
  Zebra puzzle"). User-facing; links the reference and the concept pages
  under `docs/ein/user/`, never explains internals; its chapter 6 is
  `06-zebra-walkthrough.md`.
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
  `-minus-15` twin, which are `zebra2` with the hypothesis rule `(rule guess
  …)` and its five `(guess …)` activators **deleted and nothing else**.
- **`docs/ein/user/02-guide/06-zebra-walkthrough.md`** — the Wikipedia human
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
  conclude `(open ?R)`, are not in the saturation agenda, and are read at
  each state's fixpoint, where a state holds `(open R)` once for each
  relation it owes. **H is the union of the hypotheses root holds**
  ([Q-M1f.570](docs/decisions/hypotheses/Q-M1f.570-h-is-the-union-of-the-hypotheses-root-holds.md)):
  the `(open F)` facts a rule concludes (a hypothesis rule), the program
  writes, and an obligation **activated** at root writes for what would
  discharge what root owes, and what a rule can bring a commitment to owe,
  read off the rules with no saturation
  ([`oblgen.rs`](ein.rs/crates/ein-infer/src/oblgen.rs)), less what root
  holds or denies; the blind combinatorial enumerator writes only where root
  holds no `open` fact and no obligation is activated, and an obligation rule
  a program loads (an import brings its module whole) and nothing activates
  writes nothing, by one reading `open.rs`, `hypgen.rs` and `mechanisms.rs`
  share. **The calculator's modules
  are under `stdlib/math/`**, `std.math.x` in `math/x.ein` (a dot a directory, at any
  depth, from all three places), their programs under `tests/stdlib/math/`,
  each with an entry name a program may list beside the names it writes:
  `std.math.digits` (the digit tables, a library of *facts*, generated),
  `std.math.arith` (`arith`: `z` and the division by zero's flag and rule,
  what every base shares), `std.math.decimal` (`decimal`: the numeral `n`,
  `named`, and the check an operation asks of a numeral), `std.math.decimal-arith`
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
  fault and its label are `std.math.decimal`'s or `std.math.binary`'s, its
  walk `std.math.positional`'s; a number an operation makes from valid ones is
  labelled valid as it is made and not walked again, and every other numeral
  is checked where an operation or a tagged call takes it, a tagged call
  handing itself on to a walk of the calculator's own (Q-M1f.404); an operand's and a `let`'s are the module's that reads them. **Boolean functions and
  circuits are under `stdlib/bool/`**, their programs under
  `tests/stdlib/bool/`, each module's entry its last name: `std.bool.gates`
  (the sixteen functions of two inputs, a table `(gate2 NAME A B OUT)` whose
  rows are generated and a relation each, `(And A B OUT)`), `std.bool.props`
  (thirty relations known of them, derived by rules), `std.bool.circuit` and
  `std.bool.inputs` (a circuit's inputs as hypotheses); a function's name is
  `std.bool.gates`', as a relation and as an argument, so a program that writes
  one lists it. **The two rules of a stored negation are modules a program
  imports**: `std.negation`, the elimination of a double negation
  (`eliminate-0` to `eliminate-6`, one rule a count of arguments, the relation
  its parameter), and `std.contradiction`, **the contradiction, a theory a
  program imports** ([Q-M1f.557](docs/decisions/stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md)):
  `clash-0` to `clash-6` conclude `(false)` from a fact beside its stored
  negation, and `clash-of-0` to `clash-of-6` activate them from every
  declaration, so the import is the switch, for the relations a program
  declares; one it does not declare takes its activator written. The kernel
  has no check of its own: without such a rule a state that holds a fact
  beside its negation is alive. `std.algebra`, `std.bijection` and
  `std.slots` conclude negations and import it, and so does every corpus
  program that relies on the death. Neither module declares a relation.
  Their programs are under `tests/stdlib/negation/` and
  `tests/stdlib/contradiction/`, and `ein-infer/tests/contradiction_rules.rs`
  holds the firing. **What an obligation requires and what pays it are facts a program
  asks for**: `std.debts`, for each of the library's four obligations a rule
  that concludes `(debt-due R J)` from its trigger and one that concludes
  `(debt-paid R J)` from the trigger with its witness (`total-due` and
  `total-paid`, `surjective-`, `room-`, `fill-`), `J` the instance, a fact no
  state holds; and, where a program writes `(debts-of-owed)`, `debts-of-total`,
  `-surjective`, `-room` and `-fill`, which activate a pair from the
  obligation's activator, and `discharged-from-nothing-unpaid`, which
  concludes `(debts-discharged)` where no requirement lacks its payment. It
  imports `std.algebra` and `std.slots`, declares no relation and writes no
  fact, and the eight words it writes or reads are no module's. Its programs
  are under `tests/stdlib/debts/`, and `ein-infer/tests/debts_rules.rs` holds
  the requirements with no payment to what the kernel owes, node by node,
  which no claim can say. **`std.core` is the module every program gets**, with no import
  line and under its full names, after the program's forms and its imports'
  ([Q-M1f.551](docs/decisions/stdlib/Q-M1f.551-std-core-is-loaded-into-every-program.md)):
  its one rule, `std.core.inconsistent`, matches a held `(false)` and states
  `:ein inconsistent`, so `(false)` is the library's convention and no word
  the kernel reads. A file may import it as any module (no second copy, the
  module stays last), a library `$EIN_STDLIB` names replaces it (one with no
  `core.ein` is refused: *std.core was not found in $EIN_STDLIB*), a saved
  file carries it, and a container stores the program without it. Its
  programs are under `tests/stdlib/core/`. `std.logic` is left free for M1g.
  Their rules are in [§ Edits that go together](#edits-that-go-together), and
  [`stdlib/README.md`](stdlib/README.md) has a section for each, and the
  reference a page for each family, usage apart from how a module works and
  every rule drawn:
  [`docs/ein/user/04-reference/04-stdlib/`](docs/ein/user/04-reference/04-stdlib/README.md).
- **`tests/`** — **the ein-lang test suites**, a third corpus root beside
  `examples/` and `stdlib/`: a `.ein` here with no `corpus.toml` entry fails
  the same completeness check. Its one subject is
  [`tests/stdlib/`](tests/README.md) — **one program per stdlib rule or tight
  family**, each carrying an `:expect`, so `ein test tests/` is the whole
  suite. They are deliberately *not* under `examples/`: that directory is
  things to read, and these exist to break. Idioms worth knowing before
  writing a new one — **naming a relation closes its held extent** (which is how a rule
  with no guard gets a negative case at all), **`(unknown …)` as a probe**
  (the only way to say a rule did *not* derive a negative, since stored
  negatives are not closed), **a refutation rule gets two files** — one where
  it fires and one where it is loaded, activated and satisfied — **where two
  rules reach one verdict, separate them by activation**, because an
  expectation made of facts cannot say which one got there, and **an
  obligation rule concludes only `(open R)`**, which no claim lists, so its
  fixture carries an ordinary `(model …)` claim, its owe counts are asserted in
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
  [`docs/ein/ein.rs/05-events.md`](docs/ein/ein.rs/05-events.md).
  **`slow = true` is a measured claim** — an entry whose declared runs cost
  1 s or more together, recorded in `cost_ms` and checked in both directions
  (`utils/bench_env.sh --cores PT python3 utils/corpus_cost.py` re-takes it;
  `corpus_cli`'s `the_slow_set_is_exactly_these_sixteen` names the set). The
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
  names seeds the guessable atoms, what the blind enumerator guesses
  about), **`ein-einb`** (the `.einb`
  binary KB container, and the **only library crate that is not
  `#![forbid(unsafe_code)]`**: its `cast.rs` is the one audited module
  `03-design/10-toolchain-and-layout.md` § 2 permits `unsafe` in; the other audited `unsafe` is
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
- **`nl2smt/`** — **the Zebra puzzle's sentences through a parser, NLTK or
  link grammar, to SMT**, a proof of concept, wired
  into nothing: no crate depends on it and the gate does not run it.
  [`nl2smt/README.md`](nl2smt/README.md) is the page, with the command and
  its tests, which run under each parser that is installed and skip where no
  parser (Python's `nltk`, `link-parser`), `racket` or every solver is
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
  One phase is left: **P1f.21, the kernel's verdicts as rules**
  ([its README](plans/m1f_hypothesis_and_documentation/p1f.21_verdicts_as_rules/README.md)),
  run in two parts on the user's word, what does not touch `open` or the
  hypotheses first, and that part is built and its pages written:
  S1f.21.6, S1f.21.8, S1f.21.10, S1f.21.15, S1f.21.17, S1f.21.11 and
  S1f.21.2 done. Of the second part, S1f.21.12, `open` facts, is built, and
  S1f.21.13, the lattice written as rules, is in the corpus, and S1f.21.18
  has removed the `hrule` form and the keyword `:hrules`, and S1f.21.20 has
  fixed what the first uses of `open` found, and S1f.21.21 has made the
  contradiction's rules cheap; S1f.21.22 has built what the pages found
  short in the binary, and S1f.21.23 reads a resumed state's delta off its
  own layers; S1f.21.19 has put the review items on the built engine, and
  S1f.21.24 has fixed the wrong answer those runs found, a hypothesis
  rule's `absent` premises. Every stage planned then is done. The user has
  answered every item of the first part's review; the ruling on item 82,
  an activator that names no relation activating nothing in silence, is
  S1f.21.25, is built; its defaults are review item 87. The part's pages,
  S1f.21.14, are written. Q-M1f.566 to
  Q-M1f.579 are ruled and the second part's builds are planned on them,
  S1f.21.12 and S1f.21.18. **On the user's word every stage left is run to
  the phase's end, on the defaults of the open review items, and the phase
  is not closed**; the items are reviewed by the user after the build, with
  their renders on the final engine (S1f.21.19). **The phase's close waits for
  the user's word.**
  The milestone's plans are deleted when it is done, and **the work stops
  after M1f** (the user's rulings); the close of M1f was started and is
  paused.
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
  (Q-M1f.n); a default stands until the user revises it. What the closed
  phases found and left, the user's calls among it marked, and the defaults
  they took, are the milestone's
  [`findings.md`](plans/m1f_hypothesis_and_documentation/findings.md) (L-n).
  A phase keeps what it finds outside its scope in a findings page of its own
  and shows it: P1f.21's (V-n), beside its README.
- **The wrong-answer register** is
  [`plans/wrong_answers/register.toml`](plans/wrong_answers/register.toml), the
  open wrong answers, with its rules in [its README](plans/wrong_answers/README.md);
  its count is `grep -c '^\[\[case\]\]' plans/wrong_answers/register.toml`,
  and the fixed ones are the project's regressions (`corpus/regressions.toml`).
  `python3 utils/wrong_answers.py`, a gate step, holds every row to what the
  binary answers today. Every open row is owned by a followup ([F22](plans/followups/f22_exact_core_past_the_cap.md),
  [F24](plans/followups/f24_written_provenance.md)). **No row is of a program the
  static check flags**: nothing is claimed of such a program's answer, so none is
  held to one (Q-M1f.564).
- **After M1f**: [M1g](plans/m1g_logic/README.md), logic in Ein (school
  algebra and geometry are its P1g.10 and P1g.11);
  [M1h](plans/m1h_beyond_word_problems/README.md);
  [M12](plans/m12_rules_as_data/README.md), rules as data (its
  `rule_census.md` tables are written by `utils/rule_census.py --page`);
  [M15](plans/m15_mcp_server/README.md);
  [M1k](plans/m1k_notations_as_facts/README.md), other notations read as Ein
  facts: a labelled graph as a knowledge base (P1k.10, from M1f's P1f.30) and
  S-expressions as facts (P1k.20); [M1n](plans/m1n_non_monotone_programs/README.md),
  non-monotone programs (F21, moved into it, with its probes; the register
  holds no row of a flagged program, and the witnesses of the rows it once held
  stay in the corpus and under `plans/wrong_answers/witnesses/`);
  [M1p](plans/m1p_ein_to_natural_language/README.md), Ein to natural language:
  a value's own format (P1p.1), every fact a sentence (P1p.2), reasoning in
  words (P1p.3); each phase
  **run only on the user's word**.

## Standing rules

### The language

Each of these is a refusal at load or at the parse, on every command, stated
normatively in [`07-diagnostics.md`](docs/ein/user/04-reference/07-diagnostics.md#2-errors-and-exit-codes) § 2
or in the grammar's table (`ein-ir/tests/grammar_pages.rs`).

- **Ein has no numbers, no ranges and no int kind**, and none is planned.
  `1` and `01` are two names. Arithmetic is a theory in the stdlib, computed
  by saturation.
- **An object is a node of the knowledge-base graph, an atom or a fact**, and
  nothing else is called an object. An **atom** is a labelled node with no
  outgoing arrow, its label its **name**; a **fact** is a node with ordered
  arrows, the first at its **relation**, an atom, the others at its
  arguments, objects. An object's identity is its name or its tuple, and the
  graph is the representation, a written list of tuples its notation. A
  relation's **extent** is every fact of it, held or not; what a knowledge
  base holds of it is the **held extent**; a declaration, and any fact with
  the relation's atom under a later arrow, speaks of the relation as one
  object ([Q-M1f.379](docs/decisions/language/Q-M1f.379-an-object-is-a-node-of-the-knowledge-base-graph.md),
  [Q-M1f.380](docs/decisions/language/Q-M1f.380-an-objects-identity-is-its-name-or-its-tuple.md),
  [Q-M1f.381](docs/decisions/language/Q-M1f.381-a-relation-is-the-atom-under-a-facts-first-arrow.md),
  [Q-M1f.382](docs/decisions/language/Q-M1f.382-a-labelled-arrow-is-a-fact-of-three-arrows.md);
  the page is [`01-knowledge-base.md`](docs/ein/reasoning/01-static-ir/01-knowledge-base.md)).
  *Value* is the code's word for an object (`ein_core::Value`), and *atom*
  never means an atomic formula: that is a fact.
- **A fact that is an argument is one a state could hold**: a fact of an
  ordinary relation or its negation. `eq`, `neq`, `and`, `or` and `false`
  with an argument are never a nested head; `not` of an atom is the negation
  of no fact. **Under an ordinary relation `open` and `absent` are names**
  ([Q-M1f.571](docs/decisions/language/Q-M1f.571-absent-takes-a-premise-and-open-a-name-or-a-fact.md)):
  `(think Ann (absent (p A)))` is a fact the kernel reads nothing of.
- **What a state owes and what it supposes are facts**
  ([Q-M1f.566](docs/decisions/hypotheses/Q-M1f.566-what-a-state-owes-is-open-r-a-fact-and-open-f.md),
  [Q-M1f.573](docs/decisions/hypotheses/Q-M1f.573-under-open-a-relation-is-owed-and-a-fact.md)):
  `(open R)`, once for each relation the state owes, and `(open F)`, a
  hypothesis. A pass writes them at a state's fixpoint into a layer no
  fork inherits, at root and where a state is printed, saved or reads them,
  every state under `--in-flight`
  ([Q-M1f.655](docs/decisions/engine/Q-M1f.655-open-facts-are-written-at-root-and-where-a-state.md),
  [Q-M1f.656](docs/decisions/cli/Q-M1f.656-a-flag-runs-the-pass-at-every-state-and-checks.md));
  a written `(open F)` is held at root alone, and the empty commitment's
  state is root's. `open` is a
  position word, read by the kernel at the head of a written fact, a premise
  or a conclusion; directly under `absent` or `open` the other is refused but
  `(absent (open P))`, and so is `(absent (absent P))`
  ([Q-M1f.568](docs/decisions/language/Q-M1f.568-directly-under-absent-or-open-the-other.md)).
  A rule may read an `open` fact, and its program is flagged
  ([Q-M1f.569](docs/decisions/hypotheses/Q-M1f.569-a-rule-may-read-an-open-fact-and-its-program.md));
  a hypothesis first held above root flags its program in flight, where
  the pass writes that state's `open` facts
  ([Q-M1f.574](docs/decisions/answer/Q-M1f.574-a-hypothesis-first-held-above-root-makes-its.md)).
- **A string is natural-language text**, written only after `:why`, `:source`
  or `:goal-text`, never an object. Names are ASCII.
- `_` and `?_` are refused everywhere. Keywords are closed per form.
  `:priority` is gone: **the agenda fires a rule that states
  `:ein inconsistent`, and one that refutes a fact beside one holding it
  alone, before every other, then what leads to one** — a rule
  concluding a fact such a rule matches (`(false)`, in every program, by
  `std.core`'s rule; read off the rules, `monotone::leads_to_an_end`), or
  either side of a pair a rule refutes, a fact beside one holding it as its
  one argument (a stored negation, where the program imports
  `std.contradiction`; read off the rules, `contradiction::Pairs`) — then the rest,
  FIFO in written order within each half (`saturator::AGENDA`, everywhere:
  root, every route's node, the oracle's, every reader;
  [Q-M1f.313](docs/decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)).
  It decides no answer where no rule's `absent` reads a relation another
  rule derives at the same boundary; `ein-infer/tests/agenda.rs` holds it on
  the corpus's monotone programs. Of a flagged program the engine claims no
  answer.
- **`:why` is required** on every relation, rule and macro;
  `--no-require-why` loads a program without. **A `:why` reads the form it
  is written on**: a declaration's, `{?n}` its n-th arrow (`{?1}` the atom
  `relation`, `{?2}` the relation's name, `{?3}` its first label), printed
  after the declaration where a rule reads it; a rule's, its variables. A
  fact of a relation takes no text from the declaration and carries none of
  its own: it reads by its `:source`, its rule's `:why`, or as written
  (Q-M1f.58).
- `__closed__` is reserved and never bound, and never an atom the blind
  enumerator guesses about, whether a rule derives a fact of it or a
  program writes it as an argument, which stays legal
  (`ein-infer/tests/dunder_objects.rs`); names the loader makes up start
  with `#` (`ein_core::text::INTERNAL_NAMES`). **The kernel reads no
  theory's name**: its names are `ein_core::terms::Kernel`'s, and `is-a`,
  `symmetric` or `__symmetric__` mean only what a rule makes them mean; the
  kernel mirrors no relation (a symmetric one is `std.algebra`'s rule).
  `ein-render/tests/renaming_invariance.rs` holds every corpus program to
  its answer with every other name renamed
  ([Q-M1f.436](docs/decisions/engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md)).
- **`false` and `not` are the standard library's names**
  ([Q-M1f.557](docs/decisions/stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md)):
  position words, names wherever a name goes but a head, where the grammar
  gives `(false)` and `(not X)` their shapes; a relation, a rule and an
  alias may bind either (`ein_core::LIBRARY_NAMES`), a macro not. A
  declarator binds none of `eq`, `neq`, `and`, `or`, `absent`, `open`,
  `relation` and `__closed__`.
- `eq` and `neq` are computed relations, never stored. A conclusion's
  variable must be bound by every way of satisfying the `:match`. An empty
  `:match` is true and fires once.
- **A rule states what it concludes under `:assert`, or a word to the
  engine under `:ein`**, never both
  ([Q-M1f.399](docs/decisions/language/Q-M1f.399-a-rule-states-facts-or-a-word-to-the-engine.md)).
  The word is no fact: no state holds it, no rule matches it, it stands under
  no arrow. There is one, `inconsistent`
  ([Q-M1f.550](docs/decisions/language/Q-M1f.550-the-engines-word-is-inconsistent.md)):
  where a rule that states it fires, the state's saturation ends and the
  state is dead, and no fact is written (`ein_core::Kb::ended` holds the
  firing's record, which a core is taken over). A word that is none of the
  engine's and `:ein` beside an `:assert` are refused at load. **The kernel reads `(false)` and `not` by name nowhere to judge a
  state**: a held `(false)` kills by `std.core`'s rule, and a fact beside its
  stored negation by `std.contradiction`'s, where a program imports it. An
  explanation names the rules a program wrote or imported and reads through
  a rule of `std.core` (`ein_core::Rule::implicit`), and through a firing
  whose premises are a fact and one holding it to the pair
  (`ein_infer::contradiction::detect`), so `refuted by` names the rule that
  concluded the `(false)`, or the rules that derived the pair's two sides.
- **Under `:assert` a rule concludes a conjunction of literals** — facts,
  stored negations and hypotheses `(open F)`, with `(false)` and checks
  among them — or `()`, or an obligation, `(open R)`, as its whole
  `:assert`
  ([Q-M1f.575](docs/decisions/language/Q-M1f.575-open-r-is-a-rules-whole-assert-and-open-f-stands.md)). An `(and …)` is read flat at any depth, by every
  reader through one function, `ein_ir::Ast::conjuncts`. No disjunction and
  no compound fact: *one of* is the hypothesis layer's. **A hypothesis is one
  fact**: `(open (and …))` is refused, and a rule concludes several
  hypotheses as several `(open F)`.
- **A hypothesis rule is a rule that concludes `(open F)`**, activated as
  every rule is, by a fact of its name the program writes anywhere, or by
  none where it has no parameter; there is no `hrule` form and no `:hrules`
  keyword, and a program that writes either is refused at the parse
  ([Q-M1f.567](docs/decisions/hypotheses/Q-M1f.567-a-hypothesis-rule-is-a-rule-that-concludes-open.md)).
- A query's `:goal` is held to what a rule's `:match` is held to. Every name
  in a `(solve …)` list names something the program has. A macro's call is no
  fact.
- **A relation is overloaded by its count of arguments**
  ([Q-M1f.447](docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md)):
  a declaration is an ordinary fact and holds no form to its count, so a
  form of a declared relation at another count loads, in silence, a fact no
  pattern of the other count matches. A relation may be declared more than
  once, at one count or at several, and no declaration is refused as a
  duplicate; what reads a declaration reads each (`ein_core::Program::declarations`):
  the blind enumerator guesses at each declared count, and `std.contradiction`
  activates the rule of each. Ein has no types, and a label means what a
  rule makes it mean (Q-M1f.372).
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
- **The two lists and a closure configure the blind enumerator, and agree
  with the `open` facts root holds**
  ([Q-M1f.578](docs/decisions/hypotheses/Q-M1f.578-the-two-lists-and-a-closure-configure-the-blind.md)),
  read on root's fixpoint: no `:no-hypothesis` names, no
  `:hypothesis-relations` leaves out and no closure closes a relation root
  holds an `open` fact of, a hypothesis or what it owes; beside them
  `:hypothesis-relations` names exactly their relations; and no relation is
  in both lists or listed and closed. A redundant name is a `note:`
  (`ein_infer::mechanisms`). **`(solve :no-search)`**, a flag with no value,
  states that the lattice is root alone: nothing is proposed, so R is {∅} or
  empty, and it is how a program says *judge root alone*. It relaxes no
  conflict; beside a hypothesis rule or a list it is a `note:`, beside an obligation
  or a closure silent.

### The answer

- **The answer is a region R**, the commitments whose state is alive and
  discharged. A bare `ein solve` describes R; `--minimal`, `--maximal`,
  `--single`, `-n N` and `--count` each do the least work their question
  needs; **`ein oracle` is the reference every mode is held to**
  (`ein-infer/tests/explore.rs`). No two commands may disagree.
  **`--no-search`** (`ein solve`, `ein test`) declines to discover R: it
  reports root as the full run reaches it, states no R, and decides only the
  claims root decides.
- **A non-monotone program loads, and nothing is claimed of its answer**
  ([Q-M1f.564](docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)).
  No command refuses a program for an `absent`, whatever its rules conclude
  from it. A program the static check flags (`ein_infer::monotone`) is
  answered by the cube, with no licence, and **every command that answers
  about a commitment says in capitals that the program is not monotone and
  that nothing is claimed of the answer**: the read-out's first row,
  `WARNING`, stderr's first line and the summary's `monotonicity.warning`
  (`ein_infer::detector::WARNING`; `ein-cli/tests/non_monotone_warning.rs`
  holds the three on every flagged corpus program and none on a monotone
  one); no page, comment or test says what it answers is right, sound or
  complete. `ein-infer/tests/non_monotone_loads.rs` holds the corpus programs
  whose refutation rests on an `absent` a commitment can still fill to
  *flagged, and no licence*. **No suite validates a flagged program's
  answer**: it may state an `:expect`, which `ein test FILE` checks for
  whoever asks, but no program under `tests/` is flagged, a flagged corpus
  entry declares no `test` run and no run that checks a claim it states
  (`ein-corpus`'s `a_flagged_program_declares_no_run_that_checks_a_claim`),
  the shape golden banks none of its answers (`Op::is_answer`,
  `Unit::flagged`), and no regression row, register row or assertion holds
  one to what it answers. What compares a run with a run stays on it: one R
  on every command and mode, under every invariance.
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
  `01-grammar.md`'s two grammar regions, the lexer's table among them, and
  the sources of its EBNF diagrams, `04-reference/diagrams/01-grammar-*.puml`
  (from `ein.l` and `ein.y`, `EIN_BLESS=1`);
  **every `.svg` under a `diagrams/` folder** (`utils/render_diagrams.py`,
  from the `.dot` or `.puml` beside it);
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
  `09-read-out-examples.md` (`utils/read_out_examples.py`); the rule regions
  of the `04-stdlib/` pages and the sources of the rules' drawings,
  `04-stdlib/diagrams/NN-MODULE.RULE.dot` (`utils/gen_stdlib_rule_pages.py`,
  which needs the binary, and draws their pictures); `06-config.md`'s
  defaults block and `docs/ein/ein.rs/04-api.md`'s `rust` block;
  `ein-core/src/printable.rs` (`utils/gen_unicode_printable.py`, with
  `ein-core/tests/golden/repr_escapes.txt` re-blessed). All of those have a
  `--check` or a test in the gate; `utils/check_generated.py` runs every
  generator's `--check`, and no crate test runs a generator. None names its
  generator; this list and `utils/README.md` are where the script is written
  down.
- **`ein.rs/vendor/lrpar/` is outside the workspace**: edit it only through a
  change to `ein.patch`. Nothing goes upstream.
- A change to what `ein solve` prints changes `08-read-out.md` in the same
  commit. A shape the grammar newly refuses gets a row in the grammar page's
  table.
- **A file that moves takes every reference to it along**, in the commit
  that moves it: `utils/doc_audit.py --links --check --tracked` reads each
  `.md`'s links, the links in a Rust comment and the bare paths in a `.ein`
  comment and in `corpus/corpus.toml`, and `--inbound --tracked --target
  PREFIX` counts them per target.
- **A page added to or taken out of `docs/ein/` renumbers its folder**: it
  takes the number of its place in the folder's `README.md`, the pages after
  it move, and every reference moves with them in the same commit
  (`utils/check_doc_blocks.py` fails on a gap or a number twice; `--list`
  prints every finding of the pages' rules, `--rule N` one rule's).
- **A diagram that is edited is drawn again in the same commit**, and one that
  is added or dropped takes its picture and its page's image line with it:
  `python3 utils/render_diagrams.py` draws what is missing or stale, and
  rule 1 of `utils/check_doc_blocks.py` fails on a source whose picture was
  not drawn from it as it stands, by the hash stamped on the picture's last
  line. **A picture is never compared with a second drawing**: its bytes
  depend on the Graphviz that drew it. A source the tool does not write
  (the standard library's rule drawings, the grammar's diagrams) changes
  through its generator, and then the tool draws it.
- **The release voice has no baseline**: `utils/check_release_voice.py` fails
  on any line of the project's prose (every project `.md`, every crate comment,
  every `.ein` comment, `corpus/corpus.toml` and `corpus/regressions.toml`)
  that carries a milestone, phase, stage or task id, a plan's finding, followup
  or review id, a case of the wrong-answer register, a path into the process,
  or a date. A decision id passes; a line that must keep one says
  `release-voice-ok: <reason>`.
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
  or `stdlib_coverage.rs` fails, and a region on its family's rules page
  under `04-stdlib/`, or `check_generated.py` fails. A rule or relation added to
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
EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test corpus_cli  # + the slow entries, which the_slow_set_is_exactly_these_sixteen names
EIN_ID_SEEDS=8 EIN_CORPUS_SLOW=1 cargo test … -p ein-render --test id_order_invariance   # the dev host's, every program
EIN_JOBS_SWEEP=2,4,8,16 cargo test … -p ein-render --test jobs_invariance
EIN_BLESS=1       cargo test … --workspace                   # re-bank the goldens
EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test … -p ein-render --test id_order_invariance   # the shape golden's every line, tests/ too
EIN_LEFTOVER=1    ein solve … --json-summary out.json        # the leftover probe
ein test examples tests stdlib --json-report r.json          # every claim's verdict, as JSON
ein test FILE --diff full                                    # each claim's models against what its run found: match, near (one fact off), missing, extra
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
python3 utils/check_release_voice.py --list [PATH …]         # every line the release-voice check finds
python3 utils/check_doc_blocks.py --list [--rule N] [PATH …]  # every finding of the pages' rules: diagrams as files, numbering, reasoning's languages, formulas, character diagrams
python3 utils/render_diagrams.py [--check | --all | --list] [PATH …]   # draw every diagram source that is missing its picture or changed since; --check draws nothing and reads the stamps
python3 utils/history_census.py                              # the history census: per tree, lines carrying an id, a plans/ reference, a date
```

The read-out's vocabulary is
[`docs/ein/user/04-reference/08-read-out.md`](docs/ein/user/04-reference/08-read-out.md),
every row `ein solve` prints and what it counts, and its golden page, written
by the binary, is
[`09-read-out-examples.md`](docs/ein/user/04-reference/09-read-out-examples.md), both
held to the printer by `ein-cli/tests/read_out_reference.rs`.
[`docs/ein/user/04-reference/06-config.md`](docs/ein/user/04-reference/06-config.md) is the census
of every `(config …)` field, CLI option and `EIN_*` name the binary reads, with a
*does it change the answer* column on every flag row, pinned by
`ein-cli/tests/config_reference.rs`. **The test harness's `EIN_*` names are the
process's**, and this table is their census: the test suite reads them, the binary
none, and setting one has no effect on any `ein` command. `grep -rhoE
'env::var(_os)?\(\s*"EIN_[A-Z0-9_]+' ein.rs/crates | grep -oE 'EIN_[A-Z0-9_]+' | sort -u`
re-takes the set, the binary's two among it.

| name | what it does |
|---|---|
| `EIN_BLESS` | `=1` rewrites a golden instead of comparing it, `06-config.md`'s generated block among them |
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
ten static checks (`--tests-only` skips them and the bench smoke), then
`cargo test --profile gate --workspace`, then the process's checks that need
the binary it built, then the bench smoke test. The tests run with
`TMPDIR` set to a fresh directory under `build/`, and
`utils/check_test_scratch.py` fails the gate on anything they leave there,
naming the file that writes it. **`cargo test --profile gate --workspace`
alone is not the gate**: it checks none of the ten, and none of what the
process holds itself to. **The tests read only the project**: what ties a
process file to the project is a `utils/` check and a gate step, never a crate
test — `check_gate_steps.py` (the step lists), `check_tool_mirrors.py` (the
editor grammar and the review driver against the engine's lists),
`check_generated.py` (every generator's `--check`) and
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
