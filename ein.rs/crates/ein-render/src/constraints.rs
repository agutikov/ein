//! The constraint-scope → DOT renderer — `ein.py`'s `render/constraints.py`.
//!
//! A puzzle's *structural constraints* are the **un-annotated**
//! rule-application facts — the implicit "co-located is symmetric / color-loc
//! is a bijection" context the solver supplies, as opposed to the explicit
//! puzzle conditions. They are identified structurally: an ontology fact
//! whose head is neither a kernel keyword (`relation` / `type` / `instance`)
//! nor a declared relation name. That captures property activators such as
//! `bijective` while excluding relation data such as `(is-a House Attribute)`
//! — no hardcoded property list, so the view tracks whatever rules a puzzle
//! declares.

use ein_ir::{Ast, Node, NodeId};

use crate::dot_util::{GROUND_SHAPE, HYPER_SHAPE, TYPE_SHAPE, digraph_open, quote, value_label};
use crate::palette::hash_color;

/// Kernel ontology keywords — declarations, not constraint facts.
const KERNEL_ONTOLOGY_HEADS: [&str; 3] = ["relation", "type", "instance"];

const NON_FACT_HEADS: [&str; 8] = [
    "rule",
    "hrule",
    "query",
    "trace",
    "config",
    // the deprecated wrapper forms
    "ontology",
    "facts",
    "reasoning",
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

fn render_group(ast: &Ast, form: NodeId) -> &'static str {
    let kw = kw_names(ast, form);
    if kw.contains(&"rule") || kw.contains(&"using") {
        "reasoning"
    } else if kw.contains(&"source") {
        "fact"
    } else {
        "ontology"
    }
}

/// A flat top-level form in the ontology group — a relation decl or any
/// un-annotated fact, but not a rule / query / trace / config, a `:source`d
/// condition, or a `:rule`-derived fact.
fn is_ontology_form(ast: &Ast, f: NodeId) -> bool {
    let Node::SForm { head, .. } = ast.node(f) else {
        return false;
    };
    let Node::Atom(s) = ast.node(head) else {
        return false;
    };
    if NON_FACT_HEADS.contains(&ast.sym(s)) {
        return false;
    }
    render_group(ast, f) == "ontology"
}

/// Relation names declared via `(relation Name …)` among `decls`.
fn declared_relations<'a>(ast: &'a Ast, decls: &[NodeId]) -> Vec<&'a str> {
    let mut out = Vec::new();
    for decl in decls {
        if ast.head_name(*decl) == Some("relation")
            && let Some(&first) = ast.form_args(*decl).first()
            && let Node::Atom(s) = ast.node(first)
        {
            out.push(ast.sym(s));
        }
    }
    out
}

/// Render the structural-constraint scopes of a parsed program.
///
/// `forms` is the tuple of top-level forms. A deprecated `(ontology …)`
/// wrapper is used when present; otherwise the ontology group is synthesised
/// from the relation decls and un-annotated facts among the top-level forms.
pub fn render_constraints(ast: &Ast, forms: &[NodeId], name: &str) -> String {
    let wrapper = forms
        .iter()
        .copied()
        .find(|f| ast.head_name(*f) == Some("ontology"));
    let decls: Vec<NodeId> = match wrapper {
        Some(w) => ast.form_args(w).to_vec(),
        None => forms
            .iter()
            .copied()
            .filter(|f| is_ontology_form(ast, *f))
            .collect(),
    };
    let declared_rel = declared_relations(ast, &decls);

    // relation → [property, …]; lookups only, so the map's own order is not
    // observable — `relations` below is what fixes the emission order.
    let mut unary: Vec<(String, Vec<String>)> = Vec::new();
    let mut binary: Vec<(String, String, String)> = Vec::new();
    let mut nary: Vec<(String, Vec<String>)> = Vec::new();
    let mut relations: Vec<String> = Vec::new(); // node labels, first-seen
    let mut types: Vec<String> = Vec::new(); // binary-edge targets, drawn as types

    for decl in &decls {
        let Node::SForm { head, .. } = ast.node(*decl) else {
            continue;
        };
        let Node::Atom(sym) = ast.node(head) else {
            continue;
        };
        let prop = ast.sym(sym).to_string();
        if KERNEL_ONTOLOGY_HEADS.contains(&prop.as_str()) || declared_rel.contains(&prop.as_str()) {
            continue;
        }
        let labels: Vec<String> = ast
            .form_args(*decl)
            .iter()
            .copied()
            .filter(|a| !matches!(ast.node(*a), Node::KwPair { .. }))
            .map(|a| value_label(ast, a))
            .collect();
        let note = |label: &str, relations: &mut Vec<String>| {
            if !relations.iter().any(|r| r == label) {
                relations.push(label.to_string());
            }
        };
        match labels.len() {
            1 => {
                match unary.iter_mut().find(|(k, _)| *k == labels[0]) {
                    Some(slot) => slot.1.push(prop),
                    None => unary.push((labels[0].clone(), vec![prop])),
                }
                note(&labels[0], &mut relations);
            }
            2 => {
                binary.push((prop, labels[0].clone(), labels[1].clone()));
                note(&labels[0], &mut relations);
                note(&labels[1], &mut relations);
                if !types.contains(&labels[1]) {
                    types.push(labels[1].clone());
                }
            }
            n if n >= 3 => {
                for lbl in &labels {
                    note(lbl, &mut relations);
                }
                nary.push((prop, labels));
            }
            _ => {}
        }
    }

    let mut lines = digraph_open(name, Some("LR"), Some("fontname=\"Inter\""));
    if relations.is_empty() {
        lines.push(
            "  // no structural-constraint facts found (rule-headed ontology facts)".to_string(),
        );
        lines.push("}".to_string());
        return lines.join("\n");
    }

    // Nodes — relations as boxes, badged with their unary properties.
    for rel in &relations {
        let props = unary.iter().find(|(k, _)| k == rel).map(|(_, v)| v);
        let shape = if types.contains(rel) && props.is_none() {
            TYPE_SHAPE
        } else {
            GROUND_SHAPE
        };
        let label = match props {
            Some(props) => format!("{rel}\\n«{}»", props.join(", ")),
            None => rel.clone(),
        };
        lines.push(format!(
            "  {} [shape={shape}, label={}];",
            quote(rel),
            quote(&label)
        ));
    }

    // Binary properties → labelled, property-coloured edges.
    for (prop, a, b) in &binary {
        let colour = hash_color(prop);
        lines.push(format!(
            "  {} -> {} [label={}, color=\"{colour}\", fontcolor=\"{colour}\"];",
            quote(a),
            quote(b),
            quote(prop)
        ));
    }

    // Arity-≥3 properties → Levi octagon.
    for (i, (prop, args)) in nary.iter().enumerate() {
        let colour = hash_color(prop);
        let h = quote(&format!("c{}_{prop}", i + 1));
        lines.push(format!(
            "  {h} [shape={HYPER_SHAPE}, label={}, color=\"{colour}\", fontcolor=\"{colour}\"];",
            quote(prop)
        ));
        for (j, arg) in args.iter().enumerate() {
            lines.push(format!(
                "  {h} -> {} [label=\"{}\", color=\"{colour}\"];",
                quote(arg),
                j + 1
            ));
        }
    }

    lines.push("}".to_string());
    lines.join("\n")
}
