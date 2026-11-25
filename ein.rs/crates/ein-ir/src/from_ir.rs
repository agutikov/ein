//! IR → `Kb` — the loader.
//!
//! The loader walks a **flat sequence** of top-level forms and classifies each
//! by its head: `relation` declares one, `rule` a rule, `macro` a pattern
//! macro, `query` / `config` fill their slots, `trace` is
//! engine output and is ignored, and **anything else is a fact**. No form is a
//! block wrapper: a `(facts …)` form loads as a fact whose relation is `facts`,
//! like any other head.
//!
//! It is a validation surface as much as a construction one: errors
//! **accumulate** and surface as one `; `-joined message, so both the text of
//! each error and the order they were found in are observable. `examples/broken/load/` is the fixture set.
//!
//! It lives in `ein-ir` rather than beside the store because it needs the
//! frontend and the data model at once, and `ein-core` depends on nothing.

use crate::ast::{Ast, Node, NodeId, loc_repr};
use crate::grammar::ParseError;
use crate::imports::Resolver;
use crate::macros::{self, Macro as IrMacro, MacroError};
use ein_core::config::{FLAGS, SolverConfig};
use ein_core::entities::{EngineWord, ExprRef, Loc, Macro, Pattern, Query, Relation, Rule, Solve};
use ein_core::{Diagnostic, Kind, Place};
use ein_core::{
    FactId, Kb, Overflow, Program, Prov, ProvId, Symbol, Terms, Value, detect_provenance_cycles,
    is_predicate, is_reserved,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// **A load's refusal, as values** (Q-M2.9): its findings, in the order the
/// loader's passes found them, each a [`Diagnostic`] with its [`Kind`], the
/// refusal of `07-diagnostics.md` it is, its [`Place`] where the line names
/// one, and its line; or, where a text did not parse, that text's
/// [`ParseError`], whole, whose errors are its findings.
///
/// What a command prints is its `Display`, behind `kb load error: `: the
/// findings' lines `; `-joined, or the parse error's report, byte for byte
/// what the load printed before its findings were values.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KbLoadError {
    findings: Vec<Diagnostic>,
    parse: Option<ParseError>,
}

impl KbLoadError {
    /// A refusal of these findings.
    pub fn new(findings: Vec<Diagnostic>) -> Self {
        KbLoadError {
            findings,
            parse: None,
        }
    }

    /// A refusal of one finding: `kind`, its line `message`, at `place`.
    pub fn one(kind: Kind, message: impl Into<String>, place: Option<Place>) -> Self {
        KbLoadError::new(vec![Diagnostic::error(kind, message).at(place)])
    }

    /// The findings, each with its kind, its place and its line.
    pub fn findings(&self) -> &[Diagnostic] {
        &self.findings
    }

    /// The parse error, where the program's text, or a module's, did not
    /// parse.
    pub fn parse(&self) -> Option<&ParseError> {
        self.parse.as_ref()
    }
}

impl std::fmt::Display for KbLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.parse {
            Some(p) => write!(f, "{p}"),
            None => {
                let lines: Vec<&str> = self.findings.iter().map(|d| d.message.as_str()).collect();
                f.write_str(&lines.join("; "))
            }
        }
    }
}

impl std::error::Error for KbLoadError {}

impl From<crate::imports::LoadError> for KbLoadError {
    fn from(e: crate::imports::LoadError) -> Self {
        let (findings, parse) = e.into_parts();
        KbLoadError { findings, parse }
    }
}

impl From<ParseError> for KbLoadError {
    fn from(e: ParseError) -> Self {
        KbLoadError {
            findings: e.diagnostics(),
            parse: Some(e),
        }
    }
}

impl From<Overflow> for KbLoadError {
    /// The store's own limits (`07-diagnostics.md` § 2.5).
    fn from(e: Overflow) -> Self {
        KbLoadError::one(Kind::StoreLimit, e.to_string(), None)
    }
}

/// A refusal of `kind`, its line `message`, at `place`.
fn refusal(kind: Kind, place: Option<Place>, message: String) -> Diagnostic {
    Diagnostic::error(kind, message).at(place)
}

/// Where `node` is written, as a finding's place: the position [`placed`]
/// writes in front of a line.
fn at(ast: &Ast, node: NodeId) -> Option<Place> {
    crate::ast::place(ast, ast.loc(node))
}

/// [`at`] `node`, or at `outer`: [`placed_or`]'s position.
fn at_or(ast: &Ast, node: NodeId, outer: NodeId) -> Option<Place> {
    at(ast, node).or_else(|| at(ast, outer))
}

/// The first position at or under `node`, in the order it is written:
/// [`placed_form`]'s.
fn written_at(ast: &Ast, node: NodeId) -> Option<crate::ast::Loc> {
    ast.loc(node).or_else(|| match ast.node(node) {
        Node::SForm { head, .. } => written_at(ast, head)
            .or_else(|| ast.form_args(node).iter().find_map(|&a| written_at(ast, a))),
        _ => None,
    })
}

/// [`written_at`], as a finding's place.
fn at_form(ast: &Ast, node: NodeId) -> Option<Place> {
    crate::ast::place(ast, written_at(ast, node))
}

/// What a load requires of a program beyond what the grammar does.
///
/// **One option, and it decides only whether a program loads**: `:why` is
/// optional in the grammar, and this option makes the loader require it. A
/// program it lets through loads to the same `Kb` it loads to without it, so it
/// is not a lever under the config principle (`docs/ein/user/04-reference/06-config.md`
/// § 2.5). It is not a `(config …)` field, which a program could set against
/// itself and which the KB-shape digest renders, and not an `EIN_*` name; the
/// CLI's `--no-require-why` turns it off on every command that loads a program.
///
/// **Required by default**: every program the corpus, the stdlib and the Rust
/// suites load carries its `:why`s, so [`load`], [`load_query`] and
/// [`load_file`] refuse one that does not; `require_why: false` loads it, as
/// `--no-require-why` does (Q-M1f.62).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoadOptions {
    /// Refuse every `(relation …)`, `(rule …)` and `(macro …)`
    /// the load reads, the stdlib's and an imported module's included, that
    /// carries no `:why "…"`, one line each ([`WHY_REQUIRED_OF`]); the parser
    /// reads a macro's `:why` as it reads the others', and the stdlib's macros
    /// carry theirs. **Default true.**
    pub require_why: bool,
}

impl Default for LoadOptions {
    fn default() -> Self {
        LoadOptions { require_why: true }
    }
}

/// The forms [`LoadOptions::require_why`] asks for a `:why`, as its refusal
/// names them.
pub const WHY_REQUIRED_OF: &str = "relation, rule and macro";

/// The most distinct variables a rule's `:match`, its parameters among them,
/// or a query's `:goal` may write (Q-M1f.73).
///
/// The compiler numbers a plan's variables into a fixed register file of this
/// many slots, so the matcher's inner loop allocates nothing
/// (`ein_infer::MAX_REGS`, which is this constant): one file for the plan, its
/// parameters and the variables its positive premises and guards write, and
/// one per lifted `(absent …)`, the variables its body writes. The loader
/// counts every distinct variable the clause writes, anywhere in it, with the
/// parameters, which bounds each file, so the compiler meets no clause past it
/// (`refuse_wide_matches`). The bound is the loader's, not the compiler's, so
/// a clause past it is refused whether or not any state activates its rule or
/// is answered from its goal.
pub const MAX_MATCH_VARIABLES: usize = 256;

/// The line [`LoadOptions::require_why`] refuses a form with: its head and its
/// name as written, `rule 'r'`, as the loader's other refusals spell a form.
fn missing_why(head: &str, name: &str) -> String {
    format!(
        "{head} '{name}' has no :why \"…\", which this load requires of every {WHY_REQUIRED_OF}"
    )
}

/// Parse and load a `.ein` file, resolving its imports file-relative: a
/// program, or a saved file ([`crate::saved::load_text`]).
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
    let text = std::fs::read_to_string(path).map_err(|e| {
        KbLoadError::one(Kind::Unreadable, format!("{}: {e}", path.display()), None)
    })?;
    // A program, or a saved file (`crate::saved`).
    crate::saved::load_text(ast, terms, &text, path.to_str(), path.parent(), 0, opts)
        .map(|l| l.kb)
        .map_err(|e| match e {
            crate::saved::TextError::Parse(e) => KbLoadError::from(e),
            crate::saved::TextError::Load(e) => e,
        })
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
    load_query_by(
        &Resolver::new(),
        ast,
        terms,
        forms,
        base_dir,
        active_query,
        opts,
    )
}

/// [`load_query_with`], its imports resolved by `resolver`: a saved file's
/// program by the modules it carries ([`Resolver::carrying`]), and any load
/// whose caller reads afterwards what the resolver read
/// ([`Resolver::modules_read`], `crate::saved`).
pub fn load_query_by(
    resolver: &Resolver,
    ast: &mut Ast,
    terms: &mut Terms,
    forms: &[NodeId],
    base_dir: Option<&Path>,
    active_query: usize,
    opts: &LoadOptions,
) -> Result<Kb, KbLoadError> {
    let mut kb = Kb::new(Program::new());
    kb.program_mut().active_query = active_query;
    let mut errors: Vec<Diagnostic> = Vec::new();

    // A saved file, its first form its `(program …)`: its program against
    // the modules it carries, with its choices (`crate::saved`). Its
    // wrappers stand at the top of a saved file alone: a program holds none
    // (Q-M1f.401).
    if forms
        .first()
        .is_some_and(|&f| ast.head_name(f) == Some("program"))
    {
        return crate::saved::load_forms(ast, terms, forms, active_query, opts);
    }
    if let Some(&w) = forms
        .iter()
        .find(|&&f| matches!(ast.head_name(f), Some("program" | "module" | "commit")))
    {
        return Err(KbLoadError::one(
            Kind::SavedFile,
            format!(
                "a program holds a ({} …) at {} — (program …), (module …) and (commit …) are a \
                 saved file's, which opens with its (program …)",
                ast.head_name(w).unwrap_or(""),
                loc_repr(ast, ast.loc(w))
            ),
            at(ast, w),
        ));
    }

    // Imports resolve up front into one flat, import-free, qualified stream.
    // Resolution errors are **fatal** and return immediately: a half-resolved
    // program cannot be ingested.
    let forms = resolver.resolve_program(ast, forms, base_dir)?;
    // The rules of the module every program gets, which no import line
    // brought (Q-M1f.551): marked, for the readers that tell a rule a program
    // wrote or imported from one it was given.
    let implicit = resolver.implicit_names();

    let (mut relations, mut rules, mut macro_forms, mut facts) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let (mut query_blocks, mut config_blocks): (Vec<NodeId>, Vec<NodeId>) =
        (Vec::new(), Vec::new());
    let mut solve_blocks: Vec<NodeId> = Vec::new();
    for &form in &forms {
        // Every top-level form has a name for its head: a declarator's word,
        // or a fact's relation (`ein.y`'s `Form`). The grammar refuses a form
        // that has none, and a fact headed by a claim's word, `(region …)`,
        // `(merged …)` or `(owing …)`, because a top-level fact's relation
        // (`FactHead`) is none of the three.
        let head = ast
            .head_name(form)
            .expect("a top-level form's head is a name (ein.y's Form)");
        match head {
            "relation" => relations.push(form),
            "rule" => rules.push(form),
            "macro" => macro_forms.push(form),
            "import" => errors.push(refusal(
                Kind::Import,
                at(ast, form),
                format!(
                    "unresolved (import …) at {} — internal error",
                    loc_repr(ast, ast.loc(form))
                ),
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
            errors.push(refusal(
                Kind::MissingWhy,
                None,
                missing_why("macro", &m.name),
            ));
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
    // Pass 2 — rules. After this, rule-name resolution is possible.
    ingest_rules(
        ast,
        terms,
        &mut kb,
        &rules,
        Declarations {
            macros: &declared_macros,
            implicit: &implicit,
        },
        opts,
        &mut errors,
    )?;
    // Pass 3 — facts. A written `(open …)` is held aside until the
    // program's relations are known (`write_open_facts`).
    let mut written_open: Vec<(FactId, ProvId, NodeId)> = Vec::new();
    for &form in &facts {
        ingest_fact(ast, terms, &mut kb, form, &mut written_open)?;
    }
    // A written fact of `eq` at another arity.
    refuse_eq_facts_of_another_arity(ast, &facts, &mut errors);
    // A macro's call written as a fact, which nothing expands, and a relation
    // declared under a macro's name.
    let stdlib_macros = resolver.stdlib_macro_names();
    refuse_macro_facts(
        ast,
        &relations,
        &facts,
        &declared_macros,
        &stdlib_macros,
        &mut errors,
    );
    // An obligation made to owe `open` (Q-M1f.66).
    refuse_open_owed(ast, terms, &kb, &facts, &mut errors);
    // Which positions hold relation names, read off the program (Q-M1f.41,
    // Q-M1f.42): `eq`, `absent` and `false` passed where one heads a form of
    // another arity are refused (Q-M1f.46), and the names passed there are
    // relations of the program (Q-M1f.522).
    let relation_places = read_relation_places(ast, terms, &kb, &facts, &solve_blocks, &mut errors);
    // A conclusion `(not ?x)` whose variable the program shows can hold a
    // name.
    refuse_negated_names(ast, terms, &kb, &facts, &solve_blocks, &mut errors);

    // Every query block, in source order: a program may ask several
    // questions, and none is discarded. `config` keeps last-wins: a config is
    // a setting, a query is content.
    //
    // A goal is a premise, so a macro called in one is expanded as a rule's
    // `:match` is: every reader of the query's `:goal`
    // meets the expansion, and `Query::goal_written` keeps the goal as the
    // program wrote it, for the surfaces that print it.
    let mut queries: Vec<Query> = Vec::with_capacity(query_blocks.len());
    for &form in &query_blocks {
        queries.push(expand_goal(ast, form, &declared_macros, &mut errors));
    }
    kb.program_mut().queries = queries;
    // One lattice per program: a second `(solve …)` would be a second R,
    // and a program states one R, which its queries all ask about.
    if solve_blocks.len() > 1 {
        errors.push(refusal(
            Kind::Claim,
            None,
            format!(
                "{} (solve …) forms — a program states its lattice once",
                solve_blocks.len()
            ),
        ));
    }
    kb.program_mut().solve = solve_blocks.first().map(|&form| Solve {
        kw_pairs: ast.form_args(form).iter().map(|n| ExprRef(n.0)).collect(),
    });
    // The relations of the program, every atom it writes first in a tuple
    // outside its goals and its `(solve …)` lists (Q-M1f.522, Q-M1f.114):
    // the registry takes them, and they are what a claim may list and a goal
    // may ask about. A program already refused is not read for them, since a
    // rule the loader refused may be missing from the registries they are
    // read off.
    let relations_of = if errors.is_empty() {
        let written = relations_written(ast, terms, &kb, &facts, &relation_places);
        register_relations(terms, &mut kb, &written)?;
        Some(program_relations(&written))
    } else {
        None
    };
    // Kept on the program: what a goal given beside it is held to
    // ([`read_goal`]), and what the slice of a knowledge base reads a
    // relation by (Q-M1f.523).
    if let Some(all) = relations_of.as_ref() {
        let symbols = all
            .iter()
            .map(|name| terms.intern_text(name))
            .collect::<Result<Vec<Symbol>, Overflow>>()?;
        kb.program_mut().relations_of = symbols.into_boxed_slice();
    }
    validate_solve(ast, &kb, relations_of.as_ref(), &mut errors);
    // The `open` facts the program writes, into the part of root no fork
    // inherits (Q-M1f.566).
    write_open_facts(ast, terms, &mut kb, &written_open, &mut errors);
    // A `(solve …)` list that names a relation the program does not have.
    if let Some(known) = relations_of.as_ref() {
        refuse_solve_names(ast, &kb, known, &mut errors);
    }
    let unimported: Vec<String> = stdlib_macros
        .iter()
        .filter(|n| !declared_macros.contains_key(*n))
        .cloned()
        .collect();
    validate_queries(ast, &kb, relations_of.as_ref(), &unimported, &mut errors);
    // …and one a claim lists, which is a fact too. A claim headed
    // by the call names no relation of the program and has its own line
    // (Q-M1f.118), so this one is for a program nothing else refuses.
    if errors.is_empty() {
        refuse_macro_claims(ast, &kb, &declared_macros, &stdlib_macros, &mut errors);
    }
    if let Some(&last) = config_blocks.last() {
        let args: Vec<NodeId> = ast.form_args(last).to_vec();
        kb.program_mut().config = Some(config_from_kw_pairs(ast, &args));
    }

    // The unimported-macro guard: a `(forall …)` / `(unknown …)` used without importing
    // `std.macro` would leave the invocation in place and the rule would
    // silently never fire.
    let mut rule_matches: Vec<(String, NodeId)> = Vec::new();
    for (name, rule) in kb.program().every_rule() {
        if let Some(p) = rule.match_.as_ref() {
            rule_matches.push((format!("rule '{}'", terms.sym(name)), NodeId(p.expr.0)));
        }
    }
    // …and a goal, which is a premise too: unexpanded, the call would
    // match nothing and the goal would have no row in any state.
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
        let path: Vec<String> = cycle.iter().map(|&f| terms.fact_text(f)).collect();
        errors.push(refusal(
            Kind::DerivationCycle,
            None,
            format!("derivation cycle: {}", path.join(" -> ")),
        ));
    }

    if errors.is_empty() {
        Ok(kb)
    } else {
        Err(KbLoadError::new(errors))
    }
}

/// Intern every name a **rule** or the `(query …)` block mentions, so that
/// nothing is left for the compiler to intern later.
///
/// The compiler resolves a pattern's leaves against `Terms` the first time it
/// builds a plan for a rule, and a constant that appears only inside a rule —
/// `Ann` in `(rule guess … :assert (open (seat Ann ?v)))`, which no fact mentions —
/// is therefore first seen *during the search*. That is one symbol on one
/// corpus file, and it would be a curiosity except for what
/// [03-design/08-parallelism.md](../../../../docs/ein/ein.rs/03-design/08-parallelism.md) needs:
/// [`Interner::text`](ein_core::Interner::text) hands out a `&str` borrowed
/// from the arena, so an interner that is *shared* must be one that does not
/// **grow**, and the search is exactly where it would be shared
/// ([03-design/08-parallelism.md § 3](../../../../docs/ein/ein.rs/03-design/08-parallelism.md#3-what-must-be-sync-and-how)).
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
/// which is a name the program has; an activator that binds it to a nested
/// fact activates nothing (`ein_infer::activators_for`), and no plan is
/// compiled for it. A run of digits is a name, interned here like any other.
fn intern_program_names(ast: &Ast, terms: &mut Terms, kb: &Kb) {
    fn walk(ast: &Ast, terms: &mut Terms, node: NodeId) {
        match ast.node(node) {
            Node::Atom(s) | Node::Var(s) => {
                let _ = terms.intern_text(ast.sym(s));
            }
            // A string is natural-language text, here a query's `:goal-text`,
            // and names nothing, so it is not interned.
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
    for (_, rule) in program.every_rule() {
        roots.extend(
            [rule.match_.as_ref(), rule.assert_.as_ref()]
                .into_iter()
                .flatten()
                .map(|p| NodeId(p.expr.0)),
        );
    }
    // Every query, not just the active one: a name that only query 2 mentions
    // is still a name the compiler must not meet for the first time mid-run.
    for q in &program.queries {
        roots.extend(q.kw_pairs.iter().map(|e| NodeId(e.0)));
    }
    // …and the `(solve …)` block's lists.
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
/// compiler reads it as a value (`form-in-guard`), and must not intern it
/// for the first time during
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
/// (`ein.y`'s `SolveClause` and `QueryClause`): any other is a parse error at
/// its place, a query's lattice keyword with a note naming its home on
/// `(solve …)`, so this pass reads no keyword list of its own
/// ([claims](../../../../docs/ein/user/03-syntax-and-semantics/12-query.md#claims)).
fn validate_solve(
    ast: &Ast,
    kb: &Kb,
    relations_of: Option<&BTreeSet<String>>,
    errors: &mut Vec<Diagnostic>,
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
            errors.push(refusal(Kind::Claim, None, message));
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
            // stored (Q-M1e2.26), and a region claim lists a state's stored
            // facts, so one that lists either can never hold, and is refused.
            if is_predicate(fact.relation) {
                if reported.insert(fact.relation.to_string()) {
                    errors.push(refusal(
                        Kind::Claim,
                        None,
                        format!(
                            "(solve … :expect …) names {}, which a state never stores — eq and \
                             neq are computed from their definition, so a region claim, which \
                             lists a state's facts, holds neither",
                            fact.relation
                        ),
                    ));
                }
                continue;
            }
            // A relation of the program, not only one a declaration or
            // a fact makes, so a claim may list a fact only a rule's `:assert`
            // derives.
            let known = relations_of.is_none_or(|k| k.contains(fact.relation));
            if !known && reported.insert(fact.relation.to_string()) {
                errors.push(refusal(
                    Kind::Claim,
                    None,
                    format!(
                        "(solve … :expect …) names {0}, and the program names no relation {0} — \
                         {CLAIM_RELATION}",
                        fact.relation
                    ),
                ));
            }
        }
    }
}

/// Where a program holds relation names, as [`read_relation_places`] reads
/// them off it.
#[derive(Default)]
struct RelationPlaces {
    /// Every name a fact or a conclusion writes where a relation's name is
    /// held: a rule's parameter that heads a form, or a marker's argument.
    passed: BTreeSet<String>,
}

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
                              writes in a rule";

/// **The atoms the program writes first in a tuple** (Q-M1f.522), outside its
/// goals and its `(solve …)` lists (Q-M1f.114): the relation of every fact it
/// states and of every fact inside one, at any depth; of every form at any
/// depth in a rule's or an obligation's `:match` or `:assert`, as
/// expanded; every relation it declares; and every name a fact or a conclusion
/// passes where a relation's name is held (`places.passed`), the relation an
/// activator gives a parameter that heads a form, `happy` in `(tag happy)`.
///
/// The kernel's own words are among them where the program writes them, `not`
/// or `__closed__`: a claim lists a state's `(__closed__ R)` as it lists any
/// fact. [`register_relations`] takes the program's relations from it, and
/// [`program_relations`] the list a goal and a claim are held to.
fn relations_written(
    ast: &Ast,
    terms: &Terms,
    kb: &Kb,
    facts: &[NodeId],
    places: &RelationPlaces,
) -> BTreeSet<String> {
    let program = kb.program();
    let mut written: BTreeSet<String> = places.passed.clone();
    for (name, relation) in program.relations.iter() {
        if relation.declared {
            written.insert(terms.sym(name).to_string());
        }
    }
    for (_, rule) in program.every_rule() {
        for pat in [rule.match_.as_ref(), rule.assert_.as_ref()]
            .into_iter()
            .flatten()
        {
            relation_heads(ast, NodeId(pat.expr.0), &mut written);
        }
    }
    for &f in facts {
        relation_heads(ast, f, &mut written);
    }
    written
}

/// **The program's relations, registered** (Q-M1f.522): every atom the program
/// writes first in a tuple ([`relations_written`]) that the registry does not
/// hold yet joins it as a relation no declaration made, in the order of its
/// text, so that a relation is one whether a fact, a fact inside one, a rule
/// or an activator writes it, and the blind enumerator takes none of them for
/// an object. A word of the kernel's, `not`, `__closed__` or `relation`, is
/// none ([`ein_core::terms::heads_a_relation`]).
fn register_relations(
    terms: &mut Terms,
    kb: &mut Kb,
    written: &BTreeSet<String>,
) -> Result<(), Overflow> {
    for name in written {
        if !ein_core::terms::heads_a_relation(name) {
            continue;
        }
        let name = terms.intern_text(name)?;
        if !kb.program().relations.contains(name) {
            kb.program_mut().add_relation(Relation {
                name,
                signature: Box::new([]),
                declared: false,
                why: None,
                loc: None,
            });
        }
    }
    Ok(())
}

/// **The relations a goal and a claim are held to**: every atom the program
/// writes first in a tuple ([`relations_written`]), its relations and the
/// kernel's words it writes, and `eq` and `neq`, the kernel's computed
/// relations, every program's. So a relation only a rule's `:assert` derives
/// is one, and so is one written only inside a fact.
///
/// A goal's own relations are not among them: a goal reads, and a relation
/// only a goal names is one no state holds a fact of. [`refuse_solve_names`]
/// adds them for its lists (Q-M1f.114).
fn program_relations(written: &BTreeSet<String>) -> BTreeSet<String> {
    let mut known = written.clone();
    // The two relations of equality, the kernel's computed ones, are every
    // program's; no theory's relation is.
    known.extend(["eq", "neq"].map(str::to_string));
    known
}

/// **A `(solve …)` list names what the program has**.
///
/// The two lists configure the blind enumerator, so a name in one that
/// matches nothing would move H, and R with it, in silence:
/// `:no-hypothesis (like)` would restrict nothing. Each is refused here, one
/// line per name, placed at the name: every name `:hypothesis-relations` or
/// `:no-hypothesis` holds is a relation of the program.
///
/// **A relation of the program** is an atom the program writes first in a
/// tuple outside the `(solve …)` lists ([`relations_written`], Q-M1f.522):
/// the relation of a fact it states and of a fact inside one, of any form in
/// a rule's or an obligation's `:match` or `:assert`, one it declares, and a
/// name a fact or a conclusion passes where a relation's name is held,
/// `(bijective color-loc)`; and here, for the lists, the relation of any form
/// in a query's `:goal`. So a relation only a rule's `:assert` derives is
/// one. It is not *a relation something makes*: what tells a misspelt name
/// is that the program writes it nowhere else (Q-M1f.114).
fn refuse_solve_names(
    ast: &Ast,
    kb: &Kb,
    relations_of: &BTreeSet<String>,
    errors: &mut Vec<Diagnostic>,
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
    const RELATION: &str = "a relation of the program is one it declares, states a fact of, or \
                            writes in a rule or a goal";

    for &pair in solve.kw_pairs.iter() {
        let Node::KwPair { key, value } = ast.node(NodeId(pair.0)) else {
            continue;
        };
        let Node::Keyword(key) = ast.node(key) else {
            continue;
        };
        let list = ast.sym(key);
        if !matches!(list, "hypothesis-relations" | "no-hypothesis") {
            continue;
        }
        for (node, name) in names(ast, value) {
            if !known.contains(&name) {
                errors.push(refusal(
                    Kind::SolveList,
                    at_or(ast, node, value),
                    format!(
                        "{}solve: its :{list} names {name}, and the program names no \
                         relation {name} — {RELATION}",
                        placed_or(ast, node, value),
                    ),
                ));
            }
        }
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
/// keywords a query takes, and a claim's shape, are the grammar's
/// ([`validate_solve`]).
///
/// Runs after the fact pass, because two of the three `:expect` rules are
/// about the *program*: a relation it names must be one the program knows, and
/// the goal's relations must be among the ones it closes.
fn validate_queries(
    ast: &Ast,
    kb: &Kb,
    relations_of: Option<&BTreeSet<String>>,
    unimported: &[String],
    errors: &mut Vec<Diagnostic>,
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
            refuse_goal(ast, goal, relations_of, unimported, errors);
        }
        if let Some(node) = query_keyword(ast, query, "over")
            && let Err(message) = crate::expect::Over::parse(ast, node)
        {
            errors.push(refusal(Kind::Claim, None, message));
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
                // `neq` are relations a goal may ask about (Q-M1e2.26): their
                // facts are computed, so no declaration or fact makes them
                // one, and the check below is the goal's. So is an
                // `(absent …)` the goal writes, whose instance a class lists
                // beside the rest. So is a fact of a kernel relation a goal
                // reads, `(relation next-to)`, which the loader stores and
                // a goal may ask about ([`refuse_goal_relations`]), so that
                // a class's `as :expect` loads back under its query
                // (Q-M1f.532).
                //
                // The rest is a relation of the program, [`program_relations`],
                // not only one a declaration or a fact makes, so a
                // claim may list a fact only a rule's `:assert` derives.
                let known = is_predicate(fact.relation)
                    || fact.relation == "absent"
                    || is_reserved(fact.relation)
                    || relations_of.is_none_or(|k| k.contains(fact.relation));
                if !known {
                    errors.push(refusal(
                        Kind::Claim,
                        None,
                        format!(
                            ":expect names {0}, and the program names no relation {0} — \
                             {CLAIM_RELATION}",
                            fact.relation
                        ),
                    ));
                }
            }
        }
        // A query's `:expect` lists its projection classes (`12-query.md`):
        // each `(model …)` is one class,
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
                    errors.push(refusal(
                        Kind::Claim,
                        None,
                        format!(
                            ":expect lists {}, which no atom of the query's :goal matches — a \
                             query's claim lists its projection classes; a claim about the \
                             state is (solve … :expect (region …))'s",
                            crate::expect::render(ast, f)
                        ),
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
                errors.push(refusal(
                    Kind::Claim,
                    None,
                    format!(":expect lists the class (model {}) twice", class.join(" ")),
                ));
            } else {
                classes.push(class);
            }
        }
    }
}

/// **A template that names a variable no reference can reach is refused**
/// ([`07-diagnostics.md` § 2.11](../../../../docs/ein/user/04-reference/07-diagnostics.md#211-a-template-reference-no-reference-can-reach)).
/// A reference reads a
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
    errors: &mut Vec<Diagnostic>,
) {
    for v in ein_core::why::unreachable_references(template, vars.iter().map(String::as_str)) {
        errors.push(refusal(
            Kind::Template,
            None,
            format!(
                "{what}: its {clause} writes {{?{v}}}, a reference to ?{v}, which no reference \
                 can name — a reference reads a variable's name up to its first }}"
            ),
        ));
    }
}

/// **A goal is a premise**, held to what a rule's `:match` is held to: the one
/// walk a rule's clauses get reads it, a guard over a form holding a variable
/// (Q-M1f.30), a premise that is no form, which a macro's expansion can leave,
/// and a relation written as a variable, at the top level and inside a
/// pattern; it is compiled as a `:match` is, and refused where the compiler
/// would refuse it; and a relation it asks about is a relation of the
/// program, `known`, where the program's relations are read. A query's goal
/// and a goal given beside the program ([`read_goal`]) alike.
fn refuse_goal(
    ast: &Ast,
    goal: NodeId,
    known: Option<&BTreeSet<String>>,
    unimported: &[String],
    errors: &mut Vec<Diagnostic>,
) {
    let n_refused = errors.len();
    refuse_no_fact_holds(ast, "query", &[], ":goal", goal, errors);
    refuse_nested_disjunctions(ast, "query", ":goal", goal, errors);
    refuse_wide_matches(ast, "query", ":goal", &[], goal, errors);
    // A kernel primitive in a shape only a macro's expansion writes, read by
    // the same walk a rule's clauses get.
    if errors.len() == n_refused {
        refuse_malformed_primitives(ast, "query", ":goal", goal, errors);
    }
    if errors.len() == n_refused
        && let Some(known) = known
    {
        refuse_goal_relations(ast, goal, known, unimported, errors);
    }
}

/// **A goal given beside a loaded program** (Q-M1f.529): `text`, a goal as a
/// `(query …)`'s `:goal` writes it, read as the `:goal` of one more query of
/// the program and held to what that query's would be held to. It is parsed
/// as `(query :goal TEXT)`, under the name `name`, into the program's own
/// `ast`, so a line a refusal prints reads as a query's; its macro calls are
/// expanded by the program's macros; it is refused by the walks a query's
/// goal gets (`refuse_goal`), its relations held to the program's,
/// [`ein_core::Program::relations_of`]. Any other keyword beside `:goal`,
/// or a text that is not one goal, is refused: a text that does not parse as
/// the parser refuses a program, and a goal the loader would refuse with
/// the lines it would print, after `kb load error: `.
///
/// What it answers is the query it was read as, the goal expanded, which the
/// caller matches against a knowledge base of the program
/// (`ein_infer::slice::answering`); nothing is added to `kb`.
pub fn read_goal(
    ast: &mut Ast,
    terms: &Terms,
    kb: &Kb,
    text: &str,
    name: &str,
) -> Result<Query, crate::saved::TextError> {
    use crate::saved::TextError;
    let wrapped = format!("(query :goal {text})");
    let forms = crate::parse(ast, &wrapped, Some(name)).map_err(TextError::Parse)?;
    let one_goal = match forms.as_slice() {
        [form] if ast.head_name(*form) == Some("query") => {
            let args = ast.form_args(*form);
            let only_goal = args.len() == 1
                && matches!(ast.node(args[0]), Node::KwPair { key, .. }
                    if matches!(ast.node(key), Node::Keyword(k) if ast.sym(k) == "goal"));
            only_goal.then_some(*form)
        }
        _ => None,
    };
    let Some(form) = one_goal else {
        return Err(TextError::Load(KbLoadError::one(
            Kind::Goal,
            format!(
                "{name}: {} is not one goal — it is read as (query :goal …)'s goal, and nothing \
                 stands beside it",
                ein_core::repr::repr_str(text)
            ),
            None,
        )));
    };
    let program = kb.program();
    let macros: BTreeMap<String, IrMacro> = program
        .macros
        .iter()
        .map(|(_, m)| {
            let name = terms.sym(m.name).to_string();
            let body = NodeId(m.body.0);
            let ir = IrMacro {
                name: name.clone(),
                params: m.params.iter().map(|p| terms.sym(*p).to_string()).collect(),
                body,
                why: None,
                form: body,
            };
            (name, ir)
        })
        .collect();
    let mut errors: Vec<Diagnostic> = Vec::new();
    let query = expand_goal(ast, form, &macros, &mut errors);
    let goal = query_keyword(ast, &query, "goal").expect("read as (query :goal …)");
    let known: BTreeSet<String> = program
        .relations_of
        .iter()
        .map(|&r| terms.sym(r).to_string())
        .collect();
    if errors.is_empty() {
        refuse_goal(ast, goal, Some(&known), &[], &mut errors);
    }
    if errors.is_empty() {
        Ok(query)
    } else {
        Err(TextError::Load(KbLoadError::new(errors)))
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
/// A fact holds names and facts (`ein.y`'s `FactArg`): the grammar refuses a
/// string or a variable as a fact's argument at the parse, so neither reaches
/// here.
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
/// The **reading** is [`macros::collect_macros`]'s — one implementation, so
/// the dump path and the loader refuse the same programs with the same
/// sentences. What stays here is the half that needs a
/// `Terms` and a `Kb`: interning the name and the parameters, and storing the
/// macro on the program.
fn ingest_macros(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    forms: &[NodeId],
    errors: &mut Vec<Diagnostic>,
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
    errors: &mut Vec<Diagnostic>,
) -> Result<(), Overflow> {
    let loc = loc_repr(ast, ast.loc(form));
    let args = ast.form_args(form).to_vec();
    // Flat args. An EMPTY signature is legal —
    // `(relation R)` declares a relation node with no declared arg types, and
    // is deliberately not a hypothesis target, since the "declared domain
    // relation" signal is signature *presence*.
    //
    // A declaration is a name, then names, then its `:why` (`ein.y`'s
    // `Relation`): the grammar refuses `(relation)` with no name, and a
    // signature holding anything but names, so the wrapped
    // `(relation R (T1 T2))` never loads as a bare declaration.
    let name = ast
        .atom_name(args[0])
        .expect("a declaration's first argument is its name (ein.y's Relation)")
        .to_string();
    if opts.require_why && string_value(ast, kw_get(&kw_pairs(ast, &args), "why")).is_none() {
        errors.push(refusal(
            Kind::MissingWhy,
            None,
            missing_why("relation", &name),
        ));
    }
    let signature: Vec<String> = args[1..]
        .iter()
        .filter_map(|&a| ast.atom_name(a).map(str::to_string))
        .collect();
    if !ein_core::binds_no_reserved(&name) {
        errors.push(refusal(
            Kind::Declaration,
            at(ast, form),
            format!("relation '{name}' shadows a reserved kernel name at {loc}"),
        ));
        return Ok(());
    }
    let name_sym = terms.intern_text(&name)?;
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
    // A relation may be declared more than once, at one count or at several
    // (Q-M1f.447): the registry keeps the first declaration, and the program
    // every declaration, in the order written.
    kb.program_mut().add_relation(Relation {
        name: name_sym,
        signature: sig.clone().into_boxed_slice(),
        declared: true,
        why,
        loc: core_loc(ast, form),
    });
    kb.program_mut().declarations.push(ein_core::Declaration {
        name: name_sym,
        signature: sig.clone().into_boxed_slice(),
        why,
    });
    // The declaration is also stored as an ordinary fact, so rules can
    // introspect signatures with a `(relation ?R ?A ?B)` pattern.
    let relation = terms.kernel.relation;
    let mut decl_args = vec![Value::sym(name_sym)];
    decl_args.extend(sig.iter().map(|&s| Value::sym(s)));
    kb.add_fact(terms, relation, &decl_args, None)?;
    // The companion arity-1 *membership* fact. Matching is
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
/// a rule's clauses with.
#[derive(Clone, Copy)]
struct Declarations<'a> {
    macros: &'a BTreeMap<String, IrMacro>,
    /// The names the module every program gets declares
    /// ([`Resolver::implicit_names`]).
    implicit: &'a [String],
}

fn ingest_rules(
    ast: &mut Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    forms: &[NodeId],
    declarations: Declarations,
    opts: &LoadOptions,
    errors: &mut Vec<Diagnostic>,
) -> Result<(), Overflow> {
    let Declarations {
        macros: declared_macros,
        implicit,
    } = declarations;
    for &form in forms {
        let head = ast.head_name(form).expect("bucketed by head").to_string();
        let loc = loc_repr(ast, ast.loc(form));
        let args = ast.form_args(form).to_vec();
        // A rule is a name, a parameter list, then its clauses (`ein.y`'s
        // `Rule`): the grammar refuses a rule with fewer than two arguments,
        // and one whose name or parameter list is not one.
        let name = ast
            .atom_name(args[0])
            .expect("a rule's first argument is its name (ein.y's Rule)")
            .to_string();
        let params_form = args[1];
        if opts.require_why && string_value(ast, kw_get(&kw_pairs(ast, &args), "why")).is_none() {
            errors.push(refusal(Kind::MissingWhy, None, missing_why(&head, &name)));
        }
        // A rule named `absent` / `false` / `eq` / `relation` would never
        // fire — the compiler reads those as primitives — so it is rejected
        // rather than registered dead.
        if !ein_core::binds_no_reserved(&name) {
            errors.push(refusal(
                Kind::Declaration,
                at(ast, form),
                format!("{head} '{name}' shadows a reserved kernel name at {loc}"),
            ));
            continue;
        }
        let name_sym = terms.intern_text(&name)?;
        // Every rule shares one name-space, whichever pass walks it.
        if kb.program().rules.contains(name_sym)
            || kb.program().obligations.contains(name_sym)
            || kb.program().hypotheses.contains(name_sym)
        {
            errors.push(refusal(
                Kind::Declaration,
                at(ast, form),
                format!("duplicate rule name '{name}' at {loc}"),
            ));
            continue;
        }
        let pairs = kw_pairs(ast, &args);
        // A rule states what it concludes under `:assert`, facts, or a word
        // to the engine under `:ein`, which is no fact (Q-M1f.399). The
        // passes below read such a rule's conclusion as `()`, which writes
        // nothing.
        let Ok(ein) = engine_word(ast, &head, &name, &pairs, errors) else {
            continue;
        };
        let concluded = match ein {
            Some(_) => Some(ast.sform_named(ein_core::text::EMPTY_HEAD, &[], None)),
            None => kw_get(&pairs, "assert"),
        };
        let (Some(match_node), Some(assert_node)) = (kw_get(&pairs, "match"), concluded) else {
            errors.push(refusal(
                Kind::Declaration,
                at(ast, form),
                format!("({head} {name}) missing :match or :assert at {loc}"),
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
                    errors.push(refusal(
                        e.0.kind,
                        e.0.place.clone(),
                        format!("({head} {name}): {e}"),
                    ));
                    continue;
                }
            }
        };
        // A wildcard as a relation, in a rule, and the shapes that
        // read nothing.
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
        // file: the two shapes the compiler cannot compile, read here, after
        // expansion, so a macro's `(or …)` too (Q-M1f.74).
        refuse_nested_disjunctions(ast, &who, ":match", match_node, errors);
        refuse_wide_matches(ast, &who, ":match", &param_vars, match_node, errors);
        if errors.len() != n_refused {
            continue;
        }
        // A kernel primitive the grammar shapes, which a macro's expansion
        // wrote in another shape (Q-M1f.80): read
        // here, after expansion, so no command compiles, fires or proposes it.
        refuse_malformed_primitives(ast, &who, ":match", match_node, errors);
        refuse_malformed_primitives(ast, &who, ":assert", assert_node, errors);
        if errors.len() != n_refused {
            continue;
        }
        // A conclusion's variable nothing binds.
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
        // What the rule concludes under `open` (Q-M1f.566, Q-M1f.573): a
        // relation it owes, the rule's whole `:assert`, which makes it an
        // **obligation**, refused here where its projection does not resolve;
        // or hypotheses, which a rule concludes beside any facts. Either is
        // routed out of the saturation agenda below.
        let n_errors = errors.len();
        let opened = validate_open(
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
        // A top-level `(or …)` `:match` and an `(and …)` `:assert` stay as
        // ordinary AST on ONE rule, which keeps its source name; the compiler
        // lowers them to several plans and templates, an `(and …)` read flat at
        // any depth (`Ast::conjuncts`).
        let match_ = pattern_from_ir(ast, terms, match_node)?;
        let assert_ = match ein {
            Some(_) => None,
            None => Some(pattern_from_ir(ast, terms, assert_node)?),
        };
        // The parameters that head a form: a name written twice counts at its
        // last position, whose value the compiler seeds it with.
        let relation_params: Box<[usize]> = param_names
            .iter()
            .enumerate()
            .filter(|&(i, p)| {
                !param_names[i + 1..].contains(p)
                    && [Some(&match_), assert_.as_ref()]
                        .into_iter()
                        .flatten()
                        .any(|pat| names_relation(ast, NodeId(pat.expr.0), p))
            })
            .map(|(i, _)| i)
            .collect();
        let rule = Rule {
            name: name_sym,
            params: params.into_boxed_slice(),
            match_: Some(match_),
            assert_,
            ein,
            implicit: implicit.contains(&name),
            relation_params,
            why,
            loc: core_loc(ast, form),
        };
        match opened {
            // Never the saturator's: it derives nothing, so it has no place in
            // a queue that orders derivation, and it is read once per quiescent
            // KB after the fixpoint instead (`08-obligations.md`).
            Opened::Owes => kb.program_mut().add_obligation(rule),
            // A rule that concludes hypotheses is read by the pass that
            // writes a state's `open` facts, at every state, and never by the
            // saturator (Q-M1f.569); the facts it concludes beside them are an
            // ordinary rule's, the same rule with them alone.
            Opened::Hypotheses { rest } => {
                if !rest.is_empty() {
                    let conclusion = if rest.len() == 1 {
                        rest[0]
                    } else {
                        ast.sform_named("and", &rest, None)
                    };
                    let mut ordinary = rule.clone();
                    ordinary.assert_ = Some(pattern_from_ir(ast, terms, conclusion)?);
                    kb.program_mut().add_rule(ordinary);
                }
                kb.program_mut().add_hypothesis(rule);
            }
            Opened::Nothing => kb.program_mut().add_rule(rule),
        }
    }
    Ok(())
}

/// **The word a rule gives the engine under `:ein`** (Q-M1f.399), or `None`
/// for a rule that states none. The grammar takes any name there (`ein.y`'s
/// `RuleClause`), and this holds it to the engine's words
/// ([`EngineWord::ALL`]), so that a misspelt word does not load in silence.
///
/// It refuses, each with one line that starts at the word's position and
/// lists the engine's words:
///
/// - **a word that is none of the engine's**, `:ein dead`;
/// - **`:ein` beside an `:assert`**: a rule states facts or a word to the
///   engine, never both, since a state the firing ends takes no fact from
///   it.
fn engine_word(
    ast: &Ast,
    head: &str,
    name: &str,
    pairs: &[(String, NodeId)],
    errors: &mut Vec<Diagnostic>,
) -> Result<Option<EngineWord>, ()> {
    let Some(node) = kw_get(pairs, "ein") else {
        return Ok(None);
    };
    let who = format!("{}{head} '{name}'", placed(ast, node));
    let place = at(ast, node);
    let written = ast
        .atom_name(node)
        .expect(":ein takes a name (ein.y's RuleClause)");
    let words = EngineWord::ALL.map(EngineWord::as_str).join(", ");
    if kw_get(pairs, "assert").is_some() {
        errors.push(refusal(
            Kind::EngineWord,
            place,
            format!(
                "{who}: states :ein {written} beside an :assert — a rule states what it \
                 concludes under :assert, or a word to the engine under :ein, never both: write \
                 one rule for each (the engine's words: {words})"
            ),
        ));
        return Err(());
    }
    match EngineWord::of(written) {
        Some(word) => Ok(Some(word)),
        None => {
            errors.push(refusal(
                Kind::EngineWord,
                place,
                format!(
                    "{who}: :ein {written} — {written} is no word of the engine's (the engine's \
                     words: {words})"
                ),
            ));
            Err(())
        }
    }
}

/// **What no fact can hold, and what reads nothing, in a rule is refused**
/// (Q-M1e2.18; a bare `_` is refused at the parse, Q-M1e2.22).
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
///   `(and …)` or an `(or …)`, an `(absent …)`'s body: only a macro's body
///   puts one there, a name or a variable;
/// - a **guard's argument that is a pattern**, a form that holds a variable,
///   `(eq ?x (f ?y))` (Q-M1f.30).
///
/// **A guard's other arguments load** (Q-M1e2.26): `eq` and `neq` are
/// relations, a fact that holds no variable is a value as it is in
/// `(p (f a))`, and a premise means the same in any order of its conjuncts,
/// so a guard waits for the premise that binds its variable, and a variable
/// nothing binds ranges over the program's names (`ein_infer::compile`'s
/// `Seq`).
///
/// **The grammar refuses the rest**, so this walk meets none of them: a
/// wildcard written as a relation, `(_ a)`; a keyword pair inside a pattern
/// inside a pattern, `(box (a :k v))`; `_` or a keyword pair as a guard's
/// argument; and an atom, `_`, a string or a variable written where a
/// premise goes, each a row of the grammar's table
/// (`ein-ir/tests/grammar_pages.rs`). So it does a keyword pair at a
/// pattern's top level, `(n ?x :where v)`, or as a premise of an `(and …)`,
/// `:where (neq ?a ?b)` (Q-M1e2.28); `_` as a pattern's argument; and
/// `_` or a keyword pair at an `:assert`'s conclusion's top level. A keyword
/// pair met as a premise would be refused here as a premise that is no form.
/// Ein has no ranges (Q-M1e2.24): `1..3` is a parse error.
///
/// **A query's `:goal` is read by the same walk**, `who` being `query` and
/// `clause` `:goal`: a goal is a premise. A query has no activator, so every
/// relation written as a variable is refused, at the top level and
/// inside a pattern, and so is a premise that is no form, which a
/// macro's expansion leaves in a goal as in a rule.
fn refuse_no_fact_holds(
    ast: &Ast,
    who: &str,
    params: &[&str],
    clause: &str,
    node: NodeId,
    errors: &mut Vec<Diagnostic>,
) {
    // The grammar refuses every other shape, so no program reaches them: a
    // wildcard written as a relation, a keyword pair inside a pattern inside
    // a pattern, `_` as a guard's argument and a keyword pair as one. The
    // parser builds no `_` (Q-M1e2.22) and no keyword pair inside a clause's
    // value (Q-M1e2.28), and macro expansion copies only what a program
    // wrote. `NotAForm` is reached: a macro's body is a template, and
    // `(macro id (?x) ?x)` called as `(id foo)` leaves the name `foo` where a
    // premise goes.
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
        /// its arguments as slots. A pattern holds no keyword pair (`ein.y`'s
        /// `Args` and `Terms`).
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
        /// computed (Q-M1e2.26), so each argument is a value — a variable, a
        /// name, or a fact that holds no variable — and a variable no premise
        /// binds is one the guard binds, in any order of the conjuncts. A
        /// form that holds a variable is a pattern, and is refused
        /// (Q-M1f.30).
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
            // No premise is a keyword pair (the grammar's, Q-M1e2.28), and
            // one met would be a premise that is no form.
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
        /// flat at any depth), each a pattern. No keyword pair parses beside
        /// them.
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
        // A goal's relation variable is § 2.15's, the shapes no plan can be
        // made of; the rest are § 2.9's, a rule's and a goal's alike.
        let kind = match f {
            Found::FreeHead(..) if clause == ":goal" => Kind::NoPlan,
            _ => Kind::RuleShape,
        };
        let line = match f {
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
        };
        errors.push(refusal(kind, None, line));
    }
}

/// **A conclusion's every variable is bound**
/// ([`07-diagnostics.md` § 2.13](../../../../docs/ein/user/04-reference/07-diagnostics.md#213-a-conclusions-variable-nothing-binds)).
///
/// Every variable a rule's `:assert` writes, at any depth, must
/// be bound on every way its `:match` can hold, as the compiler reads it
/// (`ein_infer::compile`):
///
/// - by a **parameter**, which the activator binds;
/// - by a **relation premise**, at any depth of its arguments — a stored
///   `(not P)` among them, a pattern over stored negations;
/// - or by an **`eq` or `neq` guard**, whose variable nothing else binds
///   ranges over the program's names (Q-M1e2.26).
///
/// A variable seen only inside an `(absent …)` — `forall` and `unknown` expand
/// to one — is not bound outside it. A top-level `(or …)` is one way per
/// disjunct, and the variable must be bound in each; a nested one binds what
/// every one of its arms binds (and is refused, [`refuse_nested_disjunctions`]).
/// A hypothesis, `(open F)`, is read as `F` is: a variable it writes is bound
/// as any conclusion's; what a state owes, `(open R)` or `(open ?R)`, writes a
/// name or a parameter.
///
/// One line per variable, in the order the `:assert` writes them, naming the
/// rule, the variable, the disjuncts that leave it unbound where some bind it,
/// and the two explicit spellings of what the implicit form could mean:
/// *the objects a relation holds of*, a premise of any relation that binds it,
/// `(R ?y)`, and *every name of the program*, `(eq ?y ?y)`. The line names no
/// relation of a theory's: the kernel reads none (Q-M1f.436).
///
/// **Checked at load, on every command**, so no firing meets a variable it
/// cannot ground, and whether a program loads does not depend on whether a
/// run reaches a node that fires the rule: a rule only a commitment fires is
/// refused as one root fires is, and so is a rule whose hypothesis could
/// propose nothing.
fn refuse_unbound_conclusions(
    ast: &Ast,
    head: &str,
    name: &str,
    params: &[&str],
    match_node: NodeId,
    assert_node: NodeId,
    errors: &mut Vec<Diagnostic>,
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
        written(ast, c, &mut vars);
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
        errors.push(refusal(
            Kind::UnboundConclusion,
            None,
            format!(
                "{head} '{name}': its :assert writes ?{v}, which no premise or parameter \
                 binds{place} — to conclude it of the objects a relation holds of, bind it with \
                 a premise such as (R ?{v}), or of every name of the program with (eq ?{v} ?{v})"
            ),
        ));
    }
}

/// **An `(or …)` is a whole `:match`, or a whole `:goal`**
/// ([`07-diagnostics.md` § 2.15](../../../../docs/ein/user/04-reference/07-diagnostics.md#215-a-nested-or--a-wide-match-and-a-goals-relation-variable)).
///
/// A top-level `(or …)` is one way the rule holds per disjunct, each compiled
/// as a plan of its own (`ein_infer::compile`'s `match_disjuncts`). One written
/// as a premise, of an `(and …)`, an `(absent …)` or a top-level `(or …)`, is
/// not lowered. A plan is compiled per activator, by the pass that writes a
/// state's `open` facts for a rule that concludes a hypothesis, and where a
/// state is answered for a goal, so a refusal where the plan is compiled
/// would let a rule only a commitment activates, such a rule, a goal and one
/// no state reaches load, and some commands answer what others
/// stop on. The loader refuses it instead, one line per disjunction as
/// written, naming what it is a premise of; the walk is the compiler's
/// (`premise`), so it does not descend into a pattern, a guard or a stored
/// negation, where an `(or …)` is no premise.
fn refuse_nested_disjunctions(
    ast: &Ast,
    who: &str,
    clause: &str,
    node: NodeId,
    errors: &mut Vec<Diagnostic>,
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
        errors.push(refusal(
            Kind::NoPlan,
            None,
            format!(
                "{who}: its {clause} writes {} as a premise of {parent} — an (or …) is a whole \
                 {clause}, each of its premises one way it holds: lift it to the top of the \
                 {clause}, or {split}",
                crate::dump::dump_compact(ast, or),
            ),
        ));
    }
}

/// **A macro's expansion is held to the shapes the grammar gives a written
/// rule** (Q-M1f.67;
/// [`07-diagnostics.md` § 2.16](../../../../docs/ein/user/04-reference/07-diagnostics.md#216-a-malformed-primitive-a-macros-expansion-writes)).
///
/// A macro's body is a template whose head may be any name at any arity
/// (`ein.y`'s `Template`), and a call's argument may be any premise, so the
/// grammar, which shapes each kernel primitive where a rule writes it, never
/// reads the expanded rule. Unread here, what it writes would be met only
/// where it is used, or not at all: `eq` and `absent` at another arity by the
/// compiler, where it compiles a plan, so whether a program loads would hang
/// on whether a command activates the rule; a `not` of no fact, a
/// connective where a value goes, `false` with an argument and an `(or …)`
/// concluded would be matched, concluded or proposed as they stand; and a
/// conclusion that is no form would reach the firing and the
/// hypothesis layer. An unused macro is not read here, so one that no
/// rule calls loads whatever its body (Q-M1f.67); one a rule calls is refused
/// where it is called, at load, on every command (Q-M1f.80).
///
/// **The walk holds the value rule too** (Q-M1f.113: *an object is an atom, or a
/// fact a state could hold: a fact of an ordinary relation, or its
/// negation*). The grammar refuses what a program writes of it,
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
/// rule's whole `:assert`, Q-M1f.66, and a macro's body and a call's argument
/// write no `open`), a premise that is no form and a relation variable no
/// parameter binds (`refuse_no_fact_holds`), and an `(or …)` written as a
/// premise (`refuse_nested_disjunctions`). A written rule fails it only by a
/// connective written as a pattern's argument: the grammar refuses the other
/// shapes at the parse. One line per form, in the spelling of the loader's
/// other clause refusals: the position where the expanded form is
/// written, which is the macro's body or the call's argument, then the rule,
/// the clause, the form as Ein text and what to write instead (Q-M1f.82).
fn refuse_malformed_primitives(
    ast: &Ast,
    who: &str,
    clause: &str,
    node: NodeId,
    errors: &mut Vec<Diagnostic>,
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
    found: Vec<Diagnostic>,
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
    /// `(or …)`, has none (`at None`), so its first part's is
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

    /// `<place><who>: its <clause> writes <form>, and <what>`, a finding of
    /// `kind` placed where [`Shapes::place`] places it: § 2.16's for a
    /// malformed primitive, § 2.17's where `open` or `absent` stands under
    /// the other, § 2.22's where what is no value stands where a value goes.
    fn refuse(&mut self, node: NodeId, kind: Kind, what: String) {
        let line = format!(
            "{}{}: its {} writes {}, and {what}",
            self.place(node),
            self.who,
            self.clause,
            self.text(node)
        );
        self.found
            .push(refusal(kind, at_form(self.ast, node), line));
    }

    /// `eq` and `neq` compare two values (`ein.y`'s `Guard`).
    fn guard(&mut self, node: NodeId, name: &str) {
        let args = self.args(node);
        if args.len() != 2 {
            self.refuse(
                node,
                Kind::MalformedPrimitive,
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
                        Kind::MalformedPrimitive,
                        format!(
                            "absent takes exactly 1 argument, not {} — negation as failure is \
                             over one premise, so wrap several in an (and …)",
                            args.len()
                        ),
                    );
                    return;
                }
                // `absent` takes a premise, which may hold an `absent` inside
                // a conjunction, the universal, but is not one directly
                // (Q-M1f.568, Q-M1f.571).
                if matches!(self.shape(args[0]), Shape::Form(Some("absent"))) {
                    self.refuse(
                        node,
                        Kind::OpenShape,
                        "absent directly under absent is refused — absent takes a premise, a \
                         pattern or a conjunction, and (absent (absent P)) is no test a state \
                         makes"
                            .to_string(),
                    );
                    return;
                }
                self.premise(args[0]);
            }
            Some("not") => self.negation(node, false, false),
            Some(g @ ("eq" | "neq")) => self.guard(node, g),
            // `false` takes no argument, as a premise too.
            Some("false") if !args.is_empty() => self.false_with_arguments(node, "holds where"),
            // A premise `(open …)` reads the `open` facts a state holds
            // (Q-M1f.569): one argument, a fact's pattern or a variable.
            Some("open") => {
                if self.opened(node) {
                    for a in args {
                        self.arg(node, a);
                    }
                }
            }
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
    /// still a pattern's argument is a pattern no fact matches: unread,
    /// `(q (and (p ?x)))` would match nothing, in silence, and under an
    /// `(absent …)` hold, so the rule would fire.
    fn arg(&mut self, within: NodeId, node: NodeId) {
        // A premise pattern's argument is § 2.22's, a connective there too.
        if let Some((_, what)) = self.no_value(node) {
            self.refuse(within, Kind::Object, what);
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
    /// has (the value rule, Q-M1f.113: *an object is an atom, or a fact a state
    /// could hold: a fact of an ordinary relation, or the stored negation of a
    /// fact a state could hold*): a
    /// connective, `and`, `or` or `neq`; `eq`, computed and never stored;
    /// `absent`, a premise's test; and `false` with an argument.
    ///
    /// The kind is the section that states the line: a connective where a
    /// value goes is § 2.16's, a malformed primitive, where a guard, a
    /// conclusion or a negation writes it, and the rest is § 2.22's, the
    /// value rule's.
    fn no_value(&self, node: NodeId) -> Option<(Kind, String)> {
        const VALUE: &str = "a value is a name, a variable, or a fact a state could hold, \
                             (relation argument …) or its negation";
        let text = self.text(node);
        match self.shape(node) {
            Shape::Form(Some(c @ ("and" | "or" | "neq"))) => Some((
                Kind::MalformedPrimitive,
                format!(
                    "{text} is a connective where a value goes — {c} heads no fact, and a value \
                     is a name, a variable or a fact (relation argument …)"
                ),
            )),
            Shape::Form(Some("eq")) => Some((
                Kind::Object,
                format!(
                    "{text} is written where a value goes — eq is computed and never stored, \
                     and {VALUE}"
                ),
            )),
            Shape::Form(Some("false")) if !self.args(node).is_empty() => Some((
                Kind::Object,
                format!(
                    "{text} is written where a value goes — false takes no argument, and {VALUE}"
                ),
            )),
            _ => None,
        }
    }

    /// `(false k)` written as a premise, § 2.22's, or a conclusion, § 2.16's.
    fn false_with_arguments(&mut self, node: NodeId, says: &str) {
        let n = self.args(node).len();
        let kind = if self.clause == ":assert" {
            Kind::MalformedPrimitive
        } else {
            Kind::Object
        };
        self.refuse(
            node,
            kind,
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
        if let Some((kind, what)) = self.no_value(node) {
            self.refuse(within, kind, what);
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
    /// a stored negation among them (`NegPattern`, Q-M1f.391), and, as a
    /// conclusion, of a variable bound to one (`(not Var)`). Written
    /// as a premise or a conclusion of its own it may negate an equality,
    /// `(not (eq a b))`, which is computed (`ein.y`'s `Negated`); as a `value`,
    /// inside a pattern or a conclusion, it negates a fact a state could hold.
    fn negation(&mut self, node: NodeId, conclusion: bool, value: bool) {
        let args = self.args(node);
        if args.len() != 1 {
            self.refuse(
                node,
                Kind::MalformedPrimitive,
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
            Shape::Form(Some("eq" | "false")) => {
                if let Some((kind, what)) = self.no_value(p) {
                    return self.refuse(node, kind, what);
                }
            }
            _ => {}
        }
        let fits = match (self.ast.node(p), self.shape(p)) {
            (Node::Var(_), _) => conclusion,
            // The negation of a negation is a fact (Q-M1f.391): what it
            // denies is a stored negation, held to what a negation written
            // as a value is, so never one of a variable or of an equality.
            (_, Shape::Form(Some("not"))) => return self.negation(p, false, true),
            (_, Shape::Form(Some("and" | "or" | "neq"))) => false,
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
            self.refuse(node, Kind::MalformedPrimitive, what);
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
    /// every reader of a conclusion shares), each a form.
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
            self.found.push(refusal(
                Kind::MalformedPrimitive,
                at_form(self.ast, node),
                line,
            ));
            return;
        };
        let args = self.args(node);
        match head {
            Some("or") => self.refuse(
                node,
                Kind::MalformedPrimitive,
                "an (or …) is no conclusion — a rule concludes every conclusion it writes, and a \
                 disjunction is a :match's, one way it holds per premise: write one rule per \
                 disjunct"
                    .to_string(),
            ),
            Some("not") => self.negation(node, true, false),
            Some("false") if !args.is_empty() => self.false_with_arguments(node, "concludes that"),
            // No fact is of `absent`, so no conclusion is.
            Some("absent") => self.refuse(
                node,
                Kind::Object,
                "absent is negation as failure, a premise's test, and no fact is of it — a \
                 conclusion is a fact (relation argument …); the stored negation of a fact is \
                 (not …)"
                    .to_string(),
            ),
            Some(g @ ("eq" | "neq")) => self.guard(node, g),
            // What a state owes, or a hypothesis: one argument, which
            // `validate_open` reads (Q-M1f.573).
            Some("open") => {
                if self.opened(node) {
                    for a in args {
                        self.term(node, a);
                    }
                }
            }
            _ => {
                for a in args {
                    self.term(node, a);
                }
            }
        }
    }

    /// `(open …)` where the kernel reads it, a premise's or a conclusion's
    /// head (Q-M1f.568): it takes one argument, and directly under it neither
    /// `open` nor `absent` is written, `(open (open P))` and
    /// `(open (absent P))`. The grammar gives each shape, and this holds a
    /// macro's expansion to it. `false` where it refused.
    fn opened(&mut self, node: NodeId) -> bool {
        let args = self.args(node);
        if args.len() != 1 {
            self.refuse(
                node,
                Kind::OpenShape,
                format!(
                    "open takes exactly 1 argument, not {} — a relation's name, which a state \
                     owes, or a fact, which is a hypothesis",
                    args.len()
                ),
            );
            return false;
        }
        if let Shape::Form(Some(k @ ("open" | "absent"))) = self.shape(args[0]) {
            self.refuse(
                node,
                Kind::OpenShape,
                format!(
                    "{k} directly under open is refused — open takes a relation's name or a \
                     fact a state could hold, and the kernel reads {k} there"
                ),
            );
            return false;
        }
        true
    }
}

/// **A rule binds at most [`MAX_MATCH_VARIABLES`] variables, and so does a
/// goal** (Q-M1f.73).
///
/// The count is every distinct variable the clause writes, at any depth, and
/// the rule's parameters, which bounds each register file the compiler builds
/// for it (`MAX_REGS`). Read at load, so a rule no state activates and a goal
/// no state is answered from are refused as any other is.
fn refuse_wide_matches(
    ast: &Ast,
    who: &str,
    clause: &str,
    params: &[&str],
    node: NodeId,
    errors: &mut Vec<Diagnostic>,
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
    errors.push(refusal(
        Kind::NoPlan,
        None,
        format!(
            "{who}: its {clause} writes {} distinct variables{among}, and {binds} binds at most \
             {MAX_MATCH_VARIABLES} — {split}",
            vars.len()
        ),
    ));
}

/// **A goal asks about relations of the program**.
///
/// A premise of a goal over a relation the program names nowhere else can
/// have no row in any state, and under an `(absent …)` holds in every one, so
/// a relation misspelt in a goal would read as a goal that is undetermined. Each is
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
    errors: &mut Vec<Diagnostic>,
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
            errors.push(refusal(
                Kind::Goal,
                at_or(ast, node, goal),
                format!(
                    "{}query: its :goal writes {}, and the program names no relation {name} — \
                     {CLAIM_RELATION}, so no state holds a fact of {name}",
                    placed_or(ast, node, goal),
                    crate::dump::dump_compact(ast, node),
                ),
            ));
        }
    }
}

/// One `(query …)` as the program holds it: its clauses, with the macro calls
/// of its `:goal` expanded as a rule's `:match`'s are.
///
/// A goal is a premise. Unexpanded, a call in a goal, `(unknown (p ?x))`,
/// would read as a pattern over a relation `unknown`: the goal would have no
/// row in any state, `:expect (model)` would hold of it, and the claim that
/// lists what the macro means would be refused at load. The expansion replaces the `:goal` pair's value, so every reader of the query
/// meets it, and [`Query::goal_written`] keeps the goal as written, which the
/// read-out and `ein test` print (Q-M1f.117). A call that does not expand, by
/// its arity or by a macro that expands to itself, is one line, and the goal
/// is left as written.
fn expand_goal(
    ast: &mut Ast,
    form: NodeId,
    macros: &BTreeMap<String, IrMacro>,
    errors: &mut Vec<Diagnostic>,
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
                Err(e) => errors.push(refusal(
                    e.0.kind,
                    e.0.place.clone(),
                    format!("query: its :goal: {e}"),
                )),
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

/// **Which positions hold relation names**, read off the program's text
/// (Q-M1f.41). A rule's parameter that heads a form, `(?R …)`, holds one, and
/// every argument of `(relation …)` holds a name. An argument of a
/// **declared** relation holds any value of its type, a fact among them, as
/// does an argument inside a fact pattern, `(p (g ?x))`, and an argument of a
/// relation a parameter names, `(?S ?x)`. An argument of an **undeclared**
/// relation, a marker such as `(bijective color-loc)`, and a rule's parameter
/// hold what the program passes on from them: where a variable bound there
/// reaches a position that holds relation names, so does that position,
/// inferred to a fixpoint. This is how `bijective-properties` passes
/// `(bijective ?R)`'s `?R` to `functional`.
///
/// A conclusion written `(?T …)` writes into the relation its activator
/// names: it is read as writing into every rule of its arity whose name the
/// program writes as an argument (Q-M1f.42), which is where a rule's name
/// can come from. `(not P)` is read as `P`, in a premise, a conclusion and a
/// fact, since a stored negative holds the values its positive would.
///
/// An activator that binds a relation parameter to a fact, written or
/// derived, activates nothing (`ein_infer::activators_for`), and nothing says
/// so (Q-M1f.1): the positions are read for the two things below, and for no
/// line of their own.
///
/// **What is refused**: `eq`, `absent` and `false` where a relation name
/// passes at another arity (`refuse_eq_at_another_arity`, Q-M1f.46).
///
/// **What it returns**: every name the program writes at a position that
/// holds relation names, in a fact or a conclusion — the relations a program
/// names only by passing them to a rule, which [`relations_written`] takes
/// into the program's relations (Q-M1f.522).
fn read_relation_places(
    ast: &Ast,
    terms: &Terms,
    kb: &Kb,
    facts: &[NodeId],
    solve_blocks: &[NodeId],
    errors: &mut Vec<Diagnostic>,
) -> RelationPlaces {
    /// A position: a relation's name and an argument's index.
    type Pos = (String, usize);
    /// Where a premise binds a variable.
    enum Occ {
        /// The deriving rule's parameter, by index.
        Param(usize),
        /// An argument of a premise whose relation is a name.
        Arg { rel: String, k: usize },
        /// An argument of a fact pattern inside a premise, or of a premise
        /// whose relation is a parameter: a place that holds any value.
        Value,
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
        ways: Vec<Vec<Occ>>,
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
                for (k, &a) in ast.form_args(p).iter().enumerate() {
                    match ast.node(a) {
                        Node::Var(s) if ast.sym(s) == var => out.push(match &rel {
                            Some(rel) => Occ::Arg {
                                rel: rel.clone(),
                                k,
                            },
                            None => Occ::Value,
                        }),
                        Node::SForm { .. } => inner(ast, a, var, out),
                        _ => {}
                    }
                }
            }
        }
    }
    fn inner(ast: &Ast, form: NodeId, var: &str, out: &mut Vec<Occ>) {
        for &a in ast.form_args(form) {
            match ast.node(a) {
                Node::Var(s) if ast.sym(s) == var => out.push(Occ::Value),
                Node::SForm { .. } => inner(ast, a, var, out),
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
    let rules: Vec<(&'static str, (Symbol, &Rule))> =
        program.every_rule().map(|r| ("rule", r)).collect();
    // Each rule's parameters, and the positions that hold relation names
    // because a parameter heads a form.
    let mut params: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut rp: BTreeSet<Pos> = BTreeSet::new();
    for &(_, (name, rule)) in &rules {
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
                rp.insert((name.clone(), i));
            }
        }
        params.insert(name, ps);
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
    // from, for a conclusion written `(?T …)` (Q-M1f.42).
    let mut written: BTreeSet<String> = BTreeSet::new();
    for &f in facts.iter().chain(solve_blocks) {
        arguments(ast, f, &mut written);
    }
    for &(_, (_, rule)) in &rules {
        for pat in [rule.match_.as_ref(), rule.assert_.as_ref()]
            .into_iter()
            .flatten()
        {
            arguments(ast, NodeId(pat.expr.0), &mut written);
        }
    }

    // Every argument a conclusion writes, with where the deriving rule's
    // `:match` binds it.
    let mut sites: Vec<Site> = Vec::new();
    for &(head, (name, rule)) in &rules {
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
            let Some(c) = stated(ast, c) else {
                continue;
            };
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
                                    o
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

    // A way binds the argument to a name where one of its occurrences is at a
    // position that holds relation names, or a guard.
    let named = |rp: &BTreeSet<Pos>, rule: &str, occ: &Occ| -> bool {
        match occ {
            Occ::Param(m) => rp.contains(&(rule.to_string(), *m)),
            Occ::Arg { rel, k } => rel == "relation" || rp.contains(&(rel.clone(), *k)),
            Occ::Guard => true,
            Occ::Value => false,
        }
    };
    // The fixpoint: a way that binds the argument nowhere a name is held, but
    // at a marker's argument or the rule's own parameter, makes that position
    // one that holds relation names.
    loop {
        let mut grew = false;
        for s in &sites {
            if !rp.contains(&s.to) || !matches!(ast.node(s.arg), Node::Var(_)) {
                continue;
            }
            for way in &s.ways {
                if way.iter().any(|o| named(&rp, &s.rule, o)) {
                    continue;
                }
                for o in way {
                    let pos = match o {
                        Occ::Param(m) => (s.rule.clone(), *m),
                        Occ::Arg { rel, k } if !declared.contains(rel) => (rel.clone(), *k),
                        _ => continue,
                    };
                    if rp.insert(pos) {
                        grew = true;
                    }
                }
            }
        }
        if !grew {
            break;
        }
    }

    // `eq` at another arity, and each of the three words that head no value
    // (the value rule, Q-M1f.113): `eq`, `absent`, and `false` with an
    // argument.
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
            errors: &mut *errors,
        });
    }

    // What the program passes where a relation name is held: a fact's
    // argument, and a conclusion's.
    let mut places = RelationPlaces {
        passed: BTreeSet::new(),
    };
    for &f in facts {
        let p = positive(ast, f);
        let Some(rel) = relation_of(ast, p) else {
            continue;
        };
        for (k, &a) in ast.form_args(p).iter().enumerate() {
            if let Node::Atom(s) = ast.node(a)
                && rp.contains(&(rel.clone(), k))
            {
                places.passed.insert(ast.sym(s).to_string());
            }
        }
    }
    for s in &sites {
        if let Node::Atom(a) = ast.node(s.arg)
            && rp.contains(&s.to)
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
        rp: &'a BTreeSet<Pos>,
        sites: &'a [Site],
        written: &'a BTreeSet<String>,
        facts: &'a [NodeId],
        errors: &'a mut Vec<Diagnostic>,
    }

    /// **A relation parameter that can be bound to `eq` heads a form of
    /// another arity**.
    ///
    /// `eq` is the relation of equality, between exactly two values
    /// (Q-M1e2.26), and the grammar refuses `(eq ?x)` wherever a premise or a
    /// conclusion is written; but a form whose relation is a parameter,
    /// `(?R ?x)`, is `(eq ?x)` once an activator binds `?R` to `eq`, a name a
    /// fact may hold. Unread here, such a premise would load and meet the
    /// compiler's arity check (`CompileError`) only where the activator's plan
    /// is compiled, at root or only under a commitment, and such a conclusion
    /// would store `(eq a)`.
    ///
    /// Read as the check above reads relation names: a position that holds
    /// relation names can hold `eq` where a fact, an activator among them, or
    /// a conclusion writes `eq` there; or where a conclusion writes a
    /// variable that its `:match` binds, in some disjunct, only at positions
    /// that can hold `eq`, or only with an `eq` or `neq` guard, over the
    /// program's names, while the program writes `eq` as a name; to a
    /// fixpoint. A parameter that can hold `eq` is refused for each premise or
    /// conclusion it heads whose arity is not 2, one line each.
    ///
    /// **The same reading holds a parameter to the value rule** (Q-M1f.113: an
    /// object is an atom, or a fact a state could hold). `c.word` is `eq`, `absent` or
    /// `false`, and a parameter that can hold it is refused for each premise
    /// or conclusion it heads that the grammar refuses written out, `(absent
    /// ?x)` at any arity and `(false ?x)` at one argument or more, and for
    /// each form it heads where a value goes, inside a pattern or a conclusion,
    /// which for `eq` is `(q (?R ?x ?y))`. Unread, `(?R ?x)` concluded with
    /// `?R` bound to `absent` would store `(absent a)`, and `(q (?R ?x ?x))`
    /// with `?R` bound to `eq` would store `(q (eq a a))`.
    fn refuse_eq_at_another_arity(c: EqArity<'_>) {
        /// Why a position can hold `eq`: where it is written, or what passes
        /// it on from a position that can.
        enum EqFrom {
            Fact(NodeId),
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
        // Written: a fact's argument, an activator's among them.
        for &f in c.facts {
            let p = positive(c.ast, f);
            let Some(rel) = relation_of(c.ast, p) else {
                continue;
            };
            for (k, &a) in c.ast.form_args(p).iter().enumerate() {
                let pos = (rel.clone(), k);
                if is_eq(a) && c.rp.contains(&pos) {
                    at.entry(pos).or_insert(EqFrom::Fact(f));
                }
            }
        }
        // A `(relation …)` declaration's argument holds `eq` where its
        // signature writes it as a type, at its index, the name being the
        // first.
        for d in c.program.declarations.iter() {
            for (j, &t) in d.signature.iter().enumerate() {
                if c.terms.sym(t) == w {
                    at.entry(("relation".to_string(), j + 1))
                        .or_insert(EqFrom::Signature(c.terms.sym(d.name).to_string()));
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
                if !c.rp.contains(&s.to) || at.contains_key(&s.to) {
                    continue;
                }
                let from = if is_eq(s.arg) {
                    Some(EqFrom::Conclusion {
                        head: s.head,
                        rule: s.rule.clone(),
                        conclusion: s.conclusion,
                    })
                } else if matches!(c.ast.node(s.arg), Node::Var(_)) {
                    s.ways.iter().find_map(|way| {
                        let named: Vec<Pos> = way
                            .iter()
                            .filter_map(|o| match o {
                                Occ::Param(m) if c.rp.contains(&(s.rule.clone(), *m)) => {
                                    Some((s.rule.clone(), *m))
                                }
                                Occ::Arg { rel, k, .. }
                                    if rel == "relation" || c.rp.contains(&(rel.clone(), *k)) =>
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
        let rules: Vec<(&'static str, (Symbol, &Rule))> =
            c.program.every_rule().map(|r| ("rule", r)).collect();
        for &(head, (name, rule)) in &rules {
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
                        let Some(conclusion) = stated(c.ast, conclusion) else {
                            continue;
                        };
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
                    EqFrom::Fact(_) => {
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
                    // `eq` at another arity is § 2.14's; `false` and
                    // `absent` where no form of them is one are § 2.22's.
                    let kind = if w == "eq" {
                        Kind::EqArity
                    } else {
                        Kind::Object
                    };
                    c.errors.push(refusal(
                        kind,
                        None,
                        format!(
                            "{head} '{rule_name}': its parameter ?{param} heads {}, a form of {n} \
                             argument{}, and {binds} — {why}",
                            crate::dump::dump_compact(c.ast, form),
                            if n == 1 { "" } else { "s" },
                        ),
                    ));
                }
                // A value: a fact a state could hold (the value rule,
                // Q-M1f.113), so never one of `eq`, of `absent` or of
                // `false` with an argument.
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
                    c.errors.push(refusal(
                        Kind::Object,
                        None,
                        format!(
                            "{head} '{rule_name}': its parameter ?{param} heads {}, written \
                             where a value goes, and {binds} — {why}, and a value is a name, or \
                             a fact a state could hold",
                            crate::dump::dump_compact(c.ast, form),
                        ),
                    ));
                }
            }
        }
    }
}

/// **A conclusion `(not ?x)` whose variable can hold a name**, under the
/// value rule (Q-M1f.113: *an object is an atom, or a fact a state could hold: a
/// fact of an ordinary relation, or the stored negation of a fact a state
/// could hold*). `not` of a name is the negation of no fact.
///
/// The grammar takes `(not Var)` as a conclusion, for a variable bound to a
/// fact, `(hypothesis ?h)` then `(not ?h)`; whether it is bound to one depends
/// on the data. Unread, a rule whose variable holds a name would store `(not
/// k)`, which no program can write and whose `--dump-answer` state file would
/// not reload, and a hypothesis would propose it, so R would hold two states
/// that differ in a fact nothing can read. A written `(not k)` is a parse error,
/// and a macro's is refused by [`Shapes::negation`].
///
/// **What is read** (Q-M1f.127). A position, a relation
/// and an argument's index, *holds a name* where the program shows one there:
/// a written fact, at any depth and under a `(not …)`; a `(relation …)`
/// declaration; a rule's conclusion that writes a name, a hypothesis
/// `(open F)` among them, or a variable that can hold one, to a fixpoint; for
/// a rule's parameter, its written activator; and, where the program declares
/// no rule that concludes a hypothesis, every argument of a declared relation the
/// `(solve …)` lists leave among the hypotheses, since a commitment states a
/// fact of one over the program's names. A conclusion
/// variable *can hold a name* where, in one disjunct of the `:match`, every
/// relation premise that binds it binds it at a position that holds a name,
/// or nothing but a guard binds it, `(neq ?x k)` over the program's names and
/// `(eq ?x k)` to the name. A premise under `(absent …)` binds nothing.
///
/// A pattern or a conclusion whose relation is a parameter, `(?R ?x)`, is read
/// as one of each relation a written activator names there: a fact `(deny
/// paid)`, or a conclusion that writes the name.
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
    errors: &mut Vec<Diagnostic>,
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
    // Every rule that concludes facts, once, whole: the obligations conclude
    // what a state owes and no fact.
    let mut sites: Vec<Site<'_>> = Vec::new();
    for (name, rule) in program
        .every_rule()
        .filter(|&(n, _)| !program.obligations.contains(n))
    {
        sites.push(Site {
            head: "rule",
            name: terms.sym(name).to_string(),
            rule,
        });
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
    // A commitment's facts. With no rule that concludes a hypothesis, the
    // candidates are facts
    // of the declared relations the `(solve …)` lists leave in
    // (`ein_infer::hypgen::eligible_relations`), over the program's names: a
    // state can hold a name at each of their positions. Not read where the
    // program writes the closure marker, which takes a relation out of them,
    // nor where it states `(solve :no-search)`, whose lattice is root alone
    // and holds no commitment.
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
    if program.hypotheses.is_empty() && !closure_written && !no_search {
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
            let widest = program
                .signatures(relation.name)
                .map(<[_]>::len)
                .max()
                .unwrap_or(0);
            for i in 0..widest {
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
            errors.push(refusal(
                Kind::NegatedName,
                crate::ast::place(ast, first_loc(ast, node)),
                format!(
                    "{place}{} '{}': its :assert writes {}, and ?{var} can be a name: {how} — not \
                     takes a pattern (relation argument …) or a variable bound to a fact: it is \
                     the stored negation of one fact, and a name is no fact; negation as failure \
                     is (absent …)",
                    site.head,
                    site.name,
                    crate::dump::dump_compact(ast, node),
                ),
            ));
        }
    }
}

/// **A written fact of `eq` at another arity** (Q-M1f.47): `eq` is the
/// relation of equality between two values (Q-M1e2.26).
///
/// `ingest_fact` reads a written fact of `eq`, or of `(not (eq …))`, as a
/// conclusion with no premise, checked and never stored (Q-M1f.33): at two
/// arguments. At any other, `(eq a)`, `(eq)`, `(eq a a a)` or `(not (eq a))`, it
/// is no fact of equality, true or false, and the grammar refuses `(eq ?x)` as
/// a premise and a conclusion; unread here, it would be stored as an ordinary
/// fact. One line per fact, `; `-joined with the loader's others. A
/// nested `(box (eq a))` or `(box (eq a b))` is a parse error: `eq` heads no
/// value.
fn refuse_eq_facts_of_another_arity(ast: &Ast, facts: &[NodeId], errors: &mut Vec<Diagnostic>) {
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
            errors.push(refusal(
                Kind::EqArity,
                None,
                format!(
                    "the fact {} writes eq with {n} argument{} — eq is the relation of equality, \
                     which holds between exactly two values",
                    crate::dump::dump_compact(ast, f),
                    if n == 1 { "" } else { "s" }
                ),
            ));
        }
    }
}

/// **A macro's call is a premise, a conclusion or a goal, and never a fact**
/// ([`07-diagnostics.md` § 2.24](../../../../docs/ein/user/04-reference/07-diagnostics.md#224-a-macros-call-is-no-fact)).
///
/// The loader expands a macro's call in a rule's `:match` and
/// `:assert` and in a query's `:goal` (Q-M1f.130). A fact headed by a macro's
/// name, `(pp j)` beside `(macro pp (?a) (p ?a))`, would otherwise be stored
/// as a fact of a relation `pp`: the fact the body spells would not, no
/// premise could read the one stored, since `(pp ?x)` in a `:match` is the
/// call, and nothing would say so. The same holds at any depth, `(q (pp j))`,
/// under `(not …)` and in a fact's `:using`, of an imported `std.macro` macro,
/// `(unknown (p j))`, and of one the program does not import, which a rule's
/// `:match` is refused for too (Q-M1f.132).
///
/// Refused here, one line per call at the call, and
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
    errors: &mut Vec<Diagnostic>,
) {
    for &form in relations {
        let Some(&first) = ast.form_args(form).first() else {
            continue;
        };
        if let Some(name) = ast.atom_name(first)
            && let Some(m) = declared.get(name)
        {
            errors.push(refusal(
                Kind::MacroFact,
                at_form(ast, form),
                format!(
                    "{}relation '{name}': {name} is a macro too, {} — a premise ({name} …) is the \
                     macro's call, replaced by its body, and a fact ({name} …) is refused, so \
                     nothing could be of the relation; give one of the two another name",
                    placed_form(ast, form),
                    macro_as_written(ast, m),
                ),
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
            errors.push(refusal(
                Kind::MacroFact,
                at_form(ast, call),
                format!(
                    "{}{subject} — {MACRO_CALL_IS_NO_FACT}",
                    placed_form(ast, call)
                ),
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

/// A claim that lists a macro's call (Q-M1f.133). A claim lists facts
/// (Q-M1f.118), so the call is expanded in none: one inside a listed fact,
/// `(model (q (pp j)))`, would load and the claim fail against a state
/// holding `(q (p j))`, the call *no class*. Refused as a written fact's is.
fn refuse_macro_claims(
    ast: &Ast,
    kb: &Kb,
    declared: &BTreeMap<String, IrMacro>,
    stdlib_macros: &[String],
    errors: &mut Vec<Diagnostic>,
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
            errors.push(refusal(
                Kind::MacroFact,
                at_form(ast, call),
                format!(
                    "{}{who}: its :expect lists {}, a call of {what} — a claim lists facts, and \
                     {MACRO_CALL_IS_NO_FACT}",
                    placed_form(ast, call),
                    crate::dump::dump_compact(ast, call),
                ),
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

/// Is `node` a form headed `open`: what a state owes, `(open R)`, or a
/// hypothesis, `(open F)`?
fn is_open_form(ast: &Ast, node: NodeId) -> bool {
    matches!(ast.node(node), Node::SForm { .. }) && ast.head_name(node) == Some("open")
}

/// **The fact a conclusion's conjunct states**, as a check that reads what a
/// state can hold reads it: the conjunct itself, where it is a form; `F` for
/// a hypothesis written as a fact, `(open F)`, which a commitment brings into
/// a state as it is written; and `None` for what a state owes, `(open R)` or
/// `(open ?R)`, and for a hypothesis a variable holds, `(open ?f)`, neither
/// of which writes a form.
fn stated(ast: &Ast, node: NodeId) -> Option<NodeId> {
    if !matches!(ast.node(node), Node::SForm { .. }) {
        return None;
    }
    if !is_open_form(ast, node) {
        return Some(node);
    }
    match ast.form_args(node) {
        [f] if matches!(ast.node(*f), Node::SForm { .. }) => Some(*f),
        _ => None,
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

/// What a rule concludes under `open`, as [`validate_open`] reads it.
enum Opened {
    /// No `(open …)` conjunct.
    Nothing,
    /// `(open R)` or `(open ?R)`, `R` a relation's name or `?R` a
    /// parameter that names one, the rule's whole `:assert`: an
    /// **obligation**.
    Owes,
    /// One `(open F)` conjunct or more, each a hypothesis, `F` a fact or a
    /// variable the `:match` binds; `rest` the conjuncts beside them, which
    /// the rule concludes as any rule does.
    Hypotheses { rest: Vec<NodeId> },
}

/// Load-time validation of what a rule concludes under `open`, and the
/// classification that follows from it (Q-M1f.566, Q-M1f.573, Q-M1f.575).
///
/// Under `open`, read here: **a relation's name, or a parameter, is what an
/// obligation owes**, and the rule's whole `:assert`, which routes it out of
/// the saturation agenda
/// ([the form](../../../../docs/ein/user/03-syntax-and-semantics/08-obligations.md#the-form-open-r));
/// **a fact, or a variable the `:match` binds, is a hypothesis**, which
/// stands in a conjunction like any fact. Errors are pushed rather than
/// returned so one malformed rule does not hide the next.
///
/// The grammar gives `(open …)` one argument, and refuses `(open)` and
/// `open` or `absent` directly under it (`ein.y`'s `Conclusion`, Q-M1f.568);
/// a macro's expansion is held to the same shapes by
/// [`refuse_malformed_primitives`]. What this pass refuses is what only the
/// program can tell:
///
/// 1. **an obligation's `(open …)` beside another conclusion** — what a
///    state owes is a rule's whole `:assert` (Q-M1f.575);
/// 2. **a projection that does not resolve** — `(open ?R)` names the relation
///    whose extent is incomplete and the engine reads the rest out of the
///    rule's own `absent`: exactly one `absent` holding a positive `?R`
///    premise, and exactly one such premise bearing a variable the guard does
///    not already bind. None, two, or a ground body is refused rather than
///    guessed.
///
/// Each line starts with the position the `(open …)` is written at, as the
/// loader's other clause refusals do.
fn validate_open(
    ast: &Ast,
    head: &str,
    name: &str,
    params: &[String],
    match_node: NodeId,
    assert_node: NodeId,
    errors: &mut Vec<Diagnostic>,
) -> Opened {
    let conjuncts = ast.conjuncts(assert_node);
    let opens: Vec<NodeId> = conjuncts
        .iter()
        .copied()
        .filter(|&c| is_open_form(ast, c))
        .collect();
    if opens.is_empty() {
        return Opened::Nothing;
    }
    // What each `(open …)` takes: a name, or a parameter, is a relation the
    // rule owes; anything else a hypothesis. A form with no argument, or
    // more than one, is `refuse_malformed_primitives`' line.
    let owes = |c: NodeId| -> Option<(String, bool)> {
        match ast.form_args(c) {
            [arg] => match ast.node(*arg) {
                Node::Var(s) if params.iter().any(|p| p == ast.sym(s)) => {
                    Some((ast.sym(s).to_string(), true))
                }
                Node::Atom(s) => Some((ast.sym(s).to_string(), false)),
                _ => None,
            },
            _ => None,
        }
    };
    let owed: Vec<(NodeId, (String, bool))> = opens
        .iter()
        .filter_map(|&c| owes(c).map(|o| (c, o)))
        .collect();
    let Some((at, (rel, is_var))) = owed.first().cloned() else {
        let rest = conjuncts
            .iter()
            .copied()
            .filter(|c| !opens.contains(c))
            .collect();
        return Opened::Hypotheses { rest };
    };
    let who = format!("{}{head} '{name}'", placed(ast, at));
    let place = self::at(ast, at);
    let shown = if is_var {
        format!("?{rel}")
    } else {
        rel.clone()
    };
    if conjuncts.len() > 1 {
        errors.push(refusal(
            Kind::Obligation,
            place,
            format!(
                "{who}: `(open {shown})` stands beside another conclusion — what a state owes is \
                 a rule's whole :assert, which the pass reads at the fixpoint and which \
                 concludes no fact: write one rule for the obligation and one for the rest"
            ),
        ));
        return Opened::Owes;
    }
    validate_projection(ast, (&who, place), &rel, is_var, match_node, errors);
    Opened::Owes
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
/// `who` is the line's start, the position and the rule, with the position as
/// a finding's place.
fn validate_projection(
    ast: &Ast,
    (who, place): (&str, Option<Place>),
    rel: &str,
    is_var: bool,
    match_node: NodeId,
    errors: &mut Vec<Diagnostic>,
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
        errors.push(refusal(
            Kind::Obligation,
            place,
            format!(
                "{who}: `(open {shown})` needs an `(absent …)` in :match holding a positive \
                 `{shown}` premise — that premise is the witness the obligation owes"
            ),
        ));
        return;
    }
    if holders.len() > 1 {
        errors.push(refusal(
            Kind::Obligation,
            place,
            format!(
                "{who}: {} `(absent …)` guards hold a positive `{shown}` premise — which one \
                 states the obligation is not decidable",
                holders.len()
            ),
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
            // Only a variable is a slot the `absent` introduces: a bare `_`
            // is refused at the parse (Q-M1e2.22).
            vs.iter().any(|v| !outside.contains(v))
        })
        .collect();
    match witnesses.len() {
        1 => {}
        0 => errors.push(refusal(
            Kind::Obligation,
            place,
            format!(
                "{who}: `(open {shown})`'s `{shown}` premise binds no variable of its own, so \
                 the obligation is ground — that is a plain `absent` check and not something a \
                 witness could discharge"
            ),
        )),
        n => errors.push(refusal(
            Kind::Obligation,
            place,
            format!(
                "{who}: {n} positive `{shown}` premises each bind a witness variable — a \
                 compound witness has no single slot to branch on"
            ),
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
/// this one ([claims about a set of states](../../../../docs/ein/reasoning/02-kb-evolution/10-satisfiability.md#3-claims-about-the-set-of-states)).
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

/// **The `open` facts a program writes** (Q-M1f.566, Q-M1f.66): each held
/// at root alone, in the part of a state no fork inherits, so a fork drops it
/// and no state above root holds it again. `(open F)`, `F` a fact, is a
/// hypothesis the program writes; `(open R)`, `R` a relation of the program,
/// what root owes, written. A name that is no relation of the program is
/// refused: under `open` it is a hypothesis of no fact, and owed by nothing.
fn write_open_facts(
    ast: &Ast,
    terms: &Terms,
    kb: &mut Kb,
    written: &[(FactId, ProvId, NodeId)],
    errors: &mut Vec<Diagnostic>,
) {
    for &(id, prov, form) in written {
        let arg = terms.facts.args(id)[0];
        if let Some(name) = arg.as_sym()
            && !kb.program().relations.contains(name)
        {
            errors.push(refusal(
                Kind::OpenShape,
                at_form(ast, form),
                format!(
                    "{}(open {}) is written, and {} is neither a fact nor a relation of the \
                     program — under open a fact is a hypothesis and a relation's name is what a \
                     state owes",
                    placed_form(ast, form),
                    terms.sym(name),
                    terms.sym(name),
                ),
            ));
            continue;
        }
        kb.write_open(terms, &[id], |_| Some(prov));
    }
}

/// **An obligation made to owe `open`** (Q-M1f.66): an activator the
/// program writes binds the parameter an obligation's `(open ?R)` names to
/// the name `open`. `open` is the kernel's relation, of what a state owes and
/// of its hypotheses, and no obligation owes it: refused, one line per
/// activator, placed where it is written.
fn refuse_open_owed(
    ast: &Ast,
    terms: &Terms,
    kb: &Kb,
    facts: &[NodeId],
    errors: &mut Vec<Diagnostic>,
) {
    for &form in facts {
        let Some(head) = ast.head_name(form) else {
            continue;
        };
        let Some(rule) = terms
            .syms
            .get(head)
            .and_then(|s| kb.program().obligations.get(s))
        else {
            continue;
        };
        let Some(open) = rule.assert_.as_ref().map(|a| NodeId(a.expr.0)) else {
            continue;
        };
        let Some(&arg) = ast.form_args(open).first() else {
            continue;
        };
        let Node::Var(v) = ast.node(arg) else {
            continue;
        };
        let Some(i) = rule.params.iter().position(|&p| terms.sym(p) == ast.sym(v)) else {
            continue;
        };
        let written: Vec<NodeId> = ast
            .form_args(form)
            .iter()
            .copied()
            .filter(|&a| !matches!(ast.node(a), Node::KwPair { .. }))
            .collect();
        if written.get(i).and_then(|&a| ast.atom_name(a)) == Some("open") {
            errors.push(refusal(
                Kind::OpenShape,
                at_form(ast, form),
                format!(
                    "{}{} makes obligation '{head}' owe open — open is the kernel's relation, of \
                     what a state owes and of its hypotheses, and no obligation owes it",
                    placed_form(ast, form),
                    crate::dump::dump_compact(ast, form),
                ),
            ));
        }
    }
}

/// One fact, with its provenance. A fact's relation is a name (`ein.y`'s
/// `FactHead`): the grammar refuses a fact whose relation is not. A written
/// `(open …)` (`ein.y`'s `OpenFact`) is interned and handed back in
/// `written_open`, which [`write_open_facts`] reads once the program's
/// relations are known.
fn ingest_fact(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    form: NodeId,
    written_open: &mut Vec<(FactId, ProvId, NodeId)>,
) -> Result<(), Overflow> {
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

    // **A fact of `eq`, or the negation of one, is checked, never stored**:
    // `eq` and `neq` are relations whose facts are computed, and no fact
    // changes them (Q-M1e2.26). One that holds adds nothing; one that does
    // not is `(false)`, written where it is, as a rule's false conclusion of
    // one is — read as a conclusion with no premise (Q-M1f.31 and Q-M1f.33).
    // Stored, it would be an ordinary fact, which no guard reads.
    // `(neq …)` is no fact the grammar takes.
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

    // An undeclared relation is registered where a fact of it is stated,
    // unless the head is a word of the kernel's, which it reads as a form of
    // its language and never as a relation: a stored `(not X)` is of X's
    // relation, and `eq` and `neq` are computed (Q-M1f.522). The relations
    // written inside a fact or in a rule join after the rules are read
    // (`register_relations`).
    let head = terms.intern_text(&head_name)?;
    if !kb.program().relations.contains(head) && ein_core::terms::heads_a_relation(&head_name) {
        kb.program_mut().add_relation(Relation {
            name: head,
            signature: Box::new([]),
            declared: false,
            why: None,
            loc: core_loc(ast, form),
        });
    }
    let args = fact_args(ast, terms, &args)?;
    if head == terms.kernel.open {
        written_open.push((terms.intern_fact(head, &args)?, prov, form));
        return Ok(());
    }
    kb.add_fact(terms, head, &args, Some(prov))?;
    Ok(())
}

// ── Config ─────────────────────────────────────────────────────────

/// `SolverConfig.from_kw_pairs` — the two flags a program can set, each
/// `true` or `false` (`ein_core::config::FLAGS`). A repeated flag takes its
/// last value.
///
/// **Infallible.** A `(config …)` takes those two keywords and a `true` or a
/// `false` after each (`ein.y`'s `ConfigClause` and `Bool`), so the grammar
/// refuses a body that is not keyword pairs, an unknown flag, a retired flag
/// (its sentence a note on the parse error, read from
/// `ein_core::config::RETIRED`), and a value that is no bool. No flag takes
/// an integer, a number or a string, so none is coerced.
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
    use crate::parse;
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
        kb.expect_err("must fail").to_string()
    }

    fn facts(terms: &Terms, kb: &Kb) -> Vec<String> {
        kb.facts().map(|f| terms.fact_text(f)).collect()
    }

    /// The parser's errors on `text`, which it must refuse, each `line:col:
    /// what`: the grammar refuses many shapes before the loader reads them.
    fn refused(text: &str) -> Vec<String> {
        parse(&mut Ast::new(), text, None)
            .expect_err("the parser refuses it")
            .errors
            .iter()
            .map(|e| format!("{}:{}: {}", e.line, e.col, e.what))
            .collect()
    }

    /// The reserved verdict atom, and the classification it drives
    /// ([the form](../../../../docs/ein/user/03-syntax-and-semantics/08-obligations.md#the-form-open-r)).
    ///
    /// A rule whose `:assert` is `(open …)` and nothing else is an
    /// **obligation**: it derives nothing, so it never enters the saturator's
    /// agenda, and it is read once per quiescent KB after the fixpoint. The
    /// routing is the load-time half of that, and the assertion below is that
    /// `rules` stays empty while `obligations` gets it.
    #[test]
    fn an_open_assert_routes_the_rule_out_of_the_saturation_agenda() {
        let src = "(relation is-a T T :why \"{?3} is a {?4}\")\n(relation r A B :why \"r holds of {?3} and {?4}\")\n\
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
        // The saturation's rules are those every program gets and no other.
        assert!(
            kb.program().rules.values().all(|r| r.implicit) && kb.program().hypotheses.is_empty(),
            "…and to neither of the two that fire during a search"
        );

        // A rule that concludes a hypothesis, `(open F)`, is read by the pass
        // at each state's fixpoint, and its other conclusions are an
        // ordinary rule's (Q-M1f.566, Q-M1f.573).
        let (terms, kb) = ok(
            "(relation r A B :why \"r holds of {?3} and {?4}\")\n(relation s A :why \"s holds of {?3}\")\n(r a b)\n\
             (rule ask () :match (r ?x ?y) :assert (and (open (r ?y ?x)) (s ?x)) :why \"is r of {?y} and {?x}?\")\n",
        );
        let ask = terms.syms.get("ask").expect("interned");
        assert!(
            kb.program().hypotheses.contains(ask) && !kb.program().obligations.contains(ask),
            "a rule that concludes `(open F)` concludes a hypothesis, and owes nothing"
        );
        assert!(
            kb.program()
                .rules
                .get(ask)
                .and_then(|r| r.assert_.as_ref())
                .is_some_and(|a| !a.relation_names.contains(&terms.kernel.open)),
            "…and its other conclusion is an ordinary rule's, with no `open`"
        );

        // An ordinary rule is untouched by any of it.
        let (terms, kb) = ok(
            "(relation r A B :why \"r holds of {?3} and {?4}\")\n(rule copy () :match (r ?a ?b) :assert (r ?b ?a) :why \"r holds of {?a} and {?b}, so r holds of {?b} and {?a}\")\n",
        );
        assert!(
            kb.program()
                .rules
                .contains(terms.syms.get("copy").expect("interned"))
                && kb.program().obligations.is_empty()
        );
    }

    /// **A rule states what it concludes under `:assert`, or a word to the
    /// engine under `:ein`** (Q-M1f.399): the rule is registered among the
    /// saturation's with no conclusion and the word, and the three shapes the
    /// grammar takes and the loader holds are refused, each with the engine's
    /// words listed.
    #[test]
    fn a_rule_states_a_word_to_the_engine_in_the_place_of_an_assert() {
        const P: &str = "(relation p T :why \"p holds of {?3}\")\n";
        let (terms, kb) = ok(&format!(
            "{P}(rule no-p ()\n  :match (p ?x)\n  :ein inconsistent\n  :why \"p holds of {{?x}}, which cannot be\")\n"
        ));
        let rule = kb
            .program()
            .rules
            .get(terms.syms.get("no-p").expect("interned"))
            .expect("a saturation rule");
        assert_eq!(rule.ein, Some(EngineWord::Inconsistent));
        assert!(rule.assert_.is_none(), "it concludes no fact");
        assert!(rule.match_.is_some());
        assert!(kb.program().hypotheses.is_empty() && kb.program().obligations.is_empty());

        // A word that is none of the engine's.
        assert_eq!(
            err(&format!(
                "{P}(rule r ()\n  :match (p ?x)\n  :ein dead\n  :why \"x\")\n"
            )),
            "<string>:4:8: rule 'r': :ein dead — dead is no word of the engine's (the engine's \
             words: inconsistent)"
        );
        // A word of the language is none either: `false` is a name here.
        assert!(
            err(&format!(
                "{P}(rule r ()\n  :match (p ?x)\n  :ein false\n  :why \"x\")\n"
            ))
            .contains(":ein false — false is no word of the engine's"),
        );
        // Beside an `:assert`, in either order.
        for clauses in [
            ":assert (p ?x)\n  :ein inconsistent",
            ":ein inconsistent\n  :assert (p ?x)",
        ] {
            let e = err(&format!(
                "{P}(rule r ()\n  :match (p ?x)\n  {clauses}\n  :why \"x\")\n"
            ));
            assert!(
                e.contains(
                    "rule 'r': states :ein inconsistent beside an :assert — a rule states what \
                     it concludes under :assert, or a word to the engine under :ein, never both"
                ) && e.ends_with("(the engine's words: inconsistent)"),
                "{e}"
            );
        }
    }

    /// **A variable used once is what to write where any value will do**, and
    /// it loads, in a pattern and in an obligation's witness premise.
    ///
    /// A bare `_` is refused at the parse in every position (Q-M1e2.22), so
    /// a wildcard written as a relation or as an argument is a row of the
    /// grammar's table (`ein-ir/tests/grammar_pages.rs`), not a case here.
    #[test]
    fn a_variable_used_once_loads_where_a_wildcard_was_written() {
        let pre = "(relation n T :why \"n holds of {?3}\") (relation hit T :why \"hit holds of {?3}\") (relation box T :why \"box holds of {?3}\") (relation tag T T :why \"tag holds of {?3} and {?4}\")\n(n 2)\n";
        ok(&format!(
            "{pre}(rule r () :match (and (n ?u) (box (tag ?v ?w))) :assert (hit r) :why \"n holds of {{?u}} and box holds of (tag {{?v}} {{?w}}), so hit holds of r\")"
        ));
        ok(&format!(
            "{pre}(rule owe () :match (and (n ?x) (absent (tag ?x ?any))) :assert (open tag) :why \"n holds of {{?x}} and no tag of {{?x}} holds yet — one is still owed\")"
        ));
    }

    /// **A rule shape that reads nothing is refused at load**
    /// (Q-M1e2.18). Each shape in each place it can take, in a rule, a hypothesis rule
    /// and an obligation, one line naming the rule and the clause; and what
    /// stays: a guard on values, a parameter head.
    ///
    /// **Half of the shapes are the grammar's**, each a parse error at its
    /// place and a row of the grammar's table
    /// (`ein-ir/tests/grammar_pages.rs`): `_` as a guard's argument or as a
    /// premise (Q-M1e2.22), a keyword pair as a guard's argument or inside a
    /// pattern (Q-M1e2.28), a name, a variable or a string as a premise. The
    /// loader keeps what a context-free grammar does not see: a form that
    /// holds a variable as a guard's argument (Q-M1f.30) and a relation
    /// variable no parameter binds. An activator binding a relation
    /// parameter to a nested fact loads, and nothing is said of it: it
    /// activates nothing (Q-M1f.1).
    ///
    /// **Two shapes load** (Q-M1e2.26): a fact that holds no variable as a
    /// guard's argument, a value, and a guard on a variable no premise before
    /// it binds, which waits for the premise that does, or ranges over the
    /// program's names where none does.
    #[test]
    fn a_rule_shape_that_reads_nothing_is_refused_at_load() {
        const GUARD: &str = "a guard compares two values, each a variable, a name, or a fact \
                             that holds no variable";
        const FREE: &str = "which no parameter binds — a relation is named by a name or by a \
                            parameter its activator binds";
        let pre = "(relation p T :why \"p holds of {?3}\") (relation q T :why \"q holds of {?3}\") (relation hit T :why \"hit holds of {?3}\") (relation box T :why \"box holds of {?3}\") (relation tag T T :why \"tag holds of {?3} and {?4}\")\n\
                   (relation item T :why \"{?3} is an item\") (relation lbl T :why \"lbl holds of {?3}\")\n(p a) (q a) (box (p a))\n";
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
                "(rule h () :match (p ?r) :assert (open (?r a)) :why \"p holds of {?r}: guess that {?r} holds of a\")".to_string(),
                format!("rule 'h': its :assert writes (?r a) with ?r as its relation, {FREE}"),
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
        // activator is written: a fact, or a literal in an `:assert`. It
        // loads, and a written one is held as the fact it is.
        let mirror = "(rule mirror (?R) :match (box (?R a)) :assert (hit a) :why \"box holds of ({?R} a), so hit holds of a\")";
        for (activator, held) in [
            ("(mirror (p a))", true),
            (
                "(rule mk () :match (q ?x) :assert (mirror (p a)) :why \"q holds of {?x}, so rule mirror is switched on for (p a)\")",
                false,
            ),
        ] {
            let (terms, kb) = ok(&format!("{pre}{mirror}\n{activator}"));
            assert_eq!(
                facts(&terms, &kb).iter().any(|f| f == "(mirror (p a))"),
                held,
                "{activator}"
            );
        }
        // What stays: a guard on values the premises before it bind, a
        // parameter head, a variable used once where any value will do, a
        // guard under an `absent` on a variable the enclosing premises bound;
        // and (Q-M1e2.26) a fact that holds no variable as a guard's
        // argument, and a guard on a variable no premise before it binds, in
        // a conjunction and under an `absent`.
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

    /// **Three shapes the compiler cannot compile are refused at load**. An
    /// `(or …)`
    /// written as a premise, of an `(and …)`, an `(absent …)` or a top-level
    /// `(or …)`, in a rule's, a hypothesis rule's or a goal's clause, written or
    /// produced by a macro (Q-M1f.74); a `:match` or a `:goal` past the
    /// register file, a rule's parameters counted (Q-M1f.73); and a goal's
    /// premise whose relation is a variable, under `and`, `or` and `absent`.
    /// Then what loads: a top-level `(or …)`, an `(or …)` inside a pattern's
    /// argument (no premise), a rule at the ceiling, a goal with its relation
    /// named, and a macro's `(or …)` called as a whole `:match`.
    #[test]
    fn the_compilers_refusals_a_program_reached_are_refused_at_load() {
        let pre = "(relation a T :why \"a holds of {?3}\") (relation p T :why \"p holds of {?3}\") \
                   (relation q T :why \"q holds of {?3}\") (relation out T :why \"out holds of {?3}\")\n(a k)\n";
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
                "(rule h () :match (and (a ?x) (or (p ?x) (q ?x))) :assert (open (out ?x)) :why \"h\")".to_string(),
                nested("rule 'h'", ":match", "and", or, "split the rule"),
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

    /// **A macro's expansion is held to the grammar's shapes** (Q-M1f.67).
    ///
    /// Each shape a macro's body or a call's argument can malform, in a rule's,
    /// a hypothesis rule's and an obligation's `:match` and as a conclusion, refused at
    /// load with one line naming the rule, the clause and the expanded form,
    /// the position where it is written first. Then what loads: an unused
    /// malformed macro (Q-M1f.67), and every shape the grammar takes where a
    /// rule writes it, written and through a macro, so the walk refuses no
    /// written rule the grammar accepts.
    #[test]
    fn a_macros_expansion_is_held_to_the_grammars_shapes() {
        let pre = "(relation a T :why \"a holds of {?3}\") (relation p T :why \"p holds of {?3}\") \
                   (relation q T :why \"q holds of {?3}\")\n(a k)\n";
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
                format!("(rule r () :match (and (a ?x) {call}) :assert (open (p ?x)) :why \"r\")"),
            ]
        };
        let concl = |call: &str| {
            [
                format!("(rule r () :match (a ?x) :assert {call} :why \"r\")"),
                format!("(rule r (?R) :match (?R ?x) :assert {call} :why \"r\")"),
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
                let who = "rule 'r'";
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
        // Five shapes refused by the value rule (Q-M1f.113): a value is a
        // name, or a fact a state could hold, so `eq`, a connective and
        // `false` with an argument head none, and no conclusion is of
        // `absent`. The grammar refuses three written out, and the walk the
        // two it cannot tell from a macro's call. Under an ordinary relation
        // `absent` is an ordinary name (Q-M1f.571), so a value headed by it
        // loads, below.
        for (text, want) in [
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
            "(rule r () :match (and (a ?x) (absent (not (absent (p ?x))))) :assert (p ?x) :why \"r\")",
            "(rule r () :match (and (a ?x) (eq ?x (absent (p k)))) :assert (p ?x) :why \"r\")",
            "(rule r () :match (and (a ?x) (not (eq ?x k))) :assert (p ?x) :why \"r\")",
            "(rule r () :match (and (a ?x) (p (not (q k)))) :assert (p ?x) :why \"r\")",
            "(rule r () :match (and (a ?x) (absent (false))) :assert (p ?x) :why \"r\")",
            // `(not ?x)` of a variable bound to a fact: `h` holds no name.
            // With `(a ?x)`, which `(a k)` binds to a name, it is refused
            // (`a_conclusion_negating_a_name_is_refused_at_load`).
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
    /// load**, under the value rule (Q-M1f.113, Q-M1f.127).
    ///
    /// Each way the program shows a name where the variable is bound: a
    /// written fact, at the top and inside another fact; a declaration; a
    /// parameter's written activator; another rule's conclusion, a name and a
    /// variable that can hold one, a hypothesis among them; a guard alone; one
    /// disjunct of two; a macro's `(not ?a)`; a pattern and a conclusion whose
    /// relation is a parameter, through its written activator; and a declared
    /// relation the hypotheses are taken from. Then what loads: a position
    /// that holds facts alone, two premises of which one holds no name, a
    /// variable bound through the relation of a parameter whose activator is
    /// derived, a premise under `(absent …)` beside one that holds facts, a
    /// guard beside a premise that holds facts, and a declared relation no
    /// commitment states a fact of: held out by a `(solve …)` list, beside a
    /// rule that concludes a hypothesis, or closed.
    #[test]
    fn a_conclusion_negating_a_name_is_refused_at_load() {
        let pre =
            "(relation a T :why \"a holds of {?3}\") (relation p T :why \"p holds of {?3}\")\n";
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
                format!(
                    "(p k)\n(rule mk () :match (p ?y) :assert (open (m ?y)) :why \"mk\")\n{not_m}"
                ),
                "(m ?x) binds ?x where rule 'mk' writes (open (m ?y))",
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
            "(a (p k))\n(rule g () :match (p ?y) :assert (open (p ?y)) :why \"g\")\n\
             (rule r () :match (a ?x) :assert (not ?x) :why \"r\")",
            "(a (p k)) (__closed__ a)\n(rule r () :match (a ?x) :assert (not ?x) :why \"r\")",
        ] {
            ok(&format!("{pre}{text}"));
        }
    }

    /// **A conclusion's every variable is bound, and is refused at load where
    /// it is not**. Each shape, one line per variable naming the rule, the variable
    /// and the two explicit spellings: a free variable, an `(or …)` arm, a
    /// variable seen only under an `(absent …)`, and a hypothesis rule; and a guard's
    /// under an `(absent …)`, a nested conclusion's, an empty `:match`'s, two
    /// variables in one conclusion and a `forall`'s. Then what binds, and
    /// loads: a parameter, a relation premise at any depth, a stored `(not
    /// P)`, an `eq` or `neq` guard (Q-M1e2.26), a typed premise,
    /// `(is-a ?y House)`, and an `(or …)` whose every arm binds.
    #[test]
    fn a_conclusion_variable_nothing_binds_is_refused_at_load() {
        let line = |head: &str, name: &str, var: &str, place: &str| {
            format!(
                "{head} '{name}': its :assert writes ?{var}, which no premise or parameter \
                 binds{place} — to conclude it of the objects a relation holds of, bind it \
                 with a premise such as (R ?{var}), or of every name of the program with \
                 (eq ?{var} ?{var})"
            )
        };
        let pre = "(relation p T :why \"p holds of {?3}\") (relation q T :why \"q holds of {?3}\") (relation hit T :why \"hit holds of {?3}\") (relation box T :why \"box holds of {?3}\") \
                   (relation tag T T :why \"tag holds of {?3} and {?4}\")\n(p a) (q b)\n";
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
            // A hypothesis.
            (
                "(rule h () :match (p ?x) :assert (open (q ?y)) :why \"p holds of {?x}: guess that q holds of a value no premise binds\")",
                line("rule", "h", "y", ""),
            ),
            // Nested in a conclusion's argument, and an empty `:match`.
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
            "(rule h () :match (p ?x) :assert (open (q ?x)) :why \"p holds of {?x}: guess that q holds of {?x}\")",
            "(rule r () :match () :assert (q a) :why \"q holds of a, with no premise\")",
        ] {
            ok(&format!("{pre}{rule}"));
        }
    }

    /// **A program in which a rule could derive an activator binding a
    /// relation parameter to a fact loads, and nothing is said of it**
    /// (Q-M1f.1): such an activator activates nothing, as one of another
    /// count does (Q-M1f.115). Each shape the reading of positions meets
    /// (Q-M1f.41, Q-M1f.42, [`read_relation_places`]): a variable from a
    /// value position, a fact written where a relation is held, a marker and
    /// a parameter that pass a name on, a conclusion whose relation is a
    /// parameter; and the zebra2 and stdlib fan-outs. What the positions are
    /// read for is held by [`eq_at_another_arity_is_refused_at_load`] and by
    /// the relations of the program.
    #[test]
    fn a_derived_activator_binding_a_relation_parameter_to_a_fact_loads() {
        let r = "(rule r (?R) :match (box (?R b)) :assert (hit z) :why \"box holds of ({?R} b), so hit holds of z\")";
        let pre = "(relation box Node :why \"box holds of {?3}\") (relation hit Node :why \"hit holds of {?3}\") (relation item Node :why \"{?3} is an item\") (relation q T T :why \"q holds of {?3} and {?4}\")\n";
        for src in [
            // A variable from a declared relation's argument: `g14`'s two
            // programs, with the fact and without it.
            format!(
                "{pre}{r}\n(rule mk () :match (item ?f) :assert (r ?f) :why \"{{?f}} is an item, so rule r is switched on for {{?f}}\")\n(item (a b))"
            ),
            format!(
                "{pre}{r}\n(rule mk () :match (item ?f) :assert (r ?f) :why \"{{?f}} is an item, so rule r is switched on for {{?f}}\")"
            ),
            // A variable inside a fact pattern, and one under a relation a
            // parameter names; a hypothesis rule derives too.
            format!(
                "{pre}{r}\n(rule mk () :match (box (g ?f)) :assert (r ?f) :why \"box holds of (g {{?f}}), so rule r is switched on for {{?f}}\")"
            ),
            format!(
                "{pre}{r}\n(rule mk (?S) :match (?S ?f) :assert (r ?f) :why \"{{?S}} holds of {{?f}}, so rule r is switched on for {{?f}}\")\n(mk item)"
            ),
            format!(
                "{pre}{r}\n(rule mk () :match (item ?f) :assert (open (r ?f)) :why \"{{?f}} is an item: guess that rule r is switched on for {{?f}}\")"
            ),
            // A marker passes a relation name on: a fact written there, a
            // literal written there, and a variable from a value position
            // written there.
            format!(
                "{pre}{r}\n(rule fan () :match (marker ?m) :assert (r ?m) :why \"the marker names {{?m}}, so rule r is switched on for {{?m}}\")\n(marker (a b))"
            ),
            format!(
                "{pre}{r}\n(rule fan () :match (marker ?m) :assert (r ?m) :why \"the marker names {{?m}}, so rule r is switched on for {{?m}}\")\n\
                 (rule mk () :match (item ?x) :assert (marker (a b)) :why \"{{?x}} is an item, so the marker names (a b)\")\n(item c)"
            ),
            format!(
                "{pre}{r}\n(rule fan () :match (marker ?m) :assert (r ?m) :why \"the marker names {{?m}}, so rule r is switched on for {{?m}}\")\n\
                 (rule mk () :match (item ?x) :assert (marker ?x) :why \"{{?x}} is an item, so the marker names {{?x}}\")"
            ),
            // A rule's parameter passes it on: its activator, written.
            format!(
                "{pre}{r}\n(rule pass (?x) :match () :assert (r ?x) :why \"pass is given {{?x}}, so rule r is switched on for {{?x}}\")\n(pass (a b))"
            ),
            // A conclusion whose relation is a parameter writes into every rule
            // of its arity whose name the program writes (Q-M1f.42).
            format!(
                "{pre}{r}\n(rule copy (?S ?T) :match (?S ?x) :assert (?T ?x) :why \"{{?S}} holds of {{?x}}, so {{?T}} holds of {{?x}}\")\n(copy item r)"
            ),
            // An `(or …)` whose one arm binds it where a name is held.
            format!(
                "{pre}{r}\n(rule mk () :match (or (item ?f) (relation ?f ?a)) :assert (r ?f) :why \"{{?f}} is an item or a declared relation, so rule r is switched on for {{?f}}\")"
            ),
            // A relation signature's name, a rule's relation parameter
            // (zebra2's `co-located-fanout`), a marker that passes a name on
            // and is written with names (the stdlib's `bijective-properties`),
            // an `eq` guard, and a value parameter.
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
        // What the positions are read for beside Q-M1f.46's refusal: a name
        // the program passes where a relation's name is held, through a
        // marker (Q-M1f.41) or through a conclusion whose relation is a
        // parameter (Q-M1f.42), is a relation of the program (Q-M1f.522); one
        // written at a declared relation's argument is not.
        for (src, relation) in [
            (
                format!(
                    "{pre}{r}\n(rule fan () :match (marker ?m) :assert (r ?m) :why \"the marker names {{?m}}, so rule r is switched on for {{?m}}\")\n(marker zork)"
                ),
                true,
            ),
            (
                format!(
                    "{pre}{r}\n(rule copy (?T) :match (marker ?x) :assert (?T ?x) :why \"the marker names {{?x}}, so {{?T}} holds of {{?x}}\")\n(copy r)\n(marker zork)"
                ),
                true,
            ),
            (
                format!(
                    "{pre}{r}\n(rule mk () :match (item ?f) :assert (r ?f) :why \"{{?f}} is an item, so rule r is switched on for {{?f}}\")\n(item zork)"
                ),
                false,
            ),
        ] {
            let (terms, kb) = ok(&src);
            assert_eq!(
                kb.program()
                    .relations_of
                    .iter()
                    .any(|&s| terms.sym(s) == "zork"),
                relation,
                "{src}"
            );
        }
    }

    /// **`eq` at another arity is refused at load** (Q-M1f.46,
    /// Q-M1f.47): a relation parameter that can hold `eq`
    /// heading a premise or a conclusion whose arity is not 2, read as the
    /// check above reads relation names, one line each; and a written fact of
    /// `eq`, or its negation, at another arity.
    #[test]
    fn eq_at_another_arity_is_refused_at_load() {
        const TAIL: &str =
            "— eq is the relation of equality, which holds between exactly two values";
        let pre = "(relation p Node :why \"p holds of {?3}\") (relation hit Node :why \"hit holds of {?3}\") (relation q Node Node :why \"q holds of {?3} and {?4}\")\n(p a)\n";
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
            // of the program, and a hypothesis rule's written activator.
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
                    "{pre}(rule h (?R) :match (?R ?x) :assert (open (hit ?x)) :why \"{{?R}} holds of {{?x}}: guess that hit holds of {{?x}}\")\n(h eq)"
                ),
                format!(
                    "rule 'h': its parameter ?R heads (?R ?x), a form of 1 argument, and the \
                     fact (h eq) binds ?R to eq {TAIL}"
                ),
            ),
            // A written fact, and its negation.
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
        // A parameter heading a pattern inside an argument, a value: a value
        // is no form of `eq`, at any arity. A nested `(eq a)` written out is
        // a parse error.
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
        // What loads: two arguments, the guard (Q-M1e2.26); a declared relation
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

    /// **An obligation refuses every shape it cannot resolve**
    /// (`07-diagnostics.md` § 2.2's table). **The shapes a position states are
    /// the grammar's**: an `(open R)` beside another conclusion, an `(open …)`
    /// of more than one argument or with `open` or `absent` directly under it,
    /// and `open` bound by a declarator, are parse errors at their place
    /// (`open` is a word a position gives a meaning to, Q-M1f.66), each a row
    /// of the grammar's table (`ein-ir/tests/grammar_pages.rs`). Everywhere the
    /// kernel does not read it, under an ordinary relation among them, `open`
    /// is a name (Q-M1f.571). Below are the refusals a context-free grammar
    /// does not see: the projection's ways of not resolving, each placed where
    /// the `(open …)` is written.
    #[test]
    fn the_verdict_atom_refuses_every_shape_it_cannot_resolve() {
        let pre = "(relation is-a T T :why \"{?3} is a {?4}\")\n(relation r A B :why \"r holds of {?3} and {?4}\")\n";
        // Nested rather than concluded, `(not (open ?R))` is no obligation:
        // the rule concludes a stored negation, and loads as any rule that
        // does. An obligation's `(open ?R)` is its rule's whole `:assert`
        // (Q-M1f.575).
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
            // An obligation's `(open R)` is a rule's whole `:assert`
            // (Q-M1f.575).
            (
                "stands beside another conclusion",
                format!(
                    "{pre}(rule bad (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) \
                     (absent (and (?isa ?b ?B) (?R ?a ?b)))) :assert (and (open ?R) (r ?a b)) :why \"{{?a}} has no {{?R}} partner yet\")"
                ),
            ),
            // Written as a premise's relation too, the rule pass refuses it
            // first, as it refuses any relation variable no parameter binds,
            // before this check reads the projection.
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

    /// The stdlib's four obligations — `total-owed` and `surjective-owed` of
    /// `std.algebra`, `slot-owed-room` and `slot-owed-fill` of `std.slots` —
    /// written out here in their shapes, each resolves its projection and is
    /// routed to the obligation pass
    /// ([the stdlib's four](../../../../docs/ein/user/03-syntax-and-semantics/08-obligations.md#the-stdlibs-four)).
    ///
    /// The projection resolves on the shapes the stdlib writes: the two
    /// `std.algebra` duals mirror `total` / `surjective`, and the two
    /// `std.slots` duals mirror `slot-no-room` / `slot-no-fill`, one modality
    /// down — scanning **absence** where those scan a stored `(not …)`, and
    /// saying *unfinished* where they say *dead*.
    #[test]
    fn the_stdlib_obligations_shapes_resolve_their_projection() {
        let src = "(relation is-a T T :why \"{?3} is a {?4}\")\n\
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
            kb.program().rules.values().all(|r| r.implicit),
            "none of the four belongs to the saturation agenda"
        );
    }

    #[test]
    fn a_declaration_stores_itself_as_facts_so_rules_can_read_signatures() {
        let (terms, kb) = ok(
            "(relation r A B :why \"r holds of {?3} and {?4}\")\n(relation bare :why \"bare holds\")",
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

    /// **A relation is any atom the program writes first in a tuple**
    /// (Q-M1f.522), and the registry holds each, declared or not: a stated
    /// fact's (`think`, the activator `unpack`), a fact's inside another (`=`
    /// and `+`, and `p` under a stored negation), a rule's pattern's at any
    /// depth (`said`, `liquid`), and the relation an activator gives a
    /// parameter that heads a form (`happy`). The kernel's words are none:
    /// `not`, `__closed__`, `and`, `false`.
    #[test]
    fn the_registry_holds_every_atom_first_in_a_tuple() {
        let (terms, kb) = ok(
            "(think Bob (= 7 (+ 3 4)))\n(not (p a))\n(__closed__ think)\n\
             (rule unpack (?R) :match (think ?p (?R ?a ?b)) :assert (?R ?a ?b) :why \"{?p} thinks it\")\n\
             (unpack =)\n\
             (rule tag (?R) :match (think ?p ?f) :assert (?R ?p) :why \"{?p} is {?R}\")\n\
             (tag happy)\n\
             (rule pour () :match (and (said ?x (liquid ?x)) (think ?x ?f)) \
             :assert (and (liquid ?x) (not (q ?x))) :why \"{?x} is liquid\")\n\
             (rule never () :match (think ?x ?f) :assert (false) :why \"no one thinks\")",
        );
        let open_world: Vec<&str> = kb
            .program()
            .relations
            .iter()
            .filter(|(_, r)| !r.declared)
            .map(|(n, _)| terms.sym(n))
            .collect();
        assert_eq!(
            open_world,
            [
                "think", "unpack", "tag", "+", "=", "happy", "liquid", "p", "q", "said"
            ],
            "the stated facts' relations in their order, then every other in the order of its text"
        );
    }

    /// A nested form is a fact, interned and not believed. A nested fact's
    /// head is a name: `(p (?q a))`, a variable in a fact, is refused at the
    /// parse, so no fact reaches the loader with a nested head that is not a
    /// name, and the `#nested` rename such a head collapses to is reached by
    /// no program.
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
        // the fact list, and the fact that holds it is.
        assert_eq!(kb.n_facts(), 2);
        let negation = kb.facts().next().expect("a fact");
        let inner = terms.facts.args(negation)[0].as_fact().expect("nested");
        assert!(!kb.contains(inner));
        assert_eq!(
            terms.facts.probe(terms.kernel.not, &[Value::fact(inner)]),
            Some(negation)
        );
        assert!(kb.contains(negation));
    }

    /// A fact's `:using` lists its premises, each a fact: `:using ((r a
    /// b))`. A list with a head, `:using (p (r a b))`, is refused at the
    /// `p`.
    #[test]
    fn provenance_comes_from_source_or_from_rule_and_using() {
        assert_eq!(
            refused("(relation r T T)\n(r e f :rule x :using (p (r a b)))"),
            ["2:24: unexpected `p`: a name"]
        );
        let (terms, kb) = ok(
            "(relation r T T :why \"r holds of {?3} and {?4}\")\n(r a b :source \"(1)\")\n(r c d)\n\
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
        assert_eq!(terms.fact_text(prov.premises[0]), "(r a b)");
    }

    /// A `:source` must be a string and a `:rule` a name. **Each keyword's
    /// value is the grammar's**, so in `(r a b :source sentence :rule "x")`
    /// the bare `sentence` is refused at the parse, and so is the string
    /// `"x"`. What reads as absent is a `:source` with no `:rule`: a given
    /// fact, its sentence kept.
    #[test]
    fn a_present_but_unusable_annotation_reads_as_absent() {
        assert_eq!(
            refused("(relation r T T)\n(r a b :source sentence :rule \"x\")")[0],
            "2:16: unexpected `sentence`: a name"
        );
        let (terms, kb) =
            ok("(relation r T T :why \"r holds of {?3} and {?4}\")\n(r a b :source \"sentence\")");
        let fact = kb.facts().last().expect("a fact");
        let prov = terms.provs.get(kb.primary(fact).expect("recorded"));
        assert_eq!(prov.kind, ProvKind::Source);
        assert_eq!(
            prov.source.map(|s| terms.sym(s).to_string()).as_deref(),
            Some("sentence")
        );
    }

    /// Every `(query …)` loads, and the last `(config …)` wins. A second
    /// query discarded in silence is the one thing a file carrying `:expect`
    /// must not do; the config half is here in the same test because the
    /// *contrast* is the decision — a config is a setting, a query is
    /// content.
    #[test]
    fn every_query_is_kept_and_the_last_config_still_wins() {
        // The facts make `a` and `b` relations of the program, which a goal
        // asks about.
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

    // ── `:expect`, at load ─────────────────────────────────────────
    //
    // Every one of these is a load *error* rather than a run that checks
    // nothing, which is the whole reason the keyword exists. An unknown query
    // keyword and a malformed `:expect` are parse errors, rows of the
    // grammar's table (`ein-ir/tests/grammar_pages.rs`); what is here is what
    // a grammar cannot see.

    #[test]
    fn an_expect_naming_a_relation_the_program_does_not_have_is_a_load_error() {
        let e = err(
            "(relation p T P :why \"p holds of {?3} and {?4}\")\n(p A H)\n\
             (query :goal (p A ?h) :expect (model (p A H) (nosuch A)))",
        );
        assert!(e.contains("nosuch"), "{e}");
        assert!(e.contains("the program names no relation nosuch"), "{e}");
        // A relation only a rule's `:assert` derives is the program's:
        // a claim may list a fact of it.
        ok(
            "(relation p T P :why \"p holds of {?3} and {?4}\")\n(p A H)\n\
             (rule r () :match (p ?a ?h) :assert (won ?a) :why \"{?a} won\")\n\
             (query :goal (won ?a) :expect (model (won A)))",
        );
    }

    /// A query's `:expect` lists projection classes —
    /// the goal's atoms under its rows — so a listed fact the goal does not
    /// match is refused, and the message names where a claim about the state
    /// belongs.
    #[test]
    fn a_query_claim_lists_only_facts_its_goal_matches() {
        let e = err(
            "(relation p T P :why \"p holds of {?3} and {?4}\")\n(relation q T P :why \"q holds of {?3} and {?4}\")\n(p A H)\n(q A H)\n\
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
            "(relation p T P :why \"p holds of {?3} and {?4}\")\n(relation q T P :why \"q holds of {?3} and {?4}\")\n(p A H)\n(q A H)\n\
             (query :goal (p A ?h) \
              :expect (or (model (p A H)) (model (q A H))))",
        );
        assert!(e.contains("(q A H), which no atom"), "{e}");
        // A goal's constant must match, and a variable bind one value.
        let e = err(
            "(relation p T P :why \"p holds of {?3} and {?4}\")\n(p A H)\n(p B H)\n\
             (query :goal (p A ?h) :expect (model (p B H)))",
        );
        assert!(e.contains("(p B H), which no atom"), "{e}");
        let (_terms, _kb) = ok(
            "(relation p T P :why \"p holds of {?3} and {?4}\")\n(p A H)\n\
             (query :goal (and (p ?x ?h) (p ?x H)) :expect (model (p A H)))",
        );
    }

    /// A class is listed once, and `:over` is one of three words.
    #[test]
    fn a_class_twice_and_an_unknown_over_are_load_errors() {
        let e = err(
            "(relation p T P :why \"p holds of {?3} and {?4}\")\n(p A H)\n\
             (query :goal (p A ?h) :expect (or (model (p A H)) (model (p A H))))",
        );
        assert!(e.contains("lists the class (model (p A H)) twice"), "{e}");
        let e = err(
            "(relation p T P :why \"p holds of {?3} and {?4}\")\n(p A H)\n(query :goal (p A ?h) :over best)",
        );
        assert!(
            e.contains(":over best — one of all, minimal, maximal"),
            "{e}"
        );
        for over in ["all", "minimal", "maximal"] {
            ok(&format!(
                "(relation p T P :why \"p holds of {{?3}} and {{?4}}\")\n(p A H)\n(query :goal (p A ?h) :over {over})"
            ));
        }
    }

    /// A query with no `:expect` is untouched by any of the above — the
    /// corpus is 128 entries of exactly that.
    #[test]
    fn a_query_without_expect_is_unaffected() {
        let (_terms, kb) = ok(
            "(relation p T P :why \"p holds of {?3} and {?4}\")\n(p A H)\n(query :goal (p A ?h))",
        );
        assert_eq!(kb.program().queries.len(), 1);
    }

    /// A later query, here one with `:over all`, is reached by its index.
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
            "(relation r T T :why \"r holds of {?3} and {?4}\")\n(relation s T T :why \"s holds of {?3} and {?4}\")\n\
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

    /// Every pass accumulates rather than stopping at the first problem,
    /// held with problems the loader owns, not the grammar: a macro declared
    /// twice (pass 0), a relation and a rule named after a kernel dunder
    /// (passes 1 and 2). `(relation)` with no name and a rule
    /// named `absent` are the grammar's, rows of its table.
    #[test]
    fn every_pass_accumulates_rather_than_stopping_at_the_first_problem() {
        let message = err(
            "(macro m (?a) (x ?a) :why \"x holds of {?a}\")\n(macro m (?a) (y ?a) :why \"y holds of {?a}\")\n(relation __closed__ :why \"closed\")\n\
             (rule __closed__ () :match (x ?a) :assert (y ?a) :why \"x holds of {?a}, so y holds of {?a}\")",
        );
        assert_eq!(
            message,
            "duplicate macro 'm' at None; \
             relation '__closed__' shadows a reserved kernel name at None; \
             rule '__closed__' shadows a reserved kernel name at None"
        );
    }

    /// **A macro's call written as a fact is refused at load**: a fact headed by a macro's name, at any depth, one of
    /// `std.macro`'s the program does not import, a relation declared under a
    /// macro's name, and a call inside a fact a claim lists. A call in a
    /// `:match`, an `:assert` or a `:goal` is expanded, and the name written
    /// as a value is a name.
    #[test]
    fn a_macros_call_written_as_a_fact_is_refused_at_load() {
        let pre = "(relation p T :why \"p holds of {?3}\") (relation q T :why \"q holds of {?3}\")\n\
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
                "(relation pp T :why \"pp holds of {?3}\")",
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
            kb.unwrap_or_else(|e| panic!("{text}: {e}"));
        }
    }
}
