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
//! [design/08](../../../../plans/m1a_rust/design/08_parallelism.md) §2 has to
//! validate against.

use std::time::Instant;

use ein_core::{FactId, Kb, Prov, SolverConfig, Symbol, Terms, Value};
use ein_ir::Ast;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::apriori::{CanonicalSetId, generate_layer, layer_1, order_candidates};
use crate::canon::state_key;
use crate::commitment::{Kind, try_commitment_set};
use crate::compile::CompileError;
use crate::events::Events;
use crate::firing::Firing;
use crate::saturator::{SaturateError, Saturator, Session};
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
    /// see [`Q-M1a.5`](../../../../plans/m1a_rust/open_questions.md).
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

/// The lifecycle hooks a state dumper receives — implemented in
/// [S1a.5.3](../../../../plans/m1a_rust/p1a.5_presentation/s1a.5.3_state_dumps.md)
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
    /// Written from the single exit hook when the verdict carries a proof, so
    /// a `kb_index/` tree and its index land *before* the cumulative summary.
    fn proof_summary(&mut self, proof: &LatticeProof, terms: &Terms) {}
    fn summary(&mut self, verdict: &Answer, stats: &MonotonicStats) {}
    fn close(&mut self) {}
}

/// A dumper that does nothing — the common no-dumper path, without an
/// `Option` at every call site.
pub struct NoDumper;
impl Dumper for NoDumper {}

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

pub struct Solved {
    pub answer: Answer,
    pub proof: Option<LatticeProof>,
    pub stats: MonotonicStats,
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
        stats: MonotonicStats::new(),
        lstate: LoopState {
            nodes: Vec::new(),
            node_at: FxHashMap::default(),
            dead: Vec::new(),
            alive_at_end: Vec::new(),
            state_key_merges: 0,
            truncated: false,
        },
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
            Ok(Solved {
                answer,
                proof,
                stats: run.stats,
            })
        }
        Err(SolveError::Budget { reason, stats }) if opts.on_budget == OnBudget::Verdict => {
            Ok(Solved {
                answer: Answer::Aborted { reason },
                proof: None,
                stats: *stats,
            })
        }
        Err(e) => Err(e),
    }
}

struct Run<'o> {
    shuffle: Option<crate::mt19937::Mt19937>,
    cfg: SolverConfig,
    stats: MonotonicStats,
    lstate: LoopState,
    t_start: Instant,
    opts: &'o SolveOptions,
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
            };
            let mut sat = Saturator::new(&mut s)?;
            // Root saturation is the slow part of a Phase-1 solve, so the
            // firing count streams to the dumper. Both paths run the same
            // loop — ein.py splits them to keep the C-speed `list()` drain on
            // the common path, and the split is a cost decision, not a
            // behavioural one.
            let mut n = 0usize;
            sat.saturate(&mut s, None, &mut |_| {
                n += 1;
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
        if alive.is_empty() {
            // Empty alive and no contradiction ⇒ root is itself a complete,
            // consistent model — the unique solution.
            self.record_node(root, terms, Vec::new(), Vec::new(), 0);
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
        let mut phase_2_done = false;
        for layer in 1..=self.opts.max_set_size {
            if phase_2_done {
                break;
            }
            self.stats.base.layers_explored = layer as u64;
            dumper.layer_start(layer, root, terms, alive.len());

            let candidates = if layer == 1 {
                a_prev.clone()
            } else {
                let store = root.nogoods().clone();
                let guard = store.read().expect("the no-good store");
                generate_layer(terms, &a_prev, &alive, &guard)
            };
            let mut candidates =
                order_candidates(root, terms, &candidates, &self.cfg.lattice_order)
                    .map_err(|e| SolveError::Compile(CompileError(e.to_string())))?;
            // After `order_candidates`, never instead of it — the shuffle is a
            // permutation of the ordered list, and the harness that probes
            // traversal-order dependence needs both to have happened.
            if let Some(rng) = self.shuffle.as_mut() {
                rng.shuffle(&mut candidates);
            }

            let mut a_layer: Vec<CanonicalSetId> = Vec::new();
            for c in &candidates {
                self.check_budget(dumper)?;
                self.stats.base.enterings_total += 1;
                let result = try_commitment_set(root, terms, ast, events, c, None)?;
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
                    self.handle_dead(root, terms, events, dumper, c, layer, &result);
                    continue;
                }

                self.stats.base.enterings_alive += 1;
                // F-ENG-12 — consistency is already established on an alive
                // branch (`try_commitment_set` returns `alive` only after its
                // post-saturation detect came back empty, and `result.kb` is
                // that unmutated fork), so ask completeness directly:
                // `is_solution_node` would re-run a full `detect()` on a KB
                // already proved consistent, which is both wasted work and a
                // counter difference.
                let mut fork = result.kb;
                let solved = {
                    let mut s = Session {
                        kb: &mut fork,
                        terms,
                        ast,
                        events,
                    };
                    crate::hypgen::complete(&mut s)?
                };

                // S1.5b.27 — the saturation-commutativity sanity check. Off by
                // default; `-y` turns it on. Orthogonal to `store_lattice` and
                // to the dumper: the premise applies to every alive
                // commitment. Singletons are skipped inside — no parents.
                if self.cfg.lattice_sanity_check
                    && c.len() >= 2
                    && let Some(err) =
                        crate::sanity::check_commutativity(root, terms, ast, events, c)?
                {
                    return Err(SolveError::Sanity(Box::new(err)));
                }

                if solved {
                    // Before `record_node`, which takes the firings: ein.py
                    // calls the hook after, but nothing between the two lines
                    // is observable to it — `_record_node` writes no event and
                    // only seals a layer the fact list spans anyway.
                    dumper.entering(
                        layer,
                        c,
                        terms,
                        "solution",
                        &EnteringInfo {
                            kind: result.kind,
                            firings: &result.firings,
                            unsat_core: &result.unsat_core,
                            kb: Some(&fork),
                            facts_merged: 0,
                            nogood_emitted: false,
                            nogood_subsumed: false,
                        },
                    );
                    self.record_node(&mut fork, terms, c.clone(), result.firings, layer);
                    if self
                        .opts
                        .stop_after
                        .is_some_and(|n| self.lstate.nodes.len() as u64 >= n)
                    {
                        self.lstate.truncated = true;
                        return Ok(());
                    }
                    continue;
                }
                dumper.entering(
                    layer,
                    c,
                    terms,
                    "alive",
                    &EnteringInfo {
                        kind: result.kind,
                        firings: &result.firings,
                        unsat_core: &result.unsat_core,
                        kb: Some(&fork),
                        facts_merged: 0,
                        nogood_emitted: false,
                        nogood_subsumed: false,
                    },
                );
                a_layer.push(c.clone());
            }

            dumper.layer_end(layer, root, terms, alive.len(), a_layer.len());
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
                // The backbone determines every cell.
                self.record_node(root, terms, Vec::new(), Vec::new(), 0);
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

    // ── Helpers ────────────────────────────────────────────────

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
            let rule = terms
                .intern_text(FORCED_POSITIVE)
                .expect("room for a reserved engine string");
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
                };
                let mut sat = Saturator::new(&mut s)?;
                sat.saturate(&mut s, None, &mut |_| {})?;
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
        terms: &Terms,
        commitment: CanonicalSetId,
        firings: Vec<Firing>,
        layer: u32,
    ) {
        let key = state_key(node_kb);
        let record = |kb: &mut Kb| SolutionRecord {
            commitment: commitment.clone(),
            // A snapshot, so it survives later root mutation.
            kb: kb.snapshot(),
            firings,
            layer,
        };
        match self.lstate.node_at.get(&key).copied() {
            None => {
                self.lstate
                    .node_at
                    .insert(key.clone(), self.lstate.nodes.len());
                let r = record(node_kb);
                self.lstate.nodes.push((key, r));
            }
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
                if crate::apriori::cmp_set(terms, &mine, &theirs) == std::cmp::Ordering::Less {
                    // In place: ein.py assigns into the dict, which keeps the
                    // original insertion position, and that position is an
                    // `Ambiguity`'s branch order.
                    self.lstate.nodes[at].1 = record(node_kb);
                }
            }
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
        result: &crate::commitment::CommitmentSetResult,
    ) {
        if result.kind == Kind::DeadPre {
            self.stats.base.enterings_dead_pre += 1;
        } else {
            self.stats.base.enterings_dead_post += 1;
        }
        // D-R5-1 — `landed` is read unconditionally below, so it must exist
        // even when no clause was attempted. `false` is the honest value:
        // nothing landed because nothing was emitted.
        let mut landed = false;
        if self.cfg.enable_path_nogoods {
            landed = crate::nogoods::emit_nogood(root, terms, events, c, 1);
            if landed {
                self.stats.base.nogoods_emitted += 1;
            } else {
                self.stats.base.nogoods_subsumed += 1;
            }
        }
        if c.len() == 1 && self.cfg.enable_singleton_writeback {
            write_negation(root, terms, events, c[0]);
        }
        self.lstate.dead.push(DeadCommitment {
            commitment: c.to_vec(),
            unsat_core: result.unsat_core.clone(),
            learned_clause: c.to_vec(),
            layer,
            kind: result.kind,
            state_key: state_key(&result.kb),
        });
        dumper.entering(
            layer,
            c,
            terms,
            result.kind.as_str(),
            &EnteringInfo {
                kind: result.kind,
                firings: &result.firings,
                unsat_core: &result.unsat_core,
                kb: Some(&result.kb),
                facts_merged: 0,
                nogood_emitted: landed,
                // Not `!landed`: with no-goods off nothing was *attempted*, so
                // nothing was subsumed either.
                nogood_subsumed: self.cfg.enable_path_nogoods && !landed,
            },
        );
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
        let mut branches: Vec<Solution> = Vec::with_capacity(self.lstate.nodes.len());
        for (_, n) in self.lstate.nodes.iter_mut() {
            branches.push(Solution {
                kb: n.kb.snapshot(),
                trace: n.firings.clone(),
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
pub const FORCED_POSITIVE: &str = "<forced-positive>";
/// The singleton-death writeback's rule name — same contract.
pub const MONOTONIC_UNCONDITIONAL: &str = "<monotonic-unconditional>";

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
        let rule: Symbol = terms
            .intern_text(MONOTONIC_UNCONDITIONAL)
            .expect("room for a reserved engine string");
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
