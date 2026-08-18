//! S1a.3.2 acceptance — the matcher produces the same matches, in the same
//! order, with the same bindings and the same premises.
//!
//! The matcher's signature is the firing order, so this is the stage's gate:
//! `ein_infer::match_shape` runs every plan over every corpus KB — a full run
//! and a `run_seeded` at every fact — and renders one line per match;
//! `utils/ir_oracle.py`'s `match-shape` op renders the same from ein.py.
//!
//! T2 event-trace parity is the *phase*'s gate and needs the saturator
//! (S1a.3.3). This is the half that can be checked without one, and it is the
//! half that would otherwise be diagnosed through a 40 k-event log: a bind
//! order, a premise position, a candidate that the participation index
//! narrowed away.

use ein_core::Terms;
use ein_ir::{Ast, load_file};
use ein_oracle::{Answer, IR_ORACLE, Oracle, corpus_files, repo_root, skip};
use std::path::Path;

fn rust_matches(path: &Path) -> Option<Answer> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, path).ok()?;
    Some(match ein_infer::match_shape(&ast, &mut terms, &kb) {
        Ok(text) => Answer::Ok(text),
        Err(e) => Answer::Err {
            kind: "CompileError".into(),
            msg: e.0,
        },
    })
}

#[test]
fn the_whole_corpus_matches_the_same_way() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("the_whole_corpus_matches_the_same_way");
    };
    let (mut bad, mut compared, mut matches) = (Vec::new(), 0, 0usize);
    for path in &corpus_files() {
        let Some(got) = rust_matches(path) else {
            continue;
        };
        let want = py.file("match-shape", path);
        let name = path.strip_prefix(repo_root()).unwrap_or(path).display();
        match (&got, &want) {
            (Answer::Ok(a), Answer::Ok(b)) => {
                compared += 1;
                matches += a.lines().filter(|l| !l.starts_with("PLAN ")).count();
                if a != b {
                    bad.push(format!("{name}\n{}", first_difference(a, b)));
                }
            }
            (Answer::Err { msg: a, .. }, Answer::Err { msg: b, .. }) => {
                if a != b {
                    bad.push(format!("{name}\n  ein.py: {b}\n  ein.rs: {a}"));
                }
            }
            _ => bad.push(format!("{name}\n  rs: {got:?}\n  py: {want:?}")),
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {compared} files differ:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    assert!(
        compared >= 50 && matches >= 1500,
        "only {compared} files / {matches} matches compared"
    );
}

/// The first differing line with its `PLAN` header — a match line is not
/// legible without knowing which plan produced it.
fn first_difference(a: &str, b: &str) -> String {
    let (ours, theirs): (Vec<&str>, Vec<&str>) = (a.lines().collect(), b.lines().collect());
    for (i, (x, y)) in ours.iter().zip(theirs.iter()).enumerate() {
        if x != y {
            let header = ours[..i]
                .iter()
                .rev()
                .find(|l| l.starts_with("PLAN "))
                .copied()
                .unwrap_or("<no plan header>");
            return format!(
                "  under {header}\n  at line {}\n  ein.py: {y}\n  ein.rs: {x}",
                i + 1
            );
        }
    }
    let (extra, side) = if ours.len() > theirs.len() {
        (ours.get(theirs.len()), "ein.rs")
    } else {
        (theirs.get(ours.len()), "ein.py")
    };
    format!(
        "  same prefix, different length: ein.py {} lines, ein.rs {} lines\n  \
         first extra ({side}): {extra:?}",
        theirs.len(),
        ours.len(),
    )
}
