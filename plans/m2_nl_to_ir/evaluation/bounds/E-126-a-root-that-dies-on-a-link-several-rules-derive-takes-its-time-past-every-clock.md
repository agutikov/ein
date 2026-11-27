# E-126 — A root that dies on a link several rules derive takes its time past every clock, unless the program sets a lever

**Kind:** limit
**Status:** closed by [S2.2e.3](../../p2.2e_kernel_fixes/s2.2e.3_clock_and_a_dead_root.md) (`3a5c162b`)
**Filed:** by S2.2.5, 2026-11-26, on `8c2037b0`
**Owner:** none; [E-113](E-113-a-dead-state-s-core-costs-minutes-on-a-link-re-derived-many.md)'s, of which it is the
case at root under every question
**Since P2.2's close (2026-11-27):** [S2.2e.3](../../p2.2e_kernel_fixes/s2.2e.3_clock_and_a_dead_root.md); [Q-M2.34](../../open_questions/kernel/Q-M2.34-does-einaf-s-kernel-call-set-the-lever-that-keeps-one-justification.md)

## The limit

**The model's Zebra program, dead at root, takes 72 seconds under `--single`, and the clock does not stop it.** The
program the model wrote through the three steps for the Zebra puzzle (S2.2.4's, made again by S2.2.5 byte for byte)
selects `std.slots`' partition and `std.algebra`'s `includes`, `symmetric`, `converse`, `top` and `meet` over the
link `same-house`, and names eleven houses for five places beside four kinds with no member (L-M2.23), so root dies (`std.slots.slot-no-fill`, the core naming
*Coffee is drunk in the green house.*, E-125). `ein solve program.ein --single -T 5` returns after 72 s, *Contradiction*,
its timing table saying *route 73 453 ms: root, H and every node visited: 1 saturation*; the same program with
`(config :record-alternative-justifications false)` added returns in 63 ms. `zebra3.1.ein`, the target shape, which
sets no lever and stays alive, takes 37 ms. **E-113 found the cost on the surfaces that print every dead state's core
(`--events`); here it is root's own saturation, under `--single` and `--no-search` alike, where no core is printed,
and past the clock**: what runs between two reads of the clock is few firings, each recording every justification of
a link several rules derive. EinAf's bounded call returns as late: each question's object, the load's judgement among it,
says 78 s, and `einaf from-text` on the puzzle took 291.8 s, of which the model's three calls 41.5 s and the rest the
judgements of step 2's and the complete program's, a bounded call and a root each.

## Evidence

`ein solve plans/m2_nl_to_ir/p2.2_formalizer/steps/runs/steps/zebra/program.ein --single -T 5 -t` (72 s) against the
same file with the `(config …)` line appended (63 ms); the live run's stderr, [`from_text/runs/library/zebra/stderr.txt`](../../p2.2_formalizer/from_text/runs/library/zebra/stderr.txt)
(*--no-search: inconsistent (root is dead, so every node is), 78.2 s*), and its replay, `einaf from-text
nl2smt/puzzles/zebra.txt --id zebra --answer-line zebra --replay
plans/m2_nl_to_ir/p2.2_formalizer/from_text/runs/library/zebra/generations.jsonl` (239.8 s);
[the feedback object § 4](../../../../docs/einaf/03-feedback.md#4-what-the-bounds-do-not-stop), its fourth row.

## What it bounds

A formalizer's first mistake on an attribute puzzle, a membership too many, gives a program whose every kernel call
costs over a minute and whose clock is no bound: the loop's budget of kernel time is not what it sets. The lever
changes which core a dead node reports and no state
([Q-M1e2.30](../../../../docs/decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)), and a
program may not hold a `(config …)` ([the contract § 1.4](../../../../docs/einaf/04-contract.md#14-what-a-program-may-not-hold)),
so whether EinAf's call sets it is the user's question,
[Q-M2.34](../../open_questions/kernel/Q-M2.34-does-einaf-s-kernel-call-set-the-lever-that-keeps-one-justification.md).

## Closed

By S2.2e.3, `3a5c162b`, 2026-11-27. **What ran past the clock was not root's saturation but root's core**: the program's
saturation makes its 22 969 firings in 0.1 s, every justification recorded, and the 72 s are the explanation's label
search over those justifications (`explain::contradiction_frontier`, 15 rounds of some 19 000 folds each, up to 24
million environments a round), which the node function takes at a dead root under every question and which read no
clock. The search now takes the deadline the node's saturation took and reads it every 4 096 units of its work, a
fact read, a justification folded, an environment made; past it the run is the clock's abort, as anywhere
([Q-M2.16](../../../../docs/decisions/answer/Q-M2.16-every-saturation-a-question-about-r-computes.md)). The search writes
nothing, so a stopped one leaves nothing behind. `ein solve program.ein -T 5` returns in 5.01 s on every question,
exit 2; EinAf's call returns at its clock, each question *not decided* with the deadline's diagnostic. With no clock
nothing moves: the same read-out in the same 72 s, and the corpus's core searches cost what they cost, with no clock
and with a far one ([the stage's record](../../p2.2e_kernel_fixes/s2.2e.3_clock_and_a_dead_root.md#record)). Held by
`ein-cli/tests/bounded_saturation.rs`' `the_clock_ends_a_dead_roots_core_search` and `ein-infer/tests/explain_semantics.rs`'
`a_deadline_stops_a_core_search_and_one_not_reached_moves_nothing`, on `ein-cli/tests/bounded/dead_link.ein`, the
shape's smallest program; and EinAf's `outcomes.rs`' `a_dead_roots_core_search_is_not_decided_at_the_kernels_clock`.

What is left is not the clock's: the search costs what it costs with no clock (E-113, F22); a question that prints no
core takes root's all the same, so `--single` on this program is *not decided* at any clock shorter than the search,
where it could answer at once ([E-130](../../p2.2e_kernel_fixes/evaluation/bounds/E-130-a-question-that-prints-no-core-takes-a-dead-roots-core.md));
and whether EinAf's call sets the lever is still [Q-M2.34](../../open_questions/kernel/Q-M2.34-does-einaf-s-kernel-call-set-the-lever-that-keeps-one-justification.md).
