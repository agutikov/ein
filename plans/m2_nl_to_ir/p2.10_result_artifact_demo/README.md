# P2.10 — The result, the artifact, the demo (Stages O, Q)

**Depth:** a phase README, on purpose; its stages are named at the Level C gate and written last, because what
two of them do is determined by the numbers ([README § How deep this plan is](../README.md#how-deep-this-plan-is)).
**Depends on:** everything before it. The **Level C gate** ([P2.7](../p2.7_failure_scaling_generalization/README.md))
for the claim's sentence; [P2.8](../p2.8_representations/README.md) and [P2.9](../p2.9_formal_account/README.md)
for Level D's *comparison* and *formal treatment*; and the frozen `test` split
([S2.3.4](../p2.3_benchmark/s2.3.4_splits.md)), which the loop meets here for the first time.
**Blocks:** [M5](../../m5_presentation/README.md): the paper, Stage P, is written from this phase's result and
artifact. The **Level D gate** closes with M5's paper and this phase's six other layers, and it is a stop
([Q-M2.15](../open_questions.md#q-m215--where-does-m2-stop-for-the-next-iteration-of-ein)).
**Feeds:** the next iteration of Ein's development, by [the evaluation of Ein](../ein_evaluation.md), which this
phase writes up as a deliverable of the milestone beside the research result.
**Research plan:** [`EinAf.md` § Stage O](../EinAf.md#stage-o--produce-the-central-research-result),
[§ Stage Q](../EinAf.md#stage-q--public-research-artifact). Stage P is M5's.

---

## Why this phase exists

The milestone has two main ideas ([README](../README.md#the-two-main-ideas-the-user-2026-11-25)) and so it has two
results. The first is the research plan's: one claim about the loop, tested once on instances no prompt was
developed against. The second is the user's: what this application found of Ein, its strengths, weaknesses, gaps,
bugs and limits, written so that the next iteration of Ein starts from it. Around both stands the artifact,
enough released material for someone else to reproduce or challenge the first result. And at its end the
milestone's plan is finished, and a finished plan is deleted: this phase is where everything the plan folder
still owns moves to its owner.

## The result (O)

The research plan names three admissible shapes and insists that the third is one:

> *Structured symbolic diagnostics improve iterative autoformalization compared with direct answering, one-shot
> formalization, generic retries, and self-refinement under comparable inference budgets.*
>
> *Unsat-core feedback substantially improves repair of over-constrained theories, while full provenance provides
> little additional benefit.*
>
> *Symbolic verification dramatically increases precision of accepted answers but does not improve overall task
> accuracy because semantic misformalization dominates.*

Positive, narrow, negative; *do not design the experiment around obtaining a predetermined positive conclusion.*
The phase's first act is to take the sentence [P2.7](../p2.7_failure_scaling_generalization/README.md) wrote at
the Level C gate, the claim as the validation split supports it, and run **exactly the conditions that sentence
names**, once, on `test`. The result is the sentence `test` supports: the same sentence or a weaker one, never a
stronger one and never another one found by looking. The two tables, the one that supports the claim and the one
that would have refuted it, are published side by side.

What the sentence has to carry on today's plan:

- **The arms it compares**, among B0 to B6 ([README § The arms](../README.md#the-arms)). Whatever its shape, the
  table beside it has B1's row, since the milestone's experiment is a chain of formal reasoning against a chain
  of thought, and both arrangements, the filter and the tool.
- **The reading of *certified* it is made under.** If the user has decided
  [Q-M2.10](../open_questions.md#q-m210--what-does-ein-certifies-a-unique-model-mean-on-r) by then, that one;
  if not, both conditional probabilities are reported under each reading and the sentence names the one it
  speaks of. A flagged program is certified under none and has a column of its own
  ([Q-M2.11](../open_questions.md#q-m211--what-is-a-flagged-program-to-the-loop)).
- **The kernel it was measured on**: Ein's commit and the library's manifest, as every record has them.

**The milestone is built so that the third shape can be seen.** The gap between `P(correct | certified)` and
`P(faithful | certified)` is where the plan says the interesting problem is: faithfulness is judged on every
repair ([S2.4.4](../p2.4_loop/s2.4.4_faithfulness.md)), the benchmark has instances whose right answer is
*ambiguous*, and a chain of thought runs at a matched budget. If the negative result is what the table says, it is the result, and the plan's sentence, *that is still
research*, is the reading.

**What `test` has been opened for.** The claim stands only if nothing after the benchmark was frozen was developed
against `test`. [S2.3.4](../p2.3_benchmark/s2.3.4_splits.md) says which openings are sanctioned; each is a record
with its reason, and this phase's is the one the claim is made on. Its acceptance is that the records that touched
`test` are the registered ones and no others, and that no failure on `test` was read before this run.

## The evaluation of Ein, written up

[The evaluation](../ein_evaluation.md) is kept from the first stage on, an entry a finding, in the user's five
kinds, and shown at every gate. This phase makes of it a document a reader outside the plan can use:

- **every entry with its evidence in a record**: the program, the command or the call, what was expected and what
  came back, and what it cost the application. An entry seeded from the analysis before any stage ran is either
  confirmed by a record of the application at work or said to be an audit's finding still;
- **the strengths as carefully as the rest**: where the kernel's reply was what a repair turned on, where an
  outcome the kernel said for itself saved the loop a guess, where a target other than Ein could not say what Ein
  said ([P2.8](../p2.8_representations/README.md));
- **the limits with their curves**: Stage J's solver curve is the measured part
  ([P2.7](../p2.7_failure_scaling_generalization/README.md));
- **each open entry with an owner**, so that nothing is left on a page that is about to be deleted. A bug that is
  a wrong answer is already a row of [the register](../../wrong_answers/README.md).

By default the write-up is a page of `docs/einaf/`, since its evidence is EinAf's records and EinAf may name Ein;
the entries still open move, at the close, to the followups or to the plan of the iteration of Ein the user
starts, which the user says at the gate.

## The artifact (Q)

Seven layers, each usable without the others.

| layer | what someone can do | where it is | what this phase checks |
|---|---|---|---|
| 1 executable system | run Ein, and run the loop | Ein: the binary, by the two channels of [`01-install.md`](../../../docs/ein/user/01-install.md); a release workflow builds it on a tag `v*`, and no such tag exists. EinAf: the package `einaf/`, built with `cargo` from a checkout, since it links the engine's crates by path | that both build from a fresh clone by the documented steps. Whether a release tag is cut for the artifact is the user's; by default the artifact names a commit |
| 2 benchmark | read the tasks, the canonical programs, the gold classes and answers | `einaf/benchmark/`, versioned, the splits frozen | that every canonical program's `:expect` holds on the artifact's kernel, and that the `test` split's digest is published with the result |
| 3 experimental harness | run the evaluation again | the `einaf` command ([S2.5.1](../p2.5_harness/s2.5.1_experiment_record.md)) | that the main table is regenerated from the records by one documented command |
| 4 raw results | read the generations and the kernel's replies, not only the charts | `einaf/experiments/`, immutable, one record a run | the size. If the whole set is too large to keep in the repository, an index with digests is kept and the user says where the set itself is published |
| 5 technical documentation | read the semantics and the architecture | Ein's: [`docs/ein/`](../../../docs/ein/README.md) and `docs/decisions/`. EinAf's: `docs/einaf/`, with the formal account ([P2.9](../p2.9_formal_account/README.md)) | that `docs/einaf/` states the system as built: the loop's architecture in both arrangements, the record's format, how to reproduce. The page of the architecture is this phase's to write if no phase before it did |
| 6 research paper | questions, method, experiments, conclusions | [M5](../../m5_presentation/README.md) | not this milestone's |
| 7 minimal demonstration | see the idea in one command | **this phase** | below |

**The model's file is not in the repository.** A record names it by its SHA and a run is given its path; the
artifact says where the file was taken from and what it needs to run (a llama.cpp the binding can load it with,
and a device).

**Ein has no binding for another language** and the artifact asks for none
([Q-M1a.23](../../../docs/decisions/engine/Q-M1a.23-ein-has-no-binding-for-another-language.md)): EinAf is Rust
and links the crates, and a reader who reproduces a table runs `cargo` and the `einaf` command.

### The demonstration, and the other thing called a demo

Two things carry the word, and this plan keeps them apart.

- **The demonstration** is Stage Q's seventh layer and this phase's: **one command of `einaf`** that shows the
  loop on one instance, understandable without the paper.
- **[`docs/demo/`](../../../docs/demo/README.md)** is the folder of the article and the talk *From Zebra puzzle to
  Autoformalization*, five beats, the fifth of which is this milestone's subject. Nothing of it is written, and
  it is [M5](../../m5_presentation/README.md)'s work, not this phase's. The demonstration's transcript is what the
  talk can show.

The demonstration is the research plan's sequence on one instance, chosen from the benchmark for being short and
for failing one shot in an instructive way:

```text
English problem → incorrect formalization → Ein detects ambiguity
→ symbolic diagnostic → LLM repairs the theory → unique model → verified answer
```

In today's words it prints: the text; the first program; the kernel's reply, the outcome *several*, which is
`Ambiguity` with two states that differ, **rendered at the feedback level that fixed it**; the repaired program
with its difference from the first; the outcome *one*, with the question that was asked of R named beside the
word; and **the short answer Ein prints**, the query's `:goal-text` filled, with EinAf's formatters over its
values ([Q-M2.12](../open_questions.md#q-m212--what-prints-the-answer)). It prints no count of models: a count is
a description's row, and the word is `--single`'s.

It is the filter's sequence. Whether the same instance is also shown as an episode of the tool
([S2.4.6](../p2.4_loop/s2.4.6_ein_as_a_tool.md)) is the stage's, by what Level C found of B6.

**A transcript that matches, and what can be made to match.** The kernel's part of a run is a function of the
program; the model's part is reproducible only on the same file, binding, sampler, seed and device. So by default
the command has two modes. **Replay** reads the record of the demonstration's run and judges every program in it
again: it needs no model, its output is the checked-in transcript byte for byte, and it can run in EinAf's step
of the gate. **Live** generates with the model's file and may differ in its tokens on another machine; its
record says with what it ran.

## Closing the milestone

**A finished plan is deleted** ([`AGENTS.md` § Where the work is](../../../AGENTS.md#where-the-work-is)): the
folder goes once what still reads it is re-aimed and what it still owns has moved to its owner. There is no
`docs/history/`, no closing record and no dated status. What the folder owns at the close, and where each thing
is by then:

| what | where |
|---|---|
| the records, the benchmark, the prompts and grammars | `einaf/experiments/`, `einaf/benchmark/`, `einaf/prompts/`, `einaf/grammars/`: never in the plan |
| what EinAf is and how it was measured: the architecture, the formal account, the outcomes, the record's format, the result with its tables | `docs/einaf/` |
| a ruling about EinAf, kept in [the ledger](../open_questions.md) while EinAf was planned | a page of `docs/einaf/`, once the thing it decides is built |
| a decision about Ein made in this milestone | a file under `docs/decisions/`, under its id, written with the change that made it true |
| the evaluation of Ein | its write-up in `docs/einaf/`; each entry still open in a followup or in the plan of the iteration of Ein |
| a wrong answer of the kernel | a row of [the register](../../wrong_answers/README.md), from the day it was found |
| a ruling about how the work is done that still binds | the root ledger, [`plans/open_questions.md`](../../open_questions.md) |
| a question still open | the root ledger where it spans milestones, else the followups |
| the research plan [`EinAf.md`](../EinAf.md), the user's note | by default it moves, unedited, to `docs/einaf/`, as the plan EinAf's pages cite for its stages and labels; the user says otherwise at the gate |
| [`analysis/`](../analysis/README.md), the stage files, this README | deleted with the folder; they stay in `git log` |

What reads the folder is re-aimed in the same commit: `AGENTS.md`'s pointer, `plans/README.md`'s row and tree,
and every plan and page that links a file of it, among them M5, which takes Stage P's outline from `EinAf.md`.

**Level D is reached with the paper.** This phase closes M2 with six layers and the demonstration; the seventh
layer is M5's.

## Stages, in shape

Named at the Level C gate.

**S2.10.1 — The `test` run.** The claim's conditions, once; the sentence `test` supports; the supporting table
and the refuting one; the list of records that touched `test`.

**S2.10.2 — The evaluation of Ein, written up.** Every entry with its record, the strengths among them; each open
entry with an owner; shown at the gate.

**S2.10.3 — The release.** The seven layers checked as a list, each with its path; the index of the records; the
architecture page; a build of Ein and of EinAf from a fresh clone.

**S2.10.4 — The demonstration.** The instance chosen, the command, its record and transcript, replay in EinAf's
step of the gate, and the paragraph that explains it.

**S2.10.5 — The close.** The table of § Closing the milestone carried out: what the folder owns moved, what reads
it re-aimed, the folder deleted.

## Acceptance

- The claim on `test`, in one of the three shapes, with both tables, B1's row, the reading of *certified* it is
  made under, and Ein's commit.
- The records that touched `test` are the registered ones and no others.
- The evaluation of Ein is written up, every entry with its evidence, every open entry with an owner, and shown.
- Six layers, each usable without the others, each with a path; Ein and EinAf build from a fresh clone by the
  documented steps.
- The demonstration runs from a fresh clone; its replay matches the checked-in transcript byte for byte, and its
  live mode states its prerequisites.
- The Level D gate's stop: the user has the result, the artifact and the evaluation in front of them, and says
  what comes next.
- The close: nothing the folder owned is left in it, nothing links it, and it is deleted.

## Risks

- **Looking at `test`.** One run, of conditions named beforehand. A second run for any reason is a new record
  with its reason, and the result says that there were two.
- **An evaluation that reads as a list of defects.** The user asked for strengths as well as weaknesses; an entry
  of either kind without a record is an opinion.
- **A live demonstration that does not repeat.** Hence replay; the live mode promises the sequence, not the
  tokens.
- **The size of the raw results.** A description of one ambiguous instance can be megabytes
  ([analysis § 3](../analysis/README.md#p25-and-p26--records-baselines-metrics-ablations-audit)); what is kept in
  the repository is decided on the measured size, with digests for the rest.
- **Deleting the plan before what it owns has moved.** The close is a stage with a table, and its acceptance is
  that nothing links the folder.
- **Where the article lives is not this phase's to settle, and it bears on it.** `docs/demo/` is a folder of
  Ein's documentation, and its fifth beat is EinAf's result; nothing of Ein names EinAf. M5's re-plan has to say
  where a text that reports both lives.

## Connections

- [`EinAf.md` § Stage O](../EinAf.md#stage-o--produce-the-central-research-result),
  [§ Stage Q](../EinAf.md#stage-q--public-research-artifact), [§ Level progression](../EinAf.md#level-progression).
- [The evaluation of Ein](../ein_evaluation.md); [the wrong-answer register](../../wrong_answers/README.md); the
  followups, [`plans/followups/`](../../followups/README.md), where an entry with no other owner goes.
- [M5](../../m5_presentation/README.md), the paper: § 9.3 of [Stage P's outline](../EinAf.md#stage-p--research-paper),
  *Faithfulness vs satisfiability*, is this phase's result read twice. Its own README is older than the reshape
  below its header and is re-planned by M5.
- [`docs/demo/`](../../../docs/demo/README.md), the article and the talk.
- [`analysis/audit_p2.7_p2.10_neighbours.md`](../analysis/audit_p2.7_p2.10_neighbours.md) § A.6 and § B.4: the
  old text against the tree.
- [M1p](../../m1p_ein_to_natural_language/README.md), which takes EinAf's formatters as its starting point;
  [M15](../../m15_mcp_server/README.md), another way a model would call Ein, which the artifact does not ship.

## Defaults taken at the re-plan

Each is the re-plan's, recorded so that the user can correct it.

1. **The write-up of the evaluation is a page of `docs/einaf/`**; open entries go to the followups or to the plan
   of the iteration of Ein, as the user says at the gate.
2. **`EinAf.md` moves, unedited, to `docs/einaf/`** at the close.
3. **The artifact names a commit**; a release tag is the user's to cut.
4. **EinAf is released as source**, built with `cargo` from a checkout; no binary of it is built by a workflow.
5. **The demonstration has a replay mode**, which needs no model and is held byte for byte, and a live mode,
   which is not.
6. **The demonstration shows the filter**; a tool episode beside it is the stage's.
7. **The claim's table always has B1's row and both arrangements.**
8. **If the raw results are too large for the repository, an index with digests is kept there**, and where the
   set is published is the user's.
9. **The phase has five stages in shape**, two more than before: the evaluation's write-up and the close.

Dropped as moot, each by [audit § A.6](../analysis/audit_p2.7_p2.10_neighbours.md#a6-p210): the verdict printed
*with `k` and `exhausted`* and the answer *through `:why` templates* (43); `docs/history/` as where a closed
milestone's records go (44); *M2 done* as a status with a date (45); the loop's channel and the harness's names
*in Q-M2.7's language*, which is ruled; and the phase's estimate.
