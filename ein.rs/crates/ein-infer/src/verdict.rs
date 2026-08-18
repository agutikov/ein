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

use ein_core::{FactId, Kb};

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
    out.sort_unstable();
    out.dedup();
    out
}
