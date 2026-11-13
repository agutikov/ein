//! The rendered trace, against checked-in goldens — S1a.6.11's real solves.
//!
//! `tests/golden/trace_*.md` are real runs. A rendered trace is narration —
//! two runs can narrate different amounts of the same derivation — so the
//! invariance sweeps compare one for presence only and `corpus_shapes.md5`
//! can only say that one moved; these say what. Since M1e2 S1e2.1a.5 they are
//! **region traces**, `ein solve --trace`'s `# Region trace`
//! ([`ein_render::trace::region`]).
//!
//! Until M1e2 S1e2.1c.5 a second kind sat beside them: `trace_3step.md`, a
//! synthetic three-step trace through the walk's markdown renderer, and a
//! test that its `--reorder` emitted every step once. That renderer had no
//! producer since the walk's trace left its CLI path, and it went with the
//! walk on the user's ruling of 2026-11-03; the region trace's `--reorder`
//! is held by `ein-cli/tests/region_modes.rs`'s
//! `the_trace_takes_its_three_shaping_flags`.
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
    ("owing", "tests/region/04_owing.ein", ""),
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
