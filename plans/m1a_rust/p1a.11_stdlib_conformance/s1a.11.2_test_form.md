# S1a.11.2 — The `(test …)` form

**Phase:** P1a.11 (stdlib conformance)
**Estimate:** 3 days
**Depends on:** [S1a.11.1](s1a.11.1_what_the_stdlib_promises.md)
**Decides:** Q-M1a.19 (in-file expectations vs a sidecar), Q-M1a.20 (what an
expectation may say)

## Context

The user's shape: **a `test` head that replaces `query`**, so a file is either
a puzzle to solve or a program to check, and a `{solve,render,saturate,test}`
subcommand set.

That is a good shape and it needs three decisions before it is a form.

### Q-M1a.19 — in-file, or beside the file?

| | in-file `(test …)` | sidecar (`corpus.toml`, `.expect`) |
|---|---|---|
| grammar change | yes — parser, dumper, macro pass, AST walkers, `grammar.lark`, M2's GBNF | none |
| reads next to the rule | **yes** | no |
| survives a file being copied | **yes** | no |
| can a puzzle carry both a query and expectations? | needs a rule | trivially |
| reserved word cost | `test` leaves SYMBOL | none |

**The user asked for the in-file form and that is the recommendation**, for
the reason the table's third row names: an expectation that travels with the
program is one that cannot rot apart from it. The grammar cost is real and is
this stage's main work.

### Q-M1a.20 — what may an expectation say?

Deliberately small, and each entry has to be demanded by a rule from
[S1a.11.1](s1a.11.1_what_the_stdlib_promises.md):

- **`:derives (fact …)`** — the fact is in the saturated state;
- **`:absent (fact …)`** — it is not. This is the direction the
  disjunctive-prune bug lived in and it is not optional;
- **`:verdict Solution | Ambiguity | Contradiction`** — with `:k N` where the
  count is the point;
- **`:fires rule-name`** / **`:does-not-fire rule-name`** — which rule did the
  work, because "the right fact by the wrong route" is exactly what a stdlib
  test should catch.

Everything else waits for a rule that needs it.

### Does `test` replace `query`, or coexist?

Replacing is cleaner and matches the user's framing: a `test` program is
checked, a `query` program is solved. But `:verdict` expectations need the
search, and the search needs a query to have a goal. **Recommendation:** a
`(test …)` form *may* carry the query keys it needs (`:goal`,
`:hypothesis-relations`), so it subsumes `query` rather than sitting beside
it, and a file with both is a load error.

## Acceptance

- `grammar.lark` carries the form, and the change is reviewed as a
  cross-milestone edit — [M2](../../m2_nl_to_ir/README.md)'s GBNF lift reads
  that file, and a form the NL frontend can emit but not mean is a trap.
- Parse → dump → parse round-trips, like every other form
  ([`docs/kernel/ir/`](../../../docs/kernel/ir/)).
- A `(test …)` in a file run under `solve` is a **load error with a clear
  message**, not silence. The failure mode to design against is a file whose
  expectations nothing ever evaluates.
- `test` as a relation name in an existing program is either still legal or
  the breakage is enumerated. `grammar.lark`'s SYMBOL exclusion list is the
  mechanism; the corpus says whether anything is affected.
- The form is documented in `docs/kernel/ir/03-ein-lang/` alongside `query`.

## Tasks

### Task T1a.11.2.1 — Settle Q-M1a.19 and Q-M1a.20, in writing
### Task T1a.11.2.2 — Grammar and AST
### Task T1a.11.2.3 — Loader and validation

An expectation naming an undeclared relation, or a `:fires` naming a rule that
does not exist, is a **load error**. A test that silently checks nothing is
worse than no test.

### Task T1a.11.2.4 — Dumper, macro pass, renderers
### Task T1a.11.2.5 — Round-trip and error-message tests

## Notes

- The strongest argument for keeping the vocabulary small is that this phase
  is not building a test framework — it is stating what seven modules of rules
  promise. Every key that is not demanded by a rule in S1a.11.1's table is
  speculative surface.
