# Q-M1f.225 — The fact-grain closure gives up past 10 000 shapes, and the test that reads it fails there

Fact grain, the prototype of a finer monotonicity check beside the engine, closes a program's candidates under its
rules over shapes. The closure is computed only where a site of the static check reads it, so a program the check
does not flag costs it nothing. On a flagged program it gives up once 10 000 shapes are added: the shapes are then
taken to be every fact, every site is dynamic, and the verdict is the safe one, *not monotone*. The test that holds
the prototype's verdicts to the cube fails on a corpus program on which the closure gives up, naming it, so the
bound never decides a verdict that test reads. Such a program is a flagged one in which a supposed fact leads,
through a rule with an operand the commitment does not bind, to a request to `std.math.decimal-arith`, whose
circuits have shapes without end; the corpus has none.

**Why.** Without a bound the closure does not end on such a program. With a bound whose safe verdict the test read,
the test would check less, on a program chosen by the size of its closure, and say nothing of it. Failing keeps what
the test means on every program, and ends.

**Held by.** `ein-infer/examples/shared/fact_grain.rs`: `MAX_SHAPES`, `FactGrain::gave_up`;
`ein-infer/tests/region.rs`: `fact_grain_never_calls_a_violated_cube_monotone`,
`fact_grain_closure_runs_where_a_site_reads_it_and_gives_up_on_the_safe_side`;
[saturation and negation in code § 4](../../ein/ein.rs/08-reasoning-in-code/02-saturation-and-negation.md#4-monotone-and-non-monotone-programs).
