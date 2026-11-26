//! **GBNF, as llama.cpp reads it.** A grammar is written from a model of
//! rules ([`Expr`], [`Rule`], [`Grammar`]), and its text is read back into
//! llama.cpp's own form ([`Compiled`]): rules of elements, each group and
//! each repetition expanded into the rules it stands for, by the algorithm of
//! llama.cpp's grammar parser. A compiled grammar is run as llama.cpp's
//! grammar sampler runs it, over characters: a set of stacks of positions,
//! each advanced to the characters it can take ([`Compiled::accepts`]). That
//! is **EinAf's acceptor**, which reads the text a generation is constrained
//! by, not the model it was written from. And a sampler that writes texts a
//! compiled grammar allows ([`Compiled::sample`]).
//!
//! The acceptor follows `llama-grammar.cpp` of the llama.cpp the binding
//! builds (`llama_grammar_advance_stack`, `llama_grammar_accept`,
//! `llama_grammar_match_char`): the same elements, the same expansion of a
//! rule reference into one stack an alternative, an empty stack where the
//! grammar may end. A stack is held as a node of a shared tree, so that a
//! stack is its node's number and two equal stacks are one.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::hash::{BuildHasherDefault, Hasher};

// ── The model a grammar is written from ─────────────────────────────

/// **An expression of GBNF.**
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    /// `""`.
    Empty,
    /// `"…"`: these characters, in order.
    Lit(String),
    /// `[…]` or `[^…]`: one character of the ranges, or of none of them.
    Class {
        negated: bool,
        ranges: Vec<(char, char)>,
    },
    /// `.`: any character.
    Any,
    /// A rule, by its name.
    Ref(String),
    Seq(Vec<Expr>),
    Alt(Vec<Expr>),
    Star(Box<Expr>),
    Plus(Box<Expr>),
    Opt(Box<Expr>),
    /// `x{m,n}`, `x{m,}`.
    Times {
        expr: Box<Expr>,
        min: u32,
        max: Option<u32>,
    },
}

impl Expr {
    pub fn lit(s: &str) -> Expr {
        if s.is_empty() {
            Expr::Empty
        } else {
            Expr::Lit(s.to_string())
        }
    }

    pub fn r(name: &str) -> Expr {
        Expr::Ref(name.to_string())
    }

    /// A class of these ranges, sorted and merged.
    pub fn class(negated: bool, ranges: &[(char, char)]) -> Expr {
        Expr::Class {
            negated,
            ranges: merge_ranges(ranges),
        }
    }

    /// A sequence, flat: a member that is a sequence gives its members, an
    /// empty member none, and a sequence of one is that one.
    pub fn seq(items: Vec<Expr>) -> Expr {
        let mut flat: Vec<Expr> = Vec::new();
        for i in items {
            match i {
                Expr::Seq(v) => flat.extend(v),
                Expr::Empty => {}
                Expr::Lit(s) => match flat.last_mut() {
                    Some(Expr::Lit(a)) => a.push_str(&s),
                    _ => flat.push(Expr::Lit(s)),
                },
                other => flat.push(other),
            }
        }
        match flat.len() {
            0 => Expr::Empty,
            1 => flat.remove(0),
            _ => Expr::Seq(flat),
        }
    }

    /// An alternation, flat, each alternative once.
    pub fn alt(items: Vec<Expr>) -> Expr {
        let mut flat: Vec<Expr> = Vec::new();
        for i in items {
            let more = match i {
                Expr::Alt(v) => v,
                other => vec![other],
            };
            for m in more {
                if !flat.contains(&m) {
                    flat.push(m);
                }
            }
        }
        match flat.len() {
            0 => panic!("an alternation of nothing"),
            1 => flat.remove(0),
            _ => Expr::Alt(flat),
        }
    }

    pub fn star(e: Expr) -> Expr {
        Expr::Star(Box::new(e))
    }

    pub fn plus(e: Expr) -> Expr {
        Expr::Plus(Box::new(e))
    }

    pub fn opt(e: Expr) -> Expr {
        Expr::Opt(Box::new(e))
    }

    pub fn times(e: Expr, min: u32, max: Option<u32>) -> Expr {
        match (min, max) {
            (0, None) => Expr::star(e),
            (1, None) => Expr::plus(e),
            (0, Some(1)) => Expr::opt(e),
            (1, Some(1)) => e,
            _ => Expr::Times {
                expr: Box::new(e),
                min,
                max,
            },
        }
    }

    /// The expression as GBNF text, on one line.
    pub fn text(&self) -> String {
        match self {
            Expr::Alt(v) => v.iter().map(Expr::seq_item).collect::<Vec<_>>().join(" | "),
            _ => self.seq_item(),
        }
    }

    /// As a member of a sequence: an alternation in parentheses.
    fn seq_item(&self) -> String {
        match self {
            Expr::Seq(v) => v.iter().map(Expr::seq_member).collect::<Vec<_>>().join(" "),
            other => other.seq_member(),
        }
    }

    fn seq_member(&self) -> String {
        match self {
            Expr::Seq(_) | Expr::Alt(_) => format!("( {} )", self.text()),
            _ => self.atom(),
        }
    }

    /// As the operand of a repetition, or anywhere: a group where it is not
    /// one symbol.
    fn atom(&self) -> String {
        match self {
            Expr::Empty => "\"\"".to_string(),
            Expr::Lit(s) => {
                let mut out = String::from("\"");
                for c in s.chars() {
                    out.push_str(&escape(c, false));
                }
                out.push('"');
                out
            }
            Expr::Class { negated, ranges } => {
                let mut out = String::from("[");
                if *negated {
                    out.push('^');
                }
                for &(a, b) in ranges {
                    out.push_str(&escape(a, true));
                    if b != a {
                        // Two neighbours are two members, not a range.
                        if u32::from(b) != u32::from(a) + 1 {
                            out.push('-');
                        }
                        out.push_str(&escape(b, true));
                    }
                }
                out.push(']');
                out
            }
            Expr::Any => ".".to_string(),
            Expr::Ref(n) => n.clone(),
            Expr::Seq(_) | Expr::Alt(_) => format!("( {} )", self.text()),
            Expr::Star(e) => format!("{}*", e.operand()),
            Expr::Plus(e) => format!("{}+", e.operand()),
            Expr::Opt(e) => format!("{}?", e.operand()),
            Expr::Times { expr, min, max } => match max {
                Some(m) if m == min => format!("{}{{{min}}}", expr.operand()),
                Some(m) => format!("{}{{{min},{m}}}", expr.operand()),
                None => format!("{}{{{min},}}", expr.operand()),
            },
        }
    }

    fn operand(&self) -> String {
        match self {
            Expr::Star(_) | Expr::Plus(_) | Expr::Opt(_) | Expr::Times { .. } => {
                format!("( {} )", self.atom())
            }
            _ => self.atom(),
        }
    }

    /// The rules the expression names, each once, in the order met.
    pub fn refs(&self, out: &mut Vec<String>) {
        match self {
            Expr::Ref(n) => {
                if !out.contains(n) {
                    out.push(n.clone());
                }
            }
            Expr::Seq(v) | Expr::Alt(v) => v.iter().for_each(|x| x.refs(out)),
            Expr::Star(x) | Expr::Plus(x) | Expr::Opt(x) | Expr::Times { expr: x, .. } => {
                x.refs(out)
            }
            Expr::Empty | Expr::Lit(_) | Expr::Class { .. } | Expr::Any => {}
        }
    }

    /// The expression with every rule it names renamed by `f`.
    pub fn renamed(&self, f: &dyn Fn(&str) -> String) -> Expr {
        let boxed = |x: &Expr| Box::new(x.renamed(f));
        match self {
            Expr::Ref(n) => Expr::Ref(f(n)),
            Expr::Seq(v) => Expr::Seq(v.iter().map(|x| x.renamed(f)).collect()),
            Expr::Alt(v) => Expr::Alt(v.iter().map(|x| x.renamed(f)).collect()),
            Expr::Star(x) => Expr::Star(boxed(x)),
            Expr::Plus(x) => Expr::Plus(boxed(x)),
            Expr::Opt(x) => Expr::Opt(boxed(x)),
            Expr::Times { expr, min, max } => Expr::Times {
                expr: boxed(expr),
                min: *min,
                max: *max,
            },
            other => other.clone(),
        }
    }

    /// **The expression with `chars` taken out of every character it
    /// allows**: a class less them, a negated class with them added, `.`
    /// every character but them. A literal that holds one of them is an
    /// error: no text it allows is left.
    pub fn without(&self, chars: &[char]) -> Result<Expr, String> {
        let out: Vec<(char, char)> = chars.iter().map(|&c| (c, c)).collect();
        Ok(match self {
            Expr::Lit(s) => {
                if let Some(c) = s.chars().find(|c| chars.contains(c)) {
                    return Err(format!("the literal {s:?} holds {c:?}"));
                }
                self.clone()
            }
            Expr::Class {
                negated: true,
                ranges,
            } => {
                let mut all = ranges.clone();
                all.extend(out);
                Expr::class(true, &all)
            }
            Expr::Class {
                negated: false,
                ranges,
            } => {
                let mut left: Vec<(char, char)> = Vec::new();
                for &(a, b) in ranges {
                    let mut lo = u32::from(a);
                    let hi = u32::from(b);
                    let mut cut: Vec<u32> = chars
                        .iter()
                        .map(|&c| u32::from(c))
                        .filter(|&c| lo <= c && c <= hi)
                        .collect();
                    cut.sort_unstable();
                    cut.push(hi + 1);
                    for c in cut {
                        if lo < c
                            && let (Some(x), Some(y)) = (char::from_u32(lo), char::from_u32(c - 1))
                        {
                            left.push((x, y));
                        }
                        lo = c + 1;
                    }
                }
                Expr::class(false, &left)
            }
            Expr::Any => Expr::class(true, &out),
            Expr::Seq(v) => Expr::Seq(
                v.iter()
                    .map(|x| x.without(chars))
                    .collect::<Result<_, _>>()?,
            ),
            Expr::Alt(v) => Expr::Alt(
                v.iter()
                    .map(|x| x.without(chars))
                    .collect::<Result<_, _>>()?,
            ),
            Expr::Star(x) => Expr::Star(Box::new(x.without(chars)?)),
            Expr::Plus(x) => Expr::Plus(Box::new(x.without(chars)?)),
            Expr::Opt(x) => Expr::Opt(Box::new(x.without(chars)?)),
            Expr::Times { expr, min, max } => Expr::Times {
                expr: Box::new(expr.without(chars)?),
                min: *min,
                max: *max,
            },
            Expr::Empty | Expr::Ref(_) => self.clone(),
        })
    }
}

/// A character as GBNF writes it in a literal or a class: the escapes
/// llama.cpp's parser reads (`\\ \" \[ \] \- \t \r \n \xHH \uHHHH
/// \UHHHHHHHH`), a printing ASCII character as itself.
fn escape(c: char, in_class: bool) -> String {
    match c {
        '\\' => "\\\\".to_string(),
        '"' => "\\\"".to_string(),
        '\n' => "\\n".to_string(),
        '\t' => "\\t".to_string(),
        '\r' => "\\r".to_string(),
        '[' | ']' if in_class => format!("\\{c}"),
        '-' if in_class => "\\-".to_string(),
        // `^` first in a class negates it; written by its code point
        // anywhere in a class, so that no place of it does.
        '^' if in_class => "\\x5E".to_string(),
        c if (' '..='~').contains(&c) => c.to_string(),
        c if (c as u32) < 0x100 => format!("\\x{:02X}", c as u32),
        c if (c as u32) < 0x10000 => format!("\\u{:04X}", c as u32),
        c => format!("\\U{:08X}", c as u32),
    }
}

/// Sorted, overlapping and touching ranges merged.
pub fn merge_ranges(ranges: &[(char, char)]) -> Vec<(char, char)> {
    let mut v: Vec<(u32, u32)> = ranges
        .iter()
        .map(|&(a, b)| (u32::from(a), u32::from(b)))
        .collect();
    v.sort();
    let mut out: Vec<(u32, u32)> = Vec::new();
    for (a, b) in v {
        match out.last_mut() {
            Some(last) if a <= last.1.saturating_add(1) => last.1 = last.1.max(b),
            _ => out.push((a, b)),
        }
    }
    out.into_iter()
        .filter_map(|(a, b)| Some((char::from_u32(a)?, char::from_u32(b)?)))
        .collect()
}

/// **A rule**: its name (letters, digits and `-`, as llama.cpp's parser
/// reads one), its body, and the comment lines written above it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    pub name: String,
    pub body: Expr,
    pub doc: Vec<String>,
}

impl Rule {
    pub fn new(name: &str, body: Expr) -> Rule {
        assert!(
            !name.is_empty() && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-'),
            "a GBNF rule's name is letters, digits and `-`: {name:?}"
        );
        Rule {
            name: name.to_string(),
            body,
            doc: Vec::new(),
        }
    }

    pub fn doc(mut self, line: &str) -> Rule {
        self.doc.push(line.to_string());
        self
    }
}

/// **A grammar**: its rules in the order written, and the comment lines
/// at its head.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Grammar {
    pub head: Vec<String>,
    pub rules: Vec<Rule>,
}

impl Grammar {
    pub fn push(&mut self, rule: Rule) {
        assert!(
            !self.rules.iter().any(|r| r.name == rule.name),
            "the rule {} is written twice",
            rule.name
        );
        self.rules.push(rule);
    }

    pub fn has(&self, name: &str) -> bool {
        self.rules.iter().any(|r| r.name == name)
    }

    /// **The grammar of the rules `root` reaches**, in the order written,
    /// under `head`.
    pub fn reachable(&self, root: &str, head: Vec<String>) -> Grammar {
        let mut seen: Vec<String> = vec![root.to_string()];
        let mut i = 0;
        while i < seen.len() {
            if let Some(r) = self.rules.iter().find(|r| r.name == seen[i]) {
                r.body.refs(&mut seen);
            }
            i += 1;
        }
        Grammar {
            head,
            rules: self
                .rules
                .iter()
                .filter(|r| seen.contains(&r.name))
                .cloned()
                .collect(),
        }
    }

    /// **The grammar as GBNF text**: its head's comments, then each rule
    /// under its own, its alternatives one a line, each line but the last
    /// ending in `|`, which is where llama.cpp's parser lets a rule go on.
    pub fn text(&self) -> String {
        let mut out = String::new();
        for l in &self.head {
            writeln!(out, "#{}{l}", if l.is_empty() { "" } else { " " }).unwrap();
        }
        for r in &self.rules {
            out.push('\n');
            for l in &r.doc {
                writeln!(out, "#{}{l}", if l.is_empty() { "" } else { " " }).unwrap();
            }
            let alts: Vec<String> = match &r.body {
                Expr::Alt(v) => v.iter().map(Expr::seq_item).collect(),
                other => vec![other.seq_item()],
            };
            let pad = " ".repeat(r.name.len() + 3);
            write!(out, "{} ::= ", r.name).unwrap();
            for (i, a) in alts.iter().enumerate() {
                if i > 0 {
                    write!(out, " |\n{pad}").unwrap();
                }
                out.push_str(a);
            }
            out.push('\n');
        }
        out
    }
}

// ── The grammar as llama.cpp reads it ───────────────────────────────

/// One element of a compiled rule, llama.cpp's `llama_gretype`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Elem {
    End,
    Alt,
    RuleRef(u32),
    Char(u32),
    CharNot(u32),
    CharRngUpper(u32),
    CharAlt(u32),
    CharAny,
}

impl Elem {
    fn is_end(self) -> bool {
        matches!(self, Elem::End | Elem::Alt)
    }
}

/// llama.cpp's limit on what a repetition may expand to.
const MAX_REPETITION_THRESHOLD: u64 = 2000;

/// **A grammar's text as llama.cpp's parser reads it**: its rules of
/// elements, every element of every rule in one array, each rule starting
/// at its offset.
#[derive(Clone, Debug)]
pub struct Compiled {
    elems: Vec<Elem>,
    starts: Vec<u32>,
    names: Vec<String>,
    by_name: HashMap<String, u32>,
    /// A rule's least height of a derivation, for the sampler.
    height: Vec<u32>,
}

/// The parser's state, llama.cpp's `llama_grammar_parser`.
struct Parser<'a> {
    src: &'a [u8],
    symbol_ids: HashMap<String, u32>,
    names: Vec<String>,
    rules: Vec<Vec<Elem>>,
}

fn is_word_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'-'
}

impl<'a> Parser<'a> {
    fn at(&self, pos: usize) -> u8 {
        self.src.get(pos).copied().unwrap_or(0)
    }

    fn get_symbol_id(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.symbol_ids.get(name) {
            return id;
        }
        let id = self.symbol_ids.len() as u32;
        self.symbol_ids.insert(name.to_string(), id);
        self.names.push(name.to_string());
        id
    }

    fn generate_symbol_id(&mut self, base: &str) -> u32 {
        let id = self.symbol_ids.len() as u32;
        let name = format!("{base}_{id}");
        self.symbol_ids.insert(name.clone(), id);
        self.names.push(name);
        id
    }

    fn add_rule(&mut self, id: u32, rule: Vec<Elem>) {
        let id = id as usize;
        if self.rules.len() <= id {
            self.rules.resize(id + 1, Vec::new());
        }
        self.rules[id] = rule;
    }

    fn parse_space(&self, mut pos: usize, newline_ok: bool) -> usize {
        loop {
            let c = self.at(pos);
            if c == b' ' || c == b'\t' || (newline_ok && (c == b'\r' || c == b'\n')) {
                pos += 1;
            } else if c == b'#' {
                while pos < self.src.len() && self.at(pos) != b'\r' && self.at(pos) != b'\n' {
                    pos += 1;
                }
            } else {
                return pos;
            }
        }
    }

    fn parse_name(&self, pos: usize) -> Result<usize, String> {
        let mut end = pos;
        while is_word_char(self.at(end)) {
            end += 1;
        }
        if end == pos {
            return Err(format!("expecting name at byte {pos}"));
        }
        Ok(end)
    }

    fn parse_hex(&self, pos: usize, size: usize) -> Result<(u32, usize), String> {
        let mut value: u32 = 0;
        for i in 0..size {
            let c = self.at(pos + i);
            let d = (c as char)
                .to_digit(16)
                .ok_or_else(|| format!("expecting {size} hex chars at byte {pos}"))?;
            value = (value << 4) + d;
        }
        Ok((value, pos + size))
    }

    /// One character of a literal or a class, with its escapes.
    fn parse_char(&self, pos: usize) -> Result<(u32, usize), String> {
        if self.at(pos) == b'\\' {
            return match self.at(pos + 1) {
                b'x' => self.parse_hex(pos + 2, 2),
                b'u' => self.parse_hex(pos + 2, 4),
                b'U' => self.parse_hex(pos + 2, 8),
                b't' => Ok((u32::from('\t'), pos + 2)),
                b'r' => Ok((u32::from('\r'), pos + 2)),
                b'n' => Ok((u32::from('\n'), pos + 2)),
                c @ (b'\\' | b'"' | b'[' | b']' | b'-') => Ok((u32::from(c), pos + 2)),
                _ => Err(format!("unknown escape at byte {pos}")),
            };
        }
        if pos >= self.src.len() {
            return Err("unexpected end of input".to_string());
        }
        let rest = std::str::from_utf8(&self.src[pos..])
            .or_else(|e| std::str::from_utf8(&self.src[pos..pos + e.valid_up_to()]))
            .map_err(|_| format!("no UTF-8 at byte {pos}"))?;
        let c = rest
            .chars()
            .next()
            .ok_or_else(|| format!("no UTF-8 at byte {pos}"))?;
        Ok((u32::from(c), pos + c.len_utf8()))
    }

    fn parse_alternates(
        &mut self,
        pos: usize,
        rule_name: &str,
        rule_id: u32,
        nested: bool,
    ) -> Result<usize, String> {
        let mut rule = Vec::new();
        let mut pos = self.parse_sequence(pos, rule_name, &mut rule, nested)?;
        while self.at(pos) == b'|' {
            rule.push(Elem::Alt);
            pos = self.parse_space(pos + 1, true);
            pos = self.parse_sequence(pos, rule_name, &mut rule, nested)?;
        }
        rule.push(Elem::End);
        self.add_rule(rule_id, rule);
        Ok(pos)
    }

    fn parse_sequence(
        &mut self,
        src: usize,
        rule_name: &str,
        rule: &mut Vec<Elem>,
        nested: bool,
    ) -> Result<usize, String> {
        let mut last_sym_start = rule.len();
        let mut pos = src;
        let mut n_prev_rules: u64 = 1;
        while pos < self.src.len() {
            let c = self.at(pos);
            if c == b'"' {
                pos += 1;
                last_sym_start = rule.len();
                n_prev_rules = 1;
                while self.at(pos) != b'"' {
                    if pos >= self.src.len() {
                        return Err("unexpected end of input".to_string());
                    }
                    let (ch, next) = self.parse_char(pos)?;
                    pos = next;
                    rule.push(Elem::Char(ch));
                }
                pos = self.parse_space(pos + 1, nested);
            } else if c == b'[' {
                pos += 1;
                let mut negated = false;
                if self.at(pos) == b'^' {
                    pos += 1;
                    negated = true;
                }
                last_sym_start = rule.len();
                n_prev_rules = 1;
                while self.at(pos) != b']' {
                    if pos >= self.src.len() {
                        return Err("unexpected end of input".to_string());
                    }
                    let (ch, next) = self.parse_char(pos)?;
                    pos = next;
                    rule.push(if last_sym_start < rule.len() {
                        Elem::CharAlt(ch)
                    } else if negated {
                        Elem::CharNot(ch)
                    } else {
                        Elem::Char(ch)
                    });
                    if self.at(pos) == b'-' && self.at(pos + 1) != b']' {
                        if pos + 1 >= self.src.len() {
                            return Err("unexpected end of input".to_string());
                        }
                        let (end, next) = self.parse_char(pos + 1)?;
                        pos = next;
                        rule.push(Elem::CharRngUpper(end));
                    }
                }
                pos = self.parse_space(pos + 1, nested);
            } else if c == b'<' || c == b'!' {
                return Err(format!(
                    "a token reference at byte {pos}: EinAf's reader takes characters only"
                ));
            } else if is_word_char(c) {
                let end = self.parse_name(pos)?;
                let name = std::str::from_utf8(&self.src[pos..end]).expect("ASCII");
                let id = self.get_symbol_id(name);
                pos = self.parse_space(end, nested);
                last_sym_start = rule.len();
                n_prev_rules = 1;
                rule.push(Elem::RuleRef(id));
            } else if c == b'(' {
                pos = self.parse_space(pos + 1, true);
                let n_before = self.symbol_ids.len() as u64;
                let sub = self.generate_symbol_id(rule_name);
                pos = self.parse_alternates(pos, rule_name, sub, true)?;
                n_prev_rules = (self.symbol_ids.len() as u64 - n_before).max(1);
                last_sym_start = rule.len();
                rule.push(Elem::RuleRef(sub));
                if self.at(pos) != b')' {
                    return Err(format!("expecting ')' at byte {pos}"));
                }
                pos = self.parse_space(pos + 1, nested);
            } else if c == b'.' {
                last_sym_start = rule.len();
                n_prev_rules = 1;
                rule.push(Elem::CharAny);
                pos = self.parse_space(pos + 1, nested);
            } else if c == b'*' || c == b'+' || c == b'?' {
                pos = self.parse_space(pos + 1, nested);
                let (min, max) = match c {
                    b'*' => (0, None),
                    b'+' => (1, None),
                    _ => (0, Some(1)),
                };
                self.repeat(
                    rule_name,
                    rule,
                    last_sym_start,
                    &mut n_prev_rules,
                    min,
                    max,
                    pos,
                )?;
            } else if c == b'{' {
                pos = self.parse_space(pos + 1, nested);
                let int = |p: &Parser, at: usize| -> Result<(u64, usize), String> {
                    let mut end = at;
                    while p.at(end).is_ascii_digit() {
                        end += 1;
                    }
                    if end == at {
                        return Err(format!("expecting an int at byte {at}"));
                    }
                    let s = std::str::from_utf8(&p.src[at..end]).expect("ASCII");
                    Ok((s.parse().map_err(|_| "an int too large".to_string())?, end))
                };
                let (min, end) = int(self, pos)?;
                pos = self.parse_space(end, nested);
                let max;
                if self.at(pos) == b'}' {
                    max = Some(min);
                    pos = self.parse_space(pos + 1, nested);
                } else if self.at(pos) == b',' {
                    pos = self.parse_space(pos + 1, nested);
                    let mut m = None;
                    if self.at(pos).is_ascii_digit() {
                        let (n, end) = int(self, pos)?;
                        m = Some(n);
                        pos = self.parse_space(end, nested);
                    }
                    if self.at(pos) != b'}' {
                        return Err(format!("expecting '}}' at byte {pos}"));
                    }
                    max = m;
                    pos = self.parse_space(pos + 1, nested);
                } else {
                    return Err(format!("expecting ',' at byte {pos}"));
                }
                if min > MAX_REPETITION_THRESHOLD {
                    return Err("number of repetitions exceeds sane defaults".to_string());
                }
                let max = max.filter(|&m| m <= MAX_REPETITION_THRESHOLD);
                self.repeat(
                    rule_name,
                    rule,
                    last_sym_start,
                    &mut n_prev_rules,
                    min,
                    max,
                    pos,
                )?;
            } else {
                break;
            }
        }
        Ok(pos)
    }

    /// llama.cpp's `handle_repetitions`: `S{m,n}` is `S` m times and then
    /// `n − m` nested optional rules, `S{m,}` `S` m times and a rule
    /// `S' ::= S S' |`.
    #[allow(clippy::too_many_arguments)]
    fn repeat(
        &mut self,
        rule_name: &str,
        rule: &mut Vec<Elem>,
        last_sym_start: usize,
        n_prev_rules: &mut u64,
        min: u64,
        max: Option<u64>,
        pos: usize,
    ) -> Result<(), String> {
        if last_sym_start == rule.len() {
            return Err(format!(
                "expecting preceding item to */+/?/{{ at byte {pos}"
            ));
        }
        let prev: Vec<Elem> = rule[last_sym_start..].to_vec();
        let total_rules = match max {
            Some(m) if m > 0 => m,
            _ if min > 0 => min,
            _ => 1,
        };
        if *n_prev_rules * total_rules > MAX_REPETITION_THRESHOLD {
            return Err("a repetition expands past llama.cpp's limit".to_string());
        }
        if min == 0 {
            rule.truncate(last_sym_start);
        } else {
            for _ in 1..min {
                rule.extend_from_slice(&prev);
            }
        }
        let n_opt = match max {
            None => 1,
            Some(m) => m - min,
        };
        let mut last_rec = 0;
        for i in 0..n_opt {
            let mut rec = prev.clone();
            let id = self.generate_symbol_id(rule_name);
            if i > 0 || max.is_none() {
                rec.push(Elem::RuleRef(if max.is_none() { id } else { last_rec }));
            }
            rec.push(Elem::Alt);
            rec.push(Elem::End);
            self.add_rule(id, rec);
            last_rec = id;
        }
        if n_opt > 0 {
            rule.push(Elem::RuleRef(last_rec));
        }
        *n_prev_rules *= total_rules;
        Ok(())
    }

    fn parse_rule(&mut self, src: usize) -> Result<usize, String> {
        let name_end = self.parse_name(src)?;
        let pos = self.parse_space(name_end, false);
        let name = std::str::from_utf8(&self.src[src..name_end])
            .expect("ASCII")
            .to_string();
        let id = self.get_symbol_id(&name);
        if !self.src[pos..].starts_with(b"::=") {
            return Err(format!("expecting ::= at byte {pos}"));
        }
        let pos = self.parse_space(pos + 3, true);
        let mut pos = self.parse_alternates(pos, &name, id, false)?;
        match self.at(pos) {
            b'\r' => pos += if self.at(pos + 1) == b'\n' { 2 } else { 1 },
            b'\n' => pos += 1,
            0 if pos >= self.src.len() => {}
            _ => return Err(format!("expecting newline or end at byte {pos}")),
        }
        Ok(self.parse_space(pos, true))
    }
}

/// llama.cpp's `llama_grammar_detect_left_recursion`.
fn left_recursive(
    rules: &[Vec<Elem>],
    i: usize,
    visited: &mut [bool],
    in_progress: &mut [bool],
    may_be_empty: &mut [bool],
) -> bool {
    if in_progress[i] {
        return true;
    }
    in_progress[i] = true;
    let rule = &rules[i];
    let mut at_start = true;
    for e in rule {
        if e.is_end() {
            if at_start {
                may_be_empty[i] = true;
                break;
            }
            at_start = true;
        } else {
            at_start = false;
        }
    }
    let mut recurse = true;
    for e in rule {
        match *e {
            Elem::RuleRef(r) if recurse => {
                if left_recursive(rules, r as usize, visited, in_progress, may_be_empty) {
                    return true;
                }
                if !may_be_empty[r as usize] {
                    recurse = false;
                }
            }
            e if e.is_end() => recurse = true,
            _ => recurse = false,
        }
    }
    in_progress[i] = false;
    visited[i] = true;
    false
}

/// **What the acceptor says of a text.**
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// The grammar takes the whole text, and may end after it.
    Accepted,
    /// The grammar takes no stack past the character at byte `at`, line
    /// `line` and column `column` (both from 1); or, where `at` is the
    /// text's length, the text ends where the grammar cannot.
    Refused {
        at: usize,
        line: usize,
        column: usize,
    },
}

impl Verdict {
    pub fn accepted(&self) -> bool {
        matches!(self, Verdict::Accepted)
    }
}

/// A hasher for the stacks' small keys.
#[derive(Default)]
struct Fx(u64);

impl Hasher for Fx {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(5) ^ u64::from(b)).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
        }
    }
    fn write_u32(&mut self, n: u32) {
        self.0 = (self.0.rotate_left(5) ^ u64::from(n)).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
    }
    fn write_u64(&mut self, n: u64) {
        self.0 = (self.0.rotate_left(5) ^ n).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
    }
    fn write_usize(&mut self, n: usize) {
        self.write_u64(n as u64);
    }
}

type FxMap<K, V> = HashMap<K, V, BuildHasherDefault<Fx>>;
type FxSet<K> = HashSet<K, BuildHasherDefault<Fx>>;

/// The empty stack: the grammar may end here.
const NIL: u32 = u32::MAX;

/// **Stacks, each a node of one tree**: a node is a position and the stack
/// under it, made once, so that a stack is its node's number.
#[derive(Default)]
struct Stacks {
    nodes: Vec<(u32, u32)>,
    index: FxMap<(u32, u32), u32>,
}

impl Stacks {
    fn push(&mut self, under: u32, pos: u32) -> u32 {
        if let Some(&n) = self.index.get(&(pos, under)) {
            return n;
        }
        let n = self.nodes.len() as u32;
        self.nodes.push((pos, under));
        self.index.insert((pos, under), n);
        n
    }
}

/// The most stacks the acceptor holds before it gives up.
const MAX_STACKS: usize = 100_000;

impl Compiled {
    /// **Read a grammar's text** as llama.cpp's parser reads it, and refuse
    /// what its sampler refuses: a rule never defined, and left recursion.
    pub fn parse(text: &str) -> Result<Compiled, String> {
        let mut p = Parser {
            src: text.as_bytes(),
            symbol_ids: HashMap::new(),
            names: Vec::new(),
            rules: Vec::new(),
        };
        let mut pos = p.parse_space(0, true);
        while pos < p.src.len() {
            pos = p.parse_rule(pos)?;
        }
        if p.rules.len() < p.names.len() {
            p.rules.resize(p.names.len(), Vec::new());
        }
        for (i, r) in p.rules.iter().enumerate() {
            if r.is_empty() {
                return Err(format!("undefined rule {}", p.names[i]));
            }
        }
        let n = p.rules.len();
        let (mut visited, mut in_progress, mut empty) =
            (vec![false; n], vec![false; n], vec![false; n]);
        for i in 0..n {
            if visited[i] {
                continue;
            }
            if left_recursive(&p.rules, i, &mut visited, &mut in_progress, &mut empty) {
                return Err(format!("left recursion at the rule {}", p.names[i]));
            }
        }
        let mut elems = Vec::new();
        let mut starts = Vec::with_capacity(n);
        for r in &p.rules {
            starts.push(elems.len() as u32);
            elems.extend_from_slice(r);
        }
        let by_name: HashMap<String, u32> = p.symbol_ids;
        let mut c = Compiled {
            elems,
            starts,
            names: p.names,
            by_name,
            height: Vec::new(),
        };
        c.height = c.heights();
        Ok(c)
    }

    /// The rules llama.cpp holds, the ones each group and repetition
    /// expands to among them.
    pub fn rules(&self) -> usize {
        self.starts.len()
    }

    /// Each rule's name, as llama.cpp's parser names it: a group's and a
    /// repetition's `NAME_N`, after the rule it is written in.
    pub fn rule_names(&self) -> &[String] {
        &self.names
    }

    /// The elements of every rule.
    pub fn elements(&self) -> usize {
        self.elems.len()
    }

    pub fn has_rule(&self, name: &str) -> bool {
        self.by_name.contains_key(name)
    }

    fn is_end(&self, pos: u32) -> bool {
        self.elems[pos as usize].is_end()
    }

    /// The start of each alternative of a rule.
    fn alternatives(&self, rule: u32) -> Vec<u32> {
        let mut out = Vec::new();
        let mut a = self.starts[rule as usize];
        loop {
            out.push(a);
            while !self.is_end(a) {
                a += 1;
            }
            if self.elems[a as usize] == Elem::Alt {
                a += 1;
            } else {
                return out;
            }
        }
    }

    /// llama.cpp's `llama_grammar_match_char`: whether `c` is in the class
    /// at `pos`, and the position after the class.
    fn match_char(&self, mut pos: u32, c: u32) -> (bool, u32) {
        let first = self.elems[pos as usize];
        let positive = matches!(first, Elem::Char(_) | Elem::CharAny);
        let mut found = false;
        loop {
            let e = self.elems[pos as usize];
            let next = self.elems[pos as usize + 1];
            let v = match e {
                Elem::Char(v) | Elem::CharNot(v) | Elem::CharAlt(v) => Some(v),
                _ => None,
            };
            if let (Elem::CharRngUpper(hi), Some(lo)) = (next, v) {
                found = found || (lo <= c && c <= hi);
                pos += 2;
            } else if e == Elem::CharAny {
                found = true;
                pos += 1;
            } else {
                found = found || v == Some(c);
                pos += 1;
            }
            if !matches!(self.elems[pos as usize], Elem::CharAlt(_)) {
                break;
            }
        }
        (found == positive, pos)
    }

    /// llama.cpp's `llama_grammar_advance_stack`: the stacks `s` stands for,
    /// each with a character's element on top, or the empty stack.
    fn advance(&self, st: &mut Stacks, s: u32, out: &mut Vec<u32>, out_set: &mut FxSet<u32>) {
        let mut todo = vec![s];
        let mut seen: FxSet<u32> = FxSet::default();
        while let Some(s) = todo.pop() {
            if !seen.insert(s) {
                continue;
            }
            if s == NIL {
                if out_set.insert(NIL) {
                    out.push(NIL);
                }
                continue;
            }
            let (pos, under) = st.nodes[s as usize];
            match self.elems[pos as usize] {
                Elem::RuleRef(r) => {
                    let base = if !self.is_end(pos + 1) {
                        st.push(under, pos + 1)
                    } else {
                        under
                    };
                    for a in self.alternatives(r) {
                        let ns = if !self.is_end(a) {
                            st.push(base, a)
                        } else {
                            base
                        };
                        todo.push(ns);
                    }
                }
                Elem::Char(_) | Elem::CharNot(_) | Elem::CharAny => {
                    if out_set.insert(s) {
                        out.push(s);
                    }
                }
                e => unreachable!("a stack is never left on {e:?}"),
            }
        }
    }

    /// The stacks a rule begins with, as `llama_grammar_init_impl` makes them.
    fn begin(&self, st: &mut Stacks, root: u32) -> (Vec<u32>, FxSet<u32>) {
        let mut out = Vec::new();
        let mut set = FxSet::default();
        for a in self.alternatives(root) {
            let s = if !self.is_end(a) {
                st.push(NIL, a)
            } else {
                NIL
            };
            self.advance(st, s, &mut out, &mut set);
        }
        (out, set)
    }

    /// **Does the grammar, from `root`, take the whole of `text`?** Each
    /// character is accepted as `llama_grammar_accept` accepts it; at the end
    /// the text is taken where an empty stack is left. The refusal is the
    /// first character no stack takes, or the end.
    pub fn accepts(&self, root: &str, text: &str) -> Result<Verdict, String> {
        let root = *self
            .by_name
            .get(root)
            .ok_or_else(|| format!("no rule {root}"))?;
        let mut st = Stacks::default();
        let (mut cur, _) = self.begin(&mut st, root);
        let (mut line, mut column) = (1, 1);
        for (at, c) in text.char_indices() {
            let mut next = Vec::with_capacity(cur.len());
            let mut set = FxSet::default();
            for &s in &cur {
                if s == NIL {
                    continue;
                }
                let (pos, under) = st.nodes[s as usize];
                let (ok, after) = self.match_char(pos, u32::from(c));
                if ok {
                    let ns = if !self.is_end(after) {
                        st.push(under, after)
                    } else {
                        under
                    };
                    self.advance(&mut st, ns, &mut next, &mut set);
                }
            }
            if next.is_empty() {
                return Ok(Verdict::Refused { at, line, column });
            }
            if next.len() > MAX_STACKS {
                return Err(format!(
                    "more than {MAX_STACKS} stacks at line {line}, column {column}"
                ));
            }
            cur = next;
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        if cur.contains(&NIL) {
            Ok(Verdict::Accepted)
        } else {
            Ok(Verdict::Refused {
                at: text.len(),
                line,
                column,
            })
        }
    }

    /// The least height of a derivation of each rule: one more than the
    /// highest rule its least alternative names.
    fn heights(&self) -> Vec<u32> {
        let n = self.starts.len();
        let mut h = vec![u32::MAX; n];
        loop {
            let mut changed = false;
            for r in 0..n {
                for a in self.alternatives(r as u32) {
                    let mut need = 0u32;
                    let mut p = a;
                    while !self.is_end(p) {
                        if let Elem::RuleRef(x) = self.elems[p as usize] {
                            need = need.max(h[x as usize]);
                        }
                        p += 1;
                    }
                    let v = need.saturating_add(1);
                    if v < h[r] {
                        h[r] = v;
                        changed = true;
                    }
                }
            }
            if !changed {
                return h;
            }
        }
    }

    /// The height of an alternative.
    fn alt_height(&self, a: u32) -> u32 {
        let mut need = 0u32;
        let mut p = a;
        while !self.is_end(p) {
            if let Elem::RuleRef(x) = self.elems[p as usize] {
                need = need.max(self.height[x as usize]);
            }
            p += 1;
        }
        need.saturating_add(1)
    }

    /// **A text the grammar allows, from `root`**, written by a walk of its
    /// rules from the seed's stream. An alternative is chosen at random, one
    /// that begins with a reference to a rule `favour` names weighing three
    /// times the rule's other alternatives together; past `depth` rules deep (a
    /// rule named last in an alternative is no deeper than the rule that
    /// names it) or `size` characters long, the least alternative is taken. A
    /// character of a class is chosen at random from its ranges, or from a
    /// pool of characters a negated class does not exclude.
    pub fn sample(
        &self,
        root: &str,
        rng: &mut Rng,
        depth: u32,
        size: usize,
        favour: &[&str],
    ) -> Result<String, String> {
        let root = *self
            .by_name
            .get(root)
            .ok_or_else(|| format!("no rule {root}"))?;
        if self.height[root as usize] == u32::MAX {
            return Err("the root derives nothing".to_string());
        }
        let favoured: Vec<u32> = favour
            .iter()
            .filter_map(|n| self.by_name.get(*n).copied())
            .collect();
        let mut out = String::new();
        self.walk(root, 0, depth, size, &favoured, rng, &mut out);
        Ok(out)
    }

    #[allow(clippy::too_many_arguments)]
    fn walk(
        &self,
        rule: u32,
        d: u32,
        depth: u32,
        size: usize,
        favoured: &[u32],
        rng: &mut Rng,
        out: &mut String,
    ) {
        let alts = self.alternatives(rule);
        let a = if d >= depth || out.len() >= size {
            *alts
                .iter()
                .min_by_key(|&&a| self.alt_height(a))
                .expect("a rule has an alternative")
        } else {
            let ok: Vec<u32> = alts
                .iter()
                .copied()
                .filter(|&a| self.alt_height(a) != u32::MAX)
                .collect();
            let favourite = |a: u32| matches!(self.elems[a as usize], Elem::RuleRef(r) if favoured.contains(&r));
            let others = ok.iter().filter(|&&a| !favourite(a)).count().max(1);
            let weights: Vec<usize> = ok
                .iter()
                .map(|&a| if favourite(a) { 3 * others } else { 1 })
                .collect();
            let mut pick = rng.below(weights.iter().sum());
            let mut chosen = ok[0];
            for (&a, &w) in ok.iter().zip(&weights) {
                if pick < w {
                    chosen = a;
                    break;
                }
                pick -= w;
            }
            chosen
        };
        let mut p = a;
        while !self.is_end(p) {
            match self.elems[p as usize] {
                Elem::RuleRef(r) => {
                    let tail = self.is_end(p + 1);
                    let next = if tail { d } else { d + 1 };
                    self.walk(r, next, depth, size, favoured, rng, out);
                    p += 1;
                }
                Elem::Char(_) | Elem::CharNot(_) | Elem::CharAny => {
                    let (c, after) = self.pick(p, rng);
                    out.push(c);
                    p = after;
                }
                e => unreachable!("a sequence holds no {e:?}"),
            }
        }
    }

    /// A character the class at `pos` takes, and the position after it.
    fn pick(&self, pos: u32, rng: &mut Rng) -> (char, u32) {
        let mut ranges = Vec::new();
        let mut p = pos;
        let mut any = false;
        loop {
            let e = self.elems[p as usize];
            let next = self.elems[p as usize + 1];
            let v = match e {
                Elem::Char(v) | Elem::CharNot(v) | Elem::CharAlt(v) => Some(v),
                _ => None,
            };
            if let (Elem::CharRngUpper(hi), Some(lo)) = (next, v) {
                ranges.push((lo, hi));
                p += 2;
            } else if e == Elem::CharAny {
                any = true;
                p += 1;
            } else {
                ranges.push((v.expect("a character"), v.expect("a character")));
                p += 1;
            }
            if !matches!(self.elems[p as usize], Elem::CharAlt(_)) {
                break;
            }
        }
        let positive = matches!(self.elems[pos as usize], Elem::Char(_));
        if positive && !any {
            let (lo, hi) = ranges[rng.below(ranges.len())];
            // A wide range: a character of its ASCII part, or its ends.
            let c = if hi - lo > 200 {
                match rng.below(3) {
                    0 => lo,
                    1 => hi,
                    _ => lo + rng.below(((hi.min(0x7e)).saturating_sub(lo) + 1) as usize) as u32,
                }
            } else {
                lo + rng.below((hi - lo + 1) as usize) as u32
            };
            return (char::from_u32(c).unwrap_or('a'), p);
        }
        // A negated class, or `.`: from the pool, what the class allows.
        let ok: Vec<char> = POOL
            .iter()
            .copied()
            .filter(|&c| self.match_char(pos, u32::from(c)).0)
            .collect();
        let c = if ok.is_empty() {
            'a'
        } else {
            ok[rng.below(ok.len())]
        };
        (c, p)
    }
}

/// The characters a negated class is sampled from: letters and digits,
/// the punctuation Ein reads, white space, and a few characters past ASCII.
const POOL: &[char] = &[
    'a',
    'b',
    'Z',
    'q',
    '0',
    '7',
    ' ',
    ' ',
    '\n',
    '\t',
    '.',
    ',',
    '-',
    '_',
    '?',
    ':',
    ';',
    '#',
    '|',
    '(',
    ')',
    '{',
    '}',
    '"',
    '\\',
    '\'',
    '/',
    '<',
    '>',
    '!',
    '*',
    'é',
    'ж',
    '—',
    '“',
    '€',
    '\u{a0}',
    '\u{1F600}',
];

/// **A stream of numbers from a seed**: SplitMix64, so that a seed is the
/// same texts on every machine.
#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A number below `n`, `n` at least 1.
    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n.max(1) as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn accepts(g: &str, root: &str, text: &str) -> bool {
        Compiled::parse(g)
            .unwrap()
            .accepts(root, text)
            .unwrap()
            .accepted()
    }

    #[test]
    fn a_grammar_reads_as_llama_cpp_reads_it() {
        let g = "root ::= \"a\" b* [x-z]+ ( \"q\" | [^q\\n] )?\nb ::= \"b\"\n";
        assert!(accepts(g, "root", "abbxyz"));
        assert!(accepts(g, "root", "axq"));
        assert!(accepts(g, "root", "ax!"));
        assert!(!accepts(g, "root", "ax\n"));
        assert!(!accepts(g, "root", "ab"));
        assert!(accepts("root ::= \"a\"{2,3}\n", "root", "aaa"));
        assert!(!accepts("root ::= \"a\"{2,3}\n", "root", "aaaa"));
        assert!(!accepts("root ::= \"a\"{2,3}\n", "root", "a"));
    }

    #[test]
    fn left_recursion_and_an_undefined_rule_are_refused() {
        assert!(Compiled::parse("root ::= root \"a\" | \"a\"\n").is_err());
        assert!(Compiled::parse("root ::= x\n").is_err());
    }

    #[test]
    fn the_printer_writes_what_the_reader_reads() {
        let mut g = Grammar::default();
        g.push(Rule::new(
            "root",
            Expr::alt(vec![
                Expr::seq(vec![Expr::lit("x\"\\"), Expr::star(Expr::r("c"))]),
                Expr::class(
                    true,
                    &[('^', '^'), ('-', '-'), (']', ']'), ('\u{e9}', '\u{e9}')],
                ),
            ]),
        ));
        g.push(Rule::new(
            "c",
            Expr::class(false, &[('a', 'c'), ('[', '[')]),
        ));
        let c = Compiled::parse(&g.text()).unwrap();
        for (t, ok) in [("x\"\\ab[", true), ("q", true), ("^", false), ("é", false)] {
            assert_eq!(c.accepts("root", t).unwrap().accepted(), ok, "{t}");
        }
    }

    #[test]
    fn a_sample_is_accepted() {
        let g = "root ::= ( \"(\" item* \")\" )+\nitem ::= [a-z]+ \" \" | root \" \"\n";
        let c = Compiled::parse(g).unwrap();
        let mut rng = Rng::new(7);
        for _ in 0..50 {
            let t = c.sample("root", &mut rng, 12, 400, &[]).unwrap();
            assert!(c.accepts("root", &t).unwrap().accepted(), "{t}");
        }
    }
}
