//! What the clock costs a core's search on the corpus: every corpus program
//! that is not `slow`, described through the library with every visited dead
//! node's core taken (`Cores::ALL`), under no deadline (`none`) or one a day
//! away (`far`); one line a program, its path, its wall in microseconds, the
//! dead members of N its description holds, and whether it answered; the
//! total on stderr.
//!
//! It is an example of `ein-infer` and runs from there: copied in, built at
//! the commit to measure, and removed again (corpus.sh does the three).
//!
//!     cp core_clock_probe.rs ein.rs/crates/ein-infer/examples/
//!     cargo build --release -p ein-infer --example core_clock_probe
//!     utils/bench_env.sh --cores PT:1 ein.rs/target/release/examples/core_clock_probe far
use std::time::{Duration, Instant};

use ein_core::Terms;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_ir::Ast;

fn main() {
    let far = std::env::args().nth(1).as_deref() == Some("far");
    let slow: std::collections::BTreeSet<std::path::PathBuf> = ein_corpus::corpus()
        .entry
        .iter()
        .filter(|e| e.slow)
        .map(|e| ein_corpus::repo_root().join(&e.path))
        .collect();
    let mut total = Duration::ZERO;
    for path in ein_corpus::corpus_files() {
        if slow.contains(&path) {
            continue;
        }
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Ok(mut kb) = ein_ir::load_file(&mut ast, &mut terms, &path) else {
            continue;
        };
        let mut opts = explore::Options::new(Mode {
            which: Which::All,
            ask: Ask::Describe,
        });
        opts.cores = ein_infer::region::Cores::ALL;
        if far {
            opts.deadline = Some(Instant::now() + Duration::from_secs(86_400));
        }
        let t = Instant::now();
        let r = explore::run(&mut kb, &mut terms, &ast, &opts);
        let w = t.elapsed();
        total += w;
        let dead = r.as_ref().map(|a| a.described.as_ref().map_or(0, |d| d.dead.len())).unwrap_or(0);
        println!(
            "{}\t{}\t{}\t{}",
            path.strip_prefix(ein_corpus::repo_root()).unwrap().display(),
            w.as_micros(),
            dead,
            r.is_ok()
        );
    }
    eprintln!("total {:?}", total);
}
