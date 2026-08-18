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

/// The modes of the trace / answer diff — [S1a.5.2](../../../../plans/m1a_rust/p1a.5_presentation/s1a.5.2_trace_and_answer.md).
pub const TRACE_MODES: [&str; 3] = ["trace", "answer", "no-proof"];

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

// ── The trace and answer surface ───────────────────────────────────

/// The bounded solve a trace mode runs.
///
/// `first` is the *fast* regime, and it is what the `trace` mode wants: a
/// puzzle that stops at its first solution reaches one in a dozen enterings
/// and hands the renderer a spine of several hundred firings, where the
/// exhaustive regime spends its whole budget and aborts with nothing to
/// narrate. The exhaustive regime is what the `answer` mode wants, for the
/// opposite reason: `Ambiguity`, `Contradiction` and `Aborted` are only
/// reachable there, and they are three of the table's four shapes.
fn solve_for_trace(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    store_lattice: bool,
    first: bool,
) -> Result<ein_infer::solve::Solved, String> {
    use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
    let opts = SolveOptions {
        stop_after: if first { Some(1) } else { None },
        max_set_size: 3,
        max_enterings: Some(if first { 300 } else { 60 }),
        on_budget: OnBudget::Verdict,
        store_lattice,
        ..SolveOptions::default()
    };
    let mut events = ein_infer::events::Events::off();
    solve(kb, terms, ast, &mut events, &mut NoDumper, &opts).map_err(|e| e.to_string())
}

/// The four `solve --trace` flags, as one value.
#[derive(Clone, Copy, Default)]
struct Flags {
    diagrams: bool,
    full_kb: bool,
    relevant: bool,
    reorder: bool,
}

fn trace_markdown(
    ast: &Ast,
    terms: &Terms,
    root: &Kb,
    solved: &ein_infer::solve::Solved,
    flags: Flags,
) -> String {
    let trace = crate::trace::linearize(
        ast,
        terms,
        root,
        solved,
        crate::trace::LinearizeOpts {
            diagrams: flags.diagrams,
            full_kb_snapshots: flags.full_kb,
            relevant: flags.relevant,
        },
    );
    crate::trace::render_markdown(
        &trace,
        if flags.reorder {
            crate::trace::Mode::Reorder
        } else {
            crate::trace::Mode::Engine
        },
        flags.diagrams,
    )
}

/// Render one mode of the trace / answer surface.
pub fn trace_shape(
    ast: &mut Ast,
    terms: &mut Terms,
    forms: &[NodeId],
    base_dir: Option<&Path>,
    mode: &str,
) -> Result<String, String> {
    let mut kb = ein_ir::load(ast, terms, forms, base_dir).map_err(|e| e.to_string())?;

    if mode == "no-proof" {
        let solved = solve_for_trace(ast, terms, &mut kb, false, false)?;
        return Ok(trace_markdown(
            ast,
            terms,
            &kb,
            &solved,
            Flags {
                diagrams: true,
                ..Flags::default()
            },
        ));
    }

    if mode == "answer" {
        let solved = solve_for_trace(ast, terms, &mut kb, true, false)?;
        let mut out: Vec<String> = Vec::new();
        for exhausted in [true, false] {
            out.push(format!("--- answer exhausted={}", py_bool(exhausted)));
            out.push(crate::answer::render_answer(
                ast,
                terms,
                &kb,
                &solved.answer,
                exhausted,
            ));
            out.push(format!("--- table exhausted={}", py_bool(exhausted)));
            out.push(crate::answer::render_solution_table(
                ast,
                terms,
                &kb,
                &solved.answer,
                Some(solved.stats.solution_nodes),
                exhausted,
                Some("<source>"),
            ));
        }
        // The exhaustive regime's own trace: this is where the unsat and
        // many-solution lattice shapes are, and `trace` never sees one.
        out.push("--- markdown exhaustive".to_string());
        out.push(trace_markdown(
            ast,
            terms,
            &kb,
            &solved,
            Flags {
                diagrams: true,
                ..Flags::default()
            },
        ));
        return Ok(out.join("\n"));
    }

    let solved = solve_for_trace(ast, terms, &mut kb, true, true)?;
    let f = |diagrams, full_kb, relevant, reorder| Flags {
        diagrams,
        full_kb,
        relevant,
        reorder,
    };
    let flags = [
        ("default", f(true, false, false, false)),
        ("no-diagrams", f(false, false, false, false)),
        ("full-kb", f(true, true, false, false)),
        ("reorder", f(false, false, false, true)),
        ("relevant", f(false, false, true, false)),
        ("relevant-reorder", f(false, false, true, true)),
    ];
    let mut out: Vec<String> = Vec::new();
    for (name, flags) in flags {
        out.push(format!("--- markdown {name}"));
        out.push(trace_markdown(ast, terms, &kb, &solved, flags));
    }
    // The round-trip is a *property*, and its witness is a text both
    // implementations can print: dump the steps as IR, parse them back, dump
    // again, and show both. Equal halves mean the round-trip held.
    let steps = crate::trace::linearize(
        ast,
        terms,
        &kb,
        &solved,
        crate::trace::LinearizeOpts {
            diagrams: false,
            full_kb_snapshots: false,
            relevant: false,
        },
    )
    .steps;
    let ir = crate::trace::trace_to_ir(&steps);
    let mut reparse = Ast::new();
    // A parse failure is ein.py's `IRParseError`, propagating out of the op —
    // so it propagates here too rather than degrading to an empty trace.
    let forms = ein_ir::parse(&mut reparse, &ir, None).map_err(|e| e.to_string())?;
    let again = match forms.first() {
        Some(&f) => crate::trace::trace_to_ir(&crate::trace::parse_trace_steps(&reparse, f)),
        None => "(trace)".to_string(),
    };
    out.push("--- ir".to_string());
    out.push(ir.clone());
    out.push("--- ir-reparsed".to_string());
    out.push(again.clone());
    out.push(format!(
        "--- round-trip {}",
        if ir == again { "ok" } else { "DIFFERS" }
    ));
    Ok(out.join("\n"))
}

/// `str(bool)` — `True` / `False`, because the separator lines interpolate one.
fn py_bool(b: bool) -> &'static str {
    if b { "True" } else { "False" }
}
