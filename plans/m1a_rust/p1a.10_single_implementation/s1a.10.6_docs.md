# S1a.10.6 — The docs after the oracle

**Phase:** P1a.10 (One implementation)
**Estimate:** 2 days
**Depends on:** [S1a.10.5](s1a.10.5_removal.md)

## Context

Three documentation trees describe the repo, and each is wrong in a different
way once the Python engine is gone:

- **`docs/api/`** is *the Python embedding contract* — `parse` →
  `KnowledgeBase` → `solve` → verdict. Its subject moves from ein.py to
  [P1a.9](../p1a.9_bindings_release/README.md)'s PyO3 module, and **that
  module does not exist yet**: the phase dependency was reversed on
  2026-08-21, so P1a.9 runs *after* this one. This stage therefore cannot
  give `docs/api/` a subject. What it must do instead is say so **on the
  pages themselves** — a documented API that quietly names a dead module is
  the failure mode; one that names the stage where its implementation lands
  ([S1a.9.1](../p1a.9_bindings_release/s1a.9.1_pyo3_surface.md), documented
  by [S1a.9.4](../p1a.9_bindings_release/s1a.9.4_documentation.md)) is a
  plan. The contract itself is meant to survive unchanged; that is what
  [S1a.9.2](../p1a.9_bindings_release/s1a.9.2_api_parity_tests.md) is for.
- **`docs/kernel/`** is the specification ein.rs implements, and it is now the
  *only* statement of intent that is not also the implementation. It gets more
  load-bearing. `docs/kernel/inference/python_impl.md` is the exception — it
  describes the Python engine's internals and has no subject.
- **`CLAUDE.md` / `AGENTS.md`** describe a two-implementation repo in almost
  every section.

## Acceptance

- `docs/api/` no longer describes a module that can be imported from this
  repo, and **every page says which stage gives it one** rather than reading
  as current. Describing the PyO3 surface, and verifying the worked example
  against it, is
  [S1a.9.4](../p1a.9_bindings_release/s1a.9.4_documentation.md)'s — this
  stage's job is that no page is *false* in the interval.
- `docs/kernel/` contains no claim that rests on "ein.py does X" as evidence.
  Where the Python implementation *was* the specification of a quirk — the
  `%ignore` delayed-match parse-error positions
  ([Q-M1a.3](../open_questions.md#q-m1a3--parse-error-message-parity)),
  `sorted()` over mixed-type args ([D2](../divergences.md)) — the quirk is now
  **ein.rs's own defined behaviour** and has to be *stated*, not referenced.
  This is the substantive half of the stage.
- `CLAUDE.md` describes the tree that exists: no `ein.py/`, no
  `nlp/`/`smt/`, one engine, one gate.
- `docs/guide/` and `docs/lib/` re-checked for invocations that assumed the
  Python CLI.
- The milestone's own documents keep their history: **P1a.0–P1a.9 were
  written against an oracle and their numbers are real**. They are not
  rewritten, they are read as history — and where a document's *instrument*
  is gone, a line says so.

## Tasks

### Task T1a.10.6.1 — `docs/api/` re-pointed
### Task T1a.10.6.2 — `python_impl.md` retired

Deleted, or reduced to a historical note next to the divergence ledger.
Whichever, `docs/kernel/README.md`'s orientation must stop sending readers to
it.

### Task T1a.10.6.3 — The quirks, restated as ein.rs's own

The list is short and known: it is exactly what
[divergences.md](../divergences.md) and the resolved `Q-M1a.*` entries
already enumerate. Each becomes a paragraph in `docs/kernel/` saying what the
engine does and why, with the Python provenance as an aside rather than as the
definition.

### Task T1a.10.6.4 — `CLAUDE.md` / `AGENTS.md`
### Task T1a.10.6.5 — The instrument sweep

Every measured claim in `plans/m1a_rust/**` whose instrument no longer exists
gets a one-line marker. Not a rewrite — the numbers were true — but a reader
must be able to tell "re-measurable" from "frozen".

## Notes

- The most valuable thing this stage produces is T1a.10.6.3. A quirk that was
  only ever defined as "whatever ein.py did" becomes undefined the moment
  ein.py is gone, and undefined behaviour in a *specification* repo is worse
  than a quirk.
