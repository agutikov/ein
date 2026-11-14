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
/// `--no-require-why` turns it off on every command that loads a program.
///
/// **Required by default since S1f.5.14's T1f.5.14.7** (2026-11-10), the
/// user's *"then after other stages finished swith to default true and fix
/// remaining ein code"*: every program the corpus, the stdlib and the Rust
/// suites load carries its `:why`s, so [`load`], [`load_query`] and
/// [`load_file`] refuse one that does not; `require_why: false` loads it, as
/// `--no-require-why` does (Q-M1f.62).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoadOptions {
    /// Refuse every `(relation …)`, `(rule …)`, `(hrule …)` and `(macro …)`
    /// the load reads, the stdlib's and an imported module's included, that
    /// carries no `:why "…"`, one line each ([`WHY_REQUIRED_OF`]). A macro
    /// since S1f.5.5's swap (T1f.5.5.4), when the one parser became the one
    /// that reads a macro's `:why` and the stdlib's two macros got theirs.
    /// **Default true** since S1f.5.14's T1f.5.14.7; false until then.
    pub require_why: bool,
}

impl Default for LoadOptions {
    fn default() -> Self {
        LoadOptions { require_why: true }
    }
}

/// The forms [`LoadOptions::require_why`] asks for a `:why`, as its refusal
/// names them.
pub const WHY_REQUIRED_OF: &str = "relation, rule, hrule and macro";

/// The most distinct variables a rule's `:match`, its parameters among them,
/// or a query's `:goal` may write — M1f S1f.5.21 (W63, Q-M1f.73).
///
/// The compiler numbers a plan's variables into a fixed register file of this
/// many slots, so the matcher's inner loop allocates nothing
/// (`ein_infer::MAX_REGS`, which is this constant): one file for the plan, its
/// parameters and the variables its positive premises and guards write, and
/// one per lifted `(absent …)`, the variables its body writes. The loader
/// counts every distinct variable the clause writes, anywhere in it, with the
/// parameters, which bounds each file, so the compiler meets no clause past it
/// (`refuse_wide_matches`). Until S1f.5.21 the compiler refused one where it
/// compiled the plan, so a rule no state activated, and a goal no state was
/// answered from, loaded and answered.
pub const MAX_MATCH_VARIABLES: usize = 256;

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
    for &form in &relations {
        ingest_relation(ast, terms, &mut kb, form, opts, &mut errors)?;
    }
    // What each declaration declares of its relation's count of arguments
    // (M1f S1f.6.23, W89): read by the rules pass, the facts' and the
    // queries'.
    let declared = declared_counts(ast, &relations);
    // Pass 2 — rules. After this, rule-name resolution is possible.
    ingest_rules(
        ast,
        terms,
        &mut kb,
        &rules,
        Declarations {
            macros: &declared_macros,
            counts: &declared,
        },
        opts,
        &mut errors,
    )?;
    // Pass 3 — facts.
    for &form in &facts {
        ingest_fact(ast, terms, &mut kb, form)?;
    }
    // A fact, or a fact it holds, of a declared relation at another count of
    // arguments than declared (M1f S1f.6.23, W89).
    refuse_wrong_counts_in_facts(ast, &facts, &declared, &mut errors);
    // A written fact of `eq` at another arity (M1f S1f.5.9, W61).
    refuse_eq_facts_of_another_arity(ast, &facts, &mut errors);
    // A macro's call written as a fact, which nothing expands, and a relation
    // declared under a macro's name (M1f S1f.7.12, W86).
    let stdlib_macros = Resolver::new().stdlib_macro_names();
    refuse_macro_facts(
        ast,
        &relations,
        &facts,
        &declared_macros,
        &stdlib_macros,
        &mut errors,
    );
    // A rule parameter that names a relation, bound to a nested fact by an
    // activator the program writes down (M1e2 S1e2.2.4, S1e2.2.24).
    validate_relation_parameters(ast, terms, &kb, &facts, &mut errors);
    // …and one a rule could derive, read off the program (M1f S1f.5.18,
    // Q-M1f.1 (b)).
    let relation_places =
        refuse_derived_relation_activators(ast, terms, &kb, &facts, &solve_blocks, &mut errors);
    // A conclusion `(not ?x)` whose variable the program shows can hold a
    // name (M1f S1f.7.4, W84).
    refuse_negated_names(ast, terms, &kb, &facts, &solve_blocks, &mut errors);

    // Every query block, in source order — plural since M1c S1c.1.2, because a
    // second one used to load and be discarded in silence. `config` keeps
    // last-wins: a config is a setting, a query is content.
    //
    // A goal is a premise, so a macro called in one is expanded as a rule's
    // `:match` is (M1f S1f.7.5, W77): every reader of the query's `:goal`
    // meets the expansion, and `Query::goal_written` keeps the goal as the
    // program wrote it, for the surfaces that print it.
    let mut queries: Vec<Query> = Vec::with_capacity(query_blocks.len());
    for &form in &query_blocks {
        queries.push(expand_goal(ast, form, &declared_macros, &mut errors));
    }
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
    // The relations of the program, as it writes them outside its goals and
    // its `(solve …)` lists (M1f S1f.7.8's notion, Q-M1f.114): what a claim
    // may list and a goal may ask about (S1f.7.11). A program already refused
    // is not read for them, since a rule the loader refused may be missing
    // from the registries they are read off.
    let relations_of = errors
        .is_empty()
        .then(|| program_relations(ast, terms, &kb, &facts, &relation_places));
    // What a state can hold a fact of: those, and the relations a
    // `(solve :hrules …)` activator has its hrule propose.
    let holdable = relations_of.as_ref().map(|known| {
        let mut all = known.clone();
        all.extend(activator_relations(ast, &kb, &relation_places));
        all
    });
    validate_solve(ast, &kb, holdable.as_ref(), &mut errors);
    // A `(solve …)` list that names an hrule or a relation the program does
    // not have (M1f S1f.7.8, W76).
    if let Some(known) = relations_of.as_ref() {
        refuse_solve_names(ast, terms, &kb, known, &relation_places, &mut errors);
    }
    let unimported: Vec<String> = stdlib_macros
        .iter()
        .filter(|n| !declared_macros.contains_key(*n))
        .cloned()
        .collect();
    validate_queries(ast, &kb, holdable.as_ref(), &unimported, &mut errors);
    // A goal's pattern and a claim's fact are held to a declaration's count
    // as a rule's and a written fact are (M1f S1f.6.23, W89).
    refuse_wrong_counts_in_queries(ast, &kb, &declared, &mut errors);
    // …and one a claim lists, which is a fact too (S1f.7.12). A claim headed
    // by the call names no relation of the program and has its own line
    // (Q-M1f.118), so this one is for a program nothing else refuses.
    if errors.is_empty() {
        refuse_macro_claims(ast, &kb, &declared_macros, &stdlib_macros, &mut errors);
    }
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
                rule_matches.push((format!("rule '{}'", terms.sym(name)), NodeId(p.expr.0)));
            }
        }
    }
    // …and a goal, which is a premise too (M1f S1f.7.5, W77): unexpanded, the
    // call matched nothing and the goal had no row in any state.
    for query in &kb.program().queries {
        if let Some(goal) = query_keyword(ast, query, "goal") {
            rule_matches.push(("query".to_string(), goal));
        }
    }
    errors.extend(macros::unimported_macro_errors(
        ast,
        &rule_matches,
        &declared_macros,
        &stdlib_macros,
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
fn validate_solve(
    ast: &Ast,
    kb: &Kb,
    relations_of: Option<&BTreeSet<String>>,
    errors: &mut Vec<String>,
) {
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
            // A relation of the program (M1f S1f.7.11, W78): until then one
            // a declaration or a fact makes, which refused a claim listing a
            // fact only a rule's `:assert` derives.
            let known = relations_of.is_none_or(|k| k.contains(fact.relation));
            if !known && reported.insert(fact.relation.to_string()) {
                errors.push(format!(
                    "(solve … :expect …) names {0}, and the program names no relation {0} — \
                     {CLAIM_RELATION}",
                    fact.relation
                ));
            }
        }
    }
}

/// Where a program holds relation names, as
/// [`refuse_derived_relation_activators`] reads them off it.
#[derive(Default)]
struct RelationPlaces {
    /// `(name, index)`: a rule's or an hrule's parameter, or a marker's
    /// argument, that holds a relation's name.
    positions: BTreeSet<(String, usize)>,
    /// Every name a fact or a conclusion writes at one.
    passed: BTreeSet<String>,
}

/// **A `(solve …)` list names what the program has** — M1f S1f.7.8 (W76).
///
/// The three lists choose the candidates, so a name in one that matches
/// nothing moved H, and R with it, in silence: `:hrules (gues …)` beside an
/// hrule `guess` proposed nothing, an activator `(att Color House)` proposed
/// facts of a relation `att` nothing else names, and `:no-hypothesis (like)`
/// restricted nothing. Each is refused here, one line per name, placed at
/// the name:
///
/// - **`:hrules`**: the list's first name, and every bare name after it, is
///   an hrule the program declares; an activator gives its hrule as many
///   arguments as the hrule has parameters; and an activator's argument at a
///   parameter that names a relation is a relation of the program.
/// - **`:hypothesis-relations`**, **`:no-hypothesis`**: every name is a
///   relation of the program.
///
/// **A relation of the program** is a name the program writes as a relation
/// outside the `(solve …)` lists: one it declares or states a fact of,
/// `is-a`, the relation of any form in a rule's, an hrule's or an
/// obligation's `:match` or `:assert`, of a query's `:goal` or of a fact's
/// argument, and a name a fact or a conclusion passes where a relation's name
/// is held, `(bijective color-loc)`. So a relation only a rule's `:assert`
/// derives is one, where the claim check ([`validate_solve`]) refuses it. It
/// is not *a relation something makes*: an activator's hrule makes the
/// relation it is given, `att` as well as `at`, so what tells a misspelt name
/// is that the program writes it nowhere else (Q-M1f.114).
///
/// An activator's argument at a parameter that names no relation is a value,
/// a type or an object, and is not checked: `(guess (at Colour House))`
/// loads, and proposes nothing (F25's R-2).
/// The relation of every form at or under `node`, where it is a name.
fn relation_heads(ast: &Ast, node: NodeId, out: &mut BTreeSet<String>) {
    match ast.node(node) {
        Node::SForm { head, .. } => {
            if let Node::Atom(s) = ast.node(head) {
                out.insert(ast.sym(s).to_string());
            }
            for &a in ast.form_args(node) {
                relation_heads(ast, a, out);
            }
        }
        Node::KwPair { value, .. } => relation_heads(ast, value, out),
        _ => {}
    }
}

/// What a claim's refusal, and a goal's, says a relation of the program is.
const CLAIM_RELATION: &str = "a relation of the program is one it declares, states a fact of, or \
                              writes in a rule or an hrule";

/// **The relations of the program**, as it writes them outside its goals and
/// its `(solve …)` lists — M1f S1f.7.8's notion (Q-M1f.114), which S1f.7.11
/// holds a goal and a claim to: a relation it declares or states a fact of,
/// `is-a`, `eq` and `neq`, the relation of any form at any depth in a rule's,
/// an hrule's or an obligation's `:match` or `:assert`, as expanded, or in a
/// fact, a name a fact or a conclusion passes where a relation's name is held
/// (`places.passed`). So a relation only a rule's `:assert` derives is one.
///
/// A goal's own relations are not among them: a goal reads, and a relation
/// only a goal names is one no state holds a fact of. [`refuse_solve_names`]
/// adds them for its lists (Q-M1f.114 (b)).
fn program_relations(
    ast: &Ast,
    terms: &Terms,
    kb: &Kb,
    facts: &[NodeId],
    places: &RelationPlaces,
) -> BTreeSet<String> {
    let program = kb.program();
    let mut known: BTreeSet<String> = places.passed.clone();
    // `is-a` and the two relations of equality are every program's.
    known.extend(["is-a", "eq", "neq"].map(str::to_string));
    for (name, _) in program.relations.iter() {
        known.insert(terms.sym(name).to_string());
    }
    for registry in [&program.rules, &program.hrules, &program.obligations] {
        for (_, rule) in registry.iter() {
            for pat in [rule.match_.as_ref(), rule.assert_.as_ref()]
                .into_iter()
                .flatten()
            {
                relation_heads(ast, NodeId(pat.expr.0), &mut known);
            }
        }
    }
    for &f in facts {
        relation_heads(ast, f, &mut known);
    }
    known
}

/// The relations a `(solve :hrules …)` activator gives its hrule at a
/// parameter that names a relation: that hrule proposes facts of each, so a
/// state can hold one, and a goal may ask about it and a claim list it
/// (M1f S1f.7.11). [`refuse_solve_names`] holds each to the program's other
/// relations, a goal's among them.
fn activator_relations(ast: &Ast, kb: &Kb, places: &RelationPlaces) -> BTreeSet<String> {
    let mut known: BTreeSet<String> = BTreeSet::new();
    if let Some(solve) = kb.program().solve()
        && let Some(value) = solve_keyword(ast, &solve.kw_pairs, "hrules")
        && let Node::SForm { head, .. } = ast.node(value)
    {
        let mut hrule: Option<&str> = None;
        for item in std::iter::once(head).chain(ast.form_args(value).iter().copied()) {
            match ast.node(item) {
                Node::Atom(s) if hrule.is_none() => hrule = Some(ast.sym(s)),
                Node::SForm { head: h, .. } => {
                    let Some(name) = hrule else { continue };
                    for (i, &a) in std::iter::once(&h).chain(ast.form_args(item)).enumerate() {
                        if let Some(arg) = ast.atom_name(a)
                            && places.positions.contains(&(name.to_string(), i))
                        {
                            known.insert(arg.to_string());
                        }
                    }
                }
                _ => {}
            }
        }
    }
    known
}

fn refuse_solve_names(
    ast: &Ast,
    terms: &Terms,
    kb: &Kb,
    relations_of: &BTreeSet<String>,
    places: &RelationPlaces,
    errors: &mut Vec<String>,
) {
    /// A list's names with their nodes: a bare name, or a form's head and
    /// its atom arguments. `()` holds none.
    fn names(ast: &Ast, value: NodeId) -> Vec<(NodeId, String)> {
        let nodes: Vec<NodeId> = match ast.node(value) {
            Node::Atom(_) => vec![value],
            Node::SForm { head, .. } => std::iter::once(head)
                .chain(ast.form_args(value).iter().copied())
                .collect(),
            _ => Vec::new(),
        };
        nodes
            .into_iter()
            .filter_map(|n| ast.atom_name(n).map(|s| (n, s.to_string())))
            .filter(|(_, s)| !ein_core::text::INTERNAL_NAMES.contains(&s.as_str()))
            .collect()
    }

    let Some(solve) = kb.program().solve() else {
        return;
    };
    // A program already refused is not read further: a rule the loader
    // refused may be missing from the registries this reads.
    if !errors.is_empty() {
        return;
    }
    let program = kb.program();
    // The program's relations, and here every relation a goal reads too.
    let mut known: BTreeSet<String> = relations_of.clone();
    for query in &program.queries {
        if let Some(goal) = query_keyword(ast, query, "goal") {
            relation_heads(ast, goal, &mut known);
        }
    }
    let hrules: Vec<String> = program
        .hrules
        .iter()
        .map(|(n, _)| terms.sym(n).to_string())
        .collect();
    let declared_hrules = if hrules.is_empty() {
        "it declares no hrule".to_string()
    } else {
        format!("it declares {}", hrules.join(", "))
    };
    const RELATION: &str = "a relation of the program is one it declares, states a fact of, or \
                            writes in a rule, an hrule or a goal";

    for &pair in solve.kw_pairs.iter() {
        let Node::KwPair { key, value } = ast.node(NodeId(pair.0)) else {
            continue;
        };
        let Node::Keyword(key) = ast.node(key) else {
            continue;
        };
        match ast.sym(key) {
            list @ ("hypothesis-relations" | "no-hypothesis") => {
                for (node, name) in names(ast, value) {
                    if !known.contains(&name) {
                        errors.push(format!(
                            "{}solve: its :{list} names {name}, and the program names no \
                             relation {name} — {RELATION}",
                            placed_or(ast, node, value),
                        ));
                    }
                }
            }
            "hrules" => {
                let Node::SForm { head, .. } = ast.node(value) else {
                    continue;
                };
                let mut hrule: Option<String> = None;
                for item in std::iter::once(head).chain(ast.form_args(value).iter().copied()) {
                    match ast.node(item) {
                        // `()` is the list that names none.
                        Node::Atom(s) if ein_core::text::INTERNAL_NAMES.contains(&ast.sym(s)) => {}
                        Node::Atom(s) => {
                            let name = ast.sym(s).to_string();
                            if hrule.is_none() {
                                hrule = Some(name.clone());
                            }
                            if !hrules.contains(&name) {
                                errors.push(format!(
                                    "{}solve: its :hrules names {name}, and the program \
                                     declares no hrule {name} — {declared_hrules}",
                                    placed_or(ast, item, value),
                                ));
                            }
                        }
                        Node::SForm { .. } => {
                            let Some(h) = hrule.as_ref().filter(|h| hrules.contains(h)) else {
                                continue;
                            };
                            let rule = terms
                                .syms
                                .get(h)
                                .and_then(|sym| program.hrules.get(sym))
                                .expect("a declared hrule");
                            let args = names(ast, item);
                            let written = crate::dump::dump_compact(ast, item);
                            if args.len() != rule.params.len() {
                                let params: Vec<String> = rule
                                    .params
                                    .iter()
                                    .map(|&p| format!("?{}", terms.sym(p)))
                                    .collect();
                                errors.push(format!(
                                    "{}solve: its :hrules gives hrule '{h}' {written}, {}, and \
                                     {h} takes {}, ({}) — an activator binds each parameter once",
                                    placed_or(ast, item, value),
                                    count(args.len(), "argument"),
                                    rule.params.len(),
                                    params.join(" "),
                                ));
                                continue;
                            }
                            for (i, (node, name)) in args.iter().enumerate() {
                                if places.positions.contains(&(h.clone(), i))
                                    && !known.contains(name)
                                {
                                    errors.push(format!(
                                        "{}solve: its :hrules gives hrule '{h}' {written}, \
                                         whose {name} is its parameter ?{}, which names a \
                                         relation, and the program names no relation {name} — \
                                         {RELATION}",
                                        placed_or(ast, *node, item),
                                        terms.sym(rule.params[i]),
                                    ));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

/// `n argument`, `n arguments`.
fn count(n: usize, what: &str) -> String {
    if n == 1 {
        format!("1 {what}")
    } else {
        format!("{n} {what}s")
    }
}

/// [`placed`] at `node`, or at `outer` where the parser kept no position for
/// it.
fn placed_or(ast: &Ast, node: NodeId, outer: NodeId) -> String {
    let at = placed(ast, node);
    if at.is_empty() {
        placed(ast, outer)
    } else {
        at
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
fn validate_queries(
    ast: &Ast,
    kb: &Kb,
    relations_of: Option<&BTreeSet<String>>,
    unimported: &[String],
    errors: &mut Vec<String>,
) {
    for query in &kb.program().queries {
        if let Some(text) = string_value(ast, query_keyword(ast, query, "goal-text")) {
            let mut vars: Vec<String> = Vec::new();
            if let Some(goal) = query_keyword(ast, query, "goal") {
                walk_pattern(ast, goal, &mut vars, &mut Vec::new());
            }
            refuse_unreachable_references("query", ":goal-text", text, &vars, errors);
        }
        if let Some(goal) = query_keyword(ast, query, "goal") {
            // A goal is a premise, held to what a rule's `:match` is held to
            // (M1f S1f.7.5, S1f.7.1): the one walk a rule's clauses get reads
            // it, a guard over a form holding a variable (Q-M1f.30), a
            // premise that is no form, which a macro's expansion can leave,
            // and a relation written as a variable, at the top level (M1f
            // S1f.5.21, W64) and inside a pattern (S1f.7.1, W79).
            let n_refused = errors.len();
            refuse_no_fact_holds(ast, "query", &[], ":goal", goal, errors);
            // A goal is compiled as a rule's `:match` is, and refused where the
            // compiler would refuse it (M1f S1f.5.21: W62, W63).
            refuse_nested_disjunctions(ast, "query", ":goal", goal, errors);
            refuse_wide_matches(ast, "query", ":goal", &[], goal, errors);
            // A kernel primitive in a shape only a macro's expansion writes
            // (S1f.5.24's walk, S1f.7.5).
            if errors.len() == n_refused {
                refuse_malformed_primitives(ast, "query", ":goal", goal, errors);
            }
            // A relation the goal asks about is a relation of the program
            // (S1f.7.11, W78).
            if errors.len() == n_refused
                && let Some(known) = relations_of
            {
                refuse_goal_relations(ast, goal, known, unimported, errors);
            }
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
                // fact makes them one, and the check below is the goal's. So
                // is an `(absent …)` the goal writes, whose instance a class
                // lists beside the rest (S1f.5.7's T1f.5.7.6, W56, Q-M1f.35).
                //
                // Since M1f S1f.7.11 (W78) the rest is a relation of the
                // program, [`program_relations`]: until then one a declaration
                // or a fact makes, which refused a claim listing a fact only a
                // rule's `:assert` derives.
                let known = is_predicate(fact.relation)
                    || fact.relation == "absent"
                    || relations_of.is_none_or(|k| k.contains(fact.relation));
                if !known {
                    errors.push(format!(
                        ":expect names {0}, and the program names no relation {0} — \
                         {CLAIM_RELATION}",
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

/// What the rules pass reads of the passes before it: the macros it expands
/// a rule's clauses with, and the count each `(relation …)` declares, which
/// the expanded clauses are held to (M1f S1f.6.23).
#[derive(Clone, Copy)]
struct Declarations<'a> {
    macros: &'a BTreeMap<String, IrMacro>,
    counts: &'a Declared,
}

fn ingest_rules(
    ast: &mut Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    forms: &[NodeId],
    declarations: Declarations,
    opts: &LoadOptions,
    errors: &mut Vec<String>,
) -> Result<(), Overflow> {
    let Declarations {
        macros: declared_macros,
        counts: declared,
    } = declarations;
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
        let who = format!("{head} '{name}'");
        refuse_no_fact_holds(ast, &who, &param_vars, ":match", match_node, errors);
        refuse_no_fact_holds(ast, &who, &param_vars, ":assert", assert_node, errors);
        // An `(or …)` written as a premise, and a `:match` past the register
        // file: the compiler's two refusals a program reached, read here since
        // M1f S1f.5.21 (W62, W63), after expansion, so a macro's `(or …)` too
        // (Q-M1f.74).
        refuse_nested_disjunctions(ast, &who, ":match", match_node, errors);
        refuse_wide_matches(ast, &who, ":match", &param_vars, match_node, errors);
        // A pattern or a conclusion of a declared relation at another count
        // of arguments than declared (M1f S1f.6.23, W89): read here, after
        // expansion, so a macro's body is held where a rule calls it.
        refuse_wrong_counts(ast, &who, ":match", "writes", match_node, declared, errors);
        refuse_wrong_counts(
            ast,
            &who,
            ":assert",
            "writes",
            assert_node,
            declared,
            errors,
        );
        if errors.len() != n_refused {
            continue;
        }
        // A kernel primitive the grammar shapes, which a macro's expansion
        // wrote in another shape (M1f S1f.5.24, W66 to W69, Q-M1f.80): read
        // here, after expansion, so no command compiles, fires or proposes it.
        refuse_malformed_primitives(ast, &who, ":match", match_node, errors);
        refuse_malformed_primitives(ast, &who, ":assert", assert_node, errors);
        if errors.len() != n_refused {
            continue;
        }
        // A conclusion's variable nothing binds (M1f S1f.5.10, F13).
        refuse_unbound_conclusions(
            ast,
            &head,
            &name,
            &param_vars,
            match_node,
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
        // The reserved verdict atom: the grammar writes it only as a rule's
        // whole `:assert`, at arity 0 or 1 (M1f S1f.5.23), and it is refused
        // here where its projection does not resolve. A rule that asserts it is
        // an **obligation** and is routed out of the saturation agenda below.
        let n_errors = errors.len();
        let is_obligation = validate_open(
            ast,
            &head,
            &name,
            &param_names,
            match_node,
            assert_node,
            errors,
        );
        if errors.len() != n_errors {
            continue;
        }
        let params: Vec<Symbol> = param_names
            .iter()
            .map(|p| terms.intern_text(p))
            .collect::<Result<_, _>>()?;
        // S1.8.A13 — a top-level `(or …)` `:match` and an `(and …)` `:assert`
        // stay as ordinary AST on ONE rule, which keeps its source name; the
        // compiler lowers them to several plans and templates, an `(and …)` at
        // any depth since M1f S1f.6.37 (`Ast::conjuncts`).
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
///
/// **A query's `:goal` is read by the same walk** since M1f S1f.7.5 and
/// S1f.7.1, `who` being `query` and `clause` `:goal`: a goal is a premise.
/// A query has no activator, so every relation written as a variable is
/// refused, at the top level (M1f S1f.5.21, W64: until then the compiler
/// refused it where a state was answered from the goal, so `ein saturate` and
/// `ein solve` on a program whose R is empty answered it) and inside a
/// pattern (S1f.7.1, W79: until then the compiler lowered the inner pattern as
/// a slot that matches nothing, so `(p (?R ?x))` had no row beside `(p (g
/// k))`, and under an `(absent …)` held in every state). Until S1f.7.5 a goal
/// had two walks of its own, `refuse_guard_patterns` and
/// `refuse_goal_relation_variables`, and neither read a premise that is no
/// form, which a macro's expansion leaves in a goal as in a rule.
fn refuse_no_fact_holds(
    ast: &Ast,
    who: &str,
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
        /// A conclusion: its conjuncts (`Ast::conjuncts`, an `(and …)` read
        /// flat at any depth since M1f S1f.6.37; the top one alone until
        /// then, so an inner `(and …)` was read here as a pattern headed
        /// `and`), each a pattern. A keyword pair beside them was skipped
        /// until S1f.5.11's T1f.5.11.2, and none parses there since M1f
        /// S1f.5.5's swap.
        fn conclusion(&mut self, node: NodeId) {
            let ast = self.ast;
            for c in ast.conjuncts(node) {
                if matches!(ast.node(c), Node::SForm { .. }) {
                    self.pattern(c, None);
                }
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
    if clause != ":assert" {
        let mut bound: Vec<String> = params.iter().map(|p| p.to_string()).collect();
        walk.premise(node, None, &mut bound);
    } else {
        walk.conclusion(node);
    }
    let dc = |n: NodeId| crate::dump::dump_compact(ast, n);
    const PREMISE: &str = "a premise is a pattern (relation argument …), a guard (eq a b) or \
                           (neq a b), an (absent …), or an (and …) or (or …) of premises";
    const GUARD: &str = "a guard compares two values, each a variable, a name, or a fact \
                         that holds no variable";
    for f in walk.out {
        errors.push(match f {
            Found::FreeHead(form, h) if clause == ":goal" => format!(
                "{who}: its {clause} writes {} with {} as its relation, which nothing binds — a \
                 relation is named by a name, or by a rule's parameter its activator binds, \
                 and a query has no activator",
                dc(form),
                dc(h)
            ),
            Found::FreeHead(form, h) => format!(
                "{who}: its {clause} writes {} with {} as its relation, which no \
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
                    "{who}: its {clause} {place}, and {} is no premise — {PREMISE}",
                    dc(p)
                )
            }
            Found::GuardNode(guard, arg) => format!(
                "{who}: its {clause} writes {}, and the form {} holds a variable — \
                 {GUARD}",
                dc(guard),
                dc(arg)
            ),
        });
    }
}

/// **A conclusion's every variable is bound** — M1f S1f.5.10, on the user's
/// ruling of 2026-11-07 on F13 (*"yes, file F13 with the refusal"*).
///
/// Every variable a rule's or an hrule's `:assert` writes, at any depth, must
/// be bound on every way its `:match` can hold, as the compiler reads it
/// (`ein_infer::compile`):
///
/// - by a **parameter**, which the activator binds;
/// - by a **relation premise**, at any depth of its arguments — a stored
///   `(not P)` among them, a pattern over stored negations;
/// - or by an **`eq` or `neq` guard**, whose variable nothing else binds
///   ranges over the program's names (M1f S1f.5.7).
///
/// A variable seen only inside an `(absent …)` — `forall` and `unknown` expand
/// to one — is not bound outside it. A top-level `(or …)` is one way per
/// disjunct, and the variable must be bound in each; a nested one binds what
/// every one of its arms binds (and is refused since M1f S1f.5.21, [`refuse_nested_disjunctions`]).
/// An `(open …)` conclusion names a relation, which `validate_open` checks.
///
/// One line per variable, in the order the `:assert` writes them, naming the
/// rule, the variable, the disjuncts that leave it unbound where some bind it,
/// and the two explicit spellings of what the implicit form could mean:
/// *every object of a type*, a premise such as `(is-a ?y T)`, and *every name
/// of the program*, `(eq ?y ?y)`.
///
/// **Until S1f.5.10** the loader did not check it, and the run found it at
/// whichever node first fired the rule: it stopped with
/// `ein_infer::firing::FireError::UnboundVar`'s *unbound var ?y in :assert —
/// bindings: {'x': 'a'}*, exit 1, or, where only a commitment fires the rule,
/// answered on the commands that never reached one; an hrule's template
/// proposed nothing, in silence; and a rule that never fired loaded and
/// answered.
fn refuse_unbound_conclusions(
    ast: &Ast,
    head: &str,
    name: &str,
    params: &[&str],
    match_node: NodeId,
    assert_node: NodeId,
    errors: &mut Vec<String>,
) {
    /// What a premise binds outside itself.
    fn binds(ast: &Ast, node: NodeId, out: &mut BTreeSet<String>) {
        if !matches!(ast.node(node), Node::SForm { .. }) {
            return;
        }
        match ast.head_name(node) {
            Some("absent") => {}
            Some("and") => {
                for &a in ast.form_args(node) {
                    binds(ast, a, out);
                }
            }
            Some("or") => {
                let mut every: Option<BTreeSet<String>> = None;
                for &a in ast.form_args(node) {
                    let mut arm = BTreeSet::new();
                    binds(ast, a, &mut arm);
                    every = Some(match every {
                        None => arm,
                        Some(e) => e.intersection(&arm).cloned().collect(),
                    });
                }
                out.extend(every.unwrap_or_default());
            }
            // A guard's argument is a variable, a name, or a fact that holds
            // no variable (`refuse_no_fact_holds`), and a relation premise's
            // every variable binds, so both read the same here.
            _ => vars_under(ast, node, out),
        }
    }
    /// Each variable under `node`, in source order, once.
    fn written(ast: &Ast, node: NodeId, out: &mut Vec<String>) {
        match ast.node(node) {
            Node::Var(s) => {
                let s = ast.sym(s);
                if !out.iter().any(|o| o == s) {
                    out.push(s.to_string());
                }
            }
            Node::SForm { .. } => {
                for &a in ast.form_args(node) {
                    written(ast, a, out);
                }
            }
            _ => {}
        }
    }
    let disjuncts: Vec<NodeId> = if matches!(ast.node(match_node), Node::SForm { .. })
        && ast.head_name(match_node) == Some("or")
    {
        ast.form_args(match_node).to_vec()
    } else {
        vec![match_node]
    };
    let bound: Vec<BTreeSet<String>> = disjuncts
        .iter()
        .map(|&d| {
            let mut b: BTreeSet<String> = params.iter().map(|p| p.to_string()).collect();
            binds(ast, d, &mut b);
            b
        })
        .collect();
    let mut vars: Vec<String> = Vec::new();
    for c in ast.conjuncts(assert_node) {
        if !is_open_form(ast, c) {
            written(ast, c, &mut vars);
        }
    }
    for v in vars {
        let unbound: Vec<NodeId> = disjuncts
            .iter()
            .zip(&bound)
            .filter(|(_, b)| !b.contains(&v))
            .map(|(&d, _)| d)
            .collect();
        if unbound.is_empty() {
            continue;
        }
        let place = if unbound.len() == disjuncts.len() {
            String::new()
        } else {
            let arms: Vec<String> = unbound
                .iter()
                .map(|&d| crate::dump::dump_compact(ast, d))
                .collect();
            format!(" where its :match holds through {}", arms.join(" or "))
        };
        errors.push(format!(
            "{head} '{name}': its :assert writes ?{v}, which no premise or parameter binds{place} \
             — to conclude it of every object of a type, bind it with a premise such as \
             (is-a ?{v} T), or of every name of the program with (eq ?{v} ?{v})"
        ));
    }
}

/// **An `(or …)` is a whole `:match`, or a whole `:goal`** — M1f S1f.5.21,
/// on the user's ruling on the close review's finding 1 (*"new stage in
/// p1f.5"*), W62.
///
/// A top-level `(or …)` is one way the rule holds per disjunct, each compiled
/// as a plan of its own (`ein_infer::compile`'s `match_disjuncts`). One written
/// as a premise, of an `(and …)`, an `(absent …)` or a top-level `(or …)`, is
/// not lowered: until S1.22.0 the compiler dropped it, so the rule fired
/// unguarded or its guard retired its candidates, and from then until
/// S1f.5.21 it refused it where it compiled the plan, with the Python IR's
/// spelling. A plan is compiled per activator, by the hypothesis layer for an
/// hrule and where a state is answered for a goal, so a rule only a commitment
/// activates, an hrule, a goal and one no state reached loaded, and some
/// commands answered what others stopped on. The loader refuses it since, one
/// line per disjunction as written, naming what it is a premise of; the walk
/// is the compiler's (`premise`), so it does not descend into a pattern, a
/// guard or a stored negation, where an `(or …)` is no premise.
fn refuse_nested_disjunctions(
    ast: &Ast,
    who: &str,
    clause: &str,
    node: NodeId,
    errors: &mut Vec<String>,
) {
    fn premise(ast: &Ast, node: NodeId, within: NodeId, out: &mut Vec<(NodeId, NodeId)>) {
        if !matches!(ast.node(node), Node::SForm { .. }) {
            return;
        }
        match ast.head_name(node) {
            Some("or") => out.push((node, within)),
            Some("and") | Some("absent") => {
                for &a in ast.form_args(node) {
                    premise(ast, a, node, out);
                }
            }
            _ => {}
        }
    }
    let mut found: Vec<(NodeId, NodeId)> = Vec::new();
    if matches!(ast.node(node), Node::SForm { .. }) {
        match ast.head_name(node) {
            // A top-level `(or …)`: each disjunct is a way, and an `(or …)`
            // written as one is nested.
            Some("or") | Some("and") | Some("absent") => {
                for &a in ast.form_args(node) {
                    premise(ast, a, node, &mut found);
                }
            }
            _ => {}
        }
    }
    let split = if clause == ":goal" {
        "ask two queries"
    } else {
        "split the rule"
    };
    for (or, within) in found {
        let parent = ast.head_name(within).unwrap_or("");
        errors.push(format!(
            "{who}: its {clause} writes {} as a premise of {parent} — an (or …) is a whole \
             {clause}, each of its premises one way it holds: lift it to the top of the {clause}, \
             or {split}",
            crate::dump::dump_compact(ast, or),
        ));
    }
}

/// **A macro's expansion is held to the shapes the grammar gives a written
/// rule** — M1f S1f.5.24, on the user's ruling of 2026-11-10 on Q-M1f.67
/// (*"the ultimate goal in this case is not allow execution of (eq
/// <arg>)"*), W66 to W69.
///
/// A macro's body is a template whose head may be any name at any arity
/// (`ein.y`'s `Template`), and a call's argument may be any premise, so the
/// grammar, which shapes each kernel primitive where a rule writes it, never
/// reads the expanded rule. Until S1f.5.24 what it wrote was met where it was
/// used, or not at all: `eq` and `absent` at another arity by the compiler,
/// where it compiled the plan, so a rule only a commitment activates, an hrule,
/// an obligation and a rule no activator reaches loaded and some or every
/// command answered (W66); a `not` of no fact, a connective where a value goes,
/// `false` with an argument and an `(or …)` concluded were matched, concluded
/// or proposed as they stood, `(eq k)` stored (W67, W68); and a conclusion that
/// is no form stopped the firing that met it and was dropped by the hypothesis
/// layer (W69). An unused macro is not read here, so one that no rule calls
/// loads whatever its body (the ruling); one a rule calls is refused where it
/// is called, at load, on every command (Q-M1f.80).
///
/// **Since M1f S1f.7.7, S1f.7.6, S1f.7.10 and S1f.7.3 the walk holds the value
/// rule too** (the user's ruling of 2026-11-11, Q-M1f.113: *a value is a name,
/// or a fact a state could hold: an atom of an ordinary relation, or its
/// negation*; W80 to W83). The grammar refuses what a program writes of it,
/// through `ValueHead`; what is left to this walk is what a macro's expansion
/// writes, and the one shape the grammar cannot refuse, a connective as a
/// premise pattern's argument, which it takes so that a macro can be called
/// with premises ([`Shapes::arg`]). So here, a rule's too: `and`, `or`, `neq`,
/// `eq`, `absent` and `false` with an argument where a value goes, a premise
/// `(false ?x)`, a conclusion headed `absent`, and a `(not …)` that is a value
/// negating an equality.
///
/// The walk is the grammar's, by position (Q-M1f.81): a premise (`Premise`), a
/// pattern's argument (`Arg`, which takes a connective so a macro can be called
/// there), a value (`Term`), the pattern a `not` negates (`Pattern`) and a
/// conclusion (`Conclusion`). It reads a form headed by `eq`, `neq`, `absent`,
/// `not`, `false`, `and` or `or` against the shape the grammar gives it there,
/// and a conclusion that is no form; what other checks read it leaves to them:
/// the verdict `(open …)` (`validate_open`; the grammar writes it only as a
/// rule's whole `:assert` since M1f S1f.5.23, and a macro's body and a call's
/// argument write no `open`), a premise that is no form and a relation variable
/// no parameter binds (`refuse_no_fact_holds`), and an `(or …)` written as a
/// premise (`refuse_nested_disjunctions`). Until M1f S1f.7.3 a written rule
/// never failed it, the grammar having refused the same shapes at the parse;
/// since, a connective written as a pattern's argument does. One line per form, in the
/// compiler's spelling (S1f.5.21's): the position where the expanded form is
/// written, which is the macro's body or the call's argument, then the rule,
/// the clause, the form as Ein text and what to write instead (Q-M1f.82).
fn refuse_malformed_primitives(
    ast: &Ast,
    who: &str,
    clause: &str,
    node: NodeId,
    errors: &mut Vec<String>,
) {
    let mut walk = Shapes {
        ast,
        who,
        clause,
        found: Vec::new(),
    };
    if clause == ":assert" {
        walk.conclusion(node);
    } else {
        walk.premise(node);
    }
    errors.extend(walk.found);
}

/// What a node is, to [`Shapes`]: no form, or a form and its head's name
/// (`None` for a head that is a variable).
enum Shape<'a> {
    Leaf,
    Form(Option<&'a str>),
}

/// [`refuse_malformed_primitives`]' walk: one method per position of
/// `ein.y` a rule's clause reaches.
struct Shapes<'a> {
    ast: &'a Ast,
    who: &'a str,
    clause: &'a str,
    found: Vec<String>,
}

impl<'a> Shapes<'a> {
    fn shape(&self, node: NodeId) -> Shape<'a> {
        let ast: &'a Ast = self.ast;
        match ast.node(node) {
            Node::SForm { .. } => Shape::Form(ast.head_name(node)),
            _ => Shape::Leaf,
        }
    }

    fn args(&self, node: NodeId) -> Vec<NodeId> {
        self.ast.form_args(node).to_vec()
    }

    fn text(&self, node: NodeId) -> String {
        crate::dump::dump_compact(self.ast, node)
    }

    /// `<file>:<line>:<col>: `, where the expanded form is written: the first
    /// position at or under it, in the order it is written. A form the
    /// macro's body writes as `(eq ?a)` has its own, the body's; one the
    /// grammar builds for a keyword, `(not …)`, `(neq …)`, `(and …)` or
    /// `(or …)`, has none (`at None`, F23's D-2), so its first part's is
    /// taken, which is the call's argument where the argument is what is
    /// malformed, and in the program's file where `std.macro`'s `unknown`
    /// wrote the form. Nothing where no part has one.
    fn place(&self, node: NodeId) -> String {
        fn first(ast: &Ast, node: NodeId) -> Option<crate::ast::Loc> {
            if let Some(l) = ast.loc(node) {
                return Some(l);
            }
            match ast.node(node) {
                Node::SForm { head, .. } => first(ast, head)
                    .or_else(|| ast.form_args(node).iter().find_map(|&a| first(ast, a))),
                _ => None,
            }
        }
        match first(self.ast, node) {
            Some(loc) => format!("{}: ", loc_repr(self.ast, Some(loc))),
            None => String::new(),
        }
    }

    /// `<place><who>: its <clause> writes <form>, and <what>`.
    fn refuse(&mut self, node: NodeId, what: String) {
        let line = format!(
            "{}{}: its {} writes {}, and {what}",
            self.place(node),
            self.who,
            self.clause,
            self.text(node)
        );
        self.found.push(line);
    }

    /// `eq` and `neq` compare two values (`ein.y`'s `Guard`).
    fn guard(&mut self, node: NodeId, name: &str) {
        let args = self.args(node);
        if args.len() != 2 {
            self.refuse(
                node,
                format!(
                    "{name} takes exactly 2 arguments, not {} — it compares two values, so write \
                     one guard per pair",
                    args.len()
                ),
            );
            return;
        }
        for a in args {
            self.term(node, a);
        }
    }

    /// A premise (`ein.y`'s `Premise`): a pattern, or a connective.
    fn premise(&mut self, node: NodeId) {
        let Shape::Form(head) = self.shape(node) else {
            return; // a premise that is no form: `refuse_no_fact_holds`'s
        };
        let args = self.args(node);
        match head {
            Some("and" | "or") => {
                for a in args {
                    self.premise(a);
                }
            }
            Some("absent") => {
                if args.len() != 1 {
                    self.refuse(
                        node,
                        format!(
                            "absent takes exactly 1 argument, not {} — negation as failure is \
                             over one premise, so wrap several in an (and …)",
                            args.len()
                        ),
                    );
                    return;
                }
                self.premise(args[0]);
            }
            Some("not") => self.negation(node, false, false),
            Some(g @ ("eq" | "neq")) => self.guard(node, g),
            // `false` takes no argument, as a premise too (M1f S1f.7.6).
            Some("false") if !args.is_empty() => self.false_with_arguments(node, "holds where"),
            // No premise is `(open …)` since M1f S1f.5.23: the grammar writes
            // `open` only as a rule's whole `:assert`, which `validate_open`
            // holds; until then this arm left one to `validate_open`'s refusal.
            _ => {
                for a in args {
                    self.arg(node, a);
                }
            }
        }
    }

    /// A premise pattern's argument in the pattern `within` (`ein.y`'s `Arg`):
    /// what a value can be, a name, a variable, a pattern of an ordinary
    /// relation, or `(not <pattern>)`, a stored negation.
    ///
    /// The grammar takes a connective there, so that a macro such as `forall`
    /// can be called with premises; once the macros are expanded, one that is
    /// still a pattern's argument is a pattern no fact matches (M1f S1f.7.3,
    /// W83). Until then `(q (and (p ?x)))` loaded and matched nothing, in
    /// silence, and under an `(absent …)` held, so the rule fired.
    fn arg(&mut self, within: NodeId, node: NodeId) {
        if let Some(what) = self.no_value(node) {
            self.refuse(within, what);
            return;
        }
        match self.shape(node) {
            Shape::Form(Some("not")) => self.negation(node, false, true),
            Shape::Form(_) => {
                for a in self.args(node) {
                    self.arg(node, a);
                }
            }
            Shape::Leaf => {}
        }
    }

    /// Why the form `node` is no value, where it is one of the heads no value
    /// has (M1f P1f.7's value rule, the user's ruling of 2026-11-11: *a value
    /// is a name, or a fact a state could hold: an atom of an ordinary
    /// relation, or its negation*): a connective, `and`, `or` or `neq`, as
    /// S1f.5.24 refused; `eq`, computed and never stored (S1f.7.10); `absent`,
    /// a premise's test (S1f.7.7); and `false` with an argument (S1f.7.6).
    fn no_value(&self, node: NodeId) -> Option<String> {
        const VALUE: &str = "a value is a name, a variable, or a fact a state could hold, \
                             (relation argument …) or its negation";
        let text = self.text(node);
        match self.shape(node) {
            Shape::Form(Some(c @ ("and" | "or" | "neq"))) => Some(format!(
                "{text} is a connective where a value goes — {c} heads no fact, and a value is a \
                 name, a variable or a fact (relation argument …)"
            )),
            Shape::Form(Some("eq")) => Some(format!(
                "{text} is written where a value goes — eq is computed and never stored, and \
                 {VALUE}"
            )),
            Shape::Form(Some("absent")) => Some(format!(
                "{text} is written where a value goes — absent is negation as failure, a \
                 premise's test that no fact is of, and {VALUE}; the stored negation of a fact \
                 is (not …)"
            )),
            Shape::Form(Some("false")) if !self.args(node).is_empty() => Some(format!(
                "{text} is written where a value goes — false takes no argument, and {VALUE}"
            )),
            _ => None,
        }
    }

    /// `(false k)` written as a premise or a conclusion (M1f S1f.7.6).
    fn false_with_arguments(&mut self, node: NodeId, says: &str) {
        let n = self.args(node).len();
        self.refuse(
            node,
            format!(
                "false takes no argument, not {n} — (false) {says} the state is impossible, and \
                 names nothing"
            ),
        );
    }

    /// A value (`ein.y`'s `Term`) in the form `within`: a name, a variable, a
    /// fact, or the stored negation of one; never a connective, and never a
    /// form of `eq`, of `absent` or of `false` with an argument.
    fn term(&mut self, within: NodeId, node: NodeId) {
        if let Some(what) = self.no_value(node) {
            self.refuse(within, what);
            return;
        }
        match self.shape(node) {
            Shape::Form(Some("not")) => self.negation(node, false, true),
            Shape::Form(_) => {
                for a in self.args(node) {
                    self.term(node, a);
                }
            }
            Shape::Leaf => {}
        }
    }

    /// `(not P)`: the stored negation of one fact (`ein.y`'s `(not Pattern)`),
    /// and, as a conclusion, of a variable bound to one (`(not Var)`). Written
    /// as a premise or a conclusion of its own it may negate an equality,
    /// `(not (eq a b))`, which is computed (`ein.y`'s `Negated`); as a `value`,
    /// inside a pattern or a conclusion, it negates a fact a state could hold.
    fn negation(&mut self, node: NodeId, conclusion: bool, value: bool) {
        let args = self.args(node);
        if args.len() != 1 {
            self.refuse(
                node,
                format!(
                    "not takes exactly 1 argument, not {} — it is the stored negation of one fact",
                    args.len()
                ),
            );
            return;
        }
        let p = args[0];
        match self.shape(p) {
            Shape::Form(Some("eq")) if !value => return self.guard(p, "eq"),
            Shape::Form(Some("eq" | "absent" | "false")) => {
                if let Some(what) = self.no_value(p) {
                    return self.refuse(node, what);
                }
            }
            _ => {}
        }
        let fits = match (self.ast.node(p), self.shape(p)) {
            (Node::Var(_), _) => conclusion,
            (_, Shape::Form(Some("and" | "or" | "not" | "neq"))) => false,
            (_, Shape::Form(_)) => true,
            _ => false,
        };
        if !fits {
            let takes = if conclusion {
                "a pattern (relation argument …) or a variable"
            } else {
                "a pattern (relation argument …)"
            };
            let what = format!(
                "not takes {takes}, not {} — it is the stored negation of one fact; negation as \
                 failure is (absent …)",
                self.text(p)
            );
            self.refuse(node, what);
            return;
        }
        if matches!(self.shape(p), Shape::Form(_)) {
            for a in self.args(p) {
                self.term(p, a);
            }
        }
    }

    /// A conclusion (`ein.y`'s `Assert` and `Conclusion`): its conjuncts
    /// (`Ast::conjuncts`, an `(and …)` read flat at any depth, the reading
    /// every reader of a conclusion shares since M1f S1f.6.37), each a form.
    fn conclusion(&mut self, node: NodeId) {
        for c in self.ast.conjuncts(node) {
            self.conjunct(c);
        }
    }

    /// One conjunct of a conclusion: a form, and no `(and …)`.
    fn conjunct(&mut self, node: NodeId) {
        let Shape::Form(head) = self.shape(node) else {
            let line = format!(
                "{}{}: its :assert concludes {}, which is not a fact — a conclusion is a fact \
                 (relation argument …), or an (and …) of facts",
                self.place(node),
                self.who,
                self.text(node)
            );
            self.found.push(line);
            return;
        };
        let args = self.args(node);
        match head {
            Some("or") => self.refuse(
                node,
                "an (or …) is no conclusion — a rule concludes every conclusion it writes, and a \
                 disjunction is a :match's, one way it holds per premise: write one rule per \
                 disjunct"
                    .to_string(),
            ),
            Some("not") => self.negation(node, true, false),
            Some("false") if !args.is_empty() => self.false_with_arguments(node, "concludes that"),
            // No fact is of `absent`, so no conclusion is (M1f S1f.7.7).
            Some("absent") => self.refuse(
                node,
                "absent is negation as failure, a premise's test, and no fact is of it — a \
                 conclusion is a fact (relation argument …); the stored negation of a fact is \
                 (not …)"
                    .to_string(),
            ),
            Some(g @ ("eq" | "neq")) => self.guard(node, g),
            // The verdict, a rule's whole `:assert`: `validate_open`'s.
            Some("open") => {}
            _ => {
                for a in args {
                    self.term(node, a);
                }
            }
        }
    }
}

/// **A rule binds at most [`MAX_MATCH_VARIABLES`] variables, and so does a
/// goal** — M1f S1f.5.21, W63 (Q-M1f.73).
///
/// The count is every distinct variable the clause writes, at any depth, and
/// the rule's parameters, which bounds each register file the compiler builds
/// for it. Until S1f.5.21 the compiler refused a plan past its file where it
/// compiled it (`MAX_REGS`, S1a.3.1's one compile-time bound), so a rule no
/// state activated and a goal no state was answered from loaded and answered.
fn refuse_wide_matches(
    ast: &Ast,
    who: &str,
    clause: &str,
    params: &[&str],
    node: NodeId,
    errors: &mut Vec<String>,
) {
    let mut vars: BTreeSet<String> = params.iter().map(|p| p.to_string()).collect();
    vars_under(ast, node, &mut vars);
    if vars.len() <= MAX_MATCH_VARIABLES {
        return;
    }
    let (among, binds, split) = if clause == ":goal" {
        ("", "a goal", "ask two queries")
    } else if params.is_empty() {
        ("", "a rule", "split the rule")
    } else {
        (", its parameters among them", "a rule", "split the rule")
    };
    errors.push(format!(
        "{who}: its {clause} writes {} distinct variables{among}, and {binds} binds at most \
         {MAX_MATCH_VARIABLES} — {split}",
        vars.len()
    ));
}

/// **A goal asks about relations of the program** — M1f S1f.7.11, W78.
///
/// A premise of a goal over a relation the program names nowhere else can
/// have no row in any state, and under an `(absent …)` holds in every one, so
/// a relation misspelt in a goal read as a goal that is undetermined. Each is
/// refused, one line per name, placed at the pattern: the relation of every
/// pattern the goal writes, at any depth, a nested pattern's and a stored
/// negation's too, must be one of `known`, [`program_relations`]. `eq` and
/// `neq` are every program's; the connectives and `absent` are not patterns;
/// and a pattern over a kernel relation, `(relation ?R ?A ?B)`, reads what the
/// loader stores.
fn refuse_goal_relations(
    ast: &Ast,
    goal: NodeId,
    known: &BTreeSet<String>,
    unimported: &[String],
    errors: &mut Vec<String>,
) {
    fn walk(ast: &Ast, node: NodeId, out: &mut Vec<(NodeId, String)>) {
        let Node::SForm { head, .. } = ast.node(node) else {
            return;
        };
        if let Node::Atom(s) = ast.node(head) {
            let name = ast.sym(s);
            if !is_reserved(name)
                && !is_predicate(name)
                && !matches!(name, "and" | "or" | "absent" | "not")
                && !ein_core::text::INTERNAL_NAMES.contains(&name)
                && !out.iter().any(|(_, n)| n == name)
            {
                out.push((node, name.to_string()));
            }
        }
        for &a in ast.form_args(node) {
            walk(ast, a, out);
        }
    }
    let mut found: Vec<(NodeId, String)> = Vec::new();
    walk(ast, goal, &mut found);
    for (node, name) in found {
        // A `std.macro` call the program did not import has its own line
        // (`macros::unimported_macro_errors`).
        if !known.contains(&name) && !unimported.contains(&name) {
            errors.push(format!(
                "{}query: its :goal writes {}, and the program names no relation {name} — \
                 {CLAIM_RELATION}, so no state holds a fact of {name}",
                placed_or(ast, node, goal),
                crate::dump::dump_compact(ast, node),
            ));
        }
    }
}

/// One `(query …)` as the program holds it: its clauses, with the macro calls
/// of its `:goal` expanded as a rule's `:match`'s are — M1f S1f.7.5, W77.
///
/// A goal is a premise. Until S1f.7.5 only a rule's clauses were expanded, so
/// a call in a goal, `(unknown (p ?x))`, was read as a pattern over a relation
/// `unknown`: the goal had no row in any state, `:expect (model)` held of it,
/// and the claim that lists what the macro means was refused at load. The
/// expansion replaces the `:goal` pair's value, so every reader of the query
/// meets it, and [`Query::goal_written`] keeps the goal as written, which the
/// read-out and `ein test` print (Q-M1f.117). A call that does not expand, by
/// its arity or by a macro that expands to itself, is one line, and the goal
/// is left as written.
fn expand_goal(
    ast: &mut Ast,
    form: NodeId,
    macros: &BTreeMap<String, IrMacro>,
    errors: &mut Vec<String>,
) -> Query {
    let mut pairs: Vec<ExprRef> = ast.form_args(form).iter().map(|n| ExprRef(n.0)).collect();
    let mut goal_written = None;
    if !macros.is_empty() {
        for slot in pairs.iter_mut() {
            let pair = NodeId(slot.0);
            let Node::KwPair { key, value } = ast.node(pair) else {
                continue;
            };
            if !matches!(ast.node(key), Node::Keyword(k) if ast.sym(k) == "goal") {
                continue;
            }
            match macros::expand_macros(ast, value, macros) {
                Ok(expanded) => {
                    let loc = ast.loc(pair);
                    *slot = ExprRef(
                        ast.push(
                            Node::KwPair {
                                key,
                                value: expanded,
                            },
                            loc,
                        )
                        .0,
                    );
                    goal_written = Some(ExprRef(value.0));
                }
                Err(e) => errors.push(format!("query: its :goal: {e}")),
            }
            // The first `:goal` is the query's (`query_keyword`).
            break;
        }
    }
    Query {
        kw_pairs: pairs.into_boxed_slice(),
        goal_written,
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
/// only to a run; since M1f S1f.5.18 the loader refuses a program in which a
/// rule could derive one, [`refuse_derived_relation_activators`], below, and
/// the compiler's refusal (`ein_infer::compile`) is the backstop behind both,
/// which a direct caller of `compile_rule` reaches. An activator of the wrong arity activates nothing
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

/// **A rule could derive an activator that binds a relation parameter to a
/// fact** — M1f S1f.5.18, on the user's ruling of 2026-11-08 on Q-M1f.1 (b)
/// (*"Q1 - b"*): G-14's derived activator, refused at load by a static check,
/// whether or not a run derives it.
///
/// [`validate_relation_parameters`] refuses a relation parameter bound to a
/// fact where the activator is written down (W24). An activator a rule
/// derives, `(rule mk () :match (item ?f) :assert (r ?f))`, binds `r`'s
/// parameter to whatever `?f` holds, which only a run knows; until S1f.5.18 the
/// run stopped where `r`'s plan was compiled, or answered where no run derived
/// it (W59). This check reads the program instead.
///
/// **Which positions hold relation names** (Q-M1f.41 (b), taken by default on
/// the user's instruction of 2026-11-09). A rule's parameter that heads a form,
/// `(?R …)`, holds one, and every argument of `(relation …)` holds a name. An
/// argument of a **declared** relation holds any value of its type, a fact
/// among them, as does an argument inside a fact pattern, `(p (g ?x))`, and an
/// argument of a relation a parameter names, `(?S ?x)`. An argument of an
/// **undeclared** relation, a marker such as `(bijective color-loc)`, and a
/// rule's parameter hold what the program passes on from them: where a
/// variable bound there reaches a position that holds relation names, so does
/// that position, inferred to a fixpoint. This is what lets the stdlib's
/// fan-outs load, `bijective-properties` passing `(bijective ?R)`'s `?R` to
/// `functional`, which the sketch of the stage file would refuse with 32
/// corpus programs.
///
/// **What is refused**, one line per site, in Ein's spelling:
/// - a conclusion that writes, where a relation name is held, a variable no
///   premise binds where a name is held, the deriving rule, the conclusion,
///   the parameter and the premise named;
/// - a conclusion or a fact that writes a fact where an inferred position holds
///   a relation name (a rule's own activator is W24's, above).
///
/// A conclusion written `(?T …)` writes into the relation its activator
/// names: it is read as writing into every rule of its arity whose name the
/// program writes as an argument (Q-M1f.42 (a)), which is where a rule's name
/// can come from. `(not P)` is read as `P`, in a premise, a conclusion and a
/// fact, since a stored negative holds the values its positive would.
///
/// **What it returns** (M1f S1f.7.8): the positions that hold relation names,
/// and every name the program writes at one, in a fact or a conclusion — the
/// relations a program names only by passing them to a rule, which
/// [`refuse_solve_names`] reads.
fn refuse_derived_relation_activators(
    ast: &Ast,
    terms: &Terms,
    kb: &Kb,
    facts: &[NodeId],
    solve_blocks: &[NodeId],
    errors: &mut Vec<String>,
) -> RelationPlaces {
    /// A position: a relation's name and an argument's index.
    type Pos = (String, usize);
    /// Why a position holds relation names.
    enum Holds {
        /// A rule's parameter that heads a form in it.
        Param { rule: String, param: String },
        /// A variable bound there reaches `to`, through rule `by`: a premise
        /// of `by` binds it (`premise` as written), or it is `by`'s parameter.
        Passed {
            by: String,
            premise: Option<String>,
            var: String,
            to: Pos,
        },
    }
    /// Where a premise binds a variable.
    enum Occ {
        /// The deriving rule's parameter, by index.
        Param(usize),
        /// An argument of a premise whose relation is a name.
        Arg {
            rel: String,
            k: usize,
            premise: NodeId,
        },
        /// An argument of a fact pattern inside a premise.
        Inner { premise: NodeId, inner: NodeId },
        /// An argument of a premise whose relation is a parameter.
        Head { premise: NodeId, head: String },
        /// An `eq` or `neq` guard, which binds it over the program's names.
        Guard,
    }
    /// One argument a conclusion writes where a rule's activator may stand.
    struct Site {
        head: &'static str,
        rule: String,
        conclusion: NodeId,
        to: Pos,
        arg: NodeId,
        /// Per disjunct of the deriving rule's `:match`, where it binds `arg`.
        ways: Vec<(NodeId, Vec<Occ>)>,
    }

    fn describe(rp: &BTreeMap<Pos, Holds>, pos: &Pos) -> String {
        match &rp[pos] {
            Holds::Param { rule, param } => {
                format!("rule '{rule}''s parameter {param}, which names a relation")
            }
            Holds::Passed {
                by,
                premise: Some(p),
                var,
                to,
            } => format!(
                "{var} of {p}, which rule '{by}' passes to {}",
                describe(rp, to)
            ),
            Holds::Passed {
                by,
                premise: None,
                var,
                to,
            } => format!(
                "rule '{by}''s parameter {var}, which it passes to {}",
                describe(rp, to)
            ),
        }
    }
    /// `(not P)` read as `P`.
    fn positive(ast: &Ast, node: NodeId) -> NodeId {
        if matches!(ast.node(node), Node::SForm { .. })
            && ast.head_name(node) == Some("not")
            && ast.form_args(node).len() == 1
        {
            ast.form_args(node)[0]
        } else {
            node
        }
    }
    fn is_guard(ast: &Ast, node: NodeId) -> bool {
        matches!(ast.head_name(positive(ast, node)), Some("eq") | Some("neq"))
    }
    /// The relation a form names as written, or `None` for a parameter.
    fn relation_of(ast: &Ast, node: NodeId) -> Option<String> {
        match ast.node(node) {
            Node::SForm { head, .. } => match ast.node(head) {
                Node::Var(_) => None,
                _ => ast.head_name(node).map(str::to_string),
            },
            _ => None,
        }
    }
    /// Where `node`, a premise or a connective, binds `var` outside itself.
    fn occs(ast: &Ast, node: NodeId, var: &str, out: &mut Vec<Occ>) {
        if !matches!(ast.node(node), Node::SForm { .. }) {
            return;
        }
        match ast.head_name(node) {
            Some("absent") => {}
            Some("and") | Some("or") => {
                for &a in ast.form_args(node) {
                    occs(ast, a, var, out);
                }
            }
            _ if is_guard(ast, node) => {
                let mut vs = BTreeSet::new();
                vars_under(ast, node, &mut vs);
                if vs.contains(var) {
                    out.push(Occ::Guard);
                }
            }
            _ => {
                let p = positive(ast, node);
                let rel = relation_of(ast, p);
                let head = match ast.node(p) {
                    Node::SForm { head, .. } => match ast.node(head) {
                        Node::Var(s) => format!("?{}", ast.sym(s)),
                        _ => String::new(),
                    },
                    _ => String::new(),
                };
                for (k, &a) in ast.form_args(p).iter().enumerate() {
                    match ast.node(a) {
                        Node::Var(s) if ast.sym(s) == var => out.push(match &rel {
                            Some(rel) => Occ::Arg {
                                rel: rel.clone(),
                                k,
                                premise: node,
                            },
                            None => Occ::Head {
                                premise: node,
                                head: head.clone(),
                            },
                        }),
                        Node::SForm { .. } => inner(ast, a, var, node, out),
                        _ => {}
                    }
                }
            }
        }
    }
    fn inner(ast: &Ast, form: NodeId, var: &str, premise: NodeId, out: &mut Vec<Occ>) {
        for &a in ast.form_args(form) {
            match ast.node(a) {
                Node::Var(s) if ast.sym(s) == var => out.push(Occ::Inner {
                    premise,
                    inner: form,
                }),
                Node::SForm { .. } => inner(ast, a, var, premise, out),
                _ => {}
            }
        }
    }
    /// Every name written as an argument under `node`, at any depth.
    fn arguments(ast: &Ast, node: NodeId, out: &mut BTreeSet<String>) {
        match ast.node(node) {
            Node::SForm { .. } => {
                for &a in ast.form_args(node) {
                    if let Node::Atom(s) = ast.node(a) {
                        out.insert(ast.sym(s).to_string());
                    }
                    arguments(ast, a, out);
                }
            }
            Node::KwPair { value, .. } => arguments(ast, value, out),
            _ => {}
        }
    }

    let program = kb.program();
    let registries: [(&'static str, &ein_core::entities::Registry<Rule>); 3] = [
        ("rule", &program.rules),
        ("hrule", &program.hrules),
        ("rule", &program.obligations),
    ];
    // Each rule's parameters, and the positions that hold relation names
    // because a parameter heads a form.
    let mut params: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut rp: BTreeMap<Pos, Holds> = BTreeMap::new();
    for (_, registry) in registries {
        for (name, rule) in registry.iter() {
            let name = terms.sym(name).to_string();
            let ps: Vec<String> = rule
                .params
                .iter()
                .map(|&p| terms.sym(p).to_string())
                .collect();
            for (i, p) in ps.iter().enumerate() {
                if [rule.match_.as_ref(), rule.assert_.as_ref()]
                    .into_iter()
                    .flatten()
                    .any(|pat| names_relation(ast, NodeId(pat.expr.0), p))
                {
                    rp.insert(
                        (name.clone(), i),
                        Holds::Param {
                            rule: name.clone(),
                            param: format!("?{p}"),
                        },
                    );
                }
            }
            params.insert(name, ps);
        }
    }
    if rp.is_empty() {
        return RelationPlaces::default();
    }
    let declared: BTreeSet<String> = program
        .relations
        .iter()
        .filter(|(_, r)| r.declared)
        .map(|(n, _)| terms.sym(n).to_string())
        .collect();
    // The names a program writes as an argument: where a rule's name can come
    // from, for a conclusion written `(?T …)` (Q-M1f.42 (a)).
    let mut written: BTreeSet<String> = BTreeSet::new();
    for &f in facts.iter().chain(solve_blocks) {
        arguments(ast, f, &mut written);
    }
    for (_, registry) in registries {
        for (_, rule) in registry.iter() {
            for pat in [rule.match_.as_ref(), rule.assert_.as_ref()]
                .into_iter()
                .flatten()
            {
                arguments(ast, NodeId(pat.expr.0), &mut written);
            }
        }
    }

    // Every argument a conclusion writes, with where the deriving rule's
    // `:match` binds it.
    let mut sites: Vec<Site> = Vec::new();
    for (head, registry) in registries {
        for (name, rule) in registry.iter() {
            let Some(assert) = rule.assert_.as_ref() else {
                continue;
            };
            let rule_name = terms.sym(name).to_string();
            let disjuncts: Vec<NodeId> = match rule.match_.as_ref() {
                None => Vec::new(),
                Some(m) => {
                    let m = NodeId(m.expr.0);
                    if matches!(ast.node(m), Node::SForm { .. }) && ast.head_name(m) == Some("or") {
                        ast.form_args(m).to_vec()
                    } else {
                        vec![m]
                    }
                }
            };
            for c in ast.conjuncts(NodeId(assert.expr.0)) {
                if is_open_form(ast, c) || !matches!(ast.node(c), Node::SForm { .. }) {
                    continue;
                }
                let conclusion = positive(ast, c);
                let args = ast.form_args(conclusion).to_vec();
                let targets: Vec<String> = match relation_of(ast, conclusion) {
                    Some(rel) => vec![rel],
                    None => params
                        .iter()
                        .filter(|(r, ps)| ps.len() == args.len() && written.contains(*r))
                        .map(|(r, _)| r.clone())
                        .collect(),
                };
                for to in targets {
                    for (i, &arg) in args.iter().enumerate() {
                        let ways = match ast.node(arg) {
                            Node::Var(s) => {
                                let var = ast.sym(s);
                                let by_param: Vec<usize> = rule
                                    .params
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, p)| terms.sym(**p) == var)
                                    .map(|(m, _)| m)
                                    .collect();
                                let ds: Vec<NodeId> = if disjuncts.is_empty() {
                                    vec![NodeId(assert.expr.0)]
                                } else {
                                    disjuncts.clone()
                                };
                                ds.iter()
                                    .map(|&d| {
                                        let mut o: Vec<Occ> =
                                            by_param.iter().map(|&m| Occ::Param(m)).collect();
                                        if !disjuncts.is_empty() {
                                            occs(ast, d, var, &mut o);
                                        }
                                        (d, o)
                                    })
                                    .collect()
                            }
                            _ => Vec::new(),
                        };
                        sites.push(Site {
                            head,
                            rule: rule_name.clone(),
                            conclusion: c,
                            to: (to.clone(), i),
                            arg,
                            ways,
                        });
                    }
                }
            }
        }
    }

    // A way binds the argument to a name where one of its occurrences is at a
    // position that holds relation names, or a guard.
    let named = |rp: &BTreeMap<Pos, Holds>, rule: &str, occ: &Occ| -> bool {
        match occ {
            Occ::Param(m) => rp.contains_key(&(rule.to_string(), *m)),
            Occ::Arg { rel, k, .. } => rel == "relation" || rp.contains_key(&(rel.clone(), *k)),
            Occ::Guard => true,
            Occ::Inner { .. } | Occ::Head { .. } => false,
        }
    };
    // The fixpoint: a way that binds the argument nowhere a name is held, but
    // at a marker's argument or the rule's own parameter, makes that position
    // one that holds relation names.
    loop {
        let mut grew = false;
        for s in &sites {
            if !rp.contains_key(&s.to) || !matches!(ast.node(s.arg), Node::Var(_)) {
                continue;
            }
            let var = crate::dump::dump_compact(ast, s.arg);
            for (_, way) in &s.ways {
                if way.iter().any(|o| named(&rp, &s.rule, o)) {
                    continue;
                }
                for o in way {
                    let (pos, premise) = match o {
                        Occ::Param(m) => ((s.rule.clone(), *m), None),
                        Occ::Arg { rel, k, premise } if !declared.contains(rel) => (
                            (rel.clone(), *k),
                            Some(crate::dump::dump_compact(ast, *premise)),
                        ),
                        _ => continue,
                    };
                    if let std::collections::btree_map::Entry::Vacant(e) = rp.entry(pos) {
                        e.insert(Holds::Passed {
                            by: s.rule.clone(),
                            premise,
                            var: var.clone(),
                            to: s.to.clone(),
                        });
                        grew = true;
                    }
                }
            }
        }
        if !grew {
            break;
        }
    }

    const NAMED: &str = "a relation is named by a symbol, not a fact";
    for s in &sites {
        if !rp.contains_key(&s.to) {
            continue;
        }
        let conclusion = crate::dump::dump_compact(ast, s.conclusion);
        let arg = crate::dump::dump_compact(ast, s.arg);
        match ast.node(s.arg) {
            // A fact written as a rule's own activator, where the rule's
            // parameter heads a form, is W24's line, above.
            Node::SForm { .. } => {
                let written_as_activator = relation_of(ast, positive(ast, s.conclusion)).is_some()
                    && matches!(rp[&s.to], Holds::Param { .. });
                if !written_as_activator {
                    errors.push(format!(
                        "{} '{}': its :assert writes {conclusion}, whose {arg} is {} — {NAMED}",
                        s.head,
                        s.rule,
                        describe(&rp, &s.to)
                    ));
                }
            }
            Node::Var(_) => {
                let bad: Vec<(NodeId, &Occ)> = s
                    .ways
                    .iter()
                    .filter(|(_, way)| !way.iter().any(|o| named(&rp, &s.rule, o)))
                    .filter_map(|(d, way)| way.first().map(|o| (*d, o)))
                    .collect();
                let Some(&(_, first)) = bad.first() else {
                    continue;
                };
                let how = match first {
                    Occ::Arg { rel, premise, .. } => format!(
                        "{} can bind {arg} to a fact, as an argument of the declared relation {rel}",
                        crate::dump::dump_compact(ast, *premise)
                    ),
                    Occ::Inner { premise, inner } => format!(
                        "{} can bind {arg} to a fact, as an argument of {}",
                        crate::dump::dump_compact(ast, *premise),
                        crate::dump::dump_compact(ast, *inner)
                    ),
                    Occ::Head { premise, head } => format!(
                        "{} can bind {arg} to a fact, as an argument of the relation {head} names",
                        crate::dump::dump_compact(ast, *premise)
                    ),
                    Occ::Param(_) | Occ::Guard => unreachable!("a parameter or a guard is named"),
                };
                let place = if bad.len() == s.ways.len() || s.ways.len() < 2 {
                    String::new()
                } else {
                    let arms: Vec<String> = bad
                        .iter()
                        .map(|(d, _)| crate::dump::dump_compact(ast, *d))
                        .collect();
                    format!(" where its :match holds through {}", arms.join(" or "))
                };
                errors.push(format!(
                    "{} '{}': its :assert writes {conclusion}, whose {arg} is {}, and {how}{place} — {NAMED}",
                    s.head,
                    s.rule,
                    describe(&rp, &s.to)
                ));
            }
            _ => {}
        }
    }
    // A fact that writes a fact where an inferred position holds a relation
    // name; one written as a rule's own activator is W24's, above.
    for &f in facts {
        let p = positive(ast, f);
        let Some(rel) = relation_of(ast, p) else {
            continue;
        };
        for (k, &a) in ast.form_args(p).iter().enumerate() {
            let pos = (rel.clone(), k);
            if matches!(rp.get(&pos), Some(Holds::Passed { .. }))
                && matches!(ast.node(a), Node::SForm { .. })
            {
                errors.push(format!(
                    "the fact {} writes {} as {} — {NAMED}",
                    crate::dump::dump_compact(ast, f),
                    crate::dump::dump_compact(ast, a),
                    describe(&rp, &pos)
                ));
            }
        }
    }

    // `eq` at another arity, and each of the three words that head no value
    // (M1f P1f.7's value rule): `eq`, `absent`, and `false` with an argument.
    for word in ["eq", "absent", "false"] {
        refuse_eq_at_another_arity(EqArity {
            ast,
            terms,
            program,
            word,
            rp: &rp,
            sites: &sites,
            written: &written,
            facts,
            solve_blocks,
            errors: &mut *errors,
        });
    }

    // What the program passes where a relation name is held: a fact's
    // argument, and a conclusion's (M1f S1f.7.8).
    let mut places = RelationPlaces {
        positions: rp.keys().cloned().collect(),
        passed: BTreeSet::new(),
    };
    for &f in facts {
        let p = positive(ast, f);
        let Some(rel) = relation_of(ast, p) else {
            continue;
        };
        for (k, &a) in ast.form_args(p).iter().enumerate() {
            if let Node::Atom(s) = ast.node(a)
                && rp.contains_key(&(rel.clone(), k))
            {
                places.passed.insert(ast.sym(s).to_string());
            }
        }
    }
    for s in &sites {
        if let Node::Atom(a) = ast.node(s.arg)
            && rp.contains_key(&s.to)
        {
            places.passed.insert(ast.sym(a).to_string());
        }
    }
    return places;

    /// What [`refuse_eq_at_another_arity`] reads off the check above.
    struct EqArity<'a> {
        ast: &'a Ast,
        terms: &'a Terms,
        program: &'a ein_core::program::Program,
        /// `eq`, `absent` or `false`: the name a parameter may be bound to.
        word: &'static str,
        rp: &'a BTreeMap<Pos, Holds>,
        sites: &'a [Site],
        written: &'a BTreeSet<String>,
        facts: &'a [NodeId],
        solve_blocks: &'a [NodeId],
        errors: &'a mut Vec<String>,
    }

    /// **A relation parameter that can be bound to `eq` heads a form of
    /// another arity** — M1f S1f.5.9, W60, the shape S1f.5.18 found.
    ///
    /// `eq` is the relation of equality, between exactly two values (M1f
    /// S1f.5.7), and the grammar refuses `(eq ?x)` wherever a premise or a
    /// conclusion is written; but a form whose relation is a parameter,
    /// `(?R ?x)`, is `(eq ?x)` once an activator binds `?R` to `eq`, a name a
    /// fact may hold. Until S1f.5.9 such a premise loaded and stopped the run
    /// where the activator's plan was compiled (M1e S1e.2.1's arity check, the
    /// compiler's `CompileError`), at root, or only under a commitment, where
    /// `ein saturate` answered; and such a conclusion stored `(eq a)`.
    ///
    /// Read as the check above reads relation names: a position that holds
    /// relation names can hold `eq` where a fact, a conclusion or a `(solve
    /// :hrules …)` activator writes `eq` there; or where a conclusion writes a
    /// variable that its `:match` binds, in some disjunct, only at positions
    /// that can hold `eq`, or only with an `eq` or `neq` guard, over the
    /// program's names, while the program writes `eq` as a name; to a
    /// fixpoint. A parameter that can hold `eq` is refused for each premise or
    /// conclusion it heads whose arity is not 2, one line each.
    ///
    /// **The same reading holds a parameter to the value rule** since M1f
    /// S1f.7.7, S1f.7.6 and S1f.7.10 (the user's ruling of 2026-11-11: a value
    /// is a name, or a fact a state could hold). `c.word` is `eq`, `absent` or
    /// `false`, and a parameter that can hold it is refused for each premise
    /// or conclusion it heads that the grammar refuses written out, `(absent
    /// ?x)` at any arity and `(false ?x)` at one argument or more, and for
    /// each form it heads where a value goes, inside a pattern or a conclusion,
    /// which for `eq` is `(q (?R ?x ?y))`. Until then `(?R ?x)` concluded with
    /// `?R` bound to `absent` stored `(absent a)`, and `(q (?R ?x ?x))` with
    /// `?R` bound to `eq` stored `(q (eq a a))`.
    fn refuse_eq_at_another_arity(c: EqArity<'_>) {
        /// Why a position can hold `eq`: where it is written, or what passes
        /// it on from a position that can.
        enum EqFrom {
            Fact(NodeId),
            Hrules(NodeId),
            Signature(String),
            Conclusion {
                head: &'static str,
                rule: String,
                conclusion: NodeId,
            },
            Guard {
                head: &'static str,
                rule: String,
                conclusion: NodeId,
            },
            Passed {
                head: &'static str,
                rule: String,
                conclusion: NodeId,
                from: Pos,
            },
        }
        /// Where `eq` comes from, written down first.
        fn render(ast: &Ast, w: &str, at: &BTreeMap<Pos, EqFrom>, pos: &Pos) -> String {
            let dump = |n: NodeId| crate::dump::dump_compact(ast, n);
            match &at[pos] {
                EqFrom::Fact(f) => format!("the fact {}", dump(*f)),
                EqFrom::Hrules(s) => dump(*s),
                EqFrom::Signature(r) => {
                    format!("the declaration of {r}, which writes {w} as a type")
                }
                EqFrom::Conclusion {
                    head,
                    rule,
                    conclusion,
                } => format!("{head} '{rule}''s conclusion {}", dump(*conclusion)),
                EqFrom::Guard {
                    head,
                    rule,
                    conclusion,
                } => format!(
                    "an eq or neq guard of {head} '{rule}', over the program's names, {w} among \
                     them, through its conclusion {}",
                    dump(*conclusion)
                ),
                EqFrom::Passed {
                    head,
                    rule,
                    conclusion,
                    from,
                } => format!(
                    "{}, through {head} '{rule}''s conclusion {}",
                    render(ast, w, at, from),
                    dump(*conclusion)
                ),
            }
        }
        let w = c.word;
        let is_eq = |n: NodeId| matches!(c.ast.node(n), Node::Atom(s) if c.ast.sym(s) == w);
        let mut at: BTreeMap<Pos, EqFrom> = BTreeMap::new();
        // Written: a fact's argument, and a `(solve :hrules …)` activator's.
        for &f in c.facts {
            let p = positive(c.ast, f);
            let Some(rel) = relation_of(c.ast, p) else {
                continue;
            };
            for (k, &a) in c.ast.form_args(p).iter().enumerate() {
                let pos = (rel.clone(), k);
                if is_eq(a) && c.rp.contains_key(&pos) {
                    at.entry(pos).or_insert(EqFrom::Fact(f));
                }
            }
        }
        for &block in c.solve_blocks {
            for &pair in c.ast.form_args(block) {
                let Node::KwPair { key, value } = c.ast.node(pair) else {
                    continue;
                };
                if !matches!(c.ast.node(key), Node::Keyword(k) if c.ast.sym(k) == "hrules") {
                    continue;
                }
                let Some(hrule) = c.ast.head_name(value) else {
                    continue;
                };
                for &item in c.ast.form_args(value) {
                    let Node::SForm { head, .. } = c.ast.node(item) else {
                        continue;
                    };
                    let tuple = std::iter::once(head).chain(c.ast.form_args(item).iter().copied());
                    for (k, a) in tuple.enumerate() {
                        let pos = (hrule.to_string(), k);
                        if is_eq(a) && c.rp.contains_key(&pos) {
                            at.entry(pos).or_insert(EqFrom::Hrules(block));
                        }
                    }
                }
            }
        }
        // A `(relation …)` declaration's argument holds `eq` where its
        // signature writes it as a type, at its index, the name being the
        // first.
        for (name, r) in c.program.relations.iter() {
            for (j, &t) in r.signature.iter().enumerate() {
                if c.terms.sym(t) == w {
                    at.entry(("relation".to_string(), j + 1))
                        .or_insert(EqFrom::Signature(c.terms.sym(name).to_string()));
                }
            }
        }
        let eq_named = c.written.contains(w) || !at.is_empty();
        // Derived: a conclusion's argument, to a fixpoint. A variable its
        // `:match` binds where relation names are held, in some disjunct, can
        // hold `eq` where every such position can; one only a guard binds
        // holds any name of the program.
        loop {
            let mut grew = false;
            for s in c.sites {
                if !c.rp.contains_key(&s.to) || at.contains_key(&s.to) {
                    continue;
                }
                let from = if is_eq(s.arg) {
                    Some(EqFrom::Conclusion {
                        head: s.head,
                        rule: s.rule.clone(),
                        conclusion: s.conclusion,
                    })
                } else if matches!(c.ast.node(s.arg), Node::Var(_)) {
                    s.ways.iter().find_map(|(_, way)| {
                        let named: Vec<Pos> = way
                            .iter()
                            .filter_map(|o| match o {
                                Occ::Param(m) if c.rp.contains_key(&(s.rule.clone(), *m)) => {
                                    Some((s.rule.clone(), *m))
                                }
                                Occ::Arg { rel, k, .. }
                                    if rel == "relation"
                                        || c.rp.contains_key(&(rel.clone(), *k)) =>
                                {
                                    Some((rel.clone(), *k))
                                }
                                _ => None,
                            })
                            .collect();
                        let guard = way.iter().any(|o| matches!(o, Occ::Guard));
                        if named.is_empty() {
                            (guard && eq_named).then(|| EqFrom::Guard {
                                head: s.head,
                                rule: s.rule.clone(),
                                conclusion: s.conclusion,
                            })
                        } else {
                            named
                                .iter()
                                .all(|p| at.contains_key(p))
                                .then(|| EqFrom::Passed {
                                    head: s.head,
                                    rule: s.rule.clone(),
                                    conclusion: s.conclusion,
                                    from: named[0].clone(),
                                })
                        }
                    })
                } else {
                    None
                };
                if let Some(from) = from {
                    at.insert(s.to.clone(), from);
                    grew = true;
                }
            }
            if !grew {
                break;
            }
        }
        if at.is_empty() {
            return;
        }
        /// Where `?param` heads a premise under `node`: through `and`, `or`,
        /// `not` and `absent`, never inside a pattern's argument, which a
        /// nested value fills.
        fn premises(
            ast: &Ast,
            node: NodeId,
            param: &str,
            out: &mut Vec<NodeId>,
            values: &mut Vec<NodeId>,
        ) {
            let Node::SForm { head, .. } = ast.node(node) else {
                return;
            };
            match ast.head_name(node) {
                Some("and" | "or" | "not" | "absent") => {
                    for &a in ast.form_args(node) {
                        premises(ast, a, param, out, values);
                    }
                }
                _ => {
                    if matches!(ast.node(head), Node::Var(s) if ast.sym(s) == param) {
                        out.push(node);
                    }
                    for &a in ast.form_args(node) {
                        nested(ast, a, param, values);
                    }
                }
            }
        }
        /// Where `?param` heads a value under `node`: a form inside a
        /// pattern, a conclusion or a guard, at any depth, a `(not …)`'s
        /// among them.
        fn nested(ast: &Ast, node: NodeId, param: &str, values: &mut Vec<NodeId>) {
            let Node::SForm { head, .. } = ast.node(node) else {
                return;
            };
            if matches!(ast.node(head), Node::Var(s) if ast.sym(s) == param) {
                values.push(node);
            }
            for &a in ast.form_args(node) {
                nested(ast, a, param, values);
            }
        }
        let registries: [(&'static str, &ein_core::entities::Registry<Rule>); 3] = [
            ("rule", &c.program.rules),
            ("hrule", &c.program.hrules),
            ("rule", &c.program.obligations),
        ];
        for (head, registry) in registries {
            for (name, rule) in registry.iter() {
                let rule_name = c.terms.sym(name).to_string();
                for (i, &p) in rule.params.iter().enumerate() {
                    let pos = (rule_name.clone(), i);
                    if !at.contains_key(&pos) {
                        continue;
                    }
                    let param = c.terms.sym(p);
                    let mut forms: Vec<NodeId> = Vec::new();
                    let mut values: Vec<NodeId> = Vec::new();
                    if let Some(m) = rule.match_.as_ref() {
                        premises(c.ast, NodeId(m.expr.0), param, &mut forms, &mut values);
                    }
                    if let Some(a) = rule.assert_.as_ref() {
                        for conclusion in c.ast.conjuncts(NodeId(a.expr.0)) {
                            let conclusion = positive(c.ast, conclusion);
                            if matches!(c.ast.node(conclusion), Node::SForm { head, .. }
                                if matches!(c.ast.node(head), Node::Var(s) if c.ast.sym(s) == param))
                            {
                                forms.push(conclusion);
                            }
                            for &a in c.ast.form_args(conclusion) {
                                nested(c.ast, a, param, &mut values);
                            }
                        }
                    }
                    let source = render(c.ast, w, &at, &pos);
                    let binds = match at[&pos] {
                        EqFrom::Fact(_) | EqFrom::Hrules(_) => {
                            format!("{source} binds ?{param} to {w}")
                        }
                        _ => format!("?{param} can be bound to {w} by {source}"),
                    };
                    for form in forms {
                        let n = c.ast.form_args(form).len();
                        let why = match w {
                            "eq" if n == 2 => continue,
                            "eq" => {
                                "eq is the relation of equality, which holds between exactly two \
                                 values"
                            }
                            "false" if n == 0 => continue,
                            "false" => {
                                "false takes no argument: (false) says that the state is \
                                 impossible, and names nothing"
                            }
                            _ => {
                                "absent is negation as failure, a premise's test, and no fact is \
                                 of it"
                            }
                        };
                        c.errors.push(format!(
                            "{head} '{rule_name}': its parameter ?{param} heads {}, a form of {n} \
                             argument{}, and {binds} — {why}",
                            crate::dump::dump_compact(c.ast, form),
                            if n == 1 { "" } else { "s" },
                        ));
                    }
                    // A value: a fact a state could hold (M1f P1f.7's value
                    // rule), so never one of `eq` (S1f.7.10), of `absent`
                    // (S1f.7.7) or of `false` with an argument (S1f.7.6).
                    for form in values {
                        let n = c.ast.form_args(form).len();
                        if w == "false" && n == 0 {
                            continue;
                        }
                        let why = match w {
                            "eq" => "eq is computed and never stored",
                            "false" => "false takes no argument",
                            _ => "absent is negation as failure, a premise's test",
                        };
                        c.errors.push(format!(
                            "{head} '{rule_name}': its parameter ?{param} heads {}, written where \
                             a value goes, and {binds} — {why}, and a value is a name, or a fact a \
                             state could hold",
                            crate::dump::dump_compact(c.ast, form),
                        ));
                    }
                }
            }
        }
    }
}

/// **A conclusion `(not ?x)` whose variable can hold a name** — M1f S1f.7.4,
/// W84, under the value rule (the user's ruling of 2026-11-11, Q-M1f.113: *a
/// value is a name, or a fact a state could hold: an atom of an ordinary
/// relation, or its negation*). `not` of a name is the negation of no fact.
///
/// The grammar takes `(not Var)` as a conclusion, for a variable bound to a
/// fact, `(hypothesis ?h)` then `(not ?h)`; whether it is bound to one depends
/// on the data. Until S1f.7.4 a rule whose variable held a name stored `(not
/// k)`, which no program can write and whose `--dump-answer` state file did not
/// reload, and an hrule proposed it, so R held two states that differed in a
/// fact nothing can read. A written `(not k)` is a parse error, and a macro's
/// is refused by [`Shapes::negation`] (S1f.5.24, W68).
///
/// **What is read** (Q-M1f.127, the stage's default). A position, a relation
/// and an argument's index, *holds a name* where the program shows one there:
/// a written fact, at any depth and under a `(not …)`; a `(relation …)`
/// declaration; a rule's or an hrule's conclusion that writes a name, or a
/// variable that can hold one, to a fixpoint; for a rule's parameter, its
/// activator, written as a fact or in `(solve :hrules …)`; and, where the
/// program declares no hrule, every argument of a declared relation the
/// `(solve …)` lists leave among the hypotheses, since a commitment states a
/// fact of one over the program's names. A conclusion
/// variable *can hold a name* where, in one disjunct of the `:match`, every
/// relation premise that binds it binds it at a position that holds a name,
/// or nothing but a guard binds it, `(neq ?x k)` over the program's names and
/// `(eq ?x k)` to the name. A premise under `(absent …)` binds nothing.
///
/// A pattern or a conclusion whose relation is a parameter, `(?R ?x)`, is read
/// as one of each relation a written activator names there: a fact `(deny
/// paid)`, a `(solve :hrules …)` entry, or a conclusion that writes the name.
///
/// **What is not read**: such a pattern where the activator is only derived
/// through a variable, `:assert (deny ?R)`, and what the obligations rung
/// proposes of a relation held out of the hypotheses. A binding that arrives
/// that way is the firing's, which stores and proposes no negation of a name
/// (`ein_infer::firing::build_fact`, Q-M1f.128). So the check refuses only
/// what the program's own text shows, and a variable bound at a position that
/// holds facts alone loads, as the three
/// `examples/saturation/hypothesis-contradiction/` programs do, whose
/// `hypothesis` is no declared relation.
///
/// A position that holds both, `(claim (paid Bob))` beside `(claim Ann)`, is
/// refused: the rule would negate a name on some binding.
fn refuse_negated_names(
    ast: &Ast,
    terms: &Terms,
    kb: &Kb,
    facts: &[NodeId],
    solve_blocks: &[NodeId],
    errors: &mut Vec<String>,
) {
    type Pos = (String, usize);
    /// Why a position holds a name.
    enum Holds {
        Fact {
            fact: String,
            name: String,
        },
        Concluded {
            who: String,
            conclusion: String,
        },
        Activator {
            solve: String,
            name: String,
        },
        /// A declared relation the blind enumerator proposes facts of.
        Commitment {
            relation: String,
        },
    }
    /// Where a disjunct binds a variable.
    enum Occ {
        /// An argument of a pattern: the positions it is bound at, one per
        /// relation the pattern can be of (one, where its relation is a
        /// name). `premise` is `None` for the rule's own parameter.
        Arg {
            at: Vec<Pos>,
            premise: Option<NodeId>,
        },
        /// An argument of a pattern whose relation is a parameter no written
        /// activator binds.
        Unread,
    }
    /// The names the program writes at each position: what a parameter that
    /// heads a form can name, read off the rule's written activators.
    type Passed = BTreeMap<Pos, BTreeSet<String>>;
    struct Site<'a> {
        head: &'static str,
        name: String,
        rule: &'a Rule,
    }

    fn positive(ast: &Ast, node: NodeId) -> NodeId {
        if matches!(ast.node(node), Node::SForm { .. })
            && ast.head_name(node) == Some("not")
            && ast.form_args(node).len() == 1
        {
            ast.form_args(node)[0]
        } else {
            node
        }
    }
    /// The relation a form names as written, or `None` for a variable's.
    fn relation_of(ast: &Ast, node: NodeId) -> Option<String> {
        match ast.node(node) {
            Node::SForm { head, .. } => match ast.node(head) {
                Node::Var(_) => None,
                _ => ast.head_name(node).map(str::to_string),
            },
            _ => None,
        }
    }
    fn is_var(ast: &Ast, node: NodeId, var: &str) -> bool {
        matches!(ast.node(node), Node::Var(s) if ast.sym(s) == var)
    }
    fn first_loc(ast: &Ast, node: NodeId) -> Option<crate::ast::Loc> {
        if let Some(l) = ast.loc(node) {
            return Some(l);
        }
        match ast.node(node) {
            Node::SForm { head, .. } => first_loc(ast, head)
                .or_else(|| ast.form_args(node).iter().find_map(|&a| first_loc(ast, a))),
            _ => None,
        }
    }
    /// Every name a written fact holds, by position, at any depth.
    fn written(
        ast: &Ast,
        form: NodeId,
        top: NodeId,
        out: &mut BTreeMap<Pos, Holds>,
        passed: &mut Passed,
    ) {
        let form = positive(ast, form);
        let Some(rel) = relation_of(ast, form) else {
            return;
        };
        for (i, &a) in ast.form_args(form).iter().enumerate() {
            match ast.node(a) {
                Node::Atom(s) if !ein_core::text::INTERNAL_NAMES.contains(&ast.sym(s)) => {
                    passed
                        .entry((rel.clone(), i))
                        .or_default()
                        .insert(ast.sym(s).to_string());
                    out.entry((rel.clone(), i)).or_insert_with(|| Holds::Fact {
                        fact: crate::dump::dump_compact(ast, top),
                        name: ast.sym(s).to_string(),
                    });
                }
                Node::SForm { .. } => written(ast, a, top, out, passed),
                _ => {}
            }
        }
    }
    /// The relations the form `form` of `site`'s rule can be of: the one it
    /// names, or, where a parameter heads it, each name a written activator
    /// binds the parameter to. None where the parameter's activators are only
    /// derived.
    fn relations_of(
        ast: &Ast,
        terms: &Terms,
        site: &Site<'_>,
        passed: &Passed,
        form: NodeId,
    ) -> Vec<String> {
        if let Some(rel) = relation_of(ast, form) {
            return vec![rel];
        }
        let Node::SForm { head, .. } = ast.node(form) else {
            return Vec::new();
        };
        let Node::Var(v) = ast.node(head) else {
            return Vec::new();
        };
        let var = ast.sym(v);
        site.rule
            .params
            .iter()
            .enumerate()
            .filter(|(_, p)| terms.sym(**p) == var)
            .flat_map(|(m, _)| {
                passed
                    .get(&(site.name.clone(), m))
                    .into_iter()
                    .flatten()
                    .cloned()
            })
            .collect()
    }
    /// Where the pattern `form`, inside the premise `premise`, binds `var`:
    /// per occurrence, the positions it is bound at, one per relation the
    /// pattern can be of.
    fn pattern(
        ast: &Ast,
        relations: &dyn Fn(NodeId) -> Vec<String>,
        form: NodeId,
        premise: NodeId,
        var: &str,
        out: &mut Vec<Occ>,
    ) {
        let form = positive(ast, form);
        let rels = relations(form);
        for (k, &a) in ast.form_args(form).iter().enumerate() {
            match ast.node(a) {
                Node::Var(_) if is_var(ast, a, var) => out.push(if rels.is_empty() {
                    Occ::Unread
                } else {
                    Occ::Arg {
                        at: rels.iter().map(|r| (r.clone(), k)).collect(),
                        premise: Some(premise),
                    }
                }),
                Node::SForm { .. } => pattern(ast, relations, a, premise, var, out),
                _ => {}
            }
        }
    }
    /// Where the premise `node` binds `var`, and the guards that name it.
    fn premise(
        ast: &Ast,
        relations: &dyn Fn(NodeId) -> Vec<String>,
        node: NodeId,
        var: &str,
        out: &mut Vec<Occ>,
        guards: &mut Vec<NodeId>,
    ) {
        if !matches!(ast.node(node), Node::SForm { .. }) {
            return;
        }
        match ast.head_name(positive(ast, node)) {
            _ if ast.head_name(node) == Some("absent") => {}
            _ if matches!(ast.head_name(node), Some("and" | "or")) => {
                for &a in ast.form_args(node) {
                    premise(ast, relations, a, var, out, guards);
                }
            }
            Some("eq" | "neq") => {
                let mut vs = BTreeSet::new();
                vars_under(ast, node, &mut vs);
                if vs.contains(var) {
                    guards.push(node);
                }
            }
            _ => pattern(ast, relations, node, node, var, out),
        }
    }

    let program = kb.program();
    let registries: [(&'static str, &ein_core::entities::Registry<Rule>); 2] =
        [("rule", &program.rules), ("hrule", &program.hrules)];
    let mut sites: Vec<Site<'_>> = Vec::new();
    for (head, registry) in registries {
        for (name, rule) in registry.iter() {
            sites.push(Site {
                head,
                name: terms.sym(name).to_string(),
                rule,
            });
        }
    }
    // The conclusions `(not ?x)`, per rule: nothing to read where none is
    // written.
    let negated = |site: &Site<'_>| -> Vec<(NodeId, String)> {
        let Some(assert) = site.rule.assert_.as_ref() else {
            return Vec::new();
        };
        ast.conjuncts(NodeId(assert.expr.0))
            .into_iter()
            .filter(|&c| {
                matches!(ast.node(c), Node::SForm { .. }) && ast.head_name(c) == Some("not")
            })
            .filter_map(|c| match ast.form_args(c) {
                [p] => match ast.node(*p) {
                    Node::Var(s) => Some((c, ast.sym(s).to_string())),
                    _ => None,
                },
                _ => None,
            })
            .collect()
    };
    if sites.iter().all(|s| negated(s).is_empty()) {
        return;
    }

    let mut names: BTreeMap<Pos, Holds> = BTreeMap::new();
    let mut passed: Passed = BTreeMap::new();
    for &f in facts {
        written(ast, f, f, &mut names, &mut passed);
    }
    // An hrule's activators, `(solve :hrules (NAME (a b) …))`.
    for &block in solve_blocks {
        for &pair in ast.form_args(block) {
            let Node::KwPair { key, value } = ast.node(pair) else {
                continue;
            };
            if !matches!(ast.node(key), Node::Keyword(k) if ast.sym(k) == "hrules") {
                continue;
            }
            let Some(hrule) = relation_of(ast, value) else {
                continue;
            };
            for &item in ast.form_args(value) {
                let Node::SForm { head, .. } = ast.node(item) else {
                    continue;
                };
                for (i, a) in std::iter::once(head)
                    .chain(ast.form_args(item).iter().copied())
                    .enumerate()
                {
                    if let Some(name) = ast.atom_name(a) {
                        passed
                            .entry((hrule.clone(), i))
                            .or_default()
                            .insert(name.to_string());
                        names
                            .entry((hrule.clone(), i))
                            .or_insert_with(|| Holds::Activator {
                                solve: format!(
                                    "(solve :hrules {})",
                                    crate::dump::dump_compact(ast, value)
                                ),
                                name: name.to_string(),
                            });
                    }
                }
            }
        }
    }

    // A commitment's facts. With no hrule declared, the candidates are facts
    // of the declared relations the `(solve …)` lists leave in
    // (`ein_infer::hypgen::eligible_relations`), over the program's names: a
    // state can hold a name at each of their positions. Not read where the
    // program writes the closure marker, which takes a relation out of them,
    // nor where it states `(solve :no-search)`, whose lattice is root alone
    // and holds no commitment (M1f S1f.6.36).
    let closes = |node: NodeId| {
        let mut forms = Vec::new();
        sforms_under(ast, node, &mut forms);
        forms
            .iter()
            .any(|&f| ast.head_name(f) == Some(ein_core::terms::CLOSED))
    };
    let closure_written = facts.iter().any(|&f| closes(f))
        || sites.iter().any(|s| {
            s.rule
                .assert_
                .as_ref()
                .is_some_and(|a| closes(NodeId(a.expr.0)))
        });
    let no_search = solve_blocks.iter().any(|&block| {
        ast.form_args(block).iter().any(|&pair| {
            matches!(ast.node(pair), Node::KwPair { key, .. }
                if matches!(ast.node(key), Node::Keyword(k) if ast.sym(k) == "no-search"))
        })
    });
    if program.hrules.is_empty() && !closure_written && !no_search {
        let listed = |want: &str| -> Option<BTreeSet<String>> {
            let value = solve_blocks.iter().find_map(|&block| {
                ast.form_args(block).iter().find_map(|&pair| match ast.node(pair) {
                    Node::KwPair { key, value }
                        if matches!(ast.node(key), Node::Keyword(k) if ast.sym(k) == want) =>
                    {
                        Some(value)
                    }
                    _ => None,
                })
            })?;
            let nodes: Vec<NodeId> = match ast.node(value) {
                Node::SForm { head, .. } => std::iter::once(head)
                    .chain(ast.form_args(value).iter().copied())
                    .collect(),
                _ => vec![value],
            };
            Some(
                nodes
                    .into_iter()
                    .filter_map(|n| ast.atom_name(n).map(str::to_string))
                    .collect(),
            )
        };
        let allowed = listed("hypothesis-relations");
        let excluded = listed("no-hypothesis").unwrap_or_default();
        for (name, relation) in program.relations.iter() {
            let name = terms.sym(name).to_string();
            if !relation.declared
                || excluded.contains(&name)
                || allowed.as_ref().is_some_and(|a| !a.contains(&name))
            {
                continue;
            }
            for i in 0..relation.signature.len() {
                names
                    .entry((name.clone(), i))
                    .or_insert_with(|| Holds::Commitment {
                        relation: name.clone(),
                    });
            }
        }
    }

    // How `var` can hold a name in `site`'s rule: the occurrence that shows
    // it, or the guard, as the line says it.
    let can_name = |names: &BTreeMap<Pos, Holds>,
                    passed: &Passed,
                    site: &Site<'_>,
                    var: &str|
     -> Option<String> {
        let relations = |form: NodeId| relations_of(ast, terms, site, passed, form);
        let shown = |pos: &Pos, premise: Option<NodeId>| -> Option<String> {
            let v = format!("?{var}");
            if pos.0 == "relation" {
                let p = premise?;
                return Some(format!(
                    "{} binds {v} where a declaration names a relation or a type",
                    crate::dump::dump_compact(ast, p)
                ));
            }
            let holds = names.get(pos)?;
            Some(match (premise, holds) {
                (Some(p), Holds::Fact { fact, name }) => format!(
                    "{} binds {v} where the fact {fact} holds {name}",
                    crate::dump::dump_compact(ast, p)
                ),
                (Some(p), Holds::Concluded { who, conclusion }) => format!(
                    "{} binds {v} where {who} writes {conclusion}",
                    crate::dump::dump_compact(ast, p)
                ),
                (Some(p), Holds::Activator { solve, name }) => format!(
                    "{} binds {v} where {solve} names {name}",
                    crate::dump::dump_compact(ast, p)
                ),
                (Some(p), Holds::Commitment { relation }) => format!(
                    "{} binds {v} where a commitment can hold a name, {relation} being a \
                     declared relation the hypotheses are taken from",
                    crate::dump::dump_compact(ast, p)
                ),
                (None, Holds::Commitment { relation }) => format!(
                    "{v} is its parameter, which a commitment's fact of {relation} binds to a name"
                ),
                (None, Holds::Fact { fact, name }) => {
                    format!("{v} is its parameter, which the fact {fact} binds to {name}")
                }
                (None, Holds::Concluded { who, conclusion }) => {
                    format!("{v} is its parameter, which {who} binds by writing {conclusion}")
                }
                (None, Holds::Activator { solve, name }) => {
                    format!("{v} is its parameter, which {solve} binds to {name}")
                }
            })
        };
        let by_param: Vec<Occ> = site
            .rule
            .params
            .iter()
            .enumerate()
            .filter(|(_, p)| terms.sym(**p) == var)
            .map(|(m, _)| Occ::Arg {
                at: vec![(site.name.clone(), m)],
                premise: None,
            })
            .collect();
        let whole = site
            .rule
            .match_
            .as_ref()
            .map(|m| NodeId(m.expr.0))
            .filter(|&m| matches!(ast.node(m), Node::SForm { .. }));
        let disjuncts: Vec<Option<NodeId>> = match whole {
            Some(m) if ast.head_name(m) == Some("or") => {
                ast.form_args(m).iter().map(|&d| Some(d)).collect()
            }
            other => vec![other],
        };
        for d in disjuncts {
            let mut occs: Vec<Occ> = by_param
                .iter()
                .map(|o| match o {
                    Occ::Arg { at, premise } => Occ::Arg {
                        at: at.clone(),
                        premise: *premise,
                    },
                    Occ::Unread => Occ::Unread,
                })
                .collect();
            let mut guards: Vec<NodeId> = Vec::new();
            if let Some(d) = d {
                premise(ast, &relations, d, var, &mut occs, &mut guards);
            }
            if occs.is_empty() {
                // Nothing but a guard binds it: `neq` over the program's
                // names, `eq` to its other side.
                for g in guards {
                    let p = positive(ast, g);
                    let negated = p != g;
                    let is_eq = (ast.head_name(p) == Some("eq")) != negated;
                    let other_is_name = ast
                        .form_args(p)
                        .iter()
                        .any(|&a| matches!(ast.node(a), Node::Atom(_)));
                    let text = crate::dump::dump_compact(ast, g);
                    if !is_eq {
                        return Some(format!(
                            "{text} binds ?{var} over the program's names, nothing else binding it"
                        ));
                    }
                    if other_is_name {
                        return Some(format!("{text} binds ?{var} to a name"));
                    }
                }
                continue;
            }
            let mut first: Option<String> = None;
            let all = occs.iter().all(|o| match o {
                Occ::Unread => false,
                Occ::Arg { at, premise } => match at.iter().find_map(|pos| shown(pos, *premise)) {
                    Some(how) => {
                        first.get_or_insert(how);
                        true
                    }
                    None => false,
                },
            });
            if all && let Some(how) = first {
                return Some(how);
            }
        }
        None
    };

    // What each conclusion writes: a name, or a variable that can hold one,
    // to a fixpoint.
    fn concluded(
        ast: &Ast,
        relations: &dyn Fn(NodeId) -> Vec<String>,
        form: NodeId,
        top: NodeId,
        who: &str,
        can: &dyn Fn(&str) -> bool,
        out: &mut Vec<(Pos, Holds, Option<String>)>,
    ) {
        let form = positive(ast, form);
        let rels: Vec<String> = relations(form)
            .into_iter()
            .filter(|r| !matches!(r.as_str(), "eq" | "neq" | "open" | "false"))
            .collect();
        for (i, &a) in ast.form_args(form).iter().enumerate() {
            let (names, name) = match ast.node(a) {
                Node::Atom(s) if !ein_core::text::INTERNAL_NAMES.contains(&ast.sym(s)) => {
                    (true, Some(ast.sym(s).to_string()))
                }
                Node::Var(s) => (can(ast.sym(s)), None),
                Node::SForm { .. } => {
                    concluded(ast, relations, a, top, who, can, out);
                    (false, None)
                }
                _ => (false, None),
            };
            if names {
                for rel in &rels {
                    out.push((
                        (rel.clone(), i),
                        Holds::Concluded {
                            who: who.to_string(),
                            conclusion: crate::dump::dump_compact(ast, top),
                        },
                        name.clone(),
                    ));
                }
            }
        }
    }
    loop {
        let mut found: Vec<(Pos, Holds, Option<String>)> = Vec::new();
        for site in &sites {
            let Some(assert) = site.rule.assert_.as_ref() else {
                continue;
            };
            let who = format!("{} '{}'", site.head, site.name);
            for c in ast.conjuncts(NodeId(assert.expr.0)) {
                concluded(
                    ast,
                    &|form| relations_of(ast, terms, site, &passed, form),
                    c,
                    c,
                    &who,
                    &|var| can_name(&names, &passed, site, var).is_some(),
                    &mut found,
                );
            }
        }
        let mut grew = false;
        for (pos, holds, name) in found {
            // A name a conclusion writes where an activator stands is one its
            // rule's parameter can name.
            if let Some(name) = name {
                grew |= passed.entry(pos.clone()).or_default().insert(name);
            }
            if let std::collections::btree_map::Entry::Vacant(e) = names.entry(pos) {
                e.insert(holds);
                grew = true;
            }
        }
        if !grew {
            break;
        }
    }

    for site in &sites {
        for (node, var) in negated(site) {
            let Some(how) = can_name(&names, &passed, site, &var) else {
                continue;
            };
            let place = match first_loc(ast, node) {
                Some(loc) => format!("{}: ", loc_repr(ast, Some(loc))),
                None => String::new(),
            };
            errors.push(format!(
                "{place}{} '{}': its :assert writes {}, and ?{var} can be a name: {how} — not takes \
                 a pattern (relation argument …) or a variable bound to a fact: it is the stored \
                 negation of one fact, and a name is no fact; negation as failure is (absent …)",
                site.head,
                site.name,
                crate::dump::dump_compact(ast, node),
            ));
        }
    }
}

/// **A written fact of `eq` at another arity** — M1f S1f.5.9, W61, on the
/// user's ruling of 2026-11-07 that `eq` is the relation of equality between
/// two values (Q-M1e2.26), taken by default on the user's instruction of
/// 2026-11-09 (Q-M1f.47).
///
/// `ingest_fact` reads a written fact of `eq`, or of `(not (eq …))`, as a
/// conclusion with no premise, checked and never stored (Q-M1f.33): at two
/// arguments. At any other, `(eq a)`, `(eq)`, `(eq a a a)` or `(not (eq a))`, it
/// is no fact of equality, true or false, and the grammar refuses `(eq ?x)` as
/// a premise and a conclusion; until S1f.5.9 it was stored as an ordinary fact
/// (W61). One line per fact, `; `-joined with the loader's others. A nested
/// `(box (eq a))` was a value until M1f S1f.7.10, as `(box (eq a b))` was, and
/// loaded; since, `eq` heads no value, and each is a parse error.
fn refuse_eq_facts_of_another_arity(ast: &Ast, facts: &[NodeId], errors: &mut Vec<String>) {
    let terms_of = |node: NodeId| -> Vec<NodeId> {
        ast.form_args(node)
            .iter()
            .copied()
            .filter(|&a| !matches!(ast.node(a), Node::KwPair { .. }))
            .collect()
    };
    for &f in facts {
        let sides = match ast.head_name(f) {
            Some("eq") => terms_of(f),
            Some("not") => match terms_of(f).first() {
                Some(&inner) if ast.head_name(inner) == Some("eq") => terms_of(inner),
                _ => continue,
            },
            _ => continue,
        };
        if sides.len() != 2 {
            let n = sides.len();
            errors.push(format!(
                "the fact {} writes eq with {n} argument{} — eq is the relation of equality, which \
                 holds between exactly two values",
                crate::dump::dump_compact(ast, f),
                if n == 1 { "" } else { "s" }
            ));
        }
    }
}

/// What each `(relation …)` form declares: its name, and its types, whose
/// count is the count of arguments a form of the relation has. The first
/// declaration of a name, a second being refused as a duplicate; a reserved
/// name, refused as a declaration, declares nothing.
type Declared = BTreeMap<String, Vec<String>>;

fn declared_counts(ast: &Ast, relations: &[NodeId]) -> Declared {
    let mut out = Declared::new();
    for &form in relations {
        let args = ast.form_args(form);
        let Some(name) = args.first().and_then(|&a| ast.atom_name(a)) else {
            continue;
        };
        if is_reserved(name) {
            continue;
        }
        let types: Vec<String> = args[1..]
            .iter()
            .filter_map(|&a| ast.atom_name(a).map(str::to_string))
            .collect();
        out.entry(name.to_string()).or_insert(types);
    }
    out
}

/// **A form headed by a declared relation has as many arguments as the
/// declaration has types** — M1f
/// [S1f.6.23](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.23_the_loaders_arity_check.md)
/// (W89), on the user's ruling of 2026-11-12 on question 3 of P1f.6's
/// `questions.md`, *"(a)"*: *every form headed by a declared relation, at any
/// depth, in a fact, a rule or a query, refused at load with one line naming
/// the relation, the count declared and the count written.*
///
/// Until S1f.6.23 no form was held to its relation's declaration. `(likes
/// Bob)` beside `(relation likes Person Drink)` was stored; matching is by
/// count, so a premise `(likes ?who)` matched no fact of the declared count
/// and a conclusion `(fan ?who ?what)` derived facts no premise of it read;
/// and the calculator's `(add (n z 5))`, one operand where `add` declares
/// two, and a `std.math.num9` number of eight digits were answered by silence,
/// exit 0 (E-2 of P1f.6's `findings.md`).
///
/// This collects every such form at or under `node`, in the order written: a
/// form whose head is the name of a relation the program declares, its own
/// declaration or an imported one (an import is resolved before the loader
/// reads a form, so `(import M :as o)`'s relation is `o.likes`), and whose
/// arguments, its keyword pairs aside (`:source`, `:rule`, `:using`), are not
/// as many as the declaration's types. A bare `(relation flag)` declares
/// none, so `(flag)` is its one form (Q-M1f.201). It descends into every
/// argument and every keyword pair's value, so a fact inside a fact, under
/// `(not …)` and `(absent …)`, and in a fact's `:using` are read.
///
/// **What it does not read**: a relation nothing declares, which has no count
/// to be held to, the kernel's own among them, `__closed__`, `__symmetric__`
/// and the `(relation R T…)` fact a declaration stores, read at any count
/// (Q-M1f.203); a relation written as a variable, `(?R ?a ?b)`, whatever the
/// program shows its parameter holds (Q-M1f.202); and an activator against
/// its rule's parameters, which is no declared relation's count. It is no
/// type check: what is written in an argument's place is not held to the
/// type declared for it. The callers hand it a fact, a rule's expanded
/// clause, a goal and a claim's listed fact, and never a `(solve …)` list,
/// which is a list of names and no form.
fn wrong_counts(ast: &Ast, node: NodeId, declared: &Declared, out: &mut Vec<NodeId>) {
    match ast.node(node) {
        Node::SForm { .. } => {
            if let Some(types) = ast.head_name(node).and_then(|name| declared.get(name))
                && written_count(ast, node) != types.len()
            {
                out.push(node);
            }
            for &a in ast.form_args(node) {
                wrong_counts(ast, a, declared, out);
            }
        }
        Node::KwPair { value, .. } => wrong_counts(ast, value, declared, out),
        _ => {}
    }
}

/// A form's arguments, its keyword pairs aside.
fn written_count(ast: &Ast, form: NodeId) -> usize {
    ast.form_args(form)
        .iter()
        .filter(|&&a| !matches!(ast.node(a), Node::KwPair { .. }))
        .count()
}

/// What every line about a declared relation's count ends with.
const DECLARED_COUNT: &str =
    "a form headed by a declared relation has as many arguments as its declaration has types";

/// `gives likes 1 argument, and (relation likes Person Drink) declares 2 — …`,
/// of a form [`wrong_counts`] found.
fn count_line(ast: &Ast, form: NodeId, declared: &Declared) -> String {
    let name = ast
        .head_name(form)
        .expect("a form wrong_counts found is headed by a name");
    let types = &declared[name];
    let declaration = if types.is_empty() {
        format!("(relation {name})")
    } else {
        format!("(relation {name} {})", types.join(" "))
    };
    format!(
        "gives {name} {}, and {declaration} declares {} — {DECLARED_COUNT}",
        count(written_count(ast, form), "argument"),
        types.len()
    )
}

/// [`wrong_counts`] over a rule's, an hrule's or a query's clause, one line a
/// form, at the form: `<place>rule 'fans': its :match writes (likes ?who),
/// which gives likes 1 argument, and (relation likes Person Drink) declares 2
/// — …`. `verb` is `writes`, or `lists` for a claim's fact.
fn refuse_wrong_counts(
    ast: &Ast,
    who: &str,
    clause: &str,
    verb: &str,
    node: NodeId,
    declared: &Declared,
    errors: &mut Vec<String>,
) {
    let mut found = Vec::new();
    wrong_counts(ast, node, declared, &mut found);
    for form in found {
        let subject = if form == node || verb == "writes" {
            crate::dump::dump_compact(ast, form)
        } else {
            format!(
                "{}, which holds {}",
                crate::dump::dump_compact(ast, node),
                crate::dump::dump_compact(ast, form)
            )
        };
        errors.push(format!(
            "{}{who}: its {clause} {verb} {subject}, which {}",
            placed_form(ast, form),
            count_line(ast, form, declared)
        ));
    }
}

/// [`wrong_counts`] over the program's written facts: `<place>the fact (likes
/// Bob) gives likes 1 argument, and …`, and for a fact inside one, `<place>the
/// fact (add (n 1) (n z 5)) holds (n 1), which gives n 1 argument, and …`.
fn refuse_wrong_counts_in_facts(
    ast: &Ast,
    facts: &[NodeId],
    declared: &Declared,
    errors: &mut Vec<String>,
) {
    for &fact in facts {
        let mut found = Vec::new();
        wrong_counts(ast, fact, declared, &mut found);
        for form in found {
            let subject = if form == fact {
                format!("the fact {}", crate::dump::dump_compact(ast, fact))
            } else {
                format!(
                    "the fact {} holds {}, which",
                    crate::dump::dump_compact(ast, fact),
                    crate::dump::dump_compact(ast, form)
                )
            };
            errors.push(format!(
                "{}{subject} {}",
                placed_form(ast, form),
                count_line(ast, form, declared)
            ));
        }
    }
}

/// [`wrong_counts`] over each query's `:goal`, as expanded, and over the facts
/// a claim lists, a query's `:expect` and the `(solve … :expect …)`: a claim's
/// own words, `model`, `region`, `merged`, `owing` and `or`, are its shape
/// and no relation's form, so only what a `(model …)` lists is read.
fn refuse_wrong_counts_in_queries(
    ast: &Ast,
    kb: &Kb,
    declared: &Declared,
    errors: &mut Vec<String>,
) {
    for query in &kb.program().queries {
        if let Some(goal) = query_keyword(ast, query, "goal") {
            refuse_wrong_counts(ast, "query", ":goal", "writes", goal, declared, errors);
        }
        if let Some(node) = query_keyword(ast, query, "expect") {
            for model in crate::expect::parse(ast, node).models() {
                for &fact in &model.facts {
                    refuse_wrong_counts(ast, "query", ":expect", "lists", fact, declared, errors);
                }
            }
        }
    }
    let claim = kb
        .program()
        .solve()
        .and_then(|solve| solve_keyword(ast, &solve.kw_pairs, "expect"))
        .and_then(|node| crate::claim::parse(ast, node).ok());
    if let Some(crate::claim::Claim::Region(region)) = claim {
        let models = region
            .models
            .iter()
            .chain(region.merged.iter().map(|m| &m.to))
            .chain(region.merged.iter().flat_map(|m| m.from.iter()))
            .chain(region.owing.iter());
        for model in models {
            for &fact in &model.facts {
                refuse_wrong_counts(ast, "solve", ":expect", "lists", fact, declared, errors);
            }
        }
    }
}

/// **A macro's call is a premise, a conclusion or a goal, and never a fact** —
/// M1f [S1f.7.12](../../../../plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/s1f.7.12_a_macro_call_written_as_a_fact.md)
/// (W86).
///
/// The loader expands a macro's call in a rule's or an hrule's `:match` and
/// `:assert` and, since S1f.7.5, in a query's `:goal`. Until S1f.7.12 a fact
/// headed by a macro's name, `(pp j)` beside `(macro pp (?a) (p ?a))`, was
/// stored as a fact of a relation `pp`: the fact the body spells was not, no
/// premise could read the one stored, since `(pp ?x)` in a `:match` is the
/// call, and nothing said so. The same held at any depth, `(q (pp j))`, under
/// `(not …)` and in a fact's `:using`, of an imported `std.macro` macro,
/// `(unknown (p j))`, and of one the program did not import, which a rule's
/// `:match` has been refused for since S1.8a.f20.
///
/// Refused here, one line per call at the call (the stage's default (a)), and
/// with it a `(relation …)` declared under a macro's name, of which no fact
/// could be written and no premise read (Q-M1f.131). A call is not expanded
/// into the fact its body spells: a body with a connective spells none, and
/// the macro would have a second meaning.
fn refuse_macro_facts(
    ast: &Ast,
    relations: &[NodeId],
    facts: &[NodeId],
    declared: &BTreeMap<String, IrMacro>,
    stdlib_macros: &[String],
    errors: &mut Vec<String>,
) {
    for &form in relations {
        let Some(&first) = ast.form_args(form).first() else {
            continue;
        };
        if let Some(name) = ast.atom_name(first)
            && let Some(m) = declared.get(name)
        {
            errors.push(format!(
                "{}relation '{name}': {name} is a macro too, {} — a premise ({name} …) is the \
                 macro's call, replaced by its body, and a fact ({name} …) is refused, so nothing \
                 could be of the relation; give one of the two another name",
                placed_form(ast, form),
                macro_as_written(ast, m),
            ));
        }
    }
    for &fact in facts {
        for (call, what) in macro_calls_under(ast, fact, declared, stdlib_macros) {
            let subject = if call == fact {
                format!(
                    "the fact {} is a call of {what}",
                    crate::dump::dump_compact(ast, fact)
                )
            } else {
                format!(
                    "the fact {} holds {}, a call of {what}",
                    crate::dump::dump_compact(ast, fact),
                    crate::dump::dump_compact(ast, call)
                )
            };
            errors.push(format!(
                "{}{subject} — {MACRO_CALL_IS_NO_FACT}",
                placed_form(ast, call)
            ));
        }
    }
}

/// [`placed`], reading the form's head and then its arguments where the parser
/// kept no position for the form itself, as it keeps none for a top-level one.
fn placed_form(ast: &Ast, node: NodeId) -> String {
    fn first(ast: &Ast, node: NodeId) -> Option<crate::ast::Loc> {
        ast.loc(node).or_else(|| match ast.node(node) {
            Node::SForm { head, .. } => {
                first(ast, head).or_else(|| ast.form_args(node).iter().find_map(|&a| first(ast, a)))
            }
            _ => None,
        })
    }
    match first(ast, node) {
        Some(l) => format!("{}: ", loc_repr(ast, Some(l))),
        None => String::new(),
    }
}

/// What every line about a macro's call written as a fact ends with.
const MACRO_CALL_IS_NO_FACT: &str = "a macro is called in a :match, an :assert or a :goal, where \
     its call is replaced by its body; a fact is not expanded, and no fact is of a macro: write \
     the fact itself";

/// A claim that lists a macro's call — M1f S1f.7.12. A claim lists facts
/// (Q-M1f.118), so the call is expanded in none: one inside a listed fact,
/// `(model (q (pp j)))`, loaded and the claim failed against a state holding
/// `(q (p j))`, the call *no class*. Refused as a written fact's is.
fn refuse_macro_claims(
    ast: &Ast,
    kb: &Kb,
    declared: &BTreeMap<String, IrMacro>,
    stdlib_macros: &[String],
    errors: &mut Vec<String>,
) {
    let mut claims: Vec<(&str, NodeId)> = Vec::new();
    for query in &kb.program().queries {
        if let Some(node) = query_keyword(ast, query, "expect") {
            claims.push(("query", node));
        }
    }
    if let Some(solve) = kb.program().solve()
        && let Some(node) = solve_keyword(ast, &solve.kw_pairs, "expect")
    {
        claims.push(("solve", node));
    }
    for (who, claim) in claims {
        for (call, what) in macro_calls_under(ast, claim, declared, stdlib_macros) {
            errors.push(format!(
                "{}{who}: its :expect lists {}, a call of {what} — a claim lists facts, and \
                 {MACRO_CALL_IS_NO_FACT}",
                placed_form(ast, call),
                crate::dump::dump_compact(ast, call),
            ));
        }
    }
}

/// `(macro pp (?a) (p ?a))`, a macro as its declaration writes it, its `:why`
/// aside.
fn macro_as_written(ast: &Ast, m: &IrMacro) -> String {
    let params: Vec<String> = m.params.iter().map(|p| format!("?{p}")).collect();
    format!(
        "(macro {} ({}) {})",
        m.name,
        params.join(" "),
        crate::dump::dump_compact(ast, m.body)
    )
}

/// Every form under `node`, `node` included, whose head is a macro's name, in
/// source order, each with what the macro is: one the program declares or
/// imports, as written, or `std.macro`'s where the program imports none of
/// that name. A call's own arguments are not read: the line is the call's.
fn macro_calls_under(
    ast: &Ast,
    node: NodeId,
    declared: &BTreeMap<String, IrMacro>,
    stdlib_macros: &[String],
) -> Vec<(NodeId, String)> {
    fn walk(
        ast: &Ast,
        node: NodeId,
        declared: &BTreeMap<String, IrMacro>,
        stdlib_macros: &[String],
        out: &mut Vec<(NodeId, String)>,
    ) {
        match ast.node(node) {
            Node::SForm { .. } => {
                if let Some(name) = ast.head_name(node) {
                    if let Some(m) = declared.get(name) {
                        out.push((
                            node,
                            format!("the macro {name}, {}", macro_as_written(ast, m)),
                        ));
                        return;
                    }
                    if stdlib_macros.iter().any(|n| n == name) {
                        out.push((
                            node,
                            format!("std.macro's macro {name}, which the program does not import"),
                        ));
                        return;
                    }
                }
                for &a in ast.form_args(node) {
                    walk(ast, a, declared, stdlib_macros, out);
                }
            }
            Node::KwPair { value, .. } => walk(ast, value, declared, stdlib_macros, out),
            _ => {}
        }
    }
    let mut out = Vec::new();
    walk(ast, node, declared, stdlib_macros, &mut out);
    out
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
/// **Where `open` stands is the grammar's since M1f S1f.5.23** (the user's
/// ruling on Q-M1f.66, *"Refuse it"*): `open` is no name, and `ein.y`'s
/// `Assert` is the one production that takes it, `(open)` or `(open ?R)`, a
/// rule's whole `:assert`, its argument a variable or a relation's name. So no
/// `(open …)` stands in a `:match`, inside a conclusion, beside another
/// conclusion or with two arguments, written or through a macro, whose body and
/// arguments write no `open` either; this pass refused each of those four, and
/// an argument that was neither a variable nor a name, until then
/// (`grammar_pages`' `DELETED`, the grammar's table holding each), and it holds
/// that as an invariant now. What it still refuses is what only the program can
/// tell, the projection:
///
/// 1. **`(open ?R)` whose `?R` is not a parameter** — a relation variable is
///    bound by the activator, never by a premise, so it could never resolve;
/// 2. **a projection that does not resolve** — `(open ?R)` names the relation
///    whose extent is incomplete and the engine reads the rest out of the
///    rule's own `absent`: exactly one `absent` holding a positive `?R`
///    premise, and exactly one such premise bearing a variable the guard does
///    not already bind. None, two, or a ground body is refused rather than
///    guessed.
///
/// Each line starts with the position the `(open …)` is written at, the
/// spelling M1f S1f.5.21 gave a refusal; until S1f.5.23 each ended `at None`
/// (F23's D-2), a rule being a form the parser keeps no position for.
///
/// [S1d.2.3]: `docs/history/m1d_satisfiability/README.md#s1d23--the-form`
fn validate_open(
    ast: &Ast,
    head: &str,
    name: &str,
    params: &[String],
    match_node: NodeId,
    assert_node: NodeId,
    errors: &mut Vec<String>,
) -> bool {
    let mut forms = Vec::new();
    open_forms(ast, match_node, &mut forms);
    open_forms(ast, assert_node, &mut forms);
    let whole = is_open_form(ast, assert_node);
    assert!(
        forms.len() == usize::from(whole) && (!whole || ast.form_args(assert_node).len() <= 1),
        "{head} '{name}': an (open …) stands where the grammar writes none — ein.y's Assert is \
         the one production that takes open, a rule's whole :assert, (open) or (open ?R) \
         (M1f S1f.5.23)"
    );
    if !whole {
        return false;
    }
    let who = format!("{}{head} '{name}'", placed(ast, assert_node));
    let Some(&arg) = ast.form_args(assert_node).first() else {
        return true; // bare `(open)` — countable, nothing to project.
    };
    let (rel, is_var) = match ast.node(arg) {
        Node::Var(s) => (ast.sym(s).to_string(), true),
        Node::Atom(s) => (ast.sym(s).to_string(), false),
        _ => unreachable!(
            "{who}: (open …)'s argument is a variable or a name (ein.y's Assert, AnyHead)"
        ),
    };
    // A variable relation head is bound by the activator, never by a
    // premise (`compile.rs`: "M1 matches relations per activator"), so an
    // `(open ?R)` whose `?R` is not a parameter could never resolve.
    if is_var && !params.iter().any(|p| p == &rel) {
        errors.push(format!(
            "{who}: `(open ?{rel})` names a relation the activator does not bind — `?{rel}` is \
             not in the parameter list"
        ));
        return true;
    }
    validate_projection(ast, &who, &rel, is_var, match_node, errors);
    true
}

/// `<file>:<line>:<col>: `, where `node` is written, or nothing where the
/// parser kept no position for it.
fn placed(ast: &Ast, node: NodeId) -> String {
    match ast.loc(node) {
        Some(l) => format!("{}: ", loc_repr(ast, Some(l))),
        None => String::new(),
    }
}

/// (2) — resolve the witness step out of the rule's own `absent`, or refuse.
/// `who` is the line's start, the position and the rule.
fn validate_projection(
    ast: &Ast,
    who: &str,
    rel: &str,
    is_var: bool,
    match_node: NodeId,
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
            "{who}: `(open {shown})` needs an `(absent …)` in :match holding a positive \
             `{shown}` premise — that premise is the witness the obligation owes"
        ));
        return;
    }
    if holders.len() > 1 {
        errors.push(format!(
            "{who}: {} `(absent …)` guards hold a positive `{shown}` premise — which one states \
             the obligation is not decidable",
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
            "{who}: `(open {shown})`'s `{shown}` premise binds no variable of its own, so the \
             obligation is ground — that is a plain `absent` check and not something a witness \
             could discharge"
        )),
        n => errors.push(format!(
            "{who}: {n} positive `{shown}` premises each bind a witness variable — a compound \
             witness has no single slot to branch on"
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

    // **A fact of `eq`, or the negation of one, is checked, never stored**
    // (M1f S1f.5.7): `eq` and `neq` are relations whose facts are computed,
    // and no fact changes them (the user's ruling of 2026-11-07 on
    // Q-M1e2.26). One that holds adds nothing; one that does not is
    // `(false)`, written where it is, as a rule's false conclusion of one is
    // (G-21) — read as a conclusion with no premise (Q-M1f.31 and Q-M1f.33,
    // taken by default on the user's instruction of 2026-11-09). Until then it
    // was an ordinary fact, which no guard reads (W50, W51). `(neq …)` is no
    // fact the grammar takes.
    let mut head_name = head_name;
    let mut args = args;
    let computed: Option<(bool, Vec<NodeId>)> = match head_name.as_str() {
        "eq" => Some((true, args.clone())),
        "not" => args
            .iter()
            .find(|&&a| !matches!(ast.node(a), Node::KwPair { .. }))
            .filter(|&&a| ast.head_name(a) == Some("eq"))
            .map(|&a| (false, ast.form_args(a).to_vec())),
        _ => None,
    };
    if let Some((eq, sides)) = computed {
        let values = fact_args(ast, terms, &sides)?;
        if values.len() == 2 {
            if (values[0] == values[1]) == eq {
                return Ok(());
            }
            head_name = "false".to_string();
            args.retain(|&a| matches!(ast.node(a), Node::KwPair { .. }));
        }
    }

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
        let src = "(relation is-a T T :why \"{?1} is a {?2}\")\n(relation r A B :why \"r holds of {?1} and {?2}\")\n\
                   (rule total-owed (?R ?isa)\n  \
                     :match (and (relation ?R ?A ?B) (?isa ?a ?A)\n              \
                                 (absent (and (?isa ?b ?B) (?R ?a ?b))))\n  \
                     :assert (open ?R) :why \"{?a} has no {?R} partner of type {?B} yet — one is still owed\")\n";
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
        let (terms, kb) = ok(
            "(relation r A B :why \"r holds of {?1} and {?2}\")\n(rule owes () :match (absent (r a b)) :assert (open) :why \"r does not hold of a and b yet — that fact is still owed\")\n",
        );
        assert!(
            kb.program()
                .obligations
                .contains(terms.syms.get("owes").expect("interned")),
            "`(open)` is countable and routes the same way"
        );

        // An ordinary rule is untouched by any of it.
        let (terms, kb) = ok(
            "(relation r A B :why \"r holds of {?1} and {?2}\")\n(rule copy () :match (r ?a ?b) :assert (r ?b ?a) :why \"r holds of {?a} and {?b}, so r holds of {?b} and {?a}\")\n",
        );
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
        let pre = "(relation n T :why \"n holds of {?1}\") (relation hit T :why \"hit holds of {?1}\") (relation box T :why \"box holds of {?1}\") (relation tag T T :why \"tag holds of {?1} and {?2}\")\n(n 2)\n";
        ok(&format!(
            "{pre}(rule r () :match (and (n ?u) (box (tag ?v ?w))) :assert (hit r) :why \"n holds of {{?u}} and box holds of (tag {{?v}} {{?w}}), so hit holds of r\")"
        ));
        ok(&format!(
            "{pre}(rule owe () :match (and (n ?x) (absent (tag ?x ?any))) :assert (open tag) :why \"n holds of {{?x}} and no tag of {{?x}} holds yet — one is still owed\")"
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
        let pre = "(relation p T :why \"p holds of {?1}\") (relation q T :why \"q holds of {?1}\") (relation hit T :why \"hit holds of {?1}\") (relation box T :why \"box holds of {?1}\") (relation tag T T :why \"tag holds of {?1} and {?2}\")\n\
                   (relation item T :why \"{?1} is an item\") (relation lbl T :why \"lbl holds of {?1}\")\n(p a) (q a) (box (p a))\n";
        // At load: what a context-free grammar does not see.
        for (rule, want) in [
            (
                "(rule e () :match (and (p ?x) (eq ?x (p ?y))) :assert (hit ?x) :why \"p holds of {?x}, which equals a p fact, so hit holds of {?x}\")".to_string(),
                format!(
                    "rule 'e': its :match writes (eq ?x (p ?y)), and the form (p ?y) holds a \
                     variable — {GUARD}"
                ),
            ),
            (
                // Under an `absent`, and at any depth.
                "(rule e () :match (and (p ?x) (absent (neq ?x (box (p ?x))))) :assert (hit ?x) :why \"p holds of {?x} and {?x} equals (box (p {?x})), so hit holds of {?x}\")"
                    .to_string(),
                format!(
                    "rule 'e': its :match writes (neq ?x (box (p ?x))), and the form (box (p ?x)) \
                     holds a variable — {GUARD}"
                ),
            ),
            // A relation variable no parameter binds, inside a pattern and at
            // the top level, in either clause.
            (
                "(rule r () :match (box (?r a)) :assert (hit a) :why \"box holds of ({?r} a), so hit holds of a\")".to_string(),
                format!("rule 'r': its :match writes (?r a) with ?r as its relation, {FREE}"),
            ),
            (
                "(rule r () :match (and (p ?x) (absent (not (?r ?x)))) :assert (hit ?x) :why \"p holds of {?x} and nothing is stored false of {?x}, so hit holds of {?x}\")"
                    .to_string(),
                format!("rule 'r': its :match writes (?r ?x) with ?r as its relation, {FREE}"),
            ),
            (
                "(rule r () :match (and (p ?r) (?r a)) :assert (hit a) :why \"p holds of {?r} and {?r} holds of a, so hit holds of a\")".to_string(),
                format!("rule 'r': its :match writes (?r a) with ?r as its relation, {FREE}"),
            ),
            (
                "(rule r () :match (p ?r) :assert (box (?r a)) :why \"p holds of {?r}, so box holds of ({?r} a)\")".to_string(),
                format!("rule 'r': its :assert writes (?r a) with ?r as its relation, {FREE}"),
            ),
            (
                "(hrule h () :match (p ?r) :assert (?r a) :why \"p holds of {?r}: guess that {?r} holds of a\")".to_string(),
                format!("hrule 'h': its :assert writes (?r a) with ?r as its relation, {FREE}"),
            ),
            // An obligation's `:match`.
            (
                "(rule owe () :match (and (item ?x) (box (?r a)) (absent (tag ?x ?l))) \
                 :assert (open tag) :why \"item {?x} has no tag yet — one is still owed\")"
                    .to_string(),
                format!("rule 'owe': its :match writes (?r a) with ?r as its relation, {FREE}"),
            ),
        ] {
            assert_eq!(err(&format!("{pre}{rule}")), want, "{rule}");
        }
        // A relation parameter its activator binds to a nested fact, where the
        // activator is written: a fact, or a literal in an `:assert`.
        let mirror = "(rule mirror (?R) :match (box (?R a)) :assert (hit a) :why \"box holds of ({?R} a), so hit holds of a\")";
        for activator in [
            "(mirror (p a))".to_string(),
            "(rule mk () :match (q ?x) :assert (mirror (p a)) :why \"q holds of {?x}, so rule mirror is switched on for (p a)\")".to_string(),
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
             :assert (hit ?x) :why \"p holds of {?x} and q of {?y}, {?x} is not {?y} or b and is a and 1, so hit holds of {?x}\")",
            "(rule e () :match (and (p ?x) (absent (and (q ?y) (neq ?y ?x)))) :assert (hit ?x) :why \"p holds of {?x} and q holds of nothing other than {?x}, so hit holds of {?x}\")",
            "(rule e () :match (and (p ?x) (eq ?x a)) :assert (hit ?x) :why \"p holds of {?x}, which is a, so hit holds of {?x}\")",
            "(rule e () :match (and (box (p ?u)) (p ?v)) :assert (hit a) :why \"box holds of (p {?u}) and p holds of {?v}, so hit holds of a\")",
            "(rule e (?R) :match (box (?R a)) :assert (hit a) :why \"box holds of ({?R} a), so hit holds of a\")",
            "(rule e () :match (and (p ?x) (eq ?x (p a))) :assert (hit ?x) :why \"p holds of {?x}, which is the fact (p a), so hit holds of {?x}\")",
            "(rule e () :match (and (neq ?x a) (p ?x)) :assert (hit ?x) :why \"p holds of {?x}, which is not a, so hit holds of {?x}\")",
            "(rule e () :match (and (p ?x) (eq ?x ?u)) :assert (hit ?u) :why \"p holds of {?x} and {?u} equals it, so hit holds of {?u}\")",
            "(rule e () :match (and (p ?x) (absent (and (neq ?y a) (q ?y)))) :assert (hit ?x) :why \"p holds of {?x} and q holds of nothing but a, so hit holds of {?x}\")",
        ] {
            ok(&format!("{pre}{rule}"));
        }
    }

    /// **The compiler's three refusals a program reached, at load** — M1f
    /// S1f.5.21 (W62, W63, W64; the close review's finding 1). An `(or …)`
    /// written as a premise, of an `(and …)`, an `(absent …)` or a top-level
    /// `(or …)`, in a rule's, an hrule's or a goal's clause, written or
    /// produced by a macro (Q-M1f.74); a `:match` or a `:goal` past the
    /// register file, a rule's parameters counted (Q-M1f.73); and a goal's
    /// premise whose relation is a variable, under `and`, `or` and `absent`.
    /// Then what loads: a top-level `(or …)`, an `(or …)` inside a pattern's
    /// argument (no premise), a rule at the ceiling, a goal with its relation
    /// named, and a macro's `(or …)` called as a whole `:match`.
    #[test]
    fn the_compilers_refusals_a_program_reached_are_refused_at_load() {
        let pre = "(relation a T :why \"a holds of {?1}\") (relation p T :why \"p holds of {?1}\") \
                   (relation q T :why \"q holds of {?1}\") (relation out T :why \"out holds of {?1}\")\n(a k)\n";
        let nested = |who: &str, clause: &str, within: &str, or: &str, split: &str| {
            format!(
                "{who}: its {clause} writes {or} as a premise of {within} — an (or …) is a \
                 whole {clause}, each of its premises one way it holds: lift it to the top of \
                 the {clause}, or {split}"
            )
        };
        let r = |w: &str, or: &str| nested("rule 'r'", ":match", w, or, "split the rule");
        let g = |w: &str| nested("query", ":goal", w, "(or (p ?x) (q ?x))", "ask two queries");
        let or = "(or (p ?x) (q ?x))";
        for (text, want) in [
            (
                "(rule r () :match (and (a ?x) (or (p ?x) (q ?x))) :assert (out ?x) :why \"r\")".to_string(),
                r("and", or),
            ),
            (
                "(rule r () :match (and (a ?x) (absent (or (p ?x) (q ?x)))) :assert (out ?x) :why \"r\")".to_string(),
                r("absent", or),
            ),
            (
                "(rule r () :match (or (or (p ?x) (q ?x)) (a ?x)) :assert (out ?x) :why \"r\")".to_string(),
                r("or", or),
            ),
            (
                "(rule r (?R) :match (and (?R ?x) (or (p ?x) (q ?x))) :assert (out ?x) :why \"r\")".to_string(),
                r("and", or),
            ),
            (
                "(hrule h () :match (and (a ?x) (or (p ?x) (q ?x))) :assert (out ?x) :why \"h\")".to_string(),
                nested("hrule 'h'", ":match", "and", or, "split the rule"),
            ),
            (
                "(macro either (?x) (or (p ?x) (q ?x)) :why \"either\")\n\
                 (rule r () :match (and (a ?x) (either ?x)) :assert (out ?x) :why \"r\")".to_string(),
                r("and", or),
            ),
            (
                "(import std.macro :symbols (forall))\n\
                 (rule r () :match (and (a ?x) (forall ?x (or (p ?x) (q ?x)) (a ?x))) :assert (out ?x) :why \"r\")".to_string(),
                r("and", or),
            ),
            ("(query :goal (and (a ?x) (or (p ?x) (q ?x))))".to_string(), g("and")),
            ("(query :goal (absent (or (p ?x) (q ?x))))".to_string(), g("absent")),
            (
                "(query :goal (?R k))".to_string(),
                "query: its :goal writes (?R k) with ?R as its relation, which nothing binds — a \
                 relation is named by a name, or by a rule's parameter its activator binds, and \
                 a query has no activator"
                    .to_string(),
            ),
            (
                "(query :goal (and (a ?x) (absent (?R ?x))))".to_string(),
                "query: its :goal writes (?R ?x) with ?R as its relation, which nothing binds — a \
                 relation is named by a name, or by a rule's parameter its activator binds, and \
                 a query has no activator"
                    .to_string(),
            ),
        ] {
            assert_eq!(err(&format!("{pre}{text}")), want, "{text}");
        }
        // The register ceiling: 257 variables, a parameter counted, and a goal.
        let wide = |n: usize| {
            (0..n)
                .map(|i| format!("(p ?v{i})"))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let at = MAX_MATCH_VARIABLES;
        assert_eq!(
            err(&format!(
                "{pre}(rule w () :match (and {}) :assert (out ?v0) :why \"w\")",
                wide(at + 1)
            )),
            "rule 'w': its :match writes 257 distinct variables, and a rule binds at most 256 — \
             split the rule"
        );
        assert_eq!(
            err(&format!(
                "{pre}(rule w (?R) :match (and {}) :assert (out ?v0) :why \"w\")",
                wide(at)
            )),
            "rule 'w': its :match writes 257 distinct variables, its parameters among them, and a \
             rule binds at most 256 — split the rule"
        );
        assert_eq!(
            err(&format!("{pre}(query :goal (and {}))", wide(at + 1))),
            "query: its :goal writes 257 distinct variables, and a goal binds at most 256 — ask \
             two queries"
        );
        // What loads.
        for text in [
            "(rule r () :match (or (p ?x) (q ?x)) :assert (out ?x) :why \"r\")".to_string(),
            "(rule r () :match (or (and (a ?x) (p ?x)) (q ?x)) :assert (out ?x) :why \"r\")"
                .to_string(),
            format!(
                "(rule w () :match (and {}) :assert (out ?v0) :why \"w\")",
                wide(at)
            ),
            "(query :goal (or (p ?x) (q ?x)))".to_string(),
            "(query :goal (and (a ?x) (p ?x)))".to_string(),
            format!("(query :goal (and {}))", wide(at)),
            "(macro either (?x) (or (p ?x) (q ?x)) :why \"either\")\n\
             (rule r () :match (either ?x) :assert (out ?x) :why \"r\")"
                .to_string(),
            "(macro either (?x) (or (p ?x) (q ?x)) :why \"either, used by no rule\")".to_string(),
        ] {
            ok(&format!("{pre}{text}"));
        }
    }

    /// **A macro's expansion is held to the grammar's shapes** — M1f S1f.5.24,
    /// on the user's ruling on Q-M1f.67, W66 to W69.
    ///
    /// Each shape a macro's body or a call's argument can malform, in a rule's,
    /// an hrule's and an obligation's `:match` and as a conclusion, refused at
    /// load with one line naming the rule, the clause and the expanded form,
    /// the position where it is written first. Then what loads: an unused
    /// malformed macro (the ruling), and every shape the grammar takes where a
    /// rule writes it, written and through a macro, so the walk refuses no
    /// written rule the grammar accepts.
    #[test]
    fn a_macros_expansion_is_held_to_the_grammars_shapes() {
        let pre = "(relation a T :why \"a holds of {?1}\") (relation p T :why \"p holds of {?1}\") \
                   (relation q T :why \"q holds of {?1}\")\n(a k)\n";
        let m = [
            "(macro one (?a) (eq ?a) :why \"one\")",
            "(macro app (?R ?a) (?R ?a) :why \"app\")",
            "(macro two (?a ?b) (absent ?a ?b) :why \"two\")",
            "(macro app2 (?R ?a ?b) (?R ?a ?b) :why \"app2\")",
            "(macro neg (?a) (not ?a) :why \"neg\")",
            "(macro diff (?a ?b) (neq ?a ?b) :why \"diff\")",
            "(macro bad (?a) (false ?a) :why \"bad\")",
            "(macro either (?a) (or (p ?a) (q ?a)) :why \"either\")",
            "(macro wrap (?a) (q (and (p ?a))) :why \"wrap\")",
            "(macro id (?x) ?x :why \"id\")",
            "(import std.macro :symbols (unknown))",
        ]
        .join("\n");
        let prem = |call: &str| {
            [
                format!("(rule r () :match (and (a ?x) {call}) :assert (p ?x) :why \"r\")"),
                format!("(rule r (?R) :match (and (?R ?x) {call}) :assert (p ?x) :why \"r\")"),
                format!("(hrule r () :match (and (a ?x) {call}) :assert (p ?x) :why \"r\")"),
                format!("(rule r () :match (and (a ?x) {call}) :assert (open) :why \"r\")"),
            ]
        };
        let concl = |call: &str| {
            [
                format!("(rule r () :match (a ?x) :assert {call} :why \"r\")"),
                format!("(rule r (?R) :match (?R ?x) :assert {call} :why \"r\")"),
                format!("(hrule r () :match (a ?x) :assert {call} :why \"r\")"),
            ]
        };
        let refusals: Vec<(Vec<String>, &str)> = vec![
            (
                prem("(one ?x)").to_vec(),
                ":match writes (eq ?x), and eq takes exactly 2 arguments, not 1",
            ),
            (
                prem("(app eq ?x)").to_vec(),
                ":match writes (eq ?x), and eq takes exactly 2 arguments, not 1",
            ),
            (
                prem("(two (p ?x) (q ?x))").to_vec(),
                ":match writes (absent (p ?x) (q ?x)), and absent takes exactly 1 argument, not 2",
            ),
            (
                prem("(app2 absent (p ?x) (q ?x))").to_vec(),
                ":match writes (absent (p ?x) (q ?x)), and absent takes exactly 1 argument, not 2",
            ),
            (
                prem("(absent (neg (and (p ?x) (q ?x))))").to_vec(),
                ":match writes (not (and (p ?x) (q ?x))), and not takes a pattern (relation \
                 argument …), not (and (p ?x) (q ?x))",
            ),
            (
                prem("(unknown (and (p ?x) (q ?x)))").to_vec(),
                ":match writes (not (and (p ?x) (q ?x))), and not takes a pattern",
            ),
            (
                prem("(neg ?x)").to_vec(),
                ":match writes (not ?x), and not takes a pattern (relation argument …), not ?x",
            ),
            (
                prem("(diff ?x (and (p k)))").to_vec(),
                ":match writes (neq ?x (and (p k))), and (and (p k)) is a connective where a value \
                 goes",
            ),
            (
                concl("(one ?x)").to_vec(),
                ":assert writes (eq ?x), and eq takes exactly 2 arguments, not 1",
            ),
            (
                concl("(app eq ?x)").to_vec(),
                ":assert writes (eq ?x), and eq takes exactly 2 arguments, not 1",
            ),
            (
                concl("(bad ?x)").to_vec(),
                ":assert writes (false ?x), and false takes no argument, not 1",
            ),
            (
                concl("(app false ?x)").to_vec(),
                ":assert writes (false ?x), and false takes no argument, not 1",
            ),
            (
                concl("(either ?x)").to_vec(),
                ":assert writes (or (p ?x) (q ?x)), and an (or …) is no conclusion",
            ),
            (
                concl("(neg k)").to_vec(),
                ":assert writes (not k), and not takes a pattern (relation argument …) or a \
                 variable, not k",
            ),
            (
                concl("(wrap ?x)").to_vec(),
                ":assert writes (q (and (p ?x))), and (and (p ?x)) is a connective where a value goes",
            ),
            (
                concl("(id ?x)").to_vec(),
                ":assert concludes ?x, which is not a fact",
            ),
            (
                concl("(id k)").to_vec(),
                ":assert concludes k, which is not a fact",
            ),
        ];
        for (texts, want) in &refusals {
            for text in texts {
                let got = err(&format!("{pre}{m}\n{text}"));
                let who = if text.starts_with("(hrule") {
                    "hrule 'r'"
                } else {
                    "rule 'r'"
                };
                assert!(
                    got.lines().count() == 1 && got.contains(&format!("{who}: its {want}")),
                    "{text}\nwants {who}: its {want}\ngot {got}"
                );
            }
        }
        // The position is where the expanded form is written: the macro's body
        // for a form it writes, the call's argument for one the grammar builds.
        assert!(
            err(&format!(
                "{pre}{m}\n(rule r () :match (a ?x) :assert (one ?x) :why \"r\")"
            ))
            .starts_with("<string>:3:18: rule 'r': its :assert writes (eq ?x)"),
        );
        // What loads: every macro above, used by no rule; and every shape the
        // grammar takes where a rule writes it, written and through a macro.
        ok(&format!("{pre}{m}"));
        // Seven shapes loaded here until M1f S1f.7.7, S1f.7.6, S1f.7.10 and
        // S1f.7.3: a value is a name, or a fact a state could hold, so
        // `absent`, `eq`, a connective and `false` with an argument head none,
        // and no conclusion is of `absent`. The grammar refuses five written
        // out, and the walk the two it cannot tell from a macro's call.
        for (text, want) in [
            (
                "(rule r () :match (and (a ?x) (absent (not (absent (p ?x))))) :assert (p ?x) :why \"r\")",
                None,
            ),
            (
                "(rule r () :match (and (a ?x) (eq ?x (absent (p k)))) :assert (p ?x) :why \"r\")",
                None,
            ),
            (
                "(rule r () :match (and (a ?x) (false ?x)) :assert (p ?x) :why \"r\")",
                None,
            ),
            (
                "(rule r () :match (a ?x) :assert (and (p ?x) (q (eq ?x k))) :why \"r\")",
                None,
            ),
            (
                "(rule r () :match (a ?x) :assert (absent (p ?x)) :why \"r\")",
                None,
            ),
            (
                "(rule r () :match (and (a ?x) (p (eq k k))) :assert (p ?x) :why \"r\")",
                Some(":match writes (p (eq k k)), and (eq k k) is written where a value goes"),
            ),
            (
                "(rule r () :match (and (a ?x) (p (and (q ?x)))) :assert (p ?x) :why \"r\")",
                Some(
                    ":match writes (p (and (q ?x))), and (and (q ?x)) is a connective where a value goes",
                ),
            ),
        ] {
            let src = format!("{pre}{text}");
            match want {
                None => assert!(
                    crate::parse(&mut Ast::new(), &src, Some("<string>")).is_err(),
                    "parses: {text}"
                ),
                Some(want) => {
                    let got = err(&src);
                    assert!(got.contains(want), "{text}: {got}");
                }
            }
        }
        for text in [
            "(rule r () :match (and (a ?x) (not (eq ?x k))) :assert (p ?x) :why \"r\")",
            "(rule r () :match (and (a ?x) (p (not (q k)))) :assert (p ?x) :why \"r\")",
            "(rule r () :match (and (a ?x) (absent (false))) :assert (p ?x) :why \"r\")",
            // `(not ?x)` of a variable bound to a fact: `h` holds no name.
            // With `(a ?x)`, which `(a k)` binds to a name, it is refused
            // since M1f S1f.7.4 (`a_conclusion_negating_a_name_is_refused_at_load`).
            "(h (p k))\n(rule r () :match (h ?x) :assert (not ?x) :why \"r\")",
            "(rule r () :match (a ?x) :assert (not (p ?x)) :why \"r\")",
            "(rule r () :match (a ?x) :assert (and (p ?x) (not (eq ?x k))) :why \"r\")",
            "(rule r () :match (a ?x) :assert (neq ?x k) :why \"r\")",
            "(rule r () :match (a ?x) :assert (false) :why \"r\")",
            "(rule r () :match () :assert () :why \"r\")",
            "(rule r (?R) :match (and (a ?x) (absent (?R ?x ?y))) :assert (open ?R) :why \"r\")",
            "(rule r () :match (and (a ?x) (one ?x ?x)) :assert (p ?x) :why \"r\")",
            "(rule r () :match (and (a ?x) (app q ?x)) :assert (p ?x) :why \"r\")",
            "(rule r () :match (and (a ?x) (two (p ?x))) :assert (p ?x) :why \"r\")",
            "(rule r () :match (and (a ?x) (absent (neg (p ?x)))) :assert (p ?x) :why \"r\")",
            "(rule r () :match (and (a ?x) (unknown (p ?x))) :assert (p ?x) :why \"r\")",
            "(rule r () :match (and (a ?x) (diff ?x k)) :assert (p ?x) :why \"r\")",
            "(h (p k))\n(rule r () :match (h ?x) :assert (neg ?x) :why \"r\")",
            "(rule r () :match (a ?x) :assert (id (p ?x)) :why \"r\")",
        ] {
            let (one, two) = (
                "(macro one (?a ?b) (eq ?a ?b) :why \"one\")",
                "(macro two (?a) (absent ?a) :why \"two\")",
            );
            let rest = m
                .lines()
                .filter(|l| !l.starts_with("(macro one") && !l.starts_with("(macro two"))
                .collect::<Vec<_>>()
                .join("\n");
            ok(&format!("{pre}{one}\n{two}\n{rest}\n{text}"));
        }
    }

    /// **A conclusion `(not ?x)` whose variable can hold a name is refused at
    /// load** — M1f S1f.7.4, W84, under the value rule (Q-M1f.113, Q-M1f.127).
    ///
    /// Each way the program shows a name where the variable is bound: a
    /// written fact, at the top and inside another fact; a declaration; a
    /// parameter's activator, written and in `(solve :hrules …)`; another
    /// rule's conclusion, a name and a variable that can hold one; a guard
    /// alone; one disjunct of two; a macro's `(not ?a)`; an hrule's; a pattern
    /// and a conclusion whose relation is a parameter, through its written
    /// activator; and a declared relation the hypotheses are taken from. Then
    /// what loads: a position that holds facts alone, two premises of which
    /// one holds no name, a variable bound through the relation of a parameter
    /// whose activator is derived, a
    /// premise under `(absent …)` beside one that holds facts, a guard beside
    /// a premise that holds facts, and a declared relation no commitment
    /// states a fact of: held out by a `(solve …)` list, beside an hrule, or
    /// closed.
    #[test]
    fn a_conclusion_negating_a_name_is_refused_at_load() {
        let pre =
            "(relation a T :why \"a holds of {?1}\") (relation p T :why \"p holds of {?1}\")\n";
        let not = "(rule r () :match (a ?x) :assert (not ?x) :why \"r\")";
        let not_m = "(rule r () :match (m ?x) :assert (not ?x) :why \"r\")";
        for (text, want) in [
            (
                format!("(a k)\n{not}"),
                "rule 'r': its :assert writes (not ?x), and ?x can be a name: (a ?x) binds ?x \
                 where the fact (a k) holds k — not takes a pattern (relation argument …) or a \
                 variable bound to a fact: it is the stored negation of one fact, and a name is \
                 no fact; negation as failure is (absent …)",
            ),
            (
                format!("(says (a k))\n{not}"),
                "(a ?x) binds ?x where the fact (says (a k)) holds k",
            ),
            (
                format!("(not (a k))\n{not}"),
                "(a ?x) binds ?x where the fact (not (a k)) holds k",
            ),
            (
                format!("(a (p k)) (a j)\n{not}"),
                "(a ?x) binds ?x where the fact (a j) holds j",
            ),
            (
                "(rule r () :match (relation ?x ?t) :assert (not ?x) :why \"r\")".to_string(),
                "(relation ?x ?t) binds ?x where a declaration names a relation or a type",
            ),
            (
                "(r k)\n(rule r (?x) :match () :assert (not ?x) :why \"r\")".to_string(),
                "?x is its parameter, which the fact (r k) binds to k",
            ),
            (
                "(hrule r (?x) :match () :assert (not ?x) :why \"r\")\n(solve :hrules (r (k)))"
                    .to_string(),
                "hrule 'r': its :assert writes (not ?x), and ?x can be a name: ?x is its \
                 parameter, which (solve :hrules (r (k))) binds to k",
            ),
            // `m` is no declared relation: what a rule writes there shows it.
            (
                format!("(rule mk () :match () :assert (m k) :why \"mk\")\n{not_m}"),
                "(m ?x) binds ?x where rule 'mk' writes (m k)",
            ),
            (
                format!(
                    "(p k)\n(rule mk () :match (p ?y) :assert (seen (m ?y)) :why \"mk\")\n{not_m}"
                ),
                "(m ?x) binds ?x where rule 'mk' writes (seen (m ?y))",
            ),
            (
                format!("(p k)\n(hrule mk () :match (p ?y) :assert (m ?y) :why \"mk\")\n{not_m}"),
                "(m ?x) binds ?x where hrule 'mk' writes (m ?y)",
            ),
            (
                "(rule r () :match (neq ?x k) :assert (not ?x) :why \"r\")".to_string(),
                "(neq ?x k) binds ?x over the program's names, nothing else binding it",
            ),
            (
                "(rule r () :match (eq ?x k) :assert (not ?x) :why \"r\")".to_string(),
                "(eq ?x k) binds ?x to a name",
            ),
            (
                "(a k) (h (p k))\n\
                 (rule r () :match (or (h ?x) (a ?x)) :assert (not ?x) :why \"r\")"
                    .to_string(),
                "(a ?x) binds ?x where the fact (a k) holds k",
            ),
            (
                "(a k)\n(macro neg (?v) (not ?v) :why \"neg\")\n\
                 (rule r () :match (a ?x) :assert (and (p ?x) (neg ?x)) :why \"r\")"
                    .to_string(),
                "rule 'r': its :assert writes (not ?x), and ?x can be a name",
            ),
            (
                "(a k)\n(hrule r () :match (a ?x) :assert (not ?x) :why \"r\")".to_string(),
                "hrule 'r': its :assert writes (not ?x), and ?x can be a name",
            ),
            // A pattern whose relation is a parameter, read as each relation
            // a written activator names; and a name written through one.
            (
                "(a k) (r a)\n(rule r (?R) :match (?R ?x) :assert (not ?x) :why \"r\")".to_string(),
                "(?R ?x) binds ?x where the fact (a k) holds k",
            ),
            (
                format!(
                    "(p k) (mk m)\n(rule mk (?R) :match (p ?y) :assert (?R ?y) :why \"mk\")\n{not_m}"
                ),
                "(m ?x) binds ?x where rule 'mk' writes (?R ?y)",
            ),
            (
                format!(
                    "(p k)\n(rule on () :match (p ?y) :assert (mk m) :why \"on\")\n\
                     (rule mk (?R) :match (p ?y) :assert (?R ?y) :why \"mk\")\n{not_m}"
                ),
                "(m ?x) binds ?x where rule 'mk' writes (?R ?y)",
            ),
            // No fact of `a` is written: a commitment states one.
            (
                not.to_string(),
                "(a ?x) binds ?x where a commitment can hold a name, a being a declared \
                 relation the hypotheses are taken from",
            ),
            (
                format!("{not}\n(solve :hypothesis-relations (a))"),
                "(a ?x) binds ?x where a commitment can hold a name",
            ),
        ] {
            let got = err(&format!("{pre}{text}"));
            assert!(got.contains(want), "{text}: {got}");
        }
        for text in [
            "(h (p k))\n(rule r () :match (h ?x) :assert (not ?x) :why \"r\")",
            "(a k) (h (p k))\n(rule r () :match (and (a ?y) (h ?x)) :assert (not ?x) :why \"r\")",
            "(a k) (h (p k))\n(rule r () :match (and (a ?x) (h ?x)) :assert (not ?x) :why \"r\")",
            // A parameter's relation whose activator is derived through a
            // variable is not read: the firing's (Q-M1f.128).
            "(a k) (marks a)\n(rule on () :match (marks ?R) :assert (r ?R) :why \"on\")\n\
             (rule r (?R) :match (?R ?x) :assert (not ?x) :why \"r\")",
            "(a k) (h (p k))\n\
             (rule r () :match (and (h ?x) (absent (a ?x))) :assert (not ?x) :why \"r\")",
            "(h (p k))\n(rule r () :match (and (h ?x) (neq ?x k)) :assert (not ?x) :why \"r\")",
            "(h (p k))\n(rule r () :match (and (h ?x) (eq ?y ?x)) :assert (not ?y) :why \"r\")",
            "(r (p k))\n(rule r (?x) :match () :assert (not ?x) :why \"r\")",
            "(a k)\n(rule r () :match (a ?x) :assert (not (p ?x)) :why \"r\")",
            // A declared relation the hypotheses are not taken from.
            "(a (p k))\n(solve :no-hypothesis (a))\n\
             (rule r () :match (a ?x) :assert (not ?x) :why \"r\")",
            "(a (p k))\n(solve :hypothesis-relations (p))\n\
             (rule r () :match (a ?x) :assert (not ?x) :why \"r\")",
            "(a (p k))\n(hrule g () :match (p ?y) :assert (p ?y) :why \"g\")\n\
             (rule r () :match (a ?x) :assert (not ?x) :why \"r\")",
            "(a (p k)) (__closed__ a)\n(rule r () :match (a ?x) :assert (not ?x) :why \"r\")",
        ] {
            ok(&format!("{pre}{text}"));
        }
    }

    /// **A conclusion's every variable is bound, and is refused at load where
    /// it is not** — M1f S1f.5.10, on the user's ruling of 2026-11-07 on
    /// P1f.5's F13 (*"yes, file F13 with the refusal"*). Each shape the
    /// stage file names, one line per variable naming the rule, the variable
    /// and the two explicit spellings: a free variable, an `(or …)` arm, a
    /// variable seen only under an `(absent …)`, and an hrule; and a guard's
    /// under an `(absent …)`, a nested conclusion's, an empty `:match`'s, two
    /// variables in one conclusion and a `forall`'s. Then what binds, and
    /// loads: a parameter, a relation premise at any depth, a stored `(not
    /// P)`, an `eq` or `neq` guard (M1f S1f.5.7), the acceptance's typed
    /// premise, and an `(or …)` whose every arm binds.
    #[test]
    fn a_conclusion_variable_nothing_binds_is_refused_at_load() {
        let line = |head: &str, name: &str, var: &str, place: &str| {
            format!(
                "{head} '{name}': its :assert writes ?{var}, which no premise or parameter \
                 binds{place} — to conclude it of every object of a type, bind it with a \
                 premise such as (is-a ?{var} T), or of every name of the program with \
                 (eq ?{var} ?{var})"
            )
        };
        let pre = "(relation p T :why \"p holds of {?1}\") (relation q T :why \"q holds of {?1}\") (relation hit T :why \"hit holds of {?1}\") (relation box T :why \"box holds of {?1}\") \
                   (relation tag T T :why \"tag holds of {?1} and {?2}\")\n(p a) (q b)\n";
        for (rule, want) in [
            // A free variable.
            (
                "(rule r () :match (p ?x) :assert (q ?y) :why \"p holds of {?x}, so q holds of a value no premise binds\")",
                line("rule", "r", "y", ""),
            ),
            // An `(or …)` arm that leaves it unbound.
            (
                "(rule r () :match (or (p ?x) (q ?y)) :assert (hit ?x) :why \"p or q holds of a value, so hit holds of the value p holds of\")",
                line("rule", "r", "x", " where its :match holds through (q ?y)"),
            ),
            // Seen only under an `(absent …)`, and under a guard inside one.
            (
                "(rule r () :match (and (p ?x) (absent (q ?y))) :assert (hit ?y) :why \"p holds of {?x} and q of nothing, so hit holds of a value no premise binds\")",
                line("rule", "r", "y", ""),
            ),
            (
                "(rule r () :match (and (p ?x) (absent (eq ?y a))) :assert (hit ?y) :why \"p holds of {?x} and nothing equals a, so hit holds of a value no premise binds\")",
                line("rule", "r", "y", ""),
            ),
            // An hrule.
            (
                "(hrule h () :match (p ?x) :assert (q ?y) :why \"p holds of {?x}: guess that q holds of a value no premise binds\")",
                line("hrule", "h", "y", ""),
            ),
            // Nested in a conclusion's argument, and an empty `:match` (F12).
            (
                "(rule r () :match (p ?x) :assert (box (tag ?x ?y)) :why \"p holds of {?x}, so box holds of a tag of {?x} and a value no premise binds\")",
                line("rule", "r", "y", ""),
            ),
            (
                "(rule r () :match () :assert (q ?y) :why \"with no premise, q holds of a value nothing binds\")",
                line("rule", "r", "y", ""),
            ),
        ] {
            assert_eq!(err(&format!("{pre}{rule}")), want, "{rule}");
        }
        // Two variables, one line each, in the order the `:assert` writes
        // them; and a `forall`'s bound variable, which its guard reads alone.
        assert_eq!(
            err(&format!(
                "{pre}(rule r () :match (p ?x) :assert (tag ?z ?y) :why \"p holds of {{?x}}, so tag holds of two values no premise binds\")"
            )),
            format!(
                "{}; {}",
                line("rule", "r", "z", ""),
                line("rule", "r", "y", "")
            )
        );
        assert_eq!(
            err(&format!(
                "(import std.macro :symbols (forall))\n{pre}\
                 (rule r () :match (and (p ?x) (forall ?y (q ?y) (p ?y))) :assert (tag ?x ?y) :why \"p holds of {{?x}} and of every q value, so tag holds of {{?x}} and a value no premise binds\")"
            )),
            line("rule", "r", "y", "")
        );
        // What binds, and loads.
        for rule in [
            "(rule r (?y) :match (p ?x) :assert (tag ?x ?y) :why \"p holds of {?x}, so tag holds of {?x} and {?y}\")",
            "(rule r () :match (box (tag ?x ?y)) :assert (tag ?y ?x) :why \"box holds of (tag {?x} {?y}), so tag holds of {?y} and {?x}\")",
            "(rule r () :match (not (p ?x)) :assert (hit ?x) :why \"p is stored false of {?x}, so hit holds of {?x}\")",
            "(rule r () :match (and (p ?x) (eq ?y ?y)) :assert (q ?y) :why \"p holds of {?x} and {?y} is any name of the program, so q holds of {?y}\")",
            "(rule r () :match (and (p ?x) (neq ?y ?x)) :assert (tag ?x ?y) :why \"p holds of {?x} and {?y} is another name, so tag holds of {?x} and {?y}\")",
            "(rule r () :match (and (p ?x) (not (eq ?y ?x))) :assert (tag ?x ?y) :why \"p holds of {?x} and {?y} is not {?x}, so tag holds of {?x} and {?y}\")",
            "(rule r () :match (and (p ?x) (is-a ?y House)) :assert (q ?y) :why \"p holds of {?x} and {?y} is a House, so q holds of {?y}\")",
            "(rule r () :match (or (p ?x) (q ?x)) :assert (hit ?x) :why \"p or q holds of {?x}, so hit holds of {?x}\")",
            "(rule r () :match (and (p ?x) (absent (q ?x))) :assert (hit ?x) :why \"p holds of {?x} and q does not, so hit holds of {?x}\")",
            "(hrule h () :match (p ?x) :assert (q ?x) :why \"p holds of {?x}: guess that q holds of {?x}\")",
            "(rule r () :match () :assert (q a) :why \"q holds of a, with no premise\")",
        ] {
            ok(&format!("{pre}{rule}"));
        }
    }

    /// **A rule that could derive an activator binding a relation parameter
    /// to a fact is refused at load** — M1f S1f.5.18, on the user's ruling of
    /// 2026-11-08 on Q-M1f.1 (b), [`refuse_derived_relation_activators`]:
    /// each shape the stage file names, a variable from a value position and
    /// a fact written where a relation is held, and the ones the reading of
    /// Q-M1f.41 (b) and Q-M1f.42 (a) adds; and what loads, the zebra2 and
    /// stdlib fan-outs among them.
    #[test]
    fn a_derived_activator_binding_a_relation_parameter_to_a_fact_is_refused_at_load() {
        const NAMED: &str = "a relation is named by a symbol, not a fact";
        let r = "(rule r (?R) :match (box (?R b)) :assert (hit z) :why \"box holds of ({?R} b), so hit holds of z\")";
        let r_param = "rule 'r''s parameter ?R, which names a relation";
        let pre = "(relation box Node :why \"box holds of {?1}\") (relation hit Node :why \"hit holds of {?1}\") (relation item Node :why \"{?1} is an item\") (relation q T T :why \"q holds of {?1} and {?2}\")\n";
        for (src, want) in [
            // A variable from a declared relation's argument: `g14`'s two
            // programs, with the fact and without it.
            (
                format!(
                    "{pre}{r}\n(rule mk () :match (item ?f) :assert (r ?f) :why \"{{?f}} is an item, so rule r is switched on for {{?f}}\")\n(item (a b))"
                ),
                format!(
                    "rule 'mk': its :assert writes (r ?f), whose ?f is {r_param}, and (item ?f) \
                     can bind ?f to a fact, as an argument of the declared relation item — {NAMED}"
                ),
            ),
            (
                format!(
                    "{pre}{r}\n(rule mk () :match (item ?f) :assert (r ?f) :why \"{{?f}} is an item, so rule r is switched on for {{?f}}\")"
                ),
                format!(
                    "rule 'mk': its :assert writes (r ?f), whose ?f is {r_param}, and (item ?f) \
                     can bind ?f to a fact, as an argument of the declared relation item — {NAMED}"
                ),
            ),
            // A variable inside a fact pattern, and one under a relation a
            // parameter names; an hrule derives too.
            (
                format!(
                    "{pre}{r}\n(rule mk () :match (box (g ?f)) :assert (r ?f) :why \"box holds of (g {{?f}}), so rule r is switched on for {{?f}}\")"
                ),
                format!(
                    "rule 'mk': its :assert writes (r ?f), whose ?f is {r_param}, and (box (g \
                     ?f)) can bind ?f to a fact, as an argument of (g ?f) — {NAMED}"
                ),
            ),
            (
                format!(
                    "{pre}{r}\n(rule mk (?S) :match (?S ?f) :assert (r ?f) :why \"{{?S}} holds of {{?f}}, so rule r is switched on for {{?f}}\")\n(mk item)"
                ),
                format!(
                    "rule 'mk': its :assert writes (r ?f), whose ?f is {r_param}, and (?S ?f) \
                     can bind ?f to a fact, as an argument of the relation ?S names — {NAMED}"
                ),
            ),
            (
                format!(
                    "{pre}{r}\n(hrule mk () :match (item ?f) :assert (r ?f) :why \"{{?f}} is an item: guess that rule r is switched on for {{?f}}\")"
                ),
                format!(
                    "hrule 'mk': its :assert writes (r ?f), whose ?f is {r_param}, and (item ?f) \
                     can bind ?f to a fact, as an argument of the declared relation item — {NAMED}"
                ),
            ),
            // A marker passes a relation name on: a fact written there, a
            // literal written there, and a variable from a value position
            // written there, each refused through the marker.
            (
                format!(
                    "{pre}{r}\n(rule fan () :match (marker ?m) :assert (r ?m) :why \"the marker names {{?m}}, so rule r is switched on for {{?m}}\")\n(marker (a b))"
                ),
                format!(
                    "the fact (marker (a b)) writes (a b) as ?m of (marker ?m), which rule 'fan' \
                     passes to {r_param} — {NAMED}"
                ),
            ),
            (
                format!(
                    "{pre}{r}\n(rule fan () :match (marker ?m) :assert (r ?m) :why \"the marker names {{?m}}, so rule r is switched on for {{?m}}\")\n\
                     (rule mk () :match (item ?x) :assert (marker (a b)) :why \"{{?x}} is an item, so the marker names (a b)\")\n(item c)"
                ),
                format!(
                    "rule 'mk': its :assert writes (marker (a b)), whose (a b) is ?m of (marker \
                     ?m), which rule 'fan' passes to {r_param} — {NAMED}"
                ),
            ),
            (
                format!(
                    "{pre}{r}\n(rule fan () :match (marker ?m) :assert (r ?m) :why \"the marker names {{?m}}, so rule r is switched on for {{?m}}\")\n\
                     (rule mk () :match (item ?x) :assert (marker ?x) :why \"{{?x}} is an item, so the marker names {{?x}}\")"
                ),
                format!(
                    "rule 'mk': its :assert writes (marker ?x), whose ?x is ?m of (marker ?m), \
                     which rule 'fan' passes to {r_param}, and (item ?x) can bind ?x to a fact, \
                     as an argument of the declared relation item — {NAMED}"
                ),
            ),
            // A rule's parameter passes it on: its activator, written.
            (
                format!(
                    "{pre}{r}\n(rule pass (?x) :match () :assert (r ?x) :why \"pass is given {{?x}}, so rule r is switched on for {{?x}}\")\n(pass (a b))"
                ),
                format!(
                    "the fact (pass (a b)) writes (a b) as rule 'pass''s parameter ?x, which it \
                     passes to {r_param} — {NAMED}"
                ),
            ),
            // A conclusion whose relation is a parameter writes into every rule
            // of its arity whose name the program writes (Q-M1f.42 (a)).
            (
                format!(
                    "{pre}{r}\n(rule copy (?S ?T) :match (?S ?x) :assert (?T ?x) :why \"{{?S}} holds of {{?x}}, so {{?T}} holds of {{?x}}\")\n(copy item r)"
                ),
                format!(
                    "rule 'copy': its :assert writes (?T ?x), whose ?x is {r_param}, and (?S ?x) \
                     can bind ?x to a fact, as an argument of the relation ?S names — {NAMED}"
                ),
            ),
            // An `(or …)` whose one arm binds it where a name is held.
            (
                format!(
                    "{pre}{r}\n(rule mk () :match (or (item ?f) (relation ?f ?a)) :assert (r ?f) :why \"{{?f}} is an item or a declared relation, so rule r is switched on for {{?f}}\")"
                ),
                format!(
                    "rule 'mk': its :assert writes (r ?f), whose ?f is {r_param}, and (item ?f) \
                     can bind ?f to a fact, as an argument of the declared relation item where \
                     its :match holds through (item ?f) — {NAMED}"
                ),
            ),
        ] {
            assert_eq!(err(&src), want, "{src}");
        }
        // What loads: a relation signature's name, a rule's relation
        // parameter (zebra2's `co-located-fanout`), a marker that passes a
        // name on and is written with names (the stdlib's
        // `bijective-properties`), an `eq` guard, and a value parameter.
        for src in [
            format!(
                "{pre}{r}\n(rule mk () :match (relation ?f ?a) :assert (r ?f) :why \"{{?f}} is a relation declared over {{?a}}, so rule r is switched on for {{?f}}\")"
            ),
            format!(
                "{pre}(rule co (?R1 ?V1 ?R2 ?V2) :match (?R1 ?V1 ?h) :assert (?R2 ?V2 ?h) :why \"{{?R1}} holds of {{?V1}} and {{?h}}, so {{?R2}} holds of {{?V2}} and {{?h}}\")\n\
                 (rule fan () :match (co ?R1 ?V1 ?R2 ?V2) :assert (co ?R2 ?V2 ?R1 ?V1) :why \"rule co copies {{?R1}} {{?V1}} to {{?R2}} {{?V2}}, so it also copies back the other way\")\n\
                 (co q a q b)"
            ),
            format!(
                "{pre}{r}\n(rule fan () :match (marker ?m) :assert (r ?m) :why \"the marker names {{?m}}, so rule r is switched on for {{?m}}\")\n(marker box)\n\
                 (rule mk () :match (marker ?m) :assert (marker ?m) :why \"the marker names {{?m}}, so it names {{?m}} again\")"
            ),
            format!(
                "{pre}{r}\n(rule mk () :match (eq ?f ?f) :assert (r ?f) :why \"{{?f}} is any name of the program, so rule r is switched on for {{?f}}\")"
            ),
            format!(
                "{pre}(rule v (?y) :match (item ?y) :assert (hit ?y) :why \"{{?y}} is an item, so hit holds of {{?y}}\")\n\
                 (rule mk () :match (item ?f) :assert (v ?f) :why \"{{?f}} is an item, so rule v is switched on for {{?f}}\")\n(item (a b))"
            ),
        ] {
            ok(&src);
        }
    }

    /// **`eq` at another arity is refused at load** — M1f S1f.5.9 (W60,
    /// Q-M1f.46; W61, Q-M1f.47): a relation parameter that can hold `eq`
    /// heading a premise or a conclusion whose arity is not 2, read as the
    /// check above reads relation names, one line each; and a written fact of
    /// `eq`, or its negation, at another arity.
    #[test]
    fn eq_at_another_arity_is_refused_at_load() {
        const TAIL: &str =
            "— eq is the relation of equality, which holds between exactly two values";
        let pre = "(relation p Node :why \"p holds of {?1}\") (relation hit Node :why \"hit holds of {?1}\") (relation q Node Node :why \"q holds of {?1} and {?2}\")\n(p a)\n";
        let r = "(rule r (?R) :match (?R ?x) :assert (hit ?x) :why \"{?R} holds of {?x}, so hit holds of {?x}\")";
        let line = |form: &str, n: usize, binds: &str| {
            format!(
                "rule 'r': its parameter ?R heads {form}, a form of {n} argument{}, and {binds} {TAIL}",
                if n == 1 { "" } else { "s" }
            )
        };
        for (src, want) in [
            // Written: a fact, a conclusion, a premise of three, under
            // `absent` and under `not`, and a conclusion.
            (
                format!("{pre}{r}\n(r eq)"),
                line("(?R ?x)", 1, "the fact (r eq) binds ?R to eq"),
            ),
            (
                format!(
                    "{pre}(rule r (?R) :match (and (p ?x) (?R ?x ?x ?x)) :assert (hit ?x) :why \"p holds of {{?x}} and {{?R}} holds of {{?x}}, {{?x}} and {{?x}}, so hit holds of {{?x}}\")\n(r eq)"
                ),
                line("(?R ?x ?x ?x)", 3, "the fact (r eq) binds ?R to eq"),
            ),
            (
                format!(
                    "{pre}(rule r (?R) :match (and (p ?x) (absent (?R ?x))) :assert (hit ?x) :why \"p holds of {{?x}} and {{?R}} does not, so hit holds of {{?x}}\")\n(r eq)"
                ),
                line("(?R ?x)", 1, "the fact (r eq) binds ?R to eq"),
            ),
            (
                format!(
                    "{pre}(rule r (?R) :match (and (p ?x) (not (?R ?x))) :assert (hit ?x) :why \"p holds of {{?x}} and {{?R}} is stored false of it, so hit holds of {{?x}}\")\n(r eq)"
                ),
                line("(?R ?x)", 1, "the fact (r eq) binds ?R to eq"),
            ),
            (
                format!(
                    "{pre}(rule r (?R) :match (p ?x) :assert (?R ?x) :why \"p holds of {{?x}}, so {{?R}} holds of {{?x}}\")\n(r eq)"
                ),
                line("(?R ?x)", 1, "the fact (r eq) binds ?R to eq"),
            ),
            // Derived: a conclusion, a marker, a guard while `eq` is a name
            // of the program, and an hrule's `(solve :hrules …)` activator.
            (
                format!(
                    "{pre}{r}\n(rule mk () :match (p ?y) :assert (r eq) :why \"p holds of {{?y}}, so rule r is switched on for eq\")"
                ),
                line(
                    "(?R ?x)",
                    1,
                    "?R can be bound to eq by rule 'mk''s conclusion (r eq)",
                ),
            ),
            (
                format!(
                    "{pre}{r}\n(rule mk () :match (mark ?f) :assert (r ?f) :why \"{{?f}} is marked, so rule r is switched on for {{?f}}\")\n(mark eq)"
                ),
                line(
                    "(?R ?x)",
                    1,
                    "?R can be bound to eq by the fact (mark eq), through rule 'mk''s conclusion (r ?f)",
                ),
            ),
            (
                format!(
                    "{pre}{r}\n(rule mk () :match (and (p ?y) (eq ?f ?f)) :assert (r ?f) :why \"p holds of {{?y}} and {{?f}} is any name of the program, so rule r is switched on for {{?f}}\")\n(note eq)"
                ),
                line(
                    "(?R ?x)",
                    1,
                    "?R can be bound to eq by an eq or neq guard of rule 'mk', over the program's \
                     names, eq among them, through its conclusion (r ?f)",
                ),
            ),
            (
                format!(
                    "{pre}(hrule h (?R) :match (?R ?x) :assert (hit ?x) :why \"{{?R}} holds of {{?x}}: guess that hit holds of {{?x}}\")\n(solve :hrules (h (eq)))"
                ),
                format!(
                    "hrule 'h': its parameter ?R heads (?R ?x), a form of 1 argument, and (solve \
                     :hrules (h (eq))) binds ?R to eq {TAIL}"
                ),
            ),
            // W61: a written fact, and its negation.
            (
                format!("{pre}(eq a)"),
                format!("the fact (eq a) writes eq with 1 argument {TAIL}"),
            ),
            (
                format!("{pre}(eq a a a)"),
                format!("the fact (eq a a a) writes eq with 3 arguments {TAIL}"),
            ),
            (
                format!("{pre}(not (eq a))"),
                format!("the fact (not (eq a)) writes eq with 1 argument {TAIL}"),
            ),
        ] {
            assert_eq!(err(&src), want, "{src}");
        }
        // A parameter heading a pattern inside an argument, a value, loaded
        // until M1f S1f.7.10: a value is no form of `eq`, at any arity. And a
        // nested `(eq a)`, which loaded too, is a parse error since.
        for (form, n) in [("(?R ?x)", 1), ("(?R ?x ?y)", 2)] {
            let got = err(&format!(
                "{pre}(rule r (?R) :match (p {form}) :assert (hit ?x) :why \"p holds of ({{?R}} …), so hit holds of {{?x}}\")\n(r eq)"
            ));
            assert!(
                got.contains(&format!(
                    "rule 'r': its parameter ?R heads {form}, written where a value goes, and \
                     the fact (r eq) binds ?R to eq — eq is computed and never stored"
                )),
                "{n}: {got}"
            );
        }
        // What loads: two arguments, the guard (S1f.5.7); a declared relation
        // of another arity, which matches nothing; a guard while `eq` is no
        // name of the program; and a written `(eq a b)`.
        for src in [
            format!(
                "{pre}(rule r (?R) :match (and (p ?x) (?R ?x ?x)) :assert (hit ?x) :why \"p holds of {{?x}} and {{?R}} holds of {{?x}} and {{?x}}, so hit holds of {{?x}}\")\n(r eq)"
            ),
            format!("{pre}{r}\n(r q)"),
            format!(
                "{pre}{r}\n(rule mk () :match (and (p ?y) (eq ?f ?f)) :assert (r ?f) :why \"p holds of {{?y}} and {{?f}} is any name of the program, so rule r is switched on for {{?f}}\")"
            ),
            format!("{pre}(eq a a)"),
        ] {
            ok(&src);
        }
    }

    /// **The verdict atom refuses every shape it cannot resolve** — M1d
    /// S1d.2.3, `defined_behaviour.md` § 4.2's table. **Since M1f S1f.5.5's
    /// swap (T1f.5.5.4) the shapes a position states are the grammar's**: an
    /// `(open …)` as a premise or beside another conclusion, with three
    /// arguments, or with a term for its argument, and `open` bound by a
    /// declarator, are parse errors at their place (`open` is a word a position
    /// gives a meaning to, the paper grammar's default 1), each a row of the
    /// grammar's table since T1f.5.5.5 (`ein-ir/tests/grammar_pages.rs`). **Since
    /// M1f S1f.5.23 every other place is too**: `open` is no name (the user's
    /// ruling on Q-M1f.66), so a pattern's argument, a conclusion's, a macro's
    /// body and a call's argument write none, and the loader's lines for the
    /// placements are deleted (`grammar_pages`' `DELETED`). Below are the
    /// refusals a context-free grammar does not see: the projection's ways of
    /// not resolving, each placed where the `(open …)` is written.
    #[test]
    fn the_verdict_atom_refuses_every_shape_it_cannot_resolve() {
        let pre = "(relation is-a T T :why \"{?1} is a {?2}\")\n(relation r A B :why \"r holds of {?1} and {?2}\")\n";
        // Nested rather than concluded, `(not (open ?R))`, was refused here
        // until M1f S1f.5.23, and is refused at the parse since: the grammar
        // writes `open` only as a rule's whole `:assert` (`grammar_pages`'
        // table).
        for (want, src) in [
            // The projection's three ways of not resolving.
            (
                "needs an `(absent …)` in :match",
                format!(
                    "{pre}(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A)) :assert (open ?R) :why \"{{?a}} is a {{?A}}, which {{?R}} is declared over, so a {{?R}} partner is owed\")"
                ),
            ),
            (
                "2 `(absent …)` guards",
                format!(
                    "{pre}(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) \
                     (absent (and (?isa ?b ?B) (?R ?a ?b))) \
                     (absent (and (?isa ?c ?B) (?R ?a ?c)))) :assert (open ?R) :why \"{{?a}} has no {{?R}} partner of type {{?B}} yet — one is still owed\")"
                ),
            ),
            (
                "compound witness",
                format!(
                    "{pre}(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) \
                     (absent (and (?R ?a ?x) (?R ?x ?b)))) :assert (open ?R) :why \"{{?a}} reaches nothing through two {{?R}} steps yet — such a path is still owed\")"
                ),
            ),
            (
                "the obligation is ground",
                format!(
                    "{pre}(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) \
                     (absent (?R ?a b))) :assert (open ?R) :why \"{{?a}} has no {{?R}} link to b yet — one is still owed\")"
                ),
            ),
            // A variable relation head comes from the activator, never a premise.
            (
                "is not in the parameter list",
                format!(
                    "{pre}(rule bad (?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A)) \
                     :assert (open ?R) :why \"{{?a}} is a {{?A}}, which {{?R}} is declared over, so a {{?R}} partner is owed\")"
                ),
            ),
            // Written as a premise's relation too, the rule pass refuses it
            // first, as it refuses any relation variable no parameter binds
            // (M1e2 S1e2.2.24, W24); until then this case reached the line above.
            (
                "with ?R as its relation, which no parameter binds",
                format!(
                    "{pre}(rule bad (?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) \
                     (absent (?R ?a ?b))) :assert (open ?R) :why \"{{?a}} is a {{?A}} with no {{?R}} partner yet — one is still owed\")"
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
        let src = "(relation is-a T T :why \"{?1} is a {?2}\")\n\
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
        let (terms, kb) = ok(
            "(relation r A B :why \"r holds of {?1} and {?2}\")\n(relation bare :why \"bare holds\")",
        );
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
        let (terms, kb) = ok(
            "(relation r T T :why \"r holds of {?1} and {?2}\")\n(r a b :source \"(1)\")\n(r c d)\n\
             (r e f :rule x :using ((r a b)))",
        );
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
        let (terms, kb) =
            ok("(relation r T T :why \"r holds of {?1} and {?2}\")\n(r a b :source \"sentence\")");
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
        // The facts make `a` and `b` relations of the program, which a goal
        // asks about since M1f S1f.7.11.
        let (_terms, kb) = ok(
            "(a k) (b k)\n(query :goal (a ?x))\n(query :goal (b ?x) :over all)\n\
             (config :record-alternative-justifications false)\n(config :lattice-sanity-check true)",
        );
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
        let e = err(
            "(relation p T P :why \"p holds of {?1} and {?2}\")\n(p A H)\n\
             (query :goal (p A ?h) :expect (model (p A H) (nosuch A)))",
        );
        assert!(e.contains("nosuch"), "{e}");
        assert!(e.contains("the program names no relation nosuch"), "{e}");
        // A relation only a rule's `:assert` derives is the program's (M1f
        // S1f.7.11, W78): a claim may list a fact of it.
        ok(
            "(relation p T P :why \"p holds of {?1} and {?2}\")\n(p A H)\n\
             (rule r () :match (p ?a ?h) :assert (won ?a) :why \"{?a} won\")\n\
             (query :goal (won ?a) :expect (model (won A)))",
        );
    }

    /// Since M1e2 S1e2.1a.5 a query's `:expect` lists projection classes —
    /// the goal's atoms under its rows — so a listed fact the goal does not
    /// match is refused, and the message names where a claim about the state
    /// belongs.
    #[test]
    fn a_query_claim_lists_only_facts_its_goal_matches() {
        let e = err(
            "(relation p T P :why \"p holds of {?1} and {?2}\")\n(relation q T P :why \"q holds of {?1} and {?2}\")\n(p A H)\n(q A H)\n\
             (query :goal (p A ?h) :expect (model (q A H)))",
        );
        assert!(
            e.contains("(q A H), which no atom of the query's :goal matches"),
            "{e}"
        );
        assert!(e.contains("(solve … :expect (region …))"), "{e}");
        // …in every disjunct, so one good class beside one bad one is still
        // refused.
        let e = err(
            "(relation p T P :why \"p holds of {?1} and {?2}\")\n(relation q T P :why \"q holds of {?1} and {?2}\")\n(p A H)\n(q A H)\n\
             (query :goal (p A ?h) \
              :expect (or (model (p A H)) (model (q A H))))",
        );
        assert!(e.contains("(q A H), which no atom"), "{e}");
        // A goal's constant must match, and a variable bind one value.
        let e = err(
            "(relation p T P :why \"p holds of {?1} and {?2}\")\n(p A H)\n(p B H)\n\
             (query :goal (p A ?h) :expect (model (p B H)))",
        );
        assert!(e.contains("(p B H), which no atom"), "{e}");
        let (_terms, _kb) = ok(
            "(relation p T P :why \"p holds of {?1} and {?2}\")\n(p A H)\n\
             (query :goal (and (p ?x ?h) (p ?x H)) :expect (model (p A H)))",
        );
    }

    /// A class is listed once, and `:over` is one of three words.
    #[test]
    fn a_class_twice_and_an_unknown_over_are_load_errors() {
        let e = err(
            "(relation p T P :why \"p holds of {?1} and {?2}\")\n(p A H)\n\
             (query :goal (p A ?h) :expect (or (model (p A H)) (model (p A H))))",
        );
        assert!(e.contains("lists the class (model (p A H)) twice"), "{e}");
        let e = err(
            "(relation p T P :why \"p holds of {?1} and {?2}\")\n(p A H)\n(query :goal (p A ?h) :over best)",
        );
        assert!(
            e.contains(":over best — one of all, minimal, maximal"),
            "{e}"
        );
        for over in ["all", "minimal", "maximal"] {
            ok(&format!(
                "(relation p T P :why \"p holds of {{?1}} and {{?2}}\")\n(p A H)\n(query :goal (p A ?h) :over {over})"
            ));
        }
    }

    /// A query with no `:expect` is untouched by any of the above — the
    /// corpus is 128 entries of exactly that.
    #[test]
    fn a_query_without_expect_is_unaffected() {
        let (_terms, kb) = ok(
            "(relation p T P :why \"p holds of {?1} and {?2}\")\n(p A H)\n(query :goal (p A ?h))",
        );
        assert_eq!(kb.program().queries.len(), 1);
    }

    /// The second query writes `:over all` since M1f S1f.5.5's swap, where it
    /// wrote `:mode all`, which the grammar refuses (S1f.5.11).
    #[test]
    fn a_later_query_is_reachable_by_index() {
        let src = "(a k) (b k)\n(query :goal (a ?x))\n(query :goal (b ?x) :over all)";
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
        let (terms, kb) = ok(
            "(relation r T T :why \"r holds of {?1} and {?2}\")\n(relation s T T :why \"s holds of {?1} and {?2}\")\n\
             (rule j (?p) :match (and (r ?a ?b) (not (s ?b ?c))) \
              :assert (r ?a ?c) :why \"r holds of {?a} and {?b} and s is denied of {?b} and {?c}, so r holds of {?a} and {?c}\")",
        );
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
            "(macro m (?a) (x ?a) :why \"x holds of {?a}\")\n(macro m (?a) (y ?a) :why \"y holds of {?a}\")\n(relation dup :why \"dup holds\")\n\
             (relation dup :why \"dup holds\")\n(rule __closed__ () :match (x ?a) :assert (y ?a) :why \"x holds of {?a}, so y holds of {?a}\")",
        );
        assert_eq!(
            message,
            "duplicate macro 'm' at None; duplicate relation 'dup' at None; \
             rule '__closed__' shadows a reserved kernel name at None"
        );
    }

    /// **A macro's call written as a fact is refused at load** — M1f S1f.7.12
    /// (W86): a fact headed by a macro's name, at any depth, one of
    /// `std.macro`'s the program does not import, a relation declared under a
    /// macro's name, and a call inside a fact a claim lists. A call in a
    /// `:match`, an `:assert` or a `:goal` is expanded, and the name written
    /// as a value is a name.
    #[test]
    fn a_macros_call_written_as_a_fact_is_refused_at_load() {
        let pre = "(relation p T :why \"p holds of {?1}\") (relation q T :why \"q holds of {?1}\")\n\
                   (macro pp (?a) (p ?a) :why \"pp of {?a}\")\n";
        let tail = "a macro is called in a :match, an :assert or a :goal, where its call is \
                    replaced by its body; a fact is not expanded, and no fact is of a macro: write \
                    the fact itself";
        for (text, want) in [
            (
                "(pp j)",
                "<string>:3:2: the fact (pp j) is a call of the macro pp, (macro pp (?a) (p ?a))"
                    .to_string(),
            ),
            (
                "(q (pp j))",
                "<string>:3:5: the fact (q (pp j)) holds (pp j), a call of the macro pp, (macro \
                 pp (?a) (p ?a))"
                    .to_string(),
            ),
            (
                "(not (pp j))",
                "the fact (not (pp j)) holds (pp j), a call of the macro pp".to_string(),
            ),
            (
                "(unknown (p j))",
                "the fact (unknown (p j)) is a call of std.macro's macro unknown, which the \
                 program does not import"
                    .to_string(),
            ),
            (
                "(relation pp T :why \"pp holds of {?1}\")",
                "relation 'pp': pp is a macro too, (macro pp (?a) (p ?a))".to_string(),
            ),
            (
                "(q (p j))\n(query :goal (q ?x) :expect (model (q (pp j))))",
                "query: its :expect lists (pp j), a call of the macro pp".to_string(),
            ),
        ] {
            let message = err(&format!("{pre}{text}"));
            assert!(message.contains(&want), "{text}: {message}");
            if !text.starts_with("(relation") {
                assert!(message.ends_with(tail), "{text}: {message}");
            }
        }
        for text in [
            "(p j)\n(rule r () :match (pp ?x) :assert (q ?x) :why \"r\")",
            "(q j)\n(rule r () :match (q ?x) :assert (pp ?x) :why \"r\")",
            "(p j)\n(query :goal (pp ?x))",
            "(q pp)",
        ] {
            let (_ast, _terms, kb) = load_text(&format!("{pre}{text}"));
            kb.unwrap_or_else(|e| panic!("{text}: {}", e.0));
        }
    }
}
