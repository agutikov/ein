//! The parser, generated from `src/ein.l` and `src/ein.y` (Q-M1e2.25).
//!
//! | claim | test |
//! |---|---|
//! | a refused file's report is the same twice in one process | [`a_report_is_the_same_twice`] |
//! | a report cut at the step budget says so | [`a_report_cut_at_the_budget_says_so`] |
//! | a macro carries its `:why` (Q-M1f.59) | [`a_macro_carries_its_why`] |
//! | no name the loader makes up is a name a program can write (Q-M1f.339) | [`no_name_the_loader_makes_up_is_a_name`] |
//! | a program at the nesting limit parses and goes through the dumper, macro expansion and the loader on a test thread; one past it is refused with a line | [`the_nesting_limit`] |
//! | `ein.l`'s one skip rule and its start condition give the parser the tokens its plain form does | [`the_lexer_gives_the_tokens_t4s_did`] |
//! | a string's escapes are the decided ones, and a `\u{…}` reads exactly the Unicode scalar values (Q-M1e2.23, W38) | [`a_strings_escapes_are_the_rulings`] |
//!
//! The fuzzer's half of the same-report claim is in `fuzz_properties.rs`.

use std::path::{Path, PathBuf};

use ein_corpus::{golden, golden_path, repo_root};
use ein_ir::Ast;
use ein_ir::grammar::{self, NESTING_LIMIT, RECOVERY_STEP_BUDGET};

/// Every `.ein` under the four corpus roots, relative to the repo, sorted:
/// `examples/`, `stdlib/`, `tests/` and `corpus/`.
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

/// Every input whose report the same-report tests compare: each corpus file
/// the generated parser refuses, then `n` seeded mutants of the corpus
/// (`ein_corpus::mutants`).
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

/// **A refused file's report is the same twice, in one process**.
///
/// `lrpar` 0.15 ranked repairs of equal length in a `HashSet`'s order, and
/// every `HashSet` of a process draws its own hash keys, so two parses of one
/// file in one process could list its repairs in two orders, and go on with
/// different ones; and it stopped recovery at 500 ms of wall clock, so a
/// loaded machine reported fewer errors. The vendored patch ranks equal
/// repairs in the order the search found them and counts search steps. Each
/// input here is parsed twice and the two reports compared byte for byte:
/// every corpus file the generated parser refuses, and 1 000 seeded mutants.
/// With the ranking unpatched it fails.
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

/// **A report cut at the step budget says so**.
///
/// `golden/recovery_cut.ein` is two mistakes recovery repairs and then a
/// comment whose second line lost its semicolons, so that its prose reads as
/// tokens: the search for a repair there runs past [`RECOVERY_STEP_BUDGET`]
/// (it needs 73 234 steps, re-taken with the vendored constant raised).
/// Unpatched, `lrpar` gives such an error no repairs, which reads
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

/// **The end of the input is where the text ends**: an input that stops
/// while a form is open is refused at its last character's end, whatever
/// whitespace or comment follows the last token. `lrpar` makes its
/// end-of-input lexeme at the end of the last token, which the report does not
/// use, or `(p a` and a newline would say `1:5`.
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

/// **A macro carries its `:why`**, as a relation and a rule do: every form
/// an NL template can go on takes one (Q-M1f.59). The parser reads
/// `:why "…"` once after a macro's body and the macro reader keeps it
/// (`Macro::why`). Whether a program must write one is the loader's decision,
/// required by default and turned off by `--no-require-why` (Q-M1f.62).
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

/// **A string's escapes are the decided ones, and a `\u{…}` reads exactly
/// the Unicode scalar values** (W38).
///
/// A string is Unicode text with the escapes `\n \t \r \" \\` and `\u{…}`,
/// one to six hex digits naming that code point (Q-M1e2.23), and any other
/// escape is refused at the parse. A code point that names no character, one
/// past 10FFFF or a surrogate (D800 to DFFF), is no escape either: `ein.l`'s
/// `TEXT` takes only a scalar value's digits, so no such escape is read as
/// U+FFFD in silence. Held over every boundary of the scalar values in every width
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

/// **No name the loader makes up is a name a program can write**
/// (Q-M1f.339).
///
/// The parser, the loader and the engine name what a program writes no name
/// for: the empty form `()`, a nested form whose head is not a name, a
/// parameter list, a fact's `:using` list and a query's goal
/// (`ein_core::text::INTERNAL_NAMES`). None is one `NAME` of the lexer, and
/// the parser refuses each where a name goes. `@empty`, `<nested>`,
/// `@params`, `@list` and `<query>` are each one `NAME` under the name rule,
/// so none of them is one of the loader's names.
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
/// `(p (<nested> a))`**: under the name rule each pair's second member is a
/// name a program can write, and a loader whose made-up names were spelt so
/// would store each pair as one fact.
///
/// The grammar refuses `()` and `_` as a fact's argument (a fact holds names
/// and facts), so the loader's names are made by no fact a program writes,
/// and `(p (@empty))` and `(p (<nested> a))` load as the names they spell.
/// The generated parser's own twin is `ein-infer/tests/rule_semantics.rs`'
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
    // pattern's relations leave out the synthetic heads: neither reads a head
    // with a leading `@`, `@empty` or `@params`, as written, as one.
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

/// **The nesting limit**.
///
/// What reads the tree after the parse recurses, so a fact nested 100 000
/// deep would abort `ein solve` with a stack overflow were it read. At
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

/// `ein.l`'s skip rule as it ships, and the three plain rules it stands
/// for, each as `ein.l` writes it, without a start condition. The whitespace
/// in both is ASCII's, never `\s`, Unicode's (W73): the two lexers this test
/// compares read one class.
const SKIP_RULE: &str =
    r"<INITIAL,AFTER_WORD>([\x09-\x0D\x20\x1C-\x1F]|;[^\n]*|#\|([^|]|\|+[^|#])*\|+#)+ <INITIAL>;";
const THREE_SKIP_RULES: &str =
    "[\\x09-\\x0D\\x20\\x1C-\\x1F]+ ;\n;[^\\n]* ;\n#\\|([^|]|\\|+[^|#])*\\|+# ;";

type LexerDef = lrlex::LRNonStreamingLexerDef<lrlex::DefaultLexerTypes<u32>>;

/// `ein.l`'s plain form, from `ein.l` as it ships: the skip rule split into
/// the three it stands for, and the start condition `AFTER_WORD` taken out,
/// its declaration, the states a rule is tried in and the state a rule
/// enters.
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

/// **The lexer gives the parser the tokens its plain form does**.
///
/// lrlex runs the regex of every rule the current start condition allows at
/// every position, and `ein.l` cuts what it runs twice: whitespace, `;`
/// comments and `#| … |#` comments are one rule, not three, so a run of them
/// is one position; and after any token but a paren, the exclusive state
/// `AFTER_WORD` tries only `(`, `)` and that rule, the only three that can
/// match there. The tokens the parser receives must be the plain form's.
/// Each input is lexed by `ein.l` as it ships and by its plain form
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
