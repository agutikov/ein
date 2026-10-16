# F20 — The three rule kinds are the three sentence forms

**Added:** 2026-10-16, from the user, during M1e2 S1e2.1a.9:

> I just noticed how our rule types (asserting positive facts, restirctions
> asserting negative facts or false, and obligation) are mapping well to
> natural language statements forms: assertion, negation, question; note this
> in followups

## The observation

Ein has three kinds of rule, told apart by what they conclude. Each matches
a sentence form, and each has its own role in the region R
([`solution_semantics.md` § 4](../../docs/kernel/inference/solution_semantics.md#4-restrictions-and-obligations--the-two-lines)):

| rule kind | concludes | sentence form | Zebra, in words | what it does to R |
|---|---|---|---|---|
| **assertion** | a positive fact | *X is Y* | "The Englishman lives in the red house" | closes a node: what follows from a commitment, cl(C) |
| **restriction** | `(not …)` or `(false)` | *X is not Y*, *never both* | "The Spaniard does not drink coffee" | kills nodes, and sets the **maximal line**: the user's *"top bound defined by restrictions"* |
| **obligation** | `(open ?R)`: a debt | *Who / which X …?* | "Who owns the zebra?"; implicitly, *who lives in each house?* | a node owes until the question is answered, which sets the **minimal line**: the user's *"bottom bound defined by obligations"* |

Read this way, **R is the set of ways to answer the program's questions
without contradicting its negations, closed under its assertions.**

- An obligation's **alternatives** are the question's possible answers.
- A witness is an answer, and *discharged* means every question is answered.
- `Open — owes n` is *n questions still unanswered*. It is a different thing
  from `Contradiction`, a negation broken.

## Why it is worth keeping

- **M2 (NL → IR)** has to classify each clause of a puzzle text. The mood of
  the sentence (declarative, negative, interrogative) is a first,
  surface-level signal for which kind of rule to emit. The formalizer's passes
  ([S2.2.4](../m2_nl_to_ir/p2.2_formalizer/s2.2.4_passes.md)) already turn
  the puzzle's question into goal variables (`?who_water`, `?h_zebra`). An
  obligation is the same thing, inside the program.
- **The way back (IR → NL)**: [idea 08](../ideas/08-human-style-deductive-trace.md)'s
  explanation can render each rule in its own mood, and an open state as the
  questions it has not answered.
- **The verdict words** line up the same way. `Solution` means every
  question is answered consistently. `Contradiction` means a negation is
  broken. `Open` means a question is unanswered. `Ambiguity` means a question
  has more than one consistent answer.

## Prior art to read first

- **Question semantics.** In Hamblin's and Karttunen's semantics, a question
  denotes the set of its possible answers. Inquisitive semantics
  (Groenendijk and Roelofsen) treats a question as a set of possibilities to
  be resolved. Both are close to *an obligation is the disjunction of its
  alternatives, discharged by one of them*
  ([`completeness.md`](../../docs/history/m1d_satisfiability/completeness.md)).
- **Answer set programming** has the same three forms:
  - normal rules are assertions;
  - integrity constraints (`:- body.`) are negations;
  - choice and cardinality rules (`1 { a; b; c } 1.`) are questions with
    their answers enumerated.
- **Speech-act theory** (Searle): assertives, against the directives that
  questions are.

## Why not now, and what would trigger it

M1e2 is about correctness. The mapping changes no engine behaviour and
settles no open question. It becomes work when either of these starts:

- M2's formalizer decides which rule kind a clause becomes, at its contract
  or passes stage (S2.2.1, S2.2.4). The mood is then a feature or a
  constraint of the grammar ([F2](f2_self_modifying_language.md), GBNF);
- the explanation work of idea 08 renders rules and open states back into
  text.

The open question it would answer: **does every interrogative sentence in a
puzzle become an obligation, or only the ones that require an answer of the
solution?** The query's `:goal` asks without requiring. `(solution …)` and
`:expect` state an expected answer, which is S1e2.1a.4's grammar.
