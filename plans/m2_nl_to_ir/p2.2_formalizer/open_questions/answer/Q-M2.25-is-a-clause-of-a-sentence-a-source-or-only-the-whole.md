# Q-M2.25 — Is a clause of a sentence a `:source`, or only the whole sentence?

**Status:** open

**Raised 2026-11-26 by [S2.2.1](../../s2.2.1_contract.md)**, computing the contract's categories over the sixteen
word problems of `examples/word_problems/` (its T2.2.1.2;
[the contract's § 4](../../../../../docs/einaf/04-contract.md#4-what-each-form-is-the-categories)).

**The question.** The contract has a fact's `:source` be a sentence of the input, verbatim, and a sentence that
states several facts quoted whole by each, as `zebra3.1.ein`'s five houses each quote *There are five houses.*
Nine of the sixteen word problems quote less than a sentence: a clause where one sentence states two facts
(*"Bob has 5 apples,"* and *"Alice has 3 pears."* of *Bob has 5 apples, Alice has 3 pears.*; *"and 2 pears."*;
*"144 pencils in each."*), twenty quotations in all; and two mark an assumption by a text no sentence has
(*"ASSUMPTION, not in the problem: Alice had no apples."*). Under the contract each of the twenty-two is the
faithfulness judge's event `unfound_source` (`einaf/tests/contract.rs`'s
`the_categories_partition_every_form_of_the_reference_programs` holds the count of each file). Is a clause found?

**The default taken**: the whole sentence, as the contract is written. The word problems are examples of the widest
library condition, and a prompt's copy of one is edited to the contract (the contract's § 5, item 3); the assumption
texts stay events under any reading.

**What the default costs.** A word problem quoted unedited teaches the model to quote a clause, which the judge then
counts against it; and a sentence of two facts is quoted twice whole, so the judge cannot tell which part of it a
fact states.

**The alternatives.**

1. **The whole sentence** (the default).
2. **A clause is found**: a contiguous run of one sentence, bounded by its punctuation or its ends; any other
   fragment, *"has 5"*, is not. The twenty clauses are found, the two assumptions are not.
3. **Any run of the text's words is found**, which makes `unfound_source` the assumption's event alone.

**What bears on it.** [Q-M2.3](../../../open_questions/answer/Q-M2.3-what-is-the-unit-of-faithfulness-without-a-gold-theory.md),
the unit of faithfulness, which S2.4.4 decides with its mechanical judge.
