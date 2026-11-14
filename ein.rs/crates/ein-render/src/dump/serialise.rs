//! The dumps' shared serialisers.
//!
//! Engine-agnostic projections of a fact, a firing or a whole KB into ein
//! source text or machine-parseable JSON. The walk's `00_timeline.jsonl`
//! writer went with the walk at M1e2 S1e2.1c.5.

use ein_core::{FactId, Kb, ProvKind, Tag, Terms, Value};

use crate::sentence::Origin;
use ein_infer::firing::Firing;

use super::json::Json;

/// A fact's provenance as the comment a state file writes after it: `;
/// source "…"`, `; rule r` or `; hypothesis N`, and nothing for a fact with
/// none (M1e2 S1e2.3.8, the inventory's S-5). A comment, so the loader skips
/// it and a reloaded fact reads as given (`answer_dump.md`). Until S1e2.3.8
/// these were keywords, `:source "…"`, `:rule "r"` and `:hypothesis N`, of
/// which the loader read the first, took `:rule` only as an atom, and had no
/// third, so a reloaded state's facts read as given anyway.
///
/// **And the fact in English** since M1f S1f.5.14's T1f.5.14.2
/// ([`crate::sentence`]): a rule's `:why` filled with the firing's bindings
/// after `: `, and the fact's statement, its relation's `:why`, after ` — `
/// (alone, for a given with no sentence): `; rule r: <reason> — <statement>`.
/// A line break in either starts a new comment line ([`crate::sentence::as_comment`]).
fn provenance_comment(kb: &Kb, terms: &Terms, f: FactId) -> Option<String> {
    let s = crate::sentence::fact_sentence(kb, terms, f);
    let mut text = match &s.origin {
        Origin::Source(sentence) => format!("source {}", ein_core::text::quote(sentence)),
        Origin::Rule(rule) => format!("rule {}", ein_core::text::symbol(rule)),
        Origin::Hypothesis(n) => format!("hypothesis {n}"),
        Origin::Given => String::new(),
    };
    if let Some(reason) = &s.reason {
        text.push_str(": ");
        text.push_str(reason);
    }
    if let Some(statement) = &s.statement {
        if !text.is_empty() {
            text.push_str(" — ");
        }
        text.push_str(statement);
    }
    (!text.is_empty()).then(|| format!("; {}", crate::sentence::as_comment(&text)))
}

/// One fact as one line: the one printer's text ([`Terms::fact_text`]), which
/// loads back to the same fact, and its provenance as a comment.
fn fact_line(kb: &Kb, terms: &Terms, f: FactId) -> String {
    let text = terms.fact_text(f);
    match provenance_comment(kb, terms, f) {
        Some(c) => format!("{text} {c}"),
        None => text,
    }
}

/// A KB as a **flat** sequence of ein forms, one fact per line, which loads
/// as a program.
///
/// The block wrappers are gone: each fact is a top-level form and where it
/// came from rides on its provenance comment. Un-annotated facts are emitted
/// first purely for readability; the round trip is order-independent.
///
/// A typed declaration `(relation R T…)` is stored with an arity-1 companion
/// `(relation R)`, which the loader adds itself on reading the declaration
/// and refuses as a second one if the file states it. So the companion is left
/// out wherever its declaration is written, and the file loads back to the
/// same facts. So is every `(written T)`, which the loader states from the
/// facts the file holds and refuses where a file writes it (M1f S1f.6.38).
pub fn kb_to_ein_text(kb: &Kb, terms: &Terms) -> String {
    let relation = terms.kernel.relation;
    let written = terms.kernel.written;
    let typed: rustc_hash::FxHashSet<Value> = kb
        .facts()
        .filter_map(|f| {
            let (rel, args) = terms.fact(f);
            (rel == relation && args.len() > 1).then(|| args[0])
        })
        .collect();
    let companion = |f: FactId| {
        let (rel, args) = terms.fact(f);
        rel == written || rel == relation && args.len() == 1 && typed.contains(&args[0])
    };
    let (mut ont, mut rest): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
    for f in kb.facts().filter(|&f| !companion(f)) {
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
        .push(fact_line(kb, terms, f));
    }
    ont.extend(rest);
    let mut text = ont.join("\n");
    text.push('\n');
    text
}

/// Some of a KB's facts as top-level ein forms, in the order given, one per
/// line, each with the provenance comment [`kb_to_ein_text`] gives it: the
/// text loads back to the same facts, and every fact reloads as given.
///
/// M1e2 S1e2.1.2's: a `--dump-answer` state file chooses its facts (the
/// program's schema is not repeated, so the file loads) and their order
/// (sorted, so two model files diff line by line), which the whole-KB writer
/// above does not let a caller do.
pub fn facts_to_ein_text(kb: &Kb, terms: &Terms, facts: &[FactId]) -> String {
    let mut text = String::new();
    for &f in facts {
        text.push_str(&fact_line(kb, terms, f));
        text.push('\n');
    }
    text
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
                    .map(|&v| Json::str(terms.display(v)))
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
