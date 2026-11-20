//! Every DOT view of a file, as one text.
//!
//! The renderers *do* have a CLI surface, but not all of it and not in every
//! combination: `ein render` reaches four of them, `kb_dot::to_dot`'s option
//! surface is library-only, and the slice cones are reachable only through a
//! `--trace` ([01-architecture.md § 3.4](../../../../docs/ein/ein.rs/01-architecture.md#34-ein-einb-ein-render-ein-cli)).
//! So the renderers are pinned the way the loader and the compiler are: every
//! view is rendered over the whole corpus and the texts are digested
//! (`ein-render/tests/golden/corpus_shapes.md5`, which
//! `ein-render/tests/id_order_invariance.rs` holds every rendering to).
//!
//! Unlike `plan-shape` or `kb-shape` this invents no rendering of its own.
//! Every view calls a renderer entry point exactly as a CLI subcommand or the
//! trace calls it, so what the digest compares is the artefact a user sees.

use std::path::Path;

use ein_core::{Kb, Terms};
use ein_ir::{Ast, NodeId};

use crate::ir_dot::{DotOpts, TraceView, to_dot, to_dot_form};
use crate::kb_dot::{ColourBy, KbDotOpts};
use crate::rules::{RuleMode, render_rules_forms};

/// The views that need only the parsed forms.
pub const PARSE_VIEWS: [&str; 8] = [
    "ir",
    "ir-fold",
    "ir-overlay",
    "ir-trace-dag",
    "ir-forms",
    "rules",
    "rules-fold",
    "rules-overlay",
];

/// The views that need a loaded KB. The constraint view draws root at its
/// fixpoint, as `ein render constraints` does.
pub const KB_VIEWS: [&str; 7] = [
    "kb",
    "kb-fold",
    "kb-origin",
    "kb-none",
    "kb-since",
    "constraints",
    "constraints-fold",
];

pub fn all_views() -> Vec<&'static str> {
    PARSE_VIEWS.iter().chain(KB_VIEWS.iter()).copied().collect()
}

fn parse_view(ast: &Ast, forms: &[NodeId], view: &str) -> Result<String, String> {
    let opts = |rule_mode, trace_view, fold| DotOpts {
        rule_mode,
        trace_view,
        fold,
    };
    let default = DotOpts::default();
    let rules_of = || -> Vec<NodeId> {
        forms
            .iter()
            .copied()
            .filter(|f| ast.head_name(*f) == Some("rule"))
            .collect()
    };
    Ok(match view {
        "ir" => to_dot(ast, forms, default),
        "ir-fold" => to_dot(
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
        "rules" => render_rules_forms(ast, &rules_of(), RuleMode::SideBySide, false),
        "rules-fold" => render_rules_forms(ast, &rules_of(), RuleMode::SideBySide, true),
        "rules-overlay" => render_rules_forms(ast, &rules_of(), RuleMode::Overlay, false),
        _ => return Err(format!("unknown dot view {view:?}")),
    })
}

fn kb_view(ast: &Ast, terms: &mut Terms, kb: &mut Kb, view: &str) -> Result<String, String> {
    let d = KbDotOpts::default;
    Ok(match view {
        "kb" => crate::kb_dot::to_dot(kb, terms, &d()),
        "kb-fold" => crate::kb_dot::to_dot(kb, terms, &KbDotOpts { fold: true, ..d() }),
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
        "constraints" | "constraints-fold" => {
            saturate(ast, terms, kb)?;
            crate::constraints::render_constraints(
                kb,
                terms,
                "constraints",
                view == "constraints-fold",
            )
        }
        _ => return Err(format!("unknown dot view {view:?}")),
    })
}

/// Root's state, as `ein render kb` and `ein render constraints` draw it:
/// its fixpoint, and the `open` facts the pass writes there (Q-M1f.572).
fn saturate(ast: &Ast, terms: &mut Terms, kb: &mut Kb) -> Result<(), String> {
    ein_infer::open::root_state(ast, terms, kb).map_err(|e| e.to_string())
}

/// Render one view of a parsed program. No view runs a search, so none
/// takes a job count.
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
    kb_dot_shape(ast, terms, &mut kb, view)
}

/// One of [`KB_VIEWS`] over a KB already loaded from the file, by the same
/// renderer call as [`dot_shape`]'s, so the same text: for a caller that
/// loads a program once and draws each view from a copy of the load, as the
/// corpus sweeps do.
pub fn kb_dot_shape(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    view: &str,
) -> Result<String, String> {
    if KB_VIEWS.contains(&view) {
        return kb_view(ast, terms, kb, view);
    }
    Err(format!("unknown dot view {view:?}"))
}
