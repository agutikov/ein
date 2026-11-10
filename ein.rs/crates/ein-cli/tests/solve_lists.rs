//! **A `(solve …)` list names what the program has** — M1f S1f.7.8 (W76,
//! Q-M1f.114 to .116).
//!
//! The three lists of a `(solve …)` choose the candidates, so the lattice the
//! answer is a region of. Until S1f.7.8 a name in one that matched nothing was
//! read as one that matches none: `:hrules (gues …)` beside an hrule `guess`
//! proposed nothing, an activator `(att Color House)` proposed facts of a
//! relation nothing else names, an activator of another arity was skipped,
//! `:hypothesis-relations (like)` left H empty and `:no-hypothesis (like)`
//! restricted nothing, each in silence. Since then the loader refuses each
//! (`ein_ir::from_ir`'s `refuse_solve_names`), one line at the name, the same
//! on every command.
//!
//! What must keep loading is held beside it: a relation only a rule's or an
//! hrule's `:assert` derives, one named only by a goal or a premise, one the
//! program only passes to a rule (`(functional at)`), an imported rule's,
//! `is-a`, `eq`, the empty list, and an activator's argument that is a value.

use std::path::Path;
use std::process::Command;

use ein_corpus::{Scratch, repo_root};

struct Run {
    code: i32,
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
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// A program with one hrule, `guess (?R ?T1 ?T2)`, whose `?R` names the
/// relation it proposes facts of; `at` is declared, `near` is named only by
/// being passed to `functional`, and `glad` only by a rule's `:assert`.
const HEAD: &str = "(relation at Color House :why \"{?1} is at {?2}\")\n\
     (rule functional (?R)\n  :match (and (?R ?a ?b) (?R ?a ?c) (neq ?b ?c))\n  :assert (false)\n  \
     :why \"{?R} is not functional: {?a} has both {?b} and {?c}\")\n\
     (rule glad ()\n  :match (at ?c ?h)\n  :assert (glad ?c)\n  \
     :why \"{?c} is at {?h}, so {?c} is glad\")\n\
     (hrule guess (?R ?T1 ?T2)\n  :match (and (is-a ?a ?T1) (is-a ?b ?T2))\n  :assert (?R ?a ?b)\n  \
     :why \"guess that {?R} holds of {?a} and {?b}\")\n\
     (hrule plain ()\n  :match (is-a ?a Color)\n  :assert (seen ?a)\n  \
     :why \"guess that {?a} is seen\")\n\
     (functional at)\n(functional near)\n\
     (is-a Red Color) (is-a H1 House) (is-a H2 House)\n";

/// A claim `ein test` checks, so that it reads the program.
const CLAIM: &str = "(query :goal (at Red ?h) :expect (model))\n";

/// `(shape, the solve form, what the line says after `solve: its `)`.
const REFUSED: [(&str, &str, &str); 9] = [
    (
        "hrules-name",
        "(solve :hrules (gues (at Color House)))",
        ":hrules names gues, and the program declares no hrule gues — it declares guess, plain",
    ),
    (
        "hrules-name-of-a-rule",
        "(solve :hrules (glad))",
        ":hrules names glad, and the program declares no hrule glad — it declares guess, plain",
    ),
    (
        "hrules-later-bare-name",
        "(solve :hrules (guess (at Color House) plan))",
        ":hrules names plan, and the program declares no hrule plan — it declares guess, plain",
    ),
    (
        "hrules-activator-relation",
        "(solve :hrules (guess (att Color House)))",
        ":hrules gives hrule 'guess' (att Color House), whose att is its parameter ?R, which \
         names a relation, and the program names no relation att",
    ),
    (
        "hrules-second-activator-relation",
        "(solve :hrules (guess (at Color House) (nearr House House)))",
        ":hrules gives hrule 'guess' (nearr House House), whose nearr is its parameter ?R, which \
         names a relation, and the program names no relation nearr",
    ),
    (
        "hrules-activator-short",
        "(solve :hrules (guess (at Color)))",
        ":hrules gives hrule 'guess' (at Color), 2 arguments, and guess takes 3, (?R ?T1 ?T2)",
    ),
    (
        "hrules-activator-for-no-parameter",
        "(solve :hrules (plain (at)))",
        ":hrules gives hrule 'plain' (at), 1 argument, and plain takes 0, ()",
    ),
    (
        "hypothesis-relations",
        "(solve :hypothesis-relations (at glda))",
        ":hypothesis-relations names glda, and the program names no relation glda",
    ),
    (
        "no-hypothesis",
        "(solve :no-hypothesis (is-a att))",
        ":no-hypothesis names att, and the program names no relation att",
    ),
];

/// Lists that name what the program has, each of which loads.
const ACCEPTED: [(&str, &str); 12] = [
    ("declared", "(solve :hrules (guess (at Color House)))"),
    (
        "passed-to-a-rule",
        "(solve :hrules (guess (near House House)))",
    ),
    (
        "two-activators-and-the-name-again",
        "(solve :hrules (guess (at Color House) guess (near House House) plain))",
    ),
    ("no-parameter", "(solve :hrules (plain))"),
    (
        "a-value-that-is-no-relation",
        "(solve :hrules (guess (at Colour House)))",
    ),
    ("a-rules-conclusion", "(solve :hypothesis-relations (glad))"),
    ("an-hrules-conclusion", "(solve :no-hypothesis (seen is-a))"),
    ("a-premise-of-a-goal", "(solve :no-hypothesis (wanted))"),
    ("is-a-alone", "(solve :hypothesis-relations (is-a))"),
    ("the-empty-lists", "(solve :hypothesis-relations ())"),
    ("the-empty-hrules", "(solve :hrules ())"),
    ("no-list", "(solve)"),
];

/// Every command that reads a program, the file last, each with the exit a
/// load error takes (`ein test`: 2); `macro_bodies.rs`'s list.
const COMMANDS: [(&[&str], i32); 20] = [
    (&["saturate"], 1),
    (&["saturate", "--dump"], 1),
    (&["solve"], 1),
    (&["solve", "--minimal"], 1),
    (&["solve", "--maximal"], 1),
    (&["solve", "--single"], 1),
    (&["solve", "--count"], 1),
    (&["solve", "-n", "1"], 1),
    (&["solve", "--events", "{events}"], 1),
    (&["oracle"], 1),
    (&["test"], 2),
    (&["test", "--events", "{events}"], 2),
    (&["render", "lattice"], 1),
    (&["render", "kb"], 1),
    (&["render", "program"], 1),
    (&["render", "rules"], 1),
    (&["render", "rule", "--name", "guess"], 1),
    (&["render", "constraints"], 1),
    (&["kb", "save"], 1),
    (&["kb", "save", "--saturate"], 1),
];

fn argv(cmd: &[&str], file: &Path, einb: &Path, events: &Path) -> Vec<String> {
    let mut args: Vec<String> = cmd
        .iter()
        .map(|a| match *a {
            "{events}" => events.display().to_string(),
            a => a.to_string(),
        })
        .collect();
    args.push(file.display().to_string());
    if cmd[0] == "kb" {
        args.push(einb.display().to_string());
    }
    args
}

/// `file` refused on every command with one first line, the same on each,
/// `want` in it after `solve: its `, and no container written.
fn refused_everywhere(file: &Path, shown: &str, want: &str, dir: &Path) -> Vec<String> {
    let mut bad = Vec::new();
    let einb = dir.join("x.einb");
    let events = dir.join("events.jsonl");
    let mut first: Option<String> = None;
    for (cmd, code) in COMMANDS {
        let args = argv(cmd, file, &einb, &events);
        let argv: Vec<&str> = args.iter().map(String::as_str).collect();
        let r = ein(&argv);
        let line = r.err.lines().next().unwrap_or("").to_string();
        if r.code != code {
            bad.push(format!(
                "ein {argv:?}: exit {}, want {code}: {line}",
                r.code
            ));
            continue;
        }
        let fits = line.starts_with(&format!("kb load error: {shown}:"))
            && line.contains(&format!(": solve: its {want}"));
        if !fits {
            bad.push(format!(
                "ein {argv:?}: {line:?}\n  wants kb load error: {shown}:<line>:<col>: solve: its \
                 {want} …"
            ));
        }
        match &first {
            None => first = Some(line.clone()),
            Some(f) if *f != line => bad.push(format!(
                "ein {argv:?}: {line:?}, where the first command printed {f:?}"
            )),
            Some(_) => {}
        }
        if einb.exists() {
            bad.push(format!(
                "ein {argv:?} wrote a container of a refused program"
            ));
            let _ = std::fs::remove_file(&einb);
        }
    }
    bad
}

#[test]
fn a_list_naming_nothing_is_refused_on_every_command() {
    let scratch = Scratch::new("ein-solve-lists");
    let mut bad = Vec::new();
    for (shape, solve, want) in REFUSED {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, format!("{HEAD}{solve}\n{CLAIM}")).expect("written");
        let shown = file.display().to_string();
        for b in refused_everywhere(&file, &shown, want, scratch.path()) {
            bad.push(format!("{shape}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The register's five witnesses (W76), each read from its file, as
/// `wrong_answers.rs` reads one command's run of each.
#[test]
fn the_registers_witnesses_are_refused_on_every_command() {
    let scratch = Scratch::new("ein-solve-list-witnesses");
    let witnesses: [(&str, &str); 5] = [
        ("solve-hrules-no-such-hrule", ":hrules names gues"),
        (
            "solve-hrules-no-such-relation",
            ":hrules gives hrule 'guess' (att Color House), whose att is its parameter ?R",
        ),
        (
            "solve-hrules-activator-arity",
            ":hrules gives hrule 'guess' (at Color), 2 arguments, and guess takes 3",
        ),
        (
            "solve-hypothesis-relations-no-such-relation",
            ":hypothesis-relations names like",
        ),
        (
            "solve-no-hypothesis-no-such-relation",
            ":no-hypothesis names like",
        ),
    ];
    let mut bad = Vec::new();
    for (stem, want) in witnesses {
        let shown = format!("examples/ein-bugs/wrong-answers/{stem}.ein");
        for b in refused_everywhere(Path::new(&shown), &shown, want, scratch.path()) {
            bad.push(format!("{stem}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// What a list may name: every relation the program writes outside the
/// lists, however it comes by it. The claim check's refusal of a relation
/// only a rule derives is not copied here.
#[test]
fn a_list_naming_what_the_program_has_loads() {
    let scratch = Scratch::new("ein-solve-lists-accepted");
    let mut bad = Vec::new();
    let goal = "(query :goal (and (at Red ?h) (absent (wanted ?h))))\n";
    let mut programs: Vec<(String, String)> = ACCEPTED
        .iter()
        .map(|(shape, solve)| (shape.to_string(), format!("{HEAD}{solve}\n{goal}")))
        .collect();
    // A relation only an imported rule concludes, and `eq` given to a
    // parameter that names a relation.
    programs.push((
        "an-imported-rules-conclusion".to_string(),
        "(import std.algebra :symbols (bijective-properties functional))\n\
         (relation r A B :why \"{?1} r {?2}\")\n(is-a a A) (is-a b B)\n(bijective r)\n\
         (solve :no-hypothesis (functional injective is-a))\n"
            .to_string(),
    ));
    programs.push((
        "eq-as-the-relation".to_string(),
        "(hrule h (?R)\n  :match (and (is-a ?x T) (?R ?x ?x))\n  :assert (hit ?x)\n  \
         :why \"{?R} holds of {?x} and itself: guess that {?x} is hit\")\n\
         (is-a k T)\n(solve :hrules (h (eq)))\n"
            .to_string(),
    ));
    for (shape, text) in &programs {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, text).expect("written");
        let f = file.display().to_string();
        let einb = scratch
            .path()
            .join(format!("{shape}.einb"))
            .display()
            .to_string();
        let runs: [Vec<&str>; 5] = [
            vec!["saturate", &f],
            vec!["solve", "--count", &f],
            vec!["render", "program", &f],
            vec!["render", "kb", &f],
            vec!["kb", "save", &f, &einb],
        ];
        for argv in runs {
            let r = ein(&argv);
            if r.code != 0 {
                bad.push(format!(
                    "{shape}: ein {argv:?}: exit {}: {}",
                    r.code,
                    r.err.lines().next().unwrap_or("")
                ));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
