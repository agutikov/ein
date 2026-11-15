//! **A program in its three import forms**, and the names they differ by.
//!
//! - **S**, the program as written, its imports selective, `:symbols (…)`;
//! - **W**, every selective import of the program whole, `(import M)`, and
//!   every name the program wrote short written in full
//!   (`ein_ir::Resolver::short_names` says which);
//! - **A**, every selective import `(import M :as impK)`, and every such name
//!   written `impK.name`.
//!
//! [`form`] writes W or A from S, [`beside_its_modules`] puts one where its
//! modules are found, and [`strip`] takes the module prefixes and the aliases
//! off a text, which is how two forms' outputs are compared.
//!
//! Shared by `#[path]`, since a test binary's functions are no crate's:
//! `import_invariance.rs` holds every corpus program's answer to it through
//! the binary, and `ein-infer/tests/structure_invariance.rs` the structure
//! of the hypothesis set in process.

use std::collections::BTreeSet;
use std::path::Path;

use ein_corpus::{Scratch, repo_root};
use ein_ir::{Ast, Node, parse};

/// A name with every prefix of `prefixes` taken off where it starts a name.
///
/// The prefixes come as characters, split once by the caller: split again
/// at every boundary of every string, they were most of what a sweep over a
/// long output cost.
pub fn strip(s: &str, prefixes: &[Vec<char>]) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes: Vec<char> = s.chars().collect();
    let mut i = 0;
    let boundary = |c: char| c == '(' || c == ')' || c == ' ' || c == '"' || c == '\n';
    while i < bytes.len() {
        let at_start = i == 0 || boundary(bytes[i - 1]);
        if at_start
            && let Some(pc) = prefixes.iter().find(|pc| {
                bytes.len() > i + pc.len()
                    && bytes[i..i + pc.len()] == pc[..]
                    && bytes[i + pc.len()] == '.'
            })
        {
            i += pc.len() + 1;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    out
}

/// The program's text in form W (`alias` none) or A: its selective imports
/// rewritten, every name it binds through them rewritten to match, its other
/// forms as the canonical dumper writes them.
pub fn form(path: &Path, text: &str, mode: char) -> Option<(String, Vec<String>)> {
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
pub fn module_prefixes(dir: &Path) -> Vec<String> {
    let stdlib = repo_root().join("stdlib");
    // Every module at any depth, a dot a directory: `std.math.decimal-arith`.
    let mut out: Vec<String> = ein_corpus::ein_files_under(&stdlib)
        .into_iter()
        .filter_map(|p| {
            let rel = p.strip_prefix(&stdlib).ok()?.with_extension("");
            let parts: Vec<String> = rel
                .components()
                .map(|c| c.as_os_str().to_string_lossy().to_string())
                .collect();
            Some(format!("std.{}", parts.join(".")))
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

/// `written`, a form of the program at `path`, in a scratch directory under
/// the program's own file name, beside links to the program's directory's
/// other modules: a module is found from the program file's directory.
pub fn beside_its_modules(path: &Path, written: &str) -> Scratch {
    let dir = path.parent().expect("a directory");
    let scratch = Scratch::new("ein-invariance");
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        if p.extension().is_some_and(|x| x == "ein") && p != path {
            #[cfg(unix)]
            std::os::unix::fs::symlink(&p, scratch.path().join(e.file_name())).expect("a link");
        }
    }
    let file = path.file_name().expect("a name");
    std::fs::write(scratch.path().join(file), written).expect("the form");
    scratch
}
