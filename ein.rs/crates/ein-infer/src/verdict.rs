//! A query's goal, projected over a state: its keyword values, its binding
//! rows ([`goal_bindings`]) and whether its plan compiles
//! ([`goal_plan_error`]) — what the read-out answers each `(query …)` with.
//!
//! The module was the walk's verdict types too — `Solution`, `Ambiguity`,
//! `Contradiction` and `Open` over the models it recorded, and the count that
//! qualified them. They went with the walk at M1e2 S1e2.1c.5 (2026-11-03):
//! `--single`'s four words are [`crate::explore::Verdict`], read off `R`.

use ein_core::entities::{ExprRef, Pattern, Query, Rule};
use ein_core::{Kb, Symbol, Terms, Value};
use ein_ir::{Ast, Node, NodeId};

// ── The query's goal, projected ────────────────────────────────────

/// A `(query …)` keyword's value, by keyword name.
///
/// The **first** match wins.
pub fn query_value(ast: &Ast, query: &Query, kw_name: &str) -> Option<NodeId> {
    kw_value(ast, &query.kw_pairs, kw_name)
}

/// A `(solve …)` keyword's value, by keyword name — M1e2 S1e2.1a.4, where the
/// lattice's keywords moved. The **first** match wins, as in [`query_value`].
pub fn solve_value(ast: &Ast, solve: &ein_core::Solve, kw_name: &str) -> Option<NodeId> {
    kw_value(ast, &solve.kw_pairs, kw_name)
}

fn kw_value(ast: &Ast, kw_pairs: &[ein_core::ExprRef], kw_name: &str) -> Option<NodeId> {
    for &pair in kw_pairs.iter() {
        let Node::KwPair { key, value } = ast.node(NodeId(pair.0)) else {
            continue;
        };
        if let Node::Keyword(name) = ast.node(key)
            && ast.sym(name) == kw_name
        {
            return Some(value);
        }
    }
    None
}

/// Run the query's `:goal` pattern against `kb` and return the binding rows.
///
/// The same matcher machinery the solve loop counts matches with — here the
/// rows come back so a caller can project an answer, which is what the CLI's
/// solution table and `:goal-text` do. `goal` defaults to the KB's own
/// `(query :goal …)`; pass one explicitly to project a different question over
/// a solved model.
///
/// There is no free-standing pattern compiler —
/// [`compile_rule`](crate::compile_rule) is the entry point — so the goal is
/// wrapped in a parameter-less synthetic rule named `#query`
/// ([`ein_core::terms::QUERY_RULE`]), which compiles
/// to the pattern's own steps: no activator to bind, no `:assert` templates,
/// no `:why`.
pub fn goal_bindings(
    ast: &Ast,
    terms: &mut Terms,
    kb: &Kb,
    goal: Option<NodeId>,
) -> Vec<Vec<(Symbol, Value)>> {
    let goal = match goal {
        Some(g) => g,
        None => {
            let query = match kb.program().query() {
                Some(q) => q,
                None => return Vec::new(),
            };
            match query_value(ast, query, "goal") {
                Some(g) => g,
                None => return Vec::new(),
            }
        }
    };
    let rule = Rule {
        name: terms.kernel.query_rule,
        params: Box::new([]),
        match_: Some(Pattern {
            expr: ExprRef(goal.0),
            variables: Box::new([]),
            relation_names: Box::new([]),
        }),
        assert_: None,
        why: None,
        loc: None,
    };
    let plan = match crate::compile::compile_rule(ast, terms, &rule, None) {
        Ok(plan) => plan,
        // A goal the compiler rejects — `(query :goal (?R Rex Animal))`, an
        // unbound relation head — ends the *whole run* rather than projecting
        // nothing. Callers that render a model ask [`goal_plan_error`] first;
        // this one keeps its infallible signature.
        Err(_) => return Vec::new(),
    };
    let mut rows: Vec<Vec<(Symbol, Value)>> = Vec::new();
    let mut matcher = crate::match_::Matcher::new();
    let mut judge = crate::match_::Matcher::new();
    matcher.run(kb, terms, ast, &plan, &mut |m| {
        // A top-level `(absent …)` is lifted out of the plan, as a rule's is,
        // for the boundary to judge, and a goal has no boundary: it is judged
        // here, against the state, which is a fixpoint. It holds where its
        // query has no match (M1f S1f.5.7's T1f.5.7.6, W56; until then it was
        // never judged, so `(and (p ?y) (absent (q ?y)))` answered `?y = b`
        // beside `(q b)`).
        if m.guards()
            .iter()
            .any(|g| judge.holds(kb, terms, ast, &plan, g, m.regs()))
        {
            return std::ops::ControlFlow::Continue(());
        }
        // The last binding of a repeated name wins, and the row keeps
        // first-bind order, which is what the trace and the table print.
        let mut row: Vec<(Symbol, Value)> = Vec::new();
        for (name, value) in m.bindings() {
            match row.iter_mut().find(|(n, _)| *n == name) {
                Some(slot) => slot.1 = value,
                None => row.push((name, value)),
            }
        }
        rows.push(row);
        std::ops::ControlFlow::Continue(())
    });
    rows
}

/// The error compiling this query goal fails with.
///
/// `None` when there is no goal, or when it compiles. [`goal_bindings`] cannot
/// report it — its signature is infallible — so the CLI asks this before it
/// renders a model, and refuses the run with this error. Found by
/// [S1a.6.6](../../../../docs/history/m1a_rust/README.md#s1a66--the-fuzzer)'s
/// fuzzer on a two-line program; `examples/ein-bugs/query-goal-free-head.ein`
/// is the fixture.
pub fn goal_plan_error(
    ast: &Ast,
    terms: &mut Terms,
    kb: &Kb,
    goal: Option<NodeId>,
) -> Option<crate::compile::CompileError> {
    let goal = match goal {
        Some(g) => g,
        None => query_value(ast, kb.program().query()?, "goal")?,
    };
    let rule = Rule {
        name: terms.kernel.query_rule,
        params: Box::new([]),
        match_: Some(Pattern {
            expr: ExprRef(goal.0),
            variables: Box::new([]),
            relation_names: Box::new([]),
        }),
        assert_: None,
        why: None,
        loc: None,
    };
    crate::compile::compile_rule(ast, terms, &rule, None).err()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_ir::{from_ir::load, parse};

    /// **A goal's `(absent …)` is judged** (M1f S1f.5.7's T1f.5.7.6, W56): the
    /// lifted guard holds where its query has no match in the state, so a row
    /// whose `(q ?y)` the state holds is no row; a `forall`'s nested shape is
    /// judged the same way. Until then the goal's evaluation never judged it,
    /// and `?y = b` was a row beside `(q b)`.
    #[test]
    fn a_goals_absent_is_judged() {
        let rows = |goal: &str| -> Vec<String> {
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let src = format!(
                "(relation p T) (relation q T) (relation r T T)\n(p a) (p b) (q b) (r a a)\n\
                 (query :goal {goal})\n"
            );
            let forms = parse(&mut ast, &src, Some("<test>")).expect("parses");
            let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
            let mut out: Vec<String> = goal_bindings(&ast, &mut terms, &kb, None)
                .into_iter()
                .map(|row| {
                    row.iter()
                        .map(|(k, v)| format!("{}={}", terms.sym(*k), terms.display(*v)))
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect();
            out.sort();
            out
        };
        assert_eq!(rows("(and (p ?y) (absent (q ?y)))"), ["y=a"]);
        assert_eq!(rows("(absent (q ?x))"), Vec::<String>::new(), "a q exists");
        assert_eq!(
            rows("(absent (p c))"),
            [""],
            "no (p c): one row, binding nothing"
        );
        // Every `r` out of `?y` goes to `?y`: `a` has one, to itself, and `b`
        // none, which holds vacuously.
        assert_eq!(
            rows("(and (p ?y) (absent (and (r ?y ?z) (neq ?y ?z))))"),
            ["y=a", "y=b"]
        );
        // A `forall`'s shape, a nested `(absent …)`: is every other `p` one
        // with an `r` to itself? For `a` the other is `b`, which has none; for
        // `b` it is `a`, which has `(r a a)`.
        assert_eq!(
            rows("(and (p ?y) (absent (and (p ?z) (neq ?y ?z) (absent (r ?z ?z)))))"),
            ["y=b"]
        );
    }
}
