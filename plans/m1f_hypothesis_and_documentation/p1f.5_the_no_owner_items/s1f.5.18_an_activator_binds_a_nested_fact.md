# S1f.5.18 — An activator's nested fact: a value parameter binds it, and a relation parameter is refused at load

**Phase:** [P1f.5](README.md) (The no-owner items, then NL and `:priority`)

**Status:** specified 2026-11-08 on the user's two rulings of that day:
[Q-M1f.3](../open_questions.md#q-m1f3--three-wrong-answers-the-census-found-with-no-row) (file W33 and give it this
stage) and [Q-M1f.1](../open_questions.md#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused) (b), whose
load-time check is the half of G-14 no grammar can see. The user ruled on 2026-11-09 *"just continue with default
decisions till the end of the phase"*. **Done 2026-11-09**, in lane 2's worktree (`p1f5-lane2`), after S1f.5.10
(`8249e872`), in five commits: T1f.5.18.1 ([its record](#t1f5181-record-2026-11-09)), every move listed, W59's two rows
filed, four questions taken by default (Q-M1f.41 to Q-M1f.44) (`e96a8d45`); T1f.5.18.2
([its record](#t1f5182-record-2026-11-09)), the seed, W33 and W58 fixed, `FireError::UnboundVar` retired (`2901cac1`);
T1f.5.18.3 ([its record](#t1f5183-record-2026-11-09)), the load-time check, W59 fixed (`52d45b11`); T1f.5.18.4
([its record](#t1f5184-record-2026-11-09)), the pages (`73f9a448`); T1f.5.18.5, the acceptance re-taken
([§ Acceptance re-taken](#acceptance-re-taken-2026-11-09)) and what the merge writes ([§ For the merge](#for-the-merge)).
The register on the lane reads 148 fixed, 18 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort |
uniq -c`, 2026-11-09).
**Depends on:** [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md), which files W33 as an `open` row first.
It is independent of the grammar stages. It touches `ein-infer/src/compile.rs`, as
[S1f.5.7](s1f.5.7_eq_and_neq_are_relations.md) and [S1f.5.10](s1f.5.10_a_conclusion_variable_must_be_bound.md)
do, so it runs after them and before [S1f.5.9](s1f.5.9_an_empty_match_fires.md).
**Merged into master at `a90c452b`, 2026-11-09** (T1f.5.M.1, [P1f.5 § The order](README.md#the-order), item 10).

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

## T1f.5.18.3 record (2026-11-09)

**The check** (`ein_ir::from_ir`'s `refuse_derived_relation_activators`, after W24's `validate_relation_parameters`),
Q-M1f.1 (b) read as [Q-M1f.41](../open_questions.md#q-m1f41--what-can-hold-a-fact-means-for-g-14s-derived-activator)
(b) and [Q-M1f.42](../open_questions.md#q-m1f42--a-conclusion-whose-relation-is-a-parameter) (a):

- **Positions that hold relation names**: a rule's, an hrule's or an obligation's parameter that heads a form in it, and
  every argument of `(relation …)`. To a fixpoint, a marker's argument (a relation no `(relation …)` declares) and a
  rule's own parameter join them where a variable bound there reaches one through a conclusion: `bijective-properties`
  passes `(bijective ?R)`'s `?R` to `functional`'s `?R`, so `(bijective, 0)` holds relation names.
- **Sites**: every argument each top-level conclusion of a rule, an hrule or an obligation writes at such a position, a
  conclusion `(not P)` read as `P`; one written `(?T …)` writes into every rule of its arity whose name the program
  writes as an argument (a fact at any depth, a rule's clause, the `(solve …)` form).
- **Refused**, one line per site, `; `-joined with the loader's others, exit 1 (`ein test`: 2) on every command that
  loads: a variable that some disjunct of the deriving rule's `:match` binds only where a fact can stand, an argument of
  a declared relation, one inside a fact pattern, or one of a relation a parameter names; a fact a conclusion writes at
  an inferred position; and a fact a fact writes there (one at a rule's own parameter is W24's line, above).

```text
rule 'mk': its :assert writes (r ?f), whose ?f is rule 'r''s parameter ?R, which names a relation, and (item ?f) can
bind ?f to a fact, as an argument of the declared relation item — a relation is named by a symbol, not a fact
the fact (src (q A B)) writes (q A B) as ?f of (src ?f), which rule 'make' passes to rule 'mirror''s parameter ?R, which
names a relation — a relation is named by a symbol, not a fact
```

The compiler's refusal (`head_not_a_name`) stays, the backstop behind both loader checks; no program the loader accepts
reaches it, and a direct caller of `compile_rule` with such an activator does.

**The fixtures**: `examples/broken/load/derived_activator.ein` and `derived_activator_never.ein`, S1f.5.5's two `g14`
probes with `:why`s added, each with its `.expected`, the line above, and a corpus entry (`load-negative`, `solve`,
`saturate`, `render rules`); `examples/broken/load/README.md` gains their section.

**The tests.**
- `ein-ir`'s `from_ir::tests::a_derived_activator_binding_a_relation_parameter_to_a_fact_is_refused_at_load`: the line
  for each shape, a variable from a declared relation's argument (both `g14` programs), from inside a fact pattern, from
  a relation a parameter names, and through an hrule; a fact, a conclusion's literal fact and a conclusion's variable
  written at a marker's inferred position; a fact written as the activator of a rule whose parameter passes it on; a
  conclusion `(?T ?x)` into a rule whose name the program writes; and an `(or …)` one arm of which binds the variable where
  a name is held. What loads: a signature's name, `(relation ?f ?a)`; zebra2's shape, a rule's relation parameter passed
  on by a fan-out of the same rule's activators, `(co q a q b)`; a marker written with names that passes them on; an
  `eq` guard; and a value parameter bound to a fact through a derived activator.
- `ein-cli`'s `cli_semantics::a_derived_activator_binding_a_relation_parameter_to_a_fact_is_refused_at_load`: the two
  fixtures and W59's two witnesses on the eight commands that load, the exit and the line, with no compile line on
  stderr; and `examples/zebra2.ein` and `tests/stdlib/bijection/01_setup_and_negatives.ein`, which load and answer.
- `ein-infer`'s `compile_semantics::a_relation_parameter_a_rule_binds_to_a_nested_fact_is_refused`: the derived
  activator it held to the compiler's line, `make`'s `(mirror ?f)` over `(src (q A B))`, is refused at load, `src` a
  marker whose argument `make` passes on, with the fact line above; and the compiler's line stays, held by a direct
  `compile_rule` call with the activator `(mirror (q A B))`.

**What moved** (`build/s1f.5.18/xcheck/run_all.sh t3`, this build against `8249e872`'s, 542 files with the four this stage
banked, 527 the same, `zebra2-minus-15.ein` re-run alone): T1f.5.18.2's nine; W59's two witnesses and the two `g14`
probes under `plans/`, each refused at load, every command, where the first stopped where `r`'s plan was compiled and
the second answered; and the two new fixtures, refused, which `8249e872`'s build loads. No other program moves: the
check refuses nothing else in the five roots.

**Two programs written in Rust tests** were refused, which the sweep of the five roots could not see and the first gate
run found (`build/gate-s1f.5.18-T3-first.log`): `ein-cli`'s `program_read::a_containers_positions_name_its_stored_program`
wrote G-14's residue as the error past the load whose position a container names, and now writes another, a pattern
whose relation a parameter binds to `eq` written with one argument, `(rule r (?R) :match (?R ?x) …)` under `(r eq)`,
which the compiler refuses at the first firing (M1e S1e.2.1's arity check, M1f S1f.5.7's `eq`); and `ein-infer`'s
`stdlib_semantics::a_rule_that_derives_its_own_activator_still_reaches_a_fixpoint` declared `(relation mark T)` for a
marker that passes `a`'s relation name back to `a`, which the typed reading refuses, a declared argument holding any
value (Q-M1f.41): the declaration is dropped, so `mark` is a marker as the stdlib's `bijective` is, and the loop closes
as it did.

**The register**: W59's two rows `fixed`, each `now` the refusal and no region, `[case.fixed]` the null id, which the next
commit writes; W33's and W58's carry `2901cac1`, T1f.5.18.2's. The witnesses' headers gain a section saying what
replaced the answer, and their corpus notes say so. The register reads **148 fixed, 18 open and 1 not wrong** (`grep
'^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).

**Banked**: `corpus_exits.txt` gains the fixtures' six cells (`solve` and `saturate` 1, `render rules` 0) and moves
`derived-activator-never.ein`'s two, 0 → 1; `corpus_shapes.md5` removes 56 digests and adds 54: each witness loses the 21
renderings a loaded program draws and changes 7, and each fixture adds 20, none of any other program; `alive_invariant`
drops the two witnesses' names, which load no more, and `detector` counts 48 flagged programs again.

## T1f.5.18.4 record (2026-11-09)

**The pages.** No current kernel page describes a nested fact as binding no parameter, the `KeyError` line as reachable,
or a derived activator binding a relation parameter to a fact as loading:

- [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md): § 3.2's table and prose, an activator's every
  argument binding its parameter, a nested fact too, with what it did until then (W33, W58), `Plan::activator_args`
  (Q-M1f.43), and the `binding-key` family's restated `:expect` and its `--dump` since; § 4's table loses the `KeyError`
  row, a paragraph saying why (Q-M1f.37), and the class names Python spells are two; § 4.6's last paragraph; § 4.9's
  table, the derived activator's row the loader's line and a row for a fact written where a relation name is passed on,
  and a paragraph for the check, its reading (Q-M1f.41, Q-M1f.42), what it replaced (W59) and the compiler's line as the
  backstop; and § 4.13, nothing reaching the retired line since.
- [`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md): the parameter list says an activator binds each
  parameter to its argument, any value, a nested fact included, and a relation parameter bound to a fact is refused where
  written and where a rule could derive it; *What a premise may be* names the derived case.
- The inventory's [G-14](../../m1e2_correctness/inventory_2026-11-06.md) says its derived activator is done, and the
  census's [W33 finding](s1f.5.5_census.md#findings-outside-the-grammar) says what fixed it.

W59's two `[case.fixed]` blocks carry `52d45b11`, T1f.5.18.3's, where that commit wrote the null id. The phase README's
row, Q-M1f.1 and Q-M1f.3, `AGENTS.md` and the live read-out page are the merge's ([§ For the merge](#for-the-merge)).

## Acceptance re-taken (2026-11-09)

Each item, and what holds it:

- *`activator_nested_value`'s witness derives `(hit (f a))` only, and W33's row is `fixed`*: `ein saturate --dump
  examples/ein-bugs/wrong-answers/activator-nested-value.ein` lists `(hit (f a) :rule act)` and no other `hit`
  (2026-11-09, release and debug); `W33/activator-nested-value` is `fixed`, its `now` root's state holding `(hit (f a))`;
  `compile_semantics::a_value_parameter_an_activator_binds_to_a_nested_fact_is_bound` holds the same from a written
  activator and a derived one.
- *`parameter-nested-assert`'s witness derives `(hit (f a))`, W58's row is `fixed`, and no run stops on an unbound
  conclusion variable: `FireError::UnboundVar` is gone*: the witness's `saturate --dump` lists `(hit (f a) :rule act)`,
  every command exits 0 (`cli_semantics::a_conclusion_variable_nothing_binds_is_refused_at_load`), and
  `W58/parameter-nested-assert` is `fixed`. The variant is gone (`grep -rn "UnboundVar(" ein.rs/crates --include='*.rs'`
  finds nothing, 2026-11-09); `firing::resolve` asserts that no conclusion reads an unbound register, and
  `fuzz_properties::no_firing_meets_an_unbound_variable` holds every plan of every accepted program of its stream to it
  with no parameter excepted. S1f.5.10's acceptance item, moved here by Q-M1f.37, is met.
- *`g14/derived-activator.ein` and `g14/derived-activator-never.ein` are refused at load, one line each*: `ein solve` on
  each prints the one line and exits 1 (2026-11-09); the two refusal fixtures under `examples/broken/load/` hold the line
  in their `.expected`, W59's two witnesses hold it in the register, and
  `cli_semantics::a_derived_activator_binding_a_relation_parameter_to_a_fact_is_refused_at_load` holds it on the eight
  commands that load.
- *No corpus program that answers today is refused, unless the user has seen it and agreed*: **met**. The sweep of every
  program of the five roots against `8249e872` (`build/s1f.5.18/xcheck/run_all.sh t3`, 542 files) refuses W59's two
  witnesses and the two `g14` probes, the programs the ruling names, and nothing else; the reading that does so is
  Q-M1f.41 (b), taken by default, where the stage file's sketch would have refused 32 corpus programs. One program
  written in a Rust test, which no corpus root holds, declared the marker it fanned out from and was rewritten to leave
  it undeclared (T1f.5.18.3's record).
- *`./run_tests.sh` is green*: at every code commit, each body naming its log, with `--no-fail-fast` forwarded to `cargo
  test` (T1f.5.18.1, 16 min 21 s; T1f.5.18.2, 16 min 33 s; T1f.5.18.3, 17 min 7 s, each after a first run its body
  names); T1f.5.18.4 and this record are docs and the register's hashes, checked by `utils/doc_audit.py --links --check`
  and the register's readers.

**S1f.5.18 is done, 2026-11-09.**

## For the merge

S1f.5.18 ran in lane 2 (`p1f5-lane2`, after S1f.5.7 and S1f.5.10, from `dc4c0bdc`), which does not edit the prose both
lanes share. What those places should gain when the coordinator merges the branch:

1. **P1f.5's README, the stage table's S1f.5.18 row**, its state: *"**done 2026-11-09**, on lane 2 in five commits
   ([the records](s1f.5.18_an_activator_binds_a_nested_fact.md#t1f5181-record-2026-11-09)): T1f.5.18.1, every move
   listed, W59 filed (`e96a8d45`); T1f.5.18.2, a nested fact binds its parameter, W33 and W58 fixed,
   `FireError::UnboundVar` retired, `Plan::activator_args` every argument (`2901cac1`); T1f.5.18.3, the load-time check,
   W59 fixed (`52d45b11`); T1f.5.18.4, the pages (`73f9a448`); T1f.5.18.5, the acceptance re-taken.
   [Q-M1f.41](../open_questions.md#q-m1f41--what-can-hold-a-fact-means-for-g-14s-derived-activator) to Q-M1f.44 taken by
   default; on the lane the register reads 148 fixed, 18 open and 1 not wrong"*.
2. **[`open_questions.md`](../open_questions.md)'s Q-M1f.1 and Q-M1f.3** (outside lane 2's block): Q-M1f.1's index cell
   gains *"; **built 2026-11-09** by S1f.5.18 (T1f.5.18.3, W59): a program in which a rule could derive an activator
   binding a relation parameter to a fact is refused at load, read off the program (Q-M1f.41)"*, and its section ends
   with *"**Built 2026-11-09** by S1f.5.18's T1f.5.18.3 (its stage file linked as the index cell links it): both `g14` programs are refused at load, one line each, and no corpus program that answers is (W59's two
   rows)."*; Q-M1f.3's index cell gains *"; W33 **fixed 2026-11-09** by S1f.5.18 (T1f.5.18.2)"*, and its section ends
   with *"W33 **fixed 2026-11-09** by S1f.5.18's T1f.5.18.2: an activator binds a parameter to a nested fact as to a
   name."*
3. **The register's counts**: S1f.5.18 files two rows (W59) and fixes four (W33, W58 and W59's two): +4 fixed, −2 open,
   +2 rows. With S1f.5.7's and S1f.5.10's, lane 2 adds +22 fixed, −4 open and +18 rows to `dc4c0bdc`'s register (126
   fixed, 22 open, 1 not wrong); on the lane `grep '^status' corpus/wrong_answers.toml | sort | uniq -c` reads 148 fixed,
   18 open, 1 not wrong (167 rows).
4. **AGENTS.md**, after S1f.5.10's sentence: *"**S1f.5.18 ran to its end the same day** (lane 2): an activator binds a
   parameter to a nested fact as to a name (W33, the user's ruling of 2026-11-08 on S1f.5.5's census), so a value
   parameter matches and concludes the fact, and the last path to `FireError::UnboundVar`, a parameter read only by a
   conclusion (W58), is gone with the variant and its `KeyError` line (Q-M1f.37); `Plan::activator_args` is every
   argument, as values (Q-M1f.43); and the loader refuses a program in which a rule could derive an activator binding a
   relation parameter to a fact, whether or not a run derives it (Q-M1f.1 (b), `ein_ir::from_ir`'s
   `refuse_derived_relation_activators`, `defined_behaviour.md` § 4.9), reading which positions hold relation names off
   the program: a declared relation's argument can hold a fact, and a marker's argument holds what a rule passes on
   from it (Q-M1f.41), so the stdlib's fan-outs load; W59's two rows, the `g14` programs, filed and fixed; Q-M1f.41 to
   Q-M1f.44 defaulted"*, linking this file as
   `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.18_an_activator_binds_a_nested_fact.md`, with
   the merged count.
5. **The milestone README**: nothing; it names no stage's state.
6. **The live read-out page, `docs/kernel/inference/read_out.md`, and its generated golden page** (S1f.5.16's, on
   `master`): no row or number moves. Where the page describes a site's or a refusal's `[activator …]`, it names each
   argument, a nested fact as Ein text, `[activator (box a)]`, since S1f.5.18 (Q-M1f.43); until then a nested fact was
   left out, and an activator whose every argument was one printed no `[activator …]` at all. No corpus read-out moves:
   the one corpus line that moved is *forbid*'s refusal on `examples/ein-bugs/activator-all-nested.ein`, on stderr. No
   summary field's name or schema moves (`ein-summary/2`); a `monotonicity` site's `activator` array holds the fact's
   text the same way. A program the check refuses prints the loader's line, `defined_behaviour.md` § 4.9's.
7. **The register's schema**: no bump. Every row S1f.5.18 files uses schema /16's fields; /18 stays unused.
8. **The id blocks**: row W59 used, W60 to W69 unused; questions Q-M1f.41 to Q-M1f.44 used, Q-M1f.45 to Q-M1f.49 unused.
9. **Code the merge must reconcile**: `Plan::activator_args`, `firing::Firing::activator`, `obligations::Owed::activator`,
   `oblgen::InstanceKey`, `forbid::Refusal::activator`, `monotone::Site::activator` and `AbsentRead::activator`,
   `naf_deps::NafDep::activator` and `detector`'s root plans are `Box<[Value]>` since T1f.5.18.2, where they were
   `Box<[Symbol]>`; code on master that reads one as symbols (S1f.5.12's, if it touches cores' labels) prints a value
   with `Terms::display` and orders it with `Terms::cmp_semantic`. `FireError::UnboundVar` is gone, so a match on it does
   not compile.
10. **Files both lanes may have touched**, which the merge should re-take rather than splice: `corpus_exits.txt` and
    `corpus_shapes.md5` (`EIN_BLESS=1`; S1f.5.18's cells are W59's witnesses', the two fixtures', W58's, and the digests
    of `activator-all-nested`, the `binding-key` family and W33's and W58's witnesses); the register's header paragraphs
    (lane 2 appends three after S1f.5.10's); `corpus/corpus.toml` (two entries after W58's, two load-negative entries
    before `unbound_head`, and the notes of W33's, W58's and the `binding-key` family's); `examples/README.md` (the
    witness count, 161 on the lane); `examples/broken/load/README.md` (a section); `open_questions.md`'s index table
    (lane 2's rows follow Q-M1f.40); `defined_behaviour.md` (§ 3.2, § 4's table and its class paragraph, § 4.6, § 4.9,
    § 4.13); `01_grammar.md`'s parameter list; `alive_invariant.rs`' list and `detector.rs`' flagged count (master's
    banked programs move both); `cli_semantics.rs`, `program_read.rs`, `stdlib_semantics.rs`, `compile_semantics.rs`,
    `rule_semantics.rs`, `refutation_under_absent.rs` and `fuzz_properties.rs`; and M1e2's inventory and S1f.5.5's census
    (G-14, W33).
