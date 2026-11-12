# S1f.6.12 — Malformed numbers, and the names the theory occupies

**Phase:** [P1f.6](README.md)
**Status:** **T1 done 2026-11-11, stopped for the user:** [`malformed_numbers.md`](malformed_numbers.md) is what it
found, on the shipped `std.arith`, with T2's probes: the table, a recommendation a row, and six questions
([§ T1 record](#t1-record-2026-11-11)). Nothing is built. Planned 2026-11-11, on the user's ruling of that day.
**The build is ruled and planned (2026-11-12):** the user's replies to the page's questions are in
[§ The build, as ruled](#the-build-as-ruled-2026-11-12), which replaces *T3 onward* below. Not started.
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

**Status:** planned 2026-11-12, not started. It replaces [§ T3 onward](#t3-onward-the-build-after-the-users-ruling-on-t1):
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
| [5](questions.md#q5), a name that no `let` states | *"Why absent? Why not (false)? It's calculator semantic - used but not declared variable (constant in our case). Or that means it can be deduced, and then after declaration appears finished calculation?"* | a question back; [round 2, item 5](questions_round2.md#q5) explains and takes **(b)** meanwhile: `(undeclared NAME REQUEST)` through one `absent` over `let`, and `(false)` from it. **Replied in round 2: *"(b)"*** (as it stood in the working copy when this plan was written (2026-11-12, not yet committed); the stage re-reads it before it starts) |
| [6](questions.md#q6), should a request that no rule completes say so? | *"(a)"* | no general mechanism; each module's page says *a flag means malformed; no flag does not mean well-formed*, and lists what stays silent |
| [7](questions.md#q7), may a module reserve its names? | *"(a), btw, `2 + 2 = 5` is interesting question - what happens to the program that states false facts? If it is not solved already by-product in existing theories - I expect it belongs to either p1f.8 or M1g"* | not in this phase: the module's page lists the names and says only requests and `let` are a program's to state. The false-facts question is [P1f.8's S1f.8.10](../p1f.8_school_algebra_and_geometry/s1f.8.10_a_program_that_states_false_facts.md) |
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
