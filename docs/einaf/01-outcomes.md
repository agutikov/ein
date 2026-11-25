# The outcomes

What the kernel says of a program under one question about its answer, as EinAf reads it. The answer is the region R,
the commitments whose state is alive and discharged ([solution semantics](../ein/reasoning/02-kb-evolution/06-solution-semantics.md#2-the-answer-the-region-r));
a question is one of `ein solve`'s ([the command line § 4](../ein/user/04-reference/05-cli.md#4-which-command-answers-which-question));
and **the outcome is of the question**, not of the program: the same program can be *one* under one question and
*not decided* under another, and the function says which question it was asked.

The function is [`einaf::kernel::outcome`](../../einaf/src/kernel/outcome.rs). It reads the library's values, the
load's result and the read-out of the run, and nothing of what `ein solve` prints.

## 1. The seven

| outcome | what the kernel says | read from the library | for a reader of the binary | held by |
|---|---|---|---|---|
| **invalid** | the program was refused, at the read, the parse or the load: no answer exists | the load's error: the parser's value (`ein_ir::ParseError`, every error with its line, column, notes and repairs), or the loader's message, which the library hands back as text; the judgement of the hypothesis mechanisms on root's fixpoint is among the load's refusals ([`07-diagnostics.md` § 2](../ein/user/04-reference/07-diagnostics.md#2-errors-and-exit-codes)) | no summary is written; the message on stderr; exit 1, which a refuted claim also gives | `invalid_is_read_off_the_refusal` |
| **flagged** | the program is not monotone: an answer is computed and nothing is claimed of it ([Q-M1f.564](../decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)) | the answer's monotonicity: a site of the static check, or a hypothesis found in flight, or the warning | `monotonicity.warning`, `monotonicity.sites` | `flagged_is_read_before_any_answer` |
| **not decided** | a bound stopped the run before the question was decided, or the question does not decide it | the answer's `stopped`: the facts budget, the cube's limit, the cap on one saturation's firings (`--max-steps`), the clock; or the clock's error, which hands back no answer; or `--no-search` on a root that is alive | `region.exhausted false` and `region.stopped.why` `budget`, `limit` or `steps`, `verdict.word null`; under `-T`, exit 2 and no summary | `not_decided_is_a_run_a_bound_stopped` |
| **inconsistent** | R is empty and every node is dead | `Contradiction`; a description exhausted with no state and no node alive; a dead root under `--no-search` | `verdict.word "Contradiction"`, `region.empty "dead"` | `inconsistent_is_an_empty_r_every_node_dead` |
| **incomplete** | R is empty and every alive node owes an obligation the program states | `Open`; a description exhausted with no state and a node alive | `verdict.word "Open"`, `region.empty "owing"` | `incomplete_is_an_empty_r_every_alive_node_owing` |
| **several** | the part of R the reading counts holds two states or more | `Ambiguity`, with the two states that decided it; a description or a line exhausted with two states or more in the part | `verdict.word "Ambiguity"`; `region.k_s`, `region.maximal.states` with `region.exhausted true` | `several_and_one_are_read_over_a_part` |
| **one** | the part of R the reading counts is one state | `Solution`; a description or a line exhausted with one state in the part | `verdict.word "Solution"`; `region.k_s 1` with `region.exhausted true` | `one_is_one_state` |

The tests are [`einaf/tests/outcomes.rs`](../../einaf/tests/outcomes.rs)'s, each on a corpus program that has the
outcome. [`einaf/tests/corpus.rs`](../../einaf/tests/corpus.rs) holds the reading over the corpus: every program
that is not `slow`, every question, one outcome a reading; a `parse-negative` entry is invalid at the parse and a
`load-negative` one at the parse or the load; a program the engine's static check flags is flagged under every
question and nothing else; a program is invalid exactly where `ein solve` exits non-zero and refutes no claim;
questions that read one part of R agree wherever both decided it; and a claim the program states, where the kernel
holds it, allows the outcome over the claim's part. No value of a flagged program's answer is asserted.

The four answers, *inconsistent*, *incomplete*, *several* and *one*, are the kernel's four verdict words
([embedding the crates § 2.5](../ein/ein.rs/04-api.md#25-read)), and *not decided* is the fifth arm of the same
match, never read as one of them ([Q-M1d.6](../decisions/answer/Q-M1d.6-contradiction-is-said-only-where-no-node-is.md)).

## 2. The order of the reading

The outcome is read in the order of the table, and the order is part of what it means:

1. **A refusal first**: a program the kernel refused has no answer to read.
2. **Flagged before any answer.** A flagged program still has a verdict word under `--single`, printed below the
   warning; the function does not read it.
3. **A stopped run before its word.** A description stopped at its budget can have listed every state of R and still
   is *not decided*: its question was to describe R, and the kernel says it did not finish. Under `--single` the same
   program says `Ambiguity` after two states, and is *several*.
4. **Then the answer**, over the part the reading counts.

A run the clock stops hands back no answer at all, so it is *not decided* even of a program the static check flags:
the flag is a field of the answer the clock did not return.

## 3. The questions and the readings

*One* and *several* are counted over a part of R: all of it, its minimal line or its maximal line. A program that
states restrictions and no obligation has ∅ in R beside its full assignments, so it is *several* over all of R and
can be *one* over the maximal line ([solution semantics § 4](../ein/reasoning/02-kb-evolution/06-solution-semantics.md#4-restrictions-and-obligations-the-two-lines)).
Which reading certifies a unique answer is not decided, so the function returns the outcome with the reading it was
taken under, and a caller that wants both asks both.

| question | `ein solve` | its readings |
|---|---|---|
| a description | `ein solve FILE` | all of R, the minimal line, the maximal line: the description lists every state with its lines |
| one state? | `--single`, `--minimal --single`, `--maximal --single` | the part asked |
| a line | `--minimal`, `--maximal` | the line asked |
| the counts | `--count` | all of R |
| root alone | `--no-search` | all of R: *inconsistent* on a dead root, *not decided* otherwise, since R was not searched |

An empty R is empty over every part, so *inconsistent* and *incomplete* do not depend on the reading.

## 4. What the outcome does not say

- **A query's own axis**: its classes over the part it asks about, their rows, and whether they agree. A program with
  several states and one answer is *several* and its answer query has one class; the outcome alone does not say so.
- **Whether a claim the program states held**: the kernel checks each `:expect` and says so beside the answer.
- **The problem's class.** *Incomplete* is the kernel's word, and also what a problem with no solution looks like
  where no rule refutes it; *several* says the program leaves the answer open, not that the problem does.
