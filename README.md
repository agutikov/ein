# Ein

**Ein is a reasoner for logic puzzles and the problems that look like them.** A program states relations, facts
and rules in an S-expression language; the engine saturates the rules to a least fixpoint, guesses the facts the
program leaves open, and answers with the **region R**: every set of guessed facts whose state is consistent and
owes nothing. Every derived fact carries its provenance, so an answer can be explained in the program's own words,
and a contradiction names the facts it rests on.

Ein is built to be the symbolic half of a loop in which a language model proposes a formalization of an informal
problem (its atoms, relations, facts, rules and goal) and the reasoner judges it: a solution, the ambiguity that is
left, a contradiction with its core, a requirement nobody met. That loop is not in this repository; the reasoner is,
with its language, its standard library and its documentation.

## What it does

- **A program is a hypergraph written as text.** Its nodes are atoms, each a name, and facts, each joining a
  relation to its arguments, atoms or facts. Relations are declared with a label for each argument, and rules read
  facts (`:match`) and conclude facts, stored negations or `(false)` (`:assert`). Negation as failure, `(absent P)`,
  is judged once, at the closure's fixpoint. A relation property such as `(bijective pet-loc)` is a fact, and the
  standard library's rules give it meaning.
- **Saturation** fires the rules, semi-naively, to a least fixpoint. A fact never changes and is never removed.
- **Hypotheses** are the candidate facts the program leaves undecided: proposed by the program's `(hrule …)`, or
  read off what its obligations owe, or enumerated blind. A **commitment** is a set of them; its **state** is what
  saturation derives from root and the commitment.
- **The answer is R**: the commitments whose state is alive (no contradiction) and discharged (owes nothing),
  counted in states (`k_s`) and in commitments (`k_c`). Each question about R does the least work it needs, and
  `ein oracle`, R by brute force, is the reference every question is held to.
- **An empty R says why**: every node dead, with root's core and the `:source` of each core fact, or every alive
  node owing, with what it still owes.
- **A program can state what its answer should be** (`:expect`), and `ein test` checks it. Every answer exits 0;
  only a refuted claim exits 1.

## The running example

The Zebra puzzle, in two encodings that reach the same model: [`examples/zebra2.ein`](examples/zebra2.ein), one
relation per attribute and `std.bijection`, and [`examples/zebra.ein`](examples/zebra.ein), one generic `co-located`
relation and `std.slots`:

<!-- transcript: ein solve examples/zebra2.ein --single -->
```sh
$ ein solve examples/zebra2.ein --single
solve · examples/zebra2.ein --single
──────────────────────────────────────────────────────────────
  asked             is R one state? (--single)
  verdict           Solution — R is one state
  decided           yes — R holds no other state
  lattice           monotone (the static check) — R is convex: no hole
  universe          56 candidates
    outside         is-a*, next-to — no candidate: no hrule proposes them. R holds of them only what the program states and saturation derives
  route             the obligation tree, over states, branching on the owed instance with the fewest alternatives left, the first by text among equals — L1 dead is upward-closed · L2 discharged is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local · L5 an owed instance's alternatives are exhaustive · L8 what a state denies is dead above it
  work              7 saturations · 278 facts beyond root

  ;; state 1 of 1 — 18 candidates · from {(color-loc Green House-5), (drink-loc Juice House-4)}
  (model (color-loc Blue House-2) (color-loc Green House-5)
         (color-loc Ivory House-4) (color-loc Red House-3)
         (color-loc Yellow House-1) (drink-loc Coffee House-5)
         (drink-loc Juice House-4) (drink-loc Milk House-3)
         (drink-loc Tea House-2) (drink-loc Water House-1)
         (nation-loc Englishman House-3) (nation-loc Japanese House-5)
         (nation-loc Norwegian House-1) (nation-loc Spaniard House-4)
         (nation-loc Ukrainian House-2) (pet-loc Dog House-4)
         (pet-loc Fox House-1) (pet-loc Horse House-2) (pet-loc Snail House-3)
         (pet-loc Zebra House-5) (smoke-loc Chesterfields House-2)
         (smoke-loc Kools House-1) (smoke-loc Lucky_Strike House-4)
         (smoke-loc Old_Gold House-3) (smoke-loc Parliament House-5))

  query             (and (drink-loc Water ?h_water) (nation-loc ?who_water ?h_water) (pet-loc Zebra ?h_zebra) (nation-loc ?who_zebra ?h_zebra))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?h_water = House-1 · ?h_zebra = House-5 · ?who_water = Norwegian · ?who_zebra = Japanese
                        “The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”
  as :expect        (model (drink-loc Water House-1)
                           (nation-loc Japanese House-5)
                           (nation-loc Norwegian House-1)
                           (pet-loc Zebra House-5))
```
<!-- /transcript -->

Every word of that answer comes from the **puzzle**, not the engine: the `(query … :goal-text "…")` template renders
the headline from the goal variables, and `--single` decides that R holds no other state, which is what makes the
answer *the* answer. A bare `ein solve examples/zebra2.ein` describes all of R, and it is the same one state; its
`work` row says what that cost. The [guide](docs/ein/user/02-guide/README.md) builds this program from nothing, and
[the Zebra walkthrough](docs/ein/user/02-guide/06-zebra-walkthrough.md) pairs every clue of the published solution with
its fact and every human deduction with the rule firing that makes it.

## How it reasons

The engine fuses a deductive database, a constraint search and an assumption-based truth maintenance system, split at
the seam between what only grows and what a guess can retract
([firing a rule](docs/ein/reasoning/02-kb-evolution/01-firing.md#the-core-operations)):

| op | what Ein does | nearest analogs |
|---|---|---|
| O1 matching | bind a rule's premises against the knowledge base, structurally | relational join, RETE / LEAPS, worst-case-optimal joins |
| O2 saturation | fire rules to a least fixpoint, semi-naively | Datalog bottom-up evaluation, magic sets, differential dataflow |
| O3 negation as failure | `(absent P)` judged once, at the closure's fixpoint ([`absent`](docs/ein/reasoning/02-kb-evolution/03-absent-and-naf.md)) | stratified Datalog, the closed-world assumption, stable models |
| O4 equality | none of its own: equality is whatever relation a program states; `eq` and `neq` compare two objects as written | union-find, congruence closure, e-graphs |
| O5 contradiction | `(false)`, a fact beside its stored negation, or a rule that states `:ein inconsistent` | clause violation, tableau clash |
| O6 provenance and the core | every derivation recorded; the smallest set of givens and commitments a contradiction rests on ([cores](docs/ein/reasoning/02-kb-evolution/09-cores.md)) | ATMS / JTMS justifications, provenance semirings, MUS |
| O7 the candidate set and the lattice | the undecided candidates read off root's fixpoint, commitments as subsets of them ([the hypothesis space](docs/ein/reasoning/02-kb-evolution/05-hypothesis-space.md)) | DPLL decisions, Apriori, ATMS environments |
| O8 learning from a refutation | on a monotone program a superset of a dead commitment is dead, so one dead node decides every node above it ([finding R](docs/ein/reasoning/02-kb-evolution/07-finding-r.md)) | CDCL, backjumping, arc consistency |
| O9 state identity | two commitments whose worlds hold the same facts are one state; the identity is the sorted fact set | symmetry breaking, SBDS / SBDD |

A program whose rules read `absent` over a relation a guess can still extend may not be monotone. It loads as any
other program does. The static check flags it
([monotonicity](docs/ein/reasoning/02-kb-evolution/08-monotonicity.md)), every route then classifies the cube, and
every command that answers warns, in capitals, that the program is not monotone: of such a program the engine gives
one R, every command the same, and claims nothing of that answer
([Q-M1f.564](docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)).

## The standard library

[`stdlib/`](stdlib/README.md) is the single source of truth for `std.*`; the binary embeds a copy, identified by
`MANIFEST.sha256`. A property is stated as a fact and the module's rules apply it: `(transitive left-of)` is a theory
instantiated. Every rule has a program under [`tests/stdlib/`](tests/README.md) that activates it, and `ein test
tests/` runs them all. The per-symbol reference is [`04-stdlib/`](docs/ein/user/04-reference/04-stdlib/README.md).

| module | provides |
|---|---|
| `std.algebra` | the relation algebra: `converse`, `compose`, `identity`, `meet`, `join`, `difference`, `complement`; `functional`, `injective`, `total`, `surjective`, `bijective`; `symmetric`, `transitive`, `includes` closures |
| `std.bijection` | closed-world bijection inference read off the signature: negative completion, domain and range elimination, typecheck (`zebra2.ein`'s formulation) |
| `std.slots` | the same inference for one co-location relation whose classes are slots (`zebra.ein`'s formulation) |
| `std.elim` | closed-world domain elimination with positional markers |
| `std.closure` | a total function closed: `functional ∧ total ⇒ (__closed__ R)` |
| `std.typing` | a membership hierarchy, named once |
| `std.negation` | the elimination of a double negation, `(not (not g))` ⟹ `g`, one rule a count of arguments, activated a relation |
| `std.contradiction` | a fact beside its negation as a rule that concludes `(false)`, for one relation or for every declared one |
| `std.debts` | what an obligation requires and what pays it, as facts, and `(debts-discharged)` where nothing required is unpaid |
| `std.macro` | the `forall` and `unknown` pattern macros |
| `std.math.*` | the calculator: decimal and binary numerals, natural numbers, integers, rationals, gcd and lcm, conversion between bases, computed by saturation over digit tables. Ein has no numbers of its own |
| `std.bool.*` | the sixteen Boolean functions of two inputs, what is known of them, circuits, and a circuit's inputs as hypotheses |

## Build and run

```sh
cargo build --release --manifest-path ein.rs/Cargo.toml -p ein-cli
ein.rs/target/release/ein --version
```

[The install page](docs/ein/user/01-install.md) has the two channels (a release binary, `cargo install --path`), what
the five lines of `ein --version` say, and how to check which standard library a binary reads. The default build
links the `snmalloc` allocator and needs `cmake` and a C++ compiler; `--no-default-features` builds against the
system allocator, without `ein kb` and `--jobs`, and answers every program the same.

```sh
ein solve <file>                   # describe R: its states, lines, components, N
ein solve <file> --single          # is R one state? Solution / Ambiguity / Contradiction / Open
ein solve <file> --maximal         # the maximal line, the complete answers (--minimal: the minimal one)
ein solve <file> -n N              # N states, the least work that finds them
ein solve <file> --count           # how many: k_s and k_c, nothing listed
ein solve <file> --no-search       # root alone: R is not searched
ein solve <file> --trace out.md    # + a self-contained markdown derivation, in the program's :why texts
ein solve <file> --events e.jsonl  # + the narration stream, one JSON object per step
ein solve <file> --dump-answer DIR # + the read-out uncut, and every state saved: its program and the choices made
ein oracle <file>                  # R by brute force: the reference every question is held to
ein test <file-or-dir>…            # check the claims programs state about their own answers
ein saturate <file> [--dump]       # the least fixpoint alone
ein show hyps <file>               # H, the candidates, as Ein facts
ein render <view> <file>           # DOT: rules, constraints, the program, a KB, the lattice, H's structure
ein kb save <file> out.einb        # a loaded program as a binary container
```

Each flag names the part of R it asks about ([the command line](docs/ein/user/04-reference/05-cli.md)); every row the
read-out prints is in [the read-out reference](docs/ein/user/04-reference/08-read-out.md). `--region-budget F` bounds the
facts a description may derive (2²⁰ by default); a run that reaches it stops with an answer, its bounds and a
`note:`. `--jobs N` changes the wall clock and nothing else, and no configuration option, lever or flag changes R
([config](docs/ein/user/04-reference/06-config.md)). `.einb` is a private cache format, never an interchange one: every
command that takes a `.ein` path takes a `.einb` too, and answers the same.

The engine's own tests run with `cargo test --manifest-path ein.rs/Cargo.toml --workspace`, and need Graphviz on
`PATH`: the test that checks every DOT view is well-formed fails rather than skips without it.

## ein-lang at a glance

A `.ein` file is a **flat** sequence of S-expression forms, each classified by its head
([syntax](docs/ein/user/04-reference/02-syntax.md#2-top-level-forms)):

| head | role |
|---|---|
| `relation` | declare a relation, a label for each of its arguments, and the `:why` that reads the declaration |
| `rule` / `hrule` | a saturation rule / a hypothesis rule: `:match` → `:assert`, with a `:why` |
| `query` | a question about the answer: a `:goal`, a `:goal-text` template, and with `:expect` what the answer should be |
| `solve` | the problem's lattice, stated once (`:hrules`, `:hypothesis-relations`, `:no-hypothesis`, `:no-search`), and the expected region |
| `config` | the two solver flags |
| `import` / `macro` | bring a module into the program / a pattern alias expanded at load |
| `trace` | a derivation log the engine writes |
| *anything else* | a **fact**, `(pet-loc Zebra House-5)`, or its stored negation, `(not …)` |

Ein has no numbers, no ranges and no types of its own: `1` and `01` are two names, and arithmetic is a theory in the
standard library. A `:why` is required on every relation, rule, hrule and macro. The grammar is
[`01-grammar.md`](docs/ein/user/04-reference/01-grammar.md), and the names the language reserves are
[`03-reserved-names.md`](docs/ein/user/04-reference/03-reserved-names.md).

## Documentation

[`docs/ein/`](docs/ein/README.md) describes Ein as it ships, in three perspectives:

- [**user/**](docs/ein/user/README.md): the language, how to run `ein`, what the answer means and every row it
  prints, and [a guide](docs/ein/user/02-guide/README.md) that solves the Zebra puzzle;
- [**reasoning/**](docs/ein/reasoning/README.md): what a loaded program means, how its knowledge base grows, what
  the answer is and why every way of finding it gives the same one, true of any implementation;
- [**ein.rs/**](docs/ein/ein.rs/README.md): the Rust workspace, its crates and design contracts, and
  [the library API](docs/ein/ein.rs/04-api.md).

Beside them, [`docs/decisions/`](docs/decisions/README.md) holds every question about Ein that has an answer, as that
answer; among them, [Q-M1e.1](docs/decisions/docs/Q-M1e.1-a-behaviour-is-refuted-by-a-banked-probe.md) and
[Q-M1e.2](docs/decisions/docs/Q-M1e.2-an-argument-is-enough-only-when-its-premise-is.md) say what it takes to call a
claim on these pages settled.

## Layout

| path | what's in it |
|---|---|
| [`ein.rs/`](ein.rs/) | the implementation: a Cargo workspace of eight crates, six that ship (`ein-core`, `ein-ir`, `ein-infer`, `ein-einb`, `ein-render`, `ein-cli`) and two its tests use ([architecture](docs/ein/ein.rs/01-architecture.md)) |
| [`stdlib/`](stdlib/README.md) | the standard library, `std.*`, and its manifest |
| [`tests/`](tests/README.md) | the language's test suites: one program per stdlib rule or tight family, each stating what it should and should not derive, and programs over imports, the region and text |
| [`examples/`](examples/README.md) | the Zebra puzzle in its encodings, per-feature fixtures, the calculator's and the circuits' programs, regressions, and programs the loader must refuse |
| [`corpus/`](corpus/README.md) | `corpus.toml`, one entry per `.ein` with the runs it is exercised under; the fuzzer's findings |
| [`docs/`](docs/ein/README.md) | the documentation and the decisions; [`docs/demo/`](docs/demo/README.md) and [`docs/ein/overview/`](docs/ein/overview/README.md) for the article and the comparison with other systems |

Outside the project, as references: [`c/`](c/README.md), three plain-C Zebra baselines that differ only in how much
the search is told about the constraints; [`nl2smt/`](nl2smt/README.md), the puzzle's English sentences through a
parser, NLTK or link grammar, to an SMT solver; and [`docs/lib/`](docs/lib/README.md), a catalogue of the external reasoning systems,
solvers and research Ein relates to.

## License

MIT ([`LICENSE`](LICENSE)).
