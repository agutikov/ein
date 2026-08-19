//! One-step contradiction lookahead — the costliest hypgen filter.
//!
//! Before a candidate `h` is emitted (and later forked and saturated), ask the
//! cheap question: *does adding `h` to the already-saturated KB produce a
//! contradiction in a single rule firing?* If yes, drop it — it would only
//! fork, saturate and die.
//!
//! The mechanism is the matcher's own semi-naive seed. `h` is injected into
//! each `:match` premise whose relation it unifies with, the rule's *other*
//! premises run against the saturated KB, and every resulting `:assert` fact
//! is checked against the detector's shapes. No fork, no saturation, and the
//! only write is the kill cache one frame out ([`crate::hypgen`]).
//!
//! ### The contract, and why it is one-sided
//!
//! The filter may only ever **under**-approximate death: a missed kill just
//! forks and dies normally, but a hypothesis wrongly reported dead is silently
//! lost. Two guards keep it on the safe side and both are semantics, not
//! optimisations:
//!
//! - a disjunct whose guards cannot be judged in the world *with* `h` is
//!   skipped rather than assumed to pass ([`unjudgeable`]);
//! - a guard that can be judged is judged against `kb` **and** against the
//!   one fact `h` adds ([`Lookahead::guards_pass_with`]) — the probe
//!   hypothesises `h` into a positive premise but the KB it runs against does
//!   not contain it, so asking `kb` alone would answer about a different
//!   world, and a rule whose guard watches the candidate's own relation would
//!   kill a live hypothesis.
//!
//! ### Its sign
//!
//! `enable_pre_branch_lookahead` measures *slightly negative* today (0.9× on
//! exhaustive `zebra2`): it pays a simulation to avoid forks that S1.9.E23's
//! fail-fast already made cheap. Because it runs the matcher it inherits
//! [design/05](../../../../plans/m1a_rust/design/05_matcher.md)'s speedup
//! wholesale, which may flip that sign — a
//! [P1a.6](../../../../plans/m1a_rust/p1a.6_performance/README.md)
//! measurement, not a decision to take here. The default does not move
//! without it.

use std::ops::ControlFlow;
use std::sync::Arc;

use ein_core::{FactId, Kb, Terms, Value};

use crate::compile::CompileError;
use crate::firing::{Env, build_fact};
use crate::match_::Matcher;
use crate::plan::{NafGuard, Plan, Span, Step};
use crate::saturator::Session;

/// A one-step contradiction simulator over a fixed rule set.
///
/// Compile the plans once, reuse [`Lookahead::dies_immediately`] across every
/// candidate. The plans bake in activator-bound relation names, so one built
/// at the root stays valid for every fork — forks share the rule set.
pub struct Lookahead {
    /// `compile_all` walks `kb.rules` only, so an `(hrule …)` is never in
    /// here and no filtering is needed: the lookahead cannot simulate a
    /// speculation.
    plans: Vec<Arc<Plan>>,
}

impl Lookahead {
    pub fn new(s: &mut Session<'_>) -> Result<Lookahead, CompileError> {
        let mut engine = crate::engine::Engine::with_memo(s.memo.clone());
        engine.compile_all(s.ast, s.terms, s.kb, s.events)?;
        let plans = (0..engine.len()).map(|i| engine.plan_arc(i)).collect();
        Ok(Lookahead { plans })
    }

    /// True iff adding `h` to `kb` yields a one-step contradiction.
    ///
    /// `kb` is expected to be saturated — the engine only calls this on a
    /// post-saturation KB. Read-only: no fork, and no mutation of the KB.
    pub fn dies_immediately(&self, s: &mut Session<'_>, m: &mut Matcher, h: FactId) -> bool {
        ein_core::counters::bump(|c| c.lookahead_probe += 1);
        let rel = s.terms.facts.rel(h);
        let mut envs: Vec<(Vec<Value>, Vec<crate::plan::Reg>)> = Vec::new();
        for plan in &self.plans {
            // A rule may conclude several facts (S1.8.A13) and any one could
            // contradict `h`, so every fact-shaped conclusion is probed.
            let templates: Vec<crate::plan::Slot> = plan
                .asserts
                .iter()
                .copied()
                .filter(|t| matches!(t, crate::plan::Slot::Nested { .. }))
                .collect();
            if templates.is_empty() {
                continue;
            }
            for (i, d) in plan.disjuncts.iter().enumerate() {
                if unjudgeable(plan, *d) {
                    // S1.21.8 (D3) — a guard we cannot evaluate in the world
                    // *with* `h` must not be assumed to pass. Skipping the
                    // disjunct only loses a kill, which is the safe direction.
                    continue;
                }
                let guards = plan.guards(d.guards);
                for (at, step) in plan.steps(d.steps).iter().enumerate() {
                    let Step::Rel(r) = step else { continue };
                    if r.rel != rel {
                        continue;
                    }
                    // The matcher borrows `terms` immutably and `build_fact`
                    // interns, so one seeding's environments are collected and
                    // its conclusions built after the walk. Sound here for a
                    // stronger reason than in `Hrules::candidates`: this walk
                    // writes nothing at all, to the KB or anywhere else.
                    envs.clear();
                    m.run_seeded_at(s.kb, s.terms, s.ast, plan, i, at, h, &mut |mt| {
                        envs.push((mt.regs().to_vec(), mt.trail().to_vec()));
                        ControlFlow::Continue(())
                    });
                    for (regs, trail) in &envs {
                        if !self.guards_pass_with(s, m, plan, guards, regs, h) {
                            continue;
                        }
                        for &template in &templates {
                            let env = Env {
                                regs,
                                trail,
                                premises: &[],
                            };
                            // Defensive, as in ein.py: a malformed assert
                            // template never kills a candidate.
                            let Ok(f) = build_fact(s.terms, plan, env, template) else {
                                continue;
                            };
                            if is_contradiction(s.kb, s.terms, f, h) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    /// Do `guards` still pass in the world `kb` **plus** `h`?
    ///
    /// Two checks, no mutation: the guard must find no match in `kb`, and `h`
    /// must not create one. [`unjudgeable`] has already excluded the
    /// non-monotone shapes, so for what remains those two together are exactly
    /// "no match in `kb` with `h` added".
    fn guards_pass_with(
        &self,
        s: &Session<'_>,
        m: &mut Matcher,
        plan: &Plan,
        guards: &[NafGuard],
        regs: &[Value],
        h: FactId,
    ) -> bool {
        for g in guards {
            if m.holds(s.kb, s.terms, s.ast, plan, g, regs) {
                return false;
            }
            if m.holds_seeded(s.kb, s.terms, s.ast, plan, g, regs, h) {
                return false;
            }
        }
        true
    }
}

/// True iff some guard's verdict in `kb` plus `h` cannot be decided cheaply.
///
/// A guard containing a *nested* absent — what a `forall` desugars to — is
/// non-monotone in the KB: adding `h` can make the inner absent fail and so
/// make the outer **pass**. Deciding that needs the real world, which the
/// lookahead deliberately does not build.
fn unjudgeable(plan: &Plan, d: crate::plan::Disjunct) -> bool {
    plan.guards(d.guards)
        .iter()
        .any(|g| has_nested_absent(plan, g.sub))
}

fn has_nested_absent(plan: &Plan, sub: Span) -> bool {
    plan.steps(sub)
        .iter()
        .any(|st| matches!(st, Step::Absent { .. }))
}

/// True iff a KB holding `h` **and** the derived `f` is contradictory.
///
/// Mirrors [`crate::contradiction`]'s two shapes — direct ⊥ and the `(X, ¬X)`
/// pair, which since S1.22.1b is a contradiction regardless of how either side
/// entered the KB. Its semantics are the *hypothetical* KB's, which is why it
/// is not `contradicts`: `h` is not in `kb`.
fn is_contradiction(kb: &Kb, terms: &Terms, f: FactId, h: FactId) -> bool {
    let (rel, args) = terms.facts.get(f);
    // Direct ⊥ — a `(false …)` fact.
    if rel == terms.kernel.r#false {
        return true;
    }
    if rel == terms.kernel.not {
        // `f` is a negative `(not g)`.
        let Some(g) = args.first().and_then(|a| a.as_fact()) else {
            return false;
        };
        // `(not h)` against `h` — both hypothetical, a guaranteed pair.
        if g == h {
            return true;
        }
        // `(not g)` against an existing positive `g` — a pair, whatever `g`'s
        // origin.
        return kb.contains(g);
    }
    // `f` is positive — `h` would derive it. A contradiction iff `(not f)` is
    // already in the KB.
    kb.is_negated(f)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile::SharedMemo;
    use ein_core::Kb;
    use ein_ir::{Ast, from_ir::load, parse};

    /// Saturate `src`, then ask whether `(rel arg)` dies immediately.
    fn dies(src: &str, rel: &str, arg: &str) -> bool {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let mut kb: Kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let mut ev = crate::events::Events::off();
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut ev,
            memo: SharedMemo::default(),
        };
        let mut sat = crate::saturator::Saturator::new(&mut s).expect("compiles");
        sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
        let r = s.terms.intern_text(rel).expect("room");
        let a = s.terms.intern_text(arg).expect("room");
        let h = s.terms.intern_fact(r, &[Value::sym(a)]).expect("room");
        let l = Lookahead::new(&mut s).expect("compiles");
        l.dies_immediately(&mut s, &mut Matcher::new(), h)
    }

    /// P1.21 R4 — the guard is judged in the world `kb` **plus** `h`.
    ///
    /// `false ← (cand ?x) ∧ (absent (cand ?x))` can never fire in any real
    /// match: its premises are jointly unsatisfiable in one world. The probe
    /// nonetheless posits `h` into the positive premise, so evaluating the
    /// guard against a KB *without* `h` would answer about a different world
    /// and report a live hypothesis dead — the one thing the filter may never
    /// do. `examples/branching/13_lookahead_naf_world.ein` is the same shape
    /// as a whole-solve fixture.
    #[test]
    fn the_naf_world_includes_the_candidate() {
        let src = "(rule self-block ()\n  :match (and (cand ?x) (absent (cand ?x)))\n\
                   \x20 :assert (false)\n  :why \"unsatisfiable in any one world\" :priority 100)\n\
                   (relation cand T)";
        assert!(!dies(src, "cand", "A"));
    }

    /// A **nested** absent is the shape the filter must refuse to judge.
    ///
    /// The guard reads "no `p` that is not blocked by an unmet `q`". Adding
    /// `(s B)` makes the *inner* absent fail, which makes the middle one
    /// pass, which gives the outer query a match — so the guard fails in
    /// `kb + h` and the rule does not fire. Neither cheap check sees that:
    /// `kb` alone has no match, and `h` is not a positive premise of the
    /// query, so seeding it finds nothing either. Judged by those two the
    /// guard would "pass", `(false)` would be derived, and a live hypothesis
    /// would be lost — which is why the whole disjunct is skipped instead.
    /// `examples/branching/14_lookahead_unjudgeable.ein` is the same shape as
    /// a whole-solve fixture.
    #[test]
    fn a_nested_absent_is_not_judged_at_all() {
        let src = "(rule r ()\n  :match (and (s ?x) (absent (and (p ?y) \
                   (absent (and (q ?y) (absent (s ?y)))))))\n  :assert (false) \
                   :priority 100)\n\
                   (relation s T) (relation p T) (relation q T)\n\
                   (p B :source \"(1)\") (q B :source \"(2)\")";
        assert!(!dies(src, "s", "B"));
    }

    /// …and the fix must not disarm the filter: a purely positive rule that
    /// derives `(false)` from the candidate still kills it, and only for the
    /// argument that has the other premise.
    #[test]
    fn a_positive_rule_still_kills() {
        let src = "(rule blow-up ()\n  :match (and (cand ?x) (bad ?x))\n  :assert (false)\n\
                   \x20 :why \"cand + bad is absurd\" :priority 100)\n\
                   (relation cand T)\n(relation bad T)\n(bad A :source \"(1)\")";
        assert!(dies(src, "cand", "A"));
        assert!(!dies(src, "cand", "B"));
    }
}
