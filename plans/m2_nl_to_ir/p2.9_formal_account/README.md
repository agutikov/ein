# P2.9 — The formal account (Stage M)

**Depth:** a phase README, on purpose; its stages are named and written at the Level C gate
([README § How deep this plan is](../README.md#how-deep-this-plan-is)). **It does not wait for M5's venue**
([README § The defaults the re-plan took](../README.md#the-defaults-the-re-plan-took), 12): it is written at the
depth of a page of [`docs/ein/reasoning/`](../../../docs/ein/reasoning/README.md), definitions and propositions
with their arguments, and [M5](../../m5_presentation/README.md) cuts from it what its venue takes.
**Depends on:** to start, nothing but the tree: its first stage reads Stage M's sketch against what the kernel's
pages state. For the loop's half, [P2.1](../../../docs/einaf/03-feedback.md)'s feedback object and
[P2.4](../p2.4_loop/README.md)'s two arrangements **as built**: the account is of a system that exists. To finish,
the **Level C gate**: the central question has to name the feedback functions the ablations ran, and the readings
of *certified* still in force.
**Blocks:** §§ 4 and 5 of [Stage P's outline](../EinAf.md#stage-p--research-paper), *Ein* and *Autoformalization*,
which M5 writes from it; Level D's *formal treatment*.
**Where it is written:** `docs/einaf/`, EinAf's documentation.
**Research plan:** [`EinAf.md` § Stage M](../EinAf.md#stage-m--formalize-the-conceptual-model).

---

## What Stage M asks, and what is already written

Stage M has two halves. The first: *describe Ein mathematically, independently of the Rust implementation*, a
knowledge state, saturation as an operator with a least fixpoint, hypotheses and a model space, a verdict by the
count of models. The second: autoformalization as synthesis, the kernel as an evaluation that returns a verdict
and a diagnostic, iterative repair, and the central question as a probability.

**The first half is largely written, and not by this milestone.** [`docs/ein/reasoning/`](../../../docs/ein/reasoning/README.md)
states Ein's reasoning as mathematics, *so that a second implementation would be held to it*, in fifteen pages
with one notation table; [`docs/ein/ein.rs/08-reasoning-in-code/`](../../../docs/ein/ein.rs/08-reasoning-in-code/README.md)
says for each of those pages where the engine implements it and which test holds it. So **this phase writes the
loop's half and cites the kernel's half without restating it.** The loop is not Ein: nothing under `docs/ein/`
states it, and nothing there may.

### Stage M's sketch against the kernel's account

The kernel's account differs from the sketch wherever the engine does. The account this phase writes says each
difference once, by citing the page. [`analysis/audit_p2.7_p2.10_neighbours.md` § B.3](../analysis/audit_p2.7_p2.10_neighbours.md#b3-p29--the-formal-account-against-docseinreasoning)
has the comparison symbol by symbol, with line numbers.

| Stage M writes | the kernel's pages state | where |
|---|---|---|
| a knowledge state `K = (O, R, F, Γ, P)` | no tuple. A knowledge base $K$ is a finite set of facts; $O$ the objects and $F$ the facts among them; a relation is one atom; the rules are the program $P$, no part of a state; the provenance is $J_K(f)$, the derivations recorded for a fact | [the knowledge base](../../../docs/ein/reasoning/01-static-ir/01-knowledge-base.md), [the rules](../../../docs/ein/reasoning/01-static-ir/02-rewriting-rules.md), [cores](../../../docs/ein/reasoning/02-kb-evolution/09-cores.md) |
| `T_Γ` monotone, `K* = lfp(T_Γ)` | $T_P$ is inflationary, and **monotone only where no rule reads an absence**; $\operatorname{sat}$ is its limit; with absences a second phase, the boundary; then a pass that writes what a state owes and supposes. **Termination is assumed, not given.** `K*` is $\mathrm{root} = \operatorname{sat}(K_0)$ | [saturation](../../../docs/ein/reasoning/02-kb-evolution/02-saturation.md), [absence](../../../docs/ein/reasoning/02-kb-evolution/03-absent-and-naf.md) |
| hypotheses, commitments, a model space `K* ⊆ M`, `M ⊨ Γ` | the candidates $H = \operatorname{gen}(P, \mathrm{root})$, a commitment $C \subseteq H$, its state $\operatorname{st}(C)$, and **the answer, a region $R$**: the commitments whose state is alive and discharged. **A set of commitments over $H$, not a set of models**: a model is a projection that decides nothing, and the answer names what $H$ leaves out | [hypothesis generation](../../../docs/ein/reasoning/02-kb-evolution/04-hypothesis-generation.md), [solution semantics](../../../docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md), [satisfiability](../../../docs/ein/reasoning/02-kb-evolution/10-satisfiability.md) |
| a verdict of three values by the count of models | $\operatorname{verdict}(R)$ of **four**: one solution, several, contradiction, open. It is **asked of a part of $R$**: all of it, $\min R$ or $\max R$. There are two counts, $k_s$ of states and $k_c$ of commitments, and a count is *the* count only where everything asked was classified | [solution semantics § 5](../../../docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md#5-the-questions-asked-of-r) |
| constraints, as a kind of thing beside rules | rules by their conclusion: one that derives facts, a restriction, a hypothesis rule, and an **obligation**, which is first-class: what a state owes is half of $R$'s definition | [the rules](../../../docs/ein/reasoning/01-static-ir/02-rewriting-rules.md), [satisfiability § 4](../../../docs/ein/reasoning/02-kb-evolution/10-satisfiability.md#4-why-an-obligations-alternatives-are-jointly-exhaustive) |
| nothing | a program that is not monotone, of whose answer **nothing is claimed** | [monotonicity](../../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md) |
| nothing | how $R$ is found: a reference and the routes licensed to agree with it, no order of evaluation stated | [finding R](../../../docs/ein/reasoning/02-kb-evolution/07-finding-r.md) |
| nothing | what an answer states of a program, $\operatorname{Ans}(P)$, and the rewrites of a rule set that leave it unchanged | [transformations](../../../docs/ein/reasoning/03-rule-evolution/03-transformations.md) |

## What this phase writes: the loop's half

### The objects

Each is defined in the account from what EinAf built, and where it is made of the kernel's notions it is written
in the kernel's notation.

| Stage M | what it is in EinAf | made of |
|---|---|---|
| `x` | the text of a problem | a benchmark instance ([P2.3](../p2.3_benchmark/README.md)) |
| `T`, `Tᵢ` | what the model wrote at step *i*: a program, which is the kernel's $P$ with the knowledge base $K_0$ its text is read into, or a text that is refused and is no program | [the knowledge base of a program](../../../docs/ein/reasoning/01-static-ir/01-knowledge-base.md#the-knowledge-base-of-a-program) |
| `E(T) → (v, d)` | the kernel's judgement of one text under one question: EinAf's feedback object. `v` is its [outcome](../README.md#the-outcomes), one of seven; `d` is the rest | [S2.1.3](../../../docs/einaf/03-feedback.md). On the kernel's side: a refusal, or what the question establishes of $\operatorname{Ans}(P)$ ($\operatorname{verdict}$ of the part asked, the states listed, what root owes); the sites of the static check; and beside them the explanations ($\operatorname{core}$, a derivation) |
| not in Stage M | **the rendering**: F0 to F8, nine functions from a feedback object to what the model is shown. It is the experimental variable, so it is an object of the account and not a detail of `E` | [S2.4.3](../p2.4_loop/s2.4.3_feedback_ladder.md) |
| `A_θ` | the formalizer: the model's file, its sampler and seed, the prompts and, in the filter, the grammar. A random map from a context to an output | [P2.2](../p2.2_formalizer/README.md); the fields of the record ([S2.5.1](../p2.5_harness/s2.5.1_experiment_record.md)) |
| `Tᵢ₊₁ = A_θ(x, Tᵢ, E(Tᵢ))` | the filter's step, with the rendering between the judgement and the model | [S2.4.1](../p2.4_loop/s2.4.1_state_machine.md), [S2.4.2](../p2.4_loop/s2.4.2_repair.md) |
| the verified answer | the short text Ein prints for a program the kernel passes: the query's `:goal-text` filled from a row of the query's class, with EinAf's formatters over its values | [S2.2.5](../p2.2_formalizer/s2.2.5_from_text.md); on the kernel's side the projection $\pi_g$ of a state on the goal |
| `correct`, `faithful` | predicates of a run against the instance's gold. *Correct*: the answer's text is the gold's, and for an arm that runs Ein the gold claim, a claim $\Gamma$ of the kernel's, is held. *Faithful*: by the unit [S2.4.4](../p2.4_loop/s2.4.4_faithfulness.md) settles | [S2.5.3](../p2.5_harness/s2.5.3_metrics.md); [satisfiability § 3](../../../docs/ein/reasoning/02-kb-evolution/10-satisfiability.md#3-claims-about-the-set-of-states) |
| *certified* | a predicate of a judgement, one for each reading of [Q-M2.10](../open_questions/answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md) still in force; false of a flagged program under every reading | [S2.5.3](../p2.5_harness/s2.5.3_metrics.md) |

### Two arrangements, two shapes

The ruling on [Q-M2.13](../open_questions/einaf/Q-M2.13-who-calls-the-kernel.md) makes the loop two things, and the
account states both.

- **The filter is an iteration.** A sequence of texts, each judged, the harness deciding that the kernel runs and
  when the sequence ends. Its formal shape is Stage M's chain `x → T₀ → E(T₀) → T₁ → ⋯ → Tₙ`, with the rendering
  made explicit and the termination policy part of the definition: what ends a run, and what each ending is
  called.
- **The tool is an episode** ([S2.4.6](../p2.4_loop/s2.4.6_ein_as_a_tool.md)). A sequence of the model's
  turns, each a text or a call that carries a program; the reply to a call is the same judgement under one
  rendering; the model decides whether to call, how often and when to stop; the answer is its last message.
  Stage M has no line for it. Two things the filter has by construction have to be defined: whether an episode's
  answer **is** what its last run printed, and what *certified* means of an episode. The number of calls is a
  random variable, and an episode with none is a chain of thought.

### What can be argued

The propositions the account states, each with its argument, and each either held by a test or said to be an
argument only. The list is a starting point; the stage that writes the account adds and drops.

1. **The judgement is a function of the text and the question.** No option of the kernel changes $R$, at any
   count of threads. So all the randomness of a run is the model's. Its two parts differ: what is read off
   $\operatorname{Ans}(P)$ is the same, up to a renaming, across the rewrites that preserve the answer, and an
   explanation is not: a core may vary with the order of the rules
   ([Q-M1f.6](../../../docs/decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)).
2. **The filter stops**, given that every judgement returns. A judgement need not: termination of a saturation is
   assumed. So the bound EinAf puts around the kernel is part of the definition of `E`, and its firing is the
   outcome *not decided*.
3. **What the filter answers, the kernel printed.** If a run has an answer, some text of the run was judged *one*
   under the reading in force, and the answer is that program's. Nothing follows about the text of the problem:
   this is the proposition at which formal validity and faithfulness part, and the account says so there.
4. **A certificate is over $H$.** *One* is one state of $R$, or of a line of it, $R$ is over the candidates, and
   the answer names what $H$ leaves out
   ([satisfiability § 5](../../../docs/ein/reasoning/02-kb-evolution/10-satisfiability.md#5-a-state-of-r-is-a-model-of-the-program-as-h-scopes-it)).
   What *Ein certifies* claims is cited from there, not argued again.
5. **A flagged program is never certified**, by definition, and nothing is claimed of its answer.
6. **The two fixed points** of [Q-M2.4](../open_questions/answer/Q-M2.4-is-the-fixed-point-syntactic-or-semantic.md).
   Equal texts have equal answers, by 1; the converse fails, and
   [transformations](../../../docs/ein/reasoning/03-rule-evolution/03-transformations.md) is the kernel's own
   list of rewrites that change a program's text and not its answer. The account states the fixed point the loop
   was built to stop on ([S2.4.1](../p2.4_loop/s2.4.1_state_machine.md)) in terms of $\operatorname{Ans}(P)$; the
   `K*ᵢ` of the plan of August named neither $\mathrm{root}$ nor the answer.

### The central quantity

`P(faithful(Tₙ) ∧ correct(Tₙ))` as a function of what the kernel is allowed to say, written with what ran: the
renderings among F0 to F8 that [P2.6](../p2.6_ablations/README.md) measured, and the arm, B0 to B6
([README § The arms](../README.md#the-arms)). Beside it the two conditional probabilities, given *certified*, under
each reading the user has not ruled out. The ablation tables are its values, cited by their records under
`einaf/experiments/`.

## Notation

`docs/ein/reasoning/` has one notation, [the table of its README](../../../docs/ein/reasoning/README.md#notation),
and Stage M's letters collide with it:

| Stage M's letter | what the table already means by it |
|---|---|
| `R`, the relations | $R$, the region |
| `P`, the provenance; `P(…)`, a probability | $P$, the program |
| `K`, a five-tuple | $K$, a knowledge base, a finite set of facts |
| `Γ`, the rules and constraints | $\Gamma$, a claim (the satisfiability page's own letter) |
| `T`, a theory | $T_P$, the consequence operator; $T$, a closed set (the page of finding R) |
| `E`, the evaluation | $E$, the minimal discharging sets |
| `θ`, the model's parameters | $\theta$, a substitution |
| `x`, the text | $x$, a variable |
| `v`, the verdict | $v$, an object |
| `n`, the last step | $n = \lvert H \rvert$ |

Outside the mathematics two more things carry these letters: the entries of
[the evaluation of Ein](../evaluation/README.md) are `E-n`, and the feedback levels are F0 to F8 beside $F$, the
facts.

**The rule the account keeps.** A symbol of the notation table means in the account what it means there. An object
of the loop takes a name that is in neither of the table's two lists, and the account opens with the map from
Stage M's letters to its own. Which names is the writing stage's; the default is words set as operators rather
than single letters, since few letters are free.

## Where it is written, and to what rules

- **Under `docs/einaf/`**, as a page or pages named by the convention that tree's first pages set
  ([S2.1.1](../../../docs/einaf/README.md)). EinAf may name Ein: the account
  links pages of `docs/ein/reasoning/` and of `08-reasoning-in-code/`. **Ein never names EinAf**: no page under
  `docs/ein/` links the account, and none changes for it.
- **Independently of the Rust, for the loop too.** A definition is stated so that a second harness would be held
  to it. Which module of `einaf/src/` implements a definition, and which test holds a proposition, stands beside
  the account in the way `08-reasoning-in-code/` stands beside the reasoning. A proposition with no test is said
  to be an argument only.
- **The kernel's pairing is cited, not rebuilt.** The acceptance the plan of August wrote, *every symbol names
  the kernel page and the test that holds it*, cannot be met inside a page of `reasoning/`, which names no test;
  it is met by `08-reasoning-in-code/` as it stands.
- **A sentence of `docs/ein/` found wrong while the account is written** is an entry of
  [the evaluation of Ein](../evaluation/README.md), not an edit made here.
- **The conventions**: by default those of `docs/ein/` for a formula and a diagram (TeX; a diagram a file with its
  picture), so that the two halves read as one. The checks that hold them are Ein's and do not read
  `docs/einaf/`.

## Stages, in shape

Named and written at the Level C gate.

**S2.9.1 — The map and the notation.** Stage M's sketch, line by line, against the pages that state the kernel's
half; each difference said once, with its page; the table of colliding letters and the account's own names. It
needs nothing but the tree and `docs/einaf/`.

**S2.9.2 — The loop.** The objects, the two arrangements, the propositions with their arguments and tests. It
describes [P2.4](../p2.4_loop/README.md) as built.

**S2.9.3 — The central quantity, and what is left.** The probability with the feedback functions that ran and the
readings of *certified* in force, after the Level C gate; and one paragraph each on what the account does not
state and whose it is: the fragment of first-order logic Ein covers
([M1g's P1g.4](../../m1g_logic/p1g.4_first_order_logic/README.md), which took
[F1b](../../followups/f1b_logical_formulation.md)'s question and has not started), and the categorical reading
([F1](../../followups/f1_categorical_formulation.md), not started).

## Acceptance

- Every symbol of [§ Stage M](../EinAf.md#stage-m--formalize-the-conceptual-model) is either cited to the page of
  `docs/ein/reasoning/` that defines the kernel's notion, with the difference from the sketch stated where there
  is one, or defined in the account as an object of the loop.
- No symbol of the notation table means anything else in the account.
- Both arrangements have a formal shape, and the rendering is an object of the account.
- Each proposition names the test that holds it, of EinAf's or of Ein's, or says that it is an argument only.
- The central question is written with the set of feedback functions that ran, and the ablation tables are cited
  as its values.
- Nothing under `docs/ein/` names the account or was changed for it.

## Risks

- **Restating the kernel and getting it subtly wrong.** The reason to cite: a claim of the reasoning pages is
  paired with the test that holds it, and a paraphrase of it is held by nothing.
- **An account of a loop that was not built.** S2.9.2 is written of the code, each definition beside its module.
- **Two claimants for one question.** The plan of August had this phase answer *which fragment Ein decides*;
  that is M1g's now, and the account states no fragment of its own.
- **A venue that wants less, or more.** The account is written once, at its own depth; what M5's venue takes is
  M5's cut.
- **A second notation by drift.** The map from Stage M's letters is written first, and the acceptance holds every
  symbol to the table.

## Connections

- [`EinAf.md` § Stage M](../EinAf.md#stage-m--formalize-the-conceptual-model).
- [`docs/ein/reasoning/`](../../../docs/ein/reasoning/README.md), the kernel's half, and
  [`08-reasoning-in-code/`](../../../docs/ein/ein.rs/08-reasoning-in-code/README.md), its tests;
  [`13-glossary.md`](../../../docs/ein/user/04-reference/13-glossary.md).
- [`analysis/audit_p2.7_p2.10_neighbours.md`](../analysis/audit_p2.7_p2.10_neighbours.md) § A.5 and § B.3: the
  old text against the tree.
- [F26](../../followups/f26_consistency_and_completeness.md), whose trigger this phase is: *when M2 or M5 has to
  say in a paper what Ein's verdicts certify*. Its glossary of senses is M1g's S1g.4.5; proposition 4 is where the
  account says it for the loop.
- [F12](../../followups/f12_rules_and_relations/ideas.md) and [F15](../../followups/f15_math_formulae/ideas.md),
  the user's notes on rules as operators and as formulas; F15's export of rules as formulas is taken up by M1g.
- [M1h](../../m1h_beyond_word_problems/README.md), which hands M2 by name the case *the solution could be
  correct, but reasoning not*: the account's `correct` and `faithful` are where that case is defined.
- [M5](../../m5_presentation/README.md), the reader of the account.

## Defaults taken at the re-plan

Each is the re-plan's, recorded so that the user can correct it.

1. **A program of the loop is written as the kernel writes a program**, $P$ with its $K_0$; Stage M's `T` is not
   kept.
2. **The loop's objects are named by words, not single letters**; the writing stage picks them.
3. **The depth is that of a page of `docs/ein/reasoning/`**: definitions, and propositions with their arguments.
4. **The account follows `docs/ein/`'s conventions for formulas and diagrams**, in `docs/einaf/`.
5. **A proposition's test is named beside it**, and one with none is said to be an argument only.
6. **The fragment and the categorical reading are left** to M1g and F1, each with a paragraph.
7. **A wrong sentence found in `docs/ein/` is recorded in the evaluation**, not fixed here.

Dropped as moot, each by [audit § A.5](../analysis/audit_p2.7_p2.10_neighbours.md#a5-p29): the kernel half as a
thing still to write (33); *the typed hypergraph* (34); the model as *a complete, consistent leaf* (36); the
verdict *read off `k` with `exhausted`* and a fourth state `Aborted` (37); the three divergences the account *must
add*, of which the first is written in the reasoning pages, the second is stale and the third false (38, 39);
waiting for M5's venue; and the phase's estimate.
