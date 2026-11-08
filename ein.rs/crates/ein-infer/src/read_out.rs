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
use crate::detector::{Paths, SPOT_PARENTS};
use crate::error::SolveError;
use crate::events::sexpr;
use crate::expect::{Outcome, Report};
use crate::explore::{self, Answer, Ask, Verdict, Which};
use crate::region::Nodes;
use crate::verdict::{goal_bindings, query_value, solve_value};
use ein_ir::expect::Over;

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
    /// members met are not listed. Since M1e2 S1e2.2.26 (W27) a line mode's
    /// past `--region-limit` on a flagged program too: the members it
    /// certifies on its line, which are describe's, and the others met.
    Certified { others: u64 },
    /// A mode's answer: a line, an `-n` cut, `--single`'s one or two states.
    Members,
    /// `--count`'s: nothing is listed, and no state is read (M1e2
    /// T1e2.1c.4.6).
    Counted,
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
    /// What it asks about: its `:over`, all of `R` by default (M1e2
    /// T1e2.1a.5.3).
    pub over: Over,
    /// The classes are over the set [`Self::over`] names. When the run asked
    /// another — a mode's line, an `-n` cut, `--single` — they are over the
    /// states it listed, and say so.
    pub as_asked: bool,
    /// The classes are over a complete set — all of `R`, or all of the line
    /// the run listed — and not a lower bound.
    pub exact: bool,
    /// Sorted by size, then by facts.
    pub classes: Vec<Class>,
    /// The query's `:expect`, checked against [`Self::classes`].
    pub claim: Option<Report>,
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
    /// `-y` on a monotone program: the parent-path spot check, unbounded,
    /// whose every path reached its state (a path that did not is
    /// [`SolveError::SpotCheck`]). On a flagged program the check is the
    /// warning's `paths` ([`crate::detector::Warning::paths`]), and `None`
    /// here.
    pub spot: Option<Paths>,
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
    mut answer: Answer,
    opts: &explore::Options,
) -> Result<ReadOut, SolveError> {
    if answer.mode.ask == Ask::Count {
        return counted(root, terms, ast, answer, opts);
    }
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
            match answer.partial.as_deref() {
                // A line mode past `--region-limit` on a flagged program: its
                // states are the line members describe's partial description
                // certifies, and the other members met are counted, not
                // listed, and written by `--dump-answer`, as describe's are
                // (M1e2 S1e2.2.26, W27).
                Some(d) => {
                    let on_line = |s: &crate::described::State| match answer.mode.which {
                        Which::Minimal => s.minimal,
                        Which::Maximal => s.maximal,
                        Which::All => false,
                    };
                    let mut others = 0u64;
                    for s in d.states.iter().filter(|s| !on_line(s)) {
                        others += 1;
                        states.push(Listed {
                            closure: s.closure.clone(),
                            rep: s.rep.clone(),
                            commitments: None,
                            generators: s.generators.clone(),
                            minimal: false,
                            maximal: false,
                            listed: false,
                            model: Vec::new(),
                        });
                    }
                    Listing::Certified { others }
                }
                None => Listing::Members,
            }
        }
    };

    let goals = asked(root, ast);
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
    // A goal the compiler rejects — `(query :goal (?R Rex Animal))`, an
    // unbound relation head — ends the run where a state would be answered
    // from it, rather than projecting no row (M1a S1a.6.6). With no state
    // listed the goal is never compiled, which is the asymmetry
    // `query-goal-free-head-unsat.ein` pins.
    if !states.is_empty() {
        for g in &goals {
            if let Some(e) = crate::verdict::goal_plan_error(ast, terms, root, Some(g.goal)) {
                return Err(SolveError::Compile(e));
            }
        }
    }
    // Per goal, per state, its rows.
    let mut rows: Vec<Vec<Vec<Row>>> = vec![Vec::new(); goals.len()];
    let mut owing_states: Vec<OwingState> = Vec::new();
    // The detector's spot check: on a flagged program the parent paths of
    // each printed state of two commitments or more, at most `SPOT_PARENTS`
    // of each; under `-y` every one, and on a monotone program too (M1e2
    // S1e2.1c.5, the user's ruling of 2026-11-03).
    let unbounded = sanity(root, opts);
    let bound = (!unbounded).then_some(SPOT_PARENTS);
    let per_state = |n: usize| bound.map_or(n, |b| n.min(b));
    let mut paths = (answer.warning.is_some() || unbounded).then(|| Paths {
        bound,
        ..Paths::default()
    });
    if !states.is_empty() || !owing.is_empty() {
        let facts = |set: &[u32]| -> Vec<ein_core::FactId> {
            set.iter()
                .map(|&b| answer.candidates[b as usize].fact)
                .collect()
        };
        let mut nodes = Nodes::new(root, terms, ast, &answer.candidates)?;
        let mut direct: Vec<Option<Box<[ein_core::FactId]>>> = Vec::with_capacity(states.len());
        for s in states.iter_mut() {
            let spot = paths.is_some() && s.listed && s.rep.len() >= 2;
            let (model, per_goal, key) = nodes.with_fixpoint(&facts(&s.rep), |kb, terms| {
                let model = model_of(kb, terms, &named);
                let per_goal: Vec<Vec<Row>> = goals
                    .iter()
                    .map(|g| bindings(ast, terms, kb, g.goal))
                    .collect();
                (model, per_goal, spot.then(|| crate::canon::state_key(kb)))
            })?;
            s.model = model;
            direct.push(key);
            for (g, r) in per_goal.into_iter().enumerate() {
                rows[g].push(r);
            }
        }
        if let Some(p) = paths.as_mut() {
            let name = |set: &[u32]| -> String {
                let parts: Vec<&str> = set
                    .iter()
                    .map(|&b| answer.candidates[b as usize].text.as_str())
                    .collect();
                format!("{{{}}}", parts.join(", "))
            };
            for (s, key) in states.iter().zip(&direct) {
                let Some(key) = key else { continue };
                p.states += 1;
                for i in 0..per_state(s.rep.len()) {
                    let parent: Vec<u32> = s
                        .rep
                        .iter()
                        .enumerate()
                        .filter(|&(j, _)| j != i)
                        .map(|(_, &b)| b)
                        .collect();
                    let last = answer.candidates[s.rep[i] as usize].fact;
                    p.saturations += 1;
                    match nodes.by_parent(&facts(&parent), last)? {
                        None => p.dead += 1,
                        Some(k) => {
                            p.saturations += 1;
                            p.parents += 1;
                            if k != *key {
                                p.differ.push(format!(
                                    "{} by way of its parent without {}",
                                    name(&s.rep),
                                    answer.candidates[s.rep[i] as usize].text
                                ));
                            }
                        }
                    }
                }
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
    // Nothing printed to check, and states met that are not printed: what
    // checking theirs would cost, at most.
    if let Some(p) = paths.as_mut()
        && p.states == 0
        && states.iter().any(|s| !s.listed)
    {
        let cost: u64 = states
            .iter()
            .filter(|s| s.rep.len() >= 2)
            .map(|s| 2 * per_state(s.rep.len()) as u64)
            .sum();
        p.unprinted = Some((states.len() as u64, cost));
    }
    let spot = match (paths, answer.warning.as_mut()) {
        // A flagged program: the warning's `paths` row, which reports and
        // never fails the run (F21: detect, do not decide).
        (Some(p), Some(w)) => {
            w.paths = Some(p);
            None
        }
        // A monotone program under `-y`: every path must reach its state, by
        // the § 5.1 theorem, so one that does not is the engine's error.
        (Some(p), None) => {
            if let Some(first) = p.differ.first() {
                return Err(SolveError::SpotCheck(format!(
                    "-y: {first}, on a program the static check calls monotone — a \
                     parent path that reaches another state contradicts the § 5.1 \
                     theorem, so this is an engine error ({} of {} parent paths \
                     differ)",
                    p.differ.len(),
                    p.parents
                )));
            }
            Some(p)
        }
        (None, _) => None,
    };

    let queries = goals
        .iter()
        .zip(rows)
        .map(|(g, per_state)| {
            // The states the query is over, and whether they are the set it
            // asks about, complete.
            let (over, as_asked, exact) = match answer.described.as_deref() {
                Some(d) => {
                    let on = |s: &Listed| match g.over {
                        Over::All => true,
                        Over::Minimal => s.minimal,
                        Over::Maximal => s.maximal,
                    };
                    let idx: Vec<usize> = (0..states.len()).filter(|&i| on(&states[i])).collect();
                    (idx, true, d.exhausted)
                }
                None => {
                    // `--single` that decided one state or none has listed the
                    // whole of the part it asked; over all of R that state is
                    // on both lines, and an empty R has neither.
                    let whole = answer.mode.ask == Ask::Single
                        && answer.stopped.is_none()
                        && matches!(
                            answer.verdict,
                            Some(Verdict::Solution | Verdict::Contradiction | Verdict::Open)
                        );
                    let asked = (whole && answer.mode.which == Which::All)
                        || matches!(
                            (g.over, answer.mode.which),
                            (Over::All, Which::All)
                                | (Over::Minimal, Which::Minimal)
                                | (Over::Maximal, Which::Maximal)
                        );
                    // An `-n` run that exhausted its part found fewer than it
                    // asked for, so it listed the whole part.
                    let exact = asked && (whole || answer.exhausted);
                    // Over the states listed: a line mode past the limit
                    // meets members it does not list (M1e2 S1e2.2.26).
                    let listed: Vec<usize> =
                        (0..states.len()).filter(|&i| states[i].listed).collect();
                    (listed, asked, exact)
                }
            };
            let picked: Vec<(usize, Vec<Row>)> = per_state
                .into_iter()
                .enumerate()
                .filter(|(i, _)| over.contains(i))
                .collect();
            let classes = classes(ast, g.goal, g.text.as_deref(), picked);
            let claim = g
                .expect
                .map(|node| query_claim(ast, node, g.over, as_asked, exact, &classes, &answer));
            Query {
                goal: ein_ir::dump::dump_compact(ast, g.goal),
                over: g.over,
                as_asked,
                exact,
                classes,
                claim,
            }
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
        spot,
    })
}

/// One `(query …)` as the program writes it.
struct Asked {
    goal: NodeId,
    text: Option<String>,
    over: Over,
    expect: Option<NodeId>,
}

/// The program's `(query …)` forms, in order.
fn asked(root: &Kb, ast: &Ast) -> Vec<Asked> {
    root.program()
        .queries
        .iter()
        .filter_map(|q| {
            let goal = query_value(ast, q, "goal")?;
            let text = query_value(ast, q, "goal-text").and_then(|n| match ast.node(n) {
                Node::Str(s) => Some(ast.sym(s).to_string()),
                _ => None,
            });
            // The loader refused every other value.
            let over = query_value(ast, q, "over")
                .and_then(|n| Over::parse(ast, n).ok())
                .unwrap_or_default();
            Some(Asked {
                goal,
                text,
                over,
                expect: query_value(ast, q, "expect"),
            })
        })
        .collect()
}

/// Why `--count` checks no claim: the one line every claim it leaves carries.
pub const COUNT_CHECKS_NO_CLAIM: &str =
    "--count reads no state, so it checks no claim: a bare `ein solve FILE` checks it";

/// `--count`'s read-out — M1e2 T1e2.1c.4.6, the user's ruling of 2026-11-03:
/// describe's answer, read no further. No state's model is read, so nothing is
/// listed and no query is answered. Each query is kept with no class over the
/// states listed, which are none, and each claim the program states, a query's
/// `:expect` or its `(solve … :expect …)`, is not checked, with why, so that
/// none is skipped silently. On a flagged program the detector's parent-path
/// spot check is not run either, and its row says what it would cost, as it
/// does where describe prints no state of two commitments or more.
///
/// A goal the compiler rejects is refused where describe refuses it, when `R`
/// has a state, so the two exit alike on every program but one whose claim
/// describe refutes.
fn counted(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    mut answer: Answer,
    opts: &explore::Options,
) -> Result<ReadOut, SolveError> {
    let goals = asked(root, ast);
    let met = answer.described.as_deref().map_or(0, |d| d.states.len());
    if met > 0 {
        for g in &goals {
            if let Some(e) = crate::verdict::goal_plan_error(ast, terms, root, Some(g.goal)) {
                return Err(SolveError::Compile(e));
            }
        }
    }
    // What checking the met states would cost, at most: the warning's row on
    // a flagged program, and under `-y` the spot check's on a monotone one.
    let unbounded = sanity(root, opts);
    let bound = (!unbounded).then_some(SPOT_PARENTS);
    let mut spot = None;
    if let Some(d) = answer.described.as_deref()
        && (answer.warning.is_some() || unbounded)
    {
        let cost: u64 = d
            .states
            .iter()
            .filter(|s| s.rep.len() >= 2)
            .map(|s| 2 * bound.map_or(s.rep.len(), |b| s.rep.len().min(b)) as u64)
            .sum();
        let p = Paths {
            bound,
            unprinted: (met > 0).then_some((met as u64, cost)),
            ..Paths::default()
        };
        match answer.warning.as_mut() {
            Some(w) => w.paths = Some(p),
            None => spot = Some(p),
        }
    }
    let not_checked = || Report {
        outcome: Outcome::NotChecked,
        lines: vec![COUNT_CHECKS_NO_CLAIM.to_string()],
    };
    let queries = goals
        .iter()
        .map(|g| Query {
            goal: ein_ir::dump::dump_compact(ast, g.goal),
            over: g.over,
            as_asked: false,
            exact: false,
            classes: Vec::new(),
            claim: g.expect.map(|_| not_checked()),
        })
        .collect();
    let claim = root
        .program()
        .solve()
        .and_then(|s| solve_value(ast, s, "expect"))
        .map(|_| not_checked());
    Ok(ReadOut {
        answer,
        listing: Listing::Counted,
        states: Vec::new(),
        queries,
        claim,
        spot,
    })
}

/// `-y`, or the program's own `(config :lattice-sanity-check true)`: the
/// parent-path spot check unbounded, on every program (M1e2 S1e2.1c.5, the
/// user's ruling of 2026-11-03).
fn sanity(root: &Kb, opts: &explore::Options) -> bool {
    opts.sanity
        || root
            .program()
            .config
            .as_ref()
            .is_some_and(|c| c.lattice_sanity_check)
}

/// What the run asked about R, for a claim it could not check.
fn asked_words(answer: &Answer) -> String {
    match (answer.mode.which, answer.mode.ask) {
        (Which::All, Ask::Describe) => "all of R".to_string(),
        (Which::All, Ask::Find(n)) => format!("{n} of its states (-n {n})"),
        (Which::All, Ask::Single) => "whether it is one state (--single)".to_string(),
        (Which::Minimal, Ask::Describe) => "its minimal line".to_string(),
        (Which::Maximal, Ask::Describe) => "its maximal line".to_string(),
        (Which::Minimal, Ask::Find(n)) => format!("{n} states of its minimal line"),
        (Which::Maximal, Ask::Find(n)) => format!("{n} states of its maximal line"),
        (Which::Minimal, Ask::Single) => "whether its minimal line is one state".to_string(),
        (Which::Maximal, Ask::Single) => "whether its maximal line is one state".to_string(),
        (_, Ask::Count) => "how many states and commitments R holds (--count)".to_string(),
    }
}

fn classes_word(n: usize) -> String {
    format!("{n} {}", if n == 1 { "class" } else { "classes" })
}

/// A query's `:expect` against its classes (`region_language.md` § 5): the
/// classes it lists, as a set, are the query's. Checked when the classes are
/// over the set the query asks about and complete; otherwise not checked,
/// with why.
fn query_claim(
    ast: &Ast,
    node: NodeId,
    over: Over,
    as_asked: bool,
    exact: bool,
    classes: &[Class],
    answer: &Answer,
) -> Report {
    let not_checked = |why: String| Report {
        outcome: Outcome::NotChecked,
        lines: vec![why],
    };
    if !as_asked {
        return not_checked(format!(
            "the claim is about {}, and this run asked {}",
            over.words(),
            asked_words(answer)
        ));
    }
    if !exact {
        return not_checked(match answer.mode.ask {
            Ask::Describe => format!(
                "the claim is about {}, and this run did not exhaust it",
                over.words()
            ),
            _ => format!(
                "the claim is about {}, and this run asked {}",
                over.words(),
                asked_words(answer)
            ),
        });
    }
    // A claim's shape is the grammar's, so a loaded claim always reads.
    let e = ein_ir::expect::parse(ast, node);
    let want: Vec<Vec<String>> = e
        .models()
        .iter()
        .map(|m| {
            let mut v: Vec<String> = m
                .facts
                .iter()
                .map(|&f| ein_ir::expect::render(ast, f))
                .collect();
            v.sort();
            v.dedup();
            v
        })
        .collect();
    let have: Vec<&Vec<String>> = classes.iter().map(|c| &c.facts).collect();
    let model = |f: &[String]| {
        if f.is_empty() {
            "(model)".to_string()
        } else {
            format!("(model {})", f.join(" "))
        }
    };
    let missing: Vec<&Vec<String>> = have.iter().copied().filter(|c| !want.contains(c)).collect();
    let extra: Vec<&Vec<String>> = want.iter().filter(|w| !have.contains(w)).collect();
    if missing.is_empty() && extra.is_empty() {
        return Report {
            outcome: Outcome::Held,
            lines: Vec::new(),
        };
    }
    let head = format!(
        "the claim lists {}, the query has {}",
        classes_word(want.len()),
        have.len()
    );
    let first = match (missing.first(), extra.first()) {
        (Some(m), _) => format!("{head}: {} is not listed", model(m)),
        (None, Some(x)) => format!("{head}: {} is no class of {}", model(x), over.words()),
        (None, None) => head,
    };
    let mut lines = vec![first];
    lines.extend(
        missing
            .iter()
            .map(|m| format!("{} is a class the claim does not list", model(m))),
    );
    lines.extend(
        extra
            .iter()
            .map(|x| format!("the claim lists {}, which is no class", model(x))),
    );
    Report {
        outcome: Outcome::Failed,
        lines,
    }
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
                .map(|(k, v)| {
                    (
                        terms.sym(*k).to_string(),
                        crate::events::sexpr_value(terms, *v),
                    )
                })
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
fn classes(
    ast: &Ast,
    goal: NodeId,
    text: Option<&str>,
    per_state: Vec<(usize, Vec<Row>)>,
) -> Vec<Class> {
    let conj = conjuncts(ast, goal);
    let mut by_key: BTreeMap<Vec<String>, Class> = BTreeMap::new();
    for (i, rows) in per_state {
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
