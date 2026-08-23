# P2.2 — The formalizer (Stage B)

**Estimate:** 3 weeks — 5 stages, 15 days.
**Depends on:** [P2.1](../p2.1_kernel_as_instrumentation/README.md) — the
contract names `ein-feedback/1` as the form its diagnostics arrive in, and
[S2.1.3](../p2.1_kernel_as_instrumentation/s2.1.3_boundary.md) has fixed the
language the client and the passes are written in.
**Blocks:** [P2.4](../p2.4_loop/README.md) — the loop's GENERATE state *is*
this phase's one-shot; [P2.5](../p2.5_harness/README.md) — baseline B2 is
this phase's output unchanged. The **Level B gate** is shared with
[P2.3](../p2.3_benchmark/README.md): a formalizer is Level B only on tasks it
has not seen.
**Research plan:** [`EinAf.md` § Stage B](../EinAf.md#stage-b--define-autoformalization-as-an-explicit-task),
B1–B4.

---

## Goal

The neural component does **one difficult thing** and nothing else: turn a
natural-language problem into an executable Ein theory, one shot, under a
written contract — and the kernel answers. No solving by the model, no
repair yet (that is [P2.4](../p2.4_loop/README.md)), no benchmark yet (that
is [P2.3](../p2.3_benchmark/README.md)). The phase ends with `einaf from-text` —
the old plan's `ein from-text`, renamed because the kernel stays LLM-free and
the command is the harness's ([S2.2.5](s2.2.5_from_text.md)) — doing exactly
what the old plan's first acceptance criterion asked — the Zebra text to the canonical answer — and doing it as an
**experiment**: every run a record with the prompt hashes, the grammar hashes,
the model SHA, the seed and the raw generation
([S2.2.5](s2.2.5_from_text.md)).

## What the formalizer emits

The plan's B1 allows the output to be "an Ein program" and optionally a
decomposition first; this milestone fixes the decomposition, because the
kernel's own input has four parts and the user's notes say the fourth is
where the difficulty is:

```text
ontology   — relations with their signatures; the types and instances
theory     — which stdlib theories apply, asserted as activators:
             (import std.bijection)  (bijective pet-loc)  (transitive left-of)
             and, only when the library has none, rules written out
instance   — the facts the text states, each with :source "…" quoting it
query      — the goal, with the variables the text asks for
```

Three things follow and each is a stage's subject.

**The theory part is selection before synthesis.** [F12](../../followups/f12_rules_and_relations/ideas.md):
*select a theory, do not invent properties* — the formalizer asserts
`(transitive left-of)` and the engine instantiates the rule; it writes a
`rule` form only when no `std.*` module says what it needs. That ordering is
the user's *three neural actions* ([F13](../../followups/f13_puzzles_beyond_zebra/ideas.md):
reinterpretation, theory selection, theory synthesis) with the third made
expensive on purpose, and it is the activator-induction
[F7 B](../../followups/f7_rule_induction.md#connection-to-m2) said M2 could not
do without — the old plan's pipeline produced relations and facts and *no
activators*, so the engine would have sat idle. [S2.2.4](s2.2.4_passes.md).

**Every form carries where it came from.** B2's five categories —
*source-derived fact*, *generic theory (stdlib)*, *generated auxiliary
relation*, *generated inference rule*, *generated assumption* — are what
makes hallucination measurable later ([S2.4.4](../p2.4_loop/s2.4.4_faithfulness.md)).
The first is already ein-lang: a fact with `:source` is layered as a fact by
the engine's own provenance rule, and the old plan made the quote mandatory.
The other four are not, and how they are marked — a reserved keyword on the
form, or a sidecar in the record keyed by form — is [S2.2.1](s2.2.1_contract.md)'s
first decision, taken with the census's answer on whether the parser accepts
an unknown keyword on a fact.

**The model knows exactly what it is given.** B3: the ein-lang specification
(which pages, at which commit), the stdlib *catalogue* — one line per module
and per property, generated from [`stdlib/`](../../../stdlib/README.md) and
hashed — the few-shot examples, the problem. B4: every one of those is a
field of the record, and **a prompt change is an experimental change** —
there is no "improving the prompt" inside a phase's table, only a new
experiment id.

## Stages

| ID | title | est. | ends with |
|---|---|---:|---|
| [S2.2.1](s2.2.1_contract.md) | The formalization contract — B1–B4 | 2 d | one page, `docs/einaf/contract.md`: input, the four-part output, the five provenance categories and their marking, what the model receives, what is recorded; the reading list the old S2.1.1 asked for |
| [S2.2.2](s2.2.2_llama_server_and_client.md) | `llama-server` and the client | 3 d | the Compose service with a pinned image and a pinned model file; the client in S2.1.3's language, `grammar` field honoured, typed failures, a deterministic seed; [Q23](../open_questions.md#q23--local-model-choice) and [Q-M2.2](../open_questions.md#q-m22--must-the-model-be-local) decided by a measurement on the seed set |
| [S2.2.3](s2.2.3_gbnf.md) | GBNF for ein-lang | 3 d | `grammars/ein.gbnf` **generated from [`00_ebnf.md`](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md)**, one grammar per pass, the facts grammar parameterised on the ontology's relation names and instance atoms so an undeclared one is unparseable; the echo round-trip over the corpus; tokens/s measured; [Q24](../open_questions.md#q24--one-gbnf-or-many) |
| [S2.2.4](s2.2.4_passes.md) | The passes — ontology, theory, instance, query | 4 d | four passes with four prompts and four grammars; the theory pass selecting from the catalogue and synthesising only on a named condition; sentence-by-sentence facts with `:source`; [Q9](../open_questions.md#q9--ontology-provenance) |
| [S2.2.5](s2.2.5_from_text.md) | `einaf from-text`, one-shot, recorded | 3 d | the command, in the harness, not the kernel; the program assembled and handed to `ein solve`; the record written per run; Zebra from text to its canonical answer |

## Acceptance

- `einaf from-text examples/zebra.txt` — the Wikipedia statement, **to be
  checked in** beside the two hand encodings (no `.txt` exists today) — yields a program the kernel solves to
  `Norwegian / Water / House-1, Japanese / Zebra / House-5`, one shot, with
  the theory selected from `std.*` and not written out.
- On the seed set ([S2.3.1](../p2.3_benchmark/s2.3.1_families_and_seed_set.md)),
  one-shot: parse rate, load rate, verdict distribution and answer accuracy
  reported per puzzle, in a record — numbers, not a demo.
- Every generated form is attributable to one of the five B2 categories by
  reading the program and the record alone.
- Two runs with the same record fields produce the same program — the seed
  is honoured end to end — or the record names the field that moved.
- Nothing in the prompts contains a benchmark instance's solution: the
  few-shot examples are the Zebra smoke test and hand-written micro-examples,
  and Zebra is excluded from every split ([S2.3.4](../p2.3_benchmark/s2.3.4_splits.md)).

## Risks

- **The theory pass is the hard part and has no prior art in the old plan.**
  If selection from the catalogue fails on the seed set — the model asserts
  `(transitive left-of)` and forgets `(bijective nation-loc)` — the loop
  ([P2.4](../p2.4_loop/README.md)) is where it gets a second chance, and the
  one-shot number is the baseline B2 the loop is measured against. A low B2
  is a finding, not a blocker.
- **GBNF makes syntax free and buys nothing semantic.** The plan says so
  (*syntax is the relatively easy part*) and the old plan knew it
  (*semantic validity is checked in P2.4*). The facts grammar parameterised
  on the ontology is the one place the grammar does semantic work — an
  undeclared atom cannot be emitted — and it is kept.
- **Decoding speed under a recursive grammar.** The old T2.3.1.4 target,
  ≥ 30 tok/s on the dev machine, stands; if the full grammar misses it, the
  per-pass grammars are the answer and Q24 is decided by that number.
- **Local vs hosted** ([Q-M2.2](../open_questions.md#q-m22--must-the-model-be-local)).
  The phase builds for local; a hosted replication arm needs only a second
  client and loses GBNF, and the contract says what that costs.

## Connections

- [`EinAf.md` § Stage B](../EinAf.md#stage-b--define-autoformalization-as-an-explicit-task).
- [Idea 04](../../ideas/04-nlp-to-graph-to-solver-pipeline.md) — the
  pipeline sketch, realised here; [idea 01](../../ideas/01-self-modifying-constraint-language.md) —
  GBNF as the syntactic firewall; [`docs/lib/01`](../../../docs/lib/01-llm-constrained-generation.md).
- [F16](../../followups/f16_autoformalization/ideas.md), [F12](../../followups/f12_rules_and_relations/ideas.md),
  [F7 B](../../followups/f7_rule_induction.md#connection-to-m2), [F4 Q38](../../followups/f4_cross_cutting.md) —
  the LLM as fact / relation / type / rule extractor.
- [`stdlib/`](../../../stdlib/README.md) — the catalogue's source;
  [`07_stdlib_api.md`](../../../docs/kernel/ir/03-ein-lang/07_stdlib_api.md).
- `/home/user/work/acva/packaging/compose/docker-compose.yml` — the `llama`
  service (`ghcr.io/ggml-org/llama.cpp:server-cuda`) the Compose file is
  lifted from; outside this repo.
