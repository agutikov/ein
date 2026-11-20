//! The grammar's decisions.
//!
//! Accept/reject is the weaker half; the *message* is the half a puzzle author
//! reads, and every `examples/broken/` fixture is baselined against it, so a
//! "better" diagnostic is a re-baseline rather than an improvement in passing
//! ([03-design/04-ir-frontend.md § 6](../../../../docs/ein/ein.rs/03-design/04-ir-frontend.md#6-parse-errors), Q-M1a.3).
//!
//! | claim | where |
//! |---|---|
//! | the corpus-wide accept/reject surface | `corpus_shapes.md5`'s 111 `ir[parse]` lines, which carry the *message* of a refusal as well as the dump of an acceptance |
//! | the thirty `examples/broken/*.ein` reports | `.expected` files beside them — see [`the_parse_negative_fixtures_reproduce_their_message`] |
//! | the 77 ambiguity cases | a checked-in decision table |
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

/// **The parse-negative fixtures reproduce their report.**
///
/// `examples/broken/` is the parse-negative group, and each fixture has an
/// `.expected` beside it holding its report, with the absolute path replaced
/// by `{FILE}` — the same placeholder convention
/// `examples/broken/load/` uses, and for the same reason: a message captured
/// verbatim passes on the machine that blessed it and fails everywhere else.
///
/// **Twenty-eight, and each `.expected` the whole report**: every error recovery
/// reaches, each with its position, its notes and its ranked repairs, three
/// and then a count. Most are shapes the grammar refuses where the loader or
/// the compiler would otherwise see them, `kwpair-below-the-filter` among
/// them (Q-M1e2.28).
///
/// The digest in `corpus_shapes.md5` covers these too, but as sixteen hex
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
        27,
        "the parse-negative group is twenty-seven fixtures"
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

/// **The 77 cases where the parser's reading is not obvious**, each held to
/// the generated parser's answer (`ir_semantics`'
/// `the_documented_ambiguities_resolve_as_recorded` says what each reads as,
/// and why).
///
/// A reserved word is a keyword only as a whole word, and a word character
/// after it makes the whole name a SYMBOL, never split into the keyword and
/// the rest (Q-M1e2.17), so a trace word a word character
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
/// `dump_canonical`), and these seventy-seven one-liners are not in the
/// corpus.
#[test]
fn the_documented_ambiguities_keep_their_decisions() {
    let cases = [
        // A reserved word a word character follows is no keyword: the whole
        // name is the SYMBOL it spells (Q-M1e2.17). The five trace rows are a
        // parse error, a trace holding only events, never the event `x`.
        "(rulex (?a) :match X :assert Y)",
        "(rulex A)",
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
    assert_eq!(cases.len(), 77, "the decision table lost a case");
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

/// **A keyword is a whole word, in every place the parser reads one**
/// (Q-M1e2.17, and the name rule, Q-M1e2.24).
///
/// Every word of `LITERALS` below, the language's literal words, and as
/// controls the kernel's eight other heads, each followed by a name
/// character and an argument shape, written in each of the twenty places the
/// parser reads a keyword: 51 480 parses. None splits the name into the
/// keyword and the rest of it, `(neqw ?x)` as `(neq w ?x)`:
///
/// - a word followed by a name character is never split: the whole name
///   parses as the `NAME` it spells, or, where no reading has it (a trace
///   event, which is `step` alone, a non-ASCII letter no name takes), the
///   parse is an error;
/// - a trace word followed by a name character, in a trace event, is a parse
///   error, a `(trace …)` holding only `step` events;
/// - `-`, `.` and `*` are name characters, so `(neq-1 ?x)` is the fact or
///   pattern of the relation `neq-1`, never `(neq -1 ?x)`.
#[test]
fn a_keyword_is_a_whole_word_in_every_place() {
    const CONTROLS: [&str; 8] = [
        "absent", "forall", "eq", "false", "open", "unknown", "true", "exists",
    ];
    // A word character (the first seven), then three other name characters.
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
            "hypothesis-match",
            "(rule h () :match (and (p ?x) (S)) :assert (open (q ?x)))",
        ),
        ("hypothesis", "(rule h () :match (p ?x) :assert (open (S)))"),
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
    let name_char = ein_core::text::is_name_char;
    // The name intact as a token of the dump.
    let whole = |dump: &str, name: &str| {
        dump.match_indices(name).any(|(i, _)| {
            !dump[..i].chars().next_back().is_some_and(name_char)
                && !dump[i + name.len()..].chars().next().is_some_and(name_char)
        })
    };
    // The language's literal words.
    const LITERALS: [&str; 18] = [
        "relation",
        "rule",
        "query",
        "solve",
        "config",
        "trace",
        "macro",
        "import",
        "not",
        "and",
        "or",
        "neq",
        "step",
        "branch-open",
        "branch-close",
        "branch-ref",
        "contradiction",
        "symmetry-class",
    ];
    let words: Vec<&str> = LITERALS.iter().copied().chain(CONTROLS).collect();
    assert_eq!(words.len(), 26);
    // The 520 cells, a place and a word each, on a worker per core:
    // most of these parses are refused, and a refusal runs
    // recovery, whose report this test does not read. The counts and the split
    // parses are summed in the order one loop over the places and the words
    // took them, so a failure reads as it did. A cell gives its parses, its
    // split parses and its `-1` parses read whole.
    type Cell = (usize, Vec<String>, usize);
    let one_cell = |place: &str, template: &str, word: &str| -> Cell {
        let (mut parses, mut split, mut named_dash) = (0usize, Vec::new(), 0usize);
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
                // Never split; and a trace word in a trace event has no
                // reading at all.
                if !whole(&dump, &name) {
                    split.push(format!("{place}: {src} -> {dump}"));
                }
                assert!(
                    !(place == "trace-event" && TRACE_WORDS.contains(&word)),
                    "{src} parsed as {dump}: a trace word read whole is no event"
                );
                if suffix == "-1" && VALUE_WORDS.contains(&word) {
                    named_dash += 1;
                }
            }
        }
        (parses, split, named_dash)
    };
    let cells: Vec<(&str, &str, &str)> = PLACES
        .iter()
        .flat_map(|&(place, template)| words.iter().map(move |&word| (place, template, word)))
        .collect();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let workers = std::thread::available_parallelism().map_or(4, |n| n.get());
    let mut done: Vec<(usize, Cell)> = std::thread::scope(|s| {
        let threads: Vec<_> = (0..workers)
            .map(|_| {
                s.spawn(|| {
                    let mut out = Vec::new();
                    loop {
                        let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let Some(&(place, template, word)) = cells.get(i) else {
                            return out;
                        };
                        out.push((i, one_cell(place, template, word)));
                    }
                })
            })
            .collect();
        threads
            .into_iter()
            .flat_map(|t| {
                t.join()
                    .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
            })
            .collect()
    });
    done.sort_by_key(|(i, _)| *i);
    assert_eq!(done.len(), cells.len());
    let (mut parses, mut split, mut named_dash) = (0usize, Vec::new(), 0usize);
    for (_, (p, sp, nd)) in done {
        parses += p;
        split.extend(sp);
        named_dash += nd;
    }
    assert_eq!(parses, 51_480);
    assert!(
        split.is_empty(),
        "{} parses split a name after a keyword:\n{}",
        split.len(),
        split.join("\n")
    );
    // The name rule: a value word followed by `-` and a digit is the name it
    // spells wherever it parses, never the keyword and `-1`. 348 parse, each
    // reading the whole name (this assertion); the rest of the 4 × 20 × 9
    // `-1` cells are a parse error for their place or argument shape.
    assert_eq!(named_dash, 348, "the `-1` parses that read the whole name");
}
