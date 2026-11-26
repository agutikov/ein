//! `einaf grammar`: the grammars' subcommands. `write` writes the whole
//! grammar and its manifest (`--check` writes nothing and says whether the
//! files are what it would write); `accept` runs EinAf's acceptor on files;
//! `sample` writes texts a grammar allows; `render` writes the whole
//! grammar's root `root` with the rules it reaches, or a step's grammar for a
//! program, or the grammar a request carries; `catalogue`
//! lists the catalogue the steps' grammars are rendered on; and, with the
//! `llama` feature, `llama` runs files through llama.cpp's own grammar
//! sampler, token by token, and `--json-schema` the GBNF llama.cpp writes
//! for a JSON schema.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use einaf::grammar::gbnf::{Compiled, Rng, Verdict};
use einaf::grammar::{MANIFEST_FILE, WHOLE_FILE, steps, whole};

use crate::usage_error;

/// The package's root, where `grammars/` is.
fn package_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn grammar(args: &[String]) -> ExitCode {
    match args.first().map(String::as_str) {
        Some("write") => write(&args[1..]),
        Some("accept") => accept(&args[1..]),
        Some("sample") => sample(&args[1..]),
        Some("render") => render(&args[1..]),
        Some("catalogue") => catalogue(),
        Some("llama") => crate::model_cmd::grammar_llama(&args[1..]),
        Some("--json-schema") => crate::model_cmd::grammar(args),
        _ => usage_error(
            "grammar takes write, accept, sample, render, catalogue, llama or --json-schema",
        ),
    }
}

/// `einaf grammar write [--check] [--dir DIR]`.
fn write(args: &[String]) -> ExitCode {
    let mut check = false;
    let mut dir = package_dir();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--check" => check = true,
            "--dir" => match it.next() {
                Some(d) => dir = PathBuf::from(d),
                None => return usage_error("--dir takes the package's directory"),
            },
            s => return usage_error(&format!("unknown argument {s}")),
        }
    }
    let text = einaf::grammar::whole_text();
    let manifest = einaf::grammar::manifest_text(&text);
    let files = [
        (dir.join(WHOLE_FILE), text),
        (dir.join(MANIFEST_FILE), manifest),
    ];
    let mut stale = Vec::new();
    for (path, want) in &files {
        let have = std::fs::read_to_string(path).unwrap_or_default();
        if &have != want {
            stale.push(path.display().to_string());
            if !check {
                if let Some(parent) = path.parent()
                    && let Err(e) = std::fs::create_dir_all(parent)
                {
                    eprintln!("error: {}: {e}", parent.display());
                    return ExitCode::from(1);
                }
                if let Err(e) = std::fs::write(path, want) {
                    eprintln!("error: {}: {e}", path.display());
                    return ExitCode::from(1);
                }
            }
        }
    }
    match (check, stale.is_empty()) {
        (_, true) => {
            println!(
                "einaf grammar: {} and {} as written",
                WHOLE_FILE, MANIFEST_FILE
            );
            ExitCode::SUCCESS
        }
        (true, false) => {
            eprintln!(
                "einaf grammar: not what `einaf grammar write` writes: {}",
                stale.join(", ")
            );
            ExitCode::from(1)
        }
        (false, false) => {
            println!("einaf grammar: wrote {}", stale.join(", "));
            ExitCode::SUCCESS
        }
    }
}

/// The grammar a command reads: `--grammar FILE`, or the whole grammar.
fn read_grammar(path: &Option<String>) -> Result<String, String> {
    match path {
        Some(p) => std::fs::read_to_string(p).map_err(|e| format!("{p}: {e}")),
        None => Ok(einaf::grammar::whole_text()),
    }
}

/// `einaf grammar accept [--grammar FILE] [--root NAME] FILE...`: each file
/// accepted, or refused at a line and a column. Exits 0 where every file is
/// accepted, 1 where one is refused.
fn accept(args: &[String]) -> ExitCode {
    let (mut grammar, mut root, mut files) = (None, whole::PROGRAM.to_string(), Vec::new());
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--grammar" => grammar = it.next().cloned(),
            "--root" => match it.next() {
                Some(r) => root = r.clone(),
                None => return usage_error("--root takes a rule's name"),
            },
            s if s.starts_with('-') => return usage_error(&format!("unknown option {s}")),
            s => files.push(s.to_string()),
        }
    }
    if files.is_empty() {
        return usage_error("accept takes a FILE");
    }
    let compiled = match read_grammar(&grammar).and_then(|t| Compiled::parse(&t)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: the grammar: {e}");
            return ExitCode::from(1);
        }
    };
    let mut refused = 0;
    for f in &files {
        let text = match std::fs::read_to_string(f) {
            Ok(t) => t,
            Err(e) => {
                println!("{f}: unreadable: {e}");
                refused += 1;
                continue;
            }
        };
        match compiled.accepts(&root, &text) {
            Ok(Verdict::Accepted) => println!("{f}: accepted"),
            Ok(Verdict::Refused { at, line, column }) => {
                refused += 1;
                let what = text[at..]
                    .chars()
                    .next()
                    .map_or("the end".to_string(), |c| format!("{c:?}"));
                println!("{f}: refused at {line}:{column}, {what}");
            }
            Err(e) => {
                refused += 1;
                println!("{f}: {e}");
            }
        }
    }
    if refused == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

/// `einaf grammar sample [--grammar FILE] [--root NAME] [--seed N]
/// [--count N] [--depth N] [--size N]`: texts the grammar allows, each
/// followed by a line `;; ----`.
fn sample(args: &[String]) -> ExitCode {
    let (mut grammar, mut root) = (None, whole::PROGRAM.to_string());
    let (mut seed, mut count, mut depth, mut size) = (1u64, 1usize, 48u32, 2_000usize);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut num = |what: &str| -> Result<u64, String> {
            it.next()
                .and_then(|v| v.parse().ok())
                .ok_or_else(|| format!("{what} takes a number"))
        };
        let r = match a.as_str() {
            "--grammar" => {
                grammar = it.next().cloned();
                Ok(())
            }
            "--root" => match it.next() {
                Some(x) => {
                    root = x.clone();
                    Ok(())
                }
                None => Err("--root takes a rule's name".to_string()),
            },
            "--seed" => num("--seed").map(|n| seed = n),
            "--count" => num("--count").map(|n| count = n as usize),
            "--depth" => num("--depth").map(|n| depth = n as u32),
            "--size" => num("--size").map(|n| size = n as usize),
            s => Err(format!("unknown argument {s}")),
        };
        if let Err(e) = r {
            return usage_error(&e);
        }
    }
    let compiled = match read_grammar(&grammar).and_then(|t| Compiled::parse(&t)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: the grammar: {e}");
            return ExitCode::from(1);
        }
    };
    let mut rng = Rng::new(seed);
    for _ in 0..count {
        match compiled.sample(&root, &mut rng, depth, size, einaf::grammar::FAVOURED) {
            Ok(t) => println!("{t}\n;; ----"),
            Err(e) => {
                eprintln!("error: {e}");
                return ExitCode::from(1);
            }
        }
    }
    ExitCode::SUCCESS
}

/// `einaf grammar catalogue`: the catalogue the step grammars are rendered
/// on, read off the library: a line a module, with its names, its
/// activators at their counts, and the relations it reads that no module
/// declares.
fn catalogue() -> ExitCode {
    let c = steps::Catalogue::library();
    for m in &c.modules {
        let acts: Vec<String> = m
            .activators
            .iter()
            .map(|(a, n)| format!("{a}/{n}"))
            .collect();
        let un: Vec<String> = m
            .unlisted
            .iter()
            .map(|(u, c)| {
                format!(
                    "{u}/{}",
                    c.iter()
                        .map(|k| k.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
            })
            .collect();
        println!(
            "{}: {} names; activators {}; read, declared by none: {}",
            m.module,
            m.symbols.len(),
            if acts.is_empty() {
                "none".to_string()
            } else {
                acts.join(" ")
            },
            if un.is_empty() {
                "none".to_string()
            } else {
                un.join(" ")
            }
        );
    }
    ExitCode::SUCCESS
}

/// `einaf grammar render GRAMMAR [--program FILE] [--text FILE] [--fence]
/// [--after-thinking] [--request]`: the whole grammar's root `root` and the
/// rules it reaches (`whole`), or a step's
/// (`1`, `2`, `2b`, `3`) rendered for the program of the steps before it and
/// the input's sentences, in its frame (inside a fence, after the thinking);
/// with `--request`, the grammar as a request carries it, as JSON.
fn render(args: &[String]) -> ExitCode {
    let mut n = None;
    let (mut program, mut text) = (None, None);
    let (mut after, mut request, mut fence) = (false, false, false);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--program" => program = it.next().cloned(),
            "--text" => text = it.next().cloned(),
            "--after-thinking" => after = true,
            "--fence" => fence = true,
            "--request" => request = true,
            s if n.is_none() && !s.starts_with('-') => n = Some(s.to_string()),
            s => return usage_error(&format!("unknown argument {s}")),
        }
    }
    let read = |p: &Option<String>| -> Result<Option<String>, String> {
        p.as_ref()
            .map(|p| std::fs::read_to_string(p).map_err(|e| format!("{p}: {e}")))
            .transpose()
    };
    let (program_text, input) = match (read(&program), read(&text)) {
        (Ok(a), Ok(b)) => (a, b),
        (Err(e), _) | (_, Err(e)) => {
            eprintln!("error: {e}");
            return ExitCode::from(1);
        }
    };
    let (gbnf, root) = match n.as_deref() {
        Some("whole") => (einaf::grammar::root_text(), whole::ROOT.to_string()),
        Some(s) => {
            let Some(step) = steps::Step::parse(s) else {
                return usage_error("render takes whole, 1, 2, 2b or 3");
            };
            let base = program.as_deref().and_then(|p| Path::new(p).parent());
            match steps::render_for(step, program_text.as_deref(), input.as_deref(), base) {
                Ok(g) => (g.text, g.root),
                Err(e) => {
                    eprintln!("error: {e}");
                    return ExitCode::from(1);
                }
            }
        }
        None => return usage_error("render takes whole, 1, 2, 2b or 3"),
    };
    let frame = einaf::grammar::Frame {
        after_thinking: after,
        fence,
    };
    let g = einaf::grammar::request_grammar(&gbnf, &root, frame);
    if request {
        println!("{}", serde_json::to_string(&g).expect("a grammar is JSON"));
    } else {
        print!("{}", g.text);
    }
    ExitCode::SUCCESS
}
