//! T1a.7.1.7 — **what a fork's derivation records outlive, and what they do
//! not.**
//!
//! The provenance arena is the shared structure
//! [design/08 §6](../../../../plans/m1a_rust/design/08_parallelism.md#6-what-must-be-sync-and-how)
//! has no row for, and the one with a real write rate: 100 % of enterings push
//! a record, and `features/01 -e` pushes **2 135 093** of them —
//! [shared_state.md §2b](../../../../plans/m1a_rust/p1a.7_parallelism/shared_state.md).
//! Whether a worker can hold its own arena instead of sharing one turns on a
//! single claim:
//!
//! > **A fork's derivation records die with the fork.**
//!
//! That claim has two halves and only one of them can be checked from the read
//! side. `ProvArena::get`'s assertion — armed in every debug build, so the
//! whole gate exercises it — says *nothing reads* a retired record. This file
//! asks the stronger question a reclamation would need: **does anything still
//! hold one?** An id that is stored and never read trips no assertion and
//! would still be corrupted by reuse, so a read-side check alone would license
//! a change that is not safe.
//!
//! ## What "retired" means, and the one reader that is not a bug
//!
//! `Run::entering` marks the range `try_commitment_set` created and hands it
//! to `ProvArena::retire` at the two points where the fork is definitively
//! gone — after `handle_dead`, and after an alive-but-unsolved entering is
//! pushed to the next layer. The **solution** path deliberately does not
//! retire: `record_node` snapshots the fork's KB, so those records are live by
//! construction.
//!
//! Arming that assertion over the gate found exactly one reader, and it turned
//! out to be a *scan* rather than a reference: `ein-einb`'s writer walks the
//! arena end to end (`write_prov` says so in as many words — it stores
//! "records no *believed* fact points at any more, which a search leaves
//! behind"). Scans now go through `ProvArena::scan`, which is the seam that
//! separates the two kinds of read.

use ein_core::{ProvId, Terms};
use ein_corpus::{corpus_files, repo_root};
use ein_infer::events::Events;
use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
use ein_ir::{Ast, parse};
use rustc_hash::FxHashSet;

const MAX_ENTERINGS: u64 = 60;

/// Nothing root believes is justified by a record a finished fork created.
///
/// This is the claim a fork-local arena rests on, and it is checked from the
/// *holding* side rather than the reading side — see the module note.
#[test]
fn no_live_fact_is_justified_by_a_retired_forks_record() {
    let (mut solved_files, mut skipped, mut checked) = (0usize, 0usize, 0usize);
    let mut bad: Vec<String> = Vec::new();
    for path in corpus_files() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_path_buf();
        let Ok(text) = std::fs::read_to_string(&path) else {
            skipped += 1;
            continue;
        };
        let mut ast = Ast::new();
        let Ok(forms) = parse(&mut ast, &text, path.to_str()) else {
            skipped += 1;
            continue;
        };
        let mut terms = Terms::new();
        let Ok(mut kb) = ein_ir::load(&mut ast, &mut terms, &forms, path.parent()) else {
            skipped += 1;
            continue;
        };
        let opts = SolveOptions {
            stop_after: None,
            max_enterings: Some(MAX_ENTERINGS),
            on_budget: OnBudget::Verdict,
            ..SolveOptions::default()
        };
        let mut events = Events::off();
        if solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts).is_err() {
            skipped += 1;
            continue;
        }
        solved_files += 1;
        let mut live: FxHashSet<ProvId> = FxHashSet::default();
        for f in kb.facts() {
            live.extend(kb.justifications(f));
        }
        checked += live.len();
        for p in live {
            if terms.provs.is_retired(p) {
                bad.push(format!(
                    "{}: root believes a fact justified by {p:?}",
                    rel.display()
                ));
                break;
            }
        }
    }
    assert!(
        solved_files >= 90,
        "only {solved_files} corpus files reached a solve ({skipped} skipped)"
    );
    assert!(
        bad.is_empty(),
        "a fork's record outlived the fork on {} file(s):\n  {}",
        bad.len(),
        bad.join("\n  ")
    );
    assert!(
        cfg!(not(debug_assertions)) || checked > 0,
        "no justification was examined — `is_retired` has stopped being asked"
    );
    eprintln!(
        "provenance: {solved_files} files solved ({skipped} skipped), \
         {checked} live justifications, 0 of them retired"
    );
}
