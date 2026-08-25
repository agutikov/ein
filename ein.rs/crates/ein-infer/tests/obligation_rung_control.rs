//! M1d S1d.2.5 — **the ladder's control arm**, in a process of its own.
//!
//! One test, one file, and the file is the isolation: `EIN_OBLIGATION_CHOICE`
//! is read once per generation call from the process environment, so a test
//! that sets it cannot share a binary with tests that assert the default.
//! Cargo gives each `tests/*.rs` its own process, which is the cheapest
//! serialisation there is.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::events::{Buffer, Events, Level};
use ein_infer::oblgen::Choice;
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_ir::{Ast, load_file};

/// The distinct `rung` modes a run narrates, plus layer 1's `alive`.
fn probe(rel: &str) -> (BTreeSet<String>, u64) {
    let buffer = Buffer::new();
    let mut events = Events::to(Box::new(buffer.clone()), Level::Normal);
    let path: PathBuf = repo_root().join(rel);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let opts = SolveOptions {
        config: Some(kb.program().config.clone().unwrap_or_default()),
        max_set_size: 1,
        max_enterings: Some(1),
        ..SolveOptions::default()
    };
    // The budget is the point: the blind arm does not finish, and layer 1's
    // census row is emitted however the layer ends.
    let _ = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts);
    let (mut modes, mut alive) = (BTreeSet::new(), 0);
    for line in buffer.to_string_lossy().lines() {
        let ev: serde_json::Value = serde_json::from_str(line).expect("event line");
        match ev["e"].as_str() {
            Some("rung") => {
                modes.insert(ev["mode"].as_str().unwrap_or("?").to_string());
            }
            Some("layer") if ev["layer"] == 1 => alive = ev["alive"].as_u64().unwrap_or_default(),
            _ => {}
        }
    }
    (modes, alive)
}

/// **The control arm reproduces the engine as it was**, which is what makes
/// the model-set comparison in `obligation_rung.rs` a comparison of one thing.
///
/// `EIN_OBLIGATION_CHOICE=off` declines every call, so the ladder collapses to
/// its pre-S1d.2.5 shape: `zebra2-obligations` falls to the blind enumerator
/// and layer 1 goes from **56 candidates to 3 734** — 66.7× — which is the
/// size of the branch the theory bought. Read at layer 1 rather than as a wall
/// clock because the blind arm does not finish.
///
/// The env var is deliberately not a `SolverConfig` field: the config is
/// rendered into the KB-shape digest, and a knob whose two settings are being
/// *compared* would re-bless every shape golden in the corpus to record a
/// default nobody has chosen yet.
#[test]
fn the_control_arm_is_the_blind_enumerator() {
    let (modes, alive) = probe("examples/zebra2-obligations.ein");
    assert_eq!(
        modes,
        BTreeSet::from(["obligations".to_string()]),
        "the default arm branches on the obligations"
    );
    assert_eq!(alive, 56, "the theory's branch at layer 1");

    // SAFETY: this binary holds one test, so nothing else is reading the
    // environment while it is written. `Choice` is read once per generation
    // call, so the write has to land before `solve` and not during it.
    unsafe { std::env::set_var("EIN_OBLIGATION_CHOICE", "off") };
    assert_eq!(Choice::from_env(), Choice::Off, "the lever did not take");
    let (modes, blind) = probe("examples/zebra2-obligations.ein");
    unsafe { std::env::remove_var("EIN_OBLIGATION_CHOICE") };

    assert_eq!(
        modes,
        BTreeSet::from(["declined".to_string()]),
        "the control arm must decline every call"
    );
    assert_eq!(blind, 3_734, "the blind enumerator's branch at layer 1");
    assert!(
        blind > alive * 60,
        "{blind} against {alive} is not the gap the stage measured"
    );
}
