//! The dumps' shared serialisers.
//!
//! Engine-agnostic projections of a fact, a firing or a whole KB into ein
//! source text or machine-parseable JSON. The walk's `00_timeline.jsonl`
//! writer went with the walk at M1e2 S1e2.1c.5.

use ein_core::{FactId, Kb, ProvKind, Tag, Terms, Value};
use ein_infer::firing::Firing;
use ein_ir::{Ast, Node, NodeId, dump_canonical};

use super::json::Json;

/// Lower a fact argument to an IR node.
fn arg_to_node(scratch: &mut Ast, terms: &Terms, arg: Value) -> NodeId {
    match arg.tag() {
        Tag::Fact => fact_to_sform(scratch, terms, None, arg.as_fact().expect("tagged Fact")),
        Tag::Int => {
            let sym = scratch.intern(terms.int_text(arg.as_int().expect("tagged Int")));
            scratch.push(Node::Int(sym), None)
        }
        // `Atom(name=str(arg))` — every non-fact, non-int argument.
        Tag::Sym => {
            let sym = scratch.intern(terms.sym(arg.as_sym().expect("tagged Sym")));
            scratch.push(Node::Atom(sym), None)
        }
    }
}

/// A fact as `(rel arg0 arg1 … :source "…" / :rule "…" / :hypothesis N)`.
///
/// A nested fact argument is lowered recursively **without** its keywords, so
/// it reads as a bare `(rel args…)` inside the outer form — which is why `kb`
/// is an `Option` rather than a plain reference: `None` means "no keywords".
fn fact_to_sform(scratch: &mut Ast, terms: &Terms, kb: Option<&Kb>, f: FactId) -> NodeId {
    let (rel, args) = terms.fact(f);
    let args = args.to_vec();
    let mut children: Vec<NodeId> = args
        .into_iter()
        .map(|a| arg_to_node(scratch, terms, a))
        .collect();
    if let Some(kb) = kb
        && let Some(prov) = kb.primary(f).map(|p| terms.provs.get(p))
    {
        let kw = |scratch: &mut Ast, key: &str, value: NodeId| {
            let name = scratch.intern(key);
            let key = scratch.push(Node::Keyword(name), None);
            scratch.push(Node::KwPair { key, value }, None)
        };
        match prov.kind {
            // Both skipped when empty, so an empty string or a missing name
            // falls through to no keyword at all.
            ProvKind::Source => {
                if let Some(s) = prov.source.map(|s| terms.sym(s)).filter(|s| !s.is_empty()) {
                    let sym = scratch.intern(s);
                    let value = scratch.push(Node::Str(sym), None);
                    let pair = kw(scratch, "source", value);
                    children.push(pair);
                }
            }
            ProvKind::Rule => {
                if let Some(r) = prov.rule.map(|r| terms.sym(r)).filter(|r| !r.is_empty()) {
                    let sym = scratch.intern(r);
                    let value = scratch.push(Node::Str(sym), None);
                    let pair = kw(scratch, "rule", value);
                    children.push(pair);
                }
            }
            ProvKind::Hypothesis => {
                let sym = scratch.intern(&prov.branch.unwrap_or(0).to_string());
                let value = scratch.push(Node::Int(sym), None);
                let pair = kw(scratch, "hypothesis", value);
                children.push(pair);
            }
            ProvKind::Rejected => {}
        }
    }
    // The keyword records the origin for a reader. A reload keeps the fact and
    // not always the origin: the loader reads `:source "…"`, takes `:rule` only
    // as an atom, and has no `:hypothesis` (`dump_states_load.rs`).
    let name = scratch.intern(terms.sym(rel));
    let head = scratch.push(Node::Atom(name), None);
    scratch.sform(head, &children, None)
}

/// A KB as a **flat** sequence of ein forms, which loads as a program.
///
/// The block wrappers are gone: each fact is a top-level form and where it
/// came from rides on its provenance. Un-annotated facts are emitted first
/// purely for readability; the round trip is order-independent.
///
/// A typed declaration `(relation R T…)` is stored with an arity-1 companion
/// `(relation R)`, which the loader adds itself on reading the declaration
/// and refuses as a second one if the file states it. So the companion is left
/// out wherever its declaration is written, and the file loads back to the
/// same facts (`dump_states_load.rs`).
pub fn kb_to_ein_text(kb: &Kb, terms: &Terms) -> String {
    let relation = terms.kernel.relation;
    let typed: rustc_hash::FxHashSet<Value> = kb
        .facts()
        .filter_map(|f| {
            let (rel, args) = terms.fact(f);
            (rel == relation && args.len() > 1).then(|| args[0])
        })
        .collect();
    let companion = |f: FactId| {
        let (rel, args) = terms.fact(f);
        rel == relation && args.len() == 1 && typed.contains(&args[0])
    };
    let mut scratch = Ast::new();
    let (mut ont, mut rest): (Vec<NodeId>, Vec<NodeId>) = (Vec::new(), Vec::new());
    for f in kb.facts().filter(|&f| !companion(f)) {
        let node = fact_to_sform(&mut scratch, terms, Some(kb), f);
        // Given is a `:source`-kind with a source; derived is any non-source
        // kind. Everything else is ontology and goes first.
        let prov = kb.primary(f).map(|p| terms.provs.get(p));
        let given = prov.is_some_and(|p| p.kind == ProvKind::Source && p.source.is_some());
        let derived = prov.is_some_and(|p| p.kind != ProvKind::Source);
        if given || derived {
            &mut rest
        } else {
            &mut ont
        }
        .push(node);
    }
    ont.extend(rest);
    dump_canonical(&scratch, &ont)
}

/// Some of a KB's facts as top-level ein forms, in the order given, each with
/// the provenance keyword [`kb_to_ein_text`] gives it — so `:rule`, `:source`
/// and `:hypothesis` read back as the origin they record.
///
/// M1e2 S1e2.1.2's: a `--dump-answer` state file chooses its facts (the
/// program's schema is not repeated, so the file loads) and their order
/// (sorted, so two model files diff line by line), which the whole-KB writer
/// above does not let a caller do.
pub fn facts_to_ein_text(kb: &Kb, terms: &Terms, facts: &[FactId]) -> String {
    let mut scratch = Ast::new();
    let nodes: Vec<NodeId> = facts
        .iter()
        .map(|&f| fact_to_sform(&mut scratch, terms, Some(kb), f))
        .collect();
    dump_canonical(&scratch, &nodes)
}

/// A firing for JSONL output — rule, activator, bindings, the primary derived
/// fact, and the premises.
pub fn firing_to_json(terms: &Terms, firing: &Firing) -> Json {
    let bindings: Vec<(String, Json)> = firing
        .bindings
        .iter()
        .map(|(k, v)| {
            let text = match v.tag() {
                // A `Fact` binding is dropped to a marker — chase it through
                // the premises instead.
                Tag::Fact => {
                    let (rel, _) = terms.fact(v.as_fact().expect("tagged Fact"));
                    format!("<fact:{}>", terms.sym(rel))
                }
                _ => terms.display(*v),
            };
            (terms.sym(*k).to_string(), Json::Str(text))
        })
        .collect();
    let fact_json = |f: FactId| {
        let (rel, args) = terms.fact(f);
        Json::obj(vec![
            ("relation", Json::str(terms.sym(rel))),
            (
                "args",
                Json::Array(args.iter().map(|a| Json::str(terms.display(*a))).collect()),
            ),
        ])
    };
    // The derived side shows the *primary* conclusion; a nested fact argument
    // renders as its own `{relation, args}` object rather than as text.
    let (drel, dargs) = terms.fact(firing.derived[0]);
    let derived = Json::obj(vec![
        ("relation", Json::str(terms.sym(drel))),
        (
            "args",
            Json::Array(
                dargs
                    .iter()
                    .map(|a| match a.tag() {
                        Tag::Fact => fact_json(a.as_fact().expect("tagged Fact")),
                        _ => Json::str(terms.display(*a)),
                    })
                    .collect(),
            ),
        ),
    ]);
    Json::obj(vec![
        ("rule", Json::str(terms.sym(firing.rule))),
        (
            "activator",
            Json::Array(
                firing
                    .activator
                    .iter()
                    .map(|s| Json::str(terms.sym(*s)))
                    .collect(),
            ),
        ),
        ("bindings", Json::Object(bindings)),
        ("redundant", Json::Bool(firing.redundant)),
        ("derived", derived),
        (
            "premises",
            Json::Array(firing.premises.iter().map(|p| fact_json(*p)).collect()),
        ),
    ])
}

/// A fact as a recursive `{relation, args}` object — nested facts nest, and
/// everything else stringifies.
pub fn fact_summary(terms: &Terms, f: FactId) -> Json {
    let (rel, args) = terms.fact(f);
    Json::obj(vec![
        ("relation", Json::str(terms.sym(rel))),
        (
            "args",
            Json::Array(
                args.iter()
                    .map(|a| match a.tag() {
                        Tag::Fact => fact_summary(terms, a.as_fact().expect("tagged Fact")),
                        _ => Json::str(terms.display(*a)),
                    })
                    .collect(),
            ),
        ),
    ])
}
