//! **The analysis as Ein facts**: the vocabulary's declarations
//! ([`super::VOCABULARY`]), then, a sentence after another, its graph as
//! facts, each quoting the sentence as its `:source`, and last `(solve
//! :no-search)`: a program of declarations and facts, which `ein solve
//! --no-search` loads and `ein render kb` draws. **The `(solve :no-search)`
//! is the analysis's word that it states every fact there is**: a program
//! that states no mechanism has H written by the blind enumerator, over
//! every declared relation and every name, which on an analysis of a few
//! hundred facts is tens of thousands of candidates at root: a load of
//! seconds where it takes milliseconds, and a drawing of the candidates'
//! `open` facts hundreds of times the size of the analysis's own.
//!
//! | AMR | the fact |
//! |---|---|
//! | sentence `n` | the name `Sn` |
//! | the graph's top | `(amr-top Sn N)` |
//! | a node `z1 / drink-01` | `(amr-node Sn sn-z1 drink-01)` |
//! | `:ARG0 (z2 …)`, `:ARG0-of` read the other way | `(amr-role sn-z1 ARG0 sn-z2)` |
//! | `:quant 5`, any other value | `(amr-const sn-z1 quant 5)` |
//! | `(z4 / name :op1 "Old" :op2 "Gold")` | `(amr-name sn-z4 Old_Gold)` beside its `amr-node` |
//! | `:polarity -` | `(amr-negative sn-z1)` |
//! | a node's word, by a learned aligner | `(amr-word sn-z1 drunk)` |
//! | a named entity | `(named-entity Sn Old_Gold PRODUCT)` |
//!
//! Every concept, role, value and word is written by [`super::name`].

use std::fmt::Write as _;

use super::penman::Target;
use super::{Given, VOCABULARY, name};

/// **The facts form**: its text and its count of facts.
pub struct Program {
    pub text: String,
    pub facts: usize,
}

/// A string as Ein writes one.
fn quoted(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// **The facts of a given analysis**, its parser's graphs, the named
/// entities beside them where they are given.
pub fn program(given: &Given) -> Program {
    let mut text = VOCABULARY
        .render(&Default::default())
        .expect("the vocabulary has no slot");
    let mut facts = 0;
    let shown = given.words_shown();
    for r in given.graphs() {
        let _ = write!(text, "\n;; {}. {}\n", r.n, r.sentence);
        let source = quoted(r.sentence);
        let mut fact = |text: &mut String, args: &[&str]| {
            let _ = writeln!(text, "({} :source {source})", args.join(" "));
            facts += 1;
        };
        let sentence = format!("S{}", r.n);
        match &r.graph {
            Err(why) => {
                let _ = writeln!(text, ";; no graph: {why}");
            }
            Ok(g) => {
                let node = |var: &str| format!("s{}-{}", r.n, name(var));
                fact(&mut text, &["amr-top", &sentence, &node(&g.top)]);
                for n in &g.nodes {
                    let me = node(&n.var);
                    let concept = name(n.concept.as_deref().unwrap_or(""));
                    fact(&mut text, &["amr-node", &sentence, &me, &concept]);
                    for w in r.words(shown, &n.var) {
                        fact(&mut text, &["amr-word", &me, &name(w)]);
                    }
                    let is_name = n.concept.as_deref() == Some("name");
                    let op =
                        |role: &str| -> Option<usize> { role.strip_prefix("op")?.parse().ok() };
                    if is_name {
                        let mut words: Vec<(usize, &str)> = g
                            .out_of(&n.var)
                            .filter_map(|e| match (&e.target, op(&e.role)) {
                                (Target::Str(s), Some(i)) => Some((i, s.as_str())),
                                _ => None,
                            })
                            .collect();
                        words.sort();
                        if !words.is_empty() {
                            let joined: Vec<&str> = words.iter().map(|(_, w)| *w).collect();
                            fact(&mut text, &["amr-name", &me, &name(&joined.join(" "))]);
                        }
                    }
                    for e in g.out_of(&n.var) {
                        let role = name(&e.role);
                        match &e.target {
                            Target::Str(_) if is_name && op(&e.role).is_some() => {}
                            Target::Node(m) => fact(&mut text, &["amr-role", &me, &role, &node(m)]),
                            Target::Constant(c) if e.role == "polarity" && c == "-" => {
                                fact(&mut text, &["amr-negative", &me])
                            }
                            Target::Constant(v) | Target::Str(v) => {
                                fact(&mut text, &["amr-const", &me, &role, &name(v)])
                            }
                        }
                    }
                }
            }
        }
        if given.choice.entities
            && let Some(e) = &given.analysis.entities
        {
            for ent in e.sentences.get(r.n - 1).into_iter().flatten() {
                fact(
                    &mut text,
                    &[
                        "named-entity",
                        &sentence,
                        &name(&ent.text),
                        &name(&ent.label),
                    ],
                );
            }
        }
    }
    // The analysis states every fact there is: root alone is judged, and no
    // candidate is guessed about its relations.
    text.push_str("\n(solve :no-search)\n");
    Program { text, facts }
}
