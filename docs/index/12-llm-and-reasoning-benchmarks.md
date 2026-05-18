# Reasoning Benchmarks (LLM, symbolic, neuro-symbolic)

What "reasoning" means depends on the benchmark. Roughly:

| Family                 | What it tests                       |
|------------------------|-------------------------------------|
| symbolic logic         | formal deduction                    |
| commonsense reasoning  | everyday causal/world knowledge     |
| multi-hop reasoning    | inference chains over passages      |
| mathematical reasoning | arithmetic, algebra, olympiad math  |
| planning / search      | strategy / multi-step plans         |
| theorem proving        | machine-checked proofs              |
| neuro-symbolic         | LLM + symbolic engine integration   |
| agentic reasoning      | reasoning + tool use + memory       |

This file catalogues benchmarks; the *human-facing* puzzle classes (Zebra,
Knights/Knaves, hat puzzles, Monty Hall, paradoxes) live in
[`docs/ideas/09-puzzles-beyond-zebra.md`](../ideas/09-puzzles-beyond-zebra.md)
because they motivate the engine's *target capabilities*, not its tooling.

---

## 1. Broad reasoning suites

### BIG-Bench / BIG-Bench Hard
Google Research's collection of 200+ tasks (logical deduction, object
tracking, symbolic manipulation, causal reasoning, strategy). BBH = the
hardest subset on which LLMs lagged human performance.
- Repo: <https://github.com/google/BIG-bench>
- BBH paper: <https://arxiv.org/abs/2210.09261>

### MMLU — Massive Multitask Language Understanding
57 academic subjects (formal logic, math, law, philosophy, …). Knowledge +
reasoning mixed; not a pure reasoning benchmark.
- Paper: <https://arxiv.org/abs/2009.03300>

### ARC (Allen Institute)
Grade-school science questions; multi-step inference, causality, basic
physics. Predates the unrelated ARC-AGI below.
- Paper: <https://arxiv.org/abs/1803.05457>

### ARC-AGI (François Chollet)
Abstraction + few-shot generalisation: solve novel grid transformations
from a handful of examples. Designed *against* memorisation. Currently
the highest-prestige "AGI proxy" benchmark.
- Site: <https://arcprize.org/>

### GPQA — Graduate-level Google-Proof Q&A
Expert-level science questions resistant to web search.
- Paper: <https://arxiv.org/abs/2311.12022>

### Humanity's Last Exam
Frontier-difficulty general-reasoning benchmark.
- Site: <https://lastexam.ai/>

---

## 2. Commonsense / language reasoning

### HellaSwag
"What happens next" completion. Trivial for humans; long-time hard for LMs.
- Paper: <https://arxiv.org/abs/1905.07830>

### Winograd Schema Challenge
Pronoun-disambiguation that requires world knowledge ("The trophy didn't fit
into the suitcase because *it* was too big" — which?).
- Paper: <http://commonsensereasoning.org/2011/papers/Levesque.pdf>

### TruthfulQA
Resistance to confident misinformation; reasoning under misleading priors.
- Paper: <https://arxiv.org/abs/2109.07958>

---

## 3. Formal logic & deduction

### ProofWriter
Multi-hop deduction with explicit Horn-clause facts/rules; produces *proofs*
plus answers. Closest in spirit to ein-bot's three task classes.
- Paper: <https://arxiv.org/abs/2012.13048>

### FOLIO — First-Order Logic
Natural-language premises + FOL formalisations + entailment labels. Closer
to predicate-logic theorem proving than typical NLI.
- Paper: <https://arxiv.org/abs/2209.00840>

### LogiQA
Chinese (translated) LSAT-style logical reasoning MCQ. LMs historically
struggle; closer to human-style logic puzzles than ProofWriter.
- Paper: <https://arxiv.org/abs/2007.08124>

### ReClor
Reading comprehension + logical reasoning: assumptions, contradictions,
implication, flaw detection.
- Paper: <https://arxiv.org/abs/2002.04326>

---

## 4. Mathematical reasoning

### GSM8K
Grade-school math word problems. Made chain-of-thought prompting famous.
- Paper: <https://arxiv.org/abs/2110.14168>
- CoT: <https://arxiv.org/abs/2201.11903>

### MATH
Competition-style maths problems (AMC/AIME/olympiad). Multi-step.
- Paper: <https://arxiv.org/abs/2103.03874>

### AIME
American Invitational Mathematics Examination. Currently a *practical*
frontier reasoning benchmark for top LLMs.

---

## 5. Planning, search, agents

### Game of 24
Use four digits + arithmetic to make 24. Used in *Tree-of-Thoughts*.
- ToT: <https://arxiv.org/abs/2305.10601>

### Mini Crosswords
Constrained search + commonsense. Also in ToT.

### ALFWorld
Text-game planning benchmark.
- Site: <https://alfworld.github.io/>

### WebArena
Browser-agent benchmark; long-horizon tool use.
- Site: <https://webarena.dev/>

---

## 6. Theorem proving / formalised math

### Lean + miniF2F + ProofNet + LeanDojo
LLM-driven formal proof of olympiad math. Lean 4 has become the dominant
substrate for LLM-augmented proving.
- miniF2F: <https://github.com/openai/miniF2F>
- ProofNet: <https://github.com/zhangir-azerbayev/ProofNet>
- LeanDojo: <https://leandojo.org/>

Cross-references: [`03-theorem-proving-formal-methods.md`](03-theorem-proving-formal-methods.md).

---

## 7. CSP / SAT-style problem sets

Less standardised than the LLM benchmarks above; usually generated:

- **Sudoku** — domain-restricted, well-studied baseline.
- **Zebra-style** custom sets — *the* class of relevance to ein-bot.
- **Graph colouring** — n-colourability over benchmark graph families.
- **Scheduling / planning** — IPC (International Planning Competition).
- **Pigeonhole / Ramsey** — classical hard SAT instances.

These are not LM benchmarks but appear in comparisons of LLM vs SAT/SMT/CP-SAT
performance on structured CSP tasks. Cross-references:
[`02-solvers-csp-sat-smt.md`](02-solvers-csp-sat-smt.md).

---

## 8. Common architectures evaluated against these

| Architecture       | Idea                                         |
|--------------------|----------------------------------------------|
| Toolformer         | LLM calls external solvers as tools          |
| ReAct              | interleaved reasoning + action               |
| DSPy               | declarative LLM pipelines                    |
| Program-of-Thought | LLM emits programs (often Python)            |
| PAL                | program-aided language: solve via Python     |
| Graph-of-Thought   | reasoning over an explicit graph             |
| Tree-of-Thought    | search tree of intermediate thoughts         |
| SAT-augmented LLM  | constrained decoding from a SAT/CSP back-end |
| NeuroSAT           | neural SAT solving (learnt heuristics)       |

Cross-references: [`09-cognitive-architectures-neurosymbolic.md`](09-cognitive-architectures-neurosymbolic.md),
[`11-search-optimization-algorithms.md`](11-search-optimization-algorithms.md).

---

## 9. Where LLMs and symbolic solvers diverge

LLMs are good at: language, abstraction, heuristic decomposition, partial-
information reasoning.

Symbolic solvers (SAT/SMT/Prolog/Lean/planners) are good at: exhaustive
search, correctness guarantees, structured deduction.

Almost every frontier reasoning system today combines the two: the
*pattern-completion* part is delegated to the LLM, the *exhaustive-
correctness* part to a solver. This is the framing
[idea 04](../ideas/04-nlp-to-graph-to-solver-pipeline.md) commits to, and the
benchmarks on this page are how progress along that axis gets measured.

Cross-references: [`docs/ideas/02-graph-as-formal-substrate.md`](../ideas/02-graph-as-formal-substrate.md),
[`docs/ideas/04-nlp-to-graph-to-solver-pipeline.md`](../ideas/04-nlp-to-graph-to-solver-pipeline.md).
