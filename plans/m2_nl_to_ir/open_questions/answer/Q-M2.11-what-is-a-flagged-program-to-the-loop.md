# Q-M2.11 — What is a flagged program to the loop?

**Status:** ruled by the user, 2026-11-25

**Ruled by the user, 2026-11-25:** *"(a)"*: an outcome of its own.

**What (a) was:** a program the static check flags as not monotone
([Q-M1f.564](../../../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)) is **never
certified**; where it is flagged (`monotonicity.sites`) is fed back as a diagnostic at the feedback levels that
carry one; every table counts it in a column of its own; and the benchmark's canonical programs are never flagged,
as no program under `tests/` is. Not taken: (b), treating it as a refusal, and (c), reading its answer like any
other.

**For the re-plan:** it is a seventh outcome beside Stage A1's six, in the state machine
([S2.4.1](../../p2.4_loop/s2.4.1_state_machine.md)), in *certified*
([Q-M2.10](Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md)) and in the metrics
([S2.5.3](../../p2.5_harness/s2.5.3_metrics.md)). A formalizer will write such programs, since a closed-world reading
(*the youngest is the one nobody is younger than*) is an `absent` over a relation a commitment can fill, and how
often it does is itself a finding. What a flagged program's answer is stays
[M1n](../../../m1n_non_monotone_programs/README.md)'s.
