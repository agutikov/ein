//! **`ein show hyps`**: a program's hypothesis set `H`, listed as Ein facts.
//!
//! The command computes nothing of its own:
//! [`ein_infer::structure::hypotheses`] builds `H` as root's generator
//! builds it and `ein_render::structure` writes a candidate a line. What is
//! held here is the door:
//!
//! 1. **stdout is the facts and nothing else**: one candidate a line, every
//!    candidate, uncut, so it pipes, and what is said about them is one
//!    `note:` on stderr, the rung and the count (after the warning's line on
//!    a program the static check flags, which `non_monotone_warning.rs`
//!    holds);
//! 2. **an empty `H` prints nothing and exits 0**, and the note says why in
//!    the words of the read-out's `universe` row;
//! 3. **a program that is refused is refused** as every command refuses it,
//!    by the loader or by the engine, exit 1, with nothing on stdout;
//! 4. a `.einb` gives its `.ein`'s lines, `ein show` alone is a usage error
//!    as `ein render` alone is, and the command takes no option that could
//!    move `H`.
//!
//! That the lines are the candidates `ein render h-space`'s document lists
//! and `ein solve -H` lists, and that each is a fact a file could hold, is
//! `h_space_view.rs`'s, on the six instances; that they are the same under a
//! permuted id space and under every import form, and cost one saturation
//! and no node, is `ein-infer/tests/structure_invariance.rs`'s.

use std::process::Command;

use ein_corpus::{Scratch, repo_root};

struct Run {
    code: i32,
    out: String,
    err: String,
}

fn ein(args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// The `universe` row of `ein solve FILE --no-search`, its text.
fn universe_row(file: &str) -> String {
    let r = ein(&["solve", file, "--no-search"]);
    assert_eq!(r.code, 0, "{file}: {}", r.err);
    r.out
        .lines()
        .find_map(|l| l.trim_start().strip_prefix("universe"))
        .unwrap_or_else(|| panic!("{file}: no universe row:\n{}", r.out))
        .trim()
        .to_string()
}

/// **stdout is `H` and nothing else, and stderr one note.** Every line is
/// one fact in parentheses, each candidate once, as many as the read-out
/// counts candidates, and the note names the rung and the count; on the
/// program's own rung and on the blind enumerator's, whose listing is past
/// the hundred lines a read-out shares and is not cut.
#[test]
fn stdout_is_the_facts_and_stderr_one_note() {
    for (file, rung, more_than) in [
        ("examples/zebra.ein", "union", 1),
        ("examples/branching/06_lookahead_on.ein", "blind", 100),
    ] {
        let r = ein(&["show", "hyps", file]);
        assert_eq!(r.code, 0, "{file}: {}", r.err);
        let lines: Vec<&str> = r.out.lines().collect();
        assert!(
            lines.len() > more_than,
            "{file}: {} lines: a listing is not cut",
            lines.len()
        );
        assert!(r.out.ends_with('\n') && !r.out.contains("\n\n"), "{file}");
        for line in &lines {
            assert!(
                line.starts_with('(') && line.ends_with(')') && !line.contains(';'),
                "{file}: not a bare fact: {line}"
            );
        }
        let mut once = lines.clone();
        once.sort_unstable();
        once.dedup();
        assert_eq!(once.len(), lines.len(), "{file}: a candidate listed twice");
        assert_eq!(
            r.err,
            format!(
                "note: {file}: H, from `{rung}`: {} candidates\n",
                lines.len()
            )
        );
        // The count is the read-out's.
        let row = universe_row(file);
        assert!(
            row.starts_with(&format!("{} candidates", lines.len())),
            "{file}: {row}"
        );
    }
}

/// **An empty `H` prints nothing and exits 0, and the note says why in the
/// read-out's words**: what follows the rung is the `universe` row of a run
/// that searches nothing, for a program that proposes no candidate and for
/// one that states `(solve :no-search)`.
#[test]
fn an_empty_h_prints_nothing_and_says_why() {
    for (file, rung, why) in [
        (
            "examples/saturation/symmetric/couple.ein",
            "blind",
            "no candidate",
        ),
        (
            "examples/ein-bugs/minimal-line-stated-obligation.ein",
            "no-search",
            "no candidate: the program states :no-search",
        ),
    ] {
        let r = ein(&["show", "hyps", file]);
        assert_eq!(r.code, 0, "{file}: {}", r.err);
        assert!(r.out.is_empty(), "{file}: {}", r.out);
        assert_eq!(universe_row(file), why, "{file}: the read-out's words");
        assert_eq!(
            r.err,
            format!("note: {file}: H, from `{rung}`: {why}\n")
        );
    }
}

/// **A dead root has its `H` all the same**: the generator runs at root's
/// fixpoint whatever it holds, and the listing is what a route would be
/// handed.
#[test]
fn a_dead_roots_h_is_listed() {
    let file = "examples/ein-bugs/wrong-answers/or-one-key.ein";
    let r = ein(&["show", "hyps", file]);
    assert_eq!(r.code, 0, "{}", r.err);
    let n = r.out.lines().count();
    assert!(n > 0, "{file} proposes nothing any more");
    assert!(
        universe_row(file).starts_with(&format!("{n} candidate")),
        "{}",
        universe_row(file)
    );
}

/// **A refused program lists nothing**: one the loader refuses, with the
/// loader's line; a file that is not there. Each exit 1 with an empty stdout.
#[test]
fn a_refused_program_lists_nothing() {
    let file = "examples/broken/load/derivation_cycle.ein";
    let refusal = ein(&["saturate", file]);
    assert_eq!(refusal.code, 1, "{file}: `ein saturate` reads it now");
    let r = ein(&["show", "hyps", file]);
    assert_eq!(r.code, 1, "{file}: {}", r.err);
    assert!(r.out.is_empty(), "{file}: {}", r.out);
    assert_eq!(r.err, refusal.err, "{file}: the refusal's own lines");
    let r = ein(&["show", "hyps", "examples/no-such-file.ein"]);
    assert_eq!(r.code, 1, "{}", r.err);
    assert!(r.out.is_empty());
}

/// **The door**: `ein show` alone is a usage error that names its
/// subcommands, as `ein render` alone is; so are a missing file and an
/// option the command does not take, among them every option of the view
/// that could be thought to move `H`; and a `.einb` gives its `.ein`'s
/// lines, the note echoing the path it was given.
#[test]
fn the_door() {
    let (show, render) = (ein(&["show"]), ein(&["render"]));
    assert_eq!((show.code, render.code), (2, 2));
    assert!(show.err.contains("[subcommands: hyps]"), "{}", show.err);
    assert_eq!(
        show.err.lines().next().map(|l| l.replace("show", "render")),
        render.err.lines().next().map(str::to_string)
    );
    assert_eq!(ein(&["show", "hyps"]).code, 2);
    assert_eq!(ein(&["show", "structure", "examples/zebra2.ein"]).code, 2);
    let file = "examples/zebra2.ein";
    for option in [
        &["--region-budget", "2000"][..],
        &["--view", "json"],
        &["--jobs", "4"],
        &["-o", "out.ein"],
    ] {
        let mut args = vec!["show", "hyps", file];
        args.extend(option);
        let r = ein(&args);
        assert_eq!(r.code, 2, "{option:?}: {}", r.err);
        assert!(r.out.is_empty());
    }

    if !cfg!(feature = "einb") {
        return;
    }
    let plain = ein(&["show", "hyps", file]);
    let tmp = Scratch::new("ein-show-hyps-einb");
    let einb = tmp.path().join("zebra2.einb");
    let einb = einb.to_string_lossy().into_owned();
    let saved = ein(&["kb", "save", file, &einb]);
    assert_eq!(saved.code, 0, "{}", saved.err);
    let held = ein(&["show", "hyps", &einb]);
    assert_eq!(held.code, 0, "{}", held.err);
    assert!(
        held.out == plain.out,
        "the container's H is not the program's"
    );
    assert_eq!(held.err.replace(&einb, file), plain.err);
}

/// **The page's listing is the binary's**: the block
/// `docs/ein/ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md` shows under *H, listed* is
/// the first three lines `ein show hyps examples/zebra2.ein` prints. A
/// listing that moves fails here with the lines to paste.
#[test]
fn the_pages_listing_is_the_binarys() {
    const BEGIN: &str =
        "<!-- generated: ein show hyps examples/zebra2.ein | head -3 -->\n```text\n";
    const END: &str = "\n```\n<!-- /generated -->";
    let text = std::fs::read_to_string(
        repo_root().join("docs/ein/ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md"),
    )
    .expect("docs/ein/ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md");
    let start = text.find(BEGIN).expect("the page's generated block") + BEGIN.len();
    let end = start + text[start..].find(END).expect("the block's end");
    let r = ein(&["show", "hyps", "examples/zebra2.ein"]);
    assert_eq!(r.code, 0, "{}", r.err);
    let head: Vec<&str> = r.out.lines().take(3).collect();
    assert!(
        text[start..end] == head.join("\n"),
        "docs/ein/ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md's listing is not the binary's. Paste:\n\n{}",
        head.join("\n")
    );
}
