# Q-M1f.345 — Nothing in the project reads, links or names the work that produces it

The project is Ein as it ships: `README.md`, `ein.rs/` with its tests, `stdlib/`, `examples/`, `tests/`, `corpus/`,
and `docs/` but `docs/lib/`. The work on it is not part of it: its plans, its development tooling, its continuous
integration and the guidance its contributors work by. The work reads the project, which it sees in many states;
the project does not read the work. No page, comment, test, example or manifest note of the project reads a file of
the work, runs one, links to one, or names one. `docs/lib/`, the catalogue of other systems, and the plain-C
baselines and the natural-language prototype beside `ein.rs/` are neither, and nothing of the project depends on
them either.

**Why.** The work's files describe states the project passes through and are deleted when they are finished; a
project that read or cited them would break when they go, and would describe itself by the steps that built it
([Q-M1f.341](Q-M1f.341-the-project-documents-only-its-current-state.md)). Its code, comments, docs and decisions
are closed: each reference resolves inside the project. What the work decides about itself stays with it
([Q-M1f.346](Q-M1f.346-the-decisions-are-about-ein-not-about-how.md)).

**Held by.** The decisions' *Held by* lines, which name only the project's tests, code and pages
([Q-M1f.344](Q-M1f.344-each-decision-is-one-file-in-a-folder-by-subject.md)); a page cites a decision rather than
the work ([Q-M1f.343](Q-M1f.343-a-page-or-a-comment-cites-a-decision-by-its-id.md)), and a generated file names no
tool ([Q-M1f.348](Q-M1f.348-a-generated-file-says-it-is-generated-and-names.md)).
