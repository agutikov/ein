# S1a.6.10 — The parity contract relaxes: answers, not narration

**Phase:** P1a.6 (Performance)
**Estimate:** 2 days
**Depends on:** [S1a.6.9](s1a.6.9_fork_entry_delta.md) — which produced the
divergence this stage teaches the harness to hold, and the measurement that
says it is safe to.
**Answers:** the second half of
[Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
— the decision was taken; this is the mechanism.
**Successor:** [S1a.6.11](s1a.6.11_fixture_goldens.md) — the narration that
stops being compared against ein.py has to be compared against *something*.

## Context

Through P1a.5 the parity contract was byte-identical everything, and that was
right: a port is only falsifiable against an oracle, and the cheapest oracle
is "the same bytes". [P1a.5](../p1a.5_presentation/README.md) closed that gate
with the ledger at two entries, both input-specific.

[S1a.6.9](s1a.6.9_fork_entry_delta.md) is the first change where byte-identical
narration and a *better engine* pull apart. A fork that resumes root's
saturation instead of re-deriving it does 74–77 % fewer firings, meets the
milestone's last unmet target, and produces a trace that starts where a human
walkthrough starts. It also narrates a quarter of what it used to, so the
cells that compare a firing list report a difference —
[D3](../divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it).

**Measured, because "the harness goes red" is not a scope.** T3 goes from
472/473 to **465/473**: seven cells, and they are exactly the seven corpus
entries that declare a `solve --trace` or a `solve --dump-states` run. Every
other T3 cell — 332 `positive`, all of `stdlib`, `load-negative`,
`parse-negative` — is still byte-identical, because stdout, `summary.json`
and everything else a solve writes did not move. T2 costs **97 cells of 240** — larger by
construction, since it compares the event stream and that is where three
quarters of the lines went. Both numbers are in
[baseline.md §11](baseline.md#11-the-resumed-fork-saturator-measured).

**The standard moves to where it was always pointed.** ein.rs and ein.py must
agree on the *answer* — the verdict, `k`, the models, the query bindings, the
unsat core, and every counter in `summary.json`, which is T0 and T1 in full.
They no longer have to agree on how much of the derivation each one narrates.
That is not a weakening of the M1a contract so much as a naming of what the
contract was for: `utils/fork_delta_verify.py` compares 1.06 M enterings fact
by fact and finds the answers identical while the firing lists differ by three
quarters.

**What this stage is not.** It is not a licence to stop comparing. T0 and T1
stay exact on every cell, T3 stays exact on everything that is not a firing
list, and the narration that leaves the diff is picked up by
[S1a.6.11](s1a.6.11_fixture_goldens.md) as ein.rs fixtures — checked in,
diffed against themselves, so a regression in the trace is still a failing
test rather than a shrug.

## Tasks

### Task T1a.6.10.0 — Replace the five ad-hoc cuts with one rule

S1a.6.9 shipped with six separate relaxations, made one at a time as each
test went red: `hypgen_parity`'s `Compare::IgnoringForkNarration`,
`dot_parity`'s `NARRATION` list, `trace_parity`'s `NARRATION_BLOCKS`, and
three in `dump_shape` and its `ir_oracle.py` twin. They are tabulated in
[D3](../divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it),
and the chain they form is this stage's specification, because read downwards
it is a single sentence:

> **A fork's derivation, and anything keyed on a dying fork's stopping point,
> is narration.**

Write that once, in [design/01 §5](../design/01_parity_contract.md), implement
it once, and delete the six. A relaxation that has to be discovered by
running the tests is not a contract.

### Task T1a.6.10.1 — The normalisation row

[design/01 §5](../design/01_parity_contract.md) is the closed list of
legitimate divergences, and adding to it "requires an entry in
`open_questions.md`" — which
[Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
is. Add the row, with the normalisation stated precisely enough to implement:

| what | why | normalisation |
|---|---|---|
| the **firing lists** of a solve: `fire` / `enqueue` / `park` / `admit` / `retire` / `alt` / `quiesce` events, `n_firings` in `--trace`, `("firings", len)` in `--dump-states`, and `render/shape.rs`'s first five | ein.rs resumes root's saturation across the fork boundary and ein.py re-derives it ([D3](../divergences.md)) | compared **for the productive subsequence only**: the ordered list of `(rule, premises, derived)` for firings with `redundant = false`, which is the derivation both engines actually perform. Redundant firings and the enqueue traffic are elided |

The productive subsequence is the right cut and it is not a guess: S1a.6.9
measured it identical — 6 136 → 6 136 on `zebra -e`, and +9 on `zebra2 -e`
where fail-fast stops a dying fork at a different firing. Eliding the whole
event stream would be the lazy cut and would stop catching a port that
silently stopped deriving something.

### Task T1a.6.10.2 — The harness

`ein-conformance`'s `tier.rs` gains the normalisation:

- **T2** — `event_diff` filters both streams to the productive subsequence
  before comparing, and reports the elided counts per side so a run still says
  *how much* narration each engine produced. A difference in the productive
  subsequence is still a T2 failure.
- **T3** — the file comparison normalises `("firings", n)` and `n_firings` out
  of `--trace` / `--dump-states` artefacts, exactly as
  `utils/fork_delta_verify.py` already does, and compares everything else byte
  for byte. `summary.json` is **not** normalised: it is T0 + T1 and it does
  not move.
- The `--strict` flag turns all of it off, so the old contract is one flag
  away and the determinism sweep (ein.py against itself, two hash seeds) still
  runs under it.

`conformance/README.md`'s tier paragraph and `EVENTS.md` § Levels both say
what the tiers compare, and both are now wrong; they move with the code.

### Task T1a.6.10.3 — Re-run the gate and record the number

T3 back to green on the whole corpus with [D2](../divergences.md) the only
cell, as it was before S1a.6.9. The before number — how many cells D3 costs
without this stage — is recorded in
[baseline.md §11](baseline.md#11-the-resumed-fork-saturator-measured) so the
stage is judged against it rather than against "it passes now".

Also re-run the determinism sweep under `--strict`: ein.py against itself with
`PYTHONHASHSEED=0` / `=42`, which is how hazards H1 and H4 were found and
which must keep working on the *unrelaxed* contract.

## Acceptance

- The `--tier T3` run is green with D2 the only differing cell.
- `--strict` reproduces the old, byte-identical comparison, and the
  determinism sweep passes under it.
- A deliberately broken ein.rs — one productive firing dropped — is **caught**
  at T2 under the relaxed comparison. A relaxation that cannot be shown to
  still catch the thing it was relaxed around is a hole, not a decision.
- design/01 §5, `conformance/README.md` and `EVENTS.md` agree with the code.

## Notes

- The reason the row goes on the normalisation list rather than staying a
  ledger entry: [divergences.md](../divergences.md) is for differences tied to
  an *input*, each with a fixture that demonstrates it. D3 is tied to no
  input — it is every solve with more than one entering — so it has no
  fixture to point at, and a ledger entry that cannot be demonstrated by a
  file is exactly the shrug rule 1 forbids. D3 stays as the written decision;
  the harness stops reporting it as 300-odd failing cells.
- The six cuts S1a.6.9 left behind are **tolerances, not assertions**, and
  T1a.6.10.0 should fix that too: `dot_parity`'s `NARRATION` skips the byte
  check for the `slice` view of *every* entry, where the same file's
  `DIVERGENT` discipline asserts the exact set that diverges — "a file listed
  here that stops diverging fails as loudly as one that starts". The relaxed
  comparison should keep that discipline, or a slice that starts differing for
  an unrelated reason goes unseen.
- **Do not** relax T0 or T1 in any direction. The answer and the search are
  the contract now, and they are compared *more* carefully than before —
  `fork_delta_verify.py` added `summary.json` to its own comparison for
  exactly that reason.
