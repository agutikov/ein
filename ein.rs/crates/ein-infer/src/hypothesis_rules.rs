//! The rules that conclude a hypothesis — `(open F)`.
//!
//! **A rule that concludes `(open F)`**, `F` a fact or a variable its
//! `:match` binds to one, states a hypothesis (Q-M1f.566, Q-M1f.573,
//! Q-M1f.567): the puzzle says which facts are worth supposing rather than
//! leaving it all to the blind combinatorial enumerator. It is activated as
//! any rule is, by a fact of its name the program writes, or none for a rule
//! without parameters ([`crate::compile::activators_for`]), and the facts it
//! concludes beside its `(open …)` conjuncts are an ordinary rule's, which
//! the saturator fires (`ein_core::Program::hypotheses`).
//!
//! Such a rule lives outside the saturator's registry, so `compile_all` never
//! walks it: **the saturator never fires the `(open …)` a rule concludes**. A
//! fork drops every `open` fact, so a rule that concludes one is read at
//! every state by the pass that writes a state's `open` facts at its fixpoint
//! ([`crate::open`], Q-M1f.569), and at root its hypotheses are $H$'s
//! ([`crate::hypgen`]): each `:match` over the KB yields a binding, and the
//! hypothesis's template, resolved under it, is `F`.

use std::ops::ControlFlow;
use std::sync::Arc;

use ein_core::entities::Rule;
use ein_core::{FactId, Symbol};

use crate::compile::CompileError;
use crate::firing::Env;
use crate::match_::Matcher;
use crate::plan::{Plan, Slot};
use crate::saturator::Session;

/// The compiled plans of the rules that conclude a hypothesis — built once,
/// reused per generation pass and per state.
pub struct HypothesisRules {
    plans: Vec<Arc<Plan>>,
}

impl HypothesisRules {
    /// Compile one plan per activator the state holds of each rule that
    /// concludes `(open F)`, one alone for such a rule with no parameters.
    /// With no matching activator a rule with parameters contributes nothing.
    pub fn new(s: &mut Session<'_>) -> Result<HypothesisRules, CompileError> {
        // Compiled here, never into the memo: a rule that concludes facts
        // beside its hypotheses has its saturation plan there under the same
        // name, and the memo holds what a state's saturation compiled.
        let rules: Vec<Rule> = s.kb.program().hypotheses.values().cloned().collect();
        let mut plans = Vec::new();
        for rule in &rules {
            for act in crate::compile::activators_for(s.kb, s.terms, rule) {
                plans.push(Arc::new(crate::compile::compile_rule(
                    s.ast, s.terms, rule, act,
                )?));
            }
        }
        Ok(HypothesisRules { plans })
    }

    /// No plan: no rule concludes a hypothesis at the state.
    pub(crate) fn is_empty(&self) -> bool {
        self.plans.is_empty()
    }

    /// The compiled plans, in the generator's order: one per activator of
    /// each rule that concludes `(open F)`, in load order
    /// ([`crate::mechanisms`] reads which rule and activator each is, and the
    /// static check what each reads).
    pub fn plans(&self) -> &[Arc<Plan>] {
        &self.plans
    }

    /// The relations these plans' hypotheses are of, where each is written
    /// as a fact — what the rules can still propose, for the eligibility
    /// ([`crate::hypgen::eligible_relations`]) and for what the rules'
    /// universe covers ([`crate::scope`]).
    pub(crate) fn asserted_relations(
        &self,
        terms: &ein_core::Terms,
    ) -> rustc_hash::FxHashSet<ein_core::Symbol> {
        self.plans
            .iter()
            .flat_map(|p| {
                hypotheses_of(p, terms)
                    .into_iter()
                    .filter_map(|x| match x {
                        Slot::Nested { rel, .. } => Some(rel),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Each plan with the relation of each hypothesis it states, in the
    /// generator's order: the relation where `F` is written as a fact, `None`
    /// where it is a variable ([`crate::mechanisms`] reads it).
    pub(crate) fn proposals(
        &self,
        terms: &ein_core::Terms,
    ) -> Vec<(Arc<Plan>, Option<ein_core::Symbol>)> {
        let mut out = Vec::new();
        for p in &self.plans {
            for x in hypotheses_of(p, terms) {
                let rel = match x {
                    Slot::Nested { rel, .. } => Some(rel),
                    _ => None,
                };
                out.push((Arc::clone(p), rel));
            }
        }
        out
    }

    /// One candidate per hypothesis per match against the KB.
    ///
    /// The fact is *not* written — it is a hypothesis, handed back to hypgen
    /// for filtering. A hypothesis whose binding is no fact, a name, yields
    /// no candidate, and neither does an instance of `eq` or `neq`: a
    /// commitment states a fact a state stores, and a state stores neither,
    /// whose facts are computed (Q-M1e2.26, Q-M1f.32).
    ///
    /// ### A match is one whose `absent` premises hold
    ///
    /// The compiler lifts a top-level `(absent …)` out of the match, as it
    /// does a rule's that concludes a fact, so a walk of the plan's positive
    /// steps is not yet a match: each lifted guard is judged here, against
    /// the state, which is a fixpoint, as the saturator judges it at the
    /// boundary, and a match one of whose guards' queries has a match is
    /// none (Q-M1f.685). So a rule that concludes a fact and one that
    /// concludes a hypothesis over one `:match` conclude of the same
    /// matches, and so does one rule that concludes both.
    ///
    /// ### Why the matches are collected before any is emitted
    ///
    /// The matcher borrows the KB immutably for the whole walk and the callback
    /// needs it mutably, so one plan's walk finishes before its first
    /// candidate is emitted.
    ///
    /// No caller writes the KB between two candidates: the one pass,
    /// `hypgen::generate_config_free`, writes nothing. So each plan is
    /// matched as it is reached, against the state it was asked about, and a
    /// caller that stops early matches no plan it does not reach.
    pub fn candidates(
        &self,
        s: &mut Session<'_>,
        f: &mut dyn FnMut(&mut Session<'_>, FactId) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        self.candidates_by(s, &mut |s, _, fact| f(s, fact))
    }

    /// [`Self::candidates`], each with the rule whose match gave it.
    pub(crate) fn candidates_by(
        &self,
        s: &mut Session<'_>,
        f: &mut dyn FnMut(&mut Session<'_>, Symbol, FactId) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        type Envs = Vec<(Vec<ein_core::Value>, Vec<crate::plan::Reg>)>;
        let mut matcher = Matcher::new();
        let mut judge = Matcher::new();
        for plan in &self.plans {
            // A plan's `(open F)` conjuncts: each is a hypothesis, `F` a
            // fact's template or a variable its `:match` binds, which names
            // no hypothesis where it holds a name.
            let templates = hypotheses_of(plan, s.terms);
            if templates.is_empty() {
                continue;
            }
            let mut envs: Envs = Vec::new();
            let (kb, terms, ast) = (&*s.kb, &*s.terms, s.ast);
            matcher.run(kb, terms, ast, plan, &mut |m| {
                // The lifted `(absent …)` premises, judged against the
                // state: the match holds where none of their queries has a
                // match, as a goal's and the boundary's do.
                if m.guards()
                    .iter()
                    .any(|g| judge.holds(kb, terms, ast, plan, g, m.regs()))
                {
                    return ControlFlow::Continue(());
                }
                envs.push((m.regs().to_vec(), m.trail().to_vec()));
                ControlFlow::Continue(())
            });
            for (regs, trail) in &envs {
                let env = Env {
                    regs,
                    trail,
                    premises: &[],
                };
                for &x in &templates {
                    // An `eq` or `neq` instance is no hypothesis: a template
                    // a parameter heads, `(?R ?x ?y)`, is one where an
                    // activator binds the parameter to either.
                    if matches!(x, Slot::Nested { .. })
                        && crate::firing::computed(s.terms, plan, x).is_some()
                    {
                        continue;
                    }
                    if let Ok(Some(value)) = crate::firing::resolve(s.terms, s.ast, plan, env, x)
                        && let Some(fact) = value.as_fact()
                    {
                        f(s, plan.rule, fact)?;
                    }
                }
            }
        }
        ControlFlow::Continue(())
    }
}

/// What each `(open F)` conjunct of `plan`'s `:assert` states: `F`'s slot,
/// a fact's template or a register, in the order written; none for an
/// `(open R)`, which no plan of a rule that concludes a hypothesis holds.
fn hypotheses_of(plan: &Plan, terms: &ein_core::Terms) -> Vec<Slot> {
    plan.asserts
        .iter()
        .filter_map(|&t| match t {
            Slot::Nested { rel, slots } if rel == terms.kernel.open && slots.len() == 1 => {
                Some(plan.slots[slots.start as usize])
            }
            _ => None,
        })
        .filter(|x| matches!(x, Slot::Nested { .. } | Slot::Reg(_)))
        .collect()
}
