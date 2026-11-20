# 83 · Three pieces of the search have no subject under the union

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the finding](../../../findings.md#v-47--three-pieces-of-the-search-have-no-subject-in-the-corpus-under-the-union)

A finding of the phase that is **yours to call**. It was on the findings page alone; it is here so that everything
that waits for you is in one place. Nothing waits on it.

## What it is

Under the union an activated obligation writes every witness its guard gives at root, so on a monotone program no
instance root owes has an unproposed witness. Three pieces of the search were built for exactly that case: the
last reason of licence L5, licence L9 (`ein_infer::unheld`), and the coupling's unlicensed path. No corpus program
reaches any of them now. They are kept, and their tests say they license nothing today.

## The question

- **(a)** The machinery stays, with tests that say it licenses nothing today. **Default.**
- **(b)** The machinery goes, with its tests and its pages.

## On the built engine

Taken by [S1f.21.19](../../../s1f.21.19_the_review_items_on_the_built_engine.md) on two engines: the final binary,
the release build of `7b72bd84` (`runs/version.txt`), and the engine before the phase's second part, `ddf09492`, run
on its own corpus and library (`runs/old/version.txt`; its runs are under `runs/old/`). The tests ran on the
worktree at `7b72bd84` in the gate profile. The tool is `tools/run.sh`, with `tools/census.py` beside it; `runs/`
and `tools/` are in the main checkout beside this page, not in git. The item has no program of its own: every
program it runs is the corpus's.

**Where the three pieces are**, in the final tree, and the pages that say what each is:

| piece | code | page |
|---|---|---|
| L5's last reason: a fact that can witness an owed instance is neither a candidate nor denied at root | `ein-infer/src/oblgen.rs`: `alternatives` (L5's premise, checked at load; the last of its four refusals) and the scan of one instance's witnesses, `witnesses`, whose refusal reads *a `R` fact can witness it that no candidate proposes* or *`F` can witness it and is not a candidate*; `ein-infer/src/structure.rs`: the structure's reading of the same reason, `UNPROPOSED` and `WitnessIs::Unproposed` | [07-finding-r.md § 4](../../../../../../docs/ein/reasoning/02-kb-evolution/07-finding-r.md#4-a-monotone-program-three-properties-and-the-licences), L5 |
| L9, a fact nothing proposes and nothing writes is in no state | `ein-infer/src/unheld.rs`: `Unwritten::of` and `is_unwritten` (the relations no plan root compiled writes), `breach` and `refusal` (the premise read at a node, and the run refused where it fails), `without` and `trusting` (the two switches only tests reach); `ein-infer/src/oblgen.rs`: `Instance::unheld`, `Unheld`, `Alternatives::with_l9`, `by_l9` and `unheld_relations`, and the scan in `witnesses` reading on past L5's refusal while each such witness is of an unwritten relation; `ein-infer/src/region.rs`: `NodeOut::unheld`; `ein-infer/src/explore.rs`: `Licence::Unheld`, the route row's words | 07-finding-r.md § 4, L9; [02-algorithms.md § 8.2](../../../../../../docs/ein/ein.rs/02-algorithms.md#82-the-obligation-tree-and-the-lines); [08-read-out.md](../../../../../../docs/ein/user/04-reference/08-read-out.md), the `route` row |
| the coupling's unlicensed path: an instance L5 does not license is a debt whose payers are read off its witnesses' holders | `ein-infer/src/structure/coupling.rs`: `Debt.licensed`, the debts in `Coupling::read` (an unlicensed instance's payers are the candidates whose own node at root holds one of its witnesses, `held_at_root`, read only where root's singles were), `Coupling::payers_read`, `Coupling::unlicensed`; `ein-infer/src/structure.rs`: `Owing::Unlicensed { payers, about }`, built in `ExactlyOne::of`, `ExactlyOne::unlicensed`, `ExactlyOne::brought`; `ein-render/src/structure.rs`: the document's rows of such an instance (`why`, `pay_by_the_theory_alone`, `about_group`) and the stopped layer's sentence *an instance L5 does not license has no payers read* | [05-hypothesis-space.md](../../../../../../docs/ein/reasoning/02-kb-evolution/05-hypothesis-space.md#coupling-questions-edges-ties-and-debts), *the payers read*; [03-hypotheses-and-the-answer.md](../../../../../../docs/ein/ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md), the document's keys |

**The tests that say they license nothing today**, each run on the worktree (`runs/test.<file>.txt`; every one
passes, `ok`):

| test | what it asserts |
|---|---|
| `ein-infer/tests/unheld.rs` `l9_licenses_where_nothing_writes_the_witness_relation` | over every corpus program and regression witness, the instances L9 licenses, program by program: the list is empty; and `13_witness_of_two.ein`'s one instance is licensed by L5 |
| `unheld.rs` `l9_is_the_oracle_on_every_monotone_program_it_reaches` | `programs == 0 && nodes_read == 0 && owing == 0`, *"a program with an instance L9 licenses is back, and this sweep reads it"*; it prints `L9: 0 programs, 0 nodes, 0 owed instances L9 licenses` |
| `unheld.rs` `every_question_answers_alike_with_l9_and_without` | returns at once where no corpus program has an instance L9 licenses (`if files.is_empty() { return; }`): today it reads nothing |
| `unheld.rs` `a_witness_a_rule_writes_is_a_candidate_too` | on a program in the test, a witness a rule writes is a candidate under the union: L5 licenses the debt, R is five states, no question names L9, and under `unheld::trusting` every question answers as without it |
| `ein-infer/tests/coupling.rs` `the_coupling_is_held_to_n_and_to_r` | a census it prints, not an assertion: *17 debts are edges, on 5 programs, 0 of them of an instance L5 does not license, on 0* |
| `coupling.rs` `a_debt_is_an_edge` | `send.debts.is_empty() && send.unlicensed == 0` (`send_more_money.ein`) |
| `coupling.rs` `holders_unread_leave_the_coupling_incomplete` | on both zebras at a budget of one fact, `p.coupling.unlicensed == 0 && p.coupling.payers_read`: *"both zebras' tables hold none"* |
| `ein-infer/tests/exactly_one.rs` `one_relation_owes_a_group_in_each_orientation_and_holds_no_grid` | `zebra.ein`: `(x.unlicensed(), x.not_groups()) == ([], [])`, every one of its 36 instances licensed |
| `exactly_one.rs` `a_cryptarithms_letters_are_owed_and_its_digits_are_not` | `(x.owed.table().len(), x.unlicensed().len()) == (8, 0)` |
| `exactly_one.rs` `a_member_in_more_than_two_groups_is_in_no_grid` | `06_friends_tables_everyone_sits.ein`: `x.unlicensed() == []` |
| `ein-infer/tests/structure_invariance.rs` `a_form_that_reorders_the_universe_moves_no_set` | `(c.debts.len(), c.unlicensed) == (2, 0)` |
| `ein-infer/tests/region.rs` `a_witness_two_candidates_derive_together_is_a_candidate_itself` | `13_witness_of_two.ein`: 4 candidates, its one instance licensed by L5 with the witness `(fed Ann Soup)` its one alternative |
| `ein-cli/tests/h_space_view.rs` `the_document_is_the_owners_reading_on_every_instance` | `zebra.ein`'s document: `exactly_one.unlicensed` empty (beside 112 candidates and 352 groups) |
| `h_space_view.rs` `holders_a_stopped_layer_left_unread_are_said` | `zebra.ein` at `--region-budget 1`: `coupling.unlicensed_instances == 0`, `coupling.payers_read == true` |

**The two programs that were the subject, on both engines** (`runs/X.licences.txt` gathers the rows; each from
`X.show-hyps.txt`, `X.solve.txt`, `X.oracle.txt`, `X.h-space.txt` and `X.events.jsonl`, under `runs/` and
`runs/old/`):

| | `13_witness_of_two.ein` on `ddf09492` | final | `14_seats_nothing_proposes.ein` on `ddf09492` | final |
|---|---|---|---|---|
| H | 3, built by the rung `hrules` | 4, ``from `union` ``: the witness `(fed Ann Soup)` among them | 12, built by the rung `hrules` | 16, ``from `union` ``: the four seats Cat and Dan could pay with at the small table among them |
| instances L5 does not license (`ein render h-space`) | 1 of the 1 root owes | 0 of 1 | 2 of 4 | 0 of 4 |
| the `route` row | the state route; L1, L3, L4 | the tree and the classes; L1 to L5 | the tree and the classes; L1 to L5 and **L9** | the tree and the classes; L1 to L5 |
| R, by `ein solve` and by `ein oracle` | 5 states | 8 states | 4 states | 8 states |
| saturations · facts beyond root, describing R | 8 · 17 | 16 · 37 | 69 · 226 | 149 · 556 |

The events: no event kind names a licence or a holder ([05-events.md](../../../../../../docs/ein/ein.rs/05-events.md)
has none). The `region` event carries the summary's `route` and `licences`, which are the route row's; the final
binary's stream adds one `rung` event, `mode` `union` (13: `owed` 1, `candidates` 1; 14: `owed` 4, `candidates` 16),
which the engine before did not write on these programs. Holders are read only by `ein render h-space`: its
document lists, for each witness, the candidates whose own node holds it (`held_at_root_by`). R moved on both
programs: the union supposes the witnesses outright, which
[item 58](../58-the-corpus-under-the-rulings/README.md) lists with the corpus.

**The classic zebra was the coupling's subject** (`runs/old/zebra.h-space.txt`, `runs/zebra.h-space.txt`): on
`ddf09492` 18 of its 36 instances were unlicensed, each about a group (*18 an unlicensed instance is about*), and
18 debts were edges, all unlicensed, 11 edges a debt's alone; on the final binary 0, 0 and 0.

**The census over the corpus** (`runs/census.final.txt`, `runs/old/census.txt`, a row a program in the `.tsv`
beside each): every corpus program through a describing `ein solve --json-summary` (its `region.route` and
`region.licences`) and `ein render h-space --view json` (its owed instances, why each unlicensed one is, and the
coupling's debts).

| | `ddf09492`, its corpus | the final binary |
|---|---|---|
| programs; answered with a route | 933; 719 | 931; 744 |
| programs whose route names L5 | 38 | 40 |
| programs whose route names L9 | 2: `06_friends_tables_everyone_sits.ein`, `14_seats_nothing_proposes.ein` | 0 |
| programs whose root owes an instance (h-space); instances | 63; 391 | 59; 385 |
| instances L5 does not license | 116 on 28 programs | 85 on 19 |
| of them for L5's last reason | 106 on 20: the 15 below, and `13_mixed_solution_and_open.ein` 1, `06_friends_tables_everyone_sits.ein` 3, `13_witness_of_two.ein` 1, `14_seats_nothing_proposes.ein` 2, `zebra.ein` 18 | 81 on 15, **every one a program that states `(solve :no-search)`**, H empty |
| groups an unlicensed instance is about | 18, on `zebra.ein` | 0 |
| debts of an unlicensed instance that are edges | 19 on 2: `zebra.ein` 18, `oblig-fresh-name-two-declared.ein` 1 | 0 |

The other 4 unlicensed instances of the final corpus are refused for another of L5's reasons, a witness with a slot
nothing binds (`runs/census.final.tsv`, the `last_reason` column 0).

**Beside the finding.** The finding says no corpus program reaches any of the three pieces now. For L9 and for the
coupling's unlicensed debts the census agrees: no route names L9, and no unlicensed instance is a debt that is an
edge. L5's last reason is still *given*, on 15 programs, all under `tests/`, each stating
`(solve :no-search)`: H is empty there by the program's word, so every witness is no candidate, and there is no
search for the reason to steer; the coupling counts these instances as unlicensed (85 on 19 programs, the 15 among
them) and reads no payer for them, there being no candidate. The finding's argument (*an activated obligation writes
every witness its guard gives at root*) holds where the program searches; `:no-search` proposes nothing (a finding
of this stage, in its record). The stopped layer's sentence the stage's record names is still said on a program
with no unlicensed instance: `runs/zebra.h-space-budget-1.txt`, `zebra.ein` at `--region-budget 1`, *0 instances
L5 does not license*, and *an instance L5 does not license has no payers read*.
