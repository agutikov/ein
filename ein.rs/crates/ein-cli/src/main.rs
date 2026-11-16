//! `ein` — the binary.
//!
//! A thin shell over [`ein_cli::run`]: the surface lives in the library so the
//! help-content check can *introspect* the parsers rather than scrape their
//! output.
//!
//! Two process-wide choices live here and nowhere else, because a library that
//! made either would make it for every program that links it: the global
//! allocator, and what a write to a pipe whose reader has gone does
//! ([`restore_sigpipe`]). This file is its own crate root, so `lib.rs`'s
//! `#![forbid(unsafe_code)]` does not reach it; the one `unsafe` block here is
//! that second choice, audited below.

/// The engine allocates in enormous counts and tiny sizes, which is what a
/// per-thread-cache allocator is for; these four lines are what buys it.
///
/// It lives on the **binary**, not on an engine crate: a library that installs
/// a global allocator makes the choice for every program that links it, and
/// `ein-core` / `ein-infer` are meant to be embedded. The bench target
/// declares the same one, so `cargo bench` measures the shipped program
/// (`ein.rs/crates/ein-corpus/benches/engine.rs`).
#[cfg(feature = "snmalloc")]
#[global_allocator]
static GLOBAL: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

fn main() -> std::process::ExitCode {
    restore_sigpipe();
    ein_cli::run()
}

/// **A closed stdout ends `ein` by `SIGPIPE`, as it ends `cat`** (Q-M1f.68).
///
/// Rust's runtime sets `SIGPIPE` to *ignored* before `main` runs. A write to a
/// pipe whose reader has exited then fails with `EPIPE` instead of ending the
/// process, and `print!` / `println!` panic on a failed write: `ein saturate
/// … | head` would print *failed printing to stdout: Broken pipe (os error
/// 32)* and exit 101, and whether a command met it would be a race between
/// its writes and the reader's exit (W75).
/// Restoring the default disposition makes the kernel end the process at that
/// write: nothing is printed, and the shell reports 141, 128 + 13, as it does
/// for `cat`, `grep` and `head`, so `set -o pipefail` reads `ein` as it reads
/// them. The process never reaches an exit code of its own, so *exit non-zero
/// only on error*, a rule about the codes Ein returns, is not engaged.
///
/// It is the binary's, as the allocator is: a disposition is process-wide, and
/// a program that embeds the engine keeps its own (`docs/ein/ein.rs/api.md`). It
/// covers every pipe the process writes, an artefact flag's FIFO or
/// `/dev/stdout` too (`docs/ein/user/reference/diagnostics.md` § 2.19).
/// Rust 1.97.1, the pinned
/// toolchain, has no stable option for it, so it is one `libc` call, the
/// second audited `unsafe` site beside `ein-einb`'s `cast.rs`. Pinned by
/// `ein-cli/tests/closed_stdout.rs`.
#[cfg(unix)]
fn restore_sigpipe() {
    // SAFETY: `signal` is called once, as the first statement of `main`,
    // before the program starts a thread or installs a handler, so nothing
    // can race the change or observe a half-made one. It installs `SIG_DFL`,
    // the kernel's own disposition, which runs no code of ours. Its argument
    // is a valid signal number and a valid disposition, and its result, the
    // previous disposition (`SIG_IGN`, Rust's), is not needed; `SIG_ERR`
    // cannot come back for those two arguments.
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

/// Windows has no `SIGPIPE`: a write to a closed pipe is an error there.
#[cfg(not(unix))]
fn restore_sigpipe() {}
