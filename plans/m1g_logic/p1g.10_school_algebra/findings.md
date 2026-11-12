# P1g.10 — what school algebra found in the engine

**Phase:** [P1g.10](README.md). Opened 2026-11-11, with the plan, as the one `findings.md` of M1f's P1f.8.
**Split on 2026-11-13** with the phase ([README § The move](README.md#the-move-of-2026-11-13)): the rows that
were met by the algebra's probes are here, and F-3, F-4 and F-5, met by the geometry's, are in
[P1g.11's `findings.md`](../p1g.11_school_geometry/findings.md). **No row was renumbered,** so an id cited
before the split still names its row. **The next free id is F-7,** in one sequence over the two files: a new
row in either takes the lowest id used in neither, so that an F-id names one finding across the two phases.

The findings this phase's stages make about the engine or the language, outside their own scope. They are kept
**in the phase**, on the user's ruling of 2026-11-11 (a phase keeps what it finds; F25 was for P1f.7 alone, as
the calculator's [`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md) records).
Each row is shown to the user with the stage that found it and is fixed by a stage of this phase, ruled as it
stands, or given an owner before the phase closes. A **wrong answer** among them is a row of
[`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml) before anything fixes it.

**The rows below are seeds, not findings yet.** Each was met once, by the planning agent's probes of 2026-11-11
([README § First probes](README.md#first-probes-2026-11-11)), on a scratch program that is not banked. The stage
named re-takes each as a kept witness, and only then is the row a finding: confirmed, or struck with the run
that struck it.

| id | what | witness (probe) | today (one run, 2026-11-11) | to be re-taken by | state |
|---|---|---|---|---|---|
| F-1 | **A small program beside the calculator, with no `(solve …)` line, is refused with exit 1.** The program declares relations of its own and three rules over `std.arith`, and does not close them; the blind enumerator takes them as candidates over the names in scope | A-8: A-1's program without `(solve :hypothesis-relations ())` | one line, *a state's commitments pass 2^128: the state route cannot count them*, exit 1. The calculator's README names the cause as a risk (*H is not empty by default*), and the user ruled the cause left as it is (*"author knows what he's doing"*). What is new is the outcome: not a large answer but a refusal, whose line does not say what to write | [S1g.10.1](s1g.10.1_survey_and_design.md) | seed. To establish: whether exit 1 here agrees with *every answer exits 0* (it is a limit, not an answer), and whether the line should name the relations and the two remedies. E-1 and E-7 of the calculator's findings are its neighbours |
| F-2 | **A trace cannot be read at one theory's level.** A problem solved by three rules of its own over the calculator traces as 123 steps, 120 of them the calculator's columns; the school solution is steps 2, 19 and 123 | A-1, `ein solve FILE --trace T.md` | the three steps are there, each with its `:why` rendered, among the rest; nothing selects them | S1g.10.1 (shown), [S1g.10.4](s1g.10.4_linear_equations.md) (ruled) | seed. Not a wrong answer. The phase's acceptance line *the trace is the school solution* rests on it; question f of the [README](README.md#the-questions-for-the-user). The geometry's wish for a *proof of this fact* read-out ([S1g.11.2](../p1g.11_school_geometry/s1g.11.2_theorems_as_rules.md#the-two-column-proof)) is put to the user with it |
| F-6 | **`unknown` cannot be a relation's name** where `std.macro` is in reach: the fact `(unknown x)` is refused as a macro's call | A-5 | refused at load, one line, which names the macro and the import | [S1g.10.2](s1g.10.2_expressions_and_evaluation.md) | seed. Right as ruled (S1f.7.12). A note for the phase's page on names, not a defect |
