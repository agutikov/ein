//! A fact in English — M1f S1f.5.14's T1f.5.14.2, fact-level rendering from
//! provenance ([`docs/kernel/ir/03-ein-lang/09_natural_language.md`](../../../../docs/kernel/ir/03-ein-lang/09_natural_language.md)).
//!
//! **Two texts a fact can have, from two places.** Its *statement* is what the
//! fact says, its relation's `:why` with the fact's arguments in its
//! positional references (`{?1}`, `{?2}`, …). Its *reason* is why it holds, read
//! off its **first recorded justification** ([`Kb::primary`]): the `:source`
//! sentence of a given, or the deriving rule's `:why` filled with that
//! firing's bindings. The trace's step sentence is the second, which
//! [`rule_sentence`] lifts out of `trace/linearize.rs` so the trace, the DOT
//! slice and a `--dump-answer` state file fill a rule's `:why` one way.
//!
//! **Which justification: the first recorded**, the one a fact's provenance
//! names before its alternatives. A fact can carry up to
//! `MAX_ALT_JUSTIFICATIONS` more, and which arrived first can move with rule
//! order; a reason is an explanation, as a core is, and the user's ruling of
//! 2026-11-08 leaves both to F22 to make canonical. Recording order is
//! deterministic in a run (design/02), so the text is under the same invariance
//! `id_order_invariance` and `jobs_invariance` hold the rest of the answer to.
//!
//! **No invented prose.** A fact whose relation carries no `:why` has no
//! statement, and one whose justification has no text has no reason; where
//! both are missing the fact reads as its s-expression, as every surface wrote
//! it before. The engine adds the provenance's own words (`rule r`, `source
//! "…"`, `hypothesis N`), which `reserved_engine_strings.md` owns, and nothing
//! of the domain's.

use ein_core::{FactId, Kb, Program, ProvKind, Symbol, Terms, Value, render_why};

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

/// Where a fact came from, by its first recorded justification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Origin {
    /// Stated by the program with a `:source` sentence.
    Source(String),
    /// Derived by the named rule.
    Rule(String),
    /// The commitment's: a hypothesis, the branch's number.
    Hypothesis(u32),
    /// Stated by the program with no sentence, or recorded with no
    /// provenance at all.
    Given,
}

/// A fact for a reader: where it came from, why it holds, and what it says.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FactSentence {
    pub origin: Origin,
    /// The deriving rule's `:why`, filled; `None` for every other origin.
    pub reason: Option<String>,
    pub statement: Option<String>,
}

impl FactSentence {
    /// Does any text of the program's render this fact — a `:source`, a rule's
    /// `:why`, or its relation's? The census's *rendered*: where this is
    /// false the fact reads as its s-expression.
    pub fn renders(&self) -> bool {
        matches!(self.origin, Origin::Source(_))
            || self.reason.is_some()
            || self.statement.is_some()
    }
}

/// `f` for a reader, from its first recorded justification in `kb` and the
/// program's templates.
pub fn fact_sentence(kb: &Kb, terms: &Terms, f: FactId) -> FactSentence {
    let program = kb.program();
    let prov = kb.primary(f).map(|p| terms.provs.get(p));
    let (origin, reason) = match prov {
        None => (Origin::Given, None),
        Some(p) => match p.kind {
            ProvKind::Source => match p.source.map(|s| terms.sym(s)).filter(|s| !s.is_empty()) {
                Some(s) => (Origin::Source(s.to_string()), None),
                None => (Origin::Given, None),
            },
            ProvKind::Rule => match p.rule.filter(|r| !terms.sym(*r).is_empty()) {
                Some(r) => (
                    Origin::Rule(terms.sym(r).to_string()),
                    rule_sentence(program, terms, r, &p.bindings),
                ),
                None => (Origin::Given, None),
            },
            ProvKind::Hypothesis => (Origin::Hypothesis(p.branch.unwrap_or(0)), None),
            // A rejected hypothesis is never a fact of a state; it reads as
            // its statement alone where a caller meets one.
            ProvKind::Rejected => (Origin::Given, None),
        },
    };
    FactSentence {
        origin,
        reason,
        statement: statement(program, terms, f),
    }
}

/// A text as the rest of a `;` comment: each line break (`\n`, `\r`, or the
/// pair) starts a new comment line, so the text is carried character for
/// character and the file still reads as the facts it states.
pub fn as_comment(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next();
                }
                out.push_str("\n;   ");
            }
            '\n' => out.push_str("\n;   "),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::as_comment;

    #[test]
    fn a_line_break_continues_the_comment() {
        assert_eq!(as_comment("a\nb\rc\r\nd"), "a\n;   b\n;   c\n;   d");
        assert_eq!(as_comment("plain"), "plain");
    }
}
