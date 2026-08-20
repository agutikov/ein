//! The rendered trace, against checked-in fixtures — S1a.5.2's synthetic one
//! and [S1a.6.11](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.11_fixture_goldens.md)'s
//! real solves.
//!
//! **Two kinds of golden, and the difference is the point.**
//!
//! - `tests/golden/trace_3step.md` is **ein.py's**: its
//!   `tests/trace/test_render.py` builds a deterministic synthetic three-step
//!   trace and locks the markdown. That committed file is the fixture, and the
//!   trace is rebuilt against the port's own types — a `TraceStep` in ein.rs
//!   holds an owned [`FactRef`] rather than an interned id, precisely so a
//!   trace can exist without a KB, and this is where that claim is exercised.
//! - `tests/golden/trace_*.md` are **ein.rs's own**, and they exist because
//!   the synthetic one is not enough: it locks the *renderer* and says nothing
//!   about what a solve produces, which is why it kept passing through
//!   [S1a.6.9](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md)
//!   while the rendered trace lost half its rules. Since
//!   [S1a.6.10](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.10_parity_contract.md)
//!   nothing compares a rendered trace against ein.py any more — the two
//!   engines narrate different amounts of the same derivation on purpose — so
//!   this is the whole of its regression coverage.
//!
//! The one that matters most is the **root-saturation section**: *Before any
//! assumption*, then `Assuming …`, then the hypothesis's own steps, numbered
//! as one sequence. That is the half of the trace
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
use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
use ein_ir::{Ast, parse};
use ein_oracle::{golden, golden_path, repo_root};
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
    let want = std::fs::read_to_string(
        repo_root().join("ein.rs/crates/ein-render/tests/golden/from_ein_py/trace_3step.md"),
    )
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

// ── S1a.6.11 — the goldens over *real* solves ──────────────────────

/// One fixture per trace shape, chosen small enough to read in a diff.
///
/// | golden | shape it pins |
/// |---|---|
/// | `unconditional` | solved at root: the whole trace **is** the *Before any assumption* section, and ein.py renders "no surviving derivation" for the same run |
/// | `one-hypothesis` | a single-element commitment: root's 7 steps, `Assuming …`, then the 10 the hypothesis adds |
/// | `ambiguous` | the **same puzzle exhaustively** — `k = 2`, so the trace has to say which of the two solution nodes it narrates. The pair is what makes `-e`'s effect on a trace reviewable |
/// | `two-level` | a two-element commitment reached at layer 2, with two refuted branches — the reductio section |
/// | `unsat` | no solution at all, and twelve reductios |
const REAL_TRACES: [(&str, &str, bool); 5] = [
    ("unconditional", "examples/domain_elim/ab.ein", false),
    (
        "one-hypothesis",
        "examples/branching/12_typed_blind_solve.ein",
        false,
    ),
    (
        "ambiguous",
        "examples/branching/12_typed_blind_solve.ein",
        true,
    ),
    ("two-level", "examples/branching/04_two_levels.ein", false),
    (
        "unsat",
        "examples/saturation/type-exclusivity/pets.ein",
        false,
    ),
];

/// Solve `rel` and render its trace exactly as `ein solve --trace` does.
fn real_trace(rel: &str, exhaustive: bool) -> String {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).expect("the fixture is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("the fixture parses");
    let mut kb =
        ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).expect("the fixture loads");
    // Exactly what `ein solve [-e] --trace` resolves to: the CLI's own
    // defaults, no entering budget, and `store_lattice` — which `--trace`
    // implies, because the trace is a projection of the lattice. A golden
    // rendered under a *different* configuration would pin something no user
    // ever sees. `config: None` is not a gap: `solve` falls back to the KB's
    // own `(config …)` block, which is what the CLI ends up passing.
    let opts = SolveOptions {
        stop_after: if exhaustive { None } else { Some(1) },
        max_set_size: 5,
        on_budget: OnBudget::Verdict,
        store_lattice: true,
        ..SolveOptions::default()
    };
    let mut events = ein_infer::events::Events::off();
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
        .expect("the fixture solves");
    let trace = ein_render::linearize(&ast, &terms, &kb, &solved, ein_render::LinearizeOpts::new());
    strip_dot(&render_markdown(&trace, Mode::Engine, true))
}

/// Replace every fenced `dot` block with one marker line.
///
/// A block that *vanished* still fails — the marker counts — and what is
/// inside one is `golden_dot.rs`'s and `dot_parity.rs`'s to pin.
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
    for (name, rel, exhaustive) in REAL_TRACES {
        let got = real_trace(rel, exhaustive);
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

/// The root section is a *property*, not just bytes: an unconditional solve
/// narrates its derivation, and a hypothesis's steps continue root's
/// numbering rather than restarting.
///
/// Kept separate from the byte golden on purpose — blessing a golden makes a
/// change to it invisible, and this is the part of the change that must never
/// be blessed away.
#[test]
fn the_root_section_is_rendered_and_numbered_as_one_sequence() {
    let uncond = real_trace("examples/domain_elim/ab.ein", false);
    assert!(
        uncond.contains("## Before any assumption — 23 steps"),
        "an unconditional solve stopped narrating root's derivation"
    );
    assert!(
        !uncond.contains("Assuming"),
        "an unconditional solve grew a hypothesis"
    );

    let hyp = real_trace("examples/branching/04_two_levels.ein", false);
    let root_at = hyp
        .find("## Before any assumption")
        .expect("a root section");
    let assuming_at = hyp.find("\nAssuming ").expect("a hypothesis");
    assert!(
        root_at < assuming_at,
        "root's steps come after the assumption"
    );
    // 16 unconditional, then 4 more — one sequence, not two.
    let steps: Vec<usize> = hyp
        .lines()
        .filter_map(|l| l.strip_prefix("## Step "))
        .filter_map(|l| l.split_whitespace().next())
        .filter_map(|n| n.parse().ok())
        .collect();
    assert_eq!(
        steps,
        (1..=20).collect::<Vec<usize>>(),
        "the step numbers restart across the fork boundary"
    );
    let last_root = hyp[..assuming_at]
        .lines()
        .filter(|l| l.starts_with("## Step "))
        .count();
    assert_eq!(
        last_root, 16,
        "the root section is not the unconditional 16"
    );
}
