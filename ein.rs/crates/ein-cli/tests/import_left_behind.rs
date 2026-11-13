//! **One line where a selection brings a relation's declaration and no rule
//! that derives it** — M1f S1f.6.25, E-20 of P1f.6's `findings.md`
//! (Q-M1f.217).
//!
//! A theory built on a theory: `upper` imports `lower` by `lower`'s entry and
//! a program imports `upper` by `upper`'s. The closure of that selection
//! follows the names `upper`'s rules write, the relations of `lower` among
//! them, and reaches none of `lower`'s rules unless `upper`'s entry names
//! `lower`'s. That is the closure rule as `01_grammar.md` § Import states it,
//! and it is not changed. But nothing said so: no request was answered, the
//! lower theory's relations were open, and the symptom, hundreds of
//! candidates, was far from the cause. It was met three times in P1f.6.
//!
//! The loader now says it, **a diagnostic and not a refusal**: one line on
//! stderr, on every command that loads the program, once; the program loads
//! and answers as it did, and no exit code moves. The condition was chosen on
//! a census of every import in the corpus (the stage's record): quiet on
//! every tracked program, loud on the three witnesses.
//!
//! The witness is `tests/imports/02_nested_import_left_behind.ein`,
//! `bob1_nested_import.ein` of S1f.6.20's prototype rebuilt small; its twin,
//! `01_nested_import_whole.ein`, imports the same theory with the entry name
//! in place and prints nothing.

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

fn ein(args: &[&str]) -> Run {
    ein_in(&repo_root(), args)
}

const WITNESS: &str = "tests/imports/02_nested_import_left_behind.ein";
const TWIN: &str = "tests/imports/01_nested_import_whole.ein";

/// The line, whole.
const LINE: &str = "note: tests/imports/02_nested_import_left_behind.ein:25:9: (import \
    upper_without_entry :symbols (upper)) brings the declaration of double-of and no rule that \
    derives it: upper_without_entry has such a rule, its own or one it imports, and the \
    selection names none — a selection brings the names it lists and what their declarations \
    write, so list the rule, or the entry that names it, if double-of is to be derived";

/// The lines of `stderr` that are the loader's about a selection.
fn left_behind(stderr: &str) -> Vec<&str> {
    stderr
        .lines()
        .filter(|l| l.starts_with("note: ") && l.contains(" brings the declaration"))
        .collect()
}

/// Every command that loads a program, as the argv before the path.
const COMMANDS: [&[&str]; 14] = [
    &["saturate"],
    &["saturate", "--dump"],
    &["solve"],
    &["solve", "--minimal"],
    &["solve", "--maximal"],
    &["solve", "--single"],
    &["solve", "--count"],
    &["solve", "-n", "2"],
    &["oracle"],
    &["test"],
    &["render", "kb"],
    &["render", "lattice"],
    &["render", "program"],
    &["render", "constraints"],
];

/// The witness prints the line once on every command that loads it, whatever
/// the command loads it for and however often, and the command exits as it
/// does on the twin, which prints none.
#[test]
fn the_line_is_printed_once_on_every_command_and_moves_no_exit() {
    for cmd in COMMANDS {
        let mut a: Vec<&str> = cmd.to_vec();
        a.push(WITNESS);
        let w = ein(&a);
        assert_eq!(left_behind(&w.stderr), vec![LINE], "{cmd:?}\n{}", w.stderr);
        let mut b: Vec<&str> = cmd.to_vec();
        b.push(TWIN);
        let t = ein(&b);
        assert!(left_behind(&t.stderr).is_empty(), "{cmd:?}\n{}", t.stderr);
        assert_eq!(
            (w.code, t.code),
            (0, 0),
            "{cmd:?}\n{}{}",
            w.stderr,
            t.stderr
        );
    }
}

/// It is a diagnostic: the witness answers what it answered. The request is
/// derived and never answered, so the goal has no row, and the claims the
/// file states of that hold; the twin's request is answered.
#[test]
fn the_program_loads_and_answers_as_it_did() {
    let w = ein(&["solve", WITNESS]);
    assert_eq!(w.code, 0, "{}", w.stderr);
    assert!(
        w.stdout.contains("the goal has no row: undetermined here"),
        "{}",
        w.stdout
    );
    let d = ein(&["saturate", "--dump", WITNESS]);
    assert!(d.stdout.contains("(ask a :rule want)"), "{}", d.stdout);
    assert!(!d.stdout.contains("(double-of a aa"), "{}", d.stdout);
    assert!(!d.stdout.contains("(rule answer"), "{}", d.stdout);
    let t = ein(&["saturate", "--dump", TWIN]);
    assert!(t.stdout.contains("(got a aa :rule read)"), "{}", t.stdout);
    let all = ein(&["test", "tests/imports"]);
    assert_eq!(all.code, 0, "{}{}", all.stdout, all.stderr);
    assert_eq!(left_behind(&all.stderr), vec![LINE], "{}", all.stderr);
}

/// **What it is quiet on**, each a program written here, and one it is loud
/// on in each of its forms.
#[test]
fn the_condition() {
    let dir = Scratch::new("ein-import-left-behind");
    let write = |name: &str, text: &str| {
        std::fs::write(dir.path().join(name), text).expect("written");
    };
    // A library of facts has no rule to leave behind.
    write(
        "facts.ein",
        "(import std.digits :symbols (fa nz __closed__))\n(solve :hypothesis-relations ())\n",
    );
    // A theory: one relation its rule derives, one it reads.
    write(
        "theory.ein",
        "(relation asked Thing :why \"{?1} is asked\")\n\
         (relation told Thing :why \"{?1} is told\")\n\
         (relation heard Thing :why \"{?1} is heard\")\n\
         (rule tell () :match (asked ?x) :assert (and (told ?x) (not (heard ?x))) :why \"t\")\n\
         (rule echo () :match (told ?x) :assert (noted (told ?x)) :why \"e\")\n\
         (rule entry () :match () :assert (__closed__ asked) :why \"the entry\")\n",
    );
    // A relation listed by name is asked for on purpose, with its rule left.
    write(
        "listed.ein",
        "(import theory :symbols (told))\n(solve :hypothesis-relations ())\n",
    );
    // The rule that derives it kept: nothing is left behind.
    write(
        "kept.ein",
        "(import theory :symbols (echo tell))\n(solve :hypothesis-relations ())\n",
    );
    // A rule of the program's own derives it.
    write(
        "own.ein",
        "(import theory :symbols (echo))\n\
         (rule mine () :match (seen ?x) :assert (told ?x) :why \"m\")\n\
         (solve :hypothesis-relations ())\n",
    );
    // Another import brings the rule.
    write(
        "other.ein",
        "(import theory :symbols (echo))\n(import theory :symbols (tell))\n\
         (solve :hypothesis-relations ())\n",
    );
    // A qualified import brings every rule.
    write(
        "qualified.ein",
        "(import theory :as t)\n(solve :hypothesis-relations ())\n",
    );
    for quiet in [
        "facts.ein",
        "theory.ein",
        "listed.ein",
        "kept.ein",
        "own.ein",
        "other.ein",
        "qualified.ein",
    ] {
        let r = ein_in(dir.path(), &["saturate", quiet]);
        assert_eq!(r.code, 0, "{quiet}: {}", r.stderr);
        assert!(left_behind(&r.stderr).is_empty(), "{quiet}: {}", r.stderr);
    }
    // `echo` reads `told`, whose declaration its closure brings, and `tell`,
    // which derives it, stays behind: one relation. A relation written
    // inside a conclusion, `(noted (told ?x))`, is a value and no conclusion.
    write(
        "one.ein",
        "(import theory :symbols (echo))\n(solve :hypothesis-relations ())\n",
    );
    let r = ein_in(dir.path(), &["saturate", "one.ein"]);
    assert_eq!(r.code, 0, "{}", r.stderr);
    assert_eq!(
        left_behind(&r.stderr),
        vec![
            "note: one.ein:1:9: (import theory :symbols (echo)) brings the declaration of told \
             and no rule that derives it: theory has such a rule, its own or one it imports, and \
             the selection names none — a selection brings the names it lists and what their \
             declarations write, so list the rule, or the entry that names it, if told is to be \
             derived"
        ],
    );
    // Two relations, and more: the line stays one line.
    write(
        "wide.ein",
        "(relation a Thing :why \"a {?1}\")\n(relation b Thing :why \"b {?1}\")\n\
         (relation c Thing :why \"c {?1}\")\n(relation d Thing :why \"d {?1}\")\n\
         (rule make () :match (seed ?x) :assert (and (a ?x) (b ?x) (c ?x) (d ?x)) :why \"m\")\n\
         (rule two () :match (and (a ?x) (b ?x)) :assert (two ?x) :why \"2\")\n\
         (rule four () :match (and (a ?x) (b ?x) (c ?x) (d ?x)) :assert (four ?x) :why \"4\")\n",
    );
    for (rule, what, them) in [
        ("two", "the declarations of a and b", "either"),
        (
            "four",
            "the declarations of a and 3 more relations",
            "any of them",
        ),
    ] {
        write(
            "w.ein",
            &format!("(import wide :symbols ({rule}))\n(solve :hypothesis-relations ())\n"),
        );
        let r = ein_in(dir.path(), &["saturate", "w.ein"]);
        let lines = left_behind(&r.stderr);
        assert_eq!(lines.len(), 1, "{}", r.stderr);
        assert!(
            lines[0].contains(&format!(
                "(import wide :symbols ({rule})) brings {what} and no rule that derives {them}: "
            )),
            "{}",
            lines[0]
        );
    }
}

/// A container is written from the loaded program, so saving one says the
/// line, and opening one does not: the program it stores has no import left
/// to read (E-42 of P1f.6's `findings.md`).
#[cfg(feature = "einb")]
#[test]
fn a_container_says_it_when_saved_and_not_when_opened() {
    let dir = Scratch::new("ein-import-left-behind-einb");
    let out = dir.path().join("w.einb");
    let out = out.to_str().expect("utf-8");
    let saved = ein(&["kb", "save", WITNESS, out]);
    assert_eq!(saved.code, 0, "{}", saved.stderr);
    assert_eq!(left_behind(&saved.stderr), vec![LINE], "{}", saved.stderr);
    let opened = ein(&["solve", out]);
    assert_eq!(opened.code, 0, "{}", opened.stderr);
    assert!(left_behind(&opened.stderr).is_empty(), "{}", opened.stderr);
}
