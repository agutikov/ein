//! S1a.5.1 acceptance — Graphviz accepts every file the port emits.
//!
//! The byte sweep in `dot_parity.rs` proves the two implementations agree. It
//! cannot prove they agree on something *valid*: two identically broken files
//! match just as well as two good ones. So every view of every corpus entry
//! also goes through `dot -Tsvg`, which is the only authority on whether the
//! grammar and the attribute values are real.
//!
//! Skipped, loudly, when Graphviz is not installed — a check that quietly
//! passes because the tool is missing is worse than no check.

use ein_core::Terms;
use ein_ir::{Ast, parse};
use ein_oracle::{corpus_files, repo_root, skip};
use ein_render::shape::{all_views, dot_shape};
use std::io::Write;
use std::process::{Command, Stdio};

fn have_graphviz() -> bool {
    Command::new("dot")
        .arg("-V")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

/// Feed one DOT text to `dot -Tsvg`, discarding the SVG. Returns Graphviz's
/// complaint, or `None` when it is happy.
fn graphviz_rejects(dot: &str) -> Option<String> {
    let mut child = Command::new("dot")
        .args(["-Tsvg", "-o", "/dev/null"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("graphviz was there a moment ago");
    child
        .stdin
        .take()
        .expect("piped")
        .write_all(dot.as_bytes())
        .expect("dot reads its input");
    let out = child.wait_with_output().expect("dot exits");
    let err = String::from_utf8_lossy(&out.stderr);
    // Graphviz exits 0 on a warning, so the stderr text is the real signal.
    if out.status.success() && err.trim().is_empty() {
        None
    } else {
        Some(format!("exit {}: {}", out.status, err.trim()))
    }
}

/// The DOT inside one view.
///
/// Two things in a view are the shape op's and not Graphviz's: the `--- …`
/// separator lines that `ir-forms` and `slice` interleave between digraphs,
/// and the `NO PROOF` sentinel a budget-aborted solve returns instead of a
/// lattice. Strip the first, and let the second read as an empty view — what
/// is left is a stream of digraphs, which `dot` reads happily.
fn digraphs_only(view: &str) -> String {
    if view.trim() == "NO PROOF" {
        return String::new();
    }
    view.lines()
        .filter(|l| !l.starts_with("--- "))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn graphviz_accepts_every_view_of_every_corpus_file() {
    if !have_graphviz() {
        return skip("graphviz_accepts_every_view_of_every_corpus_file");
    }
    let views = all_views();
    let (mut bad, mut checked) = (Vec::new(), 0usize);
    for path in &corpus_files() {
        let name = path.strip_prefix(repo_root()).unwrap_or(path).display();
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        for view in &views {
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let Ok(forms) = parse(&mut ast, &text, path.to_str()) else {
                continue;
            };
            let Ok(out) = dot_shape(&mut ast, &mut terms, &forms, path.parent(), view) else {
                continue;
            };
            let dot = digraphs_only(&out);
            // An empty view is not a broken one: a file with no rules renders
            // no rule library, and `(config …)` renders nothing at all.
            if dot.trim().is_empty() {
                continue;
            }
            checked += 1;
            if let Some(why) = graphviz_rejects(&dot) {
                bad.push(format!("{name} [{view}]\n  {why}"));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {checked} views are not valid DOT:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    eprintln!("graphviz: {checked} non-empty views accepted");
    assert!(checked >= 500, "only {checked} views reached Graphviz");
}
