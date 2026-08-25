# The `--events` protocol

**Schema version:** `ein-events/1`

`ein solve --events FILE`, `ein saturate --events FILE` and — since M1c
[S1c.1.3](../../history/m1c_external_validation/README.md#s1c13--ein-test)
— `ein test --events FILE` make the engine narrate what it did: **one JSON
object per line**, opt-in, off by default.
Every compile miss, enqueue, firing, mirror, park/admit/retire, quiescence,
alternative justification, hypothesis verdict, entering, no-good and writeback,
in the order the engine performed them.

It is specified as a schema rather than as debug output, so every observer — a
trace viewer, a benchmark harness, an embedder,
[M20](../../../plans/m20_gui/README.md)'s likely feed — reads the same
stream.

> **Where it came from.** The protocol was built at
> [S1a.0.2](../../history/m1a_rust/README.md#s1a02--the-oracle-event-protocol)
> as the operand of the Rust port's **T2 parity tier** — "the two engines took
> the same steps" — and its design rationale is
> [`design/01`](../../history/m1a_rust/design/01_parity_contract.md) §3. The
> second engine left the tree at
> [P1a.10](../../history/m1a_rust/README.md#p1a10--one-implementation) and
> the tier went with it; the *format* did not, because nothing about it was
> ever about there being two engines. What reads it now is named in
> [§ Comparison](#comparison).

## Why not reuse `--dump-states`

`MonotonicDumper` already writes a `00_timeline.jsonl`, and it stays. But it
covers the *search* layer's five lifecycle hooks — root, layer, entering,
layer-end, summary — and says nothing about the deductive layer underneath.
Firings, enqueues, parks, admissions, retirements, quiescences, alternative
justifications: that is where a port drifts, and none of it is visible there.

## Ground rules

1. **Off is free.** With no `--events`, the writer is `None` and the emit call
   returns on an identity check. No formatting, no generator materialisation,
   no branch on any hot path that did not already exist.
2. **Emitting must not change behaviour.** In particular nothing here may
   advance `Saturator._tiebreaker`, consume a generator, or reorder a dict.
   A protocol that perturbs the run it describes narrates a different run.
3. **No internal ids.** Facts are rendered with `cli._factdump.fact_sexpr`,
   the same canonical s-expression the CLI prints. Interned integers, object
   identities and dict addresses stay inside their implementations.
4. **Cost when enabled is irrelevant.** This is a debugging and comparison
   mode, never a benchmark mode; nothing times an `--events` run.

## Line format

```json
{"e": "fire", "n": 41, "rule": "symmetric", "…": "…"}
```

| field | meaning |
|---|---|
| `e` | event kind, from the table below |
| `n` | per-run monotonic sequence number, from 0 |

Field order within a line is fixed by the emitter (`e`, `n`, then the kind's
own fields in the order listed below) so that a raw `diff` of two files is
readable even without the differ.

The first line of every run is a `run` event carrying the schema version, so a
consumer can reject a file it does not understand before reading further.

## Levels

`--events-level {normal,verbose}` (default `normal`).

Exhaustive `zebra2` produces roughly 40 k productive firings and a further
~194 k redundant ones. At `normal` a redundant firing is counted but not
emitted, which keeps a hand-readable file. **Everything that compares two
streams runs at `verbose`**: a dropped redundant firing is exactly the kind of
difference a port or an optimisation introduces.

> Not every difference at `verbose` is one. Since M1a
> [S1a.6.9](../../history/m1a_rust/README.md#s1a69--the-fork-entry-delta-the-resumed-saturator)
> a fork *resumes* root's saturation rather than re-deriving it, so two runs
> that reach the same answer can narrate different amounts of the same
> derivation —
> [D3](../../history/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it),
> which [S1a.10.1 §5](../../history/m1a_rust/oracle_ledger.md#5-what-the-successor-found)
> then reproduced *inside one engine* by permuting the id space. The difference
> worth catching is a dropped **productive** firing, and since
> [S1a.6.10](../../history/m1a_rust/README.md#s1a610--the-parity-contract-relaxes-answers-not-narration)
> that is what the comparison is narrowed to — see § Comparison. `verbose` is
> still what it reads: the redundant firings are what the *elided-count* report
> counts, and an ein.rs golden
> ([S1a.6.11](../../history/m1a_rust/README.md#s1a611--einrss-own-fixtures-for-what-parity-stopped-comparing))
> is what pins them.

## Events

### Lifecycle

| `e` | emitted at | payload |
|---|---|---|
| `run` | process start | `version`, `level`, `impl`, `file`, `argv`, `config` (every resolved `SolverConfig` field, kebab-cased) |
| `load` | after `kb.from_ir` | `relations`, `rules`, `hrules`, `macros`, `facts` counts; `relation_names` and `rule_names` in registry order |
| `verdict` | end | `type`, `k`, `exhausted`, `counters` (every `MonotonicStats` field), `core` (sorted), `models` (each a sorted fact list, the list itself sorted) |

`impl` and `argv` are **not compared**. `impl` names which implementation ran,
which is the point of the comparison rather than a finding; `argv` carries the
artefact paths the *caller* chose, so `--events a.jsonl` against
`--events b.jsonl` is not a divergence. Both stay in the file, where they
document the run.

### Deductive layer

| `e` | emitted at | payload |
|---|---|---|
| `compile` | `Engine.compile_for` — **miss only** | `rule`, `activator`, `n_steps`, `n_disjuncts`, `n_guards`, `asserts` |
| `enqueue` | `Saturator._enqueue_binding`, after the dedup check | `rule`, `activator`, `bindings` (in binding order), `priority`, `tiebreaker`, `parked` |
| `fire` | every `Firing` yielded by `_closure_step` | `rule`, `activator`, `bindings`, `premises`, `derived`, `redundant` |
| `mirror` | the native `__symmetric__` arg-swap write | `relation`, `src`, `derived` |
| `park` / `admit` / `retire` | `_admit_from_boundary` decisions | `tiebreaker`, `round`, `rule`, `watched` (the failing guard's watch set) |
| `quiesce` | closure quiescence, before the boundary speaks | `round`, `n_facts`, `n_queue`, `n_parked` |
| `alt` | `store.record_justification` returns True | `fact`, `rule`, `premises` |
| `owe` | the **post-fixpoint obligation pass**, once per quiescent KB, one line per undischarged instance | `rule`, `activator`, `relation` (the `(open ?R)` argument, `""` for a bare `(open)`), `bindings`, `why` (rendered) |

**`owe` is not a firing** — M1d
[S1d.2.4](../../../plans/m1d_satisfiability/p1d.2_obligations/s1d.2.4_obligations_in_the_saturator.md).
A rule whose `:assert` is the verdict atom `open` derives nothing, so it is
kept out of the saturation agenda entirely
([`06_reserved_names.md` § the verdict atom](../ir/03-ein-lang/06_reserved_names.md))
and runs as one pass over the KB *after* the fixpoint. Three consequences a
reader of this stream should know:

- **A rule never emits both.** An obligation rule can never produce a `fire`;
  a saturation rule can never produce an `owe`. That is what makes `owe` the
  activation evidence for the obligation half of the stdlib, which is how
  `ein-infer/tests/stdlib_coverage.rs` counts it.
- **It is emitted per quiescent KB, and only where the KB is consistent.** The
  read-out is three states in one order — `(false)` first, then the tally — so
  a node with a contradiction never has its debts consulted and narrates none.
  Root's lines come first, then one group per alive entering, each before that
  entering's `enter` line.
- **Nothing it reports is stored.** An `open` conclusion is a tally on the
  search-lattice node, never a fact: contradiction survives an extension and
  openness exists to be destroyed by one, so a stored `open` would outlive its
  own discharge in a fork that paid it. Re-reading the KB will not find what
  this line reports; `--json-summary`'s `owes` block is the same tally as a
  document.

A mirror produces a `mirror` event and **not** a `fire`, so every firing is
reported exactly once whichever path made it. `alt` is narrower than it looks:
`record_justification` returns True only for rule-kind provenance with at least
one premise, so a re-derived *source* fact records nothing and emits nothing.

`enqueue`'s `tiebreaker` is the value `Saturator._tiebreaker` took for that
entry. It is the engine's own total order over queue entries
([design/02](../../history/m1a_rust/design/02_determinism_and_order.md) §2), so
carrying it makes the heap's pop order checkable rather than inferred.

### Search layer

| `e` | emitted at | payload |
|---|---|---|
| `rung` | a generation call that **reached** the obligations rung | `mode`, `reason`, `owed`, `branches`, `declined`, `candidates`, `uncovered` |
| `hyp` | each constructed hypgen candidate | `fact`, `verdict` ∈ {`emitted`, the name of the filter that dropped it} |
| `hypskip` | a **pre-candidate** skip — *verbose only* | `relation`, `reason`, `object` (for `self_edge`) |
| `enter` | `try_commitment_set` returns | `layer`, `commitment`, `kind`, `n_firings`, `core` |
| `layer` | every way out of a layer | the census row — sixteen counters, below |
| `nogood` | `emit_nogood` | `clause`, `emitted`, `subsumed` |
| `writeback` | singleton `(not h)` / forced-positive promotion | `fact`, `reason` |

`hyp`'s `verdict` is the *name of the thing that dropped the candidate*, not a
boolean: `raw == emitted + Σ filtered` is the invariant `HypGenStats` already
asserts, and naming the filter makes a counter difference locate itself.
(`hypgen._apply_filters` returns that name rather than a bool for exactly this
reason.)

A pre-candidate skip gets its own kind rather than a `hyp` with an invented
`fact`, because at that point no candidate exists: `closed_relation`,
`relation_not_whitelisted` and `no_hypothesis_relation` are decisions about a
*relation*, and `self_edge` about an (object, relation) pair. They are verbose
only — `self_edge` alone fires once per (object, filler, relation, slot).

#### `rung` — which generator proposed

Added by M1d
[S1d.2.5](../../../plans/m1d_satisfiability/p1d.2_obligations/s1d.2.5_hypotheses_from_obligations.md),
which turned hypothesis generation from a switch into a **ladder**: the user's
`(hrule …)` if there is one, else the undischarged obligations, else the blind
enumerator. One line per generation call, and **only** for a program that
declares an obligation rule — the other 150 corpus files emit none, which is
what keeps every pre-M1d stream byte-identical.

**Rung 1 and rung 3 narrate nothing.** A puzzle with an `(hrule …)` and a
puzzle with no obligation rule both emit exactly the stream they emitted before
M1d, to the byte; a line appears only where the rung this stage added was
actually consulted, and its `mode` says how that went:

| `mode` | what it means |
|---|---|
| `obligations` | it generated: the candidates are the facts that would discharge what this state owes |
| `stuck` | it owed something and proposed nothing — every debt scoped out by `:no-hypothesis` / `:hypothesis-relations` / `(__closed__ R)`, or every candidate already refuted. **The state is not silently complete**: this line is the report |
| `declined` | it refused the whole call and the blind enumerator ran instead — an obligation whose guard scans a relation the rung itself proposes (the [domain contract](../../../plans/m1d_satisfiability/p1d.2_obligations/domain_contract.md)'s C4), or a projection that did not resolve for this activator |

`reason` is the C4 or projection sentence for `declined`, and the walk order
(`rule-order` / `fail-first`, `EIN_OBLIGATION_CHOICE`) otherwise. `owed` counts
the undischarged instances at this quiescence, `branches` those branched on,
`declined` those scoped out — `owed = branches + declined` — and `candidates`
the facts proposed before the filter pipeline, which is the `hyp` stream's
`raw` for this call.

`uncovered` is the ladder's **completeness condition as a number**: how many
hypothesis-eligible relations no obligation names, under the same eligibility
test the blind enumerator applies. `0` says the rung is exhaustive by
construction — every relation a hypothesis could be about is one some
obligation owes. Non-zero does not say it is *in*complete: it says the claim
now rests on saturation determining those relations, which only a model-set
comparison settles, and
[the stage record](../../../plans/m1d_satisfiability/p1d.2_obligations/hypotheses_from_obligations.md)
is where the zebra family settled it.

A `rung` line is emitted per **generation call**, not per node: `complete`
asks the generator once per alive entering, so the count tracks
`enterings_alive` plus the root and inter-layer calls.

#### `layer` — the clause-yield census

Added by M1d
[S1d.10.1](../../../plans/m1d_satisfiability/p1d.10_exhaustive_search/s1d.10.1_why_it_does_not_finish.md),
because the stream said what a layer's deaths *produced* and nothing said what
the resulting clauses *removed*. One line per layer, carrying
`ein_infer::LayerCensus` whole:

| field | what it counts |
|---|---|
| `layer` | 1-based depth — the cardinality of the commitments this layer entered |
| `alive` | hypothesis facts still unrefuted when the layer opened |
| `frontier` | the sets this layer joins over. **Not the previous line's `next`**: between them sits the inter-layer retain, so the difference is what recomputing `alive` at the barrier was worth |
| `joined` | what the Apriori prefix join proposed. Layer 1 has no join, so it is `alive` |
| `dropped_dead` | …rejected because an element had left `alive` |
| `dropped_nogood` | …rejected because a learned clause covers the set. **The column that did not exist** |
| `candidates` | survivors — what the layer's loop was handed |
| `entered` | how many it actually entered; fewer exactly when `-n`, `-T` or `-E` cut the layer |
| `alive_enterings`, `dead_pre`, `dead_post` | the entering split, for this layer |
| `models` | **distinct** solution nodes added — after the `state_key` dedup, so it is smaller than the number of enterings that reached one |
| `nogoods_emitted`, `nogoods_subsumed` | clauses learned, and clauses a held one already covered |
| `writebacks` | singleton `(not h)` writes. A forced positive runs *after* the barrier and is `MonotonicStats::forced_positives` |
| `next` | the frontier handed on, before the retain |

Every counter but `alive`, `frontier`, `joined` and the two `dropped_*` is a
difference of two whole-run counters taken at the layer's open and its close, so
a field added to `BaseStats` reaches this line without anyone re-deriving it.

**The two `dropped_*` are attributed in check order.** A candidate can fail both
questions; that is `dropped_dead`, so `dropped_nogood` means *every element
still alive and a learned clause covered the set anyway* — which is the clause
store's yield, and the reason for the split.

**A layer emits its row however it ends** — the barrier, an `-n` cut, a `-T` or
`-E` budget — so `Σ entered` is `enterings_total` on any run at all. That is
what makes a budget a **probe**: `solve -e -m 4 -E <enterings so far + 1>`
*generates* layer 4 and reports what the join proposed without entering it,
which is how [S1d.10.1](../../../plans/m1d_satisfiability/p1d.10_exhaustive_search/layer_census.md)
priced a depth nobody can run.

**No timing here.** A `ms` field would make the stream non-deterministic and the
goldens unreadable; ground rule 4 says an instrumented run is not a benchmark
anyway. `utils/layer_census.py` times a **second, bare** child for that reason.

## Comparison

Two streams are compared by
[`ein_parity::events`](../../../ein.rs/crates/ein-parity/src/events.rs), a
library function rather than a command: `split` cuts a log into segments and
`diff` reports the first segment that disagrees. The `ein-conformance diff`
subcommand that used to front it was retired with the second engine
([S1a.10.3](../../history/m1a_rust/README.md#s1a103--the-corpus-without-a-second-engine));
what calls it now is
[`ein-infer/tests/event_cut_control.rs`](../../../ein.rs/crates/ein-infer/tests/event_cut_control.rs),
which mutates a real stream and checks that the cut below still reports the
mutation.

`n` is compared as a **position, not a field**: one extra event on either side
renumbers every line after it, and a differ that reported all of them would
bury the one difference that caused them.

### What "compared" means since S1a.6.10

The stream is split into **segments** — root's saturation, then one per
entering — at every `enter`, and at the **first hypgen event**, which is what
closes root's. (That second boundary matters under `--lookahead`, where the
first entering is a probe that usually dies: without it, root's whole
derivation would share a segment with a `dead-post` fork and be skipped along
with it.) Within a segment:

| | compared |
|---|---|
| the **spine**: `run`, `load`, `hyp`, `hypskip`, `enter`, `nogood`, `writeback`, `warn`, `verdict` | in order, exactly — minus `enter`'s `n_firings`, and a `dead-post` `enter`'s `core` |
| the **derivation**: every `fire` with `redundant = false`, and every `mirror` | the **multiset of facts derived** and the **set of rules** that derived them |
| the scheduling traffic: `enqueue`, `park`, `admit`, `retire`, `quiesce`, `alt`, `compile`, and every redundant `fire` | nothing — counted per side and printed |

A **`dead-post`** segment's derivation is not compared at all: fail-fast stops
a dying fork at the firing that kills it, so its firing list is a prefix by
construction. Its `kind` and its position still are.

That is [design/01 §5](../../history/m1a_rust/design/01_parity_contract.md#the-fork-row-stated-once)'s
fork row, implemented in `ein.rs/crates/ein-parity`, whose module doc carries
the measurement that chose it (six candidate cuts over the same 240 cells;
this one is the strongest that leaves D2 as the only differing cell).
`EIN_PARITY_STRICT=1` restores the event-by-event comparison.

**A relaxation nothing exercises is a hole rather than a decision**, so the cut
has a negative control: `event_cut_control.rs` builds a real verbose stream,
deletes one event from it, and asserts that deleting a *productive* firing is
reported while deleting a redundant one or an `enqueue` is not. It is
the three mutations `utils/mutant_ein.py` applied until S1a.10.4, with the
two processes taken out.

## Versioning

`ein-events/<n>`. A new optional field is not a version bump; a removed field,
a renamed event, or a changed field meaning is. The version rides in the `run`
event rather than a header line so that a truncated file still identifies
itself — the writer flushes per line, and a crashed run's prefix is the most
useful artefact it can leave.
