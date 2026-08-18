//! S1a.1.1 acceptance — the parser agrees with `lark.parse` on what is
//! ein-lang, and says the same thing when it is not.
//!
//! Accept/reject is the weaker half; the *message* is the one that pins the
//! port, because the harness diffs stderr and a "better" diagnostic is a T3
//! failure (design/04 §4, Q-M1a.3).

#[path = "oracle.rs"]
mod oracle;

use ein_ir::{Ast, parse};
use oracle::{Answer, Oracle, corpus_files, repo_root, skip};

/// What ein.rs answers, in the oracle's vocabulary, so the two are comparable
/// without either side knowing about the other.
fn rust_accept(text: &str, filename: Option<&str>) -> Answer {
    let mut ast = Ast::new();
    match parse(&mut ast, text, filename) {
        Ok(_) => Answer::Ok(String::new()),
        Err(e) => Answer::Err {
            kind: "IRParseError".into(),
            msg: e.to_string(),
        },
    }
}

fn compare(got: &Answer, want: &Answer, what: &str) -> Option<String> {
    match (got, want) {
        (Answer::Ok(_), Answer::Ok(_)) => None,
        (Answer::Err { msg: a, .. }, Answer::Err { msg: b, .. }) if a == b => None,
        (Answer::Err { msg, .. }, Answer::Ok(_)) => Some(format!(
            "{what}\n  ein.py: accepted\n  ein.rs: rejected — {msg}"
        )),
        (Answer::Ok(_), Answer::Err { msg, .. }) => Some(format!(
            "{what}\n  ein.py: rejected — {msg}\n  ein.rs: accepted"
        )),
        (Answer::Err { msg: a, .. }, Answer::Err { msg: b, .. }) => {
            Some(format!("{what}\n  ein.py: {b:?}\n  ein.rs: {a:?}"))
        }
    }
}

#[test]
fn the_whole_corpus_parses_identically() {
    let Some(mut py) = Oracle::start() else {
        return skip("the_whole_corpus_parses_identically");
    };
    let files = corpus_files();
    assert!(files.len() >= 90, "only {} corpus files found", files.len());
    let mut bad = Vec::new();
    for path in &files {
        let text = std::fs::read_to_string(path).expect("readable");
        let name = path.to_str().expect("utf-8");
        let got = rust_accept(&text, Some(name));
        let want = py.file("accept", path);
        if let Some(d) = compare(&got, &want, name) {
            bad.push(d);
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {} files differ:\n{}",
        bad.len(),
        files.len(),
        bad.join("\n")
    );
}

#[test]
fn the_broken_fixtures_reproduce_larks_message_byte_for_byte() {
    let Some(mut py) = Oracle::start() else {
        return skip("the_broken_fixtures_reproduce_larks_message_byte_for_byte");
    };
    let dir = repo_root().join("examples/broken");
    let mut fixtures: Vec<_> = std::fs::read_dir(&dir)
        .expect("examples/broken")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "ein"))
        .collect();
    fixtures.sort();
    assert_eq!(
        fixtures.len(),
        4,
        "the parse-negative group is four fixtures"
    );

    for path in fixtures {
        let text = std::fs::read_to_string(&path).expect("readable");
        let name = path.to_str().expect("utf-8");
        let got = rust_accept(&text, Some(name));
        let want = py.file("accept", &path);
        assert!(
            matches!(want, Answer::Err { .. }),
            "{name} is in broken/ but ein.py accepts it"
        );
        if let Some(d) = compare(&got, &want, name) {
            panic!("{d}");
        }
    }
}

/// The cases that make the port non-obvious, asserted against the oracle
/// rather than against a remembered string. Each one is a *decision* in
/// `parse.rs`; if any drifts, this names which.
#[test]
fn the_documented_ambiguities_resolve_the_way_lark_resolves_them() {
    let Some(mut py) = Oracle::start() else {
        return skip("the_documented_ambiguities_resolve_the_way_lark_resolves_them");
    };
    let cases = [
        // A reserved word is a legal *prefix* of a SYMBOL, and the split
        // reading wins wherever it parses.
        "(rulex (?a) :match X :assert Y)",
        "(rulex A)",
        "(hrulex (?a) :match X :assert Y)",
        "(macrox (?a) B)",
        "(importx :as m)",
        "(importfoo.bar :as m)",
        "(notx)",
        "(notx A)",
        "(a (notx))",
        "(a (andx A B))",
        "(a (orx A B))",
        "(a (neqx A B))",
        "(relationx R A B)",
        "(relationx R (T1 T2))",
        "(queryx :goal Y)",
        "(configx)",
        "(tracex)",
        "(trace (stepx :a b))",
        "(trace (branch-openx :a b))",
        "(trace (branch-refx))",
        "(trace (contradictionx :a b))",
        "(trace (symmetry-classx :a b))",
        "(a (stepx :a b))",
        // …but only at a word boundary.
        "(rule-x A)",
        "(rule.x A)",
        "(rule_x (?a) :match X :assert Y)",
        "(not_a X)",
        "(neq_test X)",
        "(std.rule X)",
        "(relation-x R)",
        // `relation` is the one declarator that is also a SYMBOL.
        "(relation R A B)",
        "(relation R (T1 T2))",
        "(relation)",
        "(relation R)",
        // value* then kw_pair*, and the interleaving `list_item*` allows.
        "(a 1 :k 2)",
        "(a :k 1 :k 2)",
        "(a :k 1 2)",
        "(a 1 :k 2 3)",
        "(x (a 1 :k 2 3))",
        "(x (a :k 2 3))",
        // Arity-pinned shapes.
        "(= a b)",
        "(= a b c)",
        "(= a b :k 1)",
        "(x (= a b c))",
        "(not A B)",
        "(not A :k 1)",
        "(query)",
        "(config)",
        "(trace)",
        "(rule r ())",
        "(rule r () :match X)",
        "(macro m () B)",
        "(macro m (?a) B)",
        // Terminals.
        "(a __closed__)",
        "(a _)",
        "(a _x)",
        "(a 007)",
        "(a -0)",
        "(a 1..5)",
        "(a 1..*)",
        "(a 1..)",
        "(a \"h\\di\")",
        "(a \"a\nb\")",
        "(a \"a\\\nb\")",
        "(a \"unterminated)",
        "(x (?p ?q))",
        "(x (_ ?a))",
        "()",
        "(x ())",
        "(x (()))",
        // Trivia, including the unterminated block comment.
        "(x ; comment\n y)",
        "(x #| c |# y)",
        "(x #| never closed",
        "",
        "   ",
        ";; only a comment\n",
        "(a))",
        "(a",
    ];
    let mut bad = Vec::new();
    for case in cases {
        let got = rust_accept(case, None);
        let want = py.text("accept", case, None);
        if let Some(d) = compare(&got, &want, &format!("{case:?}")) {
            bad.push(d);
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {} cases differ:\n{}",
        bad.len(),
        cases.len(),
        bad.join("\n")
    );
}
