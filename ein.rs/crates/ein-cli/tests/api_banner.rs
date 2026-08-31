//! **`docs/api/`'s five history banners name the CLI that ships** — M1e
//! `CD-L1`.
//!
//! The five Python pages are history and are kept unedited on purpose
//! ([Q-M1a.23]). The 🏛 banner at the top of each is the one part of them that
//! describes the **present**, and it is the same 21 lines copied five times.
//! It went stale the day after it was written: the banner landed 2026-08-23
//! and M1c [S1c.1.3] added `ein test` on 2026-08-24, so five history pages
//! spent a milestone naming four subcommands.
//!
//! Five verbatim copies of one text is
//! [`AR-M1`](../../../../plans/m1e_review_processing/p1e.3_medium/s1e.3.4_architecture.md)'s
//! shape, and markdown has no include mechanism, so the state it can reach is
//! the third one that rule allows: **mechanically compared by a test**. The
//! `<!-- api-history-banner -->` markers serve both halves — `grep -rn
//! api-history-banner docs/api/` finds all five, and they are this file's
//! extraction anchor.
//!
//! The subcommand list is not restated here either: it is read off
//! `golden/help_shape.txt`, which already owns the CLI surface
//! (`help_surface.rs`), so the banner cites an owner rather than becoming a
//! sixth list.
//!
//! [Q-M1a.23]: ../../../../docs/history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding
//! [S1c.1.3]: ../../../../docs/history/m1c_external_validation/README.md#s1c13--ein-test

use ein_corpus::repo_root;

const PAGES: [&str; 5] = ["ein.md", "ir.md", "kb.md", "inference.md", "trace.md"];
const MARKER: &str = "<!-- api-history-banner -->";
const END: &str = "<!-- /api-history-banner -->";

/// The marked region of one page.
fn banner(page: &str) -> String {
    let path = repo_root().join("docs/api").join(page);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let (_, rest) = text
        .split_once(MARKER)
        .unwrap_or_else(|| panic!("docs/api/{page} has no {MARKER}"));
    let (region, _) = rest
        .split_once(END)
        .unwrap_or_else(|| panic!("docs/api/{page} opens a banner region and never closes it"));
    region.to_string()
}

/// Every top-level subcommand of the default build, from the golden that owns
/// the surface — the `COMMAND ein <name>` rows with no further space in them.
fn subcommands() -> Vec<String> {
    let golden = repo_root().join("ein.rs/crates/ein-cli/tests/golden/help_shape.txt");
    let text = std::fs::read_to_string(&golden).expect("help_shape.txt");
    let mut out: Vec<String> = text
        .lines()
        .filter_map(|l| l.strip_prefix("COMMAND ein "))
        .filter(|n| !n.contains(' '))
        .map(str::to_string)
        .collect();
    out.sort();
    out.dedup();
    out
}

/// **The five banners are one text.**
///
/// An edit to one that is not made to the other four fails here, which is the
/// only state available to five copies in a tree with no include mechanism.
#[test]
fn the_five_banners_are_one_text() {
    let first = banner(PAGES[0]);
    for page in &PAGES[1..] {
        assert_eq!(
            banner(page),
            first,
            "docs/api/{page}'s banner has drifted from docs/api/{}'s. It is the \
             one part of a history page that describes the present, and it is \
             maintained by hand in five places (CD-L1)",
            PAGES[0]
        );
    }
}

/// **The banner names every subcommand the binary has.**
///
/// It named four of five for a week. The list comes from `help_shape.txt`
/// rather than from a literal here, so a subcommand added to the default
/// surface fails this without anyone having to remember the banner exists.
#[test]
fn the_banner_names_every_subcommand() {
    let text = banner(PAGES[0]);
    let names = subcommands();
    assert_eq!(
        names.len(),
        5,
        "the default CLI surface changed — {names:?}. That is the event this \
         test exists for: add it to the banner in all five docs/api pages"
    );
    for name in &names {
        // The banner writes `ein solve <file>`, so the closing backtick is not
        // adjacent to the name — match the opening form and a boundary.
        let want = format!("`ein {name}");
        assert!(
            text.contains(&want) && !text.contains(&format!("{want}-")),
            "the docs/api banner does not name `ein {name}`, which \
             golden/help_shape.txt says the binary has (CD-L1)"
        );
    }
}
