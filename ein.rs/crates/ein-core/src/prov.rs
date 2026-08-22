//! Provenance records — where a fact came from.
//!
//! Provenance is what makes Ein's answers explainable, and since S1.21.7 a
//! fact is an **OR-node** over its recorded derivations, each of which is an
//! AND-node over its premises. The records themselves live here; which
//! records a given KB has recorded is a per-KB table in [`crate::kb`], and the
//! policy that decides what may be recorded is
//! [`crate::kb::Kb::record_justification`]
//! ([design/03](../../../../plans/m1a_rust/design/03_data_model.md) §7).
//!
//! **The arena is global, like the fact store.** design/03 §5 sketches it
//! inside `KbCore`; putting it beside the other interned tables instead is
//! the same trade the fact store makes — creating a record says nothing about
//! whether any KB has recorded it, so a fork may build one without the parent
//! seeing it, and a `ProvId` means the same thing in every branch. What ein.py
//! copies per fork (and must: a justification recorded inside a hypothesis
//! fork can name premises root never assumed) is the *table*, and that stays
//! per-KB. The cost is that a dead fork's records are not reclaimed until the
//! run ends; the `accepts_justification` pre-check is what keeps that bounded,
//! because it is why the saturator's hot path never builds one.

use crate::entities::Loc;
use crate::facts::FactId;
use crate::intern::Symbol;
use crate::value::Value;

/// Index into [`ProvArena`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ProvId(pub u32);

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
    /// the trace ([design/02](../../../../plans/m1a_rust/design/02_determinism_and_order.md) §2).
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

/// The append-only record arena.
#[derive(Default, Debug)]
pub struct ProvArena {
    records: Vec<Prov>,
    /// Ids a caller has declared finished with — see [`ProvArena::retire`].
    /// Debug builds only: it is an assertion's evidence, not state the engine
    /// reads.
    #[cfg(debug_assertions)]
    retired: crate::bitset::BitSet,
}

impl ProvArena {
    pub fn new() -> Self {
        Self::default()
    }

    /// Declare the records in `range` finished with — nothing may read one
    /// again.
    ///
    /// The caller is the search layer, and what it declares is that a fork's
    /// own derivation records die with the fork. That is the claim
    /// [T1a.7.1.7](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.1_sync_shared_state.md#task-t1a717--the-provenance-arena)
    /// rests on, and it is why an arena a worker writes need not be shared at
    /// all: `features/01 -e` pushes **2 135 093** records and root references
    /// **none** of them when the solve ends
    /// ([shared_state.md §2b](../../../../plans/m1a_rust/p1a.7_parallelism/shared_state.md)).
    ///
    /// **Nothing is freed.** Retiring is an assertion, not a deallocation: in
    /// release it compiles to nothing at all, and in debug it arms
    /// [`ProvArena::get`] to panic on a read that should be impossible. Making
    /// it *actually* reclaim is the change this claim licenses, not one this
    /// method performs.
    pub fn retire(&mut self, range: std::ops::Range<usize>) {
        #[cfg(debug_assertions)]
        for id in range.clone() {
            self.retired.insert(id as u32);
        }
        let _ = range;
    }

    /// No hash-consing: two firings with the same premises produce two
    /// records, and the dedup that matters happens in
    /// `record_justification`, where ein.py does it too.
    pub fn push(&mut self, prov: Prov) -> ProvId {
        crate::counters::bump(|c| c.prov_push += 1);
        let id = ProvId(self.records.len() as u32);
        self.records.push(prov);
        id
    }

    pub fn get(&self, id: ProvId) -> &Prov {
        crate::counters::bump(|c| c.prov_read += 1);
        #[cfg(debug_assertions)]
        assert!(
            !self.retired.contains(id.0),
            "provenance record {} was read after the fork that created it ended — \
             see ProvArena::retire",
            id.0
        );
        &self.records[id.0 as usize]
    }

    /// Every record, **retired ones included**, for a consumer that is
    /// *scanning* the arena rather than following a reference to one.
    ///
    /// The distinction is the whole of what [`ProvArena::retire`] measures,
    /// and it is not academic: T1a.7.1.7 armed the assertion in
    /// [`ProvArena::get`] over the whole gate and found **exactly one**
    /// reader of a retired record — `ein-einb`'s writer, which walks the
    /// arena end to end. So "no live structure references a dead fork's
    /// record" is true, and "nothing reads one" is not, and a consumer that
    /// scans says so here rather than by tripping an assertion.
    ///
    /// That the scan exists is itself a finding: `.einb` writes the arena in
    /// full, so a saved KB carries every record a search left behind —
    /// 2 135 093 of them on `features/01 -e`, of which twelve are live.
    pub fn scan(&self) -> impl ExactSizeIterator<Item = &Prov> {
        self.records.iter()
    }

    /// Has `id` been retired? Debug builds only; `false` everywhere else.
    ///
    /// [`ProvArena::get`]'s assertion answers "did anything *read* a dead
    /// fork's record". This answers the stronger question a reclamation would
    /// need — "does anything still *hold* one" — which a read-side assertion
    /// cannot, because an id that is stored and never read trips nothing and
    /// would still be corrupted by reuse. `ein-infer/tests/provenance.rs`
    /// asks it of every corpus file that solves.
    pub fn is_retired(&self, id: ProvId) -> bool {
        #[cfg(debug_assertions)]
        {
            self.retired.contains(id.0)
        }
        #[cfg(not(debug_assertions))]
        {
            let _ = id;
            false
        }
    }

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
