//! The KB-shape dump — a test instrument, not engine output.
//!
//! [P1a.2](../../../../docs/history/m1a_rust/README.md#p1a2--kb-core)'s gate was
//! "every corpus file loads to the same KB", and a KB has no CLI surface of
//! its own: the registries, the seven indexes and the participation counts
//! were exactly the things a harness comparing two `ein` *processes* could not
//! see. This renders all of them as one deterministic text, which was diffed
//! against `utils/ir_oracle.py`'s `kb-shape` op until S1a.10.2 and is
//! `corpus_shapes.md5`'s `load` op since.
//!
//! Two rules make the text comparable:
//!
//! - **Facts are named by their position** in the fact list, so the very first
//!   disagreement a diff can show is a fact-order one.
//! - **Values are rendered with `repr`**, so `7` the integer and `7` the atom
//!   cannot collide, and every map is emitted in a sorted order rather than
//!   its own — ein.py's `names` dict is built over a *set* union, and its
//!   order is not reproducible even run to run.

use crate::entities::NameCategory;
use crate::facts::FactId;
use crate::kb::{Kb, SlotKey};
use crate::prov::ProvKind;
use crate::pyrepr::{PyValue, repr};
use crate::terms::Terms;
use crate::value::Value;

/// Render `kb` as the parity text.
pub fn shape(kb: &Kb, terms: &Terms) -> String {
    let facts: Vec<FactId> = kb.facts().collect();
    let index = |id: FactId| {
        facts
            .iter()
            .position(|&f| f == id)
            .map_or("?".to_string(), |i| i.to_string())
    };
    let ids = |list: &[FactId]| list.iter().map(|&f| index(f)).collect::<Vec<_>>().join(",");
    let program = kb.program();
    let mut out: Vec<String> = Vec::new();

    for (i, &f) in facts.iter().enumerate() {
        out.push(format!("F {i} {}", identity(terms, f)));
    }
    for (name, rel) in program.relations.iter() {
        let sig: Vec<String> = rel.signature.iter().map(|&s| terms.sym(s).into()).collect();
        out.push(format!(
            "REL {} sig=({}) declared={} why={}",
            terms.sym(name),
            sig.join(" "),
            py_bool(rel.declared),
            repr(&PyValue::Str(
                rel.why.map_or("", |w| terms.sym(w)).to_string()
            ))
        ));
    }
    for (kind, registry) in [("RULE", &program.rules), ("HRULE", &program.hrules)] {
        for (name, rule) in registry.iter() {
            let params: Vec<String> = rule.params.iter().map(|&p| terms.sym(p).into()).collect();
            out.push(format!(
                "{kind} {} params=({}) priority={} why={} vars=({}) rels=({})",
                terms.sym(name),
                params.join(" "),
                rule.priority
                    .map_or("None".to_string(), |p| terms.int_text(p).to_string()),
                repr(&PyValue::Str(
                    rule.why.map_or("", |w| terms.sym(w)).to_string()
                )),
                pattern_names(terms, rule.match_.as_ref().map(|p| &*p.variables)),
                pattern_names(terms, rule.match_.as_ref().map(|p| &*p.relation_names)),
            ));
        }
    }
    for (name, mac) in program.macros.iter() {
        let params: Vec<String> = mac.params.iter().map(|&p| terms.sym(p).into()).collect();
        out.push(format!(
            "MACRO {} params=({})",
            terms.sym(name),
            params.join(" ")
        ));
    }
    // One line per `(query …)` block, in source order. Identical to the
    // pre-M1c single line for the 0- and 1-query files that are the whole
    // corpus; a second block used to be discarded at load and so could not
    // reach a digest at all.
    if program.queries.is_empty() {
        out.push("QUERY None".to_string());
    } else {
        for q in &program.queries {
            out.push(format!("QUERY {}", q.kw_pairs.len()));
        }
    }
    match &program.config {
        None => out.push("CONFIG None".to_string()),
        Some(c) => {
            for (name, value) in crate::config::rendered_fields(c) {
                out.push(format!("CONFIG {name}={value}"));
            }
        }
    }

    // The seven indexes, each enumerated in a sorted order of its own.
    let mut relations: Vec<_> = facts.iter().map(|&f| terms.facts.rel(f)).collect();
    relations.sort_by_key(|&r| terms.syms.rank(r));
    relations.dedup();
    for rel in relations {
        out.push(format!(
            "EXTENT {} {}",
            terms.sym(rel),
            ids(&kb.facts_of(rel).collect::<Vec<_>>())
        ));
    }
    let mut slots: Vec<(String, SlotKey)> = Vec::new();
    for &f in &facts {
        let (rel, args) = terms.facts.get(f);
        for (slot, value) in args.iter().enumerate() {
            if value.tag() != crate::value::Tag::Fact {
                // `DIRECT` on purpose: this dump is a parity artefact and
                // enumerates the keys ein.py's index has, not the ones
                // T1a.6.3.0 added underneath it.
                let key = SlotKey::direct(rel, slot as u16, *value);
                slots.push((
                    format!("{} {} {}", terms.sym(rel), slot, render(terms, *value)),
                    key,
                ));
            }
        }
    }
    slots.sort_by(|a, b| a.0.cmp(&b.0));
    slots.dedup_by(|a, b| a.0 == b.0);
    for (label, key) in slots {
        out.push(format!(
            "PSI {label} {}",
            ids(&kb.facts_with(key).collect::<Vec<_>>())
        ));
    }
    let mut names = kb.names();
    names.sort_by_key(|&n| terms.syms.rank(n));
    for name in names {
        out.push(format!(
            "NAME {} {} head=({}) arg=({})",
            terms.sym(name),
            category(kb.category(terms, name)),
            ids(&kb.name_as_head(name).collect::<Vec<_>>()),
            ids(&kb.name_as_arg(name).collect::<Vec<_>>()),
        ));
    }
    let mut negated: Vec<String> = kb.negated().map(|f| identity(terms, f)).collect();
    negated.sort();
    for inner in negated {
        out.push(format!("NEG {inner}"));
    }
    for (name, _) in program.rules.iter() {
        let apps = kb.rule_apps_by_rule(name).collect::<Vec<_>>();
        if !apps.is_empty() {
            out.push(format!("RULEAPP {} {}", terms.sym(name), ids(&apps)));
        }
    }
    for (name, _) in program.relations.iter() {
        let apps = kb.rule_apps_on_relation(name).collect::<Vec<_>>();
        if !apps.is_empty() {
            out.push(format!("RELAPP {} {}", terms.sym(name), ids(&apps)));
        }
        let rules = kb.rules_of_relation(name);
        if !rules.is_empty() {
            let names: Vec<&str> = rules.iter().map(|&r| terms.sym(r)).collect();
            out.push(format!("RULESREL {} {}", terms.sym(name), names.join(",")));
        }
    }
    for (i, &f) in facts.iter().enumerate() {
        if let Some(p) = kb.primary(f) {
            let prov = terms.provs.get(p);
            let detail = match prov.kind {
                ProvKind::Source => format!(
                    "source={}",
                    prov.source
                        .map_or("None".to_string(), |s| repr(&PyValue::Str(
                            terms.sym(s).to_string()
                        )))
                ),
                ProvKind::Rule => format!(
                    "rule={} using=[{}]",
                    prov.rule
                        .map_or("None".to_string(), |r| terms.sym(r).to_string()),
                    prov.premises
                        .iter()
                        .map(|&p| identity(terms, p))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                ProvKind::Hypothesis | ProvKind::Rejected => format!(
                    "branch={}",
                    prov.branch.map_or("None".to_string(), |b| b.to_string())
                ),
            };
            out.push(format!("PROV {i} {} {detail}", prov.kind.as_str()));
        }
        let alts = kb.alternatives(f);
        if !alts.is_empty() {
            out.push(format!("ALT {i} {}", alts.len()));
        }
    }
    out.join("\n")
}

/// `repr((relation_name, args))` — a fact's identity tuple as CPython prints
/// it, which is unambiguous across the three value shapes.
fn identity(terms: &Terms, f: FactId) -> String {
    let (rel, args) = terms.facts.get(f);
    repr(&PyValue::Tuple(vec![
        PyValue::Str(terms.sym(rel).to_string()),
        PyValue::Tuple(args.iter().map(|&a| terms.py_value(a)).collect()),
    ]))
}

fn render(terms: &Terms, v: Value) -> String {
    repr(&terms.py_value(v))
}

fn pattern_names(terms: &Terms, names: Option<&[crate::intern::Symbol]>) -> String {
    names.map_or(String::new(), |ns| {
        ns.iter()
            .map(|&n| terms.sym(n))
            .collect::<Vec<_>>()
            .join(" ")
    })
}

fn category(c: NameCategory) -> &'static str {
    c.as_str()
}

fn py_bool(b: bool) -> &'static str {
    if b { "True" } else { "False" }
}
