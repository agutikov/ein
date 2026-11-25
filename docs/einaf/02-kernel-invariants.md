# The kernel's invariants

What EinAf relies on the kernel for, beside each outcome it reads: six statements of what an answer guarantees, each
with the test of Ein's that holds it or the decision that says what holds instead. They are Ein's, held by Ein's
tests; EinAf adds no test to Ein and restates none, and this page is a list of where each lives.

## 1. The six

| invariant | in Ein's terms | held by |
|---|---|---|
| **Every fact is justified**, by the program's text or by a rule and the facts it read | every fact a knowledge base holds records how it came to be there: a given with its `:source`, a rule's firing with its premises, or a commitment; a fact is an OR over its recorded derivations ([Q18](../decisions/engine/Q18-provenance-is-per-fact-an-or-of-derivations-over.md)) | the record itself, `ein_core::Kb::record_justification`, and [`explain_semantics.rs`](../../ein.rs/crates/ein-infer/tests/explain_semantics.rs)'s `a_rebuild_preserves_the_alternative_justifications` and `a_full_list_keeps_the_smallest_closure_and_the_pair_report_one_core`, which hold what is recorded. No test of Ein walks every fact of every program for its record |
| **Every contradiction has a recorded derivation** | a state dies where a rule that states `:ein inconsistent` fires ([Q-M1f.550](../decisions/language/Q-M1f.550-the-engines-word-is-inconsistent.md)), and the firing's record is kept (`ein_core::Kb::ended`); a dead node's core is read off it, and names the node's commitment where the refutation needs it and, with root, derives the contradiction ([Q-M1f.6](../decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)) | [`corpus_cores.rs`](../../ein.rs/crates/ein-infer/tests/corpus_cores.rs): `every_core_names_its_commitment_and_derives_its_contradiction`, over every core a run reports on the corpus, and `a_rule_with_no_premise_refutes_root_with_the_empty_core` |
| **Every reported solution satisfies every active constraint** | every state a run prints is a member of the oracle's R, alive and discharged: contract C1 of [the command line](../ein/user/04-reference/05-cli.md#1-the-contract) ([Q-M1e2.32](../decisions/answer/Q-M1e2.32-no-two-commands-disagree-and-the-oracle-is.md)) | [`explore.rs`](../../ein.rs/crates/ein-infer/tests/explore.rs): `every_mode_is_the_oracle`, every question against the cube on every corpus program the oracle enumerates; [`answer_round_trip.rs`](../../ein.rs/crates/ein-cli/tests/answer_round_trip.rs): `every_state_file_is_saved_and_loaded_as_itself_0` to `_7`, every state a description dumps reloaded as itself |
| **Unique means one model, and the search exhausted** | `Solution` says R, or the line asked, is one state, and is said only of a decided question: a run stops early only on a decided question or a bound, and a bound's stop says no word ([Q-M1d.1](../decisions/answer/Q-M1d.1-a-search-stops-early-only-on-a-decided-question.md)); a model is a state of R, which owes nothing | `every_mode_is_the_oracle`: every verdict word is the oracle's, and a run a budget stopped prints bounds and no word (contracts C2 and C3) |
| **Ambiguous means two distinct models exist** | `Ambiguity` is said with the two states that decided it, each a member of R, and they differ | `every_mode_is_the_oracle`: every printed state is a member of the part asked, and the states printed are distinct (contract C1) |
| **Unsat means exhaustive search found no model** | R empty is said only where it is shown empty, and of two kinds: `Contradiction` only where no node is alive, `Open` where every alive node owes ([Q-M1d.6](../decisions/answer/Q-M1d.6-contradiction-is-said-only-where-no-node-is.md)) | `every_mode_is_the_oracle`, which holds each word, the two empty kinds apart, to the cube's |

The answer is the region R ([Q-M1e2.7](../decisions/answer/Q-M1e2.7-the-answer-is-a-region-r-of-the-hypothesis.md)),
and *unique*, *ambiguous* and *unsat* are three readings of it. Over which part of R *unique* certifies an answer is
EinAf's to settle by experiment; [the outcomes](01-outcomes.md#3-the-questions-and-the-readings) carry the reading.
The premises the answer rests on, each with what enforces it, are
[the design's](../ein/ein.rs/03-design/README.md#what-the-answer-rests-on), and the reasoning's pages paired with
their code and tests are [reasoning in code](../ein/ein.rs/08-reasoning-in-code/README.md).

## 2. Three statements that are false by decision

Three things an autoformalization loop might expect of the kernel, which Ein decides otherwise:

- **A contradiction's core is not always non-empty.** `[]` is the empty core of a dead node whose refutation rests
  on nothing the node holds, a rule with an empty `:match` that concludes `(false)`
  ([Q-M1e2.27](../decisions/read-out/Q-M1e2.27-in-eins-json-is-the-empty-set-and-null-what-does.md),
  [Q-M1f.48](../decisions/read-out/Q-M1f.48-a-text-surface-writes-the-empty-core.md)); held by
  `a_rule_with_no_premise_refutes_root_with_the_empty_core`. A question that prints no core, `--single` among them,
  takes none.
- **A core is not the smallest.** A core is an explanation: it may vary with the rules' order, and it is not
  subset-minimal ([Q-M1f.6](../decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)). Feedback that shows a core
  shows one explanation of the death.
- **The kernel does not hold a fact apart from its negation.** A state that holds a fact beside its stored negation
  is dead only where the program imports `std.contradiction`, whose rules refute the pair; without it the state is
  alive ([Q-M1f.557](../decisions/stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md)). A program a
  model writes that relies on the death imports the theory.
