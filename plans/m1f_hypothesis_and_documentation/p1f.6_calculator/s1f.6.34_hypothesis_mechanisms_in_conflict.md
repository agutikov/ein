# S1f.6.34 — A conflict between the hypothesis mechanisms is reported

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-14** ([§ Record](#record-2026-11-14)); planned 2026-11-13, on the user's ruling of that day on E-47. **An engine stage.**
**Depends on:** nothing in the phase. Independent of the stdlib layout. It owns
[E-47](findings.md) of the phase's findings.

## The ruling

[S1f.6.28](s1f.6.28_word_problems_second_pass.md)'s T4 filed E-47: *forbid*'s refusal names
`(solve :no-hypothesis (R))` as its first remedy, and on a program where an hrule proposes facts of R the remedy
does nothing, so the refusal stands word for word on the program that wrote it. The finding asked whether the
message should stop naming `:no-hypothesis` there, or `:no-hypothesis` should also filter an hrule's candidates.
The user, 2026-11-13, on that question:

> E-47: report conflict if any hypothesis mechanism constradicts itself

So neither of the two: **a program whose hypothesis declarations contradict one another is told so**, and the
contradiction E-47 met, `:no-hypothesis (R)` beside an hrule that proposes R, is one case of it, not the whole.

## The user's simplification, 2026-11-13

> actually we can make it easier: the program ussually has either positive hypothesis generation - by
> obligations or hrules or hypothesis-relations, or negative filtering of all possible hypothesis by
> no-hypothesis and __closed__; are those all 5 control mechanisms? let's make all pairs and analyze potential
> conflicts; I guess it's easy to detect most of them during load, e.g. no-hypothesis make no sense if there
> are hrules - if the conflict then it's conflict, if not - then it filters nothing; filtered obligations are
> nonsense by definition; obligated relation can't be __closed__ also, naming same relations in
> hypothesis-relations and no-hypothesis - also nonsense; combining positives e.g. hrules with obligations and
> hypothesis-relations - maybe possible but not very practical, but they don't conflict at least

## The mechanisms, read off the code (2026-11-13)

`hypgen.rs`'s ladder: **hrules** if the program declares any `(hrule …)`, and then the other two rungs never
run; otherwise **obligations** while the state owes something it may branch on (`oblgen::generate`, *declined*
falls through); otherwise the **blind enumerator**. So the five the user names, and a sixth that no declaration
writes:

| | mechanism | what it does | read by |
|---|---|---|---|
| H | `(hrule …)`, activated by `(solve :hrules …)` | generates, rung 1, alone where declared | itself |
| O | an obligation rule, `:assert (open ?R)`, activated by a fact such as `(total R isa)` | **two roles**: a state that owes is not in R, whatever generated it; and, with no hrule, rung 2 proposes facts of the owed relation | the discharge test; rung 2 |
| B | the blind enumerator | generates, rung 3, the default: no hrule, **and no obligation rule loaded**, or the obligations rung declined (corrected by T1, E-51: a loaded obligation rule that owes nothing at root leaves H empty) | — |
| HR | `(solve :hypothesis-relations …)` | a whitelist, *speculate only these* | rungs 2 and 3 (`oblgen::scoping`, `hypgen::relation_plan`); the `outside` row (E-52) |
| NH | `(solve :no-hypothesis …)` | a blacklist, *never these* | rungs 2 and 3; the `outside` row (E-52) |
| C | `(__closed__ R)`, stated or derived | excludes R from rungs 2 and 3, **and** so keeps R out of `D`, which makes an `absent` over it closed-world | rungs 2 and 3, and the read-out's `outside` row; no matcher (corrected by T1, E-52) |

HR is a filter that the user counts among the positives: it generates nothing of itself. **H and O together are
the Zebra puzzle's own form,** not an impractical one: `examples/zebra2.ein` declares `(hrule guess …)` and
imports `std.bijection`, whose `total-owed` and `surjective-owed` judge every state the hrule reaches.

## The pairs

| pair | verdict |
|---|---|
| H + O | **no conflict**: H generates, O judges (zebra2) |
| H + HR | HR is **dead**, rungs 2 and 3 never running; a **conflict** where an active hrule proposes a relation HR leaves out |
| H + NH | NH is **dead**; a **conflict** where an active hrule proposes a relation NH lists (E-47) |
| H + C | a **conflict** where an active hrule proposes a closed relation; closing another relation is C's second role and stands |
| O + HR | with no hrule, a **conflict** where an owed relation is outside HR: the owing state cannot branch, *stuck* |
| O + NH | with no hrule, a **conflict** where an owed relation is in NH: *"filtered obligations are nonsense by definition"* |
| O + C | a **conflict** where an owed relation is closed: *"obligated relation can't be __closed__ also"* |
| HR + NH | a **conflict** where one relation is in both; NH naming a relation HR already leaves out is **redundant** |
| HR + C | a **conflict** where a listed relation is closed |
| NH + C | an overlap is **redundant**, both saying the same; never a conflict |

**When each is known.** The lists, an hrule's `:assert` heads (a parameterised hrule's through the activators
`:hrules` gives it), a written `__closed__` fact and a written obligation activator are known at load. A
*derived* closure (the stdlib's closing rules, `(__closed__ fa)` and the like) and a derived activator
(`bijective-setup`'s `(total color-loc is-a)`) are known at root's fixpoint, before any search. So the check runs
once, on root, on every command that reads the program.

E-47's witness is `build/s1f.6.28/p2/kinds_listed_has_hyp_stated.ein` (gitignored; the stage re-creates it): an
hrule supposing `(has ?p apples (n z 1) ?t)`, `(solve :hrules (maybe-apples) :no-hypothesis (has))`, refused by
*forbid* with the remedy it already wrote; H + NH.

## Tasks

1. **T1f.6.34.1 — the pairs on real programs, shown before the loader moves.** For each row of § The pairs: a
   witness program, what each command answers today, and the corpus programs (`examples/`, `tests/`, `stdlib/`)
   that carry it, counted by a script that reads root's fixpoint, never by reading the directories. A row the
   corpus carries as a conflict is shown with its answer; a row the code reads otherwise than the table says is
   a finding, and the table is corrected. Recorded as the record's first section.
2. **T1f.6.34.2 — the report.** A **conflict** is refused, one line per conflict, naming both declarations and
   the relation, with the load-error exit (1; `ein test`: 2), on every command, as
   [`defined_behaviour.md` § 4.20](../../../docs/kernel/defined_behaviour.md#420-a-solve--list-names-what-the-program-has--m1f-s1f78)
   refuses a list that names nothing. A **dead** or **redundant** declaration prints one `note:` line on stderr
   and moves no exit code (the user: *"if not - then it filters nothing"*; Q-M1f row, defaulted). No lever,
   option or flag decides either. A corpus program in conflict is corrected where its conflict is an accident,
   or kept as a fixture of the refusal. Every new refusal is a section of § 4; Q-M1f rows for the defaults.
3. **T1f.6.34.3 — *forbid*'s remedies.** A remedy that the check would refuse is not offered: where R's
   candidates are an hrule's, the message names the hrule, not `:no-hypothesis`. Held by a test on E-47's
   witness.
4. **T1f.6.34.4 — the pages and the records:** `defined_behaviour.md`, the grammar's § Solve,
   `06_reserved_names.md` § Hypothesis / query control (the mechanisms table above made normative), the
   read-out reference if a printed line moves; E-47 closed in [`findings.md`](findings.md); the phase README's
   row; the stage's record.

## Acceptance

- Every row of § The pairs has a witness in the record and behaves as its verdict says: a conflict refused, a
  dead or redundant declaration noted, H + O silent.
- E-47's witness is refused for the conflict, naming the hrule and the list, and no longer for *forbid* with a
  remedy it already wrote; its twin without `listed_is_all` is refused for the same conflict.
- No program that loads and has no conflict changes its answer, its stdout or its exit code: the sweep over
  every tracked program, before and after; a dead or redundant declaration's `note:` is on stderr, and the
  record counts the programs that print one.
- *Forbid* offers no remedy the loader refuses.

## What it must not do

- **Not make `:no-hypothesis` or `:hypothesis-relations` filter the hrule rung**, nor the hrule rung read them:
  the ruling is to report the contradiction, not to resolve it one way.
- **Not move R** on any program that loads with no conflict; no lever, option or flag decides a conflict.
- Not touch F25's R-2 and R-3 (a bare name in `:hrules` that selects nothing) unless the census finds them to
  be a contradiction between two mechanisms, in which case they are shown and the stage says which.

## The standing rules

A program the stage newly refuses is shown with its answer first. A wrong answer of the engine met on the way is
a row in `corpus/wrong_answers.toml`, filed `open` before its fix. Findings go to [`findings.md`](findings.md)
under the next free id; questions to [`open_questions.md`](../open_questions.md) under the next free id. Every
answer exits 0. The gate is `./run_tests.sh`, then `./run_tests.sh --nightly` on the stage's last commit;
`utils/ci_container.sh` is not needed unless a DOT writer, the review bundle or what CI installs changes.

## Record, 2026-11-14

### T1f.6.34.1 — the pairs on real programs, shown before the loader moves, done

**How it is read.** A derived closure and a derived obligation activator exist only at root's fixpoint, so the
mechanisms are read there and nowhere else: `ein_infer::mechanisms::read`, new and behaviour-neutral, takes a
state and returns what the program declares at it, each hrule plan the generator would compile (one per hrule
without parameters, one per `:hrules` activator of one with them) with the relation its `:assert` proposes,
each obligation plan the state's activators compile with the relation its `(open ?R)` owes, the two lists as
`hypgen` reads them, and every `R` with `(__closed__ R)` at the state. An obligation is read as a **plan**,
*activated*, not as an instance owed at root, as an hrule is read as active and not by whether its `:match`
has a row: a declaration that contradicts another does so whether or not root happens to owe (the user:
*"filtered obligations are nonsense by definition"*); the census prints what root owes beside it. The census is
`ein-infer`'s example `mechanism_census` (`cargo run --release -p ein-infer --example mechanism_census -- [--json]
[FILE…]`), which loads each program, saturates root as `region::prepare` does (`examples/shared/census.rs`'s
`root_of`), reads the closures before and after, and prints per program the mechanisms present, the rung root's
universe took, and per pair the relations the pair's condition holds of. With no file it sweeps the corpus and the
register's witnesses.

**The table of § The mechanisms, against the code.** Three rows read otherwise than the code does, and are
corrected here (E-51, E-52):

1. **B, the blind enumerator, runs where no obligation rule is loaded, or the obligations rung declines** — not
   where *nothing is owed*. The ladder's second question is `!program.obligations.is_empty()`, a rule loaded,
   activated or not; a rung 2 that owes nothing at root proposes nothing, and the blind enumerator does not run
   (S1d.2.6's scope rule, *discharge where stated*, `oblgen`'s module docs). So `o_satisfied_blind.ein`, an
   obligation `(total-owed at is-a)` its root discharges, has H empty and R {∅}; `o_unactivated_blind.ein`, the
   same with the activator deleted and the rule still imported, has H empty and R {∅}; `no_o_blind.ein`, the
   same with the import deleted too, has 22 candidates and R at least 110 057 states (the budget). In the corpus
   the three stdlib modules read as programs (`stdlib/algebra.ein`, `bijection.ein`, `slots.ein`) load
   obligation rules and activate none. E-51; not a wrong answer: the ladder is as S1d.2.6 ruled it, by the
   obligation rules a program states.
2. **C is read by the generator alone.** `(__closed__ R)` is read by rungs 2 and 3 (`hypgen::relation_plan`,
   `oblgen`'s scoping), by the read-out's `outside` row (`scope.rs`, clause 2) and by `shape.rs`'s
   `emit_closed` pass; no matcher reads it. Its *second role*, an `absent` over `R` read closed-world, is a
   consequence of the first: a relation with no candidate is not in `D`, the static check's and *forbid*'s set of
   relations a commitment can change. An hrule that proposes a closed relation puts it in `D` whatever the
   closure says, which is E-47's witness (`has`, closed by `words`' closing rule and proposed by
   `maybe-apples`). E-52.
3. **Under H the two lists and C filter no candidate, and the read-out still reads them.** No rung but the
   first runs, and the hrule rung reads none of the three; `scope.rs`'s `outside` row reads all three as the
   program's own scoping: `h_outside_plain.ein`, an hrule proposing `at` and a goal over a declared `near`,
   prints `outside near — no candidate: no hrule proposes it`, and `h_outside_nh.ein` (`:no-hypothesis (near)`)
   and `h_outside_c.ein` (`(__closed__ near)`) print no `outside` line. So *dead* below means *filters no
   candidate*; R does not depend on a dead list, and a line of the read-out does. E-52.

Two precisions, which are not corrections: an obligation's generating role acts **at root**, since H is the
candidate list the ladder builds there (M1e2), so an obligation root discharges proposes nothing whatever the
lists say; and **a declared hrule with no activator still takes rung 1** (`h_inactive_nh.ein`: H empty, R
{∅}, its `:no-hypothesis` dead), which is [F25](../../followups/f25_roughness_found_outside_zebra.md)'s R-2 and
not a pair, so the stage leaves it there.

**The pairs, each on a witness.** `build/s1f.6.34/w/` (gitignored; `python3 build/s1f.6.34/gen_witnesses.py`
writes them, `python3 build/s1f.6.34/run_witnesses.py` runs each under every command, the release binary at
`748d7be5`, 2026-11-14). A shared base: `(relation at Color House …)`, two colours and two houses as `is-a`
facts, `(near H1 H2)` undeclared; the hrule `guess (?R ?T1 ?T2)` of `solve_lists.rs`; the obligation
`std.algebra`'s `(total-owed at is-a)`. `ein saturate` and `ein render program` exit 0 on every witness;
none states a claim, so `ein test` has nothing to check, exit 2; `ein oracle` answers as `ein solve` does.

| pair | witness | the census reads | `ein solve` today | verdict, as T2 builds it |
|---|---|---|---|---|
| H + O | `h_o.ein` | `at` owed and proposed | 9 states, 4 candidates; ∅ not in R, root owes 2 | silent |
| H + HR | `h_hr.ein`, `:hypothesis-relations (near)` | `guess` proposes `at`, the list leaves it out | 16 states, 4 candidates: the list is not read | **conflict** |
| | `h_hr_dead.ein`, `:hypothesis-relations (at)` | nothing left out | 16 states | **dead**: a note |
| H + NH | `h_nh.ein`, `:no-hypothesis (at)` | `guess` proposes `at`, the list names it | 16 states, 4 candidates: the list is not read | **conflict** |
| | `h_nh_dead.ein`, `:no-hypothesis (near)` | nothing proposed is named | 16 states | **dead**: a note |
| | E-47's `kinds_listed_has_hyp_stated.ein` | `maybe-apples` proposes `has`; `:no-hypothesis (has)`; `has` closed by `words` | refused by *forbid*, exit 1 (`ein test` 2), four lines naming `(solve :no-hypothesis (has))` as their first remedy | **conflict**, two lines (H + NH, H + C) |
| | its twin `nolisted_has_hyp_stated.ein` | the same three | 2 states, 1 candidate | **conflict**, the same two |
| H + C | `h_c.ein`, `(__closed__ at)` written | `guess` proposes a closed `at` | 16 states, 4 candidates: the closure is not read | **conflict** |
| | `h_c_derived.ein`, a rule derives it | the same, derived | 16 states | **conflict** |
| | `h_c_other.ein`, `(__closed__ near)` | nothing proposed is closed | 16 states | silent: C's other role |
| O + HR | `o_hr.ein`, `:hypothesis-relations (near)` | `at` owed, the list leaves it out | **R empty**, root owes 2, *stuck*, no candidate | **conflict** |
| | `o_hr_listed.ein`, `(at)` | — | 9 states | silent |
| O + NH | `o_nh.ein`, `:no-hypothesis (at)` | `at` owed and named | **R empty**, root owes 2, *stuck* | **conflict** |
| | `o_nh_other.ein`, `(near)` | — | 9 states | silent |
| | `o_nh_h.ein`, the same beside an hrule on `near` | `at` owed and named, H declared | R empty, root owes 2, 3 candidates (the hrule's) | **dead** (H + NH): a note; O + NH is a conflict only with no hrule |
| O + C | `o_c.ein`, `(__closed__ at)` | `at` owed and closed | **R empty**, root owes 2, *stuck* | **conflict** |
| | `o_c_derived.ein` | the same, derived | R empty, *stuck* | **conflict** |
| | `o_c_h.ein`, beside an hrule on `near` | `at` owed and closed, H declared | R empty, root owes 2, 3 candidates | **conflict**: the closure contradicts the obligation whatever generates |
| HR + NH | `hr_nh.ein`, `(at near)` and `(at)` | `at` in both | R {∅}, no candidate | **conflict** |
| | `hr_nh_redundant.ein`, `(at)` and `(near)` | `near` named, and left out by HR | 4 096 states, 12 candidates | **redundant**: a note |
| HR + C | `hr_c.ein`, `(at near)` and `(__closed__ at)` | `at` listed and closed | R {∅}, no candidate | **conflict** |
| NH + C | `nh_c.ein`, `(at)` and `(__closed__ at)` | `at` named and closed | R {∅}, no candidate | **redundant**: a note |

Every row behaves today as the table's verdict supposes: an hrule's candidates are untouched by the three
(H + HR, H + NH, H + C), and a state that owes a relation the lists or a closure take out of rung 2 is *stuck*,
R empty (O + HR, O + NH, O + C).

**The corpus, by the census** (`mechanism_census` over the 747 files of `corpus.toml`, which are also every
register witness; 526 load and reach root's fixpoint, and the other 221 are refused at the parse or the load on
every command, `ein render program` exiting non-zero on each). The mechanisms present, by program: none 138; C
alone 152; NH 66; H 58; HR 22; O + NH 34; H + O 19; O + C 9; O 9; H + O + C 8; an obligation rule with no
activator 3 (the stdlib modules) and 2 beside an hrule; NH + C 2; O + NH + C 2; H + O + NH 1; HR + C 1. Per pair:

| pair | programs that carry it | of which its condition holds of a relation | so, as T2 builds the check |
|---|---|---|---|
| H + O | 30 | — | silent |
| H + HR | 0 | 0 | — |
| H + NH | 1 | 0 | 1 dead list: `examples/features/13_mixed_solution_and_open.ein` |
| H + C | 8 | 0 | silent |
| O + HR | 0 | 0 | — |
| O + NH | 37 | 23, 22 of them with no hrule | **22 conflicts** |
| O + C | 19 | 2 | **2 conflicts** |
| HR + NH | 0 | 0 | — |
| HR + C | 1 | 0 | silent |
| NH + C | 4 | 1 | 1 redundant list: `examples/word_problems/kinds_listed.ein`, `has` named and closed by `words` |

H + O is the zebra form: `zebra.ein`, `zebra2.ein` and its two variants, the lattice fixtures, the calculator's
search programs; the census finds no closure and no list on a relation any of them owes.

**The corpus programs in conflict, each with its answer today** (`python3 build/s1f.6.34/corpus_today.py`,
`build/s1f.6.34/corpus-before.txt`). Twenty-four, every one a deliberate fixture: each switches the obligation's
generating role off, by `:no-hypothesis` on the relation it owes or by a closure of it, so that the program's
question is root's saturation or root's verdict alone and the search has nothing to do. None is an accident of
spelling. Every one answers today with exit 0 and its claim held.

| group | programs | the relation | root | today |
|---|---|---|---|---|
| A, root discharges every debt | `tests/stdlib/algebra/24_total_owed_satisfied`, `26_surjective_owed_satisfied`, `tests/stdlib/slots/01_partition`, `08_spatial_adjacent`, `10_owed_room_satisfied`, `12_owed_fill_satisfied`, `examples/ein-bugs/minimal-line-stated-obligation` | `likes`, `co-loc` in `:no-hypothesis` | owes nothing | R {∅}, no candidate; claim held (the last states none) |
| B, root owes, its question R empty | `tests/region/04_owing`, `05_empty`, `tests/stdlib/algebra/23_total_owed`, `25_surjective_owed`, `tests/stdlib/slots/09_owed_room`, `11_owed_fill` | `likes`, `co-loc` | owes 1 (`likes`) or 3 (`co-loc`) | R empty, *stuck*; `(owing …)` or `(region)` held |
| B, root owes, its question root's saturation | `tests/stdlib/bijection/01_setup_and_negatives`, `02_domain_elimination`, `03_range_elimination`, `tests/stdlib/slots/02_negative`, `03_fill`, `04_elimination`, `07_spatial_prune`, `13_adjacent_bwd_neg_direction` | `wears`, `co-loc`, owed through `bijective-setup` and `slot-partition-setup` | owes 4 (`wears`), 10, 12 or 16 (`co-loc`) | R empty, *stuck*; `(owing (model …))` of root's facts held |
| C | `tests/stdlib/bijection/06_blind_enumeration` | `is-a`, owed by its own `sorted-somewhere`, written to make the rung decline | owes nothing | 1 state, 16 candidates, all dead alone; claim held |
| D, O + C | `tests/stdlib/closure/02_closed_and_satisfied`, `03_closed_and_owing` | `r`, closed by `infer-closure` and owed by `(total-owed r is-a)` | 02 owes nothing, 03 owes 1 | 02 R {∅}; 03 R empty, *stuck*; claims held |

What the search would answer were the filter dropped (`ein solve --count`, each with the owed relation taken out of
`:no-hypothesis`): group A, the same, R {∅}; `04`, `05`, `23`, `25` 3 states each; `bijection/01` to `03` 2 each;
`slots/07` 8, `09` and `11` 27 each, `13` 72; `slots/02`, `03`, `04` past `--region-budget`, not monotone
(`probe-undecided`'s `unknown`), the cube 2^24; `bijection/06` blind over `is-a`, not finished.

The two notes, today: `examples/features/13_mixed_solution_and_open.ein`, an hrule proposing `choice` beside
`:no-hypothesis (is-a likes)` and an obligation owing `likes`: 1 state, 2 candidates, claim held;
`examples/word_problems/kinds_listed.ein`: 1 state, no candidate, three claims held.

Nothing else in the corpus is in a pair's condition. F25's R-2 and R-3 are not met by the census as a
contradiction between two mechanisms: no program declares an hrule that is inactive at root, and a bare name in
`:hrules` is no mechanism of its own. The two programs that load an obligation rule no activator of root's
compiles beside an active hrule, `examples/lattice/09_activator_hole.ein` and `12_activator_chain_hole.ein`, are
H + O, silent.

### T1f.6.34.2 — the report, done

**The check.** `ein_infer::mechanisms::check` saturates a program's root and judges what `read` finds there:
`judge` walks the pairs in the table's order and writes one line per conflict, placed where the program writes the
declaration (the list's name, the list where it names none, `()` at its keyword, the hrule's `:assert`, a written
activator or closure by its form), and naming both declarations and the relation: an hrule with its activator as
`:hrules` writes it, an obligation with the activator that compiles it and where that activator comes from
(*written at FILE:L:C*, or *which rule 'r' derives*), a closure the same way. Where nothing conflicts it writes the
notes: a list the program declares beside an hrule, which filters no candidate (E-52), and, with no hrule, a
`:no-hypothesis` name `:hypothesis-relations` already leaves out or a closure already closes. The verdicts, as built:

| pair | conflict | note |
|---|---|---|
| H + O | never | — |
| H + HR | an active hrule proposes a relation the list leaves out | the list is dead |
| H + NH | an active hrule proposes a relation the list names | the list is dead |
| H + C | an active hrule proposes a closed relation | — |
| O + HR | no hrule, an obligation owes a relation the list leaves out | — |
| O + NH | no hrule, an obligation owes a relation the list names | — |
| O + C | an obligation owes a closed relation, whatever generates (Q-M1f.244) | — |
| HR + NH | a relation in both | no hrule: a name HR already leaves out |
| HR + C | a listed relation is closed | — |
| NH + C | never | no hrule: a name a closure already closes |

An obligation is its plans at root, owed or not (Q-M1f.243); `:hypothesis-relations ()` is a list that names none,
as `hypgen` reads it, so beside an obligation or an active hrule it leaves every relation out.

**Where it runs** (Q-M1f.245). `ein-cli`'s `common::load_held`, the one load path of every command, after the
loader's notes: where `mechanisms::may_conflict` says the program could declare two mechanisms at all (two of H, O,
HR and NH as written, or one and `__closed__` written anywhere), it loads the program a second time into its own
tables (a text from its text, a container from its bytes), saturates that root and judges; a conflict is the load
error, `kb load error: <line>` each, at the command's load-error exit (1; `ein test`: 2), and the notes go through
the loader's `say_notes`, once a process. The command's own tables are untouched. A program whose root does not
saturate is judged no further and the command reports what it reports. Not in `region::prepare`: a library caller
calls `check`.

**What prints** (`build/s1f.6.34/w/`, the release binary of this commit, 2026-11-14):

```text
$ ein solve h_nh.ein
kb load error: h_nh.ein:9:57: solve: its :no-hypothesis names at, and hrule 'guess' (at Color House) proposes facts of at — no list filters an hrule's candidates, so the program says both that at is guessed and that it is not: drop at from the list, which filters nothing where an hrule is declared, or the hrule's at
$ ein solve o_c_derived.ein
kb load error: obligation 'total-owed', activated by (total-owed at is-a), owes at, and (__closed__ at), which rule 'close-at' derives, says at is closed — an obligated relation is not closed: an obligation is paid by facts of the relation it owes, so a state that owes at and may not be given one is stuck; drop the closure, or the obligation
$ ein solve --count nh_c.ein
note: nh_c.ein:6:24: solve: its :no-hypothesis names at, which (__closed__ at), written at nh_c.ein:5:2, already closes
```

E-47's witness is refused for two conflicts, H + NH and H + C on `has` (the second: *hrule 'maybe-apples' proposes
facts of has, and (__closed__ has), which rule 'words' derives, says has is closed — …*), and no longer by *forbid*;
its twin without `listed_is_all`, which answered two states, is refused for the same two. Every witness of T1 behaves
as the table's last column says (`witnesses-after.txt`).

**The corpus, corrected** (Q-M1f.246; each shown with its answer in T1). Every program that loads and has no
conflict answers as it did: the sweep (`build/s1f.6.34/sweep.py`) ran `ein solve` and `ein test` over the 854
tracked `.ein` files with the release binary at `ddc48e4b` and with this commit's, stdout hashed with `ein test`'s
wall clock taken out (`sweep_diff.py`): no exit and no stdout moved outside the twenty-four, and two programs print a
note, `examples/features/13_mixed_solution_and_open.ein` (*its :no-hypothesis filters no candidate — the program
declares hrule 'pick'*) and `examples/word_problems/kinds_listed.ein` (*names has, which (__closed__ has), which rule
'words' derives, already closes*). Of the twenty-four:

- **Fourteen answer as before**, twelve of them printing as before too, the old text under the old binary against
  the new text under the new, on `solve`, `solve --single`, `--json-summary` and `test`: group A, the owed relation
  taken out of `:no-hypothesis`, root owing nothing so the rung proposed nothing either way
  (`tests/stdlib/algebra/24`, `26`, `tests/stdlib/slots/01`, `08`, `10`, `12`,
  `examples/ein-bugs/minimal-line-stated-obligation.ein`); `tests/region/04_owing.ein` and `05_empty.ein`, `likes`
  out of the list and Ann's two likings stored false, so the debt is unpayable by the program's own facts and R stays
  empty, root owing; `tests/stdlib/bijection/06`, its `sorted-somewhere` a bare `(open)`, which declines the rung for
  its own reason; `tests/stdlib/closure/02`, its obligation dropped, `Solution` by exhaustion where it was by
  discharge; and `tests/stdlib/bijection/01` to `03`, whose subject is root's saturation under `bijective-setup`'s
  fan-out, the obligations among it: each declares its own generator, an hrule `as-stated` that proposes only the
  `wears` root holds, so nothing new is guessed and the obligations judge root, which owes, R empty, the claim
  unmoved (`01` prints as before; `02` and `03` one `outside` line more, `undecided`, which the list scoped out).
  The bijection fixtures first let the obligations propose, R the two bijections; `utils/stdlib_mutants.py --bless`
  then banked six new survivors, `bijective-setup`'s activators of the two negative-completion rules swapped and
  `domain-elimination`'s and `range-elimination`'s type premises swapped, since a state holding the whole bijection
  derives each negative two ways and each forced arrow by search too; with the hrule all six die again
  (`stdlib_mutants.py -k` on the three rules: 8 of 8, 7 of 7, 3 of 3).
- **Nine answer a different R, by their own edit and not the engine's**, each claim restated and held: the
  obligation rules' own tests let the obligation propose, `tests/stdlib/algebra/23_total_owed` and `25` (R the three
  states that pay Ann's or Soup's debt), `tests/stdlib/slots/09_owed_room` and `11_owed_fill` (three negatives each,
  so that one candidate is left per debt and R is the one state that pays all three, where it would be 27); and
  `slot-partition`'s fan-out is written out without its two obligations in `tests/stdlib/slots/02`, `03`, `04`,
  `07` and `13`, R root, `(owing (model …))` restated `(region (model …))` over the same facts.
- **One is the refusal's fixture**: `tests/stdlib/closure/03_closed_and_owing.ein`, moved to
  `examples/ein-bugs/closed-and-owing.ein`, its corpus entry `regression`, `solve`, `saturate` and `render rules`
  each exit 1.

No register row: no program that loads with no conflict changed its R, and the nine that did changed by their own
text.

### T1f.6.34.3 — *forbid*'s remedies, done

*Forbid*'s first remedy was *close the relation, `(solve :no-hypothesis (R))`, if no commitment is meant to extend
it*, on every program. Where the program declares an hrule no list filters a candidate, so the form lifted nothing,
and since T2 it is a conflict wherever the hrule proposes R. Since this task the remedy reads the ladder:
`forbid::Proposers`, read at root by `region::prepare` where a refusal is about to be returned (the active hrule
plans and the relation each proposes), and `Refusal::text` writes, where the program declares an hrule,

- *let no hrule propose R, if no commitment is meant to extend it — hrule 'h' proposes it, and no list filters an
  hrule's candidates*, naming every active hrule that proposes R (*hrule 'h' (args)* for one with parameters);
- *let no hrule propose what R is derived from, if no commitment is meant to extend it — the hrules are this
  program's commitments, and no list filters their candidates*, where none proposes R itself (R in `D⁺` by
  derivation, or the relation a parameter names);

- with no hrule, where an obligation of root's activators owes R, *let no obligation owe R, if no commitment is meant
  to extend it — obligation 'o' owes it, and no list may filter what an obligation owes* (O + NH being a conflict
  too);

and `(solve :no-hypothesis (R))` as before otherwise. The other two forms are unchanged. Nothing else moves: the
refused set is the same, and only a refusal's text where an hrule is declared or an obligation owes the relation.

**On E-47's witness**, through the library, whose routes do not run T2's check: four lines, each *over has, a
relation a commitment can still extend … let no hrule propose has, if no commitment is meant to extend it — hrule
'maybe-apples' proposes it, and no list filters an hrule's candidates*, and none names `:no-hypothesis`; the same
without the list; the twin without `listed_is_all` answers (`refutation_under_absent.rs`'s
`beside_an_hrule_the_remedy_names_the_hrule_and_not_the_list`). Beside an obligation, a program whose `hungry`
concludes `(false)` from `(absent (likes ?p Soup))` while `(total-owed likes is-a)` owes `likes`: the remedy names
`total-owed`, and the same program without the obligation still names the list
(`beside_an_obligation_the_remedy_names_the_obligation`).

**The register**: ten rows' recorded refusal moves to the new text, every one a witness that declares an hrule,
`W1/alive-empty-interlayer`, `W3/three-components`, `W3/trigger-hole`, `W16/activator-all-int`,
`W16/activator-all-nested`, `W17/forbid-chain-1`, `-2`, `-node-absent`, `-param-absent` and `-node-writes`: eight
name the hrule that proposes the relation, and `-param-absent` and `-node-writes`, whose relation a rule derives,
say *what … is derived from*. Restated in `corpus/wrong_answers.toml` from `wrong_answers.rs`'s report, no status
moved.

### T2 and T3, the tests and the goldens

**New**: `ein-cli/tests/mechanism_conflicts.rs`, four tests: eleven refused shapes (H + HR, H + NH, H + C written and
derived, O + HR, O + HR with `()`, O + NH, O + C, O + C derived beside an hrule, HR + NH, HR + C), each on the twenty
commands of `solve_lists.rs` with one line, the same on each, the load-error exit, and no container; twelve that
load, six silent (H + O, H + C on another relation, O + HR listing the owed relation, O + NH and O + C on another,
HR + C on another) and six with their one note (H + HR and H + NH dead, an inactive hrule's dead list, O + NH beside
an hrule, HR + NH and NH + C redundant), under five commands and reopened from the container `kb save` wrote;
E-47's witness and its twin, two lines each and none of *forbid*'s; the corpus's fixture. `ein-infer`'s
`refutation_under_absent.rs` gains T3's test.

**Moved with the fixtures**: `obligation_reports.rs` (the owing fixture of *a state that owes is Open* is
`tests/region/04_owing.ein`; an owing root is no state of R and the obligation proposes what pays it; the closed and
owing pair is `the_closed_and_owing_corner_is_a_conflict`, `02` loading with no obligation and the fixture refused by
`mechanisms::check`, one line naming `total-owed`, its activator and `infer-closure`); `obligation_rung.rs` (the
stuck fixture is `04_owing`, owed 1, branched on 1, declined 0, where `23` was owed 1, declined 1); `read_out.rs`,
`region_modes.rs`, `region_answer.rs`, `summary_properties.rs`, `embedding.rs`, `test_cli.rs` and `golden_trace.rs`
read `04_owing` (and `05_empty`) where they read `03_closed_and_owing` or `slots/03_fill` as the owing example;
`models_grid.rs` writes its own owing program with a functional relation, the grid's row; `test_cli.rs`'s floor of
owing claims in the corpus is four, where it was ten; `solve_lists.rs` reads its four list shapes beside a `plain`
with a parameter and no activator, so that the list it holds names is dead and not in conflict.

**Goldens, counted before blessing** (`EIN_BLESS=1`, 2026-11-14): `corpus_exits.txt`, nine cells: three new
(`examples/ein-bugs/closed-and-owing.ein`'s `solve`, `saturate`, `render rules`, each 1), five gone
(`tests/stdlib/closure/03_closed_and_owing.ein`'s), and one moved, `tests/stdlib/bijection/01`'s `render rules`, 1
to 0, its hrule a form the view draws where it held none; `corpus_shapes.md5`, 736 digests: 42 new and 42 gone (the
moved fixture), 526 moved over the 23 corrected programs (their IR, their text having moved; for eleven their read-out
too, nine of them a different R), and 168 over twelve programs *forbid* refuses that declare an hrule (14 each, the
ops that print the refusal: T3's text), `examples/ein-bugs/activator-all-int`, `-nested`, `-symbol`,
`alive-empty-interlayer`, `wrong-answers/forbid-chain-0`, `-1`, `-2`, `-node-absent`, `-node-writes`,
`-param-absent`, `lattice/synth/three_components` and `trigger_hole`; `trace_owing.md`, the owing trace of
`04_owing`, where it was `03_closed_and_owing`'s. `docs/kernel/inference/read_out_examples.md`, regenerated by
`utils/read_out_examples.py`: X6 is `04_owing`. `tests/mutation_survivors.txt`: two lines gone, `bijective-setup`'s
`total-owed` and `surjective-owed` activators swapped, killed since; `python3 utils/stdlib_mutants.py --check`
(2026-11-14): 439 of 500 killed, 61 survivors, the banked set, exit 0.

### T1f.6.34.4 — the pages and the records, done

- **`defined_behaviour.md`**: § 4.27, new, *A program's hypothesis mechanisms do not contradict one another*: the
  pairs, refused or noted; what is read and where (root's fixpoint, an active hrule, an obligation as declared,
  `()`); where it runs (every command, at load, a second load's root); each conflict's line and the notes; what each
  pair answered until the stage, E-47, and the corpus's twenty-four; the pins. § 4.7: where the program declares an
  hrule, *forbid*'s first remedy names the hrules that propose the relation. § 4.20: a list that names what the
  program has and contradicts another declaration is § 4.27's. § 5: the cap-of-zero example of `Open — owes 1` is
  `04_owing` since.
- **`01_grammar.md` § Solve**: the two lists are read by the obligations rung and the blind enumerator, never the
  hrule rung; and the lists do not contradict the program's other hypothesis declarations, with the pairs in a
  sentence and § 4.27's link.
- **`06_reserved_names.md` § Hypothesis / query control**: the table of § The mechanisms, corrected by T1 (B, C and
  what reads the lists, E-51 and E-52), made normative as *The five mechanisms, and each pair*, with the pairs'
  verdicts; the rows of `__closed__`, `hypothesis-relations` and `no-hypothesis` say what reads them (and that
  `emit_closed` runs on no command since M1e2 S1e2.3.1, where the row said *usually auto-inferred*); the `Open`
  verdict's example is `tests/region/04_owing.ein`.
- **`read_out.md`**: § 10's stderr table gains the load-time `note:` line; X6's four links follow the example.
  **`read_out_examples.md`**, regenerated: X6 is `tests/region/04_owing.ein`, whose prose says why it owes.
- **`AGENTS.md`**: § The language gains the rule, beside *forbid*'s.
- **`tests/README.md`**: the closure directory's row, `bijection/06`'s decline, and how a fixture says what the
  obligation's generating role was switched off to say. **`examples/README.md`**: `ein-bugs/closed-and-owing.ein`.
  Four links of `docs/history/` and one of a P1f.10 stage file to the moved fixture follow it to its new path,
  their text as it was.
- **Records**: E-47 closed in `findings.md`; the phase README's row; this file's status and record; Q-M1f.242 to
  .246 in `open_questions.md`.

### The stage, closed

**The commits**: T1's record and reader, `ddc48e4b`; T2 and T3, the check, the remedy, the corpus corrected, the
tests and goldens, `e45c4c18` (the fixture's rename, staged with `git mv`, went into the index the coordinator's
records commit `03365506` was made from, so that commit holds the rename and `e45c4c18` the rest); T4, the pages
and the records (this one).

**The acceptance, point by point.**

- *Every row of § The pairs has a witness in the record and behaves as its verdict says*: T1's table, twenty-one
  witnesses and E-47's two, each run before (`witnesses-before.txt`) and after (`witnesses-after.txt`); the eleven
  refused shapes and twelve accepted ones of `mechanism_conflicts.rs` hold it on every command. H + O is silent on
  every corpus program that carries it (30).
- *E-47's witness is refused for the conflict, naming the hrule and the list, and no longer for forbid*: two lines,
  H + NH (`:no-hypothesis names has, and hrule 'maybe-apples' proposes facts of has`) and H + C (`has` closed by
  `words`), no line of *forbid*'s; its twin without `listed_is_all` the same two.
- *No program that loads and has no conflict changes its answer, its stdout or its exit code*: the sweep over the 854
  tracked `.ein` files, `ein solve` and `ein test` under the binary of `ddc48e4b` and of this stage, moved none but
  the corrected twenty-three and the moved fixture; two programs print a `note:`
  (`examples/features/13_mixed_solution_and_open.ein`, `examples/word_problems/kinds_listed.ein`), on stderr.
- *Forbid offers no remedy the loader refuses*: beside an hrule it names the hrules, never `:no-hypothesis`; with
  no hrule, beside an obligation that owes R, it names the obligation; and otherwise `(solve :no-hypothesis (R))`
  is no conflict, R being no active hrule's and owed by no obligation (`refutation_under_absent.rs`'s two tests).
  A closure of R, its first form, is a conflict only beside an hrule that proposes R or an obligation that owes it,
  and *forbid* names neither as a closure there.

**Defaults** (each the user's to revise): Q-M1f.242, a dead or redundant declaration is one `note:`; Q-M1f.243, an
obligation read as declared; Q-M1f.244, O + C a conflict whatever generates; Q-M1f.245, the check at load on every
command, on a second load's root, not in `region::prepare`; Q-M1f.246, the corpus's twenty-four corrected, one kept
as the refusal's fixture.

**Findings**: E-51 (the blind enumerator runs where no obligation rule is loaded) and E-52 (`__closed__` is read by
the generator alone; under an hrule the lists are read by the `outside` row alone), neither a wrong answer; E-47
closed. **No register row**: no program that loads with no conflict moved, and the nine whose R moved moved by their
own text; ten rows' recorded refusal text moved with T3.

**What was not done, and why.** The library's routes (`ein_infer::explore`, `region`) do not run the check
(Q-M1f.245), so `forbid::studying`'s fixtures and the census examples read a conflicting program as before.
F25's R-2 and R-3 are untouched: the census met neither as a contradiction of two mechanisms. The
gate and the nightly are the coordinator's, on this commit.
