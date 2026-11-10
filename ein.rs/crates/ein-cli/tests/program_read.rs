//! **Every command reads a program the same way** — M1f S1f.5.5, T1f.5.5.13.
//!
//! One path reads a program from a path, `common::read_program`, and every
//! command that takes one goes through it: a `.einb` container is told from
//! UTF-8 text by its first bytes, a text is loaded against its own directory
//! so a file-relative `(import …)` resolves, and a file that is neither is
//! refused with one line in Ein's words. Until that task `ein saturate`, the
//! `render` views that parse a program and `ein kb save` read text only (W40),
//! `saturate` loaded with no base directory (W41), and the line for a file
//! that is not UTF-8 was Python's, `UnicodeDecodeError: …`, from a
//! `process::exit` that took `ein test`'s whole selection down, exit 1.
//!
//! Each command is held to four programs: the same `.ein`, its `.einb`, a
//! program with a file-relative import, and a file that is not UTF-8. It
//! answers the first three, the container as it answers the `.ein`, and
//! refuses the fourth with the same line and its own load-error exit (1, and 2
//! for `ein test`, whose load error is 2).
//!
//! Gated on `einb`, as `einb_cli.rs` is: without it `ein kb` does not exist
//! and a container is refused by every command, one line each.

#![cfg(feature = "einb")]

use std::path::Path;
use std::process::Command;

use ein_corpus::Scratch;

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

fn ein_in(dir: &Path, args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// One rule and one fact, so that `saturate` fires, `render rules` and
/// `render rule --name step` draw, and `ein test` has a claim to check.
const PROGRAM: &str = "\
(relation p T :why \"{?1} is a p\")
(relation q T :why \"{?1} is a q\")
(p a)
(rule step () :match (p ?x) :assert (q ?x) :why \"{?x} is a p, so a q\")
(query :goal (q ?x) :expect (model (q a)))
";

/// Every command that takes a program, as the argv before and after the
/// path, and the exit a load error takes there. `{out}` is a fresh file in
/// the scratch directory.
const COMMANDS: [(&[&str], &[&str], i32); 12] = [
    (&["saturate"], &["--dump"], 1),
    (&["solve"], &[], 1),
    (&["solve", "--maximal"], &[], 1),
    (&["oracle"], &[], 1),
    (&["test"], &[], 2),
    (&["render", "rules"], &[], 1),
    (&["render", "rule"], &["--name", "step"], 1),
    (&["render", "constraints"], &[], 1),
    (&["render", "program"], &[], 1),
    (&["render", "lattice"], &[], 1),
    (&["render", "kb"], &[], 1),
    (&["kb", "save"], &["{out}"], 1),
];

fn argv<'a>(
    cmd: &(&'a [&'a str], &'a [&'a str], i32),
    file: &'a str,
    out: &'a str,
) -> Vec<&'a str> {
    let mut v: Vec<&str> = cmd.0.to_vec();
    v.push(file);
    v.extend(cmd.1.iter().map(|a| if *a == "{out}" { out } else { *a }));
    v
}

/// What two runs of one command are compared on: the path each was handed
/// aside, which a header or a label echoes, and the lines that say what the
/// run read and how long it took (`saturate`'s `input:`, its size, `parse:`,
/// `kb load:` or `open:`), each number a line times masked: one before `ms`,
/// and `ein test`'s `(0.01 s)`, which T1f.5.5.13's first gate read as the
/// same twice by chance and T7's did not.
fn comparable(text: &str, path: &str) -> String {
    let mut out = Vec::new();
    for line in text.replace(path, "{FILE}").lines() {
        let t = line.trim_start();
        let size = t.ends_with(" chars") || t.ends_with(" bytes, a .einb container");
        if ["input:", "parse:", "kb load:", "open:"]
            .iter()
            .any(|p| t.starts_with(p))
            || (size && t.starts_with(|c: char| c.is_ascii_digit()))
            || t.starts_with("wall ")
        {
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        let masked: Vec<&str> = words
            .iter()
            .enumerate()
            .map(|(i, w)| {
                let timed = words
                    .get(i + 1)
                    .is_some_and(|n| matches!(*n, "ms" | "s" | "s)"));
                if timed && w.trim_start_matches('(').parse::<f64>().is_ok() {
                    "#"
                } else {
                    w
                }
            })
            .collect();
        out.push(masked.join(" "));
    }
    out.join("\n")
}

/// The four programs, written to a scratch directory: `w.ein`, its `w.einb`,
/// `sub/main.ein` importing `sub/mod.ein` by its file-relative name, and
/// `bad.ein`, whose byte at line 2, column 4 is not UTF-8.
fn programs(dir: &Path) {
    std::fs::write(dir.join("w.ein"), PROGRAM).expect("written");
    let saved = ein_in(dir, &["kb", "save", "w.ein", "w.einb"]);
    assert_eq!(saved.code, 0, "kb save: {}", saved.stderr);
    std::fs::create_dir_all(dir.join("sub")).expect("a directory");
    std::fs::write(
        dir.join("sub/mod.ein"),
        "(relation q T :why \"{?1} is a q\")\n",
    )
    .expect("written");
    std::fs::write(
        dir.join("sub/main.ein"),
        "(import mod :symbols (q))\n\
         (relation p T :why \"{?1} is a p\")\n\
         (p a)\n\
         (rule step () :match (p ?x) :assert (q ?x) :why \"{?x} is a p, so a q\")\n\
         (query :goal (q ?x) :expect (model (q a)))\n",
    )
    .expect("written");
    std::fs::write(dir.join("bad.ein"), b"(relation p T)\n(p \xff)\n").expect("written");
}

#[test]
fn every_command_answers_a_program_its_container_and_a_file_relative_import() {
    let scratch = Scratch::new("ein-program-read");
    let dir = scratch.path();
    programs(dir);
    let mut bad = Vec::new();
    for (i, cmd) in COMMANDS.iter().enumerate() {
        let out_ein = format!("out-{i}-ein.einb");
        let out_einb = format!("out-{i}-einb.einb");
        let out_imp = format!("out-{i}-imp.einb");
        let plain = ein_in(dir, &argv(cmd, "w.ein", &out_ein));
        let boxed = ein_in(dir, &argv(cmd, "w.einb", &out_einb));
        // Run from the scratch root, so the import resolves against the
        // file's own directory and not the working one.
        let imported = ein_in(dir, &argv(cmd, "sub/main.ein", &out_imp));
        let name = cmd.0.join(" ");
        for (what, r) in [
            ("w.ein", &plain),
            ("w.einb", &boxed),
            ("sub/main.ein", &imported),
        ] {
            if r.code != 0 {
                bad.push(format!(
                    "`ein {name}` on {what}: exit {}, stderr: {}",
                    r.code,
                    r.stderr.lines().next().unwrap_or("<empty>")
                ));
            }
        }
        if comparable(&plain.stdout, "w.ein") != comparable(&boxed.stdout, "w.einb") {
            bad.push(format!(
                "`ein {name}`: the container's stdout is not the program's:\n--- w.ein\n{}\n--- w.einb\n{}",
                comparable(&plain.stdout, "w.ein"),
                comparable(&boxed.stdout, "w.einb")
            ));
        }
    }
    // `kb save` of a container saves it again: the two containers answer as
    // the program does.
    let resaved = ein_in(dir, &["solve", "out-11-einb.einb"]);
    let saved = ein_in(dir, &["solve", "out-11-ein.einb"]);
    assert_eq!(resaved.code, 0, "{}", resaved.stderr);
    assert_eq!(
        comparable(&resaved.stdout, "out-11-einb.einb"),
        comparable(&saved.stdout, "out-11-ein.einb"),
        "a container saved again does not answer as the one saved from the program"
    );
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

#[test]
fn every_command_refuses_a_file_that_is_not_utf8_with_one_line() {
    let scratch = Scratch::new("ein-program-read");
    let dir = scratch.path();
    programs(dir);
    let want = "bad.ein:2:4: byte 0xff is not UTF-8: a program is UTF-8 text, or a .einb container";
    let mut bad = Vec::new();
    for (i, cmd) in COMMANDS.iter().enumerate() {
        let out = format!("out-{i}.einb");
        let r = ein_in(dir, &argv(cmd, "bad.ein", &out));
        let name = cmd.0.join(" ");
        if r.code != cmd.2 {
            bad.push(format!(
                "`ein {name}`: exit {}, its load error's is {}",
                r.code, cmd.2
            ));
        }
        if r.stderr.lines().next() != Some(want) {
            bad.push(format!("`ein {name}`: stderr {:?}", r.stderr));
        }
    }
    // `ein test` counts the file an error and goes on to the next one, where
    // the read's `process::exit` stopped the whole selection, exit 1.
    let r = ein_in(dir, &["test", "bad.ein", "w.ein"]);
    assert_eq!(r.code, 2, "{}{}", r.stdout, r.stderr);
    assert!(
        r.stdout.contains("2 files, 1 expectation: 1 held"),
        "the selection stopped at the file it could not read:\n{}",
        r.stdout
    );
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// A container's stored program is parsed under a name that says so, which
/// every position in it carries: `<program stored in x.einb>` and the stored
/// text's line, where it was `<string>` (or the first file name the
/// provenance table held) and the same line (Q-M1f.15).
///
/// The program is one whose error names a position: a macro whose template
/// writes `eq` with one argument (`02_patterns.md`). Since M1f S1f.5.24 the
/// loader refuses it, `g.ein:3:18:` first, the macro body's position (W66,
/// Q-M1f.80), so `ein kb save` does too, and the container is written here
/// through `ein_einb::save` with that program stored beside a KB that loads:
/// what a command reads back from a container is its stored program, which it
/// loads as it loads a `.ein`. Until S1f.5.24 `kb save` wrote it, the compiler
/// refusing the rule only where it compiled it (Q-M1f.67); until M1f S1f.5.18
/// this test wrote G-14's residue, a relation parameter an activator a rule
/// derives binds to a nested fact, which the loader refuses since (W59); and
/// until M1f S1f.5.9 a pattern whose relation a parameter binds to `eq`,
/// `(?R ?x)` under `(r eq)`, which the loader refuses since (W60). Since M1f
/// S1f.5.21 the line names the position in Ein's words, `g.ein:3:18:`, where
/// it wrote `at Loc(file='g.ein', line=3, col=18)`.
#[test]
fn a_containers_positions_name_its_stored_program() {
    let scratch = Scratch::new("ein-program-read");
    let dir = scratch.path();
    let text = "(relation p Node :why \"p holds of {?1}\") (relation hit Node :why \"{?1} is hit\")\n\
                (p a)\n\
                (macro one (?a) (eq ?a) :why \"the equality (eq {?a}), written with one argument\")\n\
                (rule r () :match (and (p ?x) (one ?x)) :assert (hit ?x) :why \"p holds of {?x} and the pattern one holds of {?x}, so {?x} is hit\")\n\
                (query :goal (hit ?x))\n";
    std::fs::write(dir.join("g.ein"), text).expect("written");
    let line = "rule 'r': its :match writes (eq ?x), and eq takes";
    let plain = ein_in(dir, &["saturate", "g.ein"]);
    assert!(
        plain
            .stderr
            .starts_with(&format!("kb load error: g.ein:3:18: {line}")),
        "{}",
        plain.stderr
    );
    let saved = ein_in(dir, &["kb", "save", "g.ein", "g.einb"]);
    assert_eq!(saved.code, 1, "kb save refuses what the loader refuses");
    assert!(
        !dir.join("g.einb").exists(),
        "a container of a refused program"
    );

    // The stored program is the refused one; the KB beside it is a program's
    // that loads.
    let mut ast = ein_ir::Ast::new();
    let mut terms = ein_core::Terms::new();
    let forms = ein_ir::parse(&mut ast, text, Some("g.ein")).expect("it parses");
    let held = ein_ir::parse(
        &mut ast,
        "(relation p Node :why \"p holds of {?1}\") (p a)",
        Some("k.ein"),
    )
    .expect("it parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &held, None).expect("it loads");
    ein_einb::save(
        &dir.join("g.einb"),
        &kb,
        &terms,
        &mut ast,
        &forms,
        Some(dir),
        &ein_einb::SaveOptions::default(),
    )
    .expect("a container is written");
    for args in [&["saturate", "g.einb"][..], &["solve", "g.einb"]] {
        let r = ein_in(dir, args);
        assert_eq!(r.code, 1, "{args:?}: {}", r.stderr);
        // The position is the stored text's, as the loader read it back
        // (Q-M1f.15), so its line and column are that text's; a stored
        // program the loader refuses is said to be one.
        assert!(
            r.stderr.starts_with(
                "kb load error: .einb PROGRAM does not load: <program stored in g.einb>:"
            ) && r.stderr.contains(&format!(": {line}")),
            "{args:?}: {}",
            r.stderr
        );
        assert!(!r.stderr.contains("<string>"), "{args:?}: {}", r.stderr);
    }
}
