# Q-M2.12 — What prints the answer?

**Status:** ruled by the user, 2026-11-25

**Ruled by the user, 2026-11-25:** *"don't run M1p now, use as much of existing ein NL features, e.g. query
:goal-text, for numbers and other compound facts use additional external formatters, they will be later starting
point for M1p"*.

The question was raised by the user's statement of the core idea
([README](../../README.md#the-two-main-ideas-the-user-2026-11-25)): *"ideally the final test problem answer must be
short and must be produced by ein program NL output"*. Of the three ways it weighed (the answer as Ein prints it
today; the part of M1p an answer needs run first, which was recommended; the harness formats values) the ruling is
the third, with as much of the first as there is. `git show 6a3c9a75:plans/m2_nl_to_ir/open_questions.md` has the
table.

**Read as:**

- **[M1p](../../../m1p_ein_to_natural_language/README.md) does not run**, and the kernel is not changed for the
  answer's words.
- **The answer of a test problem is what its Ein program says, through what Ein has today**: the query's
  `:goal-text`, filled with a row's bindings; and, where an explanation is shown, a rule's and a relation's `:why`
  and a fact's `:source`
  ([`14-natural-language.md`](../../../../docs/ein/user/03-syntax-and-semantics/14-natural-language.md)). So the program
  the formalizer writes carries its own answer sentence: the query's `:goal-text` is part of the contract
  ([S2.2.1](../../p2.2_formalizer/s2.2.1_contract.md)).
- **Where a value does not read as text, EinAf formats it, outside the kernel.** Today
  [`bob1.ein`](../../../../examples/word_problems/bob1.ein) answers `“(amount Bob apples t1): (n z 3)”`: a number is its
  term, `(n z 3)`, and so is a fact that stands as a value. The formatters are EinAf's, in `einaf/`, over the
  library's values: a numeral of the calculator's first, then the other compound values a benchmark family
  prints.
- **They are written to be M1p's starting point**: a formatter a kind of value, its cases as data, its tests
  beside it, and no knowledge of the loop in it, so that the phase of M1p that gives a value its own format
  ([P1p.1](../../../m1p_ein_to_natural_language/p1p.1_custom_formatting/README.md)) can take them as they are.

**What it changes in the plan:** the formatters are a stage of the formalizer
([S2.2.5](../../p2.2_formalizer/s2.2.5_from_text.md)); a benchmark instance's gold answer is a short text beside its
`:expect` claim ([P2.3](../../p2.3_benchmark/README.md)); and what scores an answer compares that text across every arm
([S2.5.3](../../p2.5_harness/s2.5.3_metrics.md)). What Ein lacks here (the answer's line is on the read-out and in no
key of `ein-summary/2`; a value has no text of its own) is recorded in
[the evaluation of Ein](../../evaluation/README.md), for the iteration of Ein that follows.

**Ruled further, 2026-11-25**, to two defaults of the re-plan:

- **The answer is a sentence.** To the default that every arm is scored on a bare value (*3*, *Cy*): *"no, I meant
  a sentence, like zebra's “The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”"*. That
  sentence is what [`zebra.ein`](../../../../examples/zebra.ein)'s query prints through its `:goal-text`. **Read as:**
  *short* means a sentence or two in the problem's own words, not a bare value; the answer query's `:goal-text` is
  such a sentence; every arm is asked for one; and a gold answer is the sentence its canonical program prints.
  **What it changes:** an answer is no longer scored by equality of a normalised value. How a sentence is scored
  is [S2.5.3](../../p2.5_harness/s2.5.3_metrics.md)'s to design, by default **by what it states**: the gold holds, beside
  its sentence, the values the sentence binds (the answer query's rows), and an answer is right when it states
  each of them and no other value in its place; for an arm that runs Ein the rows are the program's own and the
  gold claim is checked by the kernel where it is comparable.
- **EinAf's fixed texts, where no program prints the answer:** *"OK"*. The two texts
  ([S2.2.5](../../p2.2_formalizer/s2.2.5_from_text.md): *The statements do not determine the answer.* and *The
  statements contradict each other.*) are EinAf's and stand, said as such.
