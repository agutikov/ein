# P2.1 — The kernel as instrumentation (Stage A)

**Estimate:** 1.5 weeks — 3 stages, 8 days.
**Depends on:** [M1a](../../../docs/history/m1a_rust/README.md), shipped. Not
on [M1d](../../m1d_satisfiability/README.md): the one word M2 needs from it
(`unknown`) is computed here from fields the engine already emits, and
replaced by the engine's own the day M1d gives it one
([Q-M2.1](../open_questions.md#q-m21--when-is-the-kernel-frozen)).
**Blocks:** [P2.2](../p2.2_formalizer/README.md) — the formalization contract
names the protocol its diagnostics arrive in; [P2.4](../p2.4_loop/README.md) —
the feedback ladder is nine renderers of the object this phase defines.
**Research plan:** [`EinAf.md` § Stage A](../EinAf.md#stage-a--establish-the-symbolic-kernel-as-the-experimental-foundation),
A1–A5.

---

## Why this phase exists

Stage A asks for five things, and the engine ships four of them. The plan's
premise is right — *the existing engine becomes the trusted symbolic component
of the experiment* — and the milestone's job here is not to build a kernel but
to stop treating the one it has as a prototype. The phase is therefore short,
mostly a census, and ends with **one new artefact**: a versioned, structured
object that says what the kernel found, in a vocabulary the loop consumes and
no human-readable table is allowed to stand in for
([A3](../EinAf.md#a3-establish-a-stable-machine-interface): *human-readable
CLI output should not be used by the experimental system*).

### What Stage A asks for, against what ships

Taken 2026-08-23 against `ein.rs` at the M1a close; [S2.1.1](s2.1.1_census.md)
re-takes it with `file:line` and turns every *missing* into a task or a
named owner.

| Stage A item | what it asks | what exists | what is missing |
|---|---|---|---|
| **A1** the semantic boundary | what an Ein program *means*: objects, types, relations, facts, rules, hypothesis rules, constraints, queries, saturation, model completion, contradiction, ambiguity, uniqueness, provenance, no-goods, search | [`docs/kernel/`](../../../docs/kernel/README.md) — the graph semantics, the data model, ein-lang, [`architecture_and_algorithms.md`](../../../docs/kernel/inference/architecture_and_algorithms.md) O1–O9, [`absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md), the [glossary](../../../docs/kernel/glossary.md); since M1a P1a.10 *the only statement of intent that is not also the implementation*, checked by the gate | the **six outcomes** stated as one list with their operational test each — five of the six exist (*invalid* = exit 1 with a diagnostic; *inconsistent* = `Contradiction` with a core or exhausted; *multiple models* = `k ≥ 2` — two distinct sorted fact lists prove it whatever `exhausted` says; *unique* = `k = 1 ∧ exhausted`; *satisfiable incomplete* is the **query** axis — `k ≥ 1` and no model binds the goal — until M1d gives the engine an obligation-level *incomplete*), and the sixth, **`unknown`**, does not: see the next table. [S2.1.1](s2.1.1_census.md) T2.1.1.3 writes the mapping out |
| **A2** kernel invariants as tests | every fact justified; every contradiction derived; every solution satisfies the constraints; *unique* = one model **and** exhausted; *ambiguous* = two distinct models; *unsat* = exhaustion with zero | the gate — 619 tests, the thirteen counter identities `summary_properties.rs` holds the summary to, the id-order and `--jobs` invariance sweeps, the six-property fuzzer; provenance on every derived fact (O6); `-e` as the certification of uniqueness | the invariants are *held* but not *listed*: a page that states each of the plan's six in one line and names the test that holds it — S2.1.1 T2.1.1.3 writes it into [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md), where the thirteen behaviours already live |
| **A3** a stable machine interface, versioned | structured input / output; structured errors; a protocol version | `--json-summary` is **`ein-summary/1`**: `verdict {type, k, exhausted, unsat_core, solutions[{facts, goal_bindings}]}` — the models sorted, the core as s-expressions — plus `stats`, `root`, `config`; `--events` is **`ein-events/1`**, the fifteen kinds [`events.md`](../../../docs/kernel/inference/events.md) tables plus `admit` and `warn` at the call sites, `fire` carrying `rule / bindings / premises / derived`; `ein --version` names the engine, the protocol, the features and the stdlib manifest SHA | a **feedback** object: the summary is a *run* record, additive and order-free, written for a parity harness; it has no diagnostics block at all (a parse or load failure never reaches it — the process exits 1 with a line of text), no `unknown`, and the exit code for a budget abort is **2, the usage-error code, by design** — so a loop reading exit codes cannot tell a typo from a timeout. [S2.1.2](s2.1.2_feedback_object.md) |
| **A4** a diagnostic vocabulary apart from presentation | `syntax_error … arity_mismatch / contradiction, unsat_core / ambiguous … alternative_models / unused_fact, unreachable_query, dead_relation / resource_limit, search_limit` | `ParseError` is a struct — `file, line, col, context` — and the only typed one; [`defined_behaviour.md` § 1](../../../docs/kernel/defined_behaviour.md) pins the parse and load *messages* as normative strings; the unsat core is a value (`verdict.unsat_core`); `warn` events carry a `category` | `KbLoadError`, `LoadError`, `MacroError`, `CompileError` are **`String` newtypes** — the accumulated problems `; `-joined, no kind, no location, no id; no `dead_relation` / `unreachable_query` / `unused_fact` analysis anywhere (S2.1.1 T2.1.1.2 checks whether the *matcher* already knows which rules never fired — `fire` events say which did). The vocabulary is S2.1.2's, as an enum with every field optional |
| **A5** validation suites | parser, types, saturation, constraints, search, provenance, unsat core, model count, determinism, performance regression; *deliberately pathological cases* | the corpus: 128 entries in six groups (`positive`, `stdlib`, `parse-negative`, `load-negative`, `compile-negative`, `regression`), the negative fixtures under `examples/broken/` with their `.expected` messages (37 today, in three groups), `ein-bugs/`, the fuzz findings; [`corpus_cost.md`](../../../docs/history/m1a_rust/measurements/corpus_cost.md) as the performance ledger; [design/02](../../../docs/history/m1a_rust/design/02_determinism_and_order.md) for determinism | **unsat-core correctness and model-count correctness have no external check** — every test compares the engine to its own past. That is [M1c](../../../docs/history/m1c_external_validation/README.md)'s thesis and [M10](../../m10_external_benchmarks/README.md)'s job, named here and **not scheduled here** |

### The sixth outcome, today

The plan's A1 list ends with *resource-limit / unknown* and the rule that it
*must never silently become `false`, `ambiguous`, or `contradiction`*. The
engine has two caps and they say different things:

| cap | what the engine reports | exit | what the loop would read |
|---|---|---:|---|
| `--max-time`, `--max-enterings` | `** aborted: <reason> **`; with the summary, `verdict.type = "Aborted"`, `reason`, empty core and models | **2** | *unknown* — correctly, if it reads the summary; *usage error*, if it reads the exit code |
| `--max-set-size` (default 5) with a non-empty frontier | **`Contradiction`**, `k = 0`, `exhausted = false`, **empty unsat core** | 0 | *inconsistent* — wrongly. This is [Q-M1d.6](../../m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false): ten corpus entries end this way, and `Contradiction` with an empty core is the tell |

The rendering makes the asymmetry visible: a truncated `Solution` prints
*(not certified — pass `--exhaustive`)* and a truncated `Contradiction`
prints nothing. **M2 does not fix this in the engine** — it is M1d's, and
the choice between *say `Aborted`*, *qualify the rendering* and *a fourth
word* is theirs to take with the obligations work. What M2 does is refuse to
read `verdict.type` alone: the feedback object's verdict is a function of
`(type, k, exhausted, unsat_core)`, and `Contradiction ∧ ¬exhausted ∧ core = ∅`
maps to **`unknown`** with `search_complete = false` — and so does
`k = 1 ∧ ¬exhausted`, the uncertified solution, because the plan's rule is
*unique means one model **and** an exhausted search*. When M1d lands a word,
the function changes and the field does not.

## Stages

| ID | title | est. | ends with |
|---|---|---:|---|
| [S2.1.1](s2.1.1_census.md) | The census — Stage A against the shipped engine | 2 d | the table above re-taken with `file:line`; each *missing* a task here or a named owner elsewhere; the six outcomes and the A2 invariants written into `defined_behaviour.md` with the test that holds each |
| [S2.1.2](s2.1.2_feedback_object.md) | The feedback object — `ein-feedback/1` | 4 d | the schema, versioned; the A4 vocabulary as an enum; the mapping from `ein-summary/1` + `ein-events/1` + the typed errors into it; `unknown` derived; the corpus's 128 entries each producing one, banked as goldens |
| [S2.1.3](s2.1.3_boundary.md) | The boundary — Q25 decided | 2 d | the language of the loop, with the protocol as the boundary either way; the directory `einaf/` and what is in it; the decision recorded beside [Q25](../open_questions.md#q25--what-language-is-the-frontend-written-in) |

## Acceptance

- `defined_behaviour.md` states the six outcomes and the six A2 invariants,
  each with its operational test and the gate test that holds it; the one
  outcome the engine cannot say is marked as M1d's with the derivation M2
  uses meanwhile.
- `ein-feedback/1` exists as a schema and a producer; for every corpus entry
  a feedback object is produced and banked, and `cargo test --workspace`
  diffs them the way it diffs the summary goldens.
- Every `.ein` under `examples/broken/` — the negative fixtures with their
  `.expected` messages — yields a
  feedback object with a diagnostic of a **kind** from the vocabulary and a
  location where the source has one, not only the pinned message.
- The loop's language is decided and written down, with the measurement or
  argument it rests on, and the three trip-wires of
  [Q-M1a.23](../../../docs/history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
  re-checked against the decision.
- **Nothing in `ein-infer`'s semantics changes.** The phase adds a
  consumer-side object — or, as [S2.1.3](s2.1.3_boundary.md) recommends, an
  additive `ein-cli` flag, `--feedback FILE.json`, beside `--json-summary` —
  and, at most, *kinds* on errors that already exist.

## Risks

- **Over-specifying the vocabulary before the ladder says what matters.**
  A4's list is the plan's guess; [P2.6](../p2.6_ablations/README.md) is the
  experiment. So the object's fields are all optional, the enum is open
  (`other(String)` with the pinned message), and a renderer that wants a kind
  the engine does not supply writes `unknown_kind` rather than parsing prose.
- **Freezing what two milestones want to change.**
  [Q-M2.1](../open_questions.md#q-m21--when-is-the-kernel-frozen) — the kernel
  is frozen per experiment by commit, not for the milestone; this phase makes
  the record carry the commit and the manifest SHA so the freeze is a field.
- **Typing the load and compile errors looks like a kernel change.** It is
  not a semantic one: the message strings stay pinned by
  `defined_behaviour.md` § 1 and § 4, and a `kind` *beside* a message breaks
  no golden. If a kind cannot be assigned without changing a message, the
  message wins and the kind is `other`.

## Connections

- [`EinAf.md` § Stage A](../EinAf.md#stage-a--establish-the-symbolic-kernel-as-the-experimental-foundation).
- [`docs/kernel/defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md) —
  the thirteen behaviours; the six outcomes and six invariants join them.
- [`docs/kernel/inference/events.md`](../../../docs/kernel/inference/events.md) —
  `ein-events/1`; [`ein-cli/src/summary.rs`](../../../ein.rs/crates/ein-cli/src/summary.rs) —
  `ein-summary/1`; [`docs/api/rust.md`](../../../docs/api/rust.md) — the crate
  surface a Rust loop would link.
- [M1d Q-M1d.6](../../m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false),
  [Q-M1d.1](../../m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted) —
  the verdict vocabulary's owner; [M1c](../../../docs/history/m1c_external_validation/README.md) /
  [M10](../../m10_external_benchmarks/README.md) — A5's external check.
- [F16](../../followups/f16_autoformalization/ideas.md) — the list of
  diagnostics *a reasoning engine can return that a compiler cannot*, which is
  where A4's vocabulary and the ladder's F3–F7 come from.
