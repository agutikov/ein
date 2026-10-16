//! `ein solve`'s region modes — M1e2
//! [S1e2.1a.9](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
//! T1e2.1a.9.4: `--minimal`, `--maximal`, `--single` and `--region-budget`,
//! with `-n`, answered by [`ein_infer::explore`].
//!
//! **Until S1e2.1a.5 the default is still the walk** (ruling 4: the goldens
//! move once, with the read-out that prints the new default). One of the four
//! region flags is therefore what takes a run here, and every option of the
//! walk's is refused beside them rather than ignored: what an option means
//! under a region mode is S1e2.1a.5's to say, and a flag that does nothing is
//! a claim nobody checks.
//!
//! The read-out is **interim**: one header block, one line per fact. Its
//! words and layout are S1e2.1a.5's page diff.

use std::path::Path;
use std::time::{Duration, Instant};

use clap::ArgMatches;
use clap::parser::ValueSource;
use ein_core::Terms;
use ein_infer::explore::{self, Answer, Ask, Mode, Route, Stopped, Verdict, Which};
use ein_ir::Ast;

/// The `solve` options a region mode takes. Every other one is the walk's.
const TAKES: [&str; 10] = [
    "file",
    "solutions",
    "max-time",
    "jobs",
    "region-limit",
    "stats",
    "minimal",
    "maximal",
    "single",
    "region-budget",
];

/// More states than this, under describe, are counted and not listed.
const LIST_MAX: usize = 64;

/// Whether this `solve` run asks a region mode.
pub fn wanted(m: &ArgMatches) -> bool {
    m.get_flag("minimal")
        || m.get_flag("maximal")
        || m.get_flag("single")
        || m.get_one::<i64>("region-budget").is_some()
}

/// The mode the flags ask for.
pub fn mode_of(m: &ArgMatches) -> Mode {
    let which = if m.get_flag("minimal") {
        Which::Minimal
    } else if m.get_flag("maximal") {
        Which::Maximal
    } else {
        Which::All
    };
    let ask = if m.get_flag("single") {
        Ask::Single
    } else if m.value_source("solutions") == Some(ValueSource::CommandLine) {
        Ask::Find(*m.get_one::<i64>("solutions").expect("defaulted") as u64)
    } else {
        Ask::Describe
    };
    Mode { which, ask }
}

pub fn run(m: &ArgMatches) -> i32 {
    let solve = crate::cmdline::command();
    let solve = solve.find_subcommand("solve").expect("solve is registered");
    for arg in solve.get_arguments() {
        let id = arg.get_id().as_str();
        if TAKES.contains(&id) || m.value_source(id) != Some(ValueSource::CommandLine) {
            continue;
        }
        let name = arg
            .get_long()
            .map(|l| format!("--{l}"))
            .unwrap_or_else(|| id.to_string());
        eprintln!(
            "error: {name} is the walk's: the region modes (--minimal, --maximal, --single, \
             --region-budget) do not take it until S1e2.1a.5 says what it means there"
        );
        return 2;
    }
    let file = m.get_one::<String>("file").expect("required").clone();
    let mode = mode_of(m);
    let mut opts = explore::Options::new(mode);
    if let Some(&b) = m.get_one::<i64>("region-budget") {
        opts.budget = b as u64;
    }
    opts.limit = (*m.get_one::<i64>("region-limit").unwrap_or(&16)).clamp(0, 63) as u32;
    opts.jobs = crate::solve::resolve_jobs(*m.get_one::<i64>("jobs").unwrap_or(&1));
    let max_time = m.get_one::<f64>("max-time").copied();
    let stats = m.get_flag("stats");

    let (mut rc, mut index) = (0, 0usize);
    loop {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Some(mut kb) =
            crate::common::load_any_query_or_exit(&mut ast, &mut terms, Path::new(&file), index)
        else {
            return rc.max(1);
        };
        let n_queries = kb.program().queries.len();
        if n_queries > 1 {
            println!("query {} of {n_queries}", index + 1);
        }
        opts.deadline = max_time.map(|s| Instant::now() + Duration::from_secs_f64(s.max(0.0)));
        let answer = match explore::run(&mut kb, &mut terms, &ast, &opts) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{e}");
                return 1;
            }
        };
        print(&answer, &opts, stats);
        rc = rc.max(code(&answer, &file, max_time));
        index += 1;
        if index >= n_queries {
            break;
        }
        println!();
    }
    rc
}

/// The exit code, and the stderr line a `--single` that did not hold owes —
/// the one `:expect` gives (M1d S1d.4.3).
fn code(a: &Answer, file: &str, max_time: Option<f64>) -> i32 {
    if a.stopped == Some(Stopped::Deadline) {
        eprintln!(
            "** aborted: --max-time {} s reached before the question was decided **",
            max_time.unwrap_or(0.0)
        );
        return 2;
    }
    if a.mode.ask != Ask::Single {
        return 0;
    }
    match a.verdict {
        Some(Verdict::Solution) => 0,
        Some(v) => {
            eprintln!(
                "{file}: --single FAILED — expected exactly one state of {}, found {}",
                which_words(a.mode.which),
                match v {
                    Verdict::Ambiguity => "two or more (Ambiguity)",
                    Verdict::Open => "none: every alive node owes (Open)",
                    _ => "none (Contradiction)",
                }
            );
            1
        }
        None => {
            eprintln!(
                "{file}: --single NOT CHECKED — the run stopped before it decided ({})",
                stopped_words(a.stopped.as_ref())
            );
            1
        }
    }
}

fn which_words(w: Which) -> &'static str {
    match w {
        Which::All => "R",
        Which::Minimal => "R's minimal line",
        Which::Maximal => "R's maximal line",
    }
}

fn stopped_words(s: Option<&Stopped>) -> String {
    match s {
        None => "it did not stop".to_string(),
        Some(Stopped::Budget { spent, projected }) => format!(
            "at the budget: {spent} facts derived and {projected} more projected for the next step"
        ),
        Some(Stopped::Limit { n, limit }) => {
            format!("past --region-limit {limit}: n = {n}, and the question needs the whole cube")
        }
        Some(Stopped::Deadline) => "at --max-time".to_string(),
    }
}

fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

/// The interim read-out.
pub fn print(a: &Answer, opts: &explore::Options, stats: bool) {
    let ask = match a.mode.ask {
        Ask::Describe => "describe".to_string(),
        Ask::Find(n) => format!("-n {n}"),
        Ask::Single => "--single".to_string(),
    };
    let what = match a.mode.which {
        Which::All => "all of R",
        Which::Minimal => "the minimal line",
        Which::Maximal => "the maximal line",
    };
    println!("region      {what} · {ask}");
    println!(
        "route       {}",
        match a.route {
            Route::DownSet => "1, the alive down-set",
            Route::Cube => "2, the cube",
        }
    );
    let class = if a.monotonicity.is_monotone() {
        "monotone (the static check)".to_string()
    } else {
        format!(
            "not monotone ({}): no licence but the cube",
            plural(a.monotonicity.sites.len(), "site", "sites")
        )
    };
    println!("program     n = {} root candidates · {class}", a.n());
    println!(
        "licences    {}",
        a.licences
            .iter()
            .map(|l| l.label())
            .collect::<Vec<_>>()
            .join(" · ")
    );

    if let Some(c) = a.counts {
        let exact = if a.exhausted { "" } else { "  (a lower bound)" };
        println!("k_s         {}{exact}", c.k_s);
        println!("k_c         {}{exact}", c.k_c);
    }
    if let Some(sh) = &a.shape {
        println!("∅ in R      {}", if sh.empty_in_r { "yes" } else { "no" });
        println!(
            "minimal     {}",
            plural(sh.minimal.len(), "commitment", "commitments")
        );
        println!(
            "maximal     {}",
            plural(sh.maximal.len(), "commitment", "commitments")
        );
        println!("components  {}", sh.components);
        if let Some(s) = sh.subcubes {
            println!("sub-cubes   {s}");
        }
        println!(
            "N           {}",
            plural(sh.nogoods, "minimal dead set", "minimal dead sets")
        );
        println!(
            "merged      {}",
            plural(
                sh.merged,
                "state reached by several commitments",
                "states reached by several commitments"
            )
        );
        if let Some(h) = sh.holes {
            println!("holes       {h}");
        }
    }

    let k = a.states.len();
    let answer = if a.mode.ask == Ask::Single {
        String::new()
    } else if a.exhausted {
        if k == 0 {
            match (a.mode.which, a.root_dead) {
                (_, true) => "none: R is empty, root is dead".to_string(),
                _ => "none: R is empty, every alive node owes".to_string(),
            }
        } else {
            format!("{} — all of them", plural(k, "state", "states"))
        }
    } else if a.stopped.is_some() {
        format!(
            "{} found before the run stopped — a lower bound",
            plural(k, "state", "states")
        )
    } else {
        format!(
            "{} — the first the route met; more may exist",
            plural(k, "state", "states")
        )
    };
    if !answer.is_empty() {
        println!("answer      {answer}");
    }
    if a.mode.ask == Ask::Single {
        match a.verdict {
            Some(v) => println!("verdict     {}", v.word()),
            None => println!("verdict     not decided"),
        }
    }
    if let Some(s) = &a.stopped {
        let mut why = stopped_words(Some(s));
        if matches!(s, Stopped::Budget { .. }) {
            why.push_str(&format!(" over --region-budget {}", opts.budget));
        }
        println!("stopped     {why}");
    }
    println!("exhausted   {}", a.exhausted);
    if stats {
        println!(
            "work        {} saturations · {} facts beyond root · {:.1} ms",
            a.saturations,
            a.facts,
            a.wall.as_secs_f64() * 1e3
        );
    }

    let shown = if a.mode.ask == Ask::Describe && k > LIST_MAX {
        LIST_MAX
    } else {
        k
    };
    for (i, s) in a.states.iter().take(shown).enumerate() {
        println!();
        if s.closure.is_empty() {
            println!("state {}     root itself: ∅, holding no candidate", i + 1);
        } else {
            println!(
                "state {}     {}",
                i + 1,
                plural(s.closure.len(), "candidate", "candidates")
            );
        }
        if a.mode.which == Which::Minimal {
            let from: Vec<String> = a
                .minimal
                .iter()
                .filter(|(_, at)| *at == i)
                .map(|(set, _)| a.name(set))
                .collect();
            println!("  minimal   {}", from.join(" · "));
        } else if a.mode.which == Which::All && !s.set.is_empty() {
            println!("  from      {}", a.name(&s.set));
        }
        for &b in s.closure.iter() {
            println!("  {}", a.candidates[b as usize].text);
        }
    }
    if shown < k {
        println!();
        println!(
            "… and {} more states: listing them all is the read-out's, S1e2.1a.5",
            k - shown
        );
    }
}
