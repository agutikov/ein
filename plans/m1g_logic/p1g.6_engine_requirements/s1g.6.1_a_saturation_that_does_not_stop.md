# S1g.6.1 — A saturation that does not stop: a bound, a word, and what a run then says

**Phase:** [P1g.6](README.md) — candidate C-1
**Status:** planned 2026-11-11, not started. Provisional until P1g.1's stop.
**Depends on:** P1g.1's rulings; [S1g.4.5](../p1g.4_first_order_logic/s1g.4.5_the_boundary.md)'s
decidability table tells it what can be checked statically, and may run after it.
**Pulls in:** [F26](../../followups/f26_consistency_and_completeness.md)'s promotion 3: *"The engine's word
for did not stop: a saturation bound and a fifth outcome, or a static check that a program's rules can only
shrink or keep a term's depth. It is the one gap here that a real program can meet."*
**Stops for the user:** its first task ([Q-M1g.6](../open_questions.md#q-m1g6--the-word-for-a-saturation-that-does-not-stop)).

## The witness

A sketch; it is probe 3's program, run once on the release binary at `8860d616` (2026-11-11) with
`timeout 20 ein saturate FILE`: the run was at 8 388 505 facts when the timeout ended it, printing a progress
line every 500 steps and nothing else.

```lisp
;; sketch: three lines of rule and a saturation with no end
(relation s Nat   :why "the successor of {?1}")
(relation nat Nat :why "{?1} is a natural number")
(nat z)
(rule succ ()
  :match (nat ?n) :assert (nat (s ?n))
  :why   "the successor of {?n} is a number")
```

It is not exotic. It is Peano's second axiom written the obvious way, and every theory of this milestone
past P1g.2 is one rule away from it. Before nested facts a program's derivable facts were bounded by its
names; now they are not, and *does root derive `(false)`* is undecidable in general (F26).

## What has to be decided

1. **What is bounded.** Facts in the store, firings, the depth of a derived term, or memory. Each stops
   probe 3; they differ on programs that are large and finite (a calculator batch of a million requests is
   many facts and no divergence) and on programs that diverge slowly. A depth bound has a natural value the
   engine already has: the parser's nesting limit of 1 000 (C-4), past which a derived fact cannot be
   written back anyway. **To be measured** on the corpus: the largest count of each kind any tracked program
   reaches today, so a default sits far above all of them and moves no answer.
2. **Whether there is a default.** With none, the engine keeps today's behaviour unless asked; with one,
   every run is protected and some future program meets the bound unasked.
3. **What a run says.** A state whose saturation was cut is **not** alive and not dead: it is not known. So
   it is none of R's members, none of N's, and `exhausted` cannot be *yes*. The read-out needs a row and a
   word; the candidates are in the spirit of the existing ones (*`Open — owes n`* separated *no model* from
   *not yet a model*; this separates *alive* from *not known to be alive*). Every surface that prints a
   state's status carries it: the read-out, `--json-summary`, `--dump-answer`, `--trace`, `ein saturate
   --dump`, `ein test` (a claim against a cut run is `NOT CHECKED`, as a claim past the region budget is),
   and `ein oracle`.
4. **The exit code.** The user's ruling of 2026-10-16: every answer exits 0 and only an error does not. A cut
   saturation at root answers nothing about R, and is not an error in the input either. The recommendation
   the stage brings is the user's to accept: 0, with the word, as a run that stops at `--region-budget` is 0
   today.
5. **Is it config?** A bound that changes whether an answer is *computed* does not change what the answer
   *is*, as the region budget does not; `lever_invariance` holds that reading for the budget. The bound must
   meet the same test: under any bound large enough for a program to finish, every surface is byte-identical
   to an unbounded run.
6. **Fairness.** If a diverging rule starves a rule that would derive `(false)`, a cut state that is in fact
   dead reads *not known*, which is sound, and the bound's value decides how often that happens. S1g.4.5's
   fairness program is the test.
7. **The static side.** A cheap check that *cannot* diverge is worth having beside the bound: no rule
   concludes a term deeper than its premises bind. It accepts every zebra program and says so in one row;
   it rejects the calculator, which terminates for a reason the check cannot see. So it is a diagnostic in
   the read-out, like the strata row since S1f.5.15, never a refusal.

## Tasks

1. **T1g.6.1.1 — the page for the user.** The witness and three more (a Skolem father who is a person; the
   witness search of [S1g.5.4](../p1g.5_peano_and_induction/s1g.5.4_goedel_made_concrete.md); a diverging
   rule beside a reachable contradiction), each run on a prototype of each kind of bound; the corpus's
   largest counts; a mock of every surface's wording, written by the prototype and not by hand; the options
   for points 1 to 5, with a recommendation each.
2. **T1g.6.1.2 — the build,** as ruled: the bound in the saturator, the status carried to every surface,
   `ein oracle` held to it.
3. **T1g.6.1.3 — the tests:** each witness reports the word under every command, deterministically (the
   same cut on every run, every `--jobs`, every id permutation: the bound counts something the engine
   defines, never time); `lever_invariance` over the bound; the corpus unmoved, shown by a sweep of every
   surface before and after.
4. **T1g.6.1.4 — the diagnostic** of point 7, and its row in the read-out reference.
5. **T1g.6.1.5 — the pages:** `defined_behaviour.md`, `read_out.md` (held by `read_out_reference.rs`),
   `configuration.md`, `solution_semantics.md` (a third status beside alive and dead), `events.md`; and
   F26's promotion 3 marked done.

## Acceptance

- The user has ruled on what is bounded, the default, the word and the exit code.
- Each witness ends, says the word on every surface, and says the same thing on every run.
- No corpus program's output moves on any surface (the sweep's command and date on the page); the bound is
  invisible to every program that finishes.
- A cut state is never counted in R or N, and `exhausted` is never *yes* on a run that cut one, held by a
  test.
- The theories of P1g.3 to P1g.5 that ran under an external timeout run under the bound instead.

## What it may find

- **That a time-free bound which protects every case does not exist** (a rule that derives few facts very
  slowly). The page says which case is left to the caller's timeout.
- **That a cut at a non-root node needs more than a word:** R is then known only in part, which the read-out
  already has a shape for past `--region-limit`. Reusing that shape is the expected answer.
