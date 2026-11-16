# Q-M1f.195 — A problem that asks what is needed imports the readings, whatever its numbers

A word problem that asks how many containers are *needed* imports `readings.ein` and names its hrule,
`(solve :hrules (read-need))`, whether or not its division leaves a remainder, and is one file.
`boxes12.ein`, twelve apples in boxes of three, is such a file: its two states, a reading each, agree on 4 boxes,
and its claims state the region, the answer, the readings and the model's assumption.
`boxes13.ein` is the same problem with a remainder, whose readings disagree.

**Why.** Whoever writes a problem does not know before the run whether its division leaves something over, so the
form of a problem cannot depend on it. Where the readings agree, the answer is one and the solutions two: that is
what the run shows, not something the author must foresee.

**Held by.** `examples/word_problems/boxes12.ein`'s claims, `(solve … :expect (region …))` and its three queries,
held by its `test` run; its `--trace` run in `corpus/corpus.toml`.
