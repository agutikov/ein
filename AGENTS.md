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
  S1e2.2.5, each refused with the stage and why), the 78 CLI options, the two `EIN_*` names the binary
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
  (`ein-cli/tests/answer_round_trip.rs`), and the register is unmoved. Next
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
  **Since M1e S1e.2.2 every one of the pages — 45 since M1e2 S1e2.1.2 added
  `inference/answer_dump.md`, S1e2.1a.1 `inference/region_dump.md`, S1e2.1b.6
  `inference/monotonicity.md` and M1f S1f.5.16 `inference/read_out.md` and its
  examples page (`git ls-files 'docs/kernel/*.md' | wc -l`, 2026-11-09) — is in
  exactly one of three
  declared states, visible from the page rather than from a plan** —
  [README § Which pages to trust](docs/kernel/README.md): *current* (42, no
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
  keeps the other 40 honest is [README § Keeping this
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
  (`ein-infer/tests/stdlib_coverage.rs`, 0.04 s): every stdlib rule is
  activated by a program *here* — **77 of 77**, with no `examples/` entry
  contributing — and every program here states an expectation. **Activation is
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
  and the **only crate that is not `#![forbid(unsafe_code)]`**: its `cast.rs`
  is the one audited module design/12 §2 permits `unsafe` in, which is why it
  is a crate at all), `ein-render` (DOT views, the read-out, the region trace,
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
- **`utils/`** — **thirty-one scripts** (`ls utils/*.py utils/*.sh | wc -l`,
  2026-11-08), **all but one driving `ein.rs`**.
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
  a cargo test, in-process and scoped to `tests/stdlib/`. It **runs nightly**
  since S1e.4.5, beside `stdlib_mutants.py --check`. What stays here is the
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

## Running the gate

`./build.sh` builds everything first (see above); this runs it.

```sh
./run_tests.sh                                               # the whole per-commit tier
./run_tests.sh --tests-only                                  # skip the static checks
./run_tests.sh --nightly                                     # M1e2 S1e2.3.6: instead of the gate, every step of nightly.yml, its three jobs in order
EIN_FUZZ_ITERS=1000000 EIN_FUZZ_SEED=1 cargo test … -p ein-ir --release --test fuzz_properties   # the nightly's fuzz job, once per seed (1, 987654321)
cargo bench … --bench engine -- --test                       # the nightly's bench job, and the gate's last step
cargo test --manifest-path ein.rs/Cargo.toml --workspace     # the tests alone
EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test corpus_cli  # + the 14 slow entries
EIN_ID_SEEDS=8    cargo test … -p ein-render --test id_order_invariance
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
`(config …)` fields and the 78 CLI options beside it and a *does it change the
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
