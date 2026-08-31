//! **A marked transcript is a real run** — M1e `CD-L2`.
//!
//! [`docs/guide/`](../../../../docs/guide/) is the newcomer's first contact
//! with the tool, and nothing ran its transcripts. Chapter 4's — the tutorial's
//! final page, the payoff — differed from the binary in two places: it packed
//! four sorted bindings two-per-line, and it dropped the 62-character rule the
//! solve table prints under its title.
//!
//! It was not drift. `git show b4d5158` adds that text on 2026-06-17, and the
//! engine shipping that day already printed one binding per line under a rule
//! (`ein.py`'s `_two_col` and `_rule`, and `ein-render`'s `answer.rs` does the
//! same today). **No engine in this repo's history has printed what the page
//! printed** — which is why the fix is *take the bytes from a run*, and the
//! pin is *keep taking them*.
//!
//! ## What is pinned, and what is not
//!
//! A block is pinned by wrapping it in
//! `<!-- transcript: <argv> -->` … `<!-- /transcript -->`. The marker carries
//! the command, so adding a block later is adding a marker, not editing this
//! file. Two are marked today: chapter 4's and `README.md`'s copy of the same
//! run, which is the one that was right and is the copy the guide's drifted
//! from.
//!
//! **Chapter 2's three blocks stay hand-maintained on purpose.** They are
//! *excerpts* — they elide the header, the rule and the empty
//! `query bindings` — and an exact diff cannot express an elision. Pinning
//! them would push seven lines of `(query has no :goal-text template)` into a
//! tutorial to satisfy a test. Their lines are byte-correct today; that they
//! are excerpts is stated in
//! [`docs/guide/README.md`](../../../../docs/guide/README.md), which is where
//! a reader who wonders why one page is generated and another is not will
//! look.
//!
//! ```sh
//! EIN_BLESS=1 cargo test -p ein-cli --test guide_transcripts
//! ```
//!
//! Its blind spot is [`CD-M4`]'s: prose outside the marker is unguarded.
//!
//! [`CD-M4`]: ../../../../plans/m1e_review_processing/p1e.3_medium/s1e.3.7_code_doc_consistency.md

use std::path::PathBuf;
use std::process::Command;

use ein_corpus::repo_root;

/// Every page that may carry a marked transcript, repo-relative.
const PAGES: [&str; 6] = [
    "README.md",
    "docs/guide/README.md",
    "docs/guide/01_objects_and_relations.md",
    "docs/guide/02_first_rules.md",
    "docs/guide/03_rule_families.md",
    "docs/guide/04_solving_the_whole_puzzle.md",
];

const OPEN: &str = "<!-- transcript: ";
const CLOSE: &str = "<!-- /transcript -->";

fn path_of(rel: &str) -> PathBuf {
    repo_root().join(rel)
}

/// What the binary prints for `argv`, as the page shows it: the `$` line, then
/// stdout and stderr in that order.
fn transcript(argv: &str) -> String {
    let args: Vec<&str> = argv.split_whitespace().skip(1).collect();
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(&args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    let mut s = format!("$ {argv}\n");
    s.push_str(&String::from_utf8_lossy(&out.stdout));
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

/// **Every marked block on a guide page is what the binary prints.**
///
/// The `assert!(checked >= 2)` at the end is the guard against the failure
/// this test exists to prevent happening to *it*: a renamed marker would
/// otherwise leave it green over nothing.
#[test]
fn every_marked_guide_transcript_is_a_real_run() {
    let bless = std::env::var("EIN_BLESS").as_deref() == Ok("1");
    let mut checked = 0usize;
    for rel in PAGES {
        let path = path_of(rel);
        let mut text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}"));
        let mut from = 0usize;
        while let Some(i) = text[from..].find(OPEN) {
            let i = from + i;
            let head_end = text[i..]
                .find("-->")
                .expect("an unterminated transcript marker")
                + i;
            let argv = text[i + OPEN.len()..head_end].trim().to_string();
            let body_start = text[head_end..]
                .find("```sh\n")
                .expect("a transcript marker with no ```sh block after it")
                + head_end
                + "```sh\n".len();
            let body_end = text[body_start..]
                .find("\n```\n")
                .expect("an unterminated transcript block")
                + body_start
                + 1;
            let close = text[body_end..]
                .find(CLOSE)
                .expect("a transcript block with no closing marker")
                + body_end;

            let want = transcript(&argv);
            let got = &text[body_start..body_end];
            checked += 1;
            if got != want {
                assert!(
                    bless,
                    "{rel}: the block for `{argv}` is not what the binary prints. \
                     Re-bank it with `EIN_BLESS=1 cargo test -p ein-cli --test \
                     guide_transcripts` and read the diff (CD-L2).\n\
                     --- page ---\n{got}\n--- binary ---\n{want}"
                );
                text.replace_range(body_start..body_end, &want);
                std::fs::write(&path, &text).expect("the page is writable");
                from = 0;
                continue;
            }
            from = close + CLOSE.len();
        }
    }
    assert!(
        checked >= 2,
        "no marked transcript was found on any page — the marker was renamed \
         and this test went quiet, which is the shape of failure it exists to \
         prevent (CD-L2)"
    );
}
