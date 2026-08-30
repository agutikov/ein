//! Static NAF dependency map — advisory only.
//!
//! Which `(absent …)` guards watch a **rule-derived** relation, which watch a
//! **declared-only** one whose extension the puzzle fixes, and — since M1e
//! S1e.2.3 — which watch one the **hypothesis generator can still extend**
//! while concluding a refutation from its absence.
//!
//! The three questions share one scan, deliberately: they are the same walk
//! over the same guards, differing only in what the watched relation is
//! classified against, and two passes would be two places for the
//! classification to drift ([`AR-M1`]).
//!
//! [`AR-M1`]: ../../../../plans/m1e_review_processing/review/architecture/medium.md
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
//!
//! ## The second question — a refutation resting on an `absent` (M1e S1e.2.3)
//!
//! [`Q-M1e.9`] reproduced, 2026-08-28: **`dead` is not upward-closed under
//! `absent`**. A rule
//!
//! ```lisp
//! (rule bad () :match (and (p ?x) (absent (q ?x))) :assert (false))
//! ```
//!
//! makes `{(p A)}` dead and `{(p A), (q A)}` alive, and three shipped
//! mechanisms read the refuted premise `X ⊆ Y ∧ dead(X) ⇒ dead(Y)` — the
//! lookahead kill cache, the singleton writeback, and the width-1 no-good
//! clause. Five of six shipped configurations answer the twenty-line probe
//! **wrongly**, all of them reporting `exhausted = true`.
//!
//! The condition is a conjunction of two things this module already sees, plus
//! one the caller supplies:
//!
//! 1. the plan's `:match` carries an `(absent …)` — the same guards the
//!    stratification question walks;
//! 2. its `:assert` concludes `(false)` or a `(not …)` — a *refutation*, as
//!    against a derivation;
//! 3. a watched relation is **hypothesis-eligible for this program**, i.e.
//!    this program's generator can still propose a fact of it
//!    ([`crate::hypgen::eligible_relations`]).
//!
//! (3) is what makes it a program property rather than a rule property, and
//! it is why the whole `std.slots` prune / endpoint / adjacency family is
//! **safe**: their `absent` reads the position structure `?S`, which no
//! generator proposes. The one stdlib rule with the exposed shape is
//! `std.algebra`'s `connex`, whose `absent` reads the subject relation `?R`
//! itself.
//!
//! The **containment** is a warning, not a refusal
//! ([D4](../../../../plans/m1e_review_processing/p1e.1_open_questions/s1e.1.1_search_soundness_probes/d4_q_m1e9_upward_closure.md)
//! option B): refusing at load would refuse `connex` before anyone has decided
//! whether that rule should be rewritten, which is
//! [S1f.10.8](../../../../plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md)'s.
//! Promoting warn → refuse afterwards is a one-line change here.
//!
//! [`Q-M1e.9`]: ../../../../plans/m1e_review_processing/open_questions.md

use ein_core::{Symbol, Terms};
use rustc_hash::FxHashSet;

use crate::compile::{asserted_relation, naf_relation_refs, negated_relation};
use crate::engine::Engine;
use crate::plan::{Plan, Slot};

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
    /// M1e S1e.2.3 — [`Q-M1e.9`]'s exposure, or `None`.
    ///
    /// `Some` **only** when both halves hold: the `:assert` refutes, and at
    /// least one watched relation is one the program's generator can still
    /// propose. The field's presence is the finding, so a caller never has to
    /// re-derive the conjunction.
    ///
    /// [`Q-M1e.9`]: ../../../../plans/m1e_review_processing/open_questions.md
    pub refutation: Option<Refutation>,
}

/// A rule that concludes a refutation from an absence the search can still
/// fill — the shape `dead`'s upward-closure premise fails on.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Refutation {
    /// What the `:assert` concludes: `"(false)"` or `"(not …)"`.
    pub concludes: &'static str,
    /// The watched relations the generator can still extend, sorted labels —
    /// a subset of `derived ∪ declared_only`, since it is the same walk.
    pub watching: Box<[String]>,
}

/// Does a plan's `:assert` conclude a **refutation** rather than a derivation?
///
/// Every conclusion is inspected, not only the first: `(and …)` multi-assert
/// (A13) means a rule can derive a fact *and* refute, and the refuting arm is
/// what matters here. `asserted_relation` / `negated_relation` read
/// `assert_template()` alone because they reproduce ein.py's `S1.7.4` map;
/// this is a new question and has no parity to keep.
fn refuting_conclusion(plan: &Plan, terms: &Terms) -> Option<&'static str> {
    let mut found = None;
    for slot in plan.asserts.iter() {
        let Slot::Nested { rel, .. } = slot else {
            continue;
        };
        if *rel == terms.kernel.r#false {
            // Direct ⊥ outranks a negative: it is the stronger claim, and the
            // message's advice differs.
            return Some("(false)");
        }
        if *rel == terms.kernel.not {
            found = Some("(not …)");
        }
    }
    found
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
///
/// `eligible` is the program's hypothesis-eligible relation set
/// ([`crate::hypgen::eligible_relations`]) and drives [`NafDep::refutation`]
/// alone; pass an empty set to ask only the stratification question, which is
/// what a caller with no KB in hand does.
pub fn compute_naf_map(
    engine: &Engine,
    terms: &Terms,
    eligible: &FxHashSet<Symbol>,
) -> Vec<NafDep> {
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
        // S1e.2.3's third bucket, filled from the *same* walk. A negated watch
        // `(absent (not (R …)))` is deliberately **not** exposure: what the
        // generator proposes is `R`, and adding an `R` cannot make a missing
        // `(not (R …))` appear. Only the positive read is a hazard.
        let mut exposed = FxHashSet::default();
        let refutes = refuting_conclusion(plan, terms);
        for (rel, neg) in refs {
            let pool = if neg { &negated } else { &producible };
            let bucket = if pool.contains(&rel) {
                &mut derived
            } else {
                &mut declared
            };
            bucket.insert(label(terms, rel, neg));
            if refutes.is_some() && !neg && eligible.contains(&rel) {
                exposed.insert(label(terms, rel, neg));
            }
        }
        deps.push(NafDep {
            rule: plan.rule,
            activator: plan.activator_args.clone(),
            derived: sorted(derived),
            declared_only: sorted(declared),
            refutation: match (refutes, exposed.is_empty()) {
                (Some(concludes), false) => Some(Refutation {
                    concludes,
                    watching: sorted(exposed),
                }),
                _ => None,
            },
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

/// One advisory line, and which kind it is.
///
/// `category` is the `warn` event's, and it is what tells the two apart in a
/// stream: `DerivedNafWarning` is ein.py's stratification advice,
/// `RefutationUnderAbsentWarning` is M1e S1e.2.3's soundness one.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NafWarning {
    pub category: &'static str,
    pub text: String,
}

pub const DERIVED_NAF: &str = "DerivedNafWarning";
pub const REFUTATION_UNDER_ABSENT: &str = "RefutationUnderAbsentWarning";

fn activator_suffix(terms: &Terms, activator: &[Symbol]) -> String {
    if activator.is_empty() {
        return String::new();
    }
    let names: Vec<&str> = activator.iter().map(|&a| terms.sym(a)).collect();
    format!(" [activator {}]", names.join(" "))
}

/// Every advisory line the NAF map yields, in map order.
///
/// **One walk, two questions** — the stratification one ein.py shipped and the
/// refutation-exposure one M1e S1e.2.3 added. There is no second switch:
/// `eligible` **is** the switch, because a refutation line needs a relation
/// the generator can propose and an empty set has none. So
/// [`derived_naf_warnings`] is this function over an empty set, and a caller
/// that wants both computes the set.
///
/// A "warning" in ein.py is a `DerivedNafWarning` raised through the
/// `warnings` module; here it is a string the caller decides what to do with —
/// the CLI turns it into a `warn` event and a stderr line.
pub fn naf_warnings(
    engine: &Engine,
    terms: &Terms,
    eligible: &FxHashSet<Symbol>,
) -> Vec<NafWarning> {
    let mut out = Vec::new();
    for d in compute_naf_map(engine, terms, eligible) {
        let act = activator_suffix(terms, &d.activator);
        let rule = ein_core::pyrepr::repr_str(terms.sym(d.rule));
        if !d.derived.is_empty() {
            out.push(NafWarning {
                category: DERIVED_NAF,
                text: format!(
                    "rule {rule}{act}: (absent …) watches rule-derived relation(s) {} \
                     — the rule set may not be stratified, in which case the engine \
                     reports one model rather than the several that exist. Sound \
                     either way (the guard is evaluated on the closure/world \
                     boundary, S1.21.8). See S1.7.4.",
                    d.derived.join(", ")
                ),
            });
        }
        if let Some(r) = d.refutation {
            out.push(NafWarning {
                category: REFUTATION_UNDER_ABSENT,
                text: format!(
                    "rule {rule}{act}: concludes {} from (absent …) over {}, which this \
                     program's hypothesis generator can still propose — so a state the \
                     search has not finished extending is refuted, and a commitment that \
                     would have discharged the guard is never reached (`dead` is not \
                     upward-closed under `absent`; Q-M1e.9). Write the constraint over a \
                     STORED negative, as std.algebra's `total` does — demand \
                     `(not (R ?a ?b))` for every candidate rather than concluding from \
                     its absence — or, if this is a requirement and not a refutation, \
                     state it as `(open ?R)` and let the verdict report what the state owes.",
                    r.concludes,
                    r.watching.join(", ")
                ),
            });
        }
    }
    out
}

/// The stratification lines alone — ein.py's `DerivedNafWarning` set.
///
/// Kept as a name because two pages and a test cite it. It is
/// [`naf_warnings`] over an **empty** eligible set, which is the same scan
/// asked one question instead of two, not a second scan.
pub fn derived_naf_warnings(engine: &Engine, terms: &Terms) -> Vec<String> {
    naf_warnings(engine, terms, &FxHashSet::default())
        .into_iter()
        .map(|w| w.text)
        .collect()
}
