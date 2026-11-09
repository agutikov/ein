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
//!
//! The walk's three trace flags read here as they did there: `--relevant`
//! keeps the goal-relevant slice of each derivation, `--reorder` groups each
//! section's steps by the entity they are about, and `--full-kb-snapshots`
//! appends each state's whole KB.

use ein_core::{Kb, Terms};
use ein_infer::read_out::{Listing, ReadOut};
use ein_ir::Ast;

use super::ast::TraceStep;
use super::render::{dot_block, premises_line};

/// One state's section.
pub struct StateTrace {
    /// `State 3 of 5 — 1 candidate · from {(co-located Bob Coffee)}`.
    pub title: String,
    /// The commitment it is derived from, rendered.
    pub commitment: String,
    pub steps: Vec<TraceStep>,
    /// `--full-kb-snapshots`: the state's whole KB, as DOT.
    pub full_kb: Option<String>,
}

/// A minimal dead set, and why.
pub struct DeadTrace {
    pub set: String,
    pub core: Vec<String>,
    /// What the core's search established (M1e2 S1e2.2.23).
    pub core_status: Option<ein_infer::explain::CoreStatus>,
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
    /// `--reorder`: each section's steps grouped by their entity.
    pub reorder: bool,
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

/// One section's steps, in engine order or grouped by entity (`--reorder`):
/// every step once, keeping its number and its order within the group.
fn steps(steps: &[TraceStep], diagrams: bool, reorder: bool) -> Vec<String> {
    if !reorder {
        return steps.iter().flat_map(|s| step(s, diagrams)).collect();
    }
    let mut groups: Vec<(String, Vec<&TraceStep>)> = Vec::new();
    for s in steps {
        let key = s.section.clone().unwrap_or_else(|| "(misc)".to_string());
        match groups.iter_mut().find(|(k, _)| *k == key) {
            Some(g) => g.1.push(s),
            None => groups.push((key, vec![s])),
        }
    }
    let mut out = Vec::new();
    for (entity, group) in groups {
        out.push(format!("### About {entity}"));
        out.push(String::new());
        for s in group {
            let why = if s.why.is_empty() {
                String::new()
            } else {
                format!(" — {}", s.why)
            };
            out.push(format!(
                "**Step {}** · `{}` → `{}`{why}",
                s.n,
                s.rule,
                s.derived_label()
            ));
            out.push(String::new());
            out.push(premises_line(s));
            out.push(String::new());
            if diagrams {
                out.extend(dot_block(s.diagram.as_ref()));
            }
        }
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
    lines.extend(steps(&t.root, diagrams, t.reorder));
    for st in &t.states {
        lines.push(format!(
            "## {} — {} step{}",
            st.title,
            st.steps.len(),
            if st.steps.len() == 1 { "" } else { "s" }
        ));
        lines.push(String::new());
        if st.commitment != "∅" {
            lines.push(format!("Assuming **{}**.", st.commitment));
            lines.push(String::new());
        }
        if st.steps.is_empty() {
            lines.push("_Nothing beyond root and the commitment._".to_string());
            lines.push(String::new());
        }
        lines.extend(steps(&st.steps, diagrams, t.reorder));
        if let Some(dot) = &st.full_kb {
            lines.push("**The state's whole KB.**".to_string());
            lines.push(String::new());
            lines.extend(dot_block(Some(dot)));
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
            let core: Vec<String> = d.core.iter().map(|c| format!("`{c}`")).collect();
            lines.push(format!("Core: {}", core.join(", ")));
            if let Some(q) = d.core_status.and_then(|c| c.qualifier()) {
                lines.push(String::new());
                lines.push(format!("Core search: {q}."));
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

/// The trace flags: `--no-diagrams`, `--relevant`, `--reorder`,
/// `--full-kb-snapshots`.
#[derive(Clone, Copy, Debug)]
pub struct Shape {
    pub diagrams: bool,
    pub relevant: bool,
    pub reorder: bool,
    pub full_kb: bool,
}

/// `ein solve --trace FILE.md`: root's derivation, then each state stdout
/// printed, derived from its first minimal commitment, then N and the owing
/// frontier (`read_out.md` § 8.4). `root_firings` are root's own saturation's
/// ([`ein_infer::region::root_firings`]), and `options` the read-out's, whose
/// summary rows head the trace and whose listing says which states it traces.
pub fn of_read_out(
    r: &ReadOut,
    kb: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    root_firings: &[ein_infer::Firing],
    options: &crate::read_out::Options<'_>,
    shape: Shape,
) -> Result<String, String> {
    let Shape {
        diagrams,
        relevant,
        reorder,
        full_kb,
    } = shape;
    let a = &r.answer;
    let cands: Vec<String> = a.candidates.iter().map(|c| c.text.clone()).collect();
    let name = |set: &[u32]| -> String {
        if set.is_empty() {
            "∅".to_string()
        } else {
            let v: Vec<&str> = set.iter().map(|&b| cands[b as usize].as_str()).collect();
            v.join(", ")
        }
    };
    let set_name = |set: &[u32]| -> String {
        if set.is_empty() {
            "∅".to_string()
        } else {
            format!("{{{}}}", name(set))
        }
    };
    let text = crate::read_out::render(r, options);
    let head: Vec<String> = text
        .stdout
        .lines()
        .skip(2)
        .take_while(|l| !l.is_empty())
        .map(str::to_string)
        .collect();
    let root = crate::trace::steps_of_with(ast, terms, root_firings, kb, diagrams, &[], relevant);
    let mut states = Vec::new();
    let k = r.states.len();
    {
        let mut nodes = ein_infer::region::Nodes::new(kb, terms, ast, &a.candidates)
            .map_err(|e| e.to_string())?;
        for &i in &text.shown {
            let s = &r.states[i];
            let from = s.generators.first().unwrap_or(&s.rep).clone();
            let facts: Vec<ein_core::FactId> = from
                .iter()
                .map(|&b| a.candidates[b as usize].fact)
                .collect();
            let (steps, whole) = nodes
                .with_fixpoint_traced(&facts, |kb, terms, firings| {
                    (
                        crate::trace::steps_of_with(
                            ast, terms, firings, kb, diagrams, &facts, relevant,
                        ),
                        full_kb.then(|| crate::slice::render_state(kb, terms, None, "state")),
                    )
                })
                .map_err(|e| e.to_string())?;
            states.push(StateTrace {
                title: crate::read_out::state_title(i, k, s, &cands),
                commitment: name(&from),
                steps,
                full_kb: whole,
            });
        }
    }
    let mut dead: Vec<DeadTrace> = Vec::new();
    if let Some(d) = r.described() {
        // Every member's core is the run's, taken after the walk where the
        // walk took none (M1f S1f.5.12, `region::Cores::nogoods`); a member
        // with none was described by a run not asked for N's cores.
        for x in &d.dead {
            if x.core_status.is_none() {
                return Err(format!(
                    "N's member {} carries no core: the run was not asked for N's cores \
                     (region::Cores::nogoods)",
                    set_name(&x.set)
                ));
            }
            dead.push(DeadTrace {
                set: set_name(&x.set),
                core: x.core.clone(),
                core_status: x.core_status,
                refuted_by: x.refuted_by.clone(),
            });
        }
    }
    let owing = match &r.listing {
        Listing::Owing(f) => f
            .iter()
            .map(|o| (set_name(&o.node.set), o.node.owed.clone()))
            .collect(),
        _ => Vec::new(),
    };
    let listed = r.listed().count();
    Ok(region_markdown(
        &RegionTrace {
            head,
            root,
            untraced: listed.saturating_sub(states.len()),
            states,
            dead,
            owing,
            reorder,
        },
        diagrams,
    ))
}
