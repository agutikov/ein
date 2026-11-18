# Q-M1f.405 — which numerals are "taken from the input program", and what "once" is

## 1. The question

The ruling that erased `written` (Q-M1f.404) says: *"all numbers taken from input program will be checked once and
labeled valid"*. Two words need a reading. **Taken**: which numerals get checked — only an operand that an operation
looks up, or also a numeral a program hands straight to one of the calculator's internal *tagged calls*
(`(add-walk TAG X Y C z)` and its four siblings, the interface a theory built on the calculator uses), or every
numeral the knowledge base holds? **Once**: is the check a fact of the numeral (walked once, whoever asks), or of
each request (walked again for every request naming it)? Under the default, one answer moved outside the corpus: a
malformed numeral handed straight to a tagged call was dead before and is alive now.

Recorded in [`../../open_questions.md` § Q-M1f.405](../../../open_questions.md#q-m1f405--which-numerals-are-taken-from-the-input-program-and-what-once-is);
the build that took the default is S1f.22.6, whose record is read from git
(`git show b5fb8d70^:plans/m1f_hypothesis_and_documentation/p1f.22_nested_facts/s1f.22.6_written_erased.md`,
§ *Not done, or uncertain*, first bullet).

## 2. What is so today

Every program below was run under two binaries: **before** = `build/review-bin/ein-0a4a97a3` with
`EIN_STDLIB=build/wt-rev-0a4/stdlib` (the calculator with `written`), **today** = `build/review-bin/ein-head-17085b27`
(master's code, the drawing's stages merged). Each run's every artefact is in `runs/`, and `*.diff` is `diff -u` before → today.

**The case that moved** — [`programs/e_tagged_call_malformed.ein`](programs/e_tagged_call_malformed.ein), rebuilt
from the record: `(add-walk (mine a) (n z x) (n z 1) 0 z)`, `x` where a digit goes, handed straight to the tagged
call. Its claims are today's answer.

- **Before: dead.** The loader stated `(written (n z x))`, the flag read it
  ([facts](runs/e_tagged_call_malformed.before.saturate-facts.txt)):

  <!-- output -->
  ```
    (false :rule std.math.decimal.not-decimal-is-false)
    (not-decimal (n z x) x :rule std.math.decimal.numeral-digit)
    (written (mine a) :rule "#written")
    (written (n z x) :rule "#written")
    (written (n z 1) :rule "#written")
  ```
  and the read-out ([solve](runs/e_tagged_call_malformed.before.solve.txt)) says `states (k_s) 0`,
  `unsat core (add-walk (mine a) (n z x) (n z 1) 0 z)`, `refuted by std.math.decimal.not-decimal-is-false`.
- **Today: alive, R = {∅}, and nothing is answered.** A call looks nothing up, so no operand is found, no `check` is
  asked; the walk finds no row of the addition table for `x` and stalls. The only derived fact the filter keeps is
  `(std.math.decimal.valid z :rule std.math.decimal.decimal)`
  ([facts](runs/e_tagged_call_malformed.today.saturate-facts.txt)); `states (k_s) 1`, `(done ?tag ?s)` has no row
  ([solve](runs/e_tagged_call_malformed.today.solve.txt)). The diffs:
  [read-out](runs/e_tagged_call_malformed.solve.diff), [facts](runs/e_tagged_call_malformed.saturate-facts.diff),
  [trace steps](runs/e_tagged_call_malformed.trace-steps.diff) (before: `numeral-digit` and `not-decimal-is-false`
  fire at root; today: neither), [ein test](runs/e_tagged_call_malformed.test.diff) (before 3 FAILED, exit 1; today
  3 held, exit 0). The KB views cut to the check's relations:
  [before](renders/e_tagged_call_malformed.before.kb-cut.svg), [today](renders/e_tagged_call_malformed.today.kb-cut.svg).
  Today's is drawn as the graph has it: the call an octagon with no name in it, its arrow 1 at the box `add-walk`
  and its arrows 2 to 4 at `(mine a)`, `(n z x)` and `(n z 1)`, three octagons in the frame `add-walk ctx`; and
  `(valid z)` an octagon whose arrow 1 is at `std.math.decimal.valid` and arrow 2 at `z`, dashed, with
  `by std.math.decimal.decimal` beside it. Before's is the earlier binary's drawing: labelled octagons,
  `(add-walk)`, a `(written)` over each of the three, and the flag an arrow labelled `not-decimal` from `(n z x)` to
  `x`.

**"Once", as built** — [`programs/a_once.ein`](programs/a_once.ein): 12 is named by three requests (as an operand,
as `price`'s value, and in a request the program's rule `ask` derives). The trace excerpt
[`runs/a_once.today.trace-check-steps.md`](runs/a_once.today.trace-check-steps.md) shows `operand-check` firing for
12 at steps 12, 14 and 29, each deriving the same `check((n (n z 1) 2), (n (n z 1) 2))`, and the walk of 12 running
once: step 18 (`numeral-step`, the digit 2), step 23 (the digit 1), step 31 (`numeral-valid`). The KB holds three
`check` facts of 12 and one `(valid (n (n z 1) 2))` ([facts](runs/a_once.today.saturate-facts.txt)). The answer is
the same under both binaries ([read-out diff](runs/a_once.solve.diff): no difference).

**A rule's request is taken too** — [`programs/b_rule_writes_the_request.ein`](programs/b_rule_writes_the_request.ein),
a copy of the corpus's `tests/stdlib/math/decimal/07_in_a_rules_conclusion_that_fires.ein`: the rule `assemble`
derives `(add (n (n ?s 1) x) (n z 5))`; today `operand-check` asks the check and `numeral-digit` flags it, root dead
([today](runs/b_rule_writes_the_request.today.solve.txt)). Before, the same program was **alive**
([before](runs/b_rule_writes_the_request.before.solve.txt), its `(false)` claim FAILED): the loader's `written`
step stated no fact a rule derives with a variable, so the assembled numeral was never flagged.

**A numeral no operation takes** — [`programs/c_fact_of_the_programs_own.ein`](programs/c_fact_of_the_programs_own.ein),
a copy of `tests/stdlib/math/decimal/03_in_a_fact_of_the_programs_own.ein`: `(price apple (n 1 2))`. Before dead
(unsat core `(price apple (n 1 2))`, `(std.math.digits.nz 1)`), today R = {∅}
([diff](runs/c_fact_of_the_programs_own.solve.diff)). This is Q-M1f.407's subject; see `../12-Q-M1f.407/`.

**What the calculator hands itself** — [`programs/e_cost_product.ein`](programs/e_cost_product.ein), 47 × 38. Inside,
the theory calls `(mul-call TAG 47 38)` and `(add-walk TAG 1410 376 0 z)` (the two rows' sum). Today only 47 and 38
are walked; 1410, 376 and 1786 get their label from `turn-end` as they are made
([facts diff](runs/e_cost_product.saturate-facts.diff)). The answer is unchanged ([read-out diff](runs/e_cost_product.solve.diff)).

## 3. The options

- **(a) a numeral an operation takes** — an operand found by the look-up (written, or a `let`'s value), a part of an
  integer or a rational an operation takes, a number `to-bin`, `to-dec` and `to9` take; whoever wrote the request.
  *Once* = the ask `(check N N)` and the label `(valid N)` are facts of the numeral. **This is what runs today**
  (above). Its moved case: `e_tagged_call_malformed` alive.
- **(b) only the numerals of requests the file writes.** `b_rule_writes_the_request` would be alive: mock
  [`runs/b_rule_writes_the_request.option-b.MOCK.saturate-facts.txt`](runs/b_rule_writes_the_request.option-b.MOCK.saturate-facts.txt)
  (today's facts with the five `check` facts, the two walk labels, the flag and `(false)` deleted; the sum's label by
  `turn-end` kept) and [`…option-b.MOCK.solve.txt`](runs/b_rule_writes_the_request.option-b.MOCK.solve.txt) (the
  alive read-out, its lines the before binary's). It needs a statement of what the file wrote — the `written`
  facts the before run shows are exactly such a statement, which the ruling erased.
- **(c) every numeral the knowledge base holds.** `c_fact_of_the_programs_own` would be dead again: mock
  [`runs/c_fact_of_the_programs_own.option-c.MOCK.saturate-facts.txt`](runs/c_fact_of_the_programs_own.option-c.MOCK.saturate-facts.txt)
  (three lines added: an ask of `(n 1 2)` by a rule that reads every fact, named `every-numeral` there because no
  module has one, its flag and `(false)`). The read-out (c) gives is the before binary's
  ([before](runs/c_fact_of_the_programs_own.before.solve.txt)). It needs a walk of every fact for its nested terms —
  the loader's step again, or a rule over every relation.
- **(d) once a request**: the check keyed by the operand's tag. In `a_once`, 12 would be walked three times, 3
  twice, 5 once: mock [`runs/a_once.option-d.MOCK.saturate-facts.txt`](runs/a_once.option-d.MOCK.saturate-facts.txt)
  (today's seven `check` facts replaced by fifteen `(check TAG N REST)`). No answer changes; the work grows with the
  number of requests that name a numeral.
- **(e) as (a), and every tagged call checks what it is handed.** `e_tagged_call_malformed` would be dead, as before:
  mocks [`runs/e_tagged_call_malformed.option-e.MOCK.saturate-facts.txt`](runs/e_tagged_call_malformed.option-e.MOCK.saturate-facts.txt)
  (the asks of `(n z x)` and `(n z 1)`, the walk of `(n z 1)`, the flag and `(false)` added, by a rule named
  `call-check` there) and [`…option-e.MOCK.solve.txt`](runs/e_tagged_call_malformed.option-e.MOCK.solve.txt) (the
  dead read-out, its lines the before binary's). The cost is every number the calculator hands itself walked again:
  in 47 × 38 that is 1410 and 376, nine `check` facts a firing each —
  mock [`runs/e_cost_product.option-e.MOCK.saturate-facts.txt`](runs/e_cost_product.option-e.MOCK.saturate-facts.txt).
  A longer product or a quotient hands itself a number a row or a step.

## 4. The default

**(a)**, built by S1f.22.6 (`bf9693dc`), stated by Q-M1f.404's decision. The ledger gives no reason beyond what
it says of the others: (b) needs a statement of what the file wrote, *the context the ruling erased*; (c) needs *the
loader's step again, or a rule over every relation*; under (e) *every intermediate of a product and a quotient is
walked again, a firing a place*. The record adds that no corpus program hands a malformed numeral to a tagged call,
so the stage's stop condition, a corpus answer moving, did not hold.

## 5. Files

Programs: `e_tagged_call_malformed`, `a_once` and `e_cost_product` are written for this review (each claims today's
answer); `b_rule_writes_the_request` is a copy of `tests/stdlib/math/decimal/07_in_a_rules_conclusion_that_fires.ein`
and `c_fact_of_the_programs_own` of `tests/stdlib/math/decimal/03_in_a_fact_of_the_programs_own.ein`, both as at
`17085b27`.

Per program and binary: *read-out* `ein solve`; *summary* `--json-summary`; *events* `--events` (JSONL) and its
*events table* (markdown, by `tools/jsonl2md.py`); *trace* `--trace`; *dump-answer* `--dump-answer DIR`; *saturate
--dump* the whole dump and *its facts* (the facts that bear on the check: no digit table, declaration, closure, rule
list or macro activation; `tools/facts.py`); `ein test`; `ein show hyps`; *kb-cut* `ein render kb` cut to the facts
of `check valid not-decimal written add let sum-of add-walk done price mul mul-call product-of` and what they nest
(`tools/kbsub.py`, every line kept the binary's own; it reads both binaries' forms, a fact's relation the box under
its arrow 1 today and an octagon's label before), drawn with `fdp`. The whole KB view is not kept: it is
mostly the digit tables, some 10 000 lines, and `fdp` takes minutes to draw it. Each text output ends with the
command's exit code, `[exit N]`, added by `tools/run_all.sh`.

| program | binary | artefacts |
|---|---|---|
| [`e_tagged_call_malformed`](programs/e_tagged_call_malformed.ein) | before | [read-out](runs/e_tagged_call_malformed.before.solve.txt) · [summary](runs/e_tagged_call_malformed.before.summary.json) · [events](runs/e_tagged_call_malformed.before.events.jsonl) · [events table](runs/e_tagged_call_malformed.before.events.md) · [trace](runs/e_tagged_call_malformed.before.trace.md) · [dump-answer](runs/e_tagged_call_malformed.before.answer/) · [saturate --dump](runs/e_tagged_call_malformed.before.saturate-dump.txt) · [its facts](runs/e_tagged_call_malformed.before.saturate-facts.txt) · [ein test](runs/e_tagged_call_malformed.before.test.txt) · [show hyps](runs/e_tagged_call_malformed.before.show-hyps.txt) · [kb-cut.dot](renders/e_tagged_call_malformed.before.kb-cut.dot) · [kb-cut.svg](renders/e_tagged_call_malformed.before.kb-cut.svg) |
| [`e_tagged_call_malformed`](programs/e_tagged_call_malformed.ein) | today | [read-out](runs/e_tagged_call_malformed.today.solve.txt) · [summary](runs/e_tagged_call_malformed.today.summary.json) · [events](runs/e_tagged_call_malformed.today.events.jsonl) · [events table](runs/e_tagged_call_malformed.today.events.md) · [trace](runs/e_tagged_call_malformed.today.trace.md) · [dump-answer](runs/e_tagged_call_malformed.today.answer/) · [saturate --dump](runs/e_tagged_call_malformed.today.saturate-dump.txt) · [its facts](runs/e_tagged_call_malformed.today.saturate-facts.txt) · [ein test](runs/e_tagged_call_malformed.today.test.txt) · [show hyps](runs/e_tagged_call_malformed.today.show-hyps.txt) · [kb-cut.dot](renders/e_tagged_call_malformed.today.kb-cut.dot) · [kb-cut.svg](renders/e_tagged_call_malformed.today.kb-cut.svg) |
| [`e_tagged_call_malformed`](programs/e_tagged_call_malformed.ein) | diff | [saturate-facts.diff](runs/e_tagged_call_malformed.saturate-facts.diff) · [solve.diff](runs/e_tagged_call_malformed.solve.diff) · [test.diff](runs/e_tagged_call_malformed.test.diff) · [trace-steps.diff](runs/e_tagged_call_malformed.trace-steps.diff) |
| [`a_once`](programs/a_once.ein) | before | [read-out](runs/a_once.before.solve.txt) · [summary](runs/a_once.before.summary.json) · [events](runs/a_once.before.events.jsonl) · [events table](runs/a_once.before.events.md) · [trace](runs/a_once.before.trace.md) · [dump-answer](runs/a_once.before.answer/) · [saturate --dump](runs/a_once.before.saturate-dump.txt) · [its facts](runs/a_once.before.saturate-facts.txt) · [ein test](runs/a_once.before.test.txt) · [show hyps](runs/a_once.before.show-hyps.txt) · [kb-cut.dot](renders/a_once.before.kb-cut.dot) · [kb-cut.svg](renders/a_once.before.kb-cut.svg) |
| [`a_once`](programs/a_once.ein) | today | [read-out](runs/a_once.today.solve.txt) · [summary](runs/a_once.today.summary.json) · [events](runs/a_once.today.events.jsonl) · [events table](runs/a_once.today.events.md) · [trace](runs/a_once.today.trace.md) · [dump-answer](runs/a_once.today.answer/) · [saturate --dump](runs/a_once.today.saturate-dump.txt) · [its facts](runs/a_once.today.saturate-facts.txt) · [ein test](runs/a_once.today.test.txt) · [show hyps](runs/a_once.today.show-hyps.txt) · [kb-cut.dot](renders/a_once.today.kb-cut.dot) · [kb-cut.svg](renders/a_once.today.kb-cut.svg) |
| [`a_once`](programs/a_once.ein) | diff | [saturate-facts.diff](runs/a_once.saturate-facts.diff) · [solve.diff](runs/a_once.solve.diff) · [test.diff](runs/a_once.test.diff) · [trace-steps.diff](runs/a_once.trace-steps.diff) |
| [`b_rule_writes_the_request`](programs/b_rule_writes_the_request.ein) | before | [read-out](runs/b_rule_writes_the_request.before.solve.txt) · [summary](runs/b_rule_writes_the_request.before.summary.json) · [events](runs/b_rule_writes_the_request.before.events.jsonl) · [events table](runs/b_rule_writes_the_request.before.events.md) · [trace](runs/b_rule_writes_the_request.before.trace.md) · [dump-answer](runs/b_rule_writes_the_request.before.answer/) · [saturate --dump](runs/b_rule_writes_the_request.before.saturate-dump.txt) · [its facts](runs/b_rule_writes_the_request.before.saturate-facts.txt) · [ein test](runs/b_rule_writes_the_request.before.test.txt) · [show hyps](runs/b_rule_writes_the_request.before.show-hyps.txt) · [kb-cut.dot](renders/b_rule_writes_the_request.before.kb-cut.dot) · [kb-cut.svg](renders/b_rule_writes_the_request.before.kb-cut.svg) |
| [`b_rule_writes_the_request`](programs/b_rule_writes_the_request.ein) | today | [read-out](runs/b_rule_writes_the_request.today.solve.txt) · [summary](runs/b_rule_writes_the_request.today.summary.json) · [events](runs/b_rule_writes_the_request.today.events.jsonl) · [events table](runs/b_rule_writes_the_request.today.events.md) · [trace](runs/b_rule_writes_the_request.today.trace.md) · [dump-answer](runs/b_rule_writes_the_request.today.answer/) · [saturate --dump](runs/b_rule_writes_the_request.today.saturate-dump.txt) · [its facts](runs/b_rule_writes_the_request.today.saturate-facts.txt) · [ein test](runs/b_rule_writes_the_request.today.test.txt) · [show hyps](runs/b_rule_writes_the_request.today.show-hyps.txt) · [kb-cut.dot](renders/b_rule_writes_the_request.today.kb-cut.dot) · [kb-cut.svg](renders/b_rule_writes_the_request.today.kb-cut.svg) |
| [`b_rule_writes_the_request`](programs/b_rule_writes_the_request.ein) | diff | [saturate-facts.diff](runs/b_rule_writes_the_request.saturate-facts.diff) · [solve.diff](runs/b_rule_writes_the_request.solve.diff) · [test.diff](runs/b_rule_writes_the_request.test.diff) · [trace-steps.diff](runs/b_rule_writes_the_request.trace-steps.diff) |
| [`c_fact_of_the_programs_own`](programs/c_fact_of_the_programs_own.ein) | before | [read-out](runs/c_fact_of_the_programs_own.before.solve.txt) · [summary](runs/c_fact_of_the_programs_own.before.summary.json) · [events](runs/c_fact_of_the_programs_own.before.events.jsonl) · [events table](runs/c_fact_of_the_programs_own.before.events.md) · [trace](runs/c_fact_of_the_programs_own.before.trace.md) · [dump-answer](runs/c_fact_of_the_programs_own.before.answer/) · [saturate --dump](runs/c_fact_of_the_programs_own.before.saturate-dump.txt) · [its facts](runs/c_fact_of_the_programs_own.before.saturate-facts.txt) · [ein test](runs/c_fact_of_the_programs_own.before.test.txt) · [show hyps](runs/c_fact_of_the_programs_own.before.show-hyps.txt) · [kb-cut.dot](renders/c_fact_of_the_programs_own.before.kb-cut.dot) · [kb-cut.svg](renders/c_fact_of_the_programs_own.before.kb-cut.svg) |
| [`c_fact_of_the_programs_own`](programs/c_fact_of_the_programs_own.ein) | today | [read-out](runs/c_fact_of_the_programs_own.today.solve.txt) · [summary](runs/c_fact_of_the_programs_own.today.summary.json) · [events](runs/c_fact_of_the_programs_own.today.events.jsonl) · [events table](runs/c_fact_of_the_programs_own.today.events.md) · [trace](runs/c_fact_of_the_programs_own.today.trace.md) · [dump-answer](runs/c_fact_of_the_programs_own.today.answer/) · [saturate --dump](runs/c_fact_of_the_programs_own.today.saturate-dump.txt) · [its facts](runs/c_fact_of_the_programs_own.today.saturate-facts.txt) · [ein test](runs/c_fact_of_the_programs_own.today.test.txt) · [show hyps](runs/c_fact_of_the_programs_own.today.show-hyps.txt) · [kb-cut.dot](renders/c_fact_of_the_programs_own.today.kb-cut.dot) · [kb-cut.svg](renders/c_fact_of_the_programs_own.today.kb-cut.svg) |
| [`c_fact_of_the_programs_own`](programs/c_fact_of_the_programs_own.ein) | diff | [saturate-facts.diff](runs/c_fact_of_the_programs_own.saturate-facts.diff) · [solve.diff](runs/c_fact_of_the_programs_own.solve.diff) · [test.diff](runs/c_fact_of_the_programs_own.test.diff) · [trace-steps.diff](runs/c_fact_of_the_programs_own.trace-steps.diff) |
| [`e_cost_product`](programs/e_cost_product.ein) | before | [read-out](runs/e_cost_product.before.solve.txt) · [summary](runs/e_cost_product.before.summary.json) · [events](runs/e_cost_product.before.events.jsonl) · [events table](runs/e_cost_product.before.events.md) · [trace](runs/e_cost_product.before.trace.md) · [dump-answer](runs/e_cost_product.before.answer/) · [saturate --dump](runs/e_cost_product.before.saturate-dump.txt) · [its facts](runs/e_cost_product.before.saturate-facts.txt) · [ein test](runs/e_cost_product.before.test.txt) · [show hyps](runs/e_cost_product.before.show-hyps.txt) · [kb-cut.dot](renders/e_cost_product.before.kb-cut.dot) · [kb-cut.svg](renders/e_cost_product.before.kb-cut.svg) |
| [`e_cost_product`](programs/e_cost_product.ein) | today | [read-out](runs/e_cost_product.today.solve.txt) · [summary](runs/e_cost_product.today.summary.json) · [events](runs/e_cost_product.today.events.jsonl) · [events table](runs/e_cost_product.today.events.md) · [trace](runs/e_cost_product.today.trace.md) · [dump-answer](runs/e_cost_product.today.answer/) · [saturate --dump](runs/e_cost_product.today.saturate-dump.txt) · [its facts](runs/e_cost_product.today.saturate-facts.txt) · [ein test](runs/e_cost_product.today.test.txt) · [show hyps](runs/e_cost_product.today.show-hyps.txt) · [kb-cut.dot](renders/e_cost_product.today.kb-cut.dot) · [kb-cut.svg](renders/e_cost_product.today.kb-cut.svg) |
| [`e_cost_product`](programs/e_cost_product.ein) | diff | [saturate-facts.diff](runs/e_cost_product.saturate-facts.diff) · [solve.diff](runs/e_cost_product.solve.diff) · [test.diff](runs/e_cost_product.test.diff) · [trace-steps.diff](runs/e_cost_product.trace-steps.diff) |

Also:

- [`runs/a_once.today.trace-check-steps.md`](runs/a_once.today.trace-check-steps.md) — the steps of the trace that
  ask, walk and label (`tools/trace_excerpt.py`).
- [`runs/e_tagged_call_malformed.before.trace-check-steps.md`](runs/e_tagged_call_malformed.before.trace-check-steps.md)
  — the two steps of the before trace that flag `(n z x)` and derive `(false)`.
- The mocks (hand-edited, each says at its top what was edited):
  [`a_once.option-d.MOCK.saturate-facts.txt`](runs/a_once.option-d.MOCK.saturate-facts.txt),
  [`b_rule_writes_the_request.option-b.MOCK.saturate-facts.txt`](runs/b_rule_writes_the_request.option-b.MOCK.saturate-facts.txt),
  [`b_rule_writes_the_request.option-b.MOCK.solve.txt`](runs/b_rule_writes_the_request.option-b.MOCK.solve.txt),
  [`c_fact_of_the_programs_own.option-c.MOCK.saturate-facts.txt`](runs/c_fact_of_the_programs_own.option-c.MOCK.saturate-facts.txt),
  [`e_cost_product.option-e.MOCK.saturate-facts.txt`](runs/e_cost_product.option-e.MOCK.saturate-facts.txt),
  [`e_tagged_call_malformed.option-e.MOCK.saturate-facts.txt`](runs/e_tagged_call_malformed.option-e.MOCK.saturate-facts.txt),
  [`e_tagged_call_malformed.option-e.MOCK.solve.txt`](runs/e_tagged_call_malformed.option-e.MOCK.solve.txt).
- [`tools/`](tools/) — the scripts that made `runs/` and `renders/`: `run_all.sh` (every artefact of one program
  under one binary), `diffs.sh` (before → today diffs), `facts.py` (the facts of a dump that bear on the check),
  `jsonl2md.py` (events as a table), `kbsub.py` (the KB view cut by relation), `trace_excerpt.py` (a trace's steps
  of named rules), `steps_of.py` (a trace's steps that check or label one number).

## 6. Re-run

From the repo root:

```sh
D=plans/m1f_hypothesis_and_documentation/review/02-numerals/04-Q-M1f.405
# one program, by hand, under each binary
build/review-bin/ein-head-17085b27 solve $D/programs/e_tagged_call_malformed.ein
EIN_STDLIB=build/wt-rev-0a4/stdlib build/review-bin/ein-0a4a97a3 solve $D/programs/e_tagged_call_malformed.ein
build/review-bin/ein-head-17085b27 saturate $D/programs/e_tagged_call_malformed.ein --dump
# everything in runs/ and renders/ (the mocks are hand-made and are not re-made)
for p in e_tagged_call_malformed a_once b_rule_writes_the_request c_fact_of_the_programs_own e_cost_product; do
  for L in before today; do
    KBRELS="check valid not-decimal written add let sum-of add-walk done price mul mul-call product-of" \
      $D/tools/run_all.sh $L $D/programs/$p.ein $D
  done
done
$D/tools/diffs.sh $D e_tagged_call_malformed a_once b_rule_writes_the_request c_fact_of_the_programs_own e_cost_product
python3 -I $D/tools/trace_excerpt.py $D/runs/a_once.today.trace.md $D/runs/a_once.today.trace-check-steps.md \
  operand-check numeral-step numeral-step-zero numeral-valid turn-end
```

## 7. Found while building

- The record's case reproduces as stated: `states (k_s) 1` today, `0` before — **but only with `(__closed__ mine)`**.
  Without it the blind enumerator guesses `(mine …)` facts and today's R has 32 768 states (before: still 0). The
  record does not say how its program closed the tag's relation; this one closes it.
- `tests/stdlib/math/decimal/07_in_a_rules_conclusion_that_fires.ein`, added by the same build, is answered
  **alive** by the before binary: its `(false)` claim holds only under today's design. The record's "no other
  program's R moved" is about the programs that existed before, so this is no contradiction, but it shows (a)
  flags something the `written` design never did — a numeral a rule assembles with a variable.
- **Re-taken with `ein-head-17085b27`** (the first look ran `ein-head-b5fb8d70`; the standard library and
  `tests/stdlib/` are the same in both trees, `git diff --stat b5fb8d70 17085b27 -- stdlib tests/stdlib` is empty):
  every read-out, summary, event stream, dump, `ein test` and `ein show hyps` output is the same, wall-clock aside,
  and so is every trace's text and every excerpt of it; what moved is the drawings, each trace's `dot` blocks and
  today's KB cuts, now in the canonical form. The before binary's outputs re-run the same.

## 8. Your call

- **(a)** a numeral an operation takes, whoever wrote the request; once a numeral — **the default, as built**.
- **(b)** only the requests the file writes — needs a statement of what the file wrote.
- **(c)** every numeral the knowledge base holds — needs a walk of every fact.
- **(d)** once a request — the check keyed by the tag, walked again per request.
- **(e)** as (a), and every tagged call checks its numbers — `e_tagged_call_malformed` dead again; every intermediate
  walked again.
