# P2.1 — The kernel as instrumentation (Stage A)

**Status:** **started 2026-11-25** on the user's word, *"start implementation of M2, everything in p2.1, then stop and let me review; each stage in subagents, subagents commit their work"*. The phase runs its three stages to
the end and stops for the user's review; no phase after it starts until the user says so.
**Depends on:** the kernel as it stands after M1f's close
([Q-M2.1](../open_questions.md#q-m21--when-is-the-kernel-frozen)), and nothing else.
**Blocks:** [P2.2](../p2.2_formalizer/README.md): its code goes into the package this phase creates, and the
formalization contract names the outcomes and the diagnostics a program gets back;
[P2.4](../p2.4_loop/README.md): the feedback ladder renders the object this phase defines, and nothing else.
**Research plan:** [`EinAf.md` § Stage A](../EinAf.md#stage-a--establish-the-symbolic-kernel-as-the-experimental-foundation),
A1 to A5.
**The first phase that changes Ein**: [S2.1.2](s2.1.2_diagnostics_surface.md), by the user's rulings on
[Q-M2.9](../open_questions.md#q-m29--does-the-kernel-grow-a-machine-surface-for-diagnostics) and
[Q-M2.16](../open_questions.md#q-m216--what-of-the-kernel-may-change-inside-m2) (a feature or a bug fix that leaves Ein's syntax and semantics as they are), under Ein's own rules.

---

## Why this phase exists

Stage A asks that *the existing engine become the trusted symbolic component of the experiment*: a semantic
boundary, invariants held by tests, a machine interface, a diagnostic vocabulary apart from presentation, and
validation suites. The plan of August expected to build most of an interface. Five kernel milestones later the
engine says for itself nearly all of it, and the phase is three things:

1. **EinAf has to exist.** A package under `einaf/` that links the engine's crates, a step of the gate that builds
   and tests it, the process told where it is, and its first page under `docs/einaf/`
   ([Q-M2.7](../open_questions.md#q-m27--what-language-is-the-frontend-written-in),
   [Q-M2.8](../open_questions.md#q-m28--where-does-einaf-live)). With it, the first thing EinAf knows about the
   kernel: which of [the seven outcomes](../README.md#the-outcomes) a run is.
2. **The kernel has to say a diagnostic to a program, not to a terminal.** It is the one thing of Stage A still
   missing: a refusal, an abort and a run's notes are text on stderr, and four of the library's load errors are
   strings. That is a change to Ein, ruled, and made in Ein's terms.
3. **EinAf's feedback object**: one value that holds everything the kernel found of one program under one
   question, which every level of the feedback ladder renders and beside which nothing else reads the kernel
   ([A3](../EinAf.md#a3-establish-a-stable-machine-interface): *human-readable CLI output should not be used by
   the experimental system*).

### What Stage A asks for, against what ships

[`analysis/audit_p2.1.md`](../analysis/audit_p2.1.md) has each cell with its evidence: § 0 is the fact base, what
the machine surface says in each of A1's situations, and § B what each of the plan's deliverables became.

| Stage A item | what exists | what is left, and whose |
|---|---|---|
| **A1** the semantic boundary | the answer as a region R and what each question of it means ([`06-solution-semantics.md`](../../../docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md)); the commands' contract and exit codes ([`05-cli.md`](../../../docs/ein/user/04-reference/05-cli.md)); every row and every key of the read-out ([`08-read-out.md`](../../../docs/ein/user/04-reference/08-read-out.md)). The kernel says five of A1's six outcomes by name or by a named field, and *resource limit* as *not decided*, which no stopped run confuses with a contradiction ([Q-M1d.6](../../../docs/decisions/answer/Q-M1d.6-contradiction-is-said-only-where-no-node-is.md)) | **one list keyed by the outcomes**, with the test of each, and the seventh, a flagged program: [S2.1.1](s2.1.1_tree_and_outcomes.md), in EinAf's documentation |
| **A2** invariants as tests | each of A2's six is held or decided otherwise: the oracle every mode is held to, the contract that every state printed is a member of R, every dumped state reloaded, provenance on every derived fact. **Three of the plan's sketches are false by decision**: a contradiction's core may be `[]`, a core is an explanation and not the smallest ([Q-M1f.6](../../../docs/decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)), and no kernel check holds a fact apart from its negation ([Q-M1f.557](../../../docs/decisions/stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md)) | the list, each invariant beside the test of Ein's that holds it: S2.1.1, in EinAf's documentation. Nothing is added to Ein's |
| **A3** a machine interface, versioned | for a caller of the library, the whole answer as values ([`04-api.md`](../../../docs/ein/ein.rs/04-api.md)); for a caller of the binary, `ein-summary/2`, `ein-answer/3`, `ein-test-report/6`, `ein-events/1` | **structured errors**: a refusal and an abort write no summary, and the report of `ein test` says `error` with no message. [S2.1.2](s2.1.2_diagnostics_surface.md) |
| **A4** a diagnostic vocabulary apart from presentation | the parse error is typed and placed, with its notes and ranked repairs; the refusals are normative text, a section each of [`07-diagnostics.md`](../../../docs/ein/user/04-reference/07-diagnostics.md) § 2 | **kinds**: `KbLoadError`, `LoadError`, `MacroError` and `CompileError` are strings. S2.1.2. The plan's list of kinds was a guess, and two of its names (`type_error`, `arity_mismatch`) are of notions Ein does not have; the vocabulary is the refusals Ein has |
| **A5** validation suites | the corpus with its runs, the negative fixtures with their `.expected` texts, the programs that state their answer as `:expect`, the oracle, the regressions | nothing compares the engine with a system that is not Ein: [M10](../../m10_external_benchmarks/README.md)'s, named and not scheduled here |

## Stages

| ID | title | ends with |
|---|---|---|
| [S2.1.1](s2.1.1_tree_and_outcomes.md) | EinAf's tree, and the outcomes | `einaf/`, a Cargo package that links the engine's crates, built and tested by a step of the gate and of CI; the process told of it; `docs/einaf/` with its first pages; the seven outcomes as a function over the library's values, held over the corpus |
| [S2.1.2](s2.1.2_diagnostics_surface.md) | The kernel's diagnostics as values, and its bounds inside a saturation | in Ein: every refusal a value with a kind and, where the source has one, a place; a refusal, an abort and a run's notes on the machine surfaces; a core's sources beside the core; the clock and a limit reaching inside a saturation, so that no program runs without end; its decision under `docs/decisions/`, its pages and tests in the same commit. **Ran to its end 2026-11-25** ([its record](s2.1.2_diagnostics_surface.md#record)); waits for the user's review with the phase |
| [S2.1.3](s2.1.3_feedback_object.md) | The feedback object | EinAf's `einaf-feedback/1`: one value for one program under one question, built from the library's values by a call the kernel's own bounds end, serialised for the record; one for every corpus program, held to what the binary's summary says of the same run. **Ran to its end 2026-11-25** ([its record](s2.1.3_feedback_object.md#record)); waits for the user's review with the phase |

## Acceptance

- `einaf/` builds and its tests pass as a step of `./run_tests.sh` and of the CI workflow, and
  `utils/check_gate_steps.py` holds the two lists to each other. Nothing under `ein.rs/`, `docs/ein/`, `stdlib/`,
  `examples/`, `tests/` or `corpus/` names EinAf.
- Every corpus program has exactly one of the seven outcomes by EinAf's function, and the outcome agrees with what
  the corpus says of the program (its group, its exit, its `:expect`).
- In Ein: each `.ein` under `examples/broken/` that is refused yields a diagnostic with a **kind** and, where the
  source has one, a place; its message is the normative text, unchanged; the same diagnostic is on the machine
  surface the stage defines. A `-T` abort and a load's notes are there too. The gate is green on the change.
- A feedback object is produced for every corpus program, and what it says agrees with `ein-summary/2` of the same
  program under the same question.
- [The evaluation of Ein](../ein_evaluation.md) has what the phase found, and is shown at the close.

## What the phase does not do

- **It changes no exit code and no answer.** S2.1.2 makes what the kernel already knows reachable and ends a
  run that would not end; R is as it was, and the only option it may add is the limit on a saturation.
- **It gives no place to a message that has none today.** A loader message about a top-level form ends `at None`
  ([Q-M1a.6](../../../docs/decisions/diagnostics/Q-M1a.6-a-loader-message-about-a-top-level-form-names-no.md)),
  and that stays [F23](../../followups/f23_diagnostics_in_eins_words.md)'s.
- **It does not make `--single` write a core.** A question that prints no core takes none
  ([Q-M1f.29](../../../docs/decisions/read-out/Q-M1f.29-a-question-that-prints-no-core-writes-a-dead.md)); EinAf
  asks for a description where it wants one.
- **It decides nothing about the loop's question or the ladder's levels**: the object has room for what they need,
  and [P2.4](../p2.4_loop/README.md) fills it.

## Defaults taken at the re-plan

Each is the coordinator's and the user's to correct.

1. **The outcomes and the invariants are written in EinAf's documentation**, `docs/einaf/`, which may name Ein's
   pages and tests; nothing is written into `docs/ein/` for them. This settles what the phase left *to settle at
   its re-plan* when `defined_behaviour.md` went.
2. **The kernel's vocabulary of kinds is the refusals it has**, one kind a section of `07-diagnostics.md` § 1 and
   § 2, not the research plan's A4 list.
3. **The machine surface is the summary and the test report**, written in more cases and with one more key, and no
   new option: a design the stage confirms against Ein's rules before it writes code
   ([S2.1.2](s2.1.2_diagnostics_surface.md) § The design).
4. **EinAf banks properties of the object over the corpus, not goldens of its values**, as the engine's
   `summary_properties.rs` does of the summary and for its reason; and no value of a flagged program's answer is
   banked at all.
5. **The object is EinAf's and versioned by EinAf** (`einaf-feedback/1`); the plan's `ein-feedback/1`, written by
   `ein-cli`, is not made.
6. **The kernel bounds its own run, and EinAf calls it in its own process** (the user's ruling, [Q-M2.16](../open_questions.md#q-m216--what-of-the-kernel-may-change-inside-m2),
   which replaced this re-plan's first answer, a worker process that EinAf stops from outside). Nothing in a solve
   stops a saturation that has no fixpoint, a model will write such programs, and S2.1.2 gives the kernel the
   bound. A worker process is built only if a probe shows a case those bounds do not stop, or a measurement needs
   the kernel alone in a process.

## Risks

- **The diagnostics change looks small and touches normative text.** The messages are pinned by `.expected`
  fixtures and by `07-diagnostics.md`; the stage adds a kind *beside* a message and changes no message. If a kind
  cannot be given without changing one, the message wins and the case is recorded in the evaluation.
- **A summary written where none was written is a behaviour of the CLI that six scripts under `utils/` and the
  corpus sweeps read.** The key is additive and the cases are new ones; the stage runs the gate, and the sweeps
  that read `--json-summary` are its check.
- **EinAf's tests reading the corpus couple it to Ein's dev crates.** `ein-corpus` is a crate the engine's tests
  use; if depending on it from outside the workspace costs more than reading `corpus/corpus.toml`, EinAf reads the
  file.
- **Freezing what a later milestone will change.** The kernel is fixed a record at a time, by its commit
  ([Q-M2.1](../open_questions.md#q-m21--when-is-the-kernel-frozen)); the object carries the commit and the
  library's manifest so that the freeze is a field.

## Connections

- [`EinAf.md` § Stage A](../EinAf.md#stage-a--establish-the-symbolic-kernel-as-the-experimental-foundation).
- [`analysis/audit_p2.1.md`](../analysis/audit_p2.1.md): the fact base (§ 0), what each deliverable became (§ B),
  the standing rules the old plan met (§ C).
- [`04-api.md`](../../../docs/ein/ein.rs/04-api.md), the crates EinAf links, whose worked example is the test
  [`embedding.rs`](../../../ein.rs/crates/ein-cli/tests/embedding.rs);
  [`05-events.md`](../../../docs/ein/ein.rs/05-events.md), `ein-events/1`.
- [F23](../../followups/f23_diagnostics_in_eins_words.md), whose D-18 and L-125 are S2.1.2;
  [F22](../../followups/f22_exact_core_past_the_cap.md), the exact core;
  [F26](../../followups/f26_consistency_and_completeness.md), Ein's words beside logic's, kept for what a verdict
  certifies.
- [F16](../../followups/f16_autoformalization/ideas.md): the list of diagnostics *a reasoning engine can return
  that a compiler cannot*, where the ladder's levels come from.
