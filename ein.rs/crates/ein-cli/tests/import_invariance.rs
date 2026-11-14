//! **The answer does not depend on how a program imports** — M1f S1f.6.35's
//! acceptance, the user's criterion of 2026-11-14: *"the answer MUST be
//! invarian of import type - write 2 same programs with different imports and
//! check that both have equivalent solution (up to names prefixes). If they
//! move - then import works not correclty"*.
//!
//! Every corpus program that imports, in three forms, each solved by the `ein`
//! binary with `--json-summary`:
//!
//! - **S**, the program as written, its imports selective, `:symbols (…)`;
//! - **W**, every selective import of the program whole, `(import M)`, and
//!   every name the program wrote short written in full, `std.arith.add`
//!   (`ein_ir::Resolver::short_names` says which);
//! - **A**, every selective import `(import M :as impK)`, and every such name
//!   written `impK.add`.
//!
//! R — the states as sets of facts, `k_s`, `k_c`, root's verdict and debts,
//! whether the run exhausted — every query's classes, and every claim's
//! outcome, are compared with every module prefix and alias taken off. Since
//! the user's ruling of that day (*"in all cases we import everything, the
//! difference only in names"*) the three are one program by construction:
//! this is the guard that keeps them so. A program S refuses is held to be
//! refused in W and A too.
//!
//! The forms are written into a scratch directory beside links to the
//! program's directory's modules, since a module is found from the program
//! file's directory. `EIN_CORPUS_SLOW=1` adds the slow entries, as
//! `corpus_cli.rs`'s sweep does.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use ein_corpus::{Scratch, corpus, repo_root};
use ein_ir::{Ast, Node, parse};

fn include_slow() -> bool {
    std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1")
}

/// The JSON summary of `ein solve FILE`, run in `dir`, or the exit code of a
/// run that wrote none.
fn summary(dir: &Path, file: &str) -> Result<serde_json::Value, i32> {
    let scratch = Scratch::new("ein-invariance-json");
    let json = scratch.path().join("s.json");
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(["solve", file, "--json-summary"])
        .arg(&json)
        .current_dir(dir)
        .output()
        .expect("the `ein` binary runs");
    let code = out.status.code().unwrap_or(-1);
    match std::fs::read_to_string(&json) {
        Ok(text) if code == 0 || code == 1 => serde_json::from_str(&text).map_err(|_| code),
        _ => Err(code),
    }
}

/// A name with every prefix of `prefixes` taken off where it starts a name.
fn strip(s: &str, prefixes: &[String]) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes: Vec<char> = s.chars().collect();
    let mut i = 0;
    let boundary = |c: char| c == '(' || c == ')' || c == ' ' || c == '"' || c == '\n';
    while i < bytes.len() {
        let at_start = i == 0 || boundary(bytes[i - 1]);
        if at_start
            && let Some(p) = prefixes.iter().find(|p| {
                let pc: Vec<char> = p.chars().collect();
                bytes.len() > i + pc.len()
                    && bytes[i..i + pc.len()] == pc[..]
                    && bytes[i + pc.len()] == '.'
            })
        {
            i += p.chars().count() + 1;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    out
}

/// R, the queries and the claims of a summary, names normalized.
fn signature(v: &serde_json::Value, prefixes: &[String]) -> String {
    let n = |s: &str| strip(s, prefixes);
    let strs = |v: &serde_json::Value| -> Vec<String> {
        let mut out: Vec<String> = v
            .as_array()
            .map(|a| a.iter().map(|x| n(&x.to_string())).collect())
            .unwrap_or_default();
        out.sort();
        out
    };
    let g = &v["region"];
    let mut states: Vec<Vec<String>> = g["states"]
        .as_array()
        .map(|a| a.iter().map(|s| strs(&s["facts"])).collect())
        .unwrap_or_default();
    states.sort();
    let mut queries = Vec::new();
    for q in v["queries"].as_array().into_iter().flatten() {
        let mut classes: Vec<(Vec<String>, Vec<String>)> = q["classes"]
            .as_array()
            .map(|a| {
                a.iter()
                    .map(|c| (strs(&c["facts"]), strs(&c["rows"])))
                    .collect()
            })
            .unwrap_or_default();
        classes.sort();
        queries.push(format!(
            "{} {:?} {}",
            n(&q["goal"].to_string()),
            classes,
            q["expect"]["outcome"]
        ));
    }
    format!(
        "k_s {} k_c {} exhausted {} root {} owes {} owed {:?} states {:?} queries {:?} claim {}",
        g["k_s"],
        g["k_c"],
        g["exhausted"],
        g["root"],
        g["root_owes"],
        strs(&g["root_owed"]),
        states,
        queries,
        v["claim"]["outcome"]
    )
}

/// The program's text in form W (`alias` none) or A: its selective imports
/// rewritten, every name it binds through them rewritten to match, its other
/// forms as the canonical dumper writes them.
fn form(path: &Path, text: &str, mode: char) -> Option<(String, Vec<String>)> {
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, path.to_str()).ok()?;
    let names = ein_ir::Resolver::new()
        .short_names(&mut ast, &forms, path.parent())
        .ok()?;
    let mut imports: Vec<String> = Vec::new();
    let mut aliases: Vec<(String, String)> = Vec::new();
    let mut rest = Vec::new();
    for &f in &forms {
        if ast.head_name(f) != Some("import") {
            rest.push(f);
            continue;
        }
        let args = ast.form_args(f).to_vec();
        let module = ast.atom_name(args[0])?.to_string();
        let selective = args.iter().any(|&a| {
            matches!(ast.node(a), Node::KwPair { key, .. }
            if matches!(ast.node(key), Node::Keyword(s) if ast.sym(s) == "symbols"))
        });
        let line = if !selective {
            ein_ir::dump::dump_compact(&ast, f)
        } else if mode == 'W' {
            format!("(import {module})")
        } else {
            let alias = match aliases.iter().find(|(m, _)| *m == module) {
                Some((_, a)) => a.clone(),
                None => {
                    let a = format!("imp{}", aliases.len());
                    aliases.push((module.clone(), a.clone()));
                    a
                }
            };
            format!("(import {module} :as {alias})")
        };
        if !imports.contains(&line) {
            imports.push(line);
        }
    }
    // How the form writes each bound short name.
    let spell = |short: &str| -> Option<String> {
        let full = names.get(short)?;
        if full == short {
            return None;
        }
        let (owner, rest) = full.rsplit_once('.')?;
        Some(match aliases.iter().find(|(m, _)| m == owner) {
            Some((_, a)) if mode == 'A' => format!("{a}.{rest}"),
            _ => full.clone(),
        })
    };
    let body = ein_ir::dump_canonical(&ast, &rest);
    let mut out = imports.join("\n");
    out.push('\n');
    // Rename name tokens outside strings.
    let mut chars = body.chars().peekable();
    let mut tok = String::new();
    let flush = |tok: &mut String, out: &mut String| {
        if !tok.is_empty() {
            match spell(tok) {
                Some(s) if !tok.starts_with('?') && !tok.starts_with(':') => out.push_str(&s),
                _ => out.push_str(tok),
            }
            tok.clear();
        }
    };
    while let Some(c) = chars.next() {
        if c == '"' {
            flush(&mut tok, &mut out);
            out.push(c);
            while let Some(d) = chars.next() {
                out.push(d);
                if d == '\\' {
                    if let Some(e) = chars.next() {
                        out.push(e);
                    }
                } else if d == '"' {
                    break;
                }
            }
        } else if c == ';' && tok.is_empty() {
            out.push(c);
            for d in chars.by_ref() {
                out.push(d);
                if d == '\n' {
                    break;
                }
            }
        } else if c.is_whitespace() || c == '(' || c == ')' {
            flush(&mut tok, &mut out);
            out.push(c);
        } else {
            tok.push(c);
        }
    }
    flush(&mut tok, &mut out);
    let mut prefixes: Vec<String> = names
        .values()
        .filter_map(|f| f.rsplit_once('.').map(|(m, _)| m.to_string()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    prefixes.extend(aliases.into_iter().map(|(_, a)| a));
    Some((out, prefixes))
}

/// Every module name a program's text and its modules' may print: the
/// stdlib's and the program's directory's, longest first.
fn module_prefixes(dir: &Path) -> Vec<String> {
    let mut out: Vec<String> = std::fs::read_dir(repo_root().join("stdlib"))
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            if p.extension()? != "ein" {
                return None;
            }
            Some(format!("std.{}", p.file_stem()?.to_str()?))
        })
        .chain(
            std::fs::read_dir(dir)
                .into_iter()
                .flatten()
                .flatten()
                .filter_map(|e| {
                    let p = e.path();
                    (p.extension()? == "ein")
                        .then(|| p.file_stem()?.to_str().map(str::to_string))?
                }),
        )
        .collect();
    out.sort_by_key(|p| std::cmp::Reverse(p.len()));
    out
}

/// One program: `Ok(compared forms)` or the first difference.
fn check(path: &Path) -> Result<usize, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let dir = path.parent().expect("a directory");
    let file = path.file_name().expect("a name").to_str().expect("utf-8");
    let mut prefixes = module_prefixes(dir);
    let s = summary(dir, file);
    let mut compared = 0;
    for mode in ['W', 'A'] {
        let Some((written, extra)) = form(path, &text, mode) else {
            // Its imports refused: S is refused too, and there is no form.
            if s.is_err() {
                compared += 1;
                continue;
            }
            return Err(format!(
                "{}: form {mode} could not be written",
                path.display()
            ));
        };
        for p in extra {
            if !prefixes.contains(&p) {
                prefixes.push(p);
            }
        }
        prefixes.sort_by_key(|p| std::cmp::Reverse(p.len()));
        // Beside links to the program's directory's modules.
        let scratch = Scratch::new("ein-invariance");
        for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let p = e.path();
            if p.extension().is_some_and(|x| x == "ein") && p != path {
                #[cfg(unix)]
                std::os::unix::fs::symlink(&p, scratch.path().join(e.file_name())).expect("a link");
            }
        }
        std::fs::write(scratch.path().join(file), &written).expect("the form");
        let f = summary(scratch.path(), file);
        match (&s, &f) {
            (Ok(a), Ok(b)) => {
                let (a, b) = (signature(a, &prefixes), signature(b, &prefixes));
                if a != b {
                    return Err(format!(
                        "{}: S and {mode} differ\n  S: {a}\n  {mode}: {b}\n--- form {mode} ---\n{written}",
                        path.display()
                    ));
                }
            }
            (Err(a), Err(b)) if a == b => {}
            _ => {
                return Err(format!(
                    "{}: S {:?} and {mode} {:?} end differently",
                    path.display(),
                    s.as_ref().map(|_| 0),
                    f.as_ref().map(|_| 0)
                ));
            }
        }
        compared += 1;
    }
    Ok(compared)
}

/// The corpus programs that import, in manifest order.
fn programs() -> Vec<PathBuf> {
    let root = repo_root();
    corpus()
        .select(&[], None, include_slow())
        .into_iter()
        .map(|e| root.join(&e.path))
        .filter(|p| {
            let Ok(text) = std::fs::read_to_string(p) else {
                return false;
            };
            let mut ast = Ast::new();
            parse(&mut ast, &text, p.to_str())
                .map(|forms| forms.iter().any(|&f| ast.head_name(f) == Some("import")))
                .unwrap_or(false)
        })
        .collect()
}

#[test]
fn every_corpus_program_answers_alike_under_every_import_form() {
    let todo = Mutex::new(programs());
    let total = todo.lock().expect("lock").len();
    let failures = Mutex::new(Vec::new());
    let compared = Mutex::new(0usize);
    let threads = std::thread::available_parallelism()
        .map_or(4, |n| n.get())
        .min(16);
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                loop {
                    let Some(path) = todo.lock().expect("lock").pop() else {
                        return;
                    };
                    match check(&path) {
                        Ok(n) => *compared.lock().expect("lock") += n,
                        Err(e) => failures.lock().expect("lock").push(e),
                    }
                }
            });
        }
    });
    let failures = failures.into_inner().expect("lock");
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
    // Not vacuous: the corpus's importing programs, every one in two forms.
    assert!(total >= 250, "{total} programs that import");
    assert_eq!(compared.into_inner().expect("lock"), 2 * total);
}
