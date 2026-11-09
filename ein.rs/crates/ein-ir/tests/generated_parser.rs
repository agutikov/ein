//! The parser — M1f S1f.5.5's, generated from `src/ein.l` and `src/ein.y`
//! (T1f.5.5.3), and since the swap (T1f.5.5.4) the only one.
//!
//! | claim | test |
//! |---|---|
//! | a refused file's report is the same twice in one process (F11) | [`a_report_is_the_same_twice`] |
//! | a report cut at the step budget says so (F11) | [`a_report_cut_at_the_budget_says_so`] |
//! | a macro carries its `:why` (the user, 2026-11-08) | [`a_macro_carries_its_why`] |
//! | no name the loader makes up is a name a program can write (T1f.5.5.8) | [`no_name_the_loader_makes_up_is_a_name`] |
//! | a program at the nesting limit parses and goes through the dumper, macro expansion and the loader on a test thread; one past it is refused with a line (F10) | [`the_nesting_limit`] |
//! | `ein.l`'s one skip rule and its start condition give the parser the tokens T4's `ein.l` did (T1f.5.5.12) | [`the_lexer_gives_the_tokens_t4s_did`] |
//! | a string's escapes are S1f.5.2's ruling's, and a `\u{…}` reads exactly the Unicode scalar values (T1f.5.2.5, W38) | [`a_strings_escapes_are_the_rulings`] |
//!
//! Until the swap this file also held the generated parser to the
//! hand-written one, tree for tree on every corpus file both accepted, and
//! banked every file where they differed (`the_builder_holds_on_the_corpus`,
//! `golden/generated_parser_corpus.txt`); T1f.5.5.9 attributed each of those
//! to a ruling before the hand-written parser went, and both went with it.
//! The fuzzer's half of F11 is in `fuzz_properties.rs`.

use std::path::{Path, PathBuf};

use ein_corpus::{golden, golden_path, repo_root};
use ein_ir::Ast;
use ein_ir::grammar::{self, NESTING_LIMIT, RECOVERY_STEP_BUDGET};

/// Every `.ein` under the four corpus roots, relative to the repo, sorted:
/// `examples/`, `stdlib/`, `tests/` and `corpus/`, the census's set.
fn corpus() -> Vec<(String, PathBuf)> {
    let root = repo_root();
    let mut out = Vec::new();
    for dir in ["examples", "stdlib", "tests", "corpus"] {
        for p in ein_corpus::ein_files_under(&root.join(dir)) {
            let rel = p
                .strip_prefix(&root)
                .expect("under the root")
                .to_string_lossy()
                .into_owned();
            out.push((rel, p));
        }
    }
    out.sort();
    out
}

/// Every input whose report the F11 tests compare: each corpus file the
/// generated parser refuses, then `n` seeded mutants of the corpus
/// (`ein_corpus::mutants`, the census's generator).
fn refused_inputs(n: usize) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (rel, path) in corpus() {
        let text = std::fs::read_to_string(&path).expect("readable");
        if grammar::parse(&mut Ast::new(), &text, Some(&rel)).is_err() {
            out.push((rel, text));
        }
    }
    let sources = ein_corpus::mutants::sources();
    for (i, (which, text)) in ein_corpus::mutants::mutants(&sources, n, 0)
        .into_iter()
        .enumerate()
    {
        let name = format!("mutant {i} of {}", relative(&sources[which].0));
        out.push((name, text));
    }
    out
}

/// **A refused file's report is the same twice, in one process** (F11).
///
/// `lrpar` 0.15 ranked repairs of equal length in a `HashSet`'s order, and
/// every `HashSet` of a process draws its own hash keys, so two parses of one
/// file in one process could list its repairs in two orders, and go on with
/// different ones; and it stopped recovery at 500 ms of wall clock, so a
/// loaded machine reported fewer errors. The vendored patch ranks equal
/// repairs in the order the search found them and counts search steps. Each
/// input here is parsed twice and the two reports compared byte for byte:
/// every corpus file the generated parser refuses, and 1 000 seeded mutants.
/// With the ranking unpatched it fails (the T3 record says on how many).
#[test]
fn a_report_is_the_same_twice() {
    let inputs = refused_inputs(1000);
    let (mut refused, mut ranked, mut cut) = (0, 0, 0);
    let mut differ = Vec::new();
    for (name, text) in &inputs {
        let once = grammar::parse(&mut Ast::new(), text, Some(name));
        let twice = grammar::parse(&mut Ast::new(), text, Some(name));
        match (&once, &twice) {
            (Ok(_), Ok(_)) => {}
            (Err(a), Err(b)) => {
                refused += 1;
                ranked += usize::from(a.errors.iter().any(|e| e.repairs.len() > 1));
                cut += usize::from(a.cut);
                assert!(a.recovery_steps <= RECOVERY_STEP_BUDGET, "{name}");
                if a.to_string() != b.to_string() {
                    differ.push(format!("{name}:\n--- once\n{a}--- twice\n{b}"));
                }
            }
            _ => differ.push(format!("{name}: refused once, accepted once")),
        }
    }
    assert!(
        differ.is_empty(),
        "{} of {} refused inputs reported differently on their second parse:\n{}",
        differ.len(),
        refused,
        differ.join("\n")
    );
    // The comparison is only worth something where a tie can be ordered two
    // ways and where the budget cuts.
    assert!(refused >= 400, "only {refused} inputs were refused");
    assert!(
        ranked >= 100,
        "only {ranked} refusals ranked two repairs or more"
    );
    assert!(cut >= 1, "no input was cut at the budget");
}

/// **A report cut at the step budget says so** (F11).
///
/// `golden/recovery_cut.ein` is two mistakes recovery repairs and then a
/// comment whose second line lost its semicolons, so that its prose reads as
/// tokens: the search for a repair there runs past [`RECOVERY_STEP_BUDGET`]
/// (it needs 73 234 steps, re-taken with the vendored constant raised; the
/// T3 record). Unpatched, `lrpar` gave such an error no repairs, which reads
/// as *no repair exists*; the report says where recovery stopped instead,
/// after its last error, and lists nothing past it.
#[test]
fn a_report_cut_at_the_budget_says_so() {
    let path = golden_path("ein-ir", "recovery_cut.ein");
    let text = std::fs::read_to_string(&path).expect("the fixture");
    let r = grammar::parse(&mut Ast::new(), &text, Some("recovery_cut.ein"))
        .expect_err("the fixture is refused");
    assert!(r.cut, "recovery was not cut:\n{r}");
    assert_eq!(r.recovery_steps, RECOVERY_STEP_BUDGET);
    let (last, before) = r.errors.split_last().expect("an error");
    assert!(last.repairs.is_empty(), "the cut error has repairs:\n{r}");
    assert!(
        before.len() >= 2 && before.iter().all(|e| !e.repairs.is_empty()),
        "the errors before the cut are not repaired:\n{r}"
    );
    if let Some(msg) = golden(
        &golden_path("ein-ir", "recovery_cut.txt"),
        &format!("{r}\n"),
    ) {
        panic!("{msg}");
    }
}

/// **The end of the input is where the text ends** (M1f S1f.5.5,
/// T1f.5.5.13): an input that stops while a form is open is refused at its
/// last character's end, whatever whitespace or comment follows the last
/// token. `lrpar` makes its end-of-input lexeme at the end of the last token,
/// which the report used until then, so `(p a` and a newline said `1:5`.
#[test]
fn the_end_of_input_is_where_the_text_ends() {
    for (text, at) in [
        ("(p a", (1, 5)),
        ("(p a\n", (2, 1)),
        ("(p a ; a comment", (1, 17)),
        ("(p a\n\n;; trailing\n\n", (5, 1)),
        ("(p a)\n(q\n  b\n", (4, 1)),
        ("(relation r T", (1, 14)),
    ] {
        let r = grammar::parse(&mut Ast::new(), text, None).expect_err("an open form");
        let last = r.errors.last().expect("an error");
        assert_eq!(last.what, "unexpected end of input", "{text:?}:\n{r}");
        assert_eq!((last.line, last.col), at, "{text:?}:\n{r}");
    }
}

/// **A macro carries its `:why`**, as a relation and a rule do: the user's
/// ruling of 2026-11-08, that every form an NL template can go on takes one.
/// The parser reads `:why "…"` once after a macro's body and the macro reader
/// keeps it (`Macro::why`); until the swap the hand-written parser, the
/// shipped one, refused it. Whether a program must write one is the loader's
/// flag, `--require-why` (S1f.5.14).
#[test]
fn a_macro_carries_its_why() {
    let with = "(macro owns (?p ?x) (has ?p ?x) :why \"{?p} owns {?x}\")\n(has a b)";
    let mut ast = Ast::new();
    let forms = grammar::parse(&mut ast, with, Some("why.ein")).expect("a macro with a :why");
    let macros = ein_ir::collect_macros(&ast, &forms).expect("one macro");
    assert_eq!(macros["owns"].why.as_deref(), Some("{?p} owns {?x}"));
    assert_eq!(macros["owns"].params, ["p", "x"]);
    let mut terms = ein_core::Terms::new();
    ein_ir::load(&mut ast, &mut terms, &forms, None).expect("it loads");

    let without = "(macro owns (?p ?x) (has ?p ?x))";
    let mut ast = Ast::new();
    let forms = grammar::parse(&mut ast, without, None).expect("a macro without one");
    assert_eq!(
        ein_ir::collect_macros(&ast, &forms).expect("one")["owns"].why,
        None
    );

    for refused in [
        "(macro owns (?p ?x) (has ?p ?x) :why \"a\" :why \"b\")",
        "(macro owns (?p ?x) (has ?p ?x) :why owns)",
        "(macro owns (?p ?x) :why \"a\" (has ?p ?x))",
    ] {
        assert!(
            grammar::parse(&mut Ast::new(), refused, None).is_err(),
            "{refused}"
        );
    }
}

/// A relation's `:why` with `body` between its quotes: the text the parser
/// stores, or the report it refuses the program with.
fn why_text(body: &str) -> Result<String, grammar::ParseError> {
    let program = format!("(relation q T :why \"{body}\")");
    let mut ast = Ast::new();
    let forms = grammar::parse(&mut ast, &program, None)?;
    let text = ast
        .form_args(forms[0])
        .iter()
        .find_map(|&a| match ast.node(a) {
            ein_ir::Node::KwPair { value, .. } => match ast.node(value) {
                ein_ir::Node::Str(s) => Some(ast.sym(s).to_string()),
                _ => None,
            },
            _ => None,
        })
        .expect("a :why");
    Ok(text)
}

/// **A string's escapes are the ruling's, and a `\u{…}` reads exactly the
/// Unicode scalar values** (S1f.5.2's T1f.5.2.5, W38).
///
/// S1f.5.2's ruling (the user, 2026-11-06: *"strings literals are unicode with
/// reasonable escape symbols"*) gives a string `\n \t \r \" \\` and `\u{…}`, one
/// to six hex digits naming that code point, and refuses any other escape at
/// the parse. A code point that names no character, one past 10FFFF or a
/// surrogate (D800 to DFFF), is no escape either: until T1f.5.2.5 `ein.l`'s
/// `TEXT` took any six digits and the parser read such an escape as U+FFFD,
/// in silence. Held over every boundary of the scalar values in every width
/// from its shortest to seven digits, in both cases, and over every 257th
/// value up to 11FFFF in its shortest and its six-digit spelling: each reads as
/// its character where it names one in six digits or fewer, and is refused
/// otherwise, with a note naming the escape and its column.
#[test]
fn a_strings_escapes_are_the_rulings() {
    for (escape, read) in [
        ("\\n", "\n"),
        ("\\t", "\t"),
        ("\\r", "\r"),
        ("\\\"", "\""),
        ("\\\\", "\\"),
        ("Жук 日本 🦓 ⟹", "Жук 日本 🦓 ⟹"),
        ("a\nb", "a\nb"),
        ("\\u{1F993}\\u{416}", "🦓Ж"),
    ] {
        assert_eq!(why_text(escape).expect(escape), read, "{escape}");
    }
    for (body, escape) in [
        ("\\q", "\\q"),
        ("\\0", "\\0"),
        ("\\x41", "\\x"),
        ("\\u0416", "\\u"),
        ("\\u{}", "\\u{}"),
        ("\\u{41", "\\u{41"),
        ("\\u{0000041}", "\\u{0000041}"),
        ("\\u{110000}", "\\u{110000}"),
        ("\\u{D800}", "\\u{D800}"),
    ] {
        let report = why_text(body).expect_err(body).to_string();
        let note = format!("note: {escape}, at 1:21, is no escape a string reads: ");
        assert!(report.contains(&note), "{body}:\n{report}");
    }

    let mut values: Vec<u32> = vec![
        0, 1, 0x7F, 0x80, 0xFF, 0x7FF, 0x800, 0xFFF, 0x1000, 0xD7FF, 0xD800, 0xDBFF, 0xDC00,
        0xDFFF, 0xE000, 0xFFFD, 0xFFFF, 0x1_0000, 0xF_FFFF, 0x10_0000, 0x10_FFFF, 0x11_0000,
        0x1F_FFFF, 0xFF_FFFF,
    ];
    let boundaries = values.len();
    values.extend((0..=0x11_FFFFu32).step_by(257));
    let (mut read, mut refused) = (0, 0);
    for (i, &v) in values.iter().enumerate() {
        let shortest = format!("{v:X}").len();
        let widths: Vec<usize> = if i < boundaries {
            (shortest..=7).collect()
        } else {
            vec![shortest, 6.max(shortest)]
        };
        for width in widths {
            for hex in [format!("{v:0width$X}"), format!("{v:0width$x}")] {
                let body = format!("\\u{{{hex}}}");
                match (char::from_u32(v).filter(|_| width <= 6), why_text(&body)) {
                    (Some(c), Ok(text)) => {
                        assert_eq!(text, c.to_string(), "{body}");
                        read += 1;
                    }
                    (None, Err(e)) => {
                        let note = format!("note: {body}, at 1:21, is no escape a string reads");
                        assert!(e.to_string().contains(&note), "{body}:\n{e}");
                        refused += 1;
                    }
                    (Some(_), Err(e)) => panic!("{body} names a character, refused:\n{e}"),
                    (None, Ok(text)) => panic!("{body} names none, read as {text:?}"),
                }
            }
        }
    }
    assert!(
        read > 15_000 && refused > 1_000,
        "{read} read, {refused} refused"
    );
}

/// **No name the loader makes up is a name a program can write** (T1f.5.5.8,
/// on S1f.5.4's Q1, decided (a) on 2026-11-08).
///
/// The parser, the loader and the engine name what a program writes no name
/// for: the empty form `()`, a nested form whose head is not a name, a
/// parameter list, a fact's `:using` list and a query's goal
/// (`ein_core::text::INTERNAL_NAMES`). None is one `NAME` of the lexer, and
/// the parser refuses each where a name goes. The spellings they had until
/// then, `@empty`, `<nested>`, `@params`, `@list` and `<query>`, are each one
/// `NAME` under the name rule, which is why they moved.
#[test]
fn no_name_the_loader_makes_up_is_a_name() {
    for name in ein_core::text::INTERNAL_NAMES {
        assert!(
            !grammar::is_one_name(name),
            "{name} is a NAME of the generated lexer"
        );
        for text in [
            format!("(p {name})"),
            format!("(p ({name} a))"),
            format!("({name} a)"),
        ] {
            assert!(
                grammar::parse(&mut Ast::new(), &text, None).is_err(),
                "the parser reads {text}"
            );
        }
    }
    for old in ["@empty", "<nested>", "@params", "@list", "<query>"] {
        assert!(grammar::is_one_name(old), "{old} is no NAME");
    }
    assert!(grammar::is_one_name("a-1"));
    assert!(
        !grammar::is_one_name("rule"),
        "a reserved word is its own token"
    );
    assert!(!grammar::is_one_name("a b"));
}

/// The argument of the one `p` fact `text` stores, interned in `terms`.
fn p_argument(terms: &mut ein_core::Terms, text: &str) -> ein_core::Value {
    let text = format!("(relation p Node :why \"p holds {{?a}}\")\n{text}\n");
    let mut ast = Ast::new();
    let forms = grammar::parse(&mut ast, &text, Some("twin.ein")).expect("the parser reads it");
    let kb = ein_ir::load(&mut ast, terms, &forms, None).expect("it loads");
    let p = terms.syms.get("p").expect("p is interned");
    let fact = kb
        .facts()
        .find(|&f| terms.facts.get(f).0 == p)
        .expect("a p fact");
    terms.facts.args(fact)[0]
}

/// **`(p ())` and `(p (@empty))` are two facts, and so are `(p (_ a))` and
/// `(p (<nested> a))`** (T1f.5.5.8): the twins S1f.5.4's step 1 found on a
/// prototype that read the name rule with today's parser, where each pair
/// stored one fact ([its finding](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md#the-finding-two-synthetic-names-become-spellable)).
///
/// The grammar refuses `()` and `_` as a fact's argument (a fact holds names
/// and facts), so the loader's names are made by no fact a program writes,
/// and `(p (@empty))` and `(p (<nested> a))` load as the names they spell.
/// Until the swap (T1f.5.5.4) the hand-written parser read the first of each
/// pair into the same interner, and the two were held apart there. The
/// generated parser's own twin is `ein-infer/tests/rule_semantics.rs`'
/// `an_empty_conclusion_and_one_named_at_empty_are_two_facts`, and the two
/// places below read a written name as the parser's.
#[test]
fn the_loaders_names_and_the_names_a_program_writes_are_two() {
    let mut terms = ein_core::Terms::new();
    for (internal, made, written, head) in [
        (
            "(p ())",
            ein_core::text::EMPTY_HEAD,
            "(p (@empty))",
            "@empty",
        ),
        (
            "(p (_ a))",
            ein_core::text::NESTED_HEAD,
            "(p (<nested> a))",
            "<nested>",
        ),
    ] {
        assert!(
            grammar::parse(&mut Ast::new(), internal, None).is_err(),
            "{internal} is no fact a program writes"
        );
        let by_program = p_argument(&mut terms, written);
        let (rel, _) = terms.fact(by_program.as_fact().expect("a nested fact"));
        assert_eq!(terms.sym(rel), head);
        assert_ne!(terms.sym(rel), made);
    }

    // The dumper prints a synthetic head's form without its head, and a
    // pattern's relations leave out the synthetic heads: until T1f.5.5.8 the
    // first read any head with a leading `@` as one, and the second `@empty`
    // and `@params` as written.
    let text = "(@params b)\n(rule r () :match (and (@empty ?x) (@params ?x)) :assert (q ?x) :why \"q {?x}\")\n";
    let mut ast = Ast::new();
    let forms = grammar::parse(&mut ast, text, Some("heads.ein")).expect("it parses");
    assert_eq!(ein_ir::dump_compact(&ast, forms[0]), "(@params b)");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("it loads");
    let (_, rule) = kb.program().rules.iter().next().expect("one rule");
    let read: Vec<&str> = rule
        .match_
        .as_ref()
        .expect("a :match")
        .relation_names
        .iter()
        .map(|&r| terms.sym(r))
        .collect();
    assert_eq!(read, ["@empty", "@params"]);
}

/// A program whose deepest `(` is `depth` deep: a macro, a rule whose
/// `:match` nests a call of it at the limit, and a fact nested to it.
fn nested(depth: usize) -> String {
    let d = depth;
    format!(
        "(macro m (?x) (p ?x))\n\
         (rule r (?x) :match {}(m ?x){} :assert (q ?x))\n\
         {}a{}\n",
        "(p ".repeat(d - 2),
        ")".repeat(d - 2),
        "(p ".repeat(d),
        ")".repeat(d),
    )
}

/// **The nesting limit** (F10).
///
/// `ein solve` on a fact nested 100 000 deep aborted with a stack overflow
/// (2026-11-07), because what reads the tree after the parse recurses. At
/// [`NESTING_LIMIT`] the generated parser builds the tree, and the dumper,
/// macro expansion (through the loader, which expands the rule's call) and
/// the loader read it on this test's own thread, a debug build's 2 MiB test
/// stack; one past the limit is refused at the parse with one line naming the
/// `(` that passes it; and 100 000 deep is refused the same way rather than
/// aborting.
#[test]
fn the_nesting_limit() {
    let at = nested(NESTING_LIMIT);
    let mut ast = Ast::new();
    let forms = grammar::parse(&mut ast, &at, Some("at.ein")).expect("at the limit");
    assert_eq!(forms.len(), 3);
    let dumped = ein_ir::dump_canonical(&ast, &forms);
    assert!(
        dumped.lines().count() > 3,
        "the pretty dumper broke the deep forms"
    );
    assert_eq!(
        ein_ir::dump_compact(&ast, forms[2]),
        format!(
            "{}a{}",
            "(p ".repeat(NESTING_LIMIT),
            ")".repeat(NESTING_LIMIT)
        )
    );
    let mut terms = ein_core::Terms::new();
    // Whether it loads is the loader's to say; that it answers is the point.
    let loaded = ein_ir::load(&mut ast, &mut terms, &forms, None);
    if let Err(e) = &loaded {
        assert!(!e.to_string().is_empty());
    }

    let past = nested(NESTING_LIMIT + 1);
    let r = grammar::parse(&mut Ast::new(), &past, Some("past.ein")).expect_err("past the limit");
    let line = past.lines().position(|l| l.starts_with("(rule")).unwrap() + 1;
    let col = "(rule r (?x) :match ".len() + "(p ".len() * (NESTING_LIMIT - 1) + 1;
    assert_eq!(
        r.to_string(),
        format!(
            "past.ein:{line}:{}: this `(` nests deeper than the limit of {NESTING_LIMIT} parentheses",
            col
        ),
        "the first `(` past the limit, 1-based: its column counts the `(p ` before it"
    );

    let deep = format!("{}a{}", "(p ".repeat(100_000), ")".repeat(100_000));
    let r = grammar::parse(&mut Ast::new(), &deep, Some("deep.ein")).expect_err("deep");
    assert_eq!(r.errors.len(), 1);
    assert_eq!(
        (r.errors[0].line, r.errors[0].col),
        (1, 3 * NESTING_LIMIT as u32 + 1)
    );
}

fn relative(p: &Path) -> String {
    p.strip_prefix(repo_root())
        .unwrap_or(p)
        .to_string_lossy()
        .into_owned()
}

/// `ein.l`'s skip rule as it ships, and the three rules it replaced at
/// T1f.5.5.12, each as `ein.l` writes it, without a start condition.
const SKIP_RULE: &str =
    r"<INITIAL,AFTER_WORD>([\s\x1c-\x1f]|;[^\n]*|#\|([^|]|\|+[^|#])*\|+#)+ <INITIAL>;";
const THREE_SKIP_RULES: &str = "[\\s\\x1c-\\x1f]+ ;\n;[^\\n]* ;\n#\\|([^|]|\\|+[^|#])*\\|+# ;";

type LexerDef = lrlex::LRNonStreamingLexerDef<lrlex::DefaultLexerTypes<u32>>;

/// `ein.l` as T1f.5.5.4 shipped it, from `ein.l` as it ships: the skip rule
/// split back into the three it replaced, and the start condition
/// `AFTER_WORD` taken out, its declaration, the states a rule is tried in and
/// the state a rule enters.
fn as_t4_wrote_it(shipped: &str) -> String {
    let (mut out, mut entered, mut tried) = (Vec::new(), 0, 0);
    for line in shipped.lines() {
        if line == "%x AFTER_WORD" {
            continue;
        }
        if line == SKIP_RULE {
            out.push(THREE_SKIP_RULES.to_string());
            continue;
        }
        let mut line = line.to_string();
        if let Some(rest) = line.strip_prefix("<INITIAL,AFTER_WORD>") {
            line = rest.to_string();
            tried += 1;
        }
        for target in [" <AFTER_WORD>\"", " <INITIAL>\""] {
            if line.contains(target) {
                line = line.replacen(target, " \"", 1);
                entered += 1;
            }
        }
        out.push(line);
    }
    assert_eq!(tried, 2, "the parens are not tried after a word");
    assert!(entered > 50, "only {entered} rules enter a state");
    out.join("\n")
}

/// A token stream as the parser receives it, by rule name: each token's kind
/// and span, and each lexing error's position.
fn tokens(def: &LexerDef, text: &str) -> Vec<(String, usize, usize)> {
    use lrlex::LexerDef as _;
    use lrpar::{LexError as _, Lexeme as _, Lexer as _};
    def.lexer(text)
        .iter()
        .map(|t| match t {
            Ok(l) => {
                let rule = def
                    .iter_rules()
                    .find(|r| r.tok_id() == Some(l.tok_id()))
                    .and_then(|r| r.name())
                    .unwrap_or("?");
                (rule.to_string(), l.span().start(), l.span().len())
            }
            Err(e) => ("a lexing error".to_string(), e.span().start(), 0),
        })
        .collect()
}

/// **The lexer gives the parser the tokens T4's did** (T1f.5.5.12).
///
/// lrlex runs the regex of every rule the current start condition allows at
/// every position, and T1f.5.5.12 cut what it runs twice in `ein.l`: whitespace,
/// `;` comments and `#| … |#` comments are one rule where they were three, so
/// a run of them is one position; and after any token but a paren, the
/// exclusive state `AFTER_WORD` tries only `(`, `)` and that rule, the only
/// three that can match there. The tokens the parser receives must not move.
/// Each input is lexed by `ein.l` as it ships and by `ein.l` as T4 wrote it
/// ([`as_t4_wrote_it`]), both read at run time through `lrlex` (`build.rs`
/// compiles the same text with the same flags), and the two streams are
/// compared, kind and span, lexing errors included: every `.ein` under
/// `examples/`, `stdlib/`, `tests/`, `corpus/` and `plans/` (the fuzzer's
/// seed files among them), 500 seeded mutants of the corpus, edge cases
/// around the rules and the separators, and 20 000 seeded strings over the
/// characters they read.
#[test]
fn the_lexer_gives_the_tokens_t4s_did() {
    use lrlex::LexerDef as _;
    let shipped = include_str!("../src/ein.l");
    assert_eq!(
        shipped.lines().filter(|l| *l == SKIP_RULE).count(),
        1,
        "ein.l's skip rule is not the one this test holds"
    );
    let now = LexerDef::from_str(shipped).expect("ein.l");
    let t4 = LexerDef::from_str(&as_t4_wrote_it(shipped)).expect("ein.l as T4 wrote it");
    assert_eq!(t4.iter_rules().count(), now.iter_rules().count() + 2);
    assert_eq!(now.iter_start_states().count(), 2);
    assert_eq!(t4.iter_start_states().count(), 1);

    let mut inputs: Vec<(String, String)> = Vec::new();
    let root = repo_root();
    for dir in ["examples", "stdlib", "tests", "corpus", "plans"] {
        for p in ein_corpus::ein_files_under(&root.join(dir)) {
            let text = std::fs::read_to_string(&p).expect("readable");
            inputs.push((relative(&p), text));
        }
    }
    let files = inputs.len();
    let sources = ein_corpus::mutants::sources();
    for (i, (which, text)) in ein_corpus::mutants::mutants(&sources, 500, 0)
        .into_iter()
        .enumerate()
    {
        inputs.push((
            format!("mutant {i} of {}", relative(&sources[which].0)),
            text,
        ));
    }
    const EDGES: &[&str] = &[
        "",
        ";",
        ";\n",
        ";;;",
        "; é ☃ \u{a0}\n(p a)",
        ";\r\n(p a)",
        ";\x1c(p a)\n(q b)",
        "(p a);",
        "(p a) ; c\n(q b)",
        "(p a);c\n;d\n   ;e\n(q b)",
        "#|",
        "#||#",
        "#|||#",
        "#| a | b |#",
        "#| |# |#",
        "#| #| |# |#",
        "#|\n|#",
        "#|é☃|#",
        "#",
        "#a",
        "a#|b|#c",
        "#|#",
        "|#",
        "##|x|#",
        "#|x|",
        "#|x||",
        "#|x|#|#",
        "#|;|#;#||#",
        "\u{a0}(p a)\u{2028}",
        "\u{3000}\u{85}(p a)",
        "\x1c\x1d\x1e\x1f(p a)",
        "\x0b\x0c\r(p a)",
        "(p\u{a0}a)",
        "(p\u{200b}a)",
        "(p\u{feff}a)",
        "(p \"a ; b\")",
        "(p \"#|\")",
        "(p \"|#\")",
        "(p :why \"\\u{1F600}\")",
        "(p :why \"\\u{110000}\")",
        "(p :why \"\\u{}\")",
        "(p :why \"é\")",
        "(p :why \"a\\\"b\")",
        "(p :why \"a\\\\\")",
        "(p :why \"\\q\")",
        "(p :why \"x\"y)",
        "(p :why \"x\";c\n)",
        "(p :why \"abc",
        "(p :why \"a\nb\")",
        "(p :why \"a\\\nb\")",
        "\"x\"#|c|#",
        "\"x\"#c",
        "(<forced-positive> a)",
        "(p <forced-positive>x)",
        "(p <monotonic-unconditional>;c\n)",
        "(p <lookahead-dies-immediately>#|c|#)",
        "(p a;b)",
        "(p a#|b|#)",
        "(p a#b)",
        "(p ?x;)",
        "(p :k#|c|# v)",
        "(p _)",
        "(p ?_)",
        "(p __)",
        "(p 1..5)",
        "(p -7)",
        "(p it's)",
        "(p a,b)",
        "(p a\x00b)",
        "(p a\x7fb)",
        "(p a);c\n(q b)#|x|#(r c)",
        "( ; c\n p ; d\n a ; e\n )",
    ];
    inputs.extend(EDGES.iter().map(|e| (format!("{e:?}"), e.to_string())));
    const CHARS: &[char] = &[
        ' ',
        '\n',
        '\t',
        '\r',
        '\x0b',
        '\x0c',
        '\x1c',
        '\x1d',
        '\x1e',
        '\x1f',
        '\u{85}',
        '\u{a0}',
        '\u{1680}',
        '\u{2000}',
        '\u{2028}',
        '\u{2029}',
        '\u{3000}',
        '\u{feff}',
        '\u{200b}',
        ';',
        ';',
        '#',
        '#',
        '|',
        '|',
        '"',
        '"',
        '\\',
        '(',
        ')',
        'a',
        'Z',
        '0',
        '7',
        '_',
        '-',
        '.',
        '?',
        ':',
        '<',
        '>',
        '{',
        '}',
        'u',
        'n',
        'é',
        '☃',
        '\u{1F600}',
        '\x00',
        '\x7f',
        ',',
    ];
    const PIECES: &[&str] = &[
        "#|",
        "|#",
        "||",
        "##",
        ";;",
        "\\u{41}",
        "\\u{110000}",
        "\\u{}",
        "\\n",
        "\\q",
        "\\\"",
        "<forced-positive>",
        "<monotonic-unconditional>",
        "<lookahead-dies-immediately>",
        "?_",
        ":why",
        "not",
        "rule",
        "relation",
        "1..5",
        "it's",
        " ; c\n",
        "#| c |#",
        "\"s\"",
        "\r\n",
    ];
    // xorshift64*, as `fuzz_properties` draws its stream.
    let mut x: u64 = 0x5EED_1A17 | 1;
    let mut next = move || {
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D) as usize
    };
    for i in 0..20_000 {
        let mut text = String::new();
        for _ in 0..next() % 48 {
            if next() % 4 == 0 {
                text.push_str(PIECES[next() % PIECES.len()]);
            } else {
                text.push(CHARS[next() % CHARS.len()]);
            }
        }
        inputs.push((format!("string {i}"), text));
    }

    let (mut moved, mut lexemes, mut errors) = (Vec::new(), 0, 0);
    for (name, text) in &inputs {
        let (a, b) = (tokens(&now, text), tokens(&t4, text));
        lexemes += a.len();
        errors += a.iter().filter(|t| t.0 == "a lexing error").count();
        if a != b {
            let at = a.iter().zip(&b).take_while(|(p, q)| p == q).count();
            moved.push(format!(
                "{name}: token {at}: {:?} now, {:?} as T4 wrote it",
                a.get(at),
                b.get(at)
            ));
        }
    }
    assert!(
        moved.is_empty(),
        "{} of {} inputs lex otherwise than under T4's ein.l:\n{}",
        moved.len(),
        inputs.len(),
        moved.join("\n")
    );
    assert!(files > 500, "only {files} files");
    assert!(
        lexemes > 100_000 && errors > 1_000,
        "{lexemes} tokens, {errors} lexing errors"
    );
}
