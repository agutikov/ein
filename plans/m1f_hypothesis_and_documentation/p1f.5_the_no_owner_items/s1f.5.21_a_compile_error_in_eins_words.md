# S1f.5.21 — A refusal the compiler makes reads the same on every command, in Ein's words

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 1
([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). Not started.
**Depends on:** nothing. It runs before [S1f.5.24](s1f.5.24_a_macro_body_checked_where_a_rule_is.md), which moves
the one refusal this stage leaves where it is, the one a macro's body reaches.
**Takes:** F23's [D-1](../../followups/f23_diagnostics_in_eins_words.md), its `CompileError` half (the Python class
prefix and the IR's `SForm(…)` repr). The rest of D-1 is re-taken at T1 and taken here if it is the same code path,
or left to F23 with a line saying so.

## The ruling

The close review's finding 1, as written on 2026-11-10:

> **A compile error's first line differs by command, and prints the IR's Python spelling.** On
> `examples/broken/compile/nested_or.ein`: `ein solve`, `ein oracle` and `ein render lattice` print the message
> bare; `ein saturate` puts `ein.inference.compile.CompileError:` in front, a Python class name; `ein render kb`
> puts the file name in front; `ein render program` and `ein render rules` draw the rule and exit 0. Every one
> prints `SForm(head=Atom(name='or'), …)` and cites "S1.8.A13" and "the M1 compiler", and the fixture's
> `nested_or.expected` pins that text. This breaks the one-tool rule.

The user, 2026-11-10: *"new stage in p1f.5"*.

## The case

Re-taken on 2026-11-10, release binary at `21c24c07`, on `examples/broken/compile/nested_or.ein`:

| command | exit | stderr's first line, cut at 120 characters |
|---|---:|---|
| `ein solve` | 1 | ``nested `(or …)` in a `:match` premise: SForm(head=Atom(name='or'), args=(SForm(head=Atom(name='p'), …`` |
| `ein oracle` | 1 | the same |
| `ein render lattice` | 1 | the same |
| `ein saturate` | 1 | ``ein.inference.compile.CompileError: nested `(or …)` in a `:match` premise: SForm(…`` |
| `ein render kb` | 1 | ``examples/broken/compile/nested_or.ein: nested `(or …)` in a `:match` premise: SForm(…`` |
| `ein render program` | 0 | none: the rule is drawn |
| `ein render rules` | 0 | none: the rule is drawn |
| `ein kb save` | 0 | none: a container is written that every command above refuses |
| `ein test` | 2 | *nothing to check*: a query with no `:expect` is never solved, which is `ein test`'s contract |

The `(macro one (?a) (eq ?a) …)` case, finding 4, takes the same line (`` `(eq …)` takes exactly 2 arguments and was
given 1: SForm(head=Atom(name='eq'), …) at Loc(file=…, line=2, col=18) ``) and is
[S1f.5.24](s1f.5.24_a_macro_body_checked_where_a_rule_is.md)'s.

## What it changes

- **One line, one spelling.** Every refusal the compiler makes that a program can reach prints the same first line
  on every command that reads the program: `<file>:<line>:<col>: rule '<name>': <what>`, as the loader's refusals
  since S1f.5.9, .10 and .18 print. The form is written as Ein text (`ein_core::text`), never as the IR's repr. No
  Python class name, no stage id (`S1.8.A13`), and no *the M1 compiler*.
- **Where.** Default (T1 confirms it per refusal): each such refusal moves to the loader, the precedent of S1f.5.9's
  `eq` at another arity, S1f.5.10's unbound conclusion variable and S1f.5.18's derived activator. Then the
  compiler's line is an invariant no program reaches, and a `.einb` cannot store a program every command refuses.
  **Not** the refusals a macro's body reaches: those are S1f.5.24's, where the user rules where they belong.
- **Every command agrees.** A program refused at load is refused by every command that reads a program, with the
  same line and exit code (`ein test`'s 2). That includes `kb save`, and `render program` and `render rules`,
  which today draw a program every other command refuses. Default (T1 records it as a Q-M1f row): the two views
  refuse it too, because the one-tool rule is *no two commands may disagree*, and a view that exits 0 on a refused
  program disagrees about whether the program is one. The alternative, drawing it and printing the line as a
  `note:`, is the row's (b).
- **What a refusal replaces is a register row** where it moves an answer
  ([Q-M1f.36](../open_questions.md#q-m1f36--what-the-refusal-replaces-which-of-it-is-a-register-row)): `kb save`'s
  and the two views' exit 0, each a command run under the register's schema `/16`.

## Defaults

Each is taken as the stage's default and recorded in its file and in `open_questions.md` as a Q-M1f row:
- the line's shape, `<file>:<line>:<col>: rule '<name>': …`, with the form as Ein text;
- the views refuse a refused program (above);
- the D-1 lines outside the compiler (`KeyError`, `FileNotFoundError`): taken here if T1 finds them on the same path,
  else left to F23 with a line in F23's table.

## Tasks

### Task T1f.5.21.1 — every refusal the compiler can make, and every command's line

Read `ein-infer/src/compile.rs` and `error.rs` for every `CompileError` a program can reach, and build a witness for
each. Run every witness on every command that reads a program (`solve` with each mode, `oracle`, `saturate`,
`test` with an `:expect`, `render lattice`, `render kb`, `render program`, `render rules`, `kb save`, and a
container of it where `kb save` writes one), and record the exit and stderr's first line for each. List every
golden, fixture `.expected` and register row the change moves before any edit. File a register row for each
command run whose exit moves. Record D-1's other lines.

### Task T1f.5.21.2 — the refusals at load, in Ein's words

Move each refusal to the loader, one line per slot, and retire the compiler's arm into an invariant naming the
loader check that makes it unreachable (Q-M1f.9 (a)'s pattern). Move `examples/broken/compile/` fixtures that
are now load refusals to `examples/broken/load/`, rewrite their `.expected`, and flip the rows.

### Task T1f.5.21.3 — the pages

[`defined_behaviour.md` § 4](../../../docs/kernel/defined_behaviour.md#4-errors-and-exit-codes) (the refusal lines and exit codes), `02_patterns.md`'s
*where a malformed primitive is caught*, F23's D-1 row (what this stage took), and the stage's records.

## Acceptance

- No command prints `CompileError`, `SForm(`, `Atom(name=`, `Loc(` or `S1.8.` on stderr for any corpus program,
  held by a test over the corpus and the witnesses, every command.
- Every witness is refused by every command with one first line and its load-error exit code, held by a test.
- Every exit that moved is a register row, `fixed`.
- `./run_tests.sh` green.
