# S1f.5.9 — An empty `:match` fires

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.9**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).

**Status:** specified 2026-11-07 on the user's rulings of that day on
[F12](README.md#f12-an-empty-match-never-fires). The user ruled on 2026-11-08 that P1f.5 runs, and on 2026-11-09 *"just
continue with default decisions till the end of the phase"*. **In progress 2026-11-09**, in lane 2's worktree
(`p1f5-lane2`), lane 2's last stage, after S1f.5.7, S1f.5.10 and S1f.5.18 (`ee3da657`): T1f.5.9.1
([its record](#t1f591-record-2026-11-09)), every move listed, W60's four rows and W61 filed, three questions taken by
default (Q-M1f.45 to Q-M1f.47).
**Depends on:** nothing it waits for. [S1f.5.5](s1f.5.5_the_grammar.md)'s grammar refuses `(and)`, and since its swap
`()` parses only as a whole `:match` or `:assert`. [S1f.5.10](s1f.5.10_a_conclusion_variable_must_be_bound.md) landed
first, so a rule with `:match ()` and an unbound conclusion variable is refused at load (W57's
`assert-var-never-fires`) rather than stopping the run at root. [S1f.5.8](s1f.5.8_no_contradiction_no_core.md), on
`master` while this lane runs, spells *no core* `null` and a dead node's empty core `[]`; the first `[]` core in Ein is
this stage's ([§ For the merge](#for-the-merge)).

## The ruling

The agent put three questions on F12. The user, 2026-11-07: *"1 fire; 2 same; 3 reject (and) while it is the
same as () and `and` should have arguments"*.

1. **An empty `:match` fires.** An empty conjunction is true, so a rule with no premise fires once, or once
   per activator fact when it has parameters, and concludes what it asserts.
2. **An hrule's empty `:match` fires the same way**, and proposes its conclusion.
3. **`(and)` stays refused.** It means what `()` means, and `and` takes at least one conjunct, so the empty
   conjunction is written `()` alone.

A fourth question, a conclusion's variable that nothing binds, was filed as F13 and ruled the same day: refused
at load, by [S1f.5.10](s1f.5.10_a_conclusion_variable_must_be_bound.md), which lands first.

## The case

Taken on 2026-11-07 at `916d97ac`, with `ein saturate FILE --dump` and `ein solve FILE`, over
`(relation q Node) (relation p Node) (q a)` or `(p a)`. No corpus program writes an empty `:match` (`grep`
over `examples/`, `tests/`, `stdlib/` and `corpus/`).

| the rule | today | right |
|---|---|---|
| `(rule r () :match () :assert (false))` | loads, never fires: ∅ in R | root is dead, so R is empty. Its core is `[]`, the empty set ([Q-M1e2.27](../../m1e2_correctness/open_questions.md#q-m1e227--what-do--and-null-mean-in-eins-json)), and `refuted by r` |
| `(rule r () :match () :assert (q b))` | loads, never fires: no `(q b)` | root holds `(q b :rule r)` |
| `(rule r (?x) :match () :assert (hit ?x))` with `(r a)` | loads, never fires: no `(hit a)` | root holds `(hit a :rule r)`, one firing per activator |
| `(hrule h () :match () :assert (q b))` with `(solve :hrules (h))` | proposes nothing: *no candidate* | proposes `(q b)`, as `:match (p ?x)` does: one candidate |
| `(rule r () :match (and) :assert (false))` | refused at parse: *unexpected input* | refused, with a line that says `and` takes a conjunct and the empty conjunction is `()` |
| `(rule r () :match (q ?x) :assert ())` over `(q a)` | stores a fact named `#empty` (`@empty` until S1f.5.5's T1f.5.5.8), listed `( :rule r)`, which joins H: R two states, the second `(model (q "#empty") (q a))` | an empty conclusion asserts nothing: R {∅}, one state |

The first four are wrong answers, a shape that reads nothing in silence, as W24's were. [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md) filed them
on 2026-11-08 as W31, `open` and owned by this stage, each witness under `examples/ein-bugs/wrong-answers/`
and a corpus entry, and step 2 fixes them.

**The last row was F12's fifth**, which said *loads, adds nothing* and called it right. S1f.5.5's census
found otherwise ([§ Findings outside the grammar](s1f.5.5_census.md#findings-outside-the-grammar)), the user
ruled on 2026-11-08 that it be filed (*"Q3 - as recommended"*), and S1f.5.0 filed it the same day as **W34**,
`W34/assert-empty`, `open` and owned by this stage: an empty conclusion is F12's to fix, and the paper
grammar keeps `:assert ()` as the empty conclusion (its default 7). Its `--dump-answer` state files do not
load, the stored fact written as a line `()`, so `ein-cli/tests/answer_round_trip.rs` and `answer_dump.rs`
name the witness (`DO_NOT_LOAD`) until step 2 takes it off both lists. Its `#empty` is also a name no fact or
rule writes, which the M1 alive-set invariant's R1 finds, so `ein-infer/tests/alive_invariant.rs` names it
(`KNOWN_R1`) until the same commit empties that list.

## What it changes

- **Rules.** A rule whose `:match` is `()` compiles to a plan with no premise. It fires once at root, or once
  per activator fact, with only its parameters bound.
- **Hrules.** The same, so the hrule rung proposes the conclusion once, or once per activator.
- **An empty conclusion** (W34, added 2026-11-08). `:assert ()` asserts nothing: no fact with the parser's
  synthetic head is stored, and `#empty` is no name of the program.
- **`(and)`.** Refused at load on today's parser, in Ein's spelling, naming the rule and the clause. On
  S1f.5.5's grammar, `and` takes one conjunct or more, and the parse error says so.
- **What it reaches.** A contradiction derived this way has an empty core, the first `[]` core in Ein. A
  derivation with no premise is the shape of W25, which F22 owns: the store does not record one as an
  alternative justification. This stage makes it reachable on a rule written for it, and does not change it.
- **`eq` at another arity** (added 2026-11-09 by the coordinator's brief, T1f.5.9.4). S1f.5.18 found a shape that
  loads and stops and left it unfiled: `(rule r (?R) :match (?R ?x) …)` with `(r eq)` stops at the first firing,
  where the plan is compiled, with M1e S1e.2.1's arity line, `eq` being the relation of equality between two values
  (S1f.5.7). A run that stops is no answer, so it is filed (W60) and, where a load-time check consistent with
  S1f.5.18's can see it, refused at load on every command, one line in Ein's spelling. T1f.5.9.1 found it reaches a
  conclusion and an hrule too, and beside it a written fact `(eq a)`, stored (W61).
- **What does not move.** No corpus program writes any of it, so no corpus answer moves.

## The method

### Task T1f.5.9.1 — the rows, every move listed

It starts from W31's four rows and W34's one, which S1f.5.0 filed `open` on 2026-11-08, each `now` taken at `ec004a48`
(the engine `916d97ac`'s), release and debug alike, every one the case's answer above:

| row | witness | `now`, today |
|---|---|---|
| `W31/empty-match-false` | `empty-match-false.ein` | `solve --minimal`: root's state, ∅ in R |
| `W31/empty-match-fact` | `empty-match-fact.ein` | `solve --minimal`: `(model (q a))`, no `(q b)` |
| `W31/empty-match-activator` | `empty-match-activator.ein` | `solve --minimal`: no `(hit a)` |
| `W31/empty-match-hrule` | `empty-match-hrule.ein` | `solve`: no candidate, R {∅} |
| `W34/assert-empty` | `assert-empty.ein`, the census's `assert_empty.ein` | `solve`: R two states, `(q "#empty")` a candidate (`(q "@empty")` at `ec004a48`; S1f.5.5's T1f.5.5.8 renamed the head and restated the `now`) |

Each re-taken, with the `(and)` refusal's probe. `ein-infer/tests/alive_invariant.rs`' `SEEDED` names `b` on
`empty-match-fact` and `empty-match-hrule`, a name only the conclusion writes; step 2 says whether the fix
moves either line. W34's fix takes `assert-empty.ein` off `DO_NOT_LOAD` in both tests and empties
`alive_invariant.rs`' `KNOWN_R1`. The corpus swept, and
`cargo test --workspace` run on a prototype. Every move must be one of the case's rows. **Stop and show the
user the programs**, answers computed.

### Task T1f.5.9.2 — the empty conjunction fires

The compiler and the hrule rung, and the case's rows fixed. `ein-infer/tests/explore.rs` and the oracle
agree on each witness. `ein-infer/tests/corpus_cores.rs` holds the dead witness's core, `[]`, and its rule.

### Task T1f.5.9.3 — the refusal and the pages

The `(and)` line. `01_grammar.md`'s rule section: an empty `:match` is the empty conjunction, true, written
`()`, and `and` takes a conjunct. `02_patterns.md` likewise. The inventory's F12 says what fixed it.

### Task T1f.5.9.4 — `eq` at another arity

Added 2026-11-09 by the coordinator's brief. W60's rows, filed by T1f.5.9.1, fixed by a load-time check consistent with
S1f.5.18's: a relation parameter that can hold `eq` and heads a premise or a conclusion whose arity is not 2 is refused,
one line each, on every command (Q-M1f.46). W61's row fixed beside Q-M1f.33's check of a written `eq` fact
(Q-M1f.47). `program_read.rs`' test, which S1f.5.18 built on the shape as an error past the load, keeps its purpose
with another such error. `defined_behaviour.md` and `01_grammar.md` say so.

### Task T1f.5.9.5 — the acceptance re-taken

Each item, and what holds it; and what the merge writes ([§ For the merge](#for-the-merge)), with lane 2's total.

## Acceptance

- A rule or hrule whose `:match` is `()` fires once, or once per activator fact, on every command.
- `(rule r () :match () :assert (false))` makes root dead, with the core `[]` and `refuted by r`.
- `(and)` is refused at load with a line naming the rule, and `()` is the one spelling of the empty
  conjunction.
- `:assert ()` asserts nothing: W34's witness answers R {∅}, and its state files load.
- The case's rows are `fixed`, and no corpus answer moves.
- A relation parameter that can hold `eq` and heads a form of another arity is refused at load on every command, one
  line in Ein's spelling, and W60's and W61's rows are `fixed` (T1f.5.9.4, added 2026-11-09).
- `./run_tests.sh` is green.

## T1f.5.9.1 record (2026-11-09)

**Run on the user's instruction of 2026-11-09**, *"just continue with default decisions till the end of the phase"*:
every decision a reviewer would call one is taken by default and is a row of [`open_questions.md`](../open_questions.md),
Q-M1f.45 to Q-M1f.47. The stage ran in lane 2's worktree, on `p1f5-lane2` at `ee3da657`, after S1f.5.7, S1f.5.10 and
S1f.5.18, so `eq` and `neq` are relations, a conclusion variable nothing binds is refused at load, and an activator
binds a parameter to a nested fact.

**The case, re-taken** on `ee3da657`'s release and debug builds alike (`build/s1f.5.9/case/`, `ein saturate --dump`,
`ein solve`, `solve --minimal`, `ein oracle`): every row of the case stands as the stage file has it, and every `now`
S1f.5.0 recorded holds (`wrong_answers.rs`, release, on `ee3da657`):

| row | `ee3da657` |
|---|---|
| `W31/empty-match-false` | `r` never fires: root alive, R two states, ∅ and `{(p a)}`, exit 0 |
| `W31/empty-match-fact` | root holds `(q a)` alone; `(q b)` a candidate, R eight states |
| `W31/empty-match-activator` | root holds no `hit`; `(hit a)` a candidate, R four states |
| `W31/empty-match-hrule` | *no candidate: R is {∅} or empty, decided by root alone*, R {∅} |
| `W34/assert-empty` | root stores `( :rule r)`, `(q "#empty")` a candidate, R two states, the query's classes two |
| `(rule r () :match (and) :assert (false))` | `file:3:23: unexpected `)``, repairs *Delete `)`, Delete `:assert`*, then *unexpected end of input*, exit 1 on every command (`ein test`: 2); no word of the empty conjunction |

`(or)` in the same place is refused the same way, and so is `(and)` in an `:assert`, a query's `:goal`, an hrule's
`:match` and a macro's template (`build/s1f.5.9/case/and-*.ein`, `or-empty.ein`). An obligation whose `:match` is `()`
is refused at load already (*`(open ?R)` needs an `(absent …)` in :match*), so F12 does not reach one.

**The programs, answers computed** (the stage file's *show the user the programs*, taken by default: shown here), on
the prototype below:

| program | `ee3da657` | the prototype |
|---|---|---|
| `empty-match-false.ein`: `(q a)`, `(rule r () :match () :assert (false))` | R two states | root dead, R empty; root's core `[]`, `core_status` `within-budget`, `refuted by` `r` (`--dump-answer`'s `nogoods.json`) |
| `empty-match-fact.ein`: `(q a)`, `(rule r () :match () :assert (q b))` | R eight states, `(q b)` a candidate | root holds `(q b :rule r)`; H `(p a)`, `(p b)`, R their cube, four states, the minimal line root's |
| `empty-match-activator.ein`: `(rule r (?x) :match () :assert (hit ?x))`, `(r a)` | R four states, `(hit a)` a candidate | root holds `(hit a :rule r)`; H `(p a)`, R two states |
| `empty-match-hrule.ein`: `(hrule h () :match () :assert (q b))`, `(solve :hrules (h))` | H empty, R {∅} | H `(q b)`, R ∅ and `{(q b)}` |
| `assert-empty.ein`: `(rule r () :match (q ?x) :assert ())` | R two states, `(q "#empty")` | root holds `(q a)` alone, H empty, R {∅}, one class |
| `(rule r (?x) :match () :assert (q ?x))` as an hrule, `(solve :hrules (h (b) (c)))` | H empty, R {∅} | H `(q b)`, `(q c)`, R their cube, once per activator |
| `(rule r () :match (and) :assert (false))` | the parse error above | the same error, with a note: *rule 'r''s :match writes (and), the empty conjunction, whose one spelling is () — and takes one premise or more* |

**The prototype** (`build/s1f.5.9/prototype.patch`), three parts:

- `ein_infer::compile`: a `:match` that is `()` compiles to the one empty disjunct the compiler already built for a
  rule with no `:match`, which the matcher takes as one vacuous match, so the plan fires once at root, or once per
  activator, its parameters bound; an `:assert` that is `()` compiles to no template, so an application derives
  nothing and is no firing, as one whose every conclusion holds. Until then `()` was a premise, and a conclusion, over
  the parser's synthetic `#empty`.
- `ein_ir::grammar`'s notes: where a parse error meets the `)` of `(and)` or `(or)`, a note naming the rule or form
  and the clause (Q-M1f.45).
- `ein_ir::from_ir`: the `eq`-arity check below (Q-M1f.46).

**Every program of the five roots, through both builds** (`build/s1f.5.9/xcheck/run_all.sh t1`, S1f.5.18's cross-check:
`solve --json-summary -H --dump-answer --events`, `solve --minimal`, `saturate --dump`, `test` and `oracle`, the exit,
stdout and stderr of each, the summary, the events and every file the dump wrote compared, timings masked; 542 files,
226 s; `zebra2-minus-15.ein` and its `-obligations` twin re-run alone, the first compared by
`build/s1f.5.9/xcheck/compare_lean.py`, which hashes each file as it reads it, its 8.4 GB of output too much for the
comparator to hold). **536 are the same and six move**, each one of the case's rows:

| program | what moves |
|---|---|
| `wrong-answers/empty-match-false.ein` | root dead, R empty, root's core `[]` and `refuted by r` (W31) |
| `wrong-answers/empty-match-fact.ein` | root holds `(q b :rule r)`, R four states (W31) |
| `wrong-answers/empty-match-activator.ein` | root holds `(hit a :rule r)`, R two states (W31) |
| `wrong-answers/empty-match-hrule.ein` | H `(q b)`, R two states; `saturate` unmoved, an hrule playing no part in a saturation (W31) |
| `wrong-answers/assert-empty.ein` | root holds `(q a)` alone, R {∅}, one class, the state file loads (W34) |
| `plans/…/s1f.5.5_parser_probe/programs/refused/rules.ein` | stderr only: its `(rule r1 () :match (and) …)`'s parse error gains the note (Q-M1f.45) |

No other corpus program, stdlib rule, fuzz finding or plan probe writes `()` as a `:match` or an `:assert`, `(and)`, or
`eq` where an arity check reads it.

**The extra item: `eq` at another arity** (the coordinator's brief; S1f.5.18's finding, its `program_read.rs` test's
error past the load). Probed on `ee3da657` (`build/s1f.5.9/case/arity-*.ein`):

| program | `ee3da657` |
|---|---|
| `(rule r (?R) :match (?R ?x) …)`, `(r eq)` | every command stops at root where the plan is compiled, exit 1, *`(eq …)` takes exactly 2 arguments and was given 1: SForm(head=Var(name='R'), …) at Loc(…) — a built-in predicate compares exactly two values, so write one guard per pair.* (`saturate`: prefixed `ein.inference.compile.CompileError:`) |
| the same, `(?R ?x ?x ?x)`, or `(absent (?R ?x))` | the same stop, *given 3*, *given 1* |
| the same activator derived, `:assert (r eq)`, through a marker `(mark eq)`, or through an `(eq ?f ?f)` guard while `eq` is a name the program writes | the same stop at root |
| the marker's derivation only where a commitment holds a `p` | `saturate` answers, exit 0; `solve`, `solve --minimal` and `oracle` stop at the commitment, exit 1 |
| an hrule `(hrule h (?R) :match (?R ?x) …)`, `(solve :hrules (h (eq)))` | `saturate` answers, exit 0; `solve`, `--minimal` and `oracle` stop before reading a state, exit 1 |
| a conclusion `(?R ?x)` under `(r eq)` | root stores `(eq a :rule r)`, R answered, exit 0 |
| `(?R ?x ?y)` under `(r eq)` | answers: `eq`'s guard, as S1f.5.7 made it |
| **through a declared relation of another arity**, `(r q)` with `(relation q Node Node)` | **answers, and is no row**: `(q ?x)` matches nothing, as written directly it does (a fact `(q a)` of a binary declaration loads too; matching is arity-coupled, `01_grammar.md`'s *The membership fact*) |
| a written fact `(eq a)`, `(eq)`, `(eq a a a)`, `(not (eq a))` | loads, the fact stored; Q-M1f.33's check reads two arguments only |

`(r neq)` is a parse error, `neq` being no name, so only `eq` reaches a parameter. **A load-time check can see every
row** (Q-M1f.46): the values a relation parameter can hold are what S1f.5.18's check reads, positions that hold
relation names fed by facts, conclusions, `(solve :hrules …)` activators, markers, parameters and guards, and `eq` is
a name, written where it flows. So the rows are filed now and fixed by T1f.5.9.4. The prototype's check refuses each
row's program with one line, and moves no other program of the five roots (the sweep above).

**The rows** ([Q-M1f.46](../open_questions.md#q-m1f46--a-relation-parameter-bound-to-eq-at-another-arity),
[Q-M1f.47](../open_questions.md#q-m1f47--a-written-fact-of-eq-at-another-arity), Q-M1f.36's reading): each way the
engine answers today is a row whose `right` is the refusal, filed `open`, owned by S1f.5.9, each witness under
`examples/ein-bugs/wrong-answers/` with its hand derivation, `:why`s and a corpus entry, each `wrong` and `now` taken at
`ee3da657`, release and debug alike:

| row | witness | `now`, today | `right` |
|---|---|---|---|
| `W60/eq-arity-premise` | `eq-arity-premise.ein`: `(r eq)` written | every command stops at root, exit 1, the compiler's arity line | refused at load |
| `W60/eq-arity-under-commitment` | `eq-arity-under-commitment.ein`: `(mark eq)` passed on where a `p` holds | describe, `--minimal` and `oracle` stop; `saturate` answers, exit 0 (the corpus entry banks all three runs) | refused at load on every command |
| `W60/eq-arity-hrule` | `eq-arity-hrule.ein`: `(solve :hrules (h (eq)))` | describe, `--minimal` and `oracle` stop; `saturate` answers | refused at load on every command |
| `W60/eq-arity-conclusion` | `eq-arity-conclusion.ein`: `:assert (?R ?x)` under `(r eq)` | root stores `(eq a :rule r)`, R two states, exit 0 | refused at load |
| `W61/fact-eq-arity` | `fact-eq-arity.ein`: `(eq a)` | stored, R two states, exit 0 | refused at load |

W31's four rows and W34's stay `open` until T1f.5.9.2. The register reads **148 fixed, 23 open and 1 not wrong**
(`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).

**What moves by banking**, with no engine change: `corpus_exits.txt` gains the five witnesses' eleven cells
(`eq-arity-premise` 1 and 1, `-under-commitment` `solve` 1, `solve --minimal` 1, `saturate` 0, `-hrule` 1 and 0,
`-conclusion` 0 and 0, `fact-eq-arity` 0 and 0), and `corpus_shapes.md5` 205 digests, 41 per witness, none of any other
program moved; `examples/README.md` names the witnesses (161 → 166 programs under `ein-bugs/wrong-answers/`); and
`detector`'s count of the corpus programs the static check flags is 49 where it was 48, `eq-arity-under-commitment.ein`'s
`r` an `activator` site root compiled no plan of, `mk` deriving the activator where a `p` holds, which a commitment can
change; and `ein-render`'s `line_modes_past_the_limit` names that witness (`ORACLE_STOPS`, held both ways), a flagged
program whose oracle stops where `(r eq)`'s plan is compiled, which the test held to answer, until T1f.5.9.4 refuses it at
load. The first run of the workspace's tests found those two (`build/s1f.5.9/t1-tests.log`), which this commit updates;
the gate run its body names is the whole gate again. The prototype is set aside as `build/s1f.5.9/prototype.patch`;
T1f.5.9.2 to .4 land it, a part each.

## For the merge

Written by T1f.5.9.5, at the stage's end: what the prose both lanes share should gain, and lane 2's total.
