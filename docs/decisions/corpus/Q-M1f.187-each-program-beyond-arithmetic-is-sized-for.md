# Q-M1f.187 — Each program beyond arithmetic is sized for the commands that read it

Each program of `examples/beyond_arith/` is written at a size at which a bare `ein solve` describes R within the
default budget and its answer dump stays small: the laws over 0 to 3, the primes to 30, casting out nines to 40, the
perfect numbers to 12, a Collatz trajectory of 16 steps and one stopped at its bound of 20, equations of two and
three digits, I + BB = ILL. SEND + MORE = MONEY is the largest: a bare `ein solve` describes its R, one state, by the
obligation tree and the classes, and it is a `slow` entry. Its claim is held by `ein test`, which decides it with
`--single`'s question, as it is for the same program with its letters declared in another order.

**Why.** Every corpus program is solved, dumped and reloaded, and a state is written as text, so a program the
corpus reads must be one whose description fits. At these sizes each probe still shows what it is for. The puzzle
everyone knows is kept at its own size because its claim is one question, *is there exactly one answer*, which
`--single` decides without describing R.

**Held by.** The programs' entries in `corpus/corpus.toml`, the two SEND + MORE = MONEY programs the only `slow`
ones, `ein-corpus`'s `the_slow_set_is_exactly_these_fifteen`; each program's claim, which its `test` run holds.
