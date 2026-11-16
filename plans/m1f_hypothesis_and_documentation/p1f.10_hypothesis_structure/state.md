# P1f.10 — where the phase stands

**Written 2026-11-16, at a pause the user asked for** (*"pause after current agents finish, write down the phase
state somewhere in it's plans"*). This page is the place to resume from. It is a snapshot: the stage files hold
the records, [`findings.md`](findings.md) what was found outside a stage's scope, and
[`open_questions.md`](../open_questions.md) the rulings (Q-M1f.305 to .310). When the phase closes this page is
folded into its close and deleted.

## In one paragraph

Part 1 (everything measured, nothing spent) is done, and the gate and the nightly passed on its last code commit,
`4a07010f`. The user was shown the bundle, the price list and the four questions at the phase's one stop and
ruled all of them on 2026-11-16. Part 2 is under way: the surface, the fix of Q-M1f.307 and the first search
improvement are merged into `master`; three agents were still running when the pause was asked, each in its own
worktree, and nothing new is started after them. **Nothing merged since `4a07010f` had been through the gate, the
nightly or the runner's container** when this was written; § Owed says where that stands. Nothing is pushed.

## The stages

| stage | track | subject | state | on `master` as |
|---|---|---|---|---|
| [S1f.10.1](s1f.10.1_exclusion_census.md) | S | the pair layer | done | `706f9b47` |
| [S1f.10.10](s1f.10.10_the_theorys_share.md) | S | the theory's share | done | `fa040a33` |
| [S1f.10.2](s1f.10.2_groups.md) | S | groups, the cover | done, the second run | `ad5a4fc5` |
| [S1f.10.4](s1f.10.4_bounded_groups.md) | S | exactly one: owed groups, grids | done | `b14c6da3` |
| [S1f.10.11](s1f.10.11_coupling.md) | S | coupling, the hypergraph of questions | done; K-17 fixed | `9f80ea3e`, `025c7b38` |
| [S1f.10.13](s1f.10.13_the_price_list.md) | R | the price list | done | `85fb476b` |
| [S1f.10.12](s1f.10.12_the_structure_shown.md) | S | the structure shown | done: bundle, stop, `ein render h-space`, `ein show hyps`, the page | `42cf4e73`, `d8b31e22`, `7886425b`, `025c7b38` |
| [S1f.10.6](s1f.10.6_obligations_under_hypothesis.md) | H | a debt root was not told of | done: Q-M1f.307 (A), W26 fixed | `3e59b90a` |
| [S1f.10.3](s1f.10.3_the_restricted_join.md) | R | at most one, spent | done for denials (licence L8); the pair of a death re-priced, still *go*, **not built** | `f8751257` |
| [S1f.10.9](s1f.10.9_what_the_blind_enumerator_takes_for_an_object.md) | H | what the blind enumerator takes for an object | done: Q-M1f.306, W96 fixed, the `relation names` row | `bc3eb522` |
| [S1f.10.5](s1f.10.5_ordering.md) | R | the order on the obligation tree | done: the order built for six of eight questions, E-29 closed | `2e7cde48` |
| [S1f.10.15](s1f.10.15_the_rung_a_loaded_rule_chooses.md) | H | the rung a loaded rule chooses | done: Q-M1f.308 (b), W97 fixed, E-51 closed | `060576ec` |
| [S1f.10.14](s1f.10.14_exactly_one_spent.md) | R | exactly one, spent | done: T1 *no* (`13_witness_of_two.ein`), licence L9 built, Q-M1f.311 put | `44889fba` |
| [S1f.10.16](s1f.10.16_what_the_walk_left.md) | — | what the walk left | done: the walk's unread generator pieces deleted, the read ones kept with their reader named, Q-M1f.312 put | `50f3e01c` |
| S1f.10.7, S1f.10.8 | — | dropped; closed | stubs | — |

## What the user ruled at the stop (2026-11-16)

The entries in [`open_questions.md`](../open_questions.md) hold the user's words.

| question | ruling |
|---|---|
| Q-M1f.309, the surface | (a), a view of `ein render`. Then, the same day: the view shows only the hypothesis set's structure, so it is renamed (the main session chose `ein render h-space` of the user's two examples, `h-structure` and `h-space`; the user may change it); and a plain listing of H as ein facts is a new top-level command, `ein show hyps` (*"ein render writes dot, by listing hypothesis I meant listing them as ein facts, so add one more top level command `show` and one sub-command `show hyps` that lists them"*) |
| Q-M1f.306, objects | E-1 (b), **wide**: a kernel dunder is never an object, one a program writes as a value among them. E-7 (a): left, and the read-out names such candidates. E-54 (d): left |
| Q-M1f.307, a debt root was not told of | (A), by the static reading |
| Q-M1f.308, the rung | (b): the second rung only where an obligation is activated at root |
| Q-M1f.310, part 1's defaults | accepted (*"all defaults ok"*) |

## The branches that were running at the pause

Each agent works in a git worktree under `.claude/worktrees/` on a branch `worktree-agent-<id>`, cut from
`master`, and is merged with `git merge --no-ff` (never squashed: a register row names its fixing commit by hash).
[§ What happened to them](#what-happened-to-them) below is filled in as each one ends.

None is running now: all six are merged into `master` and their worktrees and branches removed.

S1f.10.3's untracked price-list runs, which its record's tables were read from, were kept as `build/s1f.10.3/`.

### What happened to them

- **`worktree-agent-a3b0431c091872649`, merged as `025c7b38`.** `ein render structure` is `ein render h-space`, with
  no alias; its document is `h-space.json`, its page
  [`hypothesis_space.md`](../../../docs/kernel/inference/hypothesis_space.md). `ein show` is a new top-level command
  with one subcommand, `ein show hyps FILE`: H as Ein facts, one a line, uncut, for root's saturation and the
  generator. K-17 is fixed in the owner (`Space::cached`, `Coupling::payers_read`). It filed K-19, open: the
  support check's node read is dropped the same way past the budget, and only the census example asks for it.
  Its tests were re-run on the merged tree and pass; the gate was not run.

- **`worktree-agent-a5f864753ca542a7f`, merged as `bc3eb522`.** S1f.10.9's task 4, Q-M1f.306 as ruled. The blind
  enumerator takes no kernel dunder for an object (wide: one a program writes as a value among them); the
  register's W96, four rows, filed and fixed; no existing program's H, R, read-out or exit code moved. The
  generator's narration moved and was re-banked on the main session's decision: `hyp` on 22 programs, `hyp+closed`
  on 185, each named in the fixing commit `4ee131e3`. E-7: a `relation names` row under `universe`, on one corpus
  program. E-54: nothing built. It filed H9-4 and H9-5. The goldens and the tests the merged branches share were
  re-run on the merged tree and pass; the gate was not run.

- **`worktree-agent-ac394c3d42ce474da`, S1f.10.5, merged as `2e7cde48`.** Its first agent was cut off by a usage
  limit in T4; a second finished it from the worktree. The tree branches on the owed instance closest to closing a
  coupling read off the whole node alone (`structure::NodeCoupling`, one saturation, no pair layer), then the one
  with the fewest alternatives left. By the price list's rule it ships for six of the eight questions the tree
  answers; the minimal line, whole and under `--single`, keeps the written order (*no*: `send_more_money_written`
  stopped both ways, and the `zebra2-minus-15` pair). `send_more_money_written.ein --single` is decided, one
  state in 18 030 saturations, where it was stopped after 131 134; `send_more_money.ein` 18 030 where it was
  17 747. E-29 closed. Its measurement files are kept as `build/s1f.10.5/`. Its worktree and branch are removed.

## Owed before anything else is built

**Taken on 2026-11-16, on `037b7f55`** (the six part-2 branches merged): `utils/ci_container.sh`, the gate in the
runner's image, **green**; `utils/ci_container.sh --nightly` **red** at its first sharded step, `corpus_cores`' slow
half: the coupled order narrated the whole node as a dead node with the core `[]` and no status, on the two SENDs
under `--maximal`. Fixed on `master` as `a8fa30d3` (the node's core taken where the run is narrated; six lines of
the shape golden moved), the slow test green here. The nightly stopped there, so its later steps have not run on
part 2: **the container nightly is owed again**, once, on the batch's last commit after S1f.10.14, S1f.10.15 and
S1f.10.16 are merged, with the container gate beside it.

1. **One `./run_tests.sh` on `master`'s head**, then **`./run_tests.sh --nightly`** (a route changed, and nightly
   steps were added), then **`utils/ci_container.sh`** (DOT writers changed: the h-space view's graphs and
   `region_dot.rs`). None has run on anything merged since `4a07010f`. The partial runs made at each merge are in
   the stage records and are not the gate.
2. **Walls on a quiet machine, on AC**, four P-core threads, `EIN_TEST_JOBS=4`: the nightly steps that are new or
   grew, each against its five minutes: `known_dead`'s slow half (277 s measured under load, near the line; shard
   it if it is over), `dot_wellformed` with the structure sweep, `structure_invariance`, `coupling`.
3. **`corpus_cli`'s slow flag** on `send_more_money.ein` failed once under load in S1f.10.3's worktree (8 404 ms
   recorded, 36 630 swept); re-take on a quiet machine before believing either number.
4. `build/structure/` (the bundle and the censuses, untracked) predates the fix of Q-M1f.307, L8 and the rename:
   the bundle shows part 1 as the user saw it. Re-take it only if it is to be shown again.

## Then, in this order

1. **S1f.10.17, describe by borders**, on the user's ruling of Q-M1f.311, (e): planned and running.
2. **The container gate and the container nightly**, once, on the batch's last commit, as § Owed says, and the
   walls of the nightly steps that grew on a quiet machine.
3. **The phase's close**: every default taken in part 2 shown to the user, Q-M1f.311 and Q-M1f.312 put to them, the
   findings below, this page folded in.

**Also written on 2026-11-16, at the user's request**: [`finding_r.md`](../../../docs/kernel/inference/finding_r.md),
how the oracle and the routes find `R`, as formulas: the licences L0 to L9, what a refuted candidate does, and how
`--single` proves one state.

## Defaults taken in part 2 that the user has not seen ruled

- **`ein render h-space`** as the name, of the user's two examples.
- **S1f.10.3, `--minimal`**: the corpus total in saturations rose (231 846 to 235 969) because two cells the
  budget stops both ways visit more nodes for the same facts; over the 157 cells that finish it fell and none is
  higher. By the letter of the price list's rule those two are losers; the step was kept.
- **S1f.10.3, the tree's choice of instance reads the memory of denials**, so the tree's order moved before
  S1f.10.5: `zebra2-minus-15.ein --single` and `-n 1` list other states of R than before.
- **S1f.10.3, the pair of a death not built** though still *go* for describe, `--count` and `--minimal`: it is
  worth 433 saturations under describe and needs the state route's layer cut into steps.
- **S1f.10.9, the goldens of the generator's narration re-banked** (`hyp` on 22 programs, `hyp+closed` on 185)
  as the consequence of the ruling, no program's H, R, read-out or exit code moving.
- **S1f.10.14, L9 built though the plan builds T2 only if T1 holds**: T1's claim is false, and L9 is a narrower
  licence with its own argument (`unheld.rs`), its premise checked on every node; *go* for all eight questions the
  tree answers with no loser, on two programs.
- **S1f.10.15, the re-plan's probe banked in its dead-root form**, and the refusal banked as register data rather
  than under `examples/broken/load/`.
- Each stage's own list is under *Defaults taken* in its record.

## Findings that want the user

All are in [`findings.md`](findings.md); these read as wrong answers or stale statements and have no owner.

- **H9-1**: `eq` written as a value is never proposed as an object. Reads as a wrong answer; no register row.
- **H9-4**: a `(__symmetric__ R)` a rule derives does not mirror `R`; the same symmetry written as a fact does.
  Reads as a wrong answer; no register row.
- **H6-1**: an hrule that reads what a commitment can change proposes at a node what it did not at root; the
  static check calls the program monotone and nothing flags it. (A) by the static reading does not reach it.
- **H6-3**, **H6-4**: the mechanisms check and `eligible_relations` read root's activators only.
- **K-12**: `docs/kernel/inference/domain_elim_vs_hypothesis.md` states three things false today.
- **K-13**, its second half: `std.algebra`'s `functional` and `injective` conclude `(false)` and store no
  negation, so I-SEND gets nothing from L8.
- **K-21**: a describe its budget stops saturates ⊤ though its own memory says ⊤ is dead.
- **K-19**: a support's check is dropped without a word where root's layer has spent its budget (the census
  example alone asks for it).
- **H15-6**: `register_region.rs`'s bless inserts a region block at the first `[case.right]` text in a row, a
  quoted note's included.
- **K-29**: `ein_core::Kb`'s learned no-good store is the deleted walk's, and its doc calls it part of the engine.
- **K-30**: `hypgen::is_solution_node` and `consistent` are the walk's definition of an answer, read by one test.
- K-5, K-9: the classes of program the small-KB rule does not fit. K-20: stale counts in a test's comment.

## How to resume

Read this page, then `git log --oneline 4a07010f..master` and `git worktree list`. If a branch above is unmerged,
read its stage file's record on the branch first. Run what § Owed lists before starting a stage. The standing
rules are `AGENTS.md`'s; for this phase: a stage per subagent, the main session reviews and merges; both tracks
are required and track R reads track S's results; no lever changes R or the structure; a way of spending the
structure that does not pay is a result about that way and does not close the track.
