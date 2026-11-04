//! What the universe leaves out — M1e2
//! [S1e2.2.8](../../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.8_uncovered_certified.md),
//! W6, on the user's ruling of 2026-11-03 on
//! [Q-M1e2.3](../../../../plans/m1e2_correctness/open_questions.md#q-m1e23--what-does-exhausted-certify-when-the-rung-leaves-relations-uncovered),
//! (a) *scoped, and the read-out says so*.
//!
//! R is over H, the candidate list the generation ladder builds at root
//! ([`solution_semantics.md` § 1.4](../../../../docs/kernel/inference/solution_semantics.md#14-candidate-and-universe)),
//! so `exhausted` certifies all of R over H and nothing beyond it. Where the
//! rung that built H proposes no fact of a relation the program **can
//! conclude or ask about**, every state of R holds of that relation only what
//! the program states and saturation derives, and the read-out says so: the
//! `outside` row under `universe`, `root.hypgen.outside` in `--json-summary`,
//! a line beside each claim `ein test` prints, and `ein oracle --dump`'s
//! `region.md` and `region.json`.
//!
//! **Which relations.** A declared relation (one with a signature) is named
//! when all three hold:
//!
//! 1. **the program can conclude or ask about it**: a saturation plan root
//!    compiled writes it, or a query's `:goal` reads it. A relation no rule
//!    concludes and no goal asks about holds what the program states, and the
//!    sentence is not about it;
//! 2. **the program does not scope it out itself**: `(__closed__ R)`, a
//!    `:hypothesis-relations` list without it, or `:no-hypothesis` naming it.
//!    That scope is the program's own statement, and the read-out does not
//!    repeat it (W6's `arity3.ein` scopes `is-a` out, and only `r3` is named);
//! 3. **the rung that built H proposes none of its facts**, at relation
//!    grain: the hrules assert no fact of it, no obligation owes it, or, for
//!    the blind enumerator, its arity is 3 or more (`hypgen`'s `fill_slot`
//!    enumerates arity 1 and 2).
//!
//! Measured 2026-11-04 at `ee0b74c4` (T1e2.2.8.1): of the 234 programs of
//! the sweep that reach H, 22 name a relation, 16 of them under the hrules.
//! The rule's third clause is the rung's scope and not H's contents: a
//! relation the rung covers whose every candidate root already holds is not
//! outside the universe, it has nothing left to guess.
//!
//! **What it is not.** It is not the `rung` event's `uncovered`, M1d's
//! completeness counter, which counts every eligible relation no obligation
//! names, a relation no rule concludes among them (`uncovered.ein`'s `is-a`).
//! And it is relation grain: a covered relation's facts the rung does not
//! propose (a self-edge, which the blind enumerator never builds; an
//! obligation's candidates, which its guard types) are not named.

use std::sync::Arc;

use ein_core::Symbol;
use rustc_hash::FxHashSet;

use crate::compile::CompileError;
use crate::oblgen::Mode;
use crate::plan::Plan;
use crate::saturator::Session;

/// Why the rung that built H proposes no fact of a relation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Why {
    /// The obligations rung built H (or was stuck), and no obligation owes
    /// the relation.
    NotOwed,
    /// The hrules built H, and none asserts a fact of the relation.
    NoHrule,
    /// The blind enumerator built H, which proposes arity 1 and 2 only.
    Arity,
}

impl Why {
    /// `--json-summary`'s spelling.
    pub fn key(self) -> &'static str {
        match self {
            Why::NotOwed => "not-owed",
            Why::NoHrule => "no-hrule",
            Why::Arity => "arity",
        }
    }
}

/// One relation outside the universe.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Outside {
    pub relation: String,
    pub arity: usize,
    pub why: Why,
}

/// The relations outside the universe, sorted by name: see the module's
/// three clauses. `s` is root at its fixpoint with the memo its saturation
/// compiled; `mode` is the rung the universe's generator pass took, and
/// `plans` the memo's plans.
pub(crate) fn outside(
    s: &mut Session<'_>,
    mode: Mode,
    plans: &[Arc<Plan>],
) -> Result<Vec<Outside>, CompileError> {
    let program = s.kb.program_arc();
    let (allowed, excluded, closed) = crate::hypgen::scoping(s);
    let scoped_out = |r: Symbol| {
        closed.contains(&r)
            || allowed.as_ref().is_some_and(|a| !a.contains(&r))
            || excluded.contains(&r)
    };
    let declared: Vec<(Symbol, usize)> = program
        .relations
        .values()
        .filter(|r| !r.signature.is_empty())
        .map(|r| (r.name, r.signature.len()))
        .collect();
    // Clause 3: what the rung proposes facts of.
    let proposed: FxHashSet<Symbol> = match mode {
        Mode::Hrules => crate::hrule::Hrules::new(s)?.asserted_relations(s.terms),
        Mode::Obligations | Mode::Stuck => {
            let memo = s.memo.clone();
            crate::obligations::plans_for(s.kb, s.terms, s.ast, &memo)?
                .iter()
                .filter_map(|p| crate::obligations::open_argument(p, s.terms))
                .collect()
        }
        Mode::Blind | Mode::Declined => declared
            .iter()
            .filter(|&&(_, arity)| arity <= 2)
            .map(|&(r, _)| r)
            .collect(),
    };
    // Clause 1: what the program can conclude or ask about.
    let mut concluded: FxHashSet<Symbol> = FxHashSet::default();
    for p in plans.iter().filter(|p| program.rules.contains(p.rule)) {
        for w in crate::monotone::writes_of(p, s.terms) {
            if !w.negated {
                concluded.insert(w.name);
            }
        }
    }
    let mut asked: FxHashSet<String> = FxHashSet::default();
    for q in &program.queries {
        if let Some(goal) = crate::verdict::query_value(s.ast, q, "goal") {
            asked.extend(ein_ir::pattern_relations(s.ast, goal));
        }
    }
    let why = match mode {
        Mode::Hrules => Why::NoHrule,
        Mode::Obligations | Mode::Stuck => Why::NotOwed,
        Mode::Blind | Mode::Declined => Why::Arity,
    };
    let mut out: Vec<Outside> = declared
        .iter()
        .filter(|&&(r, _)| !proposed.contains(&r) && !scoped_out(r))
        .filter(|&&(r, _)| concluded.contains(&r) || asked.contains(s.terms.sym(r)))
        .map(|&(r, arity)| Outside {
            relation: s.terms.sym(r).to_string(),
            arity,
            why,
        })
        .collect();
    out.sort_by(|a, b| a.relation.cmp(&b.relation));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_core::Terms;
    use ein_ir::{Ast, from_ir::load, parse};

    /// The relations [`outside`] names on `src`, with why, as every route
    /// starts from them: root's fixpoint and H, by `--monotone`'s path.
    fn named(src: &str) -> Vec<(String, Why)> {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let mut kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        let checked = crate::explore::check_monotone(
            &mut kb,
            &mut terms,
            &ast,
            &mut crate::events::Events::off(),
        )
        .expect("root and H");
        checked
            .root
            .outside
            .into_iter()
            .map(|o| (o.relation, o.why))
            .collect()
    }

    /// W6's `uncovered.ein`: the obligations rung owes `seats` alone.
    const OBLIGED: &str = "(import std.algebra :symbols (total-owed surjective-owed))\n\
        (relation is-a T T)\n(relation seats Guest Chair)\n(relation knows Guest Guest)\n\
        (is-a Ann Guest) (is-a Bob Guest) (is-a C1 Chair) (is-a C2 Chair)\n\
        (total-owed seats is-a)\n(surjective-owed seats is-a)\n";
    const MUTUAL: &str = "(rule knows-is-mutual () :match (knows ?a ?b) :assert (knows ?b ?a))\n";

    /// Clause 3 on the obligations rung, and clause 1: `knows`, which a rule
    /// concludes and no obligation owes, is named; `is-a`, which no rule
    /// concludes and no goal asks about, is not, though the rung proposes
    /// none of it either.
    #[test]
    fn a_concluded_relation_no_obligation_owes_is_outside() {
        let src = format!("{OBLIGED}{MUTUAL}(query :goal (seats ?g ?c))");
        assert_eq!(named(&src), [("knows".to_string(), Why::NotOwed)]);
        let bare = format!("{OBLIGED}(query :goal (seats ?g ?c))");
        assert!(
            named(&bare).is_empty(),
            "nothing concludes or asks about knows"
        );
        let asked = format!("{OBLIGED}(query :goal (knows ?a ?b))");
        assert_eq!(named(&asked), [("knows".to_string(), Why::NotOwed)]);
    }

    /// W6's `arity3.ein`: the blind enumerator proposes arity 1 and 2, so a
    /// goal over an arity-3 relation is outside; clause 2, a relation the
    /// program scopes out, is not named.
    #[test]
    fn an_arity_the_blind_enumerator_skips_is_outside_unless_scoped_out() {
        let decls = "(relation is-a T T)\n(relation r3 T T T)\n(relation r2 T T)\n\
                     (is-a A T) (is-a B T)\n";
        let src = format!("{decls}(solve :no-hypothesis (is-a))\n(query :goal (r3 ?x ?y ?z))");
        assert_eq!(named(&src), [("r3".to_string(), Why::Arity)]);
        let unasked = format!("{decls}(solve :no-hypothesis (is-a))");
        assert!(
            named(&unasked).is_empty(),
            "nothing concludes or asks about r3"
        );
        let scoped =
            format!("{decls}(solve :no-hypothesis (is-a r3))\n(query :goal (r3 ?x ?y ?z))");
        assert!(
            named(&scoped).is_empty(),
            "the program scoped r3 out itself"
        );
    }

    /// The hrules: a relation a rule concludes from what they propose is
    /// outside, though R's states differ in it.
    #[test]
    fn a_concluded_relation_no_hrule_proposes_is_outside() {
        let src = "(relation thing T)\n(relation a T)\n(relation p T)\n(thing one)\n\
                   (hrule guess-a () :match (thing ?x) :assert (a ?x))\n\
                   (rule from-a () :match (a ?x) :assert (p ?x))\n\
                   (query :goal (a ?x))";
        assert_eq!(named(src), [("p".to_string(), Why::NoHrule)]);
    }
}
