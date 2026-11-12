# S1f.6.12 — Malformed numbers, and the names the theory occupies

**Phase:** [P1f.6](README.md)
**Status:** **T1 done 2026-11-11, stopped for the user:** [`malformed_numbers.md`](malformed_numbers.md) is what it
found, on the shipped `std.arith`, with T2's probes: the table, a recommendation a row, and six questions
([§ T1 record](#t1-record-2026-11-11)). Nothing is built. Planned 2026-11-11, on the user's ruling of that day.
**The build is ruled and planned (2026-11-12):** the user's replies to the page's questions are in
[§ The build, as ruled](#the-build-as-ruled-2026-11-12), which replaces *T3 onward* below.
**Built 2026-11-13:** [§ Build record](#build-record-2026-11-13). Q-M1f.205 to .209 defaulted, the user's to
revise; Q-M1f.210 open (`std.int` and `std.rat`); E-36 to E-38 found and shown.
**Depends on:** the investigation needs only the prototype (`build/s1f.6/prototype/r3/k.ein`); the build needs
[S1f.6.6](s1f.6.6_division.md), so every operation flags.

## The ruling

The user, 2026-11-11: *"with theory loaded names `n`, `d`, `z`, `sum-of` etc became occupated and must not be
used with other meaning, so if theory can detect malformed numbers - do it, what cons/objections?"*

Today a request the theory cannot read derives nothing: exit 0 and no word
([design.md, round 1's finding 3 and round 2 § A](design.md)). The ruled direction: the theory detects what it
can, a malformed request derives `(malformed <request>)`, and a second rule derives `(false)` from the flag, as
division by zero does.

## T1, the investigation: what can be detected, and what it costs

The answer to *what cons/objections* is taken by experiment, on the prototype, before anything is planned
further. A detection must be monotone: no `absent`, no `not`.

| the malformation | the candidate detection | to find out |
|---|---|---|
| a name that is no digit where a digit goes, `(n z x)` | ten `neq` guards on the digit's slot, `(neq ?a 0)` … `(neq ?a 9)`; `neq` is computed and monotone | does it load and fire only on a non-digit; what a guard that ranges a variable over the program's names costs when the slot is bound; whether one table of the ten digits with a guard is cheaper |
| a nested fact where a digit goes, `(n z (n z 1))` | a pattern, `(n ?p (n ?q ?a))`, and one for each other head a value can have | whether *any fact that is no digit* can be said without listing heads |
| a number that does not end in `z`, `(n 1 2)`, `(n end 1)` | the innermost prefix is a name that is not `z`: a `neq` guard against `z`, and against being a fact | the same questions |
| a name where a number goes, `(add foo z)` | as the row above, at the operand | whether it collides with named constants, `(let foo N)`: a name is a number exactly when a `let` binds it, and *no `let` binds it* is a negation |
| a nested fact of another relation where a number goes, `(add (pair 1 2) z)` | none known without listing every other relation | stated as not detectable, or found otherwise |
| `n` with the wrong count of arguments, `(n 1)`, `(n z 1 2)` | none in the theory: no pattern of arity two matches it. It is E-2 of [`findings.md`](findings.md), and the fix is the loader holding a nested fact to its relation's declared arity | what the loader does today with a top-level fact of the wrong arity; what a check on nested facts would refuse in the corpus (a census); whether it is refused at load, which is better than a flag |
| a request with the wrong count of operands, `(add X)` | the same loader check | the same |

For each row T1 reports: detected or not, by which rule, the exact output, and **what the detection costs a
well-formed request** in firings and in the measure of S1f.6.3's test, since a check that walks every operand
before every operation doubles the walks. It also says where the check runs: on the way through the circuit,
where a walk that meets a non-digit flags and costs nothing extra, or as a pass of its own.

The objections T1 is expected to weigh, each to be confirmed or dropped by the experiment:
- **Cost,** if detection is a separate pass.
- **Completeness.** Some malformations cannot be detected without negation; a program then cannot read the
  absence of a flag as *well-formed*.
- **`(false)` for a typing mistake** empties R for the whole program, as ruled for division by zero. That is the
  ruled meaning; T1 only shows it on a witness.
- **Rules that fire on other people's facts.** A detection rule whose pattern is loose can flag a fact of the
  program's own that happens to use `n`. This is the occupied-names question below.

T1 ends with the list, a recommendation per row, and **stops for the user**.

## T2, the names the theory occupies

The user's point: once the theory is imported, `n`, `z`, `sum-of` and the rest mean what the theory says. T2
establishes what a clash does today, by probes, and what should be said:

- a program that declares its own `(relation n …)` beside a flat import of the theory (the loader's
  duplicate-name guard, or two declarations merged);
- a program that states facts of `n` with another meaning and never declares it;
- a program that uses `z` as an unrelated name, in a fact of its own, and in an operand's place;
- a program that declares a relation named as one of the theory's internal ones (`done`, `b`, a tag);
- the same four under a qualified import, where the theory's names carry a prefix and nothing clashes.

Then the proposal: the module's header lists every name it occupies, split into its interface and its internals;
the internals get names unlikely to clash (a common prefix is the default); and where the loader can refuse a
clash it already refuses, the page says so. Whether a module may **reserve** names, so that a clash is refused
at load, is a language question the stage puts to the user with the probes and does not decide.

## T3 onward, the build, after the user's ruling on T1

1. The detection rules T1's ruling keeps, in `std.arith`, under the rule of the theory.
2. `(malformed <request>)`, and the rule from it to `(false)`.
3. The loader's arity check on nested facts, if ruled: an engine task with its register rows filed `open` first
   (a nested fact of the wrong arity answered by silence is the witness), its census, its sweep and its pages,
   as S1f.6.11's.
4. One program per rule under `tests/stdlib/arith/`, a harness test of malformed requests, and the search-free
   test on well-formed ones unchanged.
5. The same for `std.num9`, where a digit slot and the fixed count of digits are the cases.

## Acceptance

- T1's table, every row with its output, and the user's ruling recorded.
- Every malformation ruled detectable derives the flag and then `(false)`; every one that is not is named on the
  module's page as answered by silence.
- A well-formed request costs what T1 said it would.
- E-2 is fixed by this stage or ruled.

## T1 record, 2026-11-11

Taken on the shipped `std.arith` (89 rules), not on the prototype `r3/k.ein` the plan named: S1f.6.6 was done, so
the experiment could be made on what a detection would be built into. The page is
[`malformed_numbers.md`](malformed_numbers.md); the probes, three prototype theories (`stdlib-b1`, `-b2`, `-a`,
written by `proto.py` from the shipped `stdlib/`) and the scripts are under `build/s1f.6.12/`.

**What it found, in the table's order:**

| the malformation | detected with no `absent` and no `not`? | recommended |
|---|---|---|
| a name that is no digit where a digit goes | yes: ten `neq` guards on the digit's slot, which hold of any other name and of any fact | build |
| a fact where a digit goes | yes, by the same rule | build |
| a number that does not end in `z` | where it ends in a digit or a named constant, by a lookup; where it ends in another name, only by a guard over the program's names, whose cost grows with the program | build the first two; the third stays silent |
| a digit where a number goes, `(add 7 X)` | yes, by a lookup | build |
| a name where a number goes that no `let` states | **no**: it is a negation | the user's: silence, or one `absent` over `let` (Q2) |
| a fact of another relation where a number goes | **no**: no rule can say *not of this shape* | stays silent |
| `n`, or a request, with the wrong count of arguments | one count a pattern, not in general | **the loader** (Q1): a census finds no program that loads today refused |
| a number written with high zeros | not malformed | — |
| *found by T2:* a name stated twice with two numbers; `z` or a digit as a constant's name | yes, by a join on the name | build: today each gives **two answers to one request** |

**What it costs:** twenty firings a request for the recommended set, one a digit of each operand, 0.03 ms; a batch
still costs in proportion to its count. The guards inside a walk are free; the guard over the program's names is
what does not scale (1 000 sums in 761 ms beside 4 000 named constants, where 37 ms shipped). The `candidates`
counter sees none of it (E-27 of [`findings.md`](findings.md)).

**The objections** that survived: completeness (no flag does not mean well-formed, and the page would have to say
so); twenty-two rules more in a module of 89; `neq` in a theory that writes no guard today. The one that was dropped:
a detection firing on other people's facts, since the check is asked for by an operand.

**T2's probes:** a second declaration of any name of the theory is refused at load already, with the name in the
message; facts of the theory's relations are not, and a written `(done (sum 2 2) 5)` is an answer nobody computed;
`z` and the digits are names and are shared harmlessly; a qualified import prefixes the relations and not `z`.

**E-2** is extended by it: a *top-level* fact with the wrong count of arguments loads too.

**The questions for the user** are the page's § 8: the loader's arity check (Q1), a name no `let` states (Q2),
whether to build the recommended set (Q3), whether a module may reserve its names (Q4), a prefix for the internal
names (Q5), and `std.num9` (Q6). T3 onward waits for them.

## The build, as ruled (2026-11-12)

**Status:** built 2026-11-13 ([§ Build record](#build-record-2026-11-13)). It replaces [§ T3 onward](#t3-onward-the-build-after-the-users-ruling-on-t1):
item 3 there, the loader's arity check, is a stage of its own, [S1f.6.23](s1f.6.23_the_loaders_arity_check.md),
and item 5, `std.num9`, is ruled out.
**Depends on:** [S1f.6.6](s1f.6.6_division.md). It edits `stdlib/arith.ein`, so it runs one at a time with the
other stages that do: after [S1f.6.22](s1f.6.22_integer_division.md), which does not, and before
[S1f.6.24](s1f.6.24_division_by_zero_at_the_call.md), which does. Independent of the stdlib layout.

### The rulings

The user's replies of 2026-11-12, each to the question of [`questions.md`](questions.md) named:

| question | the reply, as typed | what it makes of the build |
|---|---|---|
| [4](questions.md#q4), build the recommended detection rules? | *"(a)"* | the set the page calls *b1* is built, *as a checking pass of its own at the operand, the cheaper placement measured by the build before it is chosen* |
| [5](questions.md#q5), a name that no `let` states | *"Why absent? Why not (false)? It's calculator semantic - used but not declared variable (constant in our case). Or that means it can be deduced, and then after declaration appears finished calculation?"* | a question back; [round 2, item 5](questions_round2.md#q5) explains and takes **(b)** meanwhile: `(undeclared NAME REQUEST)` through one `absent` over `let`, and `(false)` from it. **Replied in round 2: *"(b)"*** (committed as `dc88520b`) |
| [6](questions.md#q6), should a request that no rule completes say so? | *"(a)"* | no general mechanism; each module's page says *a flag means malformed; no flag does not mean well-formed*, and lists what stays silent |
| [7](questions.md#q7), may a module reserve its names? | *"(a), btw, `2 + 2 = 5` is interesting question - what happens to the program that states false facts? If it is not solved already by-product in existing theories - I expect it belongs to either p1f.8 or M1g"* | not in this phase: the module's page lists the names and says only requests and `let` are a program's to state. The false-facts question is [P1g.10's S1g.10.6](../../m1g_logic/p1g.10_school_algebra/s1g.10.6_a_program_that_states_false_facts.md) |
| [9](questions.md#q9), does `std.num9` get the same flag? | *"(b), I don't expect we will use num9 further, it is recorded as an example of alternative numbers representation"* | `std.num9` stays silent on a non-digit; its generator is not touched |

[Question 3](questions.md#q3), the loader's arity check, is S1f.6.23. [Question 8](questions.md#q8), the
internal names, is [S1f.6.30](s1f.6.30_name_clashes_between_imported_theories.md).

### What it builds

**Set b1** (`questions.md` 4): a digit's place holding anything but a digit; a number that ends in a digit or
in a named constant where `z` goes, `(n 1 2)`; a digit where a number goes, `(add 7 X)`; a name stated twice
with two numbers; `z` or a digit used as a constant's name. The last two give **two answers to one request**
today. All are monotone: ten `neq` guards or a table lookup, no `absent`, no `not`. Twenty-two rules and three
relations more in a module of 89. A detected request derives `(malformed REQUEST)`, the request as written, and
a second rule derives `(false)` from the flag, as a division by zero does.

**What it costs,** as measured on the prototype (`python3 build/s1f.6.12/cost.py`, 2026-11-11; `questions.md` 4),
4 000 requests on nine-digit operands:

| | shipped: firings a request, ms | with b1 |
|---|---|---|
| sum | 27.0, 138.9 | 47.0, 260.8 |
| comparison | 16.0, 104.4 | 36.0, 222.8 |
| product | 486.2, 3 828.7 | 506.2, 3 984.7 |

Twenty firings a request whatever the operation; a batch still costs in proportion to its count.

**An undeclared name** (round 2, item 5, option (b), the user's reply there): `(add foo X)` where nothing says `(let foo …)`
derives `(undeclared foo (add foo X))` and `(false)` from it. The premise is `absent` over `let`, which a
program writes and no rule derives, and which the theory closes. The theory's claim changes from *no `absent`,
one stratum* to **one `absent`, over a relation no rule derives**. Round 2 records that the check cost *some
hundreds of comparisons a request* on the prototype, for asking *is this a name*; the build re-measures it and
reports the figure.

**What stays silent,** and is named on the page (reply 6 (a)): a number that ends in a name that is neither a
digit nor a constant; a fact of another relation where a number goes; a natural number where `std.int` expects
an integer, and an integer where `std.rat` expects a rational; every malformation of `std.num9`. A wrong
count of arguments is refused at load once S1f.6.23 has landed, and until then is silent too.

### Tasks

1. **T1f.6.12.3 — the undeclared name shown first, on the shipped theory and the release binary,** before any
   rule is moved into the stdlib (a design change shows real computed examples first). One program with an
   undeclared operand and its well-formed twin, under `ein solve`, `ein saturate`, `ein test` and
   `ein oracle`, with what each prints. It has to establish three things, and **the stage stops for the user
   if any fails**:
   - the loader's *forbid* does not refuse the theory (round 2 says it does not, since `let` is closed and
     derived by no rule);
   - **the static check does not flag every program that imports the calculator.** One `absent` in the
     theory may make each such program read as non-monotone to the check, which decides the route a run
     takes and what `--minimal` and `--maximal` certify. If it does, the search-free claim of every
     arithmetic program is at stake, and that is the user's to weigh against the flag;
   - a `let` added to the program answers the request and removes the flag, with nothing stale left: root is
     the only state, so the non-monotone premise has no second state to be stale in.
2. **T1f.6.12.4 — the placement measured:** b1 as a pass of its own at the operand against b1 inside the
   walks, on the prototype's theories (`build/s1f.6.12/`), firings and time a request; the cheaper is built,
   and the table goes into the record.
3. **T1f.6.12.5 — the rules into `stdlib/arith.ein`:** b1, `(malformed REQUEST)`, the rule from it to
   `(false)`, `(undeclared NAME REQUEST)` and its rule to `(false)`; the closing rule's two lists. The rule
   of the theory holds for every new rule (no rule joins two facts on a number); the twice-named constant is
   detected by a join on the *name*, which the rule allows and the text test must be taught to tell apart.
4. **T1f.6.12.6 — the tests restated and extended.** `arith_theory.rs`: the check that the module writes no
   `absent` becomes *exactly one, over `let`*; that it writes no guard becomes *`neq` guards in the checking
   rules alone*; a harness of malformed requests, each with exactly one flag and no answer; the well-formed
   harness unchanged, every answer still right. `arith_search_free.rs`: a malformed program is held as a
   division by zero is, an empty R decided by root, claiming `(solve :expect (false))`.
5. **T1f.6.12.7 — E-27's second measure.** The `candidates` counter does not see a guard, so
   `arith_batch.rs` would pass on a theory whose guard walks the program's names (row 71 of `questions.md`).
   The batch test gains a second measure, the firings and a count of guard evaluations, and holds the
   detection's cost flat in the batch size by both. It is run by hand
   ([S1f.6.21](s1f.6.21_the_gate_reduced.md)); E-27's row is closed with what was built.
6. **T1f.6.12.8 — one program per rule** under `tests/stdlib/arith/`, each with its `:expect`;
   `stdlib_coverage.rs` holds them; `python3 utils/stdlib_mutants.py` with no unexplained survivor.
7. **T1f.6.12.9 — `std.int` and `std.rat` checked, not changed:** a malformed natural number inside an
   integer or a rational reaches `std.arith`'s walks through their tagged calls; the task records which of
   b1's flags fire there and which do not, on real programs, and the pages say it. No rule of either module
   is added without showing the user that table.
8. **T1f.6.12.10 — the pages.** [`stdlib/README.md`](../../../stdlib/README.md) and the module's header:
   the flags; **what stays silent**; *a flag means malformed; no flag does not mean well-formed*; the names
   the module occupies, split into what a program states (the requests and `let`) and what is the circuit's
   own; the theory's claim about `absent`, restated. `std.num9`'s section: silent on a non-digit, by the
   user's ruling. `AGENTS.md`'s calculator paragraphs; this phase's README (§ The principle says *no
   `absent` and no `not` premise anywhere in the theory*, and § Acceptance holds it by a test);
   [`malformed_numbers.md`](malformed_numbers.md) gets a closing note that points here and is otherwise
   left as the investigation's record.

### Acceptance

- Every malformation of set b1 derives `(malformed REQUEST)` and then `(false)`; an undeclared name derives
  `(undeclared NAME REQUEST)` and then `(false)`; each is one flag a request and no number.
- A well-formed request is answered as before, and costs what T2 measured; the cost is flat in the batch
  size by both of E-27's measures.
- No existing program's `ein solve` moves but by the facts the checking pass adds to a state (the sweep, old
  stdlib against new; every golden expected to move named before blessing).
- The pages list what stays silent, and that only requests and `let` are a program's to state.
- `std.num9` is unchanged.

### What it must not do

- **Not flag `std.num9`** (reply 9 (b)).
- **Not add a second `absent`,** and none over a relation a rule derives.
- **Not rename the theory's internal relations** (question 8 is S1f.6.30's) and **not reserve names** in the
  loader (reply 7 (a)).
- **Not build a general report of unanswered requests** (reply 6 (a)).

### The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Config never changes an answer; a
malformed program is an answer (an empty R) and exits 0. The gate is `./run_tests.sh`, then `./run_tests.sh
--nightly` after the code commit; `utils/ci_container.sh` is not needed.

## Build record, 2026-11-13

Every task of [§ The build, as ruled](#the-build-as-ruled-2026-11-12), in its order. The scripts, the
prototypes and the logs are under `build/s1f.6.12/build/` (gitignored); each number has its command.

### What the theory flags now

26 rules and 7 relations more in `stdlib/arith.ein`: 114 rules and the closing rule, where 88 (`grep -c
'^(rule ' stdlib/arith.ein`, less one, 2026-11-13). Two existing rules, `operand-number` and `operand-name`,
conclude one fact more.

| what a program wrote | the flag | the rule that finds it |
|---|---|---|
| a digit's place that holds a name that is no digit, or a fact | `(malformed REQUEST)` | `check-digit`, ten `neq` guards |
| a number that ends in a digit where `z` goes | `(malformed REQUEST)` | `check-prefix-digit`, `check-prefix-zero` |
| a number that ends in a named constant; a name that stands for a name | `(malformed REQUEST)` | `check-prefix-named` |
| a digit where a number goes | `(malformed REQUEST)` | `operand-digit`, `operand-digit-zero` |
| a name no `let` states | `(undeclared NAME REQUEST)` | `operand-undeclared`, the one `absent` |
| a name stated twice with two numbers | `(malformed (let NAME NUMBER))`, each `let` | `let-twice` |
| `z` or a digit as a constant's name | `(malformed (let NAME NUMBER))` | `let-zero`, `let-digit`, `let-digit-zero` |

**How a flag is derived.** `operand-number` and `operand-name` hand each number they find to `(check TAG
NUMBER)`; `check-step` goes one place down a firing. A fault is `(bad-operand TAG FAULT)`, FAULT being
`(not-a-number THING)`, what was found where a digit or a number goes, or `(no-let NAME)`. Ten rules, one a
tag and alike but for the operation, `bad-add-first` to `bad-div-second`, conclude `(bad-request REQUEST
FAULT)`; `bad-request-malformed` and `bad-request-undeclared` conclude the flag; `malformed-is-false` and
`undeclared-is-false` conclude `(false)`. The relations: the flags `malformed` and `undeclared`, and the
circuit's `check`, `bad-operand`, `bad-request`, `not-a-number` and `no-let`.

**What stays silent** (reply 6 (a)), on `stdlib/README.md` and in the module's header, and held by
`arith_theory.rs`'s `what_stays_silent`: a number that ends in a name that is neither a digit nor a
constant; a fact of another relation where a number goes, as an operand or inside a number; a name that
stands for either; everything of an operand in `std.int` and `std.rat` but a `let` that cannot be read
(T1f.6.12.9, below); every malformation of `std.num9`; the binary example theory.

### T1f.6.12.3 — the undeclared name shown first, on the shipped theory

`build/s1f.6.12/build/t3/undeclared.ein`: the shipped `std.arith` at `d038579d`, the release binary built
from it, and the rule written in the program, with its two routes and its rule to `(false)`; one request
`(add foo (n z 5))` and one well-formed sum. `declared.ein` is the same with `(let foo (n z 7))`.
2026-11-13, every command exit 0:

| | `undeclared.ein` | `declared.ein` |
|---|---|---|
| `ein solve` | `states (k_s) 0`, *every node is dead — root's own fixpoint derives (false)*, `unsat core (add foo (n z 5))`, `refuted by undeclared-is-false`, `lattice monotone (the static check)`, `universe no candidate`, 1 saturation | `states (k_s) 1`, `∅ in R`, `foo + (n z 5) = (n (n z 1) 2)` and the other sum, `lattice monotone (the static check)` |
| `ein saturate` | 1 `unknown-operand`, 1 `undeclared`, 1 `false`, each by its rule; the other sum computed | no `undeclared`, no `false` |
| `ein test` | `(solve :expect (false))` and the query's `(false)` held | the claimed sum held |
| `ein oracle` | the same read-out, *the cube: 0 dead nodes with an alive superset — the static verdict holds* | one state |
| `ein solve --minimal`, `--maximal`, `--count` | 0 states | 1 state |
| `ein solve --single` | `Contradiction — R is empty` | `Solution` |

The three things the task had to establish, each held:

1. **The loader's *forbid* does not refuse the theory.** The program loads on every command.
2. **The static check does not flag a program that imports the calculator.** `lattice monotone (the static
   check)` on both, and on the built theory on every searching program of the corpus too: the sweep below
   moves no `lattice` row, `examples/beyond_arith/equation_two_unknowns.ein`, 1 466 saturations, among them.
   The summary's `monotonicity` block reads `monotone: true`, `absent_reads: {static: 1, dynamic_monotone:
   0, dynamic_non_monotone: 0}`. **What does move: `ein saturate --dump`'s `;; Strata` line reads
   *stratified, 2 strata* where it read 1**, for every program that holds `std.arith`'s rules.
3. **A `let` answers the request and removes the flag, with nothing stale.** `declared.ein` above; in the
   built theory, `tests/stdlib/arith/29_undeclared_name.ein` and `30_declared_name.ein`, and
   `an_undeclared_name_is_flagged_and_a_let_answers_it`.

**Found there, and it shapes the rule** (Q-M1f.208, E-36, E-37): `(absent (let ?x ?w))` holds of a number
too, so the rule must say that the operand is a *name*, and the one way is two guards on a variable nothing
else binds, `(neq ?v z) (eq ?v ?x)`, in that order. Written the other way round the pair holds of facts as
well (`build/s1f.6.12/neq/g1.ein` against `build/s1f.6.12/build/g2.ein`). The engine does what
`01_grammar.md` says of a guard's free variable, *bound where it is written*, so this is a finding and not
a register row.

### T1f.6.12.4 — the placement measured

Two prototypes of the whole set, written from the shipped stdlib by `build/s1f.6.12/build/proto2.py`: **the
pass**, 26 rules, and **mixed**, 58 rules: a sum, a difference and a comparison checked by guards on their
own walks, a product and a quotient by the pass. Both flag every probe of the investigation alike
(`build/s1f.6.12/today/`). Firings a request and the saturator's milliseconds, batches of nine-digit
operands (a divisor of four), best of five (`utils/bench_env.sh python3 build/s1f.6.12/build/cost2.py`,
2026-11-13, one P-core of an i9-14900HX, `powersave`):

| operation | requests | shipped: firings a request | ms | pass: firings a request | ms | mixed: firings a request | ms |
|---|---:|---:|---:|---:|---:|---:|---:|
| add | 250 | 27.0 | 8.7 | 45.0 | 15.7 | 27.0 | 11.8 |
| add | 1000 | 27.0 | 33.2 | 45.0 | 62.9 | 27.0 | 45.8 |
| add | 4000 | 27.0 | 146.1 | 45.0 | 292.9 | 27.0 | 199.0 |
| sub | 250 | 37.0 | 12.8 | 55.0 | 19.7 | 37.0 | 17.5 |
| sub | 1000 | 37.0 | 51.1 | 55.0 | 79.6 | 37.0 | 70.4 |
| sub | 4000 | 37.0 | 242.8 | 55.0 | 378.7 | 37.0 | 318.1 |
| cmp | 250 | 16.0 | 6.3 | 34.0 | 13.8 | 16.0 | 9.6 |
| cmp | 1000 | 16.0 | 25.6 | 34.0 | 55.7 | 16.0 | 37.5 |
| cmp | 4000 | 16.0 | 107.5 | 34.0 | 261.2 | 16.0 | 158.1 |
| mul | 250 | 486.3 | 195.3 | 504.3 | 205.9 | 506.3 | 230.2 |
| mul | 1000 | 486.0 | 922.6 | 504.0 | 948.4 | 506.0 | 1052.9 |
| mul | 4000 | 486.2 | 4223.8 | 504.2 | 4406.1 | 506.2 | 4912.6 |
| div | 250 | 448.3 | 182.4 | 461.3 | 202.1 | 463.3 | 232.2 |
| div | 1000 | 447.3 | 853.9 | 460.3 | 936.7 | 462.3 | 1071.4 |
| div | 4000 | 447.5 | 3902.3 | 460.5 | 4287.1 | 462.5 | 4866.6 |

**Neither is cheaper for every operation.** Mixed costs no firing for a sum, a difference and a comparison
and about two thirds of the pass's time there, and is dearer than the pass for a product and a quotient,
since every internal walk of those then carries eight more rules. **The pass is built** (Q-M1f.205,
defaulted): half the rules, one mechanism, the request named by ten small rules, no operand left unchecked
(`(mul X z)` reads X in no walk), and no flag under a tag of a module that calls the walks itself. The pass
is 18 firings a request here where the investigation's was 20: the two rules that find a number start the
check themselves.

**The built theory against the one before it**, the same batches (`utils/bench_env.sh python3
build/s1f.6.12/build/cost3.py`, 2026-11-13):

| operation | requests | before: firings a request | ms | built: firings a request | ms |
|---|---:|---:|---:|---:|---:|
| add | 250 | 27.0 | 9.6 | 45.0 | 17.7 |
| add | 1000 | 27.0 | 37.7 | 45.0 | 73.6 |
| add | 4000 | 27.0 | 163.7 | 45.0 | 318.2 |
| sub | 250 | 37.0 | 14.2 | 55.0 | 22.1 |
| sub | 1000 | 37.0 | 60.1 | 55.0 | 92.5 |
| sub | 4000 | 37.0 | 260.9 | 55.0 | 403.1 |
| cmp | 250 | 16.0 | 7.0 | 34.0 | 15.7 |
| cmp | 1000 | 16.0 | 28.0 | 34.0 | 64.5 |
| cmp | 4000 | 16.0 | 117.3 | 34.0 | 278.5 |
| mul | 250 | 486.3 | 214.4 | 504.3 | 226.1 |
| mul | 1000 | 486.0 | 935.0 | 504.0 | 977.7 |
| mul | 4000 | 486.2 | 4208.0 | 504.2 | 4396.9 |
| div | 250 | 448.3 | 193.9 | 461.3 | 214.8 |
| div | 1000 | 447.3 | 859.0 | 460.3 | 952.1 |
| div | 4000 | 447.5 | 3858.5 | 460.5 | 4245.5 |

| 1 000 sums beside | before: ms | built: ms |
|---|---:|---:|
| 0 `let`s | 36.6 | 69.7 |
| 1000 `let`s | 37.2 | 89.1 |
| 4000 `let`s | 41.3 | 140.7 |

| 1 000 sums of named operands, beside | before: ms | built: ms |
|---|---:|---:|
| 100 `let`s | 35.6 | 72.2 |
| 1000 `let`s | 41.0 | 94.0 |
| 4000 `let`s | 45.4 | 147.5 |

At 4 000 requests a sum takes 318 ms where 164, a difference 403 where 261, a comparison 279 where 117, a
product 4.40 s where 4.21 and a quotient 4.25 s where 3.86: twice for the three short operations, a
twentieth to a tenth more for the two long ones. **This is more than the investigation's b1** (261, 326,
223 ms, 3.98 and 3.84 s on its day): the difference is the undeclared name's guard, which b1 did not have.
A batch still costs in proportion to its count. **The last two tables are the cost that grows with the
program's names** (E-36): 1 000 sums beside 4 000 named constants take 140.7 ms where 41.3.

**The guards' share, and the undeclared name's.** By the guards' own count, 992 values a request for two
nine-digit operands: 180 are `check-digit`'s, ten a digit, and the rest `operand-undeclared`'s, two a name
of the program for each operand. The second grows with the program's names (E-36).

### T1f.6.12.5 — the rules in `stdlib/arith.ein`

As above. The closing rule closes the seven relations and names the 26 rules
(`the_closing_rule_names_every_rule_and_closes_every_relation`). The rule of the theory holds: every new
rule has one premise that holds a number; its others are guards, a probe of one argument the first premise
binds (`nz`, `named`), or, in `let-twice`, a second `let` of the same name. `python3
utils/stdlib_manifest.py --write` re-banked the manifest.

`examples/binary_arithmetic/gen_binary.py` was run: it carried the decimal check into the binary theory by
name, ten decimal digits and a table that theory lacks. **It now leaves the check out** (Q-M1f.209), and
`bin.ein` is byte for byte what it was (`python3 examples/binary_arithmetic/gen_binary.py --check`, exit 0).

### T1f.6.12.6 — the tests restated and extended

`ein-infer/tests/arith_theory.rs`:

- `the_theory_writes_no_negation` is **`the_theory_writes_one_absent_over_let`**: no `not`, no `or`;
  exactly one `absent`, in `operand-undeclared`, over `let`; no rule concludes `let`; `eq` and `neq` in
  `check-digit`, `operand-undeclared` and `let-twice` alone; three rules conclude `(false)`, each from its
  flag and nothing else.
- `no_rule_joins_two_facts_on_a_number` reads a guard as no premise that holds a number (and holds that its
  arguments are no form), a probe as one lookup on a variable the first premise binds, and names the second
  join on a name, `let-twice`, beside `operand-name`.
- **The harness of malformed requests**, four tests. `a_malformed_request_is_flagged_once_and_never_answered`:
  fifteen malformed operands, under each of the five operations, as the first operand, the second and
  both, 45 requests an operation: exactly the 45 flags, no other flag, `(false)`, and the requests answered
  are the three well-formed ones beside them. `an_undeclared_name_is_flagged_and_a_let_answers_it`,
  `a_let_that_cannot_be_read_is_flagged`, `what_stays_silent`.
- `the_rules_from_a_tag_to_its_request_are_alike`: the ten route rules, by their text.
- The well-formed harness is unchanged and passes: every answer right, each request answered once.
- **The circuit's pins moved, by the check's firings and nothing else**: 123078 + 4567 is (31, 21) where
  (21, 21), the depth unmoved; a subtraction (38, 21), a comparison (23, 13), 123 × 456 (78, 72), 127645 ÷
  25 (228, 220), and the rest, each its old size and the digits of the operands that are found.

`ein-cli/tests/arith_search_free.rs`: a dead program is a division by zero, a malformed program or an
undeclared name, by the rule `refuted by` names; a malformed one's core names what the flag names and
otherwise `let`s and table rows, its flag is derived by one of six rules, and the request it names has no
result fact. A live program reads **one `absent` statically where it holds `std.arith`'s rules and none at
a node**, and has two strata there; a program of `std.num9` alone reads none and has one.

`ein-infer/tests/alive_invariant.rs` lists the seventeen new programs' names. A first draft's fault was a
bare name, `not-a-number`, which a rule asserted: that made it a name of every program that imports the
theory, 106 new entries of that list; the fault is a fact since.

### T1f.6.12.7 — E-27's second measure

`ein_core::counters` has a counter more, `predicate_value`: one per value an `eq` or `neq` premise is
decided for, and one per name of the program where a free side ranges over them
(`ein-infer/src/match_.rs`, `guard`). It is compiled out of every shipped build, as the others are.
`arith_batch.rs` holds it flat in the batch beside `candidates`, and under a bound. Run by hand (`cargo
test --manifest-path ein.rs/Cargo.toml -p ein-infer --features counters --test arith_batch -- --nocapture`,
2026-11-13, 5 passed), a request at 250, 1 000 and 4 000 requests:

| operation | firings | candidates | guard values |
|---|---:|---:|---:|
| add | 45.00 | 181.38, 181.38, 181.39 | 991.92, 992.65, 991.93 |
| sub | 55.00 | 362.00 | 991.92, 992.65, 991.93 |
| cmp | 34.00 | 901.00 | 991.92, 992.65, 991.93 |
| mul | 504.84, 504.60, 504.36 | 9 603.87, 9 601.18, 9 598.70 | 991.92, 992.65, 991.93 |
| div | 458.85, 459.59, 459.66 | 6 033.98, 6 030.25, 6 030.94 | 964.86, 964.96, 964.34 |

**`candidates` is what it was before the stage, to the last digit**: the check's probes are lookups with
every argument bound, which offer no candidate. `std.num9`'s, `std.int`'s and `std.rat`'s batches pass
unchanged. `a_number_operand_does_not_walk_the_lets` still holds by `candidates`, and pins the guards'
growth: 500 requests beside 200 `let`s cost exactly `2 × 2 × 500 × 200` guard values more than apart
(496 462, 896 662 and 200). E-27 is closed.

### T1f.6.12.8 — one program per rule

Seventeen programs, `tests/stdlib/arith/22` to `38`, each with a corpus entry: fourteen that fire one fault
through one of the ten routes and claim `(solve :expect (false))`, and three that load the rules where none
fires, `30_declared_name`, `34_let_twice_the_same` and `38_what_stays_silent`, and claim R = {∅} with the
flags' extents empty. `07_only_what_is_asked.ein` lost its request `(add a rebate)`, which is flagged now
and is `29`. `stdlib_coverage.rs`: every rule activated, 367 of 367 (`cat stdlib/*.ein | grep -c '^(rule
'`, 2026-11-13). `ein test tests/stdlib/arith`: 38 files, 72 expectations held.

**The mutation sweep: 437 of 499 killed, 58 of the 70 new mutants** (`python3 utils/stdlib_mutants.py`,
`build/s1f.6.12/build/mutants.log`, 2026-11-13; taken before the fault became a fact, which touches no
mutated premise). **Twelve survive, each explained and banked** in `tests/mutation_survivors.txt`: a dead
program's one claim cannot say which flag killed it. Ten drop one of `operand-undeclared`'s `(neq ?x DIGIT)`
guards, under which a digit as an operand is flagged twice and is dead either way;
`bad-request-undeclared` with its flag's arguments exchanged; and `undeclared-is-false` with its premise's
two variables exchanged, which is the same rule. `arith_theory.rs`'s harness holds all twelve by the flags'
text. The bank was edited by hand from the run's list; `--check` was not re-run after the fault became a
fact, and the nightly runs it.

### T1f.6.12.9 — `std.int` and `std.rat` checked, not changed

26 programs, `build/s1f.6.12/build/t9/` (`ein saturate --dump`, 2026-11-13). No rule of either module was
added; their headers' *one stratum* is restated.

| the operand | `std.int` | `std.rat` |
|---|---|---|
| a malformed number inside it: a digit's place, a number's end, a named constant, a digit as a part | no flag | no flag |
| the same behind a name, `(let a (int (n z x) z))` | no flag | no flag |
| an undeclared name, either operand | `(bad-operand (int-first REQUEST Y) (no-let foo))` in root's state; **no flag, no `(false)`** | the same under `(rat-of …)` |
| a digit as the operand, `(int-add 7 X)` | `(bad-operand TAG (not-a-number 7))`; no flag | the same |
| a natural number for an integer, an integer for a rational | no flag | no flag |
| a name stated twice with two values | `(malformed (let …))` twice, `(false)` | the same |
| `(let z …)`, `(let 3 …)` | `(malformed (let …))`, `(false)` | the same |

So of the set, the `let` rules fire there and nothing else is flagged: the two modules find their own
operands, hand the parts straight to `std.arith`'s walks, and have no rule from their tags to a request.
What would change it is three rules in each, or those and a check of each part: **the user's,
[Q-M1f.210](../open_questions.md#q-m1f210--stdint-and-stdrat-and-the-flags)**, E-38.

### T1f.6.12.10 — the pages

`stdlib/arith.ein`'s header (the flags, what stays silent, the claim about `absent`, the names in three
parts); `stdlib/README.md` (§ `std.arith`, a new § Malformed requests, the names, `std.num9`'s, `std.int`'s
and `std.rat`'s sections, the module table, and § What a program pays re-taken); `stdlib/int.ein`'s and
`rat.ein`'s headers, a sentence each; `tests/README.md`; `examples/README.md`, `examples/arith/add.ein` and
`num9_both_ways.ein` (31 firings where 21), `examples/binary_arithmetic/README.md`; the phase's README (§
The principle, § Acceptance, the stage's row); `malformed_numbers.md`'s closing note; `findings.md` (E-27
closed, E-36 to E-38); `open_questions.md` (Q-M1f.205 to .210); `AGENTS.md`, one standing rule.

### The sweep: no existing program's answer moved but as intended

The release binary and the stdlib of `d038579d` against the tree's, `ein saturate --dump` and `ein solve`
over the 817 `.ein` files `d038579d` tracks (`python3 build/s1f.6.12/build/sweep.py --diff`,
`build/s1f.6.12/build/sweep.txt`, 2026-11-13). 89 files moved, every one a program that holds `std.arith`'s
rules:

- **`ein solve` moved on two.** `examples/beyond_arith/equation_two_unknowns.ein`: its `work` row, *75 423
  facts beyond root* where 60 223, the check's facts in 1 466 saturations; R, the lattice row and every
  query unmoved. `examples/broken/load/import_module_not_found.ein`: the path in its refusal, which is the
  old stdlib's directory.
- **`ein saturate --dump` moved on 87**: the counts, the two closed-relation and rule lists, the `;; Strata`
  line, and the `check` facts with their `check-step` firings. No fact a state held is gone.
- **One program was changed on purpose**: `tests/stdlib/arith/07_only_what_is_asked.ein` as `d038579d` had
  it asks `(add a rebate)`, and is dead under the built theory (the first sweep, `sweep1.txt`); the request
  moved to `29`.

### The goldens re-banked, each named before it was blessed

From the run without `EIN_BLESS` (`cargo test --workspace --no-fail-fast`,
`build/s1f.6.12/build/workspace-nff.log`): four tests failed, two of them goldens.

- `ein-cli/tests/golden/corpus_exits.txt`: **85 new cells**, the seventeen new programs under their five
  runs; no existing cell moved.
- `ein-render/tests/golden/corpus_shapes.md5`: the digests of every program that holds `std.arith`'s rules
  (`examples/arith`, `beyond_arith`, `binary_arithmetic`, `word_problems`, `tests/stdlib/arith`, `int`,
  `rat`, one of `num9`, the two `stdlib/` modules over it and two W89 witnesses, which import it): their
  load, saturate, plan, IR, KB and states renderings, by the 26 rules, the seven relations and the check's
  facts; and the new programs' rows. **Of the read-outs, only
  `equation_two_unknowns.ein`'s five moved**, by its `work` row.
- The other two: `alive_invariant.rs`'s list (above), and `ein-ir`'s embedded-manifest test, which failed
  because the stdlib was edited while that run was going, and passes.

### What was run

- **The gate: green.** `./run_tests.sh` on the tree as committed but for this section's text
  (`build/gate-s1f.6.12.log`, exit 0, 2026-11-13); the link check re-run after it, 0 findings.
- `cargo test -p ein-infer --features counters --test arith_batch`, by hand: 5 passed.
- `python3 utils/stdlib_mutants.py`: 437 of 499, as above. `python3
  examples/binary_arithmetic/gen_binary.py --check`: exit 0.
- `utils/ci_container.sh` was not run: no DOT writer, review bundle or CI install changed.
- The nightly is run on the commit and recorded in the commit after it.

**The register is unmoved**: no wrong answer of the engine was met (E-37 is filed as a finding, with why).
**Engine source moved in one place**, the counter and its three increments in `match_.rs`, compiled out of
every build without `--features counters`.
