//! **A fact of no argument prints as it is written, `(r)`, on every surface.**
//!
//! A fact's text is the one printer's (`ein_core::Terms::fact_text`): its
//! relation and its arguments one space apart between parentheses, and with
//! no argument the relation alone, `(r)`, with no space inside. Every surface
//! that prints a fact as Ein text prints that text, so one fact reads the
//! same wherever a run shows it, and what a surface prints loads back as the
//! fact.
//!
//! The witness is a program whose every fact has no argument: one given with
//! a source, one a candidate of H, one derived from the two. The surfaces
//! read here are the read-out and its four forms, the saturated dump, `ein
//! show hyps`, the oracle and its dump, the event stream, the trace, the
//! answer dump, `ein test`'s diff, the `stale` row of a flagged program, and
//! the loader's refusal of a derivation cycle.

use std::path::Path;
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

fn write(scratch: &Scratch, name: &str, text: &str) -> String {
    let file = scratch.path().join(name);
    std::fs::write(&file, text).expect("written");
    file.display().to_string()
}

/// Every file under `dir`, each with its text, in path order.
fn files(dir: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|e| e.expect("an entry").path())
        .collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            out.extend(files(&p));
        } else {
            let text = std::fs::read_to_string(&p).expect("text");
            out.push((p.display().to_string(), text));
        }
    }
    out
}

/// The relations of the witnesses, none with an argument.
const RELATIONS: [&str; 6] = ["wired", "on", "lit", "alive", "p", "q"];

/// No text holds a fact of no argument with a space inside, `(r )`.
fn no_space_inside(surface: &str, text: &str) {
    for r in RELATIONS {
        let spaced = format!("({r} )");
        assert!(
            !text.contains(&spaced),
            "{surface} prints {spaced}, a fact of no argument with a space inside:\n{text}"
        );
    }
}

/// `text` holds each of `facts` as it is written.
fn holds(surface: &str, text: &str, facts: &[&str]) {
    no_space_inside(surface, text);
    for f in facts {
        assert!(text.contains(f), "{surface} does not print {f}:\n{text}");
    }
}

/// The lines a form prints under a state's `final-state facts` label.
fn form_facts(read_out: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut inside = false;
    for l in read_out.lines() {
        if l.starts_with("final-state facts") {
            inside = true;
        } else if inside && l.starts_with("  (") {
            out.push(l.trim());
        } else {
            inside = false;
        }
    }
    out
}

/// A given with a source, a candidate, and a fact derived from the two.
const LAMP: &str = r#"(relation wired :why "the house is wired")
(relation on :why "the switch is on")
(relation lit :why "the lamp is lit")
(wired :source "the electrician's note")
(rule lights ()
  :match  (and (wired) (on))
  :assert (lit)
  :why    "a wired house with its switch on is lit.")
(hrule guess ()
  :match  ()
  :assert (on)
  :why    "suppose the switch is on.")
(query :goal (lit))
(solve :expect (region (model) (model (on))))
"#;

/// **Every surface prints `(on)` and `(lit)`, and none `(on )` or `(lit )`.**
/// The form `-p` lists root's `open` fact and state 2's facts, its `open`
/// fact among them, `-P` state 2's two facts with no `open` one
/// (Q-M1f.577), `-f` its one fact of the
/// candidates' relation, each line the fact's text; the saturated dump writes
/// a fact with its clause after the relation, `(wired :source …)`, and its
/// declaration as `(relation wired)`.
#[test]
fn every_surface_prints_a_fact_of_no_argument_as_written() {
    let scratch = Scratch::new("ein-fact-of-no-argument");
    let file = write(&scratch, "lamp.ein", LAMP);

    let plain = ein(&["solve", &file]);
    assert_eq!(plain.code, 0, "{}", plain.err);
    holds("ein solve", &plain.out, &["(model (on))", "(model (lit))"]);

    for (flag, facts) in [
        ("-p", &["(open (on))", "(lit)", "(on)", "(open (on))"][..]),
        ("-P", &["(lit)", "(on)"][..]),
        ("-f", &["(on)"][..]),
    ] {
        let r = ein(&["solve", &file, flag]);
        assert_eq!(r.code, 0, "{}", r.err);
        no_space_inside(&format!("ein solve {flag}"), &r.out);
        assert_eq!(form_facts(&r.out), facts, "ein solve {flag}:\n{}", r.out);
    }
    let h = ein(&["solve", &file, "-H"]);
    holds("ein solve -H", &h.out, &["    (on)"]);

    let dump = ein(&["saturate", &file, "--dump"]);
    assert_eq!(dump.code, 0, "{}", dump.err);
    holds(
        "ein saturate --dump",
        &dump.out,
        &["(relation wired)", "  (wired :source "],
    );

    let hyps = ein(&["show", "hyps", &file]);
    assert_eq!(hyps.out, "(on)\n", "ein show hyps: {}", hyps.err);

    let cube = scratch.path().join("cube");
    let oracle = ein(&[
        "oracle",
        &file,
        "--dump",
        &cube.display().to_string(),
        "--states",
    ]);
    assert_eq!(oracle.code, 0, "{}", oracle.err);
    holds("ein oracle", &oracle.out, &["(model (on))"]);
    let dumped = files(&cube);
    let all: String = dumped.iter().map(|(_, t)| t.as_str()).collect();
    holds("ein oracle --dump --states", &all, &["(on)", "(lit)"]);

    let events = scratch.path().join("events.jsonl");
    let trace = scratch.path().join("trace.md");
    let answer = scratch.path().join("answer");
    let r = ein(&[
        "solve",
        &file,
        "--events",
        &events.display().to_string(),
        "--trace",
        &trace.display().to_string(),
        "--dump-answer",
        &answer.display().to_string(),
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    holds(
        "--events",
        &std::fs::read_to_string(&events).expect("the event stream"),
        &["\"(wired)\"", "\"(on)\"", "\"(lit)\""],
    );
    holds(
        "--trace",
        &std::fs::read_to_string(&trace).expect("the trace"),
        &["(on)"],
    );
    for (path, text) in files(&answer) {
        no_space_inside(&path, &text);
    }
    let saved = std::fs::read_to_string(answer.join("models/02.ein")).expect("state 2's file");
    holds("a saved state", &saved, &["(commit\n  (on))"]);

    let test = ein(&["test", &file, "--diff", "full"]);
    assert_eq!(test.code, 0, "{}{}", test.out, test.err);
    no_space_inside("ein test --diff full", &test.out);
}

/// A flagged program: `(alive)` is derived at root under an `absent`, which
/// the one candidate's node falsifies, so the node holds it stale.
const STALE: &str = r#"(relation on :why "the switch is on")
(relation alive :why "the state is alive")
(rule no-switch ()
  :match  (on)
  :assert (false)
  :why    "the switch is never on.")
(hrule guess ()
  :match  ()
  :assert (on)
  :why    "suppose the switch is on.")
(rule alive-from-no-false ()
  :match  (absent (false))
  :assert (alive)
  :why    "a state that holds no (false) is alive.")
"#;

/// **The `stale` row and `-p` print one text for one fact**: the row names
/// `(alive)` as the fact the dead node holds stale, and the form lists it
/// among root's facts beyond the program, beside root's hypothesis, which
/// `-p` lists as part of the state (Q-M1f.572).
#[test]
fn the_stale_row_and_the_form_print_one_text() {
    let scratch = Scratch::new("ein-fact-of-no-argument-stale");
    let file = write(&scratch, "stale.ein", STALE);
    let r = ein(&["solve", &file, "-p"]);
    assert_eq!(r.code, 0, "{}", r.err);
    no_space_inside("ein solve -p", &r.out);
    let stale = r
        .out
        .lines()
        .find(|l| l.trim_start().starts_with("stale "))
        .unwrap_or_else(|| panic!("no stale row:\n{}", r.out));
    assert!(
        stale.contains("{(on)}, dead: (alive) ← alive-from-no-false"),
        "{stale}"
    );
    assert_eq!(form_facts(&r.out), ["(alive)", "(open (on))"], "{}", r.out);
}

/// **A load refusal prints the fact as written**: a written derivation cycle
/// through two facts of no argument is named by their texts.
#[test]
fn a_derivation_cycle_names_its_facts_as_written() {
    let scratch = Scratch::new("ein-fact-of-no-argument-cycle");
    let file = write(
        &scratch,
        "cycle.ein",
        "(relation p :why \"p\")\n(relation q :why \"q\")\n\
         (p :rule a :using ((q)))\n(q :rule b :using ((p)))\n",
    );
    for command in [&["solve"][..], &["saturate"][..], &["show", "hyps"][..]] {
        let mut args = command.to_vec();
        args.push(&file);
        let r = ein(&args);
        assert_ne!(r.code, 0, "{command:?} loads a derivation cycle");
        no_space_inside(&format!("ein {command:?}"), &r.err);
        assert!(
            r.err.contains("derivation cycle: (p) -> (q) -> (p)"),
            "{command:?}: {}",
            r.err
        );
    }
}
