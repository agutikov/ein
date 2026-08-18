//! Every DOT view of a file, as one text — the S1a.5.1 diff.
//!
//! The renderers *do* have a CLI surface, but not all of it and not in every
//! combination: `ein render` reaches four of them, `kb.to_dot`'s keyword
//! surface is library-only, and the slice cones are reachable only through a
//! `--trace` that [S1a.5.2](../../../../plans/m1a_rust/p1a.5_presentation/s1a.5.2_trace_and_answer.md)
//! has not built yet. So the renderers are compared the way the loader and
//! the compiler were: both implementations render the same views over the
//! same corpus and the texts are diffed (`utils/ir_oracle.py`'s `dot-shape`
//! op is the other half).
//!
//! Unlike `plan-shape` or `kb-shape` this invents no rendering of its own.
//! Every view calls a renderer entry point exactly as a CLI subcommand or the
//! trace calls it, so what the diff compares is the artefact a user sees —
//! which is the whole point of the byte gate.

use std::path::Path;

use ein_core::{Kb, Terms};
use ein_ir::{Ast, NodeId};

use crate::ir_dot::{DotOpts, TraceView, to_dot, to_dot_form};
use crate::kb_dot::{ColourBy, KbDotOpts};
use crate::lattice_dag::{LatticeView, render_lattice};
use crate::rules::{RuleMode, render_rules_forms};

/// The views that need only the parsed forms.
pub const PARSE_VIEWS: [&str; 8] = [
    "ir",
    "ir-levi",
    "ir-overlay",
    "ir-trace-dag",
    "ir-forms",
    "rules",
    "rules-overlay",
    "constraints",
];

/// The views that need a loaded KB.
pub const KB_VIEWS: [&str; 6] = [
    "kb",
    "kb-origin",
    "kb-none",
    "kb-no-types",
    "kb-no-instances",
    "kb-since",
];

/// The views that run the engine.
pub const SOLVE_VIEWS: [&str; 3] = ["lattice", "lattice-full", "slice"];

pub fn all_views() -> Vec<&'static str> {
    PARSE_VIEWS
        .iter()
        .chain(KB_VIEWS.iter())
        .chain(SOLVE_VIEWS.iter())
        .copied()
        .collect()
}

fn parse_view(ast: &Ast, forms: &[NodeId], view: &str) -> Result<String, String> {
    let opts = |rule_mode, trace_view, levi| DotOpts {
        rule_mode,
        trace_view,
        levi,
    };
    let default = DotOpts::default();
    Ok(match view {
        "ir" => to_dot(ast, forms, default),
        "ir-levi" => to_dot(
            ast,
            forms,
            opts(RuleMode::SideBySide, TraceView::PerStep, true),
        ),
        "ir-overlay" => to_dot(
            ast,
            forms,
            opts(RuleMode::Overlay, TraceView::PerStep, false),
        ),
        "ir-trace-dag" => to_dot(
            ast,
            forms,
            opts(RuleMode::SideBySide, TraceView::Dag, false),
        ),
        "ir-forms" => {
            // One digraph per top-level form, through the single-node
            // dispatch — the only way `(config …)`'s empty string is
            // reachable.
            let mut out: Vec<String> = Vec::new();
            for (i, f) in forms.iter().enumerate() {
                let head = ast.head_name(*f).unwrap_or("?");
                out.push(format!("--- {i} {head}\n{}", to_dot_form(ast, *f, default)));
            }
            out.join("\n")
        }
        "rules" | "rules-overlay" => {
            let mode = if view.ends_with("overlay") {
                RuleMode::Overlay
            } else {
                RuleMode::SideBySide
            };
            let rules: Vec<NodeId> = forms
                .iter()
                .copied()
                .filter(|f| matches!(ast.head_name(*f), Some("rule" | "hrule")))
                .collect();
            render_rules_forms(ast, &rules, mode)
        }
        "constraints" => crate::constraints::render_constraints(ast, forms, "constraints"),
        _ => return Err(format!("unknown dot view {view:?}")),
    })
}

fn kb_view(ast: &Ast, terms: &mut Terms, kb: &mut Kb, view: &str) -> Result<String, String> {
    let d = KbDotOpts::default;
    Ok(match view {
        "kb" => crate::kb_dot::to_dot(kb, terms, &d()),
        "kb-origin" => crate::kb_dot::to_dot(
            kb,
            terms,
            &KbDotOpts {
                colour_by: ColourBy::Origin,
                ..d()
            },
        ),
        "kb-none" => crate::kb_dot::to_dot(
            kb,
            terms,
            &KbDotOpts {
                colour_by: ColourBy::None,
                name: "plain",
                ..d()
            },
        ),
        "kb-no-types" => crate::kb_dot::to_dot(
            kb,
            terms,
            &KbDotOpts {
                include_types: false,
                ..d()
            },
        ),
        "kb-no-instances" => crate::kb_dot::to_dot(
            kb,
            terms,
            &KbDotOpts {
                include_instances: false,
                ..d()
            },
        ),
        "kb-since" => {
            let root = kb.snapshot();
            saturate(ast, terms, kb)?;
            crate::kb_dot::to_dot(
                kb,
                terms,
                &KbDotOpts {
                    since: Some(&root),
                    name: "state",
                    ..d()
                },
            )
        }
        _ => return Err(format!("unknown dot view {view:?}")),
    })
}

fn saturate(ast: &Ast, terms: &mut Terms, kb: &mut Kb) -> Result<(), String> {
    let mut events = ein_infer::events::Events::off();
    let mut s = ein_infer::saturator::Session {
        kb,
        terms,
        ast,
        events: &mut events,
    };
    let mut sat = ein_infer::saturator::Saturator::new(&mut s).map_err(|e| e.to_string())?;
    sat.saturate(&mut s, None, &mut |_| {})
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn solve_view(ast: &Ast, terms: &mut Terms, kb: &mut Kb, view: &str) -> Result<String, String> {
    use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};

    let opts = SolveOptions {
        stop_after: None,
        max_set_size: 3,
        max_enterings: Some(60),
        on_budget: OnBudget::Verdict,
        store_lattice: true,
        ..SolveOptions::default()
    };
    let mut events = ein_infer::events::Events::off();
    let solved =
        solve(kb, terms, ast, &mut events, &mut NoDumper, &opts).map_err(|e| e.to_string())?;
    let Some(proof) = solved.proof.as_ref() else {
        return Ok("NO PROOF".to_string());
    };
    Ok(match view {
        "lattice" => render_lattice(terms, proof, LatticeView::Solution, "lattice"),
        // Always the fallback-with-a-note path: `solve` never populates a
        // per-commitment SetNode DAG, which is what `--view full`'s help says.
        "lattice-full" => render_lattice(terms, proof, LatticeView::Full, "lattice"),
        // The three shapes `trace.linearize` builds, with its own arguments:
        // the whole-commitment cone, the per-firing step diagram, and the
        // reductio.
        "slice" => {
            let mut out: Vec<String> = Vec::new();
            for (i, s) in proof.solutions.iter().enumerate() {
                out.push(format!("--- solution {i}"));
                out.push(crate::slice::render_slice(
                    terms,
                    &s.commitment,
                    &s.firings,
                    Some(&s.kb),
                    &format!("sol{i}"),
                    None,
                    None,
                ));
                for (n, firing) in s.firings.iter().take(5).enumerate() {
                    out.push(format!("--- step {i}.{}", n + 1));
                    out.push(crate::slice::render_slice(
                        terms,
                        &[],
                        std::slice::from_ref(firing),
                        Some(&s.kb),
                        &format!("step{}", n + 1),
                        None,
                        None,
                    ));
                }
            }
            for (i, d) in proof.dead_commitments.iter().enumerate() {
                out.push(format!("--- dead {i}"));
                out.push(crate::slice::render_slice(
                    terms,
                    &d.commitment,
                    &[],
                    Some(kb),
                    "reductio",
                    Some((&d.unsat_core, &d.learned_clause)),
                    None,
                ));
            }
            out.join("\n")
        }
        _ => return Err(format!("unknown dot view {view:?}")),
    })
}

/// Render one view of a parsed program.
pub fn dot_shape(
    ast: &mut Ast,
    terms: &mut Terms,
    forms: &[NodeId],
    base_dir: Option<&Path>,
    view: &str,
) -> Result<String, String> {
    if PARSE_VIEWS.contains(&view) {
        return parse_view(ast, forms, view);
    }
    let mut kb = ein_ir::load(ast, terms, forms, base_dir).map_err(|e| e.to_string())?;
    if KB_VIEWS.contains(&view) {
        return kb_view(ast, terms, &mut kb, view);
    }
    solve_view(ast, terms, &mut kb, view)
}
