//! What the two corpus censuses of the static check share — M1e2
//! [S1e2.1b.1](../../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.1_the_site_census.md):
//! `refutation_census` (S1e2.4.1) and `site_census` (S1e2.1b.1) select the
//! same files, load them the same way, and read the same root fixpoint, so
//! that one's rows can be re-derived from the other's.
//!
//! [`root_of`] is `region::prepare` as a reader outside the crate can run it:
//! root saturated to quiescence, the universe taken config-free, root's
//! obligation tally (which compiles the obligation plans), and
//! [`monotone::classify`] over every plan the memo then holds. It copies no
//! logic: each step is the library's own function.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use ein_core::{FactId, Kb, Symbol, Terms, Value};
use ein_corpus::repo_root;
use ein_infer::compile::SharedMemo;
use ein_infer::events::Events;
use ein_infer::firing::Firing;
use ein_infer::hypgen::{HypGenStats, generate_config_free};
use ein_infer::monotone::{self, Monotonicity};
use ein_infer::obligations::{self, Owes};
use ein_infer::plan::Plan;
use ein_infer::saturator::{Saturator, Session};
use ein_ir::{Ast, load, parse};

/// A program's own load, or `None` for a file that does not parse or load
/// (the negative fixtures).
pub fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

/// The selection: `corpus_files()` plus the register's witnesses, sorted and
/// distinct.
pub fn selection() -> Vec<PathBuf> {
    let mut files = ein_corpus::corpus_files();
    files.extend(register_witnesses());
    files.sort();
    files.dedup();
    files
}

/// Every file a register row names as a witness, sorted and distinct.
pub fn register_witnesses() -> Vec<PathBuf> {
    let mut files = Vec::new();
    for row in ein_corpus::register().case {
        for (_, a) in row.answers() {
            files.push(repo_root().join(row.witness_of(a)));
        }
    }
    files.sort();
    files.dedup();
    files
}

/// `path`, relative to the repository.
pub fn relative(path: &Path) -> String {
    path.strip_prefix(repo_root())
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}

/// Root's fixpoint, and what the static check reads off it.
pub struct Root {
    /// The firings root's saturation made, in order.
    pub firings: Vec<Firing>,
    /// The universe, config-free, in the generator's order.
    pub proposed: Vec<FactId>,
    /// The rung of the generation ladder it came from.
    pub rung: ein_infer::oblgen::Mode,
    /// What root owes.
    pub owes: Owes,
    /// Every plan the memo holds after root's saturation and tally.
    pub plans: Vec<Arc<Plan>>,
    pub monotonicity: Monotonicity,
}

/// Saturate `kb`, a freshly loaded program, to quiescence; hand the fixpoint
/// to `at_fixpoint`; take the universe; tally root's debts; classify. `None`
/// where any step fails, as `region::prepare` would.
pub fn root_of<T>(
    kb: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    at_fixpoint: impl FnOnce(&Saturator, &mut Session<'_>) -> Option<T>,
) -> Option<(Root, T)> {
    let memo = SharedMemo::default();
    let mut events = Events::off();
    let mut firings = Vec::new();
    let read = {
        let mut s = Session {
            kb,
            terms,
            ast,
            events: &mut events,
            memo: memo.clone(),
        };
        let mut sat = Saturator::new(&mut s).ok()?;
        sat.saturate(&mut s, None, &mut |f| firings.push(f.clone()))
            .ok()?;
        at_fixpoint(&sat, &mut s)?
    };
    let mut proposed = Vec::new();
    let rung = {
        let mut s = Session {
            kb,
            terms,
            ast,
            events: &mut events,
            memo: memo.clone(),
        };
        let mut stats = HypGenStats::new();
        generate_config_free(&mut s, &mut stats, &mut |f| {
            proposed.push(f);
            std::ops::ControlFlow::Continue(())
        })
        .ok()?;
        stats.rung.mode
    };
    let owes = obligations::tally(kb, terms, ast, &memo, &mut events).ok()?;
    let hypothesised: rustc_hash::FxHashSet<Symbol> =
        proposed.iter().map(|&f| terms.facts.get(f).0).collect();
    let plans = monotone::plans_of(&memo);
    let monotonicity = monotone::classify(kb, terms, &plans, &hypothesised);
    Some((
        Root {
            firings,
            proposed,
            rung,
            owes,
            plans,
            monotonicity,
        },
        read,
    ))
}

/// `(false)` or `(not …)`, from every conclusion, `(false)` outranking.
pub fn refutes(plan: &Plan, terms: &Terms) -> Option<&'static str> {
    let w = monotone::writes_of(plan, terms);
    if w.iter()
        .any(|r| !r.negated && r.name == terms.kernel.r#false)
    {
        Some("(false)")
    } else if w.iter().any(|r| r.negated) {
        Some("(not …)")
    } else {
        None
    }
}

/// `rule[a b]`.
pub fn named(terms: &Terms, rule: Symbol, act: &[Value]) -> String {
    let a: Vec<String> = act
        .iter()
        .map(|&v| match v.as_sym() {
            Some(s) => terms.sym(s).to_string(),
            None => terms.display(v),
        })
        .collect();
    format!("{}[{}]", terms.sym(rule), a.join(" "))
}

/// Where `(rule NAME` is written, `file:line`: the entry itself, else every
/// stdlib module that defines the name (a loaded rule carries no `Loc` the
/// loader fills, so this reads the text).
pub fn located(entry: &Path, name: &str) -> String {
    let hits = locations(entry, name);
    if hits.is_empty() {
        "?".to_string()
    } else {
        hits.join(" | ")
    }
}

/// [`located`], narrowed to the stdlib modules `entry` imports, directly or
/// through a module it imports, where that leaves one: `typecheck-arg-0` is
/// written in both `std.elim` and `std.bijection`, and a program importing
/// one of them gets that one.
pub fn located_imported(entry: &Path, name: &str) -> String {
    let hits = locations(entry, name);
    if hits.len() > 1 {
        let imported = imported_modules(entry);
        let kept: Vec<&String> = hits
            .iter()
            .filter(|h| {
                h.strip_prefix("stdlib/")
                    .and_then(|f| f.split(".ein:").next())
                    .is_some_and(|stem| imported.contains(&format!("std.{stem}")))
            })
            .collect();
        if kept.len() == 1 {
            return kept[0].clone();
        }
    }
    if hits.is_empty() {
        "?".to_string()
    } else {
        hits.join(" | ")
    }
}

/// `program`, or the stdlib module a [`located`] string names (`std.elim`).
pub fn module_of(location: &str) -> String {
    match location.strip_prefix("stdlib/") {
        Some(rest) => match rest.split_once(".ein:") {
            Some((stem, _)) if !location.contains(" | ") => format!("std.{stem}"),
            _ => "stdlib (ambiguous)".to_string(),
        },
        None if location == "?" => "?".to_string(),
        None => "program".to_string(),
    }
}

fn locations(entry: &Path, name: &str) -> Vec<String> {
    let head = format!("(rule {name} ");
    let find = |p: &Path| -> Option<usize> {
        let text = std::fs::read_to_string(p).ok()?;
        text.lines()
            .position(|l| l.trim_start().starts_with(&head))
            .map(|i| i + 1)
    };
    if let Some(line) = find(entry) {
        return vec![format!("{}:{line}", relative(entry))];
    }
    let mut hits: Vec<String> = ein_corpus::ein_files_under(&repo_root().join("stdlib"))
        .into_iter()
        .filter_map(|p| find(&p).map(|line| format!("{}:{line}", relative(&p))))
        .collect();
    hits.sort();
    hits
}

/// The `std.X` modules `entry` imports, closed over the stdlib's own
/// imports — read from the text, as [`located`] reads it.
fn imported_modules(entry: &Path) -> std::collections::BTreeSet<String> {
    let imports = |p: &Path| -> Vec<String> {
        std::fs::read_to_string(p)
            .unwrap_or_default()
            .lines()
            .filter_map(|l| l.trim_start().strip_prefix("(import "))
            .filter_map(|rest| rest.split_whitespace().next())
            .map(|m| m.trim_end_matches(')').to_string())
            .filter(|m| m.starts_with("std."))
            .collect()
    };
    let mut seen = std::collections::BTreeSet::new();
    let mut stack = imports(entry);
    while let Some(m) = stack.pop() {
        if !seen.insert(m.clone()) {
            continue;
        }
        let file = repo_root().join("stdlib").join(format!(
            "{}.ein",
            m.trim_start_matches("std.").replace('.', "/")
        ));
        stack.extend(imports(&file));
    }
    seen
}
