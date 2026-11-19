//! **A non-monotone program loads, and no licence is used on it**
//! ([Q-M1f.564](../../../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md),
//! [03-absent-and-naf.md](../../../../docs/ein/reasoning/02-kb-evolution/03-absent-and-naf.md#a-refutation-resting-on-an-absent)).
//!
//! A rule that concludes `(false)` or `(not …)` resting on an `(absent P)`
//! over a relation a commitment can still extend makes `dead` not
//! upward-closed: a state that has not decided `P` is dead, and a commitment
//! that decides it can be alive. A route that took a superset of a dead set
//! for dead would lose that state, so what matters of such a program is that
//! **the static check flags it**: a flagged program's routes are the cube's,
//! which skip nothing by a theorem about the program.
//!
//! This file holds that, and nothing of what such a program answers: of a
//! flagged program's answer nothing is claimed.

use std::path::Path;

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::events::Events;
use ein_infer::explore::{self, Ask, Licence, Mode, Route, Which};
use ein_ir::{Ast, load, parse};

// The programs written for the censuses: the two activator shapes.
#[allow(dead_code)]
#[path = "../examples/shared/written.rs"]
mod written;

/// The upward-closure probe: `{(p A)}` dead and `{(p A), (q A)}` alive.
const PROBE: &str = "examples/ein-bugs/naf-upward-closure.ein";

/// **The corpus programs whose refutation rests on an `absent` a commitment
/// can fill**: directly (the probe, `naf-nowb`, `alive-empty-interlayer`,
/// the two `branching/` fixtures, the two `lattice/synth/` ones and
/// `rung-connex-import`, through `std.algebra`'s `connex`), through a fact a
/// rule derived under the `absent` (the `stale-naf-fork` pair, `d3-unsat-core`),
/// and through an activator (`activator-all-*`, and the `activator-chain-*`
/// family, whose chain first appears at a node).
const UNDER_AN_ABSENT: &[&str] = &[
    "examples/branching/13_lookahead_naf_world.ein",
    "examples/branching/14_lookahead_unjudgeable.ein",
    "examples/ein-bugs/activator-all-int.ein",
    "examples/ein-bugs/activator-all-nested.ein",
    "examples/ein-bugs/activator-all-symbol.ein",
    "examples/ein-bugs/alive-empty-interlayer.ein",
    "examples/ein-bugs/d3-unsat-core.ein",
    "examples/ein-bugs/naf-upward-closure.ein",
    "examples/ein-bugs/wrong-answers/activator-chain-0.ein",
    "examples/ein-bugs/wrong-answers/activator-chain-1.ein",
    "examples/ein-bugs/wrong-answers/activator-chain-2.ein",
    "examples/ein-bugs/wrong-answers/activator-chain-node-absent.ein",
    "examples/ein-bugs/wrong-answers/activator-chain-node-writes.ein",
    "examples/ein-bugs/wrong-answers/activator-chain-param-absent.ein",
    "examples/ein-bugs/wrong-answers/naf-nowb.ein",
    "examples/ein-bugs/wrong-answers/rung-connex-import.ein",
    "examples/ein-bugs/wrong-answers/stale-naf-fork-warn.ein",
    "examples/ein-bugs/wrong-answers/stale-naf-fork.ein",
    "examples/lattice/synth/three_components.ein",
    "examples/lattice/synth/trigger_hole.ein",
];

/// Every cell of `ein solve`'s two axes, and `--count`.
fn cells() -> Vec<Mode> {
    let mut v = Vec::new();
    for which in [Which::All, Which::Minimal, Which::Maximal] {
        for ask in [Ask::Describe, Ask::Find(1), Ask::Find(2), Ask::Single] {
            v.push(Mode { which, ask });
        }
    }
    v.push(Mode {
        which: Which::All,
        ask: Ask::Count,
    });
    v
}

/// What is wrong with how `text` (read from `at`) is answered, as lines: it
/// loads, the static check flags it, and every mode answers it by the cube
/// with the warning, or, where root is dead, by root alone (a fork only
/// adds), naming no licence that is a theorem about the program.
fn unlicensed(name: &str, text: &str, at: &Path) -> Vec<String> {
    let loaded = || {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, text, at.to_str()).map_err(|e| e.to_string())?;
        let kb = load(&mut ast, &mut terms, &forms, at.parent()).map_err(|e| e.to_string())?;
        Ok::<_, String>((kb, terms, ast))
    };
    let (mut kb, mut terms, ast) = match loaded() {
        Ok(l) => l,
        Err(e) => return vec![format!("{name}: does not load: {e}")],
    };
    let checked = match explore::check_monotone(&mut kb, &mut terms, &ast, &mut Events::off()) {
        Ok(c) => c,
        Err(e) => return vec![format!("{name}: the static check does not run: {e}")],
    };
    if checked.monotonicity.is_monotone() {
        return vec![format!(
            "{name}: the static check calls it monotone, so a route would use its licences"
        )];
    }
    let mut bad = Vec::new();
    for mode in cells() {
        let (mut kb, mut terms, ast) = loaded().expect("loaded once already");
        let a = match explore::run(&mut kb, &mut terms, &ast, &explore::Options::new(mode)) {
            Ok(a) => a,
            Err(e) => {
                bad.push(format!("{name} {mode:?}: no answer: {e}"));
                continue;
            }
        };
        if a.warning.is_none() {
            bad.push(format!("{name} {mode:?}: answered with no warning"));
        }
        let route = if a.root_dead {
            Route::DownSet
        } else {
            Route::Cube
        };
        if a.route != route {
            bad.push(format!("{name} {mode:?}: route {:?}", a.route));
        }
        let theorems: Vec<&Licence> = a
            .licences
            .iter()
            .filter(|l| !matches!(l, Licence::TheCube | Licence::ForkOnlyAdds))
            .collect();
        if !theorems.is_empty() {
            bad.push(format!("{name} {mode:?}: licences {theorems:?}"));
        }
    }
    bad
}

/// **Each of them loads, is flagged, and is answered with no licence.**
#[test]
fn a_refutation_under_an_absent_loads_flagged_and_unlicensed() {
    let mut bad = Vec::new();
    for rel in UNDER_AN_ABSENT {
        let path = repo_root().join(rel);
        let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}"));
        bad.extend(unlicensed(rel, &text, &path));
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **The two written activator programs** (`examples/shared/written.rs`),
/// which no corpus program has: a rule with parameters that reads its
/// parameter's relation under an `absent` and refutes, with no plan at root,
/// and a refutation that reads no `absent` and whose activator rests on one.
#[test]
fn the_written_activator_programs_load_flagged_and_unlicensed() {
    let at = repo_root().join("written.ein");
    let mut bad = Vec::new();
    for (name, text) in written::WRITTEN {
        bad.extend(unlicensed(name, text, &at));
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **The stream says so**: `--events` narrates root's saturation and then one
/// `monotonicity` line whose verdict is `non-monotone`, before any node.
#[test]
fn the_probes_stream_narrates_the_verdict() {
    let path = repo_root().join(PROBE);
    let text = std::fs::read_to_string(&path).expect("the probe is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("it parses");
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("it loads");
    let buf = ein_infer::events::Buffer::new();
    let mut events = Events::to(Box::new(buf.clone()), ein_infer::events::Level::Normal);
    let opts = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    });
    explore::run_with(&mut kb, &mut terms, &ast, &opts, &mut events).expect("it answers");
    let stream = buf.to_string_lossy();
    let line = stream
        .lines()
        .find(|l| l.contains("\"e\": \"monotonicity\""))
        .unwrap_or_else(|| panic!("no monotonicity line:\n{stream}"));
    assert!(line.contains("\"verdict\": \"non-monotone\""), "{line}");
    let quiesce = stream.find("\"e\": \"quiesce\"").expect("root saturates");
    assert!(
        quiesce < stream.find("\"e\": \"monotonicity\"").expect("found above"),
        "the verdict is narrated after root's saturation:\n{stream}"
    );
}

/// The stratification advisory (the library's `derived_naf_warnings`) is
/// silent on the probe: it watches an `(absent …)` over a **rule-derived**
/// relation, and `q` is only ever proposed by the generator. Two adjacent
/// shapes, and the advisory is about the other one.
#[test]
fn the_stratification_warning_does_not_cover_this() {
    let path = repo_root().join(PROBE);
    let text = std::fs::read_to_string(&path).expect("the probe is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("it parses");
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("it loads");
    let mut events = Events::off();
    let mut s = ein_infer::saturator::Session {
        kb: &mut kb,
        terms: &mut terms,
        ast: &ast,
        events: &mut events,
        memo: Default::default(),
    };
    let mut sat = ein_infer::saturator::Saturator::new(&mut s).expect("a saturator");
    sat.saturate(&mut s, None, &mut |_| {})
        .expect("it saturates");
    assert!(
        ein_infer::naf_deps::derived_naf_warnings(&sat.engine, s.terms).is_empty(),
        "the stratification warning fired on a hypothesis-eligible watch"
    );
}
