# Q-M1f.119 — A goal and a claim ask only about relations of the program

Every relation a query's `:goal` asks about, at any depth and under `(absent …)` and `(not …)` too, and every relation
a claim lists a fact of, is a relation of the program, an atom it writes first in a tuple
([Q-M1f.522](Q-M1f.522-a-relation-of-the-program-is-any-atom-it-writes.md)): one it declares, states a fact of, writes
as the relation of a fact that is an argument, or writes in a rule's, an hrule's or an obligation's `:match` or
`:assert`, at any depth, or passes where a relation's name is held; or one a `(solve :hrules …)` activator has its
hrule propose. `eq`, `neq` and the declarations, `(relation ?R ?T)`, are every program's, and no theory's relation is:
`is-a` is a relation of the program only where it writes one, as any relation is
([Q-M1f.436](../engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md)). A goal's own relations do not count. Any other
is refused at load, placed at the pattern: `query: its :goal writes (payed ?g), and the program names no relation
payed — …`.

**Why.** No state holds a fact of a relation the program names nowhere, so a goal over one has no row in any state,
and under `(absent …)` holds of everything: a misspelling answered as a question. One notion of a relation of the
program serves the `(solve …)` lists ([Q-M1f.114](Q-M1f.114-a-relation-a-solve-list-names-is-one-the-program.md)),
a goal and a claim. A relation only a rule's `:match` reads counts, since a library rule may read what its importer
supplies.

**Held by.** `ein_ir::from_ir`'s `program_relations`; `ein-cli/tests/goal_premises.rs`:
`a_goals_relation_is_refused_at_its_pattern` and `a_goal_over_a_relation_of_the_program_loads`.
