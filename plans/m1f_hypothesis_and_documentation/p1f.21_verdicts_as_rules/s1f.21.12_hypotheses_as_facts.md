# S1f.21.12 — `open` facts: the hypotheses held, and the obligation's verdict a fact

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*) and
**planned again 2026-11-20 on the user's rulings** of [Q-M1f.566](../open_questions.md#q-m1f566--how-a-requirement-is-emitted-the-must-beside-open) to [Q-M1f.579](../open_questions.md#q-m1f579--an-obligation-whose-guard-or-trigger-reads-what-a-commitment-can-change)
([the answers to items 44 to 46](README.md#the-answers-to-items-44-45-and-46),
[to items 47 to 51](README.md#the-answers-to-items-47-to-51),
[to items 52 to 55](README.md#the-answers-to-items-52-to-55),
[to items 56 and 57](README.md#the-answers-to-items-56-and-57)). **Its task 1, the examples, started 2026-11-20** on the user's word
(*"Start stages, if anything can be done in parallel - do it."*), beside part 1's two builds: it changed no code,
and is done ([the record](#record)). **Its build is unblocked by the user, 2026-11-20**: *"pick defaults, implement
the phase to the end but not close … I will review questions after."* It is built on the defaults of the examples'
six questions, Q-M1f.595 to Q-M1f.600, and starts when part 1's two builds are merged.
**Waits on:** part 1. No question of it is open.
**After it:** [S1f.21.18](s1f.21.18_the_hrule_form_removed.md), which removes the `hrule` form and the keyword
`:hrules`; [S1f.21.14](s1f.21.14_the_pages_of_open.md), the pages.
**Writes:** the grammar (`ein-ir/src/ein.y`) and the loader; `ein-core/src/terms.rs`; `ein-infer/src/`
(`obligations.rs`, `oblgen.rs`, `hypgen.rs`, `hrule.rs`, `region.rs`, `mechanisms.rs`, `monotone.rs`,
`detector.rs`); `ein-render` (the read-out, the dumps, the drawings); `ein show hyps`; the goldens.

## What is ruled, and what it makes this stage build

| ruled | built here |
|---|---|
| **The obligation stays as it is** ([Q-M1f.566](../open_questions.md#q-m1f566--how-a-requirement-is-emitted-the-must-beside-open)): a rule with its trigger and an `absent` over its witness, concluding `(open R)`; the kernel counts its instances as before, and reads the candidates off its `absent` as before | nothing of the count or of the candidates' reading changes |
| **`(open R)` is held as a fact** ([Q-M1f.566](../open_questions.md#q-m1f566--how-a-requirement-is-emitted-the-must-beside-open)), once, where a state owes an instance of it | the pass that counts writes it |
| **`(open F)`, F a fact, is a hypothesis** ([Q-M1f.566](../open_questions.md#q-m1f566--how-a-requirement-is-emitted-the-must-beside-open), [Q-M1f.573](../open_questions.md#q-m1f573--open-as-an-obligation-or-as-a-hypothesis-and-which-hypotheses-are-candidates)). Any rule may conclude one; a program may write one; the obligations' candidates and the blind enumerator's guesses are written as such | `open` becomes a relation the kernel reads at the top of a fact |
| **The bare `(open)` is refused at load** ([Q-M1f.566](../open_questions.md#q-m1f566--how-a-requirement-is-emitted-the-must-beside-open)) | a refusal, a row of the grammar page's table; one corpus program rewritten (task 1.6) |
| **A fork drops every `open` fact**, a written one among them, and they are made again at the fixpoint; every rule that concludes `(open …)` is read at every state; no rule deletes a fact ([Q-M1f.566](../open_questions.md#q-m1f566--how-a-requirement-is-emitted-the-must-beside-open), [Q-M1f.569](../open_questions.md#q-m1f569--what-may-read-an-open-fact-and-when)) | the `open` facts live where no fork inherits them, and one pass at each state's fixpoint writes them |
| **$H$ is root's `open` facts**, less those root holds or denies; one first held above root is held and not proposed ([Q-M1f.569](../open_questions.md#q-m1f569--what-may-read-an-open-fact-and-when), [Q-M1f.573](../open_questions.md#q-m1f573--open-as-an-obligation-or-as-a-hypothesis-and-which-hypotheses-are-candidates)) | $H$ read once, at root, off the facts |
| **The union** ([Q-M1f.570](../open_questions.md#q-m1f570--a-rule-that-concludes-open-f-beside-an-obligation-override-or-union)): $H$ is every `(open F)` root holds, a rule's and an obligation's together, identical ones one fact. **The blind enumerator** writes only where no rule concludes `(open F)` at root and no obligation is activated ([Q-M1f.576](../open_questions.md#q-m1f576--the-unions-edges-an-obligation-whose-candidates-cannot-be-read-and-the-blind-enumerator)) | the ladder's override goes |
| **The blind enumerator's configuration** ([Q-M1f.578](../open_questions.md#q-m1f578--the-two-lists-of-solve-and-a-closure-once-the-hypotheses-are-open-facts)): `:hypothesis-relations`, `:no-hypothesis` and `(__closed__ R)` are its white list and black list. It runs only where root's fixpoint holds no `open` fact, and never beside hypotheses a rule emits. Beside `open` facts, one of the three that disagrees with them is an error: a hypothesis of a closed relation, a `:no-hypothesis` that names an `open` fact's relation, a `:hypothesis-relations` that lists a relation no `open` fact is of | the conflicts of `mechanisms.rs`, said again over `open` facts, each a load error with its row of the diagnostics page; the notes for a list that filters nothing go where the list is now an error or agrees |
| **An obligation whose guard or trigger reads what a commitment can change never steps aside** ([Q-M1f.579](../open_questions.md#q-m1f579--an-obligation-whose-guard-or-trigger-reads-what-a-commitment-can-change)): it writes its guard's candidates at root, and the program is flagged | the rung's decline for that reason goes; a site of the static check for such a guard; the eight programs that step aside today are flagged, and the claims six of them state are no longer held by a suite (Q-M1f.564) |
| **One `:assert`** ([Q-M1f.575](../open_questions.md#q-m1f575--what-one-assert-holds-beside-open-r-and-beside-open-f)): `(open R)` is a rule's whole `:assert`, as today; `(open F)` stands in a conjunction like any fact | the loader's checks on a conclusion |
| **A hypothesis first held above root makes its program not monotone, found two ways** ([Q-M1f.574](../open_questions.md#q-m1f574--a-hypothesis-first-held-above-root-how-the-program-is-found-not-monotone)): a site of the static check for a rule that concludes `(open F)` and reads what a commitment can change; and in flight, a state the run builds that holds an `(open F)` root does not. Found in flight, the answer stands, nothing is claimed, and the warning is *shown immediately when detected, once* | the site; the check at each state built; the warning's line on stderr at once, and the read-out's row |
| **A view without the `open` facts** ([Q-M1f.577](../open_questions.md#q-m1f577--a-view-of-a-state-without-its-open-facts)): `-P` leaves them out as it leaves out the `(not …)` | `-P` and its form of `--dump-answer` |
| **A rule may read an `open` fact, and its program is then not monotone automatically** ([Q-M1f.569](../open_questions.md#q-m1f569--what-may-read-an-open-fact-and-when)): warned, nothing claimed, no test | a site of the static check for a `:match` that reads `open`; the pass's facts are there before such a rule fires |
| **What nests in what** ([Q-M1f.568](../open_questions.md#q-m1f568--absent-open-and-not-nested-the-cells-that-are-the-users), [Q-M1f.571](../open_questions.md#q-m1f571--what-absent-and-open-take-and-what-each-is-under-an-ordinary-relation)): the kernel reads `absent` at the head of a premise and `open` at the head of a written fact or of a conclusion; directly under one the other is refused, but `(absent (open P))`; `absent` takes a premise, `open` a relation's name or a fact; under an ordinary relation both are ordinary names | the grammar and the loader; three forms refused today load, and one that loads today is refused |
| **Under `open`** ([Q-M1f.573](../open_questions.md#q-m1f573--open-as-an-obligation-or-as-a-hypothesis-and-which-hypotheses-are-candidates)): a relation's name, or a parameter that names one, is the obligation; a fact, or a variable the `:match` binds, a hypothesis | read at load |
| **The `open` facts are part of a state** ([Q-M1f.572](../open_questions.md#q-m1f572--whether-a-states-open-facts-are-shown-and-counted-with-its-facts)): listed, drawn and counted with its facts | every listing of a state, the counts, the goldens |
| **The `hrule` form and the keyword `:hrules` go** ([Q-M1f.567](../open_questions.md#q-m1f567--the-hrule-form-dropped-for-a-rule-that-concludes-open-f)); an activator is a fact the program writes | here the form is read as *a rule that concludes `(open F)`, activated by the clause's activators*, so the corpus runs unchanged; [S1f.21.18](s1f.21.18_the_hrule_form_removed.md) rewrites every program and deletes the form |

## Task 1 — the examples (stops for the user)

The phase's rule: a new syntax is shown on real corpus programs, their answers computed, before it is built. With
today's binary and a stand-in relation where the parse refuses `open`, as the review items did.

1. **The union over the corpus.** Every corpus program with a hypothesis rule and an activated obligation (19 by
   the census of [`one_way_each/`](one_way_each/), to be taken again): $H$ and $R$ today, and under the union, by
   the stand-in of [the page of Q-M1f.570](q-m1f.570_override_or_union/README.md). The programs whose answer
   moves are listed with the claim each states today and the claim it would state (8 expected, `zebra.ein` among
   them by its commitments). **Nothing is built until the user has read the list.**
2. **Three programs in the spelling ruled**, as files the parse refuses today: the seats program, *zebra2*, and
   one with a rule that concludes `(open F)` beside a fact ([Q-M1f.575](../open_questions.md#q-m1f575--what-one-assert-holds-beside-open-r-and-beside-open-f)).
3. **A state with its `open` facts**, as `-p`, a dump and a drawing would show it, on the stand-in: one program
   of each writer, a rule's, an obligation's, the blind enumerator's.
4. **A program whose rule reads an `open` fact**, flagged: the three of
   [the page of Q-M1f.569](q-m1f.569_what_may_read_an_open_fact/README.md), with the warning's rows as they would
   read.
5. **What a fork drops**, on the seats cube: each node's `open` facts as the pass would write them, `(open seats)`
   at the nodes that owe and at no state of $R$.
6. **The bare `(open)`**: `tests/stdlib/bijection/06_blind_enumeration.ein` uses it to make the obligations rung
   decline. Another way to the same claim is shown, or the claim it can no longer state is named.
7. **The eight programs whose obligations rung steps aside today** ([Q-M1f.579](../open_questions.md#q-m1f579--an-obligation-whose-guard-or-trigger-reads-what-a-commitment-can-change)), each as it answers today and as it
   would with its guard read at root: `oblig-derived-domain.ein` and its seven siblings, six of which state a claim
   that holds today.
8. **The lists and a closure** ([Q-M1f.578](../open_questions.md#q-m1f578--the-two-lists-of-solve-and-a-closure-once-the-hypotheses-are-open-facts)): every corpus program that writes one of the three beside a rule that
   proposes or an obligation, as it loads today and whether it is an error under the ruling; the eight programs of
   [the page of the question](q-m1f.578_the_lists_and_a_closure/README.md) and `examples/features/13_mixed_solution_and_open.ein`,
   the one corpus program with a hypothesis rule, an obligation and a list.

## Task 2 — the build

1. **The grammar and the loader.** `open` at the head of a written fact, of a conclusion's literal and of a
   pattern; the nesting rule; `(open)` refused; what stands under `open` read at load. A row of the grammar page's
   table for each shape newly refused, in the same commit.
2. **The pass.** At each state's fixpoint it writes `(open R)` for each relation an instance is owed of, and
   `(open F)` for each hypothesis a rule concludes, an obligation's `absent` gives or the blind enumerator guesses,
   into a part of the state no fork inherits. The count of instances is as today.
3. **$H$.** Read at root off those facts, less what root holds or denies; the union; the two lists and a closure
   as [Q-M1f.578](../open_questions.md#q-m1f578--the-two-lists-of-solve-and-a-closure-once-the-hypotheses-are-open-facts) rules: they configure the blind enumerator, and a disagreement with the `open` facts is a load error.
4. **A rule that reads `open`.** Its facts are there before it fires: the pass, then such rules, until nothing
   changes. Its program is flagged by the static check, with a site of its own kind. No test holds a flagged
   program's answer (Q-M1f.564).
5. **A hypothesis first held above root** ([Q-M1f.574](../open_questions.md#q-m1f574--a-hypothesis-first-held-above-root-how-the-program-is-found-not-monotone)): a site of the static check for a rule that concludes
   `(open F)` and reads a relation a commitment can change; and the run reports one it meets, the warning on
   stderr at once, and once.
6. **The surfaces.** The read-out's rows, `-p`, `-P` ([Q-M1f.577](../open_questions.md#q-m1f577--a-view-of-a-state-without-its-open-facts)), `-f`, the dumps, `ein saturate --dump`, the
   drawings, the events, the summary, `ein show hyps`; `08-read-out.md` in the same commit as any row that moves.
7. **The `hrule` form**, read as a rule that concludes `(open F)` with the clause's activators, with no change to
   what a program writes. It is deleted by the next stage.

## What must hold

- **$R$ of every corpus program as before**, but the programs task 1.1 lists, each with the claim shown there;
  every mode against the oracle (`ein-infer/tests/explore.rs`); `ein-cli/tests/lever_invariance.rs`;
  `ein-render/tests/id_order_invariance.rs`; `ein-cli/tests/import_invariance.rs`.
- **`ein show hyps` of every corpus program the same set**, but those programs.
- **The kernel's `owes` at every node as before** (`ein-infer/tests/obligation_reports.rs`, `debts_rules.rs`).
- **No program under `tests/` is flagged**, and no suite holds a flagged program's answer.
- The goldens that list a state move with the `open` facts and are banked again; the shape golden's lines too.
- The drawings are DOT writers': `utils/ci_container.sh`; and the nightly, a large change to the engine.

## Acceptance

The seats program holds `(open seats)` at each of its nodes that owe and at neither state of $R$, and its four
candidates as `(open F)` at root; *zebra2* answers as today; a program whose rule reads an `open` fact loads under
the warning; `(open)`, `(open (open P))`, `(open (absent P))` and `(absent (absent P))` are refused, each with its
row; one gate on the last commit, the nightly, and the container gate.

## After the build: one program shown to the user

`examples/ein-bugs/wrong-answers/oblig-derived-domain.ein` is run on the built engine and shown, with what it
answered before. The user, ruling [Q-M1f.579](../open_questions.md#q-m1f579--an-obligation-whose-guard-or-trigger-reads-what-a-commitment-can-change): *"implement it and double check
examples/ein-bugs/wrong-answers/oblig-derived-domain.ein, I expect it will still be solvable, despite
non-monotonicity. This is my expectation, not requirement. Let's review it after implementation."* It is no
condition of the build: whatever it answers is reported, and the review is the user's. On the stand-in of
[the page of the question](q-m1f.579_an_obligation_that_steps_aside/README.md) it answers an empty $R$.

## Record

**Task 1, the examples: done 2026-11-20**, in a subagent of its own beside part 1's two builds; it changed no code,
no stdlib, no corpus file and no page of `docs/`. Its items are 58 to 65 of [part 2 of the bundle](review/part-2/README.md),
each a page and its programs; every run is the gate build of the engine at `669d7d43`'s branch point
(`build/review-bin/ein-head-edbe6c3b`) with the checkout's `stdlib/`, on the corpus at `669d7d43`, which the two
builds beside it do not touch here. Each item's `tools/run.sh` re-takes its `runs/`; both are ignored by git and
mirrored under `build/review-incoming/p1f21-s12-examples/`. Each `run.sh` was run once more from clean on the last
commit, and every number on a page is in its `runs/`. Commits: `f529cde2` (the items, the index, the answers' file,
this record), and the one after it (item 58's wording).

| task 1 | item | what it found |
|---|---|---|
| 1, and the corpus lists of 6, 7 and 8 | [58](review/part-2/58-the-corpus-under-the-rulings/README.md) | taken again on today's corpus (931 entries): 15 programs that load today move (the union 5, of which `13_witness_of_two` and `14_seats_nothing_proposes` lose their claim; an obligation that no longer steps aside 8, 6 newly flagged, each read by a `test` run or a regression row; `13_mixed_solution_and_open` an error; `06_blind_enumeration` refused); no program under `tests/` becomes flagged; no new flag from Q-M1f.574's site (its three programs are flagged today); no rule reads an `open` fact or could; 20 of the 30 regression witnesses of Q-M1f.66 and Q-M1f.86 would load (15 by the rulings, 5 by Q-M1f.596's default) |
| 2 | [59](review/part-2/59-three-programs-as-ruled/README.md) | *zebra2* as ruled answers as today on the stand-in; the seats answer 16 candidates and 8 states by the union; a hypothesis beside a fact loads |
| 3 | [60](review/part-2/60-a-state-with-its-open-facts/README.md) | a state's `open` facts, one program a writer, root and one state above, `-p` composed, a dump and a drawing on the stand-in |
| 4 | [61](review/part-2/61-a-rule-that-reads-open/README.md) | the three programs of Q-M1f.569's page in the spelling ruled, refused today, each flagged once built; the warning's rows are today's, the site's row composed |
| 5 | [62](review/part-2/62-what-a-fork-drops/README.md) | on the seats cube, by two readings that agree at every node: `(open seats)` at the nine nodes that owe, at neither state of $R$ |
| 6 | [63](review/part-2/63-the-bare-open/README.md) | `06_blind_enumeration.ein`: its claim kept two ways; its Rust test's check on names kept by a program with no obligation activated; the check on activator facts can no longer be asked; once built no corpus program steps aside |
| 7 | [64](review/part-2/64-an-obligation-that-no-longer-steps-aside/README.md) | the eight on the stand-in (the guard read at root): four answer an empty $R$; `oblig-derived-domain.ein` answers an empty $R$, and `(likes Ann Pie)` if Pie is a food at root or a rule proposes it, flagged either way |
| 8 | [65](review/part-2/65-the-lists-and-a-closure/README.md) | of the ruling's page's eight, program 7 becomes an error; in the corpus `13_mixed_solution_and_open` alone |

**Questions put, each on its item with a default; ids reserved for the coordinator to file:**

| id | item | question | default |
|---|---|---|---|
| Q-M1f.595 | 65 | an `open` fact of a relation `:hypothesis-relations` leaves out: an error too? | (a) yes: a white list beside `open` facts names exactly their relations |
| Q-M1f.596 | 58 | where `open` may be written once it is the kernel's relation, in the places the rulings do not name | (a) a name where a name stands, a macro's body may write `(open ?R)`; refused where a program would declare it, write `(open x)` with `x` no fact, make an obligation owe it, or name it in a list |
| Q-M1f.597 | 65 | whether the lists and a closure are judged against the `open` facts root holds or what the program declares | (a) the `open` facts root holds, as the ruling's words |
| Q-M1f.598 | 62 | whether the pass writes at a dead state | (a) yes, as at any state, where it owes |
| Q-M1f.599 | 58 | the blind enumerator beside a rule that concludes nothing at root | (a) it runs where root holds no `open` fact and no obligation is activated |
| Q-M1f.600 | 63 | what becomes of `06_blind_enumeration.ein` | (a) rewritten with no obligation activated, the blind enumerator running |

Q-M1f.601 to Q-M1f.604 are not used.

**Findings outside the task, for the coordinator to file (V-40 to V-42; V-43 and V-44 not used):**

- **V-40.** `examples/broken/open_in_match.ein` and `examples/broken/open_mixed_assert.ein`, parse-negative
  witnesses of Q-M1f.66, write no `:why`. Once `open` parses where they write it, both are refused for the missing
  `:why` before reaching what their corpus notes say they witness; the first's subject, a rule that reads `open`, is
  legal by Q-M1f.569.
- **V-41.** `tests/saved/04_a_saved_node.ein` holds an `(hrule …)` inside a saved program's `(program …)` form.
  S1f.21.18's sweep, which writes every program's hypothesis rules again, has to reach a saved file's forms too.
- **V-42.** `examples/syntax/query-full.ein`'s goal names `drinks`, which only its two hypothesis rules write. Once
  they conclude `(open (drinks …))`, the loader's check that a goal names a relation of the program has to read
  under `open`, or the program is refused, as it is today with the hypothesis rules deleted.

**Task 2, the build: done 2026-11-20**, in a subagent of its own, on the branch of its worktree from `621c2ab1`,
then rebased onto `master`. Commits: `e1f37012` (the grammar and the loader), `e46c4c89` (the engine and its tests), `21865cbc` (the corpus), `f0bbd066` (the pages, the decisions, AGENTS.md), `04d780fb` (the goldens), `375e3d4f` (a node's `beyond` without its `open` facts, the rebase's anchor and the examples page); two helpers of the build's own rewrote the pages that told the ladder and restated the structure tests the union moved (`coupling`, `cover`, `exactly_one`, `theory_share`, `h_space_view`), each change in the commits above.

| step | what was built |
|---|---|
| 1, the grammar and the loader | `open` is a position word (`ein.y`'s `OpenFact`, `OpenHead`, `OrdinaryHead`, `OpenTerm`, `AbsentPremise`, `ClaimNested`): the kernel's word at the head of a written fact, a premise and a conclusion, a name elsewhere, bound by no declarator. `(open)`, `(open (open P))`, `(open (absent P))` and `(absent (absent P))` are parse errors, each a row of the grammar page's table (seven rows added, 48 rows and the five `open-*` deleted lines removed: their shapes load now). The loader's `validate_open` reads a name or a parameter under `open` as what a state owes, the rule's whole `:assert`, and anything else as a hypothesis; a written `(open F)` is held at root; a written `(open x)` with `x` no fact and no relation, an obligation made to owe `open`, and an hrule that concludes `(open …)` are refused at load |
| 2, the pass | `ein_infer::open`: at each state's fixpoint it writes `(open R)` once per relation the state owes and `(open F)` for each hypothesis its writers give there, into `Kb`'s `open` layer, which no fork inherits; the count of instances is as before (`obligations::tally`) |
| 3, H | read at root off the `open` facts, less what root holds or denies; the union of the rules that conclude a hypothesis (hrules among them), the program's written ones and the activated obligations' payments; the blind enumerator only where none writes; `mechanisms::judge` holds the two lists and a closure to root's `open` facts, a disagreement a load error (`no_hypothesis_beside_open.ein`); obligations never decline |
| 4, a rule that reads `open` | the pass, then such rules, until nothing changes; the static check's site `open-read` (also through a relation parameter an activator binds to `open`) |
| 5, a hypothesis first held above root | the site `hypothesis` (a rule that concludes `(open F)` and reads `D` outside an `absent` or under an even number), the site `obligation-guard` (Q-M1f.579), and `Monotonicity::in_flight`, read off the run: the read-out's `in flight` row, the summary's `monotonicity.in_flight`, stderr's warning once per file |
| 6, the surfaces | `-p` lists a state's `open` facts and `-P` leaves them out; the dumps, `ein saturate --dump`, the drawings, the events (`rung` mode `union`), the summary and `ein show hyps` carry them; `08-read-out.md` and `11-region-dump.md` in the same commit |
| 7, the hrule | read as a rule that concludes `(open F)` with its clause's activators (`hrule::Hrules` over `program.hrules` and `program.hypotheses`) |

**The corpus, against item 58.** Run on the built engine against `621c2ab1`'s (`build/s12/compare.py`):

- *By the union*, as item 58 says: `zebra.ein` (H 56 → 112, R one state as before), `06_friends_tables_everyone_sits`
  (19 → 25), `08_absent_witness_hole` (3 → 4), `13_witness_of_two` (3 → 4, R 8 states, its claim restated from the
  oracle), `14_seats_nothing_proposes` (12 → 16, R 8 states, its claim restated from the oracle; it now costs past a
  second and is flagged `slow`, its `cost_ms` the gate run's figure and not re-taken), `13_mixed_solution_and_open`
  (its `:no-hypothesis (is-a likes)` dropped, item 65: R three states, the claim restated; the program as it was is
  `examples/broken/load/no_hypothesis_beside_open.ein`, refused at load).
- *An obligation that no longer steps aside*, as item 64 says: the six `oblig-fresh-name*` and `oblig-derived-domain*`
  witnesses, newly flagged, and the two `oblig-stdlib-total-owed*`, flagged already; each lost the regression rows and
  claim-checking runs a flagged program may not have, and its header says what is true.
- *Where `open` is written*: the regressions' `open-*` witnesses (item 58's list): twenty load now, `open` a name
  where the kernel does not read it, each row's region banked; `open-fact-head`, `open-relation-parameter`,
  `open-obligation-parameter` and `open-solve-list` stay refused, by the loader's new lines. `absent-fact-nested.ein`
  loads (Q-M1f.571), which item 58 did not list. `open_in_match.ein` and `open_mixed_assert.ein` moved (Q-M1f.614,
  below). `tests/stdlib/bijection/06_blind_enumeration.ein` rewritten with no obligation activated (Q-M1f.600).
- `13_witness_of_two` and `14_seats_nothing_proposes` lose the subject the tests read them for (an unproposed
  witness, L9): see V-48.

**`oblig-derived-domain.ein`, run on the built engine** (`ein solve`): flagged (`obligation-guard reads food at absent
depth 1`), H one candidate, `(likes Ann Soup)`, dead alone; R empty, every alive node owes (*Ann owes a liked food*);
its claim, `(model (likes Ann Pie))`, FAILS. Before (`621c2ab1`): H six candidates by the blind enumerator after the
decline, R one state, `(likes Ann Pie)`, the claim held, monotone. Item 64's stand-in said the same of the build.

**Defaults taken in the build, ids for the coordinator to file (Q-M1f.606 to Q-M1f.620).**
None is cited by a project file; each is what the build does until the user rules otherwise.

| id | question | default the build takes |
|---|---|---|
| Q-M1f.606 | which reads of a rule that concludes `(open F)` are the `hypothesis` site | a read of `D` outside an `absent` or inside an even number of them, as a saturation rule's monotone reads; the literal reading of Q-M1f.574 (any depth) flagged `hrule-reads-unknown.ein` and its kin for an `unknown` probe |
| Q-M1f.607 | the blind enumerator under `(solve :no-search)` | it writes nothing: no candidate, as no writer runs |
| Q-M1f.608 | a state's `open` facts within its own pass | written once at the fixpoint and only added to while a rule that reads `open` derives (the pass, then such rules, until nothing changes); none is withdrawn within a state |
| Q-M1f.609 | what the blind enumerator and a later debt write above root | what root read: the blind enumerator's raw list at root and the later debts' candidates, less what the state holds or denies |
| Q-M1f.610 | an obligation whose witness step does not resolve under an activator | the call is declined: nothing is written for any obligation, and the blind enumerator does not run beside an activated obligation (`rung` mode `declined`) |
| Q-M1f.611 | may a claim list an `open` fact | no: a claim lists facts of the program's relations (`ein.y`'s `ClaimNested`); `(model (open k))` is a parse error |
| Q-M1f.612 | an `open` fact's provenance | a pass-written one carries a rule's provenance naming its writer (the obligation, the rule, or `open` for the blind enumerator), printed *which rule … writes*; a written one the loader's source provenance |
| Q-M1f.613 | `(open ?x)` in a conclusion | `?x` a parameter of the rule names a relation (what a state owes); a variable the `:match` binds is a hypothesis |
| Q-M1f.614 | the two parse-negative fixtures Q-M1f.66's old shape made | `open_in_match.ein` moves to `examples/features/23_a_rule_reads_open.ein` (its subject, a rule that reads `open`, is legal and flagged); `open_mixed_assert.ein` moves to `examples/broken/load/` with `:why`s, refused at load |
| Q-M1f.615 | the in-flight warning on stderr | once per file, at once, and only where the static check did not already flag the program (a flagged program's warning is already printed) |
| Q-M1f.616 | does a node's `open` facts count as work and against `--region-budget` | no: `region::NodeOut::work` counts the facts saturation built beyond root, the `open` facts aside, in every route's `facts` |
| Q-M1f.617 | is `open` a guessable atom where a program writes it as a name | no: the kernel's word, left out of the blind enumerator's atoms as `absent` is (`hypgen::candidate_objects`) |
| Q-M1f.618 | `open` and `absent` heading a fact under `not`, in a guard's value and in a `:using` list | names, as under an ordinary relation (Q-M1f.571 read where it is silent): `(not (absent k))`, `(neq ?x (absent k))`, `(q (absent ?x))` load; directly under `open` or `absent`, written at the top and concluded they stay refused |
| Q-M1f.619 | the structure's theory share under the union | the program's own writers built H (`structure.rs`' `declared`), and the relations they read, the rules' and the obligations', are what the universe is made of |
| Q-M1f.620 | an hrule whose `:assert` concludes `(open …)` | refused at load: an hrule proposes a fact, a hypothesis already; a rule that concludes `(open …)` is written instead |

**Findings outside the build, for the coordinator to file (V-46 to V-55):**

- **V-46.** `examples/ein-bugs/wrong-answers/absent-fact-nested.ein` loads (Q-M1f.571), which item 58 did not list;
  its regression row says so, and `ein-cli/tests/value_rule.rs` moved seven `absent` shapes from the refused list
  to the loading one (Q-M1f.618).
- **V-47.** Under the union an activated obligation writes every witness its guard gives at root, so on a monotone
  program no instance root owes has an unproposed witness: L5's last reason, L9 (`ein_infer::unheld`) and the
  coupling's unlicensed path (`Debt.licensed`, `Owing::Unlicensed`, the payers read off holders) have no subject in
  the corpus, nor can have but through the filter pipeline. `unheld.rs` holds that L9 licenses nothing and reads one
  the day it appears; the structure tests dropped the assertions that had no subject. The machinery stays: whether it
  goes is the user's. The read-out's holder sentence at a stopped budget still says *an instance L5 does not license
  has no payers read* on a program with none.
- **V-48.** `zebra.ein`'s H holds each placement beside its mirror (56 → 112). Its structure reads 352 groups and no
  grid where it read 36 groups and five grids, 33 305 coupling edges where 386, and its root layer 705 808 facts where
  181 088 (`ein render h-space examples/zebra.ein`); `zebra.ein` and `zebra2.ein` have one cover and one theory only
  folded by orientation, which the tests now say. The cover reads two candidates that imply each other as two.
- **V-49.** A written `(open R)`, `R` a relation's name, loads and is held at root, and nothing reads it as owed: R is
  {∅} beside it.
- **V-50.** A rule's `(open F)` is held at a state that holds `F`, a rule writing what it concludes, where the
  obligations' and the blind enumerator's are written less what the state holds; `-p` lists both.
- **V-51.** The state route commits a state's closure, so where one candidate implies another its dead extension,
  stopped at its first contradiction, holds a candidate route 1's fork from root has not derived:
  `13_mixed_solution_and_open` counts one fact more on the state route, which `route_invariance.rs` names
  (``e1f37012` (the grammar and the loader), `e46c4c89` (the engine and its tests), `21865cbc` (the corpus), `f0bbd066` (the pages, the decisions, AGENTS.md), `04d780fb` (the goldens), `375e3d4f` (a node's `beyond` without its `open` facts, the rebase's anchor and the examples page); two helpers of the build's own rewrote the pages that told the ladder and restated the structure tests the union moved (`coupling`, `cover`, `exactly_one`, `theory_share`, `h_space_view`), each change in the commits above_AN_IMPLIED_CANDIDATE`); the saturations and the read-out are one.
- **V-52.** `14_seats_nothing_proposes.ein`'s declared runs cost past a second (the oracle over 16 candidates): it is
  flagged `slow`, its `cost_ms` the gate run's figure (9 128), to re-take under `utils/bench_env.sh`.
- **V-53.** The generator's words: the mode `hrules` now names the rules that conclude a hypothesis and the program's
  written `open` facts, and `ein show hyps`' note still says *built by the rung …*; the words stay until the hrule
  form is deleted.
- **V-54.** `(?R ?x)` written where a value goes with `?R` bound to `absent` stays refused (`value_rule.rs`'
  `absent-parameter-value`), while `(q (absent ?x))` loads (Q-M1f.618).
- **V-55.** Q-M1f.66's file name, `open-is-written-only-as-a-rules-verdict`, says what is no longer true: its heading
  and text are rewritten, its slug is not, every link to it would move.

**What the user must see**: the oblig-derived-domain answer above; V-47 (L9 and the unlicensed path have no
subject); V-48 (zebra's structure under the union); the defaults Q-M1f.606 to Q-M1f.620, Q-M1f.618 above all
(`absent` and `open` as names under `not`).

**The gate.** After the record, `2788c84d` (the code formatted, the doc links to private items written as code, the
determinism note on the line that iterates) and `247dd3c3` (`utils/region_synth.py` writes a debt nothing pays with no
witness for its guard, so the obligation writes no payment beside the hrule's candidates, and the detector's two
hundred random programs are written again from it). `./run_tests.sh` on `247dd3c3`, rebased on `b4c29759`: exit 0, its
last step `cargo bench --bench engine -- --test`. `EIN_CORPUS_SLOW=1 cargo test --profile gate -p ein-render --test
dot_wellformed` on the same commit: 5 passed; `STRUCTURE_SKIPPING` gained `zebra.ein`'s four graphs, and the views'
`SKIPPING` nothing. Not run, as the brief says: the nightly and `utils/ci_container.sh`, which a change to the DOT
writers' input (a state's `open` facts drawn) and a large engine change owe.
