# Q-M1f.393 — A rule unpacks a fact by its count

A rule moves a fact out of a context by writing it: its conclusion is a fact pattern that stands inside one of its
premises, `(claims ?who (knows ?a ?b))` with the conclusion `(knows ?a ?b)`. What the text fixes is the fact's count
of arguments. Its relation may be a parameter, which an activator binds, and its arguments are variables, so one
rule a count serves every relation an activator names.

A variable that stands for a whole fact is no conclusion and no premise: `:assert ?f`, and `?f`, `(not ?f)` or
`(absent ?f)` in a `:match`, are refused
([Q-M1f.351](Q-M1f.351-a-bare-variable-or-name-is-no-whole-conclusion.md)). The one literal over such a variable is
the conclusion `(not ?f)`, which denies whatever fact `?f` is bound to
([Q-M1f.127](Q-M1f.127-a-conclusion-not-x-is-refused-where-the-program.md)). So a program can deny a fact it found
in a context whatever its relation, and can hold one, or ask whether one is held, only by spelling its shape.

**Why.** Every reading of a rule set asks which relations a rule reads and which it concludes: the strata, the
monotonicity check, the refusal of a refutation that rests on an absence, and the invariant that a state names no
relation the program lacks. Each reads the relation off the text, or off the activator that fills a parameter. A
conclusion or a premise of no fixed relation would be one of any relation a fact in a context can have. And a
premise of no fixed relation has no held extent to be matched in.

**Held by.** `ein-ir/src/ein.y`'s `Premise`, `Assert` and `Conclusion`; `ein-cli/tests/cli_semantics.rs`:
`a_conclusion_no_fact_can_hold_is_one_line_in_eins_spelling`; the claims of `examples/beliefs/01_common_belief.ein`,
which unpacks one rule a shape. The pages: [the rules](../../ein/reasoning/01-static-ir/02-rewriting-rules.md),
§ Packing and unpacking, with the table of what a rule can spell;
[rules](../../ein/user/03-syntax-and-semantics/05-rules.md), § Rules and contexts.
