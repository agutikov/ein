//! Closed-relation inference — the auto `(__closed__ R)` pass.
//!
//! A relation is *closed* — the hypothesis generator must never speculate
//! facts of it — when **no rule can positively conclude an R-fact**. Such a
//! relation's extension is fixed by the puzzle's given facts: no inference
//! path reaches a new R-fact, so a hypothesis about R could never be confirmed
//! by saturation; it would only bloat the search.
//!
//! `is-a` (the inheritance forest) and `right-of` (the house row) of the Zebra
//! puzzle are closed by this test; `co-located` and `next-to` — propagated by
//! `symmetric` / `transitive` / `implies` / `square-*` — are not.
//!
//! [`emit_closed`] replaces hand-written declarations, and the marker may also
//! be authored directly or derived by `std.closure`'s `infer-closure`
//! (functional ∧ total ⇒ closed). [`crate::hypgen`] does not care which path
//! produced the fact — the index lookup is the same.
//!
//! **Where it is called from matters, and it is not `solve`.** Both ein.py
//! call sites — `cli/solve.py`'s `--hyp-stats` preview and `cli/_summary.py`'s
//! root observables — run it on a **fork**, so the search itself sees every
//! relation open. It moves a lot when it does run: over the corpus it takes
//! `closed_relation` from 6 pre-candidate skips to 278 and `raw` from 4 479
//! candidates to 3 022.

use ein_core::{Symbol, Value};
use rustc_hash::FxHashSet;

use crate::compile::{CompileError, asserted_relation};
use crate::saturator::Session;

/// The kernel's closed-relation trigger. Per the dunder convention the
/// kernel-hardcoded behaviour keys on `__closed__`, **not** the bare `closed`,
/// which is a free userspace name.
pub const CLOSED: &str = crate::hypgen::CLOSED;

/// Relation names some compiled rule positively asserts.
///
/// Walks the compiled `(rule, activator)` plans; a plan whose `:assert`
/// template is `(R …)` — head not `not` — proves `R` is rule-derivable. A T2
/// rule contributes once per activator, so a relation reachable only through
/// an *un-activated* rule is correctly absent.
pub fn producible_relations(s: &mut Session<'_>) -> Result<FxHashSet<Symbol>, CompileError> {
    let mut engine = crate::engine::Engine::with_memo(s.memo.clone());
    engine.compile_all(s.ast, s.terms, s.kb, s.events)?;
    let terms = &*s.terms;
    Ok((0..engine.len())
        .filter_map(|i| asserted_relation(engine.plan(i), terms))
        .collect())
}

/// Write a `(__closed__ R)` fact for every declared relation no rule can
/// positively conclude; returns the newly-closed names in registry order.
///
/// Idempotent: a relation already carrying the marker is left alone, so an
/// authored declaration and a re-run both no-op. Run **before** the initial
/// saturation, so `hypgen` sees the facts.
///
/// Only declared *domain* relations carry a signature; the property /
/// rule-name relations (`symmetric`, …) do not, and are never hypothesis
/// targets anyway.
pub fn emit_closed(s: &mut Session<'_>) -> Result<Vec<Symbol>, CompileError> {
    let producible = producible_relations(s)?;
    let closed = s.terms.kernel.closed;
    let already: FxHashSet<Symbol> =
        s.kb.facts_of(closed)
            .filter_map(|f| s.terms.facts.args(f).first().and_then(|v| v.as_sym()))
            .collect();
    let declared: Vec<Symbol> =
        s.kb.program()
            .relations
            .values()
            .filter(|r| !r.signature.is_empty())
            .map(|r| r.name)
            .collect();
    let mut newly = Vec::new();
    for name in declared {
        if producible.contains(&name) || already.contains(&name) {
            continue;
        }
        s.kb.add_and_index_fact(s.terms, closed, &[Value::sym(name)], None)
            .expect("room for a closure marker");
        newly.push(name);
    }
    Ok(newly)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile::SharedMemo;
    use ein_core::{Kb, Terms};
    use ein_ir::{Ast, from_ir::load, parse};

    /// `emit_closed` is run **per fork**, by `--hyp-stats` and by the JSON
    /// summary, so running it twice over the same state must be a no-op. The
    /// corpus sweep only ever runs it once; what it does check is the other
    /// half — that an *authored* `(__closed__ R)` is left alone.
    #[test]
    fn a_second_pass_closes_nothing_new() {
        let src = "(relation open-r T T)\n(relation shut-r T T)\n\
                   (rule fill () :match (shut-r ?a ?b) :assert (open-r ?a ?b))";
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let mut kb: Kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let mut ev = crate::events::Events::off();
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut ev,
            memo: SharedMemo::default(),
        };
        let first: Vec<String> = emit_closed(&mut s)
            .expect("compiles")
            .iter()
            .map(|&n| s.terms.sym(n).to_string())
            .collect();
        // `open-r` is what the rule concludes, so only `shut-r` closes.
        assert_eq!(first, ["shut-r"]);
        assert!(emit_closed(&mut s).expect("compiles").is_empty());
        let closed = s.terms.syms.get(CLOSED).expect("interned");
        assert_eq!(s.kb.n_facts_of(closed), 1, "the marker was written twice");
    }
}
