//! The rendered trace, against checked-in goldens — S1a.5.2's synthetic one
//! and S1a.6.11's real solves.
//!
//! **Two kinds of golden, and the difference is the point.**
//!
//! - `tests/golden/trace_3step.md` is a deterministic synthetic three-step
//!   trace, built from the trace types alone — a `TraceStep` holds an owned
//!   [`FactRef`] rather than an interned id, precisely so a trace can exist
//!   without a KB, and this is where that claim is exercised.
//! - `tests/golden/trace_*.md` are real runs, and they exist because the
//!   synthetic one is not enough: it locks the *renderer* and says nothing
//!   about what a run produces, which is why it kept passing through
//!   [S1a.6.9](../../../../docs/history/m1a_rust/README.md#s1a69--the-fork-entry-delta-the-resumed-saturator)
//!   while the rendered trace lost half its rules. A rendered trace is
//!   narration — two runs can narrate different amounts of the same
//!   derivation — so the invariance sweeps compare one for presence only and
//!   `corpus_shapes.md5` can only say that one moved; these say what. Since
//!   M1e2 S1e2.1a.5 they are **region traces**, `ein solve --trace`'s
//!   `# Region trace` ([`ein_render::trace::region`]): the walk's trace left
//!   with its CLI path, and its renderer keeps the synthetic golden.
//!
//! The one that matters most is the **root section**: root's derivation, then
//! each state's from its commitment, `Assuming …`, with its own steps. That is
//! the half of the trace
//! [idea-08](../../../../plans/ideas/08-human-style-deductive-trace.md) is
//! about, and it is what the accidental fork re-derivation used to stand in
//! for.
//!
//! ```text
//! EIN_BLESS=1 cargo test -p ein-render          # regenerate every golden
//! ```
//!
//! The DOT blocks are replaced by a marker: they are rendered (so the diagram
//! path runs) and they have their own goldens in `golden_dot.rs`, and a trace
//! whose every step carried a digraph would be a golden nobody reads.

use ein_core::Terms;
use ein_corpus::{golden, golden_path, repo_root};
use ein_infer::explore::{self, Ask, Mode as Asked, Which};
use ein_ir::{Ast, parse};
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
    let got = render_markdown(&synthetic_trace(), Mode::Engine, false);
    if let Some(e) = golden(&golden_path("ein-render", "trace_3step.md"), &got) {
        panic!("{e}");
    }
}

/// The reorder pass is a *presentation* pass: same steps, grouped. It is the
/// property that makes `--reorder` safe to hand a reader.
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

// ── S1a.6.11 — the goldens over *real* solves ──────────────────────

/// One fixture per trace shape, chosen small enough to read in a diff, each
/// under the flag that asks its question.
///
/// | golden | shape it pins |
/// |---|---|
/// | `unconditional` | R is root alone: the whole trace **is** the root section |
/// | `one-hypothesis` | `--maximal`: one state, a one-candidate commitment over root |
/// | `ambiguous` | the **same program described**: every state of R traced, ∅ among them |
/// | `two-level` | `--maximal`: two states, each from a two-candidate commitment, and N |
/// | `dead` | R empty of the dead kind: root's derivation of `(false)` |
/// | `owing` | R empty of the owing kind: the frontier and what it owes |
const REAL_TRACES: [(&str, &str, &str); 6] = [
    ("unconditional", "examples/domain_elim/ab.ein", ""),
    (
        "one-hypothesis",
        "examples/branching/12_typed_blind_solve.ein",
        "--maximal",
    ),
    (
        "ambiguous",
        "examples/branching/12_typed_blind_solve.ein",
        "",
    ),
    (
        "two-level",
        "examples/branching/04_two_levels.ein",
        "--maximal",
    ),
    (
        "dead",
        "tests/stdlib/algebra/09_irreflexive_violated.ein",
        "",
    ),
    ("owing", "tests/stdlib/closure/03_closed_and_owing.ein", ""),
];

/// Run `rel` under `flag` and render its trace exactly as `ein solve [flag]
/// --trace` does: root saturated first with its firings kept, the run, then
/// [`ein_render::trace::region::of_read_out`] with diagrams on.
fn real_trace(rel: &str, flag: &str) -> String {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).expect("the fixture is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("the fixture parses");
    let mut kb =
        ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).expect("the fixture loads");
    let mode = match flag {
        "--maximal" => Asked {
            which: Which::Maximal,
            ask: Ask::Describe,
        },
        _ => Asked {
            which: Which::All,
            ask: Ask::Describe,
        },
    };
    let opts = explore::Options::new(mode);
    let mut events = ein_infer::events::Events::off();
    let firings = ein_infer::region::root_firings(&mut kb, &mut terms, &ast, &mut events)
        .expect("root saturates");
    let r = ein_infer::read_out::run_with(&mut kb, &mut terms, &ast, &opts, &mut events)
        .expect("the fixture runs");
    let argv = format!("{rel} {flag}").trim_end().to_string();
    let md = ein_render::trace::region::of_read_out(
        &r,
        &mut kb,
        &mut terms,
        &ast,
        &firings,
        &ein_render::read_out::Options {
            command: "solve",
            argv: &argv,
            file: rel,
            budget: opts.budget,
            uncut: false,
        },
        ein_render::trace::region::Shape {
            diagrams: true,
            relevant: false,
            reorder: false,
            full_kb: false,
        },
    )
    .expect("the trace renders");
    strip_dot(&md)
}

/// Replace every fenced `dot` block with one marker line.
///
/// A block that *vanished* still fails — the marker counts — and what is
/// inside one is `golden_dot.rs`'s and `corpus_shapes.rs`'s to pin.
fn strip_dot(md: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    let mut in_dot = false;
    for line in md.lines() {
        if line.trim_end() == "```dot" {
            in_dot = true;
            out.push("<dot>");
            continue;
        }
        if in_dot {
            if line.trim_end() == "```" {
                in_dot = false;
            }
            continue;
        }
        out.push(line);
    }
    out.join("\n") + "\n"
}

#[test]
fn every_real_solve_reproduces_its_golden() {
    let mut bad: Vec<String> = Vec::new();
    let mut lines = 0usize;
    for (name, rel, flag) in REAL_TRACES {
        let got = real_trace(rel, flag);
        lines += got.lines().count();
        if let Some(e) = golden(
            &golden_path("ein-render", &format!("trace_{name}.md")),
            &got,
        ) {
            bad.push(e);
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n\n"));
    eprintln!(
        "goldens (trace): {} shapes, {lines} lines",
        REAL_TRACES.len()
    );
}

/// The root section is a *property*, not just bytes: a program solved at root
/// narrates its derivation there, and a state's section is its commitment's
/// derivation over root, numbered from 1 under its own heading.
///
/// Kept separate from the byte golden on purpose — blessing a golden makes a
/// change to it invisible, and this is the part of the change that must never
/// be blessed away.
#[test]
fn the_root_section_comes_first_and_each_state_is_derived_from_its_commitment() {
    let uncond = real_trace("examples/domain_elim/ab.ein", "");
    assert!(
        uncond.contains("## Root — 23 steps"),
        "a program solved at root stopped narrating root's derivation:\n{uncond}"
    );
    assert!(
        !uncond.contains("Assuming"),
        "a program solved at root grew a commitment"
    );
    let hyp = real_trace("examples/branching/04_two_levels.ein", "--maximal");
    let root_at = hyp.find("## Root — ").expect("a root section");
    let assuming_at = hyp.find("\nAssuming ").expect("a commitment");
    assert!(
        root_at < assuming_at,
        "root's steps come after the assumption"
    );
    let root_steps = hyp[..assuming_at]
        .lines()
        .filter(|l| l.starts_with("### Step "))
        .count();
    assert_eq!(
        root_steps, 16,
        "the root section is not the unconditional 16"
    );
    // Each state's steps start again at 1, under its own heading.
    let state = &hyp[assuming_at..];
    assert!(
        state.contains("### Step 1 — "),
        "a state's derivation is not numbered from 1:\n{state}"
    );
}
