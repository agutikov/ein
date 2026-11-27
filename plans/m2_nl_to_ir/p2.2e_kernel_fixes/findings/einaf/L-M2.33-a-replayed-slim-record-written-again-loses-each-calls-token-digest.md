# L-M2.33 — A slim record replayed and written again loses each call's token digest

**Status:** open
**Found:** by [S2.2e.5](../../s2.2e.5_recordings_follow_the_library.md), 2026-11-27, checking its new replay test's
mechanics on a local copy of `einaf/tests/fixtures/from_text/zebra-contradiction/` (not committed)
**Owner:** none in this phase; the default, [S2.5.1](../../../p2.5_harness/s2.5.1_experiment_record.md), which takes
over or replaces the stand-in record `einaf from-text --record` writes

## What

**`einaf from-text --replay generations.jsonl --record DIR` writes each call's `tokens_sha256` as the digest of no
tokens.** A slim call (`einaf::model::Slim`) keeps no token ids, only their digest; replay answers a request with that
generation as it stands, `tokens: []` and `tokens_sha256` the recorded digest; and `Recorded::slim`, which writes the
record's `generations.jsonl` again, takes the ids (none) and writes their digest over the one the generation carried:

```rust
let ids = std::mem::take(&mut g.envelope.generation.tokens);
g.envelope.generation.tokens_sha256 = Some(tokens_digest(&ids));
```

On the fixture's three calls replayed (`einaf/target/gate/einaf`, the stage's working tree):

| call | `tokens_sha256` in the record replayed | in the record the replay wrote |
|---|---|---|
| step 1 | `sha256:c5899d76…` | `sha256:e3b0c442…` (SHA-256 of the empty text) |
| step 2 | `sha256:0a506ff7…` | `sha256:e3b0c442…` |
| step 3 | `sha256:b42c02b2…` | `sha256:e3b0c442…` |

Nothing else of the call moves: the counts, the stop, the answer, the envelope's other fields. The program, the
sidecar and the answer replay as they were, and no test reads a replayed record's `generations.jsonl`.

## What it costs

A record written by a replay is not the record replayed: a comparison of the two by bytes (P2.2's
`from_text/replay.sh` writes `replayed/…`, which git ignores) finds every call changed, and a replayed record has
lost what pins the model's tokens. The fix is EinAf's and small (keep a digest the generation carries where it carries
no ids); whether a replayed record is held to the record it replays is the experiment record's to say.
