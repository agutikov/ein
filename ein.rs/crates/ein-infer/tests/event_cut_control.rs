//! **The D3 event cut's negative control** — T1a.10.3.3.
//!
//! [S1a.6.10](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.10_parity_contract.md)
//! narrowed the event comparison from "the whole stream" to "what each segment
//! derived", because a fork resumes root's saturation rather than re-deriving
//! it and the two engines therefore narrated different amounts of the same
//! derivation
//! ([D3](../../../../plans/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)).
//! **A relaxation that cannot be shown to still catch the thing it was relaxed
//! around is a hole rather than a decision**, so the cut has always had a
//! control: [`utils/mutant_ein.py`](../../../../utils/mutant_ein.py), a wrapper
//! that ran the shipping binary and then deleted one event from the log it
//! wrote, which the gate had to report.
//!
//! That wrapper needed two processes and a harness to be the second operand.
//! Both are gone
//! ([S1a.10.3](../../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.3_corpus_without_an_oracle.md)),
//! and the control does not need either: the mutation was always applied to
//! the *artefact*, so the only thing the processes bought was a way to produce
//! one. Here the stream is produced in-process and mutated in memory, and the
//! three mutations are the script's three, unchanged:
//!
//! | delete | the cut must |
//! |---|---|
//! | the first `fire` with `redundant = false` | **report it** — a derivation went missing |
//! | the first `fire` with `redundant = true` | stay silent — that is the narration it elides |
//! | the first `enqueue` | stay silent — likewise |
//!
//! The last two are the *positive* controls: a cut that reported them would
//! still be comparing narration, and D3 would still be costing it 97 of 240
//! cells. Run all three and the relaxation is calibrated in both directions
//! rather than asserted.
//!
//! This is the ledger's
//! [§3.7](../../../../plans/m1a_rust/p1a.10_single_implementation/oracle_ledger.md#3-the-instruments-that-are-not-tiers)
//! row, which is why `ein-parity`'s `events` module survives a phase that
//! retired the harness it was written for.

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::events::{Buffer, Events, Level};
use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
use ein_ir::{Ast, parse};
use serde_json::Value;

/// The same three fixtures `golden_events.rs` pins, for the same reason: they
/// are the smallest set that emits the whole elided vocabulary. A control that
/// ran on a stream with no redundant firings would pass its second row by
/// having nothing to delete.
const STREAMS: [(&str, bool); 3] = [
    ("examples/features/06_symmetric_native.ein", false),
    ("examples/branching/12_typed_blind_solve.ein", false),
    ("examples/domain_elim/ab.ein", false),
];

/// One fixture's whole `--events` stream at `Level::Verbose`, parsed.
fn stream(rel: &str, exhaustive: bool) -> Vec<Value> {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).expect("the fixture is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("the fixture parses");
    let mut kb =
        ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).expect("the fixture loads");
    let buf = Buffer::new();
    let mut events = Events::to(Box::new(buf.clone()), Level::Verbose);
    let opts = SolveOptions {
        stop_after: if exhaustive { None } else { Some(1) },
        max_set_size: 5,
        on_budget: OnBudget::Verdict,
        ..SolveOptions::default()
    };
    solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
        .expect("the fixture solves");
    buf.to_string_lossy()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("one JSON object per line"))
        .collect()
}

/// `mutant_ein.py`'s three predicates, by name.
fn picked(which: &str, e: &Value) -> bool {
    let kind = e.get("e").and_then(Value::as_str).unwrap_or("");
    let redundant = e.get("redundant").and_then(Value::as_bool) == Some(true);
    match which {
        "productive" => kind == "fire" && !redundant,
        "redundant" => kind == "fire" && redundant,
        "enqueue" => kind == "enqueue",
        other => unreachable!("unknown mutation {other}"),
    }
}

/// Delete the first matching event. `None` when the stream has none, which is
/// "this mutation does not apply here" rather than a failure — the floors at
/// the end of the test are what make sure it applied somewhere.
fn mutate(log: &[Value], which: &str) -> Option<Vec<Value>> {
    let at = log.iter().position(|e| picked(which, e))?;
    let mut out = log.to_vec();
    out.remove(at);
    Some(out)
}

#[test]
fn the_cut_reports_a_dropped_derivation_and_ignores_dropped_narration() {
    // `(mutation, must the cut report it?)`
    const CASES: [(&str, bool); 3] = [
        ("productive", true),
        ("redundant", false),
        ("enqueue", false),
    ];
    let mut applied: Vec<usize> = vec![0; CASES.len()];
    let mut bad: Vec<String> = Vec::new();

    for (rel, exhaustive) in STREAMS {
        let log = stream(rel, exhaustive);
        // A control over an unchanged stream first: if a log did not agree
        // with itself, every verdict below would be noise.
        assert!(
            ein_parity::events::diff(&log, &log).is_empty(),
            "{rel}: a stream does not compare equal to itself"
        );
        for (i, (which, must_report)) in CASES.iter().enumerate() {
            let Some(mutated) = mutate(&log, which) else {
                continue;
            };
            applied[i] += 1;
            let report = ein_parity::events::diff(&log, &mutated);
            if report.is_empty() == *must_report {
                bad.push(format!(
                    "  {rel} :: delete the first `{which}` event — the cut {} it, \
                     and must {}:\n    {}",
                    if report.is_empty() {
                        "missed"
                    } else {
                        "reported"
                    },
                    if *must_report { "report" } else { "not" },
                    report.join("\n    "),
                ));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    for (i, (which, _)) in CASES.iter().enumerate() {
        assert!(
            applied[i] > 0,
            "no fixture emitted a `{which}` event, so that row was never tested"
        );
    }
}

/// **Elided is counted, not ignored.** The other half of the cut's contract:
/// a redundant firing does not reach the comparison, but the run still says
/// how much of it there was, so "b emitted no `park` events at all" stays a
/// sentence someone can read.
///
/// Without this, `diff` staying silent on the second and third mutations above
/// would be indistinguishable from the parser dropping those lines on the
/// floor.
#[test]
fn a_deleted_narration_event_still_moves_the_elided_count() {
    let mut checked = 0usize;
    for (rel, exhaustive) in STREAMS {
        let log = stream(rel, exhaustive);
        let before = ein_parity::events::split(&log).elided_total();
        for which in ["redundant", "enqueue"] {
            let Some(mutated) = mutate(&log, which) else {
                continue;
            };
            let after = ein_parity::events::split(&mutated).elided_total();
            assert_eq!(
                after + 1,
                before,
                "{rel}: deleting one `{which}` event left the elided total at {after}"
            );
            checked += 1;
        }
    }
    assert!(checked >= 3, "only {checked} elided-count checks ran");
}
