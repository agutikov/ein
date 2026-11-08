//! The rule dependency graph and its strata — M1f
//! [S1f.5.15](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md)'s
//! T3 and T4, on the user's note of 2026-08-28, *"remove `:priority`
//! completely / analyze rules structure instead"*.
//!
//! **The graph.** An edge from each relation a compiled plan reads to each it
//! writes, negative where the read is under an odd number of `absent`s, and a
//! positive one from a rule's activator relation (its name) to what its plans
//! write ([`edges`]). It is over the **compile cache**, not the source: a
//! relation-polymorphic rule, `(rule symmetric (?R) …)`, has no relation in
//! its head until an activator binds `?R`, so its edges exist only per plan.
//! Pass the plans of a run that has saturated root, as `naf_deps` asks. The
//! detector's cycle check ([`crate::detector`]) is the same graph restricted to
//! `D`, and reads it from here.
//!
//! **The strata** ([`graph`]). The classic construction: a relation's stratum
//! is the longest path into it counting negative edges, over the graph's
//! strongly connected components, and a component with a negative edge inside
//! is a **negative cycle**, where no stratification exists ([`Cycle`]). A
//! rule's stratum is the highest of the relations its plans write.
//!
//! **What it is for.** S1f.5.15 began by deriving a band per rule from its
//! shape, to order the agenda where `:priority` had (T3, the diff against the
//! typed numbers: 275 of 471 agreed). The user ruled on 2026-11-08 (Q-M1f.6,
//! *"plain FIFO, core is an explanation, F22 makes it canonical"*) that nothing
//! orders the agenda or the boundary but the written and arrival order, so the
//! band went with `:priority`, and the graph is the stratification diagnostic's
//! (T4: [`crate::monotone::Monotonicity::negative_cycles`]) and the stratum
//! `ein saturate --dump` prints beside each rule ([`rule_strata`]).

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use ein_core::{Kb, Symbol, Terms};

use crate::monotone;
use crate::plan::Plan;

// ── The graph ───────────────────────────────────────────────────────

/// A labelled edge of the dependency graph: from, to, negative, the rule.
pub type Edge = (String, String, bool, String);

/// A strongly connected component of the dependency graph with a negative
/// edge inside it — a **negative cycle**.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cycle {
    /// Its relations, as [`monotone::Rel::label`] writes them, sorted.
    pub relations: Vec<String>,
    /// Its negative edges, `rule: a →¬ b`, sorted.
    pub negative: Vec<String>,
    /// Some cycle in it has an odd number of negative edges: a state with no
    /// stale fact need not exist. Even: there can be several.
    pub odd: bool,
}

impl Cycle {
    /// The cycle as the stratification diagnostic says it (M1f S1f.5.15's
    /// T4): its relations, its negative edges, and what the parity may mean
    /// for root's fixpoint. A possibility the graph licenses, never a fact
    /// about a run: a cycle whose rules never fire changes nothing.
    pub fn stratum_text(&self) -> String {
        format!(
            "{} — {} — {}",
            self.relations.join(", "),
            self.negative.join(" · "),
            if self.odd {
                "odd: a fixpoint supported at the boundary need not exist, and the one root reaches \
                 may depend on the boundary's order"
            } else {
                "even: several fixpoints may exist, and which one root reaches may depend on the \
                 order the boundary admits in"
            }
        )
    }
}

/// The saturation rules' dependency graph over every relation their plans
/// name (`node_census.md` § 5's): an edge from each relation a plan reads to
/// each it writes, negative at odd `absent` depth, and a positive one from an
/// activator relation to what the rule's plans write. A wildcard negative read,
/// `(not ?)`, reads every stored negative the graph names.
pub fn edges(kb: &Kb, terms: &Terms, plans: &[Arc<Plan>]) -> BTreeSet<Edge> {
    let program = kb.program();
    let sat: Vec<&Plan> = plans
        .iter()
        .filter(|p| program.rules.contains(p.rule))
        .map(|p| &**p)
        .collect();
    let mut labels: BTreeSet<String> = BTreeSet::new();
    for p in &sat {
        for w in monotone::writes_of(p, terms) {
            labels.insert(w.label(terms));
        }
        for (r, _) in monotone::reads_of(p, terms) {
            labels.insert(r.label(terms));
        }
    }
    let negatives: Vec<String> = labels
        .iter()
        .filter(|l| l.starts_with("(not ") && *l != "(not ?)")
        .cloned()
        .collect();
    let mut edges: BTreeSet<Edge> = BTreeSet::new();
    for p in &sat {
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
        for p in sat.iter().filter(|p| p.rule == rule.name) {
            for w in monotone::writes_of(p, terms) {
                edges.insert((act.clone(), w.label(terms), false, act.clone()));
            }
        }
    }
    edges
}

/// The strongly connected components of `edges` with a negative edge inside,
/// each with its parity.
pub fn components(edges: &[&Edge]) -> Vec<Cycle> {
    let mut ids: BTreeMap<&str, usize> = BTreeMap::new();
    for (a, b, _, _) in edges {
        let k = ids.len();
        ids.entry(a.as_str()).or_insert(k);
        let k = ids.len();
        ids.entry(b.as_str()).or_insert(k);
    }
    let mut names = vec![""; ids.len()];
    for (s, &i) in &ids {
        names[i] = s;
    }
    let n = ids.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (a, b, _, _) in edges {
        adj[ids[a.as_str()]].push(ids[b.as_str()]);
    }
    let (comp, ncomp) = scc(&adj);
    let mut out = Vec::new();
    for c in 0..ncomp {
        let inside: Vec<&Edge> = edges
            .iter()
            .copied()
            .filter(|(a, b, _, _)| comp[ids[a.as_str()]] == c && comp[ids[b.as_str()]] == c)
            .collect();
        if !inside.iter().any(|e| e.2) {
            continue;
        }
        let members: Vec<usize> = (0..n).filter(|&v| comp[v] == c).collect();
        // A 2-colouring by negative-edge parity: a conflict is an odd cycle.
        let mut parity: BTreeMap<usize, bool> = BTreeMap::new();
        parity.insert(members[0], false);
        let mut odd = false;
        let mut changed = true;
        while changed {
            changed = false;
            for (a, b, neg, _) in inside.iter().copied() {
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
        let mut relations: Vec<String> = members.iter().map(|&v| names[v].to_string()).collect();
        relations.sort();
        let negative: BTreeSet<String> = inside
            .iter()
            .filter(|e| e.2)
            .map(|(a, b, _, r)| format!("{r}: {a} →¬ {b}"))
            .collect();
        out.push(Cycle {
            relations,
            negative: negative.into_iter().collect(),
            odd,
        });
    }
    out
}

/// Tarjan's strongly connected components: each vertex's component, and how
/// many there are. A component is numbered before every component with an edge
/// into it, so the numbering, read backwards, is a topological order.
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

/// The dependency graph of a run's saturation plans, with its strata.
#[derive(Clone, Debug, Default)]
pub struct Graph {
    /// Every edge, sorted ([`edges`]).
    pub edges: Vec<Edge>,
    /// Each relation's stratum: the most negative edges on a path into it,
    /// over the components. A component's relations share one.
    pub stratum: BTreeMap<String, u32>,
    /// The negative cycles, sorted by their relations: where no
    /// stratification exists. Empty on a stratified program.
    pub cycles: Vec<Cycle>,
}

impl Graph {
    /// Does a stratification exist?
    pub fn stratified(&self) -> bool {
        self.cycles.is_empty()
    }

    /// The number of strata: one more than the highest.
    pub fn depth(&self) -> u32 {
        self.stratum.values().max().map_or(0, |m| m + 1)
    }
}

/// The dependency graph of `plans` and its strata. Across a negative cycle a
/// stratum is still assigned, counting the edges into the component and not
/// those inside it; the cycle is reported beside it ([`Graph::cycles`]).
pub fn graph(kb: &Kb, terms: &Terms, plans: &[Arc<Plan>]) -> Graph {
    let edges: Vec<Edge> = edges(kb, terms, plans).into_iter().collect();
    let mut ids: BTreeMap<&str, usize> = BTreeMap::new();
    for (a, b, _, _) in &edges {
        let k = ids.len();
        ids.entry(a.as_str()).or_insert(k);
        let k = ids.len();
        ids.entry(b.as_str()).or_insert(k);
    }
    let n = ids.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (a, b, _, _) in &edges {
        adj[ids[a.as_str()]].push(ids[b.as_str()]);
    }
    let (comp, ncomp) = scc(&adj);
    // Edges between components, read in topological order: Tarjan numbers a
    // component before every component with an edge into it.
    let mut level = vec![0u32; ncomp];
    let mut into: Vec<Vec<(usize, bool)>> = vec![Vec::new(); ncomp];
    for (a, b, neg, _) in &edges {
        let (ca, cb) = (comp[ids[a.as_str()]], comp[ids[b.as_str()]]);
        if ca != cb {
            into[cb].push((ca, *neg));
        }
    }
    for c in (0..ncomp).rev() {
        let l = into[c]
            .iter()
            .map(|&(from, neg)| level[from] + u32::from(neg))
            .max()
            .unwrap_or(0);
        level[c] = l;
    }
    let stratum = ids
        .iter()
        .map(|(name, &v)| (name.to_string(), level[comp[v]]))
        .collect();
    let refs: Vec<&Edge> = edges.iter().collect();
    let mut cycles = components(&refs);
    cycles.sort_by(|a, b| a.relations.cmp(&b.relations));
    Graph {
        edges,
        stratum,
        cycles,
    }
}

// ── A rule's stratum ────────────────────────────────────────────────

/// Every saturation rule of the program, in load order, with the highest
/// stratum of what its plans write, or `None` where the run compiled no plan
/// of it: what `ein saturate --dump` prints beside each rule.
pub fn rule_strata(
    kb: &Kb,
    terms: &Terms,
    plans: &[Arc<Plan>],
    graph: &Graph,
) -> Vec<(Symbol, Option<u32>)> {
    kb.program()
        .rules
        .values()
        .map(|r| {
            let mut stratum: Option<u32> = None;
            for p in plans.iter().filter(|p| p.rule == r.name) {
                for w in monotone::writes_of(p, terms) {
                    if let Some(&k) = graph.stratum.get(&w.label(terms)) {
                        stratum = Some(stratum.map_or(k, |m| m.max(k)));
                    }
                }
            }
            (r.name, stratum)
        })
        .collect()
}
