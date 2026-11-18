//! The derivation DAG as a Graphviz `digraph`.
//!
//! **Each fact is drawn as its node** (Q-M1f.419): an octagon with no name in
//! it, its first arrow at its relation's atom and the next at its
//! arguments, never a node labelled with its text; beside it, what it rests
//! on, its source's text, `by <rule>` or the hypothesis that put it there.
//! **Each derivation is a node of its own**, a small diamond, with the
//! premises feeding it and it feeding the conclusion, so that a fact with
//! several recorded derivations shows them as alternatives rather than as
//! one big conjunction. The graph stands in layers, as a knowledge base does
//! ([`crate::fact_graph`]), each premise above the derivation it feeds, and
//! the derivation stands out as a slice's does (Q-M1f.428), in the first hue
//! of [`crate::slice::DERIVATION_PALETTE`]: a fact a derivation added, one it
//! concludes whose first record is a rule's, its octagon and its arrows, in
//! the hue's derived shade at [`crate::slice::DERIVED_WIDTH`]; each diamond
//! and the arrows into and out of it in the explanation's shade at twice
//! Graphviz's default width; every other fact in the default black at the
//! default width.

use std::collections::HashSet;

use crate::dot_util::{HYPER_SHAPE, POLYGON_RANKSEP, fact_key, hashed_id, quote};
use crate::fact_graph::{FACT_NODE, first_arrow};
use crate::slice::{DERIVATION_PALETTE, DERIVATION_WIDTH, DERIVED_WIDTH};
use ein_core::{DerivationDag, FactId, Kb, ProvKind, Terms};

/// Render `dag` — the bytes `ein-render/tests/golden/dot/kb_provenance_dag.dot`
/// pins.
pub fn derivation_dag_to_dot(dag: &DerivationDag, kb: &Kb, terms: &Terms) -> String {
    let mut lines = vec![
        "digraph derivation {".to_string(),
        "  rankdir=TB;".to_string(),
        format!("  {POLYGON_RANKSEP};"),
        "  node [shape=box];".to_string(),
    ];
    // A derivation records no rule here: every one is drawn in the
    // palette's first hue.
    let hue = DERIVATION_PALETTE[0];
    // The facts a derivation added: concluded here, and recorded first as a
    // rule's. A given fact a derivation also concludes was there before it.
    let added: HashSet<FactId> = dag
        .and_nodes
        .iter()
        .map(|(c, _)| *c)
        .filter(|c| {
            kb.primary(*c)
                .is_some_and(|p| terms.provs.get(p).kind == ProvKind::Rule)
        })
        .collect();
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
        // A fact a derivation added: the brightest, and wide; its note
        // stays black, text being read where a line is only seen.
        let tail = if added.contains(&f) {
            format!(", color=\"{}\", penwidth={DERIVED_WIDTH}", hue.derived)
        } else {
            String::new()
        };
        lines.push(format!(
            "  {nid} [shape={HYPER_SHAPE}, {FACT_NODE}{tail}{note}];"
        ));
        let (rel, args) = terms.fact(f);
        arrows.push(format!(
            "  {}",
            first_arrow(&nid, &quote(terms.sym(rel)), &tail)
        ));
        for (j, a) in args.iter().enumerate() {
            let to = match a.as_fact() {
                Some(g) => {
                    queue.push(g);
                    fact_dot_id(terms, g)
                }
                None => quote(&terms.display(*a)),
            };
            arrows.push(format!("  {nid} -> {to} [xlabel=\"{}\"{tail}];", j + 2));
        }
    }
    lines.extend(arrows);
    let attrs = format!("color=\"{}\", {DERIVATION_WIDTH}", hue.explanation);
    for (i, (conclusion, premises)) in dag.and_nodes.iter().enumerate() {
        let jid = format!("j{i}");
        lines.push(format!(
            "  {jid} [shape=diamond, width=.2, height=.2, label=\"\", {attrs}];"
        ));
        for &p in premises {
            lines.push(format!("  {} -> {jid} [{attrs}];", fact_dot_id(terms, p)));
        }
        lines.push(format!(
            "  {jid} -> {} [{attrs}];",
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
