//! The texts a trace step reads
//! ([`docs/ein/user/03-syntax-and-semantics/14-natural-language.md`](../../../../docs/ein/user/03-syntax-and-semantics/14-natural-language.md)).
//!
//! **Two texts, from two places.** A fact's *statement* is what the fact
//! says, its relation's `:why` with the fact's arguments in its positional
//! references (`{?1}`, `{?2}`, …), [`statement`]. A firing's *reason* is why
//! its conclusions hold, the rule's `:why` filled with the firing's bindings,
//! [`rule_sentence`], which the trace's step and the DOT slice both fill
//! through, so a rule's `:why` is filled one way.
//!
//! **No invented prose.** A fact whose relation carries no `:why` has no
//! statement, and a rule with none gives no reason; where both are missing
//! the fact reads as the trace prints it, `R(a, b)`.

use ein_core::{FactId, Program, Symbol, Terms, Value, render_why};

/// A rule's `:why`, filled with a firing's bindings; `None` where the rule
/// (or hrule) carries none, or an empty one.
pub fn rule_sentence(
    program: &Program,
    terms: &Terms,
    rule: Symbol,
    bindings: &[(Symbol, Value)],
) -> Option<String> {
    let rule = program
        .rules
        .get(rule)
        .or_else(|| program.hrules.get(rule))?;
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

/// A fact's statement: its relation's `:why`, with `{?1}` its first argument,
/// `{?2}` its second, …; `None` where the relation carries none.
pub fn statement(program: &Program, terms: &Terms, f: FactId) -> Option<String> {
    let (rel, args) = terms.fact(f);
    let template = terms.sym(program.relations.get(rel)?.why?);
    if template.is_empty() {
        return None;
    }
    let positional: Vec<(String, String)> = args
        .iter()
        .enumerate()
        .map(|(i, a)| ((i + 1).to_string(), terms.display(*a)))
        .collect();
    Some(render_why(template, &positional))
}
