//! Hypothesis generation — the enumerator that proposes what to guess.
//!
//! **Three** modes since M1d S1d.2.5, and the switch is a ladder: rule-driven
//! when the puzzle declares any `(hrule …)` ([`crate::hrule`]); otherwise
//! obligation-driven where an obligation is **activated** at the state, a plan
//! its activators compile ([`crate::oblgen`], which proposes what would pay
//! the debts the state owes, and for a debt a rule can bring a commitment to
//! owe as well), so that under an activated obligation a state that owes
//! nothing proposes nothing and is complete; otherwise, or where that rung
//! declines, blind combinatorial. An obligation rule the program loads and
//! nothing activates chooses no rung: every import brings its module whole,
//! and a module imported for another of its names takes nothing off the blind
//! enumerator. The switch is one function, `rung`, which
//! [`eligible_relations`] reads too. Then an eight-stage
//! filter pipeline whose **attribution** — which counter a drop lands in — is
//! observable through [`HypGenStats`] and through the `hyp` / `hypskip`
//! events, and every rung feeds it: what changes between them is which
//! candidates are proposed, never what happens to one afterwards.
//!
//! ### What must not change
//!
//! - **The filter order.** It decides which counter a drop is attributed to,
//!   and the counters are observable. A reordering that looks like an
//!   optimisation changes them
//!   ([design/search.md § 1](../../../../docs/ein/ein.rs/design/search.md#1-hypothesis-generation)).
//! - **The candidate order.** A route reads it only as a tie-break: H is
//!   sorted by each candidate's raw text and deduped before any route sees it
//!   (`region::order_key`), and the generator's order decides only between
//!   two candidates of one raw text. It is observable all the same: it is the
//!   order the `hyp` events narrate a call in, under `--events`, and it
//!   decides which repeat of a candidate `seen_in_call` counts.
//! - **The kernel imposes no type system** (S1.7.23). The enumerator proposes
//!   type-blind and the puzzle's own rules do the pruning; Rust code that
//!   reaches for `is-a` here is reintroducing what that stage removed. The
//!   *signature* atoms are excluded, which is a different thing: a name a
//!   puzzle declared as a type role, derived without naming `is-a`. Since M1e2
//!   S1e2.2.10 [`crate::types_as_objects`] reads `is-a` by name, after this
//!   pass and outside it, to say where the enumerator bound a type as an
//!   object (Q-M1e.12 (a)): it decides nothing, and no candidate moves.
//!
//! ### What is free
//!
//! The cost. There are ~18 k raw candidates per call on `zebra2`, and the
//! *row key is the identity*, so a candidate costs one intern — a hash lookup
//! that returns the same `FactId` every later call — and the two filters that
//! reject almost all of them are single bit tests on it
//! ([design/search.md § 1](../../../../docs/ein/ein.rs/design/search.md#1-hypothesis-generation)).
//!
//! design/07 calls this "intern-on-demand: probe, and only materialise on
//! survival". The probe and the intern turned out to be *the same hash
//! lookup* — `FactStore::intern` is `probe` plus a push on a miss — and the
//! push is bounded by the distinct candidate space, not by the number of
//! calls, so the split would buy a branch and cost the caller a second
//! lookup. What the design was actually protecting against — a per-candidate
//! allocation on the rejected path — is absent either way, and a candidate is
//! never anything but its `FactId`.

use std::ops::ControlFlow;

use ein_core::entities::NameCategory;
use ein_core::{FactId, Kb, Symbol, Terms, Value};
use ein_ir::Ast;
use rustc_hash::FxHashSet;

use crate::compile::CompileError;
use crate::hrule::Hrules;
use crate::saturator::Session;

/// The `(query …)` keyword restricting which relations the blind enumerator
/// builds candidates for (S1.7.25). Reserved engine string.
pub const HYPOTHESIS_RELATIONS: &str = "hypothesis-relations";

/// Its exclusion dual (S1.9.E3): relations never to guess on, while saturation
/// rules on them still fire. Reserved engine string.
pub const NO_HYPOTHESIS: &str = "no-hypothesis";

/// The `(solve …)` flag that states the lattice is root alone (M1f
/// S1f.6.36): no rung runs, so no candidate is proposed whatever the program
/// also declares, and `R` is {∅} where root is alive and discharged, empty
/// otherwise. Reserved engine string.
pub const NO_SEARCH: &str = "no-search";

/// Does the program state `(solve … :no-search)`? Read off the `(solve …)`
/// block as written, as the two lists are (`query_relations`): the flag is
/// part of the program, and every command reads it.
pub fn declares_no_search(program: &ein_core::Program, ast: &Ast) -> bool {
    program.solve().is_some_and(|solve| {
        solve.kw_pairs.iter().any(|&pair| {
            matches!(ast.node(ein_ir::NodeId(pair.0)), ein_ir::Node::KwPair { key, .. }
                if matches!(ast.node(key), ein_ir::Node::Keyword(k) if ast.sym(k) == NO_SEARCH))
        })
    })
}

/// A candidate-level filter — the name it bumps in `stats.filtered`, and the
/// `verdict` a `hyp` event carries.
///
/// The discriminants are the **sorted** key order, because the JSON summary
/// walks them sorted.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Drop {
    FactAlreadyExists = 0,
    NegatedFact = 1,
    SeenInCall = 2,
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

/// Per-filter counters for one pass of the generator.
///
/// The two per-filter tallies are dense arrays, and a **zero count means the
/// key was never bumped**, since a bump is `+= 1`. That equivalence is what
/// lets the JSON summary print only the keys that were bumped.
///
/// `raw == emitted + sum(filtered)`: every candidate built is emitted or
/// dropped by exactly one filter.
#[derive(Clone, Default, Debug)]
pub struct HypGenStats {
    pub raw: u64,
    pub emitted: u64,
    pub filtered: [u64; 3],
    pub pre_candidate: [u64; 4],
    /// Which rung of the ladder this call took, and what it found owed — M1d
    /// S1d.2.5. All zeros and [`crate::oblgen::Mode::Blind`] for a program
    /// that declares no obligation rule, which is the shape every counter
    /// baseline was taken under.
    pub rung: crate::oblgen::RungReport,
    /// The debts the state does not owe and a commitment can bring, each with
    /// the candidates this call emitted for it — the obligations rung's
    /// ([`crate::oblgen::LaterDebt`]). Empty on every other rung, and on a
    /// program in which no rule that reads what a commitment can change
    /// writes an obligation's activator.
    pub later: Vec<crate::oblgen::LaterDebt>,
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
}

/// `(__closed__ R)` — the kernel's closed-relation marker (S1.7.25). A closed
/// relation contributes zero candidates; the generator does not care whether
/// the fact was authored or derived by a rule (`std.closure`'s).
pub const CLOSED: &str = ein_core::terms::CLOSED;

// ── The generator ──────────────────────────────────────────────────

/// The blind combinatorial enumerator alone, whatever the program declares,
/// and **config-free** as [`generate_config_free`] is.
///
/// **Not a search entry point.** No route walks the ladder; this answers a
/// question *about* a state that the ladder cannot be asked, because the rung
/// that is active there stops as soon as its own candidates run out: *how many
/// facts would the blind enumerator still propose at a state the rung called
/// complete?* That number is the state's
/// [leftover-open count](../../../../docs/ein/reasoning/kb-evolution/satisfiability.md#5-a-state-of-r-is-a-model-of-the-program-as-h-scopes-it),
/// and it is what separates "one model" from "2ⁿ models" when the reading
/// is open-world.
///
/// Config-free since M1e2
/// [S1e2.3.1](../../../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.1_the_summary_root_block.md),
/// the user's ruling of 2026-11-03: the count is taken outside H, and H is
/// taken with no filter, so a count through a filter H's pass does not apply
/// would not be comparable with it. Until then the pass ran the walk's
/// one-step lookahead, and `zebra2`'s unique model left 3 678 where it leaves
/// 8 203 now. It writes nothing but the interning of its candidates.
pub fn generate_blind(
    s: &mut Session<'_>,
    stats: &mut HypGenStats,
    f: &mut dyn FnMut(FactId) -> ControlFlow<()>,
) -> Result<(), CompileError> {
    generate_with(s, stats, f, Rungs::BlindOnly)
}

/// Yield candidate hypothesis facts in the ladder's order, calling `f` per
/// survivor: the **config-free** candidate list, whatever the program's
/// `(config …)` says. It is the list H is built from at root's fixpoint
/// (`region::prepare_with`), so every command and the oracle read it.
///
/// No config option may change R, so the lattice's universe cannot be a list a
/// switch filtered, and no field of the config reaches this pass. It writes
/// nothing: the call is pure apart from interning its candidates.
///
/// A callback rather than an iterator because the walk can stop early: a
/// `ControlFlow::Break` from `f` ends it, and so does an interning a lent
/// table refuses (`Terms::refused`).
pub fn generate_config_free(
    s: &mut Session<'_>,
    stats: &mut HypGenStats,
    f: &mut dyn FnMut(FactId) -> ControlFlow<()>,
) -> Result<(), CompileError> {
    generate_with(s, stats, f, Rungs::Ladder)
}

/// The rung of the ladder a program takes at a state, before any rung runs:
/// what [`rung`] decides.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Rung {
    /// `(solve :no-search)`: no rung runs.
    NoSearch,
    /// The program declares an `(hrule …)`: rung 1, alone.
    Hrules,
    /// An obligation is activated at the state: rung 2, which may still
    /// decline to the blind enumerator ([`crate::oblgen::Mode::Declined`]).
    Obligations,
    /// Neither: rung 3, the blind enumerator. A program that loads an
    /// obligation rule and activates none is here.
    Blind,
}

/// **The switch**: which rung the ladder takes at `s`'s state, which is root
/// wherever H is built. In order: `(solve :no-search)`; an hrule the program
/// declares, active or not; an obligation **activated** at the state, a plan
/// the state's activators compile, an obligation rule without parameters at
/// every state ([`crate::mechanisms::obligation_plans`], the one reading of
/// an obligation, which the mechanisms check reads too); otherwise the blind
/// enumerator.
///
/// So the second rung is chosen by what the program states, an obligation
/// with its activator, and not by what it loads: every import brings its
/// module whole, obligation rules among it, and a rule nothing activates
/// chooses no rung. Nor is it chosen by what the state happens to owe: under
/// an activated obligation a discharged state proposes nothing and is
/// complete, which is what *complete means discharged* has to mean.
///
/// The ladder (`generate_with`) and [`eligible_relations`] read this one
/// function, so the rung a call takes and the relations the advisory calls
/// eligible are decided once.
pub(crate) fn rung(s: &mut Session<'_>) -> Result<Rung, CompileError> {
    if declares_no_search(s.kb.program(), s.ast) {
        return Ok(Rung::NoSearch);
    }
    if !s.kb.program().hrules.is_empty() {
        return Ok(Rung::Hrules);
    }
    if crate::mechanisms::obligation_plans(s)?.is_empty() {
        return Ok(Rung::Blind);
    }
    Ok(Rung::Obligations)
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
/// One body for every entry point, because the setup — the `(query …)`
/// relation lists, the per-call `Ctx` — is what every rung shares, and a
/// second copy of it is a second place for the filter order to drift.
fn generate_with(
    s: &mut Session<'_>,
    stats: &mut HypGenStats,
    f: &mut dyn FnMut(FactId) -> ControlFlow<()>,
    rungs: Rungs,
) -> Result<(), CompileError> {
    ein_core::counters::bump(|c| c.hypgen_call += 1);
    // The switch, [`rung`]; the probe takes the blind enumerator whatever the
    // program declares.
    let rung = match rungs {
        Rungs::Ladder => rung(s)?,
        Rungs::BlindOnly => Rung::Blind,
    };
    // `(solve :no-search)` (M1f S1f.6.36): the program states that its
    // lattice is root alone, so no rung runs and nothing is proposed, whatever
    // hrules, obligations or lists it also declares. Before anything is
    // compiled, so the call narrates nothing.
    if rung == Rung::NoSearch {
        stats.rung.mode = crate::oblgen::Mode::NoSearch;
        return Ok(());
    }
    let allowed = query_relations(s, HYPOTHESIS_RELATIONS);
    let excluded = query_relations(s, NO_HYPOTHESIS).unwrap_or_default();
    let mut ctx = Ctx {
        seen: FxHashSet::default(),
    };

    if rung == Rung::Hrules {
        // Rung 1 — the user's own generator, an override. `(hrule …)` presence
        // *is* the switch, exactly as design/07 wrote it: a puzzle that says
        // what to guess is never second-guessed by what it owes.
        stats.rung.mode = crate::oblgen::Mode::Hrules;
        let hrules = Hrules::new(s)?;
        let _ = hrules.candidates(s, &mut |s, fact| emit(s, &mut ctx, stats, fact, f));
        return Ok(());
    }

    // Rung 2 — the theory's own generator: branch on what the state owes,
    // where an obligation is activated. `Mode::Declined` is the one answer
    // that falls through, and it falls through having emitted nothing.
    if rung == Rung::Obligations {
        let closed = closed_relations(s.kb, s.terms);
        let report = crate::oblgen::generate(
            s,
            &allowed,
            &excluded,
            &closed,
            stats,
            &mut |s, stats, fact| emit(s, &mut ctx, stats, fact, f),
        )?;
        stats.rung = report;
        if report.mode != crate::oblgen::Mode::Declined {
            return Ok(());
        }
    }

    // T1a.4.1.1 — hoisted out of the per-(object, relation, slot) fill, which
    // would otherwise re-sort the names in each. Safe because the sequence
    // cannot change mid-call: the pass writes nothing to the KB, and
    // interning a candidate indexes no name.
    let objects = candidate_objects(s.kb, s.terms, s.ast);
    let by_count = by_participation(s.kb, s.terms, &objects);
    let relations = relation_plan(s, &allowed, &excluded);
    for focal in by_count {
        if raw_candidates(s, &mut ctx, stats, focal, &objects, &relations, f)?.is_break() {
            return Ok(());
        }
    }
    Ok(())
}

/// What a pass carries across candidates but not across calls.
struct Ctx {
    seen: FxHashSet<FactId>,
}

/// Count the candidate, run the pipeline, narrate, hand it on.
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
/// *why*, so a counter that moves between two builds locates the filter that
/// moved it instead of having to be bisected.
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
    // seen_in_call — both Alice and Bob enumerate `(r Alice Bob)`; only the
    // first is yielded.
    if !ctx.seen.insert(fact) {
        stats.bump(Drop::SeenInCall);
        return Some(Drop::SeenInCall);
    }
    None
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
        // it was, because `pre_candidate` and the `hypskip` stream are both
        // observable and both count once per focal object.
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
/// Nothing it reads can change mid-call: the query keywords are fixed, and a
/// pass writes nothing to the KB, so no `(__closed__ R)` fact arrives. The
/// three skips are tested in the order that decides which counter a skip
/// lands in.
///
/// The program's relation registry is walked in **insertion order** —
/// declaration order, which is observable right here.
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
        // Place the two names at their slots.
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

/// The objects the enumerator may guess about, **sorted by name**: every name
/// the KB holds, and every name of the program
/// ([`crate::invariant::program_names`]).
///
/// **The second half is the seed.** M1e2 S1e2.2.4 seeded the pool with every
/// constant a rule's `:assert` names — Q-M1e.21 (b), the user's ruling of
/// 2026-10-16, *"rules are part of root, so rules can't assert new names"* —
/// and S1e2.2.14 with every name of the program, on the user's ruling of
/// 2026-11-04 on Q-M1e2.9, (a): a constant in a rule's `:match` too, and one a
/// fact names only inside a nested fact, whose arguments `kb.names()` does not
/// index (Q40). A constant only a `(query …)` names is not one. Until then the
/// pool missed them, and `examples/ein-bugs/alive-set-fresh-name.ein` (W4),
/// `wrong-answers/match-only.ein` and `negative-only.ein` (W15) lost their
/// models to it. [`crate::invariant::program_names`] is the one list of the
/// program's names, so this reads it rather than walking the program a second
/// way.
///
/// A name-free signal (S1.7.23): a node of `object` category, minus the names
/// a puzzle *declares as a type* in a relation signature, minus the reserved
/// logical primitives — the rule-body / ⊥ vocabulary `not`/`false`/`and`/`or`/
/// `absent` plus the `eq`/`neq` predicates. Those can appear as a fact **head**
/// (a `(not h)` written mid-call by the kill cache is exactly that) but are
/// never puzzle objects; without the guard the enumerator would propose
/// `(R x not)` garbage.
///
/// **A kernel dunder is never an object.** `__closed__` and `__symmetric__`
/// are the kernel's words for a relation's closure and its symmetry, not
/// things a puzzle is about, and they are left out by name, as the primitives
/// are. Without the guard a dunder was an object wherever no file wrote a
/// fact headed by it: the names index holds the head of every fact, the
/// registry the head of a fact a file writes, so a closure a rule derives put
/// `__closed__` among the objects where the same closure written as a fact
/// did not, and one program stated two ways had two answers. A dunder a
/// program writes in an argument position, as `(imply1 functional __closed__)`
/// does, is a name of the program all the same
/// ([`crate::invariant::program_names`] lists it, and the alive-set invariant
/// reads that list); it is this set, the enumerator's, that leaves it out.
/// `ein-infer/tests/dunder_objects.rs` holds both.
///
/// A relation's name is an object only where the registry does not hold it:
/// the head of a fact that only a rule concludes. Such a candidate stays, and
/// [`crate::relation_names`] names it for the read-out.
///
/// Soundness does not depend on this set being tight, only tractability:
/// type-wrong candidates that survive die downstream against the puzzle's own
/// contradiction rules.
pub fn candidate_objects(kb: &Kb, terms: &Terms, ast: &Ast) -> Vec<Symbol> {
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
    // The kernel's two dunders: its own words, never an object, whether a
    // rule derives a fact headed by one or a program writes one as a value.
    let dunders = [k.closed, k.symmetric];
    let mut names = kb.names();
    // The seed: a name of the program the names index lacks joins it, once
    // (`alive_invariant.rs` lists the corpus's).
    let mut seen = ein_core::BitSet::new();
    for &n in &names {
        seen.insert(n.0);
    }
    for n in crate::invariant::program_names(kb, terms, ast) {
        if seen.insert(n.name.0) {
            names.push(n.name);
        }
    }
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
            && !dunders.contains(&n)
            && kb.category(terms, n) == NameCategory::Object
    });
    // By name, which the rank table turns into a `u32` sort. `Kb::names` is
    // first-seen order, which is not a contract, so this sort is not a
    // convenience: it is the only thing making the sequence a function of
    // the names.
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
/// but the order should not depend on the key alone, and `sort_by` costs
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

/// The program's own three scopings, as every rung reads them: the
/// `:hypothesis-relations` list (`None` when absent or empty), the
/// `:no-hypothesis` list, and the relations with `(__closed__ R)` at this
/// state — what [`crate::scope`] does not name, because the program said it.
pub(crate) fn scoping(
    s: &Session<'_>,
) -> (
    Option<FxHashSet<Symbol>>,
    FxHashSet<Symbol>,
    FxHashSet<Symbol>,
) {
    (
        query_relations(s, HYPOTHESIS_RELATIONS),
        query_relations(s, NO_HYPOTHESIS).unwrap_or_default(),
        closed_relations(s.kb, s.terms),
    )
}

/// Which relations **this program's generator can still propose** — the
/// eligibility M1e S1e.2.3's refutation warning is stated over.
///
/// It reads the ladder rather than one rung, because the hazard is *the search
/// can still add a fact of R*, and which rung would add it does not matter.
/// The rung is the generator's own switch, `rung`, read once for both:
///
/// - **hrules** are an override, so the eligible set is exactly what they
///   conclude. `closed` / `:hypothesis-relations` / `:no-hypothesis` are not
///   applied — `apply_filters` does not consult them and neither does
///   [`Hrules`], so a puzzle that declares a generator gets what it asked for.
/// - **obligations**, where one is activated, propose the facts that would
///   discharge what the state owes, so the set is the `(open ?R)` arguments
///   of the activated plans, scoped by all three. The
///   rung proposes for a debt a commitment can bring too
///   ([`crate::oblgen::LaterDebt`]), and that debt's relation is **not** in
///   this set: which debts those are is read from the rung's own scans, and
///   this is the advisory's cheap reading of the program's plans at the
///   state.
/// - otherwise the **blind** set: every declared relation with a signature,
///   less the three scopings — `relation_plan`'s filter, read off the same
///   helpers so the two cannot drift.
///
/// The closed set is the one **in the KB**: the markers a program writes and
/// the ones a rule derives (`std.closure`'s).
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
    // The ladder's switch, read as the generator reads it.
    let rung = rung(s)?;
    // `(solve :no-search)`: no rung proposes anything (M1f S1f.6.36).
    if rung == Rung::NoSearch {
        return Ok(FxHashSet::default());
    }
    if rung == Rung::Hrules {
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
    if rung == Rung::Obligations {
        let plans = crate::mechanisms::obligation_plans(s)?;
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
/// "unrestricted". A written `()` is a list that names none, so it
/// whitelists nothing. The **first** matching keyword wins, not the last.
///
/// Since M1f S1f.7.8 every name a list holds is a relation the program
/// writes outside it, or the loader refuses the program
/// (`ein_ir::from_ir`'s `refuse_solve_names`), so a misspelt name reaches
/// neither this nor [`crate::hrule`]'s activators.
fn query_relations(s: &Session<'_>, keyword: &str) -> Option<FxHashSet<Symbol>> {
    // On `(solve …)` since M1e2 S1e2.1a.4: the candidate list is the lattice's,
    // stated once for every query.
    let solve = s.kb.program().solve()?;
    for &pair in solve.kw_pairs.iter() {
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
        // What decides whether the list restricts
        // is how many names the keyword **declared**, not how
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

#[cfg(test)]
mod tests {
    use super::*;
    use ein_ir::{Ast, from_ir::load, parse};

    fn kb_of(src: &str) -> (Ast, Terms, Kb) {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        (ast, terms, kb)
    }

    /// The blind enumerator proposes type-blind (S1.7.23) but never proposes a
    /// **type-role** atom or a kernel primitive as an object: `T` is named in
    /// a signature and `not` is reserved, so neither is a candidate — while
    /// `A`, which no signature mentions, is.
    #[test]
    fn candidate_objects_drop_signature_atoms_and_primitives() {
        let (ast, terms, kb) = kb_of(
            "(relation r T :why \"r holds of {?1}\")\n(r A)\n(not (r B))\n(relation s T :why \"s holds of {?1}\")\n(s B)",
        );
        let names: Vec<&str> = candidate_objects(&kb, &terms, &ast)
            .iter()
            .map(|&s| terms.sym(s))
            .collect();
        assert_eq!(names, ["A", "B"], "sorted by name, T and `not` excluded");
    }

    /// **A kernel dunder is never an object**, however the program comes to
    /// hold it: `__closed__` written as a rule's relation parameter and
    /// `__symmetric__` as an ordinary argument, each a name of the program.
    /// Neither is proposed about; `a`, beside them, is.
    #[test]
    fn candidate_objects_never_hold_a_kernel_dunder() {
        let (ast, terms, kb) = kb_of(
            "(relation thing T :why \"{?1} is a thing\")\n(thing a)\n\
             (rule lift (?P ?Q) :match (?P ?r) :assert (?Q ?r) :why \"{?r} is {?P}, so it is {?Q}\")\n\
             (lift complete __closed__)\n(complete thing)\n(marker __symmetric__)",
        );
        let written: Vec<&str> = crate::invariant::program_names(&kb, &terms, &ast)
            .iter()
            .map(|n| terms.sym(n.name))
            .collect();
        assert!(
            written.contains(&"__closed__") && written.contains(&"__symmetric__"),
            "both are names of the program: {written:?}"
        );
        let names: Vec<&str> = candidate_objects(&kb, &terms, &ast)
            .iter()
            .map(|&s| terms.sym(s))
            .collect();
        assert_eq!(names, ["a"], "neither dunder is an object");
    }

    /// **The seed** (M1e2 S1e2.2.4, Q-M1e.21 (b)): a constant only a rule's
    /// `:assert` names is an object of the program before any fact names it,
    /// and a constant the rule names that a fact already does is listed once.
    /// Without the seed this is `["A", "B"]` and nothing is ever proposed
    /// about `Z`.
    #[test]
    fn candidate_objects_seed_the_constants_a_rule_asserts() {
        let (ast, terms, kb) = kb_of(
            "(relation q T T :why \"q holds of {?1} and {?2}\")\n(q A B)\n\
             (rule spawn () :match (q ?a ?b) :assert (and (q ?b Z) (q A ?a)) :why \"q holds of {?a} and {?b}, so q holds of {?b} and Z, and of A and {?a}\")",
        );
        let names: Vec<&str> = candidate_objects(&kb, &terms, &ast)
            .iter()
            .map(|&s| terms.sym(s))
            .collect();
        assert_eq!(names, ["A", "B", "Z"], "Z seeded, A once, T excluded");
    }

    /// **Every name of the program** (M1e2 S1e2.2.14, the user's ruling of
    /// 2026-11-04 on Q-M1e2.9, (a)): a constant only a `:match` names (`M`),
    /// and one only a nested fact's argument names (`N`, inside the stored
    /// negative), are objects of the program; a constant only a `(query …)`
    /// names (`Q`) is not, because R is the program's and a query asks about
    /// it. A nested fact's head (`q` inside `(not (q N A))`) is a relation,
    /// not an argument. Before the ruling this was `["A", "B"]`.
    #[test]
    fn candidate_objects_hold_every_name_the_program_writes() {
        let (ast, terms, kb) = kb_of(
            "(relation q T T :why \"q holds of {?1} and {?2}\")\n(q A B)\n(not (q N A))\n\
             (rule reach () :match (q ?a M) :assert (q ?a B) :why \"q holds of {?a} and M, so q holds of {?a} and B\")\n\
             (query :goal (q Q ?x))",
        );
        let names: Vec<&str> = candidate_objects(&kb, &terms, &ast)
            .iter()
            .map(|&s| terms.sym(s))
            .collect();
        assert_eq!(
            names,
            ["A", "B", "M", "N"],
            "M (a :match) and N (a nested fact) held, Q (a query) not"
        );
    }
}
