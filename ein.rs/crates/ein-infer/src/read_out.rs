//! The read-out's data: what `ein solve` and `ein oracle` print, computed
//! once — M1e2
//! [T1e2.1a.5.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md)'s
//! third step, for the layout of
//! [`read_out.md`](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md).
//!
//! [`run`] asks [`crate::explore`] the question, then reads every state it
//! lists through **the node function every route uses**
//! ([`crate::region::Nodes`]): each state's `(model …)`, and each query's
//! projection classes over the states it is answered from. Every query is
//! answered from **one** solve (the diff's § 7.1), and the program's
//! `(solve … :expect …)` is checked against the `R` this run described, not
//! against a second one ([`crate::claim::check_described`]).
//!
//! What is printed, and how, is `ein-render`'s. Nothing here decides a word.

use std::collections::BTreeMap;

use ein_core::{Kb, Terms, render_why};
use ein_ir::{Ast, Node, NodeId};

use crate::described::{Described, Owing};
use crate::events::sexpr;
use crate::expect::{Outcome, Report};
use crate::explore::{self, Answer, Ask, Which};
use crate::region::Nodes;
use crate::solve::SolveError;
use crate::verdict::{goal_bindings, query_value, solve_value};

/// One binding row: `(variable, value)`, sorted by variable.
pub type Row = Vec<(String, String)>;

/// One state the run reached, as the read-out prints it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Listed {
    /// The candidates its fixpoint holds.
    pub closure: Box<[u32]>,
    /// A commitment that reaches it.
    pub rep: Box<[u32]>,
    /// The commitments of `R` that reach it, where the run counted them.
    pub commitments: Option<u64>,
    /// Its minimal commitments, where the run knows them.
    pub generators: Vec<Box<[u32]>>,
    /// On the minimal line, as far as the run certified it.
    pub minimal: bool,
    /// On the maximal line, as far as the run certified it.
    pub maximal: bool,
    /// Printed in the listing. On a partial description only the certified
    /// line members are; the other members met still answer the queries.
    pub listed: bool,
    /// The positive facts of the candidates' relations, root's included,
    /// sorted: what tells `R`'s states apart, and the `(model …)` printed.
    pub model: Vec<String>,
}

/// A frontier node of an empty `R` of the owing kind, with its model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwingState {
    pub node: Owing,
    pub model: Vec<String>,
}

/// What the listing is, which decides its form (rule 4 of the diff's § 3).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Listing {
    /// An exhausted description: all of `R`, printed as one `(region …)`.
    Region,
    /// R is empty and no node is alive: `(false)`.
    False,
    /// R is empty and every alive node owes: `(region (owing …))`.
    Owing(Vec<OwingState>),
    /// A partial description: its certified line members, as states. `others`
    /// members met are not listed.
    Certified { others: u64 },
    /// A mode's answer: a line, an `-n` cut, `--single`'s one or two states.
    Members,
}

/// One projection class: the goal's atoms under its rows, in the states that
/// give them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Class {
    /// The goal's conjuncts grounded by every row, sorted — the class's key
    /// and its `(model …)` in the query's `:expect`.
    pub facts: Vec<String>,
    /// Every binding row, each sorted by variable name, the rows sorted.
    pub rows: Vec<Row>,
    /// Each row rendered through the query's `:goal-text`.
    pub rendered: Vec<String>,
    /// The states that give it, as indices into [`ReadOut::states`].
    pub states: Vec<usize>,
}

/// One `(query …)`, answered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Query {
    /// The goal, as written.
    pub goal: String,
    /// The classes are over a complete set — all of `R`, or all of the line
    /// the run listed — and not a lower bound.
    pub exact: bool,
    /// Sorted by size, then by facts.
    pub classes: Vec<Class>,
}

/// What the run computed, for every surface to print.
#[derive(Debug)]
pub struct ReadOut {
    pub answer: Answer,
    pub listing: Listing,
    /// The states the run reached: under describe, every state of `R` met, in
    /// canonical order; under a mode, its states in the route's order.
    pub states: Vec<Listed>,
    pub queries: Vec<Query>,
    /// The program's `(solve … :expect …)`, checked.
    pub claim: Option<Report>,
}

impl ReadOut {
    /// The description, under describe.
    pub fn described(&self) -> Option<&Described> {
        self.answer.described.as_deref()
    }

    /// The states the listing prints.
    pub fn listed(&self) -> impl Iterator<Item = (usize, &Listed)> {
        self.states.iter().enumerate().filter(|(_, s)| s.listed)
    }
}

/// Ask `opts.mode` about `root`, a **freshly loaded, unsaturated** program, as
/// [`explore::run`] asks, and read every state it reaches.
pub fn run(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &explore::Options,
) -> Result<ReadOut, SolveError> {
    run_with(root, terms, ast, opts, &mut crate::events::Events::off())
}

/// [`run`], narrating the run into `events` ([`explore::run_with`]).
pub fn run_with(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &explore::Options,
    events: &mut crate::events::Events,
) -> Result<ReadOut, SolveError> {
    let answer = explore::run_with(root, terms, ast, opts, events)?;
    of_answer(root, terms, ast, answer, opts)
}

/// The read-out of an `answer` already computed about `root` — `ein oracle`'s,
/// whose answer is the whole cube's ([`explore::of_cube`]).
pub fn of_answer(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    answer: Answer,
    opts: &explore::Options,
) -> Result<ReadOut, SolveError> {
    let mut states: Vec<Listed> = Vec::new();
    let mut owing: Vec<Owing> = Vec::new();
    let listing = match answer.described.as_deref() {
        Some(d) => {
            for s in &d.states {
                states.push(Listed {
                    closure: s.closure.clone(),
                    rep: s.rep.clone(),
                    commitments: Some(s.commitments),
                    generators: s.generators.clone(),
                    minimal: s.minimal,
                    maximal: s.maximal,
                    listed: d.exhausted || s.minimal || s.maximal,
                    model: Vec::new(),
                });
            }
            if d.exhausted && d.states.is_empty() {
                if d.any_alive {
                    owing = d.frontier.clone();
                    Listing::Owing(Vec::new())
                } else {
                    Listing::False
                }
            } else if d.exhausted {
                Listing::Region
            } else {
                let listed = states.iter().filter(|s| s.listed).count() as u64;
                Listing::Certified {
                    others: states.len() as u64 - listed,
                }
            }
        }
        None => {
            for (i, s) in answer.states.iter().enumerate() {
                let generators: Vec<Box<[u32]>> = answer
                    .minimal
                    .iter()
                    .filter(|(_, at)| *at == i)
                    .map(|(set, _)| set.clone())
                    .collect();
                states.push(Listed {
                    closure: s.closure.clone(),
                    rep: s.set.clone(),
                    commitments: None,
                    // The commitment the route met reaches it, which is what
                    // a mode knows when it did not ask for the minimal ones.
                    generators: if generators.is_empty() {
                        vec![s.set.clone()]
                    } else {
                        generators
                    },
                    minimal: answer.mode.which == Which::Minimal,
                    maximal: answer.mode.which == Which::Maximal,
                    listed: true,
                    model: Vec::new(),
                });
            }
            Listing::Members
        }
    };

    // Every state and frontier node, read once through the node function.
    let named: Vec<String> = {
        let mut v: Vec<String> = answer
            .candidates
            .iter()
            .map(|c| relation_of(&c.text).to_string())
            .collect();
        v.sort();
        v.dedup();
        v
    };
    let goals: Vec<(NodeId, Option<String>)> = root
        .program()
        .queries
        .iter()
        .filter_map(|q| {
            let goal = query_value(ast, q, "goal")?;
            let text = query_value(ast, q, "goal-text").and_then(|n| match ast.node(n) {
                Node::Str(s) => Some(ast.sym(s).to_string()),
                _ => None,
            });
            Some((goal, text))
        })
        .collect();
    // Per goal, per state, its rows.
    let mut rows: Vec<Vec<Vec<Row>>> = vec![Vec::new(); goals.len()];
    let mut owing_states: Vec<OwingState> = Vec::new();
    if !states.is_empty() || !owing.is_empty() {
        let facts = |set: &[u32]| -> Vec<ein_core::FactId> {
            set.iter()
                .map(|&b| answer.candidates[b as usize].fact)
                .collect()
        };
        let mut nodes = Nodes::new(root, terms, ast, &answer.candidates)?;
        for s in states.iter_mut() {
            let (model, per_goal) = nodes.with_fixpoint(&facts(&s.rep), |kb, terms| {
                let model = model_of(kb, terms, &named);
                let per_goal: Vec<Vec<Row>> = goals
                    .iter()
                    .map(|&(goal, _)| bindings(ast, terms, kb, goal))
                    .collect();
                (model, per_goal)
            })?;
            s.model = model;
            for (g, r) in per_goal.into_iter().enumerate() {
                rows[g].push(r);
            }
        }
        for o in owing {
            let model =
                nodes.with_fixpoint(&facts(&o.set), |kb, terms| model_of(kb, terms, &named))?;
            owing_states.push(OwingState { node: o, model });
        }
        owing_states.sort_by(|a, b| a.model.cmp(&b.model));
    }
    let listing = match listing {
        Listing::Owing(_) => Listing::Owing(owing_states),
        l => l,
    };

    let exact = answer.exhausted;
    let queries = goals
        .iter()
        .zip(rows)
        .map(|(&(goal, ref text), per_state)| Query {
            goal: ein_ir::dump::dump_compact(ast, goal),
            exact,
            classes: classes(ast, goal, text.as_deref(), per_state),
        })
        .collect();

    let claim = match root.program().solve().cloned() {
        Some(solve) => match solve_value(ast, &solve, "expect") {
            Some(node) => Some(check_claim(root, terms, ast, node, &answer, opts)?),
            None => None,
        },
        None => None,
    };
    Ok(ReadOut {
        answer,
        listing,
        states,
        queries,
        claim,
    })
}

/// The program's region claim against this run: checked when the run
/// described all of `R`, and otherwise not checked, with why.
fn check_claim(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    node: NodeId,
    answer: &Answer,
    opts: &explore::Options,
) -> Result<Report, SolveError> {
    let claim =
        ein_ir::claim::parse(ast, node).map_err(|e| SolveError::Unsupported(e.to_string()))?;
    let not_checked = |why: String| Report {
        outcome: Outcome::NotChecked,
        lines: vec![why],
    };
    let Some(d) = answer.described.as_deref() else {
        let what = match (answer.mode.which, answer.mode.ask) {
            (Which::All, Ask::Find(n)) => format!("{n} of its states (-n {n})"),
            (Which::All, _) => "whether it is one state (--single)".to_string(),
            (Which::Minimal, _) => "its minimal line".to_string(),
            (Which::Maximal, _) => "its maximal line".to_string(),
        };
        return Ok(not_checked(format!(
            "the claim is about all of R, and this run asked {what}"
        )));
    };
    if !d.exhausted {
        return Ok(not_checked(
            "the claim is about all of R, and this run is partial".to_string(),
        ));
    }
    crate::claim::check_described(
        root,
        terms,
        ast,
        &claim,
        d,
        &crate::claim::Options {
            budget: opts.budget,
            limit: opts.limit,
            jobs: opts.jobs,
        },
    )
}

/// `(rel a b)` → `rel`.
fn relation_of(fact: &str) -> &str {
    fact.trim_start_matches('(')
        .split(|c: char| c.is_whitespace() || c == ')')
        .next()
        .unwrap_or("")
}

/// The positive facts of `named` relations, root's included, sorted.
fn model_of(kb: &Kb, terms: &Terms, named: &[String]) -> Vec<String> {
    let mut v: Vec<String> = kb
        .facts()
        .map(|f| sexpr(terms, f))
        .filter(|t| {
            !t.starts_with("(not ") && named.binary_search(&relation_of(t).to_string()).is_ok()
        })
        .collect();
    v.sort();
    v.dedup();
    v
}

/// The goal's binding rows in `kb`, each sorted by variable, sorted.
fn bindings(ast: &Ast, terms: &mut Terms, kb: &Kb, goal: NodeId) -> Vec<Row> {
    let mut rows: Vec<Row> = goal_bindings(ast, terms, kb, Some(goal))
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
    rows
}

fn conjuncts(ast: &Ast, goal: NodeId) -> Vec<NodeId> {
    if let Node::SForm { head, args } = ast.node(goal)
        && matches!(ast.node(head), Node::Atom(s) if ast.sym(s) == "and")
    {
        return ast.args(args).to_vec();
    }
    vec![goal]
}

/// A conjunct with a row's bindings substituted.
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

/// The projection classes of `per_state`, one goal's rows per state.
fn classes(ast: &Ast, goal: NodeId, text: Option<&str>, per_state: Vec<Vec<Row>>) -> Vec<Class> {
    let conj = conjuncts(ast, goal);
    let mut by_key: BTreeMap<Vec<String>, Class> = BTreeMap::new();
    for (i, rows) in per_state.into_iter().enumerate() {
        let mut facts: Vec<String> = rows
            .iter()
            .flat_map(|b| conj.iter().filter_map(|c| ground(ast, *c, b)))
            .collect();
        facts.sort();
        facts.dedup();
        by_key
            .entry(facts.clone())
            .or_insert_with(|| Class {
                rendered: match text {
                    Some(t) => rows
                        .iter()
                        .map(|r| {
                            let named: Vec<(String, String)> = r.clone();
                            render_why(t, &named)
                        })
                        .collect(),
                    None => Vec::new(),
                },
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
