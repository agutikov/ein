//! **A grammar a step**: the whole grammar restricted to the forms a step
//! of the formalization writes, in the order it writes them, written at
//! request time from what the step is rendered on
//! ([the grammars' page § 3](../../../docs/einaf/06-grammars.md#3-a-grammar-a-step)).
//!
//! | step | what it allows, in order | rendered on |
//! |---|---|---|
//! | 1 | statements (facts and stored negations, `:source` required) and queries, the answer query among them once; then `relation` forms, `:why` required and its references held to the declaration's arrows, and memberships | the input's sentences, or nothing |
//! | 2 | the ontology (`relation` forms and facts); imports of the catalogue's modules with the names each lists; the catalogue's activating facts at their counts, and the marker of a missing property | the catalogue |
//! | 2b | `rule` forms, closed over the program's vocabulary | the catalogue; the program so far |
//! | 3 | the activating facts of the modules the program imports, at their counts, over the program's names; a marker before each `rule` form; the hypothesis mechanism, `(solve …)` and written `(open F)` | the catalogue; the program so far |
//!
//! **The layout is the model's**: a paren hugs what it holds, a separator
//! between two tokens is a run of up to [`SPACES`] spaces or a line break and
//! an indentation as long, forms are a line break or a blank line apart, and
//! no comment is written but the marker, `;; missing: …`, a line of its own.
//! A name is a run of letters, digits, `_` and `-` that is no word of
//! Ein's; a string holds no `"`, `\` or line break, and a template's braces
//! only its references.
//!
//! **What a step's vocabulary closes**: step 1 writes its facts before their
//! declarations, so it closes none of them; step 2 declares relations in the
//! same output as the facts of them, so its ontology is open and its imports
//! and activating facts are the catalogue's; steps 2b and 3 are rendered on
//! the program of the steps before them, and every pattern, conclusion,
//! activating fact and hypothesis they write is of a relation the program or
//! a module it imports declares, at a count it is declared at, over the
//! program's names.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::gbnf::{Expr, Grammar, Rule};
use super::whole;
use crate::kernel::Vocabulary;

/// The longest run of spaces a separator is.
pub const SPACES: u32 = 24;
/// The most labels a declaration is written with.
pub const LABELS: usize = 8;

/// **A step of the formalization**, and step 2's second call.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Step {
    /// The facts, the answer query and a partial ontology.
    One,
    /// The complete ontology and the selection.
    Two,
    /// Step 2's second call: the rules for the property a marker names.
    TwoRules,
    /// The rules still missing, and the hypothesis mechanism.
    Three,
}

impl Step {
    pub const ALL: [Step; 4] = [Step::One, Step::Two, Step::TwoRules, Step::Three];

    pub fn parse(s: &str) -> Option<Step> {
        match s {
            "1" => Some(Step::One),
            "2" => Some(Step::Two),
            "2b" => Some(Step::TwoRules),
            "3" => Some(Step::Three),
            _ => None,
        }
    }

    pub fn word(self) -> &'static str {
        match self {
            Step::One => "1",
            Step::Two => "2",
            Step::TwoRules => "2b",
            Step::Three => "3",
        }
    }
}

/// **One module of the catalogue**: what a program may import of it and
/// write of it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Module {
    pub module: String,
    /// The names `:symbols` may list: the relations, rules and macros it
    /// declares.
    pub symbols: BTreeSet<String>,
    /// Its relations, and the counts each is declared at.
    pub relations: BTreeMap<String, BTreeSet<usize>>,
    /// Its rules of one parameter or more, a fact of whose name activates
    /// them, and the count of their parameters.
    pub activators: BTreeMap<String, usize>,
    /// The relations its rules read that no module of the library declares,
    /// written as they are, and their counts (`slot-partition`).
    pub unlisted: BTreeMap<String, BTreeSet<usize>>,
}

/// **The catalogue a grammar is rendered on**: the library's modules a
/// program may select a theory from. This one is read off the library's
/// texts, every module and every rule of one parameter or more; the
/// catalogue the steps' prompts show, each entry a program the kernel runs,
/// is written with the prompts, and a grammar rendered on it is narrower.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Catalogue {
    pub modules: Vec<Module>,
}

impl Catalogue {
    /// **The catalogue of the library**: every module of `std`, read through
    /// [`crate::kernel::vocabulary`]; `std.core`, which every program gets,
    /// aside.
    pub fn library() -> Catalogue {
        let texts = crate::kernel::library();
        let vocab: Vec<(String, Vocabulary)> = texts
            .iter()
            .filter(|(m, _)| m != "std.core")
            .filter_map(|(m, t)| crate::kernel::vocabulary(t, m).ok().map(|v| (m.clone(), v)))
            .collect();
        // A word of Ein's is no relation a module reads (`relation`, which
        // `std.contradiction` reads the declarations by).
        let words = whole::lexical(whole::EIN_L).expect("ein.l reads").words;
        let declared_anywhere: BTreeSet<&str> = vocab
            .iter()
            .flat_map(|(_, v)| v.declared.keys().chain(v.rules.keys()).map(String::as_str))
            .collect();
        let modules = vocab
            .iter()
            .map(|(m, v)| Module {
                module: m.clone(),
                symbols: v.declared.keys().chain(v.rules.keys()).cloned().collect(),
                relations: v.declared.clone(),
                activators: v
                    .rules
                    .iter()
                    .filter(|(_, n)| **n > 0)
                    .map(|(r, &n)| (r.clone(), n))
                    .collect(),
                unlisted: v
                    .read
                    .iter()
                    .filter(|(r, _)| {
                        !declared_anywhere.contains(r.as_str()) && !words.contains(r.as_str())
                    })
                    .map(|(r, c)| (r.clone(), c.clone()))
                    .collect(),
            })
            .collect();
        Catalogue { modules }
    }

    pub fn module(&self, name: &str) -> Option<&Module> {
        self.modules.iter().find(|m| m.module == name)
    }
}

/// **What a step's grammar is rendered on.**
#[derive(Clone, Debug, Default)]
pub struct Params {
    /// Step 1: the input's sentences, so that a `:source` is one of them
    /// verbatim; `None` for any string.
    pub sentences: Option<Vec<String>>,
    /// Steps 2, 2b and 3.
    pub catalogue: Catalogue,
    /// Steps 2b and 3: the program of the steps before.
    pub program: Vocabulary,
}

/// **A rendered grammar**: its text, its root, and its text's digest, the
/// envelope's.
#[derive(Clone, Debug)]
pub struct Rendered {
    pub step: Step,
    pub text: String,
    pub root: String,
    pub sha256: String,
}

/// The root of every step's grammar.
pub const ROOT: &str = "root";

impl Rendered {
    /// **The grammar of a request**, held to by the chain's choice first:
    /// from the first token, or begun after the thinking
    /// ([`super::request_grammar`]).
    pub fn request(&self, after_thinking: bool) -> crate::model::Grammar {
        super::request_grammar(&self.text, &self.root, after_thinking)
    }
}

/// **Render a step's grammar** on its parameters.
pub fn render(step: Step, p: &Params) -> Rendered {
    let mut g = Grammar {
        head: vec![
            format!(
                "The grammar of step {} of the formalization, rendered by EinAf for one request.",
                step.word()
            ),
            "Its forms are Ein's grammar's, restricted to the step's; its layout is the model's."
                .to_string(),
        ],
        rules: Vec::new(),
    };
    match step {
        Step::One => step_one(&mut g, p),
        Step::Two => step_two(&mut g, p),
        Step::TwoRules => step_rules(&mut g, p),
        Step::Three => step_three(&mut g, p),
    }
    layout(&mut g);
    let text = g.text();
    Rendered {
        step,
        sha256: crate::model::sha256_hex(text.as_bytes()),
        text,
        root: ROOT.to_string(),
    }
}

/// **Render a step's grammar from texts**: the program of the steps before
/// (steps 2b and 3), the input (step 1, its sentences), the catalogue of the
/// library.
pub fn render_for(
    step: Step,
    program: Option<&str>,
    input: Option<&str>,
    _base: Option<&Path>,
) -> Result<Rendered, String> {
    let mut p = Params {
        sentences: input.map(crate::contract::sentences),
        ..Params::default()
    };
    if step != Step::One {
        p.catalogue = Catalogue::library();
    }
    if let Some(text) = program {
        p.program = crate::kernel::vocabulary(text, "<program>").map_err(|f| {
            f.iter()
                .map(|f| f.message.clone())
                .collect::<Vec<_>>()
                .join("; ")
        })?;
    } else if matches!(step, Step::TwoRules | Step::Three) {
        return Err(format!(
            "step {}'s grammar is rendered on the program of the steps before it",
            step.word()
        ));
    }
    Ok(render(step, &p))
}

// ── The pieces every step's grammar shares ──────────────────────────

fn r(n: &str) -> Expr {
    Expr::r(n)
}

fn lit(s: &str) -> Expr {
    Expr::lit(s)
}

fn seq(v: Vec<Expr>) -> Expr {
    Expr::seq(v)
}

fn alt(v: Vec<Expr>) -> Expr {
    Expr::alt(v)
}

/// `(head` and `)` around items, each after a separator.
fn form(head: &str, items: Vec<Expr>) -> Expr {
    let mut v = vec![lit(&format!("({head}"))];
    for i in items {
        v.push(r("sp"));
        v.push(i);
    }
    v.push(lit(")"));
    seq(v)
}

/// A string: `"` and its characters and `"`, a character of `chars` or a
/// reference of `refs`.
fn string(chars: Expr, refs: Option<Expr>) -> Expr {
    let body = match refs {
        Some(re) => alt(vec![chars, seq(vec![lit("{?"), re, lit("}")])]),
        None => chars,
    };
    seq(vec![lit("\""), Expr::star(body), lit("\"")])
}

/// A character of a string: not `"`, `\` or a line break, and in a
/// template not a brace.
fn text_char(template: bool) -> Expr {
    let mut no = vec![('"', '"'), ('\\', '\\'), ('\n', '\n')];
    if template {
        no.push(('{', '{'));
        no.push(('}', '}'));
    }
    Expr::class(true, &no)
}

/// Ein's string escapes, of a sentence quoted as a `:source`.
fn escaped(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// The layout's and the lexical rules every step's grammar ends with.
fn layout(g: &mut Grammar) {
    let spaces = Expr::times(lit(" "), 1, Some(SPACES));
    g.push(
        Rule::new(
            "sp",
            alt(vec![spaces.clone(), seq(vec![lit("\n"), spaces])]),
        )
        .doc("between two tokens: a run of spaces, or a line break and an indentation"),
    );
    g.push(
        Rule::new("nl", seq(vec![lit("\n"), Expr::opt(lit("\n"))]))
            .doc("between two forms: a line break, or a blank line"),
    );
    let lx = whole::lexical(whole::EIN_L).expect("ein.l reads");
    let set: Vec<(u32, u32)> = NAME_CHARS
        .iter()
        .map(|&(a, b)| (u32::from(a), u32::from(b)))
        .collect();
    // The words no name is, those of a name's characters.
    let words: BTreeSet<String> = lx
        .words
        .iter()
        .filter(|w| {
            w.chars()
                .all(|c| NAME_CHARS.iter().any(|&(a, b)| a <= c && c <= b))
        })
        .cloned()
        .collect();
    for rule in whole::exclusion_rules("m-name", "", &set, &words) {
        g.push(rule);
    }
    let var_words: BTreeSet<String> = ["_".to_string()].into();
    for rule in whole::exclusion_rules("m-var", "?", &set, &var_words) {
        g.push(rule);
    }
}

/// The characters of a name the model writes: letters, digits, `_` and `-`.
const NAME_CHARS: [(char, char); 5] = [('-', '-'), ('0', '9'), ('A', 'Z'), ('_', '_'), ('a', 'z')];

/// A `relation` form: a name, up to [`LABELS`] labels, and a `:why` whose
/// references are the declaration's arrows past the first, `{?2}` to
/// `{?n+2}` for `n` labels.
fn declaration(g: &mut Grammar, prefix: &str) {
    let d = |k: usize| format!("{prefix}decl-{k}");
    g.push(
        Rule::new(
            &format!("{prefix}decl"),
            seq(vec![lit("(relation"), r("sp"), r("m-name"), r(&d(0))]),
        )
        .doc("a `relation` form: the name, its labels, and a `:why` that reads its arrows"),
    );
    for k in 0..=LABELS {
        let why = seq(vec![
            r("sp"),
            lit(":why"),
            r("sp"),
            r(&format!("{prefix}why-{k}")),
            lit(")"),
        ]);
        let body = if k < LABELS {
            alt(vec![why, seq(vec![r("sp"), r("m-name"), r(&d(k + 1))])])
        } else {
            why
        };
        g.push(Rule::new(&d(k), body));
    }
    for k in 0..=LABELS {
        let refs: Vec<Expr> = (2..=k + 2).map(|n| lit(&n.to_string())).collect();
        g.push(Rule::new(
            &format!("{prefix}why-{k}"),
            string(text_char(true), Some(alt(refs))),
        ));
    }
}

/// An open fact's arguments: names, and facts inside it, a numeral among
/// them.
fn open_args(g: &mut Grammar) {
    g.push(Rule::new("args", Expr::star(seq(vec![r("sp"), r("arg")]))));
    g.push(Rule::new(
        "arg",
        alt(vec![
            r("m-name"),
            seq(vec![lit("("), r("m-name"), r("args"), lit(")")]),
        ]),
    ));
}

/// The `:source` of a statement.
fn source(g: &mut Grammar, p: &Params) {
    let body = match &p.sentences {
        Some(ss) if !ss.is_empty() => seq(vec![
            lit("\""),
            alt(ss.iter().map(|s| lit(&escaped(s))).collect()),
            lit("\""),
        ]),
        _ => string(text_char(false), None),
    };
    g.push(Rule::new("source", body).doc(match &p.sentences {
        Some(_) => "a `:source`: one of the input's sentences, verbatim",
        None => "a `:source`: a string",
    }));
}

/// A template's references: variables.
fn var_refs() -> Expr {
    Expr::plus(Expr::class(false, &NAME_CHARS))
}

// ── Step 1 ──────────────────────────────────────────────────────────

fn step_one(g: &mut Grammar, p: &Params) {
    g.push(
        Rule::new(ROOT, seq(vec![r("statements"), r("nl"), r("ontology")]))
            .doc("step 1: the statements with the answer query among them, then the ontology"),
    );
    g.push(Rule::new(
        "statements",
        seq(vec![
            Expr::star(seq(vec![r("statement"), r("nl")])),
            r("answer-query"),
            Expr::star(seq(vec![r("nl"), r("statement")])),
        ]),
    ));
    g.push(Rule::new(
        "statement",
        alt(vec![r("fact"), r("negation"), r("check-query")]),
    ));
    g.push(
        Rule::new(
            "fact",
            seq(vec![
                lit("("),
                r("m-name"),
                r("args"),
                r("sp"),
                lit(":source"),
                r("sp"),
                r("source"),
                lit(")"),
            ]),
        )
        .doc("a statement: a fact of the text's words, with its sentence"),
    );
    g.push(Rule::new(
        "negation",
        seq(vec![
            lit("(not ("),
            r("m-name"),
            r("args"),
            lit(")"),
            r("sp"),
            lit(":source"),
            r("sp"),
            r("source"),
            lit(")"),
        ]),
    ));
    g.push(
        Rule::new(
            "answer-query",
            form(
                "query",
                vec![
                    lit(":goal"),
                    r("premise"),
                    lit(":goal-text"),
                    r("goal-text"),
                ],
            ),
        )
        .doc("the answer query, once: a goal and the sentence that says the answer"),
    );
    g.push(Rule::new(
        "check-query",
        form("query", vec![lit(":goal"), r("premise")]),
    ));
    g.push(Rule::new(
        "premise",
        alt(vec![r("premise-1"), form("absent", vec![r("premise-1")])]),
    ));
    g.push(
        Rule::new(
            "premise-1",
            alt(vec![
                r("pattern"),
                seq(vec![
                    lit("(and"),
                    Expr::plus(seq(vec![r("sp"), r("premise")])),
                    lit(")"),
                ]),
                form("not", vec![r("pattern")]),
            ]),
        )
        .doc("a premise that is no `absent`, which `absent` takes"),
    );
    g.push(Rule::new(
        "pattern",
        seq(vec![
            lit("("),
            r("m-name"),
            Expr::star(seq(vec![r("sp"), r("term")])),
            lit(")"),
        ]),
    ));
    g.push(Rule::new(
        "term",
        alt(vec![r("m-var"), r("m-name"), r("pattern")]),
    ));
    g.push(Rule::new(
        "goal-text",
        string(text_char(true), Some(var_refs())),
    ));
    g.push(
        Rule::new(
            "ontology",
            seq(vec![
                r("decl"),
                Expr::star(seq(vec![r("nl"), alt(vec![r("decl"), r("membership")])])),
            ]),
        )
        .doc("the ontology: declarations, the first first, and memberships"),
    );
    g.push(Rule::new(
        "membership",
        seq(vec![
            lit("("),
            r("m-name"),
            r("args"),
            Expr::opt(seq(vec![r("sp"), lit(":source"), r("sp"), r("source")])),
            lit(")"),
        ]),
    ));
    open_args(g);
    source(g, p);
    declaration(g, "");
}

// ── Step 2 ──────────────────────────────────────────────────────────

/// An import of a module of the catalogue, with the names it lists.
fn imports(g: &mut Grammar, c: &Catalogue) {
    let mut alts = Vec::new();
    for (i, m) in c.modules.iter().enumerate() {
        let syms = format!("symbol-{}", i + 1);
        let list = if m.symbols.is_empty() {
            Expr::Empty
        } else {
            g.push(
                Rule::new(&syms, alt(m.symbols.iter().map(|s| lit(s)).collect()))
                    .doc(&format!("a name {} declares", m.module)),
            );
            Expr::opt(seq(vec![
                r("sp"),
                lit(":symbols"),
                r("sp"),
                lit("("),
                r(&syms),
                Expr::star(seq(vec![r("sp"), r(&syms)])),
                lit(")"),
            ]))
        };
        alts.push(seq(vec![lit(&m.module), list]));
    }
    g.push(
        Rule::new(
            "import",
            seq(vec![lit("(import"), r("sp"), alt(alts), lit(")")]),
        )
        .doc("an import of a module of the catalogue, listing names it declares"),
    );
}

/// An activating fact of the catalogue: each activator's name with as many
/// arguments as it has parameters, and each relation a module reads and no
/// module declares at each count it is read at; the arguments `arg`.
fn activators(heads: &BTreeMap<String, BTreeSet<usize>>, arg: &str) -> Expr {
    let mut alts = Vec::new();
    for (h, counts) in heads {
        for &k in counts {
            alts.push(form(h, vec![r(arg); k]));
        }
    }
    if alts.is_empty() {
        alts.push(lit("(false)"));
    }
    alt(alts)
}

fn catalogue_heads(c: &Catalogue) -> BTreeMap<String, BTreeSet<usize>> {
    let mut heads: BTreeMap<String, BTreeSet<usize>> = BTreeMap::new();
    for m in &c.modules {
        for (a, &n) in &m.activators {
            heads.entry(a.clone()).or_default().insert(n);
        }
        for (u, counts) in &m.unlisted {
            heads
                .entry(u.clone())
                .or_default()
                .extend(counts.iter().copied());
        }
    }
    heads
}

/// **The marker of a missing property**: a comment line of its own, the one
/// comment a step's grammar allows, the property's words after it.
pub const MARKER: &str = ";; missing: ";

fn marker(g: &mut Grammar) {
    g.push(
        Rule::new(
            "marker",
            seq(vec![
                lit(MARKER),
                Expr::plus(Expr::class(true, &[('\n', '\n')])),
            ]),
        )
        .doc("the marker of a missing property: a comment line, the one comment written"),
    );
}

fn step_two(g: &mut Grammar, p: &Params) {
    let block = |item: &str| seq(vec![r(item), Expr::star(seq(vec![r("nl"), r(item)]))]);
    g.push(
        Rule::new(
            ROOT,
            alt(vec![
                seq(vec![
                    r("ontology"),
                    Expr::opt(seq(vec![r("nl"), r("imports")])),
                    Expr::opt(seq(vec![r("nl"), r("selection")])),
                ]),
                seq(vec![
                    r("imports"),
                    Expr::opt(seq(vec![r("nl"), r("selection")])),
                ]),
                r("selection"),
            ]),
        )
        .doc("step 2: the ontology whole, then the imports, then the selection; no rule"),
    );
    g.push(Rule::new("ontology", block("onto-form")));
    g.push(Rule::new("onto-form", alt(vec![r("decl"), r("onto-fact")])));
    g.push(Rule::new(
        "onto-fact",
        seq(vec![
            lit("("),
            r("m-name"),
            r("args"),
            Expr::opt(seq(vec![r("sp"), lit(":source"), r("sp"), r("source")])),
            lit(")"),
        ]),
    ));
    g.push(Rule::new("imports", block("import")));
    g.push(Rule::new("selection", block("select")));
    g.push(Rule::new("select", alt(vec![r("activator"), r("marker")])));
    g.push(
        Rule::new(
            "activator",
            activators(&catalogue_heads(&p.catalogue), "m-name"),
        )
        .doc("an activating fact of the catalogue, at its count, over any names"),
    );
    imports(g, &p.catalogue);
    marker(g);
    open_args(g);
    g.push(Rule::new("source", string(text_char(false), None)));
    declaration(g, "");
}

// ── Steps 2b and 3: the program's vocabulary closed ─────────────────

/// The relations a program may write a pattern of, by the name it writes
/// them under, at their declared counts; the activators and the relations a
/// module reads, likewise; and the names a value may be.
struct Closed {
    relations: BTreeMap<String, BTreeSet<usize>>,
    activators: BTreeMap<String, BTreeSet<usize>>,
    names: BTreeSet<String>,
}

fn closed(p: &Params) -> Closed {
    let v = &p.program;
    let mut relations: BTreeMap<String, BTreeSet<usize>> = v.declared.clone();
    let mut activators: BTreeMap<String, BTreeSet<usize>> = BTreeMap::new();
    let mut names: BTreeSet<String> = v.names.clone();
    names.extend(v.declared.keys().cloned());
    for imp in &v.imports {
        let Some(m) = p.catalogue.module(&imp.module) else {
            continue;
        };
        let listed: BTreeSet<&str> = imp.symbols.iter().flatten().map(String::as_str).collect();
        let written = |n: &str| -> String {
            if listed.contains(n) {
                n.to_string()
            } else if let Some(a) = &imp.alias {
                format!("{a}.{n}")
            } else {
                format!("{}.{n}", m.module)
            }
        };
        for (rel, counts) in &m.relations {
            relations
                .entry(written(rel))
                .or_default()
                .extend(counts.iter().copied());
        }
        for (a, &n) in &m.activators {
            activators.entry(written(a)).or_default().insert(n);
        }
        for (u, counts) in &m.unlisted {
            activators
                .entry(u.clone())
                .or_default()
                .extend(counts.iter().copied());
        }
        names.extend(m.symbols.iter().map(|s| written(s)));
    }
    Closed {
        relations,
        activators,
        names,
    }
}

/// The closed vocabulary's rules: a name, a pattern of a relation at its
/// count, a ground fact, a term.
fn closed_rules(g: &mut Grammar, c: &Closed) {
    let names: Vec<Expr> = c.names.iter().map(|n| lit(n)).collect();
    g.push(
        Rule::new(
            "v-name",
            if names.is_empty() {
                lit("_")
            } else {
                alt(names)
            },
        )
        .doc("a name of the program, or of a module it imports"),
    );
    let mut pats = Vec::new();
    let mut facts = Vec::new();
    for (rel, counts) in &c.relations {
        for &k in counts {
            pats.push(form(rel, vec![r("term"); k]));
            facts.push(form(rel, vec![r("v-arg"); k]));
        }
    }
    if pats.is_empty() {
        pats.push(lit("(false)"));
        facts.push(lit("(false)"));
    }
    g.push(
        Rule::new("v-pattern", alt(pats)).doc(
            "a pattern of a relation the program or a module it imports declares, at its count",
        ),
    );
    g.push(Rule::new("v-fact", alt(facts)));
    g.push(Rule::new(
        "term",
        alt(vec![r("m-var"), r("v-name"), r("v-pattern")]),
    ));
    g.push(Rule::new("v-arg", alt(vec![r("v-name"), r("v-fact")])));
    let rels: Vec<Expr> = c.relations.keys().map(|n| lit(n)).collect();
    g.push(Rule::new(
        "v-relation",
        if rels.is_empty() { lit("_") } else { alt(rels) },
    ));
}

/// A `rule` form over the closed vocabulary: a name, its parameters, a
/// `:match`, an `:assert` and a `:why`, in that order.
fn rule_form(g: &mut Grammar) {
    g.push(
        Rule::new(
            "rule",
            seq(vec![
                lit("(rule"),
                r("sp"),
                r("m-name"),
                r("sp"),
                r("params"),
                r("sp"),
                lit(":match"),
                r("sp"),
                r("match"),
                r("sp"),
                lit(":assert"),
                r("sp"),
                r("assert"),
                r("sp"),
                lit(":why"),
                r("sp"),
                r("why-rule"),
                lit(")"),
            ]),
        )
        .doc("a rule: its name, its parameters, `:match`, `:assert` and `:why`"),
    );
    g.push(Rule::new(
        "params",
        alt(vec![
            lit("()"),
            seq(vec![
                lit("("),
                r("m-var"),
                Expr::star(seq(vec![r("sp"), r("m-var")])),
                lit(")"),
            ]),
        ]),
    ));
    g.push(Rule::new("match", alt(vec![lit("()"), r("premise")])));
    let conj = |word: &str, item: &str| {
        seq(vec![
            lit(&format!("({word}")),
            Expr::plus(seq(vec![r("sp"), r(item)])),
            lit(")"),
        ])
    };
    g.push(Rule::new(
        "premise",
        alt(vec![r("premise-1"), form("absent", vec![r("premise-1")])]),
    ));
    g.push(
        Rule::new(
            "premise-1",
            alt(vec![
                r("v-pattern"),
                conj("and", "premise"),
                conj("or", "premise"),
                form("not", vec![r("v-pattern")]),
                form("eq", vec![r("term"), r("term")]),
                form("neq", vec![r("term"), r("term")]),
            ]),
        )
        .doc("a premise that is no `absent`, which `absent` takes"),
    );
    g.push(Rule::new("assert", alt(vec![lit("()"), r("conclusion")])));
    g.push(Rule::new(
        "conclusion",
        alt(vec![
            r("v-pattern"),
            conj("and", "conclusion"),
            form("not", vec![r("v-pattern")]),
            form("open", vec![r("v-pattern")]),
            lit("(false)"),
        ]),
    ));
    g.push(Rule::new(
        "why-rule",
        string(text_char(true), Some(var_refs())),
    ));
}

fn step_rules(g: &mut Grammar, p: &Params) {
    let c = closed(p);
    g.push(
        Rule::new(
            ROOT,
            seq(vec![r("rule"), Expr::star(seq(vec![r("nl"), r("rule")]))]),
        )
        .doc("step 2's second call: the rules for the property its marker names"),
    );
    rule_form(g);
    closed_rules(g, &c);
}

fn step_three(g: &mut Grammar, p: &Params) {
    let c = closed(p);
    let block = |item: &str| seq(vec![r(item), Expr::star(seq(vec![r("nl"), r(item)]))]);
    g.push(
        Rule::new(
            ROOT,
            alt(vec![
                Expr::Empty,
                seq(vec![
                    r("selection"),
                    Expr::opt(seq(vec![r("nl"), r("synthesis")])),
                    Expr::opt(seq(vec![r("nl"), r("mechanism")])),
                ]),
                seq(vec![r("synthesis"), Expr::opt(seq(vec![r("nl"), r("mechanism")]))]),
                r("mechanism"),
            ]),
        )
        .doc("step 3: the selection completed, the rules still missing each after its marker, the hypothesis mechanism"),
    );
    g.push(Rule::new("selection", block("activator")));
    g.push(
        Rule::new("activator", activators(&c.activators, "v-name"))
            .doc("an activating fact of a module the program imports, at its count, over the program's names"),
    );
    g.push(Rule::new(
        "synthesis",
        seq(vec![
            r("marker"),
            r("nl"),
            r("rule"),
            Expr::star(seq(vec![r("nl"), r("marker"), r("nl"), r("rule")])),
        ]),
    ));
    g.push(Rule::new("mechanism", block("mechanism-form")));
    g.push(Rule::new(
        "mechanism-form",
        alt(vec![
            lit("(solve :no-search)"),
            seq(vec![
                lit("(solve :hypothesis-relations ("),
                r("v-relation"),
                Expr::star(seq(vec![r("sp"), r("v-relation")])),
                lit("))"),
            ]),
            form("open", vec![r("v-fact")]),
        ]),
    ));
    marker(g);
    rule_form(g);
    closed_rules(g, &c);
}
