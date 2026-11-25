//! **The kinds of Ein's diagnostics are the reference's** (Q-M2.9).
//!
//! A refusal's kind is named beside the section of
//! `docs/ein/user/04-reference/07-diagnostics.md` that states it, a
//! `**Kind:**` line under the heading, and a line a run writes beside an
//! answer in the `kind` column of `08-read-out.md` § 10. These tests hold the
//! two pages and `ein_core::Kind` to one list, as the pages' messages are held
//! to the fixtures:
//!
//! | claim | where |
//! |---|---|
//! | every kind the pages name is one of the code's, and every kind of the code's is named on a page | [`the_pages_and_the_code_name_the_same_kinds`] |
//! | a section's own kind, the first its `**Kind:**` line names, is no other section's, but the parse's | [`one_kind_is_one_section`] |
//! | the section a kind's doc comment names is the one the page writes it beside | [`each_kind_is_documented_at_its_section`] |

use std::collections::{BTreeMap, BTreeSet};

use ein_core::Kind;
use ein_corpus::repo_root;

const DIAGNOSTICS: &str = "docs/ein/user/04-reference/07-diagnostics.md";
const READ_OUT: &str = "docs/ein/user/04-reference/08-read-out.md";

fn read(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

/// The backticked words of `text`, in order.
fn ticked(text: &str) -> Vec<String> {
    text.split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

/// Each `**Kind:**` line of the diagnostics page, with the number of the
/// section it stands under (`2.13`, `1`), and the kinds it names in order.
fn sections() -> Vec<(String, Vec<String>)> {
    let page = read(DIAGNOSTICS);
    let mut out = Vec::new();
    let mut section = String::new();
    for line in page.lines() {
        if let Some(h) = line
            .strip_prefix("### ")
            .or_else(|| line.strip_prefix("## "))
        {
            section = h
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_end_matches('.')
                .to_string();
        } else if let Some(rest) = line.strip_prefix("**Kind:**") {
            out.push((section.clone(), ticked(rest)));
        }
    }
    assert!(out.len() > 20, "only {} `**Kind:**` lines", out.len());
    out
}

/// The kinds of `08-read-out.md` § 10's table, its last column.
fn read_out_kinds() -> Vec<String> {
    let page = read(READ_OUT);
    let at = page
        .find("## 10. Stderr")
        .expect("§ 10 of the read-out's page");
    let section = &page[at..];
    let section = &section[..section.find("\n---").expect("the end of § 10")];
    let mut out = Vec::new();
    for row in section.lines().filter(|l| l.starts_with("| `")) {
        let cells: Vec<&str> = row.trim_end_matches('|').rsplitn(2, " | ").collect();
        let kind = ticked(cells[0]);
        assert_eq!(kind.len(), 1, "a row's kind is one name: {row}");
        out.extend(kind);
    }
    assert!(out.len() >= 7, "only {} rows in § 10", out.len());
    out
}

#[test]
fn the_pages_and_the_code_name_the_same_kinds() {
    let mut named: BTreeSet<String> = sections().into_iter().flat_map(|(_, k)| k).collect();
    named.extend(read_out_kinds());
    let code: BTreeSet<String> = Kind::ALL.iter().map(|k| k.name().to_string()).collect();
    let unknown: Vec<_> = named.difference(&code).collect();
    assert!(
        unknown.is_empty(),
        "the pages name {unknown:?}, which is no kind of ein_core::Kind"
    );
    let unnamed: Vec<_> = code.difference(&named).collect();
    assert!(
        unnamed.is_empty(),
        "{unnamed:?} is a kind of ein_core::Kind that neither {DIAGNOSTICS} nor {READ_OUT} § 10 \
         names"
    );
}

#[test]
fn one_kind_is_one_section() {
    let mut home: BTreeMap<String, String> = BTreeMap::new();
    for (section, kinds) in sections() {
        let own = kinds
            .first()
            .unwrap_or_else(|| panic!("§ {section}'s `**Kind:**` line names no kind"));
        if own == "parse" {
            continue;
        }
        if let Some(before) = home.insert(own.clone(), section.clone()) {
            panic!("`{own}` is the kind of § {before} and of § {section}");
        }
    }
}

/// Each variant's doc comment in `ein-core/src/diagnostic.rs` opens with the
/// section it is the kind of, `/// § 2.13: …`; that is the section whose
/// `**Kind:**` line names it first. A section renumbered on the page and not
/// in the code fails here.
#[test]
fn each_kind_is_documented_at_its_section() {
    let src = read("ein.rs/crates/ein-core/src/diagnostic.rs");
    let mut documented: BTreeMap<String, String> = BTreeMap::new();
    // The section a doc comment's first line opens with, and whether the
    // line before was a doc comment's.
    let (mut last, mut in_doc): (Option<String>, bool) = (None, false);
    for line in src.lines().map(str::trim) {
        if let Some(rest) = line.strip_prefix("///") {
            if !in_doc {
                last = rest
                    .trim_start()
                    .strip_prefix("§ ")
                    .and_then(|r| r.split(':').next())
                    .map(str::to_string);
            }
            in_doc = true;
            continue;
        }
        in_doc = false;
        if let Some((_, name)) = line.split_once(" = \"")
            && let Some(section) = last.take()
        {
            documented.insert(name.trim_end_matches("\",").to_string(), section);
        }
        last = None;
    }
    assert!(
        documented.len() > 20,
        "only {} documented",
        documented.len()
    );
    for (section, kinds) in sections() {
        let own = &kinds[0];
        // The parse's kind stands first beside every section whose lines are
        // the parse's; its own is § 1.
        if own == "parse" && section != "1" {
            continue;
        }
        if let Some(at) = documented.get(own) {
            assert_eq!(
                at, &section,
                "`{own}`'s doc comment names § {at}, and the page writes it beside § {section}"
            );
        }
    }
    let homes: BTreeSet<String> = sections().into_iter().map(|(_, k)| k[0].clone()).collect();
    for name in documented.keys() {
        assert!(
            homes.contains(name),
            "`{name}` is documented at a section, and no section's `**Kind:**` line names it first"
        );
    }
}
