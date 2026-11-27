# L-M2.44 — The text-analysis environment needs two Pythons and four workarounds, and takes 44 GB

**Status:** open
**Found:** by [S2.2f.1](../../s2.2f.1_the_check.md), 2026-11-27, making the environment
**Owner:** [S2.2f.2](../../s2.2f.2_the_analysis_written.md) and every stage that runs a parser again; the scripts are
[`tools/`](../../tools/)

## What

- **Two venvs**: Python 3.12 (uv-fetched) with torch 2.14.1 on CUDA 13.0 for amrlib, Stanza, spaCy and the word
  senses; Python 3.8 with torch 1.13.1 on CUDA 11.7 and fairseq 0.10.2 for IBM's parser, whose pins no newer Python
  meets (its sm_86 kernels run on the card's sm_89).
- **Four workarounds**, each in a script and said there: amrlib's three undeclared imports (`unidecode`,
  `cached-property`, `word2number`); transformers below 5 (4.57.6), since SPRING's tokenizer subclasses the slow
  `BartTokenizer`; SPRING's generation config switched off from returning a dict; IBM's
  `torch.hub.load('pytorch/fairseq', 'bart.large')`, which would build fairseq's GitHub main, pointed at fairseq's
  `bart.large` read by the installed 0.10.2.
- **Every tool fits 8 GB alone**, the largest Stanza with its coreference model at 4 000 MiB; spaCy on the CPU (no
  CuPy). Each run's outputs and peaks repeat byte for byte.
- **44 GB on disk** under `build/nlp/`: the venvs 9.6, uv's cache 10, the model archives 7.6 (amrlib 2.9, IBM and
  BART 4.7), extracted models 8.5 (IBM, BART) and 3.2 (amrlib), the hub's cache 7.3. The archives and uv's cache
  (17.6 GB) are needed only to make the environment again.
- **Nothing was written under `~`**: every cache, and `HOME` itself, pointed into `build/nlp/`; the guard `HOME`
  stayed empty.

## Evidence

[`check/README.md`](../../check/README.md) § 1; [`tools/setup.sh`](../../tools/setup.sh) and the pins beside it.

## What it costs

A stage that runs a parser sources `tools/env.sh` first, and in a worktree sets `EIN_NLP` to the main checkout's
`build/nlp`. The IBM venv is the one most likely to break with the host (an old CUDA runtime, an old Python).
