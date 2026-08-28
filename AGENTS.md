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
  **Since M1d S1d.2.6 there is a fourth verdict word, `Open`** — a consistent,
  quiescent state with an obligation the program stated still unwitnessed,
  reported as *`Open — owes n (rel: n, …)`* with `k = 0` and the state itself
  under `open_states`. It is [`ideas.md`](docs/history/m1d_satisfiability/ideas.md)'s
  middle outcome and the distinction the other three could not draw: **no
  model** against **not yet a model**. It is **scoped** — only a program that
  *states* an obligation can reach it, so a state is judged by discharge when
  it has been told what it owes and by exhaustion when it has not, and 92 of
  the 121 corpus entries that reach a fixpoint report exactly the words they
  did before P1d.2
  ([openness_census.md](docs/history/m1d_satisfiability/openness_census.md)).
  `false` outranks it, a discharged model outranks it, it exits **0** like the
  other three, and it moved twelve entries and **no** exit code.
  **Since M1d S1d.3.3 an `Ambiguity` also says whether its `k` is a count or a
  floor** — `exhausted = false` prints *`solutions (k) 5   (a lower bound — the
  search did not exhaust)`* and *"distinct complete models **found**"*, which
  `Solution` had said as *"(not certified — pass --exhaustive)"* since ein.py
  and the verdict that reports a model *set* had not. The same stage shipped
  `ein solve --models key`, the set as its determining key
  ([the verdict](docs/history/m1d_satisfiability/the_verdict.md)). It also split
  two numbers that had always agreed: `verdict.k` counts *models* and
  `stats.solution_nodes` counts what the *search* recorded — S1d.2.6 changed
  the read-out and not the traversal, so no counter and no cost moved.
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
  `python_impl.md`. **Since M1e S1e.5.1 there is also a
  [configuration reference](docs/kernel/configuration.md)** — the 17
  `(config …)` flags, the 52 CLI options, the eight `EIN_*` names the binary
  actually reads, the precedence between the three, and on every flag row a
  *does it change the answer* and a *stability* column. Its defaults block is
  `ein solve --dump-config`'s own output (`EIN_BLESS=1` re-banks it), and two
  of the seventeen flags are documented there as **inert**: `print-alive` and
  `candidate-order-seed` are parsed, validated, dumped, `--json-summary`-echoed
  and `.einb`-round-tripped, and read by no code path
  ([Q-M1e.10](plans/m1e_review_processing/open_questions.md#q-m1e10--two-config--flags-are-inert)).
  **This tree is now the only statement of intent that is
  not also the implementation**, so it is load-bearing: a claim here is
  checked by `cargo test --workspace` and by nothing else — and since M1e
  T1e.1.1.1 what it takes to call one *settled* is
  [`standard_of_proof.md`](docs/kernel/standard_of_proof.md): a behaviour is
  refuted only by a banked probe, an absence by naming the thing that checks
  it, a **risk not by argument at all**, and an argument for leaving something
  alone holds only while its premise is enforced by something that fails when
  it stops being true.
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
- **`docs/history/`** — **shipped milestones, kept as record.** Three entries.
  [`m1d_satisfiability/`](docs/history/m1d_satisfiability/README.md) is the
  newest (2026-08-21 → 2026-08-27): **from saturation to satisfiability** — four
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
  The first and larger is [`m1a_rust/`](docs/history/m1a_rust/README.md), the Rust port
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
- **`tests/`** — **the ein-lang test suites**, and since M1c S1c.1.4 a third
  corpus root beside `examples/` and `stdlib/`: a `.ein` here with no
  `corpus.toml` entry fails the same completeness check. Its one subject is
  [`tests/stdlib/`](tests/README.md) — **56 programs, one per stdlib rule or
  tight family** plus one pair that is a *corner* rather than a rule
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
  counts are asserted in `ein-infer/tests/obligation_reports.rs`. **Since
  S1d.2.6 twelve of these programs report `Open` rather than `Solution`** and
  every one of their `(model …)` claims still holds unchanged, which is the
  same rule read from the other end: an expectation is about a fact set, and
  the facts an open state reached are the facts it reached. The coverage claim
  is measured by `utils/stdlib_census.py`, never by reading the directory, and
  the suite's sensitivity by a 51-mutant sweep it catches 50 of. **Since M1c
  S1c.1.5 two claims about this directory are in `cargo test`**
  (`ein-infer/tests/stdlib_coverage.rs`, 0.04 s): every stdlib rule is
  activated by a program *here* — **77 of 77**, with no `examples/` entry
  contributing — and every program here states an expectation. **Activation is
  two events since S1d.2.4**: `fire` for a saturation rule and `owe` for an
  obligation one, which can never produce a `fire`. Scoping the
  first to the suite is what found `transitive`, whose fixture was a two-cycle
  the `(neq ?a ?c)` guard refuses every match of; it grew a three-chain.
- **`corpus/`** — the **corpus**: `corpus.toml` (one entry per `.ein`, with
  the runs it is exercised under) plus `fuzz_findings/`. A file under
  `examples/`, `stdlib/` or `tests/` with no entry fails a completeness check. What
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
- **`utils/`** — **twenty-three scripts, all of them driving `ein.rs`** since M1a
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
  to close, and S1c.1.4 closed it: **0 of 73** on the re-take of 2026-08-24,
  180 entries and 557 runs
  ([§11](docs/history/m1c_external_validation/stdlib_census.md#11-the-re-take--2026-08-24-and-the-zero-set-is-empty)).
  `--check` exits 1 while any rule is at zero and is **not** the gate: S1c.1.5
  made that a cargo test, in-process and scoped to `tests/stdlib/`. What stays
  here is the measurement the gate is a yes/no of.
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
  enumerator would still propose, none of them an attribute arrow.
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
./run_tests.sh                                               # the whole per-commit tier
./run_tests.sh --tests-only                                  # skip the static checks
cargo test --manifest-path ein.rs/Cargo.toml --workspace     # the tests alone
EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test corpus_cli  # + the 2 slow entries
EIN_ID_SEEDS=8    cargo test … -p ein-render --test id_order_invariance
EIN_JOBS_SWEEP=2,4,8,16 cargo test … -p ein-render --test jobs_invariance
EIN_BLESS=1       cargo test … --workspace                   # re-bank the goldens
EIN_OBLIGATION_CHOICE=off|fail-first ein solve …             # the M1d S1d.2.5 rung levers
EIN_TRAVERSAL=tree ein solve …                              # the M1d S1d.10.6 traversal
ein solve … -e --models key                                  # the M1d S1d.3.3 form
EIN_LEFTOVER=1    ein solve … --json-summary out.json        # the M1d S1d.3.1 probe
ein test examples tests stdlib --json-report r.json          # the M1d S1d.4.1 read-out
```

**The seven `EIN_*` names in that block are the ones an agent needs, not the
set.** The shipped binary reads **eight** (three of them only in a non-default
build), the test harness reads ten more, `utils/` and the shell two — and nine
greppable `EIN_*` names are not environment variables at all, `EIN_RS` and
`EIN_RENDER_LEVI` among them. The whole census, classified, with the 17
`(config …)` flags and the 52 CLI options beside it and a *does it change the
answer* column on every flag row:
[`docs/kernel/configuration.md`](docs/kernel/configuration.md) (M1e S1e.5.1),
pinned by `ein-cli/tests/config_reference.rs`.

**`EIN_OBLIGATION_CHOICE`** is the obligations rung's measurement lever
(default `rule-order`): `fail-first` walks the owed instances smallest-set
first, and **`off` declines the rung altogether**, which is the pre-S1d.2.5
engine and the control arm every number in
[`hypotheses_from_obligations.md`](docs/history/m1d_satisfiability/hypotheses_from_obligations.md)
is measured against. It is deliberately not a `(config …)` field: `SolverConfig`
is rendered into the KB-shape digest, so a knob whose settings are being
compared would re-bless every shape golden in the corpus.

**`EIN_TRAVERSAL=tree`** is M1d
[S1d.10.6](docs/history/m1d_satisfiability/README.md#s1d106--the-traversal)'s
and a **second traversal beside the lattice**, off by default. It branches on
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

**`--models {list,key}`** is M1d
[S1d.3.3](docs/history/m1d_satisfiability/README.md#s1d33--the-verdict)'s
and the only *lever* in this block that is a flag, because it is presentation
rather than measurement. `key` prints a model **set** as its determining key —
the smallest set of slots that tells the models apart, and the table of
combinations that occur — instead of as *k* blocks: on
`examples/zebra2-minus-15.ein -e -m 3` that is **49 lines against 516**, the
same 4 columns and 32 rows `utils/model_set_census.py --form key` prints. It is
read by the `Ambiguity` arm alone and reaches nothing recorded, and where a key
is unaffordable — `examples/branching/06_lookahead_on.ein` needs 8 of 42 slots,
`C(42, 8) = 118 030 185` — it says so and **prints the models**, which is the
form the phase priced as (e). `list` stays the default because it is what
`ein solve` has always printed, not because the key is expensive — 42 ms to
decline on `branching/06`, under 1 % of the solve on the zebra family.

Beside it, and not a flag: **an `Ambiguity` now qualifies its own count.**
`exhausted = true` prints `solutions (k) 9`; `exhausted = false` prints
`solutions (k) 5   (a lower bound — the search did not exhaust)` and
*"distinct complete models **found**"*. It matters because the corpus was full
of the case — `ein solve -e examples/saturation/type-exclusivity/colors.ein`
printed **5** for a file with **9** models, and 5 of the 10 entries that answer
`Ambiguity` under their declared runs do it unexhausted. `Contradiction` is
deliberately untouched: a refutation said under a depth cap is
[Q-M1d.1](docs/history/m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)'s
question about a *word*, and `saturation/type-exclusivity/pets.ein` — `k = 0`
at `-m 5`…`-m 8`, **35 models** at `-m 10` — is the fixture that now waits for
it.

**`EIN_LEFTOVER=1`** is its neighbour and M1d S1d.3.1's probe: it fills
`--json-summary`'s **`leftover`** block with what the **blind** enumerator
would still propose at each recorded model or open state. `complete` means
*the active rung proposes nothing*, so a node the hrule or obligations rung
called complete can still have facts a guess could be about — and their count
is what separates *one model* from *2ⁿ models* when the reading is open-world
(`zebra2`'s **unique** model leaves **3 678**). It runs on a discarded fork,
which is what makes it a read where
[P1d.2 declined one](docs/history/m1d_satisfiability/hypotheses_from_obligations.md):
with the lever on and off, every field of every summary outside that block is
identical on all 121 entries that reach a fixpoint. Off by default because it
costs a generation pass per recorded state (≈40 ms on the zebra family) and
every corpus `solve` in `cargo test` writes a summary.

**A failing `:expect` now says why on stderr too**, M1d
[S1d.4.3](docs/history/m1d_satisfiability/the_vocabulary.md),
and it is one line: `<file>: :expect NOT CHECKED — expected Ambiguity with k =
2, got Solution with k = 1`. stdout is **unchanged** — the report block stays
under the solution table, because a false claim is a *result* and not a refusal
of the input — but an exit 1 with an empty stderr is a run nobody can diagnose
from a pipeline, which is what
`corpus_cli::every_refusal_carries_a_diagnostic` forbids and what `ein solve`
was producing. It is why `examples/features/11_expect_ambiguity.ein` could not
declare plain `solve`; it declares it now, and that cell is the corpus's only
witness for `Outcome::NotChecked`. **P1d.4 closed with no keyword**: tests stay
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
since M1c S1c.1.5 — five static checks (~5 s warm, `--tests-only` skips them
all), then `cargo test --workspace`, then the bench smoke test. **`cargo test
--workspace` alone is not the gate**: it checks none of the five, and until
S1c.1.5 neither did this script, which is why CI was red for three commits on
findings the local run reported as a pass. A local gate that is a subset of the
remote one is a local gate that lies — keep the two lists the same.

| step | what it found the first time it ran |
|---|---|
| `utils/stdlib_manifest.py` | — (it has always run in CI) |
| `utils/check_hashmap_iteration.py` | one finding, and the finding was the *check*: only the line immediately above a statement counted as `determinism-ok:`, so the second line of a two-line reason silently un-annotated it. It reads the whole comment block now |
| `cargo fmt --all --check` | **three** unformatted files, all from M1c's own `:expect` work |
| `cargo clippy --workspace --all-targets -D warnings` | a `for i in 0..n` indexing a slice, and **four** `&file` where `file` was already a `&str` — the four latent, because clippy stops at the first crate that fails and had never reached `ein-cli` |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | **twelve** unresolved intra-doc links and **seven** public items whose docs linked to a private one, plus a `<path>` read as an HTML tag. Nothing here had ever run rustdoc, whose default for all of it is a *warning* |

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

**The stdlib conformance gate** is `ein-infer/tests/stdlib_coverage.rs`, and
what it costs is 0.04 s:

```sh
cargo test … -p ein-infer --test stdlib_coverage   # 77 rules, 56 programs, fires + owes
ein test tests/                                    # the same suite, as a status code
```

It loads `stdlib/*.ein` for its rule heads, solves every `tests/stdlib/`
program the way `ein test` does with `--events` on, and fails on any rule no
program activated — so **adding a rule to the stdlib without a test fails the
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
