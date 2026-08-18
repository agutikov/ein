//! The markdown trace renderer — `ein.py`'s `trace/render.py`.
//!
//! Threads a linearised [`Trace`] into one self-contained markdown narrative:
//! a numbered step per rule firing (name, English `:why`, premises with their
//! quoted source sentences, and an inline `dot` derivation slice), refuted
//! hypotheses folded into `<details>` reductio sections, and a closing
//! lattice DAG plus solution grid. Every diagram is an inline fenced `dot`
//! block — no SVG; the engine emits DOT and rasterising is a shell concern.
//!
//! `Mode::Reorder` clusters the steps by the entity they are about — a
//! presentation pass over the same steps, emitting each exactly once.

use super::ast::TraceStep;
use super::linearize::{Reductio, Trace};

/// Numbered engine order (the default), or clustered by target entity.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Engine,
    Reorder,
}

impl Mode {
    pub fn parse(s: &str) -> Option<Mode> {
        match s {
            "engine" => Some(Mode::Engine),
            "reorder" => Some(Mode::Reorder),
            _ => None,
        }
    }
}

fn dot_block(dot: Option<&String>) -> Vec<String> {
    match dot.filter(|d| !d.is_empty()) {
        None => Vec::new(),
        Some(d) => vec![
            "```dot".to_string(),
            d.clone(),
            "```".to_string(),
            String::new(),
        ],
    }
}

fn premises_line(step: &TraceStep) -> String {
    let labels = step.premise_labels();
    if !labels.is_empty() {
        let quoted: Vec<String> = labels.iter().map(|p| format!("`{p}`")).collect();
        let mut base = format!("Premises: {}", quoted.join(", "));
        if !step.sources.is_empty() {
            base.push_str(&format!(" — from {}", step.sources.join(", ")));
        }
        return base;
    }
    if !step.sources.is_empty() {
        // No derived premise — the source *is* what was given.
        return format!("Premises: from {}", step.sources.join(", "));
    }
    "Premises: —".to_string()
}

fn render_step(step: &TraceStep, diagrams: bool) -> Vec<String> {
    let mut out = vec![
        format!("## Step {} — `{}`", step.n, step.rule),
        String::new(),
    ];
    if !step.why.is_empty() {
        out.push(format!("> {}", step.why));
        out.push(String::new());
    }
    out.push(premises_line(step));
    out.push(String::new());
    out.push(format!("Derives `{}`.", step.derived_label()));
    out.push(String::new());
    if diagrams {
        out.extend(dot_block(step.diagram.as_ref()));
    }
    out
}

fn render_reductio(r: &Reductio, diagrams: bool) -> Vec<String> {
    let mut out = vec![
        "<details>".to_string(),
        format!("<summary>{}</summary>", r.summary),
        String::new(),
        format!("Assumed **{}**; the branch derives ⊥.", r.commitment),
        String::new(),
    ];
    if !r.learned_clause.is_empty() {
        out.push(format!("Lifted no-good: `{}`.", r.learned_clause));
        out.push(String::new());
    }
    if diagrams {
        out.extend(dot_block(r.diagram.as_ref()));
    }
    out.push("</details>".to_string());
    out.push(String::new());
    out
}

/// Render a [`Trace`] as markdown.
pub fn render_markdown(trace: &Trace, mode: Mode, diagrams: bool) -> String {
    let mut lines = vec![
        "# Solution trace".to_string(),
        String::new(),
        format!("> {}", trace.summary),
        String::new(),
    ];
    if !trace.commitment.is_empty()
        && trace.commitment != "∅ (unconditional)"
        && trace.commitment != "—"
    {
        lines.push(format!("Assuming **{}**.", trace.commitment));
        lines.push(String::new());
    }

    if trace.steps.is_empty() {
        lines.push("_(no surviving derivation — see the refuted branches below.)_".to_string());
        lines.push(String::new());
    } else if mode == Mode::Reorder {
        lines.extend(render_reordered(trace, diagrams));
    } else {
        let mut emitted_hyp = false;
        for step in &trace.steps {
            // The unconditional spine first, then the under-hypothesis block.
            if step.conditional && !emitted_hyp {
                lines.push(format!("## Under hypothesis — {}", trace.commitment));
                lines.push(String::new());
                emitted_hyp = true;
            }
            lines.extend(render_step(step, diagrams));
        }
    }

    if !trace.reductios.is_empty() {
        lines.push("## Refuted hypotheses".to_string());
        lines.push(String::new());
        for r in &trace.reductios {
            lines.extend(render_reductio(r, diagrams));
        }
    }

    if diagrams && trace.lattice_dot.is_some() {
        lines.push("## Commitment lattice".to_string());
        lines.push(String::new());
        lines.extend(dot_block(trace.lattice_dot.as_ref()));
    }
    if diagrams && trace.solution_dot.is_some() {
        lines.push("## Solution".to_string());
        lines.push(String::new());
        lines.extend(dot_block(trace.solution_dot.as_ref()));
    }
    if trace.full_kb_dot.is_some() {
        lines.push("## Full KB (final state)".to_string());
        lines.push(String::new());
        lines.extend(dot_block(trace.full_kb_dot.as_ref()));
    }

    format!("{}\n", lines.join("\n").trim_end())
}

/// Stable-partition the steps by their target entity.
///
/// A presentation pass: every step is emitted exactly once, keeping its engine
/// step number and its within-cluster order — so the reordered trace has the
/// *same set of steps* as engine order, grouped under `## About <entity>`.
/// Clusters appear in first-seen order; a step with no entity falls under
/// "About (misc)".
fn render_reordered(trace: &Trace, diagrams: bool) -> Vec<String> {
    let mut order: Vec<String> = Vec::new();
    let mut by_entity: Vec<(String, Vec<&TraceStep>)> = Vec::new();
    for step in &trace.steps {
        let key = step.section.clone().unwrap_or_else(|| "(misc)".to_string());
        match by_entity.iter_mut().find(|(k, _)| *k == key) {
            Some(slot) => slot.1.push(step),
            None => {
                order.push(key.clone());
                by_entity.push((key, vec![step]));
            }
        }
    }
    let mut out: Vec<String> = Vec::new();
    for entity in &order {
        out.push(format!("## About {entity}"));
        out.push(String::new());
        let steps = by_entity
            .iter()
            .find(|(k, _)| k == entity)
            .map(|(_, v)| v)
            .expect("registered above");
        for step in steps {
            let why = if step.why.is_empty() {
                String::new()
            } else {
                format!(" — {}", step.why)
            };
            out.push(format!(
                "**Step {}** · `{}` → `{}`{why}",
                step.n,
                step.rule,
                step.derived_label()
            ));
            out.push(String::new());
            out.push(premises_line(step));
            out.push(String::new());
            if diagrams {
                out.extend(dot_block(step.diagram.as_ref()));
            }
        }
    }
    out
}
