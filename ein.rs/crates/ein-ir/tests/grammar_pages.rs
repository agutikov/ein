//! **The grammar's pages.**
//!
//! The generated parser is the grammar: `src/ein.l` and `src/ein.y`, which
//! `build.rs` compiles. This file keeps two things in step with them.
//!
//! - **The page.** `docs/ein/user/04-reference/01-grammar.md`'s two grammar
//!   regions are written here, from the two files, read through grmtools'
//!   public readers (`lrlex`'s `LRNonStreamingLexerDef` and `cfgrammar`'s
//!   `YaccGrammar`), so the page cannot state a token or a production the
//!   parser does not have. A region is the file as EBNF, one diagram a
//!   group of rules: a production's alternatives as they stand, a
//!   left-recursive list as the repetition it derives ([`phrase_rule`]), and
//!   a lexer rule's regular expression translated node for node of the
//!   syntax tree `regex-syntax` reads it as ([`regex_ebnf`]). A diagram's
//!   source is a file beside the page, `diagrams/01-grammar-….puml`, the
//!   rules between `@startebnf` and `@endebnf`, and the region shows the
//!   picture drawn from it (Q-M1f.368). Nothing of a diagram is typed by
//!   hand: the two tables here name only which rule each diagram starts at,
//!   its title and its file (Q-M1f.378). The test fails when a region or a
//!   source differs, or when a source is left that no group writes, and
//!   `EIN_BLESS=1` rewrites the two marked regions and the sources, as
//!   `ein-cli/tests/embedding.rs` does for `docs/ein/ein.rs/04-api.md`.
//! - **The grammar's table** ([`TABLE`]). One row for each shape the grammar
//!   refuses at the parse, with the parse error it is refused with, banked in
//!   `tests/golden/grammar_table.txt`. A row that names a refusal line
//!   ([`DELETED`]) is a shape the loader, the compiler or the firing refused
//!   before the grammar did; that line has no code left, and a listed line
//!   with no row fails here. The refusals the grammar does not cover stay
//!   where they are, and
//!   [`the_lines_the_grammar_does_not_cover_are_still_reached`] holds a
//!   program reaching each. After the rows of the forms, place by place, come
//!   the language's refusals, each in every place it applies: a token that is
//!   no name, in every place a name goes; a string and a variable in a fact,
//!   wherever a value goes; a bare `_` and `?_`, wherever a name or a variable
//!   goes; a keyword each form does not take, one row a form, and a keyword
//!   pair in each place in a term; `(and)` and `(or)`, each with the note
//!   naming the rule and the clause, in each place a connective is written;
//!   `open` outside a rule's whole conclusion; a character that is not ASCII
//!   outside a string and a comment; `absent` as a fact's, a value's and a
//!   conclusion's relation, `false` with an argument, and `(eq …)` as a
//!   value, each in every place the grammar takes the refusal; `:no-search`
//!   on a query or with a value; and what a rule may not conclude, a disjunction, whole or
//!   inside an `(and …)` at any depth, and a connective as a conclusion's
//!   argument or under its `not`.

use std::fmt::Write as _;
use std::str::FromStr as _;

use cfgrammar::Symbol;
use cfgrammar::yacc::YaccGrammar;
use ein_core::Terms;
use ein_corpus::{golden, golden_path, repo_root};
use ein_ir::{Ast, load, parse};
use lrlex::{DefaultLexerTypes, LRNonStreamingLexerDef, LexerDef};
use regex_syntax::ast::parse::ParserBuilder;
use regex_syntax::ast::{
    Ast as Regex, ClassBracketed, ClassPerlKind, ClassSet, ClassSetBinaryOpKind, ClassSetItem,
    RepetitionKind, RepetitionRange,
};

// ── The page ───────────────────────────────────────────────────────

/// The page the regions are written into.
const PAGE: &str = "docs/ein/user/04-reference/01-grammar.md";

/// The folder of the page's diagrams, and what every source this file
/// writes there is named by.
const DIAGRAMS: &str = "docs/ein/user/04-reference/diagrams";
const SOURCE_PREFIX: &str = "01-grammar-";

/// A grammar file, as `build.rs` reads it.
fn source(name: &str) -> String {
    let path = repo_root().join("ein.rs/crates/ein-ir/src").join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn grammar() -> YaccGrammar<u32> {
    YaccGrammar::<u32>::from_str(&source("ein.y")).expect("ein.y reads through cfgrammar")
}

// ── EBNF, as the page's diagrams write it ──────────────────────────

/// An EBNF expression in ISO 14977's notation, the one PlantUML draws
/// between `@startebnf` and `@endebnf`.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Ebnf {
    /// A terminal's text, written quoted.
    Lit(String),
    /// A rule's name, or the name of a class of tokens.
    Ref(String),
    /// A special sequence, `? … ?`: a set of characters, said in words.
    Special(String),
    Seq(Vec<Self>),
    Alt(Vec<Self>),
    /// `[ x ]`.
    Opt(Box<Self>),
    /// `{ x }`: none or more.
    Rep0(Box<Self>),
    /// `{ x }-`: one or more.
    Rep1(Box<Self>),
    /// `n * x`.
    Times(u32, Box<Self>),
}

/// The width a rule is written within, where it can be.
const EBNF_WIDTH: usize = 100;
/// The width the page's own lines are written within.
const PAGE_WIDTH: usize = 116;

impl Ebnf {
    /// A sequence, flat: a member that is a sequence gives its members, and a
    /// sequence of one is that one.
    fn seq(items: Vec<Self>) -> Self {
        let mut flat: Vec<Self> = Vec::new();
        for i in items {
            match i {
                Self::Seq(v) => flat.extend(v),
                other => flat.push(other),
            }
        }
        if flat.len() == 1 {
            flat.remove(0)
        } else {
            Self::Seq(flat)
        }
    }

    /// An alternation, flat, as [`Ebnf::seq`] is.
    fn alt(items: Vec<Self>) -> Self {
        let mut flat: Vec<Self> = Vec::new();
        for i in items {
            match i {
                Self::Alt(v) => flat.extend(v),
                other => flat.push(other),
            }
        }
        if flat.len() == 1 {
            flat.remove(0)
        } else {
            Self::Alt(flat)
        }
    }

    /// `n` of `x`: one is `x` itself.
    fn times(n: u32, x: Self) -> Self {
        if n == 1 {
            x
        } else {
            Self::Times(n, Box::new(x))
        }
    }

    /// The expression on one line.
    fn flat(&self) -> String {
        match self {
            // A terminal that holds `"` is quoted with `'`: no terminal of
            // either grammar file holds both.
            Self::Lit(s) => {
                assert!(
                    !(s.contains('"') && s.contains('\'')),
                    "the terminal {s:?} holds both quotes, which no EBNF literal can"
                );
                if s.contains('"') {
                    format!("'{s}'")
                } else {
                    format!("\"{s}\"")
                }
            }
            Self::Ref(n) => n.clone(),
            Self::Special(s) => format!("? {s} ?"),
            Self::Seq(v) => v
                .iter()
                .map(|e| match e {
                    Self::Alt(_) => format!("( {} )", e.flat()),
                    _ => e.flat(),
                })
                .collect::<Vec<_>>()
                .join(" , "),
            Self::Alt(v) => v.iter().map(Self::flat).collect::<Vec<_>>().join(" | "),
            Self::Opt(e) => format!("[ {} ]", e.flat()),
            Self::Rep0(e) => format!("{{ {} }}", e.flat()),
            Self::Rep1(e) => format!("{{ {} }}-", e.flat()),
            Self::Times(n, e) => match **e {
                Self::Seq(_) | Self::Alt(_) | Self::Times(..) => format!("{n} * ( {} )", e.flat()),
                _ => format!("{n} * {}", e.flat()),
            },
        }
    }

    /// The expression from column `indent`: on one line where it fits within
    /// [`EBNF_WIDTH`], and else broken at its alternatives and its members,
    /// each line after the first carrying its own indentation.
    fn laid_out(&self, indent: usize) -> String {
        let flat = self.flat();
        // Two columns are kept for the ` ;` that ends a rule.
        if indent + flat.chars().count() + 2 <= EBNF_WIDTH {
            return flat;
        }
        let pad = " ".repeat(indent);
        let bracketed = |open: &str, close: &str, e: &Self| -> String {
            format!("{open}\n{pad}  {}\n{pad}{close}", e.laid_out(indent + 2))
        };
        match self {
            // Alternatives that are each a name or a terminal fill a line.
            Self::Alt(v) if v.iter().all(|e| matches!(e, Self::Lit(_) | Self::Ref(_))) => {
                let mut out = String::from(" ");
                let mut col = indent + 1;
                for (i, e) in v.iter().map(Self::flat).enumerate() {
                    let w = e.chars().count();
                    if i > 0 && col + 3 + w > EBNF_WIDTH {
                        write!(out, "\n{pad}| {e}").unwrap();
                        col = indent + 2 + w;
                    } else {
                        write!(out, "{}{e}", if i == 0 { " " } else { " | " }).unwrap();
                        col += w + if i == 0 { 1 } else { 3 };
                    }
                }
                out
            }
            Self::Alt(v) => {
                let mut out = String::new();
                for (i, e) in v.iter().enumerate() {
                    if i == 0 {
                        write!(out, "  {}", e.laid_out(indent + 2)).unwrap();
                    } else {
                        write!(out, "\n{pad}| {}", e.laid_out(indent + 2)).unwrap();
                    }
                }
                out
            }
            Self::Seq(v) => v
                .iter()
                .map(|e| match e {
                    Self::Alt(_) if indent + e.flat().chars().count() + 4 > EBNF_WIDTH => {
                        bracketed("(", ")", e)
                    }
                    Self::Alt(_) => format!("( {} )", e.flat()),
                    _ => e.laid_out(indent),
                })
                .collect::<Vec<_>>()
                .join(&format!(" ,\n{pad}")),
            Self::Opt(e) => bracketed("[", "]", e),
            Self::Rep0(e) => bracketed("{", "}", e),
            Self::Rep1(e) => bracketed("{", "}-", e),
            Self::Lit(_) | Self::Ref(_) | Self::Special(_) | Self::Times(..) => flat,
        }
    }

    /// The rule `name = self ;`. PlantUML reads a line that starts with `'`
    /// as a comment and drops it, so a line that would start with a terminal
    /// quoted that way is written at the end of the line before it.
    fn rule(&self, name: &str) -> String {
        let flat = format!("{name} = {} ;", self.flat());
        if flat.chars().count() <= EBNF_WIDTH {
            return flat;
        }
        let mut out = String::new();
        for line in format!("{name} =\n  {} ;", self.laid_out(2)).lines() {
            if out.is_empty() {
                out.push_str(line);
            } else if line.trim_start().starts_with('\'') {
                out.push(' ');
                out.push_str(line.trim_start());
            } else {
                out.push('\n');
                out.push_str(line);
            }
        }
        out
    }
}

/// One diagram's source: the rules between `@startebnf` and `@endebnf`. No
/// line starts with a character PlantUML reads a line by before it reads the
/// grammar: `'`, a comment, `!`, a directive, and `@`, a diagram's end.
fn diagram(rules: &[String]) -> String {
    for line in rules.iter().flat_map(|r| r.lines()) {
        assert!(
            !line.trim_start().starts_with(['\'', '!', '@']),
            "PlantUML would not read this line as part of its rule: {line}"
        );
    }
    format!("@startebnf\n{}\n@endebnf\n", rules.join("\n"))
}

/// A diagram's source file: its name in [`DIAGRAMS`], and its text.
type Source = (String, String);

/// The page's link to a grammar file.
fn file_link(name: &str) -> String {
    format!("[`{name}`](../../../../ein.rs/crates/ein-ir/src/{name})")
}

// ── The lexical grammar ────────────────────────────────────────────

/// A character as a special sequence names it: itself where it is a printing
/// ASCII character other than `?`, which would end the sequence, and its code
/// point otherwise.
fn shown(c: char) -> String {
    if c.is_ascii_graphic() && c != '?' {
        c.to_string()
    } else {
        format!("U+{:04X}", u32::from(c))
    }
}

/// The members of a character class, as words: a character, a range `a-b`,
/// `whitespace` for `\s`, and a class inside the class as its own members.
fn members(set: &ClassSet) -> Vec<String> {
    fn item(i: &ClassSetItem, out: &mut Vec<String>) {
        match i {
            ClassSetItem::Empty(_) => {}
            ClassSetItem::Literal(l) => out.push(shown(l.c)),
            ClassSetItem::Range(r) => out.push(format!("{}-{}", shown(r.start.c), shown(r.end.c))),
            ClassSetItem::Perl(p) if p.kind == ClassPerlKind::Space && !p.negated => {
                out.push("whitespace".to_string());
            }
            ClassSetItem::Bracketed(b) if !b.negated => out.extend(members(&b.kind)),
            ClassSetItem::Union(u) => u.items.iter().for_each(|i| item(i, out)),
            other => panic!("ein.l writes a class member the page has no words for: {other:?}"),
        }
    }
    let ClassSet::Item(i) = set else {
        panic!("ein.l nests a class operator, which the page has no words for: {set:?}")
    };
    let mut out = Vec::new();
    item(i, &mut out);
    out
}

/// A character class. A class of a few printing characters is their
/// alternation, so each is drawn as the terminal it is; any other is a
/// special sequence that lists its members, `one of: …`, `any character but:
/// …` for a negated class, and `one of: …, but none of: …` for a difference,
/// `[a--[b]]`.
fn class(c: &ClassBracketed) -> Ebnf {
    /// The most members a class is drawn as alternatives with.
    const FEW: usize = 6;
    if let (false, ClassSet::Item(ClassSetItem::Literal(l))) = (c.negated, &c.kind) {
        return literal(l.c);
    }
    if let (false, ClassSet::Item(ClassSetItem::Union(u))) = (c.negated, &c.kind) {
        let chars: Option<Vec<char>> = u
            .items
            .iter()
            .map(|i| match i {
                ClassSetItem::Literal(l) if l.c.is_ascii_graphic() => Some(l.c),
                _ => None,
            })
            .collect();
        if let Some(chars) = chars.filter(|v| v.len() <= FEW) {
            return Ebnf::alt(chars.into_iter().map(literal).collect());
        }
    }
    let words = match (&c.kind, c.negated) {
        (ClassSet::Item(_), false) => format!("one of: {}", members(&c.kind).join(" ")),
        (ClassSet::Item(_), true) => format!("any character but: {}", members(&c.kind).join(" ")),
        (ClassSet::BinaryOp(op), false) if op.kind == ClassSetBinaryOpKind::Difference => format!(
            "one of: {}, but none of: {}",
            members(&op.lhs).join(" "),
            members(&op.rhs).join(" ")
        ),
        _ => panic!("ein.l writes a class the page has no words for: {c:?}"),
    };
    Ebnf::Special(words)
}

/// One character of a rule's text: a terminal, or its code point where it
/// does not print.
fn literal(c: char) -> Ebnf {
    if c.is_ascii_graphic() {
        Ebnf::Lit(c.to_string())
    } else {
        Ebnf::Special(shown(c))
    }
}

/// A lexer rule's regular expression as EBNF, node for node of the syntax
/// tree `regex` reads it as: a literal is a terminal and a run of them one
/// terminal, a concatenation a sequence, `|` an alternation, `?` an option,
/// `*` and `+` a repetition, `{m,n}` `m` of it and `n − m` options of it, and
/// a class what [`class`] makes of it. A construct with no EBNF here fails
/// the test, so the page never states less than the rule.
fn regex_ebnf(ast: &Regex) -> Ebnf {
    match ast {
        Regex::Literal(l) => literal(l.c),
        Regex::Concat(c) => {
            // A run of characters is one terminal; `"` stays alone, so that
            // the terminal it is in can be quoted.
            let mut items: Vec<Ebnf> = Vec::new();
            for e in c.asts.iter().map(regex_ebnf) {
                match (items.last_mut(), &e) {
                    (Some(Ebnf::Lit(a)), Ebnf::Lit(b)) if !a.contains('"') && !b.contains('"') => {
                        a.push_str(b);
                    }
                    _ => items.push(e),
                }
            }
            Ebnf::seq(items)
        }
        Regex::Alternation(a) => Ebnf::alt(a.asts.iter().map(regex_ebnf).collect()),
        Regex::Group(g) => regex_ebnf(&g.ast),
        Regex::Repetition(r) => {
            assert!(r.greedy, "ein.l writes a lazy repetition: {ast}");
            let x = regex_ebnf(&r.ast);
            let opt = |x: &Ebnf| Ebnf::Opt(Box::new(x.clone()));
            match r.op.kind {
                RepetitionKind::ZeroOrOne => opt(&x),
                RepetitionKind::ZeroOrMore => Ebnf::Rep0(Box::new(x)),
                RepetitionKind::OneOrMore => Ebnf::Rep1(Box::new(x)),
                RepetitionKind::Range(RepetitionRange::Exactly(n)) => Ebnf::times(n, x),
                RepetitionKind::Range(RepetitionRange::AtLeast(n)) => {
                    let mut parts = Vec::new();
                    if n > 0 {
                        parts.push(Ebnf::times(n, x.clone()));
                    }
                    parts.push(Ebnf::Rep0(Box::new(x)));
                    Ebnf::seq(parts)
                }
                RepetitionKind::Range(RepetitionRange::Bounded(m, n)) => {
                    let mut parts = Vec::new();
                    if m > 0 {
                        parts.push(Ebnf::times(m, x.clone()));
                    }
                    if n > m {
                        parts.push(Ebnf::times(n - m, opt(&x)));
                    }
                    assert!(!parts.is_empty(), "ein.l repeats `{ast}` no times");
                    Ebnf::seq(parts)
                }
            }
        }
        Regex::ClassBracketed(c) => class(c),
        other => panic!("ein.l writes `{other}`, which the page's EBNF has no rule for"),
    }
}

/// A lexer rule as the page shows it.
struct Lexed {
    /// Its EBNF name: the token's, or the name of the group of tokens it is
    /// listed in.
    name: String,
    ebnf: Ebnf,
    /// The start conditions it is tried in, the one it enters, and the name a
    /// parse error gives its token.
    tried: Vec<String>,
    then: Option<String>,
    error: Option<String>,
}

/// The names `ein.l` declares as exclusive start conditions, `%x`: a rule
/// that names no condition is not tried in one.
fn exclusive_states(src: &str) -> Vec<String> {
    src.lines()
        .take_while(|l| l.trim() != "%%")
        .filter_map(|l| l.strip_prefix("%x"))
        .flat_map(|l| l.split_whitespace().map(str::to_string))
        .collect()
}

/// The lexer's rules, in the file's order but for the tokens that are their
/// own text, which come first, gathered by kind: one alternation each for the
/// punctuation, the words no name is spelt as (`reserved-word`), the words
/// that are names too, which are those `ein.y`'s `Name` takes
/// (`position-word`), and the keywords (`known-keyword`).
fn lexed() -> Vec<Lexed> {
    type Def = LRNonStreamingLexerDef<DefaultLexerTypes<u32>>;
    let src = source("ein.l");
    let def = Def::from_str(&src).expect("ein.l reads through lrlex");
    let exclusive = exclusive_states(&src);
    let states: Vec<&str> = def.iter_start_states().map(|s| s.name()).collect();
    let state = |id: usize| -> String {
        states
            .get(id)
            .copied()
            .expect("a declared start state")
            .to_string()
    };
    let grm = grammar();
    // The words `Name` takes: its productions of one token.
    let name_rule = grm.rule_idx("Name").expect("ein.y has the rule Name");
    let names_too: Vec<&str> = grm
        .rule_to_prods(name_rule)
        .iter()
        .filter_map(|&p| match grm.prod(p) {
            [Symbol::Token(t)] => grm.token_name(*t),
            _ => None,
        })
        .collect();
    let mut own_text: Vec<Lexed> = Vec::new();
    let mut classes: Vec<Lexed> = Vec::new();
    for r in def.iter_rules() {
        let ast = ParserBuilder::new()
            .octal(true)
            .build()
            .parse(r.re_str())
            .unwrap_or_else(|e| panic!("ein.l's rule `{}` does not read: {e}", r.re_str()));
        let ebnf = regex_ebnf(&ast);
        // No condition named: the initial one, and every one that is not
        // exclusive.
        let tried: Vec<String> = if r.start_states().is_empty() {
            states
                .iter()
                .filter(|s| !exclusive.iter().any(|x| x == *s))
                .map(|s| (*s).to_string())
                .collect()
        } else {
            r.start_states().iter().map(|&i| state(i)).collect()
        };
        let then = r.target_state().map(|(to, _)| state(to));
        let error = r.name().and_then(|n| {
            let t = grm.token_idx(n)?;
            grm.token_epp(t).filter(|e| *e != n).map(str::to_string)
        });
        match r.name() {
            Some(n) if ebnf == Ebnf::Lit(n.to_string()) => {
                // The page lists these before the classes, and the lexer
                // takes the earlier of two rules that match one length: a
                // word written after `NAME` would never be its own token.
                assert!(
                    classes.is_empty(),
                    "ein.l writes the token `{n}` after a rule that is no fixed text: the page \
                     lists the rules in another order than the file's"
                );
                let first = n.chars().next().expect("a token has a name");
                let kind = if first == ':' {
                    "known-keyword"
                } else if !first.is_ascii_alphabetic() {
                    "punctuation"
                } else if names_too.contains(&n) {
                    "position-word"
                } else {
                    "reserved-word"
                };
                if let Some(g) = own_text.iter_mut().find(|g| g.name == kind) {
                    assert!(
                        g.tried == tried && g.then == then && error.is_none(),
                        "the token `{n}` is tried or named unlike the other `{kind}` tokens: \
                         the page lists them as one rule"
                    );
                    let Ebnf::Alt(v) = &mut g.ebnf else {
                        unreachable!()
                    };
                    v.push(ebnf);
                } else {
                    assert!(
                        error.is_none(),
                        "`{n}` is its own text and has another name"
                    );
                    own_text.push(Lexed {
                        name: kind.to_string(),
                        ebnf: Ebnf::Alt(vec![ebnf]),
                        tried,
                        then,
                        error,
                    });
                }
            }
            n => classes.push(Lexed {
                name: n.unwrap_or("skipped").to_string(),
                ebnf,
                tried,
                then,
                error,
            }),
        }
    }
    own_text.extend(classes);
    own_text
}

/// The lexical grammar's diagrams: the rule each starts at, its title, and
/// the name of its source file after `01-grammar-lexical-`. A diagram holds
/// the rules from its first up to the next diagram's, in [`lexed`]'s order,
/// so a rule added to `ein.l` is drawn with the rule it is written after.
const LEXICAL_GROUPS: &[(&str, &str, &str)] = &[
    (
        "punctuation",
        "The tokens that are their own text",
        "own-text",
    ),
    (
        "UNDERSCORE",
        "The reserved spellings, names, variables and keywords",
        "names-variables-keywords",
    ),
    ("TEXT", "Strings", "strings"),
    (
        "BAD",
        "What is no token, and what separates tokens",
        "no-token-and-separators",
    ),
];

/// The phrase grammar's diagrams, as [`LEXICAL_GROUPS`], in `ein.y`'s order;
/// a source's name follows `01-grammar-phrase-`.
const PHRASE_GROUPS: &[(&str, &str, &str)] = &[
    ("Program", "A program and its forms", "program"),
    ("Name", "Names, and the name a form is headed by", "names"),
    (
        "Var",
        "Variables, strings, flags and lists of names",
        "variables-strings-flags",
    ),
    (
        "Fact",
        "A fact, a stored negation and a fact as a value",
        "fact",
    ),
    ("Relation", "A relation's declaration", "relation"),
    ("Rule", "A rule and an hrule", "rule"),
    ("Match", "A rule's `:match`: premises", "match"),
    ("Pattern", "A pattern and its terms", "pattern"),
    ("Assert", "A rule's `:assert`: conclusions", "assert"),
    ("Query", "A query and its claim", "query"),
    ("Solve", "A solve and its claim", "solve"),
    ("Config", "A config", "config"),
    ("Macro", "A macro", "macro"),
    ("Import", "An import", "import"),
    ("Trace", "A trace", "trace"),
];

/// `rules`, cut into the diagrams `groups` names: the region's text, each
/// diagram under a heading `### <section>.<n> <title>`, a line that links the
/// grammar file and the diagram's source and lists its rules, and the
/// picture; and the sources, each `01-grammar-<kind>-<name>.puml`.
fn grouped(
    section: usize,
    file: &str,
    kind: &str,
    groups: &[(&str, &str, &str)],
    rules: &[(String, Ebnf)],
) -> (String, Vec<Source>) {
    let starts: Vec<usize> = groups
        .iter()
        .map(|(first, _, _)| {
            rules
                .iter()
                .position(|(n, _)| n == first)
                .unwrap_or_else(|| panic!("{file} has no rule `{first}` to start a diagram at"))
        })
        .collect();
    assert_eq!(
        starts.first(),
        Some(&0),
        "the first diagram starts at {file}'s first rule"
    );
    assert!(
        starts.windows(2).all(|w| w[0] < w[1]),
        "the diagrams follow {file}'s order"
    );
    let mut out = String::new();
    let mut sources = Vec::new();
    for (g, (_, title, name)) in groups.iter().enumerate() {
        let end = starts.get(g + 1).copied().unwrap_or(rules.len());
        let part = &rules[starts[g]..end];
        let lines: Vec<String> = part.iter().map(|(n, e)| e.rule(n)).collect();
        // The rules' names, filling the page's lines.
        let mut names = String::new();
        let mut col = 0;
        for (i, (n, _)) in part.iter().enumerate() {
            let word = format!("`{n}`{}", if i + 1 == part.len() { "." } else { "," });
            if col > 0 && col + 1 + word.len() > PAGE_WIDTH {
                names.push('\n');
                col = 0;
            } else if col > 0 {
                names.push(' ');
                col += 1;
            }
            names.push_str(&word);
            col += word.len();
        }
        let stem = format!("{SOURCE_PREFIX}{kind}-{name}");
        assert!(
            name.bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'),
            "a source's name is lower case, digits and `-`: {name}"
        );
        // The picture's description: the title, as plain words.
        let alt = title.replace('`', "");
        write!(
            out,
            "### {section}.{} {title}\n\nFrom {}, as EBNF in\n\
             [`{stem}.puml`](diagrams/{stem}.puml):\n{names}\n\n\
             ![{alt}, as a railroad diagram](diagrams/{stem}.svg)\n\n",
            g + 1,
            file_link(file),
        )
        .unwrap();
        sources.push((format!("{stem}.puml"), diagram(&lines)));
    }
    (out, sources)
}

/// `ein.l` as the page shows it: every rule as EBNF, read through `lrlex` and
/// [`regex_ebnf`], in [`LEXICAL_GROUPS`]' diagrams, and after them a table of
/// what EBNF does not state: the start conditions each rule is tried in, the
/// one it enters, and the name a parse error gives its token (`ein.y`'s
/// `%epp`). With it, the diagrams' sources.
fn lexical_region() -> (String, Vec<Source>) {
    let lexed = lexed();
    let rules: Vec<(String, Ebnf)> = lexed
        .iter()
        .map(|l| (l.name.clone(), l.ebnf.clone()))
        .collect();
    let (mut out, sources) = grouped(1, "ein.l", "lexical", LEXICAL_GROUPS, &rules);
    let code = |v: &[String]| -> String {
        v.iter()
            .map(|s| format!("`{s}`"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let src = source("ein.l");
    let exclusive = exclusive_states(&src);
    writeln!(
        out,
        "### 1.{} The start conditions, and a token's name in an error\n\n\
         What the diagrams cannot state, from {}.\n\
         The exclusive start conditions are {}: a rule that names no condition is not tried in \
         one.\n\n\
         | rule | tried in | then | a parse error calls its token |\n|---|---|---|---|",
        LEXICAL_GROUPS.len() + 1,
        file_link("ein.l"),
        code(&exclusive),
    )
    .unwrap();
    for l in &lexed {
        let error = match (&l.error, l.name.as_str()) {
            (Some(e), _) => format!("`{e}`"),
            (None, "skipped") => "it makes no token".to_string(),
            (None, _) => "by its text".to_string(),
        };
        writeln!(
            out,
            "| `{}` | {} | {} | {error} |",
            l.name,
            code(&l.tried),
            l.then
                .as_ref()
                .map_or_else(|| "the same".to_string(), |s| format!("`{s}`")),
        )
        .unwrap();
    }
    (out, sources)
}

/// A rule of `ein.y` as EBNF. A token named for its text is that terminal,
/// a class of tokens its name, a rule its name; the alternatives are the
/// rule's productions. A rule that is a list, written left-recursive,
/// `R: B | R X`, is the repetition it derives: `{ X }` where `B` is empty,
/// `{ X }-` where `B` is `X`, and `B , { X }` otherwise. Any other recursion
/// is written as it stands, the rule naming itself.
fn phrase_rule(grm: &YaccGrammar<u32>, r: cfgrammar::RIdx<u32>) -> Ebnf {
    /// A production: its symbols.
    type Prod<'a> = &'a [Symbol<u32>];
    let sym = |s: &Symbol<u32>| -> Ebnf {
        match s {
            Symbol::Rule(r) => Ebnf::Ref(grm.rule_name_str(*r).to_string()),
            Symbol::Token(t) => {
                let name = grm.token_name(*t).expect("a named token");
                if name.chars().all(|c| c.is_ascii_uppercase()) {
                    Ebnf::Ref(name.to_string())
                } else {
                    Ebnf::Lit(name.to_string())
                }
            }
        }
    };
    let me = Symbol::Rule(r);
    let prods: Vec<Prod> = grm.rule_to_prods(r).iter().map(|&p| grm.prod(p)).collect();
    let one = |p: Prod| Ebnf::seq(p.iter().map(&sym).collect());
    // The alternatives of `ps`, an empty one making the others optional.
    let any = |ps: &[Prod]| -> Option<Ebnf> {
        let full: Vec<Ebnf> = ps
            .iter()
            .filter(|p| !p.is_empty())
            .map(|p| one(p))
            .collect();
        match (full.is_empty(), full.len() < ps.len()) {
            (true, _) => None,
            (false, false) => Some(Ebnf::alt(full)),
            (false, true) => Some(Ebnf::Opt(Box::new(Ebnf::alt(full)))),
        }
    };
    let (again, base): (Vec<Prod>, Vec<Prod>) =
        prods.iter().copied().partition(|p| p.first() == Some(&me));
    let list = !again.is_empty()
        && again.iter().all(|p| p.len() > 1 && !p[1..].contains(&me))
        && base.iter().all(|p| !p.contains(&me));
    if !list {
        return any(&prods)
            .unwrap_or_else(|| panic!("ein.y's rule {} derives nothing", grm.rule_name_str(r)));
    }
    let tails: Vec<Prod> = again.iter().map(|p| &p[1..]).collect();
    let item = any(&tails).expect("a list's item is not empty");
    match any(&base) {
        None => Ebnf::Rep0(Box::new(item)),
        Some(b) if b == item => Ebnf::Rep1(Box::new(item)),
        Some(b) => Ebnf::seq(vec![b, Ebnf::Rep0(Box::new(item))]),
    }
}

/// `ein.y` as the page shows it: every rule as EBNF, read through `cfgrammar`
/// and [`phrase_rule`], in [`PHRASE_GROUPS`]' diagrams. With it, the
/// diagrams' sources.
fn phrase_region() -> (String, Vec<Source>) {
    let grm = grammar();
    let start = grm.start_rule_idx();
    let rules: Vec<(String, Ebnf)> = grm
        .iter_rules()
        .filter(|&r| r != start && grm.implicit_rule() != Some(r))
        .map(|r| (grm.rule_name_str(r).to_string(), phrase_rule(&grm, r)))
        .collect();
    grouped(2, "ein.y", "phrase", PHRASE_GROUPS, &rules)
}

/// The page with each marked region holding `regions`' text.
fn page_with(page: &str, regions: &[(&str, String)]) -> String {
    let mut out = page.to_string();
    for (name, body) in regions {
        let open = format!("<!-- generated: {name} -->\n");
        let close = format!("<!-- /generated: {name} -->");
        let a = out
            .find(&open)
            .unwrap_or_else(|| panic!("{PAGE} has no {open:?} marker"))
            + open.len();
        let b = a + out[a..]
            .find(&close)
            .unwrap_or_else(|| panic!("{PAGE} has no {close:?} marker"));
        out.replace_range(a..b, body);
    }
    out
}

/// The sources of the grammar's diagrams that are on disk: the names in
/// [`DIAGRAMS`] that start with [`SOURCE_PREFIX`] and end in `.puml`.
fn sources_on_disk() -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(repo_root().join(DIAGRAMS))
        .into_iter()
        .flatten()
        .map(|entry| {
            entry
                .expect("a directory entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|n| n.starts_with(SOURCE_PREFIX) && n.ends_with(".puml"))
        .collect();
    names.sort();
    names
}

/// **`01-grammar.md`'s diagrams are the grammar's.** Each region, and the
/// source of each diagram it shows, is generated from the file `build.rs`
/// compiles. A region or a source that differs fails, and so does a source
/// of the grammar's that no group writes; `EIN_BLESS=1` writes the regions
/// and the sources, and removes such a source with its picture. A picture is
/// drawn from its source and is not this test's.
#[test]
fn the_pages_grammar_blocks_are_the_parsers() {
    let path = repo_root().join(PAGE);
    let dir = repo_root().join(DIAGRAMS);
    let page = std::fs::read_to_string(&path).expect("the page reads");
    let (lexical, mut sources) = lexical_region();
    let (phrase, phrase_sources) = phrase_region();
    sources.extend(phrase_sources);
    let want = page_with(&page, &[("ein.l", lexical), ("ein.y", phrase)]);
    let bless = std::env::var_os("EIN_BLESS").is_some();

    let mut wrong = Vec::new();
    if want != page {
        if bless {
            std::fs::write(&path, &want).expect("the page is written");
        } else {
            let first = page
                .lines()
                .zip(want.lines())
                .position(|(a, b)| a != b)
                .map_or(0, |i| i + 1);
            wrong.push(format!(
                "{PAGE}'s regions are not ein.l and ein.y as the parser reads them (first \
                 difference at line {first})"
            ));
        }
    }
    for (name, text) in &sources {
        let file = dir.join(name);
        if std::fs::read_to_string(&file).is_ok_and(|on_disk| &on_disk == text) {
            continue;
        }
        if bless {
            std::fs::create_dir_all(&dir).expect("the diagrams' folder");
            std::fs::write(&file, text).expect("a source is written");
        } else {
            wrong.push(format!(
                "{DIAGRAMS}/{name} is not its rules as the parser reads them"
            ));
        }
    }
    for name in sources_on_disk() {
        if sources.iter().any(|(n, _)| n == &name) {
            continue;
        }
        if bless {
            let file = dir.join(&name);
            std::fs::remove_file(&file).expect("a stale source is removed");
            // Its picture, where one was drawn.
            let _ = std::fs::remove_file(file.with_extension("svg"));
        } else {
            wrong.push(format!(
                "{DIAGRAMS}/{name} is the source of no diagram of the page"
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "the grammar's diagrams are not the grammar files':\n  {}\nEIN_BLESS=1 rewrites the \
         regions and the sources; a picture is then drawn again from its source",
        wrong.join("\n  ")
    );
}

/// The translation's own shapes, on rules written here: what a list, a
/// class, a counted repetition and a rule too long for a line become.
#[test]
fn the_ebnf_translation_reads_each_shape() {
    let re = |s: &str| -> String {
        let ast = ParserBuilder::new()
            .octal(true)
            .build()
            .parse(s)
            .expect("the rule reads");
        regex_ebnf(&ast).flat()
    };
    for (rule, want) in [
        (r"not", r#""not""#),
        (r":goal\-text", r#"":goal-text""#),
        (r"\?_", r#""?_""#),
        (r"[<](a|b)[>]", r#""<" , ( "a" | "b" ) , ">""#),
        (r"[A-Za-z0-9_]+", "{ ? one of: A-Z a-z 0-9 _ ? }-"),
        (
            r#""([^"\\]|\\[nt"])*""#,
            r#"'"' , { ? any character but: " \ ? | "\" , ( "n" | "t" | '"' ) } , '"'"#,
        ),
        (
            r"0{0,5}[0-9A-Fa-f]",
            r#"5 * [ "0" ] , ? one of: 0-9 A-F a-f ?"#,
        ),
        (
            r"0?[1-9][0-9]{4}",
            r#"[ "0" ] , ? one of: 1-9 ? , 4 * ? one of: 0-9 ?"#,
        ),
        (r"a{2,}", r#"2 * "a" , { "a" }"#),
        (
            r"[\x00-\x7F--[\s();\x1c-\x1f]]",
            "? one of: U+0000-U+007F, but none of: whitespace ( ) ; U+001C-U+001F ?",
        ),
        (r";[^\n]*", r#"";" , { ? any character but: U+000A ? }"#),
        (r"[?]x", r#""?x""#),
    ] {
        assert_eq!(re(rule), want, "the lexer rule `{rule}`");
    }
    let long = Ebnf::Alt(
        (0..12)
            .map(|i| {
                Ebnf::Seq(vec![
                    Ebnf::Lit("(".into()),
                    Ebnf::Ref(format!("Alternative{i}")),
                    Ebnf::Lit(")".into()),
                ])
            })
            .collect(),
    );
    // A terminal quoted with `'` never starts a line.
    let quoted = Ebnf::Seq(vec![
        Ebnf::Lit("\"".into()),
        Ebnf::Rep0(Box::new(long.clone())),
        Ebnf::Lit("\"".into()),
    ])
    .rule("Quoted");
    assert!(quoted.starts_with("Quoted = '\"' ,\n  {\n"), "{quoted}");
    assert!(quoted.ends_with("\n  } , '\"' ;"), "{quoted}");
    let text = long.rule("Long");
    assert!(text.starts_with("Long =\n    \"(\" , Alternative0 , \")\"\n  | \"(\" , Alternative1"));
    assert!(text.lines().all(|l| l.chars().count() <= EBNF_WIDTH));
}

/// Every token `ein.y` names is a rule of `ein.l`, so the lexical block lists
/// every token the phrase block writes.
#[test]
fn every_token_the_phrase_block_writes_the_lexical_block_defines() {
    type Def = LRNonStreamingLexerDef<DefaultLexerTypes<u32>>;
    let def = Def::from_str(&source("ein.l")).expect("ein.l reads");
    let lexed: Vec<&str> = def.iter_rules().filter_map(|r| r.name()).collect();
    let grm = grammar();
    for t in grm.iter_tidxs() {
        if let Some(name) = grm.token_name(t) {
            assert!(
                lexed.contains(&name),
                "ein.y's token {name:?} has no rule in ein.l"
            );
        }
    }
}

// ── The grammar's table ────────────────────────────────────────────

/// A refusal line of the loader, the compiler or the firing that has no code
/// left, because the grammar refuses its shape at the parse: its id, the file
/// it was in, and its sentence as the program read it. The last five are the
/// verdict's placements, `validate_open`'s, which only a pattern's argument
/// and a macro's body could reach while `open` was a name; `open` is no name
/// (Q-M1f.66).
const DELETED: &[(&str, &str, &str)] = &[
    (
        "top-level-form",
        "from_ir.rs",
        "unexpected top-level form: …",
    ),
    (
        "top-level-claim",
        "from_ir.rs",
        "a top-level (region …) — a region claim belongs under (solve … :expect …)",
    ),
    (
        "solve-keyword",
        "from_ir.rs",
        "(solve …): unknown keyword :k — one of :hrules :hypothesis-relations :no-hypothesis :expect",
    ),
    (
        "query-lattice-keyword",
        "from_ir.rs",
        "(query …): :hrules chooses the lattice and belongs on (solve …) — one program, one lattice",
    ),
    (
        "query-keyword",
        "from_ir.rs",
        "(query …): unknown keyword :k — one of :goal :goal-text :over :expect :mode (QUERY_KEYWORDS)",
    ),
    (
        "relation-name",
        "from_ir.rs",
        "(relation) needs a name at …",
    ),
    (
        "relation-malformed",
        "from_ir.rs",
        "malformed (relation) at …",
    ),
    (
        "rule-name-params",
        "from_ir.rs",
        "(rule) needs name + params at …",
    ),
    ("rule-malformed", "from_ir.rs", "malformed (rule …) at …"),
    (
        "wild-head",
        "from_ir.rs",
        "Found::WildHead: … writes (_ a) with the wildcard _ as its relation",
    ),
    (
        "nested-kw",
        "from_ir.rs",
        "Found::NestedKw: … writes :k v as an argument of p, and no fact can hold it",
    ),
    (
        "wild-guard",
        "from_ir.rs",
        "Found::WildGuard: … writes (eq ?x _), and _ names no value",
    ),
    (
        "guard-kwpair",
        "from_ir.rs",
        "Found::GuardNode: … writes (eq ?x :k a), and the keyword pair :k a is no value",
    ),
    ("fact-head", "from_ir.rs", "fact with non-atom head at …"),
    (
        "config-kw-pairs",
        "from_ir.rs",
        "(config …): (config …) body expects kw_pairs, got …",
    ),
    (
        "config-retired",
        "from_ir.rs",
        "(config …): config flag :k was retired …: … (a note on the parse error now)",
    ),
    (
        "config-unknown",
        "from_ir.rs",
        "(config …): unknown config flag :k (expected one of: …)",
    ),
    (
        "config-bool",
        "from_ir.rs",
        "(config …): config flag :k expects true/false, got …",
    ),
    (
        "expect-atom-false",
        "expect.rs",
        ":expect false — ⊥ is spelled `(false)`, as it is in every `:assert`",
    ),
    (
        "expect-atom",
        "expect.rs",
        ":expect all — expected `(false)`, `(model …)` or `(or (model …) …)`",
    ),
    (
        "expect-not-a-form",
        "expect.rs",
        ":expect — expected `(false)`, `(model …)` or `(or (model …) …)`, at a value no form",
    ),
    (
        "expect-head",
        "expect.rs",
        ":expect — expected …, at a form whose head is no name",
    ),
    (
        "expect-false-args",
        "expect.rs",
        ":expect (false …) — ⊥ takes no arguments",
    ),
    (
        "expect-or-empty",
        "expect.rs",
        ":expect (or) — an empty disjunction expects nothing",
    ),
    (
        "expect-or-item-not-a-form",
        "expect.rs",
        ":expect (or …) — every disjunct must be a `(model …)`, at a disjunct no form",
    ),
    (
        "expect-or-item-not-model",
        "expect.rs",
        ":expect (or …) — every disjunct must be a `(model …)`",
    ),
    (
        "expect-and",
        "expect.rs",
        ":expect (and …) — a model is `(model …)`; `and` is a connective …",
    ),
    (
        "expect-head-other",
        "expect.rs",
        ":expect (models …) — expected `(false)`, `(model …)` or `(or (model …) …)`",
    ),
    (
        "expect-not-arity",
        "expect.rs",
        ":expect — `(not …)` takes exactly one fact",
    ),
    (
        "expect-fact-not-a-form",
        "expect.rs",
        ":expect — `3` is not a fact",
    ),
    (
        "expect-fact-head",
        "expect.rs",
        ":expect — `(?p A)` has no relation name in head position",
    ),
    (
        "expect-var",
        "expect.rs",
        ":expect — `?x` is a variable; an expectation is an answer, not a pattern",
    ),
    (
        "expect-wildcard",
        "expect.rs",
        ":expect — `_` is a wildcard; an expectation is an answer, not a pattern",
    ),
    (
        "expect-nested-head",
        "expect.rs",
        ":expect — a nested fact needs a relation name in head position",
    ),
    (
        "expect-arg-other",
        "expect.rs",
        ":expect — `:k v` is not a fact argument",
    ),
    (
        "over-not-a-name",
        "expect.rs",
        "(query …): :over (all) — one of all, minimal, maximal, at a value no name",
    ),
    (
        "claim-not-a-form",
        "claim.rs",
        "(solve … :expect …) — expected `(false)` or `(region …)`",
    ),
    (
        "claim-false-args",
        "claim.rs",
        "(solve … :expect (false …)) — ⊥ takes no arguments",
    ),
    (
        "claim-model-or-or",
        "claim.rs",
        "(solve … :expect (model …)) — a region is stated as `(region (model …) …)`; `(model …)` is a query's answer",
    ),
    (
        "claim-head-other",
        "claim.rs",
        "(solve … :expect (models …)) — (solve … :expect …) — expected `(false)` or `(region …)`",
    ),
    (
        "merged-arity",
        "claim.rs",
        "(merged …) — expected `(merged (model …) <from>)`, a state and its minimal commitments",
    ),
    (
        "merged-to",
        "claim.rs",
        "(merged …) — its first part is a `(model …)`, one of the states the region lists",
    ),
    (
        "merged-or-item",
        "claim.rs",
        "(merged …) — every commitment in its `(or …)` is a `(model …)`",
    ),
    (
        "merged-from",
        "claim.rs",
        "(merged …) — its second part is a `(model …)`, or an `(or (model …) …)` of them",
    ),
    (
        "owing-empty",
        "claim.rs",
        "(owing) — it lists the alive nodes that owe; an empty R with nothing listed is `(region)`",
    ),
    (
        "owing-item",
        "claim.rs",
        "(owing …) — every item is a `(model …)`",
    ),
    (
        "region-item",
        "claim.rs",
        "(region …) — `(p A)` is not `(model …)`, `(merged …)` or `(owing …)`",
    ),
    (
        "import-module",
        "imports.rs",
        "malformed (import …) — missing module name at …",
    ),
    (
        "import-as",
        "imports.rs",
        "(import m :as …) — alias must be a bare name at …",
    ),
    (
        "import-symbols-list",
        "imports.rs",
        "(import m :symbols …) — expected a (name …) list at …",
    ),
    (
        "import-symbols-empty",
        "imports.rs",
        "(import m :symbols ()) — empty list at …",
    ),
    (
        "macro-arity",
        "macros.rs",
        "(macro) needs name + params + body at …",
    ),
    ("macro-malformed", "macros.rs", "malformed (macro …) at …"),
    (
        "conclusion-argument",
        "ein-infer firing.rs",
        "rule 'r': its :assert writes _ as an argument of p, and no fact can hold it — …",
    ),
    (
        "absent-empty",
        "ein-infer compile.rs",
        "`(absent …)` sub-plan compiled to no steps in … — the guard would fail against every KB …",
    ),
    (
        "open-in-match",
        "from_ir.rs",
        "rule 'r': `(open …)` is a verdict about the KB and is legal only in :assert — the third-state probe for a fact is `(unknown …)` at None",
    ),
    (
        "open-arity",
        "from_ir.rs",
        "rule 'r': `open` takes the incomplete relation and nothing else — `(open)` or `(open ?R)`, not 2 arguments at None",
    ),
    (
        "open-beside",
        "from_ir.rs",
        "rule 'r': a rule asserting `open` may assert nothing else — it is read after the fixpoint, where a derivation would be too late at None",
    ),
    (
        "open-inside",
        "from_ir.rs",
        "rule 'r': `(open …)` is a whole conclusion, not a term inside one at None",
    ),
    (
        "open-argument",
        "from_ir.rs",
        "rule 'r': `open`'s argument names a relation — a rule parameter or a relation name at None",
    ),
];

/// One refusal: the place in the grammar, the [`DELETED`] lines that refused
/// the shape before the grammar did (empty where none did: the shape loaded,
/// or was dropped, and the row is a decision's), and the program.
struct Row {
    place: &'static str,
    was: &'static [&'static str],
    program: &'static str,
}

const fn row(place: &'static str, was: &'static [&'static str], program: &'static str) -> Row {
    Row {
        place,
        was,
        program,
    }
}

/// Declarations every rule row's program reads, so that a row's only error is
/// its own.
const RULES: &str = "(relation n T) (relation p T) (relation q T) (relation hit T)\n\
                     (relation box T) (relation tag T T)\n";

/// **The grammar's table.** Its first rows are the tests the [`DELETED`]
/// lines had:
/// `from_ir`'s unit tests of the wildcard relation, the shapes that read
/// nothing, the verdict atom's placements, an unknown query keyword, `:mode`, a
/// malformed `:expect` and the accumulating passes; `expect`'s of a claim's
/// shape; `ein-infer`'s `claim_semantics`, `compile_semantics` (an empty
/// `absent`, the config refusals, the wrapped declaration) and `naf_semantics`
/// (the retired flag). The outcome of each is banked in
/// `tests/golden/grammar_table.txt`.
const TABLE: &[Row] = &[
    // ── The top level ──
    row(
        "a top-level form's head",
        &["top-level-form", "fact-head"],
        "((p) a)",
    ),
    row(
        "a top-level form's head",
        &["top-level-form", "fact-head"],
        "(?x a)",
    ),
    row(
        "a top-level form's head",
        &["top-level-form", "fact-head"],
        "(\"s\" a)",
    ),
    row(
        "a top-level fact's relation",
        &["top-level-claim"],
        "(relation p T) (p A) (region (model (p A)))",
    ),
    row(
        "a top-level fact's relation",
        &["top-level-claim"],
        "(relation p T) (p A) (owing (model))",
    ),
    row(
        "a top-level fact's relation",
        &["top-level-claim"],
        "(merged (model) (model))",
    ),
    // ── Declarations ──
    row("a relation's name", &["relation-name"], "(relation)"),
    row(
        "a relation's name",
        &["relation-name"],
        "(relation)\n(relation dup)\n(relation dup)\n(rule absent () :match (x ?a) :assert (y ?a))",
    ),
    row(
        "a relation's signature",
        &["relation-malformed"],
        "(relation R (T1 T2))",
    ),
    row(
        "a relation's signature",
        &["relation-malformed"],
        "(relation r ?A)",
    ),
    row(
        "a relation's name",
        &["relation-malformed"],
        "(relation (r) A)",
    ),
    row("a relation's name", &[], "(relation open A B)"),
    row(
        "a rule's name and parameters",
        &["rule-name-params"],
        "(rule x)",
    ),
    row(
        "a rule's name and parameters",
        &["rule-name-params"],
        "(hrule x)",
    ),
    row(
        "a rule's name and parameters",
        &["rule-name-params"],
        "(rule)",
    ),
    row(
        "a rule's name",
        &["rule-malformed"],
        "(rule (x) () :match (a) :assert (b))",
    ),
    row(
        "a rule's parameters",
        &["rule-malformed"],
        "(rule x y :match (a) :assert (b))",
    ),
    row(
        "a rule's parameters",
        &["rule-malformed"],
        "(rule x :match (a) :assert (b))",
    ),
    row(
        "a rule's name",
        &[],
        "(relation r A B)\n(rule open () :match (r ?a ?b) :assert (r ?b ?a))",
    ),
    row(
        "a macro's parameters and body",
        &["macro-arity"],
        "(macro foo (?p))",
    ),
    row("a macro's parameters and body", &["macro-arity"], "(macro)"),
    row("a macro's name", &["macro-malformed"], "(macro ?x ?y)"),
    row(
        "a macro's parameters",
        &["macro-malformed"],
        "(macro foo () (rel ?a))",
    ),
    row(
        "a macro's name",
        &["macro-malformed"],
        "(macro (f) (?p) (r ?p))",
    ),
    row("a macro's name", &[], "(macro open (?P) (absent ?P))"),
    // ── Imports ──
    row("an import's module", &["import-module"], "(import)"),
    row(
        "an import's module",
        &["import-module"],
        "(import (std) :as m)",
    ),
    row(
        "an import's alias",
        &["import-as"],
        "(import std.macro :as \"m\")",
    ),
    row(
        "an import's alias",
        &["import-as"],
        "(import std.macro :as (m))",
    ),
    row(
        "an import's symbols",
        &["import-symbols-list"],
        "(import std.macro :symbols forall)",
    ),
    row(
        "an import's symbols",
        &["import-symbols-empty"],
        "(import std.macro :symbols ())",
    ),
    // ── A rule's :match ──
    row(
        "a premise's relation",
        &["wild-head"],
        "(rule w () :match (_ 2) :assert (hit w))",
    ),
    row(
        "a pattern's relation",
        &["wild-head"],
        "(rule w () :match (box (_ 2)) :assert (hit w))",
    ),
    row(
        "a pattern's relation",
        &["wild-head"],
        "(rule w () :match (absent (not (_ b))) :assert (hit w))",
    ),
    row(
        "a pattern's relation, in a guard",
        &["wild-head"],
        "(rule w () :match (and (n ?x) (eq ?x (_ 2))) :assert (hit w))",
    ),
    row(
        "a conclusion's relation",
        &["wild-head"],
        "(rule w () :match (n ?x) :assert (box (_ ?x)))",
    ),
    row(
        "a premise's relation, in an hrule",
        &["wild-head"],
        "(hrule h () :match (_ ?x) :assert (tag ?x ?x))",
    ),
    row(
        "a premise's relation and a conclusion's",
        &["wild-head"],
        "(rule r () :match (and (n ?x) (_ 2)) :assert (box (_ ?x)))",
    ),
    row(
        "a pattern's argument",
        &[],
        "(rule r () :match (and (n _) (box (tag _ _))) :assert (hit r))",
    ),
    row(
        "a pattern's argument, under an absent",
        &[],
        "(rule owe () :match (and (n ?x) (absent (tag ?x _))) :assert (open tag))",
    ),
    row(
        "a pattern's argument",
        &["nested-kw"],
        "(rule r () :match (box (p a :k v)) :assert (hit a))",
    ),
    row(
        "a premise's argument",
        &[],
        "(rule r () :match (n ?x :where v) :assert (hit r))",
    ),
    row(
        "a guard's argument",
        &["wild-guard"],
        "(rule e () :match (and (p ?x) (eq ?x _)) :assert (hit ?x))",
    ),
    row(
        "a guard's argument, under an absent",
        &["wild-guard"],
        "(rule e () :match (and (p ?x) (absent (and (q ?y) (neq ?y _)))) :assert (hit ?x))",
    ),
    row(
        "a guard's argument, in an hrule",
        &["wild-guard"],
        "(hrule h () :match (and (p ?x) (eq ?x _)) :assert (hit ?x))",
    ),
    row(
        "a guard's argument",
        &["guard-kwpair"],
        "(rule e () :match (and (p ?x) (eq ?x :k a)) :assert (hit ?x))",
    ),
    row("a :match", &[], "(rule r () :match _ :assert (hit a))"),
    row("a :match", &[], "(rule r () :match foo :assert (hit a))"),
    row(
        "a conjunct",
        &[],
        "(rule r () :match (and (p ?x) ?x) :assert (hit ?x))",
    ),
    row(
        "a disjunct",
        &[],
        "(rule r () :match (or 3 (q ?y)) :assert (hit a))",
    ),
    row(
        "an absent's premise",
        &[],
        "(rule r () :match (and (p ?x) (absent \"s\")) :assert (hit ?x))",
    ),
    row(
        "a conjunct, in an hrule",
        &[],
        "(hrule h () :match (and (p ?x) _) :assert (hit ?x))",
    ),
    row(
        "an absent's premise",
        &["absent-empty"],
        "(rule vacuous ()\n  :match (and (p ?x) (absent :k v))\n  :assert (hit ?x))",
    ),
    row(
        "an absent's premise",
        &["absent-empty"],
        "(rule vacuous ()\n  :match (and (p ?x) (absent ?x))\n  :assert (hit ?x))",
    ),
    row(
        "a premise's relation",
        &[],
        "(relation is-a T T) (relation r A B)\n(rule bad (?R) :match (and (relation ?R ?A ?B) (open ?R)) :assert (r a b))",
    ),
    // ── A rule's :assert ──
    row(
        "a conclusion's argument",
        &["nested-kw", "conclusion-argument"],
        "(rule r () :match (p ?x) :assert (box (p ?x :k v)))",
    ),
    row(
        "a conclusion's argument",
        &["conclusion-argument"],
        "(rule s () :match (p ?x) :assert (tag ?x _))",
    ),
    row(
        "a conclusion's argument",
        &["conclusion-argument"],
        "(rule s () :match (p ?x) :assert (tag ?x \"lbl\"))",
    ),
    row(
        "a conclusion's keyword",
        &[],
        "(rule s () :match (p ?x) :assert (tag ?x b :source \"x\"))",
    ),
    row("an :assert", &[], "(rule s () :match (p ?x) :assert foo)"),
    row(
        "a conclusion",
        &[],
        "(rule s () :match (p ?x) :assert (and (tag ?x b) ?x))",
    ),
    row(
        "an (open …)'s arguments",
        &[],
        "(relation is-a T T) (relation r A B)\n(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) (absent (and (?isa ?b ?B) (?R ?a ?b)))) :assert (open ?b (?isa ?b ?B) (?R ?a ?b)))",
    ),
    row(
        "a conclusion beside (open …)",
        &[],
        "(relation is-a T T) (relation r A B)\n(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) (absent (and (?isa ?b ?B) (?R ?a ?b)))) :assert (and (open ?R) (r ?a b)))",
    ),
    row(
        "an (open …)'s argument",
        &[],
        "(relation is-a T T) (relation r A B)\n(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) (absent (and (?isa ?b ?B) (?R ?a ?b)))) :assert (open (r ?a ?b)))",
    ),
    // ── A (query …) ──
    row(
        "a query's keyword",
        &["query-keyword"],
        "(relation p T P)\n(p A H)\n(query :goal (p A ?h) :expct none)",
    ),
    row(
        "a query's keyword",
        &["query-keyword"],
        "(query :goal (a ?x) :mode all)",
    ),
    row(
        "a query's keyword",
        &["query-keyword"],
        "(query :goal (a ?x))\n(query :goal (b ?x) :mode all)\n(config :print-alive true)",
    ),
    row(
        "a query's keyword",
        &["query-lattice-keyword"],
        "(query :goal (a ?x) :hrules (h))",
    ),
    row(
        "a query's keyword",
        &["query-lattice-keyword"],
        "(query :goal (a ?x) :no-hypothesis (p))",
    ),
    row(
        "a query's :over",
        &["over-not-a-name"],
        "(query :goal (a ?x) :over (all))",
    ),
    row(
        "a query's :over",
        &["over-not-a-name"],
        "(query :goal (a ?x) :over ?x)",
    ),
    row(
        "a query's claim",
        &["expect-atom"],
        "(query :goal (p ?x) :expect all)",
    ),
    row(
        "a query's claim",
        &["expect-atom"],
        "(query :goal (p ?x) :expect none)",
    ),
    row(
        "a query's claim",
        &["expect-atom-false"],
        "(query :goal (p ?x) :expect false)",
    ),
    row(
        "a query's claim",
        &["expect-not-a-form"],
        "(query :goal (p ?x) :expect ?x)",
    ),
    row(
        "a query's claim",
        &["expect-not-a-form"],
        "(query :goal (p ?x) :expect \"s\")",
    ),
    row(
        "a query's claim",
        &["expect-head"],
        "(query :goal (p ?x) :expect (?m (p A)))",
    ),
    row(
        "a query's claim",
        &["expect-false-args"],
        "(query :goal (p ?x) :expect (false A))",
    ),
    row(
        "a query's claim",
        &["expect-and"],
        "(query :goal (p ?x) :expect (and (p A) (p B)))",
    ),
    row(
        "a query's claim",
        &["expect-and"],
        "(query :goal (p ?x) :expect (and (p A)))",
    ),
    row(
        "a query's claim",
        &["expect-head-other"],
        "(query :goal (p ?x) :expect (models (p A)))",
    ),
    row(
        "a query's claim",
        &["expect-or-empty"],
        "(query :goal (p ?x) :expect (or))",
    ),
    row(
        "a query's claim",
        &["expect-or-item-not-model"],
        "(query :goal (p ?x) :expect (or (p A)))",
    ),
    row(
        "a query's claim",
        &["expect-or-item-not-a-form"],
        "(query :goal (p ?x) :expect (or a))",
    ),
    row(
        "a claim's fact",
        &["expect-not-arity"],
        "(query :goal (p ?x) :expect (model (not (p A) (p B))))",
    ),
    row(
        "a claim's fact",
        &["expect-fact-not-a-form"],
        "(query :goal (p ?x) :expect (model 3))",
    ),
    row(
        "a claim's fact",
        &["expect-fact-head"],
        "(query :goal (p ?x) :expect (model (?p A)))",
    ),
    row(
        "a claim's fact's argument",
        &["expect-var"],
        "(query :goal (p ?x) :expect (model (p ?x)))",
    ),
    row(
        "a claim's fact's argument",
        &["expect-wildcard"],
        "(query :goal (p ?x) :expect (model (p _)))",
    ),
    row(
        "a claim's fact's argument",
        &["expect-nested-head"],
        "(query :goal (p ?x) :expect (model (p (?q A))))",
    ),
    row(
        "a claim's fact's argument",
        &["expect-arg-other"],
        "(query :goal (p ?x) :expect (model (p :k v)))",
    ),
    row(
        "a claim's fact's argument",
        &["expect-arg-other"],
        "(query :goal (p ?x) :expect (model (p \"s\")))",
    ),
    // ── A (solve …) ──
    row(
        "a solve's keyword",
        &["solve-keyword"],
        "(solve :hrule (h))",
    ),
    row(
        "a solve's keyword",
        &["solve-keyword"],
        "(solve :goal (p ?x))",
    ),
    row(
        "a solve's claim",
        &["claim-not-a-form"],
        "(solve :expect region)",
    ),
    row(
        "a solve's claim",
        &["claim-false-args"],
        "(solve :expect (false A))",
    ),
    row(
        "a solve's claim",
        &["claim-model-or-or"],
        "(relation p T) (p A) (solve :expect (model (p A)))",
    ),
    row(
        "a solve's claim",
        &["claim-model-or-or"],
        "(solve :expect (or (model) (model)))",
    ),
    row(
        "a solve's claim",
        &["claim-head-other"],
        "(solve :expect (models))",
    ),
    row(
        "a region's item",
        &["region-item"],
        "(relation p T) (p A) (solve :expect (region (p A)))",
    ),
    row(
        "a region's item",
        &["merged-arity"],
        "(relation p T) (p A) (solve :expect (region (merged (model (p A)))))",
    ),
    row(
        "a region's item",
        &["merged-to"],
        "(solve :expect (region (model) (merged (p A) (model))))",
    ),
    row(
        "a region's item",
        &["merged-from"],
        "(solve :expect (region (model) (merged (model) (p A))))",
    ),
    row(
        "a region's item",
        &["merged-or-item"],
        "(solve :expect (region (model) (merged (model) (or (p A)))))",
    ),
    row(
        "a region's item",
        &["owing-empty"],
        "(relation p T) (p A) (solve :expect (region (owing)))",
    ),
    row(
        "a region's item",
        &["owing-item"],
        "(solve :expect (region (owing (p A))))",
    ),
    row(
        "a claim's fact's argument",
        &[],
        "(relation p T) (p A) (solve :expect (region (model (p ?x))))",
    ),
    // ── A (config …) ──
    row("a config's body", &["config-kw-pairs"], "(config true)"),
    row(
        "a config's body",
        &["config-kw-pairs"],
        "(config lattice-sanity-check true)",
    ),
    row(
        "a config's keyword",
        &["config-unknown"],
        "(config :nope 1)",
    ),
    row(
        "a config's keyword",
        &["config-unknown"],
        "(config :lattice-sanity true)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :lattice-order-seed 7)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :candidate-order-seed 5)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :print-alive true)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :lattice-order lex)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :hypgen-rel-weight 2)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :enable-singleton-writeback false)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :enable-forced-positive false)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :enable-symmetric-mirror false)",
    ),
    row(
        "a config's keyword",
        &["config-retired"],
        "(config :warn-derived-naf true)",
    ),
    row(
        "a config flag's value",
        &["config-bool"],
        "(config :lattice-sanity-check 7)",
    ),
    row(
        "a config flag's value",
        &["config-bool"],
        "(config :record-alternative-justifications \"false\")",
    ),
    row(
        "a config flag's value",
        &["config-bool"],
        "(config :lattice-sanity-check TRUE)",
    ),
    // ── Names: a token that is no name, where a name goes ──
    // The name rule (Q-M1e2.24): a name is a run of ASCII letters, digits,
    // connectors and decorators up to a separator, and `?` and `:` only in
    // front of a variable and a keyword. So a token that runs on past one of
    // those, or holds a character no name has, is no name in any place one
    // goes: `a?x` is not `a` and `?x`, nor `"a"b` a string and a name, and
    // `a:b`, `a,b` and `Жук` are each one token the grammar refuses. A run of
    // digits in each place is a name, which
    // `ein-cli/tests/cli_semantics.rs`' `a_run_of_digits_is_a_name_in_every_place`
    // holds.
    row("a fact's argument", &[], "(relation p T) (p a?x)"),
    row("a fact's argument", &[], "(relation p T) (p Жук)"),
    row("a fact's relation", &[], "(a?x b)"),
    row(
        "a nested fact's argument",
        &[],
        "(relation p T) (p (q a:b))",
    ),
    row("a stored negation's argument", &[], "(not (p a,b))"),
    row("a relation's name", &[], "(relation a?x T)"),
    row("a relation's signature", &[], "(relation p a:b)"),
    row(
        "a rule's name",
        &[],
        "(rule a?x () :match (p ?x) :assert (q ?x))",
    ),
    row(
        "a premise's relation",
        &[],
        "(rule r () :match (a?x ?y) :assert (q ?y))",
    ),
    row(
        "a premise's argument",
        &[],
        "(rule r () :match (p a?x) :assert (q a))",
    ),
    row(
        "a premise's argument",
        &[],
        "(rule r () :match (p ?x?y) :assert (q ?x))",
    ),
    row(
        "a guard's argument",
        &[],
        "(rule r () :match (and (p ?x) (neq ?x a#b)) :assert (q ?x))",
    ),
    row(
        "an absent's premise's argument",
        &[],
        "(rule r () :match (absent (p a|b)) :assert (q a))",
    ),
    row(
        "a conclusion's argument",
        &[],
        "(rule r () :match (p ?x) :assert (q a,b))",
    ),
    row(
        "a claim's fact's argument",
        &[],
        "(relation p T) (p a) (query :goal (p ?x) :expect (model (p \"a\"b)))",
    ),
    row(
        "a solve's relation list",
        &[],
        "(relation p T) (solve :no-hypothesis (a\\b))",
    ),
    row("an import's alias", &[], "(import std.macro :as a?x)"),
    row("a macro's name", &[], "(macro a?x (?a) (p ?a))"),
    // ── Strings: a string is natural-language text, and no value ──
    // A string is written only after `:why`, `:source` or `:goal-text`
    // (Q-M1e2.23), so `(p "1")` is refused, and so is a variable in a fact:
    // neither is read as the name it spells (`(p "a")` is not `(p a)`, nor
    // `?x` in a fact the name `"?x"`). Each is refused at the parse, a string
    // with its note; the rows above that write a string (a form's head, an import's alias, an
    // `absent`'s premise, a conclusion's argument, a conclusion's `:source`,
    // an `:expect`, a claim's fact, a config flag's value) hold it in their
    // places. The three keywords' text loads and reaches every surface that
    // carries it, which `ein-cli/tests/natural_language_text.rs` holds, and
    // every command refuses `(p "1")` with one line and the note, which
    // `ein-cli/tests/cli_semantics.rs`' `a_string_is_natural_language_and_no_value`
    // holds.
    row("a fact's argument", &[], "(relation p T) (p \"1\")"),
    row("a fact's argument", &[], "(relation p T) (p \"a\")"),
    row("a fact's argument", &[], "(relation p T) (p \"Жук\")"),
    row(
        "a nested fact's argument",
        &[],
        "(relation p T) (p (q \"a\"))",
    ),
    row(
        "a stored negation's argument",
        &[],
        "(relation p T) (not (p \"a\"))",
    ),
    row(
        "a fact's argument",
        &[],
        "(relation is-a T T) (is-a \"x\" T)",
    ),
    row(
        "an activator's argument",
        &[],
        "(relation link T) (rule chase (?r) :match (link ?x) :assert (?r ?x)) (chase \"link\")",
    ),
    row("a fact's :rule", &[], "(relation p T) (p a :rule \"r\")"),
    row(
        "a fact's :using",
        &[],
        "(relation p T) (p a :rule r :using ((q \"b\")))",
    ),
    row("a fact's argument", &[], "(relation r T) (r ?x)"),
    row("a nested fact's argument", &[], "(relation p T) (p (q ?x))"),
    row(
        "a stored negation's argument",
        &[],
        "(relation p T) (not (p ?x))",
    ),
    row("a relation's name", &[], "(relation \"p\" T)"),
    row("a relation's signature", &[], "(relation p \"T\")"),
    row(
        "a rule's name",
        &[],
        "(rule \"r\" () :match (p ?x) :assert (q ?x))",
    ),
    row(
        "a rule's parameters",
        &[],
        "(rule r (\"x\") :match (p ?x) :assert (q ?x))",
    ),
    row(
        "a premise's relation",
        &[],
        "(rule r () :match (\"p\" ?x) :assert (q ?x))",
    ),
    row(
        "a premise's argument",
        &[],
        "(rule r () :match (p \"1\") :assert (q a))",
    ),
    row(
        "a premise's argument",
        &[],
        "(rule r () :match (box (p \"a\")) :assert (q a))",
    ),
    row(
        "a guard's argument",
        &[],
        "(rule r () :match (and (p ?x) (eq ?x \"a\")) :assert (q ?x))",
    ),
    row(
        "a guard's argument",
        &[],
        "(rule r () :match (and (p ?x) (neq ?x \"1\")) :assert (q ?x))",
    ),
    row(
        "an absent's premise's argument",
        &[],
        "(rule r () :match (absent (p \"a\")) :assert (q a))",
    ),
    row(
        "a stored negation's premise",
        &[],
        "(rule r () :match (not (p \"a\")) :assert (q a))",
    ),
    row(
        "a conclusion's relation",
        &[],
        "(rule r () :match (p ?x) :assert (\"q\" ?x))",
    ),
    row(
        "a conclusion's argument",
        &[],
        "(rule r () :match (p ?x) :assert (and (q ?x) (not (p \"a\"))))",
    ),
    row(
        "an open conclusion's relation",
        &[],
        "(rule r () :match (p ?x) :assert (open \"q\"))",
    ),
    row(
        "an hrule's premise",
        &[],
        "(hrule g () :match (p \"a\") :assert (q a))",
    ),
    row(
        "an hrule's conclusion",
        &[],
        "(hrule g () :match (p ?x) :assert (q \"a\"))",
    ),
    row(
        "a rule's keyword",
        &[],
        "(rule r () :match (p ?x) :assert (q ?x) :priority \"5\")",
    ),
    row(
        "a query's goal",
        &[],
        "(relation p T) (p a) (query :goal (p \"a\"))",
    ),
    row(
        "a query's :over",
        &[],
        "(relation p T) (p a) (query :goal (p ?x) :over \"all\")",
    ),
    row(
        "a claim's fact's argument",
        &[],
        "(relation p T) (p a) (query :goal (p ?x) :expect (model (p \"1\")))",
    ),
    row(
        "a claim's neq",
        &[],
        "(relation p T) (p a) (query :goal (p ?x) :expect (model (neq \"a\" b)))",
    ),
    row(
        "a region claim's fact",
        &[],
        "(relation p T) (p a) (solve :expect (region (model (p \"a\"))))",
    ),
    row(
        "a solve's relation list",
        &[],
        "(relation p T) (solve :no-hypothesis (\"p\"))",
    ),
    row(
        "a solve's relation list",
        &[],
        "(relation p T) (solve :hypothesis-relations (\"p\"))",
    ),
    row("a solve's hrules", &[], "(solve :hrules (\"g\"))"),
    row("a solve's hrules", &[], "(solve :hrules (g (\"a\")))"),
    row(
        "a config flag's value",
        &[],
        "(config :lattice-sanity-check \"true\")",
    ),
    row("a macro's name", &[], "(macro \"m\" (?a) (p ?a))"),
    row("a macro's parameters", &[], "(macro m (\"a\") (p ?a))"),
    row("a macro's body", &[], "(macro m (?a) (p ?a \"b\"))"),
    row("an import's module", &[], "(import \"std.macro\")"),
    row(
        "an import's symbols",
        &[],
        "(import std.macro :symbols (\"forall\"))",
    ),
    row("a trace step's name", &[], "(trace (step \"s1\" :rule r))"),
    row(
        "a trace step's :derives",
        &[],
        "(trace (step s1 :derives (p \"a\")))",
    ),
    row(
        "a fact's keyword",
        &[],
        "(relation p T) (p a :why \"text\")",
    ),
    row(
        "a relation's keyword",
        &[],
        "(relation p T :source \"text\")",
    ),
    row(
        "a query's keyword",
        &[],
        "(relation p T) (query :goal (p ?x) :why \"text\")",
    ),
    // ── `_`: a bare `_` and `?_` name nothing ──
    // A bare `_` is reserved and refused, and stays an ordinary character
    // inside a name or a variable (Q-M1e2.22); `?_` with it, since two `?_` in
    // one rule would bind one value, the opposite of what the spelling
    // suggests. Each is a token no production takes (`ein.l`'s `UNDERSCORE`
    // and `VARUNDERSCORE`), refused at the parse with its note, in every place
    // a name or a variable goes: neither is dropped from a fact, an activator
    // or a nested fact, nor matches any value in a rule's pattern; the rows
    // above that write `_` as a relation, a guard's argument
    // or a premise are the [`DELETED`] lines'. Every command that reads a program
    // refuses both, which `ein-cli/tests/cli_semantics.rs`'
    // `a_bare_underscore_names_nothing_in_every_place` holds.
    row("a fact's argument", &[], "(relation r T T) (r _ a)"),
    row("a fact's argument", &[], "(relation r T T) (r ?_ a)"),
    row(
        "a nested fact's argument",
        &[],
        "(relation r T T) (r (f _ a) c)",
    ),
    row(
        "a nested fact's argument",
        &[],
        "(relation r T T) (r (f ?_ a) c)",
    ),
    row("a nested fact's relation", &[], "(relation h T) (h (_ b))"),
    row("a nested fact's relation", &[], "(relation h T) (h (?_ b))"),
    row("a fact's argument", &[], "(relation is-a T T) (is-a _ T)"),
    row("a fact's argument", &[], "(relation is-a T T) (is-a ?_ T)"),
    row(
        "a stored negation's argument",
        &[],
        "(relation p T) (not (p _))",
    ),
    row(
        "a stored negation's argument",
        &[],
        "(relation p T) (not (p ?_))",
    ),
    row(
        "an activator's argument",
        &[],
        "(relation p T) (relation hit T) (rule q (?x) :match (p ?x) :assert (hit ?x)) (q _)",
    ),
    row(
        "an activator's argument",
        &[],
        "(relation p T) (relation hit T) (rule q (?x) :match (p ?x) :assert (hit ?x)) (q ?_)",
    ),
    row("a fact's :rule", &[], "(relation p T) (p a :rule _)"),
    row(
        "a fact's :using",
        &[],
        "(relation p T) (p a :rule r :using ((q ?_)))",
    ),
    row("a top-level form's head", &[], "(_ a)"),
    row("a top-level form's head", &[], "(?_ a)"),
    row("a relation's name", &[], "(relation _ T)"),
    row("a relation's name", &[], "(relation ?_ T)"),
    row("a relation's signature", &[], "(relation r _ T)"),
    row("a relation's signature", &[], "(relation r ?_ T)"),
    row(
        "a rule's name",
        &[],
        "(rule _ () :match (p ?x) :assert (q ?x))",
    ),
    row(
        "a rule's name",
        &[],
        "(rule ?_ () :match (p ?x) :assert (q ?x))",
    ),
    row(
        "a rule's parameters",
        &[],
        "(rule r (_) :match (p ?x) :assert (q ?x))",
    ),
    row(
        "a rule's parameters",
        &[],
        "(rule r (?_) :match (p ?x) :assert (q ?x))",
    ),
    row(
        "a premise's argument",
        &[],
        "(rule r () :match (n ?_) :assert (hit r))",
    ),
    row(
        "a pattern's argument",
        &[],
        "(rule r () :match (box (tag ?_ a)) :assert (hit r))",
    ),
    row(
        "a pattern's argument, under an absent",
        &[],
        "(rule r () :match (and (n ?x) (absent (tag ?x ?_))) :assert (hit ?x))",
    ),
    row(
        "a premise's argument, in an hrule",
        &[],
        "(hrule h () :match (n _) :assert (hit a))",
    ),
    row(
        "a premise's argument, in an hrule",
        &[],
        "(hrule h () :match (n ?_) :assert (hit a))",
    ),
    row(
        "a premise's relation",
        &[],
        "(rule w () :match (?_ 2) :assert (hit w))",
    ),
    row(
        "a pattern's relation",
        &[],
        "(rule w () :match (box (?_ 2)) :assert (hit w))",
    ),
    row(
        "a premise's relation, in an hrule",
        &[],
        "(hrule h () :match (?_ ?x) :assert (tag ?x ?x))",
    ),
    row(
        "a guard's argument",
        &[],
        "(rule e () :match (and (p ?x) (eq ?x ?_)) :assert (hit ?x))",
    ),
    row(
        "a guard's argument",
        &[],
        "(rule e () :match (and (p ?x) (neq ?_ ?x)) :assert (hit ?x))",
    ),
    row("a :match", &[], "(rule r () :match ?_ :assert (hit a))"),
    row(
        "a conjunct",
        &[],
        "(rule r () :match (and (p ?x) ?_) :assert (hit ?x))",
    ),
    row(
        "a disjunct",
        &[],
        "(rule r () :match (or _ (q ?y)) :assert (hit a))",
    ),
    row(
        "an absent's premise",
        &[],
        "(rule r () :match (and (p ?x) (absent _)) :assert (hit ?x))",
    ),
    row(
        "a conclusion's relation",
        &[],
        "(rule w () :match (n ?x) :assert (?_ ?x))",
    ),
    row(
        "a conclusion's argument",
        &[],
        "(rule s () :match (p ?x) :assert (tag ?x ?_))",
    ),
    row(
        "a conclusion's argument",
        &[],
        "(rule s () :match (p ?x) :assert (and (hit ?x) (not (tag ?x _))))",
    ),
    row("a conclusion", &[], "(rule s () :match (p ?x) :assert _)"),
    row("a conclusion", &[], "(rule s () :match (p ?x) :assert ?_)"),
    row(
        "an open conclusion's relation",
        &[],
        "(rule r () :match (p ?x) :assert (open _))",
    ),
    row(
        "an hrule's conclusion",
        &[],
        "(hrule g () :match (p ?x) :assert (tag ?x _))",
    ),
    row(
        "an hrule's conclusion",
        &[],
        "(hrule g () :match (p ?x) :assert (tag ?x ?_))",
    ),
    row(
        "a query's goal",
        &[],
        "(relation holds T) (holds a) (query :goal (holds _))",
    ),
    row(
        "a query's goal",
        &[],
        "(relation holds T) (holds a) (query :goal (holds ?_))",
    ),
    row(
        "a query's :over",
        &[],
        "(relation p T) (p a) (query :goal (p ?x) :over _)",
    ),
    row(
        "a claim's fact's argument",
        &[],
        "(relation p T) (p a) (query :goal (p ?x) :expect (model (p ?_)))",
    ),
    row(
        "a claim's neq",
        &[],
        "(relation p T) (p a) (query :goal (p ?x) :expect (model (neq _ b)))",
    ),
    row(
        "a region claim's fact",
        &[],
        "(relation p T) (p a) (solve :expect (region (model (p _))))",
    ),
    row(
        "a region claim's fact",
        &[],
        "(relation p T) (p a) (solve :expect (region (model (p ?_))))",
    ),
    row(
        "a solve's relation list",
        &[],
        "(relation p T) (solve :no-hypothesis (_))",
    ),
    row(
        "a solve's relation list",
        &[],
        "(relation p T) (solve :hypothesis-relations (?_))",
    ),
    row("a solve's hrules", &[], "(solve :hrules (_))"),
    row(
        "a config flag's value",
        &[],
        "(config :lattice-sanity-check _)",
    ),
    row("a macro's name", &[], "(macro _ (?a) (p ?a))"),
    row("a macro's parameters", &[], "(macro m (?_) (p a))"),
    row("a macro's body", &[], "(macro m (?a) (p ?a _))"),
    row("a macro's body", &[], "(macro m (?a) (p ?a ?_))"),
    row("an import's module", &[], "(import _)"),
    row("an import's alias", &[], "(import std.macro :as _)"),
    row(
        "an import's symbols",
        &[],
        "(import std.macro :symbols (?_))",
    ),
    row("a trace step's name", &[], "(trace (step _ :rule r))"),
    row(
        "a trace step's :derives",
        &[],
        "(trace (step s1 :derives (p ?_)))",
    ),
    // ── Keywords: each form takes a fixed set, and a term none ──
    // Each form takes a closed list of keywords, and any other is an error
    // (Q-M1e2.28). Each form's keywords are its tokens in `ein.y`, and no
    // production of a term or a nested fact takes one, so a keyword a form
    // does not list, `:where`, `:cardinality` and `:mode` among them, is
    // refused at the parse, and so is any keyword pair inside a term: none is
    // dropped in silence, by the compiler from a `:match` or a goal, by the
    // expander from a macro call, or by the loader from an annotation (the
    // regressions `where-*` and `kwpair-*`). The rows above for `query`,
    // `solve` and `config` are the
    // [`DELETED`] lines'. One row for each form, then one for each
    // place in a term; `fuzz_properties`' last property holds the lists on
    // every program the fuzzer's stream accepts.
    row(
        "a relation's keyword",
        &[],
        "(relation s Node :cardinality 1..1)",
    ),
    row(
        "a rule's keyword",
        &[],
        "(rule r () :match (p ?x) :assert (q ?x) :where (neq ?x a))",
    ),
    row(
        "an hrule's keyword",
        &[],
        "(hrule h () :match (p ?x) :assert (q ?x) :zz 1)",
    ),
    row("a fact's keyword", &[], "(relation q T) (q c :zz 1)"),
    row(
        "a stored negation's keyword",
        &[],
        "(relation q T) (not (q c) :k v)",
    ),
    row("a query's keyword", &[], "(query :goal (p ?x) :G 1)"),
    row("a solve's keyword", &[], "(solve :notes (p))"),
    row("a config's keyword", &[], "(config :zz true)"),
    row("an import's keyword", &[], "(import std.macro :zz 1)"),
    row("a macro's keyword", &[], "(macro m (?a) (p ?a) :k v)"),
    row("a trace's keyword", &[], "(trace :k v)"),
    row(
        "a trace step's keyword",
        &[],
        "(trace (step s1 :rule r :derives (q a) :zz 1))",
    ),
    row(
        "a :match's and",
        &[],
        "(rule r () :match (and (p ?x) :where (neq ?x a)) :assert (q ?x))",
    ),
    row(
        "a :match's and",
        &[],
        "(relation r T T)\n(rule old-where (?r)\n  :match (and (?r ?a ?b) :where (neq ?a ?b))\n  :assert (?r ?b ?a)\n  :why \"ow\")\n(old-where r)",
    ),
    row(
        "a :match's or",
        &[],
        "(rule r () :match (or (p ?x) (q ?x) :k v) :assert (hit ?x))",
    ),
    row(
        "a pattern",
        &[],
        "(rule r () :match (p ?x :k v) :assert (q ?x))",
    ),
    row(
        "a pattern, in an hrule",
        &[],
        "(hrule h () :match (p ?x :k v) :assert (q ?x))",
    ),
    row(
        "an absent",
        &[],
        "(rule r () :match (and (p ?x) (absent (q ?x) :k v)) :assert (hit ?x))",
    ),
    row(
        "a premise's stored negation",
        &[],
        "(rule r () :match (and (p ?x) (not (q ?x) :k v)) :assert (hit ?x))",
    ),
    row(
        "a guard",
        &[],
        "(rule r () :match (and (p ?x) (neq ?x a :k v)) :assert (hit ?x))",
    ),
    row(
        "an :assert's and",
        &[],
        "(rule r () :match (p ?x) :assert (and (q ?x) :k v))",
    ),
    row(
        "a conclusion",
        &[],
        "(rule r () :match (p ?x) :assert (q ?x :k v))",
    ),
    row(
        "a conclusion's stored negation",
        &[],
        "(rule r () :match (p ?x) :assert (not (q ?x) :k v))",
    ),
    row(
        "a :goal's and",
        &[],
        "(relation p T) (p a) (p b)\n(query :goal (and (p ?x) :zz 1) :expect (model (p a) (p b)))",
    ),
    row("a :goal's pattern", &[], "(query :goal (p ?x :zz 1))"),
    row(
        "a macro call in a :match",
        &[],
        "(import std.macro :symbols (forall))\n(rule r () :match (and (p ?x) (forall (q ?y) (p ?y) :k v)) :assert (hit ?x))",
    ),
    row(
        "a macro call in an :assert",
        &[],
        "(macro both (?a) (and (q ?a) (hit ?a)))\n(rule r () :match (p ?x) :assert (both ?x :k v))",
    ),
    row("a macro's body", &[], "(macro m (?a) (p ?a :k v))"),
    row(
        "a nested fact",
        &[],
        "(relation facts F) (relation r T T)\n(facts (r a b :source \"(1)\"))",
    ),
    row(
        "a nested fact in a :using",
        &[],
        "(relation q T) (q c :rule r :using ((p c :source \"s\")))",
    ),
    row(
        "a claim's model",
        &[],
        "(query :goal (p ?x) :expect (model (p a) (p b) :zz 1))",
    ),
    row(
        "a region claim",
        &[],
        "(relation p T) (p a) (solve :expect (region (model (p a)) :zz 1))",
    ),
    row(
        "a trace step's :derives",
        &[],
        "(trace (step s1 :derives (q a :k v)))",
    ),
    // ── The empty connective: `and` and `or` take one argument or more ──
    row(
        "a rule's :match, the empty conjunction",
        &[],
        "(rule r () :match (and) :assert (hit a))",
    ),
    row(
        "an hrule's :match, the empty conjunction",
        &[],
        "(hrule h () :match (and) :assert (hit a))",
    ),
    row(
        "a rule's :assert, the empty conclusion",
        &[],
        "(rule r () :match (p ?x) :assert (and))",
    ),
    row(
        "a conjunction inside a premise",
        &[],
        "(rule r () :match (and (p ?x) (and)) :assert (hit ?x))",
    ),
    row(
        "a rule's :match, the empty disjunction",
        &[],
        "(rule r () :match (or) :assert (hit a))",
    ),
    row(
        "a query's :goal",
        &[],
        "(relation p T) (p a) (query :goal (and))",
    ),
    row("a macro's template", &[], "(macro m (?x) (and))"),
    // ── `open`, the verdict, is written only as a rule's whole
    // :assert, `(open)` or `(open ?R)`, and is no name (Q-M1f.66) ──
    row("a top-level fact's relation", &[], "(open x)"),
    row(
        "a nested fact's relation",
        &[],
        "(relation p T) (p (open x))",
    ),
    row("a stored negation's relation", &[], "(not (open k))"),
    row("a fact's argument", &[], "(relation p T) (p open)"),
    row("an is-a's object", &[], "(is-a open Thing)"),
    row("a declaration's type", &[], "(relation t open)"),
    row("a fact's :rule", &[], "(relation p T) (p k :rule open)"),
    row(
        "a fact's :using",
        &[],
        "(relation p T) (p k :rule copy :using ((open k)))",
    ),
    row(
        "a pattern's relation inside a premise",
        &["open-in-match"],
        "(rule r () :match (and (n ?x) (p (open ?x))) :assert (hit ?x))",
    ),
    row(
        "the pattern a premise's not negates",
        &["open-in-match"],
        "(rule r () :match (and (n ?x) (not (open ?x))) :assert (hit ?x))",
    ),
    row(
        "a guard's value",
        &["open-in-match"],
        "(rule r () :match (and (n ?x) (neq ?x (open k))) :assert (hit ?x))",
    ),
    row(
        "a premise's argument",
        &[],
        "(rule r () :match (and (n ?x) (p open)) :assert (hit ?x))",
    ),
    row(
        "a guard's value",
        &[],
        "(rule r () :match (and (n ?x) (eq ?y open)) :assert (hit ?y))",
    ),
    row(
        "a conclusion's argument",
        &["open-inside"],
        "(rule r () :match (n ?x) :assert (hit (open ?x)))",
    ),
    row(
        "the pattern a conclusion's not negates",
        &["open-inside"],
        "(relation is-a T T) (relation r A B)\n(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) (absent (and (?isa ?b ?B) (?R ?a ?b)))) :assert (not (open ?R)))",
    ),
    row(
        "a conclusion's argument",
        &[],
        "(rule r () :match (n ?x) :assert (p open))",
    ),
    row(
        "an hrule's conclusion argument",
        &[],
        "(hrule h () :match (n ?x) :assert (p open))",
    ),
    row(
        "an (open …)'s argument",
        &[],
        "(rule r () :match (and (n ?x) (absent (q ?x))) :assert (open open))",
    ),
    row(
        "an activator's argument",
        &[],
        "(relation p T)\n(rule copy (?R) :match (?R ?x) :assert (p ?x))\n(copy open)",
    ),
    row(
        "a macro's template, the verdict",
        &["open-beside"],
        "(macro both (?r) (and (open ?r) (hit a)))",
    ),
    row(
        "a macro's template, the verdict",
        &["open-arity"],
        "(macro o2 (?r) (open ?r ?r))",
    ),
    row(
        "a macro's template, the verdict",
        &["open-argument", "open-in-match"],
        "(macro o1 (?r) (open ?r))",
    ),
    row(
        "a macro's template, a value",
        &[],
        "(macro po (?v) (p open))",
    ),
    row(
        "a macro call's argument",
        &[],
        "(macro pp (?v) (p ?v))\n(rule r () :match (and (n ?x) (pp open)) :assert (hit ?x))",
    ),
    row(
        "a query's :goal",
        &[],
        "(relation p T) (query :goal (p open))",
    ),
    row(
        "a pattern's relation inside a query's :goal",
        &[],
        "(relation p T) (query :goal (p (open ?x)))",
    ),
    row(
        "a query claim's fact",
        &[],
        "(relation p T) (query :goal (p ?x) :expect (model (p open)))",
    ),
    row(
        "a query claim's fact",
        &[],
        "(relation p T) (query :goal (p ?x) :expect (model (open k)))",
    ),
    row(
        "a query claim's neq",
        &[],
        "(relation p T) (query :goal (p ?x) :expect (model (neq k open)))",
    ),
    row("a query's :over", &[], "(query :goal (a ?x) :over open)"),
    row(
        "a region claim's fact",
        &[],
        "(relation p T) (solve :expect (region (model (p open))))",
    ),
    row(
        "a solve's relation list",
        &[],
        "(solve :hypothesis-relations (open))",
    ),
    row(
        "a solve's relation list",
        &[],
        "(solve :no-hypothesis (open))",
    ),
    row("a solve's :hrules", &[], "(solve :hrules (open))"),
    row("an import's module", &[], "(import open)"),
    row(
        "an import's :symbols",
        &[],
        "(import std.macro :symbols (open))",
    ),
    row(
        "a trace step's name",
        &[],
        "(trace (step open :derives (a k)))",
    ),
    row(
        "a trace step's :rule",
        &[],
        "(trace (step s1 :rule open :derives (a k)))",
    ),
    row(
        "a trace step's :derives",
        &[],
        "(trace (step s1 :derives (open k)))",
    ),
    // ── A character that is not ASCII, outside a string literal and a
    // comment (Q-M1f.65): refused at the lexer, at the character, in every
    // place; a Unicode space among them, which is no separator ──
    row("a fact's relation", &[], "(p\u{e9} k)"),
    row("a fact's value", &[], "(relation p T) (p caf\u{e9})"),
    row("a value alone", &[], "(relation p T) (p \u{2192})"),
    row("a declared relation's name", &[], "(relation t\u{2500} T)"),
    row("a declared relation's type", &[], "(relation t T\u{2500})"),
    row(
        "a rule's variable",
        &[],
        "(rule r () :match (p ?x\u{200b}) :assert (q ?x))",
    ),
    row("after a keyword", &[], "(relation t T :why\u{a0}\"t\")"),
    row(
        "a nested fact's relation",
        &[],
        "(relation p T) (p (q\u{e9} k))",
    ),
    row(
        "a query's goal",
        &[],
        "(relation p T) (query :goal (p ?x\u{1f993}))",
    ),
    row(
        "a query claim's fact",
        &[],
        "(relation p T) (query :goal (p ?x) :expect (model (p k\u{e9})))",
    ),
    row("a macro's body", &[], "(macro m (?v) (p\u{e9} ?v))"),
    row(
        "a config flag's value",
        &[],
        "(config :lattice-sanity-check true\u{3000})",
    ),
    row(
        "a solve's relation list",
        &[],
        "(solve :hypothesis-relations (p\u{e9}))",
    ),
    row(
        "a trace step's name",
        &[],
        "(trace (step s1\u{e9} :derives (a k)))",
    ),
    row("an import's module", &[], "(import std.macro\u{e9})"),
    row(
        "between two forms",
        &[],
        "(relation p T) (p k)\u{2028}(p j)",
    ),
    row("between two forms", &[], "(relation p T) (p k)\u{85}(p j)"),
    row("in place of a paren", &[], "(relation p T) \u{ff08}p k)"),
    row(
        "glued after a string",
        &[],
        "(relation t T :why \"t\"\u{e9})",
    ),
    row("after a #", &[], "(relation p T) (p k) #\u{e9}"),
    row("the file's first character", &[], "\u{feff}(relation p T)"),
    row(
        "inside a name, a bidirectional override",
        &[],
        "(relation p T) (p k\u{202e}j)",
    ),
    // ── `absent` is negation as failure, a premise's test, and heads
    // no fact, value or conclusion (the value rule, Q-M1f.113) ──
    row("a top-level fact's relation", &[], "(absent k)"),
    row(
        "a top-level fact's relation",
        &[],
        "(relation p T) (p a) (absent (p a))",
    ),
    row(
        "a nested fact's relation",
        &[],
        "(relation p T) (p (absent k))",
    ),
    row("a stored negation's relation", &[], "(not (absent k))"),
    row(
        "a fact's :using",
        &[],
        "(relation p T) (p k :rule copy :using ((absent k)))",
    ),
    row(
        "a conclusion's relation",
        &[],
        "(rule r () :match (n ?x) :assert (absent (p ?x)))",
    ),
    row(
        "an hrule's conclusion's relation",
        &[],
        "(hrule h () :match (n ?x) :assert (absent ?x))",
    ),
    row(
        "a conclusion's argument",
        &[],
        "(rule r () :match (n ?x) :assert (box (absent ?x)))",
    ),
    row(
        "the pattern a conclusion's not negates",
        &[],
        "(rule r () :match (n ?x) :assert (not (absent ?x)))",
    ),
    row(
        "the pattern a premise's not negates",
        &[],
        "(rule r () :match (and (n ?x) (not (absent ?x))) :assert (hit ?x))",
    ),
    row(
        "a guard's value",
        &[],
        "(rule r () :match (and (n ?x) (neq ?x (absent k))) :assert (hit ?x))",
    ),
    // ── `false` takes no argument, wherever it is written ──
    row("a top-level fact's relation", &[], "(false k)"),
    row(
        "a nested fact's relation",
        &[],
        "(relation p T) (p (false k))",
    ),
    row("a stored negation's relation", &[], "(not (false k))"),
    row(
        "a fact's :using",
        &[],
        "(relation p T) (p k :rule copy :using ((false k)))",
    ),
    row(
        "a premise",
        &[],
        "(rule r () :match (and (n ?x) (false ?x)) :assert (hit ?x))",
    ),
    row(
        "a premise under an absent",
        &[],
        "(rule r () :match (and (n ?x) (absent (false ?x))) :assert (hit ?x))",
    ),
    row(
        "a pattern's relation inside a premise",
        &[],
        "(rule r () :match (and (n ?x) (p (false ?x))) :assert (hit ?x))",
    ),
    row(
        "the pattern a premise's not negates",
        &[],
        "(rule r () :match (and (n ?x) (not (false k))) :assert (hit ?x))",
    ),
    row(
        "a guard's value",
        &[],
        "(rule r () :match (and (n ?x) (neq ?x (false k))) :assert (hit ?x))",
    ),
    row(
        "a conclusion's argument",
        &[],
        "(rule r () :match (n ?x) :assert (box (false ?x)))",
    ),
    row(
        "the pattern a conclusion's not negates",
        &[],
        "(rule r () :match (n ?x) :assert (not (false ?x)))",
    ),
    row("a goal", &[], "(relation p T) (query :goal (false ?s))"),
    row(
        "a claim's fact",
        &[],
        "(relation p T) (query :goal (p ?x) :expect (model (false k)))",
    ),
    // ── `(eq …)`, computed and never stored, is no value, as
    // `(neq …)` is none ──
    row(
        "a nested fact's relation",
        &[],
        "(relation p T) (p (eq k j))",
    ),
    row(
        "a nested fact's relation",
        &[],
        "(relation p T) (p (neq k j))",
    ),
    row(
        "a nested stored negation's relation",
        &[],
        "(relation p T) (p (not (eq k j)))",
    ),
    row(
        "a fact's :using",
        &[],
        "(relation p T) (p k :rule copy :using ((eq k k)))",
    ),
    row(
        "a conclusion's argument",
        &[],
        "(rule r () :match (and (n ?x) (n ?y)) :assert (box (eq ?x ?y)))",
    ),
    row(
        "a conclusion's argument, negated",
        &[],
        "(rule r () :match (n ?x) :assert (box (not (eq ?x k))))",
    ),
    row(
        "a guard's value",
        &[],
        "(rule r () :match (and (n ?x) (eq ?y (eq ?x k))) :assert (hit ?y))",
    ),
    row(
        "a claim's value",
        &[],
        "(relation p T) (query :goal (p ?x) :expect (model (p (eq a b))))",
    ),
    // ── `(solve :no-search)`, a flag ──
    // `:no-search` is a token: a lattice keyword a query does not take, and a
    // flag that takes no value.
    row(
        "a query's keyword",
        &["query-keyword"],
        "(query :goal (a ?x) :no-search)",
    ),
    row(
        "a solve's flag",
        &["solve-keyword"],
        "(relation p T) (solve :no-search (p))",
    ),
    // ── What a rule concludes: a conjunction of facts and stored negations,
    // `(false)` or a check, read flat at any depth; no disjunction, whole or
    // inside an `(and …)` however deep, and no connective inside a fact or
    // under a conclusion's `not`. Each is refused at the parse; these rows pin
    // each place ──
    row(
        "a rule's :assert",
        &[],
        "(rule r () :match (n ?x) :assert (or (p ?x) (q ?x)))",
    ),
    row(
        "an :assert's and",
        &[],
        "(rule r () :match (n ?x) :assert (and (p ?x) (or (q ?x) (hit ?x))))",
    ),
    row(
        "an and inside an :assert's and",
        &[],
        "(rule r () :match (n ?x) :assert (and (hit ?x) (and (p ?x) (or (q ?x) (box ?x)))))",
    ),
    row(
        "a conclusion's argument",
        &[],
        "(rule r () :match (n ?x) :assert (box (and (p ?x) (q ?x))))",
    ),
    row(
        "a conclusion's argument",
        &[],
        "(rule r () :match (n ?x) :assert (box (or (p ?x) (q ?x))))",
    ),
    row(
        "the pattern a conclusion's not negates",
        &[],
        "(rule r () :match (n ?x) :assert (not (and (p ?x) (q ?x))))",
    ),
    row(
        "the pattern a conclusion's not negates",
        &[],
        "(rule r () :match (n ?x) :assert (not (or (p ?x) (q ?x))))",
    ),
    row(
        "an hrule's :assert",
        &[],
        "(hrule h () :match (n ?x) :assert (or (p ?x) (q ?x)))",
    ),
];

/// A row's program: a rule row's declarations before it.
fn text(r: &Row) -> String {
    if r.program.contains("(rule ") || r.program.contains("(hrule ") {
        if r.program.contains("(relation ") {
            r.program.to_string()
        } else {
            format!("{RULES}{}", r.program)
        }
    } else {
        r.program.to_string()
    }
}

/// **Every shape the table holds is refused at the parse**, with the report
/// `tests/golden/grammar_table.txt` banks: every error, its notes, its ranked
/// repairs. A row whose `was` names a [`DELETED`] line is a shape that line
/// refused before the grammar did; one that names none loaded.
#[test]
fn every_row_of_the_grammars_table_is_refused_at_the_parse() {
    let mut out = String::new();
    for (i, r) in TABLE.iter().enumerate() {
        let program = text(r);
        let report = match parse(&mut Ast::new(), &program, Some("<row>")) {
            Ok(_) => panic!("row {i} ({}) parses: {program}", r.place),
            Err(e) => e.to_string(),
        };
        let was = if r.was.is_empty() {
            "—".to_string()
        } else {
            r.was.join(", ")
        };
        writeln!(out, "## {i}. {} (was: {was})", r.place).unwrap();
        for line in program.lines() {
            writeln!(out, "    {line}").unwrap();
        }
        writeln!(out, "{report}\n").unwrap();
    }
    let path = golden_path("ein-ir", "grammar_table.txt");
    if let Some(diff) = golden(&path, &out) {
        panic!("{diff}");
    }
}

/// **Each deleted line has a row**, and each row names only lines that were.
#[test]
fn every_deleted_line_is_a_row_of_the_table() {
    let ids: Vec<&str> = DELETED.iter().map(|d| d.0).collect();
    for r in TABLE {
        for w in r.was {
            assert!(
                ids.contains(w),
                "row {:?} names {w:?}, no deleted line",
                r.program
            );
        }
    }
    for (id, file, line) in DELETED {
        assert!(
            TABLE.iter().any(|r| r.was.contains(id)),
            "{file}'s deleted line {id:?} ({line}) has no row"
        );
    }
}

// ── What stays the loader's ────────────────────────────────────────

/// The load error `text` is refused with: it parses, and the loader refuses it.
fn load_error(text: &str) -> String {
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, Some("<loader>")).unwrap_or_else(|e| panic!("{text}\n{e}"));
    match load(&mut ast, &mut Terms::new(), &forms, None) {
        Ok(_) => panic!("{text} loads"),
        Err(e) => e.0,
    }
}

/// **The lines the grammar does not cover are still reached**, each by a
/// program the grammar accepts, which is why the loader keeps them.
///
/// - **A reserved name a declarator binds**: the kernel's two dunders are names
///   (Q-M1f.7), so the loader refuses `__closed__` bound by a
///   relation, a rule, a macro or an import's `:as`; the other reserved words
///   are tokens no declarator takes, refused at the parse (the table above).
/// - **The verdict's projection** (`validate_open`): the grammar writes
///   `open` only as a rule's whole `:assert`, `(open)` or
///   `(open ?R)`, so its placements are the table's; that `?R` is a parameter
///   and resolves against the rule's own `absent` is the program's to tell, and
///   each line starts with where the `(open …)` is written.
/// - **A premise that is not a form** (`Found::NotAForm`): a macro whose body
///   is a bare parameter puts its argument where a premise goes.
///
/// - **A kernel primitive in another shape than the grammar gives it**
///   (`refuse_malformed_primitives`, Q-M1f.80): a macro's
///   body writes `eq` or `absent` at any arity, `false` with an argument and
///   an `(or …)` as a conclusion, and a call's argument puts a connective where
///   `not` takes a pattern or a guard a value; a body that is a bare parameter
///   leaves a conclusion that is no form. The loader refuses each after
///   expansion, before the compiler or a firing sees it; every shape on every
///   command is `ein-cli/tests/macro_bodies.rs`'.
#[test]
fn the_lines_the_grammar_does_not_cover_are_still_reached() {
    for (text, want) in [
        (
            "(relation __closed__ T)",
            "relation '__closed__' shadows a reserved kernel name",
        ),
        (
            "(relation p T) (rule __symmetric__ () :match (p ?x) :assert (p ?x))",
            "rule '__symmetric__' shadows a reserved kernel name",
        ),
        (
            "(macro __closed__ (?x) (p ?x))",
            "macro '__closed__' shadows a reserved kernel name",
        ),
        (
            "(import std.macro :as __closed__)",
            "alias '__closed__' shadows a reserved kernel name",
        ),
        (
            "(relation p T) (relation hit T)\n\
             (rule r (?R) :match (and (p ?x) (absent (?R ?x ?y))) :assert (open ?S))",
            "<loader>:2:63: rule 'r': `(open ?S)` names a relation the activator does not bind",
        ),
        (
            "(macro id (?x) ?x)\n(relation p T) (relation hit T)\n(rule r () :match (id foo) :assert (hit a))",
            "rule 'r': its :match is foo, and foo is no premise",
        ),
        (
            "(macro all (?b ?G ?B) (absent (and ?G (absent ?B))))\n(relation hit T)\n\
             (rule r () :match (all ?x foo bar) :assert (hit a))",
            "rule 'r': its :match writes foo as a premise of and, and foo is no premise",
        ),
        (
            "(macro one (?a) (eq ?a))\n(relation p T) (relation hit T)\n\
             (rule r () :match (and (p ?x) (one ?x)) :assert (hit ?x))",
            "<loader>:1:18: rule 'r': its :match writes (eq ?x), and eq takes exactly 2 \
             arguments, not 1",
        ),
        (
            "(macro two (?a ?b) (absent ?a ?b))\n(relation p T) (relation hit T)\n\
             (rule r () :match (and (p ?x) (two (p ?x) (hit ?x))) :assert (hit ?x))",
            "rule 'r': its :match writes (absent (p ?x) (hit ?x)), and absent takes exactly 1 \
             argument, not 2",
        ),
        (
            "(macro neg (?a) (not ?a))\n(relation p T) (relation hit T)\n\
             (rule r () :match (and (p ?x) (absent (neg (and (p ?x) (hit ?x))))) :assert (hit ?x))",
            "rule 'r': its :match writes (not (and (p ?x) (hit ?x))), and not takes a pattern",
        ),
        (
            "(macro diff (?a ?b) (neq ?a ?b))\n(relation p T) (relation hit T)\n\
             (rule r () :match (and (p ?x) (diff ?x (and (p a)))) :assert (hit ?x))",
            "rule 'r': its :match writes (neq ?x (and (p a))), and (and (p a)) is a connective \
             where a value goes",
        ),
        (
            "(macro bad (?a) (false ?a))\n(relation p T)\n(rule r () :match (p ?x) :assert (bad ?x))",
            "rule 'r': its :assert writes (false ?x), and false takes no argument, not 1",
        ),
        (
            "(macro either (?a) (or (p ?a) (hit ?a)))\n(relation p T) (relation hit T)\n\
             (rule r () :match (p ?x) :assert (either ?x))",
            "rule 'r': its :assert writes (or (p ?x) (hit ?x)), and an (or …) is no conclusion",
        ),
        (
            "(macro id (?x) ?x)\n(relation p T)\n(rule r () :match (p ?y) :assert (id ?y))",
            "rule 'r': its :assert concludes ?y, which is not a fact",
        ),
    ] {
        let got = load_error(text);
        assert!(got.contains(want), "{text}\nwants {want:?}, got {got:?}");
    }
}
