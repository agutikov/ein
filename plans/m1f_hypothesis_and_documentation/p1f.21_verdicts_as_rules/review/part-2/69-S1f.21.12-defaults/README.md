# 69 · The defaults the build of S1f.21.12 took

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the stage's record](../../../s1f.21.12_hypotheses_as_facts.md#record)

S1f.21.12 is built: `open` is a relation the kernel reads at the top of a fact, the hypotheses are held as
`(open F)`, an obligation's verdict as `(open R)`, and $H$ is the union of every writer. The corpus moved as
[item 58](../58-the-corpus-under-the-rulings/README.md) said it would. You said to run the phase on defaults and
review the questions after. These are the fifteen the build took where your rulings were silent.

## What you should know first

- **`oblig-derived-domain.ein` is not solvable on the built engine**, against your expectation, which you called
  no requirement. It is flagged, by its obligation's guard reading `food`. $H$ is one candidate, `(likes Ann Soup)`,
  dead alone; $R$ is empty, every alive state owing; its claim `(model (likes Ann Pie))` fails. Before the build
  $H$ had six candidates, $R$ was `(likes Ann Pie)`, the claim held and the program was monotone. It is what the
  stand-in of [item 64](../64-an-obligation-that-no-longer-steps-aside/README.md) showed.
- **The classic zebra's structure under the union**
  ([V-48](../../../findings.md#v-48--the-classic-zebras-structure-under-the-union)): its answer is the one state it
  was, and `ein render h-space` reads it quite otherwise, 352 groups and no grid where it read 36 groups and five
  grids, with each placement held beside its mirror.
- **Three pieces of the search have nothing to act on in the corpus now**
  ([V-47](../../../findings.md#v-47--three-pieces-of-the-search-have-no-subject-in-the-corpus-under-the-union)):
  an obligation writes every witness its guard gives, so no owed instance has an unproposed one. Whether that
  machinery goes is yours to call.
- **Default 13 is the one to look at first**: it lets `absent` and `open` head a fact in three more places than
  your ruling named.

## The fifteen

| # | question | what was asked | the default taken |
|---|---|---|---|
| 1 | [Q-M1f.606](../../../../open_questions.md#q-m1f606--which-reads-make-a-rule-that-concludes-a-hypothesis-a-site-of-the-static-check) | which reads of a rule that concludes `(open F)` are the `hypothesis` site | a read of `D` outside an `absent` or inside an even number of them, as a saturation rule's monotone reads; the literal reading of Q-M1f.574 (any depth) flagged `hrule-reads-unknown.ein` and its kin for an `unknown` probe |
| 2 | [Q-M1f.607](../../../../open_questions.md#q-m1f607--the-blind-enumerator-under-no-search) | the blind enumerator under `(solve :no-search)` | it writes nothing: no candidate, as no writer runs |
| 3 | [Q-M1f.608](../../../../open_questions.md#q-m1f608--a-states-open-facts-within-its-own-pass) | a state's `open` facts within its own pass | written once at the fixpoint and only added to while a rule that reads `open` derives (the pass, then such rules, until nothing changes); none is withdrawn within a state |
| 4 | [Q-M1f.609](../../../../open_questions.md#q-m1f609--what-the-blind-enumerator-and-a-later-debt-write-above-root) | what the blind enumerator and a later debt write above root | what root read: the blind enumerator's raw list at root and the later debts' candidates, less what the state holds or denies |
| 5 | [Q-M1f.610](../../../../open_questions.md#q-m1f610--an-obligation-whose-witness-does-not-resolve) | an obligation whose witness step does not resolve under an activator | the call is declined: nothing is written for any obligation, and the blind enumerator does not run beside an activated obligation (`rung` mode `declined`) |
| 6 | [Q-M1f.611](../../../../open_questions.md#q-m1f611--whether-a-claim-may-list-an-open-fact) | may a claim list an `open` fact | no: a claim lists facts of the program's relations (`ein.y`'s `ClaimNested`); `(model (open k))` is a parse error |
| 7 | [Q-M1f.612](../../../../open_questions.md#q-m1f612--an-open-facts-provenance) | an `open` fact's provenance | a pass-written one carries a rule's provenance naming its writer (the obligation, the rule, or `open` for the blind enumerator), printed *which rule … writes*; a written one the loader's source provenance |
| 8 | [Q-M1f.613](../../../../open_questions.md#q-m1f613--a-variable-under-open-in-a-conclusion) | `(open ?x)` in a conclusion | `?x` a parameter of the rule names a relation (what a state owes); a variable the `:match` binds is a hypothesis |
| 9 | [Q-M1f.614](../../../../open_questions.md#q-m1f614--the-two-fixtures-of-the-old-rule-on-open) | the two parse-negative fixtures Q-M1f.66's old shape made | `open_in_match.ein` moves to `examples/features/23_a_rule_reads_open.ein` (its subject, a rule that reads `open`, is legal and flagged); `open_mixed_assert.ein` moves to `examples/broken/load/` with `:why`s, refused at load |
| 10 | [Q-M1f.615](../../../../open_questions.md#q-m1f615--the-warning-found-in-flight-on-stderr) | the in-flight warning on stderr | once per file, at once, and only where the static check did not already flag the program (a flagged program's warning is already printed) |
| 11 | [Q-M1f.616](../../../../open_questions.md#q-m1f616--whether-a-nodes-open-facts-count-as-work) | does a node's `open` facts count as work and against `--region-budget` | no: `region::NodeOut::work` counts the facts saturation built beyond root, the `open` facts aside, in every route's `facts` |
| 12 | [Q-M1f.617](../../../../open_questions.md#q-m1f617--whether-open-is-an-atom-the-blind-enumerator-guesses-about) | is `open` a guessable atom where a program writes it as a name | no: the kernel's word, left out of the blind enumerator's atoms as `absent` is (`hypgen::candidate_objects`) |
| 13 | [Q-M1f.618](../../../../open_questions.md#q-m1f618--absent-and-open-heading-a-fact-under-not-in-a-guards-value-and-in-a-using-list) | `open` and `absent` heading a fact under `not`, in a guard's value and in a `:using` list | names, as under an ordinary relation (Q-M1f.571 read where it is silent): `(not (absent k))`, `(neq ?x (absent k))`, `(q (absent ?x))` load; directly under `open` or `absent`, written at the top and concluded they stay refused |
| 14 | [Q-M1f.619](../../../../open_questions.md#q-m1f619--the-structures-theory-share-under-the-union) | the structure's theory share under the union | the program's own writers built H (`structure.rs`' `declared`), and the relations they read, the rules' and the obligations', are what the universe is made of |
| 15 | [Q-M1f.620](../../../../open_questions.md#q-m1f620--a-hypothesis-rule-that-concludes-an-open-fact) | an hrule whose `:assert` concludes `(open …)` | refused at load: an hrule proposes a fact, a hypothesis already; a rule that concludes `(open …)` is written instead |

## Your call

A number and what you want otherwise, in [`ANSWERS.md`](../ANSWERS.md) under item 69. Blank keeps the fifteen as built.
