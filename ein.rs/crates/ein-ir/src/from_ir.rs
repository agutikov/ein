//! IR → `Kb` — the loader.
//!
//! The loader walks a **flat sequence** of top-level forms and classifies each
//! by its head: `relation` declares one, `rule` / `hrule` declare a rule,
//! `macro` a pattern macro, `query` / `config` fill their slots, `trace` is
//! engine output and is ignored, and **anything else is a fact**. The former
//! block wrappers are gone (S1.7c.4): a `(facts …)` form loads as a fact whose
//! relation is `facts`, like any other head.
//!
//! It is a validation surface as much as a construction one: errors
//! **accumulate** and surface as one `; `-joined message, so both the text of
//! each error and the order they were found in are observable. `examples/broken/load/` is the fixture set.
//!
//! It lives in `ein-ir` rather than beside the store because it needs the
//! frontend and the data model at once, and `ein-core` depends on nothing.

use crate::ast::{Ast, Node, NodeId, loc_repr};
use crate::imports::Resolver;
use crate::macros::{self, Macro as IrMacro, MacroError};
use crate::parse;
use ein_core::config::{FLAGS, SolverConfig};
use ein_core::entities::{ExprRef, Loc, Macro, Pattern, Query, Relation, Rule, Solve};
use ein_core::{
    FactId, Kb, Overflow, Program, Prov, ProvId, Symbol, Terms, Value, detect_provenance_cycles,
    is_predicate, is_reserved,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// The accumulated problems, `; `-joined.
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

/// What a load requires of a program beyond what the grammar does.
///
/// **One option, and it decides only whether a program loads** — M1f
/// S1f.5.14, T1f.5.14.6, on the user's rulings of 2026-11-08: *":why is
/// optional in grammar and loader has a flag that makes it required, start
/// with default false so not break everything"*, and, asked whether it can
/// change an answer, *"yes, the flag only decides loading"*. A program it lets
/// through loads to the same `Kb` it loads to without it, so it is not a lever
/// under the config principle (`docs/kernel/configuration.md` § 2.5). It is
/// not a `(config …)` field, which a program could set against itself and
/// which the KB-shape digest renders, and not an `EIN_*` name; the CLI's
/// `--require-why` sets it on every command that loads a program.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LoadOptions {
    /// Refuse every `(relation …)`, `(rule …)`, `(hrule …)` and `(macro …)`
    /// the load reads, the stdlib's and an imported module's included, that
    /// carries no `:why "…"`, one line each ([`WHY_REQUIRED_OF`]). A macro
    /// since S1f.5.5's swap (T1f.5.5.4), when the one parser became the one
    /// that reads a macro's `:why` and the stdlib's two macros got theirs.
    /// **Default false**, until S1f.5.14's T1f.5.14.7 turns it to true.
    pub require_why: bool,
}

/// The forms [`LoadOptions::require_why`] asks for a `:why`, as its refusal
/// names them.
pub const WHY_REQUIRED_OF: &str = "relation, rule, hrule and macro";

/// The line [`LoadOptions::require_why`] refuses a form with: its head and its
/// name as written, in the W24 refusals' spelling.
fn missing_why(head: &str, name: &str) -> String {
    format!(
        "{head} '{name}' has no :why \"…\", which this load requires of every {WHY_REQUIRED_OF}"
    )
}

/// Parse and load a `.ein` file, resolving its imports file-relative.
pub fn load_file(ast: &mut Ast, terms: &mut Terms, path: &Path) -> Result<Kb, KbLoadError> {
    load_file_with(ast, terms, path, &LoadOptions::default())
}

/// [`load_file`], under `opts`.
pub fn load_file_with(
    ast: &mut Ast,
    terms: &mut Terms,
    path: &Path,
    opts: &LoadOptions,
) -> Result<Kb, KbLoadError> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| KbLoadError(format!("{}: {e}", path.display())))?;
    let forms = parse(ast, &text, path.to_str()).map_err(|e| KbLoadError(e.to_string()))?;
    load_with(ast, terms, &forms, path.parent(), opts)
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

/// [`load`], under `opts`.
pub fn load_with(
    ast: &mut Ast,
    terms: &mut Terms,
    forms: &[NodeId],
    base_dir: Option<&Path>,
    opts: &LoadOptions,
) -> Result<Kb, KbLoadError> {
    load_query_with(ast, terms, forms, base_dir, 0, opts)
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
    load_query_with(
        ast,
        terms,
        forms,
        base_dir,
        active_query,
        &LoadOptions::default(),
    )
}

/// [`load_query`], under `opts`.
pub fn load_query_with(
    ast: &mut Ast,
    terms: &mut Terms,
    forms: &[NodeId],
    base_dir: Option<&Path>,
    active_query: usize,
    opts: &LoadOptions,
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
    let mut solve_blocks: Vec<NodeId> = Vec::new();
    for &form in &forms {
        // Every top-level form has a name for its head: a declarator's word,
        // or a fact's relation (`ein.y`'s `Form`). Until M1f S1f.5.5's T5 a
        // form that had none was refused here, *unexpected top-level form*,
        // and a fact headed by a claim's word, `(region …)`, `(merged …)` or
        // `(owing …)`, *a region claim belongs under (solve … :expect …)*
        // (M1e2 S1e2.1a.4); since the swap the grammar refuses both, the
        // second because a top-level fact's relation (`FactHead`) is none of
        // the three.
        let head = ast
            .head_name(form)
            .expect("a top-level form's head is a name (ein.y's Form)");
        match head {
            "relation" => relations.push(form),
            "rule" | "hrule" => rules.push(form),
            "macro" => macro_forms.push(form),
            "import" => errors.push(format!(
                "unresolved (import …) at {} — internal error",
                loc_repr(ast, ast.loc(form))
            )),
            "query" => query_blocks.push(form),
            "solve" => solve_blocks.push(form),
            "config" => config_blocks.push(form),
            // Engine-emitted output; parsed by the trace reader, not here.
            "trace" => {}
            _ => facts.push(form),
        }
    }

    // Pass 0 — macros, first, because the rules pass expands invocations.
    let declared_macros = ingest_macros(ast, terms, &mut kb, &macro_forms, &mut errors)?;
    // A macro's `:why` is `Macro::why` (`LoadOptions::require_why`).
    if opts.require_why {
        for m in declared_macros.values().filter(|m| m.why.is_none()) {
            errors.push(missing_why("macro", &m.name));
        }
    }
    for m in declared_macros.values() {
        if let Some(why) = &m.why {
            refuse_unreachable_references(
                &format!("macro '{}'", m.name),
                ":why",
                why,
                &m.params,
                &mut errors,
            );
        }
    }
    // Pass 1 — relations, with their auto-stored declaration facts.
    for form in relations {
        ingest_relation(ast, terms, &mut kb, form, opts, &mut errors)?;
    }
    // Pass 2 — rules. After this, rule-name resolution is possible.
    ingest_rules(
        ast,
        terms,
        &mut kb,
        &rules,
        &declared_macros,
        opts,
        &mut errors,
    )?;
    // Pass 3 — facts.
    for &form in &facts {
        ingest_fact(ast, terms, &mut kb, form)?;
    }
    // A rule parameter that names a relation, bound to a nested fact by an
    // activator the program writes down (M1e2 S1e2.2.4, S1e2.2.24).
    validate_relation_parameters(ast, terms, &kb, &facts, &mut errors);

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
    // One lattice per program (M1e2 S1e2.1a.4): a second `(solve …)` would be
    // a second R, which is what splitting solving from asking removed.
    if solve_blocks.len() > 1 {
        errors.push(format!(
            "{} (solve …) forms — a program states its lattice once",
            solve_blocks.len()
        ));
    }
    kb.program_mut().solve = solve_blocks.first().map(|&form| Solve {
        kw_pairs: ast.form_args(form).iter().map(|n| ExprRef(n.0)).collect(),
    });
    validate_solve(ast, terms, &kb, &mut errors);
    validate_queries(ast, terms, &kb, &mut errors);
    if let Some(&last) = config_blocks.last() {
        let args: Vec<NodeId> = ast.form_args(last).to_vec();
        kb.program_mut().config = Some(config_from_kw_pairs(ast, &args));
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
/// A *seeded* head — `(?rel ?a ?b)` — names the relation its activator binds,
/// which is a name the program has or a nested fact, refused
/// (`validate_relation_parameters` where the activator is written down, the
/// compiler where a rule derives it). Until M1f S1f.5.4 it could also bind an
/// integer, whose decimal text the compiler would have interned as a symbol,
/// refused the same way since M1e2 S1e2.2.4; since S1f.5.5's swap a run of
/// digits is a name, interned here like any other.
fn intern_program_names(ast: &Ast, terms: &mut Terms, kb: &Kb) {
    fn walk(ast: &Ast, terms: &mut Terms, node: NodeId) {
        match ast.node(node) {
            Node::Atom(s) | Node::Var(s) => {
                let _ = terms.intern_text(ast.sym(s));
            }
            // A string is natural-language text, here a query's `:goal-text`,
            // and names nothing (M1f S1f.5.2): until T1f.5.2.3 it was
            // interned as a name, which nothing asked for.
            Node::Str(_) | Node::Keyword(_) => {}
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
    // …and the `(solve …)` block, whose `:hrules` activators bind hrule
    // parameters to names no fact may mention.
    if let Some(s) = &program.solve {
        roots.extend(s.kw_pairs.iter().map(|e| NodeId(e.0)));
    }
    for root in roots {
        walk(ast, terms, root);
        let _ = intern_guard_facts(ast, terms, root);
    }
}

/// Intern every fact an `eq` or `neq` guard compares, `(f a)` in `(neq ?x (f
/// a))`, for the same reason [`intern_program_names`] interns names: the
/// compiler reads it as a value (M1f S1f.5.7, the user's ruling of 2026-11-07
/// on `W24/form-in-guard`), and must not intern it for the first time during
/// a search. A form that holds a variable is refused (Q-M1f.30), and holds
/// none of these.
fn intern_guard_facts(ast: &Ast, terms: &mut Terms, node: NodeId) -> Result<(), Overflow> {
    match ast.node(node) {
        Node::KwPair { value, .. } => intern_guard_facts(ast, terms, value),
        Node::SForm { args, .. } => {
            let args = ast.args(args).to_vec();
            if matches!(ast.head_name(node), Some("eq") | Some("neq")) {
                for &a in &args {
                    let mut vars = Vec::new();
                    walk_pattern(ast, a, &mut vars, &mut Vec::new());
                    if matches!(ast.node(a), Node::SForm { .. }) && vars.is_empty() {
                        fact_args(ast, terms, &[a])?;
                    }
                }
            }
            for a in args {
                intern_guard_facts(ast, terms, a)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// The `(solve …)` block's `:expect`, whose claims [`crate::claim::parse`]
/// checks and whose relations must be the program's, as a query's
/// `:expect`'s are.
///
/// **Which keywords a `(solve …)` and a `(query …)` take is the grammar's**
/// since M1f S1f.5.5's swap (S1f.5.11's ruling, `ein.y`'s `SolveClause` and
/// `QueryClause`): any other is a parse error at its place, a query's lattice
/// keyword with a note naming its home on `(solve …)`. Until T1f.5.5.5 each
/// form had an allow-list here, `SOLVE_KEYWORDS` and `QUERY_KEYWORDS` (M1c
/// [S1c.1.2](../../../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects),
/// M1e2 S1e2.1a.4), which no program the grammar accepts reached since.
fn validate_solve(ast: &Ast, terms: &Terms, kb: &Kb, errors: &mut Vec<String>) {
    let Some(solve) = kb.program().solve() else {
        return;
    };
    let Some(node) = solve_keyword(ast, &solve.kw_pairs, "expect") else {
        return;
    };
    let claim = match crate::claim::parse(ast, node) {
        Ok(c) => c,
        Err(message) => {
            errors.push(message);
            return;
        }
    };
    let crate::claim::Claim::Region(region) = &claim else {
        return;
    };
    let models = region.models.iter().chain(region.owing.iter()).chain(
        region
            .merged
            .iter()
            .flat_map(|m| std::iter::once(&m.to).chain(&m.from)),
    );
    let mut reported: BTreeSet<String> = BTreeSet::new();
    for model in models {
        for &f in &model.facts {
            let fact = crate::expect::fact(ast, f);
            // `eq` and `neq` are relations whose facts are computed and never
            // stored (M1f S1f.5.7, Q-M1e2.26), and a region claim lists a
            // state's stored facts, so one that lists either can never hold
            // (S1f.5.7's default, taken on the user's instruction of
            // 2026-11-09).
            if is_predicate(fact.relation) {
                if reported.insert(fact.relation.to_string()) {
                    errors.push(format!(
                        "(solve … :expect …) names {}, which a state never stores — eq and neq \
                         are computed from their definition, so a region claim, which lists a \
                         state's facts, holds neither",
                        fact.relation
                    ));
                }
                continue;
            }
            let known = terms
                .syms
                .get(fact.relation)
                .is_some_and(|sym| kb.program().relations.get(sym).is_some());
            if !known && reported.insert(fact.relation.to_string()) {
                errors.push(format!(
                    "(solve … :expect …) names {}, which no declaration or fact makes a relation",
                    fact.relation
                ));
            }
        }
    }
}

/// A query's `:goal` that writes an `eq` or `neq` guard over a form holding a
/// variable, refused as a rule's is ([`refuse_no_fact_holds`], Q-M1f.30): one
/// line per form, in Ein's spelling.
fn refuse_guard_patterns(ast: &Ast, node: NodeId, errors: &mut Vec<String>) {
    let Node::SForm { args, .. } = ast.node(node) else {
        return;
    };
    let args = ast.args(args).to_vec();
    if matches!(ast.head_name(node), Some("eq") | Some("neq")) {
        let mut seen: Vec<String> = Vec::new();
        for &a in &args {
            let mut vars = Vec::new();
            walk_pattern(ast, a, &mut vars, &mut Vec::new());
            let text = crate::dump::dump_compact(ast, a);
            if matches!(ast.node(a), Node::SForm { .. })
                && !vars.is_empty()
                && !seen.contains(&text)
            {
                errors.push(format!(
                    "query: its :goal writes {}, and the form {text} holds a variable — a guard \
                     compares two values, each a variable, a name, or a fact that holds no \
                     variable",
                    crate::dump::dump_compact(ast, node),
                ));
                seen.push(text);
            }
        }
        return;
    }
    for a in args {
        refuse_guard_patterns(ast, a, errors);
    }
}

/// A `(solve …)` keyword's value node. The **first** match wins.
fn solve_keyword(ast: &Ast, kw_pairs: &[ExprRef], want: &str) -> Option<NodeId> {
    for &pair in kw_pairs {
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

/// The `(query …)` blocks' own validation — `:over`, and `:expect`. Which
/// keywords a query takes, and a claim's shape, are the grammar's since M1f
/// S1f.5.5's swap ([`validate_solve`]).
///
/// Runs after the fact pass, because two of the three `:expect` rules are
/// about the *program*: a relation it names must be one the program knows, and
/// the goal's relations must be among the ones it closes.
fn validate_queries(ast: &Ast, terms: &Terms, kb: &Kb, errors: &mut Vec<String>) {
    for query in &kb.program().queries {
        if let Some(text) = string_value(ast, query_keyword(ast, query, "goal-text")) {
            let mut vars: Vec<String> = Vec::new();
            if let Some(goal) = query_keyword(ast, query, "goal") {
                walk_pattern(ast, goal, &mut vars, &mut Vec::new());
            }
            refuse_unreachable_references("query", ":goal-text", text, &vars, errors);
        }
        if let Some(goal) = query_keyword(ast, query, "goal") {
            refuse_guard_patterns(ast, goal, errors);
        }
        if let Some(node) = query_keyword(ast, query, "over")
            && let Err(message) = crate::expect::Over::parse(ast, node)
        {
            errors.push(message);
        }
        let Some(node) = query_keyword(ast, query, "expect") else {
            continue;
        };
        let expectation = crate::expect::parse(ast, node);
        // Rule 3's precondition: a relation an expectation *closes* has to be
        // one the program has. A name nothing declares and no fact uses would
        // close a relation that does not exist, and pass — vacuously, for
        // ever.
        for model in expectation.models() {
            for &f in &model.facts {
                let fact = crate::expect::fact(ast, f);
                // A query's claim lists its goal's instances, and `eq` and
                // `neq` are relations a goal may ask about (M1f S1f.5.7,
                // Q-M1e2.26): their facts are computed, so no declaration or
                // fact makes them one, and the check below is the goal's.
                let known = is_predicate(fact.relation)
                    || terms
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
        // Since M1e2 T1e2.1a.5.3 a query's `:expect` lists its projection
        // classes (`region_language.md` § 5): each `(model …)` is one class,
        // the goal's atoms under its rows. So a listed fact must be one a goal
        // atom matches — anything else is a claim about the state, which is
        // the region claim's — and a class is listed once.
        let atoms = query_keyword(ast, query, "goal")
            .map(|g| crate::expect::goal_atoms(ast, g))
            .unwrap_or_default();
        let mut classes: Vec<Vec<String>> = Vec::new();
        for model in expectation.models() {
            for &f in &model.facts {
                if !atoms.iter().any(|&a| crate::expect::matches(ast, f, a)) {
                    errors.push(format!(
                        ":expect lists {}, which no atom of the query's :goal matches — a \
                         query's claim lists its projection classes; a claim about the state \
                         is (solve … :expect (region …))'s",
                        crate::expect::render(ast, f)
                    ));
                }
            }
            let mut class: Vec<String> = model
                .facts
                .iter()
                .map(|&f| crate::expect::render(ast, f))
                .collect();
            class.sort();
            class.dedup();
            if classes.contains(&class) {
                errors.push(format!(
                    ":expect lists the class (model {}) twice",
                    class.join(" ")
                ));
            } else {
                classes.push(class);
            }
        }
    }
}

/// **A template that names a variable no reference can reach is refused** —
/// M1f S1f.5.4's T1f.5.4.5 (W37), taken by default on the user's instruction
/// of 2026-11-09 as S1f.5.5's T9 review recommended. A reference reads a
/// variable's name up to its first `}` ([`ein_core::why::render_why`]), and
/// `}` is a name character, so `{?a}b}` is the reference `{?a}` and the text
/// `b}`: a rule's `:why`, a macro's or a query's `:goal-text` that writes one
/// for a variable in its scope (`vars`, names without their `?`) would print a
/// sentence its author did not write. One line a reference, naming `what` and
/// its clause. A relation's `:why` names its slots by position and is not read.
fn refuse_unreachable_references(
    what: &str,
    clause: &str,
    template: &str,
    vars: &[String],
    errors: &mut Vec<String>,
) {
    for v in ein_core::why::unreachable_references(template, vars.iter().map(String::as_str)) {
        errors.push(format!(
            "{what}: its {clause} writes {{?{v}}}, a reference to ?{v}, which no reference can \
             name — a reference reads a variable's name up to its first }}"
        ));
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
/// strings-only guard the loader applies to `:why` and `:source`.
fn string_value(ast: &Ast, node: Option<NodeId>) -> Option<&str> {
    match node.map(|n| ast.node(n)) {
        Some(Node::Str(s)) => Some(ast.sym(s)),
        _ => None,
    }
}

/// A name as a fact's argument: the symbol it spells. Anything else (a
/// nested form, and the shapes the grammar refuses in a fact) answers `None`.
///
/// A fact holds names and facts (`ein.y`'s `FactArg`). Until M1f S1f.5.5's
/// swap a string there was the name it spells, `(p "a")` the fact `(p a)`
/// (M1e2 S1e2.2.16), and a variable the name `?x`, printed `"?x"`; the swap
/// refused both at the parse (S1f.5.2's ruling and its G-6), and T1f.5.2.3
/// retired the two arms, each shown unreachable first.
fn atomic_value(ast: &Ast, terms: &mut Terms, node: NodeId) -> Result<Option<Value>, Overflow> {
    Ok(Some(match ast.node(node) {
        Node::Atom(s) => terms.value_text(ast.sym(s))?,
        Node::Str(_) | Node::Var(_) => unreachable!(
            "a fact holds names and facts, no string and no variable (ein.y's FactArg)"
        ),
        _ => return Ok(None),
    }))
}

/// Fact arguments: kw-pairs dropped, atomics flattened, a nested form interned
/// as a **relational node**.
///
/// A nested form whose head is not a bare atom takes the head
/// [`ein_core::text::NESTED_HEAD`], which is a lossy collapse.
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
            let head_name = ast
                .atom_name(head)
                .unwrap_or(ein_core::text::NESTED_HEAD)
                .to_string();
            let inner = ast.args(args).to_vec();
            let inner = fact_args(ast, terms, &inner)?;
            let rel = terms.intern_text(&head_name)?;
            out.push(terms.value_fact(rel, &inner)?);
        }
    }
    Ok(out)
}

// ── Pass 0 — macros ────────────────────────────────────────────────

/// Read the `(macro …)` declarations, then intern them into the program.
///
/// The **reading** is [`macros::collect_macros`]'s since M1e S1e.3.1 — one
/// implementation, so the dump path and the loader refuse the same programs
/// with the same sentences (`CO-M4`). What stays here is the half that needs a
/// `Terms` and a `Kb`: interning the name and the parameters, and storing the
/// macro on the program.
fn ingest_macros(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    forms: &[NodeId],
    errors: &mut Vec<String>,
) -> Result<BTreeMap<String, IrMacro>, Overflow> {
    let (declared, problems) = macros::read_macros(ast, forms);
    errors.extend(problems);
    for m in declared.values() {
        let name = terms.intern_text(&m.name)?;
        let params: Vec<Symbol> = m
            .params
            .iter()
            .map(|p| terms.intern_text(p))
            .collect::<Result<_, _>>()?;
        kb.program_mut().macros.insert_new(
            name,
            Macro {
                name,
                params: params.into_boxed_slice(),
                body: ExprRef(m.body.0),
                loc: core_loc(ast, m.form),
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
    opts: &LoadOptions,
    errors: &mut Vec<String>,
) -> Result<(), Overflow> {
    let loc = loc_repr(ast, ast.loc(form));
    let args = ast.form_args(form).to_vec();
    // Flat args post-R10. An EMPTY signature is legal since S1.22.4 —
    // `(relation R)` declares a relation node with no declared arg types, and
    // is deliberately not a hypothesis target, since the "declared domain
    // relation" signal is signature *presence*.
    //
    // A declaration is a name, then names, then its `:why` (`ein.y`'s
    // `Relation`). Until M1f S1f.5.5's T5 this pass refused `(relation)`,
    // *needs a name*, and a signature holding anything but names, *malformed
    // (relation)*, which kept the wrapped `(relation R (T1 T2))` from loading
    // as a bare declaration; since the swap the grammar refuses both.
    let name = ast
        .atom_name(args[0])
        .expect("a declaration's first argument is its name (ein.y's Relation)")
        .to_string();
    if opts.require_why && string_value(ast, kw_get(&kw_pairs(ast, &args), "why")).is_none() {
        errors.push(missing_why("relation", &name));
    }
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
    opts: &LoadOptions,
    errors: &mut Vec<String>,
) -> Result<(), Overflow> {
    for &form in forms {
        let head = ast.head_name(form).expect("bucketed by head").to_string();
        let loc = loc_repr(ast, ast.loc(form));
        let args = ast.form_args(form).to_vec();
        // A rule is a name, a parameter list, then its clauses (`ein.y`'s
        // `Rule`). Until M1f S1f.5.5's T5 this pass refused a rule with fewer
        // than two arguments, *needs name + params*, and one whose name or
        // parameter list was not one, *malformed*; since the swap the grammar
        // refuses both.
        let name = ast
            .atom_name(args[0])
            .expect("a rule's first argument is its name (ein.y's Rule)")
            .to_string();
        let params_form = args[1];
        if opts.require_why && string_value(ast, kw_get(&kw_pairs(ast, &args), "why")).is_none() {
            errors.push(missing_why(&head, &name));
        }
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
        // A wildcard as a relation, in a rule — M1e2 S1e2.2.20 (W20,
        // Q-M1e2.14 (c)) — and the shapes S1e2.2.24 refuses (W24).
        let n_refused = errors.len();
        let param_vars: Vec<&str> = ast
            .form_args(params_form)
            .iter()
            .filter_map(|&a| match ast.node(a) {
                Node::Var(s) => Some(ast.sym(s)),
                _ => None,
            })
            .collect();
        refuse_no_fact_holds(ast, &head, &name, &param_vars, ":match", match_node, errors);
        refuse_no_fact_holds(
            ast,
            &head,
            &name,
            &param_vars,
            ":assert",
            assert_node,
            errors,
        );
        if errors.len() != n_refused {
            continue;
        }
        let why = match string_value(ast, kw_get(&pairs, "why")) {
            Some(w) => Some(terms.intern_text(w)?),
            None => None,
        };
        if let Some(w) = string_value(ast, kw_get(&pairs, "why")) {
            let mut vars: Vec<String> = Vec::new();
            for node in [params_form, match_node, assert_node] {
                walk_pattern(ast, node, &mut vars, &mut Vec::new());
            }
            refuse_unreachable_references(&format!("{head} '{name}'"), ":why", w, &vars, errors);
        }
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

/// **What no fact can hold, and what reads nothing, in a rule is refused** —
/// M1e2 S1e2.2.20, W20, on the user's rulings of 2026-11-04 (Q-M1e2.13 (a))
/// and 2026-11-05 (Q-M1e2.14 (c)); and S1e2.2.24, W24, on the user's rulings
/// of 2026-11-05 (Q-M1e2.16 (1), Q-M1e2.18).
///
/// The pass reads a rule's clauses as the compiler does: a `:match` as
/// premises, each pattern's arguments as slots, and an `:assert` as
/// conclusions, given the rule's parameters. It refuses, one line per slot
/// as written, naming the rule and the clause:
///
/// - a **relation written as a variable no parameter binds**, `(?r b)`, at
///   the top level or inside a pattern, in either clause: a relation is named
///   by a name or by a parameter its activator binds (Q29);
/// - a **premise that is not a form**, the whole `:match`, a member of an
///   `(and …)` or an `(or …)`, an `(absent …)`'s body: since M1f S1f.5.5's
///   swap only a macro's body puts one there, a name or a variable;
/// - a **guard's argument that is a pattern**, a form that holds a variable,
///   `(eq ?x (f ?y))` (Q-M1f.30, taken by default on the user's instruction of
///   2026-11-09).
///
/// **Until M1f S1f.5.7 it refused every form as a guard's argument, and a
/// variable no premise before the guard binds** (Q-M1e2.18 (i)(a) and
/// (ii)(a), since S1e2.2.24). The user's ruling of 2026-11-07 on Q-M1e2.26
/// answers both: `eq` and `neq` are relations, a fact that holds no variable
/// is a value as it is in `(p (f a))`, and a premise means the same in any
/// order of its conjuncts, so a guard waits for the premise that binds its
/// variable, and a variable nothing binds ranges over the program's names
/// (`ein_infer::compile`'s `Seq`).
///
/// **Since M1f S1f.5.5's swap the grammar refuses the rest**, and T1f.5.5.5
/// deleted their lines: a wildcard written as a relation, `(_ a)`; a keyword
/// pair inside a pattern inside a pattern, `(box (a :k v))`; `_` or a keyword
/// pair as a guard's argument; and an atom, `_`, a string or a variable
/// written where a premise goes. Each is a row of the grammar's table
/// (`ein-ir/tests/grammar_pages.rs`).
///
/// **Until the swap three more shapes were not refused here.** A keyword pair
/// at a pattern's top level, `(n ?x :where v)`, or as a premise of an `(and
/// …)`, `:where (neq ?a ?b)`, which this walk skipped and the compiler dropped
/// (Q32), so the rule fired as if it were not written; `_` as a pattern's
/// argument, which matched any value (W20, S1e2.2.20); and `_` or a keyword
/// pair at an `:assert`'s conclusion's top level, which stopped the firing with
/// the line `ein_infer::firing::FireError::NotAFact` printed. The grammar
/// refuses each since (S1f.5.11's ruling, S1f.5.1's), and S1f.5.11's
/// T1f.5.11.2 retired this walk's skips with the compiler's drop: a keyword
/// pair met as a premise would be refused as a premise that is no form.
///
/// **Until S1e2.2.20** the compiler lowered a range as an opaque slot that
/// matched nothing, in silence, and a wildcard head stopped compilation with
/// the Python IR's spelling of the pattern. From S1e2.2.20 this pass refused a
/// range in a rule too, until M1f S1f.5.3 retired the token: Ein has no
/// ranges (Q-M1e2.24), and `1..3` is a parse error.
///
/// **Until S1e2.2.24** each of the rest read nothing, in silence: a guard
/// compared `_`, a form or a keyword
/// pair as a raw IR node, equal to no value, and an unbound variable as a
/// null, so `(eq ?x _)` never held and `(and (neq ?x a) (p ?x))` fired for
/// `a`; the compiler dropped a premise that is not a form, so the rule fired
/// as if it were not written; and a pattern inside a pattern whose relation
/// no parameter binds, or that holds a keyword pair, was an opaque slot that
/// matched nothing. The same variable head at the top level, and an
/// `(absent …)` whose body is no form, stopped compilation with a
/// `CompileError` in the Python IR's spelling, and in an `:assert` the firing
/// stopped with `NotAFact`'s line.
fn refuse_no_fact_holds(
    ast: &Ast,
    head: &str,
    name: &str,
    params: &[&str],
    clause: &str,
    node: NodeId,
    errors: &mut Vec<String>,
) {
    // Four more were refused here until M1f S1f.5.5's T5, each a shape the
    // grammar refuses since the swap, so no program reached them: a wildcard
    // written as a relation (`WildHead`), a keyword pair inside a pattern
    // inside a pattern (`NestedKw`), `_` as a guard's argument (`WildGuard`)
    // and a keyword pair as one (`GuardNode`'s other half). The parser builds
    // no `_` (S1f.5.1's ruling) and no keyword pair inside a clause's value
    // (S1f.5.11's), and macro expansion copies only what a program wrote.
    // `NotAForm` stays: a macro's body is a template, and `(macro id (?x) ?x)`
    // called as `(id foo)` leaves the name `foo` where a premise goes.
    enum Found {
        /// A form whose relation is a variable no parameter binds.
        FreeHead(NodeId, NodeId),
        /// A premise that is not a form, and what it is a premise of.
        NotAForm(NodeId, Option<NodeId>),
        /// A guard, and its argument that is a form holding a variable.
        GuardNode(NodeId, NodeId),
    }
    struct Walk<'a> {
        ast: &'a Ast,
        params: &'a [&'a str],
        out: Vec<Found>,
    }
    impl Walk<'_> {
        fn form_head(&mut self, node: NodeId, head: NodeId) {
            if let Node::Var(s) = self.ast.node(head)
                && !self.params.contains(&self.ast.sym(s))
            {
                self.out.push(Found::FreeHead(node, head))
            }
        }
        /// A pattern's argument, at any depth below a pattern.
        fn slot(&mut self, node: NodeId) {
            let ast = self.ast;
            if let Node::SForm { head, args } = ast.node(node) {
                self.form_head(node, head);
                for &a in ast.args(args) {
                    self.slot(a);
                }
            }
        }
        /// A pattern written as a premise or a conclusion: its relation, and
        /// its arguments as slots. Until M1f S1f.5.5's swap its own keyword
        /// pairs were skipped here, the compiler's to drop (Q32) or the
        /// firing's to refuse; since, a pattern holds none (`ein.y`'s `Args`
        /// and `Terms`), and S1f.5.11's T1f.5.11.2 retired the skip.
        fn pattern(&mut self, node: NodeId, bound: Option<&mut Vec<String>>) {
            let ast = self.ast;
            let Node::SForm { head, args } = ast.node(node) else {
                return;
            };
            self.form_head(node, head);
            let mut bound = bound;
            for &a in ast.args(args) {
                self.slot(a);
                if let Some(b) = bound.as_deref_mut() {
                    vars_in(ast, a, b);
                }
            }
        }
        /// A guard, `(eq a b)` or `(neq a b)`: a relation whose facts are
        /// computed (M1f S1f.5.7, Q-M1e2.26), so each argument is a value — a
        /// variable, a name, or a fact that holds no variable — and a variable
        /// no premise binds is one the guard binds, in any order of the
        /// conjuncts. A form that holds a variable is a pattern, and is
        /// refused (Q-M1f.30).
        ///
        /// Until S1f.5.7 a form was refused here whatever it held (Q-M1e2.18
        /// (i)(a)), and so was a variable no premise *before* the guard binds
        /// (Q-M1e2.18 (ii)(a)), each since M1e2 S1e2.2.24 (W24); the user's
        /// ruling of 2026-11-07 answers both.
        fn guard(&mut self, node: NodeId) {
            let ast = self.ast;
            // One line per argument as written: `(eq (f ?x) (f ?x))` names
            // its once.
            let mut seen: Vec<String> = Vec::new();
            for &a in ast.form_args(node) {
                if !matches!(ast.node(a), Node::SForm { .. }) {
                    continue;
                }
                let mut vars = Vec::new();
                vars_in(ast, a, &mut vars);
                let text = crate::dump::dump_compact(ast, a);
                if !vars.is_empty() && !seen.contains(&text) {
                    seen.push(text);
                    self.out.push(Found::GuardNode(node, a));
                }
            }
        }
        /// A premise, with the variables the premises before it bind: the
        /// rule's parameters, a positive pattern's variables, and inside an
        /// `(absent …)` the enclosing premises' before it, as the compiler
        /// scopes a guard.
        fn premise(&mut self, node: NodeId, within: Option<NodeId>, bound: &mut Vec<String>) {
            let ast = self.ast;
            // A keyword pair was skipped here, a premise Q32 dropped, until
            // S1f.5.11's T1f.5.11.2; no premise is one since M1f S1f.5.5's
            // swap, and one met would be a premise that is no form.
            match ast.node(node) {
                Node::SForm { args, .. } => match ast.head_name(node) {
                    Some("absent") => {
                        let mut inner = bound.clone();
                        for &a in ast.args(args) {
                            self.premise(a, Some(node), &mut inner);
                        }
                    }
                    Some("and") => {
                        for &a in ast.args(args) {
                            self.premise(a, Some(node), bound);
                        }
                    }
                    Some("or") => {
                        for &a in ast.args(args) {
                            self.premise(a, Some(node), &mut bound.clone());
                        }
                    }
                    Some("eq") | Some("neq") => self.guard(node),
                    _ => self.pattern(node, Some(bound)),
                },
                _ => self.out.push(Found::NotAForm(node, within)),
            }
        }
        /// A conclusion: a top-level `(and …)`'s members, each a pattern.
        fn conclusion(&mut self, node: NodeId, within: Option<NodeId>) {
            let ast = self.ast;
            match ast.node(node) {
                Node::SForm { args, .. }
                    if within.is_none() && ast.head_name(node) == Some("and") =>
                {
                    // Each member, a conjunct (`ein.y`'s `Conclusion`): a
                    // keyword pair beside them was skipped until S1f.5.11's
                    // T1f.5.11.2, and none parses there since M1f S1f.5.5's
                    // swap.
                    for &a in ast.args(args) {
                        self.conclusion(a, Some(node));
                    }
                }
                Node::SForm { .. } => self.pattern(node, None),
                _ => {}
            }
        }
    }
    fn vars_in(ast: &Ast, node: NodeId, out: &mut Vec<String>) {
        match ast.node(node) {
            Node::Var(s) => {
                let s = ast.sym(s);
                if !out.iter().any(|o| o == s) {
                    out.push(s.to_string());
                }
            }
            Node::SForm { .. } => {
                for &a in ast.form_args(node) {
                    vars_in(ast, a, out);
                }
            }
            _ => {}
        }
    }
    let mut walk = Walk {
        ast,
        params,
        out: Vec::new(),
    };
    if clause == ":match" {
        let mut bound: Vec<String> = params.iter().map(|p| p.to_string()).collect();
        walk.premise(node, None, &mut bound);
    } else {
        walk.conclusion(node, None);
    }
    let dc = |n: NodeId| crate::dump::dump_compact(ast, n);
    const PREMISE: &str = "a premise is a pattern (relation argument …), a guard (eq a b) or \
                           (neq a b), an (absent …), or an (and …) or (or …) of premises";
    const GUARD: &str = "a guard compares two values, each a variable, a name, or a fact \
                         that holds no variable";
    for f in walk.out {
        errors.push(match f {
            Found::FreeHead(form, h) => format!(
                "{head} '{name}': its {clause} writes {} with {} as its relation, which no \
                 parameter binds — a relation is named by a name or by a parameter its \
                 activator binds",
                dc(form),
                dc(h)
            ),
            Found::NotAForm(p, within) => {
                let place = match within.map(|form| ast.node(form)) {
                    Some(Node::SForm { head: h, .. }) => {
                        format!("writes {} as a premise of {}", dc(p), dc(h))
                    }
                    _ => format!("is {}", dc(p)),
                };
                format!(
                    "{head} '{name}': its {clause} {place}, and {} is no premise — {PREMISE}",
                    dc(p)
                )
            }
            Found::GuardNode(guard, arg) => format!(
                "{head} '{name}': its {clause} writes {}, and the form {} holds a variable — \
                 {GUARD}",
                dc(guard),
                dc(arg)
            ),
        });
    }
}

/// **A rule parameter that names a relation is bound to a name** — M1e2
/// [S1e2.2.4](../../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.4_fresh_name_unreachable.md),
/// Q-M1e.21's residue.
///
/// `(rule mirror (?R) :match (q ?a ?b) :assert (?R ?b ?a))` names the relation
/// it matches and asserts by its parameter, and the activator binds it:
/// `(mirror q)` asserts `q` facts. An activator that binds it to a **nested
/// fact**, `(mirror (p a))`, names no relation: a nested fact binds no
/// parameter (M1e2 S1e2.2.24, W24, Q-M1e2.18 (iv)), so it is refused.
///
/// **Here, where the activator is written down**: a fact, or a literal in a
/// rule's `:assert`. An activator a rule derives through a variable is known
/// only to a run, and the compiler refuses it when it binds one
/// (`ein_infer::compile`). An activator of the wrong arity activates nothing
/// (`activators_for`'s filter), so it is not checked. Until M1f S1f.5.4 an
/// activator could also bind an **integer**, `(mirror 3)`, which would have
/// had the compiler name a relation by the integer's decimal text, a name the
/// program does not have (Q-M1e.21 (b): rules cannot assert new names), and
/// was refused here since M1e2 S1e2.2.4; since S1f.5.5's swap `3` is a name
/// of the program and `(mirror 3)` names the relation `3`.
fn validate_relation_parameters(
    ast: &Ast,
    terms: &Terms,
    kb: &Kb,
    facts: &[NodeId],
    errors: &mut Vec<String>,
) {
    // Each rule with a parameter that names a relation: its name, its arity,
    // and those parameters with their positions.
    struct Named {
        rule: Symbol,
        arity: usize,
        heads: Vec<(usize, Symbol)>,
    }
    let mut named: Vec<Named> = Vec::new();
    for registry in [
        &kb.program().rules,
        &kb.program().hrules,
        &kb.program().obligations,
    ] {
        for (name, rule) in registry.iter() {
            let heads: Vec<(usize, Symbol)> = rule
                .params
                .iter()
                .copied()
                .enumerate()
                .filter(|&(_, p)| {
                    [rule.match_.as_ref(), rule.assert_.as_ref()]
                        .into_iter()
                        .flatten()
                        .any(|pat| names_relation(ast, NodeId(pat.expr.0), terms.sym(p)))
                })
                .collect();
            if !heads.is_empty() {
                named.push(Named {
                    rule: name,
                    arity: rule.params.len(),
                    heads,
                });
            }
        }
    }
    if named.is_empty() {
        return;
    }
    let mut check = |form: NodeId| {
        let Some(sym) = ast.head_name(form).and_then(|h| terms.syms.get(h)) else {
            return;
        };
        let Some(Named { rule, arity, heads }) = named.iter().find(|n| n.rule == sym) else {
            return;
        };
        let args: Vec<NodeId> = ast
            .form_args(form)
            .iter()
            .copied()
            .filter(|&a| !matches!(ast.node(a), Node::KwPair { .. }))
            .collect();
        if args.len() != *arity {
            return;
        }
        for &(i, param) in heads {
            // A nested fact binds no parameter (`ein_infer::compile`), so
            // the relation it would name is none — M1e2 S1e2.2.24, W24,
            // Q-M1e2.18 (iv).
            if let Node::SForm { .. } = ast.node(args[i]) {
                errors.push(format!(
                    "rule '{}': its parameter ?{} names a relation, and {} binds it to the \
                     fact {} — a relation is named by a symbol, not a fact",
                    terms.sym(*rule),
                    terms.sym(param),
                    crate::dump::dump_compact(ast, form),
                    crate::dump::dump_compact(ast, args[i]),
                ));
            }
        }
    };
    for &form in facts {
        check(form);
    }
    // A rule's `:assert` derives an activator like a fact states one; walk
    // every form under it, so `(and (mirror 3) …)` is read too.
    let mut asserted: Vec<NodeId> = Vec::new();
    for registry in [
        &kb.program().rules,
        &kb.program().hrules,
        &kb.program().obligations,
    ] {
        for (_, rule) in registry.iter() {
            if let Some(a) = rule.assert_.as_ref() {
                sforms_under(ast, NodeId(a.expr.0), &mut asserted);
            }
        }
    }
    for form in asserted {
        check(form);
    }
}

/// Does `?param` stand as the head of a form anywhere under `node`?
fn names_relation(ast: &Ast, node: NodeId, param: &str) -> bool {
    match ast.node(node) {
        Node::SForm { head, .. } => {
            matches!(ast.node(head), Node::Var(s) if ast.sym(s) == param)
                || ast
                    .form_args(node)
                    .iter()
                    .any(|&a| names_relation(ast, a, param))
        }
        Node::KwPair { value, .. } => names_relation(ast, value, param),
        _ => false,
    }
}

/// Every form under `node`, `node` included, in pre-order.
fn sforms_under(ast: &Ast, node: NodeId, out: &mut Vec<NodeId>) {
    if let Node::SForm { .. } = ast.node(node) {
        out.push(node);
        for &a in ast.form_args(node) {
            sforms_under(ast, a, out);
        }
    }
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

/// The top-level `:assert` conjuncts — `(and c1 … ck)` → the `ci`, else `[expr]`.
///
/// Every argument of the `and` is a conjunct, and every argument of an
/// `(open …)` one of its own: a term holds no keyword pair since M1f S1f.5.5's
/// swap (S1f.5.11). Until S1f.5.11's T1f.5.11.2 both read a form's arguments
/// with its keyword pairs filtered out (`pos_args`).
fn top_conjuncts(ast: &Ast, expr: NodeId) -> Vec<NodeId> {
    if matches!(ast.node(expr), Node::SForm { .. }) && ast.head_name(expr) == Some("and") {
        return ast.form_args(expr).to_vec();
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
        let args = ast.form_args(o).to_vec();
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
            // A wildcard was a slot the `absent` introduced, as a fresh
            // variable is (M1e2 S1e2.2.20, W20), until S1f.5.5's swap refused
            // a bare `_` (S1f.5.1).
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
                && !matches!(
                    name.as_str(),
                    ein_core::text::EMPTY_HEAD | ein_core::text::PARAMS_HEAD
                )
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

/// One fact, with its provenance. A fact's relation is a name (`ein.y`'s
/// `FactHead`); until M1f S1f.5.5's T5 this pass refused one that was not,
/// *fact with non-atom head*, which the grammar refuses since the swap.
fn ingest_fact(ast: &Ast, terms: &mut Terms, kb: &mut Kb, form: NodeId) -> Result<(), Overflow> {
    let head_name = ast
        .head_name(form)
        .expect("a fact's relation is a name (ein.y's FactHead)")
        .to_string();
    let args = ast.form_args(form).to_vec();
    let pairs = kw_pairs(ast, &args);
    let source = string_value(ast, kw_get(&pairs, "source")).map(str::to_string);
    // Present-but-not-an-atom `:rule` reads as absent, and the record falls
    // back to source-kind: a non-atom has no atom name to record.
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

/// `SolverConfig.from_kw_pairs` — the two flags a program can set, each
/// `true` or `false` (`ein_core::config::FLAGS`). A repeated flag takes its
/// last value.
///
/// **Infallible since M1f S1f.5.5's T5.** A `(config …)` takes those two
/// keywords and a `true` or a `false` after each (`ein.y`'s `ConfigClause`
/// and `Bool`), so the grammar refuses what this function refused until then:
/// a body that is not keyword pairs, an unknown flag, a flag retired at M1e2
/// (its *was retired at* sentence a note on the parse error since the swap,
/// read from `ein_core::config::RETIRED`), and a value that is no bool; and
/// the integer, number and string coercions had no flag to reach since M1e2
/// S1e2.1a.5 retired the last one.
pub fn config_from_kw_pairs(ast: &Ast, args: &[NodeId]) -> SolverConfig {
    let mut config = SolverConfig::default();
    for (flag, value) in kw_pairs(ast, args) {
        let on = match ast.atom_name(value) {
            Some("true") => true,
            Some("false") => false,
            _ => unreachable!("a flag takes true or false (ein.y's Bool)"),
        };
        match flag.as_str() {
            "lattice-sanity-check" => config.lattice_sanity_check = on,
            "record-alternative-justifications" => config.record_alternative_justifications = on,
            other => unreachable!(
                "(config …) takes {:?} (ein.y's ConfigClause), not :{other}",
                FLAGS.iter().map(|(f, _)| *f).collect::<Vec<_>>()
            ),
        }
    }
    config
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

    /// The parser's errors on `text`, which it must refuse, each `line:col:
    /// what`: since M1f S1f.5.5's swap (T1f.5.5.4) the grammar refuses many
    /// shapes the loader refused until then.
    fn refused(text: &str) -> Vec<String> {
        parse(&mut Ast::new(), text, None)
            .expect_err("the parser refuses it")
            .errors
            .iter()
            .map(|e| format!("{}:{}: {}", e.line, e.col, e.what))
            .collect()
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

    /// **A variable used once is what to write where any value will do**, and
    /// it loads, in a pattern and in an obligation's witness premise.
    ///
    /// Until M1f S1f.5.5's swap this test held a wildcard written as a
    /// relation refused at load (M1e2 S1e2.2.20, W20, Q-M1e2.14 (c)), one line
    /// per slot, and `_` as an argument loading; since the swap the grammar
    /// refuses a bare `_` in every position (S1f.5.1), and T1f.5.5.5 deleted
    /// the loader's line (`Found::WildHead`) and moved those cases to the
    /// grammar's table (`ein-ir/tests/grammar_pages.rs`).
    #[test]
    fn a_variable_used_once_loads_where_a_wildcard_was_written() {
        let pre = "(relation n T) (relation hit T) (relation box T) (relation tag T T)\n(n 2)\n";
        ok(&format!(
            "{pre}(rule r () :match (and (n ?u) (box (tag ?v ?w))) :assert (hit r))"
        ));
        ok(&format!(
            "{pre}(rule owe () :match (and (n ?x) (absent (tag ?x ?any))) :assert (open tag))"
        ));
    }

    /// **A rule shape that reads nothing is refused at load** — M1e2
    /// S1e2.2.24, W24, on the user's rulings of 2026-11-05 (Q-M1e2.16 (1),
    /// Q-M1e2.18). Each shape in each place it can take, in a rule, an hrule
    /// and an obligation, one line naming the rule and the clause; and what
    /// stays: a guard on values, a parameter head.
    ///
    /// **Since M1f S1f.5.5's swap (T1f.5.5.4) half of the shapes are the
    /// grammar's**, each a parse error at its place, and a row of the grammar's
    /// table since T1f.5.5.5 deleted the loader's lines for them
    /// (`ein-ir/tests/grammar_pages.rs`): `_` as a guard's argument or as a
    /// premise (S1f.5.1), a keyword pair as a guard's argument or inside a
    /// pattern (S1f.5.11), a name, a variable or a string as a premise (the
    /// paper grammar's own, and S1f.5.2 for the string). The loader keeps what
    /// a context-free grammar does not see: a form that holds a variable as a
    /// guard's argument (Q-M1f.30), a relation variable no parameter binds,
    /// and an activator binding a relation parameter to a nested fact. What
    /// this test held loading until the swap and the grammar refuses (a string
    /// as a guard's argument, Q32's `:where` pairs, `_` as an argument) is
    /// written in the final grammar below.
    ///
    /// **Since M1f S1f.5.7 (T1f.5.7.3) two shapes load** that S1e2.2.24
    /// refused, on the user's ruling of 2026-11-07 on Q-M1e2.26: a fact that
    /// holds no variable as a guard's argument, a value, and a guard on a
    /// variable no premise before it binds, which waits for the premise that
    /// does, or ranges over the program's names where none does.
    #[test]
    fn a_rule_shape_that_reads_nothing_is_refused_at_load() {
        const GUARD: &str = "a guard compares two values, each a variable, a name, or a fact \
                             that holds no variable";
        const FREE: &str = "which no parameter binds — a relation is named by a name or by a \
                            parameter its activator binds";
        let pre = "(relation p T) (relation q T) (relation hit T) (relation box T) (relation tag T T)\n\
                   (relation item T) (relation lbl T)\n(p a) (q a) (box (p a))\n";
        // At load: what a context-free grammar does not see.
        for (rule, want) in [
            (
                "(rule e () :match (and (p ?x) (eq ?x (p ?y))) :assert (hit ?x))".to_string(),
                format!(
                    "rule 'e': its :match writes (eq ?x (p ?y)), and the form (p ?y) holds a \
                     variable — {GUARD}"
                ),
            ),
            (
                // Under an `absent`, and at any depth.
                "(rule e () :match (and (p ?x) (absent (neq ?x (box (p ?x))))) :assert (hit ?x))"
                    .to_string(),
                format!(
                    "rule 'e': its :match writes (neq ?x (box (p ?x))), and the form (box (p ?x)) \
                     holds a variable — {GUARD}"
                ),
            ),
            // A relation variable no parameter binds, inside a pattern and at
            // the top level, in either clause.
            (
                "(rule r () :match (box (?r a)) :assert (hit a))".to_string(),
                format!("rule 'r': its :match writes (?r a) with ?r as its relation, {FREE}"),
            ),
            (
                "(rule r () :match (and (p ?x) (absent (not (?r ?x)))) :assert (hit ?x))"
                    .to_string(),
                format!("rule 'r': its :match writes (?r ?x) with ?r as its relation, {FREE}"),
            ),
            (
                "(rule r () :match (and (p ?r) (?r a)) :assert (hit a))".to_string(),
                format!("rule 'r': its :match writes (?r a) with ?r as its relation, {FREE}"),
            ),
            (
                "(rule r () :match (p ?r) :assert (box (?r a)))".to_string(),
                format!("rule 'r': its :assert writes (?r a) with ?r as its relation, {FREE}"),
            ),
            (
                "(hrule h () :match (p ?r) :assert (?r a))".to_string(),
                format!("hrule 'h': its :assert writes (?r a) with ?r as its relation, {FREE}"),
            ),
            // An obligation's `:match`.
            (
                "(rule owe () :match (and (item ?x) (box (?r a)) (absent (tag ?x ?l))) \
                 :assert (open tag))"
                    .to_string(),
                format!("rule 'owe': its :match writes (?r a) with ?r as its relation, {FREE}"),
            ),
        ] {
            assert_eq!(err(&format!("{pre}{rule}")), want, "{rule}");
        }
        // A relation parameter its activator binds to a nested fact, where the
        // activator is written: a fact, or a literal in an `:assert`.
        let mirror = "(rule mirror (?R) :match (box (?R a)) :assert (hit a))";
        for activator in [
            "(mirror (p a))".to_string(),
            "(rule mk () :match (q ?x) :assert (mirror (p a)))".to_string(),
        ] {
            assert_eq!(
                err(&format!("{pre}{mirror}\n{activator}")),
                "rule 'mirror': its parameter ?R names a relation, and (mirror (p a)) binds it \
                 to the fact (p a) — a relation is named by a symbol, not a fact",
                "{activator}"
            );
        }
        // What stays: a guard on values the premises before it bind, a
        // parameter head, a variable used once where any value will do, a
        // guard under an `absent` on a variable the enclosing premises bound.
        // Until the swap the first wrote `(neq ?x "b")`, a string, and three
        // more Q32's `:where` and `_` as an argument, which the grammar
        // refuses. And since S1f.5.7 what S1e2.2.24 refused: a fact that holds
        // no variable as a guard's argument, and a guard on a variable no
        // premise before it binds, in a conjunction and under an `absent`.
        for rule in [
            "(rule e () :match (and (p ?x) (q ?y) (neq ?x ?y) (eq ?x a) (neq ?x b) (eq ?x 1)) \
             :assert (hit ?x))",
            "(rule e () :match (and (p ?x) (absent (and (q ?y) (neq ?y ?x)))) :assert (hit ?x))",
            "(rule e () :match (and (p ?x) (eq ?x a)) :assert (hit ?x))",
            "(rule e () :match (and (box (p ?u)) (p ?v)) :assert (hit a))",
            "(rule e (?R) :match (box (?R a)) :assert (hit a))",
            "(rule e () :match (and (p ?x) (eq ?x (p a))) :assert (hit ?x))",
            "(rule e () :match (and (neq ?x a) (p ?x)) :assert (hit ?x))",
            "(rule e () :match (and (p ?x) (eq ?x ?u)) :assert (hit ?u))",
            "(rule e () :match (and (p ?x) (absent (and (neq ?y a) (q ?y)))) :assert (hit ?x))",
        ] {
            ok(&format!("{pre}{rule}"));
        }
    }

    /// **The verdict atom refuses every shape it cannot resolve** — M1d
    /// S1d.2.3, `defined_behaviour.md` § 4.2's table. **Since M1f S1f.5.5's
    /// swap (T1f.5.5.4) the shapes a position states are the grammar's**: an
    /// `(open …)` as a premise or beside another conclusion, with three
    /// arguments, or with a term for its argument, and `open` bound by a
    /// declarator, are parse errors at their place (`open` is a word a position
    /// gives a meaning to, the paper grammar's default 1), each a row of the
    /// grammar's table since T1f.5.5.5 (`ein-ir/tests/grammar_pages.rs`). The
    /// loader's lines for them stay, a pattern's argument and a macro's body
    /// still reaching each (that file's
    /// `the_lines_the_grammar_does_not_cover_are_still_reached`); below are
    /// the refusals a context-free grammar does not see: an `(open …)` under a
    /// `not`, and the projection's ways of not resolving.
    #[test]
    fn the_verdict_atom_refuses_every_shape_it_cannot_resolve() {
        let pre = "(relation is-a T T)\n(relation r A B)\n";
        let guard = "(and (relation ?R ?A ?B) (?isa ?a ?A) \
                     (absent (and (?isa ?b ?B) (?R ?a ?b))))";
        for (want, src) in [
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
                    "{pre}(rule bad (?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A)) \
                     :assert (open ?R))"
                ),
            ),
            // Written as a premise's relation too, the rule pass refuses it
            // first, as it refuses any relation variable no parameter binds
            // (M1e2 S1e2.2.24, W24); until then this case reached the line above.
            (
                "with ?R as its relation, which no parameter binds",
                format!(
                    "{pre}(rule bad (?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) \
                     (absent (?R ?a ?b))) :assert (open ?R))"
                ),
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

    /// A nested form is a fact, interned and not believed. **Since M1f
    /// S1f.5.5's swap (T1f.5.5.4)** a nested fact's head is a name: `(p (?q
    /// a))`, a variable in a fact, is refused at the parse (S1f.5.2's G-6), so
    /// no fact reaches the loader with a nested head that is not a name, and
    /// the `#nested` rename it collapsed to (printed `(_ a)`) is reached by no
    /// program; until then the second fact here was `(p (?q a))`.
    #[test]
    fn a_nested_form_becomes_a_relational_node() {
        assert_eq!(
            refused("(not (co-located a b))\n(p (?q a))"),
            ["2:5: unexpected `?q`: a variable"]
        );
        let (terms, kb) = ok("(not (co-located a b))\n(p (q a))");
        let nested: Vec<Vec<Tag>> = kb
            .facts()
            .map(|f| terms.facts.args(f).iter().map(|a| a.tag()).collect())
            .collect();
        assert_eq!(nested, vec![vec![Tag::Fact], vec![Tag::Fact]]);
        let compacts = facts(&terms, &kb);
        assert_eq!(compacts[1], "(p (q a))", "{compacts:?}");
        // The inner proposition is interned but not believed: it is not in
        // the fact list, and that is what makes the negated index meaningful.
        assert_eq!(kb.n_facts(), 2);
        let inner = terms.facts.args(kb.facts().next().expect("a fact"))[0]
            .as_fact()
            .expect("nested");
        assert!(!kb.contains(inner));
        assert!(kb.is_negated(inner));
    }

    /// Since M1f S1f.5.5's swap (T1f.5.5.4) a fact's `:using` lists its
    /// premises, each a fact (the paper grammar's default 2): `:using ((r a
    /// b))`. Until then this test wrote `:using (p (r a b))`, a list whose head
    /// the loader dropped, which the grammar refuses at the `p`.
    #[test]
    fn provenance_comes_from_source_or_from_rule_and_using() {
        assert_eq!(
            refused("(relation r T T)\n(r e f :rule x :using (p (r a b)))"),
            ["2:24: unexpected `p`: a name"]
        );
        let (terms, kb) = ok("(relation r T T)\n(r a b :source \"(1)\")\n(r c d)\n\
             (r e f :rule x :using ((r a b)))");
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

    /// A `:source` must be a string and a `:rule` a name. Until M1f S1f.5.5's
    /// swap anything else was no error: it simply did not register, and
    /// `(r a b :source sentence :rule "x")` loaded as a given fact with no
    /// source. **Since the swap each keyword's value is the grammar's**, so
    /// the bare `sentence` is refused at the parse, and so is the string
    /// `"x"` (S1f.5.2's ruling). What still reads as absent is a `:source`
    /// with no `:rule`: a given fact, its sentence kept.
    #[test]
    fn a_present_but_unusable_annotation_reads_as_absent() {
        assert_eq!(
            refused("(relation r T T)\n(r a b :source sentence :rule \"x\")")[0],
            "2:16: unexpected `sentence`: a name"
        );
        let (terms, kb) = ok("(relation r T T)\n(r a b :source \"sentence\")");
        let fact = kb.facts().last().expect("a fact");
        let prov = terms.provs.get(kb.primary(fact).expect("recorded"));
        assert_eq!(prov.kind, ProvKind::Source);
        assert_eq!(
            prov.source.map(|s| terms.sym(s).to_string()).as_deref(),
            Some("sentence")
        );
    }

    /// Was `the_last_query_and_the_last_config_win` until M1c S1c.1.2. Half of
    /// what it pinned is deliberately gone: a second `(query …)` used to load
    /// and be discarded in silence, which is the one thing a file carrying
    /// `:expect` must not do. The config half is untouched, and is here in the
    /// same test because the *contrast* is the decision — a config is a
    /// setting, a query is content.
    ///
    /// Since M1f S1f.5.5's swap (T1f.5.5.4) the second query's `:mode all`
    /// is `:over all` and the first config's retired `:print-alive` is
    /// `:record-alternative-justifications false`: the grammar refuses a
    /// keyword its form does not take (S1f.5.11), and both were, a row of the
    /// grammar's table since T1f.5.5.5 (`ein-ir/tests/grammar_pages.rs`).
    #[test]
    fn every_query_is_kept_and_the_last_config_still_wins() {
        let (_terms, kb) = ok("(query :goal (a ?x))\n(query :goal (b ?x) :over all)\n\
             (config :record-alternative-justifications false)\n(config :lattice-sanity-check true)");
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
        assert!(config.lattice_sanity_check);
        assert!(
            config.record_alternative_justifications,
            "the earlier block is discarded whole"
        );
    }

    // ── `:expect`, at load — M1c S1c.1.2 T1c.1.2.5 ────────────────
    //
    // Every one of these is a load *error* rather than a run that checks
    // nothing, which is the whole reason the keyword exists. Until M1f
    // S1f.5.5's T5 so were an unknown query keyword, the obsolete `:mode` and a
    // malformed `:expect`, the loader's lines until the swap made each a parse
    // error; they are rows of the grammar's table since
    // (`ein-ir/tests/grammar_pages.rs`), and what stays here is what a grammar
    // cannot see.

    #[test]
    fn an_expect_naming_a_relation_the_program_does_not_have_is_a_load_error() {
        let e = err("(relation p T P)\n(p A H)\n\
             (query :goal (p A ?h) :expect (model (p A H) (nosuch A)))");
        assert!(e.contains("nosuch"), "{e}");
        assert!(e.contains("no declaration or fact makes a relation"), "{e}");
    }

    /// Since M1e2 S1e2.1a.5 a query's `:expect` lists projection classes —
    /// the goal's atoms under its rows — so a listed fact the goal does not
    /// match is refused, and the message names where a claim about the state
    /// belongs.
    #[test]
    fn a_query_claim_lists_only_facts_its_goal_matches() {
        let e = err("(relation p T P)\n(relation q T P)\n(p A H)\n(q A H)\n\
             (query :goal (p A ?h) :expect (model (q A H)))");
        assert!(
            e.contains("(q A H), which no atom of the query's :goal matches"),
            "{e}"
        );
        assert!(e.contains("(solve … :expect (region …))"), "{e}");
        // …in every disjunct, so one good class beside one bad one is still
        // refused.
        let e = err("(relation p T P)\n(relation q T P)\n(p A H)\n(q A H)\n\
             (query :goal (p A ?h) \
              :expect (or (model (p A H)) (model (q A H))))");
        assert!(e.contains("(q A H), which no atom"), "{e}");
        // A goal's constant must match, and a variable bind one value.
        let e = err("(relation p T P)\n(p A H)\n(p B H)\n\
             (query :goal (p A ?h) :expect (model (p B H)))");
        assert!(e.contains("(p B H), which no atom"), "{e}");
        let (_terms, _kb) = ok("(relation p T P)\n(p A H)\n\
             (query :goal (and (p ?x ?h) (p ?x H)) :expect (model (p A H)))");
    }

    /// A class is listed once, and `:over` is one of three words.
    #[test]
    fn a_class_twice_and_an_unknown_over_are_load_errors() {
        let e = err("(relation p T P)\n(p A H)\n\
             (query :goal (p A ?h) :expect (or (model (p A H)) (model (p A H))))");
        assert!(e.contains("lists the class (model (p A H)) twice"), "{e}");
        let e = err("(relation p T P)\n(p A H)\n(query :goal (p A ?h) :over best)");
        assert!(
            e.contains(":over best — one of all, minimal, maximal"),
            "{e}"
        );
        for over in ["all", "minimal", "maximal"] {
            ok(&format!(
                "(relation p T P)\n(p A H)\n(query :goal (p A ?h) :over {over})"
            ));
        }
    }

    /// A query with no `:expect` is untouched by any of the above — the
    /// corpus is 128 entries of exactly that.
    #[test]
    fn a_query_without_expect_is_unaffected() {
        let (_terms, kb) = ok("(relation p T P)\n(p A H)\n(query :goal (p A ?h))");
        assert_eq!(kb.program().queries.len(), 1);
    }

    /// The second query writes `:over all` since M1f S1f.5.5's swap, where it
    /// wrote `:mode all`, which the grammar refuses (S1f.5.11).
    #[test]
    fn a_later_query_is_reachable_by_index() {
        let src = "(query :goal (a ?x))\n(query :goal (b ?x) :over all)";
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

    /// Every pass accumulates rather than stopping at the first problem.
    /// Since M1f S1f.5.5's swap (T1f.5.5.4) two of the three problems this
    /// test wrote are the grammar's, `(relation)` with no name and a rule
    /// named `absent`, so the parser reports both and the loader never sees
    /// the file (a row of the grammar's table since T1f.5.5.5); the loader's
    /// passes are held with problems it still owns: a macro declared twice
    /// (pass 0), a relation declared twice (pass 1) and a rule named after a
    /// kernel dunder (pass 2, T1f.5.5.11).
    #[test]
    fn every_pass_accumulates_rather_than_stopping_at_the_first_problem() {
        let message = err(
            "(macro m (?a) (x ?a))\n(macro m (?a) (y ?a))\n(relation dup)\n\
             (relation dup)\n(rule __closed__ () :match (x ?a) :assert (y ?a))",
        );
        assert_eq!(
            message,
            "duplicate macro 'm' at None; duplicate relation 'dup' at None; \
             rule '__closed__' shadows a reserved kernel name at None"
        );
    }
}
