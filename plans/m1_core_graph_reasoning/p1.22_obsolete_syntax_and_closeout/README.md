# P1.22 — Obsolete-syntax purge + M1 plans closeout

**Estimate:** ~2–4 days.
**Status:** open (created 2026-08-16).
**Depends on:** [P1.21](../p1.21_review_response/README.md) **complete**
(its improvements rewrite the same docs this phase sweeps, and its reports
must be final before the folder they live in is deleted).
**Blocks:** nothing — this is the terminal M1 phase; its last act removes
`plans/m1_core_graph_reasoning/` itself.

## Why this phase exists

Two closeout debts, by user decision (2026-08-16):

1. **`(type …)` and `(instance …)` are not valid Ein language.** The
   classic encoding's keywords are gone (S1.7.23 demoted them to ordinary
   facts; the flat-form grammar still *parses* them as undeclared-head
   facts, which is how they linger). The language is the **unified `is-a`
   model**: `(is-a a SomeType)`, relations declared via
   `(relation name T T)`, and `T` an ordinary top atom **requiring no
   declaration** ([`06_reserved_names.md:147`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)).
   Yet the classic forms survive across the repo: `examples/zebra.ein` (the
   whole classic encoding, incl. `:layer ontology` / `:source` kwargs),
   ~20 test files' fixtures, `cli/saturate.py` (prints `(type …)` /
   `(instance …)` summary lines), `inference/compile.py` / `ir/to_dot.py`
   comments, and doc prose. The removed block heads
   `(rules …)/(ontology …)/(facts …)` (P1.7c Track A) likewise linger in
   src docstrings/comments (`kb/store.py`, `kb/from_ir.py`, `render/*`,
   `trace/answer.py`, …). This phase removes the obsolete syntax **and its
   mentions** from code, comments, and documentation.
2. **M1 is shipped; its plans are scaffolding.** Per the repo's own
   convention ("the trail lives in commit history"), the milestone's plan
   folder is deleted at the end of this phase — *after* anything still
   live is rehomed and every inbound link is rewired.

## Structure — each point = two tasks

| ID | prio | stage | tasks |
|---|---|---|---|
| S1.22.1 | P0 | [Obsolete-syntax census → purge](s1.22.1_obsolete_syntax.md) | T1.22.1.1 census+report / T1.22.1.2 purge |
| S1.22.2 | P1 | [M1-plans preservation census → delete](s1.22.2_m1_plans_deletion.md) | T1.22.2.1 census+report / T1.22.2.2 migrate+delete |

**Strictly serial** (unlike P1.21): the deletion census (T1.22.2.1) must run
on the **post-purge** tree (the purge edits the very docs whose inbound
links it inventories), and the deletion itself is terminal. Order:
T1.22.1.1 → T1.22.1.2 → T1.22.2.1 → T1.22.2.2.

Reports land in [`reports/`](reports/) (they die with the folder in
T1.22.2.2 — by design; commit history keeps them).

## Hard constraints

- **Scope of "m1 plans" = `plans/m1_core_graph_reasoning/` only.**
  `plans/m1a_rust/`, `plans/m1b_gui/` are *future* milestones — kept
  (their links into the deleted folder get rewired/dropped).
- **Live backlog must not die with the history.** Known live content
  inside the folder: the **P1.9 E-catalog** (E1–E24), the **P1.21 parked
  follow-up stages S1.21.7/S1.21.8** (ex-E25/E26, moved in-phase
  2026-08-17; cross-linked from `frontier.py` and the kernel docs), the
  **P1.7c Track B** refactor-debt tail, the **P1.21 divergences** (D3, D5
  or-disjunct unsound firing, the `landed` NameError), and the M1
  [`open_questions.md`](../open_questions.md) still-open rows (Q26/Q28).
  T1.22.2.1 decides the surviving home (recommend: `plans/followups/` or a
  new `plans/backlog/`); T1.22.2.2 migrates before deleting.
- **Behaviour discipline**: `./run_tests.sh` + `ruff check .` green after
  each improvement; acceptance verdicts/bindings unchanged. Deleting
  `examples/zebra.ein` (if the census so recommends — the user has ruled
  its syntax invalid) must take its test/bench/doc references with it in
  the same change.
- Docs keep **no nostalgia**: obsolete forms are removed, not explained —
  the only sanctioned mention is a single line in the grammar doc's
  history note, if the census finds one is already there.

## Acceptance

1. **Zero obsolete syntax**: repo-wide grep (excluding `.git` and commit
   history) finds no `(type …)`/`(instance …)` Ein forms, no
   `(rules …)/(ontology …)/(facts …)` block-head references in code
   comments/docstrings/docs, no classic-encoding leftovers (`:layer` /
   `:source` fact kwargs if the census rules them classic-only), in:
   `ein.py/src`, `ein.py/tests`, `ein.py/acceptance`, `examples/`,
   `docs/`, `utils/`, top-level `README.md`. (`Layer.ONTOLOGY` /
   `Layer.FACT` enum names are the *data model*, not syntax — kept.)
2. **`plans/m1_core_graph_reasoning/` no longer exists**; `git log`
   carries it. Live backlog relocated; every former inbound link
   (docs/kernel/**, tests' plan-path comments, `plans/README.md`,
   m1a/m1b/m2/m2b/m3/followups) rewired or dropped — repo-wide grep for
   `m1_core_graph_reasoning` is clean.
3. **Suite green**: `./run_tests.sh` + `ruff check .`; acceptance
   verdicts/bindings identical to pre-phase.
4. `plans/README.md` roadmap table reflects the closeout (M1 row points
   at commit history / the closeout commit, not a folder).

## Out of scope

- Any engine behaviour change (D5's fix stays a recorded divergence
  wherever T1.22.2.1 rehomes it).
- `nlp/`, `smt/` scratch areas; `plans/ideas/` (user's own ideas — never
  deleted); other milestones' folders.
