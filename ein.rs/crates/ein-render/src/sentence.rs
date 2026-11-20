//! The texts a trace step reads
//! ([`docs/ein/user/03-syntax-and-semantics/14-natural-language.md`](../../../../docs/ein/user/03-syntax-and-semantics/14-natural-language.md)).
//!
//! **A `:why` reads the form it is written on** (Q-M1f.58). A rule's, filled
//! with a firing's bindings, is the firing's *reason*, why its conclusions
//! hold, [`rule_sentence`], which the trace's step and the DOT slice both
//! fill through, so a rule's `:why` is filled one way. A declaration's is
//! what the declaration says, its *statement*, `{?n}` its n-th arrow,
//! [`statement`]. A fact of a relation has no `:why` of its own and takes
//! none from the declaration: it reads by its `:source`, its rule's reason,
//! or as the trace prints it, `R(a, b)`.
//!
//! **No invented prose.** Where no form the program wrote carries a text,
//! nothing is said beyond the fact itself.

use ein_core::{FactId, Program, Symbol, Terms, Value, render_why};

/// A rule's `:why`, filled with a firing's bindings; `None` where the rule
/// carries none, or an empty one.
pub fn rule_sentence(
    program: &Program,
    terms: &Terms,
    rule: Symbol,
    bindings: &[(Symbol, Value)],
) -> Option<String> {
    let rule = program
        .rules
        .get(rule)
        .or_else(|| program.hypotheses.get(rule))?;
    let template = terms.sym(rule.why?);
    if template.is_empty() {
        return None;
    }
    let named: Vec<(String, String)> = bindings
        .iter()
        .map(|(k, v)| (terms.sym(*k).to_string(), terms.display(*v)))
        .collect();
    Some(render_why(template, &named))
}

/// What a declaration says: its `:why`, read on the form it is written on.
///
/// `(relation owns Person Pet :why "a {?3} {?2} a {?4}")` is the fact
/// `(relation owns Person Pet)`, and `{?n}` is its n-th arrow: `{?1}` the
/// atom `relation`, `{?2}` the relation's name, `{?k+2}` its k-th label; it
/// reads *a Person owns a Pet*. Only that fact reads by the text, however a
/// run reaches it: a fact of the relation, `(owns Ann Dog)`, has no `:why` of
/// its own and gets none from the declaration, and the companion the loader
/// stores beside a typed declaration, `(relation owns)`, is no form the
/// program wrote. `None` for every other fact, and where the declaration
/// carries no `:why`, or an empty one.
pub fn statement(program: &Program, terms: &Terms, f: FactId) -> Option<String> {
    let (rel, args) = terms.fact(f);
    if rel != terms.kernel.relation {
        return None;
    }
    let name = args.first()?.as_sym()?;
    // The declaration that wrote this fact, of the several a relation may
    // have (Q-M1f.447).
    let declared = program.declarations.iter().find(|d| {
        d.name == name
            && args.len() == d.signature.len() + 1
            && args[1..]
                .iter()
                .zip(d.signature.iter())
                .all(|(a, &label)| a.as_sym() == Some(label))
    })?;
    let template = terms.sym(declared.why?);
    if template.is_empty() {
        return None;
    }
    let arrows: Vec<(String, String)> = std::iter::once(Value::sym(rel))
        .chain(args.iter().copied())
        .enumerate()
        .map(|(i, a)| ((i + 1).to_string(), terms.display(a)))
        .collect();
    Some(render_why(template, &arrows))
}
