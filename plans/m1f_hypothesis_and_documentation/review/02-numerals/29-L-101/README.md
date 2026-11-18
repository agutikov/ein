# L-101 — `int-check-parts` and `rat-check-parts` ask the check of parts that are not numerals

## 1. The question

When an operation takes an integer `(int P N)`, `std.math.int`'s `int-check-parts` asks `std.math.decimal`'s check,
`(check X X)`, of **each part whatever it is**; when it takes a rational `(rat Z D)`, `std.math.rat`'s
`rat-check-parts` asks `std.math.int`'s check, `(int-check Z)`, of the numerator **whatever it is**. So a part that is
no numeral is still asked: a digit or a name where a part goes leaves a fact `(check 7 7)` that walks nothing; a part
`z` gives a second derivation of `(valid z)`; a natural-number numerator leaves an `(int-check (n …))` no rule reads.
No answer moves (a part at fault is flagged by the shape rules, and the program is dead). Should there be a rule a
shape, so that only a numeral is asked, or are the rules left as they are?

Recorded in [`../../findings.md` § The standard library's rules](../../../findings.md#the-standard-librarys-rules), row
**L-101** (P1f.22's N-34), marked *the user's call*. The ledger marks it *read and not re-run* at the close; it is
re-run here.

## 2. What is so today

Runs under **today** = `build/review-bin/ein-head-17085b27`, and for comparison **before** =
`build/review-bin/ein-0a4a97a3` with `EIN_STDLIB=build/wt-rev-0a4/stdlib` (no check rules then; the shape rules read
`written`).

**The rules** ([`runs/the-rules.txt`](runs/the-rules.txt), the ledger's `grep` and the readers of `int-check`):

<!-- output -->
```
721:(rule int-check-parts ()
722-  :match  (int-check (int ?p ?n))
723-  :assert (and (check ?p ?p) (check ?n ?n))
629:(rule rat-check-parts ()
630-  :match  (rat-check (rat ?i ?d))
631-  :assert (and (int-check ?i) (check ?d ?d))
```
Every rule that reads `int-check` matches `(int-check (int …))`: `int-check-parts` and the six shape rules, all in
`stdlib/math/int.ein`. Drawn with `ein render rule`: `int-check-parts`
[dot](renders/int.int-check-parts.rule.dot) · [svg](renders/int.int-check-parts.rule.svg), `rat-check-parts`
[dot](renders/rat.rat-check-parts.rule.dot) · [svg](renders/rat.rat-check-parts.rule.svg), and a shape rule beside them,
`int-p-digit` [dot](renders/int.int-p-digit.rule.dot) · [svg](renders/int.int-p-digit.rule.svg). Each is drawn as the
graph has it: in `int-check-parts` the premise is an octagon whose arrow `1` is at the box `int-check` and arrow `2`
at the octagon of `(int ?p ?n)`, in the frame `int-check ctx`; the conclusion is two dashed octagons of `check`, each
with its arrows `2` and `3` at the same variable.

![int-check-parts: the match's int-check fact an octagon whose arrow 2 enters (int ?p ?n) in the frame int-check ctx; the assert two dashed check facts, each with arrows 2 and 3 at one variable](renders/int.int-check-parts.rule.svg)

**The witness, re-taken** — [`programs/witness_int_digit_part.ein`](programs/witness_int_digit_part.ein),
`(int-add (int 7 z) (int (n z 1) z))`; `ein saturate --dump` ([whole](runs/witness_int_digit_part.today.saturate-dump.txt),
[its facts](runs/witness_int_digit_part.today.saturate-facts.txt)) holds the ledger's fact:

<!-- output -->
```
  (std.math.decimal.check 7 7 :rule std.math.int.int-check-parts)
  (std.math.decimal.check z z :rule std.math.int.int-check-parts)
  (std.math.int.not-integer (int 7 z) 7 :rule std.math.int.int-p-digit)
```
Trace ([steps](runs/witness_int_digit_part.today.steps-of-7.txt)): step 10 `int-check-parts` derives `check(7, 7)`
and `check(z, z)`; 11 `int-p-digit` flags the 7; 12 `(false)`. `(check 7 7)` walks nothing: no step reads it. Root
dead, and the read-out is the same under both binaries ([read-out diff](runs/witness_int_digit_part.solve.diff): no
difference).

**The three points, one program each:**

1. *A digit or a name where a part goes* — the witness above (`(check 7 7)`), and
   [`programs/named_part.ein`](programs/named_part.ein), `(int price z)` with `(let price (n z 5))`:
   `(check price price)` at step 11, flagged by `int-p-named` at 12
   ([steps](runs/named_part.today.steps-of-price.txt), [facts](runs/named_part.today.saturate-facts.txt)).
2. *A part `z`, a second derivation of `(valid z)`* — [`programs/part_z.ein`](programs/part_z.ein), 7 + (−3), well
   formed: step 10 `int-check-parts` asks `check(z, z)`; step 14 `numeral-valid` derives `valid(z)`, which step 2,
   the closing rule `std.math.decimal.decimal`, holds already ([steps](runs/part_z.today.steps-of-z.txt)). The dump
   shows the fact once, by its first rule ([facts](runs/part_z.today.saturate-facts.txt)).
3. *A natural numerator, an `(int-check (n …))` no rule reads* — [`programs/rat_natural_numerator.ein`](programs/rat_natural_numerator.ein),
   `(rat-add (rat (n z 3) (n z 2)) …)`: step 10 `rat-check-parts` derives `int-check(n(z, 3))`; nothing follows from
   it ([steps](runs/rat_natural_numerator.today.steps-of-3.txt), [facts](runs/rat_natural_numerator.today.saturate-facts.txt)).
   R = {∅}, no sum (the operation computes nothing with a numerator that is no integer, Q-M1f.277). The same fact
   appears on `../05-Q-M1f.409/`'s moved case, `(int-check (n z x))`.

Every claim holds under both binaries (`ein test`, the files table below), and the four read-outs are the same under
both (`*.solve.diff`: no difference); only the facts differ.

## 3. The options

- **A rule a shape, so that only a numeral is asked** (the ledger: *two rules more each, and a program each to
  activate them*). Mock: [`runs/the-rules.option-a-shape.MOCK.txt`](runs/the-rules.option-a-shape.MOCK.txt) —
  `int-check-parts` as two rules (`int-check-p`, `int-check-n`, each matching a part `(n ?q ?a)`), `rat-check-parts`
  as two (`rat-check-z` matching an `(int ?p ?n)` numerator, `rat-check-d` a `(n ?q ?a)` denominator); names made up.
  What it removes, each mock the program's today's facts with those lines commented out:
  [witness](runs/witness_int_digit_part.option-shape.MOCK.saturate-facts.txt) (`check 7 7`, `check z z`),
  [named part](runs/named_part.option-shape.MOCK.saturate-facts.txt) (`check price price`, `check z z`),
  [part z](runs/part_z.option-shape.MOCK.saturate-facts.txt) (`check z z`, and trace step 14),
  [rat](runs/rat_natural_numerator.option-shape.MOCK.saturate-facts.txt) (`int-check (n z 3)`). No answer changes.
- **As they are** — what runs today (above): one rule each, a few facts no rule reads.

## 4. The default

None taken: the ledger marks the row **the user's call**. What runs is *as they are*, built by S1f.22.6.

## 5. Files

Programs: `witness_int_digit_part` is the ledger's witness; `named_part`, `part_z` and `rat_natural_numerator` are
written for this review. Each claims the answer under both binaries.

Per program and binary: *read-out* `ein solve`; *summary* `--json-summary`; *events* `--events` (JSONL) and its
*events table*; *trace* `--trace`; *dump-answer* `--dump-answer DIR`; *saturate --dump* whole and *its facts*
(`tools/facts.py`); `ein test`; `ein show hyps`; *kb-cut* `ein render kb` cut to the facts of `check valid int-check
rat-check not-integer not-rational not-decimal written int-add rat-add` and what they nest, drawn with `fdp`
(`tools/kbsub.py`). Each text output ends with `[exit N]`.

*Today*'s kb-cut is drawn as the graph has it: a fact an octagon with no name in it, its arrow `1` at its relation's
box and `2`, `3`, … at its arguments, what it rests on (`by RULE`) beside it, and a fact under a fact's arrow that the
knowledge base does not hold in the frame of what points at it (`add ctx`); *before*'s is the earlier binary's
drawing, labelled octagons and labelled arrows. `tools/kbsub.py` reads both forms.

| program | binary | artefacts |
|---|---|---|
| [`witness_int_digit_part`](programs/witness_int_digit_part.ein) | before | [read-out](runs/witness_int_digit_part.before.solve.txt) · [summary](runs/witness_int_digit_part.before.summary.json) · [events](runs/witness_int_digit_part.before.events.jsonl) · [events table](runs/witness_int_digit_part.before.events.md) · [trace](runs/witness_int_digit_part.before.trace.md) · [dump-answer](runs/witness_int_digit_part.before.answer/) · [saturate --dump](runs/witness_int_digit_part.before.saturate-dump.txt) · [its facts](runs/witness_int_digit_part.before.saturate-facts.txt) · [ein test](runs/witness_int_digit_part.before.test.txt) · [show hyps](runs/witness_int_digit_part.before.show-hyps.txt) · [kb-cut.dot](renders/witness_int_digit_part.before.kb-cut.dot) · [kb-cut.svg](renders/witness_int_digit_part.before.kb-cut.svg) |
| [`witness_int_digit_part`](programs/witness_int_digit_part.ein) | today | [read-out](runs/witness_int_digit_part.today.solve.txt) · [summary](runs/witness_int_digit_part.today.summary.json) · [events](runs/witness_int_digit_part.today.events.jsonl) · [events table](runs/witness_int_digit_part.today.events.md) · [trace](runs/witness_int_digit_part.today.trace.md) · [dump-answer](runs/witness_int_digit_part.today.answer/) · [saturate --dump](runs/witness_int_digit_part.today.saturate-dump.txt) · [its facts](runs/witness_int_digit_part.today.saturate-facts.txt) · [ein test](runs/witness_int_digit_part.today.test.txt) · [show hyps](runs/witness_int_digit_part.today.show-hyps.txt) · [kb-cut.dot](renders/witness_int_digit_part.today.kb-cut.dot) · [kb-cut.svg](renders/witness_int_digit_part.today.kb-cut.svg) |
| [`witness_int_digit_part`](programs/witness_int_digit_part.ein) | diff | [saturate-facts.diff](runs/witness_int_digit_part.saturate-facts.diff) · [solve.diff](runs/witness_int_digit_part.solve.diff) · [test.diff](runs/witness_int_digit_part.test.diff) · [trace-steps.diff](runs/witness_int_digit_part.trace-steps.diff) |
| [`named_part`](programs/named_part.ein) | before | [read-out](runs/named_part.before.solve.txt) · [summary](runs/named_part.before.summary.json) · [events](runs/named_part.before.events.jsonl) · [events table](runs/named_part.before.events.md) · [trace](runs/named_part.before.trace.md) · [dump-answer](runs/named_part.before.answer/) · [saturate --dump](runs/named_part.before.saturate-dump.txt) · [its facts](runs/named_part.before.saturate-facts.txt) · [ein test](runs/named_part.before.test.txt) · [show hyps](runs/named_part.before.show-hyps.txt) · [kb-cut.dot](renders/named_part.before.kb-cut.dot) · [kb-cut.svg](renders/named_part.before.kb-cut.svg) |
| [`named_part`](programs/named_part.ein) | today | [read-out](runs/named_part.today.solve.txt) · [summary](runs/named_part.today.summary.json) · [events](runs/named_part.today.events.jsonl) · [events table](runs/named_part.today.events.md) · [trace](runs/named_part.today.trace.md) · [dump-answer](runs/named_part.today.answer/) · [saturate --dump](runs/named_part.today.saturate-dump.txt) · [its facts](runs/named_part.today.saturate-facts.txt) · [ein test](runs/named_part.today.test.txt) · [show hyps](runs/named_part.today.show-hyps.txt) · [kb-cut.dot](renders/named_part.today.kb-cut.dot) · [kb-cut.svg](renders/named_part.today.kb-cut.svg) |
| [`named_part`](programs/named_part.ein) | diff | [saturate-facts.diff](runs/named_part.saturate-facts.diff) · [solve.diff](runs/named_part.solve.diff) · [test.diff](runs/named_part.test.diff) · [trace-steps.diff](runs/named_part.trace-steps.diff) |
| [`part_z`](programs/part_z.ein) | before | [read-out](runs/part_z.before.solve.txt) · [summary](runs/part_z.before.summary.json) · [events](runs/part_z.before.events.jsonl) · [events table](runs/part_z.before.events.md) · [trace](runs/part_z.before.trace.md) · [dump-answer](runs/part_z.before.answer/) · [saturate --dump](runs/part_z.before.saturate-dump.txt) · [its facts](runs/part_z.before.saturate-facts.txt) · [ein test](runs/part_z.before.test.txt) · [show hyps](runs/part_z.before.show-hyps.txt) · [kb-cut.dot](renders/part_z.before.kb-cut.dot) · [kb-cut.svg](renders/part_z.before.kb-cut.svg) |
| [`part_z`](programs/part_z.ein) | today | [read-out](runs/part_z.today.solve.txt) · [summary](runs/part_z.today.summary.json) · [events](runs/part_z.today.events.jsonl) · [events table](runs/part_z.today.events.md) · [trace](runs/part_z.today.trace.md) · [dump-answer](runs/part_z.today.answer/) · [saturate --dump](runs/part_z.today.saturate-dump.txt) · [its facts](runs/part_z.today.saturate-facts.txt) · [ein test](runs/part_z.today.test.txt) · [show hyps](runs/part_z.today.show-hyps.txt) · [kb-cut.dot](renders/part_z.today.kb-cut.dot) · [kb-cut.svg](renders/part_z.today.kb-cut.svg) |
| [`part_z`](programs/part_z.ein) | diff | [saturate-facts.diff](runs/part_z.saturate-facts.diff) · [solve.diff](runs/part_z.solve.diff) · [test.diff](runs/part_z.test.diff) · [trace-steps.diff](runs/part_z.trace-steps.diff) |
| [`rat_natural_numerator`](programs/rat_natural_numerator.ein) | before | [read-out](runs/rat_natural_numerator.before.solve.txt) · [summary](runs/rat_natural_numerator.before.summary.json) · [events](runs/rat_natural_numerator.before.events.jsonl) · [events table](runs/rat_natural_numerator.before.events.md) · [trace](runs/rat_natural_numerator.before.trace.md) · [dump-answer](runs/rat_natural_numerator.before.answer/) · [saturate --dump](runs/rat_natural_numerator.before.saturate-dump.txt) · [its facts](runs/rat_natural_numerator.before.saturate-facts.txt) · [ein test](runs/rat_natural_numerator.before.test.txt) · [show hyps](runs/rat_natural_numerator.before.show-hyps.txt) · [kb-cut.dot](renders/rat_natural_numerator.before.kb-cut.dot) · [kb-cut.svg](renders/rat_natural_numerator.before.kb-cut.svg) |
| [`rat_natural_numerator`](programs/rat_natural_numerator.ein) | today | [read-out](runs/rat_natural_numerator.today.solve.txt) · [summary](runs/rat_natural_numerator.today.summary.json) · [events](runs/rat_natural_numerator.today.events.jsonl) · [events table](runs/rat_natural_numerator.today.events.md) · [trace](runs/rat_natural_numerator.today.trace.md) · [dump-answer](runs/rat_natural_numerator.today.answer/) · [saturate --dump](runs/rat_natural_numerator.today.saturate-dump.txt) · [its facts](runs/rat_natural_numerator.today.saturate-facts.txt) · [ein test](runs/rat_natural_numerator.today.test.txt) · [show hyps](runs/rat_natural_numerator.today.show-hyps.txt) · [kb-cut.dot](renders/rat_natural_numerator.today.kb-cut.dot) · [kb-cut.svg](renders/rat_natural_numerator.today.kb-cut.svg) |
| [`rat_natural_numerator`](programs/rat_natural_numerator.ein) | diff | [saturate-facts.diff](runs/rat_natural_numerator.saturate-facts.diff) · [solve.diff](runs/rat_natural_numerator.solve.diff) · [test.diff](runs/rat_natural_numerator.test.diff) · [trace-steps.diff](runs/rat_natural_numerator.trace-steps.diff) |

Also:

- [`runs/the-rules.txt`](runs/the-rules.txt) — the two rules and every reader of `int-check`, by `grep`.
- The rules drawn: [`renders/int.int-check-parts.rule.svg`](renders/int.int-check-parts.rule.svg),
  [`renders/rat.rat-check-parts.rule.svg`](renders/rat.rat-check-parts.rule.svg),
  [`renders/int.int-p-digit.rule.svg`](renders/int.int-p-digit.rule.svg), each with its `.dot`.
- The steps behind each point (`tools/steps_of.py`):
  [`witness_int_digit_part.today.steps-of-7.txt`](runs/witness_int_digit_part.today.steps-of-7.txt),
  [`named_part.today.steps-of-price.txt`](runs/named_part.today.steps-of-price.txt),
  [`part_z.today.steps-of-z.txt`](runs/part_z.today.steps-of-z.txt),
  [`rat_natural_numerator.today.steps-of-3.txt`](runs/rat_natural_numerator.today.steps-of-3.txt); and the trace
  excerpts of every ask, walk and flag: [witness](runs/witness_int_digit_part.today.trace-check-steps.md),
  [named part](runs/named_part.today.trace-check-steps.md), [part z](runs/part_z.today.trace-check-steps.md),
  [rat](runs/rat_natural_numerator.today.trace-check-steps.md).
- The mocks: [`the-rules.option-a-shape.MOCK.txt`](runs/the-rules.option-a-shape.MOCK.txt) and the four
  `*.option-shape.MOCK.saturate-facts.txt` linked in § 3.
- [`tools/`](tools/) — the scripts that made `runs/` and `renders/`: `run_all.sh` (every artefact of one program
  under one binary), `diffs.sh` (before → today diffs), `facts.py` (the facts of a dump that bear on the check),
  `jsonl2md.py` (events as a table), `kbsub.py` (the KB view cut by relation), `trace_excerpt.py` (a trace's steps
  of named rules), `steps_of.py` (a trace's steps that check or label one number), `draw_rules.sh` (§ 2's three rule drawings, with
  [`../../03-drawings/tools/draw.sh`](../../03-drawings/tools/draw.sh)).

## 6. Re-run

From the repo root:

```sh
D=plans/m1f_hypothesis_and_documentation/review/02-numerals/29-L-101
build/review-bin/ein-head-17085b27 saturate $D/programs/witness_int_digit_part.ein --dump | grep 'decimal.check 7 7'
grep -n 'rule int-check-parts' -A3 stdlib/math/int.ein
grep -n 'rule rat-check-parts' -A3 stdlib/math/rat.ein
build/review-bin/ein-head-17085b27 render rule --name int-check-parts stdlib/math/int.ein > $D/renders/int.int-check-parts.rule.dot
dot -Tsvg $D/renders/int.int-check-parts.rule.dot -o $D/renders/int.int-check-parts.rule.svg
bash $D/tools/draw_rules.sh   # the three rules of § 2 drawn today
# everything in runs/ and renders/ (the mocks are hand-made and are not re-made)
P="witness_int_digit_part named_part part_z rat_natural_numerator"
for p in $P; do for L in before today; do
  KBRELS="check valid int-check rat-check not-integer not-rational not-decimal written int-add rat-add" \
    $D/tools/run_all.sh $L $D/programs/$p.ein $D
done; done
$D/tools/diffs.sh $D $P
python3 -I $D/tools/steps_of.py $D/runs/part_z.today.trace.md z
```

## 7. Found while building

- The ledger's witness reproduces as stated: `(std.math.decimal.check 7 7)` is in the dump, by `int-check-parts`.
- Point 2 is visible only in the trace: the dump prints `(valid z)` once, with its first rule (`std.math.decimal.decimal`);
  the second derivation is step 14 of `part_z`'s trace.
- The ledger's *two rules more each*: the mock needs one rule more in each module (two rules where one was); if the
  ledger counts the rules of the option, each module has two, as here.
- **Re-taken with `ein-head-17085b27`** (the first look ran `ein-head-b5fb8d70`; the standard library and
  `tests/stdlib/` are the same in both trees, `git diff --stat b5fb8d70 17085b27 -- stdlib tests/stdlib` is empty):
  every read-out, summary, event stream, dump, `ein test` and `ein show hyps` output is the same, wall-clock aside,
  and so is every trace's text and every excerpt of it; what moved is the drawings, each trace's `dot` blocks, today's
  KB cuts and the three rule drawings, now in the canonical form. The before binary's outputs re-run the same.

## 8. Your call

- **A rule a shape** — only a numeral (or, for a rational, an integer) is asked; four rules where two are; no answer
  moves.
- **As they are** — what runs today; a few facts no rule reads. The ledger takes no default.
