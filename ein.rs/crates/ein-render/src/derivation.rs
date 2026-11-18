//! The derivation DAG as a Graphviz `digraph`.
//!
//! **Each fact is drawn as its node** (Q-M1f.419): an octagon with no name in
//! it, its first arrow at its relation's atom and the next at its
//! arguments, never a node labelled with its text; beside it, what it rests
//! on, its source's text, `by <rule>` or the hypothesis that put it there.
//! **Each derivation is a node of its own**, a small diamond, with the
//! premises feeding it and it feeding the conclusion, so that a fact with
//! several recorded derivations shows them as alternatives rather than as
//! one big conjunction.

use std::collections::HashSet;

use crate::dot_util::{HYPER_SHAPE, POLYGON_RANKSEP, fact_key, hashed_id, quote};
use crate::fact_graph::FACT_NODE;
use ein_core::{DerivationDag, FactId, Kb, ProvKind, Terms};

/// Render `dag` — the bytes `ein-render/tests/golden/dot/kb_provenance_dag.dot`
/// pins.
pub fn derivation_dag_to_dot(dag: &DerivationDag, kb: &Kb, terms: &Terms) -> String {
    let mut lines = vec![
        "digraph derivation {".to_string(),
        "  rankdir=BT;".to_string(),
        format!("  {POLYGON_RANKSEP};"),
        "  node [shape=box];".to_string(),
    ];
    let mut arrows: Vec<String> = Vec::new();
    let mut drawn: HashSet<FactId> = HashSet::new();
    let mut queue: Vec<FactId> = dag.nodes.clone();
    // Each fact of the DAG, and each fact one of them points at, as its node.
    let mut i = 0;
    while i < queue.len() {
        let f = queue[i];
        i += 1;
        if !drawn.insert(f) {
            continue;
        }
        let nid = fact_dot_id(terms, f);
        let note = provenance_note(kb, terms, f)
            .map_or(String::new(), |n| format!(", xlabel={}", quote(&n)));
        lines.push(format!("  {nid} [shape={HYPER_SHAPE}, {FACT_NODE}{note}];"));
        let (rel, args) = terms.fact(f);
        arrows.push(format!(
            "  {nid} -> {} [xlabel=\"1\"];",
            quote(terms.sym(rel))
        ));
        for (j, a) in args.iter().enumerate() {
            let to = match a.as_fact() {
                Some(g) => {
                    queue.push(g);
                    fact_dot_id(terms, g)
                }
                None => quote(&terms.display(*a)),
            };
            arrows.push(format!("  {nid} -> {to} [xlabel=\"{}\"];", j + 2));
        }
    }
    lines.extend(arrows);
    for (i, (conclusion, premises)) in dag.and_nodes.iter().enumerate() {
        let jid = format!("j{i}");
        lines.push(format!(
            "  {jid} [shape=diamond, width=.2, height=.2, label=\"\"];"
        ));
        for &p in premises {
            lines.push(format!(
                "  {} -> {jid} [style=bold];",
                fact_dot_id(terms, p)
            ));
        }
        lines.push(format!(
            "  {jid} -> {} [style=bold];",
            fact_dot_id(terms, *conclusion)
        ));
    }
    lines.push("}".to_string());
    lines.join("\n")
}

/// A stable DOT node id, derived from the fact's identity.
pub fn fact_dot_id(terms: &Terms, f: FactId) -> String {
    let (rel, args) = terms.fact(f);
    let args: Vec<String> = args.iter().map(|a| terms.display(*a)).collect();
    hashed_id("f_", &fact_key(terms.sym(rel), &args), false)
}

/// What a fact rests on, beside its node: its source's text, `by <rule>`, or
/// the hypothesis branch; none for a fact with no record.
fn provenance_note(kb: &Kb, terms: &Terms, f: FactId) -> Option<String> {
    let p = kb.primary(f)?;
    let prov = terms.provs.get(p);
    match prov.kind {
        ProvKind::Source => {
            // An empty source falls through to the default just as an absent
            // one does.
            let source = prov
                .source
                .map(|s| terms.sym(s))
                .filter(|s| !s.is_empty())
                .unwrap_or("ontology");
            Some(source.to_string())
        }
        ProvKind::Rule => Some(format!(
            "by {}",
            prov.rule
                .map_or("None".to_string(), |r| terms.sym(r).to_string())
        )),
        ProvKind::Hypothesis => Some(format!(
            "hyp #{}",
            prov.branch.map_or("None".to_string(), |b| b.to_string())
        )),
        ProvKind::Rejected => None,
    }
}
