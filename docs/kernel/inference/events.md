# The `--events` protocol

**Schema version:** `ein-events/1`

`ein solve --events FILE` and `ein saturate --events FILE` make the engine
narrate what it did: **one JSON object per line**, opt-in, off by default.
Every compile miss, enqueue, firing, mirror, park/admit/retire, quiescence,
alternative justification, hypothesis verdict, entering, no-good and writeback,
in the order the engine performed them.

It is specified as a schema rather than as debug output, so every observer — a
trace viewer, a benchmark harness, an embedder,
[M20](../../../plans/m20_gui/README.md)'s likely feed — reads the same
stream.

> **Where it came from.** The protocol was built at
> [S1a.0.2](../../../plans/m1a_rust/p1a.0_conformance_harness/s1a.0.2_oracle_event_protocol.md)
> as the operand of the Rust port's **T2 parity tier** — "the two engines took
> the same steps" — and its design rationale is
> [`design/01`](../../../plans/m1a_rust/design/01_parity_contract.md) §3. The
> second engine left the tree at
> [P1a.10](../../../plans/m1a_rust/p1a.10_single_implementation/README.md) and
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
> [S1a.6.9](../../../plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md)
> a fork *resumes* root's saturation rather than re-deriving it, so two runs
> that reach the same answer can narrate different amounts of the same
> derivation —
> [D3](../../../plans/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it),
> which [S1a.10.1 §5](../../../plans/m1a_rust/p1a.10_single_implementation/oracle_ledger.md#5-what-the-successor-found)
> then reproduced *inside one engine* by permuting the id space. The difference
> worth catching is a dropped **productive** firing, and since
> [S1a.6.10](../../../plans/m1a_rust/p1a.6_performance/s1a.6.10_parity_contract.md)
> that is what the comparison is narrowed to — see § Comparison. `verbose` is
> still what it reads: the redundant firings are what the *elided-count* report
> counts, and an ein.rs golden
> ([S1a.6.11](../../../plans/m1a_rust/p1a.6_performance/s1a.6.11_fixture_goldens.md))
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

A mirror produces a `mirror` event and **not** a `fire`, so every firing is
reported exactly once whichever path made it. `alt` is narrower than it looks:
`record_justification` returns True only for rule-kind provenance with at least
one premise, so a re-derived *source* fact records nothing and emits nothing.

`enqueue`'s `tiebreaker` is the value `Saturator._tiebreaker` took for that
entry. It is the engine's own total order over queue entries
([design/02](../../../plans/m1a_rust/design/02_determinism_and_order.md) §2), so
carrying it makes the heap's pop order checkable rather than inferred.

### Search layer

| `e` | emitted at | payload |
|---|---|---|
| `hyp` | each constructed hypgen candidate | `fact`, `verdict` ∈ {`emitted`, the name of the filter that dropped it} |
| `hypskip` | a **pre-candidate** skip — *verbose only* | `relation`, `reason`, `object` (for `self_edge`) |
| `enter` | `try_commitment_set` returns | `layer`, `commitment`, `kind`, `n_firings`, `core` |
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

## Comparison

Two streams are compared by
[`ein_parity::events`](../../../ein.rs/crates/ein-parity/src/events.rs), a
library function rather than a command: `split` cuts a log into segments and
`diff` reports the first segment that disagrees. The `ein-conformance diff`
subcommand that used to front it was retired with the second engine
([S1a.10.3](../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.3_corpus_without_an_oracle.md));
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

That is [design/01 §5](../../../plans/m1a_rust/design/01_parity_contract.md#the-fork-row-stated-once)'s
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
