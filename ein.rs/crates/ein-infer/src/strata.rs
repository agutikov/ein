//! The rule dependency graph, its strata, and a band per rule derived from its
//! shape — M1f
//! [S1f.5.15](../../../../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md)'s
//! T3, on the user's note of 2026-08-28, *"remove `:priority` completely /
//! analyze rules structure instead"*, and the ruling of 2026-11-08 on
//! Q-M1f.4, (b) *"Derive order first"*.
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
//! **The band** ([`band`]). `ein saturate --dump` has always printed the four
//! bands a declared priority falls in (`propagate` < 200 ≤ `derive` < 300 ≤
//! `eliminate` < 900 ≤ `hypothesis`, Q41's). This derives one from what the
//! rule does instead of what its author typed ([`Shape`]): what it concludes,
//! whether it reads under an `absent`, how many premises it joins, and whether
//! it writes another rule's activator. [`declared_band`] reads the typed
//! number the same way, so the two can be diffed rule by rule, which is the
//! task's deliverable.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use ein_core::{Kb, Symbol, Terms};

use crate::monotone;
use crate::plan::{Plan, Slot, Step};

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

// ── The band ────────────────────────────────────────────────────────

/// The four bands `ein saturate --dump` has printed since Q41, in the order
/// the agenda fires them.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Band {
    /// Cheap forward propagation.
    Propagate,
    /// Ordinary derivation.
    Derive,
    /// Domain or range elimination.
    Eliminate,
    /// The guessing rung's.
    Hypothesis,
}

impl Band {
    pub const ALL: [Band; 4] = [
        Band::Propagate,
        Band::Derive,
        Band::Eliminate,
        Band::Hypothesis,
    ];

    /// The word `ein saturate --dump` prints.
    pub fn label(self) -> &'static str {
        match self {
            Band::Propagate => "propagate",
            Band::Derive => "derive",
            Band::Eliminate => "eliminate",
            Band::Hypothesis => "hypothesis",
        }
    }

    /// Where the agenda puts the band, lower first: the floor of the band's
    /// range of declared priorities.
    pub fn rank(self) -> i64 {
        match self {
            Band::Propagate => 100,
            Band::Derive => 200,
            Band::Eliminate => 300,
            Band::Hypothesis => 900,
        }
    }
}

/// The band a declared priority falls in, Q41's ranges: `propagate` below 200,
/// `derive` below 300, `eliminate` below 900, `hypothesis` from 900.
pub fn declared_band(priority: i64) -> Band {
    match priority {
        p if p < 200 => Band::Propagate,
        p if p < 300 => Band::Derive,
        p if p < 900 => Band::Eliminate,
        _ => Band::Hypothesis,
    }
}

/// What a rule's conclusions are, the strongest of them: an obligation's debt,
/// `(false)`, a stored negative, or positive facts alone.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Concludes {
    Positive,
    Negative,
    False,
    Open,
}

impl Concludes {
    pub fn label(self) -> &'static str {
        match self {
            Concludes::Positive => "positive",
            Concludes::Negative => "negative",
            Concludes::False => "false",
            Concludes::Open => "open",
        }
    }
}

/// What the band is derived from: what a rule does, read off a compiled plan.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Shape {
    /// The strongest conclusion its `:assert` writes.
    pub concludes: Concludes,
    /// A disjunct reads under an `absent` (a `forall` included): its
    /// candidates wait for the boundary.
    pub naf: bool,
    /// The most top-level relation premises a disjunct joins.
    pub premises: u16,
    /// It writes another rule's activator: a fact whose relation names a
    /// rule, what a declaration's setup fan-out does.
    pub activates: bool,
    /// It reads a stored negative under a `forall`, `(absent (and G (absent
    /// (not (R …)))))`: a conclusion drawn from every alternative excluded.
    pub exhausts: bool,
}

/// A plan's [`Shape`]. `obligation` says the plan's rule asserts the verdict
/// atom `(open ?R)` — an obligation rule, which the saturator never fires.
pub fn shape(kb: &Kb, terms: &Terms, plan: &Plan, obligation: bool) -> Shape {
    let program = kb.program();
    let mut concludes = Concludes::Positive;
    let mut activates = false;
    for slot in plan.asserts.iter() {
        let Slot::Nested { rel, .. } = slot else {
            continue;
        };
        let c = if obligation || *rel == terms.kernel.open {
            Concludes::Open
        } else if *rel == terms.kernel.r#false {
            Concludes::False
        } else if *rel == terms.kernel.not {
            Concludes::Negative
        } else {
            if program.rules.contains(*rel) || program.obligations.contains(*rel) {
                activates = true;
            }
            Concludes::Positive
        };
        concludes = concludes.max(c);
    }
    let premises = plan
        .disjuncts
        .iter()
        .map(|d| {
            plan.steps(d.steps)
                .iter()
                .filter(|s| matches!(s, Step::Rel(_)))
                .count() as u16
        })
        .max()
        .unwrap_or(0);
    let exhausts = monotone::reads_of(plan, terms)
        .into_iter()
        .any(|(r, depth)| r.negated && depth >= 2 && depth % 2 == 0);
    Shape {
        concludes,
        naf: plan.has_naf(),
        premises,
        activates,
        exhausts,
    }
}

/// The band a rule's shape puts it in, read from what each band is for:
///
/// - **propagate**, cheap forward propagation: a declaration's fan-out (it
///   writes another rule's activator); a rule of one premise that reads
///   nothing under an `absent`, one fact mapped to one; and a refutation,
///   `(false)`, which ends the node, so that a dead node is found before work
///   is spent on it;
/// - **eliminate**, domain or range elimination: an obligation, `(open ?R)`,
///   the totality an exhausted domain still owes, which the obligations pass
///   reads at quiescence; and a positive conclusion drawn from a `forall` over
///   stored negatives, survivor forcing (every other value excluded, so this
///   one);
/// - **derive**, ordinary derivation: every other rule, a join that concludes
///   a positive or a stored negative, or a conclusion read under an `absent`
///   that is not a `forall` over negatives.
///
/// **hypothesis** is never derived: the guessing rung is `(hrule …)`'s and the
/// generator's, and no saturation rule guesses.
pub fn band(shape: &Shape) -> Band {
    match shape.concludes {
        Concludes::Open => Band::Eliminate,
        Concludes::False => Band::Propagate,
        _ if shape.activates => Band::Propagate,
        Concludes::Positive if shape.exhausts => Band::Eliminate,
        _ if !shape.naf && shape.premises <= 1 => Band::Propagate,
        _ => Band::Derive,
    }
}

/// One rule, its declared priority and the band its shape derives.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleBand {
    pub rule: Symbol,
    /// The rule's `:priority`, where it writes one.
    pub declared: Option<i64>,
    /// The shape of its compiled plans, where the run compiled one; a rule
    /// whose plans differ in shape takes the first's, and [`Self::shapes`]
    /// counts how many there were.
    pub shape: Option<Shape>,
    /// How many distinct shapes its plans have.
    pub shapes: usize,
    /// The band [`band`] derives, where there is a shape.
    pub derived: Option<Band>,
    /// The highest stratum of what its plans write.
    pub stratum: Option<u32>,
}

/// Every rule and obligation rule of the program, in load order, with its
/// declared priority, its shape over `plans` and the band that derives.
pub fn rule_bands(kb: &Kb, terms: &Terms, plans: &[Arc<Plan>], graph: &Graph) -> Vec<RuleBand> {
    let program = kb.program();
    let declared = |r: &ein_core::Rule| -> Option<i64> {
        r.priority.map(|p| {
            terms.ints.value(p).unwrap_or_else(|| {
                if terms.int_text(p).starts_with('-') {
                    i64::MIN
                } else {
                    i64::MAX
                }
            })
        })
    };
    let mut out = Vec::new();
    let rules = program
        .rules
        .values()
        .map(|r| (r, false))
        .chain(program.obligations.values().map(|r| (r, true)));
    for (r, obligation) in rules {
        let mut shapes: Vec<Shape> = Vec::new();
        let mut stratum: Option<u32> = None;
        for p in plans.iter().filter(|p| p.rule == r.name) {
            let s = shape(kb, terms, p, obligation);
            if !shapes.contains(&s) {
                shapes.push(s);
            }
            for w in monotone::writes_of(p, terms) {
                if let Some(&k) = graph.stratum.get(&w.label(terms)) {
                    stratum = Some(stratum.map_or(k, |m| m.max(k)));
                }
            }
        }
        let shape = shapes.first().copied();
        out.push(RuleBand {
            rule: r.name,
            declared: declared(r),
            shape,
            shapes: shapes.len(),
            derived: shape.as_ref().map(band),
            stratum,
        });
    }
    out
}
