# Q-M1f.101 — Off Unix, a closed stdout is left to Rust's runtime

Off Unix, `restore_sigpipe` does nothing. Windows has no `SIGPIPE`: a write to a pipe whose reader has gone is an
error there, which a `print!` turns into a panic, exit 101. Ein routes no write of stdout through a writer of its own
to stop quietly instead.

**Why.** What [Q-M1f.68](Q-M1f.68-a-closed-stdout-ends-ein-by-sigpipe.md) settles is the signal's disposition, and a
platform without the signal has none to restore. A quiet stop would need every write of stdout to go through one
writer, which is a different design, not the same one on another platform.

**Held by.** `ein-cli/src/main.rs`: `restore_sigpipe` under `#[cfg(not(unix))]`, an empty function.
`ein-cli/tests/closed_stdout.rs` is `#![cfg(unix)]`, so nothing holds a Windows run to a behaviour.
