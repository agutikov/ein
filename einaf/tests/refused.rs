//! **Every refused fixture is an invalid object.** Each `.ein` under
//! `examples/broken/` that the kernel refuses, through the bounded call,
//! yields an object whose outcome is *invalid* under every reading, whose
//! diagnostics are the refusal's findings, each an `error` of a kind
//! `07-diagnostics.md` names beside a section, and whose messages hold the
//! line the fixture's `.expected` banks. The program whose saturation has no
//! fixpoint is `tests/outcomes.rs`'.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use einaf::kernel::{Bounds, Outcome, Question, Source, call};

/// The kinds `07-diagnostics.md` names, each on a `**Kind:**` line.
fn refusal_kinds() -> BTreeSet<String> {
    let page = std::fs::read_to_string(
        ein_corpus::repo_root().join("docs/ein/user/04-reference/07-diagnostics.md"),
    )
    .expect("07-diagnostics.md");
    let mut out = BTreeSet::new();
    for line in page.lines() {
        let Some(rest) = line.strip_prefix("**Kind:**") else {
            continue;
        };
        for (i, part) in rest.split('`').enumerate() {
            if i % 2 == 1 {
                out.insert(part.to_string());
            }
        }
    }
    assert!(out.len() > 20, "only {} kinds on the page", out.len());
    out
}

fn ein_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .expect("a directory")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            ein_files(&p, out);
        } else if p.extension().and_then(|x| x.to_str()) == Some("ein") {
            out.push(p);
        }
    }
}

#[test]
fn every_refused_fixture_is_invalid_with_its_findings() {
    let root = ein_corpus::repo_root();
    let kinds = refusal_kinds();
    let stdlib = root.join("stdlib");
    let mut files = Vec::new();
    ein_files(&root.join("examples/broken"), &mut files);
    let found: Vec<Result<bool, String>> = ein_corpus::pool::map(
        "einaf_refused",
        &files,
        |p| (p.display().to_string(), String::new()),
        |path| {
            let at = path
                .strip_prefix(&root)
                .unwrap_or(path)
                .display()
                .to_string();
            let objects = call(Source::File(path), &Question::ALL, Bounds::default());
            let invalid = objects
                .iter()
                .flat_map(|f| f.outcome.iter().flatten())
                .filter(|r| r.outcome == Outcome::Invalid)
                .count();
            if invalid == 0 {
                // A fixture that loads: one the corpus keeps beside the
                // refused ones, a module or a program a later stage refuses.
                return Ok(false);
            }
            let f = &objects[0];
            if objects.iter().any(|o| {
                o.outcome
                    .iter()
                    .flatten()
                    .any(|r| r.outcome != Outcome::Invalid)
                    || o.diagnostics != f.diagnostics
            }) {
                return Err(format!("{at}: invalid under one question and not another"));
            }
            if f.diagnostics.is_empty() {
                return Err(format!("{at}: refused with no finding"));
            }
            for d in &f.diagnostics {
                if d.severity != "error" || !kinds.contains(&d.kind) {
                    return Err(format!(
                        "{at}: a finding of {} {}, which is no refusal the page names",
                        d.severity, d.kind
                    ));
                }
            }
            if f.states.is_some() || f.queries.is_some() || f.readings.is_some() {
                return Err(format!("{at}: refused, with an answer's parts"));
            }
            let file = path.to_str().expect("utf-8");
            let want = std::fs::read_to_string(path.with_extension("expected"))
                .map_err(|e| format!("{at}: {e}"))?
                .trim()
                .replace("{FILE}", file)
                .replace(
                    "{DIR}",
                    path.parent().and_then(Path::to_str).expect("utf-8"),
                )
                .replace("{STDLIB}", stdlib.to_str().expect("utf-8"));
            let messages: Vec<&str> = f.diagnostics.iter().map(|d| d.message.as_str()).collect();
            if !messages.join("; ").contains(&want) && !messages.join("\n").contains(&want) {
                return Err(format!(
                    "{at}: the findings do not hold the .expected line\n  want {want:?}\n  got  {messages:?}"
                ));
            }
            Ok(true)
        },
    );
    let mut refused = 0usize;
    let mut bad = Vec::new();
    for f in found {
        match f {
            Ok(true) => refused += 1,
            Ok(false) => {}
            Err(e) => bad.push(e),
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    assert!(refused >= 60, "only {refused} fixtures were refused");
}
