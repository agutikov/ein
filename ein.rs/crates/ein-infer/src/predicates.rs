//! The built-in predicate registry — `eq` and `neq`, and nothing else.
//!
//! Q33 caps the M1 registry at two: a predicate's truth is *computed* from the
//! bindings, where a relation's truth is *data*. All numeric / set /
//! cardinality / aggregation primitives are deferred to followups.
//!
//! ein.py keeps a `dict[str, Callable]` so a followup can `register()` a
//! third. Here it is an enum plus the same four-function surface, because a
//! `PredId`-indexed table is what the matcher wants and because the *set* of
//! names is what reaches output — `names()` is sorted and feeds
//! `primitives.non_object_names`, which is what stops the blind hypothesis
//! enumerator proposing `eq` as a graph object.

use ein_core::Symbol;
use ein_core::terms::Terms;

/// A registered built-in predicate.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Pred {
    Eq,
    Neq,
}

impl Pred {
    pub fn as_str(self) -> &'static str {
        match self {
            Pred::Eq => "eq",
            Pred::Neq => "neq",
        }
    }

    pub fn symbol(self, terms: &Terms) -> Symbol {
        match self {
            Pred::Eq => terms.kernel.eq,
            Pred::Neq => terms.kernel.neq,
        }
    }

    /// How many arguments the predicate reads — **and therefore how many it
    /// may be written with**, M1e S1e.2.1.
    ///
    /// It lives on the predicate rather than in the compiler because that is
    /// what makes the check a property of the registry: a third predicate
    /// registered here declares its arity in the same line that declares its
    /// name, and cannot be lowered without one. Both of M1's read exactly two
    /// — `Matcher::guard_holds` resolves `args[0]` against
    /// `args[1]` — and until S1e.2.1 nothing checked it: below two the matcher
    /// panicked, above two the tail was dropped in silence.
    pub fn arity(self) -> usize {
        match self {
            Pred::Eq | Pred::Neq => 2,
        }
    }
}

/// `predicates.get(name)` — the predicate for a head name, or `None`.
pub fn get(name: &str) -> Option<Pred> {
    match name {
        "eq" => Some(Pred::Eq),
        "neq" => Some(Pred::Neq),
        _ => None,
    }
}

/// `predicates.is_predicate(name)`.
pub fn is_predicate(name: &str) -> bool {
    get(name).is_some()
}

/// Every registered name, **sorted** — ein.py's `tuple(sorted(_REGISTRY))`.
pub fn names() -> [&'static str; 2] {
    ["eq", "neq"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_registry_is_the_two_names_m1_ships() {
        assert_eq!(names(), ["eq", "neq"]);
        assert!(names().windows(2).all(|w| w[0] < w[1]), "names() is sorted");
        assert_eq!(get("eq"), Some(Pred::Eq));
        assert_eq!(get("neq"), Some(Pred::Neq));
        // `not` is a structural wrapper, not a predicate — the distinction the
        // compiler leans on when it decides between a `Guard` and a `Scan`.
        assert!(!is_predicate("not"));
        assert!(!is_predicate("absent"));
    }

    /// Every registered predicate declares an arity, and the compiler refuses
    /// anything else — M1e S1e.2.1, `CO-H1`'s class.
    #[test]
    fn every_registered_predicate_declares_its_arity() {
        for name in names() {
            let p = get(name).expect("names() is the registry");
            assert_eq!(p.arity(), 2, "{name}: M1's two predicates are binary");
        }
    }

    #[test]
    fn a_predicate_name_interns_to_the_kernel_symbol() {
        let terms = Terms::new();
        assert_eq!(terms.sym(Pred::Eq.symbol(&terms)), "eq");
        assert_eq!(terms.sym(Pred::Neq.symbol(&terms)), "neq");
    }
}
