//! `--dump-lattice DIR` under a region mode — M1e2
//! [T1e2.1a.5.3](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md):
//! every node the run visited, whatever the route, in visit order.
//!
//! Two files, `nodes.json` (`ein-lattice/2`) and `nodes.md`, written after the
//! run from what the route's watch saw ([`ein_infer::events::Watch`]): each
//! node's commitment, its kind (`dead`, `owing`, `in-r`), what it owes, its
//! core and refuting rules when dead, its closure, and how many facts its
//! fixpoint holds beyond root's; and each step, a route-1 layer or a mode's
//! batch. The firings of each node's saturation are `--events`', which
//! narrates every node visited, so the directory does not hold a second copy.
//!
//! The walk's tree (`enterings/`, `proof_summary.json`) was its
//! `LatticeDumper`'s, which went with the walk at M1e2 S1e2.1c.5.

use std::path::Path;

use ein_infer::events::Stepped;

use super::json::{Json, dumps_indent};

/// The schema `nodes.json` carries.
pub const SCHEMA: &str = "ein-lattice/2";

/// One node, as the watch saw it.
#[derive(Clone, Debug, Default)]
pub struct VisitedNode {
    pub commitment: Vec<String>,
    pub kind: &'static str,
    pub owes: u32,
    pub owed: Vec<String>,
    pub core: Vec<String>,
    /// What the core's search established (M1e2 S1e2.2.23); `None` where
    /// none was taken.
    pub core_status: Option<ein_infer::explain::CoreStatus>,
    pub refuted_by: Vec<String>,
    pub closure: Vec<String>,
    pub facts: u64,
}

/// What a run visited: the nodes in visit order, and each step with how many
/// nodes had been visited before it began.
#[derive(Clone, Debug, Default)]
pub struct Visits {
    pub nodes: Vec<VisitedNode>,
    pub steps: Vec<(Stepped, usize)>,
}

/// What the directory says about the run.
pub struct Inputs<'a> {
    pub source: &'a str,
    /// The read-out's header: the command as typed, less what it leaves out.
    pub argv: &'a str,
    /// `1, the alive down-set` or `2, the cube`.
    pub route: &'a str,
    pub candidates: &'a [String],
}

fn strs(v: &[String]) -> Json {
    Json::Array(v.iter().map(Json::str).collect())
}

fn set(v: &[String]) -> String {
    if v.is_empty() {
        "∅".to_string()
    } else {
        format!("{{{}}}", v.join(", "))
    }
}

fn kind_word(kind: &str) -> &str {
    match kind {
        "in-r" => "in R",
        k => k,
    }
}

/// `nodes.json`.
pub fn json(v: &Visits, inputs: &Inputs<'_>) -> Json {
    let nodes = v
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| {
            Json::obj(vec![
                ("n", Json::int(i as i64)),
                ("commitment", strs(&n.commitment)),
                ("kind", Json::str(n.kind)),
                ("owes", Json::int(i64::from(n.owes))),
                ("owed", strs(&n.owed)),
                ("core", strs(&n.core)),
                (
                    "core_status",
                    super::region_answer::core_status(n.core_status),
                ),
                ("refuted_by", strs(&n.refuted_by)),
                ("closure", strs(&n.closure)),
                ("facts", Json::int(n.facts as i64)),
            ])
        })
        .collect();
    let steps = v
        .steps
        .iter()
        .map(|(s, from)| {
            Json::obj(vec![
                ("layer", s.layer.map_or(Json::Null, |l| Json::int(l as i64))),
                ("from", Json::int(*from as i64)),
                ("visited", Json::int(s.visited as i64)),
                ("dead", Json::int(s.dead as i64)),
                ("owing", Json::int(s.owing as i64)),
                ("in_r", Json::int(s.r as i64)),
                ("facts", Json::int(s.facts as i64)),
            ])
        })
        .collect();
    Json::obj(vec![
        ("schema", Json::str(SCHEMA)),
        ("source", Json::str(inputs.source)),
        ("asked", Json::str(inputs.argv)),
        ("route", Json::str(inputs.route)),
        ("candidates", strs(inputs.candidates)),
        ("visited", Json::int(v.nodes.len() as i64)),
        ("steps", Json::Array(steps)),
        ("nodes", Json::Array(nodes)),
    ])
}

/// `nodes.md`: one table per step, the nodes in visit order.
pub fn markdown(v: &Visits, inputs: &Inputs<'_>) -> String {
    let mut out = vec![
        format!("# Visited nodes — `{}`", inputs.source),
        String::new(),
        format!(
            "`ein solve {}`: {} node{} by {}, in visit order, over {} candidate{}. \
             `nodes.json` holds the same, and `--events` each node's firings.",
            inputs.argv,
            v.nodes.len(),
            if v.nodes.len() == 1 { "" } else { "s" },
            by_route(inputs.route),
            inputs.candidates.len(),
            if inputs.candidates.len() == 1 {
                ""
            } else {
                "s"
            },
        ),
        String::new(),
    ];
    let row = |i: usize, n: &VisitedNode| -> String {
        let why = match n.kind {
            "dead" if !n.core.is_empty() => match n.core_status.and_then(|c| c.cut_at()) {
                Some(at) => format!(
                    "core {} (core search cut at its budget of {at})",
                    n.core.join(", ")
                ),
                None => format!("core {}", n.core.join(", ")),
            },
            "owing" => format!("owes {}", n.owed.join("; ")),
            _ => String::new(),
        };
        format!(
            "| {} | {} | {} | {} | {} | {} |",
            i + 1,
            set(&n.commitment),
            kind_word(n.kind),
            set(&n.closure),
            n.facts,
            why.replace('|', "\\|")
        )
    };
    let head = [
        "| # | commitment | kind | closure | facts | why |".to_string(),
        "|---:|---|---|---|---:|---|".to_string(),
    ];
    let mut at = 0usize;
    for (i, (s, from)) in v.steps.iter().enumerate() {
        let name = match s.layer {
            Some(l) => format!("Layer {l}"),
            None => format!("Step {}", i + 1),
        };
        out.push(format!(
            "## {name} — {} visited: {} dead, {} owing, {} in R",
            s.visited, s.dead, s.owing, s.r
        ));
        out.push(String::new());
        out.extend(head.iter().cloned());
        let end = (*from + s.visited as usize).min(v.nodes.len());
        for (j, n) in v.nodes.iter().enumerate().take(end).skip(*from) {
            out.push(row(j, n));
        }
        out.push(String::new());
        at = end;
    }
    if at < v.nodes.len() {
        out.push("## After the last step".to_string());
        out.push(String::new());
        out.push(
            "A node visited outside a step: the route's one saturation of ⊤ when its budget \
             stopped it."
                .to_string(),
        );
        out.push(String::new());
        out.extend(head.iter().cloned());
        for (j, n) in v.nodes.iter().enumerate().skip(at) {
            out.push(row(j, n));
        }
        out.push(String::new());
    }
    format!("{}\n", out.join("\n").trim_end())
}

/// Write `nodes.json` and `nodes.md` into `dir`, which exists.
pub fn write(dir: &Path, v: &Visits, inputs: &Inputs<'_>) -> std::io::Result<()> {
    std::fs::write(
        dir.join("nodes.json"),
        dumps_indent(&json(v, inputs)) + "\n",
    )?;
    std::fs::write(dir.join("nodes.md"), markdown(v, inputs))
}

/// `ein render lattice FILE`: the nodes a describing run visited, as a DAG —
/// R green, owing amber, dead red, ∅ at the bottom, an edge from each node to
/// every visited node one candidate above it. Under `full` every node is
/// drawn; otherwise R's nodes, root, and the nodes one fact above a node of
/// R that are not in it: where R stops.
pub fn dot(v: &Visits, inputs: &Inputs<'_>, full: bool) -> String {
    use crate::dot_util::{digraph_open, multiline, quote};
    use rustc_hash::FxHashMap;

    let index: FxHashMap<&[String], usize> = v
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.commitment.as_slice(), i))
        .collect();
    let parents = |n: &VisitedNode| -> Vec<usize> {
        (0..n.commitment.len())
            .filter_map(|k| {
                let mut p = n.commitment.clone();
                p.remove(k);
                index.get(p.as_slice()).copied()
            })
            .collect()
    };
    let in_r = |i: usize| v.nodes[i].kind == "in-r";
    let drawn: Vec<bool> = v
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| {
            full || in_r(i) || n.commitment.is_empty() || parents(n).into_iter().any(in_r)
        })
        .collect();
    let mut out = digraph_open(
        "lattice",
        Some("BT"),
        Some("shape=box, style=\"rounded,filled\", fontname=\"Helvetica\", fontsize=10"),
    );
    let shown = drawn.iter().filter(|d| **d).count();
    out.push(format!(
        "  label={};",
        quote(&format!(
            "{} — {} of {} nodes visited by {}{}",
            inputs.source,
            shown,
            v.nodes.len(),
            by_route(inputs.route),
            if full { "" } else { ": R and where it stops" }
        ))
    ));
    out.push("  labelloc=t;".to_string());
    let mut by_size: Vec<Vec<usize>> = Vec::new();
    for (i, n) in v.nodes.iter().enumerate() {
        if !drawn[i] {
            continue;
        }
        let (border, fill) = match n.kind {
            "in-r" => ("#2ca02c", "#e8f6e8"),
            "owing" => ("#ff7f0e", "#fff2e5"),
            _ => ("#d62728", "#fdeaea"),
        };
        let parts: Vec<&str> = if n.commitment.is_empty() {
            vec!["∅"]
        } else {
            n.commitment.iter().map(String::as_str).collect()
        };
        out.push(format!(
            "  n{i} [label={}, color={}, fillcolor={}];",
            multiline(&parts),
            quote(border),
            quote(fill)
        ));
        let k = n.commitment.len();
        if by_size.len() <= k {
            by_size.resize(k + 1, Vec::new());
        }
        by_size[k].push(i);
    }
    for layer in by_size.iter().filter(|l| l.len() > 1) {
        let ids: Vec<String> = layer.iter().map(|i| format!("n{i}")).collect();
        out.push(format!("  {{ rank=same; {}; }}", ids.join("; ")));
    }
    for (i, n) in v.nodes.iter().enumerate() {
        if !drawn[i] {
            continue;
        }
        for p in parents(n) {
            if drawn[p] {
                out.push(format!("  n{p} -> n{i};"));
            }
        }
    }
    out.push("}".to_string());
    format!("{}\n", out.join("\n"))
}

/// `route 1, the alive down-set` for a numbered route, and the state route's
/// name as it is.
fn by_route(route: &str) -> String {
    if route.starts_with(|c: char| c.is_ascii_digit()) {
        format!("route {route}")
    } else {
        route.to_string()
    }
}
