# P2.10 — The result, the artifact, the demo (Stages O, Q)

**Estimate:** 1.5 weeks; phase README only. Its stage files are written
last, because two of the three are determined by the numbers.
**Depends on:** everything before it; specifically the **Level C gate**
([P2.7](../p2.7_failure_scaling_generalization/README.md)) for the claim,
[P2.8](../p2.8_representations/README.md) and
[P2.9](../p2.9_formal_account/README.md) for Level D's *comparison* and
*formal treatment*, and the frozen `test` split
([S2.3.4](../p2.3_benchmark/s2.3.4_splits.md)), which is opened **here and
nowhere earlier**.
**Blocks:** [M5](../../m5_presentation/README.md) — the paper (Stage P) is
written from this phase's result and artifact; the **Level D gate** closes
with M5's paper and this phase's six other layers.
**Research plan:** [`EinAf.md` § Stage O](../EinAf.md#stage-o--produce-the-central-research-result),
[§ Stage Q](../EinAf.md#stage-q--public-research-artifact); Stage P is
M5's.

---

## The result (O)

The plan names three admissible shapes and insists on the third:

> *Structured symbolic diagnostics improve iterative autoformalization
> compared with direct answering, one-shot formalization, generic retries,
> and self-refinement under comparable inference budgets.*
>
> *Unsat-core feedback substantially improves repair of over-constrained
> theories, while full provenance provides little additional benefit.*
>
> *Symbolic verification dramatically increases precision of accepted answers
> but does not improve overall task accuracy because semantic
> misformalization dominates.*

— positive, narrow, negative; *do not design the experiment around obtaining
a predetermined positive conclusion.* This phase's first act is to take the
sentence [P2.7](../p2.7_failure_scaling_generalization/README.md) wrote at the
Level C gate — the claim as `val` supports it — and run **exactly the
conditions that sentence names**, once, on `test`. The result is the
sentence `test` supports, which is the same sentence or a weaker one, never a
stronger one, and never a different one found by looking. The two tables —
the one that supports the claim and the one that would have refuted it —
are published side by side.

**The third shape is the one the kernel's own numbers predict.** The gap
between `P(correct | certified)` and `P(faithful | certified)` is where the
plan says the interesting problem is, and the milestone is built so that
the gap is measured rather than hidden: the faithfulness judge runs on every
repair, the benchmark has instances whose right answer is *ambiguous*, and
the baselines include reflection at matched budget. If the negative result is
what the table says, it is the result, and the plan's sentence — *that is
still research* — is the reading.

## The artifact (Q)

Seven layers, each independently usable, each with the phase that delivered
it:

| layer | what someone can do | delivered by | state at Level C |
|---|---|---|---|
| 1 executable system | run Ein, and run the loop | M1a; [P2.2](../p2.2_formalizer/README.md), [P2.4](../p2.4_loop/README.md); [`docs/install.md`](../../../docs/install.md) | the kernel has a release matrix awaiting its first tag; the loop needs its own channel, in Q25's language |
| 2 benchmark | inspect the tasks, the programs, the verdicts, the answers | [P2.3](../p2.3_benchmark/README.md) | a directory; versioned; the `test` split's digest published with the result |
| 3 experimental harness | rerun the evaluation | [S2.5.1](../p2.5_harness/s2.5.1_experiment_record.md) | `einaf run / evaluate / report`, or Q25's names |
| 4 raw results | read the generations and the solver outputs, not only the charts | the records of [P2.5](../p2.5_harness/README.md) – [P2.8](../p2.8_representations/README.md) | released as directories; size is the question, and an index with digests is the answer if the raw set is too large to ship |
| 5 technical documentation | the formal semantics and the architecture | [`docs/kernel/`](../../../docs/kernel/README.md); [P2.9](../p2.9_formal_account/README.md) | the kernel's exists; the loop's architecture page is this phase's to write |
| 6 research paper | questions, method, experiments, conclusions | [M5](../../m5_presentation/README.md) | Stage P; not this milestone's |
| 7 minimal demonstration | one command that shows the idea | **this phase** | — |

**The demo** is the plan's sequence, as one command on one instance chosen
from the benchmark for being short and for failing one-shot in the
instructive way:

```text
English problem → incorrect formalization → Ein detects ambiguity
→ symbolic diagnostic → LLM repairs the theory → unique model → verified answer
```

It prints the text, the first program, the feedback object *rendered at the
level that fixed it*, the repaired program with the diff, the verdict with
`k` and `exhausted`, and the answer rendered through the puzzle's own `:why`
templates — and it must be understandable without the paper. It runs
against the local model and the pinned Ein commit, and it is the one piece
of this milestone a reader meets first.

## Stages — the shape, for when it is written

- **S2.10.1** — the `test` run: the claim's conditions, once, and the
  sentence `test` supports; the supporting table and the refuting table.
- **S2.10.2** — the release: the seven layers checked as a list, each with
  its URL or path; the record index; the loop's architecture page; the
  `pip install` trip-wire of [Q-M1a.23](../../../docs/history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
  re-read, because *a paper's artifact package* was the one most likely to
  fire it.
- **S2.10.3** — the demo: the instance chosen, the command, the output as a
  checked-in transcript, and the one paragraph that explains it.

## Acceptance

- The claim on `test`, in one of the three shapes, with both tables.
- Seven layers, each usable without the others, each with a path.
- The demo runs from a fresh clone with the documented prerequisites and
  its checked-in transcript matches.
- **M2 done** — the milestone README's status and
  [`plans/README.md`](../../README.md)'s table updated with the date and the
  claim's sentence; Level D is then M5's paper away.

## Connections

- [`EinAf.md` § Stage O](../EinAf.md#stage-o--produce-the-central-research-result),
  [§ Stage Q](../EinAf.md#stage-q--public-research-artifact),
  [§ Level progression](../EinAf.md#level-progression).
- [M5](../../m5_presentation/README.md) — the paper; its § 9.3
  *Faithfulness vs satisfiability* is this phase's result read twice.
- [`docs/history/m1a_rust/`](../../../docs/history/m1a_rust/README.md) —
  the precedent for what a closed milestone leaves behind: the record, the
  measurements, the ledgers; this milestone's records are the same kind of
  artefact and go to `docs/history/` the same way when it closes.
