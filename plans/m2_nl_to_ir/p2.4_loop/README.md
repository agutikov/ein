# P2.4 — The loop (Stages E, F)

**Estimate:** 3 weeks — 5 stages, 15 days.
**Depends on:** [P2.1](../p2.1_kernel_as_instrumentation/README.md) —
`ein-feedback/1` is what every level of the ladder renders;
[P2.2](../p2.2_formalizer/README.md) — GENERATE is the one-shot formalizer,
unchanged.
**Blocks:** [P2.5](../p2.5_harness/README.md) — baselines B3, B4 and B5 are
this loop at feedback levels F0, F2 and F8; [P2.6](../p2.6_ablations/README.md) —
every ablation is a choice of level, depth or strategy made here.
**Research plan:** [`EinAf.md` § Stage E](../EinAf.md#stage-e--build-the-iterative-formalization-loop)
and [§ Stage F](../EinAf.md#stage-f--make-symbolic-feedback-progressively-richer).

---

## What closes here

The one-shot formalizer becomes `Tᵢ₊₁ = A_θ(x, Tᵢ, E(Tᵢ))`. The phase builds
the loop **as an instrument**: a state machine whose every transition is a
logged record, a formalizer that can *repair* a theory as well as regenerate
one, nine feedback renderers over one object, a faithfulness judge that runs
on every repair, and a termination policy with a cycle detector. None of it
is tuned here — what the loop is *worth* is [P2.5](../p2.5_harness/README.md)'s
table and [P2.6](../p2.6_ablations/README.md)'s ablations. What this phase
owes them is that every knob they turn is a named, logged parameter.

### The state machine, on this kernel

The plan's E1 has five states and three repair edges. On Ein, ANALYZE and
SOLVE are one process — `ein solve -e` saturates, searches and certifies in
one run, 29 ms on Zebra — so the machine is:

```text
GENERATE ──► CHECK ──────────► SOLVE ──────────► ANSWER
  (P2.2)      parse / load /     ein solve -e      k = 1 ∧ exhausted
              compile            ein-feedback/1
                │                   │
       invalid ─┘          contradiction, ambiguous (k > 1),
                           unknown (¬exhausted), answer absent
                                    │
                                    ▼
                                  REPAIR ──► CHECK …
```

Every edge writes one transition record — state, the feedback object, the
level it was rendered at, the prompt hash, the raw generation, the program's
syntactic hash and its semantic digest, tokens, wall, the Ein commit — and
the run's transcript is the sequence ([S2.4.1](s2.4.1_state_machine.md)).
`unknown` is an edge the plan insists on and the engine cannot name yet
([P2.1 § The sixth outcome](../p2.1_kernel_as_instrumentation/README.md#the-sixth-outcome-today));
the loop treats it as *the solver needs more budget or the theory is too
loose*, and which of the two it was is a finding the failure taxonomy
collects.

### The ladder, and where each rung comes from

The feedback level is **the experimental variable**. Nine levels, one
object, nine renderers — and most rungs are a *projection* of what the
engine already emits, which is why the ladder is a phase and not a
milestone:

| level | the plan's content | source in the engine | new work |
|---|---|---|---|
| **F0** | *try again* | — | the renderer that says nothing |
| **F1** | syntactic: undefined relation, type mismatch, arity mismatch | the `kind` and location [S2.1.2](../p2.1_kernel_as_instrumentation/s2.1.2_feedback_object.md) put on parse, load and compile diagnostics | the old S2.4.2 validator's three checks — *unknown atom*, *type mismatch*, *structural contradiction* — are the loader's and the grammar's already; the stage keeps only what neither catches |
| **F2** | the verdict: SAT / UNSAT / AMBIGUOUS | `verdict.type` × `exhausted` × `unsat_core`, as P2.1 maps them | one word |
| **F3** | cardinality: *4 models remain, the query has 3 bindings* | `verdict.k`, `solutions[].goal_bindings` | counting |
| **F4** | model difference: two models and *the relevant difference* | `verdict.solutions[]` — sorted fact lists, so a difference is a set difference | **new**: choose two representatives, take the symmetric difference, restrict it to the query's relations, render with the puzzle's `:why` templates |
| **F5** | unsat core: *these source-derived constraints jointly imply contradiction* | `verdict.unsat_core` — given facts, which carry `:source` | render the core as the English it quotes; the core is a minimum-cardinality frontier, not a MUS ([glossary](../../../docs/kernel/glossary.md)), and the renderer says so |
| **F6** | dependency: *`?owner` depends on nationality, on house, has no path to pet* | **nothing** — no crate computes a relation dependency graph (the census) | **new**, and outside the kernel: a static relation graph from the program — rule bodies → heads, activators → the rules they instantiate — and reachability from the query's relations; *no path* is the diagnostic |
| **F7** | provenance: *fact X ← rule R ← facts A, B* | `ein-events/1` `fire` lines (`rule`, `premises`, `derived`), the trace's per-step derivations | reassemble the chain for the facts F4 / F5 named; the trace renderer already does this for humans |
| **F8** | the combined object | all of the above | the union, and the plan's caution: *the system should not assume F8 is best* |

**Every level renders the same object**, so a level is a parameter of one
run and an ablation is two runs. That is the property [P2.6](../p2.6_ablations/README.md)
rests on.

### Repair, faithfulness, termination

**Repair vs regenerate** (E2). Two prompts, one parameter: *regenerate* gives
the model the text and the feedback; *repair* gives it the text, the current
theory and the feedback, and asks for an edited theory. The old S2.4.2
validator loop was *repair under F1, ≤ 3 attempts*; it is one cell of the
grid now. The case for repair is that the kernel localises: an unsat core
names facts, a missing dependency names a relation, and a model difference
names a sentence. [S2.4.2](s2.4.2_repair.md).

**Faithfulness** (E3). The dangerous path is *ambiguous → the model invents a
constraint → unique*. Every repair is judged against the source before it
counts: a fact whose `:source` span is not in the text, a rule or assumption
in B2's *generated* categories that the text does not license, a deleted
source fact — each is a faithfulness event in the transition record, and the
metric is built from them. [S2.4.4](s2.4.4_faithfulness.md) decides the
judge ([Q-M2.3](../open_questions.md#q-m23--what-is-the-unit-of-faithfulness-without-a-gold-theory)).

**Termination** (E4): verified success, unrecoverable invalidity, the
iteration budget, the token budget, the solver budget (`--max-time` /
`--max-enterings`, read from the feedback object, never from exit code 2),
a repeated theory, a repair cycle. Cycles by hash — the program's canonical
dump, and the saturated state's digest, which the engine defines as the
sorted fact list ([`defined_behaviour.md` § 2.4](../../../docs/kernel/defined_behaviour.md));
[Q-M2.4](../open_questions.md#q-m24--is-the-fixed-point-syntactic-or-semantic)
recommends stopping on the second and logging both.

**Alternatives kept alive** ([S2.4.5](s2.4.5_alternatives_as_hypotheses.md),
[Q8](../open_questions.md#q8--ambiguous-parses)). The old plan's one idea
the research plan does not have: a sentence the formalizer cannot resolve
may be emitted as alternatives and left to the kernel to prune, instead of
committed and caught a loop iteration later. It is a third strategy beside
regenerate and repair, and it is kept as one — an arm for
[P2.6](../p2.6_ablations/README.md), not the default.

## Stages

| ID | title | est. | ends with |
|---|---|---:|---|
| [S2.4.1](s2.4.1_state_machine.md) | The state machine and the transition log | 3 d | the five states and the edges on this kernel; the transition record; the termination policy with its seven conditions; cycle detection by both hashes; a run is a transcript |
| [S2.4.2](s2.4.2_repair.md) | Repair as well as regenerate | 3 d | the two prompts; the old validator's residue; a diff of the theory per repair in the record |
| [S2.4.3](s2.4.3_feedback_ladder.md) | The feedback ladder F0–F8 | 5 d | nine renderers of `ein-feedback/1`; F4 model difference and F6 relation dependency built; each level's output banked as a golden over the benchmark's anchors |
| [S2.4.4](s2.4.4_faithfulness.md) | Faithfulness | 3 d | the judge — mechanical over B2's categories always, human on the sample, model-assisted only with agreement reported; the faithfulness events; Q-M2.3 decided |
| [S2.4.5](s2.4.5_alternatives_as_hypotheses.md) | Alternatives as hypotheses | 1 d | whether `hrule` is the form or a new one is needed; the emission path; Q8 decided as a strategy, not a default |

## Acceptance

- The loop runs the Zebra smoke test and the seed set at every level F0–F8,
  regenerate and repair, and every run is a transcript of transition
  records a reader can replay without the model.
- Each of the seven termination conditions is hit by at least one fixture
  run, and the transcript names which.
- On a synthetic `ambiguous` instance, a loop at F2 that reaches `k = 1` is
  flagged by the faithfulness judge — the E3 failure mode is **detected by
  construction**, on the benchmark's own ground truth, before any table.
- F4's difference and F6's dependency analysis are tested over the
  benchmark's anchors: on `zebra2-minus-15` the difference between two of
  the 32 models names the deleted clue's relation; on a program whose query
  mentions a relation no rule reaches, F6 says so.

## Risks

- **The ladder's content leaks the answer.** F4 shows two models; a
  formalizer that copies a model's facts into the theory has "repaired" by
  looking at the answer. The faithfulness judge catches the unsourced fact;
  the ablation G5 reports the rate; and the renderer shows the *difference*,
  not the models.
- **F6 is the one genuinely new analysis and it is easy to overbuild.** The
  stage builds reachability over a relation graph and nothing else — no
  stratification, no well-foundedness, no NAF analysis (the engine's
  `naf_deps` is advisory and stays so). If an ablation in
  [P2.6](../p2.6_ablations/README.md) wants more, it asks.
- **The kernel cannot say `unknown`** until M1d; the loop's derivation from
  `(type, k, exhausted, core)` is the bridge and is marked as such in every
  record that used it ([Q-M2.1](../open_questions.md#q-m21--when-is-the-kernel-frozen)).

## Connections

- [`EinAf.md` § Stage E](../EinAf.md#stage-e--build-the-iterative-formalization-loop),
  [§ Stage F](../EinAf.md#stage-f--make-symbolic-feedback-progressively-richer).
- [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md) — the loop
  `(Pᵢ, Tᵢ) → Ein → Dᵢ`, `(text, Pᵢ, Tᵢ, Dᵢ) → LLM → (Pᵢ₊₁, Tᵢ₊₁)`, the
  semantic fixed point, the three neural actions; [F16](../../followups/f16_autoformalization/ideas.md) —
  the diagnostics list (*theory inconsistent, goal under-constrained,
  multiple models remain, rule r cannot fire, required property absent,
  candidate model is a counterexample*) the ladder's F2–F7 realise;
  [F17](../../followups/f17_formal_verification/ideas.md) —
  `Spec₀ → verify → counterexample → Spec₁`.
- [`docs/kernel/inference/events.md`](../../../docs/kernel/inference/events.md) —
  F7's source; the trace renderer (`--trace`, `--relevant`) — the human
  version of F7 that already exists.
- [Idea 03](../../ideas/03-three-task-classes.md) — solve / gaps /
  contradictions as one verdict, which is what makes F2 one field.
