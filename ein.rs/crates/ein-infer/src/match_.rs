//! The register matcher — S1a.3.2.
//!
//! ein.py's `match.py` is a recursive generator over a `dict` of bindings, and
//! [design/05](../../../../plans/m1a_rust/design/05_matcher.md) §1 attributes
//! **46 %** of an exhaustive `zebra2` solve's self time to it. Three costs, all
//! structural: `_bind_arg` returns `{**bindings, name: arg}` — an allocation
//! and a rehash of the whole binding set *per bound variable, per candidate
//! fact, at every level of the join*; slot dispatch is six `isinstance` calls
//! on a good path, and 31.9 M of the run's calls come from here; and every
//! step allocates a generator frame plus a fresh `rest` tuple.
//!
//! This is the same enumeration with none of that: variables live in a
//! register file, binding is `regs[r] = v` plus a push onto a **trail**,
//! unification is a `u32` compare, and a match is handed to a callback instead
//! of being materialised.
//!
//! ### What must not change
//!
//! The *result sequence*. The matcher's signature is the firing order, and the
//! firing order is the trace, so this module owes three orders exactly:
//!
//! - **matches**, in the order the nested loops produce them — which is why
//!   join reordering is rejected outright ([design/05](../../../../plans/m1a_rust/design/05_matcher.md) §6);
//! - **bindings**, in the order the matcher first bound each variable, because
//!   `Provenance.bindings` is CPython dict-insertion order and it is printed
//!   in the trace. The [`Trail`] is that order, by construction;
//! - **premises**, in plan-step order — including through `run_seeded`, whose
//!   contract is that provenance from a semi-naive seed is *identical* to
//!   provenance from a full run. ein.py splices the seeded fact back into its
//!   step's position; writing `prems[ordinal]` gets there without the splice.
//!
//! ### Recursion, and why the cursor loop is not here
//!
//! design/05 §3 sketches an explicit cursor loop over a step index. What that
//! buys in Python is the *removal of generator frames and per-step tuple
//! rebuilds*; in Rust a recursive walk over a step slice allocates neither —
//! the candidate iterator is a stack value and the step slice is borrowed. So
//! the recursion stays, the allocation claim is tested rather than argued
//! (`tests/match_alloc.rs`), and flattening the loop is left to
//! [P1a.6](../../../../plans/m1a_rust/p1a.6_performance/README.md) to *measure*
//! rather than assume. The register file, the trail and the callback — the
//! parts that were about the data model — are all here.

use std::ops::ControlFlow;

use ein_core::{FactId, Kb, SlotKey, Symbol, Tag, Terms, Value};
use ein_ir::Ast;

use crate::plan::{
    Disjunct, GuardArgKind, NafGuard, Plan, ProbeSrc, Reg, RelStep, Slot, Span, Step,
};
use crate::predicates::Pred;

/// One match, handed to the caller's callback.
///
/// Borrowed, not owned: the saturator's `_enqueue_binding` runs *inside* the
/// callback, so a match that turns out to be a duplicate costs nothing beyond
/// the key hash. ein.py gets the same effect by consuming the generator
/// lazily; the callback just removes the generator.
pub struct Match<'a> {
    pub plan: &'a Plan,
    /// Which `(or …)` branch produced this — the index into `plan.disjuncts`.
    pub disjunct: usize,
    regs: &'a [Value],
    trail: &'a [Reg],
    premises: &'a [FactId],
}

impl<'a> Match<'a> {
    /// The bound variables **in bind order** — the order `Provenance.bindings`
    /// records and the trace prints. Seed bindings come first, because the
    /// activator binding is pushed onto the trail before any step runs, which
    /// is where `dict(plan.bindings_seed)` puts them.
    pub fn bindings(&self) -> impl Iterator<Item = (Symbol, Value)> + '_ {
        self.trail
            .iter()
            .map(|&r| (self.plan.reg_names[r as usize], self.regs[r as usize]))
    }

    /// The facts the `Scan` / `Join` steps consumed, in step order.
    pub fn premises(&self) -> &'a [FactId] {
        self.premises
    }

    /// This disjunct's lifted `(absent …)` guards. Pairing them with the match
    /// is what closes the S1.21.8 D5 gap structurally: a caller cannot see a
    /// match without seeing which guards must hold for it.
    pub fn guards(&self) -> &'a [NafGuard] {
        self.plan.guards(self.plan.disjuncts[self.disjunct].guards)
    }

    pub fn value(&self, reg: Reg) -> Value {
        self.regs[reg as usize]
    }

    pub fn regs(&self) -> &'a [Value] {
        self.regs
    }

    /// The bound registers, in bind order — the raw form of
    /// [`Match::bindings`], for a caller that snapshots rather than renders.
    pub fn trail(&self) -> &'a [Reg] {
        self.trail
    }
}

/// What a callback returns: `Break` stops the enumeration.
pub type Emit<'c> = &'c mut dyn FnMut(&Match<'_>) -> ControlFlow<()>;

/// Everything a walk reads and never writes. Bundled because it is constant
/// for the whole enumeration and threading four references through six
/// signatures says nothing the name does not.
#[derive(Clone, Copy)]
struct Ctx<'a> {
    kb: &'a Kb,
    terms: &'a Terms,
    ast: &'a Ast,
    plan: &'a Plan,
}

/// Where a walk is: the step sequence, the step, the premise slot, and which
/// step (if any) a semi-naive seed already bound.
#[derive(Clone, Copy)]
struct Walk {
    steps: Span,
    i: usize,
    ordinal: usize,
    skip: Option<usize>,
}

impl Walk {
    fn at(self, i: usize, ordinal: usize) -> Walk {
        Walk { i, ordinal, ..self }
    }

    /// A nested `(absent …)` query: its own step sequence, no seed, and the
    /// enclosing walk's premise slot — see [`crate::plan::Disjunct::n_slots`].
    fn nested(self, steps: Span) -> Walk {
        Walk {
            steps,
            i: 0,
            ordinal: self.ordinal,
            skip: None,
        }
    }
}

/// The register file, trail and premise slots — reused across runs.
///
/// One allocation apiece, grown to fit the widest plan the caller has run and
/// never shrunk, so the inner loop's only writes are indexed stores.
/// The widest premise the ground fast path resolves on the stack. Every
/// relation in the corpus is far inside it; a wider one takes the scan.
const GROUND_ARITY: usize = 8;

/// What [`Matcher::ground_args`] found: one exact tuple, a tuple that cannot
/// exist, or a premise still open to search.
enum GroundArgs {
    Closed([Value; GROUND_ARITY], usize),
    Absent,
    Open,
}

#[derive(Default, Debug)]
pub struct Matcher {
    regs: Vec<Value>,
    trail: Vec<Reg>,
    prems: Vec<FactId>,
    /// What the emit path reports, set by whichever entry point is running.
    n_premises: usize,
    disjunct: usize,
    /// Is the walk inside a guard sub-plan? Instrumentation only — it splits
    /// [`ein_core::Counters::scan_bucket_guard`] and its three siblings off
    /// the whole-run totals, so "does the boundary reach the index" is a
    /// measured question rather than an argued one.
    in_guard: bool,
    /// Test-only: take every candidate from the relation's whole extent, as
    /// if no probe existed. The narrowing's whole claim is that this changes
    /// nothing observable, and `narrowing_never_changes_the_match_sequence`
    /// is that claim run against randomised insertion orders.
    #[cfg(test)]
    no_probe: bool,
}

impl Matcher {
    pub fn new() -> Matcher {
        Matcher::default()
    }

    // ── Entry points ───────────────────────────────────────────────

    /// `match.run` — every match of every disjunct, seeds merged in.
    ///
    /// A rule whose `:match` is a top-level `(or …)` carries its extra
    /// disjuncts in the same plan, and each runs from a fresh seed, so every
    /// caller sees all of them without any rule-split (S1.8.A13).
    pub fn run(&mut self, kb: &Kb, terms: &Terms, ast: &Ast, plan: &Plan, f: Emit<'_>) {
        let c = Ctx {
            kb,
            terms,
            ast,
            plan,
        };
        for i in 0..plan.disjuncts.len() {
            if self.run_disjunct(c, i, None, f).is_break() {
                return;
            }
        }
    }

    /// Every match of one disjunct — the unit `run_guarded`'s callers pair
    /// with that disjunct's guards.
    pub fn run_one(
        &mut self,
        kb: &Kb,
        terms: &Terms,
        ast: &Ast,
        plan: &Plan,
        disjunct: usize,
        f: Emit<'_>,
    ) {
        let c = Ctx {
            kb,
            terms,
            ast,
            plan,
        };
        let _ = self.run_disjunct(c, disjunct, None, f);
    }

    /// `match.run_guarded` — identical, and the callback can read
    /// [`Match::guards`]. Kept as a named entry point because ein.py has one:
    /// the distinction is the *caller's* contract, not the matcher's.
    pub fn run_guarded(&mut self, kb: &Kb, terms: &Terms, ast: &Ast, plan: &Plan, f: Emit<'_>) {
        self.run(kb, terms, ast, plan, f)
    }

    /// `match.run_seeded` — the semi-naive delta match (S1.8.B2v D5).
    ///
    /// Every match in which the newly-derived `fact` plays a *positive*
    /// premise. For each top-level `Scan`/`Join` on the fact's relation, that
    /// step is bound to the fact and the **remaining** steps run — seeding at
    /// *each* such step, since the fact may play either role in
    /// `(R ?a ?b) ∧ (R ?b ?c)`.
    ///
    /// The caller restricts this to plans where the relation is a positive
    /// premise; a plan that reads it only inside a guard must full-`run`,
    /// because seeding cannot observe an absent flipping.
    pub fn run_seeded(
        &mut self,
        kb: &Kb,
        terms: &Terms,
        ast: &Ast,
        plan: &Plan,
        fact: FactId,
        f: Emit<'_>,
    ) {
        let c = Ctx {
            kb,
            terms,
            ast,
            plan,
        };
        let rel = terms.facts.rel(fact);
        for i in 0..plan.disjuncts.len() {
            let d = plan.disjuncts[i];
            for (at, step) in plan.steps(d.steps).iter().enumerate() {
                let Step::Rel(r) = step else { continue };
                if r.rel != rel {
                    continue;
                }
                if self.run_disjunct(c, i, Some((at, fact)), f).is_break() {
                    return;
                }
            }
        }
    }

    /// One disjunct, seeded at **one** named step — the lookahead's probe.
    ///
    /// ein.py builds a throwaway `JoinPlan(steps=rest)` and full-`run`s it;
    /// this is the same enumeration through the seed machinery `run_seeded`
    /// already has, which matters for more than tidiness: the injected fact
    /// need not be in the KB, and here it is not — the lookahead is asking
    /// what *would* follow from adding it.
    ///
    /// The caller chooses `(disjunct, at)` because it has already filtered
    /// both — a disjunct whose guards cannot be judged pre-fork is skipped
    /// entirely ([`crate::lookahead`]).
    #[allow(clippy::too_many_arguments)]
    pub fn run_seeded_at(
        &mut self,
        kb: &Kb,
        terms: &Terms,
        ast: &Ast,
        plan: &Plan,
        disjunct: usize,
        at: usize,
        fact: FactId,
        f: Emit<'_>,
    ) {
        let c = Ctx {
            kb,
            terms,
            ast,
            plan,
        };
        let _ = self.run_disjunct(c, disjunct, Some((at, fact)), f);
    }

    /// `match._seed_steps` over a guard's sub-plan — does **`fact`** create a
    /// match the KB alone does not have?
    ///
    /// [`Matcher::holds`] asks the guard of the world as it stands; this asks
    /// it of the world plus one fact, which together are exactly "no match in
    /// `kb` with `fact` added" for the monotone guards the lookahead has not
    /// already excluded. Seeds at *each* sub-step on the fact's relation,
    /// since the fact may play either role in `(R ?a ?b) ∧ (R ?b ?c)`.
    #[allow(clippy::too_many_arguments)]
    pub fn holds_seeded(
        &mut self,
        kb: &Kb,
        terms: &Terms,
        ast: &Ast,
        plan: &Plan,
        guard: &NafGuard,
        parent: &[Value],
        fact: FactId,
    ) -> bool {
        let rel = terms.facts.rel(fact);
        let c = Ctx {
            kb,
            terms,
            ast,
            plan,
        };
        for (at, step) in plan.steps(guard.sub).iter().enumerate() {
            let Step::Rel(r) = *step else { continue };
            if r.rel != rel {
                continue;
            }
            self.reset(guard.n_regs, guard.n_slots as usize, 0, 0);
            // The scope projection, exactly as `holds` does it — and for the
            // same reason untrailed: a guard produces no provenance.
            for (reg, from) in guard.scope_of.iter().enumerate() {
                if let Some(p) = from {
                    self.regs[reg] = parent[*p as usize];
                }
            }
            if !self.unify(terms, plan, r.slots, terms.facts.args(fact)) {
                continue;
            }
            let mut found = false;
            let w = Walk {
                steps: guard.sub,
                i: 0,
                ordinal: 0,
                skip: Some(at),
            };
            let _ = self.walk(c, w, &mut |_| {
                found = true;
                ControlFlow::Break(())
            });
            if found {
                return true;
            }
        }
        false
    }

    /// `match.run_seeded_guarded` — see [`Matcher::run_guarded`].
    pub fn run_seeded_guarded(
        &mut self,
        kb: &Kb,
        terms: &Terms,
        ast: &Ast,
        plan: &Plan,
        fact: FactId,
        f: Emit<'_>,
    ) {
        self.run_seeded(kb, terms, ast, plan, fact, f)
    }

    /// `match.run_steps` — the boundary's `World.holds` query.
    ///
    /// Runs a guard's sub-plan in **its own** register space, seeded from the
    /// parent's registers through the guard's scope projection. Returns `true`
    /// iff some match exists, which is all `holds` asks.
    pub fn holds(
        &mut self,
        kb: &Kb,
        terms: &Terms,
        ast: &Ast,
        plan: &Plan,
        guard: &NafGuard,
        parent: &[Value],
    ) -> bool {
        let mut found = false;
        self.reset(guard.n_regs, guard.n_slots as usize, 0, 0);
        // `world.project(bindings, scope)`, resolved at compile time to
        // register pairs. Projected registers are deliberately **not** trailed:
        // the trail is the bind order provenance reads, a guard produces no
        // provenance, and an untrailed register cannot be unwound by mistake.
        for (r, from) in guard.scope_of.iter().enumerate() {
            if let Some(p) = from {
                self.regs[r] = parent[*p as usize];
            }
        }
        let c = Ctx {
            kb,
            terms,
            ast,
            plan,
        };
        let w = Walk {
            steps: guard.sub,
            i: 0,
            ordinal: 0,
            skip: None,
        };
        self.in_guard = true;
        let _ = self.walk(c, w, &mut |_| {
            found = true;
            ControlFlow::Break(())
        });
        self.in_guard = false;
        found
    }

    // ── The driver ─────────────────────────────────────────────────

    fn run_disjunct(
        &mut self,
        c: Ctx<'_>,
        index: usize,
        seed_at: Option<(usize, FactId)>,
        f: Emit<'_>,
    ) -> ControlFlow<()> {
        let (terms, plan) = (c.terms, c.plan);
        let d: Disjunct = plan.disjuncts[index];
        self.reset(
            plan.n_regs,
            d.n_slots as usize,
            d.n_premises as usize,
            index,
        );
        // `dict(plan.bindings_seed)` — inserted first, so they are the first
        // entries of every `Provenance.bindings` this plan produces.
        for (r, (_, value)) in plan.seed.iter().enumerate() {
            self.regs[r] = *value;
            self.trail.push(r as Reg);
        }
        let skip = match seed_at {
            None => None,
            Some((at, fact)) => {
                let Step::Rel(step) = plan.steps(d.steps)[at] else {
                    return ControlFlow::Continue(());
                };
                let mark = self.trail.len();
                let args = terms.facts.args(fact);
                if !self.unify(terms, plan, step.slots, args) {
                    // The one new fact does not fit that premise; ein.py's
                    // `_bind_args` returning None, and the same non-answer.
                    self.unwind(mark);
                    return ControlFlow::Continue(());
                }
                // `prem_pos` — the seeded fact sits at *its* step's position,
                // so provenance from a seed reads identically to provenance
                // from a full run.
                self.prems[rel_ordinal(plan, d.steps, at)] = fact;
                Some(at)
            }
        };
        self.walk(
            c,
            Walk {
                steps: d.steps,
                i: 0,
                ordinal: 0,
                skip,
            },
            f,
        )
    }

    /// Walk `steps` from index `i`, with `ordinal` premise slots already
    /// written. `skip` is the seeded step, which is bound before the walk and
    /// still counts towards the ordinal.
    fn walk(&mut self, c: Ctx<'_>, w: Walk, f: Emit<'_>) -> ControlFlow<()> {
        ein_core::counters::bump(|c| c.walk += 1);
        let (terms, ast, plan) = (c.terms, c.ast, c.plan);
        let (steps, i, ordinal, skip) = (w.steps, w.i, w.ordinal, w.skip);
        if i == steps.len() {
            let m = Match {
                plan,
                disjunct: self.disjunct,
                regs: &self.regs,
                trail: &self.trail,
                premises: &self.prems[..self.n_premises],
            };
            return f(&m);
        }
        if skip == Some(i) {
            return self.walk(c, w.at(i + 1, ordinal + 1), f);
        }
        match plan.steps(steps)[i] {
            Step::Rel(step) => self.rel_step(c, w, step, f),
            Step::Guard { pred, args } => {
                if self.guard_holds(terms, ast, plan, pred, args) {
                    self.walk(c, w.at(i + 1, ordinal), f)
                } else {
                    ControlFlow::Continue(())
                }
            }
            Step::Absent { sub } => {
                // Negation as failure *inside a negative query* — a `forall`'s
                // `(absent (and G (absent B)))`. It runs in this same binding
                // environment (ein.py hands it the same dict) and its own
                // bindings do not escape, which the trail mark enforces.
                let mark = self.trail.len();
                let mut any = false;
                let _ = self.walk(c, w.nested(sub), &mut |_| {
                    any = true;
                    ControlFlow::Break(())
                });
                self.unwind(mark);
                if any {
                    ControlFlow::Continue(())
                } else {
                    self.walk(c, w.at(i + 1, ordinal), f)
                }
            }
        }
    }

    fn rel_step(&mut self, c: Ctx<'_>, w: Walk, step: RelStep, f: Emit<'_>) -> ControlFlow<()> {
        let plan = c.plan;
        // A premise every one of whose slots is already bound is not a search.
        // It asks whether **one** proposition is in the KB, and the fact store
        // interns propositions, so at most one fact can answer it and the
        // store can name that fact in a hash lookup. The scan below reaches
        // the same answer by fetching the whole participation bucket and
        // unifying every fact in it — 9.96 facts per guard premise on
        // `zebra -e`, of which 71.8 % of premises are ground
        // ([T1a.6.12.3](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.12_boundary_and_snapshot.md#task-t1a6123--what-the-guard-queries-scan)).
        //
        // Identical by construction, not by test: the pattern denotes one
        // value tuple, `unify` accepts a fact iff its arguments *are* that
        // tuple, and no two facts share one. So the candidate sequence is the
        // same one-or-none sequence, with the same `FactId` in `prems`.
        #[cfg(test)]
        let ground = if self.no_probe { GroundArgs::Open } else { self.ground_args(c, plan, step.slots) };
        #[cfg(not(test))]
        let ground = self.ground_args(c, plan, step.slots);
        match ground {
            GroundArgs::Closed(args, n) => {
                let g = self.in_guard as u64;
                ein_core::counters::bump(|c| {
                    c.scan_ground += 1;
                    c.scan_ground_guard += g;
                });
                let hit = c
                    .terms
                    .facts
                    .probe(step.rel, &args[..n])
                    .filter(|&id| c.kb.contains(id));
                return match hit {
                    Some(fact) => {
                        ein_core::counters::bump(|c| {
                            c.candidates += 1;
                            c.cand_ground += 1;
                        });
                        self.prems[w.ordinal] = fact;
                        self.walk(c, w.at(w.i + 1, w.ordinal + 1), f)
                    }
                    None => ControlFlow::Continue(()),
                };
            }
            // A nested pattern naming a proposition nobody ever interned:
            // no fact holds it as an argument, so nothing can match.
            GroundArgs::Absent => {
                let g = self.in_guard as u64;
                ein_core::counters::bump(|c| {
                    c.scan_ground += 1;
                    c.scan_ground_guard += g;
                });
                return ControlFlow::Continue(());
            }
            GroundArgs::Open => {}
        }
        let key = self.probe(plan, step);
        let g = self.in_guard as u64;
        match key {
            Some(key) => {
                ein_core::counters::bump(|c| {
                    c.scan_bucket += 1;
                    c.scan_bucket_guard += g;
                });
                for fact in c.kb.facts_with(key) {
                    ein_core::counters::bump(|c| {
                        c.cand_bucket += 1;
                        c.cand_bucket_guard += g;
                    });
                    self.try_candidate(c, w, step, fact, f)?
                }
            }
            None => {
                ein_core::counters::bump(|c| {
                    c.scan_extent += 1;
                    c.scan_extent_guard += g;
                });
                for fact in c.kb.facts_of(step.rel) {
                    ein_core::counters::bump(|c| {
                        c.cand_extent += 1;
                        c.cand_extent_guard += g;
                    });
                    self.try_candidate(c, w, step, fact, f)?
                }
            }
        }
        ControlFlow::Continue(())
    }

    fn try_candidate(
        &mut self,
        c: Ctx<'_>,
        w: Walk,
        step: RelStep,
        fact: FactId,
        f: Emit<'_>,
    ) -> ControlFlow<()> {
        ein_core::counters::bump(|c| c.candidates += 1);
        let (terms, plan) = (c.terms, c.plan);
        let mark = self.trail.len();
        // **Every** slot is re-checked, even the one the probe narrowed on.
        // That re-check is what makes the narrowing behaviour-preserving
        // rather than merely equivalent-if-the-index-agrees, and it is what
        // ein.py's `_bind_args` does; design/05 §6 lists dropping it as an
        // optimisation to measure before taking, not to assume.
        let ok = self.unify(terms, plan, step.slots, terms.facts.args(fact));
        let out = if ok {
            self.prems[w.ordinal] = fact;
            self.walk(c, w.at(w.i + 1, w.ordinal + 1), f)
        } else {
            ControlFlow::Continue(())
        };
        self.unwind(mark);
        out
    }

    // ── Unification ────────────────────────────────────────────────

    fn unify(&mut self, terms: &Terms, plan: &Plan, slots: Span, args: &[Value]) -> bool {
        ein_core::counters::bump(|c| c.unify += 1);
        // Both sides as slices, then one zip: 50.2 M calls on an exhaustive
        // `zebra` average **1.02 slots each** (T1a.6.2.2), so what this loop
        // costs is its prologue and its bounds checks, not its compares.
        let slots = &plan.slots[slots.range()];
        if slots.len() != args.len() {
            return false;
        }
        for (&slot, &arg) in slots.iter().zip(args) {
            if !self.unify_slot(terms, plan, slot, arg) {
                return false;
            }
        }
        true
    }

    fn unify_slot(&mut self, terms: &Terms, plan: &Plan, slot: Slot, arg: Value) -> bool {
        ein_core::counters::bump(|c| c.unify_slot += 1);
        match slot {
            Slot::Const(v) => v == arg,
            Slot::Reg(r) => {
                let cur = self.regs[r as usize];
                if cur.is_unbound() {
                    self.regs[r as usize] = arg;
                    self.trail.push(r);
                    true
                } else {
                    cur == arg
                }
            }
            Slot::Nested { rel, slots } => match arg.as_fact() {
                None => false,
                Some(id) => {
                    // One row read, then the relation, and the arguments only
                    // if it matched: **79 %** of an exhaustive `zebra2`'s
                    // candidates die on this comparison (T1a.6.2.2) and never
                    // look at the arguments, while an exhaustive `zebra`'s
                    // almost all pass it and want them immediately. Reading
                    // the row once serves both; either half alone loses one of
                    // the two puzzles.
                    let row = terms.facts.row(id);
                    if row.rel != rel {
                        ein_core::counters::bump(|c| c.nested_rel_reject += 1);
                        return false;
                    }
                    ein_core::counters::bump(|c| c.nested_rel_hit += 1);
                    self.unify(terms, plan, slots, terms.facts.args_of(row))
                }
            },
            // `slot == arg` in ein.py, where `slot` is an IR node and `arg` is
            // a `str` / `int` / `Fact`. No IR node is equal to any of them.
            Slot::Opaque(_) => false,
        }
    }

    // ── Guards ─────────────────────────────────────────────────────

    /// Evaluate `eq` / `neq` against the current bindings.
    ///
    /// The arguments are raw IR nodes resolved at *runtime* — see
    /// [`crate::plan::GuardArg`] — and an unbound variable resolves to Python's
    /// `None`, which is equal to another `None` and to nothing else. That is
    /// not a curiosity: `(eq ?a ?b)` on two unbound variables passes in ein.py.
    fn guard_holds(&self, terms: &Terms, ast: &Ast, plan: &Plan, pred: Pred, args: Span) -> bool {
        let args = plan.guard_args(args);
        assert!(
            args.len() >= 2,
            "`{}` needs two arguments; ein.py raises IndexError here",
            pred.as_str()
        );
        let equal = self.resolved_eq(ast, &args[0], &args[1]);
        let _ = terms;
        match pred {
            Pred::Eq => equal,
            Pred::Neq => !equal,
        }
    }

    fn resolved_eq(&self, ast: &Ast, a: &crate::plan::GuardArg, b: &crate::plan::GuardArg) -> bool {
        use GuardArgKind::*;
        let value = |g: &crate::plan::GuardArg| match g.kind {
            Const(v) => Some(v),
            Reg(r) => {
                let v = self.regs[r as usize];
                (!v.is_unbound()).then_some(v)
            }
            Node => None,
        };
        match (a.kind, b.kind) {
            // Two raw nodes compare structurally, as two frozen dataclasses do.
            (Node, Node) => ast.eq_nodes(a.node, b.node),
            // A node is never equal to a `str` / `int` / `Fact`…
            (Node, _) | (_, Node) => false,
            _ => match (value(a), value(b)) {
                (Some(x), Some(y)) => x == y,
                // …and `None == None` is how two unbound variables compare.
                (None, None) => true,
                _ => false,
            },
        }
    }

    // ── Candidates ─────────────────────────────────────────────────

    /// The participation-index bucket this step narrows on, or `None` for the
    /// full extent — [`crate::plan::Probe`].
    /// Resolve a premise's slots to the one argument tuple they denote, if
    /// they denote one — the test [`Matcher::rel_step`]'s fast path rests on.
    fn ground_args(&self, c: Ctx<'_>, plan: &Plan, slots: Span) -> GroundArgs {
        let ss = plan.slots(slots);
        if ss.is_empty() || ss.len() > GROUND_ARITY {
            return GroundArgs::Open;
        }
        let mut out = [Value::UNBOUND; GROUND_ARITY];
        for (i, &slot) in ss.iter().enumerate() {
            out[i] = match slot {
                Slot::Const(v) => v,
                Slot::Reg(r) => {
                    let v = self.regs[r as usize];
                    if v.is_unbound() {
                        return GroundArgs::Open;
                    }
                    v
                }
                Slot::Nested { rel, slots } => match self.ground_args(c, plan, slots) {
                    GroundArgs::Closed(inner, n) => match c.terms.facts.probe(rel, &inner[..n]) {
                        Some(id) => Value::fact(id),
                        None => return GroundArgs::Absent,
                    },
                    GroundArgs::Absent => return GroundArgs::Absent,
                    GroundArgs::Open => return GroundArgs::Open,
                },
                // Never unifies with anything, so it is not "ground" in any
                // useful sense — leave it to the scan, which will reject every
                // candidate exactly as it does today.
                Slot::Opaque(_) => return GroundArgs::Open,
            };
        }
        GroundArgs::Closed(out, ss.len())
    }

    fn probe(&self, plan: &Plan, step: RelStep) -> Option<SlotKey> {
        let key = self.probe_key(plan, step);
        debug_assert_eq!(
            key,
            candidates_scan(plan, &self.regs, step),
            "the compiled probe list disagrees with a live `_candidates` scan"
        );
        // The differential harness of T1a.6.3.1 runs the same plans with the
        // narrowing off, and "off" has to mean *only* off: the mirror above
        // is checked either way.
        #[cfg(test)]
        if self.no_probe {
            return None;
        }
        key
    }

    fn probe_key(&self, plan: &Plan, step: RelStep) -> Option<SlotKey> {
        for p in plan.probes(step.probe) {
            let value = match p.src {
                ProbeSrc::Const(v) => v,
                ProbeSrc::Reg(r) => {
                    let v = self.regs[r as usize];
                    // Unbound: nothing to key on yet. A nested fact: the index
                    // does not hold one, so ein.py keeps scanning.
                    if v.is_unbound() || v.tag() == Tag::Fact {
                        continue;
                    }
                    v
                }
            };
            return Some(SlotKey {
                rel: step.rel,
                slot: p.slot,
                inner: p.inner,
                value,
            });
        }
        None
    }

    // ── Register file ──────────────────────────────────────────────

    fn reset(&mut self, n_regs: u16, n_slots: usize, n_premises: usize, disjunct: usize) {
        // Every entry point — `run_disjunct`, `holds`, `holds_seeded` — comes
        // through here exactly once, which is why `plan_run` is counted here
        // rather than at the seven public functions that would each need it.
        ein_core::counters::bump(|c| c.plan_run += 1);
        self.n_premises = n_premises;
        self.disjunct = disjunct;
        let n = n_regs as usize;
        if self.regs.len() < n {
            self.regs.resize(n, Value::UNBOUND);
        }
        self.regs[..n].fill(Value::UNBOUND);
        self.trail.clear();
        self.trail.reserve(n);
        // Written before read at every position the emit path looks at, so the
        // filler value is arbitrary; it exists to make the vector indexable.
        self.prems.clear();
        self.prems.resize(n_slots.max(1), FactId(0));
    }

    fn unwind(&mut self, mark: usize) {
        while self.trail.len() > mark {
            let r = self.trail.pop().expect("above the mark");
            self.regs[r as usize] = Value::UNBOUND;
        }
    }
}

/// The premise-slot index of the `Rel` step at `at` — the count of `Rel` steps
/// before it, which is `_seed_steps`' `prem_pos`.
fn rel_ordinal(plan: &Plan, steps: Span, at: usize) -> usize {
    plan.steps(steps)[..at]
        .iter()
        .filter(|s| matches!(s, Step::Rel(_)))
        .count()
}

/// `match._candidates`, evaluated from the raw slots — the reference the
/// compiled probe list is checked against in debug builds.
fn candidates_scan(plan: &Plan, regs: &[Value], step: RelStep) -> Option<SlotKey> {
    let usable = |slot: &Slot| -> Option<Value> {
        match *slot {
            Slot::Const(v) => Some(v),
            Slot::Reg(r) => {
                let v = regs[r as usize];
                // Unbound: nothing to key on. A nested fact: the index does
                // not hold one as a value of its own.
                (!v.is_unbound() && v.tag() != Tag::Fact).then_some(v)
            }
            Slot::Nested { .. } | Slot::Opaque(_) => None,
        }
    };
    for (slot, i) in plan.slots(step.slots).iter().zip(0u16..) {
        if let Slot::Nested { slots, .. } = *slot {
            // T1a.6.3.0's second level, in the same left-to-right order the
            // compiled list is built in.
            for (deep, j) in plan.slots(slots).iter().zip(0u16..) {
                if let Some(value) = usable(deep) {
                    return Some(SlotKey {
                        rel: step.rel,
                        slot: i,
                        inner: j,
                        value,
                    });
                }
            }
            continue;
        }
        if let Some(value) = usable(slot) {
            return Some(SlotKey::direct(step.rel, i, value));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_core::Kb;
    use ein_ir::{from_ir::load, parse};

    /// Every rule of a source, compiled — the differential harness runs all of
    /// them rather than the first.
    fn setup_all(src: &str) -> (Ast, Terms, Kb, Vec<Plan>) {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let rules: Vec<_> = kb.program().rules.values().cloned().collect();
        let plans = rules
            .iter()
            .map(|r| crate::compile_rule(&ast, &mut terms, r, None).expect("compiles"))
            .collect();
        (ast, terms, kb, plans)
    }

    /// The emitted match sequence of every plan, rendered — bindings in bind
    /// order and premises in step order, which together are what provenance
    /// and the trace are built from.
    ///
    /// `later` is added through the *incremental* index (`index_fact`), which
    /// is the path a fork writes through; the source's own facts go through
    /// the batch one (`rebuild_layer`). Both have to key the same things.
    fn match_sequence(src: &str, later: &[(&str, &str, &str)], no_probe: bool) -> Vec<String> {
        let (ast, mut terms, mut kb, plans) = setup_all(src);
        for (kind, x, y) in later {
            let a = terms.value_text(x).expect("room");
            let b = terms.value_text(y).expect("room");
            let likes = terms.intern_text("likes").expect("room");
            let (rel, args) = match *kind {
                "pal" => (terms.intern_text("pal").expect("room"), vec![a, b]),
                _ => {
                    let inner = terms.value_fact(likes, &[a, b]).expect("room");
                    (terms.kernel.not, vec![inner])
                }
            };
            kb.add_and_index_fact(&mut terms, rel, &args, None)
                .expect("room");
        }
        let (terms, kb) = (terms, kb);
        let mut matcher = Matcher::new();
        matcher.no_probe = no_probe;
        let mut out: Vec<String> = Vec::new();
        for plan in &plans {
            matcher.run(&kb, &terms, &ast, plan, &mut |m| {
                let binds: Vec<String> = m
                    .bindings()
                    .map(|(n, v)| format!("{}={}", terms.sym(n), terms.display(v)))
                    .collect();
                out.push(format!(
                    "{} [{}] {:?}",
                    terms.sym(plan.rule),
                    binds.join(" "),
                    m.premises()
                ));
                ControlFlow::Continue(())
            });
        }
        out
    }

    /// T1a.6.3.1 — the narrowing argument, executed rather than argued.
    ///
    /// A probe replaces "every fact of the relation" with "every fact in a
    /// bucket", and that is sound iff two things hold: the bucket **contains
    /// every fact that would have unified** (the index keys exactly what the
    /// probe asks about, and the matcher re-checks every slot anyway), and it
    /// **yields them in extent order** (buckets are appended in insertion
    /// order and read oldest layer first, like the extent itself). Then the
    /// subsequence that survives unification is the same subsequence, so the
    /// emitted matches — bindings in bind order, premises in step order — are
    /// identical and only the number of *rejected* candidates differs.
    ///
    /// The interesting case is T1a.6.3.0's second level: a `(not (likes …))`
    /// premise used to have no key at all and now has one per inner slot. The
    /// insertion order is randomised because "in extent order" is the half of
    /// the argument that a single fixed order cannot test.
    #[test]
    fn narrowing_never_changes_the_match_sequence() {
        let decls = "(relation likes T T)\n(relation pal T T)\n\
                     (rule r ()\n  \
                     :match (and (pal ?a ?b) (not (likes ?b ?c)))\n  \
                     :assert (likes ?a ?c))\n\
                     (rule s ()\n  \
                     :match (and (not (likes ?x ?y)) (pal ?y ?z))\n  \
                     :assert (pal ?x ?z))\n";
        let names = ["Alice", "Bob", "Carol", "Dave"];
        let mut facts: Vec<String> = Vec::new();
        for (i, a) in names.iter().enumerate() {
            for (j, b) in names.iter().enumerate() {
                if i != j {
                    facts.push(format!("(pal {a} {b})"));
                }
                if (i + j) % 3 != 0 {
                    facts.push(format!("(not (likes {a} {b}))"));
                }
            }
        }
        // Some of the facts arrive through the incremental index instead, the
        // way a fork's derivations do.
        let later: &[(&str, &str, &str)] = &[
            ("pal", "Erin", "Alice"),
            ("not", "Alice", "Erin"),
            ("not", "Erin", "Bob"),
            ("pal", "Bob", "Erin"),
        ];
        for seed in 0..16 {
            let mut shuffled = facts.clone();
            crate::mt19937::Mt19937::seeded(seed).shuffle(&mut shuffled);
            let src = format!("{decls}{}\n", shuffled.join("\n"));
            for extra in [&[][..], later] {
                let narrowed = match_sequence(&src, extra, false);
                let scanned = match_sequence(&src, extra, true);
                assert_eq!(
                    narrowed, scanned,
                    "seed {seed}: narrowing changed the match sequence"
                );
                assert!(
                    !narrowed.is_empty(),
                    "seed {seed}: the fixture still matches something"
                );
            }
        }
    }

    fn setup(src: &str) -> (Ast, Terms, Kb, Plan) {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let rule = kb.program().rules.values().next().expect("a rule").clone();
        let plan = crate::compile_rule(&ast, &mut terms, &rule, None).expect("compiles");
        (ast, terms, kb, plan)
    }

    /// Which x's guard passes, for the two premise orders — the S1.21.8 scope
    /// contract, executed rather than inspected.
    ///
    /// `(and (a ?x) (absent (block ?x)))` asks "is there no block for *this*
    /// x?", so it passes for the unblocked one and fails for the blocked one.
    /// `(and (absent (block ?x)) (a ?x))` asks "is there no block at all?", so
    /// with one block in the KB it fails for **every** x. Lifting a guard to
    /// the boundary must not turn the first into the second.
    fn passing(src: &str) -> Vec<String> {
        let (ast, terms, kb, plan) = setup(src);
        let mut matcher = Matcher::new();
        let mut envs: Vec<(Vec<Value>, String)> = Vec::new();
        matcher.run(&kb, &terms, &ast, &plan, &mut |m| {
            let x = m
                .bindings()
                .find(|(n, _)| terms.sym(*n) == "x")
                .expect("?x is bound");
            envs.push((m.regs().to_vec(), terms.display(x.1)));
            ControlFlow::Continue(())
        });
        let guard = &plan.guards[0];
        envs.into_iter()
            .filter(|(regs, _)| !matcher.holds(&kb, &terms, &ast, &plan, guard, regs))
            .map(|(_, name)| name)
            .collect()
    }

    #[test]
    fn a_lifted_guard_keeps_asking_its_own_question() {
        let facts = "(a free)\n(a taken)\n(block taken)\n";
        let decls = "(relation a Thing)\n(relation block Thing)\n";
        assert_eq!(
            passing(&format!(
                "{decls}(rule r ()\n  :match (and (a ?x) (absent (block ?x)))\n  \
                 :assert (a ?x))\n{facts}"
            )),
            ["free"],
            "scope {{x}} — the query is about this x"
        );
        assert_eq!(
            passing(&format!(
                "{decls}(rule r ()\n  :match (and (absent (block ?x)) (a ?x))\n  \
                 :assert (a ?x))\n{facts}"
            )),
            Vec::<String>::new(),
            "empty scope — the query is about every block, and one exists"
        );
    }

    /// A `neq` on two variables neither premise bound resolves both to Python's
    /// `None`, and `None == None`, so the guard *fails*. Surprising, ported.
    #[test]
    fn two_unbound_guard_arguments_compare_equal() {
        let (ast, terms, kb, plan) = setup(
            "(relation a Thing)\n\
             (rule r ()\n  :match (and (a ?x) (neq ?p ?q))\n  :assert (a ?x))\n\
             (a one)\n",
        );
        let mut matcher = Matcher::new();
        let mut n = 0;
        matcher.run(&kb, &terms, &ast, &plan, &mut |_| {
            n += 1;
            ControlFlow::Continue(())
        });
        assert_eq!(n, 0, "`neq` on two `None`s is false, so nothing matches");
    }

    /// `run_seeded` puts the seeded fact at *its* step's position, so a
    /// semi-naive match's provenance reads identically to a full run's — the
    /// contract `_seed_steps`' splice exists for.
    #[test]
    fn a_seeded_match_reports_the_same_premise_order() {
        let src = "(relation edge A B)\n(relation path A B)\n\
                   (rule walk ()\n  :match (and (edge ?a ?b) (edge ?b ?c))\n  \
                   :assert (path ?a ?c))\n\
                   (edge n1 n2)\n(edge n2 n3)\n";
        let (ast, terms, kb, plan) = setup(src);
        let mut matcher = Matcher::new();
        let mut full: Vec<Vec<FactId>> = Vec::new();
        matcher.run(&kb, &terms, &ast, &plan, &mut |m| {
            full.push(m.premises().to_vec());
            ControlFlow::Continue(())
        });
        assert_eq!(full.len(), 1);

        // Seeding at each fact in turn finds the same match twice — once with
        // the fact in the first premise position, once in the second — and
        // both report the premises in plan order.
        let mut seeded: Vec<Vec<FactId>> = Vec::new();
        for fact in kb.facts().collect::<Vec<_>>() {
            matcher.run_seeded(&kb, &terms, &ast, &plan, fact, &mut |m| {
                seeded.push(m.premises().to_vec());
                ControlFlow::Continue(())
            });
        }
        assert_eq!(seeded.len(), 2, "each edge seeds the join at its own step");
        assert!(
            seeded.iter().all(|p| *p == full[0]),
            "a seeded match reordered its premises: {seeded:?} vs {:?}",
            full[0]
        );
    }
}
