//! `ein solve --print-final-*` — a listed state's KB as canonical
//! s-expressions.
//!
//! A fact is *not* rendered here: the event protocol's fact renderer,
//! `ein_infer::events::sexpr`, is reused rather than duplicated.

use ein_core::{Kb, Symbol, Terms};

/// The three `--print-final-*` modes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `-p` — everything the engine derived: the propositional residue.
    All,
    /// `-P` — `All` with the `(not …)` facts dropped: the positive residue.
    Positive,
    /// `-f` — only the facts of the candidates' relations, *whatever their
    /// origin*, so the given conditions count too.
    Hfacts,
}

/// A slice of a state's facts in canonical order, as the lines the form
/// prints: its label, then one fact per line.
pub fn final_state_lines(terms: &Terms, kb: &Kb, mode: Mode, targets: &[Symbol]) -> Vec<String> {
    let (mut facts, label): (Vec<_>, String) = match mode {
        Mode::Hfacts => {
            let picked: Vec<_> = kb
                .facts()
                .filter(|&f| targets.contains(&terms.facts.get(f).0))
                .collect();
            let mut names: Vec<&str> = targets.iter().map(|&s| terms.sym(s)).collect();
            names.sort_unstable();
            let shown: Vec<String> = names.iter().map(|n| ein_core::repr::repr_str(n)).collect();
            (
                picked,
                format!(
                    "positive hypothesis-relation facts; :hrules [{}]",
                    shown.join(", ")
                ),
            )
        }
        _ => {
            let not = terms.syms.get("not");
            let picked: Vec<_> = kb
                .facts()
                .filter(|&f| is_derived(kb, terms, f))
                .filter(|&f| !(mode == Mode::Positive && Some(terms.facts.get(f).0) == not))
                .collect();
            (
                picked,
                if mode == Mode::Positive {
                    "derived facts, (not …) omitted".to_string()
                } else {
                    "derived facts".to_string()
                },
            )
        }
    };

    // Keyed on the relation name, then the args *rendered* rather than the
    // raw ids, so the order is the one a reader sees.
    let mut keyed: Vec<(&str, Vec<String>, ein_core::FactId)> = facts
        .drain(..)
        .map(|f| {
            let (rel, args) = terms.facts.get(f);
            let rendered = args
                .iter()
                .map(|a| ein_infer::events::sexpr_value(terms, *a))
                .collect();
            (terms.sym(rel), rendered, f)
        })
        .collect();
    keyed.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));

    let mut out = vec![format!(
        "final-state facts ({label}; {} facts):",
        keyed.len()
    )];
    for (rel, args, _) in &keyed {
        // Note the unconditional separator: a nullary fact prints `(rel )`
        // here, where `sexpr` would spell it `(rel)`. This line has its own
        // join, and the difference is observable.
        out.push(format!("  ({rel} {})", args.join(" ")));
    }
    out
}

/// Derived — produced by the engine, i.e. it has provenance and that
/// provenance is not a `:source` ingestion. A fact with none is neither given
/// nor derived, so it drops out of every `--print-final-*` mode but `hfacts`.
fn is_derived(kb: &Kb, terms: &Terms, fact: ein_core::FactId) -> bool {
    match kb.primary(fact) {
        Some(p) => terms.provs.get(p).kind != ein_core::ProvKind::Source,
        None => false,
    }
}
