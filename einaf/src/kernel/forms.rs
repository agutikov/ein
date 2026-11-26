//! **A program's forms, read and asked nothing**: each top-level form of the
//! program's own text, as `ein_ir::dump_canonical` prints it and by its
//! shape, and what the modules it imports bring, by the names the program
//! writes. What [the contract's categories](crate::contract) are computed
//! from.
//!
//! The program is parsed and loaded as `ein solve` loads it, its imports
//! resolved against its file's directory, and nothing is saturated: no rule
//! fires and no question is asked, so no bound is needed. A program the
//! kernel refuses at the read, the parse or the load is the refusal's
//! findings.

use std::collections::{BTreeMap, BTreeSet};

use ein_ir::{Ast, Node, NodeId};
use serde::{Deserialize, Serialize};

use super::Source;
use super::feedback::{Finding, finding};

/// **A program's top-level forms, in the order written**, and what its
/// imports bring.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Forms {
    /// The file as given, or a text's name.
    pub name: String,
    /// Each top-level form of the program's own text, in order; a module's
    /// forms are not among them.
    pub forms: Vec<Form>,
    /// What the imported modules bring, the module every program gets
    /// among them.
    pub library: Library,
}

/// One top-level form.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Form {
    /// The form as `ein_ir::dump_canonical` prints it, without its last
    /// newline: its layout and comments normalised, so two spellings of one
    /// form have one text.
    pub canonical: String,
    pub shape: Shape,
}

/// What a top-level form is, read off its head.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "is")]
pub enum Shape {
    /// `(relation NAME LABEL* :why "…")`.
    Relation { name: String },
    /// `(rule NAME (PARAMS) …)`, a rule that concludes `(open F)` among them.
    Rule { name: String },
    /// `(macro NAME (PARAMS) BODY)`.
    Macro { name: String },
    /// `(import MODULE …)`.
    Import { module: String },
    /// `(query :goal …)`: whether it carries a `:goal-text`, and the
    /// relations its goal reads, each once, in the order written.
    Query { goal_text: bool, reads: Vec<String> },
    /// `(solve …)`.
    Solve,
    /// `(config …)`.
    Config,
    /// `(trace …)`.
    Trace,
    /// A fact the program writes: of `relation`, held, its stored negation
    /// `(not F)`, or a hypothesis `(open F)`; with its `:source`, where it
    /// carries one.
    Fact {
        relation: String,
        written: Written,
        source: Option<String>,
    },
}

/// How a top-level fact is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Written {
    /// `(R a …)`: stated as holding.
    Holds,
    /// `(not (R a …))`: a stored negation.
    Negated,
    /// `(open (R a …))`: a hypothesis.
    Open,
}

/// **What the imported modules bring**, by the names the program writes: a
/// name an import lists is its short name, any other its full name, as the
/// loader renames them.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Library {
    /// Each module the program imports, as written, and the module every
    /// program gets.
    pub modules: Vec<String>,
    /// The rules of the imported modules, obligation and hypothesis rules
    /// among them: a fact of one of these names activates the rule.
    pub rules: BTreeSet<String>,
    /// The relations a rule of an imported module reads in its `:match`: a
    /// fact of one of them is read by the library.
    pub reads: BTreeSet<String>,
    /// **Every relation the loaded program declares**, its own text's and its
    /// modules', by the names the file writes, and the counts of the labels
    /// each is declared with: what a fact and a pattern are held to.
    pub declared: BTreeMap<String, BTreeSet<usize>>,
}

impl Forms {
    /// The relations the program's own text declares, each once.
    pub fn declared(&self) -> BTreeSet<&str> {
        self.forms
            .iter()
            .filter_map(|f| match &f.shape {
                Shape::Relation { name } => Some(name.as_str()),
                _ => None,
            })
            .collect()
    }

    /// The rules the program's own text writes.
    pub fn rules(&self) -> BTreeSet<&str> {
        self.forms
            .iter()
            .filter_map(|f| match &f.shape {
                Shape::Rule { name } | Shape::Macro { name } => Some(name.as_str()),
                _ => None,
            })
            .collect()
    }
}

/// **Read `source`'s forms**, parsed and loaded as `ein solve` loads it, and
/// asked nothing; a refusal is its findings, the kernel's diagnostics.
pub fn forms(source: Source<'_>) -> Result<Forms, Vec<Finding>> {
    let (text, name, base_dir) = match source {
        Source::File(path) => match ein_ir::read::read_program(path) {
            Ok(ein_ir::read::Held::Text(text)) => (text, path.display().to_string(), path.parent()),
            Ok(ein_ir::read::Held::Container(_)) => {
                let message = format!(
                    "{} is a .einb container: EinAf reads a program as text",
                    path.display()
                );
                return Err(vec![finding(&ein_core::Diagnostic::error(
                    ein_core::Kind::Unreadable,
                    message,
                ))]);
            }
            Err(e) => return Err(vec![finding(&e.diagnostic())]),
        },
        Source::Text {
            text,
            name,
            base_dir,
        } => (text.to_string(), name.to_string(), base_dir),
    };

    // The program's own forms: its text parsed alone.
    let mut own = Ast::new();
    let nodes = ein_ir::parse(&mut own, &text, Some(&name))
        .map_err(|e| e.diagnostics().iter().map(finding).collect::<Vec<_>>())?;
    let forms = nodes.iter().map(|&id| form(&own, id)).collect();

    // The program loaded whole, its imports resolved: what the modules bring.
    let mut ast = Ast::new();
    let mut terms = ein_core::Terms::new();
    let loaded = ein_ir::saved::load_text(
        &mut ast,
        &mut terms,
        &text,
        Some(&name),
        base_dir,
        0,
        &ein_ir::LoadOptions::default(),
    )
    .map_err(|e| e.findings().iter().map(finding).collect::<Vec<_>>())?;
    let program = loaded.kb.program();
    let own_rules: BTreeSet<String> = nodes
        .iter()
        .filter(|&&id| matches!(own.head_name(id), Some("rule" | "macro")))
        .filter_map(|&id| own.form_args(id).first().and_then(|&a| own.atom_name(a)))
        .map(str::to_string)
        .collect();
    let mut library = Library {
        modules: nodes
            .iter()
            .filter(|&&id| own.head_name(id) == Some("import"))
            .filter_map(|&id| own.form_args(id).first().and_then(|&a| own.atom_name(a)))
            .map(str::to_string)
            .chain(std::iter::once(ein_ir::imports::CORE.to_string()))
            .collect(),
        ..Library::default()
    };
    for (sym, rule) in program
        .rules
        .iter()
        .chain(program.obligations.iter())
        .chain(program.hypotheses.iter())
    {
        let rule_name = terms.sym(sym);
        if own_rules.contains(rule_name) {
            continue;
        }
        library.rules.insert(rule_name.to_string());
        if let Some(m) = &rule.match_ {
            for &r in m.relation_names.iter() {
                library.reads.insert(terms.sym(r).to_string());
            }
        }
    }
    for d in &program.declarations {
        library
            .declared
            .entry(terms.sym(d.name).to_string())
            .or_default()
            .insert(d.signature.len());
    }
    Ok(Forms {
        name,
        forms,
        library,
    })
}

/// **A text's top-level forms, read off its parse alone**: each as
/// [`forms()`] gives it, with nothing loaded, so that a program a load would
/// refuse (an undeclared module, a missing `:why`) is read all the same; a
/// text that does not parse is its findings.
pub fn shapes(text: &str, name: &str) -> Result<Vec<Form>, Vec<Finding>> {
    let mut own = Ast::new();
    let nodes = ein_ir::parse(&mut own, text, Some(name))
        .map_err(|e| e.diagnostics().iter().map(finding).collect::<Vec<_>>())?;
    Ok(nodes.iter().map(|&id| form(&own, id)).collect())
}

/// One top-level form, by its head.
fn form(ast: &Ast, id: NodeId) -> Form {
    let canonical = ein_ir::dump_canonical(ast, &[id])
        .trim_end_matches('\n')
        .to_string();
    let head = ast.head_name(id).unwrap_or("");
    let args = ast.form_args(id);
    let first_name = || {
        args.first()
            .and_then(|&a| ast.atom_name(a))
            .unwrap_or("")
            .to_string()
    };
    let shape = match head {
        "relation" => Shape::Relation { name: first_name() },
        "rule" => Shape::Rule { name: first_name() },
        "macro" => Shape::Macro { name: first_name() },
        "import" => Shape::Import {
            module: first_name(),
        },
        "query" => {
            let mut reads = Vec::new();
            if let Some(goal) = keyword(ast, args, "goal") {
                goal_reads(ast, goal, &mut reads);
            }
            Shape::Query {
                goal_text: keyword(ast, args, "goal-text").is_some(),
                reads,
            }
        }
        "solve" => Shape::Solve,
        "config" => Shape::Config,
        "trace" => Shape::Trace,
        "not" | "open" => {
            let inner = args
                .first()
                .and_then(|&a| ast.head_name(a))
                .unwrap_or("")
                .to_string();
            Shape::Fact {
                relation: inner,
                written: if head == "not" {
                    Written::Negated
                } else {
                    Written::Open
                },
                source: text_of(ast, keyword(ast, args, "source")),
            }
        }
        relation => Shape::Fact {
            relation: relation.to_string(),
            written: Written::Holds,
            source: text_of(ast, keyword(ast, args, "source")),
        },
    };
    Form { canonical, shape }
}

/// The value of a form's `:key`, where it carries one.
fn keyword(ast: &Ast, args: &[NodeId], key: &str) -> Option<NodeId> {
    args.iter().find_map(|&a| match ast.node(a) {
        Node::KwPair { key: k, value } => match ast.node(k) {
            Node::Keyword(s) if ast.sym(s) == key => Some(value),
            _ => None,
        },
        _ => None,
    })
}

/// A string's text.
fn text_of(ast: &Ast, id: Option<NodeId>) -> Option<String> {
    match ast.node(id?) {
        Node::Str(s) => Some(ast.sym(s).to_string()),
        _ => None,
    }
}

/// The relations a goal reads: each fact's relation, through `and`, `or`,
/// `not`, `absent` and `open`, each once; `eq` and `neq` read none, and a
/// fact that is an argument is no premise.
fn goal_reads(ast: &Ast, id: NodeId, out: &mut Vec<String>) {
    match ast.head_name(id) {
        Some("and" | "or" | "not" | "absent" | "open") => {
            for &a in ast.form_args(id) {
                goal_reads(ast, a, out);
            }
        }
        Some("eq" | "neq") | None => {}
        Some(relation) => {
            if !out.iter().any(|r| r == relation) {
                out.push(relation.to_string());
            }
        }
    }
}
