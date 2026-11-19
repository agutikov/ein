# 61 · A rule that reads an `open` fact: the program flagged, and the warning's rows

[← part 2](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.12_hypotheses_as_facts.md) ·
[the ruling's page](../../../q-m1f.569_what_may_read_an_open_fact/README.md) · [your answers](../ANSWERS.md)

**What it shows.** Task 1.4 of the stage. You ruled that a rule may read an `open` fact, and that its program is then
not monotone automatically: warned, nothing claimed, no test (Q-M1f.569). Here are the three programs of the ruling's
page in the spelling ruled, their stand-ins as they run today, and the warning's rows as they would read once built.

**The answer, short.** Once built all three load and **all three are flagged**: a rule that reads the *must*,
`(open ?R)`; one that reads a *may*, `(open (chosen ?x))`; one that reads the absence of a *may*,
`(absent (open (served ?x)))`. Today only the first stand-in is flagged, and for a reason of its own; the two
others are not, their `question` facts being made of facts no commitment changes. The warning's three rows are
today's, word for word; what is new is the site the static check names.

## 1. The runs

Today's engine, `build/review-bin/ein-head-edbe6c3b`, with the checkout's `stdlib/`. `tools/run.sh` re-takes them
into `runs/` (ignored by git; mirrored under `build/review-incoming/p1f21-s12-examples/61-a-rule-that-reads-open/`):
each target's refusal, and for each stand-in its read-out, stdout and stderr apart, the static check alone, `ein show
hyps`, `ein saturate --dump`, the oracle with every node's state, the events, the summary and its `monotonicity`
block, the knowledge base drawn. The stand-ins are the ruling's page's programs, run where they are kept.

| program | the spelling ruled | today | its stand-in, today | $R$ on the stand-in | once built |
|---|---|---|---|---|---|
| the *must*, read | [`r-held-and-read.target.ein`](programs/r-held-and-read.target.ein): `noted` matches `(open ?R)` | refused at the `open` | [`r-held-and-read.ein`](../../../q-m1f.569_what_may_read_an_open_fact/programs/r-held-and-read.ein), `(unfinished seats)` for `(open seats)`: **flagged**, one site, `unfinished`, whose `absent` reads `debt-paid` | 2 states | **flagged**: `noted` reads `open` |
| a *may*, read | [`f-held-and-read.target.ein`](programs/f-held-and-read.target.ein): `asked` matches `(open (chosen ?x))` | refused at the `(` after `open` | [`f-held-and-read.ein`](../../../q-m1f.569_what_may_read_an_open_fact/programs/f-held-and-read.ein), `question` for `open`: not flagged | 12 states | **flagged**: `asked` reads `open` |
| a *may*'s absence, read | [`f-absent-read.target.ein`](programs/f-absent-read.target.ein): `not-asked` matches `(absent (open (served ?x)))` | refused at the `(` after `open` | [`f-absent-read.ein`](../../../q-m1f.569_what_may_read_an_open_fact/programs/f-absent-read.ein): not flagged | 3 states | **flagged**: `not-asked` reads `open` under `absent` |

**What the stand-ins cannot show.** That a fork drops the `open` facts and keeps what a rule concluded from them.
The ruling's page took that node by node on the *must*: at the seven nodes of the seats cube that owe nothing,
`(open seats)` is dropped and `(noted seats)`, concluded from it at root, is held at all seven. That is what makes a
reader of an `open` fact unsafe, and why it is flagged whatever it reads.

## 2. The warning's rows, as they would read

Today's rows for a flagged program, from the first stand-in's runs, are the rows the built engine prints for each of
the three; only the site is new. **The site row is composed** in today's form: its kind's name and words are the
build's to choose.

The read-out's first row (`runs/r-held-and-read.stand-in.solve-stdout.txt`):

<!-- output -->
```text
  WARNING           THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER.
                    IT IS NOT CLAIMED TO BE RIGHT, SOUND OR COMPLETE (Q-M1f.564).
                    THE `lattice` ROWS BELOW NAME THE SITES.
```

stderr's first line (`runs/r-held-and-read.stand-in.solve-stderr.txt`), the file's name aside:

<!-- output -->
```text
WARNING: FILE: THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER (Q-M1f.564)
```

the summary's `monotonicity.warning` (`runs/r-held-and-read.stand-in.monotonicity.json`):
`"THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER"`.

The site, today's row for the first stand-in (`runs/r-held-and-read.stand-in.monotone.txt`), and the three rows the
built engine would print in its form, composed:

<!-- output -->
```text
today:     site            unfinished: absent reads debt-paid at absent depth 1, concludes a positive fact — at stake: the state
composed:  site            noted: open-read reads open at absent depth 0, concludes a positive fact — at stake: the state
composed:  site            asked: open-read reads open at absent depth 0, concludes a positive fact — at stake: the state
composed:  site            not-asked: open-read reads open at absent depth 1, concludes a positive fact — at stake: the state
```

No corpus program reads an `open` fact today, and none could
([item 58](../58-the-corpus-under-the-rulings/README.md), § 6), so no corpus program gains these rows; once built
one of the regression witnesses that loads would, `open-derived-activator.ein`.

## 3. Your call

Nothing new is asked: this is your ruling Q-M1f.569 on its own three programs.

- **(a)** As shown: each is flagged by a site of its own kind, and the warning's rows are today's. **Default.**
- **(b)** Something here is not what you meant: say which.
