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
