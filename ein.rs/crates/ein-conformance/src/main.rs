//! `ein-conformance` — the M1a parity harness.
//!
//! Runs two implementations of `ein` over the shared corpus and diffs what
//! they produced, at a chosen tier. It is a normal binary rather than a test
//! harness bolted on, because that is how it will actually get used during
//! the port: by hand, on one fixture, while chasing one difference.
//!
//! ```text
//! ein-conformance run  --impl-a "…" --impl-b "…" [--tier T3] [--group G]…
//! ein-conformance list [--group G]…
//! ein-conformance diff a.jsonl b.jsonl          (S1a.0.2)
//! ```
//!
//! The Python-vs-Python case is not a curiosity, it is the acceptance gate
//! for P1a.0: a harness that cannot detect a difference between an
//! implementation and itself cannot detect one between two implementations
//! either. The same shape, with different `--env-a` / `--env-b`, is the
//! determinism sweep — one implementation, several `PYTHONHASHSEED`s.
//!
//! Design: `plans/m1a_rust/design/01_parity_contract.md`.

mod corpus;
mod events;
mod normalise;
mod plan;
mod run;
mod tier;

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use corpus::{Corpus, Entry};
use run::Impl;
use tier::{Outcome, Tier};

const USAGE: &str = "\
ein-conformance — run two ein implementations over the corpus and diff them

USAGE:
    ein-conformance run  --impl-a CMD --impl-b CMD [OPTIONS]
    ein-conformance list [OPTIONS]
    ein-conformance diff A.jsonl B.jsonl

OPTIONS (diff):
    --classes           always print the per-event-kind summary

OPTIONS (run, list):
    --impl-a CMD        command prefix for side a, e.g. \"python3 -m ein.cli\"
    --impl-b CMD        command prefix for side b
    --tier T0|T1|T2|T3  comparison tier (default: T3)
    --group NAME        restrict to a corpus group (repeatable)
    --filter SUBSTR     restrict to entries whose path contains SUBSTR
    --include-slow      include entries marked slow (nightly tier)
    --jobs N            parallel cells (default: available parallelism)
    --corpus PATH       manifest (default: <repo>/conformance/corpus.toml)
    --repo PATH         repo root (default: found by walking up from cwd)
    --out DIR           artefact root (default: <repo>/conformance/out)
    --env K=V           env override for both sides (repeatable)
    --env-a K=V         env override for side a only (repeatable)
    --env-b K=V         env override for side b only (repeatable)
    --timeout SECS      per-cell timeout (default: 300)
    -v, --verbose       print every cell, not only the failures
";

fn main() -> std::process::ExitCode {
    match real_main() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("ein-conformance: {e}");
            std::process::ExitCode::from(2)
        }
    }
}

struct Args {
    cmd: String,
    impl_a: Option<String>,
    impl_b: Option<String>,
    tier: Tier,
    groups: Vec<String>,
    filter: Option<String>,
    include_slow: bool,
    jobs: usize,
    corpus: Option<PathBuf>,
    repo: Option<PathBuf>,
    out: Option<PathBuf>,
    env: Vec<(String, String)>,
    env_a: Vec<(String, String)>,
    env_b: Vec<(String, String)>,
    timeout: u64,
    verbose: bool,
    classes: bool,
    rest: Vec<String>,
}

fn parse_args() -> Result<Args, String> {
    let mut it = std::env::args().skip(1);
    let cmd = it.next().unwrap_or_else(|| "help".into());
    let mut a = Args {
        cmd,
        impl_a: None,
        impl_b: None,
        tier: Tier::T3,
        groups: Vec::new(),
        filter: None,
        include_slow: false,
        jobs: std::thread::available_parallelism().map_or(4, |n| n.get()),
        corpus: None,
        repo: None,
        out: None,
        env: Vec::new(),
        env_a: Vec::new(),
        env_b: Vec::new(),
        timeout: 300,
        verbose: false,
        classes: false,
        rest: Vec::new(),
    };
    let kv = |s: String| -> Result<(String, String), String> {
        s.split_once('=')
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .ok_or_else(|| format!("expected KEY=VALUE, got {s:?}"))
    };
    while let Some(arg) = it.next() {
        let mut need = |what: &str| -> Result<String, String> {
            it.next().ok_or_else(|| format!("{what} needs a value"))
        };
        match arg.as_str() {
            "--impl-a" => a.impl_a = Some(need("--impl-a")?),
            "--impl-b" => a.impl_b = Some(need("--impl-b")?),
            "--tier" => a.tier = Tier::parse(&need("--tier")?)?,
            "--group" => a.groups.push(need("--group")?),
            "--filter" => a.filter = Some(need("--filter")?),
            "--include-slow" => a.include_slow = true,
            "--jobs" => {
                a.jobs = need("--jobs")?
                    .parse()
                    .map_err(|e| format!("--jobs: {e}"))?
            }
            "--corpus" => a.corpus = Some(PathBuf::from(need("--corpus")?)),
            "--repo" => a.repo = Some(PathBuf::from(need("--repo")?)),
            "--out" => a.out = Some(PathBuf::from(need("--out")?)),
            "--env" => a.env.push(kv(need("--env")?)?),
            "--env-a" => a.env_a.push(kv(need("--env-a")?)?),
            "--env-b" => a.env_b.push(kv(need("--env-b")?)?),
            "--timeout" => {
                a.timeout = need("--timeout")?
                    .parse()
                    .map_err(|e| format!("--timeout: {e}"))?;
            }
            "-v" | "--verbose" => a.verbose = true,
            "--classes" => a.classes = true,
            "-h" | "--help" => {
                print!("{USAGE}");
                std::process::exit(0);
            }
            other if other.starts_with('-') => return Err(format!("unknown flag {other}")),
            other => a.rest.push(other.to_string()),
        }
    }
    Ok(a)
}

fn real_main() -> Result<std::process::ExitCode, String> {
    let args = parse_args()?;
    match args.cmd.as_str() {
        "run" => cmd_run(args),
        "list" => cmd_list(args),
        "diff" => cmd_diff(args),
        "help" | "--help" | "-h" => {
            print!("{USAGE}");
            Ok(std::process::ExitCode::SUCCESS)
        }
        other => Err(format!("unknown subcommand {other:?}\n\n{USAGE}")),
    }
}

/// Find the repo root by walking up for `conformance/corpus.toml`.
fn find_repo(explicit: Option<PathBuf>) -> Result<PathBuf, String> {
    if let Some(p) = explicit {
        return Ok(p);
    }
    let mut dir = std::env::current_dir().map_err(|e| e.to_string())?;
    loop {
        if dir.join("conformance/corpus.toml").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            return Err("no conformance/corpus.toml above the cwd; pass --repo".into());
        }
    }
}

/// One unit of work: an entry, one of its runs.
struct Cell<'a> {
    entry: &'a Entry,
    run: String,
}

fn cells<'a>(entries: &[&'a Entry]) -> Vec<Cell<'a>> {
    entries
        .iter()
        .flat_map(|e| {
            e.all_runs()
                .into_iter()
                .map(move |r| Cell { entry: e, run: r })
        })
        .collect()
}

fn cmd_diff(args: Args) -> Result<std::process::ExitCode, String> {
    let [a, b] = args.rest.as_slice() else {
        return Err("diff takes exactly two .jsonl paths".into());
    };
    let report = events::diff(Path::new(a), Path::new(b))?;
    let same = events::print(&report, args.classes);
    Ok(if same {
        std::process::ExitCode::SUCCESS
    } else {
        std::process::ExitCode::from(1)
    })
}

fn cmd_list(args: Args) -> Result<std::process::ExitCode, String> {
    let repo = find_repo(args.repo)?;
    let manifest = args
        .corpus
        .unwrap_or_else(|| repo.join("conformance/corpus.toml"));
    let corpus = Corpus::load(&manifest)?;
    let entries = corpus.select(&args.groups, args.filter.as_deref(), true);
    let cells = cells(&entries);
    for c in &cells {
        let slow = if c.entry.slow { " [slow]" } else { "" };
        println!(
            "{:<14} {:<46} {}{}",
            c.entry.group, c.entry.path, c.run, slow
        );
    }
    for e in &entries {
        if let Some(note) = &e.note {
            println!("\n{}: {note}", e.path);
        }
    }
    println!("\n{} entries, {} cells", entries.len(), cells.len());
    Ok(std::process::ExitCode::SUCCESS)
}

struct CellResult {
    path: String,
    run: String,
    group: String,
    outcome: Outcome,
    wall_a: Duration,
    wall_b: Duration,
}

fn cmd_run(args: Args) -> Result<std::process::ExitCode, String> {
    let repo = find_repo(args.repo.clone())?;
    let manifest = args
        .corpus
        .clone()
        .unwrap_or_else(|| repo.join("conformance/corpus.toml"));
    let corpus = Corpus::load(&manifest)?;
    let out_root = args
        .out
        .clone()
        .unwrap_or_else(|| repo.join("conformance/out"));

    let mut a = Impl::parse("a", args.impl_a.as_deref().ok_or("--impl-a is required")?)?;
    let mut b = Impl::parse("b", args.impl_b.as_deref().ok_or("--impl-b is required")?)?;
    a.env.extend(args.env.iter().cloned());
    b.env.extend(args.env.iter().cloned());
    a.env.extend(args.env_a.iter().cloned());
    b.env.extend(args.env_b.iter().cloned());

    let entries = corpus.select(&args.groups, args.filter.as_deref(), args.include_slow);
    let cells = cells(&entries);
    if cells.is_empty() {
        return Err("selection matched no cells".into());
    }
    // A stale tree would let a run that wrote nothing look like a run that
    // wrote the same thing as last time.
    if out_root.exists() {
        std::fs::remove_dir_all(&out_root).map_err(|e| format!("{}: {e}", out_root.display()))?;
    }

    eprintln!(
        "ein-conformance {tier}: {n} cells over {e} entries, {j} jobs\n  a: {a:?}\n  b: {b:?}",
        tier = args.tier,
        n = cells.len(),
        e = entries.len(),
        j = args.jobs,
        a = a.prefix.join(" "),
        b = b.prefix.join(" "),
    );

    let cursor = AtomicUsize::new(0);
    let results: Mutex<Vec<CellResult>> = Mutex::new(Vec::with_capacity(cells.len()));
    let done = AtomicUsize::new(0);
    let timeout = Duration::from_secs(args.timeout);

    std::thread::scope(|scope| {
        for _ in 0..args.jobs.max(1) {
            scope.spawn(|| {
                loop {
                    let i = cursor.fetch_add(1, Ordering::Relaxed);
                    let Some(cell) = cells.get(i) else { break };
                    let r = one_cell(cell, &a, &b, &repo, &out_root, args.tier, timeout);
                    let n = done.fetch_add(1, Ordering::Relaxed) + 1;
                    if args.verbose || r.outcome.is_diff() {
                        let mark = match &r.outcome {
                            Outcome::Same => "ok  ",
                            Outcome::Diff(_) => "DIFF",
                            Outcome::Skipped(_) => "skip",
                        };
                        eprintln!("[{n}/{}] {mark} {} :: {}", cells.len(), r.path, r.run);
                        match &r.outcome {
                            Outcome::Diff(d) => d.iter().for_each(|l| eprintln!("        {l}")),
                            Outcome::Skipped(why) => eprintln!("        {why}"),
                            Outcome::Same => {}
                        }
                    }
                    results.lock().expect("results").push(r);
                }
            });
        }
    });

    let mut results = results.into_inner().expect("results");
    results.sort_by(|x, y| (&x.path, &x.run).cmp(&(&y.path, &y.run)));
    Ok(report(&results, args.tier))
}

fn one_cell(
    cell: &Cell<'_>,
    a: &Impl,
    b: &Impl,
    repo: &Path,
    out_root: &Path,
    tier: Tier,
    timeout: Duration,
) -> CellResult {
    let base = out_root
        .join(plan::slug(&cell.entry.path))
        .join(plan::slug(&cell.run));
    let mk = |imp: &Impl| -> Result<run::Capture, String> {
        let out = base.join(&imp.side);
        let argv = plan::argv(&cell.run, &cell.entry.path, &out, tier == Tier::T2);
        run::execute(imp, &argv, repo, &out, timeout)
    };
    let (ca, cb) = (mk(a), mk(b));
    let (outcome, wall_a, wall_b) = match (ca, cb) {
        (Ok(x), Ok(y)) => {
            let o = if cell.entry.group == "crash-parity" {
                tier::compare_crash(&x, &y)
            } else {
                tier::compare(tier, &x, &y)
            };
            (o, x.wall, y.wall)
        }
        (Err(e), _) | (_, Err(e)) => (
            Outcome::Diff(vec![format!("harness error: {e}")]),
            Duration::ZERO,
            Duration::ZERO,
        ),
    };
    CellResult {
        path: cell.entry.path.clone(),
        run: cell.run.clone(),
        group: cell.entry.group.clone(),
        outcome,
        wall_a,
        wall_b,
    }
}

fn report(results: &[CellResult], tier: Tier) -> std::process::ExitCode {
    use std::collections::BTreeMap;
    let mut by_group: BTreeMap<&str, [usize; 3]> = BTreeMap::new();
    for r in results {
        let slot = by_group.entry(&r.group).or_default();
        match r.outcome {
            Outcome::Same => slot[0] += 1,
            Outcome::Diff(_) => slot[1] += 1,
            Outcome::Skipped(_) => slot[2] += 1,
        }
    }
    println!();
    println!("{:<16} {:>7} {:>7} {:>7}", "group", "same", "DIFF", "skip");
    println!("{}", "─".repeat(40));
    let (mut same, mut diff, mut skip) = (0, 0, 0);
    for (g, [s, d, k]) in &by_group {
        println!("{g:<16} {s:>7} {d:>7} {k:>7}");
        same += s;
        diff += d;
        skip += k;
    }
    println!("{}", "─".repeat(40));
    println!("{:<16} {same:>7} {diff:>7} {skip:>7}", "total");
    let wall: Duration = results.iter().map(|r| r.wall_a + r.wall_b).sum();
    println!("\ntier {tier}, {:.1}s of engine time", wall.as_secs_f64());
    if diff > 0 {
        println!("\n{diff} differing cells:");
        for r in results.iter().filter(|r| r.outcome.is_diff()) {
            println!("  {} :: {}", r.path, r.run);
            if let Outcome::Diff(d) = &r.outcome {
                for line in d.iter().take(4) {
                    println!("      {line}");
                }
            }
        }
        return std::process::ExitCode::from(1);
    }
    std::process::ExitCode::SUCCESS
}
