//! T1a.7.1.7 — **what a fork's derivation records outlive, and what they do
//! not.**
//!
//! The provenance arena is the shared structure
//! [design/08 §6](../../../../plans/m1a_rust/design/08_parallelism.md#6-what-must-be-sync-and-how)
//! has no row for, and the one with a real write rate: 100 % of enterings push
//! a record, and `features/01 -e` pushed **2 135 093** of them —
//! [shared_state.md §2b](../../../../plans/m1a_rust/p1a.7_parallelism/shared_state.md).
//! Whether a worker can hold its own arena instead of sharing one turns on a
//! single claim:
//!
//! > **A fork's derivation records die with the fork.**
//!
//! That claim has two halves and only one of them can be checked from the read
//! side. `ProvArena::get` panics on an id whose region has been discarded,
//! which says *nothing reads* a dead fork's record. This file asks the
//! stronger question the reclamation needs: **does anything still hold one?**
//! An id that is stored and never read trips nothing and would still be
//! addressing the wrong record, so a read-side check alone would license a
//! change that is not safe.
//!
//! ## Where the fork region begins and ends
//!
//! `Run::phase2` opens one around each entering and discards it at the three
//! points where the fork is gone — after `handle_dead`, after an
//! alive-but-unsolved entering is pushed to the next layer, and after
//! `record_node`. The **solution** path is the one that keeps a fork, and it
//! is why `Kb::promote_provenance` exists: the snapshot's citations are copied
//! into the arena proper before the region goes. So this file asks its
//! question of the recorded solutions as well as of root, because promotion is
//! exactly the step that could be incomplete.
//!
//! ## The one reader that is not a bug
//!
//! Arming the read-side assertion over the whole gate found exactly one reader
//! of a record whose fork had ended, and it turned out to be a *scan* rather
//! than a reference: `ein-einb`'s writer walks the arena end to end. Scans go
//! through `ProvArena::scan`, which is the seam that separates the two kinds
//! of read — and which no longer sees a fork's records at all.

use ein_core::{Kb, ProvId, Terms};
use ein_corpus::{corpus_files, repo_root};
use ein_infer::events::Events;
use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
use ein_ir::{Ast, parse};
use rustc_hash::FxHashSet;

const MAX_ENTERINGS: u64 = 60;

/// Every `ProvId` the KB cites for something it believes.
fn cited(kb: &Kb) -> FxHashSet<ProvId> {
    let mut live: FxHashSet<ProvId> = FxHashSet::default();
    for f in kb.facts() {
        live.extend(kb.justifications(f));
    }
    live
}

/// Nothing a finished solve believes — at root or in a recorded solution — is
/// justified by a record its fork took with it.
///
/// This is the claim the fork-local arena rests on, and it is checked from the
/// *holding* side rather than the reading side; see the module note.
#[test]
fn no_live_fact_is_justified_by_a_forks_record() {
    let (mut solved_files, mut skipped, mut checked) = (0usize, 0usize, 0usize);
    let (mut nodes, mut promoted) = (0usize, 0usize);
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
            // The solution nodes are half of what this file checks, and they
            // are only carried under `store_lattice` — the mode `--trace` and
            // `--dump-states` set.
            store_lattice: true,
            ..SolveOptions::default()
        };
        let mut events = Events::off();
        let Ok(solved) = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts) else {
            skipped += 1;
            continue;
        };
        solved_files += 1;
        let mut check = |what: &str, kb: &Kb| {
            let live = cited(kb);
            checked += live.len();
            // *Which* of several offenders names itself in the message would
            // depend on the order; that there is one does not, and the
            // assertion is the latter.
            // determinism-ok: an existence question over a set.
            if let Some(p) = live.into_iter().find(|p| p.is_fork()) {
                bad.push(format!(
                    "{}: {what} is justified by {p:?}, a record its fork took with it",
                    rel.display()
                ));
            }
        };
        check("a fact root believes", &kb);
        // Counted per file, before `terms` goes out of scope: it is what says
        // the promoting path *ran*. An assertion that no live fact cites a
        // fork's record is satisfied for the wrong reason by a promotion that
        // never happened.
        promoted += terms.provs.promoted() as usize;
        for (i, node) in solved
            .proof
            .iter()
            .flat_map(|p| p.solutions.iter())
            .enumerate()
        {
            nodes += 1;
            check(&format!("solution node {i}"), &node.kb);
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
        checked > 0,
        "no justification was examined — `is_fork` has stopped being asked"
    );
    assert!(
        nodes > 0,
        "no solution node was examined — the promoting path is untested"
    );
    assert!(
        promoted > 0,
        "{nodes} solution nodes and not one promoted record — either \
         `record_node` stopped promoting or every model came whole from root"
    );
    eprintln!(
        "provenance: {solved_files} files solved ({skipped} skipped), \
         {checked} live justifications over root and {nodes} solution nodes, \
         0 of them a fork's; {promoted} records promoted"
    );
}
