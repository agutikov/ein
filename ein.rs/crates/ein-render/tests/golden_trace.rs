//! S1a.5.2 acceptance — `tests/golden/trace_3step.md`, byte for byte.
//!
//! ein.py's `tests/trace/test_render.py` builds a deterministic synthetic
//! three-step trace (no diagrams, so the golden is stable) and locks its
//! markdown. That *committed* file is the fixture here; the trace is rebuilt
//! against the port's own types, which is the only interesting half — a
//! `TraceStep` in ein.rs holds an owned [`FactRef`] rather than an interned
//! id, precisely so a trace can exist without a KB, and this is where that
//! claim is exercised.

use ein_oracle::repo_root;
use ein_render::trace::ast::{FactRef, RefArg, TraceStep};
use ein_render::trace::linearize::{Reductio, Trace};
use ein_render::trace::render::{Mode, render_markdown};

fn fact(rel: &str, args: &[&str]) -> FactRef {
    FactRef {
        rel: rel.to_string(),
        args: args.iter().map(|a| RefArg::Str((*a).to_string())).collect(),
    }
}

/// `(not (color-loc Blue H1))` — the one nested reference in the fixture, and
/// the reason `FactRef::label` recurses.
fn negated(inner: FactRef) -> FactRef {
    FactRef {
        rel: "not".to_string(),
        args: vec![RefArg::Fact(inner)],
    }
}

fn synthetic_trace() -> Trace {
    let mut s1 = TraceStep::new(
        1,
        "from-condition".to_string(),
        fact("nation-loc", &["Norwegian", "H1"]),
    );
    s1.why = "By condition (10), the Norwegian lives in the first house.".to_string();
    s1.section = Some("Norwegian".to_string());
    s1.sources = vec!["condition (10)".to_string()];

    let mut s2 = TraceStep::new(
        2,
        "adjacent-via".to_string(),
        fact("color-loc", &["Blue", "H2"]),
    );
    s2.premises = vec![fact("nation-loc", &["Norwegian", "H1"])];
    s2.bindings = vec![
        ("V1".to_string(), "Norwegian".to_string()),
        ("V2".to_string(), "Blue".to_string()),
    ];
    s2.why = "The Norwegian's only neighbour is House-2, so Blue is there.".to_string();
    s2.section = Some("Blue".to_string());

    let mut s3 = TraceStep::new(
        3,
        "domain-elimination".to_string(),
        fact("color-loc", &["Yellow", "H1"]),
    );
    s3.premises = vec![negated(fact("color-loc", &["Blue", "H1"]))];
    s3.why = "Only Yellow remains for House-1.".to_string();
    s3.section = Some("Yellow".to_string());

    Trace {
        steps: vec![s1, s2, s3],
        reductios: vec![Reductio {
            summary: "Assumed {color-loc(Green, H1)} — contradicts condition (6) \
                      — refuted (dead-post)"
                .to_string(),
            commitment: "{color-loc(Green, H1)}".to_string(),
            learned_clause: "color-loc(Green, H1)".to_string(),
            diagram: None,
        }],
        summary: "Solved in 3 steps; commitment ∅ (unconditional); \
                  1 solution(s), 1 refuted."
            .to_string(),
        commitment: "∅ (unconditional)".to_string(),
        solved: true,
        n_solutions: 1,
        ..Trace::default()
    }
}

#[test]
fn the_three_step_trace_reproduces_the_committed_golden() {
    let want = std::fs::read_to_string(repo_root().join("ein.py/tests/golden/trace_3step.md"))
        .expect("the golden is checked in");
    assert_eq!(
        render_markdown(&synthetic_trace(), Mode::Engine, false),
        want
    );
}

/// The reorder pass is a *presentation* pass: same steps, grouped. ein.py
/// asserts exactly this, and it is the property that makes `--reorder` safe to
/// hand a reader.
#[test]
fn reorder_groups_by_entity_and_emits_every_step_once() {
    let trace = synthetic_trace();
    let engine = render_markdown(&trace, Mode::Engine, false);
    let reordered = render_markdown(&trace, Mode::Reorder, false);
    assert_ne!(engine, reordered);
    assert!(reordered.contains("## About Norwegian"));
    assert!(reordered.contains("## About Blue"));
    for n in 1..=3 {
        assert_eq!(
            reordered.matches(&format!("Step {n}")).count(),
            1,
            "step {n} appears more than once"
        );
    }
}
