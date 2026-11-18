//! **A slice of a knowledge base**: the held facts a selection keeps, which a
//! view draws with what they point at (`ein_render::kb_dot`, Q-M1f.527).
//!
//! Two selections, both read off the knowledge base's graph and nothing
//! else, no fact's provenance and no file it was written in:
//!
//! - [`about_relations`], **the facts about relations** (Q-M1f.523): every
//!   fact the knowledge base holds with a relation of the program directly
//!   under one of its argument arrows, `2`, `3`, …. A relation of the program
//!   is [`ein_core::Program::relations_of`], the one list a goal and a claim
//!   are held to, so the selection reads each knowledge base of the program by
//!   the same relations; an atom that is a relation is one wherever it
//!   stands, `z` under a numeral's arrow and `eq` under an order's. What
//!   `ein render constraints` draws, of root's fixpoint (Q-M1f.530).
//! - [`answering`], **the facts that answer a goal** (Q-M1f.529): every held
//!   fact a pattern of the goal matches, in a match whose `(absent …)`
//!   guards hold, the matched disjunct's alone for an `(or …)` goal. What
//!   `ein render kb --goal` and `--query` draw.
//!
//! A fact that a kept fact points at and the knowledge base does not hold is
//! not kept: it is drawn as what a kept fact points at, in its frame
//! (Q-M1f.527).

use ein_core::{FactId, Kb, Terms};
use ein_ir::{Ast, NodeId};
use rustc_hash::FxHashSet;

/// **The facts about relations** (Q-M1f.523): every fact `kb` holds with a
/// relation of the program, [`ein_core::Program::relations_of`], directly
/// under one of its argument arrows, in the knowledge base's order. A fact
/// with a relation only under an arrow of a fact under its own arrows is
/// not among them.
pub fn about_relations(kb: &Kb, terms: &Terms) -> Vec<FactId> {
    let relations: FxHashSet<ein_core::Symbol> =
        kb.program().relations_of.iter().copied().collect();
    kb.facts()
        .filter(|&f| {
            let (_, args) = terms.fact(f);
            args.iter()
                .any(|a| a.as_sym().is_some_and(|s| relations.contains(&s)))
        })
        .collect()
}

/// **The facts that answer `goal`** in `kb` (Q-M1f.529): the facts each of
/// its matches matched ([`crate::verdict::goal_matches`]), every match whose
/// lifted `(absent …)` guards hold, in the knowledge base's order. A check,
/// `eq`, `neq` or `absent`, matches no fact and keeps none. A goal the
/// compiler refuses is its error.
pub fn answering(
    ast: &Ast,
    terms: &mut Terms,
    kb: &Kb,
    goal: NodeId,
) -> Result<Vec<FactId>, crate::compile::CompileError> {
    if let Some(e) = crate::verdict::goal_plan_error(ast, terms, kb, Some(goal)) {
        return Err(e);
    }
    let matched: FxHashSet<FactId> = crate::verdict::goal_matches(ast, terms, kb, Some(goal))
        .into_iter()
        .flat_map(|m| m.premises)
        .collect();
    Ok(kb.facts().filter(|f| matched.contains(f)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_ir::{from_ir::load, parse, read_goal};

    /// A program: `r` and `s` relations, `x`, `y` and `w` objects.
    const PROGRAM: &str = "(relation r T T :why \"{?3} r {?4}\")\n\
                           (r x y) (s x) (symmetric r) (likes w y) (pair r s)\n\
                           (claims w (r y x)) (about w (inner r))\n";

    fn loaded() -> (Ast, Terms, Kb) {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, PROGRAM, Some("<test>")).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        (ast, terms, kb)
    }

    fn texts(terms: &Terms, facts: &[FactId]) -> Vec<String> {
        facts.iter().map(|&f| terms.fact_text(f)).collect()
    }

    /// **The facts about relations** (Q-M1f.523): a relation of the program
    /// directly under an argument arrow, the declaration and the fact the
    /// loader adds beside it among them; not a fact whose relation stands
    /// only under an arrow of a fact under its own, nor a fact about objects.
    #[test]
    fn the_facts_about_relations_are_those_with_a_relation_under_an_arrow() {
        let (_ast, terms, kb) = loaded();
        let mut got = texts(&terms, &about_relations(&kb, &terms));
        got.sort();
        assert_eq!(
            got,
            [
                "(pair r s)",
                "(relation r T T)",
                "(relation r)",
                "(symmetric r)"
            ]
        );
    }

    /// **The facts that answer a goal** (Q-M1f.529): what its matches
    /// matched, the matched disjunct's alone for an `(or …)`, no check, and
    /// none of a match an `(absent …)` refutes.
    #[test]
    fn the_facts_that_answer_a_goal_are_what_its_matches_matched() {
        let (mut ast, mut terms, kb) = loaded();
        let mut answer = |goal: &str| -> Vec<String> {
            let query = read_goal(&mut ast, &terms, &kb, goal, "<goal>").expect("a goal");
            let goal = crate::verdict::query_value(&ast, &query, "goal").expect("a goal");
            let facts = answering(&ast, &mut terms, &kb, goal).expect("compiles");
            let mut got = texts(&terms, &facts);
            got.sort();
            got
        };
        assert_eq!(
            answer("(and (relation ?q) (symmetric ?q))"),
            ["(relation r)", "(symmetric r)"]
        );
        assert_eq!(
            answer("(or (s ?a) (and (likes ?a ?b) (neq ?a ?b)) (r ?a w))"),
            ["(likes w y)", "(s x)"]
        );
        assert_eq!(
            answer("(and (r ?a ?b) (absent (s ?a)))"),
            Vec::<String>::new()
        );
        assert_eq!(answer("(claims ?who (r ?a ?b))"), ["(claims w (r y x))"]);
    }
}
