//! `ein` — the drop-in binary.
//!
//! A thin shell over [`ein_cli::run`]: the surface lives in the library so the
//! help-content check can *introspect* the parsers rather than scrape their
//! output (T1a.5.4.8).

fn main() -> std::process::ExitCode {
    ein_cli::run()
}
