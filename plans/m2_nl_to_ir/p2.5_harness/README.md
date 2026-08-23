# P2.5 — Baselines, metrics, records (Stages D, H, N)

**Estimate:** 2.5 weeks — 4 stages, 12 days. The first stage is written to
run **before P2.2's first number is taken**, not after P2.4 closes; see
§ Order.
**Depends on:** [P2.3](../p2.3_benchmark/README.md) — the splits;
[P2.4](../p2.4_loop/README.md) — B3–B5 are the loop at F0 / F2 / F8.
**Blocks:** [P2.6](../p2.6_ablations/README.md) and
[P2.7](../p2.7_failure_scaling_generalization/README.md) — both are more
rows of the table this phase defines, over the same records.
**Research plan:** [`EinAf.md` § Stage D](../EinAf.md#stage-d--establish-baselines),
[§ Stage H](../EinAf.md#stage-h--define-rigorous-metrics),
[§ Stage N](../EinAf.md#stage-n--reproducibility).

---

## Three stages of the plan, one phase — and why

Stages D, H and N are the *experimental method*: what is compared, what is
measured, and what is kept. They are one phase because none of the three is
usable without the other two — a baseline without a metric is a demo, a
metric without a record is a number nobody can re-take — and because all
three are **defined before the first table and not changed after it**. The
plan puts reproducibility at N, near the end; this milestone puts the record
format first, because M1a's whole measurement discipline
([`bench_env.sh`](../../../utils/bench_env.sh), the frozen PyPy columns,
`cost_ms` as a measured claim) was that a number without its instrument and
machine state is not a number.

### Order

[S2.5.1](s2.5.1_experiment_record.md) — the record — is written and used
from P2.2's first seed-set run onward. [S2.5.3](s2.5.3_metrics.md) — the
metrics — is written before P2.2's acceptance numbers are taken, so that the
one-shot rates are already in the final vocabulary. [S2.5.2](s2.5.2_baselines.md)
needs the loop and waits for P2.4. [S2.5.4](s2.5.4_first_table.md) is the
phase's close.

### The record (N)

One experiment is one immutable directory — the plan's
`experiment-0042/` — holding `config.json`, the prompts by hash, the task
ids and the split digest, `generations.jsonl` (raw model output, every
call), `theories/` (every program, every iteration), `solver.jsonl` (every
`ein-feedback/1` object and the summary it came from), `transitions.jsonl`
(the loop's transcript), `metrics.json`. The config names the model and its
file SHA, the generation parameters, the seed, the Ein commit, the stdlib
manifest SHA, the resolved `SolverConfig`, the feedback level, the strategy,
the budgets. Every table and plot in this milestone is generated **from
records only** — a number that cannot be traced to a record directory is not
in a table. Where the records live (`experiments/` at the root, not
`docs/`, because they are data) and the one canonical workflow that
produces them are S2.5.1's.

### The baselines (D)

| | condition | what it isolates |
|---|---|---|
| **B0** | direct LLM answer | what the model knows |
| **B1** | LLM with explicit reasoning / self-reflection | what thinking longer buys, with no kernel |
| **B2** | LLM → Ein, one shot ([P2.2](../p2.2_formalizer/README.md)) | formalization alone |
| **B3** | LLM → Ein → generic retry (the loop at **F0**) | more inference, no information |
| **B4** | LLM → Ein → verdict feedback (the loop at **F2**) | one word of information |
| **B5** | the full loop (the loop at **F8**) | everything the kernel can say |

B3–B5 run the loop with strategy *repair* fixed, so that the feedback level
is the only thing that varies down the table; *regenerate* and
*alternatives* are rows of [P2.6](../p2.6_ablations/README.md), not
baselines ([S2.5.2](s2.5.2_baselines.md)).

The comparison that matters is not *LLM vs Ein* — B5 spends more calls than
B0 — but **same model, similar inference budget, different feedback
information**. [S2.5.2](s2.5.2_baselines.md) defines the budget (calls,
input and output tokens, wall) and the matching rule: B1 gets the budget B5
spends, as reflection turns; B3 gets it as retries. Where the plan says
*where possible add another symbolic backend*, [P2.8](../p2.8_representations/README.md)
is that, later and separately.

### The metrics (H)

Four layers, and two conditional probabilities that are the milestone's
most informative numbers:

| layer | metrics | on which instances |
|---|---|---|
| **H1** final task | answer accuracy; exact match; unique-answer accuracy; **ambiguity detection**; **contradiction detection** | all — the last two need the three verdict classes of [S2.3.2](../p2.3_benchmark/s2.3.2_generators.md) |
| **H2** formalization | parse success; load success; solver acceptance; **semantic faithfulness**; constraint recall; constraint precision | the last three against the canonical theory on synthetic instances — exact; on external ones, [Q-M2.3](../open_questions.md#q-m23--what-is-the-unit-of-faithfulness-without-a-gold-theory)'s judge, marked as such |
| **H3** repair | fraction repaired; iterations to repair; new errors introduced; **semantic hallucination rate**; repair-cycle rate | every loop run, from the transcript |
| **H4** system | LLM calls; tokens in / out; wall; Ein runtime; enterings; saturation rounds; memory; **cost per correctly solved task** | every run, from the record — the Ein numbers are the summary's `stats` |

And:

```text
P(answer correct      | Ein certifies k = 1 ∧ exhausted)   — is certification a reliability signal?
P(formalization faithful | Ein certifies k = 1 ∧ exhausted) — or a formally valid lie?
```

The gap between the two is the plan's *faithfulness vs satisfiability*
distinction as a number, and it is reported in every table that reports
either.

## Stages

| ID | title | est. | ends with |
|---|---|---:|---|
| [S2.5.1](s2.5.1_experiment_record.md) | The experiment record and the canonical workflow | 3 d | the directory layout and the config schema; `einaf run / evaluate / report` (or the name Q25 gives them); records immutable, tables generated from them; [Q-M2.1](../open_questions.md#q-m21--when-is-the-kernel-frozen) decided — frozen per experiment, by commit |
| [S2.5.2](s2.5.2_baselines.md) | The baseline family B0–B5 and the budget match | 3 d | six conditions as six configs; the budget definition and the matching rule; answer extraction for B0 / B1 per source, with its own error rate measured |
| [S2.5.3](s2.5.3_metrics.md) | The four metric layers | 3 d | every metric defined once, computed from records by one module, with a fixture per metric whose value is known by hand; the two conditional probabilities |
| [S2.5.4](s2.5.4_first_table.md) | The first main table | 3 d | B0–B5 on `val`, per family and aggregate, H1–H4, with the two probabilities — the Level C gate's first row, and the point where the research question has a first answer |

## Acceptance

- A record directory exists for every number in every table of this
  milestone, and `einaf report` regenerates the tables from the directories
  with no other input.
- The six baselines run on the `val` split under the matching rule, and the
  budget each actually spent is a column of the table beside the budget it
  was given.
- Each metric has a hand-checked fixture; the conditional probabilities are
  reported with their denominators.
- The first main table is committed with its records, and the README of
  the experiment directory states what the table is evidence for and what
  it is not — before any ablation is run.

## Risks

- **Answer extraction for B0 / B1 is a metric of its own.** A free-text
  answer matched against `(D)` or `Norwegian` by a regex is a source of
  error that favours whichever side is stricter; S2.5.2 measures the
  extractor's agreement with a hand-read sample and reports it.
- **Budget matching is a judgement.** Tokens are not calls and calls are not
  wall; the stage picks one primary (output tokens) and reports the others,
  and the plan's phrase *similar inference budget* is made a number with a
  tolerance, stated.
- **The first table is not the result.** It is `val`; the `test` split stays
  frozen until [P2.10](../p2.10_result_artifact_demo/README.md), and the
  table is labelled with the split it is on.

## Connections

- [`EinAf.md` § Stage D](../EinAf.md#stage-d--establish-baselines),
  [§ Stage H](../EinAf.md#stage-h--define-rigorous-metrics),
  [§ Stage N](../EinAf.md#stage-n--reproducibility).
- [`utils/bench_env.sh`](../../../utils/bench_env.sh), [`baseline.md`](../../../docs/history/m1a_rust/measurements/baseline.md),
  [`corpus_cost.md`](../../../docs/history/m1a_rust/measurements/corpus_cost.md) —
  the measurement discipline the record inherits: machine state named,
  instrument named, frozen columns marked.
- [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md) — the four
  oracle ablations (*NL vs IR × known vs unknown theory*), which are four
  more conditions in S2.5.2's vocabulary and [P2.6](../p2.6_ablations/README.md)'s
  G8 / G9.
- [M5](../../m5_presentation/README.md) — § 6 *Experimental method* is
  this phase written up.
