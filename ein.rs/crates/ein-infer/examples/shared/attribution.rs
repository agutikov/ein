//! Which site made a node fall out of R — M1e2
//! [S1e2.1b.3](../../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.3_holes.md).
//!
//! One rule, read by `site_census` (S1e2.1b.1) and `hole_census` (S1e2.1b.3),
//! so the two name the same site for the same node. Given a node the cube
//! puts outside R, it names a **cause**, one of the six below, and the
//! [`Site`]s of the static check that made it:
//!
//! | cause | kind | the site named |
//! |---|---|---|
//! | [`Cause::AbsentRefutes`] | dead | an `absent` site whose rule refuted the node, over the relation of an `absent` premise of the refutation |
//! | [`Cause::Via`] | dead | an `absent` site over the relation of an `absent` premise, where the refuting rule reads none itself |
//! | [`Cause::ActivatorRefutes`] | dead | the activator site of a refuting rule root compiled no plan of |
//! | [`Cause::AbsentWitness`] | owing | an `absent` site whose rule derived a fact the node below holds and this node does not |
//! | [`Cause::Trigger`] | owing | the trigger site of an instance owed here and not below |
//! | [`Cause::ActivatorObligation`] | owing | the activator site of an obligation owed here whose plan root never compiled |
//!
//! *Below* is the node of R the debt is new against: for a hole, the largest
//! subset of it in R ([`r_below`]); for a discharged node with an owing
//! superset, the node itself. The first two rules and the trigger rule are
//! S1e2.1b.1's, unchanged. `AbsentWitness` and `ActivatorObligation` are the
//! two S1e2.1b.3 adds for the gap S1e2.1b.6's fixtures found (`lattice/08`,
//! `lattice/09`). `ActivatorRefutes` reads `refuted_by` rather than the
//! node's `absent` premises, because a refutation a node compiles can rest on
//! an `absent` that no premise records (the `activator_under_absent` program
//! in `written.rs`).
//!
//! Example code: no route reads it, and it changes no answer.

use std::collections::BTreeSet;
use std::sync::Arc;

use ein_core::{FactId, Kb, Symbol, Terms};
use ein_infer::compile::SharedMemo;
use ein_infer::events::Events;
use ein_infer::monotone::{Site, SiteKind};
use ein_infer::obligations::{self, Owed};
use ein_infer::plan::Plan;
use ein_infer::region::{self, Region};
use ein_ir::Ast;

/// What made a node fall out of R, by the read that did it.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Cause {
    AbsentRefutes,
    Via,
    ActivatorRefutes,
    AbsentWitness,
    Trigger,
    ActivatorObligation,
}

impl Cause {
    pub const ALL: [Cause; 6] = [
        Cause::AbsentRefutes,
        Cause::Via,
        Cause::ActivatorRefutes,
        Cause::AbsentWitness,
        Cause::Trigger,
        Cause::ActivatorObligation,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Cause::AbsentRefutes => "absent-refutes",
            Cause::Via => "via",
            Cause::ActivatorRefutes => "activator-refutes",
            Cause::AbsentWitness => "absent-witness",
            Cause::Trigger => "trigger",
            Cause::ActivatorObligation => "activator-obligation",
        }
    }

    pub fn words(self) -> &'static str {
        match self {
            Cause::AbsentRefutes => "an `absent` that concludes `(false)`",
            Cause::Via => "a refutation through a fact derived under an `absent`",
            Cause::ActivatorRefutes => "an activator that compiles a refutation",
            Cause::AbsentWitness => "an `absent` that takes a witness away",
            Cause::Trigger => "an obligation's trigger",
            Cause::ActivatorObligation => "an activator that compiles an obligation",
        }
    }

    /// `dead` or `owing`: the bit the cause sets.
    pub fn kind(self) -> &'static str {
        match self {
            Cause::AbsentRefutes | Cause::Via | Cause::ActivatorRefutes => "dead",
            _ => "owing",
        }
    }
}

/// A site, rendered once, before `region::Nodes` borrows the oracle's `Terms`.
pub struct SiteText {
    pub kind: SiteKind,
    /// Its relation, as [`ein_infer::monotone::Rel::label`] writes it.
    pub label: String,
    pub rule: Symbol,
    pub rule_text: String,
    pub activator: Box<[Symbol]>,
    pub text: String,
}

pub fn site_texts(sites: &[Site], terms: &Terms) -> Vec<SiteText> {
    sites
        .iter()
        .map(|s| SiteText {
            kind: s.kind,
            label: s.relation.label(terms),
            rule: s.rule,
            rule_text: terms.sym(s.rule).to_string(),
            activator: s.activator.clone(),
            text: s.render(terms),
        })
        .collect()
}

/// Root's plans as `(rule, activator)` text: a plan a node compiles is one not
/// here. Text, so a second load of the program can be read against it.
pub struct RootPlans {
    pairs: BTreeSet<(String, Vec<String>)>,
    rules: BTreeSet<String>,
}

impl RootPlans {
    pub fn of(plans: &[Arc<Plan>], terms: &Terms) -> RootPlans {
        let mut pairs = BTreeSet::new();
        let mut rules = BTreeSet::new();
        for p in plans {
            let rule = terms.sym(p.rule).to_string();
            let act = p
                .activator_args
                .iter()
                .map(|&a| terms.sym(a).to_string())
                .collect();
            pairs.insert((rule.clone(), act));
            rules.insert(rule);
        }
        RootPlans { pairs, rules }
    }

    pub fn has(&self, rule: &str, activator: &[String]) -> bool {
        self.pairs.contains(&(rule.to_string(), activator.to_vec()))
    }

    pub fn has_rule(&self, rule: &str) -> bool {
        self.rules.contains(rule)
    }
}

/// `(absent (not (q A)))`'s relation, as [`ein_infer::monotone::Rel::label`]
/// writes it.
pub fn absent_label(premise: &str) -> String {
    let inner = premise
        .strip_prefix("(absent (")
        .unwrap_or(premise)
        .trim_end_matches(')');
    let mut words = inner.split_whitespace();
    match words.next() {
        Some("not") => {
            let r = words
                .next()
                .map(|w| w.trim_start_matches('(').trim_end_matches(')'))
                .unwrap_or("?");
            if r.is_empty() || r == "_" {
                "(not ?)".to_string()
            } else {
                format!("(not {r})")
            }
        }
        Some(r) => r.to_string(),
        None => "?".to_string(),
    }
}

/// What one node is attributed to: its causes (one for nearly every node) and
/// the sites named, sorted and distinct.
#[derive(Clone, Debug, Default)]
pub struct Attributed {
    pub causes: Vec<Cause>,
    pub sites: Vec<String>,
}

impl Attributed {
    pub fn named(&self) -> bool {
        !self.sites.is_empty()
    }
}

fn finish(causes: BTreeSet<Cause>, sites: BTreeSet<String>) -> Attributed {
    Attributed {
        causes: causes.into_iter().collect(),
        sites: sites.into_iter().collect(),
    }
}

/// A dead node, by its refutation: `refuted_by` and the `absent` premises the
/// oracle read off the derivation of its first contradiction.
pub fn dead(sites: &[SiteText], root: &RootPlans, node: &region::Node) -> Attributed {
    let mut causes = BTreeSet::new();
    let mut named = BTreeSet::new();
    // A refuting rule root never compiled: a node compiled it, through an
    // activator a commitment derived.
    for s in sites.iter().filter(|s| s.kind == SiteKind::Activator) {
        if node.refuted_by.contains(&s.rule_text) && !root.has_rule(&s.rule_text) {
            causes.insert(Cause::ActivatorRefutes);
            named.insert(s.text.clone());
        }
    }
    if !named.is_empty() {
        return finish(causes, named);
    }
    // S1e2.1b.1's rule: the `absent` sites over a premise's relation, and of
    // those, the ones whose rule refuted the node, where there are any.
    let labels: BTreeSet<String> = node
        .absent_premises
        .iter()
        .map(|p| absent_label(p))
        .collect();
    let over: Vec<&SiteText> = sites
        .iter()
        .filter(|s| s.kind == SiteKind::Absent)
        .filter(|s| {
            labels.contains(&s.label)
                || (s.label == "(not ?)" && labels.iter().any(|x| x.starts_with("(not ")))
        })
        .collect();
    let refuting: Vec<&&SiteText> = over
        .iter()
        .filter(|s| node.refuted_by.contains(&s.rule_text))
        .collect();
    if !refuting.is_empty() {
        causes.insert(Cause::AbsentRefutes);
        named.extend(refuting.iter().map(|s| s.text.clone()));
    } else if !over.is_empty() {
        causes.insert(Cause::Via);
        named.extend(over.iter().map(|s| s.text.clone()));
    }
    finish(causes, named)
}

/// An owing node `at`, by the debt that is new against `below`: each instance
/// `at`'s fixpoint owes and `below`'s does not.
pub fn owing(
    nodes: &mut region::Nodes<'_>,
    ast: &Ast,
    sites: &[SiteText],
    root: &RootPlans,
    at: &[FactId],
    below: &[FactId],
) -> Attributed {
    nodes
        .with_fixpoints(&[below.to_vec(), at.to_vec()], |kbs, terms| {
            let owed = |kb: &Kb, terms: &mut Terms| -> Vec<Owed> {
                obligations::tally(kb, terms, ast, &SharedMemo::default(), &mut Events::off())
                    .map(|o| o.instances().to_vec())
                    .unwrap_or_default()
            };
            let lo = owed(&kbs[0], terms);
            let hi = owed(&kbs[1], terms);
            // The rules that derived a fact `below` holds and `at` does not.
            let lost: BTreeSet<String> = kbs[0]
                .facts()
                .filter(|&f| !kbs[1].contains(f))
                .filter_map(|f| kbs[0].primary(f))
                .filter_map(|p| terms.provs.get(p).rule)
                .map(|r| terms.sym(r).to_string())
                .collect();
            let mut causes = BTreeSet::new();
            let mut named = BTreeSet::new();
            for o in hi.iter().filter(|o| !lo.contains(o)) {
                let rule = terms.sym(o.rule).to_string();
                let act: Vec<String> = o
                    .activator
                    .iter()
                    .map(|&a| terms.sym(a).to_string())
                    .collect();
                let triggers: Vec<&SiteText> = sites
                    .iter()
                    .filter(|s| s.kind == SiteKind::ObligationTrigger)
                    .filter(|s| s.rule == o.rule && s.activator == o.activator)
                    .collect();
                if !triggers.is_empty() {
                    causes.insert(Cause::Trigger);
                    named.extend(triggers.iter().map(|s| s.text.clone()));
                    continue;
                }
                if !root.has(&rule, &act) {
                    let acts: Vec<&SiteText> = sites
                        .iter()
                        .filter(|s| s.kind == SiteKind::Activator && s.rule_text == rule)
                        .collect();
                    if !acts.is_empty() {
                        causes.insert(Cause::ActivatorObligation);
                        named.extend(acts.iter().map(|s| s.text.clone()));
                        continue;
                    }
                }
                let takers: Vec<&SiteText> = sites
                    .iter()
                    .filter(|s| s.kind == SiteKind::Absent && lost.contains(&s.rule_text))
                    .collect();
                if !takers.is_empty() {
                    causes.insert(Cause::AbsentWitness);
                    named.extend(takers.iter().map(|s| s.text.clone()));
                }
            }
            finish(causes, named)
        })
        .unwrap_or_default()
}

/// The candidates of `mask`, in bit order.
pub fn facts_of(r: &Region, mask: u64) -> Vec<FactId> {
    (0..r.n())
        .filter(|b| mask >> b & 1 == 1)
        .map(|b| r.candidates[b].fact)
        .collect()
}

/// The largest proper subset of `mask` in R, the first met in a descending
/// walk of its subsets: the node a hole's debt is new against.
pub fn r_below(r: &Region, mask: u64) -> Option<u64> {
    let mut best: Option<u64> = None;
    let mut sub = mask;
    loop {
        sub = sub.wrapping_sub(1) & mask;
        if sub == mask {
            break;
        }
        if r.in_r(sub) && best.is_none_or(|b| sub.count_ones() > b.count_ones()) {
            best = Some(sub);
        }
        if sub == 0 {
            break;
        }
    }
    best
}

/// Any node outside R: a dead one by its refutation, an owing one by its debt
/// against `below` (for a hole, [`r_below`]).
pub fn node(
    r: &Region,
    nodes: &mut region::Nodes<'_>,
    ast: &Ast,
    sites: &[SiteText],
    root: &RootPlans,
    mask: u64,
    below: Option<u64>,
) -> Attributed {
    let status = r.status[mask as usize];
    if status & region::DEAD != 0 {
        return dead(sites, root, &r.nodes[mask as usize]);
    }
    if status & region::OWING != 0 {
        let below = below.map(|b| facts_of(r, b)).unwrap_or_default();
        return owing(nodes, ast, sites, root, &facts_of(r, mask), &below);
    }
    Attributed::default()
}
