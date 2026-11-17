//! Pattern macros — a load-time AST rewrite.
//!
//! `(macro NAME (?p…) BODY)` is an alias: an invocation `(NAME a…)` in a rule
//! clause is replaced by a copy of BODY under the parameter binding *before
//! the compiler sees it*. That is how `forall` and `unknown` exist as ein
//! source (`std.macro`, `stdlib/macro.ein`) rather than as arms in the
//! compiler — and it is why any Rust code that special-cases those two heads
//! would be a bug: by the time compilation runs they are already
//! `(absent (and G (absent B)))` and `(and (absent P) (absent (not P)))`.
//!
//! Substitution walks the arena and rebuilds, so a macro body is copied as
//! integers rather than cloned as a graph.

use std::collections::BTreeMap;

use crate::ast::{Ast, Node, NodeId, loc_repr};

/// A macro that expands to itself would recurse forever, so expansion is
/// capped at this depth and refused past it.
const MAX_EXPANSION_DEPTH: u32 = 50;

/// A malformed invocation — arity mismatch or runaway recursion. The message
/// is `MacroError`'s; the loader's `({head} {name}): ` prefix is added where
/// the loader adds it ([design/data-model.md](../../../../docs/ein/ein.rs/design/data-model.md)).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroError(pub String);

impl std::fmt::Display for MacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for MacroError {}

/// A `(macro NAME (params…) BODY)` declaration, with its `:why` where it
/// has one.
#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    /// The body's substitution variables, in declaration order.
    pub params: Vec<String>,
    pub body: NodeId,
    /// `(macro NAME (params…) BODY :why "…")`'s template, which a macro
    /// carries as a relation and a rule do: every form an NL template can go
    /// on takes one (Q-M1f.59).
    pub why: Option<String>,
    /// The declaring form, for the `Loc` the loader stores on the program.
    pub form: NodeId,
}

/// The `{name: Macro}` registry a form list declares, and what is wrong with
/// it.
///
/// **One reading, and this is it**: the loader reads a program's macros here
/// ([`read_macros`]), and so does every other consumer, so each refuses
/// exactly what a load refuses and says the same sentence.
///
/// A lenient reading — the first declaration of a duplicate name taken, a
/// malformed form skipped — would let a *dump* render something where a load
/// refuses. What it would show is a program with two `(macro m …)` rendered
/// with the expansion of whichever came first: a rendering of a program that
/// cannot be run.
///
/// Errors are **accumulated**, not returned at the first: a load reports every
/// problem it found, and this is the half of one.
pub fn collect_macros(ast: &Ast, forms: &[NodeId]) -> Result<BTreeMap<String, Macro>, Vec<String>> {
    match read_macros(ast, forms) {
        (out, problems) if problems.is_empty() => Ok(out),
        (_, problems) => Err(problems),
    }
}

/// [`collect_macros`] without the verdict — every declaration it could read,
/// beside everything wrong with the ones it could not.
///
/// The loader's shape: a load reports **all** of its problems rather than the
/// first, and the passes after this one still want the well-formed
/// declarations while it does. `collect_macros` is the same reading with the
/// two halves turned into a `Result`, for the callers that only want to know
/// whether the program is loadable.
pub fn read_macros(ast: &Ast, forms: &[NodeId]) -> (BTreeMap<String, Macro>, Vec<String>) {
    let mut out: BTreeMap<String, Macro> = BTreeMap::new();
    let mut errors: Vec<String> = Vec::new();
    for &form in forms {
        if ast.head_name(form) != Some("macro") {
            continue;
        }
        let loc = loc_repr(ast, ast.loc(form));
        let args = ast.form_args(form);
        // A macro is a name, a parameter list and a body (`ein.y`'s `Macro`):
        // the grammar refuses one with fewer than three arguments, or whose
        // name or parameters are not one, so all three are here.
        let (name, params, body) = (args[0], args[1], args[2]);
        let name = ast
            .atom_name(name)
            .expect("a macro's first argument is its name (ein.y's Macro)");
        if ein_core::is_reserved(name) {
            errors.push(format!(
                "macro '{name}' shadows a reserved kernel name at {loc}"
            ));
            continue;
        }
        if out.contains_key(name) {
            errors.push(format!("duplicate macro '{name}' at {loc}"));
            continue;
        }
        let params: Vec<String> = ast
            .form_args(params)
            .iter()
            .filter_map(|p| match ast.node(*p) {
                Node::Var(s) => Some(ast.sym(s).to_string()),
                _ => None,
            })
            .collect();
        // What follows the body: `:why "…"`, the one clause the grammar lets
        // a macro take after it.
        let why = args[3..].iter().find_map(|&a| match ast.node(a) {
            Node::KwPair { key, value } => match (ast.node(key), ast.node(value)) {
                (Node::Keyword(k), Node::Str(v)) if ast.sym(k) == "why" => {
                    Some(ast.sym(v).to_string())
                }
                _ => None,
            },
            _ => None,
        });
        out.insert(
            name.to_string(),
            Macro {
                name: name.to_string(),
                params,
                body,
                why,
                form,
            },
        );
    }
    (out, errors)
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
                // Every argument of a call is one of the macro's: a call is a
                // premise or a conclusion (`ein.y`'s `Premise` and
                // `Conclusion`), which holds no keyword pair, so
                // `(forall G B :k v)` does not parse and every argument counts
                // toward the arity.
                let actual: Vec<NodeId> = ast.args(args).to_vec();
                if actual.len() != m.params.len() {
                    return Err(MacroError(format!(
                        "macro {}/{} invoked with {} args at {}",
                        m.name,
                        m.params.len(),
                        actual.len(),
                        loc_repr(ast, ast.loc(node)),
                    )));
                }
                let subst: BTreeMap<&str, NodeId> = m
                    .params
                    .iter()
                    .map(String::as_str)
                    .zip(actual.iter().copied())
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
        // A pair's value is walked as a form's arguments are. The expander is
        // handed a clause's value, which holds no pair, so no program reaches
        // the arm; the walk keeps it, total over the tree.
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
/// replacement is atom-shaped, because a head must stay one. `std.macro`'s
/// `forall` and `unknown` never use a var head; the arm serves a macro whose
/// body is relation-polymorphic.
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
        // A template holds no pair (`ein.y`'s `Template`), so no program
        // reaches the arm; the walk keeps it, as `expand` does.
        Node::KwPair { key, value } => {
            let value = substitute(ast, value, subst);
            let loc = ast.loc(template);
            ast.push(Node::KwPair { key, value }, loc)
        }
        // Atom / Str / Keyword are leaves.
        _ => template,
    }
}

/// Expand the `:match` and `:assert` clauses of every `(rule …)` / `(hrule …)`
/// in `forms`, leaving everything else alone — the same clauses
/// [`crate::from_ir`] expands, one rule at a time, through the same
/// [`expand_macros`].
///
/// Note what is *not* expanded: a `(forall …)` appearing as a **fact** stays
/// as it is, because the loader only ever runs the expander over rule clauses
/// and a goal. The loader refuses such a fact (`from_ir`'s
/// `refuse_macro_facts`, W86): a macro's call is no fact
/// ([`diagnostics.md` § 2.24](../../../../docs/ein/user/reference/diagnostics.md#224-a-macros-call-is-no-fact)).
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

/// The unimported-macro guard: a rule whose `:match`, or a query whose
/// `:goal`, names a `std.macro` macro that was never imported (Q-M1f.132). `rules`
/// pairs who writes the premise, `rule 'r'` or `query`, with the premise.
///
/// Unexpanded, the invocation survives into the compiled pattern and the rule
/// **silently never fires** — the failure mode this check exists for. It is
/// deliberately narrow: it does *not* require every match head to resolve,
/// because rules legitimately match optional marker relations (`functional`,
/// `total`, `hypothesis`, …) whose absence just means the rule does not fire.
///
/// Returns the message lines; wiring them into the loader's error list is
/// [design/data-model.md](../../../../docs/ein/ein.rs/design/data-model.md)'s.
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
    for (who, match_node) in rules {
        let mut heads = Vec::new();
        concrete_heads(ast, *match_node, &mut heads);
        for head in &unimported {
            if heads.iter().any(|h| h == *head) {
                out.push(format!(
                    "{who}: '({head} …)' is a std.macro pattern macro used \
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
    use crate::parse;

    fn expand_src(src: &str) -> Result<String, MacroError> {
        let mut ast = Ast::new();
        let forms = parse(&mut ast, src, None).expect("parses");
        let macros = collect_macros(&ast, &forms).expect("the fixture is well-formed");
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
        // expander over `:match` / `:assert`, and refuses the fact.
        let out = expand_src(&format!("{FORALL}\n(forall a b)")).expect("expands");
        assert!(out.contains("(forall a b)"), "{out}");
    }

    /// The arity of an invocation is its every argument: no keyword pair
    /// parses inside a term, so a call carries none and the parse refuses one.
    #[test]
    fn arity_is_counted_over_every_argument() {
        let err = expand_src(&format!(
            "{FORALL}\n(rule r () :match (forall (p ?x)) :assert (done))"
        ))
        .expect_err("arity mismatch");
        // The position in Ein's spelling (`loc_repr`, Q-M1f.75).
        assert_eq!(err.0, "macro forall/2 invoked with 1 args at <string>:2:20");
        // A keyword pair in a call is refused at the parse, at its keyword.
        let refused = parse(
            &mut Ast::new(),
            &format!(
                "{FORALL}\n(rule r () :match (forall (p ?x) (q ?x) :why \"m\") :assert (done))"
            ),
            None,
        )
        .expect_err("a keyword pair in a call");
        assert_eq!(
            (
                refused.errors[0].line,
                refused.errors[0].col,
                refused.errors[0].what.as_str()
            ),
            (2, 41, "unexpected `:why`"),
            "{refused}"
        );
        expand_src(&format!(
            "{FORALL}\n(rule r () :match (forall (p ?x) (q ?x)) :assert (done))"
        ))
        .expect("two positional arguments");
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
        let declared = collect_macros(&ast, &forms).expect("the fixture is well-formed");
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
        let rules = vec![("rule 'undefeated'".to_string(), match_node)];
        let stdlib = vec!["forall".to_string(), "unknown".to_string()];
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
