# S1f.5.8 — A node with no contradiction has no core: `null`, not `[]`

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.8**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).
**Estimate:** half a day.
- An hour to list every surface and golden that writes a core.
- Two hours for the two writers and the test.
- An hour for the pages.

**Status:** specified 2026-11-07 on the user's ruling of that day on D-14
([Q-M1e2.27](../../m1e2_correctness/open_questions.md#q-m1e227--what-do--and-null-mean-in-eins-json)).
**Not started, and not to start until the user says so.** No engine code, test, golden or fixture has
moved.
**Depends on:** nothing.

## The ruling

The agent put D-14 as a spelling: an owing hole is written `"core": []` where S1e2.3.9 spells *no core*
`null`. The user, 2026-11-07: *"[] - meaning is something like "empty set"; null - meaning is "some single
object does not exist"; which fits better in which case?"*

The agent's reading of that rule: a core is **one object, a set of facts**, the given and committed facts
that together derive a node's contradiction. A node that is not dead has no contradiction, so its core does
not exist: `null`. `[]` is the empty set, a core that exists and holds no fact, *"the empty set of facts
suffices"*. That is a different claim, kept for that case alone. The user: *"Yes, plan it that way"*.

| case | does the core exist? | spelled |
|---|---|---|
| a dead node | yes, a set of facts | a list: `["(p a)"]` |
| a dead node whose contradiction needs no fact | yes, the empty set | `[]` |
| a node that is not dead: in R, or owing | no | `null` |
| a dead node past `--region-limit` whose core the run did not keep | yes, not kept | `null`, beside `core_status: "not-kept"`, which says why (S1e2.3.9) |

`refuted_by` reads the same way. A dead node's is a list of rules, one per fact of its contradiction, with
`(given)` where the core holds the fact. A node that is not dead has none: `null`.

The summary already spells its holes this way, one level up: `holes.minimal: []` means *computed, and none*,
and `holes: null` means *not known*. D-14 is the one place the two meanings merged.

## The case

Taken on 2026-11-07 at `916d97ac`, on the release binary, on `lattice/09_activator_hole.ein`, whose R is
three states and whose one hole, `{(opens K1)}`, owes 1:

| surface | node | written today |
|---|---|---|
| `ein solve --json-summary`, and `answer.json` and `index.json` under `--dump-answer`: `region.holes.minimal` | the owing hole | `"core": [], "core_status": null` |
| the same files: `region.core` | root, alive | `"core": null` |
| `ein oracle --dump`'s `region.json`: `region.nodes` | every node that is not dead: the 3 members of R and the hole | `"core": [], "refuted_by": [], "core_status": null` |

The inventory found the first row, on 8 holes of 7 programs. The re-take found the third, on every node that is
not dead. A dead node is written its core and rules (`branching/01_saturate_only`'s dead nodes: a core,
`within-budget`).

**A core that is the empty set does not occur today.** A contradiction that needs no fact needs a rule with
no premise, and `:match ()` never fires. That is
[F12](README.md#f12-an-empty-match-never-fires), filed the same day and not this stage's.

## What it changes

- **The summary's holes** ([`region_answer.rs`](../../../ein.rs/crates/ein-render/src/dump/region_answer.rs)
  line 304): a hole that is not dead is written `"core": null`. `--json-summary` and `--dump-answer`'s two
  files take it from the one writer.
- **The oracle's nodes** (the region dump's writer): a node that is not dead is written `"core": null` and
  `"refuted_by": null`.
- **What stays.** A dead node's core and rules, `core_status` everywhere, and `nogoods.json`'s `not-kept` rows.
  No answer moves, and no register row.
- **What moves.** The goldens that hold an oracle dump or a summary with a node that is not dead. Step 1 lists
  each, including `ein-cli/tests/region_dump.rs`'s.

## The method

### Task T1f.5.8.1 — every surface listed

Every writer of `core` and `refuted_by` in `ein-render` and `ein-cli`, the files each reaches, and the goldens
that hold them. `utils/region.py` and `region_census.py` read the dump, and step 1 checks that neither reads a
non-dead node's `core`.

### Task T1f.5.8.2 — `null`

The two writers, and a test in `ein-cli/tests/answer_round_trip.rs`, which already reads every corpus dump:
over every corpus program, every node and hole that is not dead has `core` and `refuted_by` `null`, and every
dead one has a list.

### Task T1f.5.8.3 — the pages

`region_dump.md`'s `nodes` row, `answer_dump.md`'s summary fields, and `read_out.md`'s summary block, each
with the rule: `null`, the core does not exist; `[]`, the empty set. The inventory's D-14 says what fixed it.

## Acceptance

- Every JSON surface writes `"core": null` for a node that is not dead, and `"refuted_by": null` where it
  writes that field, on every corpus program.
- A dead node's core and rules are unchanged.
- No answer moves.
- `./run_tests.sh` is green.
