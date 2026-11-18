# Q-M1f.411 — a numeral with two faults has one flag

## 1. The question

A malformed numeral can be wrong in more than one place: `(n (n (n z x) y) 5)` has `y` where the tens digit goes and
`x` where the hundreds digit goes. Before, the loader stated every place of a written numeral at once, so each place
at fault was flagged: two flags. Now the check walks the numeral from the units, a place a firing, and the first
fault it meets stops the walk: one flag, on `(n (n z x) y)`. The program is dead either way. Is one flag, the first
place at fault from the units, right, or should every place at fault be flagged?

Recorded in [`../../open_questions.md` § Q-M1f.411](../../../open_questions.md#q-m1f411--a-numeral-with-two-faults-has-one-flag);
the decision that states it is Q-M1f.274 (re-worded by the build: [`runs/Q-M1f.274.decision.diff`](runs/Q-M1f.274.decision.diff),
*the first place at fault stops the walk and is flagged on its own `n` term*). The build is S1f.22.6 (`bf9693dc`).

## 2. What is so today

Each program run under **before** = `build/review-bin/ein-0a4a97a3` with `EIN_STDLIB=build/wt-rev-0a4/stdlib` and
**today** = `build/review-bin/ein-head-b5fb8d70`; both dead, `ein test` holds `(solve :expect (false))` under both.

**The ledger's numeral** — [`programs/two_faults.ein`](programs/two_faults.ein), `(add (n (n (n z x) y) 5) (n z 1))`.
The flags and the check's facts (`saturate --dump`, [before](runs/two_faults.before.saturate-facts.txt),
[today](runs/two_faults.today.saturate-facts.txt), [diff](runs/two_faults.saturate-facts.diff)):

<!-- output -->
```
before:
  (false :rule std.math.decimal.not-decimal-is-false)
  (not-decimal (n (n z x) y) y :rule std.math.decimal.numeral-digit)
  (not-decimal (n z x) x :rule std.math.decimal.numeral-digit)
  (written (n (n (n z x) y) 5) :rule "#written")
  (written (n (n z x) y) :rule "#written")
  (written (n z x) :rule "#written")
today:
  (false :rule std.math.decimal.not-decimal-is-false)
  (not-decimal (n (n z x) y) y :rule std.math.decimal.numeral-digit)
  (std.math.decimal.check (n (n (n z x) y) 5) (n (n (n z x) y) 5) :rule std.math.positional.operand-check)
  (std.math.decimal.check (n (n (n z x) y) 5) (n (n z x) y) :rule std.math.decimal.numeral-step)
```
Today the walk takes the units digit 5 (step 8 of the [trace excerpt](runs/two_faults.today.trace-check-steps.md)),
reaches `(n (n z x) y)`, flags `y` (step 10) and stops: `(n z x)` is never reached, and there is no `(valid …)` of
the numeral. Before, `numeral-digit` fired twice, once a written place
([excerpt](runs/two_faults.before.trace-check-steps.md), steps 2 and 4). The read-outs differ only in the unsat core,
which today adds the table row the walk read, `(std.math.digits.nz 5)` ([diff](runs/two_faults.solve.diff)).

**Binary's twin** — [`programs/bin_two_faults.ein`](programs/bin_two_faults.ein), `(b2 (b2 (b2 z x) y) 1)`: the
same, two `not-binary` flags before, one today, `(not-binary (b2 (b2 z x) y) y)`
([facts diff](runs/bin_two_faults.saturate-facts.diff)).

**Two faults at one place** — [`programs/two_kinds_of_fault.ein`](programs/two_kinds_of_fault.ein),
`(n (n 1 x) 5)`: `x` where a digit goes and the digit 1 where the numeral should end in `z`, both at the place the
walk stops on. **Two flags, before and today**: `(not-decimal (n 1 x) x)` by `numeral-digit` and `(not-decimal (n 1
x) 1)` by `numeral-prefix-digit` ([today](runs/two_kinds_of_fault.today.saturate-facts.txt),
[diff](runs/two_kinds_of_fault.saturate-facts.diff)). So *one flag* holds of faults at two places; two faults at the
first place at fault are each flagged.

## 3. The options

- **(a) one flag, the first place at fault from the units** (Q-M1f.274 re-worded so). The program is dead either way,
  and no corpus program writes such a numeral. **What runs today.**
- **(b) every place at fault flagged**: the walk carries on past a fault, which needs a second walk relation or a fault
  bit carried, so that the label is not derived for a malformed numeral. Mocks:
  [`runs/two_faults.option-b.MOCK.saturate-facts.txt`](runs/two_faults.option-b.MOCK.saturate-facts.txt) and
  [`runs/bin_two_faults.option-b.MOCK.saturate-facts.txt`](runs/bin_two_faults.option-b.MOCK.saturate-facts.txt):
  today's facts with a second walk relation, `check-faulty` (made up for the mock), carrying the walk from the first
  fault to `z` with no label, and the second flag, `(not-decimal (n z x) x)`, which the before engine also derived.
  No answer changes; the flags' extent, a query of `not-decimal` and the trace grow.

## 4. The default

**(a)**, built by S1f.22.6 (`bf9693dc`), stated by Q-M1f.274. From the ledger: the program is dead either way, and no
corpus program writes such a numeral; (b) needs a second walk relation or a fault bit, so that a malformed numeral's
walk never reaches its label.

## 5. Files

Programs: all three written for this review, each claiming `(false)`, the answer under both binaries.

Per program and binary: *read-out* `ein solve`; *summary* `--json-summary`; *events* `--events` (JSONL) and its
*events table*; *trace* `--trace`; *dump-answer* `--dump-answer DIR`; *saturate --dump* whole and *its facts*
(`tools/facts.py`); `ein test`; `ein show hyps`; *kb-cut* `ein render kb` cut to the facts of `check valid bin-check
bin-valid not-decimal not-binary written add bin-add` and what they nest, drawn with `fdp` (`tools/kbsub.py`). Each
text output ends with `[exit N]`.

| program | binary | artefacts |
|---|---|---|
| [`two_faults`](programs/two_faults.ein) | before | [read-out](runs/two_faults.before.solve.txt) · [summary](runs/two_faults.before.summary.json) · [events](runs/two_faults.before.events.jsonl) · [events table](runs/two_faults.before.events.md) · [trace](runs/two_faults.before.trace.md) · [dump-answer](runs/two_faults.before.answer/) · [saturate --dump](runs/two_faults.before.saturate-dump.txt) · [its facts](runs/two_faults.before.saturate-facts.txt) · [ein test](runs/two_faults.before.test.txt) · [show hyps](runs/two_faults.before.show-hyps.txt) · [kb-cut.dot](renders/two_faults.before.kb-cut.dot) · [kb-cut.svg](renders/two_faults.before.kb-cut.svg) |
| [`two_faults`](programs/two_faults.ein) | today | [read-out](runs/two_faults.today.solve.txt) · [summary](runs/two_faults.today.summary.json) · [events](runs/two_faults.today.events.jsonl) · [events table](runs/two_faults.today.events.md) · [trace](runs/two_faults.today.trace.md) · [dump-answer](runs/two_faults.today.answer/) · [saturate --dump](runs/two_faults.today.saturate-dump.txt) · [its facts](runs/two_faults.today.saturate-facts.txt) · [ein test](runs/two_faults.today.test.txt) · [show hyps](runs/two_faults.today.show-hyps.txt) · [kb-cut.dot](renders/two_faults.today.kb-cut.dot) · [kb-cut.svg](renders/two_faults.today.kb-cut.svg) |
| [`two_faults`](programs/two_faults.ein) | diff | [saturate-facts.diff](runs/two_faults.saturate-facts.diff) · [solve.diff](runs/two_faults.solve.diff) · [test.diff](runs/two_faults.test.diff) · [trace-steps.diff](runs/two_faults.trace-steps.diff) |
| [`bin_two_faults`](programs/bin_two_faults.ein) | before | [read-out](runs/bin_two_faults.before.solve.txt) · [summary](runs/bin_two_faults.before.summary.json) · [events](runs/bin_two_faults.before.events.jsonl) · [events table](runs/bin_two_faults.before.events.md) · [trace](runs/bin_two_faults.before.trace.md) · [dump-answer](runs/bin_two_faults.before.answer/) · [saturate --dump](runs/bin_two_faults.before.saturate-dump.txt) · [its facts](runs/bin_two_faults.before.saturate-facts.txt) · [ein test](runs/bin_two_faults.before.test.txt) · [show hyps](runs/bin_two_faults.before.show-hyps.txt) · [kb-cut.dot](renders/bin_two_faults.before.kb-cut.dot) · [kb-cut.svg](renders/bin_two_faults.before.kb-cut.svg) |
| [`bin_two_faults`](programs/bin_two_faults.ein) | today | [read-out](runs/bin_two_faults.today.solve.txt) · [summary](runs/bin_two_faults.today.summary.json) · [events](runs/bin_two_faults.today.events.jsonl) · [events table](runs/bin_two_faults.today.events.md) · [trace](runs/bin_two_faults.today.trace.md) · [dump-answer](runs/bin_two_faults.today.answer/) · [saturate --dump](runs/bin_two_faults.today.saturate-dump.txt) · [its facts](runs/bin_two_faults.today.saturate-facts.txt) · [ein test](runs/bin_two_faults.today.test.txt) · [show hyps](runs/bin_two_faults.today.show-hyps.txt) · [kb-cut.dot](renders/bin_two_faults.today.kb-cut.dot) · [kb-cut.svg](renders/bin_two_faults.today.kb-cut.svg) |
| [`bin_two_faults`](programs/bin_two_faults.ein) | diff | [saturate-facts.diff](runs/bin_two_faults.saturate-facts.diff) · [solve.diff](runs/bin_two_faults.solve.diff) · [test.diff](runs/bin_two_faults.test.diff) · [trace-steps.diff](runs/bin_two_faults.trace-steps.diff) |
| [`two_kinds_of_fault`](programs/two_kinds_of_fault.ein) | before | [read-out](runs/two_kinds_of_fault.before.solve.txt) · [summary](runs/two_kinds_of_fault.before.summary.json) · [events](runs/two_kinds_of_fault.before.events.jsonl) · [events table](runs/two_kinds_of_fault.before.events.md) · [trace](runs/two_kinds_of_fault.before.trace.md) · [dump-answer](runs/two_kinds_of_fault.before.answer/) · [saturate --dump](runs/two_kinds_of_fault.before.saturate-dump.txt) · [its facts](runs/two_kinds_of_fault.before.saturate-facts.txt) · [ein test](runs/two_kinds_of_fault.before.test.txt) · [show hyps](runs/two_kinds_of_fault.before.show-hyps.txt) · [kb-cut.dot](renders/two_kinds_of_fault.before.kb-cut.dot) · [kb-cut.svg](renders/two_kinds_of_fault.before.kb-cut.svg) |
| [`two_kinds_of_fault`](programs/two_kinds_of_fault.ein) | today | [read-out](runs/two_kinds_of_fault.today.solve.txt) · [summary](runs/two_kinds_of_fault.today.summary.json) · [events](runs/two_kinds_of_fault.today.events.jsonl) · [events table](runs/two_kinds_of_fault.today.events.md) · [trace](runs/two_kinds_of_fault.today.trace.md) · [dump-answer](runs/two_kinds_of_fault.today.answer/) · [saturate --dump](runs/two_kinds_of_fault.today.saturate-dump.txt) · [its facts](runs/two_kinds_of_fault.today.saturate-facts.txt) · [ein test](runs/two_kinds_of_fault.today.test.txt) · [show hyps](runs/two_kinds_of_fault.today.show-hyps.txt) · [kb-cut.dot](renders/two_kinds_of_fault.today.kb-cut.dot) · [kb-cut.svg](renders/two_kinds_of_fault.today.kb-cut.svg) |
| [`two_kinds_of_fault`](programs/two_kinds_of_fault.ein) | diff | [saturate-facts.diff](runs/two_kinds_of_fault.saturate-facts.diff) · [solve.diff](runs/two_kinds_of_fault.solve.diff) · [test.diff](runs/two_kinds_of_fault.test.diff) · [trace-steps.diff](runs/two_kinds_of_fault.trace-steps.diff) |

Also:

- Trace excerpts, the steps that ask, walk, label and flag (`tools/trace_excerpt.py`):
  `two_faults` [before](runs/two_faults.before.trace-check-steps.md) · [today](runs/two_faults.today.trace-check-steps.md);
  `bin_two_faults` [before](runs/bin_two_faults.before.trace-check-steps.md) · [today](runs/bin_two_faults.today.trace-check-steps.md);
  `two_kinds_of_fault` [before](runs/two_kinds_of_fault.before.trace-check-steps.md) · [today](runs/two_kinds_of_fault.today.trace-check-steps.md).
- [`runs/Q-M1f.274.decision.diff`](runs/Q-M1f.274.decision.diff) — the decision's text, `git diff 0a4a97a3 b5fb8d70`.
- The mocks: [`two_faults.option-b.MOCK.saturate-facts.txt`](runs/two_faults.option-b.MOCK.saturate-facts.txt),
  [`bin_two_faults.option-b.MOCK.saturate-facts.txt`](runs/bin_two_faults.option-b.MOCK.saturate-facts.txt).
- [`tools/`](tools/) — the scripts that made `runs/` and `renders/`: `run_all.sh` (every artefact of one program
  under one binary), `diffs.sh` (before → today diffs), `facts.py` (the facts of a dump that bear on the check),
  `jsonl2md.py` (events as a table), `kbsub.py` (the KB view cut by relation), `trace_excerpt.py` (a trace's steps
  of named rules), `steps_of.py` (a trace's steps that check or label one number).

## 6. Re-run

From the repo root:

```sh
D=plans/m1f_hypothesis_and_documentation/review/02-numerals/15-Q-M1f.411
build/review-bin/ein-head-b5fb8d70 saturate $D/programs/two_faults.ein --dump | grep -E 'not-decimal |decimal.check'
EIN_STDLIB=build/wt-rev-0a4/stdlib build/review-bin/ein-0a4a97a3 saturate $D/programs/two_faults.ein --dump | grep -E 'not-decimal |written'
# everything in runs/ and renders/ (the mocks are hand-made and are not re-made)
for p in two_faults bin_two_faults two_kinds_of_fault; do for L in before today; do
  KBRELS="check valid bin-check bin-valid not-decimal not-binary written add bin-add" $D/tools/run_all.sh $L $D/programs/$p.ein $D
done; done
$D/tools/diffs.sh $D two_faults bin_two_faults two_kinds_of_fault
```

## 7. Found while building

- **One flag is one flag a place, not one a numeral.** Two faults at the same place, `(n (n 1 x) 5)`, are flagged
  twice today as before (`numeral-digit` and `numeral-prefix-digit` both read `(check W (n 1 x))`). The question's
  title, *a numeral with two faults has one flag*, holds for faults at two places only.
- The decision Q-M1f.274 cites `arith_theory.rs`' `a_malformed_request_is_flagged_once_and_never_answered` as holding
  *each numeral's one flag*; not run here (no `cargo test` in this review).

## 8. Your call

- **(a)** one flag, at the first place at fault from the units — **the default, as built**.
- **(b)** every place at fault flagged — a second walk relation or a fault bit.
