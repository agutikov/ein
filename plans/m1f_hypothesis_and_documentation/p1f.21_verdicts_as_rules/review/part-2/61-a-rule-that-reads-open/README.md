# 61 · A rule that reads an `open` fact: the program flagged, and the warning's rows

[← part 2](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.12_hypotheses_as_facts.md) ·
[the ruling's page](../../../q-m1f.569_what_may_read_an_open_fact/README.md) · [your answers](../ANSWERS.md)

**What it shows.** Task 1.4 of the stage. You ruled that a rule may read an `open` fact, and that its program is then
not monotone automatically: warned, nothing claimed, no test (Q-M1f.569). Here are the three programs of the ruling's
page in the spelling ruled, their stand-ins as they run today, and the warning's rows as they would read once built.
[On the built engine](#on-the-built-engine), at the foot of the page, shows the rows the final binary prints for
each, beside the composed ones.

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

## On the built engine

**How it was taken.** The final binary, the release build of `7b72bd84` (`build/review-bin/ein-final-7b72bd84`),
with the checkout's `stdlib/`; its `--version` is `runs/built/version.txt`. `tools/run-built.sh` re-takes every run
into `runs/built/` (ignored by git; the stand-in's `runs/` stay as they were). The three targets are run as they
are written, and each is also its stand-in in the built spelling: the two *may* stand-ins with `question` read as
`open` and their `ask` gone (the kernel supposes each `(open F)` root holds itself), the *must*'s with
`(unfinished seats)` gone, since the kernel holds `(open seats)` where the seats owe. So no `.built.ein` file is
written. For each target: the read-out, its stdout and its stderr apart, `-p`, `--monotone`, `ein show hyps`,
`ein saturate --dump`, the oracle with every node's state and `-p`, the events, the summary and its
`monotonicity`, `--dump-lattice` and `--dump-answer`, and the knowledge base, the lattice (as visited and in full)
and the region drawn. `runs/built/warnings.txt` sets the warning's rows of every program side by side. The ruling
page's stand-ins, given to the built engine: the two *may* stand-ins are refused at the parse, for their `hrule`
and `:hrules` (`runs/built/f-held-and-read.stand-in.solve.txt`, `runs/built/f-absent-read.stand-in.solve.txt`);
the *must*'s writes neither, and runs as it ran (`runs/built/r-held-and-read.stand-in.*`).

**Every program is flagged, by the site the page composed.** The three rows the built engine prints are § 2's
composed rows word for word:

<!-- output -->
```text
  site            noted: open-read reads open at absent depth 0, concludes a positive fact — at stake: the state
  site            asked: open-read reads open at absent depth 0, concludes a positive fact — at stake: the state
  site            not-asked: open-read reads open at absent depth 1, concludes a positive fact — at stake: the state
```

(`runs/built/r-held-and-read.target.monotone.txt`, `runs/built/f-held-and-read.target.monotone.txt`,
`runs/built/f-absent-read.target.monotone.txt`; in each summary, a site of kind `open-read`, relation `open`,
`runs/built/*.target.monotonicity.json`). Each program also prints the warning's three rows of § 2, unchanged:
the read-out's `WARNING` row first (`runs/built/*.target.solve-stdout.txt`), stderr's first line
*WARNING: FILE: THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER (Q-M1f.564)*
(`runs/built/*.target.solve-stderr.txt`), and the summary's `monotonicity.warning`,
`"THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER"`. Being flagged, each is answered by the cube,
with no licence, and nothing is claimed of its answer.

| program | the stand-in, on edbe6c3b | the page, once built | the built engine | why they differ |
|---|---|---|---|---|
| the *must*, read | **flagged**: `unfinished: absent reads debt-paid at absent depth 1` (`runs/r-held-and-read.stand-in.monotone.txt`); 2 states | flagged, `noted` reads `open` | **flagged**: `noted: open-read …` at depth 0; 2 states (`runs/built/r-held-and-read.target.solve.txt`) | — |
| a *may*, read | not flagged; 12 states (`runs/f-held-and-read.stand-in.solve.txt`) | flagged, `asked` reads `open` | **flagged**: `asked: open-read …` at depth 0; 12 states (`runs/built/f-held-and-read.target.solve.txt`) | as the page said: the stand-in's `question` facts were facts no commitment changes; an `open` fact is read by a site of its own |
| a *may*'s absence, read | not flagged; 3 states (`runs/f-absent-read.stand-in.solve.txt`) | flagged, `not-asked` reads `open` under `absent` | **flagged**: `not-asked: open-read …` at depth 1; 3 states (`runs/built/f-absent-read.target.solve.txt`) | as for the *may* |
| the warning's three rows | the read-out's `WARNING`, stderr's first line, `monotonicity.warning` (`runs/r-held-and-read.stand-in.*`) | today's, word for word | the same, for each of the three (`runs/built/warnings.txt`) | — |
| the site rows | `unfinished`'s, today's (`runs/r-held-and-read.stand-in.monotone.txt`) | three rows composed | the three composed rows, word for word | — |
| the *must*'s stand-in itself | flagged by `unfinished`; 2 states | — | the same: flagged by `unfinished`; 2 states (`runs/built/r-held-and-read.stand-in.monotone.txt`, `.solve.txt`) | — |
| corpus programs that read `open` | none could (§ 2) | `open-derived-activator.ein` would | **two**: `open-derived-activator.ein`, flagged by `copy [activator open]: open-read reads open at absent depth 0 …`, and `examples/features/23_a_rule_reads_open.ein`, flagged by `noted: open-read …` (`runs/built/corpus.*.monotone.txt`, the warning's rows in `runs/built/warnings.txt`) | the second is the old parse-negative fixture `open_in_match.ein`, moved, default 9 of [item 69](../69-S1f.21.12-defaults/README.md) (Q-M1f.614) |

**What a fork drops, on the *must*.** The page's § 1 said the ruling's page took it node by node: where the seats
owe nothing, `(open seats)` is dropped and `(noted seats)`, concluded from it at root, is held. The built engine
prints it at the two states of $R$, each of which owes nothing: each holds `(noted seats)` and no `open` fact at all
(`runs/built/r-held-and-read.target.solve-p.txt`). The other five nodes that owe nothing are dead, and no command
prints a dead node's state beyond what root holds: the oracle's `--states` keeps a node's facts beyond root's, and
`(noted seats)` is root's (`runs/built/r-held-and-read.target.oracle/region.json`).

**`noted` reads more than the *must*** (V-92, a finding of this stage, in its record). In a `:match`, `(open ?R)` binds
`?R` to the argument of every `open` fact the state holds, what it owes and the hypotheses it holds alike, which is
what the language reference says a rule that reads `open` matches
([`05-rules.md`](../../../../../../docs/ein/user/03-syntax-and-semantics/05-rules.md)). At root the seats hold
`(open seats)` and the four `(open (seats …))` their obligations propose, so `noted` concludes five facts:
`(noted seats)` and `(noted (seats Ann C1))` to `(noted (seats Bob C2))`
(`runs/built/r-held-and-read.target.saturate-dump.txt`), and both states of $R$ hold all five. The stand-in's
`(unfinished seats)` stood for the *must* alone. The program is flagged either way, by the same site.
