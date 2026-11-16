//! A debt root was not told of, over the corpus — the census behind
//! Q-M1e.11.
//!
//! The universe H is the generator's list at root's fixpoint. Two things a
//! node can hold that root did not could make that list short: a **debt**,
//! an obligation instance the node owes and root did not, and a
//! **proposal**, a fact the generator would propose at the node and did not
//! at root. This census reads both, statically from root's plans and by the
//! cube.
//!
//! **It was taken before the obligations rung read a later debt**
//! ([`ein_infer::oblgen`], § *A debt root is not told of*), and its rows
//! priced that reading. The generator it calls is the engine's, so H here
//! now holds what such a debt would propose: the one corpus program its
//! `grow` column named then, `oblig-derived-activator.ein`, has no node that
//! proposes outside H, and what the column still finds is what the reading
//! leaves out, an activator written with a variable or an hrule that reads
//! what a commitment can change.
//!
//! ### The static half
//!
//! From [`ein_infer::monotone::classify`] over root's plans, with `D` the
//! relations a commitment can change and `D⁺` the same followed through the
//! rules root has no plan of ([`ein_infer::forbid::extendable`]):
//!
//! | column | what |
//! |---|---|
//! | `act` | the obligation rules whose activator relation is in `D`: an activator site of an obligation rule |
//! | `act+` | those whose activator relation is in `D⁺` and not in `D`: derived only through a rule root has no plan of |
//! | `armers` | each saturation plan of root that concludes an obligation rule's activator, as `rule>obligation`, with `*` where the plan reads a relation in `D` |
//! | `trigger` | the obligation plans with an obligation-trigger site: a commitment can add an instance of a plan root has |
//! | `hrule-reads` | the active hrule plans whose `:match` reads a relation in `D`, as `hrule:relation` |
//!
//! ### The dynamic half
//!
//! Where H is within `--limit`, every node of the cube: root's fixpoint
//! branched, the commitment written with hypothesis provenance, the
//! saturation resumed from root's snapshot, which is
//! `ein_infer::region`'s node function step for step, held to it here by
//! comparing every node's two bits and its count of debts with
//! [`ein_infer::region::compute`]'s on a second load (`check`). On each node
//! it reads the debts ([`ein_infer::obligations::tally`]) and, on an alive
//! node, the generator's list
//! ([`ein_infer::hypgen::generate_config_free`], the whole ladder):
//!
//! | column | what |
//! |---|---|
//! | `new-act` | alive nodes that owe an instance of a `(rule, activator)` root compiled no plan of |
//! | `new-trig` | alive nodes that owe an instance root did not, of a plan root has |
//! | `new-dead` | dead nodes that owe an instance root did not, of either kind |
//! | `grow` | alive nodes at which the generator proposes a fact that is not in H |
//! | `grown` | how many distinct such facts |
//! | `flip` | alive nodes whose rung is not root's, by rung |
//!
//! A row's `debts` and `proposals` columns list the instances and the facts,
//! `;`-separated, at most `--list` of each.
//!
//! The obligations rung reads `D` over every plan the memo holds, and here
//! those are root's and the ones every earlier node compiled. So a node's
//! rung, as this census reads it, can depend on the nodes read before it,
//! which the engine's own generator pass, taken at root before any node,
//! cannot. `flip` is where that would show.
//!
//! ### The universe closed under its nodes' proposals
//!
//! With `--iterate`, for each program whose cube has a `grow` node: the
//! least set `H*` that holds H and, for every alive node over `H*`, every
//! fact the generator proposes there. Round `k + 1` is the cube over round
//! `k`'s set with every proposal outside it added, by the same node
//! function, until a round adds none or the set passes `--limit`. Then R
//! over `H*`: its commitments, and its states by
//! [`ein_infer::canon::state_key`]. No route of the engine reads such a
//! set; the lines, each starting `# iterate`, say what one would be.
//!
//! ```sh
//! cargo run --release -p ein-infer --example owed_census -- \
//!     [--limit N] [--jobs J] [--list K] [--iterate] [FILE…] > rows.tsv
//! ```
//!
//! With no `FILE` it sweeps `corpus_files()`. One tab-separated row a
//! program, then a summary, each line of it starting `# `. It writes nothing
//! and changes no answer.

#[allow(dead_code)]
#[path = "shared/census.rs"]
mod census;

use std::collections::{BTreeMap, BTreeSet};
use std::ops::ControlFlow;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use ein_core::{FactId, Kb, Prov, Symbol, Terms, Value};
use ein_infer::compile::{SharedMemo, compile_rule};
use ein_infer::events::{Events, sexpr};
use ein_infer::hypgen::{HypGenStats, generate_config_free};
use ein_infer::monotone::{self, Rel, SiteKind};
use ein_infer::oblgen::Mode;
use ein_infer::obligations;
use ein_infer::region::{self, Outcome};
use ein_infer::saturator::{Saturator, Session, Snapshot};
use ein_ir::Ast;
use rustc_hash::FxHashSet;

/// One obligation instance: the rule, the activator's arguments, the match's
/// bindings.
type Key = (Symbol, Box<[Value]>, Box<[(Symbol, Value)]>);

/// The static half of a row.
#[derive(Default)]
struct Static {
    rung: &'static str,
    n: usize,
    obligation_rules: usize,
    obligation_plans: usize,
    hrules: usize,
    root_dead: bool,
    root_owes: usize,
    monotone: bool,
    sites: usize,
    act: BTreeSet<String>,
    act_chain: BTreeSet<String>,
    armers: BTreeSet<String>,
    trigger: BTreeSet<String>,
    hrule_reads: BTreeSet<String>,
}

/// The dynamic half of a row.
#[derive(Default)]
struct Cube {
    nodes: u64,
    alive: u64,
    k_c: u64,
    new_act: u64,
    new_trig: u64,
    new_dead: u64,
    grow: u64,
    /// How many distinct facts those nodes propose outside H.
    grown: usize,
    flip: BTreeMap<&'static str, u64>,
    debts: BTreeSet<String>,
    proposals: BTreeSet<String>,
    check: &'static str,
    /// `--iterate`'s lines, where a node grew.
    iterate: Vec<String>,
}

/// Root as `region`'s `prepare` leaves it: the fixpoint, the snapshot every
/// node resumes from, the memo, and the universe.
struct Prepared {
    snapshot: Snapshot,
    memo: SharedMemo,
    proposed: Vec<FactId>,
    rung: Mode,
}

fn prepare(kb: &mut Kb, terms: &mut Terms, ast: &Ast) -> Option<Prepared> {
    let memo = SharedMemo::default();
    let mut events = Events::off();
    let snapshot = {
        let mut s = Session {
            kb,
            terms,
            ast,
            events: &mut events,
            memo: memo.clone(),
        };
        let mut sat = Saturator::new(&mut s).ok()?;
        sat.saturate(&mut s, None, &mut |_| {}).ok()?;
        sat.snapshot(s.kb)
    };
    let mut proposed = Vec::new();
    let mut stats = HypGenStats::new();
    {
        let mut s = Session {
            kb,
            terms,
            ast,
            events: &mut events,
            memo: memo.clone(),
        };
        generate_config_free(&mut s, &mut stats, &mut |f| {
            proposed.push(f);
            ControlFlow::Continue(())
        })
        .ok()?;
    }
    proposed.sort();
    proposed.dedup();
    Some(Prepared {
        snapshot,
        memo,
        proposed,
        rung: stats.rung.mode,
    })
}

/// A node's fixpoint: `region`'s `fixpoint`, by the same calls.
fn fixpoint(root: &Kb, p: &Prepared, terms: &mut Terms, ast: &Ast, facts: &[FactId]) -> Option<Kb> {
    let mut fork = root.branch();
    for &h in facts {
        let (rel, args) = terms.facts.get(h);
        let args = args.to_vec();
        let prov = terms.provs.push(Prov::from_hypothesis(0, None));
        fork.add_and_index_fact(terms, rel, &args, Some(prov))
            .ok()?;
    }
    let mut events = Events::off();
    let mut s = Session {
        kb: &mut fork,
        terms,
        ast,
        events: &mut events,
        memo: p.memo.clone(),
    };
    let delta = p.snapshot.new_facts_of(s.kb);
    let mut sat = Saturator::resume(&mut s, &p.snapshot, delta).ok()?;
    sat.saturate(&mut s, None, &mut |_| {}).ok()?;
    Some(fork)
}

fn named(terms: &Terms, rule: Symbol, activator: &[Value]) -> String {
    if activator.is_empty() {
        return terms.sym(rule).to_string();
    }
    let args: Vec<String> = activator.iter().map(|&a| terms.display(a)).collect();
    format!("({} {})", terms.sym(rule), args.join(" "))
}

/// The obligation plans the memo holds, as `(rule, activator)`.
fn obligation_plans(kb: &Kb, memo: &SharedMemo) -> FxHashSet<(Symbol, Box<[Value]>)> {
    monotone::plans_of(memo)
        .iter()
        .filter(|p| kb.program().obligations.contains(p.rule))
        .map(|p| (p.rule, p.activator_args.clone()))
        .collect()
}

struct Read {
    stat: Static,
    cube: Result<Cube, &'static str>,
}

fn read(path: &Path, limit: u32, list: usize, iterate: bool) -> Result<Read, &'static str> {
    let (mut kb, mut terms, ast) = census::loaded(path).ok_or("no-load")?;
    let p = prepare(&mut kb, &mut terms, &ast).ok_or("no-fixpoint")?;
    let root_dead = ein_infer::contradiction::has_contradiction(&kb, &terms);
    let mut off = Events::off();
    let owes =
        obligations::tally(&kb, &mut terms, &ast, &p.memo, &mut off).map_err(|_| "no-tally")?;
    let root_keys: FxHashSet<Key> = owes
        .instances()
        .iter()
        .map(|o| (o.rule, o.activator.clone(), o.bindings.clone()))
        .collect();
    let root_plans = obligation_plans(&kb, &p.memo);
    let plans = monotone::plans_of(&p.memo);
    let hypothesised: FxHashSet<Symbol> =
        p.proposed.iter().map(|&f| terms.facts.get(f).0).collect();
    let m = monotone::classify(&kb, &terms, &plans, &hypothesised);
    let extendable: FxHashSet<Rel> = ein_infer::forbid::extendable(&kb, &terms, &ast, &plans, &m)
        .into_iter()
        .collect();

    let program = kb.program_arc();
    let mut stat = Static {
        rung: p.rung.as_str(),
        n: p.proposed.len(),
        obligation_rules: program.obligations.len(),
        obligation_plans: root_plans.len(),
        hrules: program.hrules.len(),
        root_dead,
        root_owes: owes.total(),
        monotone: m.is_monotone(),
        sites: m.sites.len(),
        ..Static::default()
    };
    for rule in program.obligations.values() {
        if rule.params.is_empty() {
            continue;
        }
        let rel = Rel {
            name: rule.name,
            negated: false,
        };
        if m.changes(rel, &terms) {
            stat.act.insert(terms.sym(rule.name).to_string());
        } else if extendable.contains(&rel) {
            stat.act_chain.insert(terms.sym(rule.name).to_string());
        }
    }
    for plan in plans.iter().filter(|p| program.rules.contains(p.rule)) {
        let reads_d = monotone::reads_of(plan, &terms)
            .iter()
            .any(|&(r, _)| m.changes(r, &terms));
        for w in monotone::writes_of(plan, &terms) {
            if !w.negated && program.obligations.contains(w.name) {
                stat.armers.insert(format!(
                    "{}>{}{}",
                    terms.sym(plan.rule),
                    terms.sym(w.name),
                    if reads_d { "*" } else { "" }
                ));
            }
        }
    }
    for site in &m.sites {
        if site.kind == SiteKind::ObligationTrigger {
            stat.trigger
                .insert(named(&terms, site.rule, &site.activator));
        }
    }
    // The active hrule plans, compiled as the generator compiles them, and
    // the relations of `D` each one's `:match` reads.
    if !program.hrules.is_empty() {
        let proposals = {
            let mut s = Session {
                kb: &mut kb,
                terms: &mut terms,
                ast: &ast,
                events: &mut off,
                memo: p.memo.clone(),
            };
            ein_infer::mechanisms::read(&mut s)
                .map_err(|_| "no-mechanisms")?
                .proposals
        };
        for proposal in proposals {
            let Some(rule) = program.hrules.get(proposal.rule) else {
                continue;
            };
            let Ok(plan) = compile_rule(&ast, &mut terms, rule, proposal.activator_fact) else {
                continue;
            };
            for (r, _) in monotone::reads_of(&plan, &terms) {
                if m.changes(r, &terms) {
                    stat.hrule_reads.insert(format!(
                        "{}:{}",
                        named(&terms, proposal.rule, &proposal.activator),
                        r.label(&terms)
                    ));
                }
            }
        }
    }

    // The cube, where H is within the limit and the engine answers.
    let n = p.proposed.len();
    let cube = (|| {
        if n > limit as usize || n >= 64 {
            return Err("over-limit");
        }
        let (mut kb2, mut terms2, ast2) = census::loaded(path).ok_or("no-load")?;
        let opts = region::Options {
            limit,
            keep_states: false,
            jobs: 1,
            deadline: None,
            cores: region::Cores::READ_OUT,
        };
        let r = match region::compute(&mut kb2, &mut terms2, &ast2, &opts, &mut |_| {}) {
            Ok(Outcome::Region(r)) => r,
            Ok(Outcome::Refused { .. }) => return Err("over-limit"),
            Err(_) => return Err("refused"),
        };
        // This load's candidate `i` is the region's bit `bit[i]`.
        let texts: Vec<String> = p.proposed.iter().map(|&f| sexpr(&terms, f)).collect();
        let mut bit = Vec::with_capacity(n);
        for t in &texts {
            match r.candidates.iter().position(|c| &c.text == t) {
                Some(b) => bit.push(b),
                None => return Err("mismatch"),
            }
        }
        if r.n() != n {
            return Err("mismatch");
        }
        let in_h: FxHashSet<FactId> = p.proposed.iter().copied().collect();
        let root = kb.sealed();
        let mut c = Cube {
            check: "ok",
            ..Cube::default()
        };
        let mut grown: BTreeSet<FactId> = BTreeSet::new();
        for mask in 0..1u64 << n {
            let facts: Vec<FactId> = (0..n)
                .filter(|i| mask >> i & 1 == 1)
                .map(|i| p.proposed[i])
                .collect();
            terms.provs.open_fork();
            let Some(mut fork) = fixpoint(root, &p, &mut terms, &ast, &facts) else {
                terms.provs.discard_fork();
                return Err("no-node");
            };
            let dead = ein_infer::contradiction::has_contradiction(&fork, &terms);
            let owes = obligations::tally(&fork, &mut terms, &ast, &p.memo, &mut off)
                .map_err(|_| "no-tally")?;
            // The engine's own node, by the region's mask.
            let theirs = (0..n)
                .filter(|i| mask >> i & 1 == 1)
                .fold(0u64, |m, i| m | 1 << bit[i]) as usize;
            let status = r.status[theirs];
            if (status & region::DEAD != 0) != dead || r.nodes[theirs].owes as usize != owes.total()
            {
                c.check = "differs";
            }
            c.nodes += 1;
            let mut by_activator = false;
            let mut by_trigger = false;
            for o in owes.instances() {
                let key = (o.rule, o.activator.clone(), o.bindings.clone());
                if root_keys.contains(&key) {
                    continue;
                }
                let compiled = root_plans.contains(&(o.rule, o.activator.clone()));
                if compiled {
                    by_trigger = true;
                } else {
                    by_activator = true;
                }
                if !dead && c.debts.len() < list {
                    c.debts.insert(format!(
                        "{} {}: {}",
                        if compiled { "trigger" } else { "activator" },
                        named(&terms, o.rule, &o.activator),
                        o.why
                    ));
                }
            }
            if dead {
                if by_activator || by_trigger {
                    c.new_dead += 1;
                }
            } else {
                c.alive += 1;
                if owes.is_empty() {
                    c.k_c += 1;
                }
                if by_activator {
                    c.new_act += 1;
                }
                if by_trigger {
                    c.new_trig += 1;
                }
                // The generator at this node: the ladder, config-free.
                let mut here = Vec::new();
                let mut stats = HypGenStats::new();
                {
                    let mut s = Session {
                        kb: &mut fork,
                        terms: &mut terms,
                        ast: &ast,
                        events: &mut off,
                        memo: p.memo.clone(),
                    };
                    generate_config_free(&mut s, &mut stats, &mut |f| {
                        here.push(f);
                        ControlFlow::Continue(())
                    })
                    .map_err(|_| "no-generator")?;
                }
                if stats.rung.mode != p.rung {
                    *c.flip.entry(stats.rung.mode.as_str()).or_default() += 1;
                }
                let mut grew = false;
                for f in here {
                    if !in_h.contains(&f) {
                        grew = true;
                        grown.insert(f);
                        if c.proposals.len() < list {
                            c.proposals.insert(sexpr(&terms, f));
                        }
                    }
                }
                if grew {
                    c.grow += 1;
                }
            }
            drop(fork);
            terms.provs.discard_fork();
        }
        if c.k_c != r.topology.size_r {
            c.check = "differs";
        }
        c.grown = grown.len();
        if iterate && !grown.is_empty() {
            c.iterate = closed_universe(root, &p, &mut terms, &ast, grown, limit)?;
        }
        Ok(c)
    })();
    Ok(Read { stat, cube })
}

/// `H*`, from H and round 0's proposals, and R over it — the module docs'
/// § The universe closed under its nodes' proposals.
fn closed_universe(
    root: &Kb,
    p: &Prepared,
    terms: &mut Terms,
    ast: &Ast,
    mut grown: BTreeSet<FactId>,
    limit: u32,
) -> Result<Vec<String>, &'static str> {
    let mut off = Events::off();
    let mut lines = Vec::new();
    let mut universe: Vec<FactId> = p.proposed.clone();
    let mut round = 0usize;
    loop {
        round += 1;
        let added: Vec<String> = grown.iter().map(|&f| sexpr(terms, f)).collect();
        universe.extend(grown.iter().copied());
        let n = universe.len();
        lines.push(format!(
            "round {round}: + {} = {n} candidates",
            added.join(" ")
        ));
        if n > limit as usize || n >= 64 {
            lines.push(format!("stopped: {n} candidates is past --limit {limit}"));
            return Ok(lines);
        }
        let held: FxHashSet<FactId> = universe.iter().copied().collect();
        grown = BTreeSet::new();
        let mut members: Vec<String> = Vec::new();
        let mut states: FxHashSet<Box<[FactId]>> = FxHashSet::default();
        let (mut alive, mut owing) = (0u64, 0u64);
        for mask in 0..1u64 << n {
            let facts: Vec<FactId> = (0..n)
                .filter(|i| mask >> i & 1 == 1)
                .map(|i| universe[i])
                .collect();
            terms.provs.open_fork();
            let Some(mut fork) = fixpoint(root, p, terms, ast, &facts) else {
                terms.provs.discard_fork();
                return Err("no-node");
            };
            if !ein_infer::contradiction::has_contradiction(&fork, terms) {
                alive += 1;
                let owes = obligations::tally(&fork, terms, ast, &p.memo, &mut off)
                    .map_err(|_| "no-tally")?;
                if owes.is_empty() {
                    states.insert(ein_infer::canon::state_key(&fork));
                    let mut texts: Vec<String> = facts.iter().map(|&f| sexpr(terms, f)).collect();
                    texts.sort();
                    members.push(format!("{{{}}}", texts.join(" ")));
                } else {
                    owing += 1;
                }
                let mut stats = HypGenStats::new();
                let mut s = Session {
                    kb: &mut fork,
                    terms,
                    ast,
                    events: &mut off,
                    memo: p.memo.clone(),
                };
                generate_config_free(&mut s, &mut stats, &mut |f| {
                    if !held.contains(&f) {
                        grown.insert(f);
                    }
                    ControlFlow::Continue(())
                })
                .map_err(|_| "no-generator")?;
            }
            drop(fork);
            terms.provs.discard_fork();
        }
        if grown.is_empty() {
            let mut texts: Vec<String> = universe.iter().map(|&f| sexpr(terms, f)).collect();
            texts.sort();
            lines.push(format!("closed at round {round}: H* = {}", texts.join(" ")));
            lines.push(format!(
                "over H*: {} nodes, {alive} alive, {owing} of them owing, R {} commitments in {} states",
                1u64 << n,
                members.len(),
                states.len()
            ));
            members.sort_by_key(|m| (m.len(), m.clone()));
            for m in members.iter().take(16) {
                lines.push(format!("in R: {m}"));
            }
            if members.len() > 16 {
                lines.push(format!("in R: and {} more", members.len() - 16));
            }
            return Ok(lines);
        }
    }
}

fn joined(names: &BTreeSet<String>, sep: &str) -> String {
    if names.is_empty() {
        "-".to_string()
    } else {
        names.iter().cloned().collect::<Vec<_>>().join(sep)
    }
}

fn main() {
    let mut limit = region::DEFAULT_LIMIT;
    let mut jobs = 1usize;
    let mut list = 8usize;
    let mut iterate = false;
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--limit" => {
                limit = args.next().and_then(|v| v.parse().ok()).expect("--limit N");
            }
            "--jobs" => jobs = args.next().and_then(|v| v.parse().ok()).expect("--jobs J"),
            "--list" => list = args.next().and_then(|v| v.parse().ok()).expect("--list K"),
            "--iterate" => iterate = true,
            _ => files.push(PathBuf::from(a)),
        }
    }
    if files.is_empty() {
        files = ein_corpus::corpus_files();
    }
    let groups: BTreeMap<String, String> = ein_corpus::corpus()
        .entry
        .into_iter()
        .map(|e| (e.path, e.group))
        .collect();

    // Each program on its own load, `jobs` at a time; the rows in the files'
    // order.
    let next = AtomicUsize::new(0);
    let done: Mutex<Vec<Option<Result<Read, &'static str>>>> =
        Mutex::new((0..files.len()).map(|_| None).collect());
    std::thread::scope(|scope| {
        for _ in 0..jobs.max(1) {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(path) = files.get(i) else { break };
                    let row = read(path, limit, list, iterate);
                    done.lock().expect("no reader panicked")[i] = Some(row);
                }
            });
        }
    });
    let done = done.into_inner().expect("no reader panicked");

    println!(
        "{}",
        [
            "file",
            "group",
            "status",
            "rung",
            "n",
            "obligation-rules",
            "obligation-plans",
            "hrules",
            "root",
            "root-owes",
            "verdict",
            "sites",
            "act",
            "act+",
            "armers",
            "trigger",
            "hrule-reads",
            "cube",
            "check",
            "nodes",
            "alive",
            "k_c",
            "new-act",
            "new-trig",
            "new-dead",
            "grow",
            "grown",
            "flip",
            "debts",
            "proposals",
        ]
        .join("\t")
    );
    let given = files.len();
    let mut skipped: BTreeMap<String, usize> = BTreeMap::new();
    let mut read_n = 0usize;
    // rung → [programs, within the limit, new-act, new-trig, grow, flip]
    let mut by_rung: BTreeMap<&'static str, [usize; 6]> = BTreeMap::new();
    let mut with_obligation = [0usize; 2];
    let mut lists: BTreeMap<&'static str, Vec<String>> = BTreeMap::new();
    let mut note = |what: &'static str, line: String| lists.entry(what).or_default().push(line);

    for (path, row) in files.iter().zip(done) {
        let rel = census::relative(path);
        let group = groups.get(&rel).map_or("-", |g| g.as_str());
        let row = match row.expect("every file was read") {
            Ok(row) => row,
            Err(why) => {
                *skipped.entry(format!("{why} ({group})")).or_default() += 1;
                println!("{rel}\t{group}\t{why}");
                continue;
            }
        };
        read_n += 1;
        let s = &row.stat;
        let rung = by_rung.entry(s.rung).or_default();
        rung[0] += 1;
        if s.obligation_rules > 0 {
            with_obligation[0] += 1;
            if s.obligation_plans > 0 {
                with_obligation[1] += 1;
            }
        }
        let mut cells = vec![
            rel.clone(),
            group.to_string(),
            "ok".to_string(),
            s.rung.to_string(),
            s.n.to_string(),
            s.obligation_rules.to_string(),
            s.obligation_plans.to_string(),
            s.hrules.to_string(),
            if s.root_dead { "dead" } else { "alive" }.to_string(),
            s.root_owes.to_string(),
            if s.monotone { "monotone" } else { "flagged" }.to_string(),
            s.sites.to_string(),
            joined(&s.act, ","),
            joined(&s.act_chain, ","),
            joined(&s.armers, ","),
            joined(&s.trigger, ","),
            joined(&s.hrule_reads, ","),
        ];
        let reach = match &row.cube {
            Ok(_) => "within the limit".to_string(),
            Err(why) => format!("{why}, n = {}", s.n),
        };
        if !s.act.is_empty() {
            note("act", format!("{rel} ({}; {reach})", joined(&s.act, ", ")));
        }
        if !s.act_chain.is_empty() {
            note(
                "act+",
                format!("{rel} ({}; {reach})", joined(&s.act_chain, ", ")),
            );
        }
        if !s.armers.is_empty() {
            note(
                "armers",
                format!("{rel} ({}; {reach})", joined(&s.armers, ", ")),
            );
        }
        if !s.trigger.is_empty() {
            note(
                "trigger",
                format!("{rel} ({}; {reach})", joined(&s.trigger, ", ")),
            );
        }
        if !s.hrule_reads.is_empty() {
            note(
                "hrule-reads",
                format!("{rel} ({}; {reach})", joined(&s.hrule_reads, ", ")),
            );
        }
        match &row.cube {
            Ok(c) => {
                rung[1] += 1;
                let flips: u64 = c.flip.values().sum();
                for (at, n) in [(2, c.new_act), (3, c.new_trig), (4, c.grow), (5, flips)] {
                    if n > 0 {
                        rung[at] += 1;
                    }
                }
                if c.check != "ok" {
                    note("check", rel.clone());
                }
                if c.new_act > 0 {
                    note("new-act", format!("{rel} (alive nodes: {})", c.new_act));
                }
                if c.new_trig > 0 {
                    note("new-trig", format!("{rel} (alive nodes: {})", c.new_trig));
                }
                if c.grow > 0 {
                    note(
                        "grow",
                        format!(
                            "{rel} (alive nodes: {}, rung {}, {})",
                            c.grow,
                            s.rung,
                            if s.monotone { "monotone" } else { "flagged" }
                        ),
                    );
                }
                if flips > 0 {
                    note("flip", format!("{rel} (alive nodes: {flips})"));
                }
                let flip: Vec<String> = c.flip.iter().map(|(m, n)| format!("{m}:{n}")).collect();
                cells.extend([
                    "cube".to_string(),
                    c.check.to_string(),
                    c.nodes.to_string(),
                    c.alive.to_string(),
                    c.k_c.to_string(),
                    c.new_act.to_string(),
                    c.new_trig.to_string(),
                    c.new_dead.to_string(),
                    c.grow.to_string(),
                    c.grown.to_string(),
                    if flip.is_empty() {
                        "-".to_string()
                    } else {
                        flip.join(",")
                    },
                    joined(&c.debts, "; "),
                    joined(&c.proposals, "; "),
                ]);
            }
            Err(why) => cells.push(why.to_string()),
        }
        println!("{}", cells.join("\t"));
        if let Ok(c) = &row.cube {
            for line in &c.iterate {
                println!("# iterate {rel}: {line}");
            }
        }
    }

    println!("# {given} files, {read_n} read");
    for (why, n) in &skipped {
        println!("# not read: {n} {why}");
    }
    println!(
        "# {} programs declare an obligation rule, {} of them with a plan at root",
        with_obligation[0], with_obligation[1]
    );
    println!(
        "# rung\tprograms\tcube within --limit {limit}\tnew-act\tnew-trig\tgrow\tflip (programs with a node of each)"
    );
    for (rung, [programs, within, act, trig, grow, flip]) in &by_rung {
        println!("# {rung}\t{programs}\t{within}\t{act}\t{trig}\t{grow}\t{flip}");
    }
    for what in [
        "act",
        "act+",
        "armers",
        "trigger",
        "hrule-reads",
        "new-act",
        "new-trig",
        "grow",
        "flip",
        "check",
    ] {
        let rows = lists.remove(what).unwrap_or_default();
        println!("# {what}: {} programs", rows.len());
        for r in rows {
            println!("#   {r}");
        }
    }
}
