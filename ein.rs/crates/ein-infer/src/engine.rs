//! The engine — the compile cache, its **order**, and the `_fired` record.
//!
//! ein.py's `Engine` is a dict from `(rule_name, activator_args)` to a
//! `JoinPlan` plus a set of binding keys that have already fired. Two things
//! about it are load-bearing and neither is the caching:
//!
//! 1. **The cache's iteration order is observable.** `_enqueue_pass`'s full
//!    pass walks `cache.values()`, so the order plans were first compiled in
//!    reaches the firing order and therefore the trace. A fork builds its cache
//!    by iterating `rules × the fork's own activators`, so a fork-derived
//!    activator for an early rule sorts *before* a root activator of a later
//!    rule. Sharing one cache across forks and appending would be a different
//!    order, which is why the process-wide memo holds the **plans** and each
//!    engine keeps its own ordered list
//!    ([design/06](../../../../docs/history/m1a_rust/design/06_saturation.md) § Win A).
//! 2. **`_fired` is the canonical "already applied" record**, guard-free: once
//!    the conclusion is derived, every other disjunct for those bindings is
//!    redundant, which is what it already meant.

use ein_core::entities::Rule;
use ein_core::{FactId, Kb, Symbol, Terms};
use ein_ir::Ast;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::compile::{CompileError, PlanKey, SharedMemo, activators_into, plan_key};
use crate::events::Events;
use crate::firing::{ActivatorId, BindingKey};
use std::sync::Arc;

use crate::plan::{Plan, PlanId};

/// One engine's view of the compiled program.
///
/// `Clone` because a fork that *resumes* the root's saturation
/// ([`crate::saturator::Snapshot`]) inherits the plan list, its order and
/// `fired` wholesale. The `Arc`s and the memo handle are shared, so what the
/// clone actually copies is the index vectors and the `fired` set.
#[derive(Clone, Default)]
pub struct Engine {
    /// The plans, in ein.py's cache-insertion order.
    plans: Vec<PlanId>,
    /// The same plans, resolved — parallel to `plans`.
    ///
    /// Holding the `Arc` here rather than reaching into the memo is what keeps
    /// the read path lock-free: `plan` / `plan_arc` run per enqueue pass, the
    /// memo is only touched on a cache miss.
    arcs: Vec<Arc<Plan>>,
    keys: Vec<PlanKey>,
    by_key: FxHashMap<PlanKey, usize>,
    /// The interned `plan.activator_args` of each plan, parallel to `plans`.
    activators: Vec<ActivatorId>,
    activator_ids: FxHashMap<Box<[Symbol]>, ActivatorId>,
    /// Binding keys that have already been applied.
    pub fired: FxHashSet<BindingKey>,
    /// Shared with every other engine of this run — design/06 § Win A. What
    /// may **not** be shared is the order below it, which is why `plans` is
    /// per-engine and this is not.
    memo: SharedMemo,
    /// The rule-application count the plan list was last built at — a walk of
    /// `rules × activators` can only find something new when a rule gained an
    /// activator, and nothing else adds one.
    last_rule_apps: usize,
}

impl Engine {
    /// An engine with a **private** memo — a one-shot caller that compiles
    /// nothing anyone else will ask for again.
    pub fn new() -> Engine {
        Engine::with_memo(SharedMemo::default())
    }

    /// An engine that compiles into `memo`, which outlives it.
    pub fn with_memo(memo: SharedMemo) -> Engine {
        Engine {
            last_rule_apps: usize::MAX,
            memo,
            ..Engine::default()
        }
    }

    pub fn len(&self) -> usize {
        self.plans.len()
    }

    pub fn is_empty(&self) -> bool {
        self.plans.is_empty()
    }

    pub fn plan(&self, index: usize) -> &Plan {
        &self.arcs[index]
    }

    /// A handle the caller can hold while mutating the engine — which
    /// `_apply` must, since it writes `fired` with the plan in hand.
    pub fn plan_arc(&self, index: usize) -> Arc<Plan> {
        Arc::clone(&self.arcs[index])
    }

    pub fn plan_id(&self, index: usize) -> PlanId {
        self.plans[index]
    }

    pub fn activator(&self, index: usize) -> ActivatorId {
        self.activators[index]
    }

    pub fn key(&self, index: usize) -> &PlanKey {
        &self.keys[index]
    }

    /// How many plans this run has compiled, across every engine sharing the
    /// memo — the number [S1a.6.8](../../../../docs/history/m1a_rust/README.md#s1a68--the-compile-cache-and-the-extent-counts)
    /// reports in place of a prediction.
    pub fn n_memoised(&self) -> usize {
        self.memo.lock().expect("no compiler panicked").len()
    }

    /// Walk `rules × activators` and cache one plan per pair.
    ///
    /// Skipped entirely when no rule gained an activator since the last walk —
    /// which is exact, because the walk's *only* source of new pairs is
    /// `rule_apps_by_rule`. The debug build checks the shortcut against a full
    /// recompute rather than trusting the argument.
    pub fn compile_all(
        &mut self,
        ast: &Ast,
        terms: &mut Terms,
        kb: &Kb,
        events: &mut Events,
    ) -> Result<(), CompileError> {
        let n = kb.n_rule_apps();
        if n == self.last_rule_apps {
            debug_assert!(
                self.would_not_grow(ast, terms, kb),
                "the rule-application counter said nothing changed and a full \
                 walk disagreed"
            );
            return Ok(());
        }
        self.last_rule_apps = n;
        // T1a.6.4.0b — the walk allocated a `Vec<Rule>` (one `Box<[Symbol]>`
        // per rule, cloned) and a `Vec<Option<FactId>>` per rule, on every
        // call. Neither is needed: `rules` borrows the program and `compile_for`
        // touches `self`, `terms` and `events`, so the two borrows never meet,
        // and one buffer serves every rule. The engine a `Lookahead` throws
        // away after one pass makes this walk ~120 pairs of pure setup, ~40
        // times per solve ([S1a.6.4](../../../../docs/history/m1a_rust/README.md#s1a64--hypgen-and-lattice-hot-paths)).
        let mut acts: Vec<Option<FactId>> = Vec::new();
        for rule in kb.program().rules.values() {
            activators_into(kb, terms, rule, &mut acts);
            for act in acts.iter().copied() {
                self.compile_for(ast, terms, rule, act, events)?;
            }
        }
        Ok(())
    }

    /// Compile (and cache) one pair, returning its position in the plan list.
    pub fn compile_for(
        &mut self,
        ast: &Ast,
        terms: &mut Terms,
        rule: &Rule,
        activator: Option<FactId>,
        events: &mut Events,
    ) -> Result<usize, CompileError> {
        let key = plan_key(terms, rule, activator);
        if let Some(&at) = self.by_key.get(&key) {
            return Ok(at);
        }
        let (id, plan) = {
            let mut memo = self.memo.lock().expect("no compiler panicked");
            let id = memo.intern_keyed(key.clone(), ast, terms, rule, activator)?;
            (id, memo.get_arc(id))
        };
        let at = self.plans.len();
        // The `compile` event fires on an **engine** miss, not a memo miss: it
        // reports what this engine had to add to its cache, which is what
        // ein.py's `compile_for` reports and what the cache order is about.
        if events.on() {
            let (rule_name, n_steps, n_disjuncts, n_guards, asserts) = (
                terms.sym(plan.rule).to_string(),
                plan.disjuncts[0].steps.len(),
                plan.disjuncts.len() - 1,
                // `len(plan.naf_guards)` — one guard *tuple* per disjunct,
                // so this counts disjuncts, not guarded ones. Reproduced
                // rather than corrected: the event is a comparison surface.
                plan.disjuncts.len(),
                plan.asserts.len(),
            );
            let activator_args: Vec<String> = key
                .activator
                .iter()
                .map(|&s| terms.sym(s).to_string())
                .collect();
            events.emit("compile", |l| {
                l.str("rule", &rule_name);
                l.owned_strs("activator", activator_args);
                l.num("n_steps", n_steps as i64);
                l.num("n_disjuncts", n_disjuncts as i64);
                l.num("n_guards", n_guards as i64);
                l.num("asserts", asserts as i64);
            });
        }
        // `plan` is a local `Arc`, not a borrow of `self`, so the two reads
        // below need no copy of what they read (T1a.6.4.0b): `reg_names` is
        // only ever read by `check_layout`, which is a debug-build assertion,
        // and cloning it in the shipping build was one allocation per engine
        // miss for nothing at all.
        let activator_id = self.intern_activator(&plan.activator_args);
        self.check_layout(plan.rule, activator_id, &plan.reg_names);
        self.plans.push(id);
        self.arcs.push(plan);
        self.keys.push(key.clone());
        self.activators.push(activator_id);
        self.by_key.insert(key, at);
        Ok(at)
    }

    fn intern_activator(&mut self, args: &[Symbol]) -> ActivatorId {
        if let Some(&id) = self.activator_ids.get(args) {
            return id;
        }
        let id = ActivatorId(self.activator_ids.len() as u32);
        self.activator_ids.insert(args.into(), id);
        id
    }

    /// The invariant [`BindingKey`] leans on, checked where the plan list is
    /// built.
    ///
    /// It holds because a register layout is a function of the rule and of
    /// *which* parameters the activator bound, and two activators with the
    /// same string arguments bind the same parameters — unless one carries a
    /// nested fact where the other carries a name, which is a shape no rule
    /// application has. Debug-only, because it is an argument about the data
    /// rather than about the code, and an argument is what a debug assertion
    /// is for.
    fn check_layout(&self, rule: Symbol, activator: ActivatorId, reg_names: &[Symbol]) {
        if cfg!(debug_assertions) {
            for (i, other) in self.arcs.iter().enumerate() {
                if self.activators[i] == activator && other.rule == rule {
                    assert_eq!(
                        other.reg_names.as_ref(),
                        reg_names,
                        "two plans share (rule, activator) and disagree on \
                         their register layout — `BindingKey` compares their \
                         value vectors and would be comparing different \
                         variables"
                    );
                }
            }
        }
    }

    /// Would a full walk add anything? The shortcut's proof obligation.
    #[cfg(debug_assertions)]
    fn would_not_grow(&self, _ast: &Ast, terms: &Terms, kb: &Kb) -> bool {
        let mut n = 0;
        for (_, rule) in kb.program().rules.iter() {
            n += crate::compile::activators_for(kb, terms, rule).len();
        }
        // Every pair the walk would visit is already cached, so the *count* of
        // pairs is the count of distinct keys — which the plan list holds.
        n <= self.plans.len()
    }

    #[cfg(not(debug_assertions))]
    fn would_not_grow(&self, _ast: &Ast, _terms: &Terms, _kb: &Kb) -> bool {
        true
    }
}
