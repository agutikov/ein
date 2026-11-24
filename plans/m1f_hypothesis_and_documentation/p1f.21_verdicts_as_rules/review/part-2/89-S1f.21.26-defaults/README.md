# 89 · The defaults S1f.21.26 took

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the stage's record](../../../s1f.21.26_the_key_of_a_state_at_its_own_size.md#record)

S1f.21.26 is built (`24aeebbe`): your answer to item 78, *"(b)"*. At every state the node function now reads the
state's own facts above root's mark and sorts those, where it sorted every fact the state holds and filtered root's
back out. Five choices it made that your answer did not; each is built, and a word from you changes it.

## What you should know first

- **No key moved**: `ein solve -p` and the digest of the events are byte for byte the same on all 944 corpus files,
  on the release build and on the gate's, where a check held the new key to the old one at every node and never
  fired; the oracle's `--states` dump is the same on all 944 too. A probe over 13 modes found no state whose key fell
  back to the old walk.
- **What it saves** (the costs page's final table, its fourth column; one worker, pinned, on AC):

  | program | instructions, before → after | time, before → after |
  |---|---:|---:|
  | `send_more_money.ein` | 10 441 M → 5 539 M (0.53) | 2 112 ms → 1 238 ms (0.59) |
  | `send_more_money_written.ein` | | 1 410 ms → 819 ms (0.58) |
  | `equation_two_unknowns.ein` | 3 292 M → 2 867 M (0.87) | 491 ms → 439 ms (0.89) |
  | every other entry | within 1 % | within 4 % |

  On `send_more_money.ein` the key's 38 % of the instructions and the filter's 9 % are replaced by under 1 %.

## The five

The key is read at its own size where a search takes it, at every node. A few readers take a state's key once a
printed state or a saved file, and compare it with a state built from scratch, which has no mark to read above:
those keep the whole key.

| # | question | the default taken | the other reading |
|---|---|---|---|
| 1 | [Q-M1f.695](../../../../open_questions.md#q-m1f695--the-oracles-listing-of-a-node-is-read-over-every-fact-it-holds): the oracle's listing of a node | read over every fact the node holds, as before: it is no key, and its text is sorted | read above root's mark, root's `open` facts added from the snapshot |
| 2 | [Q-M1f.696](../../../../open_questions.md#q-m1f696--the-read-outs-check-of-paths-keeps-the-whole-key): the read-out's check of a printed state's paths | the whole key, beside a fresh saturation over the whole knowledge base | both sides read above the mark |
| 3 | [Q-M1f.697](../../../../open_questions.md#q-m1f697--the-stored-key-of-a-saved-solution-stays-whole): `.einb`'s stored key of a solution | the whole key: the file format is unchanged | the part beyond root's, the format moved |
| 4 | [Q-M1f.698](../../../../open_questions.md#q-m1f698--the-axes-keep-roots-facts): `Axes::root_facts` | kept, for the listing and the gate's check | dropped |
| 5 | [Q-M1f.699](../../../../open_questions.md#q-m1f699--a-function-with-no-caller-is-left-as-it-is): `canon::distinct_by_state`, which nothing calls | left as it is | removed |

## Also found

- [V-106](../../../findings.md#v-106--a-function-that-reads-a-key-has-no-caller): that function has no caller, and
  its comment says three sites use it.
- [V-107](../../../findings.md#v-107--an-instruction-count-depends-on-how-the-program-is-run): an instruction count
  of a program of many small states depends on how it is run (`pets.ein` 3 509 M under `bench_env.sh`, 3 642 M from
  a plain shell, 4 230 M unpinned, one binary), so two sessions' rows do not compare; the costs page says so beside
  the two programs whose figures moved on every column.

## Your call

A number and what you want otherwise, in [`ANSWERS.md`](../ANSWERS.md) under item 89. Blank keeps the five as built.
