# P1g.11 — what school geometry found in the engine

**Phase:** [P1g.11](README.md). Opened 2026-11-11, with the plan, as the one `findings.md` of M1f's P1f.8.
**Split on 2026-11-13** with the phase ([README § The move](README.md#the-move-of-2026-11-13)): the rows that
were met by the geometry's probes are here, and F-1, F-2 and F-6, met by the algebra's, are in
[P1g.10's `findings.md`](../p1g.10_school_algebra/findings.md). **No row was renumbered,** so an id cited
before the split still names its row. **The next free id is F-7,** in one sequence over the two files: a new
row in either takes the lowest id used in neither, so that an F-id names one finding across the two phases.

The findings this phase's stages make about the engine or the language, outside their own scope. They are kept
**in the phase**, on the user's ruling of 2026-11-11 (a phase keeps what it finds; F25 was for P1f.7 alone, as
the calculator's [`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md) records).
Each row is shown to the user with the stage that found it and is fixed by a stage of this phase, ruled as it
stands, or given an owner before the phase closes. A **wrong answer** among them is a row of
[`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml) before anything fixes it.

**A finding is not a requirement.** A row here says what a program met. Whether the engine should change for
it is asked through [P1g.6](../p1g.6_engine_requirements/README.md), whose
[S1g.6.7](../p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md) lists the candidates this
phase is expected to bear on; each row below names its candidate there.

**The rows below are seeds, not findings yet.** Each was met once, by the planning agent's probes of 2026-11-11
([README § First probes](README.md#first-probes-2026-11-11)), on a scratch program that is not banked. The stage
named re-takes each as a kept witness, and only then is the row a finding: confirmed, or struck with the run
that struck it.

| id | what | witness (probe) | today (one run, 2026-11-11) | to be re-taken by | state |
|---|---|---|---|---|---|
| F-3 | **A nested value inside a `:why` prints as written.** With `(relation seg Point Point :why "the segment {?1}{?2}")` and `(relation seg-eq Segment Segment :why "{?1} is as long as {?2}")`, a `seg-eq` fact reads *(seg A C) is as long as (seg A B)*, not *the segment AC is as long as the segment AB* | G-1's `--trace`; the same in a query's `:goal-text` | as described. The calculator's numbers print the same way, `(n (n z 1) 9)` for 19, in A-1's `:why`s | [S1g.11.1](s1g.11.1_what_school_geometry_is.md) | seed. To establish against [`09_natural_language.md`](../../../docs/kernel/ir/03-ein-lang/09_natural_language.md): whether that page states this behaviour (then it is a request, for the user) or the other (then it is a wrong answer and a register row). S1g.6.7's T-4 |
| F-4 | **A fact derived under a harmless hypothesis is not root's.** A construction proposed by an hrule kills nothing and restricts nothing; what follows from it holds in its state only, and a query over all of R reports the theorem as one class of two, the other *undetermined* | G-2 | as described; `:over maximal` reads the theorem | [S1g.11.3](s1g.11.3_constructions_contradiction_cases.md) | seed. **Not a wrong answer:** it is R's definition ([`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md)). It is here because geometry is the first domain where a hypothesis is not a guess about the world; what the phase would need is named there, M1g's candidate C-8. S1g.6.7's T-1 |
| F-5 | **A refuted assumption concludes nothing at root.** Two candidates dead alone; the theorem that is their negation is not a fact | G-3 | as described | S1g.11.3 | seed. M1g's probe 4 and [Q-M1g.5](../open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact); owned there. Listed so that this phase's pages link one statement of it. S1g.6.7's T-7 |
