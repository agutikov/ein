# S1f.23.3 — `--diff-models` leaves `ein solve`; `ein test` shows how a claim's models differ from what it found

**Phase:** [P1f.23](README.md)
**Status:** planned 2026-11-18 on the user's word, and run after S1f.23.2 is merged (both edit `models.rs`,
`modes.rs`, `cmdline.rs`, `utils/review/` and the read-out's pages); in a worktree if S1f.23.1 is still writing on
master.

## The ruling

The user, 2026-11-18, after the main session's look at `--diff-models` (it compares the states a run lists with
models written in a file of its own format, `closes …` and `model` lines around Ein facts; only `utils/review_bundle.py`
uses it, for the wrong-answer register's case pages; the language already states an expected answer as an `:expect`
claim that `ein test` checks; what only it has is the near-miss, *this state is one fact off that expected model*),
verbatim:

> Yes I agree - remove --diff-models as it is now. But near-miss diagnosis: "this state is one fact off that expected
> model" looks useful. can we add it as an option for test ? when ein test see expected models and solution result it
> can diff them (under option) and print stats and/or diffs

So:

1. **`--diff-models` goes from `ein solve`** (and every command that has it), with its file format, its form in
   `--dump-answer`'s `forms/`, its pages and decisions.
2. **`ein test` gains an option** under which, for every claim that states models (`(solve … :expect (model …))`,
   `(region (model …) … (owing (model …)))`, and a query's claim where it lists answers), it compares the models the
   claim states with the states the run found, and prints **stats** (how many match, are missing, are extra, and are
   near, with how many facts off) and/or **the diffs** (for each missing or near model, the facts it lacks and has
   beyond the state nearest it, `-`/`+`, by the one fact printer).

## Readings (defaults; put in `open_questions.md` with the next free ids, and shown on real programs)

- **The option**: one flag with two levels, `--diff stats` and `--diff full` (a bare `--diff` the stats); a name the
  CLI does not use already. It changes no verdict and no exit code: it explains, after each claim's line.
- **What is compared**: what the claim's own check compares (a claim is about what it names,
  [`13-solutions-and-models.md`](../../../docs/ein/user/03-syntax-and-semantics/13-solutions-and-models.md)), the same
  set of facts by the same printer, so a held claim's diff is all matches and a FAILED one's says why.
- **Near**: an unmatched expected model is paired with the found state of fewest differing facts, ties by the
  states' order; *one fact off* is the case the user named, and the stats count each distance.
- **Where it prints**: on stdout after the claim's line, within `ein test`'s output; `--json-report` gains the same
  under the claim, its schema's id moved.
- **What `--diff-models`' diff code becomes**: the matching and the near-miss move to serve `ein test`
  (`models.rs`' `diff`, `near`, `missing`, `spurious`), and the file format (`Expected`, `parse_expected`, `closes`)
  goes.
- **The wrong-answer review bundle** (`utils/review/case.py`, `run.py`), the one user of `--diff-models`: it writes
  the register's right answer as the claim it is, into a copy of the case's program, and runs `ein test` with the
  option; its smoke in the gate stays green.

## Acceptance

- Every claim's verdict and every exit code as before, with and without the option (a test over `tests/` and the
  corpus's claims).
- On real programs: a held claim (all match), a FAILED claim one fact off, a claim with a missing and an extra state;
  their outputs in the record.
- The pages: `05-cli.md`, `06-config.md` (regenerated), `07-diagnostics.md`, `08-read-out.md` (§ 8 without
  `--diff-models`), `10-answer-dump.md`, `ein test`'s page; the decisions that state `--diff-models` (Q-M1e2.34,
  Q-M1f.255 and any other) rewritten.
- The checks of S1f.23.2's acceptance; the gate, `utils/ci_container.sh` and the nightly once, by the main session,
  with the phase's other stages' code.

## Record

Not started.
