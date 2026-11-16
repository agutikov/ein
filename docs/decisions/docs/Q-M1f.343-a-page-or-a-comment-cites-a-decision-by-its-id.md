# Q-M1f.343 — A page or a comment cites a decision by its id, never a plan or a date

A page cites a decision as a link to its file under `docs/decisions/`. A code comment, a doc comment, an `.ein`
comment and a note of the corpus manifest cite it by its bare id, `Q-M1e2.24`, which names its file,
`docs/decisions/<subject>/Q-M1e2.24-<slug>.md`. None of them cites a plan of the work, a step of it, or the date on
which something was decided, built or measured.

**Why.** A decision's id outlives the work that took it
([Q-M1f.342](Q-M1f.342-a-decision-is-a-question-with-its-final-answer.md)); a plan and a date name the work, which is
not part of the project and is gone when it is done
([Q-M1f.345](Q-M1f.345-nothing-in-the-project-reads-links-or-names.md)). A reader who meets an id finds the answer
in one step, by the file name alone.

**Held by.** The file names under `docs/decisions/`, each `<id>-<slug>.md`, so that an id resolves to exactly one
file; [`docs/decisions/README.md`](../README.md), which states how a decision is cited.
