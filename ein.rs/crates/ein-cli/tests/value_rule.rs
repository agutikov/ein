//! **A value is a name, or a fact a state could hold** — M1f S1f.7.7,
//! S1f.7.6, S1f.7.10 and S1f.7.3 (W80 to W83, Q-M1f.113, the user's ruling of
//! 2026-11-11, and Q-M1f.122 to .126).
//!
//! *A value is a name, or a fact a state could hold: an atom of an ordinary
//! relation, or its negation.* So no fact, value or conclusion is headed
//! `absent`, which is negation as failure, a premise's test (S1f.7.7);
//! `false` takes no argument anywhere (S1f.7.6); `(eq …)`, computed and never
//! stored, is no value, as `(neq …)` never was (S1f.7.10); and a pattern's
//! argument is what a value can be, so a connective there is refused once the
//! macros are expanded (S1f.7.3). Until then each loaded: `(absent (p a))`
//! written where `(not (p a))` was meant left the state alive, `(false k)`
//! was stored and killed root, a pattern's argument `(eq ?a ?b)` matched a
//! stored `(eq k j)` by its shape, and `(q (and (p ?x)))` matched nothing, so
//! under an `(absent …)` the rule fired.
//!
//! The grammar refuses each shape a program writes (`ein.y`'s `FactHead`,
//! `ValueHead`, `Premise`, `Negated`), with a note; the loader refuses what
//! only it can see (`ein_ir::from_ir`'s `Shapes` walk, after the macros are
//! expanded, and a relation parameter its activator binds to one of the three
//! words). Every refusal is held here on every command, and beside it what
//! must keep loading: `(false)`, a stored negation as a value, `(not (eq a
//! b))` as a fact, a premise and a conclusion of its own, `absent` in a
//! claim's class, the three words as plain names, and a macro called with
//! premises.

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

/// Five relations, one fact, and a rule `mk` whose parameter names a relation.
const HEAD: &str = "(relation p x :why \"{?1} is p\")\n\
     (relation q x :why \"{?1} is q\")\n\
     (relation h x :why \"{?1} is h\")\n\
     (relation hit a b :why \"{?1} hits {?2}\")\n\
     (relation won x :why \"{?1} won\")\n\
     (p a)\n";

const ABSENT_NOTE: &str = "note: absent is negation as failure";
const FALSE_NOTE: &str = "note: false takes no argument";
const EQ_NOTE: &str = "note: eq is computed and never stored";
const NEQ_NOTE: &str = "note: neq is computed and never stored";

/// `(shape, the forms after HEAD, what the first line of stderr holds, what
/// stderr holds besides)`.
const REFUSED: [(&str, &str, &str, &str); 52] = [
    // ── S1f.7.7: `absent` heads no fact, value or conclusion ──
    (
        "absent-fact",
        "(absent k)",
        "unexpected `absent`",
        ABSENT_NOTE,
    ),
    (
        "absent-fact-of-a-fact",
        "(absent (p a))",
        "unexpected `absent`",
        "the stored negation of a fact is (not …)",
    ),
    (
        "absent-nested",
        "(q (absent k))",
        "unexpected `absent`",
        ABSENT_NOTE,
    ),
    (
        "absent-negated",
        "(not (absent k))",
        "unexpected `absent`",
        ABSENT_NOTE,
    ),
    (
        "absent-using",
        "(q a :rule r :using ((absent k)))",
        "unexpected `absent`",
        ABSENT_NOTE,
    ),
    (
        "absent-concluded",
        "(rule r () :match (p ?x) :assert (absent (p ?x)) :why \"w\")",
        "unexpected `absent`",
        ABSENT_NOTE,
    ),
    (
        "absent-hrule-concluded",
        "(hrule r () :match (p ?x) :assert (absent ?x) :why \"w\")",
        "unexpected `absent`",
        ABSENT_NOTE,
    ),
    (
        "absent-concluded-value",
        "(rule r () :match (p ?x) :assert (q (absent ?x)) :why \"w\")",
        "unexpected `absent`",
        ABSENT_NOTE,
    ),
    (
        "absent-concluded-negated",
        "(rule r () :match (p ?x) :assert (not (absent ?x)) :why \"w\")",
        "unexpected `absent`",
        ABSENT_NOTE,
    ),
    (
        "absent-premise-negated",
        "(rule r () :match (and (p ?x) (not (absent ?x))) :assert (won ?x) :why \"w\")",
        "unexpected `absent`",
        ABSENT_NOTE,
    ),
    (
        "absent-macro-concluded",
        "(macro m (?a) (absent ?a) :why \"m\")\n\
         (rule r () :match (p ?x) :assert (m ?x) :why \"w\")",
        "rule 'r': its :assert writes (absent ?x), and absent is negation as failure, a \
         premise's test, and no fact is of it",
        "",
    ),
    (
        "absent-pattern-argument",
        "(rule r () :match (and (p ?x) (q (absent (q ?x)))) :assert (won ?x) :why \"w\")",
        "rule 'r': its :match writes (q (absent (q ?x))), and (absent (q ?x)) is written where \
         a value goes — absent is negation as failure",
        "",
    ),
    (
        "absent-parameter",
        "(mk absent)\n(rule mk (?R) :match (p ?x) :assert (?R ?x) :why \"w\")",
        "rule 'mk': its parameter ?R heads (?R ?x), a form of 1 argument, and the fact (mk \
         absent) binds ?R to absent — absent is negation as failure",
        "",
    ),
    (
        "absent-parameter-premise",
        "(mk absent)\n\
         (rule mk (?R) :match (and (p ?x) (?R (q ?x))) :assert (won ?x) :why \"w\")",
        "rule 'mk': its parameter ?R heads (?R (q ?x)), a form of 1 argument, and the fact (mk \
         absent) binds ?R to absent",
        "",
    ),
    (
        "absent-parameter-value",
        "(mk absent)\n(rule mk (?R) :match (p ?x) :assert (q (?R ?x)) :why \"w\")",
        "rule 'mk': its parameter ?R heads (?R ?x), written where a value goes, and the fact \
         (mk absent) binds ?R to absent",
        "",
    ),
    // ── S1f.7.6: `false` takes no argument ──
    (
        "false-fact",
        "(false k)",
        "unexpected `k`: a name",
        FALSE_NOTE,
    ),
    (
        "false-nested",
        "(q (false k))",
        "unexpected `k`: a name",
        FALSE_NOTE,
    ),
    (
        "false-negated",
        "(not (false k))",
        "unexpected `k`: a name",
        FALSE_NOTE,
    ),
    (
        "false-premise",
        "(rule r () :match (and (p ?x) (false ?s)) :assert (won ?x) :why \"w\")",
        "unexpected `?s`: a variable",
        FALSE_NOTE,
    ),
    (
        "false-premise-under-absent",
        "(rule r () :match (and (p ?x) (absent (false ?x))) :assert (won ?x) :why \"w\")",
        "unexpected `?x`: a variable",
        FALSE_NOTE,
    ),
    (
        "false-premise-negated",
        "(rule r () :match (and (p ?x) (not (false k))) :assert (won ?x) :why \"w\")",
        "unexpected `k`: a name",
        FALSE_NOTE,
    ),
    (
        "false-pattern-argument",
        "(rule r () :match (and (p ?x) (q (false ?s))) :assert (won ?x) :why \"w\")",
        "unexpected `?s`: a variable",
        FALSE_NOTE,
    ),
    (
        "false-goal",
        "(query :goal (false ?s))",
        "unexpected `?s`: a variable",
        FALSE_NOTE,
    ),
    (
        "false-claim",
        "(query :goal (p ?x) :expect (model (false k)))",
        "unexpected `k`: a name",
        FALSE_NOTE,
    ),
    (
        "false-concluded-value",
        "(rule r () :match (p ?x) :assert (q (false ?x)) :why \"w\")",
        "unexpected `?x`: a variable",
        FALSE_NOTE,
    ),
    (
        "false-concluded-negated",
        "(rule r () :match (p ?x) :assert (not (false ?x)) :why \"w\")",
        "unexpected `?x`: a variable",
        FALSE_NOTE,
    ),
    (
        "false-macro-premise",
        "(macro m (?a) (false ?a) :why \"m\")\n\
         (rule r () :match (and (p ?x) (absent (m ?x))) :assert (won ?x) :why \"w\")",
        "rule 'r': its :match writes (false ?x), and false takes no argument, not 1",
        "",
    ),
    (
        "false-macro-pattern-argument",
        "(macro m (?a) (q (false ?a)) :why \"m\")\n\
         (rule r () :match (and (p ?x) (m ?x)) :assert (won ?x) :why \"w\")",
        "rule 'r': its :match writes (q (false ?x)), and (false ?x) is written where a value \
         goes — false takes no argument",
        "",
    ),
    (
        "false-macro-goal",
        "(macro m (?a) (false ?a) :why \"m\")\n(query :goal (and (p ?x) (absent (m ?x))))",
        "query: its :goal writes (false ?x), and false takes no argument, not 1",
        "",
    ),
    (
        "false-parameter",
        "(mk false)\n(rule mk (?R) :match (p ?x) :assert (?R ?x) :why \"w\")",
        "rule 'mk': its parameter ?R heads (?R ?x), a form of 1 argument, and the fact (mk \
         false) binds ?R to false — false takes no argument",
        "",
    ),
    (
        "false-parameter-premise",
        "(mk false)\n(rule mk (?R) :match (and (p ?x) (?R ?x)) :assert (won ?x) :why \"w\")",
        "rule 'mk': its parameter ?R heads (?R ?x), a form of 1 argument, and the fact (mk \
         false) binds ?R to false",
        "",
    ),
    // ── S1f.7.10: `(eq …)` is no value, as `(neq …)` is none ──
    ("eq-nested", "(q (eq k j))", "unexpected `eq`", EQ_NOTE),
    ("neq-nested", "(q (neq k j))", "unexpected `neq`", NEQ_NOTE),
    (
        "eq-nested-negated",
        "(q (not (eq k j)))",
        "unexpected `eq`",
        EQ_NOTE,
    ),
    (
        "eq-using",
        "(q a :rule r :using ((eq a a)))",
        "unexpected `eq`",
        EQ_NOTE,
    ),
    (
        "eq-concluded-value",
        "(rule r () :match (and (p ?x) (p ?y)) :assert (q (eq ?x ?y)) :why \"w\")",
        "unexpected `eq`",
        EQ_NOTE,
    ),
    (
        "eq-concluded-negated-value",
        "(rule r () :match (p ?x) :assert (q (not (eq ?x b))) :why \"w\")",
        "unexpected `eq`",
        EQ_NOTE,
    ),
    (
        "eq-guard-value",
        "(rule r () :match (and (p ?x) (eq ?y (eq ?x b))) :assert (won ?y) :why \"w\")",
        "unexpected `eq`",
        EQ_NOTE,
    ),
    (
        "eq-claim-value",
        "(query :goal (p ?x) :expect (model (p (eq a b))))",
        "unexpected `eq`",
        EQ_NOTE,
    ),
    (
        "eq-pattern-argument",
        "(rule r () :match (q (eq ?a ?b)) :assert (hit ?a ?b) :why \"w\")",
        "rule 'r': its :match writes (q (eq ?a ?b)), and (eq ?a ?b) is written where a value \
         goes — eq is computed and never stored",
        "",
    ),
    (
        "eq-pattern-argument-negated",
        "(rule r () :match (and (p ?x) (q (not (eq ?x b)))) :assert (won ?x) :why \"w\")",
        "rule 'r': its :match writes (not (eq ?x b)), and (eq ?x b) is written where a value \
         goes — eq is computed and never stored",
        "",
    ),
    (
        "eq-goal-pattern-argument",
        "(query :goal (q (eq ?a ?b)))",
        "query: its :goal writes (q (eq ?a ?b)), and (eq ?a ?b) is written where a value goes",
        "",
    ),
    (
        "eq-macro-concluded-value",
        "(macro m (?a) (q (eq ?a ?a)) :why \"m\")\n\
         (rule r () :match (p ?x) :assert (m ?x) :why \"w\")",
        "rule 'r': its :assert writes (q (eq ?x ?x)), and (eq ?x ?x) is written where a value \
         goes — eq is computed and never stored",
        "",
    ),
    (
        "eq-parameter-value",
        "(mk eq)\n(rule mk (?R) :match (p ?x) :assert (q (?R ?x ?x)) :why \"w\")",
        "rule 'mk': its parameter ?R heads (?R ?x ?x), written where a value goes, and the \
         fact (mk eq) binds ?R to eq — eq is computed and never stored",
        "",
    ),
    (
        "eq-parameter-pattern-argument",
        "(mk eq)\n\
         (rule mk (?R) :match (and (p ?x) (q (?R ?x ?y))) :assert (hit ?x ?y) :why \"w\")",
        "rule 'mk': its parameter ?R heads (?R ?x ?y), written where a value goes, and the \
         fact (mk eq) binds ?R to eq",
        "",
    ),
    // ── S1f.7.3: a connective as a pattern's argument ──
    (
        "and-pattern-argument",
        "(rule r () :match (and (p ?x) (q (and (p ?x)))) :assert (won ?x) :why \"w\")",
        "rule 'r': its :match writes (q (and (p ?x))), and (and (p ?x)) is a connective where \
         a value goes — and heads no fact",
        "",
    ),
    (
        "and-pattern-argument-under-absent",
        "(rule r () :match (and (p ?x) (absent (q (and (p ?x))))) :assert (won ?x) :why \"w\")",
        "rule 'r': its :match writes (q (and (p ?x))), and (and (p ?x)) is a connective where \
         a value goes",
        "",
    ),
    (
        "or-pattern-argument",
        "(rule r () :match (and (p ?x) (absent (q (or (p ?x) (q ?x))))) :assert (won ?x) \
         :why \"w\")",
        "rule 'r': its :match writes (q (or (p ?x) (q ?x))), and (or (p ?x) (q ?x)) is a \
         connective where a value goes — or heads no fact",
        "",
    ),
    (
        "neq-pattern-argument",
        "(rule r () :match (and (p ?x) (absent (q (neq ?x b)))) :assert (won ?x) :why \"w\")",
        "rule 'r': its :match writes (q (neq ?x b)), and (neq ?x b) is a connective where a \
         value goes — neq heads no fact",
        "",
    ),
    (
        "and-pattern-argument-hrule",
        "(hrule r () :match (and (p ?x) (absent (q (and (p ?x))))) :assert (won ?x) :why \"w\")",
        "hrule 'r': its :match writes (q (and (p ?x))), and (and (p ?x)) is a connective where \
         a value goes",
        "",
    ),
    (
        "and-pattern-argument-obligation",
        "(rule r () :match (and (p ?x) (absent (q (and (p ?x))))) :assert (open p) :why \"w\")",
        "rule 'r': its :match writes (q (and (p ?x))), and (and (p ?x)) is a connective where \
         a value goes",
        "",
    ),
    (
        "and-pattern-argument-goal",
        "(query :goal (and (p ?x) (absent (q (and (p ?x))))))",
        "query: its :goal writes (q (and (p ?x))), and (and (p ?x)) is a connective where a \
         value goes",
        "",
    ),
];

/// A connective a macro leaves as a pattern's argument: through a call's
/// argument, at any depth, and the stdlib's `forall` called inside a pattern.
const REFUSED_THROUGH_A_MACRO: [(&str, &str, &str); 3] = [
    (
        "macro-argument",
        "(macro m (?a) (q ?a) :why \"m\")\n\
         (rule r () :match (and (p ?x) (absent (m (and (p ?x))))) :assert (won ?x) :why \"w\")",
        "rule 'r': its :match writes (q (and (p ?x))), and (and (p ?x)) is a connective where \
         a value goes",
    ),
    (
        "macro-argument-inside-a-pattern",
        "(macro m (?a) (q ?a) :why \"m\")\n\
         (rule r () :match (and (p ?x) (absent (h (m (and (p ?x)))))) :assert (won ?x) \
         :why \"w\")",
        "rule 'r': its :match writes (q (and (p ?x))), and (and (p ?x)) is a connective where \
         a value goes",
    ),
    (
        "forall-inside-a-pattern",
        "(import std.macro :symbols (forall))\n\
         (rule r () :match (and (p ?x) (absent (h (forall ?y (p ?y) (q ?y))))) :assert (won ?x) \
         :why \"w\")",
        "rule 'r': its :match writes (h (absent (and (p ?y) (absent (q ?y))))), and (absent \
         (and (p ?y) (absent (q ?y)))) is written where a value goes",
    ),
];

/// What keeps loading: `(shape, the forms after HEAD)`.
const ACCEPTED: [(&str, &str); 19] = [
    ("false-alone", "(false)"),
    ("false-alone-with-a-source", "(false :source \"given\")"),
    ("false-alone-as-a-value", "(q (false))"),
    ("false-alone-negated", "(not (false))"),
    (
        "false-alone-as-a-premise",
        "(rule r () :match (and (p ?x) (absent (false))) :assert (won ?x) :why \"w\")",
    ),
    ("a-stored-negation-as-a-value", "(q (not (p a)))"),
    (
        "a-stored-negation-as-a-pattern-argument",
        "(q (not (p a)))\n\
         (rule r () :match (and (p ?x) (q (not (p ?x)))) :assert (won ?x) :why \"w\")",
    ),
    (
        "a-pattern-as-a-pattern-argument",
        "(q (p a))\n(rule r () :match (q (p ?x)) :assert (won ?x) :why \"w\")",
    ),
    ("an-equality-as-a-fact", "(eq a a)"),
    ("a-negated-equality-as-a-fact", "(not (eq a b))"),
    (
        "a-negated-equality-as-a-premise",
        "(rule r () :match (and (p ?x) (not (eq ?x b))) :assert (won ?x) :why \"w\")",
    ),
    (
        "a-negated-equality-as-a-conclusion",
        "(rule r () :match (p ?x) :assert (not (eq ?x b)) :why \"w\")",
    ),
    (
        "a-guard-over-a-fact",
        "(q (p a))\n\
         (rule r () :match (and (q ?v) (eq ?v (p a))) :assert (won a) :why \"w\")",
    ),
    (
        "the-negation-of-a-bound-fact",
        "(h (q a))\n(rule r () :match (h ?s) :assert (not ?s) :why \"w\")",
    ),
    (
        "absent-in-a-claims-class",
        "(query :goal (and (p ?y) (absent (q ?y))) :expect (model (absent (q a)) (p a)))",
    ),
    (
        "the-three-words-as-names",
        "(h absent) (h false) (h eq) (hit eq absent)",
    ),
    (
        "eq-as-a-parameters-relation",
        "(mk eq)\n(rule mk (?R) :match (and (p ?x) (?R ?x ?x)) :assert (won ?x) :why \"w\")",
    ),
    (
        "a-parameter-heading-a-value",
        "(mk p)\n(rule mk (?R) :match (p ?x) :assert (q (?R ?x)) :why \"w\")",
    ),
    (
        "forall-called-with-premises",
        "(import std.macro :symbols (forall))\n\
         (rule r () :match (and (p ?x) (forall ?y (and (p ?y) (neq ?y b)) (p ?y))) \
         :assert (won ?x) :why \"w\")",
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

/// `file` refused on every command with one stderr, the same on each: its
/// first line placed in `shown` and holding `first`, the whole holding
/// `also`, and no container written.
fn refused_everywhere(
    file: &Path,
    shown: &str,
    first: &str,
    also: &str,
    dir: &Path,
) -> Vec<String> {
    let mut bad = Vec::new();
    let einb = dir.join("x.einb");
    let events = dir.join("events.jsonl");
    let mut seen: Option<String> = None;
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
        let placed = line.starts_with(&format!("{shown}:"))
            || line.starts_with(&format!("kb load error: {shown}:"))
            // A relation parameter's refusal names the rule, and has no place.
            || line.starts_with("kb load error: rule '");
        if !placed || !line.contains(first) || !r.err.contains(also) {
            bad.push(format!(
                "ein {argv:?}: {:?}\n  wants a first line at {shown} holding {first:?}, and \
                 {also:?} after it",
                r.err
            ));
        }
        match &seen {
            None => seen = Some(r.err.clone()),
            Some(s) if *s != r.err => bad.push(format!(
                "ein {argv:?}: {:?}, where the first command printed {s:?}",
                r.err
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
fn a_form_that_is_no_value_is_refused_on_every_command() {
    let scratch = Scratch::new("ein-value-rule");
    let mut bad = Vec::new();
    let through = REFUSED_THROUGH_A_MACRO.map(|(shape, body, first)| (shape, body, first, ""));
    for (shape, body, first, also) in REFUSED.iter().chain(through.iter()) {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, format!("{HEAD}{body}\n")).expect("written");
        let shown = file.display().to_string();
        for b in refused_everywhere(&file, &shown, first, also, scratch.path()) {
            bad.push(format!("{shape}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The register's nineteen witnesses (W80 to W83), each read from its file,
/// as `wrong_answers.rs` reads one command's run of each.
#[test]
fn the_registers_witnesses_are_refused_on_every_command() {
    let scratch = Scratch::new("ein-value-rule-witnesses");
    let witnesses: [(&str, &str, &str); 19] = [
        ("absent-fact-for-not", "unexpected `absent`", ABSENT_NOTE),
        ("absent-fact-name", "unexpected `absent`", ABSENT_NOTE),
        ("absent-fact-nested", "unexpected `absent`", ABSENT_NOTE),
        ("absent-concluded", "unexpected `absent`", ABSENT_NOTE),
        ("absent-hrule-concluded", "unexpected `absent`", ABSENT_NOTE),
        (
            "false-fact-argument",
            "unexpected `Bob`: a name",
            FALSE_NOTE,
        ),
        ("false-fact-nested", "unexpected `Bob`: a name", FALSE_NOTE),
        ("false-fact-negated", "unexpected `Bob`: a name", FALSE_NOTE),
        (
            "false-premise-argument",
            "unexpected `?g`: a variable",
            FALSE_NOTE,
        ),
        (
            "false-goal-argument",
            "unexpected `?g`: a variable",
            FALSE_NOTE,
        ),
        ("eq-value-fact", "unexpected `eq`", EQ_NOTE),
        ("eq-value-concluded", "unexpected `eq`", EQ_NOTE),
        ("eq-value-matched-by-shape", "unexpected `eq`", EQ_NOTE),
        ("eq-value-negation-concluded", "unexpected `eq`", EQ_NOTE),
        (
            "pattern-argument-and",
            "rule 'greet': its :match writes (says (and (paid ?g))), and (and (paid ?g)) is a \
             connective where a value goes",
            "",
        ),
        (
            "pattern-argument-and-under-absent",
            "rule 'greet': its :match writes (says (and (paid ?g))), and (and (paid ?g)) is a \
             connective where a value goes",
            "",
        ),
        (
            "pattern-argument-hrule",
            "hrule 'guess': its :match writes (says (or (paid ?g) (welcome ?g)))",
            "",
        ),
        (
            "pattern-argument-goal",
            "query: its :goal writes (says (neq ?g Bob)), and (neq ?g Bob) is a connective \
             where a value goes",
            "",
        ),
        (
            "pattern-argument-macro",
            "rule 'greet': its :match writes (says (and (paid ?g))), and (and (paid ?g)) is a \
             connective where a value goes",
            "",
        ),
    ];
    let mut bad = Vec::new();
    for (stem, first, also) in witnesses {
        let shown = format!("examples/ein-bugs/wrong-answers/{stem}.ein");
        for b in refused_everywhere(Path::new(&shown), &shown, first, also, scratch.path()) {
            bad.push(format!("{stem}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// What a value, a premise and a conclusion may still be.
#[test]
fn what_a_state_could_hold_loads() {
    let scratch = Scratch::new("ein-value-rule-accepted");
    let mut bad = Vec::new();
    for (shape, body) in ACCEPTED {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, format!("{HEAD}{body}\n")).expect("written");
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

/// A stored negation as a pattern's argument still matches, and a dead
/// pattern no longer makes an `(absent …)` hold: the rule of
/// `pattern-argument-and-under-absent`, written as it was meant, welcomes Bob
/// alone.
#[test]
fn the_meant_program_answers() {
    let scratch = Scratch::new("ein-value-rule-meant");
    let file = scratch.path().join("meant.ein");
    std::fs::write(
        &file,
        "(relation paid Guest :why \"{?1} has paid\")\n\
         (relation says Claim :why \"it is said that {?1}\")\n\
         (relation welcome Guest :why \"{?1} is welcome\")\n\
         (is-a Ann Guest) (is-a Bob Guest)\n(says (paid Ann))\n\
         (rule greet () :match (and (is-a ?g Guest) (absent (says (paid ?g)))) \
         :assert (welcome ?g) :why \"{?g} is welcome: nobody says they paid\")\n\
         (solve :hypothesis-relations ())\n\
         (query :goal (welcome ?g) :expect (model (welcome Bob)))\n",
    )
    .expect("written");
    let f = file.display().to_string();
    let r = ein(&["test", &f]);
    assert_eq!(r.code, 0, "{}", r.err);
}
