//! Every `.ein` file a state dump writes loads, and loads back to the facts it
//! was written from.
//!
//! `--dump-states` and `--dump-lattice` write root before and after each
//! layer, and each entering's state, through [`kb_to_ein_text`]. Until
//! M1e2 S1e2.1.7 none of them loaded: a typed `(relation R T…)` is stored with
//! its arity-1 companion `(relation R)`, the writer wrote both, and the loader
//! read the second as a duplicate declaration. Now the companion is left to the
//! loader, which adds it on reading the declaration.
//!
//! The claim checked is the round trip, not only the load: a snapshot, loaded
//! and written again, holds the same facts. **Not** the same provenance: the
//! writer spells an origin `:rule "name"` and `:hypothesis N` for the reader,
//! and the loader takes `:rule` only as an atom and has no `:hypothesis`, so a
//! reloaded derived fact reads back as given. The fixtures between them carry
//! rule and hypothesis provenance, kernel-written facts (`symmetric`), stored
//! negatives, a forced positive, the verdict atom `open`, and the stdlib.

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::solve::{Dumper, OnBudget, SolveOptions, solve};
use ein_ir::parse;
use ein_render::dump::{LatticeDumper, MonotonicDumper, kb_to_ein_text};
use std::path::{Path, PathBuf};

const FIXTURES: &[&str] = &[
    "examples/lattice/01_subset_pruned.ein",
    "examples/lattice/02_genuine_3set_death.ein",
    "examples/features/01_not_and_absent.ein",
    "examples/features/06_symmetric_native.ein",
    "examples/ein-bugs/wrong-answers/forced-2.ein",
    "tests/stdlib/closure/03_closed_and_owing.ein",
    "examples/domain_elim/ab.ein",
    "tests/stdlib/slots/03_fill.ein",
];

/// Parse and load `path` as `ein saturate` would, or say why not.
fn load(path: &Path) -> Result<(ein_core::Kb, Terms), String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut ast = ein_ir::Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).map_err(|e| e.to_string())?;
    let kb =
        ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).map_err(|e| e.to_string())?;
    Ok((kb, terms))
}

/// Solve `rel` with `dumper` writing into `out`.
fn solve_into(rel: &str, dumper: &mut dyn Dumper) {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).expect("the fixture is checked in");
    let mut ast = ein_ir::Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("the fixture parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).expect("it loads");
    let opts = SolveOptions {
        stop_after: None,
        max_set_size: 3,
        max_enterings: Some(60),
        on_budget: OnBudget::Verdict,
        store_lattice: true,
        ..SolveOptions::default()
    };
    let mut events = ein_infer::events::Events::off();
    solve(&mut kb, &mut terms, &ast, &mut events, dumper, &opts).expect("it solves");
}

fn ein_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            ein_files(&p, out);
        } else if p.extension().is_some_and(|e| e == "ein") {
            out.push(p);
        }
    }
}

/// The facts of a dumped KB, one line each, sorted, with the provenance
/// keywords taken out — the writer puts un-annotated facts first, a load does
/// not promise to keep that order, and provenance does not survive a load.
fn forms(text: &str) -> Vec<String> {
    let mut v: Vec<String> = text
        .split("\n\n")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(fact_only)
        .collect();
    v.sort();
    v
}

/// One form on one line, without `:rule …`, `:source …` or `:hypothesis …`.
fn fact_only(form: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut tokens = form.split_whitespace();
    while let Some(t) = tokens.next() {
        if !matches!(t, ":rule" | ":source" | ":hypothesis") {
            out.push(t.to_string());
            continue;
        }
        // Skip the value — a quoted string may span tokens — but keep the
        // closing parens that ride on its last token.
        let mut value = tokens.next().unwrap_or("");
        if value.starts_with('"') {
            while value.trim_end_matches(')').len() < 2
                || !value.trim_end_matches(')').ends_with('"')
            {
                match tokens.next() {
                    Some(next) => value = next,
                    None => break,
                }
            }
        }
        let parens = &value[value.trim_end_matches(')').len()..];
        if let Some(last) = out.last_mut() {
            last.push_str(parens);
        }
    }
    out.join(" ")
}

#[test]
fn every_dumped_state_loads_back_to_its_own_facts() {
    let mut checked = 0;
    let mut failures = Vec::new();
    for rel in FIXTURES {
        let stem = Path::new(rel)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let tmp = std::env::temp_dir().join(format!("ein-dump-states-load-{stem}"));
        let _ = std::fs::remove_dir_all(&tmp);
        let states = tmp.join("states");
        let lattice = tmp.join("lattice");
        solve_into(
            rel,
            &mut MonotonicDumper::new(Some(&states)).expect("a dumper"),
        );
        solve_into(
            rel,
            &mut LatticeDumper::new(Some(&lattice)).expect("a dumper"),
        );
        let mut files = Vec::new();
        ein_files(&tmp, &mut files);
        files.sort();
        assert!(
            files.iter().any(|f| f.starts_with(&states))
                && files.iter().any(|f| f.starts_with(&lattice)),
            "{rel}: both dumpers should have written snapshots"
        );
        for file in &files {
            checked += 1;
            let shown = file
                .strip_prefix(&tmp)
                .unwrap_or(file)
                .display()
                .to_string();
            let written = std::fs::read_to_string(file).expect("readable");
            match load(file) {
                Err(e) => failures.push(format!("{rel}: {shown} does not load: {e}")),
                Ok((kb, terms)) => {
                    let (again, before) = (forms(&kb_to_ein_text(&kb, &terms)), forms(&written));
                    if again != before {
                        let lost: Vec<&String> =
                            before.iter().filter(|f| !again.contains(f)).collect();
                        let gained: Vec<&String> =
                            again.iter().filter(|f| !before.contains(f)).collect();
                        failures.push(format!(
                            "{rel}: {shown} loads to different facts — lost {lost:?}, gained {gained:?}"
                        ));
                    }
                }
            }
        }
        let _ = std::fs::remove_dir_all(&tmp);
    }
    assert!(
        failures.is_empty(),
        "{} of {checked}:\n{}",
        failures.len(),
        failures.join("\n")
    );
    assert!(checked > 0);
}
