# L-M2.31 — EinAf's recorded model runs carry the library's digest, so a change to `stdlib/` fails three of its tests

**Status:** open
**Found:** by [S2.2e.2](../../s2.2e.2_slots_neighbours.md), 2026-11-27, running EinAf's tests on its change of
`std.slots` (the first change to `stdlib/` since the fixtures were recorded, 2026-11-26)
**Owner:** none; the coordinator's and the user's, before the phase's gate (the phase's default 5 has EinAf follow
in the same batch, and says nothing of how a recording follows)

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
