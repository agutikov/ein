//! **A declaration of the blind enumerator that disagrees with the `open`
//! facts root holds is refused, on every command** (Q-M1f.578, Q-M1f.245,
//! Q-M1f.242;
//! [`07-diagnostics.md` § 2.26](../../../../docs/ein/user/04-reference/07-diagnostics.md#226-a-programs-hypothesis-mechanisms-do-not-contradict-one-another)).
//!
//! A hypothesis rule (H) and an activated obligation (O) write `open` facts at root;
//! `:hypothesis-relations` (HR), `:no-hypothesis` (NH) and a closure
//! `(__closed__ R)` (C) configure the blind enumerator, which does not run
//! beside them. Read on root's fixpoint, since a closure and an obligation's
//! activator may be derived ([`ein_infer::mechanisms::check`]), a list or a
//! closure that disagrees with root's `open` facts is a **conflict**, refused
//! at load with one line per relation, at the command's load-error exit, and
//! so are two lists that contradict each other; a **redundant** name, where
//! root holds no `open` fact, is one `note:` line on stderr and moves nothing;
//! H + O, the Zebra puzzle's form, is silent, and so is a list that agrees.
//! Held here on witnesses of every pair, on a word problem that writes two
//! conflicts at once, and on the corpus's fixture of the refusal.

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

/// `at` declared, two colours and two houses, `near` stated and undeclared.
const DECL: &str = "(relation at Color House :why \"{?3} is at {?4}\")\n\
     (is-a Red Color) (is-a Blue Color) (is-a H1 House) (is-a H2 House)\n\
     (near H1 H2)\n";
/// The hypothesis rule `solve_lists.rs` uses, whose `?R` names the relation it proposes.
const GUESS: &str = "(rule guess (?R ?T1 ?T2)\n  :match (and (is-a ?a ?T1) (is-a ?b ?T2))\n  \
     :assert (open (?R ?a ?b))\n  :why \"guess that {?R} holds of {?a} and {?b}\")\n";
/// `std.algebra`'s `total-owed`, imported and activated on `at`.
const OWED: &str = "(import std.algebra :symbols (total-owed))\n";
const OWE_AT: &str = "(total-owed at is-a)\n";
/// A rule that derives the closure of `at`.
const CLOSE_AT: &str = "(rule close-at ()\n  :match (is-a Red Color)\n  :assert (__closed__ at)\n  \
     :why \"Red is a Color, so at is closed\")\n";
/// A claim, so that `ein test` reads the program.
const CLAIM: &str = "(query :goal (at Red ?h) :expect (model))\n";

fn program(parts: &[&str]) -> String {
    let mut s: String = parts.concat();
    s.push_str(CLAIM);
    s
}

/// `(shape, the program, what the first line says after its position)`.
fn refused() -> Vec<(&'static str, String, &'static str)> {
    let h = "(guess at Color House) (solve";
    vec![
        (
            "h-hr",
            program(&[DECL, GUESS, h, " :hypothesis-relations ())\n"]),
            "solve: its :hypothesis-relations leaves out at, and root holds (open (at ",
        ),
        (
            "h-nh",
            program(&[DECL, GUESS, h, " :no-hypothesis (at))\n"]),
            "solve: its :no-hypothesis names at, and root holds (open (at ",
        ),
        (
            "h-c",
            program(&[DECL, GUESS, "(__closed__ at)\n", h, ")\n"]),
            "root holds (open (at ",
        ),
        (
            "h-c-derived",
            program(&[DECL, GUESS, CLOSE_AT, h, ")\n"]),
            "which rule 'guess' writes, and (__closed__ at), which rule 'close-at' derives, says \
             at is closed",
        ),
        (
            "o-hr",
            program(&[
                OWED,
                DECL,
                OWE_AT,
                "(solve :hypothesis-relations (near at))\n",
            ]),
            "solve: its :hypothesis-relations names near, and root holds no open fact of near",
        ),
        (
            "o-hr-empty",
            program(&[OWED, DECL, OWE_AT, "(solve :hypothesis-relations ())\n"]),
            "solve: its :hypothesis-relations leaves out at, and root holds (open at)",
        ),
        (
            "o-nh",
            program(&[OWED, DECL, OWE_AT, "(solve :no-hypothesis (at))\n"]),
            "solve: its :no-hypothesis names at, and root holds (open at), which obligation \
             'total-owed' writes",
        ),
        (
            "o-c",
            program(&[OWED, DECL, OWE_AT, "(__closed__ at)\n(solve)\n"]),
            "root holds (open at), which obligation 'total-owed' writes, and (__closed__ at), \
             written at ",
        ),
        (
            "o-c-derived-beside-a-hypothesis-rule",
            program(&[
                OWED,
                DECL,
                GUESS,
                OWE_AT,
                CLOSE_AT,
                "(guess near House House)\n",
            ]),
            "root holds (open at), which obligation 'total-owed' writes, and (__closed__ at), \
             which rule 'close-at' derives, says at is closed",
        ),
        (
            "o-nh-beside-a-hypothesis-rule",
            program(&[
                OWED,
                DECL,
                GUESS,
                OWE_AT,
                "(guess near House House) (solve :no-hypothesis (at))\n",
            ]),
            "solve: its :no-hypothesis names at, and root holds (open at), which obligation \
             'total-owed' writes",
        ),
        (
            "h-hr-names-more",
            program(&[DECL, GUESS, h, " :hypothesis-relations (at near))\n"]),
            "solve: its :hypothesis-relations names near, and root holds no open fact of near",
        ),
        (
            "hr-nh",
            program(&[
                DECL,
                "(solve :hypothesis-relations (at near) :no-hypothesis (at))\n",
            ]),
            "solve: its :no-hypothesis names at, and its :hypothesis-relations names at too",
        ),
        (
            "hr-c",
            program(&[
                DECL,
                "(__closed__ at)\n(solve :hypothesis-relations (at near))\n",
            ]),
            "solve: its :hypothesis-relations names at, and (__closed__ at), written at ",
        ),
    ]
}

/// `(shape, the program, the one note it prints, after its position)`; an
/// empty note is a program that prints none.
fn accepted() -> Vec<(&'static str, String, String)> {
    let h = "(guess at Color House) (solve";
    vec![
        // H + O, the zebra form: silent.
        ("h-o", program(&[OWED, DECL, GUESS, OWE_AT, h, ")\n"]), ""),
        // C's other role: a hypothesis rule beside the closure of another relation.
        (
            "h-c-other",
            program(&[DECL, GUESS, "(__closed__ near)\n", h, ")\n"]),
            "",
        ),
        // A list that agrees with root's `open` facts says nothing.
        (
            "o-hr-listed",
            program(&[OWED, DECL, OWE_AT, "(solve :hypothesis-relations (at))\n"]),
            "",
        ),
        (
            "o-nh-other",
            program(&[OWED, DECL, OWE_AT, "(solve :no-hypothesis (near))\n"]),
            "",
        ),
        (
            "o-c-other",
            program(&[OWED, DECL, OWE_AT, "(__closed__ near)\n(solve)\n"]),
            "",
        ),
        (
            "hr-c-other",
            program(&[
                DECL,
                "(__closed__ near)\n(solve :hypothesis-relations (at))\n",
            ]),
            "",
        ),
        (
            "h-hr-agrees",
            program(&[DECL, GUESS, h, " :hypothesis-relations (at))\n"]),
            "",
        ),
        (
            "h-nh-other",
            program(&[DECL, GUESS, h, " :no-hypothesis (near))\n"]),
            "",
        ),
        // A hypothesis rule no clause activates writes nothing: the list is the blind
        // enumerator's.
        (
            "h-inactive-nh",
            program(&[DECL, GUESS, "(solve :no-hypothesis (near))\n"]),
            "",
        ),
        (
            "hr-nh-redundant",
            program(&[
                DECL,
                "(solve :hypothesis-relations (at) :no-hypothesis (near))\n",
            ]),
            "solve: its :no-hypothesis names near, which its :hypothesis-relations already \
             leaves out",
        ),
        (
            "nh-c-redundant",
            program(&[DECL, "(__closed__ at)\n(solve :no-hypothesis (at))\n"]),
            "solve: its :no-hypothesis names at, which (__closed__ at), written at ",
        ),
    ]
    .into_iter()
    .map(|(shape, text, note): (&str, String, &str)| (shape, text, note.to_string()))
    .collect()
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

/// `file` refused on every command with the same lines, the first starting
/// `kb load error: <file>:` and holding `want`, and no
/// container written. Returns what went wrong, and the lines the first
/// command printed.
fn refused_everywhere(file: &Path, want: &str, dir: &Path) -> (Vec<String>, Vec<String>) {
    let mut bad = Vec::new();
    let einb = dir.join("x.einb");
    let events = dir.join("events.jsonl");
    let shown = file.display().to_string();
    let mut first: Option<Vec<String>> = None;
    for (cmd, code) in COMMANDS {
        let args = argv(cmd, file, &einb, &events);
        let argv: Vec<&str> = args.iter().map(String::as_str).collect();
        let r = ein(&argv);
        let lines: Vec<String> = r.err.lines().map(str::to_string).collect();
        let line = lines.first().cloned().unwrap_or_default();
        if r.code != code {
            bad.push(format!(
                "ein {argv:?}: exit {}, want {code}: {line}",
                r.code
            ));
            continue;
        }
        if !(line.starts_with(&format!("kb load error: {shown}:")) && line.contains(want)) {
            bad.push(format!(
                "ein {argv:?}: {line:?}\n  wants kb load error: {shown}:… holding {want:?}"
            ));
        }
        if let Some(l) = lines.iter().find(|l| !l.starts_with("kb load error: ")) {
            bad.push(format!("ein {argv:?}: a line that is no conflict's: {l:?}"));
        }
        match &first {
            None => first = Some(lines),
            Some(f) if *f != lines => bad.push(format!(
                "ein {argv:?}: {lines:?}, where the first command printed {f:?}"
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
    (bad, first.unwrap_or_default())
}

#[test]
fn a_conflict_is_refused_on_every_command() {
    let scratch = Scratch::new("ein-mechanism-conflicts");
    let mut bad = Vec::new();
    for (shape, text, want) in refused() {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, &text).expect("written");
        let (b, lines) = refused_everywhere(&file, want, scratch.path());
        bad.extend(b.into_iter().map(|b| format!("{shape}: {b}")));
        if lines.len() != 1 {
            bad.push(format!(
                "{shape}: {} lines, want one: {lines:?}",
                lines.len()
            ));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// A dead or redundant declaration is one `note:` line on stderr, on every
/// command that loads the program, and the program answers; a pair with no
/// contradiction says nothing. `kb save`'s container carries the program,
/// and opening it says the same.
#[test]
fn a_dead_or_redundant_list_is_a_note_and_a_silent_pair_says_nothing() {
    let scratch = Scratch::new("ein-mechanism-notes");
    let mut bad = Vec::new();
    for (shape, text, note) in accepted() {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, &text).expect("written");
        let f = file.display().to_string();
        let einb = scratch
            .path()
            .join(format!("{shape}.einb"))
            .display()
            .to_string();
        let runs: [Vec<&str>; 6] = [
            vec!["saturate", &f],
            vec!["solve", "--count", &f],
            vec!["render", "program", &f],
            vec!["render", "kb", &f],
            vec!["kb", "save", &f, &einb],
            vec!["solve", "--count", &einb],
        ];
        for argv in runs {
            let r = ein(&argv);
            if r.code != 0 {
                bad.push(format!(
                    "{shape}: ein {argv:?}: exit {}: {}",
                    r.code,
                    r.err.lines().next().unwrap_or("")
                ));
                continue;
            }
            let notes: Vec<&str> = r
                .err
                .lines()
                .filter(|l| l.starts_with("note: ") && l.contains("solve: its :"))
                .collect();
            let ok = if note.is_empty() {
                notes.is_empty()
            } else {
                notes.len() == 1 && notes[0].contains(note.as_str())
            };
            if !ok {
                bad.push(format!(
                    "{shape}: ein {argv:?}: notes {notes:?}, want {note:?}"
                ));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **A word problem that writes two conflicts**: *what is not listed is
/// zero* adopted for `has`, a hypothesis rule supposing Alice has an apple,
/// and `(solve :no-hypothesis (has))`, a list that does nothing beside a
/// hypothesis rule. It is refused for what it says: two conflicts,
/// `maybe-apples` proposing `has` against the list that names it and against
/// the closure `words` derives, and no other line. Its twin without
/// `listed_is_all` is refused for the same two.
#[test]
fn a_word_problem_is_refused_for_its_two_conflicts() {
    let scratch = Scratch::new("ein-mechanism-e47");
    for module in ["words", "gaps", "listed_is_all"] {
        std::fs::copy(
            repo_root().join(format!("examples/word_problems/{module}.ein")),
            scratch.path().join(format!("{module}.ein")),
        )
        .expect("a word-problem module");
    }
    let facts = "(has Bob apples (n z 5) now    :source \"Bob has 5 apples,\")\n\
         (has Alice pears (n z 3) now   :source \"Alice has 3 pears.\")\n\
         (asked (together Bob Alice apples now) :source \"How many apples do they have together?\")\n\
         (rule maybe-apples () :match (has ?p pears ?v ?t) :assert (open (has ?p apples (n z 1) ?t)) \
         :why \"suppose {?p} has 1 apple as well\")\n\
         (solve :no-hypothesis (has))\n\
         (query :goal (and (asked ?q) (value ?q ?n)) :goal-text \"{?q}: {?n}\"\n\
         :expect (model (asked (together Bob Alice apples now))\n\
         (value (together Bob Alice apples now) (n z 5))))\n";
    // The names the program writes, each listed from the module that declares
    // it.
    const IMPORTS: &str = "(import words :symbols (words has asked together value))\n\
         (import std.math.decimal :symbols (n))\n(import std.math.arith :symbols (z))\n\
         (import gaps :symbols (gaps))\n";
    let witnesses = [
        (
            "kinds_listed_has_hyp_stated",
            format!("{IMPORTS}(import listed_is_all :symbols (listed-is-all))\n{facts}"),
        ),
        ("nolisted_has_hyp_stated", format!("{IMPORTS}{facts}")),
    ];
    let mut bad = Vec::new();
    for (stem, text) in witnesses {
        let file = scratch.path().join(format!("{stem}.ein"));
        std::fs::write(&file, text).expect("written");
        let (b, lines) = refused_everywhere(
            &file,
            "solve: its :no-hypothesis names has, and root holds (open (has ",
            scratch.path(),
        );
        bad.extend(b.into_iter().map(|b| format!("{stem}: {b}")));
        let second = "which rule 'maybe-apples' writes, and (__closed__ has), which rule 'words' \
                      derives, says has is closed";
        if lines.len() != 2 || !lines[1].contains(second) {
            bad.push(format!(
                "{stem}: {lines:?}, want two lines, the second {second:?}"
            ));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The corpus's fixture of the refusal, `examples/ein-bugs/closed-and-owing.ein`:
/// an obligation on a relation `infer-closure` closes.
#[test]
fn the_corpus_fixture_is_refused_on_every_command() {
    let scratch = Scratch::new("ein-mechanism-fixture");
    let (bad, lines) = refused_everywhere(
        Path::new("examples/ein-bugs/closed-and-owing.ein"),
        "root holds (open r), which obligation 'total-owed' writes, and (__closed__ r), which \
         rule 'infer-closure' derives, says r is closed",
        scratch.path(),
    );
    let mut bad = bad;
    if lines.len() != 1
        || !lines[0]
            .contains("and (__closed__ r), which rule 'infer-closure' derives, says r is closed")
    {
        bad.push(format!("{lines:?}"));
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

// ── `(solve :no-search)` beside each mechanism ───────────────────────

/// `text` with ` :no-search` written last in its `(solve …)`, which every
/// position before it keeps; a `(solve …)` form is written where it has none.
fn with_no_search(text: &str) -> String {
    let Some(start) = text.find("(solve") else {
        return text.replacen("(query", "(solve :no-search)\n(query", 1);
    };
    let mut depth = 0i32;
    for (i, c) in text[start..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    let at = start + i;
                    return format!("{} :no-search{}", &text[..at], &text[at..]);
                }
            }
            _ => {}
        }
    }
    unreachable!("a (solve …) form closes")
}

/// **`:no-search` relaxes no conflict** (Q-M1f.254): every shape the check
/// refuses is refused beside it with the same line, the
/// hypothesis rule read as declared as an obligation is.
#[test]
fn no_search_relaxes_no_conflict() {
    let scratch = Scratch::new("ein-mechanism-no-search-conflicts");
    let mut bad = Vec::new();
    for (shape, text, _) in refused() {
        let without = scratch.path().join(format!("{shape}.ein"));
        let beside = scratch.path().join(format!("{shape}-ns.ein"));
        std::fs::write(&without, &text).expect("written");
        std::fs::write(&beside, with_no_search(&text)).expect("written");
        for cmd in [&["solve"][..], &["test"], &["saturate"]] {
            let w = ein(&[cmd, &[without.to_str().expect("utf-8")]].concat());
            let b = ein(&[cmd, &[beside.to_str().expect("utf-8")]].concat());
            let rename = |s: &str| s.replace(&format!("{shape}-ns.ein"), &format!("{shape}.ein"));
            if w.code != b.code || w.err != rename(&b.err) || !b.err.starts_with("kb load error: ")
            {
                bad.push(format!(
                    "{shape} {cmd:?}: exit {} beside :no-search, {} without\n  beside: {}\n  without: {}",
                    b.code, w.code, b.err, w.err
                ));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The parts, with no claim: a claim is about R, which these state.
fn bare(parts: &[&str]) -> String {
    parts.concat()
}

/// **What `:no-search` makes dead is one note each, and beside an obligation
/// or a closure it is silent**: a hypothesis rule *proposes no candidate*, a list
/// *filters no candidate*.
/// Each answers R as root alone: {∅}, or empty where root owes.
#[test]
fn no_search_is_silent_beside_an_obligation_and_a_note_beside_what_it_makes_dead() {
    let scratch = Scratch::new("ein-mechanism-no-search-notes");
    let h = "(guess at Color House) (solve :no-search)\n";
    let rule = "its :no-search states the lattice is root alone, so rule 'guess' proposes no \
                 candidate";
    let list = |key: &str| {
        format!(
            "its :{key} filters no candidate — the program states :no-search, whose lattice is \
             root alone"
        )
    };
    let cases: Vec<(&str, String, Vec<String>, &str)> = vec![
        (
            "alone",
            bare(&[DECL, "(solve :no-search)\n"]),
            vec![],
            "in R",
        ),
        (
            "o",
            bare(&[OWED, DECL, OWE_AT, "(solve :no-search)\n"]),
            vec![],
            "owes",
        ),
        (
            "o-paid",
            bare(&[
                OWED,
                DECL,
                OWE_AT,
                "(at Red H1) (at Blue H2)\n(solve :no-search)\n",
            ]),
            vec![],
            "in R",
        ),
        (
            "c",
            bare(&[DECL, "(__closed__ at)\n(solve :no-search)\n"]),
            vec![],
            "in R",
        ),
        ("h", bare(&[DECL, GUESS, h]), vec![rule.to_string()], "in R"),
        (
            "h-o",
            bare(&[OWED, DECL, GUESS, OWE_AT, h]),
            vec![rule.to_string()],
            "owes",
        ),
        (
            "hr",
            bare(&[DECL, "(solve :hypothesis-relations (at) :no-search)\n"]),
            vec![list("hypothesis-relations")],
            "in R",
        ),
        (
            "nh",
            bare(&[DECL, "(solve :no-hypothesis (near) :no-search)\n"]),
            vec![list("no-hypothesis")],
            "in R",
        ),
        (
            "h-nh",
            bare(&[
                DECL,
                GUESS,
                "(guess at Color House) (solve :no-hypothesis (near) :no-search)\n",
            ]),
            vec![rule.to_string(), list("no-hypothesis")],
            "in R",
        ),
    ];
    let mut bad = Vec::new();
    for (shape, text, notes, root) in cases {
        let file = scratch.path().join(format!("ns-{shape}.ein"));
        std::fs::write(&file, &text).expect("written");
        let f = file.to_str().expect("utf-8");
        let r = ein(&["solve", f]);
        // `ein` keeps stderr; stdout, for the rows, is read again.
        let out = Command::new(env!("CARGO_BIN_EXE_ein"))
            .args(["solve", f])
            .current_dir(repo_root())
            .output()
            .expect("the `ein` binary runs");
        let stdout = String::from_utf8_lossy(&out.stdout);
        if r.code != 0 {
            bad.push(format!("{shape}: exit {}: {}", r.code, r.err));
            continue;
        }
        let got: Vec<&str> = r
            .err
            .lines()
            .filter(|l| l.starts_with("note: ") && l.contains("solve: its :"))
            .collect();
        let ok = got.len() == notes.len() && notes.iter().zip(&got).all(|(w, g)| g.contains(w));
        if !ok {
            bad.push(format!("{shape}: notes {got:?}, want {notes:?}"));
        }
        if !stdout.contains("no candidate: the program states :no-search") {
            bad.push(format!(
                "{shape}: the universe row does not say :no-search:\n{stdout}"
            ));
        }
        let standing = match root {
            "in R" => stdout.contains("in R — root is alive and owes nothing"),
            _ => stdout.contains("every alive node owes"),
        };
        if !standing {
            bad.push(format!("{shape}: root is not {root}:\n{stdout}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
