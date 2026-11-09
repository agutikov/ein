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

**The engine's five names are reserved, and refused as a program's own**
(M1f S1f.5.5's T1f.5.5.11, 2026-11-08, on the user's ruling of that day,
*"they are also reserved names"*, and on
[Q-M1f.7](../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f7--where-may-a-program-write-the-kernels-dunders)
(b), *"Never bound"*). Each is written only with its fixed meaning:

| name | a program may | a program may not | refused by |
|---|---|---|---|
| `__closed__`, `__symmetric__` | **use** it wherever a name is used: as the relation of a fact, a premise, a conclusion, a goal or a claim's fact, and as a value, `(imply1 functional __closed__)` among them | **bind** it: declare a relation of that name, name a rule, an hrule or a macro after it, or import a module `:as` it | the loader, *relation '__closed__' shadows a reserved kernel name* (and *rule*, *hrule*, *macro*, *alias*): the dunders are in [`ein_core::RESERVED`](../../../ein.rs/crates/ein-core/src/terms.rs), the names a declarator may not bind |
| `<forced-positive>`, `<monotonic-unconditional>`, `<lookahead-dies-immediately>` | — | write it anywhere: they name a conclusion no rule drew, which a program cannot | the parse: the generated grammar's `WRITEBACK` token, which no production takes (*unexpected `<forced-positive>`: the engine's writeback rule name (reserved)*), and the hand-written lexer, which has no token that starts with `<` |

So a printed `:rule <lookahead-dies-immediately>` is always the engine's, and a
rule's firings never print `:rule __symmetric__`, the native mirror's. The
name in a string or a comment is not the name, and a longer run that starts
with one (`<forced-positive>x`) is the name it spells.
[`ein-ir/tests/engine_names.rs`](../../../ein.rs/crates/ein-ir/tests/engine_names.rs)
holds each name in each position through both parsers. Until T1f.5.5.11 a
program could bind a dunder through either parser, and write a writeback name
through the generated one.

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
**empty `premises_raw`**. Since M1e2 S1e2.1c.5 (2026-11-03) only the third
has a writer: the first two were the walk's and went with it, and the names
stay reserved in `ein_core::terms::ENGINE`.

| name | written by | means |
|---|---|---|
| `<monotonic-unconditional>` | `solve.rs`, deleted with the walk at M1e2 S1e2.1c.5 | a size-1 dead clause's `(not h)` writeback: every branch assuming `h` died |
| `<forced-positive>` | `promote_forced_positives`, deleted with the walk at M1e2 S1e2.1c.5 | a sole-surviving alive singleton promoted to a root fact |
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

## Names the parser and the loader make up

Five names stand for something a program writes no name for. Each starts
with `#`, which is no name character: the name rule leaves it out, since `#|`
opens a comment, so no program can spell one. `ein_core::text::INTERNAL_NAMES`
lists them, `ein_core::text::is_internal` is the test the dumper and
`(import … :symbols …)` read, and
[`ein-ir/tests/generated_parser.rs`](../../../ein.rs/crates/ein-ir/tests/generated_parser.rs)'
`no_name_the_loader_makes_up_is_a_name` holds that none is a name of the
generated lexer. Until M1f S1f.5.5's T1f.5.5.8 (2026-11-08) they were spelled
with `@` and `<…>`, which the name rule makes name characters, so a program
could have written `(p (@empty))` and loaded the fact `(p ())`.

| name | until T1f.5.5.8 | names | printed as |
|---|---|---|---|
| `#empty` | `@empty` | the empty form `()`: a `:match ()`, an `:assert ()`, an empty name list | `()` |
| `#nested` | `<nested>` | a nested form whose head is not a name, `(r (_ a))` | `(_ …)` |
| `#params` | `@params` | a rule's or a macro's parameter list | the list, with no head |
| `#list` | `@list` | a fact's `:using` list, in the generated parser's tree | the list, with no head |
| `#query` | `<query>` | the rule a query's `:goal` compiles to, so a binding has a rule to be reported under (`ein_core::terms::QUERY_RULE`) | a quoted name |

No run reaches a value `#empty` since M1f
[S1f.5.9](../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md)'s
T1f.5.9.2 (2026-11-09): `:match ()` is the empty conjunction, which fires once,
and `:assert ()` the empty conclusion, which asserts nothing, each compiled
without the synthetic head. Until then `()` was a premise over `#empty`, which
no fact holds, so the rule never fired (W31), and `:assert ()` stored a fact
under `#empty`, which joined H as `(q "#empty")` (`W34/assert-empty`).

## Engine entry + verdict

> **This section is the walk's read-out, deleted with the walk at M1e2
> S1e2.1c.5 (2026-11-03)** — `solve.rs`, `LatticeProof` and `verdict.rs`'s
> verdict types. Every question about the answer is
> [`ein_infer::explore`](../../../ein.rs/crates/ein-infer/src/explore.rs)'s,
> read off the region R ([`solution_semantics.md`](solution_semantics.md)),
> and `--single`'s four words are `explore::Verdict`. What the section says
> of the unsat core — `explain::smallest_contradiction_frontier` and its
> caveats — is still the core a route reports for a dead node.

There is **one** engine entry, `solve` — the verdict is **read from the
result**, not chosen by which function was called. From the count `k`
of distinct (`state_key`-deduped) solution nodes,
`solve.rs`
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
sentence). `Solution` has long said the same thing as *"(not certified — pass
--exhaustive)"*. The words themselves do not change: the verdict
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
so the *search* does not depend on which derivation fired first. Two caveats
survive: minimality is relative to the derivations the program **records**
(only the firings the saturator attempted; of each fact's, the store keeps up
to `store.MAX_ALT_JUSTIFICATIONS` = 32 with the fewest premises and, beside
them since M1e2 S1e2.2.11, the one with the smallest closure, so the
`alt-cap-core` pair report one core; a saving between two facts behind a full
list is still refused, `W10/alt-cap-shared`, deferred to F22), and the search is **budgeted**
(`ExplanationBudget`; `Explanation.exhausted` reports whether it ran to
completion — a truncated search is still sound, just possibly not smallest —
and since M1e2 S1e2.2.23 every surface that prints a core prints its status
beside it, `core_status` and the read-out's `core search` row).
Across several dead commitments the verdict still unions their cores (with an
exhausted lattice no single dead explains unsat), but each dead's core is
itself the smallest explanation of that dead.

(Historical: the unsound sibling entries `gaps_solve` /
`contradictions_solve` — which fixed the verdict by *which function was
called* and so disagreed on the same input — were removed 2026-06-16.)

**And the `Mode` enum went with them.** There is no `Mode` — no `SOLVE` /
`GAPS` / `CONTRADICTIONS` — in
[`verdict.rs`](../../../ein.rs/crates/ein-infer/src/verdict.rs), nor anywhere
else in `ein-infer`, and no goal-check string is branched on. The
exactly-one-binding test is
[`hypgen::complete`](../../../ein.rs/crates/ein-infer/src/hypgen.rs) —
*the generator proposes nothing at this state* — taken unconditionally, and
the verdict is read off `k` afterwards.

*(Three `Mode` enums do exist in the workspace and none of them is this one:
`ein_render::trace::Mode` — `Engine` / `Reorder`, the trace layout;
`ein_cli::factdump::Mode`; and `ein_infer::oblgen::Mode`, the hypothesis
ladder's rung. Recorded M1e S1e.2.2, which found this section by resolving
every identifier on the page. The first went with the walk's trace at M1e2
S1e2.1c.5 (2026-11-03); `ein_infer::explore::Mode`, a struct, is which part
of R a run asks about and what it asks.)*

## Protocol enums

Closed string sets the engine branches on internally (surveyed in
S1.7.22).
Each is a `Literal[...]` / enum in one place; de-naming → string-to-enum is
post-M1.

| set | values | where |
|-----|--------|-------|
| provenance kind | `source` · `rule` · `hypothesis` · `rejected` | `ein-core/prov.rs` |
| lattice node verdict | `alive` · `dead` · `solution` | `ein-infer/solve.rs`, deleted with the walk at M1e2 S1e2.1c.5 (2026-11-03) |
| dead kind | `dead-pre` · `dead-post` | `ein-infer/solve.rs` (`DeadCommitment.kind`), deleted with the walk at M1e2 S1e2.1c.5 (2026-11-03) |
| hypgen scoring | `most-constrained` · `popularity` (`branch-info` reserved) | `hypgen.score_hypothesis` |

## `__closed__` — engine effect

The kernel trigger `(__closed__ R)` (a **dunder** name per the convention above;
the bare `closed` is now a free userspace name) has two engine sides, both
isolated in
[`closed.rs`](../../../ein.rs/crates/ein-infer/src/closed.rs)
(constant `CLOSED = "__closed__"`):

- **Auto-inference** — `emit_closed` writes `(__closed__ R)` for every declared
  relation no compiled rule positively asserts (`producible_relations`). The
  walk ran it once before the initial saturation. **No route and no command
  runs it** since M1e2
  [S1e2.3.1](../../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.1_the_summary_root_block.md)
  (2026-11-03): its one caller is `shape::hyp_shape_with`, the corpus's `hyp`
  op. So a solve sees only the markers a program authors or `std.closure`
  derives, and H holds the candidates of every other relation.
- **Hypgen suppression** — `hypgen::closed_relations` reads the `(__closed__
  R)` facts in the KB, and the generator contributes zero candidates for R.

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
generic closure — so the speed-up is unmeasured on any real puzzle. Opt-in by
marking the relation; ordinary puzzles take the no-op path (the mirror queue is empty
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

Not strings, but recorded here as part of the engine contract: the walk's
lattice snapshot (`ein-render/dump/snapshot.rs`, deleted with the walk at M1e2
S1e2.1c.5, 2026-11-03) was **result-level** — it keyed
solutions/deads on the post-saturation `state_key`, NOT commitment paths, and
**excluded learned nogoods** (a clause and its symmetric mirror are
equivalent only under symmetry, so the final nogood set is an
order/orientation-sensitive optimisation artifact, not part of the result).
