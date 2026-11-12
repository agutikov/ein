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
  longer silently wins, and an unrecognised query keyword is refused (a parse
  error since M1f S1f.5.5's swap).
  **Since M1e2 S1e2.1a.5 a query's `:expect` lists its projection classes**
  over the states its `:over` names (`all`, `minimal`, `maximal`), and a claim
  about the whole state is the solve form's, `(solve … :expect (region (model
  …) (merged …) (owing …)))`; the several queries of a file are read off one
  run.
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
  `Solution` already said as *"(not certified — pass --exhaustive)"* and the
  verdict that reports a model *set* had not. The same stage shipped
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
  Two pages are worth knowing:
  [`defined_behaviour.md`](docs/kernel/defined_behaviour.md) — the thirteen
  diagnostics, orderings and error strings the engine defines normatively;
  and `inference/implementation.md` + `ir/02-data-model/03_implementation.md`,
  the module maps. **Since M1e S1e.5.1 there is also a
  [configuration reference](docs/kernel/configuration.md)** — the 2
  `(config …)` flags a program can set (fifteen more retired, nine at M1e2
  S1e2.1a.5, one at S1e2.1a.6, four at S1e2.1c.5 and `warn-derived-naf` at
  S1e2.2.5, each refused with the stage and why), the 82 CLI options, the two `EIN_*` names the binary
  actually reads, the
  precedence between the three, and on every flag row a *does it change the
  answer* and a *stability* column. Its defaults block is `ein solve
  --dump-config`'s own output (`EIN_BLESS=1` re-banks it). Two of the
  seventeen fields, `print-alive` and `candidate-order-seed`, were **inert**,
  read by no code path
  ([Q-M1e.10](docs/history/m1e_review_processing/open_questions.md#q-m1e10--two-config--flags-are-inert)),
  until S1e2.1a.5 retired them.
  **Since M1e2 S1e2.1a.3 (2026-10-16) the answer is a region**:
  [`inference/solution_semantics.md`](docs/kernel/inference/solution_semantics.md)
  defines it as *R = the commitments whose state is alive and discharged*,
  counted in states and in commitments (k_s, k_c), with the old *maximal alive
  commitment* as its § 2.6 special case. `ein solve`'s walk reports less than
  R, and the page's § 6 says where. The same stage gave `configuration.md` a
  § 2.5 and an `R?` column: *no config option changes R*, measured over every
  lever by `utils/region_census.py --levers`, and `enable-symmetric-mirror`
  was the one flag that did. The user ruled it retired on 2026-11-01, and
  S1e2.1a.6 retired it the same day: the mirror is always on, and the register
  records the case as W13, fixed. **Since S1e2.1a.6 the config principle is a
  test**: `ein-cli/tests/lever_invariance.rs` holds each mode's read-out and
  summary under every lever to the default's, on every program whose R is
  known, per commit.
  **S1e2.1a.9's route 1**
  ([`ein_infer::downset`](ein.rs/crates/ein-infer/src/downset.rs)) computes a
  monotone program's R from its alive set and N alone. It is held to the
  oracle node for node, and it counts `zebra.ein`'s R as one state of 261 982
  commitments. **The interface on top of it is
  [`solve_interface.md`](plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md)**,
  approved 2026-10-16. There is one tool for R, `ein solve`, whose modes each
  do the least work their question needs, and `ein oracle` for the cube. There
  is no `ein walk`: the user withdrew it, because two commands must never give
  inconsistent answers. **The modes are built**
  ([`ein_infer::explore`](ein.rs/crates/ein-infer/src/explore.rs)):
  `ein solve --minimal`, `--maximal`, `--single` and `--region-budget F`, each
  with `-n N`, and `ein oracle FILE [--dump DIR]`.
  `ein-infer/tests/explore.rs` holds every mode to the oracle. **Since
  S1e2.1a.5's switch (2026-10-18) a bare `ein solve` describes R**: the walk
  has no CLI path (its library stayed, uncalled, until P1e2.1c measured what of
  it the modes reuse, and **S1e2.1c.5 deleted it on 2026-11-03**, keeping the
  hypothesis generator the summary's `root` block called until S1e2.3.1 made
  that block root's fixpoint and H as the routes have them), its nine options are
  refused with what replaced each,
  the four verdict words are `--single`'s alone, every answer exits 0 and only
  a refuted claim exits 1, and
  [`corpus/wrong_answers.toml`](corpus/wrong_answers.toml) records 24 rows
  fixed and 10 open, each saying why. S1e2.1a.6 filed W13 and fixed it in the
  same commit, so it reads 25 and 10. The user's *forbid* of 2026-11-02 (S1e2.4.1:
  a refutation may not rest on an `absent` over a relation a commitment can
  still extend) re-opened W3's four with a refusal as `right`: 21 and 14.
  The review (S1e2.1.7) finished on 2026-11-03 with every row read: W12 closed
  by `solve_interface.md` § 0, `W1/alive-empty-interlayer` re-opened under
  *forbid*, C1 ruled not wrong, and W8's two `deferred` to F21 and kept open,
  so it reads 21 fixed, 13 open and 1 not wrong. P1e2.2's first stage,
  S1e2.2.4, fixed W4's two on 2026-11-04 by seeding the candidates with every
  constant a rule's `:assert` names (Q-M1e.21 (b)): 23 fixed, 11 open and 1
  not wrong. S1e2.2.5 built *forbid*'s refusal the same day, with the user's
  F2, *via, per activator* (a refutation resting on such an `absent` through a
  fact a rule derived under it counts too): every command that takes the
  universe refuses the program, `ein_infer::forbid`, and W3's four,
  `W1/alive-empty-interlayer` and `W8/stale-naf-fork` are fixed by the
  refusal: 29 fixed, 5 open and 1 not wrong, `W8/features-07` the one row
  deferred to F21. S1e2.2.14 filed W15 the same day, on the user's ruling on
  names (Q-M1e2.9 (a): a constant a fact names at any depth, or a rule's
  `:match` or `:assert`, is a candidate; one only a query names is not), as
  eight open rows, and fixed all eight the same day with one list of the
  program's names, `invariant::program_names`, which the blind enumerator's
  objects union: 37 fixed, 5 open and 1 not wrong. S1e2.2.15 filed W14 the
  same day (the obligations rung never proposes a domain value a commitment
  derives) as four open rows, and the user ruled Q-M1e2.10 (a) the same day:
  when an obligation's guard scans a relation a commitment can change (D),
  the rung declines and the blind enumerator builds H. All four are fixed by
  that decline: 41 fixed, 5 open and 1 not wrong. S1e2.2.7 fixed W7 the same
  day with Q-M1e.16's first fix: a rule application's identity interns its
  activator's every argument, as the compile key does, so an `int` and a
  nested fact in one activator position no longer lose a derivation, in
  either order and either build: 42 fixed, 4 open and 1 not wrong.
  S1e2.2.8 fixed W6's two rows the same day on the user's ruling of
  2026-11-03 (Q-M1e2.3 (a), *scoped, and the read-out says so*): R is over H,
  and every question names what H leaves out, an `outside` row under
  `universe` and `root.hypgen.outside` in the summary, with no answer moved:
  44 fixed, 2 open and 1 not wrong. S1e2.2.10 typed C1's witnesses,
  `branching/06` and `/07`, the same day (Q-M1e.12 (a), *the fixture, with
  the warning*): their R is two states, Blue in H5, and where the blind
  enumerator binds a type as an object every read-out says so, a `types` row
  under `universe` and `root.hypgen.types_as_objects`: still 44, 2 and 1.
  S1e2.2.11 fixed `W10/alt-cap-core` the same day, on the user's two rulings
  on Q-M1e2.11: a full alternatives list keeps beside it the arrival with the
  smallest closure (Q-M1e.15's (3)), so the `alt-cap-core` pair report one
  core; W10's second witness, a saving between two facts behind a full list
  that no bounded retention keeps, is `open` and deferred to F22; and
  `:record-alternative-justifications false` may change a dead node's core
  and nothing else, the config principle's one exemption: 45 fixed, 2 open
  (both deferred to a followup) and 1 not wrong. S1e2.2.12, the second of the
  user's two visits to W11, found no corpus core that moves with the interner
  once every core a run reports is compared, but a dead node's `refuted_by`
  that does, through a resumed fork's `FactId`-ordered delta, and filed
  `W11/fork-delta-order`, a program on which that order moves a core, and
  fixed it the same day: the delta is in the order the commitment was
  written, nothing a run reports moves under a permuted interner, narration
  included, and the id-order sweep compares every core: 46 fixed, 2 open
  (both deferred to a followup) and 1 not wrong. S1e2.2.16 filed W16 from S1e2.2.7's notes the same day: the compile key
  interned an activator argument's text, so the int `1` and the string `"1"`
  keyed one plan and the second activator's conclusions were lost, in either
  order; and *forbid* read an activator whose every argument is an int or a
  nested fact as none, so a refutation it refuses on a symbol activator
  loaded: 46 fixed, 6 open and 1 not wrong. It fixed all four rows the same
  day: the compile key is the activator's values, not their text, and
  *forbid* asks whether a plan has an activator by its rule's parameters: 50
  fixed, 2 open (both deferred to a followup) and 1 not wrong.
  S1e2.2.17 filed W17 from S1e2.2.16's notes the same day: *forbid* read a
  rule with parameters root has no plan of only where its activator relation
  is in D, and D's closure follows reads, not activators, so a refutation
  resting on an `absent` through an activator chain that first appears at a
  node loaded with a dead hole, five witnesses: 50 fixed, 7 open and 1 not
  wrong. It fixed all five the same day: *forbid* reads *extendable* as D
  followed through the rules root has no plan of, reads every such rule its
  activator relation reaches, and lets its own `absent` taint what it
  derives, with D, the static check and the rung's decline unmoved: 55
  fixed, 2 open (both deferred to a followup) and 1 not wrong.
  S1e2.2.18 filed W18 from S1e2.2.16's notes the same day: the rule compiler
  did not lower a string as the loader lowers it in a fact, so a string in a
  rule's `:match` matched nothing, a guard's equalled no value, an `absent`
  over one always held, an hrule's template proposed nothing, and one in an
  `:assert` stopped the run, five witnesses, the register's schema `/7`
  recording that stop: 55 fixed, 7 open and 1 not wrong. It fixed all five
  the same day: a string in a rule's `:match`, `:assert` or guard is the
  symbol it names, as in a fact, and a name of the program, and the line a
  conclusion no fact can hold still stops the run with is Ein's, the rule
  and the slot as written: 60 fixed, 2 open (both deferred to a followup)
  and 1 not wrong.
  S1e2.2.19 filed W19 from S1e2.2.17's notes the same day: a dead node's
  core leaves out the activator its refutation rests on, since a derivation
  records the premises its plan matched and not the activator the plan was
  compiled from, so a core names a fact root holds, or nothing, three
  witnesses, the register's schema `/8` recording N's cores: 60 fixed, 5
  open and 1 not wrong. Which activators a core follows is Q-M1e2.12, put to
  the user: following every activator moves the cores of 40 corpus
  programs, a derived one those of three, and one only a node derives none
  (S1e2.2.19's sweep, 2026-11-04). The user ruled (c) the same day, and
  S1e2.2.19 fixed all three: a firing records the activator its plan was
  compiled from, and a dead node's core follows one a node derived or
  committed, never root's: 63 fixed, 2 open (both deferred to a followup)
  and 1 not wrong.
  S1e2.2.20 filed W20 on 2026-11-05, on the user's ruling of 2026-11-04
  (Q-M1e2.13 (a): `_` in a rule's pattern matches any value, head and
  argument; a range in a rule is refused at load): the rule compiler lowers
  both as a slot that matches nothing, five witnesses, a premise, a head
  `_` that stops compilation, a range that loads in silence, an `absent`
  and an hrule: 63 fixed, 7 open and 1 not wrong. How a head `_` matches
  every relation is Q-M1e2.14, put to the user: it needs a second path, a
  plan per relation or a step over every relation, and the agent recommends
  refusing it at load (S1e2.2.20's census, 2026-11-05). The user ruled (c)
  the same day, and S1e2.2.20 fixed all five: `_` in a rule's pattern is a
  slot that matches any value and binds nothing, and a range, or a `_`
  written as a relation, is refused at load, one line in Ein's spelling per
  slot: 68 fixed, 2 open (both deferred to a followup) and 1 not wrong.
  S1e2.2.21 filed W21 the same day, from S1e2.2.19's notes: a pair
  contradiction's core names only its positive side, since the core search
  explains each contradiction by its witness, a pair's positive `X`, so where
  `X` is root's the core names none of the commitment; four rows, two
  witnesses and two corpus programs, the rest held by the stage's
  corpus-wide test: 68 fixed, 6 open and 1 not wrong. Explaining a pair's
  two sides together moved other cores the wrong way, through refutations
  that rest on a `forall`, whose support a firing does not record. What a
  `forall` rests on is Q-M1e2.15, put to the user, and the agent recommends
  following its support at each fact's smallest frontier (S1e2.2.21's
  sweep, 2026-11-05). The user ruled (b) the same day, and S1e2.2.21 fixed
  W21: a contradiction's facts are explained together, and a firing records
  the facts its universals read, each folded in at its own smallest
  frontier; the `forall` gap is two more rows of W21, filed and fixed in the
  same commit: 74 fixed, 2 open (both deferred to a followup) and 1 not
  wrong. With it the user ruled that a core is taken only where a surface
  prints one (*minimum work per question*): a bare `ein solve` takes root's
  and the minimal holes', and no run moved (S1e2.2.21's sweep, 10 752 runs,
  2026-11-05); `ein-infer/tests/corpus_cores.rs` holds that every core the
  corpus reports names its commitment and, with root, derives its
  contradiction.
  S1e2.2.22 filed W22 the same day, from the inventory of 2026-11-05's F1 (on
  Q-M1e2.16): the parser read a reserved word as a prefix of a name, so a
  name that starts with one was the keyword and the rest of the name, and
  `(neqw ?x)` in a rule's `:assert` stored `(neq w a)`; fourteen rows, one
  per place the split changes an answer, each witness with its twin: 74
  fixed, 16 open and 1 not wrong. Where a keyword ends before `-` and a
  digit, and what a trace event reads as once a keyword is a whole word, is
  Q-M1e2.17, put to the user and ruled the same day as the agent
  recommended: a keyword ends at the word boundary `00_ebnf.md` § 1 states,
  so `(neq-1 ?x)` stays `neq` and `-1`, and a trace word read whole, which
  has no other reading, is a parse error. S1e2.2.22 fixed all fourteen the same
  day: the parser reads a reserved word only as a whole word, so a name that
  starts with one is the `SYMBOL` it spells and each witness answers its
  twin's R: 88 fixed, 2 open (both deferred to a followup) and 1 not wrong.
  S1e2.2.23 filed W23 the same day, from the inventory's F2 (on Q-M1e2.16):
  a core search cut at its budget (`explain::ExplanationBudget`, 64
  environments per fact and 64 rounds) printed its core with nothing to say
  so, since `region.rs` dropped `Explanation::exhausted`; one row, schema
  `/9` (`core_status`), the witness's ten-fact core where eight suffice: 88
  fixed, 3 open and 1 not wrong. The corpus cuts 19 of the 5 442 cores
  `--dump-answer` reports for N, `zebra.ein`'s six at the 64 rounds and the
  `zebra2` family's at the environments, and a budget sixteen times larger
  gives each the same core at no measurable cost (`ein solve FILE
  --dump-answer DIR`, 2026-11-05). It fixed the row the same day: the
  search names the cap it stopped at (`explain::Cut`), and a core's status
  (`explain::CoreStatus`) is carried beside every core a surface prints, a
  word in every JSON (`core_status`: `within-budget`, `cut-` and the caps,
  `null` where no core was taken) and a sentence in every text surface,
  which `ein-infer/tests/corpus_cores.rs` holds over the corpus: 89 fixed, 2
  open (both deferred to a followup) and 1 not wrong.
  S1e2.2.24 filed W24 the same day, from the inventory's W-12, W-13 and W-14
  (on Q-M1e2.16 (1): refuse each at load, as a range and a head `_` are):
  `_` as a guard's argument, a premise that is not a form, and a pattern
  inside a pattern whose relation no parameter binds, each reading nothing
  in silence; three rows, each witness the inventory's probe: 89 fixed, 5
  open and 1 not wrong. Its step 1 found four more shapes that read nothing,
  a form, a keyword pair and a variable no earlier premise binds as a
  guard's argument, and a relation parameter its activator binds to a nested
  fact, none written by any program, and one corpus fixture that writes the
  second shape, `examples/broken/compile/empty_absent.ein`, refused either
  way; they are Q-M1e2.18, put to the user (step 1's sweep, 693 files and
  18 018 runs, 2026-11-05). The user ruled every part as recommended the
  same day, and S1e2.2.24 fixed W24: the loader refuses each shape at load,
  one line per slot in Ein's spelling naming the rule and the clause, and the
  compiler a relation parameter a rule binds to a nested fact through a
  variable; the ruling's four are rows filed and fixed in the same commit,
  and the two compile-negative fixtures moved to `examples/broken/load/`:
  96 fixed, 2 open (both deferred to a followup) and 1 not wrong.
  S1e2.2.25 filed eight rows the same day, on Q-M1e2.16 (5): every live
  wrong answer the inventory found with an owner and no row, `open` and
  deferred to that owner, so the close names it — three W8 rows, F21's
  (`features/07` with its two rules swapped, `lattice/10`'s FIFO gap, and
  stale members of R past the oracle's limit on `corner-house`), three W10
  rows, F22's (`fork-delta-order`'s smallest but not canonical core, the
  closure estimate counting a shared given twice, and a `zebra2.ein` core
  taken after a second preparation of root), W25, F22's (a premise-less
  derivation never recorded as an alternative), and W26, M1f S1f.10.6's (an
  obligation whose activator a node derives, Q-M1e.11), W-10 and W-15 shown
  by the stage's probes; schema `/10`: 96 fixed, 10 open (every one
  deferred) and 1 not wrong. Its re-take found two modes disagreeing, which
  is a finding and not a deferred row: past `--region-limit` on a program
  the static check flags, `--minimal` and `--maximal` label every member
  met as on their line (65 214 states on `square-unique/corner-house`,
  2026-11-05), where describe certifies one minimal state and says the
  maximal line is not known. The user ruled it the same day as recommended
  (Q-M1e2.19): S1e2.2.26 files it as W27 and fixes the line modes to
  certify what describe certifies, before P1e2.3's S1e2.3.8–.10 and the
  close. F21, F22 and M1f's S1f.10.6 each list the rows they own, and the
  milestone's acceptance names all ten.
  S1e2.2.26 filed W27 the same day, on Q-M1e2.19: four rows, the smallest
  program the static check flags past the default limit,
  `examples/ein-bugs/wrong-answers/past-limit-lines.ein` (17 candidates, R
  three states), and `lattice/10_fifo_gap.ein --region-limit 1`, each under
  `--minimal` and `--maximal`; schema `/11` (`others`, the members a run met
  and does not list): 96 fixed, 14 open and 1 not wrong. Its re-take found
  no monotone route that mislabels (227 programs under six budgets) and no
  line member describe certifies that the oracle does not; under a lowered
  limit the line modes label a state off the oracle's line on 20 of the 22
  flagged programs the oracle reaches (2026-11-05). It fixed all four the
  same day: past the limit a line mode answers from describe's partial
  description (`explore::flagged`, `Space::partial_cube`), so `--minimal`
  lists the members describe certifies on the minimal line and every
  `--maximal` question lists none, the maximal line not known, each counting
  the other members met, which `--dump-answer` writes;
  `ein-render/tests/line_modes_past_the_limit.rs` holds every line mode past
  the limit to describe and to the oracle: 100 fixed, 10 open (every one
  deferred to a followup or to M1f) and 1 not wrong. With it P1e2.2 is
  complete; next are P1e2.3's S1e2.3.8–.10, then the inventory re-taken and
  shown to the user, and the close waits for the user's go-ahead.
  S1e2.3.8 (N5, the fact printer) paused on its step 1's finding: a claim
  and a query's projection classes compare printed text, which printed the
  int `1` and the name `"1"` alike, so a claim about the int held of a state
  holding the name. The user ruled Q-M1e2.20 (a) on 2026-11-06, and the
  stage filed W28: three rows, `claim-int-name`, `-solve` and
  `classes-int-name` under `examples/ein-bugs/wrong-answers/`; schema `/12`
  (`claim` and `queries`, a claim's outcome and a query's classes, a refuted
  claim's exit 1): 100 fixed, 13 open and 1 not wrong. It fixed all three the
  same day with the one printer (`ein_core::text`, `Terms::display`): a value
  prints as Ein text the loader reads back to the same value, a name bare
  only where it lexes as one `SYMBOL`, so the int `1` and the name `"1"`
  print apart on every surface, the claims are FAILED and the classes four;
  a `--dump-answer` state file writes one fact a line, its provenance a
  comment, and reloads to its state by value
  (`ein-cli/tests/answer_round_trip.rs`, every corpus program): 103 fixed,
  10 open (every one deferred to a followup or to M1f) and 1 not wrong.
  S1e2.3.8 is done (`712f23fd`); next are P1e2.3's S1e2.3.9 and S1e2.3.10, then
  the inventory re-taken and shown to the user, and the close waits for the
  user's go-ahead.
  S1e2.3.9 (N6) is done (`b6139e62`): past `--region-limit` a run keeps each node
  it met as its two bits, and `nogoods.json` wrote each member of N it met
  with an empty core, which reads as *refuted by nothing*; it writes a
  `null` core and rules beside `core_status: "not-kept"`, and `--trace`
  says why (since M1f S1f.5.12 each member gets its core after the walk, and
  `not-kept` is retired). No run that did not stop prints an empty core
  (`ein-cli/tests/answer_round_trip.rs`; since M1f S1f.5.9 but ∅'s, whose
  refutation rests on no fact), and the register is unmoved. Next
  is S1e2.3.10, then the inventory re-taken and shown to the user, and the
  close waits for the user's go-ahead.
  S1e2.3.10 started the same day: N8 is fixed, every row of `ein test
  --json-report` carrying `outside` and `types_as_objects` as the summary
  writes them (`ein-test-report/4`). N7, a dead node's `refuted by` naming
  the first contradiction's rules rather than its core's, paused on its
  step 1's finding: 5 041 printed cores per commit derive their
  contradiction through two rules, and which one to name is
  [Q-M1e2.21](plans/m1e2_correctness/open_questions.md#q-m1e221--which-rule-does-refuted-by-name-where-the-core-derives-its-contradiction-through-two),
  put to the user. No answer moved.
  The user ruled it (a) the same day, and S1e2.3.10 is done (`a3bb9e70`):
  `refuted by` names, for each fact of the contradiction the core was taken
  over, the first justification the store records whose premises the core
  derives, `(given)` for a fact the core holds, so `alt-cap-core.ein` and
  its twin both say `wide`; `:record-alternative-justifications false`
  moves it only where it moves the core, and two programs one `:priority`
  apart naming two rules beside one core is the accepted cost.
  `ein-infer/tests/corpus_cores.rs` holds every printed core's rules over
  the corpus; no answer or register row moved. With it P1e2.3 is complete.
  The inventory was re-taken on 2026-11-06 and shown to the user, whose review
  of its no-owner items ruled every one by 2026-11-08 (Q-M1e2.22 to .28). The
  user then moved those plans to M1f as P1f.5, and **M1e2 closed on
  2026-11-08**, with no `docs/history/` record and `plans/m1e2_correctness/`
  kept in place, on the user's instruction
  ([§ The close](plans/m1e2_correctness/README.md#the-close-2026-11-08)). The
  register reads 103 fixed, 1 not wrong and 10 open, each deferred to F21, F22
  or M1f's S1f.10.6. Twenty known wrong answers have no row yet, and
  [P1f.5](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/README.md)'s
  first stage, S1f.5.0, files them. **M1f is next**, P1f.5 first.
  S1f.5.0 filed them on 2026-11-08 as W29 to W32, twenty `open` rows each
  owned by the P1f.5 stage that fixes it (S1f.5.1, .7, .9 and .11), and
  three more the user added from S1f.5.5's census, W33 to W35 (S1f.5.18's,
  S1f.5.9's and S1f.5.5's), each `now` today's answer on a witness under
  `examples/ein-bugs/wrong-answers/`, under the register's schema `/13`
  (beside a refusal, a `now` may record the core and the claims the refusal
  replaces), so the register reads 103 fixed, 33 open and 1 not wrong
  (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-08).
  `W30/assert-neq-w-x`'s right, R {∅}, is the G-21 ruling's reading:
  `(neq w w)` at `{(p w)}` is a false conclusion, so S1f.5.7's table, which
  said two states, was corrected to match (`cdbd0970`).
  S1f.5.3 retired the `RANGE` token the same day (Q-M1e2.24: Ein has no
  numbers and no ranges, and none is planned): `1..5` was a parse error until
  S1f.5.5's swap made it one name, so the two corpus files that write one,
  `syntax/arg-kinds` and `wrong-answers/range-in-match`, were refused at the
  parse in between, named in `ein_corpus::REFUSED_UNTIL_S1F_5_4`, which
  `corpus_cli` held both ways and the swap emptied and deleted.
  The same day the user re-planned the rest of P1f.5's language work (*"It is
  the same work done twice, why not switch to grmtools first and remove
  hand-written parser and then verify requirement met (strings, names,
  etc...) ?"*): S1f.5.5's generated parser is built beside the hand-written
  one and swapped in first, every answer it moves attributed to S1f.5.4's,
  S1f.5.1's, S1f.5.2's or S1f.5.11's ruling or to the paper grammar's own
  before the old parser goes, so the swap and not S1f.5.4 empties that list, and
  S1f.5.4's step 1, done that day (46 of 511 programs move on its
  prototype), is the attribution's first cross-check
  ([P1f.5 § The order](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/README.md#the-order)).
  Its T3 put that parser beside the old one the same day: `ein_ir::grammar`,
  generated by `ein-ir`'s `build.rs` from `src/ein.l` and `src/ein.y`, reached
  from its tests and from the `generated-parser` cargo feature, off in every
  shipped build, whose grammar has no `:priority` (S1f.5.15 deletes it before
  the swap) and lets a macro carry a `:why` (both the user's, 2026-11-08); and
  `ein.rs/vendor/lrpar/`, grmtools' `lrpar` 0.15.0 with
  `ein.patch`, which makes recovery deterministic (a budget of 30 000 search
  steps, not 500 ms). It is outside the workspace on purpose: no gate check
  formats or lints it, and nothing here should edit it but a change to the patch
  ([S1f.5.5 § T3 record](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#t3-record-2026-11-08)).
  The user's rulings of that day make the generated grammar the phase's
  final one from the start (*"collect all requirement for the parser in this
  phase and implement grammar that will already meet all of them"*):
  S1f.5.15, which removes `:priority`, runs after T3 and before the swap
  (*"S1f.5.15 before swap"*); `:why` stays optional in the grammar and a
  loader flag, default false, makes it required, which S1f.5.14, now the
  phase's last stage, turns to true once every program carries one (*"the
  flag only decides loading"*, so it moves no answer); and the patch is not
  sent upstream (*"don't send the lrpar patch upstream either"*)
  ([P1f.5 § The order](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/README.md#the-order)).
  S1f.5.15's T1 ran the same day and stopped before any removal: every
  `:priority` taken out of the corpus moves no R, N, H, claim or exit code on
  the 511 programs, but moves the facts of cores on seven, through the
  alternatives cap, `zebra.ein`'s `:record-alternative-justifications false`
  and W25, and shows a config-principle breach that exists today,
  `features/12`'s `refuted by` moving under that flag beside one core
  ([§ T1 record](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md#t1-record-2026-11-08)).
  The user ruled both the same day (`3998a1c1`): Q-M1f.4 (b), *"Derive order
  first"*, a band derived from each rule's shape orders the closure's agenda,
  the boundary FIFO, and the sweep is re-taken before any removal, whatever
  still moves going back to the user; and Q-M1f.5 (a), *"Register row W36"*,
  filed by S1f.5.15 as `W36/alt-flag-refuted-by`, `open` and owned by F22,
  under the register's schema `/14` (a `now` records what root's `refuted by`
  names), which `lever_invariance` exempts until F22 fixes it, and a second
  W36 row the same day, `W36/alt-flag-two-derivations`, a witness no rule order
  hides: 103 fixed, 35 open and 1 not wrong (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-08). Its T3 built the rule dependency graph over the
  plans a run compiled and its strata (`ein_infer::strata`), and a band per
  rule from its shape (275 of 471 declared rules agreed); measured, a derived
  agenda held every zebra-family core and the FIFO boundary moved them
  (Q-M1f.6). The user ruled *"plain FIFO, core is an explanation, F22 makes it
  canonical"* (`f9148334`), so the band never shipped. **T4** added the static
  non-stratifiability diagnostic, a diagnostic and not a refusal: a `strata`
  row in `ein solve`'s read-out and a `;; Strata` line in `ein saturate
  --dump`, each cycle through an `absent` named (`ccc4e8ca`). **T5 removed
  `:priority`** (`a3cb1753`): the agenda and the boundary are FIFO, the written
  and arrival order; 475 clauses left 136 corpus files, and a test holds that
  none writes one; a `:priority` written anyway is dropped by the hand-written
  loader (D-15) until S1f.5.5's swap refuses it. It moved no R, N, H, claim,
  exit code or stored fact, and moved cores and `refuted by` exactly as T1's
  FIFO arm did (`zebra.ein` 292 of 458, the zebra2 family's cap-decided cores),
  each accepted as an explanation and F22's to make canonical; W36 stays open,
  `features/12` showing it too
  ([§ T5 record](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md#t5-record-2026-11-08)).
  S1f.5.14's loader flag landed early the same day (*"yes, land the :why flag
  early"*): `--require-why`, on the seven commands that load a program,
  refuses a relation, rule or hrule with no `:why`, one line each, and a macro
  only in a `generated-parser` build until S1f.5.5's swap deletes that `cfg`;
  default false, so nothing moved, and with it on 274 of the corpus's 339
  loadable files are refused (2026-11-08,
  [§ T1f.5.14.6 record](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.14_nl_required.md#t1f5146-record-2026-11-08)).
  S1f.5.16, the read-out reference, was specified the same day (*"Ok, schedule
  S1f.5.16 also"*): one live page, `docs/kernel/inference/read_out.md`, for
  every row and number `ein solve` prints and whether it counts R, what lies
  around R, or the run's work, with the read-out's golden page generated beside
  it and both held to the printer by `ein-cli/tests/read_out_reference.rs`; it
  runs after the swap's non-grammar halves and before S1f.5.7, so the stages
  that change the read-out write the live page, and M1e2's `read_out.md` stays
  the record
  ([S1f.5.16](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md)).
  **It landed on 2026-11-09**: the page is
  [`docs/kernel/inference/read_out.md`](docs/kernel/inference/read_out.md),
  every label the printer has is a variant of `ein_render::read_out::Row` and a
  row of one of its tables, and a stage that changes what `ein solve` prints
  changes the page in the same commit, or `read_out_reference.rs` fails.
  S1f.5.5's T8, the swap's prerequisite, renamed the names the parser and the
  loader make up out of the name space the same day (S1f.5.4's Q1 (a)):
  `#empty`, `#nested`, `#params`, `#query` and the generated parser's
  `#list`, where `@empty`, `<nested>`, `@params`, `<query>` and `@list` were
  names under the name rule, so `(p ())` and `(p (@empty))` would have loaded
  as one fact; `ein_core::text::INTERNAL_NAMES` lists them,
  `generated_parser::no_name_the_loader_makes_up_is_a_name` holds them, and
  no answer moved, only W34's candidate's spelling and the renderings that
  print a renamed head
  ([§ T8 record](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#t8-record-2026-11-08)).
  Its T9 review stop, ruled by the user the same day, found a wrong answer with
  no row, which T10 filed as `W37/template-var-star`, `open` and owned by
  S1f.5.4, under the register's schema `/15` (a `now` records the sentences a
  query's `:goal-text` prints): a template reference cannot name a variable
  holding a character other than a letter, a digit, `_` or `-`, so `{?n*}`
  prints as written, as the stdlib's three `{?isR*}` `:why`s do on `--trace`:
  103 fixed, 36 open and 1 not wrong (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-08).
  T10 also moved `syntax/wrappers`, the one fixture rewrite that moves no
  answer on today's parser, and S1f.5.15's T1f.5.15.9 took `:priority` out
  of `utils/region_synth.py`, `guard_split.py` and `fuzz_engine.py`. T11,
  the engine's five names reserved, stopped on
  [Q-M1f.7](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f7--where-may-a-program-write-the-kernels-dunders),
  which the user ruled (b), *"Never bound"*, and landed the same day: the
  dunders `__closed__` and `__symmetric__` are in `ein_core::RESERVED`, so
  a program may not bind either (a declared relation, a rule's, an hrule's
  or a macro's name, an import's `:as`) and uses either as a relation or a
  value, `(imply1 functional __closed__)` among them; the three writeback
  names are the generated grammar's `WRITEBACK` token, refused wherever a
  program writes them (`ein-ir/tests/engine_names.rs`).
  **The swap, T4, landed on 2026-11-09**: the generated parser is the only
  one, `lex.rs`, `parse.rs` and the `generated-parser` feature gone, and
  `ein_ir::parse` is `ein_ir::grammar::parse`, whose `ParseError` reports every
  error recovery reaches, each with its line and column, its notes (S1f.5.1's
  `_`, S1f.5.2's string, a query's lattice keyword and a retired `(config …)`
  flag) and three ranked repairs then a count, the eleven words a position
  gives a meaning to folded into *a name*. It carries the name rule (S1f.5.4:
  `(p 1) (p 01)` two facts, printed bare, `ein_core::text::is_symbol` held to
  `grammar::is_one_name`), a bare `_` refused (S1f.5.1), a string only after
  `:why`, `:source` or `:goal-text` (S1f.5.2), closed keywords (S1f.5.11) and
  the paper grammar's defaults; every name a JSON string in the summary's
  bindings (Q4); a macro's `:why` asked for by `--require-why` in every build,
  the stdlib's two macros carrying theirs; and `ein_corpus::REFUSED_UNTIL_S1F_5_4`
  deleted. It moved exactly T9's list
  ([§ T4 record](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#t4-record-2026-11-09)):
  twenty-five refusal fixtures moved to `examples/broken/` with
  `kwpair-below-the-filter`, `relation_parameter_int` and the two
  `activator-text` controls retired, `derivation_cycle` and four `syntax/`
  fixtures rewritten (`arg-kinds` scoped, R {∅}), the `binding-key-*`
  witnesses scoped; and the register's W29, W32 and W35 rows flipped to
  `fixed`, 23 fixed rows restated: 117 fixed, 22 open and 1 not wrong
  (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).
  A `.einb` opens in under 6 ms since, not one (design/10 § 6 restated, the
  user's *"Restate the budget"*). T12 bought back most of the swap's cost
  inside grmtools the same day, moving no token: one skip rule and a start
  condition in `ein.l` and a lexer per thread halve the lexing, so `ein
  saturate examples/zebra.ein` takes 9.0 ms where T4's took 11.9 and the
  hand-written parser's 4.7, and the gate 15 min 53 s where T4's took
  19 min 59 s; a patch to `lrlex` or a hand-written lexer is parked for the
  close
  ([Q-M1f.8](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f8--the-parsers-remaining-cost-within-grmtools-or-past-it),
  [§ T12 record](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#t12-record-2026-11-09)).
  **T5 (2026-11-09) made `00_ebnf.md`'s two grammar blocks generated**: they
  are `ein.l` and `ein.y` as `ein-ir/tests/grammar_pages.rs` reads them through
  grmtools, so edit the grammar and re-bless (`EIN_BLESS=1`), never the blocks;
  the same file's table holds each refusal the grammar took over from the
  loader, the 55 loader lines it covers are deleted
  ([§ T5 record](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#t5-record-2026-11-09)),
  and a stage that makes the grammar refuse a shape adds a row there.
  T6 (the same day) reads a `--diff-models` file's facts with the parser and
  compares them by value, so a fact's spelling no longer decides a match.
  **S1f.5.4 ran to its end the same day.** Its T1f.5.4.3 (`410a3388`)
  retired the int kind: a `Value` is a
  name or a nested fact, and `Tag::Int`, the int pool and `Node::Int` are
  gone, each path shown unreachable from any program first; a `.einb` keeps
  its `INTS` section, empty, and refuses one that holds an int, with no
  format bump
  ([Q-M1f.10](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f10--the-einb-ints-section-once-the-int-kind-is-gone),
  defaulted); no answer moved
  ([§ T1f.5.4.3 record](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md#t1f543-record-2026-11-09)).
  Its T1f.5.4.5 (`eccf4186`) fixed `W37/template-var-star`: a template
  reference reads the name rule's characters up to the first `}`, so the
  stdlib's `{?isR*}` and every variable the name rule spells are filled, and
  the loader refuses a template naming a variable that holds `}`, which no
  reference can reach
  ([Q-M1f.11](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f11--a-template-reference-to-a-variable-no-reference-can-name),
  defaulted): 118 fixed, 21 open and 1 not wrong (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09). Its T1f.5.4.4
  restated every page that described an `INT` token or an int value as live.
  **S1f.5.2 ran to its end the same day**: a `\u{…}` that names no character
  is refused at the parse (W38), the grammar's table refuses a string in every
  place a value goes, `tests/text/01_unicode.ein` and
  `ein-cli/tests/natural_language_text.rs` hold natural-language text byte for
  byte on every surface that carries it, and no string is a value anywhere in
  the engine
  ([S1f.5.2](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.2_strings_are_natural_language.md)):
  119 fixed, 21 open and 1 not wrong (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).
  **S1f.5.1 ran to its end the same day**: F3's cause is a rule
  application's identity, its register file (`firing::BindingKey`), which
  drops a second match that binds what the first did, and it reaches a
  top-level `(or …)` whose disjuncts bind the same registers, filed
  `W39/or-one-key`, `open` and F22's (`728e716a`); `_` and `?_` are held
  refused in every position on every command (`307021db`), and `Slot::Any`
  and `Node::Wildcard` are gone (`ef5374a1`)
  ([S1f.5.1](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.1_bare_underscore_refused.md)):
  119 fixed, 22 open and 1 not wrong (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).
  **S1f.5.11 ran to its end the same day**: nothing in the compiler or the
  macro expander drops a keyword pair, and a pair inside a term is an
  `unreachable!` naming the grammar where a match needs an arm, each path
  shown unreachable first, the walkers that descend into a pair's value kept
  ([Q-M1f.14](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f14--which-keyword-pair-arms-go-once-no-term-holds-one),
  defaulted); the grammar's table refuses a keyword in each form and each
  place in a term, `fuzz_properties` holds each form's list on every program
  it accepts, `01_grammar.md` carries the lists, and no current kernel page
  describes `:where` as live
  ([S1f.5.11](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md)):
  the register unmoved, 119 fixed, 22 open and 1 not wrong (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).
  **T13 (the same day) made every command read a program the same way**: one
  path, `ein-cli`'s `common::read_program`, tells a `.einb` from text by its
  first bytes and loads a text against its own directory, so `ein saturate`,
  the four `render` views that parse and `ein kb save` take a `.einb` (W40,
  six rows) and `saturate` a file-relative import (W41), both filed and fixed
  under the register's schema `/16` (a `now` can be a command's run that is
  not a solve); a file that is not UTF-8 is refused with one line in Ein's
  words, `<file>:<line>:<col>: byte 0x.. is not UTF-8 …`, at each command's
  load-error exit (`ein test`'s 2, where the read's `process::exit` stopped
  its selection); a container's stored program names itself, `<program stored
  in x.einb>`
  ([Q-M1f.15](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f15--where-does-a-position-in-a-containers-stored-program-point),
  defaulted); and an end-of-input error is placed where the text ends: 126
  fixed, 22 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml |
  sort | uniq -c`, 2026-11-09). **S1f.5.5 closed the same day** with T7, the
  grammar pages' prose restated to the generated grammar (`00_ebnf.md`'s § 3
  gone, `defined_behaviour.md` § 1 rewritten, the editor grammar under the name
  rule), its acceptance re-taken item by item
  ([§ Acceptance re-taken](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#acceptance-re-taken-2026-11-09)).
  **S1f.5.6 ran to its end the same day**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md#t1-record-2026-11-09)):
  where ∅ is in R every minimal question, `--minimal`, `-n N` and `--single`,
  is decided at root after one saturation on a flagged program too, within
  `--region-limit` and past it, and describe's minimal-line row lost *the
  trivial line: no obligation is stated*; and W42, a label as long as its
  column glued to its value (S1f.5.16's Q-M1f.22, and `-H`'s form), was filed
  under the register's schema `/17` and fixed: 128 fixed, 22 open and 1 not
  wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`,
  2026-11-09).
  **S1f.5.12 ran to its end the same day**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.12_cores_for_n_only.md#t1-record-2026-11-09)):
  a run's walk takes a dead node's core only where `--dump-lattice` or
  `--events` prints it (`region::Cores`), and for `--dump-answer` and
  `--trace` each member of N gets its core after the walk, against the walk's
  snapshot, past `--region-limit` too, so `not-kept` is retired and no surface
  prepares root a second time (the user's ruling on P-15, (c)); and W43, `ein
  solve X.einb --dump-answer DIR` refused, was filed under the register's
  schema `/19` and fixed: 129 fixed, 22 open and 1 not wrong (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).
  **S1f.5.8 ran to its end the same day**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.8_no_contradiction_no_core.md#t1-record-2026-11-09)):
  a node that is not dead has no core, so every JSON surface writes its core
  and rules `null` where it wrote `[]`, which reads as *the empty set of facts
  suffices* (the summary's owing holes, `region.json`, `nodes.json` and
  `--events`' `enter` lines), and a dead node's as lists, `[]` the empty set
  (the user's ruling on D-14, Q-M1e2.27); no answer moved and no register row
  was filed (Q-M1f.28): 129 fixed, 22 open and 1 not wrong (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).
  **S1f.5.13 ran to its end the same day**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.13_large_dumps_to_the_slow_tier.md#t1-record-2026-11-09)):
  the round trip's four dumps of 64 514 to 65 214 state files leave the
  per-commit tier for the nightly (the user's ruling on P-16), selected by a
  manifest field that is a count, `heavy_dump`, held both ways by the test;
  `deep-corpus` runs the test's slow half, which no CI tier ran before, and
  the per-commit run takes 8.6 s where it took 55 s (debug, 2026-11-09); three
  other tests whose slow half no CI tier runs are parked for the phase's close
  ([Q-M1f.51](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs));
  and **the nightly is red at `utils/review_bundle.py` since `fc53818f`**,
  whose index and case pages assume every register run is a solve, which
  W40's and W41's are not: reported, not fixed
  ([Q-M1f.52](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f52--the-nightlys-review-bundle-and-the-registers-runs-that-are-not-a-solve)).
  No answer moved, and the register reads 129 fixed, 22 open and 1 not wrong.
  **The second lane, `p1f5-lane2`, ran four stages the same day**, beside
  S1f.5.6, S1f.5.12, S1f.5.8 and S1f.5.13, from `dc4c0bdc`, and was merged
  into master after them (T1f.5.M.1); each stage's count was the lane's, so
  the register's is given once, merged, at the end. **S1f.5.7**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md#t1f571-record-2026-11-09)):
  `eq` and `neq` are the relations of equality and non-equality, computed and
  never stored (the user's ruling of 2026-11-07, Q-M1e2.26, and G-21). In a
  goal and a premise a guard binds a variable nothing else binds, `eq` to the
  other side and `neq` over the program's names
  (`ein_infer::predicates::domain`), and waits for the relation premise that
  binds it, so a premise means the same in any order of its conjuncts; a
  conclusion of either, a written `(eq …)` fact and `(not (eq …))` are
  checked, a false one `(false)`; a query's claim may name either. W30's
  three, W24's two restated rows and W50 to W56's eleven fixed, Q-M1f.30 to
  Q-M1f.35 defaulted. **S1f.5.10**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.10_a_conclusion_variable_must_be_bound.md#t1f5101-record-2026-11-09)):
  a rule's or an hrule's `:assert` that writes a variable some way of
  satisfying its `:match` leaves unbound is refused at load, on every command,
  one line per variable naming the rule, the variable and the two explicit
  spellings, `(is-a ?y T)` and `(eq ?y ?y)` (the user's ruling of 2026-11-07
  on F13; `ein_ir::from_ir`'s `refuse_unbound_conclusions`,
  `defined_behaviour.md` § 4.13). W57's four rows, the ways such a program
  stopped or answered until then, filed and fixed; `FireError::UnboundVar`
  stayed for W58, a parameter its activator binds to a nested fact read only
  by a conclusion, which S1f.5.18 fixed, retiring the variant;
  `fuzz_properties` holds every plan of an accepted program to the rule,
  `ein-ir` taking `ein-infer` as a dev-dependency; Q-M1f.36 to Q-M1f.40
  defaulted. **S1f.5.18**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md#t1f5181-record-2026-11-09)):
  an activator binds a parameter to a nested fact as to a name (W33, the
  user's ruling of 2026-11-08 on S1f.5.5's census), so a value parameter
  matches and concludes the fact, and the last path to
  `FireError::UnboundVar`, a parameter read only by a conclusion (W58), is
  gone with the variant and its `KeyError` line (Q-M1f.37);
  `Plan::activator_args` is every argument, as values (Q-M1f.43); and the
  loader refuses a program in which a rule could derive an activator binding a
  relation parameter to a fact, whether or not a run derives it (Q-M1f.1 (b),
  `ein_ir::from_ir`'s `refuse_derived_relation_activators`,
  `defined_behaviour.md` § 4.9), reading which positions hold relation names
  off the program: a declared relation's argument can hold a fact, and a
  marker's argument holds what a rule passes on from it (Q-M1f.41), so the
  stdlib's fan-outs load; W59's two rows, the `g14` programs, filed and fixed;
  Q-M1f.41 to Q-M1f.44 defaulted. **S1f.5.9**, the lane's last
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md#t1f591-record-2026-11-09)):
  an empty `:match` is the empty conjunction, which is true, so a rule whose
  `:match` is `()` fires once at root, or once per activator fact with its
  parameters alone bound, and an hrule's proposes its conclusion likewise
  (W31's four rows, the user's ruling of 2026-11-07 on F12); `:assert ()`
  asserts nothing, so no `#empty` fact joins H (W34); `(and)` and `(or)` stay
  refused at the parse, their error naming the rule and the clause and saying
  the empty conjunction is `()` (Q-M1f.45); the first empty core, `[]`, on
  `empty-match-false`'s dead root, `refuted by r`, prints `∅` on every text
  surface (Q-M1f.48), and S1f.5.8's `core_field` writes it `[]` on every JSON
  surface; and the shape S1f.5.18 found, a relation parameter an activator
  binds to `eq` heading a form of another arity, which stopped the run where
  the plan was compiled, is refused at load wherever a parameter can hold
  `eq`, read as S1f.5.18's check reads relation names, with a written `(eq a)`
  beside it (`ein_ir::from_ir`'s `refuse_eq_at_another_arity` and
  `refuse_eq_facts_of_another_arity`, `defined_behaviour.md` § 4.14; W60's
  four rows and W61, filed and fixed); Q-M1f.45 to Q-M1f.48 defaulted.
  **The merge**, `a90c452b`
  ([P1f.5 § The order](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/README.md#the-order),
  item 10), moved no answer and no register row: lane 2's 23 rows, W50 to
  W61, follow W43's, the goldens were re-taken rather than spliced, and the
  register reads 161 fixed, 13 open and 1 not wrong (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).
  **S1f.5.19**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.19_the_review_bundle_and_runs_that_are_not_a_solve.md#t2-record-2026-11-09),
  Q-M1f.52 (b) and Q-M1f.53 to .55 defaulted) made the nightly green again
  behind a gate that had stayed green: `utils/review_bundle.py`, red since
  `fc53818f` on W40's and W41's command runs and on four refusals its index
  read as verdicts, builds all 175 cases (`00c4ed34`), a run that is not a
  solve on a case page of its own (`utils/review/command.py`: the command, its
  exit, its stdout and stderr in full, against `wrong`, `now` and `right` as
  `wrong_answers.rs` reads them, every artefact linked and drawn); the gate's
  `--smoke` reads every register row with no run and builds two command
  cases, so the next row shape the bundle cannot read fails per commit; every
  rule view the bundle drew, cut at a subgraph's close and refused by Graphviz
  since S1e2.1.5, is drawn; and the merged tree's first nightly is green in
  every job and step, 81 min 54 s (`./run_tests.sh --nightly`, 2026-11-09),
  the register unmoved.
  **S1f.5.14, the phase's last stage, ran to its end on 2026-11-10**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.14_nl_required.md#t1f5141-record-2026-11-10),
  Q-M1f.56 to .64 defaulted): a fact reads as English in a `--dump-answer`
  state file and in `--trace`, from its first recorded justification, at no
  measurable cost; [`09_natural_language.md`](docs/kernel/ir/03-ein-lang/09_natural_language.md)
  states the texts, the ladder and whose words are whose, and
  `utils/nl_census.py` measures it, the corpus's answers 44.9 % rendered
  before the migration and 94.4 % after, `zebra2`'s state 88.7 % and 97.4 %
  (2026-11-10); the template check found 66 references printed as written,
  each fixed; every missing `:why` was written (995 in the corpus, 903 in the
  Rust suites, 32 in the doc pages), and since `4b231abd` the loader requires
  one by default, `--no-require-why` loading a program without; the register
  unmoved, 161 fixed, 13 open and 1 not wrong (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-10).
  **P1f.5 was re-opened the same day** on the user's answers to
  [its close review](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/close_review.md):
  every decision taken by default stands, *accepted without review* and open to
  revision on a new finding, and each of the review's nine findings with no
  owner became a stage, S1f.5.21 to S1f.5.28 (a compile error in Ein's words,
  Unicode only in a string, `open` reserved, a macro's body checked at load, a
  closed stdout, the tutorial, the nightly reduced, the unchecked findings),
  four of them waiting on Q-M1f.65 to .68. The wrong answers those stages
  find outside their scope are a phase of their own,
  [P1f.7](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/README.md),
  a stage each, so that P1f.5 can close (the user, 2026-11-10), and the
  coordinator stops at P1f.5's close (*"stop in the end of the phase"*).
  **P1f.5 closed on 2026-11-11**
  ([its close](plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/README.md#the-close-2026-11-11)):
  the nine re-opened stages done, S1f.5.21 to S1f.5.29, W62 to W75 filed and
  fixed, 249 fixed, 13 open and 1 not wrong (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11); the nine wrong
  answers they found outside their scope are P1f.7's S1f.7.1 to S1f.7.9, none
  started; and the defaults Q-M1f.69 to .111 stand, the ones that touch a
  ruling listed there for the user. The user ruled the same day that P1f.7
  runs before P1f.6 and that its questions take the stage's default, and
  P1f.7 started. **Before its first stage the nine findings were re-taken**,
  read-only, and all reproduced
  ([§ The analysis of 2026-11-11](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/README.md#the-analysis-of-2026-11-11)),
  and the user ruled on it the same day: S1f.7.8, a `(solve …)` list that
  names nothing, runs first, since one misspelt name moves R in silence;
  S1f.7.9 files its rows only, its fix moved to the new followup
  [F24](plans/followups/f24_written_provenance.md), written provenance (a
  written `:rule` fact leaves every core); and three findings of the analysis
  are S1f.7.10 to S1f.7.12, a nested `(eq k j)`, a goal over a relation
  nothing makes, and a macro call written as a fact. The user also adopted
  **the value rule** that day (Q-M1f.113): *a value is a name, or a fact a
  state could hold, an atom of an ordinary relation or its negation*, so
  `eq`, `neq`, `absent`, `false` with an argument, `and` and `or` are never a
  nested head and `not` stays
  ([§ The value rule](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/README.md#the-value-rule)).
  And **a finding outside a stage's scope that moves no existing zebra
  program's answer is not shown to the user**: it is fixed in place if quick,
  or a row of [F25](plans/followups/f25_roughness_found_outside_zebra.md),
  until a corpus of real Ein programs exists (P1f.6's calculator onward).
  That rule was P1f.7's alone (the user, 2026-11-11): P1f.6 keeps what it finds in its own
  [`findings.md`](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md) and shows it.
  **S1f.7.8 ran to its end the same day**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.8_a_solve_list_naming_nothing.md#t1-record-2026-11-11),
  Q-M1f.114 to .116 defaulted): every name a `(solve …)` list holds names
  something the program has, or the loader refuses the program on every
  command, one line at the name (`ein_ir::from_ir`'s `refuse_solve_names`,
  `defined_behaviour.md` § 4.20): an `:hrules` name is a declared hrule, an
  activator has as many arguments as its hrule has parameters, and an
  activator's argument at a parameter that names a relation, and every name
  of `:hypothesis-relations` and `:no-hypothesis`, is a relation the program
  writes outside the lists, one only a rule derives among them. W76's five
  rows filed and fixed, `examples/syntax/query-full.ein` declaring the two
  hrules it names, and no other corpus answer moved: 254 fixed, 13 open and
  1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`,
  2026-11-11). An activator's argument that is a value is not checked, F25's
  R-2.
  **S1f.7.5, S1f.7.11 and S1f.7.1 ran to their end the same day**, in one
  subagent
  ([S1f.7.5's records](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.5_a_macro_called_in_a_goal.md#t1-record-2026-11-11),
  [S1f.7.11's](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.11_a_goal_over_a_relation_nothing_makes.md#t1-record-2026-11-11)
  and [S1f.7.1's](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.1_a_goals_variable_relation_in_a_nested_pattern.md#t1-record-2026-11-11),
  Q-M1f.117 to .121 defaulted): **a query's `:goal` is held to what a rule's
  `:match` is held to** (`defined_behaviour.md` § 4.21). A macro call in a
  goal is expanded at load (`ein_ir::from_ir`'s `expand_goal`), matched on its
  expansion and printed as written (`ein_core::Query::goal_written`), so the
  stdlib's `unknown` and `forall` work in a goal, and a `std.macro` call
  without its import, a call of another arity and an expansion a rule would be
  refused for are refused (W77). Every relation a goal asks about, at any
  depth, and every relation a claim lists a fact of, is a relation of the
  program, S1f.7.8's notion without the goals (`program_relations`), or one a
  `:hrules` activator has its hrule propose: a goal over any other name is
  refused at the pattern, and a claim over a relation only a rule's `:assert`
  derives loads, where it was refused (W78). And one walk reads a rule's
  clauses and a goal (`refuse_no_fact_holds`), so a relation written as a
  variable inside a goal's pattern, `(says (?R ?g))`, is refused as the
  top-level one is (W79). Ten rows filed and fixed, no fixture's program
  rewritten and no other corpus answer moved, one refusal reworded
  (`examples/broken/load/expect_unknown_relation.ein`): 264 fixed, 13 open and 1 not wrong (`grep
  '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11). The pin
  is `ein-cli/tests/goal_premises.rs`. A rule whose `:match` reads a relation
  nothing makes is not refused, F25's R-6.
  **S1f.7.7, S1f.7.6, S1f.7.10 and S1f.7.3 ran to their end the same day**, in
  one subagent
  ([S1f.7.7's records](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.7_an_absent_fact.md#t1-record-2026-11-11),
  [S1f.7.6's](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.6_a_false_fact_with_an_argument.md#t1-record-2026-11-11),
  [S1f.7.10's](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.10_an_eq_fact_as_a_value.md#t1-record-2026-11-11)
  and [S1f.7.3's](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.3_a_connective_as_a_patterns_argument.md#t1-record-2026-11-11),
  Q-M1f.122 to .126 defaulted): **a value is a name, or a fact a state could
  hold**, an atom of an ordinary relation or its negation (the user's ruling
  of 2026-11-11, Q-M1f.113; `defined_behaviour.md` § 4.22). No fact, value or
  conclusion is headed `absent`, which is a premise's test and a claim's class
  atom and nothing else (W80); `false` takes no argument anywhere, `(false)`
  alone (W81); `(eq a b)` is a fact, a premise or a conclusion of its own,
  checked, and never a value, as `(neq a b)` never was (W82); and a premise
  pattern's argument is what a value can be, so `and`, `or`, `neq`, `eq`,
  `absent` and `false` with an argument there are refused, `(not <pattern>)`
  kept (W83). The grammar refuses what a program writes (`ein.y`'s
  `ValueHead`, the one list of the relations a value may have), each error
  with a note, the one at `absent` pointing at `(not …)`; the loader refuses
  what only it can see, a connective a pattern's argument still is once the
  macros are expanded, the same shapes left by a macro's expansion
  (`ein_ir::from_ir`'s `Shapes` walk), and a relation parameter an activator
  can bind to `absent`, `false` or, where a value goes, `eq`. Nineteen rows
  filed and fixed, and no other corpus answer moved: 283 fixed, 13 open and
  1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`,
  2026-11-11). The pin is `ein-cli/tests/value_rule.rs`. `(false)` with no
  argument stays a value, F25's R-7.
  **S1f.7.4 and S1f.7.2, the two about `not`, ran to their end the same
  day**, in one subagent
  ([S1f.7.4's records](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.4_the_negation_of_a_name_concluded.md#t1-record-2026-11-11)
  and [S1f.7.2's](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.2_an_hrule_that_reads_not.md#t1-record-2026-11-11),
  Q-M1f.127 to .129 defaulted; `defined_behaviour.md` § 4.23). **`not` of a
  name is the negation of no fact**: a conclusion `(not ?x)` is refused at
  load, one line on every command, wherever the program shows that `?x` can
  hold a name (`ein_ir::from_ir`'s `refuse_negated_names`: a written fact, a
  declaration, a rule's or an hrule's conclusion to a fixpoint, an activator,
  a declared relation the hypotheses are taken from, a guard alone), and
  where only a run meets the name the firing builds no fact for it, so
  nothing is stored and an hrule proposes nothing
  (`ein_infer::firing::build_fact`), F25's R-1 saying what is left. The three
  `examples/saturation/hypothesis-contradiction/` programs conclude `(not
  ?h)` of facts and load as before (W84, seven rows). And **an hrule's
  `:match` may read `not`**: the `debug_assert!` that stopped a debug build
  where release answered as `ein oracle` does is gone, and where an hrule
  reads `not` every hrule is matched before any candidate is emitted (W85,
  three rows, filed and fixed in one commit since a debug run cannot hold an
  `open` row). No other corpus answer moved: 293 fixed, 13 open and 1 not
  wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`,
  2026-11-11). The pins are `ein-cli/tests/negated_names.rs` and
  `hrule_reads_not.rs`.
  **S1f.7.12 and S1f.7.9, the phase's last two stages, ran the same day**, in
  one subagent
  ([S1f.7.12's records](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.12_a_macro_call_written_as_a_fact.md#t1-record-2026-11-11)
  and [S1f.7.9's](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.9_a_facts_rule_that_names_no_rule.md#t1-record-2026-11-11),
  Q-M1f.130 to .134 defaulted; `defined_behaviour.md` § 4.24). **A macro's
  call is no fact**: a fact that is or holds, at any depth, a form headed by
  a macro's name, one the program declares or imports or `std.macro`'s where
  it imports none, is refused at load, one line on every command, and so is
  such a form inside a fact a claim lists and a relation declared under a
  macro's name (`ein_ir::from_ir`'s `refuse_macro_facts` and
  `refuse_macro_claims`; W86, eight rows). The call is not expanded into the
  fact its body spells, and a call in a rule's or an hrule's `:assert` is
  expanded as it was. **S1f.7.9 filed its rows and fixed nothing**: W87, three
  rows, `open` and owned by
  [F24](plans/followups/f24_written_provenance.md), a written fact's `:rule`
  naming a rule the program lacks, a written `:rule` fact leaving every core,
  and a `:using` naming a fact no state holds; what a written provenance may
  claim is the user's question, so each row's `right` is the answer under
  one named reading with the others as alternatives, as W8's are. No other
  corpus answer moved: 301 fixed, 16 open and 1 not wrong (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11). The pin is
  `ein-cli/tests/macro_facts.rs`. Every stage of P1f.7 has run; its close is
  the coordinator's.
  **P1f.7 closed on 2026-11-11**
  ([its close](plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/README.md#the-close-2026-11-11)):
  twelve stages done, W76 to W87, 301 fixed, 16 open and 1 not wrong (`grep
  '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11), the 16
  owned by F21, F22, F24 or S1f.10.6; no zebra program's output moved; the
  defaults Q-M1f.114 to .134 stand unreviewed, and what the stages found
  outside their scope is F25's R-1 to R-8. **P1f.6, the calculator, is next.**
  **P1f.6 started on 2026-11-11.** Its design stage, S1f.6.1, ran three rounds
  of experiments on a prototype under `build/s1f.6/prototype/` and closed on
  the user's rulings
  ([`design.md` § The rulings](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/design.md)):
  a number is a nested fact of digit names in reading order, `(n (n (n z 1) 2)
  3)`; the theory's circuits never join two facts on a number, so a batch is
  linear; tables are plain facts, which needs a flat import to bring a
  module's facts, the engine fix S1f.6.11, first; `num9` ships as a second
  module; division by zero derives a flag and `(false)` from it. The phase
  keeps what it finds in its own `findings.md`, and S1f.6.20, word problems,
  is its first application.
  **S1f.6.11, the phase's engine stage, ran to its end the same day**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.11_importing_facts.md#t1-record-2026-11-11),
  Q-M1f.135 to .144 defaulted; `defined_behaviour.md` § 4.25). **A flat
  import brings a module's facts** (the user's ruling of 2026-11-11,
  *"importing facts is mandatory"*): `(import M :symbols (…))` keeps, beside
  every declaration whose name is in the selection's closure, every top-level
  fact of `M` whose relation is, as written and in the module's order, a
  stored negation with the relation it negates (`ein_ir::imports`' `select`).
  A listed name may be a relation the module only states facts of; the
  closure follows the names a kept declaration writes and not the names
  inside a fact; a rule nothing names is still dropped; an imported fact is a
  fact of the program, its constants names of the program; and a fact that
  arrives by two flat paths is one statement. A qualified import is unmoved:
  it renames what the module declares, so its facts land in `o.is-a`, and a
  library of facts is imported flat
  ([`01_grammar.md` § Import](docs/kernel/ir/03-ein-lang/01_grammar.md#import)).
  Until the stage a selection kept declarations alone, so a library of facts
  arrived empty, in silence. W88's four rows filed and fixed, the witnesses
  the `import-facts-*` family; no tracked program imported a module that
  holds a fact, and the sweep, old release binary against new over 669 `.ein`
  files, moved the four witnesses alone (`python3 build/s1f.6.11/sweep.py`,
  2026-11-11): 305 fixed, 16 open and 1 not wrong (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-11). The pins are
  `ein-ir/tests/imports_semantics.rs` and `ein-cli/tests/import_facts.rs`.
  The stage found E-8, a qualified import bringing a module's `(query …)`,
  `(solve …)` and `(config …)`, kept in the phase's `findings.md`. S1f.6.2,
  the tables, is next.
  **S1f.6.2, the digit tables, ran to its end the same day**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.2_digits_and_tables.md#t1-record-2026-11-11),
  Q-M1f.145 to .148 defaulted). **`std.digits` is the stdlib's first library
  of facts**, [`stdlib/digits.ein`](stdlib/digits.ein): the calculator's seven
  tables, `fa`, `fs`, `md`, `cl`, `nz`, `mid` and `adj`, 1 664 rows, a fact a
  row and no rule (`python3 utils/gen_arith_tables.py`, 2026-11-11). **The
  file is generated**, by `utils/gen_arith_tables.py`: edit the script, run
  it, then `utils/stdlib_manifest.py --write`, never the rows.
  `ein-ir/tests/digit_tables.rs` runs the script's `--check`, which is how it
  is in the gate, and holds every row to Rust's integers, none missing and
  none twice. The module closes its tables with written `(__closed__ R)`
  facts, which a flat import brings only when `__closed__` is listed,
  `(import std.digits :symbols (fa nz __closed__))`; without it `nz` and
  `adj` are relations the search guesses rows of. A theory that reads the
  tables closes them in its own closing rule. `stdlib_coverage.rs` names the
  module as rule-free, beside `std.macro`, and the two programs under
  `tests/stdlib/digits/` hold the import, R = {∅} with no candidate, and a
  core that names the row it passed through, `(fa 7 5 0 2 1)`. **No picture
  of a table can be drawn**: every view of the module is some 7 900 edges
  and neither `dot` nor `fdp` finishes one in minutes (E-10 of the phase's
  `findings.md`), so `ein-render/tests/dot_wellformed.rs` puts a view of more
  than 1 500 edges to Graphviz under `fdp` stopped after one iteration, the
  same grammar and attribute checks without `dot`'s layout
  ([Q-M1f.147](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f147--the-dot-sweep-and-a-view-no-layout-finishes));
  that sweep took 171.9 s where it took 135.9 s
  (`cargo test -p ein-render --test dot_wellformed`, 2026-11-11). **Since
  S1f.6.21 such a view is skipped**, on the user's ruling, and put to
  Graphviz under no layout. A program
  that does not import the module pays nothing, and one that does pays about
  12 ms to parse it. No engine source moved and no existing program's answer
  (`python3 build/s1f.6.2/sweep.py`, 672 files, 2026-11-11); the register is
  unmoved. S1f.6.3, addition, is next.
  **S1f.6.3, addition, ran to its end the same day**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.3_addition.md#t1-record-2026-11-11),
  Q-M1f.149 to .154 defaulted). **`std.arith` begins**,
  [`stdlib/arith.ein`](stdlib/arith.ein), written by hand: a natural number
  is `z` or `(n PREFIX DIGIT)`, of any length and normalised; a request fact
  `(add X Y)` is answered by `(sum-of X Y S)`, the operands repeated as
  written; `(let NAME NUMBER)` names a number and a request may name its
  operands. It computes as the design called K: each operand looked up, one
  walk from the units through `fa`, the digits put in front of an
  accumulator `(b DIGIT REST)`, high zeros dropped there, the answer turned
  round and handed over as `(done TAG ANSWER)`, every call carrying its
  continuation as the tag. 123078 + 4567 is 21 firings and 21 deep, and
  4 000 sums saturate in 110 ms (`python3 build/s1f.6.3/batch.py`,
  2026-11-11). **One name brings the theory**, `(import std.arith :symbols
  (arith))`: `arith` is the closing rule, which closes every relation of the
  module and the tables it reads and names every rule, so H is empty, k_s is
  1 and R is {∅} on every arithmetic program. **Whoever adds a rule or a
  relation adds it to that rule's two lists**, and
  `ein-infer/tests/arith_theory.rs` fails on one forgotten. The same file
  holds every sum to Rust's integers (every pair to 120, every pair of
  lengths to 30 digits on a fixed-seed sample, high zeros, normal form) and
  **the rule of the theory on the module's text: no rule joins two facts on
  a number**. Its measured half is `arith_batch.rs`: the matcher's
  `candidates` a request do not grow with the batch, 181.38 at 250, 1 000
  and 4 000 requests. The counters are compiled out of every other build,
  so that test had a gate step of its own, `cargo test -p ein-infer
  --features counters --test arith_batch`, **until S1f.6.21 took it out of
  the gate on the user's ruling: it is run by hand** by a stage that changes
  a rule of the calculator, and no tier runs it. The stage measured where E-3 bites: a premise with an
  unbound slot walks its relation when asked with a number, and a premise
  with every slot bound is one lookup, which `operand-name` uses, asking
  `(named ?x)` before its `let`. `ein-cli/tests/arith_search_free.rs` holds
  the search-free claim on the binary for every program under
  `tests/stdlib/arith/` and `examples/arith/`, the same `sum-of` facts under
  `ein solve`, `ein saturate` and `ein test`; a later operation joins each of
  the three files by one list
  ([§ For the stages that follow](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.3_addition.md#for-the-stages-that-follow)).
  Seven programs under `tests/stdlib/arith/` activate the 19 rules, and the
  mutation sweep kills all 21 of the module's mutants, 188 of 238 (`python3
  utils/stdlib_mutants.py`, 2026-11-11). The DOT sweep's bound moved to
  1 000 edges, under the smallest table
  ([Q-M1f.153](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f153--the-dot-sweeps-bound-under-the-smallest-table)).
  The stage found E-11: the program a `--dump-answer` state file re-writes
  drops a rule that only a selection names, one with an empty `:match`, so
  the module writes its closing rule's name as a fact, `(arith-rule arith)`,
  which the tree-shake follows, and `answer_round_trip.rs` holds its
  programs.
  No engine source moved and no existing program's answer (`python3
  build/s1f.6.3/sweep.py`, 681 files, 2026-11-11); the register is unmoved.
  **S1f.6.4, subtraction and comparison, ran to its end the same day**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.4_subtraction.md#t1-record-2026-11-11),
  Q-M1f.155 to .158 defaulted). `(sub X Y)` is answered by **two numbers**,
  `(diff-of X Y D R)`: D is X − Y where X is not the smaller and `z`
  otherwise, R what X is short by where it is the smaller and `z` otherwise;
  the theory has no negative number, and a rule that matches `(diff-of X Y z
  (n ?p ?a))` holds exactly where X is the smaller. The circuit subtracts
  **both ways at once**, two walks over `fs`: the one that ends with no
  borrow hands over its answer and the other derives `(short TAG)` and no
  number, which is what division's bisection will read. `(cmp X Y)` is
  answered by `(cmp-of X Y O)`, O the name `lt`, `eq` or `gt`, by one walk
  over `cl` that builds no number and hands over `(order-is TAG O)`.
  123078 − 4567 is 28 firings and 21 deep, the comparison 13
  (`arith_theory.rs`'s `the_circuit_of_one_subtraction_and_one_comparison`).
  **`arith_theory.rs` is now a table of operations, `OPS`**: a later
  operation joins the harness by a row, its request, its result and what
  `u128` says, and is asked every case the others are, equal operands and
  operands one apart at every length and requests by name among them.
  `arith_batch.rs` measures each operation in batches of its own, a row of
  `REQUESTS` and a bound: 181, 362 and 901 candidates a request for `add`,
  `sub` and `cmp`, the same at 250, 1 000 and 4 000 (`cargo test -p
  ein-infer --features counters --test arith_batch -- --nocapture`,
  2026-11-11). The 901 is E-12 of the phase's `findings.md`: the matcher
  keys a premise on its first bound argument, and `cl`'s first is the order
  below, which a hundred rows share. Five programs under
  `tests/stdlib/arith/` activate the 18 new rules, 114 of 114 over the
  stdlib, and the mutation sweep kills all 15 of their mutants, 203 of 253
  (`python3 utils/stdlib_mutants.py`, 2026-11-11).
  `examples/arith/sub.ein` is the example. No engine source moved, and no
  existing program's `ein solve`: the eight programs that imported the
  theory hold more of it in `ein saturate --dump`, the two tables' rows and
  the closing rule's new facts, and nothing they held is gone (`python3
  build/s1f.6.4/sweep.py`, 681 files, the stdlib of `277cfc44` against the
  tree's, 2026-11-11); the register is unmoved.
  **S1f.6.5, multiplication, ran to its end the same day**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.5_multiplication.md#t1-record-2026-11-11),
  Q-M1f.159 to .162 defaulted). `(mul X Y)` is answered by `(product-of X Y
  P)`, the school method from the multiplier's top digit: the multiplier is
  turned round once (`top-first`, a number into digits with the top one
  first, handed over as `(top-first-is TAG DIGITS)`), each of its digits is
  a row, a walk over `md` (`mul-walk`), and the sum so far times ten, one
  constructor, plus the row is S1f.6.3's `add-walk`, called with a tag that
  holds the digits still to come. It is tagged throughout, `(mul-call TAG X
  Y)` answered by `(done TAG P)`, so a later theory can call it. 16 rules,
  53 in the module and 130 over the stdlib. 123 × 456 is 72 firings and
  123078 × 123078, eleven digits, 227; the size grows as the product of the
  two lengths, and 4 000 products saturate in 1.1 s (`python3
  build/s1f.6.4/batch.py mul`, 2026-11-11). It is a row of `OPS`, of
  `REQUESTS` and of `RESULTS`: 9 600 candidates a request, the same at 250,
  1 000 and 4 000. Three programs activate the rules and the mutation sweep
  kills all 13 of their mutants, 216 of 266. **The DOT sweep runs several
  files at a time since this stage**
  ([Q-M1f.161](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f161--the-dot-sweep-several-files-at-a-time)):
  every program that imports the theory holds the tables it reads, so each
  of its six KB views is some 9 000 edges, 2.2 s under the bounded layout
  of the time, 441 s of `dot` for eighteen programs; `dot_wellformed.rs`
  shares the corpus files out among the cores, 28 s here then, and holds
  what it held (those views are skipped since S1f.6.21, and the sharing
  kept).
  `examples/arith/mul.ein` is the example. No engine source moved and no
  existing program's `ein solve` (`python3 build/s1f.6.4/sweep.py --old
  build/s1f.6.5/old/stdlib --ref 088a1f87`, 687 files, 2026-11-11); the
  register is unmoved.
  **S1f.6.6, division, ran to its end the same day**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.6_division.md#t1-record-2026-11-11),
  Q-M1f.163 to .165 defaulted), and with it **the four operations and
  comparison are built**: `std.arith` is 88 rules and its closing rule, over
  all seven tables. `(div X Y)` is answered by `(quot-of X Y Q R)`, school
  long division: the divisor's multiples by 1 to 9, each a row of
  multiplication, computed once and carried as one fact; the dividend turned
  round; a step a digit, the digit brought down beside what is left over;
  and the quotient's digit found **by bisection**, each trial a subtraction
  of the middle multiple that either answers, `(done TAG LEFT)`, or is
  `(short TAG)`, at most four a digit. Tagged, `(div-call TAG X Y)` answered
  by `(div-done TAG Q R)`. The nine trial rules, `div-try-1` to `-9`, are
  one rule written nine times, since a rule cannot pick the J-th multiple by
  a digit without joining on a number, and
  `the_rules_of_a_family_differ_by_their_digit_alone` holds them alike.
  127645 ÷ 25 is 220 firings, one chain, and 4 000 divisions saturate in
  2.6 s (`python3 build/s1f.6.4/batch.py div`, 2026-11-11); 6 030 candidates
  a request, the same at 250, 1 000 and 4 000. **Division by zero is a
  contradiction, in two steps** (the user's ruling of 2026-11-11):
  `div-by-zero` derives `(division-by-zero (div X Y))`, the request as
  written, for a divisor that is zero by value, and
  `division-by-zero-is-false` derives `(false)` from the flag, the one rule
  of the theory that concludes it. Root is dead, R is empty, every command
  exits 0, `ein solve` names the request in the unsat core (with the `let`s
  of named operands, and the `md` rows of a zero written with digits) and
  the second rule under `refuted by`, `--trace` shows two steps, `ein
  saturate --dump` shows the flag and `(false)` in root's state, and such a
  program can claim only `(solve :expect (false))`: every other query of it
  loses its answer
  ([the record's § T2](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.6_division.md#t2-record-2026-11-11-division-by-zero-on-every-surface)
  has every surface). `arith_search_free.rs` holds a program that claims
  `(false)` to an empty R decided by root alone, where it holds the others
  to R = {∅}. Six programs activate the 36 rules, 166 of 166 over the
  stdlib, and the mutation sweep kills all 18 of their mutants, 234 of 284:
  the module's 67 mutants have no survivor (`python3
  utils/stdlib_mutants.py`, 2026-11-11). `examples/arith/div.ein`, with the
  chain of `design.md`, and `div_by_zero.ein` are the examples. A program
  that imports the theory now holds 1 664 table rows: `ein test tests/`
  takes 681 ms for twenty-one arithmetic programs. No engine source moved
  and no existing program's `ein solve` (`python3 build/s1f.6.4/sweep.py
  --old build/s1f.6.6/old/stdlib --ref b2c4b238`, 691 files, 2026-11-11);
  the register is unmoved.
  **S1f.6.15, load against calculation, ran to its end the same day**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.15_load_against_calculation.md#t1-record-2026-11-11),
  Q-M1f.166 defaulted), on the user's *"add examples with long numbers and
  many calculations so we can compare load time with calculation time"*.
  **Three examples under `examples/arith/` are generated**, by
  `utils/gen_arith_examples.py`, whose `--check` is in the gate
  (`arith_search_free.rs`): `long_numbers.ein`, five requests on operands of
  100, 10 and 20 digits; `many_calculations.ein`, 100 requests on short
  operands; and `factorial.ein`, 25! from one written fact by three rules of
  the program's own. **They are small on purpose:** `ein solve` writes the
  state as text, and a circuit's state as text grows as its facts times its
  digits, so a 100-digit product is 125 ms under `ein saturate` and 4.0 s
  under `ein solve` (E-24 of the phase's `findings.md`), and every corpus
  program is solved, dumped and reloaded per commit. The plan's sizes are
  **`utils/arith_cost.py`**'s sweep, run through `bench_env.sh`: parse, load,
  saturate, firings, facts and peak RSS for the examples, as text and as a
  `.einb`, and for requests by length (to 900 digits), by count (to 10 000)
  and for the chain (to 200!), with `ein solve` beside `ein saturate`. Its
  output is
  [`stdlib/README.md` § What a program pays](stdlib/README.md#what-a-program-pays-the-tables-the-parse-the-calculation)
  (2026-11-11): a program that imports the theory pays 20.6 ms of load
  before its first request, 14.3 of it `std.digits`; **a `.einb` buys none
  of it back**, 21.9 ms to open against 21.8 to parse and load (E-25; it did
  while the theory read two tables); past a thousand sums the parse costs
  more than the addition, 748 ms against 502 at 10 000 (E-16); a batch costs
  in proportion to its count, 27, 37, 16, 486 and 589 firings a request; and
  a stored fact costs a kilobyte, 2.1 GB for 4 000 products (E-26). No
  engine source and no stdlib module moved; the register is unmoved.
  **S1f.6.12's first task, the investigation, is done and stopped for the
  user** (2026-11-11,
  [`malformed_numbers.md`](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/malformed_numbers.md)),
  on the user's *"if theory can detect malformed numbers - do it, what
  cons/objections?"*. Today every malformed request is answered by silence,
  exit 0. On prototypes of the shipped `std.arith`: a digit's place holding
  anything else, a number that ends in a digit or a named constant where
  `z` goes, a digit where a number goes, and a constant named twice (which
  today gives **two answers to one request**) can each be flagged with no
  `absent` and no `not`, by ten `neq` guards or a lookup, for twenty firings
  a request; a name no `let` states and a fact of another relation in a
  number's place cannot, since *not of this shape* is a negation; and a
  wrong count of arguments is the loader's to refuse, where a census finds
  no program that loads today refused. Nothing is built: the page's six
  questions are the user's, the loader's arity check (E-2) first.
  **S1f.6.13, `std.num9`, ran to its end the same day**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.13_num9.md#t1-record-2026-11-11-the-generator-and-the-module),
  Q-M1f.167 to .173 defaulted), on the user's *"yes, ship num9 in stdlib, why
  not"*: **the stdlib's tenth module, and its second generated one.**
  `stdlib/num9.ein` is written by `utils/gen_num9.py` (`--check` in the gate
  through `ein-infer/tests/num9_theory.rs`; edit the generator, never the
  file): the user's own number, `(num9 0 0 0 1 2 3 0 7 8)`, nine digit names
  in one fact, and 47 **wide rules**, a premise a column. Its relations are
  its own, `(add9 X Y)` answered by `(sum9-of X Y S)`, `sub9`, `cmp9`, `mul9`
  and `div9` likewise, the entry `arith9`, so both modules import flat into
  one program. A sum, a difference and a comparison are **one firing**; a
  product and a quotient eleven. **Overflow is a fact, `(overflow REQUEST)`,
  and never a truncated number**, root alive; a division by zero derives
  `std.arith`'s flag and `std.arith`'s rule derives `(false)`. `(to9 X)` and
  `(from9 W)` convert to and from `std.arith`'s numbers, a firing each, and
  nothing routes between the two. **No rule joins two facts on a number**, as
  in `std.arith`, which the prototype's division did: the divisor's nine
  multiples are computed by one rule of 72 table premises and travel in the
  state, ten step rules each read their own, and a product's step computes
  its row itself, so 4 000 divisions saturate in 1.24 s where the prototype
  took 8.1 and `std.arith` takes 1.78, and 4 000 sums in 24 ms against 113
  (`python3 build/s1f.6.13/batch.py`, 2026-11-11). It wins at volume and not
  on one request: a program that imports it pays 29.3 ms of load where
  `std.arith`'s pays 19.6. `num9_theory.rs` holds it to Rust's integers
  **exhaustively at width 2**, the generator's own output, every pair of
  operands under five operations, each request with exactly one answer or
  exactly one flag; thirteen programs under `tests/stdlib/num9/` activate the
  47 rules, 213 of 213 over the stdlib; `arith_search_free.rs` and
  `arith_batch.rs` took the module in; and the mutation sweep kills all 41 of
  its mutants, 275 of 325, the bank unmoved (`python3
  utils/stdlib_mutants.py --check`, 2026-11-11). `examples/arith/num9.ein`,
  `num9_overflow.ein` and `num9_both_ways.ein` are the examples. **A wide
  rule is wide in registers:** `div9-start` binds 162 variables of the 256 a
  rule may (`ein_infer::MAX_REGS`), where the widest rule of everything else
  binds 16, and `compile_limits.rs`'s tripwire, which held the corpus under
  an eighth of the bound, now measures the module apart with its widest
  pinned: Q-M1f.173, defaulted and **the user's to confirm**, E-28. No engine
  source moved and no existing program's answer (`python3
  build/s1f.6.13/sweep.py`, 2026-11-11); the register is unmoved.
  **S1f.6.7, negative numbers, was built but for division on 2026-11-12**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.7_negative_numbers.md#t1-record-2026-11-12),
  Q-M1f.174 to .178 defaulted). **`std.int` is the stdlib's eleventh module
  and the calculator's first extension**, [`stdlib/int.ein`](stdlib/int.ein),
  written by hand over `std.arith`, which it calls and does not edit: an
  integer is `(int P N)`, two natural numbers, its value P less N, its normal
  form one part `z`, zero `(int z z)`. Requests `int-simplify`, `int-neg`,
  `int-add`, `int-sub`, `int-mul` and `int-cmp`, answered by `int-simple-of`,
  `int-neg-of`, `int-sum-of`, `int-diff-of` (one integer, where `std.arith`
  answers two numbers), `int-product-of` and `int-cmp-of`; an operand is an
  integer written out or a name through `std.arith`'s `let`, and a natural
  number is not one. It is imported by one name, `(import std.int :symbols
  (int-arith))`, whose closing rule names `arith`, so it brings `std.arith`
  too (E-20's remedy). **Every rule has one premise**: each starts one of
  `std.arith`'s walks under a tag, and **a call's tag is its request**,
  `(int-done (int-add ?x ?y) ?s)`. A sum is two addition walks and a
  simplification, which is subtraction both ways at once; a product is four
  products and two sums, and no rule looks at a sign. 123078 + −4567 is 59
  firings and 123 × −456 is 142 (`cargo test -p ein-infer --test int_theory
  the_circuit`, 2026-11-12); 4 000 sums saturate in 357 ms where `std.arith`'s
  take 117 (`python3 build/s1f.6.7/batch.py`, 2026-11-12).
  `ein-infer/tests/int_theory.rs` holds it to Rust's signed integers, every
  pair of integers written with one-digit parts under every operation, and
  holds the one premise on the text; `arith_batch.rs` and
  `arith_search_free.rs` took the module in; eight programs under
  `tests/stdlib/int/` activate the 36 rules, 250 of 250 over the stdlib; and
  the mutation sweep kills all 42 of its mutants, 317 of 367, the bank
  unmoved (`python3 utils/stdlib_mutants.py --check`, 2026-11-12).
  `examples/arith/int.ein` is the example. Integer division was not built
  by that stage: its remainder convention was the user's question,
  [Q-M1f.179](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f179--the-remainder-of-an-integer-division),
  ruled on 2026-11-12 and **built by S1f.6.22**, below. No engine source moved and no existing
  program's answer (`python3 build/s1f.6.7/sweep.py`, 719 files,
  2026-11-12); the register is unmoved.
  **S1f.6.8, rational numbers, ran to its end on 2026-11-12**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.8_rational_numbers.md#t1-record-2026-11-12),
  Q-M1f.180 to .184 defaulted). **`std.rat` is the stdlib's twelfth module**,
  [`stdlib/rat.ein`](stdlib/rat.ein), written by hand over `std.int` and
  `std.arith`, which it calls and edits neither: a rational is `(rat Z D)`,
  an integer `(int P N)` over a natural number that is not zero, and every
  answer is **in lowest terms**, zero written 0 over 1. Requests
  `rat-simplify`, `rat-neg`, `rat-add`, `rat-sub`, `rat-mul`, `rat-div` and
  `rat-cmp`, answered by `rat-simple-of`, `rat-neg-of`, `rat-sum-of`,
  `rat-diff-of`, `rat-product-of`, `rat-quot-of` (one rational, exact, no
  remainder) and `rat-cmp-of`; imported by one name, `(import std.rat
  :symbols (rat-arith))`, whose closing rule names `int-arith` and `arith`. A
  sum is the cross products over the product of the denominators; **a
  comparison is the cross products of the difference and nothing more**; a
  quotient is a product with the divisor turned over; and lowest terms is
  Euclid's chain, each step a division of `std.arith`. **It needed no
  division of integers**, so it did not wait for Q-M1f.179. **A zero
  denominator is a division by zero**: a request with such an operand,
  however the zero is written, and a `rat-div` by a rational that is zero,
  derive `std.arith`'s flag, `(division-by-zero REQUEST)`, and its rule
  derives `(false)`; no rule of the module concludes `(false)`. Every rule
  has one premise. 1/2 + 1/3 is 492 firings and 1/2 against 1/3 is 76
  (`cargo test -p ein-infer --test rat_theory the_circuit`, 2026-11-12): most
  of a request is lowest terms, a long division's nine multiples computed
  for each step of the chain (Q-M1f.183). `ein-infer/tests/rat_theory.rs`
  holds it to exact fractions, every pair of 28 small fractions under five
  operations; **its sizes are small on purpose**, 250 requests a saturation
  and batches of 100, 400 and 1 200 in `arith_batch.rs`, since a stored fact
  costs a kilobyte (E-26). Thirteen programs under `tests/stdlib/rat/`
  activate the 56 rules, 307 of 307 over the stdlib, four of them claiming
  `(false)`; `arith_search_free.rs` reads what a division by zero may be
  from three lists, `DIVIDES`, `RESTS_ON` and `FLAGGERS`; and the mutation
  sweep kills all 44 of its mutants, 361 of 411, the bank unmoved (`python3
  utils/stdlib_mutants.py --check`, 2026-11-12).
  `examples/arith/rat.ein` is the example. No engine source moved and no
  existing program's answer (`python3 build/s1f.6.8/sweep.py`, 729 files,
  2026-11-12); the register is unmoved.
  **S1f.6.9, what is left of the width, was measured and stopped for the user
  on 2026-11-12**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.9_width.md#t1-record-2026-11-12-the-limit-re-taken)).
  **How long a number may be is three facts, none the theory's**, stated in
  [`stdlib/README.md` § How long a number may be](stdlib/README.md#how-long-a-number-may-be-m1f-s1f69)
  and in `stdlib/arith.ein`'s header. A program writes a number of 999
  digits and no longer, the parser's 1 000 parentheses; a rule can derive
  more, and the engine saturates facts 396 000 levels deep (`python3
  build/s1f.6.9/deeper.py`, 2026-11-12). **A state is deeper than its
  request**, since a step holds its operands inside its tag, by 2 levels for
  a sum, a difference and a comparison, 3 for a product and a quotient, 4 and
  5 for the integers' and 7 for a rational's numerator: so `ein solve
  --dump-answer` writes a state file that is refused on reload from operands
  of 999 digits for a sum, each a request that loads and is answered rightly
  (E-4, re-taken; `python3 build/s1f.6.9/depth.py --boundary add cmp`,
  2026-11-12). And each step holds its numbers whole: a derived fact of a
  998-digit computation is about 40 KB of text, a 998-digit sum's state 86 MB and
  6.7 s under `ein solve` where it saturates in 3 ms, and the longest product
  a program can ask, 998 digits by 998, is 4 988 896 firings, 9.8 s and
  5.9 GB under `ein saturate` (`utils/bench_env.sh python3
  build/s1f.6.9/cost.py` and `longest.py`, 2026-11-12). **What to do about
  the state file is the user's choice,**
  [Q-M1f.185](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f185--a-state-file-nested-past-the-parsers-limit):
  (a) say it, which is done; (b) flag a number past a bound by rule, probed
  on a prototype, which doubles the firings of every sum; (c) an engine
  change, not built. Nothing was built past the statement: the stdlib's
  rules did not move, only `arith.ein`'s header.
  **S1f.6.10, the investigation of what the theory can do besides
  arithmetic, ran on 2026-11-12 and is shown to the user**
  ([`beyond_arithmetic.md`](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/beyond_arithmetic.md),
  [its records](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.10_beyond_arithmetic.md#t1-record-2026-11-12);
  Q-M1f.186 to .189 defaulted). No stdlib module and no engine source moved.
  **A table is a relation, so a rule reads it in any direction**: an
  equation with one unknown, x + 4567 = 127645 or x × 7 = 861, is solved by
  saturation alone, and where a column has two answers saturation derives
  both walks and the next column ends the wrong one. **With the unknown
  digits as hypotheses it is a search**, under `total-owed` and
  `functional`: `ein solve --single` takes 10n + 1 saturations for n digits
  where the constraint is a column at a time, and about 10ⁿ where it is one
  whole-number check through `std.arith`; `ein oracle` agrees at 10 and 20
  candidates. SEND + MORE = MONEY is solved and shown unique by `--single`
  in 17 747 saturations, 0.5 s, where `zebra.ein` takes 49 (`python3
  build/s1f.6.10/t.py OUT ein solve build/s1f.6.10/p/p3_send.ein --single`,
  2026-11-12). **Three findings, none a wrong answer** (the phase's
  `findings.md`): E-29, the obligation tree branches in the order the
  obligations are written, so the same cryptarithm with its letters declared
  as written had no answer in 600 s; E-30, `ein test` decides a claim by
  describing R, so a claim `--single` decides in half a second is NOT
  CHECKED; E-31, describing R spends its time on a hypergraph transversal
  (`ein_infer::transversal`), 94 % of 5.1 s on a five-digit equation. **An
  equation with no solution reads three ways**: silence, an R that is empty
  because every alive node owes (`Open`, E-23), or a dead root whose unsat
  core is the proof, three table rows, where the table makes the unknown's
  digit unique. **Of a theorem Ein checks the finite part**: the laws of
  arithmetic on every instance of a domain, and the full adder's 200 rows
  against addition by counting; *for every number* and the induction on
  columns are outside it, [M1g](plans/m1g_logic/README.md)'s. Number theory
  by saturation, primes to 5 000, casting out nines to 9 999, perfect
  numbers to 250, Collatz, gcd, each agrees with Python, and memory is the
  limit (E-26). **Sixteen probes are banked under
  [`examples/beyond_arith/`](examples/beyond_arith/)**, beside
  `examples/arith/` and not in it, since `arith_search_free.rs` holds every
  file there to *no candidate* and four of these search; each has a claim
  and a corpus entry, sized so that a bare `ein solve` exhausts R. Eight of
  them compute through `std.arith`'s tagged calls, `add-walk`, `mul-call`,
  `div-call` and `cmp-walk`, since a check written with ordinary joins on
  numbers took 16.8 s where the tagged form takes 0.42 (E-3); whether those
  calls are an interface is the user's
  ([Q-M1f.189](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f189--a-users-program-calling-stdariths-tagged-calls)).
  **What becomes a stage is the user's**: the page recommends inverse
  requests in `std.arith`, `ein test` by the cheapest route that decides a
  claim, and a small cryptarithm theory after it. The register is unmoved.
  **Every question of P1f.6 that waits for the user is in one file,
  [`questions.md`](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md)**
  (2026-11-12).
  **The prototypes of S1f.6.16 and S1f.6.17 were banked as examples on
  2026-11-12**, on the user's *"bank binary arithmetic and logic prototypes
  into examples too"*
  ([S1f.6.16's record](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.16_binary_arithmetic.md#record-2026-11-12-way-b-banked-as-examples)
  and [S1f.6.17's](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.17_binary_logic.md#record-2026-11-12-the-recommended-variants-banked-as-examples),
  Q-M1f.190 to .193 defaulted). **They are examples and not stdlib modules**: the
  theory modules live in the two directories and the programs import them
  file-relative. **Banking the recommended variants is no ruling: the
  questions of [`binary_arithmetic.md`](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/binary_arithmetic.md)
  and [`binary_logic.md`](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/binary_logic.md)
  stay open.** [`examples/binary_arithmetic/`](examples/binary_arithmetic/README.md)
  is the page's way (b), a binary theory of its own: `bits.ein`, three tables
  of 28 rows, and `bin.ein`, a number `(b2 PREFIX BIT)`, the five operations
  as `add2` to `div2`, 67 rules and the closing rule `bin` (`python3
  examples/binary_arithmetic/gen_binary.py --count`, 2026-11-12); `conv.ein`,
  decimal to binary and back, eleven rules over both theories' tagged calls;
  and five programs with claims. **`bits.ein` and `bin.ein` are generated**,
  from `stdlib/arith.ein`, by `gen_binary.py` in that directory: edit the
  script, never the files. **The generator is run by hand and is not in the
  gate**, so an edit to `stdlib/arith.ein` fails no test and makes its
  `--check` exit 1 until it is run again.
  [`examples/binary_logic/`](examples/binary_logic/README.md) is the one
  table of the sixteen two-input functions, `(gate2 NAME A B OUT)` in
  `gates.ein`; what is known of them as generated facts, `props.ein`, with the
  31 rules that derive every one from the rows, `props_rules.ein`, and
  **`props_check.ein`, whose thirty claims hold exactly when the two agree,
  which is how that check is in the gate**; the theories `circuit.ein`,
  `inputs.ein` and `require.ein`; and the six applications, each a program:
  simulation, the truth table, satisfiability, equivalence, knights and
  knaves, and the synthesis of one gate, with the full adder from five gates
  against S1f.6.16's table, a simplifier and Post's criterion on seven sets.
  Its four generated files are `gen_gates.py`'s, hand-run as the other. Every
  claim was computed in Python apart from the engine, and no answer differs
  from the pages'. **An unsatisfiable search reads as owing there** (E-23):
  `unsatisfiable.ein` and `equivalent.ein` claim the owing frontier and
  `(false)` for the query, and answer `Open` under `--single`. The programs
  are sized for the corpus, three inputs and short operands, none `slow`;
  the scale runs, the batches and Post's criterion over all 65 535 sets stay
  on the pages and under `build/`. The stage found E-32: a file-relative
  import cannot reach a sibling directory, so `examples/binary_logic/bits.ein`
  is a copy that `gen_binary.py --check` holds equal. No engine source and no
  stdlib module moved; the register is unmoved.
  **S1f.6.20's prototype was banked on 2026-11-12**, on the user's *"bank word
  problems into examples/word_problems with recommended variants"*
  ([its record](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.20_word_problems.md#t2-record-2026-11-12-the-prototype-banked);
  Q-M1f.194 to .198 defaulted). **[`examples/word_problems/`](examples/word_problems/README.md)
  is elementary-school word problems with the calculator a library among
  other relations**: examples, not a stdlib module, three theory modules
  beside thirteen problems that import them by file name. `words.ein` is
  what the words mean, as rules: a plan backwards from the question,
  `(wanted Q)` and `(needs Q Q')`, then for each word a request to
  `std.arith` and its answer read back as `(value Q N)`, so nothing is
  computed that no question needs and the order of the operations is the
  derivation's. `gaps.ein` names a quantity the problem does not state, by
  `absent`, apart from `words`, which has none; `readings.ein` holds the two
  readings of *need* as hypotheses with nothing owed. The user's two-step
  problem, Bob 5, Alice 7, a box holds 3, answers 4 search-free
  (`boxes12.ein`); *how many together* after Bob gave 2 answers Bob's 3,
  gives *together* no row and names the gap, Alice's apples before
  (`gap.ein`); giving 7 of 5 is a flag and `(false)` (`contra.ein`). Every
  banked answer was re-taken on the banking commit's binary and stdlib and
  is the one [`word_problems.md`](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md)
  reports; each is the file's `:expect`, held by its `test` cell. **The
  banked variants are the page's recommendations and not rulings**: its
  twelve questions stay open. **The remainder variant, 13 apples, is not
  banked**, though the engine answers it as the page says: `region.rs`'s
  `fact_grain_never_calls_a_violated_cube_monotone`, a prototype analysis
  run over every corpus file, does not end on a program where a supposed
  reading leads to a `value`, which the word rules turn into a calculator
  request (E-33 of the phase's `findings.md`;
  [Q-M1f.198](plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f198--the-remainder-variant-of-the-word-problems-and-the-gate)).
  **A corpus program in which a hypothesis leads to a request to
  `std.arith` through a rule with an unbound operand may hang that test**;
  run it alone before a gate. No stdlib module and no engine source moved,
  and the register is unmoved.
  **The user replied to the phase's questions on 2026-11-12**
  ([`questions.md`](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md);
  what stayed open is
  [`questions_round2.md`](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md),
  replied in full the same day (`dc88520b`)),
  and **[`replies_2026-11-12.md`](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/replies_2026-11-12.md)
  is the index** from each of the 73 items to what it became, with the order
  the stages run in. Thirteen stages are planned and none started, S1f.6.21 to
  S1f.6.33: the gate reduced (big graphs skipped, no counters step), integer
  division under three conventions, the loader's arity check, division by zero
  flagged at the call, five findings fixed (E-5, E-8, E-11, E-20, E-22), a
  read-out row for what root owes, `ein test` by the cheapest route, word
  problems second pass, the biggest calculation (about 16 GB the bound, run by
  no test and no CI step), name clashes between imported
  theories (stops for the user), then **the stdlib layout, `std.math.*` and
  `std.bool.*`, with `std.arith` becoming `std.math.decimal-arith`**, and after
  it binary arithmetic and binary logic shipped, gcd and lcm, and `std.rat`
  reducing only when asked. Expressions (S1f.6.14) moved to P1f.8 (now P1g.10). Each stage
  says whether it rests on a typed reply, a reply in round 2, or a default,
  which stands on the user's "if something remain - pick the default and
  continue current phase". The same replies extended P1f.8 (S1f.8.10, S1f.8.11; now S1g.10.6, S1g.10.7), P1f.10
  (S1f.10.9; the order of the search takes the discovered structure as input)
  and M1g (P1g.7, unsatisfiable against owing; P1g.8, a theory of types;
  P1g.9, one equation by three methods, which the user moved out of P1f.8 (now P1g.10):
  "p1f.8 remain incomplete naive school algebra"; S1g.6.6), and created three milestones:
  [M1h](plans/m1h_beyond_word_problems/README.md), beyond word problems;
  [M15](plans/m15_mcp_server/README.md), an Ein MCP server;
  [M1j](plans/m1j_nlp_smt_poc/README.md), `nlp/` and `smt/` as one proof of
  concept.
  **On 2026-11-13 the user split P1f.8 and moved it to M1g** as
  [P1g.10](plans/m1g_logic/p1g.10_school_algebra/README.md), school algebra
  (S1g.10.1 to .7), and
  [P1g.11](plans/m1g_logic/p1g.11_school_geometry/README.md), school geometry
  (S1g.11.1 to .4), with nothing redesigned: M1f's number 8 is used and
  vacated, and the old path keeps a stub with the table of old and new ids.
  P1g.6 gained a provisional stage, S1g.6.7, the candidate engine features
  *theorems as rules* (S1g.11.2) may need, candidates only.
  **S1f.6.21, the gate reduced, ran to its end on 2026-11-12**
  ([its record](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.21_the_gate_reduced.md#record-2026-11-12),
  Q-M1f.199 defaulted), on the user's two replies of that day. **A view of
  more than 1 000 edges is skipped by `dot_wellformed.rs`**, counted and
  named in the test's output and put to Graphviz under no layout (*"don't
  test graphviz itself - that is the main point here. just skip big graphs,
  run tests only on small"*): 6 524 views accepted and 663 skipped, of 111
  files (`cargo test -p ein-render --test dot_wellformed -- --nocapture`,
  2026-11-12). **Nothing checks the DOT text of a skipped view**: those are
  the views that hold the digit tables and three rule libraries wide enough
  on their own. The test names the directories that hold a skipped view,
  `SKIPPING`, and **a program with a big view in a directory that had none
  fails the sweep until the directory is named there**. **The counters step
  left the gate** (*"we don't need this proportional gate"*): `cargo test -p
  ein-infer --features counters --test arith_batch` is in no tier, and **a
  stage that changes a rule of the calculator runs it by hand**; the static
  half, the rule of the theory on each module's text, stays per commit.
  `fdp` and its layout plugin stay installed and checked, since the review
  bundle draws KB views with it. No DOT writer, engine source or stdlib
  module moved; the register is unmoved.
  **S1f.6.22, integer division, ran to its end on 2026-11-12**
  ([its record](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.22_integer_division.md#record-2026-11-12),
  Q-M1f.200 defaulted), on the user's rulings of that day on Q-M1f.179:
  *"(d)"*, more than one convention, each under a request of its own, and
  *"(a)"*, the Euclidean remainder a natural number. **`std.int` divides
  under three conventions and there is no bare `int-div`**:
  `(int-div-euclid X Y)` answered by `(int-quot-euclid-of X Y Q R)`, 0 ≤ R <
  |Y|, Q an integer and **R a natural number of `std.arith`**;
  `(int-div-trunc X Y)` by `(int-quot-trunc-of X Y Q R)`, Q toward zero and R
  with the dividend's sign; `(int-div-floor X Y)` by `(int-quot-floor-of X Y
  Q R)`, Q rounded down and R with the divisor's sign, both integers. −7 ÷ 2
  is −4 and 1, −3 and −1, −4 and 1. 34 rules, 70 in the module beside its
  closing rule and 341 over the stdlib (`cat stdlib/*.ein | grep -c '^(rule
  '`, 2026-11-12), every one of one premise. What the three share is written
  once, under the tag `(int-div-of REQUEST)`: both operands simplified, so
  that each shows its sign by its shape, and the sizes handed to
  `std.arith`'s `div-call` with the signs in the tag, the names `pos` and
  `neg`; truncated then puts the signs on, and floored and Euclidean adjust
  two of the four combinations where a remainder is left, an addition walk
  and a subtraction walk. −7 ÷ 2 is 124 firings under Euclidean and floored
  and 112 under truncated (`cargo test -p ein-infer --test int_theory
  the_circuit`, 2026-11-12), the prototype's numbers. **A divisor that is
  zero by value derives `std.arith`'s flag**, `(division-by-zero REQUEST)`,
  the request as written, by `int-div-by-zero`, and `std.arith`'s rule
  derives `(false)`; no rule of the module concludes `(false)`.
  `int_theory.rs` holds the three as rows of `OPS`, against `i128`'s
  `div_euclid` and `rem_euclid`, `/` and `%`, and the floored pair computed
  from them, on every case the other rows are asked; six programs under
  `tests/stdlib/int/` activate the rules, three of them claiming `(false)`;
  `arith_search_free.rs`'s lists took the requests in; the mutation sweep
  kills all 18 of the new mutants, 379 of 429, the bank unmoved (`python3
  utils/stdlib_mutants.py`, 2026-11-12); and **the batch was run by hand**,
  as S1f.6.21 left it: 4 606, 4 557 and 4 610 candidates a request at 250
  requests and no more at 1 000 and 4 000 (`cargo test -p ein-infer
  --features counters --test arith_batch a_batch_of_integer -- --nocapture`,
  2026-11-12). `examples/arith/int.ein` has the three divisions of −7 by 2
  side by side. `std.arith` and `std.rat` are unchanged; a program that
  imports `std.int` or `std.rat` pays about 2 ms more of load. No engine
  source moved and no existing program's `ein solve` (`python3
  build/s1f.6.22/sweep.py`, 804 files, 2026-11-12); the register is unmoved.
  **S1f.6.23, the loader's arity check, ran to its end on 2026-11-12**
  ([its records](plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.23_the_loaders_arity_check.md#t1-record-2026-11-12-the-census-and-the-edges),
  Q-M1f.201 to .204 defaulted), an engine stage, on the user's ruling of
  that day on question 3 of the phase's `questions.md`, *"(a)"*. **A form
  headed by a declared relation has as many arguments as its declaration has
  types, or the program is refused at load**, on every command, one line a
  form, at the form: `t.ein:3:2: the fact (likes Bob) gives likes 1 argument,
  and (relation likes Person Drink) declares 2 — …` (`ein_ir::from_ir`'s
  `wrong_counts`, `defined_behaviour.md` § 4.26). It reads a written fact
  and every fact inside it, a fact's `:using`, a rule's and an hrule's
  `:match` and `:assert` as their macros expand, a query's `:goal` and the
  facts a claim lists; a declaration is the program's own or an imported
  one, `o.name` under `:as o`; a bare `(relation flag)` declares none, so
  `(flag)` is its one form; and a keyword pair is no argument. **It does not
  read** a relation nothing declares, the kernel's `__closed__`,
  `__symmetric__` and `relation` among them, a relation written as a
  variable, `(?R ?a ?b)`, an activator against its rule's parameters, a
  `(solve …)` list or a macro nothing calls, and **it is no type check**:
  `(likes Tea Ann)` loads. Until the stage nothing held a form to its
  declaration, so the calculator's `(add (n z 5))`, `(n 1)` as an operand
  and a `std.num9` number of eight digits were answered by silence, exit 0,
  a premise of another count matched nothing and a conclusion of another
  count stored facts no premise of the declared count read (E-2 of the
  phase's `findings.md`, closed). W89, seven rows, filed `open` in a commit
  of their own and fixed by the check: 312 fixed, 16 open and 1 not wrong
  (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-12).
  **No program that loaded is refused**: the sweep, the release binary
  before the check against the one after over 817 `.ein` files, moves the
  seven witnesses alone (`python3 build/s1f.6.23/sweep.py`, 2026-11-12), and
  the loader's own run over the Rust suites found five declarations
  miswritten beside a use that was meant, in three tests and one
  `examples/broken/` fixture, four rewritten and one removed. The pins are
  `ein-cli/tests/declared_counts.rs` and `fuzz_properties.rs`'s
  `no_accepted_program_holds_a_declared_relation_at_another_count`, which
  reads the loaded store and the compiled plans. **A program that declares a
  relation and then writes it at two counts no longer loads**, so a new
  fixture that wants an undeclared, any-count relation leaves it
  undeclared. What the stage found outside its scope is E-34, an activator
  of another count than its rule has parameters, which activates nothing in
  silence, and E-35, a kernel marker's fact of another count and a fact of
  a declared relation derived at another count through a parameter.
  **Two plans were added on 2026-11-11, on the user's instructions of that day.**
  [M1g](plans/m1g_logic/README.md), *logic in Ein*, runs after M1f is complete;
  no second milestone was made for its engine work (the id M1h has since been
  taken by another subject, 2026-11-12): propositional logic with valuations as hypotheses,
  proof systems, first-order logic, Peano and induction, and the engine work
  they turn out to need; it promotes F1b, and P1f.6's S1f.6.16 and S1f.6.17,
  binary arithmetic and binary logic, are its starting point.
  [F26](plans/followups/f26_consistency_and_completeness.md) sets Ein's
  *alive*, *owes* and *exhausted* beside logic's consistency and completeness.
  **A third plan was added on 2026-11-12, on the user's instruction of that
  day:** [M12](plans/m12_rules_as_data/README.md), *rules as data*, which
  gathers what the tree says of it (F5, `08_self_describing.md` § 5, ideas
  01, 06 and 10, F7, F8's C(n), M1g's S1g.6.5, F24, F26, P1g.10's S1g.10.7)
  and designs nothing. Its first phase, the census of rules, ran the same
  day: **[`rule_census.md`](plans/m12_rules_as_data/rule_census.md)** is the
  results page, shapes of rules with names abstracted, ways to group them and
  a catalogue of connections between rules, observations and no goal (*"No
  specific goals right now, want to see analyze results"*). Its tables are
  written by `python3 utils/rule_census.py --page
  plans/m12_rules_as_data/rule_census.md`: edit the script, never the tables,
  and re-take it when the corpus has grown. Later phases are the user's to
  define.
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
  measurement, `utils/corpus_cost.py` re-takes it). **Fourteen** entries are
  slow since M1e2 S1e2.1c.2 (re-taken 2026-11-03, `utils/bench_env.sh --cores
  PT python3 utils/corpus_cost.py`). Describe takes the state route since
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
  [M1j](plans/m1j_nlp_smt_poc/README.md) (planned 2026-11-12) puts the two
  into one folder as a proof of concept, Zebra sentences through link grammar
  to SMT code.

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
EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test corpus_cli  # + the slow entries, which the_slow_set_is_exactly_these_fourteen names
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
`ein-test-report/4`, a query row carrying its classes and, since M1e2
S1e2.3.10, every row what the universe leaves out. The read-out's
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
