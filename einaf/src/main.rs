//! `einaf`: EinAf's command.
//!
//! One subcommand so far, `einaf outcome FILE`, which loads a program, asks
//! it one question about its answer as `ein solve` would, and prints the
//! outcome under each reading the question gives. Every outcome exits 0; a
//! usage error exits 2, and an engine error 1.

use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use einaf::kernel::{Bounds, Part, Program, Question, Run, outcome};

const USAGE: &str = "\
usage: einaf outcome FILE [QUESTION] [--region-budget N] [--region-limit N]
                         [--max-steps N] [-T SECONDS] [--read-out]

The outcome of an Ein program under one question about its answer, R: invalid,
flagged, not decided, inconsistent, incomplete, several or one, under each
reading the question gives (the part of R one and several are counted over).

QUESTION, as `ein solve` asks it (none: describe all of R):
  --single               is R one state?
  --maximal --single     is the maximal line one state?
  --minimal --single     is the minimal line one state?
  --maximal, --minimal   the line, every state of it
  --count                R's two counts
  --no-search            root alone; R is not searched

  --region-budget N      the facts budget, 1048576 by default
  --region-limit N       the largest cube a flagged program gets, 16 by default
  --max-steps N          the firings one saturation may take, 1048576 by default
  -T, --max-time SECONDS the wall clock
  --read-out             also print `ein solve`'s read-out of the run
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("outcome") => outcome_command(&args[1..]),
        Some("-h" | "--help") => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
        _ => usage_error("expected a subcommand, `outcome`"),
    }
}

fn usage_error(why: &str) -> ExitCode {
    eprintln!("error: {why}\n\n{USAGE}");
    ExitCode::from(2)
}

/// What `einaf outcome` was asked.
struct Asked {
    file: String,
    question: Question,
    bounds: Bounds,
    read_out: bool,
}

fn parse(args: &[String]) -> Result<Asked, String> {
    let mut file = None;
    let (mut single, mut part, mut count, mut no_search, mut read_out) =
        (false, None, false, false, false);
    let mut bounds = Bounds::default();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--single" => single = true,
            "--maximal" | "--minimal" if part.is_some() => {
                return Err("--minimal and --maximal exclude each other".to_string());
            }
            "--maximal" => part = Some(Part::Maximal),
            "--minimal" => part = Some(Part::Minimal),
            "--count" => count = true,
            "--no-search" => no_search = true,
            "--read-out" => read_out = true,
            "--region-budget" => {
                let v = it.next().ok_or("--region-budget takes a number")?;
                bounds.budget = v
                    .parse()
                    .map_err(|_| format!("--region-budget {v}: not a number of facts"))?;
            }
            "--region-limit" => {
                let v = it.next().ok_or("--region-limit takes a number")?;
                bounds.limit = v
                    .parse()
                    .map_err(|_| format!("--region-limit {v}: not a number of candidates"))?;
            }
            "--max-steps" => {
                let v = it.next().ok_or("--max-steps takes a number")?;
                let n: u64 = v
                    .parse()
                    .map_err(|_| format!("--max-steps {v}: not a number of firings"))?;
                if n == 0 {
                    return Err("--max-steps takes 1 or more".to_string());
                }
                bounds.max_steps = Some(n);
            }
            "-T" | "--max-time" => {
                let v = it.next().ok_or("-T takes a number of seconds")?;
                let s: f64 = v
                    .parse()
                    .map_err(|_| format!("-T {v}: not a number of seconds"))?;
                bounds.clock = Some(Duration::from_secs_f64(s.max(0.0)));
            }
            s if s.starts_with('-') => return Err(format!("unknown option {s}")),
            s if file.is_none() => file = Some(s.to_string()),
            s => return Err(format!("one file only: {s}")),
        }
    }
    let file = file.ok_or("expected a FILE")?;
    let asks = [single, count, no_search].iter().filter(|&&b| b).count();
    if asks > 1 {
        return Err("--single, --count and --no-search exclude each other".to_string());
    }
    if part.is_some() && (count || no_search) {
        return Err("--count and --no-search are over all of R".to_string());
    }
    let question = match (single, count, no_search, part) {
        (true, _, _, p) => Question::Single(p.unwrap_or(Part::All)),
        (_, true, _, _) => Question::Count,
        (_, _, true, _) => Question::NoSearch,
        (_, _, _, Some(p)) => Question::Line(p),
        _ => Question::Describe,
    };
    Ok(Asked {
        file,
        question,
        bounds,
        read_out,
    })
}

fn outcome_command(args: &[String]) -> ExitCode {
    let asked = match parse(args) {
        Ok(a) => a,
        Err(why) => return usage_error(&why),
    };
    let run = match Program::load_file(Path::new(&asked.file)) {
        Ok(program) => match program.ask(asked.question, asked.bounds) {
            Ok(run) => run,
            Err(e) => {
                eprintln!("error: the engine failed: {e}");
                return ExitCode::from(1);
            }
        },
        Err(refusal) => {
            let shown = refusal.message.clone();
            let run = Run::refused(asked.question, refusal);
            print_readings(&asked, &run);
            eprintln!("{shown}");
            return ExitCode::SUCCESS;
        }
    };
    print_readings(&asked, &run);
    if asked.read_out
        && let Some(text) = run.read_out_text(&asked.file, asked.bounds.budget)
    {
        println!();
        print!("{text}");
    }
    ExitCode::SUCCESS
}

fn print_readings(asked: &Asked, run: &Run) {
    match asked.question {
        Question::Describe => println!("einaf outcome · {}", asked.file),
        q => println!("einaf outcome · {} {}", asked.file, q.flags()),
    }
    for part in asked.question.readings() {
        let r = outcome(run, part).expect("a reading the question gives");
        println!(
            "  {:<13} {:<23} {}",
            r.outcome.word(),
            r.part.words(),
            r.why
        );
    }
}
