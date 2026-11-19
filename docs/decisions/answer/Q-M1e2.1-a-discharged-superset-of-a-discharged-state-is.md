# Q-M1e2.1 — A discharged superset of a discharged state is in R

Every alive, discharged commitment is a node of R, so a commitment that adds a fact to a discharged state and stays
alive and discharged is a solution of its own, counted in `k_c` and, where its state differs, in `k_s`. On two guests
and two chairs where each guest owes a chair and each chair a guest, R holds seven states: the two matchings, the
four seatings of three facts, and all four facts. The two matchings are R's minimal line, which `--minimal` answers.

**Why.** An obligation is discharged by a witness, and a second witness does not undo it; whether a guest may sit
twice is a restriction's question, `functional`'s, and a program that means it states it. The count of R is the
count of its nodes, read without a preference for the smaller ones.

**Held by.** `ein-infer/tests/region.rs`: `the_six_monotone_programs_have_the_tables_regions`, which holds
`examples/ein-bugs/wrong-answers/owed-seats.ein` to `|R| 7` with its two minimal and one maximal member.
