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
    /// Plan steps entered — ein.py `match._run_steps` (1.0 M).
    pub walk: u64,
    /// Matcher entry points: one per `run` / `run_seeded` / `holds` call.
    pub plan_run: u64,

    // ── saturation ─────────────────────────────────────────────────
    /// Firing binding keys built — ein.py `saturator._binding_key` (445 k,
    /// 7 % of its self time).
    pub binding_key: u64,
    /// Rule plans compiled. Both implementations cache per `Engine` and build
    /// one engine per saturation, so both compile the same plan many times over
    /// — **17 430** on an exhaustive `zebra2`, identically. That agreement is
    /// the parity result; the cost is that `design/06` § Win A's process-wide
    /// memo was never built
    /// ([S1a.6.8](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.8_compile_cache_and_extents.md)
    /// lands it, and the number should then drop to ~170 here and stay at
    /// 17 430 in the oracle).
    pub plan_compile: u64,
    /// Facts written and indexed — ein.py `store.add_and_index_fact`.
    pub fact_insert: u64,

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

    // ── the search layer ───────────────────────────────────────────
    /// KB forks — ein.py `store.fork` (101 on an exhaustive `zebra2`, against
    /// 104 here: the three extra are the root previews).
    pub fork: u64,
    /// Provenance nodes visited by a justification walk — no single ein.py row,
    /// because `walk_premises` is a generator and cProfile attributes its work
    /// to whoever drains it.
    pub prov_node: u64,
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
            walk: 0,
            plan_run: 0,
            binding_key: 0,
            plan_compile: 0,
            fact_insert: 0,
            guard_query: 0,
            watch_stamp: 0,
            watch_stamp_rel: 0,
            fork: 0,
            prov_node: 0,
        }
    }

    /// `(name, value)` in declaration order, so a printed table and a JSON
    /// artefact cannot drift from the struct or from each other.
    pub fn rows(&self) -> [(&'static str, u64); 13] {
        [
            ("unify_slot", self.unify_slot),
            ("unify", self.unify),
            ("candidates", self.candidates),
            ("walk", self.walk),
            ("plan_run", self.plan_run),
            ("binding_key", self.binding_key),
            ("plan_compile", self.plan_compile),
            ("fact_insert", self.fact_insert),
            ("guard_query", self.guard_query),
            ("watch_stamp", self.watch_stamp),
            ("watch_stamp_rel", self.watch_stamp_rel),
            ("fork", self.fork),
            ("prov_node", self.prov_node),
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
        c.walk = 1;
        c.plan_run = 1;
        c.binding_key = 1;
        c.plan_compile = 1;
        c.fact_insert = 1;
        c.guard_query = 1;
        c.watch_stamp = 1;
        c.watch_stamp_rel = 1;
        c.fork = 1;
        c.prov_node = 1;
        assert_eq!(
            c,
            Counters { ..c },
            "no field left out of the literal above"
        );
        assert_eq!(c.rows().iter().filter(|(_, v)| *v == 1).count(), 13);
    }
}
