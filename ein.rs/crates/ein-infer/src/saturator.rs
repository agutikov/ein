//! The saturation driver — S1.21.8's **two-phase** loop, ported.
//!
//! ```text
//! step():
//!   loop {
//!       if let Some(f) = closure_step()  { return f }     // purely positive
//!       if admit_from_boundary() == 0    { return None }  // one NAF admission
//!   }
//! ```
//!
//! - **Closure (inner).** Purely positive plans fire to quiescence; no
//!   negation is consulted. Every `(absent …)` was lifted out of its disjunct
//!   at compile time, so `plan.steps` is a positive program.
//! - **Boundary (outer).** At quiescence every parked NAF-guarded candidate is
//!   judged against that fixpoint, and **at most one is admitted per round**.
//!   That is a soundness requirement, not a throttle: admitting a batch lets
//!   one admission invalidate another's guard *after* its verdict was taken,
//!   which on `p ← absent q; q ← absent p` derives both — and `{p, q}` is not a
//!   model of that program under any reading.
//!
//! Everything else ports verbatim: priority bands (advisory since S1.21.8, and
//! still the order), FIFO within a band via the monotone tiebreaker, `_seen`
//! keyed on `(binding_key, guards)`, and `naf_dropped` structurally 0 — there
//! is no enqueue/fire window left for a guard to go stale in.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::ControlFlow;
use std::sync::Arc;

use ein_core::{FactId, Kb, NafArg, NafRef, Prov, Symbol, Terms, Value};
use ein_ir::Ast;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::compile::{CompileError, SharedMemo};
use crate::engine::Engine;
use crate::events::{self, Events};
use crate::firing::{BindingKey, Env, FireError, Firing, build_fact, fire};
use crate::match_::Matcher;
use crate::plan::{NafGuard, Plan, Reg, Slot, Span, Step};

/// Rules with no `:priority` sit between the eliminate band (300) and the
/// hypothesis band (900) — well-defined, and rarely hit because every shipping
/// rule declares one.
pub const DEFAULT_PRIORITY: i64 = 1000;

/// The kernel-native symmetric mirror trigger. A relation marked
/// `(__symmetric__ R)` has its extension closed under arg-swap directly by the
/// saturator — no plan, no matcher.
pub const SYMMETRIC: &str = "__symmetric__";

/// Everything a saturation step reads and writes outside the saturator.
pub struct Session<'a> {
    pub kb: &'a mut Kb,
    pub terms: &'a mut Terms,
    pub ast: &'a Ast,
    pub events: &'a mut Events,
    /// The run's compiled plans — design/06 § Win A.
    ///
    /// Every engine built from this session compiles into it: the saturator's,
    /// a `lookahead` probe's, a `closed` marking's. A caller that hands the
    /// *same* handle to a root saturation and to each of its forks compiles
    /// each `(rule, activator)` pair once for the whole run instead of once per
    /// fork; `SharedMemo::default()` is a private one, which is what a one-shot
    /// caller wants.
    pub memo: SharedMemo,
}

#[derive(Clone, Debug)]
pub enum SaturateError {
    Compile(CompileError),
    Fire(FireError),
    /// `SaturatorStepLimitError` — the runaway-debugging budget ran out.
    StepLimit(String),
}

impl std::fmt::Display for SaturateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaturateError::Compile(e) => write!(f, "{e}"),
            SaturateError::Fire(e) => write!(f, "{e}"),
            SaturateError::StepLimit(m) => f.write_str(m),
        }
    }
}

impl From<CompileError> for SaturateError {
    fn from(e: CompileError) -> Self {
        SaturateError::Compile(e)
    }
}

impl From<FireError> for SaturateError {
    fn from(e: FireError) -> Self {
        SaturateError::Fire(e)
    }
}

/// One queued or parked candidate — ein.py's 6-tuple, with the payload in a
/// side arena so the heap compares two integers
/// ([design/02](../../../../plans/m1a_rust/design/02_determinism_and_order.md) §2).
#[derive(Clone)]
struct Entry {
    plan: usize,
    disjunct: usize,
    regs: Box<[Value]>,
    trail: Box<[Reg]>,
    premises: Box<[FactId]>,
    /// This candidate's identity, built once at enqueue.
    ///
    /// ein.py recomputes `_binding_key` at every dequeue and at every boundary
    /// round — a fresh `frozenset` per parked candidate per round, which on a
    /// zebra root is ~60 000 of them. The answer cannot change (the registers
    /// are a snapshot), so the only thing recomputing buys is the allocation.
    key: BindingKey,
    /// The interned identity of this candidate's guards — the boundary's
    /// invalidation unit.
    ///
    /// Every relation a candidate watches is a relation its *guard set*
    /// watches, and [`Disjunct::guard_key`](crate::plan::Disjunct::guard_key)
    /// encodes the watched symbols themselves, so two candidates that share an
    /// id watch exactly the same relations and go stale at exactly the same
    /// moment. That is what lets one epoch per guard set replace one stamp per
    /// candidate — [T1a.6.12.1a](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.12_boundary_and_snapshot.md#task-t1a6121--visit-what-changed-not-everything).
    guard_set: GuardSetId,
}

/// What each guard set watches — the structural half of the boundary's
/// invalidation, and the half that a fork inherits unchanged.
///
/// `rels` is every relation any guard set reads, `watched[spans[g]]` the
/// indices into it for guard set `g`. Flat, because a fork clones the
/// saturator's state once per entering and a `Vec<Box<[u32]>>` would be one
/// allocation per guard set per fork; behind an `Arc`, because compiling a
/// *new* guard set inside a fork is rare enough to pay for a copy when it
/// happens.
#[derive(Clone, Default)]
struct WatchTables {
    rels: Vec<Symbol>,
    watched: Vec<u32>,
    spans: Vec<(u32, u32)>,
}

/// `(priority, tiebreaker, entry)`. The tiebreaker is unique and monotone, so
/// the comparison never reaches the third field — exactly as ein.py's heap
/// never compares two `JoinPlan`s.
type Ranked = Reverse<(i64, u64, u32)>;

/// An interned guard tuple — the `_seen` key's second half.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct GuardSetId(u32);

pub struct Saturator {
    pub engine: Engine,
    matcher: Matcher,
    entries: Vec<Entry>,
    queue: BinaryHeap<Ranked>,
    /// The parked candidates, in (priority, FIFO) order — S1a.3.4 T7.
    ///
    /// A `BTreeSet` rather than a heap because a boundary round **reads** the
    /// whole set in order and removes at most one entry from it. ein.py pops
    /// every candidate and re-pushes the rejects, which on a zebra root is
    /// ~60 000 heap operations across the run for ~1 000 actual judgements.
    /// Ordered iteration gives the same sequence — so "the first candidate
    /// whose guards pass" is the same candidate — without touching the ones
    /// it skips.
    parked: std::collections::BTreeSet<(i64, u64, u32)>,
    seen: FxHashSet<(BindingKey, GuardSetId)>,
    guard_sets: FxHashMap<Box<[u32]>, GuardSetId>,
    tiebreaker: u64,
    needs_enqueue: bool,
    delta: Option<Vec<FactId>>,
    matched_plans: Vec<bool>,
    pos_index: FxHashMap<Symbol, Vec<usize>>,
    index_n: usize,
    /// The boundary's clock: one tick per round, and what a judgement is
    /// stamped with — [`Saturator::refresh_epochs`].
    epoch: u32,
    /// `judged_at[e]` is the epoch at which entry `e` was last judged and
    /// *failed*, or 0 for a candidate that has never been judged — which is
    /// what ein.py's *absence* from `_park_stamp` says.
    ///
    /// A side table rather than a field on [`Entry`] because a fork inherits
    /// the arena and re-judges against its own world: this is the only part of
    /// a candidate that a saturation writes after enqueue, and separating it
    /// is what [T1a.6.12.5](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.12_boundary_and_snapshot.md#task-t1a6125--the-per-entering-snapshot)
    /// needs to share the arena instead of deep-copying it.
    judged_at: Vec<u32>,
    /// `gs_epoch[g]` — the last epoch at which some relation guard set `g`
    /// watches was seen to grow. A candidate is stale exactly when this is
    /// past its `judged_at`.
    gs_epoch: Vec<u32>,
    /// What each guard set watches — see [`WatchTables`].
    watch: Arc<WatchTables>,
    /// The extent size each watched relation had at the last round — the
    /// stamp, taken **once per round** rather than once per parked candidate
    /// per round. Parallel to `watch.rels`.
    watched_sizes: Vec<usize>,
    /// Scratch, parallel to `watch.rels`: which of them grew this round.
    watched_grew: Vec<bool>,
    sym_rels: Vec<Symbol>,
    sym_n: usize,
    sym_sym: Symbol,
    mirror_queue: Vec<FactId>,
    mirror_seeded: bool,
    mirror_enabled: bool,
    record_alternatives: bool,
    last_firing: Option<Firing>,

    // ── Observables ────────────────────────────────────────────────
    pub naf_rounds: u32,
    pub naf_admitted: u32,
    pub naf_retired: u32,
    /// Guard sub-plan **evaluations** — how many times a negative query was
    /// actually run. The boundary's cost is this number times the cost of one
    /// query, and 72 % of an exhaustive solve sits under it
    /// ([design/06](../../../../plans/m1a_rust/design/06_saturation.md) §2), so
    /// it is the figure Win B is measured in. Not an ein.py observable: it is
    /// the port's own instrument, and it is deliberately *not* in the T2 diff
    /// — a number that must not change is not evidence that nothing changed.
    pub guard_evals: u64,
    /// Of those, the ones on a **monotone** guard — the only ones the
    /// semi-naive re-evaluation of
    /// [design/06](../../../../plans/m1a_rust/design/06_saturation.md) § Win B
    /// can help, since a nested absent can flip from failing to passing and
    /// has no "only a new fact can change this" argument.
    pub guard_evals_monotone: u64,
    /// Nanoseconds spent inside the boundary — one `Instant` pair per
    /// quiescence, which is 40 of them on a zebra2 root and therefore free.
    ///
    /// It exists because the boundary's *share* is the question Win B is
    /// answered against: 72 % of an exhaustive ein.py solve sits here, and an
    /// optimisation aimed at 72 % is a different proposition from one aimed at
    /// a tenth of that.
    pub boundary_nanos: u64,
    /// Structurally 0 since S1.21.8, kept as an observable so the old
    /// measurement gate still has something to assert. It counted firings
    /// dropped at dequeue because a guard that passed at enqueue no longer
    /// held at fire time; a guard is now judged once, on the boundary, at the
    /// moment the candidate is admitted.
    pub naf_dropped: u32,
}

/// A saturation at its fixpoint, in a form another saturation can continue
/// from — [S1a.6.9](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md).
///
/// The closure is semi-naive *within* a saturation (`pos_index` + `run_seeded`)
/// and abandons that at the one boundary where the delta is smallest and known
/// exactly: `try_commitment_set` forks the saturated root and builds a **fresh**
/// `Saturator`, whose `delta = None` is a FULL pass, so the parent's whole
/// deductive closure is re-derived inside the fork — 94.6 % of a fork's firings
/// on `zebra -e` ([baseline.md §9](../../../../plans/m1a_rust/p1a.6_performance/baseline.md#9-the-fork-entry-re-derivation)).
///
/// What is carried is everything that answers "has this already been done":
/// the plan list *in its order*, `fired`, `seen`, the candidate arena and the
/// parked set with its watch stamps. What is **not** carried is the run's own
/// account of itself — the observables and `last_firing` — because the fork's
/// narration is its own.
///
/// `facts` is the parent's fact set at the snapshot. It is what makes the
/// resumed delta computable without enumerating every site that writes to root
/// (a forced positive, a singleton `(not h)` writeback, a lookahead kill
/// cache): the fork's delta is *whatever root has that the snapshot did not*,
/// plus the commitment. A site added later is covered by construction.
#[derive(Clone)]
pub struct Snapshot {
    engine: Engine,
    entries: Vec<Entry>,
    queue: BinaryHeap<Ranked>,
    parked: std::collections::BTreeSet<(i64, u64, u32)>,
    seen: FxHashSet<(BindingKey, GuardSetId)>,
    guard_sets: FxHashMap<Box<[u32]>, GuardSetId>,
    tiebreaker: u64,
    matched_plans: Vec<bool>,
    pos_index: FxHashMap<Symbol, Vec<usize>>,
    index_n: usize,
    /// The boundary's clock and what it has judged — carried so a fork
    /// re-judges exactly the parked candidates whose watched relations its own
    /// delta grew, and skips the rest for the same reason the parent did.
    epoch: u32,
    judged_at: Vec<u32>,
    gs_epoch: Vec<u32>,
    watch: Arc<WatchTables>,
    watched_sizes: Vec<usize>,
    sym_rels: Vec<Symbol>,
    sym_n: usize,
    mirror_seeded: bool,
    facts: FxHashSet<FactId>,
}

impl Snapshot {
    /// The facts `kb` has and the snapshot did not — the first half of a
    /// resumed fork's delta, in `FactId` order so the sequence is a function
    /// of the input alone.
    pub fn new_facts_of(&self, kb: &Kb) -> Vec<FactId> {
        let mut out: Vec<FactId> = kb.facts().filter(|f| !self.facts.contains(f)).collect();
        out.sort_unstable();
        out
    }
}

impl Saturator {
    pub fn new(s: &mut Session<'_>) -> Result<Saturator, SaturateError> {
        let cfg = s.kb.program().config.clone().unwrap_or_default();
        let sym_sym = s
            .terms
            .intern_text(SYMMETRIC)
            .expect("room for the mirror marker");
        let mut sat = Saturator {
            engine: Engine::with_memo(s.memo.clone()),
            matcher: Matcher::new(),
            entries: Vec::new(),
            queue: BinaryHeap::new(),
            parked: std::collections::BTreeSet::new(),
            seen: FxHashSet::default(),
            guard_sets: FxHashMap::default(),
            tiebreaker: 0,
            needs_enqueue: true,
            delta: None,
            matched_plans: Vec::new(),
            pos_index: FxHashMap::default(),
            index_n: usize::MAX,
            epoch: 0,
            judged_at: Vec::new(),
            gs_epoch: Vec::new(),
            watch: Arc::new(WatchTables::default()),
            watched_sizes: Vec::new(),
            watched_grew: Vec::new(),
            sym_rels: Vec::new(),
            sym_n: usize::MAX,
            sym_sym,
            mirror_queue: Vec::new(),
            mirror_seeded: false,
            // S1.20.I2 / S1.21.7 — both read once from the resolved config:
            // the second is consulted on the redundant-firing path, the
            // highest-volume path in the engine.
            mirror_enabled: cfg.enable_symmetric_mirror,
            record_alternatives: cfg.record_alternative_justifications,
            last_firing: None,
            naf_rounds: 0,
            naf_admitted: 0,
            naf_retired: 0,
            naf_dropped: 0,
            guard_evals: 0,
            guard_evals_monotone: 0,
            boundary_nanos: 0,
        };
        sat.engine
            .compile_all(s.ast, s.terms, s.kb, s.events)
            .map_err(SaturateError::Compile)?;
        Ok(sat)
    }

    /// This saturation's state, for a fork that resumes it — [`Snapshot`].
    ///
    /// Taken at a fixpoint: `queue` is empty there, and `parked` holds exactly
    /// the candidates whose guards still fail.
    pub fn snapshot(&self, kb: &Kb) -> Snapshot {
        Snapshot {
            engine: self.engine.clone(),
            entries: self.entries.clone(),
            queue: self.queue.clone(),
            parked: self.parked.clone(),
            seen: self.seen.clone(),
            guard_sets: self.guard_sets.clone(),
            tiebreaker: self.tiebreaker,
            matched_plans: self.matched_plans.clone(),
            pos_index: self.pos_index.clone(),
            index_n: self.index_n,
            epoch: self.epoch,
            judged_at: self.judged_at.clone(),
            gs_epoch: self.gs_epoch.clone(),
            watch: self.watch.clone(),
            watched_sizes: self.watched_sizes.clone(),
            sym_rels: self.sym_rels.clone(),
            sym_n: self.sym_n,
            mirror_seeded: self.mirror_seeded,
            facts: kb.facts().collect(),
        }
    }

    /// Continue `snapshot`'s saturation over `s.kb`, seeded from `delta`.
    ///
    /// The counterpart of [`Saturator::new`], and the difference is the whole
    /// stage: `new` starts with `delta = None`, which is a FULL enqueue pass
    /// over a KB already at its parent's fixpoint. This starts with the delta
    /// the caller knows exactly, and inherits `fired` / `seen` so the matches
    /// that already fired are not re-offered.
    ///
    /// Three things make that the same fixpoint:
    ///
    /// - the parent was at quiescence, so every match over parent-only facts
    ///   was enqueued and applied there; a match that is new here reads at
    ///   least one delta fact, which is what `run_seeded` starts from;
    /// - the parked set is carried with its watch stamps, so a candidate whose
    ///   guard failed in the parent is re-judged here rather than forgotten —
    ///   and the stamp is sound across the boundary because the KB only grows,
    ///   so an equal extent size is an equal extent;
    /// - `tiebreaker` continues from the parent's high-water mark, so this
    ///   saturation's own candidates sort *after* the inherited ones at equal
    ///   priority, which is what FIFO within a band means when the queue was
    ///   not built from scratch.
    ///
    /// The observables start at zero: the fork's rounds are the fork's.
    pub fn resume(
        s: &mut Session<'_>,
        snapshot: &Snapshot,
        delta: Vec<FactId>,
    ) -> Result<Saturator, SaturateError> {
        let cfg = s.kb.program().config.clone().unwrap_or_default();
        let sym_sym = s
            .terms
            .intern_text(SYMMETRIC)
            .expect("room for the mirror marker");
        let mut sat = Saturator {
            engine: snapshot.engine.clone(),
            matcher: Matcher::new(),
            entries: snapshot.entries.clone(),
            queue: snapshot.queue.clone(),
            parked: snapshot.parked.clone(),
            seen: snapshot.seen.clone(),
            guard_sets: snapshot.guard_sets.clone(),
            tiebreaker: snapshot.tiebreaker,
            needs_enqueue: true,
            delta: Some(delta),
            matched_plans: snapshot.matched_plans.clone(),
            pos_index: snapshot.pos_index.clone(),
            index_n: snapshot.index_n,
            epoch: snapshot.epoch,
            judged_at: snapshot.judged_at.clone(),
            gs_epoch: snapshot.gs_epoch.clone(),
            watch: snapshot.watch.clone(),
            watched_sizes: snapshot.watched_sizes.clone(),
            watched_grew: vec![false; snapshot.watch.rels.len()],
            sym_rels: snapshot.sym_rels.clone(),
            sym_n: snapshot.sym_n,
            sym_sym,
            mirror_queue: Vec::new(),
            // The parent already seeded the mirror from the whole extent, so
            // re-seeding would re-walk it; what the mirror has not seen is the
            // delta, and a delta fact reaches the KB by a direct write rather
            // than by a firing, so nothing else would offer it.
            mirror_seeded: snapshot.mirror_seeded,
            mirror_enabled: cfg.enable_symmetric_mirror,
            record_alternatives: cfg.record_alternative_justifications,
            last_firing: None,
            naf_rounds: 0,
            naf_admitted: 0,
            naf_retired: 0,
            naf_dropped: 0,
            guard_evals: 0,
            guard_evals_monotone: 0,
            boundary_nanos: 0,
        };
        if sat.mirror_enabled && sat.mirror_seeded {
            let delta = sat.delta.clone().unwrap_or_default();
            sat.enqueue_mirror_sources(s, &delta);
        }
        Ok(sat)
    }

    // ── Public API ─────────────────────────────────────────────────

    /// The next firing, or `None` at the two-phase fixpoint.
    pub fn step(&mut self, s: &mut Session<'_>) -> Result<Option<Firing>, SaturateError> {
        loop {
            if let Some(f) = self.closure_step(s)? {
                return Ok(Some(f));
            }
            // Positive quiescence — the boundary gets to speak.
            if s.events.on() {
                let (round, facts, queue, parked) = (
                    self.naf_rounds as i64,
                    s.kb.n_facts() as i64,
                    self.queue.len() as i64,
                    self.parked.len() as i64,
                );
                s.events.emit("quiesce", |l| {
                    l.num("round", round);
                    l.num("n_facts", facts);
                    l.num("n_queue", queue);
                    l.num("n_parked", parked);
                });
            }
            let start = std::time::Instant::now();
            let admitted = self.admit_from_boundary(s)?;
            self.boundary_nanos += start.elapsed().as_nanos() as u64;
            if admitted == 0 {
                return Ok(None);
            }
        }
    }

    /// Run to the fixpoint, handing each firing to `f`.
    ///
    /// `max_steps` is the runaway-debugging budget; `None` runs to the
    /// fixpoint.
    pub fn saturate(
        &mut self,
        s: &mut Session<'_>,
        max_steps: Option<usize>,
        f: &mut dyn FnMut(&Firing),
    ) -> Result<usize, SaturateError> {
        let mut i = 0;
        loop {
            if max_steps.is_some_and(|m| i >= m) {
                return Err(SaturateError::StepLimit(format!(
                    "saturator hit max_steps={} without reaching fixed point — \
                     last firing was {:?}; see Saturator::last_firing for the \
                     runaway candidate.",
                    max_steps.expect("checked"),
                    self.last_firing
                )));
            }
            match self.step(s)? {
                None => return Ok(i),
                Some(firing) => {
                    f(&firing);
                    self.last_firing = Some(firing);
                    i += 1;
                }
            }
        }
    }

    /// How many `(binding_key, guards)` pairs the dedup has seen.
    pub fn n_seen(&self) -> usize {
        self.seen.len()
    }

    pub fn last_firing(&self) -> Option<&Firing> {
        self.last_firing.as_ref()
    }

    /// Record the firing a caller driving `step` itself just consumed.
    ///
    /// `saturate` does this internally; the fail-fast fork loop
    /// ([`crate::commitment`]) drives `step` directly so it can stop at the
    /// firing that kills the branch, and the runaway-budget message reads
    /// `last_firing`.
    pub fn set_last_firing(&mut self, firing: Firing) {
        self.last_firing = Some(firing);
    }

    /// True iff no firing is available — at the **two-phase** fixpoint, not
    /// merely at closure quiescence.
    ///
    /// Forces a fresh enqueue pass first, because callers may have written
    /// facts straight to the KB outside `step`'s flow. That pass is a
    /// deliberate side effect: it advances the tiebreaker and therefore later
    /// ordering, and ein.py's does too.
    pub fn is_stalled(&mut self, s: &mut Session<'_>) -> Result<bool, SaturateError> {
        self.enqueue_pass(s, None)?;
        self.needs_enqueue = false;
        if self.mirror_enabled && self.has_pending_mirror(s) {
            return Ok(false);
        }
        let unfired = self
            .queue
            .iter()
            .any(|Reverse((_, _, e))| !self.engine.fired.contains(self.entry_key(*e)));
        if unfired {
            return Ok(false);
        }
        Ok(self.admit_from_boundary(s)? == 0)
    }

    // ── The closure ────────────────────────────────────────────────

    fn closure_step(&mut self, s: &mut Session<'_>) -> Result<Option<Firing>, SaturateError> {
        if self.needs_enqueue {
            let delta = self.delta.take();
            self.enqueue_pass(s, delta)?;
            self.needs_enqueue = false;
        }
        // The native mirror runs before rule firings so the closure is
        // available to them. A no-op when no relation is marked.
        if self.mirror_enabled
            && let Some(firing) = self.next_mirror_firing(s)?
        {
            return Ok(Some(firing));
        }
        while let Some(Reverse((_, _, entry))) = self.queue.pop() {
            if self.engine.fired.contains(self.entry_key(entry)) {
                continue;
            }
            let key = self.entry_key(entry).clone();
            let Some(firing) = self.apply(s, entry, key)? else {
                continue;
            };
            if s.events.on() && (!firing.redundant || s.events.verbose()) {
                emit_fire(s, &firing);
            }
            if !firing.redundant {
                // A productive firing wrote a new fact; the next pass processes
                // only what it derived (S1.8.B2v — semi-naive), accumulated
                // until consumed.
                self.needs_enqueue = true;
                self.delta
                    .get_or_insert_with(Vec::new)
                    .extend(firing.derived.iter().copied());
                if self.mirror_enabled {
                    self.enqueue_mirror_sources(s, &firing.derived);
                }
            }
            return Ok(Some(firing));
        }
        Ok(None)
    }

    /// Apply one popped candidate — productive or redundant.
    ///
    /// The guards are **not** re-evaluated here (that was the fire-time
    /// re-check S1.21.8 deleted), but they are recorded as the firing's
    /// negative premises, so a provenance walk can see what the conclusion
    /// depended on *not* holding.
    fn apply(
        &mut self,
        s: &mut Session<'_>,
        entry: u32,
        key: BindingKey,
    ) -> Result<Option<Firing>, SaturateError> {
        let e = &self.entries[entry as usize];
        let (plan_at, disjunct) = (e.plan, e.disjunct);
        let plan = self.engine.plan_arc(plan_at);
        if plan.asserts.is_empty() {
            // Defensive — nothing to derive. Mark fired so the queue does not
            // churn on it forever.
            self.engine.fired.insert(key);
            return Ok(None);
        }
        let env = Env {
            regs: &e.regs,
            trail: &e.trail,
            premises: &e.premises,
        };
        // Tentative-build every conclusion (A13: a `:assert (and …)` has
        // several). Cheap — a pure walk over each template.
        let mut tentative = Vec::with_capacity(plan.asserts.len());
        for i in 0..plan.asserts.len() {
            tentative.push(build_fact(s.terms, &plan, env, plan.asserts[i])?);
        }
        let all_known = tentative.iter().all(|&f| s.kb.contains(f));

        // Mark fired regardless of redundancy — the matcher keeps producing
        // this binding on every pass otherwise.
        self.engine.fired.insert(key);

        let guards = plan.disjuncts[disjunct].guards;
        if all_known {
            // Every conclusion already known. This is the real dedup seam:
            // returning *before* `fire` means a wholly-redundant re-derivation
            // never builds a provenance and never reaches the store's own
            // dedup (an exhaustive zebra2: ~194 k redundant firings against 8
            // store-level hits), so the alternative justification has to be
            // recorded here — it is exactly the derivation
            // first-derivation-wins would otherwise drop.
            if self.record_alternatives {
                self.record_alternative(s, &plan, entry, &tentative, guards);
            }
            let e = &self.entries[entry as usize];
            return Ok(Some(Firing {
                rule: plan.rule,
                activator: plan.activator_args.clone(),
                bindings: Env {
                    regs: &e.regs,
                    trail: &e.trail,
                    premises: &e.premises,
                }
                .bindings(&plan)
                .collect(),
                derived: tentative.into_boxed_slice(),
                premises: e.premises.clone(),
                redundant: true,
            }));
        }
        let absent = self.negative_premises(s, &plan, entry, guards);
        let e = &self.entries[entry as usize];
        let env = Env {
            regs: &e.regs,
            trail: &e.trail,
            premises: &e.premises,
        };
        Ok(fire(s.kb, s.terms, &plan, env, absent)?)
    }

    /// Record a redundant firing as an **alternative** justification (S1.21.7).
    ///
    /// One provenance per application, shared by every conclusion — the same
    /// contract a productive firing uses, with one deliberate exception:
    /// `bindings` is left empty. This is the highest-volume path in the engine,
    /// and `bindings` is display metadata no consumer of an *alternative*
    /// reads: the explanation search reads the premises, and the trace renders
    /// the primary.
    fn record_alternative(
        &mut self,
        s: &mut Session<'_>,
        plan: &Plan,
        entry: u32,
        existing: &[FactId],
        guards: Span,
    ) {
        let n = self.entries[entry as usize].premises.len();
        if n == 0 {
            return;
        }
        // Built only once at least one conclusion can still take an
        // alternative, so a hot fact whose list is full of shorter derivations
        // costs one O(1) check and nothing else.
        let targets: Vec<FactId> = existing
            .iter()
            .copied()
            .filter(|&f| s.kb.contains(f) && s.kb.accepts_justification(s.terms, f, n))
            .collect();
        if targets.is_empty() {
            return;
        }
        // An alternative carries its OWN negative premises: provenance is per
        // derivation, so a re-derivation admitted through the boundary depends
        // on what *its* guards found missing.
        let absent = self.negative_premises(s, plan, entry, guards);
        let premises = self.entries[entry as usize].premises.clone();
        let mut prov = Prov::from_rule(plan.rule, premises, None);
        prov.absent = absent;
        let id = s.terms.provs.push(prov);
        for fact in targets {
            let recorded = s.kb.record_justification(s.terms, fact, id);
            if recorded && s.events.on() {
                let (fact_s, rule_s) = (
                    events::sexpr(s.terms, fact),
                    s.terms.sym(plan.rule).to_string(),
                );
                let prems = events::sexpr_facts(s.terms, &self.entries[entry as usize].premises);
                s.events.emit("alt", |l| {
                    l.str("fact", &fact_s);
                    l.str("rule", &rule_s);
                    l.owned_strs("premises", prems);
                });
            }
        }
    }

    // ── The enqueue pass ───────────────────────────────────────────

    /// `delta == None` ⇒ a FULL pass: full-match every plan (the cold first
    /// pass, `is_stalled`, or any caller that wrote facts outside `step`).
    /// Otherwise a DELTA pass over the facts the last firing derived.
    fn enqueue_pass(
        &mut self,
        s: &mut Session<'_>,
        delta: Option<Vec<FactId>>,
    ) -> Result<(), SaturateError> {
        // `compile_all` first, so derived activators get plans — the
        // reflective rule-implication case (S1.8.A9).
        self.engine.compile_all(s.ast, s.terms, s.kb, s.events)?;
        self.matched_plans.resize(self.engine.len(), false);
        let Some(delta) = delta else {
            for at in 0..self.engine.len() {
                self.matched_plans[at] = true;
                self.full_match(s, at);
            }
            return Ok(());
        };
        self.rebuild_index();
        let mut full_done: FxHashSet<usize> = FxHashSet::default();
        // Never-matched plans (a reflective rule's freshly-compiled one) get
        // one FULL match each: they may match existing facts, not just the
        // delta.
        for at in 0..self.engine.len() {
            if !self.matched_plans[at] {
                self.matched_plans[at] = true;
                full_done.insert(at);
                self.full_match(s, at);
            }
        }
        // Positive-premise plans: seed each delta fact. The matcher iterates
        // the one new fact at its premise instead of re-scanning the
        // relation's whole extent — 91 % of matcher output was re-discovery a
        // full re-match would recompute.
        for fact in delta {
            let rel = s.terms.facts.rel(fact);
            let plans = self.pos_index.get(&rel).cloned().unwrap_or_default();
            for at in plans {
                if !full_done.contains(&at) {
                    self.seed_match(s, at, fact);
                }
            }
        }
        Ok(())
    }

    /// relation → plans with that relation as a top-level positive premise —
    /// the D5-seedable set. Rebuilt when the plan list grows.
    fn rebuild_index(&mut self) {
        if self.index_n == self.engine.len() {
            return;
        }
        self.pos_index.clear();
        for at in 0..self.engine.len() {
            let plan = self.engine.plan(at);
            let mut rels: Vec<Symbol> = Vec::new();
            for d in plan.disjuncts.iter() {
                for step in plan.steps(d.steps) {
                    if let Step::Rel(r) = step
                        && !rels.contains(&r.rel)
                    {
                        rels.push(r.rel);
                    }
                }
            }
            for rel in rels {
                self.pos_index.entry(rel).or_default().push(at);
            }
        }
        self.index_n = self.engine.len();
    }

    fn full_match(&mut self, s: &mut Session<'_>, at: usize) {
        let plan = self.engine.plan_arc(at);
        let priority = self.priority_for(s, &plan);
        // The matcher is moved out for the duration: `enqueue_binding` runs
        // *inside* its callback and needs the saturator, which the matcher is
        // part of. Moving it out is what keeps the enqueue on the match's own
        // stack — a duplicate then costs nothing beyond the key hash.
        let mut m = std::mem::take(&mut self.matcher);
        let (kb, terms, ast) = (&*s.kb, &*s.terms, s.ast);
        for d in 0..plan.disjuncts.len() {
            let events = &mut *s.events;
            m.run_one(kb, terms, ast, &plan, d, &mut |mt| {
                self.enqueue_binding(terms, events, at, &plan, mt, priority);
                ControlFlow::Continue(())
            });
        }
        self.matcher = m;
    }

    fn seed_match(&mut self, s: &mut Session<'_>, at: usize, fact: FactId) {
        let plan = self.engine.plan_arc(at);
        let priority = self.priority_for(s, &plan);
        let mut m = std::mem::take(&mut self.matcher);
        let (kb, terms, ast, events) = (&*s.kb, &*s.terms, s.ast, &mut *s.events);
        m.run_seeded(kb, terms, ast, &plan, fact, &mut |mt| {
            self.enqueue_binding(terms, events, at, &plan, mt, priority);
            ControlFlow::Continue(())
        });
        self.matcher = m;
    }

    /// Dedup a match and route it to the queue or the park.
    ///
    /// A match whose disjunct carries `(absent …)` guards does **not** enter
    /// the firing queue: the closure must reach a fixpoint before any negation
    /// is consulted, and enqueuing it here would be asking "is P absent?" of a
    /// half-built world.
    fn enqueue_binding(
        &mut self,
        terms: &Terms,
        events: &mut Events,
        at: usize,
        plan: &Plan,
        m: &crate::match_::Match<'_>,
        priority: i64,
    ) {
        let disjunct = m.disjunct;
        let key = BindingKey::new(plan, self.engine.activator(at), m.regs());
        let guards = self.guard_set(plan, disjunct);
        let seen_key = (key.clone(), guards);
        if self.seen.contains(&seen_key) {
            return;
        }
        if self.engine.fired.contains(&key) {
            self.seen.insert(seen_key);
            return;
        }
        self.seen.insert(seen_key);
        self.tiebreaker += 1;
        let parked = !plan.disjuncts[disjunct].guards.is_empty();
        if events.on() {
            let (rule, activator, bindings) = (
                terms.sym(plan.rule).to_string(),
                plan.activator_args
                    .iter()
                    .map(|&x| terms.sym(x).to_string())
                    .collect::<Vec<_>>(),
                m.bindings()
                    .map(|(k, v)| (terms.sym(k).to_string(), events::sexpr_value(terms, v)))
                    .collect::<Vec<_>>(),
            );
            let tb = self.tiebreaker as i64;
            events.emit("enqueue", |l| {
                l.str("rule", &rule);
                l.owned_strs("activator", activator);
                l.bindings("bindings", bindings);
                l.num("priority", priority);
                l.num("tiebreaker", tb);
                l.bool("parked", parked);
            });
        }
        let entry = self.entries.len() as u32;
        self.entries.push(Entry {
            plan: at,
            disjunct,
            regs: m.regs().into(),
            trail: m.trail().into(),
            premises: m.premises().into(),
            key,
            guard_set: guards,
        });
        self.judged_at.push(0);
        if parked {
            self.parked.insert((priority, self.tiebreaker, entry));
        } else {
            self.queue.push(Reverse((priority, self.tiebreaker, entry)));
        }
    }

    /// The interned structural identity of a disjunct's guard tuple — see
    /// [`crate::plan::Disjunct::guard_key`].
    fn guard_set(&mut self, plan: &Plan, disjunct: usize) -> GuardSetId {
        let key = plan.guard_key(plan.disjuncts[disjunct].guard_key);
        if let Some(&id) = self.guard_sets.get(key) {
            return id;
        }
        let id = GuardSetId(self.guard_sets.len() as u32);
        self.guard_sets.insert(key.into(), id);
        // A new set is a new row in the epoch tables. The relations it watches
        // join `watched_rels` if no earlier set already watches them; a
        // relation's size is therefore tracked from before any candidate that
        // reads it can have been judged, which is what makes an epoch as sound
        // as the per-candidate stamp it replaces.
        let w = Arc::make_mut(&mut self.watch);
        let start = w.watched.len() as u32;
        for g in plan.guards(plan.disjuncts[disjunct].guards) {
            for &rel in g.watched.iter() {
                let at = match w.rels.iter().position(|&r| r == rel) {
                    Some(i) => i as u32,
                    None => {
                        w.rels.push(rel);
                        self.watched_sizes.push(0);
                        self.watched_grew.push(false);
                        (w.rels.len() - 1) as u32
                    }
                };
                if !w.watched[start as usize..].contains(&at) {
                    w.watched.push(at);
                }
            }
        }
        w.spans.push((start, w.watched.len() as u32 - start));
        debug_assert_eq!(self.gs_epoch.len(), id.0 as usize);
        self.gs_epoch.push(self.epoch);
        id
    }

    fn priority_for(&self, s: &Session<'_>, plan: &Plan) -> i64 {
        let Some(rule) = s.kb.program().rules.get(plan.rule) else {
            return DEFAULT_PRIORITY;
        };
        let Some(p) = rule.priority else {
            return DEFAULT_PRIORITY;
        };
        s.terms.ints.value(p).unwrap_or_else(|| {
            // Wider than an `i64`. The grammar accepts any width and nothing
            // in the corpus is past three digits, so this is a bound rather
            // than a behaviour: the extreme sorts to the extreme.
            if s.terms.int_text(p).starts_with('-') {
                i64::MIN
            } else {
                i64::MAX
            }
        })
    }

    fn entry_key(&self, entry: u32) -> &BindingKey {
        &self.entries[entry as usize].key
    }

    // ── The boundary ───────────────────────────────────────────────

    /// Judge parked candidates against the quiesced world; admit at most one.
    ///
    /// Parked candidates are examined in the engine's own (priority, FIFO)
    /// order and the **first** whose guards pass is admitted; the rest stay
    /// parked and are re-judged after the closure quiesces again.
    ///
    /// Failures stay parked, because a `forall`'s nested absent flips from
    /// failing to passing as the KB grows — `(absent (and G (absent B)))`:
    /// adding a `B` makes the inner fail and the outer pass — so a parked
    /// candidate is a standing question, not a settled one. An *anti-monotone*
    /// guard that fails is retired instead: its query is purely positive, the
    /// KB only grows, so it will keep matching and the candidate is dead.
    fn admit_from_boundary(&mut self, s: &mut Session<'_>) -> Result<u32, SaturateError> {
        if self.parked.is_empty() {
            return Ok(0);
        }
        self.naf_rounds += 1;
        self.refresh_epochs(s);
        let mut admitted = 0;
        // A snapshot, because the round removes from `self.parked` as it goes
        // and a candidate is admitted at most once. One allocation per round,
        // against ein.py's two heap operations per parked candidate per round.
        let order: Vec<(i64, u64, u32)> = self.parked.iter().copied().collect();
        for key in order {
            let (_, tb, entry) = key;
            if self.engine.fired.contains(self.entry_key(entry)) {
                self.parked.remove(&key); // fired by another route
                continue;
            }
            // Invalidation: a guard's verdict can only move if one of the
            // relations its query reads has grown. Most parked candidates wait
            // on something that did not change this round, and re-running
            // their queries is what makes a naive boundary quadratic (zebra2
            // root: 460 parked over 40 rounds). The question is asked of the
            // candidate's *guard set*, whose clock `refresh_epochs` advanced
            // once for the whole round — two integer loads, against the two
            // extent probes and the vector compare a per-candidate stamp cost.
            ein_core::counters::bump(|c| c.watch_stamp += 1);
            let e = &self.entries[entry as usize];
            let judged_at = self.judged_at[entry as usize];
            if judged_at != 0 && self.gs_epoch[e.guard_set.0 as usize] <= judged_at {
                continue;
            }
            let plan = self.engine.plan_arc(e.plan);
            let guards = plan.disjuncts[e.disjunct].guards;
            let failing = self.first_failing(s, &plan, entry, guards);
            match failing {
                None => {
                    self.parked.remove(&key);
                    self.queue.push(Reverse(key));
                    self.naf_admitted += 1;
                    admitted = 1;
                    if s.events.on() {
                        let rule = s.terms.sym(plan.rule).to_string();
                        let round = self.naf_rounds as i64;
                        s.events.emit("admit", |l| {
                            l.num("tiebreaker", tb as i64);
                            l.num("round", round);
                            l.str("rule", &rule);
                        });
                    }
                    break;
                }
                Some(g) if plan.guards(guards)[g].monotone => {
                    // Anti-monotone, and it found a match: the KB only grows,
                    // so it will keep finding one. This candidate is dead, not
                    // waiting — retire it rather than re-asking every round.
                    self.parked.remove(&key);
                    self.naf_retired += 1;
                    if s.events.on() {
                        emit_boundary(s, "retire", tb, self.naf_rounds, &plan, guards, g);
                    }
                }
                Some(g) => {
                    self.judged_at[entry as usize] = self.epoch;
                    if s.events.on() {
                        emit_boundary(s, "park", tb, self.naf_rounds, &plan, guards, g);
                    }
                }
            }
        }
        Ok(admitted)
    }

    /// Advance the boundary's clock, and with it the guard sets whose world
    /// moved — the whole round's invalidation, in one pass over the watched
    /// relations.
    ///
    /// S1a.3.4 T4 asked this per parked candidate: build the extent sizes of
    /// every relation its guards read, compare against the sizes at its last
    /// failed judgement, skip if equal. The answer is the right one — the KB
    /// grows monotonically within a run, so a relation cannot lose a fact and
    /// equal size means equal extent — but the question was asked 248 043
    /// times on `zebra -e` to reach 29 865 judgements
    /// ([baseline.md §17](../../../../plans/m1a_rust/p1a.6_performance/baseline.md#17-the-boundary-measured-before-the-stage-that-aims-at-it)),
    /// because *sizes* are a property of the world and only the comparison is
    /// a property of the candidate.
    ///
    /// So the sizes are taken once per round, and a guard set whose relations
    /// moved gets this round's epoch. A candidate is stale exactly when its
    /// set's epoch is past the epoch it was judged at — the same predicate,
    /// evaluated in two integer loads.
    ///
    /// Sound across a round because the KB **cannot change while the boundary
    /// runs**: the guard queries are read-only, and an admission ends the
    /// round without firing. Every judgement in a round therefore sees the
    /// sizes this pass recorded, and any growth between rounds is caught by
    /// the next one.
    fn refresh_epochs(&mut self, s: &Session<'_>) {
        self.epoch += 1;
        let mut any = false;
        for i in 0..self.watch.rels.len() {
            ein_core::counters::bump(|c| c.watch_stamp_rel += 1);
            let n = s.kb.n_facts_of(self.watch.rels[i]);
            if n != self.watched_sizes[i] {
                self.watched_sizes[i] = n;
                self.watched_grew[i] = true;
                any = true;
            }
        }
        if !any {
            return;
        }
        for (g, &(start, len)) in self.watch.spans.iter().enumerate() {
            let watched = &self.watch.watched[start as usize..(start + len) as usize];
            if watched.iter().any(|&i| self.watched_grew[i as usize]) {
                self.gs_epoch[g] = self.epoch;
            }
        }
        self.watched_grew.fill(false);
    }

    /// The first guard that does not pass here, or `None` if all do.
    ///
    /// The caller needs the guard, not just the verdict: an anti-monotone one
    /// that fails has failed permanently.
    fn first_failing(
        &mut self,
        s: &mut Session<'_>,
        plan: &Plan,
        entry: u32,
        guards: Span,
    ) -> Option<usize> {
        let regs = self.entries[entry as usize].regs.clone();
        let mut m = std::mem::take(&mut self.matcher);
        let mut out = None;
        for (i, g) in plan.guards(guards).iter().enumerate() {
            ein_core::counters::bump(|c| c.guard_query += 1);
            self.guard_evals += 1;
            self.guard_evals_monotone += g.monotone as u64;
            // The same two, summed over every fork of a solve — Q-M1a.17's
            // exhaustive half, which a per-saturation field cannot answer.
            ein_core::counters::bump(|c| {
                c.guard_eval += 1;
                c.guard_eval_monotone += g.monotone as u64;
            });
            if m.holds(s.kb, s.terms, s.ast, plan, g, &regs) {
                out = Some(i);
                break;
            }
        }
        self.matcher = m;
        out
    }

    /// What a firing admitted here depended on *not* holding.
    ///
    /// One [`NafRef`] per relation pattern the guard queried, with the guard's
    /// projected bindings substituted in and [`NafArg::Free`] left where the
    /// query ranged free. Nested guards contribute their patterns too: the
    /// whole query is what had to fail. Deduped preserving first-seen order.
    fn negative_premises(
        &self,
        s: &Session<'_>,
        plan: &Plan,
        entry: u32,
        guards: Span,
    ) -> Box<[NafRef]> {
        if guards.is_empty() {
            return Box::new([]);
        }
        let regs = &self.entries[entry as usize].regs;
        let mut out: Vec<NafRef> = Vec::new();
        for g in plan.guards(guards) {
            collect_refs(plan, g, g.sub, regs, &mut out);
        }
        let _ = s;
        out.dedup_by(|a, b| a == b);
        // `dict.fromkeys` dedups globally, not only adjacent duplicates.
        let mut seen: Vec<NafRef> = Vec::new();
        for r in out {
            if !seen.contains(&r) {
                seen.push(r);
            }
        }
        seen.into_boxed_slice()
    }

    // ── The `__symmetric__` native mirror ──────────────────────────

    /// Relations marked `(__symmetric__ R)`; cached, refreshed when the marker
    /// count changes (a rule may derive one).
    fn symmetric_rels(&mut self, s: &Session<'_>) -> &[Symbol] {
        let n = s.kb.n_facts_of(self.sym_sym);
        if n != self.sym_n {
            self.sym_rels =
                s.kb.facts_of(self.sym_sym)
                    .filter_map(|f| s.terms.facts.args(f).first().and_then(|v| v.as_sym()))
                    .collect();
            // ein.py builds a `frozenset` here, whose iteration order depends
            // on string hashes and so on `PYTHONHASHSEED` — the one place set
            // order leaked into firing order (M1a hazard H1). ein.py was fixed
            // to sort the cold seed; this holds the sorted order throughout, so
            // the mirror sequence is a function of the input alone.
            self.sym_rels
                .sort_by(|a, b| s.terms.sym(*a).cmp(s.terms.sym(*b)));
            self.sym_rels.dedup();
            self.sym_n = n;
        }
        &self.sym_rels
    }

    fn enqueue_mirror_sources(&mut self, s: &Session<'_>, facts: &[FactId]) {
        if self.symmetric_rels(s).is_empty() {
            return;
        }
        for &f in facts {
            let rel = s.terms.facts.rel(f);
            if self.sym_rels.contains(&rel) {
                self.mirror_queue.push(f);
            }
        }
    }

    /// True iff some marked edge `(R a b)` with `a ≠ b` lacks its mirror.
    /// Non-mutating, so `is_stalled` can consult it.
    fn has_pending_mirror(&mut self, s: &Session<'_>) -> bool {
        let rels = self.symmetric_rels(s).to_vec();
        for rel in rels {
            for f in s.kb.facts_of(rel).collect::<Vec<_>>() {
                let args = s.terms.facts.args(f);
                if args.len() == 2
                    && args[0] != args[1]
                    && s.terms
                        .probe_fact(rel, &[args[1], args[0]])
                        .is_none_or(|m| !s.kb.contains(m))
                {
                    return true;
                }
            }
        }
        false
    }

    /// One native mirror write, or `None`.
    ///
    /// A marked `R`'s extension is closed under arg-swap: rather than
    /// compiling the stdlib `symmetric` rule and running the matcher each
    /// pass, pop a source `(R a b)` and write `(R b a)` directly. The mirror
    /// feeds the delta so rules re-enqueue against it.
    fn next_mirror_firing(&mut self, s: &mut Session<'_>) -> Result<Option<Firing>, SaturateError> {
        if !self.mirror_seeded {
            self.mirror_seeded = true;
            let rels = self.symmetric_rels(s).to_vec();
            for rel in rels {
                let extent: Vec<FactId> = s.kb.facts_of(rel).collect();
                self.mirror_queue.extend(extent);
            }
        }
        // `.pop()` — the queue is a **stack**, LIFO despite the name.
        while let Some(src) = self.mirror_queue.pop() {
            let rel = s.terms.facts.rel(src);
            if !self.symmetric_rels(s).contains(&rel) {
                continue;
            }
            let args = s.terms.facts.args(src);
            if args.len() != 2 {
                continue;
            }
            let (a, b) = (args[0], args[1]);
            if a == b {
                continue;
            }
            let existing = s
                .terms
                .probe_fact(rel, &[b, a])
                .filter(|&m| s.kb.contains(m));
            if let Some(mirror) = existing {
                // S1.21.7 — the mirror already exists (typically because the
                // stdlib `symmetric` rule, or the mirror of the mirror, got
                // there first). Arg-swap is a real second derivation of it, so
                // record it rather than dropping it. This is what makes the
                // justification graph genuinely cyclic — `(R a b)` and
                // `(R b a)` justify each other — which the explanation search
                // handles by taking a least fixpoint from the sources up.
                if self.record_alternatives && s.kb.accepts_justification(s.terms, mirror, 1) {
                    let prov = Prov::from_rule(self.sym_sym, Box::new([src]), None);
                    let id = s.terms.provs.push(prov);
                    s.kb.record_justification(s.terms, mirror, id);
                }
                continue;
            }
            let prov = Prov::from_rule(self.sym_sym, Box::new([src]), None);
            let id = s.terms.provs.push(prov);
            let stored =
                s.kb.add_and_index_fact(s.terms, rel, &[b, a], Some(id))
                    .map_err(|e| SaturateError::Fire(FireError::Overflow(e)))?
                    .id();
            self.needs_enqueue = true;
            self.delta.get_or_insert_with(Vec::new).push(stored);
            self.enqueue_mirror_sources(s, &[stored]);
            if s.events.on() {
                let (relation, src_s, derived_s) = (
                    s.terms.sym(rel).to_string(),
                    events::sexpr(s.terms, src),
                    events::sexpr(s.terms, stored),
                );
                s.events.emit("mirror", |l| {
                    l.str("relation", &relation);
                    l.str("src", &src_s);
                    l.str("derived", &derived_s);
                });
            }
            let (a_sym, b_sym) = (
                s.terms.intern_text("a").expect("room"),
                s.terms.intern_text("b").expect("room"),
            );
            return Ok(Some(Firing {
                rule: self.sym_sym,
                activator: Box::new([rel]),
                bindings: Box::new([(a_sym, a), (b_sym, b)]),
                derived: Box::new([stored]),
                premises: Box::new([src]),
                redundant: false,
            }));
        }
        Ok(None)
    }
}

/// `world._collect_refs` — ground a guard sub-plan's relation patterns against
/// the projected bindings, recursing into nested guards.
fn collect_refs(plan: &Plan, g: &NafGuard, span: Span, regs: &[Value], out: &mut Vec<NafRef>) {
    for step in plan.steps(span) {
        match step {
            Step::Rel(r) => out.push(NafRef {
                rel: r.rel,
                args: plan
                    .slots(r.slots)
                    .iter()
                    .map(|s| ground(plan, g, s, regs))
                    .collect(),
            }),
            Step::Absent { sub } => collect_refs(plan, g, *sub, regs, out),
            Step::Guard { .. } => {}
        }
    }
}

/// A slot's value under the guard's *projected* environment; free otherwise.
///
/// A sub-plan-local variable grounds to `Free` even when the parent has a
/// value for the same name, because `world._collect_refs` walks with the
/// projected dict — which is the same restriction the query itself ran under.
/// A `Fact`-valued binding also grounds to `Free`: ein.py's `_ground` keeps
/// only `str` and `int`.
fn ground(plan: &Plan, g: &NafGuard, slot: &Slot, regs: &[Value]) -> NafArg {
    match slot {
        Slot::Reg(r) => match g.scope_of[*r as usize] {
            Some(parent) => {
                let v = regs[parent as usize];
                if v.is_unbound() || v.as_fact().is_some() {
                    NafArg::Free
                } else {
                    NafArg::Value(v)
                }
            }
            None => NafArg::Free,
        },
        Slot::Const(v) => NafArg::Value(*v),
        Slot::Nested { rel, slots } => NafArg::Nested {
            rel: *rel,
            args: plan
                .slots(*slots)
                .iter()
                .map(|s| ground(plan, g, s, regs))
                .collect(),
        },
        Slot::Opaque(_) => NafArg::Free,
    }
}

fn emit_fire(s: &mut Session<'_>, firing: &Firing) {
    let rule = s.terms.sym(firing.rule).to_string();
    let activator: Vec<String> = firing
        .activator
        .iter()
        .map(|&x| s.terms.sym(x).to_string())
        .collect();
    let bindings = events::binding_pairs(s.terms, &firing.bindings);
    let premises = events::sexpr_facts(s.terms, &firing.premises);
    let derived = events::sexpr_facts(s.terms, &firing.derived);
    let redundant = firing.redundant;
    s.events.emit("fire", |l| {
        l.str("rule", &rule);
        l.owned_strs("activator", activator);
        l.bindings("bindings", bindings);
        l.owned_strs("premises", premises);
        l.owned_strs("derived", derived);
        l.bool("redundant", redundant);
    });
}

fn emit_boundary(
    s: &mut Session<'_>,
    kind: &str,
    tb: u64,
    round: u32,
    plan: &Plan,
    guards: Span,
    failing: usize,
) {
    let rule = s.terms.sym(plan.rule).to_string();
    let mut watched: Vec<String> = plan.guards(guards)[failing]
        .watched
        .iter()
        .map(|&w| s.terms.sym(w).to_string())
        .collect();
    watched.sort();
    s.events.emit(kind, |l| {
        l.num("tiebreaker", tb as i64);
        l.num("round", round as i64);
        l.str("rule", &rule);
        l.owned_strs("watched", watched);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_ir::{from_ir::load, parse};

    fn saturate(src: &str) -> (Terms, Kb, Vec<String>, u32) {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let mut kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let mut ev = Events::off();
        let order: Vec<String>;
        let rounds;
        {
            let mut s = Session {
                kb: &mut kb,
                terms: &mut terms,
                ast: &ast,
                events: &mut ev,
                memo: SharedMemo::default(),
            };
            let mut sat = Saturator::new(&mut s).expect("compiles");
            let mut derived: Vec<FactId> = Vec::new();
            sat.saturate(&mut s, None, &mut |f| {
                derived.extend(f.derived.iter().copied());
            })
            .expect("saturates");
            rounds = sat.naf_rounds;
            order = derived.iter().map(|&f| events::sexpr(s.terms, f)).collect();
        }
        (terms, kb, order, rounds)
    }

    /// The `__symmetric__` cold seed iterates the marked relations **sorted by
    /// name**, so the mirror firing sequence is a function of the input alone.
    ///
    /// ein.py's `_symmetric_rels()` is a `frozenset`, whose iteration order
    /// depends on string hashes and therefore on `PYTHONHASHSEED`; with two or
    /// more markers that leaked into the seed order, hence into the firing
    /// order, hence into every derivation-order observable. That is M1a hazard
    /// H1, fixed in ein.py first and ported sorted.
    #[test]
    fn the_mirror_seeds_its_relations_in_name_order() {
        let src = "(relation zeta A B)\n(relation alpha A B)\n(relation mid A B)\n\
                   (__symmetric__ zeta)\n(__symmetric__ alpha)\n(__symmetric__ mid)\n\
                   (zeta z1 z2)\n(alpha a1 a2)\n(mid m1 m2)\n";
        let (_, _, order, _) = saturate(src);
        assert_eq!(
            order,
            ["(zeta z2 z1)", "(mid m2 m1)", "(alpha a2 a1)"],
            "the seed is sorted by name — alpha, mid, zeta — and the queue is \
             a LIFO **stack**, so the last relation seeded mirrors first"
        );
    }

    /// One admission per boundary round, and it is a soundness requirement.
    ///
    /// On the classic unstratifiable program both guards pass against the
    /// empty world, so a batch admission derives **both** — and `{p, q}` is not
    /// a model of `p ← absent q; q ← absent p` under any reading. Admitting one
    /// closes the window completely: the queue is empty at quiescence, so the
    /// admitted candidate fires immediately, against exactly the world its
    /// guard was judged against.
    #[test]
    fn an_unstratifiable_program_derives_one_of_the_two() {
        let src = "(relation p Thing)\n(relation q Thing)\n(relation thing Thing)\n\
                   (rule derive-p ()\n  :match (and (thing ?x) (absent (q ?x)))\n  \
                   :assert (p ?x))\n\
                   (rule derive-q ()\n  :match (and (thing ?x) (absent (p ?x)))\n  \
                   :assert (q ?x))\n\
                   (thing one)\n";
        let (_, _, order, rounds) = saturate(src);
        assert_eq!(order.len(), 1, "exactly one of p, q — got {order:?}");
        assert!(order[0] == "(p one)" || order[0] == "(q one)");
        assert!(rounds >= 1, "the boundary ran");
    }

    /// A resumed saturation reaches the same fixpoint as a fresh one —
    /// [`Saturator::resume`], the mechanism
    /// [S1a.6.9](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md)
    /// is about.
    ///
    /// Saturate, add a fact, then continue two ways: a **fresh** saturator
    /// over the grown KB (a FULL pass, which is what a fork does today) and a
    /// **resumed** one seeded with just the new fact. The two fact sets must
    /// agree exactly. What the test deliberately does *not* assert is the
    /// firing sequence: the resumed run skips the re-derivations, and that
    /// difference is the stage's whole subject.
    #[test]
    fn a_resumed_saturation_reaches_the_same_fixpoint() {
        let src = "(relation edge T T)\n(relation path T T)\n                   (rule step ()\n  :match (edge ?a ?b)\n  :assert (path ?a ?b))\n                   (rule trans ()\n  :match (and (path ?a ?b) (path ?b ?c))\n                     :assert (path ?a ?c))\n                   (rule tip ()\n  :match (and (path ?a ?b) (absent (edge ?b ?a)))\n                     :assert (path ?b ?b))\n                   (edge A B)\n(edge B C)\n";
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let mut kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let mut ev = Events::off();
        let memo = SharedMemo::default();

        let snapshot = {
            let mut s = Session {
                kb: &mut kb,
                terms: &mut terms,
                ast: &ast,
                events: &mut ev,
                memo: memo.clone(),
            };
            let mut sat = Saturator::new(&mut s).expect("compiles");
            sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
            sat.snapshot(s.kb)
        };

        // The delta: one new edge, written straight into two forks of the
        // fixpoint, exactly as `try_commitment_set` writes a hypothesis.
        let mut fresh = kb.fork();
        let mut resumed = kb.fork();
        let edge = terms.syms.get("edge").expect("interned");
        let (c, d) = (
            terms.intern_text("C").expect("room"),
            terms.intern_text("D").expect("room"),
        );
        let args = [Value::sym(c), Value::sym(d)];
        let added_fresh = fresh
            .add_and_index_fact(&mut terms, edge, &args, None)
            .expect("room");
        let added_resumed = resumed
            .add_and_index_fact(&mut terms, edge, &args, None)
            .expect("room");
        assert_eq!(added_fresh.id(), added_resumed.id(), "the same fact");

        let mut n_fresh = 0usize;
        {
            let mut s = Session {
                kb: &mut fresh,
                terms: &mut terms,
                ast: &ast,
                events: &mut ev,
                memo: memo.clone(),
            };
            let mut sat = Saturator::new(&mut s).expect("compiles");
            sat.saturate(&mut s, None, &mut |_| n_fresh += 1)
                .expect("saturates");
        }
        let mut n_resumed = 0usize;
        {
            let delta = snapshot.new_facts_of(&resumed);
            assert_eq!(delta, vec![added_resumed.id()], "one new fact");
            let mut s = Session {
                kb: &mut resumed,
                terms: &mut terms,
                ast: &ast,
                events: &mut ev,
                memo: memo.clone(),
            };
            let mut sat = Saturator::resume(&mut s, &snapshot, delta).expect("resumes");
            sat.saturate(&mut s, None, &mut |_| n_resumed += 1)
                .expect("saturates");
        }

        let key = |kb: &Kb| {
            let mut v: Vec<String> = kb.facts().map(|f| events::sexpr(&terms, f)).collect();
            v.sort();
            v
        };
        assert_eq!(key(&fresh), key(&resumed), "the fixpoints differ");
        assert!(
            n_resumed < n_fresh,
            "the resumed run narrated {n_resumed} firings and the fresh one              {n_fresh} — the point is that it narrates fewer"
        );
    }

    /// `naf_dropped` is **structurally** 0: a guard is judged once, on the
    /// boundary, at the moment the candidate is admitted, so there is no
    /// window between the verdict and the firing for it to go stale in.
    #[test]
    fn nothing_is_ever_dropped_for_a_stale_guard() {
        let src = "(relation a Thing)\n(relation block Thing)\n(relation seen Thing)\n\
                   (rule r ()\n  :match (and (a ?x) (absent (block ?x)))\n  \
                   :assert (seen ?x))\n\
                   (a one)\n(a two)\n(block two)\n";
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let mut kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let mut ev = Events::off();
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut ev,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).expect("compiles");
        sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
        assert_eq!(sat.naf_dropped, 0);
        assert_eq!(sat.naf_admitted, 1, "only the unblocked one");
        assert_eq!(sat.naf_retired, 1, "the blocked one is dead, not waiting");
        assert!(sat.is_stalled(&mut s).expect("stalled"));
    }

    /// The runaway budget names the last firing, because that is the runaway
    /// candidate a caller is looking for.
    #[test]
    fn the_step_limit_quotes_the_last_firing() {
        let src = "(relation edge A B)\n(relation path A B)\n\
                   (rule walk ()\n  :match (and (edge ?a ?b) (edge ?b ?c))\n  \
                   :assert (edge ?a ?c))\n\
                   (edge n1 n2)\n(edge n2 n3)\n(edge n3 n4)\n(edge n4 n5)\n";
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let mut kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let mut ev = Events::off();
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut ev,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).expect("compiles");
        let err = sat
            .saturate(&mut s, Some(2), &mut |_| {})
            .expect_err("the budget runs out");
        let msg = err.to_string();
        assert!(msg.contains("max_steps=2"), "{msg}");
        assert!(msg.contains("without reaching fixed point"), "{msg}");
        assert!(sat.last_firing().is_some());
    }
}
