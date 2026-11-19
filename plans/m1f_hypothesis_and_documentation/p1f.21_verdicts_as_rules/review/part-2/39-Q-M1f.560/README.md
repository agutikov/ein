# 39 · Q-M1f.560: `std.debts`, with its stage or with the second part

[← the topic](../README.md) · [← the phase](../../../README.md) ·
[the ledger](../../../../open_questions.md#q-m1f560--stddebts-with-its-stage-or-with-the-second-part)

**Not ruled; held, which is the default.** It was put when you cut the phase in two and S1f.21.8 was still
running; that stage reported before an answer, and the module was held.

**The question.** S1f.21.8 built three modules. Two are merged, `std.negation` and `std.contradiction`. The third,
`std.debts`, writes what an obligation requires and what pays it as facts, and `(debts-discharged)` where nothing
required is unpaid: it is the `open` side, which you left for the second part of the phase. It sits on the stage's
branch, `worktree-agent-a8ead811ae8dbe86c`, one commit (`6b76c164`), built and tested: 13 rules, 10 programs under
`tests/stdlib/debts/`, a rules page, a test of its own (`debts_rules.rs`).

## What it does

The build of the stage's branch (`build/review-bin/ein-s1f218-9e40adbd`, with the branch's `stdlib/`), on
[`programs/seats-debts.ein`](programs/seats-debts.ein), the seats program with `(import std.debts)` and
`(debts-of-owed)`; `tools/run.sh` re-takes it into `runs/` (both ignored).

- $R$ is the seats program's: 2 states, `lattice monotone (the static check)`, 10 saturations.
- Both states hold `(debts-discharged)` and four facts of `debt-paid`, one a requirement:
  `(debt-paid seats (total-owed-of seats is-a Guest Chair Ann))` and its three siblings.
- The stage's sweeps: with the module imported and asked for, the unpaid requirements are the kernel's `owes` at all
  16 nodes of this program and at all 120 nodes `ein solve` enters on `examples/zebra2-obligations.ein`; imported
  and not asked for, it moves no corpus answer that importing `std.algebra` and `std.slots` alone does not move.

It decides neither of the second part's questions. It is what Q-M1f.390's option (c) would have the kernel make,
written as rules a program asks for.

## The options

- **(a) Held for the second part**, as now. It is read again there, under the rulings of Q-M1f.390 and Q-M1f.549,
  its names (`debt-due`, `debt-paid`, `debts-discharged`, `debts-of-owed`) with it. Merging it later means writing
  the manifest, its rules page and the goldens again on the tree as it then is. **Default**: it is what you said of
  the `open` side.
- **(b) Merged now.** It is additive: nothing reads it unless a program imports it and asks. The work is done once,
  and the second part starts from a tested rule set.
