# S1f.21.15 — *forbid* becomes a warning

**Phase:** [P1f.21](README.md), part 1 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-19 on the user's ruling; **not started**: it starts on the user's word.
**Waits on:** no decision. It is ruled: [Q-M1f.564](../open_questions.md#q-m1f564--a-non-monotone-program-loads-with-a-warning-forbids-refusal-goes).
In order it runs before [S1f.21.11](s1f.21.11_kill_and_three_conventions.md), which then has no *forbid* to re-aim.
**Writes:** `ein-infer/src/forbid.rs` and what calls it; the read-out and the events; the tests of
`ein-infer/tests/refutation_under_absent.rs` and those that pin a refusal; `corpus/regressions.toml` and the
register; the decision Q-M1e2.5; `AGENTS.md` § The language; the diagnostics' § 2.7; the pages that say a program
is refused.

## The ruling

The user, 2026-11-19, verbatim: *"I tell you what I expect - it loads, ein print huge WARNING, and then we guarantee nothing about it's output. NO!  Q-M1e2.5 IS WRONG! What the hell this `forbid` is? Where did it came from?"*

**A non-monotone program loads, Ein prints a loud warning, and nothing is guaranteed of its output.** The refusal
at load of a program whose refutation could rest on an `absent` a commitment can still fill
([Q-M1e2.5](../../../docs/decisions/language/Q-M1e2.5-a-refutation-may-not-rest-on-an-absent.md)) is reversed.
Where it came from is in the ledger's entry.

## Task 1 — what loads, shown first

On the 20 corpus programs *forbid* refuses today (16 under `examples/ein-bugs/`, 2 under `examples/branching/`, 2
under `examples/lattice/`; re-taken by running `ein show hyps` over the corpus and reading the refusal's words),
with the refusal switched off in a scratch build:

1. **Each is a program the static check flags.** A route's licences, *dead is upward-closed* the first, are given
   up on a flagged program and its routes are the cube's. A program *forbid* refuses and the static check does not
   flag would be searched with licences that do not hold of it. Shown for each of the 20; where one is not flagged,
   the static check gains the site, and that is said before anything else is built.
2. **What each answers**, by `ein solve`, `ein oracle` and `ein test`, each agreeing with the others: one $R$ under
   the fork of root, as for any flagged program.
3. **The warning**, as the user will see it: its words, where it stands in the read-out, on stderr and in the JSON
   summary, on every command that today refuses. **Reading 1**, default: a block at the head of the read-out and one
   line on stderr, in capitals, naming each rule and the `absent` it rests on, with *nothing is claimed of this
   answer*; the summary carries it as a field. The other reading: the existing `lattice not monotone` rows, made
   louder.

## Task 2 — the build

1. The refusal goes; the check stays as what finds the sites the warning names.
2. The warning, as task 1.3 settles it, on every command.
3. The decision Q-M1e2.5 rewritten to the ruling; `AGENTS.md`'s rule on *forbid*; the diagnostics' § 2.7, which
   becomes a warning's row; every page that says such a program is refused.
4. **The rows whose right answer is the refusal.** A row of `corpus/regressions.toml` says *right: refused* of
   several of the 20. **Reading 2**, default: each goes back to the register of wrong answers as an open row,
   owned by M1n, as the other rows of non-monotone programs are, its witness kept; the engine claims nothing of
   them, so no row states a right answer. The other reading: the rows are dropped.
5. The messages elsewhere that tell a user to rewrite a program so that it is not refused (the remedies *forbid*
   named: close the relation, state an obligation, conclude from a stored negative) become the warning's advice.

## What must hold

- **Every answer of a program that loads today**: $R$ of every corpus program that is not one of the 20, by the
  corpus sweep and the oracle; `ein-cli/tests/lever_invariance.rs`.
- **No licence on a flagged program**: task 1.1, held by a test over the 20.
- **One answer**: every command gives the same $R$ of each of the 20.
- **Nothing is claimed**: no page, read-out or test says an answer of a flagged program is right, sound or complete.
- The goldens move for the 20 and where a refusal's line was pinned; the diff is read.
- A large change to what loads: the gate and the nightly on the stage's last commit.

## Acceptance

No command refuses a program for an `absent`; each of the 20 loads and prints the warning; the oracle agrees with
`ein solve` on each; `python3 utils/wrong_answers.py` holds the re-filed rows.

## Record

Nothing yet.
