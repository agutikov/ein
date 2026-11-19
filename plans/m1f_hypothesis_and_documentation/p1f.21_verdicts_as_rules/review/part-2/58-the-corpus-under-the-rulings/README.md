# 58 · The corpus under the rulings: every program that would move

[← part 2](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.12_hypotheses_as_facts.md) ·
[your answers](../ANSWERS.md)

**What this is.** The list task 1.1 of [S1f.21.12](../../../s1f.21.12_hypotheses_as_facts.md) asks for before
anything is built: every corpus program whose candidates, answer, claim or flag would change under your rulings
Q-M1f.566 to Q-M1f.579, with what it is today and what it would be.

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
