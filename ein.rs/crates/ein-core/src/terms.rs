//! `Terms` — the two intern tables that decide what exists.
//!
//! Symbols and facts are interned together because they are asked together:
//! a fact's argument is a `Value` whose tag says which of the two tables to
//! read, and rendering or ordering one needs both
//! ([03-design/03-data-model.md](../../../../docs/ein/ein.rs/03-design/03-data-model.md) §§2–4).
//! There is no third: Ein has no integers (Q-M1e2.24), and a run of digits
//! is a name.
//!
//! No entity carries a back-pointer to these tables: an accessor takes
//! `&Terms` (and, where belief matters, `&Kb`) explicitly, so an entity shared
//! between a root and its forks cannot answer a question about one KB with
//! the other's facts.
//!
//! Interning is `&mut`, reading is `&`. Nothing here knows about belief: a
//! `FactId` exists whether or not any KB holds the proposition true.

use crate::facts::{FactId, FactStore};
use crate::intern::{Interner, Overflow, Symbol};
use crate::prov::ProvArena;
use crate::repr;
use crate::text;
use crate::value::{Tag, Value};
use std::cmp::Ordering;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering as Atomics};

/// A table the committing thread owns outright and lends to workers for the
/// duration of a fanned-out layer.
///
/// Two states and one rule: **a table that is lent cannot grow.** That is the
/// whole of what [03-design/08-parallelism.md](../../../../docs/ein/ein.rs/03-design/08-parallelism.md)
/// needs from the interner and the fact store, and it is why neither takes a
/// lock — [03-design/08-parallelism.md § 3](../../../../docs/ein/ein.rs/03-design/08-parallelism.md#3-what-must-be-sync-and-how).
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

/// A copy is the table's own, [`Table::Own`], whether the original is lent or
/// not: a copy is held by nobody else.
impl<T: Clone> Clone for Table<T> {
    fn clone(&self) -> Table<T> {
        Table::Own(T::clone(self))
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
    /// returns rather than panicking so the message can name both tables
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
    /// The two intern tables, each [`Table::Own`] until a fanned-out layer
    /// lends it ([`Terms::share`]) and [`Table::Own`] again when the layer
    /// takes it back.
    ///
    /// A lent table is readable from every thread and growable from none, so
    /// *sharing is what forbids growing* — no lock, no shard, and the
    /// invariant that makes it affordable is the one
    /// [03-design/08-parallelism.md § 3](../../../../docs/ein/ein.rs/03-design/08-parallelism.md#3-what-must-be-sync-and-how)
    /// measured and `ein-infer/tests/interning.rs` keeps: nothing interns a
    /// name during a search.
    pub syms: Table<Interner>,
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
    /// dozen places that would have numbered it — the blind enumerator, a
    /// hypothesis a rule concludes, a rule's conclusion. Rather
    /// than teach each of them a new error to propagate, each stops what it is
    /// doing and this says so: whatever the entering produced afterwards is
    /// **not** the entering the sequential engine would have run, so the
    /// fan-out discards it and re-runs it on the committing thread
    /// ([03-design/08-parallelism.md § 3](../../../../docs/ein/ein.rs/03-design/08-parallelism.md#3-what-must-be-sync-and-how)).
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

/// **A copy of every table**, each the copy's own: the names, the facts and
/// the derivation records under the same ids, the kernel's words where they
/// were. A caller that loads a program once and runs several things over it
/// runs each on a copy of the loaded `Terms` (the corpus sweeps, which load
/// a program once), and a copy interns from where the
/// original stood, so every id it hands out is the one the original would.
impl Clone for Terms {
    fn clone(&self) -> Terms {
        Terms {
            syms: self.syms.clone(),
            facts: self.facts.clone(),
            provs: self.provs.clone(),
            kernel: self.kernel,
            refused: AtomicBool::new(self.refused.load(Atomics::Relaxed)),
        }
    }
}

/// The rule-body / ⊥ structural primitives: the kernel's reserved
/// non-relation vocabulary. Not relations (their truth is not data in the KB)
/// and not predicates (computed guards) — the calculus the compiler, matcher
/// and contradiction detector interpret directly.
///
/// `unknown` and `forall` are not here: they are ordinary `(macro …)`
/// declarations in `std.macro`, expanded at load time, so they are no kernel
/// vocabulary.
///
/// `open` is **not** here. It is reserved (it may not be bound by a
/// declarator) but it is not a rule-body primitive: it appears only as an
/// `:assert` conclusion, the compiler never meets it in a `:match`, and the
/// contradiction detector does not read it. `(false)` lives here as a word
/// the grammar shapes, a fact of no argument: the detector does not read it
/// either, and a state that holds it is dead by a rule (Q-M1f.551); an `open`
/// conclusion is never stored at all
/// (`docs/ein/user/03-syntax-and-semantics/08-obligations.md`).
pub const STRUCTURAL: [&str; 5] = ["absent", "and", "false", "not", "or"];

/// The built-in computed predicates.
pub const PREDICATES: [&str; 2] = ["eq", "neq"];

/// **The kernel's own words**: no import qualifies one, and a declarator
/// binds none but `false` and `not`, as a relation, a rule or an alias
/// ([`binds_no_reserved`]).
///
/// The grammar never reads ten words as a name (`ein-ir`'s
/// `grammar::RESERVED_WORDS`, and the printer's copy,
/// [`crate::text::LEXER_KEYWORDS`]): `and` / `or` / `neq` / `rule` /
/// `query` / `solve` / `config` / `trace` / `macro` / `import`. So what
/// still reaches the loader as a declared name is the structural primitives
/// a position gives a meaning to (`absent`, `false`, `not`, `open`), the
/// predicate `eq`, `relation` — a name wherever a name goes, so `(relation ?R
/// ?A ?B)` stays a legal pattern — and the kernel's dunder. This is about
/// *binding* a name: a fact may still have a reserved head, such as a stored
/// `(not X)` octagon.
///
/// **That set is not this one.** The two overlap in three names (`and`,
/// `neq`, `or`) and neither contains the other. `open` is here alone
/// (Q-M1f.66, Q-M1f.571): the kernel reads it at the head of a written
/// fact, a premise or a conclusion, and it is a name elsewhere, which no
/// declarator binds.
///
/// **The kernel's dunder is here** (Q-M1f.7): [`CLOSED`] is written only
/// with its fixed meaning, so a program may not bind it, as a relation it
/// declares, a rule's or a macro's name, or an import's `:as`; it
/// may use it wherever a name is used, as a relation or as a value, which is
/// how `(imply1 functional __closed__)` hands the trigger to a rule. The engine's
/// three writeback names ([`FORCED_POSITIVE`], [`MONOTONIC_UNCONDITIONAL`],
/// [`LOOKAHEAD_DIES`]) are not here: the grammar refuses them wherever a
/// program writes them.
pub const RESERVED: [&str; 10] = [
    CLOSED, "absent", "and", "eq", "false", "neq", "not", "open", "or", "relation",
];

pub fn is_predicate(name: &str) -> bool {
    PREDICATES.contains(&name)
}

pub fn is_reserved(name: &str) -> bool {
    RESERVED.contains(&name)
}

/// The two of [`RESERVED`] that are the standard library's names
/// (Q-M1f.557): `false`, whose fact `std.core`'s rule reads, and `not`, whose
/// facts `std.contradiction`'s rules read. A declarator may bind either, as a
/// relation a program or a module declares, a rule's name, or an import's
/// `:as`; a macro may not, since its calls would then be every
/// `(false)` and `(not X)`, `std.core`'s among them. The grammar and the
/// loader still give each its shape.
pub const LIBRARY_NAMES: [&str; 2] = ["false", "not"];

/// Whether a declarator may bind `name`: it is none of [`RESERVED`] but
/// [`LIBRARY_NAMES`], so `eq`, `neq`, the connectives, `absent`, `open`,
/// `relation` and the kernel's dunder.
pub fn binds_no_reserved(name: &str) -> bool {
    !is_reserved(name) || LIBRARY_NAMES.contains(&name)
}

/// **Whether a name that heads a tuple is a relation of the program**
/// (Q-M1f.522): every name is, wherever the program writes it first in a
/// tuple, but the kernel's own words, [`RESERVED`], which the kernel reads as
/// the forms of its language (the connectives `and` and `or`, `absent`, the
/// computed `eq` and `neq`, the negation `not`, `false`, the verdict `open`,
/// the declaration `relation` and the closure marker [`CLOSED`]), and the
/// names the loader makes up for what a program writes no name for
/// ([`crate::text::INTERNAL_NAMES`]). `=` is a name like any other: the
/// grammar keeps it a name, and the kernel reads no form of it.
pub fn heads_a_relation(name: &str) -> bool {
    !is_reserved(name) && !crate::text::is_internal(name)
}

/// The engine's **own** names — the ones it writes into a KB rather than reads
/// out of a program.
///
/// Three are provenance rule-names for a derivation no rule performed
/// ([`crate::prov`]), one names the synthetic rule a `(query …)` goal
/// compiles to, and one is the `(__closed__ R)` marker. Not a style list:
/// together with `ein_ir::from_ir::intern_program_names` — which closes a
/// *rule's* argument constants at load — **these five are why nothing
/// interns a name during a search**, measured over the whole corpus by
/// `ein-infer/tests/interning.rs`.
///
/// Why that matters is [03-design/08-parallelism.md](../../../../docs/ein/ein.rs/03-design/08-parallelism.md)'s
/// [03-design/08-parallelism.md § 3](../../../../docs/ein/ein.rs/03-design/08-parallelism.md#3-what-must-be-sync-and-how):
/// [`Interner::text`](crate::Interner::text) hands out a `&str` borrowed from
/// the arena, which no lock can do, so a shareable interner is one that does
/// not **grow** while it is shared, and the search is where it would be
/// shared. Three of the five were growing it there
/// ([03-design/08-parallelism.md § 3](../../../../docs/ein/ein.rs/03-design/08-parallelism.md#3-what-must-be-sync-and-how));
/// the other two arrive during root saturation, which is single-threaded.
pub const ENGINE: [&str; 5] = [
    FORCED_POSITIVE,
    LOOKAHEAD_DIES,
    MONOTONIC_UNCONDITIONAL,
    QUERY_RULE,
    CLOSED,
];

/// The forced-positive cascade's promotion — a fact root writes because every
/// other value of its slot is excluded.
pub const FORCED_POSITIVE: &str = "<forced-positive>";
/// The walk's kill-cache `(not h)`, which a candidate that died under its
/// one-step lookahead left behind. No pass writes it since that cache went;
/// the name stays the engine's, refused wherever a program writes it (`ein.l`'s
/// `WRITEBACK`), and interned at its place in [`ENGINE`].
pub const LOOKAHEAD_DIES: &str = "<lookahead-dies-immediately>";
/// The singleton writeback's `(not h)` — a layer-1 death, generalised.
pub const MONOTONIC_UNCONDITIONAL: &str = "<monotonic-unconditional>";
/// The synthetic rule a `(query :goal …)` compiles to, so a goal binding has a
/// rule name to be reported under. One of [`crate::text::INTERNAL_NAMES`], so
/// no rule a program writes can be named it.
pub const QUERY_RULE: &str = "#query";
/// `(__closed__ R)` — R is a relation no rule can positively conclude.
pub const CLOSED: &str = "__closed__";

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
    /// The reserved verdict atom — `(false)`'s dual, and the one kernel name
    /// that is never a fact. The obligation pass
    /// ([`crate::Program::obligations`]) is what reads a plan's `:assert` for
    /// it.
    pub open: Symbol,
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
}

impl Terms {
    pub fn new() -> Self {
        let mut syms = Interner::new();
        let mut intern = |s: &str| syms.intern(s).expect("room for the kernel names");
        let kernel = Kernel {
            not: intern("not"),
            relation: intern("relation"),
            rule: intern("rule"),
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
            empty: intern(crate::text::EMPTY_HEAD),
            params: intern(crate::text::PARAMS_HEAD),
            forced_positive: intern(FORCED_POSITIVE),
            lookahead_dies: intern(LOOKAHEAD_DIES),
            monotonic_unconditional: intern(MONOTONIC_UNCONDITIONAL),
            query_rule: intern(QUERY_RULE),
            closed: intern(CLOSED),
            // Interned last on purpose: a kernel name's id is its position in
            // this list, and appending is the one edit that moves none of the
            // others. `id_order_invariance` says a permuted id space moves no
            // answer, so this is about keeping the *renderings* still too.
            open: intern("open"),
        };
        Terms {
            syms: Table::Own(syms),
            facts: Table::Own(FactStore::new()),
            provs: ProvArena::new(),
            kernel,
            refused: AtomicBool::new(false),
        }
    }

    // ── Sharing ────────────────────────────────────────────────────

    /// A worker's view of these tables: the same two interners, lent, and a
    /// **record region of its own**.
    ///
    /// This is the whole of what
    /// [03-design/08-parallelism.md § 2](../../../../docs/ein/ein.rs/03-design/08-parallelism.md#2-what-runs-in-parallel)
    /// hands a worker, and the shape is
    /// [03-design/08-parallelism.md § 3](../../../../docs/ein/ein.rs/03-design/08-parallelism.md#3-what-must-be-sync-and-how)'s
    /// four decisions in one type:
    ///
    /// - the **interner** is shared by `&`, because a search almost never
    ///   names a symbol the load did not, and a worker that would gets
    ///   [`Overflow::Shared`] too;
    /// - the **fact store** is shared by `&` too, because a search assigns few
    ///   ids against many borrow-returning reads, and most nodes assign none.
    ///   A worker that would have assigned one gets [`Overflow::Shared`] and
    ///   hands the node back;
    /// - the **provenance arena** is per-worker, because it is the one with a
    ///   real write rate, every node writing records, and because a fork's
    ///   records die with the fork;
    /// - and there is **no lock**, so there is no protocol to model.
    ///
    /// The view borrows nothing, so it is `Send` and can cross into a worker;
    /// what stops the committing thread interning while a view is alive is
    /// that the tables are [`Table::Shared`], not a borrow. Reads see the
    /// tables as they stood when they were lent, which is the same thing as
    /// saying they see them as they are: nothing may write them meanwhile.
    /// Lend the two intern tables out, so [`Terms::worker`] can hand a view
    /// to each worker. Idempotent, and the point at which this `Terms` stops
    /// being able to intern.
    pub fn share(&mut self) {
        self.syms.share();
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
        self.syms.reclaim() & self.facts.reclaim()
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
    /// ([03-design/08-parallelism.md § 1](../../../../docs/ein/ein.rs/03-design/08-parallelism.md#1-the-contract)).
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

    /// A name as a value — what the loader's `atomic_value` produces for an
    /// `Atom`: the symbol its text spells.
    ///
    /// A string is natural-language text and a variable is no value, so
    /// neither is a fact's argument: `(likes A "foo")` is refused, not read
    /// as `(likes A foo)`.
    pub fn value_text(&mut self, s: &str) -> Result<Value, Overflow> {
        Ok(Value::sym(self.intern_text(s)?))
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

    pub fn fact(&self, id: FactId) -> (Symbol, &[Value]) {
        self.facts.get(id)
    }

    pub fn probe_fact(&self, rel: Symbol, args: &[Value]) -> Option<FactId> {
        self.facts.probe(rel, args)
    }

    // ── Ordering ───────────────────────────────────────────────────

    /// Order two values by content: a name by its text ([`Interner::rank`]),
    /// a fact as [`Self::cmp_fact_semantic`] orders it, and a mixed pair by
    /// tag, `Sym < Fact`
    /// ([03-design/01-determinism-and-order.md § 2.2](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#22-content-order)).
    ///
    /// Two terms are compared with a stack of the argument lists still open on
    /// each side, not the call stack, so two facts nested past any depth
    /// compare ([`Self::walk_term`]); a value is equal to itself at once, a fact
    /// being one id for one content.
    pub fn cmp_semantic(&self, a: Value, b: Value) -> Ordering {
        // Each open pair of facts: both argument lists and the next position.
        let mut open: Vec<(&[Value], &[Value], usize)> = Vec::new();
        let mut next = Some((a, b));
        loop {
            if let Some((x, y)) = next.take()
                && x != y
            {
                match (x.tag(), y.tag()) {
                    (Tag::Sym, Tag::Sym) => {
                        let ord = self
                            .syms
                            .rank(x.as_sym().expect("tagged Sym"))
                            .cmp(&self.syms.rank(y.as_sym().expect("tagged Sym")));
                        if ord != Ordering::Equal {
                            return ord;
                        }
                    }
                    (Tag::Fact, Tag::Fact) => {
                        let (x_rel, x_args) = self.facts.get(x.as_fact().expect("tagged Fact"));
                        let (y_rel, y_args) = self.facts.get(y.as_fact().expect("tagged Fact"));
                        let ord = self.syms.rank(x_rel).cmp(&self.syms.rank(y_rel));
                        if ord != Ordering::Equal {
                            return ord;
                        }
                        open.push((x_args, y_args, 0));
                    }
                    (x, y) => return tag_order(x).cmp(&tag_order(y)),
                }
            }
            let Some((x_args, y_args, i)) = open.last_mut() else {
                return Ordering::Equal;
            };
            if *i < x_args.len() && *i < y_args.len() {
                next = Some((x_args[*i], y_args[*i]));
                *i += 1;
            } else {
                let ord = x_args.len().cmp(&y_args.len());
                if ord != Ordering::Equal {
                    return ord;
                }
                open.pop();
            }
        }
    }

    /// Order two facts by their `(relation_name, args)` identity tuples:
    /// relation name first, then arguments element-wise, then — a shorter
    /// tuple that is a prefix of a longer one sorting first — by arity.
    ///
    /// This is the comparator the lattice's first layer sorts the alive set by
    /// ([03-design/01-determinism-and-order.md § 4.2](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#42-the-hypothesis-set-and-the-search)).
    pub fn cmp_fact_semantic(&self, a: FactId, b: FactId) -> Ordering {
        self.cmp_semantic(Value::fact(a), Value::fact(b))
    }

    // ── Rendering ──────────────────────────────────────────────────

    /// **Every walk of a term that renders it or reads its names**: `each` is
    /// handed what the term is made of in the order it is written, a fact's
    /// relation before its arguments ([`TermStep`]).
    ///
    /// The walk keeps a stack of the facts it has opened, not the call stack,
    /// so a term nested at any depth is walked whole: a rule that nests its
    /// conclusion one deeper at each firing makes a fact as deep as the
    /// firings, and every surface that prints one prints it through here
    /// (`03-design/01-determinism-and-order.md` § 7).
    pub fn walk_term<'a>(&'a self, v: Value, mut each: impl FnMut(TermStep<'a>)) {
        let mut open: Vec<(Symbol, &'a [Value], usize)> = Vec::new();
        let mut next = Some(v);
        loop {
            if let Some(v) = next.take() {
                match v.tag() {
                    Tag::Sym => each(TermStep::Name(v.as_sym().expect("tagged Sym"))),
                    Tag::Fact => {
                        let (rel, args) = self.facts.get(v.as_fact().expect("tagged Fact"));
                        each(TermStep::Open(rel, args));
                        open.push((rel, args, 0));
                    }
                }
            }
            let Some((rel, args, i)) = open.last_mut() else {
                return;
            };
            if let Some(&a) = args.get(*i) {
                each(TermStep::Arg(*i));
                *i += 1;
                next = Some(a);
            } else {
                each(TermStep::Close(*rel, args));
                open.pop();
            }
        }
    }

    /// The value as `repr` prints it, for the `repr`-shaped output sites
    /// ([03-design/01-determinism-and-order.md § 7](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#7-the-value-renderer)):
    /// a name as [`repr::repr_str`], a fact as
    /// `Fact(relation_name=…, args=(…))`, the relation name and the
    /// arguments and nothing else about the fact — the text
    /// [`repr::repr`] gives the [`ReprValue`](crate::repr::ReprValue) of the
    /// value, written through [`Self::walk_term`].
    pub fn repr_text(&self, v: Value) -> String {
        let mut out = String::new();
        self.walk_term(v, |step| match step {
            TermStep::Name(s) => out.push_str(&repr::repr_str(self.sym(s))),
            TermStep::Open(rel, _) => {
                out.push_str("Fact(relation_name=");
                out.push_str(&repr::repr_str(self.sym(rel)));
                out.push_str(", args=(");
            }
            TermStep::Arg(i) => {
                if i > 0 {
                    out.push_str(", ");
                }
            }
            TermStep::Close(_, args) => {
                // The one-tuple's trailing comma, then the tuple and the fact.
                if args.len() == 1 {
                    out.push(',');
                }
                out.push_str("))");
            }
        });
        out
    }

    /// A whole fact as [`Self::repr_text`] prints it.
    pub fn repr_fact_text(&self, id: FactId) -> String {
        self.repr_text(Value::fact(id))
    }

    /// The `repr` of a fact's `(relation_name, args)` identity tuple,
    /// `('p', ('A',))`: unambiguous across the value shapes, and not the
    /// whole fact's [`Self::repr_fact_text`], a different string.
    pub fn repr_identity(&self, id: FactId) -> String {
        let (rel, args) = self.facts.get(id);
        let args: Vec<String> = args.iter().map(|&a| self.repr_text(a)).collect();
        repr::tuple(&[repr::repr_str(self.sym(rel)), repr::tuple(&args)])
    }

    /// A value as Ein text the loader reads back to the same value — **the
    /// one printer** ([`crate::text`]): a name bare where it lexes as itself
    /// and quoted elsewhere, and a `Fact` as [`Terms::fact_text`]. Every
    /// surface that prints a value prints it through here. There is no int:
    /// `1` and `01` are two names, and they print apart.
    pub fn display(&self, v: Value) -> String {
        let mut out = String::new();
        self.write_display(&mut out, v);
        out
    }

    /// [`Terms::display`], appended to `out`, through [`Self::walk_term`]: a
    /// fact `(rel arg …)`, `(rel)` with no arguments and `()` for the empty
    /// form, each argument printed the same way ([`text::form`]'s shape).
    pub fn write_display(&self, out: &mut String, v: Value) {
        self.walk_term(v, |step| match step {
            TermStep::Name(s) => out.push_str(&text::symbol(self.sym(s))),
            TermStep::Open(rel, args) => {
                out.push('(');
                let rel = self.sym(rel);
                if !(rel == text::EMPTY_HEAD && args.is_empty()) {
                    out.push_str(&text::head(rel));
                }
            }
            TermStep::Arg(_) => out.push(' '),
            TermStep::Close(..) => out.push(')'),
        });
    }

    /// A fact as Ein text: `(rel arg …)`, `(rel)` with no arguments, each
    /// argument as [`Terms::display`] prints it ([`text::form`]).
    pub fn fact_text(&self, id: FactId) -> String {
        self.display(Value::fact(id))
    }
}

/// What [`Terms::walk_term`] meets, in the order a term is written.
#[derive(Clone, Copy, Debug)]
pub enum TermStep<'a> {
    /// A name.
    Name(Symbol),
    /// A fact opens: its relation, and its arguments, which follow.
    Open(Symbol, &'a [Value]),
    /// The `i`-th argument (from 0) of the innermost open fact comes next.
    Arg(usize),
    /// The innermost open fact closes after its last argument: its relation
    /// and its arguments again.
    Close(Symbol, &'a [Value]),
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

/// `Sym < Fact`
/// ([03-design/01-determinism-and-order.md § 2.2](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#22-content-order)).
/// The raw tag order agrees, but an
/// observable order is not left to an encoding (`Sym` is tag 0 because it is
/// the overwhelmingly common shape and the cheapest thing to leave untagged in
/// a debugger).
fn tag_order(tag: Tag) -> u8 {
    match tag {
        Tag::Sym => 0,
        Tag::Fact => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The intern tables cross into the workers of
    /// [03-design/08-parallelism.md](../../../../docs/ein/ein.rs/03-design/08-parallelism.md),
    /// so the property is asserted here, which is what rules out an `Rc` or a
    /// `RefCell` creeping in.
    #[test]
    fn the_intern_tables_stay_shareable_across_threads() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Terms>();
        assert_send_sync::<crate::Interner>();
        assert_send_sync::<crate::FactStore>();
    }

    /// A name's value is its text's symbol, one text one value. A string and
    /// a variable are no value: the grammar refuses either in a fact.
    #[test]
    fn a_name_is_its_texts_one_value() {
        let mut t = Terms::new();
        let foo = t.value_text("foo").expect("room");
        assert_eq!(foo, t.value_text("foo").expect("room"));
        assert_ne!(foo, t.value_text("?foo").expect("room"));
        // A run of digits is a name like any other (Q-M1e2.24), and `07`
        // is another one.
        assert_ne!(
            t.value_text("7").expect("room"),
            t.value_text("07").expect("room")
        );
    }

    #[test]
    fn semantic_order_sorts_names_by_text() {
        let mut t = Terms::new();
        let zebra = t.value_text("zebra").expect("room");
        let apple = t.value_text("apple").expect("room");
        let two = t.value_text("2").expect("room");
        let ten = t.value_text("10").expect("room");
        assert_eq!(t.cmp_semantic(apple, zebra), Ordering::Less);
        // A run of digits is a name (Q-M1e2.24), so it sorts as text:
        // "10" < "2", as `(p 10)` lists before `(p 2)`.
        assert_eq!(t.cmp_semantic(ten, two), Ordering::Less);
        // Mixed: Sym < Fact (01-determinism-and-order.md § 2.2).
        let rel = t.intern_text("p").expect("room");
        let fact = t.value_fact(rel, &[apple]).expect("room");
        assert_eq!(t.cmp_semantic(zebra, fact), Ordering::Less);
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
    fn display_of_each_of_the_two_shapes() {
        let mut t = Terms::new();
        let s = t.value_text("House-1").expect("room");
        let i = t.value_text("007").expect("room");
        let rel = t.intern_text("co-located").expect("room");
        let nested = t.value_fact(rel, &[s, i]).expect("room");
        assert_eq!(t.display(s), "House-1");
        assert_eq!(t.display(i), "007");
        assert_eq!(t.display(nested), "(co-located House-1 007)");
        // A run of digits is a name (Q-M1e2.24), which prints bare and as
        // written; there is no int.
        let one = t.value_text("1").expect("room");
        assert_eq!(t.display(one), "1", "the name 1 prints as written");
        let zero_one = t.value_text("01").expect("room");
        assert_eq!(t.display(zero_one), "01", "and 01 is another name");
    }

    /// **The text writers write what the format's table states**: `repr` of
    /// the [`ReprValue`](crate::repr::ReprValue) a value is, built by hand,
    /// for every shape a fact's argument takes (a name, a fact of no, one and
    /// two arguments, a fact nested in one), the identity tuple, and the one
    /// printer's `()` for the empty form.
    #[test]
    fn the_text_writers_write_the_formats_shapes() {
        use crate::repr::{ReprValue, repr};
        let mut t = Terms::new();
        let rel = t.intern_text("co-located").expect("room");
        let quoted = t.intern_text("quote'd").expect("room");
        let zero = t.intern_text("arity-0").expect("room");
        let a = t.value_text("Norwegian").expect("room");
        let b = t.value_text("House 2").expect("room");
        let inner = t.value_fact(rel, &[a, b]).expect("room");
        let one = t.value_fact(quoted, &[inner]).expect("room");
        let none = t.value_fact(zero, &[]).expect("room");
        let s = |x: &str| ReprValue::Str(x.to_string());
        let inner_r = ReprValue::Fact {
            relation_name: "co-located".into(),
            args: vec![s("Norwegian"), s("House 2")],
        };
        let one_r = ReprValue::Fact {
            relation_name: "quote'd".into(),
            args: vec![inner_r.clone()],
        };
        let none_r = ReprValue::Fact {
            relation_name: "arity-0".into(),
            args: vec![],
        };
        for (v, want) in [
            (a, s("Norwegian")),
            (b, s("House 2")),
            (inner, inner_r.clone()),
        ] {
            assert_eq!(t.repr_text(v), repr(&want));
        }
        assert_eq!(t.repr_text(one), repr(&one_r));
        assert_eq!(t.repr_text(none), repr(&none_r));
        let id = one.as_fact().expect("a fact");
        assert_eq!(
            t.repr_identity(id),
            repr(&ReprValue::Tuple(vec![
                s("quote'd"),
                ReprValue::Tuple(vec![inner_r])
            ]))
        );
        assert_eq!(
            t.display(one),
            "(quote'd (co-located Norwegian \"House 2\"))"
        );
        let empty = t.intern_text(text::EMPTY_HEAD).expect("room");
        let empty = t.value_fact(empty, &[]).expect("room");
        assert_eq!(t.display(empty), "()");
        assert_eq!(t.display(none), "(arity-0)");
    }

    /// **No walk of a term recurses on its depth**: a fact nested 200 000
    /// deep, past what a call stack takes, is printed whole by the one
    /// printer and by `repr`, its names read, and ordered against one a
    /// level shallower and against itself, on a thread whose stack is
    /// small enough that a recursive walk of a few thousand levels would
    /// overflow it.
    #[test]
    fn a_term_nested_past_any_call_stack_is_walked_whole() {
        const DEPTH: usize = 200_000;
        std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(|| {
                let mut t = Terms::new();
                let s = t.intern_text("s").expect("room");
                let mut v = t.value_text("zero").expect("room");
                let mut shallower = v;
                for _ in 0..DEPTH {
                    shallower = v;
                    v = t.value_fact(s, &[v]).expect("room");
                }
                let text = t.display(v);
                assert_eq!(text.len(), DEPTH * 4 + 4);
                assert!(
                    text.starts_with("(s (s ") && text.trim_end_matches(')').ends_with("(s zero")
                );
                let repr = t.repr_text(v);
                assert!(repr.starts_with("Fact(relation_name='s', args=(Fact("));
                assert!(repr.trim_end_matches([',', ')']).ends_with("args=('zero'"));
                let mut names = 0usize;
                t.walk_term(v, |step| {
                    if let TermStep::Name(_) | TermStep::Open(..) = step {
                        names += 1;
                    }
                });
                assert_eq!(names, DEPTH + 1);
                assert_eq!(t.cmp_semantic(shallower, v), Ordering::Less);
                assert_eq!(t.cmp_semantic(v, shallower), Ordering::Greater);
                assert_eq!(t.cmp_semantic(v, v), Ordering::Equal);
            })
            .expect("a thread")
            .join()
            .expect("no overflow");
    }
}
