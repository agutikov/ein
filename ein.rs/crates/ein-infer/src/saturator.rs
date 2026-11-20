//! The saturation driver — the **two-phase** loop
//! ([03-design/06-saturation.md § 1](../../../../docs/ein/ein.rs/03-design/06-saturation.md#1-two-phases)).
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
//! Everything else stands: the agenda below, its halves each in the written
//! and arrival order via the monotone tiebreaker (no rule has a priority),
//! `seen` keyed on `(binding_key, guards)`, and `naf_dropped` structurally 0 —
//! there is no enqueue/fire window left for a guard to go stale in.
//!
//! ### A rule that ends a state
//!
//! A rule that states `:ein inconsistent` (Q-M1f.399) concludes no fact:
//! where it is applied, the application is recorded on the KB
//! ([`ein_core::Kb::end`], [`crate::firing::end`]), which is then
//! inconsistent, and nothing is written. The KB holds what the firing rests
//! on as it would a derived fact's derivation, so a core is taken over it.
//! **A candidate of such a rule is before every other in the queue**
//! (`ORDINARY`), under the agenda below and under the written order alone,
//! and among the parked ones at the boundary, so the state ends at the
//! firing that concluded what the rule matched, one firing later. Every
//! program has such a rule, `std.core`'s, which matches a held `(false)`
//! (Q-M1f.551): the kernel reads `(false)` by its name nowhere.
//!
//! ### The agenda: what leads to the end of a state first
//!
//! A candidate of a rule that refutes a fact beside one holding it as its one
//! argument, alone ([`crate::contradiction::Pairs::refutes_at`]),
//! `std.contradiction`'s `clash-N`, is before every other as one of a rule
//! that states `:ein inconsistent` is (`Saturator::at_once`): so a state ends
//! as soon as it holds the pair, at the same firing whatever was queued
//! before it, and a saturation stopped at its first contradiction stops
//! there.
//!
//! Every saturation ranks its other candidates in two halves, [`AGENDA`]:
//! first those of a rule whose conclusion a contradiction can be made of, a
//! fact a rule that ends a state matches
//! ([`crate::monotone::leads_to_an_end`]: `(false)`, in every program), or
//! either side of a pair a rule refutes ([`crate::contradiction::Pairs`]): the
//! side that holds the other, a stored negation where the program imports
//! `std.contradiction`, or a fact of a relation the holding side of which
//! some plan concludes or the KB holds ([`FailFirst::Pairs`], read off the
//! compiled plans); then the rest. Within each half the order
//! is FIFO, the written and arrival order. The boundary walks its parked
//! candidates in the same order. Root's saturation, every node a route or the
//! oracle computes, the whole node and every reader of a fixpoint take it, so
//! a dead node stopped at its first contradiction (below) costs as little of
//! its derivation as the order can make it. The order decides when a fact is
//! derived and which derivation a fact's record keeps first, so a core, the
//! rules beside it and the narration follow it; on a monotone program it
//! decides no fixpoint, and no answer.
//!
//! A saturation takes the agenda it is built under: [`Saturator::new`] the
//! engine's, [`Saturator::resume`] its snapshot's, unless a switch on the
//! calling thread says otherwise. [`fifo`] runs a closure with every
//! saturation built on the thread in the written order alone, as before the
//! agenda; [`fail_first`] with the given halves, `Negations` among them. They
//! are for the test that holds every answer alike under the written order and
//! the agenda, and for a measurement; no `(config …)` field, option of `ein`
//! or `EIN_*` name reaches them.
//!
//! **The agenda's tables are read once and kept** (`Tables`): which plans
//! fire at once, the plans of the rules that end a state with the relations
//! they read, and what decides the first half are functions of the plans,
//! but for the relations whose holding side the KB holds. A snapshot keeps
//! them, that set read over its facts, and a saturation resumed from it adds
//! only what its own facts add, so a state's order is the one it would read
//! over the whole KB, and no state builds them again. A plan compiled at a
//! state reads them all again there, over the KB as it stands.
//!
//! ### A saturation stopped at its first contradiction
//!
//! `Saturator::stop_at_first_contradiction` makes [`Saturator::saturate`]
//! return at the first point its KB holds a contradiction, before any firing
//! where the facts written before it already do and otherwise after the
//! firing that ends the state, or the one whose conclusions make a pair,
//! every conclusion of that firing written; the KB is then not marked
//! saturated. Before it returns it fires what the facts written so far give
//! a rule that ends a state (`Saturator::end_step`), matching those rules'
//! plans alone, so a stopped state holds every contradiction its facts make,
//! as its fixpoint would over the same facts. It is how a route's node on
//! a monotone program is saturated (`crate::region`'s § What a node is), and
//! it is no agenda: which candidate fires first is the agenda's.
//! [`stopping_at_first_contradiction`] is the same stop as a switch on the
//! calling thread, read when a saturation is built, for a measurement over
//! [`crate::region::Nodes`]' readers, which no route stops. Not taken,
//! `saturate` runs to the fixpoint.
//!
//! ### The plans a fact seeds
//!
//! A new fact is handed to the plans that read its relation at a top-level
//! positive premise (`PlanIndex`), in the plans' order. A premise that holds
//! a fact of a known relation at an argument, `(not (R ?a ?b))`, keys its
//! plan by that relation too, the participation index's key of it
//! ([`ein_core::SlotKey::relation`]), so a new `(not (R …))` is handed to the
//! plans that read a negation of `R` and to those whose premise leaves the
//! relation open, and to no other. The index only narrows what is offered:
//! a plan left out would have found no match, its premise not unifying, and
//! the plans offered come in the order the whole list gives them.
//!
//! ### A pair matched by a probe
//!
//! A plan whose match is a fact `F` beside a fact `(W F)` holding it as its
//! one argument, and nothing else, and whose firing ends a state
//! ([`crate::contradiction::PairShape`]), `std.contradiction`'s `clash-N`,
//! is never seeded, after a step or by the pass: a new fact of its relation
//! asks whether the state holds `(W F)`, a new `(W G)` whether it holds `G`,
//! one probe by the fact's id each, and only a hit builds the match, the one
//! a seeded match would build, queued at the place it would take. `W` is the
//! wrapper the rule's shape names, never a name the kernel knows. A plan
//! compiled at a state is matched whole once, as every plan is. The gate's
//! build holds each probe to a seeded match of the same fact.

use std::cell::Cell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::ControlFlow;
use std::sync::Arc;

use ein_core::{FactId, Kb, NafArg, NafRef, Prov, SlotKey, Symbol, Terms, Value};
use ein_ir::Ast;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::compile::{CompileError, SharedMemo};
use crate::contradiction::PairShape;
use crate::engine::Engine;
use crate::events::{self, Events};
use crate::firing::{BindingKey, Env, FireError, Firing, build_conclusion, fire};
use crate::match_::Matcher;
use crate::plan::{NafGuard, Plan, Reg, Slot, Span, Step};

/// Which rules a saturation fires first, after those that end a state: those
/// whose `:assert` writes, by [`crate::monotone::writes_of`]'s reading of the
/// compiled plan, a fact a contradiction can be made of.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailFirst {
    /// A fact a rule that ends a state matches, the side of a pair a rule
    /// refutes that holds the other, or a fact of a relation the holding side
    /// of which some plan concludes or the KB holds: either side of a pair.
    /// The engine's agenda, [`AGENDA`].
    Pairs,
    /// A fact a rule that ends a state matches, or the holding side of a
    /// pair, a stored negation under `std.contradiction`: the side a rule
    /// supplies against a fact already there. A measurement's.
    Negations,
}

/// **The engine's agenda** (the module's § The agenda): the rules whose
/// conclusion either side of a pair can be made of first, then the rest,
/// FIFO within each half.
pub const AGENDA: FailFirst = FailFirst::Pairs;

thread_local! {
    /// A switch's agenda on this thread: `Some(None)` the written order
    /// alone ([`fifo`]), `Some(Some(w))` `w`'s halves ([`fail_first`]);
    /// `None`, no switch, and a saturation takes the engine's or its
    /// snapshot's.
    static AGENDA_SWITCH: Cell<Option<Option<FailFirst>>> = const { Cell::new(None) };
    static STOP_FIRST: Cell<bool> = const { Cell::new(false) };
}

/// The bit a candidate's rank carries when its rule is not among the first:
/// above every tiebreaker a run reaches, so it sorts after every candidate
/// that is, and leaves FIFO within each half.
const LATE: u64 = 1 << 62;

/// The bit every candidate's rank carries but that of a rule that states
/// `:ein inconsistent` (the module's § A rule that ends a state): above every
/// tiebreaker a run reaches and below [`LATE`], so such a rule's candidate
/// sorts before every other, in either half and under the written order
/// alone.
const ORDINARY: u64 = 1 << 61;

/// The bits of a rank that are no tiebreaker.
const BAND: u64 = ORDINARY | LATE;

/// Run `f` with the agenda switch of this thread at `to`. Restored when `f`
/// returns or unwinds.
fn switching<T>(to: Option<FailFirst>, f: impl FnOnce() -> T) -> T {
    struct Restore(Option<Option<FailFirst>>);
    impl Drop for Restore {
        fn drop(&mut self) {
            AGENDA_SWITCH.with(|s| s.set(self.0));
        }
    }
    let _restore = Restore(AGENDA_SWITCH.with(|s| s.replace(Some(to))));
    f()
}

/// Run `f` with every saturation built on this thread, by [`Saturator::new`]
/// or [`Saturator::resume`], firing `which` rules first (the module's § The
/// agenda), whatever its snapshot's. Restored when `f` returns or unwinds.
pub fn fail_first<T>(which: FailFirst, f: impl FnOnce() -> T) -> T {
    switching(Some(which), f)
}

/// Run `f` with every saturation built on this thread, by [`Saturator::new`]
/// or [`Saturator::resume`], in the written and arrival order alone, the
/// order before the agenda (the module's § The agenda). Restored when `f`
/// returns or unwinds.
pub fn fifo<T>(f: impl FnOnce() -> T) -> T {
    switching(None, f)
}

/// The agenda a saturation built now on this thread by [`Saturator::new`]
/// takes: a switch's, else [`AGENDA`]. `None` is the written order alone.
pub fn is_fail_first() -> Option<FailFirst> {
    AGENDA_SWITCH.with(Cell::get).unwrap_or(Some(AGENDA))
}

/// Run `f` with every saturation built on this thread stopping
/// [`Saturator::saturate`] at its first contradiction (the module's § A
/// saturation stopped at its first contradiction). Restored when `f` returns
/// or unwinds.
pub fn stopping_at_first_contradiction<T>(f: impl FnOnce() -> T) -> T {
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            STOP_FIRST.with(|s| s.set(self.0));
        }
    }
    let _restore = Restore(STOP_FIRST.with(|s| s.replace(true)));
    f()
}

/// Whether [`stopping_at_first_contradiction`] is running on this thread.
pub fn is_stopping_at_first_contradiction() -> bool {
    STOP_FIRST.with(Cell::get)
}

/// Everything a saturation step reads and writes outside the saturator.
pub struct Session<'a> {
    pub kb: &'a mut Kb,
    pub terms: &'a mut Terms,
    pub ast: &'a Ast,
    pub events: &'a mut Events,
    /// The run's compiled plans, shared by every engine of the run.
    ///
    /// Every engine built from this session compiles into it, the
    /// saturator's and any other a caller builds over it. A caller that hands the
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

/// One queued or parked candidate, with the payload in a side arena so the
/// heap compares two integers
/// ([03-design/01-determinism-and-order.md § 3](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#3-ordered-containers)).
#[derive(Clone)]
struct Entry {
    plan: usize,
    disjunct: usize,
    regs: Box<[Value]>,
    trail: Box<[Reg]>,
    premises: Box<[FactId]>,
    /// This candidate's identity, built once at enqueue.
    ///
    /// Recomputing it at every dequeue and at every boundary round would build
    /// a fresh key per parked candidate per round — ~60 000 of them on a zebra
    /// root. The answer cannot change (the registers are a snapshot), so the
    /// only thing recomputing buys is the allocation.
    key: BindingKey,
    /// The interned identity of this candidate's guards — the boundary's
    /// invalidation unit.
    ///
    /// Every relation a candidate watches is a relation its *guard set*
    /// watches, and [`Disjunct::guard_key`](crate::plan::Disjunct::guard_key)
    /// encodes the watched symbols themselves, so two candidates that share an
    /// id watch exactly the same relations and go stale at exactly the same
    /// moment. That is what lets one epoch per guard set replace one stamp per
    /// candidate — [03-design/06-saturation.md § 3](../../../../docs/ein/ein.rs/03-design/06-saturation.md#3-the-queue-the-delta-and-the-parked-set).
    guard_set: GuardSetId,
}

/// What a match hands the queue, owned: the disjunct, the plan's registers,
/// the bindings' order and the premises. The gate's build compares what a
/// pair's probe queues with what a seeded match of the same fact finds
/// ([`Saturator::pair_match`]).
#[cfg(debug_assertions)]
#[derive(Debug, PartialEq)]
struct SeededMatch {
    disjunct: usize,
    regs: Vec<Value>,
    trail: Vec<Reg>,
    premises: Vec<FactId>,
}

#[cfg(debug_assertions)]
impl SeededMatch {
    fn of(m: &crate::match_::Match<'_>) -> SeededMatch {
        SeededMatch {
            disjunct: m.disjunct,
            regs: m.regs()[..m.plan.n_regs as usize].to_vec(),
            trail: m.trail().to_vec(),
            premises: m.premises().to_vec(),
        }
    }
}

/// The candidate arena, in two layers — the shape
/// [03-design/03-data-model.md](../../../../docs/ein/ein.rs/03-design/03-data-model.md) gives
/// the KB, for the same reason.
///
/// A fork *resumes* its parent's saturation
/// ([03-design/06-saturation.md § 4](../../../../docs/ein/ein.rs/03-design/06-saturation.md#4-a-node-resumes-roots-saturation)),
/// so it inherits every candidate the parent enqueued — and, before
/// [03-design/06-saturation.md § 3](../../../../docs/ein/ein.rs/03-design/06-saturation.md#3-the-queue-the-delta-and-the-parked-set),
/// deep-copied all of them once per entering: a `Vec` whose rows own three
/// boxed slices and a `BindingKey`, so four allocations per candidate,
/// **3.5 % of `zebra -e` in `Vec::clone` and 1.6 % more dropping it again**.
///
/// Nothing in the parent's half is ever written after enqueue — that was
/// [03-design/06-saturation.md § 3](../../../../docs/ein/ein.rs/03-design/06-saturation.md#3-the-queue-the-delta-and-the-parked-set)'s
/// side effect, moving `judged_at` out of the row — so the parent's half is
/// shared behind an `Arc` and the fork appends to a local vector. Entry ids
/// stay dense and stable: `base` first, then `local`.
#[derive(Clone, Default)]
struct Arena {
    base: Arc<Vec<Entry>>,
    local: Vec<Entry>,
}

impl Arena {
    #[inline]
    fn get(&self, i: u32) -> &Entry {
        let n = self.base.len();
        let i = i as usize;
        if i < n {
            &self.base[i]
        } else {
            &self.local[i - n]
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.base.len() + self.local.len()
    }

    fn push(&mut self, e: Entry) {
        self.local.push(e);
    }

    /// One flat vector of everything, for a snapshot a fork will share.
    ///
    /// Paid once per *snapshot* — one per root saturation — against the
    /// per-entering copy it removes, of which `zebra -e` has 111 and
    /// `features/05 -e` 384 167.
    fn flattened(&self) -> Arc<Vec<Entry>> {
        if self.local.is_empty() {
            return self.base.clone();
        }
        let mut all = Vec::with_capacity(self.len());
        all.extend_from_slice(&self.base);
        all.extend_from_slice(&self.local);
        Arc::new(all)
    }
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

/// `(tiebreaker, entry)`: the order candidates were enqueued in, FIFO. The
/// tiebreaker is unique and monotone, so the comparison never reaches the
/// second field.
type Ranked = Reverse<(u64, u32)>;

/// An interned guard tuple — the `seen` key's second half.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct GuardSetId(u32);

pub struct Saturator {
    pub engine: Engine,
    matcher: Matcher,
    entries: Arena,
    queue: BinaryHeap<Ranked>,
    /// The parked candidates, in FIFO order.
    ///
    /// A `BTreeSet` rather than a heap because a boundary round **reads** the
    /// whole set in order and removes at most one entry from it. Popping every
    /// candidate and re-pushing the rejects would be ~60 000 heap operations
    /// across a zebra root's run for ~1 000 actual judgements. Ordered
    /// iteration gives the same sequence — so "the first candidate
    /// whose guards pass" is the same candidate — without touching the ones
    /// it skips.
    parked: std::collections::BTreeSet<(u64, u32)>,
    seen: FxHashSet<(BindingKey, GuardSetId)>,
    guard_sets: FxHashMap<Box<[u32]>, GuardSetId>,
    tiebreaker: u64,
    needs_enqueue: bool,
    delta: Option<Vec<FactId>>,
    matched_plans: Vec<bool>,
    /// Every plan below it has been matched whole ([`Self::matched_plans`]):
    /// where a pass looks for a plan never matched, it starts here, and not
    /// at the first plan of the list. 0 where a saturation is built.
    matched_upto: usize,
    /// The same of [`Self::end_plans`], for [`Saturator::match_ends`]: those
    /// below it have been matched whole.
    ends_matched: usize,
    /// The plans a new fact is handed to, by its relation and the relation of
    /// a fact it holds ([`PlanIndex`]); rebuilt when the plan list grows,
    /// shared with a snapshot and every saturation resumed from it.
    pos_index: Arc<PlanIndex>,
    index_n: usize,
    /// Scratch: the plans [`PlanIndex::plans_for`] gives one fact.
    offered: Vec<usize>,
    /// The boundary's clock: one tick per round, and what a judgement is
    /// stamped with — [`Saturator::refresh_epochs`].
    epoch: u32,
    /// `judged_at[e]` is the epoch at which entry `e` was last judged and
    /// *failed*, or 0 for a candidate that has never been judged.
    ///
    /// A side table rather than a field on [`Entry`] because a fork inherits
    /// the arena and re-judges against its own world: this is the only part of
    /// a candidate that a saturation writes after enqueue, and separating it
    /// is what [03-design/06-saturation.md § 3](../../../../docs/ein/ein.rs/03-design/06-saturation.md#3-the-queue-the-delta-and-the-parked-set)
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
    /// Scratch: what a boundary round takes out of `parked`, applied after the
    /// walk so the walk can hold the set.
    retired_scratch: Vec<(u64, u32)>,
    /// Can a parked candidate's binding key reach `fired`? — see
    /// [`Saturator::refresh_collision_risk`].
    may_collide: bool,
    /// The plan count `may_collide` was last computed at.
    plans_seen: usize,
    record_alternatives: bool,
    /// Keep what each application's universals read, a **redundant** one's
    /// among them, for [`Saturator::take_universal`]. Off on every route: a
    /// redundant firing is the highest-volume path, and only a reader of a
    /// node's whole derivation asks ([`Saturator::read_universals`]).
    reading_universals: bool,
    /// What the last application's universals read, where they are kept.
    last_universal: Box<[FactId]>,
    last_firing: Option<Firing>,
    /// The agenda this saturation was built under (the module's § The
    /// agenda); `None`, the written order alone, every rank its tiebreaker.
    fail_first: Option<FailFirst>,
    /// Under [`Self::fail_first`]: per plan, whether its rule fires first,
    /// for every plan the engine holds ([`Saturator::refresh_first`]).
    first: Vec<bool>,
    /// [`stopping_at_first_contradiction`] as it stood when this saturation
    /// was built, or `Saturator::stop_at_first_contradiction` since.
    stop_first: bool,
    /// Per plan, whether its candidate fires before every other, under the
    /// agenda and under the written order alone: its rule states
    /// `:ein inconsistent`, or refutes a fact beside one holding it, alone
    /// ([`crate::contradiction::Pairs::refutes_at`]), so a state ends as soon
    /// as it holds what such a rule reads ([`Saturator::refresh_at_once`]).
    at_once: Vec<bool>,
    /// Per plan, where its match is a pair and nothing else, the pair's
    /// shape ([`crate::contradiction::Pairs::shape_at`]): the plan is matched
    /// by a probe and never seeded (the module's § A pair matched by a
    /// probe). Read with [`Self::at_once`].
    pair_at: Vec<Option<PairShape>>,
    /// The plans of rules that fire at once ([`Self::at_once`]), in the engine's
    /// order, and an index of them as [`Self::pos_index`] is of every plan;
    /// read by [`Saturator::match_ends`] alone, and computed at `ends_n` plans.
    end_plans: Vec<usize>,
    end_index: Arc<PlanIndex>,
    ends_n: usize,
    /// Some rule that states `:ein inconsistent` has parameters, so a derived
    /// activator can give it a plan.
    open_ends: bool,
    /// How much of the pending pass [`Saturator::match_ends`] has matched
    /// for those plans: `None`, nothing; else the count of the delta's facts
    /// it seeded them with.
    ends_seeded: Option<usize>,

    // ── Observables ────────────────────────────────────────────────
    pub naf_rounds: u32,
    pub naf_admitted: u32,
    pub naf_retired: u32,
    /// Guard sub-plan **evaluations** — how many times a negative query was
    /// actually run. The boundary's cost is this number times the cost of one
    /// query, and 72 % of an exhaustive solve sits under it
    /// ([03-design/06-saturation.md § 3](../../../../docs/ein/ein.rs/03-design/06-saturation.md#3-the-queue-the-delta-and-the-parked-set)), so
    /// it is the figure Win B is measured in. It is the engine's own
    /// instrument, and it is deliberately *not* in the event stream — a number
    /// that must not change is not evidence that nothing changed.
    pub guard_evals: u64,
    /// Of those, the ones on a **monotone** guard — the only ones the
    /// semi-naive re-evaluation of
    /// [03-design/06-saturation.md § 3](../../../../docs/ein/ein.rs/03-design/06-saturation.md#3-the-queue-the-delta-and-the-parked-set)
    /// can help, since a nested absent can flip from failing to passing and
    /// has no "only a new fact can change this" argument.
    pub guard_evals_monotone: u64,
    /// Nanoseconds spent inside the boundary — one `Instant` pair per
    /// quiescence, which is 40 of them on a zebra2 root and therefore free.
    ///
    /// It exists because the boundary's *share* of a solve is what an
    /// optimisation of the boundary is aimed at, and a share is a
    /// measurement, not an assumption.
    pub boundary_nanos: u64,
    /// Structurally 0, kept as an observable `naf_semantics` asserts: firings
    /// dropped at dequeue because a guard that passed at enqueue no longer
    /// held at fire time. A guard is judged once, on the boundary, at the
    /// moment the candidate is admitted.
    pub naf_dropped: u32,
}

/// A saturation at its fixpoint, in a form another saturation can continue
/// from — [03-design/06-saturation.md § 4](../../../../docs/ein/ein.rs/03-design/06-saturation.md#4-a-node-resumes-roots-saturation).
///
/// The closure is semi-naive *within* a saturation (`pos_index` + `run_seeded`)
/// and abandons that at the one boundary where the delta is smallest and known
/// exactly: `try_commitment_set` forks the saturated root and builds a **fresh**
/// `Saturator`, whose `delta = None` is a FULL pass, so the parent's whole
/// deductive closure is re-derived inside the fork — 94.6 % of a fork's firings
/// on `zebra -e` ([03-design/06-saturation.md § 4](../../../../docs/ein/ein.rs/03-design/06-saturation.md#4-a-node-resumes-roots-saturation)).
///
/// What is carried is everything that answers "has this already been done":
/// the plan list *in its order*, `fired`, `seen`, the candidate arena and the
/// parked set with its watch stamps. What is **not** carried is the run's own
/// account of itself — the observables and `last_firing` — because the fork's
/// narration is its own.
///
/// `facts` is the parent's fact set at the snapshot. It is what makes the
/// resumed delta computable without enumerating every site that writes to root
/// (none does today; the walk's forced positives, singleton `(not h)`
/// writebacks and lookahead kill cache did): the fork's delta is *whatever root
/// has that the snapshot did not*, plus the commitment. A site added later is
/// covered by construction.
#[derive(Clone)]
pub struct Snapshot {
    engine: Engine,
    entries: Arc<Vec<Entry>>,
    queue: BinaryHeap<Ranked>,
    parked: std::collections::BTreeSet<(u64, u32)>,
    seen: FxHashSet<(BindingKey, GuardSetId)>,
    guard_sets: FxHashMap<Box<[u32]>, GuardSetId>,
    tiebreaker: u64,
    matched_plans: Vec<bool>,
    pos_index: Arc<PlanIndex>,
    index_n: usize,
    /// The boundary's clock and what it has judged — carried so a fork
    /// re-judges exactly the parked candidates whose watched relations its own
    /// delta grew, and skips the rest for the same reason the parent did.
    epoch: u32,
    judged_at: Vec<u32>,
    gs_epoch: Vec<u32>,
    watch: Arc<WatchTables>,
    watched_sizes: Vec<usize>,
    may_collide: bool,
    plans_seen: usize,
    facts: FxHashSet<FactId>,
    /// The agenda the saturation was built under, which a resumed one takes
    /// on whatever thread it runs (the module's § The agenda).
    fail_first: Option<FailFirst>,
    /// A saved file's node ([`crate::region::Floor`]), whose commitment every
    /// fork of this snapshot is written with beside its own: the node
    /// function reads it and the saturator does not. `None` at a program's
    /// root.
    floor: Option<Arc<crate::region::Floor>>,
    /// What the pass that writes a state's `open` facts writes at every
    /// state above root, read at root ([`crate::open::Writers`]): the node
    /// function reads it and the saturator does not. `None` where root's
    /// were not read, and then a node writes none.
    open: Option<crate::open::Shared>,
    /// The agenda's tables over the snapshot's plans, read once, which a
    /// saturation that resumes from it takes rather than building them again
    /// ([`Tables`]).
    tables: Arc<Tables>,
}

/// **The agenda's tables over a set of plans** (the module's § The agenda):
/// which plans fire at once, the plans of rules that end a state and the
/// relations each reads, and what decides which plans fire first. Every one
/// of them is a function of the plans but one set, the relations whose
/// holding side the knowledge base holds, so a snapshot keeps them, that set
/// read over its own facts, and a saturation that resumes from it adds to
/// that set only what its own facts add ([`Saturator::resume`]). A plan
/// compiled after the snapshot reads them all again, as before.
#[derive(Clone, Default)]
struct Tables {
    /// How many plans they were read over: the engine's length.
    n: usize,
    /// [`Saturator::at_once`]'s and [`Saturator::pair_at`]'s.
    at_once: Vec<bool>,
    pair_at: Vec<Option<PairShape>>,
    /// [`Saturator::end_plans`]' and [`Saturator::end_index`]'s.
    end_plans: Vec<usize>,
    end_index: Arc<PlanIndex>,
    /// What [`Saturator::first`] is read off.
    first: FirstTables,
}

/// **The plans a new fact is handed to** (the module's § The plans a fact
/// seeds), each list in the plans' order: by the relation of a top-level
/// positive premise, and, where every premise of that relation in a plan
/// holds a fact at some argument, by the relation of the first such fact
/// too, the participation index's key of it ([`SlotKey::relation`]). So a new
/// `(not (R …))` reaches the plans that read a negation of `R` and those
/// whose premise leaves the relation open, `(not ?x)`, and no other.
#[derive(Clone, Default)]
struct PlanIndex {
    /// Relation → the plans with a premise of it, every fact of which can
    /// seed them ([`Seeds::open`]), and whether some plan is keyed by the
    /// relation of a fact at an argument of it.
    by_rel: FxHashMap<Symbol, Seeds>,
    /// `(relation, argument, RELATION, the relation of the fact there)` →
    /// the plans keyed by it.
    by_nested: FxHashMap<SlotKey, Vec<usize>>,
}

#[derive(Clone, Default)]
struct Seeds {
    /// The plans with a premise of the relation that holds no fact at any
    /// argument, in the plans' order.
    open: Vec<usize>,
    /// Some plan is keyed in [`PlanIndex::by_nested`] under the relation.
    keyed: bool,
}

impl PlanIndex {
    /// The index of the plans `plans`, ascending, of `engine`.
    fn of(engine: &Engine, plans: impl IntoIterator<Item = usize>) -> PlanIndex {
        let mut index = PlanIndex::default();
        for at in plans {
            let plan = engine.plan(at);
            // Each relation the plan reads at a top-level premise, with the
            // keys of the relations of the facts its premises hold, or none
            // where a premise of it holds none.
            let mut rels: Vec<(Symbol, Option<Vec<SlotKey>>)> = Vec::new();
            for d in plan.disjuncts.iter() {
                for step in plan.steps(d.steps) {
                    let Step::Rel(r) = step else { continue };
                    let key = plan
                        .slots(r.slots)
                        .iter()
                        .zip(0u16..)
                        .find_map(|(slot, i)| match *slot {
                            Slot::Nested { rel, .. } => Some(SlotKey::relation(r.rel, i, rel)),
                            _ => None,
                        });
                    match rels.iter_mut().find(|(rel, _)| *rel == r.rel) {
                        None => rels.push((r.rel, key.map(|k| vec![k]))),
                        Some((_, keys)) => match (keys.as_mut(), key) {
                            (Some(keys), Some(k)) => {
                                if !keys.contains(&k) {
                                    keys.push(k);
                                }
                            }
                            _ => *keys = None,
                        },
                    }
                }
            }
            for (rel, keys) in rels {
                let seeds = index.by_rel.entry(rel).or_default();
                match keys {
                    None => seeds.open.push(at),
                    Some(keys) => {
                        seeds.keyed = true;
                        for k in keys {
                            index.by_nested.entry(k).or_default().push(at);
                        }
                    }
                }
            }
        }
        index
    }

    /// The plans `fact` can seed, into `out`: in the plans' order, each once.
    fn plans_for(&self, terms: &Terms, fact: FactId, out: &mut Vec<usize>) {
        out.clear();
        let (rel, args) = terms.facts.get(fact);
        let Some(seeds) = self.by_rel.get(&rel) else {
            return;
        };
        out.extend_from_slice(&seeds.open);
        if !seeds.keyed {
            return;
        }
        let mut lists = usize::from(!seeds.open.is_empty());
        for (arg, slot) in args.iter().zip(0u16..) {
            if let Some(held) = arg.as_fact()
                && let Some(plans) =
                    self.by_nested
                        .get(&SlotKey::relation(rel, slot, terms.facts.rel(held)))
            {
                out.extend_from_slice(plans);
                lists += 1;
            }
        }
        if lists > 1 {
            out.sort_unstable();
            out.dedup();
        }
    }
}

/// What decides which plans fire first under an agenda, all of it the
/// plans' but [`Self::held`] ([`Saturator::refresh_first`]).
#[derive(Clone, Default)]
struct FirstTables {
    /// Per plan: it concludes the holding side of a pair.
    holding: Vec<bool>,
    /// Per plan: it concludes a fact a rule that ends a state matches.
    leads: Vec<bool>,
    /// Per plan: the relations it concludes facts of.
    writes: Vec<Vec<Symbol>>,
    /// What the plans refute beside a fact.
    pairs: crate::contradiction::Pairs,
    /// The relations the held side of a pair can be of by the plans: those a
    /// plan concludes the holding side of, and every one a wrapper reads where
    /// a plan concludes it of a fact it does not name.
    planned: FxHashSet<Symbol>,
    /// Those the knowledge base holds the holding side of, read over the
    /// snapshot's facts outside its `open` layer, which no fork inherits.
    held: FxHashSet<Symbol>,
}

impl FirstTables {
    /// The tables of `plans`, [`Self::held`] empty.
    fn of(plans: &[&Plan], terms: &Terms) -> FirstTables {
        // What a rule that states `:ein inconsistent` matches: a rule that
        // concludes a fact of it leads to the end of a state.
        let leads_to = crate::monotone::leads_to_an_end(plans.iter().copied(), terms);
        // The rules that refute a fact beside one holding it, and each
        // plan's conclusions that are the holding side of such a pair.
        let pairs = crate::contradiction::Pairs::of(plans.iter().copied(), terms);
        let mut leads = Vec::with_capacity(plans.len());
        let mut writes = Vec::with_capacity(plans.len());
        for p in plans {
            let ws = crate::monotone::writes_of(p, terms);
            leads.push(ws.iter().any(|w| leads_to.contains(w)));
            writes.push(ws.iter().map(|w| w.name).collect());
        }
        let holding = plans
            .iter()
            .map(|p| {
                p.asserts
                    .iter()
                    .any(|&slot| pairs.holding_side(p, slot).is_some())
            })
            .collect();
        // determinism-ok: a set asked for membership alone.
        let mut planned: FxHashSet<Symbol> = FxHashSet::default();
        for p in plans {
            for &slot in p.asserts.iter() {
                match pairs.holding_side(p, slot) {
                    Some((_, Some(rel))) => {
                        planned.insert(rel);
                    }
                    Some((wrapper, None)) => {
                        planned.extend(pairs.wrapped_by(wrapper).into_iter().flatten());
                    }
                    None => {}
                }
            }
        }
        FirstTables {
            holding,
            leads,
            writes,
            pairs,
            planned,
            held: FxHashSet::default(),
        }
    }

    /// Add to `held` the relation of each of `facts` that holds the other
    /// side of a pair the plans refute: `(N F)`, `F` of a relation `N` is
    /// read with.
    fn hold(
        &self,
        facts: impl IntoIterator<Item = FactId>,
        terms: &Terms,
        held: &mut FxHashSet<Symbol>,
    ) {
        for f in facts {
            let wrapper = terms.facts.rel(f);
            if self.pairs.wrapped_by(wrapper).is_none() {
                continue;
            }
            if let [inner] = terms.facts.args(f)
                && let Some(inner) = inner.as_fact()
            {
                let rel = terms.facts.rel(inner);
                if self.pairs.pairs(wrapper, rel) {
                    held.insert(rel);
                }
            }
        }
    }

    /// Per plan, whether its rule fires first under `which`, `held` the
    /// relations whose holding side the knowledge base holds.
    fn first(&self, which: FailFirst, held: &FxHashSet<Symbol>) -> Vec<bool> {
        self.writes
            .iter()
            .zip(&self.holding)
            .zip(&self.leads)
            .map(|((ws, &holds), &leads)| {
                holds
                    || leads
                    || (which == FailFirst::Pairs
                        && ws
                            .iter()
                            .any(|w| held.contains(w) || self.planned.contains(w)))
            })
            .collect()
    }
}

/// The plans of rules that fire at once, `at_once`'s, in the engine's order,
/// and their index: what [`Saturator::match_ends`] matches after each step.
fn ends_of(engine: &Engine, at_once: &[bool]) -> (Vec<usize>, Arc<PlanIndex>) {
    let end_plans: Vec<usize> = (0..at_once.len()).filter(|&at| at_once[at]).collect();
    let end_index = Arc::new(PlanIndex::of(engine, end_plans.iter().copied()));
    (end_plans, end_index)
}

impl Snapshot {
    /// The saved file's node every fork of this snapshot holds, if any.
    pub(crate) fn floor(&self) -> Option<&crate::region::Floor> {
        self.floor.as_deref()
    }

    /// The same snapshot, every fork of it written with `floor`'s
    /// commitment beside its own.
    pub(crate) fn above(mut self, floor: crate::region::Floor) -> Snapshot {
        self.floor = Some(Arc::new(floor));
        self
    }

    /// The same snapshot, every fork of it given its `open` facts by
    /// `writers` at its fixpoint.
    pub(crate) fn with_open(mut self, writers: crate::open::Shared) -> Snapshot {
        self.open = Some(writers);
        self
    }

    /// The writers of a state's `open` facts every fork of it reads.
    pub(crate) fn open(&self) -> Option<&crate::open::Shared> {
        self.open.as_ref()
    }

    /// The facts `kb` has and the snapshot did not — the first half of a
    /// resumed fork's delta — in the order `kb` holds them, which is the
    /// order the caller wrote them: every route writes a commitment in
    /// candidate order, rendered text.
    ///
    /// Not `FactId` order: a `FactId` is the order names were interned in,
    /// not a function of the program. A fork is FIFO within each half of the
    /// agenda, so the delta's order is the order a dead node's derivations
    /// arrive in, and what reads the first of them — a fact's primary (the
    /// store's order `refuted_by` names rules in, the `absent`s a refutation
    /// rests on) and a full alternatives list (the core) — would move with the
    /// interner (`fork-delta-order`).
    pub fn new_facts_of(&self, kb: &Kb) -> Vec<FactId> {
        kb.facts().filter(|f| !self.facts.contains(f)).collect()
    }
}

impl Saturator {
    pub fn new(s: &mut Session<'_>) -> Result<Saturator, SaturateError> {
        let cfg = s.kb.program().config.clone().unwrap_or_default();
        let mut sat = Saturator {
            engine: Engine::with_memo(s.memo.clone()),
            matcher: Matcher::new(),
            entries: Arena::default(),
            queue: BinaryHeap::new(),
            parked: std::collections::BTreeSet::new(),
            seen: FxHashSet::default(),
            guard_sets: FxHashMap::default(),
            tiebreaker: 0,
            needs_enqueue: true,
            delta: None,
            matched_plans: Vec::new(),
            matched_upto: 0,
            ends_matched: 0,
            pos_index: Arc::default(),
            index_n: usize::MAX,
            offered: Vec::new(),
            epoch: 0,
            judged_at: Vec::new(),
            gs_epoch: Vec::new(),
            watch: Arc::new(WatchTables::default()),
            watched_sizes: Vec::new(),
            watched_grew: Vec::new(),
            retired_scratch: Vec::new(),
            may_collide: false,
            plans_seen: usize::MAX,
            // Read once from the resolved config: it is consulted on the
            // redundant-firing path, the highest-volume path in the engine.
            record_alternatives: cfg.record_alternative_justifications,
            reading_universals: false,
            last_universal: Box::default(),
            last_firing: None,
            fail_first: is_fail_first(),
            first: Vec::new(),
            stop_first: is_stopping_at_first_contradiction(),
            at_once: Vec::new(),
            pair_at: Vec::new(),
            end_plans: Vec::new(),
            end_index: Arc::default(),
            ends_n: usize::MAX,
            open_ends: s.kb.program().rules.values().any(|r| !r.params.is_empty()),
            ends_seeded: None,
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
    pub fn snapshot(&self, kb: &Kb, terms: &Terms) -> Snapshot {
        Snapshot {
            engine: self.engine.clone(),
            entries: self.entries.flattened(),
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
            may_collide: self.may_collide,
            plans_seen: self.plans_seen,
            facts: kb.facts().collect(),
            fail_first: self.fail_first,
            floor: None,
            open: None,
            tables: Arc::new(self.tables(kb, terms)),
        }
    }

    /// The agenda's tables over the engine's plans, [`FirstTables::held`]
    /// read over `kb`'s facts outside its `open` layer: what a snapshot of
    /// this saturation keeps.
    fn tables(&self, kb: &Kb, terms: &Terms) -> Tables {
        let n = self.engine.len();
        let plans: Vec<&Plan> = (0..n).map(|at| self.engine.plan(at)).collect();
        let mut first = FirstTables::of(&plans, terms);
        let at_once: Vec<bool> = (0..n)
            .map(|at| plans[at].ends || first.pairs.refutes_at(at))
            .collect();
        let pair_at: Vec<Option<PairShape>> = (0..n).map(|at| first.pairs.shape_at(at)).collect();
        let (end_plans, end_index) = ends_of(&self.engine, &at_once);
        let mut held = FxHashSet::default();
        // determinism-ok: what is read is collected into a set.
        for wrapper in first.pairs.wrappers() {
            first.hold(
                kb.facts_of(wrapper).filter(|&f| !kb.holds_open(f)),
                terms,
                &mut held,
            );
        }
        first.held = held;
        Tables {
            n,
            at_once,
            pair_at,
            end_plans,
            end_index,
            first,
        }
    }

    /// Continue `snapshot`'s saturation over `s.kb`, seeded from `delta`.
    ///
    /// The counterpart of [`Saturator::new`], and the difference is the whole
    /// point: `new` starts with `delta = None`, which is a FULL enqueue pass
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
        // The agenda's tables, the snapshot's, where they were read over the
        // plans this saturation starts with.
        let tables = &snapshot.tables;
        let kept = tables.n == snapshot.engine.len();
        // The relations whose holding side this KB holds: the snapshot's,
        // and what this state's own facts add, those beyond the snapshot's
        // and its `open` layer, which no fork inherits.
        let held = kept.then(|| {
            let mut held = tables.first.held.clone();
            tables
                .first
                .hold(s.kb.open_facts().iter().copied(), s.terms, &mut held);
            tables.first.hold(delta.iter().copied(), s.terms, &mut held);
            held
        });
        let mut sat = Saturator {
            engine: snapshot.engine.clone(),
            matcher: Matcher::new(),
            entries: Arena {
                base: snapshot.entries.clone(),
                local: Vec::new(),
            },
            queue: snapshot.queue.clone(),
            parked: snapshot.parked.clone(),
            seen: snapshot.seen.clone(),
            guard_sets: snapshot.guard_sets.clone(),
            tiebreaker: snapshot.tiebreaker,
            needs_enqueue: true,
            delta: Some(delta),
            matched_plans: snapshot.matched_plans.clone(),
            matched_upto: 0,
            ends_matched: 0,
            pos_index: snapshot.pos_index.clone(),
            index_n: snapshot.index_n,
            offered: Vec::new(),
            epoch: snapshot.epoch,
            judged_at: snapshot.judged_at.clone(),
            gs_epoch: snapshot.gs_epoch.clone(),
            watch: snapshot.watch.clone(),
            watched_sizes: snapshot.watched_sizes.clone(),
            watched_grew: vec![false; snapshot.watch.rels.len()],
            retired_scratch: Vec::new(),
            may_collide: snapshot.may_collide,
            plans_seen: snapshot.plans_seen,
            record_alternatives: cfg.record_alternative_justifications,
            reading_universals: false,
            last_universal: Box::default(),
            last_firing: None,
            fail_first: AGENDA_SWITCH.with(Cell::get).unwrap_or(snapshot.fail_first),
            first: Vec::new(),
            stop_first: is_stopping_at_first_contradiction(),
            at_once: if kept {
                tables.at_once.clone()
            } else {
                Vec::new()
            },
            pair_at: if kept {
                tables.pair_at.clone()
            } else {
                Vec::new()
            },
            end_plans: if kept {
                tables.end_plans.clone()
            } else {
                Vec::new()
            },
            end_index: if kept {
                tables.end_index.clone()
            } else {
                Arc::default()
            },
            ends_n: if kept { tables.n } else { usize::MAX },
            open_ends: s.kb.program().rules.values().any(|r| !r.params.is_empty()),
            ends_seeded: None,
            naf_rounds: 0,
            naf_admitted: 0,
            naf_retired: 0,
            naf_dropped: 0,
            guard_evals: 0,
            guard_evals_monotone: 0,
            boundary_nanos: 0,
        };
        sat.refresh_at_once(s);
        if sat.fail_first.is_some() || snapshot.fail_first.is_some() {
            // What the snapshot holds was ranked under its own agenda, and
            // the KB this saturation reads can make another plan's
            // conclusion one side of a pair: ranked again, so the order is
            // this saturation's throughout (under the written order alone,
            // every rank its tiebreaker).
            match (sat.fail_first, &held) {
                (Some(which), Some(held)) => sat.first = tables.first.first(which, held),
                _ => sat.refresh_first(s),
            }
            let parked = std::mem::take(&mut sat.parked);
            sat.parked = parked
                .into_iter()
                .map(|(tb, e)| ((tb & !BAND) | sat.band(sat.entries.get(e).plan), e))
                .collect();
            let queue = std::mem::take(&mut sat.queue);
            sat.queue = queue
                .into_iter()
                .map(|Reverse((tb, e))| {
                    Reverse(((tb & !BAND) | sat.band(sat.entries.get(e).plan), e))
                })
                .collect();
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
                // **The fixpoint, and the only place that says so.** The
                // clean mark, which the record sites read to find
                // out whether the state they are about to call a solution has
                // had its rules run over its current facts. It is set here
                // rather than in `saturate` because the fail-fast fork loop
                // drives `step` itself — marking one level up would have made
                // every fork it saturates look dirty forever, which is a check
                // that always fires and therefore checks nothing.
                s.kb.mark_saturated();
                return Ok(None);
            }
        }
    }

    /// Run to the fixpoint, handing each firing to `f`.
    ///
    /// `max_steps` is the runaway-debugging budget; `None` runs to the
    /// fixpoint. After `Self::stop_at_first_contradiction`, or built under
    /// [`stopping_at_first_contradiction`], it returns at the KB's first
    /// contradiction instead, short of the fixpoint, which it then does not
    /// mark.
    pub fn saturate(
        &mut self,
        s: &mut Session<'_>,
        max_steps: Option<usize>,
        f: &mut dyn FnMut(&Firing),
    ) -> Result<usize, SaturateError> {
        let mut i = 0;
        if self.stop_first && s.kb.is_inconsistent() {
            // Before any firing, but those that end the state over the
            // facts already written.
            while let Some(firing) = self.end_step(s)? {
                f(&firing);
                self.last_firing = Some(firing);
                i += 1;
            }
            return Ok(i);
        }
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
                // `step` marks the KB clean when it reaches the fixpoint.
                None => return Ok(i),
                Some(firing) => {
                    f(&firing);
                    let wrote = !firing.redundant && !firing.ends;
                    let mut stop = self.stop_first && firing.ends;
                    self.last_firing = Some(firing);
                    i += 1;
                    if self.stop_first && (stop || wrote) {
                        // What the facts written so far give a rule that
                        // states `:ein inconsistent` fires now, so the state
                        // holds every contradiction its facts make.
                        while let Some(firing) = self.end_step(s)? {
                            f(&firing);
                            self.last_firing = Some(firing);
                            i += 1;
                            stop = true;
                        }
                    }
                    if stop {
                        return Ok(i);
                    }
                }
            }
        }
    }

    /// From here on [`Self::saturate`] returns at the KB's first
    /// contradiction, as a saturation built under
    /// [`stopping_at_first_contradiction`] does: how a route's node on a
    /// monotone program is saturated (`crate::region`'s § What a node is).
    /// Which candidate fires first is the agenda's, which this does not touch.
    pub(crate) fn stop_at_first_contradiction(&mut self) {
        self.stop_first = true;
    }

    /// From here on [`Self::saturate`] runs to the fixpoint, a saturation
    /// stopped at its first contradiction going on from where it stopped,
    /// firing for firing what it would have fired had it not stopped: how a
    /// route's stopped node gives its core over its fixpoint
    /// (`crate::region`'s § What a node is).
    pub(crate) fn run_to_quiescence(&mut self) {
        self.stop_first = false;
    }

    /// Facts written into the KB outside the saturation, a state's `open`
    /// facts the pass wrote at its fixpoint ([`crate::open`]): the next
    /// round's delta, so a rule that reads one fires on it, and a parked
    /// candidate that reads its relation is judged again.
    pub(crate) fn feed(&mut self, facts: &[FactId]) {
        match &mut self.delta {
            Some(d) => d.extend_from_slice(facts),
            None => self.delta = Some(facts.to_vec()),
        }
        self.needs_enqueue = true;
    }

    /// How many `(binding_key, guards)` pairs the dedup has seen.
    pub fn n_seen(&self) -> usize {
        self.seen.len()
    }

    /// From here on the facts each application's universals read are kept
    /// for [`Self::take_universal`], a redundant application's too, which
    /// no record holds. It changes nothing that is derived or recorded: the
    /// store's alternative justifications are taken as before.
    pub(crate) fn read_universals(&mut self) {
        self.reading_universals = true;
    }

    /// What the universals of the application [`Self::step`] last returned
    /// read ([`ein_core::Prov::universal`]), once: empty wherever
    /// [`Self::read_universals`] was not asked.
    pub(crate) fn take_universal(&mut self) -> Box<[FactId]> {
        std::mem::take(&mut self.last_universal)
    }

    pub fn last_firing(&self) -> Option<&Firing> {
        self.last_firing.as_ref()
    }

    /// Record the firing a caller driving `step` itself just consumed.
    ///
    /// `saturate` does this internally; a caller that drives `step` directly,
    /// to stop at the firing that kills a branch, records it here, and the
    /// runaway-budget message reads `last_firing`.
    pub fn set_last_firing(&mut self, firing: Firing) {
        self.last_firing = Some(firing);
    }

    /// True iff no firing is available — at the **two-phase** fixpoint, not
    /// merely at closure quiescence.
    ///
    /// **Asking costs something, and the cost is ordering.** A fresh FULL
    /// enqueue pass runs first, because a caller may have written facts
    /// straight to the KB outside `step`'s flow and a stalled-looking queue
    /// would then be a wrong answer rather than a cheap one. That pass
    /// **advances the tiebreaker**, which is the FIFO position of every
    /// candidate enqueued after it — so a run that asks and a run that does
    /// not can fire the same rules in a different order. That is the
    /// behaviour, and it is not free.
    ///
    /// So an embedder probing quiescence mid-drive is perturbing the drive.
    /// [`docs/ein/ein.rs/04-api.md`] says so where a reader of the crate boundary will
    /// meet it, because a hazard whose only statement is a comment inside the
    /// crate is a hazard for everyone outside it.
    ///
    /// It is at least **idempotent with respect to the queue**:
    /// the pending `delta` is consumed here rather than left for a later
    /// `closure_step` to re-seed. Re-seeding was harmless — `seen` dedups it —
    /// and it was work done twice for a question that asked nothing.
    ///
    /// [`docs/ein/ein.rs/04-api.md`]: `docs/ein/ein.rs/04-api.md`
    pub fn is_stalled(&mut self, s: &mut Session<'_>) -> Result<bool, SaturateError> {
        // The FULL pass subsumes whatever the delta would have seeded, so
        // taking it loses nothing and leaves no stale seed behind.
        let _ = self.delta.take();
        self.ends_seeded = None;
        self.enqueue_pass(s, None)?;
        self.needs_enqueue = false;
        let unfired = self
            .queue
            .iter()
            .any(|Reverse((_, e))| !self.engine.fired.contains(self.entry_key(*e)));
        if unfired {
            return Ok(false);
        }
        Ok(self.admit_from_boundary(s)? == 0)
    }

    // ── The closure ────────────────────────────────────────────────

    fn closure_step(&mut self, s: &mut Session<'_>) -> Result<Option<Firing>, SaturateError> {
        if self.needs_enqueue {
            let delta = self.delta.take();
            self.ends_seeded = None;
            self.enqueue_pass(s, delta)?;
            self.needs_enqueue = false;
        }
        self.fire_next(s, false)
    }

    /// Pop and apply the queue's next candidate that has not fired; with
    /// `ends_only`, only while the next is of a rule that states
    /// `:ein inconsistent`, which sorts before every other ([`ORDINARY`]).
    fn fire_next(
        &mut self,
        s: &mut Session<'_>,
        ends_only: bool,
    ) -> Result<Option<Firing>, SaturateError> {
        loop {
            match self.queue.peek() {
                None => return Ok(None),
                Some(Reverse((rank, _))) if ends_only && rank & ORDINARY != 0 => return Ok(None),
                Some(_) => {}
            }
            let Some(Reverse((_, entry))) = self.queue.pop() else {
                return Ok(None);
            };
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
            if !firing.redundant && !firing.derived.is_empty() {
                // A productive firing wrote a new fact; the next pass processes
                // only what it derived (semi-naive), accumulated
                // until consumed.
                self.needs_enqueue = true;
                self.delta
                    .get_or_insert_with(Vec::new)
                    .extend(firing.derived.iter().copied());
            }
            return Ok(Some(firing));
        }
    }

    /// The next firing of a rule that states `:ein inconsistent` over the
    /// facts written so far, or `None` where none is left: what a saturation
    /// stopped at its first contradiction fires before it returns (the
    /// module's § A rule that ends a state).
    ///
    /// It matches such rules' plans alone, over the facts the pending pass
    /// has still to read, and leaves them for that pass, so a saturation that
    /// goes on ([`Self::run_to_quiescence`]) fires what it would have fired:
    /// the candidates enqueued here are then already seen.
    fn end_step(&mut self, s: &mut Session<'_>) -> Result<Option<Firing>, SaturateError> {
        if self.needs_enqueue {
            self.match_ends(s)?;
        }
        self.fire_next(s, true)
    }

    /// Enqueue what the pending pass would for the plans of rules that state
    /// `:ein inconsistent`, each candidate once.
    fn match_ends(&mut self, s: &mut Session<'_>) -> Result<(), SaturateError> {
        if self.open_ends {
            // Such a rule with parameters: a derived activator gives it a
            // plan, as the pending pass would find.
            self.engine.compile_all(s.ast, s.terms, s.kb, s.events)?;
        }
        let n = self.engine.len();
        if self.ends_n != n {
            self.refresh_at_once(s);
            (self.end_plans, self.end_index) = ends_of(&self.engine, &self.at_once);
            self.ends_n = n;
            self.ends_matched = 0;
        }
        if self.end_plans.is_empty() {
            return Ok(());
        }
        self.matched_plans.resize(n, false);
        let Some(delta) = self.delta.take() else {
            // A full pass is pending: every such plan, whole, once.
            if self.ends_seeded.is_none() {
                for k in 0..self.end_plans.len() {
                    let at = self.end_plans[k];
                    self.full_match(s, at);
                }
                self.ends_seeded = Some(0);
            }
            return Ok(());
        };
        // A plan the pending pass would match whole, as it is new; those
        // below `ends_matched` have been.
        for k in self.ends_matched..self.end_plans.len() {
            let at = self.end_plans[k];
            if !self.matched_plans[at] {
                self.matched_plans[at] = true;
                self.full_match(s, at);
            }
        }
        self.ends_matched = self.end_plans.len();
        let index = Arc::clone(&self.end_index);
        let mut plans = std::mem::take(&mut self.offered);
        for &fact in &delta[self.ends_seeded.unwrap_or(0)..] {
            index.plans_for(s.terms, fact, &mut plans);
            for &at in &plans {
                self.offer(s, at, fact);
            }
        }
        self.offered = plans;
        self.ends_seeded = Some(delta.len());
        self.delta = Some(delta);
        Ok(())
    }

    /// Apply one popped candidate — productive or redundant.
    ///
    /// The guards are **not** re-evaluated here (a guard is judged once, on
    /// the boundary), but they are recorded as the firing's
    /// negative premises, so a provenance walk can see what the conclusion
    /// depended on *not* holding.
    fn apply(
        &mut self,
        s: &mut Session<'_>,
        entry: u32,
        key: BindingKey,
    ) -> Result<Option<Firing>, SaturateError> {
        let e = self.entries.get(entry);
        let (plan_at, disjunct) = (e.plan, e.disjunct);
        let plan = self.engine.plan_arc(plan_at);
        if plan.ends {
            // A rule that states `:ein inconsistent`: the application ends
            // the state and writes no fact. Recorded with what it rests on,
            // as a derivation is, so a core can be taken over it.
            self.engine.fired.insert(key);
            let guards = plan.disjuncts[disjunct].guards;
            let absent = self.negative_premises(s, &plan, entry, guards);
            let universal = self.universal_support(s, &plan, entry, guards);
            if self.reading_universals {
                self.last_universal = universal.clone();
            }
            let e = self.entries.get(entry);
            let env = Env {
                regs: &e.regs,
                trail: &e.trail,
                premises: &e.premises,
            };
            return Ok(Some(crate::firing::end(
                s.kb, s.terms, &plan, env, absent, universal,
            )));
        }
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
        // Tentative-build every conclusion (a `:assert (and …)` has
        // several). Cheap — a pure walk over each template.
        let mut tentative = Vec::with_capacity(plan.asserts.len());
        for i in 0..plan.asserts.len() {
            // A true `eq` / `neq` conclusion adds nothing, and a false one is
            // `(false)` (Q-M1e2.26).
            if let Some(f) = build_conclusion(s.terms, s.ast, &plan, env, plan.asserts[i])? {
                tentative.push(f);
            }
        }
        if tentative.is_empty() {
            // Every conclusion is an `eq` / `neq` that holds: the application
            // concludes nothing, as one with no `:assert` does, and is no
            // firing — a firing derives at least one fact, which the trace
            // and the event stream read as its primary conclusion.
            self.engine.fired.insert(key);
            return Ok(None);
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
            if self.reading_universals {
                self.last_universal = self.universal_support(s, &plan, entry, guards);
            }
            let e = self.entries.get(entry);
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
                ends: false,
            }));
        }
        let absent = self.negative_premises(s, &plan, entry, guards);
        let universal = self.universal_support(s, &plan, entry, guards);
        if self.reading_universals {
            self.last_universal = universal.clone();
        }
        let e = self.entries.get(entry);
        let env = Env {
            regs: &e.regs,
            trail: &e.trail,
            premises: &e.premises,
        };
        Ok(fire(s.kb, s.terms, s.ast, &plan, env, absent, universal)?)
    }

    /// What a firing admitted here rests on through its **universals**: for
    /// each guard whose sub-plan reads a nested `(absent …)`, a `forall`'s
    /// shape, the facts the nested body matched for each match of the rest
    /// ([`crate::match_::Matcher::universal_support`]), in the guards' order
    /// and each once. A guard with no nested query rests on an absence, which
    /// [`Saturator::negative_premises`] records, and on no fact.
    ///
    /// A core that rests on a universal takes in what it read (Q-M1e2.15).
    fn universal_support(
        &mut self,
        s: &Session<'_>,
        plan: &Plan,
        entry: u32,
        guards: Span,
    ) -> Box<[FactId]> {
        if guards.is_empty() {
            return Box::new([]);
        }
        let mut out: Vec<FactId> = Vec::new();
        for g in plan.guards(guards) {
            let universal = plan
                .steps(g.sub)
                .iter()
                .any(|step| matches!(step, crate::plan::Step::Absent { .. }));
            if !universal {
                continue;
            }
            let regs = &self.entries.get(entry).regs;
            for f in self
                .matcher
                .universal_support(s.kb, s.terms, s.ast, plan, g, regs)
            {
                if !out.contains(&f) {
                    out.push(f);
                }
            }
        }
        out.into_boxed_slice()
    }

    /// Record a redundant firing as an **alternative** justification.
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
        if self.entries.get(entry).premises.is_empty() {
            return;
        }
        // Built only once at least one conclusion can still take an
        // alternative, so a hot fact whose list is full of shorter derivations
        // costs one check and nothing else: O(1), and at a full list the
        // premises' closures against the fact's.
        let premises = &self.entries.get(entry).premises;
        let targets: Vec<FactId> = existing
            .iter()
            .copied()
            .filter(|&f| s.kb.contains(f) && s.kb.accepts_justification(s.terms, f, premises))
            .collect();
        if targets.is_empty() {
            return;
        }
        // An alternative carries its OWN negative premises: provenance is per
        // derivation, so a re-derivation admitted through the boundary depends
        // on what *its* guards found missing.
        let absent = self.negative_premises(s, plan, entry, guards);
        let universal = self.universal_support(s, plan, entry, guards);
        let premises = self.entries.get(entry).premises.clone();
        let mut prov = Prov::from_rule(plan.rule, premises, None);
        prov.absent = absent;
        prov.activator = plan.activator;
        prov.universal = universal;
        let id = s.terms.provs.push(prov);
        for fact in targets {
            let recorded = s.kb.record_justification(s.terms, fact, id);
            if recorded && s.events.on() {
                let (fact_s, rule_s) = (
                    events::sexpr(s.terms, fact),
                    s.terms.sym(plan.rule).to_string(),
                );
                let prems = events::sexpr_facts(s.terms, &self.entries.get(entry).premises);
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
        // reflective rule-implication case.
        self.engine.compile_all(s.ast, s.terms, s.kb, s.events)?;
        self.matched_plans.resize(self.engine.len(), false);
        self.refresh_at_once(s);
        if self.fail_first.is_some() {
            self.refresh_first(s);
        }
        ein_core::counters::bump(|c| c.enqueue_pass += 1);
        let Some(delta) = delta else {
            for at in 0..self.engine.len() {
                self.matched_plans[at] = true;
                self.full_match(s, at);
            }
            self.matched_upto = self.engine.len();
            return Ok(());
        };
        self.rebuild_index();
        let mut full_done: FxHashSet<usize> = FxHashSet::default();
        // Never-matched plans (a reflective rule's freshly-compiled one) get
        // one FULL match each: they may match existing facts, not just the
        // delta. Every plan below `matched_upto` has been.
        for at in self.matched_upto..self.engine.len() {
            if !self.matched_plans[at] {
                self.matched_plans[at] = true;
                full_done.insert(at);
                self.full_match(s, at);
            }
        }
        self.matched_upto = self.engine.len();
        // Positive-premise plans: seed each delta fact. The matcher iterates
        // the one new fact at its premise instead of re-scanning the
        // relation's whole extent — 91 % of matcher output was re-discovery a
        // full re-match would recompute.
        let index = Arc::clone(&self.pos_index);
        let mut plans = std::mem::take(&mut self.offered);
        for fact in delta {
            index.plans_for(s.terms, fact, &mut plans);
            for &at in &plans {
                if full_done.is_empty() || !full_done.contains(&at) {
                    self.offer(s, at, fact);
                }
            }
        }
        self.offered = plans;
        Ok(())
    }

    /// The plans a new fact can seed ([`PlanIndex`]), every plan of the
    /// engine's. Rebuilt when the plan list grows.
    fn rebuild_index(&mut self) {
        if self.index_n == self.engine.len() {
            return;
        }
        self.pos_index = Arc::new(PlanIndex::of(&self.engine, 0..self.engine.len()));
        self.index_n = self.engine.len();
    }

    /// Hand the new `fact` to plan `at`: a probe where the plan's match is
    /// a pair and nothing else ([`Self::pair_at`]), else a seeded match.
    fn offer(&mut self, s: &mut Session<'_>, at: usize, fact: FactId) {
        match self.pair_at.get(at).copied().flatten() {
            Some(shape) => self.pair_match(s, at, shape, fact),
            None => self.seed_match(s, at, fact),
        }
    }

    /// **A pair matched by a probe** (the module's § A pair matched by a
    /// probe): the matches of plan `at`, whose match is the pair `shape`,
    /// that the new `fact` takes part in, found by one probe each
    /// ([`PairShape::each_match`]) and queued as a seeded match would queue
    /// them, the same registers, bindings and premises, in the same order.
    /// Where the probe finds nothing, nothing is built.
    ///
    /// The gate's build holds what is queued to a seeded match of the same
    /// fact, a check a fact.
    fn pair_match(&mut self, s: &mut Session<'_>, at: usize, shape: PairShape, fact: FactId) {
        let mut hits = [[FactId(0); 2]; 2];
        let mut n = 0;
        shape.each_match(s.kb, s.terms, fact, |premises| {
            hits[n] = premises;
            n += 1;
        });
        #[cfg(debug_assertions)]
        let mut probed: Vec<SeededMatch> = Vec::new();
        if n > 0 {
            let plan = self.engine.plan_arc(at);
            let mut m = std::mem::take(&mut self.matcher);
            let (terms, events) = (&*s.terms, &mut *s.events);
            for &premises in &hits[..n] {
                m.run_pair(terms, &plan, shape.held, premises, &mut |mt| {
                    #[cfg(debug_assertions)]
                    probed.push(SeededMatch::of(mt));
                    self.enqueue_binding(terms, events, at, &plan, mt);
                    ControlFlow::Continue(())
                });
            }
            self.matcher = m;
        }
        #[cfg(debug_assertions)]
        {
            let plan = self.engine.plan_arc(at);
            let mut seeded: Vec<SeededMatch> = Vec::new();
            let mut m = std::mem::take(&mut self.matcher);
            m.run_seeded(s.kb, s.terms, s.ast, &plan, fact, &mut |mt| {
                seeded.push(SeededMatch::of(mt));
                ControlFlow::Continue(())
            });
            self.matcher = m;
            debug_assert_eq!(
                probed, seeded,
                "the probe of a pair disagrees with a seeded match of the same fact"
            );
        }
    }

    fn full_match(&mut self, s: &mut Session<'_>, at: usize) {
        ein_core::counters::bump(|c| {
            c.enqueue_task += 1;
            c.enqueue_task_full += 1;
        });
        let plan = self.engine.plan_arc(at);
        // The matcher is moved out for the duration: `enqueue_binding` runs
        // *inside* its callback and needs the saturator, which the matcher is
        // part of. Moving it out is what keeps the enqueue on the match's own
        // stack — a duplicate then costs nothing beyond the key hash.
        let mut m = std::mem::take(&mut self.matcher);
        let (kb, terms, ast) = (&*s.kb, &*s.terms, s.ast);
        for d in 0..plan.disjuncts.len() {
            let events = &mut *s.events;
            m.run_one(kb, terms, ast, &plan, d, &mut |mt| {
                self.enqueue_binding(terms, events, at, &plan, mt);
                ControlFlow::Continue(())
            });
        }
        self.matcher = m;
    }

    fn seed_match(&mut self, s: &mut Session<'_>, at: usize, fact: FactId) {
        ein_core::counters::bump(|c| c.enqueue_task += 1);
        let plan = self.engine.plan_arc(at);
        let mut m = std::mem::take(&mut self.matcher);
        let (kb, terms, ast, events) = (&*s.kb, &*s.terms, s.ast, &mut *s.events);
        m.run_seeded(kb, terms, ast, &plan, fact, &mut |mt| {
            self.enqueue_binding(terms, events, at, &plan, mt);
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
                    .map(|&x| terms.display(x))
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
                l.num("tiebreaker", tb);
                l.bool("parked", parked);
            });
        }
        let entry = self.entries.len() as u32;
        let rank = self.tiebreaker | self.band(at);
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
            self.parked.insert((rank, entry));
        } else {
            self.queue.push(Reverse((rank, entry)));
        }
    }

    /// The bits a candidate of plan `at` carries above its tiebreaker: none
    /// for a rule that states `:ein inconsistent`, which fires before every
    /// other; [`ORDINARY`] for any other, with [`LATE`] where its rule is not
    /// among the agenda's first, never under the written order alone
    /// ([`fifo`]).
    #[inline]
    fn band(&self, at: usize) -> u64 {
        if self.engine.plan(at).ends || self.at_once.get(at).copied().unwrap_or(false) {
            return 0;
        }
        match self.fail_first {
            None => ORDINARY,
            Some(_) if self.first.get(at).copied().unwrap_or(false) => ORDINARY,
            Some(_) => ORDINARY | LATE,
        }
    }

    /// Read again which plans' rules fire at once ([`Self::at_once`]), and
    /// which are matched by a probe ([`Self::pair_at`]), once the engine
    /// holds a plan this saturation has not read.
    fn refresh_at_once(&mut self, s: &Session<'_>) {
        let n = self.engine.len();
        if self.at_once.len() == n {
            return;
        }
        let pairs = crate::contradiction::Pairs::of((0..n).map(|at| self.engine.plan(at)), s.terms);
        self.at_once = (0..n)
            .map(|at| self.engine.plan(at).ends || pairs.refutes_at(at))
            .collect();
        self.pair_at = (0..n).map(|at| pairs.shape_at(at)).collect();
    }

    /// Under an agenda, read again which plans' rules fire first, once
    /// the engine holds a plan this saturation has not read: a new plan's
    /// negation can make another plan's conclusion one side of a pair.
    fn refresh_first(&mut self, s: &Session<'_>) {
        let Some(which) = self.fail_first else {
            return;
        };
        let n = self.engine.len();
        if self.first.len() == n {
            return;
        }
        let plans: Vec<&Plan> = (0..n).map(|at| self.engine.plan(at)).collect();
        let tables = FirstTables::of(&plans, s.terms);
        // The relations whose holding side the KB holds.
        // determinism-ok: a set asked for membership alone.
        let mut held: FxHashSet<Symbol> = FxHashSet::default();
        if which == FailFirst::Pairs {
            for wrapper in tables.pairs.wrappers() {
                tables.hold(s.kb.facts_of(wrapper), s.terms, &mut held);
            }
        }
        self.first = tables.first(which, &held);
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

    fn entry_key(&self, entry: u32) -> &BindingKey {
        &self.entries.get(entry).key
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
        self.refresh_collision_risk();
        self.refresh_epochs(s);
        let mut admitted = 0;
        // The set itself, walked in place — no per-round copy of it.
        //
        // The round is ordered iteration rather than pop-and-re-push, over a
        // snapshot, because a round *removes* from the set as it goes. But the removals are known
        // in advance to be at most one admission (which ends the round) plus
        // whatever the walk retires, so they can be deferred to the end and
        // the walk can hold the set instead of copying it. That matters
        // because **a round almost never reaches the end**: 3 216 rounds of
        // `zebra -e` copy 947 758 candidates to visit 248 043 of them, and
        // stop early on the admission that ends each one.
        let parked = std::mem::take(&mut self.parked);
        let mut retired = std::mem::take(&mut self.retired_scratch);
        for &key in parked.iter() {
            let (tb, entry) = key;
            // Fired by another route — impossible unless two entries can share
            // a binding key, which is what `may_collide` decides. Asking costs
            // a `BindingKey` hash per parked candidate per round, 3.8 % of
            // `zebra -e`'s self time, for a branch no corpus program can take.
            if self.may_collide && self.engine.fired.contains(self.entry_key(entry)) {
                retired.push(key);
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
            let e = self.entries.get(entry);
            let judged_at = self.judged_at[entry as usize];
            if judged_at != 0 && self.gs_epoch[e.guard_set.0 as usize] <= judged_at {
                continue;
            }
            let plan = self.engine.plan_arc(e.plan);
            let guards = plan.disjuncts[e.disjunct].guards;
            let failing = self.first_failing(s, &plan, entry, guards);
            match failing {
                None => {
                    retired.push(key);
                    self.queue.push(Reverse(key));
                    self.naf_admitted += 1;
                    admitted = 1;
                    if s.events.on() {
                        let rule = s.terms.sym(plan.rule).to_string();
                        let round = self.naf_rounds as i64;
                        s.events.emit("admit", |l| {
                            l.num("tiebreaker", (tb & !BAND) as i64);
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
                    retired.push(key);
                    self.naf_retired += 1;
                    if s.events.on() {
                        emit_boundary(s, "retire", tb & !BAND, self.naf_rounds, &plan, guards, g);
                    }
                }
                Some(g) => {
                    self.judged_at[entry as usize] = self.epoch;
                    if s.events.on() {
                        emit_boundary(s, "park", tb & !BAND, self.naf_rounds, &plan, guards, g);
                    }
                }
            }
        }
        self.parked = parked;
        for key in retired.drain(..) {
            self.parked.remove(&key);
        }
        self.retired_scratch = retired;
        Ok(admitted)
    }

    /// Whether a **parked** candidate's binding key can ever reach
    /// `engine.fired`, recomputed when the plan list grows and never
    /// otherwise.
    ///
    /// Only another entry with the same key — same rule, same activator, same
    /// registers — can put it there, and it must carry a *different* guard
    /// set, since `seen` dedups the pair. That needs either one plan whose
    /// disjuncts ask different negative questions, or two plans sharing a
    /// `(rule, activator)` binding-key space. **The second cannot happen**:
    /// the activator id is the compile key's list, every argument
    /// (Q-M1e.16), so each plan has a space of its own. The check below still
    /// reads both, at the cost of a set insert per plan.
    ///
    /// **The first holds nowhere in the corpus**: the branch is taken 0 times
    /// across every `.ein` under `examples/` and `stdlib/`, and where a
    /// program does put it in reach the round walks with the check, exactly as
    /// before.
    ///
    /// Sticky once true, because a plan that exists cannot stop existing — and
    /// a candidate parked while it was false could not have collided with
    /// anything, since the entry it would collide with needs a plan that was
    /// not there.
    fn refresh_collision_risk(&mut self) {
        let n = self.engine.len();
        if n == self.plans_seen || self.may_collide {
            self.plans_seen = n;
            return;
        }
        self.plans_seen = n;
        let mut spaces: FxHashSet<(Symbol, crate::firing::ActivatorId)> = FxHashSet::default();
        let mut risk = false;
        for i in 0..n {
            let plan = self.engine.plan(i);
            let first = plan.guard_key(plan.disjuncts[0].guard_key);
            if plan.disjuncts[1..]
                .iter()
                .any(|d| plan.guard_key(d.guard_key) != first)
                || !spaces.insert((plan.rule, self.engine.activator(i)))
            {
                risk = true;
                break;
            }
        }
        self.may_collide = risk;
    }

    /// Advance the boundary's clock, and with it the guard sets whose world
    /// moved — the whole round's invalidation, in one pass over the watched
    /// relations.
    ///
    /// Asked per parked candidate — build the extent sizes of every relation
    /// its guards read, compare against the sizes at its last failed
    /// judgement, skip if equal — the answer is the right one: the KB grows
    /// monotonically within a run, so a relation cannot lose a fact and equal
    /// size means equal extent. But asked that way it is asked many times per
    /// judgement
    /// ([03-design/06-saturation.md § 3](../../../../docs/ein/ein.rs/03-design/06-saturation.md#3-the-queue-the-delta-and-the-parked-set)),
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
        let regs = self.entries.get(entry).regs.clone();
        let mut m = std::mem::take(&mut self.matcher);
        let mut out = None;
        for (i, g) in plan.guards(guards).iter().enumerate() {
            ein_core::counters::bump(|c| c.guard_query += 1);
            self.guard_evals += 1;
            self.guard_evals_monotone += g.monotone as u64;
            // The same two, summed over every fork of a solve, which a
            // per-saturation field cannot answer.
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
        let regs = &self.entries.get(entry).regs;
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
/// value for the same name, because the negative-provenance walk uses the
/// projected environment — which is the same restriction the query itself ran
/// under. A `Fact`-valued binding also grounds to `Free`: only a name is
/// kept.
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
        .map(|&x| s.terms.display(x))
        .collect();
    let bindings = events::binding_pairs(s.terms, &firing.bindings);
    let premises = events::sexpr_facts(s.terms, &firing.premises);
    let derived = events::sexpr_facts(s.terms, &firing.derived);
    let redundant = firing.redundant;
    let ends = firing.ends;
    s.events.emit("fire", |l| {
        l.str("rule", &rule);
        l.owned_strs("activator", activator);
        l.bindings("bindings", bindings);
        l.owned_strs("premises", premises);
        l.owned_strs("derived", derived);
        l.bool("redundant", redundant);
        // The word the rule gave the engine, where it states one: the
        // firing ended the state and derived nothing.
        if ends {
            l.str("ein", ein_core::EngineWord::Inconsistent.as_str());
        }
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
    use std::collections::BTreeSet;

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

    /// A new fact holding a fact is handed to the plans that read one of
    /// that fact's relation at that argument, and to those whose premise
    /// leaves it open, in the plans' order, each once; a fact holding none
    /// to the open ones alone.
    #[test]
    fn a_fact_holding_a_fact_seeds_the_plans_of_its_relation() {
        let src = "(relation p Thing :why \"p {?3}\")\n(relation q Thing :why \"q {?3}\")\n\
                   (relation r Thing :why \"r {?3}\")\n(relation w Thing :why \"w {?3}\")\n\
                   (rule of-p () :match (w (p ?a)) :assert (r ?a) :why \"a\")\n\
                   (rule loose () :match (and (q ?a) (w ?x)) :assert (r ?a) :why \"b\")\n\
                   (rule of-q () :match (and (w (q ?a)) (q ?a)) :assert (r ?a) :why \"c\")\n\
                   (rule both () :match (and (w (p ?a)) (w (q ?a))) :assert (r ?a) :why \"d\")\n\
                   (p a)\n(q a)\n(w (p a))\n(w (q a))\n(w b)\n";
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
        let sat = Saturator::new(&mut s).expect("compiles");
        let index = PlanIndex::of(&sat.engine, 0..sat.engine.len());
        let plans_of = |fact: FactId| -> Vec<String> {
            let mut out = Vec::new();
            index.plans_for(s.terms, fact, &mut out);
            out.iter()
                .map(|&at| s.terms.sym(sat.engine.plan(at).rule).to_string())
                .collect()
        };
        let (terms, kb) = (&*s.terms, &*s.kb);
        let w = terms.syms.get("w").unwrap();
        let held = |inner: &str| {
            kb.facts_of(w)
                .find(|&f| events::sexpr(terms, f).contains(inner))
                .unwrap()
        };
        assert_eq!(plans_of(held("(p a)")), ["of-p", "loose", "both"]);
        assert_eq!(plans_of(held("(q a)")), ["loose", "of-q", "both"]);
        assert_eq!(plans_of(held(" b")), ["loose"]);
        let q = terms.syms.get("q").unwrap();
        let q_a = kb.facts_of(q).next().unwrap();
        assert_eq!(plans_of(q_a), ["loose", "of-q"]);
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
        let src = "(relation p Thing :why \"p holds of {?3}\")\n(relation q Thing :why \"q holds of {?3}\")\n(relation thing Thing :why \"{?3} is a thing\")\n\
                   (rule derive-p ()\n  :match (and (thing ?x) (absent (q ?x)))\n  \
                   :assert (p ?x) :why \"{?x} is a thing and q does not hold of {?x}, so p holds of {?x}\")\n\
                   (rule derive-q ()\n  :match (and (thing ?x) (absent (p ?x)))\n  \
                   :assert (q ?x) :why \"{?x} is a thing and p does not hold of {?x}, so q holds of {?x}\")\n\
                   (thing one)\n";
        let (_, _, order, rounds) = saturate(src);
        assert_eq!(order.len(), 1, "exactly one of p, q — got {order:?}");
        assert!(order[0] == "(p one)" || order[0] == "(q one)");
        assert!(rounds >= 1, "the boundary ran");
    }

    /// A resumed saturation reaches the same fixpoint as a fresh one —
    /// [`Saturator::resume`], the mechanism
    /// [03-design/06-saturation.md § 4](../../../../docs/ein/ein.rs/03-design/06-saturation.md#4-a-node-resumes-roots-saturation)
    /// is about.
    ///
    /// Saturate, add a fact, then continue two ways: a **fresh** saturator
    /// over the grown KB (a FULL pass, which is what a fork does today) and a
    /// **resumed** one seeded with just the new fact. The two fact sets must
    /// agree exactly. What the test deliberately does *not* assert is the
    /// firing sequence: the resumed run skips the re-derivations, and that
    /// difference is the whole point of resuming.
    #[test]
    fn a_resumed_saturation_reaches_the_same_fixpoint() {
        let src = "(relation edge T T :why \"there is an edge from {?3} to {?4}\")\n(relation path T T :why \"there is a path from {?3} to {?4}\")\n                   (rule step ()\n  :match (edge ?a ?b)\n  :assert (path ?a ?b) :why \"there is an edge from {?a} to {?b}, so there is a path from {?a} to {?b}\")\n                   (rule trans ()\n  :match (and (path ?a ?b) (path ?b ?c))\n                     :assert (path ?a ?c) :why \"there are paths from {?a} to {?b} and from {?b} to {?c}, so there is one from {?a} to {?c}\")\n                   (rule tip ()\n  :match (and (path ?a ?b) (absent (edge ?b ?a)))\n                     :assert (path ?b ?b) :why \"there is a path from {?a} to {?b} and no edge from {?b} back to {?a}, so there is a path from {?b} to {?b}\")\n                   (edge A B)\n(edge B C)\n";
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
            sat.snapshot(s.kb, s.terms)
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
            "the resumed run narrated {n_resumed} firings and the fresh one \
             {n_fresh} — the point is that it narrates fewer"
        );
    }

    /// `fork-delta-order`: a resumed fork's delta is in
    /// the order its facts were written, never in `FactId` order. The two
    /// facts here are interned one way round and written the other.
    #[test]
    fn a_resumed_forks_delta_is_in_the_order_it_was_written() {
        let src = "(relation edge T T :why \"there is an edge from {?3} to {?4}\")\n(relation path T T :why \"there is a path from {?3} to {?4}\")\n\
                   (rule step () :match (edge ?a ?b) :assert (path ?a ?b) :why \"there is an edge from {?a} to {?b}, so there is a path from {?a} to {?b}\")\n\
                   (edge A B)\n";
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let mut kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let mut ev = Events::off();
        let snapshot = {
            let mut s = Session {
                kb: &mut kb,
                terms: &mut terms,
                ast: &ast,
                events: &mut ev,
                memo: SharedMemo::default(),
            };
            let mut sat = Saturator::new(&mut s).expect("compiles");
            sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
            sat.snapshot(s.kb, s.terms)
        };
        let edge = terms.syms.get("edge").expect("interned");
        let mut args = |a: &str, b: &str| {
            [
                Value::sym(terms.intern_text(a).expect("room")),
                Value::sym(terms.intern_text(b).expect("room")),
            ]
        };
        let (cd, de) = (args("C", "D"), args("D", "E"));
        let early = terms.intern_fact(edge, &cd).expect("room");
        let late = terms.intern_fact(edge, &de).expect("room");
        assert!(early < late, "interned in this order");
        let mut fork = kb.fork();
        for a in [de, cd] {
            fork.add_and_index_fact(&mut terms, edge, &a, None)
                .expect("room");
        }
        assert_eq!(
            snapshot.new_facts_of(&fork),
            vec![late, early],
            "the delta is the order the facts were written, not their ids'"
        );
    }

    /// `naf_dropped` is **structurally** 0: a guard is judged once, on the
    /// boundary, at the moment the candidate is admitted, so there is no
    /// window between the verdict and the firing for it to go stale in.
    #[test]
    fn nothing_is_ever_dropped_for_a_stale_guard() {
        let src = "(relation a Thing :why \"a holds of {?3}\")\n(relation block Thing :why \"there is a block for {?3}\")\n(relation seen Thing :why \"{?3} is seen\")\n\
                   (rule r ()\n  :match (and (a ?x) (absent (block ?x)))\n  \
                   :assert (seen ?x) :why \"a holds of {?x} and there is no block for {?x}, so {?x} is seen\")\n\
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
        let src = "(relation edge A B :why \"there is an edge from {?3} to {?4}\")\n(relation path A B :why \"there is a path from {?3} to {?4}\")\n\
                   (rule walk ()\n  :match (and (edge ?a ?b) (edge ?b ?c))\n  \
                   :assert (edge ?a ?c) :why \"there are edges from {?a} to {?b} and from {?b} to {?c}, so there is one from {?a} to {?c}\")\n\
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

    /// The agenda: a rule whose conclusion a contradiction can be made of
    /// fires before one written earlier whose conclusion is not, and the
    /// fixpoint is the written order's. The contradiction of a fact beside
    /// its negation is a rule's, here `clash`, activated for `r` and `s`.
    /// [`AGENDA`] is `Pairs`, a positive fact of a relation some rule denies
    /// as well among the first; `Negations` puts a stored negation such a
    /// rule reads first; [`fifo`] is the written order alone. Each switch
    /// holds only while it runs.
    #[test]
    fn a_fail_first_agenda_moves_the_order_and_not_the_fixpoint() {
        let src = "(relation p T :why \"p holds of {?3}\")\n(relation q T :why \"q holds of {?3}\")\n\
                   (relation r T :why \"r holds of {?3}\")\n(relation s T :why \"s holds of {?3}\")\n\
                   (rule clash (?R)\n  :match (and (?R ?x) (not (?R ?x)))\n  :assert (false) :why \"a fact beside its negation\")\n\
                   (clash r) (clash s)\n\
                   (rule to-q ()\n  :match (p ?x)\n  :assert (q ?x) :why \"p holds of {?x}, so q does\")\n\
                   (rule to-r ()\n  :match (p ?x)\n  :assert (r ?x) :why \"p holds of {?x}, so r does\")\n\
                   (rule not-s ()\n  :match (p ?x)\n  :assert (not (s ?x)) :why \"p holds of {?x}, so s does not\")\n\
                   (rule not-r ()\n  :match (q ?x)\n  :assert (not (r b)) :why \"q holds of {?x}, so r does not of b\")\n\
                   (p a)\n";
        let sorted = |kb: &Kb, terms: &Terms| {
            let mut v: Vec<String> = kb.facts().map(|f| events::sexpr(terms, f)).collect();
            v.sort();
            v
        };
        let (t, kb, written, _) = fifo(|| saturate(src));
        assert_eq!(
            written,
            ["(q a)", "(r a)", "(not (s a))", "(not (r b))"],
            "FIFO: the written order"
        );
        let (tn, kbn, negations, _) = fail_first(FailFirst::Negations, || saturate(src));
        assert_eq!(
            negations,
            ["(not (s a))", "(q a)", "(not (r b))", "(r a)"],
            "`not-r`, matched once `(q a)` lands, goes before `to-r`, queued earlier"
        );
        let (tp, kbp, pairs, _) = fail_first(FailFirst::Pairs, || saturate(src));
        assert_eq!(
            pairs,
            ["(r a)", "(not (s a))", "(q a)", "(not (r b))"],
            "`r` is denied by a rule, so `(r a)` is one side of a pair"
        );
        assert_eq!(sorted(&kb, &t), sorted(&kbn, &tn));
        assert_eq!(sorted(&kb, &t), sorted(&kbp, &tp));
        assert_eq!(is_fail_first(), Some(AGENDA), "restored when it returns");
        assert_eq!(saturate(src).2, pairs, "and the engine's agenda is `Pairs`");
        assert_eq!(
            fifo(|| saturate(src)).2,
            written,
            "and the written order under the switch again"
        );
    }

    /// The other switch: under [`stopping_at_first_contradiction`]
    /// `saturate` returns after the firing that ends the state, here the
    /// rule every program gets over the `(false)` `clash` concluded from a
    /// fact beside its negation, and holds only while it runs.
    #[test]
    fn a_saturation_stopped_at_its_first_contradiction() {
        let src = "(relation p T :why \"p holds of {?3}\")\n(relation q T :why \"q holds of {?3}\")\n\
                   (relation r T :why \"r holds of {?3}\")\n\
                   (rule clash (?R)\n  :match (and (?R ?x) (not (?R ?x)))\n  :assert (false) :why \"a fact beside its negation\")\n\
                   (clash q)\n\
                   (rule to-q ()\n  :match (p ?x)\n  :assert (q ?x) :why \"p holds of {?x}, so q does\")\n\
                   (rule not-q ()\n  :match (p ?x)\n  :assert (not (q ?x)) :why \"p holds of {?x}, so q does not\")\n\
                   (rule to-r ()\n  :match (p ?x)\n  :assert (r ?x) :why \"p holds of {?x}, so r does\")\n\
                   (p a)\n";
        let (_, _, whole, _) = saturate(src);
        assert_eq!(whole, ["(q a)", "(not (q a))", "(false)", "(r a)"]);
        let (_, _, stopped, _) = stopping_at_first_contradiction(|| saturate(src));
        assert_eq!(
            stopped,
            ["(q a)", "(not (q a))", "(false)"],
            "stopped where the state ends"
        );
        assert!(!is_stopping_at_first_contradiction());
        let (_, _, first, _) =
            stopping_at_first_contradiction(|| fail_first(FailFirst::Negations, || saturate(src)));
        assert_eq!(
            first,
            ["(not (q a))", "(q a)", "(false)"],
            "the pair, its negation first"
        );
    }

    /// The rules of one firing each, in the order they fired, with whether
    /// the firing ended the state, and the saturated KB.
    fn fired(src: &str) -> (Terms, Kb, Vec<(String, bool)>) {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let mut kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let mut ev = Events::off();
        let mut order: Vec<(Symbol, bool)> = Vec::new();
        {
            let mut s = Session {
                kb: &mut kb,
                terms: &mut terms,
                ast: &ast,
                events: &mut ev,
                memo: SharedMemo::default(),
            };
            let mut sat = Saturator::new(&mut s).expect("compiles");
            sat.saturate(&mut s, None, &mut |f| order.push((f.rule, f.ends)))
                .expect("saturates");
        }
        let order = order
            .into_iter()
            .map(|(r, ends)| (terms.sym(r).to_string(), ends))
            .collect();
        (terms, kb, order)
    }

    const ENDS: &str = "(relation p T :why \"p holds of {?3}\")\n(relation q T :why \"q holds of {?3}\")\n\
         (relation r T :why \"r holds of {?3}\")\n(relation s T :why \"s holds of {?3}\")\n\
         (rule to-r ()\n  :match (p ?x)\n  :assert (r ?x) :why \"p holds of {?x}, so r does\")\n\
         (rule to-q ()\n  :match (p ?x)\n  :assert (q ?x) :why \"p holds of {?x}, so q does\")\n\
         (rule to-s ()\n  :match (r ?x)\n  :assert (s ?x) :why \"r holds of {?x}, so s does\")\n\
         (rule no-q ()\n  :match (q ?x)\n  :ein inconsistent :why \"q holds of {?x}, which cannot be\")\n\
         (p a)\n";

    /// A rule that states `:ein inconsistent` ends the state where it fires
    /// and writes no fact; it fires before every other candidate, and the
    /// rule that concludes what it matches before the rest (the module's § A
    /// rule that ends a state).
    #[test]
    fn a_rule_that_states_the_key_ends_the_state_and_writes_no_fact() {
        let (terms, kb, order) = fired(ENDS);
        let names = |o: &[(String, bool)]| o.iter().map(|(r, _)| r.clone()).collect::<Vec<_>>();
        // `to-q` leads to `no-q`, so it fires before `to-r`, written first;
        // `no-q` fires as soon as `(q a)` is there.
        assert_eq!(names(&order), ["to-q", "no-q", "to-r", "to-s"]);
        assert_eq!(
            order.iter().filter(|(_, ends)| *ends).count(),
            1,
            "{order:?}"
        );
        assert!(kb.is_inconsistent());
        assert_eq!(kb.ended().len(), 1);
        let record = terms.provs.get(kb.ended()[0]);
        assert_eq!(record.rule.map(|r| terms.sym(r)), Some("no-q"));
        assert_eq!(
            record
                .premises
                .iter()
                .map(|&f| events::sexpr(&terms, f))
                .collect::<Vec<_>>(),
            ["(q a)"]
        );
        // Root runs to its fixpoint, and the firing added no fact to it.
        let facts: BTreeSet<String> = kb
            .facts()
            .filter(|&f| terms.sym(terms.facts.rel(f)) != "relation")
            .map(|f| events::sexpr(&terms, f))
            .collect();
        assert_eq!(
            facts,
            ["(p a)", "(q a)", "(r a)", "(s a)"]
                .map(str::to_string)
                .into_iter()
                .collect::<BTreeSet<_>>()
        );
        assert!(crate::contradiction::has_contradiction(&kb, &terms));
        let found = crate::contradiction::detect(&kb, &terms);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].kind(), crate::contradiction::Kind::Ended);

        // Under the written order alone the rule still fires before every
        // other candidate the queue holds.
        let (_, _, written) = fifo(|| fired(ENDS));
        assert_eq!(names(&written), ["to-r", "to-q", "no-q", "to-s"]);
    }

    /// A saturation stopped at its first contradiction stops at the firing
    /// that ends the state: after the firing that concluded what the rule
    /// matched, and that rule's, with nothing else fired.
    #[test]
    fn a_stopped_saturation_stops_at_the_firing_that_ends_the_state() {
        let (_, kb, order) = stopping_at_first_contradiction(|| fired(ENDS));
        assert_eq!(
            order,
            [("to-q".to_string(), false), ("no-q".to_string(), true)]
        );
        assert!(kb.is_inconsistent());
        let (_, kb, order) = stopping_at_first_contradiction(|| fifo(|| fired(ENDS)));
        assert_eq!(
            order.iter().map(|(r, _)| r.as_str()).collect::<Vec<_>>(),
            ["to-r", "to-q", "no-q"],
            "in the written order, at the same firing"
        );
        assert!(kb.is_inconsistent());
    }

    /// A stopped state holds every firing that ends it over the facts it
    /// has: a saturation fires the rule that states the key over what a
    /// firing wrote before it returns. A fact beside its stored negation is
    /// no contradiction by itself: with no rule over the two, the state's one
    /// contradiction is the firing's.
    #[test]
    fn a_saturation_stopped_fires_what_ends_the_state() {
        let src = "(relation p T :why \"p holds of {?3}\")\n(relation q T :why \"q holds of {?3}\")\n\
             (relation r T :why \"r holds of {?3}\")\n\
             (rule both ()\n  :match (p ?x)\n  :assert (and (q ?x) (not (p ?x))) :why \"p holds of {?x}, so q does and p does not\")\n\
             (rule to-r ()\n  :match (p ?x)\n  :assert (r ?x) :why \"p holds of {?x}, so r does\")\n\
             (rule no-q ()\n  :match (q ?x)\n  :ein inconsistent :why \"q holds of {?x}, which cannot be\")\n\
             (p a)\n";
        let (terms, kb, order) = stopping_at_first_contradiction(|| fired(src));
        assert_eq!(
            order,
            [("both".to_string(), false), ("no-q".to_string(), true)]
        );
        let kinds: Vec<&str> = crate::contradiction::detect(&kb, &terms)
            .iter()
            .map(|c| c.kind().as_str())
            .collect();
        assert_eq!(kinds, ["ended"]);
    }

    /// The same application is one firing: a KB that already holds its
    /// record keeps one, and a second saturation's firing is redundant.
    #[test]
    fn the_record_of_a_firing_that_ended_a_state_is_kept_once() {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, ENDS, Some("<test>")).expect("parses");
        let mut kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let mut ev = Events::off();
        let mut redundant: Vec<bool> = Vec::new();
        for _ in 0..2 {
            let mut s = Session {
                kb: &mut kb,
                terms: &mut terms,
                ast: &ast,
                events: &mut ev,
                memo: SharedMemo::default(),
            };
            let mut sat = Saturator::new(&mut s).expect("compiles");
            sat.saturate(&mut s, None, &mut |f| {
                if f.ends {
                    redundant.push(f.redundant);
                }
            })
            .expect("saturates");
        }
        assert_eq!(redundant, [false, true]);
        assert_eq!(kb.ended().len(), 1);
    }
}
