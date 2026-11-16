# Q-M1f.201 — A bare `(relation R)` declares no argument

A declaration with no type, `(relation flag)`, declares a relation of no argument: `(flag)` is its one form, and
`(flag up)` is refused at load as any form of another count is
([Q-M1f.315](Q-M1f.315-a-form-of-a-declared-relation-has-as-many.md)). A declaration's count is the count of its
types, with no special case for none.

**Why.** A bare declaration is how a program declares a flag, and every one is written with no argument. Were a
bare relation's forms not held, a flag would be the one kind of declared relation a misspelt count passes.

**Held by.** `ein-cli/tests/declared_counts.rs`: `a_form_of_another_count_is_refused_on_every_command`, the row
`a-bare-declaration-declares-none`, whose line is *the fact (flag up) gives flag 1 argument, and (relation flag)
declares 0*.
