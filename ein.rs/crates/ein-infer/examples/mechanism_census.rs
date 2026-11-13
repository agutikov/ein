//! The hypothesis mechanisms each program declares, and every pair of them,
//! read at root's fixpoint — M1f
//! [S1f.6.34](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.34_hypothesis_mechanisms_in_conflict.md)'s
//! T1.
//!
//! A derived closure (`std.closure`'s `infer-closure`, a calculator module's
//! closing rule) and a derived obligation activator (`bijective-setup`'s
//! `(total color-loc is-a)`) exist only at root's fixpoint, so the census
//! loads each program, saturates root as `region::prepare` does
//! (`shared/census.rs`'s `root_of`) and reads [`ein_infer::mechanisms::read`]
//! there, never the program's text. Per program it prints the mechanisms
//! present and, per pair of § The pairs, the relations the pair's condition
//! holds of:
//!
//! | pair | the relations printed |
//! |---|---|
//! | H + O | owable relations an active hrule proposes, and those it does not |
//! | H + HR | proposed relations the list leaves out |
//! | H + NH | proposed relations the list names |
//! | H + C | proposed relations that are closed |
//! | O + HR | owable relations the list leaves out |
//! | O + NH | owable relations the list names |
//! | O + C | owable relations that are closed |
//! | HR + NH | relations in both; and those NH names that HR leaves out |
//! | HR + C | listed relations that are closed |
//! | NH + C | relations in both |
//!
//! ```sh
//! cargo run --release -p ein-infer --example mechanism_census -- [--json] [FILE…]
//! ```
//!
//! With no `FILE` it sweeps `corpus_files()` plus the register's witnesses.
//! It writes nothing and changes no answer.

#[allow(dead_code)]
#[path = "shared/census.rs"]
mod census;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use ein_core::{Symbol, Terms};
use ein_infer::compile::SharedMemo;
use ein_infer::events::Events;
use ein_infer::mechanisms::{self, Mechanisms};
use ein_infer::saturator::Session;
use serde_json::json;

fn names(terms: &Terms, set: impl IntoIterator<Item = Symbol>) -> Vec<String> {
    let mut v: Vec<String> = set.into_iter().map(|s| terms.sym(s).to_string()).collect();
    v.sort();
    v.dedup();
    v
}

/// The ten pairs, each the relations its condition holds of.
fn pairs(m: &Mechanisms, terms: &Terms) -> BTreeMap<&'static str, Vec<String>> {
    let proposed = m.proposed();
    let owable = m.owable();
    let listed = m.listed.clone();
    let left_out = |r: &Symbol| listed.as_ref().is_some_and(|l| !l.contains(r));
    let mut out = BTreeMap::new();
    if !m.hrules.is_empty() && !m.obligation_rules.is_empty() {
        out.insert(
            "H+O",
            names(terms, owable.iter().copied())
                .into_iter()
                .map(|r| {
                    let p = proposed.iter().any(|&s| terms.sym(s) == r);
                    format!("{r}{}", if p { "" } else { " (no hrule proposes it)" })
                })
                .collect(),
        );
    }
    if !m.hrules.is_empty() && listed.is_some() {
        out.insert(
            "H+HR",
            names(terms, proposed.iter().copied().filter(|r| left_out(r))),
        );
    }
    if !m.hrules.is_empty() && !m.excluded.is_empty() {
        out.insert(
            "H+NH",
            names(terms, proposed.intersection(&m.excluded).copied()),
        );
    }
    if !m.hrules.is_empty() && !m.closed.is_empty() {
        out.insert(
            "H+C",
            names(terms, proposed.intersection(&m.closed).copied()),
        );
    }
    if !m.obligation_rules.is_empty() && listed.is_some() {
        out.insert(
            "O+HR",
            names(terms, owable.iter().copied().filter(|r| left_out(r))),
        );
    }
    if !m.obligation_rules.is_empty() && !m.excluded.is_empty() {
        out.insert(
            "O+NH",
            names(terms, owable.intersection(&m.excluded).copied()),
        );
    }
    if !m.obligation_rules.is_empty() && !m.closed.is_empty() {
        out.insert("O+C", names(terms, owable.intersection(&m.closed).copied()));
    }
    if let Some(l) = &listed
        && !m.excluded.is_empty()
    {
        let both = names(terms, l.intersection(&m.excluded).copied());
        let redundant = names(terms, m.excluded.iter().copied().filter(|r| !l.contains(r)));
        let mut v: Vec<String> = both.into_iter().map(|r| format!("{r} (both)")).collect();
        v.extend(redundant.into_iter().map(|r| format!("{r} (NH only)")));
        out.insert("HR+NH", v);
    }
    if let Some(l) = &listed
        && !m.closed.is_empty()
    {
        out.insert("HR+C", names(terms, l.intersection(&m.closed).copied()));
    }
    if !m.excluded.is_empty() && !m.closed.is_empty() {
        out.insert(
            "NH+C",
            names(terms, m.excluded.intersection(&m.closed).copied()),
        );
    }
    out
}

fn main() {
    let mut as_json = false;
    let mut files: Vec<PathBuf> = Vec::new();
    for a in std::env::args().skip(1) {
        match a.as_str() {
            "--json" => as_json = true,
            _ => files.push(PathBuf::from(a)),
        }
    }
    if files.is_empty() {
        files = census::selection();
    }
    let given = files.len();
    let mut swept = 0usize;
    // pair → (programs carrying it, programs whose condition holds of a relation)
    let mut tally: BTreeMap<&'static str, (BTreeSet<String>, BTreeSet<String>)> = BTreeMap::new();
    let mut rows = Vec::new();
    for path in files {
        let rel = census::relative(&path);
        let Some((mut kb, mut terms, ast)) = census::loaded(&path) else {
            continue;
        };
        // The closures written: read before root saturates.
        let written_closed = {
            let mut events = Events::off();
            let mut s = Session {
                kb: &mut kb,
                terms: &mut terms,
                ast: &ast,
                events: &mut events,
                memo: SharedMemo::default(),
            };
            match mechanisms::read(&mut s) {
                Ok(m) => m.closed,
                Err(_) => continue,
            }
        };
        let Some((root, m)) =
            census::root_of(&mut kb, &mut terms, &ast, |_, s| mechanisms::read(s).ok())
        else {
            continue;
        };
        swept += 1;
        let ps = pairs(&m, &terms);
        let mut present = Vec::new();
        if !m.hrules.is_empty() {
            present.push(if m.proposals.is_empty() {
                "H(inactive)"
            } else {
                "H"
            });
        }
        if !m.obligation_rules.is_empty() {
            present.push(if m.obligations.is_empty() {
                "O(no activator)"
            } else {
                "O"
            });
        }
        if m.listed.is_some() {
            present.push("HR");
        }
        if !m.excluded.is_empty() {
            present.push("NH");
        }
        if !m.closed.is_empty() {
            present.push("C");
        }
        for (pair, hit) in &ps {
            let e = tally.entry(pair).or_default();
            e.0.insert(rel.clone());
            if !hit.is_empty() {
                e.1.insert(rel.clone());
            }
        }
        let owed: Vec<String> = root
            .owes
            .by_relation()
            .into_iter()
            .map(|(r, n)| format!("{}:{n}", terms.sym(r)))
            .collect();
        let derived_closed = names(
            &terms,
            m.closed
                .iter()
                .copied()
                .filter(|r| !written_closed.contains(r)),
        );
        if as_json {
            rows.push(json!({
                "file": rel,
                "present": present,
                "rung": root.rung.as_str(),
                "proposed": names(&terms, m.proposed()),
                "owable": names(&terms, m.owable()),
                "owed_at_root": owed,
                "listed": m.listed.as_ref().map(|l| names(&terms, l.iter().copied())),
                "excluded": names(&terms, m.excluded.iter().copied()),
                "closed": names(&terms, m.closed.iter().copied()),
                "closed_derived": derived_closed,
                "pairs": ps,
            }));
        } else if present.len() >= 2 {
            println!(
                "{rel}  [{}]  rung={}",
                present.join(" "),
                root.rung.as_str()
            );
            for (pair, hit) in &ps {
                println!("    {pair:6} {}", hit.join(", "));
            }
        }
    }
    if as_json {
        println!(
            "{}",
            serde_json::to_string_pretty(
                &json!({"swept": swept, "given": given, "programs": rows})
            )
            .expect("json")
        );
        return;
    }
    println!();
    println!("swept {swept} of {given} files (the rest do not load or do not saturate)");
    println!("pair    carried  holds-of-a-relation");
    for (pair, (carried, hit)) in &tally {
        println!("{pair:7} {:7}  {}", carried.len(), hit.len());
    }
}
