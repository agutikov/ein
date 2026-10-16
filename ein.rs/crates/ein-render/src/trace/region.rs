//! The region trace: `--trace FILE.md` under a region mode — M1e2
//! [T1e2.1a.5.2](../../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md)'s
//! seventh step, [`read_out.md` § 8.4](../../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md#84---trace-filemd).
//!
//! **What R is made of, told in the walk's steps.** The headline is the
//! read-out's own rows, so the trace cannot say *No solution* over a run
//! with states (P1e2.3's S2). Then root's derivation; then each state the
//! read-out printed, derived from its first minimal commitment; then `N`,
//! each minimal dead set with why it is dead, where the walk had its refuted
//! hypotheses; and, for an empty R that owes, the frontier. A description is
//! traced as far as stdout lists it, and the trace says where it stopped.

use super::ast::TraceStep;
use super::render::{dot_block, premises_line};

/// One state's section.
pub struct StateTrace {
    /// `State 3 of 5 — 1 candidate · from {(co-located Bob Coffee)}`.
    pub title: String,
    /// The commitment it is derived from, rendered.
    pub commitment: String,
    pub steps: Vec<TraceStep>,
}

/// A minimal dead set, and why.
pub struct DeadTrace {
    pub set: String,
    pub core: Vec<String>,
    pub refuted_by: Vec<String>,
}

/// A region run, ready for markdown.
pub struct RegionTrace {
    /// The read-out's summary rows, as printed.
    pub head: Vec<String>,
    pub root: Vec<TraceStep>,
    pub states: Vec<StateTrace>,
    /// States the read-out did not print, and so not traced.
    pub untraced: usize,
    pub dead: Vec<DeadTrace>,
    /// For an empty R that owes: each frontier node and what it owes.
    pub owing: Vec<(String, Vec<String>)>,
}

fn step(step: &TraceStep, diagrams: bool) -> Vec<String> {
    let mut out = vec![
        format!("### Step {} — `{}`", step.n, step.rule),
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

/// Render a [`RegionTrace`] as markdown.
pub fn region_markdown(t: &RegionTrace, diagrams: bool) -> String {
    let mut lines = vec!["# Region trace".to_string(), String::new()];
    lines.push("```text".to_string());
    lines.extend(t.head.iter().cloned());
    lines.push("```".to_string());
    lines.push(String::new());
    lines.push(format!("## Root — {} steps", t.root.len()));
    lines.push(String::new());
    if t.root.is_empty() {
        lines.push("_Root derives nothing beyond what the program gives._".to_string());
        lines.push(String::new());
    }
    for s in &t.root {
        lines.extend(step(s, diagrams));
    }
    for st in &t.states {
        lines.push(format!("## {} — {} steps", st.title, st.steps.len()));
        lines.push(String::new());
        if st.commitment != "∅" {
            lines.push(format!("Assuming **{}**.", st.commitment));
            lines.push(String::new());
        }
        if st.steps.is_empty() {
            lines.push("_Nothing beyond root and the commitment._".to_string());
            lines.push(String::new());
        }
        for s in &st.steps {
            lines.extend(step(s, diagrams));
        }
    }
    if t.untraced > 0 {
        lines.push(format!(
            "_{} more {} not traced: the read-out printed {} — `--dump-answer DIR` writes every \
             state, and `models/` holds each as a loadable program._",
            t.untraced,
            if t.untraced == 1 {
                "state is"
            } else {
                "states are"
            },
            if t.states.len() == 1 {
                "one".to_string()
            } else {
                format!("the first {}", t.states.len())
            }
        ));
        lines.push(String::new());
    }
    if !t.owing.is_empty() {
        lines.push(format!(
            "## Owing — {} {} with no alive node one fact up",
            t.owing.len(),
            if t.owing.len() == 1 { "node" } else { "nodes" }
        ));
        lines.push(String::new());
        for (set, owed) in &t.owing {
            lines.push(format!("- **{set}** owes"));
            for w in owed {
                lines.push(format!("  - {w}"));
            }
        }
        lines.push(String::new());
    }
    if !t.dead.is_empty() {
        lines.push(format!("## N — {} minimal dead sets", t.dead.len()));
        lines.push(String::new());
        for d in &t.dead {
            lines.push("<details>".to_string());
            lines.push(format!("<summary>{} — dead</summary>", d.set));
            lines.push(String::new());
            if d.core.is_empty() {
                lines.push("Core: not kept by this route.".to_string());
            } else {
                let core: Vec<String> = d.core.iter().map(|c| format!("`{c}`")).collect();
                lines.push(format!("Core: {}", core.join(", ")));
            }
            if !d.refuted_by.is_empty() {
                lines.push(String::new());
                lines.push(format!("Refuted by `{}`.", d.refuted_by.join("`, `")));
            }
            lines.push(String::new());
            lines.push("</details>".to_string());
            lines.push(String::new());
        }
    }
    format!("{}\n", lines.join("\n").trim_end())
}
