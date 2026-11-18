# Q-M1f.407 — a numeral no operation takes is not checked

## 1. The question

Since `written` was erased, a numeral is checked only when an operation takes it. So a malformed numeral that no
operation takes — the value of a `let` nobody asks about, a numeral in a fact of the program's own relation, a numeral
in a rule's conclusion that never fires — is no longer flagged. Three corpus programs said it was: each was dead
before and is R = {∅} now, and each was rewritten to claim that. Is *not checked* right, and should those programs be
rewritten (as built), deleted, or should a `let`'s value at least be checked?

Recorded in [`../../open_questions.md` § Q-M1f.407](../../../open_questions.md#q-m1f407--a-numeral-no-operation-takes-is-not-checked);
the build is S1f.22.6, commit `bf9693dc` (`ec9049b5`, the other code commit of the stage, touches only the alive
invariant's banked list and none of these programs).

## 2. What is so today

The three programs, each in two texts — **v-before** as at `0a4a97a3` (claims `(false)`) and **v-today** as at
`b5fb8d70` (claims R = {∅}) — each run under two binaries: **before** = `build/review-bin/ein-0a4a97a3` with
`EIN_STDLIB=build/wt-rev-0a4/stdlib`, **today** = `build/review-bin/ein-head-b5fb8d70`. Plus `07`, the twin the build
added. `ein test`'s line for each:

| program | text | before binary | today binary |
|---|---|---|---|
| `02_in_a_let_nobody_asks_about` — `(let price (n (n z 1) x))` | v-before, `(solve :expect (false))` | dead, unsat core `(let price (n (n z 1) x))`: [1 held](runs/02_in_a_let_nobody_asks_about.v-before.before.test.txt) | R = {∅}: [1 FAILED](runs/02_in_a_let_nobody_asks_about.v-before.today.test.txt) |
| | v-today, R = {∅} and a sum | refused at load, the module provides no `valid`: [1 error](runs/02_in_a_let_nobody_asks_about.v-today.before.test.txt) | [4 held](runs/02_in_a_let_nobody_asks_about.v-today.today.test.txt) |
| `03_in_a_fact_of_the_programs_own` — `(price apple (n 1 2))` | v-before, `(false)` | dead, unsat core `(price apple (n 1 2))`, `(std.math.digits.nz 1)`: [1 held](runs/03_in_a_fact_of_the_programs_own.v-before.before.test.txt) | R = {∅}: [1 FAILED](runs/03_in_a_fact_of_the_programs_own.v-before.today.test.txt) |
| | v-today | dead: [3 FAILED](runs/03_in_a_fact_of_the_programs_own.v-today.before.test.txt) | [3 held](runs/03_in_a_fact_of_the_programs_own.v-today.today.test.txt) |
| `04_in_a_rules_conclusion` — `(add (n 0 3) (n z 5))` concluded by a rule that never fires | v-before, `(false)` | dead, unsat core ∅: [1 held](runs/04_in_a_rules_conclusion.v-before.before.test.txt) | R = {∅}: [1 FAILED](runs/04_in_a_rules_conclusion.v-before.today.test.txt) |
| | v-today | dead: [3 FAILED](runs/04_in_a_rules_conclusion.v-today.before.test.txt) | [3 held](runs/04_in_a_rules_conclusion.v-today.today.test.txt) |
| `07_in_a_rules_conclusion_that_fires` (new) — a rule hands `(n (n z 1) x)` to a request | v-today, `(false)` | R = {∅}: [1 FAILED](runs/07_in_a_rules_conclusion_that_fires.v-today.before.test.txt) | dead: [1 held](runs/07_in_a_rules_conclusion_that_fires.v-today.today.test.txt) |

**The answer moved on the same text**: the v-before rows. E.g.
[`02….v-before.solve.diff`](runs/02_in_a_let_nobody_asks_about.v-before.solve.diff):

<!-- output -->
```
-  states (k_s)      0
+  states (k_s)      1
-  empty             every node is dead — root's own fixpoint derives (false)
+  ∅                 in R — root is alive and owes nothing
-  unsat core        (let price (n (n z 1) x))
-  refuted by        std.math.decimal.not-decimal-is-false
-  (solve :expect)   holds
+  (solve :expect)   FAILED
+                    (false) says every node is dead, and R has 1 state
```

**What the knowledge base held, before and today** — e.g.
[`04….corpus-before-to-today.saturate-facts.diff`](runs/04_in_a_rules_conclusion.corpus-before-to-today.saturate-facts.diff)
(v-before under before against v-today under today): before, the loader stated `(written (n 0 3))` from the rule's
text and `numeral-prefix-zero` flagged it with nothing firing; today `(n 0 3)` appears in no fact, and the sum beside
it is walked, labelled and answered. The other two:
[02](runs/02_in_a_let_nobody_asks_about.corpus-before-to-today.saturate-facts.diff),
[03](runs/03_in_a_fact_of_the_programs_own.corpus-before-to-today.saturate-facts.diff).

**The `:expect` before and after**: the programs' own diffs, `git diff 0a4a97a3 b5fb8d70 -- FILE`:
[02](runs/02_in_a_let_nobody_asks_about.program.diff) (`(solve :expect (false))` → `(solve :expect (region
(model)))`, a sum added, and three queries: the sum, `not-decimal`'s empty extent, `valid`'s extent without the
`let`'s numeral), [03](runs/03_in_a_fact_of_the_programs_own.program.diff) and
[04](runs/04_in_a_rules_conclusion.program.diff) (the same, without the `valid` query). The corpus's notes of the
four entries: [`runs/corpus-notes.diff`](runs/corpus-notes.diff).

## 3. The options

- **(a) not checked**; the three programs rewritten to claim R = {∅}, no flag and no label, with a sum answered
  beside each; `07` added, whose rule fires and hands its numeral to a request, which is flagged. **What runs today.**
- **(b) the same, the three programs deleted.** No answer differs from (a); the suite loses the three programs that
  say a numeral no operation takes is silent. Mock: [`runs/decimal-folder.option-b.MOCK.txt`](runs/decimal-folder.option-b.MOCK.txt),
  the folder's listing today and under (b).
- **(c) a `let`'s value checked as the `let` is read**, `let` being the calculator's own relation: `02` dead again,
  `03` and `04` alive as today. Mock: [`runs/02_in_a_let_nobody_asks_about.option-c.MOCK.saturate-facts.txt`](runs/02_in_a_let_nobody_asks_about.option-c.MOCK.saturate-facts.txt)
  (the ask of `price`'s value, by a rule named `let-check` there, its flag and `(false)` added); the read-out (c)
  gives `02` is the before binary's on the v-before text,
  [`02….v-before.before.solve.txt`](runs/02_in_a_let_nobody_asks_about.v-before.before.solve.txt), unsat core
  `(let price (n (n z 1) x))`.

## 4. The default

**(a)**, built by S1f.22.6 (`bf9693dc`), stated by Q-M1f.404's decision. From the record: *a numeral no operation
takes is not checked* is the consequence of putting the detection into the operations, which is what the ruling asked;
the stage file allowed the three programs to be "changed to say that it is not, or deleted", and the build kept them,
each saying it, with a sum beside it so that the program still exercises the check on the numerals that are taken.
Binary has no such program: its malformed numerals are all in requests, and none changed.

## 5. Files

Programs: copies of `tests/stdlib/math/decimal/02_…`, `03_…` and `04_…` as at `0a4a97a3` (`*.v-before.ein`) and as at
`b5fb8d70` (`*.v-today.ein`), and of `07_…` as at `b5fb8d70` (it did not exist before).

Per program text and binary: *read-out* `ein solve`; *summary* `--json-summary`; *events* `--events` (JSONL) and its
*events table*; *trace* `--trace`; *dump-answer* `--dump-answer DIR`; *saturate --dump* whole and *its facts* (the
facts that bear on the check, `tools/facts.py`); `ein test`; `ein show hyps`; *kb-cut* `ein render kb` cut to the
facts of `check valid not-decimal written let add sum-of price seed` and what they nest, drawn with `fdp`
(`tools/kbsub.py`; the whole view is mostly the digit tables and is not kept). Each text output ends with
`[exit N]`. The diff row is the same text under the before binary against the today binary. The v-today text of
`02` is refused by the before binary, so that run has a read-out, `ein test`, `saturate --dump`, `show hyps` and a
`kb.err` and nothing else.

| program | binary | artefacts |
|---|---|---|
| [`02_in_a_let_nobody_asks_about.v-before`](programs/02_in_a_let_nobody_asks_about.v-before.ein) | before | [read-out](runs/02_in_a_let_nobody_asks_about.v-before.before.solve.txt) · [summary](runs/02_in_a_let_nobody_asks_about.v-before.before.summary.json) · [events](runs/02_in_a_let_nobody_asks_about.v-before.before.events.jsonl) · [events table](runs/02_in_a_let_nobody_asks_about.v-before.before.events.md) · [trace](runs/02_in_a_let_nobody_asks_about.v-before.before.trace.md) · [dump-answer](runs/02_in_a_let_nobody_asks_about.v-before.before.answer/) · [saturate --dump](runs/02_in_a_let_nobody_asks_about.v-before.before.saturate-dump.txt) · [its facts](runs/02_in_a_let_nobody_asks_about.v-before.before.saturate-facts.txt) · [ein test](runs/02_in_a_let_nobody_asks_about.v-before.before.test.txt) · [show hyps](runs/02_in_a_let_nobody_asks_about.v-before.before.show-hyps.txt) · [before.kb-cut.dot](renders/02_in_a_let_nobody_asks_about.v-before.before.kb-cut.dot) · [before.kb-cut.svg](renders/02_in_a_let_nobody_asks_about.v-before.before.kb-cut.svg) |
| [`02_in_a_let_nobody_asks_about.v-before`](programs/02_in_a_let_nobody_asks_about.v-before.ein) | today | [read-out](runs/02_in_a_let_nobody_asks_about.v-before.today.solve.txt) · [summary](runs/02_in_a_let_nobody_asks_about.v-before.today.summary.json) · [events](runs/02_in_a_let_nobody_asks_about.v-before.today.events.jsonl) · [events table](runs/02_in_a_let_nobody_asks_about.v-before.today.events.md) · [trace](runs/02_in_a_let_nobody_asks_about.v-before.today.trace.md) · [dump-answer](runs/02_in_a_let_nobody_asks_about.v-before.today.answer/) · [saturate --dump](runs/02_in_a_let_nobody_asks_about.v-before.today.saturate-dump.txt) · [its facts](runs/02_in_a_let_nobody_asks_about.v-before.today.saturate-facts.txt) · [ein test](runs/02_in_a_let_nobody_asks_about.v-before.today.test.txt) · [show hyps](runs/02_in_a_let_nobody_asks_about.v-before.today.show-hyps.txt) · [today.kb-cut.dot](renders/02_in_a_let_nobody_asks_about.v-before.today.kb-cut.dot) · [today.kb-cut.svg](renders/02_in_a_let_nobody_asks_about.v-before.today.kb-cut.svg) |
| [`02_in_a_let_nobody_asks_about.v-before`](programs/02_in_a_let_nobody_asks_about.v-before.ein) | diff | [saturate-facts.diff](runs/02_in_a_let_nobody_asks_about.v-before.saturate-facts.diff) · [solve.diff](runs/02_in_a_let_nobody_asks_about.v-before.solve.diff) · [test.diff](runs/02_in_a_let_nobody_asks_about.v-before.test.diff) · [trace-steps.diff](runs/02_in_a_let_nobody_asks_about.v-before.trace-steps.diff) |
| [`02_in_a_let_nobody_asks_about.v-today`](programs/02_in_a_let_nobody_asks_about.v-today.ein) | before | [read-out](runs/02_in_a_let_nobody_asks_about.v-today.before.solve.txt) · [saturate --dump](runs/02_in_a_let_nobody_asks_about.v-today.before.saturate-dump.txt) · [its facts](runs/02_in_a_let_nobody_asks_about.v-today.before.saturate-facts.txt) · [ein test](runs/02_in_a_let_nobody_asks_about.v-today.before.test.txt) · [show hyps](runs/02_in_a_let_nobody_asks_about.v-today.before.show-hyps.txt) · [before.kb.err](renders/02_in_a_let_nobody_asks_about.v-today.before.kb.err) |
| [`02_in_a_let_nobody_asks_about.v-today`](programs/02_in_a_let_nobody_asks_about.v-today.ein) | today | [read-out](runs/02_in_a_let_nobody_asks_about.v-today.today.solve.txt) · [summary](runs/02_in_a_let_nobody_asks_about.v-today.today.summary.json) · [events](runs/02_in_a_let_nobody_asks_about.v-today.today.events.jsonl) · [events table](runs/02_in_a_let_nobody_asks_about.v-today.today.events.md) · [trace](runs/02_in_a_let_nobody_asks_about.v-today.today.trace.md) · [dump-answer](runs/02_in_a_let_nobody_asks_about.v-today.today.answer/) · [saturate --dump](runs/02_in_a_let_nobody_asks_about.v-today.today.saturate-dump.txt) · [its facts](runs/02_in_a_let_nobody_asks_about.v-today.today.saturate-facts.txt) · [ein test](runs/02_in_a_let_nobody_asks_about.v-today.today.test.txt) · [show hyps](runs/02_in_a_let_nobody_asks_about.v-today.today.show-hyps.txt) · [today.kb-cut.dot](renders/02_in_a_let_nobody_asks_about.v-today.today.kb-cut.dot) · [today.kb-cut.svg](renders/02_in_a_let_nobody_asks_about.v-today.today.kb-cut.svg) |
| [`02_in_a_let_nobody_asks_about.v-today`](programs/02_in_a_let_nobody_asks_about.v-today.ein) | diff | [saturate-facts.diff](runs/02_in_a_let_nobody_asks_about.v-today.saturate-facts.diff) · [solve.diff](runs/02_in_a_let_nobody_asks_about.v-today.solve.diff) · [test.diff](runs/02_in_a_let_nobody_asks_about.v-today.test.diff) |
| [`03_in_a_fact_of_the_programs_own.v-before`](programs/03_in_a_fact_of_the_programs_own.v-before.ein) | before | [read-out](runs/03_in_a_fact_of_the_programs_own.v-before.before.solve.txt) · [summary](runs/03_in_a_fact_of_the_programs_own.v-before.before.summary.json) · [events](runs/03_in_a_fact_of_the_programs_own.v-before.before.events.jsonl) · [events table](runs/03_in_a_fact_of_the_programs_own.v-before.before.events.md) · [trace](runs/03_in_a_fact_of_the_programs_own.v-before.before.trace.md) · [dump-answer](runs/03_in_a_fact_of_the_programs_own.v-before.before.answer/) · [saturate --dump](runs/03_in_a_fact_of_the_programs_own.v-before.before.saturate-dump.txt) · [its facts](runs/03_in_a_fact_of_the_programs_own.v-before.before.saturate-facts.txt) · [ein test](runs/03_in_a_fact_of_the_programs_own.v-before.before.test.txt) · [show hyps](runs/03_in_a_fact_of_the_programs_own.v-before.before.show-hyps.txt) · [before.kb-cut.dot](renders/03_in_a_fact_of_the_programs_own.v-before.before.kb-cut.dot) · [before.kb-cut.svg](renders/03_in_a_fact_of_the_programs_own.v-before.before.kb-cut.svg) |
| [`03_in_a_fact_of_the_programs_own.v-before`](programs/03_in_a_fact_of_the_programs_own.v-before.ein) | today | [read-out](runs/03_in_a_fact_of_the_programs_own.v-before.today.solve.txt) · [summary](runs/03_in_a_fact_of_the_programs_own.v-before.today.summary.json) · [events](runs/03_in_a_fact_of_the_programs_own.v-before.today.events.jsonl) · [events table](runs/03_in_a_fact_of_the_programs_own.v-before.today.events.md) · [trace](runs/03_in_a_fact_of_the_programs_own.v-before.today.trace.md) · [dump-answer](runs/03_in_a_fact_of_the_programs_own.v-before.today.answer/) · [saturate --dump](runs/03_in_a_fact_of_the_programs_own.v-before.today.saturate-dump.txt) · [its facts](runs/03_in_a_fact_of_the_programs_own.v-before.today.saturate-facts.txt) · [ein test](runs/03_in_a_fact_of_the_programs_own.v-before.today.test.txt) · [show hyps](runs/03_in_a_fact_of_the_programs_own.v-before.today.show-hyps.txt) · [today.kb-cut.dot](renders/03_in_a_fact_of_the_programs_own.v-before.today.kb-cut.dot) · [today.kb-cut.svg](renders/03_in_a_fact_of_the_programs_own.v-before.today.kb-cut.svg) |
| [`03_in_a_fact_of_the_programs_own.v-before`](programs/03_in_a_fact_of_the_programs_own.v-before.ein) | diff | [saturate-facts.diff](runs/03_in_a_fact_of_the_programs_own.v-before.saturate-facts.diff) · [solve.diff](runs/03_in_a_fact_of_the_programs_own.v-before.solve.diff) · [test.diff](runs/03_in_a_fact_of_the_programs_own.v-before.test.diff) · [trace-steps.diff](runs/03_in_a_fact_of_the_programs_own.v-before.trace-steps.diff) |
| [`03_in_a_fact_of_the_programs_own.v-today`](programs/03_in_a_fact_of_the_programs_own.v-today.ein) | before | [read-out](runs/03_in_a_fact_of_the_programs_own.v-today.before.solve.txt) · [summary](runs/03_in_a_fact_of_the_programs_own.v-today.before.summary.json) · [events](runs/03_in_a_fact_of_the_programs_own.v-today.before.events.jsonl) · [events table](runs/03_in_a_fact_of_the_programs_own.v-today.before.events.md) · [trace](runs/03_in_a_fact_of_the_programs_own.v-today.before.trace.md) · [dump-answer](runs/03_in_a_fact_of_the_programs_own.v-today.before.answer/) · [saturate --dump](runs/03_in_a_fact_of_the_programs_own.v-today.before.saturate-dump.txt) · [its facts](runs/03_in_a_fact_of_the_programs_own.v-today.before.saturate-facts.txt) · [ein test](runs/03_in_a_fact_of_the_programs_own.v-today.before.test.txt) · [show hyps](runs/03_in_a_fact_of_the_programs_own.v-today.before.show-hyps.txt) · [before.kb-cut.dot](renders/03_in_a_fact_of_the_programs_own.v-today.before.kb-cut.dot) · [before.kb-cut.svg](renders/03_in_a_fact_of_the_programs_own.v-today.before.kb-cut.svg) |
| [`03_in_a_fact_of_the_programs_own.v-today`](programs/03_in_a_fact_of_the_programs_own.v-today.ein) | today | [read-out](runs/03_in_a_fact_of_the_programs_own.v-today.today.solve.txt) · [summary](runs/03_in_a_fact_of_the_programs_own.v-today.today.summary.json) · [events](runs/03_in_a_fact_of_the_programs_own.v-today.today.events.jsonl) · [events table](runs/03_in_a_fact_of_the_programs_own.v-today.today.events.md) · [trace](runs/03_in_a_fact_of_the_programs_own.v-today.today.trace.md) · [dump-answer](runs/03_in_a_fact_of_the_programs_own.v-today.today.answer/) · [saturate --dump](runs/03_in_a_fact_of_the_programs_own.v-today.today.saturate-dump.txt) · [its facts](runs/03_in_a_fact_of_the_programs_own.v-today.today.saturate-facts.txt) · [ein test](runs/03_in_a_fact_of_the_programs_own.v-today.today.test.txt) · [show hyps](runs/03_in_a_fact_of_the_programs_own.v-today.today.show-hyps.txt) · [today.kb-cut.dot](renders/03_in_a_fact_of_the_programs_own.v-today.today.kb-cut.dot) · [today.kb-cut.svg](renders/03_in_a_fact_of_the_programs_own.v-today.today.kb-cut.svg) |
| [`03_in_a_fact_of_the_programs_own.v-today`](programs/03_in_a_fact_of_the_programs_own.v-today.ein) | diff | [saturate-facts.diff](runs/03_in_a_fact_of_the_programs_own.v-today.saturate-facts.diff) · [solve.diff](runs/03_in_a_fact_of_the_programs_own.v-today.solve.diff) · [test.diff](runs/03_in_a_fact_of_the_programs_own.v-today.test.diff) · [trace-steps.diff](runs/03_in_a_fact_of_the_programs_own.v-today.trace-steps.diff) |
| [`04_in_a_rules_conclusion.v-before`](programs/04_in_a_rules_conclusion.v-before.ein) | before | [read-out](runs/04_in_a_rules_conclusion.v-before.before.solve.txt) · [summary](runs/04_in_a_rules_conclusion.v-before.before.summary.json) · [events](runs/04_in_a_rules_conclusion.v-before.before.events.jsonl) · [events table](runs/04_in_a_rules_conclusion.v-before.before.events.md) · [trace](runs/04_in_a_rules_conclusion.v-before.before.trace.md) · [dump-answer](runs/04_in_a_rules_conclusion.v-before.before.answer/) · [saturate --dump](runs/04_in_a_rules_conclusion.v-before.before.saturate-dump.txt) · [its facts](runs/04_in_a_rules_conclusion.v-before.before.saturate-facts.txt) · [ein test](runs/04_in_a_rules_conclusion.v-before.before.test.txt) · [show hyps](runs/04_in_a_rules_conclusion.v-before.before.show-hyps.txt) · [before.kb-cut.dot](renders/04_in_a_rules_conclusion.v-before.before.kb-cut.dot) · [before.kb-cut.svg](renders/04_in_a_rules_conclusion.v-before.before.kb-cut.svg) |
| [`04_in_a_rules_conclusion.v-before`](programs/04_in_a_rules_conclusion.v-before.ein) | today | [read-out](runs/04_in_a_rules_conclusion.v-before.today.solve.txt) · [summary](runs/04_in_a_rules_conclusion.v-before.today.summary.json) · [events](runs/04_in_a_rules_conclusion.v-before.today.events.jsonl) · [events table](runs/04_in_a_rules_conclusion.v-before.today.events.md) · [trace](runs/04_in_a_rules_conclusion.v-before.today.trace.md) · [dump-answer](runs/04_in_a_rules_conclusion.v-before.today.answer/) · [saturate --dump](runs/04_in_a_rules_conclusion.v-before.today.saturate-dump.txt) · [its facts](runs/04_in_a_rules_conclusion.v-before.today.saturate-facts.txt) · [ein test](runs/04_in_a_rules_conclusion.v-before.today.test.txt) · [show hyps](runs/04_in_a_rules_conclusion.v-before.today.show-hyps.txt) · [today.kb-cut.dot](renders/04_in_a_rules_conclusion.v-before.today.kb-cut.dot) · [today.kb-cut.svg](renders/04_in_a_rules_conclusion.v-before.today.kb-cut.svg) |
| [`04_in_a_rules_conclusion.v-before`](programs/04_in_a_rules_conclusion.v-before.ein) | diff | [saturate-facts.diff](runs/04_in_a_rules_conclusion.v-before.saturate-facts.diff) · [solve.diff](runs/04_in_a_rules_conclusion.v-before.solve.diff) · [test.diff](runs/04_in_a_rules_conclusion.v-before.test.diff) · [trace-steps.diff](runs/04_in_a_rules_conclusion.v-before.trace-steps.diff) |
| [`04_in_a_rules_conclusion.v-today`](programs/04_in_a_rules_conclusion.v-today.ein) | before | [read-out](runs/04_in_a_rules_conclusion.v-today.before.solve.txt) · [summary](runs/04_in_a_rules_conclusion.v-today.before.summary.json) · [events](runs/04_in_a_rules_conclusion.v-today.before.events.jsonl) · [events table](runs/04_in_a_rules_conclusion.v-today.before.events.md) · [trace](runs/04_in_a_rules_conclusion.v-today.before.trace.md) · [dump-answer](runs/04_in_a_rules_conclusion.v-today.before.answer/) · [saturate --dump](runs/04_in_a_rules_conclusion.v-today.before.saturate-dump.txt) · [its facts](runs/04_in_a_rules_conclusion.v-today.before.saturate-facts.txt) · [ein test](runs/04_in_a_rules_conclusion.v-today.before.test.txt) · [show hyps](runs/04_in_a_rules_conclusion.v-today.before.show-hyps.txt) · [before.kb-cut.dot](renders/04_in_a_rules_conclusion.v-today.before.kb-cut.dot) · [before.kb-cut.svg](renders/04_in_a_rules_conclusion.v-today.before.kb-cut.svg) |
| [`04_in_a_rules_conclusion.v-today`](programs/04_in_a_rules_conclusion.v-today.ein) | today | [read-out](runs/04_in_a_rules_conclusion.v-today.today.solve.txt) · [summary](runs/04_in_a_rules_conclusion.v-today.today.summary.json) · [events](runs/04_in_a_rules_conclusion.v-today.today.events.jsonl) · [events table](runs/04_in_a_rules_conclusion.v-today.today.events.md) · [trace](runs/04_in_a_rules_conclusion.v-today.today.trace.md) · [dump-answer](runs/04_in_a_rules_conclusion.v-today.today.answer/) · [saturate --dump](runs/04_in_a_rules_conclusion.v-today.today.saturate-dump.txt) · [its facts](runs/04_in_a_rules_conclusion.v-today.today.saturate-facts.txt) · [ein test](runs/04_in_a_rules_conclusion.v-today.today.test.txt) · [show hyps](runs/04_in_a_rules_conclusion.v-today.today.show-hyps.txt) · [today.kb-cut.dot](renders/04_in_a_rules_conclusion.v-today.today.kb-cut.dot) · [today.kb-cut.svg](renders/04_in_a_rules_conclusion.v-today.today.kb-cut.svg) |
| [`04_in_a_rules_conclusion.v-today`](programs/04_in_a_rules_conclusion.v-today.ein) | diff | [saturate-facts.diff](runs/04_in_a_rules_conclusion.v-today.saturate-facts.diff) · [solve.diff](runs/04_in_a_rules_conclusion.v-today.solve.diff) · [test.diff](runs/04_in_a_rules_conclusion.v-today.test.diff) · [trace-steps.diff](runs/04_in_a_rules_conclusion.v-today.trace-steps.diff) |
| [`07_in_a_rules_conclusion_that_fires.v-today`](programs/07_in_a_rules_conclusion_that_fires.v-today.ein) | before | [read-out](runs/07_in_a_rules_conclusion_that_fires.v-today.before.solve.txt) · [summary](runs/07_in_a_rules_conclusion_that_fires.v-today.before.summary.json) · [events](runs/07_in_a_rules_conclusion_that_fires.v-today.before.events.jsonl) · [events table](runs/07_in_a_rules_conclusion_that_fires.v-today.before.events.md) · [trace](runs/07_in_a_rules_conclusion_that_fires.v-today.before.trace.md) · [dump-answer](runs/07_in_a_rules_conclusion_that_fires.v-today.before.answer/) · [saturate --dump](runs/07_in_a_rules_conclusion_that_fires.v-today.before.saturate-dump.txt) · [its facts](runs/07_in_a_rules_conclusion_that_fires.v-today.before.saturate-facts.txt) · [ein test](runs/07_in_a_rules_conclusion_that_fires.v-today.before.test.txt) · [show hyps](runs/07_in_a_rules_conclusion_that_fires.v-today.before.show-hyps.txt) · [before.kb-cut.dot](renders/07_in_a_rules_conclusion_that_fires.v-today.before.kb-cut.dot) · [before.kb-cut.svg](renders/07_in_a_rules_conclusion_that_fires.v-today.before.kb-cut.svg) |
| [`07_in_a_rules_conclusion_that_fires.v-today`](programs/07_in_a_rules_conclusion_that_fires.v-today.ein) | today | [read-out](runs/07_in_a_rules_conclusion_that_fires.v-today.today.solve.txt) · [summary](runs/07_in_a_rules_conclusion_that_fires.v-today.today.summary.json) · [events](runs/07_in_a_rules_conclusion_that_fires.v-today.today.events.jsonl) · [events table](runs/07_in_a_rules_conclusion_that_fires.v-today.today.events.md) · [trace](runs/07_in_a_rules_conclusion_that_fires.v-today.today.trace.md) · [dump-answer](runs/07_in_a_rules_conclusion_that_fires.v-today.today.answer/) · [saturate --dump](runs/07_in_a_rules_conclusion_that_fires.v-today.today.saturate-dump.txt) · [its facts](runs/07_in_a_rules_conclusion_that_fires.v-today.today.saturate-facts.txt) · [ein test](runs/07_in_a_rules_conclusion_that_fires.v-today.today.test.txt) · [show hyps](runs/07_in_a_rules_conclusion_that_fires.v-today.today.show-hyps.txt) · [today.kb-cut.dot](renders/07_in_a_rules_conclusion_that_fires.v-today.today.kb-cut.dot) · [today.kb-cut.svg](renders/07_in_a_rules_conclusion_that_fires.v-today.today.kb-cut.svg) |
| [`07_in_a_rules_conclusion_that_fires.v-today`](programs/07_in_a_rules_conclusion_that_fires.v-today.ein) | diff | [saturate-facts.diff](runs/07_in_a_rules_conclusion_that_fires.v-today.saturate-facts.diff) · [solve.diff](runs/07_in_a_rules_conclusion_that_fires.v-today.solve.diff) · [test.diff](runs/07_in_a_rules_conclusion_that_fires.v-today.test.diff) · [trace-steps.diff](runs/07_in_a_rules_conclusion_that_fires.v-today.trace-steps.diff) |

Also:

- `*.corpus-before-to-today.{solve,saturate-facts,test}.diff` — the corpus as it was against the corpus as it is
  (v-before under before → v-today under today): 02
  [solve](runs/02_in_a_let_nobody_asks_about.corpus-before-to-today.solve.diff)
  [facts](runs/02_in_a_let_nobody_asks_about.corpus-before-to-today.saturate-facts.diff)
  [test](runs/02_in_a_let_nobody_asks_about.corpus-before-to-today.test.diff); 03
  [solve](runs/03_in_a_fact_of_the_programs_own.corpus-before-to-today.solve.diff)
  [facts](runs/03_in_a_fact_of_the_programs_own.corpus-before-to-today.saturate-facts.diff)
  [test](runs/03_in_a_fact_of_the_programs_own.corpus-before-to-today.test.diff); 04
  [solve](runs/04_in_a_rules_conclusion.corpus-before-to-today.solve.diff)
  [facts](runs/04_in_a_rules_conclusion.corpus-before-to-today.saturate-facts.diff)
  [test](runs/04_in_a_rules_conclusion.corpus-before-to-today.test.diff).
- `*.program.diff` — each program's text, `git diff 0a4a97a3 b5fb8d70`: [02](runs/02_in_a_let_nobody_asks_about.program.diff),
  [03](runs/03_in_a_fact_of_the_programs_own.program.diff), [04](runs/04_in_a_rules_conclusion.program.diff);
  [`corpus-notes.diff`](runs/corpus-notes.diff), the four entries' notes in `corpus/corpus.toml`.
- The mocks: [`decimal-folder.option-b.MOCK.txt`](runs/decimal-folder.option-b.MOCK.txt),
  [`02_in_a_let_nobody_asks_about.option-c.MOCK.saturate-facts.txt`](runs/02_in_a_let_nobody_asks_about.option-c.MOCK.saturate-facts.txt).
- [`tools/`](tools/) — the scripts that made `runs/` and `renders/`: `run_all.sh` (every artefact of one program
  under one binary), `diffs.sh` (before → today diffs), `facts.py` (the facts of a dump that bear on the check),
  `jsonl2md.py` (events as a table), `kbsub.py` (the KB view cut by relation), `trace_excerpt.py` (a trace's steps
  of named rules), `steps_of.py` (a trace's steps that check or label one number).

## 6. Re-run

From the repo root:

```sh
D=plans/m1f_hypothesis_and_documentation/review/02-numerals/12-Q-M1f.407
for f in 02_in_a_let_nobody_asks_about 03_in_a_fact_of_the_programs_own 04_in_a_rules_conclusion; do
  git show 0a4a97a3:tests/stdlib/math/decimal/$f.ein > $D/programs/$f.v-before.ein
  git show b5fb8d70:tests/stdlib/math/decimal/$f.ein > $D/programs/$f.v-today.ein
  git diff 0a4a97a3 b5fb8d70 -- tests/stdlib/math/decimal/$f.ein > $D/runs/$f.program.diff
done
build/review-bin/ein-head-b5fb8d70 test $D/programs/02_in_a_let_nobody_asks_about.v-before.ein
EIN_STDLIB=build/wt-rev-0a4/stdlib build/review-bin/ein-0a4a97a3 test $D/programs/02_in_a_let_nobody_asks_about.v-before.ein
# everything in runs/ and renders/
K="check valid not-decimal written let add sum-of price seed"
for p in $D/programs/*.ein; do for L in before today; do KBRELS="$K" $D/tools/run_all.sh $L $p $D; done; done
$D/tools/diffs.sh $D $(cd $D/programs && ls *.ein | sed 's/\.ein$//')
```

## 7. Found while building

- Reproduced as the record says: each of the three was dead before and is R = {∅} today, on the same text.
- `07_in_a_rules_conclusion_that_fires.ein`, added by the build, is **alive under the before binary**: the loader's
  `written` step stated no fact a rule derives with a variable in it, so `(n (n ?s 1) x)` was never flagged. It is a
  case the new design flags and the old did not (also in `../04-Q-M1f.405/`).
- The v-today texts of `03` and `04` load under the before binary and are dead there; `02`'s does not load (it lists
  `valid`).

## 8. Your call

- **(a)** not checked; the three programs rewritten to say so, `07` added — **the default, as built**.
- **(b)** not checked; the three programs deleted.
- **(c)** a `let`'s value checked as the `let` is read — `02` dead again, `03` and `04` alive.
