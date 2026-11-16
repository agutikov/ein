# Q-M1f.46 — A relation parameter that can hold `eq` heads only a form of two arguments

`eq` is the relation of equality between exactly two values. A rule's or an hrule's relation parameter that can be
bound to `eq` and heads a premise, at any depth of `and`, `or`, `not` and `absent`, or a top-level conclusion whose
arity is not 2 is refused at load, one line each, on every command that loads the program: *… its parameter ?R
heads (?R ?x), a form of 1 argument, and the fact (r eq) binds ?R to eq — eq is the relation of equality, which
holds between exactly two values*, or, where a rule derives the activator, the line names where `eq` comes from.
Which parameters can hold `eq` is read as [Q-M1f.41](Q-M1f.41-a-position-holds-a-fact-unless-the-program.md) reads
relation names: where a fact, a conclusion or a `(solve :hrules …)` activator writes `eq` at such a position, or a
conclusion passes on a variable that can hold it, to a fixpoint. A parameter bound to a declared relation of
another arity is not refused: it matches nothing, as the relation written directly does.

**Why.** The grammar refuses `(eq ?x)` where it is written
([Q-M1e.18](Q-M1e.18-eq-neq-and-absent-are-read-only-at-their-own.md)); through a parameter the same shape would
otherwise stop a run where its plan is compiled, at root on one command and only under a commitment or an hrule's
activator on another, so two commands would disagree on one program.

**Held by.** `ein-ir/src/from_ir.rs`' unit test `eq_at_another_arity_is_refused_at_load`, and
`ein-cli/tests/cli_semantics.rs`' test of the same name.
