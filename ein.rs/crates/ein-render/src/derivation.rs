//! The derivation DAG as a Graphviz `digraph` — `provenance.DerivationDAG.to_dot`.
//!
//! Source and hypothesis facts are ellipses labelled with what they rest on;
//! rule-derived facts are boxes labelled with the rule. An AND/OR graph — one
//! where some fact has more than one recorded derivation — additionally draws
//! a small diamond per justification, with the premises feeding *it* and it
//! feeding the conclusion, so alternative derivations read as alternatives
//! instead of as one big conjunction. A single-derivation DAG renders exactly
//! as it did before S1.21.7.

use crate::dot_util::{fact_key, hashed_id};
use ein_core::{DerivationDag, FactId, Kb, ProvKind, Terms};

/// Render `dag` — byte-identical to ein.py, which
/// `ein.py/tests/golden/dot/kb_provenance_dag.dot` pins.
pub fn derivation_dag_to_dot(dag: &DerivationDag, kb: &Kb, terms: &Terms) -> String {
    let mut lines = vec![
        "digraph derivation {".to_string(),
        "  rankdir=BT;".to_string(),
    ];
    for &f in &dag.nodes {
        let nid = fact_dot_id(terms, f);
        let label = fact_dot_label(kb, terms, f);
        let shape = if is_terminal_node(kb, terms, f) {
            "ellipse"
        } else {
            "box"
        };
        lines.push(format!("  {nid} [shape={shape}, label=\"{label}\"];"));
    }
    if dag.is_or_graph() {
        for (i, (conclusion, premises)) in dag.and_nodes.iter().enumerate() {
            let jid = format!("j{i}");
            lines.push(format!(
                "  {jid} [shape=diamond, width=.2, height=.2, label=\"\"];"
            ));
            for &p in premises {
                lines.push(format!("  {} -> {jid};", fact_dot_id(terms, p)));
            }
            lines.push(format!("  {jid} -> {};", fact_dot_id(terms, *conclusion)));
        }
    } else {
        for &(premise, conclusion) in &dag.edges {
            lines.push(format!(
                "  {} -> {};",
                fact_dot_id(terms, premise),
                fact_dot_id(terms, conclusion)
            ));
        }
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

/// `(rel arg arg)` — with the trailing space a nullary fact gets, because
/// ein.py's f-string puts one there unconditionally.
fn compact(terms: &Terms, f: FactId) -> String {
    let (rel, args) = terms.fact(f);
    let args: Vec<String> = args.iter().map(|a| terms.display(*a)).collect();
    format!("({} {})", terms.sym(rel), args.join(" "))
}

fn is_terminal_node(kb: &Kb, terms: &Terms, f: FactId) -> bool {
    match kb.primary(f) {
        None => true,
        Some(p) => matches!(
            terms.provs.get(p).kind,
            ProvKind::Source | ProvKind::Hypothesis
        ),
    }
}

fn fact_dot_label(kb: &Kb, terms: &Terms, f: FactId) -> String {
    let compact = compact(terms, f);
    let Some(p) = kb.primary(f) else {
        return label_esc(&compact);
    };
    let prov = terms.provs.get(p);
    // `\n` here is the two-character DOT line break, not a newline.
    match prov.kind {
        ProvKind::Source => {
            // `source or 'ontology'` — Python's `or`, so an empty string
            // falls through to the default just as `None` does.
            let source = prov
                .source
                .map(|s| terms.sym(s))
                .filter(|s| !s.is_empty())
                .unwrap_or("ontology");
            label_esc(&format!("{compact}\\n[{source}]"))
        }
        ProvKind::Rule => {
            let rule = prov
                .rule
                .map_or("None".to_string(), |r| terms.sym(r).to_string());
            label_esc(&format!("{compact}\\n[rule: {rule}]"))
        }
        ProvKind::Hypothesis => {
            let branch = prov.branch.map_or("None".to_string(), |b| b.to_string());
            label_esc(&format!("{compact}\\n[hyp #{branch}]"))
        }
        ProvKind::Rejected => label_esc(&compact),
    }
}

/// `provenance._esc` — the quote only.
///
/// Deliberately *not* [`crate::dot_util::esc`], which also escapes the
/// backslash: this renderer builds its own `\n` line breaks, so escaping
/// backslashes here would turn every one of them into a literal.
fn label_esc(s: &str) -> String {
    s.replace('"', "\\\"")
}
