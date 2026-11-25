# Q-M2.16 — Every saturation a question about R computes is bounded

A question about R bounds every saturation it computes, root's first and each node's after it, the load's
judgement of the hypothesis mechanisms and `--trace`'s root among them, by two bounds the run carries:

- **the clock**, `-T`: read between a route's steps, as it always was, and inside every saturation, every
  `saturator::CLOCK_EVERY` firings and at each quiescence. A run it stops is no answer, wherever it stops: one line on
  stderr, nothing on stdout, exit 2, a summary with no answer whose diagnostic is the deadline's
  ([Q-M2.9](../diagnostics/Q-M2.9-a-refusal-is-a-value-with-a-kind-a-place-and-its.md)), and `SolveError::Deadline`
  in the library; under `ein test` the file is an `ERROR`. `ein oracle`'s clock reaches inside its saturations too.
- **a cap on firings**, `--max-steps N` on `ein solve` and `ein test`, `explore::Options::max_steps`: the most
  firings, a redundant one counted, any one saturation of the run may make. Its default is 2^20
  (`explore::DEFAULT_MAX_STEPS`), far above every saturation a corpus program makes; it depends on no machine. A
  saturation that needs more stops the run, which is then **not decided**, as a run a budget stops is: `exhausted`
  *no*, or `--single`'s `verdict` *not decided* and `decided` *no*, with which saturation, root's or a node's, was
  stopped; a `note: … partial — …` on stderr; exit 0; the summary's `region.exhausted` false, `region.stopped`
  `{why: steps, steps, saturation}` and `verdict.word` `null`; and in the library `Stopped::Steps { cap, root }`.
  **Nothing of R is established**: no state, count or verdict, neither root's standing nor H, no query answered,
  no claim checked (NOT CHECKED, `ein test`'s exit 1 as past any budget), and the summary's `monotonicity` and `root`
  blocks are `null`.

A run whose every saturation reaches its fixpoint within the cap and before the clock is the run without them, byte
for byte. No new outcome is said: a stopped run is not decided, and a state whose saturation was stopped is never
called alive or dead. The other commands keep the saturation unbounded, as it is: `ein saturate` has its own
`--max-steps` with no default, `ein oracle` saturates every node of its cube to its fixpoint and takes no cap, and
`ein show hyps`, `ein render h-space`, `ein render kb` and `ein solve --monotone`, which read root's fixpoint and
answer no question about R, take neither bound; `ein render lattice` asks its describing question under the default
cap.

**Why.** A rule may build a fact out of a fact, so a saturation need not reach a fixpoint
([`02-saturation.md`](../../ein/reasoning/02-kb-evolution/02-saturation.md)), and before this nothing in a solve
stopped one: the clock was read only between a route's steps, and the budget counts facts beyond root, so root's own
saturation ran with no bound at all. A program that calls Ein, and a person, needs every question to end. The cap
is a count, so the run it stops says so on every machine alike, as a budget's stop does; the clock is the machine's,
so its stop stays an abort. A saturation stopped short of its fixpoint is a state that is not known, and every part
of R rests on states, so a stopped run claims nothing of R rather than a part of it. Neither bound changes what a
program means: each decides whether an answer is computed, never what it is
([Q-M1e2.30](Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)). A bound that holds by default on the
saturation itself, a word of the engine's own for *did not stop*, and a static check that a program's rules cannot
grow a term are not taken: the saturation, the outcomes and the language are as they were.

**Held by.** `ein_infer::saturator`'s `a_bound_ends_a_saturation_with_no_fixpoint` (the cap, the clock, a
saturation within its cap, a resumed saturation's bound on another thread); `ein-cli/tests/bounded_saturation.rs`:
`the_clock_ends_a_saturation_with_no_fixpoint`, `the_cap_ends_a_saturation_with_no_fixpoint` and
`the_library_says_each_stop`, on its two fixtures, a rule with no fixpoint at root and the same rule started by a
hypothesis, every question, each stop's lines, exit code and summary; `a_cap_a_run_does_not_reach_moves_nothing`
and `the_cap_is_one_firing_or_more`; `ein-cli/tests/lever_invariance.rs`'s
`no_config_flag_moves_a_line_of_any_mode`, which asks every question of every program whose R is known again with
no cap and a clock a day away, beside the flags; `ein-cli/tests/help_surface.rs` and `config_reference.rs`, the
option; `ein-cli/tests/read_out_reference.rs` and `read_out_examples.rs`, the rows, the fields and the examples
page's X22. The pages: [`05-cli.md`](../../ein/user/04-reference/05-cli.md),
[`06-config.md`](../../ein/user/04-reference/06-config.md),
[`08-read-out.md`](../../ein/user/04-reference/08-read-out.md),
[`02-saturation.md`](../../ein/reasoning/02-kb-evolution/02-saturation.md),
[`04-api.md`](../../ein/ein.rs/04-api.md).
