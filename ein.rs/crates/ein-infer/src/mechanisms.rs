//! The hypothesis mechanisms a program declares, read at a state — M1f
//! [S1f.6.34](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.34_hypothesis_mechanisms_in_conflict.md).
//!
//! Five declarations steer what a commitment may be about, and a sixth
//! mechanism, the blind enumerator, is what runs where none of the generating
//! ones does ([`crate::hypgen`]'s ladder):
//!
//! | | mechanism | read here as |
//! |---|---|---|
//! | H | `(hrule …)`, activated by `(solve :hrules …)` | [`Mechanisms::proposals`]: each active hrule plan and the relation its `:assert` proposes |
//! | O | an obligation rule, `:assert (open ?R)` | [`Mechanisms::obligations`]: each obligation plan the state's activators compile, and the relation it owes |
//! | HR | `(solve :hypothesis-relations …)` | [`Mechanisms::listed`] |
//! | NH | `(solve :no-hypothesis …)` | [`Mechanisms::excluded`] |
//! | C | `(__closed__ R)`, stated or derived | [`Mechanisms::closed`] |
//!
//! An hrule is **active** where the generator would compile a plan of it: one
//! without parameters always, one with parameters once per `:hrules`
//! activator of its arity ([`crate::hrule::Hrules::new`]). An obligation is
//! one whose activator the state holds ([`crate::obligations`]'s plans), so a
//! *derived* activator, `bijective-setup`'s `(total color-loc is-a)`, is read
//! only at a fixpoint, as a derived closure is: the reader is meant for
//! root's, before any search.
//!
//! It reads; it decides nothing and writes nothing but the plans it compiles
//! into the session's memo, which a later reader of the same memo would
//! compile alike.

use ein_core::{Symbol, Value};
use rustc_hash::FxHashSet;

use crate::compile::{CompileError, asserted_relation};
use crate::saturator::Session;

/// One plan of a generating mechanism: the rule it is compiled from, the
/// activator's arguments (empty for a rule without parameters), and the
/// relation it is about — the one an hrule's `:assert` proposes a fact of,
/// or the one an obligation's `(open ?R)` owes. `None` where the plan names
/// none: a bare `(open)`, or an hrule whose `:assert` is no fact's template.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Plan {
    pub rule: Symbol,
    pub activator: Box<[Value]>,
    pub relation: Option<Symbol>,
}

/// What a program declares, at one state.
#[derive(Clone, Default, Debug)]
pub struct Mechanisms {
    /// Every `(hrule …)` the program declares, in load order, active or not.
    /// Non-empty is what takes the ladder's first rung.
    pub hrules: Vec<Symbol>,
    /// The active hrule plans, in the generator's order.
    pub proposals: Vec<Plan>,
    /// Every obligation rule the program declares, in load order.
    pub obligation_rules: Vec<Symbol>,
    /// The obligation plans this state's activators compile, in report order.
    pub obligations: Vec<Plan>,
    /// `:hypothesis-relations`, as the generator reads it: `None` where the
    /// program writes none.
    pub listed: Option<FxHashSet<Symbol>>,
    /// `:no-hypothesis`, as the generator reads it.
    pub excluded: FxHashSet<Symbol>,
    /// Every `R` with `(__closed__ R)` at this state.
    pub closed: FxHashSet<Symbol>,
}

impl Mechanisms {
    /// The relations the active hrules propose facts of.
    pub fn proposed(&self) -> FxHashSet<Symbol> {
        self.proposals.iter().filter_map(|p| p.relation).collect()
    }

    /// The relations an obligation of this state can owe.
    pub fn owable(&self) -> FxHashSet<Symbol> {
        self.obligations.iter().filter_map(|p| p.relation).collect()
    }
}

/// Read what the program declares at `s`'s state: written declarations as
/// they were loaded, and closures and activators as the state holds them.
pub fn read(s: &mut Session<'_>) -> Result<Mechanisms, CompileError> {
    let program = s.kb.program_arc();
    let hrules: Vec<Symbol> = program.hrules.iter().map(|(n, _)| n).collect();
    let proposals = if hrules.is_empty() {
        Vec::new()
    } else {
        crate::hrule::Hrules::new(s)?
            .plans()
            .iter()
            .map(|p| Plan {
                rule: p.rule,
                activator: p.activator_args.clone(),
                relation: asserted_relation(p, s.terms),
            })
            .collect()
    };
    let obligation_rules: Vec<Symbol> = program.obligations.iter().map(|(n, _)| n).collect();
    let obligations = if obligation_rules.is_empty() {
        Vec::new()
    } else {
        let memo = s.memo.clone();
        crate::obligations::plans_for(s.kb, s.terms, s.ast, &memo)?
            .iter()
            .map(|p| Plan {
                rule: p.rule,
                activator: p.activator_args.clone(),
                relation: crate::obligations::open_argument(p, s.terms),
            })
            .collect()
    };
    let (listed, excluded, closed) = crate::hypgen::scoping(s);
    Ok(Mechanisms {
        hrules,
        proposals,
        obligation_rules,
        obligations,
        listed,
        excluded,
        closed,
    })
}
