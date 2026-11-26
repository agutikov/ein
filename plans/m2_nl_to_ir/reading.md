# M2 — the reading list

What M2's work read, grouped by the question each entry informs, one line an entry. Written by
[S2.2.1](p2.2_formalizer/s2.2.1_contract.md) (its T2.2.1.5); a stage that reads something for its work adds the
line here. It is the process's, and no page of `docs/einaf/` cites it. Notes on a paper are under
[`reading/`](reading/), one file a paper named by its arXiv id, their formulas written as `$$` and `$`
(`python3 utils/md_math.py --check plans/m2_nl_to_ir/reading/*.md`).

## Constrained generation: what a grammar can hold a model to — informs [S2.2.3](p2.2_formalizer/s2.2.3_gbnf.md)

- [`docs/lib/01-llm-constrained-generation.md`](../../docs/lib/01-llm-constrained-generation.md): decoding-time constraints, GBNF, grammar-guided sampling and its failure when a grammar is tokenizer-misaligned; the catalogue the grammars start from.
- llama.cpp, [`grammars/README.md`](https://github.com/ggml-org/llama.cpp/blob/master/grammars/README.md): GBNF as llama.cpp compiles it, and the lazy grammar `llama-server` starts at a trigger rather than the first token, which is where a grammar after the thinking begins.
- [`analysis/host.md`](analysis/host.md#what-the-model-does-through-llama-server-a-probe): the host probe, `llama-server` on the GPU honouring a GBNF and a thinking model and a grammar not composing as they stand.

## Semantic parsing: a parser's reading of a sentence as the facts' source — informs the link-grammar arm of [S2.6.4](p2.6_ablations/s2.6.4_representation_ablations.md)

- [`docs/lib/10-nlp-semantic-parsing.md`](../../docs/lib/10-nlp-semantic-parsing.md): link grammar, dependency and semantic parsers, and what each gives a formalizer.
- [`nl2smt/README.md`](../../nl2smt/README.md): the Zebra puzzle's sentences through NLTK or link grammar to SMT, the proof of concept the arm starts from.

## Autoformalization: what a formalization is held to, and how its faithfulness is judged — informs the contract ([S2.2.1](p2.2_formalizer/s2.2.1_contract.md)), [S2.4.4](p2.4_loop/s2.4.4_faithfulness.md) and [M5](../m5_presentation/README.md)

- [F16](../followups/f16_autoformalization/ideas.md): *autoformalization is not translation*; the user's notes and the references below.
- Wu et al., *Autoformalization with Large Language Models* (2022), [arXiv:2205.12615](https://arxiv.org/abs/2205.12615): competition mathematics into Isabelle/HOL, a quarter formalized exactly; the founding measurement.
- *Process-Driven Autoformalization in Lean 4* (2024), [arXiv:2406.01940](https://arxiv.org/abs/2406.01940): the compiler's feedback as the training signal, the nearest published relative of this loop.
- *Lean Workbook* (2024): Lean compilation, back-translation and an NLI check as the faithfulness test, candidate (d) of [Q-M2.3](open_questions/answer/Q-M2.3-what-is-the-unit-of-faithfulness-without-a-gold-theory.md).
- *Autoformalization in the Era of Large Language Models: A Survey* (2025), [arXiv:2505.23486](https://arxiv.org/abs/2505.23486): the pipeline end to end, datasets, model design and verification.
- *Towards a Common Framework for Autoformalization*, AAAI 2026, [ojs.aaai.org/…/42132](https://ojs.aaai.org/index.php/AAAI/article/view/42132): natural language to formal representations for reasoning, planning and knowledge representation as one task; the contract's framing.

## Models that call a solver as a tool — informs [S2.4.6](p2.4_loop/s2.4.6_ein_as_a_tool.md) and [Q-M2.13](open_questions/einaf/Q-M2.13-who-calls-the-kernel.md)

- Gao et al., *PAL: Program-aided Language Models* (2022), [arXiv:2211.10435](https://arxiv.org/abs/2211.10435): the model writes a program and an interpreter answers; the filter's shape with Python for Ein.
- Pan et al., *Logic-LM* (2023), [arXiv:2305.12295](https://arxiv.org/abs/2305.12295): a problem formalized into a solver's language, the solver's error fed back for self-refinement. The user's notes: [`reading/2305.12295.md`](reading/2305.12295.md).
- Ye et al., *SatLM* (2023), [arXiv:2305.09656](https://arxiv.org/abs/2305.09656): a declarative specification handed to a SAT solver instead of a chain of thought. The user's notes: [`reading/2305.09656.md`](reading/2305.09656.md).
- Olausson et al., *LINC* (2023), [arXiv:2310.15164](https://arxiv.org/abs/2310.15164): first-order logic for a prover, with a vote over several formalizations.
- Schick et al., *Toolformer* (2023), [arXiv:2302.04761](https://arxiv.org/abs/2302.04761), and Yao et al., *ReAct* (2022), [arXiv:2210.03629](https://arxiv.org/abs/2210.03629): a tool called inside the model's own reasoning, the arrangement the tool arm takes.
- [`docs/lib/12-llm-and-reasoning-benchmarks.md`](../../docs/lib/12-llm-and-reasoning-benchmarks.md#8-common-architectures-evaluated-against-these): its table of architectures, PAL, Toolformer and ReAct among them.

## Benchmarks: which families and instances — informs [S2.3.3](p2.3_benchmark/s2.3.3_external_benchmarks.md)

- [F13](../followups/f13_puzzles_beyond_zebra/ideas.md): the user's notes on puzzles beyond Zebra and the three neural actions (reinterpretation, theory selection, theory synthesis), of which step 1 is the first.
- [`docs/lib/12-llm-and-reasoning-benchmarks.md`](../../docs/lib/12-llm-and-reasoning-benchmarks.md): the reasoning benchmarks, logic grid puzzles and word problems among them.
