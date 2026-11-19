# Q-M1f.447 — A relation is overloaded by its count of arguments, and a declaration holds no form to it

A `(relation R L₁ … Lₙ)` is an ordinary fact, `(relation R L₁ … Lₙ)` beside `(relation R)`, which rules read; it holds
no form of `R` to its count. A program may write a fact, a pattern, a conclusion, a goal or a claim's fact of `R` at
any count, by intent or by a slip, and nothing says so: a fact of another count is another fact, which no pattern of
the first count matches. That is the one overloading Ein has, by the count of arguments, since Ein has no types and
will have none ([Q-M1f.372](Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)). A relation may be
declared more than once, at one count or at several, and no declaration is refused as a duplicate of another: each
is a fact of `relation`, and what several declarations of a name mean is what rules make of them. What reads a
declaration reads each: the blind enumerator guesses facts of a declared relation at each count it is declared
with, no label of any declaration is one of its objects, the trace reads a declaration's fact by that declaration's
`:why`, and `std.contradiction`'s `clash-of-N` activates the rule of each declared count.

**Why.** The user's ruling, on the question raised when `relation` became a convention of the standard library: a
check of a form's count against a declaration is the loader reading a theory's form, and a declaration is a fact a
rule may read as it likes. A program that is not well-formed is the program's to state.

**Held by.** `ein-cli/tests/declared_counts.rs`: `a_form_of_another_count_loads_on_every_command`,
`the_regressions_witnesses_load_on_every_command`,
`a_relation_declared_twice_loads_and_each_count_answers_its_own`, `what_is_not_a_declared_relations_form_loads`;
the seven `arity-*` witnesses of `corpus/regressions.toml`.
