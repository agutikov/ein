# 72 · The defaults S1f.21.20 took

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the stage's record](../../../s1f.21.20_what_the_first_uses_of_open_found.md#record)

S1f.21.20 is done. It fixed three defects the first programs written with `open` found in the build, built your
two rulings on where the pass runs, kept the agenda's tables, and found why the gate's build was slow on one
program. These are the nine defaults it took.

## What you should know first

- **The pass as you ruled it** (Q-M1f.655, Q-M1f.656). Above root a state of a search reads only what it owes.
  Its `open` facts are written, and the check in flight made, at a state that is printed or saved, at every state
  of a program in which a rule reads `open`, and at every state under the new flag, **`--in-flight`**, on
  `ein solve`, `ein test`, `ein oracle` and `ein render lattice` (default 6). **What follows**: a program that is
  found not monotone only in flight shows its `in flight` row where a printed state finds it, or under the flag.
- **A printed state's finding is the run's** (default 7): what the states a read-out prints find in flight is
  folded into the read-out, the summary and the oracle's dump, so the three say one thing.
- **The empty commitment's state is root's** (default 1): it holds the `(open F)` facts a program writes; a state
  that commits a fact holds none of them. A program that writes its hypotheses as facts is in the corpus,
  `examples/features/24_written_hypotheses.ein`.
- **The cost.** With the pass where it is used and the agenda's tables kept, against the engine before the stage:
  `04_open.ein` 1 586 to 624 ms, `send_more_money.ein` 4 300 to 2 464 ms, `zebra.ein` 876 to 819 ms, taken on a
  loaded machine. `--in-flight` costs 1.0 to 2.4 times a run without it. The final table, on an idle machine, is
  on [the cost page](../../../costs/README.md) once S1f.21.21 is in.
- **The gate's build**: a debug check walked every rule at every step and took 74 % of a gate run of
  `equation_two_unknowns.ein`; bounded (default 9), the program goes from 4.4 to 1.4 times its release cost, and
  the per-commit cost check passed at sixteen workers.

## The nine

| # | question | what was asked | the default taken | the other reading |
|---|---|---|---|---|
| 1 | [Q-M1f.636](../../../../open_questions.md#q-m1f636--which-state-is-the-empty-commitments-where-a-program-writes-open-facts) | which state is ∅'s where the program writes `(open F)` | the plan's: ∅'s state is root's, the written facts among it; a node that commits a fact holds none | root too drops a written one once it is read into H, so no state holds it |
| 2 | [Q-M1f.637](../../../../open_questions.md#q-m1f637--the-rungs-word-where-a-rules-candidates-stand-beside-an-obligation-that-offers-none) | the rung's word where a rule's candidates survive beside an obligation that offers none | `union`: the rules wrote beside an activated obligation, and H is the union, the obligations' share empty; `stuck` is a state that owes and to which no writer gave a candidate | `hypothesis-rules`, the writer that built H, though an obligation is activated |
| 3 | [Q-M1f.638](../../../../open_questions.md#q-m1f638--a-core-above-the-empty-commitment-that-read-a-written-open-fact) | a core above ∅ whose derivation, made at root, read a written `(open F)` | it lists what the state holds: a fork holds no written `open` fact (Q-M1f.569), and the explanation grounds out a premise the state does not hold, as it does any; at ∅ the state holds them and the core lists them (`build/s20/core-above.ein`: the core of `{(s b)}` is `(s b)`) | list the written fact, a premise the derivation read, though the state does not hold it |
| 4 | [Q-M1f.639](../../../../open_questions.md#q-m1f639--the-saturate-commands-events-and-the-firings-after-roots-open-facts) | `ein saturate --events` and the firings after root's `open` facts | counted and listed by the read-out, not narrated: the stream of `ein saturate` stays the first three states of `05-events.md`'s machine, and its root pass narrates nothing, as before | narrate them as `fire` lines after root's saturation |
| 5 | [Q-M1f.640](../../../../open_questions.md#q-m1f640--when-a-loads-note-is-said) | when a load's `note:` is said | when the run ends, before the load's `warning:` lines, on every command | before the run but after the `WARNING`, which needs the static check's verdict before any note |
| 6 | [Q-M1f.641](../../../../open_questions.md#q-m1f641--the-name-of-the-flag-that-runs-the-pass-at-every-state) | the flag's name and its commands | `--in-flight`, on `ein solve`, `ein test`, `ein oracle` and `ein render lattice`, every command that builds the states of a search; no `(config …)` field | `--open-every-state`, the mechanism's name; or a config flag |
| 7 | [Q-M1f.642](../../../../open_questions.md#q-m1f642--how-what-a-printed-state-finds-in-flight-reaches-the-surfaces) | how what a printed state finds in flight reaches the run's surfaces | the read-out folds the finding of the states it prints into its answer's verdict, where the search found nothing; `ein oracle --dump` writes `region.json` after the read-out, with it; its stderr line is said at once, as every finding's | each surface reports what the search alone found, and a printed state's finding is the stderr line alone |
| 8 | [Q-M1f.643](../../../../open_questions.md#q-m1f643--which-states-are-printed-or-saved) | which states are printed or saved | every `region::Nodes` reader (the read-out's states and forms, the dumps' state files, the trace, a member of N's core, `EIN_LEFTOVER`), `ein oracle --states`' nodes and a saved file's node; a state printed only as a count or a verdict, `--dump-lattice`'s nodes, `--events`' `enter` lines, `--count`, writes none; `ein test` as `ein solve`, a claim listing no `open` fact | every state a dump or a render names, `--dump-lattice`'s among them |
| 9 | [Q-M1f.644](../../../../open_questions.md#q-m1f644--the-cheaper-form-of-the-check-the-gates-build-made-at-every-step) | the cheaper form of task 6's check | count the rules' facts by relation, and walk only where the count moved: no call the full walk would refuse passes, where `Kb::n_facts_of` agrees with the rule-application index, which `Kb::index_fact` keeps together | ask the full walk once a state, at the state's fixpoint, which misses a skip a later walk repairs |

## Your call

A number and what you want otherwise, in [`ANSWERS.md`](../ANSWERS.md) under item 72. Blank keeps the nine as built.
