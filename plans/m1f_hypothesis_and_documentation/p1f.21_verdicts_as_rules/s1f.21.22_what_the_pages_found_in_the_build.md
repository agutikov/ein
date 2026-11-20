# S1f.21.22 — what the pages found in the build

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-20 by the coordinator, under the user's word to run the phase to its end on defaults
([the README](README.md#the-word-to-run-to-the-end)); **not started**.
**Waits on:** [S1f.21.20](s1f.21.20_what_the_first_uses_of_open_found.md), which moves where the pass runs and
the check in flight. It runs beside [S1f.21.21](s1f.21.21_the_contradictions_cost.md), which writes other files.
**Writes:** `ein-cli/src/render.rs` and the DOT views of `ein-render`, `ein-infer/src/mechanisms.rs`, the
read-out's printer, `ein-infer/src/open.rs`, their tests; comments of the engine and of two library files, with
the library's manifest; the pages and goldens that move with what is drawn or printed.
**Not its files:** `saturator.rs`, `contradiction.rs`, `compile.rs`, `plan.rs`, `match_.rs`, `ein-core/src/kb.rs`,
which are S1f.21.21's.

## What it is

[S1f.21.14](s1f.21.14_the_pages_of_open.md) wrote the pages of the second part by running the binary against
each ruling, and found the binary short of two rulings and of two of its own contracts. This stage builds those.

## Tasks

1. **[V-72](findings.md#v-72--the-kb-and-constraints-renders-draw-no-open-fact-root-holds): a drawing of root draws its `open` facts.** `ein render kb` and
   `ein render constraints` run the pass at root as every command that shows a state does, and draw what it
   writes; the view without them is the one the user ruled (Q-M1f.572, Q-M1f.577). Root is a state that is
   printed, so the pass runs there under the ruling of Q-M1f.655. The user's drawing rules hold: no arrow to or
   from an arrow, a layered layout, and a test that checks the structure and not the layout.
2. **[V-73](findings.md#v-73--a-written-open-fact-is-not-judged-against-the-lists-and-a-closure): a written `(open …)` is judged as a rule's is.** The two lists and a closure
   agree with the `open` facts root holds (Q-M1f.578): one the program writes is among them, and the load error
   names where it is written. A corpus program the check newly refuses is the stage's to show, not to keep
   loading.
3. **[V-74](findings.md#v-74--a-querys-printed-claim-over-open-does-not-read-back): what the read-out prints as a claim reads back.** A goal over `open` is
   answered; its `as :expect` row prints a claim the parser takes, or no row. Which is the stage's default.
4. **[V-77](findings.md#v-77--the-hypothesis-found-in-flight-may-depend-on-the-count-of-workers): the hypothesis found in flight is one at any count of workers.** Run it first:
   a program flagged in flight, at one job and at several, the summary's `monotonicity.in_flight` and the
   warning's line compared. If they differ, the hypothesis named is the first in the units' order, as every
   pooled result is read back. `ein-render/tests/jobs_invariance.rs` holds it.
5. **[V-75](findings.md#v-75--comments-of-the-engine-and-the-library-in-words-the-part-retired): the comments in the part's words.** Each place the record of S1f.21.14 lists,
   reworded to what the engine does; a comment that says a form is refused that loads is corrected to the
   grammar. The library's two files change in comments alone, and its manifest is written again.

## What must hold

Every corpus answer as before; every mode against the oracle; no config option, lever or flag changes $R$; a
flagged program is claimed nothing of. `dot_wellformed.rs` on every view that moves; the shape golden's lines
that move are the two renders' and no other. A DOT writer moves, so the container's gate is part of done: the
coordinator runs it once on the phase's last commit.

## Record

Nothing yet.
