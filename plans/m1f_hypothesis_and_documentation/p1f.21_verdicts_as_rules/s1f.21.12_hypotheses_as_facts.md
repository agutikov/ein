# S1f.21.12 — `open` facts: the hypotheses held, and the obligation's verdict a fact

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*) and
**planned again 2026-11-20 on the user's rulings** of [Q-M1f.566](../open_questions.md#q-m1f566--how-a-requirement-is-emitted-the-must-beside-open) to [Q-M1f.579](../open_questions.md#q-m1f579--an-obligation-whose-guard-or-trigger-reads-what-a-commitment-can-change)
([the answers to items 44 to 46](README.md#the-answers-to-items-44-45-and-46),
[to items 47 to 51](README.md#the-answers-to-items-47-to-51),
[to items 52 to 55](README.md#the-answers-to-items-52-to-55),
[to items 56 and 57](README.md#the-answers-to-items-56-and-57)). **Not started**: it starts on the user's word, after
part 1.
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

Nothing yet.
