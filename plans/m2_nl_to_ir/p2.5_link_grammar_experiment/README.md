# P2.5 — Link-grammar experiment

**Estimate:** 1-2 weeks.
**Depends on:** P2.4 closed (the baseline pipeline must exist for
comparison).
**Blocks:** decision on whether link-grammar ships in M2.
**Resolves:** [Q11](../open_questions.md#q11--link-grammar-value).

## Goal

Run a *measured* experiment to answer the user's open question:
**does feeding link-grammar output to the LLM improve NL → IR
quality?** If yes, integrate. If no, leave it deprecated.

> **The submodule is not registered.** M1a
> [S1a.10.5](../../m1a_rust/p1a.10_single_implementation/s1a.10.5_removal.md)
> deinitialised `nlp/link-grammar` — it had never been checked out, and every
> `git clone --recurse-submodules` was fetching it for an experiment that has
> not run. Registering it is this phase's first act:
> `git submodule add https://github.com/opencog/link-grammar.git nlp/link-grammar`.
> Nothing else about the phase changes; the two scratch scripts and the
> reading list in `nlp/` are untouched.

The hypothesis is plausible: link-grammar exposes head/dependent
structure that an LLM might use to disambiguate sentences with
multiple plausible readings. It is *also* plausible that a strong
modern instruction-tuned LLM already does that internally and the
link-grammar pre-pass is noise. The point is to *measure*, not
assume.

## Stages

| ID      | Title                                  | Duration |
|---------|----------------------------------------|----------|
| S2.5.1  | Link-grammar runner + serialiser       | 3-4 days |
| S2.5.2  | A/B benchmark + decision               | 4-5 days |

## Acceptance

- Decision document `docs/decisions/M2-link-grammar.md` with:
  - the benchmark setup (puzzle set, metrics, models, seeds);
  - the measured outcome (per-puzzle and aggregate);
  - the verdict — ship or deprecate.
- If verdict = ship, P2.4's pipeline gains a link-grammar pre-pass
  + flag.
- If verdict = deprecate, the submodule reference under `nlp/`
  is left in place as a historical reference (not active code).

## Connections

- The user's explicit open question.
- [`/home/user/work/Ein/nlp/`](../../../nlp/) — the existing
  submodule that triggered the question.
