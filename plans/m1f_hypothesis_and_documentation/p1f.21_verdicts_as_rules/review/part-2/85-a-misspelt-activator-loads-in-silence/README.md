# 85 · A misspelt activator, or one of another count, loads in silence

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the finding](../../../findings.md#v-58--a-misspelt-activator-or-one-of-another-count-loads-in-silence)

A finding of the phase that is **yours to call**. It was on the findings page alone; it is here so that everything
that waits for you is in one place. Nothing waits on it.

## What it is

A hypothesis rule's activator is an ordinary fact since the `hrule` form went. One with the rule's name misspelt,
or of another count than the rule's parameters, loads and activates nothing, where the `:hrules` clause refused
both. With no other writer the blind enumerator then writes $H$: on a probe, six guesses where the program meant
its rule's candidates, and no word said. The diagnostics page says so.

## The question

- **(a)** As built: an activator is a fact as every rule's, and nothing is said. **Default.**
- **(b)** A `note:` at load where a rule that concludes a hypothesis has parameters and no activator.

## On the built engine

Taken by [S1f.21.19](../../../s1f.21.19_the_review_items_on_the_built_engine.md) on the final binary, the release
build of `7b72bd84` (`runs/version.txt`), by `tools/run.sh`; both folders are in the main checkout beside this page,
not in git. Seven programs, each in [`programs/`](programs/):

- [`written-right.ein`](programs/written-right.ein): a hypothesis rule `guess (?R ?T1 ?T2)` that concludes
  `(open (?R ?a ?b))`, two colours, two houses, a rule that refutes two colours in one house, and the activator
  `(guess at Color House)`.
- [`misspelt-activator.ein`](programs/misspelt-activator.ein): the same, the activator written `(gues at Color House)`.
- [`activator-of-another-count.ein`](programs/activator-of-another-count.ein): the same, the activator written
  `(guess at Color)`, two arguments for three parameters.
- [`the-findings-probe.ein`](programs/the-findings-probe.ein): the probe the finding's *six* was taken on, copied as
  it was, a header comment added. Its activator has the rule's name and three arguments and misspells an argument,
  `Colour`, which is
  [Q-M1f.116](../../../../../../docs/decisions/language/Q-M1f.116-an-activators-argument-is-not-checked-against.md)'s
  case rather than a misspelt name or another count.
- Three more with `is-a` declared as well, and nothing else changed:
  [`written-right.is-a-declared.ein`](programs/written-right.is-a-declared.ein),
  [`misspelt-activator.is-a-declared.ein`](programs/misspelt-activator.is-a-declared.ein), and
  [`misspelt-argument.is-a-declared.ein`](programs/misspelt-argument.is-a-declared.ein), the program written right
  with the probe's misspelt argument, `(guess at Colour House)`.

Each of the first four has the full set of runs: the read-out, `-p`, `ein show hyps`, `ein saturate --dump`, the
oracle and its dump, the events and the summary, and the knowledge base, the lattice and the region drawn (`.dot` and
`.svg`); the lattice also as `--view full` for the two small cubes. `X.solve.stderr.txt` and `X.show-hyps.stderr.txt` hold the stderr of
`ein solve` and of `ein show hyps` alone. The three with `is-a` declared have `ein show hyps`, the read-out, the two
stderrs, `ein saturate --dump` and the summary: no oracle and no drawing, the blind H of the two misspelt ones being
24 and 40 candidates, past the oracle's 16, and the one written right reading as it does without the declaration.

| program | activator | `ein show hyps`' note (stderr) | H | stderr of `ein solve` | R | `ein saturate --dump` |
|---|---|---|---|---|---|---|
| written right | `(guess at Color House)` | ``H, from `hypothesis-rules`: 4 candidates`` | the four `(at C H)`, C a colour and H a house | nothing | 9 states | `guess :params (?R ?T1 ?T2)  1 activator` |
| misspelt | `(gues at Color House)` | ``H, from `blind`: 12 candidates`` | 12 guesses `(at X Y)`, X and Y two of `Red`, `Green`, `H1`, `H2`: `(at H1 H2)` and `(at Green Red)` among them | nothing | 256 states | `guess :params (?R ?T1 ?T2)  no activator` |
| another count | `(guess at Color)` | ``H, from `blind`: 12 candidates`` | the same 12 | nothing | 256 states | `guess :params (?R ?T1 ?T2)  no activator` |
| the finding's probe | `(guess at Colour House)` | ``H, from `blind`: 6 candidates`` | 6 guesses over `Red`, `H1` and `Colour` | nothing | 64 states | `guess :params (?R ?T1 ?T2)  1 activator` |

The files: `runs/X.show-hyps.txt` and `runs/X.show-hyps.stderr.txt` (the note), `runs/X.solve.stderr.txt` (each
holds only `[exit 0]`), the `states (k_s)` row of `runs/X.solve.txt` and of `runs/X.oracle.txt` (they agree),
and the `;; Hypothesis rules (1)` lines of `runs/X.saturate-dump.txt`. The two that load in silence have one H
(`runs/misspelt-vs-another-count.show-hyps.diff.txt`: *the same H, the note's file name apart*).

**The finding's figure beside today's.**

| | the finding's figure (S1f.21.18's probe) | the final binary |
|---|---|---|
| H on the probe, `(guess at Colour House)` | six guesses | 6 (`runs/the-findings-probe.show-hyps.txt`) |
| H with the rule's name misspelt | not given | 12 where the rule means 4 (`runs/misspelt-activator.show-hyps.txt`) |
| H with an activator of another count | not given | 12 where the rule means 4 (`runs/activator-of-another-count.show-hyps.txt`) |

The figure stands. The finding's probe misspells an argument, not the rule's name or the count, so the two programs
this item adds are what show the two cases the finding names.

**What says anything.** No load line and no warning: `ein solve`'s stderr is empty on the misspelt program and on
the one of another count, as on the program written right, and `ein show hyps`' stderr holds its note alone. What is
said, only where it is asked for:

- `ein show hyps` names H's writer on stderr, `blind` where the program meant `hypothesis-rules`.
- The read-out's `universe` row says `12 candidates`, and nothing names the activator (`runs/misspelt-activator.solve.txt`).
- `ein saturate --dump` lists the hypothesis rule with `no activator`, and the misspelt activator as a fact of an
  auto-vivified relation, `gues` (`runs/misspelt-activator.saturate-dump.txt`).
- The `load` event names `gues` among the program's relations and counts one hypothesis rule; root holds one fact
  of `gues` and the twelve `open` facts the blind enumerator wrote (`runs/misspelt-activator.load-and-root.txt`).

**Where the program declares the relation the rule reads** (`runs/X.show-hyps.stderr.txt`, `runs/X.solve.txt`,
`runs/X.solve.stderr.txt` and `runs/X.saturate-dump.txt` of the three `.is-a-declared` programs), the blind
enumerator guesses `is-a` facts too, and what follows depends on the misspelling:

| program | `ein show hyps`' note | the read-out | stderr of `ein solve` |
|---|---|---|---|
| written right, `is-a` declared | ``H, from `hypothesis-rules`: 4 candidates`` | 9 states, exhausted, monotone | nothing |
| the rule's name misspelt, `is-a` declared | ``H, from `blind`: 24 candidates`` | not exhausted: the state route stopped at `--region-budget` 1 048 576 before the states of 6 candidates; 103 109 states found, a lower bound | the `partial` note alone |
| an argument misspelt, `is-a` declared | ``H, from `blind`: 40 candidates`` | **flagged**: the `WARNING` row, *THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER*, and one site, ``guess [activator at Colour House]: hypothesis reads is-a at absent depth 0, concludes (open …)``; not exhausted, the cube 2^40 past `--region-limit` 16 | first line `WARNING: programs/misspelt-argument.is-a-declared.ein: THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER (Q-M1f.564)`, then the `partial` note |

The flagged one is the one place where something said unasked names the misspelt activator, `activator at Colour
House`, and it does so as a site of the static check, because the blind enumerator's `is-a` guesses can bring the activated rule a
hypothesis root does not hold; nothing is claimed of its answer. The misspelt name, with `is-a` declared, says
nothing of the activator, and the run stops at the budget where the program written right has 9 states.

**What the diagnostics page says** (§ 2.20, *What is not refused here*; `runs/diagnostics-sentence.txt` has the
line): *"A rule that concludes a hypothesis is activated as any rule is, by a fact of its name (Q-M1f.567), and an
activator is a fact, held to what every fact is held to: one of another count than the rule's parameters activates
nothing, and an argument at a parameter that stands for no relation is any object, so `(guess at Colour House)` loads
and has the rule propose nothing; where no other writer proposes either, the blind enumerator writes H. A rule with
parameters that no fact activates proposes nothing."* And § 2.6: *"An activator of the wrong arity activates nothing
and is not checked."* The runs agree with both.

**For option (b).** Its `note:` is for a rule that concludes a hypothesis, has parameters and has no activator,
which `ein saturate --dump` prints as `no activator` on the misspelt program and on the one of another count, and as
`1 activator` on the finding's probe, whose rule proposes nothing because an argument is misspelt. On these four
programs such a note would be said on the first two and not on the probe; with `is-a` declared, the dump prints
`no activator` on the misspelt name and `1 activator` on the misspelt argument (`runs/X.saturate-dump.txt`).
