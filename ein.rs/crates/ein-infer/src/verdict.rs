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

/// The distinct branches among `branches`, by canonical state — first
/// occurrence kept.
///
/// **One implementation of "same model".** It had three: the table counted
/// `k` this way, `expect` compared claims this way with a doc comment saying
/// *keyed the way `answer.rs` counts `k`*, and the key table's `variables`
/// filtered this way inline. Three copies of a rule is M1e `AR-M1`'s shape,
/// and one of the three deciding differently is how `k` would have parted
/// from `k`.
///
/// The comparison is [`crate::canon::state_key`] — the sorted fact list
/// itself, never a hash of it, so two branches are the same model exactly
/// when they hold the same facts. Quadratic in the number of models, which is
/// a `k`: the corpus's largest is 32.
pub fn distinct<'a, I>(branches: I) -> Vec<&'a Solution>
where
    I: IntoIterator<Item = &'a Solution>,
{
    crate::canon::distinct_by_state(branches, |s| &s.kb)
}

/// What a read-out prints above a verdict: the count, and the parenthetical
/// that qualifies it.
///
/// **M1e `AR-M2`.** `Verdict` was computed once and *read out* three times —
/// `answer.rs`'s table, `ein test`'s header, `--stats` — and each of the three
/// chose its own number and its own qualifier. Two of them chose wrong
/// ([`CO-M2`], [`SE-M1`]) and a third printed the search's counter under this
/// one's label. The fix is not three corrections: it is that a surface is
/// *handed* the count instead of picking one, so adding the next verdict word
/// is a change in this crate. S1d.2.6 and S1d.3.3 each added a word and each
/// missed a site, which is the evidence that the seam was the finding.
///
/// [`CO-M2`]: `plans/m1e_review_processing/p1e.3_medium/s1e.3.1_correctness.md`
/// [`SE-M1`]: `plans/m1e_review_processing/p1e.3_medium/s1e.3.2_semantics.md`
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ReadOut {
    /// The number to print. Always the verdict's own `k` — never
    /// `MonotonicStats::solution_nodes`, which counts what the *search*
    /// recorded and parts from this on every `Open`.
    pub k: usize,
    /// The parenthetical that qualifies `k`, **without** the spaces that
    /// separate the two; `""` when the count needs none. A renderer that
    /// wants a suffix asks [`ReadOut::suffix`] rather than re-deciding the
    /// spacing.
    pub qualifier: &'static str,
}

impl ReadOut {
    /// `""`, or the qualifier with the three spaces that set it off from `k`.
    pub fn suffix(&self) -> String {
        if self.qualifier.is_empty() {
            String::new()
        } else {
            format!("   {}", self.qualifier)
        }
    }
}

/// The four verdicts. `Aborted` is deliberately **not** one of them — see
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
    /// `k = 0` **models**, and the reason is a debt rather than a conflict —
    /// M1d [S1d.2.6].
    ///
    /// [`ideas.md`]'s middle outcome: *нарушений нет, но остаются
    /// обязательства* — the state is consistent, quiescent and complete by the
    /// generator's test, and an obligation it stated is still unwitnessed. The
    /// distinction the other three words cannot draw is between *no model* and
    /// *not yet a model*, and it is the whole of why `Contradiction` was the
    /// wrong word for it.
    ///
    /// **Scoped**: only a program that states an obligation can reach this,
    /// which is [`crate::solve::OwesReport::in_scope`]. A state is judged by
    /// discharge when it has been told what it owes and by exhaustion when it
    /// has not, so every corpus entry that states none reports exactly the
    /// words it reported before P1d.2 — 92 of the 121 that reach a fixpoint,
    /// counted by `utils/openness_census.py` (this comment said 119 until M1e
    /// S1e.2.2; the census is the number's owner).
    ///
    /// `states` and `owes` are parallel and both non-empty; `states` is what
    /// `:expect` reads, because an expectation is an assertion about *facts*
    /// and the facts of an open state are the facts it reached.
    ///
    /// [S1d.2.6]: `docs/history/m1d_satisfiability/README.md#s1d26--verdicts-counters-corpus`
    /// [`ideas.md`]: `docs/history/m1d_satisfiability/ideas.md`
    Open {
        states: Vec<Solution>,
        owes: Vec<crate::obligations::Owes>,
    },
}

impl Verdict {
    /// The name the CLI and the events print.
    pub fn as_str(&self) -> &'static str {
        match self {
            Verdict::Solution(_) => "Solution",
            Verdict::Ambiguity(_) => "Ambiguity",
            Verdict::Contradiction { .. } => "Contradiction",
            Verdict::Open { .. } => "Open",
        }
    }

    /// The **models** this verdict reports, deduplicated by canonical state.
    ///
    /// The one owner of *which branches are distinct models*, and the reason
    /// it is here rather than in a renderer: M1e `AR-M2` found the count being
    /// chosen again at every surface that prints it, with three of the choices
    /// disagreeing. An `Open` verdict reports **no** models — its states are
    /// [`Verdict::Open::states`], and the read-out declining to call them
    /// models is the whole of M1d S1d.2.6.
    pub fn models(&self) -> Vec<&Solution> {
        match self {
            Verdict::Solution(s) => vec![s],
            Verdict::Ambiguity(bs) => distinct(bs),
            Verdict::Contradiction { .. } | Verdict::Open { .. } => Vec::new(),
        }
    }

    /// Every state the verdict carries, model or not, in the search's order.
    ///
    /// The distinction from [`Verdict::models`] is M1d S1d.2.6's, and three
    /// surfaces want *this* one: the `:expect` comparison, `--print-final-*`
    /// and the `verdict` event's fact sets. All three are about a **fact
    /// set**, and an open state has one — *"the facts an open state reached
    /// are the facts it reached"*. Each of the three used to write the
    /// four-arm match by hand, which is three chances to disagree about what
    /// `Open` contributes.
    ///
    /// Not deduplicated: these are the nodes the search recorded, and a
    /// consumer that wants the models asks for the models.
    pub fn states(&self) -> Vec<&Solution> {
        match self {
            Verdict::Solution(s) => vec![s],
            Verdict::Ambiguity(bs) => bs.iter().collect(),
            Verdict::Open { states, .. } => states.iter().collect(),
            Verdict::Contradiction { .. } => Vec::new(),
        }
    }

    /// How many **models** the verdict reports — `Open` reports none.
    ///
    /// Distinct from `MonotonicStats::solution_nodes`, which counts what the
    /// *search* recorded and is unchanged by S1d.2.6: an open state is a node
    /// the lattice found and the read-out declines to call a model, so the two
    /// numbers disagree on exactly the entries this stage is about.
    pub fn k(&self) -> usize {
        self.models().len()
    }

    /// What a read-out prints above this verdict — [`ReadOut`].
    ///
    /// `exhausted` is not a property of the verdict, so it is passed in: the
    /// same four models are *the* models or *models found* depending on
    /// whether the lattice was exhausted, and that is the qualifier's whole
    /// subject.
    pub fn read_out(&self, exhausted: bool) -> ReadOut {
        ReadOut {
            k: self.k(),
            qualifier: if exhausted {
                ""
            } else {
                match self {
                    // A `k = 1` under a cap is *a* model, not *the* model.
                    Verdict::Solution(_) => "(not certified — pass --exhaustive)",
                    // A `k > 1` under a cap is a floor — M1d S1d.3.3.
                    Verdict::Ambiguity(_) => "(a lower bound — the search did not exhaust)",
                    // A `k = 0` under a cap is *none within the cap* — M1d
                    // T1d.10.5.2b. `Open` takes the same words for the same
                    // reason: an unwitnessed obligation under a truncated
                    // search could still be discharged a layer down.
                    Verdict::Contradiction { .. } | Verdict::Open { .. } => {
                        "(none found — the search did not exhaust)"
                    }
                }
            },
        }
    }

    /// The instances an `Open` verdict is owed, summed over its states.
    pub fn owed(&self) -> usize {
        match self {
            Verdict::Open { owes, .. } => owes.iter().map(|o| o.total()).sum(),
            _ => 0,
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

    /// The count a read-out prints, for an answer that may not be a verdict.
    ///
    /// `Aborted` has no verdict and therefore no models — what it can report
    /// is what the search recorded before the budget cut, which is why the
    /// counter is a parameter rather than a field. Three surfaces wrote this
    /// same two-arm match by hand (`--json-summary`, the `verdict` event,
    /// `ein test`'s row); it is one function now, for M1e `AR-M2`'s reason.
    pub fn k(&self, solution_nodes: u64) -> usize {
        match self {
            Answer::Verdict(v) => v.k(),
            Answer::Aborted { .. } => solution_nodes as usize,
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
        priority: None,
        loc: None,
    };
    crate::compile::compile_rule(ast, terms, &rule, None).err()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_core::Program;

    fn empty() -> Solution {
        Solution {
            kb: Kb::new(Program::new()),
            trace: Vec::new(),
        }
    }

    /// The four verdicts, times exhausted and not — the whole qualifier table
    /// in one place, which is what M1e `AR-M2` bought.
    ///
    /// **Two of these eight cells no `.ein` program reaches.** Every `Open` in
    /// the corpus is exhausted (`openness_census.py`; the arm's own comment in
    /// `render_answer` says so), so the truncated `Open` qualifier is here or
    /// it is nowhere — and a qualifier that ships untested is how
    /// `Contradiction`'s stayed missing until T1d.10.5.2b.
    #[test]
    fn every_verdict_qualifies_its_own_count() {
        let table: [(Verdict, &str); 4] = [
            (
                Verdict::Solution(empty()),
                "(not certified — pass --exhaustive)",
            ),
            (
                Verdict::Ambiguity(vec![empty()]),
                "(a lower bound — the search did not exhaust)",
            ),
            (
                Verdict::Contradiction {
                    unsat_core: Vec::new(),
                },
                "(none found — the search did not exhaust)",
            ),
            (
                Verdict::Open {
                    states: vec![empty()],
                    owes: Vec::new(),
                },
                "(none found — the search did not exhaust)",
            ),
        ];
        for (v, want) in table.iter() {
            assert_eq!(
                v.read_out(true).qualifier,
                "",
                "{}: an exhausted count needs no qualifier",
                v.as_str()
            );
            assert_eq!(v.read_out(true).suffix(), "", "{}", v.as_str());
            assert_eq!(v.read_out(false).qualifier, *want, "{}", v.as_str());
            assert_eq!(
                v.read_out(false).suffix(),
                format!("   {want}"),
                "{}: three spaces set the qualifier off from k",
                v.as_str()
            );
        }
    }

    /// `k` is the count of **models**, and `read_out` prints that number and
    /// no other. `Open` is the verdict the two numbers part on.
    #[test]
    fn the_read_outs_count_is_the_verdicts_own() {
        let cases: [(Verdict, usize); 4] = [
            (Verdict::Solution(empty()), 1),
            (Verdict::Ambiguity(vec![empty(), empty()]), 1),
            (
                Verdict::Contradiction {
                    unsat_core: Vec::new(),
                },
                0,
            ),
            (
                Verdict::Open {
                    states: vec![empty()],
                    owes: Vec::new(),
                },
                0,
            ),
        ];
        for (v, k) in cases.iter() {
            assert_eq!(v.k(), *k, "{}", v.as_str());
            assert_eq!(v.models().len(), *k, "{}: k is models().len()", v.as_str());
            assert_eq!(v.read_out(true).k, *k, "{}", v.as_str());
        }
    }

    /// Two branches that reached the same facts are **one** model — the rule
    /// `Verdict::k`, `expect` and the key table now share. The `Ambiguity`
    /// case above is the same claim from the other side: two identical empty
    /// KBs are `k = 1`, not `k = 2`.
    #[test]
    fn distinct_keys_branches_by_their_facts() {
        let bs = vec![empty(), empty(), empty()];
        assert_eq!(distinct(&bs).len(), 1);
        assert_eq!(distinct(std::iter::empty()).len(), 0);
    }

    /// `Aborted` has no verdict, so its count is what the search recorded —
    /// the one arm that reads the counter, in the one place that reads it.
    #[test]
    fn an_aborted_answer_reports_what_the_search_recorded() {
        let a = Answer::Aborted {
            reason: "budget".to_string(),
        };
        assert_eq!(a.k(7), 7);
        assert_eq!(Answer::Verdict(Verdict::Solution(empty())).k(7), 1);
    }
}
