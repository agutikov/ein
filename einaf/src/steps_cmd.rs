//! `einaf steps` and `einaf catalogue`: the formalization in steps, a run
//! one command, answered by the model or by a record; the contract's check
//! on a program; and the catalogue the steps select from.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

use einaf::catalogue::{self, Catalogue, Size};
use einaf::check;
use einaf::formalize::{self, Input, Mode, Options, RequestMode, Run};
use einaf::kernel::{self, Bounds, Source};
use einaf::model::Recorded;
use einaf::prompts;

use crate::model_cmd::{Opened, RuntimeArgs};
use crate::usage_error;

/// `einaf steps run|check`.
pub fn steps(args: &[String]) -> ExitCode {
    match args.first().map(String::as_str) {
        Some("run") => steps_run(&args[1..]),
        Some("check") => steps_check(&args[1..]),
        _ => usage_error("steps takes `run` or `check`"),
    }
}

/// `einaf steps run TEXT --out DIR RUNTIME [options]`.
fn steps_run(args: &[String]) -> ExitCode {
    let mut rt = RuntimeArgs::new();
    let mut options = Options::default();
    let (mut text, mut out, mut id) = (None::<PathBuf>, None::<PathBuf>, None::<String>);
    let mut line = "generic".to_string();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match rt.take(a, &mut it) {
            Ok(true) => continue,
            Ok(false) => {}
            Err(e) => return usage_error(&e),
        }
        let mut value = |name: &str| it.next().cloned().ok_or(format!("{name} takes a value"));
        let r: Result<(), String> = (|| {
            match a.as_str() {
                "--out" => out = Some(value("--out")?.into()),
                "--id" => id = Some(value("--id")?),
                "--answer-line" => line = value("--answer-line")?,
                "--mode" => {
                    let v = value("--mode")?;
                    options.mode = Mode::parse(&v).ok_or(format!("--mode {v}: steps or whole"))?;
                }
                "--no-grammar" => options.grammar = false,
                "--request-mode" => {
                    let v = value("--request-mode")?;
                    options.request = RequestMode::parse(&v)
                        .ok_or(format!("--request-mode {v}: thinking-off"))?;
                }
                "--catalogue" => {
                    let v = value("--catalogue")?;
                    options.catalogue =
                        Size::parse(&v).ok_or(format!("--catalogue {v}: whole or minimal"))?;
                }
                "--max-tokens" => {
                    let v = value("--max-tokens")?;
                    options.max_tokens = v.parse().map_err(|_| format!("--max-tokens {v}"))?;
                }
                "-T" | "--max-time" => {
                    let v = value("-T")?;
                    options.clock_s = v.parse().map_err(|_| format!("-T {v}"))?;
                }
                s if s.starts_with('-') => return Err(format!("unknown option {s}")),
                s => text = Some(s.into()),
            }
            Ok(())
        })();
        if let Err(e) = r {
            return usage_error(&e);
        }
    }
    let (Some(text_path), Some(out)) = (text, out) else {
        return usage_error("steps run takes a TEXT file and --out DIR");
    };
    let Some(answer_line) = prompts::answer_line(&line) else {
        return usage_error(&format!("--answer-line {line}: zebra, word or generic"));
    };
    let text = match std::fs::read_to_string(&text_path) {
        Ok(t) => t,
        Err(e) => return usage_error(&format!("{}: {e}", text_path.display())),
    };
    let id = id.unwrap_or_else(|| {
        text_path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "text".to_string())
    });
    let mut runtime = match rt.open() {
        Ok(r) => r,
        Err(Opened::Failed(f)) => {
            eprintln!("error: the model did not load: {f}");
            return ExitCode::from(1);
        }
        Err(e) => return usage_error(&e.to_string()),
    };
    let input = Input {
        id,
        text,
        answer_line,
    };
    let mut calls: Vec<Recorded> = Vec::new();
    let run = formalize::run(runtime.as_mut(), &input, options, &mut calls);
    if let Err(e) = write_run(&out, &run, &calls) {
        eprintln!("error: {}: {e}", out.display());
        return ExitCode::from(1);
    }
    print!("{}", summary(&run));
    ExitCode::SUCCESS
}

/// A run's files: every call, the run, each step's output and the program.
fn write_run(dir: &Path, run: &Run, calls: &[Recorded]) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let mut f = std::fs::File::create(dir.join("calls.jsonl"))?;
    for c in calls {
        writeln!(f, "{}", serde_json::to_string(c).expect("a call is JSON"))?;
    }
    std::fs::write(
        dir.join("run.json"),
        serde_json::to_string_pretty(run).expect("a run is JSON") + "\n",
    )?;
    for s in &run.steps {
        if let Some(o) = &s.output {
            std::fs::write(dir.join(format!("step{}.out.ein", s.step)), o)?;
        }
    }
    if let Some(p) = &run.program {
        std::fs::write(dir.join("program.ein"), p)?;
    }
    Ok(())
}

/// **A run in lines**: each step's call, parse and judgement, the check's
/// findings, the complete program's outcome and answer.
pub fn summary(run: &Run) -> String {
    let mut s = format!(
        "{}: {} mode, grammar {}, {}\n",
        run.id,
        match run.options.mode {
            Mode::Steps => "steps",
            Mode::Whole => "whole",
        },
        if run.options.grammar { "on" } else { "off" },
        run.options.catalogue.word()
    );
    for st in &run.steps {
        let parsed = match &st.parsed {
            Some(p) if p.forms.is_some() => format!("{} forms", p.forms.unwrap_or(0)),
            Some(p) => format!(
                "does not parse: {}",
                p.findings.first().map(|f| f.message.as_str()).unwrap_or("")
            ),
            None => "no output".to_string(),
        };
        s.push_str(&format!(
            "  step {:<5} {} prompt tokens, {} generated ({}), {}",
            st.step,
            st.prompt_tokens.unwrap_or(0),
            st.generated_tokens.unwrap_or(0),
            st.stop.as_deref().or(st.failure.as_deref()).unwrap_or("-"),
            parsed
        ));
        if let Some(j) = &st.judged {
            let root = j
                .feedback
                .first()
                .and_then(|f| f.outcome.as_ref())
                .and_then(|o| o.first())
                .map(|r| r.outcome.word().to_string())
                .unwrap_or_else(|| "no outcome".to_string());
            s.push_str(&format!(
                "; at root {root}; H from {} ({}); {} findings",
                j.root.writer.as_deref().unwrap_or("none"),
                j.root.candidates,
                j.check.findings.len()
            ));
        }
        if !st.markers.is_empty() {
            s.push_str(&format!("; {} markers", st.markers.len()));
        }
        s.push('\n');
    }
    if let Some(why) = &run.stopped {
        s.push_str(&format!("  stopped: {why}\n"));
    }
    if let Some(c) = &run.complete {
        for f in &c.feedback {
            let outcome = f
                .outcome
                .as_ref()
                .and_then(|o| o.first())
                .map(|r| r.outcome.word())
                .unwrap_or("no outcome");
            s.push_str(&format!("  complete, {}: {outcome}\n", f.asked.question));
            for q in f
                .queries
                .iter()
                .flatten()
                .filter(|q| q.goal_text.is_some())
                .take(1)
            {
                for c in &q.classes {
                    for l in &c.lines {
                        s.push_str(&format!("    {l}\n"));
                    }
                }
            }
        }
        let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
        for f in &c.check.findings {
            *kinds.entry(f.kind.name()).or_insert(0) += 1;
        }
        s.push_str(&format!(
            "  findings: {}\n",
            if kinds.is_empty() {
                "none".to_string()
            } else {
                kinds
                    .iter()
                    .map(|(k, n)| format!("{k} {n}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        ));
    }
    s
}

/// `einaf steps check PROGRAM --text TEXT [--step N] [--catalogue SIZE]`.
fn steps_check(args: &[String]) -> ExitCode {
    let (mut program, mut text, mut step) = (None::<String>, None::<String>, 3u8);
    let mut size = Size::Whole;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--text" => text = it.next().cloned(),
            "--step" => match it.next().and_then(|v| v.parse().ok()) {
                Some(n @ 1..=3) => step = n,
                _ => return usage_error("--step takes 1, 2 or 3"),
            },
            "--catalogue" => match it.next().and_then(|v| Size::parse(v)) {
                Some(s) => size = s,
                None => return usage_error("--catalogue takes whole or minimal"),
            },
            s if s.starts_with('-') => return usage_error(&format!("unknown option {s}")),
            s => program = Some(s.to_string()),
        }
    }
    let Some(program) = program else {
        return usage_error("steps check takes a PROGRAM");
    };
    let read = |p: &str| std::fs::read_to_string(p).map_err(|e| format!("{p}: {e}"));
    let (source, input) = match (read(&program), text.as_deref().map(read).transpose()) {
        (Ok(s), Ok(t)) => (s, t.unwrap_or_default()),
        (Err(e), _) | (_, Err(e)) => return usage_error(&e),
    };
    let catalogue = Catalogue::new(size);
    let generated = catalogue::generated();
    let bounds = Bounds {
        clock: Some(Duration::from_secs(60)),
        ..Bounds::default()
    };
    let src = Source::Text {
        text: &source,
        name: &program,
        base_dir: Path::new(&program).parent(),
    };
    let root = kernel::root(src, bounds);
    let checked = check::check(&check::Input {
        program: &source,
        name: &program,
        step,
        text: &input,
        catalogue: &catalogue,
        generated: &generated,
        step1: None,
        root: &root,
    });
    println!(
        "{program}: H from {} ({} candidates); the program states {}",
        checked.mechanism.writer.as_deref().unwrap_or("none"),
        checked.mechanism.candidates,
        if checked.mechanism.stated.is_empty() {
            "no mechanism".to_string()
        } else {
            checked.mechanism.stated.join(" and ")
        }
    );
    if let Some(r) = &checked.not_loaded {
        println!("not loaded: {r}");
    }
    for f in &checked.findings {
        println!(
            "{} ({}, from step {}): {}{}",
            f.kind.name(),
            f.kind.number(),
            f.from_step,
            f.message,
            f.form
                .as_deref()
                .map(|c| format!("\n    {}", c.lines().next().unwrap_or("")))
                .unwrap_or_default()
        );
    }
    println!("{} findings", checked.findings.len());
    ExitCode::SUCCESS
}

/// `einaf catalogue [--minimal] [--generated] [--grammar] [--tokens MODEL]`.
pub fn catalogue_cmd(args: &[String]) -> ExitCode {
    let mut size = Size::Whole;
    let (mut generated, mut grammar, mut tokens) = (false, false, None::<PathBuf>);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--minimal" => size = Size::Minimal,
            "--generated" => generated = true,
            "--grammar" => grammar = true,
            "--tokens" => match it.next() {
                Some(p) => tokens = Some(p.into()),
                None => return usage_error("--tokens takes the model's file"),
            },
            s => return usage_error(&format!("unknown argument {s}")),
        }
    }
    if generated {
        print!("{}", catalogue::generated().text());
        return ExitCode::SUCCESS;
    }
    let c = Catalogue::new(size);
    if grammar {
        for m in c.grammar().modules {
            println!(
                "{}: {} names; selected by {}; read, declared by none: {}; values: {}",
                m.module,
                m.symbols.len(),
                list(m.activators.iter().map(|(a, n)| format!("{a}/{n}"))),
                list(
                    m.unlisted
                        .iter()
                        .flat_map(|(a, ns)| ns.iter().map(move |n| format!("{a}/{n}")))
                ),
                list(m.values.iter().cloned())
            );
        }
        return ExitCode::SUCCESS;
    }
    let text = c.text();
    if let Some(model) = tokens {
        return count_tokens(&model, &text, size);
    }
    print!("{text}");
    ExitCode::SUCCESS
}

fn list(items: impl Iterator<Item = String>) -> String {
    let v: Vec<String> = items.collect();
    if v.is_empty() {
        "none".to_string()
    } else {
        v.join(" ")
    }
}

#[cfg(feature = "llama")]
fn count_tokens(model: &Path, text: &str, size: Size) -> ExitCode {
    match einaf::model::llama::Vocab::load(model) {
        Ok(v) => {
            println!(
                "the {} catalogue: {} bytes, {} tokens, {}",
                size.word(),
                text.len(),
                v.tokens(text).len(),
                einaf::model::sha256_hex(text.as_bytes())
            );
            for t in prompts::TEMPLATES {
                println!(
                    "{}: {} bytes, {} tokens, {}",
                    t.file,
                    t.text.len(),
                    v.tokens(t.text).len(),
                    t.sha256()
                );
            }
            for e in prompts::EXAMPLES {
                let p = e.program();
                println!(
                    "example {}: its program {} bytes, {} tokens",
                    e.id,
                    p.len(),
                    v.tokens(&p).len()
                );
            }
            ExitCode::SUCCESS
        }
        Err(f) => {
            eprintln!("error: {f}");
            ExitCode::from(1)
        }
    }
}

#[cfg(not(feature = "llama"))]
fn count_tokens(model: &Path, _: &str, _: Size) -> ExitCode {
    eprintln!(
        "error: {}: this einaf is built without the model's runtime; build it with `--features cuda` (or `llama`)",
        model.display()
    );
    ExitCode::from(2)
}
