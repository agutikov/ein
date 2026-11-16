# S1f.10.15 — The rung a loaded rule chooses

**Phase:** [P1f.10](README.md), track H (what H is)
**Status:** **done 2026-11-16.** Planned 2026-11-15, at the re-plan. **Tasks 1 to 3 ran on 2026-11-15**
([§ Record](#record-2026-11-15-tasks-1-to-3)): the witnesses rebuilt and banked, the census taken, the
question written as [Q-M1f.308](../open_questions.md#q-m1f308--the-rung-a-loaded-obligation-rule-chooses),
stopped at the phase's one stop ([S1f.10.12](s1f.10.12_the_structure_shown.md)). The user ruled **(b)** on
2026-11-16 (*"308 (b) ok"*), and **task 4 ran that day** ([§ Record](#record-2026-11-16-task-4)): the inventory
first, the register's W97 filed `open`, the switch, the rows flipped, the ladder restated. **An exception to *no
answer changes***: W97's three witnesses, one of them a program that loaded and is refused at load.
**Depends on:** nothing in the phase.
**Owns:** E-51 of the calculator's [`findings.md`](../p1f.6_calculator/findings.md), given to this phase on
2026-11-14 and until the re-plan in no stage file.

## The finding

Hypothesis generation is a ladder: an hrule's proposals if the program declares one, else what would pay the
debts the state owes, else the blind enumerator. **The second rung is taken wherever the program *loads* an
obligation rule**, activated or not (`hypgen::generate_with`, `!program.obligations.is_empty()`), and a second
rung that owes nothing at root proposes nothing. So the blind enumerator does not run and H is empty.

That was M1d's scope rule, *discharge where stated*, read by the obligation rules a program states. Two things
have changed under it:

- **Every import brings its module whole.** A program that imports `std.algebra` for `symmetric` loads
  `total-owed` and `surjective-owed` with it, so it is on the second rung without having stated a debt, and
  its own open relations are not guessed about. *States* no longer means what it meant.
- **The mechanisms are read as declared at root.** `mechanisms.rs` reads an obligation as *a plan root's
  activators compile*: an obligation rule nothing activates is not among a program's mechanisms there, and it
  still chooses the rung here. Two readers, two readings of one declaration.

The witnesses of 2026-11-14, on the binary of that day: `(total-owed at is-a)` discharged at root, H empty, R
{∅}; the same with the activator deleted and the import kept, H empty, R {∅}; with the import deleted too, 22
candidates and R at least 110 057 states at the budget.

And one met at the re-plan, 2026-11-15, the release binary at `dbedc535`: a program that imports
`std.algebra :symbols (connex)`, declares `lt` over three nodes, states two `lt` facts and names no obligation
anywhere. Its read-out: *universe — no candidate: R is {∅} or empty, decided by root alone*, and *outside — lt
— no candidate: **no obligation owes it***. The program stated no obligation; the import did.

## Tasks

1. **T1f.10.15.1 — the three witnesses rebuilt** under `build/` on the binary and the stdlib as they stand,
   each read-out kept, and a fourth: a program that imports `std.algebra :symbols (symmetric)` beside a
   declared relation of its own that nothing closes.
2. **T1f.10.15.2 — the census**, by the `mechanism_census` example: the corpus programs that load an
   obligation rule no fact of root activates; of those, the ones with a relation the blind enumerator would
   propose about; and for each, R under each option below, by `ein oracle` within its limit.
3. **T1f.10.15.3 — the question**, under the next free id, put at the phase's stop:
   - (a) **as it is**: the rung is chosen by a rule loaded. The read-out says so where it bites: a `universe`
     row that names the rule which took the program off the blind enumerator, and `(solve :no-search)` remains
     the way to say *judge root alone* on purpose.
   - (b) **by an obligation activated at root**: the second rung where root's activators compile a plan of an
     obligation rule, the reading `mechanisms.rs` has. A program that imports a module for something else is
     on the third rung again. A discharged state under an activated obligation still proposes nothing and is
     complete, so *complete means discharged* is untouched.
   - (c) **by a debt owed at root**: the second rung only where root owes. Rejected by M1d for a reason that
     stands, that a discharged root would fall to the blind enumerator and exhaustion would be back in charge
     of completeness; listed so that the reply can say so.

   **Recommended: (b).** It makes the two readers agree and makes an import of one name not change what is
   guessed. **It restates a ruling of M1d's**, so it is the user's to take and is not acted on as a default.
4. **T1f.10.15.4 — as ruled.** Under (b): every corpus program whose R moves listed first, each a register
   row filed `open` before the fix where today's answer is wrong by the ruling; the switch in
   `hypgen::generate_with`; `06_reserved_names.md` § Hypothesis / query control and the ladder's sentence in
   `oblgen.rs`'s and `hypgen.rs`'s headers restated; the rows flipped in the fixing commit. Under (a): the
   read-out's row, with [`read-out.md`](../../../docs/ein/user/reference/read-out.md) and its reference test in
   the same commit. E-51 closed in the calculator's `findings.md` with what was done.

## Acceptance

- The witnesses and the census are on the stage's record, each with its command and date.
- The user has ruled, in their words, in `open_questions.md`.
- Under (b): `import_invariance` still green, every moved R on the list made before the change, and the
  phase's README names this stage as an exception to *no answer changes*.
- Under (a): a program that an unactivated rule took off the blind enumerator says so in its read-out.

## What it must not do

- **Not change the ladder before the ruling.**
- **Not reopen what a discharged state under a stated obligation proposes**: nothing, and it is complete.
- **Not touch what the blind enumerator proposes** once it runs: that is
  [S1f.10.9](s1f.10.9_what_the_blind_enumerator_takes_for_an_object.md)'s.

## The standing rules

A wrong answer is a row in `corpus/wrong_answers.toml`, filed `open` before its fix. Config never changes an
answer. The gate is `./run_tests.sh`, once on the last commit of the batch.

## Record, 2026-11-15: tasks 1 to 3

**No engine behaviour changed, the ladder is what it was, no register row was filed, no expectation was
edited, no corpus program was added.** What was added: one measurement example,
`ein-infer/examples/rung_census.rs`, and `examples/shared/marks.rs`, the marks of `object_census` moved to
where both examples read them (`object_census` prints over the corpus what it printed, byte for byte, against
`build/s1f.10.9/census/rows.tsv`); this record;
[Q-M1f.308](../open_questions.md#q-m1f308--the-rung-a-loaded-obligation-rule-chooses); five findings in the
phase's [`findings.md`](findings.md#from-s1f1015) (H15-1 to H15-5).

**The binary.** The release build in the lane's worktree at `c6251db3` (`cargo build --release -p ein-cli`;
no file under `ein.rs/crates/*/src` or `stdlib/` differs from `dbedc535`), on AC. The census example is
`e432b657`'s, built against the same sources. `ein --version`, under the script's `$EIN_STDLIB`:

```
ein 0.1.0
protocol   ein-events/1
container  einb/2.0
features   einb, parallel, snmalloc
stdlib     sha256:3ed7f9d0f377124b7b8d8a723deaa85931bce8a41c8f597d9e957abe5aa3de8e  $EIN_STDLIB …/stdlib
```

**The artefacts** are under the untracked `build/s1f.10.15/`: `w/`, the witnesses, and `b/` and `c/`, the
copies that stand for options (b) and (c), each program with its `ein solve` read-out (`X.solve.txt`), its
`ein oracle` read-out (`X.oracle.txt`) and dump (`X.oracle/`); `b/corpus/` and `c/corpus/`, for every corpus
program an option moves, the copy, the original's read-out (`X.today.txt`), the copy's (`X.solve.txt`) and
its summary (`X.json`); `census/rows.tsv`, the census's row for every corpus program, `census/witnesses.tsv`,
the same over `w/`, `census/readouts.txt`, `census/object.tsv` and `census/mechanisms.json`; `run.sh` and
`run.txt`, the script that writes all of it and what it printed; `skips.py` and `line.py`. `build/` is not
tracked, so every program this stage wrote and every command is below; the copies are written by `run.sh`'s
`declined` and its `sed` from those programs and from tracked files. `run.sh` was run again after the census
example's last build and printed what it had, and `rows.tsv` came out the same byte for byte.

### What the engine does, which is what the three options differ on

Each premise of [§ The finding](#the-finding) was read against the sources and holds.

- **The switch** is `hypgen::generate_with`: with no hrule, `!s.kb.program().obligations.is_empty()` sends the
  call to `oblgen::generate`, and only its `Mode::Declined` falls through to the blind enumerator. With no
  activator at root `obligations::plans_for` returns no plan, so nothing is owed, nothing is emitted and the
  mode is `Obligations`: H is empty. The same switch is written a second time, in
  `hypgen::eligible_relations`, which `shape.rs` reads ([§ What contradicts the plan](#what-contradicts-the-plan)).
- **`(solve :no-search)` and an hrule come first**, so the switch is read only on a program that declares
  neither.
- **`mechanisms::read` reads an obligation as a plan root's activators compile**
  (`Mechanisms::obligations`, from `obligations::plans_for`), and keeps the loaded rules beside it
  (`Mechanisms::obligation_rules`). `mechanism_census` prints the difference as `O(no activator)`: 45
  programs.
- **Every import brings its module whole**, so `std.algebra`'s `total-owed` and `surjective-owed` are loaded
  by every program that imports `std.algebra`, `std.bijection`, `std.closure`, `std.typing` or
  `std.bool.inputs`, and `std.slots` brings its own two (the census's `rules` column on the stdlib's files).
- **What M1d ruled.** The decision table of
  [M1d's README](../../../docs/history/m1d_satisfiability/README.md#the-decisions-and-who-took-them) has two
  rows: *obligations supersede `:hrules` as the generator, as a ladder* (user, 2026-08-24), and *the read-out
  is scoped — discharge where stated, exhaustion where not* (2026-08-25, with no name in its *taken* column).
  The second was a rule about the verdict, and
  [`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md) records that under R it is
  gone: *under R there is one definition for every program*. What is left of *where stated* is the switch
  above, and its reason is `oblgen.rs`'s header: *falling back to the blind enumerator whenever nothing was
  owed would put exhaustion back in charge of completeness*. That reason is about a rung that owes nothing
  **under a stated obligation**, which is option (c)'s case. It does not speak of a rule nothing activates,
  which in M1d a program loaded only by naming it.

### How an option's H and R were taken

The ladder was not changed. Under (b) or (c) a program an option moves has, for H, the blind enumerator's
list at its own root, and R is the commitments over that list whose node is alive and discharged. That was
computed three ways, which agree wherever more than one reaches:

1. **In process**, by `rung_census`: `hypgen::generate_blind` at root's fixpoint for the list; over it, the
   static check and *forbid* as a load would read them (`monotone::classify`, `forbid::check`, over root's
   plans); then every node of the cube by the calls `region`'s node function makes, within the oracle's limit
   of 16. With `--check` the same cube is run on every program whose H the blind enumerator builds today and
   held, node for node, to `region::compute`'s two bits and its count of debts: **437 programs, all agree**;
   7 more are refused by the engine and have no region, and 26 are past the limit.
2. **By `ein solve` and `ein oracle` on a copy** that differs from the program by one added rule:

   ```
   (rule s15-off () :match (and (s15-on) (absent (s15-paid))) :assert (open) :why "never owed: a bare (open) takes the obligations rung to the blind enumerator")
   ```

   It is an obligation rule without parameters, so its plan is compiled at every root; `(s15-on)` holds in no
   state, so no state owes it; and its `(open)` names no relation, so `oblgen::generate` declines and the
   ladder falls through to the blind enumerator, as it does on
   [`tests/stdlib/bijection/06_blind_enumeration.ein`](../../../tests/stdlib/bijection/06_blind_enumeration.ein),
   which earns the blind enumerator back the same way. The rule's two heads are written in no fact and in no
   argument position, so they are no object and no relation with a signature, and add no candidate: on every
   copy the read-out's `universe` count is the census's `blind:n` (4, 11, 22, 82, 342). This form reaches
   past the oracle's limit, by describe at the default `--region-budget`.
3. **By deleting the import**, where the program does not use it (witness 3 is witness 2 so; the fourth
   witness has a twin with `symmetric` written out).

### T1f.10.15.1: the witnesses

Ten programs under `build/s1f.10.15/w/`. `o_satisfied.ein`, witness 1:

```
;; S1f.10.15, T1, witness 1 (E-51's o_satisfied_blind.ein): an obligation
;; stated and discharged at root. `at` is total and owed, and both colours
;; have a house; `near` is the program's own open relation.
(import std.algebra :symbols (total-owed))
(relation at Color House :why "{?1} is at {?2}")
(relation near House House :why "{?1} is near {?2}")
(is-a Red Color) (is-a Blue Color) (is-a H1 House) (is-a H2 House)
(total-owed at is-a)
(at Red H1) (at Blue H2)
(solve)
```

`o_unactivated.ein`, witness 2:

```
;; S1f.10.15, T1, witness 2 (E-51's o_unactivated_blind.ein): witness 1 with
;; its activator deleted and the import kept. The program states no
;; obligation; std.algebra's two obligation rules are loaded with the module.
(import std.algebra :symbols (total-owed))
(relation at Color House :why "{?1} is at {?2}")
(relation near House House :why "{?1} is near {?2}")
(is-a Red Color) (is-a Blue Color) (is-a H1 House) (is-a H2 House)
(at Red H1) (at Blue H2)
(solve)
```

`no_o.ein`, witness 3:

```
;; S1f.10.15, T1, witness 3 (E-51's no_o_blind.ein): witness 2 with the
;; import deleted too. No obligation rule is loaded.
(relation at Color House :why "{?1} is at {?2}")
(relation near House House :why "{?1} is near {?2}")
(is-a Red Color) (is-a Blue Color) (is-a H1 House) (is-a H2 House)
(at Red H1) (at Blue H2)
(solve)
```

The blind enumerator's list on those three is 22 candidates, past the oracle's limit, so each has a twin over
one colour and two houses, 11 candidates. `o_satisfied_small.ein`:

```
;; S1f.10.15, T1: witness 1 over one colour and two houses, so that the
;; blind enumerator's list is within the oracle's limit.
(import std.algebra :symbols (total-owed))
(relation at Color House :why "{?1} is at {?2}")
(relation near House House :why "{?1} is near {?2}")
(is-a Red Color) (is-a H1 House) (is-a H2 House)
(total-owed at is-a)
(at Red H1)
(solve)
```

`o_unactivated_small.ein`:

```
;; S1f.10.15, T1: witness 2 over one colour and two houses.
(import std.algebra :symbols (total-owed))
(relation at Color House :why "{?1} is at {?2}")
(relation near House House :why "{?1} is near {?2}")
(is-a Red Color) (is-a H1 House) (is-a H2 House)
(at Red H1)
(solve)
```

`no_o_small.ein`:

```
;; S1f.10.15, T1: witness 3 over one colour and two houses.
(relation at Color House :why "{?1} is at {?2}")
(relation near House House :why "{?1} is near {?2}")
(is-a Red Color) (is-a H1 House) (is-a H2 House)
(at Red H1)
(solve)
```

`connex_lt.ein`, the re-plan's witness. Its program was not kept, so this is rebuilt from the four things
[§ The finding](#the-finding) says of it, and its read-out has the two rows quoted there:

```
;; S1f.10.15, T1, the re-plan's witness, rebuilt from its description:
;; std.algebra imported for `connex`, `lt` declared over three nodes, two
;; `lt` facts stated, and no obligation named anywhere. A and C are not
;; compared, so root is dead.
(import std.algebra :symbols (connex))
(relation lt Node Node :why "{?1} is less than {?2}")
(instance A Node) (instance B Node) (instance C Node)
(connex lt instance Node)
(lt A B) (lt B C)
(query :goal (lt ?x ?y))
```

`connex_lt_alive.ein`:

```
;; S1f.10.15, T1: the re-plan's witness with the third pair stated, so root
;; is alive.
(import std.algebra :symbols (connex))
(relation lt Node Node :why "{?1} is less than {?2}")
(instance A Node) (instance B Node) (instance C Node)
(connex lt instance Node)
(lt A B) (lt B C) (lt A C)
(query :goal (lt ?x ?y))
```

`symmetric_own.ein`, the fourth witness:

```
;; S1f.10.15, T1, the fourth witness: std.algebra imported for `symmetric`,
;; beside a declared relation of the program's own that nothing closes.
(import std.algebra :symbols (symmetric))
(relation near Thing Thing :why "{?1} is near {?2}")
(relation likes Thing Thing :why "{?1} likes {?2}")
(thing A) (thing B)
(symmetric near)
(solve)
```

`symmetric_own_no_import.ein`:

```
;; S1f.10.15, T1: the fourth witness with the import deleted and the one
;; rule it used, std.algebra's `symmetric`, written in its place. No
;; obligation rule is loaded.
(relation near Thing Thing :why "{?1} is near {?2}")
(relation likes Thing Thing :why "{?1} likes {?2}")
(thing A) (thing B)
(rule symmetric (?rel)
  :match  (?rel ?a ?b)
  :assert (?rel ?b ?a)
  :why    "{?rel} is symmetric: {?a} ↔ {?b}.")
(symmetric near)
(solve)
```

**All three witnesses of 2026-11-14 reproduce**, the third to the state: *110 057 … a lower bound*. What each
answers, today and under each option (`ein solve`, and `ein oracle` where H is within its limit; (a) is
today):

| program | class at root | (a) today | (b) | (c) |
|---|---|---|---|---|
| witness 1, `o_satisfied` | discharged | H 0, R {∅} | as today | H 22: R the whole cube, 4 194 304 commitments, at least 110 057 states at the budget |
| its twin, `o_satisfied_small` | discharged | H 0, R {∅} | as today | H 11: R 2 048 commitments, 2 048 states |
| witness 2, `o_unactivated` | unactivated | H 0, R {∅} | H 22: the whole cube, at least 110 057 states at the budget; **witness 3's read-out, row for row** | as (b) |
| its twin, `o_unactivated_small` | unactivated | H 0, R {∅} | H 11: R 2 048 commitments, 2 048 states, `no_o_small`'s | as (b) |
| witness 3, `no_o` | none | H 22: the whole cube, at least 110 057 states at the budget | as today | as today |
| the re-plan's, `connex_lt` | unactivated | H 0, root dead, R empty; `outside lt — no candidate: no obligation owes it` | **refused at load** by *forbid*, exit 1: the blind enumerator proposes `lt`, 4 candidates, and `connex` rests on an `absent` over it | as (b) |
| `connex_lt_alive` | unactivated | H 0, R {∅}; the same `outside` row | **refused at load**, 3 candidates | as (b) |
| the fourth, `symmetric_own` | unactivated | H 0, R {∅}; `outside near — no candidate: no obligation owes it` | H 4, `(likes A B) (likes B A) (near A B) (near B A)`: **R 16 commitments, 8 states**, its twin's without the import | as (b) |
| `symmetric_own_no_import` | none | H 4, R 16 commitments, 8 states | as today | as today |

- **The fourth witness is the finding in four candidates.** The program names no obligation and closes
  nothing, and of `likes`, its own relation, R holds nothing. With the import deleted and the one rule it
  used written out, R is 8 states. The `outside` row names `near` and not `likes`: it names a relation a rule
  concludes or a query asks about, and nothing does either of `likes`. So the read-out says nothing at all
  about the relation the program left open.
- **Under (b) the two `connex` programs stop loading.** Today the import is what makes them legal: nothing
  proposes `lt`, so `connex`'s `absent` is over a relation no commitment extends. The refusal names its
  remedy, `(solve :no-hypothesis (lt))`
  ([H15-2](findings.md#h15-2--under-b-a-program-that-loads-today-is-refused-forbids-premise-depends-on-the-rung)).
- **(b) and (c) differ on witness 1 alone**, and there (c) gives 22 untyped candidates, `(at Blue Red)`
  among them, to a program whose one obligation is paid. That is the case `oblgen.rs`'s header rules out.
- **What the blind enumerator proposes is untyped** on every row: `(at H1 H2)` and `(near Red H2)` are
  candidates. That is its own, ruled, behaviour, and not this stage's.

### T1f.10.15.2: the census

`ein.rs/crates/ein-infer/examples/rung_census.rs`; its module docs define each column.

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example rung_census -- --jobs 8 --check > build/s1f.10.15/census/rows.tsv
```

2026-11-15, the oracle's default limit of 16. Of 867 corpus files 644 load (35 `load-negative`, 30
`parse-negative` and 158 `regression` entries do not), and **127 load an obligation rule**. By what is true
at root's fixpoint and by the rung the ladder takes today:

| class | programs | on the second rung | under an hrule | `:no-search` | declined, the blind enumerator | no obligation rule: blind / hrule |
|---|---:|---:|---:|---:|---:|---:|
| none: no obligation rule loaded | 517 | | | | | 461 / 56 |
| **unactivated**: a rule loaded, no plan at root | **45** | **41** | 4 | | | |
| **discharged**: a plan at root, root owes nothing | **16** | **1** | 7 | 7 | 1 | |
| owing: root owes | 66 | 23 (3 of them stuck) | 21 | 14 | 8 | |

The three counts the task asks for:

- **The corpus programs that load an obligation rule no fact of root activates: 45.** 41 are on the second
  rung, with H empty; 4 declare an hrule (`examples/zebra2-hints.ein`, `examples/ein-bugs/zebra2-hints.ein`,
  `examples/lattice/09_activator_hole.ein`, `examples/lattice/12_activator_chain_hole.ein`), which builds
  their H under every option.
- **Of those, the ones with a relation the blind enumerator would propose about: 5**, the 4 under an hrule
  (16 565 candidates between them, which no option makes H) and **1 of the 41**,
  `examples/features/09_adjacent_via_same_house.ein`, 82 candidates of `colour-loc` and `pet-loc`. On the
  other 40 the blind enumerator's list is empty: the five stdlib modules read as programs state no fact, and
  the 35 others name every relation it would fill in a `:no-hypothesis` list, two of them closing some as
  well (`census/readouts.txt` has the generator's counters on each copy; on the three whose root is dead the
  summary carries none, and their lists name each of their three relations).
- **The programs whose obligation is activated and whose root owes nothing: 16, and 1 of them is on the
  second rung**, `stdlib/bool/inputs.ein`, the module read as a program with no input wire. That one program
  is the whole difference between (b) and (c) on the corpus. Seven of the other fifteen say
  `(solve :no-search)`, among them the stdlib's own tests of a satisfied debt
  (`tests/stdlib/algebra/24_total_owed_satisfied.ein`, `26_surjective_owed_satisfied.ein`,
  `tests/stdlib/slots/{01,08,10,12}`), seven declare an hrule, and one declines on purpose
  (`tests/stdlib/bijection/06_blind_enumeration.ein`).

**R under each option, for every program an option moves.** (a) is today's binary. Under (b) the 41 are on
the third rung, under (c) the 41 and `stdlib/bool/inputs.ein`. This is the list task 4 needs, made before any
change:

| programs | (a) today | (b) | (c) |
|---|---|---|---|
| **40**: `examples/ein-bugs/wrong-answers/converse-lemma-subtype.ein`, `examples/features/10_expect.ein`, `stdlib/{algebra,bijection,closure,slots,typing}.ein`, `tests/region/06_false.ein`, `tests/stdlib/algebra/{01…22,27}` (23 files), `tests/stdlib/bijection/{04,05}`, `tests/stdlib/closure/{01,02}`, `tests/stdlib/slots/{05,06}`, `tests/stdlib/typing/{01,02,03}` | H 0 | **H 0, R the same; the read-out the same, row for row** (each copy against the original, `ein solve`) | as (b) |
| `examples/features/09_adjacent_via_same_house.ein` | H 0, root dead, R empty; `universe  no candidate …` and `outside  pet-loc — no candidate: no obligation owes it …` | **H 82, 82 dead alone, R empty**: root is dead. Two rows of the read-out change, `universe  82 candidates, 82 dead alone`, and the `outside` row goes | as (b) |
| `stdlib/bool/inputs.ein` | H 0, R {∅} | as today | **H 342, R at least 55 747 states**, describe not exhausted at the default budget (it stops before the states of two candidates), N at least 2 907 |

- **Under (b) no corpus program's R moves.** One program's H moves, and R is empty over both.
- **Under (c) one corpus program's R moves**, from one state to an unexhausted region over 342 candidates of
  `in`, on a module that states `(total-owed in is-a)` and has no input to owe for.
- **Not reached by the cube**: `features/09` (82 candidates) and `stdlib/bool/inputs.ein` (342), both read
  instead by `ein solve` on the copy, the first to exhaustion, the second to the default budget. No other
  program an option moves has a candidate.
- **Refused at load under an option**: none in the corpus. The two corpus programs that activate `connex`,
  `tests/stdlib/algebra/08_checks_satisfied.ein` and `12_connex_violated.ein`, name its relation in a
  `:no-hypothesis` list.

**The corpus has no program on which the finding moves an answer**, and the reason is in the files: the 35
lists say what must not be guessed, which only the third rung would guess.
`features/09`'s header says so of itself: *"Argument typing, so the blind hypothesis generator stays inside
the declared signatures"*, and of its whitelist, *"if the guard comes back, root is consistent and the engine
goes looking … Four candidates is enough to report a verdict and stop."* So the program was put through
that: `w/features09_guard_back.ein` is the tracked file with `(neq ?h_other ?h1)` put back before the
`absent` of each of its two pruners (`run.sh`'s `sed`), the regression the fixture exists to fail on.

| `features/09` with the guard back | H | R |
|---|---|---|
| (a) today | 0; `outside pet-loc — no candidate: no obligation owes it` | {∅}, one state; the query answers `?h = House-2` |
| (b), and (c) | 82, 78 dead alone: the four the header counts are alive | 9 commitments, 9 states, 4 maximal, N 80 |

Today the fixture would still fail on the regression, root alive where it was dead, and it would not show
what its header describes
([H15-3](findings.md#h15-3--the-corpus-was-written-for-the-third-rung-one-header-and-35-lists-say-so)).

**The candidates `object_census` would mark**, on the programs an option gives a new H
(`census/object.tsv`, and `rung_census`'s own columns, which read the same marks and add `IN` and `IR`):

| program | moved by | new candidates | marked | which |
|---|---|---:|---:|---|
| `examples/features/09_adjacent_via_same_house.ein` | (b), (c) | 82 | 44, `R1` | name `adjacent-via-bwd` or `adjacent-via-fwd`, the heads of facts only a rule derives: E-7 |
| `stdlib/bool/inputs.ein` | (c) | 342 | 340: 36 `K1`, 336 `IN` | 36 name `__closed__`: E-1. 336 name one of the sixteen functions, `f0000` … `f1111`, which `std.bool.gates` writes and this file does not: E-54, on a module read as a program |

Those are 2 of the 12 programs of
[H9-3](findings.md#h9-3--e-1-and-e-7-are-absent-from-the-corpuss-answers-by-the-rung-twelve-programs-take);
the other ten keep their rung under every option, seven owing on the second and three under an hrule. So a
ruling of (b) makes Q-M1f.306 bear on one more corpus program, on which every candidate is dead with root,
and a ruling of (c) on two
([H15-1](findings.md#h15-1--under-b-q-m1f306-bears-on-one-more-corpus-program-and-under-c-on-two)).
Neither bears on [H6-2](findings.md#h6-2--under-b-q-m1f306-bears-on-two-more-programs-and-marks-nothing-on-them)'s
pair: `oblig-derived-activator.ein` and its control owe at root and keep their rung here.

### T1f.10.15.3: the question, and the stop

[Q-M1f.308](../open_questions.md#q-m1f308--the-rung-a-loaded-obligation-rule-chooses): the three options
with what each does on the witnesses and on each corpus program it moves, and the recommendation as
[§ Tasks](#tasks) states it, (b), **which restates a ruling of M1d's, is not a default and is not acted on**.
The census does not speak against (b): it moves no corpus answer, and the one H it changes is all dead. What
it puts beside the recommendation is in the question: the corpus cannot show what (b) changes, since no
tracked program's answer moves; (b) refuses a program that loads today; and the blind enumerator a program
comes back to is untyped and, on one corpus program, proposes 44 candidates that name a relation.

### `run.sh`, `skips.py` and `line.py`

`EIN_REPO=/path/to/checkout sh build/s1f.10.15/run.sh > build/s1f.10.15/run.txt`, with the ten programs of T1
in place under `w/`. `line.py` is [S1f.10.6's](s1f.10.6_obligations_under_hypothesis.md#runsh-and-linepy),
byte for byte.

`run.sh`:

```sh
#!/bin/sh
# S1f.10.15, tasks 1 and 2: the witnesses under `ein solve` and `ein oracle
# --dump`, today and as options (b) and (c) would have them; the census over
# the corpus; and, for every corpus program an option moves to the blind
# enumerator, its read-out today beside its read-out under the option. It
# writes beside the programs; $EIN_REPO is the checkout (default: two
# directories up), $EIN_BIN the binary and $EIN_RUNG, $EIN_OBJECT and
# $EIN_MECH the census examples (defaults: the checkout's release builds,
# `cargo build --release -p ein-cli` and `cargo build --release -p ein-infer
# --example rung_census --example object_census --example mechanism_census`).
# $EIN_JOBS is the census's --jobs (default 8).
cd "$(dirname "$0")" || exit 1
REPO=${EIN_REPO:-$(cd ../.. && pwd)}
EIN=${EIN_BIN:-$REPO/ein.rs/target/release/ein}
RUNG=${EIN_RUNG:-$REPO/ein.rs/target/release/examples/rung_census}
OBJECT=${EIN_OBJECT:-$REPO/ein.rs/target/release/examples/object_census}
MECH=${EIN_MECH:-$REPO/ein.rs/target/release/examples/mechanism_census}
JOBS=${EIN_JOBS:-8}
# The copies are outside the checkout: the stdlib is named.
export EIN_STDLIB=$REPO/stdlib
$EIN --version | sed "s|$REPO|…|" > version.txt
mkdir -p b c census
one() {  # FILE
    b=${1%.ein}
    $EIN solve "$1" > "$b.solve.txt" 2>&1; echo "exit $?" >> "$b.solve.txt"
    rm -rf "$b.oracle"
    $EIN oracle "$1" --dump "$b.oracle" > "$b.oracle.txt" 2>&1; x=$?; echo "exit $x" >> "$b.oracle.txt"
    printf '%-30s ' "$1"
    if [ -f "$b.oracle/region.json" ]; then python3 line.py "$b.oracle"
    else printf 'no dump, exit %s: %s\n' $x "$(head -1 "$b.oracle.txt" | cut -c1-150)"; fi
}
# `declined SRC OUT`: SRC with one obligation rule added that no state owes
# and that names no relation. Its plan is compiled at root, a rule without
# parameters having no activator to wait for, and a bare `(open)` makes the
# obligations rung decline, so the ladder falls through to the blind
# enumerator: the copy's H is the blind enumerator's list at SRC's root. The
# rule's two heads are written in no fact and no argument position, so they
# are no object and no relation of the registry, and they add no candidate.
declined() {
    {
        echo ";; $(basename "$1") taken off the obligations rung (S1f.10.15's run.sh)."
        cat "$1"
        echo '(rule s15-off () :match (and (s15-on) (absent (s15-paid))) :assert (open) :why "never owed: a bare (open) takes the obligations rung to the blind enumerator")'
    } > "$2"
}

echo '== T1: the witnesses today'
for f in w/o_satisfied.ein w/o_unactivated.ein w/no_o.ein \
         w/o_satisfied_small.ein w/o_unactivated_small.ein w/no_o_small.ein \
         w/connex_lt.ein w/connex_lt_alive.ein \
         w/symmetric_own.ein w/symmetric_own_no_import.ein; do
    one "$f"
done
echo '== T1: the witnesses as (b) would have them, (c) the same on each'
for f in o_unactivated o_unactivated_small connex_lt connex_lt_alive symmetric_own; do
    declined w/$f.ein b/$f.ein; one b/$f.ein
done
echo '== T1: witness 1 as (c) alone would have it'
for f in o_satisfied o_satisfied_small; do
    declined w/$f.ein c/$f.ein; one c/$f.ein
done

echo '== T2: the corpus program (b) gives a new H, with the regression its header names put back'
# examples/features/09_adjacent_via_same_house.ein with `(neq ?h_other ?h1)`
# before the `absent` of each of its two pruners.
{
    echo ';; S1f.10.15, T2: examples/features/09_adjacent_via_same_house.ein with the'
    echo ';; (neq ?h_other ?h1) premise put back in both pruners, the regression its'
    echo ';; header says the fixture exists to fail on (S1f.10.15'"'"'s run.sh).'
    sed -e 's/^\( *\)(absent (?S ?h_other ?h1)))$/\1(neq ?h_other ?h1)\n\1(absent (?S ?h_other ?h1)))/' \
        -e 's/^\( *\)(absent (?S ?h1 ?h_other)))$/\1(neq ?h_other ?h1)\n\1(absent (?S ?h1 ?h_other)))/' \
        "$REPO/examples/features/09_adjacent_via_same_house.ein"
} > w/features09_guard_back.ein
echo "$(grep -c '(neq ?h_other ?h1)$' w/features09_guard_back.ein) premises put back"
one w/features09_guard_back.ein
declined w/features09_guard_back.ein b/features09_guard_back.ein; one b/features09_guard_back.ein
grep -E '^  (states|commitments|universe|    outside)' w/features09_guard_back.solve.txt b/features09_guard_back.solve.txt

echo '== T2: the census over the witnesses'
$RUNG --check w/*.ein > census/witnesses.tsv
cut -f1,4,5,12-17,33-44 census/witnesses.tsv | grep -v '^#' | sed "s|$PWD/||"
echo '== T2: the census over the corpus'
$MECH --json > census/mechanisms.json
$RUNG --jobs "$JOBS" --check > census/rows.tsv
grep '^#' census/rows.tsv | grep -v 'H the same:'
echo "and $(grep -c '^# (b) H the same:' census/rows.tsv) lines '(b) H the same', $(grep -c '^# (c) H the same:' census/rows.tsv) lines '(c) H the same'"

echo '== T2: every corpus program an option moves, today and under the option'
: > census/readouts.txt
grep -v '^#' census/rows.tsv | awk -F'\t' '$15 == "b,c" || $15 == "c" { print $1, $15, $16 }' |
while read -r f option n; do
    case $option in b,c) d=b ;; *) d=c ;; esac
    name=$(echo "${f%.ein}" | tr '/' '.')
    mkdir -p $d/corpus
    declined "$REPO/$f" "$d/corpus/$name.ein"
    ( cd "$REPO" && $EIN solve "$f" ) > "$d/corpus/$name.today.txt" 2>&1; echo "exit $?" >> "$d/corpus/$name.today.txt"
    $EIN solve "$d/corpus/$name.ein" --json-summary "$d/corpus/$name.json" > "$d/corpus/$name.solve.txt" 2>&1; echo "exit $?" >> "$d/corpus/$name.solve.txt"
    # The two read-outs less the line that echoes the path.
    tail -n +2 "$d/corpus/$name.today.txt" > census/.today
    tail -n +2 "$d/corpus/$name.solve.txt" > census/.option
    if cmp -s census/.today census/.option; then same=same; else same=differs; fi
    rm -f census/.today census/.option
    printf '%s\t(%s)\tblind:n %s\tread-out %s\t%s\t%s\n' "$f" "$option" "$n" "$same" \
        "$(grep -m1 -E '^  universe' "$d/corpus/$name.solve.txt" | sed 's/^  universe *//')" \
        "$(python3 skips.py "$d/corpus/$name.json")" >> census/readouts.txt
done
cut -f2-4 census/readouts.txt | sort | uniq -c
echo 'why the blind enumerator proposes nothing, on the programs whose H is the same:'
grep 'blind:n 0' census/readouts.txt | cut -f6 | sort | uniq -c
grep 'differs' census/readouts.txt
echo '== T2: the candidates object_census marks, on the programs whose H an option changes'
grep 'differs' census/readouts.txt | cut -f1 | sed "s|^|$REPO/|" | xargs $OBJECT > census/object.tsv
cut -f1,4,21-29 census/object.tsv | grep -v '^#'
```

`skips.py`:

```python
#!/usr/bin/env python3
"""What the generator counted on one `ein solve --json-summary FILE`: the
raw candidates, those emitted, and, by relation-level skip, how many
(object, relation, slot) fills were skipped.

    python3 skips.py FILE
"""
import json, sys

h = json.load(open(sys.argv[1]))["root"]["hypgen"]
skips = " ".join("%s:%d" % kv for kv in sorted(h.get("pre_candidate", {}).items()))
print("raw %d emitted %d skipped %s" % (h["raw"], h["emitted"], skips or "none"))
```

### What contradicts the plan

1. **The switch is written twice.** Task 4 names `hypgen::generate_with`. `hypgen::eligible_relations`
   repeats `!program.obligations.is_empty()` to say which relations the active rung proposes, and
   `shape::naf_map` reads it, a shape the corpus digests. A ruling of (b) or (c) changes both or the two disagree.
   `forbid::Proposers::of` and the L5 table (`oblgen::alternatives`, `explore.rs`) read the same test and
   are indifferent to it where no plan is compiled; `scope::outside` reads the rung the pass took and
   follows.
2. **The ladder's sentence is in more pages than task 4 lists.** Beside `06_reserved_names.md` and the two
   module headers it is in [`solution-semantics.md`](../../../docs/ein/reasoning/kb-evolution/solution-semantics.md#14-candidate-and-universe)
   § 1.4 and in `AGENTS.md`, and there, as in `hypgen.rs`'s header, it reads *else the facts that would
   discharge what root owes, else the blind enumerator*, which is option (c)'s wording and not the engine's
   ([H15-4](findings.md#h15-4--the-ladder-is-stated-in-five-places-and-three-of-them-read-as-option-c)).
3. ***(b) … a program that imports a module for something else is on the third rung again*, and it may then
   not load.** The re-plan's own witness is refused under (b) (H15-2). Task 4's acceptance, *every moved R on
   the list*, has no line for a program that goes from an answer to a refusal; the corpus has none.
4. **Under (b), task 4 files no register row and flips none**: no corpus program's R moves, so *each a
   register row filed `open` before the fix* is an empty list, and the change would land with no tracked
   program whose answer shows it. The fourth witness and `features09_guard_back.ein` are the two candidates
   for one.
5. **(a)'s read-out row would print on one corpus program.** *A `universe` row that names the rule which
   took the program off the blind enumerator* bites where the blind enumerator would have proposed, which in
   the corpus is `features/09` alone; printed wherever an unactivated rule chose the rung, it is 41 rows.
6. ***M1d's scope rule* was the read-out's**, and R has replaced it (the fourth bullet of
   [§ What the engine does](#what-the-engine-does-which-is-what-the-three-options-differ-on)). The ruling
   (b) restates is the ladder's middle row as `oblgen.rs`'s header words it; its table in
   [`hypotheses_from_obligations.md`](../../../docs/history/m1d_satisfiability/hypotheses_from_obligations.md)
   has no row for a rule loaded and nothing owed.
7. **The re-plan's witness could not be rebuilt to the letter**: its program is nowhere, and a program of its
   description with `connex` activated has a dead root.

### Not done

- Task 4, by instruction: the ladder is unchanged, nothing is restated, no row is filed, and E-51 is open in
  the calculator's `findings.md`.
- No option was built. (b) and (c) are read by the three forms of
  [§ How an option's H and R were taken](#how-an-options-h-and-r-were-taken), none of which is the engine
  under the option. **What the narration would be was not taken**: under (b) the 41 programs' `rung` event,
  their `root.hypgen` counters and with them their shape digests move, on 40 of them with H, R and the
  read-out unchanged. The copies narrate `declined`, which is not what (b) would.
- (a)'s read-out row was not prototyped.
- The 26 programs on the blind rung past the oracle's limit were not held to `region::compute` by `--check`.
- `import_invariance` was not run against any option; activation is read off facts at root, which no import
  form changes, and that is an argument and not a run.
- No probe was made a tracked witness. The phase README's status line and stage table are not edited.
- `./run_tests.sh` was not run, by instruction. Run: `cargo fmt --all --check`, `cargo clippy --workspace
  --all-targets -- -D warnings` (both as `run_tests.sh` runs them), `python3 utils/check_hashmap_iteration.py`
  and `python3 utils/doc_audit.py --links --check`.

## Task 4's inventory (2026-11-16), taken before the ladder changed

**What moves under (b), every corpus program, listed before the switch is committed.** Two release binaries, on AC:
`build/s15t4/base/ein`, `cargo build --release -p ein-cli` at `a79eea49` (`master`), and `build/s15t4/b/ein`, the
same tree with the switch applied (the second rung where an obligation is activated at root, read by the function
`mechanisms.rs` reads; the diff is kept as `build/s15t4/fix_v1.diff`), built and then taken off the tree again. Both
were run over every `corpus/corpus.toml` entry, 874, from the repo root, by `inventory.py` below: `ein solve FILE -H
--json-summary s.json --events e.jsonl` (stdout, stderr, exit, the summary, the event stream hashed), and `ein test
FILE` on the 358 entries that declare a `test` run, compared again with the run's seconds masked. The shape golden
was re-banked on the same change, `EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test --profile gate -p ein-render --test
id_order_invariance`, every line, and diffed against `master`'s; `python3 utils/read_out_examples.py --check --bin
build/s15t4/b/ein` was run against the page.

**41 programs change rung, 1 changes H, none changes R**, which is what
[Q-M1f.308](../open_questions.md#q-m1f308--the-rung-a-loaded-obligation-rule-chooses) said of (b):

| programs | rung | H | R | read-out (`ein solve` stdout) | summary | narration |
|---|---|---|---|---|---|---|
| `examples/features/09_adjacent_via_same_house.ein` | second (`obligations`, nothing owed) → blind | 0 → **82**, all dead alone | empty, root dead, both | **three rows**: `universe` *no candidate …* → *82 candidates, 82 dead alone*; `outside pet-loc — no candidate: no obligation owes it …` goes; `relation names adjacent-via-bwd, adjacent-via-fwd — relations only a rule concludes, and 44 candidates of colour-loc, pet-loc name one as an object …` comes | `region` (the universe), `root.hypgen` | the `rung` event goes; the 82 `hyp` lines and the nodes they open come |
| 32: `examples/ein-bugs/wrong-answers/converse-lemma-subtype.ein`, `examples/features/10_expect.ein`, `tests/region/06_false.ein`, `tests/stdlib/algebra/01_copiers.ein`, `tests/stdlib/algebra/02_compose.ein`, `tests/stdlib/algebra/03_boolean.ein`, `tests/stdlib/algebra/04_extensive.ein`, `tests/stdlib/algebra/05_tag_lemmas.ein`, `tests/stdlib/algebra/06_equational.ein`, `tests/stdlib/algebra/07_schroder.ein`, `tests/stdlib/algebra/08_checks_satisfied.ein`, `tests/stdlib/algebra/09_irreflexive_violated.ein`, `tests/stdlib/algebra/10_antisymmetric_violated.ein`, `tests/stdlib/algebra/11_asymmetric_violated.ein`, `tests/stdlib/algebra/12_connex_violated.ein`, `tests/stdlib/algebra/13_empty_violated.ein`, `tests/stdlib/algebra/14_functional_violated.ein`, `tests/stdlib/algebra/15_injective_violated.ein`, `tests/stdlib/algebra/18_totality_open_world.ein`, `tests/stdlib/algebra/19_total_violated.ein`, `tests/stdlib/algebra/20_surjective_violated.ein`, `tests/stdlib/algebra/21_transitive.ein`, `tests/stdlib/algebra/22_symmetric_negative.ein`, `tests/stdlib/algebra/27_converse_welltyped_either_way.ein`, `tests/stdlib/bijection/04_typecheck_arg0_violated.ein`, `tests/stdlib/bijection/05_typecheck_arg1_violated.ein`, `tests/stdlib/closure/01_infer_closure.ein`, `tests/stdlib/closure/02_closed_and_satisfied.ein`, `tests/stdlib/slots/05_no_room.ein`, `tests/stdlib/slots/06_no_fill.ein`, `tests/stdlib/typing/01_reflexive.ein`, `tests/stdlib/typing/03_type_hierarchy_welltyped.ein` | second → blind | 0, both | the same | the same, byte for byte | `root.hypgen.pre_candidate` alone: the blind enumerator runs and counts the relations its lists take out (`no_hypothesis_relation`) | the `rung` event goes, and the event numbers after it shift by one; at `--events-level verbose`, which the golden's `hyp` ops read, the `hypskip` lines come |
| 8: `stdlib/{algebra,bijection,closure,slots,typing}.ein`, `tests/stdlib/algebra/16_converse_illtyped_dom.ein`, `17_converse_illtyped_ran.ein`, `tests/stdlib/typing/02_type_hierarchy_converse.ein` | second → blind | 0, both | the same | the same | the same | the `rung` event goes |

Nothing else moved: no other program's stdout, stderr, exit code, summary or event stream; no `ein test` read-out of
the 358 (no `:expect` moves, and no exit code: `corpus_exits.txt` does not move); and
[`read-out-examples.md`](../../../docs/ein/user/reference/read-out-examples.md) is the binary's page as it is
(`--check` exits 0; `tests/stdlib/algebra/09_irreflexive_violated.ein`, its X7 and X18, is among the 40 whose
read-out does not move).

**The goldens that move**, and nothing else:

- **`ein-render/tests/golden/corpus_shapes.md5`: 94 lines.** `hyp` and `hyp+closed` on each of the 41 programs, 82
  lines, the generator's narration (the `rung` event and its counters); and 12 more on `features/09`:
  `read-out[describe]`, `read-out[maximal]`, `read-out[minimal]`, `read-out[single]`, `read-out[n2]`,
  `region-trace`, `cores[describe]`, `cores[maximal]`, `cores[minimal]`, `cores[single]`, `cores[n2]` and
  `oracle`. Not `naf`, which reads `hypgen::eligible_relations`, on any program; not `states` or `region-lattice`
  on `features/09`, whose R is empty under both. Re-banked with the change: `build/s15t4/corpus_shapes_b.md5`.
- `corpus_exits.txt`, `read_out_examples.md`, every `:expect`, the register's existing rows: **none**.
- The witnesses task 4 files (below) are new programs: their lines are added by the rows commit and move in the
  fixing commit, and are not on this list.

`inventory.py`, as run (`python3 build/s15t4/inventory.py build/s15t4/base/ein build/s15t4/b/ein build/s15t4/inv
--jobs 8 > build/s15t4/inventory.tsv`; `build/` is not tracked):

```python
#!/usr/bin/env python3
"""S1f.10.15 T4's inventory: every corpus program under two binaries.

    python3 build/s15t4/inventory.py BASE_BIN NEW_BIN OUT_DIR [--jobs N]

For every `corpus/corpus.toml` entry, run from the repo root:

    ein solve FILE -H --json-summary s.json --events e.jsonl     (stdout, stderr, exit)
    ein test FILE                                                 (where the entry declares `test`)

under both binaries, and print one tab-separated line per program on which
anything differs: which of stdout / exit / stderr / summary / events / test
moved, and the summary's H, R and rung on each side. The events stream's
`run` line is compared less its `argv` and `file`, which name the scratch
paths. Nothing else is normalised.
"""
import hashlib, json, os, subprocess, sys, tomllib
from concurrent.futures import ThreadPoolExecutor

base, new, out = sys.argv[1], sys.argv[2], sys.argv[3]
jobs = int(sys.argv[sys.argv.index("--jobs") + 1]) if "--jobs" in sys.argv else 12
repo = os.getcwd()
entries = tomllib.load(open("corpus/corpus.toml", "rb"))["entry"]


def run(binary, tag, e):
    name = e["path"].replace("/", ".")
    d = os.path.join(out, tag, name)
    os.makedirs(d, exist_ok=True)
    s, ev = os.path.join(d, "s.json"), os.path.join(d, "e.jsonl")
    for f in (s, ev):
        if os.path.exists(f):
            os.remove(f)
    p = subprocess.run([binary, "solve", e["path"], "-H", "--json-summary", s, "--events", ev],
                       capture_output=True, text=True, timeout=900)
    summary = json.load(open(s)) if os.path.exists(s) else None
    # The stream is hashed line by line and deleted: on the zebra family it
    # runs to gigabytes. The `rung` lines are kept, which `brief` reads.
    h, rungs = hashlib.sha256(), []
    if os.path.exists(ev):
        with open(ev, "rb") as fh:
            for line in fh:
                if line.startswith(b'{"e": "run"'):
                    o = json.loads(line)
                    o.pop("argv", None)
                    o.pop("file", None)
                    line = json.dumps(o).encode()
                elif line.startswith(b'{"e": "rung"'):
                    rungs.append(json.loads(line))
                h.update(line)
        os.remove(ev)
    events = (h.hexdigest(), rungs)
    test = None
    if "test" in e.get("runs", []):
        t = subprocess.run([binary, "test", e["path"]], capture_output=True, text=True, timeout=900)
        test = (t.returncode, t.stdout, t.stderr)
    return dict(stdout=p.stdout, stderr=p.stderr, exit=p.returncode, summary=summary,
                events=events, test=test)


def brief(r):
    sm = r["summary"]
    if sm is None:
        return "no summary, exit %d: %s" % (r["exit"], (r["stderr"].splitlines() or [""])[0][:90])
    g = sm["region"]
    rung = [o.get("mode") for o in r["events"][1]]
    h = sm["root"]["hypgen"]
    return "H %s k_s %s k_c %s rung-events %s" % (h.get("emitted"), g.get("k_s"), g.get("k_c"),
                                                   ",".join(rung[:1]) + ("x%d" % len(rung) if rung else "none"))


def one(e):
    a, b = run(base, "base", e), run(new, "new", e)
    moved = [k for k in ("stdout", "exit", "stderr", "summary", "events", "test") if a[k] != b[k]]
    if not moved:
        return None
    region = []
    if a["summary"] and b["summary"]:
        for k in ("region", "verdict", "queries", "claim"):
            if a["summary"].get(k) != b["summary"].get(k):
                region.append(k)
        if a["summary"]["root"]["hypgen"] != b["summary"]["root"]["hypgen"]:
            region.append("root.hypgen")
    return "\t".join([e["path"], ",".join(moved), "summary:" + (",".join(region) or "-"),
                      "base: " + brief(a), "new: " + brief(b)])


with ThreadPoolExecutor(jobs) as pool:
    for line in pool.map(one, entries):
        if line:
            print(line, flush=True)
print("# %d entries compared" % len(entries))
```

## Record (2026-11-16): task 4

**On the user's ruling of 2026-11-16 on Q-M1f.308, *"308 (b) ok"*: the ladder's second rung is taken where an
obligation is activated at root.** Five commits on the stage's branch, in this order:

1. `c03e7e33`, **the inventory** ([§ Task 4's inventory](#task-4s-inventory-2026-11-16-taken-before-the-ladder-changed)),
   docs only, before anything moved.
2. `743c6d71`, **W97 filed `open`**: three rows, six programs under `examples/ein-bugs/wrong-answers/` with corpus
   entries, the goldens' additions (12 cells of `corpus_exits.txt`, 246 lines of `corpus_shapes.md5`), and
   `AGENTS.md`'s pointer naming this stage as the owner of an open row. No engine source.
3. `e939fec4`, **the fix**: the switch, the rows flipped with the null id, the goldens re-banked, the ladder
   restated, the tests below.
4. `82b9b4a4`, the `[case.fixed]` hash.
5. This record, with the phase's README, `open_questions.md`, the two `findings.md`.

### What was built

- **The switch is one function**, `hypgen::rung` (`hypgen::Rung`: `NoSearch`, `Hrules`, `Obligations`, `Blind`),
  read by `hypgen::generate_with`, the ladder, and by `hypgen::eligible_relations`, the advisory that
  `shape::naf_map`, `naf_deps` and `structure` read: the two places H15-4's first half named, which wrote
  `!program.obligations.is_empty()` twice. In order: `(solve :no-search)`; an hrule the program declares; **an
  obligation activated at root**; the blind enumerator. The probe, `generate_blind`, still takes the blind
  enumerator whatever the program declares.
- **The reading is `mechanisms.rs`'s, shared, no third copy**: `mechanisms::obligation_plans`, the plans root's
  activators compile (one per obligation rule and activator root holds, a rule without parameters always), is
  what `mechanisms::read` builds `Mechanisms::obligations` from, what `hypgen::rung` tests for emptiness, what
  `eligible_relations` reads the owed relations off, and what `forbid::Proposers::of` names an obligation's remedy
  from (it called `obligations::plans_for` itself, the same reading; no behaviour moved there).
- **A discharged root under an activated obligation still proposes nothing**: the second rung's code is as it
  was; only the test that sends a call to it changed. `rung-discharged.ein` holds it (H 0, R {∅}).
- **A debt only a commitment can bring, with nothing activated at root**, is the blind enumerator's by the letter
  of the ruling: the rung that would read it as a later debt is not taken, and the blind enumerator proposes the
  debt's relation with every other open one, a superset. `W26/oblig-derived-activator`'s witness activates
  `total-owed` at root and keeps its rung; no corpus program of that shape moved (the inventory).
- **The ladder restated** where it is written: `hypgen.rs`'s and `oblgen.rs`'s headers (the table's middle row
  is *an obligation activated at the state*, and the paragraph under it says why neither what is owed nor what
  is loaded chooses the rung), `06_reserved_names.md` § Hypothesis / query control (the prose, the O and B rows,
  and *read as declared … by this check and by the ladder alike*), `solution_semantics.md` § 1.4, `AGENTS.md`'s
  sentence in place; and what the stage's `grep -rn 'blind' docs/kernel ein.rs/crates/ein-infer/src | grep -i
  'obligation\|ladder\|rung'` found besides: `events.md`'s `rung` section (*only for a program that declares an
  obligation rule* is now *only where an obligation is activated*, and the count of files that emit none went),
  `oblgen::Mode::Blind`'s doc, `oblgen::RungReport`'s (its count of files went), `structure::Stated::declared`'s,
  and `Mechanisms::obligations`' and `mechanisms.rs`'s header (the ladder reads O as the check does). The other
  hits (`hypothesis_space.md`'s rung list, `implementation.md`'s module rows, `01_grammar.md`'s list keywords,
  `relation_names.rs`, `types_as_objects.rs`) say nothing a loaded rule decides and were left. `rung_census.rs`'s
  header says the ladder was (a) when it was written. No page under `docs/kernel/` and no code cites a plan path,
  stage id, finding id or date in what this stage wrote.
- **Held**: `ein-cli/tests/import_invariance.rs` gains
  `a_module_imported_for_one_rule_takes_the_rung_of_the_rule_written_out`: `rung-symmetric-import.ein`, which
  imports `std.algebra` for `symmetric`, and `rung-symmetric-written.ein`, the rule written out, take the rung
  `blind` (`ein render h-space`'s `H` row), one H (`ein show hyps`, four facts) and one R (the summary's states,
  k_s 8); and `rung-unactivated.ein` against `rung-unactivated-no-import.ein` the same, eleven facts and 2 048
  states. `ein-infer/tests/refutation_under_absent.rs`'s exact refused set gains the probe's line.

### The inventory against what moved

What the inventory listed moved, and nothing else:

- **`corpus_shapes.md5`**: 143 lines changed in the fixing commit, re-banked per commit and with
  `EIN_CORPUS_SLOW=1`. The filter, over `e939fec4`:

  ```sh
  git diff -U0 e939fec4~1 e939fec4 -- ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5 | grep '^[-+][0-9a-f]' \
    | awk '{print $3}' | sort -u > moved
  # the inventory's 94 keys (build/s15t4/shapes_b.diff, the same diff taken on fix_v1.diff):
  comm -23 moved inventory | grep -v 'wrong-answers/rung-'   # prints nothing
  comm -13 moved inventory                                    # prints nothing
  ```

  Every line outside the W97 witnesses is, digest for digest, the inventory's re-bank
  (`build/s15t4/corpus_shapes_b.md5`). The 49 others are the three witnesses whose answer moved:
  `rung-connex-import.ein` 17 (every op but `load`, `saturate`, `plan`, `match`, the `explain` pair, the `ir[…]` and
  `dot[…]` views; `naf` among them, its refusal now read), `rung-symmetric-import.ein` and `rung-unactivated.ein`
  16 each (the same, `naf` not among them). The controls and the twin did not move.
- **`corpus_exits.txt`**: one line, `rung-connex-import.ein :: solve`, 0 to 1. No existing program's.
- **Two counts pinned in tests**, both by the refused probe and no existing program: `refutation_under_absent.rs`'s
  exact refused set gains its line, and `detector.rs`'s count of flagged corpus programs goes from 48 to 49, its
  `connex` being an `absent` site over `lt` once a commitment can extend `lt` (a refused program keeps its static
  half, as the others the count holds do).
- **The register**: the three W97 rows flipped; no other row.
- `read_out_examples.md`, every `:expect`, every other exit code: none, as listed.

### The rows

| row | witness | today (filed `open`) | right, and since the fix |
|---|---|---|---|
| `W97/unactivated` | `rung-unactivated.ein`: `std.algebra` for `total-owed`, never written, beside `at` and `near`, one colour, two houses (Q-M1f.308's witness 2 over fewer names) | H 0, R {∅} | H 11, R 2 048 states, the control's (`rung-unactivated-no-import.ein`); the row's run asks the maximal line. Its twin, `rung-discharged.ein`, the obligation stated and paid: H 0, R {∅}, both ways |
| `W97/symmetric-import` | `rung-symmetric-import.ein`: `std.algebra` for `symmetric` beside an open `likes` (the fourth witness) | H 0, R {∅}, `outside near` | H 4, 16 commitments, 8 states, the control's (`rung-symmetric-written.ein`), and no `outside` row |
| `W97/connex-import` | `rung-connex-import.ein`: `std.algebra` for `connex` over `lt`, A and C not compared (the re-plan's probe) | loads: H 0, root dead, R empty, `outside lt` | **refused at load**, exit 1 |

All three within `ein oracle`'s limit; witnesses 1 and 3 are not wrong under (b), and witness 1's twin is the
first row's `[[case.twin]]`. `features09_guard_back.ein` (82 candidates) is past the limit and was not banked.

### The refusal

**A change of behaviour on a program that loads today.** `rung-connex-import.ein` loaded under the old switch: its
import's obligation rules, which nothing activates, took the second rung, nothing proposed `lt`, and *forbid*,
which reads what a commitment can still extend off H, had nothing to refuse. Under (b) the blind enumerator
proposes `(lt A C)`, `(lt B A)`, `(lt C A)` and `(lt C B)`, and every question about a commitment refuses it:

```
rule 'connex' [activator lt instance Node]: concludes (false) from (absent …) over lt, a relation a commitment can still extend — … close the relation, (solve :no-hypothesis (lt)), if no commitment is meant to extend it; …
```

`ein saturate` still saturates. Banked where the corpus keeps programs *forbid* refuses, with the refusal as
data: the register's `W97/connex-import` (`[case.now].refused`, the line in full), `refutation_under_absent.rs`'s
exact set (the line to its dash), and the exit golden. Not under `examples/broken/load/`, which holds the loader's
own refusals with an `.expected` beside each; *forbid*'s are `examples/ein-bugs/` witnesses. No corpus program
stopped loading: its two programs that activate `connex` name `lt` in a `:no-hypothesis` list.

### K-11

**Unchanged by (b).** Its probe's obligation rule is loaded **and activated**, every object's type being stated,
and root owes it nothing: the second rung, H empty, R `{∅}`, as before. Only (c) would have moved it. It is
H15-5's remaining case.

### Defaults taken

Each is the reading that moves the fewest answers, and none moved one beyond the inventory.

1. **Activated means a plan root's activators compile**, `obligations::plans_for`'s reading, which `mechanisms.rs`
   had: an obligation rule without parameters is activated at every state, so a program that writes one (the
   blind-enumeration trick of `tests/stdlib/bijection/06_blind_enumeration.ein`, the stage's `s15-off` copies)
   is on the second rung and declines it as before.
2. **The rung is read at the state the generator is called at**, which is root wherever H is built
   (`region::prepare_with`, `shape`'s `hyp` op, the tests' `generate` calls). No route calls the ladder at a node.
3. **`eligible_relations` follows the switch**: on a program whose loaded obligation rules nothing activates, it
   names the blind enumerator's relations where it named none. The `naf` digest it feeds moved on no corpus
   program (only on the refused probe, whose refusal it now reads).
4. **`forbid::Proposers::of` reads the shared function**, for no answer: it read the same plans.
5. **The probe banked is the re-plan's own, the dead-root form**; `connex_lt_alive.ein` is refused the same way
   and is not banked.
6. **Witness 1's twin is the first row's `[[case.twin]]`**, a corpus program, so that a discharged root under an
   activated obligation, which (c) would have moved, is held by the shape golden.
7. **`W97/unactivated`'s run is `solve --maximal`**, one state, its R being 2 048 states, as W96's first row did.

### Not done

- `features/09`'s header and corpus note were not restated (H15-3: true again as written; the note's *k=4* is the
  walk's word).
- S1f.10.9's object census was not re-taken whole; the read-out names `features/09`'s 44 marked candidates.
- `examples/README.md`'s catalog line for `ein-bugs/wrong-answers/` does not name the `rung-*` family (it does
  not name W96's either).
- `oblgen.rs` was edited in its module header and two doc comments above line 200 only, since S1f.10.14 edits it
  below; `oblgen::alternatives_of`'s doc (*a program that states no obligation*) reads `obligations.is_empty()`
  and is indifferent to the rung, and was left.
- Verification is below; `./run_tests.sh`, the nightly and `utils/ci_container.sh` were not run, by instruction.

### Verification

On the final tree, `82b9b4a4` with this record's pages, 2026-11-16, in the worktree, `TMPDIR` under `build/`:

- `cargo test --manifest-path ein.rs/Cargo.toml --profile gate --no-fail-fast -p ein-infer -p ein-corpus -p
  ein-cli -p ein-render`: every test binary of the four crates, 132 results, all `ok`; nothing left in `TMPDIR`.
- `EIN_CORPUS_SLOW=1`, once: `id_order_invariance` (4 passed), `import_invariance` (2 passed: every program in
  every import form, the slow entries among them, and the new test) and `explore` (7 passed).
- `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `RUSTDOCFLAGS="-D
  warnings" cargo doc --workspace --no-deps`; `python3 utils/check_hashmap_iteration.py`; `python3
  utils/stdlib_manifest.py`; `python3 utils/doc_audit.py --links --check docs plans README.md AGENTS.md
  corpus/README.md tests/README.md examples/README.md stdlib/README.md utils/README.md c/README.md`: clean.
- On the fix, before this record: `python3 utils/read_out_examples.py --check` against the release build with the
  switch, clean; `EIN_BLESS=1 … --test register_region` on the flipped rows changed none of their region
  blocks, written by hand from the fixed binary's runs.
- **Not run**, by instruction: `./run_tests.sh`, the nightly, `utils/ci_container.sh`. `corpus_cli` ran in its
  default selection only; no `cost_ms` was re-banked.

### Taken over from a first agent

A first agent started this task and stopped when its session ended, leaving an uncommitted diff to `hypgen.rs` and
`mechanisms.rs` (kept as `build/p1f10/stopped/af583060cb240b5c5.diff` in the main checkout). It was judged and
**kept**: `hypgen::Rung` and `hypgen::rung`, `generate_with` and `eligible_relations` switching on it, and
`mechanisms::obligation_plans` with `mechanisms::read` built on it. **Changed**: a public item's doc linked the
crate-private `rung` (a rustdoc error under `-D warnings`), now named in backticks; `rung`'s doc says a rule
without parameters is activated everywhere; `obligation_plans`' doc names *forbid* as a reader. Everything else
(the inventory, the rows, `forbid.rs`, the headers and pages, the tests) is this agent's. The diff was taken onto
`master`'s head, `a79eea49`, and applied only once the inventory and the rows were committed.
