# The `--events` protocol

**Schema version:** `ein-events/1`

`ein solve --events FILE`, `ein saturate --events FILE` and — since M1c
[S1c.1.3](../../history/m1c_external_validation/README.md#s1c13--ein-test)
— `ein test --events FILE` make the engine narrate what it did: **one JSON
object per line**, opt-in, off by default.
Every compile miss, enqueue, firing, mirror, park/admit/retire, quiescence,
alternative justification, obligation, load-time warning, generation rung,
hypothesis verdict, entering, layer, no-good and writeback, in the order the
engine performed them — with one exception, and it is declared: under
`EIN_TRAVERSAL=tree` four of those kinds are not emitted at all, which is
[§ `traversal`](#traversal--the-second-traversal-and-the-four-kinds-it-does-not-emit).

**[§ Events](#events) is the enumeration; that sentence is a summary of it.**
The two were parallel copies until M1e `CD-M2`, and the sentence was the one
that had gone stale — a `warn` line had been in the stream since S1e.2.3 with
no row on this page.
[`events_reference.rs`](../../../ein.rs/crates/ein-cli/tests/events_reference.rs)
now fails on a kind the emitters produce and § Events does not name.

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
| `warn` | after root saturation, before the search — one line per breach, in the checker's own order | `category`, `message` (rendered, one line) |

**`type` gained a fourth value at M1d
[S1d.2.6](../../../docs/history/m1d_satisfiability/README.md#s1d26--verdicts-counters-corpus)
— `Open`** — and `k` changed meaning with it, from *recorded nodes* to
**models**. The two were the same number until a state could be complete and
still owe; on an `Open` line `k` is 0 while `counters.solution_nodes` is the
node count, and the difference is the open states. Both are in the same event
on purpose: a consumer that saw one could not tell an open state from a model,
and one that sees both can. `models` carries the open states' fact sets, in the
same slot and the same sorted form, because a fact set is a fact set and `type`
is what says they were not called models.

Programs that state no obligation never emit `Open`, so every pre-M1d stream is
byte-identical.

**`warn` is the only kind that is about the *program* rather than about a step
the engine took**, which is why it is here and not in either layer below. Three
categories ship, and the spelling is not uniform because two of them are class
names and one is not:

| `category` | what it says | where |
|---|---|---|
| `DerivedNafWarning` | a rule's `(absent …)` reads a relation some rule *derives*, so its verdict depends on when it is asked | [`naf_deps.rs`](../../../ein.rs/crates/ein-infer/src/naf_deps.rs) |
| `RefutationUnderAbsentWarning` | …and the reading is a **refutation**, which is [Q-M1e.9](../../../plans/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent)'s shape | ⤳ |
| `alive-set-invariant` | a rule asserts a constant the ontology does not name — M1e [`ST-M1`](../../../plans/m1e_review_processing/p1e.3_medium/s1e.3.3_state_model.md), where a breach costs a **verdict** | [`invariant.rs`](../../../ein.rs/crates/ein-infer/src/invariant.rs) |

The first two are **opt-in** behind `(config :warn-derived-naf true)` — one
flag, two questions, because `SolverConfig` is rendered into the KB-shape
digest and an eighteenth field would re-bless every shape golden in the corpus.
The third is unconditional; like every other kind here it costs nothing while
`--events` is off. A consumer should match on `category` and treat an unknown
one as informational: this list grows whenever a static check does.

`impl` and `argv` are **not compared**. `impl` names which implementation ran,
which is the point of the comparison rather than a finding; `argv` carries the
artefact paths the *caller* chose, so `--events a.jsonl` against
`--events b.jsonl` is not a divergence. Both stay in the file, where they
document the run.

### Deductive layer

| `e` | emitted at | payload |
|---|---|---|
| `compile` | `Engine::compile_for` — **miss only** | `rule`, `activator`, `n_steps`, `n_disjuncts`, `n_guards`, `asserts` |
| `enqueue` | `Saturator::enqueue_binding`, after the dedup check | `rule`, `activator`, `bindings` (in binding order), `priority`, `tiebreaker`, `parked` |
| `fire` | every `Firing` yielded by `Saturator::closure_step` | `rule`, `activator`, `bindings`, `premises`, `derived`, `redundant` |
| `mirror` | the native `__symmetric__` arg-swap write | `relation`, `src`, `derived` |
| `park` / `retire` | `Saturator::admit_from_boundary` — the candidate waits for a later round, or is dead | `tiebreaker`, `round`, `rule`, `watched` (**the failing guard's** watch set, sorted) |
| `admit` | `Saturator::admit_from_boundary` — the one candidate a round admits | `tiebreaker`, `round`, `rule`. **No `watched`** — nothing failed, so there is no failing guard to read a watch set off |
| `quiesce` | closure quiescence, before the boundary speaks | `round`, `n_facts`, `n_queue`, `n_parked` |
| `alt` | `Kb::record_justification` returns true | `fact`, `rule`, `premises` |
| `owe` | the **post-fixpoint obligation pass**, once per quiescent KB, one line per undischarged instance | `rule`, `activator`, `relation` (the `(open ?R)` argument, `""` for a bare `(open)`), `bindings`, `why` (rendered) |

**Three of `compile`'s six numbers are not what they are named**, and they are
reproduced rather than corrected because the event is a comparison surface —
`ein.py` printed these and a renamed field would silence a diff instead of
resolving one ([`engine.rs`](../../../ein.rs/crates/ein-infer/src/engine.rs)).
On a plan with *d* disjuncts:

- **`n_guards` is *d*** — `len(plan.naf_guards)`, one guard *tuple* per
  disjunct, whether or not the disjunct has a guard in it. It is the disjunct
  count and not the guard count, and it is the field a consumer is most likely
  to sum;
- **`n_disjuncts` is *d* − 1** — the *extra* disjuncts, so a rule with no
  `(or …)` reports `n_disjuncts 0` and `n_guards 1`;
- **`n_steps` is the first disjunct's** step count alone, not the plan's.

`asserts` and `rule` mean what they say. This is the whole of the divergence:
the truth used to live in an `engine.rs` comment and nowhere a consumer of the
protocol would read it (M1e `CD-M2`).

**`owe` is not a firing** — M1d
[S1d.2.4](../../../docs/history/m1d_satisfiability/README.md#s1d24--obligations-in-the-saturator).
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
`record_justification` returns true only for rule-kind provenance with at least
one premise, so a re-derived *source* fact records nothing and emits nothing.

`enqueue`'s `tiebreaker` is the value `Saturator::tiebreaker` took for that
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
| `traversal` | the **second traversal** deciding whether to run — once at root, and again at any node that changes its mind. `EIN_TRAVERSAL=tree` only | `kind` (always `tree`), `verdict` ∈ {`accepted`, `declined`}, `reason` (the rung `mode`), then `max_set_size` + `stop_after` on `accepted` and `depth` on a node's `declined` |

`hyp`'s `verdict` is the *name of the thing that dropped the candidate*, not a
boolean: `raw == emitted + Σ filtered` is the invariant `HypGenStats` already
asserts, and naming the filter makes a counter difference locate itself.
(`hypgen::apply_filters` returns that name rather than a bool for exactly this
reason.)

A pre-candidate skip gets its own kind rather than a `hyp` with an invented
`fact`, because at that point no candidate exists: `closed_relation`,
`relation_not_whitelisted` and `no_hypothesis_relation` are decisions about a
*relation*, and `self_edge` about an (object, relation) pair. They are verbose
only — `self_edge` alone fires once per (object, filler, relation, slot).

#### `rung` — which generator proposed

Added by M1d
[S1d.2.5](../../../docs/history/m1d_satisfiability/README.md#s1d25--hypotheses-from-obligations),
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
| `declined` | it refused the whole call and the blind enumerator ran instead — an obligation whose guard scans a relation the rung itself proposes (the [domain contract](../../../docs/history/m1d_satisfiability/domain_contract.md)'s C4), or a projection that did not resolve for this activator |

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
[the stage record](../../../docs/history/m1d_satisfiability/hypotheses_from_obligations.md)
is where the zebra family settled it.

A `rung` line is emitted per **generation call**, not per node: `complete`
asks the generator once per alive entering, so the count tracks
`enterings_alive` plus the root and inter-layer calls.

#### `layer` — the clause-yield census

Added by M1d
[S1d.10.1](../../../docs/history/m1d_satisfiability/README.md#s1d101--why-it-does-not-finish),
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
`-E` budget — so `Σ entered` is `enterings_total` on any lattice run at all.
That is what makes a budget a **probe**: `solve -e -m 4 -E <enterings so far +
1>` *generates* layer 4 and reports what the join proposed without entering it,
which is how [S1d.10.1](../../../docs/history/m1d_satisfiability/layer_census.md)
priced a depth nobody can run. Under `EIN_TRAVERSAL=tree` there are no layers
and the sum is `0` against a non-zero `enterings_total` — see
[§ `traversal`](#traversal--the-second-traversal-and-the-four-kinds-it-does-not-emit).

**No timing here.** A `ms` field would make the stream non-deterministic and the
goldens unreadable; ground rule 4 says an instrumented run is not a benchmark
anyway. `utils/layer_census.py` times a **second, bare** child for that reason.

#### `traversal` — the second traversal, and the four kinds it does not emit

Added by M1e
[S1e.3.2](../../../plans/m1e_review_processing/p1e.3_medium/s1e.3.2_semantics.md)
for a line M1d
[S1d.10.6](../../history/m1d_satisfiability/README.md#s1d106--the-traversal)
had been emitting since it shipped. **`EIN_TRAVERSAL=tree` only** — every
stream taken without it is byte-identical to the streams taken before the
traversal existed, which is the same promise `rung` makes.

The tree branches on **one owed instance's alternatives** instead of
enumerating subsets of a fixed `alive`, and it runs on the obligations rung and
on no other: an hrule's candidates are not jointly exhaustive, and branching on
them is the `d!`-per-set depth-first solver P1.5b deleted. So the first thing
it does is ask the rung, and this line is the answer.

| `verdict` | when | the rest of the payload |
|---|---|---|
| `accepted` | root's probe came back `obligations` — the tree runs and the lattice does not | `max_set_size`, the **sentence** *not applicable — depth is bounded by discharge*, and `stop_after` (`-1` when the run is unbounded) |
| `declined` | root's probe came back anything else — the run is handed straight back to the lattice | — |
| `declined` **with `depth`** | an inner node re-read the rung and it was no longer `obligations` | `depth` |

`reason` is the rung `mode` verbatim (`obligations`, `hrules`, `blind`,
`stuck`, `declined`), so a decline says *which* generator answered.

**A declined run is the lattice's answer and not the lattice's stream.** Root's
probe is a real generation call, not a lookup, so the stream carries one extra
pass of it: on `examples/zebra2.ein` that is 125 further `hyp` lines, 125
further `compile` lines and the `traversal` line, 16 435 events against 16 184.
Every field of `--json-summary` is identical — counters, enterings and all —
because the probe's `HypGenStats` is local and dropped. So a stream diff across
this variable is a difference and a *verdict* diff is not.

The third row is the one worth knowing about. Root's probe used to be the only
one, on the premise that the rung is a property of the program; oblgen's mode
per node depends on activator **facts** and a fork derives facts, so
`Run::tree_node` re-reads it at every node it expands (M1e
[S1e.2.1](../../../plans/m1e_review_processing/p1e.2_high/s1e.2.1_correctness.md)).
An inner node cannot hand the run back to the lattice, so *decline* there means
*expand no further*, and this line is the only report of it. No corpus program
reaches it — today's stdlib activators are all root-asserted — and the
regression test is owed to
[S1f.10.6](../../../plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md).

**What an accepted tree does not narrate.** Four kinds this page lists are
absent from a tree stream, and none of them is a bug:

| kind | why |
|---|---|
| `enter` | it is emitted by `Run::finish_entering`, the lattice's wrapper; `tree_node` calls `commitment::try_commitment_set` directly and handles the result itself. **The enterings are invisible in the stream** — `stats.enterings_total` still counts them |
| `layer` | there are no layers. `stats.layers_explored` carries the **deepest node** instead, which is a different quantity under the same name — [T1d.10.6.4](../../history/m1d_satisfiability/README.md#s1d106--the-traversal)'s question and the reason this is behind an environment variable |
| `nogood` | a dead branch is **recorded and not learned from**. Its commitment and unsat core reach the answer, so a `Contradiction` states what it refuted; nothing is added to the clause store, so `nogoods_emitted` and `nogoods_subsumed` are `0` and the search stays byte-identical to the published **86 enterings** |
| `writeback` | the singleton `(not h)` write is the other half of the same decision |

Measured on the smallest program that reaches the dead arm — one person, two
foods, a rule that refutes every choice — the lattice emits `enter` ×2,
`layer`, `nogood` ×2 and `writeback` ×2 where the tree emits one `traversal`
and none of those seven lines. Both enter twice and both report
`enterings_total = 2`.

**And the verdict is qualified.** A tree terminates by *discharge* and a
lattice by *exhaustion*, so `Run::tree` sets `truncated` unconditionally: the
`verdict` event's `exhausted` is `false` on every tree run, whatever it found.
On the fixture above that is the difference between the lattice's *No solution
— the constraints are contradictory* and the tree's *No model found — the
search did not exhaust the lattice*, over the same two-fact core. What
discharge would license instead is T1d.10.5.1's sentence and it is not written;
until it is, a consumer reads `exhausted` as it reads it everywhere else.

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
