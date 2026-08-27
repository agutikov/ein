# S1e.3.5 — Error handling (Medium)

**Phase:** [P1e.3](README.md) (Medium)
**Estimate:** 1.5 days
**Depends on:** nothing. Related to
[CO-M5](s1e.3.1_correctness.md) — the same stdlib-resolution tier.
**Findings:** [`EH-M1`](../review/error-handling/medium.md),
[`EH-M2`](../review/error-handling/medium.md).

## Context

Two findings about **silence where a machine consumer is listening**.

`EH-M1` — a failed `--events` open, a failed `--json-summary` write and a
failed `--trace` write each print one stderr line and the run exits as if the
artefact existed
([`ein-cli/src/solve.rs:314-319, 613-618, 699-705`](../../../ein.rs/crates/ein-cli/src/solve.rs),
[`test.rs:489-493, 809-815`](../../../ein.rs/crates/ein-cli/src/test.rs)). A
pipeline that asked for `--json-summary` gets **exit 0 with no file** on an
unwritable path. The corpus tests only assert that artefacts appear on
*successful* writes.

There is a real argument for the current behaviour and the review states it:
these flags are *additive*, and the strictest reading of additive is that the
exit code is unchanged by their presence or their failure. Against that: the
surfaces M1d built — the summary, the report, the event stream — exist
**specifically for machine consumption**, and the one signal such a consumer
gets is a stderr line it is probably not reading.

`EH-M2` — `$EIN_STDLIB` is accepted with **no check at all**: no
`MANIFEST.sha256`, no existence test
([`stdlib.rs:113-136`](../../../ein.rs/crates/ein-ir/src/stdlib.rs)), while
the checkout walk *requires* the marker. The marker exists precisely because
*a directory called `stdlib/` proves nothing* (`stdlib.rs:32-34`) — and the
highest-precedence source skips the proof. A typo'd or stale override yields
*"module not found at &lt;path&gt;"* once per import instead of one root-cause
error. Additionally `resolve_default()` walks from `current_exe()`, so a
binary copied under an unrelated checkout containing `stdlib/MANIFEST.sha256`
silently prefers that tree.

## Acceptance

- `EH-M1` is **ruled on deliberately** and the ruling is written into
  [`defined_behaviour.md` § 4](../../../docs/kernel/defined_behaviour.md)
  beside the exit codes, with a test pinning whichever contract is chosen.
  *"It is arguably fine"* is not a disposition.
- `EH-M2`: the override path requires at least existence, and preferably the
  marker, with a single readable refusal that **names the environment
  variable** — because the diagnosis cost is the finding, not the wrong
  answer.
- The three-tier resolution order (`$EIN_STDLIB` → checkout → embedded) has a
  test per tier. Today the harness always sets the env var, so two tiers are
  untested and the release binary uses one of them.

## Tasks

### Task T1e.3.5.1 — `EH-M1`: rule on artefact-write failure

Two coherent contracts, and the stage picks one:

| contract | what it means | what it costs |
|---|---|---|
| **a distinct exit code** — the run succeeded but a requested artefact could not be written | a pipeline learns from the exit code alone | a fourth meaning for an exit code, in a tool where 2 already means two things ([TE-M4](s1e.3.6_tests.md)) |
| **exit 0 with a stderr line**, documented as the contract | strictly additive, matches today's behaviour | a consumer must read stderr, which is exactly what a JSON-consuming pipeline does not do |

The first is cleaner in principle and collides with a real constraint: exit 2
already means *usage error* and *budget abort*, and adding a third code is a
CLI-vocabulary change with its own blast radius. A middle option worth
considering and rejecting explicitly: fail **only** when the artefact is the
run's evident purpose — no, because *evident purpose* is not a property the
CLI can compute.

Recommendation: **document exit-0-with-stderr as the contract, and pin it**,
plus make the stderr line unambiguous (the flag, the path, the OS error). Then
file the exit-code question as a `Q-M1e.<n>` for whoever owns the CLI
vocabulary next, with `TE-M4`'s exit-2 overload attached — the two are one
conversation and neither should be settled alone.

Whichever is chosen, add the test: an unwritable path (a directory, or a path
under a read-only dir) for each of the three artefacts, asserting the exit
code and the stderr shape.

### Task T1e.3.5.2 — `EH-M2`: make the override prove itself

Require the marker on the `$EIN_STDLIB` path, with one refusal that names the
variable, the path, and what was missing. The message matters more than the
check here: the failure mode being fixed is *N confusing errors instead of
one clear one*, so a check that produces a terse error is half a fix.

Then decide the `current_exe()` walk. A binary copied under an unrelated
checkout that happens to contain `stdlib/MANIFEST.sha256` silently prefers
that tree — which is a genuine hazard for the release channel and a
convenience for development. Options: keep it and document it in
[`docs/install.md`](../../../docs/install.md) (which already explains the
resolution order and is where a user would look); bound it to a fixed number
of parent levels; or require the manifest's SHA to match the binary's
embedded one, warning on mismatch. The third is the strongest and
`ein --version` already prints both halves, so the comparison exists.

### Task T1e.3.5.3 — A test per resolution tier

The harness sets `$EIN_STDLIB` unconditionally (`stdlib.rs:183`), so the
checkout walk and the embedded copy are exercised by **no test** — while the
embedded copy is what an installed binary uses. Three tests:

1. `$EIN_STDLIB` set to a valid tree — the current path, plus the new refusal
   cases from T1e.3.5.2.
2. Unset, with a checkout containing the marker — the walk finds it.
3. Unset, with the walk defeated (a temp cwd, a copied binary) — the embedded
   copy loads and a program that imports `std.*` solves.

The third is the one that would have caught [CO-M5](s1e.3.1_correctness.md)'s
embedded-root identity degradation, which is why that finding's fix names the
same test. Write it once here and let `CO-M5` cite it.

## Notes

Both findings are in the same neighbourhood as a third the review filed
elsewhere: [EH-L2](../p1e.4_low/s1e.4.4_error_handling.md), where a
non-`einb` build sniffs 5 magic bytes and `is_einb` requires 8. All three are
about **what the CLI does when the world is not as expected**, and all three
are cheap. If the stage runs short, pulling `EH-L2` forward costs an hour and
saves a context switch.
