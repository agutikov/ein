//! **Every program block in the guide runs on today's engine** — M1f S1f.5.26.
//!
//! [`docs/guide/`](../../../../docs/guide/) is the newcomer's first contact
//! with Ein, and until S1f.5.26 nothing read its `lisp` blocks:
//! [`guide_transcripts.rs`](guide_transcripts.rs) diffs the *marked
//! transcripts* and nothing else. So chapter 4's block went on writing
//! `(config :enable-pre-branch-lookahead true)` after M1e2 S1e2.1c.5 retired
//! the flag, and a reader who copied it got a refusal; chapter 3's
//! `adjacent-via-fwd` and `disjunctive-prune-fwd` carried no `:why` once
//! S1f.5.14's flip required one. The close review's finding 6 found the
//! first, and the user ruled *"fix the tutorial, new stage"* (2026-11-10).
//!
//! ## What is read, and how
//!
//! Every fenced `lisp` block of every page in `docs/guide/`, by
//! [`read_guide`]:
//!
//! - **A whole program** is a block that writes no `…` outside a string. It is
//!   loaded with `ein_ir::load_file`, under the default options (so a `:why`
//!   is required), and then `ein solve FILE` must answer it: exit 0. A block
//!   that states an `:expect` the engine refutes fails here.
//! - **An excerpt** writes `…`. Each top-level form it writes whole is parsed
//!   and loaded, all of them together as one program and each where it stands
//!   in the block, so a position the parser prints is the page's. A top-level
//!   form with a `…` in it outside a string and a comment is *elided*, and is
//!   blanked rather than read, as is a `…` standing between forms.
//!
//! A retired `(config …)` flag, a keyword a form does not take and a missing
//! `:why` each fail [`every_program_block_in_the_guide_runs`], and
//! [`a_planted_refusal_in_a_copy_of_the_guide_fails_it`] holds the reading
//! the other way, on a copy of the pages in a scratch directory.
//!
//! What this does not read is the prose around a block, the `sh` commands
//! outside a marked transcript, and whether an excerpt quotes the file it says
//! it is from; [`docs/guide/README.md`](../../../../docs/guide/README.md) says
//! so where a reader of the guide will look. The choices above, that `…`
//! decides the kind and that an excerpt's whole forms load together, are
//! [Q-M1f.69]'s, taken by default.
//!
//! ```sh
//! cargo test -p ein-cli --test guide_programs
//! ```
//!
//! [Q-M1f.69]: ../../../../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f69--what-the-guides-pin-reads-in-a-program-block

use std::path::{Path, PathBuf};
use std::process::Command;

use ein_core::Terms;
use ein_corpus::{Scratch, repo_root};
use ein_ir::Ast;

/// The guide, repo-relative.
const GUIDE: &str = "docs/guide";

/// What marks an elision on a guide page.
const ELLIPSIS: char = '…';

/// One fenced `lisp` block of a page.
struct Block {
    /// The page's file name.
    page: String,
    /// The 1-based line of the opening fence; the block's line `n` is the
    /// page's line `fence + n`.
    fence: usize,
    /// How far the fence is indented; each line of the block loses as much.
    indent: usize,
    /// The block's lines, the indentation taken off, each ending in `\n`.
    text: String,
}

/// Every fenced `lisp` block of `page`, whose text is `text`.
fn blocks(page: &str, text: &str) -> Vec<Block> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim_start();
        if !trimmed.starts_with("```lisp") {
            i += 1;
            continue;
        }
        let indent = lines[i].len() - trimmed.len();
        let fence = i + 1;
        let mut body = String::new();
        i += 1;
        while i < lines.len() && lines[i].trim() != "```" {
            let line = lines[i];
            let lead = line.len() - line.trim_start_matches(' ').len();
            body.push_str(&line[lead.min(indent)..]);
            body.push('\n');
            i += 1;
        }
        assert!(
            i < lines.len(),
            "{page}:{fence}: a ```lisp block with no closing fence"
        );
        out.push(Block {
            page: page.to_string(),
            fence,
            indent,
            text: body,
        });
        i += 1;
    }
    out
}

/// A block read as Ein text, before anything loads it.
struct Reading {
    /// It writes `…` outside a string, so it is an excerpt.
    excerpt: bool,
    /// The text that is loaded: the block, with every elided top-level form
    /// and every `…` between forms blanked character for character, newlines
    /// kept, so a line and column in it are the block's.
    checked: String,
    /// Top-level forms written whole.
    complete: usize,
    /// Top-level forms with a `…` in them.
    elided: usize,
}

/// Read `text` as the lexer does where it matters here (`ein-ir`'s
/// `src/ein.l`): a string is `"…"` with `\` escaping the next character, a
/// comment runs from `;` to the end of the line or from `#|` to `|#`, and a
/// form is what one parenthesis opens and its match closes.
fn read(text: &str) -> Reading {
    let chars: Vec<char> = text.chars().collect();
    let mut blank = vec![false; chars.len()];
    let (mut excerpt, mut complete, mut elided) = (false, 0, 0);
    let (mut depth, mut start, mut this_elided) = (0usize, 0usize, false);
    let (mut in_str, mut esc, mut line_comment, mut block_comment) = (false, false, false, false);
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if line_comment || block_comment {
            if c == ELLIPSIS {
                excerpt = true;
            }
            if line_comment && c == '\n' {
                line_comment = false;
            } else if block_comment && c == '|' && chars.get(i + 1) == Some(&'#') {
                block_comment = false;
                i += 1;
            }
        } else if in_str {
            if esc {
                esc = false;
            } else if c == '\\' {
                esc = true;
            } else if c == '"' {
                in_str = false;
            }
        } else {
            match c {
                ';' => line_comment = true,
                '#' if chars.get(i + 1) == Some(&'|') => {
                    block_comment = true;
                    i += 1;
                }
                '"' => in_str = true,
                '(' => {
                    if depth == 0 {
                        start = i;
                        this_elided = false;
                    }
                    depth += 1;
                }
                ')' if depth > 0 => {
                    depth -= 1;
                    if depth == 0 {
                        if this_elided {
                            elided += 1;
                            blank[start..=i].fill(true);
                        } else {
                            complete += 1;
                        }
                    }
                }
                ELLIPSIS => {
                    excerpt = true;
                    if depth == 0 {
                        blank[i] = true;
                    } else {
                        this_elided = true;
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    // A form left open is the parser's to refuse, unless it is an elision.
    if depth > 0 && this_elided {
        elided += 1;
        blank[start..].fill(true);
    }
    let checked = chars
        .iter()
        .zip(&blank)
        .map(|(&c, &b)| if b && c != '\n' { ' ' } else { c })
        .collect();
    Reading {
        excerpt,
        checked,
        complete,
        elided,
    }
}

/// `err` with every position in the scratch file `path` written as the
/// page's: `<path>:<line>:<col>` → `<page>:<fence + line>:<indent + col>`.
fn at_page(err: &str, path: &str, page: &str, block: &Block) -> String {
    let prefix = format!("{path}:");
    let mut out = String::new();
    let mut rest = err;
    while let Some(k) = rest.find(&prefix) {
        out.push_str(&rest[..k]);
        let after = &rest[k + prefix.len()..];
        let mut nums = after.splitn(3, ':');
        let (line, col) = (nums.next().unwrap_or(""), nums.next().unwrap_or(""));
        match (line.parse::<usize>(), col.parse::<usize>()) {
            (Ok(l), Ok(c)) => {
                out.push_str(&format!("{page}:{}:{}", block.fence + l, block.indent + c));
                rest = &after[line.len() + 1 + col.len()..];
            }
            _ => {
                out.push_str(&format!("{page}:"));
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

/// What [`read_guide`] found.
#[derive(Default)]
struct Report {
    /// One entry per block refused or not answered, naming its page and line.
    findings: Vec<String>,
    /// `(page, fence, excerpt, complete forms)` for every block read.
    read: Vec<(String, usize, bool, usize)>,
}

/// Read every `lisp` block of every `.md` page in `dir`, which `shown` names
/// in a finding (`docs/guide` for the guide itself). Each block's text is
/// written under `scratch` to be loaded and, if it is a whole program, solved.
fn read_guide(dir: &Path, shown: &str, scratch: &Path) -> Report {
    let mut pages: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|entry| entry.expect("a directory entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "md"))
        .collect();
    pages.sort();
    let mut report = Report::default();
    for page_path in pages {
        let name = page_path
            .file_name()
            .and_then(|n| n.to_str())
            .expect("a page name")
            .to_string();
        let page = format!("{shown}/{name}");
        let text = std::fs::read_to_string(&page_path).unwrap_or_else(|e| panic!("{page}: {e}"));
        for block in blocks(&name, &text) {
            let reading = read(&block.text);
            report.read.push((
                block.page.clone(),
                block.fence,
                reading.excerpt,
                reading.complete,
            ));
            if reading.complete == 0 {
                continue;
            }
            let kind = if reading.excerpt {
                format!(
                    "the excerpt ({} whole form(s) loaded together, {} elided)",
                    reading.complete, reading.elided
                )
            } else {
                "the whole program".to_string()
            };
            let file = scratch.join(format!(
                "{}-L{}.ein",
                name.trim_end_matches(".md"),
                block.fence
            ));
            std::fs::write(&file, &reading.checked).expect("the scratch is writable");
            let file_str = file.to_str().expect("a UTF-8 scratch path");

            let (mut ast, mut terms) = (Ast::new(), Terms::new());
            if let Err(e) = ein_ir::load_file(&mut ast, &mut terms, &file) {
                report.findings.push(format!(
                    "{page}:{}: {kind} does not load:\n{}",
                    block.fence,
                    at_page(&e.to_string(), file_str, &page, &block)
                ));
                continue;
            }
            if reading.excerpt {
                continue;
            }
            let out = Command::new(env!("CARGO_BIN_EXE_ein"))
                .arg("solve")
                .arg(&file)
                .output()
                .expect("the `ein` binary runs");
            if !out.status.success() || out.stdout.is_empty() {
                report.findings.push(format!(
                    "{page}:{}: {kind} loads, and `ein solve` does not answer it ({}):\n{}",
                    block.fence,
                    out.status,
                    at_page(
                        &String::from_utf8_lossy(&out.stderr),
                        file_str,
                        &page,
                        &block
                    )
                ));
            }
        }
    }
    report
}

/// **No `lisp` block in the guide writes what today's engine refuses**, and
/// every whole program in it is answered.
///
/// The guards at the end are the ones `guide_transcripts.rs` keeps against
/// going quiet: a renamed fence would leave this green over nothing, so it
/// asks for a whole program, an excerpt, and chapter 4's block read as an
/// excerpt with forms to load.
#[test]
fn every_program_block_in_the_guide_runs() {
    let scratch = Scratch::new("ein-guide-programs");
    let report = read_guide(&repo_root().join(GUIDE), GUIDE, scratch.path());
    assert!(
        report.findings.is_empty(),
        "{} guide block(s) that today's engine refuses or does not answer. Fix \
         the page, never the engine (S1f.5.26):\n\n{}",
        report.findings.len(),
        report.findings.join("\n\n")
    );
    assert!(
        report.read.iter().any(|&(_, _, excerpt, _)| !excerpt),
        "no whole program was read in {GUIDE}: the reading went quiet"
    );
    assert!(
        report
            .read
            .iter()
            .any(|&(_, _, excerpt, n)| excerpt && n > 0),
        "no excerpt with a whole form was read in {GUIDE}: the reading went quiet"
    );
    assert!(
        report
            .read
            .iter()
            .any(|(page, _, excerpt, n)| page.starts_with("04_") && *excerpt && *n > 0),
        "chapter 4's block, the one finding 6 was about, was not read as an \
         excerpt with forms to load"
    );
}

/// **The reading fails on what it exists to catch**, held on a copy of the
/// guide: chapter 4's block with the retired flag it used to write, an
/// excerpt's form with no `:why`, a whole program's form with a keyword it
/// does not take, and a whole program whose `:expect` the engine refutes,
/// which loads and only the solve can fail. Each is one finding on its page
/// and no other page has one; the first is placed on the page's line.
#[test]
fn a_planted_refusal_in_a_copy_of_the_guide_fails_it() {
    const PLANTS: [(&str, &str); 4] = [
        (
            "04_solving_the_whole_puzzle.md",
            "(config :enable-pre-branch-lookahead true)\n",
        ),
        ("02_first_rules.md", "(relation unexplained T T)\n"),
        (
            "01_objects_and_relations.md",
            "(relation located T T :why \"{?1} is in {?2}\" :priority 100)\n",
        ),
        (
            "03_rule_families.md",
            "(relation p T :why \"p holds of {?1}\")\n(p a)\n\
             (query :goal (p ?x) :expect (model (p b)))\n",
        ),
    ];
    let scratch = Scratch::new("ein-guide-programs-planted");
    let copy = scratch.join("guide");
    let loads = scratch.join("loads");
    std::fs::create_dir_all(&copy).expect("the scratch is writable");
    std::fs::create_dir_all(&loads).expect("the scratch is writable");
    for entry in std::fs::read_dir(repo_root().join(GUIDE)).expect("docs/guide") {
        let path = entry.expect("a directory entry").path();
        std::fs::copy(&path, copy.join(path.file_name().expect("a name"))).expect("a copy");
    }
    let mut planted_at = None;
    for (page, plant) in PLANTS {
        let path = copy.join(page);
        let text = std::fs::read_to_string(&path).expect("the copied page");
        let fence = text
            .find("```lisp\n")
            .unwrap_or_else(|| panic!("{page} has no ```lisp block to plant in"));
        let at = fence + "```lisp\n".len();
        if page.starts_with("04_") {
            planted_at = Some(text[..at].lines().count() + 1);
        }
        let planted = format!("{}{plant}{}", &text[..at], &text[at..]);
        std::fs::write(&path, planted).expect("the copy is writable");
    }

    let report = read_guide(&copy, "copy", &loads);
    for (page, _) in PLANTS {
        let on_page: Vec<&String> = report
            .findings
            .iter()
            .filter(|f| f.starts_with(&format!("copy/{page}:")))
            .collect();
        assert_eq!(
            on_page.len(),
            1,
            "the plant in {page} should be one finding; the findings are:\n\n{}",
            report.findings.join("\n\n")
        );
    }
    assert_eq!(
        report.findings.len(),
        PLANTS.len(),
        "a page with no plant has a finding:\n\n{}",
        report.findings.join("\n\n")
    );
    let finding = |page: &str| {
        report
            .findings
            .iter()
            .find(|f| f.starts_with(&format!("copy/{page}:")))
            .expect("a finding on the page")
            .clone()
    };
    let retired = finding("04_solving_the_whole_puzzle.md");
    let line = planted_at.expect("chapter 4 was planted");
    assert!(
        retired.contains("retired")
            && retired.contains(&format!("copy/04_solving_the_whole_puzzle.md:{line}:")),
        "the retired flag is refused at its line on the page, {line}:\n{retired}"
    );
    assert!(
        finding("02_first_rules.md").contains("has no :why"),
        "the excerpt's relation with no :why is refused for it"
    );
    assert!(
        finding("01_objects_and_relations.md").contains(":priority"),
        "the whole program's keyword is refused"
    );
    let claim = finding("03_rule_families.md");
    assert!(
        claim.contains("`ein solve` does not answer it") && claim.contains("FAILED"),
        "the whole program loads, and its refuted claim fails the solve:\n{claim}"
    );
}
