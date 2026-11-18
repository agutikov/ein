//! **The negation of a negation is a fact** (Q-M1f.391).
//!
//! A stored negation is a fact, so it has a stored negation of its own:
//! `(not (not g))` is the fact of `not` whose one argument is the fact
//! `(not g)`. A file writes it wherever it writes a negation, as a fact, as
//! an argument, in a pattern, in a conclusion, in a goal and in a claim, at
//! any depth (`ein.y`'s `NegFact` and `NegPattern`), and a rule's conclusion
//! `(not ?x)` builds one where `?x` is bound to a stored negation.
//!
//! The kernel reads it as it reads any fact: a fact held with its negation is
//! a contradiction, so `(not g)` beside `(not (not g))` is dead and `g`
//! beside `(not (not g))` is alive, and nothing concludes `g` from it. That
//! step is a rule a program states, one rule a count of arguments with the
//! relation a parameter.
//!
//! **The claim is held by an invariance**, not by a banked output: a program
//! in which a rule builds the fact and the same program with the fact written
//! in the file answer alike, on `ein solve`, `ein oracle` and `ein test`,
//! alone, beside the denial and beside the fact; and so do the two ways of
//! writing the liar's rule under a search, over a variable and by shape.
//!
//! What stays refused: `not` of a name, of an equality and of a variable
//! under another `not`; and a bare variable under a `not` that is an
//! argument.

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

/// The read-out's rows that say what R is: its counts, and that it is empty.
fn region(read_out: &str) -> Vec<String> {
    read_out
        .lines()
        .filter(|l| {
            let l = l.trim_start();
            l.starts_with("states (k_s)")
                || l.starts_with("commitments (k_c)")
                || l.starts_with("empty ")
                || l.starts_with("N ")
        })
        .map(|l| l.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect()
}

/// The facts `ein saturate --dump` lists as held, each without its clauses.
fn held(dump: &str) -> Vec<String> {
    dump.lines()
        .filter(|l| l.starts_with("  ("))
        .map(|l| {
            let l = l.trim();
            match l.find(" :rule ") {
                Some(i) => format!("{})", &l[..i]),
                None => l.to_string(),
            }
        })
        .collect()
}

const G: &str = "(= 5 (+ 2 2))";

/// Every place a file writes a negation, with a negation under it.
const WRITTEN: [(&str, &str); 12] = [
    ("fact", "(not (not (p a)))"),
    ("fact-thrice", "(not (not (not (p a))))"),
    (
        "fact-with-a-source",
        "(not (not (p a)) :source \"said twice\")",
    ),
    ("argument", "(told Ann (not (not (p a))))"),
    (
        "argument-deep",
        "(told Ann (heard Bob (not (not (not (p a))))))",
    ),
    ("falsity", "(not (not (false)))"),
    (
        "premise",
        "(not (not (p a)))\n(rule r () :match (not (not (p ?x))) :assert (q ?x) :why \"w\")",
    ),
    (
        "premise-argument",
        "(told Ann (not (not (p a))))\n\
         (rule r () :match (told ?w (not (not (p ?x)))) :assert (q ?x) :why \"w\")",
    ),
    (
        "conclusion",
        "(p a)\n(rule r () :match (p ?x) :assert (not (not (p ?x))) :why \"w\")",
    ),
    (
        "conclusion-argument",
        "(p a)\n(rule r () :match (p ?x) :assert (told Ann (not (not (p ?x)))) :why \"w\")",
    ),
    (
        "goal-and-claim",
        "(not (not (p a)))\n\
         (query :goal (not (not (p ?x))) :expect (model (not (not (p a)))))",
    ),
    (
        "through-a-macro",
        "(not (not (p a)))\n(macro nn (?X) (not (not ?X)) :why \"nn\")\n\
         (rule r () :match (nn (p ?x)) :assert (q ?x) :why \"w\")",
    ),
];

#[test]
fn a_file_writes_it_wherever_it_writes_a_negation() {
    let scratch = Scratch::new("ein-negneg-written");
    let mut bad = Vec::new();
    for (shape, body) in WRITTEN {
        let f = write(&scratch, &format!("{shape}.ein"), &format!("{body}\n"));
        let einb = scratch
            .path()
            .join(format!("{shape}.einb"))
            .display()
            .to_string();
        let mut runs: Vec<Vec<&str>> = vec![
            vec!["saturate", &f],
            vec!["saturate", "--dump", &f],
            vec!["solve", &f],
            vec!["oracle", &f],
            vec!["show", "hyps", &f],
            vec!["render", "program", &f],
            vec!["render", "kb", &f],
            vec!["kb", "save", &f, &einb],
        ];
        // `ein test` wants a claim to check: the shape that states one.
        if body.contains(":expect") {
            runs.push(vec!["test", &f]);
        }
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
        // The container holds what the text holds.
        let (text, container) = (
            ein(&["saturate", "--dump", &f]),
            ein(&["saturate", "--dump", &einb]),
        );
        if held(&text.out) != held(&container.out) {
            bad.push(format!(
                "{shape}: the container holds {:?}, the text {:?}",
                held(&container.out),
                held(&text.out)
            ));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// A premise and a conclusion over one match as a written one does.
#[test]
fn a_rule_matches_one_and_concludes_one() {
    let scratch = Scratch::new("ein-negneg-rules");
    let f = write(
        &scratch,
        "rules.ein",
        "(not (not (= a b)))\n\
         (not (not (likes Dan Soup)))\n\
         (told Ann (not (not (= c d))))\n\
         (= e f)\n\
         (rule eliminate (?R) :match (not (not (?R ?x ?y))) :assert (?R ?x ?y) :why \"dne\")\n\
         (eliminate =)\n\
         (rule eliminate-in () :match (told ?w (not (not (= ?x ?y)))) :assert (told ?w (= ?x ?y)) \
         :why \"dne in\")\n\
         (rule introduce () :match (= ?x ?y) :assert (not (not (= ?x ?y))) :why \"dni\")\n\
         (solve :no-search)\n\
         (query :goal (= ?x ?y) :expect (model (= a b) (= e f)))\n\
         (query :goal (likes ?x ?y) :expect (model))\n\
         (query :goal (told Ann (= ?x ?y)) :expect (model (told Ann (= c d))))\n\
         (query :goal (not (not (= ?x ?y))) \
         :expect (model (not (not (= a b))) (not (not (= e f)))))\n",
    );
    let r = ein(&["test", &f]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(r.out.contains("4 held, 0 FAILED"), "{}", r.out);
}

/// The program in which the liar's rule builds the negation of a negation,
/// and the same program with that fact written in the file and no rule.
fn pair(extra: &str, expect: &str) -> (String, String) {
    let head = format!(
        "(think Alice (not {G}))\n(liar Alice)\n{extra}\n(solve :no-search :expect {expect})\n"
    );
    let built = format!(
        "{head}(rule distrust () :match (and (liar ?p) (think ?p ?f)) :assert (not ?f) \
         :why \"{{?p}} lies\")\n"
    );
    let written = format!("{head}(not (not {G}))\n");
    (built, written)
}

#[test]
fn built_by_a_rule_or_written_in_the_file_the_answer_is_the_same() {
    let scratch = Scratch::new("ein-negneg-invariance");
    let mut bad = Vec::new();
    // Alone it is alive; beside the denial it is dead, a fact with its
    // negation; beside the fact it is alive, since nothing denies the fact.
    let cases: [(&str, String, &str, &str); 3] = [
        ("alone", String::new(), "(region (model))", "1"),
        ("denied", format!("(not {G})"), "(false)", "0"),
        ("held", G.to_string(), "(region (model))", "1"),
    ];
    for (name, extra, expect, states) in cases {
        let (built, written) = pair(&extra, expect);
        let b = write(&scratch, &format!("{name}-built.ein"), &built);
        let w = write(&scratch, &format!("{name}-written.ein"), &written);
        for cmd in ["solve", "oracle"] {
            let (rb, rw) = (ein(&[cmd, &b]), ein(&[cmd, &w]));
            if rb.code != 0 || rw.code != 0 {
                bad.push(format!(
                    "{name}: ein {cmd}: exit {} and {}",
                    rb.code, rw.code
                ));
            }
            let (qb, qw) = (region(&rb.out), region(&rw.out));
            if qb != qw {
                bad.push(format!("{name}: ein {cmd}: built {qb:?}, written {qw:?}"));
            }
            if !qb.contains(&format!("states (k_s) {states}")) {
                bad.push(format!("{name}: ein {cmd}: {qb:?}, want {states} states"));
            }
        }
        for f in [&b, &w] {
            let r = ein(&["test", f]);
            if r.code != 0 || !r.out.contains("1 held, 0 FAILED") {
                bad.push(format!("{name}: ein test {f}: exit {}: {}", r.code, r.out));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// Under a search: the liar's rule over a variable and the same rule spelt
/// by shape, a commitment choosing who lies. Each of the four programs is
/// answered alike by the two rules, and by `ein solve` and `ein oracle`.
#[test]
fn a_search_reaches_the_same_region_by_a_variable_and_by_shape() {
    let scratch = Scratch::new("ein-negneg-search");
    let over_a_variable = "(rule distrust () :match (and (liar ?p) (think ?p ?f)) \
                           :assert (not ?f) :why \"{?p} lies\")";
    let by_shape = "(rule distrust-neg () :match (and (liar ?p) (think ?p (not (ok ?x)))) \
                    :assert (not (not (ok ?x))) :why \"{?p} lies about not {?x}\")\n\
                    (rule distrust-pos () :match (and (liar ?p) (think ?p (ok ?x))) \
                    :assert (not (ok ?x)) :why \"{?p} lies about {?x}\")";
    let mut bad = Vec::new();
    for (name, extra, states) in [
        ("nothing", "", "4"),
        ("the-denial", "(not (ok a))", "2"),
        ("the-fact", "(ok a)", "4"),
        ("the-other-fact", "(ok b)", "2"),
    ] {
        let mut seen: Option<Vec<String>> = None;
        for (rule, text) in [("variable", over_a_variable), ("shape", by_shape)] {
            let f = write(
                &scratch,
                &format!("{name}-{rule}.ein"),
                &format!(
                    "(person Ann)\n(person Bob)\n(think Ann (not (ok a)))\n(think Bob (ok b))\n\
                     {extra}\n\
                     (hrule guess () :match (person ?p) :assert (liar ?p) \
                     :why \"suppose {{?p}} lies\")\n{text}\n"
                ),
            );
            for cmd in ["solve", "oracle"] {
                let r = ein(&[cmd, &f]);
                let q = region(&r.out);
                if r.code != 0 || !q.contains(&format!("states (k_s) {states}")) {
                    bad.push(format!("{name}, {rule}: ein {cmd}: exit {}: {q:?}", r.code));
                }
                match &seen {
                    None => seen = Some(q),
                    Some(s) if *s != q => {
                        bad.push(format!("{name}, {rule}: ein {cmd}: {q:?}, where {s:?}"))
                    }
                    Some(_) => {}
                }
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// `(shape, the program, what the first line of stderr holds)`.
const REFUSED: [(&str, &str, &str); 9] = [
    ("of-a-name", "(not (not Bob))", "unexpected `Bob`"),
    (
        "of-a-name-in-an-argument",
        "(told Ann (not (not Bob)))",
        "unexpected `Bob`",
    ),
    ("of-an-equality", "(not (not (eq a b)))", "unexpected `eq`"),
    (
        "of-an-equality-in-a-premise",
        "(rule r () :match (and (p ?x ?y) (not (not (eq ?x ?y)))) :assert (q ?x) :why \"w\")",
        "unexpected `eq`",
    ),
    (
        "of-a-variable-in-a-premise",
        "(rule r () :match (and (p ?x) (not (not ?x))) :assert (q ?x) :why \"w\")",
        "unexpected `?x`",
    ),
    (
        "of-a-variable-in-a-conclusion",
        "(rule r () :match (p ?x) :assert (not (not ?x)) :why \"w\")",
        "unexpected `?x`",
    ),
    (
        "a-variable-under-a-not-that-is-an-argument",
        "(rule r () :match (told ?w (not ?f)) :assert (q ?w) :why \"w\")",
        "unexpected `?f`",
    ),
    (
        "of-a-variable-through-a-macro",
        "(macro nn (?X) (not (not ?X)) :why \"nn\")\n\
         (rule r () :match (and (p ?x) (nn ?x)) :assert (q ?x) :why \"w\")",
        "rule 'r': its :match writes (not ?x), and not takes a pattern (relation argument …), \
         not ?x",
    ),
    (
        "of-another-count",
        "(relation p A B :why \"{?1} p {?2}\")\n(not (not (p a)))",
        "the fact (not (not (p a))) holds (p a), which gives p 1 argument",
    ),
];

#[test]
fn what_not_does_not_deny_is_refused_under_a_negation_too() {
    let scratch = Scratch::new("ein-negneg-refused");
    let mut bad = Vec::new();
    for (shape, body, first) in REFUSED {
        let f = write(&scratch, &format!("{shape}.ein"), &format!("{body}\n"));
        for (cmd, code) in [("saturate", 1), ("solve", 1), ("oracle", 1), ("test", 2)] {
            let r = ein(&[cmd, &f]);
            let line = r.err.lines().next().unwrap_or("");
            if r.code != code || !line.contains(first) {
                bad.push(format!(
                    "{shape}: ein {cmd}: exit {}, want {code}: {line:?}, want {first:?}",
                    r.code
                ));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// A state that holds one is saved as a file that loads to it: the fact a
/// rule built is derived again, by the rule that built it.
#[test]
fn a_state_that_holds_one_is_a_file_that_loads() {
    let scratch = Scratch::new("ein-negneg-state-file");
    let (built, _) = pair(G, "(region (model))");
    let f = write(&scratch, "held.ein", &built);
    let dir = scratch.path().join("answer").display().to_string();
    let r = ein(&["solve", &f, "--dump-answer", &dir]);
    assert_eq!(r.code, 0, "{}", r.err);
    let state = scratch.path().join("answer/models/01.ein");
    assert!(state.is_file(), "the one state, a file");
    let dump = ein(&["saturate", "--dump", &state.display().to_string()]);
    assert_eq!(dump.code, 0, "{}", dump.err);
    assert!(
        dump.out.contains(&format!("(not (not {G}) :rule ")),
        "the state holds the fact: {}",
        dump.out
    );
    let again = ein(&["solve", &state.display().to_string()]);
    assert_eq!(again.code, 0, "{}", again.err);
    assert!(
        region(&again.out).contains(&"states (k_s) 1".to_string()),
        "{}",
        again.out
    );
}
