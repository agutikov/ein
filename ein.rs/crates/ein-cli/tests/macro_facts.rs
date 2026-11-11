//! **A macro's call is no fact** — M1f S1f.7.12 (W86, Q-M1f.130 to .133).
//!
//! The loader expands a macro's call in a rule's or an hrule's `:match` and
//! `:assert` and in a query's `:goal`, and nowhere else. Until S1f.7.12 a
//! fact headed by a macro's name, `(settled Ann)` beside `(macro settled (?g)
//! (paid ?g))`, was stored as a fact of a relation `settled`: the fact the
//! body spells was not stored, and no premise could read the one that was,
//! since `(settled ?g)` in a `:match` is the call. The same held inside a
//! fact, under `(not …)`, in a fact's `:using`, inside a fact a claim lists,
//! of an imported `std.macro` macro and of one the program does not import,
//! and a relation could be declared under a macro's name. Since then the
//! loader refuses each (`ein_ir::from_ir`'s `refuse_macro_facts` and
//! `refuse_macro_claims`), one line at the call, the same on every command.
//!
//! What must keep loading is held beside it: a call in a `:match`, inside a
//! pattern, in a rule's and an hrule's `:assert` (expanded, as before the
//! stage: Q-M1f.130) and in a goal, a macro's name written as a value, and a
//! macro nothing calls.

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

/// Two guests, a relation `paid`, one that holds a fact, `says`, and the
/// macro `settled`, whose body is `(paid ?g)`.
const HEAD: &str = "(relation paid Guest :why \"{?1} has paid\")\n\
     (relation says Claim :why \"it is said that {?1}\")\n\
     (relation welcome Guest :why \"{?1} is welcome\")\n\
     (is-a Ann Guest) (is-a Bob Guest)\n\
     (macro settled (?g) (paid ?g) :why \"{?g} has settled: they have paid\")\n";

/// What every line about a fact ends with.
const TAIL: &str = "a macro is called in a :match, an :assert or a :goal, where its call is replaced \
     by its body; a fact is not expanded, and no fact is of a macro: write the fact itself";

const SETTLED: &str = "the macro settled, (macro settled (?g) (paid ?g))";

/// `(shape, what the program writes after HEAD, what the line says after the
/// position)`, `{TAIL}` left to the check.
fn refused() -> Vec<(&'static str, String, String)> {
    let fact = |shape: &'static str, text: &str, subject: String| {
        (
            shape,
            format!("{HEAD}{text}\n"),
            format!("{subject} — {TAIL}"),
        )
    };
    vec![
        fact(
            "a-fact",
            "(settled Ann)",
            format!("the fact (settled Ann) is a call of {SETTLED}"),
        ),
        fact(
            "a-fact-of-another-arity",
            "(settled Ann Bob)",
            format!("the fact (settled Ann Bob) is a call of {SETTLED}"),
        ),
        fact(
            "inside-a-fact",
            "(says (settled Ann))",
            format!("the fact (says (settled Ann)) holds (settled Ann), a call of {SETTLED}"),
        ),
        fact(
            "two-deep",
            "(says (says (settled Ann)))",
            format!(
                "the fact (says (says (settled Ann))) holds (settled Ann), a call of {SETTLED}"
            ),
        ),
        fact(
            "under-not",
            "(not (settled Bob))",
            format!("the fact (not (settled Bob)) holds (settled Bob), a call of {SETTLED}"),
        ),
        fact(
            "in-is-a",
            "(is-a (settled Ann) Claim)",
            format!("the fact (is-a (settled Ann) Claim) holds (settled Ann), a call of {SETTLED}"),
        ),
        fact(
            "in-using",
            "(rule greet () :match (paid ?g) :assert (welcome ?g) :why \"{?g} is welcome\")\n\
             (welcome Ann :rule greet :using ((settled Ann)))",
            format!(
                "the fact (welcome Ann :rule greet :using ((settled Ann))) holds (settled Ann), a \
                 call of {SETTLED}"
            ),
        ),
        fact(
            "a-rules-activator",
            "(rule settled (?t) :match (is-a ?g ?t) :assert (welcome ?g) :why \"{?g} is welcome\")\n\
             (settled Guest)",
            format!("the fact (settled Guest) is a call of {SETTLED}"),
        ),
        (
            "an-imported-macro",
            format!("(import std.macro :symbols (unknown))\n{HEAD}(unknown (paid Bob))\n"),
            format!(
                "the fact (unknown (paid Bob)) is a call of the macro unknown, (macro unknown \
                 (?P) (and (absent ?P) (absent (not ?P)))) — {TAIL}"
            ),
        ),
        (
            "an-imported-macro-qualified",
            format!("(import std.macro)\n{HEAD}(std.macro.unknown (paid Bob))\n"),
            format!(
                "the fact (std.macro.unknown (paid Bob)) is a call of the macro \
                 std.macro.unknown, (macro std.macro.unknown (?P) (and (absent ?P) (absent (not \
                 ?P)))) — {TAIL}"
            ),
        ),
        fact(
            "a-macro-not-imported",
            "(unknown (paid Bob))",
            "the fact (unknown (paid Bob)) is a call of std.macro's macro unknown, which the \
             program does not import"
                .to_string(),
        ),
        fact(
            "a-macro-not-imported-inside-a-fact",
            "(says (forall x (paid Ann) (is-a Ann Guest)))",
            "the fact (says (forall x (paid Ann) (is-a Ann Guest))) holds (forall x (paid Ann) \
             (is-a Ann Guest)), a call of std.macro's macro forall, which the program does not \
             import"
                .to_string(),
        ),
        (
            "a-relation-of-a-macros-name",
            format!("{HEAD}(relation settled Guest :why \"{{?1}} has settled\")\n"),
            "relation 'settled': settled is a macro too, (macro settled (?g) (paid ?g)) — a \
             premise (settled …) is the macro's call, replaced by its body, and a fact (settled …) \
             is refused, so nothing could be of the relation; give one of the two another name"
                .to_string(),
        ),
        (
            "inside-a-claims-fact",
            format!(
                "{HEAD}(says (paid Ann))\n(query :goal (says ?c) :expect (model (says (settled \
                 Ann))))\n"
            ),
            format!(
                "query: its :expect lists (settled Ann), a call of {SETTLED} — a claim lists \
                 facts, and {TAIL}"
            ),
        ),
        (
            "inside-a-region-claims-fact",
            format!(
                "{HEAD}(says (paid Ann))\n(solve :hypothesis-relations () :expect (region (model \
                 (says (settled Ann)))))\n"
            ),
            format!(
                "solve: its :expect lists (settled Ann), a call of {SETTLED} — a claim lists \
                 facts, and {TAIL}"
            ),
        ),
    ]
}

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
    (&["render", "rule", "--name", "greet"], 1),
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
/// `want` in it after a position in `shown`, and no container written.
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
        // `<file>:<line>:<col>: ` and then the line.
        let fits = line
            .strip_prefix(&format!("kb load error: {shown}:"))
            .and_then(|rest| rest.split_once(": "))
            .is_some_and(|(at, rest)| {
                at.split(':').all(|n| n.parse::<u32>().is_ok()) && rest.starts_with(want)
            });
        if !fits {
            bad.push(format!(
                "ein {argv:?}: {line:?}\n  wants kb load error: {shown}:<line>:<col>: {want} …"
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
fn a_macros_call_written_as_a_fact_is_refused_on_every_command() {
    let scratch = Scratch::new("ein-macro-facts");
    let mut bad = Vec::new();
    for (shape, text, want) in refused() {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, text).expect("written");
        let shown = file.display().to_string();
        for b in refused_everywhere(&file, &shown, &want, scratch.path()) {
            bad.push(format!("{shape}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The register's eight witnesses (W86), each read from its file, as
/// `wrong_answers.rs` reads one command's run of each.
#[test]
fn the_registers_witnesses_are_refused_on_every_command() {
    let scratch = Scratch::new("ein-macro-fact-witnesses");
    let witnesses: [(&str, &str); 8] = [
        (
            "macro-call-fact",
            "the fact (settled Ann) is a call of the macro settled",
        ),
        (
            "macro-call-nested-fact",
            "the fact (says (settled Ann)) holds (settled Ann), a call of the macro settled",
        ),
        (
            "macro-call-negated-fact",
            "the fact (not (settled Bob)) holds (settled Bob), a call of the macro settled",
        ),
        (
            "macro-call-stdlib-fact",
            "the fact (unknown (paid Bob)) is a call of the macro unknown",
        ),
        (
            "macro-call-stdlib-unimported",
            "the fact (unknown (paid Bob)) is a call of std.macro's macro unknown, which the \
             program does not import",
        ),
        (
            "macro-call-in-claim",
            "query: its :expect lists (settled Ann), a call of the macro settled",
        ),
        (
            "macro-call-in-region-claim",
            "solve: its :expect lists (settled Ann), a call of the macro settled",
        ),
        (
            "macro-and-relation-one-name",
            "relation 'settled': settled is a macro too",
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

/// Where a macro is called, and what else may spell its name: each loads on
/// every command, and the calls answer by their bodies.
#[test]
fn a_macro_called_where_it_is_expanded_loads() {
    let scratch = Scratch::new("ein-macro-facts-accepted");
    let greet =
        "(rule greet () :match (settled ?g) :assert (welcome ?g) :why \"{?g} is welcome\")\n";
    // `(shape, the program after HEAD, a line `saturate --dump` prints)`.
    let programs: [(&str, String, &str); 7] = [
        (
            "a-call-in-a-match",
            format!("(paid Ann)\n{greet}"),
            "(welcome Ann :rule greet)",
        ),
        (
            "a-call-inside-a-pattern",
            "(says (paid Ann))\n(rule greet () :match (says (settled ?g)) :assert (welcome ?g) \
             :why \"{?g} is welcome\")\n"
                .to_string(),
            "(welcome Ann :rule greet)",
        ),
        // A conclusion's call is expanded, as before S1f.7.12 (Q-M1f.130).
        (
            "a-call-in-an-assert",
            "(welcome Ann)\n(rule pay () :match (welcome ?g) :assert (settled ?g) :why \"{?g} \
             pays\")\n"
                .to_string(),
            "(paid Ann :rule pay)",
        ),
        (
            "a-call-inside-a-conclusion",
            "(welcome Ann)\n(rule tell () :match (welcome ?g) :assert (says (settled ?g)) :why \
             \"it is said {?g} paid\")\n"
                .to_string(),
            "(says (paid Ann) :rule tell)",
        ),
        (
            "a-call-in-an-hrules-assert-and-a-goal",
            "(hrule guess () :match (is-a ?g Guest) :assert (settled ?g) :why \"guess {?g} \
             paid\")\n(solve :hrules (guess))\n(query :goal (settled ?g))\n"
                .to_string(),
            "(is-a Ann Guest)",
        ),
        (
            "the-name-as-a-value",
            "(relation word Name :why \"{?1} is a word\")\n(word settled)\n".to_string(),
            "(word settled)",
        ),
        (
            "a-macro-nothing-calls",
            "(paid Ann)\n".to_string(),
            "(paid Ann)",
        ),
    ];
    let mut bad = Vec::new();
    for (shape, text, stored) in &programs {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, format!("{HEAD}{text}")).expect("written");
        let f = file.display().to_string();
        let einb = scratch
            .path()
            .join(format!("{shape}.einb"))
            .display()
            .to_string();
        let runs: [Vec<&str>; 6] = [
            vec!["saturate", "--dump", &f],
            vec!["solve", "--count", &f],
            vec!["oracle", &f],
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
            } else if argv[0] == "saturate" && !r.out.lines().any(|l| l.trim() == *stored) {
                bad.push(format!(
                    "{shape}: `ein saturate --dump` does not print {stored}"
                ));
            }
            // No fact of the macro's name is stored (the stage's acceptance).
            if argv[0] == "saturate"
                && r.out
                    .lines()
                    .any(|l| l.trim_start().starts_with("(settled "))
            {
                bad.push(format!("{shape}: a fact of `settled` is stored"));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
