# M2 — the registry of benchmarks

Every benchmark M2's files name, one row each: what it is, where it comes from (the paper, and the data where it is
published), where M2 names it, and what M2 does with it. Written 2026-11-26 on the user's word, *"also in this
milestone make a registry of all mentioned benchmarks with links where they were used and links to their sources"*.
It is the process's; a stage that names a benchmark not here adds its row.

**What it covers**: the milestone's plans, audits, ledgers and [the reading list](reading.md) with
[the notes on papers](reading/). **No file of a benchmark is in the tree** ([the audit of P2.3](analysis/audit_p2.3.md),
its datasets row): [S2.3.3](p2.3_benchmark/s2.3.3_external_benchmarks.md) is where one is first fetched, with its
version, digest and licence. Re-take the list of names with

```sh
grep -rhoE "ProofWriter|FOLIO|CLUTRR|BBH|PrOntoQA|AR-LSAT|LogiQA|ReClor|GSM8K|BoardgameQA|bAbI|Lean Workbook|TPTP" plans/m2_nl_to_ir | sort | uniq -c
```

## 1. Adapted: P2.3's external sources

[S2.3.3](p2.3_benchmark/s2.3.3_external_benchmarks.md) writes an adapter for each, in this order, each the external
source of one of [P2.3's families](p2.3_benchmark/README.md#the-shape); [S2.3.4](p2.3_benchmark/s2.3.4_splits.md)
puts their items into the splits.

| benchmark | what it is | source | where M2 names it | M2's use |
|---|---|---|---|---|
| **BIG-Bench Hard** (BBH): `logical_deduction_{three,five,seven}_objects` | an order from partial statements, a multiple-choice question | Suzgun et al. 2022, [arXiv:2210.09261](https://arxiv.org/abs/2210.09261); data [suzgunmirac/BIG-Bench-Hard](https://github.com/suzgunmirac/BIG-Bench-Hard); the task in [BIG-bench](https://github.com/google/BIG-bench/tree/main/bigbench/benchmark_tasks/logical_deduction) | [S2.3.3](p2.3_benchmark/s2.3.3_external_benchmarks.md), [P2.3](p2.3_benchmark/README.md), [S2.3.4](p2.3_benchmark/s2.3.4_splits.md), [Q-M2.3](open_questions/answer/Q-M2.3-what-is-the-unit-of-faithfulness-without-a-gold-theory.md), the audits of [P2.3](analysis/audit_p2.3.md), [P2.5 and P2.6](analysis/audit_p2.5_p2.6.md) and [P2.7 to P2.10](analysis/audit_p2.7_p2.10_neighbours.md); the family in [EinAf.md](EinAf.md#logical-deduction), [S2.3.1](p2.3_benchmark/s2.3.1_families_and_seed_set.md), [S2.3.2](p2.3_benchmark/s2.3.2_generators.md); [the notes on Logic-LM](reading/2305.12295.md) (as *LogicalDeduction*) | the external source of *logical deduction*, and of *ordering* read by position; `dev`, `val` and `test` |
| BBH: `tracking_shuffled_objects_{three,five,seven}_objects` | an object's place after a sequence of swaps | as above; the task in [BIG-bench](https://github.com/google/BIG-bench/tree/main/bigbench/benchmark_tasks/tracking_shuffled_objects) | [S2.3.3](p2.3_benchmark/s2.3.3_external_benchmarks.md), [P2.3](p2.3_benchmark/README.md) | the external source of *object tracking*, a family the library has no theory of; `val` and `test` only |
| BBH: `temporal_sequences` | when a person could have done something, from a day's events | as above; the task in [BIG-bench](https://github.com/google/BIG-bench/tree/main/bigbench/benchmark_tasks/temporal_sequences) | [S2.3.3](p2.3_benchmark/s2.3.3_external_benchmarks.md), [P2.3](p2.3_benchmark/README.md) | the external source of *temporal*; `val` and `test` only |
| **CLUTRR** | kinship stories: the relation between two people, by the length of the chain | Sinha et al. 2019, [arXiv:1908.06177](https://arxiv.org/abs/1908.06177); data [facebookresearch/clutrr](https://github.com/facebookresearch/clutrr) | [S2.3.3](p2.3_benchmark/s2.3.3_external_benchmarks.md), [P2.3](p2.3_benchmark/README.md), [S2.3.4](p2.3_benchmark/s2.3.4_splits.md), [Q-M2.3](open_questions/answer/Q-M2.3-what-is-the-unit-of-faithfulness-without-a-gold-theory.md), [the analysis](analysis/README.md), [the audit of P2.3](analysis/audit_p2.3.md); [the notes on SatLM](reading/2305.09656.md) | the external source of *graph*; `test`, developed on short chains and tested on long |
| **ProofWriter** | facts and rules in controlled English, a question answered *True*, *False* or *Unknown*, by proof depth, closed-world and open-world | Tafjord et al. 2020, [arXiv:2012.13048](https://arxiv.org/abs/2012.13048); data [allenai.org/data/proofwriter](https://allenai.org/data/proofwriter) | [S2.3.3](p2.3_benchmark/s2.3.3_external_benchmarks.md), [P2.3](p2.3_benchmark/README.md), [S2.3.4](p2.3_benchmark/s2.3.4_splits.md), [the analysis](analysis/README.md), the audits of [P2.3](analysis/audit_p2.3.md) and [P2.5 and P2.6](analysis/audit_p2.5_p2.6.md); the notes on [SatLM](reading/2305.09656.md) and [Logic-LM](reading/2305.12295.md) | an external source of *set and category*; `test` |
| **FOLIO** | premises and a conclusion in English, *True*, *False* or *Unknown*, with first-order annotations | Han et al. 2022, [arXiv:2209.00840](https://arxiv.org/abs/2209.00840); data [Yale-LILY/FOLIO](https://github.com/Yale-LILY/FOLIO) | [S2.3.3](p2.3_benchmark/s2.3.3_external_benchmarks.md), [P2.3](p2.3_benchmark/README.md), [S2.3.4](p2.3_benchmark/s2.3.4_splits.md), [Q-M2.3](open_questions/answer/Q-M2.3-what-is-the-unit-of-faithfulness-without-a-gold-theory.md), the audits of [P2.3](analysis/audit_p2.3.md) and [P2.5 and P2.6](analysis/audit_p2.5_p2.6.md); [the notes on Logic-LM](reading/2305.12295.md) | an external source of *set and category*; `test`; its annotations a reference theory for the faithfulness sample, never shown to a model |

## 2. Named and not adapted

| benchmark | what it is | source | where M2 names it | why not |
|---|---|---|---|---|
| **LogiQA** | multiple-choice logical reading comprehension, from Chinese civil-service exams | Liu et al. 2020, [arXiv:2007.08124](https://arxiv.org/abs/2007.08124); data [lgw863/LogiQA-dataset](https://github.com/lgw863/LogiQA-dataset) | [S2.3.3](p2.3_benchmark/s2.3.3_external_benchmarks.md), [P2.3](p2.3_benchmark/README.md) (its default 13), the audits of [P2.3](analysis/audit_p2.3.md) and [P2.5 and P2.6](analysis/audit_p2.5_p2.6.md) | argument questions in the style of entrance exams, reading comprehension more than theory construction |
| **ReClor** | multiple-choice logical reasoning from law-school admission and graduate tests | Yu et al. 2020, [arXiv:2002.04326](https://arxiv.org/abs/2002.04326); data [whyu.me/reclor](https://whyu.me/reclor/) | [S2.3.3](p2.3_benchmark/s2.3.3_external_benchmarks.md), [P2.3](p2.3_benchmark/README.md), [the audit of P2.3](analysis/audit_p2.3.md) | as LogiQA |
| **GSM8K** | grade-school arithmetic word problems | Cobbe et al. 2021, [arXiv:2110.14168](https://arxiv.org/abs/2110.14168); data [openai/grade-school-math](https://github.com/openai/grade-school-math) | [S2.3.3](p2.3_benchmark/s2.3.3_external_benchmarks.md), [P2.3](p2.3_benchmark/README.md); [the notes on SatLM](reading/2305.09656.md) (as *GSM*) | would be the word problems' external source; P2.3 keeps the corpus's sixteen and its generators |
| **ARC** (the Abstraction and Reasoning Corpus, ARC-AGI) | grid puzzles: a transformation induced from a few examples | Chollet 2019, [arXiv:1911.01547](https://arxiv.org/abs/1911.01547); [arcprize.org](https://arcprize.org/) | [S2.3.3](p2.3_benchmark/s2.3.3_external_benchmarks.md), [the audit of P2.3](analysis/audit_p2.3.md) | out by the user's criterion in [F13](../followups/f13_puzzles_beyond_zebra/ideas.md), text only; its difficulty is rule induction, [F7](../followups/f7_rule_induction.md)'s subject |
| **bAbI** | twenty synthetic question-answering tasks | Weston et al. 2015, [arXiv:1502.05698](https://arxiv.org/abs/1502.05698); data [facebookarchive/bAbI-tasks](https://github.com/facebookarchive/bAbI-tasks) | [the audit of P2.3](analysis/audit_p2.3.md), which records that no plan names it | none planned |

## 3. Named only by the papers read

The datasets the two papers of [the notes](reading/) evaluate on, beside those above. M2 adapts none of them; they are
here because the notes compare the papers' numbers on them.

| benchmark | what it is | source | where M2 names it |
|---|---|---|---|
| **PrOntoQA** | synthetic deduction over a fictional ontology, answers *True* or *False* | Saparov and He 2022, [arXiv:2210.01240](https://arxiv.org/abs/2210.01240); data [asaparov/prontoqa](https://github.com/asaparov/prontoqa) | [the notes on Logic-LM](reading/2305.12295.md) |
| **AR-LSAT** (SatLM's *LSAT*) | analytical reasoning questions of the Law School Admission Test | Zhong et al. 2021, [arXiv:2104.06598](https://arxiv.org/abs/2104.06598); data [zhongwanjun/AR-LSAT](https://github.com/zhongwanjun/AR-LSAT) | the notes on [Logic-LM](reading/2305.12295.md) and [SatLM](reading/2305.09656.md) |
| **BoardgameQA** | deduction with contradictory rules and preferences among them | Kazemi et al. 2023, [arXiv:2306.07934](https://arxiv.org/abs/2306.07934) | [the notes on SatLM](reading/2305.09656.md) |
| **GSM-Sys** | GSM-style problems that need a system of equations | built by SatLM: Ye et al. 2023, [arXiv:2305.09656](https://arxiv.org/abs/2305.09656); code and data [xiye17/SAT-LM](https://github.com/xiye17/SAT-LM) | [the notes on SatLM](reading/2305.09656.md) |
| **Algebra** | algebra word problems | He-Yueya et al. 2023, [arXiv:2304.09102](https://arxiv.org/abs/2304.09102) | [the notes on SatLM](reading/2305.09656.md) |
| **Colored Objects** (SatLM's *ColoredObject*) | questions about the colours and places of objects on a surface | BIG-bench, [`reasoning_about_colored_objects`](https://github.com/google/BIG-bench/tree/main/bigbench/benchmark_tasks/reasoning_about_colored_objects) | [the notes on SatLM](reading/2305.09656.md) |
| **StructuredRegex** | regular expressions synthesised from a description and examples | Ye et al. 2020, [arXiv:2005.00663](https://arxiv.org/abs/2005.00663) ([ACL 2020](https://aclanthology.org/2020.acl-main.541)) | [the notes on SatLM](reading/2305.09656.md) |

## 4. Formalization and theorem proving

| benchmark | what it is | source | where M2 names it | M2's use |
|---|---|---|---|---|
| **Lean Workbook** | contest problems formalized in Lean 4 by an iterative autoformalization pipeline, with natural-language statements | Ying et al. 2024, [arXiv:2406.03847](https://arxiv.org/abs/2406.03847); data [internlm/Lean-Workbook](https://huggingface.co/datasets/internlm/Lean-Workbook) | [the reading list](reading.md), [S2.2.1](p2.2_formalizer/s2.2.1_contract.md), [Q-M2.3](open_questions/answer/Q-M2.3-what-is-the-unit-of-faithfulness-without-a-gold-theory.md), the audits of [P2.2](analysis/audit_p2.2.md) and [P2.4](analysis/audit_p2.4.md) | not a benchmark of M2's: its faithfulness test (compilation, back-translation, an NLI check) is candidate (d) of Q-M2.3 |
| **TPTP** | the problem library of automated theorem proving | Sutcliffe, [tptp.org](https://www.tptp.org/) | [the audit of P2.7 to P2.10](analysis/audit_p2.7_p2.10_neighbours.md) | none: the audit records that M1g names [M10](../m10_external_benchmarks/README.md) as where TPTP runs |

## 5. Puzzles

Classes of problem, not published datasets.

| puzzle | source | where M2 names it | M2's use |
|---|---|---|---|
| **The Zebra puzzle** (Einstein's) | *Life International*, 1962; [Wikipedia](https://en.wikipedia.org/wiki/Zebra_Puzzle); [Rosetta Code](https://rosettacode.org/wiki/Zebra_puzzle) | everywhere: the minimal goal ([Q-M2.17](open_questions/milestone/Q-M2.17-what-is-the-milestones-minimal-goal.md)); the text [`nl2smt/puzzles/zebra.txt`](../../nl2smt/puzzles/zebra.txt); [S2.2.0a](p2.2_formalizer/s2.2.0a_zebra_formulations.md)'s six encodings and [their probes](p2.2_formalizer/zebra_formulations/README.md); [S2.3.1](p2.3_benchmark/s2.3.1_families_and_seed_set.md), [S2.3.2](p2.3_benchmark/s2.3.2_generators.md), [the audit of P2.3](analysis/audit_p2.3.md) (its Einstein variant) | the family *attribute matching*, its external source the Zebra texts of `nl2smt/puzzles/` |
| **Logic grid puzzles** | [Wikipedia, *Logic puzzle*](https://en.wikipedia.org/wiki/Logic_puzzle) | [S2.3.1](p2.3_benchmark/s2.3.1_families_and_seed_set.md), [S2.3.2](p2.3_benchmark/s2.3.2_generators.md), [the audit of P2.3](analysis/audit_p2.3.md), [the reading list](reading.md) | *attribute matching* generated at other sizes and ontologies |
| **Knights and Knaves** | Smullyan; [Wikipedia](https://en.wikipedia.org/wiki/Knights_and_Knaves) | [the audit of P2.3](analysis/audit_p2.3.md): the corpus program `examples/binary_logic/knights.ein` | a corpus program with several answers |
| **n-queens** | [Wikipedia, *Eight queens puzzle*](https://en.wikipedia.org/wiki/Eight_queens_puzzle) | [the audit of P2.3](analysis/audit_p2.3.md): one of [M10](../m10_external_benchmarks/README.md)'s problems, at 4, 6 and 8, not written | none in M2 |
| **Sudoku** | [Wikipedia](https://en.wikipedia.org/wiki/Sudoku) | [Q9](open_questions/formalizer/Q9-ontology-provenance.md), its option C (a library of standard ontologies) | none |

## 6. M2's own benchmark

**Eight families and the word problems**, generated with an exact gold answer and a canonical program
([P2.3 § The shape](p2.3_benchmark/README.md#the-shape), [S2.3.1](p2.3_benchmark/s2.3.1_families_and_seed_set.md),
[S2.3.2](p2.3_benchmark/s2.3.2_generators.md)), with the external sources above beside them:

| family | external source |
|---|---|
| logical deduction | BBH `logical_deduction` |
| ordering | BBH `logical_deduction`, read by position |
| spatial | none |
| object tracking | BBH `tracking_shuffled_objects` |
| attribute matching | the Zebra texts of [`nl2smt/puzzles/`](../../nl2smt/puzzles/zebra.txt) |
| set and category | ProofWriter, FOLIO |
| graph | CLUTRR |
| temporal | BBH `temporal_sequences` |
| word problems | none adapted (GSM8K is not); the corpus's sixteen, [`examples/word_problems/`](../../examples/word_problems/README.md) |

**Not benchmarks here**: SMT-LIB and MiniZinc are named as languages a model may target
([P2.8](p2.8_representations/README.md)), and `nl2smt/smt/`'s three files as hand-written encodings, which
[M10](../m10_external_benchmarks/README.md) counts in its own corpus.

## Elsewhere in the repository

[`docs/lib/12-llm-and-reasoning-benchmarks.md`](../../docs/lib/12-llm-and-reasoning-benchmarks.md), the catalogue
outside both projects, lists BBH, ARC-AGI, ProofWriter, FOLIO, LogiQA, ReClor, GSM8K and others; CLUTRR is not in it.
[F13](../followups/f13_puzzles_beyond_zebra/ideas.md) is the ladder of external sources S2.3.3 starts from.
[M10](../m10_external_benchmarks/README.md) is the milestone of external benchmarks for the engine.
