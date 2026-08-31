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

### Task T1e.3.5.1 — `EH-M1`: rule on artefact-write failure ✅

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

### Task T1e.3.5.2 — `EH-M2`: make the override prove itself ✅

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

### Task T1e.3.5.3 — A test per resolution tier ✅

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

---

## Outcome

Taken 2026-08-31.

| | |
|---|---|
| **`EH-M1`** | **ruled**, the additive way the stage recommended, and written into [`defined_behaviour.md` § 4.4](../../../docs/kernel/defined_behaviour.md) with the reasoning. A failed artefact write leaves the exit code alone; `--dump-states` is the exception and says why at its call site. Pinned by `ein-cli/tests/artefact_contract.rs`, three tests over all **six** flag × subcommand pairs |
| the message | **one shape**, `error: --<flag> <path>: <os error>`. There were four: three bare OS errors (*Is a directory (os error 21)* — on a run that may carry three artefact flags), one that named the path and not the flag, and one that carried its failure only in the exit code |
| **the defect the finding did not name** | an **empty path** reached all five options, and `--dump-states ""` *succeeded*: `create_dir_all("")` is `Ok`, so the run dropped `00_root_initial.ein`, `00_timeline.jsonl`, `summary.json` and `layers/` into the caller's working directory. Refused now at the value parser, exit 2, for `--solutions 0`'s reason |
| **`EH-M2`** | **fixed**: `$EIN_STDLIB` must carry `MANIFEST.sha256`, the same marker the checkout walk requires, and the refusal names the variable, the path and what is missing. Asked at the **first `std.*` import**, so a program that imports nothing from the stdlib is not refused for the shape of a variable it never reads |
| the `current_exe()` walk | **kept, with a written reason** — [`docs/install.md`](../../../docs/install.md). The stage's own preferred guard (warn when the resolved manifest differs from the embedded copy) is **refused**, and the reason is that the mismatch is the *normal* state of stdlib development: the checkout tier exists so an edited module takes effect with no rebuild, so the warning would fire on the working case |
| **the three tiers** | one test each, and none of them skips. `resolve` is now `resolve_with(from, override)` plus one line that reads the environment, so a test drives every tier as a pure function |
| filed | [Q-M1e.22](../open_questions.md#q-m1e22--should-a-failed-artefact-write-have-an-exit-code-of-its-own) — should the additive arm have an exit code of its own — with `TE-M4`'s exit-2 overload attached, as the stage instructed |
| gate | `./run_tests.sh` green — **792 tests**. No golden moved: nothing in the corpus reaches an unwritable artefact path or a markerless override |

### Three things the tasks did not predict

**1. The stage's premise about the harness was false, and it mattered.**
T1e.3.5.3 says *"the harness sets `$EIN_STDLIB` unconditionally
(`stdlib.rs:183`), so the checkout walk and the embedded copy are exercised by
no test."* Line 183 is not a setter — it is a **guard inside a test**, `if
std::env::var_os("EIN_STDLIB").is_some() { return; }`, and the variable is
unset in the gate. So tier 2 was running all along. What is true, and is the
worse version of the same finding, is that two tier tests were written to
*answer nothing* under a configuration somebody believed was the normal one:
[TE-M1](s1e.3.6_tests.md)'s shape, in the file the review was reading. Both are
unconditional now, driven through `resolve_with`.

**2. The four diagnostics were the finding, not the exit code.** `EH-M1` is
written as *the exit code does not change*, and the exit code turns out to be
the part with a defensible answer on both sides — which is why it is filed
rather than settled. What had no defence was that a run carrying `--events`,
`--trace` and `--json-summary` printed `Is a directory (os error 21)` and left
the reader to guess which of the three it was about.

**3. `an_override_is_honoured_whatever_it_points_at` argued against itself.**
The test that had to move for `EH-M2` opened with *"a directory called
`stdlib/` proves nothing — `MANIFEST.sha256` is what identifies the checkout
during the walk"* and concluded from it that the **highest-precedence** source
need not carry the marker. The first clause is the reason the marker exists.
The test now asserts the refusal, and keeps the half that was always right:
the resolved root is the only one consulted, with no quiet fall-back.

### What this stage did **not** do

- **Take `EH-L2`.** The Notes offer it — *"if the stage runs short, pulling it
  forward costs an hour"* — and the stage did not run short. It stays
  [P1e.4](../p1e.4_low/s1e.4.4_error_handling.md)'s.
- **Give `--dump-states` the additive contract.** Making it exit 0 for
  consistency would have thrown away the one exit-code signal the family has,
  on the one flag whose failure is discovered *before* the run and whose
  `--help` claims no additivity. Two arms with a stated reason beat one arm
  that loses a signal.
- **Refuse an unprovable `$EIN_STDLIB` at `ein --version`.** That line reports
  the manifest as `unreadable` and keeps printing, which is how a user finds
  out what their binary will load; a version line that refused to render would
  be a worse way to learn the same thing.
