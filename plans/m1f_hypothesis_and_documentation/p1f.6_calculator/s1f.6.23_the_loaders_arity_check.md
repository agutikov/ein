# S1f.6.23 — The loader holds a form to its relation's declared count of arguments

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-12**, on the user's reply of that day: the loader refuses a form headed by a
declared relation whose arguments are not as many as its declaration's types
([§ T1 record](#t1-record-2026-11-12-the-census-and-the-edges),
[§ T2 record](#t2-record-2026-11-12-the-rows-filed-open),
[§ T3 to T7 record](#t3-to-t7-record-2026-11-12-the-check-the-pin-the-sweep-and-the-rows-fixed)). Q-M1f.201 to
.204 defaulted, the user's to revise; E-34 and E-35 found. **An engine stage,** for every program and not the
calculator alone.
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

## T1 record (2026-11-12): the census and the edges

**The census, re-taken.** Of 20 809 forms headed by a declared relation in the 810 tracked `.ein` files, 12 in
6 files have another count than declared, the six of 2026-11-11, and each is refused already for another
reason (`python3 build/s1f.6.12/arity_census.py`, 2026-11-12, at `f9a6fd1d`, before the witnesses).

**The Rust suites and the doc pages.** Two readings, since a text scan cannot follow a program a test
assembles from parts:

- *a scan*, `python3 build/s1f.6.23/inline_census.py` (2026-11-12): every string literal of the `.rs` files
  under `ein.rs/crates` and every fenced block of the tracked `.md` files, 676 texts that declare a relation
  and 2 332 forms headed by a name the text declares; 4 forms in 3 files have another count;
- *the loader itself*, with the check built: `cargo test --workspace --no-fail-fast`
  (`build/s1f.6.23/workspace-nff.log`, 2026-11-12), every inline program and every doc block a test loads.

| the form | where | loads today? | what was done |
|---|---|---|---|
| `(oops)`, `oops` declared `(relation oops T)` | `ein-infer/tests/rule_semantics.rs`, `a_forall_whose_bound_var_is_absent_from_its_guard_still_compiles` | yes | the declaration rewritten bare, `(relation oops)`: the test is about a `forall`, and its rule concludes `(oops)` |
| `(all-beaten ?p)`, declared `(relation all-beaten T T)` | the same file, `the_forall_bound_variable_does_not_escape_the_guard`; found by the loader's run, the scan having missed it | yes | the declaration removed: the test concludes the relation with one argument and then with two, `(all-beaten ?p ?q)`, to show an unbound `?q` refused, and an undeclared relation has no count |
| `(x ?a)` and `(y ?a)`, each declared with two types | `ein-ir/tests/kb_semantics.rs`, `the_reserved_name_guard_is_on_declarators_and_matches_whole_names` | yes | the two declarations rewritten with one type: the test is about a rule's name |
| `(p "a"b)`, `p` declared with one | `ein-ir/tests/grammar_pages.rs`, a row of the refusals table | no: a parse error, which the row holds | nothing |
| `(player ?p)`, `(player ?q)`, declared `(relation player T T)` | `examples/broken/load/unimported_std_macro.ein` | no: refused for its unimported `forall` | the declaration rewritten with one type, so the fixture's refusal stays the one line its `.expected` holds |

No doc page's block was refused, and no other test failed for a refused program: the three tests above were
each a declaration miswritten beside a use that was meant, or one the test did not need, none a test of a
wrong count. Nothing was shown to
the user before the check landed, each being a program of a test and not of a user.

**The edges**, each settled by reading `ein_ir::from_ir` and by a probe on the release binary of `f9a6fd1d`
(`build/s1f.6.23/p/`, every one loading and exiting 0 there):

| the edge | settled | by |
|---|---|---|
| what declares a count | a `(relation R T₁ … Tₙ)`: n. A bare `(relation R)`: none, so `(R)` is its one form | [Q-M1f.201](../open_questions.md#q-m1f201--what-a-bare-relation-r-declares-of-its-count), defaulted; `bare.ein` |
| an imported declaration | held, flat under its name and under `:as o` as `o.name`: the loader reads the program after its imports are resolved | `calc.ein`, `asimp.ein` |
| a rule's `:match`, `:assert` and what a guard holds; an hrule's; an obligation's | held, at any depth, under `(absent …)` and `(not …)`; a guard's argument is a name or a variable, never a form (W24) | `match.ein`, `assert.ein`, `neg.ein` |
| a macro's body | held on its expansion, where a rule or a goal calls it; not read where nothing calls it | [Q-M1f.204](../open_questions.md#q-m1f204--which-written-places-the-declared-count-is-held-in), defaulted |
| a query's `:goal`; a claim's facts | held; a claim's own words are its shape, and only what a `(model …)` lists is read | Q-M1f.204; `goal.ein` |
| a fact's `:using` | held | Q-M1f.204; `kw.ein` |
| a `(solve …)` list | not read: a list of names | Q-M1f.204 |
| a relation written through a parameter, `(?R ?x ?y)` | not read | [Q-M1f.202](../open_questions.md#q-m1f202--a-relation-written-through-a-parameter-and-the-declared-count), defaulted; `param.ein`, `param3.ein` |
| an activator against its rule's parameters | outside the stage, the stage file's default; **a finding, E-34** | [Q-M1f.203](../open_questions.md#q-m1f203--the-kernels-own-relations-and-an-activators-count); `activator.ein` |
| the kernel's own relations | not held: no program declares `__closed__`, `__symmetric__` or `relation`, each reserved, and `is-a` is a relation a program declares like any other; `eq` is § 4.14's. **A finding, E-35** | Q-M1f.203; `kernel.ein`, `decl-fact.ein` |

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

## T3 to T7 record (2026-11-12): the check, the pin, the sweep and the rows fixed

**The check** is `ein_ir::from_ir`'s `wrong_counts`, beside the loader's other whole-program refusals: every
form at or under a node whose head is the name of a declared relation and whose arguments, its keyword pairs
aside, are not as many as the declaration's types. Its callers hand it each written fact, each rule's and
hrule's `:match` and `:assert` after expansion, each query's `:goal` after expansion, and each fact a claim
lists. One line a form, placed at the form, the loader's lines `; `-joined as they are:

```text
kb load error: t.ein:3:2: the fact (likes Bob) gives likes 1 argument, and (relation likes Person Drink) declares 2 — a form headed by a declared relation has as many arguments as its declaration has types
kb load error: t.ein:2:7: the fact (add (n 1) (n z 5)) holds (n 1), which gives n 1 argument, and (relation n Number Digit) declares 2 — …
kb load error: t.ein:6:12: rule 'fans': its :match writes (likes ?who), which gives likes 1 argument, and (relation likes Person Drink) declares 2 — …
kb load error: t.ein:4:15: query: its :goal writes (likes ?who), which gives likes 1 argument, and … declares 2 — …
```

It is the loader's, so every command that loads a program refuses the same programs, at its load-error exit
(1; `ein test`: 2), and no flag turns it off.

**The pin** is `ein-cli/tests/declared_counts.rs`: twenty-four shapes refused on twenty commands with the
line's text and place, the same first line on each and no container written; three forms in one program,
three lines; the seven witnesses; an imported declaration, flat and qualified, from the stdlib and from a
file; and eleven programs that must load, a case per row of T1's table of edges. `ein-ir`'s
`fuzz_properties.rs` holds every program the loader accepts to the rule, read off the loaded store and the
compiled plans and not off the forms the check walks
(`no_accepted_program_holds_a_declared_relation_at_another_count`): 418 programs loaded and 1 refused for a
count at the per-commit budget (`cargo test -p ein-ir --test fuzz_properties no_accepted_program_holds --
--nocapture`, 2026-11-12).

**The sweep.** The release binary of the engine before the check (engine sources `5b252d50`'s) against the new one, `ein saturate --dump` and `ein solve` over every
`.ein` file git knows, 817 files (`python3 build/s1f.6.23/sweep.py`, 2026-11-12): 7 moved, the
seven witnesses, each answered before and refused now, and nothing else. `unimported_std_macro.ein`, refused
before and after, had gained two lines of the check's before its declaration was rewritten (T1's table), and
moves no more. **No program that loaded is refused.**

**The goldens that moved**, each named before it was blessed: the corpus exits of the seven witnesses, 0 to 1
under `solve` and `saturate`; their shape digests, and those of `unimported_std_macro.ein`'s rewritten
declaration; nothing else of `corpus_shapes.md5`. The lists the open rows had needed went back:
`answer_round_trip.rs`'s `UNFILLED_TEMPLATE` is empty, and `alive_invariant.rs`'s `SEEDED` and
`dot_wellformed.rs`'s `SKIPPING` are as they were before T2.

**The rows.** W89's seven are `fixed`, each `now` the loader's line, each `[case.region]` gone, a refused
program having none; `[case.fixed]` carries the null id in the fixing commit and the hash in the next. The
register reads 312 fixed, 16 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`,
2026-11-12).

**The pages**: [`defined_behaviour.md` § 4.26](../../../docs/kernel/defined_behaviour.md#426-a-form-of-a-declared-relation-has-the-declared-count-of-arguments--m1f-s1f623);
`01_grammar.md` § Relation declarator, that a declaration's count is held; `stdlib/README.md`'s two sentences
on a malformed number, `std.arith`'s and `std.num9`'s, each losing its wrong-count case to *refused at load*
([`malformed_numbers.md`](malformed_numbers.md) is S1f.6.12's record and is not edited: its rows m6 to m6d are
this stage's); `examples/README.md`; `AGENTS.md`; and E-2 of [`findings.md`](findings.md), closed.

**Found outside the stage's scope**, each in `findings.md` with its probe: E-34, an activator of another
count than its rule has parameters activates nothing, in silence; and E-35, a kernel marker's fact of another
count loads, and a rule can derive a fact of a declared relation at another count through a parameter.

### Acceptance, re-taken

- *A form headed by a declared relation with another count of arguments is refused at load, on every
  command, with one line that names the place, the relation and the two counts*: `declared_counts.rs`.
- *The register's rows are fixed, and no other corpus answer moved, by the sweep*: above.
- *No program of the corpus, the Rust suites or the doc pages that should load is refused*: T1's table; four
  declarations rewritten and one removed, in three tests and one fixture.
- *E-2 is closed*: its row.
