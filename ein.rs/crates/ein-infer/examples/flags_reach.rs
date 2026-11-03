//! Every mode of `ein solve` on a monotone program, beside describe — M1e2
//! [S1e2.1c.4](../../../../plans/m1e2_correctness/p1e2.1c_the_region_search/s1e2.1c.4_the_flags_at_scale.md)'s
//! measurement, for `solve_interface.md` § 3.4 (T1e2.1c.4.3) and for what of
//! the walk's library the routes reuse (T1e2.1c.4.4).
//!
//! For each file and each mode it runs [`explore::run`], the routes
//! `ein solve` takes, and prints one JSON line: the route, the licences,
//! saturations, facts, the answer and the wall. With `--walk` it runs the
//! walk's library instead, exhausted (`-e`), and prints its enterings and its
//! models; run it under `EIN_TRAVERSAL=tree` for the walk's obligation tree,
//! which reads that variable as it always has. Nothing here is read by a
//! route.
//!
//! ```sh
//! cargo build --release -p ein-infer --example flags_reach
//! ein.rs/target/release/examples/flags_reach [--budget F] [--jobs J] FILE…
//! EIN_TRAVERSAL=tree ein.rs/target/release/examples/flags_reach --walk FILE…
//! ein.rs/target/release/examples/flags_reach --table FILE…   # L5's table, timed
//! ```

use std::path::Path;
use std::time::Instant;

use ein_core::Terms;
use ein_infer::Events;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_ir::{Ast, load, parse};
use serde_json::json;

fn loaded(path: &str) -> (ein_core::Kb, Terms, Ast) {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, Some(path)).unwrap_or_else(|e| panic!("{path}: {e:?}"));
    let kb = load(&mut ast, &mut terms, &forms, Path::new(path).parent())
        .unwrap_or_else(|e| panic!("{path}: {e:?}"));
    (kb, terms, ast)
}

const MODES: [(&str, Which, Ask); 11] = [
    ("describe", Which::All, Ask::Describe),
    ("-n 1", Which::All, Ask::Find(1)),
    ("-n 3", Which::All, Ask::Find(3)),
    ("--single", Which::All, Ask::Single),
    ("--minimal", Which::Minimal, Ask::Describe),
    ("--minimal -n 1", Which::Minimal, Ask::Find(1)),
    ("--minimal --single", Which::Minimal, Ask::Single),
    ("--maximal", Which::Maximal, Ask::Describe),
    ("--maximal -n 1", Which::Maximal, Ask::Find(1)),
    ("--maximal --single", Which::Maximal, Ask::Single),
    ("-n 1000", Which::All, Ask::Find(1000)),
];

fn main() {
    let mut budget = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    })
    .budget;
    let mut jobs = 1usize;
    let mut walk = false;
    let mut table = false;
    let mut files: Vec<String> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--budget" => {
                budget = args
                    .next()
                    .and_then(|s| s.parse().ok())
                    .expect("--budget F")
            }
            "--jobs" => jobs = args.next().and_then(|s| s.parse().ok()).expect("--jobs J"),
            "--walk" => walk = true,
            "--table" => table = true,
            f => files.push(f.to_string()),
        }
    }
    for file in &files {
        if table {
            // L5's table: root prepared, every instance root owes read.
            let (mut kb, mut terms, ast) = loaded(file);
            let t0 = Instant::now();
            let t = ein_infer::oblgen::alternatives_of(&mut kb, &mut terms, &ast);
            let ms = t0.elapsed().as_secs_f64() * 1000.0;
            let line = match t {
                Ok(Some(t)) => json!({
                    "file": file,
                    "instances": t.instances.len(),
                    "licensed": t.licensed(),
                    "prepare_and_table_ms": ms,
                }),
                Ok(None) => json!({"file": file, "instances": 0, "prepare_and_table_ms": ms}),
                Err(e) => json!({"file": file, "error": e.to_string()}),
            };
            println!("{line}");
            continue;
        }
        if walk {
            let (mut kb, mut terms, ast) = loaded(file);
            let opts = ein_infer::solve::SolveOptions {
                stop_after: None,
                max_set_size: u32::MAX,
                ..ein_infer::solve::SolveOptions::default()
            };
            let t0 = Instant::now();
            let r = ein_infer::solve::solve(
                &mut kb,
                &mut terms,
                &ast,
                &mut Events::off(),
                &mut ein_infer::solve::NoDumper,
                &opts,
            );
            let line = match r {
                Ok(s) => json!({
                    "file": file,
                    "walk": std::env::var("EIN_TRAVERSAL").unwrap_or_default(),
                    "answer": s.answer.as_str(),
                    "models": s.stats.solution_nodes,
                    "exhausted": s.stats.exhausted,
                    "enterings": s.stats.base.enterings_total,
                    "alive": s.stats.base.enterings_alive,
                    "saturations": s.stats.base.saturate_count,
                    "wall_ms": t0.elapsed().as_secs_f64() * 1000.0,
                }),
                Err(e) => json!({"file": file, "walk": "error", "error": e.to_string()}),
            };
            println!("{line}");
            continue;
        }
        for (name, which, ask) in MODES {
            let (mut kb, mut terms, ast) = loaded(file);
            let mut opts = explore::Options::new(Mode { which, ask });
            opts.budget = budget;
            opts.jobs = jobs;
            let t0 = Instant::now();
            let line = match explore::run(&mut kb, &mut terms, &ast, &opts) {
                Ok(a) => json!({
                    "file": file,
                    "mode": name,
                    "monotone": a.monotonicity.is_monotone(),
                    "route": format!("{:?}", a.route),
                    "licences": a.licences.iter().map(|l| format!("{l:?}")).collect::<Vec<_>>(),
                    "saturations": a.saturations,
                    "facts": a.facts,
                    "exhausted": a.exhausted,
                    "stopped": a.stopped.is_some(),
                    "verdict": a.verdict.map(|v| v.word()),
                    "states": a.states.len(),
                    "k_s": a.counts.map(|c| c.k_s),
                    "wall_ms": t0.elapsed().as_secs_f64() * 1000.0,
                }),
                Err(e) => json!({"file": file, "mode": name, "error": e.to_string()}),
            };
            println!("{line}");
        }
    }
}
