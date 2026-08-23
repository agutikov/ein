//! T1a.7.2.1 — **the seam a worker crosses**, without a thread yet.
//!
//! [S1a.7.2](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.2_parallel_enterings.md)
//! fans a layer out by handing every worker the same root and letting each run
//! [`try_commitment_set`] against it. Four things have to be true before that
//! is a design, and all four are properties of *types* rather than of threads,
//! which is why they are checked here and not behind a job count:
//!
//! 1. **Root is `&`.** [`Kb::branch`] takes a shared reference, so the claim
//!    the commitment primitive has always made in prose — that it is pure with
//!    respect to root — is the signature.
//! 2. **The intern tables are lent, not locked.** [`Terms::share`] makes them
//!    unwritable for as long as a worker holds a view, and an entering that
//!    would have assigned an id gets [`Overflow::Shared`] instead of a race.
//! 3. **A worker's records are its own**, and travel back with its result, so
//!    the ordered commit can read the derivations it is about to narrate.
//! 4. **The tables come back.** [`Terms::reclaim`] is what lets the next
//!    layer's hypothesis generator intern again.
//!
//! What is *not* here is the fan-out. These run on one thread, deliberately: a
//! seam that only works under a scheduler is a seam whose failures are
//! timing-dependent.

use ein_core::intern::Overflow;
use ein_core::{Kb, Terms, Value};
use ein_infer::commitment::try_commitment_set;
use ein_infer::compile::SharedMemo;
use ein_infer::events::{self, Events};
use ein_ir::{Ast, load_file};
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

fn load(rel: &str) -> (Ast, Terms, Kb) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, &repo_root().join(rel)).expect("loads");
    (ast, terms, kb)
}

/// A hypothesis out of the file's **own** vocabulary — `(color-loc Red
/// House-1)` — looked up rather than invented, so an entering over it is an
/// entering the search itself could have made.
fn hypothesis(terms: &mut Terms, rel: &str, args: &[&str]) -> ein_core::FactId {
    let rel = terms
        .syms
        .get(rel)
        .unwrap_or_else(|| panic!("{rel} is not a name this file uses"));
    let args: Vec<Value> = args
        .iter()
        .map(|&a| {
            Value::sym(
                terms
                    .syms
                    .get(a)
                    .unwrap_or_else(|| panic!("{a} is not a name this file uses")),
            )
        })
        .collect();
    terms.intern_fact(rel, &args).expect("room")
}

/// What an entering produced, in a form two runs can be compared by.
#[derive(PartialEq, Eq, Debug)]
struct Outcome {
    kind: &'static str,
    firings: usize,
    facts: usize,
    core: Vec<String>,
}

fn outcome(terms: &Terms, r: &ein_infer::commitment::CommitmentSetResult) -> Outcome {
    let mut core: Vec<String> = r
        .unsat_core
        .iter()
        .map(|&f| events::sexpr(terms, f))
        .collect();
    core.sort();
    Outcome {
        kind: r.kind.as_str(),
        firings: r.firings.len(),
        facts: r.kb.n_facts(),
        core,
    }
}

/// **The seam, end to end.** The same entering, once on the committing thread
/// and once on a worker's view, is the same entering.
///
/// The worker arm is exactly the shape the fan-out has: share the tables, take
/// a view, open a region, enter, hand the region back with the result, and
/// only then read what the result derived.
#[test]
fn an_entering_on_a_worker_view_is_the_entering_the_committer_would_have_run() {
    for (rel, head, args) in [
        ("examples/zebra2.ein", "color-loc", ["Red", "House-1"]),
        ("examples/zebra.ein", "co-located", ["Red", "House-1"]),
        (
            "examples/branching/07_lookahead_off.ein",
            "co-located",
            ["Blue", "H5"],
        ),
    ] {
        let (ast, mut terms, mut kb) = load(rel);
        let h = hypothesis(&mut terms, head, &args);
        let memo = SharedMemo::default();
        let mut off = Events::off();

        // The committing thread's arm: today's path, unchanged.
        terms.provs.open_fork();
        let direct = try_commitment_set(
            kb.sealed(),
            &mut terms,
            &ast,
            &mut off,
            &memo,
            &[h],
            None,
            None,
        )
        .expect("enters");
        let expected = outcome(&terms, &direct);
        drop(direct);
        terms.provs.discard_fork();

        // The worker's arm.
        terms.share();
        assert!(terms.is_shared(), "{rel}: share() did not lend the tables");
        let region = {
            let mut view = terms.worker();
            view.provs.open_fork();
            let result = try_commitment_set(
                kb.sealed(),
                &mut view,
                &ast,
                &mut off,
                &memo,
                &[h],
                None,
                None,
            )
            .expect("a worker enters");
            let got = outcome(&view, &result);
            assert_eq!(
                expected, got,
                "{rel}: the worker's entering is not the committer's"
            );
            // Every derivation the fork believes is one of the worker's own
            // records, and they are readable through the view that made them.
            let cited: Vec<_> = result
                .kb
                .facts()
                .flat_map(|f| result.kb.justifications(f))
                .collect();
            assert!(
                cited.iter().any(|p| p.is_fork()),
                "{rel}: the fork cited no record of its own — the region is \
                 not carrying what the commit will have to narrate"
            );
            view.provs.take_fork()
        };
        assert!(!region.is_empty(), "{rel}: the worker's region is empty");

        // …and the commit can read them, by installing what the result
        // carried. Nothing else may read a fork id while it is installed,
        // which is why the region travels with the result rather than in a
        // side table.
        terms.reclaim();
        assert!(
            !terms.is_shared(),
            "{rel}: reclaim() did not take them back"
        );
        let n = region.len();
        let saved = terms.provs.swap_fork(region);
        assert_eq!(
            terms.provs.fork_is_empty(),
            n == 0,
            "{rel}: the installed region is the worker's"
        );
        let _ = terms.provs.swap_fork(saved);
    }
}

/// **What a worker cannot do**, and what happens instead of a race.
///
/// A lent table answers a *lookup* — which is why a worker can still compile a
/// plan, `intern_program_names` having interned every name a rule can name at
/// load — and refuses an *assignment*. The refusal is a value, so the fan-out
/// can hand the entering back to the committing thread; it is not a panic and
/// it is not a lock.
#[test]
fn a_lent_table_answers_a_lookup_and_refuses_an_assignment() {
    let (_ast, mut terms, _kb) = load("examples/zebra2.ein");
    let red = terms.syms.get("Red").expect("the file names it");
    let color = terms.syms.get("Color").expect("the file names it");
    let is_a = terms.syms.get("is-a").expect("the file declares it");

    terms.share();
    let mut view = terms.worker();

    assert_eq!(
        view.intern_text("Red"),
        Ok(red),
        "a lent interner still answers for a name it holds"
    );
    assert_eq!(
        view.intern_text("House-99"),
        Err(Overflow::Shared),
        "a lent interner must refuse to assign, not assign off-thread"
    );
    // The fact store is the one that matters: a fork *derives* propositions.
    // `(is-a Red Color)` is one the file states, so it has a number already.
    let known_fact = view
        .intern_fact(is_a, &[Value::sym(red), Value::sym(color)])
        .expect("a proposition the loader already numbered");
    assert_eq!(
        view.probe_fact(is_a, &[Value::sym(red), Value::sym(color)]),
        Some(known_fact)
    );
    assert_eq!(
        // The other way round is a proposition nothing had reason to number.
        view.intern_fact(is_a, &[Value::sym(color), Value::sym(red)]),
        Err(Overflow::Shared),
        "a lent fact store must refuse to number a new proposition"
    );

    // And the committing thread cannot either, while the view is alive — the
    // half that stops the *other* thread racing.
    assert_eq!(terms.intern_text("House-99"), Err(Overflow::Shared));

    drop(view);
    terms.reclaim();
    assert!(
        terms.intern_text("House-99").is_ok(),
        "the tables came back growable"
    );
}

/// Reclaiming while a view is alive is a bug in the fan-out, and it says so
/// rather than quietly leaving the tables unable to grow — which would make
/// every later entering hand itself back, for ever, at full speed.
#[test]
#[should_panic(expected = "still holds a view")]
fn reclaiming_under_a_live_view_is_loud() {
    let (_ast, mut terms, _kb) = load("examples/zebra2.ein");
    terms.share();
    let _view = terms.worker();
    terms.reclaim();
}

/// **A panic inside the lend window leaves the tables usable** — T1a.7.5.6.
///
/// `Terms::share` and `Terms::reclaim` have to come in pairs, and the window
/// between them is one function call wide today. What makes that safe is not
/// that the call cannot fail: a worker panic propagates out of `rayon`'s
/// `install` on the calling thread, and a future `?` in the same window would
/// return through it. Either way a bare `share()` would leave the tables lent
/// — not a crash, but a `Terms` that has silently stopped growing, which is
/// exactly what `reclaiming_under_a_live_view_is_loud` above is about from the
/// other side. `Terms::lend` makes the pairing the borrow checker's.
///
/// The panic is caught here because that is the only way to observe the state
/// afterwards; nothing in the engine catches one.
#[test]
fn a_panic_inside_the_lend_window_gives_the_tables_back() {
    let (_ast, mut terms, _kb) = load("examples/zebra2.ein");
    let before = terms.syms.len();
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let lent = terms.lend();
        assert!(lent.get().is_shared(), "lend() did not lend the tables");
        panic!("a worker died");
    }));
    assert!(caught.is_err(), "the panic did not propagate");
    assert!(
        !terms.is_shared(),
        "the tables are still lent after a panic — every later entering would \
         hand itself back, for ever"
    );
    // …and they can still grow, which is the half that matters.
    let fresh = terms
        .intern_text("@after-the-panic")
        .expect("a reclaimed table assigns");
    assert_eq!(terms.syms.len(), before + 1);
    assert_eq!(terms.syms.text(fresh), "@after-the-panic");
}

/// Branching an unsealed root would hand the fork a view that does not contain
/// the parent's newest facts. It is an assertion in every build, not a debug
/// one: what it prevents is a fork that silently believes less than its
/// parent.
#[test]
#[should_panic(expected = "unsealed KB")]
fn branching_without_sealing_is_loud() {
    let (_ast, _terms, kb) = load("examples/zebra2.ein");
    let _ = kb.branch();
}
