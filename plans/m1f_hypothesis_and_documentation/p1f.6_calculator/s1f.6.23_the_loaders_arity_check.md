# S1f.6.23 — The loader holds a form to its relation's declared count of arguments

**Phase:** [P1f.6](README.md)
**Status:** started 2026-11-12, on the user's reply of that day: T1f.6.23.2 done, the register's rows filed
`open` ([§ T2 record](#t2-record-2026-11-12-the-rows-filed-open)); the check is next. **An engine stage,** for every
program and not the calculator alone.
**Depends on:** nothing in the phase. It was item 3 of
[S1f.6.12](s1f.6.12_malformed_numbers.md)'s build (*the loader's arity check on nested facts, if ruled*) and
has a file of its own because it is the engine's and the rest of that build is the theory's.

## The ruling

[Question 3](questions.md#q3): *Should the loader refuse a fact whose count of arguments is not the one its
relation declares?* Option (a): *Yes. Register rows first, then the check, a sweep and the pages.* The user,
2026-11-12: *"(a)"*.

What was asked, in the question's words: *every form headed by a declared relation, at any depth, in a fact,
a rule or a query, refused at load with one line naming the relation, the count declared and the count
written.* It is E-2 of [`findings.md`](findings.md), which S1f.6.12's T1 extended: a *top-level* fact with
the wrong count loads too.

## Is it a wrong answer?

Yes, under the ruling. `(add (n z 5))`, one operand where `add` declares two, loads and is answered by
silence, exit 0; so does a `num9` number of eight digits (`questions.md` 3). A program the ruling refuses is
answered today, which is what the register holds. The rows are filed `open` before the check exists.

## What is known before it starts

Of 9 779 forms headed by a declared relation in the 699 tracked `.ein` files, 12 in 6 files have another
count than declared, and each of the six is refused already for another reason (`python3
build/s1f.6.12/arity_census.py`, 2026-11-11; `questions.md` 3). **The Rust suites' inline programs were not
scanned,** and they are where a program that loads today and would be refused can still be.

## Tasks

1. **T1f.6.23.1 — read-only: the census re-taken and completed.** `build/s1f.6.12/arity_census.py` re-run on
   the tree as it stands (the corpus has grown since 2026-11-11), and **the inline programs of the Rust
   suites scanned too**: every `.rs` under `ein.rs/crates/*/tests` and `*/src` that holds Ein text, and the
   `ein` blocks of the doc pages, which S1f.5.14 found to be programs as well. The task's table: each form
   that would be refused, its file, and whether it loads today. It also settles, by reading the loader and
   by probes on the release binary, the edges the ruling's sentence leaves:
   - what *declares* a count: a `(relation R T₁ … Tₙ)` declaration; whether a relation declared with no
     argument types declares zero arguments or none at all;
   - an imported declaration, flat and under `:as`;
   - a form in a rule's `:match`, `:assert` and guard, in an hrule, in a macro's body (checked on its
     expansion, as S1f.5.24 checks a macro's body), in a query's `:goal` and in a claim's facts;
   - a relation written through a parameter, `(?R ?x ?y)`: checked where the program shows what the parameter
     holds, as S1f.5.18's check reads relation names, or not at all;
   - an activator fact against its rule's count of parameters, which is not a declared relation's count.
     **Default: outside this stage,** since the ruling says *declared relation*; what the census finds of it
     is a finding, shown.
   - the kernel's own relations (`is-a`, `__closed__`, `__symmetric__`) and `eq`/`neq`, the last already
     held to their count by `defined_behaviour.md` § 4.14.
   Anything among these that the ruling's sentence does not decide is a question under the next free id, with
   the default taken and the probe that shows it.
2. **T1f.6.23.2 — the register rows, filed `open` before any fix.** Witnesses under
   `examples/ein-bugs/wrong-answers/`: a top-level fact of the wrong count; a nested one (`(n 1)` inside a
   request); the calculator's `(add (n z 5))`; a `num9` of eight digits; a form of the wrong count in a
   rule's `:match` (which matches nothing, in silence) and in a goal. Each `now` is today's answer, each
   `right` the refusal. Held to the unchanged engine by `wrong_answers.rs`.
3. **T1f.6.23.3 — the check,** in `ein_ir::from_ir`, beside the loader's other whole-program refusals: one
   line a form, in Ein's spelling, with the file, line and column, the relation, the count declared and the
   count written. On every command that loads a program, at each command's load-error exit.
4. **T1f.6.23.4 — the pin:** a test file under `ein-cli/tests/` with a case per row of T1's table, the
   refusal's text included; `fuzz_properties` holds every program it accepts to the rule.
5. **T1f.6.23.5 — the sweep.** Old release binary against new over every tracked `.ein` file, `ein solve`
   and `ein saturate --dump`: nothing moves but T2's witnesses and what T1's census named. Anything else that
   moves is shown before a golden is blessed; every golden expected to move is named in the record first.
6. **T1f.6.23.6 — the pages:** [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md), a section
   of its own under the next free number; `01_grammar.md` § Relation, that a declaration's argument list is
   held; the calculator's module pages, whose table of malformations loses its wrong-count rows to *refused
   at load*; `examples/README.md` for the witnesses; `AGENTS.md`; E-2's row of [`findings.md`](findings.md)
   closed.
7. **T1f.6.23.7 — the rows flipped to `fixed`** in the commit that fixes them.

## Acceptance

- A form headed by a declared relation with another count of arguments is refused at load, on every command,
  with one line that names the place, the relation and the two counts.
- The register's rows are fixed, and no other corpus answer moved, by the sweep.
- No program of the corpus, the Rust suites or the doc pages that should load is refused; each one T1 found
  that would be was rewritten or shown to the user before the check landed.
- E-2 is closed.

## What it must not do

- **Not refuse a relation nothing declares.** An undeclared relation has no count to be held to; whether
  facts of undeclared relations are wanted at all is not this stage's.
- **Not become a type check.** The declaration's argument *types* are not checked against what is written;
  that is M1g's types theory ([reply 12](questions.md#q12)).
- **Not add a flag that turns it off.** Config never changes an answer, and a refusal is one.

## The standing rules

The register rows come before the fix. Findings go to [`findings.md`](findings.md) under the next free id;
questions to [`open_questions.md`](../open_questions.md) under the next free id. Every command refuses the
same programs (one tool, consistent answers); a refusal is an error and exits as each command's load error
does, and nothing else changes an exit code. The gate is `./run_tests.sh`, then `./run_tests.sh --nightly`
after the code commit; `utils/ci_container.sh` is not needed.

## T2 record (2026-11-12): the rows filed open

**W89, seven rows, each `open`,** owned by this stage, in
[`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml), each witness under
`examples/ein-bugs/wrong-answers/` with a corpus entry: the six the task names and a rule's `:assert`, which
stores facts of the wrong count where a `:match` only matches nothing.

| row | witness | the form | what every command answers today |
|---|---|---|---|
| `W89/top-level-fact` | `arity-top-level-fact.ein` | `(likes Bob)`, `likes` declared with two | loads; root holds the fact; the goal has Ann's row alone |
| `W89/nested-fact` | `arity-nested-fact.ein` | `(n 1)` inside `(add (n 1) (n z 5))`, `std.arith` | loads; nothing derived; the goal has no row |
| `W89/request` | `arity-request.ein` | `(add (n z 5))`, `std.arith` | loads; nothing derived; the goal has no row |
| `W89/num9-eight-digits` | `arity-num9-eight-digits.ein` | `(num9 0 0 1 2 3 0 7 8)`, `std.num9` | loads; neither a sum nor an overflow; the goal has no row |
| `W89/in-match` | `arity-in-match.ein` | a rule's premise `(likes ?who)` | loads; the rule never fires |
| `W89/in-assert` | `arity-in-assert.ein` | a rule's conclusion `(fan ?who ?what)`, `fan` declared with one | loads; root holds `(fan Ann Tea)`, which the goal `(fan ?who)` does not match |
| `W89/in-goal` | `arity-in-goal.ein` | a goal `(likes ?who)` | loads; *the goal has no row: undetermined here* |

Each exits 0 with nothing on stderr on `ein saturate --dump`, `ein solve` and `ein oracle`, release, at
`f9a6fd1d` (2026-11-12). Each row's `now` is `ein saturate --dump`, a command's run that answers, as W76's
were while open, and its `right` a refusal; `wrong_answers.rs` holds the seven to the unchanged engine.

Two more symptoms were met on the way and are the rows' own: a `--dump-answer` state file prints the English
of a fact that is short of arguments with a place nothing fills, `(likes Bob) ; Bob likes {?2}` and `(add (n z
5)) ; asked: (n z 5) + {?2}`, which `answer_round_trip.rs` forbids, so the two witnesses are named in its
`UNFILLED_TEMPLATE` while their rows are open, and must still print one; and the three witnesses that import
the calculator join `alive_invariant.rs`'s list of programs with a name only the seed holds, and their
directory `dot_wellformed.rs`'s `SKIPPING`, each holding the digit tables. The commit that fixes W89 empties
the first and takes them off the other two. A row whose witness loads states its region, so each of the seven
carries a `[case.region]` while open, R {∅}, which the fix removes, a refused program having none.

The register reads 305 fixed, 23 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq
-c`, 2026-11-12).
