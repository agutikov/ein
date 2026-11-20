//! The rules that conclude a hypothesis — `(open F)` — and `(hrule …)`.
//!
//! **A rule that concludes `(open F)`**, `F` a fact or a variable its
//! `:match` binds to one, states a hypothesis (Q-M1f.566, Q-M1f.573): the
//! puzzle says which facts are worth supposing rather than leaving it all to
//! the blind combinatorial enumerator. It is activated as any rule is, by a
//! fact of its name ([`crate::compile::activators_for`]), and the facts it
//! concludes beside its `(open …)` conjuncts are an ordinary rule's, which
//! the saturator fires (`ein_core::Program::hypotheses`).
//!
//! **An hrule is read as such a rule**: `(hrule guess (?R ?T) :match M
//! :assert F)` is the rule that concludes `(open F)`, activated by the
//! activators its `(solve … :hrules (guess (a b) …))` clause writes, which
//! are not facts of the state. A parameter-less hrule needs none.
//!
//! Both live outside the saturator's registry, so `compile_all` never walks
//! them: **the saturator never fires the `(open …)` a rule concludes**. A fork
//! drops every `open` fact, so a rule that concludes one is read at every
//! state by the pass that writes a state's `open` facts at its fixpoint
//! ([`crate::open`], Q-M1f.569), and at root its hypotheses are $H$'s
//! ([`crate::hypgen`]): each `:match` over the KB yields a binding, and
//! [`crate::firing::build_fact`] over the hypothesis's template is `F`.

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

/// The compiled plans of the rules that conclude a hypothesis — built once,
/// reused per generation pass and per state.
pub struct Hrules {
    plans: Vec<Arc<Plan>>,
    /// Per plan: whether it is a rule of `ein_core::Program::hypotheses`,
    /// whose hypotheses are its `(open F)` templates, rather than an hrule,
    /// whose `:assert` is `F`.
    opened: Vec<bool>,
}

impl Hrules {
    /// Compile one plan per parameter-less hrule, and one per `:hrules`
    /// activator of a generic one; then one per activator the state holds of
    /// each rule that concludes `(open F)`, one alone for such a rule with no
    /// parameters. With no matching activator a generic rule contributes
    /// nothing.
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
        let mut opened = vec![false; plans.len()];
        // Compiled here, never into the memo: a rule that concludes facts
        // beside its hypotheses has its saturation plan there under the same
        // name, and the memo holds what a state's saturation compiled.
        let rules: Vec<Rule> = s.kb.program().hypotheses.values().cloned().collect();
        for rule in &rules {
            crate::compile::note_inert_activators(s.ast, s.terms, s.kb, rule);
            for act in crate::compile::activators_for(s.kb, s.terms, rule) {
                plans.push(Arc::new(crate::compile::compile_rule(
                    s.ast, s.terms, rule, act,
                )?));
                opened.push(true);
            }
        }
        Ok(Hrules { plans, opened })
    }

    /// No plan: no rule concludes a hypothesis at the state.
    pub(crate) fn is_empty(&self) -> bool {
        self.plans.is_empty()
    }

    /// The compiled plans, in the generator's order: one per hrule without
    /// parameters and one per activator of one with them, then the rules
    /// that conclude `(open F)` ([`crate::mechanisms`] reads which rule and
    /// activator each is, and the static check what each reads).
    pub fn plans(&self) -> &[Arc<Plan>] {
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
            .zip(&self.opened)
            .flat_map(|(p, &opened)| {
                if opened {
                    hypotheses_of(p, terms)
                        .into_iter()
                        .filter_map(|x| match x {
                            Slot::Nested { rel, .. } => Some(rel),
                            _ => None,
                        })
                        .collect()
                } else {
                    crate::compile::asserted_relation(p, terms)
                        .into_iter()
                        .collect::<Vec<_>>()
                }
            })
            .collect()
    }

    /// Each plan with the relation of each hypothesis it states, in the
    /// generator's order: an hrule's `:assert`'s, a rule's `(open F)`
    /// conjuncts' where `F` is written as a fact, `None` where it is a
    /// variable or no fact's template ([`crate::mechanisms`] reads it).
    pub(crate) fn proposals(
        &self,
        terms: &ein_core::Terms,
    ) -> Vec<(Arc<Plan>, Option<ein_core::Symbol>)> {
        let mut out = Vec::new();
        for (p, &opened) in self.plans.iter().zip(&self.opened) {
            if opened {
                for x in hypotheses_of(p, terms) {
                    let rel = match x {
                        Slot::Nested { rel, .. } => Some(rel),
                        _ => None,
                    };
                    out.push((Arc::clone(p), rel));
                }
            } else {
                out.push((Arc::clone(p), crate::compile::asserted_relation(p, terms)));
            }
        }
        out
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
        self.candidates_by(s, &mut |s, _, fact| f(s, fact))
    }

    /// [`Self::candidates`], each with the rule whose match gave it.
    pub(crate) fn candidates_by(
        &self,
        s: &mut Session<'_>,
        f: &mut dyn FnMut(&mut Session<'_>, Symbol, FactId) -> ControlFlow<()>,
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
        for (plan, &opened) in self.plans.iter().zip(&self.opened) {
            if opened {
                // A rule's `(open F)` conjuncts: each is a hypothesis, `F` a
                // fact's template or a variable its `:match` binds, which
                // names no hypothesis where it holds a name.
                let templates = hypotheses_of(plan, s.terms);
                if templates.is_empty() {
                    continue;
                }
                let envs = matches(s, plan);
                for (regs, trail) in &envs {
                    let env = Env {
                        regs,
                        trail,
                        premises: &[],
                    };
                    for &x in &templates {
                        if let Ok(Some(value)) =
                            crate::firing::resolve(s.terms, s.ast, plan, env, x)
                            && let Some(fact) = value.as_fact()
                        {
                            f(s, plan.rule, fact)?;
                        }
                    }
                }
                continue;
            }
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
                    Ok(Some(fact)) => f(s, plan.rule, fact)?,
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
