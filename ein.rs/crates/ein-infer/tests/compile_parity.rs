//! S1a.3.1 acceptance — every `(rule, activator)` pair in the corpus compiles
//! to the same plan, and every way of refusing to compile says the same thing.
//!
//! A `JoinPlan` has no CLI surface, so this follows `load_parity.rs`'s shape:
//! `ein_infer::plan_shape` renders every plan a KB compiles as one
//! deterministic text, `utils/ir_oracle.py`'s `plan-shape` op renders the same
//! text from ein.py, and the two are diffed. A `CompileError` is compared the
//! same way, which extends the four S1.22.0 message fixtures to the whole
//! corpus.

use ein_core::Terms;
use ein_ir::{Ast, load_file};
use ein_oracle::{Answer, IR_ORACLE, Oracle, corpus_files, repo_root, skip};
use std::path::{Path, PathBuf};

/// What ein.rs answers, in the oracle's vocabulary. A file that does not load
/// is not this stage's business — the loader is already at parity — so it
/// answers `None` and the caller skips it.
fn rust_shape(path: &Path, filter: bool) -> Option<Answer> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, path).ok()?;
    Some(
        match ein_infer::plan_shape_with(&ast, &mut terms, &kb, filter) {
            Ok(text) => Answer::Ok(text),
            Err(e) => Answer::Err {
                kind: "CompileError".into(),
                msg: e.0,
            },
        },
    )
}

/// The oracle's `plan-shape`, with the arity filter as the caller wants it.
fn py_shape(py: &mut Oracle, path: &Path, filter: bool) -> Answer {
    py.ask(serde_json::json!({
        "op": "plan-shape",
        "path": path.to_string_lossy(),
        "filter": filter,
    }))
}

#[test]
fn the_whole_corpus_compiles_to_the_same_plans() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("the_whole_corpus_compiles_to_the_same_plans");
    };
    let files = corpus_files();
    assert!(files.len() >= 90, "only {} corpus files found", files.len());
    let (mut bad, mut compared, mut plans, mut rejected) = (Vec::new(), 0, 0, 0);
    for path in &files {
        let Some(got) = rust_shape(path, true) else {
            continue;
        };
        let want = py_shape(&mut py, path, true);
        let name = path.strip_prefix(repo_root()).unwrap_or(path).display();
        match (&got, &want) {
            (Answer::Ok(a), Answer::Ok(b)) => {
                compared += 1;
                plans += a.lines().filter(|l| l.starts_with("PLAN ")).count();
                if a != b {
                    bad.push(format!("{name}\n{}", first_difference(a, b)));
                }
            }
            (Answer::Err { msg: a, .. }, Answer::Err { msg: b, .. }) => {
                rejected += 1;
                if a != b {
                    bad.push(format!("{name}\n  ein.py: {b}\n  ein.rs: {a}"));
                }
            }
            (Answer::Ok(_), Answer::Err { msg, .. }) => bad.push(format!(
                "{name}\n  ein.py refused: {msg}\n  ein.rs compiled"
            )),
            (Answer::Err { msg, .. }, Answer::Ok(_)) => bad.push(format!(
                "{name}\n  ein.py compiled\n  ein.rs refused: {msg}"
            )),
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {} files differ:\n\n{}",
        bad.len(),
        compared + rejected,
        bad.join("\n\n")
    );
    // A corpus that compiled nothing would pass the diff and prove nothing.
    assert!(
        compared >= 50 && plans >= 200,
        "only {compared} files / {plans} plans compared"
    );
}

/// The four `CompileError`s, each on its own fixture, compared on the message
/// **and** against the `.expected` text checked in beside it.
///
/// The corpus sweep above would catch three of them only by accident — an
/// authoring error is not something a shipping example contains — and the
/// fourth (`activator arity`) is unreachable through the ordinary walk at all,
/// because `activators_for` filters mismatched activators before the compiler
/// ever sees one. That filter is exactly what S1.22.0 added, so the fixtures
/// run **unfiltered**, which is what a direct caller of `compile_rule` is.
#[test]
fn every_compile_error_says_what_ein_py_says() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("every_compile_error_says_what_ein_py_says");
    };
    let dir = repo_root().join("examples/broken/compile");
    let mut fixtures: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "ein"))
        .collect();
    fixtures.sort();
    assert!(
        fixtures.len() == 4,
        "expected the compile-error fixtures in {}",
        dir.display()
    );
    for path in &fixtures {
        let got = rust_shape(path, false).expect("the fixture loads; only compiling fails");
        let want = py_shape(&mut py, path, false);
        let name = path.file_name().expect("a file").to_string_lossy();
        let expected = std::fs::read_to_string(path.with_extension("expected"))
            .unwrap_or_else(|e| panic!("{name}.expected: {e}"));
        match (&got, &want) {
            (Answer::Err { msg: a, .. }, Answer::Err { msg: b, .. }) => {
                assert_eq!(a, b, "{name}: message differs");
                assert_eq!(
                    a.trim_end(),
                    expected.trim_end(),
                    "{name}: .expected is stale"
                );
            }
            _ => {
                panic!("{name}: expected both sides to refuse, got\n  rs: {got:?}\n  py: {want:?}")
            }
        }
    }
}

/// The first differing line, with the four lines of context a plan diff needs
/// to be readable — a step sequence is only legible next to its `PLAN` header.
fn first_difference(a: &str, b: &str) -> String {
    let (ours, theirs): (Vec<&str>, Vec<&str>) = (a.lines().collect(), b.lines().collect());
    for (i, (x, y)) in ours.iter().zip(theirs.iter()).enumerate() {
        if x != y {
            let from = i.saturating_sub(4);
            let context: Vec<String> = ours[from..i].iter().map(|l| format!("    {l}")).collect();
            return format!(
                "  line {}:\n{}\n  ein.py: {y}\n  ein.rs: {x}",
                i + 1,
                context.join("\n")
            );
        }
    }
    format!(
        "  same prefix, different length: ein.py {} lines, ein.rs {} lines\n  \
         first extra: {:?}",
        theirs.len(),
        ours.len(),
        ours.get(theirs.len()).or_else(|| theirs.get(ours.len()))
    )
}
