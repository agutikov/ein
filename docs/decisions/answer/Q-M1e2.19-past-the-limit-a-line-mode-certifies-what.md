# Q-M1e2.19 — Past the limit, a line mode certifies what describe certifies

On a program the static check flags, past `--region-limit` a question that needs the whole cube is answered from
the members the run met in layer order. There `--minimal` lists exactly the members describe certifies on the
minimal line, a member met with no member of R met below it, and `--maximal` lists none and says the maximal line
is not known, as describe does (`--single`: *not decided*). The other members met are counted, not listed, and
`--dump-answer` writes each. Where ∅ is in R the minimal line is {∅}, so `--minimal`, `--minimal -n N` and
`--minimal --single` each answer ∅ alone, decided, after one saturation, at every limit.

**Why.** Two modes of one tool may not disagree. A member met past the limit is in R, but whether nothing below it
or above it is in R needs the nodes the run did not visit; only ∅, below everything, needs none.

**Held by.** `ein-render/tests/line_modes_past_the_limit.rs`:
`past_the_limit_a_line_mode_certifies_what_describe_does`, every line mode on every flagged corpus program under
lowered limits, held to describe and, where it is known, to the oracle.
