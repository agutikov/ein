//! Derivation walks — the premise closure, the DAG, the unsat core, and the
//! load-time cycle check.
//!
//! Every walk here answers the same question with a different appetite: what
//! does this fact rest on? Since S1.21.7 a fact is an **OR-node** over its
//! recorded derivations, so each walk takes an explicit choice —
//! [`Justifications::Primary`] or [`Justifications::All`] — rather than
//! taking the first-recorded one by accident, which is what every walker did
//! before the alternatives existed.
//!
//! Cycles are real here and are not an error: the symmetric mirror makes
//! `(R a b)` and `(R b a)` justify each other in any ordinary puzzle. The
//! walks break at re-visit; [`detect_provenance_cycles`] rejects only
//! *user-authored* cycles, at load time, where a circular `:using` chain is a
//! malformed input.

use crate::bitset::BitSet;
use crate::facts::FactId;
use crate::kb::Kb;
use crate::prov::{ProvId, ProvKind};
use crate::terms::Terms;

/// Which derivations a walk expands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Justifications {
    /// `Fact.provenance` — the pre-S1.21.7 single-derivation reading, and the
    /// default at every call site.
    Primary,
    /// The whole OR-node. The result is a strictly larger frontier: useful as
    /// a soundness envelope ("no explanation can name a fact outside this"),
    /// but **not** an explanation — no single derivation used all of those
    /// premises.
    All,
}

impl Justifications {
    fn of(self, kb: &Kb, fact: FactId) -> Vec<ProvId> {
        match self {
            Justifications::Primary => kb.primary(fact).into_iter().collect(),
            Justifications::All => kb.justifications(fact),
        }
    }
}

/// Resolve a premise id to a fact this KB actually holds.
///
/// ein.py's `_fact_by_id` scans the relation extent, so it answers for
/// *indexed* facts; belief and indexing part company only between the
/// loader's `add_fact` and the `rebuild_indexes` that ends every load, and no
/// walk runs in that window.
fn resolve(kb: &Kb, premise: FactId) -> Option<FactId> {
    kb.contains(premise).then_some(premise)
}

/// Is this fact on the derivation **frontier** — what the engine treats as
/// given?
pub fn is_frontier(kb: &Kb, terms: &Terms, fact: FactId) -> bool {
    match kb.primary(fact) {
        None => true,
        Some(p) => matches!(
            terms.provs.get(p).kind,
            ProvKind::Source | ProvKind::Hypothesis
        ),
    }
}

/// Every fact in `root`'s transitive premise closure for which `keep` holds.
///
/// `keep` decides membership only; it does **not** stop the walk — rule-kind
/// facts are always expanded. `visited` guards cycles and memoises across
/// roots, so passing a shared set collects the frontier of several conflicting
/// facts in one pass; the union is identical to walking each separately.
///
/// Iterative, because a deep derivation chain would otherwise be a recursion
/// limit. Returns first-visit order — ein.py returns a `set`, whose iteration
/// order was the H4 hazard, so an order that is merely *defined* is already
/// an improvement; every display site sorts regardless.
pub fn walk_premises(
    kb: &Kb,
    terms: &Terms,
    root: FactId,
    keep: &dyn Fn(&Kb, &Terms, FactId) -> bool,
    visited: &mut BitSet,
    justifications: Justifications,
) -> Vec<FactId> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(f) = stack.pop() {
        if !visited.insert(f.0) {
            continue;
        }
        if keep(kb, terms, f) {
            out.push(f);
        }
        for p in justifications.of(kb, f) {
            let prov = terms.provs.get(p);
            if prov.kind != ProvKind::Rule {
                continue;
            }
            for &premise in &prov.premises {
                if let Some(premise) = resolve(kb, premise) {
                    stack.push(premise);
                }
            }
        }
    }
    out
}

/// The frontier of given/assumed facts across a set of conflicting facts.
///
/// Primary-justification only by default, and that is a choice rather than
/// leftover behaviour: unioning over alternatives makes the core
/// monotonically *larger*, which is the opposite of what a legible
/// explanation needs. For a minimum-cardinality answer the search layer
/// *chooses* one justification per fact instead of unioning them.
pub fn unsat_core(
    kb: &Kb,
    terms: &Terms,
    conflicting: &[FactId],
    justifications: Justifications,
) -> Vec<FactId> {
    let mut visited = BitSet::new();
    let mut core = Vec::new();
    for &f in conflicting {
        core.extend(walk_premises(
            kb,
            terms,
            f,
            &is_frontier,
            &mut visited,
            justifications,
        ));
    }
    core
}

/// A directed acyclic graph of fact derivations.
///
/// Edges go premise → conclusion. `and_nodes` carries the conjunction
/// structure `edges` cannot: one entry per *justification*, pairing a
/// conclusion with the premises of that one derivation. A flat edge set loses
/// this — with several justifications the in-edges of a node are the union
/// over derivations, and which subset constitutes one proof is unrecoverable.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DerivationDag {
    pub root: FactId,
    pub nodes: Vec<FactId>,
    pub edges: Vec<(FactId, FactId)>,
    pub and_nodes: Vec<(FactId, Vec<FactId>)>,
}

impl DerivationDag {
    /// True iff some fact here has more than one recorded derivation — the
    /// AND/**OR** case, which the renderer draws differently.
    pub fn is_or_graph(&self) -> bool {
        let mut seen = BitSet::new();
        self.and_nodes
            .iter()
            .any(|(conclusion, _)| !seen.insert(conclusion.0))
    }

    /// Terminal facts: source-kind, hypothesis-kind, or un-provenanced.
    pub fn sources(&self, kb: &Kb, terms: &Terms) -> Vec<FactId> {
        self.nodes
            .iter()
            .copied()
            .filter(|&n| is_frontier(kb, terms, n))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

/// BFS over premises, resolving each id in `kb`.
///
/// Cycles are broken by tracking visited facts; the revisited fact appears as
/// a node but is not re-expanded.
pub fn build_derivation_dag(
    kb: &Kb,
    terms: &Terms,
    root: FactId,
    justifications: Justifications,
) -> DerivationDag {
    let mut dag = DerivationDag {
        root,
        nodes: vec![root],
        edges: Vec::new(),
        and_nodes: Vec::new(),
    };
    let mut seen = BitSet::new();
    seen.insert(root.0);
    let mut queue = std::collections::VecDeque::from([root]);
    while let Some(f) = queue.pop_front() {
        for p in justifications.of(kb, f) {
            let prov = terms.provs.get(p);
            if prov.kind != ProvKind::Rule {
                continue;
            }
            let mut group = Vec::new();
            for &premise in &prov.premises {
                let Some(premise) = resolve(kb, premise) else {
                    continue;
                };
                group.push(premise);
                dag.edges.push((premise, f));
                if seen.insert(premise.0) {
                    dag.nodes.push(premise);
                    queue.push_back(premise);
                }
            }
            dag.and_nodes.push((f, group));
        }
    }
    dag
}

/// Cycles in the **user-authored** provenance graph, empty when there are
/// none. The loader rejects a non-empty result.
///
/// Deliberately primary-justification only, and deliberately load-time only:
/// a cycle in authored provenance is a malformed input, while a cycle in
/// engine-recorded provenance is normal once re-derivations are recorded, so
/// running this over a saturated KB would reject well-founded knowledge bases.
///
/// Iterative, with the recursion's visit order preserved exactly — the loader
/// prints `cycles[0]`, so which cycle is found first is observable.
pub fn detect_provenance_cycles(kb: &Kb, terms: &Terms) -> Vec<Vec<FactId>> {
    enum Step {
        Enter(FactId),
        Leave,
    }

    let mut out: Vec<Vec<FactId>> = Vec::new();
    let mut visited = BitSet::new();
    let mut on_path = BitSet::new();
    let mut path: Vec<FactId> = Vec::new();

    for root in kb.facts() {
        let mut stack = vec![Step::Enter(root)];
        while let Some(step) = stack.pop() {
            let f = match step {
                Step::Leave => {
                    let left = path.pop().expect("a frame to leave");
                    on_path.remove(left.0);
                    continue;
                }
                Step::Enter(f) => f,
            };
            if on_path.contains(f.0) {
                let at = path.iter().position(|&p| p == f).expect("on the path");
                let mut cycle = path[at..].to_vec();
                cycle.push(f);
                out.push(cycle);
                continue;
            }
            if !visited.insert(f.0) {
                continue;
            }
            path.push(f);
            on_path.insert(f.0);
            stack.push(Step::Leave);
            if let Some(p) = kb.primary(f) {
                let prov = terms.provs.get(p);
                if prov.kind == ProvKind::Rule {
                    // Reversed, so the children pop in premise order.
                    for &premise in prov.premises.iter().rev() {
                        if let Some(premise) = resolve(kb, premise) {
                            stack.push(Step::Enter(premise));
                        }
                    }
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::Relation;
    use crate::program::Program;
    use crate::prov::Prov;
    use crate::value::Value;

    struct World {
        terms: Terms,
        kb: Kb,
    }

    impl World {
        fn new() -> World {
            let mut terms = Terms::new();
            let mut program = Program::new();
            for name in ["p", "q", "r"] {
                let name = terms.intern_text(name).expect("room");
                program.add_relation(Relation {
                    name,
                    signature: Box::new([]),
                    declared: true,
                    why: None,
                    loc: None,
                });
            }
            World {
                kb: Kb::new(program),
                terms,
            }
        }

        fn given(&mut self, rel: &str, args: &[&str]) -> FactId {
            let prov = self.terms.provs.push(Prov::from_source(None, None));
            self.add(rel, args, Some(prov))
        }

        fn derived(&mut self, rel: &str, args: &[&str], rule: &str, from: &[FactId]) -> FactId {
            let rule = self.terms.intern_text(rule).expect("room");
            let prov = self.terms.provs.push(Prov::from_rule(
                rule,
                from.to_vec().into_boxed_slice(),
                None,
            ));
            self.add(rel, args, Some(prov))
        }

        fn add(&mut self, rel: &str, args: &[&str], prov: Option<ProvId>) -> FactId {
            let rel = self.terms.intern_text(rel).expect("room");
            let args: Vec<Value> = args
                .iter()
                .map(|a| self.terms.value_text(a).expect("room"))
                .collect();
            self.kb
                .add_and_index_fact(&mut self.terms, rel, &args, prov)
                .expect("room")
                .id()
        }
    }

    #[test]
    fn the_dag_stops_at_the_frontier_and_breaks_at_a_revisit() {
        let mut w = World::new();
        let a = w.given("p", &["a"]);
        let b = w.given("p", &["b"]);
        let ab = w.derived("q", &["a", "b"], "join", &[a, b]);
        let top = w.derived("r", &["a"], "lift", &[ab, a]);

        let dag = build_derivation_dag(&w.kb, &w.terms, top, Justifications::Primary);
        assert_eq!(dag.nodes, vec![top, ab, a, b]);
        assert_eq!(dag.sources(&w.kb, &w.terms), vec![a, b]);
        // `a` is reached twice — once through the join, once directly — and
        // appears as a node once, with both edges.
        assert_eq!(dag.edges.iter().filter(|(p, _)| *p == a).count(), 2);
        assert!(!dag.is_or_graph(), "one derivation each");
    }

    #[test]
    fn an_unresolvable_premise_is_skipped_rather_than_invented() {
        let mut w = World::new();
        let a = w.given("p", &["a"]);
        // A premise the branch never believed: interned, but not held here.
        let phantom = {
            let rel = w.terms.intern_text("p").expect("room");
            let arg = w.terms.value_text("ghost").expect("room");
            w.terms.intern_fact(rel, &[arg]).expect("room")
        };
        let top = w.derived("q", &["a"], "join", &[a, phantom]);
        let dag = build_derivation_dag(&w.kb, &w.terms, top, Justifications::Primary);
        assert_eq!(dag.nodes, vec![top, a]);
        assert_eq!(dag.and_nodes, vec![(top, vec![a])]);
    }

    #[test]
    fn the_unsat_core_is_the_union_of_the_frontiers() {
        let mut w = World::new();
        let a = w.given("p", &["a"]);
        let b = w.given("p", &["b"]);
        let c = w.given("p", &["c"]);
        let x = w.derived("q", &["x"], "join", &[a, b]);
        let y = w.derived("q", &["y"], "join", &[b, c]);
        let mut core = unsat_core(&w.kb, &w.terms, &[x, y], Justifications::Primary);
        core.sort();
        assert_eq!(core, vec![a, b, c]);
        // A derived fact is never in the core; only what it rests on.
        assert!(!core.contains(&x));
    }

    #[test]
    fn an_all_justifications_walk_is_a_superset_of_the_primary_one() {
        let mut w = World::new();
        let a = w.given("p", &["a"]);
        let b = w.given("p", &["b"]);
        let c = w.given("p", &["c"]);
        let top = w.derived("q", &["top"], "first", &[a, b]);
        let rule = w.terms.intern_text("second").expect("room");
        let alt = w
            .terms
            .provs
            .push(Prov::from_rule(rule, Box::new([c]), None));
        assert!(w.kb.record_justification(&w.terms, top, alt));

        let primary = unsat_core(&w.kb, &w.terms, &[top], Justifications::Primary);
        let mut all = unsat_core(&w.kb, &w.terms, &[top], Justifications::All);
        all.sort();
        assert_eq!(primary.len(), 2);
        assert_eq!(all, vec![a, b, c]);
    }

    #[test]
    fn a_cycle_is_reported_once_with_its_path() {
        // A circular `:using` chain. Building one is easy *because* interning
        // is not belief: both ids exist before either fact is believed, so
        // each record can name the other.
        let mut terms = Terms::new();
        let mut kb = Kb::new(Program::new());
        let p = terms.intern_text("p").expect("room");
        let a_arg = terms.value_text("a").expect("room");
        let b_arg = terms.value_text("b").expect("room");
        let a = terms.intern_fact(p, &[a_arg]).expect("room");
        let b = terms.intern_fact(p, &[b_arg]).expect("room");
        let forth = terms.intern_text("forth").expect("room");
        let back = terms.intern_text("back").expect("room");
        let from_b = terms
            .provs
            .push(Prov::from_rule(forth, Box::new([b]), None));
        let from_a = terms.provs.push(Prov::from_rule(back, Box::new([a]), None));
        kb.add_and_index_fact(&mut terms, p, &[a_arg], Some(from_b))
            .expect("room");
        kb.add_and_index_fact(&mut terms, p, &[b_arg], Some(from_a))
            .expect("room");

        assert_eq!(detect_provenance_cycles(&kb, &terms), vec![vec![a, b, a]]);
    }

    #[test]
    fn a_well_founded_chain_reports_no_cycle() {
        let mut w = World::new();
        let a = w.given("p", &["a"]);
        let b = w.derived("q", &["b"], "step", &[a]);
        w.derived("r", &["c"], "step", &[b]);
        assert!(detect_provenance_cycles(&w.kb, &w.terms).is_empty());
    }
}
