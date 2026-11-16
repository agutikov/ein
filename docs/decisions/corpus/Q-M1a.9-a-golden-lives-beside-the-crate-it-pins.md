# Q-M1a.9 — A golden lives beside the crate it pins

A golden, a checked-in expected output, lives under `ein.rs/crates/<crate>/tests/golden/`, beside the crate whose
output it pins, and not in a tree shared by every crate. `ein_corpus::golden_path(krate, name)` is the whole of the
convention: a test names its crate and the golden's name.

**Why.** A golden is read by one test, of one crate, so it belongs where that crate's tests are; a shared tree would
put a file beside nothing that reads it.

**Held by.** `ein_corpus::golden_path` (`ein-corpus/src/lib.rs`); the goldens under `ein-cli/tests/golden/`,
`ein-core/tests/golden/`, `ein-ir/tests/golden/` and `ein-render/tests/golden/`.
