//! Static NAF dependency map — advisory only.
//!
//! Which `(absent …)` guards watch a **rule-derived** relation, and which
//! watch a **declared-only** one whose extension the puzzle fixes.
//!
//! **What this means changed with S1.21.8.** It used to be a soundness signal:
//! the fire-time NAF re-check was what made a derived-NAF rule sound, and this
//! map told the author which rules leaned on it. That re-check is gone —
//! `(absent …)` is evaluated on the closure/world boundary against a positive
//! fixpoint — so a derived-NAF rule is sound whatever the priorities say.
//!
//! What the distinction still buys is **stratification**. NAF over a derived
//! relation is exactly the shape that can make a rule set non-stratified, and
//! that is the remaining hazard: on a genuinely unstratifiable program
//! (`p ← absent q; q ← absent p`) the engine still produces *an* answer,
//! chosen by boundary-admission order, rather than reporting that two models
//! exist. Declared-only NAF cannot do that.
//!
//! **Completeness needs a saturated cache.** Most NAF-bearing rules in the
//! Zebra family (`adjacent-via-*`, `typecheck-arg-*`, the elimination and
//! totality rules) are activated by *derived* facts that do not exist at load,
//! so their plan is compiled only once the saturator's enqueue pass has
//! refreshed the cache. Pass the engine of a saturator that has run.

use ein_core::{Symbol, Terms};
use rustc_hash::FxHashSet;

use crate::compile::{asserted_relation, naf_relation_refs, negated_relation};
use crate::engine::Engine;

/// One `(rule, activator)` pair's NAF dependency classification.
///
/// `derived` and `declared_only` are sorted **labels**, not names: a negated
/// watch `(absent (not (R …)))` is labelled `"(not R)"` to keep it distinct
/// from the positive `"R"`. Both, and the record order, are stable, so a
/// golden diffs cleanly.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NafDep {
    pub rule: Symbol,
    pub activator: Box<[Symbol]>,
    pub derived: Box<[String]>,
    pub declared_only: Box<[String]>,
}

fn label(terms: &Terms, rel: Symbol, negated: bool) -> String {
    if negated {
        format!("(not {})", terms.sym(rel))
    } else {
        terms.sym(rel).to_string()
    }
}

/// The static map over a compile cache: one record per plan carrying at least
/// one `(absent …)` guard.
///
/// Each watched relation — recursively, both nesting levels — is classified
/// against the *same* cache, so the producible set reflects exactly the
/// activators that actually exist. Records are sorted by
/// `(rule_name, activator_args)`, **by text**: the ids the interner assigned
/// are not the order ein.py sorts strings in.
pub fn compute_naf_map(engine: &Engine, terms: &Terms) -> Vec<NafDep> {
    let producible: FxHashSet<Symbol> = (0..engine.len())
        .filter_map(|i| asserted_relation(engine.plan(i), terms))
        .collect();
    // Scope B — `forall` / `total` / domain-elimination desugar to
    // `(absent (… (absent (not (R …)))))`. The literal head there is `not`,
    // but the watched fact is the *derived* `(not (R …))`, so a negated ref is
    // classified against the rules that assert one.
    let negated: FxHashSet<Symbol> = (0..engine.len())
        .filter_map(|i| negated_relation(engine.plan(i), terms))
        .collect();

    let mut deps: Vec<NafDep> = Vec::new();
    for i in 0..engine.len() {
        let plan = engine.plan(i);
        let refs = naf_relation_refs(plan, terms);
        if refs.is_empty() {
            continue;
        }
        // ein.py buckets into `set`s and sorts, so a relation watched twice is
        // one entry — and a relation watched both ways is two.
        let (mut derived, mut declared) = (FxHashSet::default(), FxHashSet::default());
        for (rel, neg) in refs {
            let pool = if neg { &negated } else { &producible };
            let bucket = if pool.contains(&rel) {
                &mut derived
            } else {
                &mut declared
            };
            bucket.insert(label(terms, rel, neg));
        }
        deps.push(NafDep {
            rule: plan.rule,
            activator: plan.activator_args.clone(),
            derived: sorted(derived),
            declared_only: sorted(declared),
        });
    }
    deps.sort_by(|a, b| {
        terms
            .syms
            .cmp_text(a.rule, b.rule)
            .then_with(|| cmp_args(terms, &a.activator, &b.activator))
    });
    deps
}

fn sorted(set: FxHashSet<String>) -> Box<[String]> {
    // ein.py buckets into a `set` and hands `tuple(sorted(...))` out, so the
    // set is a dedup and the sort is what anyone reads. Labels are distinct
    // strings, so the sort has no ties for the iteration order to break.
    // determinism-ok: sorted on the next line, before it can reach a caller.
    let mut v: Vec<String> = set.into_iter().collect();
    v.sort_unstable();
    v.into_boxed_slice()
}

/// `tuple[str, ...]` ordering: element-wise by text, and a prefix sorts first.
fn cmp_args(terms: &Terms, a: &[Symbol], b: &[Symbol]) -> std::cmp::Ordering {
    for (x, y) in a.iter().zip(b.iter()) {
        let o = terms.syms.cmp_text(*x, *y);
        if o != std::cmp::Ordering::Equal {
            return o;
        }
    }
    a.len().cmp(&b.len())
}

/// One warning line per `(rule, activator)` with a derived-NAF dependency,
/// with ein.py's text.
///
/// A "warning" in ein.py is a `DerivedNafWarning` raised through the
/// `warnings` module; here it is a string the caller decides what to do with —
/// the CLI turns it into a stderr line. Gated by `warn_derived_naf`, which is
/// off by default: the suite runs under `filterwarnings=["error"]`, and while
/// the guard is sound either way the warning is pure advice.
pub fn derived_naf_warnings(engine: &Engine, terms: &Terms) -> Vec<String> {
    compute_naf_map(engine, terms)
        .into_iter()
        .filter(|d| !d.derived.is_empty())
        .map(|d| {
            let act = if d.activator.is_empty() {
                String::new()
            } else {
                let names: Vec<&str> = d.activator.iter().map(|&a| terms.sym(a)).collect();
                format!(" [activator {}]", names.join(" "))
            };
            format!(
                "rule {}{act}: (absent …) watches rule-derived relation(s) {} \
                 — the rule set may not be stratified, in which case the engine \
                 reports one model rather than the several that exist. Sound \
                 either way (the guard is evaluated on the closure/world \
                 boundary, S1.21.8). See S1.7.4.",
                ein_core::pyrepr::repr_str(terms.sym(d.rule)),
                d.derived.join(", ")
            )
        })
        .collect()
}
