//! **The world-anchor list is the grep's own answer** — M1e `TE-L2`.
//!
//! `examples/zebra.ein` and `examples/zebra2.ein` are hard-coded into tests
//! across six crates. They are deliberate anchor tests and their own docs say
//! so; what nobody had was the **list**, so a reviewer editing `zebra2.ein`
//! could not tell what would fire. The review said four crates, M1e S1e.1.6
//! measured six and 26 files, and by the time this stage re-took it two days
//! later it was 28 — both additions from this milestone's own commits.
//!
//! That is why the list is generated rather than written: a hand-copied one
//! was stale before the phase closed. It lives in
//! [`examples/README.md`](../../../../examples/README.md) § What an edit to
//! these two files fans out into, in a marked block this test diffs, on
//! `config_reference.rs`'s pattern — *edit the code, run the test, paste;
//! never edit the block by hand.*
//!
//! ```sh
//! EIN_BLESS=1 cargo test -p ein-cli --test world_anchors
//! ```
//!
//! What the test cannot re-derive is in the page's rings 3 and 4 — the
//! `utils/` scripts, `docs/api/rust.md`, `corpus.toml`, and the three
//! `from_ein_py/` goldens that may never be re-blessed. Ring 4 is the one that
//! matters and the one no instrument can hold: an edit changing the *parse*
//! spends the repo's last independent provenance, and nothing fails at the
//! moment it happens.

use std::path::{Path, PathBuf};

use ein_corpus::repo_root;

const BEGIN: &str = "<!-- generated: grep -rl 'examples/zebra' ein.rs/crates/*/tests/*.rs \
                     ein.rs/crates/*/benches/*.rs -->\n```text\n";
const END: &str = "\n```\n<!-- /generated -->";

fn page_path() -> PathBuf {
    repo_root().join("examples/README.md")
}

/// Every `tests/` or `benches/` source under `ein.rs/crates/` naming a zebra
/// puzzle, crate-relative and sorted — the grep in the marker, in Rust.
fn coupled_files() -> Vec<String> {
    let crates = repo_root().join("ein.rs/crates");
    let mut out: Vec<String> = Vec::new();
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&crates)
        .expect("ein.rs/crates")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    for krate in dirs {
        for sub in ["tests", "benches"] {
            let dir = krate.join(sub);
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for e in entries.filter_map(|e| e.ok()) {
                let path = e.path();
                if path.extension().and_then(|x| x.to_str()) != Some("rs") {
                    continue;
                }
                let rel = relative(&path, &crates);
                // This file names both puzzles in its own prose and depends on
                // neither, so listing it would be the registry describing
                // itself. Every other self-reference is a real coupling.
                if rel == "ein-cli/tests/world_anchors.rs" {
                    continue;
                }
                let text = std::fs::read_to_string(&path).expect("a readable test source");
                if text.contains("examples/zebra") {
                    out.push(rel);
                }
            }
        }
    }
    out.sort();
    out
}

fn relative(path: &Path, base: &Path) -> String {
    path.strip_prefix(base)
        .expect("under ein.rs/crates")
        .to_string_lossy()
        .replace('\\', "/")
}

/// The page's generated block **is** the grep's answer.
///
/// It fails when a test starts or stops naming a zebra puzzle, which is the
/// event the section exists to make visible: the list moved 26 → 28 in two
/// days without anyone noticing, because nothing read it.
#[test]
fn the_anchor_list_is_the_greps_own_answer() {
    let want = coupled_files().join("\n");
    let page = std::fs::read_to_string(page_path()).expect("examples/README.md");
    let (before, rest) = page
        .split_once(BEGIN)
        .expect("examples/README.md has no world-anchor opening marker");
    let (got, after) = rest
        .split_once(END)
        .expect("examples/README.md has no world-anchor closing marker");

    if std::env::var("EIN_BLESS").as_deref() == Ok("1") && got != want {
        std::fs::write(page_path(), format!("{before}{BEGIN}{want}{END}{after}"))
            .expect("the page is writable");
        return;
    }
    assert_eq!(
        got, want,
        "the anchor list in examples/README.md is not what the grep says. \
         Re-bank it with `EIN_BLESS=1 cargo test -p ein-cli --test world_anchors`, \
         and read the diff first: a test gaining a zebra reference is a test that \
         will fire the next time somebody edits one of those two puzzles (TE-L2)"
    );
}

/// **Both puzzle files point at the section**, which is the acceptance line.
///
/// A `;`-comment is lexer trivia and reaches no golden, so the header costs
/// nothing — except through the generator, which copies `zebra2.ein` whole
/// into four checked-in files. That is `cli_semantics`'s `--check` cell, and
/// it is why a header on `zebra2.ein` is the one comment in this repo that can
/// turn the gate red.
#[test]
fn both_puzzles_name_the_registry() {
    for rel in ["examples/zebra.ein", "examples/zebra2.ein"] {
        let text = std::fs::read_to_string(repo_root().join(rel)).expect(rel);
        assert!(
            text.contains("examples/README.md"),
            "{rel} does not point at the world-anchor registry — a reviewer \
             editing it has no list of what will fire (TE-L2)"
        );
    }
}
