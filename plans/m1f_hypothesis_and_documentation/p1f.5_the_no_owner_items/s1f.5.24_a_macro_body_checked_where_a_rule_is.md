# S1f.5.24 — A macro's body is checked where a rule's is: at load

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 4
([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). T1f.5.24.1, the explanation the
ruling asks for, was taken by the coordinator the same day ([its record](#t1-record-2026-11-10)). Where the refusal
goes was put to the user as
[Q-M1f.67](../open_questions.md#q-m1f67--where-is-a-macro-body-that-no-rule-could-use-refused), and **ruled the same
day**: an unused malformed macro is allowed, and the requirement is that `(eq <arg>)` is never executed
([§ The ruling on Q-M1f.67](#the-ruling-on-q-m1f67)). The stage is cut to that.
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
