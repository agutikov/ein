//! The grammar's decisions — S1a.1.1's acceptance.
//!
//! Accept/reject is the weaker half; the *message* is the half a puzzle author
//! reads, and every `examples/broken/` fixture is baselined against it, so a
//! "better" diagnostic is a re-baseline rather than an improvement in passing
//! (design/04 §4, Q-M1a.3).
//!
//! | claim | where |
//! |---|---|
//! | the corpus-wide accept/reject surface | `corpus_shapes.md5`'s 111 `ir[parse]` lines, which carry the *message* of a refusal as well as the dump of an acceptance |
//! | the four `examples/broken/*.ein` messages | `.expected` files beside them — see [`the_parse_negative_fixtures_reproduce_their_message`] |
//! | the 78 hand-written ambiguity cases | a checked-in decision table |
//!
//! The `.expected` files carry machine paths replaced by `{FILE}`, the way
//! `examples/broken/load/` already does it.

use ein_corpus::{golden, golden_path, repo_root};
use ein_ir::{Ast, parse};

/// One parse, as text: `ok` or the diagnostic.
fn answer(text: &str, filename: Option<&str>) -> String {
    let mut ast = Ast::new();
    match parse(&mut ast, text, filename) {
        Ok(forms) => format!("ok ({} form(s))", forms.len()),
        Err(e) => e.to_string(),
    }
}

/// **The four parse-negative fixtures reproduce their message.**
///
/// `examples/broken/` is the parse-negative group, and each fixture has an
/// `.expected` beside it holding its message, with the absolute path replaced
/// by `{FILE}` — the same placeholder convention
/// `examples/broken/load/` uses, and for the same reason: a message captured
/// verbatim passes on the machine that blessed it and fails everywhere else.
///
/// The digest in `corpus_shapes.md5` covers these four too, but as sixteen hex
/// digits. This is the half a reader can review.
#[test]
fn the_parse_negative_fixtures_reproduce_their_message() {
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

    let mut bad = Vec::new();
    for path in fixtures {
        let text = std::fs::read_to_string(&path).expect("readable");
        let name = path
            .file_name()
            .expect("a file")
            .to_string_lossy()
            .to_string();
        let expected = std::fs::read_to_string(path.with_extension("expected"))
            .unwrap_or_else(|e| panic!("{name}: no .expected beside it: {e}"))
            .trim_end()
            .replace("{FILE}", path.to_str().expect("utf-8"));
        let got = answer(&text, path.to_str());
        if got.trim_end() != expected {
            bad.push(format!(
                "{name}
  want: {expected:?}
  got:  {got:?}"
            ));
        }
    }
    assert!(
        bad.is_empty(),
        "{}",
        bad.join(
            "

"
        )
    );
}

/// **The 78 cases that make the parser non-obvious**, each one a decision in
/// `lex.rs` or `parse.rs`.
///
/// A reserved word is a keyword only as a whole word, and a word character
/// after it makes the whole name a SYMBOL (M1e2 S1e2.2.22: until then the split
/// reading won wherever it parsed, W22), so a trace word a word character
/// follows, which a trace holds no other reading of, is a parse error;
/// `relation` is the one declarator that is also a SYMBOL; `value* kw_pair*` interleaves the way
/// `list_item*` allows; `=` and `not` are arity-pinned; and a block comment
/// that is never closed is a lexer decision rather than a parser one.
///
/// Checked in as a table rather than diffed, and the table is the *answer* —
/// the form count on an acceptance, the whole diagnostic on a refusal — so a
/// case that started parsing differently rather than merely failing shows up.
/// The structure of an accepted parse is pinned wherever the corpus reaches
/// it, through `corpus_shapes.md5`'s `ir[parse]` (which digests
/// `dump_canonical`), and these seventy-eight one-liners are not in the
/// corpus.
#[test]
fn the_documented_ambiguities_keep_their_decisions() {
    let cases = [
        // A reserved word a word character follows is no keyword: the whole
        // name is the SYMBOL it spells (M1e2 S1e2.2.22, the user's ruling on
        // Q-M1e2.17). Until then the split reading won wherever it parsed,
        // and the five trace rows parsed as the event `x`; now they are a
        // parse error, a trace holding only events.
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
        // At a word boundary the keyword is read, and `SYMBOL` refuses it.
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

/// **A keyword is a whole word, in every place the parser reads one** — M1e2
/// S1e2.2.22 (W22), on the user's ruling of 2026-11-05 on Q-M1e2.17.
///
/// Every word the parser matches as a literal (`lex::LITERALS`), and as
/// controls the kernel's eight other heads, each followed by a name
/// character and an argument shape, written in each of the twenty places the
/// parser reads a keyword: 53 460 parses. Until the stage 2 691 of them split
/// the name into the keyword and the rest of it, `(neqw ?x)` read as `(neq w
/// ?x)`. Now:
///
/// - a word followed by a word character (`_`, a letter, a digit) is never
///   split: the whole name parses as the `SYMBOL` it spells, or, where no
///   reading has it (a trace event, a non-ASCII letter `SYMBOL` does not
///   take), the parse is an error;
/// - a trace word followed by a word character, in a trace event, is a parse
///   error, a `(trace …)` holding only events (Q-M1e2.17 (ii)(a));
/// - `-`, `.` and `*` are no word characters, so the word before them is the
///   keyword (Q-M1e2.17 (i)(a)): `(neq-1 ?x)` is `(neq -1 ?x)`, and nothing
///   after `.` or `*` parses.
#[test]
fn a_keyword_is_a_whole_word_in_every_place() {
    const CONTROLS: [&str; 8] = [
        "absent", "forall", "eq", "false", "open", "unknown", "true", "exists",
    ];
    // A word character (the first seven), then `SYMBOL`'s three others.
    const SUFFIXES: [&str; 11] = ["w", "Z", "1", "_", "_a", "__a", "é", "-a", "-1", ".a", "*"];
    const ARGS: [&str; 9] = [
        "",
        " ?x",
        " ?x ?y",
        " a",
        " Node",
        " :k v",
        " () :match (p ?x) :assert (q ?x)",
        " (?a) (p ?a)",
        " :a b",
    ];
    // `S` is the shape; the places the parser reads a keyword in.
    const PLACES: [(&str, &str); 20] = [
        ("fact-arg", "(relation f Node) (f (S))"),
        ("fact-kw", "(f a :k (S))"),
        ("toplevel", "(S)"),
        ("match", "(rule r () :match (S) :assert (q a))"),
        (
            "match-and",
            "(rule r () :match (and (p ?x) (S)) :assert (q ?x))",
        ),
        (
            "absent",
            "(rule r () :match (and (p ?x) (absent (S))) :assert (q ?x))",
        ),
        ("assert", "(rule r () :match (p ?x) :assert (S))"),
        (
            "assert-and",
            "(rule r () :match (p ?x) :assert (and (q ?x) (S)))",
        ),
        ("assert-not", "(rule r () :match (p ?x) :assert (not (S)))"),
        ("nested", "(rule r () :match (p ?x) :assert (q (S)))"),
        (
            "hrule-match",
            "(hrule h () :match (and (p ?x) (S)) :assert (q ?x))",
        ),
        ("hrule-assert", "(hrule h () :match (p ?x) :assert (S))"),
        (
            "obligation",
            "(rule o (?R) :match (and (relation ?R ?A) (?A ?a) (absent (S))) :assert (open ?R))",
        ),
        ("query-goal", "(query :goal (S))"),
        ("query-expect", "(query :goal (p ?y) :expect (model (S)))"),
        (
            "query-or",
            "(query :goal (p ?y) :expect (or (model (S)) (model (p a))))",
        ),
        ("solve-expect", "(solve :expect (region (model (S))))"),
        ("config", "(config :k (S))"),
        ("macro-body", "(macro m (?a) (S))"),
        ("trace-event", "(trace (S))"),
    ];
    const VALUE_WORDS: [&str; 4] = ["not", "and", "or", "neq"];
    const TRACE_WORDS: [&str; 6] = [
        "step",
        "branch-open",
        "branch-close",
        "branch-ref",
        "contradiction",
        "symmetry-class",
    ];
    let name_char = |c: char| c.is_ascii_alphanumeric() || matches!(c, '_' | '*' | '.' | '-');
    // The name intact as a token of the dump.
    let whole = |dump: &str, name: &str| {
        dump.match_indices(name).any(|(i, _)| {
            !dump[..i].chars().next_back().is_some_and(name_char)
                && !dump[i + name.len()..].chars().next().is_some_and(name_char)
        })
    };
    let words: Vec<&str> = ein_ir::lex::LITERALS
        .iter()
        .copied()
        .chain(CONTROLS)
        .collect();
    assert_eq!(words.len(), 27);
    let (mut parses, mut split, mut keyword_dash) = (0usize, Vec::new(), 0usize);
    for (place, template) in PLACES {
        for word in &words {
            for suffix in SUFFIXES {
                for args in ARGS {
                    let name = format!("{word}{suffix}");
                    let src = template.replacen('S', &format!("{name}{args}"), 1);
                    parses += 1;
                    let mut ast = Ast::new();
                    let dump = match parse(&mut ast, &src, None) {
                        Ok(forms) => forms
                            .iter()
                            .map(|&f| ein_ir::dump_compact(&ast, f))
                            .collect::<Vec<_>>()
                            .join(" "),
                        Err(_) => {
                            continue;
                        }
                    };
                    let word_char = suffix.starts_with(|c: char| c.is_alphanumeric() || c == '_');
                    if word_char {
                        // Never split; and a trace word in a trace event has
                        // no reading at all.
                        if !whole(&dump, &name) {
                            split.push(format!("{place}: {src} -> {dump}"));
                        }
                        assert!(
                            !(place == "trace-event" && TRACE_WORDS.contains(word)),
                            "{src} parsed as {dump}: a trace word read whole is no event"
                        );
                    } else if suffix == "-1" && VALUE_WORDS.contains(word) && !whole(&dump, &name) {
                        // The keyword, and the value -1.
                        assert!(dump.contains(&format!("({word} -1")), "{src} -> {dump}");
                        keyword_dash += 1;
                    }
                }
            }
        }
    }
    assert_eq!(parses, 53_460);
    assert!(
        split.is_empty(),
        "{} parses split a name after a keyword:\n{}",
        split.len(),
        split.join("\n")
    );
    // Q-M1e2.17 (i)(a): a keyword followed by `-` and a digit is the
    // keyword, at `35186bc0` and since.
    assert_eq!(keyword_dash, 435);
}
