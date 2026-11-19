//! Contradiction detection — the two shapes, and the incremental check.
//!
//! 1. **ended** — a rule that states `:ein inconsistent` fired
//!    ([`ein_core::Kb::ended`], Q-M1f.399): the record of its application,
//!    which is no fact. A state that holds `(false)` is of this shape: the
//!    fact is a proposition like any other, and what ends the state is the
//!    rule every program gets, which matches it and states the key
//!    (Q-M1f.551). The kernel reads `(false)` by its name nowhere.
//! 2. **pair** — an `(X, (not X))` pair. A rule asserted `(not X)` and `X` is
//!    also in the KB. The kernel's own check, of every fact, with no rule.
//!
//! Each encodes a **branch failure** under the append-only KB: there is no
//! way to retract a fact or a firing, so the only resolution is for whatever
//! caused the conflict — typically a hypothesis fork — to be unwound.
//!
//! **How a fact got into the KB is irrelevant.** A KB holding both `X` and
//! `(not X)` is inconsistent whether `X` is a stated clue, a background
//! assumption or a derivation, so every such pair is reported, whichever
//! knowledge layers its facts sit in: skipping a pair across layers would
//! accept a flatly inconsistent puzzle in silence.
//!
//! The lookup is two bit tests
//! ([03-design/06-saturation.md § 6.1](../../../../docs/ein/ein.rs/03-design/06-saturation.md#61-contradiction)):
//! is `X` negated, and is `X` in the KB. The result **order** is fixed — the
//! firings that ended the state in the order they fired, then pairs in extent
//! order — because it reaches the unsat core and the trace.

use ein_core::{FactId, Kb, ProvId, Terms};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Ended,
    Pair,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Ended => "ended",
            Kind::Pair => "pair",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Contradiction {
    /// A rule that states `:ein inconsistent` fired: the record of the
    /// application ([`ein_core::Kb::ended`]).
    Ended(ProvId),
    /// The fact `X` beside its stored negation `(not X)`.
    Pair { positive: FactId, negative: FactId },
}

impl Contradiction {
    pub fn kind(&self) -> Kind {
        match self {
            Contradiction::Ended(_) => Kind::Ended,
            Contradiction::Pair { .. } => Kind::Pair,
        }
    }

    /// The fact that names the contradiction: for a pair the positive `X`,
    /// and for a firing that ended the state the first premise it matched,
    /// the `(false)` of a state that holds one, `None` where it matched no
    /// fact.
    /// It is the core search's tie-break and
    /// [`crate::explain::Explanation::target`].
    ///
    /// It is **not** what the core is the frontier of: that is
    /// [`Contradiction::sides`]. A core of this fact alone would leave out
    /// the derivation of a pair's `(not X)`, where a commitment typically is.
    pub fn witness(&self, terms: &Terms) -> Option<FactId> {
        match *self {
            Contradiction::Ended(p) => terms.provs.get(p).premises.first().copied(),
            Contradiction::Pair { positive, .. } => Some(positive),
        }
    }

    /// The facts the contradiction is made of, which the core search
    /// explains together ([`crate::explain::contradiction_frontier`]): a
    /// pair's `X` and its `(not X)`, the witness first, or the premises of
    /// the firing that ended the state, in the order its plan matched them.
    pub fn sides(&self, terms: &Terms) -> Vec<FactId> {
        match *self {
            Contradiction::Ended(p) => terms.provs.get(p).premises.to_vec(),
            Contradiction::Pair { positive, negative } => vec![positive, negative],
        }
    }

    /// The firing that ended the state, where the contradiction is one.
    pub fn ended(&self) -> Option<ProvId> {
        match *self {
            Contradiction::Ended(p) => Some(p),
            Contradiction::Pair { .. } => None,
        }
    }

    /// **The last steps of the contradiction's first-recorded derivation**:
    /// the primary of each fact it is made of, and for a firing that ended
    /// the state the firing itself, or, where its rule is of the module
    /// every program gets ([`ein_core::Rule::implicit`]), the primaries of
    /// the premises it matched: an explanation names the rules a program
    /// wrote or imported, and reads through such a rule to what its firing
    /// rests on.
    pub fn first_steps(&self, kb: &Kb, terms: &Terms) -> Vec<ProvId> {
        match *self {
            Contradiction::Ended(p) if !ended_by_an_implicit_rule(kb, terms, p) => vec![p],
            _ => self
                .sides(terms)
                .into_iter()
                .filter_map(|f| kb.primary(f))
                .collect(),
        }
    }
}

/// Whether the firing `prov` records is of a rule of the module every
/// program gets ([`ein_core::Rule::implicit`]), which an explanation reads
/// through.
pub fn ended_by_an_implicit_rule(kb: &Kb, terms: &Terms, prov: ProvId) -> bool {
    terms
        .provs
        .get(prov)
        .rule
        .and_then(|r| kb.program().rules.get(r))
        .is_some_and(|r| r.implicit)
}

/// Every contradiction in the KB: the firings that ended it, then the pairs.
pub fn detect(kb: &Kb, terms: &Terms) -> Vec<Contradiction> {
    let mut out: Vec<Contradiction> = kb
        .ended()
        .iter()
        .map(|&p| Contradiction::Ended(p))
        .collect();
    for negative in kb.facts_of(terms.kernel.not) {
        // Q40 / R9's widening: the inner of a `(not …)` is expected to be a
        // fact. A name leaves no positive proposition to match
        // against — the loader and matcher never produce that shape, but it is
        // tolerated rather than asserted.
        let Some(inner) = terms.facts.args(negative).first().and_then(|v| v.as_fact()) else {
            continue;
        };
        if kb.contains(inner) {
            out.push(Contradiction::Pair {
                positive: inner,
                negative,
            });
        }
    }
    out
}

/// Short-circuit yes/no — stops on the first contradiction found.
pub fn has_contradiction(kb: &Kb, terms: &Terms) -> bool {
    if kb.is_inconsistent() {
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

/// True iff `fact`, just written to `kb`, makes a pair in it.
///
/// The **incremental** dual of [`detect`]'s facts: the scan asks "does this
/// KB hold a contradiction?", this asks "did *this fact* create one?". On a
/// KB that was consistent before `fact` landed the two agree exactly, and
/// that equivalence is what lets fork saturation fail fast: the KB is
/// append-only, so a contradiction can only be *created* by an insertion and
/// can never be retracted, so checking each derived fact as it lands finds
/// every death a post-fixpoint scan would, and earlier: a dying fork stops at
/// the firing that kills it, not at its fixpoint. A firing that ends a state
/// writes no fact, and the saturator reads it off the firing
/// ([`crate::firing::Firing::ends`]).
pub fn contradicts(kb: &Kb, terms: &Terms, fact: FactId) -> bool {
    let rel = terms.facts.rel(fact);
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

    /// A held `(false)` is a fact like any other: the kernel reads no
    /// contradiction off it, and only a pair is one where no rule has fired.
    /// What ends a state that holds it is the firing of the rule every
    /// program gets, which `ein_infer::saturator`'s tests and
    /// `tests/search_semantics.rs` hold.
    #[test]
    fn a_held_false_is_no_contradiction_by_its_name() {
        let (terms, kb) = kb_of(
            "(relation p Thing :why \"p holds of {?3}\")\n(p one)\n(not (p one))\n(false)\n(p two)\n(not (p three))\n",
        );
        let found = detect(&kb, &terms);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].kind(), Kind::Pair);
        // `(not (p three))` has no positive to pair with, and `(p two)` has no
        // negation — neither is a contradiction on its own.
        assert!(has_contradiction(&kb, &terms));
        let (terms, kb) = kb_of("(relation p Thing :why \"p holds of {?3}\")\n(p one)\n(false)\n");
        assert!(detect(&kb, &terms).is_empty());
        assert!(!has_contradiction(&kb, &terms));
        for f in kb.facts() {
            assert!(!contradicts(&kb, &terms, f));
        }
    }

    #[test]
    fn a_pairs_sides_are_both_its_facts() {
        let (terms, kb) =
            kb_of("(relation p Thing :why \"p holds of {?3}\")\n(p one)\n(not (p one))\n");
        let found = detect(&kb, &terms);
        let pair = found[0];
        let Contradiction::Pair { positive, negative } = pair else {
            panic!("{pair:?}")
        };
        assert_eq!(
            pair.sides(&terms),
            vec![positive, negative],
            "the witness first, then the negative"
        );
        assert_eq!(Some(pair.sides(&terms)[0]), pair.witness(&terms));
    }

    #[test]
    fn the_incremental_check_agrees_with_the_scan() {
        let (terms, kb) =
            kb_of("(relation p Thing :why \"p holds of {?3}\")\n(p one)\n(not (p one))\n(p two)\n");
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
        let (terms, kb) =
            kb_of("(relation p Thing :why \"p holds of {?3}\")\n(p one)\n(not (p two))\n");
        assert!(detect(&kb, &terms).is_empty());
        assert!(!has_contradiction(&kb, &terms));
        for f in kb.facts() {
            assert!(!contradicts(&kb, &terms, f));
        }
    }
}
