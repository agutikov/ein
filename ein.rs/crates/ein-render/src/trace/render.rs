//! The markdown pieces a trace's steps are rendered with: a step's premises
//! line and its inline `dot` block — the region trace's
//! ([`super::region`]).
//!
//! The walk's trace renderer — one narrative per solve, its reductios and its
//! closing lattice DAG — went with the walk at M1e2 S1e2.1c.5 (2026-11-03).

use super::ast::TraceStep;

pub(crate) fn dot_block(dot: Option<&String>) -> Vec<String> {
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

pub(crate) fn premises_line(step: &TraceStep) -> String {
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
