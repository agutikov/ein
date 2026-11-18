# Q-M1f.408 — where the check's rules live

## 1. The question

The check of a numeral is a walk (a place a firing, from the units, to `z`), a label at its end, and four flags that
read the walk. Someone has to ask for it (the operations), and someone has to hold the rules. As built, the
**representation** holds them — `std.math.decimal` (and `std.math.binary` under `bin-`) declares `check` and
`valid` and holds the walk and the flags — and the operations only ask: `std.math.positional`'s `operand-check`,
`std.math.convert`'s and `std.math.num9`'s requests, `std.math.int`'s and `std.math.rat`'s part checks. The
alternatives are the walk in `std.math.positional` (parameterised by the base), or each operation checking its own
operands with the representation keeping nothing.

Recorded in [`../../open_questions.md` § Q-M1f.408](../../../open_questions.md#q-m1f408--where-the-checks-rules-live);
the build is S1f.22.6 (`bf9693dc`).

## 2. What is so today

**The rules, before and today** — every rule of `std.math`'s modules that reads or writes the check's relations or a
flag, by `tools/check_census.py` over each tree:
[before](runs/check-rules.before.census.txt) (`build/wt-rev-0a4/stdlib`), [today](runs/check-rules.today.census.txt)
(`stdlib/`), [diff](runs/check-rules.census.diff). Before, the four flags of each representation read the loader's
`(written N)` and nothing asked. Today:

<!-- output -->
```
std.math.decimal        numeral-step               reads check                        writes check
std.math.decimal        numeral-step-zero          reads check                        writes check
std.math.decimal        numeral-valid              reads check                        writes valid
std.math.decimal        numeral-digit              reads check                        writes not-decimal
…                       (numeral-prefix-digit, -zero, -named the same)
std.math.decimal        decimal                    reads -                            writes valid
std.math.binary         bin-numeral-step …         (the same under bin-)
std.math.positional     operand-check              reads -                            writes ?CHECK
std.math.positional     turn-end                   reads -                            writes ?VALID
std.math.positional     div-end                    reads -                            writes ?VALID
std.math.convert        to-bin-ask                 reads -                            writes check
std.math.convert        to-dec-ask                 reads -                            writes bin-check
std.math.num9           to9-check                  reads -                            writes check
std.math.num9           from9-9 … from9-1          reads -                            writes valid
std.math.int            int-operand-check          reads -                            writes int-check
std.math.int            int-check-parts            reads int-check                    writes check
std.math.rat            rat-check-ask              reads -                            writes rat-check
std.math.rat            rat-check-parts            reads rat-check                    writes check,int-check
```

**The modules' text** — `git diff 0a4a97a3 b5fb8d70 -- stdlib/math/MODULE.ein`:
[`decimal.ein.diff`](runs/decimal.ein.diff) (`check` and `valid` declared; `numeral-step`, `numeral-step-zero`,
`numeral-valid` added; each flag's `(written (n ?p ?a))` premise becomes `(check ?w (n ?p ?a))`; the closing rule
labels `(valid z)`), [`binary.ein.diff`](runs/binary.ein.diff) (the same under `bin-`),
[`positional.ein.diff`](runs/positional.ein.diff) (`operand-check`; `turn-end` and `div-end` take `?VALID`; the
macro's list grows by `?CHECK` and `?VALID`), and the askers: [`convert.ein.diff`](runs/convert.ein.diff),
[`num9.ein.diff`](runs/num9.ein.diff), [`int.ein.diff`](runs/int.ein.diff), [`rat.ein.diff`](runs/rat.ein.diff),
[`decimal-arith.ein.diff`](runs/decimal-arith.ein.diff), [`binary-arith.ein.diff`](runs/binary-arith.ein.diff),
[`gcd.ein.diff`](runs/gcd.ein.diff) (comments only: `gcd` takes its operands through `operand`).

**The rules drawn** — `ein render rule --name RULE stdlib/math/MODULE.ein`, DOT and SVG (`dot -Tsvg`):

| rule | today | before | DOT diff |
|---|---|---|---|
| `std.math.decimal.numeral-step` | [dot](renders/decimal.numeral-step.today.rule.dot) · [svg](renders/decimal.numeral-step.today.rule.svg) · overlay [dot](renders/decimal.numeral-step.today.rule-overlay.dot) [svg](renders/decimal.numeral-step.today.rule-overlay.svg) | — (new) | |
| `std.math.decimal.numeral-step-zero` | [dot](renders/decimal.numeral-step-zero.today.rule.dot) · [svg](renders/decimal.numeral-step-zero.today.rule.svg) | — (new) | |
| `std.math.decimal.numeral-valid` | [dot](renders/decimal.numeral-valid.today.rule.dot) · [svg](renders/decimal.numeral-valid.today.rule.svg) | — (new) | |
| `std.math.decimal.numeral-digit` | [dot](renders/decimal.numeral-digit.today.rule.dot) · [svg](renders/decimal.numeral-digit.today.rule.svg) | [dot](renders/decimal.numeral-digit.before.rule.dot) · [svg](renders/decimal.numeral-digit.before.rule.svg) | [diff](runs/decimal.numeral-digit.rule-dot.diff) |
| `std.math.decimal.numeral-prefix-digit` | [dot](renders/decimal.numeral-prefix-digit.today.rule.dot) · [svg](renders/decimal.numeral-prefix-digit.today.rule.svg) | [dot](renders/decimal.numeral-prefix-digit.before.rule.dot) · [svg](renders/decimal.numeral-prefix-digit.before.rule.svg) | [diff](runs/decimal.numeral-prefix-digit.rule-dot.diff) |
| `std.math.decimal.numeral-prefix-zero` | [dot](renders/decimal.numeral-prefix-zero.today.rule.dot) · [svg](renders/decimal.numeral-prefix-zero.today.rule.svg) | [dot](renders/decimal.numeral-prefix-zero.before.rule.dot) · [svg](renders/decimal.numeral-prefix-zero.before.rule.svg) | [diff](runs/decimal.numeral-prefix-zero.rule-dot.diff) |
| `std.math.decimal.numeral-prefix-named` | [dot](renders/decimal.numeral-prefix-named.today.rule.dot) · [svg](renders/decimal.numeral-prefix-named.today.rule.svg) | [dot](renders/decimal.numeral-prefix-named.before.rule.dot) · [svg](renders/decimal.numeral-prefix-named.before.rule.svg) | [diff](runs/decimal.numeral-prefix-named.rule-dot.diff) |
| `std.math.binary.bin-numeral-step` | [dot](renders/binary.bin-numeral-step.today.rule.dot) · [svg](renders/binary.bin-numeral-step.today.rule.svg) | — (new) | |
| `std.math.binary.bin-numeral-valid` | [dot](renders/binary.bin-numeral-valid.today.rule.dot) · [svg](renders/binary.bin-numeral-valid.today.rule.svg) | — (new) | |
| `std.math.binary.bin-numeral-bit` | [dot](renders/binary.bin-numeral-bit.today.rule.dot) · [svg](renders/binary.bin-numeral-bit.today.rule.svg) | [dot](renders/binary.bin-numeral-bit.before.rule.dot) · [svg](renders/binary.bin-numeral-bit.before.rule.svg) | [diff](runs/binary.bin-numeral-bit.rule-dot.diff) |
| `std.math.positional.operand-check` | [dot](renders/positional.operand-check.today.rule.dot) · [svg](renders/positional.operand-check.today.rule.svg) | — (new) | |
| `std.math.positional.turn-end` | [dot](renders/positional.turn-end.today.rule.dot) · [svg](renders/positional.turn-end.today.rule.svg) | | |
| `std.math.positional.div-end` | [dot](renders/positional.div-end.today.rule.dot) · [svg](renders/positional.div-end.today.rule.svg) | | |

In `numeral-digit`'s drawing the match's context frame was `written ctx` with a red `written` octagon over `(n ?p ?a)`;
today it is `check ctx`, an arrow `check` from `?w` to `(n ?p ?a)` ([diff](runs/decimal.numeral-digit.rule-dot.diff)).
`operand-check` is drawn with its parameters as written, `?OPERAND-IS`, `?N`, `?CHECK`.

**Where each rule fires** — [`programs/check_rules.ein`](programs/check_rules.ein): 10 + 5, and 1x5 + 5 (`x` one
place in). The trace names each rule by its full name, so its module
([excerpt today](runs/check_rules.today.trace-check-steps.md)): `std.math.positional.operand-check` (steps 8, 10, 17,
21) asks; `std.math.decimal.numeral-step-zero` (12), `numeral-step` (14, 16, 24) walk; `std.math.decimal.numeral-digit`
(19) flags `(n (n z 1) x)` and `not-decimal-is-false` (20) derives `(false)`; `numeral-valid` (23, 27) labels. Before,
two steps: `numeral-digit` reading `written(n(n(z, 1), x))` and `not-decimal-is-false`
([excerpt before](runs/check_rules.before.trace-check-steps.md)). Both dead; today's unsat core adds the table row the
walk read, `(std.math.digits.nz 5)` ([read-out diff](runs/check_rules.solve.diff)). Binary's twin,
[`programs/bin_check_rules.ein`](programs/bin_check_rules.ein): the same with `std.math.binary.bin-*`
([excerpt today](runs/bin_check_rules.today.trace-check-steps.md), [read-out diff](runs/bin_check_rules.solve.diff)).

## 3. The options

- **(a) the representation's**: `std.math.decimal` declares `check` and `valid` and holds the walk and its four flags;
  `std.math.binary` the same under `bin-`; the operations ask. **What runs today.**
- **(b) the walk in `std.math.positional`**, parameterised by the base's digits, the representation keeping the
  relations and the flags. Mocks: [`runs/rules-placement.option-b.MOCK.txt`](runs/rules-placement.option-b.MOCK.txt)
  (the three walk rules as `std.math.positional` would hold them, hand-written from today's) and
  [`runs/check_rules.option-b.MOCK.trace-check-steps.md`](runs/check_rules.option-b.MOCK.trace-check-steps.md)
  (today's trace excerpt with the three walk rules renamed `std.math.positional.*`). No answer changes; one copy of the
  walk serves both bases, and a program that loads a representation without an operation loads no walk.
- **(c) each operation checking its own operands**, the representation keeping nothing. Mocks:
  [`runs/rules-placement.option-c.MOCK.txt`](runs/rules-placement.option-c.MOCK.txt) (where each walk and its flags
  would live: one copy for the operands, one in `convert`, one in `num9`, one in `int`, one in `rat`) and
  [`runs/check_rules.option-c.MOCK.trace-check-steps.md`](runs/check_rules.option-c.MOCK.trace-check-steps.md) (the
  walk, the flags and the relations renamed to the operation's).

## 4. The default

**(a)**, built by S1f.22.6 (`bf9693dc`), stated by Q-M1f.404's decision. From the stage file's reading 4 (read from
git): *what a well-formed numeral is, and the label, stay the representation's, as Q-M1f.210 placed them*; the
operations ask for the check and label their results. One walk serves every module that takes a numeral of a base.

## 5. Files

Programs: `check_rules` and `bin_check_rules`, written for this review, each claiming `(false)`, the answer under both
binaries.

Per program and binary: *read-out* `ein solve`; *summary* `--json-summary`; *events* `--events` (JSONL) and its
*events table*; *trace* `--trace`; *dump-answer* `--dump-answer DIR`; *saturate --dump* whole and *its facts*
(`tools/facts.py`); `ein test`; `ein show hyps`; *kb-cut* `ein render kb` cut to the facts of `check valid bin-check
bin-valid not-decimal not-binary written add bin-add` and what they nest, drawn with `fdp` (`tools/kbsub.py`). Each
text output ends with `[exit N]`.

| program | binary | artefacts |
|---|---|---|
| [`check_rules`](programs/check_rules.ein) | before | [read-out](runs/check_rules.before.solve.txt) · [summary](runs/check_rules.before.summary.json) · [events](runs/check_rules.before.events.jsonl) · [events table](runs/check_rules.before.events.md) · [trace](runs/check_rules.before.trace.md) · [dump-answer](runs/check_rules.before.answer/) · [saturate --dump](runs/check_rules.before.saturate-dump.txt) · [its facts](runs/check_rules.before.saturate-facts.txt) · [ein test](runs/check_rules.before.test.txt) · [show hyps](runs/check_rules.before.show-hyps.txt) · [kb-cut.dot](renders/check_rules.before.kb-cut.dot) · [kb-cut.svg](renders/check_rules.before.kb-cut.svg) |
| [`check_rules`](programs/check_rules.ein) | today | [read-out](runs/check_rules.today.solve.txt) · [summary](runs/check_rules.today.summary.json) · [events](runs/check_rules.today.events.jsonl) · [events table](runs/check_rules.today.events.md) · [trace](runs/check_rules.today.trace.md) · [dump-answer](runs/check_rules.today.answer/) · [saturate --dump](runs/check_rules.today.saturate-dump.txt) · [its facts](runs/check_rules.today.saturate-facts.txt) · [ein test](runs/check_rules.today.test.txt) · [show hyps](runs/check_rules.today.show-hyps.txt) · [kb-cut.dot](renders/check_rules.today.kb-cut.dot) · [kb-cut.svg](renders/check_rules.today.kb-cut.svg) |
| [`check_rules`](programs/check_rules.ein) | diff | [saturate-facts.diff](runs/check_rules.saturate-facts.diff) · [solve.diff](runs/check_rules.solve.diff) · [test.diff](runs/check_rules.test.diff) · [trace-steps.diff](runs/check_rules.trace-steps.diff) |
| [`bin_check_rules`](programs/bin_check_rules.ein) | before | [read-out](runs/bin_check_rules.before.solve.txt) · [summary](runs/bin_check_rules.before.summary.json) · [events](runs/bin_check_rules.before.events.jsonl) · [events table](runs/bin_check_rules.before.events.md) · [trace](runs/bin_check_rules.before.trace.md) · [dump-answer](runs/bin_check_rules.before.answer/) · [saturate --dump](runs/bin_check_rules.before.saturate-dump.txt) · [its facts](runs/bin_check_rules.before.saturate-facts.txt) · [ein test](runs/bin_check_rules.before.test.txt) · [show hyps](runs/bin_check_rules.before.show-hyps.txt) · [kb-cut.dot](renders/bin_check_rules.before.kb-cut.dot) · [kb-cut.svg](renders/bin_check_rules.before.kb-cut.svg) |
| [`bin_check_rules`](programs/bin_check_rules.ein) | today | [read-out](runs/bin_check_rules.today.solve.txt) · [summary](runs/bin_check_rules.today.summary.json) · [events](runs/bin_check_rules.today.events.jsonl) · [events table](runs/bin_check_rules.today.events.md) · [trace](runs/bin_check_rules.today.trace.md) · [dump-answer](runs/bin_check_rules.today.answer/) · [saturate --dump](runs/bin_check_rules.today.saturate-dump.txt) · [its facts](runs/bin_check_rules.today.saturate-facts.txt) · [ein test](runs/bin_check_rules.today.test.txt) · [show hyps](runs/bin_check_rules.today.show-hyps.txt) · [kb-cut.dot](renders/bin_check_rules.today.kb-cut.dot) · [kb-cut.svg](renders/bin_check_rules.today.kb-cut.svg) |
| [`bin_check_rules`](programs/bin_check_rules.ein) | diff | [saturate-facts.diff](runs/bin_check_rules.saturate-facts.diff) · [solve.diff](runs/bin_check_rules.solve.diff) · [test.diff](runs/bin_check_rules.test.diff) · [trace-steps.diff](runs/bin_check_rules.trace-steps.diff) |

Also:

- The rules drawn: the table in § 2 (`renders/MODULE.RULE.{today,before}.rule.{dot,svg}`), and their DOT diffs
  `runs/MODULE.RULE.rule-dot.diff`.
- The modules' diffs: `runs/MODULE.ein.diff`, linked in § 2.
- The census: [`runs/check-rules.before.census.txt`](runs/check-rules.before.census.txt),
  [`runs/check-rules.today.census.txt`](runs/check-rules.today.census.txt),
  [`runs/check-rules.census.diff`](runs/check-rules.census.diff).
- Trace excerpts: [`check_rules.before`](runs/check_rules.before.trace-check-steps.md),
  [`check_rules.today`](runs/check_rules.today.trace-check-steps.md),
  [`bin_check_rules.before`](runs/bin_check_rules.before.trace-check-steps.md),
  [`bin_check_rules.today`](runs/bin_check_rules.today.trace-check-steps.md).
- The mocks: [`rules-placement.option-b.MOCK.txt`](runs/rules-placement.option-b.MOCK.txt),
  [`rules-placement.option-c.MOCK.txt`](runs/rules-placement.option-c.MOCK.txt),
  [`check_rules.option-b.MOCK.trace-check-steps.md`](runs/check_rules.option-b.MOCK.trace-check-steps.md),
  [`check_rules.option-c.MOCK.trace-check-steps.md`](runs/check_rules.option-c.MOCK.trace-check-steps.md).
- [`runs/render-rule-by-name.txt`](runs/render-rule-by-name.txt) — `ein render rule --name` given a program and given
  the module's file.
- [`tools/`](tools/) — the scripts that made `runs/` and `renders/`: `run_all.sh` (every artefact of one program
  under one binary), `diffs.sh` (before → today diffs), `facts.py` (the facts of a dump that bear on the check),
  `jsonl2md.py` (events as a table), `kbsub.py` (the KB view cut by relation), `trace_excerpt.py` (a trace's steps
  of named rules), `steps_of.py` (a trace's steps that check or label one number), and `check_census.py` (the check's rules of every module of a tree).

## 6. Re-run

From the repo root:

```sh
D=plans/m1f_hypothesis_and_documentation/review/02-numerals/13-Q-M1f.408
git diff 0a4a97a3 b5fb8d70 -- stdlib/math/decimal.ein stdlib/math/binary.ein
build/review-bin/ein-head-b5fb8d70 render rule --name numeral-step stdlib/math/decimal.ein > $D/renders/decimal.numeral-step.today.rule.dot
dot -Tsvg $D/renders/decimal.numeral-step.today.rule.dot -o $D/renders/decimal.numeral-step.today.rule.svg
EIN_STDLIB=build/wt-rev-0a4/stdlib build/review-bin/ein-0a4a97a3 render rule --name numeral-digit \
  build/wt-rev-0a4/stdlib/math/decimal.ein > $D/renders/decimal.numeral-digit.before.rule.dot
python3 -I $D/tools/check_census.py stdlib > $D/runs/check-rules.today.census.txt
python3 -I $D/tools/check_census.py build/wt-rev-0a4/stdlib > $D/runs/check-rules.before.census.txt
for p in check_rules bin_check_rules; do for L in before today; do
  KBRELS="check valid bin-check bin-valid not-decimal not-binary written add bin-add" $D/tools/run_all.sh $L $D/programs/$p.ein $D
done; done
$D/tools/diffs.sh $D check_rules bin_check_rules
```

## 7. Found while building

- `ein render rule --name` draws a rule of the standard library only from the module's own file, by its short name
  (`--name numeral-step stdlib/math/decimal.ein`). Given a program that imports the module, neither the full name
  nor the short one is found: `no rule named …`, exit 1
  ([`runs/render-rule-by-name.txt`](runs/render-rule-by-name.txt)). Not this question's; noted for the renderer.
- Q-M1f.408 (a)'s *the label at a walk's end is written by `std.math.positional`* means the arithmetic walk's end
  (`turn-end`, `div-end`); a numeral's check walk is labelled by `std.math.decimal`'s own `numeral-valid`, as the
  same answer's list says and the census shows (also in `../11-Q-M1f.406/`).

## 8. Your call

- **(a)** the representation holds the walk, the label and the flags; the operations ask — **the default, as
  built**.
- **(b)** the walk in `std.math.positional`, the representation keeping the flags.
- **(c)** each operation checks its own operands; the representation keeps nothing.
