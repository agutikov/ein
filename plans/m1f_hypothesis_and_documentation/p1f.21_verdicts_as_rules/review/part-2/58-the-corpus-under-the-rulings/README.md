# 58 · The corpus under the rulings: every program that would move

[← part 2](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.12_hypotheses_as_facts.md) ·
[your answers](../ANSWERS.md)

**What this is.** The list task 1.1 of [S1f.21.12](../../../s1f.21.12_hypotheses_as_facts.md) asks for before
anything is built: every corpus program whose candidates, answer, claim or flag would change under your rulings
Q-M1f.566 to Q-M1f.579, with what it is today and what it would be. The second part is now built, and
[On the built engine](#on-the-built-engine), at the foot of the page, takes the list again from the two binaries
and sets this prediction beside what happened.

**The answer, short.** Of the 931 programs of the corpus, **15 that load today would answer otherwise**, and **20
that are refused today would load**:

- **5 by the union** (Q-M1f.570): `zebra.ein`, `13_witness_of_two.ein`, `14_seats_nothing_proposes.ein`,
  `08_absent_witness_hole.ein`, `06_friends_tables_everyone_sits.ein` gain candidates; the first four answer
  otherwise (`zebra.ein` by its count of commitments, its one state the same), and the claims of `13_witness_of_two.ein` and `14_seats_nothing_proposes.ein` fail.
- **8 by an obligation that no longer steps aside** (Q-M1f.579): the eight `oblig-*.ein` witnesses. **6 become
  flagged**, and every one of the six is read by a suite today: three by a `test` run, three by a regression row.
- **1 becomes an error under the lists** (Q-M1f.578): `13_mixed_solution_and_open.ein`, which has a `test` run.
- **1 is refused for its bare `(open)`** (Q-M1f.566): `tests/stdlib/bijection/06_blind_enumeration.ein`, under
  `tests/`, with a `test` run and a regression row. Item [63](../63-the-bare-open/README.md) has it.
- **20 regression witnesses of `open` written elsewhere would load**, by the rulings (15) and by the default of the
  question this page puts (5 more); 10 stay refused.
- **None by a hypothesis first held above root** (Q-M1f.574): the three programs with such a rule are flagged
  today. **None by a rule that reads an `open` fact** (Q-M1f.569): no program that loads today can read one.

**No program under `tests/` becomes flagged.** The programs a suite reads that move are in
[§ 8](#8-what-a-suite-reads-that-moves).

## 1. How it was taken

Nothing is built: every run is **today's engine**, `build/review-bin/ein-head-edbe6c3b` (the gate build of the
engine at this stage's branch point), with the checkout's `stdlib/`, on **the corpus as committed at `669d7d43`**.
The corpus is frozen there for this page: S1f.21.17 and S1f.21.11 run beside this task and a few programs gain an
import in their work (`std.core` and `std.contradiction` by import), which this census does not see.

`tools/run.sh` re-takes everything into `runs/` (both ignored by git; mirrored under
`build/review-incoming/p1f21-s12-examples/58-the-corpus-under-the-rulings/`). `runs/census.md` is what the tools
wrote; every number below is in it.

| pass | what it runs | what it gives |
|---|---|---|
| 1 | every program: `ein solve --monotone --events --json-summary` (no search) and `ein show hyps` | the rung that builds H today, H, D (the relations a commitment can change), the static check's sites |
| 2 | each program on the rung `hrules`, as **B**: its hypothesis rules and `:hrules` clause deleted (in a copy of its directory) | whether an obligation is activated beside the rule, and what it proposes |
| | and as **U**, where B proposes what A does not: the program with one hypothesis rule more per such candidate, empty `:match` | H as the union, with today's engine; R described in full, the claim checked |
| sites | the rules of each program and its modules, under their activators, against D | the two new static sites: a hypothesis rule that reads D (Q-M1f.574), an obligation's guard that reads D (Q-M1f.579) |
| lists | each program whose root would hold an `(open F)`, against its lists and root's `(__closed__ R)` facts | the errors of Q-M1f.578 |
| 3 | the eight step-aside programs as they are and with **their guard read at root** (item [64](../64-an-obligation-that-no-longer-steps-aside/README.md)'s stand-ins) | R today and on the stand-in |
| witnesses | every program that writes `open` other than as an obligation's whole `:assert`, as it is and with `opn` for `open` | today's refusal; what the stand-in answers |

**Three kinds of number.** A *today* column is a run of today's engine on the corpus program. A *union* or
*stand-in* column is a run of today's engine on a variant that stands for the built engine: U, the guard read at
root, `opn`. A *flagged after* or *error* cell is a reading: the static check's rule as ruled, applied to D and to
the rules, or the rule of Q-M1f.578 applied to root's `open` facts; no binary computes it today.

## 2. The union (Q-M1f.570)

95 programs are on the rung `hrules` today. With their hypothesis rules deleted (B):

| B | programs | what the union does |
|---|---:|---|
| the blind enumerator | 61 | nothing: no obligation is activated, so the union is the rule's candidates, as today |
| the obligations rung, proposing nothing A does not | 22 | nothing: B ⊆ A (`zebra2.ein`, `send_more_money.ein`, `boxes12.ein` and 19 more) |
| **the obligations rung, proposing more** | **5** | **H grows** (the table below) |
| the obligations rung stepping aside | 4 | nothing: under Q-M1f.579 the obligation writes its guard's candidates at root; two owe nothing at root, and the other two's guard gives `(h A)`, which the rule proposes already |
| refused, or not to be read | 3 | `13_mixed_solution_and_open.ein`: B is refused for its own list (§ 4); `query-full.ein`: B is refused for a goal only the rule names, and it activates no obligation; `tests/saved/04_a_saved_node.ein`: its rule sits in a saved program's `(program …)` form, and it activates no obligation |

`boxes12.ein` and `boxes13.ein` import their hypothesis rule from a module by name; B takes the name out of the
import's `:symbols` and is then on the obligations rung with A's two candidates.

**The five, today and under the union** (pass 2: A as it is, U the union):

| program | candidates | states of R | commitments of R | flagged | its claim | what the union adds |
|---|---|---|---|---|---|---|
| [`zebra.ein`](../../../../../../examples/zebra.ein) | 56 → 112 | 1 → 1 | 261 982 → 68 719 459 314 | no → no | none stated | 56 facts of `co-located` the obligations propose and the rule does not |
| [`13_witness_of_two.ein`](../../../../../../examples/lattice/13_witness_of_two.ein) | 3 → 4 | 5 → 8 | 5 → 13 | no → no | **held → fails** | `(fed Ann Soup)`, supposed outright: the meal without anyone who orders, cooks or shops |
| [`14_seats_nothing_proposes.ein`](../../../../../../examples/lattice/14_seats_nothing_proposes.ein) | 12 → 16 | 4 → 8 | 4 → 8 | no → no | **held → fails** | Cat and Dan at the small table, which the rule's guard excludes: four more seatings, the pair at the big table |
| [`08_absent_witness_hole.ein`](../../../../../../examples/lattice/08_absent_witness_hole.ein) | 3 → 4 | 5 → 11 | 5 → 13 | **flagged** → **flagged** | none stated | `(meal Ann Soup)`, supposed outright |
| [`06_friends_tables_everyone_sits.ein`](../../../../../../examples/lattice/06_friends_tables_everyone_sits.ein) | 19 → 25 | 8 → 8 | 8 → 8 | no → no | none stated | six seats the rule excludes; R is the same |

The union flags none that is not flagged today: U's static check has the same D as A's, and no new site reads
it. `13_witness_of_two` and `14_seats_nothing_proposes` have a `solve` run that checks their `(solve :expect)`, so the build restates each
claim as the union's R, or the program says what it means another way (a rule that denies what the guard
excluded). [The page of Q-M1f.570](../../../q-m1f.570_override_or_union/README.md) has three of them three ways.

## 3. An obligation that no longer steps aside (Q-M1f.579)

Nine programs step aside today; one, `06_blind_enumeration.ein`, for its bare `(open)` (§ 5). The eight others step
aside because the obligation's guard reads a relation a commitment can change. Ruled: it writes its guard's
candidates at root and the program is flagged. Item [64](../64-an-obligation-that-no-longer-steps-aside/README.md)
has each program and its stand-in.

| program (all under `examples/ein-bugs/wrong-answers/`) | candidates today → guard at root | states today → stand-in | claim today → stand-in | flagged today → after | read by |
|---|---|---|---|---|---|
| [`oblig-derived-domain.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-derived-domain.ein) | 6 → 1 | 1 → 0 | held → fails | no → **flagged** | a regression row |
| [`oblig-derived-domain-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-derived-domain-declared.ein) | 6 → 2 | 1 → 1 | held → holds | no → **flagged** | a `test` run |
| [`oblig-fresh-name.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name.ein) | 6 → 1 | 1 → 0 | held → fails | no → **flagged** | a regression row |
| [`oblig-fresh-name-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name-declared.ein) | 6 → 2 | 1 → 1 | held → holds | no → **flagged** | a `test` run |
| [`oblig-fresh-name-two.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name-two.ein) | 12 → 2 | 12 → 0 | held → fails | no → **flagged** | a regression row |
| [`oblig-fresh-name-two-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name-two-declared.ein) | 12 → 4 | 12 → 3 | held → fails | no → **flagged** | a `test` run |
| [`oblig-stdlib-total-owed.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed.ein) | 6 → 1 | 16 → 0 | held → fails | **flagged** → **flagged** | none |
| [`oblig-stdlib-total-owed-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed-declared.ein) | 6 → 2 | 16 → 1 | held → fails | **flagged** → **flagged** | none |

*Holds* and *fails* are what the stand-in's run prints. **Once built, nothing is claimed of any of the eight**: each
is flagged, so a claim it states may be checked by whoever asks, and no suite may hold it (Q-M1f.564). Each of the
first six also has a `solve` run, which checks the claim; the build takes the `test` runs, the three regression rows
and the claims or those runs out.

## 4. The lists and a closure (Q-M1f.578)

136 programs would hold an `(open F)` at root once built; 23 of them write `:hypothesis-relations` or
`:no-hypothesis`, or hold a `(__closed__ R)` at root. **One disagrees**:

| program | root's `open` facts are of | its list | today | once built |
|---|---|---|---|---|
| [`13_mixed_solution_and_open.ein`](../../../../../../examples/features/13_mixed_solution_and_open.ein) | `choice` (its rule), `likes` (its obligation, root owes 1) | `:no-hypothesis (is-a likes)` | loads: the rule overrides, the obligation proposes nothing | **an error**: a `:no-hypothesis` that names an `open` fact's relation |

It has a `test` run. The 22 others agree: no `:hypothesis-relations` stands beside an `open` fact in the corpus,
and no `:no-hypothesis` or closure names an `open` fact's relation. The one program refused today for a closure
beside an obligation, `examples/ein-bugs/closed-and-owing.ein`, stays refused: root owes there, so the closed
relation has `open` facts. Item [65](../65-the-lists-and-a-closure/README.md) has the eight programs of the
question's page and two questions the ruling leaves.

## 5. The bare `(open)` (Q-M1f.566)

Three programs write it. Two are refused today for their own reason and stay refused
(`macro-eq-obligation.ein`, `macro-not-conjunction-obligation.ein`). The third answers today:
[`tests/stdlib/bijection/06_blind_enumeration.ein`](../../../../../../tests/stdlib/bijection/06_blind_enumeration.ein)
uses `(open)` so that the obligations rung steps aside and the blind enumerator runs. Once built it is refused.
Item [63](../63-the-bare-open/README.md) has the ways to its claim, and the one it can no longer state.

## 6. A hypothesis first held above root (Q-M1f.574), and a rule that reads `open` (Q-M1f.569)

**The new static site of Q-M1f.574**, a rule that concludes `(open F)` and reads a relation of D: three programs
have a hypothesis rule that reads `is-a`, which a commitment can change there:
`examples/lattice/04_obligation_trigger_hole.ein`, `examples/lattice/17_flagged_region_claim.ein` and
`examples/ein-bugs/wrong-answers/past-limit-lines.ein`. **All three are flagged today** (an obligation's trigger
reads `is-a`), so none moves. The check in flight, at each state a run builds, is not taken by this census.

**The new site of Q-M1f.579**, an obligation's guard that reads D: the eight programs of § 3, and the same three.

**A rule that reads an `open` fact**: none in the corpus. No pattern of a program that loads today is headed by
`open`, and a pattern's relation is a name or a parameter an activator binds: the three programs that write a
relation no parameter binds are refused today (`runs/readers.tsv`). Once built, one of § 7's witnesses would be
such a program, `open-derived-activator.ein`.

## 7. `open` written elsewhere: the witnesses of Q-M1f.66 and Q-M1f.86

31 programs write `open` other than as an obligation's whole `:assert` (`runs/open-tokens.tsv`). 30 are refused
today, each the witness that a place refuses it; 26 of them hold that refusal in a regression row. Your rulings make
some of those places legal: *under an ordinary relation `open` and `absent` are ordinary names* (Q-M1f.571), a
written `(open P)`, `(not (open P))`, a rule that reads `open` (Q-M1f.569). Other places are not ruled: the
question below.

| once built | programs |
|---|---|
| **loads, by the rulings** (15) | `open` or `(open …)` under an ordinary relation: `open-claim-fact-head`, `open-claim-value`, `open-conclusion-value`, `open-fact-value`, `open-goal-pattern-head`, `open-goal-value`, `open-hrule-value`, `open-is-a-object`, `open-macro-argument`, `open-macro-value`, `open-negation-head`, `open-nested-fact-head`, `open-premise-value`, `open-signature-type`; and `open-derived-activator`, whose derived `(copy open)` makes `copy` read `(open ?x)`: **loads flagged** |
| **loads, by the default below** (5) | `open-eq-value` (a name `eq` compares), `open-fact-clause` (`:rule open`), `open-trace-step` (a trace step's rule), `open-macro-unused` and `open-macro-verdict` (a macro's body writes `(open ?R)`, checked where it is called) |
| **stays refused** (10) | `reserved_open_direct` (a macro named `open`), `reserved_open_symbols` (an import lists it), `open_in_match` and `open_mixed_assert` (no `:why` anywhere; the second also writes `(open ?R)` beside a fact, Q-M1f.575), `macro-eq-obligation` and `macro-not-conjunction-obligation` (the bare `(open)`), `open-fact-head` and `open-relation-parameter` (a written `(open x)`, `x` a name and no fact), `open-obligation-parameter` (`(owe open)` makes an obligation owe `open`), `open-solve-list` (a `(solve …)` list names a relation the program never writes) |

What each answers with `opn` written for `open` is in `runs/census.md`: 19 of the 20 answer on the stand-in, 2 to
32 states each, none flagged there (`open-derived-activator`'s flag is one the stand-in cannot show); the two whose
claim names `open` print a refuted claim, exit 1. `open-goal-pattern-head`'s stand-in is refused: its goal names
`opn`, a relation the program never writes, and that says nothing of `open`. Every one of the 20 is held refused by
a regression row today, so the build rewrites or removes those rows, and their corpus notes, with the decisions
Q-M1f.66 and Q-M1f.86.

## 8. What a suite reads that moves

| program | the move | what reads it today |
|---|---|---|
| `tests/stdlib/bijection/06_blind_enumeration.ein` | refused (bare `(open)`) | under `tests/`, its `test` run, a regression row, `obligation_reports.rs`, `obligation_rung.rs`, `region_modes.rs` |
| `examples/features/13_mixed_solution_and_open.ein` | an error (Q-M1f.578) | its `test` run; `read_out.rs`, `test_cli.rs` |
| `oblig-derived-domain-declared.ein`, `oblig-fresh-name-declared.ein`, `oblig-fresh-name-two-declared.ein` | **flagged** | each a `test` run and a `solve` run that checks its claim |
| `oblig-derived-domain.ein`, `oblig-fresh-name.ein`, `oblig-fresh-name-two.ein` | **flagged** | each a regression row and a `solve` run that checks its claim |
| `13_witness_of_two.ein`, `14_seats_nothing_proposes.ein` | its claim fails | a `solve` run that checks it; `test_cli.rs`, `unheld.rs`, `region.rs`, `read_out.rs` and `tree_classes.rs` name them |
| `zebra.ein`, `08_absent_witness_hole.ein`, `06_friends_tables_everyone_sits.ein` | another H, and R for two of them | named by many tests; whether a test pins what moves the build sees |
| the 20 witnesses of § 7 | load | a regression row each |

**No program under `tests/` becomes flagged**, and no program under `tests/` moves but `06_blind_enumeration.ein`.

## 9. What does not move

- **519 programs on the blind enumerator**, 404 of them with no candidate: no rule writes an `(open F)` and no
  obligation is activated, so it runs as today; the lists and closures configure it as today.
- **23 programs on the obligations rung**: H is the obligation's candidates today, and the union adds nothing; none
  steps aside, so neither its trigger nor its guard reads D, and no site is new.
- **2 stuck programs** (`kw-or-in-obligation.ein` and its twin): the obligation has no guard; it writes no
  candidate today or once built.
- **55 programs with `(solve :no-search)`**: H stays empty. 15 of them have a rule or an obligation that would write
  `open` facts at root, which root would then list; none disagrees with a list.
- **198 programs refused today for another reason** than `open`, stay refused.
- **The five programs whose hypothesis rule proposes nothing at root** (`ab.ein`, `zebra2-bad.ein` and three more):
  two decide every candidate at root (9 and 125 proposals, none a candidate) and so hold `open` facts, which keep the
  blind enumerator off; `readings.ein` activates an obligation; `hrule-eq.ein` and `query-full.ein` conclude nothing
  at root and activate no obligation, so the blind enumerator would run there, and proposes nothing in either
  (question Q-M1f.599 below).

## 10. Your call

**Is this the list to build to?**

- **(a)** Yes. The build moves exactly these programs, as shown, and rewrites what holds them: the claims of
  `13_witness_of_two` and `14_seats_nothing_proposes` restated to the union's R, the six newly flagged programs'
  `test` runs, regression rows and claim-checking runs taken out, `13_mixed_solution_and_open` and
  `06_blind_enumeration` rewritten (items [65](../65-the-lists-and-a-closure/README.md) and
  [63](../63-the-bare-open/README.md)), the 20 witnesses' rows rewritten. **Default.**
- **(b)** Revise a ruling first, saying which.

**Q-M1f.596, put here: where `open` may be written once it is the kernel's relation.** The rulings make `open` a
relation the kernel reads at the head of a written fact, a conclusion and a pattern, and an ordinary name under an
ordinary relation. They do not say what it is in four other places: a name `eq` compares, the `:rule` clause of a
fact, a trace step's rule, a macro's body that writes `(open ?R)` (Q-M1f.86 refuses it today); nor, refused under
both options, a macro named `open`, an import that lists it, a written `(open x)` with `x` no fact, an obligation
made to owe `open`, and a `(solve …)` list that names it.

- **(a)** Where a name may stand, `open` is a name; a macro's body may write `(open ?R)`, checked where it is called
  (Q-M1f.67). It stays refused where a program would declare it or make it more than the kernel's: a macro or an
  import's symbol named `open`, `(open x)` written with `x` no fact, an obligation that owes `open`, a list that
  names it. 20 witnesses load. **Default.**
- **(b)** Q-M1f.66 and Q-M1f.86 stand wherever the rulings do not reach: the four places stay refused, and their 5
  witnesses with them. 15 load.

**Q-M1f.599, put here: the blind enumerator beside a rule that concludes nothing at root.** Ruled: it writes only
where no rule concludes `(open F)` at root and no obligation is activated. Today a declared `(hrule …)` keeps it off
whether or not the rule proposes; once the form is a rule, *declared* has no meaning.

- **(a)** As the ruling's words: where root holds no `(open F)` and no obligation is activated, it runs, whatever
  rules the program has. **Default.** No corpus program moves: in the two where this happens it proposes nothing.
- **(b)** A program with a rule that can conclude `(open F)` keeps it off, as a declared hypothesis rule does today.

## On the built engine

The second part of the phase is built ([S1f.21.12](../../../s1f.21.12_hypotheses_as_facts.md),
[S1f.21.18](../../../s1f.21.18_the_hrule_form_removed.md), [S1f.21.13](../../../s1f.21.13_the_search_and_rules.md),
[S1f.21.20](../../../s1f.21.20_what_the_first_uses_of_open_found.md) and the pages), so the list is taken again from
the binaries, with no stand-in: every corpus program as the engine before the second part answers it and as the final
engine answers it, each on the corpus of its own commit. The figures in §§ 1 to 9 above are left as they were: they
are the prediction, taken on the stand-in engine at `edbe6c3b`.

### How it was taken

- **Before**: the release build of `ddf09492`, the first part's last commit (`build/costs/bin/ein-ddf09492`), on the
  corpus and the standard library as they were at `ddf09492`: 933 programs. **After**: the final binary
  (`build/review-bin/ein-final-7b72bd84`) on the corpus at `7b72bd84` (the commits after it change plans only): 931
  programs. The two `--version`s are in `runs/built/version.txt`.
- For each program, on each engine: `ein show hyps` (the set H, and the writer of H that stderr's note names),
  `ein solve --monotone` (the static check), and a describing `ein solve FILE -T 120 --json-summary` (R's states and
  commitments, whether R was exhausted, each claim's verdict, the flag), with the region budget at its default. No
  run reached the 120-second wall budget on either side.
- The programs are paired by path, and the two the build moved to another path by its record. **A program moved**
  where it loads on one side and is refused on the other, or is refused on both with another line, or its H (as a
  set of facts), its flag, R's counts or a claim's verdict differ. A refusal whose line changed only in its position
  or in the word *hrule* is counted as reworded, not as a move.
- `tools/run-built.sh` re-takes every run, with `tools/built_pass.py` (the runs) and `tools/built_compare.py` (the
  comparison), into `runs/built/`: `old.jsonl` and `new.jsonl` (one record a program), each run's output under
  `runs/built/old/` and `runs/built/new/`, `compare.jsonl` (one comparison a program), `corpus-log.txt` (which commit
  changed which corpus file), and `runs/built/census.md`, which the tool writes and which holds every number below.
- *Before* is not the page's *today*: the page ran the stand-in engine at `edbe6c3b` on the corpus at `669d7d43`, and
  the first part's last builds came between. On every program the page names in §§ 2 to 5 and 7 the two give the
  same figures (`runs/built/census.md`, *The page's prediction beside the built engine*, which shows both).

### The two corpora

| | before | after |
|---|---:|---:|
| programs (corpus entries) | 933 | 931 |
| refused, at the parse or at load | 214 | 187 |
| load | 719 | 744 |
| flagged (not monotone) | 51 | 59 |
| flagged, under `tests/` | 0 | 0 |
| R not exhausted (the region budget, or a flagged program's cube past its limit): the same 15 programs | 15 | 15 |
| a claim the describing run does not hold (failed, or not checked) | 5 | 13 |

**925 programs are on both sides: 46 moved, 879 did not.** Of the 879, **124 have a text the build rewrote and
answer as before**: 116 by S1f.21.18 (each hypothesis rule written as a rule that concludes `(open F)`, its
activators as facts), 6 by S1f.21.12 (three witnesses refused on both sides, `stdlib/debts.ein`,
`stdlib/macro.ein`, and `06_blind_enumeration.ein`, below), 2 by S1f.21.22 (the
comments of `stdlib/closure.ein` and `stdlib/bool/inputs.ein`); 755 have the same text. On 29 of the 879 only the
name of H's writer moved: `hypothesis-rules` → `union` on 25 (an obligation is activated beside the rule and proposes
nothing the rule does not), → `blind` on 2 (`hrule-eq.ein` and `query-full.ein`, H empty on both sides), →
`obligations` on 1 (`readings.ein`, H empty), and `declined` → `blind` on 1 (`06_blind_enumeration.ein`). The engine
before printed `hrules` where the final one prints `hypothesis-rules`, the name S1f.21.18 gave it (Q-M1f.626); the
census reads the two as one writer. 16 programs are refused on both sides with the line reworded (`runs/built/census.md`,
*The refusals reworded and not moved*). Apart from the 925: 2 moved to another path, 6 removed, 4 added.

**The flagged programs below** load and warn: the read-out's first row is `WARNING  THIS PROGRAM IS NOT MONOTONE:
NOTHING IS CLAIMED OF THIS ANSWER.`, and stderr's first line `WARNING: FILE: THIS PROGRAM IS NOT MONOTONE: NOTHING IS
CLAIMED OF THIS ANSWER (Q-M1f.564)` (each in `runs/built/new/<program>.solve.txt`). Nothing is claimed of a flagged
program's answer: its figures are what it prints, and none is called right or wrong.

### The 46 that moved

*k_s / k_c* are R's states and commitments; *before → after* throughout.

**By the union** (Q-M1f.570, built by S1f.21.12): an obligation's candidates join the rule's.

| program | H | k_s / k_c | claim | flag | what the build wrote in the file |
|---|---|---|---|---|---|
| [`zebra.ein`](../../../../../../examples/zebra.ein) | 56 → 112 | 1 / 261 982 → 1 / 68 719 459 314; the one state the same | none | monotone | S1f.21.18's spelling |
| [`13_witness_of_two.ein`](../../../../../../examples/lattice/13_witness_of_two.ein) | 3 → 4 | 5 / 5 → 8 / 13 | held → held | monotone | its claim restated to the new R (S1f.21.12); S1f.21.18's spelling |
| [`14_seats_nothing_proposes.ein`](../../../../../../examples/lattice/14_seats_nothing_proposes.ein) | 12 → 16 | 4 / 4 → 8 / 8 | held → held | monotone | the same |
| [`08_absent_witness_hole.ein`](../../../../../../examples/lattice/08_absent_witness_hole.ein) | 3 → 4 | 5 / 5 → 11 / 13 | none | **flagged** on both sides | S1f.21.18's spelling |
| [`06_friends_tables_everyone_sits.ein`](../../../../../../examples/lattice/06_friends_tables_everyone_sits.ein) | 19 → 25 | 8 / 8 → 8 / 8; the states the same | none | monotone | its header restated for the union (S1f.21.12); S1f.21.18's spelling |
| [`13_mixed_solution_and_open.ein`](../../../../../../examples/features/13_mixed_solution_and_open.ein) | 2 → 3 | 1 / 1 → 3 / 4 | held → held | monotone | its `:no-hypothesis (is-a likes)` dropped and its claim restated (S1f.21.12, item [65](../65-the-lists-and-a-closure/README.md)); S1f.21.18's spelling |

**By an obligation that no longer steps aside** (Q-M1f.579, built by S1f.21.12): each obligation writes its guard's
candidates at root, and the site `obligation-guard` flags the program. All eight are under
`examples/ein-bugs/wrong-answers/`.

| program | H | k_s / k_c | claim | flag | its corpus runs | regression row |
|---|---|---|---|---|---|---|
| [`oblig-derived-domain.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-derived-domain.ein) | 6 → 1 | 1 / 1 → 0 / 0 | held → failed | monotone → **flagged** | `solve` → `solve --count` | dropped |
| [`oblig-derived-domain-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-derived-domain-declared.ein) | 6 → 2 | 1 / 1 → 1 / 1; the state the same | held → held | monotone → **flagged** | `solve`, `test` → `solve --count` | none |
| [`oblig-fresh-name.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name.ein) | 6 → 1 | 1 / 1 → 0 / 0 | held → failed | monotone → **flagged** | `solve` → `solve --count` | dropped |
| [`oblig-fresh-name-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name-declared.ein) | 6 → 2 | 1 / 1 → 1 / 1; the state the same | held → held | monotone → **flagged** | `solve`, `test` → `solve --count` | none |
| [`oblig-fresh-name-two.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name-two.ein) | 12 → 2 | 12 / 12 → 0 / 0 | held → failed | monotone → **flagged**; in flight `(likes Ann Z)` | `solve` → `solve --count` | dropped |
| [`oblig-fresh-name-two-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name-two-declared.ein) | 12 → 4 | 12 / 12 → 3 / 3 | held → failed | monotone → **flagged** | `solve`, `test` → `solve --count` | none |
| [`oblig-stdlib-total-owed.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed.ein) | 6 → 1 | 16 / 16 → 0 / 0 | held → failed | **flagged** on both | `solve --count` on both | none |
| [`oblig-stdlib-total-owed-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed-declared.ein) | 6 → 2 | 16 / 16 → 1 / 1 | held → failed | **flagged** on both | `solve --count` on both | none |

H shrinks in every row: the candidates the blind enumerator guessed once the obligation stepped aside give way to
the ones the guard reads at root. Each claim is still written in its file; a describing run checks it for whoever
asks, and no suite reads it (S1f.21.12's corpus commit made each `test` run and each claim-checking `solve` run a
`solve --count`, and dropped the three rows).

**`open` written elsewhere, refused before, loading now** (21, by S1f.21.12's grammar and loader): the twenty
witnesses of § 7 that the page said would load, and one it did not list.

| program (under `examples/ein-bugs/wrong-answers/`) | the page: once built | the page's stand-in (`opn`) | after: H, its writer | k_s / k_c | claim | flag |
|---|---|---|---|---|---|---|
| `open-claim-fact-head.ein` | loads, by the rulings | 4 / 4, exit 1 | 2, `blind` | 4 / 4 | failed | monotone |
| `open-claim-value.ein` | loads, by the rulings | 4 / 4, exit 1 | 2, `blind` | 4 / 4 | failed | monotone |
| `open-conclusion-value.ein` | loads, by the rulings | 16 / 16 | 2, `blind` | 4 / 4 | none | monotone |
| `open-derived-activator.ein` | loads, by the rulings: flagged | 4 / 4 | 2, `blind` | 4 / 4 | none | **flagged** (`open-read`) |
| `open-fact-value.ein` | loads, by the rulings | 16 / 16 | 2, `blind` | 4 / 4 | none | monotone |
| `open-goal-pattern-head.ein` | loads, by the rulings | refused (its goal names `opn`) | 2, `blind` | 4 / 4 | none | monotone |
| `open-goal-value.ein` | loads, by the rulings | 4 / 4 | 2, `blind` | 4 / 4 | none | monotone |
| `open-hrule-value.ein` | loads, by the rulings | 2 / 2 | 1, `hypothesis-rules` | 2 / 2 | none | monotone |
| `open-is-a-object.ein` | loads, by the rulings | 32 / 32 | 2, `blind` | 4 / 4 | none | monotone |
| `open-macro-argument.ein` | loads, by the rulings | 22 / 32 | 2, `blind` | 4 / 4 | none | monotone |
| `open-macro-value.ein` | loads, by the rulings | 22 / 32 | 2, `blind` | 4 / 4 | none | monotone |
| `open-negation-head.ein` | loads, by the rulings | 4 / 4 | 2, `blind` | 4 / 4 | none | monotone |
| `open-nested-fact-head.ein` | loads, by the rulings | 32 / 32 | 5, `blind` | 32 / 32 | none | monotone |
| `open-premise-value.ein` | loads, by the rulings | 22 / 32 | 2, `blind` | 4 / 4 | none | monotone |
| `open-signature-type.ein` | loads, by the rulings | 8 / 8 | 3, `blind` | 8 / 8 | none | monotone |
| `open-eq-value.ein` | loads, by the default (Q-M1f.596) | 16 / 16 | 2, `blind` | 4 / 4 | none | monotone |
| `open-fact-clause.ein` | loads, by the default | 2 / 2 | 1, `blind` | 2 / 2 | none | monotone |
| `open-trace-step.ein` | loads, by the default | 4 / 4 | 2, `blind` | 4 / 4 | none | monotone |
| `open-macro-unused.ein` | loads, by the default | 4 / 4 | 2, `blind` | 4 / 4 | none | monotone |
| `open-macro-verdict.ein` | loads, by the default | 4 / 4 | 0, `stuck` | 0 / 0: every alive node owes | none | monotone |
| [`absent-fact-nested.ein`](../../../../../../examples/ein-bugs/wrong-answers/absent-fact-nested.ein) | not listed | — | 0, `blind` | 1 / 1 | none | monotone |

Eleven answer as the stand-in did. Seven answer 4 states where the stand-in answered 16, 22 or 32: `open` is no atom
the blind enumerator guesses about (Q-M1f.617, the twelfth default of item [69](../69-S1f.21.12-defaults/README.md)),
where `opn` was one, so H is `(p k)` and `(q k)` alone
(`runs/built/new/examples_ein_bugs_wrong_answers_open_is_a_object.show-hyps.txt` and the six others' files of that
form). `open-macro-verdict.ein`'s macro call expands to `(open likes)`, which is now what a state owes (Q-M1f.566):
root owes an instance no writer gives a candidate for, and R is empty; the stand-in's `opn` was an ordinary fact and
could not show that. `open-goal-pattern-head.ein` was refused on the stand-in for its goal's `opn`, which says
nothing of `open`, as the page said. The two whose claim names `open` refute it, as on the stand-in: no state holds
the fact each lists, `(p open)` or `(p (open k))`. Nineteen keep a regression row, which S1f.21.12 rewrote to
bank the region; `open-derived-activator.ein` lost its row, being flagged. `absent-fact-nested.ein` loads because under an ordinary relation `absent` is a name
(Q-M1f.571); S1f.21.12's record noted it (V-46).

**Refused on both sides, with another line** (11):

| program | before | after | why |
|---|---|---|---|
| `hrule-and.ein`, `hrule-and-in-and.ein` | an hrule concludes 2 or 3 facts, at load | `unexpected and`, at the parse | S1f.21.18 writes the conclusion `(open (and …))`, which the grammar refuses (Q-M1f.622) |
| `not-name-hrule-concluded.ein` | `(not ?g)` of a name, at load | `unexpected ?g: a variable`, at the parse | `(open (not ?g))` (Q-M1f.622; S1f.21.18's V-56) |
| `macro-eq-candidate.ein` | `(eq ?x)` takes 2 arguments | `(eq ?x)` written where a value goes | `(open (eq ?x))` (Q-M1f.622) |
| `eq-arity-hrule.ein` | `(solve :hrules (h (eq)))` binds `?R` to `eq` | the fact `(h eq)` binds `?R` to `eq` | the same refusal; the activator is now a fact (Q-M1f.621, Q-M1f.623) |
| `open-fact-head.ein`, `open-relation-parameter.ein` | `unexpected open`, at the parse | `(open x) is written, and x is neither a fact nor a relation`, at load | S1f.21.12's loader: refused, as the page said, by the line the build gave it |
| `open-obligation-parameter.ein` | `unexpected open` | `(owe open) makes obligation 'owe' owe open` | the same |
| `open-solve-list.ein` | `unexpected open` | `its :hypothesis-relations names open, and the program names no relation open` | the same |
| [`reserved_open_symbols.ein`](../../../../../../examples/broken/load/reserved_open_symbols.ein) | its own `:symbols (open)`, at the parse | its module's line, `reserved_open_direct.ein:16:8: unexpected open` | a list of names takes `open` now; the module it lists is refused, as the comment S1f.21.12 wrote says |
| [`closed-and-owing.ein`](../../../../../../examples/ein-bugs/closed-and-owing.ein) | an obligated relation is closed | root holds `(open r)` and `(__closed__ r)` says `r` is closed | Q-M1f.578's error, as the page said (§ 4) |

The first five are under `examples/ein-bugs/wrong-answers/` and moved with the `hrule` form (S1f.21.18); the four
`open-*` there with S1f.21.12's loader. Every refusal line, before and after, is in `runs/built/census.md`.

**Moved to another path** (2), each by Q-M1f.614 (S1f.21.12):

| before | after | before → after |
|---|---|---|
| `examples/broken/open_in_match.ein` | [`examples/features/23_a_rule_reads_open.ein`](../../../../../../examples/features/23_a_rule_reads_open.ein) | refused at the parse → loads, **flagged** (`open-read`): H 1, R 1 / 1 |
| `examples/broken/open_mixed_assert.ein` | [`examples/broken/load/open_mixed_assert.ein`](../../../../../../examples/broken/load/open_mixed_assert.ein) | refused at the parse → refused at load: `(open ?R)` stands beside another conclusion (Q-M1f.575) |

**Removed** (6, each refused before; S1f.21.18, Q-M1f.623 and Q-M1f.624): `examples/broken/hrule_reserved_name.ein`,
`examples/broken/load/ein_on_hrule.ein`, and under `examples/ein-bugs/wrong-answers/` `macro-bare-conclusion-hrule.ein`,
`solve-hrules-activator-arity.ein`, `solve-hrules-no-such-hrule.ein` and `solve-hrules-no-such-relation.ein`.

**Added** (4): [`examples/broken/load/no_hypothesis_beside_open.ein`](../../../../../../examples/broken/load/no_hypothesis_beside_open.ein)
(S1f.21.12: `13_mixed_solution_and_open.ein` as it was, refused at load: *its :no-hypothesis names likes, and root
holds (open likes)*); [`examples/features/24_written_hypotheses.ein`](../../../../../../examples/features/24_written_hypotheses.ein)
(S1f.21.20: H 2, R 3 / 3, its two claims held, monotone); [`examples/lattice/18_two_candidates.ein`](../../../../../../examples/lattice/18_two_candidates.ein)
(S1f.21.13: H 2, R 2 / 2, two claims held) and [`18_two_candidates_as_rules.ein`](../../../../../../examples/lattice/18_two_candidates_as_rules.ein)
(S1f.21.13: H 0, R 1 / 1, four claims held).

### The prediction beside what happened

Item by item of *The answer, short* (at the top of the page) and § 6:

| the page said | on the built engine | |
|---|---|---|
| **15 that load today would answer otherwise** | 14 that load on both sides moved: the union's 5, `13_mixed_solution_and_open.ein`, and the obligations' 8. The fifteenth, `06_blind_enumeration.ein`, answers as before | **different, by two rewrites** the page's default (a) of § 10 named: see the two rows below |
| **20 refused today would load** | the 20 load, and one more: `absent-fact-nested.ein` | **confirmed**; one not predicted |
| **5 by the union**, with H, k_s and k_c for each | the 5, every H, k_s and k_c as the page gave them (`zebra.ein` 56 → 112 candidates and 261 982 → 68 719 459 314 commitments, its one state the same) | **confirmed** |
| the claims of `13_witness_of_two` and `14_seats_nothing_proposes` fail | both hold: S1f.21.12 restated each claim to the new R | **not seen**, by the rewrite (a) said |
| **8 by an obligation that no longer steps aside**, each as item [64](../64-an-obligation-that-no-longer-steps-aside/README.md)'s stand-in | the 8, every H, k_s and claim as the stand-in gave them | **confirmed** |
| **6 become flagged**, each read by a suite today (3 by a `test` run, 3 by a regression row) | 6 newly flagged, by `obligation-guard`; each read by no suite now: runs `solve --count`, the 3 rows dropped | **confirmed** |
| **1 becomes an error under the lists**: `13_mixed_solution_and_open.ein` | the program as it was, now `no_hypothesis_beside_open.ein`, is refused at load with Q-M1f.578's error; the file at the old path was rewritten without its `:no-hypothesis` and moves by the union (H 2 → 3, R 1 / 1 → 3 / 4, its claim restated and held) | **confirmed on the moved copy**; the path moves otherwise, by a rewrite |
| **1 is refused for its bare `(open)`**: `06_blind_enumeration.ein` | it was rewritten with no obligation activated (Q-M1f.600's default, item [63](../63-the-bare-open/README.md)): it loads, H 16 by the blind enumerator, R 1 / 1, its claim held, as before | **not seen**, by the rewrite |
| **20 regression witnesses of `open` load** (15 by the rulings, 5 by the default); **10 stay refused** | 15 and 5 load; of the 10, 9 stay refused (six with another line, above) and `open_in_match.ein` loads, flagged, as `23_a_rule_reads_open.ein` | **confirmed** for the 20; **one of the 10 loads**, by its move (Q-M1f.614) |
| on the stand-in, 19 of the 20 answer, 2 to 32 states each, none flagged; `open-derived-activator` flagged once built; the two whose claim names `open` refute it | 20 of 20 answer, 0 to 32 states; one flagged, `open-derived-activator.ein`; the two refute their claim | **confirmed**, with 9 answers other than the stand-in's: 7 by Q-M1f.617, `open-macro-verdict.ein` by `(open likes)` being an obligation, and `open-goal-pattern-head.ein`, refused on the stand-in |
| **none by a hypothesis first held above root** (Q-M1f.574): the site's 3 programs are flagged today | the site `hypothesis` on those 3, each flagged before; the check in flight, which the page did not take, finds one on 2 describing runs (`oblig-fresh-name-two.ein`, `(likes Ann Z)`; [`maximal_not_top.ein`](../../../../../../examples/lattice/synth/maximal_not_top.ein), `(h Z)`), both flagged by a site already | **confirmed**: no program newly flagged by it |
| **none by a rule that reads `open`** (Q-M1f.569); once built `open-derived-activator.ein` | the site `open-read` on 2: `open-derived-activator.ein` and `23_a_rule_reads_open.ein` | **confirmed**, and one more by the move |
| Q-M1f.579's site: the eight of § 3 and the same three | `obligation-guard` on those 11 | **confirmed** |
| **no program under `tests/` becomes flagged** | 0 before, 0 after | **confirmed** |
| Q-M1f.599: the two programs where the blind enumerator would run beside a rule propose nothing | `hrule-eq.ein` and `query-full.ein`: writer `blind`, H empty on both sides | **confirmed** |

And § 8, *what a suite reads that moves*:

| program | the page: the move | after | what reads it after |
|---|---|---|---|
| `tests/stdlib/bijection/06_blind_enumeration.ein` | refused | no move (rewritten) | as before: under `tests/`, its `test` run and `solve` run, a regression row |
| `13_mixed_solution_and_open.ein` | an error | the union's move (rewritten) | its `test` run and its `solve` run, which hold the restated claim |
| the three `-declared` obligation programs | flagged | flagged | no suite: each `solve --count` |
| `oblig-derived-domain.ein`, `oblig-fresh-name.ein`, `oblig-fresh-name-two.ein` | flagged | flagged | no suite: rows dropped, each `solve --count` |
| `13_witness_of_two.ein`, `14_seats_nothing_proposes.ein` | its claim fails | R moves, its claim restated and held | a `solve` run that checks it |
| `zebra.ein`, `08_absent_witness_hole.ein`, `06_friends_tables_everyone_sits.ein` | another H, R for two | another H, R for two (`zebra.ein` by k_c, `08_absent_witness_hole.ein` by both; `06_…`'s R the same) | no `test` run or row; Rust tests name the files |
| the 20 witnesses | load | load | 19 regression rows; `open-derived-activator.ein`'s dropped |

**No program under `tests/` moved at all**: the page expected one, `06_blind_enumeration.ein`, which the build
rewrote to keep its answer.

### Moves the page did not predict

- `absent-fact-nested.ein` loads (Q-M1f.571; S1f.21.12's record has it).
- `open_in_match.ein` moved to `23_a_rule_reads_open.ein`, which loads, flagged (Q-M1f.614, the ninth default of
  item [69](../69-S1f.21.12-defaults/README.md)).
- Five refusals moved with the `hrule` form, which the page did not cover: four conclusions `(open …)` now wraps,
  three refused at the parse and `macro-eq-candidate.ein` at load, and `eq-arity-hrule.ein`'s activator, a fact
  (item [71](../71-S1f.21.18-defaults/README.md)); and the six programs S1f.21.18 removed with the form.
- The four programs added by S1f.21.12, S1f.21.13 and S1f.21.20.
- On 29 programs the writer's name moved and nothing else did: S1f.21.18's name for the hypothesis rules, and the
  union's word where an obligation is activated beside them.

Every difference between the prediction and the binaries is a ruling's, a default's, or a stage's rewrite of the
corpus; the census found no move that none of these explains.
