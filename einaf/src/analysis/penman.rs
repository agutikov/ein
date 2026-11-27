//! **PENMAN, read**: the notation an AMR parser prints a sentence's graph in,
//! `(variable / concept :role target …)`, a target a node, a variable met
//! again, a constant or a string, each concept, constant or string with the
//! aligner's marker after it where it has one (`~e.2`, amrlib's aligner's;
//! `~2`, IBM's parser's), a token's index from 0.
//!
//! The reader follows the `penman` package's lexicon: a symbol is a run of
//! characters other than white space, `(`, `)`, `/`, `:` and `~`; a role is
//! `:` and such a run; a string is between double quotes, `\"` and `\\` its
//! escapes. **A role read the other way** (`:ARG0-of`, a role that ends in
//! `-of` and is none of AMR's three roles that are spelt so, `:consist-of`,
//! `:prep-out-of` and `:prep-on-behalf-of`) is the role itself between the
//! two nodes swapped, as the `penman` package normalizes it: `(m / man
//! :ARG0-of (s / smoke-02))` is the edge `s :ARG0 m`. Nothing here reads an
//! English word.

use std::collections::BTreeSet;

/// What an edge points at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    /// A node of the graph, by its variable.
    Node(String),
    /// A constant: a number, `-`, a symbol that is no variable.
    Constant(String),
    /// A string, unescaped.
    Str(String),
}

/// **A node**: its variable, its concept, and the tokens the aligner gave
/// its concept.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub var: String,
    /// `None` for a node written with no concept, `(x)`.
    pub concept: Option<String>,
    pub align: Vec<usize>,
}

/// **An edge**: from a node, a role, to a target, its role normalized (no
/// colon, not inverted).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub source: String,
    pub role: String,
    pub target: Target,
    /// The tokens the aligner gave the target, where it is a constant or a
    /// string.
    pub align: Vec<usize>,
    /// The parser wrote the role the other way (`:ARG0-of`).
    pub inverted: bool,
}

/// **A sentence's graph**: its top, its nodes in the order written, its edges
/// in the order written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Graph {
    pub top: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    /// The node of a variable.
    pub fn node(&self, var: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.var == var)
    }

    /// The edges out of a node, in the order written, inverted ones
    /// normalized.
    pub fn out_of<'a>(&'a self, var: &'a str) -> impl Iterator<Item = &'a Edge> + 'a {
        self.edges.iter().filter(move |e| e.source == var)
    }

    /// Whether a node's concept carries the aligner's marker.
    pub fn aligned(&self, var: &str) -> bool {
        self.node(var).is_some_and(|n| !n.align.is_empty())
    }
}

/// The three roles AMR spells with `-of` that are not read the other way.
const NOT_INVERTED: [&str; 3] = ["consist-of", "prep-out-of", "prep-on-behalf-of"];

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tok {
    Open,
    Close,
    Slash,
    Role(String),
    Symbol(String),
    Str(String),
    Align(Vec<usize>),
}

/// The tokens of a PENMAN text, each with its byte range.
fn lex(text: &str) -> Result<Vec<(Tok, std::ops::Range<usize>)>, String> {
    let b = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    let symbol_end = |mut j: usize| {
        while j < b.len() && !b[j].is_ascii_whitespace() && !b"()/:~\"".contains(&b[j]) {
            j += 1;
        }
        j
    };
    while i < b.len() {
        let c = b[i];
        let start = i;
        match c {
            c if c.is_ascii_whitespace() => {
                i += 1;
                continue;
            }
            b'#' if i == 0 || b[i - 1] == b'\n' => {
                // A metadata or comment line.
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            b'(' => {
                i += 1;
                out.push((Tok::Open, start..i));
            }
            b')' => {
                i += 1;
                out.push((Tok::Close, start..i));
            }
            b'/' => {
                i += 1;
                out.push((Tok::Slash, start..i));
            }
            b':' => {
                i = symbol_end(i + 1);
                out.push((Tok::Role(text[start + 1..i].to_string()), start..i));
            }
            b'"' => {
                let mut s = String::new();
                i += 1;
                loop {
                    match b.get(i) {
                        None => return Err(format!("a string not closed at byte {start}")),
                        Some(b'"') => {
                            i += 1;
                            break;
                        }
                        Some(b'\\') if i + 1 < b.len() => {
                            let n = text[i + 1..].chars().next().unwrap_or('\\');
                            s.push(n);
                            i += 1 + n.len_utf8();
                        }
                        Some(_) => {
                            let ch = text[i..].chars().next().unwrap_or('\0');
                            s.push(ch);
                            i += ch.len_utf8();
                        }
                    }
                }
                out.push((Tok::Str(s), start..i));
            }
            b'~' => {
                // `~` then an optional prefix letter and a dot, then indices.
                i += 1;
                if i < b.len() && b[i].is_ascii_alphabetic() {
                    i += 1;
                    if i < b.len() && b[i] == b'.' {
                        i += 1;
                    }
                }
                let digits_start = i;
                while i < b.len() && (b[i].is_ascii_digit() || b[i] == b',') {
                    i += 1;
                }
                let idx: Result<Vec<usize>, _> = text[digits_start..i]
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(str::parse)
                    .collect();
                let idx = idx.map_err(|_| format!("an alignment at byte {start} is not read"))?;
                out.push((Tok::Align(idx), start..i));
            }
            _ => {
                i = symbol_end(i);
                if i == start {
                    return Err(format!("an unexpected character at byte {start}"));
                }
                out.push((Tok::Symbol(text[start..i].to_string()), start..i));
            }
        }
    }
    Ok(out)
}

/// **The graph as the parser printed it, the aligner's markers removed**:
/// every other character as written, metadata lines too.
pub fn without_alignments(text: &str) -> Result<String, String> {
    let toks = lex(text)?;
    let mut out = String::new();
    let mut at = 0;
    for (t, r) in toks {
        if matches!(t, Tok::Align(_)) {
            out.push_str(&text[at..r.start]);
            at = r.end;
        }
    }
    out.push_str(&text[at..]);
    Ok(out)
}

/// What a target was read as before the variables are all known.
enum Raw {
    Node(String),
    Symbol(String),
    Str(String),
}

struct RawEdge {
    source: String,
    role: String,
    target: Raw,
    align: Vec<usize>,
}

/// **Read a PENMAN graph**: one node at the top, its metadata lines (`#`)
/// skipped.
pub fn read(text: &str) -> Result<Graph, String> {
    let toks: Vec<Tok> = lex(text)?.into_iter().map(|(t, _)| t).collect();
    let mut pos = 0;
    let mut nodes = Vec::new();
    let mut raw = Vec::new();
    let top = node(&toks, &mut pos, &mut nodes, &mut raw)?;
    if pos != toks.len() {
        return Err("text after the graph's last parenthesis".to_string());
    }
    let vars: BTreeSet<&str> = nodes.iter().map(|n: &Node| n.var.as_str()).collect();
    let mut edges = Vec::new();
    for e in raw {
        let target = match e.target {
            Raw::Node(v) => Target::Node(v),
            Raw::Symbol(s) if vars.contains(s.as_str()) => Target::Node(s),
            Raw::Symbol(s) => Target::Constant(s),
            Raw::Str(s) => Target::Str(s),
        };
        let inverted = e.role.ends_with("-of") && !NOT_INVERTED.contains(&e.role.as_str());
        edges.push(match (&target, inverted) {
            (Target::Node(t), true) => Edge {
                source: t.clone(),
                role: e.role[..e.role.len() - 3].to_string(),
                target: Target::Node(e.source),
                align: e.align,
                inverted: true,
            },
            _ => Edge {
                source: e.source,
                role: e.role,
                target,
                align: e.align,
                inverted: false,
            },
        });
    }
    Ok(Graph { top, nodes, edges })
}

fn node(
    toks: &[Tok],
    pos: &mut usize,
    nodes: &mut Vec<Node>,
    edges: &mut Vec<RawEdge>,
) -> Result<String, String> {
    let expect = |p: &usize, what: &str| -> String { format!("{what} expected at token {p}") };
    if toks.get(*pos) != Some(&Tok::Open) {
        return Err(expect(pos, "("));
    }
    *pos += 1;
    let Some(Tok::Symbol(var)) = toks.get(*pos) else {
        return Err(expect(pos, "a variable"));
    };
    let var = var.clone();
    *pos += 1;
    let mut n = Node {
        var: var.clone(),
        concept: None,
        align: Vec::new(),
    };
    if toks.get(*pos) == Some(&Tok::Slash) {
        *pos += 1;
        match toks.get(*pos) {
            Some(Tok::Symbol(c)) => n.concept = Some(c.clone()),
            Some(Tok::Str(c)) => n.concept = Some(c.clone()),
            _ => return Err(expect(pos, "a concept")),
        }
        *pos += 1;
        if let Some(Tok::Align(a)) = toks.get(*pos) {
            n.align = a.clone();
            *pos += 1;
        }
    }
    nodes.push(n);
    loop {
        match toks.get(*pos) {
            Some(Tok::Close) => {
                *pos += 1;
                return Ok(var);
            }
            Some(Tok::Role(role)) => {
                let role = role.clone();
                *pos += 1;
                if let Some(Tok::Align(_)) = toks.get(*pos) {
                    *pos += 1;
                }
                let (target, mut align) = match toks.get(*pos) {
                    // A node: its edge before the node's own, in the order
                    // written.
                    Some(Tok::Open) => {
                        let Some(Tok::Symbol(child)) = toks.get(*pos + 1) else {
                            return Err(expect(&(*pos + 1), "a variable"));
                        };
                        edges.push(RawEdge {
                            source: var.clone(),
                            role,
                            target: Raw::Node(child.clone()),
                            align: Vec::new(),
                        });
                        node(toks, pos, nodes, edges)?;
                        continue;
                    }
                    Some(Tok::Symbol(s)) => {
                        *pos += 1;
                        (Raw::Symbol(s.clone()), Vec::new())
                    }
                    Some(Tok::Str(s)) => {
                        *pos += 1;
                        (Raw::Str(s.clone()), Vec::new())
                    }
                    _ => return Err(expect(pos, "a role's target")),
                };
                if !matches!(target, Raw::Node(_))
                    && let Some(Tok::Align(a)) = toks.get(*pos)
                {
                    align = a.clone();
                    *pos += 1;
                }
                edges.push(RawEdge {
                    source: var.clone(),
                    role,
                    target,
                    align,
                });
            }
            _ => return Err(expect(pos, "a role or )")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_graph_with_an_inverted_role_a_name_and_markers_reads() {
        let g = read(
            "# ::snt x\n(z1 / live-01~e.2\n  :ARG0 (z2 / man\n    :ARG0-of (z3 / smoke-02\n      :ARG1 (z4 / name :op1 \"Old\"~e.1 :op2 \"Gold\")))\n  :polarity -\n  :location z2)",
        )
        .unwrap();
        assert_eq!(g.top, "z1");
        assert_eq!(g.nodes.len(), 4);
        assert_eq!(g.node("z1").unwrap().align, vec![2]);
        let smoke: Vec<_> = g.out_of("z3").collect();
        assert_eq!(smoke[0].role, "ARG0");
        assert_eq!(smoke[0].target, Target::Node("z2".into()));
        assert!(smoke[0].inverted);
        let z4: Vec<_> = g.out_of("z4").collect();
        assert_eq!(z4[0].target, Target::Str("Old".into()));
        assert_eq!(z4[0].align, vec![1]);
        let z1: Vec<_> = g.out_of("z1").collect();
        assert_eq!(z1[1].target, Target::Constant("-".into()));
        assert_eq!(z1[2].target, Target::Node("z2".into()));
    }

    #[test]
    fn consist_of_is_not_read_the_other_way_and_markers_are_removed() {
        let text = "(r / row~e.8 :consist-of (c / chair~e.11 :quant 3~e.10))";
        let g = read(text).unwrap();
        assert_eq!(g.edges[0].source, "r");
        assert_eq!(g.edges[0].role, "consist-of");
        assert_eq!(
            without_alignments(text).unwrap(),
            "(r / row :consist-of (c / chair :quant 3))"
        );
        assert_eq!(
            without_alignments("(d / drunk~2 :op1 \"a~1\"~0)").unwrap(),
            "(d / drunk :op1 \"a~1\")"
        );
    }

    #[test]
    fn a_text_that_is_no_graph_is_an_error() {
        assert!(read("(a / b").is_err());
        assert!(read("(a / b) (c / d)").is_err());
        assert!(read("(a / b :op1)").is_err());
    }
}
