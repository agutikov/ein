# S1f.6.34 — A conflict between the hypothesis mechanisms is reported

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-13, on the user's ruling of that day on E-47; started 2026-11-14, T1 done ([§ Record](#record-2026-11-14)). **An engine stage.**
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
