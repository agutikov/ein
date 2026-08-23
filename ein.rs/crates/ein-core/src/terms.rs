//! `Terms` — the three intern tables that decide what exists.
//!
//! Symbols, integers and facts are interned together because they are asked
//! together: a fact's argument is a `Value` whose tag says which of the three
//! tables to read, and rendering or ordering one needs all three
//! ([design/03](../../../../plans/m1a_rust/design/03_data_model.md) §§2–4).
//!
//! ein.py reaches the same information through a `_kb` back-pointer on every
//! entity. This port has none: an accessor takes `&Terms` (and, where belief
//! matters, `&Kb`) explicitly, which is what makes the caveat ein.py
//! documents — that a shared entity's back-pointer sees the *root*'s facts, so
//! `Relation.facts` on a fork answers for the wrong KB — evaporate rather than
//! need reproducing.
//!
//! Interning is `&mut`, reading is `&`. Nothing here knows about belief: a
//! `FactId` exists whether or not any KB holds the proposition true.

use crate::facts::{FactId, FactStore};
use crate::intern::{Interner, Overflow, Symbol};
use crate::prov::ProvArena;
use crate::pyrepr::{self, PyValue};
use crate::value::{IntId, IntPool, Tag, Value};
use std::cmp::Ordering;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering as Atomics};

/// A table the committing thread owns outright and lends to workers for the
/// duration of a fanned-out layer.
///
/// Two states and one rule: **a table that is lent cannot grow.** That is the
/// whole of what [P1a.7](../../../../plans/m1a_rust/p1a.7_parallelism/README.md)
/// needs from the interner and the fact store, and it is why neither takes a
/// lock — [shared_state.md §4](../../../../plans/m1a_rust/p1a.7_parallelism/shared_state.md#4-what-this-chooses).
///
/// **Why not simply `Arc<T>` in both states.** Growing an `Arc`'s contents
/// means [`Arc::get_mut`], whose uniqueness test is a locked read-modify-write
/// on the weak count. Interning is not rare — `branching/06 -e` makes 2 318 815
/// calls, of which 417 assign — so paying an atomic per call to discover that
/// nobody is sharing costs **4 %** of that file's solve, measured against the
/// `Arc`-in-both-states version this replaced. Two states pay a branch
/// instead, and it is perfectly predicted because the state is constant for
/// the whole of a layer.
#[derive(Debug)]
pub enum Table<T> {
    /// Nobody else holds it: read it, and grow it.
    Own(T),
    /// Lent to workers: read it by `&` from any thread, grow it nowhere.
    Shared(Arc<T>),
}

impl<T> std::ops::Deref for Table<T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            Table::Own(t) => t,
            Table::Shared(t) => t,
        }
    }
}

impl<T: Default> Table<T> {
    /// The growable half, or `None` while the table is lent.
    pub fn own_mut(&mut self) -> Option<&mut T> {
        match self {
            Table::Own(t) => Some(t),
            Table::Shared(_) => None,
        }
    }

    /// Lend it — idempotent, so a layer can ask without knowing whether an
    /// earlier one already did.
    fn share(&mut self) {
        if let Table::Own(t) = self {
            *self = Table::Shared(Arc::new(std::mem::take(t)));
        }
    }

    /// A second holder. Only a lent table has one to give, and asking an owned
    /// one is a fan-out that forgot to lend.
    fn holder(&self) -> Table<T> {
        match self {
            Table::Own(_) => panic!("a worker view was asked for before Terms::share"),
            Table::Shared(t) => Table::Shared(Arc::clone(t)),
        }
    }

    /// Take it back, or say why it cannot be taken.
    ///
    /// `false` means a worker's view is still alive. That is a bug in the
    /// fan-out rather than a condition to recover from — the alternative is a
    /// table that has silently stopped growing — so the caller panics; this
    /// returns rather than panicking so the message can name all three tables
    /// at once.
    fn reclaim(&mut self) -> bool {
        let Table::Shared(t) = self else {
            return true;
        };
        let t = std::mem::take(t);
        match Arc::try_unwrap(t) {
            Ok(t) => {
                *self = Table::Own(t);
                true
            }
            Err(t) => {
                *self = Table::Shared(t);
                false
            }
        }
    }
}

#[derive(Debug)]
pub struct Terms {
    /// The three intern tables, each [`Table::Own`] until a fanned-out layer
    /// lends it ([`Terms::share`]) and [`Table::Own`] again when the layer
    /// takes it back.
    ///
    /// A lent table is readable from every thread and growable from none, so
    /// *sharing is what forbids growing* — no lock, no shard, and the
    /// invariant that makes it affordable is the one
    /// [shared_state.md §3](../../../../plans/m1a_rust/p1a.7_parallelism/shared_state.md#3-the-interner-does-not-grow-during-a-search--now)
    /// measured and `ein-infer/tests/interning.rs` keeps: nothing interns a
    /// name during a search.
    pub syms: Table<Interner>,
    pub ints: Table<IntPool>,
    pub facts: Table<FactStore>,
    /// Derivation records. Global for the same reason the fact store is —
    /// see [`crate::prov`] — except for the fork region, which is **not**
    /// shared, because it is the one structure a worker really writes.
    pub provs: ProvArena,
    /// The kernel vocabulary, interned up front so a comparison against it is
    /// an integer compare and no hot path needs a `&mut Terms` to ask.
    pub kernel: Kernel,
    /// Was an interning call refused because the tables are lent?
    ///
    /// **The single signal the fan-out reads.** An entering that needed to
    /// number something cannot be finished on a worker, and the engine has a
    /// dozen places that would have numbered it — the blind enumerator, the
    /// lookahead kill cache, an `hrule` activator, a rule's conclusion. Rather
    /// than teach each of them a new error to propagate, each stops what it is
    /// doing and this says so: whatever the entering produced afterwards is
    /// **not** the entering the sequential engine would have run, so the
    /// fan-out discards it and re-runs it on the committing thread
    /// ([shared_state.md §4.2](../../../../plans/m1a_rust/p1a.7_parallelism/shared_state.md#4-what-this-chooses)).
    ///
    /// Atomic only because [`Terms`] is `Sync`; it is written and read by the
    /// one thread that owns this `Terms`, so `Relaxed` is the whole ordering
    /// requirement.
    refused: AtomicBool,
}

impl Default for Terms {
    fn default() -> Self {
        Terms::new()
    }
}

/// The rule-body / ⊥ structural primitives: the kernel's reserved
/// non-relation vocabulary. Not relations (their truth is not data in the KB)
/// and not predicates (computed guards) — the calculus the compiler, matcher
/// and contradiction detector interpret directly.
///
/// `open` / `forall` used to live here as compile-time sugar; since P1.8
/// S1.5.9 they are ordinary `(macro …)` declarations in `std.macro`, expanded
/// at load time, so they are no longer kernel vocabulary.
pub const STRUCTURAL: [&str; 5] = ["absent", "and", "false", "not", "or"];

/// The built-in computed predicates.
pub const PREDICATES: [&str; 2] = ["eq", "neq"];

/// Names a declarator — `rule` / `hrule` / `relation` / `macro` — may not
/// **bind**.
///
/// The grammar already SYMBOL-excludes `not` / `and` / `or` / `neq` / `rule` /
/// `hrule` / `query` / `config` / `trace` / `macro` / `import`, so what still
/// reaches the loader as a declared name is the structural primitives, the
/// predicates, and `relation` — kept a plain SYMBOL so `(relation ?R ?A ?B)`
/// stays a legal pattern. This is about *binding* a name: a fact may still
/// have a reserved head, such as a stored `(not X)` octagon.
pub const RESERVED: [&str; 8] = [
    "absent", "and", "eq", "false", "neq", "not", "or", "relation",
];

pub fn is_predicate(name: &str) -> bool {
    PREDICATES.contains(&name)
}

pub fn is_reserved(name: &str) -> bool {
    RESERVED.contains(&name)
}

/// The engine's **own** names — the ones it writes into a KB rather than reads
/// out of a program.
///
/// Four are provenance rule-names for a derivation no rule performed
/// ([`crate::prov`]), one marks a relation the kernel closes under arg-swap,
/// one names the synthetic rule a `(query …)` goal compiles to, one is the
/// `(__closed__ R)` marker, and the last two are the mirror firing's binding
/// variables. Not a style list: together with
/// `ein_ir::from_ir::intern_program_names` — which closes a *rule's* argument
/// constants at load — **these eight are why nothing interns a name during a
/// search**, measured over the whole corpus by
/// `ein-infer/tests/interning.rs`.
///
/// Why that matters is [P1a.7](../../../../plans/m1a_rust/p1a.7_parallelism/README.md)'s
/// [S1a.7.1](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.1_sync_shared_state.md):
/// [`Interner::text`](crate::Interner::text) hands out a `&str` borrowed from
/// the arena, which no lock can do, so a shareable interner is one that does
/// not **grow** while it is shared, and the search is where it would be
/// shared. Three of the eight were growing it there
/// ([shared_state.md](../../../../plans/m1a_rust/p1a.7_parallelism/shared_state.md) §3);
/// the other five arrive during root saturation, which is single-threaded —
/// they are here because `__symmetric__` was interned by every
/// `Saturator::new` and [`MIRROR_A`] / [`MIRROR_B`] by every mirror firing,
/// which is a hash lookup for a constant.
pub const ENGINE: [&str; 8] = [
    FORCED_POSITIVE,
    LOOKAHEAD_DIES,
    MONOTONIC_UNCONDITIONAL,
    QUERY_RULE,
    CLOSED,
    SYMMETRIC,
    MIRROR_A,
    MIRROR_B,
];

/// The forced-positive cascade's promotion — a fact root writes because every
/// other value of its slot is excluded.
pub const FORCED_POSITIVE: &str = "<forced-positive>";
/// The kill-cache `(not h)` a candidate that dies under one-step lookahead
/// leaves behind.
pub const LOOKAHEAD_DIES: &str = "<lookahead-dies-immediately>";
/// The singleton writeback's `(not h)` — a layer-1 death, generalised.
pub const MONOTONIC_UNCONDITIONAL: &str = "<monotonic-unconditional>";
/// The synthetic rule a `(query :goal …)` compiles to, so a goal binding has a
/// rule name to be reported under.
pub const QUERY_RULE: &str = "<query>";
/// `(__closed__ R)` — R is a relation no rule can positively conclude.
pub const CLOSED: &str = "__closed__";
/// `(__symmetric__ R)` — R's extension is closed under arg-swap by the kernel.
pub const SYMMETRIC: &str = "__symmetric__";
/// The two binding-variable names the native mirror reports its firing under.
pub const MIRROR_A: &str = "a";
pub const MIRROR_B: &str = "b";

/// The names the kernel itself knows. Interning them first costs nothing:
/// symbol ids are never observable, so their assignment order is free.
#[derive(Clone, Copy, Debug)]
pub struct Kernel {
    /// The head of a stored negation, `(not X)`.
    pub not: Symbol,
    /// `KERNEL_META_RELATIONS` — the two heads that categorise as relations
    /// whether or not a puzzle declares them.
    pub relation: Symbol,
    pub rule: Symbol,
    pub hrule: Symbol,
    pub macro_: Symbol,
    pub query: Symbol,
    pub config: Symbol,
    pub trace: Symbol,
    pub import: Symbol,
    pub and: Symbol,
    pub or: Symbol,
    pub eq: Symbol,
    pub neq: Symbol,
    pub absent: Symbol,
    pub r#false: Symbol,
    /// `=`, which the grammar keeps as a *named* terminal so it survives token
    /// filtering and arrives as an ordinary atom.
    pub equals: Symbol,
    /// The synthetic heads the parser gives `()` and a parameter list.
    pub empty: Symbol,
    pub params: Symbol,
    /// [`ENGINE`] — the names the engine writes rather than reads.
    pub forced_positive: Symbol,
    pub lookahead_dies: Symbol,
    pub monotonic_unconditional: Symbol,
    pub query_rule: Symbol,
    pub closed: Symbol,
    pub symmetric: Symbol,
    pub mirror_a: Symbol,
    pub mirror_b: Symbol,
}

impl Terms {
    pub fn new() -> Self {
        let mut syms = Interner::new();
        let mut intern = |s: &str| syms.intern(s).expect("room for the kernel names");
        let kernel = Kernel {
            not: intern("not"),
            relation: intern("relation"),
            rule: intern("rule"),
            hrule: intern("hrule"),
            macro_: intern("macro"),
            query: intern("query"),
            config: intern("config"),
            trace: intern("trace"),
            import: intern("import"),
            and: intern("and"),
            or: intern("or"),
            eq: intern("eq"),
            neq: intern("neq"),
            absent: intern("absent"),
            r#false: intern("false"),
            equals: intern("="),
            empty: intern("@empty"),
            params: intern("@params"),
            forced_positive: intern(FORCED_POSITIVE),
            lookahead_dies: intern(LOOKAHEAD_DIES),
            monotonic_unconditional: intern(MONOTONIC_UNCONDITIONAL),
            query_rule: intern(QUERY_RULE),
            closed: intern(CLOSED),
            symmetric: intern(SYMMETRIC),
            mirror_a: intern(MIRROR_A),
            mirror_b: intern(MIRROR_B),
        };
        Terms {
            syms: Table::Own(syms),
            ints: Table::Own(IntPool::new()),
            facts: Table::Own(FactStore::new()),
            provs: ProvArena::new(),
            kernel,
            refused: AtomicBool::new(false),
        }
    }

    // ── Sharing ────────────────────────────────────────────────────

    /// A worker's view of these tables: the same three interners, lent, and a
    /// **record region of its own**.
    ///
    /// This is the whole of what
    /// [S1a.7.2](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.2_parallel_enterings.md)
    /// T1a.7.2.1 hands a worker, and the shape is
    /// [shared_state.md §4](../../../../plans/m1a_rust/p1a.7_parallelism/shared_state.md#4-what-this-chooses)'s
    /// four decisions in one type:
    ///
    /// - the **interner and the integer pool** are shared by `&`, because they
    ///   do not grow during a search — 0 of the 90 corpus files that solve;
    /// - the **fact store** is shared by `&` too, because a search assigns 41
    ///   to 417 ids against 5.8–26 M borrow-returning reads, and *per
    ///   entering* four of six workloads assign none at all. A worker that
    ///   would have assigned one gets [`Overflow::Shared`] and hands the
    ///   entering back;
    /// - the **provenance arena** is per-worker, because it is the one with a
    ///   real write rate — 100 % of enterings, 2 135 093 records on
    ///   `features/01 -e` — and because a fork's records die with the fork;
    /// - and there is **no lock**, so there is no protocol to model.
    ///
    /// The view borrows nothing, so it is `Send` and can cross into a worker;
    /// what stops the committing thread interning while a view is alive is
    /// that the tables are [`Table::Shared`], not a borrow. Reads see the
    /// tables as they stood when they were lent, which is the same thing as
    /// saying they see them as they are: nothing may write them meanwhile.
    /// Lend the three intern tables out, so [`Terms::worker`] can hand a view
    /// to each worker. Idempotent, and the point at which this `Terms` stops
    /// being able to intern.
    pub fn share(&mut self) {
        self.syms.share();
        self.ints.share();
        self.facts.share();
    }

    /// Take the tables back, and be able to intern again.
    ///
    /// Panics if a worker view is still alive: what a leaked view would cost
    /// is not a crash but a table that has silently stopped growing, and an
    /// entering that then hands itself back **for ever**.
    pub fn reclaim(&mut self) {
        assert!(
            self.try_reclaim(),
            "Terms::reclaim while a worker still holds a view — every \
             Terms::worker has to be dropped before the layer closes"
        );
    }

    /// The same, reporting rather than asserting.
    ///
    /// One caller: [`Lent`]'s `Drop` while the thread is already panicking,
    /// where the assertion above would be a *second* panic and would abort the
    /// process with the original message lost.
    pub fn try_reclaim(&mut self) -> bool {
        self.syms.reclaim() & self.ints.reclaim() & self.facts.reclaim()
    }

    /// Lend the tables for the duration of a fan-out, and take them back when
    /// the guard drops.
    ///
    /// [`Terms::share`] and [`Terms::reclaim`] have to come in pairs, and the
    /// window between them is the one place a `?`, a `return` or a panic would
    /// leave the tables lent — which is [`Terms::reclaim`]'s own warning read
    /// from the other side: not a crash, but a `Terms` that has silently
    /// stopped growing and an entering that hands itself back for ever. The
    /// pairing is structural here rather than a rule someone has to keep
    /// ([S1a.7.5](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.5_jobs_contract.md)
    /// T1a.7.5.6).
    pub fn lend(&mut self) -> Lent<'_> {
        self.share();
        Lent { terms: self }
    }

    /// Is this `Terms` lending its tables — would an interning call have to
    /// hand its entering back?
    pub fn is_shared(&self) -> bool {
        matches!(self.facts, Table::Shared(_))
    }

    /// Did anything ask this `Terms` to number something it could not — see
    /// the field.
    pub fn refused(&self) -> bool {
        self.refused.load(Atomics::Relaxed)
    }

    /// Forget a refusal. The fan-out has re-run the entering that caused it.
    pub fn clear_refused(&self) {
        self.refused.store(false, Atomics::Relaxed);
    }

    fn refuse<T>(&self) -> Result<T, Overflow> {
        self.refused.store(true, Atomics::Relaxed);
        Err(Overflow::Shared)
    }

    /// One worker's view — call [`Terms::lend`] first.
    pub fn worker(&self) -> Terms {
        Terms {
            syms: self.syms.holder(),
            ints: self.ints.holder(),
            facts: self.facts.holder(),
            provs: self.provs.share(),
            kernel: self.kernel,
            refused: AtomicBool::new(false),
        }
    }

    // ── Interning ──────────────────────────────────────────────────

    /// Assign `s` a [`Symbol`], or return the one it already has.
    ///
    /// On a [shared](Terms::share) view the assignment half is unavailable, so
    /// this is a lookup that can fail with [`Overflow::Shared`]. The *hit*
    /// path is unchanged and needs nothing but `&`, which is why a worker
    /// compiling a plan still works: `intern_program_names` interns every name
    /// a rule can name at load, so the compiler asks and never assigns.
    pub fn intern_text(&mut self, s: &str) -> Result<Symbol, Overflow> {
        match self.syms.own_mut() {
            Some(syms) => syms.intern(s),
            None => self.syms.get(s).map_or_else(|| self.refuse(), Ok),
        }
    }

    /// A textual argument — what ein.py's `_atomic_value` produces for an
    /// `Atom` (its `name`) and for a `String` (its `value`).
    ///
    /// That both collapse to one shape is a **semantic** fact about ein-lang,
    /// not an implementation detail: `(likes A "foo")` and `(likes A foo)` are
    /// the same fact today, and interning must keep them so.
    pub fn value_text(&mut self, s: &str) -> Result<Value, Overflow> {
        Ok(Value::sym(self.intern_text(s)?))
    }

    /// A `Var` in argument position — `?name`, as `_atomic_value` renders it.
    pub fn value_var(&mut self, name: &str) -> Result<Value, Overflow> {
        self.value_text(&format!("?{name}"))
    }

    /// A `Range` in argument position — `lo..hi`, or `lo..*` when unbounded.
    pub fn value_range(&mut self, low: &str, high: Option<&str>) -> Result<Value, Overflow> {
        match high {
            Some(high) => self.value_text(&format!("{low}..{high}")),
            None => self.value_text(&format!("{low}..*")),
        }
    }

    /// An integer literal, at any width. `text` need not be canonical.
    pub fn value_int(&mut self, text: &str) -> Result<Value, Overflow> {
        Ok(Value::int(self.intern_int(text)?))
    }

    /// See [`Terms::intern_text`] for what a shared view can and cannot do.
    pub fn intern_int(&mut self, text: &str) -> Result<IntId, Overflow> {
        match self.ints.own_mut() {
            Some(ints) => ints.intern(text),
            None => self.ints.get(text).map_or_else(|| self.refuse(), Ok),
        }
    }

    /// See [`Terms::intern_text`] for what a shared view can and cannot do.
    ///
    /// This is the one that matters: a fork *derives* propositions, and the
    /// handful a search numbers for the first time are numbered here. Four of
    /// the six measured workloads never reach the assigning branch from inside
    /// an entering at all.
    pub fn intern_fact(&mut self, rel: Symbol, args: &[Value]) -> Result<FactId, Overflow> {
        match self.facts.own_mut() {
            Some(facts) => facts.intern(rel, args),
            None => self
                .facts
                .probe(rel, args)
                .map_or_else(|| self.refuse(), Ok),
        }
    }

    /// The nested-fact argument for `(not X)` and friends: intern `X`, then
    /// tag its id.
    pub fn value_fact(&mut self, rel: Symbol, args: &[Value]) -> Result<Value, Overflow> {
        Ok(Value::fact(self.intern_fact(rel, args)?))
    }

    // ── Reading ────────────────────────────────────────────────────

    pub fn sym(&self, sym: Symbol) -> &str {
        self.syms.text(sym)
    }

    pub fn int_text(&self, id: IntId) -> &str {
        self.ints.text(id)
    }

    pub fn fact(&self, id: FactId) -> (Symbol, &[Value]) {
        self.facts.get(id)
    }

    pub fn probe_fact(&self, rel: Symbol, args: &[Value]) -> Option<FactId> {
        self.facts.probe(rel, args)
    }

    // ── Ordering ───────────────────────────────────────────────────

    /// Order two values the way ein.py's `sorted()` would — where ein.py can
    /// sort them at all.
    ///
    /// Python compares `str` to `str` and `int` to `int`; a mixed pair raises
    /// `TypeError`, and a `Fact` has no `__lt__` at all. `Value` is totally
    /// ordered by construction, so ein.rs answers where ein.py raises — the
    /// accepted divergence in
    /// [design/02](../../../../plans/m1a_rust/design/02_determinism_and_order.md) §5 H2,
    /// which fixes the cross-tag order as `Int < Sym < Fact`.
    pub fn cmp_semantic(&self, a: Value, b: Value) -> Ordering {
        match (a.tag(), b.tag()) {
            (Tag::Sym, Tag::Sym) => self
                .syms
                .rank(a.as_sym().expect("tagged Sym"))
                .cmp(&self.syms.rank(b.as_sym().expect("tagged Sym"))),
            (Tag::Int, Tag::Int) => self.ints.cmp_value(
                a.as_int().expect("tagged Int"),
                b.as_int().expect("tagged Int"),
            ),
            (Tag::Fact, Tag::Fact) => self.cmp_fact_semantic(
                a.as_fact().expect("tagged Fact"),
                b.as_fact().expect("tagged Fact"),
            ),
            (x, y) => tag_order(x).cmp(&tag_order(y)),
        }
    }

    /// Order two facts as `sorted()` orders their `(relation_name, args)`
    /// identity tuples: relation name first, then arguments element-wise,
    /// then — a shorter tuple that is a prefix of a longer one sorting first —
    /// by arity.
    ///
    /// This is the comparator `apriori.layer_1`'s `sorted(alive)` needs
    /// ([design/02](../../../../plans/m1a_rust/design/02_determinism_and_order.md) §3b).
    pub fn cmp_fact_semantic(&self, a: FactId, b: FactId) -> Ordering {
        let (a_rel, a_args) = self.facts.get(a);
        let (b_rel, b_args) = self.facts.get(b);
        self.syms
            .rank(a_rel)
            .cmp(&self.syms.rank(b_rel))
            .then_with(|| {
                for (x, y) in a_args.iter().zip(b_args.iter()) {
                    let ord = self.cmp_semantic(*x, *y);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
                a_args.len().cmp(&b_args.len())
            })
    }

    // ── Rendering ──────────────────────────────────────────────────

    /// The value as CPython would hold it, for the `repr()`-shaped output
    /// sites ([design/02](../../../../plans/m1a_rust/design/02_determinism_and_order.md) §7).
    pub fn py_value(&self, v: Value) -> PyValue {
        match v.tag() {
            Tag::Sym => PyValue::Str(self.sym(v.as_sym().expect("tagged Sym")).to_string()),
            Tag::Int => PyValue::Int(self.int_text(v.as_int().expect("tagged Int")).to_string()),
            Tag::Fact => self.py_fact(v.as_fact().expect("tagged Fact")),
        }
    }

    /// A whole fact as CPython would hold it — `Fact(relation_name=…,
    /// args=(…))`, whose `provenance` / `raw` / `loc` / `_kb` fields are
    /// `repr=False` and so never appear.
    pub fn py_fact(&self, id: FactId) -> PyValue {
        let (rel, args) = self.facts.get(id);
        PyValue::Fact {
            relation_name: self.sym(rel).to_string(),
            args: args.iter().map(|a| self.py_value(*a)).collect(),
        }
    }

    /// `({rel} {args…})` — the compact form the loader's derivation-cycle
    /// message and the derivation DAG's labels both build.
    ///
    /// The space after the relation name is unconditional, so a nullary fact
    /// renders as `(q )`; both ein.py sites build it with the same f-string.
    pub fn compact(&self, id: FactId) -> String {
        let (rel, args) = self.facts.get(id);
        let args: Vec<String> = args.iter().map(|a| self.display(*a)).collect();
        format!("({} {})", self.sym(rel), args.join(" "))
    }

    /// `str(value)` — what provenance bindings and the dumper's `_compact`
    /// print. A `str` prints as itself, an `int` as its canonical decimal
    /// text, and a `Fact` as its dataclass `repr` (a frozen dataclass has no
    /// `__str__` of its own).
    pub fn display(&self, v: Value) -> String {
        match v.tag() {
            Tag::Sym => self.sym(v.as_sym().expect("tagged Sym")).to_string(),
            Tag::Int => self.int_text(v.as_int().expect("tagged Int")).to_string(),
            Tag::Fact => pyrepr::repr(&self.py_fact(v.as_fact().expect("tagged Fact"))),
        }
    }
}

/// The tables, lent for the duration of one fan-out — [`Terms::lend`].
///
/// It carries the `&mut Terms` and hands out `&Terms`, which is exactly the
/// asymmetry the seam needs: workers may look up and may not assign, and the
/// borrow checker is what says so rather than a comment.
pub struct Lent<'a> {
    terms: &'a mut Terms,
}

impl Lent<'_> {
    /// What a worker is handed.
    pub fn get(&self) -> &Terms {
        self.terms
    }
}

impl Drop for Lent<'_> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            // A failed reclaim here means a view outlived the fan-out, which
            // is worth an assertion on the ordinary path and is worth nothing
            // on this one: a second panic while unwinding aborts the process
            // and takes the *first* panic's message with it, and that message
            // is the one somebody needs.
            let _ = self.terms.try_reclaim();
        } else {
            self.terms.reclaim();
        }
    }
}

/// `Int < Sym < Fact` — design/02 §5 H2's recommendation, which is *not* the
/// raw tag order (`Sym` is tag 0 because it is the overwhelmingly common
/// shape and the cheapest thing to leave untagged in a debugger).
fn tag_order(tag: Tag) -> u8 {
    match tag {
        Tag::Int => 0,
        Tag::Sym => 1,
        Tag::Fact => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// P1a.2–6 are single-threaded, but retrofitting `Send + Sync` onto the
    /// intern tables under
    /// [P1a.7](../../../../plans/m1a_rust/p1a.7_parallelism/README.md) would
    /// touch every call site, so the property is asserted from the start —
    /// which is what rules out an `Rc` or a `RefCell` creeping in later.
    #[test]
    fn the_intern_tables_stay_shareable_across_threads() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Terms>();
        assert_send_sync::<crate::Interner>();
        assert_send_sync::<crate::FactStore>();
        assert_send_sync::<crate::IntPool>();
    }

    #[test]
    fn atom_string_and_var_flatten_the_way_atomic_value_does() {
        let mut t = Terms::new();
        // `_atomic_value(Atom("foo"))` and `_atomic_value(String("foo"))` are
        // both the str "foo" — so `(likes A foo)` and `(likes A "foo")` are
        // one fact.
        let from_atom = t.value_text("foo").expect("room");
        let from_string = t.value_text("foo").expect("room");
        assert_eq!(from_atom, from_string);
        // A Var arg is its `?name` spelling, and is therefore an ordinary
        // symbol that collides with a literal atom of that name — as it does
        // in ein.py.
        assert_eq!(
            t.value_var("x").expect("room"),
            t.value_text("?x").expect("room")
        );
        assert_eq!(
            t.value_range("1", Some("5")).expect("room"),
            t.value_text("1..5").expect("room")
        );
        assert_eq!(
            t.value_range("1", None).expect("room"),
            t.value_text("1..*").expect("room")
        );
        // An int is *not* the atom that spells it.
        assert_ne!(
            t.value_int("7").expect("room"),
            t.value_text("7").expect("room")
        );
    }

    #[test]
    fn semantic_order_sorts_names_and_numbers_the_way_python_does() {
        let mut t = Terms::new();
        let zebra = t.value_text("zebra").expect("room");
        let apple = t.value_text("apple").expect("room");
        let two = t.value_int("2").expect("room");
        let ten = t.value_int("10").expect("room");
        assert_eq!(t.cmp_semantic(apple, zebra), Ordering::Less);
        // Numeric, not lexicographic — "10" < "2" as text.
        assert_eq!(t.cmp_semantic(two, ten), Ordering::Less);
        // Mixed: ein.py raises; ein.rs answers Int < Sym (design/02 §5 H2).
        assert_eq!(t.cmp_semantic(ten, apple), Ordering::Less);
        // Identity order would have put `zebra` first — it was interned first.
        assert_eq!(zebra.cmp_identity(apple), Ordering::Less);
    }

    #[test]
    fn facts_order_as_their_identity_tuples_do() {
        let mut t = Terms::new();
        let rel = t.intern_text("co-located").expect("room");
        let other = t.intern_text("adjacent").expect("room");
        let a = t.value_text("a").expect("room");
        let b = t.value_text("b").expect("room");
        let ab = t.intern_fact(rel, &[a, b]).expect("room");
        let aa = t.intern_fact(rel, &[a, a]).expect("room");
        let short = t.intern_fact(rel, &[a]).expect("room");
        let adjacent = t.intern_fact(other, &[b, b]).expect("room");
        let mut ids = vec![ab, aa, short, adjacent];
        ids.sort_by(|x, y| t.cmp_fact_semantic(*x, *y));
        assert_eq!(ids, vec![adjacent, short, aa, ab]);
    }

    #[test]
    fn display_matches_str_for_each_of_the_three_shapes() {
        let mut t = Terms::new();
        let s = t.value_text("House-1").expect("room");
        let i = t.value_int("007").expect("room");
        let rel = t.intern_text("co-located").expect("room");
        let nested = t.value_fact(rel, &[s, i]).expect("room");
        assert_eq!(t.display(s), "House-1");
        assert_eq!(t.display(i), "7");
        assert_eq!(
            t.display(nested),
            "Fact(relation_name='co-located', args=('House-1', 7))"
        );
    }
}
