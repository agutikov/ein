//! **A `(solve …)` list names what the program has** (Q-M1f.114).
//!
//! The two lists of a `(solve …)` configure the blind enumerator, so the
//! lattice the answer is a region of. A name in one that matched nothing,
//! read as one that matches none, would act in silence:
//! `:hypothesis-relations (like)` would leave H empty and `:no-hypothesis
//! (like)` restrict nothing. The loader refuses each (`ein_ir::from_ir`'s
//! `refuse_solve_names`), one line at the name, the same on every command.
//!
//! What must keep loading is held beside it: a relation only a rule's
//! `:assert` derives, a hypothesis among them, one named only by a goal that
//! an activator has a rule that concludes a hypothesis propose (a goal over a
//! relation nothing else names is itself refused, Q-M1f.119), one the program
//! only passes to a rule (`(functional at)`), an imported rule's, `is-a`,
//! `eq`, and the empty list.

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

/// A program with one rule that concludes a hypothesis, `guess (?R ?T1
/// ?T2)`, whose `?R` names the relation it proposes facts of, and one with no
/// parameter, `plain`; `at` is declared, `near` is named only by being passed
/// to `functional`, and `glad` only by a rule's `:assert`.
const HEAD: &str = "(relation at Color House :why \"{?3} is at {?4}\")\n\
     (rule functional (?R)\n  :match (and (?R ?a ?b) (?R ?a ?c) (neq ?b ?c))\n  :assert (false)\n  \
     :why \"{?R} is not functional: {?a} has both {?b} and {?c}\")\n\
     (rule glad ()\n  :match (at ?c ?h)\n  :assert (glad ?c)\n  \
     :why \"{?c} is at {?h}, so {?c} is glad\")\n\
     (rule guess (?R ?T1 ?T2)\n  :match (and (is-a ?a ?T1) (is-a ?b ?T2))\n  :assert (open (?R ?a ?b))\n  \
     :why \"guess that {?R} holds of {?a} and {?b}\")\n\
     (rule plain ()\n  :match (is-a ?a Color)\n  :assert (open (seen ?a))\n  \
     :why \"guess that {?a} is seen\")\n\
     (functional at)\n(functional near)\n\
     (is-a Red Color) (is-a H1 House) (is-a H2 House)\n";

/// A claim `ein test` checks, so that it reads the program.
const CLAIM: &str = "(query :goal (at Red ?h) :expect (model))\n";

/// `(shape, the solve form, what the line says after `solve: its `)`.
const REFUSED: [(&str, &str, &str); 2] = [
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

/// Programs that name what they have, each of which loads: the lists, and
/// what a rule that concludes a hypothesis lets a goal name.
const ACCEPTED: [(&str, &str); 8] = [
    ("a-rules-conclusion", "(solve :hypothesis-relations (glad))"),
    (
        "a-hypothesis-rules-conclusion",
        "(solve :no-hypothesis (seen is-a))",
    ),
    // Named only by the goal below, which may ask about it because this
    // activator has `guess` propose it (Q-M1f.119).
    ("a-premise-of-a-goal", "(guess wanted Color House)"),
    ("is-a-alone", "(solve :hypothesis-relations (is-a))"),
    ("the-empty-lists", "(solve :hypothesis-relations ())"),
    ("no-list", "(solve)"),
    ("an-activator", "(guess at Color House)"),
    ("no-parameter", ""),
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

/// The regressions' two witnesses of a `(solve …)` list, each read from its
/// file, as `regressions.rs` reads one command's run of each.
#[test]
fn the_registers_witnesses_are_refused_on_every_command() {
    let scratch = Scratch::new("ein-solve-list-witnesses");
    let witnesses: [(&str, &str); 2] = [
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
    // `wanted` is named only by this goal, and a goal may ask about it only
    // where an activator has `guess` propose it (Q-M1f.119).
    let wanted = "(query :goal (and (at Red ?h) (absent (wanted Red ?h))))\n";
    let goal = "(query :goal (and (at Red ?h) (absent (glad ?h))))\n";
    // A list beside a rule that proposes what it filters is a conflict
    // (`mechanism_conflicts.rs`), and `plain`, which has no parameter, always
    // proposes `seen`: the shapes that write a list are read beside a `plain`
    // with a parameter and no activator, which proposes nothing and still
    // writes `seen` in its `:assert`, so the list is dead, one `note:`, and
    // the names it holds are what this test is about.
    let quiet = HEAD.replace(
        "(rule plain ()\n  :match (is-a ?a Color)",
        "(rule plain (?T)\n  :match (is-a ?a ?T)",
    );
    assert_ne!(
        quiet, HEAD,
        "the quiet head is HEAD with plain parameterised"
    );
    let mut programs: Vec<(String, String)> = ACCEPTED
        .iter()
        .map(|(shape, solve)| {
            let goal = if *shape == "a-premise-of-a-goal" {
                wanted
            } else {
                goal
            };
            let head =
                if solve.contains(":hypothesis-relations") || solve.contains(":no-hypothesis") {
                    &quiet
                } else {
                    HEAD
                };
            (shape.to_string(), format!("{head}{solve}\n{goal}"))
        })
        .collect();
    // A relation only an imported rule concludes, and `eq` given to a
    // parameter that names a relation. `functional` is listed: a name the
    // program writes and does not list is its own. `injective` is not
    // listed: with `std.algebra` whole, its generic `imply1` could write
    // `(injective <fact>)`, and the loader warns of that program (Q-M1f.1).
    programs.push((
        "an-imported-rules-conclusion".to_string(),
        "(import std.algebra :symbols (bijective-properties functional))\n\
         (relation r A B :why \"{?3} r {?4}\")\n(is-a a A) (is-a b B)\n(bijective r)\n\
         (solve :no-hypothesis (functional is-a))\n"
            .to_string(),
    ));
    programs.push((
        "eq-as-the-relation".to_string(),
        "(rule h (?R)\n  :match (and (is-a ?x T) (?R ?x ?x))\n  :assert (open (hit ?x))\n  \
         :why \"{?R} holds of {?x} and itself: guess that {?x} is hit\")\n\
         (is-a k T)\n(h eq)\n"
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
