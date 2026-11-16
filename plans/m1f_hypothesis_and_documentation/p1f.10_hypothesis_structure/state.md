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
worktree, and nothing new is started after them. **Nothing merged since `4a07010f` has been through the gate, the
nightly or the runner's container.** Nothing is pushed.

## The stages

| stage | track | subject | state | on `master` as |
|---|---|---|---|---|
| [S1f.10.1](s1f.10.1_exclusion_census.md) | S | the pair layer | done | `706f9b47` |
| [S1f.10.10](s1f.10.10_the_theorys_share.md) | S | the theory's share | done | `fa040a33` |
| [S1f.10.2](s1f.10.2_groups.md) | S | groups, the cover | done, the second run | `ad5a4fc5` |
| [S1f.10.4](s1f.10.4_bounded_groups.md) | S | exactly one: owed groups, grids | done | `b14c6da3` |
| [S1f.10.11](s1f.10.11_coupling.md) | S | coupling, the hypergraph of questions | done; K-17 open, its fix on a running branch | `9f80ea3e` |
| [S1f.10.13](s1f.10.13_the_price_list.md) | R | the price list | done | `85fb476b` |
| [S1f.10.12](s1f.10.12_the_structure_shown.md) | S | the structure shown | bundle, stop, surface and page done; **the rename and `ein show hyps` on a running branch** | `42cf4e73`, `d8b31e22`, `7886425b` |
| [S1f.10.6](s1f.10.6_obligations_under_hypothesis.md) | H | a debt root was not told of | done: Q-M1f.307 (A), W26 fixed | `3e59b90a` |
| [S1f.10.3](s1f.10.3_the_restricted_join.md) | R | at most one, spent | done for denials (licence L8); the pair of a death re-priced, still *go*, **not built** | `f8751257` |
| [S1f.10.9](s1f.10.9_what_the_blind_enumerator_takes_for_an_object.md) | H | what the blind enumerator takes for an object | tasks 1 to 3 done; **task 4 on a running branch** | — |
| [S1f.10.5](s1f.10.5_ordering.md) | R | the order on the obligation tree | **on a running branch** | — |
| [S1f.10.15](s1f.10.15_the_rung_a_loaded_rule_chooses.md) | H | the rung a loaded rule chooses | tasks 1 to 3 done; **task 4 not started**, ruled (b) | — |
| [S1f.10.14](s1f.10.14_exactly_one_spent.md) | R | exactly one, spent | not started | — |
| [S1f.10.16](s1f.10.16_what_the_walk_left.md) | — | what the walk left | not started | — |
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

| branch | work | base |
|---|---|---|
| `worktree-agent-a5f864753ca542a7f` | S1f.10.9 task 4: W96 filed open (four rows), the wide E-1 fix, E-7's read-out naming | `3e59b90a` |
| `worktree-agent-a3b0431c091872649` | the rename to `ein render h-space`; `ein show hyps`; K-17 fixed in the owner | `2050a886` |
| `worktree-agent-ac394c3d42ce474da` | S1f.10.5, the order | `f8751257` |

Three merged branches' worktrees are still on disk and can be removed with their branches:
`worktree-agent-a601d26e3778f9c83` (S1f.10.6), `-a09b7d8b049cf055e` (the surface), `-a68ac81f0382b6c04` (S1f.10.3).

### What happened to them

*(filled in as each ends)*

## Owed before anything else is built

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

1. **S1f.10.15 task 4**, Q-M1f.308 (b). After S1f.10.9's branch is merged: under (b) `features/09`'s read-out
   needs the count E-7's naming adds. The entry says no tracked program's answer shows the change, so the task
   adds a witness.
2. **S1f.10.14**, exactly one, spent. After S1f.10.5 is merged (both edit `tree.rs`). S1f.10.3 left it this: a
   wiped-out owed group is now visible to the tree before any saturation. S1f.10.13 left it the *alive and owing*
   shares, and that no visited state of `zebra.ein` owes only unlicensed instances.
3. **S1f.10.16**, what the walk left. After the track-H branches are merged (it edits `hypgen.rs`). It also
   inherits the `hyp+closed` digest, which runs a pass no command calls.
4. **The phase's close**: every default taken in part 2 shown to the user, the findings below, this page folded
   in.

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
- K-5, K-9: the classes of program the small-KB rule does not fit. K-20: stale counts in a test's comment.

## How to resume

Read this page, then `git log --oneline 4a07010f..master` and `git worktree list`. If a branch above is unmerged,
read its stage file's record on the branch first. Run what § Owed lists before starting a stage. The standing
rules are `AGENTS.md`'s; for this phase: a stage per subagent, the main session reviews and merges; both tracks
are required and track R reads track S's results; no lever changes R or the structure; a way of spending the
structure that does not pay is a result about that way and does not close the track.
