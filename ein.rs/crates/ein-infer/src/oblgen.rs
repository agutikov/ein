//! The obligations rung — hypotheses from what a state still owes.
//!
//! M1d [P1d.2](../../../../docs/history/m1d_satisfiability/README.md#p1d2--obligations)
//! [S1d.2.5](../../../../docs/history/m1d_satisfiability/README.md#s1d25--hypotheses-from-obligations).
//! [S1d.2.4](../../../../docs/history/m1d_satisfiability/README.md#s1d24--obligations-in-the-saturator)
//! made a quiescent state able to say what it owes; this makes the debt a
//! *choice point*.
//!
//! ### The ladder
//!
//! [design/07](../../../../docs/history/m1a_rust/design/07_search_layer.md)'s
//! "hrule presence *is* the switch", grown one rung:
//!
//! | the program declares | hypotheses come from |
//! |---|---|
//! | any `(hrule …)` | the user's hrules — an override, exactly as before |
//! | an obligation activated at the state, no hrule | **this module** |
//! | neither | the blind combinatorial enumerator |
//!
//! The middle row is about what the program **states**, an obligation and its
//! activator, not about what this state happens to owe: a discharged state
//! under rung 2 proposes nothing and is therefore complete, which is what
//! *complete means discharged* has to mean. Falling back to the blind
//! enumerator whenever nothing was owed would put exhaustion back in charge of
//! completeness and make the rung unreachable at exactly the states it exists
//! to judge. Nor is it about what the program **loads**: an obligation rule
//! is activated where the state holds its activator, and always where it has
//! no parameters (`mechanisms::obligation_plans`, the one reading of
//! an obligation). Every import brings its module whole, obligation rules
//! among it, so a rule nothing activates takes no program off the blind
//! enumerator.
//!
//! ### The candidate set
//!
//! An obligation rule is `absent (and G(b) (R ā b))` — *no witness exists* —
//! and `(open ?R)` names `R`. So the facts that would discharge one instance
//! are exactly `{(R ā b) : G(b)}`, and the engine reads them by running the
//! guard's own sub-plan with the witness step **skipped**
//! ([`crate::match_::Matcher::scan_without`]). Nothing restates the domain:
//! the branch set is the guard, evaluated at this quiescence, which is
//! [`domain_contract.md`](../../../../docs/history/m1d_satisfiability/domain_contract.md)
//! C1 and C4 in one call.
//!
//! ### What it proposes, and why it is the union
//!
//! One obligation's candidate set is mutually exclusive and jointly
//! exhaustive: some `b` must witness it, so branching over them loses no
//! model. **What this module builds is H**, though, and H is one set: built
//! once at root's fixpoint and read by every route and the oracle, each of
//! whose nodes is root with a subset of it committed. So the rung proposes the
//! **union** over the accepted instances, in report order, of which every
//! single instance's set is a subset. The branch is taken elsewhere: the
//! obligation tree, which answers `-n`, `--single` and the two lines, reads
//! each owed instance's own alternatives from `alternatives`, L5's table,
//! scanned at root by this module's projection, and branches on one instance
//! at a time where L5 or L9 licenses it.
//!
//! ### When it declines
//!
//! The domain contract's C4: a branch is jointly exhaustive only while the
//! candidate set cannot grow underneath it, so an obligation whose guard scans
//! a relation the rung itself proposes is **declined** — and a declined
//! obligation takes the whole call back to the blind generator, which is what
//! keeps completeness a property of the ladder rather than of the puzzle.
//!
//! **A relation grows by derivation too** (M1e2
//! [S1e2.2.15](../../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.15_obligations_rung_universe.md),
//! W14). The candidates are the guard's scan over *root's* KB, so a domain
//! value only a commitment derives is in no candidate, and R missed the model
//! it licenses. Since then the rung also declines when an obligation's trigger,
//! or a guard beside its witness step, reads a relation in `D`: the relations
//! a commitment can change, seeded with what this rung proposes and closed
//! under the saturation plans, which [`crate::monotone`] computes once for
//! this and for its static check. The user ruled the fall-through on
//! 2026-11-04 (Q-M1e2.10 (a)): R is then the blind enumerator's region, which
//! on an untyped relation holds facts no rule refutes or requires. `D` is at
//! relation grain, so a program that names the value at root declines too.
//! Scoping is the other half: `:no-hypothesis`, `:hypothesis-relations` and
//! `(__closed__ R)` bind here exactly as they bind the blind enumerator, and
//! a state owing only relations they exclude is **stuck** — reported, never
//! silently complete.
//!
//! ### A debt root is not told of
//!
//! H is taken once, at root, and an obligation's activator is an ordinary
//! fact: a rule can derive one at a node. That node owes what the activator's
//! plan matches, and root, which never held the activator, proposed nothing
//! that pays it. So **the rung also proposes what such a debt would**
//! (Q-M1e.11, by the static reading):
//!
//! 1. the **activators a commitment can derive** are read off the rules and
//!    root's plans, with no saturation
//!    ([`crate::forbid`]'s `derivable_activators`): each conclusion that
//!    holds no variable and would activate an obligation rule, in the
//!    `:assert` of a rule that reads a relation of `D⁺` or is activated by
//!    one. `D⁺` is *forbid*'s, by its one reading of the rules, seeded with
//!    the relations of the candidates the call would emit with every switch
//!    off;
//! 2. for each one the state does not hold, the rule's plan under it is
//!    compiled, beside the memo, and **scanned at this state as a plan of
//!    the state's own is**: the instances it matches here, each one's guard
//!    with the witness step skipped;
//! 3. a candidate that adds a relation adds it to the seed, so the two steps
//!    are **repeated until they add nothing**.
//!
//! A later plan is held to every decline above together with the state's:
//! a later debt whose guard scans a relation a commitment can change declines
//! the call, and the blind enumerator builds H. Scoping binds it as it binds
//! any plan. What the state owes is counted as before
//! ([`RungReport::owed`], `branches`, `candidates`); what a later debt adds
//! counts apart ([`RungReport::later_debts`], `later_candidates`) and is
//! emitted after it, so a call that reads no later debt emits what it emitted,
//! in that order, and narrates the line it narrated.
//!
//! **The reading is at relation grain and inexact both ways**, and both are
//! held by name in `ein-infer/tests/later_debts.rs`. *Over*: a rule that
//! reads what a commitment can change need fire at no alive node, and H then
//! holds a candidate for a debt nothing owes, which R keeps wherever nothing
//! refutes it. *Under*: an activator its rule writes with a variable is not
//! read, and the node that derives it owes what no candidate pays. It does
//! not reach a generator of another kind either: an hrule whose `:match`
//! reads what a commitment can change proposes at a node what it did not at
//! root, and the hrule rung is as it was. The program stays one the static
//! check flags, since the activator's relation is one a commitment can
//! change; what a node is there is not this module's.

use std::ops::ControlFlow;
use std::sync::Arc;

use ein_core::{FactId, Symbol, Value};
use rustc_hash::FxHashSet;

use crate::compile::CompileError;
use crate::hypgen::{HypGenStats, Skip};
use crate::match_::Matcher;
use crate::plan::{NafGuard, Plan, Slot, Span, Step};
use crate::saturator::Session;

/// Which rung of the ladder a generation call took.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum Mode {
    /// No obligation is activated at the state — the blind enumerator.
    #[default]
    Blind,
    /// A `(hrule …)` overrides the ladder. Never narrated — rung 1 emits the
    /// stream it emitted before M1d, to the byte.
    Hrules,
    /// The rung generated: every owed obligation's candidates, deduped.
    Obligations,
    /// Owed, and nothing may be branched on — the ladder's dead end.
    Stuck,
    /// An obligation was declined (C4, over what the rung proposes or since
    /// M1e2 S1e2.2.15 over `D`, or a projection that will not resolve), so
    /// the call fell through to the blind enumerator.
    Declined,
    /// The program states `(solve :no-search)` (M1f S1f.6.36): no rung ran,
    /// and nothing was proposed. Never narrated, as rung 1 is not.
    NoSearch,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Blind => "blind",
            Mode::Hrules => "hrules",
            Mode::Obligations => "obligations",
            Mode::Stuck => "stuck",
            Mode::Declined => "declined",
            Mode::NoSearch => "no-search",
        }
    }
}

/// What one generation call did with the obligations it found.
///
/// Zero everywhere for a program that activates no obligation at the state:
/// the ladder asks that before anything here runs.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct RungReport {
    pub mode: Mode,
    /// Undischarged obligation instances at this quiescence.
    pub owed: u64,
    /// …of which branched on.
    pub branches: u64,
    /// …of which not, because scoping excludes the relation they owe.
    pub declined: u64,
    /// Candidate facts proposed, before the filter pipeline.
    pub candidates: u64,
    /// Hypothesis-eligible relations **no obligation names** — the ladder's
    /// completeness condition, as a number.
    ///
    /// The rung is exhaustive iff obligations and saturation between them
    /// determine every remaining open fact ([S1d.2.5] T1d.2.5.3). `0` makes
    /// that structural: every relation a hypothesis could be about is one
    /// some obligation owes, so a branch set that discharges every debt has
    /// left nothing undecided. Non-zero does **not** make it false — it says
    /// the claim now rests on saturation determining those relations, which
    /// only a model-set comparison can settle, and the zebra family is where
    /// it was settled.
    ///
    /// [S1d.2.5]: `docs/history/m1d_satisfiability/README.md#s1d25--hypotheses-from-obligations`
    pub uncovered: u64,
    /// Debts this state does not owe and a commitment can bring: the
    /// activators of an obligation rule, not held here, that a rule's
    /// `:assert` writes and the rung compiled a plan of (§ *A debt root is
    /// not told of*). `0` on every program in which no rule that reads what a
    /// commitment can change writes one, and then the call is what it was
    /// without the reading.
    pub later_debts: u64,
    /// Candidate facts proposed for them, before the filter pipeline. Not in
    /// [`Self::candidates`], which counts what the state owes.
    pub later_candidates: u64,
}

/// The order a call emits the owed instances' candidates in, as its `rung`
/// line's `reason` names it: **report order**, the obligation plans in order
/// and each plan's matches in match order. It is the one order the rung has.
/// A route reads it only as the tie-break of H's sort by raw text
/// (`region::order_key`); it is the order of the `hyp` events that follow the
/// `rung` line.
pub const ORDER: &str = "rule-order";

/// The static half of the ladder's decision, per `(rule, activator)`.
struct Proj {
    plan: Arc<Plan>,
    /// `(open ?R)`'s argument, substituted.
    relation: Symbol,
    /// Per disjunct: which of its guards holds the witness, and where inside
    /// that guard's sub-plan the witness step is.
    witness: Vec<(usize, usize)>,
    /// The pre-candidate skip scoping earns this relation, if any.
    skip: Option<Skip>,
}

/// One owed instance's branch: the facts that would discharge it.
///
/// A `Vec<Vec<FactId>>` by another name. A call emits the branches' union, in
/// report order, and counts each instance's candidates; the per-instance sets
/// a route branches on are `alternatives`'.
struct Branch {
    candidates: Vec<FactId>,
}

/// What one plan matches at a state: how many instances it leaves owed and,
/// where scoping lets the rung branch on its relation, each one's
/// alternatives. Reading it bumps no counter and narrates nothing.
struct Scan {
    owed: usize,
    branches: Vec<Branch>,
}

/// One debt root does not owe and a commitment can bring: an obligation
/// rule's activator that root's fixpoint does not hold and a rule's `:assert`
/// writes, with what the rung proposed for it (§ *A debt root is not told
/// of*, the module's).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LaterDebt {
    /// The obligation rule.
    pub rule: Symbol,
    /// The activator, a fact no state need hold.
    pub activator: FactId,
    /// The relation its plan owes: `(open ?R)`'s argument under the
    /// activator.
    pub relation: Symbol,
    /// The saturation rules whose `:assert` writes the activator, in the
    /// order the reading met them.
    pub by: Vec<Symbol>,
    /// The candidates the call emitted for it that no debt before it had
    /// proposed, in emission order. Empty where every payment was a candidate
    /// already, where scoping takes its relation out, or where nothing
    /// matches its trigger at this state.
    pub added: Vec<FactId>,
}

/// A [`LaterDebt`] as text, for the surfaces that print one.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LaterDebtText {
    /// The obligation rule's name.
    pub rule: String,
    /// The activator, as the fact a rule would conclude.
    pub activator: String,
    /// The relation owed.
    pub relation: String,
    /// The rules that write the activator.
    pub by: Vec<String>,
    /// The candidates added for it, in the universe's order.
    pub added: Vec<String>,
}

impl LaterDebt {
    pub fn text(&self, terms: &ein_core::Terms) -> LaterDebtText {
        let mut added: Vec<(String, String)> = self
            .added
            .iter()
            .map(|&f| {
                (
                    crate::region::order_key(terms, f),
                    crate::events::sexpr(terms, f),
                )
            })
            .collect();
        added.sort();
        LaterDebtText {
            rule: terms.sym(self.rule).to_string(),
            activator: crate::events::sexpr(terms, self.activator),
            relation: terms.sym(self.relation).to_string(),
            by: self.by.iter().map(|&r| terms.sym(r).to_string()).collect(),
            added: added.into_iter().map(|(_, text)| text).collect(),
        }
    }
}

/// Resolve each of `plans` and push it onto `projs`, or say why the rung
/// declines: the first plan that names no relation, or whose witness step
/// does not resolve under its activator.
fn admit(
    terms: &ein_core::Terms,
    plans: Vec<Arc<Plan>>,
    allowed: &Option<FxHashSet<Symbol>>,
    excluded: &FxHashSet<Symbol>,
    closed: &FxHashSet<Symbol>,
    projs: &mut Vec<Proj>,
) -> Result<(), &'static str> {
    for plan in plans {
        // The bare `(open)` counts and reports and names nothing, so there is
        // no relation to branch on and no guard to read a domain out of
        // (S1d.2.3 item 4's table: `(open)` counts, `(open ?R)` also
        // attributes and branches). One of them in the program is enough to
        // stop the rung claiming exhaustiveness, so the call falls through
        // rather than silently owing something nobody proposes for.
        if crate::obligations::open_argument(&plan, terms).is_none() {
            return Err("a bare `(open)` names no relation to branch on");
        }
        let Some(mut proj) = project(&plan, terms) else {
            // A projection that will not resolve here is one the loader could
            // not have refused: it depends on the activator's substitution.
            // Declining is the conservative half of the same rule C4 uses.
            return Err("a projection did not resolve per activator");
        };
        proj.skip = scoping(proj.relation, allowed, excluded, closed);
        projs.push(proj);
    }
    Ok(())
}

/// The instances `p` leaves owed at `s`'s state, and their alternatives.
fn scan(s: &mut Session<'_>, p: &Proj, matcher: &mut Matcher, guards: &mut Matcher) -> Scan {
    let mut owed: Vec<(usize, Box<[Value]>)> = Vec::new();
    // **Deduped by bindings only where two disjuncts could collide**, which
    // is `obligations::tally`'s rule and its reason: a register the match
    // did not bind still holds what the previous one left there, so the raw
    // file is not an identity, and two `(or …)` arms reaching the same
    // bindings are one debt. Collecting the bindings costs an allocation
    // per match, so a single-disjunct rule — which is every obligation rule
    // in the stdlib — does not pay for a collision it cannot have.
    let mut seen: FxHashSet<Box<[(Symbol, Value)]>> = FxHashSet::default();
    let dedup = p.plan.disjuncts.len() > 1;
    matcher.run(s.kb, s.terms, s.ast, &p.plan, &mut |m| {
        if dedup && !seen.insert(m.bindings().collect::<Vec<_>>().into_boxed_slice()) {
            return ControlFlow::Continue(());
        }
        owed.push((m.disjunct, m.regs().into()));
        ControlFlow::Continue(())
    });
    // A guard that *holds* found the witness: that instance is discharged
    // and owes nothing. The same predicate `obligations::tally` reads, and
    // it is the rule's own `absent` rather than a second statement of it.
    owed.retain(|(disjunct, regs)| {
        let span = p.plan.disjuncts[*disjunct].guards;
        !p.plan
            .guards(span)
            .iter()
            .any(|g| guards.holds(s.kb, s.terms, s.ast, &p.plan, g, regs))
    });
    let mut out = Scan {
        owed: owed.len(),
        branches: Vec::new(),
    };
    if p.skip.is_some() {
        // Scoped out: counted and not branched on.
        return out;
    }
    for (disjunct, regs) in owed {
        let (gi, at) = p.witness[disjunct];
        let guard = &p.plan.guards(p.plan.disjuncts[disjunct].guards)[gi];
        let Step::Rel(step) = p.plan.steps(guard.sub)[at] else {
            continue;
        };
        // Two passes, because the walk borrows the tables the intern
        // writes to — the shape `hrule::candidates` takes for the same
        // reason.
        let mut rows: Vec<Box<[Value]>> = Vec::new();
        matcher.scan_without(
            s.kb,
            s.terms,
            s.ast,
            &p.plan,
            guard,
            &regs,
            at,
            &mut |sub| {
                rows.push(sub.into());
                ControlFlow::Continue(())
            },
        );
        let mut candidates = Vec::with_capacity(rows.len());
        for row in &rows {
            if let Some(fact) = witness_fact(s, &p.plan, step.rel, step.slots, row) {
                candidates.push(fact);
            }
        }
        out.branches.push(Branch { candidates });
    }
    out
}

/// Generate from the obligations, or say why not.
///
/// Returns the rung it took. `Mode::Declined` means **nothing was emitted and
/// the caller must fall through to the blind enumerator**; every other mode
/// means this call is the whole generation.
///
/// Every plan the call reads, the state's and those of the activators a
/// commitment can derive (§ *A debt root is not told of*), is settled before
/// the first candidate is emitted, because a decline has to come before any
/// emission and a later plan can decline the call as one of the state's can.
#[allow(clippy::too_many_arguments)]
pub(crate) fn generate(
    s: &mut Session<'_>,
    allowed: &Option<FxHashSet<Symbol>>,
    excluded: &FxHashSet<Symbol>,
    closed: &FxHashSet<Symbol>,
    stats: &mut HypGenStats,
    emit: &mut dyn FnMut(&mut Session<'_>, &mut HypGenStats, FactId) -> ControlFlow<()>,
) -> Result<RungReport, CompileError> {
    let memo = s.memo.clone();
    let program = s.kb.program_arc();
    let plans = crate::obligations::plans_for(s.kb, s.terms, s.ast, &memo)?;
    // The plans are the memo's, as the static check's are: root's saturation
    // compiled them into it, the activators' included, before any generation
    // call. A plan of a later debt is compiled beside the memo, never into
    // it: the memo holds what a state compiled, and the static check reads
    // it as that.
    let compiled = crate::monotone::plans_of(&memo);
    let mut projs: Vec<Proj> = Vec::with_capacity(plans.len());
    if let Err(reason) = admit(s.terms, plans, allowed, excluded, closed, &mut projs) {
        return Ok(declined(s, reason));
    }
    // `projs[..at_state]` are the plans of the activators this state holds;
    // those past it are the later debts', one each, in `later`'s order.
    let at_state = projs.len();
    let mut scans: Vec<Scan> = Vec::with_capacity(projs.len());
    let mut later: Vec<LaterDebt> = Vec::new();
    let mut matcher = Matcher::new();
    let mut guards = Matcher::new();
    let uncovered = loop {
        // C4 — the candidate set must not be able to grow underneath the
        // branch, so no accepted obligation may scan a relation this rung
        // proposes.
        let owed_rels: FxHashSet<Symbol> = projs.iter().map(|p| p.relation).collect();
        for p in &projs {
            if let Some(rel) = scanned_relation_in(p, &owed_rels) {
                let rel = s.terms.sym(rel).to_string();
                let reason = format!("an obligation scans `{rel}`, which the rung itself proposes");
                return Ok(declined(s, &reason));
            }
        }
        // C4 over `D` (M1e2 S1e2.2.15, W14): nor may it grow by derivation. A
        // relation a commitment can change — `D`, seeded with what this rung
        // proposes and closed under the saturation plans, `crate::monotone`'s
        // one computation of it — can gain at a node a domain value root's
        // scan never saw, and the candidate that value licenses is then in no
        // `H`.
        let proposed = projs
            .iter()
            .filter(|p| p.skip.is_none())
            .map(|p| p.relation);
        let dynamic = crate::monotone::dynamic(s.kb, s.terms, &compiled, proposed);
        for p in &projs {
            if let Some(rel) = premise_reads(p, s.terms)
                .into_iter()
                .find(|&r| crate::monotone::in_set(&dynamic, r, s.terms))
            {
                let reason = format!(
                    "an obligation scans `{}`, a relation a commitment can change",
                    rel.label(s.terms)
                );
                return Ok(declined(s, &reason));
            }
        }
        // The plans admitted since the last round, each scanned once, in
        // report order: plan order, then match order.
        for p in &projs[scans.len()..] {
            let scan = scan(s, p, &mut matcher, &mut guards);
            scans.push(scan);
        }
        // What a debt root is not told of would propose. The relations a
        // commitment can add are those of the candidates the call would emit
        // with every switch off, which is `H`'s pass: a fact this state
        // neither holds nor denies.
        let mut seed: Vec<Symbol> = Vec::new();
        for b in scans.iter().flat_map(|scan| scan.branches.iter()) {
            for &fact in &b.candidates {
                let rel = s.terms.facts.get(fact).0;
                if !seed.contains(&rel) && !s.kb.is_negated(fact) && !s.kb.contains(fact) {
                    seed.push(rel);
                }
            }
        }
        let mut fresh: Vec<Arc<Plan>> = Vec::new();
        for d in crate::forbid::derivable_activators(
            s.kb,
            s.terms,
            s.ast,
            &compiled,
            seed.into_iter().map(crate::monotone::Rel::pos),
        ) {
            let Some(activator) = crate::compile::ground_fact(s.ast, s.terms, d.form) else {
                continue;
            };
            // An activator the state holds is no later debt: its plan is
            // among the state's own.
            if s.kb.contains(activator) {
                continue;
            }
            if let Some(known) = later.iter_mut().find(|l| l.activator == activator) {
                if !known.by.contains(&d.by) {
                    known.by.push(d.by);
                }
                continue;
            }
            let Some(rule) = program.obligations.get(d.obligation) else {
                continue;
            };
            // A plan that does not compile under this activator proposes
            // nothing: the node that derives the activator is where the
            // refusal belongs, and it is made there.
            let Ok(plan) = crate::compile::compile_rule(s.ast, s.terms, rule, Some(activator))
            else {
                continue;
            };
            let Some(relation) = crate::obligations::open_argument(&plan, s.terms) else {
                return Ok(declined(
                    s,
                    "a bare `(open)` names no relation to branch on",
                ));
            };
            later.push(LaterDebt {
                rule: d.obligation,
                activator,
                relation,
                by: vec![d.by],
                added: Vec::new(),
            });
            fresh.push(Arc::new(plan));
        }
        if fresh.is_empty() {
            break uncovered_relations(s, allowed, excluded, closed, &owed_rels);
        }
        // A later plan declines the call as one of the state's does: the
        // same two tests here, and C4's two on the next round, over every
        // plan together.
        if let Err(reason) = admit(s.terms, fresh, allowed, excluded, closed, &mut projs) {
            return Ok(declined(s, reason));
        }
    };

    // The owed instances, in report order: plan order, then match order.
    // What the state owes is counted and narrated; a later debt is not owed
    // here, and counts apart.
    let mut branches: Vec<Branch> = Vec::new();
    let mut report = RungReport {
        mode: Mode::Obligations,
        uncovered,
        later_debts: later.len() as u64,
        ..RungReport::default()
    };
    let mut scans = scans.into_iter();
    for (p, scan) in projs[..at_state].iter().zip(scans.by_ref()) {
        report.owed += scan.owed as u64;
        if let Some(skip) = p.skip {
            // Scoped out: counted, narrated and not branched on. A state whose
            // every debt lands here is the stuck one.
            //
            // The counter is per **instance** and the `hypskip` line is per
            // relation, which is the split `hypgen::raw_candidates` already
            // makes: the verdict is about a relation, the count is about each
            // thing that wanted one.
            for _ in 0..scan.owed {
                stats.skip(skip);
            }
            report.declined += scan.owed as u64;
            if scan.owed > 0 {
                hypskip(s, p.relation, skip);
            }
            continue;
        }
        report.branches += scan.owed as u64;
        for b in scan.branches {
            report.candidates += b.candidates.len() as u64;
            branches.push(b);
        }
    }
    let later_branches: Vec<Vec<Branch>> = scans.map(|scan| scan.branches).collect();
    report.later_candidates = later_branches
        .iter()
        .flatten()
        .map(|b| b.candidates.len() as u64)
        .sum();

    let before = stats.emitted;
    let mut broke = false;
    // Every instance's alternatives, in report order: the union, which is
    // what H holds. `apply_filters`' `seen_in_call` drops a candidate an
    // earlier instance already offered, so a later instance's share can come
    // back short or empty, which the union does not mind.
    for b in &branches {
        for &fact in &b.candidates {
            if emit(s, stats, fact).is_break() {
                broke = true;
                break;
            }
        }
        if broke {
            break;
        }
    }
    if report.owed > 0 && !broke && stats.emitted == before {
        // Owed, and nothing survived: every debt was scoped out, or every
        // candidate is already refuted. Either way the generator proposes
        // nothing and the caller would otherwise call this state complete.
        // Judged on what the state owes alone, before a later debt's
        // candidates are offered.
        report.mode = Mode::Stuck;
    }
    // The later debts' candidates, after everything the state owes, so a
    // call that reads none emits what it emitted and in that order.
    if !broke {
        'later: for (debt, branches) in later.iter_mut().zip(&later_branches) {
            for &fact in branches.iter().flat_map(|b| b.candidates.iter()) {
                let emitted = stats.emitted;
                if emit(s, stats, fact).is_break() {
                    break 'later;
                }
                if stats.emitted > emitted {
                    debt.added.push(fact);
                }
            }
        }
    }
    stats.later = later;
    narrate(s, &report);
    Ok(report)
}

/// The rung fell through — nothing emitted, the blind enumerator takes it.
fn declined(s: &mut Session<'_>, reason: &str) -> RungReport {
    let report = RungReport {
        mode: Mode::Declined,
        ..RungReport::default()
    };
    if s.events.on() {
        let mode = report.mode.as_str();
        s.events.emit("rung", |l| {
            l.str("mode", mode);
            l.str("reason", reason);
            l.num("owed", 0);
            l.num("branches", 0);
            l.num("declined", 0);
            l.num("candidates", 0);
            l.num("uncovered", 0);
        });
    }
    report
}

fn narrate(s: &mut Session<'_>, r: &RungReport) {
    if !s.events.on() {
        return;
    }
    let (mode, order) = (r.mode.as_str(), ORDER);
    let (owed, branches, declined, candidates, uncovered) =
        (r.owed, r.branches, r.declined, r.candidates, r.uncovered);
    let (later_debts, later_candidates) = (r.later_debts, r.later_candidates);
    s.events.emit("rung", |l| {
        l.str("mode", mode);
        l.str("reason", order);
        l.num("owed", owed as i64);
        l.num("branches", branches as i64);
        l.num("declined", declined as i64);
        l.num("candidates", candidates as i64);
        l.num("uncovered", uncovered as i64);
        // Optional, so `ein-events/1` stays: only where the call read a
        // later debt.
        if later_debts > 0 {
            l.num("later_debts", later_debts as i64);
            l.num("later_candidates", later_candidates as i64);
        }
    });
}

fn hypskip(s: &mut Session<'_>, rel: Symbol, skip: Skip) {
    if !(s.events.on() && s.events.verbose()) {
        return;
    }
    let relation = s.terms.sym(rel).to_string();
    let reason = skip.as_str();
    s.events.emit("hypskip", |l| {
        l.str("relation", &relation);
        l.str("reason", reason);
    });
}

/// How many hypothesis-eligible relations no obligation names — see
/// [`RungReport::uncovered`].
///
/// The same eligibility test [`crate::hypgen`]'s `relation_plan` applies, so a
/// relation the blind enumerator would skip is not counted as uncovered here
/// either: what is left is exactly the relations a hypothesis *could* be about
/// and no obligation constrains.
fn uncovered_relations(
    s: &Session<'_>,
    allowed: &Option<FxHashSet<Symbol>>,
    excluded: &FxHashSet<Symbol>,
    closed: &FxHashSet<Symbol>,
    owed: &FxHashSet<Symbol>,
) -> u64 {
    s.kb.program()
        .relations
        .values()
        .filter(|r| !r.signature.is_empty())
        .filter(|r| !owed.contains(&r.name))
        .filter(|r| scoping(r.name, allowed, excluded, closed).is_none())
        .count() as u64
}

/// The three pre-candidate relation skips, in the order [`crate::hypgen`]
/// applies them — so a relation both closed and un-whitelisted is counted the
/// same way here as there.
fn scoping(
    rel: Symbol,
    allowed: &Option<FxHashSet<Symbol>>,
    excluded: &FxHashSet<Symbol>,
    closed: &FxHashSet<Symbol>,
) -> Option<Skip> {
    if closed.contains(&rel) {
        Some(Skip::ClosedRelation)
    } else if allowed.as_ref().is_some_and(|a| !a.contains(&rel)) {
        Some(Skip::RelationNotWhitelisted)
    } else if excluded.contains(&rel) {
        Some(Skip::NoHypothesisRelation)
    } else {
        None
    }
}

/// Resolve `(open ?R)` and the witness step, per disjunct.
///
/// The loader has already refused the shapes that cannot resolve *for any*
/// activator ([`ein_ir`]'s four projection diagnostics). What is left to fail
/// here is activator-dependent: `?isa` and `?R` bound to the same relation
/// leaves two witness candidates where the AST had one, and the answer is to
/// decline rather than to pick.
fn project(plan: &Arc<Plan>, terms: &ein_core::Terms) -> Option<Proj> {
    let relation = crate::obligations::open_argument(plan, terms)?;
    let mut witness = Vec::with_capacity(plan.disjuncts.len());
    for d in plan.disjuncts.iter() {
        let mut found = None;
        for (gi, g) in plan.guards(d.guards).iter().enumerate() {
            for (at, step) in plan.steps(g.sub).iter().enumerate() {
                let Step::Rel(r) = step else { continue };
                if r.rel != relation || !bears_local(plan, g, r.slots) {
                    continue;
                }
                if found.is_some() {
                    return None; // two candidates — see the note above.
                }
                found = Some((gi, at));
            }
        }
        witness.push(found?);
    }
    Some(Proj {
        plan: Arc::clone(plan),
        relation,
        witness,
        skip: None,
    })
}

/// Does this step bear a variable the guard introduces rather than one the
/// parent projected in? That is the loader's witness test, in register form.
fn bears_local(plan: &Plan, guard: &NafGuard, slots: Span) -> bool {
    plan.slots(slots).iter().any(|s| match s {
        Slot::Reg(r) => guard.scope_of.get(*r as usize).copied().flatten().is_none(),
        _ => false,
    })
}

/// The first relation this obligation's guard **scans** that the rung also
/// proposes — C4's disqualifier, or `None` when the branch is stable.
fn scanned_relation_in(p: &Proj, owed: &FxHashSet<Symbol>) -> Option<Symbol> {
    for (d, &(gi, at)) in p.witness.iter().enumerate() {
        let guard = &p.plan.guards(p.plan.disjuncts[d].guards)[gi];
        if let Some(rel) = scan_rels(&p.plan, guard.sub, Some(at), owed) {
            return Some(rel);
        }
        // Every other guard of the disjunct narrows the same instance, so a
        // relation *it* scans can grow the instance set the same way.
        for (j, g) in p.plan.guards(p.plan.disjuncts[d].guards).iter().enumerate() {
            if j != gi
                && let Some(rel) = scan_rels(&p.plan, g.sub, None, owed)
            {
                return Some(rel);
            }
        }
    }
    // The rule's own positive premises decide which instances exist at all.
    for d in p.plan.disjuncts.iter() {
        if let Some(rel) = scan_rels(&p.plan, d.steps, None, owed) {
            return Some(rel);
        }
    }
    None
}

/// Relations read by `span`, nested queries included, minus the step at
/// `skip` — intersected with `owed`.
fn scan_rels(
    plan: &Plan,
    span: Span,
    skip: Option<usize>,
    owed: &FxHashSet<Symbol>,
) -> Option<Symbol> {
    for (i, step) in plan.steps(span).iter().enumerate() {
        if skip == Some(i) {
            continue;
        }
        match step {
            Step::Rel(r) if owed.contains(&r.rel) => return Some(r.rel),
            Step::Absent { sub } => {
                if let Some(rel) = scan_rels(plan, *sub, None, owed) {
                    return Some(rel);
                }
            }
            _ => {}
        }
    }
    None
}

/// The witness step's slots, resolved against one scan row.
///
/// `None` when a slot is neither a constant nor a bound register: skipping the
/// step binds nothing, so a witness slot no other premise reaches has no value
/// and there is no candidate to propose.
fn witness_fact(
    s: &mut Session<'_>,
    plan: &Plan,
    rel: Symbol,
    slots: Span,
    regs: &[Value],
) -> Option<FactId> {
    let args = witness_args(plan, slots, regs)?;
    // A lent table cannot number a proposition nobody has numbered yet; the
    // refusal ends this branch exactly as it ends a blind fill.
    s.terms.intern_fact(rel, &args).ok()
}

/// The witness step's arguments under one scan row — `None` when a slot is
/// neither a constant nor a bound register.
fn witness_args(plan: &Plan, slots: Span, regs: &[Value]) -> Option<Vec<Value>> {
    let mut args: Vec<Value> = Vec::with_capacity(slots.len());
    for slot in plan.slots(slots) {
        let v = match slot {
            Slot::Const(v) => *v,
            Slot::Reg(r) => *regs.get(*r as usize)?,
            _ => return None,
        };
        if v.is_unbound() {
            return None;
        }
        args.push(v);
    }
    Some(args)
}

// ── L5's table: each owed instance's alternatives (M1e2 S1e2.1c.4) ─────────

/// One obligation instance as the table and the node function both key it:
/// the rule, the activator's arguments and the match's bindings, which is
/// [`crate::obligations::Owed`]'s identity.
pub type InstanceKey = (Symbol, Box<[Value]>, Box<[(Symbol, Value)]>);

/// Every obligation instance root owes, each with the candidates that can
/// witness it — licence L5's alternatives — or why that set is not known.
/// Built once at root by `alternatives`, held to the oracle by
/// `ein-infer/tests/region.rs`'s
/// `l5_is_the_oracle_on_every_monotone_program_it_reaches`, and read by the
/// modes' obligation tree (`crate::tree`, T1e2.1c.4.2), whose every node
/// names the instances it owes by their place here.
#[derive(Clone, Debug, Default)]
pub struct Alternatives {
    /// In [`crate::obligations::tally`]'s report order.
    pub instances: Vec<Instance>,
    /// Each instance's place in [`Self::instances`].
    pub index: rustc_hash::FxHashMap<InstanceKey, u32>,
    /// The relations of the witnesses L9 left out, where the table is a
    /// route's and holds L9's alternatives ([`Self::with_l9`]); empty in the
    /// table as read. A node that holds a fact of one beyond root's that is
    /// no candidate breaks L9's premise ([`crate::unheld`]).
    pub unheld_relations: Box<[Symbol]>,
}

/// One instance root owes.
#[derive(Clone, Debug)]
pub struct Instance {
    pub key: InstanceKey,
    /// The candidates (bit indices, ascending) that can witness it, when L5's
    /// premise holds of it; why not, when it does not.
    pub alternatives: Result<Box<[u32]>, String>,
    /// Witnesses left out because root's fixpoint stores their negative: no
    /// alive fixpoint holds one.
    pub excluded: u32,
    /// **L9**: where L5 does not license the instance only because some of
    /// its witnesses are no candidate and not denied, every such witness is
    /// of a relation no node can hold a fact of beyond root's, and some
    /// witness is a candidate ([`crate::unheld`]), what L9 licenses. `None`
    /// where L5 licenses the instance, or refuses it for any other reason.
    pub unheld: Option<Unheld>,
}

/// What L9 licenses of one instance ([`Instance::unheld`]).
#[derive(Clone, Debug)]
pub struct Unheld {
    /// The candidates among its witnesses, bit indices ascending: its
    /// alternatives under L9.
    pub alternatives: Box<[u32]>,
    /// How many of its witnesses no node can hold.
    pub witnesses: u32,
    /// Their relation, the instance's witness relation.
    pub relation: Symbol,
}

impl Alternatives {
    /// Instances L5 licenses.
    pub fn licensed(&self) -> usize {
        self.instances
            .iter()
            .filter(|i| i.alternatives.is_ok())
            .count()
    }

    /// **The table a route branches on with L9**: each instance L9 licenses
    /// takes its alternatives, the rest stay as read, and
    /// [`Self::unheld_relations`] names the relations its witnesses left out,
    /// for the node function's check. The same table where L9 licenses
    /// nothing.
    pub fn with_l9(&self) -> Alternatives {
        let mut out = self.clone();
        let mut relations: Vec<Symbol> = Vec::new();
        for inst in &mut out.instances {
            if let (Err(_), Some(u)) = (&inst.alternatives, &inst.unheld) {
                inst.alternatives = Ok(u.alternatives.clone());
                if !relations.contains(&u.relation) {
                    relations.push(u.relation);
                }
            }
        }
        out.unheld_relations = relations.into_boxed_slice();
        out
    }

    /// Instance `i` is branched on by L9's licence: the table is a route's
    /// ([`Self::with_l9`]) and L5 did not license it.
    pub fn by_l9(&self, i: u32) -> bool {
        self.instances
            .get(i as usize)
            .is_some_and(|x| x.unheld.is_some() && x.alternatives.is_ok())
    }
}

/// **L5's premise, checked at load** — M1e2
/// [S1e2.1c.4](../../../../plans/m1e2_correctness/p1e2.1c_the_region_search/s1e2.1c.4_the_flags_at_scale.md),
/// [L5 and its argument](../../../../docs/ein/reasoning/kb-evolution/finding-r.md#4-a-monotone-program-three-properties-and-the-licences).
///
/// `kb` is root's fixpoint, on a program the static check calls monotone, and
/// `candidates` maps each candidate of `H` to its bit. For every obligation
/// instance root owes, in report order, this reads every fact that can
/// witness it, and the instance is **licensed** when each of them is a
/// candidate or has its negative stored at root. It is not licensed when:
///
/// - its rule is the bare `(open)`, which names no witness relation;
/// - its witness step does not resolve ([`project`]);
/// - its trigger, or any read of its guards beside the witness step, is of a
///   relation a commitment can change (`D`, [`crate::monotone`]): the set of
///   witnesses could then grow underneath the instance, or the instance be
///   paid some other way;
/// - a fact that can witness it is neither a candidate nor excluded at root.
///
/// **And what L9 licenses of an instance L5 refuses for the last reason
/// alone** ([`Instance::unheld`]): where every such witness is of a relation
/// no node can hold a fact of beyond root's ([`crate::unheld::Unwritten`])
/// and some witness is a candidate, its alternatives are the candidates
/// among its witnesses. L5's verdict and its reason are as they were; a
/// route takes L9's through [`Alternatives::with_l9`].
///
/// Every scan is root's own matcher, run once; nothing is interned and no
/// event is narrated. The guard's sub-plan is scanned with the witness step
/// skipped, as the obligations rung does ([`generate`]), which is the reuse
/// of the walk's library T1e2.1c.4.4 records.
pub(crate) fn alternatives(
    kb: &ein_core::Kb,
    terms: &mut ein_core::Terms,
    ast: &ein_ir::Ast,
    memo: &crate::compile::SharedMemo,
    monotonicity: &crate::monotone::Monotonicity,
    candidates: &rustc_hash::FxHashMap<FactId, u32>,
) -> Result<Alternatives, CompileError> {
    let mut out = Alternatives::default();
    if kb.program().obligations.is_empty() {
        return Ok(out);
    }
    let plans = crate::obligations::plans_for(kb, terms, ast, memo)?;
    let terms: &ein_core::Terms = terms;
    let unwritten = crate::unheld::Unwritten::of(kb, terms, memo);
    let mut matcher = Matcher::new();
    let mut guards = Matcher::new();
    for plan in plans {
        let premise: Result<Proj, String> = match crate::obligations::open_argument(&plan, terms) {
            None => Err("a bare `(open)` names no relation, so no fact witnesses it".into()),
            Some(_) => match project(&plan, terms) {
                None => Err("its witness step does not resolve".into()),
                Some(p) => static_premises(&p, monotonicity, terms).map(|()| p),
            },
        };
        for (disjunct, regs, bindings) in owed_by(kb, terms, ast, &plan, &mut matcher, &mut guards)
        {
            let (alternatives, excluded, unheld) = match &premise {
                Err(why) => (Err(why.clone()), 0, None),
                Ok(p) => {
                    match witnesses(
                        kb,
                        terms,
                        ast,
                        &mut guards,
                        p,
                        disjunct,
                        &regs,
                        candidates,
                        &unwritten,
                    ) {
                        Ok(read) => read.verdicts(),
                        Err(why) => (Err(why), 0, None),
                    }
                }
            };
            let key: InstanceKey = (plan.rule, plan.activator_args.clone(), bindings);
            out.index.insert(key.clone(), out.instances.len() as u32);
            out.instances.push(Instance {
                key,
                alternatives,
                excluded,
                unheld,
            });
        }
    }
    Ok(out)
}

/// One match of an obligation plan: the disjunct that reached it, its
/// registers and its bindings.
type Hit = (usize, Box<[Value]>, Box<[(Symbol, Value)]>);

/// The instances `plan` owes at `kb`, as `obligations::tally` finds them:
/// one per distinct bindings, judged by the first disjunct that reached
/// them, and owed where none of that disjunct's guards holds.
fn owed_by(
    kb: &ein_core::Kb,
    terms: &ein_core::Terms,
    ast: &ein_ir::Ast,
    plan: &Arc<Plan>,
    matcher: &mut Matcher,
    guards: &mut Matcher,
) -> Vec<Hit> {
    let mut hits: Vec<Hit> = Vec::new();
    let mut seen: FxHashSet<Box<[(Symbol, Value)]>> = FxHashSet::default();
    matcher.run(kb, terms, ast, plan, &mut |m| {
        let bindings: Box<[(Symbol, Value)]> = m.bindings().collect::<Vec<_>>().into_boxed_slice();
        if seen.insert(bindings.clone()) {
            hits.push((m.disjunct, m.regs().into(), bindings));
        }
        ControlFlow::Continue(())
    });
    hits.retain(|(disjunct, regs, _)| {
        let span = plan.disjuncts[*disjunct].guards;
        !plan
            .guards(span)
            .iter()
            .any(|g| guards.holds(kb, terms, ast, plan, g, regs))
    });
    hits
}

/// One fact that can witness an instance: its relation and its arguments,
/// as the witness step's scan finds them at root. It need not be a fact
/// anything has interned.
pub type WitnessRow = (Symbol, Box<[Value]>);

/// What can witness one instance root owes ([`witness_rows`]).
pub(crate) struct Witnesses {
    pub key: InstanceKey,
    /// L5's static premise of its rule: the trigger and every guard beside
    /// the witness step read no relation a commitment can change, or which
    /// one does, in [`alternatives`]' own words.
    pub premise: Result<(), String>,
    /// Every fact that can witness it, each once, in the scan's order; why
    /// none is known where the rule names no witness: the bare `(open)`, a
    /// witness step that does not resolve, a slot nothing binds.
    pub rows: Result<Vec<WitnessRow>, String>,
}

/// **Every fact that can witness each instance root owes**, in
/// [`alternatives`]' order and by its scan, whether or not L5 licenses the
/// instance: the rows [`alternatives`] asks of each whether it is a
/// candidate or denied, before it asks.
///
/// For the reader that asks what an unlicensed instance's witnesses are
/// ([`crate::structure`]); no route reads it.
pub(crate) fn witness_rows(
    kb: &ein_core::Kb,
    terms: &mut ein_core::Terms,
    ast: &ein_ir::Ast,
    memo: &crate::compile::SharedMemo,
    monotonicity: &crate::monotone::Monotonicity,
) -> Result<Vec<Witnesses>, CompileError> {
    let mut out = Vec::new();
    if kb.program().obligations.is_empty() {
        return Ok(out);
    }
    let plans = crate::obligations::plans_for(kb, terms, ast, memo)?;
    let terms: &ein_core::Terms = terms;
    let mut matcher = Matcher::new();
    let mut guards = Matcher::new();
    for plan in plans {
        let projected: Result<Proj, String> = match crate::obligations::open_argument(&plan, terms)
        {
            None => Err("a bare `(open)` names no relation, so no fact witnesses it".into()),
            Some(_) => project(&plan, terms).ok_or("its witness step does not resolve".into()),
        };
        let premise = match &projected {
            Ok(p) => static_premises(p, monotonicity, terms),
            Err(_) => Ok(()),
        };
        for (disjunct, regs, bindings) in owed_by(kb, terms, ast, &plan, &mut matcher, &mut guards)
        {
            let rows = projected.as_ref().map_err(Clone::clone).and_then(|p| {
                let (gi, at) = p.witness[disjunct];
                let guard = &p.plan.guards(p.plan.disjuncts[disjunct].guards)[gi];
                let Step::Rel(step) = p.plan.steps(guard.sub)[at] else {
                    return Err("its witness step is not a relation".into());
                };
                let mut scanned: Vec<Box<[Value]>> = Vec::new();
                guards.scan_without(kb, terms, ast, &p.plan, guard, &regs, at, &mut |sub| {
                    scanned.push(sub.into());
                    ControlFlow::Continue(())
                });
                let mut rows: Vec<WitnessRow> = Vec::with_capacity(scanned.len());
                for row in &scanned {
                    let Some(args) = witness_args(&p.plan, step.slots, row) else {
                        return Err(format!(
                            "a `{}` witness has a slot nothing binds",
                            terms.sym(step.rel)
                        ));
                    };
                    rows.push((step.rel, args.into_boxed_slice()));
                }
                // One fact once, in the scan's order.
                let mut seen: FxHashSet<WitnessRow> = FxHashSet::default();
                rows.retain(|r| seen.insert(r.clone()));
                Ok(rows)
            });
            out.push(Witnesses {
                key: (plan.rule, plan.activator_args.clone(), bindings),
                premise: premise.clone(),
                rows,
            });
        }
    }
    Ok(out)
}

/// L5's table on a **freshly loaded** program: root saturated, the
/// universe taken and the static check run as every route does
/// ([`crate::region`]). `None` on a program the static check flags, or one
/// that states no obligation. For the tests and the instruments that hold L5
/// to the oracle and count what it licenses.
pub fn alternatives_of(
    root: &mut ein_core::Kb,
    terms: &mut ein_core::Terms,
    ast: &ein_ir::Ast,
) -> Result<Option<Alternatives>, crate::error::SolveError> {
    let p = crate::region::prepare(root, terms, ast)?;
    if !p.monotonicity.is_monotone() || root.program().obligations.is_empty() {
        return Ok(None);
    }
    let candidates: rustc_hash::FxHashMap<FactId, u32> = p
        .candidates
        .iter()
        .enumerate()
        .map(|(i, c)| (c.fact, i as u32))
        .collect();
    Ok(Some(alternatives(
        root,
        terms,
        ast,
        &p.memo,
        &p.monotonicity,
        &candidates,
    )?))
}

/// The trigger and every guard read beside the witness step are static, or
/// which one is not.
fn static_premises(
    p: &Proj,
    monotonicity: &crate::monotone::Monotonicity,
    terms: &ein_core::Terms,
) -> Result<(), String> {
    if let Some(rel) = premise_reads(p, terms)
        .into_iter()
        .find(|&r| monotonicity.changes(r, terms))
    {
        return Err(format!(
            "it reads `{}` beside its witness, a relation a commitment can change",
            rel.label(terms)
        ));
    }
    Ok(())
}

/// Every relation an obligation reads beside its witness step, disjunct by
/// disjunct: its trigger, which says who owes, and its guards, which scan the
/// witness's domain. L5's premise ([`static_premises`]) and the rung's C4
/// over `D` ([`generate`]) read the same list.
fn premise_reads(p: &Proj, terms: &ein_core::Terms) -> Vec<crate::monotone::Rel> {
    let mut reads = Vec::new();
    for (d, &(gi, at)) in p.witness.iter().enumerate() {
        let disjunct = &p.plan.disjuncts[d];
        reads.extend(crate::monotone::reads_in(
            &p.plan,
            disjunct.steps,
            None,
            terms,
        ));
        for (j, g) in p.plan.guards(disjunct.guards).iter().enumerate() {
            let skip = (j == gi).then_some(at);
            reads.extend(crate::monotone::reads_in(&p.plan, g.sub, skip, terms));
        }
    }
    reads
}

/// One instance's witnesses at root, read ([`witnesses`]).
struct Read {
    /// The candidates among them, bit indices ascending.
    alts: Box<[u32]>,
    /// How many root denies.
    excluded: u32,
    /// L5's reason for the first that is neither, where one is.
    refused: Option<String>,
    /// How many of those no node can hold ([`crate::unheld`]), and their
    /// relation; `None` where one of them a node can hold, or the scan
    /// stopped before the last.
    unheld: Option<(u32, Symbol)>,
}

impl Read {
    /// L5's alternatives, the denied count, and what L9 licenses.
    fn verdicts(self) -> (Result<Box<[u32]>, String>, u32, Option<Unheld>) {
        match self.refused {
            None => (Ok(self.alts), self.excluded, None),
            Some(why) => {
                let unheld = self.unheld.map(|(witnesses, relation)| Unheld {
                    alternatives: self.alts,
                    witnesses,
                    relation,
                });
                (Err(why), 0, unheld)
            }
        }
    }
}

/// One instance's witnesses at root: its alternatives (candidate bits,
/// ascending), how many root excludes, and what L9 makes of the others; or
/// why no witness is known.
///
/// L5's reason is the first witness's that is neither a candidate nor
/// denied, as it was when the scan stopped there; the scan reads on past it
/// only while every such witness is one L9 can leave out.
#[allow(clippy::too_many_arguments)]
fn witnesses(
    kb: &ein_core::Kb,
    terms: &ein_core::Terms,
    ast: &ein_ir::Ast,
    matcher: &mut Matcher,
    p: &Proj,
    disjunct: usize,
    regs: &[Value],
    candidates: &rustc_hash::FxHashMap<FactId, u32>,
    unwritten: &crate::unheld::Unwritten,
) -> Result<Read, String> {
    let (gi, at) = p.witness[disjunct];
    let guard = &p.plan.guards(p.plan.disjuncts[disjunct].guards)[gi];
    let Step::Rel(step) = p.plan.steps(guard.sub)[at] else {
        return Err("its witness step is not a relation".into());
    };
    let mut rows: Vec<Box<[Value]>> = Vec::new();
    matcher.scan_without(kb, terms, ast, &p.plan, guard, regs, at, &mut |sub| {
        rows.push(sub.into());
        ControlFlow::Continue(())
    });
    let rel = terms.sym(step.rel).to_string();
    let mut alts: Vec<u32> = Vec::new();
    let mut excluded = 0u32;
    let mut refused: Option<String> = None;
    // L9 takes a witness of an unwritten relation; one it does not take ends
    // the scan, L5's reason being the first refusal's.
    let l9 = unwritten.is_unwritten(step.rel);
    let mut unheld = 0u32;
    for row in &rows {
        let Some(args) = witness_args(&p.plan, step.slots, row) else {
            if refused.is_some() {
                // L5 stopped before this row; L9 cannot read it.
                return Ok(Read {
                    alts: Box::default(),
                    excluded,
                    refused,
                    unheld: None,
                });
            }
            return Err(format!("a `{rel}` witness has a slot nothing binds"));
        };
        let fact = terms.probe_fact(step.rel, &args);
        if let Some(&b) = fact.and_then(|f| candidates.get(&f)) {
            alts.push(b);
            continue;
        }
        if fact.is_some_and(|f| kb.is_negated(f)) {
            excluded += 1;
            continue;
        }
        if refused.is_none() {
            refused = Some(match fact {
                None => format!("a `{rel}` fact can witness it that no candidate proposes"),
                Some(f) => format!(
                    "`{}` can witness it and is not a candidate",
                    crate::events::sexpr(terms, f)
                ),
            });
        }
        if !l9 {
            break;
        }
        unheld += 1;
    }
    alts.sort_unstable();
    alts.dedup();
    // An instance with no candidate among its witnesses is not L9's (the
    // module's § L9 in [`crate::unheld`]).
    let unheld = (refused.is_some() && l9 && !alts.is_empty()).then_some((unheld, step.rel));
    Ok(Read {
        alts: alts.into_boxed_slice(),
        excluded,
        refused,
        unheld,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::Events;
    use crate::hypgen::HypGenStats;
    use ein_core::{Kb, Terms};
    use ein_ir::{Ast, from_ir::load, parse};

    fn kb_of(src: &str) -> (Ast, Terms, Kb) {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        (ast, terms, kb)
    }

    /// Run the rung over `src` and report what it decided.
    ///
    /// `keep` stands in for the filter pipeline: a candidate that survives it
    /// bumps `emitted`, and whether *anything* did is what tells a discharged
    /// state from a stuck one.
    fn rung_of(src: &str) -> RungReport {
        rung_with(src, true)
    }

    fn rung_with(src: &str, keep: bool) -> RungReport {
        let (ast, mut terms, mut kb) = kb_of(src);
        let mut events = Events::off();
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: Default::default(),
        };
        let mut stats = HypGenStats::new();
        generate(
            &mut s,
            &None,
            &FxHashSet::default(),
            &FxHashSet::default(),
            &mut stats,
            &mut |_, stats, _| {
                stats.raw += 1;
                if keep {
                    stats.emitted += 1;
                }
                ControlFlow::Continue(())
            },
        )
        .expect("the rung runs")
    }

    /// The three-fact shape every case below varies: one relation, one
    /// membership scan, one thing that has no partner yet.
    const OWING: &str = "(relation is-a T T :why \"{?1} is a {?2}\")\n(relation likes P F :why \"{?1} likes {?2}\")\n\
                         (is-a Ann P)\n(is-a Soup F)\n";

    /// **The branch is the guard's scan**, and its size is the extent of the
    /// relation the guard walks — 2 candidates for two foods, not the 16
    /// arrows a blind pass over four names would build.
    #[test]
    fn the_branch_is_the_extent_the_guard_scans() {
        let r = rung_of(&format!(
            "{OWING}(is-a Stew F)\n\
             (rule owed (?R ?isa)\n\
             \x20 :match  (and (relation ?R ?A ?B) (?isa ?a ?A)\n\
             \x20               (absent (and (?isa ?b ?B) (?R ?a ?b))))\n\
             \x20 :assert (open ?R) :why \"{{?a}} is a {{?A}} with no {{?B}} under {{?R}} yet — one is still owed\"\n)\n\
             (owed likes is-a)\n"
        ));
        assert_eq!(r.mode, Mode::Obligations);
        assert_eq!((r.owed, r.branches, r.declined), (1, 1, 0));
        assert_eq!(r.candidates, 2, "Soup and Stew, and nothing else");
    }

    /// **A bare `(open)` declines the whole call.** It counts and reports —
    /// S1d.2.3 kept it for that — but it names no relation, so there is no
    /// branch to take and the rung may not claim to be exhaustive. Falling
    /// through to the blind enumerator is the only answer that stays complete.
    #[test]
    fn a_bare_open_has_no_branch_and_declines() {
        let r = rung_of(&format!(
            "{OWING}(rule owed (?R ?isa)\n\
             \x20 :match  (and (relation ?R ?A ?B) (?isa ?a ?A)\n\
             \x20               (absent (and (?isa ?b ?B) (?R ?a ?b))))\n\
             \x20 :assert (open) :why \"{{?a}} is a {{?A}} with no {{?B}} under {{?R}} yet — something is still owed\"\n)\n\
             (owed likes is-a)\n"
        ));
        assert_eq!(r.mode, Mode::Declined);
    }

    /// **Owed, and nothing survived the pipeline ⇒ stuck.** The same program
    /// as above with every candidate rejected — which is what a relation whose
    /// whole row already carries a stored negative looks like. The generator
    /// proposes nothing either way; the `stuck` line is the only thing that
    /// tells this state from a discharged one, and without it the caller calls
    /// both of them complete.
    #[test]
    fn owed_with_nothing_surviving_is_stuck() {
        let src = format!(
            "{OWING}(is-a Stew F)\n\
             (rule owed (?R ?isa)\n\
             \x20 :match  (and (relation ?R ?A ?B) (?isa ?a ?A)\n\
             \x20               (absent (and (?isa ?b ?B) (?R ?a ?b))))\n\
             \x20 :assert (open ?R) :why \"{{?a}} is a {{?A}} with no {{?B}} under {{?R}} yet — one is still owed\"\n)\n\
             (owed likes is-a)\n"
        );
        assert_eq!(rung_with(&src, false).mode, Mode::Stuck);
        assert_eq!(rung_with(&src, true).mode, Mode::Obligations);
    }

    /// **C4**: an obligation whose guard scans a relation the rung itself
    /// proposes is declined, and the decline takes the whole call. Here the
    /// same rule is activated twice — once owing `likes` and scanning `is-a`,
    /// once owing `is-a` — so the branch over foods could grow underneath
    /// itself.
    #[test]
    fn an_obligation_that_scans_what_the_rung_proposes_declines() {
        let r = rung_of(&format!(
            "{OWING}(relation sort T T :why \"{{?1}} is of sort {{?2}}\")\n(sort P S)\n(sort F S)\n\
             (rule owed (?R ?isa)\n\
             \x20 :match  (and (relation ?R ?A ?B) (?isa ?a ?A)\n\
             \x20               (absent (and (?isa ?b ?B) (?R ?a ?b))))\n\
             \x20 :assert (open ?R) :why \"{{?a}} is a {{?A}} with no {{?B}} under {{?R}} yet — one is still owed\"\n)\n\
             (owed likes is-a)\n(owed is-a sort)\n"
        ));
        assert_eq!(r.mode, Mode::Declined);
    }

    /// Root saturated with the memo the rung then reads, as
    /// `region::prepare` leaves it, and the rung's report with the `rung`
    /// line it narrated.
    fn rung_at_root(src: &str) -> (RungReport, String) {
        let (ast, mut terms, mut kb) = kb_of(src);
        let memo = crate::compile::SharedMemo::default();
        let mut off = Events::off();
        {
            let mut s = Session {
                kb: &mut kb,
                terms: &mut terms,
                ast: &ast,
                events: &mut off,
                memo: memo.clone(),
            };
            let mut sat = crate::saturator::Saturator::new(&mut s).expect("compiles");
            sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
        }
        let buffer = crate::events::Buffer::new();
        let mut events = Events::to(Box::new(buffer.clone()), crate::events::Level::Normal);
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo,
        };
        let mut stats = HypGenStats::new();
        let r = generate(
            &mut s,
            &None,
            &FxHashSet::default(),
            &FxHashSet::default(),
            &mut stats,
            &mut |_, stats, _| {
                stats.raw += 1;
                stats.emitted += 1;
                ControlFlow::Continue(())
            },
        )
        .expect("the rung runs");
        let rung = buffer
            .to_string_lossy()
            .lines()
            .find(|l| l.contains("\"rung\""))
            .expect("the rung narrates")
            .to_string();
        (r, rung)
    }

    /// W14's shape (`examples/ein-bugs/wrong-answers/oblig-fresh-name.ein`):
    /// Ann owes a liked food, and `special` makes Z a food once anyone likes
    /// anything.
    const OWES_FOOD: &str = "(relation person T :why \"{?1} is a person\")\n(relation food T :why \"{?1} is a food\")\n\
                             (relation likes T T :why \"{?1} likes {?2}\")\n(person Ann)\n(food Soup)\n\
                             (rule owes-food ()\n\
                             \x20 :match  (and (person ?a)\n\
                             \x20               (absent (and (food ?b) (likes ?a ?b))))\n\
                             \x20 :assert (open likes) :why \"{?a} is a person who likes no food yet — one is still owed\"\n)\n";

    /// **C4 over `D`** (M1e2 S1e2.2.15, W14; Q-M1e2.10 (a), ruled
    /// 2026-11-04): a guard that scans a relation a commitment can change
    /// declines the rung, and its `rung` line says so. Without `special`,
    /// `food` is static and the same obligation branches.
    #[test]
    fn an_obligation_that_scans_a_relation_a_commitment_derives_declines() {
        let (r, rung) = rung_at_root(&format!(
            "{OWES_FOOD}(rule special ()\n  :match  (likes ?p ?f)\n  :assert (food Z) :why \"{{?p}} likes {{?f}}, so Z is a food\")\n"
        ));
        assert_eq!(r.mode, Mode::Declined);
        assert!(
            rung.contains(
                "\"reason\": \"an obligation scans `food`, a relation a commitment can change\""
            ),
            "{rung}"
        );
        let (r, rung) = rung_at_root(OWES_FOOD);
        assert_eq!(r.mode, Mode::Obligations, "{rung}");
        assert!(rung.contains("\"reason\": \"rule-order\""), "{rung}");
    }
}
