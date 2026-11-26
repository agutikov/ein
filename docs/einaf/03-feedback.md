# The feedback object

**One value holds what the kernel found of one program under one question**: `einaf::kernel::Feedback`, written
as JSON under the schema `einaf-feedback/1`. It is what every reader of the kernel in EinAf reads: a level of the
feedback ladder renders a part of it, the tool returns one of those renderings to the model, and the experiment
record stores it, so that a table is recomputed and a level rendered again without running Ein again. Nothing in
EinAf reads the kernel another way.

It is built by [`einaf/src/kernel/feedback.rs`](../../einaf/src/kernel/feedback.rs) from the library's values, the
load's result and the read-out of the run, and from nothing the kernel printed. `einaf feedback FILE [QUESTION]
[BOUNDS]` prints it.

## 1. The bounded call

`einaf::kernel::call(source, questions, bounds)` is the one door to the kernel's answers: a program, a file or a
text, and the questions asked of it go in, and one object a question comes out, in their order, whatever the program
is. (`einaf::kernel::forms` reads a program's forms and asks it nothing, so it saturates nothing and needs no bound;
[the contract](04-contract.md#4-what-each-form-is-the-categories) reads it.)

- **The program is loaded once**, as `ein solve` loads it: parsed and loaded against its file's directory, then
  judged, `ein_infer::mechanisms::judge_loaded`, which refuses what the binary refuses. Each question is asked of a
  copy of the load.
- **Every saturation of the call is bounded by the kernel**, the judgement's among them: the clock (`-T`) and the
  cap on one saturation's firings (`--max-steps`, $2^{20}$ by default) are read inside a saturation
  ([Q-M2.16](../decisions/answer/Q-M2.16-every-saturation-a-question-about-r-computes.md)), and the facts budget and
  the cube's limit by the routes. The clock runs from the call's start for the load's judgement and from each
  question's start for the question. A judgement a bound stops judges nothing, and the question reports the stop. A
  run a bound stopped is an object whose outcome is *not decided*, with the diagnostic that says which bound.
- **A question that needs the cube is asked under a limit of 0.** On a program the static check flags, a
  description, `--count`, a line and `--maximal --single` read every node of the cube up to `--region-limit`, an
  answer of which nothing is claimed. The limit is read by a flagged program alone, so on every other program the run
  is the one asked; on a flagged one it reads the static check at root and stops at once, and the object holds the
  flag, the sites and the warning, and no answer. `--single` over all of R or the minimal line, which stop when
  decided, is asked under the limit asked, and its answer is kept.
- **A panic of the kernel's is caught there**, and so is an error of the engine's that names no kind: either is the
  object's `fault`, with no outcome, and the call's other questions are still asked.
- **What the bounds do not stop** is listed in [§ 4](#4-what-the-bounds-do-not-stop).

## 2. The keys

**Every key on every object, and `null` where it does not apply**: the rule of the engine's summary
([Q-M1e2.27](../decisions/read-out/Q-M1e2.27-in-eins-json-is-the-empty-set-and-null-what-does.md)). `[]` is a list
known to be empty, `null` a part that does not apply to the question or the outcome, or that the run did not
establish. A renderer reads a part by its key and renders nothing for `null`. The parts are in this order.

| key | what | `null` where |
|---|---|---|
| `schema` | `einaf-feedback/1`; the number moves when a part changes its meaning | never |
| `program.name` | the file as given, or a text's name: what the messages name it by | never |
| `program.digest` | `sha256:` and the hex digest of what `program.of` names | nothing was read |
| `program.of` | `canonical-dump`, the program's forms as `ein_ir::dump_canonical` prints them, which normalises layout and comments; `text`, the bytes of a text that did not parse; `bytes`, a file not read as text | nothing was read |
| `kernel.version` | Ein's version, as `ein --version`'s first line has it: the engine's workspace version | never |
| `kernel.protocol` | the `--events` protocol, `ein-events/1` | never |
| `kernel.stdlib` | `sha256:` of the standard library's manifest as this process resolved it, as `ein --version` prints it | the manifest could not be read |
| `kernel.commit` | the commit of Ein, which the record sets from git | always, as the call writes it |
| `asked.question` | the question as `ein solve`'s flags, `describe` for the bare run | never |
| `asked.which` | `all`, `minimal` or `maximal` | never |
| `asked.ask` | `describe`, `single`, `count` or `no-search` | never |
| `asked.budget`, `asked.limit` | `--region-budget` in facts, `--region-limit` in candidates | never |
| `asked.max_steps` | `--max-steps` in firings | no cap |
| `asked.clock_us` | `-T` in microseconds | no clock |
| `outcome[]` | one entry a reading the question gives ([the outcomes § 3](01-outcomes.md#3-the-questions-and-the-readings)): `part` (`all`, `minimal`, `maximal`), `outcome` (`invalid`, `flagged`, `not-decided`, `inconsistent`, `incomplete`, `several`, `one`) and `why`, a line saying what it was read from | the kernel faulted |
| `readings.word` | `--single`'s word, `{part, word}`: `Solution`, `Ambiguity`, `Contradiction` or `Open` | another question, or the run stopped first |
| `readings.k_s`, `readings.k_c` | a description's or `--count`'s counts of R's states and commitments | another question |
| `readings.minimal`, `readings.maximal` | the states of each line: a description's and `--count`'s both, a line's its own | another question |
| `readings.exhausted` | the part asked was listed whole | — |
| `readings.stopped` | what stopped the run: `{why, saturation}`, `why` one of `budget`, `limit`, `steps`, `clock` and `asked` (`--no-search`), `saturation` `root` or `node` under `steps` | nothing stopped it |
| `readings` | the whole part | no answer came back: a refusal, the clock's abort, a flagged program's cube question |
| `diagnostics[]` | what the kernel said, in the order `ein solve` prints it on stderr: a refusal's findings, the clock's abort, the warning of a program found not monotone, a partial run's note, each claim not checked or refuted, and the load's notes last; each `{kind, severity, message, place}` as the summary's [§ 11.3](../ein/user/04-reference/08-read-out.md#113-diagnostics-what-the-run-said-as-values) writes it | never; `[]` where it said nothing |
| `flagged.sites[]` | the static check's sites, `{rule, activator, kind, relation, depth, concludes, at_stake}`, as the summary's `monotonicity.sites` | the program is monotone |
| `flagged.in_flight` | the hypothesis a state above root held that root does not, where the run found it so | not found in flight |
| `queries[]` | each `(query …)` with a goal: `goal` as written; `goal_text`; `over`, the part its `:over` names or `listed`; `exact`; `agree`, exactly one class; `classes[]`, each `{facts, rows, lines, states}`, `rows` a map from each goal variable to its value, `lines` each row filled into the `:goal-text`, `states` 1-based into `states`; `expect`, the query's `:expect`, `{classes, outcome, lines}`, `classes` how many it lists | no answer came back |
| `claim` | the `(solve … :expect …)`: `shape` (`false`, `region`, `owing`, `unread`), `states` the states it lists, `outcome` (`held`, `failed`, `not-checked`) and `lines` | the program states none, or no answer came back |
| `states[]` | the states the run reached, as the summary's `region.states`: `facts`, a state's facts over the candidates' relations, root's included, sorted; `digest`, `sha256:` of those facts one a line; `of`, `model`, what the digest is of; `lines`, `minimal` and `maximal`; `listed`, printed by `ein solve` | `--count`, which reads no state; a run the cap stopped; no answer |
| `owed.root` | each instance root owes, through its rule's `:why` | another question than a description |
| `owed.frontier[]` | an empty R's owing frontier, `{facts, owed}` | R is not empty of the owing kind, or the route did not compute it |
| `core` | a dead root's core, under a description: `facts`, `[]` where the contradiction rests on no fact; `sources`, each fact's `:source` or `null`; `status`, `within-budget` or `cut-` and its caps; `refuted_by` | root is not dead, or the question takes no core |
| `work.saturations`, `work.facts`, `work.layers` | the run's work, as the summary's `work` | no answer came back |
| `work.run_us` | the run's wall clock as the library measured it, in microseconds | no answer came back |
| `work.load_us`, `work.ask_us` | the load with its judgement, shared by the call's questions, and this question asked and read, measured by EinAf | never |
| `dependencies`, `provenance` | **left open**: the ladder's F6 and F7, which the ladder's stage adds | always, until then |
| `fault` | `{kind, message}`, `kind` `panic` or `engine`: the kernel failed at the call | the kernel did not fail |

**One object, one question.** A loop that asks `--single` to decide and then a description for a core has two
objects, each saying what it was asked. **A state's digest is of its facts over the candidates' relations**, which is
what the library gives of a state: two states that differ only in a fact of another relation have one digest.

The object holds what a renderer or a metric reads, not the whole answer: `states` is bounded by the question asked,
and a record that wants the whole answer stores `ein solve --dump-answer`'s directory beside the object.

## 3. Held by

[`einaf/tests/corpus.rs`](../../einaf/tests/corpus.rs), over every corpus program that is not `slow`:
`every_object_survives_a_round_trip_through_json`, every question's object read back as itself; and
`every_object_says_what_the_binary_says_of_the_same_run`, under each question the program's entry declares, the
object against `ein solve --json-summary` of the same run, from the binary the gate built (`$EIN_BIN`, by default
`ein.rs/target/gate/ein`): the outcome against the summary's verdict, `region.empty`, `region.exhausted` and
`monotonicity`; the readings, the states, the queries' classes and claims, the program's claim, what is owed, the
core and the work against their keys; and the diagnostics against the summary's, one for one, the paths as the
binary names them. Of a flagged program, only that it is flagged, and its sites. A missing binary fails the test with
the command that builds it.

[`einaf/tests/refused.rs`](../../einaf/tests/refused.rs): every refused fixture under `examples/broken/` is an object
whose outcome is *invalid* under every question, whose diagnostics are its findings, each an `error` of a kind
[`07-diagnostics.md`](../ein/user/04-reference/07-diagnostics.md#2-errors-and-exit-codes) names, holding the line its
`.expected` banks. [`einaf/tests/outcomes.rs`](../../einaf/tests/outcomes.rs)'s
`a_saturation_with_no_fixpoint_is_not_decided_at_the_kernels_bound`: a program whose root has no fixpoint, and one
whose load's judgement saturates such a root, are *not decided* under every question at the cap and at the clock.

## 4. What the bounds do not stop

The clock is read every $2^{10}$ firings and at each quiescence of a saturation, and the cap counts firings. Work that
fires nothing between two reads of the clock, or is no saturation, is under neither, and the call returns after it,
past its clock:

| program | what runs past the clock |
|---|---|
| a join that never fires: a cycle of five edges asked of a large complete bipartite graph, which has none | the matcher, between two firings |
| a relation of two places that nothing derives, over a few hundred names, and no hypothesis mechanism | the blind enumerator, building H, a candidate for every pair of names |
| a term nested a few thousand deep by a rule over a chain, which a query binds | the read-out, rendering the state's facts and the query's rows, in time that grows faster than the square of the depth; the clock is not read there, so a run the clock would have stopped is answered |
| a root that dies on a link several rules derive, its alternative justifications recorded (no `(config :record-alternative-justifications false)`): a model's Zebra puzzle that names eleven houses for five places and kinds with no member | root's saturation, past its clock: `ein solve` of it under `--single -T 5` returns after 72 s, and in 63 ms with the line; the bounded call's clock, every question's and the load's, returns as late |

None is a program the call does not return from: it returns late. The memory of a call is not bounded by the kernel;
the cap on firings bounds a runaway root's.
