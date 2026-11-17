# Algorithms

Which algorithm `ein.rs` uses for each operation the engine performs, where that places it among the known ones, and
the levers that change how it computes and never what. The operations themselves, and the paradigms they come from
(forward chaining to a fixpoint, negation as failure, a search over assumptions), are defined independently of this
code; this page is the choices `ein.rs` makes for them.

## 1. The operations, and the choice for each

| operation | what `ein.rs` does | the family | where |
|---|---|---|---|
| **join**: every match of a rule's premises | a register machine over a fixed join order, with candidates narrowed by an index on argument positions (one level inside a nested argument) and a fully bound premise answered by one interned lookup | nested-loop join over alpha memories, in the RETE family, with no beta memories; not a worst-case-optimal join, whose reordering would change the firing order | [`03-design/05-matcher.md`](03-design/05-matcher.md) |
| **saturation**: the fixpoint | forward chaining, semi-naive: after a firing, only the delta is matched, each plan seeded at the premise a new fact plays; plans compiled once per run | the Datalog ladder: naive, then semi-naive with a seeded delta join | [`03-design/06-saturation.md`](03-design/06-saturation.md) |
| **negation as failure** | the boundary: a guard is judged only on a quiescent KB, one admission a round, a parked candidate judged again only where a relation it watches grew | stratification computed by the run rather than in advance; a refutation resting on an `absent` a commitment could extend is refused | [`03-design/06-saturation.md`](03-design/06-saturation.md), [`03-design/07-search.md`](03-design/07-search.md) |
| **equality** | none: `eq` is name identity, computed, never stored, and names are never merged ([Q1](../../decisions/language/Q1-the-knowledge-base-is-a-typed-labelled.md)) | no congruence closure | [`03-design/03-data-model.md`](03-design/03-data-model.md) |
| **contradiction** | two bit tests per new fact, `(false)` and a pair `X`, `(not X)`; the agenda fires first what can make one, and a route's node stops at its first | incremental clash detection | [`03-design/06-saturation.md`](03-design/06-saturation.md) |
| **provenance and cores** | every derivation recorded as an OR over AND-nodes, alternatives kept by premise count and smallest closure; a core is the smallest frontier, by an ATMS label search with a budget | ATMS labels; a minimum-cardinality frontier rather than a subset-minimal unsatisfiable subset | [`03-design/03-data-model.md`](03-design/03-data-model.md), [`03-design/07-search.md`](03-design/07-search.md) |
| **hypotheses**: the candidates H | a ladder: the program's hrules, else what the obligations owe, else blind enumeration over the program's names by type | rule-driven, obligation-driven and generate-and-test generation | [`03-design/07-search.md`](03-design/07-search.md) |
| **the search**: the region `R` | routes over the lattice of commitments, each the least work its question needs: the state route (each alive state once), the obligation tree, the climb and the shrink, the alive down-set, the cube | monotone, upward-closed pruning (a dead node's supersets are dead), minimal transversals for the dead sets and the minimal generators, branching on an owed instance's exhaustive alternatives | [`03-design/07-search.md`](03-design/07-search.md) |
| **canonicalisation** | a state is its sorted fact set, compared whole, never by hash | canonical forms rather than isomorphism | [`03-design/07-search.md`](03-design/07-search.md) |

No problem is handed to an external solver: every operation above is the engine's own
([Q2](../../decisions/engine/Q2-ein-hands-no-problem-to-an-external-solver.md)). Arithmetic is a theory in the
standard library, computed by these same operations.

## 2. What it is shaped against

The engine is dominated by the join inside saturation; the choices above are each a way of asking the join less. The
alpha index reaching inside a nested argument, the ground-premise lookup, the plan memo shared by every node, a node
resumed from root's snapshot rather than saturated from scratch, a dead node stopped at its first contradiction, and
the state route visiting states rather than commitments each remove work without changing a match, a firing order or
an answer. Each is held to that by the invariance tests ([`03-design/01-determinism-and-order.md`](03-design/01-determinism-and-order.md)).

## 3. The levers

A lever changes how the engine computes, never what it answers: **no configuration field, option or environment name
changes `R`** ([Q-M1e2.30](../../decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)),
held for every mode under every lever by `ein-cli/tests/lever_invariance.rs`.

| lever | what it changes |
|---|---|
| `--jobs N` | the nodes of a batch computed on `N` threads; nothing a run prints ([`03-design/08-parallelism.md`](03-design/08-parallelism.md)) |
| `--region-budget F` | how far a run goes before it stops and says what it established |
| `--region-limit N` | how large a flagged program's cube a run computes whole |
| `--describe-route tree\|states` | which of describe's two routes runs on a monotone program |
| `-T S` | a deadline, past which the run is an error |
| `--no-search` | the question: root alone, `R` not discovered |
| `(config :lattice-sanity-check true)`, `-y` | the parent-path spot check, read by the read-out |
| `(config :record-alternative-justifications false)` | alternatives not recorded; the one lever that may change a dead node's core, never `R` |

Every field the `(config …)` form accepts, every option and every `EIN_*` name is listed, with whether it changes the
answer, in the configuration reference (today [`docs/ein/user/04-reference/06-config.md`](../user/04-reference/06-config.md)); a
field that is retired is refused at load with the reason (`ein_core::config::RETIRED`).
