//! [`docs/kernel/inference/events.md`] — the `--events` schema, pinned to its
//! emitters.
//!
//! M1e [S1e.3.7](../../../../plans/m1e_review_processing/p1e.3_medium/s1e.3.7_code_doc_consistency.md)
//! `CD-M2`. The page sells itself as the schema an external observer codes to
//! — [M20](../../../../plans/m20_gui/README.md)'s likely feed — and it had **no
//! mechanical check against the emitters at all**. What that cost, in one
//! review: `admit` was documented as carrying a `watched` field it has never
//! carried, and a whole kind (`warn`, in the stream since M1e S1e.2.3) had no
//! row.
//!
//! This is the cheap half of the check, and it is deliberately the *set* of
//! kinds rather than their payloads. A payload checker would have to model
//! `EventLine`'s builder to know which `l.str` / `l.num` calls a closure
//! makes, and the closure is where the conditional fields live (`traversal`
//! carries `depth` only when a node declines). The kind set is a grep, it is
//! total, and it is the axis on which the page was actually wrong: a missing
//! row is invisible to every reader, where a wrong field at least appears
//! beside a right one.
//!
//! Two directions, because they fail for different reasons:
//!
//! - an emitter with no row — a kind was added and the page was not
//!   ([`every_emitted_kind_has_a_row`]);
//! - a row with no emitter — a kind was removed or renamed and the row
//!   outlived it ([`every_row_has_an_emitter`]).
//!
//! [`docs/kernel/inference/events.md`]: ../../../../docs/kernel/inference/events.md

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_corpus::repo_root;

/// The crates that narrate. `ein-render` is **excluded on purpose**: its
/// `dump/serialise.rs` has an `emit` of its own, for the state-dump JSON, and
/// it is not this protocol.
const NARRATING_CRATES: [&str; 2] = ["ein-infer", "ein-cli"];

fn page() -> String {
    std::fs::read_to_string(repo_root().join("docs/kernel/inference/events.md"))
        .expect("docs/kernel/inference/events.md")
}

fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("a crate source directory") {
        let path = entry.expect("a directory entry").path();
        if path.is_dir() {
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// The literal first argument of every `…emit("kind", …)`, plus the two
/// `emit_boundary` call sites, which pass the kind through.
///
/// The one thing this over-collects is a kind emitted only from a `#[cfg(test)]`
/// block — `events.rs`'s round-trip test writes a `fire` line. That is harmless
/// while the kind is real, and if a test ever invents one the failure says so
/// in the right direction: **do not name a kind in a test that the engine does
/// not emit.**
fn emitted_kinds() -> BTreeSet<String> {
    let mut files = Vec::new();
    for krate in NARRATING_CRATES {
        rust_sources(
            &repo_root().join("ein.rs/crates").join(krate).join("src"),
            &mut files,
        );
    }
    assert!(files.len() > 10, "the source scan found almost nothing");

    let mut kinds = BTreeSet::new();
    let mut non_literal = Vec::new();
    for path in &files {
        let src = std::fs::read_to_string(path).expect("a source file");
        for line in src.lines() {
            // A comment is not a call site, and `events.rs`'s own module doc
            // spells `events.emit(...)` to explain the guard around it.
            if line.trim_start().starts_with("//") {
                continue;
            }
            for (call, arg) in [(".emit(", 0usize), ("emit_boundary(", 1)] {
                let Some(at) = line.find(call) else { continue };
                let rest = &line[at + call.len()..];
                // `emit_boundary(s, "park", …)` — skip the receiver.
                let rest = match arg {
                    0 => rest,
                    _ => match rest.split_once(',') {
                        Some((_, tail)) => tail.trim_start(),
                        None => continue,
                    },
                };
                match rest.strip_prefix('"').and_then(|r| r.split_once('"')) {
                    Some((kind, _)) => {
                        kinds.insert(kind.to_string());
                    }
                    // A non-literal kind cannot be read by grep. There is one
                    // — `emit(kind, …)` inside `emit_boundary`, whose two
                    // callers are matched above — and a second would make this
                    // check quietly incomplete, so it is an error rather than
                    // a skip.
                    // The **file**, not the line: this used to bank
                    // `saturator.rs:1586` and broke on any edit above it —
                    // M1e S1e.4.8 moved it to :1623 by growing a doc comment
                    // thirty lines earlier, which is a test failing for a
                    // reason unrelated to what it checks. What the check needs
                    // is *there is exactly one, and it is the known one*.
                    None if !rest.starts_with(')') => non_literal.push(
                        path.strip_prefix(repo_root())
                            .unwrap_or(path)
                            .display()
                            .to_string(),
                    ),
                    None => {}
                }
            }
        }
    }
    assert_eq!(
        non_literal,
        ["ein.rs/crates/ein-infer/src/saturator.rs"],
        "an `emit` with a non-literal kind that this scan cannot read. Either \
         pass a literal, or route it through a helper whose call sites do — as \
         `emit_boundary` does for park/retire."
    );
    kinds
}

/// The kinds § Events names, read off the three schema tables.
///
/// A table is identified by its header row rather than by a section heading,
/// so a fourth layer needs no change here. The first cell can name several
/// kinds (`` `park` / `retire` ``), so every backticked token in it counts.
fn documented_kinds() -> BTreeSet<String> {
    const HEADER: &str = "| `e` | emitted at | payload |";
    let page = page();
    let mut kinds = BTreeSet::new();
    let mut tables = 0;
    for table in page.split(HEADER).skip(1) {
        tables += 1;
        for line in table.lines().skip(1) {
            if line.starts_with("|---") {
                continue; // the separator row
            }
            let Some(cell) = line.strip_prefix("| ") else {
                break;
            };
            let cell = cell.split('|').next().expect("a first cell");
            for tok in cell.split('`').skip(1).step_by(2) {
                kinds.insert(tok.to_string());
            }
        }
    }
    assert_eq!(tables, 3, "the page's schema tables moved or were renamed");
    kinds
}

/// Every kind the emitters produce has a row on the page.
///
/// This is the direction that failed in M1e: `warn` had been emitted since
/// S1e.2.3 and named only in § Comparison's parity spine, which is a list of
/// what is *diffed*, not a schema.
#[test]
fn every_emitted_kind_has_a_row() {
    let missing: Vec<_> = emitted_kinds()
        .difference(&documented_kinds())
        .cloned()
        .collect();
    assert!(
        missing.is_empty(),
        "emitted but undocumented: {missing:?}. Add a row to \
         docs/kernel/inference/events.md § Events — the page claims to be every \
         step the engine took, and a consumer cannot code to a kind it cannot see."
    );
}

/// Every kind the page names is still emitted.
///
/// The failure this catches is the reverse rot: a kind renamed or dropped, and
/// the row left behind for an observer to wait forever on.
#[test]
fn every_row_has_an_emitter() {
    let orphaned: Vec<_> = documented_kinds()
        .difference(&emitted_kinds())
        .cloned()
        .collect();
    assert!(
        orphaned.is_empty(),
        "documented but never emitted: {orphaned:?}. Either the emitter was \
         renamed — fix the row — or the kind is gone, and the row goes with it."
    );
}

/// The count, stated once so a reader of either side has a number to check
/// against. It is a floor and an exact value at the same time: both tests
/// above already force the two sets equal, so this only guards against both
/// halves shrinking together.
#[test]
fn the_protocol_has_twenty_two_kinds() {
    assert_eq!(emitted_kinds().len(), 22, "{:?}", emitted_kinds());
}
