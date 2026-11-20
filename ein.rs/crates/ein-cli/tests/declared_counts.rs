//! **A relation is overloaded by its count of arguments, and a declaration
//! holds no form to its count** (Q-M1f.447).
//!
//! Ein has no types, and a declaration is an ordinary fact
//! (Q-M1f.372, Q-M1f.522): a form headed by a declared relation may have
//! another count of arguments than the declaration has labels, and loads in
//! silence, a fact of that count, which no pattern of another count matches.
//! A relation may be declared more than once, at one count or at several,
//! and each declaration is a fact of `relation`; none is refused as a
//! duplicate of another.
//!
//! What is held here: every shape the loader once refused for its count
//! loads on every command that answers a program; the seven witnesses of
//! `corpus/regressions.toml` that held the refusal load; a relation declared
//! twice, and at two counts, loads, and a query at each count answers that
//! count's facts.

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

/// Two declared relations, a fact of each, and a rule `r`.
const HEAD: &str = "(relation likes Person Drink :why \"{?3} likes {?4}\")\n\
     (relation fan Person :why \"{?3} is a fan\")\n\
     (relation flag :why \"the flag is up\")\n\
     (likes Ann Tea)\n(fan Ann)\n(flag)\n\
     (rule r ()\n  :match (likes ?a ?b)\n  :assert (seen ?a)\n  :why \"{?a} likes {?b}, so {?a} is seen\")\n";

/// `(shape, what is added to [`HEAD`])`: a form of a declared relation at
/// another count than its declaration's, in each place a program writes one.
const OTHER_COUNTS: [(&str, &str); 22] = [
    ("top-level-fact-short", "(likes Bob)\n"),
    ("top-level-fact-long", "(fan Bob Tea)\n"),
    ("top-level-fact-none", "(likes)\n"),
    ("a-bare-declaration-declares-none", "(flag up)\n"),
    (
        "a-keyword-pair-is-no-argument",
        "(likes Bob :source \"said\")\n",
    ),
    ("nested-fact", "(says Ann (likes Bob))\n"),
    ("nested-two-deep", "(says Ann (thinks Bob (fan Cy Dee)))\n"),
    ("under-a-stored-negation", "(not (likes Bob))\n"),
    ("a-facts-using", "(seen Bob :rule r :using ((likes Bob)))\n"),
    (
        "a-rules-match",
        "(rule s ()\n  :match (likes ?a)\n  :assert (seen ?a)\n  :why \"{?a} is seen\")\n",
    ),
    (
        "a-rules-match-under-absent",
        "(rule s ()\n  :match (and (fan ?a) (absent (likes ?a)))\n  :assert (seen ?a)\n  \
         :why \"{?a} is seen\")\n",
    ),
    (
        "a-rules-match-under-not",
        "(rule s ()\n  :match (and (fan ?a) (not (likes ?a ?a ?a)))\n  :assert (seen ?a)\n  \
         :why \"{?a} is seen\")\n",
    ),
    (
        "a-rules-match-inside-a-pattern",
        "(rule s ()\n  :match (says ?a (likes ?b))\n  :assert (seen ?a)\n  \
         :why \"{?a} is seen, {?b}\")\n",
    ),
    (
        "a-rules-assert",
        "(rule s ()\n  :match (likes ?a ?b)\n  :assert (fan ?a ?b)\n  \
         :why \"{?a} likes {?b}\")\n",
    ),
    (
        "a-rules-assert-inside-a-conclusion",
        "(rule s ()\n  :match (likes ?a ?b)\n  :assert (and (seen ?a) (says ?a (fan)))\n  \
         :why \"{?a} likes {?b}\")\n",
    ),
    (
        "a-hypothesis-rules-match",
        "(rule h ()\n  :match (fan ?a ?b)\n  :assert (open (seen ?a))\n  \
         :why \"guess that {?a} is seen, {?b}\")\n",
    ),
    (
        "a-hypothesis-rules-assert",
        "(rule h ()\n  :match (fan ?a)\n  :assert (open (likes ?a))\n  \
         :why \"guess that {?a} likes\")\n",
    ),
    (
        "an-obligations-match",
        "(rule owed (?R)\n  :match (and (fan ?a) (likes ?a) (absent (?R ?a ?b)))\n  :assert (open ?R)\n  \
         :why \"{?a} owes a {?R}\")\n(owed fan)\n",
    ),
    (
        "a-macros-body-where-a-rule-calls-it",
        "(macro liked (?a) (likes ?a) :why \"{?a} is liked\")\n\
         (rule s ()\n  :match (liked ?a)\n  :assert (seen ?a)\n  :why \"{?a} is seen\")\n",
    ),
    ("a-goal", "(query :goal (likes ?a))\n"),
    (
        "a-queries-claim",
        "(query :goal (says ?a ?b) :expect (model (says Ann Bob)))\n(says Ann Bob)\n",
    ),
    (
        "a-region-claim",
        "(solve :hypothesis-relations () :expect (region (model (likes Ann Tea) (fan Ann) \
         (flag) (seen Ann))))\n",
    ),
];

/// `file` answered on every command that reads a program and asks it no
/// claim, exit 0 and nothing on stderr. `ein test` asks a claim, which these
/// files state none of, and `ein oracle` refuses a cube past its limit, which
/// is no load.
fn loads_everywhere(file: &Path, dir: &Path) -> Vec<String> {
    let f = file.display().to_string();
    let einb = dir.join("x.einb").display().to_string();
    let runs: [Vec<&str>; 7] = [
        vec!["saturate", &f],
        vec!["solve", &f],
        vec!["solve", "--count", &f],
        vec!["solve", "--single", &f],
        vec!["show", "hyps", &f],
        vec!["render", "program", &f],
        vec!["kb", "save", &f, &einb],
    ];
    let mut bad = Vec::new();
    for argv in runs {
        let r = ein(&argv);
        if r.code != 0 || r.err.lines().any(|l| l.contains("error")) {
            bad.push(format!(
                "ein {argv:?}: exit {}: {}",
                r.code,
                r.err.lines().next().unwrap_or("")
            ));
        }
    }
    bad
}

#[test]
fn a_form_of_another_count_loads_on_every_command() {
    let scratch = Scratch::new("ein-declared-counts");
    let mut bad = Vec::new();
    for (shape, added) in OTHER_COUNTS {
        let file = scratch.path().join(format!("{shape}.ein"));
        std::fs::write(&file, format!("{HEAD}{added}")).expect("written");
        for b in loads_everywhere(&file, scratch.path()) {
            bad.push(format!("{shape}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The regressions' seven witnesses of a form at another count than its
/// relation's declaration, each read from its file: each loads.
#[test]
fn the_regressions_witnesses_load_on_every_command() {
    let scratch = Scratch::new("ein-declared-count-witnesses");
    let mut bad = Vec::new();
    for stem in [
        "arity-top-level-fact",
        "arity-nested-fact",
        "arity-request",
        "arity-num9-eight-digits",
        "arity-in-match",
        "arity-in-assert",
        "arity-in-goal",
    ] {
        let shown = format!("examples/ein-bugs/wrong-answers/{stem}.ein");
        for b in loads_everywhere(Path::new(&shown), scratch.path()) {
            bad.push(format!("{stem}: {b}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **A relation declared twice, and at two counts**: every declaration loads,
/// each is a fact of `relation`, and a query at each count answers that
/// count's facts alone: overloading by the count of arguments.
#[test]
fn a_relation_declared_twice_loads_and_each_count_answers_its_own() {
    let scratch = Scratch::new("ein-declared-twice");
    let file = scratch.path().join("twice.ein");
    std::fs::write(
        &file,
        "(relation likes Person Drink :why \"{?3} likes {?4}\")\n\
         (relation likes Person Drink :why \"{?3} is fond of {?4}\")\n\
         (relation likes Person :why \"{?3} likes something\")\n\
         (likes Ann Tea)\n(likes Bob)\n\
         (rule fond () :match (likes ?a ?b) :assert (fan ?a) :why \"{?a} likes {?b}\")\n\
         (solve :no-search)\n\
         (query :goal (likes ?a ?b) :expect (model (likes Ann Tea)))\n\
         (query :goal (likes ?a) :expect (model (likes Bob)))\n\
         (query :goal (fan ?a) :expect (model (fan Ann)))\n\
         (query :goal (relation likes ?t) :expect (model (relation likes Person)))\n",
    )
    .expect("written");
    let f = file.display().to_string();
    let r = ein(&["test", &f]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(r.out.contains("4 held, 0 FAILED"), "{}", r.out);
    let bad = loads_everywhere(&file, scratch.path());
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// What a declaration's count never read, each of which loads on every
/// command that answers a program.
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
        // A relation written through a parameter, at another count.
        (
            "through-a-parameter",
            "(rule mirror (?R)\n  :match (?R ?a)\n  :assert (?R ?a ?a ?a)\n  \
             :why \"{?R} holds of {?a}\")\n(mirror likes)\n",
        ),
        // The declaration's own fact, read at any count (`02-syntax.md`).
        (
            "the-declaration-as-a-pattern",
            "(rule s ()\n  :match (relation likes ?t)\n  :assert (seen ?t)\n  \
             :why \"{?t} is a type of likes\")\n",
        ),
        // A kernel marker at another count than the kernel reads.
        ("a-kernel-marker", "(__closed__ likes extra)\n"),
        // A name is not a form.
        (
            "a-relations-name-as-a-value",
            "(says likes fan)\n(tag flag)\n",
        ),
        // A `(solve …)` list is a list of names, not a form.
        (
            "a-solve-list",
            "(rule guess (?R ?T)\n  :match (is-a ?a ?T)\n  :assert (open (?R ?a))\n  \
             :why \"guess that {?R} holds of {?a}\")\n(is-a Bob Person)\n\
             (guess fan Person) (solve :no-hypothesis (likes))\n",
        ),
        (
            "a-solve-list-of-relations",
            "(solve :hypothesis-relations (likes fan flag))\n",
        ),
        // A macro no rule calls is not read (Q-M1f.67).
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
