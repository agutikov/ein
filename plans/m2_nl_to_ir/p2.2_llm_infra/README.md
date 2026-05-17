# P2.2 — LLM infra (acva-mirrored)

**Estimate:** 1-2 weeks.
**Depends on:** P2.1 S2.1.6 (model decided).
**Blocks:** P2.3, P2.4 (both call the LLM).

## Goal

Stand up the local LLM as a separate process — `llama.cpp`'s
`llama-server` in a Docker Compose service — and provide a thin
Python client. Pattern lifted from
[`/home/user/work/acva/packaging/compose/docker-compose.yml`](../../../acva/packaging/compose/docker-compose.yml).

Why a separate process: stable warm cache; never blocks the
ein-bot Python; easy to swap models; honest reproducibility via
container digest + model file SHA pinning.

## Stages

| ID      | Title                                  | Duration |
|---------|----------------------------------------|----------|
| S2.2.1  | Compose service + model pin            | 3-4 days |
| S2.2.2  | Python HTTP client + GBNF + retries    | 3-4 days |

## Acceptance

- `docker compose -f packaging/compose/llm.yml up -d` starts the
  llama-server with a pinned model.
- `python -m ein_bot.llm.smoke "Hello"` returns a constrained
  greeting using the smallest GBNF grammar in `grammars/`.
- Model file SHA + container image digest are recorded in
  `docs/decisions/M2-model-gbnf.md`.
- HTTP client retries on transient errors and surfaces
  rate-limit / context-overflow errors as typed exceptions.

## Connections

- [acva](../../../acva/README.md) — same pattern, Python client
  this time instead of C++.
- [Idea 01 §Self-modifying constraint language](../../../docs/ideas/01-self-modifying-constraint-language.md) —
  the future GBNF-self-modification work
  ([F2](../../followups/f2_self_modifying_language.md)) reuses
  this infra unchanged.
