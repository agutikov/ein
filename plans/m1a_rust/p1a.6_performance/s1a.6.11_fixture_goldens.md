# S1a.6.11 — ein.rs's own fixtures for what parity stopped comparing

**Phase:** P1a.6 (Performance)
**Estimate:** 2 days
**Depends on:** [S1a.6.10](s1a.6.10_parity_contract.md) — which is what makes
this necessary rather than merely nice.

## Context

[S1a.6.10](s1a.6.10_parity_contract.md) stops the harness diffing ein.rs's
narration against ein.py's, because since
[S1a.6.9](s1a.6.9_fork_entry_delta.md) the two engines deliberately narrate
different amounts of the same derivation. That leaves a gap: the trace, the
event stream and the state dumps were tested *only* by being compared to
ein.py, so relaxing the comparison un-tests them.

The replacement is the ordinary one and it is overdue independently of D3: a
port is compared against its oracle, but a **shipping engine** is compared
against checked-in fixtures. ein.rs is becoming the shipping engine.

## Tasks

### Task T1a.6.11.1 — Trace goldens over *real* solves

`ein.rs/crates/ein-render/tests/golden_trace.rs` exists and is not what this
needs: it renders a **synthetic** three-step `Trace` built by hand, which
locks the renderer and says nothing about what a solve produces. That is why
it kept passing through S1a.6.9 while the rendered trace lost half its rules.

Extend it to a handful of real solves that between them exercise the shape:
one unconditional, one with a single-hypothesis solution, one with a `k ≥ 2`
commitment, one unsat with reductios. The golden is the rendered markdown with
the dot blocks stripped (those have their own goldens — `golden_dot.rs`).

The one that matters most is **the root-saturation section**
([S1a.6.9](s1a.6.9_fork_entry_delta.md) T1a.6.9.4): the "Before any
assumption" block, then `Assuming …`, then the hypothesis's own steps, with
step numbers running as one sequence. Nothing compares that against ein.py any
more, and it is the half of the trace idea-08 is about.

### Task T1a.6.11.2 — The walkthrough-rule assertion, ported

`ein.py/tests/trace/test_idea08_acceptance.py::test_zebra2_fires_walkthrough_rules`
asserts that a solution's proof exhibits the nine rules
[`zebra_walkthrough.md`](../../../docs/kernel/inference/zebra_walkthrough.md)
narrates. It is the test that caught the near-miss in S1a.6.9 — the resumed
fork dropped `symmetric` out of the solution's firing list, because
`symmetric` fires only at root — and it exists only on the ein.py side.

Port it: an ein.rs test that renders the trace and asserts the rule set, so
the next change to the fork boundary or the renderer meets the same alarm.
This is the acceptance criterion of
[`08-human-style-deductive-trace`](../../ideas/08-human-style-deductive-trace.md)
and it should not be an ein.py-only guarantee once ein.rs is what ships.

### Task T1a.6.11.3 — The `slice` DOT view

`dot_parity.rs` compares seventeen DOT views of every corpus entry byte for
byte against ein.py. One of them — **`slice`**, the provenance cone — renders
a *derivation*, so it moved with D3 on 16 entries and is now in that test's
`NARRATION` list: still run on both sides, still required to answer, no longer
byte-compared. It needs an ein.rs golden of its own, for the same reason the
trace does.

### Task T1a.6.11.4 — Event-stream goldens

One `--events --events-level verbose` golden per shape, kept small on purpose:
a fixture whose stream is thousands of lines is a golden nobody reads and
everybody regenerates. Pick from `examples/features/` and `examples/branching/`,
not from the zebra puzzles.

What the goldens are *for* is the elided half of
[S1a.6.10](s1a.6.10_parity_contract.md)'s normalisation: the redundant firings
and the enqueue traffic, which the relaxed T2 no longer compares between the
engines and which nothing else would notice changing.

### Task T1a.6.11.5 — Wire them into the gate

`./run_tests.sh` and the Rust suite both, with a documented regeneration
command (`EIN_BLESS=1 cargo test -p ein-render`), because a golden without one
gets edited by hand and drifts.

## Outcome — shipped 2026-08-19

Every artefact [S1a.6.10](s1a.6.10_parity_contract.md) took out of the
cross-engine diff — and the three S1a.6.9 had already taken out — now has an
ein.rs fixture that fails if it changes. **Twelve goldens, 2 188 lines**, none
over 318, so a regeneration is reviewable.

| golden | what it pins | lines |
|---|---|---:|
| `ein-render/tests/golden/trace_{unconditional,one-hypothesis,ambiguous,two-level,unsat}.md` | five real solves, one per trace shape, DOT blocks replaced by a marker | 185–245 |
| `ein-render/tests/golden/slice_{forall,two-level}.dot` | the provenance cone, for two of the sixteen entries whose `slice` view D3 moves | 37, 313 |
| `ein-render/tests/golden/dump_enterings_subset-pruned.txt` | a fork's **own** dump — the firing list, the state dump in the fork's derivation order, a dying fork's core — which `dump_shape` elides where it is produced and the diff therefore never sees; plus the timeline's per-entering `firings` count, the one field of `00_timeline.jsonl` that is blanked rather than elided | 318 |
| `ein-render/tests/golden/dump_snapshot_subset-pruned.txt` | the snapshot's dead `state_key`s and both lattice DOTs the DAG merges by them | 33 |
| `ein-infer/tests/golden/events_{symmetric-native,naf-boundary,unconditional}.jsonl` | the `--events` stream at `verbose`, covering **every** class the relaxed T2 elides | 41–277 |

### The three that are not goldens

A byte golden can be blessed away, so the properties that must never be
blessed away are asserted separately:

- **`the_root_section_is_rendered_and_numbered_as_one_sequence`** — an
  unconditional solve narrates its 23 root steps; a hypothesis's steps
  *continue* root's numbering (16 + 4 = steps 1…20, not 1…16 then 1…4); and
  the root section comes before `Assuming …`. This is the shape
  [T1a.6.9.4](s1a.6.9_fork_entry_delta.md) added and the reason the fork
  boundary could move at all.
- **`between_them_the_goldens_cover_every_elided_class`** — the event goldens
  must contain a `park`, an `admit`, a `retire`, an `alt`, a `quiesce`, an
  `enqueue`, a `compile`, a `mirror` and a redundant firing, iterating
  `ein_parity::events::SCHEDULING` itself. Otherwise they are three more files
  that happen to be stable rather than coverage of the elided half. Choosing
  the fixtures *was* this assertion: `04_two_levels` emits 1 498 events and no
  `park` at all, and `12_typed_blind_solve` turned out to be the only small
  entry that emits the whole scheduling vocabulary.
- **`dot_parity`'s `NARRATED_SLICES`** — the sixteen entries whose `slice`
  view actually diverges, asserted rather than tolerated, so a cone that
  starts differing for an unrelated reason cannot hide behind the cut. That
  was S1a.6.10's Notes asking for the `DIVERGENT` discipline to survive the
  relaxation; it did.

### idea-08, on the engine that ships

`ein-render/tests/idea08_acceptance.rs` ports
`test_idea08_acceptance.py` in full — both puzzles, both levels — and asserts
against the **rendered markdown** rather than the firing list, because since
S1a.6.9 root's steps are a section of the document and not part of the
solution node's firings. `symmetric` gets its own assertion beside the set
one: it is the rule the near-miss dropped, and "one of nine missing" is a
worse failure message than naming it. Both puzzles run unconditionally — the
Python original gates the firing half behind `EIN_RUN_SLOW` because exhaustive
zebra2 costs 35 s on CPython; on ein.rs the whole file is 0.15 s.

### The gate

`./run_tests.sh` gained a **Phase 3**: `cargo test --workspace`, after the
pytest suite and the acceptance gate, skipped by `--fast` and by `--no-rust`,
and **loudly** skipped when there is no cargo on `PATH` — a skipped phase that
reads as green is how a gate stops being one. That is not tidiness: with the
parity harness no longer diffing ein.rs's narration against ein.py's, a gate
that runs only the Python half no longer covers the trace, the cone or the
event stream at all. Regeneration is one documented command,
`EIN_BLESS=1 cargo test --workspace`, implemented once in
`ein_oracle::golden`.

**The gates, run:** `./run_tests.sh` → **1 506** unit + **21** acceptance +
**302** ein.rs tests, exit 0.

## Acceptance

- Every artefact removed from the cross-engine diff has an ein.rs fixture that
  would fail if it changed — S1a.6.10's, and the three already taken out by
  S1a.6.9 to keep the suite green: `dot_parity`'s `NARRATION` list and
  `hypgen_parity`'s `Compare::IgnoringForkNarration`.
- The idea-08 walkthrough-rule assertion runs on ein.rs.
- Regenerating is one documented command, and the diff of a regeneration is
  reviewable — no golden larger than a few hundred lines.

## Notes

- This is deliberately *after* [S1a.6.10](s1a.6.10_parity_contract.md) and not
  merged into it: one stage relaxes a comparison, the next replaces it, and
  keeping them apart means the gap between them is visible in the history
  rather than hidden inside one commit.
