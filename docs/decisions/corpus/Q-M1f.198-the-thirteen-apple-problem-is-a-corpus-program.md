# Q-M1f.198 — The thirteen-apple problem is a corpus program

`examples/word_problems/boxes13.ein`, *Bob has 5 apples, Alice has 8, a box holds 3: how many boxes do they need?*,
is a corpus program like the others: two states and two answers, 4 boxes with one apple left unpacked if *need* is
read as whole boxes only, 5 boxes if every apple must be packed, each state naming the reading it rests on. The
region test's fact-grain check reads it as it reads every other corpus program.

**Why.** The problem is the one whose readings disagree, which is what a word with two readings is for; the readings
are always owed ([Q-M1f.332](Q-M1f.332-a-word-with-two-readings-always-has-one-chosen.md)), so the program is
monotone, and the fact-grain closure runs only where a relation-grain site reads it, which no unflagged program has.

**Held by.** `boxes13.ein`'s claims, held by its `test` run; `ein-infer/tests/region.rs`:
`fact_grain_never_calls_a_violated_cube_monotone`, which reads every corpus program within the oracle's limit, this
one among them.
