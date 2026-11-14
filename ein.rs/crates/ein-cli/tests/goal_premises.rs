//! **A query's `:goal` is held to what a rule's `:match` is held to** — M1f
//! S1f.7.5 (W77), S1f.7.11 (W78) and S1f.7.1 (W79), Q-M1f.117 to .121.
//!
//! A goal is a premise, and until these stages the loader read it less
//! closely than a rule's:
//!
//! - **a macro call in a goal was never expanded** (S1f.7.5): `(unknown (p
//!   ?x))` was a pattern over a relation `unknown`, so the goal had no row,
//!   `:expect (model)` held of it and the claim listing what the macro means
//!   was refused; without the import, in silence, where a rule is refused;
//! - **a goal over a relation the program names nowhere had no row**, and
//!   under an `(absent …)` held of everything (S1f.7.11), while a claim over a
//!   relation only a rule's `:assert` derives was refused;
//! - **a pattern inside a pattern whose relation is a variable matched
//!   nothing** (S1f.7.1), and under an `(absent …)` the goal held in every
//!   state, where a rule writing the shape is refused (W24).
//!
//! Since then the loader expands a goal's macro calls (`ein_ir::from_ir`'s
//! `expand_goal`), reads the expanded goal with the walks a rule's `:match`
//! gets, and holds the relations a goal and a claim name to the relations of
//! the program (`program_relations`). Each refusal is one first line, the
//! same on every command; what must keep loading is held beside it.

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

/// Two guests, of whom Ann has paid; `admitted` is named only by a rule's
/// `:assert`, `seen` only by an hrule's, which no activator reaches, `near`
/// only by being passed to
/// `functional`, and `(says (paid Bob))` holds a statement. No candidate
/// unless a program's `(solve …)` says otherwise.
const HEAD: &str = "(relation paid Guest :why \"{?1} has paid\")\n\
     (relation says Guest :why \"{?1} is said\")\n\
     (macro settled (?g) (paid ?g) :why \"{?g} has settled\")\n\
     (macro itself (?x) ?x :why \"{?x} itself\")\n\
     (macro lone (?x) (eq ?x) :why \"{?x} alone\")\n\
     (macro again (?x) (again ?x) :why \"{?x} again\")\n\
     (rule functional (?R)\n  :match (and (?R ?a ?b) (?R ?a ?c) (neq ?b ?c))\n  :assert (false)\n  \
     :why \"{?R} is not functional: {?a} has both {?b} and {?c}\")\n\
     (rule welcome ()\n  :match (paid ?g)\n  :assert (admitted ?g)\n  \
     :why \"{?g} has paid, so {?g} is admitted\")\n\
     (hrule notice (?T)\n  :match (is-a ?g ?T)\n  :assert (seen ?g)\n  \
     :why \"guess that {?g} is seen\")\n\
     (hrule guess (?R ?T)\n  :match (is-a ?a ?T)\n  :assert (?R ?a)\n  \
     :why \"guess that {?R} holds of {?a}\")\n\
     (functional near)\n\
     (is-a Ann Guest) (is-a Bob Guest)\n(paid Ann)\n(says (paid Bob))\n";

/// No candidate: R is root's state alone.
const NO_H: &str = "(solve :hypothesis-relations ())\n";

/// `(shape, the query, what the first line holds)`.
const REFUSED: [(&str, &str, &str); 19] = [
    // S1f.7.5: a call that cannot be expanded, and an expansion a rule's
    // `:match` would be refused for.
    (
        "macro-unimported-unknown",
        "(query :goal (and (is-a ?g Guest) (unknown (paid ?g))))",
        "query: '(unknown …)' is a std.macro pattern macro used without importing it — add \
         (import std.macro :symbols (unknown))",
    ),
    (
        "macro-unimported-forall",
        "(query :goal (forall ?g (is-a ?g Guest) (paid ?g)))",
        "query: '(forall …)' is a std.macro pattern macro used without importing it",
    ),
    (
        "macro-unimported-under-absent",
        "(query :goal (absent (unknown (paid Ann))))",
        "query: '(unknown …)' is a std.macro pattern macro used without importing it",
    ),
    (
        "macro-arity",
        "(query :goal (settled ?g Guest))",
        "query: its :goal: macro settled/1 invoked with 2 args at ",
    ),
    (
        "macro-expands-to-itself",
        "(query :goal (again ?g))",
        "query: its :goal: macro expansion exceeded depth 50",
    ),
    (
        "macro-leaves-no-premise",
        "(query :goal (and (paid ?g) (itself Ann)))",
        "query: its :goal writes Ann as a premise of and, and Ann is no premise",
    ),
    (
        "macro-writes-eq-of-one",
        "(query :goal (and (paid ?g) (lone ?g)))",
        "query: its :goal writes (eq ?g), and eq takes exactly 2 arguments, not 1",
    ),
    (
        "claim-lists-a-macro-call",
        "(query :goal (settled ?g) :expect (model (settled Ann)))",
        ":expect names settled, and the program names no relation settled",
    ),
    // S1f.7.11: a relation the program names nowhere.
    (
        "goal-relation",
        "(query :goal (payed ?g))",
        "query: its :goal writes (payed ?g), and the program names no relation payed — a \
         relation of the program is one it declares, states a fact of, or writes in a rule or \
         an hrule, so no state holds a fact of payed",
    ),
    (
        "goal-relation-under-absent",
        "(query :goal (and (is-a ?g Guest) (absent (payed ?g))))",
        "query: its :goal writes (payed ?g), and the program names no relation payed",
    ),
    (
        "goal-relation-negated",
        "(query :goal (not (payed ?g)))",
        "query: its :goal writes (payed ?g), and the program names no relation payed",
    ),
    (
        "goal-relation-in-a-disjunct",
        "(query :goal (or (paid ?g) (payed ?g)))",
        "query: its :goal writes (payed ?g), and the program names no relation payed",
    ),
    (
        "goal-relation-inside-a-pattern",
        "(query :goal (says (payed ?g)))",
        "query: its :goal writes (payed ?g), and the program names no relation payed",
    ),
    (
        "goal-relation-of-a-second-query",
        "(query :goal (paid ?g))\n(query :goal (payed ?g))",
        "query: its :goal writes (payed ?g), and the program names no relation payed",
    ),
    (
        "goal-relation-only-another-goal-names",
        "(query :goal (and (paid ?g) (absent (payed ?g))))\n(query :goal (payed Ann))",
        "query: its :goal writes (payed ?g), and the program names no relation payed",
    ),
    (
        "claim-relation",
        "(solve :expect (region (model (payed Ann))))",
        "(solve … :expect …) names payed, and the program names no relation payed — a relation \
         of the program is one it declares, states a fact of, or writes in a rule or an hrule",
    ),
    // S1f.7.1: a relation written as a variable, at any depth.
    (
        "relation-variable-inside-a-pattern",
        "(query :goal (says (?R ?g)))",
        "query: its :goal writes (?R ?g) with ?R as its relation, which nothing binds — a \
         relation is named by a name, or by a rule's parameter its activator binds, and a query \
         has no activator",
    ),
    (
        "relation-variable-inside-a-pattern-under-absent",
        "(query :goal (absent (says (?R ?g))))",
        "query: its :goal writes (?R ?g) with ?R as its relation, which nothing binds",
    ),
    (
        "relation-variable-two-deep",
        "(query :goal (and (paid ?g) (says (says (?R ?g)))))",
        "query: its :goal writes (?R ?g) with ?R as its relation, which nothing binds",
    ),
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
    (&["render", "rule", "--name", "welcome"], 1),
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
/// a load error holding `want`, and no container written.
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
                "ein {argv:?}: {line:?}\n  wants kb load error: … {want} …"
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
fn a_goal_a_rule_would_be_refused_for_is_refused_on_every_command() {
    let scratch = Scratch::new("ein-goal-premises");
    let mut bad = Vec::new();
    for (shape, query, want) in REFUSED {
        let file = scratch.path().join(format!("{shape}.ein"));
        let solve = if query.contains("(solve") { "" } else { NO_H };
        std::fs::write(&file, format!("{HEAD}{solve}{query}\n")).expect("written");
        for b in refused_everywhere(&file, want, scratch.path()) {
            bad.push(format!("{shape}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// A refusal that names a place names where the pattern writes its relation.
#[test]
fn a_goals_relation_is_refused_at_its_pattern() {
    let scratch = Scratch::new("ein-goal-premises-place");
    let file = scratch.path().join("placed.ein");
    std::fs::write(
        &file,
        "(relation paid Guest :why \"{?1} has paid\")\n(paid Ann)\n\
         (query :goal (and (paid ?g)\n                  (absent (payed ?g))))\n",
    )
    .expect("written");
    let f = file.display().to_string();
    let r = ein(&["saturate", &f]);
    let line = r.err.lines().next().unwrap_or("");
    assert!(
        line.starts_with(&format!(
            "kb load error: {f}:4:28: query: its :goal writes (payed ?g)"
        )),
        "{line}"
    );
}

/// The register's witnesses whose right answer is a refusal (W77, W78, W79),
/// each read from its file, as `wrong_answers.rs` reads one command's run.
#[test]
fn the_registers_witnesses_are_refused_on_every_command() {
    let scratch = Scratch::new("ein-goal-premise-witnesses");
    let witnesses: [(&str, &str); 6] = [
        (
            "goal-macro-unimported",
            "query: '(unknown …)' is a std.macro pattern macro used without importing it",
        ),
        (
            "goal-macro-arity",
            "query: its :goal: macro settled/1 invoked with 2 args at \
             examples/ein-bugs/wrong-answers/goal-macro-arity.ein:",
        ),
        (
            "goal-no-such-relation",
            "query: its :goal writes (payed ?g), and the program names no relation payed",
        ),
        (
            "goal-absent-no-such-relation",
            "query: its :goal writes (payed ?g), and the program names no relation payed",
        ),
        (
            "goal-nested-relation-variable",
            "query: its :goal writes (?R ?g) with ?R as its relation, which nothing binds",
        ),
        (
            "goal-absent-nested-relation-variable",
            "query: its :goal writes (?R ?g) with ?R as its relation, which nothing binds",
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

/// `(shape, what follows HEAD, the exit of `ein solve`, lines its stdout
/// holds)`. A macro call in a goal means what it means in a rule's `:match`,
/// the goal is printed as written, and a claim lists the expansion's
/// instances.
const ANSWERED: [(&str, &str, i32, &[&str]); 9] = [
    (
        "a-macro-of-the-programs",
        "(solve :hypothesis-relations ())\n(query :goal (settled ?g) :expect (model (paid Ann)))",
        0,
        &[
            "  query             (settled ?g)",
            "?g = Ann",
            "  as :expect        (model (paid Ann))",
            "  :expect           holds",
        ],
    ),
    (
        "the-claim-of-no-row-fails",
        "(solve :hypothesis-relations ())\n(query :goal (settled ?g) :expect (model))",
        1,
        &[
            "  query             (settled ?g)",
            "  :expect           FAILED",
        ],
    ),
    (
        "unknown-imported",
        "(import std.macro :symbols (unknown))\n(solve :hypothesis-relations ())\n\
         (query :goal (and (is-a ?g Guest) (unknown (paid ?g)))\n       \
         :expect (model (absent (not (paid Bob))) (absent (paid Bob)) (is-a Bob Guest)))",
        0,
        &[
            "  query             (and (is-a ?g Guest) (unknown (paid ?g)))",
            "?g = Bob",
            "  :expect           holds",
        ],
    ),
    (
        "forall-imported",
        "(import std.macro :symbols (forall))\n(solve :hypothesis-relations ())\n\
         (query :goal (and (is-a ?g Guest) (forall ?h (says (paid ?h)) (neq ?h ?g))))",
        0,
        &[
            "  query             (and (is-a ?g Guest) (forall ?h (says (paid ?h)) (neq ?h ?g)))",
            "?g = Ann",
        ],
    ),
    (
        "a-macro-in-a-goal-text",
        "(solve :hypothesis-relations ())\n\
         (query :goal (settled ?g) :goal-text \"{?g} has settled up\")",
        0,
        &["  query             (settled ?g)", "Ann has settled up"],
    ),
    (
        "a-claim-over-a-rules-conclusion",
        "(solve :hypothesis-relations ())\n\
         (query :goal (admitted ?g) :expect (model (admitted Ann)))",
        0,
        &["?g = Ann", "  :expect           holds"],
    ),
    (
        "a-region-claim-over-a-rules-conclusion",
        "(solve :hypothesis-relations ()\n       \
         :expect (region (model (admitted Ann) (paid Ann))))",
        0,
        &["  (solve :expect)   holds"],
    ),
    (
        "a-goal-over-what-an-activator-proposes",
        "(solve :hrules (guess (wanted Guest)))\n(query :goal (wanted ?g))",
        0,
        &["  query             (wanted ?g)", "?g = Ann", "?g = Bob"],
    ),
    (
        "a-claim-over-what-an-activator-proposes",
        "(solve :hrules (guess (wanted Guest))\n       :expect (region (model) \
         (model (wanted Ann)) (model (wanted Bob)) (model (wanted Ann) (wanted Bob))))\n\
         (query :goal (wanted ?g))",
        0,
        &["  (solve :expect)   holds"],
    ),
];

#[test]
fn a_goal_is_answered_as_a_rules_match_would_be() {
    let scratch = Scratch::new("ein-goal-premises-answered");
    let mut bad = Vec::new();
    for (shape, rest, code, lines) in ANSWERED {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, format!("{HEAD}{rest}\n")).expect("written");
        let f = file.display().to_string();
        let r = ein(&["solve", &f]);
        if r.code != code {
            bad.push(format!(
                "{shape}: ein solve: exit {}, want {code}: {}",
                r.code,
                r.err.lines().next().unwrap_or("")
            ));
            continue;
        }
        for want in lines {
            if !r.out.contains(want) {
                bad.push(format!("{shape}: ein solve prints no {want:?}:\n{}", r.out));
            }
        }
        // `ein test` agrees with `ein solve` on the claim, where one is made.
        if rest.contains(":expect") {
            let t = ein(&["test", &f]);
            if t.code != code {
                bad.push(format!(
                    "{shape}: ein test: exit {}, want {code}: {}{}",
                    t.code, t.out, t.err
                ));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// What a goal may ask about: every relation of the program, however it comes
/// by it, and the kernel's. Each loads on five commands.
#[test]
fn a_goal_over_a_relation_of_the_program_loads() {
    let goals: [(&str, &str); 14] = [
        ("declared", "(query :goal (paid ?g))"),
        ("a-rules-conclusion", "(query :goal (admitted ?g))"),
        ("an-hrules-conclusion", "(query :goal (seen ?g))"),
        ("passed-to-a-rule", "(query :goal (near ?a ?b))"),
        ("a-facts-own", "(query :goal (functional ?R))"),
        ("is-a", "(query :goal (is-a ?g Guest))"),
        (
            "a-guard",
            "(query :goal (and (is-a ?g Guest) (neq ?g Ann)))",
        ),
        ("the-declarations", "(query :goal (relation ?R ?T))"),
        ("a-stored-negation", "(query :goal (not (paid ?g)))"),
        ("inside-a-pattern", "(query :goal (says (paid ?g)))"),
        (
            "under-absent",
            "(query :goal (and (is-a ?g Guest) (absent (admitted ?g))))",
        ),
        ("a-disjunction", "(query :goal (or (paid ?g) (seen ?g)))"),
        ("a-macro-call", "(query :goal (settled ?g))"),
        (
            "a-macro-call-under-absent",
            "(query :goal (and (is-a ?g Guest) (absent (settled ?g))))",
        ),
    ];
    let scratch = Scratch::new("ein-goal-premises-accepted");
    let mut bad = Vec::new();
    let mut programs: Vec<(String, String)> = goals
        .iter()
        .map(|(shape, goal)| (shape.to_string(), format!("{HEAD}{NO_H}{goal}\n")))
        .collect();
    // A relation only an imported rule concludes, and an unused macro whose
    // expansion no goal could hold (Q-M1f.67: one no rule or goal calls loads).
    // The goal's `injective` is listed: a name the program writes and does
    // not list is its own since M1f S1f.6.35.
    programs.push((
        "an-imported-rules-conclusion".to_string(),
        "(import std.algebra :symbols (bijective-properties functional injective))\n\
         (relation r A B :why \"{?1} r {?2}\")\n(is-a a A) (is-a b B)\n(bijective r)\n\
         (query :goal (injective ?R))\n"
            .to_string(),
    ));
    programs.push((
        "an-unused-macro".to_string(),
        format!(
            "{HEAD}{NO_H}(macro odd (?x) (eq ?x) :why \"{{?x}} odd\")\n(query :goal (paid ?g))\n"
        ),
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
            vec!["solve", &f],
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
        // A container answers as the text does.
        let (text_run, einb_run) = (ein(&["solve", &f]), ein(&["solve", &einb]));
        let query = |s: &str| -> String {
            s.lines()
                .skip_while(|l| !l.starts_with("  query"))
                .collect::<Vec<_>>()
                .join("\n")
        };
        if query(&text_run.out) != query(&einb_run.out) {
            bad.push(format!(
                "{shape}: the container answers its queries otherwise:\n{}\n— the text:\n{}",
                query(&einb_run.out),
                query(&text_run.out)
            ));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
