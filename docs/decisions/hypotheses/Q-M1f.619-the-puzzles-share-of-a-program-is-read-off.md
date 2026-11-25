# Q-M1f.619 — The puzzle's share of a program is read off the writers that built H

The structure of H splits the facts a program states into the theory's share and the puzzle's: a **given**, a fact of
a relation a candidate is of, and a **clue**, a fact about the candidates' names. Which facts are clues depends on what
built H, read at root's fixpoint. **Where the program's own writers built it**, its rules that conclude a hypothesis,
its activated obligations, or the two together, their union
([Q-M1f.570](Q-M1f.570-h-is-the-union-of-the-hypotheses-root-holds.md)), the relations those writers read, a rule's
premises and an obligation's trigger and witness, are what the universe is made of: a fact of one is the theory's,
and every other fact that names a candidate's name is a clue. **Where the blind enumerator built it**, which reads
no relation, a clue is a fact that names two of the candidates' names, or one of them with a candidates' relation. A
fact that names none of the candidates' names is the theory's under either. No relation is read by its name and no
fact by its `:source`.

**Why.** A writer that reads a relation to propose a candidate says, by reading it, what kind of thing a candidate is
about, and a fact of that relation types the universe rather than constrains it. Under the union the hypotheses are
whoever wrote them, so every writer's reads are the universe's; the blind enumerator reads only names, and says
nothing of what the universe is made of.

**Held by.** `ein_infer::structure::small_kb`, and `Stated`'s `declared` and `generator_reads`, which take the rung
the generator ran at root; `ein-infer/tests/theory_share.rs`: `the_theory_excludes_a_declared_bijections_rows_and_columns`,
`two_programs_one_clue_apart_have_one_theory`, `the_same_puzzle_in_two_encodings_has_one_theory` and
`the_theory_derives_no_more_than_the_program`. The page:
[the hypothesis space § The theory's share, and the puzzle's](../../ein/reasoning/02-kb-evolution/05-hypothesis-space.md#the-theorys-share-and-the-puzzles).
