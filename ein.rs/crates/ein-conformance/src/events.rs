//! `ein-conformance diff a.jsonl b.jsonl` — the T2 event differ.
//!
//! Two `--events` logs, compared event by event. The schema is
//! `conformance/EVENTS.md`.
//!
//! The report is built around the two questions a T2 failure actually raises,
//! in the order they are worth asking:
//!
//! 1. **Did one side stop narrating a whole class of thing?** The class
//!    summary answers that before any line detail. "b emitted no `park`
//!    events at all" is a more useful first sentence than a field diff at
//!    line 4 — it says *which subsystem* diverged, not which byte.
//! 2. **Where did the two streams part?** The first differing event, with the
//!    preceding few from both sides for context, and a field-level diff of the
//!    pair. Everything after the first divergence is downstream of it and is
//!    not reported.

use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

/// Events preceding the first difference, shown from both sides.
const CONTEXT: usize = 4;

pub struct Report {
    pub classes_a: BTreeMap<String, usize>,
    pub classes_b: BTreeMap<String, usize>,
    pub first_diff: Option<FirstDiff>,
    pub len_a: usize,
    pub len_b: usize,
}

pub struct FirstDiff {
    pub index: usize,
    pub context: Vec<(String, String)>,
    pub fields: Vec<String>,
    pub a: Option<String>,
    pub b: Option<String>,
}

fn parse(text: &str, what: &str) -> Result<Vec<Value>, String> {
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).map_err(|e| format!("{what}: {e}")))
        .collect()
}

fn load(path: &Path) -> Result<Vec<Value>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    parse(&text, &path.display().to_string())
}

fn classes(events: &[Value]) -> BTreeMap<String, usize> {
    let mut out = BTreeMap::new();
    for e in events {
        let kind = e.get("e").and_then(Value::as_str).unwrap_or("<no e>");
        *out.entry(kind.to_string()).or_insert(0) += 1;
    }
    out
}

/// Strip the fields that differ by construction rather than by behaviour.
///
/// - `n` is a per-run counter, so it differs the moment either side emits one
///   extra event. It is reported as a *position*, not a field.
/// - the `run` event's `impl` names which implementation ran, which is the
///   whole point of the comparison, and its `argv` carries the artefact paths
///   the *caller* chose — `--events a.jsonl` vs `--events b.jsonl` is not a
///   divergence. Both stay in the file, where they document the run; neither
///   is compared.
fn comparable(e: &Value) -> Value {
    let mut e = e.clone();
    let is_run = e.get("e").and_then(Value::as_str) == Some("run");
    if let Some(obj) = e.as_object_mut() {
        obj.remove("n");
        if is_run {
            obj.remove("impl");
            obj.remove("argv");
        }
    }
    e
}

pub fn diff(a_path: &Path, b_path: &Path) -> Result<Report, String> {
    compare(load(a_path)?, load(b_path)?)
}

/// The same comparison over two in-memory logs — what the runner's T2 tier
/// uses, so the hand tool and the harness cannot drift apart.
pub fn diff_text(a: &str, b: &str) -> Result<Report, String> {
    compare(parse(a, "a")?, parse(b, "b")?)
}

fn compare(a: Vec<Value>, b: Vec<Value>) -> Result<Report, String> {
    let mut first_diff = None;
    for i in 0..a.len().max(b.len()) {
        let (ea, eb) = (a.get(i), b.get(i));
        let same = match (ea, eb) {
            (Some(x), Some(y)) => comparable(x) == comparable(y),
            _ => false,
        };
        if same {
            continue;
        }
        let context = (i.saturating_sub(CONTEXT)..i)
            .map(|j| {
                (
                    a.get(j).map(one_line).unwrap_or_default(),
                    b.get(j).map(one_line).unwrap_or_default(),
                )
            })
            .collect();
        first_diff = Some(FirstDiff {
            index: i,
            context,
            fields: field_diff(ea, eb),
            a: ea.map(one_line),
            b: eb.map(one_line),
        });
        break;
    }
    Ok(Report {
        classes_a: classes(&a),
        classes_b: classes(&b),
        first_diff,
        len_a: a.len(),
        len_b: b.len(),
    })
}

fn one_line(v: &Value) -> String {
    let s = v.to_string();
    if s.chars().count() > 160 {
        format!("{}…", s.chars().take(159).collect::<String>())
    } else {
        s
    }
}

/// Which fields of the two events differ, by name.
fn field_diff(a: Option<&Value>, b: Option<&Value>) -> Vec<String> {
    let (Some(a), Some(b)) = (a, b) else {
        return vec![match (a, b) {
            (Some(_), None) => "b ended first".into(),
            (None, Some(_)) => "a ended first".into(),
            _ => "both absent".into(),
        }];
    };
    let (Some(oa), Some(ob)) = (a.as_object(), b.as_object()) else {
        return vec![format!("{a} vs {b}")];
    };
    let mut keys: Vec<&String> = oa.keys().chain(ob.keys()).filter(|k| *k != "n").collect();
    keys.sort();
    keys.dedup();
    let is_run = a.get("e").and_then(Value::as_str) == Some("run");
    keys.into_iter()
        .filter(|k| !(is_run && (*k == "impl" || *k == "argv")))
        .filter(|k| oa.get(*k) != ob.get(*k))
        .map(|k| {
            format!(
                "  {k}: {} vs {}",
                oa.get(k).map_or("<absent>".into(), one_line),
                ob.get(k).map_or("<absent>".into(), one_line)
            )
        })
        .collect()
}

/// Print the report; return true iff the two logs agree.
pub fn print(report: &Report, show_classes: bool) -> bool {
    if show_classes || report.first_diff.is_some() {
        let mut kinds: Vec<&String> = report
            .classes_a
            .keys()
            .chain(report.classes_b.keys())
            .collect();
        kinds.sort();
        kinds.dedup();
        println!("{:<12} {:>9} {:>9}", "event", "a", "b");
        println!("{}", "─".repeat(32));
        for k in kinds {
            let (x, y) = (
                report.classes_a.get(k).copied().unwrap_or(0),
                report.classes_b.get(k).copied().unwrap_or(0),
            );
            let mark = if x == y { ' ' } else { '*' };
            println!("{k:<12} {x:>9} {y:>9} {mark}");
        }
        println!("{}", "─".repeat(32));
        println!("{:<12} {:>9} {:>9}", "total", report.len_a, report.len_b);
    }
    match &report.first_diff {
        None => {
            println!("\nidentical — {} events", report.len_a);
            true
        }
        Some(d) => {
            println!("\nfirst difference at event {}:", d.index);
            for (x, y) in &d.context {
                println!("  = {x}");
                if x != y {
                    println!("  ? {y}");
                }
            }
            println!("  a {}", d.a.as_deref().unwrap_or("<end of log>"));
            println!("  b {}", d.b.as_deref().unwrap_or("<end of log>"));
            for line in &d.fields {
                println!("{line}");
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn write(name: &str, lines: &[&str]) -> PathBuf {
        let dir = std::env::temp_dir().join("ein-conformance-tests");
        std::fs::create_dir_all(&dir).expect("tmp");
        let path = dir.join(name);
        std::fs::write(&path, lines.join("\n") + "\n").expect("write");
        path
    }

    const RUN_A: &str = r#"{"e":"run","n":0,"version":"ein-events/1","impl":"ein.py","argv":["--events","a.jsonl"]}"#;
    const RUN_B: &str = r#"{"e":"run","n":0,"version":"ein-events/1","impl":"ein.rs","argv":["--events","b.jsonl"]}"#;
    const FIRE: &str = r#"{"e":"fire","n":1,"rule":"symmetric","derived":["(knows B A)"]}"#;

    #[test]
    fn the_run_events_harness_chosen_fields_are_not_compared() {
        // Same run, two implementations, two artefact paths. Neither is a
        // divergence — and both stay in the file.
        let a = write("run_a.jsonl", &[RUN_A, FIRE]);
        let b = write("run_b.jsonl", &[RUN_B, FIRE]);
        assert!(diff(&a, &b).expect("diff").first_diff.is_none());
    }

    #[test]
    fn the_sequence_number_is_a_position_not_a_field() {
        // b emits one event fewer, so every later `n` is off by one. The
        // report must point at the *first* real difference, not at every
        // renumbered line after it.
        let a = write(
            "seq_a.jsonl",
            &[RUN_A, FIRE, r#"{"e":"quiesce","n":2,"round":1}"#],
        );
        let b = write(
            "seq_b.jsonl",
            &[RUN_A, r#"{"e":"quiesce","n":1,"round":1}"#],
        );
        let d = diff(&a, &b)
            .expect("diff")
            .first_diff
            .expect("a difference");
        assert_eq!(d.index, 1);
        assert!(
            d.fields.iter().any(|f| f.contains("\"fire\"")),
            "{:?}",
            d.fields
        );
    }

    #[test]
    fn a_field_difference_names_the_field() {
        let a = write("fld_a.jsonl", &[RUN_A, FIRE]);
        let b = write(
            "fld_b.jsonl",
            &[
                RUN_A,
                r#"{"e":"fire","n":1,"rule":"symmetric","derived":["(knows A B)"]}"#,
            ],
        );
        let d = diff(&a, &b)
            .expect("diff")
            .first_diff
            .expect("a difference");
        assert_eq!(d.fields.len(), 1);
        assert!(d.fields[0].starts_with("  derived:"), "{:?}", d.fields);
    }

    #[test]
    fn class_counts_show_a_wholesale_divergence() {
        // The question worth asking first: did one side stop narrating a
        // whole class of thing?
        let a = write("cls_a.jsonl", &[RUN_A, FIRE, FIRE]);
        let b = write("cls_b.jsonl", &[RUN_A]);
        let r = diff(&a, &b).expect("diff");
        assert_eq!(r.classes_a.get("fire"), Some(&2));
        assert_eq!(r.classes_b.get("fire"), None);
    }
}
