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
//!
//! # What T2 and T3 do **not** compare, and why
//!
//! Since [S1a.6.9](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md)
//! the two engines narrate different amounts of the same derivation on purpose
//! ([D3](../../../../plans/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)),
//! and the rule that says which parts of an artefact that makes unreadable is
//! `ein-parity` — one crate, shared with every crate's own parity tests, so
//! the gate and the tests cannot drift apart. [`Ctx`] carries it here.
//!
//! **T0 and T1 are not relaxed in any direction**, and `summary.json` — which
//! is what they read — is excluded from the normalisation by name, not by
//! accident: a firing count appearing in it would be a T1 move, which is the
//! one thing the relaxation must never hide.

use serde_json::Value;
use std::fmt;

use crate::run::Capture;

/// How strictly to compare, and what this cell wrote that is a rendered
/// derivation.
///
/// `strict` is `--strict`: it restores the byte-identical contract
/// P1a.1–P1a.5 was built against, and the determinism sweep runs under it.
pub struct Ctx {
    pub strict: bool,
    /// Artefacts of this run that are rendered derivations — the `--trace`
    /// markdown. From [`ein_parity::narrated_artefacts`].
    pub narrated: Vec<String>,
}

impl Ctx {
    /// The unrelaxed contract with no narrated artefacts. Only the tests
    /// below construct one this way; `cmd_run` builds a `Ctx` per cell,
    /// because `narrated` is a property of the run.
    #[cfg(test)]
    pub fn strict() -> Ctx {
        Ctx {
            strict: true,
            narrated: Vec::new(),
        }
    }

    /// The D3 normalisation of one captured stream or file. A `summary.json`
    /// is never normalised; see the module note.
    ///
    /// By **basename**, because a run writes two of them: the
    /// `--json-summary` T0/T1 reads, and the one inside a `--dump-states`
    /// tree. Neither is narration, and matching only the first would have made
    /// that a coincidence rather than a rule.
    fn blank(&self, name: &str, text: &str) -> String {
        let base = name.rsplit('/').next().unwrap_or(name);
        if self.strict || base == "summary.json" {
            return text.to_string();
        }
        ein_parity::blank(text)
    }
}

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

pub fn compare(tier: Tier, a: &Capture, b: &Capture, ctx: &Ctx) -> Outcome {
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
        Tier::T2 => match event_diff(a, b, ctx) {
            Some(d) => diffs.extend(d),
            None => {
                if diffs.is_empty() {
                    return Outcome::Skipped("no event log (run emits none)".into());
                }
            }
        },
        Tier::T3 => {
            for (what, x, y) in [
                ("stdout", &a.stdout, &b.stdout),
                ("stderr", &a.stderr, &b.stderr),
            ] {
                let (x, y) = (ctx.blank(what, x), ctx.blank(what, y));
                if x != y {
                    diffs.push(first_line_diff(what, &x, &y));
                }
            }
            // The event log is JSON-per-line whose `n` renumbers after any
            // divergence and whose `run` event names the implementation, so a
            // byte comparison would report a wall of noise where the
            // structural differ reports one first difference.
            diffs.extend(event_diff(a, b, ctx).unwrap_or_default());
            diffs.extend(file_diff(a, b, ctx));
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
fn event_diff(a: &Capture, b: &Capture, ctx: &Ctx) -> Option<Vec<String>> {
    let (ea, eb) = (
        a.files.get(crate::plan::EVENTS_FILE),
        b.files.get(crate::plan::EVENTS_FILE),
    );
    match (ea, eb) {
        (None, None) => None,
        (Some(_), None) | (None, Some(_)) => {
            Some(vec!["event log written by only one side".into()])
        }
        (Some(x), Some(y)) => Some(match crate::events::diff_text(x, y, ctx.strict) {
            Err(e) => vec![format!("events: {e}")],
            Ok(r) => r.report(),
        }),
    }
}

fn file_diff(a: &Capture, b: &Capture, ctx: &Ctx) -> Vec<String> {
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
            (Some(x), Some(y)) => {
                // A **rendered derivation** — the `--trace` markdown — is
                // compared for presence: both sides wrote one, and neither
                // wrote an empty one. What replaces the byte diff is an
                // ein.rs golden (S1a.6.11), because there is no normalisation
                // that makes the two texts agree: ein.rs's trace opens with a
                // *Before any assumption* section ein.py has no counterpart
                // for, and ein.py's spine carries its fork's re-derivation of
                // root, redundant steps included.
                if !ctx.strict && ctx.narrated.iter().any(|n| n == name) {
                    if x.trim().is_empty() != y.trim().is_empty() {
                        out.push(format!("{name}: empty on one side only"));
                    }
                    continue;
                }
                let (x, y) = (ctx.blank(name, x), ctx.blank(name, y));
                if x != y {
                    out.push(first_line_diff(name, &x, &y));
                }
            }
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
            compare(Tier::T2, &cap(0), &cap(0), &Ctx::strict()),
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
        let ctx = Ctx::strict();
        assert!(matches!(compare(Tier::T2, &a, &a, &ctx), Outcome::Same));
        assert!(compare(Tier::T2, &a, &b, &ctx).is_diff());
    }

    fn cap_files(files: &[(&str, &str)]) -> Capture {
        Capture {
            code: 0,
            stdout: String::new(),
            stderr: String::new(),
            files: files
                .iter()
                .map(|(n, t)| ((*n).to_string(), (*t).to_string()))
                .collect(),
            summary: None,
            wall: std::time::Duration::ZERO,
            timed_out: false,
        }
    }

    /// D3 at T3: the timeline's firing count is narration, and the trace is a
    /// rendered derivation. Everything beside them on the same record is not.
    #[test]
    fn t3_reads_the_timeline_past_its_firing_count() {
        let relaxed = Ctx {
            strict: false,
            narrated: vec!["trace.md".into()],
        };
        let a = cap_files(&[
            (
                "states/00_timeline.jsonl",
                "{\"kind\": \"alive\", \"firings\": 18}",
            ),
            ("trace.md", "# Solution trace\n> Solved in 20 steps"),
        ]);
        let b = cap_files(&[
            (
                "states/00_timeline.jsonl",
                "{\"kind\": \"alive\", \"firings\": 2}",
            ),
            (
                "trace.md",
                "# Solution trace\n> Solved in 4 steps after 16 unconditional",
            ),
        ]);
        assert!(matches!(compare(Tier::T3, &a, &b, &relaxed), Outcome::Same));
        // …and the same pair is a difference on the unrelaxed contract, which
        // is what `--strict` is for.
        assert!(compare(Tier::T3, &a, &b, &Ctx::strict()).is_diff());
        // A trace that *vanished* is still a difference, and so is one that
        // came out empty.
        let gone = cap_files(&[(
            "states/00_timeline.jsonl",
            "{\"kind\": \"alive\", \"firings\": 2}",
        )]);
        assert!(compare(Tier::T3, &a, &gone, &relaxed).is_diff());
        let empty = cap_files(&[
            (
                "states/00_timeline.jsonl",
                "{\"kind\": \"alive\", \"firings\": 2}",
            ),
            ("trace.md", "\n"),
        ]);
        assert!(compare(Tier::T3, &a, &empty, &relaxed).is_diff());
    }

    /// The line the relaxation must never cross: `summary.json` is T0 + T1 and
    /// is compared exactly, whatever it happens to contain.
    #[test]
    fn summary_json_is_never_normalised() {
        let relaxed = Ctx {
            strict: false,
            narrated: Vec::new(),
        };
        for name in ["summary.json", "states/summary.json"] {
            let a = cap_files(&[(name, "{\"stats\": {\"firings\": 18}}")]);
            let b = cap_files(&[(name, "{\"stats\": {\"firings\": 2}}")]);
            assert!(
                compare(Tier::T3, &a, &b, &relaxed).is_diff(),
                "{name} was normalised"
            );
        }
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
