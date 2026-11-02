//! The node census — M1e2
//! [S1e2.1b.2](../../../../plans/m1e2_correctness/p1e2.1b_the_non_monotone_lattice/s1e2.1b.2_fork_or_scratch.md):
//! a node's state built two ways, compared node for node on every cube the
//! oracle reaches.
//!
//! - **Fork** is the oracle's node (`region.rs` § What a node is): root's
//!   fixpoint, the commitment written in, the saturation resumed. It is read
//!   through [`region::Nodes`], the node function every route shares, and its
//!   two status bits are held to [`region::compute`]'s on a second load.
//! - **Scratch** is `utils/region.py`'s node: the program's text with the
//!   commitment appended as given facts, loaded and saturated from nothing.
//!
//! Both are read the same way: the two status bits, every fact of the
//! fixpoint, and the **stale** facts. A fact is stale when its primary
//! derivation was admitted under an `absent` guard that the node's own
//! fixpoint now falsifies. The guard is re-judged by the engine's own matcher
//! ([`Matcher::holds`]), on the plan and the bindings the provenance names.
//! Where no plan of the fact's rule reproduces the provenance's recorded
//! `absent` patterns, the check falls back to those patterns and counts the
//! fact as *inexact*.
//!
//! Per program it also takes the rule dependency graph and its cycles through
//! an odd `absent`, over all relations and over `D` (option (D)'s refusal), and
//! the parent-path check `-y` used to run ([`sanity::check_commutativity`]).
//! `--invariance` runs both routes under `--jobs`, permuted interners
//! (`id_order_invariance`'s permutation), every order of equal-priority rules
//! and every `(config …)` flag flipped. `--cost` times both routes per node.
//!
//! ```sh
//! cargo run --release -p ein-infer --example node_census                     # the flagged cubes within the limit
//! cargo run --release -p ein-infer --example node_census -- --control        # the monotone cubes: the theorem's premise
//! cargo run --release -p ein-infer --example node_census -- --strata         # (D): every program that loads and saturates
//! cargo run --release -p ein-infer --example node_census -- --written        # the programs written into this file
//! cargo run --release -p ein-infer --example node_census -- --invariance     # the levers, on the flagged cubes
//! cargo run --release -p ein-infer --example node_census -- --cost [--control]
//! cargo run --release -p ein-infer --example node_census -- [--json] FILE…   # any files, e.g. S1e2.4.1's probes
//! ```
//!
//! The selection is `site_census`'s, `corpus_files()` plus the register's
//! witnesses. No route reads this file, no command reaches it, and it writes
//! nothing.

#[allow(dead_code)]
#[path = "shared/census.rs"]
mod census;

use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use ein_core::{FactId, IntId, Kb, NafArg, NafRef, Prov, Symbol, Tag, Terms, Value};
use ein_corpus::repo_root;
use ein_infer::compile::SharedMemo;
use ein_infer::events::{Events, sexpr};
use ein_infer::match_::Matcher;
use ein_infer::monotone;
use ein_infer::mt19937::Mt19937;
use ein_infer::plan::{Disjunct, NafGuard, Plan, Slot, Span, Step};
use ein_infer::region::{self, DEAD, OWING, Outcome, Topology};
use ein_infer::saturator::{DEFAULT_PRIORITY, Saturator, Session};
use ein_infer::{contradiction, obligations, sanity};
use ein_ir::{Ast, load, parse};
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use serde_json::{Value as Json, json};

const LIMIT: usize = region::DEFAULT_LIMIT as usize;

// ── Programs ────────────────────────────────────────────────────────

/// A program as text, with the directory its imports resolve from.
#[derive(Clone)]
struct Source {
    name: String,
    text: String,
    dir: PathBuf,
}

impl Source {
    fn of(path: &Path) -> Option<Source> {
        Some(Source {
            name: census::relative(path),
            text: std::fs::read_to_string(path).ok()?,
            dir: path.parent()?.to_path_buf(),
        })
    }

    fn with_text(&self, text: String) -> Source {
        Source {
            text,
            ..self.clone()
        }
    }

    /// Parse and load the text plus `extra`, into `terms`.
    fn load(&self, extra: &str, terms: Terms) -> Option<(Kb, Terms, Ast)> {
        let text = if extra.is_empty() {
            self.text.clone()
        } else {
            format!("{}\n{extra}\n", self.text)
        };
        let mut ast = Ast::new();
        let mut terms = terms;
        let forms = parse(&mut ast, &text, Some(&self.name)).ok()?;
        let kb = load(&mut ast, &mut terms, &forms, Some(&self.dir)).ok()?;
        Some((kb, terms, ast))
    }
}

/// Programs written into the census, for the readings no corpus program
/// within the limit shows.
const WRITTEN: &[(&str, &str)] = &[
    // The synthesizer's refusal (`region_synth.py`'s `RootDead`): ∅ dead and
    // R not empty. Written by hand in its shape: one dead implicant, ¬A.
    (
        "dead_root",
        "(relation thing T)\n(relation h T)\n(thing A)\n\
         (rule dead-0 () :match (and (thing ?x) (absent (h ?x))) :assert (false) :priority 300)\n\
         (query :goal (h ?x))\n",
    ),
    // The same, through a derived positive (S1e2.4.1's `via` class): `p ← ¬h`,
    // `p → ⊥`. The refutation reads no `absent`; the rule that derived its
    // premise did.
    (
        "dead_root_via",
        "(relation thing T)\n(relation h T)\n(relation p T)\n(thing A)\n\
         (rule derive-p () :match (and (thing ?x) (absent (h ?x))) :assert (p ?x) :priority 200)\n\
         (rule p-dead () :match (p ?x) :assert (false) :priority 300)\n\
         (query :goal (h ?x))\n",
    ),
    // Stratified, and still stale from scratch: `q ← ¬r` and `p ← ¬q` wait at
    // one boundary, and the earlier-written one is admitted first.
    (
        "stratified_stale",
        "(relation thing T)\n(relation p T)\n(relation q T)\n(relation r T)\n(thing one)\n\
         (rule derive-p () :match (and (thing ?x) (absent (q ?x))) :assert (p ?x) :priority 200)\n\
         (rule derive-q () :match (and (thing ?x) (absent (r ?x))) :assert (q ?x) :priority 200)\n\
         (query :goal (r ?x))\n",
    ),
];

fn written() -> Vec<Source> {
    WRITTEN
        .iter()
        .map(|(name, text)| Source {
            name: format!("written:{name}"),
            text: text.to_string(),
            dir: repo_root().join("examples"),
        })
        .collect()
}

// ── Reading a fixpoint ──────────────────────────────────────────────

/// One node's fixpoint, read.
#[derive(Clone, Default)]
struct Read {
    dead: bool,
    owes: u32,
    /// Every fact, rendered and sorted — only when kept.
    facts: Vec<String>,
    /// A hash of the sorted rendered facts, always.
    digest: u64,
    /// The rule that derived each fact, where one did — only when kept.
    rule: BTreeMap<String, String>,
    /// The stale facts, as `fact ← rule (absent …)`.
    stale: Vec<String>,
    /// Facts whose primary derivation rests on a stale fact, the stale ones
    /// included.
    resting: usize,
    /// A contradiction's witness rests on a stale fact.
    dead_on_stale: bool,
    /// Stale checks that fell back to the recorded patterns.
    inexact: usize,
    refuted_by: Vec<String>,
}

impl Read {
    fn status(&self) -> u8 {
        (if self.dead { DEAD } else { 0 }) | (if self.owes > 0 { OWING } else { 0 })
    }
}

fn class(status: u8) -> &'static str {
    if status & DEAD != 0 {
        "dead"
    } else if status & OWING != 0 {
        "owing"
    } else {
        "R"
    }
}

fn read(kb: &Kb, terms: &mut Terms, ast: &Ast, plans: &[Arc<Plan>], keep: bool) -> Read {
    let cs = contradiction::detect(kb, terms);
    let owes = obligations::tally(kb, terms, ast, &SharedMemo::default(), &mut Events::off())
        .map(|o| o.total() as u32)
        .unwrap_or(u32::MAX);
    let mut facts: Vec<String> = kb.facts().map(|f| sexpr(terms, f)).collect();
    facts.sort();
    let mut h = FxHasher::default();
    facts.hash(&mut h);
    let (stale, inexact) = stale_facts(kb, terms, ast, plans);
    let ids: Vec<FactId> = stale.iter().map(|s| s.0).collect();
    let rests = resting(kb, terms, &ids);
    let dead_on_stale = cs.iter().any(|c| {
        [c.positive, Some(c.negative)]
            .into_iter()
            .flatten()
            .any(|f| rests.contains(&f))
    });
    let mut refuted_by: Vec<String> = cs
        .iter()
        .flat_map(|c| [c.positive, Some(c.negative)])
        .flatten()
        .filter_map(|f| kb.primary(f))
        .map(|p| match terms.provs.get(p).rule {
            Some(r) => terms.sym(r).to_string(),
            None => "(given)".to_string(),
        })
        .collect();
    refuted_by.sort();
    refuted_by.dedup();
    let mut rule = BTreeMap::new();
    if keep {
        for f in kb.facts() {
            if let Some(r) = kb.primary(f).and_then(|p| terms.provs.get(p).rule) {
                rule.insert(sexpr(terms, f), terms.sym(r).to_string());
            }
        }
    }
    Read {
        dead: !cs.is_empty(),
        owes,
        facts: if keep { facts } else { Vec::new() },
        digest: h.finish(),
        rule,
        stale: stale.into_iter().map(|s| s.1).collect(),
        resting: rests.len(),
        dead_on_stale,
        inexact,
        refuted_by,
    }
}

/// Every fact whose primary derivation's `absent` guard the fixpoint now
/// falsifies, rendered with the rule and the guard; and how many checks fell
/// back to the recorded patterns.
fn stale_facts(
    kb: &Kb,
    terms: &Terms,
    ast: &Ast,
    plans: &[Arc<Plan>],
) -> (Vec<(FactId, String)>, usize) {
    let mut out = Vec::new();
    let mut inexact = 0;
    let mut m = Matcher::new();
    let mut ids: Vec<FactId> = kb.facts().collect();
    ids.sort_by_key(|&f| sexpr(terms, f));
    for f in ids {
        let Some(p) = kb.primary(f) else { continue };
        let prov = terms.provs.get(p);
        if prov.absent.is_empty() {
            continue;
        }
        let Some(rule) = prov.rule else { continue };
        let mut verdict: Option<bool> = None;
        'plans: for plan in plans.iter().filter(|pl| pl.rule == rule) {
            let regs = regs_of(plan, &prov.bindings);
            for d in plan.disjuncts.iter() {
                if d.guards.is_empty() || refs_of(plan, d, &regs).as_slice() != &*prov.absent {
                    continue;
                }
                verdict = Some(
                    plan.guards(d.guards)
                        .iter()
                        .any(|g| m.holds(kb, terms, ast, plan, g, &regs)),
                );
                break 'plans;
            }
        }
        let falsified = match verdict {
            Some(v) => v,
            None => {
                inexact += 1;
                prov.absent.iter().any(|r| naf_holds(kb, terms, r))
            }
        };
        if falsified {
            let guards: Vec<String> = prov
                .absent
                .iter()
                .map(|r| format!("(absent {})", naf_text(terms, r.rel, &r.args)))
                .collect();
            out.push((
                f,
                format!(
                    "{} ← {} {}",
                    sexpr(terms, f),
                    terms.sym(rule),
                    guards.join(" ")
                ),
            ));
        }
    }
    (out, inexact)
}

/// The facts whose primary derivation reaches one of `stale`.
fn resting(kb: &Kb, terms: &Terms, stale: &[FactId]) -> FxHashSet<FactId> {
    let mut memo: FxHashMap<FactId, bool> = stale.iter().map(|&f| (f, true)).collect();
    fn rests(kb: &Kb, terms: &Terms, f: FactId, memo: &mut FxHashMap<FactId, bool>) -> bool {
        if let Some(&v) = memo.get(&f) {
            return v;
        }
        memo.insert(f, false); // a cycle reaches nothing new
        let premises: Vec<FactId> = kb
            .primary(f)
            .map(|p| terms.provs.get(p).premises.to_vec())
            .unwrap_or_default();
        let v = premises.into_iter().any(|q| rests(kb, terms, q, memo));
        memo.insert(f, v);
        v
    }
    kb.facts()
        .filter(|&f| rests(kb, terms, f, &mut memo))
        .collect()
}

/// The parent register file a firing ran with, from its provenance: the
/// activator's seed, then every binding by name.
fn regs_of(plan: &Plan, bindings: &[(Symbol, Value)]) -> Vec<Value> {
    let mut regs = vec![Value::UNBOUND; plan.n_regs as usize];
    for (i, (_, v)) in plan.seed.iter().enumerate() {
        regs[i] = *v;
    }
    for (name, v) in bindings {
        if let Some(r) = plan.reg_names.iter().position(|n| n == name) {
            regs[r] = *v;
        }
    }
    regs
}

/// The `absent` patterns a firing through `d` records — the saturator's
/// `negative_premises`, rebuilt so the plan can be told from its provenance.
fn refs_of(plan: &Plan, d: &Disjunct, regs: &[Value]) -> Vec<NafRef> {
    fn ground(plan: &Plan, g: &NafGuard, slot: &Slot, regs: &[Value]) -> NafArg {
        match slot {
            Slot::Reg(r) => match g.scope_of[*r as usize] {
                Some(parent) => {
                    let v = regs[parent as usize];
                    if v.is_unbound() || v.as_fact().is_some() {
                        NafArg::Free
                    } else {
                        NafArg::Value(v)
                    }
                }
                None => NafArg::Free,
            },
            Slot::Const(v) => NafArg::Value(*v),
            Slot::Nested { rel, slots } => NafArg::Nested {
                rel: *rel,
                args: plan
                    .slots(*slots)
                    .iter()
                    .map(|s| ground(plan, g, s, regs))
                    .collect(),
            },
            Slot::Opaque(_) => NafArg::Free,
        }
    }
    fn collect(plan: &Plan, g: &NafGuard, span: Span, regs: &[Value], out: &mut Vec<NafRef>) {
        for step in plan.steps(span) {
            match step {
                Step::Rel(r) => out.push(NafRef {
                    rel: r.rel,
                    args: plan
                        .slots(r.slots)
                        .iter()
                        .map(|s| ground(plan, g, s, regs))
                        .collect(),
                }),
                Step::Absent { sub } => collect(plan, g, *sub, regs, out),
                Step::Guard { .. } => {}
            }
        }
    }
    let mut out = Vec::new();
    for g in plan.guards(d.guards) {
        collect(plan, g, g.sub, regs, &mut out);
    }
    out.dedup();
    let mut seen: Vec<NafRef> = Vec::new();
    for r in out {
        if !seen.contains(&r) {
            seen.push(r);
        }
    }
    seen
}

/// Does some fact match a recorded `absent` pattern? The fallback.
fn naf_holds(kb: &Kb, terms: &Terms, r: &NafRef) -> bool {
    fn args_match(terms: &Terms, args: &[Value], pat: &[NafArg]) -> bool {
        args.len() == pat.len()
            && args.iter().zip(pat).all(|(v, p)| match p {
                NafArg::Free => true,
                NafArg::Value(x) => v == x,
                NafArg::Nested { rel, args: ps } => v.as_fact().is_some_and(|f| {
                    let (r2, a2) = terms.facts.get(f);
                    r2 == *rel && args_match(terms, a2, ps)
                }),
            })
    }
    kb.facts_of(r.rel)
        .any(|f| args_match(terms, terms.facts.args(f), &r.args))
}

fn naf_text(terms: &Terms, rel: Symbol, args: &[NafArg]) -> String {
    let mut s = format!("({}", terms.sym(rel));
    for a in args {
        s.push(' ');
        match a {
            NafArg::Free => s.push('_'),
            NafArg::Value(v) => s.push_str(&ein_infer::events::sexpr_value(terms, *v)),
            NafArg::Nested { rel, args } => s.push_str(&naf_text(terms, *rel, args)),
        }
    }
    s.push(')');
    s
}

// ── The two routes ──────────────────────────────────────────────────

fn bits(mask: u64, n: usize) -> impl Iterator<Item = usize> {
    (0..n).filter(move |b| mask >> b & 1 == 1)
}

fn name(cands: &[String], mask: u64) -> String {
    if mask == 0 {
        return "∅".to_string();
    }
    let v: Vec<&str> = bits(mask, cands.len()).map(|b| cands[b].as_str()).collect();
    format!("{{{}}}", v.join(", "))
}

/// What both routes share: root's universe and its static verdict.
#[derive(Clone, Default)]
struct Universe {
    cands: Vec<String>,
    monotone: bool,
    sites: Vec<String>,
    site_rules: BTreeSet<String>,
    strata: Json,
    /// The unsaturated program's facts, for "beyond the program".
    base: BTreeSet<String>,
}

struct ForkOut {
    uni: Universe,
    reads: Vec<Read>,
    terms: Terms,
}

/// The fork route: root's fixpoint through `census::root_of`, then every
/// node through `region::Nodes`. `None` where the program does not load or
/// saturate; no reads past the limit.
fn fork_route(src: &Source, terms0: Terms, keep: bool) -> Option<ForkOut> {
    fork_route_with(src, terms0, keep, true)
}

/// [`fork_route`], with `nodes = false` for the universe and the graph alone.
fn fork_route_with(src: &Source, terms0: Terms, keep: bool, nodes: bool) -> Option<ForkOut> {
    let (mut kb, mut terms, ast) = src.load("", terms0)?;
    let base: BTreeSet<String> = kb.facts().map(|f| sexpr(&terms, f)).collect();
    let (root, ()) = census::root_of(&mut kb, &mut terms, &ast, |_, _| Some(()))?;
    let mut cands = root.proposed.clone();
    cands.sort_by_key(|&f| sexpr(&terms, f));
    cands.dedup();
    let texts: Vec<String> = cands.iter().map(|&f| sexpr(&terms, f)).collect();
    let n = cands.len();
    let m = &root.monotonicity;
    let dynamic: BTreeSet<String> = m.dynamic.iter().map(|r| r.label(&terms)).collect();
    let uni = Universe {
        cands: texts,
        monotone: m.is_monotone(),
        sites: m.sites.iter().map(|s| s.render(&terms)).collect(),
        site_rules: m
            .sites
            .iter()
            .map(|s| terms.sym(s.rule).to_string())
            .collect(),
        strata: strata(&kb, &terms, &root.plans, &dynamic),
        base,
    };
    if n > LIMIT || !nodes {
        return Some(ForkOut {
            uni,
            reads: Vec::new(),
            terms,
        });
    }
    let plans = root.plans.clone();
    let mut reads = Vec::with_capacity(1 << n);
    {
        let mut nodes = region::Nodes::new(&mut kb, &mut terms, &ast, &[]).ok()?;
        for mask in 0..1u64 << n {
            let facts: Vec<FactId> = bits(mask, n).map(|b| cands[b]).collect();
            let r = nodes
                .with_fixpoint(&facts, |kb, terms| read(kb, terms, &ast, &plans, keep))
                .ok()?;
            reads.push(r);
        }
    }
    Some(ForkOut { uni, reads, terms })
}

/// One scratch node: the text plus the commitment as given facts, saturated
/// from nothing.
fn scratch_node(src: &Source, commitment: &str, keep: bool, terms0: Terms) -> Option<Read> {
    let (mut kb, mut terms, ast) = src.load(commitment, terms0)?;
    let memo = SharedMemo::default();
    let mut events = Events::off();
    {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: memo.clone(),
        };
        let mut sat = Saturator::new(&mut s).ok()?;
        sat.saturate(&mut s, None, &mut |_| {}).ok()?;
    }
    let plans = monotone::plans_of(&memo);
    Some(read(&kb, &mut terms, &ast, &plans, keep))
}

fn commitment_text(cands: &[String], mask: u64) -> String {
    bits(mask, cands.len())
        .map(|b| cands[b].as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

type TermsFn<'a> = &'a (dyn Fn() -> Terms + Sync);

/// Every scratch node of the cube over `cands`, on `jobs` threads. Each node
/// is its own load, so the order they run in is nothing a node can see.
fn scratch_route(
    src: &Source,
    cands: &[String],
    keep: bool,
    jobs: usize,
    terms0: TermsFn<'_>,
) -> Vec<Option<Read>> {
    let size = 1usize << cands.len();
    let mut out: Vec<Option<Read>> = vec![None; size];
    let jobs = jobs.max(1).min(size);
    let chunk = size.div_ceil(jobs);
    std::thread::scope(|sc| {
        for (j, slot) in out.chunks_mut(chunk).enumerate() {
            sc.spawn(move || {
                for (k, r) in slot.iter_mut().enumerate() {
                    let mask = (j * chunk + k) as u64;
                    *r = scratch_node(src, &commitment_text(cands, mask), keep, terms0());
                }
            });
        }
    });
    out
}

// ── The rule dependency graph ───────────────────────────────────────

/// Every cycle through an odd `absent` in the saturation rules' dependency
/// graph, over all relations and over `D`. An edge runs from a relation a plan
/// reads to each it writes, negative when the read is at odd `absent` depth;
/// a rule with an activator adds a positive edge from its activator relation.
fn strata(kb: &Kb, terms: &Terms, plans: &[Arc<Plan>], dynamic: &BTreeSet<String>) -> Json {
    let program = kb.program();
    let mut edges: BTreeSet<(String, String, bool, String)> = BTreeSet::new();
    let mut nodes: BTreeSet<String> = BTreeSet::new();
    let mut ruled_without_plan: BTreeSet<String> = BTreeSet::new();
    let sat_plans: Vec<&Plan> = plans
        .iter()
        .filter(|p| program.rules.contains(p.rule))
        .map(|p| &**p)
        .collect();
    for p in &sat_plans {
        let rule = terms.sym(p.rule).to_string();
        for w in monotone::writes_of(p, terms) {
            nodes.insert(w.label(terms));
        }
        for (r, _) in monotone::reads_of(p, terms) {
            nodes.insert(r.label(terms));
        }
        let _ = rule;
    }
    let negatives: Vec<String> = nodes
        .iter()
        .filter(|l| l.starts_with("(not ") && *l != "(not ?)")
        .cloned()
        .collect();
    for p in &sat_plans {
        let rule = terms.sym(p.rule).to_string();
        let writes: Vec<String> = monotone::writes_of(p, terms)
            .into_iter()
            .map(|w| w.label(terms))
            .collect();
        for (r, depth) in monotone::reads_of(p, terms) {
            let from = r.label(terms);
            let froms = if from == "(not ?)" {
                negatives.clone()
            } else {
                vec![from]
            };
            for f in froms {
                for w in &writes {
                    edges.insert((f.clone(), w.clone(), depth % 2 == 1, rule.clone()));
                }
            }
        }
    }
    for rule in program.rules.values() {
        if rule.params.is_empty() {
            continue;
        }
        let act = terms.sym(rule.name).to_string();
        let mine: Vec<&&Plan> = sat_plans.iter().filter(|p| p.rule == rule.name).collect();
        if mine.is_empty() {
            ruled_without_plan.insert(act.clone());
        }
        for p in mine {
            for w in monotone::writes_of(p, terms) {
                edges.insert((act.clone(), w.label(terms), false, act.clone()));
            }
        }
    }
    let over = |keep: &dyn Fn(&str) -> bool| -> Vec<Json> {
        let es: Vec<&(String, String, bool, String)> = edges
            .iter()
            .filter(|(a, b, _, _)| keep(a) && keep(b))
            .collect();
        cycles(&es)
    };
    let all = over(&|_| true);
    let in_d = over(&|l: &str| dynamic.contains(l));
    json!({
        "stratified": all.is_empty(),
        "stratified_over_d": in_d.is_empty(),
        "cycles": all,
        "cycles_over_d": in_d,
        "activators_without_plan": ruled_without_plan,
    })
}

/// The strongly connected components holding a negative edge, each with its
/// negative edges and whether some cycle in it has an odd number of them.
fn cycles(edges: &[&(String, String, bool, String)]) -> Vec<Json> {
    let mut ids: BTreeMap<&str, usize> = BTreeMap::new();
    for (a, b, _, _) in edges {
        let k = ids.len();
        ids.entry(a.as_str()).or_insert(k);
        let k = ids.len();
        ids.entry(b.as_str()).or_insert(k);
    }
    let names: Vec<&str> = {
        let mut v = vec![""; ids.len()];
        for (s, &i) in &ids {
            v[i] = s;
        }
        v
    };
    let n = ids.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (a, b, _, _) in edges {
        adj[ids[a.as_str()]].push(ids[b.as_str()]);
    }
    let (comp, ncomp) = scc(&adj);
    let mut out = Vec::new();
    for c in 0..ncomp {
        let inside: Vec<&&(String, String, bool, String)> = edges
            .iter()
            .filter(|(a, b, _, _)| comp[ids[a.as_str()]] == c && comp[ids[b.as_str()]] == c)
            .collect();
        if !inside.iter().any(|e| e.2) {
            continue;
        }
        // An odd cycle exists iff no parity labelling is consistent.
        let members: Vec<usize> = (0..n).filter(|&v| comp[v] == c).collect();
        let mut parity: BTreeMap<usize, bool> = BTreeMap::new();
        parity.insert(members[0], false);
        let mut odd = false;
        let mut changed = true;
        while changed {
            changed = false;
            for (a, b, neg, _) in inside.iter().map(|e| &***e) {
                let (a, b) = (ids[a.as_str()], ids[b.as_str()]);
                if let Some(&pa) = parity.get(&a) {
                    let want = pa ^ neg;
                    match parity.get(&b) {
                        None => {
                            parity.insert(b, want);
                            changed = true;
                        }
                        Some(&pb) if pb != want => odd = true,
                        _ => {}
                    }
                }
            }
        }
        let mut rels: Vec<&str> = members.iter().map(|&v| names[v]).collect();
        rels.sort();
        let negs: BTreeSet<String> = inside
            .iter()
            .filter(|e| e.2)
            .map(|(a, b, _, r)| format!("{r}: {a} →¬ {b}"))
            .collect();
        out.push(json!({ "relations": rels, "negative": negs, "odd": odd }));
    }
    out
}

/// Tarjan's strongly connected components: each node's component, and how
/// many there are.
fn scc(adj: &[Vec<usize>]) -> (Vec<usize>, usize) {
    struct T<'a> {
        adj: &'a [Vec<usize>],
        index: Vec<usize>,
        low: Vec<usize>,
        on: Vec<bool>,
        stack: Vec<usize>,
        comp: Vec<usize>,
        next: usize,
        ncomp: usize,
    }
    impl T<'_> {
        fn visit(&mut self, v: usize) {
            self.index[v] = self.next;
            self.low[v] = self.next;
            self.next += 1;
            self.stack.push(v);
            self.on[v] = true;
            for k in 0..self.adj[v].len() {
                let w = self.adj[v][k];
                if self.index[w] == usize::MAX {
                    self.visit(w);
                    self.low[v] = self.low[v].min(self.low[w]);
                } else if self.on[w] {
                    self.low[v] = self.low[v].min(self.index[w]);
                }
            }
            if self.low[v] == self.index[v] {
                loop {
                    let w = self.stack.pop().expect("on the stack");
                    self.on[w] = false;
                    self.comp[w] = self.ncomp;
                    if w == v {
                        break;
                    }
                }
                self.ncomp += 1;
            }
        }
    }
    let n = adj.len();
    let mut t = T {
        adj,
        index: vec![usize::MAX; n],
        low: vec![0; n],
        on: vec![false; n],
        stack: Vec::new(),
        comp: vec![usize::MAX; n],
        next: 0,
        ncomp: 0,
    };
    for v in 0..n {
        if t.index[v] == usize::MAX {
            t.visit(v);
        }
    }
    (t.comp, t.ncomp)
}

// ── Readings per option ─────────────────────────────────────────────

/// R and its lines under one status vector; `reads` gives the states.
fn reading(n: usize, status: &[u8], reads: &[Read], uni: &Universe) -> Json {
    let t = Topology::of(n, status);
    let in_r: Vec<u64> = (0..1u64 << n)
        .filter(|&m| status[m as usize] == 0)
        .collect();
    let states: BTreeSet<u64> = in_r.iter().map(|&m| reads[m as usize].digest).collect();
    let state_of = |m: u64| -> Vec<String> {
        reads[m as usize]
            .facts
            .iter()
            .filter(|f| !uni.base.contains(*f))
            .cloned()
            .collect()
    };
    json!({
        "R": t.size_r,
        "k_s": states.len(),
        "minimal": t.minimal.iter().map(|&m| name(&uni.cands, m)).collect::<Vec<_>>(),
        "maximal": t.maximal.iter().map(|&m| name(&uni.cands, m)).collect::<Vec<_>>(),
        "maximal_states": t.maximal.iter().map(|&m| state_of(m)).collect::<Vec<_>>(),
        "holes": t.holes.len(),
    })
}

/// Stale nodes counted as dead: option (C) over a route.
fn guarded(reads: &[Read]) -> Vec<u8> {
    reads
        .iter()
        .map(|r| r.status() | if r.stale.is_empty() { 0 } else { DEAD })
        .collect()
}

// ── Comparing the two routes on one program ─────────────────────────

/// The `-y` check on every node of two facts or more: does a `(k−1)`-subset
/// parent path reach the direct path's state?
fn paths(src: &Source) -> Json {
    let Some((mut kb, mut terms, ast)) = src.load("", Terms::new()) else {
        return Json::Null;
    };
    let Some((root, ())) = census::root_of(&mut kb, &mut terms, &ast, |_, _| Some(())) else {
        return Json::Null;
    };
    let mut cands = root.proposed.clone();
    cands.sort_by_key(|&f| sexpr(&terms, f));
    cands.dedup();
    let texts: Vec<String> = cands.iter().map(|&f| sexpr(&terms, f)).collect();
    let n = cands.len();
    if n > 10 {
        return Json::Null;
    }
    let memo = SharedMemo::default();
    let plans = root.plans.clone();
    let (mut checked, mut errors) = (0, 0);
    let mut differ: Vec<String> = Vec::new();
    // (parent's facts, the fact added last, the line it explains)
    let mut rebuild: Vec<(Vec<FactId>, FactId, usize)> = Vec::new();
    for mask in 0..1u64 << n {
        if mask.count_ones() < 2 {
            continue;
        }
        let facts: Vec<FactId> = bits(mask, n).map(|b| cands[b]).collect();
        checked += 1;
        match sanity::check_commutativity(
            &mut kb,
            &mut terms,
            &ast,
            &mut Events::off(),
            &memo,
            &facts,
        ) {
            Ok(Some(e)) => {
                let render = |ids: &[FactId]| -> BTreeSet<String> {
                    ids.iter().map(|&f| sexpr(&terms, f)).collect()
                };
                let direct = render(&e.direct_state_key);
                for (parent, key) in &e.parent_state_keys {
                    let via = render(key);
                    if let Some(&last) = facts.iter().find(|f| !parent.contains(f)) {
                        rebuild.push((parent.clone(), last, differ.len()));
                    }
                    let missing: Vec<String> = facts
                        .iter()
                        .filter(|f| !parent.contains(f))
                        .map(|&f| sexpr(&terms, f))
                        .collect();
                    differ.push(format!(
                        "{} via {} + {}: only direct {:?}, only the path {:?}",
                        name(&texts, mask),
                        name(
                            &texts,
                            mask & !bits(mask, n)
                                .filter(|&b| !parent.contains(&cands[b]))
                                .fold(0, |a, b| a | 1 << b)
                        ),
                        missing.join(" "),
                        direct.difference(&via).collect::<Vec<_>>(),
                        via.difference(&direct).collect::<Vec<_>>(),
                    ));
                }
            }
            Ok(None) => {}
            Err(_) => errors += 1,
        }
    }
    // Each differing path's state, rebuilt as `-y` builds it (the parent's
    // fixpoint, the last fact written, a full saturation) and read for
    // staleness.
    let mut stale_on_path: Vec<Vec<String>> = vec![Vec::new(); differ.len()];
    if !rebuild.is_empty()
        && let Ok(mut nodes) = region::Nodes::new(&mut kb, &mut terms, &ast, &[])
    {
        for (parent, last, line) in rebuild {
            let got = nodes.with_fixpoints(std::slice::from_ref(&parent), |kbs, terms| {
                let kb = &mut kbs[0];
                let (rel, args) = terms.facts.get(last);
                let args = args.to_vec();
                let prov = terms.provs.push(Prov::from_hypothesis(0, None));
                let _ = kb.add_and_index_fact(terms, rel, &args, Some(prov));
                let mut events = Events::off();
                let mut s = Session {
                    kb,
                    terms,
                    ast: &ast,
                    events: &mut events,
                    memo: SharedMemo::default(),
                };
                if let Ok(mut sat) = Saturator::new(&mut s) {
                    let _ = sat.saturate(&mut s, None, &mut |_| {});
                }
                stale_facts(s.kb, s.terms, &ast, &plans)
                    .0
                    .into_iter()
                    .map(|x| x.1)
                    .collect::<Vec<_>>()
            });
            if let Ok(v) = got {
                stale_on_path[line] = v;
            }
        }
    }

    let differ: Vec<Json> = differ
        .into_iter()
        .zip(stale_on_path)
        .map(|(d, st)| json!({ "node": d, "stale_on_the_path": st }))
        .collect();
    json!({ "checked": checked, "differ": differ, "errors": errors })
}

fn compare(src: &Source, jobs: usize, keep: bool, with_paths: bool) -> Json {
    let mut row = json!({ "path": src.name });
    let Some(fork) = fork_route(src, Terms::new(), keep) else {
        row["loads"] = json!(false);
        return row;
    };
    let uni = &fork.uni;
    let n = uni.cands.len();
    row["n"] = json!(n);
    row["monotone"] = json!(uni.monotone);
    row["sites"] = json!(uni.sites);
    row["strata"] = uni.strata.clone();
    if n > LIMIT {
        row["past_limit"] = json!(true);
        return row;
    }
    // The oracle, on its own load: the fork route read here is its node.
    let oracle: Option<Vec<u8>> = src.load("", Terms::new()).and_then(|(mut k, mut t, a)| {
        let opts = region::Options {
            jobs,
            ..region::Options::default()
        };
        match region::compute(&mut k, &mut t, &a, &opts, &mut |_| {}) {
            Ok(Outcome::Region(r)) => Some(r.status.clone()),
            _ => None,
        }
    });
    let fstat: Vec<u8> = fork.reads.iter().map(Read::status).collect();
    row["fork_is_the_oracle"] = json!(oracle.as_ref() == Some(&fstat));

    let scratch = scratch_route(src, &uni.cands, keep, jobs, &Terms::new);
    if scratch.iter().any(Option::is_none) {
        row["scratch_failed"] = json!(scratch.iter().filter(|r| r.is_none()).count());
        return row;
    }
    let scratch: Vec<Read> = scratch.into_iter().map(Option::unwrap).collect();
    let sstat: Vec<u8> = scratch.iter().map(Read::status).collect();

    let mut status_agree = 0;
    let mut state_agree = 0;
    let mut differ: Vec<Json> = Vec::new();
    for mask in 0..1u64 << n {
        let (f, s) = (&fork.reads[mask as usize], &scratch[mask as usize]);
        let st = f.status() == s.status();
        let fa = f.digest == s.digest;
        status_agree += st as usize;
        state_agree += fa as usize;
        if st && fa {
            continue;
        }
        let mut d = json!({
            "node": name(&uni.cands, mask),
            "fork": class(f.status()),
            "scratch": class(s.status()),
            "fork_refuted_by": f.refuted_by,
            "scratch_refuted_by": s.refuted_by,
        });
        if keep {
            let fs: BTreeSet<&String> = f.facts.iter().collect();
            let ss: BTreeSet<&String> = s.facts.iter().collect();
            let tag = |x: &String, r: &Read| -> String {
                match r.rule.get(x) {
                    Some(rule) if uni.site_rules.contains(rule) => {
                        format!("{x} ← {rule} [a site's rule]")
                    }
                    Some(rule) => format!("{x} ← {rule}"),
                    None => format!("{x} (given)"),
                }
            };
            d["only_fork"] = json!(fs.difference(&ss).map(|x| tag(x, f)).collect::<Vec<_>>());
            d["only_scratch"] = json!(ss.difference(&fs).map(|x| tag(x, s)).collect::<Vec<_>>());
        }
        differ.push(d);
    }
    row["nodes"] = json!(1usize << n);
    if keep {
        row["candidates"] = json!(uni.cands);
        row["fork_status"] = json!(fstat.iter().map(|&x| class(x)).collect::<Vec<_>>());
        row["scratch_status"] = json!(sstat.iter().map(|&x| class(x)).collect::<Vec<_>>());
    }
    row["status_agree"] = json!(status_agree);
    row["state_agree"] = json!(state_agree);
    row["differ"] = json!(differ);

    let stale_rows = |reads: &[Read]| -> Json {
        let nodes: Vec<Json> = (0..1u64 << n)
            .filter(|&m| !reads[m as usize].stale.is_empty())
            .map(|m| {
                let r = &reads[m as usize];
                json!({
                    "node": name(&uni.cands, m),
                    "status": class(r.status()),
                    "stale": r.stale,
                    "resting": r.resting,
                    "dead_on_stale": r.dead_on_stale,
                })
            })
            .collect();
        json!({
            "nodes": nodes.len(),
            "alive": (0..1usize << n).filter(|&m| !reads[m].stale.is_empty() && !reads[m].dead).count(),
            "in_r": (0..1usize << n).filter(|&m| !reads[m].stale.is_empty() && reads[m].status() == 0).count(),
            "dead_on_stale": (0..1usize << n).filter(|&m| reads[m].dead_on_stale).count(),
            "facts": reads.iter().map(|r| r.stale.len()).sum::<usize>(),
            "inexact": reads.iter().map(|r| r.inexact).sum::<usize>(),
            "list": if keep { json!(nodes) } else { Json::Null },
        })
    };
    row["stale_fork"] = stale_rows(&fork.reads);
    row["stale_scratch"] = stale_rows(&scratch);

    if keep {
        row["options"] = json!({
            "A_fork": reading(n, &fstat, &fork.reads, uni),
            "B_scratch": reading(n, &sstat, &scratch, uni),
            "C_over_A": reading(n, &guarded(&fork.reads), &fork.reads, uni),
            "C_over_B": reading(n, &guarded(&scratch), &scratch, uni),
        });
    }
    if with_paths {
        row["paths"] = paths(src);
    }
    row
}

// ── Invariance ──────────────────────────────────────────────────────

/// A route's answer, keyed so two runs over different interners compare:
/// commitment text → (status, digest of the rendered state).
type Answer = BTreeMap<String, (u8, u64)>;

fn answer_of(cands: &[String], reads: &[Read]) -> Answer {
    (0..reads.len() as u64)
        .map(|m| {
            let r = &reads[m as usize];
            (name(cands, m), (r.status(), r.digest))
        })
        .collect()
}

/// `None` when the program does not load, saturate or fits no cube.
fn both(src: &Source, jobs: usize, terms0: TermsFn<'_>) -> Option<(Answer, Answer, Terms)> {
    let fork = fork_route(src, terms0(), false)?;
    if fork.uni.cands.len() > LIMIT {
        return None;
    }
    let f = answer_of(&fork.uni.cands, &fork.reads);
    let s: Vec<Read> = scratch_route(src, &fork.uni.cands, false, jobs, terms0)
        .into_iter()
        .collect::<Option<Vec<_>>>()?;
    Some((f, answer_of(&fork.uni.cands, &s), fork.terms))
}

/// How two answers differ: `same`, `states` (the node statuses agree, a
/// state does not), `R` (a status moved), `universe` (the candidates moved).
fn moved(a: &Answer, b: &Answer) -> &'static str {
    if a.keys().ne(b.keys()) {
        return "universe";
    }
    if a.iter().zip(b).any(|((_, x), (_, y))| x.0 != y.0) {
        return "R";
    }
    if a.iter().zip(b).any(|((_, x), (_, y))| x.1 != y.1) {
        return "states";
    }
    "same"
}

/// A `Terms` whose id space is a permutation of `after`'s —
/// `ein-render/tests/id_order_invariance.rs`'s `permuted`, copied because a
/// test's helper cannot be imported.
fn permuted(after: &Terms, seed: i64) -> Terms {
    let mut rng = Mt19937::seeded(seed);
    let mut names: Vec<String> = (0..after.syms.len())
        .map(|i| after.syms.text(Symbol(i as u32)).to_string())
        .collect();
    rng.shuffle(&mut names);
    let mut ints: Vec<String> = (0..after.ints.len())
        .map(|i| after.ints.text(IntId(i as u32)).to_string())
        .collect();
    rng.shuffle(&mut ints);
    let mut terms = Terms::new();
    let junk = terms.intern_text("@id-order-probe").expect("room");
    for k in 0..7 {
        let arg = terms.value_text(&format!("@probe-{k}")).expect("room");
        terms.intern_fact(junk, &[arg]).expect("room");
    }
    for name in &names {
        terms.intern_text(name).expect("room");
    }
    for text in &ints {
        terms.value_int(text).expect("room");
    }
    let n = after.facts.len();
    let mut depth = vec![0usize; n];
    for i in 0..n {
        let (_, args) = after.facts.get(FactId(i as u32));
        depth[i] = args
            .iter()
            .filter_map(|v| v.as_fact())
            .map(|f| depth[f.0 as usize] + 1)
            .max()
            .unwrap_or(0);
    }
    let mut by_depth: Vec<Vec<u32>> =
        vec![Vec::new(); depth.iter().copied().max().unwrap_or(0) + 1];
    for (i, d) in depth.iter().enumerate() {
        by_depth[*d].push(i as u32);
    }
    let mut moved_to: FxHashMap<u32, FactId> = FxHashMap::default();
    for bucket in &mut by_depth {
        rng.shuffle(bucket);
        for &i in bucket.iter() {
            let (rel, args) = after.facts.get(FactId(i));
            let rel_text = after.syms.text(rel).to_string();
            let args: Vec<Value> = args
                .iter()
                .map(|&v| remap(after, &mut terms, &moved_to, v))
                .collect();
            let rel = terms.intern_text(&rel_text).expect("interned");
            let id = terms.intern_fact(rel, &args).expect("room");
            moved_to.insert(i, id);
        }
    }
    terms
}

fn remap(after: &Terms, terms: &mut Terms, moved: &FxHashMap<u32, FactId>, v: Value) -> Value {
    match v.tag() {
        Tag::Sym => {
            let text = after.syms.text(Symbol(v.payload())).to_string();
            terms.value_text(&text).expect("room")
        }
        Tag::Int => {
            let text = after.ints.text(IntId(v.payload())).to_string();
            terms.value_int(&text).expect("room")
        }
        Tag::Fact => Value::fact(moved[&v.payload()]),
    }
}

/// The spans of the top-level forms, comments and strings skipped.
fn top_forms(text: &str) -> Vec<(usize, usize)> {
    let b = text.as_bytes();
    let (mut i, mut depth, mut start) = (0, 0usize, 0);
    let mut out = Vec::new();
    while i < b.len() {
        match b[i] {
            b';' => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            b'#' if b.get(i + 1) == Some(&b'|') => {
                i = text[i + 2..].find("|#").map_or(b.len(), |j| i + 2 + j + 2);
                continue;
            }
            b'"' => {
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    i += if b[i] == b'\\' { 2 } else { 1 };
                }
            }
            b'(' => {
                if depth == 0 {
                    start = i;
                }
                depth += 1;
            }
            b')' => {
                depth -= 1;
                if depth == 0 {
                    out.push((start, i + 1));
                }
            }
            _ => {}
        }
        i += 1;
    }
    out
}

fn priority_of(form: &str) -> i64 {
    let mut words = form.split_whitespace();
    while let Some(w) = words.next() {
        if w == ":priority"
            && let Some(Ok(p)) = words.next().map(|v| v.trim_end_matches(')').parse())
        {
            return p;
        }
    }
    DEFAULT_PRIORITY
}

/// Every reordering of the program's equal-priority `(rule …)` forms, the
/// identity first; all of them while there are at most 24, else the reverse
/// and eight seeded shuffles.
fn reorders(text: &str) -> Vec<String> {
    let forms = top_forms(text);
    let rules: Vec<(usize, usize)> = forms
        .into_iter()
        .filter(|&(a, z)| text[a..z].starts_with("(rule "))
        .collect();
    let mut groups: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    for (k, &(a, z)) in rules.iter().enumerate() {
        groups.entry(priority_of(&text[a..z])).or_default().push(k);
    }
    let groups: Vec<Vec<usize>> = groups.into_values().filter(|g| g.len() > 1).collect();
    let count: usize = groups
        .iter()
        .map(|g| (1..=g.len()).product::<usize>())
        .product();
    let mut orders: Vec<Vec<usize>> = Vec::new(); // a permutation of rule slots
    let ident: Vec<usize> = (0..rules.len()).collect();
    if groups.is_empty() {
        return vec![text.to_string()];
    }
    if count <= 24 {
        let mut acc = vec![ident.clone()];
        for g in &groups {
            let mut next = Vec::new();
            for base in &acc {
                for perm in permutations(g) {
                    let mut o = base.clone();
                    for (slot, &k) in g.iter().zip(&perm) {
                        o[*slot] = k;
                    }
                    next.push(o);
                }
            }
            acc = next;
        }
        orders = acc;
    } else {
        orders.push(ident.clone());
        let mut rev = ident.clone();
        for g in &groups {
            for (slot, &k) in g.iter().zip(g.iter().rev()) {
                rev[*slot] = k;
            }
        }
        orders.push(rev);
        let mut rng = Mt19937::seeded(2026);
        for _ in 0..8 {
            let mut o = ident.clone();
            for g in &groups {
                let mut p = g.clone();
                rng.shuffle(&mut p);
                for (slot, &k) in g.iter().zip(&p) {
                    o[*slot] = k;
                }
            }
            orders.push(o);
        }
    }
    orders
        .into_iter()
        .map(|o| {
            let mut s = String::with_capacity(text.len());
            let mut at = 0;
            for (slot, &(a, z)) in rules.iter().enumerate() {
                s.push_str(&text[at..a]);
                let (a2, z2) = rules[o[slot]];
                s.push_str(&text[a2..z2]);
                at = z;
            }
            s.push_str(&text[at..]);
            s
        })
        .collect()
}

fn permutations(v: &[usize]) -> Vec<Vec<usize>> {
    if v.len() <= 1 {
        return vec![v.to_vec()];
    }
    let mut out = Vec::new();
    for i in 0..v.len() {
        let mut rest = v.to_vec();
        let x = rest.remove(i);
        for mut p in permutations(&rest) {
            p.insert(0, x);
            out.push(p);
        }
    }
    out
}

/// The seven `(config …)` flags and their defaults (`configuration.md` § 2.1).
const FLAGS: &[(&str, bool)] = &[
    ("enable-pre-branch-lookahead", true),
    ("enable-lookahead-kill-cache", true),
    ("warn-derived-naf", false),
    ("lattice-sanity-check", false),
    ("enable-path-nogoods", true),
    ("record-alternative-justifications", true),
    ("enable-fail-fast-fork", true),
];

/// The program with one flag flipped from its default — `None` where the
/// program already sets it.
fn flipped(text: &str, flag: &str, default: bool) -> Option<String> {
    let kw = format!(":{flag} ");
    let forms = top_forms(text);
    let cfg = forms
        .iter()
        .find(|&&(a, _)| text[a..].starts_with("(config"));
    let value = if default { "false" } else { "true" };
    match cfg {
        Some(&(a, z)) if text[a..z].contains(&kw) => None,
        Some(&(a, _)) => {
            let at = a + "(config".len();
            Some(format!("{} :{flag} {value}{}", &text[..at], &text[at..]))
        }
        None => Some(format!("{text}\n(config :{flag} {value})\n")),
    }
}

fn invariance(src: &Source, seeds: usize, jobs: usize) -> Json {
    let mut row = json!({ "path": src.name });
    let Some((f0, s0, after)) = both(src, jobs, &Terms::new) else {
        row["skipped"] = json!(true);
        return row;
    };
    let lever = |runs: Vec<Option<(Answer, Answer)>>| -> Json {
        let mut fm = BTreeMap::<&str, usize>::new();
        let mut sm = BTreeMap::<&str, usize>::new();
        let mut failed = 0;
        for r in &runs {
            match r {
                Some((f, s)) => {
                    *fm.entry(moved(&f0, f)).or_default() += 1;
                    *sm.entry(moved(&s0, s)).or_default() += 1;
                }
                None => failed += 1,
            }
        }
        json!({ "runs": runs.len(), "fork": fm, "scratch": sm, "failed": failed })
    };

    // --jobs: the oracle's own fan-out for fork, the node fan-out for scratch.
    let oracle_at = |j: usize| -> Option<(Vec<u8>, Vec<u64>)> {
        let (mut k, mut t, a) = src.load("", Terms::new())?;
        let opts = region::Options {
            jobs: j,
            ..region::Options::default()
        };
        match region::compute(&mut k, &mut t, &a, &opts, &mut |_| {}) {
            Ok(Outcome::Region(r)) => {
                Some((r.status.clone(), r.nodes.iter().map(|x| x.class).collect()))
            }
            _ => None,
        }
    };
    let cands: Vec<String> = f0.keys().cloned().collect();
    let _ = cands;
    let o1 = oracle_at(1);
    let o16 = oracle_at(16);
    let fork_univ = fork_route(src, Terms::new(), false).map(|f| f.uni.cands);
    let scratch_at = |j: usize| -> Option<Answer> {
        let c = fork_univ.as_ref()?;
        let rs: Vec<Read> = scratch_route(src, c, false, j, &Terms::new)
            .into_iter()
            .collect::<Option<Vec<_>>>()?;
        Some(answer_of(c, &rs))
    };
    row["jobs"] = json!({
        "fork_oracle_1_vs_16": if o1.is_some() && o1 == o16 { "same" } else { "differ" },
        "scratch_1_vs_16": match (scratch_at(1), scratch_at(16)) {
            (Some(a), Some(b)) => moved(&a, &b),
            _ => "failed",
        },
    });

    // Permuted interners.
    let runs: Vec<Option<(Answer, Answer)>> = (0..seeds as i64)
        .map(|i| {
            let seed = 1 + i * 7919;
            let make = || permuted(&after, seed);
            both(src, jobs, &make).map(|(f, s, _)| (f, s))
        })
        .collect();
    row["interners"] = lever(runs);

    // Equal-priority rule orders, the identity excluded.
    let variants = reorders(&src.text);
    let runs: Vec<Option<(Answer, Answer)>> = variants
        .iter()
        .skip(1)
        .map(|t| both(&src.with_text(t.clone()), jobs, &Terms::new).map(|(f, s, _)| (f, s)))
        .collect();
    row["rule_orders"] = lever(runs);
    // Which order moved what, for the programs where one did.
    let mut detail: Vec<Json> = Vec::new();
    for t in variants.iter().skip(1) {
        if let Some((f, s, _)) = both(&src.with_text(t.clone()), jobs, &Terms::new) {
            let (mf, ms) = (moved(&f0, &f), moved(&s0, &s));
            if mf != "same" || ms != "same" {
                let order: Vec<String> = top_forms(t)
                    .into_iter()
                    .filter(|&(a, _)| t[a..].starts_with("(rule "))
                    .map(|(a, z)| t[a..z].split_whitespace().nth(1).unwrap_or("?").to_string())
                    .collect();
                let r_of = |ans: &Answer| -> Vec<String> {
                    ans.iter()
                        .filter(|(_, v)| v.0 == 0)
                        .map(|(k, _)| k.clone())
                        .collect()
                };
                detail.push(json!({
                    "order": order,
                    "fork": mf, "scratch": ms,
                    "fork_R": r_of(&f), "scratch_R": r_of(&s),
                    "universe": f.keys().cloned().collect::<Vec<_>>(),
                }));
            }
        }
    }
    row["rule_orders"]["moved"] = json!(detail);
    row["base"] = json!({
        "fork_R": f0.iter().filter(|(_, v)| v.0 == 0).map(|(k, _)| k.clone()).collect::<Vec<_>>(),
        "scratch_R": s0.iter().filter(|(_, v)| v.0 == 0).map(|(k, _)| k.clone()).collect::<Vec<_>>(),
    });

    // Every flag flipped.
    let runs: Vec<Option<(Answer, Answer)>> = FLAGS
        .iter()
        .filter_map(|(flag, d)| flipped(&src.text, flag, *d))
        .map(|t| both(&src.with_text(t), jobs, &Terms::new).map(|(f, s, _)| (f, s)))
        .collect();
    row["flags"] = lever(runs);
    row
}

// ── Cost ────────────────────────────────────────────────────────────

/// Per-node wall and facts beyond the start, one thread, each route read to
/// its two bits only. Cubes past `sample` nodes are sampled evenly.
fn cost(src: &Source, sample: usize) -> Json {
    let mut row = json!({ "path": src.name });
    let Some((mut kb, mut terms, ast)) = src.load("", Terms::new()) else {
        return row;
    };
    let Some((root, ())) = census::root_of(&mut kb, &mut terms, &ast, |_, _| Some(())) else {
        return row;
    };
    let mut cands = root.proposed.clone();
    cands.sort_by_key(|&f| sexpr(&terms, f));
    cands.dedup();
    let texts: Vec<String> = cands.iter().map(|&f| sexpr(&terms, f)).collect();
    let n = cands.len();
    if n > LIMIT {
        return row;
    }
    let size = 1u64 << n;
    let masks: Vec<u64> = if size as usize <= sample {
        (0..size).collect()
    } else {
        (0..sample as u64)
            .map(|i| ((size - 1) as u128 * i as u128 / (sample as u64 - 1) as u128) as u64)
            .collect()
    };
    let mut fork_wall: Vec<Duration> = Vec::new();
    let mut fork_beyond: Vec<usize> = Vec::new();
    let root_derived;
    {
        let base = {
            let (k, _, _) = src.load("", Terms::new()).expect("loads twice");
            k.n_facts()
        };
        let mut nodes = region::Nodes::new(&mut kb, &mut terms, &ast, &[]).expect("prepares");
        let (root_n, held) = nodes
            .with_fixpoint(&[], |kb, _| {
                let held: u64 = (0..n)
                    .filter(|&b| kb.contains(cands[b]))
                    .fold(0, |a, b| a | 1 << b);
                (kb.n_facts(), held)
            })
            .expect("root");
        root_derived = root_n - base;
        for &mask in &masks {
            let facts: Vec<FactId> = bits(mask, n).map(|b| cands[b]).collect();
            let t = Instant::now();
            let beyond = nodes
                .with_fixpoint(&facts, |kb, terms| {
                    let _ = contradiction::detect(kb, terms);
                    let _ = obligations::tally(
                        kb,
                        terms,
                        &ast,
                        &SharedMemo::default(),
                        &mut Events::off(),
                    );
                    // What the node's own saturation derived: beyond root and
                    // beyond the commitment written into it.
                    kb.n_facts() - root_n - (mask & !held).count_ones() as usize
                })
                .expect("a node");
            fork_wall.push(t.elapsed());
            fork_beyond.push(beyond);
        }
    }
    let mut scratch_wall: Vec<Duration> = Vec::new();
    let mut scratch_sat: Vec<Duration> = Vec::new();
    let mut scratch_derived: Vec<usize> = Vec::new();
    for &mask in &masks {
        let t = Instant::now();
        let Some((mut kb, mut terms, ast)) = src.load(&commitment_text(&texts, mask), Terms::new())
        else {
            continue;
        };
        let start = kb.n_facts();
        let t_sat = Instant::now();
        let memo = SharedMemo::default();
        let mut events = Events::off();
        {
            let mut s = Session {
                kb: &mut kb,
                terms: &mut terms,
                ast: &ast,
                events: &mut events,
                memo: memo.clone(),
            };
            let Ok(mut sat) = Saturator::new(&mut s) else {
                continue;
            };
            let _ = sat.saturate(&mut s, None, &mut |_| {});
        }
        let _ = contradiction::detect(&kb, &terms);
        let _ = obligations::tally(&kb, &mut terms, &ast, &memo, &mut Events::off());
        scratch_wall.push(t.elapsed());
        scratch_sat.push(t_sat.elapsed());
        scratch_derived.push(kb.n_facts() - start);
    }
    let us = |v: &[Duration]| -> (f64, f64) {
        let mut s: Vec<f64> = v.iter().map(|d| d.as_secs_f64() * 1e6).collect();
        s.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
        let mean = s.iter().sum::<f64>() / s.len().max(1) as f64;
        (mean, s.get(s.len() / 2).copied().unwrap_or(0.0))
    };
    let mean = |v: &[usize]| v.iter().sum::<usize>() as f64 / v.len().max(1) as f64;
    let (fm, fmed) = us(&fork_wall);
    let (sm, smed) = us(&scratch_wall);
    let (ssat, _) = us(&scratch_sat);
    row["n"] = json!(n);
    row["nodes_timed"] = json!(masks.len());
    row["root_derived"] = json!(root_derived);
    row["fork_us_mean"] = json!((fm * 10.0).round() / 10.0);
    row["fork_us_median"] = json!((fmed * 10.0).round() / 10.0);
    row["scratch_us_mean"] = json!((sm * 10.0).round() / 10.0);
    row["scratch_us_median"] = json!((smed * 10.0).round() / 10.0);
    row["scratch_saturation_us_mean"] = json!((ssat * 10.0).round() / 10.0);
    row["ratio_mean"] = json!(((sm / fm.max(1e-9)) * 10.0).round() / 10.0);
    row["ratio_saturation"] = json!(((ssat / fm.max(1e-9)) * 10.0).round() / 10.0);
    row["fork_facts_derived_mean"] = json!((mean(&fork_beyond) * 10.0).round() / 10.0);
    row["scratch_facts_derived_mean"] = json!((mean(&scratch_derived) * 10.0).round() / 10.0);
    row
}

/// The scratch route alone over the whole cube, to n = 20 — for a program
/// past the oracle's limit, whose fork answer is known without the cube
/// (`features/12`: root is dead, so every fork is).
fn scratch_cube_row(src: &Source, jobs: usize) -> Json {
    let Some(fork) = fork_route_with(src, Terms::new(), false, false) else {
        return json!({ "path": src.name, "loads": false });
    };
    let cands = &fork.uni.cands;
    let n = cands.len();
    if n > 20 {
        return json!({ "path": src.name, "n": n, "skipped": "n > 20" });
    }
    let t = Instant::now();
    let reads = scratch_route(src, cands, false, jobs, &Terms::new);
    let mut by: BTreeMap<&str, usize> = BTreeMap::new();
    let mut alive: Vec<String> = Vec::new();
    for (m, r) in reads.iter().enumerate() {
        let c = r.as_ref().map_or("failed", |r| class(r.status()));
        *by.entry(c).or_default() += 1;
        if c != "dead" && alive.len() < 8 {
            alive.push(name(cands, m as u64));
        }
    }
    json!({ "path": src.name, "n": n, "nodes": reads.len(), "scratch": by,
            "first_not_dead": alive, "seconds": (t.elapsed().as_secs_f64() * 10.0).round() / 10.0 })
}

// ── The selection and main ──────────────────────────────────────────

/// Root's static verdict and `n`, read cheaply, for the selection.
fn verdict(src: &Source) -> Option<(bool, usize)> {
    let (mut kb, mut terms, ast) = src.load("", Terms::new())?;
    let (root, ()) = census::root_of(&mut kb, &mut terms, &ast, |_, _| Some(()))?;
    let mut c = root.proposed.clone();
    c.sort_by_key(|&f| sexpr(&terms, f));
    c.dedup();
    Some((root.monotonicity.is_monotone(), c.len()))
}

fn main() {
    let mut as_json = false;
    let mut jobs = std::thread::available_parallelism().map_or(1, |n| n.get().min(16));
    let (mut control, mut strata_only, mut inv, mut costs, mut use_written, mut scratch_cube) =
        (false, false, false, false, false, false);
    let mut seeds = 8usize;
    let mut sample = 4096usize;
    let mut max_n = LIMIT;
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--json" => as_json = true,
            "--control" => control = true,
            "--strata" => strata_only = true,
            "--invariance" => inv = true,
            "--cost" => costs = true,
            "--written" => use_written = true,
            "--scratch-cube" => scratch_cube = true,
            "--jobs" => jobs = args.next().and_then(|v| v.parse().ok()).expect("--jobs J"),
            "--seeds" => seeds = args.next().and_then(|v| v.parse().ok()).expect("--seeds S"),
            "--sample" => {
                sample = args
                    .next()
                    .and_then(|v| v.parse().ok())
                    .expect("--sample K")
            }
            "--max-n" => max_n = args.next().and_then(|v| v.parse().ok()).expect("--max-n N"),
            _ => {
                let p = PathBuf::from(&a);
                let p = if p.exists() { p } else { repo_root().join(p) };
                files.push(std::fs::canonicalize(&p).unwrap_or(p));
            }
        }
    }
    let t_all = Instant::now();
    let explicit = !files.is_empty() || use_written;
    let mut sources: Vec<Source> = files.iter().filter_map(|p| Source::of(p)).collect();
    if use_written {
        sources.extend(written());
    }
    if !explicit {
        sources = census::selection()
            .iter()
            .filter_map(|p| Source::of(p))
            .collect();
    }

    if scratch_cube {
        for src in &sources {
            println!("{}", scratch_cube_row(src, jobs));
        }
        if !as_json {
            println!("wall {:.1} s", t_all.elapsed().as_secs_f64());
        }
        return;
    }

    if strata_only {
        let (mut loads, mut refused, mut refused_d) = (0, Vec::new(), Vec::new());
        for src in &sources {
            let Some(fork) = fork_route_with(src, Terms::new(), false, false) else {
                continue;
            };
            loads += 1;
            let s = &fork.uni.strata;
            let row = json!({
                "path": src.name,
                "n": fork.uni.cands.len(),
                "monotone": fork.uni.monotone,
                "strata": s,
            });
            if as_json {
                println!("{row}");
            }
            if !s["stratified"].as_bool().unwrap_or(true) {
                refused.push(row.clone());
            }
            if !s["stratified_over_d"].as_bool().unwrap_or(true) {
                refused_d.push(row);
            }
        }
        if !as_json {
            println!("programs that load and saturate: {loads}");
            println!(
                "a cycle through an odd absent, over all relations: {}",
                refused.len()
            );
            for r in &refused {
                println!(
                    "  {}  n {}  {}",
                    r["path"].as_str().unwrap_or("?"),
                    r["n"],
                    if r["monotone"].as_bool() == Some(true) {
                        "monotone"
                    } else {
                        "flagged"
                    }
                );
                for c in r["strata"]["cycles"].as_array().into_iter().flatten() {
                    println!(
                        "    {} {} — {}",
                        if c["odd"].as_bool() == Some(true) {
                            "odd "
                        } else {
                            "even"
                        },
                        c["relations"],
                        c["negative"]
                    );
                }
            }
            println!("… over D, which (D) refuses: {}", refused_d.len());
            for r in &refused_d {
                println!("  {}", r["path"].as_str().unwrap_or("?"));
                for c in r["strata"]["cycles_over_d"]
                    .as_array()
                    .into_iter()
                    .flatten()
                {
                    println!(
                        "    {} {} — {}",
                        if c["odd"].as_bool() == Some(true) {
                            "odd "
                        } else {
                            "even"
                        },
                        c["relations"],
                        c["negative"]
                    );
                }
            }
            println!("wall {:.1} s", t_all.elapsed().as_secs_f64());
        }
        return;
    }

    // Which programs: the explicit ones as given; else the flagged within the
    // limit, or with --control the monotone within the limit.
    let chosen: Vec<Source> = if explicit {
        sources
    } else {
        sources
            .into_iter()
            .filter(|s| match verdict(s) {
                Some((mono, n)) => n <= max_n && mono == control,
                None => false,
            })
            .collect()
    };

    if costs {
        let picked: Vec<&Source> = if control && !explicit {
            // The control: the largest monotone cubes.
            chosen
                .iter()
                .filter(|s| verdict(s).is_some_and(|(_, n)| n >= 12))
                .collect()
        } else {
            chosen.iter().collect()
        };
        for src in picked {
            let row = cost(src, sample);
            if as_json {
                println!("{row}");
            } else {
                println!(
                    "{}  n {}  timed {}  fork {} µs  scratch {} µs (×{}), of which saturation {} µs (×{})  derived: fork {}, scratch {}, root {}",
                    row["path"].as_str().unwrap_or("?"),
                    row["n"],
                    row["nodes_timed"],
                    row["fork_us_mean"],
                    row["scratch_us_mean"],
                    row["ratio_mean"],
                    row["scratch_saturation_us_mean"],
                    row["ratio_saturation"],
                    row["fork_facts_derived_mean"],
                    row["scratch_facts_derived_mean"],
                    row["root_derived"]
                );
            }
        }
        if !as_json {
            println!("wall {:.1} s", t_all.elapsed().as_secs_f64());
        }
        return;
    }

    if inv {
        for src in &chosen {
            let row = invariance(src, seeds, jobs);
            if as_json {
                println!("{row}");
            } else {
                println!("{}", src.name);
                for k in ["jobs", "interners", "rule_orders", "flags"] {
                    let mut v = row[k].clone();
                    if let Some(o) = v.as_object_mut() {
                        o.remove("moved");
                    }
                    println!("  {k:<12} {v}");
                }
                for m in row["rule_orders"]["moved"].as_array().into_iter().flatten() {
                    println!(
                        "    order {} — fork {} R {} · scratch {} R {}",
                        m["order"], m["fork"], m["fork_R"], m["scratch"], m["scratch_R"]
                    );
                }
            }
        }
        if !as_json {
            println!("wall {:.1} s", t_all.elapsed().as_secs_f64());
        }
        return;
    }

    let keep = !control;
    let (mut programs, mut nodes, mut status_differ, mut state_differ) = (0, 0, 0, 0);
    let mut oracle_mismatch: Vec<String> = Vec::new();
    for src in &chosen {
        let row = compare(src, jobs, keep, keep);
        if as_json {
            println!("{row}");
        }
        if row.get("nodes").is_none() {
            if !as_json {
                println!("{}: not compared — {}", src.name, row);
            }
            continue;
        }
        programs += 1;
        let nn = row["nodes"].as_u64().unwrap_or(0);
        nodes += nn;
        status_differ += nn - row["status_agree"].as_u64().unwrap_or(0);
        state_differ += nn - row["state_agree"].as_u64().unwrap_or(0);
        if row["fork_is_the_oracle"].as_bool() != Some(true) {
            oracle_mismatch.push(src.name.clone());
        }
        if !as_json && (keep || nn != row["state_agree"].as_u64().unwrap_or(0)) {
            print_block(&row);
        }
    }
    if !as_json {
        println!(
            "\n{programs} programs, {nodes} nodes: status differs on {status_differ}, state on {state_differ}; \
             fork ≠ oracle on {} program(s){}",
            oracle_mismatch.len(),
            if oracle_mismatch.is_empty() {
                String::new()
            } else {
                format!(": {}", oracle_mismatch.join(", "))
            }
        );
        println!("wall {:.1} s", t_all.elapsed().as_secs_f64());
    }
}

fn print_block(row: &Json) {
    let s = &row["strata"];
    println!(
        "{}  n {} · {} · stratified {} / over D {}",
        row["path"].as_str().unwrap_or("?"),
        row["n"],
        if row["monotone"].as_bool() == Some(true) {
            "monotone".to_string()
        } else {
            format!(
                "flagged ({} sites)",
                row["sites"].as_array().map_or(0, |a| a.len())
            )
        },
        s["stratified"],
        s["stratified_over_d"],
    );
    println!(
        "  nodes {} · status agree {} · state agree {} · fork is the oracle: {}",
        row["nodes"], row["status_agree"], row["state_agree"], row["fork_is_the_oracle"]
    );
    for d in row["differ"].as_array().into_iter().flatten() {
        println!(
            "  differ {}  fork {} {}  scratch {} {}",
            d["node"].as_str().unwrap_or("?"),
            d["fork"].as_str().unwrap_or("?"),
            d["fork_refuted_by"],
            d["scratch"].as_str().unwrap_or("?"),
            d["scratch_refuted_by"]
        );
        if d.get("only_fork").is_some() {
            println!("         only fork    {}", d["only_fork"]);
            println!("         only scratch {}", d["only_scratch"]);
        }
    }
    for (k, label) in [("stale_fork", "fork"), ("stale_scratch", "scratch")] {
        let st = &row[k];
        println!(
            "  stale {label:<7} nodes {} (alive {}, in R {}, dead on stale {}) · facts {} · inexact {}",
            st["nodes"], st["alive"], st["in_r"], st["dead_on_stale"], st["facts"], st["inexact"]
        );
        for x in st["list"].as_array().into_iter().flatten() {
            println!(
                "    {} {} — {}{}",
                x["node"].as_str().unwrap_or("?"),
                x["status"].as_str().unwrap_or("?"),
                x["stale"],
                if x["dead_on_stale"].as_bool() == Some(true) {
                    " · its death rests on it"
                } else {
                    ""
                }
            );
        }
    }
    if let Some(o) = row["options"].as_object() {
        for (k, v) in o {
            println!(
                "  {k:<9} |R| {} k_s {} holes {} · minimal {} · maximal {} · states {}",
                v["R"], v["k_s"], v["holes"], v["minimal"], v["maximal"], v["maximal_states"]
            );
        }
    }
    if !row["paths"].is_null() {
        println!("  paths   {}", row["paths"]);
    }
    for c in s["cycles"].as_array().into_iter().flatten() {
        println!(
            "  cycle   {} {} — {}",
            if c["odd"].as_bool() == Some(true) {
                "odd "
            } else {
                "even"
            },
            c["relations"],
            c["negative"]
        );
    }
}
