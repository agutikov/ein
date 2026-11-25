//! **What a run said, against its summary** (Q-M2.9): the summary's
//! `diagnostics` are the lines the run printed on stderr, kind by kind.
//!
//! [`agree`] reads both. Each diagnostic is a value of one shape,
//! `{kind, severity, message, place}`, its kind one of `ein_core::Kind`'s
//! names and its severity one of three; and stderr is the diagnostics, in
//! their order, and nothing else. A line a diagnostic stands alone on, a
//! note, a warning, the clock's abort, a claim refuted, an artefact's
//! failure, is its [`ein_core::Diagnostic::line`], a whole line of stderr; a
//! refusal's finding is its message inside the refusal's text, which joins
//! a load's findings `; ` behind `kb load error: `, a parse's one under
//! another and a judgement's one a line, and nothing else is between them.
//!
//! [`unanswered`] reads the other half: a summary with no answer, a refused
//! program's or a run the clock ended, every key of the answer `null`.

// Each test binary that reads a summary takes what it needs of the module.
#![allow(dead_code)]

use ein_core::{Diagnostic, Kind, Place, Severity};
use serde_json::Value as J;

/// What a refusal's text holds besides its findings' messages.
const GLUE: [&str; 3] = ["kb load error: ", ".einb PROGRAM does not load: ", "; "];

/// The keys of an answer, every one `null` in a summary with no answer.
pub const ANSWER_KEYS: [&str; 8] = [
    "region",
    "verdict",
    "queries",
    "claim",
    "work",
    "monotonicity",
    "leftover",
    "root",
];

/// Whether `s` is a summary with no answer (Q-M2.9): a program refused at
/// the parse, the load or its node, or a run the clock ended, every key of
/// the answer `null` and an error among its diagnostics saying why.
pub fn unanswered(s: &J) -> bool {
    ANSWER_KEYS.iter().all(|k| s[k].is_null())
        && s["diagnostics"]
            .as_array()
            .is_some_and(|d| d.iter().any(|x| x["severity"] == "error"))
}

/// A diagnostic of the summary, read back as the value it was written from;
/// `Err` where it is not of the one shape.
pub fn diagnostic(v: &J) -> Result<Diagnostic, String> {
    let J::Object(m) = v else {
        return Err(format!("a diagnostic that is no object: {v}"));
    };
    // Read as a set: a JSON object's keys come back sorted.
    let mut keys: Vec<&str> = m.keys().map(String::as_str).collect();
    keys.sort_unstable();
    if keys != ["kind", "message", "place", "severity"] {
        return Err(format!("a diagnostic's keys are {keys:?}"));
    }
    let kind = v["kind"]
        .as_str()
        .and_then(Kind::of_name)
        .ok_or_else(|| format!("no kind of Ein's: {}", v["kind"]))?;
    let severity = match v["severity"].as_str() {
        Some("error") => Severity::Error,
        Some("warning") => Severity::Warning,
        Some("note") => Severity::Note,
        _ => return Err(format!("no severity: {}", v["severity"])),
    };
    let message = v["message"]
        .as_str()
        .ok_or_else(|| format!("a message that is no string: {}", v["message"]))?
        .to_string();
    let place = match &v["place"] {
        J::Null => None,
        p @ J::Object(pm) => {
            let mut keys: Vec<&str> = pm.keys().map(String::as_str).collect();
            keys.sort_unstable();
            let line = p["line"].as_u64();
            let col = p["col"].as_u64();
            match (keys.as_slice(), p["file"].as_str(), line, col) {
                (["col", "file", "line"], Some(file), Some(line), Some(col))
                    if line >= 1 && col >= 1 =>
                {
                    Some(Place {
                        file: file.to_string(),
                        line: line as u32,
                        col: col as u32,
                    })
                }
                _ => return Err(format!("a place that is not {{file, line, col}}: {p}")),
            }
        }
        p => return Err(format!("a place that is no object: {p}")),
    };
    Ok(Diagnostic {
        kind,
        severity,
        place,
        message,
    })
}

/// Whether `d` is a refusal's finding, printed inside the refusal's text and
/// not on a line of its own.
fn in_a_refusal(d: &Diagnostic) -> bool {
    d.severity == Severity::Error && !matches!(d.kind, Kind::Deadline | Kind::Artefact)
}

/// **The summary's diagnostics are what stderr printed**, kind by kind:
/// `Ok` where every diagnostic is of the one shape, each is on stderr in its
/// order, as its line or inside its refusal's text, and stderr holds nothing
/// else; `Err` says what differs.
pub fn agree(summary: &J, stderr: &str) -> Result<(), String> {
    let Some(list) = summary["diagnostics"].as_array() else {
        return Err(format!(
            "`diagnostics` is no list: {}",
            summary["diagnostics"]
        ));
    };
    let mut covered = vec![false; stderr.len()];
    let mut at = 0usize;
    for v in list {
        let d = diagnostic(v)?;
        let (want, whole) = if in_a_refusal(&d) {
            (d.message.clone(), false)
        } else {
            (d.line(), true)
        };
        let found = stderr[at..]
            .match_indices(want.as_str())
            .map(|(i, _)| at + i)
            .find(|&i| {
                !whole
                    || ((i == 0 || stderr.as_bytes()[i - 1] == b'\n')
                        && stderr[i + want.len()..].starts_with('\n'))
            });
        let Some(i) = found else {
            return Err(format!(
                "the summary's {} {} is not on stderr after what came before it: {want:?}\n\
                 stderr:\n{stderr}",
                d.severity.name(),
                d.kind.name()
            ));
        };
        covered[i..i + want.len()]
            .iter_mut()
            .for_each(|c| *c = true);
        at = i + want.len();
    }
    // What no diagnostic carries is glue, or nothing.
    let mut rest = String::new();
    for (i, ch) in stderr.char_indices() {
        if !covered[i] {
            rest.push(ch);
        }
    }
    let mut left = rest;
    for g in GLUE {
        left = left.replace(g, "");
    }
    if left.trim().is_empty() {
        Ok(())
    } else {
        Err(format!(
            "stderr prints what no diagnostic of the summary carries: {:?}\nstderr:\n{stderr}",
            left.trim()
        ))
    }
}
