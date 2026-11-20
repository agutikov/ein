# 59 · Three programs in the spelling ruled: the seats, *zebra2*, and a hypothesis beside a fact

[← part 2](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.12_hypotheses_as_facts.md) ·
[your answers](../ANSWERS.md)

**What it shows.** Task 1.2 of the stage: three programs written as your rulings write them, each a file today's
parse refuses, and beside each its stand-in, which runs today, `question` standing for `open`. The three now run
on the built engine: [On the built engine](#on-the-built-engine), at the foot of the page, sets each number below
beside what the final binary gives.

**The answer, short.** *zebra2* written with a rule that concludes `(open (?R ?a ?b))` and five activator facts
answers as today on the stand-in, byte for byte: 56 candidates, one state, 261 982 commitments. The seats program
written so answers otherwise, because its obligation's candidates join its rule's (the union): 16 candidates and 8
states where today it has 12 and 4. A rule that concludes a hypothesis and a fact in one `:assert` loads on the
stand-in: root holds the facts, and H is the hypotheses.

## 1. How it was taken

Today's engine, `build/review-bin/ein-head-edbe6c3b`, with the checkout's `stdlib/`. `tools/run.sh` re-takes every
run into `runs/` (ignored by git; mirrored under `build/review-incoming/p1f21-s12-examples/59-three-programs-as-ruled/`):
each target's refusal, and for each stand-in and each corpus program its read-out, `-p`, `--single`, `ein show hyps`,
`ein saturate --dump`, the events, the summary and the knowledge base drawn; the oracle where H has 16 candidates or
fewer. The seats stand-in's events are left out: its cube is described node by node, and the stream is hundreds of
MB. `tools/gen.py` wrote the *zebra2* and seats files from the corpus files.

**What stands for what.** In each stand-in, `question` stands for `open`: the rule concludes `(question F)` where the
spelling ruled concludes `(open F)`, and one hypothesis rule, `ask`, supposes what is asked, which is what the kernel
would do itself, $H$ being root's `open` facts (Q-M1f.573). What the stand-in cannot show: that a fork drops the
`open` facts, since a fact a rule concludes at root is in every state above it; item
[62](../62-what-a-fork-drops/README.md) takes that node by node.

| | the spelling ruled (refused today) | the stand-in | today's program |
|---|---|---|---|
| *zebra2* | [`zebra2.target.ein`](programs/zebra2.target.ein) | [`zebra2.asked.ein`](programs/zebra2.asked.ein) | [`examples/zebra2.ein`](../../../../../../examples/zebra2.ein) |
| the seats | [`seats.target.ein`](programs/seats.target.ein) | [`seats.asked.ein`](programs/seats.asked.ein) | [`examples/lattice/14_seats_nothing_proposes.ein`](../../../../../../examples/lattice/14_seats_nothing_proposes.ein) |
| a hypothesis beside a fact | [`with-a-fact.target.ein`](programs/with-a-fact.target.ein) | [`with-a-fact.asked.ein`](programs/with-a-fact.asked.ein) | |

Today's refusals (`runs/*.target.solve.txt`): *zebra2* and the seats at the `(` after `open` in their `:assert`; the
third at the `open` itself, with today's note, *open is reserved: it is the verdict a rule's :assert states*.

## 2. *zebra2*

What changes in the file, and nothing else does:

```lisp
(rule guess (?R ?T1 ?T2)                    ; was (hrule guess …)
  :match  (and (is-a ?a ?T1) (is-a ?b ?T2))
  :assert (open (?R ?a ?b))                 ; was (?R ?a ?b)
  :why    "guess: ({?R} {?a} {?b})?")
(guess color-loc Color House)               ; were the (solve :hrules (guess …)) clause,
(guess nation-loc Nationality House)        ; which goes with the (solve …) it stood alone in
(guess drink-loc Drink House)
(guess smoke-loc Cigarette House)
(guess pet-loc Pet House)
```

| | candidates | states of $R$ | commitments | `--single` |
|---|---:|---:|---:|---|
| `examples/zebra2.ein` | 56 | 1 | 261 982 | 7 saturations |
| `zebra2.asked.ein` | 56 | 1 | 261 982 | 7 saturations |

`ein solve`, `--single` and `ein show hyps` print the same, the file's name apart (`runs/zebra2-pair.diff.txt`). Its
obligations, from `(bijective *-loc)`, propose nothing the rule does not, so the union is the rule's 56
([item 58](../58-the-corpus-under-the-rulings/README.md), § 2).

**What a state holds.** The rule reads only `is-a`, so it concludes the same 125 `(open (?R ?a ?b))` at every
state: the 56 candidates and the 69 facts root decides. Once built the one state of $R$ lists them beside its other
facts (the stand-in's `-p` lists 125 `question` facts in it); a state that supposes F holds both F and
`(open F)` ([the page of Q-M1f.573](../../../q-m1f.573_obligation_or_hypothesis/README.md), § 2).

## 3. The seats

`seat` is the program's hypothesis rule, written as a rule that concludes `(open (sits ?p ?s))`; the obligation
`(total-owed sits is-a)` stays as it is. Once built both write at root, and $H$ is the two together (Q-M1f.570). The
stand-in has a second rule, `owed-seat`, which concludes under `question` what the obligation's guard gives at
root.

| | candidates | states of $R$ | commitments | its claim |
|---|---:|---:|---:|---|
| `14_seats_nothing_proposes.ein` | 12 | 4 | 4 | holds |
| `seats.asked.ein`, the union | 16 | 8 | 8 | none stated: the target drops it, the union refutes it |

The four seats it adds are Cat's and Dan's at the small table, which the rule's guard left out; $R$ gains the four
seatings with the pair at the big table and the duo at the small one. The same numbers as item 58's union of this
program, taken another way.

**The stand-in is flagged and the program would not be.** `owed-seat` reads the absence of a seat, which a
commitment brings, so today's static check flags the stand-in (`runs/seats.asked.solve.txt`: one site, `owed-seat`)
and describes its cube in 65 536 saturations. The obligation once built is no rule; its guard reads `is-a`, which no
commitment changes, and item 58's union of the same program, which proposes the same 16 by plain hypothesis rules,
is not flagged.

## 4. A hypothesis beside a fact

Ruled (Q-M1f.575 (a)): `(open R)` is a rule's whole `:assert`; `(open F)` stands in a conjunction like any fact.

```lisp
(rule propose ()
  :match  (thing ?x)
  :assert (and (open (p ?x)) (seen ?x))
  :why    "{?x} is seen, and may be a p.")
```

On the stand-in root holds `(seen A)`, `(seen B)` and the two `question` facts; $H$ is `(p A)` `(p B)`; $R$ is the
four states over them. The form that stays refused, `(and (open ?R) (r ?a b))`, is the corpus's
`examples/broken/open_mixed_assert.ein`.

## 5. Your call

Nothing here is new: the three files are your rulings written out.

- **(a)** The spelling stands as written here; S1f.21.18 writes the corpus's hypothesis rules this way. **Default.**
- **(b)** Something in it is not what you meant: say which.

## On the built engine

**How it was taken.** The final binary, the release build of `7b72bd84` (`build/review-bin/ein-final-7b72bd84`),
with the checkout's `stdlib/`; its `--version` is `runs/built/version.txt`. `tools/run-built.sh` re-takes every run
into `runs/built/` (ignored by git, as `runs/` is; the stand-in's runs stay where they are). The three targets are
run as they are written: each is also its stand-in in the built spelling (`question` read as `open`, the stand-in's
`ask` gone, since the kernel supposes each `(open F)` root holds itself, and the `hrule` form and `:hrules` gone), so
no `.built.ein` file is written. For each target and each corpus program the page compares with: the read-out,
`-p`, `--single`, `ein show hyps`, `ein saturate --dump`, the events and the summary, `--dump-lattice` and
`--dump-answer` (with `-p`, so every state's facts are kept uncut), and the knowledge base and the lattice drawn.
Where $H$ has 16 candidates or fewer (the seats, the hypothesis beside a fact) also the oracle with every node's
state, the lattice in full and the region drawn; *zebra2*'s 56 candidates are past the oracle's limit, so it has
neither. Each stand-in, given to the built engine, is refused at the parse (`runs/built/*.asked.solve.txt`: the
`hrule` form and `:hrules` are gone).

| | the stand-in, on edbe6c3b | the built engine | why they differ |
|---|---|---|---|
| *zebra2*: candidates | 56 (`runs/zebra2.asked.solve.txt`) | 56, $H$ from `union` (`runs/built/zebra2.target.show-hyps.txt`) | — |
| *zebra2*: states of $R$, commitments | 1, 261 982 | 1, 261 982 (`runs/built/zebra2.target.solve.txt`) | — |
| *zebra2*: `--single` | 7 saturations (`runs/zebra2.asked.solve-single.txt`) | 7 saturations (`runs/built/zebra2.target.solve-single.txt`) | — |
| *zebra2*: against the corpus's `zebra2.ein` | the same, the file's name apart (`runs/zebra2-pair.diff.txt`) | the same, the file's name apart, `-p` too (`runs/built/zebra2-pair.diff.txt`) | — |
| *zebra2*: `open` facts the one state of $R$ lists | 125 `question` facts (`runs/zebra2.asked.solve-p.txt`) | 125 `(open …)` (`runs/built/zebra2.target.solve-p.txt`; `runs/built/open-per-state.txt`) | — |
| *zebra2*: `open` facts root holds | — | 130: the 125 and five `(open R)`, one for each `*-loc` relation root owes (`runs/built/zebra2.target.saturate-dump.txt`) | a state of $R$ owes nothing, so it holds no `(open R)` |
| the seats: candidates | 16 (`runs/seats.asked.solve.txt`) | 16, $H$ from `union` (`runs/built/seats.target.show-hyps.txt`) | — |
| the seats: states of $R$, commitments | 8, 8 | 8, 8 (`runs/built/seats.target.solve.txt`) | — |
| the seats: the static check | **flagged**, one site, `owed-seat` | **not flagged**: `monotone (the static check)` (`runs/built/seats.target.solve.txt`) | as § 3 said: the obligation is no rule, and its guard reads `is-a` |
| the seats: the work of describing $R$ | 65 536 saturations, the cube (`runs/seats.asked.solve.txt`) | 149 saturations (`runs/built/seats.target.solve.txt`) | the program is not flagged, so the tree and the classes are licensed; the oracle's cube is still 65 536 (`runs/built/seats.target.oracle.txt`) |
| the seats: `open` facts a state of $R$ lists | 16 `question` facts in state 1 (`runs/seats.asked.solve-p.txt`) | 12 in each of the 8, `seat`'s (`runs/built/open-per-state.txt`) | a fork drops every `open` fact and the pass writes them again (Q-M1f.566): the rule's 12 are written at every state, the obligation's only where something is owed, and a state of $R$ owes nothing |
| the seats: `open` facts root holds | — | 17: `(open sits)`, 12 by `seat` and 4 by `total-owed`, Cat's and Dan's at the small table (`runs/built/seats.target.saturate-dump.txt`) | — |
| `14_seats_nothing_proposes.ein` | 12 candidates, 4 states, 4 commitments, its claim holds (`runs/seats.solve.txt`) | 16, 8, 8, and its claim, now the 8 states, holds (`runs/built/seats.solve.txt`) | the corpus file is written in the spelling ruled (S1f.21.18): it is the target with its comments and an `:expect` of the 8 states |
| a hypothesis beside a fact: root | `(seen A)` `(seen B)` and two `question` facts (`runs/with-a-fact.asked.saturate-dump.txt`) | `(seen A)` `(seen B)` `(open (p A))` `(open (p B))` (`runs/built/with-a-fact.target.saturate-dump.txt`) | — |
| a hypothesis beside a fact: $H$, $R$ | `(p A)` `(p B)`; 4 states (`runs/with-a-fact.asked.solve.txt`) | `(p A)` `(p B)`, from `hypothesis-rules` (`runs/built/with-a-fact.target.show-hyps.txt`); 4 states, each listing the two `open` facts (`runs/built/with-a-fact.target.solve-p.txt`) | — |
| the form that stays refused | `examples/broken/open_mixed_assert.ein` | `examples/broken/load/open_mixed_assert.ein`, refused at load: *`(open ?R)` stands beside another conclusion* (`runs/built/open_mixed_assert.solve.txt`) | moved, default 9 of [item 69](../69-S1f.21.12-defaults/README.md) (Q-M1f.614) |

**What a state holds, built.** As § 2 said, *zebra2*'s one state lists its 125 `open` facts beside its other facts,
and a fact it supposes beside its own `open` fact: `(color-loc Blue House-2)` and `(open (color-loc Blue House-2))`
(`runs/built/zebra2.target.solve-p.txt`). That holds of a hypothesis a **rule** writes, since the rule concludes it
again at every state. It does not hold of one only the **obligation** proposed: the seats' state 1 holds
`(sits Cat S1)` and no `(open (sits Cat S1))` (`runs/built/seats.target.dump-answer/forms/print-final-state.txt`),
because the fork dropped it and the obligation, owed nothing there, writes nothing; [item 60](../60-a-state-with-its-open-facts/README.md)
shows the same on its seats. So the sentence of § 2, *a state that supposes F holds both*, is true of a rule's
hypotheses, not of every hypothesis.

**Where the `open` facts of a state above root are printed.** `ein solve -p` and `ein oracle -p` list them
(`runs/built/seats.target.oracle.txt`); the oracle's `--states` does not, since it keeps a node's facts beyond
root's and root holds the same `open` facts (`runs/built/with-a-fact.target.oracle/region.json`: the state of
`{(p A)}` is `(p A)` alone).
