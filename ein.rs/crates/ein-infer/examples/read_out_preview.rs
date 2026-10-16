//! The read-out S1e2.1a.5 proposes, printed from the library's own R — M1e2
//! [T1e2.1a.5.1](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md)'s
//! page diff, [`read_out.md`](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md).
//!
//! **A preview, not the read-out.** It exists so that every example on the
//! page diff is what the engine computes, not what someone typed: R from
//! route 1 ([`ein_infer::downset`]) or the cube ([`ein_infer::region`]), a
//! mode's answer from [`ein_infer::explore`], each state's facts from its own
//! fixpoint, and each query's projection classes from
//! [`ein_infer::goal_bindings`] on those fixpoints. The layout and the words
//! are the proposal's. When the user has ruled, T1e2.1a.5.2 builds the
//! surfaces in `ein-render` and `ein-cli`, and this file goes.
//!
//! ```sh
//! cargo run --release -p ein-infer --example read_out_preview -- \
//!     [--minimal | --maximal] [-n N | --single] [--region-budget F] \
//!     [--region-limit N] [--over all|minimal|maximal] [--jobs J] FILE
//! ```
//!
//! With no mode flag it prints the proposed default, `ein solve FILE`: R
//! described. The last line is the exit code the proposal gives the run.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use ein_core::{Kb, Terms, render_why};
use ein_infer::compile::SharedMemo;
use ein_infer::downset::{self, Declined, DownSet};
use ein_infer::events::sexpr;
use ein_infer::explore::{self, Answer, Ask, Mode, Stopped, Verdict, Which};
use ein_infer::monotone::Monotonicity;
use ein_infer::region::{self, Outcome, Region};
use ein_infer::{Events, goal_bindings, query_value, solve_value, try_commitment_set};
use ein_ir::{Ast, Node, NodeId, load, parse};

const RULE: &str = "──────────────────────────────────────────────────────────────";

struct Args {
    which: Which,
    ask: Ask,
    budget: u64,
    limit: u32,
    jobs: usize,
    over: Which,
    file: String,
    argv: String,
}

fn args() -> Args {
    let mut a = Args {
        which: Which::All,
        ask: Ask::Describe,
        budget: downset::DEFAULT_BUDGET,
        limit: region::DEFAULT_LIMIT,
        jobs: 1,
        over: Which::All,
        file: String::new(),
        argv: String::new(),
    };
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut shown: Vec<String> = Vec::new();
    let mut it = raw.into_iter();
    while let Some(x) = it.next() {
        match x.as_str() {
            "--minimal" => {
                a.which = Which::Minimal;
                shown.push(x);
            }
            "--maximal" => {
                a.which = Which::Maximal;
                shown.push(x);
            }
            "--single" => {
                a.ask = Ask::Single;
                shown.push(x);
            }
            "-n" => {
                let n = it.next().expect("-n N");
                a.ask = Ask::Find(n.parse().expect("-n takes a count"));
                shown.push(format!("-n {n}"));
            }
            "--region-budget" => {
                let f = it.next().expect("--region-budget F");
                a.budget = f.parse().expect("a count of facts");
                shown.push(format!("--region-budget {f}"));
            }
            "--region-limit" => {
                let n = it.next().expect("--region-limit N");
                a.limit = n.parse().expect("an n");
                shown.push(format!("--region-limit {n}"));
            }
            "--jobs" => a.jobs = it.next().expect("--jobs J").parse().expect("a count"),
            "--over" => {
                a.over = match it.next().expect("--over WHICH").as_str() {
                    "minimal" => Which::Minimal,
                    "maximal" => Which::Maximal,
                    _ => Which::All,
                }
            }
            _ => a.file = x,
        }
    }
    shown.push(a.file.clone());
    a.argv = shown.join(" ");
    a
}

fn loaded(path: &Path) -> (Kb, Terms, Ast) {
    let text = std::fs::read_to_string(path).expect("the file reads");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("the file parses");
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("the file loads");
    (kb, terms, ast)
}

fn plural(n: u64, one: &str, many: &str) -> String {
    format!("{} {}", group(n), if n == 1 { one } else { many })
}

/// `261 982`: thin-space-free digit groups, the corpus pages' spelling.
fn group(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(' ');
        }
        out.push(c);
    }
    out
}

fn row(label: &str, value: impl AsRef<str>) {
    println!("  {label:<18}{}", value.as_ref());
}

fn relation_of(fact: &str) -> &str {
    fact.trim_start_matches('(')
        .split(|c: char| c.is_whitespace() || c == ')')
        .next()
        .unwrap_or("")
}

// ── A state's facts, from its own fixpoint ─────────────────────────────

/// One state of what the run lists.
struct St {
    /// A commitment that reaches it, as candidate indices.
    rep: Vec<u32>,
    /// The candidates its fixpoint holds.
    closure: Vec<u32>,
    /// Its minimal generating commitments, when known.
    generators: Vec<Vec<u32>>,
    /// How many commitments of R reach it, when known.
    class_size: Option<u64>,
    minimal: bool,
    maximal: bool,
    /// The positive facts of the candidates' relations, root's included —
    /// the `(model …)` the claim prints.
    model: Vec<String>,
    kb: Option<Kb>,
}

/// The fixpoint of `root ∪ set`, for a root a route has already saturated.
fn fixpoint(root: &Kb, terms: &mut Terms, ast: &Ast, facts: &[ein_core::FactId]) -> Kb {
    let memo = SharedMemo::default();
    let mut off = Events::off();
    try_commitment_set(root, terms, ast, &mut off, &memo, facts, None, None)
        .expect("a node saturates")
        .kb
}

/// ⊤, every candidate, when it is in R. In a monotone program ⊤ alive means
/// no node is dead, and ⊤ in R makes the maximal line {⊤}: the trivial line,
/// certified by one saturation (L1 and L2).
fn top_in_r(root: &Kb, terms: &mut Terms, ast: &Ast, facts: &[ein_core::FactId]) -> Option<Kb> {
    let memo = SharedMemo::default();
    let mut off = Events::off();
    let r = try_commitment_set(root, terms, ast, &mut off, &memo, facts, None, None)
        .expect("⊤ saturates");
    (matches!(r.kind, ein_infer::Kind::Alive) && r.owes.total() == 0).then_some(r.kb)
}

fn model_of(kb: &Kb, terms: &Terms, named: &BTreeSet<String>) -> Vec<String> {
    let mut v: Vec<String> = kb
        .facts()
        .map(|f| sexpr(terms, f))
        .filter(|t| !t.starts_with("(not ") && named.contains(relation_of(t)))
        .collect();
    v.sort();
    v.dedup();
    v
}

fn model_text(facts: &[String]) -> String {
    if facts.is_empty() {
        "(model)".to_string()
    } else {
        format!("(model {})", facts.join(" "))
    }
}

/// `(model a b c)` broken at `width` with a hanging indent.
fn wrapped(head: &str, items: &[String], indent: usize, width: usize) -> Vec<String> {
    if items.is_empty() {
        return vec![format!("{}({head})", " ".repeat(indent))];
    }
    let mut lines = Vec::new();
    let mut cur = format!("{}({head}", " ".repeat(indent));
    let hang = indent + head.len() + 2;
    let mut first = true;
    for it in items {
        if !first && cur.chars().count() + 1 + it.chars().count() > width {
            lines.push(cur);
            cur = format!("{}{it}", " ".repeat(hang));
        } else {
            cur.push(' ');
            cur.push_str(it);
        }
        first = false;
    }
    cur.push(')');
    lines.push(cur);
    lines
}

fn set_name(cands: &[String], set: &[u32]) -> String {
    if set.is_empty() {
        return "∅".to_string();
    }
    let parts: Vec<&str> = set.iter().map(|&b| cands[b as usize].as_str()).collect();
    format!("{{{}}}", parts.join(", "))
}

fn set_facts(cands: &[String], set: &[u32]) -> Vec<String> {
    let mut v: Vec<String> = set.iter().map(|&b| cands[b as usize].clone()).collect();
    v.sort();
    v
}

// ── Queries: projection classes ────────────────────────────────────────

fn conjuncts(ast: &Ast, goal: NodeId) -> Vec<NodeId> {
    if let Node::SForm { head, args } = ast.node(goal)
        && matches!(ast.node(head), Node::Atom(s) if ast.sym(s) == "and")
    {
        return ast.args(args).to_vec();
    }
    vec![goal]
}

fn ground(ast: &Ast, conj: NodeId, b: &[(String, String)]) -> Option<String> {
    let Node::SForm { head, args } = ast.node(conj) else {
        return None;
    };
    let Node::Atom(rel) = ast.node(head) else {
        return None;
    };
    let out: Vec<String> = ast
        .args(args)
        .iter()
        .map(|a| match ast.node(*a) {
            Node::Var(s) => {
                let name = ast.sym(s);
                b.iter()
                    .find(|(k, _)| k == name)
                    .map(|(_, v)| v.clone())
                    .unwrap_or_else(|| format!("?{name}"))
            }
            Node::Atom(s) | Node::Int(s) => ast.sym(s).to_string(),
            _ => ein_ir::dump::dump_compact(ast, *a),
        })
        .collect();
    Some(if out.is_empty() {
        format!("({})", ast.sym(rel))
    } else {
        format!("({} {})", ast.sym(rel), out.join(" "))
    })
}

/// One projection class: the goal's rows in a state, and the states that
/// give them.
struct Class {
    facts: Vec<String>,
    rows: Vec<Vec<(String, String)>>,
    states: Vec<usize>,
}

fn classes(ast: &Ast, terms: &mut Terms, goal: NodeId, states: &[St]) -> Vec<Class> {
    let conj = conjuncts(ast, goal);
    let mut by_key: BTreeMap<Vec<String>, Class> = BTreeMap::new();
    for (i, s) in states.iter().enumerate() {
        let Some(kb) = s.kb.as_ref() else { continue };
        let mut rows: Vec<Vec<(String, String)>> = goal_bindings(ast, terms, kb, Some(goal))
            .into_iter()
            .map(|r| {
                let mut r: Vec<(String, String)> = r
                    .iter()
                    .map(|(k, v)| (terms.sym(*k).to_string(), terms.display(*v)))
                    .collect();
                r.sort();
                r
            })
            .collect();
        rows.sort();
        rows.dedup();
        let mut facts: Vec<String> = rows
            .iter()
            .flat_map(|b| conj.iter().filter_map(|c| ground(ast, *c, b)))
            .collect();
        facts.sort();
        facts.dedup();
        by_key
            .entry(facts.clone())
            .or_insert_with(|| Class {
                facts,
                rows,
                states: Vec::new(),
            })
            .states
            .push(i);
    }
    let mut v: Vec<Class> = by_key.into_values().collect();
    v.sort_by(|a, b| (a.facts.len(), &a.facts).cmp(&(b.facts.len(), &b.facts)));
    v
}

fn which_words(w: Which) -> &'static str {
    match w {
        Which::All => "all of R",
        Which::Minimal => "the minimal line",
        Which::Maximal => "the maximal line",
    }
}

/// Every query's projection classes over `states`, said to be `over`.
fn print_queries(
    kb: &Kb,
    ast: &Ast,
    terms: &mut Terms,
    states: &[St],
    over: &str,
    exact: bool,
    asked: Which,
) {
    let queries = kb.program().queries.clone();
    let n_q = queries.len();
    for (qi, q) in queries.iter().enumerate() {
        let Some(goal) = query_value(ast, q, "goal") else {
            continue;
        };
        println!();
        let label = if n_q > 1 {
            format!("query {} of {n_q}", qi + 1)
        } else {
            "query".to_string()
        };
        row(&label, ein_ir::dump::dump_compact(ast, goal));
        if asked != Which::All && over != which_words(asked) {
            row(
                "",
                format!(
                    "asks {} (:over); this run listed {over}, so it is answered over that",
                    which_words(asked)
                ),
            );
        }
        if states.is_empty() && exact {
            row(
                "over",
                format!("{over} — R is empty: no state to answer from"),
            );
            row("as :expect", "(false)");
            continue;
        }
        let cl = classes(ast, terms, goal, states);
        let n_states = cl.iter().map(|c| c.states.len()).sum::<usize>() as u64;
        let tail = if exact { "" } else { "  (a lower bound)" };
        row(
            "over",
            format!(
                "{over} — {} → {}{tail}",
                plural(n_states, "state", "states"),
                plural(cl.len() as u64, "projection class", "projection classes")
            ),
        );
        let text = query_value(ast, q, "goal-text").and_then(|n| match ast.node(n) {
            Node::Str(s) => Some(ast.sym(s).to_string()),
            _ => None,
        });
        let shown = if exact {
            cl.len()
        } else {
            cl.len().min(PARTIAL_LIST)
        };
        for (ci, c) in cl.iter().take(shown).enumerate() {
            let mut which: Vec<String> = c
                .states
                .iter()
                .take(12)
                .map(|i| (i + 1).to_string())
                .collect();
            if c.states.len() > 12 {
                which.push("…".to_string());
            }
            let head = format!("class {} of {}", ci + 1, cl.len());
            // State numbers point into the listing, so only a complete
            // listing gets them.
            let where_ = if exact {
                format!(
                    "{} ({})",
                    plural(c.states.len() as u64, "state", "states"),
                    which.join(", ")
                )
            } else {
                plural(c.states.len() as u64, "state", "states")
            };
            if c.rows.is_empty() {
                row(
                    &format!("  {head}"),
                    format!("{where_} — the goal has no row: undetermined here"),
                );
                continue;
            }
            row(&format!("  {head}"), where_);
            for r in &c.rows {
                let b: Vec<String> = r.iter().map(|(k, v)| format!("?{k} = {v}")).collect();
                if b.is_empty() {
                    row("", "  the goal holds: it has no variable");
                } else {
                    row("", format!("  {}", b.join(" · ")));
                }
                if let Some(t) = &text {
                    row("", format!("    “{}”", render_why(t, r)));
                }
            }
        }
        if !exact {
            if shown < cl.len() {
                row(
                    "",
                    format!(
                        "… {} more classes, written by --dump-answer",
                        group((cl.len() - shown) as u64)
                    ),
                );
            }
            row(
                "as :expect",
                "not printed: these classes are a lower bound, and the claim would say they are all",
            );
            continue;
        }
        // The query's `:expect`, one class per line: `(model …)` for one,
        // `(or …)` for several, `(false)` for none.
        let mut lines: Vec<String> = Vec::new();
        if cl.is_empty() {
            lines.push("(false)".to_string());
        } else if cl.len() == 1 {
            lines = wrapped("model", &cl[0].facts, 0, 58);
        } else {
            for (ci, c) in cl.iter().enumerate() {
                let mut m = wrapped("model", &c.facts, 4, 58);
                if ci == 0 {
                    m[0] = format!("(or {}", m[0].trim_start());
                }
                if ci + 1 == cl.len()
                    && let Some(last) = m.last_mut()
                {
                    last.push(')');
                }
                lines.extend(m);
            }
        }
        for (li, l) in lines.iter().enumerate() {
            row(if li == 0 { "as :expect" } else { "" }, l);
        }
    }
}

// ── The rows every run prints ──────────────────────────────────────────

fn lattice_row(m: &Monotonicity, holes: Option<usize>) {
    if m.is_monotone() {
        row(
            "lattice",
            "monotone (the static check) — R is convex: no hole",
        );
    } else {
        let h = match holes {
            Some(0) => "no hole in this cube".to_string(),
            Some(h) => plural(h as u64, "hole", "holes"),
            None => "holes not known: the cube was not enumerated".to_string(),
        };
        row(
            "lattice",
            format!(
                "not monotone: {} (the static check) — {h}; assumes a monotone lattice \
                 for what a node is (P1e2.1b)",
                plural(m.sites.len() as u64, "site", "sites")
            ),
        );
    }
}

fn header(a: &Args, asked: &str) {
    println!("solve · {}", a.argv);
    println!("{RULE}");
    row("asked", asked);
}

// ── Describe: route 1 ──────────────────────────────────────────────────

fn describe(a: &Args) -> i32 {
    let path = Path::new(&a.file);
    let (mut kb, mut terms, ast) = loaded(path);
    let d = downset::compute(
        &mut kb,
        &mut terms,
        &ast,
        &downset::Options {
            budget: a.budget,
            max_visits: u64::MAX,
            jobs: a.jobs,
        },
    )
    .expect("route 1 runs");
    if matches!(d.declined, Some(Declined::NotMonotone)) {
        return describe_cube(a);
    }
    describe_route_1(a, kb, terms, ast, d)
}

fn describe_route_1(a: &Args, kb: Kb, mut terms: Terms, ast: Ast, d: DownSet) -> i32 {
    let cands: Vec<String> = d.candidates.iter().map(|c| c.text.clone()).collect();
    let named: BTreeSet<String> = cands.iter().map(|c| relation_of(c).to_string()).collect();
    let n = cands.len();
    let points = &d.points;
    let exhausted = d.description.is_some();
    let completed = d.layers.len();

    // States: one per class, in R, ordered by size then by candidates.
    let mut class_size: BTreeMap<u32, u64> = BTreeMap::new();
    for p in points.iter().filter(|p| p.in_r()) {
        *class_size.entry(p.class).or_default() += 1;
    }
    let desc = d.description.as_ref();
    let min_classes: BTreeSet<u32> = desc
        .map(|x| {
            x.minimal
                .iter()
                .map(|&i| points[i as usize].class)
                .collect()
        })
        .unwrap_or_else(|| {
            // Partial: a member of R whose every proper subset was visited
            // and is outside R is certified minimal. Route 1 visits every
            // alive subset of a visited point, so that is every member of R
            // whose one-down subsets are all outside R.
            minimal_found(&d)
        });
    let max_classes: BTreeSet<u32> = desc
        .map(|x| {
            x.maximal
                .iter()
                .map(|&i| points[i as usize].class)
                .collect()
        })
        .unwrap_or_else(|| maximal_found(&d));
    let mut gens: BTreeMap<u32, Vec<Vec<u32>>> = BTreeMap::new();
    if let Some(x) = desc {
        for m in &x.merged {
            gens.insert(
                m.class,
                m.generators
                    .iter()
                    .map(|&g| points[g as usize].set.to_vec())
                    .collect(),
            );
        }
    }
    let mut states: Vec<St> = Vec::new();
    for (i, p) in points.iter().enumerate() {
        if !p.in_r() || p.class as usize != i {
            continue;
        }
        let size = class_size[&p.class];
        states.push(St {
            rep: p.set.to_vec(),
            closure: p.closure.to_vec(),
            generators: gens
                .get(&p.class)
                .cloned()
                .unwrap_or_else(|| vec![p.set.to_vec()]),
            class_size: Some(size),
            minimal: min_classes.contains(&p.class),
            maximal: max_classes.contains(&p.class),
            model: Vec::new(),
            kb: None,
        });
    }
    states.sort_by(|x, y| (x.closure.len(), &x.closure).cmp(&(y.closure.len(), &y.closure)));
    for s in states.iter_mut() {
        let facts: Vec<_> = s
            .rep
            .iter()
            .map(|&b| d.candidates[b as usize].fact)
            .collect();
        let st = fixpoint(&kb, &mut terms, &ast, &facts);
        s.model = model_of(&st, &terms, &named);
        s.kb = Some(st);
    }

    // A partial run still certifies the trivial maximal line, with one
    // saturation of ⊤.
    let mut top_certified = false;
    let mut top_extra = 0u64;
    if !exhausted && n > 0 {
        let all: Vec<_> = d.candidates.iter().map(|c| c.fact).collect();
        if let Some(top) = top_in_r(&kb, &mut terms, &ast, &all) {
            top_certified = true;
            let full: Vec<u32> = (0..n as u32).collect();
            match states.iter_mut().find(|s| s.closure == full) {
                Some(s) => s.maximal = true,
                None => {
                    top_extra = 1;
                    states.push(St {
                        rep: full.clone(),
                        closure: full,
                        generators: Vec::new(),
                        class_size: None,
                        minimal: false,
                        maximal: true,
                        model: model_of(&top, &terms, &named),
                        kb: Some(top),
                    });
                }
            }
        }
    }
    let k_s = states.len() as u64;
    let k_c: u64 = class_size.values().sum::<u64>() + top_extra;
    let root = &points[0];
    let lb = if exhausted {
        String::new()
    } else {
        format!(
            "   (a lower bound: complete up to commitments of {})",
            plural(
                completed.saturating_sub(1) as u64,
                "candidate",
                "candidates"
            )
        )
    };

    header(a, "all of R — describe");
    row("states (k_s)", format!("{}{lb}", group(k_s)));
    row("commitments (k_c)", format!("{}{lb}", group(k_c)));
    match &d.declined {
        None => row(
            "exhausted",
            "yes — every commitment is classified: this is all of R",
        ),
        Some(Declined::Budget {
            layer,
            next,
            spent,
            projected,
        }) => row(
            "exhausted",
            format!(
                "no — route 1 stopped before layer {layer}: its {} commitments project {} \
                 facts on top of the {} spent, past --region-budget {}",
                group(*next),
                group(*projected),
                group(*spent),
                group(a.budget)
            ),
        ),
        Some(other) => row("exhausted", format!("no — {}", other.as_str())),
    }
    let root_alive = root.status & region::DEAD == 0;
    if k_s == 0 && exhausted {
        if !root_alive {
            row(
                "empty",
                "every node is dead — root's own fixpoint derives (false)",
            );
        } else {
            row(
                "empty",
                "every alive node owes — no commitment pays every debt",
            );
        }
    } else if root.in_r() {
        row("∅", "in R — root is alive and owes nothing");
    } else if !root_alive {
        row("∅", "dead — root's own fixpoint derives (false)");
    } else {
        row(
            "∅",
            format!(
                "not in R — root owes {}",
                plural(root.owes as u64, "instance", "instances")
            ),
        );
    }

    if k_s > 0 || !exhausted {
        let obligations_stated = d.points.iter().any(|p| p.owes > 0);
        let restrictions_stated = d.points.iter().any(|p| p.status & region::DEAD != 0);
        let n_min = states.iter().filter(|s| s.minimal).count() as u64;
        let n_max = states.iter().filter(|s| s.maximal).count() as u64;
        let smallest = desc
            .map(|x| {
                x.minimal
                    .iter()
                    .map(|&i| points[i as usize].set.len())
                    .min()
                    .unwrap_or(0)
            })
            .unwrap_or(0);
        let min_note = if root.in_r() {
            if obligations_stated {
                " — ∅".to_string()
            } else {
                " — ∅, the trivial line: no obligation is stated".to_string()
            }
        } else if exhausted {
            format!(
                ", from {} — the least guess is {}",
                plural(
                    desc.map_or(0, |x| x.minimal.len()) as u64,
                    "commitment",
                    "commitments"
                ),
                plural(smallest as u64, "candidate", "candidates")
            )
        } else {
            String::new()
        };
        if exhausted || root.in_r() {
            row(
                "minimal line",
                format!("{}{min_note}", plural(n_min, "state", "states")),
            );
        } else {
            row(
                "minimal line",
                format!(
                    "≥ {} — complete up to commitments of {}",
                    plural(n_min, "state", "states"),
                    plural(
                        completed.saturating_sub(1) as u64,
                        "candidate",
                        "candidates"
                    )
                ),
            );
        }
        let max_note = if exhausted
            && !restrictions_stated
            && states.iter().any(|s| s.maximal && s.closure.len() == n)
        {
            " — ⊤, the trivial line: no restriction bites"
        } else {
            ""
        };
        if exhausted {
            row(
                "maximal line",
                format!("{}{max_note}", plural(n_max, "state", "states")),
            );
        } else if top_certified {
            row(
                "maximal line",
                "1 state — ⊤, every candidate: no node is dead, the trivial line, \
                 certified by one saturation",
            );
        } else if n_max > 0 {
            row(
                "maximal line",
                format!(
                    "≥ {} — all those a commitment of up to {} reaches",
                    plural(n_max, "state", "states"),
                    plural(
                        completed.saturating_sub(2) as u64,
                        "candidate",
                        "candidates"
                    )
                ),
            );
        } else {
            row("maximal line", "not reached");
        }
        if let Some(x) = desc {
            let comps = &x.components;
            let sizes: Vec<u64> = comps.iter().map(|c| c.size).collect();
            let note = if comps.len() > 1 && sizes.iter().all(|&s| s == 1) {
                ", each a single commitment".to_string()
            } else if comps.len() > 1 {
                format!(
                    ", with {} commitments",
                    sizes
                        .iter()
                        .map(|s| group(*s))
                        .collect::<Vec<_>>()
                        .join(" + ")
                )
            } else {
                String::new()
            };
            row("components", format!("{}{note}", group(comps.len() as u64)));
            let mut dims: BTreeMap<usize, u64> = BTreeMap::new();
            for &(lo, hi) in &x.subcubes {
                let dim = points[hi as usize].set.len() - points[lo as usize].set.len();
                *dims.entry(dim).or_default() += 1;
            }
            let total: u64 = dims.values().sum();
            let dims: Vec<String> = if dims.len() == 1 {
                let d = dims.keys().next().copied().unwrap_or(0);
                vec![if total == 1 {
                    format!("of dimension {d}")
                } else {
                    format!("each of dimension {d}")
                }]
            } else {
                dims.iter()
                    .rev()
                    .map(|(d, c)| format!("{} of dimension {d}", group(*c)))
                    .collect()
            };
            row(
                "sub-cubes",
                format!("{}, {}", group(x.subcubes.len() as u64), dims.join(", ")),
            );
            let merged = &x.merged;
            if merged.is_empty() {
                row(
                    "merged",
                    "none — every state is reached by one commitment, so k_c = k_s",
                );
            } else {
                let g: u64 = merged.iter().map(|m| m.generators.len() as u64).sum();
                row(
                    "merged",
                    format!(
                        "{} reached by more than one commitment, with {} minimal",
                        plural(merged.len() as u64, "state", "states"),
                        group(g)
                    ),
                );
            }
            row(
                "N",
                plural(
                    x.nogoods.len() as u64,
                    "minimal dead set",
                    "minimal dead sets",
                ),
            );
        } else {
            row("components", "not known: R was not exhausted");
            row("sub-cubes", "not known");
            let dead = points
                .iter()
                .filter(|p| p.status & region::DEAD != 0)
                .count() as u64;
            row(
                "N",
                format!(
                    "≥ {} — complete up to size {}",
                    plural(dead, "minimal dead set", "minimal dead sets"),
                    completed.saturating_sub(1)
                ),
            );
        }
    }
    lattice_row(&d.monotonicity, None);
    let dead_alone = d.layers.get(1).map_or(0, |l| l.dead);
    if n == 0 {
        row(
            "universe",
            "no candidate: R is {∅} or empty, decided by root alone",
        );
    } else {
        row(
            "universe",
            format!(
                "{}, {} dead alone",
                plural(n as u64, "candidate", "candidates"),
                group(dead_alone)
            ),
        );
    }
    row(
        "route",
        "1, the alive down-set — licence L1, dead is upward-closed",
    );
    let facts: u64 = d.layers.iter().map(|l| l.facts).sum();
    row(
        "work",
        format!(
            "{} · {} beyond root · {}",
            plural(points.len() as u64, "saturation", "saturations"),
            plural(facts, "fact", "facts"),
            plural(completed as u64, "layer", "layers")
        ),
    );

    // The region itself.
    println!();
    if k_s == 0 && exhausted {
        if !root_alive {
            for c in &root.core {
                row("unsat core", c);
            }
            if !root.refuted_by.is_empty() {
                row("refuted by", root.refuted_by.join(", "));
            }
            println!();
            println!("  (false)");
        } else {
            let frontier = frontier_of(&d);
            for &i in &frontier {
                let p = &points[i as usize];
                row(
                    "owing",
                    format!(
                        "{} — owes {}",
                        set_name(&cands, &p.set),
                        group(p.owes as u64)
                    ),
                );
                for o in &p.owed {
                    row("", format!("  {o}"));
                }
            }
            println!();
            let mut items: Vec<String> = Vec::new();
            for &i in &frontier {
                let p = &points[i as usize];
                let facts: Vec<_> = p
                    .set
                    .iter()
                    .map(|&b| d.candidates[b as usize].fact)
                    .collect();
                let st = fixpoint(&kb, &mut terms, &ast, &facts);
                items.push(model_text(&model_of(&st, &terms, &named)));
            }
            items.sort();
            let mut lines = wrapped("region (owing", &items, 2, 78);
            if let Some(last) = lines.last_mut() {
                last.push(')');
            }
            for l in lines {
                println!("{l}");
            }
        }
    } else if exhausted {
        print_region(&states, &cands);
    } else {
        let lines: Vec<&St> = states.iter().filter(|s| s.minimal || s.maximal).collect();
        print_partial(&lines, k_s, &cands);
    }

    let over = match a.over {
        Which::All => "all of R",
        w => which_words(w),
    };
    let listed: Vec<St> = match a.over {
        Which::All => states,
        Which::Minimal => states.into_iter().filter(|s| s.minimal).collect(),
        Which::Maximal => states.into_iter().filter(|s| s.maximal).collect(),
    };
    print_queries(&kb, &ast, &mut terms, &listed, over, exhausted, Which::All);
    let code = claim_line(a);
    println!();
    println!("exit {code}");
    code
}

/// Members of R the partial run certifies minimal: every one-down subset
/// visited and outside R.
fn minimal_found(d: &DownSet) -> BTreeSet<u32> {
    let mut status: BTreeMap<&[u32], u8> = BTreeMap::new();
    for p in &d.points {
        status.insert(&p.set, p.status);
    }
    d.points
        .iter()
        .enumerate()
        .filter(|(_, p)| p.in_r())
        .filter(|(_, p)| {
            (0..p.set.len()).all(|k| {
                let mut s = p.set.to_vec();
                s.remove(k);
                status.get(&s[..]).is_some_and(|&st| st != 0)
            })
        })
        .map(|(_, p)| p.class)
        .collect()
}

/// States the partial run certifies maximal: a member `C` below the last
/// complete layer whose every extension by a candidate outside its closure is
/// dead (visited dead, or above a dead point and therefore never generated).
/// `C ∪ {x}` has the state of `cl(C) ∪ {x}`, so that is L4 for the state.
fn maximal_found(d: &DownSet) -> BTreeSet<u32> {
    let last = d.layers.len();
    if last < 2 {
        return BTreeSet::new();
    }
    let n = d.candidates.len() as u32;
    let mut status: BTreeMap<Vec<u32>, u8> = BTreeMap::new();
    for p in &d.points {
        status.insert(p.set.to_vec(), p.status);
    }
    d.points
        .iter()
        .filter(|p| p.in_r() && p.set.len() + 1 < last)
        .filter(|p| {
            (0..n).filter(|b| !p.closure.contains(b)).all(|b| {
                let mut s = p.set.to_vec();
                let at = s.partition_point(|&x| x < b);
                s.insert(at, b);
                status.get(&s).is_none_or(|&st| st & region::DEAD != 0)
            })
        })
        .map(|p| p.class)
        .collect()
}

/// Alive, owing, and no alive node one fact up.
fn frontier_of(d: &DownSet) -> Vec<u32> {
    let alive: BTreeSet<&[u32]> = d
        .points
        .iter()
        .filter(|p| p.status & region::DEAD == 0)
        .map(|p| &p.set[..])
        .collect();
    let n = d.candidates.len() as u32;
    d.points
        .iter()
        .enumerate()
        .filter(|(_, p)| p.status & region::DEAD == 0 && p.status & region::OWING != 0)
        .filter(|(_, p)| {
            !(0..n).any(|b| {
                p.set.binary_search(&b).is_err() && {
                    let mut s = p.set.to_vec();
                    let at = s.partition_point(|&x| x < b);
                    s.insert(at, b);
                    alive.contains(&s[..])
                }
            })
        })
        .map(|(i, _)| i as u32)
        .collect()
}

fn on_lines(s: &St) -> &'static str {
    match (s.minimal, s.maximal) {
        (true, true) => " · minimal and maximal",
        (true, false) => " · minimal",
        (false, true) => " · maximal",
        _ => "",
    }
}

fn print_region(states: &[St], cands: &[String]) {
    let k = states.len();
    let mut out: Vec<String> = vec!["  (region".to_string()];
    for (i, s) in states.iter().enumerate() {
        let what = if s.closure.is_empty() {
            "∅, root itself".to_string()
        } else {
            plural(s.closure.len() as u64, "candidate", "candidates")
        };
        let from = match s.class_size {
            Some(1) | None if s.closure.is_empty() => String::new(),
            Some(1) | None => format!(" · from {}", set_name(cands, &s.rep)),
            Some(c) => format!(
                " · {} commitments, {} minimal",
                group(c),
                group(s.generators.len() as u64)
            ),
        };
        out.push(format!(
            "    ;; state {} of {k} — {what}{}{from}",
            i + 1,
            on_lines(s)
        ));
        for l in wrapped("model", &s.model, 4, 78) {
            out.push(l);
        }
    }
    for (i, s) in states.iter().enumerate() {
        if s.class_size.unwrap_or(1) < 2 {
            continue;
        }
        out.push(format!(
            "    ;; state {} is reached by {} commitments; its {} minimal",
            i + 1,
            group(s.class_size.unwrap_or(0)),
            group(s.generators.len() as u64)
        ));
        let to = wrapped("model", &s.model, 12, 78);
        out.push(format!("    (merged {}", to[0].trim_start()));
        for l in &to[1..] {
            out.push(l.clone());
        }
        let mut g: Vec<Vec<String>> = s.generators.iter().map(|g| set_facts(cands, g)).collect();
        g.sort();
        if g.len() == 1 {
            let lines = wrapped("model", &g[0], 12, 78);
            let last = lines.len() - 1;
            for (j, l) in lines.iter().enumerate() {
                out.push(format!("{l}{}", if j == last { ")" } else { "" }));
            }
        } else {
            for (j, m) in g.iter().enumerate() {
                let lines = wrapped("model", m, 16, 78);
                let n_lines = lines.len();
                for (li, l) in lines.into_iter().enumerate() {
                    let l = if j == 0 && li == 0 {
                        format!("{}(or {}", " ".repeat(12), l.trim_start())
                    } else {
                        l
                    };
                    let close = if j + 1 == g.len() && li + 1 == n_lines {
                        "))"
                    } else {
                        ""
                    };
                    out.push(format!("{l}{close}"));
                }
            }
        }
    }
    if let Some(last) = out.last_mut() {
        last.push(')');
    }
    for l in out {
        println!("{l}");
    }
}

/// A partial run lists this many certified line members, or query classes,
/// and says how many more there are. An exhausted run lists everything.
const PARTIAL_LIST: usize = 64;

/// A partial description's listing: the line members it certified, at most
/// [`PARTIAL_LIST`] of them, and where the rest are.
fn print_partial(lines: &[&St], found: u64, cands: &[String]) {
    println!(
        "  ;; the line members this run certified. Not a (region …): read back, it \
         would claim these are all of R."
    );
    let shown = lines.len().min(PARTIAL_LIST);
    let more_lines = (lines.len() - shown) as u64;
    let other = found - lines.len() as u64;
    let mut rest = Vec::new();
    if more_lines > 0 {
        rest.push(format!("{} more certified line members", group(more_lines)));
    }
    if other > 0 {
        rest.push(format!("the {} other members met", group(other)));
    }
    if !rest.is_empty() {
        println!(
            "  ;; {} {} written by --dump-answer, not listed here.",
            rest.join(" and "),
            if rest.len() == 1 && more_lines + other == 1 {
                "is"
            } else {
                "are"
            }
        );
    }
    print_members(&lines[..shown], cands, None);
}

/// States listed as members, not as a region: a line, `-n`'s cut, a partial
/// run.
fn print_members(states: &[&St], cands: &[String], line: Option<&str>) {
    let k = states.len();
    for (i, s) in states.iter().enumerate() {
        let what = if s.closure.is_empty() {
            "∅, root itself".to_string()
        } else {
            plural(s.closure.len() as u64, "candidate", "candidates")
        };
        let from = if s.closure.is_empty() || s.generators.is_empty() {
            String::new()
        } else if s.generators.len() > 1 || s.generators.first() != Some(&s.rep) {
            let g: Vec<String> = s.generators.iter().map(|g| set_name(cands, g)).collect();
            format!(" · minimal {}", g.join(" | "))
        } else {
            format!(" · from {}", set_name(cands, &s.rep))
        };
        let on = match line {
            Some(l) => format!(" · {l}"),
            None => on_lines(s).to_string(),
        };
        println!("  ;; state {} of {k} — {what}{on}{from}", i + 1);
        for l in wrapped("model", &s.model, 2, 78) {
            println!("{l}");
        }
    }
}

// ── Describe: the cube, for a program the static check flags ───────────

fn describe_cube(a: &Args) -> i32 {
    let path = Path::new(&a.file);
    let (mut kb, mut terms, ast) = loaded(path);
    let out = region::compute(
        &mut kb,
        &mut terms,
        &ast,
        &region::Options {
            limit: a.limit,
            keep_states: false,
            jobs: a.jobs,
        },
        &mut |_| {},
    )
    .expect("the cube runs");
    let r: Box<Region> = match out {
        Outcome::Region(r) => r,
        // Past the limit, describe is explore's: members in layer order up
        // to 2^limit nodes, printed as members and not as line members.
        Outcome::Refused { .. } => return mode(a),
    };
    let n = r.n();
    let cands: Vec<String> = r.candidates.iter().map(|c| c.text.clone()).collect();
    let named: BTreeSet<String> = cands.iter().map(|c| relation_of(c).to_string()).collect();
    let t = &r.topology;
    let set_of = |m: u64| -> Vec<u32> { (0..n as u32).filter(|&b| m >> b & 1 == 1).collect() };
    let mut by_class: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    for m in 0..r.status.len() as u64 {
        if r.in_r(m) {
            by_class
                .entry(r.nodes[m as usize].class)
                .or_default()
                .push(m);
        }
    }
    let min: BTreeSet<u64> = t
        .minimal
        .iter()
        .map(|&m| r.nodes[m as usize].class)
        .collect();
    let max: BTreeSet<u64> = t
        .maximal
        .iter()
        .map(|&m| r.nodes[m as usize].class)
        .collect();
    let mut states: Vec<St> = by_class
        .iter()
        .map(|(&class, members)| {
            let gens: Vec<Vec<u32>> = members
                .iter()
                .filter(|&&m| !members.iter().any(|&o| o != m && o & m == o))
                .map(|&m| set_of(m))
                .collect();
            St {
                rep: set_of(class),
                closure: set_of(r.nodes[class as usize].closure),
                generators: gens,
                class_size: Some(members.len() as u64),
                minimal: min.contains(&class),
                maximal: max.contains(&class),
                model: Vec::new(),
                kb: None,
            }
        })
        .collect();
    states.sort_by(|x, y| (x.closure.len(), &x.closure).cmp(&(y.closure.len(), &y.closure)));
    for s in states.iter_mut() {
        let facts: Vec<_> = s
            .rep
            .iter()
            .map(|&b| r.candidates[b as usize].fact)
            .collect();
        let st = fixpoint(&kb, &mut terms, &ast, &facts);
        s.model = model_of(&st, &terms, &named);
        s.kb = Some(st);
    }
    let k_s = states.len() as u64;
    header(a, "all of R — describe");
    row("states (k_s)", group(k_s));
    row("commitments (k_c)", group(t.size_r));
    row(
        "exhausted",
        "yes — every commitment of the cube is classified: this is all of R",
    );
    if r.in_r(0) {
        row("∅", "in R — root is alive and owes nothing");
    } else if r.root_dead {
        row("∅", "dead — root's own fixpoint derives (false)");
    } else {
        row(
            "∅",
            format!(
                "not in R — root owes {}",
                plural(r.nodes[0].owes as u64, "instance", "instances")
            ),
        );
    }
    row(
        "minimal line",
        plural(
            states.iter().filter(|s| s.minimal).count() as u64,
            "state",
            "states",
        ),
    );
    row(
        "maximal line",
        plural(
            states.iter().filter(|s| s.maximal).count() as u64,
            "state",
            "states",
        ),
    );
    row("components", group(t.components.len() as u64));
    match &t.subcubes {
        Some(s) => row("sub-cubes", group(s.len() as u64)),
        None => row("sub-cubes", "not known: past the interval search's bound"),
    }
    row(
        "N",
        plural(
            t.nogoods.len() as u64,
            "minimal dead set",
            "minimal dead sets",
        ),
    );
    row(
        "E",
        plural(
            t.discharging.len() as u64,
            "minimal discharging set",
            "minimal discharging sets",
        ),
    );
    lattice_row(&r.monotonicity, Some(t.holes.len()));
    if !t.holes.is_empty() {
        for &h in t.minimal_holes.iter().take(4) {
            let why = &r.nodes[h as usize];
            let what = if r.status[h as usize] & region::DEAD != 0 {
                format!("dead: {}", why.core.join(" "))
            } else {
                format!("owes {}", why.owes)
            };
            row(
                "  minimal hole",
                format!("{} — {what}", set_name(&cands, &set_of(h))),
            );
        }
    }
    let dead_alone = (0..n)
        .filter(|&b| r.status[1usize << b] & region::DEAD != 0)
        .count() as u64;
    row(
        "universe",
        format!(
            "{}, {} dead alone",
            plural(n as u64, "candidate", "candidates"),
            group(dead_alone)
        ),
    );
    row(
        "route",
        "2, the cube — no licence: the static check flags the program",
    );
    row(
        "work",
        format!("{} · 2^{n}", plural(1u64 << n, "saturation", "saturations")),
    );
    println!();
    if states.is_empty() {
        println!("  (region)");
    } else {
        print_region(&states, &cands);
    }
    print_queries(&kb, &ast, &mut terms, &states, "all of R", true, Which::All);
    let code = claim_line(a);
    println!();
    println!("exit {code}");
    code
}

// ── The program's region claim, checked against R ─────────────────────

fn claim_line(a: &Args) -> i32 {
    let path = Path::new(&a.file);
    let (mut kb, mut terms, ast) = loaded(path);
    let Some(solve) = kb.program().solve().cloned() else {
        return 0;
    };
    let Some(node) = solve_value(&ast, &solve, "expect") else {
        return 0;
    };
    let claim = ein_ir::claim::parse(&ast, node).expect("the loader validated it");
    let checked = ein_infer::claim::check(
        &mut kb,
        &mut terms,
        &ast,
        &claim,
        &ein_infer::claim::Options {
            budget: a.budget,
            limit: a.limit,
            jobs: a.jobs,
        },
    )
    .expect("the claim checks");
    println!();
    let label = match checked.report.outcome {
        ein_infer::Outcome::Held => "holds",
        ein_infer::Outcome::Failed => "FAILED",
        ein_infer::Outcome::NotChecked => "NOT CHECKED",
    };
    row("(solve :expect)", label);
    for l in &checked.report.lines {
        row("", l);
    }
    i32::from(!checked.report.passed())
}

// ── A mode: explore's answer ───────────────────────────────────────────

fn mode(a: &Args) -> i32 {
    let path = Path::new(&a.file);
    let (mut kb, mut terms, ast) = loaded(path);
    let mut opts = explore::Options::new(Mode {
        which: a.which,
        ask: a.ask,
    });
    opts.budget = a.budget;
    opts.limit = a.limit;
    opts.jobs = a.jobs;
    let ans: Answer = explore::run(&mut kb, &mut terms, &ast, &opts).expect("the mode runs");
    let cands: Vec<String> = ans.candidates.iter().map(|c| c.text.clone()).collect();
    let named: BTreeSet<String> = cands.iter().map(|c| relation_of(c).to_string()).collect();
    let line = if a.which == Which::All {
        "R"
    } else {
        which_words(a.which)
    };
    let asked = match a.ask {
        Ask::Describe if a.which == Which::All => "all of R — describe".to_string(),
        Ask::Describe => format!("{line} — every state on it"),
        Ask::Find(n) => format!("{} of {line} (-n {n})", plural(n, "state", "states")),
        Ask::Single => format!("is {line} one state? (--single)"),
    };
    header(a, &asked);

    let mut states: Vec<St> = ans
        .states
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let gens: Vec<Vec<u32>> = ans
                .minimal
                .iter()
                .filter(|(_, at)| *at == i)
                .map(|(set, _)| set.to_vec())
                .collect();
            St {
                rep: s.set.to_vec(),
                closure: s.closure.to_vec(),
                generators: if gens.is_empty() {
                    vec![s.set.to_vec()]
                } else {
                    gens
                },
                class_size: None,
                minimal: a.which == Which::Minimal,
                maximal: a.which == Which::Maximal,
                model: Vec::new(),
                kb: None,
            }
        })
        .collect();
    for s in states.iter_mut() {
        let facts: Vec<_> = s
            .rep
            .iter()
            .map(|&b| ans.candidates[b as usize].fact)
            .collect();
        let st = fixpoint(&kb, &mut terms, &ast, &facts);
        s.model = model_of(&st, &terms, &named);
        s.kb = Some(st);
    }
    let k = states.len() as u64;
    // A flagged program past the limit: the members met in layer order with
    // no member of R below them are certified minimal, monotone or not. The
    // preview asks the minimal cut for them; the read-out gets them from the
    // same walk.
    let partial_describe = a.ask == Ask::Describe && a.which == Which::All && !ans.exhausted;
    if partial_describe {
        let (mut kb2, mut terms2, ast2) = loaded(path);
        let mut mo = explore::Options::new(Mode {
            which: Which::Minimal,
            ask: Ask::Find(u64::MAX),
        });
        mo.budget = a.budget;
        mo.limit = a.limit;
        mo.jobs = a.jobs;
        let m = explore::run(&mut kb2, &mut terms2, &ast2, &mo).expect("the minimal cut runs");
        let min: BTreeSet<Vec<u32>> = m.states.iter().map(|s| s.closure.to_vec()).collect();
        for s in states.iter_mut() {
            s.minimal = min.contains(&s.closure);
        }
    }
    if a.ask == Ask::Describe && a.which == Which::All {
        let c = ans.counts.unwrap_or_default();
        let lb = if ans.exhausted {
            ""
        } else {
            "   (a lower bound: the run stopped before R was exhausted)"
        };
        row("states (k_s)", format!("{}{lb}", group(c.k_s.max(k))));
        row("commitments (k_c)", format!("{}{lb}", group(c.k_c)));
    }
    let label = match a.which {
        Which::All => "states of R",
        Which::Minimal => "minimal line",
        Which::Maximal => "maximal line",
    };
    let mut stderr: Option<String> = None;
    let code = match a.ask {
        Ask::Single => match ans.verdict {
            Some(Verdict::Solution) => {
                row("verdict", format!("Solution — {line} is one state"));
                0
            }
            Some(Verdict::Ambiguity) => {
                row(
                    "verdict",
                    format!("Ambiguity — {line} holds two states or more; two of them below"),
                );
                stderr = Some(format!(
                    "{}: --single FAILED — expected exactly one state of {line}, found two \
                     or more (Ambiguity)",
                    a.file
                ));
                1
            }
            Some(v @ (Verdict::Contradiction | Verdict::Open)) => {
                let why = if v == Verdict::Contradiction {
                    "R is empty: every node is dead"
                } else {
                    "R is empty: every alive node owes"
                };
                row("verdict", format!("{} — {why}", v.word()));
                stderr = Some(format!(
                    "{}: --single FAILED — expected exactly one state of {line}, found none ({})",
                    a.file,
                    v.word()
                ));
                1
            }
            None => {
                row("verdict", "not decided — the run stopped first");
                stderr = Some(format!(
                    "{}: --single NOT CHECKED — the run stopped before it decided",
                    a.file
                ));
                1
            }
        },
        _ => {
            let q = if ans.exhausted {
                if k == 0 {
                    if ans.root_dead {
                        " — none: R is empty, every node is dead".to_string()
                    } else {
                        " — none: R is empty, every alive node owes".to_string()
                    }
                } else {
                    " — all of them".to_string()
                }
            } else if ans.stopped.is_some() {
                "   (a lower bound: the run stopped first)".to_string()
            } else {
                format!(", as asked — {line} may hold more")
            };
            if !(a.ask == Ask::Describe && a.which == Which::All) {
                row(label, format!("{}{q}", plural(k, "state", "states")));
            }
            0
        }
    };
    match &ans.stopped {
        None if a.ask == Ask::Single => row(
            "decided",
            if matches!(ans.verdict, Some(Verdict::Contradiction | Verdict::Open)) {
                format!("yes — {line} is empty")
            } else if ans.exhausted {
                format!("yes — {line} holds no other state")
            } else {
                format!("yes — two states settle it; the rest of {line} was not visited")
            },
        ),
        None if ans.exhausted => row("exhausted", format!("yes — this is all of {line}")),
        None => row(
            "exhausted",
            "no — stopped as asked; each state shown is certified, nothing is said of the rest",
        ),
        Some(Stopped::Budget { spent, projected }) => row(
            "exhausted",
            format!(
                "no — stopped at --region-budget {}: {} facts spent, the next step projects {}",
                group(a.budget),
                group(*spent),
                group(*projected)
            ),
        ),
        Some(Stopped::Limit { n, limit }) => row(
            "exhausted",
            format!(
                "no — not monotone, and the question needs the whole cube: 2^{n}, past \
                 --region-limit {limit}"
            ),
        ),
        Some(Stopped::Deadline) => row("exhausted", "no — --max-time"),
    }
    let empty_in_r = states.iter().any(|s| s.closure.is_empty());
    if partial_describe && empty_in_r {
        row("∅", "in R — root is alive and owes nothing");
        row(
            "minimal line",
            "1 state — ∅: root is in R, so nothing in R is below it",
        );
        row(
            "maximal line",
            "not known — without a licence it needs the whole cube",
        );
    } else if partial_describe {
        row(
            "minimal line",
            format!(
                "≥ {} — every member met in layer order with nothing in R below it",
                plural(
                    states.iter().filter(|s| s.minimal).count() as u64,
                    "state",
                    "states"
                )
            ),
        );
        row(
            "maximal line",
            "not known — without a licence it needs the whole cube",
        );
    }
    lattice_row(&ans.monotonicity, ans.shape.as_ref().and_then(|s| s.holes));
    row(
        "universe",
        plural(ans.n() as u64, "candidate", "candidates"),
    );
    row(
        "route",
        format!(
            "{} — {}",
            match ans.route {
                explore::Route::DownSet => "1, the alive down-set",
                explore::Route::Cube => "2, the cube",
            },
            ans.licences
                .iter()
                .map(|l| l.label())
                .collect::<Vec<_>>()
                .join(" · ")
        ),
    );
    row(
        "work",
        format!(
            "{} · {} beyond root",
            plural(ans.saturations, "saturation", "saturations"),
            plural(ans.facts, "fact", "facts")
        ),
    );
    if !states.is_empty() {
        println!();
        let tag = match a.which {
            Which::All => None,
            Which::Minimal => Some("minimal"),
            Which::Maximal => Some("maximal"),
        };
        if partial_describe {
            let lines: Vec<&St> = states.iter().filter(|s| s.minimal).collect();
            print_partial(&lines, k, &cands);
        } else {
            print_members(&states.iter().collect::<Vec<_>>(), &cands, tag);
        }
    }
    print_queries(
        &kb,
        &ast,
        &mut terms,
        &states,
        &if partial_describe {
            format!("the {} met", plural(k, "member", "members"))
        } else {
            format!("the {} listed", plural(k, "state", "states"))
        },
        ans.exhausted,
        a.over,
    );
    if let Some(e) = stderr {
        println!();
        println!("stderr: {e}");
    }
    println!();
    println!("exit {code}");
    code
}

fn main() {
    let a = args();
    // The proposal's exit code is the last line printed; the preview itself
    // exits 0, so that a script can render every example in one pass.
    if a.which == Which::All && a.ask == Ask::Describe {
        describe(&a);
    } else {
        mode(&a);
    }
}
