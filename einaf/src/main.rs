//! `einaf`: EinAf's command.
//!
//! Two subcommands, each one bounded call of the kernel,
//! [`einaf::kernel::call`], on one program and one question:
//! `einaf outcome FILE` prints the outcome under each reading the question
//! gives, and `einaf feedback FILE` prints the whole feedback object as
//! JSON. The outcome is read off the object. Every outcome exits 0; a usage
//! error exits 2, and a fault of the kernel's 1.
//!
//! And the model's ([`einaf::model`]): `einaf render` renders a request's
//! prompt through a chat template, `einaf generate` makes one call and
//! prints its record, `einaf model` loads the model and prints its pin, and
//! `einaf probe` writes the probe's plan and runs it; `einaf steps` runs the
//! formalization in steps ([`einaf::formalize`]) and the contract's check,
//! and `einaf catalogue` prints the catalogue the steps select from. A call
//! is answered by
//! the model (`--model PATH` or `$EINAF_MODEL`, with the `llama` feature) or
//! by a record (`--replay FILE`, always).

use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use einaf::kernel::{Bounds, Feedback, Part, Question, Source, call};

const USAGE: &str = "\
usage: einaf outcome FILE [QUESTION] [BOUNDS]
       einaf feedback FILE [QUESTION] [BOUNDS]
       einaf render --template FILE REQUEST.json
       einaf generate RUNTIME REQUEST.json...
       einaf grammar write [--check]
       einaf grammar accept [--grammar FILE] [--root NAME] FILE...
       einaf grammar sample [--grammar FILE] [--root NAME] [--seed N] [--count N]
       einaf grammar render whole|1|2|2b|3 [--program FILE] [--text FILE] [--fence] [--after-thinking] [--request]
       einaf grammar catalogue
       einaf grammar llama RUNTIME [--grammar FILE] [--root NAME] [--list FILE] [--special] FILE...
       einaf grammar --json-schema FILE
       einaf model --model PATH [--ctx N] [--verbose]
       einaf probe plan [--out FILE]
       einaf probe run --plan FILE --out FILE RUNTIME [--only PREFIX]...
       einaf steps run TEXT --out DIR RUNTIME [--id ID] [--answer-line zebra|word|generic]
                [--mode steps|whole] [--no-grammar] [--request-mode thinking-off]
                [--catalogue whole|minimal] [--max-tokens N] [-T SECONDS]
       einaf steps check PROGRAM [--text TEXT] [--step 1|2|3] [--catalogue whole|minimal]
       einaf catalogue [--minimal] [--generated] [--grammar] [--tokens MODEL]

outcome   the outcome of an Ein program under one question about its answer, R:
          invalid, flagged, not decided, inconsistent, incomplete, several or
          one, under each reading the question gives (the part of R one and
          several are counted over)
feedback  the feedback object of the same call, einaf-feedback/1, as JSON

QUESTION, as `ein solve` asks it (none: describe all of R):
  --single               is R one state?
  --maximal --single     is the maximal line one state?
  --minimal --single     is the minimal line one state?
  --maximal, --minimal   the line, every state of it
  --count                R's two counts
  --no-search            root alone; R is not searched

BOUNDS, each set on every call:
  --region-budget N      the facts budget, 1048576 by default
  --region-limit N       the largest cube a flagged program gets, 16 by default
  --max-steps N          the firings one saturation may take, 1048576 by default
  -T, --max-time SECONDS the wall clock, none by default

render    a request's prompt, rendered through a chat template, as text
generate  a call of the model a request, in order, in one process; its record
          as one JSON line a call
grammar   the grammars: `write` writes einaf/grammars/ein.gbnf from ein.l and
          ein.y and its manifest (`--check`: write nothing, exit 1 where they
          differ); `accept` runs EinAf's acceptor of a grammar on files (the
          whole grammar from its root `program` by default); `sample` writes
          texts a grammar allows; `render` writes the whole grammar's root
          `root` or a step's grammar for a program, or the grammar a request
          carries; `catalogue` the catalogue the steps' grammars are rendered
          on; `llama` reads files through llama.cpp's grammar sampler with
          the model's vocabulary (`llama`; `--special`: a marker of the chat
          template one token, as the model generates it); `--json-schema`
          the GBNF llama.cpp writes for
          a JSON schema (`llama`)
model     the model loaded, its pin, the runtime and the context as JSON
steps     the formalization in steps: `run` writes a program for a text in three
          calls of the model (or one, `--mode whole`), each under its step's
          grammar inside a fenced block (`--no-grammar`: none), thinking off,
          each step's program judged at root, and writes the run to --out
          (calls.jsonl, every call, which --replay reads; run.json; each
          step's output; program.ein); `check` runs the contract's check on
          a program, its findings for a step's program
catalogue the catalogue of the library the steps select from, as a prompt
          shows it (`--minimal`: the relation algebra's alone); the library's
          rules, generated (`--generated`); the catalogue the steps' grammars
          are rendered on (`--grammar`); its length and the prompts' in the
          model's tokens (`--tokens MODEL`, `llama`)
probe     the probe of Ein written with no grammar: `plan` writes every run
          as JSON lines, `run` makes each call and judges its answer through
          the kernel, a line a run appended to --out (a run already there is
          not made again)

RUNTIME, what answers a call:
  --model PATH           the model's file, by the binding to llama.cpp (the
                         `llama` feature); $EINAF_MODEL where none is given
  --ctx N                the context's length in tokens, 65536 by default
  --verbose              llama.cpp's own log on stderr
  --replay FILE          a record: each request answered by the call it holds
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("outcome") => command(&args[1..], Print::Outcome),
        Some("feedback") => command(&args[1..], Print::Feedback),
        Some("render") => model_cmd::render(&args[1..]),
        Some("generate") => model_cmd::generate(&args[1..]),
        Some("model") => model_cmd::model(&args[1..]),
        Some("grammar") => grammar_cmd::grammar(&args[1..]),
        Some("probe") => model_cmd::probe(&args[1..]),
        Some("steps") => steps_cmd::steps(&args[1..]),
        Some("catalogue") => steps_cmd::catalogue_cmd(&args[1..]),
        Some("-h" | "--help") => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
        _ => usage_error(
            "expected a subcommand: outcome, feedback, render, generate, grammar, model, probe, steps or catalogue",
        ),
    }
}

mod grammar_cmd;
mod model_cmd;
mod steps_cmd;

fn usage_error(why: &str) -> ExitCode {
    eprintln!("error: {why}\n\n{USAGE}");
    ExitCode::from(2)
}

/// What a subcommand prints of the object.
#[derive(Clone, Copy)]
enum Print {
    Outcome,
    Feedback,
}

/// What a subcommand was asked.
struct Asked {
    file: String,
    question: Question,
    bounds: Bounds,
}

fn parse(args: &[String]) -> Result<Asked, String> {
    let mut file = None;
    let (mut single, mut part, mut count, mut no_search) = (false, None, false, false);
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
                if !s.is_finite() {
                    return Err(format!("-T {v}: not a number of seconds"));
                }
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
    })
}

fn command(args: &[String], print: Print) -> ExitCode {
    let asked = match parse(args) {
        Ok(a) => a,
        Err(why) => return usage_error(&why),
    };
    let f = call(
        Source::File(Path::new(&asked.file)),
        &[asked.question],
        asked.bounds,
    )
    .pop()
    .expect("one object a question");
    match print {
        Print::Feedback => println!("{}", f.to_json()),
        Print::Outcome => print_outcome(&asked, &f),
    }
    if let Some(fault) = &f.fault {
        eprintln!(
            "error: the kernel failed ({}): {}",
            fault.kind, fault.message
        );
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

/// The outcome under each reading, a line each, read off the object; an
/// invalid program's findings on stderr, as `ein solve` prints them.
fn print_outcome(asked: &Asked, f: &Feedback) {
    match asked.question {
        Question::Describe => println!("einaf outcome · {}", asked.file),
        q => println!("einaf outcome · {} {}", asked.file, q.flags()),
    }
    for r in f.outcome.iter().flatten() {
        println!(
            "  {:<13} {:<23} {}",
            r.outcome.word(),
            r.part.words(),
            r.why
        );
    }
    let invalid = f
        .outcome
        .iter()
        .flatten()
        .any(|r| r.outcome == einaf::kernel::Outcome::Invalid);
    if invalid {
        for d in &f.diagnostics {
            eprintln!("{}", d.message);
        }
    }
}
