# Q-M1f.47 — A written fact of `eq` at another arity than two is refused at load

A top-level fact headed `eq`, or its negation `(not (eq …))`, is read as a conclusion with no premise, checked and
never stored ([Q-M1f.33](Q-M1f.33-a-written-fact-of-eq-is-a-conclusion-with-no.md)). Written with any number of
arguments but two, `(eq a)`, `(eq)`, `(eq a a a)` or `(not (eq a))`, it is refused at load, one line per fact, on
every command that loads the program: `the fact (eq a) writes eq with 1 argument — eq is the relation of equality,
which holds between exactly two values`.

**Why.** `eq` holds between exactly two objects and its facts are computed, never stored, so a fact of another arity
states nothing that could be checked. The grammar refuses `(eq ?x)` as a premise and as a conclusion; the loader is
where a written `eq` fact is read, so it is where this shape is refused.

**Held by.** `ein-ir`'s `from_ir::tests::eq_at_another_arity_is_refused_at_load` and
`ein-cli/tests/cli_semantics.rs`: `eq_at_another_arity_is_refused_at_load`, which holds the line and the exit on
every command.
