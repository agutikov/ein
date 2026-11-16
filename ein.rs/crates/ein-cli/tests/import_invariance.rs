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
//! **And the work of a search, where a form leaves the universe's order as
//! it is.** Where the program as written answers `--single` by the
//! obligation tree, each form is asked `--single` too, and the verdict, the
//! saturations and the facts beyond root are compared. The tree orders its
//! branches by the structure and, among equals, by rendered text, and it
//! takes a branch's alternatives in the universe's order, which is rendered
//! text too; a form respells a name, and where that moves one candidate
//! past another the same answer is found by other work. The programs on
//! which it does are named ([`RESPELLED_PAST`]), each with why, and every
//! other program's work is the same in all three forms. The `slow` entries
//! are left to the answer's comparison: `--single` on them is held under a
//! permuted id space by `id_order_invariance`.
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
    summary_of(dir, file, &[])
}

/// What `ein solve FILE --single` says of its search, run in `dir`: the
/// route, the verdict and the work. `None` where the run wrote no summary.
fn single(dir: &Path, file: &str) -> Option<(String, String)> {
    let v = summary_of(dir, file, &["--single"]).ok()?;
    let route = v["region"]["route"].as_str().unwrap_or("").to_string();
    Some((
        route,
        format!(
            "{} · {} saturations · {} facts",
            v["verdict"]["word"], v["work"]["saturations"], v["work"]["facts"]
        ),
    ))
}

/// [`summary`], with `flags` after the file.
fn summary_of(dir: &Path, file: &str, flags: &[&str]) -> Result<serde_json::Value, i32> {
    let scratch = Scratch::new("ein-invariance-json");
    let json = scratch.path().join("s.json");
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .arg("solve")
        .arg(file)
        .args(flags)
        .arg("--json-summary")
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

/// **The programs whose tree does other work in another import form**, and
/// why. A branch's alternatives are taken in the universe's order, rendered
/// text, and the forms spell the names of an imported module otherwise.
///
/// - `binary_logic/synthesis.ein`: the hole's sixteen alternatives are the
///   functions of `std.bool.gates`, of which the file lists `Xor`, `And` and
///   `Or` short. As written those three sort before the thirteen spelled in
///   full, the two that fit are among them, and `--single` has its two
///   states after 4 saturations; with every name in full, or under an
///   alias, the sixteen are in one alphabet and it takes 16.
const RESPELLED_PAST: &[&str] = &["examples/binary_logic/synthesis.ein"];

/// One program: `Ok((compared forms, forms whose tree's work was compared,
/// forms whose tree's work is another))` or the first difference in an
/// answer.
fn check(path: &Path, slow: bool) -> Result<(usize, usize, usize), String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let dir = path.parent().expect("a directory");
    let file = path.file_name().expect("a name").to_str().expect("utf-8");
    let mut prefixes = module_prefixes(dir);
    let s = summary(dir, file);
    // The tree's work, where the program as written is answered by one.
    let tree = (!slow)
        .then(|| single(dir, file))
        .flatten()
        .filter(|(route, _)| route == "tree");
    let (mut compared, mut trees, mut moved) = (0, 0, 0);
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
        if let Some((_, as_written)) = &tree {
            match single(scratch.path(), file) {
                Some((route, in_form)) if route == "tree" && in_form == *as_written => trees += 1,
                // The same verdict by other work: counted, and held to the
                // list by the caller.
                Some((route, in_form))
                    if route == "tree"
                        && in_form.split(" · ").next() == as_written.split(" · ").next() =>
                {
                    trees += 1;
                    moved += 1;
                }
                other => {
                    return Err(format!(
                        "{}: `--single` is {as_written} as written and {:?} in form {mode}",
                        path.display(),
                        other
                    ));
                }
            }
        }
        compared += 1;
    }
    Ok((compared, trees, moved))
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
    let slow: std::collections::BTreeSet<PathBuf> = corpus()
        .entry
        .iter()
        .filter(|e| e.slow)
        .map(|e| root.join(&e.path))
        .collect();
    let results = ein_corpus::pool::map(
        "import_invariance",
        &todo,
        |p| {
            (
                p.strip_prefix(&root).unwrap_or(p).display().to_string(),
                String::new(),
            )
        },
        |p| check(p, slow.contains(p)),
    );
    let mut failures = Vec::new();
    let (mut compared, mut trees) = (0usize, 0usize);
    let mut respelled: Vec<String> = Vec::new();
    for (p, r) in todo.iter().zip(results) {
        match r {
            Ok((n, t, moved)) => {
                compared += n;
                trees += t;
                if moved > 0 {
                    respelled.push(p.strip_prefix(&root).unwrap_or(p).display().to_string());
                }
            }
            Err(e) => failures.push(e),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
    // The tree's work is the same in every form, but on the programs named.
    assert_eq!(
        respelled, RESPELLED_PAST,
        "the programs whose `--single` does other work in another import form"
    );
    // Not vacuous: the corpus's importing programs, every one in two forms,
    // and trees among them.
    assert!(total >= 250, "{total} programs that import");
    assert_eq!(compared, 2 * total);
    assert!(trees >= 40, "{trees} forms whose tree's work was compared");
}
