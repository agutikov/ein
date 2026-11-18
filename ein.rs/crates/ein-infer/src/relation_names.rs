//! A relation's name bound as an object.
//!
//! The blind enumerator's objects are the KB's names and the program's, less
//! what a signature declares, the kernel's words and **every name the
//! registry calls a relation or a rule**
//! ([`crate::hypgen::candidate_objects`]). The registry holds a relation a
//! `(relation …)` declares and the head of every fact a file writes. It does
//! not hold the head of a fact that only a rule concludes: such a head is in
//! the names index beside the arguments, no registry knows it, and so it is an
//! object. Where a program derives `(liquid Tea)` by a rule and declares
//! `(relation tasty Thing)`, `(tasty liquid)` is a candidate like `(tasty
//! Tea)`, and a state of R can hold it.
//!
//! The candidate stays: the engine does what the program as written says, and
//! declaring the relation is the program's way out, since a declared relation
//! is no object. What this module does is **say so**.
//!
//! **What is named.** When the blind enumerator built H (the rung took
//! [`Mode::Blind`], or [`Mode::Declined`] and fell through to it), every
//! argument of a candidate that is
//!
//! - the head of a fact root's fixpoint holds, or
//! - a relation a rule's `:assert` names as written.
//!
//! An argument of a candidate is an object, so the registry holds no relation
//! of that name: no file declares it and none writes a fact of it, and every
//! fact of it is one a rule concluded. The first clause reads the state, the
//! second the rules, so that a relation no rule has fired for at root is
//! named where the program writes its name as a value. A relation a rule
//! writes through a parameter, `(?Q ?r)`, is read by the first clause alone,
//! where root holds a fact of it.
//!
//! The kernel's two dunders are never among them: `__closed__` and
//! `__symmetric__` are no objects, whoever concludes a fact of them.
//!
//! **A fact about the KB.** It decides nothing: H, R, every count and every
//! exit code are what they are without it. Nothing is named when a rung the
//! program wrote built H: an hrule says what to guess, and an obligation's
//! guard says what would pay it.

use ein_core::{FactId, Kb, Symbol, Terms};
use rustc_hash::FxHashSet;

use crate::oblgen::Mode;

/// The relations' names the blind enumerator bound as objects: empty when
/// none.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct RelationNamesAsObjects {
    /// The names that only a rule concludes a fact of and that a candidate
    /// names, sorted.
    pub objects: Vec<String>,
    /// The relations of those candidates, sorted.
    pub relations: Vec<String>,
    /// How many candidates of H name one.
    pub candidates: usize,
}

impl RelationNamesAsObjects {
    pub fn is_empty(&self) -> bool {
        self.candidates == 0
    }
}

/// Every relation a rule's `:assert` names as written, over the three
/// registries.
fn concluded(kb: &Kb) -> FxHashSet<Symbol> {
    let program = kb.program();
    let mut out = FxHashSet::default();
    for reg in [&program.rules, &program.hrules, &program.obligations] {
        for (_, rule) in reg.iter() {
            if let Some(a) = rule.assert_.as_ref() {
                out.extend(a.relation_names.iter().copied());
            }
        }
    }
    out
}

/// Read H against root's fixpoint: `mode` is the rung the universe's pass
/// took, `candidates` H.
pub fn of(kb: &Kb, terms: &Terms, mode: Mode, candidates: &[FactId]) -> RelationNamesAsObjects {
    if !matches!(mode, Mode::Blind | Mode::Declined) || candidates.is_empty() {
        return RelationNamesAsObjects::default();
    }
    let concluded = concluded(kb);
    let mut objects: Vec<String> = Vec::new();
    let mut relations: Vec<String> = Vec::new();
    let mut n = 0;
    for &c in candidates {
        let (head, args) = terms.facts.get(c);
        let mut hit = false;
        for a in args {
            if let Some(s) = a.as_sym()
                && (concluded.contains(&s) || kb.name_as_head(s).next().is_some())
            {
                objects.push(ein_core::text::symbol(terms.sym(s)).into_owned());
                hit = true;
            }
        }
        if hit {
            n += 1;
            relations.push(terms.sym(head).to_string());
        }
    }
    objects.sort();
    objects.dedup();
    relations.sort();
    relations.dedup();
    RelationNamesAsObjects {
        objects,
        relations,
        candidates: n,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_core::Terms;
    use ein_ir::{Ast, from_ir::load, parse};

    /// What [`of`] names on `src`, as every route starts from it: root's
    /// fixpoint and H, by `--monotone`'s path.
    fn named(src: &str) -> RelationNamesAsObjects {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let mut kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        crate::explore::check_monotone(&mut kb, &mut terms, &ast, &mut crate::events::Events::off())
            .expect("root and H")
            .root
            .relation_names
    }

    const DRINKS: &str = "(relation is-a Thing Type :why \"{?1} is a {?2}\")\n(is-a Tea Drink)\n\
        (relation tasty Thing :why \"{?1} is tasty\")\n";

    /// The shape: `liquid` heads the fact a rule derives, no file declares it
    /// or writes a fact of it, and the blind enumerator proposes it.
    #[test]
    fn a_head_only_a_rule_derives_is_named() {
        let src = format!(
            "{DRINKS}(rule liquids () :match (is-a ?x Drink) :assert (liquid ?x) :why \"{{?x}} is a drink, so a liquid\")\n\
             (solve :no-hypothesis (is-a))"
        );
        let r = named(&src);
        assert_eq!(r.objects, ["liquid"]);
        assert_eq!(r.relations, ["tasty"]);
        // `(tasty liquid)`, beside `(tasty Drink)` and `(tasty Tea)`.
        assert_eq!(r.candidates, 1);
    }

    /// The program's way out: a declared relation is no object, so nothing
    /// is named.
    #[test]
    fn a_declared_relation_is_no_object() {
        let src = format!(
            "{DRINKS}(relation liquid Thing :why \"{{?1}} is a liquid\")\n\
             (rule liquids () :match (is-a ?x Drink) :assert (liquid ?x) :why \"{{?x}} is a drink, so a liquid\")\n\
             (solve :no-hypothesis (is-a liquid))"
        );
        assert!(named(&src).is_empty());
    }

    /// A relation no rule has fired for at root is named where the program
    /// writes its name as a value: the rules are read as well as the state.
    #[test]
    fn a_relation_a_rule_would_conclude_is_named_where_it_is_a_value() {
        let src = format!(
            "{DRINKS}(marker liquid)\n\
             (rule liquids () :match (tasty ?x) :assert (liquid ?x) :why \"{{?x}} is tasty, so a liquid\")\n\
             (solve :no-hypothesis (is-a))"
        );
        let r = named(&src);
        assert_eq!(r.objects, ["liquid"]);
        assert_eq!(r.candidates, 1);
    }

    /// A rung the program wrote: an hrule says what to guess, so nothing is
    /// named though its candidate names the derived head.
    #[test]
    fn a_rung_the_program_wrote_names_nothing() {
        let src = format!(
            "{DRINKS}(rule liquids () :match (is-a ?x Drink) :assert (liquid ?x) :why \"{{?x}} is a drink, so a liquid\")\n\
             (hrule guess () :match (is-a ?x Drink) :assert (tasty liquid) :why \"guess that liquid is tasty\")"
        );
        assert!(named(&src).is_empty());
    }
}
