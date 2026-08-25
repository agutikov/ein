//! The obligation pass — what a quiescent KB still **owes**.
//!
//! M1d [P1d.2](../../../../plans/m1d_satisfiability/p1d.2_obligations/README.md)
//! S1d.2.4. [S1d.2.3](../../../../plans/m1d_satisfiability/p1d.2_obligations/s1d.2.3_the_form.md)
//! reserved the verdict atom and routed the rules that assert it into
//! [`ein_core::Program::obligations`], where nothing walked them. This is what
//! walks them.
//!
//! ### Not the agenda, and not a band
//!
//! An obligation rule derives nothing, so it has no business in a queue whose
//! job is to order derivation. It runs **once per quiescent KB, after the
//! fixpoint** — a band would have ordered its selection *inside* the loop,
//! where a debt that negative-completion (240) or elimination (400) was about
//! to pay is still outstanding. Read at the fixpoint, the tally is a function
//! of the final KB rather than of a moment on the way to it. `:priority` keeps
//! one residual meaning here: the **report order** among obligation rules,
//! which is what makes the outstanding list deterministic.
//!
//! ### Firing *is* undischarged
//!
//! Under the superseded triple `(open ?b G B)` the engine re-ran `∃b: G ∧ B`
//! per instance to ask whether the obligation was still owed — a second
//! statement of the guard, which could disagree with it. Under `(open ?R)`
//! there is one statement and it is the guard: the rule matches while the
//! witness is missing and stops matching once it has arrived. So the pass has
//! no discharge query to pay for beyond the guards it would evaluate anyway,
//! and `absents_still_pass` goes back to being about ordinary negative
//! premises.
//!
//! ### Never stored
//!
//! `(false)` can be a fact because contradiction is extension-stable: a dead
//! state stays dead under any addition. Openness is the opposite — it exists
//! to be destroyed by an extension — so a stored `open` would survive its own
//! discharge in a fork that paid it. The tally is state of the **search
//! lattice node** ([`crate::commitment::CommitmentSetResult::owes`]), beside
//! `kind`, which is the other per-node verdict that is not a fact.
//!
//! ### What it costs
//!
//! [`Owes::default`] on the first line for a program that declares no lower
//! bound, which is 150 of the corpus's 173 `.ein` files. Where there are
//! rules, one matcher run per `(rule, activator)` per quiescent KB, and the
//! plans come from the process-wide memo — so the compile is paid once per
//! pair per *process*, not once per node. The projection was resolved at load
//! (S1d.2.3), so nothing here is per-firing.
//!
//! It builds its plan list through [`PlanMemo`] rather than through an
//! [`crate::engine::Engine`] on purpose: an `Engine` narrates a `compile`
//! event for every pair it adds to its own cache, and a fresh engine per
//! quiescence would put that line in the stream once per node — narration
//! about the reader rather than about the run.

use std::ops::ControlFlow;

use ein_core::entities::Rule;
use ein_core::{Kb, Symbol, Terms, Value, render_why};
use ein_ir::Ast;
use rustc_hash::FxHashSet;

use crate::compile::{CompileError, PlanMemo, SharedMemo, activators_for, plan_key};
use crate::events::{self, Events};
use crate::firing::rendered_bindings;
use crate::match_::Matcher;
use crate::plan::{Plan, Slot};
use crate::saturator::DEFAULT_PRIORITY;

/// One undischarged obligation — a `(rule, bindings)` instance that still
/// matches at the fixpoint.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Owed {
    pub rule: Symbol,
    /// `plan.activator_args` — the parameters the activator fact bound.
    pub activator: Box<[Symbol]>,
    /// The relation whose extent is incomplete: `(open ?R)`'s argument,
    /// substituted per activator. `None` is the bare `(open)`, which counts
    /// and reports but does not attribute.
    pub relation: Option<Symbol>,
    pub bindings: Box<[(Symbol, Value)]>,
    /// The rule's `:why`, rendered against `bindings` — the report. Empty when
    /// the rule declares none.
    pub why: String,
}

/// What a quiescent KB owes: the instances, in report order.
///
/// Report order is `(:priority, the rule's load order, the activator's order,
/// the match order)` — all four deterministic, so two runs over one KB list
/// the same debts in the same order.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Owes {
    instances: Vec<Owed>,
}

impl Owes {
    pub fn instances(&self) -> &[Owed] {
        &self.instances
    }

    pub fn total(&self) -> usize {
        self.instances.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    /// The per-relation tally, in first-owed order — what makes
    /// `--json-summary` say `owes: {pet-loc: 9, nation-loc: 8, …}` and what
    /// the conservation audit checks against a declaration.
    ///
    /// A bare `(open)` contributes to [`Owes::total`] and to nothing here: it
    /// has no slot to name.
    pub fn by_relation(&self) -> Vec<(Symbol, usize)> {
        let mut out: Vec<(Symbol, usize)> = Vec::new();
        for i in &self.instances {
            let Some(r) = i.relation else { continue };
            match out.iter_mut().find(|(s, _)| *s == r) {
                Some((_, n)) => *n += 1,
                None => out.push((r, 1)),
            }
        }
        out
    }

    /// The count for one relation — the conservation audit's reader.
    pub fn owed_by(&self, relation: Symbol) -> usize {
        self.instances
            .iter()
            .filter(|i| i.relation == Some(relation))
            .count()
    }
}

/// Read what `kb` owes, at its fixpoint.
///
/// Free — one `is_empty` — for a program that states no obligation. The caller
/// is expected to have saturated: the pass reads the KB it is given and
/// derives nothing, so calling it early is not unsound, only early.
///
/// `events` narrates one `owe` line per instance when `--events` is on. It is
/// emitted here rather than by the caller so a worker's deferred sink carries
/// it like every other event.
pub fn tally(
    kb: &Kb,
    terms: &mut Terms,
    ast: &Ast,
    memo: &SharedMemo,
    events: &mut Events,
) -> Result<Owes, CompileError> {
    if kb.program().obligations.is_empty() {
        return Ok(Owes::default());
    }
    let plans = plans_for(kb, terms, ast, memo)?;

    let mut matcher = Matcher::new();
    let mut guards = Matcher::new();
    let mut instances = Vec::new();
    let mut hits: Vec<Hit> = Vec::new();
    for plan in plans {
        let relation = open_argument(&plan, terms);
        // Two matchers, because the run borrows the first for its whole
        // length: the matches are collected here and their guards are judged
        // below.
        hits.clear();
        let mut seen: FxHashSet<Box<[(Symbol, Value)]>> = FxHashSet::default();
        matcher.run(kb, terms, ast, &plan, &mut |m| {
            // The **bindings**, not the register file: a register this match
            // did not bind still holds whatever the previous one left in it,
            // so the raw slice is not an identity. What the plan bound *is*
            // one — and two disjuncts reaching the same bindings are one debt,
            // not two.
            let bindings: Box<[(Symbol, Value)]> =
                m.bindings().collect::<Vec<_>>().into_boxed_slice();
            if seen.insert(bindings.clone()) {
                hits.push(Hit {
                    disjunct: m.disjunct,
                    regs: m.regs().into(),
                    bindings,
                });
            }
            ControlFlow::Continue(())
        });
        for hit in hits.drain(..) {
            let Hit {
                disjunct,
                regs,
                bindings,
            } = hit;
            // A guard that *holds* found the witness, so this instance is
            // discharged and is not a debt.
            let span = plan.disjuncts[disjunct].guards;
            if plan
                .guards(span)
                .iter()
                .any(|g| guards.holds(kb, terms, ast, &plan, g, &regs))
            {
                continue;
            }
            let why = plan
                .why
                .map(|w| render_why(terms.sym(w), &rendered_bindings(terms, &bindings)))
                .unwrap_or_default();
            instances.push(Owed {
                rule: plan.rule,
                activator: plan.activator_args.clone(),
                relation,
                bindings,
                why,
            });
        }
    }

    narrate(&instances, terms, events);
    Ok(Owes { instances })
}

/// One match, kept until its guards are judged.
struct Hit {
    disjunct: usize,
    regs: Box<[Value]>,
    /// [`crate::match_::Match::bindings`], snapshotted: the trail it walks
    /// belongs to the matcher and is gone once the run ends.
    bindings: Box<[(Symbol, Value)]>,
}

/// One plan per `(obligation rule, activator)`, in report order.
pub(crate) fn plans_for(
    kb: &Kb,
    terms: &mut Terms,
    ast: &Ast,
    memo: &SharedMemo,
) -> Result<Vec<std::sync::Arc<Plan>>, CompileError> {
    // Load order breaks a tie inside a priority, so the sort is stable and the
    // enumeration order is the registry's.
    let mut rules: Vec<(i64, usize, &Rule)> = kb
        .program()
        .obligations
        .values()
        .enumerate()
        .map(|(i, r)| (priority_of(terms, r), i, r))
        .collect();
    rules.sort_by_key(|&(p, i, _)| (p, i));

    let mut out = Vec::new();
    for (_, _, rule) in rules {
        for act in activators_for(kb, terms, rule) {
            let key = plan_key(terms, rule, act);
            let mut memo: std::sync::MutexGuard<'_, PlanMemo> =
                memo.lock().expect("no compiler panicked");
            let id = memo.intern_keyed(key, ast, terms, rule, act)?;
            out.push(memo.get_arc(id));
        }
    }
    Ok(out)
}

fn narrate(instances: &[Owed], terms: &Terms, events: &mut Events) {
    if !events.on() {
        return;
    }
    for i in instances {
        let rule = terms.sym(i.rule).to_string();
        let activator: Vec<String> = i
            .activator
            .iter()
            .map(|&x| terms.sym(x).to_string())
            .collect();
        let relation = i.relation.map(|r| terms.sym(r).to_string());
        let bindings = events::binding_pairs(terms, &i.bindings);
        events.emit("owe", |l| {
            l.str("rule", &rule);
            l.owned_strs("activator", activator);
            l.str("relation", relation.as_deref().unwrap_or(""));
            l.bindings("bindings", bindings);
            l.str("why", &i.why);
        });
    }
}

/// `(open ?R)`'s argument, substituted — or `None` for the bare `(open)`.
///
/// The parameter is bound by the activator, so the compiler has already turned
/// `?R` into a `Slot::Const` naming a relation ([`crate::compile`]: "the
/// activator fact binds the rule's parameter list **before** matching
/// begins"). Nothing here re-derives it.
pub(crate) fn open_argument(plan: &Plan, terms: &Terms) -> Option<Symbol> {
    let Some(Slot::Nested { rel, slots }) = plan.assert_template() else {
        return None;
    };
    if rel != terms.kernel.open {
        return None;
    }
    match plan.slots(slots).first() {
        Some(Slot::Const(v)) => v.as_sym(),
        _ => None,
    }
}

fn priority_of(terms: &Terms, rule: &Rule) -> i64 {
    let Some(p) = rule.priority else {
        return DEFAULT_PRIORITY;
    };
    terms.ints.value(p).unwrap_or_else(|| {
        if terms.int_text(p).starts_with('-') {
            i64::MIN
        } else {
            i64::MAX
        }
    })
}
