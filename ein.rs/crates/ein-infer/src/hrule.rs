//! Rule-driven hypothesis generation — `(hrule …)`.
//!
//! An hrule is a rule by shape whose *firing yields a candidate hypothesis*
//! instead of a derived fact: the puzzle declares which hypotheses are worth
//! trying rather than leaving it all to the blind combinatorial enumerator.
//!
//! Because hrules live in their own registry, `compile_all` never walks them —
//! **the saturator and the one-step lookahead never see an hrule**.
//! [`crate::hypgen`] is the sole consumer: each `:match` over the KB yields a
//! binding, and [`crate::firing::build_fact`] over the `:assert` template
//! becomes a candidate routed through the ordinary filter pipeline.
//!
//! A generic hrule — `(hrule guess (?rel ?type) …)` — takes its activators
//! from the `(solve … :hrules (NAME (a b) …))` keywords rather than from
//! puzzle state, because an hrule activator *steers the search*. A
//! parameter-less hrule needs none.

use std::ops::ControlFlow;
use std::sync::Arc;

use ein_core::entities::Rule;
use ein_core::{FactId, Symbol, Value};
use rustc_hash::FxHashMap;

use crate::compile::CompileError;
use crate::firing::{Env, build_fact};
use crate::match_::Matcher;
use crate::plan::{Plan, Slot};
use crate::saturator::Session;

/// The compiled hrule plans — built once, reused per generation pass.
pub struct Hrules {
    plans: Vec<Arc<Plan>>,
}

impl Hrules {
    /// Compile one plan per parameter-less hrule, and one per `:hrules`
    /// activator of a generic one. With no matching activator a generic hrule
    /// contributes nothing.
    pub fn new(s: &mut Session<'_>) -> Result<Hrules, CompileError> {
        let activators = hrule_activators(s);
        let hrules: Vec<Rule> = s.kb.program().hrules.values().cloned().collect();
        let mut plans = Vec::new();
        for h in &hrules {
            if h.params.is_empty() {
                plans.push(Arc::new(crate::compile::compile_rule(
                    s.ast, s.terms, h, None,
                )?));
                continue;
            }
            for argtuple in activators.get(&h.name).map_or(&[][..], |v| &v[..]) {
                // S1.22.0 — an arity-mismatched activator cannot bind the
                // parameters, so every parameter-headed premise would compile
                // with an unbound head var. `Engine::activators_for` skips
                // these for ordinary rules; the same skip belongs here.
                if argtuple.len() != h.params.len() {
                    continue;
                }
                let args: Vec<Value> = argtuple.iter().map(|&a| Value::sym(a)).collect();
                // The activator is a synthetic `(h.name argtuple…)` fact that
                // is deliberately **not** written to the KB — a throwaway.
                // A lent table cannot number a synthetic activator it has
                // not seen; `Terms::refused` sends the entering back.
                let Ok(act) = s.terms.intern_fact(h.name, &args) else {
                    continue;
                };
                plans.push(Arc::new(crate::compile::compile_rule(
                    s.ast,
                    s.terms,
                    h,
                    Some(act),
                )?));
            }
        }
        Ok(Hrules { plans })
    }

    /// The relations these plans conclude — what an hrule-driven search can
    /// still propose, for M1e S1e.2.3's eligibility
    /// ([`crate::hypgen::eligible_relations`]) and, since M1e2 S1e2.2.8, what
    /// the hrules' universe covers ([`crate::scope`]).
    pub(crate) fn asserted_relations(
        &self,
        terms: &ein_core::Terms,
    ) -> rustc_hash::FxHashSet<ein_core::Symbol> {
        self.plans
            .iter()
            .filter_map(|p| crate::compile::asserted_relation(p, terms))
            .collect()
    }

    /// One candidate per hrule match against the KB.
    ///
    /// The fact is *not* written — it is a hypothesis, handed back to hypgen
    /// for filtering. A malformed `:assert` template yields no candidate
    /// rather than failing the generation.
    ///
    /// ### Why the matches are collected before any is emitted
    ///
    /// The matcher borrows the KB immutably for the whole walk and the callback
    /// needs it mutably, so one hrule's walk finishes before its first
    /// candidate is emitted.
    ///
    /// No shipped caller writes the KB between two candidates: every command
    /// takes H config-free (`hypgen::generate_config_free`), with the
    /// lookahead's kill cache off, since M1e2 S1e2.3.1. The config-reading
    /// `hypgen::generate`, which the shape digest's `hyp` op and the tests
    /// call, can have it on, and it then writes `(not h)` between one
    /// candidate and the next. So where an hrule's `:match` reads `not`
    /// (`reads_negation`: a stored negation, `(absent (not …))`, the
    /// stdlib's `unknown`), **every** hrule is matched before any candidate is
    /// emitted, and no write the pipeline makes is visible to the
    /// enumeration: an hrule that reads `not` reads the state it was asked
    /// about, with the cache on and off (M1f S1f.7.2, W85). Where none reads
    /// it the two orders agree, and each hrule is matched as it is reached, so
    /// a caller that stops early matches no hrule it does not reach.
    ///
    /// Until S1f.7.2 a `debug_assert!` held that no hrule read `not`, so a
    /// debug build stopped, exit 101, on a program a release build answered,
    /// and answered as `ein oracle` does.
    pub fn candidates(
        &self,
        s: &mut Session<'_>,
        f: &mut dyn FnMut(&mut Session<'_>, FactId) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        type Envs = Vec<(Vec<Value>, Vec<crate::plan::Reg>)>;
        let mut matcher = Matcher::new();
        let mut matches = |s: &Session<'_>, plan: &Plan| -> Envs {
            let mut envs: Envs = Vec::new();
            matcher.run(s.kb, s.terms, s.ast, plan, &mut |m| {
                envs.push((m.regs().to_vec(), m.trail().to_vec()));
                ControlFlow::Continue(())
            });
            envs
        };
        // A plan that proposes: its `:assert` is a fact's template, and not an
        // `eq` / `neq` instance, which is no hypothesis: a commitment states a
        // fact a state stores, and a state stores neither, whose facts are
        // computed (M1f S1f.5.7, Q-M1e2.26; Q-M1f.32, taken by default).
        let proposes = |s: &Session<'_>, plan: &Plan| -> Option<Slot> {
            let template = plan.assert_template()?;
            (matches!(template, Slot::Nested { .. })
                && crate::firing::computed(s.terms, plan, template).is_none())
            .then_some(template)
        };
        let mut ahead: Option<Vec<Envs>> = self
            .plans
            .iter()
            .any(|p| reads_negation(p, s.terms))
            .then(|| {
                self.plans
                    .iter()
                    .map(|plan| match proposes(s, plan) {
                        Some(_) => matches(s, plan),
                        None => Vec::new(),
                    })
                    .collect()
            });
        for (i, plan) in self.plans.iter().enumerate() {
            let Some(template) = proposes(s, plan) else {
                continue;
            };
            let envs = match &mut ahead {
                Some(all) => std::mem::take(&mut all[i]),
                None => matches(s, plan),
            };
            for (regs, trail) in &envs {
                let env = Env {
                    regs,
                    trail,
                    premises: &[],
                };
                match build_fact(s.terms, s.ast, plan, env, template) {
                    Ok(Some(fact)) => f(s, fact)?,
                    // The negation of a name is no fact, and no hypothesis
                    // (M1f S1f.7.4, W84): `(not ?x)` with `?x` bound to a
                    // name the loader could not foresee.
                    Ok(None) => continue,
                    // `NotAFact` — a malformed template yields no candidate.
                    // No template is a whole conclusion that is no fact since
                    // M1f S1f.5.24, which refuses one a macro's expansion
                    // writes at load; until then it proposed nothing, in
                    // silence (`W69/macro-bare-conclusion-hrule`).
                    // No template reads a variable nothing binds since M1f
                    // S1f.5.10: the loader refuses one no premise or
                    // parameter binds, and S1f.5.18 retired the error it
                    // stopped with, `FireError::UnboundVar`. Until S1f.5.10 a
                    // template that wrote one proposed nothing, in silence
                    // (`W57/hrule-assert-var`). An `Overflow` is not that,
                    // but it is unreachable here for the same reason it is
                    // elsewhere: the interner holds 2^30 rows.
                    Err(_) => continue,
                }
            }
        }
        ControlFlow::Continue(())
    }
}

/// True iff any premise or guard of `plan` reads the `not` relation, under an
/// `(absent …)` too: what [`Hrules::candidates`] matches ahead for.
fn reads_negation(plan: &Plan, terms: &ein_core::Terms) -> bool {
    fn walk(plan: &Plan, terms: &ein_core::Terms, span: crate::plan::Span) -> bool {
        plan.steps(span).iter().any(|step| match step {
            crate::plan::Step::Rel(r) => r.rel == terms.kernel.not,
            crate::plan::Step::Absent { sub } => walk(plan, terms, *sub),
            crate::plan::Step::Guard { .. } => false,
        })
    }
    plan.disjuncts.iter().any(|d| {
        walk(plan, terms, d.steps)
            || plan
                .guards(d.guards)
                .iter()
                .any(|g| walk(plan, terms, g.sub))
    })
}

/// Activator argument-tuples per hrule name, read from the
/// `(solve … :hrules (NAME (a b) …))` keywords — on `(query …)` until M1e2
/// S1e2.1a.4 moved the lattice's keywords to `(solve …)`.
///
/// A `:hrules` value is `(NAME (args…) (args…) …)`: its head is the hrule
/// name, and each remaining item is one argument tuple binding that hrule's
/// parameters once. Unlike the relation-set keywords, **every** `:hrules` pair
/// contributes — they accumulate rather than the first one winning.
fn hrule_activators(s: &Session<'_>) -> FxHashMap<Symbol, Vec<Vec<Symbol>>> {
    let mut out: FxHashMap<Symbol, Vec<Vec<Symbol>>> = FxHashMap::default();
    let Some(solve) = s.kb.program().solve() else {
        return out;
    };
    for &pair in solve.kw_pairs.iter() {
        let ein_ir::Node::KwPair { key, value } = s.ast.node(ein_ir::NodeId(pair.0)) else {
            continue;
        };
        let ein_ir::Node::Keyword(name) = s.ast.node(key) else {
            continue;
        };
        if s.ast.sym(name) != "hrules" {
            continue;
        }
        let ein_ir::Node::SForm { head, args } = s.ast.node(value) else {
            continue;
        };
        let Some(hrule_name) = s.ast.atom_name(head) else {
            continue;
        };
        let Some(hrule_sym) = s.terms.syms.get(hrule_name) else {
            continue;
        };
        for &item in s.ast.args(args) {
            let ein_ir::Node::SForm {
                head: ihead,
                args: iargs,
            } = s.ast.node(item)
            else {
                continue;
            };
            let mut argtuple = Vec::new();
            for node in std::iter::once(ihead).chain(s.ast.args(iargs).iter().copied()) {
                if let Some(text) = s.ast.atom_name(node)
                    && let Some(sym) = s.terms.syms.get(text)
                {
                    argtuple.push(sym);
                }
            }
            out.entry(hrule_sym).or_default().push(argtuple);
        }
    }
    out
}
