//! Work counters — what the engine *did*, not how long it took (T1a.6.1.3).
//!
//! Wall-clock is machine-dependent; work is not. A "26× faster" claim splits
//! into two very different statements — *did less* and *did the same, faster* —
//! and only the second is a port result. The first would mean the two engines
//! are solving different problems, which is a parity bug the harness should have
//! caught, so the split is worth having as a number even when it is boring.
//!
//! ein.py gets these for free: every one of these counters is the `ncalls`
//! column of a cProfile row (`match._bind_arg`, `match._bind_args`,
//! `saturator._binding_key`, …). ein.rs has no profiler that counts calls, and
//! the functions concerned are inlined into their callers anyway, so it counts
//! them explicitly — and only when asked:
//!
//! ```sh
//! cargo run --release --features counters -p ein-infer --example counter_cost
//! ```
//!
//! **Compiled out by default.** Without the `counters` feature [`bump`] has no
//! body, the closure passed to it is never called, and the field arithmetic
//! disappears; the shipped binary is byte-for-byte the one it was without this
//! module. That is not politeness, it is a measurement requirement: an
//! always-on counter in `unify_slot` would be an increment on the single
//! hottest path in the engine — 6.0 M of them on an exhaustive `zebra2`, 60 M
//! on `zebra` — and would perturb the very profile it exists to explain.
//!
//! Thread-local, because [P1a.7](../../../../plans/m1a_rust/p1a.7_parallelism/README.md)
//! will run several searches at once and a shared counter would either race or
//! serialise. [`snapshot`] reads the calling thread's; a parallel run sums them
//! at the join.

/// One field per countable unit of engine work. The comment on each names the
/// ein.py row it is comparable to, or says that there is none.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counters {
    // ── the matcher ────────────────────────────────────────────────
    /// One argument bound or compared — ein.py `match._bind_arg` (6.0 M calls
    /// on an exhaustive `zebra2`, 20 % of its self time).
    pub unify_slot: u64,
    /// One premise's whole argument list. **No ein.py counterpart**: its
    /// `_bind_args` is per-candidate only, because `_bind_arg` walks a nested
    /// pattern inline where this recurses back through `unify`. So `unify` is
    /// `candidates` plus the nested descents — 5.57 M against 4.61 M candidates
    /// on an exhaustive `zebra2` — and the comparable pair is `unify_slot`.
    pub unify: u64,
    /// Candidates *tried*: one per fact offered to a premise and unified
    /// against — ein.py `match._bind_args`. What an index narrows and a
    /// beta-memory would avoid re-deriving. ein.py additionally materialises
    /// each bucket as a tuple, so its `_candidates` sum is ~1.5× this even
    /// where the two try the same facts.
    pub candidates: u64,
    /// How a premise reached its candidates: through a participation-index
    /// bucket, or by walking the relation's whole extent. **No ein.py
    /// counterpart** — this is a question about *this* implementation's data
    /// layout, and it is the one that chose
    /// [S1a.6.2](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.2_memory_layout.md)'s
    /// remaining tasks: on an exhaustive `zebra` **99.1 %** of candidates come
    /// from an extent scan and only 0.9 % from a bucket, because the index does
    /// not key a nested-fact argument and `(not (R …))` is most of the corpus.
    /// A bucket-major layout would therefore have been built for 0.9 % of the
    /// work.
    pub scan_bucket: u64,
    pub scan_extent: u64,
    /// The same split over `candidates` themselves — counted directly rather
    /// than derived, which is what caught a `n_facts_of` histogram that had
    /// been taken over *declared* relations and missed the two big ones.
    /// `cand_extent / scan_extent` is the mean extent walked: **368** facts on
    /// an exhaustive `zebra`, 61 on `zebra2`.
    pub cand_bucket: u64,
    pub cand_extent: u64,
    /// The same four again, restricted to the **guard sub-plans** the NAF
    /// boundary runs — `join = total − guard`.
    ///
    /// [S1a.6.3](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.3_beta_memories.md)
    /// made the join 4.5× faster by keying the index one level inside a nested
    /// argument, and guard sub-plans go through the same `Matcher::walk`
    /// driver, so they should have got it too. Whole-run totals cannot say
    /// whether they did: they are dominated by whichever caller runs more.
    /// [T1a.6.12.3](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.12_boundary_and_snapshot.md#task-t1a6123--what-the-guard-queries-scan)
    /// splits them, because a guard that *could* key on a bound argument and
    /// does not is a bug with a 30 % price tag, and one that offers nothing to
    /// key on is a fact about the query.
    pub scan_bucket_guard: u64,
    pub scan_extent_guard: u64,
    pub cand_bucket_guard: u64,
    pub cand_extent_guard: u64,
    /// Premises answered by **one interned lookup** rather than by a scan —
    /// a step every one of whose slots was already bound, which asks whether
    /// one exact proposition is in the KB
    /// ([T1a.6.12.3](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.12_boundary_and_snapshot.md#task-t1a6123--what-the-guard-queries-scan)).
    /// `scan_ground` counts the questions and `cand_ground` the ones that
    /// found their fact; `scan_bucket + scan_extent + scan_ground` is every
    /// premise the matcher resolved.
    ///
    /// **No ein.py counterpart, and this is where the two engines stop doing
    /// the same work**: ein.py fetches the participation bucket and unifies
    /// every fact in it, so its `candidates` is larger here by construction —
    /// 71.8 % of `zebra -e`'s *guard* premises are ground, and each was a
    /// ~10-fact bucket walk.
    pub scan_ground: u64,
    pub scan_ground_guard: u64,
    pub cand_ground: u64,
    /// A nested premise — `(not (R ?a ?b))` — descending into the fact its
    /// argument names, split by whether the inner relation matched.
    ///
    /// The two puzzles are opposites here and it decided the shape of the
    /// nested step: **79 %** of an exhaustive `zebra2`'s candidates die on the
    /// relation comparison and never read the inner arguments, while an
    /// exhaustive `zebra`'s 25 M candidates almost all pass it and want them
    /// immediately.
    pub nested_rel_reject: u64,
    pub nested_rel_hit: u64,
    /// Plan steps entered — ein.py `match._run_steps` (1.0 M).
    pub walk: u64,
    /// Matcher entry points: one per `run` / `run_seeded` / `holds` call.
    pub plan_run: u64,

    // ── saturation ─────────────────────────────────────────────────
    /// Firing binding keys built — ein.py `saturator._binding_key` (445 k,
    /// 7 % of its self time).
    pub binding_key: u64,
    /// Rule plans compiled. ein.py caches per `Engine` and builds one engine
    /// per saturation, so it compiles the same plan many times over — **17 430**
    /// on an exhaustive `zebra2`, which is what ein.rs did too until
    /// [S1a.6.8](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.8_compile_cache_and_extents.md)
    /// built design/06 § Win A's per-run memo. **This is the one counter the
    /// two implementations are expected to disagree on**: 305 here against
    /// 17 430 in the oracle on that run, and 305 is the number of distinct
    /// `(rule, activator)` pairs rather than a target — design/06 guessed ~170,
    /// and the forks derive activators the root never had. The parity item next
    /// to it is the `compile` **event** count (17 250, identical), because that
    /// fires on an *engine* miss and not a memo miss.
    pub plan_compile: u64,
    /// Facts written and indexed — ein.py `store.add_and_index_fact`.
    pub fact_insert: u64,

    // ── the intern tables, as *shared* state ───────────────────────
    /// The four reads on [`crate::facts::FactStore`] that hand out a borrow —
    /// `rel`, `args`, `row`, `get`. **No ein.py counterpart**: this counts the
    /// thing that decides whether the store can be shared at all, because a
    /// `&[Value]` into the argument arena is what no lock can return
    /// (T1a.7.1.0).
    pub fact_read: u64,
    /// Lookups that create nothing — `FactStore::probe`, the hypothesis
    /// generator's "does this proposition already have a number".
    pub fact_probe: u64,
    /// Calls to `FactStore::intern`, hits included, against `fact_new` — the
    /// ids it actually assigned. The **ratio** is the measurement: on
    /// `branching/06 -e` it is 2 318 949 to 505, so the store is a read
    /// structure with a rare append rather than a write structure
    /// ([shared_state.md](../../../../plans/m1a_rust/p1a.7_parallelism/shared_state.md) §2).
    pub fact_intern: u64,
    pub fact_new: u64,
    /// Provenance records created — [`crate::prov::ProvArena::push`]. The
    /// arena is shared for the same reason the fact store is (see
    /// [`crate::prov`]) and has the same borrow-returning read, but
    /// [design/08 §6](../../../../plans/m1a_rust/design/08_parallelism.md#6-what-must-be-sync-and-how)
    /// does not list it, so nobody had asked how hard it is written.
    pub prov_push: u64,
    /// Records **read** — `ProvArena::get`, which returns a `&Prov`. The
    /// counterpart to [`Self::fact_read`], and what prices a branch on the
    /// read path if the arena ever splits into a global half and a fork-local
    /// one (T1a.7.1.7).
    pub prov_read: u64,

    // ── and the same two questions, *per entering* ─────────────────
    /// Enterings measured by the two counters below — a denominator that
    /// lives in the same snapshot as its numerators, so a partial run cannot
    /// produce a ratio against a total taken somewhere else. Equal to
    /// `MonotonicStats::enterings_total` on a run that finishes.
    pub entering: u64,
    /// …of which assigned at least one **fact id**, and at least one
    /// **provenance record**. This is the rate at which a worker forbidden to
    /// append would have to hand its entering back
    /// ([shared_state.md §5](../../../../plans/m1a_rust/p1a.7_parallelism/shared_state.md)),
    /// and it is a different question from the totals: 417 ids spread one per
    /// entering is a design and 417 in one entering is another.
    pub entering_fact_new: u64,
    pub entering_prov_new: u64,
    /// The largest **within-layer index** of an entering that assigned a fact
    /// id, plus one — so `0` means no entering did.
    ///
    /// The bail-out rate above is a *count*; this is where the count sits. If
    /// every interning entering is near the head of its layer, "run the first
    /// K sequentially, fan out the rest" removes them all, and the bound on
    /// wasted work stops being `count × jobs`.
    pub entering_fact_new_max_i: u64,
    /// Records pushed **from inside** an entering — the fork's own, as against
    /// root's. `prov_push - prov_push_in_entering` is what the committing
    /// thread wrote, and the split is the whole of T1a.7.1.7's question: a
    /// fork's records die with the fork, root's do not.
    pub prov_push_in_entering: u64,

    // ── negation at the boundary ───────────────────────────────────
    /// Guard sub-plan evaluations — one per guard, so ein.py's comparable site
    /// is `World.absent` and not the `first_failing` that loops over it. 33 113
    /// on an exhaustive `zebra2`, in **both** implementations.
    pub guard_query: u64,
    /// Boundary invalidation stamps taken, and the per-relation extent counts
    /// they add up. `watch_stamp_rel / watch_stamp` is the average number of
    /// relations a parked entry watches; the *product* is what
    /// [`crate::kb::Kb::n_facts_of`] is called for — 644 166 times on an
    /// exhaustive `zebra2`, in both implementations, and 9.5 % of the run here
    /// because a layered KB answers it in O(depth) where ein.py's flat index
    /// answers it in O(1).
    pub watch_stamp: u64,
    pub watch_stamp_rel: u64,
    /// Map probes performed by [`crate::kb::Kb::n_facts_of`] — the instrument
    /// for its O(1)-in-depth claim, and the only counter here that measures
    /// *this* implementation rather than the work both do. It equals
    /// `watch_stamp_rel` plus the engine's other extent questions; a fold over
    /// the layer stack would multiply it by `Kb::depth()`.
    pub extent_probe: u64,

    // ── the search layer ───────────────────────────────────────────
    /// KB forks — ein.py `store.fork` (101 on an exhaustive `zebra2`, against
    /// 104 here: the three extra are the root previews).
    pub fork: u64,
    /// Provenance nodes visited by a justification walk — no single ein.py row,
    /// because `walk_premises` is a generator and cProfile attributes its work
    /// to whoever drains it.
    pub prov_node: u64,
    /// Hypothesis-generation passes, and how many of them were the
    /// short-circuiting `complete()` question rather than `open_hypotheses()`
    /// — ein.py `hypgen.generate_hypotheses` calls, split by caller. The two
    /// have very different costs per call and the same cost per *candidate*,
    /// which is the split
    /// [S1a.6.4](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.4_hypgen_and_lattice.md)
    /// exists to measure.
    pub hypgen_call: u64,
    pub hypgen_complete: u64,
    /// One-step lookahead simulations — ein.py `lookahead.dies_immediately`.
    /// The costliest candidate filter, and the reason a pass costs 14x more
    /// with the lever on than off.
    pub lookahead_probe: u64,
    /// Guard sub-plan queries actually **run**, and the monotone half of them.
    ///
    /// Equal to `guard_query` since
    /// [T1a.6.12.2](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.12_boundary_and_snapshot.md#task-t1a6122--the-per-round-guard-memo-priced)
    /// removed the per-round memo that used to sit between them: its hit rate
    /// was 0–1.2 %, because the watch stamp had already filtered out the
    /// candidates that would have shared a question. The two counters are kept
    /// apart because the *difference* is the measurement — a memo reinstated
    /// here would have to earn the gap back.
    ///
    /// The `Saturator` counts both per saturation
    /// (`guard_evals` / `guard_evals_monotone`); these are the same two summed
    /// over every fork of a whole solve, which is what
    /// [Q-M1a.17](../../../../plans/m1a_rust/open_questions.md#q-m1a17--win-bs-80--assumed-monotone-guards-dominate)
    /// asks for and no per-saturation field can answer: design/06 § Win B's
    /// mechanism only reaches a *monotone* guard, its projection is ≥ 80 %, and
    /// at root scale the measured mix was 11–30 % the other way.
    pub guard_eval: u64,
    pub guard_eval_monotone: u64,

    // ── the frontend ───────────────────────────────────────────────
    /// `parse()` calls and the source bytes they were handed — one per
    /// *module text*, so the pair prices the import diamond: `zebra2` pulls
    /// `std.algebra` and `std.bijection`, and `std.bijection` pulls
    /// `std.algebra` again. **No ein.py counterpart** — its loader re-parses
    /// the same way, but nothing there counts it either.
    pub parse_call: u64,
    pub parse_bytes: u64,
    /// Terminal match *attempts* — the denominator a backtracking parser
    /// needs, since it asks several terminals about the same position and most
    /// of the asks fail. `lex_symbol` is the subset that ran `SYMBOL`'s
    /// eleven-word reserved-name walk, which is how S1a.6.5 priced replacing
    /// it (1 250 runs per load: not worth replacing).
    pub lex_match: u64,
    pub lex_symbol: u64,
    /// Interner calls and misses. A miss is the only one that allocates, which
    /// is what makes "the lexer allocates nothing per token" checkable rather
    /// than asserted.
    pub intern: u64,
    pub intern_miss: u64,

    /// Compile-cache keys built — [`crate::terms::Terms`]-level bookkeeping
    /// with **no ein.py counterpart**, like [`Self::extent_probe`]: ein.py's
    /// key is a tuple of the strings it already has, while a `PlanKey` has to
    /// intern them. It counts the *engine walk*, not the compiler:
    /// `plan_compile` is what a key that missed the memo cost.
    pub plan_key: u64,
}

#[cfg(feature = "counters")]
mod imp {
    use super::Counters;
    use std::cell::RefCell;

    thread_local! {
        static COUNTERS: RefCell<Counters> = const { RefCell::new(Counters::new_zero()) };
    }

    pub fn bump(f: impl FnOnce(&mut Counters)) {
        // `try_with`, not `with`: a thread tearing down its TLS must not panic
        // inside a counter, and a lost increment at exit is not a measurement
        // anyone reads.
        let _ = COUNTERS.try_with(|c| f(&mut c.borrow_mut()));
    }

    pub fn snapshot() -> Counters {
        COUNTERS.with(|c| *c.borrow())
    }

    pub fn reset() {
        COUNTERS.with(|c| *c.borrow_mut() = Counters::new_zero());
    }
}

#[cfg(not(feature = "counters"))]
mod imp {
    use super::Counters;

    /// The whole point of the module: with the feature off this is nothing at
    /// all. `f` is dropped without being called, so the closure body — and the
    /// field it would have touched — never reaches codegen.
    #[inline(always)]
    pub fn bump(f: impl FnOnce(&mut Counters)) {
        let _ = f;
    }

    #[inline(always)]
    pub fn snapshot() -> Counters {
        Counters::new_zero()
    }

    #[inline(always)]
    pub fn reset() {}
}

pub use imp::{bump, reset, snapshot};

impl Counters {
    /// `Default::default()` is not `const`, and the thread-local wants a const
    /// initialiser.
    pub const fn new_zero() -> Counters {
        Counters {
            unify_slot: 0,
            unify: 0,
            candidates: 0,
            scan_bucket: 0,
            scan_extent: 0,
            cand_bucket: 0,
            cand_extent: 0,
            scan_bucket_guard: 0,
            scan_extent_guard: 0,
            cand_bucket_guard: 0,
            cand_extent_guard: 0,
            scan_ground: 0,
            scan_ground_guard: 0,
            cand_ground: 0,
            nested_rel_reject: 0,
            nested_rel_hit: 0,
            walk: 0,
            plan_run: 0,
            binding_key: 0,
            plan_compile: 0,
            fact_insert: 0,
            fact_read: 0,
            fact_probe: 0,
            fact_intern: 0,
            fact_new: 0,
            prov_push: 0,
            prov_read: 0,
            entering: 0,
            entering_fact_new: 0,
            entering_prov_new: 0,
            entering_fact_new_max_i: 0,
            prov_push_in_entering: 0,
            guard_query: 0,
            watch_stamp: 0,
            watch_stamp_rel: 0,
            extent_probe: 0,
            fork: 0,
            prov_node: 0,
            hypgen_call: 0,
            hypgen_complete: 0,
            lookahead_probe: 0,
            plan_key: 0,
            guard_eval: 0,
            guard_eval_monotone: 0,
            parse_call: 0,
            parse_bytes: 0,
            lex_match: 0,
            lex_symbol: 0,
            intern: 0,
            intern_miss: 0,
        }
    }

    /// `(name, value)` in declaration order, so a printed table and a JSON
    /// artefact cannot drift from the struct or from each other.
    pub fn rows(&self) -> [(&'static str, u64); 50] {
        [
            ("unify_slot", self.unify_slot),
            ("unify", self.unify),
            ("candidates", self.candidates),
            ("scan_bucket", self.scan_bucket),
            ("scan_extent", self.scan_extent),
            ("cand_bucket", self.cand_bucket),
            ("cand_extent", self.cand_extent),
            ("scan_bucket_guard", self.scan_bucket_guard),
            ("scan_extent_guard", self.scan_extent_guard),
            ("cand_bucket_guard", self.cand_bucket_guard),
            ("cand_extent_guard", self.cand_extent_guard),
            ("scan_ground", self.scan_ground),
            ("scan_ground_guard", self.scan_ground_guard),
            ("cand_ground", self.cand_ground),
            ("nested_rel_reject", self.nested_rel_reject),
            ("nested_rel_hit", self.nested_rel_hit),
            ("walk", self.walk),
            ("plan_run", self.plan_run),
            ("binding_key", self.binding_key),
            ("plan_compile", self.plan_compile),
            ("fact_insert", self.fact_insert),
            ("fact_read", self.fact_read),
            ("fact_probe", self.fact_probe),
            ("fact_intern", self.fact_intern),
            ("fact_new", self.fact_new),
            ("prov_push", self.prov_push),
            ("prov_read", self.prov_read),
            ("entering", self.entering),
            ("entering_fact_new", self.entering_fact_new),
            ("entering_prov_new", self.entering_prov_new),
            ("entering_fact_new_max_i", self.entering_fact_new_max_i),
            ("prov_push_in_entering", self.prov_push_in_entering),
            ("guard_query", self.guard_query),
            ("watch_stamp", self.watch_stamp),
            ("watch_stamp_rel", self.watch_stamp_rel),
            ("extent_probe", self.extent_probe),
            ("fork", self.fork),
            ("prov_node", self.prov_node),
            ("hypgen_call", self.hypgen_call),
            ("hypgen_complete", self.hypgen_complete),
            ("lookahead_probe", self.lookahead_probe),
            ("plan_key", self.plan_key),
            ("guard_eval", self.guard_eval),
            ("guard_eval_monotone", self.guard_eval_monotone),
            ("parse_call", self.parse_call),
            ("parse_bytes", self.parse_bytes),
            ("lex_match", self.lex_match),
            ("lex_symbol", self.lex_symbol),
            ("intern", self.intern),
            ("intern_miss", self.intern_miss),
        ]
    }

    /// True when the build has counters compiled in and something ran.
    pub fn any(&self) -> bool {
        self.rows().iter().any(|(_, v)| *v > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bump_is_a_no_op_without_the_feature_and_counts_with_it() {
        reset();
        bump(|c| c.unify_slot += 3);
        let n = snapshot().unify_slot;
        if cfg!(feature = "counters") {
            assert_eq!(n, 3);
        } else {
            assert_eq!(n, 0, "the feature is off; nothing may be counted");
        }
    }

    #[test]
    fn rows_covers_every_field() {
        // A field added without a row would be silently unreportable.
        let mut c = Counters::new_zero();
        c.unify_slot = 1;
        c.unify = 1;
        c.candidates = 1;
        c.scan_bucket = 1;
        c.scan_extent = 1;
        c.cand_bucket = 1;
        c.cand_extent = 1;
        c.scan_bucket_guard = 1;
        c.scan_extent_guard = 1;
        c.cand_bucket_guard = 1;
        c.cand_extent_guard = 1;
        c.scan_ground = 1;
        c.scan_ground_guard = 1;
        c.cand_ground = 1;
        c.nested_rel_reject = 1;
        c.nested_rel_hit = 1;
        c.walk = 1;
        c.plan_run = 1;
        c.binding_key = 1;
        c.plan_compile = 1;
        c.fact_insert = 1;
        c.fact_read = 1;
        c.fact_probe = 1;
        c.fact_intern = 1;
        c.fact_new = 1;
        c.prov_push = 1;
        c.prov_read = 1;
        c.entering = 1;
        c.entering_fact_new = 1;
        c.entering_prov_new = 1;
        c.entering_fact_new_max_i = 1;
        c.prov_push_in_entering = 1;
        c.guard_query = 1;
        c.watch_stamp = 1;
        c.watch_stamp_rel = 1;
        c.fork = 1;
        c.prov_node = 1;
        c.hypgen_call = 1;
        c.hypgen_complete = 1;
        c.lookahead_probe = 1;
        c.plan_key = 1;
        c.guard_eval = 1;
        c.guard_eval_monotone = 1;
        c.parse_call = 1;
        c.parse_bytes = 1;
        c.lex_match = 1;
        c.lex_symbol = 1;
        c.intern = 1;
        c.intern_miss = 1;
        assert_eq!(
            c,
            Counters { ..c },
            "no field left out of the literal above"
        );
        assert_eq!(c.rows().iter().filter(|(_, v)| *v == 1).count(), 49);
    }
}
