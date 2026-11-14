//! **A form headed by a declared relation has as many arguments as the
//! declaration has types** — M1f S1f.6.23 (W89), on the user's ruling of
//! 2026-11-12 on question 3 of P1f.6's `questions.md`, *"(a)"*; Q-M1f.201 to
//! .204 defaulted.
//!
//! Until S1f.6.23 no form was held to its relation's declaration: `(likes
//! Bob)` beside `(relation likes Person Drink)` was stored, a premise
//! `(likes ?who)` matched nothing, a conclusion `(fan ?who ?what)` derived
//! facts no premise of the declared count read, and the calculator's `(add (n
//! z 5))` and a `std.math.num9` number of eight digits were answered by silence,
//! exit 0. Since then the loader refuses each (`ein_ir::from_ir`'s
//! `refuse_wrong_counts`), one line a form, at the form, the same on every
//! command.
//!
//! What must keep loading is held beside it: a relation nothing declares, at
//! any count; a keyword pair, which is no argument; a relation written
//! through a rule's parameter; the kernel's own relations; a declared
//! relation's name as a value and in a `(solve …)` list; and a macro no rule
//! calls.

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

/// Two declared relations, a fact of each, and a rule `r`.
const HEAD: &str = "(relation likes Person Drink :why \"{?1} likes {?2}\")\n\
     (relation fan Person :why \"{?1} is a fan\")\n\
     (relation flag :why \"the flag is up\")\n\
     (likes Ann Tea)\n(fan Ann)\n(flag)\n\
     (rule r ()\n  :match (likes ?a ?b)\n  :assert (seen ?a)\n  :why \"{?a} likes {?b}, so {?a} is seen\")\n";

/// What every line ends with.
const TAIL: &str =
    " — a form headed by a declared relation has as many arguments as its declaration has types";

const LIKES: &str = "(relation likes Person Drink) declares 2";
const FAN: &str = "(relation fan Person) declares 1";

/// `(shape, what is added to [`HEAD`], the line's `<line>:<col>`, the line
/// after the position)`, the tail aside.
fn refused() -> Vec<(&'static str, &'static str, &'static str, String)> {
    vec![
        (
            "top-level-fact-short",
            "(likes Bob)\n",
            "11:2",
            format!("the fact (likes Bob) gives likes 1 argument, and {LIKES}"),
        ),
        (
            "top-level-fact-long",
            "(fan Bob Tea)\n",
            "11:2",
            format!("the fact (fan Bob Tea) gives fan 2 arguments, and {FAN}"),
        ),
        (
            "top-level-fact-none",
            "(likes)\n",
            "11:2",
            format!("the fact (likes) gives likes 0 arguments, and {LIKES}"),
        ),
        (
            "a-bare-declaration-declares-none",
            "(flag up)\n",
            "11:2",
            "the fact (flag up) gives flag 1 argument, and (relation flag) declares 0".to_string(),
        ),
        (
            "a-keyword-pair-is-no-argument",
            "(likes Bob :source \"said\")\n",
            "11:2",
            format!("the fact (likes Bob :source \"said\") gives likes 1 argument, and {LIKES}"),
        ),
        (
            "nested-fact",
            "(says Ann (likes Bob))\n",
            "11:12",
            format!(
                "the fact (says Ann (likes Bob)) holds (likes Bob), which gives likes 1 argument, \
                 and {LIKES}"
            ),
        ),
        (
            "nested-two-deep",
            "(says Ann (thinks Bob (fan Cy Dee)))\n",
            "11:24",
            format!(
                "the fact (says Ann (thinks Bob (fan Cy Dee))) holds (fan Cy Dee), which gives \
                 fan 2 arguments, and {FAN}"
            ),
        ),
        (
            "under-a-stored-negation",
            "(not (likes Bob))\n",
            "11:7",
            format!(
                "the fact (not (likes Bob)) holds (likes Bob), which gives likes 1 argument, and \
                 {LIKES}"
            ),
        ),
        (
            "a-facts-using",
            "(seen Bob :rule r :using ((likes Bob)))\n",
            "11:28",
            format!(
                "the fact (seen Bob :rule r :using ((likes Bob))) holds (likes Bob), which gives \
                 likes 1 argument, and {LIKES}"
            ),
        ),
        (
            "a-rules-match",
            "(rule s ()\n  :match (likes ?a)\n  :assert (seen ?a)\n  :why \"{?a} is seen\")\n",
            "12:11",
            format!(
                "rule 's': its :match writes (likes ?a), which gives likes 1 argument, and {LIKES}"
            ),
        ),
        (
            "a-rules-match-under-absent",
            "(rule s ()\n  :match (and (fan ?a) (absent (likes ?a)))\n  :assert (seen ?a)\n  \
             :why \"{?a} is seen\")\n",
            "12:33",
            format!(
                "rule 's': its :match writes (likes ?a), which gives likes 1 argument, and {LIKES}"
            ),
        ),
        (
            "a-rules-match-under-not",
            "(rule s ()\n  :match (and (fan ?a) (not (likes ?a ?a ?a)))\n  :assert (seen ?a)\n  \
             :why \"{?a} is seen\")\n",
            "12:30",
            format!(
                "rule 's': its :match writes (likes ?a ?a ?a), which gives likes 3 arguments, \
                 and {LIKES}"
            ),
        ),
        (
            "a-rules-match-inside-a-pattern",
            "(rule s ()\n  :match (says ?a (likes ?b))\n  :assert (seen ?a)\n  \
             :why \"{?a} is seen, {?b}\")\n",
            "12:20",
            format!(
                "rule 's': its :match writes (likes ?b), which gives likes 1 argument, and {LIKES}"
            ),
        ),
        (
            "a-rules-assert",
            "(rule s ()\n  :match (likes ?a ?b)\n  :assert (fan ?a ?b)\n  \
             :why \"{?a} likes {?b}\")\n",
            "13:12",
            format!(
                "rule 's': its :assert writes (fan ?a ?b), which gives fan 2 arguments, and {FAN}"
            ),
        ),
        (
            "a-rules-assert-inside-a-conclusion",
            "(rule s ()\n  :match (likes ?a ?b)\n  :assert (and (seen ?a) (says ?a (fan)))\n  \
             :why \"{?a} likes {?b}\")\n",
            "13:36",
            format!("rule 's': its :assert writes (fan), which gives fan 0 arguments, and {FAN}"),
        ),
        (
            "an-hrules-match",
            "(hrule h ()\n  :match (fan ?a ?b)\n  :assert (seen ?a)\n  \
             :why \"guess that {?a} is seen, {?b}\")\n",
            "12:11",
            format!(
                "hrule 'h': its :match writes (fan ?a ?b), which gives fan 2 arguments, and {FAN}"
            ),
        ),
        (
            "an-hrules-assert",
            "(hrule h ()\n  :match (fan ?a)\n  :assert (likes ?a)\n  \
             :why \"guess that {?a} likes\")\n",
            "13:12",
            format!(
                "hrule 'h': its :assert writes (likes ?a), which gives likes 1 argument, and \
                 {LIKES}"
            ),
        ),
        (
            "an-obligations-match",
            "(rule owed (?R)\n  :match (and (fan ?a) (absent (likes ?a)))\n  :assert (open ?R)\n  \
             :why \"{?a} owes a {?R}\")\n(owed likes)\n",
            "12:33",
            format!(
                "rule 'owed': its :match writes (likes ?a), which gives likes 1 argument, and \
                 {LIKES}"
            ),
        ),
        (
            "a-macros-body-where-a-rule-calls-it",
            "(macro liked (?a) (likes ?a) :why \"{?a} is liked\")\n\
             (rule s ()\n  :match (liked ?a)\n  :assert (seen ?a)\n  :why \"{?a} is seen\")\n",
            "11:20",
            format!(
                "rule 's': its :match writes (likes ?a), which gives likes 1 argument, and {LIKES}"
            ),
        ),
        (
            "a-goal",
            "(query :goal (likes ?a))\n",
            "11:15",
            format!(
                "query: its :goal writes (likes ?a), which gives likes 1 argument, and {LIKES}"
            ),
        ),
        (
            "a-goal-inside-a-pattern",
            "(says Ann Bob)\n(query :goal (and (fan ?a) (absent (says ?a (fan)))))\n",
            "12:46",
            format!("query: its :goal writes (fan), which gives fan 0 arguments, and {FAN}"),
        ),
        (
            "a-macros-body-where-a-goal-calls-it",
            "(macro liked (?a) (likes ?a) :why \"{?a} is liked\")\n(query :goal (liked ?a))\n",
            "11:20",
            format!(
                "query: its :goal writes (likes ?a), which gives likes 1 argument, and {LIKES}"
            ),
        ),
        (
            "a-queries-claim",
            "(query :goal (says ?a ?b) :expect (model (says Ann (likes Bob))))\n(says Ann Bob)\n",
            "11:53",
            format!(
                "query: its :expect lists (says Ann (likes Bob)), which holds (likes Bob), which \
                 gives likes 1 argument, and {LIKES}"
            ),
        ),
        (
            "a-region-claim",
            "(solve :hypothesis-relations () :expect (region (model (likes Ann))))\n",
            "11:57",
            format!(
                "solve: its :expect lists (likes Ann), which gives likes 1 argument, and {LIKES}"
            ),
        ),
    ]
}

/// Every command that reads a program, the file last, each with the exit a
/// load error takes (`ein test`: 2); `solve_lists.rs`'s list.
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
    (&["render", "rule", "--name", "r"], 1),
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
/// holding `want`, and no container written.
fn refused_everywhere(file: &Path, want: &str, dir: &Path) -> Vec<String> {
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
        if !(line.starts_with("kb load error: ") && line.contains(want)) {
            bad.push(format!(
                "ein {argv:?}: {line:?}\n  wants kb load error: … {want}"
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
fn a_form_of_another_count_is_refused_on_every_command() {
    let scratch = Scratch::new("ein-declared-counts");
    let mut bad = Vec::new();
    for (shape, added, at, says) in refused() {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, format!("{HEAD}{added}")).expect("written");
        let want = format!("{}:{at}: {says}{TAIL}", file.display());
        for b in refused_everywhere(&file, &want, scratch.path()) {
            bad.push(format!("{shape}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// One line a form: a program with three forms of another count is refused
/// with three lines, `; `-joined as the loader's lines are, in the order the
/// loader reads the program, rules, then facts, then queries.
#[test]
fn each_form_has_a_line_of_its_own() {
    let scratch = Scratch::new("ein-declared-counts-lines");
    let file = scratch.path().join("three.ein");
    let text = format!(
        "{HEAD}(likes Bob)\n(rule s ()\n  :match (fan ?a ?b)\n  :assert (seen ?a)\n  \
         :why \"{{?a}} is seen, {{?b}}\")\n(query :goal (flag ?x))\n"
    );
    std::fs::write(&file, text).expect("written");
    let f = file.display().to_string();
    let r = ein(&["saturate", &f]);
    let want = format!(
        "kb load error: {f}:13:11: rule 's': its :match writes (fan ?a ?b), which gives fan 2 \
         arguments, and {FAN}{TAIL}; {f}:11:2: the fact (likes Bob) gives likes 1 argument, and \
         {LIKES}{TAIL}; {f}:16:15: query: its :goal writes (flag ?x), which gives flag 1 \
         argument, and (relation flag) declares 0{TAIL}"
    );
    assert_eq!(r.code, 1);
    assert_eq!(r.err.lines().next().unwrap_or(""), want);
}

/// The register's seven witnesses (W89), each read from its file, as
/// `wrong_answers.rs` reads one command's run of each.
#[test]
fn the_registers_witnesses_are_refused_on_every_command() {
    let scratch = Scratch::new("ein-declared-count-witnesses");
    let witnesses: [(&str, &str); 7] = [
        (
            "arity-top-level-fact",
            ": the fact (likes Bob) gives likes 1 argument, and (relation likes Person \
             Drink) declares 2",
        ),
        (
            "arity-nested-fact",
            ": the fact (add (n 1) (n z 5)) holds (n 1), which gives n 1 argument, and \
             (relation n Number Digit) declares 2",
        ),
        (
            "arity-request",
            ": the fact (add (n z 5)) gives add 1 argument, and (relation add Operand \
             Operand) declares 2",
        ),
        (
            "arity-num9-eight-digits",
            "holds (num9 0 0 1 2 3 0 7 8), which gives num9 8 arguments, and (relation num9 \
             Digit Digit Digit Digit Digit Digit Digit Digit Digit) declares 9",
        ),
        (
            "arity-in-match",
            ": rule 'fans': its :match writes (likes ?who), which gives likes 1 argument, \
             and (relation likes Person Drink) declares 2",
        ),
        (
            "arity-in-assert",
            ": rule 'fans': its :assert writes (fan ?who ?what), which gives fan 2 \
             arguments, and (relation fan Person) declares 1",
        ),
        (
            "arity-in-goal",
            ": query: its :goal writes (likes ?who), which gives likes 1 argument, and \
             (relation likes Person Drink) declares 2",
        ),
    ];
    let mut bad = Vec::new();
    for (stem, want) in witnesses {
        let shown = format!("examples/ein-bugs/wrong-answers/{stem}.ein");
        for b in refused_everywhere(Path::new(&shown), want, scratch.path()) {
            bad.push(format!("{stem}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// An imported declaration is held as the program's own is: listed, under
/// `:as`, where the relation has the qualified name, and from a file beside
/// the program. A list names what the program writes since M1f S1f.6.35: an
/// `add` the program does not list is its own relation, which nothing
/// declares.
#[test]
fn an_imported_declaration_is_held_too() {
    let scratch = Scratch::new("ein-declared-counts-imports");
    std::fs::write(
        scratch.path().join("lib.ein"),
        "(relation likes Person Drink :why \"{?1} likes {?2}\")\n",
    )
    .expect("written");
    let cases: [(&str, &str, &str); 4] = [
        (
            "flat-stdlib",
            "(import std.math.decimal-arith :symbols (decimal-arith add n))\n(add (n z 5))\n",
            ":2:2: the fact (add (n z 5)) gives add 1 argument, and (relation add Operand \
             Operand) declares 2",
        ),
        (
            "qualified-stdlib",
            "(import std.math.decimal-arith :as a)\n(a.add (a.n z 5) (a.n 5))\n",
            ":2:19: the fact (a.add (a.n z 5) (a.n 5)) holds (a.n 5), which gives a.n 1 \
             argument, and (relation a.n Number Digit) declares 2",
        ),
        (
            "flat-file",
            "(import lib :symbols (likes))\n(likes Bob)\n",
            ":2:2: the fact (likes Bob) gives likes 1 argument, and (relation likes Person \
             Drink) declares 2",
        ),
        (
            "qualified-file",
            "(import lib :as o)\n(o.likes Bob)\n",
            ":2:2: the fact (o.likes Bob) gives o.likes 1 argument, and (relation o.likes",
        ),
    ];
    let mut bad = Vec::new();
    for (shape, text, want) in cases {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, text).expect("written");
        for b in refused_everywhere(&file, want, scratch.path()) {
            bad.push(format!("{shape}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// What the check does not hold, each of which loads on every command that
/// answers a program.
#[test]
fn what_is_not_a_declared_relations_form_loads() {
    let scratch = Scratch::new("ein-declared-counts-accepted");
    let accepted: [(&str, &str); 11] = [
        ("the-declared-counts", "(likes Bob Tea)\n(fan Bob)\n"),
        (
            "a-keyword-pair-is-no-argument",
            "(likes Bob Tea :source \"said\")\n(seen Bob :rule r :using ((likes Bob Tea)))\n",
        ),
        // A relation nothing declares has no count to be held to.
        (
            "an-undeclared-relation",
            "(says Ann)\n(says Ann Bob)\n(says)\n",
        ),
        // A relation written through a parameter is not read (Q-M1f.202).
        (
            "through-a-parameter",
            "(rule mirror (?R)\n  :match (?R ?a)\n  :assert (?R ?a ?a ?a)\n  \
             :why \"{?R} holds of {?a}\")\n(mirror likes)\n",
        ),
        // The declaration's own fact, read at any count (`01_grammar.md`).
        (
            "the-declaration-as-a-pattern",
            "(rule s ()\n  :match (relation likes ?t)\n  :assert (seen ?t)\n  \
             :why \"{?t} is a type of likes\")\n",
        ),
        // The kernel's markers are declared by no program (Q-M1f.203).
        ("a-kernel-marker", "(__closed__ likes extra)\n"),
        // A name is not a form.
        (
            "a-relations-name-as-a-value",
            "(says likes fan)\n(tag flag)\n",
        ),
        // A `(solve …)` list is a list of names, not a form.
        (
            "a-solve-list",
            "(hrule guess (?R ?T)\n  :match (is-a ?a ?T)\n  :assert (?R ?a)\n  \
             :why \"guess that {?R} holds of {?a}\")\n(is-a Bob Person)\n\
             (solve :hrules (guess (fan Person)) :no-hypothesis (likes))\n",
        ),
        (
            "a-solve-list-of-relations",
            "(solve :hypothesis-relations (likes fan flag))\n",
        ),
        // A macro no rule calls is not read (S1f.5.24's rule).
        (
            "an-unused-macro",
            "(macro liked (?a) (likes ?a) :why \"{?a} is liked\")\n",
        ),
        // The types are not checked against what is written.
        (
            "not-a-type-check",
            "(likes Tea Ann)\n(fan (likes Ann Tea))\n",
        ),
    ];
    let mut bad = Vec::new();
    for (shape, added) in accepted {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, format!("{HEAD}{added}")).expect("written");
        let f = file.display().to_string();
        let einb = scratch
            .path()
            .join(format!("{shape}.einb"))
            .display()
            .to_string();
        let runs: [Vec<&str>; 5] = [
            vec!["saturate", &f],
            vec!["solve", "--count", &f],
            vec!["oracle", &f],
            vec!["render", "program", &f],
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
