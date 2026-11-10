# S1f.5.24 — A macro's body is checked where a rule's is: at load

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 4
([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). T1f.5.24.1, the explanation the
ruling asks for, was taken by the coordinator the same day ([its record](#t1-record-2026-11-10)). Where the refusal
goes was put to the user as
[Q-M1f.67](../open_questions.md#q-m1f67--where-is-a-macro-body-that-no-rule-could-use-refused), and **ruled the same
day**: an unused malformed macro is allowed, and the requirement is that `(eq <arg>)` is never executed
([§ The ruling on Q-M1f.67](#the-ruling-on-q-m1f67)). The stage is cut to that. **Done 2026-11-10**, in one working
tree for the coordinator to commit: T1f.5.24.2 ([its record](#t2-record-2026-11-10)), eighteen shapes at every site on
every path, the compile-time refusal shown to hold only at root, and seven primitives matched, concluded or
proposed, `(eq k)` stored among them; W66 to W69, sixteen rows, filed `open` and held to the unchanged engine;
T1f.5.24.3 ([its record](#t3-record-2026-11-10)), the loader holding a macro's expansion to the grammar's shapes,
every command refusing before any path runs the rule, the sixteen rows `fixed`, and the pin; T1f.5.24.4
([its record](#t4-record-2026-11-10)), the pages; the acceptance re-taken
([§ Acceptance, re-taken](#acceptance-re-taken-2026-11-10)). Five questions taken by default, Q-M1f.80 to Q-M1f.84.
The register reads 193 fixed, 13 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`,
2026-11-10). Four wrong answers found outside the stage are recorded in the T2 record, not fixed, and given to
[S1f.5.28](s1f.5.28_the_unchecked_findings_triaged.md) as its items 13 to 16, and moved the same day to
[P1f.7](../p1f.7_findings_outside_scope/README.md) as S1f.7.2 to S1f.7.5. Committed as `ddc5c28a`, whose hash W66
to W69's `[case.fixed]` rows carry.
**Depends on:** [S1f.5.21](s1f.5.21_a_compile_error_in_eins_words.md), for the line's spelling.

## The ruling

The close review's finding 4, as written on 2026-11-10:

> **A macro template `(eq ?a)` loads, then every command refuses it at compile, all in the same way** (S1f.5.9's
> record; not re-checked).

The user, 2026-11-10: *"new stage, explain"*.

## T1 record, 2026-11-10

Taken by the coordinator on the release binary at `21c24c07`, on

```lisp
(relation q T :why "q holds of {?1}")
(macro one (?a) (eq ?a) :why "one {?a}")
(rule r () :match (and (q ?x) (one ?x)) :assert (q ?x) :why "r")
(q a)
```

**What happens.** The rule `r` uses the macro `one`, whose body expands `(one ?x)` to `(eq ?x)`: `eq` with one
argument, where `eq` compares two. Written directly in the rule, `(eq ?x)` is a parse error, since the grammar's
`Guard ::= '(' 'eq' Term Term ')'` takes exactly two. Written in a macro's body, it parses, loads and is
refused only when the rule is compiled:

| command | exit | what it prints |
|---|---:|---|
| `ein solve` (every mode), `ein oracle`, `ein render lattice` | 1 | ``(eq …)` takes exactly 2 arguments and was given 1: SForm(head=Atom(name='eq'), args=(Var(name='x'),)) at Loc(file='eq-macro.ein', line=2, col=18) — a built-in predicate compares exactly two values, so write one guard per pair`` |
| `ein saturate` | 1 | the same, behind `ein.inference.compile.CompileError:` |
| `ein render rules`, `ein render program` | 0 | the rule, drawn |
| `ein kb save` | 0 | a container every command above refuses (`program_read::a_containers_positions_name_its_stored_program` relies on it) |
| `ein test` | 2 | *nothing to check*, since the file states no `:expect`; with one, it is refused as `solve` refuses it |

So *every command refuses it the same way* holds only for the commands that compile. The line is S1f.5.21's to
put in Ein's words. Whether it is a load refusal is this stage's question.

**Why the grammar lets it through.** A macro's body is a `Template`, which takes any head at any arity
(`Template ::= … | '(' AnyHead TemplateItems ')'`). It has to: `std.macro`'s `forall` writes `(absent (and ?G
(absent ?B)))` and `unknown` writes `(and (absent ?P) (absent (not ?P)))`, and each `?G`, `?B` and `?P` stands for a
whole premise. No rule production can be written until the arguments are known. The expansion happens after the
parse, on the tree, so the expanded rule is never read by the grammar. S1f.5.5's T5 kept the twelve loader lines
that a program still reaches only this way (`grammar_pages::the_lines_the_grammar_does_not_cover_are_still_reached`).
`eq`'s and `absent`'s arity stayed the compiler's (`02_patterns.md`'s *where a malformed primitive is caught*).

**What can be decided where.**
- **At the definition**, whenever the body's head is a literal primitive. `(eq ?a)` is wrong for every use: `eq`
  is literal, and one argument is one argument whatever `?a` becomes. The same holds for a literal `neq`,
  `absent`, `not`, `false`, `and`/`or` with nothing, and `open` (S1f.5.23).
- **Only at a use**, when the shape depends on the arguments. `(macro m (?R ?a) (?R ?a))` used as `(m eq ?x)`
  expands to `(eq ?x)`, and `(absent ?P)` with `?P` given two premises expands to an `absent` of two.

**The census.** The corpus writes five programs with a `(macro …)` besides the stdlib's two macros, and none is
refused, so neither placement moves a corpus answer. Only `program_read`'s witness and the stage's own move.

## The ruling on Q-M1f.67

The user, 2026-11-10: *"So the question is "do we allow invalid macro if it is not instantiated - the answer is
yes, we allow, error will raise later during either compilation of instantiation or even later, the ultimate goal in
this case is not allow execution of (eq <arg>)"*.

So **(a)'s definition-time refusal is not built**. A macro's body is not checked at its definition. A malformed
primitive it produces may be refused at expansion, at compile, or later, and the stage's requirement is one
property: **no path executes a malformed primitive**. Today's compile-time refusal meets it wherever every path that
fires a rule compiles first. The stage shows that, pins it, and fixes a path that does not. It moves the refusal
only if a path executes before compiling. `kb save`, `render program` and `render rules` keep accepting such a
program, since none of them executes a rule. The line is S1f.5.21's to put in Ein's words. The task list below is
cut to this. The options below are the record of what was put.

## What it changes, by Q-M1f.67 (the options put to the user)

- **(a), recommended**: refused at the definition where the literal head decides, and at each use after expansion
  where it cannot. One path: the expanded rule is checked by the same checks the grammar applies to a written rule,
  so the twelve lines T5 kept and the compiler's arity lines are one check, named in Ein's words, naming the rule,
  the macro and the place. A macro no rule uses still loads if its body is well-formed for some use.
- **(b)**: refused only at each use, after expansion. A macro no rule uses loads, whatever its body.
- **(c)**: left at compile. Only S1f.5.21's wording changes, and `kb save` and the views keep accepting it.

Under (a) and (b) the compiler's two arity lines become invariants no program reaches (Q-M1f.9 (a)'s pattern), and
each command run whose exit moves is a register row, filed before the fix (Q-M1f.44).

## Defaults, as taken (2026-11-10)

Five choices the stage leaves open, each taken by default and recorded as a row of
[`open_questions.md`](../open_questions.md), marked *defaulted 2026-11-10*:

- **Where a malformed primitive is refused**, once T2 found paths that execute one or answer without compiling it:
  at load, after expansion, the instantiation the ruling allows, by every command, the views and `kb save` among
  them ([Q-M1f.80](../open_questions.md#q-m1f80--where-a-malformed-primitive-a-macros-expansion-writes-is-refused)).
  The alternative, an eager check every executing command runs before it fires with the views and `kb save` still
  accepting the program, as the ruling's section above expected while the compile-time refusal was thought to
  suffice, is the row's (b).
- **What a malformed primitive is**: every form the grammar would refuse where it stands if the expanded rule were
  written, not only `eq`'s and `absent`'s arity
  ([Q-M1f.81](../open_questions.md#q-m1f81--what-a-malformed-primitive-is)).
- **The line**: the compiler's, S1f.5.21's spelling, behind `kb load error: `, its position the expanded form's
  ([Q-M1f.82](../open_questions.md#q-m1f82--the-line-a-malformed-primitive-is-refused-with)).
- **Which moves are register rows**: one per way the engine answered where a refusal is right, sixteen
  ([Q-M1f.83](../open_questions.md#q-m1f83--which-of-what-the-refusal-replaces-is-a-register-row)).
- **The compiler's two arity arms and the firing's whole-conclusion line**: invariants
  ([Q-M1f.84](../open_questions.md#q-m1f84--the-compilers-arity-arms-and-the-firings-whole-conclusion-line)).

## Tasks

### Task T1f.5.24.1 — the explanation (done, [above](#t1-record-2026-11-10))

### Task T1f.5.24.2 — every path, shown not to execute

One witness per primitive and per shape a macro's body can malform it: a literal head, a head through a
parameter, and `absent` or `not` given a premise list. Use each in a rule, in an hrule and in an obligation rule.
Run each witness on every command and every path that fires a rule: `saturate`, every `solve` mode, `oracle`,
`test` with an `:expect`, `render lattice`, `render kb`, `--events`, a container of it, and the hrule and
obligation passes. Record, per run, whether a malformed primitive is reached before the refusal. A path that
reaches one is a register row, filed before the fix.

### Task T1f.5.24.3 — the pin, and a fix where a path executes

A test over the witnesses and every path above: each refuses before any firing, with S1f.5.21's line, and none
executes a malformed primitive. Where T2 finds a path that does, it is refused there, at expansion or at compile,
whichever comes before it, and the row is flipped.

### Task T1f.5.24.4 — the pages

`02_patterns.md`'s *where a malformed primitive is caught*, `00_ebnf.md` § 4 (what the grammar does not refuse),
[`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md), `defined_behaviour.md` § 4, the
stage's records.

## Acceptance

- No path executes a malformed primitive a macro's body produces, held by a test over every witness and every
  path that fires a rule; each refuses with S1f.5.21's line.
- An unused malformed macro loads (the ruling), held by the same test.
- Every exit that moved is a register row, `fixed`; no corpus answer moved.
- `./run_tests.sh` green.

## T2 record, 2026-11-10

Taken on the release binary at `dbfd7862` (`./build.sh --engine`), before any engine edit. The scripts and the
programs are under the session's scratch (`census.py`, `summ.py`, `table.py`, `sweep.sh`); every program a register
row reads is banked under `examples/ein-bugs/wrong-answers/`.

**What the grammar and the loader refused already**, so no witness is banked for it. A template shapes `not` (one
template), `neq` (two items), `and` and `or` (one or more), so `(not ?a ?b)`, `(not)`, `(neq ?a)`, `(and)` and
`(or)` in a macro's body are parse errors, and `not`, `neq`, `and` and `or` are no names, so a call cannot pass one
to a parameter written as a head (`(app neq ?x)` is *unexpected `neq`*). `open` in a `:match`, at another arity or
with a parameter its rule does not bind is `validate_open`'s, at load, after expansion. A premise that is no form,
which a body that is a bare parameter writes, `(id foo)`, is `refuse_no_fact_holds`' (W24), and a nested `(or …)` from a
macro is S1f.5.21's (W62, Q-M1f.74). A macro is expanded only in a rule's, an hrule's and an obligation's `:match`
and `:assert`, never in a query's `:goal`, where a call reads as a relation of its name (the fourth finding
below), so no witness is a goal.

**What loads**, eighteen shapes: in a `:match`, `eq` at one argument and `absent` at two (and, probed, `eq` at none or
three and `absent` at none, which read the same line), each written as the body's head or reached through a parameter
written as one (`(macro app (?R ?a) (?R ?a))` called `(app eq ?x)`),
`(not ?a)` called with a conjunction or a variable (and `std.macro`'s `unknown` called with a conjunction, whose
`(absent (not ?P))` is the same), and `(neq ?a ?b)` called with a conjunction as a value; as a conclusion, `eq` at
one argument (literal and through a parameter), `false` with an argument (literal and through a parameter), an
`(or …)`, `(not k)`, `(not (and …))`, a conjunction as a value, and a body that is a bare parameter, called with a
variable or a name. Each in a rule at root, in a rule only a commitment activates, in a rule no activator reaches,
in an hrule and, in a `:match`, in an obligation (`:assert (open)`): eighty programs. Every one was run on
`saturate`, `saturate --dump`, `solve` (describe), `--minimal`, `--maximal`, `--single`, `--count`, `-n 1`, `solve
--events`, `oracle`, `test` with a claim, `render lattice`, `render kb`, `render program`, `render rules`, `kb save`,
and `saturate`, `solve`, `oracle` and `test` on the container `kb save` wrote. What happened, by class and site (a
run *executes* where the rule's plan is compiled and it fires, is matched by the hypothesis layer or owes, per
`--events` and `saturate --dump`):

| class | at root | only under a commitment | no activator | hrule | obligation |
|---|---|---|---|---|---|
| `eq` or `absent` at another arity in a `:match`, literal or through a parameter | every command that compiles stops on the compiler's line, the container's too; the views and `kb save` answer. Nothing executes: **no row** (the ruling allows an error at compile) | `saturate`, `--minimal`, `--single`, `-n 1`, `render kb` and the container's `saturate` **answer**, never compiling `r`; describe, `--maximal`, `--count`, `--events`, `oracle`, `test` and `render lattice` stop | **every command answers** | `saturate` and `render kb` **answer**; every solve question, `oracle`, `test`, `render lattice` stop where the hypothesis layer compiles it | as an hrule: the obligation pass compiles it |
| `not` of a conjunction or a variable, `unknown` of a conjunction, a conjunction as `neq`'s value, in a `:match` | **executed**: the rule fires at root, `(p k)` derived, on every command that saturates; every command answers | **executed** under the commitment; every command answers | every command answers | **executed** by the hypothesis layer, `(p k)` proposed (an hrule that reads `not` stops a debug build, the first finding below) | **executed** by the obligation pass: every state owes |
| `eq` at one argument, `false` with an argument, an `(or …)`, `(not k)`, `(not (and …))` or a conjunction as a value, as a conclusion | **executed**: `(eq k)`, `(false k)` (root then dead, R empty, *refuted by r*), `(or (p k) (q k))`, `(not k)`, `(not (and (p k) (q k)))`, `(q (and (p k)))` stored on every command that saturates; every command answers | **executed** under the commitment | every command answers | **executed** by the hypothesis layer: each proposed as a candidate, `(eq k)` committed in a state | — (an obligation concludes `(open …)`) |
| a conclusion that is no form, from a body that is a bare parameter | every command that saturates stops at the firing (`rule 'r': its :assert concludes ?x, which is not a fact …`); the views and `kb save` answer. **No row**, as at root above | `saturate`, `--minimal`, `--single`, `-n 1` and `render kb` **answer**; describe, `--maximal`, `--count`, `oracle`, `test`, `render lattice` stop at the firing | every command answers | **answers**: the hypothesis layer drops the candidate in silence, H empty | — |

So today's compile-time refusal met the ruling's property only at root: under a commitment, in an hrule, in an
obligation and in a rule no activator reaches, commands answered a program others refused, and seven primitives
the compiler never checked were matched, concluded or proposed, `(eq k)` among them, which is the ruling's own
case. Each such way is a register row, and a refusal at load comes before every path, so the stage moves the
refusal (Q-M1f.80).

**Register rows**, filed `open` by this task (Q-M1f.83, Q-M1f.36's reading), each `owner` S1f.5.24, `right` the
refusal, witness under `examples/ein-bugs/wrong-answers/` with a corpus entry (`regression`), `wrong` and `now`
taken at `dbfd7862`, held to the unchanged engine by `wrong_answers.rs` (`cargo test -p ein-cli --test
wrong_answers`, 2026-11-10, green; the register then read 177 fixed, 29 open and 1 not wrong):

- **W66**, `eq` or `absent` at another arity in a `:match`, refused only where the compiler met it, four rows:
  `macro-eq-under-commitment`, `macro-absent-hrule`, `macro-eq-obligation` (`eq` through a parameter),
  `macro-absent-never-compiled` (`absent` through a parameter), the last standing for the no-activator site in
  W67 to W69 too;
- **W67**, a malformed primitive in a `:match` matched, four rows: `macro-not-conjunction`, `macro-neq-connective`
  (at root), `macro-neq-connective-hrule` (the hypothesis layer), `macro-not-conjunction-obligation` (the
  obligation pass);
- **W68**, a malformed conclusion concluded, six rows: `macro-eq-conclusion`, `macro-false-argument`,
  `macro-or-conclusion`, `macro-not-name-conclusion`, `macro-connective-value` (at root) and `macro-eq-candidate`
  (proposed by an hrule);
- **W69**, a conclusion that is no form, two rows: `macro-bare-conclusion-under-commitment` and
  `macro-bare-conclusion-hrule`.

Sixteen rows, the register's schema `/20` unchanged: a row whose `now` answered while open is a command's run
(`saturate --dump`, or `oracle` where only the hypothesis layer or the obligation pass reaches the rule) with
`answered = true`, and one that stopped is `solve` with the stop's `error`. The hrule row of W67 is
`macro-neq-connective-hrule`, not an hrule over `(not (and …))`, because a debug build stops on that program before
the fix as it does on any hrule reading `not` (the first finding below), which a row held by the debug test binary
cannot record.

**The corpus**, `sweep.sh` (every `.ein` under `examples tests stdlib corpus plans docs`, 576 files, `ein kb save`
with the binary before the fix and after, exit and first line compared): seventeen move, the sixteen witnesses
and `examples/broken/compile/eq_in_a_macro_body.ein`, S1f.5.21's fixture; no other program's load moves, so no
other answer can (one more line differs, `import_module_not_found`'s, only where each binary was run from, which
names where it looks for the stdlib).

**What will move**, listed before any golden was re-blessed:

- `examples/broken/compile/eq_in_a_macro_body.{ein,expected}` → `examples/broken/load/`, its header restated,
  its `.expected` the loader's line (`{FILE}:23:18: rule 'r': …`); its corpus entry `load-negative`, its runs
  `solve`, `saturate`, `render rules`, as every load-negative entry's are. The `compile-negative` group has no
  entry since; `activator_arity` stays in the directory, `positive`.
- *`ein.rs/crates/ein-cli/tests/golden/corpus_exits.txt`*: `broken/compile/eq_in_a_macro_body.ein`'s two cells
  (`saturate`, `solve`, exit 1) out; `broken/load/eq_in_a_macro_body.ein`'s three in (`saturate`, `solve`,
  `render rules`, each 1); the sixteen witnesses' thirty-four cells in (`solve` and `saturate` each, and
  `solve --minimal` on the two commitment witnesses), each exit 1. So 2 lines out and 37 in, 1 539 cells.
- *`ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5`*: `broken/compile/eq_in_a_macro_body.ein`'s 42 lines
  out, `broken/load/eq_in_a_macro_body.ein`'s 20 in (a refused program's, as `nested_or`'s), and the sixteen
  witnesses' 20 lines each in: 42 out and 340 in. No other corpus program's rendering moves, its read-out among
  them, so no other answer does.
- *Tests* that pin today's behaviour: `ein-cli`'s `cli_semantics::a_compile_refusal_reads_the_same_on_every_command`
  (the macro body's line, the views and `kb save` accepting it, the container's position) and
  `program_read::a_containers_positions_name_its_stored_program` (`kb save` writing that container);
  `ein-infer`'s `compile_semantics` (`every_compile_error_says_what_its_expected_file_says`'s two stems,
  `the_compile_and_firing_lines_a_macro_still_reaches`, `no_corpus_program_is_refused_in_the_python_spelling`'s
  one compile refusal); `ein-ir`'s `fuzz_properties::the_compiler_refuses_nothing_the_loader_accepts` (its
  exemption for a macro body's arity), `grammar_pages::the_lines_the_grammar_does_not_cover_are_still_reached`
  and `load_semantics::the_load_negative_fixtures_are_byte_identical` (the moved fixture).

No other golden: `help_shape.txt` is unmoved (no option), and so is every summary, read-out and plan-shape golden.

**Found outside the stage, recorded and not fixed** (given to [S1f.5.28](s1f.5.28_the_unchecked_findings_triaged.md) as
its items 13 to 16, then to [P1f.7](../p1f.7_findings_outside_scope/README.md) as S1f.7.2 to S1f.7.5) (each on the release binary at `dbfd7862`, programs in the
session's scratch):

1. **An hrule whose `:match` reads `not` stops a debug build.** `(hrule h () :match (and (a ?x) (absent (not (p
   ?x)))) …)`, written, no macro: `hrule.rs`'s `debug_assert!` that no hrule reads `not` (*an hrule reads `not`,
   which hypgen's kill cache writes mid-enumeration*) fires, exit 101, on every solve question; the release build
   answers, two states. Nothing refuses such an hrule at load, so the two builds disagree.
2. **A connective written as a premise pattern's argument reads nothing, in silence.** `(rule r () :match (and (p
   ?x) (q (and (p ?x)))) …)`: the grammar's `Arg` takes a connective, so a macro can be called there, and the
   loader and the compiler read `(and (p ?x))` as a pattern inside a pattern whose relation is `and`, which no fact
   holds, so `r` never fires (written, and through a macro's argument, `(macro m (?a) (q ?a))` called with a
   conjunction). The same shape is a parse error as a conclusion's or a guard's value, and S1f.5.24 refuses it
   there; in a premise pattern it is the grammar's own, so not this stage's.
3. **A conclusion `(not ?x)` whose variable binds a name stores the negation of a name.** `(rule r () :match (a
   ?x) :assert (not ?x) …)` stores `(not k :rule r)`, which no program can write (`(not k)` is a parse error as a
   fact and as a conclusion). The grammar takes `(not Var)` as a conclusion, for a variable bound to a fact.
4. **A macro called in a query's `:goal` is not expanded.** `(macro pp (?a) (p ?a))` and `(query :goal (pp ?x))`:
   the goal reads a relation `pp`, which nothing declares, and *the goal has no row*, in silence, where the same
   call in a rule's `:match` is expanded (`expand_rule_clauses` reads rules only), and an unimported `std.macro`
   call in a rule is refused (`unimported_macro_errors`).

`(absent (p ?x))` as a conclusion, which stores a fact of a relation `absent`, is the grammar's (`ConclusionHead`
takes `absent`) and S1f.5.23's list of position words a fact head admits; it is not re-filed here.

## T3 record, 2026-11-10

**The loader** (`ein-ir/src/from_ir.rs`): `refuse_malformed_primitives`, called in `ingest_rules` on a rule's, an
hrule's and an obligation's `:match` and `:assert`, after macro expansion and after the checks S1f.5.21 moved there
(a form they refuse is not read twice), before `refuse_unbound_conclusions` (Q-M1f.80). Its walk is the grammar's,
one method per position of `ein.y` (`Shapes`: a premise, a pattern's argument, a value, the pattern a `not` negates,
a conclusion), and it reads a form headed `eq`, `neq`, `absent`, `not`, `false`, `and` or `or` against the shape the
grammar gives it there, and a conclusion that is no form (Q-M1f.81); `open`, a premise that is no form and a nested
`(or …)` stay `validate_open`'s, `refuse_no_fact_holds`' and `refuse_nested_disjunctions`'. One line per form, in
the compiler's words (Q-M1f.82): the position where the expanded form is written (its own, the macro body's; else,
for a form the grammar builds for a keyword, its first part's, the call's argument), the rule as the loader names it,
the clause, the form as Ein text and what to write: *`<file>:23:18: rule 'r': its :match writes (eq ?x), and eq
takes exactly 2 arguments, not 1 — it compares two values, so write one guard per pair`*, *`… its :assert writes
(false ?x), and false takes no argument, not 1 — (false) concludes that the state is impossible, and names
nothing`*, *`… its :match writes (not (and (p ?x) (q ?x))), and not takes a pattern (relation argument …), not (and
(p ?x) (q ?x)) — it is the stored negation of one fact; negation as failure is (absent …)`*, *`… (and (p k)) is a
connective where a value goes — and heads no fact, and a value is a name, a variable or a fact (relation argument
…)`*, *`… an (or …) is no conclusion — …: write one rule per disjunct`*, and the firing's *`… its :assert concludes
?x, which is not a fact — …`*.

**The compiler and the firing** (`ein-infer`): the `absent` and `eq` arity arms in `compile.rs`'s `premise` are
`assert!`s naming the loader's checks, and `bad_arity` is gone; `firing.rs`'s whole-conclusion arm of `not_a_fact` is
`unreachable!`, and `hrule.rs`'s note on a template that is no fact says why (Q-M1f.84).

**The census, re-taken on the build** (`census.py` over the eighty programs, release build of the working tree,
2026-11-10): every one is refused on every command, `saturate`, `saturate --dump`, the six solve questions,
`--events`, `oracle`, `test` with a claim, `render lattice`, `render kb`, `render program`, `render rules` and `kb
save`, exit 1 (`ein test` 2), with one first line, the same on each; `kb save` writes no container, and no `--events`
log holds a step of the rule. A program declaring every one of the bodies, called by no rule, loads and is answered
by every command, its container too. **The corpus**, `sweep.sh` again: the seventeen programs T2 listed move, and
no other.

**What moved**, exactly T2's list:

- `corpus/wrong_answers.toml`: W66's four, W67's four, W68's six and W69's two rows `fixed`, each `now` the loader's
  line, `[case.fixed]` the null id (the stage's commit not yet written), `wrong` the record T1f.5.24.2 took at
  `dbfd7862`; schema `/20` unchanged. The register reads **193 fixed, 13 open and 1 not wrong** (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-10).
- `corpus_exits.txt`, re-blessed (`EIN_BLESS=1 cargo test -p ein-cli --test corpus_cli`): 2 lines out and 37 in
  (`git diff --numstat`, 2026-11-10), 1 539 cells (`wc -l`), exactly the cells T2 named.
- `corpus_shapes.md5`, re-blessed (`EIN_BLESS=1 cargo test -p ein-render --test corpus_shapes`): 42 lines out and
  340 in, on exactly the seventeen programs T2 named, 20 lines each in.
- `eq_in_a_macro_body.{ein,expected}` under `examples/broken/load/`, its header restated, its `.expected` `{FILE}:23:18:
  rule 'r': …`; its corpus entry `load-negative`; `compile-negative` has no entry, its header saying why.

**The tests.** New: `ein-cli/tests/macro_bodies.rs`, the pin
(`every_malformed_primitive_is_refused_before_any_path_runs_it`: eighteen shapes at every site, eighty programs, each
on twenty commands, `test --events` and `kb save --saturate` among them; `the_registers_witnesses_are_refused_on_every_command`,
the sixteen witnesses and the moved fixture on the same twenty; `an_unused_malformed_macro_loads`), held both ways:
with the loader's two calls commented out, the first two fail (the compiler's invariants abort the root cases, exit
101, and the rest answer) and the third holds; `ein-ir`'s `from_ir::tests::a_macros_expansion_is_held_to_the_grammars_shapes`
(each shape at four sites, the position, an unused body, and twenty-three shapes the grammar takes, written and
through a macro, that load). Restated: `cli_semantics::a_compile_refusal_reads_the_same_on_every_command` (the moved
fixture among its witnesses, refused by all seventeen commands), `program_read::a_containers_positions_name_its_stored_program`
(`kb save` refuses the program, so the container is written through `ein_einb::save`, and the stored program's line
is `kb load error: .einb PROGRAM does not load: <program stored in g.einb>:…`), `compile_semantics`'
`every_compile_error_says_what_its_expected_file_says` (one stem), `the_lines_a_macro_body_reaches_are_the_loaders`
(was `the_compile_and_firing_lines_a_macro_still_reaches`) and `no_corpus_program_is_refused_in_the_python_spelling`
(no compile refusal), `fuzz_properties::the_compiler_refuses_nothing_the_loader_accepts` (its exemption for a macro
body's arity gone), `grammar_pages::the_lines_the_grammar_does_not_cover_are_still_reached` (seven lines added),
`load_semantics::the_load_negative_fixtures_are_byte_identical` (the moved fixture) and `ein-infer`'s
`register_region::every_row_states_its_witness_s_region`, which reads `{FILE}` in a row's refused line as the path
the run was handed, as `wrong_answers.rs` does (schema `/16`), the first refused rows whose line places its form.

## T4 record, 2026-11-10

**The pages.**
- [`02_patterns.md`](../../../docs/kernel/ir/03-ein-lang/02_patterns.md)'s *where a malformed primitive is caught*:
  the grammar where a rule writes a primitive and the loader, after expansion, where a macro's body or a call's
  argument does; what the loader reads, the line, an unused macro loading, and what the compiler and the firing met
  until S1f.5.24 (W66 to W69).
- [`00_ebnf.md`](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md) § 4: *what a macro's body expands to* names every
  shape the loader refuses since, and that a macro no rule calls is not read.
- [`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md): `unknown`'s expansion is held
  to the grammar, `(unknown (and …))` refused (W67).
- [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md) § 4: the compile row (no program reaches a
  compile refusal) and the conclusion row (no program reaches the firing's line), § 4.3's closing paragraph, § 4's
  conclusion lines' history, § 1.5's count (14 of 35 fixtures end `at None`, `grep -L 'at None'
  examples/broken/load/*.expected | wc -l` 21, 2026-11-10), and a new
  [§ 4.16](../../../docs/kernel/defined_behaviour.md#416-a-malformed-primitive-a-macros-expansion-writes-at-load--m1f-s1f524),
  every line, the position, what each replaced, what pins it and what is not refused.
- The fixture READMEs (`examples/broken/compile/`, `examples/broken/load/`), `examples/README.md`'s catalogue (193
  witnesses under `ein-bugs/wrong-answers/`, `ls examples/ein-bugs/wrong-answers | wc -l`, 2026-11-10, the `macro-*`
  family described) and `corpus/README.md`'s `compile-negative` row; the register's header; the sixteen witnesses'
  headers.

`utils/doc_audit.py --links --check` over the gate's scope: 399 pages, 0 findings (2026-11-10).

## Acceptance, re-taken (2026-11-10)

Each item, and what holds it:

- *No path executes a malformed primitive a macro's body produces, held by a test over every witness and every path
  that fires a rule; each refuses with S1f.5.21's line*: **met**. `macro_bodies.rs`' first two tests: eighty programs
  and the seventeen witnesses, each on twenty commands (every solve question, `--events`, `oracle`, `test` with a
  claim, `saturate`, the views, `kb save`), exit 1 (`ein test` 2), one first line, the compiler's words behind `kb
  load error: `, no container, and no step of the rule in the `--events` log; held both ways (T3). The refusal is at
  load, which comes before every path: root's saturation, a commitment's, the hypothesis layer's and the obligation
  pass's.
- *An unused malformed macro loads (the ruling), held by the same test*: **met**. `macro_bodies.rs`'
  `an_unused_malformed_macro_loads`, every body declared and called by no rule, answered by every command and its
  container; and `from_ir::tests::a_macros_expansion_is_held_to_the_grammars_shapes`.
- *Every exit that moved is a register row, `fixed`; no corpus answer moved*: **met**. W66's four, W67's four, W68's
  six and W69's two, `fixed`, filed `open` first and held to the unchanged engine (T2); every other exit that moved
  is the moved fixture's (its views' and `kb save`'s under Q-M1f.71, Q-M1f.83) or a witness's; `corpus_shapes.md5`
  moved on no other program, so no read-out did. `wrong_answers.rs` and `ein-corpus`'s `the_register_is_well_formed`
  hold the rows.
- *`./run_tests.sh` green*: **met**, `build/gate-s1f.5.24.log` (green, 17 min 30 s, `./run_tests.sh`, 2026-11-10, exit 0: the six static checks, `cargo test --workspace`, the scratch check, the review bundle's smoke, 207 register rows read, and the bench smoke). The first run of the gate stopped at `register_region`, which compared a refused line's `{FILE}` literally; it reads it as `wrong_answers.rs` does since (T3).
