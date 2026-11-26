//! **A program's vocabulary, read off its parse**: the relations it declares
//! and at what counts, its rules and their parameters, its imports and the
//! names they list, the relations its facts and patterns are written of, and
//! every name it writes. Nothing is loaded and nothing asked: what a grammar
//! of a later step is rendered on ([`crate::grammar::steps`]). And the
//! standard library's modules, as texts, which the grammars' catalogue is
//! read from.

use std::collections::{BTreeMap, BTreeSet};

use ein_ir::{Ast, Node, NodeId};
use serde::{Deserialize, Serialize};

use super::feedback::{Finding, finding};

/// **What a program's text names.**
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vocabulary {
    /// Each relation a `(relation …)` form declares, and the counts of the
    /// labels it is declared with.
    pub declared: BTreeMap<String, BTreeSet<usize>>,
    /// Each rule and macro, and the count of its parameters.
    pub rules: BTreeMap<String, usize>,
    /// Each import, in the order written.
    pub imports: Vec<Imported>,
    /// Each relation a fact, a stored negation, a hypothesis or a pattern is
    /// written of, and the counts of its arguments: a rule's `:match` and
    /// `:assert`, a query's goal, a fact inside a fact.
    pub written: BTreeMap<String, BTreeSet<usize>>,
    /// The relations a rule's `:match` reads, a subset of `written`.
    pub read: BTreeMap<String, BTreeSet<usize>>,
    /// Every name the text writes where a value goes, a declaration's labels
    /// and a fact's relation among them.
    pub names: BTreeSet<String>,
}

/// An `(import MODULE …)` form.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Imported {
    pub module: String,
    /// The names `:symbols` lists; `None` where it lists none.
    pub symbols: Option<Vec<String>>,
    /// The alias `:as` gives.
    pub alias: Option<String>,
}

/// The heads a pattern is no relation under.
const CONNECTIVES: [&str; 8] = ["and", "or", "not", "absent", "open", "eq", "neq", "false"];

/// **Read `text`'s vocabulary**, parsing it as Ein's syntax and loading
/// nothing; a text that does not parse is its findings.
pub fn vocabulary(text: &str, name: &str) -> Result<Vocabulary, Vec<Finding>> {
    let mut ast = Ast::new();
    let forms = ein_ir::parse(&mut ast, text, Some(name))
        .map_err(|e| e.diagnostics().iter().map(finding).collect::<Vec<_>>())?;
    let mut v = Vocabulary::default();
    for &f in &forms {
        top(&ast, f, &mut v);
    }
    Ok(v)
}

/// A form's `:key` value.
fn keyword(ast: &Ast, args: &[NodeId], key: &str) -> Option<NodeId> {
    args.iter().find_map(|&a| match ast.node(a) {
        Node::KwPair { key: k, value } => match ast.node(k) {
            Node::Keyword(s) if ast.sym(s) == key => Some(value),
            _ => None,
        },
        _ => None,
    })
}

/// The arguments of a form that are no key pair.
fn plain(ast: &Ast, args: &[NodeId]) -> Vec<NodeId> {
    args.iter()
        .copied()
        .filter(|&a| !matches!(ast.node(a), Node::KwPair { .. }))
        .collect()
}

fn top(ast: &Ast, f: NodeId, v: &mut Vocabulary) {
    let args = ast.form_args(f);
    match ast.head_name(f) {
        Some("relation") => {
            let p = plain(ast, args);
            if let Some(name) = p.first().and_then(|&a| ast.atom_name(a)) {
                v.declared
                    .entry(name.to_string())
                    .or_default()
                    .insert(p.len() - 1);
                for &a in &p {
                    if let Some(n) = ast.atom_name(a) {
                        v.names.insert(n.to_string());
                    }
                }
            }
        }
        Some("rule" | "macro") => {
            if let Some(name) = args.first().and_then(|&a| ast.atom_name(a)) {
                let params = args.get(1).map_or(0, |&p| ast.form_args(p).len());
                v.rules.insert(name.to_string(), params);
            }
            if let Some(m) = keyword(ast, args, "match") {
                pattern(ast, m, v, true);
            }
            if let Some(a) = keyword(ast, args, "assert") {
                pattern(ast, a, v, false);
            }
        }
        Some("import") => {
            let module = args
                .first()
                .and_then(|&a| ast.atom_name(a))
                .unwrap_or("")
                .to_string();
            let symbols = keyword(ast, args, "symbols").map(|l| {
                let mut out: Vec<String> =
                    ast.head_name(l).map(str::to_string).into_iter().collect();
                out.extend(
                    ast.form_args(l)
                        .iter()
                        .filter_map(|&a| ast.atom_name(a))
                        .map(str::to_string),
                );
                out
            });
            let alias = keyword(ast, args, "as")
                .and_then(|a| ast.atom_name(a))
                .map(str::to_string);
            v.imports.push(Imported {
                module,
                symbols,
                alias,
            });
        }
        Some("query") => {
            if let Some(g) = keyword(ast, args, "goal") {
                pattern(ast, g, v, false);
            }
        }
        Some("solve" | "config" | "trace" | "program" | "module" | "commit") => {}
        _ => pattern(ast, f, v, false),
    }
}

/// A fact, a pattern or a connective over them.
fn pattern(ast: &Ast, id: NodeId, v: &mut Vocabulary, read: bool) {
    match ast.node(id) {
        Node::Atom(s) => {
            let n = ast.sym(s);
            if !n.starts_with('#') {
                v.names.insert(n.to_string());
            }
        }
        Node::SForm { .. } => {
            let args = plain(ast, ast.form_args(id));
            match ast.head_name(id) {
                Some(h) if CONNECTIVES.contains(&h) || h.starts_with('#') => {
                    for a in args {
                        pattern(ast, a, v, read);
                    }
                }
                Some(h) => {
                    v.written
                        .entry(h.to_string())
                        .or_default()
                        .insert(args.len());
                    if read {
                        v.read.entry(h.to_string()).or_default().insert(args.len());
                    }
                    v.names.insert(h.to_string());
                    for a in args {
                        pattern(ast, a, v, read);
                    }
                }
                // A variable's relation.
                None => {
                    for a in args {
                        pattern(ast, a, v, read);
                    }
                }
            }
        }
        _ => {}
    }
}

/// **The standard library's modules**, each by its name (`std.algebra`,
/// `std.math.decimal-arith`) and its text, as the kernel resolves the
/// library; sorted by name.
pub fn library() -> Vec<(String, String)> {
    let source = ein_ir::stdlib::resolve_default();
    let mut out: Vec<(String, String)> = source
        .modules()
        .into_iter()
        .filter_map(|rel| {
            let text = source.read(&rel)?;
            let name = format!("std.{}", rel.trim_end_matches(".ein").replace('/', "."));
            Some((name, text))
        })
        .collect();
    out.sort();
    out
}

/// **A rule a text writes**, as its own form says it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleDoc {
    pub name: String,
    /// Its parameters, each a variable's name without its `?`; `[]` for a
    /// rule that fires on every match.
    pub params: Vec<String>,
    /// Its own `:why`, as written; `null` where it has none.
    pub why: Option<String>,
    /// `rule` or `macro`.
    pub form: String,
    /// What it concludes: its `:assert` as `ein_ir::dump_canonical` prints
    /// it, or `:ein` and the engine's word; `null` for a macro.
    pub concludes: Option<String>,
}

/// **Each `rule` and `macro` form of `text`**, its name, its parameters and
/// its `:why`, read off its parse; nothing loaded. The catalogue's generated
/// part reads the library's modules so.
pub fn rule_docs(text: &str, name: &str) -> Result<Vec<RuleDoc>, Vec<Finding>> {
    let mut ast = Ast::new();
    let forms = ein_ir::parse(&mut ast, text, Some(name))
        .map_err(|e| e.diagnostics().iter().map(finding).collect::<Vec<_>>())?;
    let mut out = Vec::new();
    for &f in &forms {
        let form = match ast.head_name(f) {
            Some(h @ ("rule" | "macro")) => h.to_string(),
            _ => continue,
        };
        let args = ast.form_args(f);
        let Some(rule) = args.first().and_then(|&a| ast.atom_name(a)) else {
            continue;
        };
        let params = args.get(1).map_or_else(Vec::new, |&p| {
            ast.form_args(p)
                .iter()
                .filter_map(|&a| match ast.node(a) {
                    Node::Var(s) => Some(ast.sym(s).trim_start_matches('?').to_string()),
                    Node::Atom(s) => Some(ast.sym(s).to_string()),
                    _ => None,
                })
                .collect()
        });
        let why = keyword(&ast, args, "why").and_then(|w| match ast.node(w) {
            Node::Str(s) => Some(ast.sym(s).to_string()),
            _ => None,
        });
        let concludes = keyword(&ast, args, "assert")
            .map(|a| {
                ein_ir::dump_canonical(&ast, &[a])
                    .trim_end_matches('\n')
                    .to_string()
            })
            .or_else(|| {
                keyword(&ast, args, "ein")
                    .and_then(|w| ast.atom_name(w))
                    .map(|w| format!(":ein {w}"))
            });
        out.push(RuleDoc {
            name: rule.to_string(),
            params,
            why,
            form,
            concludes,
        });
    }
    Ok(out)
}
