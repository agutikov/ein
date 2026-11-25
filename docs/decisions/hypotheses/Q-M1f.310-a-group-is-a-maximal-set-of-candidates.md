# Q-M1f.310 — A group is a maximal set of candidates that pairwise exclude by the theory alone

A **group** of the hypothesis set H is a maximal set of two or more candidates every two of which exclude each other
by the theory alone: a maximal clique of the graph over all of H whose edges are the theory's pairs, those dead in
the small KB, the program with the puzzle's statements taken out. A candidate dead alone at root is a member like any
other, and marked; a candidate in no pair of the theory is in no group. Groups overlap, a fact of a relation stated a
bijection lying in its row and in its column, so the groups are a **cover** of the candidates they hold and no
partition. The maximal cliques of root's own exclusion graph are kept beside them, as what the clues add, and are
not groups.

What the structure builds on the groups is read off them and what root owes, and nothing else: an **owed group** is a
group whose members are exactly a licensed instance's alternatives, of which a state of R holds exactly one open
member; a **grid** is a component of the theory's graph whose groups fall into two families, every member in exactly
one group of each and two groups sharing at most one member; a **bijection** is a grid of two equal families every
group of which is owed; and a **question** of the coupling is a group or a candidate in no group. Nothing is
synthesised: where a *must* is missing, the structure says what is missing and not what would fill it.

**Why.** Root's own graph mixes the theory with the clues: once the clues have decided part of a puzzle, a row is
strictly inside a larger clique of root's pairs, and on the Zebra puzzle no row or column of two or more open members
is a maximal clique of root's graph. Over the theory's pairs and all of H, a declared bijection's groups are exactly
its rows and columns, every candidate in two, which is the control any right reading must meet, and no relation's
name or declaration is read to get there.

**Held by.** `ein-infer/tests/cover.rs`: `the_groups_of_a_declared_bijection_are_its_rows_and_columns`,
`every_cover_is_whole_and_its_groups_are_the_maximal_cliques`, `a_conflict_of_three_is_in_no_group`;
`ein-infer/tests/exactly_one.rs`: `a_declared_bijections_rows_and_columns_are_owed_groups`,
`a_member_in_more_than_two_groups_is_in_no_grid`; `ein_infer::structure::Cover`, `ein_infer::structure::ExactlyOne`;
[the hypothesis space and its structure](../../ein/reasoning/02-kb-evolution/05-hypothesis-space.md#the-group-and-the-cover),
§ The group, and the cover, and
[§ Exactly one](../../ein/reasoning/02-kb-evolution/05-hypothesis-space.md#exactly-one-owed-groups-and-grids).
What shows it: [Q-M1f.309](../cli/Q-M1f.309-ein-render-h-space-shows-the-hypothesis-space.md).
