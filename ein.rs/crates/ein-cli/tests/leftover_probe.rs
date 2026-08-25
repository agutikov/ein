//! M1d S1d.3.1 — **the leftover-open probe, and the claim that it is a read.**
//!
//! One test in a file of its own, for
//! [`obligation_rung_control.rs`](../../ein-infer/tests/obligation_rung_control.rs)'s
//! reason and by its rule: `EIN_LEFTOVER` is read from the process
//! environment, so nothing that asserts the default may share a binary — or a
//! `--test-threads` — with something that sets it. Cargo gives each
//! `tests/*.rs` its own process; one `#[test]` inside it is the rest of the
//! serialisation, and it is why the three claims below are three sections
//! rather than three functions.
//!
//! What the probe answers is the question `complete` cannot be asked. A
//! solution node is one the *active* rung proposed nothing at, and the active
//! rung is whichever of the three the program earned; the **blind** enumerator
//! may still have candidates there, and their number is the state's
//! leftover-open count. P1d.2 handed the measurement forward rather than
//! taking it, because a blind pass over the live node writes `(not h)` per
//! lookahead kill and would move the node's `state_key`. Running it on a fork
//! is what makes it a read — and *that* is the claim worth a test, because it
//! is the one that would fail silently.

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::events::Events;
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_ir::{Ast, load_file};
use serde_json::Value as J;

/// One entry's summary, exhaustively, as `--json-summary` writes it.
fn summary(rel: &str, max_set_size: u32) -> J {
    let path = repo_root().join(rel);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let config = kb.program().config.clone().unwrap_or_default();
    let opts = SolveOptions {
        stop_after: None,
        max_set_size,
        config: Some(config.clone()),
        ..SolveOptions::default()
    };
    let mut events = Events::off();
    let mut solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
        .unwrap_or_else(|_| panic!("{rel}: solve"));
    let json = ein_cli::summary::build(
        &ast,
        &mut terms,
        &mut kb,
        &mut solved.answer,
        &solved.stats,
        &config,
        rel,
        &mut events,
        &solved.owes,
    )
    .unwrap_or_else(|e| panic!("{rel}: {e}"));
    serde_json::from_str(&ein_render::dump::json::dumps_indent(&json)).expect("summary is JSON")
}

fn counts(summary: &J, key: &str) -> Vec<u64> {
    summary["leftover"][key]
        .as_array()
        .expect("leftover array")
        .iter()
        .map(|n| n.as_u64().expect("a count"))
        .collect()
}

#[test]
fn the_leftover_probe_is_off_by_default_is_a_read_and_means_what_it_says() {
    // ── 1. Off unless it is on, and emitted either way ──────────────
    //
    // A field that appears only sometimes is a field a consumer has to guess
    // about — the rule the `owes` and `config` blocks already follow — so
    // `taken` carries the state and the two arrays are empty rather than
    // absent. Read before any write to the environment.
    let closed_default = summary("examples/branching/12_typed_blind_solve.ein", 10);
    assert_eq!(closed_default["leftover"]["taken"], J::Bool(false));
    assert!(counts(&closed_default, "models").is_empty());
    assert!(counts(&closed_default, "open_states").is_empty());

    let before = summary("examples/branching/06_lookahead_on.ein", 10);

    // SAFETY: this binary holds exactly one test, so nothing else in the
    // process reads the environment while it is written. `EIN_LEFTOVER` is
    // read once per summary, so the write has to land before `build` and not
    // during it.
    unsafe { std::env::set_var("EIN_LEFTOVER", "1") };
    let after = summary("examples/branching/06_lookahead_on.ein", 10);
    let closed = summary("examples/branching/12_typed_blind_solve.ein", 10);
    let open = summary("examples/zebra2.ein", 5);
    unsafe { std::env::remove_var("EIN_LEFTOVER") };

    // ── 2. The probe changes nothing but its own block ──────────────
    //
    // The property that makes it a measurement rather than a different
    // engine, taken on a multi-model entry so that the model *set* is in
    // scope: the walk writes into its fork, and if the fork were the node the
    // writes would move `state_key`, the dedup, and therefore `k`. Everything
    // outside `leftover` — the verdict, every counter, the root block, the
    // owes report — is compared whole, so a probe that leaks is caught
    // wherever it leaks.
    assert_eq!(after["leftover"]["taken"], J::Bool(true), "the lever took");
    let mut a = before.clone();
    let mut b = after.clone();
    a.as_object_mut().expect("object").remove("leftover");
    b.as_object_mut().expect("object").remove("leftover");
    assert_eq!(a, b, "the probe moved something outside its own block");
    assert_eq!(
        counts(&after, "models").len() as u64,
        after["verdict"]["k"].as_u64().expect("k"),
        "one row per model, index-aligned with verdict.solutions"
    );

    // ── 3. What the count means ─────────────────────────────────────
    //
    // The two shapes a corpus entry comes in, and the contrast is the whole
    // point of the number:
    //
    // * `branching/12_typed_blind_solve` closes its one relation, so at each
    //   of its two models every candidate the blind enumerator can build is
    //   already a fact or already negated. **Zero** — the model is a complete
    //   assignment, and the open- and closed-world readings agree on it.
    // * `zebra2` does not, and its **unique** model still leaves thousands of
    //   atoms undecided: `next-to`, `is-a*`, and every cross-type pair the
    //   type-blind enumerator will build. Under an open-world reading that
    //   model is 2ⁿ models; the puzzle means the closed-world one, and nothing
    //   in the file says so.
    //
    // Neither half is a golden — the bound is `> 1000`, not the 3 678 this
    // engine reports today.
    let rows = counts(&closed, "models");
    assert_eq!(
        rows.len() as u64,
        closed["verdict"]["k"].as_u64().expect("k")
    );
    assert!(
        rows.iter().all(|&n| n == 0),
        "a closed domain leaves nothing open: {rows:?}"
    );
    let rows = counts(&open, "models");
    assert_eq!(rows.len(), 1, "zebra2 has one model");
    assert!(
        rows[0] > 1000,
        "an open domain leaves the enumerator plenty: {rows:?}"
    );
}
