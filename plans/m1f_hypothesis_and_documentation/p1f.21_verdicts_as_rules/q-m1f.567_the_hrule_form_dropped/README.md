# 45 · Q-M1f.567: the `hrule` form dropped for a rule that concludes `(open F)`

[← part 2](../review/part-2/README.md) · [← the phase](../README.md) ·
[the ledger](../../open_questions.md#q-m1f567--the-hrule-form-dropped-for-a-rule-that-concludes-open-f) ·
[the analysis](../open_and_owe.md#4-can-the-hrule-form-go)

**Ruled 2026-11-19, and out of the review bundle**, where it was item 45 of part 2. Your answer, in the part's
answers' file:

> I already decided that all mechanisms that work with hypotheses - work with facts under (open ...) constext. The
> hrule form initially was producing hypotheses - now it can do any rule that assert open facts. So there is no
> more need for :hrules special place - we just activate hypotheses generating rules anywhere. The special form
> (hrule ...) is not required anymore so we remove it and keyword :hrules.

It is filed under the question in the ledger and in [the phase's README](../README.md#the-answers-to-items-44-45-and-46).
What it leaves open is review items 48 and 51 ([part 2 of the bundle](../review/part-2/README.md)). The page is kept, with its
programs, as the survey the second part's stages cite; where it says *not ruled* and *the default*, read it
as it stood before the answer. Its outputs and its tools (`runs/` and `tools/` below) are on this machine under
`build/review-archive/p1f21-part-2/45-Q-M1f.567/`, as they were run.

**Raised** by your answer to item 34, 2026-11-19: *"If we will emit (open H) facts as pure hypotheses, and
obligations would be emitted different way, can we then drop hrules as special form according to
plans/m1f_hypothesis_and_documentation/p1f.21_verdicts_as_rules/one_way_each.md and use rules that emit open facts
in :hrules?"*

**Not ruled. The default is (b).** It revises your ruling of the same day on Q-M1f.561, *"we leave all existing
options to control hypotheses as is"*, which is why it is a question. Nothing is built: every run below is today's
engine, the gate build of master at `edbe6c3b` (`build/review-bin/ein-head-edbe6c3b`), on the programs in
[`programs/`](programs/). `tools/run.sh` re-takes them all into `runs/` (both ignored, so on this machine only); a
path below written as code is a file there.

## 1. The options, as each is written

Two drinks; each may be chosen and may be served; only one is chosen.

| | the rule | its activators | the program | today |
|---|---|---|---|---|
| **(a)** | `(hrule guess (?R ?T) … :assert (?R ?x))` | in `(solve :hrules …)` | [`a-drinks-hrule.ein`](programs/a-drinks-hrule.ein) | runs: it is today |
| **(b)** | `(rule guess (?R ?T) … :assert (open (?R ?x)))` | in `(solve :hrules ((guess chosen Drink) (guess served Drink)))` | [`b-drinks-open.target.ein`](programs/b-drinks-open.target.ein) | refused at the parse: `open` over a fact |
| **(c)** | the same rule | ordinary facts, `(guess chosen Drink)`; the clause is gone | [`c-drinks-open.target.ein`](programs/c-drinks-open.target.ein) | refused at the parse, the same way |

The two `.target.ein` files are the spelling asked about, with the clause as you ruled it of Q-M1f.565. Their
refusal today is `runs/b-drinks-open.target.solve.txt`.

## 2. It runs today, with another word for `open`

[`bc-drinks-asked.ein`](programs/bc-drinks-asked.ein) is (b) and (c) on today's engine. `guess` is an ordinary
rule; it concludes each candidate as a held fact under `question`, which stands for `open`; and one hypothesis rule
supposes what is asked, which is the part the kernel would do itself, *$H$ is the facts root holds under `open`*.

```lisp
(rule guess (?R ?T)
  :match  (is-a ?x ?T)
  :assert (question (?R ?x))
  :why    "guess: is {?x} {?R}?")
(guess chosen Drink) (guess served Drink)
```

| pair | `ein solve` | `ein show hyps` | `ein oracle` | `ein solve --single` |
|---|---|---|---|---|
| `a-drinks-hrule` against `bc-drinks-asked` | the same | the same | the same | |
| `examples/zebra2.ein` against the same with its `hrule` and its clause replaced ([the blocks](programs/zebra2-asked.blocks.ein), `runs/zebra2-asked.ein`) | the same: 56 candidates, 1 state, 261 982 commitments, 120 saturations | the same | | the same: 7 saturations |

*The same* is byte for byte, the file's name apart (`runs/pairs.diff.txt`). What differs is what a state holds:
`-p` lists the `(question …)` facts in every state, four here and 125 in *zebra2*.

## 3. What the form does today that a rule would not

**A rule fires at every node; a hypothesis rule is read once, at root.** The pair
[`above-root-hrule.ein`](programs/above-root-hrule.ein) and [`above-root-asked.ein`](programs/above-root-asked.ein)
has a second rule whose premise only a commitment brings: *a drink is chosen: does it come with sugar?*
(`runs/above-root.states.txt`).

| | $H$ | the state of `{(chosen Tea)}` beyond root |
|---|---|---|
| as hypothesis rules | `(chosen Milk)` `(chosen Tea)` | `(chosen Tea)` |
| as rules that conclude what is asked | `(chosen Milk)` `(chosen Tea)`, the same | `(chosen Tea)` **`(question (with Tea Sugar))`** |

$R$ is the same four commitments. As rules, a state above root holds a question that was never proposed: a
hypothesis that depends on another, which you parked
([V-21](../findings.md#v-21--dependent-hypotheses-a-candidate-proposed-above-root)). Under (b) and (c) the
reading is: **$H$ is root's `open` facts, and an `(open F)` first concluded above root is held and not proposed.**

The other differences ([the analysis](../open_and_owe.md#4-can-the-hrule-form-go) has the table): a rule may
conclude any conjunction, where a hypothesis rule concludes one fact; the ladder's first rung becomes *root holds a
fact of `open`*; and the obligation's verdict needs another word than `open`, which is item
[44](../q-m1f.566_how_the_must_is_emitted/README.md).

## 4. One thing the stand-in met

[`above-root-asked.generic.ein`](programs/above-root-asked.generic.ein) writes the supposing rule once for any
relation, a rule a count of arguments. **It is refused at load**
(`runs/above-root-asked.generic.solve.txt`):

> kb load error: hrule 'ask-1': its :assert writes (?R ?x), whose ?x is rule 'ask-2''s parameter ?R, which names a
> relation, and (question (?R ?x)) can bind ?x to a fact, as an argument of (?R ?x) — a relation is named by a
> symbol, not a fact

That is the loader's check on what a rule could derive, the one item 38 was about and S1f.21.17's first task turns
into a warning if you confirm that reading. It bears on the stand-in only: under (b) and (c) no program writes the
supposing rule.

## 5. The default, and why

**(b)**: the form goes and the clause stays, which is the question as you put it. A hypothesis rule is then a rule
like any other, its candidates are facts a rule can read, and what a program supposes is still stated in one
place, its `(solve …)`. **(c)** removes the clause too: fewer forms, and an activator is a fact a rule could
conclude, so what is supposed can depend on what is derived. **(a)** keeps your ruling of Q-M1f.561 as it stands.

Under (b) or (c) every program with an `hrule` is written again: 52 files under `examples/`, 4 under `tests/`, 1 in
`stdlib/`, by S1f.21.16's count of the files that state the clause.

## 6. Files

- [`programs/`](programs/): the seven programs above, and the two blocks that replace *zebra2*'s.
- `runs/`: for each program its read-out (`.solve.txt`, `.solve-p.txt`), `ein show hyps`, `ein saturate --dump`,
  the events and the summary; for the four that load and search, the oracle's read-out and its dump with every
  node's state, the answer and the lattice as files, and the knowledge base drawn (`.kb.dot`, `.kb.svg`);
  `zebra2.ein` and `zebra2-asked.ein` with their runs; `pairs.diff.txt`; `above-root.states.txt`.
- `tools/run.sh`.

## 7. The ruling

Ruled: the answer at the head of this page.
