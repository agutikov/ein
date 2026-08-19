//! What a **load** costs, phase by phase, and what it allocates —
//! [S1a.6.5](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.5_frontend.md)'s
//! confirmation and the allocation report its acceptance asks for.
//!
//! ```sh
//! cargo run --release -p ein-infer --example frontend_cost
//! cargo run --release --features counters -p ein-infer --example frontend_cost
//! ```
//!
//! `cargo bench`'s `parse/*` and `load/*` rows say what the whole path costs;
//! they cannot say *which part*. This splits one load into the pieces that are
//! separately callable —
//!
//! | phase | what it is |
//! |---|---|
//! | `read` | `fs::read_to_string` of the puzzle |
//! | `parse` | the puzzle's own forms |
//! | `resolve` | `(import …)` — **including parsing every module it pulls** |
//! | `Resolver::new` | `$EIN_STDLIB` → checkout walk → embedded (twice per load) |
//! | `macro guard` | `stdlib_macro_names()`, the S1.8a.f20 check |
//! | `rebuild_indexes` | the six groupings, from the fact list |
//! | `prov cycles` | `detect_provenance_cycles` |
//! | *ingest* | the residual: relations, rules, macro expansion, facts |
//!
//! — and reports each one's allocations next to its time, because the
//! stage's question is whether the load path's cost is *work* or *traffic*.
//!
//! Counts, not just times: with `--features counters` the `parse_call` /
//! `parse_bytes` pair prices the diamond. `zebra2` imports `std.algebra` and
//! `std.bijection`, and `std.bijection` imports `std.algebra` again — so the
//! bytes parsed per load exceed the bytes on disk, and by how much is the
//! number [T1a.6.5.3](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.5_frontend.md)
//! is about.
//!
//! The allocator here wraps **`System`**, as `alloc_cost` does: an allocation
//! *count* is a property of the program, and the example that counts them must
//! not also link the allocator whose whole job is to make them cheap. The
//! times are therefore comparable within this table and to each other, not to
//! `cargo bench`'s snmalloc rows — which is what the last column is for.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use ein_core::{Kb, Terms, detect_provenance_cycles};
use ein_ir::{Ast, Resolver, load_file, parse};

thread_local! {
    static ALLOCS: Cell<u64> = const { Cell::new(0) };
    static BYTES: Cell<u64> = const { Cell::new(0) };
}

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        note(layout.size() as u64);
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        note(layout.size() as u64);
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        note(new_size.saturating_sub(layout.size()) as u64);
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

fn note(bytes: u64) {
    let _ = ALLOCS.try_with(|c| c.set(c.get() + 1));
    let _ = BYTES.try_with(|c| c.set(c.get() + bytes));
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// One measured phase: best-of-N time, and the allocations of one run.
struct Phase {
    time: Duration,
    allocs: u64,
    bytes: u64,
}

/// Run `f` `rounds` times, keeping the best time and the *last* run's
/// allocation counts — allocations are deterministic here, so any run's are
/// the run's, and taking the last keeps the interner's cold start out of them
/// only when the caller wants that (each round builds its own `Ast`).
fn measure<T>(_name: &'static str, rounds: u32, mut f: impl FnMut() -> T) -> Phase {
    let mut best = Duration::MAX;
    let (mut allocs, mut bytes) = (0, 0);
    for _ in 0..rounds {
        ALLOCS.with(|c| c.set(0));
        BYTES.with(|c| c.set(0));
        let t = Instant::now();
        let out = f();
        let dt = t.elapsed();
        std::hint::black_box(&out);
        best = best.min(dt);
        allocs = ALLOCS.with(|c| c.get());
        bytes = BYTES.with(|c| c.get());
    }
    Phase {
        time: best,
        allocs,
        bytes,
    }
}

fn us(d: Duration) -> f64 {
    d.as_secs_f64() * 1e6
}

fn cell(root: &Path, rel: &str, rounds: u32) {
    let path = root.join(rel);
    let base = path.parent().map(Path::to_path_buf);
    let mut phases: Vec<Phase> = Vec::new();

    phases.push(measure("read", rounds, || {
        std::fs::read_to_string(&path).expect("reads")
    }));
    let text = std::fs::read_to_string(&path).expect("reads");

    phases.push(measure("parse", rounds, || {
        let mut ast = Ast::new();
        let forms = parse(&mut ast, &text, path.to_str()).expect("parses");
        forms.len()
    }));

    phases.push(measure("resolve imports", rounds, || {
        let mut ast = Ast::new();
        let forms = parse(&mut ast, &text, path.to_str()).expect("parses");
        let resolved = Resolver::new()
            .resolve_imports(&mut ast, &forms, base.as_deref())
            .expect("resolves");
        resolved.len()
    }));
    // The resolve row above includes the parse it starts from; report the
    // difference, which is what the imports themselves cost.
    let resolve_only = phases[2].time.saturating_sub(phases[1].time);
    let resolve_allocs = phases[2].allocs.saturating_sub(phases[1].allocs);

    // Macro expansion, on the forms a resolve produces — `collect_macros`
    // plus the rule-clause rewrite the loader runs inside `ingest_rules`
    // (T1a.6.5.4).
    phases.push(measure("macro expand", rounds, || {
        let mut ast = Ast::new();
        let forms = parse(&mut ast, &text, path.to_str()).expect("parses");
        let resolved = Resolver::new()
            .resolve_imports(&mut ast, &forms, base.as_deref())
            .expect("resolves");
        let macros = ein_ir::macros::collect_macros(&ast, &resolved);
        ein_ir::macros::expand_rule_clauses(&mut ast, &resolved, &macros)
            .expect("expands")
            .len()
    }));
    let expand_only = phases[3].time.saturating_sub(phases[2].time);
    let expand_allocs = phases[3].allocs.saturating_sub(phases[2].allocs);

    phases.push(measure("Resolver::new", rounds, Resolver::new));
    phases.push(measure("macro guard", rounds, || {
        Resolver::new().stdlib_macro_names()
    }));
    let (rnew, guard) = (&phases[4], &phases[5]);

    let load = measure("load (whole)", rounds, || {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let kb = load_file(&mut ast, &mut terms, &path).expect("loads");
        kb.n_facts()
    });

    // The two tail passes, timed on an already-loaded KB.
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb: Kb = load_file(&mut ast, &mut terms, &path).expect("loads");
    let index = measure("rebuild_indexes", rounds, || {
        kb.rebuild_indexes(&terms);
    });
    let cycles = measure("prov cycles", rounds, || {
        detect_provenance_cycles(&kb, &terms)
    });

    let accounted = phases[0].time + phases[3].time + rnew.time * 2 + guard.time
        + index.time
        + cycles.time;
    let ingest = load.time.saturating_sub(accounted);
    let accounted_allocs = phases[0].allocs + phases[3].allocs + rnew.allocs * 2
        + guard.allocs
        + index.allocs
        + cycles.allocs;
    let ingest_allocs = load.allocs.saturating_sub(accounted_allocs);

    println!("\n{rel}   ({} bytes on disk)", text.len());
    println!(
        "  {:<18} {:>10} {:>10} {:>12} {:>7}",
        "phase", "best µs", "allocs", "bytes", "% load"
    );
    let share = |t: Duration| 100.0 * us(t) / us(load.time);
    let row = |name: &str, t: Duration, a: u64, b: u64| {
        println!(
            "  {:<18} {:>10.1} {:>10} {:>12} {:>6.1}%",
            name,
            us(t),
            a,
            b,
            share(t)
        );
    };
    row("read", phases[0].time, phases[0].allocs, phases[0].bytes);
    row("parse", phases[1].time, phases[1].allocs, phases[1].bytes);
    row(
        "  imports",
        resolve_only,
        resolve_allocs,
        phases[2].bytes.saturating_sub(phases[1].bytes),
    );
    row("  macro expand", expand_only, expand_allocs, 0);
    row("Resolver::new x2", rnew.time * 2, rnew.allocs * 2, rnew.bytes * 2);
    row("macro guard", guard.time, guard.allocs, guard.bytes);
    row("ingest (residual)", ingest, ingest_allocs, 0);
    row("rebuild_indexes", index.time, index.allocs, index.bytes);
    row("prov cycles", cycles.time, cycles.allocs, cycles.bytes);
    println!(
        "  {:<18} {:>10.1} {:>10} {:>12} {:>6}",
        "load (whole)",
        us(load.time),
        load.allocs,
        load.bytes,
        "100%"
    );

    // What the load produced, so a per-item cost can be read off.
    let resolved_forms = {
        let mut ast = Ast::new();
        let forms = parse(&mut ast, &text, path.to_str()).expect("parses");
        Resolver::new()
            .resolve_imports(&mut ast, &forms, base.as_deref())
            .expect("resolves")
            .len()
    };
    let mut ast = Ast::new();
    let own_forms = parse(&mut ast, &text, path.to_str()).expect("parses").len();
    let (nodes, args, syms) = ast.arena_sizes();
    println!(
        "  own parse arenas: {nodes} nodes, {args} args, {syms} symbols          ({:.1} bytes of source per node)",
        text.len() as f64 / nodes as f64
    );
    println!(
        "  forms {own_forms} own → {resolved_forms} resolved · facts {} · relations {} · rules {}",
        kb.n_facts(),
        kb.program().relations.len(),
        kb.program().rules.len() + kb.program().hrules.len(),
    );
    // Counters, for **one** load — reset here rather than around the timing
    // loop above, which runs the path `rounds` times.
    ein_core::counters::reset();
    {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let kb = load_file(&mut ast, &mut terms, &path).expect("loads");
        std::hint::black_box(kb.n_facts());
    }
    let c = ein_core::counters::snapshot();
    if c.parse_call > 0 {
        println!(
            "  parse_call {} · parse_bytes {} ({:.2}x the {} on disk) · \
             lex_match {} · lex_symbol {} · intern {} ({} miss)",
            c.parse_call,
            c.parse_bytes,
            c.parse_bytes as f64 / text.len() as f64,
            text.len(),
            c.lex_match,
            c.lex_symbol,
            c.intern,
            c.intern_miss,
        );
    }
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root");
    let rounds: u32 = std::env::args()
        .skip_while(|a| a != "--rounds")
        .nth(1)
        .and_then(|v| v.parse().ok())
        .unwrap_or(50);
    let files: Vec<String> = std::env::args()
        .skip(1)
        .filter(|a| a.ends_with(".ein"))
        .collect();
    let files: Vec<&str> = if files.is_empty() {
        vec![
            "examples/zebra2.ein",
            "examples/zebra.ein",
            "examples/features/05_stdlib_domain_elim.ein",
        ]
    } else {
        files.iter().map(String::as_str).collect()
    };
    println!("best of {rounds}, System allocator (not the binary's snmalloc)");
    for rel in files {
        cell(&root, rel, rounds);
    }
}
