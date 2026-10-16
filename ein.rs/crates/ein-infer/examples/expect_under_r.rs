//! Every corpus `:expect`, read against R — M1e2
//! [S1e2.1a.4](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.4_the_language.md)'s
//! grammar diff, which has to name the migration before anything moves.
//!
//! An `:expect` today is a claim about the states the walk recorded. Under
//! Q-M1e2.7 the answer is R, so each claim is read four ways against the
//! oracle's cube, every state of R in full (root's facts and the node's):
//!
//! | reading | the claim is about |
//! |---|---|
//! | `all` | every state of R, today's rules: as many listed models as states, matched one to one, each model closing the relations it names |
//! | `maximal` | the states of R's maximal line, the same rules |
//! | `minimal` | the states of R's minimal line, the same rules |
//! | `projection` | (c)'s reading: every state of R cut to the goal's relations, deduplicated, against the listed models cut the same way |
//!
//! `(false)` holds under every reading exactly when R is empty. One JSON line
//! per file.
//!
//! ```sh
//! cargo run --release -p ein-infer --example expect_under_r -- [--jobs J] [--limit N] FILE…
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use ein_core::{Kb, Terms};
use ein_infer::query_value;
use ein_infer::region::{self, Outcome};
use ein_ir::expect::{Expectation, Model};
use ein_ir::{Ast, Node, NodeId, load, parse};
use serde_json::json;

fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

/// A state as the checker sees it: every rendered fact, and the positive
/// extent per relation.
struct Seen {
    all: BTreeSet<String>,
    by_rel: BTreeMap<String, BTreeSet<String>>,
}

fn relation_of(fact: &str) -> &str {
    let inner = fact.trim_start_matches('(');
    inner
        .split(|c: char| c.is_whitespace() || c == ')')
        .next()
        .unwrap_or("")
}

fn seen(facts: &BTreeSet<String>) -> Seen {
    let mut by_rel: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for f in facts {
        let rel = relation_of(f);
        if rel != "not" {
            by_rel.entry(rel.to_string()).or_default().insert(f.clone());
        }
    }
    Seen {
        all: facts.clone(),
        by_rel,
    }
}

/// One listed model: the positive extent it closes, and the negatives it
/// wants present.
struct Want {
    closes: BTreeMap<String, BTreeSet<String>>,
    negatives: Vec<String>,
}

fn want(ast: &Ast, m: &Model) -> Want {
    let mut closes: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut negatives = Vec::new();
    for &node in &m.facts {
        let Ok(f) = ein_ir::expect::fact(ast, node) else {
            continue;
        };
        if f.negated {
            negatives.push(f.rendered);
        } else {
            closes
                .entry(f.relation.to_string())
                .or_default()
                .insert(f.rendered);
        }
    }
    Want { closes, negatives }
}

fn fits(w: &Want, s: &Seen) -> bool {
    let empty = BTreeSet::new();
    w.closes
        .iter()
        .all(|(rel, ext)| s.by_rel.get(rel).unwrap_or(&empty) == ext)
        && w.negatives.iter().all(|n| s.all.contains(n))
}

/// Today's rule: as many models as states, and a perfect matching.
fn holds(wants: &[Want], states: &[Seen]) -> bool {
    if wants.len() != states.len() {
        return false;
    }
    let mut owner: Vec<Option<usize>> = vec![None; states.len()];
    fn augment(
        i: usize,
        wants: &[Want],
        states: &[Seen],
        seen: &mut [bool],
        owner: &mut [Option<usize>],
    ) -> bool {
        for j in 0..states.len() {
            if seen[j] || !fits(&wants[i], &states[j]) {
                continue;
            }
            seen[j] = true;
            if owner[j].is_none_or(|k| augment(k, wants, states, seen, owner)) {
                owner[j] = Some(i);
                return true;
            }
        }
        false
    }
    (0..wants.len()).all(|i| {
        let mut seen = vec![false; states.len()];
        augment(i, wants, states, &mut seen, &mut owner)
    })
}

/// The goal's relations, read off the `:goal` value's atoms.
fn goal_relations(ast: &Ast, node: NodeId, out: &mut BTreeSet<String>) {
    if let Node::SForm { head, args } = ast.node(node) {
        if let Node::Atom(s) = ast.node(head) {
            let name = ast.sym(s).to_string();
            if !matches!(name.as_str(), "and" | "or" | "not" | "absent") {
                out.insert(name);
            }
        }
        for &i in ast.args(args) {
            goal_relations(ast, i, out);
        }
    }
}

fn main() {
    let mut jobs = 8usize;
    let mut limit = 16u32;
    let mut files = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--jobs" => jobs = args.next().and_then(|v| v.parse().ok()).unwrap_or(jobs),
            "--limit" => limit = args.next().and_then(|v| v.parse().ok()).unwrap_or(limit),
            _ => files.push(a),
        }
    }
    for file in files {
        let path = Path::new(&file);
        let Some((mut kb, mut terms, ast)) = loaded(path) else {
            println!("{}", json!({"path": file, "error": "load"}));
            continue;
        };
        let Some(query) = kb.program().query().cloned() else {
            continue;
        };
        let Some(enode) = query_value(&ast, &query, "expect") else {
            continue;
        };
        let Ok(expectation) = ein_ir::expect::parse(&ast, enode) else {
            continue;
        };
        let mut goal = BTreeSet::new();
        if let Some(g) = query_value(&ast, &query, "goal") {
            goal_relations(&ast, g, &mut goal);
        }
        let shape = match &expectation {
            Expectation::Contradiction => "false",
            Expectation::One(_) => "model",
            Expectation::Any(_) => "or",
        };
        let wants: Vec<Want> = expectation.models().iter().map(|m| want(&ast, m)).collect();
        let names: BTreeSet<String> = wants
            .iter()
            .flat_map(|w| w.closes.keys().cloned())
            .collect();
        let beyond_goal: Vec<&String> = names.iter().filter(|r| !goal.contains(*r)).collect();
        let negated = wants.iter().map(|w| w.negatives.len()).sum::<usize>();

        let opts = region::Options {
            limit,
            keep_states: true,
            jobs,
        };
        let outcome = region::compute(&mut kb, &mut terms, &ast, &opts, &mut |_| {});
        let Ok(Outcome::Region(r)) = outcome else {
            println!("{}", json!({"path": file, "shape": shape, "refused": true}));
            continue;
        };
        let root: BTreeSet<String> = kb
            .facts()
            .map(|f| ein_infer::events::sexpr(&terms, f))
            .collect();
        let states = r.states.as_ref().expect("keep_states");
        let mut by_class: BTreeMap<u64, BTreeSet<String>> = BTreeMap::new();
        let (mut max, mut min) = (BTreeSet::new(), BTreeSet::new());
        let any_alive = r.status.iter().any(|s| s & region::DEAD == 0);
        for mask in 0..r.status.len() as u64 {
            if !r.in_r(mask) {
                continue;
            }
            let class = r.nodes[mask as usize].class;
            let mut facts = root.clone();
            facts.extend(states[mask as usize].iter().cloned());
            by_class.insert(class, facts);
            if r.topology.maximal.contains(&mask) {
                max.insert(class);
            }
            if r.topology.minimal.contains(&mask) {
                min.insert(class);
            }
        }
        let all: Vec<Seen> = by_class.values().map(seen).collect();
        let pick =
            |set: &BTreeSet<u64>| -> Vec<Seen> { set.iter().map(|c| seen(&by_class[c])).collect() };
        let empty = by_class.is_empty();
        let reading = |states: &[Seen]| -> bool {
            match expectation {
                Expectation::Contradiction => empty,
                _ => holds(&wants, states),
            }
        };
        // (c): every state cut to the goal's relations, deduplicated, against
        // the listed models cut the same way.
        let project = |facts: &BTreeSet<String>| -> BTreeSet<String> {
            facts
                .iter()
                .filter(|f| goal.contains(relation_of(f)))
                .cloned()
                .collect()
        };
        let classes: BTreeSet<BTreeSet<String>> = by_class.values().map(project).collect();
        let max_classes: BTreeSet<BTreeSet<String>> =
            max.iter().map(|c| project(&by_class[c])).collect();
        let listed: BTreeSet<BTreeSet<String>> = wants
            .iter()
            .map(|w| {
                w.closes
                    .iter()
                    .filter(|(rel, _)| goal.contains(*rel))
                    .flat_map(|(_, ext)| ext.iter().cloned())
                    .collect()
            })
            .collect();
        let projection = match expectation {
            Expectation::Contradiction => empty,
            _ => classes == listed,
        };
        let projection_max = match expectation {
            Expectation::Contradiction => empty,
            _ => max_classes == listed,
        };
        println!(
            "{}",
            json!({
                "path": file,
                "n": r.n(),
                "monotone": r.monotonicity.is_monotone(),
                "shape": shape,
                "listed": wants.len(),
                "goal": goal,
                "beyond_goal": beyond_goal,
                "negated": negated,
                "k_s": by_class.len(),
                "max": max.len(),
                "min": min.len(),
                "classes": classes.len(),
                "empty": if !empty { "no" } else if r.root_dead || !any_alive { "no-alive" } else { "all-owe" },
                "all": reading(&all),
                "maximal": reading(&pick(&max)),
                "minimal": reading(&pick(&min)),
                "projection": projection,
                "projection_max": projection_max,
            })
        );
    }
}
