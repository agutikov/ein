# S1f.10.6 — A debt root was not told of

**Phase:** [P1f.10](README.md), track H (what H is)
**Status:** re-planned 2026-11-15. **Tasks 1 to 3 ran on 2026-11-15** ([§ Record](#record-2026-11-15-tasks-1-to-3)):
the witness and the control re-taken, the pair written for the hrule rung and for the blind rung, the census
taken, answer (A) priced under its two readings, the question written as
[Q-M1f.307](../open_questions.md#q-m1f307--a-debt-root-was-not-told-of-what-h-holds), and the stage stopped at
the question, put at the phase's one stop ([S1f.10.12](s1f.10.12_the_structure_shown.md)). **Ruled 2026-11-16:
(A), by the static reading** (the user: *"all defaults ok"*). **Task 4 ran that day**
([§ Record](#record-2026-11-16-task-4)): the obligations rung proposes at root what a debt a commitment can
bring would, the register's row is `fixed`, the witness alone moved, the read-out has a `later debt` row,
Q-M1e.11 is recorded as ruled. **The stage is done**, but for the gate the main session runs on the batch.
**Depends on:** nothing in the phase.
**Owns:** [Q-M1e.11](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis),
*what happens to an obligation derived under a hypothesis*, ruled 2026-11-16, and the register row
**`W26/oblig-derived-activator`**, `fixed` that day by this stage, `open` and reviewed `deferred` until then
([`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml)).
**Was:** *obligations derived under a hypothesis, the review's `Q6`*: a probe of the walk's tree, whose rung
mode could flip at a node. The tree and its guard were deleted on 2026-11-03. The question was never about the
tree; M1e2's S1e2.2.25 restated it over R and banked the witness, and this stage starts there.

## The question, over R

H is built at root. An obligation's activator is an ordinary fact, so a rule can derive one at a node, and the
facts that would pay the new debt were never candidates.

**The witness**,
[`examples/ein-bugs/wrong-answers/oblig-derived-activator.ein`](../../../examples/ein-bugs/wrong-answers/oblig-derived-activator.ein):
`std.algebra`'s `total-owed` over `likes`, stated, and over `drinks`, whose activator a rule derives from any
`likes` fact. Today (`ein solve` and `ein oracle`, the release binary at `dbedc535`, 2026-11-15): H is the one
candidate `(likes Ann Soup)`; the node that holds it owes *drinks owes Ann a Drink*; nothing pays it; **R is
empty, every alive node owing**. The control, `oblig-derived-activator-declared.ein`, states the activator: H
is two candidates and R one state, `{(drinks Ann Tea), (likes Ann Soup)}`.

**What the engine already says of it.** The static check flags the program: *total-owed: its activator
relation is one a commitment can change*, one site, so it takes the cube and no licence. And on a monotone
program a node that owes an instance root did not **refuses the run** (`tree.rs`, `check_owed`). So the shape
is detected in both classes. What is not decided is the answer.

## The four answers

Q-M1e.11's table, restated over R; the register row carries (A) as its `right` and the other three as
alternatives.

| | what | consequence |
|---|---|---|
| **A** | **H holds what a node's debt would propose.** The universe is closed under the debts a commitment can bring: what would pay an obligation whose activator a node derives is a candidate | the control's R. The universe stops being *root's generator list*: it needs either a static reading of which activators a commitment can derive (`monotone.rs` computes the relations a commitment can change, and follows activators) with the guard's scan run for them, or an iteration over nodes. Its cost and its exactness are T2's to measure |
| **B** | **decline**: such a program's H is the blind enumerator's, as Q-M1e2.10 (a) took for W14 | a larger R that holds the control's state and more |
| **C** | **refuse at load**: every obligation activator must hold at root | a diagnostic instead of an answer, the shape of *forbid*. The static check already finds the site |
| **D** | **the answer stands, and the read-out says why**: the debt's payment is outside H | the row becomes `not-wrong`, with a line in the read-out |

## Tasks

1. **T1f.10.6.1 — the witness and the control re-taken** on the binary as it stands, each read-out kept under
   `build/`, and the same pair written for the hrule rung and the blind rung, to show whether the question is
   the obligations rung's alone.
2. **T1f.10.6.2 — the census.** Over every corpus program: does a rule conclude an activator of an obligation
   rule from a relation a commitment can change (static, from `monotone.rs`'s sites; the `site_census`
   example); and of those within the oracle's reach, does a node owe an instance root did not (by the cube).
   Expected: the witness and few others. And (A) priced on them: the universe each of its two readings gives,
   and R over it by `ein oracle` on a copy that states what the reading would add.
3. **T1f.10.6.3 — the question**, filed under the next free id in
   [`open_questions.md`](../open_questions.md) with the census, and put at the phase's stop. **Default: (A)
   where T2 shows a static reading that gives the control's R on the witness and moves no other corpus
   program's R; otherwise (C).** Both name a wrong answer today; (B) trades it for a different H, and (D)
   keeps an empty R where a model exists. The default is not acted on before the user replies.
4. **T1f.10.6.4 — as ruled.** The fix, with the row flipped in the fixing commit and the control's
   expectation unchanged; under (A), the read-out's `universe` row says what was added and
   [`solution-semantics.md`](../../../docs/ein/reasoning/kb-evolution/solution-semantics.md#12-names) § 1.2 is restated; under
   (C), a `defined_behaviour.md` § 4 section and a `broken/load/` fixture; under (D), the row's status and the
   read-out's line with `read_out.md`. Q-M1e.11 recorded as ruled, in the user's words.
   [`events.md`](../../../docs/ein/ein.rs/events.md)'s pointer to this stage restated to say what the
   engine does.

## Acceptance

- The witness, the control and the census are on the stage's record, each with its command.
- The user has ruled; Q-M1e.11 is closed; W26 is `fixed` or `not-wrong`, and the register has no row left
  whose owner is a stage of this phase.
- **[README § The domain](README.md#the-domain)'s second paragraph is true of the engine after the ruling**,
  and is restated if the ruling changes what H is.
- If an answer moved, it moved by the register row, and the phase's README names this stage as an exception
  to *no answer changes*.

## What it must not do

- **Not change H before the ruling.**
- **Not decide what a node is on a non-monotone program.** That is
  [F21](../../followups/f21_non_monotone_programs_decided.md)'s; this stage's question is what H is, and it
  says where the two meet.
- **Not weaken `check_owed`.** Under any ruling, a monotone program on which a node owes what root did not is
  either inside the new H or still refused.

## The standing rules

A wrong answer is a row in `corpus/wrong_answers.toml`, and the fix flips the row in the fixing commit. Config
never changes an answer. The gate is `./run_tests.sh`, once on the last commit of the batch.

## Record, 2026-11-15: tasks 1 to 3

**No engine behaviour changed, H is what it was, `check_owed` was not touched, no register row was filed or
flipped, no expectation was edited, no corpus program was added.** What was added: one measurement example,
`ein-infer/examples/owed_census.rs`; this record;
[Q-M1f.307](../open_questions.md#q-m1f307--a-debt-root-was-not-told-of-what-h-holds); two findings in the
phase's [`findings.md`](findings.md#from-s1f106) (H6-1, H6-2).

**The binary.** The release build of `14063474` (`cargo build --release -p ein-cli`, in a worktree of the
lane's branch; no file under `ein.rs/crates/*/src` or `stdlib/` differs from `dbedc535`), on AC. The census
example is `8f02d398`'s, built against the same sources. `ein --version`:

```
ein 0.1.0
protocol   ein-events/1
container  einb/2.0
features   einb, parallel, snmalloc
stdlib     sha256:3ed7f9d0f377124b7b8d8a723deaa85931bce8a41c8f597d9e957abe5aa3de8e  checkout …/stdlib
```

**The artefacts** are under the untracked `build/s1f.10.6/`: each program, beside it its `ein solve` read-out
(`X.solve.txt`), its `ein oracle` read-out (`X.oracle.txt`) and dump (`X.oracle/region.json`, `region.md`);
`run.sh` and `run.txt`, the script that writes them and what it printed; `line.py`; `census/rows.tsv`, the
census's row for every corpus program, `census/probes.tsv`, the same example over the probes with
`--iterate`, and `census/past_limit_17.tsv`. `build/` is not tracked, so every program this stage wrote and
every command is below; the copies `run.sh` derives (`X_declared.ein`, `X_a.ein`, `X_r1.ein`, `X_r2.ein`,
`lNN_scan.ein`) are written by its two functions and its `grep` from those programs and from tracked files.
`run.sh` was run twice, and the second run printed what the first did.

### What the engine does, which is what the four answers differ on

- **H is taken once.** `region::prepare_with` runs `hypgen::generate_config_free` at root's fixpoint and
  nowhere else. On the obligations rung that is `oblgen::generate`: the plans `obligations::plans_for` compiles
  from the activator facts **root holds**, the instances each matches at root, and for each owed instance the
  guard's sub-plan scanned with the witness step skipped.
- **A node's debts are read over the node's activators.** `region`'s node function reads them by
  `obligations::tally` on the node's fixpoint, which compiles a plan for every activator fact the node holds.
  So a node owes what a plan root never compiled matches, and nothing generates at a node.
- **The static check sees the shape as an activator site**: a rule with parameters whose name is in `D`
  (`monotone::classify`). `D` is closed over the saturation plans root compiled. `D⁺`, the same followed
  through the rules root has no plan of (`monotone::through_activators`), is read by *forbid* alone.
- **`check_owed` (`tree.rs`) guards a verdict, not this program.** It refuses a run of the obligation tree in
  which a node owes an instance L5's table lacks, and the tree runs only on a program the static check calls
  monotone. A program with an activator site is flagged and takes the cube, which reads every node and
  refuses nothing. The census below holds the premise: on the 560 monotone cubes it read, no node, alive or
  dead, owes an instance root did not.

### T1f.10.6.1: the pair on each rung

The witness and the control are the tracked files, copied to `obl/derived.ein` and `obl/declared.ein`. The
other eight programs were written here. Each row is `ein oracle FILE --dump` read by `line.py`; `ein solve
FILE` answers the same H and R on each (`X.solve.txt`).

| rung | program | what it varies | H | R |
|---|---|---|---:|---|
| obligations | `obl/derived.ein`, the witness | | 1, `(likes Ann Soup)` | **empty**: 2 nodes, both owing; `{(likes Ann Soup)}` owes *drinks owes Ann a Drink* |
| obligations | `obl/declared.ein`, the control | the activator also stated | 2 | 1 state, `{(drinks Ann Tea) (likes Ann Soup)}` |
| hrule | `hr/derived.ein` | each hrule proposes a relation where its activator holds, as the obligations rung does | 1 | **empty**, as the witness |
| hrule | `hr/declared.ein` | the activator also stated | 2 | 1 state, the control's |
| hrule | `hr/static_derived.ein` | the hrules read `is-a` alone | 2 | 1 state, the control's |
| hrule | `hr/static_declared.ein` | the activator also stated | 2 | 1 state, the control's |
| hrule | `hr/reads_h.ein` | **no derived activator**: both debts stated, no rule `arm`, and the hrule that proposes a drink reads `likes` | 1 | **empty**: `{(likes Ann Soup)}` owes the drink the hrule would propose there |
| hrule | `hr/reads_h_stated.ein` | `(likes Ann Soup)` stated | 1, `(drinks Ann Tea)` | 1 state, `{(drinks Ann Tea)}` |
| blind | `blind/derived.ein` | a bare `(open)` rule that owes nothing declines the obligations rung | 12 | 1 024 states; the minimal line is one state, the control's |
| blind | `blind/declared.ein` | the activator also stated | 12 | 1 024 states, the same |

**What it shows.**

1. **The question is not the obligations rung's alone, and it is not the blind rung's at all.** It is the
   question of a generator that reads the state: the obligations rung always, since it reads root's debts; an
   hrule where its `:match` reads a relation a commitment can change; the blind enumerator never, since it
   proposes every fact of every open relation over every name, whatever root holds. On the blind rung the
   witness and its control have one H and one R.
2. **An hrule cannot have a derived activator**: its activators are the `(solve … :hrules …)` list's and
   never a fact of the state (`hrule.rs`). So the hrule rung meets the question only through an hrule's
   `:match`. An hrule whose `:match` writes a variable for a relation is refused at load unless a parameter
   binds it, so `hr/derived.ein` writes one hrule a relation.
3. **`hr/reads_h.ein` is the same loss with no obligation derived at a node, on a program nothing flags.**
   The static check calls it monotone (no rule concludes an activator; an hrule's plan is not read by the
   check), `ein solve` takes the state route and `--single` the obligation tree with L5, `check_owed` has
   nothing to refuse since root owes both instances, and R is empty where the twin that states the `likes`
   fact has a model. It is outside this stage's question as written, which is about an activator, and is
   [H6-1](findings.md#h6-1--an-hrule-that-reads-what-a-commitment-can-change-proposes-at-a-node-what-it-did-not-at-root-and-nothing-flags-it).
4. **`blind/derived.ein` is answer (B) on the witness**, taken as M1e2's S1e2.2.15 took W14's: H is 12
   candidates, R 1 024 states, and the facts no rule refutes or requires are free.

The eight programs.

`hr/derived.ein`:

```
;; S1f.10.6, T1, the hrule rung: the witness with two hrules that propose
;; a relation where its activator holds, as the obligations rung does.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(rule arm () :match (likes ?p ?f) :assert (total-owed drinks is-a) :why "{?p} likes {?f}, which switches on the debt that every Person drinks a Drink")
(hrule guess-likes  () :match (and (total-owed likes is-a)  (is-a ?a Person) (is-a ?b Food))  :assert (likes ?a ?b)  :why "likes is owed; guess: (likes {?a} {?b})?")
(hrule guess-drinks () :match (and (total-owed drinks is-a) (is-a ?a Person) (is-a ?b Drink)) :assert (drinks ?a ?b) :why "drinks is owed; guess: (drinks {?a} {?b})?")
(solve :no-hypothesis (is-a))
```
`hr/declared.ein`:

```
;; S1f.10.6, T1, the hrule rung: hr/derived.ein with the derived activator
;; also stated, as the control states it.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(total-owed drinks is-a)
(rule arm () :match (likes ?p ?f) :assert (total-owed drinks is-a) :why "{?p} likes {?f}, which switches on the debt that every Person drinks a Drink")
(hrule guess-likes  () :match (and (total-owed likes is-a)  (is-a ?a Person) (is-a ?b Food))  :assert (likes ?a ?b)  :why "likes is owed; guess: (likes {?a} {?b})?")
(hrule guess-drinks () :match (and (total-owed drinks is-a) (is-a ?a Person) (is-a ?b Drink)) :assert (drinks ?a ?b) :why "drinks is owed; guess: (drinks {?a} {?b})?")
(solve :no-hypothesis (is-a))
```
`hr/static_derived.ein`:

```
;; S1f.10.6, T1, the hrule rung: the witness with two hrules that read
;; nothing a commitment can change.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(rule arm () :match (likes ?p ?f) :assert (total-owed drinks is-a) :why "{?p} likes {?f}, which switches on the debt that every Person drinks a Drink")
(hrule guess-likes  () :match (and (is-a ?a Person) (is-a ?b Food))  :assert (likes ?a ?b)  :why "guess: (likes {?a} {?b})?")
(hrule guess-drinks () :match (and (is-a ?a Person) (is-a ?b Drink)) :assert (drinks ?a ?b) :why "guess: (drinks {?a} {?b})?")
(solve :no-hypothesis (is-a))
```
`hr/static_declared.ein`:

```
;; S1f.10.6, T1, the hrule rung: hr/static_derived.ein with the derived
;; activator also stated.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(total-owed drinks is-a)
(rule arm () :match (likes ?p ?f) :assert (total-owed drinks is-a) :why "{?p} likes {?f}, which switches on the debt that every Person drinks a Drink")
(hrule guess-likes  () :match (and (is-a ?a Person) (is-a ?b Food))  :assert (likes ?a ?b)  :why "guess: (likes {?a} {?b})?")
(hrule guess-drinks () :match (and (is-a ?a Person) (is-a ?b Drink)) :assert (drinks ?a ?b) :why "guess: (drinks {?a} {?b})?")
(solve :no-hypothesis (is-a))
```
`hr/reads_h.ein`:

```
;; S1f.10.6, T1, the hrule rung with no derived activator: both debts are
;; stated, there is no rule `arm`, and the hrule that proposes a drink reads
;; `likes`, which the other hrule proposes.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(total-owed drinks is-a)
(hrule guess-likes  () :match (and (is-a ?a Person) (is-a ?b Food))  :assert (likes ?a ?b)  :why "guess: (likes {?a} {?b})?")
(hrule guess-drinks () :match (and (likes ?p ?f) (is-a ?d Drink)) :assert (drinks ?p ?d) :why "{?p} likes {?f}; guess: (drinks {?p} {?d})?")
```
`hr/reads_h_stated.ein`:

```
;; S1f.10.6, T1: hr/reads_h.ein with the fact its second hrule reads stated.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(total-owed drinks is-a)
(likes Ann Soup)
(hrule guess-likes  () :match (and (is-a ?a Person) (is-a ?b Food))  :assert (likes ?a ?b)  :why "guess: (likes {?a} {?b})?")
(hrule guess-drinks () :match (and (likes ?p ?f) (is-a ?d Drink)) :assert (drinks ?p ?d) :why "{?p} likes {?f}; guess: (drinks {?p} {?d})?")
```
`blind/derived.ein`:

```
;; S1f.10.6, T1, the blind rung: the witness with a bare `(open)` rule that
;; owes nothing, which declines the obligations rung, so the blind enumerator
;; builds H. It is also answer (B) on the witness.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(rule arm () :match (likes ?p ?f) :assert (total-owed drinks is-a) :why "{?p} likes {?f}, which switches on the debt that every Person drinks a Drink")
(relation never T :why "{?1} never holds")
(rule bare () :match (never ?x) :assert (open) :why "{?x} never holds, so this owes nothing; a bare open names no relation, and the obligations rung declines")
(solve :no-hypothesis (is-a never))
```
`blind/declared.ein`:

```
;; S1f.10.6, T1, the blind rung: blind/derived.ein with the derived activator
;; also stated.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(total-owed drinks is-a)
(rule arm () :match (likes ?p ?f) :assert (total-owed drinks is-a) :why "{?p} likes {?f}, which switches on the debt that every Person drinks a Drink")
(relation never T :why "{?1} never holds")
(rule bare () :match (never ?x) :assert (open) :why "{?x} never holds, so this owes nothing; a bare open names no relation, and the obligations rung declines")
(solve :no-hypothesis (is-a never))
```

### T1f.10.6.2: the census

```sh
cargo build --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example owed_census
ein.rs/target/release/examples/owed_census --jobs 12 > build/s1f.10.6/census/rows.tsv
```

2026-11-15, the oracle's default limit of 16. `owed_census` is a measurement example beside `site_census` and
`object_census`. Its static half is `monotone::classify` over root's plans, with `forbid::extendable` for
`D⁺` and each active hrule plan compiled as the generator compiles it. Its dynamic half builds every node of
the cube by the calls `region`'s node function makes, compares each node's two bits and its count of debts
with `region::compute`'s on a second load (`check`), and reads on each node its debts and, on an alive one,
the generator's list.

**Of 867 corpus files 644 load.** 127 declare an obligation rule, an import bringing its module whole; 82 of
them have a plan of one at root. By rung: blind 461, hrules 88, obligations 62, no-search 21, declined 9,
stuck 3.

**The static half, on all 644.**

| | programs | which |
|---|---:|---|
| an obligation rule's activator relation is in `D` (an activator site of an obligation rule) | 3 | the witness, the control, `examples/lattice/09_activator_hole.ein` |
| … in `D⁺` and not in `D` | 1 | `examples/lattice/12_activator_chain_hole.ein`: `total-owed` is written only by the plan of `serves`, which root never compiled, so it is no site |
| a saturation plan of root concludes an obligation rule's activator | 32 | in 29 the plan reads nothing a commitment can change (`std.bijection`'s `bijective-setup`, `std.slots`'s `slot-partition-setup`): the activator is derived at root from stated facts. In 3 it reads `D`: the first row's |
| an obligation plan with a trigger site (a commitment can add an instance of a plan root has) | 9 | `oblig-stdlib-total-owed.ein` and its `-declared`, `past-limit-lines.ein`, `lattice/04_obligation_trigger_hole.ein`, `lattice/06_friends_tables.ein`, `lattice/synth/{local_not_global,maximal_not_top,trigger_hole}.ein`, `tests/region/07_cube.ein` |
| an active hrule's `:match` reads a relation in `D` | 4 | `hrule-reads-unknown.ein` (`paid`), and `past-limit-lines.ein`, `lattice/04`, `tests/region/07_cube.ein` (`is-a`, which a rule derives of a seated guest) |

**The dynamic half, on the 582 the oracle reaches**: 364 838 nodes, 54 451 of them alive, and every node's
bits and count agree with `region::compute`'s (`check` is `ok` on all 582).

| | programs | which |
|---|---:|---|
| an alive node owes an instance of a `(rule, activator)` root compiled no plan of | 3 | the witness (1 node); `lattice/09` (1) and `lattice/12` (1), both on the hrule rung. Not the control: its activator is at root |
| an alive node owes an instance root did not, of a plan root has | 4 | `lattice/04`, `lattice/synth/local_not_global.ein`, `lattice/synth/maximal_not_top.ein`, `tests/region/07_cube.ein`, all on the hrule rung |
| … and the program is one the static check calls monotone | 0 | of 560 monotone cubes; nor does a dead node of one |
| **an alive node at which the generator proposes a fact that is not in H** | **1** | **the witness**: `{(likes Ann Soup)}` proposes `(drinks Ann Tea)` |
| an alive node whose rung is not root's | 0 | |

So *a node owes what root did not* is seven corpus programs, all flagged, and **the debt's payment is outside
H on one, the witness**. On `lattice/09` and `/12` the hrule `dine` proposes every meal of every guest from
`is-a` alone, so `(meal Ann Soup)` is in H and both have R of 3 states with the owing node `{(opens K1)}` a
hole; what R is on them is F21's and no question of H. On the four programs with an hrule that reads `D`, no
node proposes outside H.

**Not reached.** 43 programs are past the limit, named in `run.txt` and here: `beyond_arith/{cryptarithm,
equation_by_search, equation_no_solution, equation_two_unknowns, send_more_money}`, `branching/{02, 03, 04,
06, 07, 08, 10, 11, 12}`, `ein-bugs/{alt-cap-core, alt-cap-core-reordered, zebra2-hints}`,
`ein-bugs/wrong-answers/{alt-cap-shared, count-past-2-128, count-past-2-64, or-one-key, or-one-key-twin,
past-limit-lines}`, `features/{01, 04, 05}`, `lattice/{06_friends_tables, 06_friends_tables_everyone_sits}`,
the six of `saturation/square-{bwd,fwd}/`, the three of `saturation/square-unique/`, `zebra`, `zebra2`,
`zebra2-hints`, `zebra2-minus-15`, `zebra2-obligations`, `zebra2-minus-15-obligations`. The static half read
them: **none has an obligation rule's activator relation in `D` or `D⁺`**; two have a trigger site
(`past-limit-lines.ein`, n = 17, and `lattice/06_friends_tables.ein`, n = 19) and one of those an hrule that
reads `D`. That one, one candidate past the limit, was read at `--limit 17`
(`census/past_limit_17.tsv`): 131 072 nodes, 4 alive, one of them owing an instance root did not, none
proposing outside H. 19 more are refused by *forbid* on every command and have no R
(`branching/{13, 14}`, `ein-bugs/{activator-all-int, activator-all-nested, activator-all-symbol,
alive-empty-interlayer, d3-unsat-core, naf-upward-closure}`, the six `wrong-answers/forbid-chain-*`,
`wrong-answers/{naf-nowb, stale-naf-fork, stale-naf-fork-warn}`, `lattice/synth/{three_components,
trigger_hole}`); none has an obligation rule's activator relation in `D` or `D⁺`.

### T1f.10.6.2: answer (A) priced, under its two readings

Neither reading was built.

**Reading 1, static.** *The activators a commitment can derive*: for each obligation rule with parameters
whose name is in `D⁺`, each ground `(rule args…)` written by the `:assert` of a rule that reads a relation of
`D⁺` or is activated by one. H₁ is H and what the
obligations rung proposes at root's fixpoint for the plans those activators compile: the instances each
matches there, each one's guard scanned as the rung scans it. A candidate added adds its relation to H and so
to `D`, so the reading is repeated until it adds nothing. It runs no saturation. Here the activators were read
off each program's rules by hand, the scan was taken by the engine on a copy that states them
(`X_declared.ein`; for the two hrule-rung programs `lNN_scan.ein`, the hrules deleted), and R over H₁ by
`ein oracle` on a copy that states H₁ as its universe.

**Reading 2, over nodes.** H\* is the least set that holds H and, for every alive node over H\*, every fact
the generator proposes at that node, the ladder whole and config-free. `owed_census --iterate` computes it,
a round a cube (the `# iterate` lines of `census/probes.tsv`), and R over it; `ein oracle` on a copy that
states H\* answers the same R on every program below.

**A copy that states a universe** is the program with its `(solve …)` line dropped and one hrule with an
empty `:match` a candidate (`run.sh`'s `stated`). An hrule overrides the ladder and is no saturation rule, so
no rule, fact or obligation changes and every node is the program's own. A copy that states the *activator*,
as the control does, is another program: its root owes the debt at once.

**On the four programs the census finds.**

| program | today | reading 1 adds | reading 2 adds | R under either |
|---|---|---|---|---|
| the witness | H 1, R empty | `(drinks Ann Tea)`, the control's H | `(drinks Ann Tea)`, in one round | **1 state, `{(drinks Ann Tea) (likes Ann Soup)}`, the control's** (`price/witness_a.ein`) |
| the control | H 2, R 1 state | nothing: the activator is at root and the scan is root's | nothing: no node proposes outside H | unmoved |
| `lattice/09` | H 2, R 3 states | nothing: the scan gives `(meal Ann Soup)`, which is in H (`price/l09_scan.ein`); and an hrule builds this H, so the obligations rung is not asked | nothing | unmoved |
| `lattice/12` | H 2, R 3 states | nothing, as `/09` (`price/l12_scan.ein`). The activator is in `D⁺` alone: read over `D`, it is not derivable at all | nothing | unmoved |

**Under either reading (A) gives the control's R on the witness and moves no other corpus program's R.**
Reading 1 can add only where an obligation rule's activator relation is in `D⁺`, which is those four of all
644, the 43 past the limit and the 19 refused included. Reading 2 can add only where a node proposes outside
H, which is the witness among the 582 the cube read; the 43 past the limit it did not read, but for the one
above.

**Where the readings part, on programs written for it.** Each is the witness with one thing changed; `today`
and the two readings' R are `ein oracle`'s.

| probe | what it changes | today | activator stated | reading 1 | reading 2 |
|---|---|---|---|---|---|
| `price/partial.ein` | a second food; `arm` fires on Soup alone | H 2, R 1 state, `{(likes Ann Stew)}` | H 3, R 3 states | H 3, **R 4 states** | H 3, R 4 states |
| `price/dead_arm.ein` | `arm` needs two foods of one person, which `one-food` refutes: no alive node derives the activator | H 2, R 2 states | H 3, R 2 states, each with the tea | H 3, **R 4 states: over.** Two hold `(drinks Ann Tea)`, which no alive node owes | H 2, R 2 states: today's |
| `price/var_arm.ein` | `arm` writes the owed relation as a variable, which a stated fact binds | H 1, R empty | H 2, R 1 state | **nothing: under.** No `:assert` writes a ground activator; R stays empty | H 2, R 1 state |
| `price/two_steps.ein` | the tea that pays the new debt switches on a second debt, over `eats` | H 1, R empty | H 3, R 1 state | H 3 in two passes, R 1 state | H 3 in two rounds, R 1 state |
| `price/chain.ein` | `lattice/12`'s chain on the obligations rung | H 1, R empty | H 2, R 1 state | over `D⁺`: H 2, R 1 state. Over `D`: nothing | H 2, R 1 state |
| `price/flip.ein` | the new debt's domain is a relation a commitment can change | H 1, R empty | H 24, the blind enumerator's: the rung declines at root (Q-M1e2.10 (a)). Past the oracle's limit | the same decline, on the plan the reading compiles: **(B)** | the node's rung is `declined` and it proposes the blind enumerator's list: 24 candidates, **(B)**, past the limit |
| `hr/reads_h.ein` (T1) | no activator is derived | H 1, R empty | | **nothing**: it reads activators | H 2, R 1 state (`price/reads_h_r2.ein`) |

**What that prices.**

- **(A) is not *state the activator*.** On `partial` the control's R is 3 states and (A)'s is 4: `{(likes Ann
  Stew)}` is a member, since a person who likes Stew is told of no second debt. The register row's `right`,
  *the control's R*, is (A)'s answer on the witness because every member of R there derives the activator.
- **Reading 1 is exact on the corpus and inexact off it, both ways.** Over on `dead_arm`: it reads which
  activators a commitment *can* derive at relation grain, and proposes for one no alive node derives. Under on
  `var_arm`: an activator it must bind a variable to find. Each of the two is the witness with one rule
  changed, and no corpus program has either shape.
- **Reading 1 must follow `D⁺`.** `monotone.rs`'s `D`, which the static check and the rung's own decline
  read, does not hold `lattice/12`'s `total-owed`. `through_activators` does, and *forbid* is its one reader.
- **Reading 1 does not reach H6-1**, where no activator is derived. Reading 2 does.
- **Reading 2 is exact by its definition and is defined on nodes.** It costs a generator pass on every alive
  node of every round's cube: on `two_steps` the cubes of 1, 2 and 3 candidates where today's is the first.
  Only the cube takes it as it stands: a route that skips a node by a licence would have to show that the
  node proposes nothing. The witness is flagged and takes the cube already; `hr/reads_h.ein` is monotone and
  takes the state route and the tree. **And what a node is on a flagged program is F21's**: reading 2's H\*
  is whatever F21 makes of a node, while reading 1 reads rules and root alone. As priced here it has one more
  dependence: the rung at a node reads `D` over every plan the run's memo holds, those earlier nodes compiled
  among them, so a node's rung could depend on the order the nodes were read in. No corpus node's rung differs
  from root's, and `flip`'s one node declines on the plan it compiles itself.
- **Both readings inherit the existing decline.** Where the new plan's guard scans a relation in `D`, the
  rung declines and the blind enumerator builds H, at root under reading 1 and at the node under reading 2:
  (A) is then (B) on that program (`flip`).
- **(A) leaves every program it touches flagged.** Under (A) `partial`'s R has a hole, `{(likes Ann Stew)}` in
  R, `{(likes Ann Soup) (likes Ann Stew)}` owing, the three together in R, as `lattice/09`'s has today.

The six programs.

`price/partial.ein`:

```
;; S1f.10.6, T2, (A) priced: the witness with a second food, and `arm` fires
;; on Soup alone, so a member of R need not derive the activator.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Stew Food) (is-a Tea Drink)
(total-owed likes is-a)
(rule arm () :match (likes ?p Soup) :assert (total-owed drinks is-a) :why "{?p} likes Soup, which switches on the debt that every Person drinks a Drink")
(solve :no-hypothesis (is-a))
```
`price/dead_arm.ein`:

```
;; S1f.10.6, T2, (A) priced: a rule concludes the activator from a relation a
;; commitment can change, and no alive node derives it: `arm` needs two foods
;; of one person, and `one-food` refutes that.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Stew Food) (is-a Tea Drink)
(total-owed likes is-a)
(rule arm () :match (and (likes ?p Soup) (likes ?p Stew)) :assert (total-owed drinks is-a) :why "{?p} likes Soup and Stew, which switches on the debt that every Person drinks a Drink")
(rule one-food () :match (and (likes ?p ?f) (likes ?p ?g) (neq ?f ?g)) :assert (false) :why "{?p} likes {?f} and {?g}, and a Person likes one Food")
(solve :no-hypothesis (is-a))
```
`price/var_arm.ein`:

```
;; S1f.10.6, T2, (A) priced: the rule that concludes the activator writes the
;; owed relation as a variable, which a stated fact binds.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(relation wanted T :why "a debt over {?1} is wanted")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(wanted drinks)
(total-owed likes is-a)
(rule arm () :match (and (likes ?p ?f) (wanted ?R) (relation ?R ?A ?B)) :assert (total-owed ?R is-a) :why "{?p} likes {?f}, which switches on the debt over {?R}")
(solve :no-hypothesis (is-a wanted))
```
`price/two_steps.ein`:

```
;; S1f.10.6, T2, (A) priced: the fact that pays the debt a node is told of
;; switches on a second debt.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(relation eats   Person Dish :why "{?1} eats {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink) (is-a Pie Dish)
(total-owed likes is-a)
(rule arm   () :match (likes ?p ?f)  :assert (total-owed drinks is-a) :why "{?p} likes {?f}, which switches on the debt that every Person drinks a Drink")
(rule arm-2 () :match (drinks ?p ?d) :assert (total-owed eats is-a)   :why "{?p} drinks {?d}, which switches on the debt that every Person eats a Dish")
(solve :no-hypothesis (is-a))
```
`price/chain.ein`:

```
;; S1f.10.6, T2, (A) priced: examples/lattice/12_activator_chain_hole.ein's
;; chain on the obligations rung. `opens` derives `(serves K1)`, an activator;
;; the plan of `serves` it compiles, which root never had, derives the
;; obligation's activator.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(relation serves Kitchen :why "kitchen {?1} serves")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink) (is-a K1 Kitchen)
(total-owed likes is-a)
(rule opens () :match (likes ?p ?f) :assert (serves K1) :why "{?p} likes {?f}, so kitchen K1 serves")
(rule serves (?k) :match (is-a ?k Kitchen) :assert (total-owed drinks is-a) :why "{?k} serves, which switches on the debt that every Person drinks a Drink")
(solve :no-hypothesis (is-a serves))
```
`price/flip.ein`:

```
;; S1f.10.6, T2, (A) priced: the debt a node is told of is over a domain a
;; commitment can change. `arm` derives the activator of `total-owed` over
;; `drinks` with `kind` for its domain, and `more` derives a `kind` fact, so
;; at the node that plan's guard scans a relation in D and the obligations
;; rung declines there (Q-M1e2.10 (a)); at root it does not.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation kind   T T :why "{?1} is of kind {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food)
(kind Ann Person) (kind Tea Drink)
(total-owed likes is-a)
(rule arm  () :match (likes ?p ?f) :assert (total-owed drinks kind) :why "{?p} likes {?f}, which switches on the debt that every Person drinks a Drink")
(rule more () :match (likes ?p ?f) :assert (kind Juice Drink) :why "{?p} likes {?f}, so Juice is a Drink")
(solve :no-hypothesis (is-a kind))
```

`var_arm.ein` without its `(relation ?R ?A ?B)` premise is refused at load: *its :assert writes (total-owed ?R
is-a), whose ?R is rule 'total-owed''s parameter ?R, which names a relation, and (wanted ?R) can bind ?R to a
fact*.

### The other three answers on the same programs

| | the witness | the control | `lattice/09` | `lattice/12` |
|---|---|---|---|---|
| today | H 1, R empty | H 2, R 1 state | H 2, R 3 states | H 2, R 3 states |
| **A**, either reading | H 2, R 1 state, the control's | unmoved | unmoved | unmoved |
| **B**, decline | H 12, R 1 024 states, the minimal line the control's state (`blind/derived.ein`) | **moves**: H 12, R 1 024 states (`blind/declared.ein`). The decline is at relation grain, as it is for W14's controls | unmoved: an hrule builds H | unmoved |
| **C**, refuse at load | refused | **refused** by the static check's site, which is at relation grain: the activator relation is in `D` whether or not root holds the fact. Accepted only by a test that reads the fact | **refused** | refused only by a test over `D⁺`; over `D`, `total-owed` is no site here |
| **D**, the answer stands | R empty stands, the row `not-wrong`, a line in the read-out | unmoved | unmoved, and no line: the payment is in H | unmoved |

- **(B) moves two corpus programs**, and restates the row's `[case.control]`, as Q-M1e2.10 (a) restated two
  of W14's controls. Neither file states an `:expect`. `object_census`, the example behind Q-M1f.306, marks
  none of the twelve candidates on either ([H6-2](findings.md#h6-2--under-b-q-m1f306-bears-on-two-more-programs-and-marks-nothing-on-them)).
- **(C) refuses three or four corpus programs, two of them not wrong in H today.** `lattice/09` and `/12` are
  the corpus's two exercised activator sites, the third cause of a non-monotone program in
  [`monotonicity.md`](../../../docs/ein/reasoning/kb-evolution/monotonicity.md); `ein-infer/tests/detector.rs` reads
  both by name, `tests/region.rs` the first and `src/detector.rs`'s docs the second, and their renderings are
  in the shape golden.
- **(D) needs a new line.** Today the witness's read-out says where the debt comes from, *site … at stake:
  the status: owes, through a plan a node compiles*, and what is owed, *owing {(likes Ann Soup)} — owes 1:
  drinks owes Ann a Drink.* It does not say that nothing in H pays it: the `outside` row
  (`ein_infer::scope`) names a relation a saturation rule writes or a query reads, and `drinks` is neither.

### Where this question meets F21's

This stage's question is what H is. [F21](../../followups/f21_non_monotone_programs_decided.md)'s is what a
node, and so R, is on a program the static check flags. They meet in four places, and nothing here decides
the second.

1. **Every program in which a rule derives an obligation's activator from a guess is flagged**, before and
   after any of the four answers but (C): a debt a node is told of can make *discharged* not upward-closed.
   H6-1's program, which derives none, is monotone.
2. **(C) takes a class of programs out of F21's table by refusing it**, `lattice/09` and `/12` with it.
3. **Reading 2 of (A) is a function of nodes**, so its H\* moves with F21's ruling; reading 1 does not.
4. **Under (A) the witness's R is read by the cube on a flagged program**, the fork route's R, which is the
   convention F21 has open.

### T1f.10.6.3: the question, and the stop

[Q-M1f.307](../open_questions.md#q-m1f307--a-debt-root-was-not-told-of-what-h-holds): the four answers with
what each does on the witness, the control and the two other programs the census finds; (A)'s two readings
priced; the default as [§ Tasks](#tasks) states it. **The census selects the default's first branch, (A)**:
reading 1 is a static reading that gives the control's R on the witness and moves no other corpus program's
R. **It is not acted on before the user replies.** Q-M1e.11 stays open and is closed by task 4, on the ruling.

### `run.sh` and `line.py`

`EIN_REPO=/path/to/checkout build/s1f.10.6/run.sh > build/s1f.10.6/run.txt`, with the eight programs of T1
and the six of T2 in place under `hr/`, `blind/` and `price/`.

`run.sh`:

```sh
#!/bin/sh
# S1f.10.6, tasks 1 and 2: the witness, its control and the same pair on the
# hrule rung and on the blind rung, each under `ein solve` and `ein oracle
# --dump`; the census over the corpus and over the probes; and answer (A)
# priced under its two readings. It writes beside the programs; $EIN_REPO is
# the checkout (default: two directories up), $EIN_BIN the binary and
# $EIN_CENSUS the census example (defaults: the checkout's release builds,
# `cargo build --release -p ein-cli` and `cargo build --release -p ein-infer
# --example owed_census`). $EIN_JOBS is the census's --jobs (default 8).
cd "$(dirname "$0")" || exit 1
REPO=${EIN_REPO:-$(cd ../.. && pwd)}
EIN=${EIN_BIN:-$REPO/ein.rs/target/release/ein}
CENSUS=${EIN_CENSUS:-$REPO/ein.rs/target/release/examples/owed_census}
JOBS=${EIN_JOBS:-8}
W=$REPO/examples/ein-bugs/wrong-answers
L=$REPO/examples/lattice
$EIN --version | sed "s|$REPO|…|" > version.txt
one() {  # FILE
    b=${1%.ein}
    $EIN solve "$1" > "$b.solve.txt" 2>&1; echo "exit $?" >> "$b.solve.txt"
    rm -rf "$b.oracle"
    $EIN oracle "$1" --dump "$b.oracle" > "$b.oracle.txt" 2>&1; echo "exit $?" >> "$b.oracle.txt"
    printf '%-28s ' "$1"; python3 line.py "$b.oracle"
}
# `stated SRC OUT FACT…`: SRC with its `(solve …)` line dropped and the
# universe stated, one hrule with an empty :match a candidate. An hrule
# overrides the ladder and is no saturation rule, so every node over the
# stated universe is SRC's node.
stated() {
    src=$1; out=$2; shift 2
    {
        echo ";; $src with its universe stated by hrules (S1f.10.6's run.sh)."
        grep -v '^(solve' "$src"
        i=0
        for f in "$@"; do
            i=$((i + 1))
            printf '(hrule h-%d () :match () :assert %s :why "the universe holds %s")\n' $i "$f" "$f"
        done
    } > "$out"
}
# `declared SRC OUT ACTIVATOR…`: SRC with each activator also stated, before
# its `(solve …)` line, as the control states the witness's.
declared() {
    src=$1; out=$2; shift 2
    {
        echo ";; $src with the derived activators also stated (S1f.10.6's run.sh)."
        grep -v '^(solve' "$src"
        for a in "$@"; do echo "$a"; done
        grep '^(solve' "$src"
    } > "$out"
}

echo '== T1: the pair on each rung'
cp "$W/oblig-derived-activator.ein" obl/derived.ein
cp "$W/oblig-derived-activator-declared.ein" obl/declared.ein
for f in obl/derived.ein obl/declared.ein \
         hr/derived.ein hr/declared.ein hr/static_derived.ein hr/static_declared.ein \
         hr/reads_h.ein hr/reads_h_stated.ein \
         blind/derived.ein blind/declared.ein; do
    one "$f"
done

echo '== T2: the census, over the corpus'
mkdir -p census
$CENSUS --jobs "$JOBS" > census/rows.tsv
grep '^#' census/rows.tsv
echo '== T2: the corpus programs past the limit'
awk -F'\t' '$18 == "over-limit" { printf "%s (%s, n = %s, %s)\n", $1, $4, $5, $11 }' census/rows.tsv
echo '== T2: the corpus programs the engine refuses'
awk -F'\t' '$18 == "refused" { printf "%s (%s, n = %s, %s)\n", $1, $4, $5, $11 }' census/rows.tsv
echo '== T2: the one program with a trigger site and an hrule that reads D, one candidate past the limit'
$CENSUS --limit 17 --iterate "$W/past-limit-lines.ein" > census/past_limit_17.tsv
grep -v '^#' census/past_limit_17.tsv | cut -f1,4,5,11,16,17,19-28

echo '== T2: the census, over the probes'
P='price/partial.ein price/dead_arm.ein price/var_arm.ein price/two_steps.ein price/chain.ein price/flip.ein'
$CENSUS --iterate --list 40 obl/*.ein hr/*.ein blind/*.ein $P > census/probes.tsv
grep -v '^#' census/probes.tsv | cut -f1,4,5,11-17,19-28
grep '^# iterate' census/probes.tsv

echo '== T2: (A) priced. Today'
for f in $P; do one "$f"; done
echo '== T2: (A) priced. The activators a commitment can derive, stated: what the obligations rung then proposes at root'
declared price/partial.ein   price/partial_declared.ein   '(total-owed drinks is-a)'
declared price/dead_arm.ein  price/dead_arm_declared.ein  '(total-owed drinks is-a)'
declared price/var_arm.ein   price/var_arm_declared.ein   '(total-owed drinks is-a)'
declared price/two_steps.ein price/two_steps_declared.ein '(total-owed drinks is-a)' '(total-owed eats is-a)'
declared price/chain.ein     price/chain_declared.ein     '(total-owed drinks is-a)'
declared price/flip.ein      price/flip_declared.ein      '(total-owed drinks kind)'
for f in partial dead_arm var_arm two_steps chain flip; do one price/${f}_declared.ein; done
# The two corpus programs on the hrule rung: each with its hrules deleted and
# the activator its rule derives stated, so the obligations rung answers.
grep -v '^(hrule' "$L/09_activator_hole.ein" > price/l09_scan.ein
echo '(total-owed meal is-a)' >> price/l09_scan.ein
grep -v '^(hrule' "$L/12_activator_chain_hole.ein" > price/l12_scan.ein
echo '(total-owed meal is-a)' >> price/l12_scan.ein
cp "$L/09_activator_hole.ein" price/l09.ein
cp "$L/12_activator_chain_hole.ein" price/l12.ein
for f in l09 l09_scan l12 l12_scan; do one price/$f.ein; done
echo '== T2: (A) priced. R over the universe each reading gives, the universe stated'
LS='(likes Ann Soup)'; LT='(likes Ann Stew)'; DT='(drinks Ann Tea)'; EP='(eats Ann Pie)'
stated obl/derived.ein     price/witness_a.ein    "$LS" "$DT"
stated price/partial.ein   price/partial_a.ein    "$LS" "$LT" "$DT"
stated price/dead_arm.ein  price/dead_arm_r1.ein  "$LS" "$LT" "$DT"
stated price/dead_arm.ein  price/dead_arm_r2.ein  "$LS" "$LT"
stated price/var_arm.ein   price/var_arm_r1.ein   "$LS"
stated price/var_arm.ein   price/var_arm_r2.ein   "$LS" "$DT"
stated price/two_steps.ein price/two_steps_a.ein  "$LS" "$DT" "$EP"
stated price/chain.ein     price/chain_a.ein      "$LS" "$DT"
stated hr/reads_h.ein      price/reads_h_r2.ein   "$LS" "$DT"
for f in witness_a partial_a dead_arm_r1 dead_arm_r2 var_arm_r1 var_arm_r2 two_steps_a chain_a reads_h_r2; do
    one price/$f.ein
done
```
`line.py`:

```python
#!/usr/bin/env python3
"""One line for an `ein oracle --dump DIR`: H, R's two counts, and R's
commitments, the smallest first, at most six.

    python3 line.py DIR
"""
import json, sys

d = json.load(open(sys.argv[1] + "/region.json"))
cands = d["candidates"]
head = "H %d: %s" % (len(cands), " ".join(cands) if len(cands) <= 4 else " ".join(cands[:4]) + " …")
if d["region"] is None:
    print(head + " | no cube: past the oracle's limit of %d" % d["limit"])
    sys.exit(0)
nodes = d["region"]["nodes"]
r = sorted((n for n in nodes if n["status"] == "R"), key=lambda n: (n["layer"], n["mask"]))
shown = "; ".join("{%s}" % " ".join(n["facts"]) for n in r[:6])
if len(r) > 6:
    shown += "; and %d more" % (len(r) - 6)
owing = sum(1 for n in nodes if n["status"] == "owing")
dead = sum(1 for n in nodes if n["status"] not in ("R", "owing"))
print("%s | %d nodes, %d owing, %d dead | R %d commitments, %d states%s | %s" % (
    head, len(nodes), owing, dead, len(r), len({n["class"] for n in r}),
    ": " + shown if r else "", "monotone" if d["monotonicity"]["monotone"] else "flagged"))
```

### What contradicts the plan

1. **The question is not the obligations rung's alone** (T1's purpose): the hrule rung has it wherever an
   hrule's `:match` reads what a commitment can change, and in one form with no activator derived at all. The
   blind rung does not have it.
2. ***So the shape is detected in both classes*** holds of a debt root did not owe, and the census holds it on
   560 monotone cubes. It does not hold of H6-1's shape, which is monotone by the check and refused by
   nothing.
3. **The register row carries two alternatives, (B) and (C), not three**: (D) is the row's `now` kept. Its
   (B) reads *not computed here*; it is 1 024 states over 12 candidates.
4. ***The control's R* is (A)'s answer on the witness and not in general** (`price/partial.ein`).
5. ***`monotone.rs` … follows activators***: `D` does not, `D⁺` does, and a static reading over `D` misses
   `lattice/12`'s shape.
6. **`solution_semantics.md` § 1.2 is *Names*.** The universe, which task 4 restates under (A), is § 1.4.
7. **[`events.md`](../../../docs/ein/ein.rs/events.md)'s paragraph that points here** describes the
   walk's `Run::tree_node`, deleted with the walk, and says *no corpus program reaches it — today's stdlib
   activators are all root-asserted*. Three corpus programs derive an obligation's activator at a node. Task 4
   restates it; it is as it was.
8. **[README § The domain](README.md#the-domain)'s second paragraph, *H does not grow*,** is enforced as it
   says for debts on a monotone program. For an hrule's proposals nothing enforces it (H6-1). The structure
   track reads H as root builds it either way.

### Not done

- Task 4, by instruction: nothing is fixed, flipped, refused or restated, and Q-M1e.11 is open.
- Neither reading of (A) was built. Reading 1's activators were read off ten programs by hand; a reading that
  binds a variable in an activator, which `var_arm` needs, was not priced.
- (C) was not prototyped: its row is read off the census's columns.
- The dynamic half was not taken on the 43 programs past the oracle's limit, but for one at `--limit 17`.
- No probe was made a tracked witness and no register row filed, H6-1's included: task 4's and the user's.
- The phase README's status line and stage table are not edited.
- `./run_tests.sh` was not run, by instruction. Run: `cargo fmt --all --check`, `cargo clippy --workspace
  --all-targets -- -D warnings` (both as `run_tests.sh` runs them), `python3 utils/check_hashmap_iteration.py`
  and `python3 utils/doc_audit.py --links --check`.

## Record (2026-11-16): task 4

**The ruling.** The user, on 2026-11-16, shown the phase's four questions with their defaults at the stop:
*"all defaults ok"*. For [Q-M1f.307](../open_questions.md#q-m1f307--a-debt-root-was-not-told-of-what-h-holds)
that is **(A), by the static reading**: H holds what a node's debt would propose, the ground activators of an
obligation rule that a rule reading `D⁺` writes in its `:assert`, with the obligations rung's own scan run for
them at root, repeated until it adds nothing, and no saturation. This task built that and nothing beside it.

**The base.** The worktree's branch was cut at `dbedc535` and fast-forwarded to `a3c719bf`, the commit that
records the ruling, before anything was changed; every *before* below is the release binary of `a3c719bf`.

### What was built, and where

- **The reading**, `forbid::derivable_activators` (`ein-infer/src/forbid.rs`): `D⁺` from a seed, by the one
  reading *forbid* has of the rules (root's saturation plans read by `monotone::reads_of`, every rule with
  parameters read from its pattern, closed by `monotone::through_activators`), and then each conclusion that
  holds no variable, heads an obligation rule with parameters and has that rule's count of arguments, in the
  `:assert` of a rule of which a root plan reads a relation of `D⁺` or which has parameters and its own name in
  `D⁺`. In the rules' load order, a rule's conclusions as written. It reads; it interns, compiles and narrates
  nothing.
- **What it shares with *forbid***: `extendable_over`, `bodies_of`, `Body` and the AST walk are *forbid*'s, as
  they were, and `monotone::through_activators` and `extendable_over` now take the seed where they took the
  static check's verdict and read `m.dynamic` off it. *forbid* passes `m.dynamic`, so its set, its refusals and
  its messages are what they were; the rung passes the relations of its own candidates. The closure of a
  closure being the closure, the two seeds give one `D⁺` on one H. There is no second copy of the closure or
  of the reading of a rule's pattern.
- **The rung**, `oblgen::generate` (`ein-infer/src/oblgen.rs`), restructured so that every plan it reads is
  settled before the first candidate is emitted (a decline must come before any emission, and a later plan can
  decline the call): root's plans are admitted and scanned as before (`admit`, `scan`, cut out of the old body
  unchanged); then, in rounds, the seed is the relations of the candidates the call would emit with every
  switch off (a fact root neither holds nor denies), the reading names the activators, each one root does not
  hold is interned (`compile::ground_fact`) and its plan compiled by `compile_rule` **beside the memo**, the
  new plans are admitted and scanned at root, and the round repeats until it names nothing new. Every round
  holds every plan, root's and later, to the rung's two C4 declines together. Then the accounting and the
  emission: root's as before, in the same order, and the later debts' candidates after them.
- **Beside the memo, and why**: the memo is what the static check, the detector and `scope` read as *the plans
  root compiled*. A later plan in it would have changed the verdict's `D`, its sites and the summaries of the
  program. So a later plan is compiled for the scan and dropped.
- **What is counted**: `RungReport` gains `later_debts` and `later_candidates`; `owed`, `branches`, `declined`
  and `candidates` are root's, as they were. `HypGenStats::later` holds each later debt with the candidates the
  call emitted for it (`oblgen::LaterDebt`), and `region::RootStats::later` the ones H gained a candidate from,
  as text.
- **`scope::outside`** takes the later debts' relations as proposed, so the `outside` row does not name a
  relation H holds a candidate of.

### The surfaces

- **The read-out**: a `later debt` row under `universe`, one per later debt H holds a candidate for, in every
  question (describe, the modes, `--count`, `--monotone`, `--no-search`, `ein oracle`), since each is over the
  same H: the activator, the rules that conclude it, how many candidates would pay it, and the candidates, at
  most four named and the rest counted unless the read-out is uncut (`--dump-answer`'s `answer.txt`). On the
  witness: *`later debt      (total-owed drinks is-a) — a debt root does not owe: rule 'arm' concludes it from
  what a commitment can change. H holds what would pay it, 1 candidate: (drinks Ann Tea)`*. A program whose H
  gains nothing prints no such row, and its `universe` row is what it was.
- **`--json-summary`**: `root.hypgen.later_debts`, one `{obligation, activator, relation, by, added,
  candidates}` each, **written only where there is one**. `outside` and `types_as_objects` are written on every
  run, empty when nothing; this key is not, because the summary is in the shape golden's `cores[…]` lines and
  an always-present key would have moved every corpus program's.
- **`ein oracle --dump`**: `region.json`'s `later_debts`, the same and under the same rule, and a line in
  `region.md`.
- **`--events`**: the `rung` line gains `later_debts` and `later_candidates`, optional, written only where the
  call read a later debt, so `ein-events/1` stays; `--hyp-stats`' report lines the same two, under the same
  rule.

### The corpus diff: the witness alone

Taken three ways, each over every corpus program.

1. **`ein solve FILE -H` and `ein solve FILE --json-summary`**, the release binary of `a3c719bf` against the
   fixing commit's, on all 868 `.ein` files under `examples/`, `tests/` and `stdlib/` (624 write a summary;
   the others are load refusals): the stdout, stderr, exit code and summary differ on **one** program,
   `examples/ein-bugs/wrong-answers/oblig-derived-activator.ein`. (A second file's stderr differs by a path:
   the load error of `broken/load/import_module_not_found.ein` names where the stdlib was looked for, and the
   two binaries were run from two places.)
2. **The shape golden**, `ein-render/tests/golden/corpus_shapes.md5`, per commit: **The shape golden**, `ein-render/tests/golden/corpus_shapes.md5` (`cargo test … --profile gate -p ein-render
   --test id_order_invariance`, per commit, unblessed): *15 of the 17768 renderings of the 600 programs read
   moved*, every one a line of the witness: `hyp`, the five `read-out[…]`, `region-trace`, `states`,
   `region-lattice`, the five `cores[…]` and `oracle`. The witness's other 26 lines did not move (`load`,
   `saturate`, `plan`, `match`, `hyp+closed`, `naf`, the two `explain`, the four `ir[…]`, the fourteen
   `dot[…]`). Re-banked with `EIN_BLESS=1`, per commit: the file's diff is those 15 lines. Then unblessed again
   under `EIN_CORPUS_SLOW=1`, which reads every corpus program, `tests/` and the slow entries too: no line moved,
   and no rendering depends on the id order, the witness's among them.
3. **`read_out_examples.md`**, the read-out's golden page: 46 lines added, X20, and no line of X1 to X19
   changed.

The witness, before and after:

| | H | R | the rung's line |
|---|---|---|---|
| before | `(likes Ann Soup)` | empty, every alive node owing: `{(likes Ann Soup)}` owes *drinks owes Ann a Drink* | `owed 1, branches 1, candidates 1, uncovered 1` |
| after | `(drinks Ann Tea)`, `(likes Ann Soup)` | 1 state, `{(drinks Ann Tea) (likes Ann Soup)}`, the control's | the same four, `uncovered 0`, `later_debts 1, later_candidates 1` |

The control, `oblig-derived-activator-declared.ein`: H 2, R 1 state, unmoved, and no `later debt` row, the
activator being at root. `lattice/09_activator_hole.ein` and `lattice/12_activator_chain_hole.ein` are on the
hrule rung, which the reading does not touch: unmoved.

### The probes, as built against the entry's table

`ein-infer/tests/later_debts.rs` holds each as a program written in the test, by `ein oracle`'s library
(`region::compute`) and by describe (`explore::run`). The same seven files under `build/s1f.10.6/` give the
same lines by `ein oracle --dump` and `line.py` on the fixing commit's binary.

| probe | the entry's reading 1 | built |
|---|---|---|
| the witness | H 2, R 1 state, the control's | H 2, R 1 state, `{(drinks Ann Tea) (likes Ann Soup)}`; one later debt, by `arm` |
| the control | unmoved | H 2, R 1 state; no later debt |
| `partial`: `arm` fires on one food of two | R 4 states | H 3, R 4 states, `{(likes Ann Stew)}` among them |
| `dead_arm`: no alive node derives the activator | **over**: R 4 states, two holding a tea no alive node owes | H 3, R 4 states, the two with the tea among them: **over**, held as that |
| `var_arm`: the rule writes the owed relation as a variable | **under**: no ground activator, R stays empty | H 1, R empty, no later debt: **under**, held as that |
| `two_steps`: the tea that pays switches on a second debt | R 1 state, in two passes | H 3, R 1 state; two later debts, by `arm` and by `arm-2` |
| `chain`: `lattice/12`'s chain on the obligations rung | R 1 state over `D⁺` | H 2, R 1 state; the later debt is by `serves`, the rule root has no plan of |
| `flip`: the new debt's domain is one a commitment can change | the rung's existing decline: (B) | the rung declines, H is the blind enumerator's 24; R not taken, 2²⁴ nodes being past the oracle's limit and the default budget |
| H6-1's program, no activator derived | **nothing** | H 1, R empty, the hrule rung, monotone: unchanged |

A tenth program, the witness under two hrules that propose a relation where its activator holds
(`hr/derived.ein`), is held too: H 1, R empty, as before. The hrule rung is as it was.

### The docs

- [`solution-semantics.md` § 1.4](../../../docs/ein/reasoning/kb-evolution/solution-semantics.md#14-candidate-and-universe)
  says what H is on the obligations rung, with the three steps, the declines a later plan is held to, where
  the reading is over and under, and that it is not *the program with the activator stated*. (The stage file's
  task 4 says § 1.2, which is *Names*; the universe is § 1.4, as task 3's record noted.)
- [`events.md`](../../../docs/ein/ein.rs/events.md): the `rung` row and its section name the two optional
  fields, and the paragraph that pointed here says what the engine does. The section it sits in is the record
  of the walk's tree, and stays one.
- [`read-out.md` § 5.3](../../../docs/ein/user/reference/read-out.md#53-under-universe) has the `later debt`
  row; `read_out_examples.md` has X20, written by `utils/read_out_examples.py`;
  [`answer-dump.md`](../../../docs/ein/user/reference/answer-dump.md) and
  [`region-dump.md`](../../../docs/ein/user/reference/region-dump.md) the JSON key;
  [`architecture.md`](../../../docs/ein/ein.rs/architecture.md), the glossary and
  `06_reserved_names.md`'s ladder sentence say the rung proposes for a later debt too.
- **Q-M1e.11 is recorded as ruled** in
  [`open_questions.md`](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis),
  its table row and its entry, in the form Q-M1e.12's ruling took there: *Ruled 2026-11-16*, the user's words,
  then *Built 2026-11-16*.
- `AGENTS.md`, in place: the ladder sentence, and the register's owners (no open row is a stage's).

### The register

`W26/oblig-derived-activator` is `fixed`: `[case.now]` is one state, `{(drinks Ann Tea) (likes Ann Soup)}`,
which is `[case.right]`; `[case.region]` is route 2, flagged, `k_s = k_c = 1`; its review, `deferred` since
2026-11-05, is `confirmed`, the ruling's, a deferred review being an open row's by the register's own check.
`[case.control]` is as it was. Neither the witness nor the control states an `:expect`, so no expectation
moved. `[case.fixed]` carried the null id in the fixing commit, `168263f1`, and the next commit wrote the hash, the
register's convention.

### Tests and verification

All on the gate profile (`cargo test --manifest-path ein.rs/Cargo.toml --profile gate …`), in the worktree, on
AC, beside the main tree's nightly.

| what | result |
|---|---|
| `-p ein-infer -p ein-corpus`, whole, before the last formatting pass | 51 test binaries, 568 tests, 0 failed |
| `-p ein-infer --lib --test later_debts --test obligation_rung --test region --test register_region --test explore --test claim_routes --test stdlib_coverage`, on the final tree | 129 tests, 0 failed |
| `-p ein-render`, whole: `id_order_invariance` unblessed after the re-bank, `jobs_invariance`, `dot_wellformed`, the goldens | 65 tests, 0 failed |
| `EIN_CORPUS_SLOW=1 … -p ein-render --test id_order_invariance`, unblessed | 4 tests, 0 failed |
| `-p ein-cli`, whole: `wrong_answers`, `import_invariance`, `lever_invariance`, `read_out_reference`, `read_out_examples`, `corpus_cli`, `config_reference`, `answer_dump`, `region_dump`, `summary_properties`, `route_invariance` and the rest | 61 test binaries, 382 tests, 0 failed |
| `-p ein-core -p ein-ir -p ein-einb -p ein-narration -p ein-corpus` | 339 tests, 0 failed |
| `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | clean |
| `python3 utils/check_hashmap_iteration.py`; `python3 utils/doc_audit.py --links --check …`; `python3 utils/stdlib_manifest.py` | clean |

What each of the named invariances holds of this change:

- **`wrong_answers.rs`** runs the row's `[case.now]`, `ein solve` on the witness, and holds its summary to the
  one state; `ein-corpus`'s `the_register_is_well_formed` holds `now` to `right` as data, and
  `register_region.rs` recomputes `[case.region]` by `explore`.
- **The answer does not depend on the import form** (`import_invariance.rs`): the witness under `:symbols`,
  whole and `:as`. The reading resolves an activator's head through the names the loader resolved.
- **Nor on the id order** (`id_order_invariance`): the witness searches, so every rendering of it is taken again
  under a permuted interner. The reading's order is the rules' load order and each rule's conclusions as
  written; the one hash set it builds is read by membership.
- **Nor on any lever** (`lever_invariance.rs`): the seed of the reading is the candidates the call would emit
  with every switch off, read before any filter runs, so the lookahead and its kill cache cannot reach it.
- **Every mode agrees with the oracle** (`explore.rs`, `claim_routes.rs`) on the witness as on every corpus
  program; on the probes `later_debts.rs` holds describe and `--single` to the cube.
- **`read_out_reference.rs`**: the `later debt` label is a row of § 5.3's table, printed by X20, and prints its
  summary fields' values. **`answer_dump.rs`**: the page's `root.hypgen.later_debts` row is a key a summary
  writes, read off the witness's, and the puzzle's summary writes none.

### Defaults taken

Where the ruling left a choice, the one that changes fewer answers.

1. **An activator is read where the `:assert` writes it with no variable, as written.** A conclusion that
   holds a parameter of its own rule is not read, though a plan root compiled of that rule has the parameter
   bound and writes a ground fact. Reading the plans would add candidates on more programs; the entry's words
   are *writes in its `:assert`*. It is the *under* case's rule, and the corpus and the probes do not tell the
   two apart.
2. **`D⁺` is seeded with the relations of the candidates**, the facts the call would emit config-free, as the
   entry says (*a candidate added adds its relation to H and so to `D`*), and not with the relations the rung's
   plans name, which seed the rung's own decline over `D`. The decline's seed is unchanged. A plan that owes a
   relation and proposes no fact of it at root does not make that relation one a commitment can change.
3. **A later plan is held to every decline of the rung**, with root's plans, each round: a bare `(open)`, a
   projection that does not resolve, a scan of a relation the rung proposes, a read of a relation in `D`. The
   entry's `flip` row. The reasons are the existing sentences.
4. **A later debt is not counted as owed.** `owed`, `branches`, `declined`, `candidates`, the `hypskip` line
   and the *stuck* verdict are about what the state owes, as before; a later debt counts in two fields of its
   own. A later debt whose relation a list or a closure takes out proposes nothing and is narrated by no
   `hypskip` ([H6-3](findings.md#h6-3--the-mechanisms-check-reads-roots-activators-so-a-list-that-takes-out-what-a-later-debt-owes-is-accepted)).
5. **`uncovered` is over every plan the rung read**, a later debt's relation among the covered. It moves on
   the witness alone, 1 to 0.
6. **The later candidates are emitted after root's**, so a call that reads no later debt emits what it emitted
   in the order it did, and H's own order is by text either way.
7. **`one_branch` takes no later candidate.** It asks for one instance the state owes; no route calls it
   (S1f.10.16's subject).
8. **A later plan that does not compile under its activator proposes nothing**, silently; the node that
   derives the activator is where the error is made, as today. The loader refuses every shape that could reach
   this but a rule's relation variable bound to a fact, which a direct caller of `compile_rule` alone reaches.
9. **The summary's and the dump's `later_debts` is written only where there is one**, against the block's
   convention of an empty value, so that no other program's golden moves; and only the debts H gained a
   candidate from are listed there and in the read-out, so that a row pairs with an entry. The `rung` line
   counts every later debt the rung read.
10. **The row's review is `confirmed`**, dated the ruling, its bundle the commit at which the stop was
    prepared, `51fe98b0`.
11. **The probes are programs in the test's source**, not corpus files: `ein-infer`'s tests keep such probes
    that way (`oblgen`'s, `scope`'s, `region.rs`'s written programs), and a corpus file would have been a
    second program whose goldens this commit creates.

### Findings

[H6-3](findings.md#h6-3--the-mechanisms-check-reads-roots-activators-so-a-list-that-takes-out-what-a-later-debt-owes-is-accepted):
the mechanisms check reads root's activators, so a list that takes out the relation a later debt owes is
accepted, and the node that derives the activator is stuck; the twin that states the activator is refused at
load. The same before and after.
[H6-4](findings.md#h6-4--hypgeneligible_relations-does-not-name-a-later-debts-relation):
`hypgen::eligible_relations` does not name a later debt's relation. No corpus program differs. Neither is
fixed.

### Not done

- **Reading 2 was not built, and reading 1 was not improved toward it**: the *over* and *under* cases are held
  as the reading gives them, and H6-1's shape is untouched. `check_owed` is untouched.
- **No probe is a corpus file** (default 11), so the invariance sweeps over the corpus read the witness and
  the control, and the probes are held by their own test.
- **`flip`'s R is not held**: its universe is, and that the rung declines.
- **`owed_census` was not re-run over the corpus.** Its generator is the engine's, so its `grow` column on the
  witness is now empty; its doc says so. Tasks 1 to 3's rows under `build/s1f.10.6/census/` are the census of
  the engine before this task.
- **The structure track's census was not re-taken.** [§ The order](README.md#the-order) says a ruling of track
  H that changes H re-takes it for the programs it names: that is the witness, one program, whose H went from
  one candidate to two; `build/structure/` is the main tree's and was not written here.
- `./run_tests.sh`, the nightly and `utils/ci_container.sh` were not run, by instruction: the main session
  runs one gate for the batch.
