//! The parser, from the command line (M1f S1f.5.5, T1f.5.5.3; since the swap,
//! T1f.5.5.4, the only parser).
//!
//! ```sh
//! # each file's report: `parses, n forms`, or every error recovery reached
//! cargo run --release -p ein-ir --example generated_parser -- check FILE…
//! # N seeded corpus mutants (`ein_corpus::mutants`), each parsed twice: how
//! # many the parser refuses, the recovery steps of its
//! # refusals, how many recovery cut at its budget, the slowest, and whether
//! # any report differed between its two parses
//! cargo run --release -p ein-ir --example generated_parser -- mutants N [SEED [DIR]]
//! # with DIR, each mutant recovery cut at the budget is written there; with
//! # `--without-priority` anywhere, the sources' `:priority` clauses are taken
//! # out first, as S1f.5.15 will (the grammar has none: the user, 2026-11-08)
//! ```
//!
//! `mutants` is how `lrpar`'s `RECOVERY_STEP_BUDGET` was chosen: run once with
//! the vendored constant raised far past any mutant's need, so that the step
//! counts are what recovery takes rather than what it was allowed, then once
//! at the chosen budget for the times (`plans/…/s1f.5.5_the_grammar.md`
//! § T3 record).

use std::time::Instant;

use ein_corpus::mutants::{mutants, sources, without_priority};
use ein_ir::Ast;
use ein_ir::grammar::{self, RECOVERY_STEP_BUDGET};

fn check(files: &[String]) {
    for f in files {
        let text = std::fs::read_to_string(f).unwrap_or_else(|e| panic!("{f}: {e}"));
        let mut ast = Ast::new();
        match grammar::parse(&mut ast, &text, Some(f)) {
            Ok(forms) => println!("{f}: parses, {} forms", forms.len()),
            Err(r) => print!(
                "{f}: refused, {} error(s), {} recovery steps\n{r}",
                r.errors.len(),
                r.recovery_steps
            ),
        }
    }
}

/// The process's peak resident set, in KiB, reset to the current one when
/// `reset`: Linux's `VmHWM`, which writing `5` to `clear_refs` resets. `0`
/// where there is no `/proc`.
fn peak_rss_kib(reset: bool) -> u64 {
    if reset {
        let _ = std::fs::write("/proc/self/clear_refs", "5");
    }
    std::fs::read_to_string("/proc/self/status")
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with("VmHWM:"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

fn percentile(sorted: &[u64], p: f64) -> u64 {
    if sorted.is_empty() {
        return 0;
    }
    let i = ((sorted.len() - 1) as f64 * p).round() as usize;
    sorted[i]
}

fn sweep(n: usize, seed: u64, cut_dir: Option<&str>, strip: bool) {
    let mut src = sources();
    if strip {
        for (_, text) in &mut src {
            *text = without_priority(text);
        }
    }
    let set = mutants(&src, n, seed);
    let (mut refused, mut cut, mut differ) = (0, 0, 0);
    let mut steps: Vec<u64> = Vec::new();
    let mut errors = 0usize;
    let mut slowest: Vec<(f64, u64, usize, String)> = Vec::new();
    let mut total = 0.0;
    let which_name = |i: usize| {
        src[i]
            .0
            .strip_prefix(ein_corpus::repo_root())
            .map(|p| p.display().to_string())
            .unwrap_or_default()
    };
    let mut most_rss = (0u64, String::new());
    for (which, text) in &set {
        let before = peak_rss_kib(true);
        let t0 = Instant::now();
        let mut b = Ast::new();
        let first = grammar::parse(&mut b, text, None);
        let secs = t0.elapsed().as_secs_f64();
        total += secs;
        let grew = peak_rss_kib(false).saturating_sub(before);
        if grew > most_rss.0 {
            most_rss = (grew, which_name(*which));
        }
        let mut c = Ast::new();
        let second = grammar::parse(&mut c, text, None);
        let (Err(r), Err(r2)) = (&first, &second) else {
            if first.is_err() != second.is_err() {
                differ += 1;
            }
            continue;
        };
        if r.to_string() != r2.to_string() {
            differ += 1;
        }
        refused += 1;
        errors += r.errors.len();
        cut += usize::from(r.cut);
        if let (true, Some(dir)) = (r.cut, cut_dir) {
            let _ = std::fs::create_dir_all(dir);
            let name = format!(
                "{dir}/cut-{:05}-{}.ein",
                text.len(),
                which_name(*which).replace('/', "_")
            );
            std::fs::write(&name, text).unwrap_or_else(|e| panic!("{name}: {e}"));
        }
        steps.push(r.recovery_steps);
        slowest.push((secs, r.recovery_steps, r.errors.len(), which_name(*which)));
    }
    steps.sort_unstable();
    slowest.sort_by(|x, y| y.0.total_cmp(&x.0));
    println!(
        "{n} mutants (seed {seed:#x}) of {} corpus files under 6 000 bytes{}; budget {RECOVERY_STEP_BUDGET} steps",
        src.len(),
        if strip { ", `:priority` taken out" } else { "" }
    );
    println!("  the parser refuses {refused}, reporting {errors} errors");
    println!(
        "  recovery steps per refusal: median {}, 90 % {}, 99 % {}, 99.9 % {}, max {}",
        percentile(&steps, 0.5),
        percentile(&steps, 0.9),
        percentile(&steps, 0.99),
        percentile(&steps, 0.999),
        steps.last().copied().unwrap_or(0)
    );
    for budget in [10_000u64, 20_000, 30_000, 50_000, 100_000, 1_000_000] {
        let over = steps.iter().filter(|&&s| s >= budget).count();
        println!("  refusals at or past {budget} steps: {over}");
    }
    println!("  cut at the budget: {cut}");
    println!("  reports that differed between two parses: {differ}");
    println!("  the parser, first parse of each: {total:.2} s in all");
    println!(
        "  the most one parse grew the peak resident set: {} KiB, a mutant of {}",
        most_rss.0, most_rss.1
    );
    for (secs, s, e, name) in slowest.iter().take(5) {
        println!("  slowest: {secs:.3} s, {s} steps, {e} errors, a mutant of {name}");
    }
}

fn main() {
    let all: Vec<String> = std::env::args().skip(1).collect();
    let strip = all.iter().any(|a| a == "--without-priority");
    let args: Vec<String> = all
        .into_iter()
        .filter(|a| a != "--without-priority")
        .collect();
    match args.first().map(String::as_str) {
        Some("check") => check(&args[1..]),
        Some("mutants") => {
            let n = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(5000);
            let seed = args
                .get(2)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0x9E37_79B9_7F4A_7C15);
            sweep(n, seed, args.get(3).map(String::as_str), strip);
        }
        _ => {
            eprintln!("generated_parser check FILE… | mutants N [SEED [DIR]] [--without-priority]")
        }
    }
}
