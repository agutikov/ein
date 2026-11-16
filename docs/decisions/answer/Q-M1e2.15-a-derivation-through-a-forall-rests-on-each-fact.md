# Q-M1e2.15 — A derivation through a `forall` rests on each fact it read

A derivation through a `forall` rests on each fact the universal read (`Prov::universal`, recorded when the rule
fires), each folded into the core at its own smallest frontier, beside the premises the core search folds as it
does. A contradiction's facts, both sides of a pair, are explained together.

**A core is taken only where a surface prints one.** A bare `ein solve` takes root's and nothing a surface does not
print; the read-out, `--dump-answer`, `--json-summary`, `--trace`, `--dump-lattice`, `--events` and `ein oracle
--dump` take the ones they print. Which cores a run takes moves no answer.

**Why.** A `forall` holds because of every fact it ranged over, so a core that leaves one out names too little
for the contradiction to follow. A core is an explanation search the classification of a node does not need, so a
run pays for one only where it is read.

**Held by.** `ein-infer/tests/region.rs`: `a_contradictions_core_covers_both_sides_and_what_a_forall_read`;
`ein-infer/tests/corpus_cores.rs`: `every_core_names_its_commitment_and_derives_its_contradiction`;
`ein_infer::region::Cores`; `ein-infer/tests/explore.rs`: `a_run_that_prints_no_core_takes_none_and_answers_alike`.
