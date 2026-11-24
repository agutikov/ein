# 83 · Three pieces of the search have no subject under the union

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the finding](../../../findings.md#v-47--three-pieces-of-the-search-have-no-subject-in-the-corpus-under-the-union)

A finding of the phase that is **yours to call**. It was on the findings page alone; it is here so that everything
that waits for you is in one place. Nothing waits on it.

> **The pictures are files under each folder's `runs/`, which git ignores**, embedded as `<img>`. If one is
> missing, `bash tools/pieces.sh` writes the three pieces' runs and drawings, on the final binary and on the engine
> before the union. `bash tools/run.sh` re-takes the census and the tests of [§ On the built engine](#on-the-built-engine);
> it is slower.

## In one paragraph

Each of the three pieces answers one case: **an instance root owes, one of whose payments is no candidate and is
not denied at root.** L5 refuses to license such an instance (piece 1). L9 licenses it anyway where no rule can
write that payment (piece 2). The hypothesis space's coupling reads its payers off the candidates whose own node
holds the payment (piece 3). Under the union (Q-M1f.570) an activated obligation writes every payment root neither
holds nor denies as a candidate, so **on a program that searches the case never arises**, and no corpus program
reaches any of the three. They are kept, and their tests say they license nothing today. The one place the case
still arises is a program that states `(solve :no-search)`: H is empty there by its own word, and nothing is
searched for the pieces to steer.

<img src="runs/overview.svg" alt="the case the three pieces were built for: an owed instance's witness that is neither a candidate nor denied at root; piece 1, L5's last reason, refuses the instance; piece 2, L9, licenses it where no rule writes the witness's relation; piece 3, the coupling, reads its payers off the witness's holders; the union empties the case">

*Drawn by hand (`tools/overview.dot` → `runs/overview.svg`); it explains, and is no render of the engine's.*

## The three pieces

| # | piece | what it does | where you can see it work | today |
|---|---|---|---|---|
| [**1**](1-l5-last-reason/README.md) | **L5's last reason** | refuses to license an owed instance one of whose payments is neither a candidate nor denied at root; the search then cannot branch on the debt and walks every alive state | the engine before the union, on `one-way-to-eat` and the corpus's `13_witness_of_two.ein`: the state route | given only under `(solve :no-search)`, 15 corpus programs, H empty |
| [**2**](2-l9-nothing-writes-it/README.md) | **L9** | licenses such an instance anyway where the payment is of a relation no rule concludes a fact of: it can be in no state | the engine before the union, on `a-seat-nothing-proposes` and the corpus's `14_seats_nothing_proposes.ein`: a route that names L9 | no route names it |
| [**3**](3-coupling-unlicensed-debts/README.md) | **the coupling's unlicensed debts** | reads an unlicensed instance's payers off its payment's holders, the candidates whose own node holds it; where they lie in two questions, the debt is an edge | the engine before the union, on `two-ways-to-eat` and the classic `zebra.ein`: 18 unlicensed debts that are edges | none |

Each page has small programs written for it, each run on the final binary and on the engine before the union (the
release build of `ddf09492`, which reads the `.old.ein` twins), the corpus programs that were its subject, the
code and the tests, and drawings.

## The question

- **(a)** The machinery stays, with tests that say it licenses nothing today. **Default.**
- **(b)** The machinery goes, with its tests and its pages.

## On the built engine

Taken by [S1f.21.19](../../../s1f.21.19_the_review_items_on_the_built_engine.md) on two engines: the final binary,
the release build of `7b72bd84` (`runs/version.txt`), and the engine before the phase's second part, `ddf09492`, run
on its own corpus and library (`runs/old/version.txt`; its runs are under `runs/old/`). The tests ran on the
worktree at `7b72bd84` in the gate profile. The tool is `tools/run.sh`, with `tools/census.py` beside it. The pieces'
pages were added later, their runs taken by `tools/pieces.sh` on the release build of `a26a0a79`, the engine after
S1f.21.25, whose change touches none of the three. Each piece's page lists its code and its tests.

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
every witness its guard gives at root*) holds where the program searches; `:no-search` proposes nothing (V-92, a
finding of this stage, in its record). The stopped layer's sentence the stage's record names is still said on a program
with no unlicensed instance: `runs/zebra.h-space-budget-1.txt`, `zebra.ein` at `--region-budget 1`, *0 instances
L5 does not license*, and *an instance L5 does not license has no payers read*.
