//! Pattern macros — `ir/macros.py`, a load-time AST rewrite.
//!
//! `(macro NAME (?p…) BODY)` is an alias: an invocation `(NAME a…)` in a rule
//! clause is replaced by a copy of BODY under the parameter binding *before
//! the compiler sees it*. That is how `forall` and `open` exist as ein source
//! (`stdlib/macro.ein`) rather than as arms in the compiler — and it is why
//! any Rust code that special-cases those two heads would be a bug: by the
//! time compilation runs they are already `(absent (and G (absent B)))` and
//! `(and (absent P) (absent (not P)))`.
//!
//! Substitution walks the arena and rebuilds, so a macro body is copied as
//! integers rather than cloned as a graph.

use std::collections::BTreeMap;

use crate::ast::{Ast, Node, NodeId, loc_repr};

/// A macro that expands to itself would recurse forever; cap and reject
/// (`_MAX_EXPANSION_DEPTH`, Q-S1.5.9.3).
const MAX_EXPANSION_DEPTH: u32 = 50;

/// A malformed invocation — arity mismatch or runaway recursion. The message
/// is `MacroError`'s; the loader's `({head} {name}): ` prefix is added where
/// the loader adds it ([P1a.2](../../../../docs/history/m1a_rust/README.md#p1a2--kb-core)).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroError(pub String);

impl std::fmt::Display for MacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for MacroError {}

/// A `(macro NAME (params…) BODY)` declaration.
#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    /// The body's substitution variables, in declaration order.
    pub params: Vec<String>,
    pub body: NodeId,
}

/// The `{name: Macro}` registry a form list declares.
///
/// First declaration wins. ein.py's loader *rejects* a duplicate (and a name
/// that shadows kernel vocabulary) in `_ingest_macros`; both are loader
/// checks and land with the loader.
pub fn collect_macros(ast: &Ast, forms: &[NodeId]) -> BTreeMap<String, Macro> {
    let mut out: BTreeMap<String, Macro> = BTreeMap::new();
    for &form in forms {
        if ast.head_name(form) != Some("macro") {
            continue;
        }
        let args = ast.form_args(form);
        let [name, params, body] = args else { continue };
        let Some(name) = ast.atom_name(*name) else {
            continue;
        };
        let params: Vec<String> = ast
            .form_args(*params)
            .iter()
            .filter_map(|p| match ast.node(*p) {
                Node::Var(s) => Some(ast.sym(s).to_string()),
                _ => None,
            })
            .collect();
        out.entry(name.to_string()).or_insert(Macro {
            name: name.to_string(),
            params,
            body: *body,
        });
    }
    out
}

/// Rewrite every macro invocation reachable from `node`.
///
/// A form whose head atom names a macro becomes the substituted body, which is
/// then **re-expanded** (a body may invoke other macros — handled inside-out).
/// Everything else is walked into, so an invocation nested anywhere in a
/// clause is found.
pub fn expand_macros(
    ast: &mut Ast,
    node: NodeId,
    macros: &BTreeMap<String, Macro>,
) -> Result<NodeId, MacroError> {
    expand(ast, node, macros, 0)
}

fn expand(
    ast: &mut Ast,
    node: NodeId,
    macros: &BTreeMap<String, Macro>,
    depth: u32,
) -> Result<NodeId, MacroError> {
    if depth > MAX_EXPANSION_DEPTH {
        return Err(MacroError(format!(
            "macro expansion exceeded depth {MAX_EXPANSION_DEPTH} — \
             a macro likely expands to itself"
        )));
    }
    match ast.node(node) {
        Node::SForm { head, args } => {
            let name = ast.atom_name(head).map(str::to_string);
            if let Some(m) = name.as_deref().and_then(|n| macros.get(n)) {
                // Trailing kw-pairs are metadata (dropped downstream by the
                // compiler — Q32) and do not count toward arity.
                let positional: Vec<NodeId> = ast
                    .args(args)
                    .iter()
                    .copied()
                    .filter(|a| !matches!(ast.node(*a), Node::KwPair { .. }))
                    .collect();
                if positional.len() != m.params.len() {
                    return Err(MacroError(format!(
                        "macro {}/{} invoked with {} args at {}",
                        m.name,
                        m.params.len(),
                        positional.len(),
                        loc_repr(ast, ast.loc(node)),
                    )));
                }
                let subst: BTreeMap<&str, NodeId> = m
                    .params
                    .iter()
                    .map(String::as_str)
                    .zip(positional.iter().copied())
                    .collect();
                let body = m.body;
                let expanded = substitute(ast, body, &subst);
                return expand(ast, expanded, macros, depth + 1);
            }
            let args: Vec<NodeId> = ast.args(args).to_vec();
            let mut new_args = Vec::with_capacity(args.len());
            for a in args {
                new_args.push(expand(ast, a, macros, depth)?);
            }
            let loc = ast.loc(node);
            Ok(ast.sform(head, &new_args, loc))
        }
        Node::KwPair { key, value } => {
            let value = expand(ast, value, macros, depth)?;
            let loc = ast.loc(node);
            Ok(ast.push(Node::KwPair { key, value }, loc))
        }
        _ => Ok(node),
    }
}

/// `template` with each `Var` named in `subst` replaced by its node.
///
/// A parameter may appear in **head** position (`(?R ?a ?b)` — a
/// relation-polymorphic body); it is substituted there too, but only when the
/// replacement is atom-shaped, because a head must stay one. M1's `forall` /
/// `open` never use a var head; the arm is forward-compat for the
/// `imply` / `converse` family (S1.8.A7).
fn substitute(ast: &mut Ast, template: NodeId, subst: &BTreeMap<&str, NodeId>) -> NodeId {
    match ast.node(template) {
        Node::Var(s) => match subst.get(ast.sym(s)) {
            Some(&replacement) => replacement,
            None => template,
        },
        Node::SForm { head, args } => {
            let mut new_head = head;
            if let Node::Var(s) = ast.node(head)
                && let Some(&sub) = subst.get(ast.sym(s))
                && matches!(ast.node(sub), Node::Atom(_) | Node::Var(_))
            {
                new_head = sub;
            }
            let args: Vec<NodeId> = ast.args(args).to_vec();
            let new_args: Vec<NodeId> = args
                .into_iter()
                .map(|a| substitute(ast, a, subst))
                .collect();
            let loc = ast.loc(template);
            ast.sform(new_head, &new_args, loc)
        }
        Node::KwPair { key, value } => {
            let value = substitute(ast, value, subst);
            let loc = ast.loc(template);
            ast.push(Node::KwPair { key, value }, loc)
        }
        // Atom / Wildcard / Int / Range / Str / Keyword are leaves.
        _ => template,
    }
}

/// Expand the `:match` and `:assert` clauses of every `(rule …)` / `(hrule …)`
/// in `forms`, leaving everything else alone — what the loader does, and
/// therefore the shape the parity gate compares.
///
/// Note what is *not* expanded: a `(forall …)` appearing as a **fact** stays
/// as it is, because the loader only ever runs the expander over rule clauses.
pub fn expand_rule_clauses(
    ast: &mut Ast,
    forms: &[NodeId],
    macros: &BTreeMap<String, Macro>,
) -> Result<Vec<NodeId>, MacroError> {
    if macros.is_empty() {
        return Ok(forms.to_vec());
    }
    let mut out = Vec::with_capacity(forms.len());
    for &form in forms {
        if !matches!(ast.head_name(form), Some("rule") | Some("hrule")) {
            out.push(form);
            continue;
        }
        let args: Vec<NodeId> = ast.form_args(form).to_vec();
        let mut new_args = Vec::with_capacity(args.len());
        for a in args {
            let expanded = match ast.node(a) {
                Node::KwPair { key, value } => {
                    let name = match ast.node(key) {
                        Node::Keyword(s) => ast.sym(s).to_string(),
                        _ => String::new(),
                    };
                    if name == "match" || name == "assert" {
                        let value = expand_macros(ast, value, macros)?;
                        let loc = ast.loc(a);
                        ast.push(Node::KwPair { key, value }, loc)
                    } else {
                        a
                    }
                }
                _ => a,
            };
            new_args.push(expanded);
        }
        let head = match ast.node(form) {
            Node::SForm { head, .. } => head,
            _ => unreachable!("checked above"),
        };
        let loc = ast.loc(form);
        out.push(ast.sform(head, &new_args, loc));
    }
    Ok(out)
}

/// The S1.8a.f20 guard: a rule whose `:match` names a `std.macro` macro that
/// was never imported.
///
/// Unexpanded, the invocation survives into the compiled pattern and the rule
/// **silently never fires** — the failure mode this check exists for. It is
/// deliberately narrow: it does *not* require every match head to resolve,
/// because rules legitimately match optional marker relations (`functional`,
/// `total`, `hypothesis`, …) whose absence just means the rule does not fire.
///
/// Returns the message lines; wiring them into the loader's error list is
/// [P1a.2](../../../../docs/history/m1a_rust/README.md#p1a2--kb-core)'s.
pub fn unimported_macro_errors(
    ast: &Ast,
    rules: &[(String, NodeId)],
    declared: &BTreeMap<String, Macro>,
    stdlib_macros: &[String],
) -> Vec<String> {
    let unimported: Vec<&String> = stdlib_macros
        .iter()
        .filter(|n| !declared.contains_key(*n))
        .collect();
    if unimported.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for (name, match_node) in rules {
        let mut heads = Vec::new();
        concrete_heads(ast, *match_node, &mut heads);
        for head in &unimported {
            if heads.iter().any(|h| h == *head) {
                out.push(format!(
                    "rule '{name}': '({head} …)' is a std.macro pattern macro used \
                     without importing it — add (import std.macro :symbols ({head})); \
                     unexpanded it would silently never match"
                ));
            }
        }
    }
    out
}

/// Atom head-names of every form in a pattern. A variable head (`(?R …)`)
/// contributes nothing — it matches any relation.
fn concrete_heads(ast: &Ast, node: NodeId, out: &mut Vec<String>) {
    if let Node::SForm { head, args } = ast.node(node) {
        if let Some(name) = ast.atom_name(head) {
            let name = name.to_string();
            if !out.contains(&name) {
                out.push(name);
            }
        }
        for a in ast.args(args) {
            concrete_heads(ast, *a, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dump::dump_compact;
    use crate::parse::parse;

    fn expand_src(src: &str) -> Result<String, MacroError> {
        let mut ast = Ast::new();
        let forms = parse(&mut ast, src, None).expect("parses");
        let macros = collect_macros(&ast, &forms);
        let out = expand_rule_clauses(&mut ast, &forms, &macros)?;
        Ok(out
            .iter()
            .map(|f| dump_compact(&ast, *f))
            .collect::<Vec<_>>()
            .join("\n"))
    }

    const FORALL: &str = "(macro forall (?g ?b) (absent (and ?g (absent ?b))))";

    #[test]
    fn an_invocation_becomes_its_body_under_the_binding() {
        let out = expand_src(&format!(
            "{FORALL}\n(rule r () :match (forall (p ?x) (q ?x)) :assert (done))"
        ))
        .expect("expands");
        assert!(
            out.contains(":match (absent (and (p ?x) (absent (q ?x))))"),
            "{out}"
        );
    }

    #[test]
    fn only_rule_clauses_are_expanded() {
        // A `(forall …)` *fact* is left alone: the loader only ever runs the
        // expander over `:match` / `:assert`.
        let out = expand_src(&format!("{FORALL}\n(forall a b)")).expect("expands");
        assert!(out.contains("(forall a b)"), "{out}");
    }

    #[test]
    fn arity_is_counted_over_positional_args_only() {
        let err = expand_src(&format!(
            "{FORALL}\n(rule r () :match (forall (p ?x)) :assert (done))"
        ))
        .expect_err("arity mismatch");
        assert!(
            err.0
                .starts_with("macro forall/2 invoked with 1 args at Loc("),
            "{}",
            err.0
        );
        // Trailing kw-pairs are metadata, not arguments.
        expand_src(&format!(
            "{FORALL}\n(rule r () :match (forall (p ?x) (q ?x) :why \"m\") :assert (done))"
        ))
        .expect("kw-pairs do not count");
    }

    #[test]
    fn a_self_expanding_macro_hits_the_depth_cap() {
        let err =
            expand_src("(macro loop (?a) (loop ?a))\n(rule r () :match (loop x) :assert (done))")
                .expect_err("runaway");
        assert_eq!(
            err.0,
            "macro expansion exceeded depth 50 — a macro likely expands to itself"
        );
    }

    #[test]
    fn the_unimported_guard_is_narrow() {
        let mut ast = Ast::new();
        let forms = parse(
            &mut ast,
            "(rule undefeated () :match (forall (p ?x) (q ?x)) :assert (d))",
            None,
        )
        .expect("parses");
        let declared = collect_macros(&ast, &forms);
        let match_node = ast
            .form_args(forms[0])
            .iter()
            .find_map(|a| match ast.node(*a) {
                Node::KwPair { key, value } => match ast.node(key) {
                    Node::Keyword(s) if ast.sym(s) == "match" => Some(value),
                    _ => None,
                },
                _ => None,
            })
            .expect("a :match");
        let rules = vec![("undefeated".to_string(), match_node)];
        let stdlib = vec!["forall".to_string(), "open".to_string()];
        let errs = unimported_macro_errors(&ast, &rules, &declared, &stdlib);
        assert_eq!(errs.len(), 1, "{errs:?}");
        assert!(
            errs[0].starts_with("rule 'undefeated': '(forall …)'"),
            "{}",
            errs[0]
        );
        // `p` and `q` are ordinary heads: absence is not an error.
        assert!(!errs[0].contains("'(p …)'"));
    }
}
