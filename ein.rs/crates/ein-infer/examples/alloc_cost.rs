//! The memory baseline (T1a.6.1.4): allocations, live bytes, peak RSS, and the
//! **per-fork delta distribution**.
//!
//! ```sh
//! cargo run --release -p ein-infer --example alloc_cost
//! cargo run --release -p ein-infer --example alloc_cost -- --json
//! ```
//!
//! Three questions, and only the third is new:
//!
//! 1. *What does a solve allocate?* The profile puts ~21 % of an exhaustive
//!    `zebra2`'s self time in `malloc` / `free` / unsymbolised libc, so the
//!    count and the churn are what
//!    [S1a.6.2](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.2_memory_layout.md)
//!    starts from.
//! 2. *What does it hold?* Peak live bytes and peak RSS — the second including
//!    the allocator's own fragmentation, which the first cannot see.
//! 3. *What does one fork cost?* [P1a.7](../../../../plans/m1a_rust/p1a.7_parallelism/README.md)
//!    sizes `--jobs` by how many searches fit in RAM at once, and that is the
//!    *distribution* of fork deltas, not their mean: a mean hides the one
//!    entering whose saturation derives four times what the others do.
//!    Measured through the `Dumper` hook the state dumps already use, which
//!    hands every entering its saturated fork
//!    ([`EnteringInfo::kb`](../src/solve.rs)) — so this measures the real forks
//!    of a real search rather than a fork made to be measured.
//!
//! The allocator counts on the **calling thread** only, so it stays correct
//! when P1a.7 makes this a multi-threaded question; today it is one thread.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use ein_core::{FactId, Kb, Terms};
use ein_infer::solve::{Dumper, EnteringInfo, SolveOptions, solve};
use ein_infer::{Events, MonotonicStats};
use ein_ir::{Ast, load_file};

thread_local! {
    static ALLOCS: Cell<u64> = const { Cell::new(0) };
    static BYTES: Cell<u64> = const { Cell::new(0) };
    static LIVE: Cell<i64> = const { Cell::new(0) };
    static PEAK: Cell<i64> = const { Cell::new(0) };
}

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        note(layout.size() as i64);
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        note(layout.size() as i64);
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // A grow is one allocation of the *difference* as far as live bytes are
        // concerned, and one allocation event as far as churn is concerned —
        // which is why `Vec` doubling shows up here at all.
        note(new_size as i64 - layout.size() as i64);
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let _ = LIVE.try_with(|c| {
            c.set(c.get() - layout.size() as i64);
        });
        unsafe { System.dealloc(ptr, layout) }
    }
}

fn note(delta: i64) {
    // `try_with` because a thread tearing down its TLS still allocates.
    let _ = ALLOCS.try_with(|c| c.set(c.get() + 1));
    let _ = BYTES.try_with(|c| c.set(c.get() + delta.max(0) as u64));
    let _ = LIVE.try_with(|c| {
        let live = c.get() + delta;
        c.set(live);
        let _ = PEAK.try_with(|p| {
            if live > p.get() {
                p.set(live)
            }
        });
    });
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

fn reset_alloc() {
    ALLOCS.with(|c| c.set(0));
    BYTES.with(|c| c.set(0));
    // `live` is not reset: it is a running total of what is *held*, and the KB
    // loaded a moment ago is held. `peak` is, so it is the peak of this solve.
    PEAK.with(|p| p.set(LIVE.with(|c| c.get())));
}

/// Per-entering fork deltas, collected through the dumper hook.
#[derive(Default)]
struct ForkSizes {
    facts: Vec<usize>,
    bytes: Vec<usize>,
    depth: Vec<usize>,
}

impl Dumper for ForkSizes {
    fn entering(
        &mut self,
        _layer: u32,
        _commitment: &[FactId],
        _terms: &Terms,
        _outcome: &str,
        info: &EnteringInfo<'_>,
    ) {
        if let Some(kb) = info.kb {
            self.facts.push(kb.top().n_facts());
            self.bytes.push(kb.top().footprint());
            self.depth.push(kb.depth());
        }
    }
}

fn quantiles(v: &mut [usize]) -> (usize, usize, usize, usize, usize) {
    v.sort_unstable();
    if v.is_empty() {
        return (0, 0, 0, 0, 0);
    }
    let at = |q: f64| v[((v.len() - 1) as f64 * q).round() as usize];
    let mean = v.iter().sum::<usize>() / v.len();
    (v[0], at(0.5), at(0.9), v[v.len() - 1], mean)
}

fn rss_hwm_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with("VmHWM:"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

struct Row {
    cell: String,
    allocs: u64,
    bytes: u64,
    peak_live: i64,
    forks: usize,
    fork_facts: (usize, usize, usize, usize, usize),
    fork_bytes: (usize, usize, usize, usize, usize),
    max_depth: usize,
    stats: MonotonicStats,
}

fn main() {
    let json = std::env::args().any(|a| a == "--json");
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root");

    let mut rows: Vec<Row> = Vec::new();
    for rel in ["examples/zebra2.ein", "examples/zebra.ein"] {
        for (label, stop_after) in [("fast", Some(1)), ("exhaustive", None)] {
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let mut kb: Kb = load_file(&mut ast, &mut terms, &root.join(rel)).expect("loads");
            let mut events = Events::off();
            let opts = SolveOptions {
                stop_after,
                ..SolveOptions::default()
            };
            let mut sizes = ForkSizes::default();
            reset_alloc();
            let solved =
                solve(&mut kb, &mut terms, &ast, &mut events, &mut sizes, &opts).expect("solves");
            let (allocs, bytes, peak) = (
                ALLOCS.with(|c| c.get()),
                BYTES.with(|c| c.get()),
                PEAK.with(|c| c.get()),
            );
            let name = rel.rsplit('/').next().unwrap_or(rel).replace(".ein", "");
            rows.push(Row {
                cell: format!("{name} {label}"),
                allocs,
                bytes,
                peak_live: peak,
                forks: sizes.facts.len(),
                fork_facts: quantiles(&mut sizes.facts.clone()),
                fork_bytes: quantiles(&mut sizes.bytes.clone()),
                max_depth: sizes.depth.iter().copied().max().unwrap_or(0),
                stats: solved.stats,
            });
        }
    }

    if json {
        println!("[");
        for (i, r) in rows.iter().enumerate() {
            println!(
                "  {{\"cell\": \"{}\", \"allocs\": {}, \"bytes\": {}, \
                 \"peak_live_bytes\": {}, \"forks_seen\": {}, \
                 \"fork_facts\": [{}, {}, {}, {}, {}], \
                 \"fork_bytes\": [{}, {}, {}, {}, {}], \"max_depth\": {}, \
                 \"enterings\": {}, \"rss_hwm_kb\": {}}}{}",
                r.cell,
                r.allocs,
                r.bytes,
                r.peak_live,
                r.forks,
                r.fork_facts.0,
                r.fork_facts.1,
                r.fork_facts.2,
                r.fork_facts.3,
                r.fork_facts.4,
                r.fork_bytes.0,
                r.fork_bytes.1,
                r.fork_bytes.2,
                r.fork_bytes.3,
                r.fork_bytes.4,
                r.max_depth,
                r.stats.base.enterings_total,
                rss_hwm_kb(),
                if i + 1 == rows.len() { "" } else { "," }
            );
        }
        println!("]");
        return;
    }

    println!(
        "{:<19}{:>11}{:>11}{:>11}{:>8}{:>9}",
        "cell", "allocs", "alloc MB", "peak live", "forks", "depth"
    );
    println!("{}", "─".repeat(69));
    for r in &rows {
        println!(
            "{:<19}{:>11}{:>10.1}M{:>10.2}M{:>8}{:>9}",
            r.cell,
            r.allocs,
            r.bytes as f64 / 1e6,
            r.peak_live as f64 / 1e6,
            r.forks,
            r.max_depth,
        );
    }
    println!("\nper-fork delta (the saturated fork's own layer)");
    println!(
        "{:<19}{:>9}{:>9}{:>9}{:>9}{:>9}",
        "cell", "min", "median", "p90", "max", "mean"
    );
    println!("{}", "─".repeat(64));
    for r in &rows {
        let f = r.fork_facts;
        println!(
            "{:<19}{:>9}{:>9}{:>9}{:>9}{:>9}  facts",
            r.cell, f.0, f.1, f.2, f.3, f.4
        );
        let b = r.fork_bytes;
        println!(
            "{:<19}{:>8.1}K{:>8.1}K{:>8.1}K{:>8.1}K{:>8.1}K  bytes",
            "",
            b.0 as f64 / 1024.0,
            b.1 as f64 / 1024.0,
            b.2 as f64 / 1024.0,
            b.3 as f64 / 1024.0,
            b.4 as f64 / 1024.0,
        );
    }
    println!(
        "\npeak RSS (VmHWM, whole process, all four cells): {} KB",
        rss_hwm_kb()
    );
}
