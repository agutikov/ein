# S1h.2.5 — The cut as an object: an engine stage

**Phase:** [P1h.2](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13, **in outline.** Not started. **An engine stage.** It is past the phase's
stop: **its tasks are written at [S1h.2.4](s1h.2.4_what_used_means_and_the_stop.md)'s re-plan,** to the
user's rulings, and nothing below designs it.
**Depends on:** the rulings on [Q-M1h.11](../open_questions.md#q-m1h11--what-used-means) (what *used* means)
and [Q-M1h.12](../open_questions.md#q-m1h12--is-a-restriction-that-was-tested-and-held-recorded) (whether a
tested restriction is recorded).

## What it rests on

The user, 2026-11-12: *"question and inference of the answer cuts the program (which is a model of a
problem) into 2 parts used/unused, required/superfluous."* The milestone README: *What does not exist: the
cut as an object, of the **program** (its facts, its rules, its imported modules' rules) and not of the
trace, printed and dumped; and its complement, the part the answer did not touch.*

## What it builds

The cut of a loaded program by one answer, as a value the library returns and no surface prints yet: for the
facts that answer a query in a state, the given facts used, the rules used with the module each is declared
in, the commitment's own facts, and the complement of each, by the definition of *used* the user ruled. And,
if Q-M1h.12 was ruled that way, the restrictions that were tested and held, by the record the user chose.
It sits beside what already walks provenance: the core's computation in `ein-infer` and the trace's
relevance walk in `ein-render`.

## What it carries, whatever is ruled

[The phase's discipline](README.md#the-discipline), and of it here:

- **Held to the script.** A test holds the engine's cut to
  [S1h.2.1](s1h.2.1_what_a_slice_is_today.md)'s `slice.py` on every program of the set both reach, as the
  routes over R are held to `ein oracle`. The script moves to `utils/` in this stage if the test names it.
- **Taken only where asked.** No route, mode or surface that exists today computes a cut; a sweep of the old
  binary against the new shows every surface of every corpus program byte-identical, since nothing asks for
  one yet.
- **No answer moves:** R, N, H, every claim's outcome, every exit code.
- **Its cost measured** on the set and on the largest program a surface would be asked on, through
  `utils/bench_env.sh`.
- **If a rule's module is not kept by the loaded program** ([S1h.2.1 § What it may find](s1h.2.1_what_a_slice_is_today.md#what-it-may-find)),
  the stage keeps it, and says what else reads it.

## Acceptance

Written with the tasks. At the least: the test against the script is green; the sweep shows no surface
moved; `./run_tests.sh`, then `./run_tests.sh --nightly`, green on the commit.

## What it must not do

- **Not print anything.** The surfaces are [S1h.2.6](s1h.2.6_the_cut_printed_and_dumped.md)'s.
- **Not take a cut on a run that did not ask.**
- **Not define *used* otherwise than ruled,** and not silently fix the script to agree with the engine: a
  disagreement between the two is a finding with its witness.
