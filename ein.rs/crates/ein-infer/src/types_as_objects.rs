//! A type bound as an object — M1e2
//! [S1e2.2.10](../../../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.10_types_as_objects.md),
//! C1, on the user's ruling of 2026-11-03 on
//! [Q-M1e.12](../../../../docs/history/m1e_review_processing/open_questions.md#q-m1e12--the-blind-rung-is-untyped-and-a-model-binds-a-type-as-an-object),
//! (a) *the fixture, with the warning*.
//!
//! The blind enumerator is **untyped** (S1.7.23: the kernel commits to no type
//! system). It fills every slot of every open relation with every name of the
//! program a signature does not declare, so where a program writes `(is-a Red
//! Color)` and declares `(relation co-located T T)`, `(co-located Blue Color)`
//! is a candidate like `(co-located Blue H5)`, and a state of R can hold it.
//! The engine does what the signature says, and the answer is right for the
//! program as written; what the ruling asks is that the read-out **say so**, so
//! that an author learns which relation admits it.
//!
//! **What is named.** When the blind enumerator built H (the rung took
//! [`Mode::Blind`], or [`Mode::Declined`] and fell through to it), every name
//! that is the second argument of an `(is-a x y)` fact at root's fixpoint and
//! an argument of a candidate in H: the objects, the relations of the
//! candidates that name one, and how many candidates do. Nothing when a rung
//! the program typed built H: an hrule says what to guess and an obligation's
//! guard types its candidates.
//!
//! **A fact about the KB, not a type system.** It reads the relation named
//! `is-a` by its name, which no other module of this crate does (the
//! `ein-render` views do), and it decides nothing: H, R, every count and every
//! exit code are what they were without it, as `crate::invariant`'s check was a
//! report and not a refusal. A name the program declares as a type in a
//! relation signature is not a candidate object at all
//! ([`crate::hypgen::candidate_objects`]), which is why the sentence the
//! read-out prints ends on *no signature names it*: that is the program's way
//! out, and `examples/branching/04_two_levels.ein`'s `(relation co-located
//! Color House)` takes it.
//!
//! Measured 2026-11-04 at `86df5456` (T1e2.2.10.2): of the 234 programs of the
//! sweep that reach H, 151 under the blind enumerator, it names objects on 13,
//! every one a corpus entry, and on 11 once that stage typed
//! `examples/branching/06_lookahead_on.ein` and `07`, which were C1's
//! witnesses. The same 13 when every rung is read, so the scope to the blind
//! enumerator moves nothing in the corpus; it is the ruling's word, *untyped*.

use ein_core::{FactId, Kb, Symbol, Terms};
use rustc_hash::FxHashSet;

use crate::oblgen::Mode;

/// The relation whose second argument is a type, read by name.
pub const IS_A: &str = "is-a";

/// What the untyped rung bound as an object: empty when nothing.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct TypesAsObjects {
    /// The names on the right of an `is-a` that a candidate names, sorted.
    pub objects: Vec<String>,
    /// The relations of those candidates, sorted.
    pub relations: Vec<String>,
    /// How many candidates of H name one.
    pub candidates: usize,
}

impl TypesAsObjects {
    pub fn is_empty(&self) -> bool {
        self.candidates == 0
    }
}

/// Read H against root's fixpoint: `mode` is the rung the universe's pass
/// took, `candidates` H.
pub fn of(kb: &Kb, terms: &Terms, mode: Mode, candidates: &[FactId]) -> TypesAsObjects {
    if !matches!(mode, Mode::Blind | Mode::Declined) {
        return TypesAsObjects::default();
    }
    let Some(is_a) = terms.syms.get(IS_A) else {
        return TypesAsObjects::default();
    };
    let types: FxHashSet<Symbol> = kb
        .facts_of(is_a)
        .filter_map(|f| match terms.facts.args(f) {
            [_, y] => y.as_sym(),
            _ => None,
        })
        .collect();
    if types.is_empty() {
        return TypesAsObjects::default();
    }
    let mut objects: Vec<String> = Vec::new();
    let mut relations: Vec<String> = Vec::new();
    let mut n = 0;
    for &c in candidates {
        let (head, args) = terms.facts.get(c);
        let mut hit = false;
        for a in args {
            if let Some(s) = a.as_sym()
                && types.contains(&s)
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
    TypesAsObjects {
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
    fn named(src: &str) -> TypesAsObjects {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let mut kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        crate::explore::check_monotone(&mut kb, &mut terms, &ast, &mut crate::events::Events::off())
            .expect("root and H")
            .root
            .types_as_objects
    }

    const TYPES: &str = "(relation is-a T T :why \"{?1} is a {?2}\")\n(__closed__ is-a)\n\
        (is-a Red Color) (is-a Blue Color) (is-a H1 House) (is-a H2 House)\n";

    /// C1's shape: a relation over `T` admits `Color` and `House`, which are
    /// on the right of an `is-a`, and the blind enumerator proposes them.
    #[test]
    fn an_untyped_signature_binds_the_types_as_objects() {
        let src = format!(
            "{TYPES}(relation co-located T T :why \"{{?1}} and {{?2}} are co-located\")\n(query :goal (co-located Blue ?h))"
        );
        let t = named(&src);
        assert_eq!(t.objects, ["Color", "House"]);
        assert_eq!(t.relations, ["co-located"]);
        // Each of the six other names beside each type in each slot, and the
        // two types beside each other in either order: 2 × 2 × 4 + 2.
        assert_eq!(t.candidates, 18);
    }

    /// The retyped relation: a name a signature declares is no candidate
    /// object, so nothing is named.
    #[test]
    fn a_signature_naming_the_types_keeps_them_out() {
        let src = format!(
            "{TYPES}(relation co-located Color House :why \"{{?1}} and {{?2}} are co-located\")\n(query :goal (co-located Blue ?h))"
        );
        assert!(named(&src).is_empty());
    }

    /// A rung the program typed: an hrule says what to guess, so nothing is
    /// named though its candidates name a type.
    #[test]
    fn a_rung_the_program_typed_names_nothing() {
        let src = format!(
            "{TYPES}(relation co-located T T :why \"{{?1}} and {{?2}} are co-located\")\n\
             (hrule guess () :match (is-a ?c Color) :assert (co-located ?c House) :why \"guess that {{?c}}, a Color, and House are co-located\")\n\
             (query :goal (co-located Blue ?h))"
        );
        assert!(named(&src).is_empty());
    }
}
