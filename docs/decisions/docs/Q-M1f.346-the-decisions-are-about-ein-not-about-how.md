# Q-M1f.346 — The decisions are about Ein, not about how the work on it is done

`docs/decisions/` holds the decisions about Ein: its language, what it answers, its commands, its engine, its
library, its corpus and its documentation. A decision about how the work on Ein is done is the work's and stays
with it ([Q-M1f.345](Q-M1f.345-nothing-in-the-project-reads-links-or-names.md)): which part of the work takes a
finding, what a check of the development runs and on what budget or schedule, how a wrong answer is tracked while
the engine still gives it, and what a development script does.

**Why.** Such a decision is true of the work while it runs and is gone with it. A reader of Ein needs what Ein does
and why; the way it was scheduled and checked is the work's history
([Q-M1f.341](Q-M1f.341-the-project-documents-only-its-current-state.md)).

**Held by.** [`docs/decisions/README.md`](../README.md), whose subjects are the project's, and each decision's *Held
by*, which names only the project's tests, code and pages.
