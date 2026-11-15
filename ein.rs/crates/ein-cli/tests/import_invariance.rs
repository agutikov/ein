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
//!   every name the program wrote short written in full, `std.math.decimal-arith.add`
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

mod import_forms;

use std::path::{Path, PathBuf};
use std::process::Command;

use ein_corpus::{Scratch, corpus, repo_root};
use ein_ir::{Ast, parse};
use import_forms::{beside_its_modules, form, module_prefixes, strip};

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

/// R, the queries and the claims of a summary, names normalized.
fn signature(v: &serde_json::Value, prefixes: &[String]) -> String {
    let prefixes: Vec<Vec<char>> = prefixes.iter().map(|p| p.chars().collect()).collect();
    let n = |s: &str| strip(s, &prefixes);
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
        let scratch = beside_its_modules(path, &written);
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
    // On the pool (M1f S1f.6.39), one unit per program, read back in the
    // manifest's order: until S1f.6.39 this test ran its own sixteen threads
    // and listed its failures in the order they finished.
    let todo = programs();
    let total = todo.len();
    let root = repo_root();
    let results = ein_corpus::pool::map(
        "import_invariance",
        &todo,
        |p| {
            (
                p.strip_prefix(&root).unwrap_or(p).display().to_string(),
                String::new(),
            )
        },
        |p| check(p),
    );
    let mut failures = Vec::new();
    let mut compared = 0usize;
    for r in results {
        match r {
            Ok(n) => compared += n,
            Err(e) => failures.push(e),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
    // Not vacuous: the corpus's importing programs, every one in two forms.
    assert!(total >= 250, "{total} programs that import");
    assert_eq!(compared, 2 * total);
}
