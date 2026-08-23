//! Contradiction detection — the two shapes, and the incremental check.
//!
//! 1. **pair** — an `(X, (not X))` pair. A rule asserted `(not X)` and `X` is
//!    also in the KB.
//! 2. **direct** — a `(false)` fact: a rule asserted contradiction outright
//!    rather than through the self-negation idiom.
//!
//! Both encode a **branch failure** under M1's append-only KB: there is no way
//! to retract a fact, so the only resolution is for whatever caused the
//! conflict — typically a hypothesis fork — to be unwound.
//!
//! **How a fact got into the KB is irrelevant** (S1.22.1b). A KB holding both
//! `X` and `(not X)` is inconsistent whether `X` is a stated clue, a
//! background assumption or a derivation, so every such pair is reported.
//! Until S1.22.1b the detector skipped pairs whose facts sat in different
//! knowledge layers, and that was a soundness bug: a flatly inconsistent
//! puzzle was accepted in silence.
//!
//! The port's change is the *lookup*, not the algorithm
//! ([design/06](../../../../docs/history/m1a_rust/design/06_saturation.md) §6):
//! ein.py's `(rn, args) in kb._negated_facts` and `_fact_by_id(X)` become two
//! bit tests. The result **order** is preserved — direct ⊥ first, then pairs
//! in extent order — because it reaches the unsat core and the trace.

use ein_core::{FactId, Kb, Terms};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Pair,
    Direct,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Pair => "pair",
            Kind::Direct => "direct",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Contradiction {
    /// The `X` fact, or `None` for a direct ⊥.
    pub positive: Option<FactId>,
    /// The `(not X)` wrapper, or the `(false …)` fact itself.
    pub negative: FactId,
    pub kind: Kind,
}

impl Contradiction {
    /// The fact whose derivation DAG seeds the unsat-core walk.
    ///
    /// For a pair, the positive `X`: its DAG walks back to the premises that
    /// derived it, where the negative is typically a structural rule firing
    /// with a shallow one. For a direct ⊥, the `(false …)` fact, whose DAG
    /// *is* the firing chain of the rule that emitted it.
    pub fn witness(&self) -> FactId {
        self.positive.unwrap_or(self.negative)
    }
}

/// Every contradiction in the KB, direct ⊥ first.
pub fn detect(kb: &Kb, terms: &Terms) -> Vec<Contradiction> {
    let mut out = Vec::new();
    for false_fact in kb.facts_of(terms.kernel.r#false) {
        out.push(Contradiction {
            positive: None,
            negative: false_fact,
            kind: Kind::Direct,
        });
    }
    for negative in kb.facts_of(terms.kernel.not) {
        // Q40 / R9's widening: the inner of a `(not …)` is expected to be a
        // fact. A string or an int leaves no positive proposition to match
        // against — the loader and matcher never produce that shape, but it is
        // tolerated rather than asserted.
        let Some(inner) = terms.facts.args(negative).first().and_then(|v| v.as_fact()) else {
            continue;
        };
        if kb.contains(inner) {
            out.push(Contradiction {
                positive: Some(inner),
                negative,
                kind: Kind::Pair,
            });
        }
    }
    out
}

/// Short-circuit yes/no — stops on the first contradiction found.
pub fn has_contradiction(kb: &Kb, terms: &Terms) -> bool {
    if kb.n_facts_of(terms.kernel.r#false) > 0 {
        return true;
    }
    kb.facts_of(terms.kernel.not).any(|negative| {
        terms
            .facts
            .args(negative)
            .first()
            .and_then(|v| v.as_fact())
            .is_some_and(|inner| kb.contains(inner))
    })
}

/// True iff `fact`, just written to `kb`, makes it inconsistent.
///
/// The **incremental** dual of [`detect`]: the scan asks "does this KB hold a
/// contradiction?", this asks "did *this fact* create one?". On a KB that was
/// consistent before `fact` landed the two agree exactly, and that equivalence
/// is what lets fork saturation fail fast (S1.9.E23): the KB is append-only,
/// so a contradiction can only be *created* by an insertion and can never be
/// retracted, so checking each derived fact as it lands finds every death a
/// post-fixpoint scan would — about 2.5 k firings earlier on a dying zebra2
/// fork, which is ~88 % of its saturation.
pub fn contradicts(kb: &Kb, terms: &Terms, fact: FactId) -> bool {
    let rel = terms.facts.rel(fact);
    if rel == terms.kernel.r#false {
        return true; // direct ⊥
    }
    if rel == terms.kernel.not {
        // `fact` is `(not X)` — dead iff the positive `X` is believed.
        return terms
            .facts
            .args(fact)
            .first()
            .and_then(|v| v.as_fact())
            .is_some_and(|inner| kb.contains(inner));
    }
    // `fact` is positive — dead iff some `(not fact)` is believed.
    kb.is_negated(fact)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_ir::{Ast, from_ir::load, parse};

    fn kb_of(src: &str) -> (Terms, Kb) {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        (terms, kb)
    }

    #[test]
    fn a_pair_and_a_direct_are_both_found_direct_first() {
        let (terms, kb) = kb_of(
            "(relation p Thing)\n(p one)\n(not (p one))\n(false)\n(p two)\n(not (p three))\n",
        );
        let found = detect(&kb, &terms);
        assert_eq!(found.len(), 2, "{found:?}");
        assert_eq!(found[0].kind, Kind::Direct);
        assert_eq!(found[0].positive, None);
        assert_eq!(found[1].kind, Kind::Pair);
        // `(not (p three))` has no positive to pair with, and `(p two)` has no
        // negation — neither is a contradiction on its own.
        assert!(has_contradiction(&kb, &terms));
    }

    #[test]
    fn the_incremental_check_agrees_with_the_scan() {
        let (terms, kb) = kb_of("(relation p Thing)\n(p one)\n(not (p one))\n(p two)\n");
        // Every fact the KB holds, checked both ways. The scan is over the
        // whole KB, so on a consistent-until-now KB the two agree fact by fact.
        let flagged: Vec<bool> = kb.facts().map(|f| contradicts(&kb, &terms, f)).collect();
        assert_eq!(
            flagged.iter().filter(|x| **x).count(),
            2,
            "`(p one)` and `(not (p one))` each create the pair"
        );
        assert!(
            detect(&kb, &terms).len() == 1,
            "the pair is one contradiction"
        );
    }

    #[test]
    fn a_consistent_kb_reports_nothing() {
        let (terms, kb) = kb_of("(relation p Thing)\n(p one)\n(not (p two))\n");
        assert!(detect(&kb, &terms).is_empty());
        assert!(!has_contradiction(&kb, &terms));
        for f in kb.facts() {
            assert!(!contradicts(&kb, &terms, f));
        }
    }
}
