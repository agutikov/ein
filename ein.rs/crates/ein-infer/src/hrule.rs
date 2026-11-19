//! Rule-driven hypothesis generation — `(hrule …)`.
//!
//! An hrule is a rule by shape whose *firing yields a candidate hypothesis*
//! instead of a derived fact: the puzzle declares which hypotheses are worth
//! trying rather than leaving it all to the blind combinatorial enumerator.
//!
//! Because hrules live in their own registry, `compile_all` never walks them —
//! **the saturator never sees an hrule**.
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
use ein_core::{FactId, Kb, Symbol, Terms, Value};
use ein_ir::Ast;
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
        let activators = hrule_activators(s.kb, s.terms, s.ast);
        let hrules: Vec<Rule> = s.kb.program().hrules.values().cloned().collect();
        let mut plans = Vec::new();
        for h in &hrules {
            if h.params.is_empty() {
                plans.push(Arc::new(crate::compile::compile_rule(
                    s.ast, s.terms, h, None,
                )?));
                continue;
            }
            for argtuple in binding(h, &activators) {
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

    /// The compiled plans, in the generator's order: one per hrule without
    /// parameters and one per activator of one with them
    /// ([`crate::mechanisms`] reads which hrule and activator each is).
    pub(crate) fn plans(&self) -> &[Arc<Plan>] {
        &self.plans
    }

    /// The relations these plans conclude — what an hrule-driven search can
    /// still propose, for the eligibility
    /// ([`crate::hypgen::eligible_relations`]) and for what the hrules'
    /// universe covers ([`crate::scope`]).
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
    /// No caller writes the KB between two candidates: the one pass,
    /// `hypgen::generate_config_free`, writes nothing. So each hrule is
    /// matched as it is reached, against the state it was asked about, and a
    /// caller that stops early matches no hrule it does not reach.
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
        // computed (Q-M1e2.26, Q-M1f.32).
        let proposes = |s: &Session<'_>, plan: &Plan| -> Option<Slot> {
            let template = plan.assert_template()?;
            (matches!(template, Slot::Nested { .. })
                && crate::firing::computed(s.terms, plan, template).is_none())
            .then_some(template)
        };
        for plan in self.plans.iter() {
            let Some(template) = proposes(s, plan) else {
                continue;
            };
            let envs = matches(s, plan);
            for (regs, trail) in &envs {
                let env = Env {
                    regs,
                    trail,
                    premises: &[],
                };
                match build_fact(s.terms, s.ast, plan, env, template) {
                    Ok(Some(fact)) => f(s, fact)?,
                    // The negation of a name is no fact, and no hypothesis:
                    // `(not ?x)` with `?x` bound to a
                    // name the loader could not foresee.
                    Ok(None) => continue,
                    // `NotAFact` — a malformed template yields no candidate.
                    // No template is a whole conclusion that is no fact: the
                    // loader refuses one a macro's expansion writes
                    // (`macro-bare-conclusion-hrule`). No template reads
                    // a variable nothing binds: the loader refuses one no
                    // premise or parameter binds (`hrule-assert-var`).
                    // Either would propose nothing, in silence. An
                    // `Overflow` is not that,
                    // but it is unreachable here for the same reason it is
                    // elsewhere: the interner holds 2^30 rows.
                    Err(_) => continue,
                }
            }
        }
        ControlFlow::Continue(())
    }
}

/// Activator argument-tuples per hrule name, read from the
/// `(solve … :hrules (NAME (a b) …))` keywords.
///
/// A `:hrules` value is `(NAME (args…) (args…) …)`: its head is the hrule
/// name, and each remaining item is one argument tuple binding that hrule's
/// parameters once. Unlike the relation-set keywords, **every** `:hrules` pair
/// contributes — they accumulate rather than the first one winning.
fn hrule_activators(kb: &Kb, terms: &Terms, ast: &Ast) -> FxHashMap<Symbol, Vec<Vec<Symbol>>> {
    let mut out: FxHashMap<Symbol, Vec<Vec<Symbol>>> = FxHashMap::default();
    let Some(solve) = kb.program().solve() else {
        return out;
    };
    for &pair in solve.kw_pairs.iter() {
        let ein_ir::Node::KwPair { key, value } = ast.node(ein_ir::NodeId(pair.0)) else {
            continue;
        };
        let ein_ir::Node::Keyword(name) = ast.node(key) else {
            continue;
        };
        if ast.sym(name) != "hrules" {
            continue;
        }
        let ein_ir::Node::SForm { head, args } = ast.node(value) else {
            continue;
        };
        let Some(hrule_name) = ast.atom_name(head) else {
            continue;
        };
        let Some(hrule_sym) = terms.syms.get(hrule_name) else {
            continue;
        };
        for &item in ast.args(args) {
            let ein_ir::Node::SForm {
                head: ihead,
                args: iargs,
            } = ast.node(item)
            else {
                continue;
            };
            let mut argtuple = Vec::new();
            for node in std::iter::once(ihead).chain(ast.args(iargs).iter().copied()) {
                if let Some(text) = ast.atom_name(node)
                    && let Some(sym) = terms.syms.get(text)
                {
                    argtuple.push(sym);
                }
            }
            out.entry(hrule_sym).or_default().push(argtuple);
        }
    }
    out
}

/// The `:hrules` activators that bind `h`'s parameters: the tuples written
/// for its name with as many arguments as it has parameters.
///
/// An activator of another count cannot bind the parameters, so every
/// parameter-headed premise would compile with an unbound head var.
/// [`crate::compile::activators_for`] skips these for ordinary rules; the
/// same skip belongs here.
fn binding<'a>(
    h: &Rule,
    activators: &'a FxHashMap<Symbol, Vec<Vec<Symbol>>>,
) -> impl Iterator<Item = &'a Vec<Symbol>> {
    let n = h.params.len();
    activators
        .get(&h.name)
        .map_or(&[][..], |v| &v[..])
        .iter()
        .filter(move |argtuple| argtuple.len() == n)
}

/// **How many `:hrules` activators bind each hrule**, in load order: the
/// plans [`Hrules::new`] compiles of an hrule with parameters, none where no
/// activator of its count is written, and `None` for an hrule without
/// parameters, which needs no activator and is compiled once. What `ein
/// saturate --dump` prints beside each hypothesis rule.
pub fn activator_counts(kb: &Kb, terms: &Terms, ast: &Ast) -> Vec<(Symbol, Option<usize>)> {
    let activators = hrule_activators(kb, terms, ast);
    kb.program()
        .hrules
        .values()
        .map(|h| {
            let n = (!h.params.is_empty()).then(|| binding(h, &activators).count());
            (h.name, n)
        })
        .collect()
}
