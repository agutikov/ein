//! The four parity tiers — `plans/m1a_rust/design/01_parity_contract.md` §2.
//!
//! | tier | what is compared |
//! |---|---|
//! | T0 | the verdict: type, `k`, `exhausted`, the model, goal bindings, the unsat core, exit code |
//! | T1 | T0 + every counter the engine reports about its own work |
//! | T2 | T1 + the ordered event log — *lands with S1a.0.2* |
//! | T3 | T2 + byte-for-byte identical output artefacts |
//!
//! A tier subsumes the ones above it, and here that is mechanical rather than
//! a claim: T0 and T1 read `summary.json`, which T3 compares as one of the
//! run's produced files. So a T3 pass cannot hide a T1 difference.
//!
//! A run with no verdict (`render …`, `saturate …`) has nothing for T0/T1 to
//! read; those tiers compare its exit code and say so, rather than reporting
//! a green they did not earn.

use serde_json::Value;
use std::fmt;

use crate::run::Capture;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    T0,
    T1,
    T2,
    T3,
}

impl Tier {
    pub fn parse(s: &str) -> Result<Tier, String> {
        match s.to_ascii_uppercase().as_str() {
            "T0" => Ok(Tier::T0),
            "T1" => Ok(Tier::T1),
            "T2" => Ok(Tier::T2),
            "T3" => Ok(Tier::T3),
            _ => Err(format!("unknown tier {s:?} (expected T0…T3)")),
        }
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Tier::T0 => "T0",
            Tier::T1 => "T1",
            Tier::T2 => "T2",
            Tier::T3 => "T3",
        };
        f.write_str(s)
    }
}

/// The outcome of comparing one cell.
#[derive(Debug)]
pub enum Outcome {
    /// No observable difference at this tier.
    Same,
    /// Differences, most significant first.
    Diff(Vec<String>),
    /// The tier could not be evaluated (T2 before S1a.0.2, or a run that
    /// produces no verdict). Never counted as a pass.
    Skipped(String),
}

impl Outcome {
    pub fn is_diff(&self) -> bool {
        matches!(self, Outcome::Diff(_))
    }
}

/// A crash cell: the input makes an implementation die with an unhandled
/// exception. Compared by exit code and **exception class only**.
///
/// The message body is not comparable, and not merely because ein.rs has no
/// Python traceback. The `crash-parity` fixture found at S1a.0.1
/// (`examples/ein-bugs/mixed-type-hypothesis.ein`) raises
/// `TypeError: '<' not supported between instances of …`, and *which operand
/// is named first* depends on the `frozenset` iteration order inside `sorted`
/// — so ein.py alternates between two messages across `PYTHONHASHSEED`
/// values. Comparing "the first line of stderr", as Q-M1a.14 first proposed,
/// would make the determinism sweep fail on a difference that is not one.
pub fn compare_crash(a: &Capture, b: &Capture) -> Outcome {
    let mut diffs = Vec::new();
    if a.code != b.code {
        diffs.push(format!("exit code: a={} b={}", a.code, b.code));
    }
    let (ea, eb) = (exception_class(&a.stderr), exception_class(&b.stderr));
    if ea != eb {
        diffs.push(format!("exception: a={ea:?} b={eb:?}"));
    }
    if diffs.is_empty() {
        Outcome::Same
    } else {
        Outcome::Diff(diffs)
    }
}

/// The exception class from the last non-blank line of a traceback —
/// `TypeError: '<' not supported…` → `TypeError`. `None` when the stream does
/// not look like one, which is itself a comparable observable.
fn exception_class(stderr: &str) -> Option<String> {
    let last = stderr.lines().rev().find(|l| !l.trim().is_empty())?;
    let (name, _) = last.split_once(": ")?;
    let name = name.trim();
    // A dotted class name is `module.path.ClassName`, so the capital that
    // marks it as a class is on the last segment, not the first character.
    let last = name.rsplit('.').next().unwrap_or(name);
    let ok = !name.is_empty()
        && last.chars().next().is_some_and(char::is_uppercase)
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_');
    ok.then(|| name.to_string())
}

pub fn compare(tier: Tier, a: &Capture, b: &Capture) -> Outcome {
    let mut diffs = Vec::new();
    match (a.timed_out, b.timed_out) {
        // Nothing was compared, so nothing can be claimed. A cell that
        // outlives the timeout on both sides is a corpus-tuning problem
        // (`slow = true`, a longer `--timeout`), not a parity result — and
        // reporting it as a difference would train the reader to ignore
        // differences.
        (true, true) => return Outcome::Skipped("timed out on both sides".into()),
        (x, y) if x != y => {
            return Outcome::Diff(vec![format!("timeout: a={x} b={y}")]);
        }
        _ => {}
    }
    if a.code != b.code {
        diffs.push(format!("exit code: a={} b={}", a.code, b.code));
    }
    match tier {
        Tier::T0 | Tier::T1 => match (&a.summary, &b.summary) {
            (Some(sa), Some(sb)) => diffs.extend(summary_diff(tier, sa, sb)),
            (None, None) => {
                if diffs.is_empty() {
                    return Outcome::Skipped("no verdict (exit code only)".into());
                }
            }
            _ => diffs.push("summary.json produced by only one side".into()),
        },
        Tier::T2 => match event_diff(a, b) {
            Some(d) => diffs.extend(d),
            None => {
                if diffs.is_empty() {
                    return Outcome::Skipped("no event log (run emits none)".into());
                }
            }
        },
        Tier::T3 => {
            if a.stdout != b.stdout {
                diffs.push(first_line_diff("stdout", &a.stdout, &b.stdout));
            }
            if a.stderr != b.stderr {
                diffs.push(first_line_diff("stderr", &a.stderr, &b.stderr));
            }
            // The event log is JSON-per-line whose `n` renumbers after any
            // divergence and whose `run` event names the implementation, so a
            // byte comparison would report a wall of noise where the
            // structural differ reports one first difference.
            diffs.extend(event_diff(a, b).unwrap_or_default());
            diffs.extend(file_diff(a, b));
        }
    }
    if diffs.is_empty() {
        Outcome::Same
    } else {
        Outcome::Diff(diffs)
    }
}

/// T0 reads the verdict block; T1 adds `stats` and `root`.
fn summary_diff(tier: Tier, a: &str, b: &str) -> Vec<String> {
    let (va, vb): (Value, Value) = match (serde_json::from_str(a), serde_json::from_str(b)) {
        (Ok(x), Ok(y)) => (x, y),
        _ => return vec!["summary.json is not valid JSON on one side".into()],
    };
    let keys: &[&str] = if tier == Tier::T0 {
        &["verdict"]
    } else {
        &["verdict", "stats", "root"]
    };
    let mut out = Vec::new();
    for key in keys {
        walk(key, va.get(key), vb.get(key), &mut out);
    }
    out
}

/// Structural JSON diff, deepest path first, capped so one wholesale
/// divergence does not bury the rest of the report.
fn walk(path: &str, a: Option<&Value>, b: Option<&Value>, out: &mut Vec<String>) {
    if out.len() >= 12 {
        return;
    }
    match (a, b) {
        (None, None) => {}
        (Some(x), Some(y)) if x == y => {}
        (Some(Value::Object(x)), Some(Value::Object(y))) => {
            let mut keys: Vec<&String> = x.keys().chain(y.keys()).collect();
            keys.sort();
            keys.dedup();
            for k in keys {
                walk(&format!("{path}.{k}"), x.get(k), y.get(k), out);
            }
        }
        (Some(Value::Array(x)), Some(Value::Array(y))) => {
            if x.len() != y.len() {
                out.push(format!("{path}: {} items vs {}", x.len(), y.len()));
                return;
            }
            for (i, (xi, yi)) in x.iter().zip(y).enumerate() {
                walk(&format!("{path}[{i}]"), Some(xi), Some(yi), out);
            }
        }
        (x, y) => out.push(format!(
            "{path}: {} vs {}",
            x.map_or("<absent>".into(), terse),
            y.map_or("<absent>".into(), terse)
        )),
    }
}

fn terse(v: &Value) -> String {
    let s = v.to_string();
    if s.len() > 60 {
        format!("{}…", &s[..57])
    } else {
        s
    }
}

/// Compare the two `--events` logs structurally, or `None` when neither side
/// produced one.
fn event_diff(a: &Capture, b: &Capture) -> Option<Vec<String>> {
    let (ea, eb) = (
        a.files.get(crate::plan::EVENTS_FILE),
        b.files.get(crate::plan::EVENTS_FILE),
    );
    match (ea, eb) {
        (None, None) => None,
        (Some(_), None) | (None, Some(_)) => {
            Some(vec!["event log written by only one side".into()])
        }
        (Some(x), Some(y)) => Some(match crate::events::diff_text(x, y) {
            Err(e) => vec![format!("events: {e}")],
            Ok(r) => r
                .first_diff
                .map(|d| {
                    let mut v = vec![format!("events: first difference at event {}", d.index)];
                    v.extend(d.fields);
                    v
                })
                .unwrap_or_default(),
        }),
    }
}

fn file_diff(a: &Capture, b: &Capture) -> Vec<String> {
    let mut out = Vec::new();
    let mut names: Vec<&String> = a
        .files
        .keys()
        .chain(b.files.keys())
        .filter(|n| n.as_str() != crate::plan::EVENTS_FILE)
        .collect();
    names.sort();
    names.dedup();
    for name in names {
        match (a.files.get(name), b.files.get(name)) {
            (Some(x), Some(y)) if x == y => {}
            (Some(x), Some(y)) => out.push(first_line_diff(name, x, y)),
            (Some(_), None) => out.push(format!("{name}: written by a only")),
            (None, Some(_)) => out.push(format!("{name}: written by b only")),
            (None, None) => {}
        }
    }
    out
}

/// The first differing line, with its number — the useful half of a diff when
/// the report has one line per cell.
fn first_line_diff(what: &str, a: &str, b: &str) -> String {
    for (i, (la, lb)) in a.lines().zip(b.lines()).enumerate() {
        if la != lb {
            return format!("{what}:{}: {:?} vs {:?}", i + 1, trunc(la), trunc(lb));
        }
    }
    let (na, nb) = (a.lines().count(), b.lines().count());
    format!("{what}: {na} lines vs {nb}")
}

fn trunc(s: &str) -> String {
    if s.chars().count() > 70 {
        format!("{}…", s.chars().take(69).collect::<String>())
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn diffs(tier: Tier, a: Value, b: Value) -> Vec<String> {
        summary_diff(tier, &a.to_string(), &b.to_string())
    }

    #[test]
    fn t0_reads_the_verdict_and_ignores_counters() {
        let a = json!({"verdict": {"k": 1}, "stats": {"enterings_total": 10}});
        let b = json!({"verdict": {"k": 1}, "stats": {"enterings_total": 99}});
        assert!(diffs(Tier::T0, a.clone(), b.clone()).is_empty());
        assert_eq!(diffs(Tier::T1, a, b), ["stats.enterings_total: 10 vs 99"]);
    }

    #[test]
    fn a_verdict_difference_names_its_path() {
        let a = json!({"verdict": {"type": "Solution", "k": 1}});
        let b = json!({"verdict": {"type": "Ambiguity", "k": 2}});
        assert_eq!(
            diffs(Tier::T0, a, b),
            [
                "verdict.k: 1 vs 2",
                "verdict.type: \"Solution\" vs \"Ambiguity\""
            ]
        );
    }

    #[test]
    fn arrays_report_length_before_elements() {
        let a = json!({"verdict": {"solutions": [1, 2]}});
        let b = json!({"verdict": {"solutions": [1]}});
        assert_eq!(diffs(Tier::T0, a, b), ["verdict.solutions: 2 items vs 1"]);
    }

    #[test]
    fn t2_without_an_event_log_is_skipped_not_passed() {
        let cap = |c| Capture {
            code: c,
            stdout: String::new(),
            stderr: String::new(),
            files: Default::default(),
            summary: None,
            wall: std::time::Duration::ZERO,
            timed_out: false,
        };
        assert!(matches!(
            compare(Tier::T2, &cap(0), &cap(0)),
            Outcome::Skipped(_)
        ));
    }

    #[test]
    fn t2_reads_the_event_log() {
        let cap = |log: &str| Capture {
            code: 0,
            stdout: String::new(),
            stderr: String::new(),
            files: [(crate::plan::EVENTS_FILE.to_string(), log.to_string())]
                .into_iter()
                .collect(),
            summary: None,
            wall: std::time::Duration::ZERO,
            timed_out: false,
        };
        let a = cap("{\"e\":\"fire\",\"n\":0,\"rule\":\"symmetric\"}\n");
        let b = cap("{\"e\":\"fire\",\"n\":0,\"rule\":\"transitive\"}\n");
        assert!(matches!(compare(Tier::T2, &a, &a), Outcome::Same));
        assert!(compare(Tier::T2, &a, &b).is_diff());
    }

    fn cap_err(code: i32, stderr: &str) -> Capture {
        Capture {
            code,
            stdout: String::new(),
            stderr: stderr.into(),
            files: Default::default(),
            summary: None,
            wall: std::time::Duration::ZERO,
            timed_out: false,
        }
    }

    #[test]
    fn a_crash_is_compared_by_class_not_message() {
        // The same fixture under two hash seeds: same class, different text.
        let a = cap_err(
            1,
            "Traceback…\nTypeError: '<' not supported between instances of 'int' and 'str'\n",
        );
        let b = cap_err(
            1,
            "Traceback…\nTypeError: '<' not supported between instances of 'str' and 'int'\n",
        );
        assert!(matches!(compare_crash(&a, &b), Outcome::Same));
        // A different exception is a real difference.
        let c = cap_err(1, "Traceback…\nKeyError: 'x'\n");
        assert!(compare_crash(&a, &c).is_diff());
        // …and so is crashing on one side only.
        let d = cap_err(0, "");
        assert!(compare_crash(&a, &d).is_diff());
    }

    #[test]
    fn exception_class_ignores_prose() {
        assert_eq!(
            exception_class("TypeError: boom\n").as_deref(),
            Some("TypeError")
        );
        assert_eq!(
            exception_class("ein.kb.KBLoadError: x\n").as_deref(),
            Some("ein.kb.KBLoadError")
        );
        assert_eq!(exception_class("kb load error: duplicate\n"), None);
        assert_eq!(exception_class(""), None);
    }

    #[test]
    fn first_line_diff_points_at_the_line() {
        assert_eq!(
            first_line_diff("stdout", "a\nb\n", "a\nc\n"),
            "stdout:2: \"b\" vs \"c\""
        );
    }
}
