//! IR → `Kb` — `ein/kb/from_ir.py`.
//!
//! The loader walks a **flat sequence** of top-level forms and classifies each
//! by its head: `relation` declares one, `rule` / `hrule` declare a rule,
//! `macro` a pattern macro, `query` / `config` fill their slots, `trace` is
//! engine output and is ignored, and **anything else is a fact**. The former
//! block wrappers are gone (S1.7c.4): a `(facts …)` form loads as a fact whose
//! relation is `facts`, like any other head.
//!
//! It is a validation surface as much as a construction one, and that is what
//! makes it a byte-parity target: errors **accumulate** and surface as one
//! `; `-joined message, so both the text of each error and the order they were
//! found in are observable. `examples/broken/load/` is the fixture set.
//!
//! It lives in `ein-ir` rather than beside the store because it needs the
//! frontend and the data model at once, and `ein-core` depends on nothing.
//! `ein.py` puts `imports.py` in `kb/` for the mirror-image reason; the port
//! already moved that one here, and this follows it.

use crate::ast::{Ast, Node, NodeId, loc_repr, node_repr};
use crate::imports::Resolver;
use crate::macros::{self, Macro as IrMacro, MacroError};
use crate::parse::parse;
use ein_core::config::{FIELDS, FieldKind, SolverConfig};
use ein_core::entities::{ExprRef, Loc, Macro, Pattern, Query, Relation, Rule};
use ein_core::{
    FactId, Kb, Overflow, Program, Prov, ProvId, Symbol, Terms, Value, detect_provenance_cycles,
    is_predicate, is_reserved, python_float, python_int,
};
use std::collections::BTreeMap;
use std::path::Path;

/// The accumulated problems, `; `-joined — `KBLoadError`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KbLoadError(pub String);

impl std::fmt::Display for KbLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for KbLoadError {}

impl From<crate::imports::LoadError> for KbLoadError {
    fn from(e: crate::imports::LoadError) -> Self {
        KbLoadError(e.0)
    }
}

impl From<Overflow> for KbLoadError {
    fn from(e: Overflow) -> Self {
        KbLoadError(e.to_string())
    }
}

/// Parse and load a `.ein` file, resolving its imports file-relative.
pub fn load_file(ast: &mut Ast, terms: &mut Terms, path: &Path) -> Result<Kb, KbLoadError> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| KbLoadError(format!("{}: {e}", path.display())))?;
    let forms = parse(ast, &text, path.to_str()).map_err(|e| KbLoadError(e.to_string()))?;
    load(ast, terms, &forms, path.parent())
}

/// Build a populated `Kb` from parsed IR forms.
pub fn load(
    ast: &mut Ast,
    terms: &mut Terms,
    forms: &[NodeId],
    base_dir: Option<&Path>,
) -> Result<Kb, KbLoadError> {
    let mut kb = Kb::new(Program::new());
    let mut errors: Vec<String> = Vec::new();

    // Imports resolve up front into one flat, import-free, qualified stream.
    // Resolution errors are **fatal** and return immediately: a half-resolved
    // program cannot be ingested.
    let forms = Resolver::new().resolve_imports(ast, forms, base_dir)?;

    let (mut relations, mut rules, mut macro_forms, mut facts) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let (mut query_blocks, mut config_blocks): (Vec<NodeId>, Vec<NodeId>) =
        (Vec::new(), Vec::new());
    for &form in &forms {
        let Some(head) = ast.head_name(form).map(str::to_string) else {
            errors.push(format!(
                "unexpected top-level form: {}",
                node_repr(ast, form)
            ));
            continue;
        };
        match head.as_str() {
            "relation" => relations.push(form),
            "rule" | "hrule" => rules.push(form),
            "macro" => macro_forms.push(form),
            "import" => errors.push(format!(
                "unresolved (import …) at {} — internal error",
                loc_repr(ast, ast.loc(form))
            )),
            "query" => query_blocks.push(form),
            "config" => config_blocks.push(form),
            // Engine-emitted output; parsed by the trace reader, not here.
            "trace" => {}
            _ => facts.push(form),
        }
    }

    // Pass 0 — macros, first, because the rules pass expands invocations.
    let declared_macros = ingest_macros(ast, terms, &mut kb, &macro_forms, &mut errors)?;
    // Pass 1 — relations, with their auto-stored declaration facts.
    for form in relations {
        ingest_relation(ast, terms, &mut kb, form, &mut errors)?;
    }
    // Pass 2 — rules. After this, rule-name resolution is possible.
    ingest_rules(ast, terms, &mut kb, &rules, &declared_macros, &mut errors)?;
    // Pass 3 — facts.
    for form in facts {
        ingest_fact(ast, terms, &mut kb, form, &mut errors)?;
    }

    // Last one wins, for both blocks.
    if let Some(&last) = query_blocks.last() {
        let kw_pairs = ast.form_args(last).iter().map(|n| ExprRef(n.0)).collect();
        kb.program_mut().query = Some(Query { kw_pairs });
    }
    if let Some(&last) = config_blocks.last() {
        let args: Vec<NodeId> = ast.form_args(last).to_vec();
        match config_from_kw_pairs(ast, &args) {
            Ok(config) => kb.program_mut().config = Some(config),
            Err(e) => errors.push(format!("(config …): {e}")),
        }
    }

    // The S1.8a.f20 guard: a `(forall …)` / `(open …)` used without importing
    // `std.macro` would leave the invocation in place and the rule would
    // silently never fire.
    let mut rule_matches: Vec<(String, NodeId)> = Vec::new();
    for registry in [&kb.program().rules, &kb.program().hrules] {
        for (name, rule) in registry.iter() {
            if let Some(p) = rule.match_.as_ref() {
                rule_matches.push((terms.sym(name).to_string(), NodeId(p.expr.0)));
            }
        }
    }
    errors.extend(macros::unimported_macro_errors(
        ast,
        &rule_matches,
        &declared_macros,
        &Resolver::new().stdlib_macro_names(),
    ));

    intern_program_names(ast, terms, &kb);

    kb.rebuild_indexes(terms);

    // User-authored `:using` chains can be circular, which would break every
    // derivation walk; reject them with the rendered path.
    let cycles = detect_provenance_cycles(&kb, terms);
    if let Some(cycle) = cycles.first() {
        let path: Vec<String> = cycle.iter().map(|&f| terms.compact(f)).collect();
        errors.push(format!("derivation cycle: {}", path.join(" -> ")));
    }

    if errors.is_empty() {
        Ok(kb)
    } else {
        Err(KbLoadError(errors.join("; ")))
    }
}

/// Intern every name a **rule** or the `(query …)` block mentions, so that
/// nothing is left for the compiler to intern later.
///
/// The compiler resolves a pattern's leaves against `Terms` the first time it
/// builds a plan for a rule, and a constant that appears only inside a rule —
/// `Ann` in `(hrule guess … :assert (seat Ann ?v))`, which no fact mentions —
/// is therefore first seen *during the search*. That is one symbol on one
/// corpus file, and it would be a curiosity except for what
/// [P1a.7](../../../../plans/m1a_rust/p1a.7_parallelism/README.md) needs:
/// [`Interner::text`](ein_core::Interner::text) hands out a `&str` borrowed
/// from the arena, so an interner that is *shared* must be one that does not
/// **grow**, and the search is exactly where it would be shared
/// ([S1a.7.1](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.1_sync_shared_state.md)).
///
/// Deliberately a **superset** of what the compiler asks for, walked off the
/// registered rules rather than the raw forms so macro expansion has already
/// happened. Interning a name nothing goes on to use costs a span and a map
/// entry and is invisible: symbol ids are never observable, and
/// [`Interner::rank`](ein_core::Interner::rank) is content-ordered, so adding
/// an entry cannot reorder the ones already there. Mirroring the compiler's
/// exact leaf set instead would be a second opinion about which leaves it
/// reads, kept in a different file.
///
/// What it does not close is a *seeded* head — `(?rel ?a ?b)` where `?rel`
/// binds to an integer, whose decimal text the compiler interns as a symbol.
/// That needs a run to know, and `ein-infer/tests/interning.rs` is where the
/// residue is measured rather than assumed.
fn intern_program_names(ast: &Ast, terms: &mut Terms, kb: &Kb) {
    fn walk(ast: &Ast, terms: &mut Terms, node: NodeId) {
        match ast.node(node) {
            Node::Atom(s) | Node::Var(s) | Node::Str(s) => {
                let _ = terms.intern_text(ast.sym(s));
            }
            Node::Int(s) => {
                let _ = terms.value_int(ast.sym(s));
            }
            Node::Keyword(_) | Node::Wildcard | Node::Range { .. } => {}
            Node::KwPair { key, value } => {
                walk(ast, terms, key);
                walk(ast, terms, value);
            }
            Node::SForm { head, args } => {
                walk(ast, terms, head);
                for &a in ast.args(args) {
                    walk(ast, terms, a);
                }
            }
        }
    }

    let program = kb.program();
    let mut roots: Vec<NodeId> = Vec::new();
    for registry in [&program.rules, &program.hrules] {
        for (_, rule) in registry.iter() {
            roots.extend(
                [rule.match_.as_ref(), rule.assert_.as_ref()]
                    .into_iter()
                    .flatten()
                    .map(|p| NodeId(p.expr.0)),
            );
        }
    }
    if let Some(q) = program.query.as_ref() {
        roots.extend(q.kw_pairs.iter().map(|e| NodeId(e.0)));
    }
    for root in roots {
        walk(ast, terms, root);
    }
}

// ── Utility extractors ─────────────────────────────────────────────

/// `:key value` pairs, in order. A repeated key resolves to the **last**, as
/// a `dict` comprehension does.
fn kw_pairs(ast: &Ast, args: &[NodeId]) -> Vec<(String, NodeId)> {
    let mut out = Vec::new();
    for &a in args {
        if let Node::KwPair { key, value } = ast.node(a)
            && let Node::Keyword(name) = ast.node(key)
        {
            out.push((ast.sym(name).to_string(), value));
        }
    }
    out
}

fn kw_get(pairs: &[(String, NodeId)], name: &str) -> Option<NodeId> {
    pairs.iter().rev().find(|(k, _)| k == name).map(|(_, v)| *v)
}

/// The `String` body of a node, or `None` when it is not a string — the
/// `isinstance(x, String)` guard the loader applies to `:why` and `:source`.
fn string_value(ast: &Ast, node: Option<NodeId>) -> Option<&str> {
    match node.map(|n| ast.node(n)) {
        Some(Node::Str(s)) => Some(ast.sym(s)),
        _ => None,
    }
}

/// Stringify an `Atom` / `String` / `Int` / `Var` / `Range` for use as a fact
/// argument — `_atomic_value`. Anything else (a `Wildcard`, a keyword, a
/// nested form) answers `None`.
fn atomic_value(ast: &Ast, terms: &mut Terms, node: NodeId) -> Result<Option<Value>, Overflow> {
    Ok(Some(match ast.node(node) {
        Node::Atom(s) | Node::Str(s) => terms.value_text(ast.sym(s))?,
        Node::Int(s) => terms.value_int(ast.sym(s))?,
        Node::Var(s) => terms.value_var(ast.sym(s))?,
        Node::Range { low, high } => {
            let low = ast.sym(low).to_string();
            match high {
                Some(h) => {
                    let h = ast.sym(h).to_string();
                    terms.value_range(&low, Some(&h))?
                }
                None => terms.value_range(&low, None)?,
            }
        }
        _ => return Ok(None),
    }))
}

/// Fact arguments: kw-pairs dropped, atomics flattened, a nested form interned
/// as a **relational node**.
///
/// A nested form whose head is not a bare atom takes the head `"<nested>"`,
/// which is a lossy collapse ein.py also performs.
fn fact_args(ast: &Ast, terms: &mut Terms, args: &[NodeId]) -> Result<Vec<Value>, Overflow> {
    let mut out = Vec::new();
    for &a in args {
        if matches!(ast.node(a), Node::KwPair { .. }) {
            continue;
        }
        if let Some(v) = atomic_value(ast, terms, a)? {
            out.push(v);
            continue;
        }
        if let Node::SForm { head, args } = ast.node(a) {
            let head_name = ast.atom_name(head).unwrap_or("<nested>").to_string();
            let inner = ast.args(args).to_vec();
            let inner = fact_args(ast, terms, &inner)?;
            let rel = terms.intern_text(&head_name)?;
            out.push(terms.value_fact(rel, &inner)?);
        }
    }
    Ok(out)
}

// ── Pass 0 — macros ────────────────────────────────────────────────

fn ingest_macros(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    forms: &[NodeId],
    errors: &mut Vec<String>,
) -> Result<BTreeMap<String, IrMacro>, Overflow> {
    let mut declared: BTreeMap<String, IrMacro> = BTreeMap::new();
    for &form in forms {
        let loc = loc_repr(ast, ast.loc(form));
        let args = ast.form_args(form).to_vec();
        if args.len() < 3 {
            errors.push(format!("(macro) needs name + params + body at {loc}"));
            continue;
        }
        let name = ast.atom_name(args[0]).map(str::to_string);
        let params_form = args[1];
        let body = args[2];
        let Some(name) = name.filter(|_| matches!(ast.node(params_form), Node::SForm { .. }))
        else {
            errors.push(format!("malformed (macro …) at {loc}"));
            continue;
        };
        if is_reserved(&name) {
            errors.push(format!(
                "macro '{name}' shadows a reserved kernel name at {loc}"
            ));
            continue;
        }
        if declared.contains_key(&name) {
            errors.push(format!("duplicate macro '{name}' at {loc}"));
            continue;
        }
        let params: Vec<String> = ast
            .form_args(params_form)
            .iter()
            .filter_map(|&a| match ast.node(a) {
                Node::Var(s) => Some(ast.sym(s).to_string()),
                _ => None,
            })
            .collect();
        declared.insert(
            name.clone(),
            IrMacro {
                name: name.clone(),
                params: params.clone(),
                body,
            },
        );
        let name = terms.intern_text(&name)?;
        let params: Vec<Symbol> = params
            .iter()
            .map(|p| terms.intern_text(p))
            .collect::<Result<_, _>>()?;
        kb.program_mut().macros.insert_new(
            name,
            Macro {
                name,
                params: params.into_boxed_slice(),
                body: ExprRef(body.0),
                loc: core_loc(ast, form),
            },
        );
    }
    Ok(declared)
}

fn core_loc(ast: &Ast, node: NodeId) -> Option<Loc> {
    ast.loc(node).map(|l| Loc {
        file: l.file.0,
        line: l.line,
        col: l.col,
    })
}

// ── Pass 1 — relations ─────────────────────────────────────────────

fn ingest_relation(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    form: NodeId,
    errors: &mut Vec<String>,
) -> Result<(), Overflow> {
    let loc = loc_repr(ast, ast.loc(form));
    let args = ast.form_args(form).to_vec();
    // Flat args post-R10. An EMPTY signature is legal since S1.22.4 —
    // `(relation R)` declares a relation node with no declared arg types, and
    // is deliberately not a hypothesis target, since the "declared domain
    // relation" signal is signature *presence*.
    if args.is_empty() {
        errors.push(format!("(relation) needs a name at {loc}"));
        return Ok(());
    }
    let name = ast.atom_name(args[0]).map(str::to_string);
    // Every arg after the name must be a type atom or a kw-pair. The check is
    // explicit *because* an empty signature is legal: without it the wrapped
    // form `(relation R (T1 T2))` — which the grammar routes to a generic
    // fact — would silently load as a bare declaration instead of being
    // rejected.
    let ill_formed = args[1..]
        .iter()
        .any(|&a| !matches!(ast.node(a), Node::Atom(_) | Node::KwPair { .. }));
    let Some(name) = name.filter(|_| !ill_formed) else {
        errors.push(format!("malformed (relation) at {loc}"));
        return Ok(());
    };
    let signature: Vec<String> = args[1..]
        .iter()
        .filter_map(|&a| ast.atom_name(a).map(str::to_string))
        .collect();
    if is_reserved(&name) {
        errors.push(format!(
            "relation '{name}' shadows a reserved kernel name at {loc}"
        ));
        return Ok(());
    }
    let name_sym = terms.intern_text(&name)?;
    if kb.program().relations.contains(name_sym) {
        errors.push(format!("duplicate relation '{name}' at {loc}"));
        return Ok(());
    }
    // A `:why "<template>"` render template. The signature scan takes atoms
    // only, so the kw-pair never leaks into it.
    let pairs = kw_pairs(ast, &args);
    let why = match string_value(ast, kw_get(&pairs, "why")) {
        Some(w) => Some(terms.intern_text(w)?),
        None => None,
    };
    let sig: Vec<Symbol> = signature
        .iter()
        .map(|s| terms.intern_text(s))
        .collect::<Result<_, _>>()?;
    kb.program_mut().add_relation(Relation {
        name: name_sym,
        signature: sig.clone().into_boxed_slice(),
        declared: true,
        why,
        loc: core_loc(ast, form),
    });
    // The declaration is also stored as an ordinary fact, so rules can
    // introspect signatures with a `(relation ?R ?A ?B)` pattern.
    let relation = terms.kernel.relation;
    let mut decl_args = vec![Value::sym(name_sym)];
    decl_args.extend(sig.iter().map(|&s| Value::sym(s)));
    kb.add_fact(terms, relation, &decl_args, None)?;
    // S1.22.4 — the companion arity-1 *membership* fact. Matching is
    // arity-coupled, so `(relation ?R ?A ?B)` sees only binary declarations
    // and `(relation ?R)` is the arity-independent question. For a bare
    // `(relation R)` the mirror above already is that fact.
    if !sig.is_empty() {
        kb.add_fact(terms, relation, &[Value::sym(name_sym)], None)?;
    }
    Ok(())
}

// ── Pass 2 — rules ─────────────────────────────────────────────────

fn ingest_rules(
    ast: &mut Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    forms: &[NodeId],
    declared_macros: &BTreeMap<String, IrMacro>,
    errors: &mut Vec<String>,
) -> Result<(), Overflow> {
    for &form in forms {
        let head = ast.head_name(form).expect("bucketed by head").to_string();
        let loc = loc_repr(ast, ast.loc(form));
        let args = ast.form_args(form).to_vec();
        if args.len() < 2 {
            errors.push(format!("({head}) needs name + params at {loc}"));
            continue;
        }
        let name = ast.atom_name(args[0]).map(str::to_string);
        let params_form = args[1];
        let Some(name) = name.filter(|_| matches!(ast.node(params_form), Node::SForm { .. }))
        else {
            errors.push(format!("malformed ({head} …) at {loc}"));
            continue;
        };
        // A rule named `absent` / `false` / `eq` / `relation` would never
        // fire — the compiler reads those as primitives — so it is rejected
        // rather than registered dead.
        if is_reserved(&name) {
            errors.push(format!(
                "{head} '{name}' shadows a reserved kernel name at {loc}"
            ));
            continue;
        }
        let name_sym = terms.intern_text(&name)?;
        // `rule` and `hrule` share one name-space.
        if kb.program().rules.contains(name_sym) || kb.program().hrules.contains(name_sym) {
            errors.push(format!("duplicate rule/hrule name '{name}' at {loc}"));
            continue;
        }
        let pairs = kw_pairs(ast, &args);
        let (Some(match_node), Some(assert_node)) =
            (kw_get(&pairs, "match"), kw_get(&pairs, "assert"))
        else {
            errors.push(format!(
                "({head} {name}) missing :match or :assert at {loc}"
            ));
            continue;
        };
        // Macro invocations are rewritten before the clauses are compiled,
        // and before disjunct lowering, so a macro that expands to a
        // top-level `(or …)` still lowers to one plan per disjunct.
        let (match_node, assert_node) = if declared_macros.is_empty() {
            (match_node, assert_node)
        } else {
            match expand_pair(ast, match_node, assert_node, declared_macros) {
                Ok(pair) => pair,
                Err(e) => {
                    errors.push(format!("({head} {name}): {e}"));
                    continue;
                }
            }
        };
        let why = match string_value(ast, kw_get(&pairs, "why")) {
            Some(w) => Some(terms.intern_text(w)?),
            None => None,
        };
        let priority = match kw_get(&pairs, "priority").map(|n| ast.node(n)) {
            Some(Node::Int(s)) => {
                let text = ast.sym(s).to_string();
                Some(terms.ints.intern(&text)?)
            }
            _ => None,
        };
        let params: Vec<Symbol> = ast
            .form_args(params_form)
            .iter()
            .filter_map(|&a| match ast.node(a) {
                Node::Var(s) => Some(ast.sym(s).to_string()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .iter()
            .map(|p| terms.intern_text(p))
            .collect::<Result<_, _>>()?;
        // S1.8.A13 — a top-level `(or …)` `:match` and a top-level `(and …)`
        // `:assert` stay as ordinary AST on ONE rule, which keeps its source
        // name; the compiler lowers them to several plans and templates.
        let rule = Rule {
            name: name_sym,
            params: params.into_boxed_slice(),
            match_: Some(pattern_from_ir(ast, terms, match_node)?),
            assert_: Some(pattern_from_ir(ast, terms, assert_node)?),
            why,
            priority,
            loc: core_loc(ast, form),
        };
        if head == "hrule" {
            kb.program_mut().add_hrule(rule);
        } else {
            kb.program_mut().add_rule(rule);
        }
    }
    Ok(())
}

fn expand_pair(
    ast: &mut Ast,
    match_node: NodeId,
    assert_node: NodeId,
    declared: &BTreeMap<String, IrMacro>,
) -> Result<(NodeId, NodeId), MacroError> {
    let match_node = macros::expand_macros(ast, match_node, declared)?;
    let assert_node = macros::expand_macros(ast, assert_node, declared)?;
    Ok((match_node, assert_node))
}

/// The structural view of a `:match` / `:assert` clause — `Pattern.from_ir`.
///
/// Variables in first-bound order and relation names in first-seen order; the
/// structural primitives and the computed predicates contribute their
/// *arguments* but not their own head, because they are not fact relations.
fn pattern_from_ir(ast: &Ast, terms: &mut Terms, expr: NodeId) -> Result<Pattern, Overflow> {
    let mut variables: Vec<String> = Vec::new();
    let mut relations: Vec<String> = Vec::new();
    walk_pattern(ast, expr, &mut variables, &mut relations);
    Ok(Pattern {
        expr: ExprRef(expr.0),
        variables: variables
            .iter()
            .map(|v| terms.intern_text(v))
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice(),
        relation_names: relations
            .iter()
            .map(|r| terms.intern_text(r))
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice(),
    })
}

fn walk_pattern(ast: &Ast, node: NodeId, variables: &mut Vec<String>, relations: &mut Vec<String>) {
    match ast.node(node) {
        Node::Var(s) => {
            let name = ast.sym(s).to_string();
            if !variables.contains(&name) {
                variables.push(name);
            }
        }
        Node::KwPair { value, .. } => walk_pattern(ast, value, variables, relations),
        Node::SForm { head, args } => {
            // The head first — `(?rel ?a ?b)` binds `?rel`.
            if matches!(ast.node(head), Node::Var(_)) {
                walk_pattern(ast, head, variables, relations);
            }
            let head_name = ast.atom_name(head).map(str::to_string);
            let structural = matches!(
                head_name.as_deref(),
                Some("and" | "or" | "not" | "neq" | "eq" | "=")
            );
            if !structural
                && let Some(name) = head_name
                && !matches!(name.as_str(), "@empty" | "@params")
                && !relations.contains(&name)
            {
                relations.push(name);
            }
            for &a in ast.args(args) {
                walk_pattern(ast, a, variables, relations);
            }
        }
        _ => {}
    }
}

// ── Pass 3 — facts ─────────────────────────────────────────────────

fn ingest_fact(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    form: NodeId,
    errors: &mut Vec<String>,
) -> Result<(), Overflow> {
    let loc = loc_repr(ast, ast.loc(form));
    let Some(head_name) = ast.head_name(form).map(str::to_string) else {
        errors.push(format!("fact with non-atom head at {loc}"));
        return Ok(());
    };
    let args = ast.form_args(form).to_vec();
    let pairs = kw_pairs(ast, &args);
    let source = string_value(ast, kw_get(&pairs, "source")).map(str::to_string);
    // Present-but-not-an-atom `:rule` reads as absent, and the record falls
    // back to source-kind — as it does in ein.py, where `_atom_name` answers
    // `None` and the `is not None` test then fails.
    let rule_name = kw_get(&pairs, "rule").and_then(|n| ast.atom_name(n).map(str::to_string));

    // `:using` carries `(rel args)` compact forms. They are interned, not
    // asserted: a premise id may name a proposition this KB never held, and
    // the walks resolve it against belief.
    let mut premises: Vec<FactId> = Vec::new();
    if let Some(using) = kw_get(&pairs, "using")
        && let Node::SForm { args, .. } = ast.node(using)
    {
        for inner in ast.args(args).to_vec() {
            let Node::SForm { head, args } = ast.node(inner) else {
                continue;
            };
            let Some(rel) = ast.atom_name(head).map(str::to_string) else {
                continue;
            };
            let inner_args = ast.args(args).to_vec();
            let inner_args = fact_args(ast, terms, &inner_args)?;
            let rel = terms.intern_text(&rel)?;
            premises.push(terms.intern_fact(rel, &inner_args)?);
        }
    }

    let prov = match rule_name {
        Some(rule) => {
            let rule = terms.intern_text(&rule)?;
            Prov::from_rule(rule, premises.into_boxed_slice(), core_loc(ast, form))
        }
        None => {
            let source = match source {
                Some(s) => Some(terms.intern_text(&s)?),
                None => None,
            };
            Prov::from_source(source, core_loc(ast, form))
        }
    };
    let prov: ProvId = terms.provs.push(prov);

    // Undeclared relations are auto-created open-world, unless the head is a
    // built-in predicate: predicates dispatch at the matcher level and are not
    // relations, so vivifying one would put a phantom entry in the registry.
    let head = terms.intern_text(&head_name)?;
    if !kb.program().relations.contains(head) && !is_predicate(&head_name) {
        kb.program_mut().add_relation(Relation {
            name: head,
            signature: Box::new([]),
            declared: false,
            why: None,
            loc: core_loc(ast, form),
        });
    }
    let args = fact_args(ast, terms, &args)?;
    kb.add_fact(terms, head, &args, Some(prov))?;
    Ok(())
}

// ── Config ─────────────────────────────────────────────────────────

/// `SolverConfig.from_kw_pairs` — kebab flags to fields, with the coercions
/// and the messages a puzzle author reads.
pub fn config_from_kw_pairs(ast: &Ast, args: &[NodeId]) -> Result<SolverConfig, String> {
    let mut config = SolverConfig::default();
    for &arg in args {
        let Node::KwPair { key, value } = ast.node(arg) else {
            return Err(format!(
                "(config …) body expects kw_pairs, got {}",
                node_repr(ast, arg)
            ));
        };
        let key_name = match ast.node(key) {
            Node::Keyword(s) => ast.sym(s).to_string(),
            _ => node_repr(ast, key),
        };
        let Some(&(flag, kind)) = FIELDS.iter().find(|(f, _)| *f == key_name) else {
            let mut valid: Vec<&str> = FIELDS.iter().map(|(f, _)| *f).collect();
            valid.sort_unstable();
            return Err(format!(
                "unknown config flag :{key_name} (expected one of: {})",
                valid.join(", ")
            ));
        };
        set_flag(ast, &mut config, flag, kind, value)?;
    }
    Ok(config)
}

/// What `_unwrap` leaves behind, *typed* — because the coercers dispatch on
/// the Python type it produced, not on the text.
///
/// `_unwrap` returns `.value` when the node has one (`Int`, `String`), then
/// `.name` (`Atom`, `Var`, `Keyword`), then the node itself. So a `String` and
/// an `Atom` both arrive as a Python `str` and satisfy `_coerce_str`, while an
/// `Int` arrives as an `int` and does **not** — which is the one place a
/// text-only unwrapping would silently accept what ein.py rejects.
enum Unwrapped<'a> {
    Text(&'a str),
    Int(&'a str),
    Node,
}

fn unwrap(ast: &Ast, node: NodeId) -> Unwrapped<'_> {
    match ast.node(node) {
        Node::Str(s) => Unwrapped::Text(ast.sym(s)),
        // `Var.name` and `Keyword.name` are the bare names, `?` and `:` not
        // included — `_unwrap` reaches for `.name` without looking at what
        // kind of node it found.
        Node::Atom(s) | Node::Var(s) | Node::Keyword(s) => Unwrapped::Text(ast.sym(s)),
        Node::Int(s) => Unwrapped::Int(ast.sym(s)),
        _ => Unwrapped::Node,
    }
}

fn set_flag(
    ast: &Ast,
    config: &mut SolverConfig,
    flag: &str,
    kind: FieldKind,
    value: NodeId,
) -> Result<(), String> {
    let raw = unwrap(ast, value);
    let numeric = match raw {
        Unwrapped::Text(t) | Unwrapped::Int(t) => Some(t),
        Unwrapped::Node => None,
    };
    let got = || node_repr(ast, value);
    match kind {
        FieldKind::Bool => {
            let text = match raw {
                Unwrapped::Text(t) => Some(t.to_ascii_lowercase()),
                _ => None,
            };
            let b = match text.as_deref() {
                Some("true") => true,
                Some("false") => false,
                _ => {
                    return Err(format!(
                        "config flag :{flag} expects true/false, got {}",
                        got()
                    ));
                }
            };
            match flag {
                "enable-pre-branch-lookahead" => config.enable_pre_branch_lookahead = b,
                "enable-lookahead-kill-cache" => config.enable_lookahead_kill_cache = b,
                "print-alive" => config.print_alive = b,
                "warn-derived-naf" => config.warn_derived_naf = b,
                "lattice-sanity-check" => config.lattice_sanity_check = b,
                "enable-path-nogoods" => config.enable_path_nogoods = b,
                "enable-symmetric-mirror" => config.enable_symmetric_mirror = b,
                "enable-singleton-writeback" => config.enable_singleton_writeback = b,
                "enable-forced-positive" => config.enable_forced_positive = b,
                "record-alternative-justifications" => {
                    config.record_alternative_justifications = b;
                }
                "enable-fail-fast-fork" => config.enable_fail_fast_fork = b,
                other => unreachable!("{other} is not a bool flag"),
            }
        }
        FieldKind::Int => {
            // A `true` / `false` atom would coerce through `int()` in neither
            // implementation, but CPython checks `isinstance(value, bool)`
            // first and says so; the atoms reach here as text, and `int()`
            // rejects them anyway with the same message.
            let n = numeric
                .and_then(python_int)
                .ok_or_else(|| format!("config flag :{flag} expects an integer, got {}", got()))?;
            match flag {
                "candidate-order-seed" => config.candidate_order_seed = n,
                "lattice-order-seed" => config.lattice_order_seed = Some(n),
                other => unreachable!("{other} is not an int flag"),
            }
        }
        FieldKind::Float => {
            let n = numeric
                .and_then(python_float)
                .ok_or_else(|| format!("config flag :{flag} expects a number, got {}", got()))?;
            match flag {
                "hypgen-rel-weight" => config.hypgen_rel_weight = n,
                "hypgen-obj-weight" => config.hypgen_obj_weight = n,
                other => unreachable!("{other} is not a float flag"),
            }
        }
        FieldKind::Str => {
            let s = match raw {
                Unwrapped::Text(t) => t.to_string(),
                _ => {
                    return Err(format!(
                        "config flag :{flag} expects a string, got {}",
                        got()
                    ));
                }
            };
            match flag {
                "hypgen-scoring" => config.hypgen_scoring = s,
                "lattice-order" => config.lattice_order = s,
                other => unreachable!("{other} is not a string flag"),
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_core::{ProvKind, Tag};

    fn load_text(text: &str) -> (Ast, Terms, Result<Kb, KbLoadError>) {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, text, None).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, None);
        (ast, terms, kb)
    }

    fn ok(text: &str) -> (Terms, Kb) {
        let (_ast, terms, kb) = load_text(text);
        (terms, kb.expect("loads"))
    }

    fn err(text: &str) -> String {
        let (_ast, _terms, kb) = load_text(text);
        kb.expect_err("must fail").0
    }

    fn facts(terms: &Terms, kb: &Kb) -> Vec<String> {
        kb.facts().map(|f| terms.compact(f)).collect()
    }

    #[test]
    fn a_declaration_stores_itself_as_facts_so_rules_can_read_signatures() {
        let (terms, kb) = ok("(relation r A B)\n(relation bare)");
        assert_eq!(
            facts(&terms, &kb),
            vec!["(relation r A B)", "(relation r)", "(relation bare)"],
            "a signature gets the arity-1 companion; a bare declaration is \
             already it"
        );
    }

    #[test]
    fn an_undeclared_head_vivifies_a_relation_but_a_predicate_does_not() {
        let (terms, kb) = ok("(mystery a b)\n(eq a a)");
        let mystery = terms.syms.get("mystery").expect("interned");
        let eq = terms.syms.get("eq").expect("interned");
        assert!(kb.program().relations.contains(mystery));
        assert!(
            !kb.program()
                .relations
                .get(mystery)
                .expect("present")
                .declared,
            "open-world"
        );
        assert!(
            !kb.program().relations.contains(eq),
            "predicates dispatch at the matcher, and a phantom registry entry \
             would make one look like a relation"
        );
    }

    #[test]
    fn a_nested_form_becomes_a_relational_node() {
        let (terms, kb) = ok("(not (co-located a b))\n(p (?q a))");
        let nested: Vec<Vec<Tag>> = kb
            .facts()
            .map(|f| terms.facts.args(f).iter().map(|a| a.tag()).collect())
            .collect();
        assert_eq!(nested, vec![vec![Tag::Fact], vec![Tag::Fact]]);
        // A nested head that is not a bare atom collapses to `<nested>` — a
        // lossy rename ein.py also performs.
        let compacts = facts(&terms, &kb);
        assert!(compacts[1].contains("<nested>"), "{compacts:?}");
        // The inner proposition is interned but not believed: it is not in
        // the fact list, and that is what makes the negated index meaningful.
        assert_eq!(kb.n_facts(), 2);
        let inner = terms.facts.args(kb.facts().next().expect("a fact"))[0]
            .as_fact()
            .expect("nested");
        assert!(!kb.contains(inner));
        assert!(kb.is_negated(inner));
    }

    #[test]
    fn provenance_comes_from_source_or_from_rule_and_using() {
        let (terms, kb) = ok("(relation r T T)\n(r a b :source \"(1)\")\n(r c d)\n\
             (r e f :rule x :using (p (r a b)))");
        let kinds: Vec<(&str, usize)> = kb
            .facts()
            .filter_map(|f| kb.primary(f))
            .map(|p| {
                let prov = terms.provs.get(p);
                (prov.kind.as_str(), prov.premises.len())
            })
            .collect();
        assert_eq!(
            kinds,
            vec![
                // The two auto-stored declaration facts carry none at all.
                ("source", 0),
                ("source", 0),
                ("rule", 1),
            ]
        );
        // …which is to say the declaration facts have no provenance, so they
        // do not appear above.
        assert_eq!(kb.n_facts(), 5);
        let derived = kb.facts().last().expect("a fact");
        let prov = terms.provs.get(kb.primary(derived).expect("recorded"));
        assert_eq!(prov.kind, ProvKind::Rule);
        assert_eq!(terms.compact(prov.premises[0]), "(r a b)");
    }

    #[test]
    fn a_present_but_unusable_annotation_reads_as_absent() {
        // `:source` must be a String and `:rule` an Atom; anything else is
        // not an error, it simply does not register — `_atom_name` answers
        // `None` and the `is not None` test then fails.
        let (terms, kb) = ok("(relation r T T)\n(r a b :source sentence :rule \"x\")");
        let fact = kb.facts().last().expect("a fact");
        let prov = terms.provs.get(kb.primary(fact).expect("recorded"));
        assert_eq!(prov.kind, ProvKind::Source);
        assert_eq!(prov.source, None);
    }

    #[test]
    fn the_last_query_and_the_last_config_win() {
        let (_terms, kb) = ok("(query :goal (a ?x))\n(query :goal (b ?x) :mode all)\n\
             (config :print-alive true)\n(config :warn-derived-naf true)");
        assert_eq!(
            kb.program().query.as_ref().expect("a query").kw_pairs.len(),
            2
        );
        let config = kb.program().config.as_ref().expect("a config");
        assert!(config.warn_derived_naf);
        assert!(!config.print_alive, "the earlier block is discarded whole");
    }

    #[test]
    fn a_rules_pattern_view_records_variables_and_relation_heads() {
        let (terms, kb) = ok("(relation r T T)\n(relation s T T)\n\
             (rule j (?p) :match (and (r ?a ?b) (not (s ?b ?c))) \
              :assert (r ?a ?c) :why \"j\")");
        let j = terms.syms.get("j").expect("interned");
        let rule = kb.program().rules.get(j).expect("registered");
        let names = |syms: &[Symbol]| -> Vec<&str> { syms.iter().map(|&s| terms.sym(s)).collect() };
        let m = rule.match_.as_ref().expect("a match");
        assert_eq!(names(&m.variables), vec!["a", "b", "c"]);
        // `and` and `not` are structural: they contribute their arguments but
        // not their own heads.
        assert_eq!(names(&m.relation_names), vec!["r", "s"]);
        assert_eq!(names(&rule.params), vec!["p"]);
    }

    #[test]
    fn every_pass_accumulates_rather_than_stopping_at_the_first_problem() {
        let message = err(
            "(relation)\n(relation dup)\n(relation dup)\n(rule absent () :match (x ?a) :assert (y ?a))",
        );
        assert_eq!(
            message,
            "(relation) needs a name at None; duplicate relation 'dup' at None; \
             rule 'absent' shadows a reserved kernel name at None"
        );
    }
}
