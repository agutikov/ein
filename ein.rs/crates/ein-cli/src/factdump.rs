//! `ein solve --print-final-*` — a listed state's KB as canonical
//! s-expressions.
//!
//! A fact is *not* rendered here: the event protocol's fact renderer,
//! `ein_infer::events::sexpr`, is reused rather than duplicated.

use ein_core::{Kb, Symbol, Terms};

/// What `-p`'s and `-P`'s facts are, beyond the program as loaded.
const WHAT_BEYOND: &str = "root's derivations, the commitment and what it derives";

/// The three `--print-final-*` modes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `-p` — every fact beyond the program as loaded: root's derivations,
    /// the commitment and what it derives, the `(not …)` facts among them.
    All,
    /// `-P` — `All` with the `(not …)` facts dropped: the state's model, as
    /// `solution_semantics.md` § 3 defines it.
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
            // The relations as Ein spells names, one space apart: they are
            // the candidates', whether or not the program wrote `:hrules`
            // (M1e2 S1e2.3.7's N1).
            let mut names: Vec<&str> = targets.iter().map(|&s| terms.sym(s)).collect();
            names.sort_unstable();
            let label = if names.is_empty() {
                "of the candidates' relations: there is no candidate".to_string()
            } else {
                format!(
                    "of the candidates' relations, given ones too: {}",
                    names.join(" ")
                )
            };
            (picked, label)
        }
        _ => {
            // Beyond the program as loaded, not beyond root: root's own
            // derivations are in every state's block, and a commitment is
            // not derived (M1e2 S1e2.3.7's N2).
            let not = terms.syms.get("not");
            let picked: Vec<_> = kb
                .facts()
                .filter(|&f| beyond_the_program(kb, terms, f))
                .filter(|&f| !(mode == Mode::Positive && Some(terms.facts.get(f).0) == not))
                .collect();
            (
                picked,
                if mode == Mode::Positive {
                    format!("beyond the program as loaded, (not …) omitted: {WHAT_BEYOND}")
                } else {
                    format!("beyond the program as loaded: {WHAT_BEYOND}")
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

    let n = keyed.len();
    let mut out = vec![format!(
        "final-state facts ({label}; {n} {}):",
        if n == 1 { "fact" } else { "facts" }
    )];
    for (rel, args, _) in &keyed {
        // Note the unconditional separator: a nullary fact prints `(rel )`
        // here, where `sexpr` would spell it `(rel)`. This line has its own
        // join, and the difference is observable.
        out.push(format!("  ({rel} {})", args.join(" ")));
    }
    out
}

/// Beyond the program as loaded: it has provenance, and that provenance is
/// not a `:source` ingestion — a rule's firing (root's included) or a
/// commitment's hypothesis. A fact with none is the loader's, so it drops out
/// of every `--print-final-*` mode but `hfacts`.
fn beyond_the_program(kb: &Kb, terms: &Terms, fact: ein_core::FactId) -> bool {
    // A `(written T)` fact is the loader's statement of what the program
    // writes, derived at load by its step (M1f S1f.6.38): the program as
    // loaded holds it.
    if terms.facts.get(fact).0 == terms.kernel.written {
        return false;
    }
    match kb.primary(fact) {
        Some(p) => terms.provs.get(p).kind != ein_core::ProvKind::Source,
        None => false,
    }
}
