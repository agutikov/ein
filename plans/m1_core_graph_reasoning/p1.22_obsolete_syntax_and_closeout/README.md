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

## Structure

| ID | prio | stage | tasks |
|---|---|---|---|
| S1.22.0 | **P0** ✅ | [Boundary verification debt: completeness + state parity](s1.22.0_boundary_verification.md) | T1.22.0.1 attack+report / T1.22.0.2 fix+pin |
| S1.22.1 | P0 ✅ | [Obsolete-syntax census → purge](s1.22.1_obsolete_syntax.md) | T1.22.1.1 census+report / T1.22.1.2 purge |
| S1.22.1b | **P0** ✅ | [Cross-layer contradiction bug; remove knowledge layers](s1.22.1b_layer_removal.md) | T1.22.1b.1 census+report / T1.22.1b.2 fix+pin / T1.22.1b.3 remove — **shipped 2026-08-17** |
| S1.22.1a | P1 ✅ | [`zebra.ein`: modernise and make it solve](s1.22.1a_zebra_ein_modernisation.md) | T1.22.1a.1 investigate+report / T1.22.1a.2 execute |
| S1.22.3 | P1 | [Relation-signature semantics: document the kernel/userspace split](s1.22.3_relation_signature_semantics.md) | T1.22.3.1 docs pass |
| S1.22.4 | P1 | [`relation` as a kernel word: decide and rehome](s1.22.4_relation_kernel_word.md) | T1.22.4.1 decide+park |
| S1.22.99 | P1 | [M1-plans preservation census → delete](s1.22.99_m1_plans_deletion.md) | T1.22.99.1 census+report / T1.22.99.2 migrate+delete |

**Why `99`** (renumbered from S1.22.2, 2026-08-17): this stage deletes the
folder every other stage documents itself in, so it is **terminal by
construction** — anything scheduled later would have nowhere to land. The
number reserves the whole `S1.22.2`…`S1.22.98` range for work that must
precede it, so a new stage can be inserted without renumbering the deletion
again.

**Strictly serial from S1.22.1 on**: the deletion census (T1.22.99.1) must
run on the **post-purge** tree (the purge edits the very docs whose inbound
links it inventories), and the deletion itself is terminal. Order:
T1.22.1.1 → T1.22.1.2 → **S1.22.1b** → S1.22.1a → S1.22.3 → S1.22.4 →
T1.22.99.1 → T1.22.99.2.

**S1.22.3 / S1.22.4 slot into the reserved range** (added 2026-08-17, from
the root-`TODO.md` scratchpad block on relation-signature semantics —
quoted in full in S1.22.3, pruned from the scratchpad): the docs pass
(S1.22.3) edits `docs/kernel/` and so must precede the deletion census;
the decision stage (S1.22.4) consumes that pass's census, and — per this
README's own out-of-scope rule — may *park* engine work but not execute
it. `S1.22.2` stays unused: it was the deletion stage's number before the
S1.22.99 renumbering, and reusing it would collide with older commit
messages.

**S1.22.1b precedes S1.22.1a** (added 2026-08-17, by user ruling): it changes
what the search kills, so running it after `zebra.ein`'s solvability work
would invalidate that stage's tuning — and it removes the four `:layer`
lines S1.22.1a would otherwise carry into its rewrite.

**S1.22.0 is off that chain** and runs in parallel with the purge — it
touches `ein.py/src/ein/inference/` and its tests, not fixtures or docs, so
the doc-link constraint that forces the serial order does not apply. It
must, however, **finish before T1.22.99.2**: it is unfinished verification of
shipped engine code, and its findings are recorded in the P1.21 README that
the deletion removes.

Reports land in [`reports/`](reports/) (they die with the folder in
T1.22.99.2 — by design; commit history keeps them).

## Hard constraints

- **Scope of "m1 plans" = `plans/m1_core_graph_reasoning/` only.**
  `plans/m1a_rust/`, `plans/m1b_gui/` are *future* milestones — kept
  (their links into the deleted folder get rewired/dropped).
- **Live backlog must not die with the history.** Known live content
  inside the folder: the **P1.9 E-catalog** (E1–E24), the **P1.7c Track B**
  refactor-debt tail, and the M1
  [`open_questions.md`](../open_questions.md) still-open rows (Q26/Q28).
  T1.22.99.1 decides the surviving home (recommend: `plans/followups/` or a
  new `plans/backlog/`); T1.22.99.2 migrates before deleting.

  *Updated 2026-08-17:* S1.21.7 / S1.21.8 are no longer parked backlog —
  both **executed**, so they migrate as *record*, not as work. All five
  P1.21 divergences are closed (D3/D5 by S1.21.8, D-R5-1 by its fix, and
  D-S8-1/D-S8-2 — the two the follow-up stages introduced — by `95b3d36`).
  What is still live from P1.21 is the **unfinished verification** those
  divergences came out of, now [S1.22.0](s1.22.0_boundary_verification.md);
  its findings land in the P1.21 README, so T1.22.99.1 must census that file
  *after* S1.22.0 reports.
- **Behaviour discipline**: `./run_tests.sh` + `ruff check .` green after
  each improvement; acceptance verdicts/bindings unchanged. **Exception —
  [S1.22.1b](s1.22.1b_layer_removal.md)**: it is a deliberate soundness fix,
  so the *contradiction count* rises by design. Verdicts and bindings still
  may not move. (`zebra.ein` is
  **not** deleted — the user's 2026-08-17 clarification withdrew the
  invalid-syntax premise; it is rewritten by
  [S1.22.1a](s1.22.1a_zebra_ein_modernisation.md).)
- Docs keep **no nostalgia**: obsolete forms are removed, not explained —
  the only sanctioned mention is a single line in the grammar doc's
  history note, if the census finds one is already there.

## Acceptance

1. **Zero obsolete syntax**: repo-wide grep (excluding `.git` and commit
   history) finds no `(type …)`/`(instance …)` Ein forms, no
   `(rules …)/(ontology …)/(facts …)` block-head references in code
   comments/docstrings/docs, no classic-encoding leftovers (`:source`
   fact kwargs if the census rules them classic-only), in:
   `ein.py/src`, `ein.py/tests`, `ein.py/acceptance`, `examples/`,
   `docs/`, `utils/`, top-level `README.md`. (`:layer` and the `Layer`
   enum are **already gone** —
   [S1.22.1b](s1.22.1b_layer_removal.md) removed both; the only
   surviving `layer` in the tree is the *lattice* one.)
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
  wherever T1.22.99.1 rehomes it).
- `nlp/`, `smt/` scratch areas; `plans/ideas/` (user's own ideas — never
  deleted); other milestones' folders.
