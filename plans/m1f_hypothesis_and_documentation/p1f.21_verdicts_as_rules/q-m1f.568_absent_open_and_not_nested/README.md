# 46 · Q-M1f.568: `absent`, `open` and `not` nested, the four cells that are yours

[← part 2](../review/part-2/README.md) · [← the phase](../README.md) ·
[the ledger](../../open_questions.md#q-m1f568--absent-open-and-not-nested-the-cells-that-are-the-users) ·
[the table](../absent_open_not.md)

**Ruled 2026-11-19, and out of the review bundle**, where it was item 46 of part 2. Your answer, in the part's
answers' file:

> 4 - yes, 3 - yes, 2,1 and (open (absent P)) - refused at load because both `absent` and `open` are special
> kernel primitives (what are they now btw?) and their argument must be a fact, (absent P) is not a fact, while
> (open P) is, because `open` is a special kernel promitive AND a fact.

It is filed under the question in the ledger and in [the phase's README](../README.md#the-answers-to-items-44-45-and-46).
What it leaves open is review items 49 ([part 2 of the bundle](../review/part-2/README.md)). The page is kept, with its
programs, as the survey the second part's stages cite; where it says *not ruled* and *the default*, read it
as it stood before the answer. Its outputs and its tools (`runs/` and `tools/` below) are on this machine under
`build/review-archive/p1f21-part-2/46-Q-M1f.568/`, as they were run.

**Raised** by your answer to item 32, 2026-11-19: *"We have to analyze and strictly define interaction between
absent, open and not. … (not (not P)) is simple, (absent (absent P)) probably compilation error, (open (absent P))
also compilation error. While (absent (open P)) possible. Maybe kernel primitives can't be nested because it's
argument always a fact?"*

[The table](../absent_open_not.md) answers it cell by cell on runs. Two rules that are already Ein's fill
every cell once `(open P)` is a fact: *`absent` is no fact, so it stands under none*, and *a fact's argument is a
fact a state could hold*. **Four cells they do not decide are here, each with a default.**

**Not ruled.** Nothing is built: every run below is today's engine, the gate build of master at `edbe6c3b`
(`build/review-bin/ein-head-edbe6c3b`), on the programs in [`programs/`](programs/). `tools/run.sh` re-takes them
all into `runs/` (both ignored, so on this machine only); `runs/cells.txt` has one block a program. Today the parse
reads `open` nowhere but as an obligation's verdict, so each cell with `open` is shown twice: a `.target.ein`, the
form as it would be written, refused today, and the same with `question` standing for `open`, which runs.

## 1. `(absent (absent P))`, written directly

| program | today |
|---|---|
| [`1-absent-absent.ein`](programs/1-absent-absent.ein): `(and (q ?x) (absent (absent (p ?x))))` | **loads**. Root derives `(hit A)` and not `(hit B)`: it is true where `(p ?x)` is held |
| [`1-absent-and-absent.ein`](programs/1-absent-and-absent.ein): `(absent (and (q-of ?g ?x) (absent (p ?x))))` | loads. Root derives `(all-p G1)` and not `(all-p G2)`: *every member is a p*, the universal |

You expected the first to be a compilation error. It is not one today, and it says nothing a bare `(p ?x)` does not
say. The second is how the standard library's `forall` is written, and stays under either answer.

- **(a)** the direct form is refused at the parse. **Default**: no corpus program writes it.
- **(b)** it is left as it is.

## 2. `(open (open P))`

| program | today |
|---|---|
| [`2-open-open.target.ein`](programs/2-open-open.target.ein) | refused at the parse |
| [`2-question-question.ein`](programs/2-question-question.ein) | loads. $H$ is `(question (p A))`, one candidate. The state that supposes it holds `(question (p A))`, and `(p A)` is not proposed |

It is a hypothesis that P may be supposed: a hypothesis that depends on another, which you parked
([V-21](../findings.md#v-21--dependent-hypotheses-a-candidate-proposed-above-root)). As a fact it is legal by
the two rules; what it would mean to the search is what is parked.

- **(a)** refused at load, until dependent hypotheses are taken up. **Default.**
- **(b)** legal, and read as the stand-in reads it: the inner hypothesis is held above root and never proposed.

## 3. `(not (open P))` and `(open (not P))`

| program | today |
|---|---|
| [`3-not-open.target.ein`](programs/3-not-open.target.ein), [`3-open-not.target.ein`](programs/3-open-not.target.ein) | refused at the parse |
| [`3-not-question.ein`](programs/3-not-question.ein): `(question (p A))` and `(not (question (p B)))` | loads. $H$ is `(p A)`. The negation is a held fact nothing reads |
| [`3-not-question-both.ein`](programs/3-not-question-both.ein): `(question (p A))` and `(not (question (p A)))` | loads, and **root is dead**: a fact beside its stored negation. The core is the two facts |
| [`3-question-not.ein`](programs/3-question-not.ein): `(question (not (p A)))` | loads. $H$ is `(not (p A))`: the candidate is a stored negation |

Both are legal by the two rules. `(open (not P))` is a hypothesis that a negation holds, which a hypothesis rule
can already propose. `(not (open P))` is an ordinary fact; the kernel reads nothing in it, so it does not mean *do
not suppose P* unless a rule says so. Written beside `(open P)` it is a fact and its negation: dead today by the
kernel's own check, and after S1f.21.11 only where the program imports `std.contradiction`
(Q-M1f.557 (d)).

- **(a)** both legal, with no meaning to the kernel. **Default.**
- **(b)** `(not (open P))` refused, so that no program takes it for a way to take a candidate out.

## 4. `(open P)` written by a program

| program | today |
|---|---|
| [`4-open-written.target.ein`](programs/4-open-written.target.ein) | refused at the parse |
| [`4-question-written.ein`](programs/4-question-written.ein): `(question (p A))` `(question (p B))`, no rule concluding them | loads. $H$ is `(p A)` `(p B)`; $R$ is all four commitments |

A candidate given by hand, with no rule.

- **(a)** legal: it is a fact. **Default.**
- **(b)** refused: a hypothesis is only what a rule concludes.

## 5. Files

- [`programs/`](programs/): the eleven programs above.
- `runs/`: for each program its read-out (`.solve.txt`, `.solve-p.txt`), `ein show hyps`, `ein saturate --dump`,
  the oracle's read-out, and for each that loads the oracle's dump with every node's state, the events and the
  summary, the answer as files and the knowledge base drawn (`.kb.dot`, `.kb.svg`); `cells.txt`.
- `tools/run.sh`, `tools/cells.py`.
- The nine cells of the whole table, each as a premise, a conclusion and a written fact, are run by
  [`absent_open_not/run.py`](../absent_open_not/run.py).

## 6. The ruling

Ruled: the answer at the head of this page.
