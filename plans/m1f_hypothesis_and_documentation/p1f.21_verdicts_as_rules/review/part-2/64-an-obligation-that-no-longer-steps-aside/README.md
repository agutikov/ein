# 64 · An obligation that no longer steps aside: the eight programs, and `oblig-derived-domain.ein`

[← part 2](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.12_hypotheses_as_facts.md) ·
[the ruling's page](../../../q-m1f.579_an_obligation_that_steps_aside/README.md) · [your answers](../ANSWERS.md)

**What it shows.** Task 1.7 of the stage: the eight corpus programs whose obligations rung steps aside today, each
as it answers today and as it would with its obligation's guard read at root, as you ruled (Q-M1f.579 (b)). And
`examples/ein-bugs/wrong-answers/oblig-derived-domain.ein`, which you asked to see: what the stand-in answers, and
what would make it solvable or not. Your expectation, *"I expect it will still be solvable, despite
non-monotonicity. This is my expectation, not requirement. Let's review it after implementation"*, is not turned
into anything here; the program is shown to you again after the build, as the stage's plan says.

**The answer, short.** Once built, **all eight are flagged** (six newly), and nothing is claimed of their answers.
On the stand-in, four answer an empty $R$, `oblig-derived-domain.ein` among them; the four `-declared` twins,
which state the derived food at root, keep a state: two the same one state as today, two fewer states than today. `oblig-derived-domain.ein` answers
`(likes Ann Pie)` on the stand-in **if Pie is a food at root, or if a rule says what Ann may like**; it does not
otherwise, and reading the guard at every state does not change that.

## 1. How it was taken

Nothing is built: every run is today's engine, `build/review-bin/ein-head-edbe6c3b`, with the checkout's
`stdlib/`, on the corpus at `669d7d43`. `tools/run.sh` re-takes every run into `runs/` (ignored by git; mirrored
under `build/review-incoming/p1f21-s12-examples/64-an-obligation-that-no-longer-steps-aside/`): for each program its
read-out, `-p`, `ein show hyps`, `ein saturate --dump`, the static check alone, the oracle with every node's state,
the events and their `rung` line, the summary, the answer as files and the knowledge base drawn.

**The stand-in.** Each `programs/*.guard-at-root.ein` is the corpus program whole, with one hypothesis rule more,
`guard-at-root`, whose `:match` is the obligation's trigger, its `absent` and its guard, and whose `:assert` is the
fact that would pay: exactly the candidates the obligation's guard gives at root for each instance root owes. Today a
hypothesis rule overrides the obligations rung, so the stand-in's H is those candidates and nothing else, which is
what the obligation writes once it no longer steps aside. `tools/gen.py` wrote them; their `:expect` is the
original's.

**What the stand-in cannot show: the flag.** Today's static check calls six of the eight monotone, and the stand-ins
too (`runs/*.monotone.txt`). Once built, the guard of each reads a relation a commitment can change (`food`, which
the rule `special` derives once someone likes something; `is-a` in the two that use `std.algebra`'s `total-owed`),
which is the new site of the static check, so each is flagged. That is a reading of the ruling, taken from D as
today's static check computes it ([item 58](../58-the-corpus-under-the-rulings/README.md), § 6).

## 2. The eight

All under `examples/ein-bugs/wrong-answers/`.

| program | who proposes today | H today → guard at root | $R$ today → stand-in | its claim today → stand-in | flagged today → built | read by today |
|---|---|---|---|---|---|---|
| [`oblig-derived-domain.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-derived-domain.ein) · [stand-in](programs/oblig-derived-domain.guard-at-root.ein) | the blind enumerator | 6 → 1 | 1 state → **empty** | holds → fails | no → **yes** | a regression row |
| [`oblig-derived-domain-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-derived-domain-declared.ein) · [stand-in](programs/oblig-derived-domain-declared.guard-at-root.ein) | the blind enumerator | 6 → 2 | 1 → 1 | holds → holds | no → **yes** | a `test` run |
| [`oblig-fresh-name.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name.ein) · [stand-in](programs/oblig-fresh-name.guard-at-root.ein) | the blind enumerator | 6 → 1 | 1 → **empty** | holds → fails | no → **yes** | a regression row |
| [`oblig-fresh-name-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name-declared.ein) · [stand-in](programs/oblig-fresh-name-declared.guard-at-root.ein) | the blind enumerator | 6 → 2 | 1 → 1 | holds → holds | no → **yes** | a `test` run |
| [`oblig-fresh-name-two.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name-two.ein) · [stand-in](programs/oblig-fresh-name-two.guard-at-root.ein) | the blind enumerator | 12 → 2 | 12 → **empty** | holds → fails | no → **yes** | a regression row |
| [`oblig-fresh-name-two-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name-two-declared.ein) · [stand-in](programs/oblig-fresh-name-two-declared.guard-at-root.ein) | the blind enumerator | 12 → 4 | 12 → 3 | holds → fails | no → **yes** | a `test` run |
| [`oblig-stdlib-total-owed.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed.ein) · [stand-in](programs/oblig-stdlib-total-owed.guard-at-root.ein) | the blind enumerator | 6 → 1 | 16 → **empty** | holds → fails | yes → yes | none |
| [`oblig-stdlib-total-owed-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed-declared.ein) · [stand-in](programs/oblig-stdlib-total-owed-declared.guard-at-root.ein) | the blind enumerator | 6 → 2 | 16 → 1 | holds → fails | yes → yes | none |

*Holds* and *fails* are what the run prints of the claim. **Once built nothing is claimed of any of the eight**:
no `test` run, regression row or claim-checking run may hold a flagged program's answer (Q-M1f.564), so the build
takes the first six's out (§ 4).

Why the candidates shrink: the blind enumerator guesses `likes` over every name (`(likes Pie Ann)`, `(likes Soup
Pie)` and the like, a person only being required of the one who likes); the guard proposes only what pays the
debt, a food at root. Why the `-declared` twins keep a state and the others lose theirs: the twin states at root the
food a commitment would otherwise derive (`(food Pie)`, `(food Z)`, `(is-a Z Food)`), so the guard reads it there.

## 3. `oblig-derived-domain.ein`

Ann is a person; Soup is a food; Pie is a dish. Ann owes a liked food, and may not like Soup. Once anyone likes
anything, every dish is a food (`special`). Its claim: $R$ is the one state where Ann likes Pie.

| | who writes Ann's candidates at root | H | $R$ | flagged once built |
|---|---|---|---|---|
| [today](../../../../../../examples/ein-bugs/wrong-answers/oblig-derived-domain.ein) | the blind enumerator, the obligations rung having stepped aside | 6, `(likes Ann Pie)` among them | `(likes Ann Pie)` | — (today: no) |
| [the guard at root](programs/oblig-derived-domain.guard-at-root.ein), the ruling | the obligation: `(food ?b)` at root is Soup | `(likes Ann Soup)` | **empty**: root owes, the one candidate is dead | yes |
| [`(food Pie)` at root](programs/oblig-derived-domain-declared.guard-at-root.ein), its twin | the obligation: Soup and Pie | `(likes Ann Pie)` `(likes Ann Soup)` | `(likes Ann Pie)` | yes |
| [a rule says what Ann may like](programs/oblig-derived-domain.a-rule-proposes.ein) ([as ruled](programs/oblig-derived-domain.a-rule-proposes.target.ein)) | the obligation and the rule `may-like`, the union | `(likes Ann Pie)` `(likes Ann Soup)` | `(likes Ann Pie)` | yes |

So, on the stand-in of the ruling as built, **the program is not solvable: $R$ is empty**, and the read-out says root
owes and nothing pays. Two things make it answer `(likes Ann Pie)`: Pie a food at root, or a rule that proposes it
beside the obligation (`may-like` concludes `(open (likes ?a ?d))` for every dish; its file in the spelling ruled is
refused at the parse today, `runs/oblig-derived-domain.a-rule-proposes.target.solve.txt`). Either way the guard still
reads `food`, so the program is flagged and nothing is claimed of what it answers.

**What does not make it solvable.** Reading the guard at every state, as the pass does once built (Q-M1f.569): the
one commitment root proposes, `(likes Ann Soup)`, is dead, and in it Ann's debt is paid by Soup, so no state above
root writes `(open (likes Ann Pie))`; the check in flight finds nothing here, and would change no answer if it did
(Q-M1f.574: the answer stands under the warning). Supposing what is first held above root, which you parked (V-21),
would not either, for the same reason.

## 4. What the build changes for them

- Each of the eight is flagged; the read-out's first row, stderr's first line and the summary's warning say so.
- `oblig-derived-domain-declared`, `oblig-fresh-name-declared` and `oblig-fresh-name-two-declared` lose their
  `test` run; `oblig-derived-domain`, `oblig-fresh-name` and `oblig-fresh-name-two` their regression row; each of
  the six its claim, or its `solve` run, since that run checks the claim.
- `oblig-derived-domain.ein` is run again on the built engine and shown to you beside this page's stand-in.

## 5. Your call

The ruling stands as you gave it; this page reports what it gives.

- **(a)** Build as ruled. `oblig-derived-domain.ein` is shown to you after the build beside these runs. **Default.**
- **(b)** As (a), and the build adds `oblig-derived-domain.a-rule-proposes.ein`'s program, in the spelling ruled, to
  the corpus beside the witness, so the corpus has the program that answers `(likes Ann Pie)` under the ruling
  (flagged, with no claim).
