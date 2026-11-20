# 71 · The defaults S1f.21.18 took

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the stage's record](../../../s1f.21.18_the_hrule_form_removed.md#record)

S1f.21.18 is done: there is no `hrule` form and no `:hrules` keyword. A hypothesis rule is a rule that concludes
`(open F)`, activated by the facts of its name a program writes. 120 programs were written again by a script, and
each answers as it did. You said to run the phase on defaults and review the questions after. These are the ten
it took.

## What you should know first

- **A misspelt activator, or one of another count, now loads in silence**
  ([V-58](../../../findings.md#v-58--a-misspelt-activator-or-one-of-another-count-loads-in-silence)). The `:hrules`
  clause refused both. As an ordinary fact it activates nothing, and where nothing else proposes, the blind
  enumerator then writes $H$: a program that meant its rule's candidates gets every guess, with no word said.
  Default 3 below took the checks out with the clause and put nothing in their place. **Yours to call**: whether a
  note says that a rule that concludes a hypothesis has no activator.
- **The sweep found three faults of the build before it**, by holding every rewritten program to its answer, and
  fixed them: two panics, a legal goal refused, and one wrong answer, where `(open (?R ?x ?y))` with `?R` bound to
  `eq` proposed four `eq` facts and gave 16 states where $R$ is {∅}.
- **A negated hypothesis held in a variable can no longer be written**
  ([V-56](../../../findings.md#v-56--a-negated-hypothesis-held-in-a-variable-cannot-be-written)): `(open (not ?x))`
  is a parse error. No corpus program that loaded wrote one.
- **Some words a tool may read have changed** (default 6): the mode is `hypothesis-rules`, and the summary's keys
  are `hypothesis_rules`, `hypothesis_plans` and `a_hypothesis_rule_proposes_it`.

## The ten

| # | question | what was asked | the default taken |
|---|---|---|---|
| 1 | [Q-M1f.621](../../../../open_questions.md#q-m1f621--where-a-removed-clauses-activators-are-written) | where a removed `:hrules` clause's activators are written | where the `(solve …)` stood when it empties (the empty solve goes), else just before it, with the comment lines that stood before the clause inside it; a bare name in the clause activates nothing and goes |
| 2 | [Q-M1f.622](../../../../open_questions.md#q-m1f622--how-a-hypothesis-rules-conclusion-is-written-under-open) | how an hrule's `:assert X` is written | `(open X)`, the whole conclusion: what was no fact stays no fact, and is refused, now at the parse: `(open (and …))`, `(open (absent …))`, `(open (not ?g))`, `(open (eq …))` |
| 3 | [Q-M1f.623](../../../../open_questions.md#q-m1f623--the-checks-the-hrules-clause-made-of-its-activators) | the `:hrules` list's own checks (a listed name is a declared hrule, an activator's count, an argument at a relation parameter a relation of the program) | they go with the clause: an activator is a fact as every rule's (Q-M1f.567's *a rule like any other*); a wrong count activates nothing, a misspelt one is a fact of another relation; their three witnesses (`solve-hrules-*`) and rows deleted. No refusal is added for rules |
| 4 | [Q-M1f.624](../../../../open_questions.md#q-m1f624--a-fixture-or-witness-whose-subject-was-the-form-itself) | a fixture or witness whose subject is the form itself | deleted with its entry and row: `ein_on_hrule` (its refusal goes), `hrule_reserved_name` (it became `rule_reserved_name`'s program), `macro-bare-conclusion-hrule` (`(open (id ?x))` expands to `(open ?x)`, a form, and loads) |
| 5 | [Q-M1f.625](../../../../open_questions.md#q-m1f625--file-names-test-names-and-regression-ids-that-contain-hrule) | file names, test names and regression ids that contain `hrule` | kept as identifiers: plan files the stage may not edit link them, and decision slugs are kept (V-55) |
| 6 | [Q-M1f.626](../../../../open_questions.md#q-m1f626--the-words-for-a-hypothesis-rule-as-a-writer-of-h) | the words for the writer (V-53) | the mode `hypothesis-rules`; the note *H, from \`MODE\`: …*; the h-space H row *· from \`MODE\`*; JSON `hypothesis_rules`, `hypothesis_plans`, `a_hypothesis_rule_proposes_it`; the `load` event's `hypothesis_rules`; the outside reason `not-proposed`, *no hypothesis rule proposes it*. The `rung` event and `Mode` keep their names |
| 7 | [Q-M1f.627](../../../../open_questions.md#q-m1f627--what-reads-a-hypothesis-rule-where-a-check-read-an-hrule) | what reads a rule that concludes a hypothesis where a check read an hrule | every loader check, `F` under `open` read as `F` where a check reads what a state can hold, and the names index; an `eq`/`neq` instance of a hypothesis template is none (Q-M1f.32) |
| 8 | [Q-M1f.628](../../../../open_questions.md#q-m1f628--two-decisions-whose-questions-the-clause-asked) | Q-M1f.115 and Q-M1f.116, whose questions the clause asked | rewritten in place under their ids for activator facts, not deleted: `open_questions.md` links both |
| 9 | [Q-M1f.629](../../../../open_questions.md#q-m1f629--the-witness-of-a-hypothesis-rule-that-concludes-eq) | `hrule-eq.ein`, whose shape `(open (eq …))` the grammar refuses | written through a parameter, `(h eq)`, so the witness keeps its subject and its answer, H empty and R {∅} |
| 10 | [Q-M1f.630](../../../../open_questions.md#q-m1f630--the-saturated-dumps-section-of-hypothesis-rules) | `ein saturate --dump`'s hypothesis rules | the hypothesis registry under *;; Hypothesis rules (N)*, each with the activators the knowledge base holds; a rule that concludes facts beside its hypotheses stands in the agenda's section too |

## Your call

A number and what you want otherwise, in [`ANSWERS.md`](../ANSWERS.md) under item 71. Blank keeps the ten as built.
