//! S1a.5.3 acceptance — the `--dump-states` tree, byte for byte.
//!
//! Five modes per corpus entry. The tree has no line protocol to diff over, so
//! it is *rendered* as one text — every file, sorted by path, with its bytes —
//! and the texts are compared. The rendering invents nothing, so a missing
//! file, an extra file, a renamed directory and a changed byte all read the
//! same way.
//!
//! - **`monotonic`** / **`lattice`** — the two file dumpers, whose layouts
//!   differ (`layers/layer_NN_pre.ein` versus `layers/layer_NN/pre.ein`, and
//!   the per-commitment `enterings/` tree only the second writes).
//! - **`progress`** — the live `-v` view *with* an `out_dir`, because the two
//!   compose and the live view is the file dumper plus a stream.
//! - **`abort`** — a budget small enough to trip mid-search, under the raising
//!   policy: `summary.json` must be **absent** and the timeline flushed anyway.
//! - **`snapshot`** — the `LatticeSnapshotV1` projection and the lattice DOT
//!   rendered from it.
//!
//! The timestamps are the one thing that cannot match. They are on the
//! [normalisation list](../../../../plans/m1a_rust/design/01_parity_contract.md) §5
//! and are blanked *by value, not by presence*, on both sides — a record that
//! lost its `ts_ms` still fails.
//!
//! Two things in the tree are **narration** rather than state, and `ein-parity`
//! is what says so: the timeline's per-entering `firings` count, and — in the
//! `snapshot` mode — the dead commitments' `state_key`s with the two lattice
//! DOTs the DAG merges by them. Both are elided here, at comparison time, by
//! the same rule the conformance harness applies at T3. The third,
//! `dump_shape`'s `enterings/` subtree, is elided where it is *produced*, for
//! the measured reason its comment gives; this test checks that the marker it
//! writes is still the one `ein-parity` expects.

use ein_core::Terms;
use ein_ir::{Ast, parse};
use ein_oracle::{Answer, IR_ORACLE, Oracle, corpus_files, repo_root, skip};
use ein_render::shape::{DUMP_MODES, dump_shape};
use std::path::Path;

/// [D2](../../../../plans/m1a_rust/divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers),
/// reached by every mode, because every mode runs the search.
const DIVERGENT: [&str; 1] = ["examples/ein-bugs/mixed-type-hypothesis.ein"];

fn rust_mode(path: &Path, mode: &str) -> Option<Answer> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    match dump_shape(&mut ast, &mut terms, &forms, path.parent(), mode) {
        Ok(out) => Some(Answer::Ok(out)),
        Err(msg) => Some(Answer::Err {
            kind: "DumpShapeError".into(),
            msg,
        }),
    }
}

#[test]
fn the_dump_tree_is_byte_identical_on_the_corpus() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("the_dump_tree_is_byte_identical_on_the_corpus");
    };
    let (mut bad, mut compared, mut bytes, mut files) = (Vec::new(), 0usize, 0usize, 0usize);
    let mut seen_divergent: Vec<String> = Vec::new();
    let (mut aborts, mut narrated) = (0usize, 0usize);
    for path in &corpus_files() {
        let rel = path.strip_prefix(repo_root()).unwrap_or(path);
        let name = rel.display();
        let expected = DIVERGENT.contains(&rel.to_str().unwrap_or_default());
        let before = compared;
        for mode in DUMP_MODES {
            let Some(got) = rust_mode(path, mode) else {
                continue;
            };
            let want = py.ask(serde_json::json!({
                "op": "dump-shape",
                "path": path.to_string_lossy(),
                "mode": mode,
            }));
            match (&got, &want) {
                (Answer::Ok(_), Answer::Err { .. }) if expected => {
                    seen_divergent.push(format!("{name} [{mode}]"));
                }
                _ if expected => bad.push(format!(
                    "{name} [{mode}] is a ledger entry and no longer diverges\n  \
                     rs: {}\n  py: {}",
                    brief(&got),
                    brief(&want)
                )),
                (Answer::Ok(a), Answer::Ok(b)) => {
                    compared += 1;
                    bytes += a.len();
                    let (x, y) = narrow(a, b);
                    if x != y {
                        bad.push(format!("{name} [{mode}]\n{}", first_difference(&x, &y)));
                    }
                    if a != b {
                        narrated += 1;
                    }
                    // The abort mode is only coverage when it *aborts*, and
                    // whether it does depends on the puzzle. Count it, so a
                    // budget that stopped tripping anywhere is visible.
                    if mode == "abort" && a.starts_with("ABORTED True") {
                        aborts += 1;
                        assert!(
                            !a.contains("=== summary.json"),
                            "{name}: a budget abort left a summary.json behind"
                        );
                        assert!(
                            a.contains("=== 00_timeline.jsonl"),
                            "{name}: a budget abort left no timeline behind"
                        );
                    }
                }
                (Answer::Err { .. }, Answer::Err { .. }) => {}
                _ => bad.push(format!(
                    "{name} [{mode}]\n  rs: {}\n  py: {}",
                    brief(&got),
                    brief(&want)
                )),
            }
        }
        if compared > before {
            files += 1;
        }
    }
    let mut want_divergent: Vec<String> = DIVERGENT
        .iter()
        .flat_map(|f| DUMP_MODES.iter().map(move |m| format!("{f} [{m}]")))
        .collect();
    want_divergent.sort();
    seen_divergent.sort();
    assert_eq!(
        seen_divergent, want_divergent,
        "the ledger's divergent modes are not the ones that diverged"
    );
    assert!(
        bad.is_empty(),
        "{} of {compared} dumps differ:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    eprintln!(
        "T3 (dump): {files} files, {compared} modes, {bytes} bytes, \
         {aborts} budget aborts, {narrated} narration-only, 0 differences"
    );
    assert!(
        compared >= 250 && aborts >= 10,
        "only {compared} modes / {aborts} aborts compared"
    );
    // The relaxation has to be *load-bearing*, or it is not a decision about
    // anything: if no dump differs before it is applied, D3 stopped showing up
    // here and the cut should go rather than sit there unexamined.
    assert!(
        ein_parity::strict() || narrated > 0,
        "no dump needed the narration cut — D3 no longer reaches this test"
    );
}

/// The narration cut, applied to both sides — or nothing at all under
/// `EIN_PARITY_STRICT=1`, which is the pre-S1a.6.9 byte contract.
fn narrow(a: &str, b: &str) -> (String, String) {
    if ein_parity::strict() {
        return (a.to_string(), b.to_string());
    }
    (
        ein_parity::blank_blocks(a, "=== "),
        ein_parity::blank_blocks(b, "=== "),
    )
}

/// `dump_shape` elides the `enterings/` subtree where it is produced, so its
/// marker is the one thing about the rule that lives outside `ein-parity`.
/// This is what keeps that from drifting.
#[test]
fn the_produced_marker_is_the_one_the_contract_expects() {
    assert_eq!(ein_parity::NARRATED, "<narrated>");
}

fn brief(a: &Answer) -> String {
    match a {
        Answer::Ok(s) => format!("{} lines", s.lines().count()),
        Answer::Err { kind, msg } => format!("{kind}: {msg}"),
    }
}

/// The first differing line, with three lines of leading context.
fn first_difference(a: &str, b: &str) -> String {
    let (a, b): (Vec<&str>, Vec<&str>) = (a.lines().collect(), b.lines().collect());
    for i in 0..a.len().max(b.len()) {
        let (x, y) = (a.get(i), b.get(i));
        if x != y {
            let mut out: Vec<String> = ((i.saturating_sub(3))..i)
                .map(|j| format!("     {}", a[j]))
                .collect();
            out.push(format!("  rs {}", x.unwrap_or(&"<end>")));
            out.push(format!("  py {}", y.unwrap_or(&"<end>")));
            return format!("  line {}:\n{}", i + 1, out.join("\n"));
        }
    }
    "  (no line differs — trailing newline?)".to_string()
}
