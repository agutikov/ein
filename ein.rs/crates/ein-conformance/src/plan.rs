//! Run name → argv.
//!
//! A run name is the `ein` argv with the file position elided, so
//! `"solve -e"` reads as `ein solve <file> -e` and `"render rules"` as
//! `ein render rules <file>`. That keeps the manifest readable while staying
//! mechanical — the only two shapes are "subcommand takes the file first" and
//! `render`, whose view name comes before it.
//!
//! Two substitutions happen here:
//!
//! - `{out}` in any token expands to the run's own output directory, which is
//!   how `--trace {out}/trace.md` and `--dump-states {out}/states` name their
//!   artefacts without the manifest knowing where the harness writes.
//! - every `solve` run gains `--json-summary {out}/summary.json`, because the
//!   T0/T1 tiers read that file and there is no reason to make each manifest
//!   entry ask for it.

use std::path::Path;

/// The event log's name inside a run's output directory.
pub const EVENTS_FILE: &str = "events.jsonl";

/// Build the argv (after the implementation's own command prefix).
///
/// `events` adds `--events {out}/events.jsonl --events-level verbose` to the
/// subcommands that support it. Verbose, not normal: a dropped redundant
/// firing is exactly what a port loses, and T2 exists to catch it.
pub fn argv(run: &str, file: &str, out: &Path, events: bool) -> Vec<String> {
    let out = out.display().to_string();
    let toks: Vec<String> = run
        .split_whitespace()
        .map(|t| t.replace("{out}", &out))
        .collect();
    let mut argv: Vec<String> = Vec::with_capacity(toks.len() + 3);
    if toks.first().map(String::as_str) == Some("render") {
        // `render <view> <file> [flags…]`
        argv.push(toks[0].clone());
        if let Some(view) = toks.get(1) {
            argv.push(view.clone());
        }
        argv.push(file.to_string());
        argv.extend(toks.iter().skip(2).cloned());
    } else {
        argv.push(toks[0].clone());
        argv.push(file.to_string());
        argv.extend(toks.iter().skip(1).cloned());
    }
    if toks.first().map(String::as_str) == Some("solve") {
        argv.push("--json-summary".into());
        argv.push(format!("{out}/summary.json"));
    }
    if events && matches!(toks.first().map(String::as_str), Some("solve" | "saturate")) {
        argv.push("--events".into());
        argv.push(format!("{out}/{EVENTS_FILE}"));
        argv.push("--events-level".into());
        argv.push("verbose".into());
    }
    argv
}

/// A filesystem-safe slug for a run name or a path — `solve -e` →
/// `solve_-e`. Runs of separators collapse, so `{out}/trace.md` does not
/// leave a `__` scar where the braces were.
pub fn slug(run: &str) -> String {
    let mut out = String::with_capacity(run.len());
    for c in run.chars() {
        if c.is_ascii_alphanumeric() || c == '-' || c == '.' {
            out.push(c);
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_matches('_').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(run: &str) -> Vec<String> {
        argv(run, "examples/x.ein", Path::new("/o"), false)
    }

    fn ve(run: &str) -> Vec<String> {
        argv(run, "examples/x.ein", Path::new("/o"), true)
    }

    #[test]
    fn events_are_requested_only_where_they_exist() {
        assert!(ve("solve").contains(&"--events".to_string()));
        assert!(ve("saturate").contains(&"--events".to_string()));
        assert!(!ve("render rules").contains(&"--events".to_string()));
        assert!(!v("solve").contains(&"--events".to_string()));
        // T2 compares the algorithm, so it runs at the level that records it.
        assert!(ve("solve").contains(&"verbose".to_string()));
    }

    #[test]
    fn solve_takes_the_file_first_and_gains_a_summary() {
        assert_eq!(
            v("solve -e"),
            [
                "solve",
                "examples/x.ein",
                "-e",
                "--json-summary",
                "/o/summary.json"
            ]
        );
    }

    #[test]
    fn render_puts_the_view_before_the_file() {
        assert_eq!(v("render rules"), ["render", "rules", "examples/x.ein"]);
    }

    #[test]
    fn saturate_gets_no_summary() {
        assert_eq!(
            v("saturate --dump"),
            ["saturate", "examples/x.ein", "--dump"]
        );
    }

    #[test]
    fn out_expands_in_place() {
        assert_eq!(
            v("solve --trace {out}/trace.md"),
            [
                "solve",
                "examples/x.ein",
                "--trace",
                "/o/trace.md",
                "--json-summary",
                "/o/summary.json"
            ]
        );
    }

    #[test]
    fn slugs_are_filesystem_safe() {
        assert_eq!(slug("solve -e"), "solve_-e");
        assert_eq!(slug("render constraints"), "render_constraints");
        assert_eq!(
            slug("solve --trace {out}/trace.md"),
            "solve_--trace_out_trace.md"
        );
        assert_eq!(
            slug("examples/broken/load/x.ein"),
            "examples_broken_load_x.ein"
        );
    }
}
