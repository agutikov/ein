# 52 · Q-M1f.574: a hypothesis first held above root: how the program is found not monotone

[← part 2](../review/part-2/README.md) · [← the phase](../README.md) ·
[the ledger](../../open_questions.md#q-m1f574--a-hypothesis-first-held-above-root-how-the-program-is-found-not-monotone)

**Ruled 2026-11-20, and out of the review bundle**, where it was item 52 of part 2. Your answer, in the part's
answers' file:

> 1 - (c); 2 - (a) Warning shown immediately when detected, once;

It is filed under the question in the ledger and in [the phase's README](../README.md#the-answers-to-items-52-to-55).
It leaves nothing open. The page is kept, with its programs, as the survey the second part's stages cite; where it says
*the default*, read it as it stood before the answer. Its outputs and its tools (`runs/` and `tools/` below) are
on this machine under `build/review-archive/p1f21-part-2/52-Q-M1f.574/`, as they were run.

**From your answer to item 47**, 2026-11-20: *"Yes, monotone program step by step proves or refutes hypotheses, not
adding new, right? If it does - then R is not upward-closed, search lattice is not hypercube, program is not
monotone, and maybe even lattice is not finite (not sure about that). Maybe it's good point for non-monotonicity
detection during search, in addition to static rules analysis that is performed before search starts, we could
detect non-monotonicity in-flight and WARN user (if it was not done yet)."*

Nothing is built: every run is today's engine, the gate build of master at `edbe6c3b`
(`build/review-bin/ein-head-edbe6c3b`), on the programs in [`programs/`](programs/), with `question` standing for
`open`. `tools/run.sh` re-takes them into `runs/` (both ignored, so on this machine only).

## 1. *Right?* By your definition, yes; by today's static check, no

You ruled that a hypothesis proposed above root makes the program not monotone by definition (V-21). **Today's
static check does not say so.** It flags a rule whose `absent` reads what a commitment can change, and an
obligation whose trigger does. A rule with no `absent` that concludes a hypothesis from a fact only a commitment
brings is none of those (`runs/above-root.md`):

| program | the static check | states that hold a hypothesis root does not |
|---|---|---|
| [`above-root-rule.ein`](programs/above-root-rule.ein): `(chosen ?d)` and an extra give `(question (with ?d ?e))` | **monotone** | 3 of 4: `{(chosen Tea)}` holds `(question (with Tea Sugar))` |
| [`above-root-chain.ein`](programs/above-root-chain.ein): `(at ?x)` gives `(question (at (s ?x)))` | **monotone** | 1 of 2: `{(at (s Z))}` holds `(question (at (s (s Z))))` |
| [`above-root-obligation.ein`](programs/above-root-obligation.ein), a corpus program whose obligations require only with a commitment | flagged: `owe-1` and `owe-2`, each an obligation's trigger | none: it is the *must* that appears above root |

So the static check already has this kind of site for an obligation, and has none for a rule that concludes a
hypothesis.

**On the lattice.** With $H$ root's, which you confirmed, it stays a cube and finite: what is first held above
root is held and never proposed, and $R$ is computed over root's candidates. Your sentence describes what the
space would be if such a hypothesis were proposed: no cube, and not finite where a rule builds a new fact from the
one supposed, as `above-root-chain.ein` does: each step supposed asks the next.

## 2. The questions

**1. How it is detected.**

- **(a)** **Statically.** A rule that concludes `(open F)` and whose `:match` reads a relation a commitment can
  change is a site of the static check, as an obligation's trigger is today. The program is flagged before the
  search. It is at the grain of relations, so it can flag a program in which no state adds a hypothesis.
- **(b)** **In flight.** A state the run builds that holds an `(open F)` root does not hold: the run reports the
  program as not monotone, where it was not flagged. Exact on the states the run builds; a run that builds few can
  miss it. On the three programs above the describing run builds every such state.
- **(c)** **Both. Default**: your *"in addition to static rules analysis"*.

**2. What the run's answer is, once found in flight.**

- **(a)** As computed, under the WARNING, and nothing is claimed of it. **Default**: your *"WARN user"*.
- **(b)** Computed again by the cube, as a program the static check flags is answered.

The warning is the read-out's first row and stderr's first line today. Found in flight, the row is printed as it
is, the read-out being written after the run; stderr's line comes when it is found.

## 3. The ruling

Ruled: the answer at the head of this page.
