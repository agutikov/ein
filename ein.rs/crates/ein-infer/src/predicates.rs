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

    #[test]
    fn a_predicate_name_interns_to_the_kernel_symbol() {
        let terms = Terms::new();
        assert_eq!(terms.sym(Pred::Eq.symbol(&terms)), "eq");
        assert_eq!(terms.sym(Pred::Neq.symbol(&terms)), "neq");
    }
}
