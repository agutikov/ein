# S1f.21.2 — the pages of the first part: *dead*, the contradiction, the key

**Phase:** [P1f.21](README.md), part 1 ([the order](README.md#the-order-two-parts))
**Status:** planned with the phase (2026-11-17), its plan written 2026-11-19; **cut in two the same day** on the
user's word (*"leave open facts and hypotheses to the second part of the phase and finishe everything else
before"*): this stage keeps the pages of the first part, and
[S1f.21.14](s1f.21.14_the_pages_of_open.md) has those of `open` and the hypotheses. **Not started.**
**Group:** [B](modifications.md#b-the-pages-todays-kernel-stated-as-rules), items 6, 7, 9 and 12, and item 10's
half on the contradiction.
**Waits on:** the builds of the first part, [S1f.21.8](s1f.21.8_rules_the_standard_library_states.md),
[S1f.21.10](s1f.21.10_the_kernels_prefix.md) and [S1f.21.11](s1f.21.11_kill_and_three_conventions.md). A page
states the kernel as built
([Q-M1f.341](../../../docs/decisions/docs/Q-M1f.341-the-project-documents-only-its-current-state.md)), so it is the
last stage of the first part.
**Writes:** pages of `docs/ein/reasoning/` and `docs/ein/user/`, their diagrams, the glossary, and the decisions
of the first part's rulings under `docs/decisions/`.

## What it does

| item | page | what |
|---|---|---|
| 6 | `09-cores.md` § A contradiction, a pointer from `01-firing.md` | the contradiction rule: the schema, its pattern graph, why it is a schema, the rules the standard library states for it, and that the kernel judges no pair by itself |
| 7 | `06-solution-semantics.md` § 1.5 | *dead* as a rule's doing: a state is dead where a rule that states `:ein inconsistent` fired; *alive* as its absence, and why that is read on a state and never inherited |
| 9 | `03-absent-and-naf.md` § Stratification | where the rule that ends a state stands among a program's rules |
| 10, its first half | the user's `06-constraints.md` and `05-rules.md` | the same in Ein, each a program with its answer: the contradiction rule for one relation and for one count; a rule that states the key |
| 12 | the pages that introduce `(false)` | that it is a proposition of the standard library's and no truth value, what infers it and what reads it |

And, with them:

- **The key on the pages that state a rule's form**: `01-static-ir/02-rewriting-rules.md` (a conclusion is facts, or
  a word addressed to the engine), the user's `05-rules.md`, `02-syntax.md` and the grammar page, the glossary's
  *dead*, *inconsistent*, *State*.
- **`std.core`**, the module every program gets, where the standard library's pages and the page on imports say what
  a program has without importing it.
- **Each ruling of the first part as a decision**, one file under its id: Q-M1f.399, Q-M1f.550, Q-M1f.551,
  Q-M1f.447 (the count check dropped, a relation overloaded by its count), Q-M1f.557 (the contradiction a module a
  program imports), Q-M1f.559, Q-M1f.563 and Q-M1f.565 where the stage that builds one has not written it, and the
  readings the stages took; a decision the part amends
  (Q-M1f.313, the agenda; Q-M1f.436, the kernel reads no theory's name) is rewritten in place.
- **The sentence S1f.21.6 left for the pages**: `06-zebra-walkthrough.md`'s *a candidate is dead alone when assuming
  it … saturates to `(false)`*.

## What must hold

- Each perspective in its languages: `reasoning/` has no Ein source, command or file path, so a rule there is its
  formula and its graph, and the Ein is the user's page.
- A diagram is a file with its picture, a formula is TeX, a folder's numbering has no gap.
- Every command a page shows is run, and its output is the page's.
- `utils/doc_audit.py --links --check --tracked`, `utils/check_doc_blocks.py`, `utils/render_diagrams.py --check`
  and `utils/check_release_voice.py` pass.

## Acceptance

Each item is on its page; `docs/ein/` says nothing of the first part that the engine no longer does (searched for
by the old words: the kernel's reading of `(false)` and of `not`, *a rule concludes a conjunction of literals* with
no key); the checks above.

## Record

Nothing yet.
