//! The node census — M1e2
//! [S1e2.1b.2](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.2_fork_or_scratch.md):
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
//! an odd `absent`, over all relations and over `D` (option (D)'s refusal).
//! Its column of the parent-path check `-y` used to run, the walk's
//! `sanity::check_commutativity`, went with the walk at M1e2 S1e2.1c.5
//! (2026-11-03); `-y` is the detector's spot check now (`read_out.rs`).
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
#[allow(dead_code)]
#[path = "shared/written.rs"]
mod written_rs;

use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use ein_core::{FactId, Kb, NafArg, NafRef, Prov, Symbol, Tag, Terms, Value};
use ein_corpus::repo_root;
use ein_infer::compile::SharedMemo;
use ein_infer::events::{Events, sexpr};
use ein_infer::match_::Matcher;
use ein_infer::monotone;
use ein_infer::mt19937::Mt19937;
use ein_infer::plan::{Disjunct, NafGuard, Plan, Slot, Span, Step};
use ein_infer::region::{self, DEAD, OWING, Outcome, Topology};
use ein_infer::saturator::{Saturator, Session};

/// What every rule's priority was when it wrote none. No rule has a priority
/// since M1f S1f.5.15 (the agenda and the boundary are FIFO, the user's ruling
/// of 2026-11-08 on Q-M1f.6), and the loader drops a `:priority` written
/// anyway until S1f.5.5's swap refuses it: so this census's equal-priority
/// groups are one group, and its stratum emulation, which rewrote priorities,
/// reads nothing since. Kept so the census runs as the record of what it
/// measured.
const DEFAULT_PRIORITY: i64 = 1000;
use ein_infer::{contradiction, obligations};
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
    // The FIFO gap over a static read (`monotonicity.md` § 7.6, S1e2.1b.7):
    // `examples/lattice/10_fifo_gap.ein` with `a` given and only `b` a
    // candidate. `q` is outside D, so the static check finds no site; root
    // derives `q`, and a fork never re-derives it, while scratch re-derives
    // root inside the node, where r2 is admitted first.
    (
        "fifo_gap_static",
        "(relation thing T)\n(relation a T)\n(relation b T)\n(relation p T)\n(relation q T)\n(relation z T)\n\
         (thing one)\n(a one)\n\
         (hrule guess-b () :match (thing ?x) :assert (b ?x))\n\
         (rule r2 () :match (and (a ?x) (b ?x) (absent (q ?x))) :assert (p ?x) :priority 200)\n\
         (rule r1 () :match (and (a ?x) (absent (z ?x))) :assert (q ?x) :priority 200)\n\
         (query :goal (p ?x))\n",
    ),
];

fn written() -> Vec<Source> {
    WRITTEN
        .iter()
        .chain(written_rs::WRITTEN.iter())
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
            Slot::Any | Slot::Opaque(_) => NafArg::Free,
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

fn compare(src: &Source, jobs: usize, keep: bool) -> Json {
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
    let mut terms = Terms::new();
    let junk = terms.intern_text("@id-order-probe").expect("room");
    for k in 0..7 {
        let arg = terms.value_text(&format!("@probe-{k}")).expect("room");
        terms.intern_fact(junk, &[arg]).expect("room");
    }
    for name in &names {
        terms.intern_text(name).expect("room");
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

/// The two `(config …)` flags and their defaults (`configuration.md` § 2.1),
/// since M1e2 S1e2.2.5 retired `warn-derived-naf` (three after S1e2.1c.5
/// retired the four `enable-*` levers).
const FLAGS: &[(&str, bool)] = &[
    ("lattice-sanity-check", false),
    ("record-alternative-justifications", true),
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

// ── The path census (S1e2.1b.7) ─────────────────────────────────────

/// One program loaded once, every construction of a node run in it. Every KB
/// shares one `Terms`, one `Ast` and one memo, so a step is the same function
/// wherever it starts: branch a sealed KB, write facts with a hypothesis's
/// provenance, `Saturator::new`, saturate, seal. [`World::of`] hands back
/// beside it the program as loaded and not saturated, `base`, so
/// `step(base, C)` is scratch, sat(P ∪ C), and `step(step(base, ∅), C)` is
/// the fork, sat(sat(P) ∪ C).
struct World {
    terms: Terms,
    ast: Ast,
    memo: SharedMemo,
    cands: Vec<FactId>,
    texts: Vec<String>,
    monotone: bool,
    strata: Json,
}

/// A state as the path census reads it: its facts' digest, its two bits, how
/// many of its facts are stale, and — kept only where asked — the facts.
#[derive(Clone)]
struct St {
    digest: u64,
    status: u8,
    stale: usize,
    facts: Vec<String>,
    stale_list: Vec<String>,
}

impl World {
    fn of(src: &Source) -> Option<(World, Kb)> {
        let (mut kb, mut terms, ast) = src.load("", Terms::new())?;
        let base = kb.fork();
        let (root, ()) = census::root_of(&mut kb, &mut terms, &ast, |_, _| Some(()))?;
        let mut cands = root.proposed.clone();
        cands.sort_by_key(|&f| sexpr(&terms, f));
        cands.dedup();
        let texts: Vec<String> = cands.iter().map(|&f| sexpr(&terms, f)).collect();
        let dynamic: BTreeSet<String> = root
            .monotonicity
            .dynamic
            .iter()
            .map(|r| r.label(&terms))
            .collect();
        let strata = strata(&kb, &terms, &root.plans, &dynamic);
        Some((
            World {
                terms,
                ast,
                memo: SharedMemo::default(),
                cands,
                texts,
                monotone: root.monotonicity.is_monotone(),
                strata,
            },
            base,
        ))
    }

    /// `sat(parent ∪ add)`: the one step every construction is made of.
    fn step(&mut self, parent: &Kb, add: &[FactId]) -> Option<Kb> {
        let mut kb = parent.branch();
        for &h in add {
            let (rel, args) = self.terms.facts.get(h);
            let args = args.to_vec();
            let prov = self.terms.provs.push(Prov::from_hypothesis(0, None));
            kb.add_and_index_fact(&mut self.terms, rel, &args, Some(prov))
                .ok()?;
        }
        let mut events = Events::off();
        {
            let mut s = Session {
                kb: &mut kb,
                terms: &mut self.terms,
                ast: &self.ast,
                events: &mut events,
                memo: self.memo.clone(),
            };
            let mut sat = Saturator::new(&mut s).ok()?;
            sat.saturate(&mut s, None, &mut |_| {}).ok()?;
        }
        kb.seal_top();
        Some(kb)
    }

    fn facts_of(&self, mask: u64) -> Vec<FactId> {
        bits(mask, self.cands.len())
            .map(|b| self.cands[b])
            .collect()
    }

    fn read(&mut self, kb: &Kb, keep: bool) -> St {
        let mut facts: Vec<String> = kb.facts().map(|f| sexpr(&self.terms, f)).collect();
        facts.sort();
        let mut h = FxHasher::default();
        facts.hash(&mut h);
        let dead = !contradiction::detect(kb, &self.terms).is_empty();
        let owes = obligations::tally(
            kb,
            &mut self.terms,
            &self.ast,
            &self.memo,
            &mut Events::off(),
        )
        .map(|o| o.total())
        .unwrap_or(usize::MAX);
        let plans = monotone::plans_of(&self.memo);
        let (stale, _) = stale_facts(kb, &self.terms, &self.ast, &plans);
        St {
            digest: h.finish(),
            status: (if dead { DEAD } else { 0 }) | (if owes > 0 { OWING } else { 0 }),
            stale: stale.len(),
            facts: if keep { facts } else { Vec::new() },
            stale_list: if keep {
                stale.into_iter().map(|s| s.1).collect()
            } else {
                Vec::new()
            },
        }
    }
}

/// One state a node's paths reach, with how it was reached.
struct Reached {
    st: St,
    kb: Kb,
    /// Reached by a path whose every intermediate state is alive (a one-step
    /// path from P counts): what a route that never extends a dead node meets.
    alive_path: bool,
    /// Reached by a path from root, sat(P), one or more facts a step: the
    /// path tree as F21 states it, which never builds from P with a
    /// commitment written in.
    rooted: bool,
    /// Reached by these constructions: `fork`, `scratch`, a split, a chain,
    /// a path of the DAG.
    fork: bool,
    scratch: bool,
    split: bool,
    chain: bool,
    /// The first construction that reached it, for the text.
    how: String,
}

/// Every `k`-subset … of `mask`, non-empty, as masks.
fn submasks(mask: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut s = mask;
    while s != 0 {
        out.push(s);
        s = (s - 1) & mask;
    }
    out
}

/// How many nodes the path tree has to depth `k` over `n` candidates:
/// Σ_{j ≤ k} n!/(n−j)!.
fn tree_size(n: usize, k: usize) -> u128 {
    let mut total: u128 = 1;
    let mut term: u128 = 1;
    for j in 0..k.min(n) {
        term *= (n - j) as u128;
        total += term;
    }
    total
}

/// The path census of one program: every node built by every construction.
///
/// - **the DAG** — every path from P: a first step `sat(P ∪ K₁)` (K₁ = ∅ is
///   root, K₁ = C is scratch), then steps `s → sat(s ∪ Δ)` for any non-empty
///   Δ, memoized by `(C, the state's facts)`. Its states at C are what every
///   ordered set partition of C reaches, under the premise below;
/// - **the splits** — `sat(sat(P ∪ C₁) ∪ (C \ C₁))` for every C₁ ⊆ C,
///   computed apart from the DAG from scratch's own KB;
/// - **the chains** — the unmemoized path tree from root, one candidate a
///   step, to depth `chain_depth`: every maximal chain of every C with
///   |C| ≤ `chain_depth`;
/// - **the premise** — *a step is a function of the KB's contents*: every
///   step any construction takes is keyed by (its parent's facts, what it
///   adds), and two that share a key and differ in their result are a
///   conflict.
fn path_census(src: &Source, chain_bound: u128, keep: bool) -> Json {
    let mut row = json!({ "path": src.name });
    let Some((mut w, base)) = World::of(src) else {
        row["loads"] = json!(false);
        return row;
    };
    let n = w.cands.len();
    row["n"] = json!(n);
    row["monotone"] = json!(w.monotone);
    row["stratified"] = w.strata["stratified"].clone();
    row["stratified_over_d"] = w.strata["stratified_over_d"].clone();
    if n > LIMIT {
        row["past_limit"] = json!(true);
        return row;
    }
    let t0 = Instant::now();
    let size = 1usize << n;
    let full = (size - 1) as u64;
    let mut premise: FxHashMap<(u64, u64), u64> = FxHashMap::default();
    let (mut premise_keys_repeated, mut premise_conflicts) = (0usize, 0usize);
    let mut conflicts: Vec<String> = Vec::new();
    let mut steps = 0usize;
    // Record a step: (parent's facts, what it added as a mask over the
    // candidates) → the result's facts.
    let mut note = |parent: u64,
                    add: u64,
                    out: u64,
                    what: String,
                    premise: &mut FxHashMap<(u64, u64), u64>| {
        match premise.get(&(parent, add)) {
            Some(&o) => {
                premise_keys_repeated += 1;
                if o != out {
                    premise_conflicts += 1;
                    if conflicts.len() < 8 {
                        conflicts.push(what);
                    }
                }
            }
            None => {
                premise.insert((parent, add), out);
            }
        }
    };

    // Scratch, for every C: one step from P.
    let mut scratch: Vec<(Kb, St)> = Vec::with_capacity(size);
    let base_digest = {
        let mut facts: Vec<String> = base.facts().map(|f| sexpr(&w.terms, f)).collect();
        facts.sort();
        let mut h = FxHasher::default();
        facts.hash(&mut h);
        h.finish()
    };
    for mask in 0..size as u64 {
        let add = w.facts_of(mask);
        let Some(kb) = w.step(&base, &add) else {
            row["failed"] = json!("a scratch step");
            return row;
        };
        steps += 1;
        let st = w.read(&kb, keep);
        note(base_digest, mask, st.digest, String::new(), &mut premise);
        scratch.push((kb, st));
    }
    let root_digest = scratch[0].1.digest;

    // The DAG, by popcount.
    let mut order: Vec<u64> = (0..size as u64).collect();
    order.sort_by_key(|m| (m.count_ones(), *m));
    let mut reach: Vec<Vec<Reached>> = (0..size).map(|_| Vec::new()).collect();
    reach[0].push(Reached {
        st: scratch[0].1.clone(),
        kb: scratch[0].0.branch(),
        alive_path: true,
        rooted: true,
        fork: true,
        scratch: true,
        split: true,
        chain: true,
        how: "root".to_string(),
    });
    let mut fork_digest = vec![0u64; size];
    fork_digest[0] = root_digest;
    for &mask in order.iter().skip(1) {
        let mut here: Vec<Reached> = Vec::new();
        let put = |here: &mut Vec<Reached>, r: Reached| {
            if let Some(x) = here.iter_mut().find(|x| x.st.digest == r.st.digest) {
                x.alive_path |= r.alive_path;
                x.rooted |= r.rooted;
                x.fork |= r.fork;
                x.scratch |= r.scratch;
                x.split |= r.split;
            } else {
                here.push(r);
            }
        };
        let (skb, sst) = (&scratch[mask as usize].0, scratch[mask as usize].1.clone());
        put(
            &mut here,
            Reached {
                st: sst,
                kb: skb.branch(),
                alive_path: true,
                rooted: false,
                fork: false,
                scratch: true,
                split: true,
                chain: false,
                how: "scratch".to_string(),
            },
        );
        for delta in submasks(mask) {
            let rest = mask & !delta;
            let add = w.facts_of(delta);
            let parents: Vec<(u64, u8, bool, bool, Kb, String)> = reach[rest as usize]
                .iter()
                .map(|p| {
                    (
                        p.st.digest,
                        p.st.status,
                        p.alive_path,
                        p.rooted,
                        p.kb.branch(),
                        p.how.clone(),
                    )
                })
                .collect();
            for (pd, pstatus, palive, prooted, pkb, phow) in parents {
                let Some(kb) = w.step(&pkb, &add) else {
                    row["failed"] = json!("a DAG step");
                    return row;
                };
                steps += 1;
                let st = w.read(&kb, keep);
                note(
                    pd,
                    delta,
                    st.digest,
                    format!("{} + {}", phow, name(&w.texts, delta)),
                    &mut premise,
                );
                let is_fork = rest == 0;
                if is_fork {
                    fork_digest[mask as usize] = st.digest;
                }
                let how = format!("{phow} + {}", name(&w.texts, delta));
                put(
                    &mut here,
                    Reached {
                        st,
                        kb,
                        alive_path: palive && pstatus & DEAD == 0,
                        rooted: prooted,
                        fork: is_fork,
                        scratch: false,
                        split: false,
                        chain: false,
                        how,
                    },
                );
            }
        }
        reach[mask as usize] = here;
    }

    // The splits, from scratch's own KB.
    let mut split_states: Vec<BTreeSet<u64>> = vec![BTreeSet::new(); size];
    for mask in 0..size as u64 {
        for c1 in submasks(mask).into_iter().chain([0]) {
            let rest = mask & !c1;
            let d = if rest == 0 {
                scratch[mask as usize].1.digest
            } else {
                let add = w.facts_of(rest);
                let Some(kb) = w.step(&scratch[c1 as usize].0, &add) else {
                    row["failed"] = json!("a split");
                    return row;
                };
                steps += 1;
                let st = w.read(&kb, false);
                note(
                    scratch[c1 as usize].1.digest,
                    rest,
                    st.digest,
                    format!("scratch {} + {}", name(&w.texts, c1), name(&w.texts, rest)),
                    &mut premise,
                );
                st.digest
            };
            split_states[mask as usize].insert(d);
        }
    }

    // The chains: the unmemoized tree from root, one candidate a step.
    let mut depth = 0usize;
    while depth < n && tree_size(n, depth + 1) <= chain_bound {
        depth += 1;
    }
    let mut chain_states: Vec<BTreeSet<u64>> = vec![BTreeSet::new(); size];
    chain_states[0].insert(root_digest);
    let mut tree_nodes = 1usize;
    {
        // (kb, its facts' digest, the commitment so far, the sequence)
        let mut stack: Vec<(Kb, u64, u64, usize)> =
            vec![(scratch[0].0.branch(), root_digest, 0, 0)];
        while let Some((kb, d, mask, len)) = stack.pop() {
            if len >= depth {
                continue;
            }
            for b in 0..n {
                if mask >> b & 1 == 1 {
                    continue;
                }
                let h = w.cands[b];
                let Some(child) = w.step(&kb, &[h]) else {
                    row["failed"] = json!("a chain step");
                    return row;
                };
                steps += 1;
                tree_nodes += 1;
                let st = w.read(&child, false);
                note(d, 1 << b, st.digest, String::new(), &mut premise);
                let m2 = mask | 1 << b;
                chain_states[m2 as usize].insert(st.digest);
                stack.push((child, st.digest, m2, len + 1));
            }
        }
    }
    for (mask, set) in chain_states.iter().enumerate() {
        for x in reach[mask].iter_mut() {
            if set.contains(&x.st.digest) {
                x.chain = true;
            }
        }
    }
    for (mask, set) in split_states.iter().enumerate() {
        for x in reach[mask].iter_mut() {
            if set.contains(&x.st.digest) {
                x.split = true;
            }
        }
    }

    // The oracle's node: is the fork here region::Nodes' fork?
    let oracle = fork_route(src, Terms::new(), false);
    let fork_is_oracle = oracle.as_ref().is_some_and(|o| {
        o.uni.cands == w.texts
            && (0..size).all(|m| {
                o.reads[m].digest == fork_digest[m]
                    && reach[m]
                        .iter()
                        .find(|x| x.st.digest == fork_digest[m])
                        .is_some_and(|x| x.st.status == o.reads[m].status())
            })
    });
    // And is scratch here `utils/region.py`'s, the text load?
    let scratch_is_text = (0..size as u64).all(|m| {
        scratch_node(src, &commitment_text(&w.texts, m), false, Terms::new())
            .is_some_and(|r| r.digest == scratch[m as usize].1.digest)
    });

    // Per node.
    let mut dependent = 0;
    let mut dependent_alive = 0;
    let mut beyond = 0;
    let mut status_differ = 0;
    let mut status_differ_alive = 0;
    let mut splits_differ = 0;
    let mut chains_differ = 0;
    let mut chains_checked = 0;
    let mut f_hist: BTreeMap<usize, usize> = BTreeMap::new();
    let mut f_alive_hist: BTreeMap<usize, usize> = BTreeMap::new();
    let mut f_rooted_hist: BTreeMap<usize, usize> = BTreeMap::new();
    let mut states_hist: BTreeMap<usize, usize> = BTreeMap::new();
    let mut rooted_hist: BTreeMap<usize, usize> = BTreeMap::new();
    let mut pairs = 0;
    let mut distinct: BTreeSet<u64> = BTreeSet::new();
    let mut detail: Vec<Json> = Vec::new();
    for mask in 0..size {
        let here = &reach[mask];
        pairs += here.len();
        distinct.extend(here.iter().map(|x| x.st.digest));
        let fd = fork_digest[mask];
        let sd = scratch[mask].1.digest;
        if here.len() > 1 {
            dependent += 1;
        }
        let alive: Vec<&Reached> = here.iter().filter(|x| x.alive_path).collect();
        if alive.len() > 1 {
            dependent_alive += 1;
        }
        if here.iter().any(|x| x.st.digest != fd && x.st.digest != sd) {
            beyond += 1;
        }
        let statuses: BTreeSet<u8> = here.iter().map(|x| x.st.status).collect();
        if statuses.len() > 1 {
            status_differ += 1;
        }
        let statuses_alive: BTreeSet<u8> = alive.iter().map(|x| x.st.status).collect();
        if statuses_alive.len() > 1 {
            status_differ_alive += 1;
        }
        if split_states[mask].len() > 1 {
            splits_differ += 1;
        }
        if (mask as u64).count_ones() as usize <= depth {
            chains_checked += 1;
            if chain_states[mask].len() > 1 || !chain_states[mask].contains(&fd) {
                chains_differ += 1;
            }
        }
        let free = here.iter().filter(|x| x.st.stale == 0).count();
        *f_hist.entry(free.min(2)).or_default() += 1;
        let free_alive = here
            .iter()
            .filter(|x| x.st.stale == 0 && x.st.status & DEAD == 0)
            .count();
        *f_alive_hist.entry(free_alive.min(2)).or_default() += 1;
        let rooted: Vec<&Reached> = here.iter().filter(|x| x.rooted).collect();
        *f_rooted_hist
            .entry(rooted.iter().filter(|x| x.st.stale == 0).count().min(2))
            .or_default() += 1;
        *states_hist.entry(here.len()).or_default() += 1;
        *rooted_hist.entry(rooted.len()).or_default() += 1;
        if keep && here.len() > 1 && detail.len() < 12 {
            let fork_facts: BTreeSet<&String> = here
                .iter()
                .find(|x| x.st.digest == fd)
                .map(|x| x.st.facts.iter().collect())
                .unwrap_or_default();
            let states: Vec<Json> = here
                .iter()
                .map(|x| {
                    let mine: BTreeSet<&String> = x.st.facts.iter().collect();
                    let mut by: Vec<&str> = Vec::new();
                    if x.fork {
                        by.push("fork");
                    }
                    if x.scratch {
                        by.push("scratch");
                    }
                    if x.split {
                        by.push("a split");
                    }
                    if x.chain {
                        by.push("a chain");
                    }
                    if x.rooted {
                        by.push("from root");
                    }
                    json!({
                        "status": class(x.st.status),
                        "by": by,
                        "first": x.how,
                        "alive_path": x.alive_path,
                        "stale": x.st.stale_list,
                        "only_here": mine.difference(&fork_facts).collect::<Vec<_>>(),
                        "only_fork": fork_facts.difference(&mine).collect::<Vec<_>>(),
                    })
                })
                .collect();
            detail.push(json!({ "node": name(&w.texts, mask as u64), "states": states }));
        }
    }
    let _ = full;
    row["nodes"] = json!(size);
    row["steps"] = json!(steps);
    row["chain_depth"] = json!(depth);
    row["tree_nodes"] = json!(tree_nodes);
    row["fork_is_the_oracle"] = json!(fork_is_oracle);
    row["scratch_is_the_text_load"] = json!(scratch_is_text);
    row["premise"] = json!({
        "keys": premise.len(),
        "repeated": premise_keys_repeated,
        "conflicts": premise_conflicts,
        "first": conflicts,
    });
    row["dag"] = json!({ "pairs": pairs, "states": distinct.len() });
    row["dependent"] = json!(dependent);
    row["dependent_alive_paths"] = json!(dependent_alive);
    row["beyond_fork_and_scratch"] = json!(beyond);
    row["status_differs"] = json!(status_differ);
    row["status_differs_alive_paths"] = json!(status_differ_alive);
    row["splits_differ"] = json!(splits_differ);
    row["chains_checked"] = json!(chains_checked);
    row["chains_differ"] = json!(chains_differ);
    row["stale_free_states_per_node"] = json!(
        f_hist
            .iter()
            .map(|(k, v)| (
                if *k >= 2 {
                    "2+".to_string()
                } else {
                    k.to_string()
                },
                *v
            ))
            .collect::<BTreeMap<_, _>>()
    );
    row["stale_free_alive_states_per_node"] = json!(
        f_alive_hist
            .iter()
            .map(|(k, v)| (
                if *k >= 2 {
                    "2+".to_string()
                } else {
                    k.to_string()
                },
                *v
            ))
            .collect::<BTreeMap<_, _>>()
    );
    let hist = |h: &BTreeMap<usize, usize>, cap: bool| -> Json {
        json!(
            h.iter()
                .map(|(k, v)| (
                    if cap && *k >= 2 {
                        "2+".to_string()
                    } else {
                        k.to_string()
                    },
                    *v
                ))
                .collect::<BTreeMap<_, _>>()
        )
    };
    row["stale_free_rooted_states_per_node"] = hist(&f_rooted_hist, true);
    row["states_per_node"] = hist(&states_hist, false);
    row["rooted_states_per_node"] = hist(&rooted_hist, false);
    if keep {
        row["detail"] = json!(detail);
    }
    row["ms"] = json!((t0.elapsed().as_secs_f64() * 1e4).round() / 10.0);
    row
}

fn print_paths(row: &Json) {
    if row.get("nodes").is_none() {
        println!(
            "{}: not read — {}",
            row["path"].as_str().unwrap_or("?"),
            if row["past_limit"].as_bool() == Some(true) {
                "past the limit".to_string()
            } else {
                row.to_string()
            }
        );
        return;
    }
    println!(
        "{}  n {} · {} · stratified {} / over D {} · {} ms",
        row["path"].as_str().unwrap_or("?"),
        row["n"],
        if row["monotone"].as_bool() == Some(true) {
            "monotone"
        } else {
            "flagged"
        },
        row["stratified"],
        row["stratified_over_d"],
        row["ms"],
    );
    println!(
        "  nodes {} · steps {} · chains to depth {} ({} tree nodes) · fork is the oracle {} · scratch is the text load {}",
        row["nodes"],
        row["steps"],
        row["chain_depth"],
        row["tree_nodes"],
        row["fork_is_the_oracle"],
        row["scratch_is_the_text_load"]
    );
    println!(
        "  premise {} · DAG {} (C, state) pairs, {} distinct states",
        row["premise"], row["dag"]["pairs"], row["dag"]["states"]
    );
    println!(
        "  path-dependent nodes {} (alive paths {}) · beyond fork and scratch {} · status differs {} (alive paths {}) · splits differ {} · chains differ {} of {}",
        row["dependent"],
        row["dependent_alive_paths"],
        row["beyond_fork_and_scratch"],
        row["status_differs"],
        row["status_differs_alive_paths"],
        row["splits_differ"],
        row["chains_differ"],
        row["chains_checked"]
    );
    println!(
        "  states per node {} · from root {} · stale-free states per node {} · alive and stale-free {} · from root and stale-free {}",
        row["states_per_node"],
        row["rooted_states_per_node"],
        row["stale_free_states_per_node"],
        row["stale_free_alive_states_per_node"],
        row["stale_free_rooted_states_per_node"]
    );
    for d in row["detail"].as_array().into_iter().flatten() {
        println!("  node {}", d["node"].as_str().unwrap_or("?"));
        for s in d["states"].as_array().into_iter().flatten() {
            println!(
                "    {} by {} (first {}){} · only here {} · only fork {} · stale {}",
                s["status"].as_str().unwrap_or("?"),
                s["by"],
                s["first"].as_str().unwrap_or("?"),
                if s["alive_path"].as_bool() == Some(true) {
                    ""
                } else {
                    " · only through a dead parent"
                },
                s["only_here"],
                s["only_fork"],
                s["stale"]
            );
        }
    }
}

// ── The cumulativity census (S1e2.1b.7) ─────────────────────────────

/// Is saturation **cumulative** on this program, X ⊆ Y ⊆ sat(X) ⇒ sat(Y) =
/// sat(X)? Read one fact at a time: for every node, each construction's X
/// (scratch: P ∪ C; fork: sat(P) ∪ C), and every fact f its fixpoint derived
/// beyond X, whether sat(X ∪ {f}) = sat(X). In [`World`], so every
/// saturation is the same step the path census takes.
fn cumulative_census(src: &Source, keep: bool) -> Json {
    let mut row = json!({ "path": src.name });
    let Some((mut w, base)) = World::of(src) else {
        row["loads"] = json!(false);
        return row;
    };
    cumulative_in(&mut w, base, src, keep, &mut row);
    row
}

fn cumulative_in(w: &mut World, base: Kb, src: &Source, keep: bool, row: &mut Json) {
    let n = w.cands.len();
    row["n"] = json!(n);
    row["monotone"] = json!(w.monotone);
    row["stratified"] = w.strata["stratified"].clone();
    row["stratified_over_d"] = w.strata["stratified_over_d"].clone();
    if n > LIMIT {
        row["past_limit"] = json!(true);
        return;
    }
    let _ = src;
    let t0 = Instant::now();
    let Some(root) = w.step(&base, &[]) else {
        row["failed"] = json!("root");
        return;
    };
    let mut out = serde_json::Map::new();
    for (label, start) in [("scratch", &base), ("fork", &root)] {
        let start_facts: FxHashSet<FactId> = start.facts().collect();
        let (mut tests, mut fails, mut nodes_failing) = (0usize, 0usize, 0usize);
        let mut first: Vec<Json> = Vec::new();
        for mask in 0..1u64 << n {
            // Each node's provenance is reclaimed when the node is done, as
            // `region::Nodes` reclaims it, so 2ⁿ nodes do not grow the arena.
            w.terms.provs.open_fork();
            let c = w.facts_of(mask);
            let Some(fix) = w.step(start, &c) else {
                row["failed"] = json!(label);
                return;
            };
            let d0 = w.read(&fix, keep);
            let derived: Vec<FactId> = fix
                .facts()
                .filter(|f| !start_facts.contains(f) && !c.contains(f))
                .collect();
            let mut failed_here = false;
            for f in derived {
                let mut cf = c.clone();
                cf.push(f);
                let Some(y) = w.step(start, &cf) else {
                    row["failed"] = json!(label);
                    return;
                };
                tests += 1;
                let d1 = w.read(&y, keep);
                if d1.digest != d0.digest {
                    fails += 1;
                    failed_here = true;
                    if keep && first.len() < 6 {
                        let a: BTreeSet<&String> = d0.facts.iter().collect();
                        let b: BTreeSet<&String> = d1.facts.iter().collect();
                        first.push(json!({
                            "node": name(&w.texts, mask),
                            "f": sexpr(&w.terms, f),
                            "status": [class(d0.status), class(d1.status)],
                            "only_sat_x": a.difference(&b).collect::<Vec<_>>(),
                            "only_sat_x_and_f": b.difference(&a).collect::<Vec<_>>(),
                        }));
                    }
                }
            }
            nodes_failing += failed_here as usize;
            drop(fix);
            w.terms.provs.discard_fork();
        }
        out.insert(
            label.to_string(),
            json!({ "tests": tests, "fail": fails, "nodes_failing": nodes_failing, "first": first }),
        );
    }
    row["nodes"] = json!(1usize << n);
    row["cumulative"] = Json::Object(out);
    row["ms"] = json!((t0.elapsed().as_secs_f64() * 1e4).round() / 10.0);
}

// ── Admission in stratum order, emulated (S1e2.1b.7) ────────────────

/// The program's text with every saturation rule that reads under an `absent`
/// given the priority of its stratum, so that the boundary, which admits the
/// lowest priority first, admits in stratum order. An emulation of F21's
/// *admission in stratum order*, not the option built: the engine is
/// unchanged, and only the rules written in the program's own text are moved.
///
/// A relation's stratum is the most `absent` edges on a path into it, over
/// the graph `strata` reads (a plan's reads to its writes, an activator to
/// what its rule writes), where an edge counts when its read is at any
/// `absent` depth. A cycle holding such an edge has no stratum order, and the
/// program is reported as such.
fn stratum_text(src: &Source) -> Result<(String, Json), String> {
    let (mut kb, mut terms, ast) = src
        .load("", Terms::new())
        .ok_or_else(|| "does not load".to_string())?;
    let (root, ()) = census::root_of(&mut kb, &mut terms, &ast, |_, _| Some(()))
        .ok_or_else(|| "does not saturate".to_string())?;
    let program = kb.program();
    let sat_plans: Vec<&Plan> = root
        .plans
        .iter()
        .filter(|p| program.rules.contains(p.rule))
        .map(|p| &**p)
        .collect();
    let mut labels: BTreeSet<String> = BTreeSet::new();
    for p in &sat_plans {
        for w in monotone::writes_of(p, &terms) {
            labels.insert(w.label(&terms));
        }
        for (r, _) in monotone::reads_of(p, &terms) {
            labels.insert(r.label(&terms));
        }
    }
    let negatives: Vec<String> = labels
        .iter()
        .filter(|l| l.starts_with("(not ") && *l != "(not ?)")
        .cloned()
        .collect();
    // (from, to, strict)
    let mut edges: BTreeSet<(String, String, bool)> = BTreeSet::new();
    // rule → its writes, and whether it reads under an absent
    let mut rules: BTreeMap<String, (BTreeSet<String>, bool)> = BTreeMap::new();
    for p in &sat_plans {
        let rule = terms.sym(p.rule).to_string();
        let writes: Vec<String> = monotone::writes_of(p, &terms)
            .into_iter()
            .map(|w| w.label(&terms))
            .collect();
        let e = rules.entry(rule).or_default();
        e.0.extend(writes.iter().cloned());
        for (r, depth) in monotone::reads_of(p, &terms) {
            if depth >= 1 {
                e.1 = true;
            }
            let from = r.label(&terms);
            let froms = if from == "(not ?)" {
                negatives.clone()
            } else {
                vec![from]
            };
            for f in froms {
                for w in &writes {
                    edges.insert((f.clone(), w.clone(), depth >= 1));
                }
            }
        }
    }
    for rule in program.rules.values() {
        if rule.params.is_empty() {
            continue;
        }
        let act = terms.sym(rule.name).to_string();
        for p in sat_plans.iter().filter(|p| p.rule == rule.name) {
            for w in monotone::writes_of(p, &terms) {
                edges.insert((act.clone(), w.label(&terms), false));
            }
        }
    }
    let mut ids: BTreeMap<String, usize> = BTreeMap::new();
    for (a, b, _) in &edges {
        let k = ids.len();
        ids.entry(a.clone()).or_insert(k);
        let k = ids.len();
        ids.entry(b.clone()).or_insert(k);
    }
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); ids.len()];
    for (a, b, _) in &edges {
        adj[ids[a]].push(ids[b]);
    }
    let (comp, ncomp) = scc(&adj);
    if let Some((a, b, _)) = edges
        .iter()
        .find(|(a, b, s)| *s && comp[ids[a]] == comp[ids[b]])
    {
        return Err(format!(
            "no stratum order: a cycle through an absent ({a} → {b})"
        ));
    }
    let mut stratum = vec![0usize; ncomp];
    for _ in 0..=ncomp {
        let mut changed = false;
        for (a, b, s) in &edges {
            let (ca, cb) = (comp[ids[a]], comp[ids[b]]);
            let want = stratum[ca] + *s as usize;
            if ca != cb && want > stratum[cb] {
                stratum[cb] = want;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    // Each rule that reads under an absent: the stratum of what it writes.
    let mut level: BTreeMap<String, usize> = BTreeMap::new();
    for (rule, (writes, reads_absent)) in &rules {
        if !reads_absent {
            continue;
        }
        let l = writes
            .iter()
            .filter_map(|w| ids.get(w).map(|&i| stratum[comp[i]]))
            .max()
            .unwrap_or(0);
        level.insert(rule.clone(), l);
    }
    // Rewrite the text.
    let text = &src.text;
    let forms = top_forms(text);
    let mut out = String::with_capacity(text.len() + 64);
    let mut at = 0;
    let mut moved: BTreeMap<String, Json> = BTreeMap::new();
    for (a, z) in forms {
        let form = &text[a..z];
        let Some(rest) = form.strip_prefix("(rule ") else {
            continue;
        };
        let name = rest.split_whitespace().next().unwrap_or("").to_string();
        let Some(&l) = level.get(&name) else {
            continue;
        };
        let new_p = 10 + l as i64;
        let old_p = priority_of(form);
        moved.insert(
            name.clone(),
            json!({ "stratum": l, "from": old_p, "to": new_p }),
        );
        out.push_str(&text[at..a]);
        let body = &form[..form.len() - 1];
        let rewritten = match body.find(":priority") {
            Some(i) => {
                let after = &body[i + ":priority".len()..];
                let trimmed = after.trim_start();
                let num_len = trimmed
                    .find(|ch: char| ch.is_whitespace() || ch == ')')
                    .unwrap_or(trimmed.len());
                format!("{}:priority {}{})", &body[..i], new_p, &trimmed[num_len..])
            }
            None => format!("{body} :priority {new_p})"),
        };
        out.push_str(&rewritten);
        at = z;
    }
    out.push_str(&text[at..]);
    let not_written: Vec<&String> = level.keys().filter(|r| !moved.contains_key(*r)).collect();
    Ok((
        out,
        json!({ "rules": moved, "not_in_the_text": not_written }),
    ))
}

/// Admission in stratum order on one program: both routes on the rewritten
/// text against today's, and cumulativity under it.
fn stratum_row(src: &Source, jobs: usize) -> Json {
    let mut row = json!({ "path": src.name });
    let (text, how) = match stratum_text(src) {
        Ok(x) => x,
        Err(e) => {
            row["stratum_order"] = json!(e);
            return row;
        }
    };
    row["rewrite"] = how;
    let moved_any = text != src.text;
    row["text_moved"] = json!(moved_any);
    let src2 = src.with_text(text);
    let (Some((f0, s0, _)), Some((f1, s1, _))) =
        (both(src, jobs, &Terms::new), both(&src2, jobs, &Terms::new))
    else {
        row["skipped"] = json!("past the limit or does not load");
        return row;
    };
    let r_of = |ans: &Answer| -> Vec<String> {
        ans.iter()
            .filter(|(_, v)| v.0 == 0)
            .map(|(k, _)| k.clone())
            .collect()
    };
    let states_moved = |a: &Answer, b: &Answer| -> Vec<String> {
        a.iter()
            .filter(|(k, v)| b.get(*k).is_some_and(|w| w != *v))
            .map(|(k, _)| k.clone())
            .collect()
    };
    row["fork"] = json!({ "moved": moved(&f0, &f1), "R_today": r_of(&f0), "R": r_of(&f1),
                          "nodes_moved": states_moved(&f0, &f1) });
    row["scratch"] = json!({ "moved": moved(&s0, &s1), "R_today": r_of(&s0), "R": r_of(&s1),
                             "nodes_moved": states_moved(&s0, &s1) });
    // Cumulativity under it.
    let mut c = json!({});
    if let Some((mut w, base)) = World::of(&src2) {
        cumulative_in(&mut w, base, &src2, false, &mut c);
    }
    row["cumulative"] = c["cumulative"].clone();
    // And the stale facts from scratch under it: every node, both ways.
    let fork = fork_route(&src2, Terms::new(), false);
    if let Some(fk) = fork {
        let scratch: Vec<Option<Read>> =
            scratch_route(&src2, &fk.uni.cands, false, jobs, &Terms::new);
        row["stale_nodes"] = json!({
            "fork": fk.reads.iter().filter(|r| !r.stale.is_empty()).count(),
            "scratch": scratch.iter().flatten().filter(|r| !r.stale.is_empty()).count(),
        });
    }
    row
}

// ── The selection and main ──────────────────────────────────────────

/// `f` over `items` on `jobs` threads, in order.
fn par_map<T: Sync, R: Send>(items: &[T], jobs: usize, f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let next = std::sync::atomic::AtomicUsize::new(0);
    let out: Vec<std::sync::Mutex<Option<R>>> =
        items.iter().map(|_| std::sync::Mutex::new(None)).collect();
    std::thread::scope(|sc| {
        for _ in 0..jobs.max(1).min(items.len().max(1)) {
            sc.spawn(|| {
                loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if i >= items.len() {
                        break;
                    }
                    let r = f(&items[i]);
                    *out[i].lock().expect("a slot") = Some(r);
                }
            });
        }
    });
    out.into_iter()
        .map(|m| m.into_inner().expect("a slot").expect("filled"))
        .collect()
}

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
    let mut paths_mode = false;
    let (mut cumulative, mut stratum) = (false, false);
    let mut chain_bound: u128 = 13_700;
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
            "--paths" => paths_mode = true,
            "--cumulative" => cumulative = true,
            "--stratum-order" => stratum = true,
            "--chain-bound" => {
                chain_bound = args
                    .next()
                    .and_then(|v| v.parse().ok())
                    .expect("--chain-bound B")
            }
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

    if cumulative {
        let rows = par_map(&chosen, jobs, |src| cumulative_census(src, !control));
        let mut sum: BTreeMap<String, u64> = BTreeMap::new();
        for row in &rows {
            if as_json {
                println!("{row}");
            }
            if row.get("nodes").is_none() {
                if !as_json && !control {
                    println!(
                        "{}: not read — {}",
                        row["path"].as_str().unwrap_or("?"),
                        row
                    );
                }
                continue;
            }
            *sum.entry("programs".into()).or_default() += 1;
            *sum.entry("nodes".into()).or_default() += row["nodes"].as_u64().unwrap_or(0);
            let strat = row["stratified"].as_bool() == Some(true);
            for k in ["scratch", "fork"] {
                let c = &row["cumulative"][k];
                for f in ["tests", "fail", "nodes_failing"] {
                    *sum.entry(format!("{k}_{f}")).or_default() += c[f].as_u64().unwrap_or(0);
                }
                if c["fail"].as_u64().unwrap_or(0) > 0 {
                    *sum.entry(format!(
                        "{k}_programs_failing{}",
                        if strat { "_stratified" } else { "" }
                    ))
                    .or_default() += 1;
                }
            }
            if strat {
                *sum.entry("stratified".into()).or_default() += 1;
            }
            let failing = ["scratch", "fork"]
                .iter()
                .any(|k| row["cumulative"][k]["fail"].as_u64().unwrap_or(0) > 0);
            if !as_json && (!control || failing) {
                println!(
                    "{}  n {} · {} · stratified {} / over D {} · {} ms",
                    row["path"].as_str().unwrap_or("?"),
                    row["n"],
                    if row["monotone"].as_bool() == Some(true) {
                        "monotone"
                    } else {
                        "flagged"
                    },
                    row["stratified"],
                    row["stratified_over_d"],
                    row["ms"]
                );
                for k in ["scratch", "fork"] {
                    let c = &row["cumulative"][k];
                    println!(
                        "  {k:<7} {} tests · {} change the fixpoint · on {} of {} nodes",
                        c["tests"], c["fail"], c["nodes_failing"], row["nodes"]
                    );
                    for x in c["first"].as_array().into_iter().flatten() {
                        println!(
                            "    {} + {}: {} → {} · only sat(X) {} · only sat(X ∪ f) {}",
                            x["node"].as_str().unwrap_or("?"),
                            x["f"].as_str().unwrap_or("?"),
                            x["status"][0].as_str().unwrap_or("?"),
                            x["status"][1].as_str().unwrap_or("?"),
                            x["only_sat_x"],
                            x["only_sat_x_and_f"]
                        );
                    }
                }
            }
        }
        if !as_json {
            println!("\n{}", json!(sum));
            println!("wall {:.1} s", t_all.elapsed().as_secs_f64());
        }
        return;
    }

    if stratum {
        let rows = par_map(&chosen, 1.max(jobs / 4), |src| stratum_row(src, 4));
        for row in &rows {
            if as_json {
                println!("{row}");
                continue;
            }
            println!("{}", row["path"].as_str().unwrap_or("?"));
            if let Some(e) = row["stratum_order"].as_str() {
                println!("  {e}");
                continue;
            }
            println!(
                "  rewrite {} · text moved {}",
                row["rewrite"], row["text_moved"]
            );
            for k in ["fork", "scratch"] {
                println!(
                    "  {k:<7} moved {} · R today {} → {} · nodes moved {}",
                    row[k]["moved"], row[k]["R_today"], row[k]["R"], row[k]["nodes_moved"]
                );
            }
            println!(
                "  cumulative under it {} · stale nodes under it {}",
                json!({
                    "scratch_fail": row["cumulative"]["scratch"]["fail"],
                    "fork_fail": row["cumulative"]["fork"]["fail"],
                }),
                row["stale_nodes"]
            );
        }
        if !as_json {
            println!("wall {:.1} s", t_all.elapsed().as_secs_f64());
        }
        return;
    }

    if paths_mode {
        let rows = par_map(&chosen, jobs, |src| path_census(src, chain_bound, !control));
        let mut sum: BTreeMap<&str, u64> = BTreeMap::new();
        for row in &rows {
            if as_json {
                println!("{row}");
            } else if !control || row["dependent"].as_u64().unwrap_or(0) > 0 {
                print_paths(row);
            }
            if row.get("nodes").is_none() {
                continue;
            }
            *sum.entry("programs").or_default() += 1;
            for k in [
                "nodes",
                "steps",
                "tree_nodes",
                "dependent",
                "dependent_alive_paths",
                "beyond_fork_and_scratch",
                "status_differs",
                "status_differs_alive_paths",
                "splits_differ",
                "chains_differ",
                "chains_checked",
            ] {
                *sum.entry(k).or_default() += row[k].as_u64().unwrap_or(0);
            }
            *sum.entry("premise_repeated").or_default() +=
                row["premise"]["repeated"].as_u64().unwrap_or(0);
            *sum.entry("premise_conflicts").or_default() +=
                row["premise"]["conflicts"].as_u64().unwrap_or(0);
            *sum.entry("dag_pairs").or_default() += row["dag"]["pairs"].as_u64().unwrap_or(0);
            if row["fork_is_the_oracle"].as_bool() != Some(true) {
                *sum.entry("fork_not_the_oracle").or_default() += 1;
            }
            if row["scratch_is_the_text_load"].as_bool() != Some(true) {
                *sum.entry("scratch_not_the_text_load").or_default() += 1;
            }
            if row["dependent"].as_u64().unwrap_or(0) > 0 {
                *sum.entry("programs_dependent").or_default() += 1;
            }
            for (field, prefix) in [
                ("stale_free_states_per_node", "stale_free"),
                ("stale_free_rooted_states_per_node", "stale_free_rooted"),
            ] {
                for (k, v) in row[field].as_object().into_iter().flatten() {
                    *sum.entry(match (prefix, k.as_str()) {
                        ("stale_free", "0") => "stale_free_0",
                        ("stale_free", "1") => "stale_free_1",
                        ("stale_free", _) => "stale_free_2+",
                        (_, "0") => "stale_free_rooted_0",
                        (_, "1") => "stale_free_rooted_1",
                        _ => "stale_free_rooted_2+",
                    })
                    .or_default() += v.as_u64().unwrap_or(0);
                }
            }
        }
        if !as_json {
            println!("\n{}", json!(sum));
            println!("wall {:.1} s", t_all.elapsed().as_secs_f64());
        }
        return;
    }

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
        let row = compare(src, jobs, keep);
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
