# P1f.10 — the close, prepared (2026-11-16)

**Phase:** [P1f.10](README.md). Prepared 2026-11-16, once every stage was merged. **The user ruled Q-M1f.311 the
same day, option (e), which adds [S1f.10.17](s1f.10.17_describe_by_borders.md), describe by borders: the close waits
for it, and for [S1f.10.18](s1f.10.18_no_edge_no_death.md) and [S1f.10.19](s1f.10.19_a_dead_node_stopped_early.md),
which the user took up the same day (*"Yes, both"*). All three are merged, and the user's answers at the close are
built ([§ The user's answers at the close](#the-users-answers-at-the-close-2026-11-16)).** It follows the pattern of
[P1f.6's close](../p1f.6_calculator/close.md): where the phase stands, what each track delivered, the acceptance
re-taken, the defaults, the findings, and what is left for the user. It proposes no `docs/history/` record: this
directory stays where it is. [`state.md`](state.md), written at the pause, is folded in here and is kept only as the
record of the pause.

## Where the phase stands

- **Every stage is done**: S1f.10.1 to .6 and .9 to .19; S1f.10.7 dropped and S1f.10.8 closed at the re-plan. The
  commits of .1 to .16 are in [`state.md` § The stages](state.md#the-stages); S1f.10.17 is `e546ad49`, S1f.10.18
  `f9cc1153` (a written *no*), S1f.10.19 `f70552d5` and `5e761612`. No worktree or branch is left.
- **The user's rulings at the stop** (Q-M1f.306 to .310, 2026-11-16) are built: [`open_questions.md`](../open_questions.md)
  has a *Built* paragraph on each.
- **Every question of the phase is ruled**: Q-M1f.306 to .313, the last three at the close.
- **The verification of the last batch**: [§ The gate](#the-gate).

## What each track delivered

### Track S — the structure of the hypothesis set

One owner, `ein_infer::structure`, computes each object, and the census, the bundle, the surface and the routes read
it there:

- **the pair layer**: dead alone, implies, equivalent, excludes, interacts;
- **the theory's share**: the small KB's exclusions;
- **groups**: the cover, maximal cliques of the theory's exclusions;
- **owed groups, grids and bijections**;
- **the coupling**: the hypergraph of questions, with debts as edges, read by the pair layer and by the whole node.

It is shown three ways:

- `ein render h-space FILE`: a read-out, four graphs as DOT, or `h-space.json`;
- `ein show hyps FILE`: H as Ein facts, one per line;
- the review bundle, `utils/structure_bundle.py`.

It is defined in [`hypothesis_space.md`](../../../docs/kernel/inference/hypothesis_space.md). On `zebra2.ein` the 36
groups are the 36 rows and columns of the five declared bijections, every one owed.

### Track R — spending the structure on the search

Measured before built ([S1f.10.13](s1f.10.13_the_price_list.md), the rule for *go* written before the table), then
built where it paid. Each figure is the `work` row of `ein solve FILE QUESTION`, in saturations.

| built | what it does | where it pays |
|---|---|---|
| **L8**, what a state denies is dead above it ([S1f.10.3](s1f.10.3_the_restricted_join.md)) | a state that stores `(not b)` is not extended by `b`; every route asks one memory, `known_dead` | `zebra2.ein --single` 45 → 9, its describe 374 → 263; `zebra2-minus-15.ein --maximal` 2 346 → 104 |
| **the order** ([S1f.10.5](s1f.10.5_ordering.md)) | the tree branches on the owed instance closest to closing a coupling, read off the whole node in one saturation | `send_more_money_written.ein --single`: stopped after 131 134, now *Solution* in 18 030; ships for six of the eight questions the tree answers |
| **L9**, a fact nothing proposes and nothing writes is in no state ([S1f.10.14](s1f.10.14_exactly_one_spent.md)) | an instance whose unpayable witnesses no rule writes is licensed | `lattice/06_friends_tables_everyone_sits.ein --single` 207 → 61; *go* for all eight questions, no loser |

After the three, `zebra2.ein --single` is 7 saturations and `zebra.ein --single` 9, where the cube is 2⁵⁶.

Then, on the user's rulings of the same day:

| built | what it does | where it pays |
|---|---|---|
| **describe by the tree and the classes** ([S1f.10.17](s1f.10.17_describe_by_borders.md), Q-M1f.311 (e)) | where root owes a licensed instance, describe and `--count` take R's states from the exhausted tree and each state's class from dualize and advance inside it; N and an empty R's frontier are not computed and say so; `--describe-route states` asks the state route | over the 173 monotone programs that search, 1 234 652 saturations → 605 068; `send_more_money.ein`'s describe decided, one state in 18 127, where it stopped after 336 101 |
| **a dead node stopped at its first contradiction** ([S1f.10.19](s1f.10.19_a_dead_node_stopped_early.md)) | on a monotone program's routes; the oracle and root run to quiescence; a printed core still taken at the fixpoint | facts: `--single` −21 %, `-n 1` −19 %, `--maximal` −22 %, describe −3 % |
| **the fail-first agenda** (S1f.10.19, Q-M1f.313 (c)) | every saturation fires first what leads to `(false)`, a stored negation or a fact whose negation can be concluded, then FIFO | `--single` and `-n 1` −6.6 % facts beyond the stop; no answer moved on a monotone program |

The whole border route, N found by dualize and advance over H, was measured *no* for describe on the corpus and is
not built. [S1f.10.18](s1f.10.18_no_edge_no_death.md)'s licence, *a state that closes no coupling cannot die*, is
false as stated (two programs banked) and is not built.

**Measured as a loss, or not built, each with its next way** (in the stages' records):

- **The pair layer or the small KB paid per program before a run**: *no* for every question. The structure costs most
  programs more than their answer.
- **The pair of a death**: *go* for describe, `--count` and `--minimal`, but worth 433 saturations under describe on
  top of L8. Not built: it needs the state route cut into steps.
- **The core of a death**: *go* in the work's units, dearer in wall time on the zebras.
- **The minimal line's order**: *no*. It keeps the written order.
- **A witness a candidate implies, standing for it**: false, with the witness
  [`13_witness_of_two.ein`](../../../examples/lattice/13_witness_of_two.ein). L9 is the narrower licence that holds.

**Losers named:**

- `send_more_money.ein --single` is 18 030 where it was 17 747 under the order, inside the tenth.
- Two `--minimal` cells that the budget stops both ways visit more nodes for the same facts under L8.

How R is found, with every licence, is written as formulas in
[`finding_r.md`](../../../docs/kernel/inference/finding_r.md).

### Track H — what H is

| question | ruled | built | answers that changed, each by its register row |
|---|---|---|---|
| Q-M1f.307, a debt root was not told of | (A), the static reading | [S1f.10.6](s1f.10.6_obligations_under_hypothesis.md)'s task 4 | W26: the witness answers its control's R |
| Q-M1f.306, what the blind enumerator takes for an object | E-1 (b) wide, E-7 (a), E-54 (d) | [S1f.10.9](s1f.10.9_what_the_blind_enumerator_takes_for_an_object.md)'s task 4 | W96, four rows: a kernel dunder is never an object; a `relation names` row in the read-out |
| Q-M1f.308, the rung a loaded rule chooses | (b) | [S1f.10.15](s1f.10.15_the_rung_a_loaded_rule_chooses.md)'s task 4 | W97, three rows; one probe that loaded is refused at load by *forbid* |

No other corpus program's answer moved in the phase. [S1f.10.16](s1f.10.16_what_the_walk_left.md) deleted what the walk
left unread in the generator, and moved no golden.

## Acceptance, re-taken

**Track S**

- **Defined in writing, one owner**: [`hypothesis_space.md`](../../../docs/kernel/inference/hypothesis_space.md) and
  `ein_infer::structure`. The stages' records name the program that falsified each rejected alternative.
- **Held by invariances**, each a test:
  - the excluded pairs against N: `ein-infer/tests/pair_layer.rs`;
  - the I-Z2 control: `cover.rs` and `exactly_one.rs`;
  - import forms and a permuted id space: `structure_invariance.rs`;
  - I-L02's one coupling: `coupling.rs`.
- **Shown before a surface was built**: the bundle at the stop. **Then by the surface the user ruled**:
  `ein render h-space` and `ein show hyps`.
- **Its cost is stated per program**, in the census and in `ein render h-space`'s `work` row.
- **It moves no answer**: the structure reads nodes, and the routes take only what track R built.

**Track R**

- **Measured before built**: the price list, its rule committed first.
- **R identical in every mode**: `explore.rs` against the oracle, `lever_invariance`, `claim_routes`, `known_dead.rs`'s
  run with and without the denials, `order.rs`'s every order, `unheld.rs`. No expectation was edited by a track-R
  stage.
- **The discovery counted as work**: the whole node a coupled order reads is one saturation in the `work` row.
- **The work delta per program and question, losers named**: in each stage's record.
- **Every golden named before it moved**: each stage's inventory commit.
- **No lever**: every switch is internal and thread-local, and `config_reference.rs` gained no row.
- **A way that does not pay is recorded and the track goes on**: above.

**Track H**: each question ruled in the user's words and built, each wrong answer a register row filed `open` and
flipped in its fixing commit.

## The gate

On `f089cc24`, every stage merged: `utils/ci_container.sh`, the gate in the runner's image, **green**;
`utils/ci_container.sh --nightly` **red** at `deep-corpus-shards 4/4`'s `answer_round_trip`, a floor a shard's eighth
held only by the way the corpus dealt it (seventeen programs, fourteen states, every state file reloading). The
floor is fixed as `979c0966`, and all four shards and the per-commit run are green here. The nightly stopped there,
so its later steps have not run on part 2; S1f.10.17 changes a route, so the container's gate and nightly run once
more on the batch's last commit after it.

On `bb3579d1`: the container gate ran `lever_invariance` for over 45 minutes on one cell and was stopped; the
cause was the library taking every dead node's core by default (fixed, `a9b3c9e4`). On `f7f7494c`: the gate failed
on `alive_invariant`'s list, which the two `examples/beliefs` programs should have joined (`604b0e55`), and the
nightly on `first_contradiction`'s per-shard floor of cores (`661f8739`). The whole workspace and every sharded
nightly sweep, each of its four shards, are green here on `661f8739`. **The last run**: *(the container gate and
nightly on `661f8739`, filled in when they finish)*.

## The defaults

What the stages of part 1 decided is Q-M1f.310, accepted by the user (*"all defaults ok"*). What part 2 decided
without a ruling, each in its stage's *Defaults taken*:

1. **`ein render h-space`** is the name, the main session's choice between the user's two examples (`h-structure`,
   `h-space`).
2. **S1f.10.3**: the two `--minimal` cells the budget stops both ways were read as the same budget buying more nodes,
   not as losers, and L8 was kept for `--minimal`. The tree's choice of instance reads the memory of denials, so
   `zebra2-minus-15.ein --single` lists another state of R than before.
3. **S1f.10.5**: the order's share is a twentieth and its threshold twenty saturations, both constants; the minimal
   line keeps the written order by the rule's letter.
4. **S1f.10.9**: the generator's narration in the shape golden, `hyp` on 22 programs and `hyp+closed` on 185, was
   re-banked as the consequence of the ruling. No program's H, R, read-out or exit code moved.
5. **S1f.10.14**: L9 was built although the plan built its T2 only if T1 held: T1 failed and L9 is a narrower licence
   with its own argument, checked on every node.
6. **S1f.10.15**: the re-plan's probe was banked in its dead-root form, its refusal as register data.
7. **S1f.10.16**: the fifteen retired config names stay in `FIELDS`, because the `.einb` container writes it whole.

## The findings

All are in [`findings.md`](findings.md). Those that still want an owner, with a proposed disposition each, **the
user's to confirm**:

| finding | what | proposed |
|---|---|---|
| **H9-1** | `eq` written as a value is never proposed as an object | reads as a wrong answer: file it as an `open` register row, owned by a followup |
| **H9-4** | a `(__symmetric__ R)` a rule derives does not mirror `R`; the same symmetry written as a fact does | reads as a wrong answer: the same |
| **H6-1** | an hrule that reads what a commitment can change proposes at a node what it did not at root, and nothing flags it | [F21](../../followups/f21_non_monotone_programs_decided.md), the static check's |
| H6-3, H6-4 | the mechanisms check and `eligible_relations` read root's activators only | F21, with H6-1 |
| **K-12** | `domain_elim_vs_hypothesis.md` states three things false today | P1f.20, the documentation tree, which follows |
| **K-13**, its second half | `std.algebra`'s `functional` and `injective` conclude `(false)` and store no negation, so SEND gets nothing from L8 | a stdlib change of its own, with its programs under `tests/stdlib/` |
| K-19 | a support's check is dropped past the budget; only the census example asks for it | one line (`Space::cached`), with the next change to the owner |
| K-21 | a describe the budget stops saturates ⊤ though its memory says ⊤ is dead | with Q-M1f.311 |
| K-29, K-30 | the walk's no-good store in `ein_core::Kb`, and `hypgen::is_solution_node` / `consistent` | deleted with their one test, as S1f.10.16 did the rest |
| H15-6 | `register_region.rs`'s bless writes a region block into a quoted note | fixed in the test, small |
| K-5, K-9 | the classes of program the small-KB rule does not fit | stay as the rule's stated limits |

## The user's answers at the close (2026-11-16)

Asked as four questions, each with its options; the user chose:

1. **Q-M1f.312: *"(e) Remove both"*.** The `hyp` and `hyp+closed` ops leave the shape golden, and what only they
   read (`hypgen::generate`, the lookahead and its kill cache, `closed::emit_closed`) is deleted.
2. **H9-1 and H9-4: *"File and fix them in this phase"*.** Each an `open` register row with its witness, then fixed
   and flipped before the close.
3. **The other findings: *"Accept as proposed"*.** H6-1, H6-3, H6-4 and K-41 to [F21](../../followups/f21_non_monotone_programs_decided.md);
   K-12 to P1f.20; K-13's stdlib half a stdlib change of its own, later; K-21 with describe's next work; and K-19,
   K-29, K-30 and H15-6 fixed now, before the close.
4. **Part 2's defaults: *"Accept all"*.** Each stands until revised.
5. **H9-1, asked back when its stage found two rulings deciding it opposite ways:** *"while any inner fact can be
   unpacked - all of them must be the same valid facts as others - so apply all the same rules to them"*. Read as
   (b): a kernel word written as a value is no object, as it is no name at the top level; not a wrong answer, closed
   with nothing moved. H9-4 was filed (W98) and fixed.

So the close waits for three pieces of work and the last gate: the ops removed with the walk's last code (with K-29
and K-30); H9-1 and H9-4 filed and fixed; K-19 and H15-6 fixed. The container gate on `bb3579d1` found one more:
`lever_invariance`'s per-commit sweep ran for more than 45 minutes on one unit, after `zebra.ein` left the slow
set; it is found and fixed with them, and the user asked why the test is not spread over the workers more finely
(*"currently lever_invariance runs in a single thread, we already parallelized tests, why not this?"*): its units
are a program and a setting, every question of one unit run in turn.

## What the user is asked

1. **[Q-M1f.311](../open_questions.md#q-m1f311--a-describe-the-budget-would-stop-answer-what-the-tree-can-reach-instead)**,
   *should a describe that the budget would stop answer what the tree can reach instead?* **Ruled 2026-11-16: (e)**,
   a describe that visits each predicate's borders instead of every alive state; built by
   [S1f.10.17](s1f.10.17_describe_by_borders.md).
2. **[Q-M1f.312](../open_questions.md#q-m1f312--what-the-shape-goldens-hyp-and-hypclosed-ops-digest)**, *what the
   shape golden's `hyp` and `hyp+closed` ops digest*: today the walk's generator, which no command runs. Options:
   - **(a)** keep both;
   - **(b)** re-point `hyp` at H's own pass and remove `hyp+closed`;
   - **(c)** re-point `hyp` only;
   - **(d)** remove `hyp+closed` only;
   - **(e)** remove both.

   **Default (b).**
3. **The findings' dispositions** above.
4. **The part-2 defaults** above, each standing until revised.
