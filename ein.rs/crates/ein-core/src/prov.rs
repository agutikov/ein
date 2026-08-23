//! Provenance records — where a fact came from.
//!
//! Provenance is what makes Ein's answers explainable, and since S1.21.7 a
//! fact is an **OR-node** over its recorded derivations, each of which is an
//! AND-node over its premises. The records themselves live here; which
//! records a given KB has recorded is a per-KB table in [`crate::kb`], and the
//! policy that decides what may be recorded is
//! [`crate::kb::Kb::record_justification`]
//! ([design/03](../../../../docs/history/m1a_rust/design/03_data_model.md) §7).
//!
//! **The arena is global, like the fact store** — all but one region of it.
//! design/03 §5 sketches it inside `KbCore`; putting it beside the other
//! interned tables instead is the same trade the fact store makes — creating a
//! record says nothing about whether any KB has recorded it, so a fork may
//! build one without the parent seeing it, and a `ProvId` means the same thing
//! in every branch. What ein.py copies per fork (and must: a justification
//! recorded inside a hypothesis fork can name premises root never assumed) is
//! the *table*, and that stays per-KB.
//!
//! ## The fork region
//!
//! Records used to accumulate for the whole run. That cost `features/01 -e`
//! **2 135 093** of them — 205 MB — to keep the twelve anything still pointed
//! at, and it made the arena the one shared structure on a worker's write path
//! that
//! [design/08 §6](../../../../docs/history/m1a_rust/design/08_parallelism.md#6-what-must-be-sync-and-how)
//! has no row for. They no longer do. The search opens a **fork region** around
//! each entering ([`ProvArena::open_fork`]); every record the fork derives
//! lands there; and the region is discarded when the fork is
//! ([`ProvArena::discard_fork`]) — except on the one path that keeps a fork
//! alive, where [`crate::kb::Kb::promote_provenance`] copies what the solution
//! still cites into the arena proper first.
//!
//! Two properties make that safe, and both are asserted rather than assumed
//! ([T1a.7.1.7](../../../../docs/history/m1a_rust/README.md#s1a71--making-the-shared-state-sync)):
//!
//! - **A fork's records die with the fork.** Measured — ≥ 33.7 % of all pushes
//!   happen inside one, and four of the six workloads reference *none* of them
//!   when the solve ends. `ein-infer/tests/provenance.rs` asks the holding-side
//!   question over every corpus file that solves: is any live justification,
//!   root's or a recorded solution's, a fork's record?
//! - **A stale id cannot alias a live one.** The region's ids come from a
//!   **monotone** sequence that [`ProvArena::discard_fork`] advances, so an id
//!   from a finished fork falls below the live region's base and
//!   [`ProvArena::get`] panics on it — in *every* build, not only where
//!   `debug_assertions` are on, because reuse is what a read-side check in one
//!   profile would fail to cover.
//!
//! What keeps the arena proper bounded is unchanged: the
//! `accepts_justification` pre-check is why the saturator's hot path never
//! builds a record at all.

use crate::entities::Loc;
use crate::facts::FactId;
use crate::intern::Symbol;
use crate::value::Value;
use std::sync::Arc;

/// Index into [`ProvArena`].
///
/// Bit 31 is the **fork tag**: set, the remaining 31 bits are a position in
/// the arena's monotone fork sequence; clear, they index the arena proper.
/// Either space is 2 147 483 648 records deep, which is a thousandfold the
/// largest workload M1a has (`features/01 -e`, 2 135 093).
///
/// The tag preserves the ordering the untagged ids had, because a fork's
/// records are pushed after every record that existed when it opened: a fork
/// id still compares above every root id it could be compared with.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ProvId(pub u32);

impl ProvId {
    /// The tag bit — private, because outside this module a `ProvId` is opaque
    /// but for [`ProvId::is_fork`].
    const FORK: u32 = 1 << 31;

    /// Was this record derived inside a fork region, and does it therefore die
    /// with the fork that created it?
    ///
    /// The predicate `ein-infer/tests/provenance.rs` states the arena's
    /// central claim with: nothing a finished solve believes may cite one.
    pub fn is_fork(self) -> bool {
        self.0 & ProvId::FORK != 0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProvKind {
    /// Ingested from the IR, with or without a `:source` annotation.
    Source,
    /// Derived by a rule firing.
    Rule,
    /// A speculative branch introduction.
    Hypothesis,
    /// A hypothesis that was contradicted; kept for the trace and for the
    /// *contradictions* task class.
    Rejected,
}

impl ProvKind {
    /// The spelling ein.py stores in `Provenance.kind`.
    pub fn as_str(self) -> &'static str {
        match self {
            ProvKind::Source => "source",
            ProvKind::Rule => "rule",
            ProvKind::Hypothesis => "hypothesis",
            ProvKind::Rejected => "rejected",
        }
    }
}

/// A negative premise — an `(absent …)` query that had to fail on the
/// closure/world boundary for a firing to be admitted (S1.21.8).
///
/// The relation queried and the argument pattern it was queried with, with
/// [`NafArg::Free`] where the query ranged free. It is the missing half of
/// `Deps(Y)`: positive provenance records the facts a firing consumed, and
/// without this a firing's dependence on an *absence* is invisible to every
/// provenance walk.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NafRef {
    pub rel: Symbol,
    pub args: Box<[NafArg]>,
}

/// One argument position of a [`NafRef`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NafArg {
    /// The query ranged free here — ein.py's `None`.
    Free,
    /// A name or a number: a bound variable, an `Atom`'s name, an `Int`.
    Value(Value),
    /// A nested pattern, which `world._ground` renders as a
    /// `(relation, (args…))` tuple — so it can be partly free, and cannot
    /// collapse into a [`Value`].
    Nested { rel: Symbol, args: Box<[NafArg]> },
}

/// One derivation record.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Prov {
    pub kind: ProvKind,
    /// The `:source` sentence id, for `Source` kind. `None` is an
    /// unannotated background assumption — neither given nor derived.
    pub source: Option<Symbol>,
    pub rule: Option<Symbol>,
    /// Positive premises, in plan-step order.
    pub premises: Box<[FactId]>,
    /// Variable bindings in **bind order**, which is the order they land in
    /// the trace ([design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md) §2).
    ///
    /// ein.py stringifies these at record time (`(k, str(v))`); keeping the
    /// `Value` and rendering at display time is the same information with the
    /// rendering decision left where it belongs — `Terms::display` is
    /// `str(v)` for each of the three shapes.
    pub bindings: Box<[(Symbol, Value)]>,
    /// S1.21.8's negative premises. **Recorded, not yet interpreted** by any
    /// walk: making a walk honour them is a semantics change that
    /// `absent_semantics.md` explicitly leaves open.
    pub absent: Box<[NafRef]>,
    pub branch: Option<u32>,
    pub loc: Option<Loc>,
}

impl Prov {
    pub fn from_source(source: Option<Symbol>, loc: Option<Loc>) -> Prov {
        Prov {
            kind: ProvKind::Source,
            source,
            loc,
            ..Prov::empty()
        }
    }

    pub fn from_rule(rule: Symbol, premises: Box<[FactId]>, loc: Option<Loc>) -> Prov {
        Prov {
            kind: ProvKind::Rule,
            rule: Some(rule),
            premises,
            loc,
            ..Prov::empty()
        }
    }

    pub fn from_hypothesis(branch: u32, loc: Option<Loc>) -> Prov {
        Prov {
            kind: ProvKind::Hypothesis,
            branch: Some(branch),
            loc,
            ..Prov::empty()
        }
    }

    pub fn rejected(branch: u32, loc: Option<Loc>) -> Prov {
        Prov {
            kind: ProvKind::Rejected,
            branch: Some(branch),
            loc,
            ..Prov::empty()
        }
    }

    fn empty() -> Prov {
        Prov {
            kind: ProvKind::Source,
            source: None,
            rule: None,
            premises: Box::new([]),
            bindings: Box::new([]),
            absent: Box::new([]),
            branch: None,
            loc: None,
        }
    }

    /// Is this a *terminal* — something a derivation walk grounds out on?
    ///
    /// `source` and `hypothesis` are assumptions; a rule-kind record with no
    /// premises is a synthetic engine writeback (`<forced-positive>`,
    /// `<monotonic-unconditional>`, `<lookahead-dies-immediately>`) whose
    /// stated contract is exactly that walks stop there.
    pub fn is_terminal(&self) -> bool {
        self.kind != ProvKind::Rule || self.premises.is_empty()
    }

    /// The AND-node identity of a justification: `(rule, premises)`.
    /// `bindings` is display metadata and is excluded, so two firings that
    /// consumed the same premises collapse.
    pub fn same_justification(&self, other: &Prov) -> bool {
        self.rule == other.rule && self.premises == other.premises
    }
}

/// One entering's records, and the sequence number its ids are relative to.
///
/// A region travels with the entering that produced it: a worker builds one,
/// hands it back with its result, and the ordered commit installs it while
/// that result is being read
/// ([T1a.7.2.1](../../../../docs/history/m1a_rust/README.md#s1a72--level-1-parallel-enterings)).
/// Keeping the base *with* the records is what makes that safe: an id means
/// something only against the region it was issued from, and the two cannot be
/// separated.
#[derive(Default, Debug)]
pub struct Region {
    records: Vec<Prov>,
    /// The fork sequence number of `records[0]`.
    base: u32,
}

impl Region {
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
}

/// The append-only record arena, plus the region of it that is not.
///
/// `records` is the arena proper: pushed to at load, at root saturation and at
/// every commit step, and never reclaimed. It is shared by `Arc` for the same
/// reason the fact store is — a fork reads root's derivations when it explains
/// a contradiction — and grown only where nothing else holds it. `fork` is the
/// region the search opens around one entering; see the module note.
#[derive(Default, Debug)]
pub struct ProvArena {
    records: Arc<Vec<Prov>>,
    /// The fork in hand's own records, and the sequence they are numbered
    /// from. Cleared by [`ProvArena::discard_fork`], which is 100 % of
    /// enterings on four of the six measured workloads and 99.9 % on the other
    /// two.
    ///
    /// The base is **monotone across enterings**: discarding a region advances
    /// it past the ids that region issued, so a stale id is below the base and
    /// cannot address a live record.
    fork: Region,
    /// How many records [`ProvArena::promote`] has copied out of a region,
    /// for the whole run.
    ///
    /// A plain counter rather than a `counters.rs` one, because it is the
    /// evidence that the promoting path *runs*: `ein-infer/tests/provenance.rs`
    /// asserts that no live fact cites a fork's record, and a promotion that
    /// silently never happened would satisfy that assertion for the wrong
    /// reason.
    promoted: u32,
    /// Does [`ProvArena::push`] belong to the fork in hand?
    ///
    /// Separate from the region being *present*, which is why closing and
    /// discarding are two verbs: `handle_dead` writes root's own no-good and
    /// `(not h)` writeback *after* the fork is over but *before* the dumper
    /// has rendered the fork's justifications, so routing has to stop one step
    /// earlier than reclamation does.
    forking: bool,
}

impl ProvArena {
    pub fn new() -> Self {
        Self::default()
    }

    // ── The fork region ────────────────────────────────────────────

    /// Route what follows to the fork region: the caller is about to enter a
    /// commitment, and everything the fork derives dies with it.
    ///
    /// The caller is the search layer, and the scope is one iteration of
    /// `Run::phase2`'s candidate loop — which covers the nested
    /// `try_commitment_set` calls a `-y` commutativity check or a `spec-audit`
    /// build makes, because those are the fork's own speculation and die with
    /// it too.
    pub fn open_fork(&mut self) {
        // Deliberately not an assertion that no region is open. One way to
        // arrive here with one is a `?` unwound past the discard, and that run
        // is over anyway; the other is an unbalanced open, which discarding
        // makes *loud* — the records the caller is still holding ids for stop
        // resolving, and `get` says so. Joining the two regions would be the
        // quiet failure.
        self.discard_fork();
        self.forking = true;
    }

    /// Stop routing to the region, keeping it readable.
    ///
    /// The gap between this and [`ProvArena::discard_fork`] is where
    /// `handle_dead` runs: its writes are root's and must land in the arena
    /// proper, while the dumper it calls still renders the fork's own
    /// justifications.
    pub fn close_fork(&mut self) {
        self.forking = false;
    }

    /// Reclaim the region. Every id it issued is dead from here.
    pub fn discard_fork(&mut self) {
        self.forking = false;
        // Monotone, so the ids just freed can never be issued again — which is
        // what makes [`ProvArena::get`] able to reject one.
        self.fork.base = self
            .fork
            .base
            .checked_add(self.fork.records.len() as u32)
            .filter(|&n| n < ProvId::FORK)
            .expect("the fork sequence overflowed 2^31 records");
        self.fork.records.clear();
    }

    /// Hand the region in hand to its caller, leaving the arena with an empty
    /// one numbered from where this one ended.
    ///
    /// A worker calls this to send its records back with its result; the base
    /// travels with them, so the ids in the result's KB keep meaning what they
    /// meant. Numbering the *next* region from the end of this one is the same
    /// monotone step [`ProvArena::discard_fork`] takes, which is what keeps a
    /// worker's own stale ids from resolving against its next entering.
    pub fn take_fork(&mut self) -> Region {
        self.forking = false;
        let base = self
            .fork
            .base
            .checked_add(self.fork.records.len() as u32)
            .filter(|&n| n < ProvId::FORK)
            .expect("the fork sequence overflowed 2^31 records");
        std::mem::replace(
            &mut self.fork,
            Region {
                records: Vec::new(),
                base,
            },
        )
    }

    /// Install a region handed back by a worker, returning the one it
    /// displaces.
    ///
    /// The caller is the ordered commit, and the pairing is strict: install,
    /// read the result the region belongs to, then put back what came out.
    /// Nothing else may read a fork id in between, which is why the region
    /// lives on the result rather than in a side table — there is no way to
    /// install one result's records and read another's.
    pub fn swap_fork(&mut self, region: Region) -> Region {
        std::mem::replace(&mut self.fork, region)
    }

    /// A worker's arena: the same records by `Arc`, and a region of its own
    /// numbered from zero.
    ///
    /// The base can start over because a region only ever means anything
    /// against itself — [`ProvArena::swap_fork`] carries the two together, and
    /// nothing reads a worker's ids except through the result that owns them.
    pub fn share(&self) -> ProvArena {
        ProvArena {
            records: Arc::clone(&self.records),
            fork: Region::default(),
            promoted: 0,
            forking: false,
        }
    }

    /// Copy the cited fork records into the arena proper, in the fork's own
    /// push order, and say where each landed.
    ///
    /// Push order rather than the caller's iteration order because a KB's
    /// justification tables are hash maps: which ids a promotion assigns has
    /// to be a function of what the fork derived, not of where a `FactId`
    /// happened to hash — that is
    /// [design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md)
    /// §3's rule, and `id_order_invariance` is the instrument that would find
    /// it broken.
    ///
    /// The one caller is [`crate::kb::Kb::promote_provenance`].
    pub(crate) fn promote(
        &mut self,
        cited: &rustc_hash::FxHashSet<ProvId>,
    ) -> rustc_hash::FxHashMap<ProvId, ProvId> {
        let mut map =
            rustc_hash::FxHashMap::with_capacity_and_hasher(cited.len(), Default::default());
        let records = Arc::get_mut(&mut self.records)
            .expect("a worker may not promote out of a region — see ProvArena::share");
        for (i, record) in self.fork.records.iter().enumerate() {
            let old = ProvId(ProvId::FORK | (self.fork.base + i as u32));
            if !cited.contains(&old) {
                continue;
            }
            let new = ProvId(records.len() as u32);
            debug_assert!(!new.is_fork(), "the arena overflowed 2^31 records");
            records.push(record.clone());
            self.promoted += 1;
            map.insert(old, new);
        }
        map
    }

    /// How many records this run has promoted out of a fork region — see the
    /// field.
    pub fn promoted(&self) -> u32 {
        self.promoted
    }

    /// Is the fork region empty — is there nothing a promotion could move?
    pub fn fork_is_empty(&self) -> bool {
        self.fork.records.is_empty()
    }

    // ── Records ────────────────────────────────────────────────────

    /// No hash-consing: two firings with the same premises produce two
    /// records, and the dedup that matters happens in
    /// `record_justification`, where ein.py does it too.
    pub fn push(&mut self, prov: Prov) -> ProvId {
        crate::counters::bump(|c| c.prov_push += 1);
        if self.forking {
            let id = ProvId(ProvId::FORK | (self.fork.base + self.fork.records.len() as u32));
            self.fork.records.push(prov);
            return id;
        }
        // The arena proper. A worker never reaches here — its region stays
        // open for the whole entering, and root's own writes (the no-good, the
        // `(not h)` writeback, the forced positive) happen on the committing
        // thread after the region has closed.
        let records = Arc::get_mut(&mut self.records)
            .expect("a worker may not push to the arena proper — see ProvArena::share");
        let id = ProvId(records.len() as u32);
        debug_assert!(!id.is_fork(), "the arena overflowed 2^31 records");
        records.push(prov);
        id
    }

    pub fn get(&self, id: ProvId) -> &Prov {
        crate::counters::bump(|c| c.prov_read += 1);
        if id.is_fork() {
            let seq = id.0 & !ProvId::FORK;
            return match seq
                .checked_sub(self.fork.base)
                .and_then(|i| self.fork.records.get(i as usize))
            {
                Some(record) => record,
                None => panic!(
                    "provenance record {seq} was read after the fork that created it ended — \
                     see ProvArena::open_fork"
                ),
            };
        }
        &self.records[id.0 as usize]
    }

    /// Every record of the arena proper, **including ones no believed fact
    /// points at**, for a consumer that is *scanning* rather than following a
    /// reference.
    ///
    /// The distinction is not academic, and it is how the fork region was
    /// found: arming a read-side assertion over the whole gate turned up
    /// exactly one reader of a record whose fork had ended, and it was
    /// `ein-einb`'s writer walking the arena end to end. A consumer that scans
    /// says so here rather than by tripping an assertion.
    ///
    /// The region is deliberately *not* scanned. `.einb` is written between
    /// enterings, never inside one, so a file that carried a fork's records
    /// would be carrying records whose ids no longer mean anything — which is
    /// also why the writer asserts that no id it stores is a fork's.
    pub fn scan(&self) -> impl ExactSizeIterator<Item = &Prov> {
        self.records.iter()
    }

    /// How many records the arena proper holds — what `.einb` writes, and the
    /// number a fork region no longer contributes to.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_writeback_is_terminal_and_a_real_firing_is_not() {
        let writeback = Prov::from_rule(Symbol(0), Box::new([]), None);
        let firing = Prov::from_rule(Symbol(0), Box::new([FactId(1)]), None);
        assert!(writeback.is_terminal());
        assert!(!firing.is_terminal());
        assert!(Prov::from_source(None, None).is_terminal());
        assert!(Prov::from_hypothesis(3, None).is_terminal());
    }

    /// The region's whole point: an entering's records leave the arena the
    /// size they found it.
    #[test]
    fn a_forks_records_do_not_reach_the_arena() {
        let mut a = ProvArena::new();
        let root = a.push(Prov::from_source(None, None));
        assert!(!root.is_fork());
        a.open_fork();
        let inside = a.push(Prov::from_rule(Symbol(1), Box::new([FactId(0)]), None));
        assert!(inside.is_fork(), "a push inside the region is tagged");
        assert_eq!(a.get(inside).rule, Some(Symbol(1)), "and readable");
        assert_eq!(a.len(), 1, "while the arena proper has not grown");
        a.discard_fork();
        assert_eq!(a.len(), 1);
        assert!(a.fork_is_empty());
    }

    /// `handle_dead`'s shape: routing stops, the records stay readable, and
    /// what root writes in between lands in the arena proper.
    #[test]
    fn closing_the_region_stops_the_routing_and_not_the_reads() {
        let mut a = ProvArena::new();
        a.open_fork();
        let inside = a.push(Prov::from_hypothesis(0, None));
        a.close_fork();
        let after = a.push(Prov::from_rule(Symbol(7), Box::new([]), None));
        assert!(!after.is_fork(), "root's writeback is root's");
        assert_eq!(a.get(inside).kind, ProvKind::Hypothesis);
        a.discard_fork();
        assert_eq!(a.get(after).rule, Some(Symbol(7)));
    }

    /// Reuse is what a read-side assertion cannot cover, so the base is
    /// monotone and the id of a finished fork addresses nothing.
    #[test]
    #[should_panic(expected = "was read after the fork that created it ended")]
    fn a_stale_fork_id_is_not_the_next_forks_record() {
        let mut a = ProvArena::new();
        a.open_fork();
        let stale = a.push(Prov::from_hypothesis(0, None));
        a.discard_fork();
        a.open_fork();
        let _fresh = a.push(Prov::from_hypothesis(1, None));
        a.get(stale);
    }

    /// A solution keeps its fork. What it cites is copied out in the fork's
    /// own push order — not in the order the caller collected the citations,
    /// which comes off a hash map.
    #[test]
    fn promotion_follows_the_forks_push_order() {
        let mut a = ProvArena::new();
        a.open_fork();
        let first = a.push(Prov::from_rule(Symbol(1), Box::new([]), None));
        let unused = a.push(Prov::from_rule(Symbol(2), Box::new([]), None));
        let third = a.push(Prov::from_rule(Symbol(3), Box::new([]), None));
        a.close_fork();
        let cited: rustc_hash::FxHashSet<ProvId> = [third, first].into_iter().collect();
        let map = a.promote(&cited);
        a.discard_fork();
        assert_eq!(map.len(), 2);
        assert!(
            !map.contains_key(&unused),
            "an uncited record is not copied"
        );
        let (p1, p3) = (map[&first], map[&third]);
        assert!(p1 < p3, "push order, not citation order");
        assert_eq!(a.get(p1).rule, Some(Symbol(1)));
        assert_eq!(a.get(p3).rule, Some(Symbol(3)));
        assert_eq!(a.len(), 2, "and only the cited two");
    }

    #[test]
    fn justification_identity_ignores_bindings() {
        let mut a = Prov::from_rule(Symbol(1), Box::new([FactId(2), FactId(3)]), None);
        let b = Prov::from_rule(Symbol(1), Box::new([FactId(2), FactId(3)]), None);
        a.bindings = Box::new([(Symbol(9), Value::sym(Symbol(4)))]);
        assert!(a.same_justification(&b));
        let reordered = Prov::from_rule(Symbol(1), Box::new([FactId(3), FactId(2)]), None);
        assert!(
            !a.same_justification(&reordered),
            "premise order is identity"
        );
    }
}
