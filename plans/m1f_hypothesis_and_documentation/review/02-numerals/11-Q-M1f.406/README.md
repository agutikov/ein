# Q-M1f.406 — the label, and where a number an operation makes gets it

## 1. The question

The ruling says *"numbres produced by operations has to be labelled valid already"*. As built, the label is a fact,
`(valid N)` (`std.math.decimal`'s) or `(bin-valid N)` (`std.math.binary`'s). It is derived in three places: where
the check's walk of an operand reaches `z` (`numeral-valid`); where an arithmetic walk finishes a number it made
(`std.math.positional`'s `turn-end` for every addition, subtraction and product walk, a tagged call's included, and
`div-end` for a quotient and its remainder); and `z` by the module's closing rule. The question is where a made number
should get its label: as the walk that makes it ends (default), only on the public answers (`sum-of`'s S and the
like), and whether `z` should be labelled unconditionally.

Recorded in [`../../open_questions.md` § Q-M1f.406](../../../open_questions.md#q-m1f406--the-label-and-where-a-number-an-operation-makes-gets-it);
the build is S1f.22.6 (`bf9693dc`), whose record is read from git
(`git show b5fb8d70^:plans/m1f_hypothesis_and_documentation/p1f.22_nested_facts/s1f.22.6_written_erased.md`,
§ *The design, as built*).

## 2. What is so today

Runs under **today** = `build/review-bin/ein-head-17085b27`, and, where the program names no `valid`, under
**before** = `build/review-bin/ein-0a4a97a3` with `EIN_STDLIB=build/wt-rev-0a4/stdlib`, which had no label (the
diffs show where the labels appeared and the `written` facts went).

**An addition and a division** — [`programs/add_and_div.ein`](programs/add_and_div.ein), 47 + 8 and 47 / 5. Every
`valid` fact today ([facts](runs/add_and_div.today.saturate-facts.txt)), by the rule that derived it:

<!-- output -->
```
  (std.math.decimal.valid z :rule std.math.decimal.decimal)
  (std.math.decimal.valid (n (n z 4) 7) :rule std.math.decimal.numeral-valid)
  (std.math.decimal.valid (n z 8) :rule std.math.decimal.numeral-valid)
  (std.math.decimal.valid (n z 5) :rule std.math.decimal.numeral-valid)
  (std.math.decimal.valid (n (n z 5) 5) :rule std.math.positional.turn-end)
  (std.math.decimal.valid (n (n z 1) 0) :rule std.math.positional.turn-end)
  …                                     (15, 20, 25, 30, 35, 40, 45, 22, 12, 7, by turn-end)
  (std.math.decimal.valid (n z 2) :rule std.math.positional.turn-end)
  (std.math.decimal.valid (n z 9) :rule std.math.positional.div-end)
```
- the three operands where their walks end (`numeral-valid`);
- the sum 55 as `turn-end` hands it over;
- every number the division makes on the way — the multiples of 5 it tries, 10 to 45, and the trial differences
  22, 12, 7, 2 — by `turn-end`;
- the quotient 9 by `div-end`, which labels the remainder 2 too (held already, from the walk that made it): trace
  step 148 in [`runs/add_and_div.today.trace-label-steps.md`](runs/add_and_div.today.trace-label-steps.md), the
  excerpt of every step that asks, walks or labels;
- `z` by the closing rule.

Before, the same program's knowledge base held `(written N)` for the four numerals the file writes and no label
([facts diff](runs/add_and_div.saturate-facts.diff)); the answers are the same ([read-out diff](runs/add_and_div.solve.diff):
no difference).

**The smallest, drawn whole** — [`programs/small_add.ein`](programs/small_add.ein), 4 + 3, today only (it lists
`valid` and queries its extent): `(valid z)`, `(valid (n z 3))`, `(valid (n z 4))`, `(valid (n z 7))`
([read-out](runs/small_add.today.solve.txt), `query 2 of 2`). Its KB view, `ein render kb`, drawn whole with `fdp`:
[`renders/small_add.today.kb.dot`](renders/small_add.today.kb.dot) →
[`renders/small_add.today.kb.svg`](renders/small_add.today.kb.svg) (5.8 MB: the view is mostly the digit tables the
import brings, 12 917 lines of DOT and 10 818 arrows, and `fdp` took 425 s on it, on a machine another run was
loading). The same view cut to the check's and the label's facts and what they nest (`tools/kbsub.py`, each line the
binary's own): [`renders/small_add.today.kb-cut.svg`](renders/small_add.today.kb-cut.svg). In the view a label of a
numeral is a fact of `valid` drawn as the graph has it: an octagon with no name in it, its arrow `1` at the box
`valid` and its arrow `2` at the numeral's octagon, its `:rule` beside it (`by std.math.decimal.numeral-valid` for 3
and 4, `by std.math.positional.turn-end` for 7). `(valid z)` is drawn the same way, its arrow `2` at the box `z`,
`by std.math.decimal.decimal` beside it; the first look's binary drew it as a loop on `z`. The numerals, facts the
knowledge base does not hold, stand in frames named for the relations whose facts point at them.

![small_add's KB cut today: the four facts of valid, octagons whose arrow 1 is at the box valid, three with arrow 2 into a numeral's octagon in a frame and one at the box z; the add request and the walk's facts around them](renders/small_add.today.kb-cut.svg)

**A tagged call's hand-over** — [`programs/tagged_call.ein`](programs/tagged_call.ein), `(add-walk (mine a) (n z 4)
(n z 3) 0 z)`: today `(done (mine a) (n z 7) :rule turn-end)` and `(valid (n z 7) :rule turn-end)`, the hand-over
labelled though no public answer reads it ([facts](runs/tagged_call.today.saturate-facts.txt)). The operands 4 and 3
are not walked: a call looks nothing up (see `../04-Q-M1f.405/`).

**A zero** — [`programs/zero.ein`](programs/zero.ein), 5 − 5: zero is handed over by `strip-none`, not by a turn, so
no walk labels it; `(valid z)` is held by the closing rule ([facts](runs/zero.today.saturate-facts.txt)). Every
program that imports the module holds `(valid z)` the same way, one that makes no zero included
(`tagged_call`, `small_add`).

**Binary** — [`programs/binary_add.ein`](programs/binary_add.ein), 10 + 1 in base two: `(bin-valid z :rule
std.math.binary.binary)`, the operands by `bin-numeral-valid`, the sum `(b2 (b2 z 1) 1)` by
`std.math.positional.turn-end` — the same rule, with binary's names ([facts](runs/binary_add.today.saturate-facts.txt)).

## 3. The options

- **(a) `(valid N)` / `(bin-valid N)`, a fact of the numeral**: an operand's at its walk's end; a made number's as the
  firing that makes it, `turn-end` and `div-end`, a tagged call's hand-over included; `z` by the closing rule. Every
  number any operation answers with is valid already. **What runs today** (above).
- **(b) the public answers alone**, each module's answer rules labelling what they hold: `turn-end` and `div-end`
  label nothing, a tagged call's hand-over is unlabelled, and `std.math.int` and `std.math.rat` need fourteen answer
  rules written. Mocks: [`runs/add_and_div.option-b.MOCK.saturate-facts.txt`](runs/add_and_div.option-b.MOCK.saturate-facts.txt)
  (the fourteen labels `turn-end` and `div-end` derive deleted, three added by `add-answer` and `div-answer`: 55, 9
  and 2) and [`runs/tagged_call.option-b.MOCK.saturate-facts.txt`](runs/tagged_call.option-b.MOCK.saturate-facts.txt)
  (the hand-over's label deleted). No answer changes; what changes is which facts a later theory can find labelled.
- **(c) as (a), with `z` labelled only where a walk makes it or a check reaches it.** Mock:
  [`runs/zero.option-c.MOCK.saturate-facts.txt`](runs/zero.option-c.MOCK.saturate-facts.txt) (the closing rule's
  `(valid z)` replaced by one `strip-none` would derive beside the hand-over of zero). In every other program of this
  folder, which makes no zero, `(valid z)` would simply be gone.

## 4. The default

**(a)**, built by S1f.22.6 (`bf9693dc`), stated by Q-M1f.404's decision. From the record: an earlier draft labelled
the public answers (the five answer rules), and the build moved the label to the end of the walk that makes the
number (`turn-end`, `div-end`) so that every number every module over the bases answers with, a tagged call's
included, is valid as it is made, with no answer rule of `int` or `rat` to write; and `z` is labelled by the closing
rule (a zero is handed over by `strip-none`, which labels nothing: `zero` above).

## 5. Files

Programs: all five written for this review, each claiming the answer under both binaries (`small_add` runs today
only).

Per program and binary: *read-out* `ein solve`; *summary* `--json-summary`; *events* `--events` (JSONL) and its
*events table*; *trace* `--trace`; *dump-answer* `--dump-answer DIR`; *saturate --dump* whole and *its facts* (the
facts that bear on the check and the label, `tools/facts.py`); `ein test`; `ein show hyps`; *kb-cut* `ein render kb`
cut to the facts of `check valid bin-check bin-valid written add sum-of div quot-of sub diff-of add-walk done bin-add
bin-sum-of div-done` and what they nest, drawn with `fdp` (`tools/kbsub.py`); for `small_add` also *kb.dot* and
*kb.svg*, the whole view. Each text output ends with `[exit N]`.

*Today*'s kb-cut is drawn as the graph has it: a fact an octagon with no name in it, its arrow `1` at its relation's
box and `2`, `3`, … at its arguments, what it rests on (`by RULE`) beside it, and a fact under a fact's arrow that the
knowledge base does not hold in the frame of what points at it (`add ctx`); *before*'s is the earlier binary's
drawing, labelled octagons and labelled arrows. `tools/kbsub.py` reads both forms.

| program | binary | artefacts |
|---|---|---|
| [`small_add`](programs/small_add.ein) | today | [read-out](runs/small_add.today.solve.txt) · [summary](runs/small_add.today.summary.json) · [events](runs/small_add.today.events.jsonl) · [events table](runs/small_add.today.events.md) · [trace](runs/small_add.today.trace.md) · [dump-answer](runs/small_add.today.answer/) · [saturate --dump](runs/small_add.today.saturate-dump.txt) · [its facts](runs/small_add.today.saturate-facts.txt) · [ein test](runs/small_add.today.test.txt) · [show hyps](runs/small_add.today.show-hyps.txt) · [kb-cut.dot](renders/small_add.today.kb-cut.dot) · [kb-cut.svg](renders/small_add.today.kb-cut.svg) · [kb.dot](renders/small_add.today.kb.dot) · [kb.svg](renders/small_add.today.kb.svg) |
| [`add_and_div`](programs/add_and_div.ein) | before | [read-out](runs/add_and_div.before.solve.txt) · [summary](runs/add_and_div.before.summary.json) · [events](runs/add_and_div.before.events.jsonl) · [events table](runs/add_and_div.before.events.md) · [trace](runs/add_and_div.before.trace.md) · [dump-answer](runs/add_and_div.before.answer/) · [saturate --dump](runs/add_and_div.before.saturate-dump.txt) · [its facts](runs/add_and_div.before.saturate-facts.txt) · [ein test](runs/add_and_div.before.test.txt) · [show hyps](runs/add_and_div.before.show-hyps.txt) · [kb-cut.dot](renders/add_and_div.before.kb-cut.dot) · [kb-cut.svg](renders/add_and_div.before.kb-cut.svg) |
| [`add_and_div`](programs/add_and_div.ein) | today | [read-out](runs/add_and_div.today.solve.txt) · [summary](runs/add_and_div.today.summary.json) · [events](runs/add_and_div.today.events.jsonl) · [events table](runs/add_and_div.today.events.md) · [trace](runs/add_and_div.today.trace.md) · [dump-answer](runs/add_and_div.today.answer/) · [saturate --dump](runs/add_and_div.today.saturate-dump.txt) · [its facts](runs/add_and_div.today.saturate-facts.txt) · [ein test](runs/add_and_div.today.test.txt) · [show hyps](runs/add_and_div.today.show-hyps.txt) · [kb-cut.dot](renders/add_and_div.today.kb-cut.dot) · [kb-cut.svg](renders/add_and_div.today.kb-cut.svg) |
| [`add_and_div`](programs/add_and_div.ein) | diff | [saturate-facts.diff](runs/add_and_div.saturate-facts.diff) · [solve.diff](runs/add_and_div.solve.diff) · [test.diff](runs/add_and_div.test.diff) · [trace-steps.diff](runs/add_and_div.trace-steps.diff) |
| [`tagged_call`](programs/tagged_call.ein) | before | [read-out](runs/tagged_call.before.solve.txt) · [summary](runs/tagged_call.before.summary.json) · [events](runs/tagged_call.before.events.jsonl) · [events table](runs/tagged_call.before.events.md) · [trace](runs/tagged_call.before.trace.md) · [dump-answer](runs/tagged_call.before.answer/) · [saturate --dump](runs/tagged_call.before.saturate-dump.txt) · [its facts](runs/tagged_call.before.saturate-facts.txt) · [ein test](runs/tagged_call.before.test.txt) · [show hyps](runs/tagged_call.before.show-hyps.txt) · [kb-cut.dot](renders/tagged_call.before.kb-cut.dot) · [kb-cut.svg](renders/tagged_call.before.kb-cut.svg) |
| [`tagged_call`](programs/tagged_call.ein) | today | [read-out](runs/tagged_call.today.solve.txt) · [summary](runs/tagged_call.today.summary.json) · [events](runs/tagged_call.today.events.jsonl) · [events table](runs/tagged_call.today.events.md) · [trace](runs/tagged_call.today.trace.md) · [dump-answer](runs/tagged_call.today.answer/) · [saturate --dump](runs/tagged_call.today.saturate-dump.txt) · [its facts](runs/tagged_call.today.saturate-facts.txt) · [ein test](runs/tagged_call.today.test.txt) · [show hyps](runs/tagged_call.today.show-hyps.txt) · [kb-cut.dot](renders/tagged_call.today.kb-cut.dot) · [kb-cut.svg](renders/tagged_call.today.kb-cut.svg) |
| [`tagged_call`](programs/tagged_call.ein) | diff | [saturate-facts.diff](runs/tagged_call.saturate-facts.diff) · [solve.diff](runs/tagged_call.solve.diff) · [test.diff](runs/tagged_call.test.diff) · [trace-steps.diff](runs/tagged_call.trace-steps.diff) |
| [`zero`](programs/zero.ein) | before | [read-out](runs/zero.before.solve.txt) · [summary](runs/zero.before.summary.json) · [events](runs/zero.before.events.jsonl) · [events table](runs/zero.before.events.md) · [trace](runs/zero.before.trace.md) · [dump-answer](runs/zero.before.answer/) · [saturate --dump](runs/zero.before.saturate-dump.txt) · [its facts](runs/zero.before.saturate-facts.txt) · [ein test](runs/zero.before.test.txt) · [show hyps](runs/zero.before.show-hyps.txt) · [kb-cut.dot](renders/zero.before.kb-cut.dot) · [kb-cut.svg](renders/zero.before.kb-cut.svg) |
| [`zero`](programs/zero.ein) | today | [read-out](runs/zero.today.solve.txt) · [summary](runs/zero.today.summary.json) · [events](runs/zero.today.events.jsonl) · [events table](runs/zero.today.events.md) · [trace](runs/zero.today.trace.md) · [dump-answer](runs/zero.today.answer/) · [saturate --dump](runs/zero.today.saturate-dump.txt) · [its facts](runs/zero.today.saturate-facts.txt) · [ein test](runs/zero.today.test.txt) · [show hyps](runs/zero.today.show-hyps.txt) · [kb-cut.dot](renders/zero.today.kb-cut.dot) · [kb-cut.svg](renders/zero.today.kb-cut.svg) |
| [`zero`](programs/zero.ein) | diff | [saturate-facts.diff](runs/zero.saturate-facts.diff) · [solve.diff](runs/zero.solve.diff) · [test.diff](runs/zero.test.diff) · [trace-steps.diff](runs/zero.trace-steps.diff) |
| [`binary_add`](programs/binary_add.ein) | before | [read-out](runs/binary_add.before.solve.txt) · [summary](runs/binary_add.before.summary.json) · [events](runs/binary_add.before.events.jsonl) · [events table](runs/binary_add.before.events.md) · [trace](runs/binary_add.before.trace.md) · [dump-answer](runs/binary_add.before.answer/) · [saturate --dump](runs/binary_add.before.saturate-dump.txt) · [its facts](runs/binary_add.before.saturate-facts.txt) · [ein test](runs/binary_add.before.test.txt) · [show hyps](runs/binary_add.before.show-hyps.txt) · [kb-cut.dot](renders/binary_add.before.kb-cut.dot) · [kb-cut.svg](renders/binary_add.before.kb-cut.svg) |
| [`binary_add`](programs/binary_add.ein) | today | [read-out](runs/binary_add.today.solve.txt) · [summary](runs/binary_add.today.summary.json) · [events](runs/binary_add.today.events.jsonl) · [events table](runs/binary_add.today.events.md) · [trace](runs/binary_add.today.trace.md) · [dump-answer](runs/binary_add.today.answer/) · [saturate --dump](runs/binary_add.today.saturate-dump.txt) · [its facts](runs/binary_add.today.saturate-facts.txt) · [ein test](runs/binary_add.today.test.txt) · [show hyps](runs/binary_add.today.show-hyps.txt) · [kb-cut.dot](renders/binary_add.today.kb-cut.dot) · [kb-cut.svg](renders/binary_add.today.kb-cut.svg) |
| [`binary_add`](programs/binary_add.ein) | diff | [saturate-facts.diff](runs/binary_add.saturate-facts.diff) · [solve.diff](runs/binary_add.solve.diff) · [test.diff](runs/binary_add.test.diff) · [trace-steps.diff](runs/binary_add.trace-steps.diff) |

Also:

- [`runs/add_and_div.today.trace-label-steps.md`](runs/add_and_div.today.trace-label-steps.md) — the steps that ask,
  walk and label (`tools/trace_excerpt.py`).
- The mocks: [`add_and_div.option-b.MOCK.saturate-facts.txt`](runs/add_and_div.option-b.MOCK.saturate-facts.txt),
  [`tagged_call.option-b.MOCK.saturate-facts.txt`](runs/tagged_call.option-b.MOCK.saturate-facts.txt),
  [`zero.option-c.MOCK.saturate-facts.txt`](runs/zero.option-c.MOCK.saturate-facts.txt).
- [`tools/`](tools/) — the scripts that made `runs/` and `renders/`: `run_all.sh` (every artefact of one program
  under one binary), `diffs.sh` (before → today diffs), `facts.py` (the facts of a dump that bear on the check),
  `jsonl2md.py` (events as a table), `kbsub.py` (the KB view cut by relation), `trace_excerpt.py` (a trace's steps
  of named rules), `steps_of.py` (a trace's steps that check or label one number).

## 6. Re-run

From the repo root:

```sh
D=plans/m1f_hypothesis_and_documentation/review/02-numerals/11-Q-M1f.406
build/review-bin/ein-head-17085b27 saturate $D/programs/add_and_div.ein --dump | grep 'decimal.valid'
build/review-bin/ein-head-17085b27 render kb $D/programs/small_add.ein > $D/renders/small_add.today.kb.dot
fdp -Tsvg $D/renders/small_add.today.kb.dot -o $D/renders/small_add.today.kb.svg      # 425 s here, the machine shared
# everything in runs/ and renders/ (the mocks are hand-made and are not re-made)
K="check valid bin-check bin-valid written add sum-of div quot-of sub diff-of add-walk done bin-add bin-sum-of div-done"
for p in add_and_div tagged_call zero binary_add; do for L in before today; do
  KBRELS="$K" $D/tools/run_all.sh $L $D/programs/$p.ein $D; done; done
FULLKB=1 KBRELS="$K" $D/tools/run_all.sh today $D/programs/small_add.ein $D
$D/tools/diffs.sh $D add_and_div tagged_call zero binary_add
python3 -I $D/tools/trace_excerpt.py $D/runs/add_and_div.today.trace.md $D/runs/add_and_div.today.trace-label-steps.md \
  operand-check numeral-step numeral-step-zero numeral-valid turn-end div-end strip-none decimal
```

## 7. Found while building

- Wording only: Q-M1f.408 (a) says *the label at a walk's end is written by `std.math.positional` with the base's
  `?VALID`*. Read beside the same answer's list of `std.math.decimal`'s rules (`numeral-valid` among them), *a
  walk* there is an arithmetic walk; an operand's label is `std.math.decimal`'s `numeral-valid` (binary's
  `bin-numeral-valid`), and only a made number's is `std.math.positional`'s (`turn-end`, `div-end`). The runs above
  show both `:rule`s.
- In `add_and_div` the remainder 2 was made by a subtraction walk before `div-end` and labelled there: the dump shows
  `(valid (n z 2) :rule std.math.positional.turn-end)`, and `div-end`'s label of it (trace step 148) is a second
  derivation of a held fact.
- **Re-taken with `ein-head-17085b27`** (the first look ran `ein-head-b5fb8d70`; the standard library and
  `tests/stdlib/` are the same in both trees, `git diff --stat b5fb8d70 17085b27 -- stdlib tests/stdlib` is empty):
  every read-out, summary, event stream, dump, `ein test` and `ein show hyps` output is the same, wall-clock aside,
  and so is every trace's text and every excerpt of it; what moved is the drawings, each trace's `dot` blocks, today's
  KB cuts and `small_add`'s whole KB view, now in the canonical form (that view has 10 818 arrows against the first
  look's 8 464, an arrow `1` a fact and the facts that apply a rule among them; `fdp` took 425 s on it, on a machine
  another run was loading, where the first look's page says about three minutes). The before binary's outputs re-run the same.

## 8. Your call

- **(a)** `(valid N)`, at a walk's end and as `turn-end` and `div-end` make a number; `z` by the closing rule —
  **the default, as built**.
- **(b)** only the public answers labelled — tagged calls' hand-overs unlabelled; fourteen answer rules in `int` and
  `rat`.
- **(c)** as (a), `z` labelled only where a walk makes it or a check reaches it.
