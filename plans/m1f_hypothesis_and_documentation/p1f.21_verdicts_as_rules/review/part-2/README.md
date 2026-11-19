# Part 2 — `open`: what a state owes, and the hypotheses

[← the bundle](../README.md) · [← the phase](../../README.md) · [your answers](ANSWERS.md)

**Five questions are open, 47 to 51: the gaps your answers to items 44, 45 and 46 leave**, written here on your
word (*"write down gaps as questions in …/review/part-2"*, 2026-11-19). Each is a folder with its page, its
programs and their runs, and each has a default. They wait with their part: nothing of part 1 needs them.

## What is left for you

| # | question | what it asks | the defaults |
|---|---|---|---|
| [**47**](47-Q-M1f.569/README.md) | Q-M1f.569 | what may read an `open` fact, and when. A fork drops the `open` facts and keeps what a rule concluded from them | a rule may read them; they are written by a pass at each state's fixpoint, and a program whose rule reads `(open R)` or an obligation's candidate is flagged as not monotone. A written `(open P)` is there in every state |
| [**48**](48-Q-M1f.570/README.md) | Q-M1f.570 | a rule that concludes `(open F)` beside an obligation: does the rule override the obligation's candidates, as a hypothesis rule does today, or is $H$ the union | override, as today: no answer moves. With the union three corpus programs shown answer otherwise |
| [**49**](49-Q-M1f.571/README.md) | Q-M1f.571 | what `absent` and `open` take, and what each is under an ordinary relation. *Their argument must be a fact* refuses every obligation's `(absent (and …))` and does not refuse `(open (open P))` | `absent` takes a premise, the universal stays, only the direct `(absent (absent P))` is refused; `(open (open P))` refused; under an ordinary relation both are ordinary names |
| [**50**](50-Q-M1f.572/README.md) | Q-M1f.572 | whether a state's `open` facts are listed, counted and compared with its facts | held and shown apart: a state's listing, counts and identity are of its other facts |
| [**51**](51-Q-M1f.573/README.md) | Q-M1f.573 | `(open ?v)` with a variable, obligation or hypothesis; a hypothesis root has already decided; $H$ is root's | decided at load by what the variable is; a decided one is no candidate, as today; $H$ is root's |

**Where to answer.** [`ANSWERS.md`](ANSWERS.md) is yours and holds your answers to 44, 45 and 46: I have added
nothing to it. Write under a heading of the item's number there, or tell me. An item left unanswered keeps its
defaults.

## Answered, and not filed yet

Items [44](44-Q-M1f.566/README.md), [45](45-Q-M1f.567/README.md) and [46](46-Q-M1f.568/README.md) are answered in
your file. They are not filed as rulings yet, and their folders stay until they are: say *process* and I file them
in your words, with the stages planned on them.

| item | your answer, in short | what it leaves, here |
|---|---|---|
| 44 | obligations stay as they are, `(open R)` with its `absent`; `(open R)` is held as a fact and counted as before; `(open F)` over a fact is a hypothesis, the obligations' candidates among them; the bare `(open)` is refused; a fork drops every `open` fact | 47, 48, 50, 51 |
| 45 | the `hrule` form and the keyword `:hrules` go; a rule that concludes `(open F)` is activated by a fact the program writes anywhere | 48, 51 |
| 46 | cells 3 and 4 legal; `(absent (absent P))`, `(open (open P))` and `(open (absent P))` refused at load | 49 |

**What follows from them without a question**, for the stage that builds them:

- **S1f.21.16 is not started**, on your word: *"not start S1f.21.16, will drop it after :hrules removed"*. It
  would spell the `:hrules` clause again, which item 45 removes.
- **The bare `(open)`** is written by one corpus program that loads,
  `tests/stdlib/bijection/06_blind_enumeration.ein`, which uses it to make the obligations rung decline: it needs
  another way to do that. A named `(open R)` is already refused without an `absent` that holds its witness, so
  after it every obligation asks a question ([item 44](44-Q-M1f.566/README.md), § 5, fifth point).
- **S1f.21.9**, the requirement and the payment as the kernel's facts, is not run: item 44's answer keeps today's
  count. `std.debts` stays a module a program imports.

## What to read beside them

| page | what it is |
|---|---|
| [`absent_open_not.md`](../../absent_open_not.md) | the table of `absent`, `open` and `not`, each inside each, as today's binary reads it |
| [`open_and_owe.md`](../../open_and_owe.md) | your four questions of item 34, answered |

## Ruled earlier, and out of the bundle

| item | question | ruled | where it is now |
|---|---|---|---|
| 32 | Q-M1f.390 | an obligation emits `(open H)`, no justification in it; `absent` reads it as any fact | [`q-m1f.390_what_an_obligation_emits/`](../../q-m1f.390_what_an_obligation_emits/README.md) |
| 34 | Q-M1f.549 | `(open …)` is the context for hypotheses, *may*; an obligation is *must* | [`q-m1f.549_hypotheses_as_facts/`](../../q-m1f.549_hypotheses_as_facts/README.md) |
| 39 | Q-M1f.560 | (b): `std.debts` is merged. **Done**: `edbe6c3b` | in `git log` |

**The binary.** Every item runs on `build/review-bin/ein-head-edbe6c3b`, the gate build of master at `edbe6c3b`,
with the checkout's `stdlib/`; each item's `tools/run.sh` re-takes its `runs/`.
