//! The knowledge base — a stack of immutable layers and one writable top.
//!
//! ein.py's `fork()` shallow-copies the fact list and six index dicts, and
//! `snapshot()` does the same plus a `_nogoods` copy. That is cheap today
//! (0.003 s over 206 calls on an exhaustive `zebra2`) because there are only
//! 101 enterings — but [P1a.7](../../../../docs/history/m1a_rust/README.md#p1a7--parallelism)
//! wants hundreds live at once, and [design/05](../../../../docs/history/m1a_rust/design/05_matcher.md)'s
//! beta-memories are only affordable if a fork does not copy them, which is
//! the exact objection [F11](../../../../plans/followups/f11_deductive_layer_perf.md)
//! parks them on.
//!
//! So a `Kb` is `Vec<Arc<Layer>>` plus a writable [`Layer`]
//! ([design/03](../../../../docs/history/m1a_rust/design/03_data_model.md) §5):
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
/// position, and the value in it — or, since
/// [T1a.6.3.0](../../../../docs/history/m1a_rust/README.md#s1a63--beta-memories-f11-d1),
/// a position *inside* the fact in that argument.
///
/// 12 bytes with padding. design/03 §6 notes that packing it into a `u64`
/// does not fit — the `Value` needs 32 bits and the symbol another 32 — so
/// this stays a struct key, and `inner` fits in the padding that was already
/// there.
///
/// **Why the second level exists.** ein.py keys "the join-key types only": a
/// `Fact`-valued argument is not indexed, so a `(not (R ?a ?b))` premise
/// scans `not`'s whole extent. On an exhaustive `zebra` that is **99.1 %** of
/// 25.16 M candidates walking a 368-fact extent to reject all but a handful
/// ([baseline.md § 13](../../../../docs/history/m1a_rust/measurements/baseline.md)).
/// Keying one level in turns that scan into a bucket lookup. It is a
/// *narrowing* — the matcher re-checks every slot regardless — so it changes
/// which facts are offered and not which ones match, and buckets are appended
/// in insertion order like every other index here, so the surviving order is
/// the extent's.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SlotKey {
    pub rel: Symbol,
    pub slot: u16,
    /// The position inside the nested fact at `slot`, or [`SlotKey::DIRECT`]
    /// for the argument itself.
    pub inner: u16,
    pub value: Value,
}

impl SlotKey {
    /// `inner` for a key on the argument itself rather than inside it.
    pub const DIRECT: u16 = u16::MAX;

    /// The ordinary one-level key — `(rel, slot) = value`.
    pub fn direct(rel: Symbol, slot: u16, value: Value) -> SlotKey {
        SlotKey {
            rel,
            slot,
            inner: SlotKey::DIRECT,
            value,
        }
    }
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

/// Bits in a layer's participation-index Bloom filter — 256 bytes a layer.
///
/// A *hit* is never reported as a miss, so the filter can only skip a layer
/// that provably lacks the key; what the size buys is how often a miss is
/// recognised as one. [T1a.6.3.0's
/// profile](../../../../docs/history/m1a_rust/measurements/baseline.md) put
/// **15.6 %** of an exhaustive `zebra` in the layer walk of `facts_with` —
/// a fork 24 layers deep hashing its key 24 times to find the one or two
/// layers that have it — and this takes 6–7 % of the run off.
///
/// Sized by sweep rather than by arithmetic: 512 bits is −6.0 %, 2048 is
/// **−7.3 %**, and 8192 is −7.2 %, so the curve is flat past here and the
/// extra 768 bytes a layer would buy nothing. A fork's layer is ~6 KB, which
/// is what [P1a.7](../../../../docs/history/m1a_rust/README.md#p1a7--parallelism)
/// sizes `--jobs` by, so 256 bytes is 4 % of it.
const BLOOM_BITS: usize = 2048;
const BLOOM_WORDS: usize = BLOOM_BITS / 64;

/// Facts added at one point in a KB's history, with the indexes over them.
///
/// Sealed layers are shared by `Arc`; the top layer is the only writable one.
#[derive(Clone, Debug)]
pub struct Layer {
    facts: Vec<FactId>,
    present: BitSet,
    by_rel: FxHashMap<Symbol, Vec<FactId>>,
    by_rel_slot_val: FxHashMap<SlotKey, Vec<FactId>>,
    /// A Bloom filter over `by_rel_slot_val`'s keys — derived state, rebuilt
    /// wherever that map is written and never read for anything but skipping.
    slot_bloom: [u64; BLOOM_WORDS],
    /// The **inner** id of every `(not X)` — a bitset, where ein.py keeps a
    /// `set` of `(relation, args)` tuples.
    negated: BitSet,
    rule_apps_by_rule: FxHashMap<Symbol, Vec<FactId>>,
    /// How many facts this layer added whose head names a rule.
    ///
    /// `Engine.compile_all` walks `rules × activators` on every enqueue pass
    /// and the walk is only *worth* repeating when a rule gained an activator
    /// — which happens exactly when this grows
    /// ([design/06](../../../../docs/history/m1a_rust/design/06_saturation.md) § Win A).
    /// Counting it per layer keeps the question O(layers) instead of
    /// O(rules × activators), which is the size of the walk being skipped.
    rule_apps: u32,
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

impl Default for Layer {
    fn default() -> Layer {
        Layer {
            facts: Vec::new(),
            present: BitSet::default(),
            by_rel: FxHashMap::default(),
            by_rel_slot_val: FxHashMap::default(),
            slot_bloom: [0; BLOOM_WORDS],
            negated: BitSet::default(),
            rule_apps_by_rule: FxHashMap::default(),
            rule_apps: 0,
            rule_apps_on_rel: FxHashMap::default(),
            names: Registry::new(),
            primary: FxHashMap::default(),
            alts: FxHashMap::default(),
        }
    }
}

/// Where a key lands in a layer's Bloom filter — `(word, bit mask)`.
///
/// One hash, split: the low six bits pick the bit and the next three the word,
/// which is `FxHasher`'s well-mixed output used twice rather than hashed twice.
fn bloom_at(key: &SlotKey) -> (usize, u64) {
    let mut h = rustc_hash::FxHasher::default();
    std::hash::Hash::hash(key, &mut h);
    let h = std::hash::Hasher::finish(&h);
    (((h >> 6) as usize) % BLOOM_WORDS, 1u64 << (h & 63))
}

impl Layer {
    /// `false` only when this layer certainly has no bucket for `key`.
    fn bloom_may_have(&self, at: (usize, u64)) -> bool {
        self.slot_bloom[at.0] & at.1 != 0
    }

    fn bloom_add(&mut self, key: &SlotKey) {
        let (word, bit) = bloom_at(key);
        self.slot_bloom[word] |= bit;
    }

    fn is_empty(&self) -> bool {
        self.facts.is_empty() && self.alts.is_empty()
    }

    /// Does this layer cite a record from the arena's fork region?
    ///
    /// Asked before [`Layer::rewrite_provenance`] so a promotion does not
    /// `Arc::make_mut` — and so clone — a layer it has nothing to do to.
    fn cites_fork_provenance(&self) -> bool {
        // determinism-ok: an `any` over two maps; no order reaches it.
        self.primary.values().any(|p| p.is_fork())
            // determinism-ok: the second of the two, and the same reason.
            || self.alts.values().flatten().any(|p| p.is_fork())
    }

    /// Re-point this layer's fork citations at the copies a promotion made.
    ///
    /// Positions are preserved, which matters for `alts`: the list is ordered
    /// shortest-premises-first and that order is what a minimum-cardinality
    /// explanation search reads.
    fn rewrite_provenance(&mut self, map: &FxHashMap<ProvId, ProvId>) {
        // determinism-ok: every value is rewritten independently, so the visit order decides nothing.
        for p in self.primary.values_mut() {
            if let Some(&to) = map.get(p) {
                *p = to;
            }
        }
        // determinism-ok: as above.
        for a in self.alts.values_mut() {
            for p in a.iter_mut() {
                if let Some(&to) = map.get(p) {
                    *p = to;
                }
            }
        }
    }

    /// How many facts this layer added.
    pub fn n_facts(&self) -> usize {
        self.facts.len()
    }

    /// Heap bytes this layer holds — the **delta** a fork owns, which is the
    /// number [P1a.7](../../../../docs/history/m1a_rust/README.md#p1a7--parallelism)
    /// sizes `--jobs` by: a machine can hold `RAM / mean-delta` searches at
    /// once, and design/03 §5's claim that a fork is O(1) is a claim about
    /// exactly this quantity not growing with the KB.
    ///
    /// Capacity, not length, because a `Vec` that doubled holds the doubling;
    /// and the hash maps are counted at their bucket cost, so an index of ten
    /// one-element vectors is not mistaken for ten bytes. It is an estimate —
    /// `hashbrown`'s control bytes and the allocator's rounding are not
    /// visible from here — and `alloc_cost.rs` cross-checks the total against a
    /// counting allocator, which is why an estimate is good enough.
    pub fn footprint(&self) -> usize {
        fn vec_map<K, V>(m: &FxHashMap<K, Vec<V>>) -> usize {
            // One bucket per slot plus each vector's own allocation.
            m.capacity() * (size_of::<K>() + size_of::<Vec<V>>())
                // determinism-ok: a sum over the values; no order reaches it.
                + m.values()
                    .map(|v| v.capacity() * size_of::<V>())
                    .sum::<usize>()
        }
        self.facts.capacity() * size_of::<FactId>()
            + (self.present.len() + self.negated.len()) * size_of::<u64>()
            + size_of_val(&self.slot_bloom)
            + vec_map(&self.by_rel)
            + vec_map(&self.by_rel_slot_val)
            + vec_map(&self.rule_apps_by_rule)
            + vec_map(&self.rule_apps_on_rel)
            + self.names.len() * (size_of::<Symbol>() + size_of::<NameEntry>())
            + self.primary.capacity() * (size_of::<FactId>() + size_of::<ProvId>())
            + self.alts.capacity() * (size_of::<FactId>() + size_of::<Box<[ProvId]>>())
            + self
                .alts
                .values()
                .map(|v| v.len() * size_of::<ProvId>())
                .sum::<usize>()
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
        if self.rule_apps != other.rule_apps {
            return Err(format!(
                "rule-application counts: {} vs {}",
                self.rule_apps, other.rule_apps
            ));
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
    // The order picks which of several disagreements gets named first, in an `Err` that
    // exists only once the layering invariant is already broken — and the loop above
    // iterates `a` the same way.
    // determinism-ok: `check_layering`'s diagnostic, never an engine output.
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

    /// Drop one clause — `set.discard`, which subsumption needs when a new
    /// clause makes a stored superset redundant.
    pub fn remove(&mut self, clause: &[FactId]) -> bool {
        self.clauses.remove(clause)
    }

    pub fn insert(&mut self, clause: Box<[FactId]>) -> bool {
        self.clauses.insert(clause)
    }

    /// The clauses, in no particular order — see the annotation below before
    /// using this from the search layer.
    pub fn iter(&self) -> impl Iterator<Item = &[FactId]> {
        // The oracle's `_nogoods` is itself a `set[frozenset]` (`kb/store.py`), so hash
        // order is what ein.py has here too and the PYTHONHASHSEED sweep says it reaches
        // nothing: subsumption tests it with `any`/`all`, apriori filters supersets with
        // it, and the proof snapshot's only observable is `learned_nogoods_count`, a
        // count. P1a.4 inherits the constraint — consume this into something
        // order-insensitive, or sort at the point of output.
        // determinism-ok: every consumer of a no-good clause set is order-insensitive.
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
    /// `relation → |extent|`, across **every** layer.
    ///
    /// `n_facts_of` is asked 644 166 times on an exhaustive `zebra2` — once per
    /// watched relation per parked boundary entry — and folding it over the
    /// layer stack made it O(depth) where ein.py's flat index answers in O(1).
    /// The search reaches depth 35, and that cost was 9.5 % of the run
    /// ([baseline.md §7](../../../../docs/history/m1a_rust/measurements/baseline.md)
    /// item 2). Maintained wherever `by_rel` is, and cloned per fork: a map of
    /// one `u32` per *declared relation* — 17 on the zebra puzzles — against a
    /// delta that is already kilobytes.
    n_by_rel: FxHashMap<Symbol, u32>,
    classes: EqClasses,
    /// Shared by reference across forks (live branches read each other's
    /// learned clauses) and **copied** for a snapshot, which is archival and
    /// wants isolation.
    nogoods: Arc<RwLock<Nogoods>>,
}

/// `<KnowledgeBase relations=17 rules=30 facts=84>` — what ein.py's
/// `__repr__` prints, which is also the most useful thing a failing test can
/// show.
impl std::fmt::Debug for Kb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<KnowledgeBase relations={} rules={} facts={}>",
            self.program.relations.len(),
            self.program.rules.len(),
            self.n_facts()
        )
    }
}

impl Kb {
    pub fn new(program: Program) -> Kb {
        Kb {
            program: Arc::new(program),
            sealed: Vec::new(),
            top: Layer::default(),
            rules_by_relation: Arc::new(Registry::new()),
            n_by_rel: FxHashMap::default(),
            classes: EqClasses::new(),
            nogoods: Arc::new(RwLock::new(Nogoods::default())),
        }
    }

    /// A KB over registries someone else already built.
    ///
    /// The `.einb` reader's, and only its ([design/10
    /// §2](../../../../docs/history/m1a_rust/design/10_binary_format.md)): it rebuilds
    /// the program by re-loading the file's `PROGRAM` section and then installs
    /// the file's *own* fact state on top of those registries rather than the
    /// loader's. Sharing the `Arc` is what makes that two KBs over one program
    /// instead of two programs.
    pub fn with_program(program: Arc<Program>) -> Kb {
        Kb {
            program,
            sealed: Vec::new(),
            top: Layer::default(),
            rules_by_relation: Arc::new(Registry::new()),
            n_by_rel: FxHashMap::default(),
            classes: EqClasses::new(),
            nogoods: Arc::new(RwLock::new(Nogoods::default())),
        }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    /// The registries, shareable — the other half of [`Kb::with_program`].
    pub fn program_arc(&self) -> Arc<Program> {
        Arc::clone(&self.program)
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
        self.branch()
    }

    /// Seal the top layer, so later appends land in a new one — the half of
    /// [`Kb::fork`] that mutates, and the only half a *layer* has to do once.
    ///
    /// [P1a.7](../../../../docs/history/m1a_rust/README.md#p1a7--parallelism) is why
    /// the two are separable: a fanned-out layer's workers all branch from one
    /// root and none of them may write to it, so the seal happens once when the
    /// layer opens and every worker then calls [`Kb::branch`] through a `&`.
    pub fn seal_top(&mut self) {
        self.seal();
    }

    /// Seal the top layer and hand back a shared view of the result — the
    /// idiom every [`Kb::branch`] caller wants, in one call.
    ///
    /// `try_commitment_set(kb.sealed(), …)` is the sequential shape; a
    /// fanned-out layer calls [`Kb::seal_top`] once and then [`Kb::branch`]
    /// from many threads.
    pub fn sealed(&mut self) -> &Kb {
        self.seal();
        self
    }

    /// Branch from an already-sealed KB, touching nothing.
    ///
    /// Cheap and shareable: three `Arc` clones and two small maps. What makes
    /// it sound to take by `&` is the seal — a KB with facts still in its top
    /// layer would hand the branch a view that does not contain them — so it
    /// asserts rather than trusting, in every build, because the failure is a
    /// fork that silently believes less than its parent.
    pub fn branch(&self) -> Kb {
        crate::counters::bump(|c| c.fork += 1);
        assert!(
            self.top.is_empty(),
            "Kb::branch on an unsealed KB — call Kb::seal_top first"
        );
        Kb {
            program: Arc::clone(&self.program),
            sealed: self.sealed.clone(),
            top: Layer::default(),
            rules_by_relation: Arc::clone(&self.rules_by_relation),
            n_by_rel: self.n_by_rel.clone(),
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
    /// flatten, used when a branch is promoted to a root, as the check that
    /// layering changed nothing, and — since
    /// [T1a.7.2.0](../../../../docs/history/m1a_rust/README.md#s1a72--level-1-parallel-enterings)
    /// — at the search's layer barrier, because every fork inherits the whole
    /// stack and a mid-layer root write seals another one.
    ///
    /// Content-neutral and order-neutral, which is what makes it safe to call
    /// mid-search: [`Kb::materialise`] concatenates the layers oldest-first,
    /// so `facts()` and `facts_of()` yield the same sequences, and the two
    /// last-wins maps (`primary`, `alts`) end up with the same winner a
    /// newest-first lookup would have found. [`Kb::check_layering`] is the
    /// standing assertion of exactly that.
    pub fn flatten(&mut self) {
        crate::counters::bump(|c| {
            c.flatten += 1;
            c.flatten_facts += self.n_facts() as u64;
        });
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
            for (word, bits) in layer.slot_bloom.iter().enumerate() {
                out.slot_bloom[word] |= bits;
            }
            for (k, v) in &layer.rule_apps_by_rule {
                out.rule_apps_by_rule
                    .entry(*k)
                    .or_default()
                    .extend_from_slice(v);
            }
            out.rule_apps += layer.rule_apps;
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

    /// The top layer — a fork's own delta, everything below it shared.
    pub fn top(&self) -> &Layer {
        &self.top
    }

    /// How many layers deep this KB is. `1` is a root; a fork of a fork is `3`,
    /// because sealing the parent's `top` leaves it in `sealed`.
    pub fn depth(&self) -> usize {
        self.sealed.len() + 1
    }

    /// A relation's extent size — **one** map lookup, whatever the depth.
    ///
    /// The counter is the acceptance instrument for that claim
    /// ([S1a.6.8](../../../../docs/history/m1a_rust/README.md#s1a68--the-compile-cache-and-the-extent-counts)):
    /// it counts probes, and a fold over the layers would make it grow with
    /// `depth()`.
    pub fn n_facts_of(&self, rel: Symbol) -> usize {
        crate::counters::bump(|c| c.extent_probe += 1);
        self.n_by_rel.get(&rel).copied().unwrap_or(0) as usize
    }

    /// The participation index: facts with `value` in argument `slot` of
    /// `rel`.
    pub fn facts_with(&self, key: SlotKey) -> impl Iterator<Item = FactId> + '_ {
        // Hash once, then a bit test per layer instead of a lookup per layer:
        // a fork 24 layers deep asks 24 times for a key one or two of them
        // have, and the walk was 15.6 % of an exhaustive `zebra` after
        // T1a.6.3.0 made the lookup the common case.
        let at = bloom_at(&key);
        self.layers()
            .filter(move |l| {
                let may = l.bloom_may_have(at);
                debug_assert!(
                    may || !l.by_rel_slot_val.contains_key(&key),
                    "the Bloom filter reported a miss for a key the layer has"
                );
                may
            })
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

    /// How many rule-application facts the KB holds — the version counter
    /// `Engine::compile_all` skips its walk on. See `Layer::rule_apps`.
    pub fn n_rule_apps(&self) -> usize {
        self.layers().map(|l| l.rule_apps as usize).sum()
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
        // A `Symbol` is a dense `u32`, so the dedup is a bit test rather than
        // a hash — the same set in the same first-seen order, at the cost of
        // one `Vec<u64>` (T1a.6.4.2). It is called once per hypothesis-
        // generation pass, and the deeper the fork the more layers it walks.
        let mut seen = crate::bitset::BitSet::new();
        for layer in self.layers() {
            for name in layer.names.keys() {
                if seen.insert(name.0) {
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
        // Attempts, not writes: the dedup hit is the interesting half — it is a
        // rule re-deriving something, which is what `saturator._binding_key`
        // exists to avoid and what a beta-memory would remove outright.
        crate::counters::bump(|c| c.fact_insert += 1);
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

    /// Does a fact with this head **activate** a rule?
    ///
    /// Both registries that a fact can activate, and only those. A saturation
    /// rule and an obligation rule find their activators the same way — by
    /// name, in this index — because M1d
    /// [S1d.2.3](../../../../docs/history/m1d_satisfiability/README.md#s1d23--the-form)
    /// split them into two registries over **one name-space**: which pass
    /// walks a rule is not a fact about its activator. `hrules` are absent
    /// and stay absent — a generic hrule takes its activators from the query's
    /// `:hrules` keyword and never from the store, because an hrule activator
    /// steers the search.
    fn is_rule_app(&self, rel: Symbol) -> bool {
        self.program.rules.contains(rel) || self.program.obligations.contains(rel)
    }

    /// Append one fact to every reverse index — the incremental half of
    /// [`Kb::rebuild_indexes`], and it must agree with it exactly.
    pub fn index_fact(&mut self, terms: &Terms, id: FactId) {
        let (rel, args) = terms.facts.get(id);
        let is_rule_app = self.is_rule_app(rel);
        let not = terms.kernel.not;

        self.top.by_rel.entry(rel).or_default().push(id);
        *self.n_by_rel.entry(rel).or_default() += 1;
        if is_rule_app {
            self.top.rule_apps_by_rule.entry(rel).or_default().push(id);
            self.top.rule_apps += 1;
        }
        for (slot, value) in args.iter().enumerate() {
            // The join-key types only: a nested fact has no single value of
            // its own to key on…
            if value.tag() != Tag::Fact {
                let key = SlotKey::direct(rel, slot as u16, *value);
                self.top.by_rel_slot_val.entry(key).or_default().push(id);
                self.top.bloom_add(&key);
            } else if let Some(nested) = value.as_fact() {
                // …so key one level in instead (T1a.6.3.0). Only one level:
                // the corpus's nesting is `(not (R …))` and a second would
                // cost entries nothing asks for.
                for (inner, deep) in terms.facts.args(nested).iter().enumerate() {
                    if deep.tag() != Tag::Fact {
                        let key = SlotKey {
                            rel,
                            slot: slot as u16,
                            inner: inner as u16,
                            value: *deep,
                        };
                        self.top.by_rel_slot_val.entry(key).or_default().push(id);
                        self.top.bloom_add(&key);
                    }
                }
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
        self.n_by_rel = layer
            .by_rel
            .iter()
            .map(|(&rel, ids)| (rel, ids.len() as u32))
            .collect();
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
            let is_rule_app = self.is_rule_app(rel);
            layer.by_rel.entry(rel).or_default().push(id);
            if is_rule_app {
                layer.rule_apps_by_rule.entry(rel).or_default().push(id);
                layer.rule_apps += 1;
            }
            layer.names.entry(rel).as_head.push(id);
            for (slot, value) in args.iter().enumerate() {
                if value.tag() != Tag::Fact {
                    let key = SlotKey::direct(rel, slot as u16, *value);
                    layer.by_rel_slot_val.entry(key).or_default().push(id);
                    let (word, bit) = bloom_at(&key);
                    layer.slot_bloom[word] |= bit;
                } else if let Some(nested) = value.as_fact() {
                    for (inner, deep) in terms.facts.args(nested).iter().enumerate() {
                        if deep.tag() != Tag::Fact {
                            let key = SlotKey {
                                rel,
                                slot: slot as u16,
                                inner: inner as u16,
                                value: *deep,
                            };
                            layer.by_rel_slot_val.entry(key).or_default().push(id);
                            let (word, bit) = bloom_at(&key);
                            layer.slot_bloom[word] |= bit;
                        }
                    }
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
    /// The four index-entry totals `cli/saturate.py`'s snapshot reports —
    /// `facts_by_relation`, `facts_by_rel_slot_val`, `rule_apps_by_rule`,
    /// `rule_apps_on_relation`, in that order.
    ///
    /// ein.py holds one flat dict per index, so its `sum(len(v) for v in …)`
    /// ranges over the whole KB. Here the layers are summed through the
    /// materialised view, which is the same set of entries with the
    /// copy-on-write seams closed.
    ///
    /// **`facts_by_rel_slot_val` counts the `DIRECT` postings only.** Since
    /// T1a.6.3.0 the index also holds keys *inside* a nested argument, which
    /// ein.py's does not — 897 postings against 743 on `zebra2-hints`. This
    /// line is a report about the **knowledge base**: how many `(relation,
    /// slot, value)` join keys its facts produce, which is a property of the
    /// data and identical in both engines. How ein.rs additionally indexes
    /// those facts to answer a query faster is not what a reader of
    /// `saturate`'s snapshot is being told, and letting it leak in would make
    /// 43 corpus entries disagree about the *data* because the *engine*
    /// changed.
    pub fn index_sizes(&self) -> [usize; 4] {
        let m = self.materialise();
        [
            // determinism-ok: a sum over the values; no order reaches it.
            m.by_rel.values().map(Vec::len).sum(),
            m.by_rel_slot_val
                .iter()
                .filter(|(k, _)| k.inner == SlotKey::DIRECT)
                .map(|(_, v)| v.len())
                .sum(),
            // determinism-ok: a sum over the values; no order reaches it.
            m.rule_apps_by_rule.values().map(Vec::len).sum(),
            // determinism-ok: a sum over the values; no order reaches it.
            m.rule_apps_on_rel.values().map(Vec::len).sum(),
        ]
    }

    /// One name's index entry, for the snapshot's participation columns.
    pub fn name_entry(&self, name: Symbol) -> (usize, usize) {
        (
            self.name_as_head(name).count(),
            self.name_as_arg(name).count(),
        )
    }

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
        flat.diff(&rebuilt)?;
        self.check_extent_counts()
    }

    /// [`Kb::rebuild_indexes`], but keeping another KB's **load-time**
    /// rules-by-relation snapshot instead of recomputing one.
    ///
    /// That map is taken once, at the end of `load`, and then shared by
    /// reference — a property fact added during saturation deliberately does
    /// not extend it (ein.py's contract, and the reason `rules_by_relation` is
    /// an `Arc` rather than a maintained index). Recomputing it from a
    /// saturated fact set therefore produces a *different, larger* map than the
    /// KB ever had, which is what a `.einb` of a saturated KB would otherwise
    /// come back with: `rule_apps_on_rel` has grown, so every relation a
    /// derived rule application mentions gains rules the original never
    /// associated with it.
    ///
    /// `template` is a fresh load of the same program, which is where the
    /// snapshot comes from in the first place.
    pub fn rebuild_indexes_from(&mut self, terms: &Terms, template: &Kb) {
        self.rebuild_indexes(terms);
        self.rules_by_relation = Arc::clone(&template.rules_by_relation);
    }

    /// `n_by_rel` against a full walk of the layer stack — the invariant the
    /// O(1) [`Kb::n_facts_of`] rests on, checked rather than argued
    /// ([S1a.6.8](../../../../docs/history/m1a_rust/README.md#s1a68--the-compile-cache-and-the-extent-counts)
    /// T1a.6.8.2). Part of `check_layering` because that is where every
    /// KB-shape fixture already asks whether the layer stack still adds up.
    pub fn check_extent_counts(&self) -> Result<(), String> {
        let mut walked: FxHashMap<Symbol, u32> = FxHashMap::default();
        for layer in self.layers() {
            for (&rel, ids) in &layer.by_rel {
                *walked.entry(rel).or_default() += ids.len() as u32;
            }
        }
        let maintained: FxHashMap<Symbol, u32> = self
            .n_by_rel
            .iter()
            .filter(|&(_, &n)| n > 0)
            .map(|(&k, &v)| (k, v))
            .collect();
        if walked != maintained {
            return Err(format!(
                "n_by_rel disagrees with a walk of the layer stack: \
                 maintained {maintained:?}, walked {walked:?}"
            ));
        }
        Ok(())
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

    /// Copy every fork-region record this KB still cites into the arena
    /// proper, and rewrite the citations — so the KB outlives the fork that
    /// built it.
    ///
    /// The one path in the engine that keeps a fork is `Run::record_node`: a
    /// solution node snapshots the fork's KB, which is why
    /// [`crate::prov::ProvArena::discard_fork`] cannot simply run on every
    /// entering. Everything else — a dead commitment, an alive one promoted to
    /// the next layer — drops the fork and its records with it.
    ///
    /// **What it does not clone.** A layer is rewritten only if it cites a
    /// fork record, so root's shared sealed layers, which by construction cite
    /// none, are left in the `Arc` they arrived in. In practice one layer is
    /// touched: the fork's own top.
    pub fn promote_provenance(&mut self, terms: &mut Terms) {
        if terms.provs.fork_is_empty() {
            return;
        }
        let mut cited: FxHashSet<ProvId> = FxHashSet::default();
        for l in self.layers() {
            // The order the promotion assigns ids in comes off the fork's
            // push order, not off this.
            // determinism-ok: a *set* of citations.
            cited.extend(l.primary.values().copied().filter(|p| p.is_fork()));
            cited.extend(
                l.alts
                    .values()
                    // determinism-ok: the same set through the other map.
                    .flat_map(|a| a.iter().copied())
                    .filter(|p| p.is_fork()),
            );
        }
        if cited.is_empty() {
            return;
        }
        let map = terms.provs.promote(&cited);
        for i in 0..self.sealed.len() {
            if !self.sealed[i].cites_fork_provenance() {
                continue;
            }
            Arc::make_mut(&mut self.sealed[i]).rewrite_provenance(&map);
        }
        self.top.rewrite_provenance(&map);
    }

    /// Does anything this KB believes cite a record that died with a fork?
    ///
    /// Always `false` on a KB the engine has finished with — which is the
    /// claim, and which `ein-infer/tests/provenance.rs` asks of every corpus
    /// file that solves, of root and of every recorded solution.
    pub fn cites_fork_provenance(&self) -> bool {
        self.layers().any(Layer::cites_fork_provenance)
    }

    /// Believe an already-interned proposition, with the derivation a file
    /// recorded for it.
    ///
    /// [`Kb::add_fact`]'s loader path takes `(rel, args)` because it is
    /// answering "does this proposition have a number yet"; the `.einb` reader
    /// already has the number — it re-interned the whole table in id order to
    /// get it — so this is `push_fact`, exposed for the one caller that is
    /// replaying a fact list rather than deriving one. The list it replays is
    /// already deduped, and a file whose list is not is rejected by the reader
    /// before it gets here.
    pub fn restore_fact(&mut self, id: FactId, prov: Option<ProvId>) {
        debug_assert!(!self.contains(id), "a replayed fact list must be deduped");
        self.push_fact(id, prov);
    }

    /// Reinstate a fact's alternative-justification list verbatim.
    ///
    /// The `.einb` reader's counterpart to [`Kb::record_justification`], which
    /// is the wrong door for it: that one applies the policy — the duplicate
    /// test, the shortest-first insert,
    /// [`MAX_ALT_JUSTIFICATIONS`] — and a list read back out of a file has
    /// already been through it. Replaying it through the policy would sort a
    /// sorted list and cap a capped one, which is a no-op on every list the
    /// engine writes and a silent edit on any list it ever stops writing.
    pub fn restore_alternatives(&mut self, fact: FactId, alts: Box<[ProvId]>) {
        if alts.is_empty() {
            return;
        }
        self.top.alts.insert(fact, alts);
    }

    /// Field-by-field comparison against another KB — what "T1-identical"
    /// means when a round trip has to prove it
    /// ([design/10 §6](../../../../docs/history/m1a_rust/design/10_binary_format.md)).
    ///
    /// Both sides are materialised first, so the answer is about *content* and
    /// not about how many forks each one has been through — which is the same
    /// equivalence [`Kb::check_layering`] rests on. The registries are compared
    /// by identity rather than by value: two KBs over one `Arc<Program>` share
    /// it, and a reader that rebuilt the registries from `PROGRAM` gets a
    /// different `Arc` whose *contents* the fact set and the indexes already
    /// witness.
    pub fn diff(&self, other: &Kb) -> Result<(), String> {
        self.materialise().diff(&other.materialise())?;
        let (mine, theirs) = (
            self.nogoods.read().expect("no writer panicked"),
            other.nogoods.read().expect("no writer panicked"),
        );
        if mine.len() != theirs.len() {
            return Err(format!(
                "no-good counts: {} vs {}",
                mine.len(),
                theirs.len()
            ));
        }
        // determinism-ok: a membership test per clause; no order reaches the answer.
        for clause in mine.iter() {
            if !theirs.contains(clause) {
                return Err(format!("no-good clause {clause:?} is missing"));
            }
        }
        Ok(())
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
    fn the_participation_index_keys_the_join_types_and_one_level_in() {
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

        // A nested-fact argument still has no key of its *own* — there is no
        // single value at that position…
        assert_eq!(kb.facts_with(SlotKey::direct(not, 0, inner)).count(), 0);
        // …but since T1a.6.3.0 its contents do, one level in, which is what a
        // `(not (co-located ?x ?y))` premise probes with `?x` bound.
        assert_eq!(
            kb.facts_with(SlotKey {
                rel: not,
                slot: 0,
                inner: 0,
                value: norwegian
            })
            .collect::<Vec<_>>(),
            vec![f]
        );
        assert_eq!(
            kb.facts_with(SlotKey {
                rel: not,
                slot: 0,
                inner: 1,
                value: one
            })
            .collect::<Vec<_>>(),
            vec![f]
        );
        // A value that is not there is still not there.
        assert_eq!(
            kb.facts_with(SlotKey {
                rel: not,
                slot: 0,
                inner: 0,
                value: one
            })
            .count(),
            0
        );
        assert_eq!(
            kb.facts_with(SlotKey::direct(co_located, 1, one)).count(),
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

    /// S1a.6.8 T1a.6.8.2 — the extent count is one probe at any depth.
    ///
    /// The boundary asks `n_facts_of` once per watched relation per parked
    /// candidate — 644 166 times on an exhaustive `zebra2` — and the search
    /// reaches depth 35, so folding the answer over the layer stack was a
    /// 35× multiplier on 9.5 % of the run. The probe counter is what makes
    /// "one lookup" checkable rather than asserted: the previous
    /// implementation would have made it grow with `depth()`.
    #[test]
    fn n_facts_of_costs_one_probe_at_any_depth() {
        let (mut terms, mut kb) = fixture();
        let rel = sym(&mut terms, "co-located");
        let other = sym(&mut terms, "next-to");
        add(&mut kb, &mut terms, "co-located", &["Norwegian", "House-1"]);

        // One fork per level, one fact per level, forty levels deep.
        let mut deep = kb.fork();
        for i in 0..40 {
            let mut next = deep.fork();
            add(
                &mut next,
                &mut terms,
                "co-located",
                &[&format!("P{i}"), &format!("House-{i}")],
            );
            assert_eq!(next.n_facts_of(rel), i + 2, "depth {}", next.depth());
            assert_eq!(next.n_facts_of(other), 0);
            next.check_extent_counts()
                .expect("counts agree with a walk");
            deep = next;
        }
        // One layer per fact: `seal` skips an empty top, so the forty forks
        // add forty layers on top of root's one.
        assert_eq!(deep.depth(), 41, "the test is not actually deep");

        // Shallow and deep pay the same, which is the whole claim. A no-op
        // without `--features counters`, where both readings are zero and the
        // assertion is vacuously true — as it is for every other counter.
        crate::counters::reset();
        for _ in 0..100 {
            kb.n_facts_of(rel);
        }
        let shallow = crate::counters::snapshot().extent_probe;
        crate::counters::reset();
        for _ in 0..100 {
            deep.n_facts_of(rel);
        }
        assert_eq!(crate::counters::snapshot().extent_probe, shallow);
        if cfg!(feature = "counters") {
            assert_eq!(shallow, 100, "one probe per call, not one per layer");
        }
    }

    /// The batch path — `add_fact` without `index_fact`, then one rebuild —
    /// leaves the same counts the incremental path would have.
    #[test]
    fn a_rebuild_reconstructs_the_extent_counts() {
        let (mut terms, mut kb) = fixture();
        let rel = sym(&mut terms, "co-located");
        for i in 0..5 {
            let args = [
                Value::sym(sym(&mut terms, &format!("P{i}"))),
                Value::sym(sym(&mut terms, &format!("House-{i}"))),
            ];
            kb.add_fact(&mut terms, rel, &args, None).expect("room");
        }
        // Un-indexed, so both readings are still 0 — the invariant is that
        // they agree, not that either is right before the rebuild.
        assert_eq!(kb.n_facts_of(rel), 0);
        kb.rebuild_indexes(&terms);
        assert_eq!(kb.n_facts_of(rel), 5);
        kb.check_extent_counts().expect("counts agree with a walk");
    }
}
