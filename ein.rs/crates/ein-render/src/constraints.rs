//! The constraint-scope → DOT renderer.
//!
//! A puzzle's *structural constraints* are the **un-annotated** facts of
//! the relations it does not declare — the "co-located is symmetric /
//! color-loc is a bijection" context a program supplies, as opposed to its
//! stated conditions about its declared relations. They are identified
//! structurally: an un-annotated fact whose head is no declared relation's
//! name. No name is read for what it means: no property list, and no
//! relation of a theory's, so the view tracks whatever a program states. A
//! declaration is the fact the loader stores, of `relation`, which no program
//! declares, and is drawn as one.
//!
//! **Each is drawn as a fact** (Q-M1f.419), by [`crate::fact_graph`] as the
//! KB view draws one: `(symmetric co-located)` an octagon whose first arrow
//! is at `symmetric` and whose second is at `co-located`, never a property
//! written into the relation's node. A fact under an arrow of one is its own
//! node, in the frame of its context where the program does not state it
//! (Q-M1f.395, Q-M1f.398).

use ein_ir::{Ast, Node, NodeId};

use crate::dot_util::{POLYGON_RANKSEP, digraph_open};
use crate::ir_dot::fact_set_of;

/// The declarators: a form headed by one is no fact.
const NON_FACT_HEADS: [&str; 8] = [
    "rule", "hrule", "macro", "query", "solve", "config", "import", "trace",
];

fn kw_names(ast: &Ast, form: NodeId) -> Vec<&str> {
    ast.form_args(form)
        .iter()
        .filter_map(|a| match ast.node(*a) {
            Node::KwPair { key, .. } => match ast.node(key) {
                Node::Keyword(s) => Some(ast.sym(s)),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

/// A top-level fact the program writes with no provenance: no `:source`,
/// no `:rule` and no `:using`.
fn is_unannotated_fact(ast: &Ast, f: NodeId) -> bool {
    let Node::SForm { head, .. } = ast.node(f) else {
        return false;
    };
    let Node::Atom(s) = ast.node(head) else {
        return false;
    };
    if NON_FACT_HEADS.contains(&ast.sym(s)) {
        return false;
    }
    let kw = kw_names(ast, f);
    !(kw.contains(&"source") || kw.contains(&"rule") || kw.contains(&"using"))
}

/// Relation names declared via `(relation Name …)` among `forms`.
fn declared_relations<'a>(ast: &'a Ast, forms: &[NodeId]) -> Vec<&'a str> {
    let mut out = Vec::new();
    for decl in forms {
        if ast.head_name(*decl) == Some("relation")
            && let Some(&first) = ast.form_args(*decl).first()
            && let Node::Atom(s) = ast.node(first)
        {
            out.push(ast.sym(s));
        }
    }
    out
}

/// Render the structural-constraint facts of a parsed program, each in the
/// canonical form, or folded where `fold`.
pub fn render_constraints(ast: &Ast, forms: &[NodeId], name: &str, fold: bool) -> String {
    let facts: Vec<NodeId> = forms
        .iter()
        .copied()
        .filter(|f| is_unannotated_fact(ast, *f))
        .collect();
    let declared = declared_relations(ast, &facts);
    let structural: Vec<NodeId> = facts
        .iter()
        .copied()
        .filter(|f| !declared.contains(&ast.head_name(*f).unwrap_or("")))
        .collect();
    let mut lines = digraph_open(name, Some("LR"), Some("fontname=\"Inter\", shape=box"));
    lines.push(format!("  {POLYGON_RANKSEP};"));
    if structural.is_empty() {
        lines.push(
            "  // no structural-constraint facts found (rule-headed ontology facts)".to_string(),
        );
        lines.push("}".to_string());
        return lines.join("\n");
    }
    lines.extend(fact_set_of(ast, forms, &structural).body(fold));
    lines.push("}".to_string());
    lines.join("\n")
}
