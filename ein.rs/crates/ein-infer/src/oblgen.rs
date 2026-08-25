//! The obligations rung — hypotheses from what a state still owes.
//!
//! M1d [P1d.2](../../../../plans/m1d_satisfiability/p1d.2_obligations/README.md)
//! [S1d.2.5](../../../../plans/m1d_satisfiability/p1d.2_obligations/s1d.2.5_hypotheses_from_obligations.md).
//! [S1d.2.4](../../../../plans/m1d_satisfiability/p1d.2_obligations/s1d.2.4_obligations_in_the_saturator.md)
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
//! | an obligation rule, no hrule | **this module** |
//! | neither | the blind combinatorial enumerator |
//!
//! The middle row is about what the program **declares**, not about what this
//! state happens to owe: a discharged state under rung 2 proposes nothing and
//! is therefore complete, which is what *complete means discharged* has to
//! mean. Falling back to the blind enumerator whenever nothing was owed would
//! put exhaustion back in charge of completeness and make the rung
//! unreachable at exactly the states it exists to judge.
//!
//! ### The candidate set
//!
//! An obligation rule is `absent (and G(b) (R ā b))` — *no witness exists* —
//! and `(open ?R)` names `R`. So the facts that would discharge one instance
//! are exactly `{(R ā b) : G(b)}`, and the engine reads them by running the
//! guard's own sub-plan with the witness step **skipped**
//! ([`crate::match_::Matcher::scan_without`]). Nothing restates the domain:
//! the branch set is the guard, evaluated at this quiescence, which is
//! [`domain_contract.md`](../../../../plans/m1d_satisfiability/p1d.2_obligations/domain_contract.md)
//! C1 and C4 in one call.
//!
//! ### What it proposes, and why it is the union
//!
//! One obligation's candidate set is mutually exclusive and jointly
//! exhaustive: some `b` must witness it, so branching over them loses no
//! model. **This engine's traversal cannot take that branch on its own**,
//! though, and the reason is structural rather than incidental: the search is
//! a breadth-first lattice over *root's* `alive` set — layer `k` enters the
//! `k`-subsets of one fixed set — so a model needing an arrow the chosen
//! obligation never proposed is unreachable, at every depth. The rung
//! therefore proposes the **union** over the accepted instances, of which any
//! single obligation's set is a subset; the per-instance structure survives in
//! the walk order, the decline rule and the report. Choosing *one* is a
//! depth-first move, and
//! [S1d.2.5's record](../../../../plans/m1d_satisfiability/p1d.2_obligations/hypotheses_from_obligations.md)
//! is where that is measured rather than asserted.
//!
//! ### When it declines
//!
//! The domain contract's C4: a branch is jointly exhaustive only while the
//! candidate set cannot grow underneath it, so an obligation whose guard scans
//! a relation the rung itself proposes is **declined** — and a declined
//! obligation takes the whole call back to the blind generator, which is what
//! keeps completeness a property of the ladder rather than of the puzzle.
//! Scoping is the other half: `:no-hypothesis`, `:hypothesis-relations` and
//! `(__closed__ R)` bind here exactly as they bind the blind enumerator, and
//! a state owing only relations they exclude is **stuck** — reported, never
//! silently complete.

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
    /// No obligation rule is loaded — the blind enumerator, as before.
    #[default]
    Blind,
    /// A `(hrule …)` overrides the ladder. Never narrated — rung 1 emits the
    /// stream it emitted before M1d, to the byte.
    Hrules,
    /// The rung generated: every owed obligation's candidates, deduped.
    Obligations,
    /// Owed, and nothing may be branched on — the ladder's dead end.
    Stuck,
    /// An obligation was declined (C4, or a projection that will not resolve),
    /// so the call fell through to the blind enumerator.
    Declined,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Blind => "blind",
            Mode::Hrules => "hrules",
            Mode::Obligations => "obligations",
            Mode::Stuck => "stuck",
            Mode::Declined => "declined",
        }
    }
}

/// What one generation call did with the obligations it found.
///
/// Zero everywhere for a program that declares none, which is 114 of the 156
/// `.ein` files under `examples/`, `tests/` and `stdlib/` that load — the
/// ladder's first question is `obligations.is_empty()` and it is answered
/// before anything here runs.
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
    /// [S1d.2.5]: `plans/m1d_satisfiability/p1d.2_obligations/s1d.2.5_hypotheses_from_obligations.md`
    pub uncovered: u64,
}

/// The order the owed instances are walked in — T1d.2.5.2's measured pair.
///
/// It is a *walk* order, not a branch choice: the emitted set is the union
/// either way, `seen_in_call` dedups it, and `apriori::order_candidates` sorts
/// every layer canonically — so no order imposed here reaches the traversal.
/// **Measured inert**, on both fixtures and on every counter
/// ([the record §4]); it is kept because it is the interface a depth-first
/// traversal would need on day one, and it costs one `sort_by_key` over the
/// instance list.
///
/// [the record §4]: `plans/m1d_satisfiability/p1d.2_obligations/hypotheses_from_obligations.md`
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Choice {
    /// Report order — priority, then load order, then activator, then match.
    /// The control.
    RuleOrder,
    /// Smallest candidate set first. Costs a full enumeration before the first
    /// candidate is emitted, which is what the control does not.
    FailFirst,
    /// Branch on none of them: the rung declines every call and the blind
    /// enumerator runs. **The engine as it was before S1d.2.5**, kept as the
    /// measurement's control arm and as the one-line proof that the ladder is
    /// the only thing between the two numbers.
    Off,
}

impl Choice {
    /// `EIN_OBLIGATION_CHOICE`, the measurement lever.
    ///
    /// Not a [`ein_core::SolverConfig`] field on purpose: the config is
    /// rendered into the KB-shape digest, so a knob whose two settings are
    /// being *compared* would re-bless every shape golden in the corpus to
    /// record a default nobody has chosen yet.
    pub fn from_env() -> Choice {
        match std::env::var("EIN_OBLIGATION_CHOICE").as_deref() {
            Ok("fail-first") => Choice::FailFirst,
            Ok("off") => Choice::Off,
            _ => Choice::RuleOrder,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Choice::RuleOrder => "rule-order",
            Choice::FailFirst => "fail-first",
            Choice::Off => "off",
        }
    }
}

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
/// A `Vec<Vec<FactId>>` by another name, and the name is the point — the
/// per-instance grouping is what [`Choice`] orders and what a depth-first
/// traversal would branch on one of.
struct Branch {
    candidates: Vec<FactId>,
}

/// Generate from the obligations, or say why not.
///
/// Returns the rung it took. `Mode::Declined` means **nothing was emitted and
/// the caller must fall through to the blind enumerator**; every other mode
/// means this call is the whole generation.
pub(crate) fn generate(
    s: &mut Session<'_>,
    allowed: &Option<FxHashSet<Symbol>>,
    excluded: &FxHashSet<Symbol>,
    closed: &FxHashSet<Symbol>,
    choice: Choice,
    stats: &mut HypGenStats,
    emit: &mut dyn FnMut(&mut Session<'_>, &mut HypGenStats, FactId) -> ControlFlow<()>,
) -> Result<RungReport, CompileError> {
    if choice == Choice::Off {
        return Ok(declined(s, "EIN_OBLIGATION_CHOICE=off"));
    }
    let memo = s.memo.clone();
    let plans = crate::obligations::plans_for(s.kb, s.terms, s.ast, &memo)?;
    let mut projs = Vec::with_capacity(plans.len());
    for plan in plans {
        // The bare `(open)` counts and reports and names nothing, so there is
        // no relation to branch on and no guard to read a domain out of
        // (S1d.2.3 item 4's table: `(open)` counts, `(open ?R)` also
        // attributes and branches). One of them in the program is enough to
        // stop the rung claiming exhaustiveness, so the call falls through
        // rather than silently owing something nobody proposes for.
        if crate::obligations::open_argument(&plan, s.terms).is_none() {
            return Ok(declined(
                s,
                "a bare `(open)` names no relation to branch on",
            ));
        }
        let Some(mut proj) = project(&plan, s.terms) else {
            // A projection that will not resolve here is one the loader could
            // not have refused: it depends on the activator's substitution.
            // Declining is the conservative half of the same rule C4 uses.
            return Ok(declined(s, "a projection did not resolve per activator"));
        };
        proj.skip = scoping(proj.relation, allowed, excluded, closed);
        projs.push(proj);
    }
    // C4 — the candidate set must not be able to grow underneath the branch,
    // so no accepted obligation may scan a relation this rung proposes.
    let owed_rels: FxHashSet<Symbol> = projs.iter().map(|p| p.relation).collect();
    let uncovered = uncovered_relations(s, allowed, excluded, closed, &owed_rels);
    for p in &projs {
        if let Some(rel) = scanned_relation_in(p, &owed_rels) {
            let rel = s.terms.sym(rel).to_string();
            let reason = format!("an obligation scans `{rel}`, which the rung itself proposes");
            return Ok(declined(s, &reason));
        }
    }

    // The owed instances, in report order: plan order, then match order.
    let mut branches: Vec<Branch> = Vec::new();
    let mut report = RungReport {
        mode: Mode::Obligations,
        uncovered,
        ..RungReport::default()
    };
    let mut matcher = Matcher::new();
    let mut guards = Matcher::new();
    let mut owed: Vec<(usize, Box<[Value]>)> = Vec::new();
    for p in projs.iter() {
        owed.clear();
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
        report.owed += owed.len() as u64;
        if let Some(skip) = p.skip {
            // Scoped out: counted, narrated and not branched on. A state whose
            // every debt lands here is the stuck one.
            //
            // The counter is per **instance** and the `hypskip` line is per
            // relation, which is the split `hypgen::raw_candidates` already
            // makes: the verdict is about a relation, the count is about each
            // thing that wanted one.
            for _ in 0..owed.len() {
                stats.skip(skip);
            }
            report.declined += owed.len() as u64;
            if !owed.is_empty() {
                hypskip(s, p.relation, skip);
            }
            continue;
        }
        report.branches += owed.len() as u64;
        for (disjunct, regs) in owed.drain(..) {
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
            report.candidates += candidates.len() as u64;
            branches.push(Branch { candidates });
        }
    }

    if choice == Choice::FailFirst {
        // Stable, so instances of equal width keep report order — the control
        // is the tie-breaker rather than a second variable.
        branches.sort_by_key(|b| b.candidates.len());
    }
    let before = stats.emitted;
    let mut broke = false;
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
        report.mode = Mode::Stuck;
    }
    narrate(s, &report, choice);
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

fn narrate(s: &mut Session<'_>, r: &RungReport, choice: Choice) {
    if !s.events.on() {
        return;
    }
    let (mode, order) = (r.mode.as_str(), choice.as_str());
    let (owed, branches, declined, candidates, uncovered) =
        (r.owed, r.branches, r.declined, r.candidates, r.uncovered);
    s.events.emit("rung", |l| {
        l.str("mode", mode);
        l.str("reason", order);
        l.num("owed", owed as i64);
        l.num("branches", branches as i64);
        l.num("declined", declined as i64);
        l.num("candidates", candidates as i64);
        l.num("uncovered", uncovered as i64);
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
    // A lent table cannot number a proposition nobody has numbered yet; the
    // refusal ends this branch exactly as it ends a blind fill.
    s.terms.intern_fact(rel, &args).ok()
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
            Choice::RuleOrder,
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
    const OWING: &str = "(relation is-a T T)\n(relation likes P F)\n\
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
             \x20 :assert (open ?R)\n  :priority 500)\n\
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
             \x20 :assert (open)\n  :priority 500)\n\
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
             \x20 :assert (open ?R)\n  :priority 500)\n\
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
            "{OWING}(relation sort T T)\n(sort P S)\n(sort F S)\n\
             (rule owed (?R ?isa)\n\
             \x20 :match  (and (relation ?R ?A ?B) (?isa ?a ?A)\n\
             \x20               (absent (and (?isa ?b ?B) (?R ?a ?b))))\n\
             \x20 :assert (open ?R)\n  :priority 500)\n\
             (owed likes is-a)\n(owed is-a sort)\n"
        ));
        assert_eq!(r.mode, Mode::Declined);
    }
}
