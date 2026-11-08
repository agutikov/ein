//! Every register row's `[case.region]` is its witness's R — M1e2
//! [S1e2.1a.10](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.10_the_register_under_r.md)
//! T1e2.1a.10.3.
//!
//! `corpus/wrong_answers.toml` states each witness's region beside the walk's
//! answers, so that the switch (S1e2.1a.5) flips a row against a right answer
//! that was already checked rather than written the day before. This test
//! recomputes it with [`explore`], the code `ein solve` runs (route 1 on a
//! monotone program, the cube on a flagged one), and holds every field to it:
//! the route, the static verdict, k_s, k_c, the kind of an empty R, and both
//! lines as commitments. A row whose `now` is a refusal (M1e2 S1e2.2.5) states
//! no region, and the library refuses its witness too.
//!
//! ```text
//! cargo test -p ein-infer --test register_region
//! EIN_BLESS=1 cargo test -p ein-infer --test register_region   # re-bank the blocks
//! ```
//!
//! **Since M1e2 S1e2.2.25 a region can be a stopped description** (schema
//! /10): a row whose witness is past the oracle's limit (W8's rows on
//! `square-unique/corner-house.ein`) states the lower bounds the description
//! reached, the minimal line members it certified, no maximal line, and why
//! it stopped, and this test holds those.
//!
//! Both builds hold every row. A debug build skipped W7's witnesses, which
//! it aborted on, until M1e2 S1e2.2.7 fixed W7, and W16's order pair until
//! M1e2 S1e2.2.16 fixed W16.

use std::path::Path;

use ein_core::{Kb, Terms};
use ein_corpus::register::{REGISTER, Region};
use ein_corpus::repo_root;
use ein_infer::explore::{self, Answer, Ask, Mode, Route, Stopped, Verdict, Which};
use ein_ir::{Ast, load, parse};

fn loaded(path: &Path) -> (Kb, Terms, Ast) {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str())
        .unwrap_or_else(|e| panic!("{}: {e:?}", path.display()));
    let kb = load(&mut ast, &mut terms, &forms, path.parent())
        .unwrap_or_else(|e| panic!("{}: {e:?}", path.display()));
    (kb, terms, ast)
}

/// The refusal of a witness the parser or the loader refuses, as the binary's
/// first line on stderr, or `None` where it loads: a refusal at load (M1e2
/// S1e2.2.20, W20's wildcard relation), which the binary prints as `kb load
/// error: …`, and since M1f S1f.5.3 one at the parse (W20's range, until
/// S1f.5.4), `<witness>:<line>:<col>: unexpected input`. The library cannot
/// get past either.
fn refusal(witness: &str) -> Option<String> {
    let path = repo_root().join(witness);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{witness}: {e}"));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = match parse(&mut ast, &text, Some(witness)) {
        Ok(forms) => forms,
        Err(e) => return Some(e.to_string().lines().next().unwrap_or("").to_string()),
    };
    load(&mut ast, &mut terms, &forms, path.parent())
        .err()
        .map(|e| format!("kb load error: {}", e.0))
}

fn ask(path: &Path, mode: Mode) -> Answer {
    try_ask(path, mode).unwrap_or_else(|e| panic!("{}: {e:?}", path.display()))
}

fn try_ask(path: &Path, mode: Mode) -> Result<Answer, ein_infer::SolveError> {
    let (mut kb, mut terms, ast) = loaded(path);
    let mut opts = explore::Options::new(mode);
    opts.jobs = 4;
    explore::run(&mut kb, &mut terms, &ast, &opts)
}

/// The region as the library computes it, in the register's spelling.
fn computed(path: &Path) -> Result<Region, String> {
    let a = ask(
        path,
        Mode {
            which: Which::All,
            ask: Ask::Describe,
        },
    );
    if !a.exhausted {
        return stopped(&a);
    }
    let (Some(shape), Some(counts)) = (&a.shape, &a.counts) else {
        return Err("an exhausted description with no shape".to_string());
    };
    let line = |sets: &[Box<[u32]>]| -> Vec<Vec<String>> {
        let mut out: Vec<Vec<String>> = sets
            .iter()
            .map(|s| {
                let mut v: Vec<String> = s
                    .iter()
                    .map(|&b| a.candidates[b as usize].text.clone())
                    .collect();
                v.sort();
                v
            })
            .collect();
        out.sort();
        out
    };
    let empty = (counts.k_s == 0).then(|| {
        let single = ask(
            path,
            Mode {
                which: Which::All,
                ask: Ask::Single,
            },
        );
        match single.verdict {
            Some(Verdict::Open) => "all-owe".to_string(),
            _ => "no-alive".to_string(),
        }
    });
    Ok(Region {
        route: route_word(a.route),
        monotone: a.monotonicity.is_monotone(),
        k_s: counts.k_s,
        k_c: counts.k_c,
        empty,
        minimal: line(&shape.minimal),
        maximal: line(&shape.maximal),
        stopped: None,
    })
}

fn route_word(r: Route) -> String {
    match r {
        Route::DownSet => "route 1",
        Route::Cube => "route 2",
        Route::States => "state route",
        Route::Tree => "obligation tree",
    }
    .to_string()
}

/// A description that stopped before R was exhausted (schema /10, M1e2
/// S1e2.2.25: W8's rows past the oracle's limit): the lower bounds it reached,
/// the minimal line members it certified, and no maximal line, which needs
/// the whole cube.
fn stopped(a: &Answer) -> Result<Region, String> {
    let (Some(d), Some(counts)) = (&a.described, &a.counts) else {
        return Err(format!(
            "a stopped description with no counts: {:?}",
            a.stopped
        ));
    };
    let mut minimal: Vec<Vec<String>> = d
        .minimal_commitments
        .iter()
        .map(|s| {
            let mut v: Vec<String> = s
                .iter()
                .map(|&b| d.candidates[b as usize].text.clone())
                .collect();
            v.sort();
            v
        })
        .collect();
    minimal.sort();
    Ok(Region {
        route: route_word(a.route),
        monotone: a.monotonicity.is_monotone(),
        k_s: counts.k_s,
        k_c: counts.k_c,
        empty: None,
        minimal,
        maximal: Vec::new(),
        stopped: Some(
            match a.stopped {
                Some(Stopped::Limit { .. }) => "limit",
                Some(Stopped::Budget { .. }) => "budget",
                Some(Stopped::Deadline) => "deadline",
                None => "?",
            }
            .to_string(),
        ),
    })
}

fn sorted(v: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut v: Vec<Vec<String>> = v
        .iter()
        .map(|s| {
            let mut s = s.clone();
            s.sort();
            s
        })
        .collect();
    v.sort();
    v
}

fn differences(want: &Region, got: &Region) -> Vec<String> {
    let mut out = Vec::new();
    let mut field = |name: &str, w: String, g: String| {
        if w != g {
            out.push(format!("{name}: recorded {w}, computed {g}"));
        }
    };
    field("route", want.route.clone(), got.route.clone());
    field(
        "monotone",
        want.monotone.to_string(),
        got.monotone.to_string(),
    );
    field("k_s", want.k_s.to_string(), got.k_s.to_string());
    field("k_c", want.k_c.to_string(), got.k_c.to_string());
    field(
        "empty",
        format!("{:?}", want.empty),
        format!("{:?}", got.empty),
    );
    field(
        "minimal",
        format!("{:?}", sorted(&want.minimal)),
        format!("{:?}", got.minimal),
    );
    field(
        "maximal",
        format!("{:?}", sorted(&want.maximal)),
        format!("{:?}", got.maximal),
    );
    field(
        "stopped",
        format!("{:?}", want.stopped),
        format!("{:?}", got.stopped),
    );
    out
}

fn quote(s: &str) -> String {
    serde_json::to_string(s).expect("a string serialises")
}

fn line_toml(name: &str, line: &[Vec<String>]) -> String {
    let rows: Vec<String> = line
        .iter()
        .map(|s| {
            let facts: Vec<String> = s.iter().map(|f| quote(f)).collect();
            format!("[{}]", facts.join(", "))
        })
        .collect();
    if rows.len() <= 1 {
        format!("{name:<9}= [{}]", rows.join(", "))
    } else {
        format!("{name:<9}= [\n  {},\n]", rows.join(",\n  "))
    }
}

/// The block as the register spells it. It holds no blank line, which is how
/// [`bless`] finds its end.
fn block(g: &Region) -> String {
    let mut out = vec![
        "[case.region]".to_string(),
        format!("route    = {}", quote(&g.route)),
        format!("monotone = {}", g.monotone),
        format!("k_s      = {}", g.k_s),
        format!("k_c      = {}", g.k_c),
    ];
    if let Some(e) = &g.empty {
        out.push(format!("empty    = {}", quote(e)));
    }
    if let Some(w) = &g.stopped {
        out.push(format!("stopped  = {}", quote(w)));
    }
    out.push(line_toml("minimal", &g.minimal));
    out.push(line_toml("maximal", &g.maximal));
    out.join("\n")
}

/// Put `block` into row `id` of the register's text: in place of its
/// `[case.region]`, or before its `[case.right]`.
fn bless(text: &str, id: &str, block: &str) -> String {
    let at = text
        .find(&format!("id       = {}", quote(id)))
        .unwrap_or_else(|| panic!("{id}: not in the register"));
    let end = [text[at..].find("\n[[case]]"), text[at..].find("\n# ══")]
        .into_iter()
        .flatten()
        .min()
        .map_or(text.len(), |e| at + e + 1);
    let row = &text[at..end];
    let new_row = match row.find("[case.region]\n") {
        Some(r0) => {
            let r1 = row[r0..].find("\n\n").map_or(row.len(), |e| r0 + e + 2);
            format!("{}{block}\n\n{}", &row[..r0], &row[r1..])
        }
        None => {
            let r0 = row
                .find("[case.right]")
                .unwrap_or_else(|| panic!("{id}: no [case.right]"));
            format!("{}{block}\n\n{}", &row[..r0], &row[r0..])
        }
    };
    format!("{}{new_row}{}", &text[..at], &text[end..])
}

#[test]
fn every_row_states_its_witness_s_region() {
    let reg = ein_corpus::register();
    let blessing = std::env::var("EIN_BLESS").as_deref() == Ok("1");
    let path = repo_root().join(REGISTER);
    let mut text = std::fs::read_to_string(&path).expect("the register reads");
    let mut bad = Vec::new();
    let mut checked = 0usize;
    for row in &reg.case {
        if row.status.starts_with("merged:") {
            continue;
        }
        // A refused row (M1e2 S1e2.2.5, *forbid*) states no region, and the
        // library refuses its witness as the binary does: at load since M1e2
        // S1e2.2.20 (W20, a wildcard relation in a rule), the binary's line
        // the loader's, at the parse since M1f S1f.5.3 (W20's range, until
        // S1f.5.4), the parser's, or by *forbid*.
        if let Some(want) = &row.now.refused {
            if let Some(e) = refusal(&row.witness) {
                if *want != e {
                    bad.push(format!(
                        "{} ({}): [now] is the refusal {want:?} and the loader says {e:?}",
                        row.id, row.witness
                    ));
                }
                checked += 1;
                continue;
            }
            let describe = Mode {
                which: Which::All,
                ask: Ask::Describe,
            };
            match try_ask(&repo_root().join(&row.witness), describe) {
                Err(ein_infer::SolveError::Refused(_)) => {}
                other => bad.push(format!(
                    "{} ({}): [now] is a refusal and the library answers {:?}",
                    row.id,
                    row.witness,
                    other.map(|a| a.route)
                )),
            }
            checked += 1;
            continue;
        }
        // A row whose run ends in an error (M1e2 S1e2.2.18, W18) states no
        // region either: the program loads, and the library stops where the
        // binary does, at a firing, with a saturation error, or, where a rule
        // the run compiles before it saturates does not compile, with a
        // compile error (M1e2 S1e2.2.22, `W22/kw-or-in-obligation`: an
        // obligation's guard read as a nested `(or …)`).
        if row.now.error.is_some() {
            let describe = Mode {
                which: Which::All,
                ask: Ask::Describe,
            };
            match try_ask(&repo_root().join(&row.witness), describe) {
                Err(ein_infer::SolveError::Saturate(_) | ein_infer::SolveError::Compile(_)) => {}
                other => bad.push(format!(
                    "{} ({}): [now] is an error and the library answers {:?}",
                    row.id,
                    row.witness,
                    other.map(|a| a.route)
                )),
            }
            checked += 1;
            continue;
        }
        let got = match computed(&repo_root().join(&row.witness)) {
            Ok(g) => g,
            Err(e) => {
                bad.push(format!("{} ({}): {e}", row.id, row.witness));
                continue;
            }
        };
        checked += 1;
        let diffs = match &row.region {
            Some(want) => differences(want, &got),
            None => vec!["no [case.region]".to_string()],
        };
        if diffs.is_empty() {
            continue;
        }
        if blessing {
            text = bless(&text, &row.id, &block(&got));
        } else {
            bad.push(format!(
                "{} ({}):\n    {}\n  the library's block:\n{}",
                row.id,
                row.witness,
                diffs.join("\n    "),
                block(&got)
            ));
        }
    }
    if blessing {
        std::fs::write(&path, text).expect("the register writes");
    }
    assert!(checked > 0, "the register has no rows to check");
    assert!(
        bad.is_empty(),
        "{} of {checked} rows state a region the library does not compute \
         (re-bank with EIN_BLESS=1 cargo test -p ein-infer --test register_region):\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
}
