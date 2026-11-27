# L-M2.31 — EinAf's recorded model runs carry the library's digest, so a change to `stdlib/` fails three of its tests

**Status:** closed by [S2.2e.5](../../s2.2e.5_recordings_follow_the_library.md) (`8e3c6f4a`, `969ae30d`): the
catalogue a prompt shows names no digest of the library, the record does, and the two runs are recorded again
**Found:** by [S2.2e.2](../../s2.2e.2_slots_neighbours.md), 2026-11-27, running EinAf's tests on its change of
`std.slots` (the first change to `stdlib/` since the fixtures were recorded, 2026-11-26)
**Owner:** [S2.2e.5](../../s2.2e.5_recordings_follow_the_library.md), on the user's choice (2026-11-27), *"2"*, of the
three ways the coordinator offered: (1) record the runs again with the model after every change of the library;
(2) take the digest out of what the model sees, into the record, and record once again; (3) let a replay tolerate a
difference of the digest alone.

## What

**Three of EinAf's tests replay a recorded model run, and each record holds the library's manifest digest**, so any
change to `stdlib/`, whatever it changes, fails them; on the library before the change all three pass (checked with
`EIN_STDLIB` set to a copy of `0ab06c69`'s `stdlib/`):

| test | the fixture | what differs |
|---|---|---|
| `from_text.rs`, `a_recorded_run_of_the_model_replays_step_by_step_from_its_slim_record` | `einaf/tests/fixtures/from_text/zebra-contradiction/` | step 2's request: its prompt shows the catalogue, whose first line is *The library is sha256:…* (`Catalogue::text`), so the request's digest is not the recorded call's, `not-recorded`; the run then stops at step 2, *No answer: the program was refused* |
| `steps.rs`, `a_recorded_run_replays_to_the_run_it_recorded` | `einaf/tests/fixtures/steps/bob1/` | the same: two recorded requests quote *The library is sha256:292894…* |
| `model.rs`, `a_probe_line_replayed_is_judged_as_it_was` | `einaf/tests/fixtures/model/probe_lines.jsonl` | only the judged feedback's `kernel.stdlib`, twice: the request (library off) still finds its call |

## What it costs

The phase's gate is red on EinAf's step until the three follow. The probe line can be judged again from its own
record, the model's generation replayed (`einaf probe run --replay`), which writes the new digest and nothing else.
The other two cannot be replayed at all: their prompts changed, so what the model answers them is unknown until it is
asked, and writing the new digests over the recorded calls would make a record of prompts the model never saw. The
ways out are a recording again with the model (`--features cuda`, the GGUF), which may change what the fixtures show,
or a change of how EinAf's tests replay a record made against another library; either is the user's or the
coordinator's to choose, and the stage changed nothing of EinAf.

## How it was closed

By the user's second way, in [S2.2e.5](../../s2.2e.5_recordings_follow_the_library.md):

- **The digest out of what the model sees** (`8e3c6f4a`): `Catalogue::text` begins *A program selects a theory by
  importing its module, …*, the rest of its sentence word for word; the library's digest is the record's, a run's
  `stdlib` (`formalize::Run`), each step's beside its template's digest (`formalize::StepRun`), and the one shot's
  `config.json`. No template's text changed; the change moves the catalogue's digest and each rendered prompt's, a new
  experiment by `einaf::prompts`' rule.
- **A replay survives a change of the library that leaves the catalogue's text alone**, held by `from_text.rs`'
  `a_recorded_run_replays_under_a_library_whose_digest_moved_and_whose_catalogue_did_not` (a copy of `stdlib/` with a
  comment added to `std.math.gcd` and its manifest written again, under `$EIN_STDLIB`).
- **The probe line** judged again from its own record with no model (`8e3c6f4a`): `kernel.stdlib` and the judge's own
  times moved, nothing else.
- **The two runs recorded again with the model**, once, as their plans made them (`969ae30d`). Step 1, which shows no
  catalogue, generated what it generated before, token for token, in both. `bob1`'s step 3 and `zebra-contradiction`'s
  steps 2 and 3 now write otherwise: `bob1` answers *Bob has (n z 5) apples.* (one row; it answered 1, 3 and 5;
  wrong against 3 either way), and `zebra-contradiction` still prints *The statements contradict each other.*, right,
  its root dying on a contradiction of the model's own as before (L-M2.22).

EinAf's tests: 101 passed, none failed, the three replays among them.
