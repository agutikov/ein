# The oracle event protocol — `--events FILE`

**Schema version:** `ein-events/1`

T2 parity is "the two engines took the same steps". That needs both
implementations to narrate what they did in a comparable format, which is what
this file specifies: **one JSON object per line**, opt-in behind
`--events FILE`, off by default.

Design: [`plans/m1a_rust/design/01_parity_contract.md`](../plans/m1a_rust/design/01_parity_contract.md) §3.
Built at [S1a.0.2](../plans/m1a_rust/p1a.0_conformance_harness/s1a.0.2_oracle_event_protocol.md).
It is specified as a schema rather than as debug output, so every other
observer — a trace viewer, a benchmark harness, an embedder — reads the same
stream.

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
   A protocol that perturbs the run it describes is not an oracle.
3. **No internal ids.** Facts are rendered with `cli._factdump.fact_sexpr`,
   the same canonical s-expression the CLI prints. Interned integers, object
   identities and dict addresses stay inside their implementations.
4. **Cost when enabled is irrelevant.** This is a debugging and parity mode,
   never a benchmark mode; the harness never times an `--events` run.

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
emitted, which keeps a hand-readable file. **T2 comparisons run at `verbose`**:
a dropped redundant firing is exactly the kind of difference a port introduces
and the tier exists to catch.

> **That last sentence is no longer the whole story.** Since M1a
> [S1a.6.9](../plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md),
> ein.rs's forks *resume* root's saturation where ein.py's re-derive it, so
> the two engines deliberately narrate different amounts of the same
> derivation —
> [D3](../plans/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it).
> The difference the tier exists to catch is a dropped **productive** firing,
> and since
> [S1a.6.10](../plans/m1a_rust/p1a.6_performance/s1a.6.10_parity_contract.md)
> that is what the comparison is narrowed to — see § Comparison. `verbose` is
> still what the harness asks for: the redundant firings are what the
> *elided-count* report is counting, and an ein.rs golden
> ([S1a.6.11](../plans/m1a_rust/p1a.6_performance/s1a.6.11_fixture_goldens.md))
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
([design/02](../plans/m1a_rust/design/02_determinism_and_order.md) §2), so
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

```sh
ein-conformance diff a.jsonl b.jsonl
```

Reports the first differing event with a field-level diff plus the preceding
four events from each side. `--classes` prints how many events of each kind
each side produced (and it prints unasked whenever there *is* a difference), so
a wholesale divergence is obvious before the first-diff detail — "b emitted no
`park` events at all" is a more useful first sentence than a diff at line 4.

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

That is [design/01 §5](../plans/m1a_rust/design/01_parity_contract.md#the-fork-row-stated-once)'s
fork row, implemented in `ein.rs/crates/ein-parity`, whose module doc carries
the measurement that chose it (six candidate cuts over the same 240 cells;
this one is the strongest that leaves D2 as the only differing cell).
`--strict` restores the old event-by-event comparison.

The harness runs the same comparison in-process for its T2 tier
(`tier::compare`), on logs it produced by appending
`--events {out}/events.jsonl --events-level verbose` to each `solve` /
`saturate` cell — so the hand tool and the gate cannot drift apart. At T3 the
event log is compared this way too rather than byte-wise, for the same
renumbering reason.

## Versioning

`ein-events/<n>`. A new optional field is not a version bump; a removed field,
a renamed event, or a changed field meaning is. The version rides in the `run`
event rather than a header line so that a truncated file still identifies
itself — the writer flushes per line, and a crashed run's prefix is the most
useful artefact it can leave.
