//! The constraint-scope → DOT renderer.
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
//!
//! **A fact under an arrow of a drawn fact is its own node**
//! (Q-M1f.395), as in every drawing of facts: an octagon
//! with the relation's name in it and its argument arrows numbered, in a
//! dashed frame named for the relations of the facts that point at it where
//! the program does not state the fact, and in no frame where it does
//! (Q-M1f.398).

use std::collections::{BTreeSet, HashMap, HashSet};

use ein_ir::{Ast, Node, NodeId};

use crate::dot_util::{
    FRAMED_RANKING, GROUND_SHAPE, HYPER_SHAPE, TYPE_SHAPE, ctx_frames, ctx_label, digraph_open,
    quote, value_label,
};
use crate::palette::hash_color;

/// A form's arguments but its clauses.
fn positional(ast: &Ast, form: NodeId) -> Vec<NodeId> {
    ast.form_args(form)
        .iter()
        .copied()
        .filter(|a| !matches!(ast.node(*a), Node::KwPair { .. }))
        .collect()
}

/// The facts under an arrow of a drawn fact, by their text, in the order
/// first met, each with the relations of the facts that point at it.
#[derive(Default)]
struct Nested {
    found: Vec<(NodeId, BTreeSet<String>)>,
    at: HashMap<String, usize>,
}

impl Nested {
    /// Everything `fact` points at is pointed at by a drawn fact, at any
    /// depth; `names` gathers the names met on the way, which are nodes.
    fn walk(&mut self, ast: &Ast, fact: NodeId, by: &str, names: &mut Vec<String>) {
        for arg in positional(ast, fact) {
            let text = value_label(ast, arg);
            if !matches!(ast.node(arg), Node::SForm { .. }) {
                if !names.contains(&text) {
                    names.push(text);
                }
                continue;
            }
            let i = match self.at.get(&text) {
                Some(&i) => i,
                None => {
                    let i = self.found.len();
                    self.at.insert(text, i);
                    self.found.push((arg, BTreeSet::new()));
                    let inner = ast.head_name(arg).unwrap_or("").to_string();
                    self.walk(ast, arg, &inner, names);
                    i
                }
            };
            self.found[i].1.insert(by.to_string());
        }
    }
}

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
    let mut nested = Nested::default();
    // The names under the arrows of a fact that is under an arrow: nodes,
    // declared after the facts' own.
    let mut inner_names: Vec<String> = Vec::new();

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
        let labels: Vec<String> = positional(ast, *decl)
            .iter()
            .map(|a| value_label(ast, *a))
            .collect();
        nested.walk(ast, *decl, &prop, &mut inner_names);
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

    // A fact under an arrow is a node of its own, and so is each fact and
    // each name under its arrows, whether or not a drawn fact names it.
    for (fact, _) in &nested.found {
        let text = value_label(ast, *fact);
        if !relations.contains(&text) {
            relations.push(text);
        }
    }
    for name in &inner_names {
        if !relations.contains(name) {
            relations.push(name.clone());
        }
    }
    // What the program states: a fact under an arrow stands in no frame
    // where it is one.
    let stated: HashSet<String> = forms
        .iter()
        .filter(|f| matches!(ast.node(**f), Node::SForm { .. }))
        .map(|f| {
            let head = ast.head_name(*f).unwrap_or("");
            let args: Vec<String> = positional(ast, *f)
                .iter()
                .map(|a| value_label(ast, *a))
                .collect();
            if args.is_empty() {
                format!("({head})")
            } else {
                format!("({head} {})", args.join(" "))
            }
        })
        .collect();

    // Nodes — relations as boxes, badged with their unary properties; a fact
    // under an arrow as an octagon, in its frame where it is in a context
    // only.
    let mut framed: Vec<(String, String)> = Vec::new();
    for rel in &relations {
        let props = unary.iter().find(|(k, _)| k == rel).map(|(_, v)| v);
        if let Some(&i) = nested.at.get(rel) {
            let (fact, by) = &nested.found[i];
            let head = ast.head_name(*fact).unwrap_or("");
            let colour = hash_color(head);
            let label = match props {
                Some(props) => format!("{head}\\n«{}»", props.join(", ")),
                None => head.to_string(),
            };
            let decl = format!(
                "{} [shape={HYPER_SHAPE}, label={}, color=\"{colour}\", fontcolor=\"{colour}\"];",
                quote(rel),
                quote(&label)
            );
            if stated.contains(rel) {
                lines.push(format!("  {decl}"));
            } else {
                framed.push((ctx_label(by), decl));
            }
            continue;
        }
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
    if !framed.is_empty() {
        lines.push(format!("  {FRAMED_RANKING};"));
    }
    lines.extend(ctx_frames("", "  ", &framed));
    for (fact, _) in &nested.found {
        let colour = hash_color(ast.head_name(*fact).unwrap_or(""));
        let from = quote(&value_label(ast, *fact));
        for (i, arg) in positional(ast, *fact).iter().enumerate() {
            lines.push(format!(
                "  {from} -> {} [label=\"{}\", color=\"{colour}\"];",
                quote(&value_label(ast, *arg)),
                i + 1
            ));
        }
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
