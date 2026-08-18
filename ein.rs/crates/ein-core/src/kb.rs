//! The knowledge base — a stack of immutable layers and one writable top.
//!
//! ein.py's `fork()` shallow-copies the fact list and six index dicts, and
//! `snapshot()` does the same plus a `_nogoods` copy. That is cheap today
//! (0.003 s over 206 calls on an exhaustive `zebra2`) because there are only
//! 101 enterings — but [P1a.7](../../../../plans/m1a_rust/p1a.7_parallelism/README.md)
//! wants hundreds live at once, and [design/05](../../../../plans/m1a_rust/design/05_matcher.md)'s
//! beta-memories are only affordable if a fork does not copy them, which is
//! the exact objection [F11](../../../../plans/followups/f11_deductive_layer_perf.md)
//! parks them on.
//!
//! So a `Kb` is `Vec<Arc<Layer>>` plus a writable [`Layer`]
//! ([design/03](../../../../plans/m1a_rust/design/03_data_model.md) §5):
//!
//! - **Read** = walk the layers oldest-first. For every ordered list that
//!   means *concatenated* iteration, which is exactly the order "copy the
//!   list, then append" produces.
//! - **Write** = append to the top layer only. A sealed layer is never
//!   mutated, so it can be shared by `Arc` with no lock.
//! - **`fork()`** = seal the top, clone the `Vec` of `Arc`s, start a fresh
//!   top. Allocation count is independent of `|facts|`.
//!
//! ### Why this is trivially correct
//!
//! The KB is **append-only within a run** — the property S1.9.E23's fail-fast
//! and S1.21.8's monotone-growth argument already lean on. A layer that only
//! adds, over layers that never change, cannot disagree with a copy that was
//! mutated in place. [`Kb::materialise`] is that copy, and the tests assert
//! the two agree.

use crate::bitset::BitSet;
use crate::entities::{NameCategory, Registry};
use crate::facts::FactId;
use crate::intern::{Overflow, Symbol};
use crate::program::Program;
use crate::prov::ProvId;
use crate::terms::Terms;
use crate::value::{Tag, Value};
use rustc_hash::{FxHashMap, FxHashSet};
use std::sync::{Arc, RwLock};

/// S1.21.7 — the per-fact cap on recorded **alternative** justifications.
///
/// A hot fact can be re-derived hundreds of times (an exhaustive `zebra2`
/// makes ~194 k redundant firings) and keeping every one costs memory for
/// environments that repeat. The list is kept sorted by premise count, so the
/// cap retains the shortest — the ones a minimum-cardinality explanation
/// search can actually use.
pub const MAX_ALT_JUSTIFICATIONS: usize = 32;

/// A key of the participation index (S1.8.B-idx): relation, argument
/// position, and the value in it.
///
/// 12 bytes with padding. design/03 §6 notes that packing it into a `u64`
/// does not fit — the `Value` needs 32 bits and the symbol another 32 — so
/// this stays a struct key and P1a.6 measures whether hashing the triple is
/// worth the collision check.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SlotKey {
    pub rel: Symbol,
    pub slot: u16,
    pub value: Value,
}

/// One name's participation: every fact it heads, and every fact it appears
/// in as a direct argument.
///
/// Nested-fact args are **not** counted (Q40): the nested fact is its own
/// entry, so its arguments show up through that entry's `as_arg`.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct NameEntry {
    pub as_head: Vec<FactId>,
    pub as_arg: Vec<FactId>,
}

/// Facts added at one point in a KB's history, with the indexes over them.
///
/// Sealed layers are shared by `Arc`; the top layer is the only writable one.
#[derive(Clone, Default, Debug)]
pub struct Layer {
    facts: Vec<FactId>,
    present: BitSet,
    by_rel: FxHashMap<Symbol, Vec<FactId>>,
    by_rel_slot_val: FxHashMap<SlotKey, Vec<FactId>>,
    /// The **inner** id of every `(not X)` — a bitset, where ein.py keeps a
    /// `set` of `(relation, args)` tuples.
    negated: BitSet,
    rule_apps_by_rule: FxHashMap<Symbol, Vec<FactId>>,
    rule_apps_on_rel: FxHashMap<Symbol, Vec<FactId>>,
    names: Registry<NameEntry>,
    /// The first-recorded justification of each fact this layer added.
    primary: FxHashMap<FactId, ProvId>,
    /// Alternative justifications, whole-list copy-on-write: a layer that
    /// touches a fact's list holds the *complete* list, so a read takes the
    /// topmost layer that has the key. The list is not append-only — an
    /// arrival can land in the middle and the cap evicts from the end — so
    /// concatenation would be wrong here where it is right everywhere else.
    alts: FxHashMap<FactId, Box<[ProvId]>>,
}

impl Layer {
    fn is_empty(&self) -> bool {
        self.facts.is_empty() && self.alts.is_empty()
    }

    /// Field-by-field comparison, naming the first disagreement.
    ///
    /// `names` is compared **as a set** rather than in order, and that is not
    /// laziness: ein.py builds its `names` dict by comprehension over a *set*
    /// union, so its order is hash-dependent and not reproducible even
    /// run-to-run — which is why every consumer sorts (design/02 §2). Every
    /// other list here is append-ordered and is compared exactly.
    fn diff(&self, other: &Layer) -> Result<(), String> {
        if self.facts != other.facts {
            return Err(format!("fact order: {:?} vs {:?}", self.facts, other.facts));
        }
        if self.present != other.present {
            return Err("belief sets differ".to_string());
        }
        if self.negated != other.negated {
            return Err("negated sets differ".to_string());
        }
        diff_map("by_rel", &self.by_rel, &other.by_rel)?;
        diff_map(
            "by_rel_slot_val",
            &self.by_rel_slot_val,
            &other.by_rel_slot_val,
        )?;
        diff_map(
            "rule_apps_by_rule",
            &self.rule_apps_by_rule,
            &other.rule_apps_by_rule,
        )?;
        diff_map(
            "rule_apps_on_rel",
            &self.rule_apps_on_rel,
            &other.rule_apps_on_rel,
        )?;
        let mut mine: Vec<(Symbol, &NameEntry)> = self.names.iter().collect();
        let mut theirs: Vec<(Symbol, &NameEntry)> = other.names.iter().collect();
        mine.sort_by_key(|(s, _)| s.0);
        theirs.sort_by_key(|(s, _)| s.0);
        if mine != theirs {
            return Err("names differ".to_string());
        }
        if self.primary != other.primary {
            return Err("primary provenance differs".to_string());
        }
        if self.alts != other.alts {
            return Err("alternative justifications differ".to_string());
        }
        Ok(())
    }
}

fn diff_map<K: Eq + std::hash::Hash + std::fmt::Debug>(
    what: &str,
    a: &FxHashMap<K, Vec<FactId>>,
    b: &FxHashMap<K, Vec<FactId>>,
) -> Result<(), String> {
    for (k, v) in a {
        match b.get(k) {
            Some(w) if w == v => {}
            Some(w) => return Err(format!("{what}[{k:?}]: {v:?} vs {w:?}")),
            None => return Err(format!("{what}[{k:?}]: {v:?} vs absent")),
        }
    }
    for k in b.keys() {
        if !a.contains_key(k) {
            return Err(format!("{what}[{k:?}]: absent vs present"));
        }
    }
    Ok(())
}

/// Learned no-good clauses (S1.5a.18) — path conditions that are known dead.
///
/// A clause is a sorted `Box<[FactId]>`; the meaning is "any branch whose path
/// condition is a superset of this clause is dead". Emission and subsumption
/// live with the search layer.
#[derive(Clone, Default, Debug)]
pub struct Nogoods {
    clauses: FxHashSet<Box<[FactId]>>,
}

impl Nogoods {
    pub fn len(&self) -> usize {
        self.clauses.len()
    }

    pub fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }

    pub fn contains(&self, clause: &[FactId]) -> bool {
        self.clauses.contains(clause)
    }

    pub fn insert(&mut self, clause: Box<[FactId]>) -> bool {
        self.clauses.insert(clause)
    }

    pub fn iter(&self) -> impl Iterator<Item = &[FactId]> {
        self.clauses.iter().map(|c| &**c)
    }
}

/// A union-find over names — the M1 placeholder, reserved for F4's e-graph.
///
/// The engine fires no equality propagation, so this stays inert; it is
/// ported because `fork()` / `snapshot()` copy it and the copy is observable
/// through [`EqClasses::classes`].
#[derive(Clone, Default, Debug)]
pub struct EqClasses {
    /// Insertion-ordered, because ein.py's is a `dict` and `classes()`
    /// iterates it.
    parent: Registry<Symbol>,
}

impl EqClasses {
    pub fn new() -> Self {
        Self::default()
    }

    /// The class root, auto-vivifying and path-compressing exactly as ein.py
    /// does.
    pub fn find(&mut self, x: Symbol) -> Symbol {
        if self.parent.get(x).is_none() {
            self.parent.insert_new(x, x);
            return x;
        }
        let mut root = x;
        while *self.parent.get(root).expect("present") != root {
            root = *self.parent.get(root).expect("present");
        }
        let mut cur = x;
        while *self.parent.get(cur).expect("present") != root {
            let next = *self.parent.get(cur).expect("present");
            *self.parent.get_mut(cur).expect("present") = root;
            cur = next;
        }
        root
    }

    /// Union by *first argument* — `a`'s root wins, as ein.py's does.
    pub fn union(&mut self, a: Symbol, b: Symbol) -> Symbol {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra != rb {
            *self.parent.get_mut(rb).expect("present") = ra;
        }
        ra
    }

    pub fn equivalent(&mut self, a: Symbol, b: Symbol) -> bool {
        self.find(a) == self.find(b)
    }

    /// Root → members, in the insertion order of the parent map.
    pub fn classes(&mut self) -> Vec<(Symbol, Vec<Symbol>)> {
        let members: Vec<Symbol> = self.parent.keys().collect();
        let mut out: Vec<(Symbol, Vec<Symbol>)> = Vec::new();
        let mut at: FxHashMap<Symbol, usize> = FxHashMap::default();
        for m in members {
            let root = self.find(m);
            match at.get(&root) {
                Some(&i) => out[i].1.push(m),
                None => {
                    at.insert(root, out.len());
                    out.push((root, vec![m]));
                }
            }
        }
        out
    }

    pub fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }
}

/// What [`Kb::add_and_index_fact`] did.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Added {
    New(FactId),
    /// The proposition was already believed here. `alt` says whether the
    /// arriving derivation was kept as an alternative justification — which
    /// is the `alt` event's condition.
    Existing {
        id: FactId,
        alt: bool,
    },
}

impl Added {
    pub fn id(self) -> FactId {
        match self {
            Added::New(id) => id,
            Added::Existing { id, .. } => id,
        }
    }

    pub fn is_new(self) -> bool {
        matches!(self, Added::New(_))
    }
}

pub struct Kb {
    program: Arc<Program>,
    /// Oldest first. Immutable once sealed.
    sealed: Vec<Arc<Layer>>,
    top: Layer,
    /// Relation → the rules that name it. Computed by
    /// [`Kb::rebuild_indexes`] and then **shared by reference**, never
    /// maintained incrementally — ein.py's contract exactly, which is why a
    /// property fact added during saturation does not extend it.
    rules_by_relation: Arc<Registry<Box<[Symbol]>>>,
    classes: EqClasses,
    /// Shared by reference across forks (live branches read each other's
    /// learned clauses) and **copied** for a snapshot, which is archival and
    /// wants isolation.
    nogoods: Arc<RwLock<Nogoods>>,
}

impl Kb {
    pub fn new(program: Program) -> Kb {
        Kb {
            program: Arc::new(program),
            sealed: Vec::new(),
            top: Layer::default(),
            rules_by_relation: Arc::new(Registry::new()),
            classes: EqClasses::new(),
            nogoods: Arc::new(RwLock::new(Nogoods::default())),
        }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    /// The registries, mutably — **load time only**.
    ///
    /// Panics once the program is shared, which is precisely when mutating it
    /// would be a bug: every fork and snapshot holds the same `Arc`, and
    /// ein.py's registries are immutable after `load()` for the same reason.
    pub fn program_mut(&mut self) -> &mut Program {
        Arc::get_mut(&mut self.program).expect("the program is shared — load is over")
    }

    // ── Layers ─────────────────────────────────────────────────────

    fn layers(&self) -> impl Iterator<Item = &Layer> {
        self.sealed
            .iter()
            .map(|l| &**l)
            .chain(std::iter::once(&self.top))
    }

    /// Newest first — for the reads where the topmost answer wins.
    fn layers_rev(&self) -> impl Iterator<Item = &Layer> {
        std::iter::once(&self.top).chain(self.sealed.iter().rev().map(|l| &**l))
    }

    fn seal(&mut self) {
        if !self.top.is_empty() {
            self.sealed.push(Arc::new(std::mem::take(&mut self.top)));
        }
    }

    /// Branch for hypothesis exploration.
    ///
    /// Shares the registries, the rules-by-relation index and the no-good set
    /// by reference; copies the equality classes; and gives the branch a fresh
    /// top layer to append to. Takes `&mut self` because sealing the parent's
    /// top layer is what makes the two histories diverge — the parent's later
    /// appends land in a *new* top the child never sees.
    pub fn fork(&mut self) -> Kb {
        self.seal();
        Kb {
            program: Arc::clone(&self.program),
            sealed: self.sealed.clone(),
            top: Layer::default(),
            rules_by_relation: Arc::clone(&self.rules_by_relation),
            classes: self.classes.clone(),
            nogoods: Arc::clone(&self.nogoods),
        }
    }

    /// An archival copy, for a satisfying branch that has to survive later
    /// mutations of root. Differs from [`Kb::fork`] in one place: the no-good
    /// set is **copied**.
    pub fn snapshot(&mut self) -> Kb {
        let mut new = self.fork();
        new.nogoods = Arc::new(RwLock::new(
            self.nogoods.read().expect("no writer panicked").clone(),
        ));
        new
    }

    /// Collapse the layer stack into one — the operation design/03 §5 calls
    /// flatten, used when a branch is promoted to a root and as the check
    /// that layering changed nothing.
    pub fn flatten(&mut self) {
        let merged = self.materialise();
        self.sealed.clear();
        self.top = merged;
    }

    /// Base + delta as a single layer, by concatenation.
    pub fn materialise(&self) -> Layer {
        let mut out = Layer::default();
        for layer in self.layers() {
            out.facts.extend_from_slice(&layer.facts);
            for id in layer.present.iter() {
                out.present.insert(id);
            }
            for id in layer.negated.iter() {
                out.negated.insert(id);
            }
            for (k, v) in &layer.by_rel {
                out.by_rel.entry(*k).or_default().extend_from_slice(v);
            }
            for (k, v) in &layer.by_rel_slot_val {
                out.by_rel_slot_val
                    .entry(*k)
                    .or_default()
                    .extend_from_slice(v);
            }
            for (k, v) in &layer.rule_apps_by_rule {
                out.rule_apps_by_rule
                    .entry(*k)
                    .or_default()
                    .extend_from_slice(v);
            }
            for (k, v) in &layer.rule_apps_on_rel {
                out.rule_apps_on_rel
                    .entry(*k)
                    .or_default()
                    .extend_from_slice(v);
            }
            for (name, entry) in layer.names.iter() {
                let merged = out.names.entry(name);
                merged.as_head.extend_from_slice(&entry.as_head);
                merged.as_arg.extend_from_slice(&entry.as_arg);
            }
            for (k, v) in &layer.primary {
                out.primary.insert(*k, *v);
            }
            // Whole-list copy-on-write: a later layer's list replaces an
            // earlier one rather than extending it.
            for (k, v) in &layer.alts {
                out.alts.insert(*k, v.clone());
            }
        }
        out
    }

    // ── Reads ──────────────────────────────────────────────────────

    /// Every believed fact, in insertion order.
    pub fn facts(&self) -> impl Iterator<Item = FactId> + '_ {
        self.layers().flat_map(|l| l.facts.iter().copied())
    }

    pub fn n_facts(&self) -> usize {
        self.layers().map(|l| l.facts.len()).sum()
    }

    /// Is this proposition believed *here*? Interning a fact does not make it
    /// true; this bit does.
    pub fn contains(&self, id: FactId) -> bool {
        self.layers().any(|l| l.present.contains(id.0))
    }

    /// A relation's extent, in append order — what a `Scan` step walks.
    pub fn facts_of(&self, rel: Symbol) -> impl Iterator<Item = FactId> + '_ {
        self.layers()
            .filter_map(move |l| l.by_rel.get(&rel))
            .flat_map(|v| v.iter().copied())
    }

    pub fn n_facts_of(&self, rel: Symbol) -> usize {
        self.layers()
            .filter_map(|l| l.by_rel.get(&rel))
            .map(|v| v.len())
            .sum()
    }

    /// The participation index: facts with `value` in argument `slot` of
    /// `rel`.
    pub fn facts_with(&self, key: SlotKey) -> impl Iterator<Item = FactId> + '_ {
        self.layers()
            .filter_map(move |l| l.by_rel_slot_val.get(&key))
            .flat_map(|v| v.iter().copied())
    }

    /// Is `inner` the subject of a stored `(not inner)`?
    pub fn is_negated(&self, inner: FactId) -> bool {
        self.layers().any(|l| l.negated.contains(inner.0))
    }

    pub fn negated(&self) -> impl Iterator<Item = FactId> + '_ {
        self.layers()
            .flat_map(|l| l.negated.iter())
            .map(FactId)
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
    }

    /// Rule-application facts whose head is this rule's name.
    pub fn rule_apps_by_rule(&self, rule: Symbol) -> impl Iterator<Item = FactId> + '_ {
        self.layers()
            .filter_map(move |l| l.rule_apps_by_rule.get(&rule))
            .flat_map(|v| v.iter().copied())
    }

    /// Property facts targeting this relation — `(symmetric co-located)` is a
    /// property of `co-located`.
    pub fn rule_apps_on_relation(&self, rel: Symbol) -> impl Iterator<Item = FactId> + '_ {
        self.layers()
            .filter_map(move |l| l.rule_apps_on_rel.get(&rel))
            .flat_map(|v| v.iter().copied())
    }

    /// Every name that appears anywhere, in first-seen order.
    ///
    /// ein.py builds this dict by comprehension over a **set** union, so its
    /// order is set order — hash-dependent and not reproducible, which is why
    /// every consumer sorts (design/02 §2). This order is deterministic; it is
    /// still not something to depend on.
    pub fn names(&self) -> Vec<Symbol> {
        let mut out = Vec::new();
        let mut seen = FxHashSet::default();
        for layer in self.layers() {
            for name in layer.names.keys() {
                if seen.insert(name) {
                    out.push(name);
                }
            }
        }
        out
    }

    pub fn name_as_head(&self, name: Symbol) -> impl Iterator<Item = FactId> + '_ {
        self.layers()
            .filter_map(move |l| l.names.get(name))
            .flat_map(|e| e.as_head.iter().copied())
    }

    pub fn name_as_arg(&self, name: Symbol) -> impl Iterator<Item = FactId> + '_ {
        self.layers()
            .filter_map(move |l| l.names.get(name))
            .flat_map(|e| e.as_arg.iter().copied())
    }

    /// How many facts a name takes part in — hypgen's popularity score.
    pub fn participation(&self, name: Symbol) -> usize {
        self.layers()
            .filter_map(|l| l.names.get(name))
            .map(|e| e.as_head.len() + e.as_arg.len())
            .sum()
    }

    pub fn category(&self, terms: &Terms, name: Symbol) -> NameCategory {
        self.program.categorise(terms, name)
    }

    pub fn rules_of_relation(&self, rel: Symbol) -> &[Symbol] {
        self.rules_by_relation.get(rel).map_or(&[], |v| v)
    }

    pub fn classes(&mut self) -> &mut EqClasses {
        &mut self.classes
    }

    pub fn nogoods(&self) -> &Arc<RwLock<Nogoods>> {
        &self.nogoods
    }

    // ── Writes ─────────────────────────────────────────────────────

    /// The **loader's** path: append if new, no indexing.
    ///
    /// ein.py dedups here by scanning `self.facts` linearly, because the
    /// indexes are not built yet; interning answers the same question in O(1),
    /// and the answer is the same one — first occurrence wins, so a fact that
    /// arrives twice with different provenance keeps the first, preserving the
    /// most primitive declaration.
    pub fn add_fact(
        &mut self,
        terms: &mut Terms,
        rel: Symbol,
        args: &[Value],
        prov: Option<ProvId>,
    ) -> Result<Added, Overflow> {
        let id = terms.intern_fact(rel, args)?;
        if self.contains(id) {
            return Ok(Added::Existing { id, alt: false });
        }
        self.push_fact(id, prov);
        Ok(Added::New(id))
    }

    /// The **saturation** path: dedup against the live indexes and, on a
    /// genuinely-new fact, append it *and* index it.
    ///
    /// A fact re-derived by a second rule lands in the indexes exactly once —
    /// unlike the `add_fact` + unconditional `_index_fact` pattern's silent
    /// double-index — and the arriving derivation is kept as an alternative
    /// justification rather than discarded (S1.21.7's dedup drop).
    pub fn add_and_index_fact(
        &mut self,
        terms: &mut Terms,
        rel: Symbol,
        args: &[Value],
        prov: Option<ProvId>,
    ) -> Result<Added, Overflow> {
        let id = terms.intern_fact(rel, args)?;
        if self.contains(id) {
            let alt = match prov {
                Some(p) => self.record_justification(terms, id, p),
                None => false,
            };
            return Ok(Added::Existing { id, alt });
        }
        self.push_fact(id, prov);
        self.index_fact(terms, id);
        Ok(Added::New(id))
    }

    fn push_fact(&mut self, id: FactId, prov: Option<ProvId>) {
        self.top.facts.push(id);
        self.top.present.insert(id.0);
        if let Some(p) = prov {
            self.top.primary.insert(id, p);
        }
    }

    /// Append one fact to every reverse index — the incremental half of
    /// [`Kb::rebuild_indexes`], and it must agree with it exactly.
    pub fn index_fact(&mut self, terms: &Terms, id: FactId) {
        let (rel, args) = terms.facts.get(id);
        let is_rule_app = self.program.rules.contains(rel);
        let not = terms.kernel.not;

        self.top.by_rel.entry(rel).or_default().push(id);
        if is_rule_app {
            self.top.rule_apps_by_rule.entry(rel).or_default().push(id);
        }
        for (slot, value) in args.iter().enumerate() {
            // The join-key types only: a nested fact carries a
            // `NestedPattern` slot and full-scans, so it is not keyed.
            if value.tag() != Tag::Fact {
                let key = SlotKey {
                    rel,
                    slot: slot as u16,
                    value: *value,
                };
                self.top.by_rel_slot_val.entry(key).or_default().push(id);
            }
            if let Some(name) = value.as_sym()
                && is_rule_app
                && self.program.relations.contains(name)
            {
                self.top.rule_apps_on_rel.entry(name).or_default().push(id);
            }
        }
        if rel == not
            && let Some(inner) = args.first().and_then(|v| v.as_fact())
        {
            self.top.negated.insert(inner.0);
        }
        self.top.names.entry(rel).as_head.push(id);
        for value in args {
            if let Some(name) = value.as_sym() {
                self.top.names.entry(name).as_arg.push(id);
            }
        }
    }

    /// Recompute every reverse index from the registries and the fact list.
    ///
    /// Called once after batch ingest. It collapses the layer stack, because
    /// the result *is* the whole history.
    ///
    /// The alternative-justification table and the primary map are
    /// deliberately carried over rather than recomputed: every other index is
    /// a projection of the fact list, but those are a record of derivations the
    /// engine *attempted*, which no amount of looking at the current fact set
    /// can reconstruct.
    pub fn rebuild_indexes(&mut self, terms: &Terms) {
        let layer = self.rebuild_layer(terms, self.materialise());
        self.rules_by_relation = Arc::new(self.build_rules_by_relation(terms, &layer));
        self.sealed.clear();
        self.top = layer;
    }

    /// The indexes `merged`'s fact list implies, recomputed from scratch.
    ///
    /// Split out because it is also the *check*: layering is only sound if
    /// concatenating the layers gives what recomputing from the fact sequence
    /// gives ([`Kb::check_layering`]).
    fn rebuild_layer(&self, terms: &Terms, merged: Layer) -> Layer {
        let mut layer = Layer {
            facts: merged.facts,
            present: merged.present,
            primary: merged.primary,
            alts: merged.alts,
            ..Layer::default()
        };

        let not = terms.kernel.not;
        for &id in &layer.facts {
            let (rel, args) = terms.facts.get(id);
            let is_rule_app = self.program.rules.contains(rel);
            layer.by_rel.entry(rel).or_default().push(id);
            if is_rule_app {
                layer.rule_apps_by_rule.entry(rel).or_default().push(id);
            }
            layer.names.entry(rel).as_head.push(id);
            for (slot, value) in args.iter().enumerate() {
                if value.tag() != Tag::Fact {
                    let key = SlotKey {
                        rel,
                        slot: slot as u16,
                        value: *value,
                    };
                    layer.by_rel_slot_val.entry(key).or_default().push(id);
                }
                if let Some(name) = value.as_sym() {
                    layer.names.entry(name).as_arg.push(id);
                    if is_rule_app && self.program.relations.contains(name) {
                        layer.rule_apps_on_rel.entry(name).or_default().push(id);
                    }
                }
            }
            if rel == not
                && let Some(inner) = args.first().and_then(|v| v.as_fact())
            {
                layer.negated.insert(inner.0);
            }
        }
        // Registry names are unioned in, so a relation declared with no facts
        // yet still has an entry.
        for name in self.program.relations.keys() {
            layer.names.entry(name);
        }
        for name in self.program.rules.keys() {
            layer.names.entry(name);
        }
        layer
    }

    /// The invariant the whole layered design rests on: the concatenated view
    /// is what a mutated-in-place copy would have been.
    ///
    /// `Err` carries the first disagreement. Cheap enough to call after every
    /// saturation in a debug build, which is where design/03 §10 puts it.
    pub fn check_layering(&self, terms: &Terms) -> Result<(), String> {
        let mut flat = self.materialise();
        // A rebuild unions the registry names in, so a relation declared with
        // no facts yet gets an entry; the incremental path never does that.
        // The difference is real but it is not a layering question — it is
        // the same gap ein.py has between `_index_fact` and
        // `rebuild_indexes`, and it closes the moment the root is rebuilt.
        for name in self
            .program
            .relations
            .keys()
            .chain(self.program.rules.keys())
        {
            flat.names.entry(name);
        }
        let rebuilt = self.rebuild_layer(terms, self.materialise());
        flat.diff(&rebuilt)
    }

    /// Relations named in a rule's patterns, plus the property-fact side: a
    /// `(symmetric R)` fact makes `symmetric` a rule on `R` even though the
    /// rule's body never names `R` — `?rel` binds to it through the fact.
    fn build_rules_by_relation(&self, terms: &Terms, layer: &Layer) -> Registry<Box<[Symbol]>> {
        let mut named: FxHashMap<Symbol, FxHashSet<Symbol>> = FxHashMap::default();
        for (name, rule) in self.program.rules.iter() {
            for pattern in [rule.match_.as_ref(), rule.assert_.as_ref()]
                .into_iter()
                .flatten()
            {
                for &rel in &pattern.relation_names {
                    if self.program.relations.contains(rel) {
                        named.entry(rel).or_default().insert(name);
                    }
                }
            }
        }
        for (&rel, facts) in &layer.rule_apps_on_rel {
            for &f in facts {
                named.entry(rel).or_default().insert(terms.facts.rel(f));
            }
        }
        // `tuple(self.rules[n] for n in sorted(names) if n in self.rules)` —
        // the list is sorted **by rule name**, so it goes through the rank
        // table rather than through registry order.
        let mut out: Registry<Box<[Symbol]>> = Registry::new();
        for (rel, _) in self.program.relations.iter() {
            if let Some(set) = named.get(&rel) {
                let mut rules: Vec<Symbol> = set
                    .iter()
                    .copied()
                    .filter(|r| self.program.rules.contains(*r))
                    .collect();
                rules.sort_by_key(|r| terms.syms.rank(*r));
                out.insert_new(rel, rules.into_boxed_slice());
            }
        }
        out
    }

    // ── Provenance ─────────────────────────────────────────────────

    /// The first-recorded justification of a fact.
    pub fn primary(&self, fact: FactId) -> Option<ProvId> {
        self.layers_rev()
            .find_map(|l| l.primary.get(&fact).copied())
    }

    /// The recorded alternatives, in stored order (shortest first).
    pub fn alternatives(&self, fact: FactId) -> &[ProvId] {
        self.layers_rev()
            .find_map(|l| l.alts.get(&fact))
            .map_or(&[], |v| v)
    }

    /// Every recorded justification — primary first, then the alternatives.
    /// The fact's OR-node.
    pub fn justifications(&self, fact: FactId) -> Vec<ProvId> {
        let mut out = Vec::new();
        out.extend(self.primary(fact));
        out.extend_from_slice(self.alternatives(fact));
        out
    }

    pub fn has_alternative_justifications(&self) -> bool {
        self.layers().any(|l| !l.alts.is_empty())
    }

    /// Could `fact` still take an alternative justification with
    /// `n_premises` premises?
    ///
    /// The cheap pre-check that lets the saturator's redundant-firing path
    /// skip *building* a record (and stringifying its bindings) for the common
    /// case of a hot fact whose list is already full of shorter derivations. A
    /// `true` answer is not a promise — [`Kb::record_justification`] still
    /// applies the duplicate and ground-out rules.
    pub fn accepts_justification(&self, terms: &Terms, fact: FactId, n_premises: usize) -> bool {
        if n_premises == 0 {
            return false;
        }
        if let Some(pp) = self.primary(fact)
            && terms.provs.get(pp).is_terminal()
        {
            return false;
        }
        let current = self.alternatives(fact);
        current.len() < MAX_ALT_JUSTIFICATIONS
            || n_premises < terms.provs.get(current[current.len() - 1]).premises.len()
    }

    /// Record an **alternative** justification for an already-known fact.
    ///
    /// Returns `true` iff it was newly recorded. The primary stays where it
    /// is — first derivation wins — and this appends to the OR-node.
    ///
    /// Only rule-kind provenance with at least one premise is recorded:
    /// `source` / `hypothesis` kinds are assumptions, and a rule-kind record
    /// with no premises is a synthetic engine writeback whose contract is that
    /// walks ground out on it — recording it would give the fact the empty
    /// environment, "derivable from nothing", collapsing every explanation
    /// through it. Symmetrically, a fact whose *primary* is already a terminal
    /// takes no alternatives at all: a clue that happens to be re-derivable is
    /// still a clue.
    ///
    /// The list is capped and kept **sorted by premise count**, so at the cap
    /// an arrival with fewer premises evicts the longest: the cap biases
    /// towards the small explanations the minimality search is looking for,
    /// rather than towards whichever fired first. Sorted order is also what
    /// makes the rejection O(1).
    pub fn record_justification(&mut self, terms: &Terms, fact: FactId, prov: ProvId) -> bool {
        let p = terms.provs.get(prov);
        if p.is_terminal() {
            return false;
        }
        if let Some(pp) = self.primary(fact) {
            let pp = terms.provs.get(pp);
            if pp.is_terminal() || pp.same_justification(p) {
                return false;
            }
        }
        let current: Vec<ProvId> = self.alternatives(fact).to_vec();
        let n = p.premises.len();
        let full = current.len() >= MAX_ALT_JUSTIFICATIONS;
        if full && n >= terms.provs.get(current[current.len() - 1]).premises.len() {
            return false;
        }
        let mut at = current.len();
        for (i, &q) in current.iter().enumerate() {
            let q = terms.provs.get(q);
            if q.same_justification(p) {
                return false;
            }
            if at == current.len() && q.premises.len() > n {
                at = i;
            }
        }
        let kept = if full {
            &current[..current.len() - 1]
        } else {
            &current[..]
        };
        let mut next = Vec::with_capacity(kept.len() + 1);
        next.extend_from_slice(&kept[..at]);
        next.push(prov);
        next.extend_from_slice(&kept[at..]);
        self.top.alts.insert(fact, next.into_boxed_slice());
        true
    }

    // ── Views ──────────────────────────────────────────────────────

    /// A read-only window over every fact, in ingest order.
    pub fn all_facts<'a>(&'a self, terms: &'a Terms) -> FactView<'a> {
        FactView {
            facts: self.facts().collect(),
            kb: self,
            terms,
            label: "all",
        }
    }
}

/// A filtered window over a fact list — `kb.all_facts()`.
///
/// Eager in its fact list and lazy in its filters, exactly as ein.py's is.
pub struct FactView<'a> {
    facts: Vec<FactId>,
    kb: &'a Kb,
    terms: &'a Terms,
    label: &'static str,
}

impl<'a> FactView<'a> {
    pub fn len(&self) -> usize {
        self.facts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn iter(&self) -> impl Iterator<Item = FactId> + '_ {
        self.facts.iter().copied()
    }

    pub fn contains(&self, fact: FactId) -> bool {
        self.facts.contains(&fact)
    }

    /// Facts whose head relation is `name`.
    pub fn relation(&self, name: Symbol) -> impl Iterator<Item = FactId> + '_ {
        self.iter()
            .filter(move |&f| self.terms.facts.rel(f) == name)
    }

    /// Facts mentioning `target` in any argument position.
    pub fn about(&self, target: Value) -> impl Iterator<Item = FactId> + '_ {
        self.iter()
            .filter(move |&f| self.terms.facts.args(f).contains(&target))
    }

    /// Facts whose `:source` annotation matches.
    pub fn by_source(&self, source: Symbol) -> impl Iterator<Item = FactId> + '_ {
        self.iter().filter(move |&f| {
            self.kb.primary(f).is_some_and(|p| {
                let p = self.terms.provs.get(p);
                p.kind == crate::prov::ProvKind::Source && p.source == Some(source)
            })
        })
    }

    /// Every fact a specific rule derived.
    pub fn by_rule(&self, rule: Symbol) -> impl Iterator<Item = FactId> + '_ {
        self.iter().filter(move |&f| {
            self.kb.primary(f).is_some_and(|p| {
                let p = self.terms.provs.get(p);
                p.kind == crate::prov::ProvKind::Rule && p.rule == Some(rule)
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{ExprRef, Pattern, Relation, Rule};
    use crate::prov::Prov;

    /// A zebra-shaped miniature: two declared relations, one rule whose name
    /// is also a property-fact head, and the `(not …)` wrapper.
    fn fixture() -> (Terms, Kb) {
        let mut terms = Terms::new();
        let mut program = Program::new();
        for name in ["co-located", "next-to"] {
            let name = terms.intern_text(name).expect("room");
            program.add_relation(Relation {
                name,
                signature: Box::new([]),
                declared: true,
                why: None,
                loc: None,
            });
        }
        let symmetric = terms.intern_text("symmetric").expect("room");
        let co_located = terms.intern_text("co-located").expect("room");
        program.add_rule(Rule {
            name: symmetric,
            params: Box::new([]),
            match_: Some(Pattern {
                expr: ExprRef(0),
                variables: Box::new([]),
                relation_names: Box::new([co_located]),
            }),
            assert_: None,
            why: None,
            priority: None,
            loc: None,
        });
        // The property tag is an open-world relation: its atom names a rule.
        program.add_relation(Relation {
            name: symmetric,
            signature: Box::new([]),
            declared: false,
            why: None,
            loc: None,
        });
        (terms, Kb::new(program))
    }

    fn add(kb: &mut Kb, terms: &mut Terms, rel: &str, args: &[&str]) -> FactId {
        let rel = terms.intern_text(rel).expect("room");
        let args: Vec<Value> = args
            .iter()
            .map(|a| terms.value_text(a).expect("room"))
            .collect();
        kb.add_and_index_fact(terms, rel, &args, None)
            .expect("room")
            .id()
    }

    fn sym(terms: &mut Terms, s: &str) -> Symbol {
        terms.intern_text(s).expect("room")
    }

    #[test]
    fn a_fork_reads_base_then_delta_and_the_two_histories_diverge() {
        let (mut terms, mut kb) = fixture();
        let a = add(&mut kb, &mut terms, "co-located", &["Norwegian", "House-1"]);
        let b = add(
            &mut kb,
            &mut terms,
            "co-located",
            &["Englishman", "House-2"],
        );

        let mut child = kb.fork();
        let c = add(
            &mut child,
            &mut terms,
            "co-located",
            &["Spaniard", "House-3"],
        );
        // The parent keeps writing after the fork; the child must not see it.
        let d = add(&mut kb, &mut terms, "co-located", &["Ukrainian", "House-4"]);

        assert_eq!(child.facts().collect::<Vec<_>>(), vec![a, b, c]);
        assert_eq!(kb.facts().collect::<Vec<_>>(), vec![a, b, d]);
        let rel = sym(&mut terms, "co-located");
        // Base first, then the delta — the order "copy the list, then append"
        // produces.
        assert_eq!(child.facts_of(rel).collect::<Vec<_>>(), vec![a, b, c]);
        assert!(child.contains(c) && !kb.contains(c));
        assert!(kb.contains(d) && !child.contains(d));
    }

    #[test]
    fn the_layered_view_is_what_a_rebuild_would_have_produced() {
        let (mut terms, mut kb) = fixture();
        add(&mut kb, &mut terms, "co-located", &["Norwegian", "House-1"]);
        add(&mut kb, &mut terms, "symmetric", &["co-located"]);
        kb.check_layering(&terms).expect("one layer");

        let mut child = kb.fork();
        add(&mut child, &mut terms, "next-to", &["House-1", "House-2"]);
        add(
            &mut child,
            &mut terms,
            "co-located",
            &["Englishman", "House-2"],
        );
        child.check_layering(&terms).expect("two layers");

        let mut grandchild = child.fork();
        add(&mut grandchild, &mut terms, "symmetric", &["next-to"]);
        grandchild.check_layering(&terms).expect("three layers");

        // Flattening is supposed to be invisible.
        let before: Vec<_> = grandchild.facts().collect();
        grandchild.flatten();
        assert_eq!(grandchild.facts().collect::<Vec<_>>(), before);
        grandchild.check_layering(&terms).expect("flattened");
    }

    #[test]
    fn incremental_indexing_and_a_rebuild_agree() {
        let facts: [(&str, &[&str]); 5] = [
            ("co-located", &["Norwegian", "House-1"]),
            ("symmetric", &["co-located"]),
            ("next-to", &["House-1", "House-2"]),
            ("co-located", &["Englishman", "House-1"]),
            ("symmetric", &["next-to"]),
        ];
        let (mut terms, mut incremental) = fixture();
        let (_, mut batched) = fixture();
        for (rel, args) in facts {
            add(&mut incremental, &mut terms, rel, args);
            let r = terms.intern_text(rel).expect("room");
            let vs: Vec<Value> = args
                .iter()
                .map(|a| terms.value_text(a).expect("room"))
                .collect();
            batched.add_fact(&mut terms, r, &vs, None).expect("room");
        }
        batched.rebuild_indexes(&terms);
        incremental
            .materialise()
            .diff(&batched.materialise())
            .expect("the two paths agree");
    }

    #[test]
    fn the_participation_index_keys_only_the_join_types() {
        let (mut terms, mut kb) = fixture();
        let co_located = sym(&mut terms, "co-located");
        let not = terms.kernel.not;
        let norwegian = terms.value_text("Norwegian").expect("room");
        let one = terms.value_int("1").expect("room");
        let inner = terms
            .value_fact(co_located, &[norwegian, one])
            .expect("room");
        let f = kb
            .add_and_index_fact(&mut terms, not, &[inner], None)
            .expect("room")
            .id();

        // A nested-fact argument is not keyed — it carries a NestedPattern
        // slot and full-scans — but a str and an int both are.
        assert_eq!(
            kb.facts_with(SlotKey {
                rel: not,
                slot: 0,
                value: inner
            })
            .count(),
            0
        );
        assert_eq!(
            kb.facts_with(SlotKey {
                rel: co_located,
                slot: 1,
                value: one
            })
            .count(),
            0,
            "the nested fact is interned, not believed, so it is not indexed"
        );
        // The negated index holds the *inner* id.
        assert!(kb.is_negated(inner.as_fact().expect("nested")));
        assert!(!kb.is_negated(f));
        // Nor does a nested fact bump the names index (Q40).
        assert_eq!(kb.name_as_arg(sym(&mut terms, "Norwegian")).count(), 0);
        assert_eq!(kb.name_as_head(not).collect::<Vec<_>>(), vec![f]);
    }

    #[test]
    fn a_property_fact_indexes_as_a_rule_application() {
        let (mut terms, mut kb) = fixture();
        let f = add(&mut kb, &mut terms, "symmetric", &["co-located"]);
        let symmetric = sym(&mut terms, "symmetric");
        let co_located = sym(&mut terms, "co-located");
        assert_eq!(kb.rule_apps_by_rule(symmetric).collect::<Vec<_>>(), vec![f]);
        assert_eq!(
            kb.rule_apps_on_relation(co_located).collect::<Vec<_>>(),
            vec![f]
        );
        kb.rebuild_indexes(&terms);
        // The property-fact side of `_rules_by_relation`: `symmetric` is a
        // rule on `co-located` even though its body never names it.
        assert_eq!(kb.rules_of_relation(co_located), &[symmetric]);
    }

    #[test]
    fn the_loader_path_keeps_the_first_occurrence_and_does_not_index() {
        let (mut terms, mut kb) = fixture();
        let rel = sym(&mut terms, "co-located");
        let args = [
            terms.value_text("Norwegian").expect("room"),
            terms.value_text("House-1").expect("room"),
        ];
        let first = terms.provs.push(Prov::from_source(None, None));
        let second = terms.provs.push(Prov::from_source(None, None));
        let a = kb
            .add_fact(&mut terms, rel, &args, Some(first))
            .expect("room");
        let b = kb
            .add_fact(&mut terms, rel, &args, Some(second))
            .expect("room");
        assert!(a.is_new());
        assert_eq!(
            b,
            Added::Existing {
                id: a.id(),
                alt: false
            }
        );
        assert_eq!(kb.n_facts(), 1);
        assert_eq!(kb.primary(a.id()), Some(first), "first occurrence wins");
        assert_eq!(kb.facts_of(rel).count(), 0, "add_fact does not index");
        kb.rebuild_indexes(&terms);
        assert_eq!(kb.facts_of(rel).count(), 1);
    }

    #[test]
    fn a_re_derived_fact_is_indexed_once() {
        let (mut terms, mut kb) = fixture();
        let rel = sym(&mut terms, "co-located");
        let args = [
            terms.value_text("Norwegian").expect("room"),
            terms.value_text("House-1").expect("room"),
        ];
        kb.add_and_index_fact(&mut terms, rel, &args, None)
            .expect("room");
        kb.add_and_index_fact(&mut terms, rel, &args, None)
            .expect("room");
        assert_eq!(kb.facts_of(rel).count(), 1);
        assert_eq!(kb.n_facts(), 1);
    }

    #[test]
    fn a_fork_shares_no_goods_and_a_snapshot_copies_them() {
        let (mut terms, mut kb) = fixture();
        let a = add(&mut kb, &mut terms, "co-located", &["Norwegian", "House-1"]);
        let mut fork = kb.fork();
        let snapshot = kb.snapshot();
        kb.nogoods()
            .write()
            .expect("no writer panicked")
            .insert(Box::new([a]));
        // Live branches read each other's learned clauses…
        assert_eq!(fork.nogoods().read().expect("ok").len(), 1);
        // …but the archival copy is isolated.
        assert_eq!(snapshot.nogoods().read().expect("ok").len(), 0);
        // And a fork of a fork keeps sharing.
        let grandchild = fork.fork();
        assert_eq!(grandchild.nogoods().read().expect("ok").len(), 1);
    }

    #[test]
    fn equality_classes_are_copied_and_stay_inert() {
        let (mut terms, mut kb) = fixture();
        let (a, b) = (sym(&mut terms, "a"), sym(&mut terms, "b"));
        kb.classes().union(a, b);
        let mut fork = kb.fork();
        let c = sym(&mut terms, "c");
        fork.classes().union(b, c);
        assert!(fork.classes().equivalent(a, c));
        assert_eq!(kb.classes().classes(), vec![(a, vec![a, b])]);
        // `find` auto-vivifies, as ein.py's does — so *asking* the root adds
        // `c` to the root's map, without joining it to anything.
        assert!(!kb.classes().equivalent(a, c), "the copy is the fork's own");
        assert_eq!(kb.classes().classes(), vec![(a, vec![a, b]), (c, vec![c])]);
    }

    #[test]
    #[should_panic(expected = "the program is shared")]
    fn the_registries_cannot_be_edited_once_a_branch_exists() {
        let (_terms, mut kb) = fixture();
        let _fork = kb.fork();
        kb.program_mut();
    }

    // ── Provenance policy ──────────────────────────────────────────

    fn firing(terms: &mut Terms, rule: Symbol, premises: &[FactId]) -> ProvId {
        terms.provs.push(Prov::from_rule(
            rule,
            premises.to_vec().into_boxed_slice(),
            None,
        ))
    }

    #[test]
    fn only_a_real_firing_is_recorded_as_an_alternative() {
        let (mut terms, mut kb) = fixture();
        let rel = sym(&mut terms, "co-located");
        let args = [terms.value_text("a").expect("room")];
        let rule = sym(&mut terms, "symmetric");
        let premise = FactId(0);
        let primary = firing(&mut terms, rule, &[premise]);
        let fact = kb
            .add_and_index_fact(&mut terms, rel, &args, Some(primary))
            .expect("room")
            .id();

        // An assumption is not a derivation.
        let assumption = terms.provs.push(Prov::from_source(None, None));
        assert!(!kb.record_justification(&terms, fact, assumption));
        // Nor is a synthetic writeback, whose contract is that walks ground
        // out on it.
        let writeback = firing(&mut terms, rule, &[]);
        assert!(!kb.record_justification(&terms, fact, writeback));
        // The primary itself is not an alternative to itself.
        let same = firing(&mut terms, rule, &[premise]);
        assert!(!kb.record_justification(&terms, fact, same));
        // A genuinely different derivation is.
        let other = firing(&mut terms, rule, &[FactId(1), FactId(2)]);
        assert!(kb.record_justification(&terms, fact, other));
        assert_eq!(kb.alternatives(fact), &[other]);
        assert_eq!(kb.justifications(fact), vec![primary, other]);
        assert!(kb.has_alternative_justifications());
    }

    #[test]
    fn a_terminal_primary_takes_no_alternatives_at_all() {
        let (mut terms, mut kb) = fixture();
        let rel = sym(&mut terms, "co-located");
        let args = [terms.value_text("a").expect("room")];
        let source = terms.provs.push(Prov::from_source(None, None));
        let fact = kb
            .add_and_index_fact(&mut terms, rel, &args, Some(source))
            .expect("room")
            .id();
        let rule = sym(&mut terms, "symmetric");
        let derivation = firing(&mut terms, rule, &[FactId(7)]);
        assert!(!kb.accepts_justification(&terms, fact, 1));
        assert!(!kb.record_justification(&terms, fact, derivation));
        assert!(kb.alternatives(fact).is_empty());
    }

    #[test]
    fn the_cap_keeps_the_shortest_derivations() {
        let (mut terms, mut kb) = fixture();
        let rel = sym(&mut terms, "co-located");
        let args = [terms.value_text("a").expect("room")];
        let rule = sym(&mut terms, "symmetric");
        let primary = firing(&mut terms, rule, &[FactId(0)]);
        let fact = kb
            .add_and_index_fact(&mut terms, rel, &args, Some(primary))
            .expect("room")
            .id();

        // Arrive longest-first, so every arrival has to be inserted before
        // the ones already there.
        for n in (2..2 + MAX_ALT_JUSTIFICATIONS as u32).rev() {
            let premises: Vec<FactId> = (0..n).map(FactId).collect();
            let prov = firing(&mut terms, rule, &premises);
            assert!(kb.record_justification(&terms, fact, prov));
        }
        let kept = kb.alternatives(fact).to_vec();
        assert_eq!(kept.len(), MAX_ALT_JUSTIFICATIONS);
        let lengths: Vec<usize> = kept
            .iter()
            .map(|&p| terms.provs.get(p).premises.len())
            .collect();
        assert!(
            lengths.windows(2).all(|w| w[0] <= w[1]),
            "sorted: {lengths:?}"
        );

        // Full, and the arrival is no shorter than the longest kept — the
        // O(1) rejection.
        let longest = *lengths.last().expect("non-empty");
        assert!(!kb.accepts_justification(&terms, fact, longest));
        let premises: Vec<FactId> = (100..100 + longest as u32).map(FactId).collect();
        let too_long = firing(&mut terms, rule, &premises);
        assert!(!kb.record_justification(&terms, fact, too_long));

        // A shorter one evicts the longest.
        assert!(kb.accepts_justification(&terms, fact, 1));
        let short = firing(&mut terms, rule, &[FactId(50)]);
        assert!(kb.record_justification(&terms, fact, short));
        let after = kb.alternatives(fact).to_vec();
        assert_eq!(after.len(), MAX_ALT_JUSTIFICATIONS);
        assert_eq!(after[0], short);
        assert!(!after.contains(kept.last().expect("non-empty")));
    }

    #[test]
    fn a_forks_alternatives_do_not_leak_into_its_parent() {
        // ein.py copies `_alt_justifications` per fork precisely because a
        // justification recorded inside a hypothesis fork can name premises
        // root never assumed; sharing would surface a phantom assumption in a
        // root-level unsat core.
        let (mut terms, mut kb) = fixture();
        let rel = sym(&mut terms, "co-located");
        let args = [terms.value_text("a").expect("room")];
        let rule = sym(&mut terms, "symmetric");
        let primary = firing(&mut terms, rule, &[FactId(0)]);
        let fact = kb
            .add_and_index_fact(&mut terms, rel, &args, Some(primary))
            .expect("room")
            .id();
        let shared = firing(&mut terms, rule, &[FactId(1)]);
        assert!(kb.record_justification(&terms, fact, shared));

        let mut fork = kb.fork();
        let branch_only = firing(&mut terms, rule, &[FactId(2), FactId(3)]);
        assert!(fork.record_justification(&terms, fact, branch_only));
        assert_eq!(fork.alternatives(fact), &[shared, branch_only]);
        assert_eq!(kb.alternatives(fact), &[shared], "root is untouched");
    }

    // ── Views ──────────────────────────────────────────────────────

    #[test]
    fn the_fact_view_filters_the_way_ein_py_does() {
        let (mut terms, mut kb) = fixture();
        let condition = sym(&mut terms, "condition (10)");
        let rule = sym(&mut terms, "symmetric");
        let source = terms.provs.push(Prov::from_source(Some(condition), None));
        let derived = terms
            .provs
            .push(Prov::from_rule(rule, Box::new([FactId(0)]), None));
        let rel = sym(&mut terms, "co-located");
        let norwegian = terms.value_text("Norwegian").expect("room");
        let house = terms.value_text("House-1").expect("room");
        let a = kb
            .add_and_index_fact(&mut terms, rel, &[norwegian, house], Some(source))
            .expect("room")
            .id();
        let next_to = sym(&mut terms, "next-to");
        let b = kb
            .add_and_index_fact(&mut terms, next_to, &[house, house], Some(derived))
            .expect("room")
            .id();

        let view = kb.all_facts(&terms);
        assert_eq!(view.len(), 2);
        assert!(view.contains(a));
        assert_eq!(view.relation(rel).collect::<Vec<_>>(), vec![a]);
        assert_eq!(view.about(house).collect::<Vec<_>>(), vec![a, b]);
        assert_eq!(view.about(norwegian).collect::<Vec<_>>(), vec![a]);
        assert_eq!(view.by_source(condition).collect::<Vec<_>>(), vec![a]);
        assert_eq!(view.by_rule(rule).collect::<Vec<_>>(), vec![b]);
    }
}
