# S1f.5.18 — An activator's nested fact: a value parameter binds it, and a relation parameter is refused at load

**Phase:** [P1f.5](README.md) (The no-owner items, then NL and `:priority`)

**Status:** specified 2026-11-08 on the user's two rulings of that day:
[Q-M1f.3](../open_questions.md#q-m1f3--three-wrong-answers-the-census-found-with-no-row) (file W33 and give it this
stage) and [Q-M1f.1](../open_questions.md#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused) (b), whose
load-time check is the half of G-14 no grammar can see. The user ruled on 2026-11-09 *"just continue with default
decisions till the end of the phase"*. **Started 2026-11-09**, in lane 2's worktree (`p1f5-lane2`), after S1f.5.10
(`8249e872`): T1f.5.18.1 ([its record](#t1f5181-record-2026-11-09)), every move listed, W59's two rows filed, four
questions taken by default (Q-M1f.41 to Q-M1f.44).
**Depends on:** [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md), which files W33 as an `open` row first.
It is independent of the grammar stages. It touches `ein-infer/src/compile.rs`, as
[S1f.5.7](s1f.5.7_eq_and_neq_are_relations.md) and [S1f.5.10](s1f.5.10_a_conclusion_variable_must_be_bound.md)
do, so it runs after them and before [S1f.5.9](s1f.5.9_an_empty_match_fires.md).

## The case

### W33: a value parameter bound to a nested fact stays unbound

Found 2026-11-08 by [S1f.5.5's census](s1f.5.5_census.md#findings-outside-the-grammar), on `ec004a48`'s
release binary:

```lisp
(relation p T) (relation hit T)
(p a) (p b) (p (f a))
(rule act (?y) :match (p ?y) :assert (hit ?y))
(act (f a))
```

It derives `(hit a)`, `(hit b)` and `(hit (f a))`. It should derive `(hit (f a))` only. When a plan is
compiled, `ein_infer::compile` seeds the rule's parameters from the activator's arguments, and skips an argument
that is a nested fact (`compile.rs`: *"A nested Fact does not bind"*). The parameter is then a free variable of
the plan, and `(p ?y)` matches every `p`. W24 (M1e2 S1e2.2.24) made that skip a refusal where the parameter
names a relation, through `fact_bound`. Where the parameter is a value, the skip is a wrong answer in silence.

A nested fact is a value: a fact can hold one as an argument, a pattern can match one, and the printer
prints one. So a parameter binds to it as it binds to a name.

### G-14's derived activator: a relation parameter a rule can bind to a fact

[S1f.5.5's paper grammar, Q1](s1f.5.5_paper_grammar.md#q1--g-14-with-assert-checked-by-the-grammar-is-a-bad-conclusion-that-never-fires-refused)
showed the one shape of G-14 a grammar cannot see. An activator that a rule **derives** binds a relation
parameter to a fact:

| program | today, both parsers |
|---|---|
| [`g14/derived-activator.ein`](s1f.5.5_parser_probe/programs/g14/derived-activator.ein): `(rule r (?R) :match (box (?R b)) …)`, `(rule mk () :match (item ?f) :assert (r ?f))`, `(item (a b))` | the run stops when `(r (a b))`'s plan is compiled: *the relation head ?R of (?R b) … is bound to the fact (a b)*, exit 1 |
| [`g14/derived-activator-never.ein`](s1f.5.5_parser_probe/programs/g14/derived-activator-never.ein): the same without `(item (a b))` | answers, 64 states, since no node derives the activator |

**The user ruled (b), 2026-11-08** (*"Q1 - b"*). Both programs are refused at load, by a static check that
some rule could derive an activator binding a relation parameter to a fact. The second program is refused
too, although no node derives the activator. The user chose (b) over (a) knowing it.

## What it changes

- **W33: the parameter binds.** A value parameter whose activator argument is a nested fact is seeded with
  that value, as a name is, and the plan matches only facts holding it. The compile key is already the
  activator's values (W16, S1e2.2.16), so two activators that differ only in a nested fact compile two
  plans. Every analysis that reads activators gets the bound value: `forbid`, `monotone`, the static check
  and `invariant::program_names`. The stage lists any that changes its answer before it lands.
- **W58, and `FireError::UnboundVar` retired** (added by S1f.5.10 on 2026-11-09, [Q-M1f.37](../open_questions.md#q-m1f37--fireerrorunboundvars-last-path-w33s-cause), taken by default). W33's cause has a second symptom: a value parameter its activator binds to a nested fact, read only by the `:assert`, reaches the conclusion unbound, and the run stops with `FireError::UnboundVar`'s `KeyError: "unbound var ?y in :assert — bindings: {…}"` (`W58/parameter-nested-assert`, `open` and this stage's). Since S1f.5.10's load-time check it is the one path to that line, so the seed above fixes W58 too, and this stage retires the variant and its line (`firing.rs`, `common.rs`, `defined_behaviour.md` § 4's row and § 4.13) and `fuzz_properties`' parameter exception in `no_firing_meets_an_unbound_variable`. S1f.5.10's acceptance item *`FireError::UnboundVar` is gone* is this stage's.
- **The relation half stays refused.** A relation parameter bound to a nested fact is refused where the plan
  is compiled, as W24 made it. Since that refusal can now come only from a derived activator, the run-time
  line remains as the backstop behind the load-time check below.
- **The load-time check (Q-M1f.1 (b)).** At load, the program is refused when some rule's `:assert` writes an
  activator of a rule, in a relation parameter's position, with a term that could be a fact. Such a term is
  either a nested fact written there, or a variable whose premise binds it at a position that can hold a
  fact. One line per site, in Ein's spelling, naming the deriving rule, the activated rule and the parameter.
  Every command that loads exits 1, and `ein test` exits 2.

  **Defining *can hold a fact* is the stage's design work, and two constraints bound it.** A precise flow
  analysis would accept `derived-activator-never.ein`, since no fact or candidate puts a fact in `item`'s
  argument, but the ruling refuses it. A blanket reading, where any premise variable counts, would refuse
  zebra2's fan-out rules, which pass relation names on through `(co-located ?R1 ?V1 ?R2 ?V2)`. The check
  sits between the two: a position whose values are relation names is one that the program uses as a
  relation (a relation parameter, a `(?R …)` head, or a relation signature's name), and everything else
  can hold a fact. Step 1 prototypes it and sweeps the corpus. **If it refuses a corpus program that
  answers today, or no reading both refuses the two `g14` programs and keeps the corpus, the stage stops and
  shows the user** the programs and the readings, before any edit.

## The method

### Task T1f.5.18.1 — every move listed

W33's row re-taken on a fresh build, starting from S1f.5.0's row. Both changes prototyped. The corpus and
`corpus/fuzz_findings/` swept with every command that loads, comparing each run against today's. Every move
is listed: W33's witness turning right, and any program the check refuses, with its deriving rule.

### Task T1f.5.18.2 — W33 fixed

The seed, W33's and W58's rows `fixed` in the same commit, `FireError::UnboundVar` retired, and a test in `ein-infer` beside W24's that holds a value
parameter bound to a nested fact, in a single activator and in a derived one. `./run_tests.sh` green.

### Task T1f.5.18.3 — the load-time check

The check, its line, and the two `g14` programs banked as refusal fixtures under `examples/broken/load/` with
their `.expected`. A test holds each shape: a written nested fact, a variable from a value position, and the
zebra2 shape, which must load. `./run_tests.sh` green.

### Task T1f.5.18.4 — the pages

`defined_behaviour.md` § 4.9 (W24's rule shapes) gains the load-time line, `01_grammar.md`'s activator section
says that a parameter binds any value, nested facts included, and the record: this file's Status, the phase
README's row, Q-M1f.1 and Q-M1f.3 marked built, and a sentence in `AGENTS.md`.

## Acceptance

- `activator_nested_value`'s witness derives `(hit (f a))` only, and W33's row is `fixed`.
- `parameter-nested-assert`'s witness derives `(hit (f a))`, W58's row is `fixed`, and no run stops on an unbound conclusion variable: `FireError::UnboundVar` is gone (S1f.5.10's, moved here by Q-M1f.37).
- `g14/derived-activator.ein` and `g14/derived-activator-never.ein` are refused at load, one line each.
- No corpus program that answers today is refused, unless the user has seen it and agreed.
- `./run_tests.sh` is green.

## T1f.5.18.1 record (2026-11-09)

**Run on the user's instruction of 2026-11-09**, *"just continue with default decisions till the end of the phase"*:
every decision a reviewer would call one is taken by default and is a row of [`open_questions.md`](../open_questions.md),
Q-M1f.41 to Q-M1f.44. The stage ran in lane 2's worktree, on `p1f5-lane2` at `8249e872`, after S1f.5.7 and S1f.5.10, so
the generated parser is the only one, a guard binds a free variable over the program's names, and the loader refuses a
conclusion variable nothing binds.

**The case, re-taken** on `8249e872`'s release and debug builds alike (`build/s1f.5.18/case/`):

| program | `8249e872` |
|---|---|
| `W33/activator-nested-value` | root derives `(hit a)`, `(hit b)` and `(hit (f a))`, R {∅}, the query one class of three rows, exit 0 |
| `W58/parameter-nested-assert` | every command stops at root, exit 1, `unbound var ?y in :assert — bindings: {'x': 'a'}` (`saturate`: `KeyError: "…"`) |
| `g14/derived-activator.ein` | loads; root derives `(r (a b))`, and every command stops where `r`'s plan is compiled, exit 1, W24's line *the relation head ?R of (?R b) at Loc(…) is bound to the fact (a b) — a relation is named by a symbol, not a fact.* |
| `g14/derived-activator-never.ein` | loads and answers, exit 0: H six candidates, R their cube, 64 states |

So the case stands as the stage file has it.

**The seed, prototyped** (`build/s1f.5.18/prototype.patch`, its `compile.rs` half): `compile::Compiler::bind_activator`
seeds a parameter its activator binds to a nested fact as it seeds one bound to a name. A relation head written as such a
parameter meets the seeded fact and is refused where it was (`head_not_a_name`), so W24's backstop holds. W33's witness
then derives `(hit (f a))` alone, and W58's `(hit (f a))` at root, with no stop.

**The load-time check, prototyped** (the patch's `from_ir.rs` half, a census behind `EIN_S1F518`). *What can hold a
fact* is [Q-M1f.41](../open_questions.md#q-m1f41--what-can-hold-a-fact-means-for-g-14s-derived-activator), taken by
default as (b). The census read every program of the five roots three ways (`build/s1f.5.18/census/run.sh`, 538 files):

- **The stage file's sketch**, where a variable bound anywhere but a relation parameter, a `(?R …)` head or a signature's
  name can hold a fact, refuses the two `g14` programs and **32 corpus programs that answer today**: `zebra.ein`, the six
  `zebra2` programs, `domain_elim/ab`, `b_branch` and `b_only`, `features/09_adjacent_via_same_house`, `stdlib/algebra`,
  `bijection`, `slots` and `typing` loaded alone, and sixteen programs of `tests/stdlib/` (`algebra/03`, `06`,
  `bijection/01`, `02`, `03`, `06`, `slots/01`, `02`, `03`, `04`, `07`, `08`, `13`, `typing/01`, `02`, `03`). Each passes a
  relation name on through a fan-out rule that reads a marker relation, `(bijective ?R)`, `(bijection-hierarchy ?isa)`,
  `(typecheck-hierarchy ?isa)`, `(slot-partition ?R ?isa ?sub …)`, `(slot-spatial ?R ?S ?isa …)`, `(adjacent-via ?S ?R1
  ?V1 …)`, `(join ?R1 ?R2 ?R3)`, `(reflexive ?R)`, `(type-hierarchy ?isR*)`.
- **The typed reading, Q-M1f.41 (b)**: a declared relation's argument, one inside a fact pattern and one of a relation a
  parameter names can hold a fact; every argument of `(relation …)` holds a name; a marker's argument and a rule's
  parameter hold what the program passes on from them, inferred to a fixpoint. It infers 20 marker positions over 32
  programs, the fan-outs above, checks 851 derived activators' arguments at a relation position, and refuses **the two
  `g14` programs and nothing else**. No fact and no conclusion of the corpus writes a fact at an inferred position.
- **A conclusion written `(?T …)`** ([Q-M1f.42](../open_questions.md#q-m1f42--a-conclusion-whose-relation-is-a-parameter),
  (a)) is followed into every rule of its arity whose name the program writes as an argument; no corpus conclusion
  reaches one, and `build/s1f.5.18/shapes/varhead-copy.ein`, a `copy` rule that writes `(?T ?x)` with `(copy src
  mirror)` and `(src (q A B))`, is refused, where today its run stops where `mirror`'s plan is compiled.

So the stage goes on: a reading refuses both `g14` programs and keeps the corpus, which is the stage file's condition.

**Every program of the five roots, through both builds** (`build/s1f.5.18/xcheck/run_all.sh`, the cross-check of
S1f.5.10 with `solve --minimal` and the event stream added: `solve --json-summary -H --dump-answer --events`, `solve
--minimal`, `saturate --dump`, `test` and `oracle`, the exit, stdout and stderr of each, the summary, the events and every
file the dump wrote compared, timings masked; 538 files, `zebra2-minus-15.ein` re-run alone where the comparison of ten
in parallel ran out of memory). With the seed alone (185 s) **530 are the same and eight move**; with the check too
(207 s), the same eight and **the two `g14` programs**:

| program | what moves | why |
|---|---|---|
| `wrong-answers/activator-nested-value.ein` | root holds `(hit (f a))` alone; H gains `(hit a)` and `(hit b)`, which root no longer holds, so R is their cube, 4 states, the minimal line still root's; the query's class one row | W33 fixed |
| `wrong-answers/parameter-nested-assert.ein` | every command answers, exit 0, root holding `(hit (f a))`, R two states, where it stopped | W58 fixed |
| `ein-bugs/binding-key-both.ein`, `-swapped.ein`, `-nested-only.ein` | `(note edge (src Y))` binds `?f` to `(src Y)`, and no `(holds (src Y))` is a fact, so it derives nothing; each `:expect`, which stated the four `noted` facts the unbound `?f` derived, is refuted, exit 1 | each `:expect` stated W33's wrong answer, and T1f.5.18.2 restates it |
| `corpus/fuzz_findings/binding-key-layout.ein`, `-swapped.ein` | the same two `noted` facts fewer at root | the same; no claim |
| `plans/m1e2_correctness/probes/binding-key-nested-only.ein` | the same; its R, past the budget, counts other members | the same; a plan probe |
| `g14/derived-activator.ein` | refused at load, every command | W59 |
| `g14/derived-activator-never.ein` | refused at load, every command | W59 |

W7's `[case.right]`, `W7/binding-key-layout`'s, states the twelve-fact maximal state the unbound `?f` derives (*the
nested-fact activator alone derives every (noted ?a ?f)*), W33's wrong answer; its `now`, the `holds` facts alone, does
not move, and T1f.5.18.2 restates its `right`, as it does `ein-infer`'s `rule_semantics` controls that state the same.

**What the analyses that read activators do with the bound value** (the stage file's list): `forbid`, `monotone`, the
static check and `invariant::program_names` each read a plan, whose seed holds the value. On every program of the sweep
none changes its answer: no refusal moves, the static check's verdict and sites are the same on all 538, and the
program's names are the same; on W33's witness the summary's `monotonicity.hypothesised` and `dynamic` gain `hit`, since
H gains its two candidates. `Plan::activator_args`, which keeps an activator's symbol arguments only, is read as a plan's
identity by four of them (`oblgen`'s `InstanceKey`, `region.rs`' owed-instance key, `detector`'s `RootPlans`, `naf_deps`'
key), beside bindings that hold no seeded register, so two plans the seed tells apart would share one:
[Q-M1f.43](../open_questions.md#q-m1f43--planactivator_args-once-a-nested-fact-binds), taken by default as (a), makes it
every argument, which T1f.5.18.2 builds.

**The rows** ([Q-M1f.44](../open_questions.md#q-m1f44--what-the-load-time-check-replaces-which-of-it-is-a-register-row),
Q-M1f.36's reading): each way the engine answers a program the check refuses is a row whose `right` is the refusal,
filed `open`, owned by S1f.5.18, each witness the `g14` probe with `:why`s added, under `examples/ein-bugs/wrong-answers/`
with a corpus entry, each `wrong` and `now` taken at `8249e872`, release and debug alike:

| row | witness | `now`, today | `right` |
|---|---|---|---|
| `W59/derived-activator` | `derived-activator.ein`: root derives `(r (a b))` | every command stops, exit 1, W24's compile line | refused at load |
| `W59/derived-activator-never` | `derived-activator-never.ein`: no `item` fact | answers, 64 states, exit 0 | refused at load |

W33 and W58 stay `open` until T1f.5.18.2. The register reads **144 fixed, 22 open and 1 not wrong** (`grep '^status'
corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).

**What moves by banking**, with no engine change: `corpus_exits.txt` gains the two witnesses' four cells (`derived-activator`
1 and 1, `-never` 0 and 0), `corpus_shapes.md5` 82 digests, 41 per witness, none of any other program moved;
`examples/README.md` names the witnesses (159 → 161 programs under `ein-bugs/wrong-answers/`); `ein-infer`'s
`alive_invariant` lists their two names only a nested fact or a rule writes, `b` in `(box (a b))` and `z` in `r`'s
`:assert`; and `detector`'s count of the corpus programs the static check flags is 49 where it was 48,
`derived-activator-never.ein`'s `r` an `activator` site. The first gate run stopped at those two tests, which this commit
updates; the run its body names is the whole gate again, with `--no-fail-fast` forwarded to `cargo test`.

## T1f.5.18.2 record (2026-11-09)

**The seed** (`ein_infer::compile`): `Compiler::bind_activator` seeds every parameter from its activator, a nested fact as
a name, and returns the activator's every argument. The list of parameters bound to a fact, which `head_not_a_name` read
until then, is gone: a relation head written as such a parameter meets the seeded fact and is refused where it was, so
W24's compile-time line holds.

**`Plan::activator_args` is every argument, as values** ([Q-M1f.43](../open_questions.md#q-m1f43--planactivator_args-once-a-nested-fact-binds),
(a)), the compile key's list: `firing::Firing`, `obligations::Owed`, `oblgen::InstanceKey`, `forbid::Refusal`,
`monotone::Site` and `AbsentRead`, `naf_deps::NafDep` and `detector`'s root plans and sites hold values, ordered by
`Terms::cmp_semantic` (a name by its text, as before, and a nested fact after every name), and every surface that prints
one prints a name as it did and a fact as Ein text (`Terms::display`; `naf_deps::raw_text` where a label printed a
name's raw text). `detector`'s match of an owed instance against the activators a node holds compares every argument.

**`FireError::UnboundVar` is retired** (Q-M1f.37; S1f.5.10's acceptance item, moved here): the variant, its *unbound var
?y in :assert — bindings: …* line and `ein-cli`'s `KeyError: "…"` arm in `common::saturate_error_line` are gone. No
conclusion reads a register its match and its activator leave unbound, the loader refusing a variable no premise or
parameter binds (S1f.5.10) and the compiler seeding every parameter, so `firing::resolve` asserts it, naming both.
`fuzz_properties::no_firing_meets_an_unbound_variable` drops its parameter exception: a rule with parameters is compiled
only from its activators, whose seed binds each, and the stream holds every conclusion bound with none excepted, 425
programs loaded and 3 refused of 2 000 (`cargo test -p ein-ir --test fuzz_properties no_firing -- --nocapture`,
2026-11-09), the numbers S1f.5.10's run printed.

**The tests.**
- `ein-infer`'s `compile_semantics::a_value_parameter_an_activator_binds_to_a_nested_fact_is_bound`, beside W24's
  relation half: W33's program concludes `(hit (f a))` alone and W58's `(hit (f a))`, an activator a rule derives from a
  fact `src` holds binds the same, read by a premise and by a nested conclusion, and two activators that differ only in a
  nested fact each match their own value.
- `firing`'s `an_unbound_assert_var_quotes_the_bindings`, the variant's line, is replaced by
  `a_parameter_bound_to_a_nested_fact_is_concluded`: a plan compiled from `(r (f thing2))` keeps the fact in its activator
  list and writes it in the conclusion.
- `rule_semantics`' two W7 controls are restated: `activators_differing_only_by_a_nested_fact_argument_both_fire` counts
  as it did, the rule reading `?f` nowhere, and `an_int_beside_a_nested_fact_in_one_position_loses_no_derivation`, whose
  control stated *the nested-fact activator alone derives every (noted ?a ?f)*, W33's wrong answer, gains `(holds (src
  Y))`, so each activator derives its own two and the pair their union, in either order.
- `cli_semantics::a_conclusion_variable_nothing_binds_is_refused_at_load` holds W58's witness answering on `saturate`,
  `solve`, `solve --minimal` and `oracle`, exit 0, where it held the stop.
- `refutation_under_absent`'s list of *forbid*'s refusals: `activator-all-nested.ein`'s names its activator, `rule 'alarm'
  [activator (box a)]`, as its int and symbol twins name theirs.

**What moved** (`build/s1f.5.18/xcheck/run_all.sh t2`, this build against `8249e872`'s, 538 files, 529 the same,
`zebra2-minus-15.ein` re-run alone): T1f.5.18.1's eight, W33's and W58's witnesses turning right and the `binding-key`
family and its fuzz findings and plan probe, and one more, `activator-all-nested.ein`, whose *forbid* refusal and event
stream name its activator's fact (Q-M1f.43; its exit, 1, and its refusal unmoved). Every fixture whose `:expect` stated
W33's wrong answer is restated in this commit, with a section of its header saying so: `binding-key-both.ein` and
`-swapped.ein`, every state holding `(noted 1 1)` and `(noted 2 1)`, and `binding-key-nested-only.ein`, no state holding a
`noted` fact, which a model cannot say once no model names `noted`, so a query states it, `(query :goal (noted ?a ?f)
:expect (model))`, which `8249e872`'s build refutes.

**The register**: W33's and W58's rows `fixed`, each `now` the right answer (`solve --minimal`'s line, root's state
holding `(hit (f a))`) and a `[case.region]` for R, four states and two; their `[case.fixed]` carry the null id, which
the next commit writes. Two fixed rows are restated with no answer moving: `W7/binding-key-layout`'s `right`, which stated
the twelve-fact maximal state the unbound `?f` derived, now `(holds 1)`, `(holds 2)`, `(holds 3)`, `(holds Y)`, `(noted 1
1)` and `(noted 2 1)`; and `W16/activator-all-nested`'s `now`, the refusal's line naming `[activator (box a)]`. The
register reads **146 fixed, 20 open and 1 not wrong** (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`,
2026-11-09).

**Banked**: `corpus_exits.txt` two cells 1 → 0, W58's `solve` and `saturate`; `corpus_shapes.md5` 137 digests moved over
the six programs above (`activator-all-nested` 14, `binding-key-both` 26, `-nested-only` 28, `-swapped` 26, W33's witness
21, W58's 22), none of any other program.
