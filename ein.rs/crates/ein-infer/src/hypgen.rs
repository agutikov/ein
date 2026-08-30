//! Hypothesis generation — the enumerator that proposes what to guess.
//!
//! **Three** modes since M1d S1d.2.5, and the switch is a ladder: rule-driven
//! when the puzzle declares any `(hrule …)` ([`crate::hrule`]), otherwise
//! obligation-driven while the state owes something it may branch on
//! ([`crate::oblgen`]), otherwise blind combinatorial. Then an eight-stage
//! filter pipeline whose **attribution** — which counter a drop lands in — is
//! a T1 observable through [`HypGenStats`] and through the `hyp` / `hypskip`
//! events, and every rung feeds it: what changes between them is which
//! candidates are proposed, never what happens to one afterwards.
//!
//! ### What must not change
//!
//! - **The filter order.** It decides which counter a drop is attributed to,
//!   and the counters are compared. A reordering that looks like an
//!   optimisation is a parity failure
//!   ([S1a.4.1](../../../../docs/history/m1a_rust/README.md#s1a41--hypothesis-generation)).
//! - **The candidate order.** It becomes `layer_1`'s singleton order and
//!   therefore the whole traversal.
//! - **The kernel imposes no type system** (S1.7.23). The enumerator proposes
//!   type-blind and the puzzle's own rules do the pruning; Rust code that
//!   reaches for `is-a` here is reintroducing what that stage removed. The
//!   *signature* atoms are excluded, which is a different thing: a name a
//!   puzzle declared as a type role, derived without naming `is-a`.
//!
//! ### What is free
//!
//! The cost. ein.py builds a `Fact`, a `Provenance` slot and two tuple hashes
//! for each of ~18 k raw candidates per call on `zebra2`; here the *row key
//! is the identity*, so a candidate costs one intern — a hash lookup that
//! returns the same `FactId` every later call — and the two filters that
//! reject almost all of them are single bit tests on it
//! ([design/07](../../../../docs/history/m1a_rust/design/07_search_layer.md) §2).
//!
//! design/07 calls this "intern-on-demand: probe, and only materialise on
//! survival". The probe and the intern turned out to be *the same hash
//! lookup* — `FactStore::intern` is `probe` plus a push on a miss — and the
//! push is bounded by the distinct candidate space, not by the number of
//! calls, so the split would buy a branch and cost the caller a second
//! lookup. What the design was actually protecting against — a per-candidate
//! allocation on the rejected path — is absent either way, and the `Fact`
//! ein.py builds has no counterpart here at all.

use std::ops::ControlFlow;

use ein_core::entities::NameCategory;
use ein_core::{FactId, Kb, Symbol, Terms, Value};
use rustc_hash::FxHashSet;

use crate::compile::CompileError;
use crate::hrule::Hrules;
use crate::lookahead::Lookahead;
use crate::match_::Matcher;
use crate::saturator::Session;

/// The `(query …)` keyword restricting which relations the blind enumerator
/// builds candidates for (S1.7.25). Reserved engine string.
pub const HYPOTHESIS_RELATIONS: &str = "hypothesis-relations";

/// Its exclusion dual (S1.9.E3): relations never to guess on, while saturation
/// rules on them still fire. Reserved engine string.
pub const NO_HYPOTHESIS: &str = "no-hypothesis";

/// A candidate-level filter — the name it bumps in `stats.filtered`, and the
/// `verdict` a `hyp` event carries.
///
/// The discriminants are the **sorted** key order, because both readers walk
/// them sorted: `HypGenStats.as_report_lines` and the stats diff.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Drop {
    FactAlreadyExists = 0,
    LookaheadKilled = 1,
    NegatedFact = 2,
    SeenInCall = 3,
}

/// A pre-candidate skip — structural, at the relation/slot level, before any
/// candidate fact exists. Sorted key order, as [`Drop`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Skip {
    ClosedRelation = 0,
    NoHypothesisRelation = 1,
    RelationNotWhitelisted = 2,
    SelfEdge = 3,
}

impl Drop {
    pub fn as_str(self) -> &'static str {
        match self {
            Drop::FactAlreadyExists => "fact_already_exists",
            Drop::LookaheadKilled => "lookahead_killed",
            Drop::NegatedFact => "negated_fact",
            Drop::SeenInCall => "seen_in_call",
        }
    }
}

impl Skip {
    pub fn as_str(self) -> &'static str {
        match self {
            Skip::ClosedRelation => "closed_relation",
            Skip::NoHypothesisRelation => "no_hypothesis_relation",
            Skip::RelationNotWhitelisted => "relation_not_whitelisted",
            Skip::SelfEdge => "self_edge",
        }
    }
}

/// Per-filter counters for one [`generate`] call.
///
/// ein.py's two `defaultdict(int)`s are dense arrays here, and a **zero count
/// means the key was never bumped** — which is the same thing, since a bump is
/// `+= 1`. That equivalence is what lets [`HypGenStats::report_lines`]
/// reproduce `as_report_lines`, whose loops walk only the keys that exist.
///
/// Invariant: `raw == emitted + sum(filtered)`, asserted by the renderer's
/// caller on both sides.
#[derive(Clone, Default, Debug)]
pub struct HypGenStats {
    pub raw: u64,
    pub emitted: u64,
    pub filtered: [u64; 4],
    pub pre_candidate: [u64; 4],
    /// Which rung of the ladder this call took, and what it found owed — M1d
    /// S1d.2.5. All zeros and [`crate::oblgen::Mode::Blind`] for a program
    /// that declares no obligation rule, which is the shape every counter
    /// baseline was taken under.
    pub rung: crate::oblgen::RungReport,
}

impl HypGenStats {
    pub fn new() -> HypGenStats {
        HypGenStats::default()
    }

    fn bump(&mut self, d: Drop) {
        self.filtered[d as usize] += 1;
    }

    pub(crate) fn skip(&mut self, s: Skip) {
        self.pre_candidate[s as usize] += 1;
    }

    /// `raw == emitted + sum(filtered.values())` — the invariant the stage's
    /// acceptance names, checked rather than asserted in prose.
    pub fn balances(&self) -> bool {
        self.raw == self.emitted + self.filtered.iter().sum::<u64>()
    }

    /// `HypGenStats.as_report_lines` — what `--hyp-stats` prints, field widths
    /// included. Keys sorted, and absent keys omitted.
    pub fn report_lines(&self) -> Vec<String> {
        let mut out = vec![
            format!("  raw                {}", self.raw),
            format!("  emitted            {}", self.emitted),
        ];
        for (i, &n) in self.filtered.iter().enumerate() {
            if n > 0 {
                let k = DROPS[i].as_str();
                out.push(format!("  filtered.{k:18} {n}"));
            }
        }
        for (i, &n) in self.pre_candidate.iter().enumerate() {
            if n > 0 {
                let k = SKIPS[i].as_str();
                out.push(format!("  pre.{k:23} {n}"));
            }
        }
        // The ladder's rung, and **only** the rung this stage added: a program
        // that reaches the blind enumerator or declares an `(hrule …)` prints
        // what it printed before S1d.2.5, to the byte. A draft that printed it
        // for the hrule rung too moved **206** of the corpus's 8 081 shape
        // digests; restricting it to this stage's own rung moves 42, and those
        // 42 are the programs the ladder is actually about.
        use crate::oblgen::Mode;
        let r = &self.rung;
        if matches!(r.mode, Mode::Obligations | Mode::Stuck | Mode::Declined) {
            out.push(format!("  rung               {}", r.mode.as_str()));
            if r.owed > 0 || r.branches > 0 || r.declined > 0 {
                out.push(format!("  rung.owed          {}", r.owed));
                out.push(format!("  rung.branches      {}", r.branches));
                out.push(format!("  rung.declined      {}", r.declined));
            }
            out.push(format!("  rung.uncovered     {}", r.uncovered));
        }
        out
    }
}

const DROPS: [Drop; 4] = [
    Drop::FactAlreadyExists,
    Drop::LookaheadKilled,
    Drop::NegatedFact,
    Drop::SeenInCall,
];
const SKIPS: [Skip; 4] = [
    Skip::ClosedRelation,
    Skip::NoHypothesisRelation,
    Skip::RelationNotWhitelisted,
    Skip::SelfEdge,
];

/// `(__closed__ R)` — the kernel's closed-relation marker (S1.7.25). A closed
/// relation contributes zero candidates; the generator does not care whether
/// the fact was authored, auto-emitted, or derived by `std.closure`.
pub const CLOSED: &str = ein_core::terms::CLOSED;

// ── The generator ──────────────────────────────────────────────────

/// Yield candidate hypothesis facts in priority order, calling `f` per
/// survivor.
///
/// A callback rather than an iterator because the enumeration is **not pure**:
/// with `enable_lookahead_kill_cache` on, a lookahead kill writes `(not h)`
/// into the KB and later candidates in the *same call* observe it through the
/// `negated_fact` filter. A `ControlFlow::Break` stops the walk, which is what
/// keeps `complete`'s short-circuit (S1.9.E16) — and the feed-forward is why
/// [design/08](../../../../docs/history/m1a_rust/design/08_parallelism.md) §7 refuses
/// to parallelise this pipeline.
pub fn generate(
    s: &mut Session<'_>,
    stats: &mut HypGenStats,
    f: &mut dyn FnMut(FactId) -> ControlFlow<()>,
) -> Result<(), CompileError> {
    generate_rungs(s, stats, f, Rungs::Ladder, false)
}

/// [`generate`], but **one owed instance's alternatives** instead of the union.
///
/// The ladder is walked exactly as [`generate`] walks it and every candidate
/// goes through the same filters; what differs is that the obligations rung
/// stops after the instance [`crate::oblgen::Choice`] picks. M1d
/// [T1d.10.6.3](../../../../docs/history/m1d_satisfiability/README.md#s1d106--the-traversal)
/// — a depth-first traversal branches on one instance because one instance's
/// set is jointly exhaustive *by the obligation's meaning*, where the union is
/// merely a superset of each.
///
/// **Only the obligations rung honours it.** An hrule's candidates and the
/// blind enumerator's are not an owed instance's alternatives and are not
/// jointly exhaustive, so those rungs return everything they would have; the
/// caller checks [`HypGenStats::rung`]'s mode before treating the answer as a
/// branch.
pub fn generate_one_branch(
    s: &mut Session<'_>,
    stats: &mut HypGenStats,
) -> Result<Vec<FactId>, CompileError> {
    let mut out = Vec::new();
    generate_rungs(
        s,
        stats,
        &mut |fact| {
            out.push(fact);
            ControlFlow::Continue(())
        },
        Rungs::Ladder,
        true,
    )?;
    Ok(out)
}

/// [`generate`] with the ladder's two upper rungs skipped — the blind
/// combinatorial enumerator alone, whatever the program declares.
///
/// **Not a search entry point.** The search always walks the ladder; this
/// answers a question *about* a state that the ladder cannot be asked, because
/// the rung that is active there stops as soon as its own candidates run out:
/// *how many facts would the blind enumerator still propose at a node the rung
/// called complete?* That number is the state's leftover-open count — M1d
/// [S1d.3.1](../../../../docs/history/m1d_satisfiability/README.md#s1d31--what-the-models-differ-in)
/// — and it is what separates "one model" from "2ⁿ models" when the reading
/// is open-world.
///
/// The caller **must** hand it a KB it is willing to see written to: with
/// `enable_lookahead_kill_cache` on, the walk writes `(not h)` per lookahead
/// kill, exactly as the search's own generation does. On a fork that is then
/// discarded that write is invisible; on the node itself it would move the
/// `state_key` and therefore the model dedup, which is why P1d.2 declined the
/// probe rather than taking it against a live node.
pub fn generate_blind(
    s: &mut Session<'_>,
    stats: &mut HypGenStats,
    f: &mut dyn FnMut(FactId) -> ControlFlow<()>,
) -> Result<(), CompileError> {
    generate_rungs(s, stats, f, Rungs::BlindOnly, false)
}

/// Which rungs of the generation ladder a call is allowed to use.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Rungs {
    /// Every rung, in order — what the search runs.
    Ladder,
    /// The bottom rung only — [`generate_blind`]'s probe.
    BlindOnly,
}

/// The ladder, with the rungs the caller is allowed to use.
///
/// One body for both entry points, because the setup — the lookahead, the
/// `(query …)` relation lists, the per-call `Ctx` — is what every rung shares,
/// and a second copy of it is a second place for the filter order to drift.
fn generate_rungs(
    s: &mut Session<'_>,
    stats: &mut HypGenStats,
    f: &mut dyn FnMut(FactId) -> ControlFlow<()>,
    rungs: Rungs,
    one_branch: bool,
) -> Result<(), CompileError> {
    ein_core::counters::bump(|c| c.hypgen_call += 1);
    let cfg = s.kb.program().config.clone().unwrap_or_default();
    // Built once per call (it compiles the rule plans, emitting `compile`
    // events as ein.py's `Engine(kb).compile_all()` does) and reused per
    // candidate.
    let lookahead = if cfg.enable_pre_branch_lookahead {
        Some(Lookahead::new(s)?)
    } else {
        None
    };
    let allowed = query_relations(s, HYPOTHESIS_RELATIONS);
    let excluded = query_relations(s, NO_HYPOTHESIS).unwrap_or_default();
    let mut ctx = Ctx {
        lookahead,
        kill_cache: cfg.enable_lookahead_kill_cache,
        matcher: Matcher::new(),
        seen: FxHashSet::default(),
    };

    if rungs == Rungs::Ladder && !s.kb.program().hrules.is_empty() {
        // Rung 1 — the user's own generator, an override. `(hrule …)` presence
        // *is* the switch, exactly as design/07 wrote it: a puzzle that says
        // what to guess is never second-guessed by what it owes.
        stats.rung.mode = crate::oblgen::Mode::Hrules;
        let hrules = Hrules::new(s)?;
        let _ = hrules.candidates(s, &mut |s, fact| emit(s, &mut ctx, stats, fact, f));
        return Ok(());
    }

    // Rung 2 — the theory's own generator: branch on what the state owes.
    // `Mode::Declined` is the one answer that falls through, and it falls
    // through having emitted nothing.
    if rungs == Rungs::Ladder && !s.kb.program().obligations.is_empty() {
        let closed = closed_relations(s.kb, s.terms);
        let report = crate::oblgen::generate(
            s,
            &allowed,
            &excluded,
            &closed,
            crate::oblgen::Choice::from_env(),
            stats,
            &mut |s, stats, fact| emit(s, &mut ctx, stats, fact, f),
            one_branch,
        )?;
        stats.rung = report;
        if report.mode != crate::oblgen::Mode::Declined {
            return Ok(());
        }
    }

    // T1a.4.1.1 — hoisted out of `_fill_slot`, which ein.py calls once per
    // (object, relation, slot) and re-sorts `kb.names` in each. Safe because
    // the sequence cannot change mid-call: the only mutation is
    // `write_negated`'s `(not h)`, whose head bumps the name `not` and whose
    // one argument is a nested fact — so no *new* argument name is indexed,
    // and `not` itself is reserved either way.
    //
    // Eight corpus files write the kill cache **while the blind enumerator is
    // running**, so the equivalence is exercised rather than argued:
    // `branching/{01,02,03,04,06,08,12}` and `features/05`, of which
    // `06_lookahead_on.ein` writes 162 `(not h)` facts mid-call. All are
    // byte-identical through the `hyp` stream.
    let objects = candidate_objects(s.kb, s.terms);
    let by_count = by_participation(s.kb, s.terms, &objects);
    let relations = relation_plan(s, &allowed, &excluded);
    for focal in by_count {
        if raw_candidates(s, &mut ctx, stats, focal, &objects, &relations, f)?.is_break() {
            return Ok(());
        }
    }
    Ok(())
}

/// What the walk carries across candidates but not across calls.
struct Ctx {
    lookahead: Option<Lookahead>,
    kill_cache: bool,
    matcher: Matcher,
    seen: FxHashSet<FactId>,
}

/// `_emit` — count the candidate, run the pipeline, narrate, hand it on.
fn emit(
    s: &mut Session<'_>,
    ctx: &mut Ctx,
    stats: &mut HypGenStats,
    fact: FactId,
    f: &mut dyn FnMut(FactId) -> ControlFlow<()>,
) -> ControlFlow<()> {
    stats.raw += 1;
    let dropped = apply_filters(s, ctx, stats, fact);
    if dropped.is_none() {
        stats.emitted += 1;
    }
    if s.events.on() {
        let text = crate::events::sexpr(s.terms, fact);
        let verdict = dropped.map_or("emitted", Drop::as_str);
        s.events.emit("hyp", |l| {
            l.str("fact", &text);
            l.str("verdict", verdict);
        });
    }
    match dropped {
        Some(_) => ControlFlow::Continue(()),
        None => f(fact),
    }
}

/// The candidate-level pipeline, in the order that decides attribution.
///
/// Returns the **name of the filter that dropped it**, or `None` for a keeper
/// — returning the name rather than a bool is what lets a `hyp` event say
/// *why*, so a counter difference between two implementations locates itself
/// instead of having to be bisected.
fn apply_filters(
    s: &mut Session<'_>,
    ctx: &mut Ctx,
    stats: &mut HypGenStats,
    fact: FactId,
) -> Option<Drop> {
    // negated_fact — one bit test on the negated index.
    if s.kb.is_negated(fact) {
        stats.bump(Drop::NegatedFact);
        return Some(Drop::NegatedFact);
    }
    // fact_already_exists — one bit test on the presence set.
    if s.kb.contains(fact) {
        stats.bump(Drop::FactAlreadyExists);
        return Some(Drop::FactAlreadyExists);
    }
    // lookahead_killed — one rule step, no fork. Last of the checks that can
    // reject, because it is much the costliest.
    if let Some(l) = ctx.lookahead.as_ref()
        && l.dies_immediately(s, &mut ctx.matcher, fact)
    {
        stats.bump(Drop::LookaheadKilled);
        if ctx.kill_cache {
            write_negated(s, fact);
        }
        return Some(Drop::LookaheadKilled);
    }
    // seen_in_call — both Alice and Bob enumerate `(r Alice Bob)`; only the
    // first is yielded.
    if !ctx.seen.insert(fact) {
        stats.bump(Drop::SeenInCall);
        return Some(Drop::SeenInCall);
    }
    None
}

/// Cache a lookahead-killed candidate as `(not h)`, idempotently.
///
/// The kill rests on the saturated KB alone — a one-step simulation, no
/// speculative commitment — so the provenance cites **no premises**;
/// `<lookahead-dies-immediately>` is a reserved engine string whose contract is
/// that provenance walks ground out on it.
fn write_negated(s: &mut Session<'_>, hypothesis: FactId) {
    let not = s.terms.kernel.not;
    let arg = [Value::fact(hypothesis)];
    if s.terms
        .probe_fact(not, &arg)
        .is_some_and(|id| s.kb.contains(id))
    {
        return;
    }
    let rule = s.terms.kernel.lookahead_dies;
    let prov = s
        .terms
        .provs
        .push(ein_core::Prov::from_rule(rule, Box::new([]), None));
    // A worker may not be able to number `(not h)`, and the cache feeds
    // forward within the call — so an entering that reaches here on a lent
    // table is one whose *later* candidates would see a different world.
    // `Terms::refused` is set by the refusal and the fan-out discards the
    // whole entering; skipping the write keeps that discarded run cheap.
    let _ = s.kb.add_and_index_fact(s.terms, not, &arg, Some(prov));
}

// ── Blind enumeration ──────────────────────────────────────────────

/// Every non-self-edge candidate for one focal object.
///
/// The three pre-candidate relation skips run in this order because the order
/// decides which counter a skip lands in; `slot_idx` then runs ascending.
///
/// S1.7.23 — no type filter: `focal` fills *every* slot of every open
/// relation, type-blind. Type-wrong candidates are killed downstream by the
/// puzzle's own contradiction rules, not by a kernel `is-a` walk.
fn raw_candidates(
    s: &mut Session<'_>,
    ctx: &mut Ctx,
    stats: &mut HypGenStats,
    focal: Symbol,
    objects: &[Symbol],
    relations: &[RelPlan],
    f: &mut dyn FnMut(FactId) -> ControlFlow<()>,
) -> Result<ControlFlow<()>, CompileError> {
    for &RelPlan { rel, arity, skip } in relations {
        // The verdict is per relation, the **counter** is per (focal,
        // relation) — T1a.6.4.3 moves the first and leaves the second where
        // it was, because `pre_candidate` and the `hypskip` stream are T1 and
        // T2 observables and both count once per focal object.
        if let Some(skip) = skip {
            stats.skip(skip);
            hypskip(s, rel, skip.as_str(), None);
            continue;
        }
        for slot_idx in 0..arity {
            if fill_slot(s, ctx, stats, rel, arity, slot_idx, focal, objects, f)?.is_break() {
                return Ok(ControlFlow::Break(()));
            }
        }
    }
    Ok(ControlFlow::Continue(()))
}

/// One relation's standing in this call: how wide it is, and which
/// pre-candidate skip — if any — it earns before a focal object is even
/// chosen.
#[derive(Clone, Copy)]
struct RelPlan {
    rel: Symbol,
    arity: usize,
    skip: Option<Skip>,
}

/// The relation walk, decided once per call rather than once per focal object.
///
/// Nothing it reads can change mid-call: the query keywords are fixed, and the
/// only KB write a pass makes is the kill cache's `(not h)`, whose head is
/// `not` and which therefore cannot add a `(__closed__ R)` fact. The three
/// skips are tested in the order that decides which counter a skip lands in.
///
/// `kb.relations.values()` is walked in **insertion order** — a Python `dict`,
/// whose order is a language guarantee and is observable right here.
fn relation_plan(
    s: &Session<'_>,
    allowed: &Option<FxHashSet<Symbol>>,
    excluded: &FxHashSet<Symbol>,
) -> Vec<RelPlan> {
    let closed = closed_relations(s.kb, s.terms);
    s.kb.program()
        .relations
        .values()
        .filter(|r| !r.signature.is_empty())
        .map(|r| RelPlan {
            rel: r.name,
            arity: r.signature.len(),
            skip: if closed.contains(&r.name) {
                Some(Skip::ClosedRelation)
            } else if allowed.as_ref().is_some_and(|a| !a.contains(&r.name)) {
                Some(Skip::RelationNotWhitelisted)
            } else if excluded.contains(&r.name) {
                Some(Skip::NoHypothesisRelation)
            } else {
                None
            },
        })
        .collect()
}

/// Enumerate candidate-object fillers for `focal` at `fixed_slot`.
///
/// S1.22.4 — arity 1: the candidate **is** `(R focal)`; no second slot to
/// fill, so no filler loop and no self-edge check. Arity ≥ 3 is unenumerated.
///
/// S1.7.24 — no symmetric mirror: the enumerator never consults `is_symmetric`
/// to emit `(R b a)`. Both orderings already arise via different focal
/// objects; a puzzle wanting canonical pairs only declares an hrule.
///
/// S1.5.4b — Filter B ("slot already used") is intentionally absent; its
/// narrower replacement is `fact_already_exists` downstream.
#[allow(clippy::too_many_arguments)]
fn fill_slot(
    s: &mut Session<'_>,
    ctx: &mut Ctx,
    stats: &mut HypGenStats,
    rel: Symbol,
    arity: usize,
    fixed_slot: usize,
    focal: Symbol,
    objects: &[Symbol],
    f: &mut dyn FnMut(FactId) -> ControlFlow<()>,
) -> Result<ControlFlow<()>, CompileError> {
    if arity == 1 {
        // A worker cannot number a candidate nothing has numbered yet, and
        // stopping the walk is what keeps the wasted work bounded.
        // `Terms::refused` is what makes the partial enumeration unusable
        // rather than merely short, so the fan-out re-runs the entering on the
        // committing thread instead of believing it.
        let Ok(fact) = s.terms.intern_fact(rel, &[Value::sym(focal)]) else {
            return Ok(ControlFlow::Break(()));
        };
        return Ok(emit(s, ctx, stats, fact, f));
    }
    if arity != 2 {
        return Ok(ControlFlow::Continue(()));
    }
    let other_slot = 1 - fixed_slot;
    for &filler in objects {
        if filler == focal {
            stats.skip(Skip::SelfEdge);
            hypskip(s, rel, "self_edge", Some(focal));
            continue;
        }
        // `_build_args` — place the two names at their slots.
        let mut args = [Value::UNBOUND; 2];
        args[fixed_slot] = Value::sym(focal);
        args[other_slot] = Value::sym(filler);
        // See the arity-1 branch: a refusal ends the walk and `Terms::refused`
        // is what says the result is not to be believed.
        let Ok(fact) = s.terms.intern_fact(rel, &args) else {
            return Ok(ControlFlow::Break(()));
        };
        if emit(s, ctx, stats, fact, f).is_break() {
            return Ok(ControlFlow::Break(()));
        }
    }
    Ok(ControlFlow::Continue(()))
}

fn hypskip(s: &mut Session<'_>, rel: Symbol, reason: &str, object: Option<Symbol>) {
    if !(s.events.on() && s.events.verbose()) {
        return;
    }
    let relation = s.terms.sym(rel).to_string();
    let obj = object.map(|o| s.terms.sym(o).to_string());
    s.events.emit("hypskip", |l| {
        l.str("relation", &relation);
        l.str("reason", reason);
        if let Some(o) = obj.as_deref() {
            l.str("object", o);
        }
    });
}

// ── The candidate-object set ───────────────────────────────────────

/// The objects the enumerator may guess about, **sorted by name**.
///
/// A name-free signal (S1.7.23): a node of `object` category, minus the names
/// a puzzle *declares as a type* in a relation signature, minus the reserved
/// logical primitives — the rule-body / ⊥ vocabulary `not`/`false`/`and`/`or`/
/// `absent` plus the `eq`/`neq` predicates. Those can appear as a fact **head**
/// (a `(not h)` written mid-call by the kill cache is exactly that) but are
/// never puzzle objects; without the guard the enumerator would propose
/// `(R x not)` garbage.
///
/// Soundness does not depend on this set being tight, only tractability:
/// type-wrong candidates that survive die downstream against the puzzle's own
/// contradiction rules.
pub fn candidate_objects(kb: &Kb, terms: &Terms) -> Vec<Symbol> {
    // The same dense-id argument as `Kb::names` (T1a.6.4.2): every signature
    // symbol is a `u32`, so the set of type roles is a bitset built with one
    // OR per signature entry rather than a hash table filled per call.
    let mut type_nodes = ein_core::BitSet::new();
    for rel in kb.program().relations.values() {
        for &sym in rel.signature.iter() {
            type_nodes.insert(sym.0);
        }
    }
    let k = &terms.kernel;
    let reserved = [k.not, k.r#false, k.and, k.or, k.absent, k.eq, k.neq];
    let mut names = kb.names();
    // T1a.6.4.2 — **drop first, then sort**. The two steps commute: the
    // comparator is a total order (a rank is a distinct `u32` per symbol), so
    // sorting the survivors and sorting-then-dropping give the same sequence
    // — and on a blind-mode puzzle most of the list is relations, rules and
    // type roles that the filter removes. `candidate_objects` is **10.7 %** of
    // `solve examples/features/05_stdlib_domain_elim.ein -e`, which is what
    // the search costs when there is no `(hrule …)` to narrow it.
    names.retain(|&n| {
        !type_nodes.contains(n.0)
            && !reserved.contains(&n)
            && kb.category(terms, n) == NameCategory::Object
    });
    // `sorted(kb.names)` — by name, which the rank table turns into a `u32`
    // sort. `kb.names` is a set-order dict in ein.py, so this sort is not a
    // convenience: it is the only thing making the sequence reproducible.
    //
    // The table is read **once**: `Interner::rank` re-enters its `OnceCell`
    // per call and a sort calls its key function O(n log n) times, which was
    // 70 % of that symbol's samples on the blind profile.
    let ranks = terms.syms.ranks();
    names.sort_unstable_by_key(|&n| ranks[n.0 as usize]);
    names
}

/// The focal order: descending fact-participation, ties by name, **stable**.
///
/// The key already carries the name, so the stability is belt-and-braces —
/// but the port should not depend on the key alone, and `sort_by` costs
/// nothing here.
fn by_participation(kb: &Kb, terms: &Terms, objects: &[Symbol]) -> Vec<Symbol> {
    // Decorate, sort, undecorate (T1a.6.4.3). `sort_by_key` evaluates its key
    // on **every comparison**, and `Kb::participation` sums a name's entry
    // over the whole layer stack — so a fork 20 deep re-walked 20 layers
    // O(n log n) times per pass to order a list of a few dozen names.
    let ranks = terms.syms.ranks();
    let mut keyed: Vec<((std::cmp::Reverse<usize>, u32), Symbol)> = objects
        .iter()
        .map(|&n| {
            (
                (std::cmp::Reverse(kb.participation(n)), ranks[n.0 as usize]),
                n,
            )
        })
        .collect();
    keyed.sort_by_key(|&(k, _)| k);
    keyed.into_iter().map(|(_, n)| n).collect()
}

/// Every `R` with `(__closed__ R)` asserted on any layer.
///
/// Read once per generation pass. The predicate it replaces was
/// `kb.facts_of(__closed__).any(args == [rel])`, run per (focal object,
/// relation) — the whole extent, walked `|objects| × |relations|` times a pass
/// to answer a question whose answer cannot change during one (T1a.6.4.3).
/// Same membership: a `(__closed__ …)` fact whose single argument is that
/// relation's name.
fn closed_relations(kb: &Kb, terms: &Terms) -> FxHashSet<Symbol> {
    let mut out = FxHashSet::default();
    let Some(closed) = terms.syms.get(CLOSED) else {
        return out;
    };
    for f in kb.facts_of(closed) {
        if let [arg] = terms.facts.args(f)
            && let Some(rel) = arg.as_sym()
        {
            out.insert(rel);
        }
    }
    out
}

/// Which relations **this program's generator can still propose** — the
/// eligibility M1e S1e.2.3's refutation warning is stated over.
///
/// It reads the ladder rather than one rung, because the hazard is *the search
/// can still add a fact of R*, and which rung would add it does not matter:
///
/// - **hrules** are an override, so the eligible set is exactly what they
///   conclude. `closed` / `:hypothesis-relations` / `:no-hypothesis` are not
///   applied — `apply_filters` does not consult them and neither does
///   [`Hrules`], so a puzzle that declares a generator gets what it asked for.
/// - **obligations** propose the facts that would discharge what the state
///   owes, so the set is the `(open ?R)` arguments, scoped by all three.
/// - otherwise the **blind** set: every declared relation with a signature,
///   less the three scopings — `relation_plan`'s filter, read off the same
///   helpers so the two cannot drift.
///
/// Note the closed set is the one **in the KB**, not `emit_closed`'s: that
/// pass runs on a fork for `--hyp-stats` and the summary, so a solve sees only
/// authored and `std.closure`-derived markers ([`crate::closed`]).
///
/// **One residual, written down rather than papered over.** The obligations
/// rung can *decline* — a bare `(open)`, a projection that will not resolve
/// per activator, or [`crate::oblgen`]'s C4 — and fall through to the blind
/// enumerator, which proposes far more. Whether it declines is a property of
/// the state, not of the program, so this is the rung the program *means* and
/// not provably the rung it takes. Answering it exactly would mean running
/// `oblgen`'s prologue before root has a hypothesis, and the cost of being
/// wrong here is an under-warning on an advisory. Taking the union instead was
/// tried and rejected: on `zebra2-obligations.ein` it makes every declared
/// relation eligible and the warning fires 40 times on a puzzle that solves
/// correctly — which is a warning nobody would leave on.
///
/// It is also **not** the stratification question, and the two do not overlap:
/// a watched relation a *rule* derives can flip a guard during saturation,
/// which `warn_derived_naf` is about and which is sound since S1.21.8 because
/// the guard is judged at a fixpoint. This one is about a *commitment*
/// discharging the guard in a world the search never enters.
pub fn eligible_relations(s: &mut Session<'_>) -> Result<FxHashSet<Symbol>, CompileError> {
    if !s.kb.program().hrules.is_empty() {
        let hrules = Hrules::new(s)?;
        return Ok(hrules.asserted_relations(s.terms));
    }
    let allowed = query_relations(s, HYPOTHESIS_RELATIONS);
    let excluded = query_relations(s, NO_HYPOTHESIS).unwrap_or_default();
    let closed = closed_relations(s.kb, s.terms);
    let scoped = |r: Symbol| {
        !closed.contains(&r)
            && !excluded.contains(&r)
            && allowed.as_ref().is_none_or(|a| a.contains(&r))
    };
    if !s.kb.program().obligations.is_empty() {
        let memo = s.memo.clone();
        let plans = crate::obligations::plans_for(s.kb, s.terms, s.ast, &memo)?;
        return Ok(plans
            .iter()
            .filter_map(|p| crate::obligations::open_argument(p, s.terms))
            .filter(|&r| scoped(r))
            .collect());
    }
    Ok(s.kb
        .program()
        .relations
        .values()
        .filter(|r| !r.signature.is_empty())
        .map(|r| r.name)
        .filter(|&r| scoped(r))
        .collect())
}

// ── Query-scoped relation sets ─────────────────────────────────────

/// The relation-name set under a `(query … :KEYWORD …)` keyword.
///
/// `None` means the keyword is absent — for `:hypothesis-relations` that is
/// "unrestricted", and note ein.py's `or None`, which turns an *empty* list
/// back into unrestricted too. The **first** matching keyword wins, not the
/// last: ein.py returns out of the loop.
fn query_relations(s: &Session<'_>, keyword: &str) -> Option<FxHashSet<Symbol>> {
    let query = s.kb.program().query()?;
    for &pair in query.kw_pairs.iter() {
        let ein_ir::Node::KwPair { key, value } = s.ast.node(ein_ir::NodeId(pair.0)) else {
            continue;
        };
        let ein_ir::Node::Keyword(name) = s.ast.node(key) else {
            continue;
        };
        if s.ast.sym(name) != keyword {
            continue;
        }
        let (names, declared) = coerce_relation_names(s, value);
        // `frozenset(...) or None` — an *empty list* is unrestricted. What
        // decides that is how many names the keyword **declared**, not how
        // many resolved: a name no fact or declaration ever used has no
        // `Symbol`, and dropping it silently would turn a whitelist of one
        // misspelled relation — which excludes everything — into no whitelist
        // at all, which excludes nothing.
        return if declared == 0 { None } else { Some(names) };
    }
    None
}

/// A keyword's value as relation names — a bare SYMBOL (one relation) or an
/// `(r1 r2 …)` list, reading head **and** atom args.
///
/// Returns the resolvable ones and the count of *declared* ones; see the
/// caller for why those are not the same number.
fn coerce_relation_names(s: &Session<'_>, value: ein_ir::NodeId) -> (FxHashSet<Symbol>, usize) {
    let mut out = FxHashSet::default();
    let mut declared = 0;
    let mut take = |node: ein_ir::NodeId| {
        if let Some(name) = s.ast.atom_name(node) {
            declared += 1;
            if let Some(sym) = s.terms.syms.get(name) {
                out.insert(sym);
            }
        }
    };
    match s.ast.node(value) {
        ein_ir::Node::Atom(_) => take(value),
        ein_ir::Node::SForm { head, args } => {
            for node in std::iter::once(head).chain(s.ast.args(args).iter().copied()) {
                take(node);
            }
        }
        _ => {}
    }
    (out, declared)
}

// ── Scoring ────────────────────────────────────────────────────────

/// Ordering score for a hypothesis — higher means tried first.
///
/// `"most-constrained"` returns `0.0`, so the sort falls through to the
/// content tiebreakers and the S1.5a.1a determinism property is preserved.
/// `"popularity"` is `rel_w · |extent(R)| + obj_w · Σ |names[arg].as_arg|` over
/// **string** args only — a nested fact or an int has no name to index by and
/// contributes nothing.
///
/// The two reserved modes and the unknown-mode case return their ein.py
/// messages verbatim; both are surfaced at first call rather than at load.
pub fn score_hypothesis(kb: &Kb, terms: &Terms, fact: FactId) -> Result<f64, ScoreError> {
    // `kb.config or SolverConfig()` is what the *generator* does; scoring does
    // **not**. ein.py reads the mode through
    // `getattr(cfg, …) if cfg is not None else "most-constrained"`, so a KB
    // with no `(config …)` block falls to the neutral score rather than to
    // the default's `"popularity"` — and the two differ, since the default
    // flipped in S1.5a.7. Reproduced, not tidied.
    let Some(cfg) = kb.program().config.clone() else {
        return Ok(0.0);
    };
    match cfg.hypgen_scoring.as_str() {
        "most-constrained" => Ok(0.0),
        "popularity" => {
            let (rel, args) = terms.facts.get(fact);
            let rel_count = kb.n_facts_of(rel) as f64;
            let obj_count_sum: f64 = args
                .iter()
                .filter_map(|a| a.as_sym())
                .map(|sym| kb.name_as_arg(sym).count() as f64)
                .sum();
            Ok(cfg.hypgen_rel_weight * rel_count + cfg.hypgen_obj_weight * obj_count_sum)
        }
        mode @ ("branch-info" | "popularity+branch-info") => {
            Err(ScoreError::NotImplemented(format!(
                "hypgen-scoring={} is reserved for a follow-up stage; today \
                 only 'most-constrained' and 'popularity' are wired. See \
                 M1 P1.5as1.5a.7_hypgen_scoring_branch_info.md § T1.5a.7.3.",
                ein_core::pyrepr::repr_str(mode)
            )))
        }
        mode => Err(ScoreError::Unknown(format!(
            "unknown hypgen-scoring mode: {} (expected 'most-constrained' or \
             'popularity')",
            ein_core::pyrepr::repr_str(mode)
        ))),
    }
}

/// ein.py raises two *different* exception types here, and a caller may tell
/// them apart, so the port does too.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ScoreError {
    /// `NotImplementedError`.
    NotImplemented(String),
    /// `ValueError`.
    Unknown(String),
}

impl std::fmt::Display for ScoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreError::NotImplemented(m) | ScoreError::Unknown(m) => f.write_str(m),
        }
    }
}

// ── Solution predicates ────────────────────────────────────────────

/// The open set — viable, not-yet-decided hypotheses.
///
/// [`generate`] already yields exactly the candidates that are neither
/// asserted nor refuted nor immediately doomed, so this is its result as a
/// set. `open_hypotheses` and [`complete`] stay **distinct**: collapsing them
/// is a measurable regression, not a simplification.
///
/// S1.7.24 — no symmetric canonicalisation: `(R a b)` and `(R b a)` are two
/// distinct open entries even for a `(symmetric R)` relation. Correct `k` is
/// recovered generically at the `state_key` dedup, because the *user's* rule
/// established the equivalence.
pub fn open_hypotheses(s: &mut Session<'_>) -> Result<FxHashSet<FactId>, CompileError> {
    let mut open = FxHashSet::default();
    let mut stats = HypGenStats::new();
    generate(s, &mut stats, &mut |fact| {
        open.insert(fact);
        ControlFlow::Continue(())
    })?;
    Ok(open)
}

/// No open hypothesis — the generator proposes nothing undecided.
///
/// Short-circuits (S1.9.E16): the question is emptiness, so the **first**
/// candidate answers it. Building the whole set only to truth-test it re-ran
/// the per-candidate pipeline — the one-step lookahead included — for every
/// later candidate whose value was already irrelevant; measured 54 ms of a
/// 1.7 s `zebra2` `solve(stop_after=1)`, where 8 of 9 `complete` calls are
/// answered by candidate #1.
pub fn complete(s: &mut Session<'_>) -> Result<bool, CompileError> {
    complete_counted(s, &mut HypGenStats::new())
}

/// [`complete`], reporting what the short-circuit cost.
///
/// The short-circuit is not visible in the answer — only in how many
/// candidates were built to reach it — so the instrument that checks it needs
/// the stats block, and that is the only reason this is separate.
pub fn complete_counted(
    s: &mut Session<'_>,
    stats: &mut HypGenStats,
) -> Result<bool, CompileError> {
    ein_core::counters::bump(|c| c.hypgen_complete += 1);
    let mut any = false;
    generate(s, stats, &mut |_| {
        any = true;
        ControlFlow::Break(())
    })?;
    Ok(!any)
}

/// No contradiction — no `(false)`, no same-layer `X ∧ ¬X`.
pub fn consistent(kb: &Kb, terms: &Terms) -> bool {
    !crate::contradiction::has_contradiction(kb, terms)
}

/// `consistent ∧ complete` — P1.7a's domain-agnostic definition of an answer.
pub fn is_solution_node(s: &mut Session<'_>) -> Result<bool, CompileError> {
    if !consistent(s.kb, s.terms) {
        return Ok(false);
    }
    complete(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_core::SolverConfig;
    use ein_ir::{Ast, from_ir::load, parse};

    fn kb_of(src: &str) -> (Ast, Terms, Kb) {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        (ast, terms, kb)
    }

    fn scored(src: &str, mode: Option<&str>) -> Result<f64, ScoreError> {
        let (_ast, mut terms, mut kb) = kb_of(src);
        if let Some(m) = mode {
            kb.program_mut().config = Some(SolverConfig {
                hypgen_scoring: m.to_string(),
                ..SolverConfig::default()
            });
        }
        let r = terms.syms.get("r").expect("r is interned");
        let a = terms.syms.get("A").expect("A is interned");
        let fact = terms.intern_fact(r, &[Value::sym(a)]).expect("room");
        score_hypothesis(&kb, &terms, fact)
    }

    const SRC: &str = "(relation r T)\n(r A)";

    /// A KB with no `(config …)` block scores **0.0**, not the default mode's
    /// popularity — `score_hypothesis` reads the mode off `kb.config` and
    /// falls back to `"most-constrained"` when there is none, while the
    /// dataclass default has been `"popularity"` since S1.5a.7. Two different
    /// fallbacks for the same field, and the port keeps both.
    #[test]
    fn no_config_block_scores_neutral_not_popular() {
        assert_eq!(scored(SRC, None), Ok(0.0));
        assert_eq!(scored(SRC, Some("popularity")), Ok(2.0));
    }

    /// `rel_w · |extent(R)| + obj_w · Σ |names[arg].as_arg|` — one `r`-fact
    /// and one appearance of `A` as an argument.
    #[test]
    fn popularity_weighs_the_relation_and_each_named_argument() {
        assert_eq!(scored(SRC, Some("popularity")), Ok(2.0));
        assert_eq!(scored(SRC, Some("most-constrained")), Ok(0.0));
    }

    /// The two reserved modes and the unknown case, byte-for-byte — they are
    /// user-visible text, and the messages were captured from ein.py rather
    /// than paraphrased.
    #[test]
    fn the_unwired_modes_report_ein_pys_text() {
        assert_eq!(
            scored(SRC, Some("branch-info")),
            Err(ScoreError::NotImplemented(
                "hypgen-scoring='branch-info' is reserved for a follow-up \
                 stage; today only 'most-constrained' and 'popularity' are \
                 wired. See M1 P1.5as1.5a.7_hypgen_scoring_branch_info.md \
                 § T1.5a.7.3."
                    .to_string()
            ))
        );
        assert_eq!(
            scored(SRC, Some("nonsense")),
            Err(ScoreError::Unknown(
                "unknown hypgen-scoring mode: 'nonsense' (expected \
                 'most-constrained' or 'popularity')"
                    .to_string()
            ))
        );
    }

    /// `as_report_lines`' field widths and its sorted, sparse key walk — a
    /// counter that was never bumped has no line at all.
    #[test]
    fn the_stats_report_omits_the_keys_that_never_fired() {
        let mut stats = HypGenStats::new();
        stats.raw = 9;
        stats.emitted = 1;
        stats.bump(Drop::LookaheadKilled);
        stats.bump(Drop::FactAlreadyExists);
        stats.bump(Drop::FactAlreadyExists);
        stats.skip(Skip::SelfEdge);
        assert_eq!(
            stats.report_lines(),
            [
                "  raw                9",
                "  emitted            1",
                "  filtered.fact_already_exists 2",
                "  filtered.lookahead_killed   1",
                "  pre.self_edge               1",
            ]
        );
        assert!(!stats.balances(), "1 + 3 != 9");
        stats.raw = 4;
        assert!(stats.balances());
    }

    /// The blind enumerator proposes type-blind (S1.7.23) but never proposes a
    /// **type-role** atom or a kernel primitive as an object: `T` is named in
    /// a signature and `not` is reserved, so neither is a candidate — while
    /// `A`, which no signature mentions, is.
    #[test]
    fn candidate_objects_drop_signature_atoms_and_primitives() {
        let (_ast, terms, kb) = kb_of("(relation r T)\n(r A)\n(not (r B))\n(relation s T)\n(s B)");
        let names: Vec<&str> = candidate_objects(&kb, &terms)
            .iter()
            .map(|&s| terms.sym(s))
            .collect();
        assert_eq!(names, ["A", "B"], "sorted by name, T and `not` excluded");
    }
}
