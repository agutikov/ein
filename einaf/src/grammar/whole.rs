//! **The whole grammar of Ein, as GBNF, from `ein.l` and `ein.y`.**
//!
//! The two files are read through grmtools' public readers, as Ein's own
//! grammar page is written from them: `lrlex`'s `LRNonStreamingLexerDef` for
//! the lexer's rules, each regular expression read by `regex-syntax` and
//! translated node for node ([`regex_expr`]); `cfgrammar`'s `YaccGrammar`
//! for the phrase rules, each production an alternative and a list, which
//! `ein.y` writes as a rule that names itself first, the repetition it
//! derives.
//!
//! GBNF is one grammar over characters, and Ein's lexer is a pass of its own
//! that takes the longest match and, of two of one length, the earlier rule.
//! What that pass does is written by hand here, each piece a named rule of
//! the generator and a row of the inventory ([`HAND_WRITTEN`]):
//!
//! - **a name is a run of the name rule's characters that no earlier rule
//!   takes whole** (`exclusion_rules`): `and`, `relation`, `_` and the
//!   writeback names are tokens of their own, so `tok-name` is the run less
//!   those words, a tree of rules over their prefixes; `?_` likewise of a
//!   variable;
//! - **a token that is not a paren ends at a separator or a paren**: between
//!   two such tokens a separator is required, between a paren and anything it
//!   is not ([`Kind`], [`Flavor::sep`]); after a string it does not begin with
//!   `#`, which `BADTEXT` would take into the string's token;
//! - **a line comment runs to the line's end**: it is followed by a line
//!   break, or ends the text;
//! - **the tokens no production uses are left out**: `_`, `?_`, an unknown
//!   keyword, the writeback names, and the three catch-alls.
//!
//! The phrase grammar is compiled, rule by rule, into rules a state each
//! ([`Phrase`]): every token between two others carries the separator the
//! two kinds of token it stands between ask for. A rule that is not of one
//! kind at its start and one at its end is written into the rules that use
//! it, and one that is recursive through such rules is written once for each
//! kind it starts and ends with (`template-w`, `template-p`).
//!
//! **Two roots, two sets of rules.** `program` is Ein's language exactly,
//! as its lexer and parser read it. `root`, the root a model writes a
//! program under, is the contract's forms in **the characters a model
//! writes**: a name and a variable of [`ROOT_NAME_CHARS`], no block comment,
//! and no character of [`super::EXCLUDED`] in a string or a line comment,
//! so that no marker of the chat template and no fence can stand in what the
//! model writes. Its rules are `ein.y`'s compiled again, each named with
//! [`ROOT_PREFIX`] (`m-relation`, `m-tok-name`), over its own lexical rules
//! and separators ([`root_lexical`]).

use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr as _;

use cfgrammar::Symbol;
use cfgrammar::yacc::YaccGrammar;
use lrlex::{DefaultLexerTypes, LRNonStreamingLexerDef, LexerDef};
use regex_syntax::ast::parse::ParserBuilder;
use regex_syntax::ast::{
    Ast as Regex, ClassBracketed, ClassPerlKind, ClassSet, ClassSetBinaryOpKind, ClassSetItem,
    RepetitionKind, RepetitionRange,
};

use super::gbnf::{Expr, Grammar, Rule, merge_ranges};

/// `ein.l`, as the build of this package read it.
pub const EIN_L: &str = include_str!("../../../ein.rs/crates/ein-ir/src/ein.l");
/// `ein.y`, as the build of this package read it.
pub const EIN_Y: &str = include_str!("../../../ein.rs/crates/ein-ir/src/ein.y");

/// The root of the whole program, every form `ein.y`'s `Program` takes.
pub const PROGRAM: &str = "program";
/// **The root a model's program is written from**: what the contract
/// admits, `relation`, `rule`, `import`, `solve`, `query`, a fact, a stored
/// negation and a written `(open F)`, in the characters a model writes.
pub const ROOT: &str = "root";

/// **The characters of a name and a variable a model writes under
/// [`ROOT`]**: letters, digits, `_`, `.`, `*`, `+`, `=` and `-`, the
/// characters the library's names are written in and the examples' too;
/// none of [`super::EXCLUDED`], and no `/`.
pub const ROOT_NAME_CHARS: [(char, char); 7] = [
    ('*', '+'),
    ('-', '.'),
    ('0', '9'),
    ('=', '='),
    ('A', 'Z'),
    ('_', '_'),
    ('a', 'z'),
];

/// The prefix of the rules [`ROOT`] alone reaches.
pub const ROOT_PREFIX: &str = "m-";

/// The forms of `ein.y`'s `Form` the contract's root takes.
const CONTRACT_FORMS: [&str; 8] = [
    "Relation", "Rule", "Query", "Solve", "Import", "NotFact", "Fact", "OpenFact",
];

/// **Each hand-written rule of the generator, with the rule of `ein.l` or
/// `ein.y` it stands for**: the inventory's rows.
pub const HAND_WRITTEN: &[(&str, &str, &str)] = &[
    (
        "tok-name",
        "ein.l `NAME`, and every rule before it whose text is all name characters: the words (`and` … `commit`), `UNDERSCORE`, `WRITEBACK`",
        "a name is a run of the name characters that no earlier rule takes whole: the run less those words, written as a tree of rules over their prefixes (`tok-name-N`)",
    ),
    (
        "tok-var",
        "ein.l `VAR`, and `VARUNDERSCORE` before it",
        "a variable is `?` and a run of the name characters, less `?_`",
    ),
    (
        "gap, gap-word, gap-text, gap-after-text",
        "ein.l's skip rule, the start condition `AFTER_WORD`, and the catch-alls `BAD` and `BADTEXT`",
        "a token that is not a paren ends at a separator or a paren: between two such tokens a separator is required (`gap-word`), between a paren and anything it is not (`gap`); after a string it does not begin with `#`, which `BADTEXT` takes into the string's token (`gap-text`, `gap-after-text`)",
    ),
    (
        "comment-line-ended, gap-end",
        "ein.l's skip rule, its `;[^\\n]*`",
        "a line comment runs to the end of its line: it is followed by a line break (`comment-line-ended`), or ends the text (`gap-end`)",
    ),
    (
        "program, root",
        "ein.y `Program`, `TopForm` and `Form`",
        "a text is a separator, its forms with separators between them, and a separator; `root` takes the forms the contract admits (`relation`, `rule`, `import`, `solve`, `query`, a fact, a stored negation, a written `(open F)`) in the characters a model writes, over the rules named `m-…`, `program` every form `Program` takes",
    ),
    (
        "m-tok-name, m-tok-var",
        "ein.l `NAME` and `VAR`, under `root`",
        "a name the model writes is a run of letters, digits and `_ . * + = -` less the 26 words of those characters (`m-tok-name-N`, `m-tok-name-rest`): no writeback name can begin, so none is left out; a variable is `?` and the same run, less `?_`",
    ),
    (
        "m-tok-text, m-comment-line",
        "ein.l `TEXT` and the skip rule's `;[^\\n]*`, under `root`",
        "a string or a line comment the model writes is Ein's, less `<`, `>`, `|` and the backtick, the characters of the chat template's markers and of the fence",
    ),
    (
        "m-gap, m-gap-word, m-gap-end, m-comment-line-ended",
        "ein.l's skip rule and `AFTER_WORD`, under `root`",
        "the separators of `gap`, `gap-word`, `comment-line-ended` and `gap-end` with no block comment: white space and line comments only, so that a separator after a string is the one after a word",
    ),
    (
        "(left out)",
        "ein.l `UNDERSCORE`, `VARUNDERSCORE`, `WRITEBACK`, `KEYWORD`, `BAD`, `BADTEXT`, `BADESCAPE` (`ein.y`'s `%expect-unused`)",
        "the tokens no production uses are in no rule: a text that holds one is refused where it stands",
    ),
    (
        "the separators between tokens",
        "ein.y's productions",
        "each rule of `ein.y` is compiled into a rule a state, the separator between two tokens chosen by their kinds: a paren, a string, or another word",
    ),
];

// ── The lexical grammar, node for node ──────────────────────────────

/// Unicode's White_Space, which `\s` is in a class of `ein.l`'s.
const WHITE_SPACE: &[(u32, u32)] = &[
    (0x09, 0x0D),
    (0x20, 0x20),
    (0x85, 0x85),
    (0xA0, 0xA0),
    (0x1680, 0x1680),
    (0x2000, 0x200A),
    (0x2028, 0x2029),
    (0x202F, 0x202F),
    (0x205F, 0x205F),
    (0x3000, 0x3000),
];

/// A set of code points as sorted, disjoint ranges.
type Set = Vec<(u32, u32)>;

fn norm(mut v: Set) -> Set {
    v.sort();
    let mut out: Set = Vec::new();
    for (a, b) in v {
        match out.last_mut() {
            Some(l) if a <= l.1.saturating_add(1) => l.1 = l.1.max(b),
            _ => out.push((a, b)),
        }
    }
    out
}

fn complement(s: &Set) -> Set {
    let mut out = Vec::new();
    let mut next = 0u32;
    for &(a, b) in s {
        if a > next {
            out.push((next, a - 1));
        }
        next = b + 1;
    }
    if next <= 0x10FFFF {
        out.push((next, 0x10FFFF));
    }
    out
}

fn intersect(a: &Set, b: &Set) -> Set {
    let mut out = Vec::new();
    for &(x, y) in a {
        for &(u, v) in b {
            let (lo, hi) = (x.max(u), y.min(v));
            if lo <= hi {
                out.push((lo, hi));
            }
        }
    }
    norm(out)
}

fn contains(s: &Set, c: u32) -> bool {
    s.iter().any(|&(a, b)| a <= c && c <= b)
}

fn class_set(set: &ClassSet) -> Result<Set, String> {
    match set {
        ClassSet::Item(i) => class_item(i),
        ClassSet::BinaryOp(op) => {
            let (l, r) = (class_set(&op.lhs)?, class_set(&op.rhs)?);
            Ok(match op.kind {
                ClassSetBinaryOpKind::Difference => intersect(&l, &complement(&r)),
                ClassSetBinaryOpKind::Intersection => intersect(&l, &r),
                ClassSetBinaryOpKind::SymmetricDifference => norm(
                    intersect(&l, &complement(&r))
                        .into_iter()
                        .chain(intersect(&r, &complement(&l)))
                        .collect(),
                ),
            })
        }
    }
}

fn class_item(i: &ClassSetItem) -> Result<Set, String> {
    Ok(match i {
        ClassSetItem::Empty(_) => Vec::new(),
        ClassSetItem::Literal(l) => vec![(u32::from(l.c), u32::from(l.c))],
        ClassSetItem::Range(r) => vec![(u32::from(r.start.c), u32::from(r.end.c))],
        ClassSetItem::Perl(p) if p.kind == ClassPerlKind::Space => {
            let s = WHITE_SPACE.to_vec();
            if p.negated { complement(&s) } else { s }
        }
        ClassSetItem::Bracketed(b) => bracketed(b)?,
        ClassSetItem::Union(u) => {
            let mut out = Vec::new();
            for i in &u.items {
                out.extend(class_item(i)?);
            }
            norm(out)
        }
        other => {
            return Err(format!(
                "ein.l writes a class member GBNF is not given: {other:?}"
            ));
        }
    })
}

fn bracketed(b: &ClassBracketed) -> Result<Set, String> {
    let s = norm(class_set(&b.kind)?);
    Ok(if b.negated { complement(&s) } else { s })
}

fn ranges(s: &Set) -> Vec<(char, char)> {
    s.iter()
        .filter_map(|&(a, b)| {
            // A range across the surrogates is two.
            let a = char::from_u32(a).or_else(|| char::from_u32(0xE000))?;
            let b = char::from_u32(b).or_else(|| char::from_u32(0xD7FF))?;
            (a <= b).then_some((a, b))
        })
        .collect()
}

/// **A lexer rule's regular expression as GBNF, node for node** of the
/// syntax tree `regex-syntax` reads it as: a literal a literal, a
/// concatenation a sequence, `|` an alternation, `?` `*` `+` and `{m,n}`
/// their GBNF, a class the class of the same characters, written negated
/// where the regular expression negates it.
pub fn regex_expr(ast: &Regex) -> Result<Expr, String> {
    Ok(match ast {
        Regex::Empty(_) => Expr::Empty,
        Regex::Literal(l) => Expr::Lit(l.c.to_string()),
        Regex::Concat(c) => Expr::seq(c.asts.iter().map(regex_expr).collect::<Result<_, _>>()?),
        Regex::Alternation(a) => {
            Expr::alt(a.asts.iter().map(regex_expr).collect::<Result<_, _>>()?)
        }
        Regex::Group(g) => regex_expr(&g.ast)?,
        Regex::Repetition(r) => {
            if !r.greedy {
                return Err(format!("ein.l writes a lazy repetition: {ast}"));
            }
            let x = regex_expr(&r.ast)?;
            match r.op.kind {
                RepetitionKind::ZeroOrOne => Expr::opt(x),
                RepetitionKind::ZeroOrMore => Expr::star(x),
                RepetitionKind::OneOrMore => Expr::plus(x),
                RepetitionKind::Range(RepetitionRange::Exactly(n)) => Expr::times(x, n, Some(n)),
                RepetitionKind::Range(RepetitionRange::AtLeast(n)) => Expr::times(x, n, None),
                RepetitionKind::Range(RepetitionRange::Bounded(m, n)) => Expr::times(x, m, Some(n)),
            }
        }
        Regex::ClassBracketed(c) => {
            let s = norm(class_set(&c.kind)?);
            Expr::Class {
                negated: c.negated,
                ranges: merge_ranges(&ranges(&s)),
            }
        }
        Regex::ClassPerl(p) if p.kind == ClassPerlKind::Space => Expr::Class {
            negated: p.negated,
            ranges: merge_ranges(&ranges(&WHITE_SPACE.to_vec())),
        },
        other => {
            return Err(format!(
                "ein.l writes `{other}`, which GBNF is not given here"
            ));
        }
    })
}

/// The set of characters an expression of one character takes, where it is
/// one.
fn one_char(e: &Expr) -> Option<Set> {
    match e {
        Expr::Lit(s) if s.chars().count() == 1 => {
            let c = u32::from(s.chars().next()?);
            Some(vec![(c, c)])
        }
        Expr::Class { negated, ranges } => {
            let s: Set = norm(
                ranges
                    .iter()
                    .map(|&(a, b)| (u32::from(a), u32::from(b)))
                    .collect(),
            );
            Some(if *negated { complement(&s) } else { s })
        }
        _ => None,
    }
}

/// Every text a finite expression takes; `None` where it takes infinitely
/// many, or more than a few.
fn finite(e: &Expr) -> Option<Vec<String>> {
    match e {
        Expr::Empty => Some(vec![String::new()]),
        Expr::Lit(s) => Some(vec![s.clone()]),
        Expr::Alt(v) => {
            let mut out = Vec::new();
            for x in v {
                out.extend(finite(x)?);
            }
            Some(out)
        }
        Expr::Seq(v) => {
            let mut out = vec![String::new()];
            for x in v {
                let tails = finite(x)?;
                out = out
                    .iter()
                    .flat_map(|h| tails.iter().map(move |t| format!("{h}{t}")))
                    .collect();
                if out.len() > 64 {
                    return None;
                }
            }
            Some(out)
        }
        Expr::Class { .. } => {
            let s = one_char(e)?;
            let n: u32 = s.iter().map(|&(a, b)| b - a + 1).sum();
            if n > 64 {
                return None;
            }
            Some(
                s.iter()
                    .flat_map(|&(a, b)| a..=b)
                    .filter_map(char::from_u32)
                    .map(|c| c.to_string())
                    .collect(),
            )
        }
        Expr::Opt(x) => {
            let mut out = vec![String::new()];
            out.extend(finite(x)?);
            Some(out)
        }
        _ => None,
    }
}

/// One rule of `ein.l`.
#[derive(Clone, Debug)]
struct LexRule {
    name: Option<String>,
    expr: Expr,
}

fn lex_rules(src: &str) -> Result<Vec<LexRule>, String> {
    type Def = LRNonStreamingLexerDef<DefaultLexerTypes<u32>>;
    let def =
        Def::from_str(src).map_err(|e| format!("ein.l does not read through lrlex: {e:?}"))?;
    let mut out = Vec::new();
    for r in def.iter_rules() {
        let ast = ParserBuilder::new()
            .octal(true)
            .build()
            .parse(r.re_str())
            .map_err(|e| format!("ein.l's rule `{}` does not read: {e}", r.re_str()))?;
        out.push(LexRule {
            name: r.name().map(str::to_string),
            expr: regex_expr(&ast)?,
        });
    }
    Ok(out)
}

// ── A token, by its kind ─────────────────────────────────────────────

/// **What a token is, for the separator before and after it**: a paren,
/// after which and before which no separator is needed; a string, which a
/// separator beginning with `#` would be glued to; and every other token, a
/// word, between two of which a separator is required.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Paren,
    Word,
    Text,
}

impl Kind {
    fn letter(self) -> char {
        match self {
            Kind::Paren => 'p',
            Kind::Word => 'w',
            Kind::Text => 't',
        }
    }
}

/// **How the separator between two tokens is written.**
#[derive(Clone, Debug)]
pub enum Flavor {
    /// As Ein's lexer reads a text: any white space and any comment, where
    /// a separator is allowed at all.
    Exact,
    /// As Ein's lexer reads a text with no block comment, under [`ROOT`]:
    /// no separator begins with `#`, so the one after a string is the one
    /// after a word (`gap`, `gap-word`, each named with [`ROOT_PREFIX`]
    /// where it is written).
    Root,
    /// As a model writes a program: a run of spaces, or a line break and an
    /// indentation, each bounded, and no comment; the rules `sp` and `sp0`
    /// of the grammar it is written into.
    Model,
}

impl Flavor {
    /// The separator between a token of kind `prev` and one of kind `next`.
    pub fn sep(&self, prev: Kind, next: Kind) -> Expr {
        match self {
            Flavor::Exact => match (prev, next) {
                (Kind::Paren, _) | (Kind::Word, Kind::Paren) => Expr::r("gap"),
                (Kind::Word, _) => Expr::r("gap-word"),
                (Kind::Text, Kind::Paren) => Expr::r("gap-after-text"),
                (Kind::Text, _) => Expr::r("gap-text"),
            },
            Flavor::Root => match (prev, next) {
                (_, Kind::Paren) | (Kind::Paren, _) => Expr::r("gap"),
                _ => Expr::r("gap-word"),
            },
            Flavor::Model => match (prev, next) {
                (Kind::Paren, _) | (_, Kind::Paren) => Expr::r("sp0"),
                _ => Expr::r("sp"),
            },
        }
    }
}

// ── The phrase grammar ──────────────────────────────────────────────

/// A symbol of `ein.y`: a rule or a token, by index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Sym {
    Rule(u32),
    Tok(u32),
}

/// A rule's body as a regular expression over symbols.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Re {
    Eps,
    Sym(Sym),
    Seq(Vec<Re>),
    Alt(Vec<Re>),
    Star(Box<Re>),
}

/// An atom of a compiled rule: a token, or a rule of one start and end
/// kind, by its name.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Atom {
    /// A token: its GBNF, and its kind.
    Tok(String, Kind),
    /// A rule: its name, and its first and last kinds.
    Rule(String, Kind, Kind),
}

impl Atom {
    fn first(&self) -> Kind {
        match self {
            Atom::Tok(_, k) | Atom::Rule(_, k, _) => *k,
        }
    }
    fn last(&self) -> Kind {
        match self {
            Atom::Tok(_, k) | Atom::Rule(_, _, k) => *k,
        }
    }
    fn expr(&self) -> Expr {
        match self {
            Atom::Tok(t, _) => {
                if t.starts_with("tok-") {
                    Expr::r(t)
                } else {
                    Expr::lit(t)
                }
            }
            Atom::Rule(n, ..) => Expr::r(n),
        }
    }
}

/// **A regular expression over atoms**: what a rule is compiled from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ar {
    Eps,
    Atom(Atom),
    Seq(Vec<Ar>),
    Alt(Vec<Ar>),
    Star(Box<Ar>),
}

impl Ar {
    pub fn plus(a: Ar) -> Ar {
        Ar::Seq(vec![a.clone(), Ar::Star(Box::new(a))])
    }
    pub fn opt(a: Ar) -> Ar {
        Ar::Alt(vec![Ar::Eps, a])
    }
}

/// **Compile a regular expression over atoms into right-linear rules**, a
/// rule a state of its Glushkov automaton: `name` begins at a first atom with
/// no separator before it, every later atom is preceded by the separator
/// the kinds of the two atoms ask for, and the rule ends after a last atom,
/// where a state that may end has `""` among its alternatives. Only the
/// paths that start with an atom of kind `first` and end with one of kind
/// `last` are kept, where those are given. States that behave alike are
/// one rule.
pub fn compile(
    name: &str,
    re: &Ar,
    flavor: &Flavor,
    first: Option<Kind>,
    last: Option<Kind>,
) -> Vec<Rule> {
    // Linearise.
    let mut atoms: Vec<Atom> = Vec::new();
    let mut follow: Vec<BTreeSet<usize>> = Vec::new();
    let (firsts, lasts, nullable) = glushkov(re, &mut atoms, &mut follow);
    let _ = nullable;
    let n = atoms.len();
    let start: BTreeSet<usize> = firsts
        .into_iter()
        .filter(|&p| first.is_none_or(|k| atoms[p].first() == k))
        .collect();
    let finals: BTreeSet<usize> = lasts
        .into_iter()
        .filter(|&p| last.is_none_or(|k| atoms[p].last() == k))
        .collect();
    // Trim: reachable from the start and reaching a final.
    let mut reach = vec![false; n];
    let mut todo: Vec<usize> = start.iter().copied().collect();
    while let Some(p) = todo.pop() {
        if reach[p] {
            continue;
        }
        reach[p] = true;
        todo.extend(follow[p].iter().copied());
    }
    let mut coreach = vec![false; n];
    loop {
        let mut changed = false;
        for p in 0..n {
            if !coreach[p] && (finals.contains(&p) || follow[p].iter().any(|&q| coreach[q])) {
                coreach[p] = true;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let live = |p: usize| reach[p] && coreach[p];
    let fol = |p: usize| -> Vec<usize> { follow[p].iter().copied().filter(|&q| live(q)).collect() };
    // Moore's refinement: a state's class is its last kind, whether it may
    // end, and the classes of the states it goes to, by their atoms.
    let states: Vec<usize> = (0..n).filter(|&p| live(p)).collect();
    let mut class: BTreeMap<usize, usize> = BTreeMap::new();
    let key0 = |p: usize| (atoms[p].last(), finals.contains(&p));
    let mut keys: Vec<(Kind, bool)> = Vec::new();
    for &p in &states {
        let k = key0(p);
        let id = keys.iter().position(|x| *x == k).unwrap_or_else(|| {
            keys.push(k);
            keys.len() - 1
        });
        class.insert(p, id);
    }
    loop {
        let mut sigs: Vec<(usize, BTreeSet<(Atom, usize)>)> = Vec::new();
        let mut next: BTreeMap<usize, usize> = BTreeMap::new();
        for &p in &states {
            let out: BTreeSet<(Atom, usize)> = fol(p)
                .into_iter()
                .map(|q| (atoms[q].clone(), class[&q]))
                .collect();
            let sig = (class[&p], out);
            let id = sigs.iter().position(|s| *s == sig).unwrap_or_else(|| {
                sigs.push(sig.clone());
                sigs.len() - 1
            });
            next.insert(p, id);
        }
        let stable = next.values().collect::<BTreeSet<_>>().len()
            == class.values().collect::<BTreeSet<_>>().len();
        class = next;
        if stable {
            break;
        }
    }
    // Number the classes that need a rule (a state with somewhere to go) in
    // the order their first state is met.
    let mut names: BTreeMap<usize, String> = BTreeMap::new();
    let mut order: Vec<usize> = Vec::new();
    let mut seen: BTreeSet<usize> = BTreeSet::new();
    let mut queue: std::collections::VecDeque<usize> =
        start.iter().copied().filter(|&p| live(p)).collect();
    while let Some(p) = queue.pop_front() {
        if !seen.insert(p) {
            continue;
        }
        let c = class[&p];
        if !fol(p).is_empty() && !names.contains_key(&c) {
            names.insert(c, format!("{name}-{}", names.len() + 1));
            order.push(p);
        }
        queue.extend(fol(p));
    }
    let step = |prev: Kind, q: usize| -> Expr {
        let mut parts = vec![flavor.sep(prev, atoms[q].first()), atoms[q].expr()];
        if let Some(n) = names.get(&class[&q]) {
            parts.push(Expr::r(n));
        }
        Expr::seq(parts)
    };
    let mut rules = Vec::new();
    let begin: Vec<Expr> = start
        .iter()
        .filter(|&&p| live(p))
        .map(|&q| {
            let mut parts = vec![atoms[q].expr()];
            if let Some(n) = names.get(&class[&q]) {
                parts.push(Expr::r(n));
            }
            Expr::seq(parts)
        })
        .collect();
    assert!(!begin.is_empty(), "the rule {name} takes no text");
    rules.push(Rule::new(name, Expr::alt(begin)));
    for p in order {
        let mut alts: Vec<Expr> = fol(p)
            .into_iter()
            .map(|q| step(atoms[p].last(), q))
            .collect();
        if finals.contains(&p) {
            alts.push(Expr::Empty);
        }
        rules.push(Rule::new(&names[&class[&p]], Expr::alt(alts)));
    }
    rules
}

/// The Glushkov construction: number each atom, and return the first and
/// last positions and whether the expression takes the empty text,
/// `follow` filled.
fn glushkov(
    re: &Ar,
    atoms: &mut Vec<Atom>,
    follow: &mut Vec<BTreeSet<usize>>,
) -> (BTreeSet<usize>, BTreeSet<usize>, bool) {
    match re {
        Ar::Eps => (BTreeSet::new(), BTreeSet::new(), true),
        Ar::Atom(a) => {
            atoms.push(a.clone());
            follow.push(BTreeSet::new());
            let p = atoms.len() - 1;
            (BTreeSet::from([p]), BTreeSet::from([p]), false)
        }
        Ar::Alt(v) => {
            let (mut f, mut l, mut n) = (BTreeSet::new(), BTreeSet::new(), false);
            for x in v {
                let (a, b, c) = glushkov(x, atoms, follow);
                f.extend(a);
                l.extend(b);
                n |= c;
            }
            (f, l, n)
        }
        Ar::Seq(v) => {
            let (mut f, mut l, mut n): (BTreeSet<usize>, BTreeSet<usize>, bool) =
                (BTreeSet::new(), BTreeSet::new(), true);
            for x in v {
                let (a, b, c) = glushkov(x, atoms, follow);
                for &p in &l {
                    follow[p].extend(a.iter().copied());
                }
                if n {
                    f.extend(a.iter().copied());
                }
                if c {
                    l.extend(b);
                } else {
                    l = b;
                }
                n &= c;
            }
            (f, l, n)
        }
        Ar::Star(x) => {
            let (a, b, _) = glushkov(x, atoms, follow);
            for &p in &b {
                follow[p].extend(a.iter().copied());
            }
            (a, b, true)
        }
    }
}

/// `CamelCase` as `camel-case`.
fn kebab(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                out.push('-');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// A rule's start and end kinds, and whether it takes the empty text.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Sig {
    pairs: BTreeSet<(Kind, Kind)>,
    nullable: bool,
}

/// **`ein.y`, read and ready to compile.**
pub struct Phrase {
    grm: YaccGrammar<u32>,
    bodies: BTreeMap<u32, Re>,
    sigs: BTreeMap<u32, Sig>,
    tok_kind: BTreeMap<u32, Kind>,
    /// A token's GBNF: its text, or the rule of its class.
    tok_gbnf: BTreeMap<u32, String>,
    /// The rules written into the rules that use them.
    inline: BTreeSet<u32>,
}

impl Phrase {
    pub fn read(y_src: &str, literal_tokens: &BTreeSet<String>) -> Result<Phrase, String> {
        let grm = YaccGrammar::<u32>::from_str(y_src)
            .map_err(|e| format!("ein.y does not read through cfgrammar: {e:?}"))?;
        let start = grm.start_rule_idx();
        let mut tok_kind = BTreeMap::new();
        let mut tok_gbnf = BTreeMap::new();
        for t in grm.iter_tidxs() {
            let Some(name) = grm.token_name(t) else {
                continue;
            };
            let (kind, gbnf) = match name {
                "(" | ")" => (Kind::Paren, name.to_string()),
                "TEXT" => (Kind::Text, "tok-text".to_string()),
                "NAME" => (Kind::Word, "tok-name".to_string()),
                "VAR" => (Kind::Word, "tok-var".to_string()),
                n if literal_tokens.contains(n) => (Kind::Word, n.to_string()),
                // A token no production uses: left out.
                _ => continue,
            };
            tok_kind.insert(t.as_storaget(), kind);
            tok_gbnf.insert(t.as_storaget(), gbnf);
        }
        let mut bodies = BTreeMap::new();
        for r in grm.iter_rules() {
            if r == start || grm.implicit_rule() == Some(r) {
                continue;
            }
            let me = Symbol::Rule(r);
            let prods: Vec<Vec<Sym>> = grm
                .rule_to_prods(r)
                .iter()
                .map(|&p| {
                    grm.prod(p)
                        .iter()
                        .map(|s| match s {
                            Symbol::Rule(x) => Sym::Rule(x.as_storaget()),
                            Symbol::Token(t) => Sym::Tok(t.as_storaget()),
                        })
                        .collect()
                })
                .collect();
            let raw: Vec<&[Symbol<u32>]> =
                grm.rule_to_prods(r).iter().map(|&p| grm.prod(p)).collect();
            let seq = |p: &[Sym]| -> Re {
                match p.len() {
                    0 => Re::Eps,
                    1 => Re::Sym(p[0]),
                    _ => Re::Seq(p.iter().map(|&s| Re::Sym(s)).collect()),
                }
            };
            let (again, base): (Vec<usize>, Vec<usize>) =
                (0..raw.len()).partition(|&i| raw[i].first() == Some(&me));
            let list = !again.is_empty()
                && again
                    .iter()
                    .all(|&i| raw[i].len() > 1 && !raw[i][1..].contains(&me))
                && base.iter().all(|&i| !raw[i].contains(&me));
            let body = if list {
                let item = Re::Alt(again.iter().map(|&i| seq(&prods[i][1..])).collect());
                let b = Re::Alt(base.iter().map(|&i| seq(&prods[i])).collect());
                Re::Seq(vec![b, Re::Star(Box::new(item))])
            } else {
                Re::Alt(prods.iter().map(|p| seq(p)).collect())
            };
            bodies.insert(r.as_storaget(), body);
        }
        let mut ph = Phrase {
            grm,
            bodies,
            sigs: BTreeMap::new(),
            tok_kind,
            tok_gbnf,
            inline: BTreeSet::new(),
        };
        ph.signatures();
        ph.choose_inline();
        Ok(ph)
    }

    pub fn rule_idx(&self, name: &str) -> Option<u32> {
        self.grm.rule_idx(name).map(|r| r.as_storaget())
    }

    fn name_of(&self, r: u32) -> String {
        kebab(self.grm.rule_name_str(cfgrammar::RIdx(r)))
    }

    /// Every rule's start and end kinds, to a fixpoint.
    fn signatures(&mut self) {
        loop {
            let mut changed = false;
            let ids: Vec<u32> = self.bodies.keys().copied().collect();
            for r in ids {
                let s = self.sig_of(&self.bodies[&r]);
                if self.sigs.get(&r) != Some(&s) {
                    self.sigs.insert(r, s);
                    changed = true;
                }
            }
            if !changed {
                return;
            }
        }
    }

    fn sig_of(&self, re: &Re) -> Sig {
        match re {
            Re::Eps => Sig {
                pairs: BTreeSet::new(),
                nullable: true,
            },
            Re::Sym(Sym::Tok(t)) => match self.tok_kind.get(t) {
                Some(&k) => Sig {
                    pairs: BTreeSet::from([(k, k)]),
                    nullable: false,
                },
                // A token left out takes nothing.
                None => Sig::default(),
            },
            Re::Sym(Sym::Rule(r)) => self.sigs.get(r).cloned().unwrap_or_default(),
            Re::Alt(v) => {
                let mut s = Sig::default();
                for x in v {
                    let y = self.sig_of(x);
                    s.pairs.extend(y.pairs);
                    s.nullable |= y.nullable;
                }
                s
            }
            Re::Seq(v) => {
                let mut s = Sig {
                    pairs: BTreeSet::new(),
                    nullable: true,
                };
                for x in v {
                    let y = self.sig_of(x);
                    let mut pairs = BTreeSet::new();
                    for &(f, _) in &s.pairs {
                        for &(_, l) in &y.pairs {
                            pairs.insert((f, l));
                        }
                    }
                    if y.nullable {
                        pairs.extend(s.pairs.iter().copied());
                    }
                    if s.nullable {
                        pairs.extend(y.pairs.iter().copied());
                    }
                    s.pairs = pairs;
                    s.nullable &= y.nullable;
                }
                s
            }
            Re::Star(x) => {
                let y = self.sig_of(x);
                let firsts: BTreeSet<Kind> = y.pairs.iter().map(|p| p.0).collect();
                let lasts: BTreeSet<Kind> = y.pairs.iter().map(|p| p.1).collect();
                let mut pairs = BTreeSet::new();
                for &f in &firsts {
                    for &l in &lasts {
                        pairs.insert((f, l));
                    }
                }
                Sig {
                    pairs,
                    nullable: true,
                }
            }
        }
    }

    fn is_list(&self, r: u32) -> bool {
        matches!(&self.bodies[&r], Re::Seq(v) if v.len() == 2 && matches!(v[1], Re::Star(_)))
    }

    fn pure(&self, r: u32) -> bool {
        let s = &self.sigs[&r];
        s.pairs.len() == 1 && !s.nullable
    }

    fn refs(re: &Re, out: &mut BTreeSet<u32>) {
        match re {
            Re::Sym(Sym::Rule(r)) => {
                out.insert(*r);
            }
            Re::Seq(v) | Re::Alt(v) => v.iter().for_each(|x| Self::refs(x, out)),
            Re::Star(x) => Self::refs(x, out),
            _ => {}
        }
    }

    /// The rules written into their users: a list, and a rule of more than
    /// one start or end kind, unless it is recursive through such rules,
    /// where a rule that is no list breaks the cycle and is written once a
    /// kind.
    fn choose_inline(&mut self) {
        let mut cand: BTreeSet<u32> = self
            .bodies
            .keys()
            .copied()
            .filter(|&r| !self.pure(r) || self.is_list(r))
            .collect();
        loop {
            // Find a cycle among the candidates.
            let edges: BTreeMap<u32, BTreeSet<u32>> = cand
                .iter()
                .map(|&r| {
                    let mut o = BTreeSet::new();
                    Self::refs(&self.bodies[&r], &mut o);
                    (r, o.intersection(&cand).copied().collect())
                })
                .collect();
            let cyclic: Option<u32> = cand.iter().copied().find(|&r| {
                // r reaches itself
                let mut seen = BTreeSet::new();
                let mut todo: Vec<u32> = edges[&r].iter().copied().collect();
                while let Some(x) = todo.pop() {
                    if x == r {
                        return true;
                    }
                    if seen.insert(x) {
                        todo.extend(edges[&x].iter().copied());
                    }
                }
                false
            });
            let Some(r) = cyclic else {
                break;
            };
            // Break at a rule of the cycle that is no list, if there is one.
            let mut cyc = vec![r];
            let mut seen = BTreeSet::new();
            let mut todo: Vec<u32> = edges[&r].iter().copied().collect();
            while let Some(x) = todo.pop() {
                if seen.insert(x) {
                    cyc.push(x);
                    todo.extend(edges[&x].iter().copied());
                }
            }
            let reaches_back = |x: u32| {
                let mut seen = BTreeSet::new();
                let mut todo: Vec<u32> = edges[&x].iter().copied().collect();
                while let Some(y) = todo.pop() {
                    if y == x {
                        return true;
                    }
                    if seen.insert(y) {
                        todo.extend(edges[&y].iter().copied());
                    }
                }
                false
            };
            let breaker = cyc
                .iter()
                .copied()
                .filter(|&x| reaches_back(x))
                .find(|&x| !self.is_list(x))
                .unwrap_or(r);
            cand.remove(&breaker);
        }
        self.inline = cand;
    }

    /// The variants a written rule has: one a pair of start and end kinds.
    fn variants(&self, r: u32) -> Vec<(String, Kind, Kind)> {
        let s = &self.sigs[&r];
        let base = self.name_of(r);
        if s.pairs.len() == 1 {
            let (f, l) = *s.pairs.iter().next().expect("one pair");
            return vec![(base, f, l)];
        }
        s.pairs
            .iter()
            .map(|&(f, l)| {
                let suffix = if f == l {
                    f.letter().to_string()
                } else {
                    format!("{}{}", f.letter(), l.letter())
                };
                (format!("{base}-{suffix}"), f, l)
            })
            .collect()
    }

    /// A body over symbols as one over atoms: a rule written into its users
    /// replaced by its body, any other by its variants.
    fn atoms_of(&self, re: &Re) -> Ar {
        match re {
            Re::Eps => Ar::Eps,
            Re::Sym(Sym::Tok(t)) => match (self.tok_gbnf.get(t), self.tok_kind.get(t)) {
                (Some(g), Some(&k)) => Ar::Atom(Atom::Tok(g.clone(), k)),
                // A token no production uses takes nothing.
                _ => Ar::Alt(Vec::new()),
            },
            Re::Sym(Sym::Rule(r)) if self.inline.contains(r) => self.atoms_of(&self.bodies[r]),
            Re::Sym(Sym::Rule(r)) => {
                let mut alts: Vec<Ar> = self
                    .variants(*r)
                    .into_iter()
                    .map(|(n, f, l)| Ar::Atom(Atom::Rule(n, f, l)))
                    .collect();
                if self.sigs[r].nullable {
                    alts.push(Ar::Eps);
                }
                if alts.len() == 1 {
                    alts.remove(0)
                } else {
                    Ar::Alt(alts)
                }
            }
            Re::Seq(v) => Ar::Seq(v.iter().map(|x| self.atoms_of(x)).collect()),
            Re::Alt(v) => Ar::Alt(v.iter().map(|x| self.atoms_of(x)).collect()),
            Re::Star(x) => Ar::Star(Box::new(self.atoms_of(x))),
        }
    }

    /// **A rule's body over atoms**, by its name in `ein.y`.
    pub fn body(&self, name: &str) -> Option<Ar> {
        let r = self.rule_idx(name)?;
        Some(self.atoms_of(&self.bodies[&r]))
    }

    /// The written rules, in `ein.y`'s order, each with its body compiled in
    /// `flavor`, the rules reachable from `roots` alone, a rule's comment
    /// naming the rule of `ein.y` it is.
    pub fn compile_from(&self, roots: &[Ar], flavor: &Flavor) -> Vec<Rule> {
        let mut need: BTreeSet<String> = BTreeSet::new();
        let mut todo: Vec<Ar> = roots.to_vec();
        let mut done: BTreeSet<String> = BTreeSet::new();
        while let Some(a) = todo.pop() {
            let mut names = BTreeSet::new();
            Self::rule_atoms(&a, &mut names);
            for n in names {
                if done.insert(n.clone()) {
                    need.insert(n.clone());
                    let r = self.rule_of_variant(&n);
                    todo.push(self.atoms_of(&self.bodies[&r]));
                }
            }
        }
        let mut out = Vec::new();
        for (&r, body) in &self.bodies {
            if self.inline.contains(&r) {
                continue;
            }
            let ar = self.atoms_of(body);
            let ein_y = self.grm.rule_name_str(cfgrammar::RIdx(r)).to_string();
            for (name, f, l) in self.variants(r) {
                if !need.contains(&name) {
                    continue;
                }
                let one = self.sigs[&r].pairs.len() == 1;
                let mut rules =
                    compile(&name, &ar, flavor, (!one).then_some(f), (!one).then_some(l));
                rules[0] = rules[0].clone().doc(&if one {
                    format!("ein.y: {ein_y}")
                } else {
                    format!(
                        "ein.y: {ein_y}, its alternatives that begin with {} and end with {}",
                        kind_words(f),
                        kind_words(l)
                    )
                });
                out.extend(rules);
            }
        }
        out
    }

    fn rule_of_variant(&self, n: &str) -> u32 {
        for &r in self.bodies.keys() {
            if !self.inline.contains(&r) && self.variants(r).iter().any(|v| v.0 == n) {
                return r;
            }
        }
        panic!("no rule of ein.y is written as {n}")
    }

    fn rule_atoms(a: &Ar, out: &mut BTreeSet<String>) {
        match a {
            Ar::Atom(Atom::Rule(n, ..)) => {
                out.insert(n.clone());
            }
            Ar::Seq(v) | Ar::Alt(v) => v.iter().for_each(|x| Self::rule_atoms(x, out)),
            Ar::Star(x) => Self::rule_atoms(x, out),
            _ => {}
        }
    }
}

fn kind_words(k: Kind) -> &'static str {
    match k {
        Kind::Paren => "a paren",
        Kind::Word => "a word",
        Kind::Text => "a string",
    }
}

// ── The hand-written lexical rules ──────────────────────────────────

/// The rules of a name class less a set of words: a rule a prefix of a
/// word, `base` the rule for the empty prefix, `prefix` a text every token
/// starts with.
pub(crate) fn exclusion_rules(
    base: &str,
    prefix: &str,
    class: &[(u32, u32)],
    words: &BTreeSet<String>,
) -> Vec<Rule> {
    let class: &Set = &class.to_vec();
    let tail = format!("{base}-rest");
    // The prefixes, in breadth-first order, the empty one first.
    let mut prefixes: Vec<String> = vec![String::new()];
    let mut i = 0;
    while i < prefixes.len() {
        let p = prefixes[i].clone();
        let mut next: BTreeSet<char> = BTreeSet::new();
        for w in words {
            if let Some(rest) = w.strip_prefix(p.as_str())
                && let Some(c) = rest.chars().next()
            {
                next.insert(c);
            }
        }
        for c in next {
            prefixes.push(format!("{p}{c}"));
        }
        i += 1;
    }
    let name_of = |p: &str| -> String {
        let i = prefixes.iter().position(|x| x == p).expect("a prefix");
        if i == 0 {
            base.to_string()
        } else {
            format!("{base}-{i}")
        }
    };
    let mut rules = Vec::new();
    for p in &prefixes {
        let mut alts: Vec<Expr> = Vec::new();
        if !p.is_empty() && !words.contains(p) {
            alts.push(Expr::Empty);
        }
        let mut taken: Set = Vec::new();
        let kids: BTreeSet<char> = words
            .iter()
            .filter_map(|w| w.strip_prefix(p.as_str()).and_then(|r| r.chars().next()))
            .collect();
        for c in kids {
            let q = format!("{p}{c}");
            alts.push(Expr::seq(vec![
                Expr::lit(&c.to_string()),
                Expr::r(&name_of(&q)),
            ]));
            taken.push((u32::from(c), u32::from(c)));
        }
        let others = intersect(class, &complement(&norm(taken)));
        if !others.is_empty() {
            alts.push(Expr::seq(vec![
                Expr::Class {
                    negated: false,
                    ranges: merge_ranges(&ranges(&others)),
                },
                Expr::r(&tail),
            ]));
        }
        let mut body = Expr::alt(alts);
        if p.is_empty() && !prefix.is_empty() {
            body = Expr::seq(vec![Expr::lit(prefix), body]);
        }
        let mut rule = Rule::new(&name_of(p), body);
        if p.is_empty() {
            rule = rule.doc(&format!(
                "hand-written: {prefix}a run of the name characters less {}",
                words
                    .iter()
                    .map(|w| format!("`{prefix}{w}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        } else {
            rule = rule.doc(&format!("after `{prefix}{p}`"));
        }
        rules.push(rule);
    }
    rules.push(Rule::new(
        &tail,
        Expr::star(Expr::Class {
            negated: false,
            ranges: merge_ranges(&ranges(class)),
        }),
    ));
    rules
}

/// The lexical rules a grammar of the model's flavor shares with the whole
/// grammar: a name, a variable, a string. With their helper rules.
#[derive(Clone, Debug)]
pub struct Lexical {
    /// `tok-name` and its tree.
    pub name: Vec<Rule>,
    /// `tok-var` and its tree.
    pub var: Vec<Rule>,
    /// `tok-text`.
    pub text: Rule,
    /// The name characters.
    pub name_class: Expr,
    /// The name characters and a variable's after its `?`, as code points.
    pub name_set: Vec<(u32, u32)>,
    pub var_set: Vec<(u32, u32)>,
    /// The words no name is: every word of a rule before the name rule.
    pub words: BTreeSet<String>,
    /// The words no variable is, after its `?`: `_`.
    pub var_words: BTreeSet<String>,
    /// The words that are tokens of their own, by their text.
    pub literals: BTreeSet<String>,
    /// The separator pieces: white space, a line comment, a block comment.
    pub space: Expr,
    pub comment_line: Expr,
    pub comment_block: Expr,
}

/// **Read `ein.l`'s rules into the lexical rules.**
pub fn lexical(l_src: &str) -> Result<Lexical, String> {
    let rules = lex_rules(l_src)?;
    let pos = |n: &str| {
        rules
            .iter()
            .position(|r| r.name.as_deref() == Some(n))
            .ok_or_else(|| format!("ein.l has no rule {n}"))
    };
    let (name_i, var_i, text_i) = (pos("NAME")?, pos("VAR")?, pos("TEXT")?);
    // A run: a prefix, and one or more of a class.
    let run = |e: &Expr| -> Option<(String, Set)> {
        match e {
            Expr::Plus(x) => Some((String::new(), one_char(x)?)),
            Expr::Seq(v) if v.len() == 2 => match (&v[0], &v[1]) {
                (Expr::Lit(p), Expr::Plus(x)) => Some((p.clone(), one_char(x)?)),
                _ => None,
            },
            _ => None,
        }
    };
    let (np, nclass) = run(&rules[name_i].expr).ok_or("ein.l's NAME is no run of a class")?;
    let (vp, vclass) = run(&rules[var_i].expr).ok_or("ein.l's VAR is no run of a class")?;
    if !np.is_empty() || vp != "?" {
        return Err("ein.l's NAME or VAR is not the run it was".to_string());
    }
    // The words an earlier rule takes whole, of the same length as a run.
    let excluded = |upto: usize, prefix: &str, class: &Set| -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        for r in &rules[..upto] {
            for s in finite(&r.expr).unwrap_or_default() {
                if let Some(rest) = s.strip_prefix(prefix)
                    && !rest.is_empty()
                    && rest.chars().all(|c| contains(class, u32::from(c)))
                {
                    out.insert(rest.to_string());
                }
            }
        }
        out
    };
    let name_words = excluded(name_i, "", &nclass);
    let var_words = excluded(var_i, "?", &vclass);
    // The tokens that are their own text.
    let mut literals = BTreeSet::new();
    for r in &rules {
        if let (Some(n), Expr::Lit(t)) = (&r.name, &r.expr)
            && n == t
        {
            literals.insert(n.clone());
        }
    }
    // The skip rule: a run of white space, a line comment and a block
    // comment.
    let skip = rules
        .iter()
        .find(|r| r.name.is_none())
        .ok_or("ein.l has no rule that skips")?;
    let Expr::Plus(inner) = &skip.expr else {
        return Err("ein.l's skip rule is no run".to_string());
    };
    let Expr::Alt(pieces) = &**inner else {
        return Err("ein.l's skip rule is no run of alternatives".to_string());
    };
    let starts_with = |e: &Expr, t: &str| match e {
        Expr::Seq(v) => matches!(v.first(), Some(Expr::Lit(s)) if s == t),
        _ => false,
    };
    let space = pieces
        .iter()
        .find(|e| matches!(e, Expr::Class { .. }))
        .ok_or("the skip rule has no white space")?
        .clone();
    let comment_line = pieces
        .iter()
        .find(|e| starts_with(e, ";"))
        .ok_or("the skip rule has no line comment")?
        .clone();
    let comment_block = pieces
        .iter()
        .find(|e| starts_with(e, "#|"))
        .ok_or("the skip rule has no block comment")?
        .clone();
    if pieces.len() != 3 {
        return Err("the skip rule is no longer three pieces".to_string());
    }
    let name_class = Expr::Class {
        negated: false,
        ranges: merge_ranges(&ranges(&nclass)),
    };
    Ok(Lexical {
        name: exclusion_rules("tok-name", "", &nclass, &name_words),
        var: exclusion_rules("tok-var", "?", &vclass, &var_words),
        text: Rule::new("tok-text", rules[text_i].expr.clone()).doc("ein.l: TEXT, node for node"),
        name_class,
        name_set: nclass,
        var_set: vclass,
        words: name_words,
        var_words,
        literals,
        space,
        comment_line,
        comment_block,
    })
}

/// **The lexical rules and separators of [`ROOT`]**, each named with
/// [`ROOT_PREFIX`]: a name and a variable of [`ROOT_NAME_CHARS`] less the
/// words of those characters, a string and a line comment less
/// [`super::EXCLUDED`], and the separators with no block comment, white
/// space (`space`) shared with `program`.
pub fn root_lexical(lx: &Lexical) -> Result<Vec<Rule>, String> {
    let chars: Set = norm(
        ROOT_NAME_CHARS
            .iter()
            .map(|&(a, b)| (u32::from(a), u32::from(b)))
            .collect(),
    );
    let of = |words: &BTreeSet<String>, set: &Set| -> BTreeSet<String> {
        words
            .iter()
            .filter(|w| w.chars().all(|c| contains(set, u32::from(c))))
            .cloned()
            .collect()
    };
    let (nset, vset) = (
        intersect(&lx.name_set, &chars),
        intersect(&lx.var_set, &chars),
    );
    let p = |n: &str| format!("{ROOT_PREFIX}{n}");
    let mut rules = exclusion_rules(&p("tok-name"), "", &nset, &of(&lx.words, &nset));
    rules.extend(exclusion_rules(
        &p("tok-var"),
        "?",
        &vset,
        &of(&lx.var_words, &vset),
    ));
    let excluded = &super::EXCLUDED;
    rules.push(
        Rule::new(&p("tok-text"), lx.text.body.without(excluded)?)
            .doc("ein.l: TEXT, node for node, less `<`, `>`, `|` and the backtick"),
    );
    let piece = Expr::alt(vec![Expr::r("space"), Expr::r(&p("comment-line-ended"))]);
    rules.extend([
        Rule::new(&p("comment-line"), lx.comment_line.without(excluded)?)
            .doc("ein.l: the skip rule's line comment, node for node, less `<`, `>`, `|` and the backtick"),
        Rule::new(
            &p("comment-line-ended"),
            Expr::seq(vec![Expr::r(&p("comment-line")), Expr::lit("\n")]),
        )
        .doc("hand-written: a line comment runs to the end of its line"),
        Rule::new(&p("gap"), Expr::star(piece.clone()))
            .doc("hand-written: any separator of white space and line comments, none too: after a paren, or before one"),
        Rule::new(&p("gap-word"), Expr::plus(piece))
            .doc("hand-written: between two tokens that are no paren, a separator is required"),
        Rule::new(
            &p("gap-end"),
            Expr::seq(vec![Expr::r(&p("gap")), Expr::opt(Expr::r(&p("comment-line")))]),
        )
        .doc("hand-written: the text's last separator, which may end in a line comment with no line break"),
    ]);
    Ok(rules)
}

/// The separators of Ein's lexer, hand-written over the skip rule's pieces.
fn separator_rules(lx: &Lexical) -> Vec<Rule> {
    let piece = Expr::alt(vec![
        Expr::r("space"),
        Expr::r("comment-line-ended"),
        Expr::r("comment-block"),
    ]);
    vec![
        Rule::new("space", lx.space.clone()).doc("ein.l: the skip rule's white space, node for node"),
        Rule::new("comment-line", lx.comment_line.clone())
            .doc("ein.l: the skip rule's line comment, node for node"),
        Rule::new("comment-block", lx.comment_block.clone())
            .doc("ein.l: the skip rule's block comment, node for node"),
        Rule::new(
            "comment-line-ended",
            Expr::seq(vec![Expr::r("comment-line"), Expr::lit("\n")]),
        )
        .doc("hand-written: a line comment runs to the end of its line"),
        Rule::new("gap", Expr::star(piece.clone()))
            .doc("hand-written: any separator, none too: after a paren, or before one"),
        Rule::new("gap-word", Expr::plus(piece.clone()))
            .doc("hand-written: after a word and before a word or a string, a separator is required"),
        Rule::new(
            "gap-text",
            Expr::seq(vec![
                Expr::alt(vec![Expr::r("space"), Expr::r("comment-line-ended")]),
                Expr::r("gap"),
            ]),
        )
        .doc("hand-written: after a string, a separator is required before a word or a string, and does not begin with `#`"),
        Rule::new("gap-after-text", Expr::alt(vec![Expr::r("gap-text"), Expr::Empty]))
            .doc("hand-written: after a string and before a paren, none, or a separator that does not begin with `#`"),
        Rule::new(
            "gap-end",
            Expr::seq(vec![Expr::r("gap"), Expr::opt(Expr::r("comment-line"))]),
        )
        .doc("hand-written: the text's last separator, which may end in a line comment with no line break"),
    ]
}

/// **The whole grammar's text**, from `ein.l` and `ein.y`: the two roots,
/// `ein.y`'s rules compiled, and the lexical rules.
pub fn whole(l_src: &str, y_src: &str) -> Result<String, String> {
    Ok(whole_grammar(l_src, y_src)?.text())
}

/// The whole grammar, as rules.
pub fn whole_grammar(l_src: &str, y_src: &str) -> Result<Grammar, String> {
    let lx = lexical(l_src)?;
    let ph = Phrase::read(y_src, &lx.literals)?;
    let flavor = Flavor::Exact;
    let program = ph.body("Program").ok_or("ein.y has no Program")?;
    let contract: Vec<Ar> = CONTRACT_FORMS
        .iter()
        .map(|f| {
            ph.rule_idx(f)
                .ok_or_else(|| format!("ein.y has no {f}"))
                .map(|r| ph.atoms_of(&Re::Sym(Sym::Rule(r))))
        })
        .collect::<Result<_, _>>()?;
    let contract_forms = Ar::plus(Ar::Alt(contract));
    let mut g = Grammar {
        head: vec![
            "Ein's grammar as GBNF: generated from ein.l and ein.y, and the hand-written rules"
                .to_string(),
            "that stand for what Ein's lexer does beside them. Do not edit: regenerate."
                .to_string(),
            String::new(),
            format!(
                "Two roots. `{ROOT}`: the forms a program the formalization contract admits, in the"
            ),
            "characters a model writes: a name or a variable of letters, digits and `_ . * + = -`,"
                .to_string(),
            "no block comment, and no `<`, `>`, `|` or backtick in a string or a comment; the rules"
                .to_string(),
            format!(
                "it alone reaches are named `{ROOT_PREFIX}…`. `{PROGRAM}`: every form ein.y's Program takes, as"
            ),
            "Ein's lexer and parser read it.".to_string(),
        ],
        rules: Vec::new(),
    };
    let text_rule = |name: &str, forms: &str, p: &str, doc: &str| {
        Rule::new(
            name,
            Expr::alt(vec![
                Expr::r(&format!("{p}gap-end")),
                Expr::seq(vec![
                    Expr::r(&format!("{p}gap")),
                    Expr::r(forms),
                    Expr::r(&format!("{p}gap-end")),
                ]),
            ]),
        )
        .doc(doc)
    };
    // The root's rules: ein.y's compiled again, every rule they name but
    // `space` named with the prefix.
    let prefixed = |n: &str| format!("{ROOT_PREFIX}{n}");
    g.push(text_rule(
        ROOT,
        "root-forms",
        ROOT_PREFIX,
        "hand-written: a text of the forms the contract admits, between separators, in the characters a model writes",
    ));
    for mut r in compile("root-forms", &contract_forms, &Flavor::Root, None, None) {
        r.body = r.body.renamed(&|n| {
            if n.starts_with("root-forms") {
                n.to_string()
            } else {
                prefixed(n)
            }
        });
        g.push(r);
    }
    for mut r in ph.compile_from(std::slice::from_ref(&contract_forms), &Flavor::Root) {
        r.name = prefixed(&r.name);
        r.body = r.body.renamed(&prefixed);
        g.push(r);
    }
    for r in root_lexical(&lx)? {
        g.push(r);
    }
    // The program's rules: Ein's language exactly.
    g.push(text_rule(
        PROGRAM,
        "program-forms",
        "",
        "hand-written: a text of every form ein.y's Program takes, between separators",
    ));
    for r in compile("program-forms", &program, &flavor, None, None) {
        g.push(r);
    }
    for r in ph.compile_from(std::slice::from_ref(&program), &flavor) {
        g.push(r);
    }
    for r in lx.name.iter().chain(lx.var.iter()) {
        g.push(r.clone());
    }
    g.push(lx.text.clone());
    for r in separator_rules(&lx) {
        g.push(r);
    }
    Ok(g)
}

/// **The text of the root [`ROOT`] alone**: the whole grammar's rules it
/// reaches, under a head that says so.
pub fn root_text(l_src: &str, y_src: &str) -> Result<String, String> {
    let g = whole_grammar(l_src, y_src)?;
    let head = vec![
        format!(
            "The root `{ROOT}` of Ein's grammar as GBNF (einaf/grammars/ein.gbnf), the rules it reaches:"
        ),
        "the forms the formalization contract admits, in the characters a model writes."
            .to_string(),
    ];
    Ok(g.reachable(ROOT, head).text())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::gbnf::Compiled;

    #[test]
    fn the_whole_grammar_reads_back() {
        let text = whole(EIN_L, EIN_Y).unwrap();
        let c = Compiled::parse(&text).unwrap();
        let ok = |t: &str| c.accepts(PROGRAM, t).unwrap().accepted();
        assert!(ok(""));
        assert!(ok("  ; a comment"));
        assert!(ok("(p a)"));
        assert!(ok("(p a)(q b)"));
        assert!(ok("(relation p T :why \"{?3} is p\")"));
        assert!(ok("(p (q a)b)"));
        assert!(ok("(p a #|c|# b)"));
        assert!(ok("(p nota and-1 neq-1 relations)"));
        assert!(!ok("(p and)"));
        assert!(!ok("(p a ;c)"));
        assert!(!ok("(p ab\"x\")"));
        assert!(!ok("(relation p :why \"x\"#|c|#)"));
        assert!(ok("(relation p :why \"x\" #|c|#)"));
        assert!(!ok("(query:goal (p ?x))"));
        assert!(!ok("(p _)"));
        assert!(ok("(p __closed__ _x)"));
        assert!(!ok("(p a"));
        // The root: the contract's forms in the characters a model writes.
        let root = |t: &str| c.accepts(ROOT, t).unwrap().accepted();
        assert!(root("(p a.b *x+ =y- _z)"));
        assert!(root("(relation p T :why \"{?3} is p/q\")"));
        assert!(root("(p a) ; a comment"));
        assert!(root("(query :goal-text \"x\" :goal (p ?a))"));
        assert!(!root("(query :goal-text \"x\":goal (p ?a))"));
        assert!(ok("(query :goal-text \"x\" :goal (p ?a))"));
        assert!(!root("(p a/b)"));
        assert!(!root("(p </think>)"));
        assert!(!root("(p ``` x)"));
        assert!(!root("(p _)"));
        assert!(!root("(p and)"));
        assert!(root("(p nota and-1)"));
        assert!(!root("(p a #|c|# b)"));
        assert!(!root("(p a) ; `a`"));
        assert!(!root("(relation p T :why \"{?3} is <p>\")"));
        assert!(!root("(config :record-alternative-justifications false)"));
        // The root's rules are all its own but white space.
        let g = whole_grammar(EIN_L, EIN_Y).unwrap();
        let mine = g.reachable(ROOT, Vec::new());
        assert!(mine.rules.iter().all(|r| r.name == ROOT
            || r.name == "space"
            || r.name.starts_with("root-forms")
            || r.name.starts_with(ROOT_PREFIX)));
    }
}
