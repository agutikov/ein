# F26 — Consistency and completeness: Ein's two words, logic's, and common sense's

> **Pulled into [M12](../m12_rules_as_data/README.md) (2026-11-12)**, *rules as data*, on the user's instruction of that day; this file stays where it is. M12 lists the section on self-reference below.

**Filed 2026-11-11**, on the user's instruction of that day:

> I don't remember if followups or somewhere else mentioned consistency and completeness in Ein (open/owe,
> contradictions) and connections to consistency and completeness in common sense (Godel and futher), if not -
> put into some followup

**It was not written anywhere.** A search of `plans/` and `docs/` that day found Gödel named twice, both in
passing, in [idea 09](../ideas/09-puzzles-beyond-zebra.md) (the liar's sentence as a puzzle, and *Gödel, Escher,
Bach* as reading); *incompleteness* used only for a relation that owes a witness (M1d's
[`the_boundary.md`](../../docs/history/m1d_satisfiability/the_boundary.md)) and for M2's *semantic
incompleteness* ([P2.8](../m2_nl_to_ir/p2.8_representations/README.md)); and
[idea 06](../ideas/06-inference-rules-completeness.md) asking whether the *rule set* is complete, a third sense.
[F1b](f1b_logical_formulation.md) asks which fragment of first-order logic Ein covers and does not ask this.

**Theme owner:** the user (research interest).
**Trigger:** when F1b is taken up, since the fragment decides most of the answers below; when
[P1f.6](../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md)'s last stage, S1f.6.10, asks what the
arithmetic theory can do beyond arithmetic; when M2 or M5 has to say in a paper what Ein's verdicts certify; or
when [F5](f5_rules_as_data.md) makes rules data, which is where self-reference becomes writable.

**Taken up by [M1g](../m1g_logic/README.md)** (planned 2026-11-11, not started; it runs after M1f): F1b is
promoted there, which is this page's first trigger. Of § What promotion would look like, 1, 2 and 4 are
[S1g.4.5](../m1g_logic/p1g.4_first_order_logic/s1g.4.5_the_boundary.md), 3 is
[S1g.6.1](../m1g_logic/p1g.6_engine_requirements/s1g.6.1_a_saturation_that_does_not_stop.md), and 5, with
§ Does Gödel's theorem say anything about an Ein program, is
[S1g.5.4](../m1g_logic/p1g.5_peano_and_induction/s1g.5.4_goedel_made_concrete.md); 6 stays here, for M2. This
page is unchanged below and stays a sketch until those stages check it.

**Status of this page.** § What Ein means is the engine's, with its sources. Everything after it is **the
coordinator's sketch**, written to mark the questions and not checked by anything: each mapping is a claim to be
tested when the theme is taken up, and may be wrong.

## What Ein means by the two words

Ein already uses both ideas, under other names, and it keeps them apart
([`solution_semantics.md`](../../docs/kernel/inference/solution_semantics.md)):

| | Ein's notion | where it shows |
|---|---|---|
| **consistency** | a state is **alive**: its saturation does not derive `(false)`. A dead state has a **core**, the facts that suffice for the contradiction, and the rules that derived it | `refuted by`, `unsat core`, the minimal dead sets N; root dead means R is empty |
| **completeness**, of a state | a state is **discharged**: it owes nothing. An obligation is a requirement the program states (`(open ?R)`: every thing of this kind has a value, a slot is filled) that the state has not yet witnessed | `Open — owes n`, *root owes 36 instances*; M1d's *no model* against *not yet a model* |
| the answer | the region **R**: the commitments that are alive **and** discharged, so consistent and complete together | k_s, k_c, the minimal and maximal lines |
| **completeness**, of the search | `exhausted`: every commitment over the universe H is classified, so what is printed is all of R over H | the `exhausted` row; `outside` names what H leaves out |
| **completeness**, of the rules | whether the rule set derives everything the puzzle's meaning entails | [idea 06](../ideas/06-inference-rules-completeness.md); nothing in the engine certifies it |
| closed against open | `(__closed__ R)` and *naming a relation closes it*: what is not stated is false, for that relation only | the scoping of H, and a claim's reading |

So Ein has **three** things called complete (a state, a search, a rule set), and one called consistent.

## What logic means by them

Logic also has more than one thing under each word, and the confusion between them is the usual one:

- **Consistency:** no derivation of a contradiction (syntactic), or having a model (semantic). They agree for
  first-order logic, by the completeness theorem.
- **Completeness of a proof system** (Gödel 1929): every valid sentence is provable. A property of the *rules of
  inference*. Ein's counterpart is idea 06's question, and refutation-completeness of the search.
- **Completeness of a theory** (the sense of the incompleteness theorems, 1931): every sentence of the language
  is proved or refuted. A property of the *axioms*. Ein's counterpart is a program whose R is one state that
  decides every candidate: a unique solution. A program with several states is an incomplete theory in this
  sense, and each fact that differs between two states is a sentence the program leaves undecided.
- **Closed-world completeness** (Reiter, Clark's completion, the databases' sense): what is not derivable is
  false. Ein's `absent` and `__closed__`, and the reason non-monotone programs have their own followup
  ([F21](f21_non_monotone_programs_decided.md)).

**A sketch of the map**, each row a question:

| logic | Ein, as the coordinator reads it | the question |
|---|---|---|
| a theory is inconsistent | root is dead | Ein stops at `(false)` and reports a core, where classical logic derives everything. Is a dead state's content *nothing*, as the read-out says, or the facts before the contradiction, as `--trace` shows? That is a paraconsistent reading, and division by zero in P1f.6 already met it: the user ruled the flag and then `(false)` |
| a theory has no model yet: an existential axiom lacks a witness | a state owes | obligations look like the existential rules of the **chase** in database theory, and the obligations rung like a chase step that proposes witnesses from the names the program has. Henkin's witnesses are the proof-theoretic cousin. Is Ein's *owe and discharge* a chase, and which of its termination results apply? |
| a theory is incomplete: a sentence is independent | R has more than one state | an *ambiguous* puzzle is an incomplete theory, and the projection classes of a query are the independent sentence's values. S1f.6.20's missing reading is this, built on purpose |
| categoricity: one model up to isomorphism | k_s = 1 | what plays isomorphism in Ein? Two states that differ by renaming are two states today |
| the model is not the world | `outside`, and the universe H | *"the real world is potentially infinite, while the problem and theory is a model"* (the user, 2026-11-11, on S1f.6.20) |

## Does Gödel's theorem say anything about an Ein program?

The first incompleteness theorem is about a theory that is **consistent, effectively axiomatised, and
interprets enough arithmetic** (Robinson's Q is enough). The sketch, to be checked:

- **A zebra program is far below it.** Its names are finite, no name is made at run time, H is finite, and the
  language of its answers is ground atoms over H. Every question is decided by a finite search. Such a
  program can be incomplete, as an ambiguous puzzle is, and the engine **shows** the incompleteness as a region.
  Nothing there is undecidable.
- **Nested facts change the class.** A rule may conclude a fact nested deeper than its premises
  ([P1f.6](../m1f_hypothesis_and_documentation/p1f.6_calculator/design.md)'s numbers are that:
  `(n (n (n z 1) 2) 3)`), so the facts a program can derive are no longer bounded by its names, and saturation
  need not stop. That is Horn clauses with function symbols, which compute anything computable. So whether root
  derives `(false)`, **consistency itself**, is then undecidable in general: the halting problem, Turing's and
  Church's half of the same result. The engine has no word for *did not stop*; P1f.6's limits (a nesting of
  1 000, findings E-4) are where it meets it.
- **The calculator computes, and does not quantify.** P1f.6's theory derives every true ground equation it is
  asked for, and can state no sentence about *all* numbers: `x + y = y + x` for every x and y is not a fact a
  state can hold. Q has the same strength on ground equations (it proves every true Σ₁ sentence) and is
  incomplete beyond them. So the theory is on the safe side of Gödel by saying less, and S1f.6.10's question,
  whether it can prove theorems, is the question of what it would take to say more: rules as the universal
  statements, checked on a bounded range, or induction as a rule schema.
- **Self-reference needs rules as data.** The theorem's sentence speaks of its own provability. Ein programs
  cannot name their rules as values today ([F5](f5_rules_as_data.md),
  [`08_self_describing.md`](../../docs/kernel/ir/03-ein-lang/08_self_describing.md)), and idea 09's liar's
  sentence is the puzzle form of it. An Ein that could write *this rule does not fire* would have to say what
  its answer is, and [F21](f21_non_monotone_programs_decided.md)'s cycles through `absent` are the nearest thing
  the engine has: a program with no stable reading.
- **The second theorem, and what a certificate is.** A system of that strength cannot prove its own
  consistency. Ein's practical form of the question is what `exhausted` and a green gate license one to say,
  which [`standard_of_proof.md`](../../docs/kernel/standard_of_proof.md) answers for the repo and not for the
  logic.

## And common sense

The everyday words are the ones a reader of an answer uses: a story is *consistent* when it does not contradict
itself, and *complete* when nothing needed is missing. They are what M2's formalizer has to detect in a
problem's text, and where Ein's verdicts are meant to be read as plain statements:

- **Contradictory:** *Bob has 5 apples and gave 7 away*. Root is dead, and the core is the two sentences.
- **Not enough information:** *how many do they have together* with Alice's apples unstated. A state that owes,
  or a region of several states, and the gap **named**
  ([S1f.6.20](../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.20_word_problems.md)'s two kinds: a
  missing quantity and a missing reading).
- **Complete relative to what?** Common sense fills gaps silently (Alice had none; a box holds any three
  apples). A formal theory is complete only against its own language, and Ein's part is to make the filling
  explicit: an assumption is a hypothesis, and the answer says which it rests on. This is the frame problem and
  default reasoning (McCarthy's circumscription, Reiter's defaults, the stable models behind `absent`), and the
  line from Gödel onward that the user's *and further* points at: Tarski on truth, Turing and Church on
  decidability, Löwenheim and Skolem on what a first-order theory cannot pin down, and non-monotonic logic on
  what people conclude from what is not said.

## What promotion would look like

1. **A glossary of the senses**, one page in `docs/kernel/`: each of Ein's words (alive, dead, owes, discharged,
   exhausted, closed, outside) beside the logical notion it is and the ones it is not, with a program for each.
2. **The fragment, from F1b**, and with it the decidability table: which programs' questions are decided by a
   finite search (no nested conclusion deeper than its premises), and which are only semi-decided.
3. **The engine's word for *did not stop***: a saturation bound and a fifth outcome, or a static check that a
   program's rules can only shrink or keep a term's depth. It is the one gap here that a real program can meet,
   and P1f.6 is where it would be met first.
4. **The chase reading of obligations**, checked against the literature's termination conditions.
5. **A worked self-referential program**, once rules are data: the liar and a Gödel-style sentence, and what R
   is for each.
6. **The common-sense table as benchmark classes for M2:** contradictory, under-determined by a quantity,
   under-determined by a reading, complete; each with what Ein reports.

## Connections

- [F1b](f1b_logical_formulation.md), the fragment; [F1](f1_categorical_formulation.md), the categorical reading.
- [F21](f21_non_monotone_programs_decided.md), programs with no stable reading; [F17](f17_formal_verification/ideas.md),
  `Spec ≠ Intent`, which is *the model is not the world* said of programs.
- [M1d](../../docs/history/m1d_satisfiability/README.md), where *no model* was separated from *not yet a model*.
- [M2](../m2_nl_to_ir/README.md), whose benchmark holds instances whose right answer is *ambiguous*, and whose
  faithfulness question is completeness against the text.
- [Idea 06](../ideas/06-inference-rules-completeness.md) and [idea 09](../ideas/09-puzzles-beyond-zebra.md).
