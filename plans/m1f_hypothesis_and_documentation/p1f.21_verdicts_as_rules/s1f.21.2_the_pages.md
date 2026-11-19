# S1f.21.2 — the pages

**Phase:** [P1f.21](README.md)
**Status:** planned with the phase (2026-11-17), its plan written 2026-11-19 on the user's word (*"plan sections
from this list: A, B, C, ... as stages"*); **not started**.
**Group:** [B](modifications.md#b-the-pages-todays-kernel-stated-as-rules), items 6 to 13.
**Waits on:** every build of the phase. A page states the kernel as built
([Q-M1f.341](../../../docs/decisions/docs/Q-M1f.341-the-project-documents-only-its-current-state.md)), so this is
the last stage before the close, and what it writes is fixed by what
[S1f.21.7](s1f.21.7_the_debts_term.md) to [S1f.21.13](s1f.21.13_the_search_and_rules.md) built and by the
rulings they ran on.
**Writes:** pages of `docs/ein/reasoning/02-kb-evolution/` and `docs/ein/user/03-syntax-and-semantics/`, their
diagrams, the glossary, and the decisions of the phase's rulings under `docs/decisions/`.

## What it does

The eight rows of [`modifications.md`](modifications.md#b-the-pages-todays-kernel-stated-as-rules), from
[`design.md` § What S1f.21.2 writes under each ruling](design.md#what-s1f212-writes-under-each-ruling), each
worded for the kernel as it then is:

| item | page | what |
|---|---|---|
| 6 | `09-cores.md` § A contradiction, a pointer from `01-firing.md` | the contradiction rule: the schema, its pattern graph, why it is a schema, and where the kernel's doing differs from firing it, or that it is the rule fired |
| 7 | `06-solution-semantics.md` § 1.5 | *dead* and *alive* as two rules, and why the second is read on a state and never inherited |
| 8 | `10-satisfiability.md` § 1 | *owed* and *discharged* as rules; **a conclusion may be a held fact exactly when it stays true in every larger state**, with the four words of [the overview § 1](overview.md#1-the-phase-in-one-sentence) |
| 9 | `03-absent-and-naf.md` § Stratification | the verdicts as the last strata of every program |
| 10 | the user's `06-constraints.md` and `08-obligations.md` | the same in Ein, each a program with its answer |
| 11 | wherever the pages define `(open R)` | the first instruction's remark, worded by the ruling of Q-M1f.390 |
| 12 | the pages that introduce `(false)` | that it is no truth value, and what reads it |
| 13 | the reasoning pages' definition of $R$ | $R$ with no search: every state a context, the oracle its brute-force reading |

And, with them:

- **What S1f.21.12 built, on the pages that say what a hypothesis is**: `04-hypothesis-generation.md`,
  `05-hypothesis-space.md`, the user's page on hypotheses, the knowledge-base page's *a fact, its negation, both,
  or neither*, which gains the third context.
- **Each ruling of the phase as a decision**, one file under its id: Q-M1f.390, Q-M1f.399, Q-M1f.447, Q-M1f.549,
  and the readings the stages took; a decision the phase amends (Q-M1f.313, the agenda; Q-M1d.2, an obligation)
  is rewritten in place.

## What must hold

- Each perspective in its languages: `reasoning/` has no Ein source, command or file path, so a rule there is its
  formula and its graph, and the Ein is the user's page (item 10).
- A diagram is a file with its picture, a formula is TeX, a folder's numbering has no gap.
- Every command a page shows is run, and its output is the page's.
- `utils/doc_audit.py --links --check --tracked`, `utils/check_doc_blocks.py`, `utils/render_diagrams.py --check`
  and `utils/check_release_voice.py` pass.

## Acceptance

Each item is on its page; `docs/ein/` says nothing the engine no longer does (the phase's rulings searched for by
their old words: *derives (false)*, `hrule` where it went, the kernel's reading of `not`); the checks above.

## Record

Nothing yet.
