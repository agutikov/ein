# Q-M2.35 — Is there an answer query for each question of the text?

**Status:** open

**Raised 2026-11-26 by the user**, on the model's one `?who` for the Zebra puzzle's two questions
([L-M2.21](../../findings/model/L-M2.21-the-model-s-zebra-answer-query-asks-one-person-both-questions.md)); written
up in [the analysis of P2.2, § 8](../../p2.2_formalizer/report/ANALYSIS.md#8-the-one-who-for-two-questions), and filed
at the phase's close, 2026-11-27. The user's words:

> On single variable `?who`.
> Qwen can write programs in programming languages AFIAK.
> So it knows same name means same variable can mean same object in the same context.
> So LLM either does not understand the Ein semantics,
> or our `close to text` instruction may be confusing.
> Or we should have separate query for each question - then same `?who` works perfectly. Right?

**The question.** [P2.2's default 4](../../p2.2_formalizer/README.md#defaults-taken-at-the-re-plan) makes the
answer query *the one query that carries a `:goal-text`*, so that the benchmark and the metrics read one answer.
Three places ask the model for one (the step-1 template, the excerpt, step 1's grammar), and the `shops` example
shows two questions in one query. A text with two questions then has one form to hold both, and the model joins
them: on the Zebra text over one variable, on `gap_fact` over three conjuncts, of which one with no row leaves the
whole goal with none. Alone, before the contract asked for one query, the model wrote a query a question, each with
`?who`, and correct Ein (S2.2.2's probe, `step1.off.zebra`). Is every query that carries a `:goal-text` an answer
query, one for each sentence of the text that ends in `?`, in the text's order?

**The default that stands**: one answer query (default 4), until the user rules.

**The proposal**: yes. A question is a sentence, and a sentence is a form of its own, as a statement already is.
What follows is [S2.2d.1](../../p2.2d_formalizer_corrected/s2.2d.1_a_query_a_question.md)'s: the contract, the
prompts, step 1's grammar holding the count, `einaf::answer` printing each query's line and choosing the fixed text
a query, the check's `several-answer-queries` replaced by *fewer answer queries than questions*; held by an
invariance, the Zebra steps by hand with the joint query and with a query a question printing the same sentences
under every question of `ein solve`.

**What the default costs.** A joint query's classes are over the pair, so one question cannot be determined where
another is not; and the text's own word for each unknown is the same (*Who*).

**The alternatives.** One query, with the rule said in the prompt (*a variable written twice in one form is the
same object*) and distinct variables asked for; or a query a question (the proposal).

**What bears on it.** [P2.3](../../p2.3_benchmark/README.md)'s instance, which names one query as its answer
query ([S2.3.1](../../p2.3_benchmark/s2.3.1_families_and_seed_set.md), `answer.query`), and
[S2.5.3](../../p2.5_harness/s2.5.3_metrics.md)'s answer rule: both read one today, so the ruling comes before
either is built.
