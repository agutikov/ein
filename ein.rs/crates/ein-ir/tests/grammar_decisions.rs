//! The grammar's decisions — S1a.1.1's acceptance, without `lark`.
//!
//! Accept/reject was the weaker half of the comparison; the *message* was the
//! one that pinned the port, because the harness diffs stderr and a "better"
//! diagnostic is a T3 failure (design/04 §4, Q-M1a.3).
//!
//! [S1a.10.2](../../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.2_port_the_suite.md)
//! keeps both halves and loses the second parser:
//!
//! | was | now |
//! |---|---|
//! | the corpus-wide accept/reject diff | `corpus_shapes.md5`'s 111 `ir[parse]` lines, which carry the *message* of a refusal as well as the dump of an acceptance |
//! | the four `examples/broken/*.ein` messages | `.expected` files beside them, written **from ein.py** — see [`the_parse_negative_fixtures_reproduce_larks_message`] |
//! | the 78 hand-written ambiguity cases | a checked-in decision table |
//!
//! The `.expected` files are the interesting one. The ledger's row for this
//! test said the four fixtures were "owned today only as a digest" and
//! recommended writing the files out of ein.py before it goes; that is what
//! S1a.10.2 did, with `utils/ir_oracle.py`'s own `accept` answers, machine
//! paths replaced by `{FILE}` the way `examples/broken/load/` already does it.
//! They are lark's text, checked in, and nothing regenerates them from ein.rs.

use ein_ir::{Ast, parse};
use ein_oracle::{golden, golden_path, repo_root};

/// One parse, as text: `ok` or the diagnostic.
fn answer(text: &str, filename: Option<&str>) -> String {
    let mut ast = Ast::new();
    match parse(&mut ast, text, filename) {
        Ok(forms) => format!("ok ({} form(s))", forms.len()),
        Err(e) => e.to_string(),
    }
}

/// **The four parse-negative fixtures reproduce lark's message.**
///
/// `examples/broken/` is the parse-negative group, and each fixture now has an
/// `.expected` beside it holding the message **ein.py produced**, with the
/// absolute path replaced by `{FILE}` — the same placeholder convention
/// `examples/broken/load/` uses, and for the same reason: a message captured
/// verbatim passes on the machine that blessed it and fails everywhere else.
///
/// The digest in `corpus_shapes.md5` covers these four too, but as sixteen hex
/// digits. This is the half a reader can review.
#[test]
fn the_parse_negative_fixtures_reproduce_larks_message() {
    let dir = repo_root().join("examples/broken");
    let mut fixtures: Vec<_> = std::fs::read_dir(&dir)
        .expect("examples/broken")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "ein"))
        .collect();
    fixtures.sort();
    assert_eq!(fixtures.len(), 4, "the parse-negative group is four fixtures");

    let mut bad = Vec::new();
    for path in fixtures {
        let text = std::fs::read_to_string(&path).expect("readable");
        let name = path.file_name().expect("a file").to_string_lossy().to_string();
        let expected = std::fs::read_to_string(path.with_extension("expected"))
            .unwrap_or_else(|e| panic!("{name}: no .expected beside it: {e}"))
            .trim_end()
            .replace("{FILE}", path.to_str().expect("utf-8"));
        let got = answer(&text, path.to_str());
        if got.trim_end() != expected {
            bad.push(format!("{name}
  want: {expected:?}
  got:  {got:?}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("

"));
}

/// **The 78 cases that make the port non-obvious**, each one a decision in
/// `lex.rs` or `parse.rs`.
///
/// A reserved word is a legal SYMBOL *prefix* and the split reading wins
/// wherever it parses — but only across a word boundary; `relation` is the one
/// declarator that is also a SYMBOL; `value* kw_pair*` interleaves the way
/// `list_item*` allows; `=` and `not` are arity-pinned; and a block comment
/// that is never closed is a lexer decision rather than a parser one.
///
/// Checked in as a table rather than diffed, and the table is the *answer* —
/// the form count on an acceptance, the whole diagnostic on a refusal — so a
/// case that started parsing differently rather than merely failing shows up.
/// Blessed from a tree where the diff against `lark` was green, with one
/// honest caveat: what that diff compared was accept/reject **and the
/// message**, so those two are lark's answers; the *form count* is new here
/// and is a self-golden. The structure of an accepted parse is compared
/// against ein.py wherever the corpus reaches it, through
/// `corpus_shapes.md5`'s `ir[parse]` (which digests `dump_canonical`), and
/// these seventy-eight one-liners are not in the corpus.
#[test]
fn the_documented_ambiguities_keep_their_decisions() {
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
    assert_eq!(cases.len(), 78, "the decision table lost a case");
    let mut out = String::new();
    for case in cases {
        out.push_str(&format!("=== {case:?}\n"));
        for line in answer(case, None).lines() {
            out.push_str(&format!("  {line}\n"));
        }
    }
    if let Some(msg) = golden(&golden_path("ein-ir", "grammar_decisions.txt"), &out) {
        panic!("{msg}");
    }
}
