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
use std::collections::{BTreeMap, BTreeSet};
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

/// Build a populated `Kb` from parsed IR forms, about the file's **first**
/// `(query …)` block.
///
/// A file may carry several; [`load_query`] is how the others are reached, and
/// `Program::queries` is how a caller finds out there are any. See
/// [`Program::active_query`](ein_core::Program::active_query).
pub fn load(
    ast: &mut Ast,
    terms: &mut Terms,
    forms: &[NodeId],
    base_dir: Option<&Path>,
) -> Result<Kb, KbLoadError> {
    load_query(ast, terms, forms, base_dir, 0)
}

/// [`load`], about query number `active_query`.
///
/// Out-of-range is not an error and not a panic: `Program::query()` returns
/// `None`, which is the same state a file with no query at all is in.
pub fn load_query(
    ast: &mut Ast,
    terms: &mut Terms,
    forms: &[NodeId],
    base_dir: Option<&Path>,
    active_query: usize,
) -> Result<Kb, KbLoadError> {
    let mut kb = Kb::new(Program::new());
    kb.program_mut().active_query = active_query;
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

    // Every query block, in source order — plural since M1c S1c.1.2, because a
    // second one used to load and be discarded in silence. `config` keeps
    // last-wins: a config is a setting, a query is content.
    let queries: Vec<Query> = query_blocks
        .iter()
        .map(|&form| Query {
            kw_pairs: ast.form_args(form).iter().map(|n| ExprRef(n.0)).collect(),
        })
        .collect();
    kb.program_mut().queries = queries;
    validate_queries(ast, terms, &kb, &mut errors);
    if let Some(&last) = config_blocks.last() {
        let args: Vec<NodeId> = ast.form_args(last).to_vec();
        match config_from_kw_pairs(ast, &args) {
            Ok(config) => kb.program_mut().config = Some(config),
            Err(e) => errors.push(format!("(config …): {e}")),
        }
    }

    // The S1.8a.f20 guard: a `(forall …)` / `(unknown …)` used without importing
    // `std.macro` would leave the invocation in place and the rule would
    // silently never fire.
    let mut rule_matches: Vec<(String, NodeId)> = Vec::new();
    for registry in [
        &kb.program().rules,
        &kb.program().hrules,
        &kb.program().obligations,
    ] {
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
/// [P1a.7](../../../../docs/history/m1a_rust/README.md#p1a7--parallelism) needs:
/// [`Interner::text`](ein_core::Interner::text) hands out a `&str` borrowed
/// from the arena, so an interner that is *shared* must be one that does not
/// **grow**, and the search is exactly where it would be shared
/// ([S1a.7.1](../../../../docs/history/m1a_rust/README.md#s1a71--making-the-shared-state-sync)).
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
    for registry in [&program.rules, &program.hrules, &program.obligations] {
        for (_, rule) in registry.iter() {
            roots.extend(
                [rule.match_.as_ref(), rule.assert_.as_ref()]
                    .into_iter()
                    .flatten()
                    .map(|p| NodeId(p.expr.0)),
            );
        }
    }
    // Every query, not just the active one: a name that only query 2 mentions
    // is still a name the compiler must not meet for the first time mid-run.
    for q in &program.queries {
        roots.extend(q.kw_pairs.iter().map(|e| NodeId(e.0)));
    }
    for root in roots {
        walk(ast, terms, root);
    }
}

/// Every `(query …)` keyword the engine reads.
///
/// The list is an **allow-list**, and an unrecognised keyword is a load error
/// rather than a silent no-op — M1c
/// [S1c.1.2](../../../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects).
/// Before it, `(query :expct (model …))` parsed, loaded, checked nothing and
/// said nothing, which is the failure mode `:expect` exists to remove; a form
/// that carries a *test* cannot also be the place a typo goes to die.
///
/// `mode` is here and is read by nothing. It was a real keyword, three corpus
/// files carry a comment saying it is obsolete, and rejecting it would make a
/// stale file fail to load rather than fail to matter. Accepted-and-ignored is
/// documented; silently-unknown is not.
const QUERY_KEYWORDS: [&str; 7] = [
    "goal",
    "goal-text",
    "hrules",
    "hypothesis-relations",
    "no-hypothesis",
    "expect",
    "mode",
];

/// The `(query …)` blocks' own validation — keywords, and `:expect`.
///
/// Runs after the fact pass, because two of the three `:expect` rules are
/// about the *program*: a relation it names must be one the program knows, and
/// the goal's relations must be among the ones it closes.
fn validate_queries(ast: &Ast, terms: &Terms, kb: &Kb, errors: &mut Vec<String>) {
    for query in &kb.program().queries {
        for &pair in query.kw_pairs.iter() {
            let Node::KwPair { key, .. } = ast.node(NodeId(pair.0)) else {
                continue;
            };
            let Node::Keyword(name) = ast.node(key) else {
                continue;
            };
            let name = ast.sym(name);
            if !QUERY_KEYWORDS.contains(&name) {
                errors.push(format!(
                    "(query …): unknown keyword :{name} — one of {}",
                    QUERY_KEYWORDS
                        .iter()
                        .map(|k| format!(":{k}"))
                        .collect::<Vec<_>>()
                        .join(" ")
                ));
            }
        }
        let Some(node) = query_keyword(ast, query, "expect") else {
            continue;
        };
        let expectation = match crate::expect::parse(ast, node) {
            Ok(e) => e,
            Err(message) => {
                errors.push(message);
                continue;
            }
        };
        // Rule 3's precondition: a relation an expectation *closes* has to be
        // one the program has. A name nothing declares and no fact uses would
        // close a relation that does not exist, and pass — vacuously, for
        // ever.
        for model in expectation.models() {
            for &f in &model.facts {
                let Ok(fact) = crate::expect::fact(ast, f) else {
                    continue; // already reported by `expect::parse`
                };
                let known = terms
                    .syms
                    .get(fact.relation)
                    .is_some_and(|sym| kb.program().relations.get(sym).is_some());
                if !known {
                    errors.push(format!(
                        ":expect names {}, which no declaration or fact makes a relation",
                        fact.relation
                    ));
                }
            }
        }
        // Rule 1: the goal's relations are mandatory. An expectation that does
        // not pin what the query asked is not an expectation — and since
        // naming a relation closes it, "pins" and "names" are the same word.
        let Some(goal) = query_keyword(ast, query, "goal") else {
            continue;
        };
        let goal_relations = pattern_relations(ast, goal);
        for model in expectation.models() {
            let closed: Vec<&str> = model
                .facts
                .iter()
                .filter_map(|&f| crate::expect::fact(ast, f).ok())
                .filter(|f| !f.negated)
                .map(|f| f.relation)
                .collect();
            for want in &goal_relations {
                if !closed.iter().any(|r| r == want) {
                    errors.push(format!(
                        ":expect does not name {want}, which the query's :goal asks about"
                    ));
                }
            }
        }
    }
}

/// A query keyword's value node — `ein_infer::query_value`, which this crate
/// is below and cannot call. The **first** match wins, as it does there.
fn query_keyword(ast: &Ast, query: &Query, want: &str) -> Option<NodeId> {
    for &pair in query.kw_pairs.iter() {
        let Node::KwPair { key, value } = ast.node(NodeId(pair.0)) else {
            continue;
        };
        if let Node::Keyword(name) = ast.node(key)
            && ast.sym(name) == want
        {
            return Some(value);
        }
    }
    None
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
        if kb.program().rules.contains(name_sym)
            || kb.program().hrules.contains(name_sym)
            || kb.program().obligations.contains(name_sym)
        {
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
                Some(terms.intern_int(&text)?)
            }
            _ => None,
        };
        let param_names: Vec<String> = ast
            .form_args(params_form)
            .iter()
            .filter_map(|&a| match ast.node(a) {
                Node::Var(s) => Some(ast.sym(s).to_string()),
                _ => None,
            })
            .collect();
        // The reserved verdict atom: legal only in `:assert`, only at arity 0
        // or 1, only as a whole conclusion, and only where its projection
        // resolves. A rule that asserts it is an **obligation** and is routed
        // out of the saturation agenda below.
        let n_errors = errors.len();
        let is_obligation = validate_open(
            ast,
            &head,
            &name,
            &param_names,
            match_node,
            assert_node,
            &loc,
            errors,
        );
        if errors.len() != n_errors {
            continue;
        }
        let params: Vec<Symbol> = param_names
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
        if is_obligation {
            // Never the saturator's: it derives nothing, so it has no place in
            // a queue that orders derivation, and it is read once per quiescent
            // KB after the fixpoint instead. An `hrule` asserting `open` is the
            // same object and goes the same way — `:choose` was never the home
            // for obligations (`obligation_forms.md` § F).
            kb.program_mut().add_obligation(rule);
        } else if head == "hrule" {
            kb.program_mut().add_hrule(rule);
        } else {
            kb.program_mut().add_rule(rule);
        }
    }
    Ok(())
}

/// Is `node` the reserved verdict atom — `(open)` or `(open ?R)`?
fn is_open_form(ast: &Ast, node: NodeId) -> bool {
    matches!(ast.node(node), Node::SForm { .. }) && ast.head_name(node) == Some("open")
}

/// Every `(open …)` form anywhere under `node`, in source order.
fn open_forms(ast: &Ast, node: NodeId, out: &mut Vec<NodeId>) {
    if is_open_form(ast, node) {
        out.push(node);
    }
    if matches!(ast.node(node), Node::SForm { .. }) {
        for &a in ast.form_args(node) {
            open_forms(ast, a, out);
        }
    }
    if let Node::KwPair { value, .. } = ast.node(node) {
        open_forms(ast, value, out);
    }
}

/// Positional (non-`KwPair`) arguments of a form.
fn pos_args(ast: &Ast, node: NodeId) -> Vec<NodeId> {
    ast.form_args(node)
        .iter()
        .copied()
        .filter(|&a| !matches!(ast.node(a), Node::KwPair { .. }))
        .collect()
}

/// The top-level `:assert` conjuncts — `(and c1 … ck)` → the `ci`, else `[expr]`.
fn top_conjuncts(ast: &Ast, expr: NodeId) -> Vec<NodeId> {
    if matches!(ast.node(expr), Node::SForm { .. }) && ast.head_name(expr) == Some("and") {
        return pos_args(ast, expr);
    }
    vec![expr]
}

/// Every variable name occurring under `node`.
fn vars_under(ast: &Ast, node: NodeId, out: &mut BTreeSet<String>) {
    match ast.node(node) {
        Node::Var(s) => {
            out.insert(ast.sym(s).to_string());
        }
        Node::SForm { .. } => {
            for &a in ast.form_args(node) {
                vars_under(ast, a, out);
            }
        }
        Node::KwPair { value, .. } => vars_under(ast, value, out),
        _ => {}
    }
}

/// Every `(absent …)` form under `node`, outermost first.
fn absent_forms(ast: &Ast, node: NodeId, out: &mut Vec<NodeId>) {
    if matches!(ast.node(node), Node::SForm { .. }) {
        if ast.head_name(node) == Some("absent") {
            out.push(node);
            return;
        }
        for &a in ast.form_args(node) {
            absent_forms(ast, a, out);
        }
    }
}

/// Premises under `node` whose *head* is the obligation's relation — the
/// variable `?R` or the literal name `R` that `(open …)` names.
///
/// `neg` reports whether the premise sits under a `(not …)`, which is never a
/// commit target.
fn head_matches(ast: &Ast, node: NodeId, rel: &str, is_var: bool) -> bool {
    if !matches!(ast.node(node), Node::SForm { .. }) {
        return false;
    }
    let Node::SForm { head, .. } = ast.node(node) else {
        return false;
    };
    match ast.node(head) {
        Node::Var(s) if is_var => ast.sym(s) == rel,
        Node::Atom(s) if !is_var => ast.sym(s) == rel,
        _ => false,
    }
}

fn rel_premises(
    ast: &Ast,
    node: NodeId,
    rel: &str,
    is_var: bool,
    neg: bool,
    out: &mut Vec<(NodeId, bool)>,
) {
    if !matches!(ast.node(node), Node::SForm { .. }) {
        return;
    }
    if head_matches(ast, node, rel, is_var) {
        out.push((node, neg));
        return;
    }
    let under_not = neg || ast.head_name(node) == Some("not");
    for &a in ast.form_args(node) {
        rel_premises(ast, a, rel, is_var, under_not, out);
    }
}

/// Load-time validation of the reserved verdict atom, and the classification
/// that follows from it.
///
/// Returns `true` when the rule is an **obligation** — its `:assert` is
/// `(open …)` and nothing else — which routes it out of the saturation agenda
/// (M1d P1d.2 [S1d.2.3]). Errors are pushed rather than returned so one
/// malformed rule does not hide the next.
///
/// The four refusals, and the reason each is a refusal rather than a guess:
///
/// 1. **`(open …)` in `:match`** — the atom is a conclusion about the KB, not
///    a premise. The third-state *fact* probe is `(unknown P)`, and the
///    message says so because the two were one word until 2026-08-24.
/// 2. **arity ≥ 2** — the form is `(open)` or `(open ?R)`. Anything else is
///    the superseded triple `(open ?b G B)` or the positional
///    `(open ?R 0 ?a)`, both of which restated what the guard already says.
/// 3. **a mixed `:assert`** — a rule concluding `open` *and* a fact would
///    belong to both the saturation agenda and the obligation pass, and
///    refusing it is cheaper than deciding which owns it.
/// 4. **a projection that does not resolve** — `(open ?R)` names the relation
///    whose extent is incomplete and the engine reads the rest out of the
///    rule's own `absent`: exactly one `absent` holding a positive `?R`
///    premise, and exactly one such premise bearing a variable the guard does
///    not already bind. None, two, or a ground body is refused rather than
///    guessed.
///
/// [S1d.2.3]: `docs/history/m1d_satisfiability/README.md#s1d23--the-form`
#[allow(clippy::too_many_arguments)]
fn validate_open(
    ast: &Ast,
    head: &str,
    name: &str,
    params: &[String],
    match_node: NodeId,
    assert_node: NodeId,
    loc: &str,
    errors: &mut Vec<String>,
) -> bool {
    // (1) the atom is assert-side only.
    let mut in_match = Vec::new();
    open_forms(ast, match_node, &mut in_match);
    if !in_match.is_empty() {
        errors.push(format!(
            "{head} '{name}': `(open …)` is a verdict about the KB and is legal \
             only in :assert — the third-state probe for a fact is `(unknown …)` \
             at {loc}"
        ));
    }

    let conjuncts = top_conjuncts(ast, assert_node);
    let opens: Vec<NodeId> = conjuncts
        .iter()
        .copied()
        .filter(|&c| is_open_form(ast, c))
        .collect();

    // An `(open …)` buried inside a conclusion — `(not (open))`, `(f (open))` —
    // is neither a conjunct nor a fact, so it is caught here rather than
    // silently stored.
    let mut anywhere = Vec::new();
    open_forms(ast, assert_node, &mut anywhere);
    if anywhere.len() > opens.len() {
        errors.push(format!(
            "{head} '{name}': `(open …)` is a whole conclusion, not a term inside \
             one at {loc}"
        ));
        return false;
    }
    if opens.is_empty() {
        return false;
    }

    // (3) nothing else may be concluded alongside it.
    if conjuncts.len() > opens.len() {
        errors.push(format!(
            "{head} '{name}': a rule asserting `open` may assert nothing else — \
             it is read after the fixpoint, where a derivation would be too late \
             at {loc}"
        ));
    }

    for &o in &opens {
        let args = pos_args(ast, o);
        // (2) arity.
        if args.len() > 1 {
            errors.push(format!(
                "{head} '{name}': `open` takes the incomplete relation and nothing \
                 else — `(open)` or `(open ?R)`, not {} arguments at {loc}",
                args.len()
            ));
            continue;
        }
        let Some(&arg) = args.first() else {
            continue; // bare `(open)` — countable, nothing to project.
        };
        let (rel, is_var) = match ast.node(arg) {
            Node::Var(s) => (ast.sym(s).to_string(), true),
            Node::Atom(s) => (ast.sym(s).to_string(), false),
            _ => {
                errors.push(format!(
                    "{head} '{name}': `open`'s argument names a relation — a rule \
                     parameter or a relation name at {loc}"
                ));
                continue;
            }
        };
        // A variable relation head is bound by the activator, never by a
        // premise (`compile.rs`: "M1 matches relations per activator"), so an
        // `(open ?R)` whose `?R` is not a parameter could never resolve.
        if is_var && !params.iter().any(|p| p == &rel) {
            errors.push(format!(
                "{head} '{name}': `(open ?{rel})` names a relation the activator \
                 does not bind — `?{rel}` is not in the parameter list at {loc}"
            ));
            continue;
        }
        validate_projection(ast, head, name, &rel, is_var, match_node, loc, errors);
    }
    true
}

/// (4) — resolve the witness step out of the rule's own `absent`, or refuse.
#[allow(clippy::too_many_arguments)]
fn validate_projection(
    ast: &Ast,
    head: &str,
    name: &str,
    rel: &str,
    is_var: bool,
    match_node: NodeId,
    loc: &str,
    errors: &mut Vec<String>,
) {
    let shown = if is_var {
        format!("?{rel}")
    } else {
        rel.to_string()
    };
    let mut absents = Vec::new();
    absent_forms(ast, match_node, &mut absents);
    let holders: Vec<NodeId> = absents
        .iter()
        .copied()
        .filter(|&a| {
            let mut prems = Vec::new();
            rel_premises(ast, a, rel, is_var, false, &mut prems);
            prems.iter().any(|&(_, neg)| !neg)
        })
        .collect();
    if holders.is_empty() {
        errors.push(format!(
            "{head} '{name}': `(open {shown})` needs an `(absent …)` in :match \
             holding a positive `{shown}` premise — that premise is the witness \
             the obligation owes at {loc}"
        ));
        return;
    }
    if holders.len() > 1 {
        errors.push(format!(
            "{head} '{name}': {} `(absent …)` guards hold a positive `{shown}` \
             premise — which one states the obligation is not decidable at {loc}",
            holders.len()
        ));
        return;
    }
    let absent = holders[0];

    // Variables the guard binds from outside this `absent` are not the witness;
    // the witness slots are the ones the absent introduces.
    let mut outside = BTreeSet::new();
    collect_vars_outside(ast, match_node, absent, &mut outside);

    let mut prems = Vec::new();
    rel_premises(ast, absent, rel, is_var, false, &mut prems);
    let witnesses: Vec<NodeId> = prems
        .iter()
        .filter(|&&(_, neg)| !neg)
        .map(|&(n, _)| n)
        .filter(|&n| {
            let mut vs = BTreeSet::new();
            vars_under(ast, n, &mut vs);
            vs.iter().any(|v| !outside.contains(v))
        })
        .collect();
    match witnesses.len() {
        1 => {}
        0 => errors.push(format!(
            "{head} '{name}': `(open {shown})`'s `{shown}` premise binds no \
             variable of its own, so the obligation is ground — that is a plain \
             `absent` check and not something a witness could discharge at {loc}"
        )),
        n => errors.push(format!(
            "{head} '{name}': {n} positive `{shown}` premises each bind a witness \
             variable — a compound witness has no single slot to branch on at {loc}"
        )),
    }
}

/// Variables of `root` that occur anywhere except under `skip`.
fn collect_vars_outside(ast: &Ast, root: NodeId, skip: NodeId, out: &mut BTreeSet<String>) {
    if root == skip {
        return;
    }
    match ast.node(root) {
        Node::Var(s) => {
            out.insert(ast.sym(s).to_string());
        }
        Node::SForm { .. } => {
            for &a in ast.form_args(root) {
                collect_vars_outside(ast, a, skip, out);
            }
        }
        Node::KwPair { value, .. } => collect_vars_outside(ast, value, skip, out),
        _ => {}
    }
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

/// The relations a pattern **asks about**, in first-seen order.
///
/// What the `:expect` rule above needs and the only part of `walk_pattern`
/// anything outside this module has a use for: a query's `:goal` names some
/// relations, and *naming a relation closes it*, so those are exactly the
/// relations an expectation has to list the complete extent of. `ein test
/// --json-report` publishes the list per query, because the **write cost** of
/// a closure claim is `relations x models x facts` and the first factor is
/// this one (M1d [S1d.4.1](../../../../docs/history/m1d_satisfiability/README.md#s1d41--what-closure-costs)).
///
/// Connectives (`and`, `or`, `not`, `neq`, `eq`, `=`) and the two macro
/// internals are not relations and do not appear; a `(?rel ?a ?b)` head binds
/// a variable and contributes nothing here, which is the one case where a
/// goal asks about a relation this list cannot name.
pub fn pattern_relations(ast: &Ast, node: NodeId) -> Vec<String> {
    let (mut vars, mut relations) = (Vec::new(), Vec::new());
    walk_pattern(ast, node, &mut vars, &mut relations);
    relations
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

    /// The reserved verdict atom, and the classification it drives — M1d
    /// [S1d.2.3].
    ///
    /// A rule whose `:assert` is `(open …)` and nothing else is an
    /// **obligation**: it derives nothing, so it never enters the saturator's
    /// agenda, and it is read once per quiescent KB after the fixpoint. The
    /// routing is the load-time half of that, and the assertion below is that
    /// `rules` stays empty while `obligations` gets it.
    ///
    /// [S1d.2.3]: `docs/history/m1d_satisfiability/README.md#s1d23--the-form`
    #[test]
    fn an_open_assert_routes_the_rule_out_of_the_saturation_agenda() {
        let src = "(relation is-a T T)\n(relation r A B)\n\
                   (rule total-owed (?R ?isa)\n  \
                     :match (and (relation ?R ?A ?B) (?isa ?a ?A)\n              \
                                 (absent (and (?isa ?b ?B) (?R ?a ?b))))\n  \
                     :assert (open ?R))\n";
        let (terms, kb) = ok(src);
        let name = terms.syms.get("total-owed").expect("interned");
        assert!(
            kb.program().obligations.contains(name),
            "an `open`-asserting rule belongs to the obligation pass"
        );
        assert!(
            kb.program().rules.is_empty() && kb.program().hrules.is_empty(),
            "…and to neither of the two that fire during a search"
        );

        // The bare degenerate is the same object with nothing to project.
        let (terms, kb) =
            ok("(relation r A B)\n(rule owes () :match (absent (r a b)) :assert (open))\n");
        assert!(
            kb.program()
                .obligations
                .contains(terms.syms.get("owes").expect("interned")),
            "`(open)` is countable and routes the same way"
        );

        // An ordinary rule is untouched by any of it.
        let (terms, kb) =
            ok("(relation r A B)\n(rule copy () :match (r ?a ?b) :assert (r ?b ?a))\n");
        assert!(
            kb.program()
                .rules
                .contains(terms.syms.get("copy").expect("interned"))
                && kb.program().obligations.is_empty()
        );
    }

    /// The four refusals, each with the shape that provokes it.
    ///
    /// They are refusals rather than guesses because every one of them is a
    /// place where the engine would otherwise have to pick a reading: which
    /// `absent` states the obligation, which premise is the witness, whether a
    /// mixed conclusion belongs to the agenda or the pass. A wrong pick is
    /// silent, and silence is what this phase exists to remove.
    #[test]
    fn the_verdict_atom_refuses_every_shape_it_cannot_resolve() {
        let pre = "(relation is-a T T)\n(relation r A B)\n";
        let guard = "(and (relation ?R ?A ?B) (?isa ?a ?A) \
                     (absent (and (?isa ?b ?B) (?R ?a ?b))))";
        for (want, src) in [
            // Match-side placement — and the message names the probe, because
            // the two were one word until 2026-08-24.
            (
                "legal only in :assert",
                format!(
                    "{pre}(rule bad (?R) :match (and (relation ?R ?A ?B) (open ?R)) :assert (r a b))"
                ),
            ),
            // Arity: the superseded triple, and the positional sketch before it.
            (
                "not 3 arguments",
                format!(
                    "{pre}(rule bad (?R ?isa) :match {guard} :assert (open ?b (?isa ?b ?B) (?R ?a ?b)))"
                ),
            ),
            // A mixed conclusion would belong to both strata.
            (
                "may assert nothing else",
                format!(
                    "{pre}(rule bad (?R ?isa) :match {guard} :assert (and (open ?R) (r ?a b)))"
                ),
            ),
            // Nested rather than concluded.
            (
                "not a term inside one",
                format!("{pre}(rule bad (?R ?isa) :match {guard} :assert (not (open ?R)))"),
            ),
            // The projection's three ways of not resolving.
            (
                "needs an `(absent …)` in :match",
                format!(
                    "{pre}(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A)) :assert (open ?R))"
                ),
            ),
            (
                "2 `(absent …)` guards",
                format!(
                    "{pre}(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) \
                     (absent (and (?isa ?b ?B) (?R ?a ?b))) \
                     (absent (and (?isa ?c ?B) (?R ?a ?c)))) :assert (open ?R))"
                ),
            ),
            (
                "compound witness",
                format!(
                    "{pre}(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) \
                     (absent (and (?R ?a ?x) (?R ?x ?b)))) :assert (open ?R))"
                ),
            ),
            (
                "the obligation is ground",
                format!(
                    "{pre}(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) \
                     (absent (?R ?a b))) :assert (open ?R))"
                ),
            ),
            // A variable relation head comes from the activator, never a premise.
            (
                "is not in the parameter list",
                format!(
                    "{pre}(rule bad (?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) \
                     (absent (?R ?a ?b))) :assert (open ?R))"
                ),
            ),
            // …and the name itself may not be bound.
            (
                "shadows a reserved kernel name",
                format!("{pre}(relation open A B)"),
            ),
            (
                "shadows a reserved kernel name",
                format!("{pre}(macro open (?P) (absent ?P))"),
            ),
            (
                "shadows a reserved kernel name",
                format!("{pre}(rule open () :match (r ?a ?b) :assert (r ?b ?a))"),
            ),
        ] {
            let got = err(&src);
            assert!(
                got.contains(want),
                "expected an error mentioning {want:?}, got {got:?} for {src}"
            );
        }
    }

    /// The four duals the phase actually ships, resolved here before the stage
    /// that ships them — M1d [S1d.2.3] T1d.2.3.3.
    ///
    /// They are **not** in `stdlib/` yet, and that is deliberate rather than
    /// unfinished. `ein-infer/tests/stdlib_coverage.rs` reads a module's own
    /// `(rule …)` heads out of the raw forms and fails on any that no
    /// `tests/stdlib/` program activates; an obligation rule cannot activate
    /// anything until the post-fixpoint pass exists, so putting them in the
    /// stdlib now would put two permanently-silent rules behind a gate whose
    /// whole job is to forbid exactly that. S1d.2.4 adds the pass and the
    /// rules together, with their conformance programs.
    ///
    /// What this stage owes is that the *projection resolves on the shapes the
    /// phase ships*, and that is checkable without shipping them: the two
    /// `std.algebra` duals mirror `total` / `surjective`, and the two
    /// `std.slots` duals mirror `slot-no-room` / `slot-no-fill`, one modality
    /// down — scanning **absence** where those scan a stored `(not …)`, and
    /// saying *unfinished* where they say *dead*.
    ///
    /// [S1d.2.3]: `docs/history/m1d_satisfiability/README.md#s1d23--the-form`
    #[test]
    fn the_stdlib_duals_resolve_before_the_stage_that_ships_them() {
        let src = "(relation is-a T T)\n\
             (rule total-owed (?R ?isa)\n  \
               :match (and (relation ?R ?A ?B) (?isa ?a ?A)\n              \
                           (absent (and (?isa ?b ?B) (?R ?a ?b))))\n  \
               :assert (open ?R)\n  :why \"{?R} owes {?a} a {?B}\")\n\
             (rule surjective-owed (?R ?isa)\n  \
               :match (and (relation ?R ?A ?B) (?isa ?b ?B)\n              \
                           (absent (and (?isa ?a ?A) (?R ?a ?b))))\n  \
               :assert (open ?R)\n  :why \"{?R} owes {?b} an {?A}\")\n\
             (rule slot-owed-room (?R ?isa ?sub ?super ?index)\n  \
               :match (and (?isa ?a ?Ta) (?sub ?Ta ?super) (neq ?Ta ?index)\n              \
                           (absent (and (?isa ?i ?index) (?R ?a ?i))))\n  \
               :assert (open ?R)\n  :why \"{?a} owes a slot\")\n\
             (rule slot-owed-fill (?R ?isa ?sub ?super ?index)\n  \
               :match (and (?isa ?i ?index) (?sub ?Tv ?super) (neq ?Tv ?index)\n              \
                           (absent (and (?isa ?v ?Tv) (?R ?i ?v))))\n  \
               :assert (open ?R)\n  :why \"{?i} owes a value\")\n";
        let (terms, kb) = ok(src);
        for name in [
            "total-owed",
            "surjective-owed",
            "slot-owed-room",
            "slot-owed-fill",
        ] {
            let sym = terms
                .syms
                .get(name)
                .unwrap_or_else(|| panic!("{name} interned"));
            assert!(
                kb.program().obligations.contains(sym),
                "{name} must resolve and route to the obligation pass"
            );
        }
        assert!(
            kb.program().rules.is_empty(),
            "none of the four belongs to the saturation agenda"
        );
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

    /// Was `the_last_query_and_the_last_config_win` until M1c S1c.1.2. Half of
    /// what it pinned is deliberately gone: a second `(query …)` used to load
    /// and be discarded in silence, which is the one thing a file carrying
    /// `:expect` must not do. The config half is untouched, and is here in the
    /// same test because the *contrast* is the decision — a config is a
    /// setting, a query is content.
    #[test]
    fn every_query_is_kept_and_the_last_config_still_wins() {
        let (_terms, kb) = ok("(query :goal (a ?x))\n(query :goal (b ?x) :mode all)\n\
             (config :print-alive true)\n(config :warn-derived-naf true)");
        let p = kb.program();
        assert_eq!(p.queries.len(), 2, "both blocks load");
        assert_eq!(p.queries[0].kw_pairs.len(), 1);
        assert_eq!(p.queries[1].kw_pairs.len(), 2);
        assert_eq!(
            p.query().expect("an active query").kw_pairs.len(),
            1,
            "the active one is the first, not the last"
        );
        let config = p.config.as_ref().expect("a config");
        assert!(config.warn_derived_naf);
        assert!(!config.print_alive, "the earlier block is discarded whole");
    }

    // ── `:expect`, at load — M1c S1c.1.2 T1c.1.2.5 ────────────────
    //
    // Every one of these is a load *error* rather than a run that checks
    // nothing, which is the whole reason the keyword exists.

    #[test]
    fn an_unknown_query_keyword_is_a_load_error() {
        let e = err("(relation p T P)\n(p A H)\n(query :goal (p A ?h) :expct none)");
        assert!(e.contains("unknown keyword :expct"), "{e}");
        assert!(
            e.contains(":expect"),
            "the message lists the allow-list: {e}"
        );
    }

    /// `:mode` is in the allow-list and read by nothing. Three corpus files
    /// carry a comment saying it is obsolete; accepted-and-ignored is a
    /// documented state, silently-unknown is not.
    #[test]
    fn the_obsolete_mode_keyword_still_loads() {
        let (_terms, kb) = ok("(query :goal (a ?x) :mode all)");
        assert_eq!(kb.program().queries.len(), 1);
    }

    #[test]
    fn an_expect_naming_a_relation_the_program_does_not_have_is_a_load_error() {
        let e = err("(relation p T P)\n(p A H)\n\
             (query :goal (p A ?h) :expect (model (p A H) (nosuch A)))");
        assert!(e.contains("nosuch"), "{e}");
        assert!(e.contains("no declaration or fact makes a relation"), "{e}");
    }

    /// Rule 1 — the goal's relations are mandatory. Naming a relation is what
    /// closes it, so "pins what the query asked" and "names it" are one test.
    #[test]
    fn an_expect_that_does_not_name_the_goals_relation_is_a_load_error() {
        let e = err("(relation p T P)\n(relation q T P)\n(p A H)\n(q A H)\n\
             (query :goal (p A ?h) :expect (model (q A H)))");
        assert!(e.contains("does not name p"), "{e}");
    }

    /// …and it is checked per disjunct: one good model beside one bad one is
    /// still a program that would pass by accident.
    #[test]
    fn rule_one_applies_to_every_disjunct() {
        let e = err("(relation p T P)\n(relation q T P)\n(p A H)\n(q A H)\n\
             (query :goal (p A ?h) \
              :expect (or (model (p A H)) (model (q A H))))");
        assert!(e.contains("does not name p"), "{e}");
    }

    #[test]
    fn a_malformed_expect_is_a_load_error() {
        for (src, want) in [
            ("(query :goal (a ?x) :expect all)", "expected `(false)`"),
            (
                "(query :goal (a ?x) :expect (models (a b)))",
                "expected `(false)`",
            ),
            ("(query :goal (a ?x) :expect none)", "expected `(false)`"),
            (
                "(query :goal (a ?x) :expect (model (a ?x)))",
                "not a pattern",
            ),
        ] {
            let e = err(src);
            assert!(e.contains(want), "{src}: {e}");
        }
    }

    /// A query with no `:expect` is untouched by any of the above — the
    /// corpus is 128 entries of exactly that.
    #[test]
    fn a_query_without_expect_is_unaffected() {
        let (_terms, kb) = ok("(relation p T P)\n(p A H)\n(query :goal (p A ?h))");
        assert_eq!(kb.program().queries.len(), 1);
    }

    #[test]
    fn a_later_query_is_reachable_by_index() {
        let src = "(query :goal (a ?x))\n(query :goal (b ?x) :mode all)";
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = crate::parse(&mut ast, src, None).expect("parses");
        let kb = super::load_query(&mut ast, &mut terms, &forms, None, 1).expect("loads");
        assert_eq!(kb.program().query().expect("query 1").kw_pairs.len(), 2);
        let kb = super::load_query(&mut ast, &mut terms, &forms, None, 7).expect("loads");
        assert!(
            kb.program().query().is_none(),
            "out of range is the no-query state, not a panic"
        );
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
