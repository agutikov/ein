//! The built-in predicate registry — `eq` and `neq`, and nothing else.
//!
//! Q33 caps the M1 registry at two: a predicate's truth is *computed* from the
//! bindings, where a relation's truth is *data*. Ein has no numbers and none
//! is planned (the user, 2026-11-06, Q-M1e2.24), so no numeric, cardinality
//! or aggregation primitive is either.
//!
//! It is an enum with a four-function surface, because a `PredId`-indexed
//! table is what the matcher wants and because the *set* of names is fixed:
//! `names()` is sorted, and the blind hypothesis enumerator reserves both
//! names, which is what stops it proposing `eq` as a graph object.
//!
//! **Since M1f S1f.5.7 the two are relations**, of equality and of
//! non-equality (the user's ruling of 2026-11-07 on Q-M1e2.26): `(eq x x)` for
//! every value, and `(neq x y)` for every two different values, computed and
//! never stored, so no fact, rule or commitment changes them. A guard binds a
//! variable nothing else binds — `eq` to the other side, and `neq`, or `eq`
//! with both sides free, over [`domain`], the program's names.

use ein_core::terms::Terms;
use ein_core::{Kb, Symbol, Value};
use ein_ir::Ast;

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

/// The predicate for a head name, or `None`.
pub fn get(name: &str) -> Option<Pred> {
    match name {
        "eq" => Some(Pred::Eq),
        "neq" => Some(Pred::Neq),
        _ => None,
    }
}

/// Whether `name` is a built-in predicate's head.
pub fn is_predicate(name: &str) -> bool {
    get(name).is_some()
}

/// Every registered name, **sorted**.
pub fn names() -> [&'static str; 2] {
    ["eq", "neq"]
}

/// What a guard's variable ranges over where nothing else binds it: **the
/// program's names** ([`crate::invariant::program_names`], the user's ruling
/// of 2026-11-04 on Q-M1e2.9 (a)), each once, sorted by text so that no
/// interning order reaches a guard's rows.
///
/// A name only a query writes is none of them: `(query :goal (neq w ?y))`
/// answers a row per name of the program but `w`. Every state of a program
/// has the same list (`ein-infer/tests/alive_invariant.rs`, R1), so it is
/// taken once, from whichever state first asks, and kept on the program.
pub fn domain<'k>(kb: &'k Kb, terms: &Terms, ast: &Ast) -> &'k [Value] {
    kb.program().domain.get_or_init(|| {
        let mut names: Vec<Symbol> = crate::invariant::program_names(kb, terms, ast)
            .into_iter()
            .map(|n| n.name)
            .collect();
        names.sort_by(|&a, &b| terms.sym(a).cmp(terms.sym(b)));
        names.dedup();
        names.into_iter().map(Value::sym).collect()
    })
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

    /// **`eq` and `neq` are their definition over the program's names** (M1f
    /// S1f.5.7, the user's ruling of 2026-11-07 on Q-M1e2.26), each argument
    /// a name of the program, a name only the query writes, or a variable,
    /// free or one the other side shares: a goal's rows are every assignment
    /// of its variables that makes the fact hold, a variable no premise binds
    /// ranging over [`domain`] — save that `eq` binds a variable one side is
    /// to the other side's value, whatever it is, the one value that makes
    /// the fact hold.
    #[test]
    fn eq_and_neq_are_their_definition_over_the_programs_names() {
        use ein_ir::{Ast, from_ir::load_query, parse};
        const PROGRAM: &str = "(relation p Node)\n(p a)\n(p b)\n";
        let sides = ["a", "w", "?x", "?y"];
        let mut goals: Vec<String> = Vec::new();
        for pred in ["eq", "neq"] {
            for l in sides {
                for r in sides {
                    goals.push(format!("({pred} {l} {r})"));
                }
            }
        }
        let src = format!(
            "{PROGRAM}{}",
            goals
                .iter()
                .map(|g| format!("(query :goal {g})\n"))
                .collect::<String>()
        );
        for (i, goal) in goals.iter().enumerate() {
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let forms = parse(&mut ast, &src, Some("<test>")).expect("parses");
            let kb = load_query(&mut ast, &mut terms, &forms, None, i).expect("loads");
            let domain: Vec<String> = domain(&kb, &terms, &ast)
                .iter()
                .map(|&v| terms.display(v))
                .collect();
            assert_eq!(
                domain,
                ["Node", "a", "b", "p"],
                "the program's names, sorted"
            );
            let mut got: Vec<Vec<(String, String)>> =
                crate::verdict::goal_bindings(&ast, &mut terms, &kb, None)
                    .into_iter()
                    .map(|row| {
                        let mut row: Vec<(String, String)> = row
                            .into_iter()
                            .map(|(k, v)| (terms.sym(k).to_string(), terms.display(v)))
                            .collect();
                        row.sort();
                        row
                    })
                    .collect();
            got.sort();
            got.dedup();
            // The definition, by enumeration: every assignment of the goal's
            // variables, each over the names and over the goal's own names
            // (`w`), kept where the fact holds; then a variable `eq` binds to
            // a name only the query writes is kept, and a `neq` one is not.
            let parts: Vec<&str> = goal
                .trim_matches(|c| c == '(' || c == ')')
                .split(' ')
                .collect();
            let (pred, l, r) = (parts[0], parts[1], parts[2]);
            let mut vars: Vec<&str> = [l, r].into_iter().filter(|s| s.starts_with('?')).collect();
            vars.dedup();
            let mut values: Vec<String> = domain.clone();
            values.push("w".to_string());
            let mut want: Vec<Vec<(String, String)>> = Vec::new();
            let n = vars.len() as u32;
            for k in 0..values.len().pow(n) {
                let mut at: Vec<(String, String)> = Vec::new();
                let mut k = k;
                for v in &vars {
                    at.push((v[1..].to_string(), values[k % values.len()].clone()));
                    k /= values.len();
                }
                let value = |s: &str| -> String {
                    match s.strip_prefix('?') {
                        Some(name) => at.iter().find(|(n, _)| n == name).expect("bound").1.clone(),
                        None => s.to_string(),
                    }
                };
                let holds = (value(l) == value(r)) == (pred == "eq");
                // `w` is no name of the program: a free variable ranges over
                // it only where `eq` binds the variable to it.
                let outside = at.iter().any(|(_, v)| v == "w");
                let bound_by_eq = pred == "eq" && (l == "w" || r == "w");
                if holds && (!outside || bound_by_eq) {
                    at.sort();
                    want.push(at);
                }
            }
            want.sort();
            want.dedup();
            assert_eq!(got, want, "{goal}");
        }
    }

    #[test]
    fn a_predicate_name_interns_to_the_kernel_symbol() {
        let terms = Terms::new();
        assert_eq!(terms.sym(Pred::Eq.symbol(&terms)), "eq");
        assert_eq!(terms.sym(Pred::Neq.symbol(&terms)), "neq");
    }
}
