//! **The answer's formatters**: a value of a row of the answer query, as Ein
//! writes it, read back as a term and written as text a reader of the
//! answer reads, one kind of value a formatter; and the answer query's
//! `:goal-text` filled again with the formatted values, by the kernel's own
//! rule.
//!
//! **Written to be M1p's starting point**: a formatter is a kind of value's
//! and nothing else's ([`decimal`], the decimal numeral; [`facts`], a fact
//! that stands as a value, by cases that are data, one file a theory); it
//! reads the term alone; its cases are data beside it and its tests are
//! beside it. The module names nothing of EinAf but itself, and nothing of
//! Ein's crates: it is built alone, by `rustc` on this directory, in
//! EinAf's tests (`the_formatters_module_builds_alone`). What a value is
//! and how it is written are EinAf's defaults of an application; nothing of
//! Ein reads them.
//!
//! **A value is its text, in the spelling the program's main file gives its
//! names**: one numeral prints `(n z 5)`, `(d.n ar.z 5)` or
//! `(std.math.decimal.n std.math.arith.z 5)` by the form of the import that
//! brought it. So a formatter matches a term by its names' **full names**,
//! which [`Names`] resolves from the program's imports.
//!
//! **The template is filled again, the kernel's line is not edited**
//! ([`fill`]): a reference `{?v}` a row binds is replaced by the value, one
//! nothing binds is left as written. With no formatter ([`Formatters::none`])
//! the line is the kernel's, byte for byte.

pub mod decimal;
pub mod facts;

use std::collections::BTreeMap;

/// **A term**, read off a value's text: a name, or a fact, its head and its
/// arguments, each a term.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Name(String),
    Fact(Vec<Term>),
}

impl Term {
    /// The term as Ein writes it: a name, or its parts in parentheses,
    /// separated by one space.
    pub fn text(&self) -> String {
        match self {
            Term::Name(n) => n.clone(),
            Term::Fact(parts) => {
                let inner: Vec<String> = parts.iter().map(Term::text).collect();
                format!("({})", inner.join(" "))
            }
        }
    }

    /// The head's name, where the term is a fact headed by a name.
    pub fn head(&self) -> Option<&str> {
        match self {
            Term::Fact(parts) => match parts.first() {
                Some(Term::Name(n)) => Some(n),
                _ => None,
            },
            Term::Name(_) => None,
        }
    }

    /// The arguments of a fact; none of a name.
    pub fn args(&self) -> &[Term] {
        match self {
            Term::Fact(parts) if !parts.is_empty() => &parts[1..],
            _ => &[],
        }
    }
}

/// **Whether a character is one of a name's**, as Ein's lexer and the
/// kernel's filler read a name: letters, digits, `_ - .`, and
/// `$ % ^ * + = < > ! & @ ~ / ' ` [ ] { }`. Held to the kernel's filler by
/// EinAf's corpus test.
pub fn is_name_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || matches!(
            c,
            '_' | '-'
                | '.'
                | '$'
                | '%'
                | '^'
                | '*'
                | '+'
                | '='
                | '<'
                | '>'
                | '!'
                | '&'
                | '@'
                | '~'
                | '/'
                | '\''
                | '`'
                | '['
                | ']'
                | '{'
                | '}'
        )
}

/// **Read a value's text as a term**: a name, or a parenthesised list of
/// terms, white space between them. `None` where the text is not one term.
pub fn read(text: &str) -> Option<Term> {
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    let t = read_at(&chars, &mut i)?;
    skip_space(&chars, &mut i);
    (i == chars.len()).then_some(t)
}

fn skip_space(c: &[char], i: &mut usize) {
    while *i < c.len() && c[*i].is_whitespace() {
        *i += 1;
    }
}

fn read_at(c: &[char], i: &mut usize) -> Option<Term> {
    skip_space(c, i);
    match c.get(*i)? {
        '(' => {
            *i += 1;
            let mut parts = Vec::new();
            loop {
                skip_space(c, i);
                match c.get(*i)? {
                    ')' => {
                        *i += 1;
                        return (!parts.is_empty()).then_some(Term::Fact(parts));
                    }
                    _ => parts.push(read_at(c, i)?),
                }
            }
        }
        &ch if is_name_char(ch) => {
            let start = *i;
            while *i < c.len() && is_name_char(c[*i]) {
                *i += 1;
            }
            Some(Term::Name(c[start..*i].iter().collect()))
        }
        _ => None,
    }
}

/// **How a printed name resolves to its full name**, from the program's
/// imports: a name an import lists after `:symbols` is that module's; a name
/// `a.x` where an import names the alias `a` is that module's `x`; every
/// other name is as printed, a full name or the program's own.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Names {
    listed: BTreeMap<String, String>,
    aliases: BTreeMap<String, String>,
}

impl Names {
    /// No import: every name is as printed.
    pub fn new() -> Names {
        Names::default()
    }

    /// An import of `module`, with the names it lists and its alias.
    pub fn import(&mut self, module: &str, symbols: &[String], alias: Option<&str>) {
        for s in symbols {
            self.listed
                .entry(s.clone())
                .or_insert_with(|| format!("{module}.{s}"));
        }
        if let Some(a) = alias {
            self.aliases.insert(a.to_string(), module.to_string());
        }
    }

    /// The full name of a printed name.
    pub fn full(&self, printed: &str) -> String {
        if let Some(f) = self.listed.get(printed) {
            return f.clone();
        }
        for (alias, module) in &self.aliases {
            if let Some(rest) = printed.strip_prefix(&format!("{alias}.")) {
                return format!("{module}.{rest}");
            }
        }
        printed.to_string()
    }
}

/// **A formatter of one kind of value**: a term to a text, or `None` where
/// the term is not of its kind. `all` formats a part of the term, a fact's
/// argument, by every formatter.
pub trait Format {
    /// The kind's name, as a record names the formatter that wrote a value.
    fn kind(&self) -> &str;
    fn format(&self, term: &Term, names: &Names, all: &Formatters) -> Option<String>;
}

/// **The formatters an answer is written with**, tried in order; a value no
/// formatter takes prints as written.
pub struct Formatters {
    kinds: Vec<Box<dyn Format>>,
}

/// **One value, formatted**: its text as written, the text it is printed
/// as, and the formatter that wrote it, `None` where it prints as written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value {
    pub written: String,
    pub text: String,
    pub by: Option<String>,
}

impl Value {
    /// The value prints as a term: no formatter took it, and it is a fact.
    pub fn is_term(&self) -> bool {
        self.by.is_none() && self.text.starts_with('(')
    }
}

impl Formatters {
    /// No formatter: every value prints as written.
    pub fn none() -> Formatters {
        Formatters { kinds: Vec::new() }
    }

    /// **EinAf's formatters**: the decimal numeral, then the word theory's
    /// quantities.
    pub fn standard() -> Formatters {
        Formatters {
            kinds: vec![Box::new(decimal::Decimal), Box::new(facts::Cases::words())],
        }
    }

    /// These formatters and `kind` after them.
    pub fn with(mut self, kind: Box<dyn Format>) -> Formatters {
        self.kinds.push(kind);
        self
    }

    /// The names of the kinds, in order.
    pub fn kinds(&self) -> Vec<String> {
        self.kinds.iter().map(|k| k.kind().to_string()).collect()
    }

    /// A term by the first formatter that takes it; `None` where none does.
    pub fn term(&self, term: &Term, names: &Names) -> Option<(String, String)> {
        self.kinds.iter().find_map(|k| {
            k.format(term, names, self)
                .map(|t| (t, k.kind().to_string()))
        })
    }

    /// A term by the first formatter that takes it, or as written.
    pub fn term_or_written(&self, term: &Term, names: &Names) -> String {
        self.term(term, names)
            .map(|(t, _)| t)
            .unwrap_or_else(|| term.text())
    }

    /// **A value's text, formatted**: read as a term, written by the first
    /// formatter that takes it; as written where it is no term or none does.
    pub fn value(&self, written: &str, names: &Names) -> Value {
        let by = if self.kinds.is_empty() {
            None
        } else {
            read(written).and_then(|t| self.term(&t, names))
        };
        match by {
            Some((text, kind)) => Value {
                written: written.to_string(),
                text,
                by: Some(kind),
            },
            None => Value {
                written: written.to_string(),
                text: written.to_string(),
                by: None,
            },
        }
    }

    /// **A row through a template, its values formatted**: [`fill`] over the
    /// formatted values, and each value as it was formatted, in the row's
    /// order.
    pub fn fill(
        &self,
        template: &str,
        row: &[(String, String)],
        names: &Names,
    ) -> (String, Vec<Value>) {
        let values: Vec<Value> = row.iter().map(|(_, v)| self.value(v, names)).collect();
        let bound: Vec<(String, String)> = row
            .iter()
            .zip(&values)
            .map(|((k, _), v)| (k.clone(), v.text.clone()))
            .collect();
        (fill(template, &bound), values)
    }
}

/// **Fill a template by the kernel's rule**: `{?` then one or more of a
/// name's characters other than `}`, then `}`, is a reference to the
/// variable so named, replaced by its value where `row` binds it and left as
/// written where it does not; every other character is copied.
pub fn fill(template: &str, row: &[(String, String)]) -> String {
    let b = template.as_bytes();
    let mut out = String::with_capacity(template.len());
    let mut i = 0;
    while i < b.len() {
        if let Some(end) = reference_end(b, i) {
            let name = &template[i + 2..end];
            match row.iter().find(|(k, _)| k == name) {
                Some((_, v)) => out.push_str(v),
                None => out.push_str(&template[i..=end]),
            }
            i = end + 1;
            continue;
        }
        let ch = template[i..].chars().next().expect("in bounds");
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

fn reference_end(b: &[u8], i: usize) -> Option<usize> {
    if b.get(i) != Some(&b'{') || b.get(i + 1) != Some(&b'?') {
        return None;
    }
    let start = i + 2;
    let mut j = start;
    while j < b.len() && b[j] != b'}' && is_name_char(char::from(b[j])) {
        j += 1;
    }
    (j > start && b.get(j) == Some(&b'}')).then_some(j)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn a_value_is_read_as_a_term_and_written_back() {
        let t = read("(n (n z 4) 2)").expect("a term");
        assert_eq!(t.text(), "(n (n z 4) 2)");
        assert_eq!(t.head(), Some("n"));
        assert_eq!(t.args().len(), 2);
        assert_eq!(read("  Bob "), Some(Term::Name("Bob".to_string())));
        assert_eq!(read("(a b"), None);
        assert_eq!(read("a b"), None);
        assert_eq!(read("()"), None);
    }

    #[test]
    fn a_printed_name_resolves_by_the_imports() {
        let mut n = Names::new();
        n.import("std.math.decimal", &["n".to_string()], None);
        n.import("std.math.arith", &[], Some("ar"));
        assert_eq!(n.full("n"), "std.math.decimal.n");
        assert_eq!(n.full("ar.z"), "std.math.arith.z");
        assert_eq!(n.full("std.math.arith.z"), "std.math.arith.z");
        assert_eq!(n.full("Bob"), "Bob");
    }

    #[test]
    fn the_template_is_filled_by_the_kernels_rule() {
        assert_eq!(
            fill(
                "{?q}: {?n}",
                &row(&[("q", "(amount Bob apples t1)"), ("n", "3")])
            ),
            "(amount Bob apples t1): 3"
        );
        // Unbound and malformed references are left as written.
        assert_eq!(
            fill("{?x} {y} {?} {?a", &row(&[("a", "1")])),
            "{?x} {y} {?} {?a"
        );
        // A reference reads to its first `}`.
        assert_eq!(fill("{?a}b}", &row(&[("a", "1")])), "1b}");
    }

    #[test]
    fn with_no_formatter_a_value_prints_as_written() {
        let f = Formatters::none();
        let v = f.value("(n z 3)", &Names::new());
        assert_eq!(v.text, "(n z 3)");
        assert!(v.is_term());
    }
}
