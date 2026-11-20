//! `ein render` — DOT views of rules / constraints / the search lattice.
//!
//! Every body is the same three steps: load, call one renderer, write it with
//! a trailing newline. **Every view reads the program through the loader**
//! (Q-M1f.71), so a program the loader refuses is refused here with its
//! line, exit 1, as every other command refuses it; the three that draw a
//! program as written (`rules`, `rule`, `program`) draw it from its parsed
//! forms after that, its imports unresolved, and so never draw a program
//! every other command refuses. `kb` and `constraints` draw a knowledge base
//! the program loads and saturates to. The renderers themselves are `ein-render`'s
//! ([01-architecture.md § 3.4](../../../../docs/ein/ein.rs/01-architecture.md#34-ein-einb-ein-render-ein-cli));
//! what this adds is the surface around them.

use std::path::Path;

use ein_core::Terms;
use ein_ir::Ast;
use ein_ir::NodeId;
use ein_render::{RuleMode, render_constraints, render_rule_form, render_rules_forms};

use crate::common::{load_program, parse_program, rule_forms};

/// stdout with the trailing newline every `render` body writes after the DOT.
fn emit(dot: &str) {
    println!("{dot}");
}

/// Is `file` refused by the loader? Then its line is printed, as every
/// command that loads prints it, and the view exits 1 (Q-M1f.71).
///
/// The load is a check, in arenas of its own: a view draws the program as
/// written, so what it draws is read by [`parse_program`] after it, which a
/// load would have expanded.
fn refused(file: &str, load: &ein_ir::LoadOptions) -> bool {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    load_program(&mut ast, &mut terms, Path::new(file), load).is_none()
}

pub fn cmd_rules(file: &str, mode: RuleMode, fold: bool, load: &ein_ir::LoadOptions) -> i32 {
    if refused(file, load) {
        return 1;
    }
    let mut ast = Ast::new();
    let Some(forms) = parse_program(&mut ast, Path::new(file)) else {
        return 1;
    };
    let rules = rule_forms(&ast, &forms);
    if rules.is_empty() {
        eprintln!("no rule forms in {file}");
        return 1;
    }
    emit(&render_rules_forms(&ast, &rules, mode, fold));
    0
}

pub fn cmd_rule(
    file: &str,
    name: &str,
    mode: RuleMode,
    fold: bool,
    load: &ein_ir::LoadOptions,
) -> i32 {
    if refused(file, load) {
        return 1;
    }
    let mut ast = Ast::new();
    let Some(forms) = parse_program(&mut ast, Path::new(file)) else {
        return 1;
    };
    for r in rule_forms(&ast, &forms) {
        // The rule's name is its *first argument*, and it has to be an atom —
        // a `(rule "x" …)` is not addressable by `--name`.
        let first = ast.form_args(r).first().copied();
        if first.and_then(|a| ast.atom_name(a)) == Some(name) {
            emit(&render_rule_form(&ast, r, mode, fold));
            return 0;
        }
    }
    eprintln!("no rule named {} in {file}", ein_core::repr::repr_str(name));
    1
}

/// `ein render constraints` — **the facts about relations** of the program's
/// root at its fixpoint (Q-M1f.530): every fact root holds with a relation of
/// the program directly under an argument arrow
/// ([`ein_infer::slice::about_relations`], Q-M1f.523), drawn by the KB view's
/// code with what each points at ([`render_constraints`]). Root holds its
/// `open` facts (Q-M1f.572), so `(open R)`, what root owes, is one such
/// fact. A saved file's root is its program's, the choices it states not
/// committed.
pub fn cmd_constraints(file: &str, fold: bool, load: &ein_ir::LoadOptions) -> i32 {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Some(mut kb) = load_program(&mut ast, &mut terms, Path::new(file), load) else {
        return 1;
    };
    if let Err(e) = ein_infer::open::root_state(&ast, &mut terms, &mut kb) {
        eprintln!("{}", root_error_line(&e));
        return 1;
    }
    emit(&render_constraints(&kb, &terms, "constraints", fold));
    0
}

/// The line a root that does not saturate, or whose `open` facts cannot be
/// written, is refused with: the line `ein saturate` and `ein solve` print,
/// not behind the file's name.
fn root_error_line(e: &ein_infer::error::SolveError) -> String {
    match e {
        ein_infer::error::SolveError::Saturate(e) => crate::common::saturate_error_line(e),
        e => e.to_string(),
    }
}

/// `ein render lattice FILE` — the nodes a describing `ein solve` run visits,
/// drawn by [`ein_render::dump::region_lattice::dot`] from what the route's
/// watch saw: the same nodes, in the same order, as `--dump-lattice`. A run
/// that reaches its budget is drawn as far as it
/// went.
pub fn cmd_lattice(file: &str, full: bool, budget: Option<u64>, load: &ein_ir::LoadOptions) -> i32 {
    use std::sync::{Arc, Mutex};

    use ein_infer::explore::{self, Ask, Mode, Which};
    use ein_render::dump::region_lattice::{Inputs, Visits, dot};

    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Some(mut kb) = load_program(&mut ast, &mut terms, Path::new(file), load) else {
        return 1;
    };
    let visits = Arc::new(Mutex::new(Visits::default()));
    let mut events = ein_infer::events::Events::off()
        .watched(Box::new(crate::modes::Watcher::collecting(visits.clone())));
    let mut opts = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    });
    // Each dead node drawn with its core, as `--dump-lattice` asks.
    opts.cores = ein_infer::region::Cores {
        visited: true,
        nogoods: false,
    };
    if let Some(b) = budget {
        opts.budget = b;
    }
    let answer = match explore::run_with(&mut kb, &mut terms, &ast, &opts, &mut events) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    if !answer.monotonicity.is_monotone() {
        eprintln!("{}", ein_infer::detector::warning_line(file));
    }
    let v = visits.lock().expect("the run is done").clone();
    let candidates: Vec<String> = answer.candidates.iter().map(|c| c.text.clone()).collect();
    emit(&dot(
        &v,
        &Inputs {
            source: file,
            argv: file,
            route: crate::modes::route_words(&answer),
            candidates: &candidates,
        },
        full,
    ));
    0
}

/// `ein render program` — the program **as written**, by
/// [`ein_render::ir_to_dot`] (Q-M1e.20), in three parts: its knowledge base,
/// drawn as `ein render kb` draws one, its rules, and its query (Q-M1f.420).
///
/// Read through the loader first (Q-M1f.71), so a program the loader
/// refuses is refused, with its line, `examples/broken/load/`'s among them.
/// What it draws
/// is the parsed forms, so imports are not resolved: the stdlib's forms would
/// drown the program's own.
pub fn cmd_program(file: &str, mode: RuleMode, fold: bool, load: &ein_ir::LoadOptions) -> i32 {
    if refused(file, load) {
        return 1;
    }
    let mut ast = Ast::new();
    let Some(forms) = parse_program(&mut ast, Path::new(file)) else {
        return 1;
    };
    let opts = ein_render::DotOpts {
        rule_mode: mode,
        fold,
        ..ein_render::DotOpts::default()
    };
    let dot = ein_render::ir_to_dot(&ast, &forms, opts);
    if dot.trim().is_empty() {
        eprintln!("no renderable forms in {file}");
        return 1;
    }
    emit(&dot);
    0
}

/// What `ein render kb` was asked.
pub struct KbArgs<'a> {
    pub file: &'a str,
    /// `--since FILE`.
    pub since: Option<&'a str>,
    pub colour_by: ein_render::ColourBy,
    pub fold: bool,
    /// `--goal '(…)'`: a goal given beside the program, whose slice is drawn.
    pub goal: Option<&'a str>,
    /// `--query N`: the program's `N`-th query, from 1, whose goal's slice is
    /// drawn.
    pub query: Option<usize>,
}

/// `ein render kb` — one KB as one digraph, by [`ein_render::kb_to_dot`]:
/// `file` loaded and saturated (Q-M1e.20), its state's `open` facts with it
/// (Q-M1f.572).
///
/// **A goal's slice** (Q-M1f.529): under `--goal` or `--query N`, the facts
/// of the knowledge base that answer the goal
/// ([`ein_infer::slice::answering`]), drawn with what they point at. `--goal`
/// is read as the `:goal` of one more `(query …)` of the program and refused
/// as that query's would be ([`ein_ir::read_goal`]); `--query N` is the goal
/// of the program's `N`-th query, as loaded, its `:over` and `:expect` not
/// read. Either is matched against the knowledge base drawn, root's fixpoint
/// or a saved file's state, never against R.
///
/// **A model is a file**, not a `--state model:N` here. `ein solve
/// --dump-answer` saves each recorded state as its program and the choices
/// made, which load to the state again (Q-M1f.402), so `render kb
/// DIR/models/02.ein --since DIR/models/01.ein` draws model 2 with what it
/// holds beyond model 1 in bold — from the one solve whose answer the
/// directory holds, rather than from a second solve this command would have
/// to be handed the same flags for.
pub fn cmd_kb(a: &KbArgs<'_>, load: &ein_ir::LoadOptions) -> i32 {
    let KbArgs {
        file,
        since,
        colour_by,
        fold,
        ..
    } = *a;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Some(kb) = loaded_and_saturated(&mut ast, &mut terms, file, load) else {
        return 1;
    };
    let goal = match goal_of(&mut ast, &terms, &kb, a) {
        Ok(goal) => goal,
        Err(line) => {
            eprintln!("{line}");
            return 1;
        }
    };
    let only = match goal {
        Some(goal) => match ein_infer::slice::answering(&ast, &mut terms, &kb, goal) {
            Ok(facts) => Some(facts),
            Err(e) => {
                eprintln!("{e}");
                return 1;
            }
        },
        None => None,
    };
    // One `Terms` for both, so a fact is the same `FactId` in each and the
    // renderer's `since` comparison is by identity.
    let before = match since {
        Some(path) => match loaded_and_saturated(&mut ast, &mut terms, path, load) {
            Some(kb) => Some(kb),
            None => return 1,
        },
        None => None,
    };
    let opts = ein_render::KbDotOpts {
        colour_by,
        fold,
        since: before.as_ref(),
        only: only.as_deref(),
        name: if only.is_some() { "goal" } else { "kb" },
    };
    emit(&ein_render::kb_to_dot(&kb, &terms, &opts));
    0
}

/// The goal `ein render kb` was given, read against the program `kb` is of:
/// `--goal`'s, held to what a query's `:goal` is held to, or the `--query
/// N`'s goal; `None` for neither. `Err` is the line the command prints before
/// it exits 1.
fn goal_of(
    ast: &mut Ast,
    terms: &Terms,
    kb: &ein_core::Kb,
    a: &KbArgs<'_>,
) -> Result<Option<NodeId>, String> {
    if let Some(text) = a.goal {
        let query = ein_ir::read_goal(ast, terms, kb, text, "--goal").map_err(|e| e.to_string())?;
        return Ok(ein_infer::verdict::query_value(ast, &query, "goal"));
    }
    let Some(n) = a.query else {
        return Ok(None);
    };
    let queries = &kb.program().queries;
    let query = n
        .checked_sub(1)
        .and_then(|i| queries.get(i))
        .ok_or_else(|| {
            format!(
                "--query {n}: {} has {} (query …) form{}",
                a.file,
                queries.len(),
                if queries.len() == 1 { "" } else { "s" }
            )
        })?;
    ein_infer::verdict::query_value(ast, query, "goal")
        .map(Some)
        .ok_or_else(|| format!("--query {n}: the query has no :goal"))
}

fn loaded_and_saturated(
    ast: &mut Ast,
    terms: &mut Terms,
    file: &str,
    load: &ein_ir::LoadOptions,
) -> Option<ein_core::Kb> {
    let mut kb = load_program(ast, terms, Path::new(file), load)?;
    // A file with no `(commit …)`: root's state, its fixpoint and the `open`
    // facts the pass writes there (Q-M1f.572).
    if kb.program().commit.is_none() {
        return match ein_infer::open::root_state(ast, terms, &mut kb) {
            Ok(()) => Some(kb),
            Err(e) => {
                eprintln!("{}", root_error_line(&e));
                None
            }
        };
    }
    // The line `ein saturate` and `ein solve` print, not behind the file's
    // name.
    if let Err(e) = ein_infer::saturate_events(ast, terms, &mut kb) {
        eprintln!("{}", crate::common::saturate_error_line(&e));
        return None;
    }
    // A saved file's state: its node, committed and saturated by the node
    // function, its `open` facts written at its fixpoint (Q-M1f.402).
    match ein_infer::region::saved_node(&mut kb, terms, ast) {
        Ok(Some(node)) => Some(node.kb),
        Ok(None) => Some(kb),
        Err(e) => {
            eprintln!("{e}");
            None
        }
    }
}

/// What `ein render h-space` was asked.
pub struct HSpaceArgs<'a> {
    pub file: &'a str,
    /// One of the views the parser lists: `summary`, a graph's name, `json`.
    pub view: &'a str,
    /// `--dump DIR`.
    pub dump: Option<&'a str>,
    /// `--supports`.
    pub supports: bool,
    /// `--region-budget`, each layer's; the routes' default where absent.
    pub budget: Option<u64>,
    pub jobs: usize,
}

/// `ein render h-space FILE` — the program's hypothesis space, its
/// candidates and the structure of the set: every object
/// [`ein_infer::structure`] reads of it, by one call
/// ([`ein_infer::structure::whole`]), spelled by [`ein_render::structure`].
///
/// stdout holds the view asked: the read-out, one graph as DOT, or the
/// document. `--dump DIR` writes the document and the four graphs beside,
/// and moves nothing on stdout or stderr: the directory is made before the
/// work, so a path that cannot be made is exit 1 before anything is read,
/// and a write that fails after it reports itself and leaves the exit code,
/// as `ein oracle --dump`'s does.
///
/// **A structure that is not whole is said three times**: each thing a
/// budget or a bound stopped, cut or left unread is a `note:` on stderr, a
/// row of the read-out, and a sentence of the document's `unread`, and each
/// graph's title counts them. Every structure exits 0, whole or not; a
/// program the loader refuses is refused as every command refuses it, exit
/// 1, and so is one the engine refuses to read.
pub fn cmd_h_space(a: &HSpaceArgs<'_>, load: &ein_ir::LoadOptions) -> i32 {
    use ein_infer::structure::{self, Options, WholeOptions};
    use ein_render::structure as shown;

    if let Some(dir) = a.dump
        && let Err(e) = std::fs::create_dir_all(dir)
    {
        crate::solve::artefact_failed("dump", dir, e);
        return 1;
    }
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Some(mut kb) = load_program(&mut ast, &mut terms, Path::new(a.file), load) else {
        return 1;
    };
    crate::common::program_of_saved(&mut kb);
    let budget = a.budget.unwrap_or(ein_infer::downset::DEFAULT_BUDGET);
    let asked = WholeOptions {
        layer: Options {
            budget,
            jobs: a.jobs,
            deadline: None,
        },
        supports: a.supports,
    };
    let whole = match structure::whole(&mut kb, &mut terms, &ast, &asked) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let inputs = shown::Inputs {
        program: a.file,
        budget,
        supports: a.supports,
    };
    let graphs = shown::graphs(&whole, a.file);
    // A flagged program is said first, as every command that answers about a
    // commitment says it (Q-M1f.564).
    if !whole.theory.root.monotone {
        eprintln!("{}", ein_infer::detector::warning_line(a.file));
    }
    for sentence in shown::unread(&whole) {
        eprintln!("note: {}: {sentence}", a.file);
    }
    match a.view {
        "json" => print!("{}", shown::document(&whole, &terms, &inputs, &graphs)),
        "summary" => print!("{}", shown::summary(&whole, &inputs, &graphs)),
        name => match graphs.iter().find(|g| g.name == name) {
            Some(g) => print!("{}", g.dot),
            // The parser lists the views, so no other word reaches here.
            None => return 2,
        },
    }
    if let Some(dir) = a.dump {
        let dir = Path::new(dir);
        let mut files = vec![(
            shown::DOCUMENT.to_string(),
            shown::document(&whole, &terms, &inputs, &graphs),
        )];
        files.extend(
            graphs
                .iter()
                .map(|g| (shown::graph_file(g.name), g.dot.clone())),
        );
        for (name, text) in files {
            if let Err(e) = std::fs::write(dir.join(&name), text) {
                crate::solve::artefact_failed("dump", &dir.join(&name).to_string_lossy(), e);
            }
        }
    }
    0
}

/// `ein render region` — an `ein oracle --dump` directory drawn: the cube by
/// [`ein_render::region_dot`], with a region run joined to it by
/// [`ein_render::region_doc::Overlay`] when `--visited` (the run's
/// `--dump-lattice`) or `--answer` (its `--dump-answer`) names one.
///
/// Reads those files and nothing else — no load, no solve — so the picture
/// is the run's, and a directory from another checkout draws the same.
pub fn cmd_region(dir: &str, view: &str, visited: Option<&str>, answer: Option<&str>) -> i32 {
    let doc = match ein_render::region_doc::read_dir(Path::new(dir)) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let run = match ein_render::region_doc::RunDoc::read(
        answer.map(Path::new),
        visited.map(Path::new),
        &doc,
    ) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let view =
        ein_render::region_dot::View::parse(view).unwrap_or(ein_render::region_dot::View::Auto);
    let out = ein_render::region_dot::render(&doc, &run, view);
    print!("{out}");
    0
}
