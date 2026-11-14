//! **Imports by full name, through the binary** — M1f S1f.6.35's acceptance
//! (the stage file's § Acceptance), on the user's model (h) and its ruling of
//! 2026-11-14, *"in all cases we import everything, the difference only in
//! names"*.
//!
//! | claim | test |
//! |---|---|
//! | a program that lists `add`, `sum-of` and `n` and no entry rule computes | [`the_calculator_by_the_names_a_program_writes`] |
//! | a program's own `done`, `b` or rule `add-column` beside the calculator loads and is unaffected | [`a_programs_own_names_beside_the_calculator`] |
//! | `:as` makes no copy: the calculator listed and aliased holds one set of tables | [`an_alias_makes_no_copy`] |
//! | nothing is left behind: E-20's witness answers, and no command prints a `note:` | [`nothing_is_left_behind`] |
//! | a refusal of (h) is one `kb load error:` line at the command's load-error exit | [`a_refusal_is_one_line`] |
//!
//! The resolver's own cases are `ein-ir`'s `imports_full_names.rs`; that every
//! corpus program answers alike under every import form is
//! `import_invariance.rs`.

use std::path::Path;
use std::process::Command;

use ein_corpus::{Scratch, repo_root};

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

/// A scratch program, `p.ein`, beside `modules`.
fn program(tag: &str, src: &str, modules: &[(&str, &str)]) -> Scratch {
    let s = Scratch::new(tag);
    for (name, text) in modules {
        std::fs::write(s.path().join(format!("{name}.ein")), text).expect("written");
    }
    std::fs::write(s.path().join("p.ein"), src).expect("written");
    s
}

#[test]
fn the_calculator_by_the_names_a_program_writes() {
    let s = program(
        "ein-full-acc",
        "(import std.arith :symbols (add sum-of n))\n\
         (add (n z 2) (n z 3))\n\
         (query :goal (sum-of ?x ?y ?s) :expect (model (sum-of (n z 2) (n z 3) (n z 5))))\n",
        &[],
    );
    let t = ein_in(s.path(), &["test", "p.ein"]);
    assert_eq!(t.code, 0, "{}{}", t.stdout, t.stderr);
    assert!(t.stdout.contains("1 held"), "{}", t.stdout);
    // The circuit's own steps print by their full names, the program's by
    // its short ones.
    let r = ein_in(s.path(), &["solve", "p.ein", "--trace", "trace.md"]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    let trace = std::fs::read_to_string(s.path().join("trace.md")).expect("a trace");
    assert!(trace.contains("`std.arith.add-ask`"), "the circuit's rule");
    assert!(
        trace.contains("sum-of(n(z, 2), n(z, 3), n(z, 5))"),
        "the program's name"
    );
}

#[test]
fn a_programs_own_names_beside_the_calculator() {
    for own in [
        "(relation done Task :why \"{?1} is done\")\n(__closed__ done)\n(done laundry)\n\
         (query :goal (done ?t) :expect (model (done laundry)))\n",
        "(relation b T :why \"{?1} is b\")\n(__closed__ b)\n(b one)\n\
         (query :goal (b ?x) :expect (model (b one)))\n",
        "(import std.digits :symbols (fa))\n\
         (relation column Col Digit Digit :why \"column {?1} adds {?2} and {?3}\")\n\
         (relation col Col Digit Digit :why \"column {?1} gives {?2}, carry {?3}\")\n\
         (__closed__ column) (__closed__ col)\n(column units 7 5)\n\
         (rule add-column () :match (and (column ?c ?a ?b) (fa ?a ?b 0 ?d ?k)) \
          :assert (col ?c ?d ?k) :why \"{?a} + {?b} is {?d}, carry {?k}\")\n\
         (query :goal (col ?c ?d ?k) :expect (model (col units 2 1)))\n",
    ] {
        let src = format!(
            "(import std.arith :symbols (arith add n sum-of))\n(add (n z 7) (n z 5))\n{own}\
             (query :goal (sum-of ?x ?y ?s) :expect (model (sum-of (n z 7) (n z 5) (n (n z 1) 2))))\n"
        );
        let s = program("ein-full-own", &src, &[]);
        let t = ein_in(s.path(), &["test", "p.ein"]);
        assert_eq!(t.code, 0, "{src}\n{}{}", t.stdout, t.stderr);
        assert!(t.stdout.contains("2 held"), "{src}\n{}", t.stdout);
    }
}

#[test]
fn an_alias_makes_no_copy() {
    let facts = |src: &str| -> String {
        let s = program("ein-full-copy", src, &[]);
        let r = ein_in(s.path(), &["saturate", "p.ein"]);
        assert_eq!(r.code, 0, "{}", r.stderr);
        r.stdout
            .lines()
            .find(|l| l.trim_start().starts_with("facts (total)"))
            .expect("a facts line")
            .split_whitespace()
            .nth(2)
            .expect("a count")
            .to_string()
    };
    let once = facts("(import std.arith :symbols (arith add n))\n(add (n z 2) (n z 3))\n");
    let twice = facts(
        "(import std.arith :symbols (arith add n))\n(import std.arith :as a)\n\
         (add (n z 2) (n z 3))\n",
    );
    assert_eq!(once, twice, "an alias brings no second calculator");
}

#[test]
fn nothing_is_left_behind() {
    let root = repo_root();
    let witness = "tests/imports/02_nested_import_left_behind.ein";
    let t = ein_in(&root, &["test", witness]);
    assert_eq!(t.code, 0, "{}{}", t.stdout, t.stderr);
    for cmd in [
        &["solve", witness][..],
        &["saturate", witness],
        &["test", witness],
        &["render", "program", witness],
    ] {
        let r = ein_in(&root, cmd);
        assert!(
            !r.stderr.contains("note:"),
            "{cmd:?} printed a note: {}",
            r.stderr
        );
    }
}

#[test]
fn a_refusal_is_one_line() {
    let s = program(
        "ein-full-refused",
        "(import m :symbols (done))\n(import n :symbols (done))\n",
        &[
            ("m", "(relation done T :why \"d\")\n"),
            ("n", "(relation done T :why \"n's\")\n"),
        ],
    );
    for (cmd, code) in [("solve", 1), ("saturate", 1), ("test", 2)] {
        let r = ein_in(s.path(), &[cmd, "p.ein"]);
        assert_eq!(r.code, code, "{cmd}: {}", r.stderr);
        let lines: Vec<&str> = r
            .stderr
            .lines()
            .filter(|l| l.contains("kb load error"))
            .collect();
        assert_eq!(lines.len(), 1, "{cmd}: {}", r.stderr);
        assert!(
            lines[0]
                .contains("'done' is bound to two full names in the program, m.done and n.done"),
            "{cmd}: {}",
            lines[0]
        );
    }
}
