# Reserved engine strings

Engine-**internal** vocabulary — strings the inference engine keys on that
are *not* author-facing surface syntax. (For names a puzzle author writes,
see [`../ir/03-ein-lang/06_reserved_names.md`](../ir/03-ein-lang/06_reserved_names.md).)

All are **reserved engine strings for M1**: each lives behind a single
named constant / enum, documented here. De-naming them (string → richer
type, or removing the hardcode) is a **post-M1** question — the routes are
parked in S1.7.19–.22. The point of this doc is that *a name is reserved
iff it appears here or in the surface doc*, and nothing is undocumented.

**Dunder convention (2026-06-15).** A name that triggers kernel-hardcoded
*behaviour* is written `__dunder__`, lexically distinct from userspace
rule/relation names (the grammar admits a leading `__`; a bare name never
triggers kernel behaviour). `__closed__` and `__symmetric__` are the two so
far. The surface task-class / control keywords below predate the
convention and keep their bare names.

## Bookkeeping carrier heads

**None currently.** The search once wrapped speculative facts in
synthetic carrier heads — `(hypothesis (R …))`,
`(contradiction-under …)` — that the canonical state form had to
exclude; both were retired for a provenance *kind*
([`ein-core/prov.rs`](../../../ein.rs/crates/ein-core/src/prov.rs)
`kind="hypothesis"`), so the canonical
[`canon::state_key`](../../../ein.rs/crates/ein-infer/src/canon.rs)
excludes nothing: it is the sorted, provenance-free `(relation_name, args)`
projection of the whole propositional fact set (P1.21 R1). Any future
carrier head must be registered here **and** excluded in
`canon.state_key` so it doesn't perturb model identity.

## Synthetic writeback rule names

Three `Provenance(kind="rule", …)` records name a rule that does not
exist in any `(rule …)` form. They are written by the **search** layer
when it writes a conclusion back to root, and they all share one shape —
**empty `premises_raw`**:

| name | written by | means |
|---|---|---|
| `<monotonic-unconditional>` | [`solve.rs`](../../../ein.rs/crates/ein-infer/src/solve.rs) | a size-1 dead clause's `(not h)` writeback: every branch assuming `h` died |
| `<forced-positive>` | [`promote_forced_positives`](../../../ein.rs/crates/ein-infer/src/solve.rs) | a sole-surviving alive singleton promoted to a root fact |
| `<lookahead-dies-immediately>` | [`hypgen::write_negated`](../../../ein.rs/crates/ein-infer/src/hypgen.rs) | the lookahead kill-cache's `(not h)` |

The empty premise tuple is the **contract**, not an omission: these facts
have no fact-level premises because their real justification is
*meta*-level — a property of the search (every alternative was refuted),
not a derivation inside any one world. Every provenance walk therefore
**grounds out** on them: they terminate the walk and contribute nothing
to a frontier, so an unsat core never names a search conclusion as a
given.

Multi-justification provenance (S1.21.7) preserves this in both
directions:
[`Kb::record_justification`](../../../ein.rs/crates/ein-core/src/kb.rs)
refuses to record such a record as an alternative (it would give some
fact the *empty* environment — "derivable from nothing" — collapsing
every explanation through it), and refuses to attach alternatives *to* a
fact whose primary is one (so a later `saturate()` re-deriving it by a
real rule cannot silently re-open the ground-out).
[`explain.rs`](../../../ein.rs/crates/ein-infer/src/explain.rs) treats them
as terminals labelled with the empty environment, which is exactly the
"contributes nothing" behaviour the single-justification walk had.

A new writeback name must be registered here and must keep the
empty-`premises_raw` shape, or the walks will try to expand it.

## Engine entry + verdict

There is **one** engine entry, `solve` — the verdict is **read from the
result**, not chosen by which function was called. From the count `k`
of distinct (`state_key`-deduped) solution nodes,
[`solve.rs`](../../../ein.rs/crates/ein-infer/src/solve.rs)
names the
verdict; `solve(..., store_lattice=True)` attaches a sound `LatticeProof`
carrying both the gaps view (`proof.solutions` — every model) and the
contradictions view (`proof.dead_commitments` + `verdict.unsat_core`).
The verdicts are **answers to one problem**, not separate commands.

| `k` (distinct **models**) | verdict | shape |
|-------------------------------|---------|-------|
| `k = 0` | `Contradiction` | unsat core — the smallest contradiction frontier: a minimum-cardinality AND/OR search over every recorded derivation (provenance-based, NAF-safe, budgeted); **not** a subset-minimal MUS |
| `k = 0` | `Open` | the open state(s), and what they **owe** — M1d S1d.2.6 |
| `k = 1` | `Solution` | the model |
| `k > 1` | `Ambiguity` (gaps) | the distinct model states |

**`k` is a claim, and `exhausted` is what licenses it — M1d S1d.3.3.** With
the lattice exhausted, `k` is *the* model count; without it, `k` is a **lower
bound**, and the `Ambiguity` rendering says so in two places (`(a lower bound
— the search did not exhaust)` beside the number, and *models found* in the
sentence). `Solution` has said the same thing as *"(not certified — pass
--exhaustive)"* since ein.py. The words themselves do not change: the verdict
is still read from the result, and `verdict.type` is still `Ambiguity`.

**`k` counts models, and since M1d S1d.2.6 that is not the same number as
`stats.solution_nodes`.** The counter says what the *search* recorded — nodes
the generator called complete — and `k` says how many of them the read-out
calls models. They agree on every verdict but `Open`, where a node is complete
by exhaustion and undischarged by tally; the summary reports both, and the open
states themselves under `verdict.open_states` rather than `verdict.solutions`.

**`Open` is scoped.** Only a program that *states* an obligation can reach it
(`owes.declared > 0`): a state is judged by discharge when it has been told
what it owes and by exhaustion when it has not, so a program declaring none
reports exactly the words it reported before M1d P1d.2. `false` outranks — a
state that derived a refutation is `Contradiction` whatever it owes — and a
recorded model outranks an open state, so `Open` is said only when nothing was
discharged and something is owed. What it never means is *refuted*: the
distinction it exists to draw is between **no model** and **not yet a
model**.

The `k = 0` payload is per **witness**: the smallest set of *given* facts
(`source` / `hypothesis` / un-provenanced) from which **one** recorded
contradiction follows —
[`explain::smallest_contradiction_frontier`](../../../ein.rs/crates/ein-infer/src/explain.rs)
delegating to
[`explain.rs`](../../../ein.rs/crates/ein-infer/src/explain.rs), ATMS
label propagation over the AND/OR justification graph
(`kb.justifications(fact)` — a fact is an OR-node over AND-nodes of premises),
so the answer does not depend on which derivation fired first. Two caveats
survive: minimality is relative to the **recorded** derivations (only the
firings the saturator attempted, capped per fact at
`store.MAX_ALT_JUSTIFICATIONS`), and the search is **budgeted**
(`ExplanationBudget`; `Explanation.exhausted` reports whether it ran to
completion — a truncated search is still sound, just possibly not smallest).
Across several dead commitments the verdict still unions their cores (with an
exhausted lattice no single dead explains unsat), but each dead's core is
itself the smallest explanation of that dead.

(Historical: the unsound sibling entries `gaps_solve` /
`contradictions_solve` — which fixed the verdict by *which function was
called* and so disagreed on the same input — were removed 2026-06-16.)

The `Mode` enum still exists as **engine-internal** vocabulary — not a
caller-facing task switch. Single source: the `Mode` enum in
[`verdict.rs`](../../../ein.rs/crates/ein-infer/src/verdict.rs).
Only `Mode.SOLVE` reaches the live `solve` path (`is_solved` uses it for
the fork-side exactly-one-binding goal check); `GAPS` / `CONTRADICTIONS`
survive as the enum's other members but no longer name an entry.

| string | `Mode` | role |
|--------|--------|------|
| `solve` | `Mode.SOLVE` | goal check: exactly one binding (used by `solve` / `is_solved`) |
| `gaps` | `Mode.GAPS` | goal check: ≥ 1 binding (enum member; no live entry) |
| `contradictions` | `Mode.CONTRADICTIONS` | goal check: never solved (enum member; no live entry) |

## Protocol enums

Closed string sets the engine branches on internally (surveyed in
S1.7.22).
Each is a `Literal[...]` / enum in one place; de-naming → string-to-enum is
post-M1.

| set | values | where |
|-----|--------|-------|
| provenance kind | `source` · `rule` · `hypothesis` · `rejected` | `ein-core/prov.rs` |
| lattice node verdict | `alive` · `dead` · `solution` | `ein-infer/solve.rs` |
| dead kind | `dead-pre` · `dead-post` | `ein-infer/solve.rs` (`DeadCommitment.kind`) |
| hypgen scoring | `most-constrained` · `popularity` (`branch-info` reserved) | `hypgen.score_hypothesis` |

## `__closed__` — engine effect

The kernel trigger `(__closed__ R)` (a **dunder** name per the convention above;
the bare `closed` is now a free userspace name) has two engine sides, both
isolated in
[`closed.rs`](../../../ein.rs/crates/ein-infer/src/closed.rs)
(constant `CLOSED = "__closed__"`):

- **Auto-inference** — `emit_closed` writes `(__closed__ R)` for every declared
  relation no compiled rule positively asserts (`producible_relations`),
  run once before the initial saturation.
- **Hypgen suppression** — `hypgen._is_closed` reads `(__closed__ R)` facts and
  contributes zero candidates for R.

**Genuinely kernel** (a saturation rule can neither suppress hypgen generation
nor introspect "no rule asserts R" — see the symmetric contrast in the
`__symmetric__` design). Load-bearing for hypgen scoping / NAF soundness
(S1.7.10).
Renamed `closed → __closed__` 2026-06-15 per the dunder convention;
`std.closure`'s `infer-closure` asserts `(__closed__ ?R)`.

## `__symmetric__` — engine effect

The kernel trigger `(__symmetric__ R)` closes R's extension under arg-swap
**natively in the saturator**: each `(R a b)` produces `(R b a)` directly
(self-loops `a=b` and already-present mirrors skipped), as a `Firing` with rule
`__symmetric__` threading the source edge as its premise. Single source:
`SYMMETRIC = "__symmetric__"` + the mirror machinery
(`_next_mirror_firing` / `_enqueue_mirror_sources`) in
[`saturator.rs`](../../../ein.rs/crates/ein-infer/src/saturator.rs).

**A performance optimization, NOT a capability.** It computes the *identical*
closure as the stdlib `symmetric` rule (`std.algebra`) — pinned by
`stdlib_semantics.rs::the_native_mirror_computes_the_same_closure_as_the_stdlib_rule`
— but skips the plan + match the rule pays per mirror. **No real
symmetric-heavy puzzle exists yet** — zebra2 uses `co-located*` rules, not the
generic closure — so the speed-up is measured only on a synthetic generator,
and the ~1.2× that used to be quoted here came from a runner that left with
the Python engine. Opt-in by marking
the relation; ordinary puzzles take the no-op path (the mirror queue is empty
when nothing is marked, so zero overhead). Re-adds, behind the dunder, the
kernel symmetric-awareness S1.7.24
removed — now namespaced so it never masquerades as a userspace name.

## Query-scoping keys

`(query …)`-block keywords the hypothesis generator reads to scope which
relations it enumerates. Single source: the `HYPOTHESIS_RELATIONS` /
`NO_HYPOTHESIS` constants in
[`hypgen.rs`](../../../ein.rs/crates/ein-infer/src/hypgen.rs); both
scope the *blind enumerator* only (hrule-driven generation ignores them).

| key | effect | where |
|-----|--------|-------|
| `hypothesis-relations` | **whitelist** — enumerate candidates *only* for the listed relations (unset ⇒ all) | `ein-infer/hypgen.rs` |
| `no-hypothesis` | **blacklist** (S1.9.E3) — never guess on the listed relations; saturation rules on them still fire | `ein-infer/hypgen.rs` |

A relation named by both is excluded (blacklist wins). Neither touches the
saturator — hypgen-only scoping, distinct from `(__closed__ R)` above (which also
blocks rule-derivation).

## Result-level invariants (S1.7.24)

Not strings, but recorded here as part of the engine contract: the
lattice snapshot (`ein-render/dump/snapshot.rs`) is **result-level** — it keys
solutions/deads on the post-saturation `state_key`, NOT commitment paths, and
**excludes learned nogoods** (a clause and its symmetric mirror are
equivalent only under symmetry, so the final nogood set is an
order/orientation-sensitive optimisation artifact, not part of the result).
