//! `ein` — the drop-in binary.
//!
//! A thin shell over [`ein_cli::run`]: the surface lives in the library so the
//! help-content check can *introspect* the parsers rather than scrape their
//! output (T1a.5.4.8).

/// The engine allocates ~880 k times for an exhaustive `zebra2` and ~1.67 M
/// for `zebra`, at ~71 bytes each, and glibc `malloc` was **20.0 %** of the
/// first one's self time (T1a.6.2.7). These are the four lines that buy back
/// −15.9 % and −7.5 % end-to-end on the two exhaustive puzzles, for +1.2 MB of
/// peak RSS on the largest cell and +0.5 ms of process start-up.
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
    ein_cli::run()
}
