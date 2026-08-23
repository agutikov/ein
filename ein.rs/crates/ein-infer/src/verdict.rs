//! Verdict types — the three *answers* to one problem, not three problems.
//!
//! `k` distinct solution nodes → `Contradiction` / `Solution` / `Ambiguity`,
//! and the verdict is **read from the result**, never chosen up front. That is
//! what makes this P1.7a rather than its unsound predecessors: a solution node
//! is `consistent ∧ complete`, not a goal-pattern match — the distinction
//! S1.7.3 found the hard way, when a partial dead-end was being accepted.
//!
//! The query's `:goal` does not decide the verdict. It projects over the
//! model(s) afterwards.

use ein_core::entities::{ExprRef, Pattern, Query, Rule};
use ein_core::{FactId, Kb, Symbol, Terms, Value};
use ein_ir::{Ast, Node, NodeId};

use crate::firing::Firing;

/// A surviving branch.
pub struct Solution {
    pub kb: Kb,
    pub trace: Vec<Firing>,
}

/// The three verdicts. `Aborted` is deliberately **not** one of them — see
/// [`Answer`].
///
/// A `Solution` carries a whole `Kb`, so the variants differ a lot in size.
/// Boxing it would buy an allocation per verdict and cost a deref at every
/// read, for a value the caller constructs once and consumes once.
#[allow(clippy::large_enum_variant)]
pub enum Verdict {
    /// `k = 1` — the model, unique iff the search was exhausted.
    Solution(Solution),
    /// `k > 1` — that many distinct models, i.e. a genuine gap.
    Ambiguity(Vec<Solution>),
    /// `k = 0` — unsat, if exhausted. The core is the union of the recorded
    /// dead commitments' cores.
    Contradiction { unsat_core: Vec<FactId> },
}

impl Verdict {
    /// The name the CLI and the events print.
    pub fn as_str(&self) -> &'static str {
        match self {
            Verdict::Solution(_) => "Solution",
            Verdict::Ambiguity(_) => "Ambiguity",
            Verdict::Contradiction { .. } => "Contradiction",
        }
    }
}

/// What `solve` returns.
///
/// `Aborted` is kept **outside** [`Verdict`] so exhaustive verdict handling is
/// unaffected: a caller that opted into `on_budget = "verdict"` matches it
/// explicitly. It is not a proven verdict — `solution_nodes == 0` there means
/// *unexplored*, not *proven unsatisfiable*.
#[allow(clippy::large_enum_variant)]
pub enum Answer {
    Verdict(Verdict),
    Aborted { reason: String },
}

impl Answer {
    pub fn as_str(&self) -> &'static str {
        match self {
            Answer::Verdict(v) => v.as_str(),
            Answer::Aborted { .. } => "Aborted",
        }
    }
}

/// The unsat core of a `k = 0` verdict: the union over every recorded dead
/// commitment's core.
pub fn union_dead_cores(cores: &[Vec<FactId>]) -> Vec<FactId> {
    let mut out: Vec<FactId> = cores.iter().flatten().copied().collect();
    // determinism-ok: identity order as a set union's normalisation. The core
    // *is* an answer, so this one is worth being explicit about: every site
    // that renders it re-sorts by text first (`shape.rs`, `slice.rs`,
    // `trace/linearize.rs`), and `ein-parity` compares it as a set.
    out.sort_unstable();
    out.dedup();
    out
}

// ── The query's goal, projected ────────────────────────────────────

/// A `(query …)` keyword's value, by keyword name.
///
/// The **first** match wins: ein.py returns out of the loop.
pub fn query_value(ast: &Ast, query: &Query, kw_name: &str) -> Option<NodeId> {
    for &pair in query.kw_pairs.iter() {
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
/// ein.py hand-builds a `JoinPlan(rule_name="<query>", …)` around
/// `compile_pattern(goal, {})`. There is no free-standing pattern compiler
/// here — [`compile_rule`](crate::compile_rule) is the entry point — so the
/// goal is wrapped in a parameter-less synthetic rule of the same name, which
/// compiles to the same steps: no activator to bind, no `:assert` templates,
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
            let query = match kb.program().query.as_ref() {
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
        priority: None,
        loc: None,
    };
    let plan = match crate::compile::compile_rule(ast, terms, &rule, None) {
        Ok(plan) => plan,
        // ein.py lets `compile_pattern` raise straight out of `goal_bindings`,
        // so a goal the compiler rejects — `(query :goal (?R Rex Animal))`,
        // an unbound relation head — ends the *whole run* there rather than
        // projecting nothing. Callers that must reproduce that ask
        // [`goal_plan_error`] first; this one keeps its infallible signature.
        Err(_) => return Vec::new(),
    };
    let mut rows: Vec<Vec<(Symbol, Value)>> = Vec::new();
    let mut matcher = crate::match_::Matcher::new();
    matcher.run(kb, terms, ast, &plan, &mut |m| {
        // `dict(b)` — last binding of a repeated name wins, and the row keeps
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

/// The error ein.py's `compile_pattern` would raise for this query goal.
///
/// `None` when there is no goal, or when it compiles. [`goal_bindings`] cannot
/// report it — ein.py raises out of the same call and ein.rs has no exceptions
/// — so the CLI asks this before it renders, and prints the line CPython's
/// traceback would end with. Found by
/// [S1a.6.6](../../../../docs/history/m1a_rust/README.md#s1a66--the-differential-fuzzer)'s
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
        None => query_value(ast, kb.program().query.as_ref()?, "goal")?,
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
        priority: None,
        loc: None,
    };
    crate::compile::compile_rule(ast, terms, &rule, None).err()
}
