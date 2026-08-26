//! The three-phase solve loop — the one engine entry.
//!
//! **Phase 1 — root.** Saturate; a contradiction here is `k = 0` with the
//! source-frontier core. Compute `alive`, run the forced-positive cascade, and
//! if `alive` is empty and root is consistent then root *is* the unique model.
//!
//! **Phase 2 — layers.** For each layer: generate candidates, order them,
//! enter each through [`crate::commitment::try_commitment_set`]. Dead → learn a
//! no-good and, for a singleton, write `(not h)` back. Alive ∧ complete →
//! record a solution node, deduped by [`crate::canon::state_key`], and do
//! **not** expand it. Alive ∧ incomplete → expand to the next layer. Between
//! layers: recompute `alive`, run the cascade, drop commitments that left it.
//!
//! **Phase 3 — verdict.** Read it from `k`.
//!
//! ### Root stays stable
//!
//! No fork fact is ever merged back (P1.21 R2). The retired "unconditional
//! fact" extraction was unsound under NAF: a fact derived through `absent X`
//! leaves no provenance edge to the commitment that suppressed `X`, so a
//! positive-chain walk misread it as root-true. The only root writes during
//! Phase 2 are the singleton `(not h)` writeback and the forced-positive
//! promotions — both sound, both flagged by config, and both the thing
//! [design/08](../../../../docs/history/m1a_rust/design/08_parallelism.md) §2 has to
//! validate against.

use std::sync::Arc;
use std::time::Instant;

use ein_core::{FactId, Kb, Prov, SolverConfig, Symbol, Terms, Value};
use ein_ir::Ast;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::apriori::{CanonicalSetId, layer_1, order_candidates};
use crate::canon::state_key;
use crate::commitment::{CommitmentSetResult, Kind, try_commitment_set};
use crate::compile::{CompileError, SharedMemo};
use crate::events::Events;
use crate::firing::Firing;
use crate::obligations::Owes;
use crate::saturator::{SaturateError, Saturator, Session, Snapshot};
use crate::verdict::{Answer, Solution, Verdict, union_dead_cores};

// ── Counters ───────────────────────────────────────────────────────

/// Per-candidate counters shared by the run-level stats and the proof's.
///
/// ein.py factors these into a `_BaseStats` base class precisely so a counter
/// added to one cannot go missing from the other; here the same job is done by
/// [`LatticeStats`] holding a `base` rather than repeating the fields.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct BaseStats {
    pub enterings_total: u64,
    pub enterings_alive: u64,
    pub enterings_dead_pre: u64,
    pub enterings_dead_post: u64,
    pub facts_merged: u64,
    pub forced_positives: u64,
    pub saturate_count: u64,
    pub layers_explored: u64,
    pub nogoods_emitted: u64,
    pub nogoods_subsumed: u64,
}

/// Cumulative counters for one [`solve`] run.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct MonotonicStats {
    pub base: BaseStats,
    /// `k` — the deduped solution-node count.
    pub solution_nodes: u64,
    /// False when the lattice was **not** fully explored, so `k` is a lower
    /// bound: a `k = 1` from a `stop_after` run is "a model", not proven
    /// unique, and a `k = 0` from a truncated one is "no model within the
    /// cap", not proven unsat.
    pub exhausted: bool,
}

impl MonotonicStats {
    fn new() -> MonotonicStats {
        MonotonicStats {
            exhausted: true,
            ..MonotonicStats::default()
        }
    }
}

/// One layer's **clause-yield row** — the census
/// [S1d.10.1](../../../../plans/m1d_satisfiability/p1d.10_exhaustive_search/s1d.10.1_why_it_does_not_finish.md)
/// exists to take, and the `layer` event's whole payload.
///
/// The mechanism the phase opens on is *a layer that kills nothing learns
/// nothing*: pruning in this engine comes from deaths, and a death's product
/// is a learned clause (and, at width 1, a `(not h)` writeback). What no
/// counter said before this struct is the other end of that sentence — **what
/// the clauses the search already holds did to the next layer's generation**.
/// [`Self::joined`] is what the prefix-join proposed, [`Self::dropped_nogood`]
/// is what the clause store removed from it, and their ratio is the phase's
/// core measurement. On `zebra2-minus-15` layer 1 kills nothing, so layer 2 is
/// the full `C(96, 2)` with `dropped_nogood = 0`; on `zebra2` layer 1 kills 67
/// of 101 and the same column is what makes the search tractable.
///
/// **Not part of [`MonotonicStats`]**, and for [`JobStats`]'s reason turned
/// around: these are per *layer*, not per run, and a run's verdict surface
/// must not grow a field whose value depends on how the traversal was sliced.
/// The transport is the event stream — one `layer` line per layer, which a
/// census script reads across the whole corpus.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct LayerCensus {
    /// `|alive|` when the layer opened — hypothesis facts still unrefuted.
    pub alive: u64,
    /// `|A_prev|`, the frontier this layer joins over. **Not the previous
    /// row's [`Self::next`]**: between the two sits the inter-layer retain,
    /// which drops every commitment an element left `alive` under, so
    /// `next − frontier` is what recomputing `alive` at the barrier was worth.
    pub frontier: u64,
    /// Candidates [`crate::apriori::apriori_prefix_join`] proposed. At layer 1
    /// there is no join and this is [`Self::alive`], the singletons.
    pub joined: u64,
    /// …rejected because an element had left `alive`
    /// ([`crate::apriori::Filter::Dead`]).
    pub dropped_dead: u64,
    /// …rejected because a learned clause covers the set
    /// ([`crate::apriori::Filter::Nogood`]). **The column nothing reported
    /// before.**
    pub dropped_nogood: u64,
    /// Survivors — what the layer's loop was handed.
    pub candidates: u64,
    /// …and how many of them were actually entered, which is fewer exactly
    /// when a budget or `stop_after` cut the layer.
    pub entered: u64,
    pub alive_enterings: u64,
    pub dead_pre: u64,
    pub dead_post: u64,
    /// Distinct solution nodes recorded *in this layer* — the "found all
    /// models" column S1d.10.2 keeps apart from "proved there are no more".
    pub models: u64,
    pub nogoods_emitted: u64,
    pub nogoods_subsumed: u64,
    /// Singleton `(not h)` writebacks emitted **during** this layer — one of
    /// root's two Phase-2 writes, and the only one that happens inside a
    /// layer. The other, the forced-positive promotion, runs at the boundary
    /// *after* the row closes and is counted by
    /// [`BaseStats::forced_positives`] instead.
    pub writebacks: u64,
    /// The frontier handed to the next layer, before the inter-layer
    /// `alive` retain.
    pub next: u64,
}

/// The `store_lattice` proof's counters — the shared base plus three of its
/// own.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct LatticeStats {
    pub base: BaseStats,
    pub solutions_found: u64,
    pub state_key_merges: u64,
    pub elapsed_seconds: f64,
}

// ── Recorded shapes ────────────────────────────────────────────────

pub struct SolutionRecord {
    pub commitment: CanonicalSetId,
    pub kb: Kb,
    pub firings: Vec<Firing>,
    pub layer: u32,
    /// What this model still **owes** — M1d S1d.2.4.
    ///
    /// Non-empty is the `closed-and-owing` corner stated as a number: a node
    /// that is `consistent ∧ complete` by the generator's test and still has
    /// an undischarged requirement, because `complete` means *the generator
    /// proposes nothing* and not *every obligation has a witness*
    /// (`tests/stdlib/closure/03_closed_and_owing.ein`). No verdict word moves
    /// on it in this stage; [S1d.2.6] is where that is decided.
    ///
    /// [S1d.2.6]: `plans/m1d_satisfiability/p1d.2_obligations/s1d.2.6_verdicts_counters_corpus.md`
    pub owes: Owes,
}

pub struct DeadCommitment {
    pub commitment: CanonicalSetId,
    pub unsat_core: Vec<FactId>,
    pub learned_clause: Vec<FactId>,
    pub layer: u32,
    pub kind: Kind,
    pub state_key: Box<[FactId]>,
}

/// The sound proof `store_lattice` attaches: the solution set, the refutation
/// map, the frontier left alive at the depth cap, and the learned clauses.
pub struct LatticeProof {
    pub solutions: Vec<SolutionRecord>,
    /// Root's own saturation — the derivations that hold before any
    /// hypothesis, in order, plus any a forced positive added later.
    ///
    /// Collected only under `store_lattice`, which is what `--trace` and
    /// `--dump-states` set. Before
    /// [S1a.6.9](../../../../docs/history/m1a_rust/README.md#s1a69--the-fork-entry-delta-the-resumed-saturator)
    /// a trace did not need this: every fork re-derived root's fixpoint, so
    /// the solution node's own firing list happened to contain root's whole
    /// closure. A fork that *resumes* root's saturation does not, so the
    /// unconditional half of the proof has to be carried deliberately — which
    /// is also how a human walkthrough tells it, givens first.
    pub root_firings: Vec<Firing>,
    pub dead_commitments: Vec<DeadCommitment>,
    pub alive_at_end: Vec<CanonicalSetId>,
    pub learned_nogoods: Vec<Box<[FactId]>>,
    pub stats: LatticeStats,
}

// ── Options and errors ─────────────────────────────────────────────

/// What to do when a budget is spent.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OnBudget {
    /// Fail with [`SolveError::Budget`], carrying the partial stats.
    Raise,
    /// Return `Answer::Aborted` instead, so a caller need not catch.
    Verdict,
}

pub struct SolveOptions {
    /// Stop after this many *distinct* solution nodes; `None` exhausts.
    pub stop_after: Option<u64>,
    pub max_set_size: u32,
    pub config: Option<SolverConfig>,
    pub max_time: Option<f64>,
    pub max_enterings: Option<u64>,
    pub store_lattice: bool,
    pub on_budget: OnBudget,
    /// **Deferred integration** — how many enterings share one root
    /// ([S1a.7.0](../../../../docs/history/m1a_rust/README.md#s1a70--the-speculation-audit)
    /// T1a.7.0.5).
    ///
    /// `None` is the sequential engine and the only shipping value: an
    /// entering's root writes — the learned clause and the singleton
    /// `(not h)` writeback — land before the next entering forks, so
    /// candidate *i* sees what candidates `< i` learned.
    ///
    /// `Some(n)` holds those writes back and applies them every `n` enterings
    /// and at every layer end, so a batch of candidates is tested against
    /// **one** KB. `Some(usize::MAX)` is the whole layer.
    ///
    /// This is an **execution** knob, not a semantics one, which is why it
    /// lives here and not in [`SolverConfig`] — a `(config …)` block in a
    /// puzzle file must not be able to set it
    /// ([S1a.7.5](../../../../docs/history/m1a_rust/README.md#s1a75--the---jobs-contract)
    /// T1a.7.5.1 makes the same call for `--jobs`). It changes the
    /// **traversal** and therefore the counters; what it must *not* change is
    /// the answer, which is what `tests/search_invariants.rs` asserts and what
    /// [design/08](../../../../docs/history/m1a_rust/design/08_parallelism.md) §2a
    /// proves.
    pub integrate_every: Option<usize>,
    /// **Coalesce root's layer stack** at the layer barrier once it is this
    /// deep, or `None` never
    /// ([T1a.7.2.0](../../../../docs/history/m1a_rust/README.md#s1a72--level-1-parallel-enterings)).
    ///
    /// Every mid-layer root write seals another layer — [`Kb::fork`] seals the
    /// top so the parent's later appends land in a new one — and **every fork
    /// inherits the whole stack**. `branching/07 -e`'s 162 singleton
    /// writebacks put root at depth 164 and all 11 501 forks walk it;
    /// [`Kb::flatten`] at the barrier puts the 11 297 forks of layers ≥ 2 back
    /// on a stack of one.
    ///
    /// Unlike [`Self::integrate_every`] — which bought the same collapse by
    /// *deferring* the writes, and moved `enterings_total` doing it — this
    /// changes nothing about when a prune lands: root is written exactly when
    /// it is today and only its representation is rebuilt. So it is the one
    /// knob of the pair whose default is not `None`, and the invariance it
    /// owes is the strong one: same answer **and** same counters
    /// (`tests/search_invariants.rs`).
    ///
    /// The threshold is a measurement, not a constant: [`Kb::materialise`] is
    /// O(facts), so a barrier that rebuilds a 30 000-fact root to save one
    /// stack walk loses. `Some(3)` is "something was written during the
    /// layer": a barrier with no mid-layer write leaves root at depth 2, so 3
    /// is the first depth a writeback can produce
    /// ([scaling.md §6](../../../../docs/history/m1a_rust/measurements/scaling.md#6-t1a720--the-layer-stack-coalesced-at-the-barrier)).
    pub coalesce_root_at: Option<usize>,
    /// How many threads evaluate a layer's enterings. `1` is the default and
    /// is the sequential engine, line for line
    /// ([T1a.7.2.1](../../../../docs/history/m1a_rust/README.md#s1a72--level-1-parallel-enterings)).
    ///
    /// **`--jobs N` is the same computation as `--jobs 1`** — the same
    /// verdict, the same models, the same unsat core and *the same counters* —
    /// because a layer is fanned out only when it cannot write a fact to root
    /// (`Run::fan_out_this_layer`) and because every result is committed in
    /// candidate order. What differs is the wall clock, and nothing else.
    ///
    /// An **execution** knob, not a semantics one, which is why it lives here
    /// and not in [`ein_core::SolverConfig`]: a `(config …)` block in a puzzle
    /// file must not be able to set it.
    pub jobs: usize,
}

impl Default for SolveOptions {
    fn default() -> Self {
        SolveOptions {
            stop_after: None,
            max_set_size: 5,
            config: None,
            max_time: None,
            max_enterings: None,
            store_lattice: false,
            on_budget: OnBudget::Raise,
            integrate_every: None,
            coalesce_root_at: Some(3),
            jobs: 1,
        }
    }
}

#[derive(Debug)]
pub enum SolveError {
    Budget {
        reason: String,
        stats: Box<MonotonicStats>,
    },
    Saturate(SaturateError),
    Compile(CompileError),
    /// `lattice_order_seed` is set and the traversal shuffle is not ported —
    /// see [`Q-M1a.5`](../../../../docs/history/m1a_rust/open_questions.md).
    Unsupported(String),
    /// `-y` found two lattice paths to one commitment that saturate to
    /// different KBs. Fatal in ein.py too — `check_commutativity` raises.
    Sanity(Box<crate::sanity::SanityError>),
}

impl std::fmt::Display for SolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolveError::Budget { reason, .. } => f.write_str(reason),
            SolveError::Saturate(e) => write!(f, "{e}"),
            SolveError::Compile(e) => write!(f, "{e}"),
            SolveError::Unsupported(m) => f.write_str(m),
            SolveError::Sanity(e) => write!(f, "{e}"),
        }
    }
}

impl From<SaturateError> for SolveError {
    fn from(e: SaturateError) -> Self {
        SolveError::Saturate(e)
    }
}

impl From<CompileError> for SolveError {
    fn from(e: CompileError) -> Self {
        SolveError::Compile(e)
    }
}

/// What a dumper records about one entering.
///
/// ein.py hands its hook the whole `CommitmentSetResult`; here the fields are
/// borrowed separately, because by the time the *solution* outcome is known
/// the result has been partially moved — its KB into the completed fork, its
/// firings into the recorded node. Same information, one borrow later.
pub struct EnteringInfo<'a> {
    pub kind: Kind,
    pub firings: &'a [Firing],
    pub unsat_core: &'a [FactId],
    /// The saturated fork. Written out only for a solution, but passed on
    /// every outcome so the hook, not the loop, decides.
    pub kb: Option<&'a Kb>,
    pub facts_merged: u64,
    pub nogood_emitted: bool,
    pub nogood_subsumed: bool,
}

/// One entering, at the point where the committing thread takes over.
///
/// The split is [S1a.7.2](../../../../docs/history/m1a_rust/README.md#s1a72--level-1-parallel-enterings)'s
/// and it is where a fanned-out layer cuts: [`Run::speculate`] produces one of
/// these against a root nobody may write to, and [`Run::commit_entering`]
/// turns it into counters, clauses, events and nodes **in candidate order**.
/// Nothing in `speculate` touches the run's state; nothing in
/// `commit_entering` looks at a fork it was not given.
struct Entered {
    kind: Kind,
    unsat_core: Vec<FactId>,
    /// The derivations, for the dumper hook and for a recorded solution.
    /// Emptied by the worker when neither will read them — see [`Entered::kb`].
    firings: Vec<Firing>,
    /// The fork, after `complete()` has probed it — an alive entering's KB is
    /// mutated by that probe (the lookahead kill cache writes into it), which
    /// is why the probe belongs to the speculative half and not to the commit.
    ///
    /// **`None` when the worker dropped it**, which it does whenever nothing at
    /// the commit would read it: not a solution node, no `store_lattice`, and a
    /// dumper that does not ask ([`Dumper::reads_forks`]). That is not a
    /// micro-optimisation — freeing a fork on the committing thread is freeing
    /// memory some *other* thread allocated, which every modern allocator makes
    /// the slow path, and it was **192 ms of `features/01 -e`'s 269 ms commit
    /// loop** at `--jobs 8`
    /// ([scaling.md §8](../../../../docs/history/m1a_rust/measurements/scaling.md#the-commits-real-cost-and-it-is-not-the-commit)).
    kb: Option<Kb>,
    /// Alive **and** complete: a solution node.
    solved: bool,
    /// What this node's fixpoint owes — [`crate::obligations`]. Empty on a
    /// dead node, where the tally is unobservable, and on every program that
    /// declares no obligation.
    owes: Owes,
}

/// What one worker brought back: the entering, its records, its narration.
///
/// The record region travels **with** the result rather than in a side table,
/// and that is what makes the ordered commit safe: an id issued inside a fork
/// means something only against the region that issued it, and there is no way
/// to install one entering's records and read another's.
#[cfg(feature = "parallel")]
struct Speculated {
    /// `None` when the worker handed the entering back — it needed to number a
    /// proposition and a lent table cannot ([`ein_core::Overflow::Shared`]).
    /// Nothing else in this struct is usable then.
    entered: Option<Result<Entered, SolveError>>,
    region: ein_core::Region,
    narration: Events,
}

/// The lifecycle hooks a state dumper receives — implemented in
/// [S1a.5.3](../../../../docs/history/m1a_rust/README.md#s1a53--state-dumps)
/// by `ein-render`, which is where formatting lives.
///
/// Every hook that shows a fact takes `&Terms` as well, because a `FactId`
/// means nothing without one: ein.py's `Fact` carries its own text and this
/// port's does not.
#[allow(unused_variables)]
pub trait Dumper {
    fn root_saturating(&mut self, n_firings: usize) {}
    fn root_initial(&mut self, kb: &Kb, terms: &Terms) {}
    fn layer_start(&mut self, layer: u32, kb: &Kb, terms: &Terms, n_alive: usize) {}
    /// The layer's candidates exist and nothing has been entered yet — the
    /// **generation** half of [`LayerCensus`], which is the only moment it is
    /// observable on its own.
    ///
    /// `alive`, `frontier`, `joined`, `dropped_dead`, `dropped_nogood` and
    /// `candidates` are final here; every other column is still zero. A hook
    /// that wants the whole row wants [`Dumper::layer_census`] instead.
    fn layer_generated(&mut self, layer: u32, census: &LayerCensus) {}
    fn entering(
        &mut self,
        layer: u32,
        commitment: &[FactId],
        terms: &Terms,
        outcome: &str,
        info: &EnteringInfo<'_>,
    ) {
    }
    fn layer_end(&mut self, layer: u32, kb: &Kb, terms: &Terms, n_alive: usize, n_next: usize) {}
    /// The layer's census row, complete — the same sixteen counters the
    /// `layer` event carries, handed to a dumper that has no event stream.
    ///
    /// Called from `close_census`, so **every** way out of a layer reaches it,
    /// a budget cut included: a row where `entered < candidates` is the cut,
    /// stated rather than inferred. [`Dumper::layer_end`] is the narrower
    /// hook — it fires only at the ordinary barrier.
    fn layer_census(&mut self, layer: u32, census: &LayerCensus) {}
    /// Does this dumper read [`EnteringInfo::kb`]?
    ///
    /// `true` by default, because a hook that is handed a fork may look at it
    /// and a wrong `false` would hand it `None`. Answering `false` lets a
    /// fanned-out layer's worker **drop the fork where it allocated it**, which
    /// is worth 192 ms of `features/01 -e`'s 269 ms commit loop at `--jobs 8`
    /// ([scaling.md §8](../../../../docs/history/m1a_rust/measurements/scaling.md#the-commits-real-cost-and-it-is-not-the-commit)) —
    /// so it is a claim about the hook, and [`NoDumper`] is the one that can
    /// make it.
    fn reads_forks(&self) -> bool {
        true
    }

    /// Written from the single exit hook when the verdict carries a proof, so
    /// a `kb_index/` tree and its index land *before* the cumulative summary.
    fn proof_summary(&mut self, proof: &LatticeProof, terms: &Terms) {}
    fn summary(&mut self, verdict: &Answer, stats: &MonotonicStats) {}
    fn close(&mut self) {}
}

/// A dumper that does nothing — the common no-dumper path, without an
/// `Option` at every call site.
pub struct NoDumper;
impl Dumper for NoDumper {
    fn reads_forks(&self) -> bool {
        false
    }
}

// ── The loop ───────────────────────────────────────────────────────

/// What one run accumulates.
struct LoopState {
    /// Deduped solution nodes. A `Vec` plus an index rather than a hash map,
    /// because ein.py's is a `dict` and `verdict_of` reads
    /// `list(...values())` — so **insertion order is the branch order** of an
    /// `Ambiguity`, and a replacement keeps its original position.
    nodes: Vec<(Box<[FactId]>, SolutionRecord)>,
    node_at: FxHashMap<Box<[FactId]>, usize>,
    dead: Vec<DeadCommitment>,
    alive_at_end: Vec<CanonicalSetId>,
    state_key_merges: u64,
    /// A `stop_after` or depth-cap cut → `stats.exhausted = false`.
    truncated: bool,
}

/// What the fan-out did — **deliberately not in [`MonotonicStats`]**.
///
/// Every counter in `MonotonicStats` is compared exactly between `--jobs 1`
/// and `--jobs N`, so a number that *must* differ by job count cannot live
/// there. These are the numbers
/// [T1a.7.2.5](../../../../docs/history/m1a_rust/README.md#s1a72--level-1-parallel-enterings)
/// asks for, in the one place where differing is the point.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct JobStats {
    /// Threads a fanned-out layer used. `0` when no layer was fanned out.
    pub workers: usize,
    /// Enterings evaluated on a worker.
    pub speculated: u64,
    /// …of which committed as computed. `speculated - committed` is the waste:
    /// a `stop_after` cut, a spent budget, or an entering handed back.
    pub committed: u64,
    /// Enterings a worker could not finish because it would have had to number
    /// a proposition, and which the committing thread therefore re-ran
    /// ([`ein_core::Overflow::Shared`]). The claim
    /// [shared_state.md §2a](../../../../docs/history/m1a_rust/measurements/shared_state.md#2a-and-a-total-is-the-wrong-shape-of-number-for-it)
    /// rests on, as a running count rather than as a sweep.
    pub handed_back: u64,
    /// Enterings that ran on the sequential path because their layer could
    /// write a fact to root — layer 1 with `enable_singleton_writeback` on.
    /// Amdahl's numerator, per run.
    pub sequential: u64,
}

pub struct Solved {
    pub answer: Answer,
    pub proof: Option<LatticeProof>,
    pub stats: MonotonicStats,
    pub jobs: JobStats,
    /// What the run found outstanding — M1d S1d.2.4. Both halves are empty
    /// for a program that states no obligation, which is every corpus entry
    /// that declares no lower bound.
    pub owes: OwesReport,
}

/// The outstanding obligations a solve found, at the two places a reader asks.
///
/// `root` is the state the search starts from — the number
/// `obligation_forms.md` §5 counted by hand on `zebra2-minus-15`. `models` is
/// one tally per recorded model, in the verdict's branch order, and a
/// non-empty entry there is the `closed-and-owing` corner: a state the
/// generator calls complete that still owes a witness.
///
/// `models` rather than `solutions` on purpose: `ein-einb`'s
/// `nothing_in_the_solve_path_reads_the_solution_store` is a textual guard on
/// `.solutions` in the CLI source — F9's hazard, that a solve could read a
/// *stored* answer — and a second field of that name would make it fire on
/// something it is not about.
#[derive(Clone, Default, Debug)]
pub struct OwesReport {
    pub root: Owes,
    pub models: Vec<Owes>,
    /// How many obligation rules the program **states** — M1d S1d.2.6's scope
    /// rule, and the reason it is a separate number from the two tallies.
    ///
    /// `root.total() == 0` cannot answer "is this program judged by
    /// discharge?": it is equally true of a program that owes nothing because
    /// every debt is paid and of one that owes nothing because it never said
    /// what it owed. Only the first may be called *satisfied* by discharge —
    /// so the read-out asks this, and 0 means the verdict words are exactly
    /// what they were before P1d.2.
    pub declared: usize,
}

impl OwesReport {
    /// Nothing to report — no obligation was stated, or every one is
    /// discharged everywhere.
    pub fn is_empty(&self) -> bool {
        self.root.is_empty() && self.models.iter().all(Owes::is_empty)
    }

    /// Whether the three-state read-out applies at all — S1d.2.6's scope rule.
    ///
    /// A state is judged by *discharge* when it has been told what it owes,
    /// and by *exhaustion* when it has not.
    pub fn in_scope(&self) -> bool {
        self.declared > 0
    }
}

/// The one solver entry.
///
/// Its verdict is *read* from the result — `k` distinct solution nodes — never
/// chosen up front.
pub fn solve(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    events: &mut Events,
    dumper: &mut dyn Dumper,
    opts: &SolveOptions,
) -> Result<Solved, SolveError> {
    let cfg = opts
        .config
        .clone()
        .or_else(|| root.program().config.clone())
        .unwrap_or_default();
    root.program_mut().config = Some(cfg.clone());
    // One generator per solve, **not** one per layer: its state advances
    // across layers, so each layer gets a different permutation from the same
    // seed. That is what makes a seeded run replayable, and it is why the
    // generator has to be CPython's bit for bit (Q-M1a.5).
    let shuffle = cfg.lattice_order_seed.map(crate::mt19937::Mt19937::seeded);
    let mut run = Run {
        shuffle,
        cfg,
        memo: SharedMemo::default(),
        root_snapshot: None,
        root_firings: Vec::new(),
        root_owes: Owes::default(),
        declares_obligations: false,
        stats: MonotonicStats::new(),
        lstate: LoopState {
            nodes: Vec::new(),
            node_at: FxHashMap::default(),
            dead: Vec::new(),
            alive_at_end: Vec::new(),
            state_key_merges: 0,
            truncated: false,
        },
        deferred: Vec::new(),
        jobs: JobStats::default(),
        census: LayerCensus::default(),
        #[cfg(feature = "parallel")]
        pool: (opts.jobs > 1)
            .then(|| {
                rayon::ThreadPoolBuilder::new()
                    .num_threads(opts.jobs)
                    .thread_name(|i| format!("ein-worker-{i}"))
                    .build()
                    .ok()
            })
            .flatten(),
        t_start: Instant::now(),
        opts,
    };
    match run.go(root, terms, ast, events, dumper) {
        Ok(answer) => {
            let proof = if opts.store_lattice {
                Some(run.proof(root))
            } else {
                None
            };
            // The single exit hook, in ein.py's two-step order: the proof
            // index first, so a `LatticeDumper` materialises `kb_index/` and
            // `proof_summary.json` before the cumulative summary lands.
            if let Some(proof) = proof.as_ref() {
                dumper.proof_summary(proof, terms);
            }
            dumper.summary(&answer, &run.stats);
            // The recorded nodes are in the verdict's branch order, which is
            // `lstate.nodes`' order — the same walk `finalise` made.
            let owes = OwesReport {
                root: run.root_owes.clone(),
                models: run
                    .lstate
                    .nodes
                    .iter()
                    .map(|(_, n)| n.owes.clone())
                    .collect(),
                declared: root.program().obligations.len(),
            };
            Ok(Solved {
                answer,
                proof,
                stats: run.stats,
                jobs: run.jobs,
                owes,
            })
        }
        Err(SolveError::Budget { reason, stats }) if opts.on_budget == OnBudget::Verdict => {
            Ok(Solved {
                answer: Answer::Aborted { reason },
                proof: None,
                stats: *stats,
                jobs: JobStats::default(),
                // A budget cut is not a fixpoint, so there is no state whose
                // debts this could be about.
                owes: OwesReport::default(),
            })
        }
        Err(e) => Err(e),
    }
}

/// A commit-time root write, held back by `integrate_every`.
///
/// These are the only two things an entering writes to root — everything else
/// it produces is fork-local or lives in `LoopState`
/// ([design/08](../../../../docs/history/m1a_rust/design/08_parallelism.md) §2a). So
/// buffering exactly these two is what makes a batch of enterings share one
/// KB.
enum Deferred {
    /// The learned clause, for [`crate::nogoods::emit_nogood`].
    Nogood(Vec<FactId>),
    /// `h`, for the singleton `(not h)` writeback.
    Writeback(FactId),
}

struct Run<'o> {
    shuffle: Option<crate::mt19937::Mt19937>,
    cfg: SolverConfig,
    /// The run's compiled plans, shared by the root saturation, every
    /// entering, every `complete` / `open_hypotheses` probe and every
    /// `lookahead` — design/06 § Win A, and the whole of
    /// [S1a.6.8](../../../../docs/history/m1a_rust/README.md#s1a68--the-compile-cache-and-the-extent-counts).
    /// Each engine still keeps its own ordered plan list, which is the part
    /// that reaches the trace.
    memo: SharedMemo,
    /// The root saturator at its fixpoint, for a fork that **resumes** it
    /// instead of re-deriving it
    /// ([S1a.6.9](../../../../docs/history/m1a_rust/README.md#s1a69--the-fork-entry-delta-the-resumed-saturator)).
    ///
    /// Refreshed after every root *re*-saturation (a forced positive), and
    /// otherwise left alone: root's other writers — the singleton `(not h)`
    /// writeback, the lookahead kill cache — add a fact without re-reaching
    /// the fixpoint, and `Snapshot::new_facts_of` hands those to the fork as
    /// part of its delta rather than invalidating the snapshot.
    root_snapshot: Option<Arc<Snapshot>>,
    /// See [`LatticeProof::root_firings`]. Empty unless `store_lattice`.
    root_firings: Vec<Firing>,
    /// What root's fixpoint owes — read once Phase 1 has settled root
    /// (saturation, then the forced-positive cascade), which is the state the
    /// search starts from and the one `obligation_forms.md` §5 counted by
    /// hand. Empty when root is contradictory: the read-out consults
    /// `(false)` first, so a dead root's debts are unobservable.
    root_owes: Owes,
    /// Whether the program **states** an obligation — S1d.2.6's scope rule,
    /// latched once at root's fixpoint because it is a property of the loaded
    /// program and not of any state. `false` is 119 of the 146 corpus entries
    /// that reach a fixpoint, and for those `finalise` is bit-for-bit the
    /// pre-P1d.2 read-out.
    declares_obligations: bool,
    stats: MonotonicStats,
    lstate: LoopState,
    /// Root writes waiting for the next integration barrier. Always empty
    /// when `opts.integrate_every` is `None`.
    deferred: Vec<Deferred>,
    /// What the fan-out did — see [`JobStats`], and note that it is not part
    /// of [`MonotonicStats`] on purpose.
    jobs: JobStats,
    /// The layer being counted — reset at every layer's open, emitted at its
    /// close. Lives here rather than in `phase2`'s frame because the two
    /// columns nothing else reports are bumped four call frames down, in
    /// [`Self::handle_dead`] and [`Self::integrate`].
    census: LayerCensus,
    /// The fan-out's workers, **built once per solve and parked between
    /// batches**.
    ///
    /// A layer runs in bounded batches so that the results in flight cannot
    /// grow with the layer, and that makes the *cost of a batch* the thing to
    /// watch: spawning `jobs` threads per batch cost 96 000 spawns and a 3×
    /// **slowdown** at `--jobs 2` on `features/01 -e`, which is why this is a
    /// pool and not a `std::thread::scope`
    /// ([scaling.md §8](../../../../docs/history/m1a_rust/measurements/scaling.md)).
    ///
    /// `None` at `--jobs 1`, which is the default: a sequential solve builds no
    /// threads at all.
    #[cfg(feature = "parallel")]
    pool: Option<rayon::ThreadPool>,
    t_start: Instant,
    opts: &'o SolveOptions,
}

#[cfg(feature = "parallel")]
/// Enterings in flight per worker — the fan-out's batch, over [`jobs`].
///
/// [`SolveOptions::jobs`]: SolveOptions::jobs
/// [`jobs`]: SolveOptions::jobs
fn batch_per_worker() -> usize {
    std::env::var("EIN_BATCH_PER_WORKER")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|&n: &usize| n > 0)
        .unwrap_or(BATCH_PER_WORKER)
}

#[cfg(feature = "parallel")]
const BATCH_PER_WORKER: usize = 512;

/// Does a fork **resume** root's saturation? Yes, since
/// [S1a.6.9](../../../../docs/history/m1a_rust/README.md#s1a69--the-fork-entry-delta-the-resumed-saturator)
/// — this is the shipping path, and
/// [D3](../../../../docs/history/m1a_rust/divergences.md) is what it costs.
///
/// The one way to get the old fresh-saturator path back is a `fork-delta`
/// build with `EIN_FORK_DELTA=0`, and that exists for one reason: D3's rule 2
/// wants a fixture that demonstrates the divergence and keeps it from
/// silently widening. `utils/fork_delta_verify.py` is that fixture, and it
/// needs both arms out of one binary.
fn resume_forks() -> bool {
    !(cfg!(feature = "fork-delta") && std::env::var_os("EIN_FORK_DELTA").is_some_and(|v| v == "0"))
}

impl Run<'_> {
    fn go(
        &mut self,
        root: &mut Kb,
        terms: &mut Terms,
        ast: &Ast,
        events: &mut Events,
        dumper: &mut dyn Dumper,
    ) -> Result<Answer, SolveError> {
        match self.phase1(root, terms, ast, events, dumper)? {
            Phase1::Done => return Ok(self.finalise()),
            Phase1::Continue { alive, a_prev } => {
                self.phase2(root, terms, ast, events, dumper, alive, a_prev)?;
            }
        }
        Ok(self.finalise())
    }

    // ── Phase 1 ────────────────────────────────────────────────

    fn phase1(
        &mut self,
        root: &mut Kb,
        terms: &mut Terms,
        ast: &Ast,
        events: &mut Events,
        dumper: &mut dyn Dumper,
    ) -> Result<Phase1, SolveError> {
        {
            let mut s = Session {
                kb: root,
                terms,
                ast,
                events,
                memo: self.memo.clone(),
            };
            let mut sat = Saturator::new(&mut s)?;
            // Root saturation is the slow part of a Phase-1 solve, so the
            // firing count streams to the dumper. Both paths run the same
            // loop — ein.py splits them to keep the C-speed `list()` drain on
            // the common path, and the split is a cost decision, not a
            // behavioural one.
            let mut n = 0usize;
            let keep = self.opts.store_lattice;
            let root_firings = &mut self.root_firings;
            sat.saturate(&mut s, None, &mut |f| {
                n += 1;
                if keep {
                    root_firings.push(f.clone());
                }
                if n.is_multiple_of(ROOT_SAT_PROGRESS_EVERY) {
                    dumper.root_saturating(n);
                }
            })?;
            self.stats.base.saturate_count += 1;
            if self.cfg.warn_derived_naf {
                // Post-saturation, so the cache holds the plans of rules with
                // rule-derived activators. The warnings reuse this
                // saturator's populated engine rather than recompiling.
                for w in crate::naf_deps::derived_naf_warnings(&sat.engine, s.terms) {
                    s.events.emit("warn", |l| {
                        l.str("category", "DerivedNafWarning");
                        l.str("message", &w);
                    });
                }
            }
            self.snapshot_root(&sat, s.kb);
        }
        dumper.root_initial(root, terms);

        if crate::contradiction::has_contradiction(root, terms) {
            // Root is contradictory before any commitment → `k = 0`, with the
            // source-frontier core.
            self.root_dead(root, terms);
            return Ok(Phase1::Done);
        }

        let mut alive = self.compute_alive(root, terms, ast, events)?;
        if self.cfg.enable_forced_positive {
            let (next, term) = self.promote_forced_positives(root, terms, ast, events, alive)?;
            alive = next;
            if term {
                // `solve` never goal-terminates the cascade, so a terminal
                // here is a contradiction → `k = 0`.
                self.root_dead(root, terms);
                return Ok(Phase1::Done);
            }
        }
        // Root's fixpoint, after the cascade that may still have moved it:
        // the state the search starts from, and the one a reader means by
        // "what does this puzzle still owe".
        self.root_owes = crate::obligations::tally(root, terms, ast, &self.memo, events)?;
        self.declares_obligations = !root.program().obligations.is_empty();
        if alive.is_empty() {
            // Empty alive and no contradiction ⇒ root is itself a complete,
            // consistent model — the unique solution.
            let owes = self.root_owes.clone();
            self.record_node(root, terms, Vec::new(), Vec::new(), 0, owes);
            return Ok(Phase1::Done);
        }
        let a_prev = layer_1(terms, &alive);
        Ok(Phase1::Continue { alive, a_prev })
    }

    // ── Phase 2 ────────────────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    fn phase2(
        &mut self,
        root: &mut Kb,
        terms: &mut Terms,
        ast: &Ast,
        events: &mut Events,
        dumper: &mut dyn Dumper,
        mut alive: FxHashSet<FactId>,
        mut a_prev: Vec<CanonicalSetId>,
    ) -> Result<(), SolveError> {
        // T1d.10.5.0 — the cap at **zero** cuts before the first layer, and
        // the loop below cannot say so because it never runs. Same rule the
        // cap applies inside it (`layer == max_set_size`: a non-empty frontier
        // at the cap means the lattice was not explored), evaluated one step
        // earlier — Phase 1 reaches here only with `alive` non-empty, so
        // `a_prev` is a set of commitments this run will never look at.
        //
        // `alive_at_end` stays **empty** on purpose, and that is the one place
        // this differs from the in-loop cut: the field is the commitments that
        // were entered and *survived*, and at a cap of zero nothing was
        // entered. Truncated with no frontier to hand a deeper run is
        // `stop_after`'s shape, not the depth cap's, and claiming 96
        // never-entered singletons had survived would be the same
        // overstatement in the other direction.
        if self.opts.max_set_size == 0 {
            debug_assert!(
                !a_prev.is_empty(),
                "Phase 1 continues only with a live frontier"
            );
            self.lstate.truncated = true;
            return Ok(());
        }
        let mut phase_2_done = false;
        for layer in 1..=self.opts.max_set_size {
            if phase_2_done {
                break;
            }
            self.stats.base.layers_explored = layer as u64;
            dumper.layer_start(layer, root, terms, alive.len());

            // T1d.10.1.1 — the layer's census row opens here and closes at
            // `layer_end`. The `base` snapshot is the denominator's other
            // half: every per-layer number below is a difference of two
            // whole-run counters, so a counter added to `BaseStats` is in the
            // census without anyone re-deriving it.
            let base_at_open = self.stats.base;
            let models_at_open = self.lstate.nodes.len() as u64;
            self.census = LayerCensus {
                alive: alive.len() as u64,
                frontier: a_prev.len() as u64,
                ..LayerCensus::default()
            };

            let candidates = if layer == 1 {
                // No join at layer 1: `a_prev` *is* the singletons of `alive`,
                // already filtered by construction.
                self.census.joined = a_prev.len() as u64;
                a_prev.clone()
            } else {
                let store = root.nogoods().clone();
                let guard = store.read().expect("the no-good store");
                let mut census = std::mem::take(&mut self.census);
                let out = self.generate_layer(terms, &a_prev, &alive, &guard, &mut census);
                self.census = census;
                out
            };
            self.census.candidates = candidates.len() as u64;
            dumper.layer_generated(layer, &self.census);
            let mut candidates = order_candidates(root, terms, candidates, &self.cfg.lattice_order)
                .map_err(|e| SolveError::Compile(CompileError(e.to_string())))?;
            // After `order_candidates`, never instead of it — the shuffle is a
            // permutation of the ordered list, and the harness that probes
            // traversal-order dependence needs both to have happened.
            if let Some(rng) = self.shuffle.as_mut() {
                rng.shuffle(&mut candidates);
            }

            // S1a.7.0 — the speculative arm, when the build asked for it.
            // Opened *after* the order is fixed, because what it audits is
            // this layer's candidates in this layer's order.
            #[cfg(feature = "spec-audit")]
            let mut audit = crate::spec_audit::LayerAudit::start(root, layer);

            // T1a.7.2.8 — the invariant the whole fan-out decision rests on.
            // A layer this predicate calls fanned-out must not see root grow
            // under it, because a worker forks root once at the layer's open
            // and every later fork of the same layer has to be the same KB.
            // Root's fact count is the exact statement of that: `depth()` is
            // not, because the layer's *first* fork seals root's top whether or
            // not anything was written.
            let fanned_out = self.fan_out_this_layer(layer);
            let facts_at_open = root.n_facts();

            let mut a_layer: Vec<CanonicalSetId> = Vec::new();
            let jobs = self.opts.jobs.max(1);
            // T1a.7.2.1 — the fan-out, and the three things that turn it off.
            // `fanned_out` is the predicate the whole decision rests on; the
            // `spec-audit` arm compares each entering against `R0` *in order*
            // and a fanned-out layer's forks already are `R0`; and one thread
            // is the sequential engine, line for line.
            #[allow(unused_mut, unused_variables)]
            let mut fan_out = fanned_out && jobs > 1 && cfg!(feature = "parallel");
            #[cfg(feature = "spec-audit")]
            {
                fan_out = fan_out && audit.is_none();
            }
            // **How many enterings are in flight**, and it is a memory
            // question before it is a scheduling one: every speculated result
            // holds a fork's KB and its record region until the commit reaches
            // it, so a batch of the whole layer holds the whole layer. On
            // `features/01 -e` — 384 167 enterings in one layer — that is 84 MB
            // against **1.9 GB**, measured, which is also why it was *slower*
            // at `--jobs 2` than at `--jobs 1`
            // ([scaling.md §8](../../../../docs/history/m1a_rust/measurements/scaling.md)).
            //
            // A bounded batch fixes both. It stays a multiple of `jobs` because
            // the shared cursor needs slack to balance a layer whose enterings
            // cost wildly different amounts — a `dead-pre` is a contradiction
            // check and an alive one is a whole saturation.
            //
            // **A cut narrows it, and the narrowing has to be temporary**
            // (T1a.7.2.4). `stop_after`, `max_enterings` and `max_time` all
            // stop the run mid-layer, and everything speculated past the cut
            // is thrown away — so a run that can cut starts at one round of
            // workers and *doubles* from there, capped at the full batch:
            //
            //     batch = clamp(enterings committed so far, jobs, jobs × 32)
            //
            // which bounds the waste by the work. The enterings discarded at a
            // cut are at most one batch, and a batch is at most what has
            // already been committed, so **a cut can never more than double a
            // run's work** — while a search that never cuts pays the small
            // batch only for its first `jobs × 32` enterings and runs at full
            // width after that.
            //
            // The flat `batch = jobs` this replaces was right about the cut and
            // wrong about everything else: `-n 1` is the CLI's *default*, and
            // three of the four workloads of the phase's measurement set never
            // reach a solution under it — so the common invocation paid a
            // barrier every `jobs` enterings for a search that cut nothing.
            // That is `houses -n 1` at 2.72× where `houses -e` is 4.38×, for
            // the same 21 699 enterings
            // ([scaling.md §8a](../../../../docs/history/m1a_rust/measurements/scaling.md#8a-t1a724--the-early-stop-and-the-batch-that-was-flat)).
            #[cfg(feature = "parallel")]
            let full_batch = jobs.saturating_mul(batch_per_worker());
            #[cfg(feature = "parallel")]
            let may_cut = self.opts.stop_after.is_some()
                || self.opts.max_enterings.is_some()
                || self.opts.max_time.is_some();

            let mut i = 0usize;
            while i < candidates.len() {
                #[cfg(feature = "parallel")]
                if fan_out && candidates.len() - i > 1 {
                    let batch = if may_cut {
                        (self.stats.base.enterings_total as usize).clamp(jobs, full_batch)
                    } else {
                        full_batch
                    };
                    let end = i.saturating_add(batch).min(candidates.len());
                    // Once for the batch, not once per worker: sealing is the
                    // half of `Kb::fork` that mutates, and after it every
                    // worker branches the same root through a `&`.
                    root.seal_top();
                    // `lend` rather than `share` + `reclaim`, so the pairing is
                    // the borrow checker's rather than this function's: an
                    // early `?` or a worker panic between the two would leave
                    // the tables lent, and a lent `Terms` is one that has
                    // silently stopped growing (T1a.7.5.6).
                    let speculated = {
                        let lent = terms.lend();
                        self.fan_out(
                            root,
                            lent.get(),
                            ast,
                            events.narration(),
                            self.opts.store_lattice || dumper.reads_forks(),
                            layer,
                            &candidates[i..end],
                        )
                    };
                    self.jobs.workers = self.jobs.workers.max(jobs.min(end - i));
                    self.jobs.speculated += (end - i) as u64;

                    for (k, sp) in speculated.into_iter().enumerate() {
                        let c = &candidates[i + k];
                        if let Err(e) = self.before_commit(i + k, root, terms, events, dumper) {
                            self.close_census(
                                events,
                                dumper,
                                layer,
                                base_at_open,
                                models_at_open,
                                a_layer.len(),
                            );
                            return Err(e);
                        }
                        let Speculated {
                            entered,
                            region,
                            narration,
                        } = sp;
                        let (entered, region) = match entered {
                            Some(entered) => {
                                // The worker's narration is the run's, and this
                                // is where it gets the run's ordinals.
                                events.replay(narration);
                                self.jobs.committed += 1;
                                (entered?, Some(region))
                            }
                            None => {
                                // Handed back: the entering needed to number a
                                // proposition and a lent table cannot. Nothing
                                // the worker produced after the refusal is what
                                // the sequential engine would have produced, so
                                // none of it is used — its narration included.
                                // Re-running here puts the records in root's own
                                // region, so there is nothing to install.
                                self.jobs.handed_back += 1;
                                drop(region);
                                drop(narration);
                                terms.provs.open_fork();
                                let result = try_commitment_set(
                                    root.sealed(),
                                    terms,
                                    ast,
                                    events,
                                    &self.memo,
                                    c,
                                    None,
                                    self.root_snapshot.as_deref(),
                                )?;
                                (
                                    self.finish_entering(terms, ast, events, layer, c, result)?,
                                    None,
                                )
                            }
                        };
                        // The region travels with the result, so installing it
                        // is what makes the fork's derivations readable — and
                        // swapping it back is what keeps root's own sequence
                        // where it was.
                        let saved = region.map(|r| terms.provs.swap_fork(r));
                        let stop = self.commit_entering(
                            root,
                            terms,
                            ast,
                            events,
                            dumper,
                            layer,
                            c,
                            entered,
                            &mut a_layer,
                        );
                        if let Some(saved) = saved {
                            terms.provs.swap_fork(saved);
                        }
                        if stop? {
                            self.close_census(
                                events,
                                dumper,
                                layer,
                                base_at_open,
                                models_at_open,
                                a_layer.len(),
                            );
                            return Ok(());
                        }
                    }
                    i = end;
                    continue;
                }

                let c = &candidates[i];
                if !fanned_out {
                    self.jobs.sequential += 1;
                }
                if let Err(e) = self.before_commit(i, root, terms, events, dumper) {
                    self.close_census(
                        events,
                        dumper,
                        layer,
                        base_at_open,
                        models_at_open,
                        a_layer.len(),
                    );
                    return Err(e);
                }
                // T1a.7.1.7 — everything derived from here to `close_fork` is
                // the *fork's* own, and dies with the fork. The region covers
                // the whole entering and not just `try_commitment_set`:
                // `complete`'s lookahead kill-cache writes into the fork too,
                // as do the nested forks a `-y` commutativity check makes.
                // Root's own records — the no-good, the singleton writeback,
                // the forced-positive promotion — are written after the region
                // closes, and land in the arena proper.
                terms.provs.open_fork();
                // T1a.7.1.2 — what this entering appends to the *shared*
                // tables, which is what decides whether a worker can be handed
                // a `&Terms` and nothing else. Compiled out with the
                // `counters` feature; `snapshot()` is a thread-local read.
                #[cfg(feature = "counters")]
                let before = ein_core::counters::snapshot();
                let result = try_commitment_set(
                    root.sealed(),
                    terms,
                    ast,
                    events,
                    &self.memo,
                    c,
                    None,
                    self.root_snapshot.as_deref(),
                )?;
                #[cfg(feature = "counters")]
                {
                    let after = ein_core::counters::snapshot();
                    let (facts, provs) = (
                        after.fact_new > before.fact_new,
                        after.prov_push > before.prov_push,
                    );
                    ein_core::counters::bump(|c| {
                        c.entering += 1;
                        c.entering_fact_new += u64::from(facts);
                        c.entering_prov_new += u64::from(provs);
                        c.prov_push_in_entering += after.prov_push - before.prov_push;
                        if facts {
                            c.entering_fact_new_max_i = c.entering_fact_new_max_i.max(i as u64 + 1);
                        }
                    });
                }
                // Before the commit step, which is what grows `W`.
                #[cfg(feature = "spec-audit")]
                if let Some(a) = audit.as_mut() {
                    a.check(
                        root,
                        terms,
                        ast,
                        &self.memo,
                        c,
                        &result,
                        self.root_snapshot.as_deref(),
                    );
                }
                let entered = self.finish_entering(terms, ast, events, layer, c, result)?;
                if self.commit_entering(
                    root,
                    terms,
                    ast,
                    events,
                    dumper,
                    layer,
                    c,
                    entered,
                    &mut a_layer,
                )? {
                    self.close_census(
                        events,
                        dumper,
                        layer,
                        base_at_open,
                        models_at_open,
                        a_layer.len(),
                    );
                    return Ok(());
                }
                i += 1;
            }

            debug_assert!(
                !fanned_out || root.n_facts() == facts_at_open,
                "layer {layer} was fanned out and root grew from {} to {} \
                 facts while it ran — see Run::fan_out_this_layer. \
                 Whatever wrote is a `W` the parallel path does not repair, \
                 and design/08 §2's validator is the design that was \
                 measured for it",
                facts_at_open,
                root.n_facts()
            );

            // The layer barrier. With `integrate_every = Some(usize::MAX)`
            // this is the *only* one, which is the "one KB per layer" mode.
            self.integrate(root, terms, events);
            self.close_census(
                events,
                dumper,
                layer,
                base_at_open,
                models_at_open,
                a_layer.len(),
            );
            dumper.layer_end(layer, root, terms, alive.len(), a_layer.len());
            // T1a.7.2.0 — and after the dumper, so what it renders is the KB
            // it renders today. Coalescing here rather than at the next
            // layer's start is worth the two lines' difference: `compute_alive`
            // forks root and `promote_forced_positives` re-*saturates* it, and
            // both run below on whatever stack this layer left behind.
            self.coalesce_root(root);
            if phase_2_done {
                break;
            }
            if a_layer.is_empty() {
                break;
            }
            // The sound inter-layer prune: this layer's deaths wrote `¬g`, so
            // recompute `alive` and promote any backbone singletons. Not a
            // fork-fact merge — that extraction was retired in P1.21 R2.
            alive = self.compute_alive(root, terms, ast, events)?;
            if self.cfg.enable_forced_positive {
                let (next, term) =
                    self.promote_forced_positives(root, terms, ast, events, alive)?;
                alive = next;
                if term {
                    self.root_dead(root, terms);
                    break;
                }
            }
            if alive.is_empty() {
                // The backbone determines every cell. Root moved since
                // Phase 1 — the cascade promoted facts into it — so the tally
                // is re-read rather than reused.
                let owes = crate::obligations::tally(root, terms, ast, &self.memo, events)?;
                self.root_owes = owes.clone();
                self.record_node(root, terms, Vec::new(), Vec::new(), 0, owes);
                break;
            }
            // Drop the commitments no longer entirely within `alive` — an
            // element got promoted into root or refuted.
            a_layer.retain(|c| c.iter().all(|e| alive.contains(e)));
            if a_layer.is_empty() {
                break;
            }
            if layer == self.opts.max_set_size {
                // A non-empty frontier at the depth cap means the lattice was
                // not fully explored.
                self.lstate.alive_at_end = a_layer.clone();
                self.lstate.truncated = true;
            }
            a_prev = a_layer;
            let _ = &mut phase_2_done;
        }
        Ok(())
    }

    /// Close the layer's census row and narrate it — the `layer` event.
    ///
    /// Every column but three is a difference of two whole-run counters taken
    /// at the layer's open and its close, which is what keeps the row honest
    /// when a budget cuts the layer half-way: `entered < candidates` is then
    /// the cut, stated rather than inferred.
    ///
    /// Called from the layer barrier, **after** [`Self::integrate`], so a
    /// deferred clause counts in the layer whose entering produced it — and
    /// from **every** other way out of a layer: the `stop_after` cut and the
    /// `-T` / `-E` budget, on both the sequential and the fanned-out path. A
    /// row per layer with no exceptions is what makes `Σ entered =
    /// enterings_total` an invariant rather than a usual case, and it is what
    /// turns a budget into a **probe**: `solve -e -m 4 -E <n>` on a search
    /// nobody can finish reports what layer 4's join actually proposed, which
    /// is the number [S1d.10.2](../../../../plans/m1d_satisfiability/p1d.10_exhaustive_search/s1d.10.2_depth_required.md)
    /// wants and no completed run can supply.
    fn close_census(
        &mut self,
        events: &mut Events,
        dumper: &mut dyn Dumper,
        layer: u32,
        at_open: BaseStats,
        models_at_open: u64,
        next: usize,
    ) {
        let now = self.stats.base;
        let c = &mut self.census;
        c.entered = now.enterings_total - at_open.enterings_total;
        c.alive_enterings = now.enterings_alive - at_open.enterings_alive;
        c.dead_pre = now.enterings_dead_pre - at_open.enterings_dead_pre;
        c.dead_post = now.enterings_dead_post - at_open.enterings_dead_post;
        c.nogoods_emitted = now.nogoods_emitted - at_open.nogoods_emitted;
        c.nogoods_subsumed = now.nogoods_subsumed - at_open.nogoods_subsumed;
        c.models = self.lstate.nodes.len() as u64 - models_at_open;
        c.next = next as u64;
        if events.on() {
            let c = self.census;
            events.emit("layer", |l| {
                l.num("layer", layer as i64);
                l.num("alive", c.alive as i64);
                l.num("frontier", c.frontier as i64);
                l.num("joined", c.joined as i64);
                l.num("dropped_dead", c.dropped_dead as i64);
                l.num("dropped_nogood", c.dropped_nogood as i64);
                l.num("candidates", c.candidates as i64);
                l.num("entered", c.entered as i64);
                l.num("alive_enterings", c.alive_enterings as i64);
                l.num("dead_pre", c.dead_pre as i64);
                l.num("dead_post", c.dead_post as i64);
                l.num("models", c.models as i64);
                l.num("nogoods_emitted", c.nogoods_emitted as i64);
                l.num("nogoods_subsumed", c.nogoods_subsumed as i64);
                l.num("writebacks", c.writebacks as i64);
                l.num("next", c.next as i64);
            });
        }
        dumper.layer_census(layer, &self.census);
    }

    /// Layer *L+1*'s candidates — the prefix join, then the downward-closure
    /// filter, **which is the fan-out's second one**.
    ///
    /// [`crate::apriori::filter_candidate`] asks two questions of a candidate:
    /// is every element still alive, and is any learned clause a subset of it.
    /// The second walks the whole no-good store, so the pass is
    /// `candidates × clauses` — **47.7 ms of `branching/07 -e`'s 109 ms** at
    /// `--jobs 8`, which is what made it the largest serial term in Phase 2
    /// once the ordered commit stopped freeing the workers' memory
    /// ([scaling.md §8](../../../../docs/history/m1a_rust/measurements/scaling.md#t1a727--and-the-layers-own-serial-work-turned-out-to-be-three-things)).
    ///
    /// It parallelises for the same reason the enterings do and with less to
    /// argue about: the predicate reads `alive` and the clause store by `&` and
    /// writes nothing at all. The order is kept by computing a **mask** through
    /// an indexed `collect_into_vec` and filtering with it, rather than by
    /// trusting a filtered collect to be ordered — a layer's candidate order
    /// *is* the traversal, so it is worth spending a `Vec<bool>` to make the
    /// claim structural.
    /// The **census** is why this returns the reason rather than a `bool`
    /// (T1d.10.1.1). `filter_reason` asks exactly the questions
    /// `filter_candidate` already asked and in the same order, so the split
    /// costs one byte per candidate in the mask that was a `bool` anyway, and
    /// a fold over it — against a predicate that allocates a `Vec` and walks
    /// the whole clause store per candidate. Measured on
    /// `zebra2-minus-15 -m 3`: within the run-to-run noise of the 48 745-
    /// entering search it counts, which is why it is unconditional rather
    /// than behind [`ein_core::counters`]'s feature. A counter that has to be
    /// asked for is a counter no corpus sweep has.
    fn generate_layer(
        &self,
        terms: &Terms,
        a_prev: &[CanonicalSetId],
        alive: &FxHashSet<FactId>,
        nogoods: &ein_core::Nogoods,
        census: &mut LayerCensus,
    ) -> Vec<CanonicalSetId> {
        use crate::apriori::Filter;
        let joined = crate::apriori::apriori_prefix_join(terms, a_prev);
        census.joined = joined.len() as u64;
        let mut tally = |verdict: Filter| match verdict {
            Filter::Keep => {}
            Filter::Dead => census.dropped_dead += 1,
            Filter::Nogood => census.dropped_nogood += 1,
        };
        #[cfg(feature = "parallel")]
        if let Some(pool) = self.pool.as_ref()
            && joined.len() > 1
        {
            use rayon::prelude::*;
            let mut keep = Vec::with_capacity(joined.len());
            pool.install(|| {
                joined
                    .par_iter()
                    .map(|c| crate::apriori::filter_reason(c, alive, nogoods))
                    .collect_into_vec(&mut keep)
            });
            keep.iter().copied().for_each(&mut tally);
            return joined
                .into_iter()
                .zip(keep)
                .filter_map(|(c, verdict)| (verdict == Filter::Keep).then_some(c))
                .collect();
        }
        joined
            .into_iter()
            .filter(|c| {
                let verdict = crate::apriori::filter_reason(c, alive, nogoods);
                tally(verdict);
                verdict == Filter::Keep
            })
            .collect()
    }

    /// The three things every candidate does before its result is committed,
    /// whoever computed it.
    ///
    /// It is *before* rather than *around* on purpose: a budget that fires
    /// here has not counted this entering, which is the sequential engine's
    /// order and therefore the one `--jobs N` has to reproduce.
    fn before_commit(
        &mut self,
        i: usize,
        root: &mut Kb,
        terms: &mut Terms,
        events: &mut Events,
        dumper: &mut dyn Dumper,
    ) -> Result<(), SolveError> {
        // The batch barrier, at the top so every path out of the body below is
        // covered by one check.
        if let Some(n) = self.opts.integrate_every
            && i > 0
            && i.is_multiple_of(n)
        {
            self.integrate(root, terms, events);
        }
        self.check_budget(dumper)?;
        self.stats.base.enterings_total += 1;
        Ok(())
    }

    /// Evaluate a batch of candidates on `jobs` threads, and return what each
    /// produced **in candidate order**.
    ///
    /// The scheduling is a shared cursor rather than a static split, because a
    /// layer's enterings cost wildly different amounts — a `dead-pre` is a
    /// contradiction check and an alive one is a whole saturation — so handing
    /// worker *w* every `w`-th candidate would leave most of them idle. Order
    /// is recovered by the index each worker records beside its result, not by
    /// the order they finish in: that is the whole of what makes `--jobs N`
    /// the same computation as `--jobs 1`.
    ///
    /// Every argument but `jobs` is shared by `&`. That is not a style choice
    /// — it is the seam T1a.7.2.1 built, and the compiler is what checks it.
    #[cfg(feature = "parallel")]
    #[allow(clippy::too_many_arguments)]
    fn fan_out(
        &self,
        root: &Kb,
        terms: &Terms,
        ast: &Ast,
        narration: Option<crate::events::Level>,
        keep_forks: bool,
        layer: u32,
        batch: &[CanonicalSetId],
    ) -> Vec<Speculated> {
        use rayon::prelude::*;
        let run = |c: &CanonicalSetId| {
            // A view per entering, not per worker: a fresh record region every
            // time is what makes a stale id from the previous entering unable
            // to resolve against this one.
            let mut view = terms.worker();
            let mut narration = Events::worker_for(narration);
            view.provs.open_fork();
            let mut entered = self.speculate(root, &mut view, ast, &mut narration, layer, c);
            // Asked before the region is taken and before anything is
            // believed: a refusal means what follows it is not this entering.
            let refused = view.refused();
            let mut region = view.provs.take_fork();
            // **Free it where it was allocated.** A fork the commit will not
            // read is memory some *other* thread would otherwise return, and
            // every modern allocator makes that its slow path: on
            // `features/01 -e` at `--jobs 8` it was 192 ms of a 269 ms commit
            // loop. A solution keeps its fork because `record_node` snapshots
            // it and promotes what it cites; everything else is the worker's
            // to drop, and dropping it here is also *parallel*.
            if !keep_forks && !matches!(&entered, Ok(e) if e.solved) {
                if let Ok(e) = entered.as_mut() {
                    e.kb = None;
                    e.firings = Vec::new();
                }
                region = ein_core::Region::default();
            }
            Speculated {
                entered: (!refused).then_some(entered),
                region,
                narration,
            }
        };
        let mut out = Vec::with_capacity(batch.len());
        // `collect_into_vec` on an **indexed** parallel iterator, never an
        // unordered reduce: the vector is in candidate order whatever order the
        // workers finished in, which is the whole of what makes `--jobs N` the
        // same computation as `--jobs 1`.
        match self.pool.as_ref() {
            Some(pool) => pool.install(|| batch.par_iter().map(run).collect_into_vec(&mut out)),
            // A pool that would not build. The fan-out predicate already said
            // this layer *may* be evaluated in any order, so running it here is
            // slower and not different.
            None => out.extend(batch.iter().map(run)),
        }
        out
    }

    #[cfg(feature = "parallel")]
    /// Everything a worker does: branch root, saturate, narrate, probe.
    ///
    /// Pure with respect to the run — no counter, no clause, no dumper hook,
    /// no root write — which is what lets a fanned-out layer call it on many
    /// threads at once and what makes [`Run::commit_entering`] the only place
    /// order can be lost.
    ///
    /// The `spec-audit` arm is deliberately *not* here: it compares this
    /// entering against `R0`, root as the layer opened, and on a fanned-out
    /// layer every fork already **is** `R0`.
    fn speculate(
        &self,
        root: &Kb,
        terms: &mut Terms,
        ast: &Ast,
        events: &mut Events,
        layer: u32,
        c: &[FactId],
    ) -> Result<Entered, SolveError> {
        let result = try_commitment_set(
            root,
            terms,
            ast,
            events,
            &self.memo,
            c,
            None,
            self.root_snapshot.as_deref(),
        )?;
        self.finish_entering(terms, ast, events, layer, c, result)
    }

    /// Narrate the entering, and ask a survivor whether it is complete.
    ///
    /// The order matters and is ein.py's: the `enter` event lands **between**
    /// the fork's own saturation events and the `hyp` events `complete` emits.
    /// A worker records all three into its own buffer, so replaying that
    /// buffer at the commit reproduces the sequential stream exactly — which
    /// is why the event is emitted here rather than by the caller.
    fn finish_entering(
        &self,
        terms: &mut Terms,
        ast: &Ast,
        events: &mut Events,
        layer: u32,
        c: &[FactId],
        mut result: CommitmentSetResult,
    ) -> Result<Entered, SolveError> {
        if events.on() {
            let commitment: Vec<String> =
                c.iter().map(|&f| crate::events::sexpr(terms, f)).collect();
            let mut core: Vec<String> = result
                .unsat_core
                .iter()
                .map(|&f| crate::events::sexpr(terms, f))
                .collect();
            core.sort();
            let (kind, n) = (result.kind.as_str(), result.firings.len());
            events.emit("enter", |l| {
                l.num("layer", layer as i64);
                l.owned_strs("commitment", commitment);
                l.str("kind", kind);
                l.num("n_firings", n as i64);
                l.owned_strs("core", core);
            });
        }
        if result.kind != Kind::Alive {
            return Ok(Entered {
                kind: result.kind,
                unsat_core: result.unsat_core,
                firings: result.firings,
                kb: Some(result.kb),
                solved: false,
                owes: result.owes,
            });
        }
        // F-ENG-12 — consistency is already established on an alive branch
        // (`try_commitment_set` returns `alive` only after its post-saturation
        // detect came back empty, and `result.kb` is that unmutated fork), so
        // ask completeness directly: `is_solution_node` would re-run a full
        // `detect()` on a KB already proved consistent, which is both wasted
        // work and a counter difference.
        let solved = {
            let mut s = Session {
                kb: &mut result.kb,
                terms,
                ast,
                events,
                memo: self.memo.clone(),
            };
            crate::hypgen::complete(&mut s)?
        };
        Ok(Entered {
            kind: result.kind,
            unsat_core: result.unsat_core,
            firings: result.firings,
            kb: Some(result.kb),
            solved,
            owes: result.owes,
        })
    }

    /// Turn one entering into the run's state — counters, learned clause,
    /// `(not h)` writeback, dumper hooks, solution node, early stop.
    ///
    /// **Called in candidate order, always**, whether the entering was
    /// computed here or on a worker. Returns `true` when `stop_after` cut the
    /// search at this candidate.
    #[allow(clippy::too_many_arguments)]
    fn commit_entering(
        &mut self,
        root: &mut Kb,
        terms: &mut Terms,
        ast: &Ast,
        events: &mut Events,
        dumper: &mut dyn Dumper,
        layer: u32,
        c: &CanonicalSetId,
        entered: Entered,
        a_layer: &mut Vec<CanonicalSetId>,
    ) -> Result<bool, SolveError> {
        let mut entered = entered;
        if entered.kind != Kind::Alive {
            // Closed, not discarded: `handle_dead` writes root's no-good and
            // `(not h)` — which belong to the arena proper — and then hands the
            // dead fork to a dumper that renders its justifications.
            terms.provs.close_fork();
            self.handle_dead(root, terms, events, dumper, c, layer, &entered);
            terms.provs.discard_fork();
            return Ok(false);
        }

        self.stats.base.enterings_alive += 1;

        fn info(entered: &Entered) -> EnteringInfo<'_> {
            EnteringInfo {
                kind: entered.kind,
                firings: &entered.firings,
                unsat_core: &entered.unsat_core,
                kb: entered.kb.as_ref(),
                facts_merged: 0,
                nogood_emitted: false,
                nogood_subsumed: false,
            }
        }

        // S1.5b.27 — the saturation-commutativity sanity check. Off by
        // default; `-y` turns it on. Orthogonal to `store_lattice` and to the
        // dumper: the premise applies to every alive commitment. Singletons
        // are skipped inside — no parents.
        if self.cfg.lattice_sanity_check
            && c.len() >= 2
            && let Some(err) =
                crate::sanity::check_commutativity(root, terms, ast, events, &self.memo, c)?
        {
            return Err(SolveError::Sanity(Box::new(err)));
        }

        if entered.solved {
            // Before `record_node`, which takes the firings: ein.py calls the
            // hook after, but nothing between the two lines is observable to
            // it — `_record_node` writes no event and only seals a layer the
            // fact list spans anyway.
            dumper.entering(layer, c, terms, "solution", &info(&entered));
            // The one path that keeps a fork, so the one that promotes:
            // `record_node` snapshots this KB, and a snapshot citing a
            // discarded region would be a KB whose derivations had stopped
            // meaning anything.
            terms.provs.close_fork();
            let firings = std::mem::take(&mut entered.firings);
            let mut kb = entered.kb.take().expect("a solution keeps its fork");
            let owes = std::mem::take(&mut entered.owes);
            self.record_node(&mut kb, terms, c.clone(), firings, layer, owes);
            terms.provs.discard_fork();
            if self
                .opts
                .stop_after
                .is_some_and(|n| self.lstate.nodes.len() as u64 >= n)
            {
                self.lstate.truncated = true;
                // The proof reads root's no-good store, so what was learned
                // before the cut has to be in it.
                self.integrate(root, terms, events);
                return Ok(true);
            }
            return Ok(false);
        }

        dumper.entering(layer, c, terms, "alive", &info(&entered));
        terms.provs.close_fork();
        drop(entered);
        terms.provs.discard_fork();
        a_layer.push(c.clone());
        Ok(false)
    }

    // ── Helpers ────────────────────────────────────────────────

    /// May this layer's enterings be evaluated against **one** root, and
    /// therefore on many cores?
    ///
    /// > A layer is fanned out **iff it cannot write a fact to root.**
    ///
    /// That is [S1a.7.2](../../../../docs/history/m1a_rust/README.md#s1a72--level-1-parallel-enterings)
    /// § The decision, and it is the whole reason this port needs no
    /// speculate-and-validate. A worker forks root when the layer opens, so
    /// the question a parallel layer has to answer is not "can the repair be
    /// made cheap" but "is there anything to repair".
    ///
    /// **Why layer 1 is the only layer that can write.** The search is a
    /// cardinality BFS — layer *L* enters commitment *sets of size L* — so a
    /// dead commitment `{h_1 … h_L}` licenses `¬(h_1 ∧ … ∧ h_L)`, a clause of
    /// width *L*. A clause is not a fact, and only at *L = 1* does it collapse
    /// to something root can hold: the singleton `(not h)` writeback
    /// ([`write_negation`]). Everything else a dead entering produces is a
    /// no-good clause, and that lands in a store no fork reads while it
    /// saturates — [`crate::apriori::filter_candidate`] is the only reader and
    /// it runs at the layer's open.
    ///
    /// **And it is measured, not only argued**: **248 of 248** writebacks
    /// corpus-wide are in layer 1, over 8 158 205 enterings spanning five
    /// layers, and layer 1 is **0.016 %** of those enterings
    /// ([scaling.md §3a](../../../../docs/history/m1a_rust/measurements/scaling.md#3a-where-the-writebacks-are-inside-layer-1--and-the-split-that-is-not-there)).
    /// The other direction of the same measurement is what makes this a
    /// predicate rather than `layer > 1`: with `enable_singleton_writeback`
    /// off nothing writes back at any depth, so layer 1 is fanned out too —
    /// and that is the regime an exhaustive `zebra2` grows from 101 enterings
    /// to 3 336+ in, which is the one that most wants the cores.
    ///
    /// The caller **asserts** the predicate rather than trusting it: `phase2`
    /// compares root's fact count across a fanned-out layer, because a
    /// mechanism that started writing to root mid-layer above the first would
    /// change nothing visible until a fork happened to read it. Should that
    /// day come, design/08 §2's validator is a design that was measured and
    /// costed — deleted from the build, not from the record.
    fn fan_out_this_layer(&self, layer: u32) -> bool {
        layer > 1 || !self.cfg.enable_singleton_writeback
    }

    /// Keep `sat`'s fixpoint for the next entering to resume from.
    fn snapshot_root(&mut self, sat: &Saturator, kb: &Kb) {
        if resume_forks() {
            self.root_snapshot = Some(Arc::new(sat.snapshot(kb)));
        }
    }

    fn compute_alive(
        &mut self,
        kb: &mut Kb,
        terms: &mut Terms,
        ast: &Ast,
        events: &mut Events,
    ) -> Result<FxHashSet<FactId>, SolveError> {
        // ein.py's `events.ON` is a module flag, so **every** hypgen run
        // narrates — including this one, and including `complete`'s inside the
        // layer loop. Silencing them here would leave the two implementations
        // agreeing on what they printed and disagreeing on what they did; the
        // event stream's `n` is what makes that visible.
        let mut s = Session {
            kb,
            terms,
            ast,
            events,
            memo: self.memo.clone(),
        };
        Ok(crate::hypgen::open_hypotheses(&mut s)?)
    }

    /// While `alive` is a singleton `{h}`, promote `h` to a root fact,
    /// re-saturate and recompute. Returns `(alive, terminal)`.
    ///
    /// **Soundness**: `alive = {h}` means every other slot-mate has been
    /// refuted — the singleton-death writeback wrote `(not h_other)` at root,
    /// or hypgen filtered it — so with the puzzle's own exclusivity constraint
    /// `h` must hold. The post-promotion detect catches anything that
    /// surfaces.
    ///
    /// The promotion's provenance is `<forced-positive>` with **empty**
    /// premises: a reserved engine string whose contract is that provenance
    /// walks ground out on it, so a derivation grounds out at the promotion
    /// rather than reading it as a speculative hypothesis.
    fn promote_forced_positives(
        &mut self,
        root: &mut Kb,
        terms: &mut Terms,
        ast: &Ast,
        events: &mut Events,
        mut alive: FxHashSet<FactId>,
    ) -> Result<(FxHashSet<FactId>, bool), SolveError> {
        while alive.len() == 1 {
            // determinism-ok: `alive.len() == 1`, so there is one element to take.
            let h = *alive.iter().next().expect("len 1");
            let (rel, args) = terms.facts.get(h);
            let args = args.to_vec();
            let rule = terms.kernel.forced_positive;
            let prov = terms.provs.push(Prov::from_rule(rule, Box::new([]), None));
            root.add_and_index_fact(terms, rel, &args, Some(prov))
                .expect("room for a promotion");
            self.stats.base.facts_merged += 1;
            self.stats.base.forced_positives += 1;
            if events.on() {
                let text = crate::events::sexpr(terms, h);
                events.emit("writeback", |l| {
                    l.str("fact", &text);
                    l.str("reason", "forced-positive");
                });
            }
            {
                let mut s = Session {
                    kb: root,
                    terms,
                    ast,
                    events,
                    memo: self.memo.clone(),
                };
                let mut sat = Saturator::new(&mut s)?;
                let keep = self.opts.store_lattice;
                let root_firings = &mut self.root_firings;
                sat.saturate(&mut s, None, &mut |f| {
                    if keep {
                        root_firings.push(f.clone());
                    }
                })?;
                self.snapshot_root(&sat, s.kb);
            }
            self.stats.base.saturate_count += 1;
            if crate::contradiction::has_contradiction(root, terms) {
                return Ok((alive, true));
            }
            alive = self.compute_alive(root, terms, ast, events)?;
        }
        Ok((alive, false))
    }

    /// Record a solution node, deduped by [`state_key`] — exact canonical
    /// equality, so no hash collision can collapse two distinct models.
    ///
    /// When the same model state is reached by two commitment paths — the two
    /// orientations of a symmetric pair, say — the **lex-smallest** commitment
    /// wins, so the recorded representative is deterministic regardless of
    /// traversal order.
    fn record_node(
        &mut self,
        node_kb: &mut Kb,
        terms: &mut Terms,
        commitment: CanonicalSetId,
        firings: Vec<Firing>,
        layer: u32,
        owes: Owes,
    ) {
        // **The key first, and the promotion only if the node is kept.**
        //
        // `state_key` reads the fact list and `Kb::promote_provenance` rewrites
        // justification tables, so the key is the same whichever runs first —
        // and running the key first means a node the dedup throws away costs a
        // sort rather than a promotion *and* a snapshot. This used to be the
        // other way round, on the reasoning that promoting the handful of
        // records a solution cites was cheaper than threading the fork
        // region's lifetime through the decision. The region now travels with
        // the entering (T1a.7.2.1), so there is nothing to thread — and the
        // measurement says the ordering was worth 10 ms of `branching/06 -e`'s
        // 64 ms at `--jobs 8`, because that file calls this **1 221 times to
        // keep 22 nodes**.
        let key = state_key(node_kb);
        let at = match self.lstate.node_at.get(&key).copied() {
            None => None,
            Some(at) => {
                // `tuple(sorted(commitment)) < tuple(sorted(cur.commitment))`
                // — sorted and compared by **content**, not by `FactId`.
                // Interning order is an artefact of what the loader saw
                // first, and using it here picks a different representative
                // for a model two commitment paths both reach.
                let mut mine = commitment.clone();
                mine.sort_by(|&a, &b| terms.cmp_fact_semantic(a, b));
                let mut theirs = self.lstate.nodes[at].1.commitment.clone();
                theirs.sort_by(|&a, &b| terms.cmp_fact_semantic(a, b));
                if crate::apriori::cmp_set(terms, &mine, &theirs) != std::cmp::Ordering::Less {
                    // The stored representative wins; this path's derivation
                    // is not kept, so neither are its records.
                    return;
                }
                Some(at)
            }
        };
        node_kb.promote_provenance(terms);
        let r = SolutionRecord {
            commitment,
            // A snapshot, so it survives later root mutation.
            kb: node_kb.snapshot(),
            firings,
            layer,
            owes,
        };
        match at {
            None => {
                self.lstate
                    .node_at
                    .insert(key.clone(), self.lstate.nodes.len());
                self.lstate.nodes.push((key, r));
            }
            // In place: ein.py assigns into the dict, which keeps the original
            // insertion position, and that position is an `Ambiguity`'s branch
            // order.
            Some(at) => self.lstate.nodes[at].1 = r,
        }
    }

    fn root_dead(&mut self, root: &Kb, terms: &Terms) {
        let core = source_frontier_core(root, terms);
        self.lstate.dead.push(DeadCommitment {
            commitment: Vec::new(),
            unsat_core: core,
            learned_clause: Vec::new(),
            layer: 0,
            kind: Kind::DeadPost,
            state_key: state_key(root),
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_dead(
        &mut self,
        root: &mut Kb,
        terms: &mut Terms,
        events: &mut Events,
        dumper: &mut dyn Dumper,
        c: &[FactId],
        layer: u32,
        entered: &Entered,
    ) {
        if entered.kind == Kind::DeadPre {
            self.stats.base.enterings_dead_pre += 1;
        } else {
            self.stats.base.enterings_dead_post += 1;
        }
        // D-R5-1 — `landed` is read unconditionally below, so it must exist
        // even when no clause was attempted. `false` is the honest value:
        // nothing landed because nothing was emitted.
        let mut landed = false;
        let deferring = self.opts.integrate_every.is_some();
        if self.cfg.enable_path_nogoods {
            landed = if deferring {
                // The store is *read* here and written at the barrier, so
                // `landed` answers against the batch's own KB. That is the
                // mode, not an approximation of it: a clause learned by an
                // earlier candidate of the same batch is deliberately not
                // visible yet, which is why the counters move and the answer
                // does not.
                let fresh = !crate::nogoods::subsumed(root, c, 1);
                self.deferred.push(Deferred::Nogood(c.to_vec()));
                fresh
            } else {
                crate::nogoods::emit_nogood(root, terms, events, c, 1)
            };
            if landed {
                self.stats.base.nogoods_emitted += 1;
            } else {
                self.stats.base.nogoods_subsumed += 1;
            }
        }
        if c.len() == 1 && self.cfg.enable_singleton_writeback {
            if deferring {
                self.deferred.push(Deferred::Writeback(c[0]));
            } else {
                self.census.writebacks += 1;
                write_negation(root, terms, events, c[0]);
            }
        }
        self.lstate.dead.push(DeadCommitment {
            commitment: c.to_vec(),
            unsat_core: entered.unsat_core.clone(),
            learned_clause: c.to_vec(),
            layer,
            kind: entered.kind,
            // Empty exactly when the worker dropped the fork, which it does
            // only when nothing reads one — `store_lattice` is the reader, and
            // it keeps them.
            state_key: entered.kb.as_ref().map(state_key).unwrap_or_default(),
        });
        dumper.entering(
            layer,
            c,
            terms,
            entered.kind.as_str(),
            &EnteringInfo {
                kind: entered.kind,
                firings: &entered.firings,
                unsat_core: &entered.unsat_core,
                kb: entered.kb.as_ref(),
                facts_merged: 0,
                nogood_emitted: landed,
                // Not `!landed`: with no-goods off nothing was *attempted*, so
                // nothing was subsumed either.
                nogood_subsumed: self.cfg.enable_path_nogoods && !landed,
            },
        );
    }

    /// The integration barrier: apply every held-back root write, in the
    /// order the enterings produced them.
    ///
    /// When `integrate_every` is `None` the buffer is always empty, so this is
    /// a `mem::take` of an empty `Vec` and no iterations — the sequential path
    /// stays the reference implementation rather than becoming a special case
    /// of this one.
    fn integrate(&mut self, root: &mut Kb, terms: &mut Terms, events: &mut Events) {
        for d in std::mem::take(&mut self.deferred) {
            match d {
                Deferred::Nogood(c) => {
                    crate::nogoods::emit_nogood(root, terms, events, &c, 1);
                }
                Deferred::Writeback(h) => {
                    self.census.writebacks += 1;
                    write_negation(root, terms, events, h);
                }
            }
        }
    }

    /// Rebuild root as a single layer once the layer's writes have stacked up
    /// ([`SolveOptions::coalesce_root_at`], T1a.7.2.0).
    ///
    /// This is a *representation* change and nothing else: every fact root
    /// holds it held a line ago, in the same order, with the same primary
    /// justification. What it buys is the read path — a fork inherits the
    /// stack and every `contains` / `facts_of` / `facts_with` walks it — and
    /// the reason it belongs at the barrier rather than at each write is that
    /// a layer's writebacks arrive one per candidate while its forks arrive
    /// thousands per layer.
    ///
    /// Not called at the `stop_after` cut: there is no next layer to read the
    /// flattened root, so the copy would be pure cost.
    fn coalesce_root(&mut self, root: &mut Kb) {
        if self
            .opts
            .coalesce_root_at
            .is_some_and(|min| root.depth() >= min)
        {
            root.flatten();
        }
    }

    fn check_budget(&mut self, dumper: &mut dyn Dumper) -> Result<(), SolveError> {
        let reason = if self
            .opts
            .max_enterings
            .is_some_and(|m| self.stats.base.enterings_total >= m)
        {
            Some(format!(
                "max-enterings ({}) reached",
                self.opts.max_enterings.expect("checked")
            ))
        } else if self
            .opts
            .max_time
            .is_some_and(|m| self.t_start.elapsed().as_secs_f64() > m)
        {
            // `str(float)` is `repr(float)` in Python 3, and Rust's `{:?}`
            // is the same shortest round-trip — the agreement `SolverConfig`'s
            // renderer already leans on.
            Some(format!(
                "max-time ({:?}s) exceeded",
                self.opts.max_time.expect("checked")
            ))
        } else {
            None
        };
        let Some(reason) = reason else {
            return Ok(());
        };
        // The abort raises before the verdict is built, so not-exhausted is
        // recorded here; otherwise the partial stats keep the default `true`
        // and an aborted run would look fully explored.
        self.stats.exhausted = false;
        dumper.close();
        Err(SolveError::Budget {
            reason,
            stats: Box::new(self.stats),
        })
    }

    fn finalise(&mut self) -> Answer {
        self.stats.solution_nodes = self.lstate.nodes.len() as u64;
        self.stats.exhausted = !self.lstate.truncated;
        if self.lstate.nodes.is_empty() {
            let cores: Vec<Vec<FactId>> = self
                .lstate
                .dead
                .iter()
                .map(|d| d.unsat_core.clone())
                .collect();
            return Answer::Verdict(Verdict::Contradiction {
                unsat_core: union_dead_cores(&cores),
            });
        }
        // The records are **not** consumed: ein.py's verdict and its
        // `LatticeProof` reference the same `SolutionRecord` objects, and the
        // proof is packaged after the verdict is read. A second `snapshot` of
        // an already-archival KB is what stands in for that sharing here — it
        // is cheap (an `Arc` base plus one delta layer) and it is the only
        // place the two would otherwise fight over ownership.
        //
        // **M1d S1d.2.6 — `complete` means discharged, in the read-out.** The
        // partition is here and nowhere else: the *search* still records every
        // node it found complete by the generator's test, so no counter, no
        // cost and no traversal moves. What changes is which of those nodes
        // the answer is allowed to call a model.
        let scoped = self.declares_obligations;
        let mut branches: Vec<Solution> = Vec::with_capacity(self.lstate.nodes.len());
        let mut open_states: Vec<Solution> = Vec::new();
        let mut open_owes: Vec<Owes> = Vec::new();
        for (_, n) in self.lstate.nodes.iter_mut() {
            let s = Solution {
                kb: n.kb.snapshot(),
                trace: n.firings.clone(),
            };
            if scoped && !n.owes.is_empty() {
                open_owes.push(n.owes.clone());
                open_states.push(s);
            } else {
                branches.push(s);
            }
        }
        // A discharged model outranks an open state: where the search found
        // both, the answer is the models and the open ones are simply not
        // among them. No corpus entry is in that regime today — the sixteen
        // owing entries owe at *root* and their models owe nothing, or every
        // recorded node owes — so this arm is defined rather than measured,
        // and `openness_census.md` §4 is where that is said with the number.
        if branches.is_empty() && !open_states.is_empty() {
            return Answer::Verdict(Verdict::Open {
                states: open_states,
                owes: open_owes,
            });
        }
        if branches.len() == 1 {
            Answer::Verdict(Verdict::Solution(branches.pop().expect("len 1")))
        } else {
            Answer::Verdict(Verdict::Ambiguity(branches))
        }
    }

    fn proof(&mut self, root: &Kb) -> LatticeProof {
        let solutions = std::mem::take(&mut self.lstate.nodes);
        let mut learned: Vec<Box<[FactId]>> = root
            .nogoods()
            .read()
            .expect("the no-good store")
            .iter()
            .map(|c| c.to_vec().into_boxed_slice())
            .collect();
        // determinism-ok: the store is a set on both sides; sorted here so the
        // proof's clause list is a function of the run.
        learned.sort();
        LatticeProof {
            solutions: solutions.into_iter().map(|(_, r)| r).collect(),
            root_firings: std::mem::take(&mut self.root_firings),
            dead_commitments: std::mem::take(&mut self.lstate.dead),
            alive_at_end: std::mem::take(&mut self.lstate.alive_at_end),
            learned_nogoods: learned,
            stats: LatticeStats {
                base: self.stats.base,
                solutions_found: self.stats.solution_nodes,
                state_key_merges: self.lstate.state_key_merges,
                elapsed_seconds: self.t_start.elapsed().as_secs_f64(),
            },
        }
    }
}

enum Phase1 {
    Done,
    Continue {
        alive: FxHashSet<FactId>,
        a_prev: Vec<CanonicalSetId>,
    },
}

/// Cadence for the live root-saturation progress line. Fixed, and deliberately
/// not tied to `--progress-every` (which paces enterings), so a verbose run
/// cannot flood with a line per firing.
const ROOT_SAT_PROGRESS_EVERY: usize = 50;

/// A reserved engine string whose empty premises make provenance walks ground
/// out on it.
pub const FORCED_POSITIVE: &str = ein_core::terms::FORCED_POSITIVE;
/// The singleton-death writeback's rule name — same contract.
pub const MONOTONIC_UNCONDITIONAL: &str = ein_core::terms::MONOTONIC_UNCONDITIONAL;

/// The unsat core for a contradiction already present in `kb`.
///
/// The **smallest** explanation of one witness, not the union over every
/// witness: when one cause propagates it fans out, so unioning over-states the
/// conflict. On `zebra2-bad` a single injected fact produces 126 witnesses
/// whose frontiers union to 39 facts, while the smallest single witness is
/// exactly the culprit.
fn source_frontier_core(kb: &Kb, terms: &Terms) -> Vec<FactId> {
    let witnesses: Vec<FactId> = crate::contradiction::detect(kb, terms)
        .iter()
        .map(|c| c.witness())
        .collect();
    if witnesses.is_empty() {
        return Vec::new();
    }
    crate::explain::smallest_contradiction_frontier(kb, terms, Some(&witnesses))
}

/// The singleton-death writeback: `(not h)` at root, so the generator excludes
/// `h` and the next `alive` shrinks.
///
/// A flat root write with no ancestor-chain coupling, and **no symmetric
/// mirror** (S1.7.24): the counterpart dies on its own branch — re-derivation
/// through the user's own `(rule symmetric)` hits the same ⊥ — and the two
/// branches collapse at the generic `state_key` dedup. The kernel keys on
/// `is_symmetric` nowhere. Idempotent.
fn write_negation(root: &mut Kb, terms: &mut Terms, events: &mut Events, h: FactId) {
    let not = terms.kernel.not;
    let arg = [Value::fact(h)];
    let already = terms
        .probe_fact(not, &arg)
        .is_some_and(|id| root.contains(id));
    if !already {
        let rule: Symbol = terms.kernel.monotonic_unconditional;
        let prov = terms.provs.push(Prov::from_rule(rule, Box::new([]), None));
        root.add_and_index_fact(terms, not, &arg, Some(prov))
            .expect("room for a writeback");
    }
    if events.on() {
        let text = format!("(not {})", crate::events::sexpr(terms, h));
        events.emit("writeback", |l| {
            l.str("fact", &text);
            l.str("reason", "singleton-dead-clause");
        });
    }
}
