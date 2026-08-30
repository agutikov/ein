//! M1d [T1d.10.6.3](../../../../docs/history/m1d_satisfiability/README.md#s1d106--the-traversal)
//! — **the per-obligation tree**, in a process of its own.
//!
//! `obligation_rung_control.rs`'s idiom, for its reason: `EIN_TRAVERSAL` is
//! read from the process environment, so a file that sets it cannot share a
//! binary with tests that assert the default. Cargo gives each `tests/*.rs` its
//! own process, which is the cheapest serialisation there is —
//! **between files.** Inside one, cargo runs tests as parallel threads of that
//! process, and every test here needs the variable to hold a different value
//! at a different moment. So [`solve_path`] takes the traversal as an argument
//! and holds a `Mutex` across the solve: the environment is set and read under
//! the same lock, and no test can observe another's setting. Before M1e
//! S1e.2.1 the two tests below set it at their own top and raced; with two
//! tests and one slow arm each it happened not to bite, which is the shape of
//! a bug rather than the absence of one — adding a third test made it fail on
//! the first run.
//!
//! What the first two tests here are for is the stage's second acceptance
//! bullet — *"the same models, on every entry that can run both. Not 'the same
//! count' — the same fact sets"* — and its guard, which is the finding that
//! building it produced: a tree on a rung that is **not** the obligations one
//! is the depth-first solver P1.5b deleted, and it costs what that cost.
//!
//! **The rest are M1e [S1e.2.1]'s**, which found three defects in this
//! traversal and fixed two of them here: it ignored the stop policy, and it
//! recorded nothing from a dead branch, so a run that found no model returned
//! a `Contradiction` whose stated evidence was the empty set. The third —
//! re-reading the rung mode at every node rather than once at root — has no
//! test in this file on purpose, because nothing in the corpus can flip the
//! mode; constructing the flip is
//! [S1f.10.6](../../../../plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md)'s,
//! and the guard is written down here as an owed regression test.
//!
//! [S1e.2.1]: ../../../../plans/m1e_review_processing/p1e.2_high/s1e.2.1_correctness.md

use std::collections::BTreeSet;

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::events::{Buffer, Events, Level, sexpr};
use ein_infer::solve::{NoDumper, SolveOptions, Solved, solve};
use ein_infer::verdict::{Answer, Verdict};
use ein_ir::{Ast, load_file};

/// One solve, and everything the tests below read off it.
///
/// The model fact sets are strings because two runs intern into two arenas and
/// a `FactId` does not survive the crossing; the unsat core is a count for the
/// same reason plus one more — what `CO-H3`(b) was about is whether there is a
/// core at all.
struct Run {
    models: BTreeSet<Vec<String>>,
    enterings: u64,
    declined: u64,
    core: usize,
    log: String,
}

/// Serialises `EIN_TRAVERSAL` against the solve that reads it. See the module
/// header: cargo runs the tests in this file as threads of one process.
static TRAVERSAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn solve_path(path: &std::path::Path, cap: u32, stop_after: Option<u64>, traversal: &str) -> Run {
    let guard = TRAVERSAL.lock().unwrap_or_else(|e| e.into_inner());
    // SAFETY: the lock above is the only writer, and it is held across the
    // solve that reads the variable.
    unsafe { std::env::set_var("EIN_TRAVERSAL", traversal) };
    let buffer = Buffer::new();
    let mut events = Events::to(Box::new(buffer.clone()), Level::Normal);
    let name = path.display();
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, path).unwrap_or_else(|e| panic!("{name}: {e}"));
    let opts = SolveOptions {
        config: Some(kb.program().config.clone().unwrap_or_default()),
        stop_after,
        max_set_size: cap,
        ..SolveOptions::default()
    };
    let solved: Solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
        .unwrap_or_else(|e| panic!("{name} solves: {e:?}"));
    let mut models = BTreeSet::new();
    let mut core = 0usize;
    match &solved.answer {
        Answer::Verdict(Verdict::Ambiguity(branches)) => {
            for b in branches.iter() {
                let mut v: Vec<String> = b.kb.facts().map(|f| sexpr(&terms, f)).collect();
                v.sort();
                models.insert(v);
            }
        }
        Answer::Verdict(Verdict::Solution(s)) => {
            let mut v: Vec<String> = s.kb.facts().map(|f| sexpr(&terms, f)).collect();
            v.sort();
            models.insert(v);
        }
        Answer::Verdict(Verdict::Contradiction { unsat_core }) => core = unsat_core.len(),
        _ => {}
    }
    let log = buffer.to_string_lossy();
    let declined = log
        .lines()
        .filter(|l| l.contains("\"traversal\"") && l.contains("declined"))
        .count() as u64;
    drop(guard);
    Run {
        models,
        enterings: solved.stats.base.enterings_total,
        declined,
        core,
        log,
    }
}

/// Solve a corpus entry, and return its model set as text plus the run's
/// counters.
fn run(rel: &str, cap: u32, traversal: &str) -> (BTreeSet<Vec<String>>, u64, u64) {
    let r = solve_path(&repo_root().join(rel), cap, None, traversal);
    (r.models, r.enterings, r.declined)
}

/// **The tree finds the lattice's models, fact for fact, and enters 567× less
/// to do it.**
///
/// `examples/zebra2-minus-15-obligations.ein` is the entry the phase is named
/// after and the only corpus program with a model set large enough for the
/// comparison to mean anything: 32 models, which the lattice reaches at depth 3
/// after **48 745** enterings and cannot certify without 17 204 592.
///
/// **The lattice arm runs at `-m 2`, and the asymmetry is affordability rather
/// than a weaker claim.** All 32 need depth 3 and 48 745 enterings, which a
/// debug build does not survive; depth 2 has **28** of them for 4 656, and
/// *"every model the lattice found is one the tree found"* is what a lost model
/// would break. The tree takes no cap at all — its depth is bounded by the 46
/// instances root owes, not by `--max-set-size` — and `= 32` pins the other
/// direction, because a tree that lost four would still contain all 28.
///
/// **That the tree takes no cap is now a decision rather than an oversight**,
/// M1e S1e.2.1: `--max-set-size` bounds the size of the commitment *sets* the
/// lattice enumerates, and the tree enumerates none. Its deepest model here
/// sits at depth **6**, one past this flag's own default of 5, so reading the
/// flag as a depth cap would delete the result at stock settings. `ein solve`
/// refuses an explicit `-m` under `EIN_TRAVERSAL=tree` rather than dropping it
/// in silence; the `cap` this helper passes reaches the lattice arm and is
/// inert on the tree one, which is why the two arms can differ.
///
/// The comparison is of **fact sets**, never of `k`. Two searches that agree on
/// a count and disagree on a model are exactly the failure
/// [`completeness.md`](../../../../docs/history/m1d_satisfiability/completeness.md)
/// exists to rule out, and a count would not see it.
#[test]
fn the_tree_finds_the_lattices_models_fact_for_fact() {
    let (lattice, lat_enterings, _) = run("examples/zebra2-minus-15-obligations.ein", 2, "lattice");
    let (tree, tree_enterings, declined) =
        run("examples/zebra2-minus-15-obligations.ein", 5, "tree");

    assert_eq!(declined, 0, "the tree declined on an obligations program");
    assert_eq!(
        lattice.len(),
        28,
        "the lattice arm is not the known baseline"
    );
    assert_eq!(tree.len(), 32, "the tree is not the known baseline");
    let lost: Vec<&Vec<String>> = lattice.difference(&tree).collect();
    assert!(
        lost.is_empty(),
        "{} model(s) the lattice found are not in the tree's set",
        lost.len()
    );
    assert!(
        tree_enterings * 10 < lat_enterings,
        "the tree entered {tree_enterings} against the lattice's {lat_enterings} \
         at a cap two layers shallower than the lattice needs"
    );
}

/// **A tree on any other rung is the solver that was deleted, so it declines.**
///
/// An hrule's candidates are not one owed instance's alternatives: they are not
/// jointly exhaustive, so branching on them walks hypothesis *paths* and
/// reaches a size-`d` commitment by `d!` routes — which is what P1.5b removed
/// the tree solver for in `8d77b02` (2026-05-29), and it came back on contact.
/// Measured before the guard existed: `examples/zebra2.ein` cost **7 877**
/// enterings against the lattice's 101, for the same single model.
///
/// So the tree asks the rung first and hands the run back. What this pins is
/// that it hands it back *identically* — the entering count on an hrule program
/// is the lattice's to the digit, because the lattice is what ran.
#[test]
fn the_tree_declines_on_a_rung_that_is_not_the_obligations_one() {
    let (lat_models, lat_enterings, _) = run("examples/zebra2.ein", 5, "lattice");
    let (tree_models, tree_enterings, declined) = run("examples/zebra2.ein", 5, "tree");

    assert_eq!(declined, 1, "the decline was not narrated");
    assert_eq!(
        tree_enterings, lat_enterings,
        "a declined tree did not hand the run back unchanged"
    );
    assert_eq!(tree_models, lat_models);
    assert_eq!(lat_enterings, 101, "the hrule baseline moved");
}

// ── M1e S1e.2.1 — CO-H3 ────────────────────────────────────────────

/// **The tree stops when the stop policy says to** — `CO-H3`(a).
///
/// `ein solve` defaults to `-n 1`. This traversal used to consult
/// `check_budget` and nothing else: no `stop_after` test after `record_node`,
/// no depth cap, so `EIN_TRAVERSAL=tree ein solve file` explored and recorded
/// the **entire** tree while being asked for one model — 32 where the lattice,
/// on the same file and the same flags, said 1.
///
/// The two arms below are the same file at `-n 1` and unbounded. What makes it
/// a claim about the *stop* rather than about the answer is the entering
/// count: the search has to have been cut, not merely the read-out trimmed.
///
/// `truncated` needs no separate assertion here — `Run::tree` sets it
/// unconditionally, *not exhaustion but discharge*, so an early return is
/// already reported as a lower bound.
#[test]
fn the_tree_honours_the_stop_policy() {
    let path = repo_root().join("examples/zebra2-minus-15-obligations.ein");
    let all = solve_path(&path, 5, None, "tree");
    let one = solve_path(&path, 5, Some(1), "tree");

    assert_eq!(all.models.len(), 32, "the unbounded baseline moved");
    assert_eq!(all.enterings, 86, "the S1d.10.6 headline moved");
    assert_eq!(one.models.len(), 1, "-n 1 did not stop the tree");
    assert!(
        one.enterings < all.enterings,
        "-n 1 trimmed the answer without cutting the search: \
         {} enterings against {}",
        one.enterings,
        all.enterings
    );
    // The model it stopped on is one of the models it would have found.
    assert!(
        all.models
            .contains(one.models.iter().next().expect("one model")),
        "the model returned at -n 1 is not in the exhaustive set"
    );
    // And the accepted traversal says what it took of the policy, which is
    // the other half of (a): `--max-set-size` is refused by `ein solve` and
    // reaches a library caller as this line.
    assert!(
        all.log.contains(r#""verdict": "accepted""#)
            && all
                .log
                .contains("not applicable — depth is bounded by discharge"),
        "the tree did not narrate what it takes of the stop policy:\n{}",
        all.log
    );
}

/// **A `Contradiction` from the tree states the commitments it refuted** —
/// `CO-H3`(b).
///
/// The dead arm bumped counters and called `dumper.entering`, and did none of
/// the three things the lattice's `handle_dead` does. Two of those change the
/// search — the learned clause and the `(not h)` writeback — and the third,
/// pushing the commitment onto `lstate.dead`, changes only what the answer may
/// say. So `finalise` unioned over an empty list and the table printed
/// *refuted so far (0 facts)* for a run that had refuted two.
///
/// The fixture is written here rather than checked in because reaching this
/// arm needs `EIN_TRAVERSAL=tree`, which no corpus entry sets, and because the
/// claim is a **comparison**: the same file under both traversals must report
/// the same core. Ann owes a Food, both Foods refute themselves, and
/// pre-branch lookahead is off so that the refutation happens *inside* the
/// search rather than before it — with lookahead on, the kill cache empties
/// `alive` in Phase 1 and neither traversal ever enters anything.
#[test]
fn a_dead_tree_branch_records_the_core_it_refuted() {
    let dir = std::env::temp_dir().join(format!("ein-tree-core-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    let path = dir.join("every_choice_refutes_itself.ein");
    std::fs::write(
        &path,
        "(config :enable-pre-branch-lookahead false)\n\
         (import std.algebra :symbols (total-owed))\n\
         (relation is-a  T T)\n\
         (relation likes Person Food)\n\
         (is-a Ann Person)\n\
         (is-a Soup Food) (is-a Stew Food)\n\
         (total-owed likes is-a)\n\
         (rule self-refuting ()\n  \
           :match  (likes ?p ?f)\n  \
           :assert (not (likes ?p ?f))\n  \
           :why    \"every choice refutes itself\")\n\
         (query :goal (likes ?who ?what))\n",
    )
    .expect("the fixture is written");

    let lattice = solve_path(&path, 5, None, "lattice");
    let tree = solve_path(&path, 5, None, "tree");

    assert!(tree.models.is_empty() && lattice.models.is_empty());
    assert_eq!(lattice.core, 2, "the lattice baseline is not two facts");
    assert_eq!(
        tree.core, lattice.core,
        "the tree reported a {}-fact core where the lattice reported {}",
        tree.core, lattice.core
    );
    assert_eq!(
        tree.enterings, lattice.enterings,
        "recording the refutation changed what the search did"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
