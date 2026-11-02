//! What a refutation rests on — M1e2
//! [S1e2.1b.4](../../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.4_refutation_resting_on_absent.md).
//!
//! A dead node's **taint** is the `absent` premises its death rests on, read
//! off its fixpoint's provenance. Option (b) of F21's D6 would let a route
//! learn from a dead node whose refutation has no `absent` premise over `D`,
//! and that is sound only if this reading misses nothing. So the walk here is
//! wider than the oracle's own `region::Node::absent_premises`, which follows
//! the first contradiction's witness alone:
//!
//! - **every contradiction** of the fixpoint, and **both sides** of a pair
//!   (`X` beside `(not X)`), each followed through every fact's primary
//!   derivation, root's facts included;
//! - with [`Walk::activators`], also the **activator** of every plan a fact was
//!   derived by: a derivation does not record the activator fact its plan was
//!   compiled from, and a node compiles a plan root never had when a
//!   commitment derives one (`written.rs`'s `ACTIVATOR_UNDER_ABSENT`).
//!
//! Each premise is classified: its relation **static** (outside `D`) or in
//! `D`, read **directly** by the rule that concluded the contradiction or
//! **via** a fact further down, recorded on a fact **root** holds or not, and
//! reached **through an activator** or not. A node is **untainted** when some
//! contradiction of it has no premise in `D`.
//!
//! Example code, read by `taint_census` and by `ein-infer/tests/region.rs`. No
//! route reads it.

use std::collections::BTreeSet;

use ein_core::{FactId, Kb, NafArg, NafRef, Terms, Value};
use rustc_hash::FxHashSet;

/// How far the walk reaches.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Walk {
    /// Follow the activator of each plan a fact was derived by.
    pub activators: bool,
}

/// One `absent` premise of a refutation.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Premise {
    /// `(absent (q A))`.
    pub text: String,
    /// Its relation, as `monotone::Rel::label` writes it: `q`, `(not r1)`.
    pub label: String,
    /// The relation is in `D`.
    pub dynamic: bool,
    /// Recorded on a fact the contradiction itself is made of, the refuting
    /// rule's own read: S1e2.4.1's `direct`. Else on a fact further down: `via`.
    pub direct: bool,
    /// Recorded on a fact root's fixpoint holds.
    pub at_root: bool,
    /// Reached only through a plan's activator.
    pub through_activator: bool,
}

/// One contradiction's taint.
#[derive(Clone, Debug, Default)]
pub struct Contradiction {
    pub premises: Vec<Premise>,
}

impl Contradiction {
    /// No premise over `D`: what option (b) would learn from.
    pub fn untainted(&self) -> bool {
        !self.premises.iter().any(|p| p.dynamic)
    }
}

/// A dead node's taint: one entry per contradiction of its fixpoint.
#[derive(Clone, Debug, Default)]
pub struct Taint {
    pub contradictions: Vec<Contradiction>,
}

impl Taint {
    /// Some contradiction rests on no premise over `D`.
    pub fn untainted(&self) -> bool {
        self.contradictions.iter().any(Contradiction::untainted)
    }

    /// Every premise, over every contradiction, sorted and distinct.
    pub fn premises(&self) -> Vec<&Premise> {
        let mut v: Vec<&Premise> = self
            .contradictions
            .iter()
            .flat_map(|c| c.premises.iter())
            .collect();
        v.sort();
        v.dedup();
        v
    }
}

/// `D` as labels, for a premise's relation to be looked up in.
pub struct Dynamic {
    labels: BTreeSet<String>,
    any_negative: bool,
}

impl Dynamic {
    pub fn of(labels: impl IntoIterator<Item = String>) -> Dynamic {
        let labels: BTreeSet<String> = labels.into_iter().collect();
        let any_negative = labels.contains("(not ?)");
        Dynamic {
            labels,
            any_negative,
        }
    }

    pub fn contains(&self, label: &str) -> bool {
        self.labels.contains(label)
            || (self.any_negative && label.starts_with("(not "))
            || (label == "(not ?)" && self.labels.iter().any(|l| l.starts_with("(not ")))
    }
}

/// A premise's relation: `q`, or `(not q)` for a stored negative.
pub fn naf_label(terms: &Terms, r: &NafRef) -> String {
    if r.rel == terms.kernel.not {
        return match r.args.first() {
            Some(NafArg::Nested { rel, .. }) => format!("(not {})", terms.sym(*rel)),
            _ => "(not ?)".to_string(),
        };
    }
    terms.sym(r.rel).to_string()
}

fn naf_text(terms: &Terms, rel: ein_core::Symbol, args: &[NafArg]) -> String {
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

/// The activator facts in `kb` of the plan `fact`'s primary derivation ran:
/// for a rule with parameters, its applications of the right arity whose
/// arguments are the derivation's bindings of the parameters, or all of them
/// where the bindings do not name the parameters.
fn activators_of(kb: &Kb, terms: &Terms, fact: FactId) -> Vec<FactId> {
    let Some(p) = kb.primary(fact) else {
        return Vec::new();
    };
    let prov = terms.provs.get(p);
    let Some(rule) = prov.rule else {
        return Vec::new();
    };
    let program = kb.program();
    let Some(r) = program
        .rules
        .get(rule)
        .or_else(|| program.obligations.get(rule))
    else {
        return Vec::new();
    };
    if r.params.is_empty() {
        return Vec::new();
    }
    let bound: Vec<Option<Value>> = r
        .params
        .iter()
        .map(|&v| prov.bindings.iter().find(|(s, _)| *s == v).map(|(_, x)| *x))
        .collect();
    kb.rule_apps_by_rule(rule)
        .filter(|&f| terms.facts.arity(f) == r.params.len())
        .filter(|&f| {
            let (_, args) = terms.facts.get(f);
            bound
                .iter()
                .zip(args)
                .all(|(b, a)| b.is_none_or(|b| b == *a))
        })
        .collect()
}

/// Every contradiction of `kb`, a dead node's fixpoint, walked.
pub fn of(kb: &Kb, terms: &Terms, root: &FxHashSet<FactId>, d: &Dynamic, walk: Walk) -> Taint {
    let mut out = Taint::default();
    for c in ein_infer::contradiction::detect(kb, terms) {
        let seeds: Vec<FactId> = [c.positive, Some(c.negative)]
            .into_iter()
            .flatten()
            .collect();
        let direct: FxHashSet<FactId> = seeds.iter().copied().collect();
        let mut seen: FxHashSet<FactId> = FxHashSet::default();
        // (fact, reached through an activator)
        let mut stack: Vec<(FactId, bool)> = seeds.iter().map(|&f| (f, false)).collect();
        let mut premises: BTreeSet<Premise> = BTreeSet::new();
        while let Some((f, via_act)) = stack.pop() {
            if !seen.insert(f) {
                continue;
            }
            let Some(p) = kb.primary(f) else { continue };
            let prov = terms.provs.get(p);
            for r in prov.absent.iter() {
                let label = naf_label(terms, r);
                premises.insert(Premise {
                    text: format!("(absent {})", naf_text(terms, r.rel, &r.args)),
                    dynamic: d.contains(&label),
                    label,
                    direct: direct.contains(&f),
                    at_root: root.contains(&f),
                    through_activator: via_act,
                });
            }
            stack.extend(prov.premises.iter().map(|&x| (x, via_act)));
            if walk.activators {
                stack.extend(activators_of(kb, terms, f).into_iter().map(|x| (x, true)));
            }
        }
        out.contradictions.push(Contradiction {
            premises: premises.into_iter().collect(),
        });
    }
    out
}
