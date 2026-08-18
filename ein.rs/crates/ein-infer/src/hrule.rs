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
//! from the `(query … :hrules (NAME (a b) …))` keywords rather than from
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
                // The activator is a synthetic `Fact(h.name, argtuple)` that
                // is deliberately **not** written to the KB — ein.py builds
                // the same throwaway.
                let act = s
                    .terms
                    .intern_fact(h.name, &args)
                    .expect("room for a synthetic hrule activator");
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

    /// One candidate per hrule match against the KB.
    ///
    /// The fact is *not* written — it is a hypothesis, handed back to hypgen
    /// for filtering. A malformed `:assert` template yields no candidate
    /// rather than failing the generation, which is ein.py's
    /// `except (KeyError, TypeError): continue`.
    ///
    /// ### Why the matches are collected before any is emitted
    ///
    /// ein.py runs this as a generator and hypgen's filter pipeline mutates
    /// the KB — the lookahead kill cache writes `(not h)` — *while* the
    /// matcher is suspended mid-enumeration. ein.py's `_candidates` returns
    /// the live `_facts_by_relation` list, so such a write is visible to the
    /// rest of that enumeration. Here the matcher borrows the KB immutably
    /// for the whole walk and the callback needs it mutably, so the walk
    /// finishes first.
    ///
    /// The two agree exactly whenever the only relation the pipeline writes —
    /// `not` — is not one an hrule's `:match` reads, which is the condition
    /// [`reads_negation`] checks under `debug_assert`. No corpus hrule reads
    /// it (they read `is-a` and the puzzle's own relations), and an hrule that
    /// did would be asking the enumerator to condition on its own kill cache.
    pub fn candidates(
        &self,
        s: &mut Session<'_>,
        f: &mut dyn FnMut(&mut Session<'_>, FactId) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let mut matcher = Matcher::new();
        for plan in &self.plans {
            let Some(template) = plan.assert_template() else {
                continue;
            };
            if !matches!(template, Slot::Nested { .. }) {
                continue;
            }
            debug_assert!(
                !reads_negation(plan, s.terms),
                "an hrule reads `not`, which hypgen's kill cache writes \
                 mid-enumeration — see `Hrules::candidates`"
            );
            let mut envs: Vec<(Vec<Value>, Vec<crate::plan::Reg>)> = Vec::new();
            matcher.run(s.kb, s.terms, s.ast, plan, &mut |m| {
                envs.push((m.regs().to_vec(), m.trail().to_vec()));
                ControlFlow::Continue(())
            });
            for (regs, trail) in &envs {
                let env = Env {
                    regs,
                    trail,
                    premises: &[],
                };
                match build_fact(s.terms, plan, env, template) {
                    Ok(fact) => f(s, fact)?,
                    // `KeyError` / `TypeError` — a malformed template yields
                    // no candidate. An `Overflow` is not that, but it is
                    // unreachable here for the same reason it is elsewhere:
                    // the interner holds 2^30 rows.
                    Err(_) => continue,
                }
            }
        }
        ControlFlow::Continue(())
    }
}

/// True iff any positive premise or guard of `plan` reads the `not` relation.
///
/// The precondition [`Hrules::candidates`] documents, checked rather than
/// argued.
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
/// `(query … :hrules (NAME (a b) …))` keywords.
///
/// A `:hrules` value is `(NAME (args…) (args…) …)`: its head is the hrule
/// name, and each remaining item is one argument tuple binding that hrule's
/// parameters once. Unlike the relation-set keywords, **every** `:hrules` pair
/// contributes — ein.py accumulates rather than returning at the first.
fn hrule_activators(s: &Session<'_>) -> FxHashMap<Symbol, Vec<Vec<Symbol>>> {
    let mut out: FxHashMap<Symbol, Vec<Vec<Symbol>>> = FxHashMap::default();
    let Some(query) = s.kb.program().query.as_ref() else {
        return out;
    };
    for &pair in query.kw_pairs.iter() {
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
