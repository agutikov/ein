# Q-M2.20 — What are `--max-steps`' name, unit and default?

**Status:** open

**Raised 2026-11-25 by S2.1.2, the bound's
defaults 1 and 2**, and the fifth of the user's choices in
[the phase report's § 7](../../defaults/p2.1.md).

**The question.** S2.1.2 bounded every saturation a question about R computes by a cap Ein did not have, under the
user's ruling on [Q-M2.16](Q-M2.16-what-of-the-kernel-may-change-inside-m2.md). Are its
name, its unit and its default the right ones?

**The defaults taken**, in S2.1.2's record:

1. *"**The option**: `--max-steps N` on `ein solve` and `ein test`, the name and unit of `ein saturate --max-steps`;
   one or more (`--region-budget`'s rule), a usage error otherwise."*
2. *"**Its default**: 2^20 firings per saturation, the region budget's power. The corpus's largest saturation is
   17 587 firings (`beyond_arith/laws.ein`; the median 4, the 99th percentile 4 096), measured once over every
   corpus file under six questions with a counter that is not kept; the default stops `grow.ein` in 1.1 s at a
   peak of 942 MB here."*

Ein's decision [Q-M2.16](../../../../docs/decisions/answer/Q-M2.16-every-saturation-a-question-about-r-computes.md)
states both: *"a cap on firings, `--max-steps N` on `ein solve` and `ein test` … Its default is 2^20"*.

**What the defaults cost.** The unit is an engine count, not a property of the program
([E-85](../../evaluation/bounds/E-85-the-cap-counts-firings-an-engine-count.md): *"a cap chosen for one program does
not carry to a rewrite of it with the same facts"*); and the default lets a runaway program take about a gigabyte
before it stops
([E-81](../../evaluation/bounds/E-81-the-default-cap-costs-about-a-gigabyte-on-a-runaway-program.md); 727 MB through
EinAf's call, [E-100](../../evaluation/bounds/E-100-nothing-bounds-a-calls-memory-in-process.md)), so a caller that
runs many programs in one process sets a lower cap of its own.

**The alternatives** (the first two were weighed and not taken in S2.1.2's design, which gives why):

1. **A cap on facts**, the semantic unit (the `work` row and the budget count facts), under a name of its own,
   `ein saturate --max-steps` keeping firings. S2.1.2: *"one option name with one meaning across commands won"*.
2. **The region budget extended to root's saturation** in place of a new option: rejected because it *"stops runs
   the corpus and the tests make today under small budgets"*, so answers would move.
3. **Another default**: one at or above the corpus's largest measured saturation, 17 587 firings, stops none of the
   runs that measurement took; a lower default than 2^20 costs a runaway program less memory and time before it is
   told *not decided*.
4. **As taken.**

**Where the answer lands**: it is about Ein, so a decision under `docs/decisions/`, or Q-M2.16's text where the
defaults stand.
