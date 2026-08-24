//! `:expect` — what a `(query …)` says its own answer is.
//!
//! The form M1c
//! [S1c.1.2](../../../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)
//! settled, and the one part of it that is *shape* rather than comparison. The
//! comparison is `ein_infer::expect`, which needs solutions; this module is
//! what the loader validates against, so a program that checks nothing is
//! rejected before anything runs.
//!
//! ```lisp
//! (query :goal   (pet-loc Zebra ?h)
//!        :expect (model (pet-loc Zebra House-5) (pet-loc Fox House-1)
//!                       (not (pet-loc Zebra House-1))))
//!
//! (query :goal   (seat ?who ?n)
//!        :expect (or (model (seat Ann 1) (seat Bob 2))
//!                    (model (seat Ann 2) (seat Bob 1))))
//!
//! (query :goal (p A ?h) :expect (false))
//! ```
//!
//! **Why a `(model …)` head.** The proposed shape was a bare list of facts —
//! `:expect ((p A H1) (p B H2))` — and it does not parse, because
//! `ListHead ::= SYMBOL | VAR | WILDCARD | EQ` and a list is none of those
//! ([`00_ebnf.md` §2](../../../../docs/kernel/ir/03-ein-lang/00_ebnf.md)).
//! Widening `ListHead` to admit a list would change the grammar of *every*
//! form to buy one keyword's ergonomics. A `SYMBOL` head costs nothing: it
//! parses, dumps and round-trips today, and `(or (model …) (model …))` falls
//! out of `OrForm ::= '(' 'or' Value+ ')'` with no further work. `model` is
//! not reserved and needs no exclusion — it is read structurally under
//! `:expect` and nowhere else, so a relation of that name is unaffected.
//!
//! **The verdict is implied, never asserted.** One `(model …)` means
//! `Solution`; `(or …)` means `Ambiguity` with `k` equal to the number of
//! disjuncts, or `Solution` when there is one; `(false)` means
//! `Contradiction`. There is no `:verdict` keyword to disagree with the models
//! beside it.
//!
//! **Why `(false)` and not an invented word.** `false` is already the kernel's
//! ⊥ — one of the five `STRUCTURAL` names, and what every refutation rule in
//! the stdlib asserts (`std.elim`'s `no-room-left`, `std.algebra`'s `total`,
//! `std.slots`' `slot-no-room`, …). A form that spelled contradiction any
//! other way would be introducing a second word for something the language
//! already says. It was `none` until the day it shipped.

use crate::ast::{Ast, Node, NodeId};

const SHAPE_TAIL: &str = "expected `(false)`, `(model …)` or `(or (model …) …)`";
const SHAPE: &str = ":expect — expected `(false)`, `(model …)` or `(or (model …) …)`";

/// One expected model — the facts it lists, in source order.
#[derive(Clone, Debug)]
pub struct Model {
    pub facts: Vec<NodeId>,
}

/// A parsed `:expect` value.
#[derive(Clone, Debug)]
pub enum Expectation {
    /// `(false)` — the search is expected to refute.
    ///
    /// Spelled with the kernel's own ⊥ rather than an invented word, and not
    /// as `()`, because an empty list reads as *the empty model* — a different
    /// claim, and one `(model)` already makes.
    Contradiction,
    /// `(model …)` — one model.
    One(Model),
    /// `(or (model …)+)` — that **set** of models, compared as a set.
    Any(Vec<Model>),
}

impl Expectation {
    /// The models this expectation lists; empty for `(false)`.
    pub fn models(&self) -> &[Model] {
        match self {
            Expectation::Contradiction => &[],
            Expectation::One(m) => std::slice::from_ref(m),
            Expectation::Any(ms) => ms,
        }
    }

    /// The verdict this shape implies, as the CLI and the events spell it.
    pub fn verdict_name(&self) -> &'static str {
        match self {
            Expectation::Contradiction => "Contradiction",
            Expectation::One(_) => "Solution",
            Expectation::Any(ms) if ms.len() == 1 => "Solution",
            Expectation::Any(_) => "Ambiguity",
        }
    }
}

/// A listed fact, split into what the comparison needs.
pub struct ExpectFact<'a> {
    /// `true` for `(not (r …))`.
    pub negated: bool,
    /// The relation the fact is *about* — `r` in both `(r a b)` and
    /// `(not (r a b))`.
    pub relation: &'a str,
    /// The rendered s-expression, byte-identical to
    /// `ein_infer::events::sexpr` for the same fact.
    pub rendered: String,
}

/// Read a `:expect` value, or say what is wrong with it.
///
/// The messages are the loader's, so they end where a loader message ends and
/// name the form rather than a node id.
pub fn parse(ast: &Ast, node: NodeId) -> Result<Expectation, String> {
    if let Node::Atom(s) = ast.node(node) {
        // A bare atom is never an expectation. `false` gets its own line
        // because writing it unparenthesised is the likely slip, and ⊥ is
        // spelled `(false)` everywhere else in the language.
        return Err(match ast.sym(s) {
            "false" => {
                ":expect false — ⊥ is spelled `(false)`, as it is in every `:assert`".to_string()
            }
            other => {
                format!(":expect {other} — expected `(false)`, `(model …)` or `(or (model …) …)`")
            }
        });
    }
    let Node::SForm { head, args } = ast.node(node) else {
        return Err(SHAPE.into());
    };
    let Some(name) = ast.atom_name(head) else {
        return Err(SHAPE.into());
    };
    match name {
        "false" if ast.args(args).is_empty() => Ok(Expectation::Contradiction),
        "false" => Err(":expect (false …) — ⊥ takes no arguments".into()),
        "model" => Ok(Expectation::One(model(ast, ast.args(args))?)),
        "or" => {
            let items = ast.args(args);
            if items.is_empty() {
                return Err(":expect (or) — an empty disjunction expects nothing".into());
            }
            let mut out = Vec::with_capacity(items.len());
            for &item in items {
                let Node::SForm {
                    head: h,
                    args: iargs,
                } = ast.node(item)
                else {
                    return Err(":expect (or …) — every disjunct must be a `(model …)`".into());
                };
                if ast.atom_name(h) != Some("model") {
                    return Err(":expect (or …) — every disjunct must be a `(model …)`".into());
                }
                out.push(model(ast, ast.args(iargs))?);
            }
            Ok(Expectation::Any(out))
        }
        // `and` / `or` / `not` land here when they are used as a model
        // wrapper, and the message says what they are missing rather than
        // reporting them as an unknown relation two checks later.
        "and" => Err(":expect (and …) — a model is `(model …)`; `and` is a \
                     connective and would not say that the listed facts are a \
                     relation's *complete* extent"
            .into()),
        other => Err(format!(":expect ({other} …) — {SHAPE_TAIL}")),
    }
}

fn model(ast: &Ast, items: &[NodeId]) -> Result<Model, String> {
    for &item in items {
        fact(ast, item)?;
    }
    Ok(Model {
        facts: items.to_vec(),
    })
}

/// Split a listed fact, checking it is a **ground** fact form.
///
/// Ground is the whole of the rule: an expectation is an answer, and an answer
/// with a `?var` in it is a pattern. A variable here would silently match
/// anything, which is the one outcome this form exists to prevent.
pub fn fact<'a>(ast: &'a Ast, node: NodeId) -> Result<ExpectFact<'a>, String> {
    let (negated, inner) = match ast.node(node) {
        Node::SForm { head, args } if ast.atom_name(head) == Some("not") => match ast.args(args) {
            [one] => (true, *one),
            _ => return Err(":expect — `(not …)` takes exactly one fact".into()),
        },
        _ => (false, node),
    };
    let Node::SForm { head, args } = ast.node(inner) else {
        return Err(format!(":expect — `{}` is not a fact", render(ast, inner)));
    };
    let Some(relation) = ast.atom_name(head) else {
        return Err(format!(
            ":expect — `{}` has no relation name in head position",
            render(ast, inner)
        ));
    };
    for &arg in ast.args(args) {
        ground(ast, arg)?;
    }
    Ok(ExpectFact {
        negated,
        relation,
        rendered: render(ast, node),
    })
}

fn ground(ast: &Ast, node: NodeId) -> Result<(), String> {
    match ast.node(node) {
        Node::Atom(_) | Node::Int(_) | Node::Str(_) => Ok(()),
        Node::Var(s) => Err(format!(
            ":expect — `?{}` is a variable; an expectation is an answer, not a pattern",
            ast.sym(s)
        )),
        Node::Wildcard => Err(":expect — `_` is a wildcard; an expectation is an answer, \
                              not a pattern"
            .into()),
        Node::SForm { head, args } => {
            if ast.atom_name(head).is_none() {
                return Err(
                    ":expect — a nested fact needs a relation name in head position".into(),
                );
            }
            for &arg in ast.args(args) {
                ground(ast, arg)?;
            }
            Ok(())
        }
        _ => Err(format!(
            ":expect — `{}` is not a fact argument",
            render(ast, node)
        )),
    }
}

/// An expectation fact as its canonical s-expression.
///
/// **This must agree with `ein_infer::events::sexpr` byte for byte**, since
/// that is what the comparison holds it against — a fact is compared by
/// *content*, never by `FactId`, because two runs do not share an interner
/// (`fork_audit`'s reason). `ein-infer`'s `rendering_agrees_with_the_fact_dump`
/// is what keeps the two honest.
pub fn render(ast: &Ast, node: NodeId) -> String {
    match ast.node(node) {
        // `Node::Int` and `Node::Str` both hold their text already: an int as
        // its canonical decimal, a string unescaped — which is exactly what
        // `Terms::display` prints for the interned value.
        Node::Atom(s) | Node::Int(s) | Node::Str(s) => ast.sym(s).to_string(),
        Node::Var(s) => format!("?{}", ast.sym(s)),
        Node::Wildcard => "_".to_string(),
        Node::Keyword(s) => format!(":{}", ast.sym(s)),
        Node::Range { low, high } => match high {
            Some(h) => format!("{}..{}", ast.sym(low), ast.sym(h)),
            None => format!("{}..*", ast.sym(low)),
        },
        Node::KwPair { key, value } => format!("{} {}", render(ast, key), render(ast, value)),
        Node::SForm { head, args } => {
            let items = ast.args(args);
            if items.is_empty() {
                // `(q)`, no trailing space — `events::sexpr`'s nullary case.
                format!("({})", render(ast, head))
            } else {
                let inner: Vec<String> = items.iter().map(|&a| render(ast, a)).collect();
                format!("({} {})", render(ast, head), inner.join(" "))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse as parse_ein;

    /// The `:expect` value of the file's first query.
    fn value(src: &str) -> (Ast, NodeId) {
        let mut ast = Ast::new();
        let forms = parse_ein(&mut ast, src, None).expect("parses");
        for &form in &forms {
            if ast.head_name(form) != Some("query") {
                continue;
            }
            for &a in ast.form_args(form) {
                if let Node::KwPair { key, value } = ast.node(a)
                    && let Node::Keyword(k) = ast.node(key)
                    && ast.sym(k) == "expect"
                {
                    return (ast, value);
                }
            }
        }
        panic!("no :expect in {src}");
    }

    fn parsed(src: &str) -> Result<Expectation, String> {
        let (ast, node) = value(src);
        parse(&ast, node)
    }

    #[test]
    fn the_three_shapes() {
        let bottom = parsed("(query :goal (p ?x) :expect (false))").expect("(false)");
        assert!(matches!(bottom, Expectation::Contradiction));
        assert_eq!(bottom.verdict_name(), "Contradiction");

        let one = parsed("(query :goal (p ?x) :expect (model (p A) (not (p B))))").expect("model");
        assert_eq!(one.models().len(), 1);
        assert_eq!(one.models()[0].facts.len(), 2);
        assert_eq!(one.verdict_name(), "Solution");

        let any =
            parsed("(query :goal (p ?x) :expect (or (model (p A)) (model (p B))))").expect("or");
        assert_eq!(any.models().len(), 2);
        assert_eq!(any.verdict_name(), "Ambiguity");
    }

    /// `(or …)` with one disjunct is a `Solution`, not an `Ambiguity` — `k` is
    /// the number of disjuncts and 1 is a legal number of them.
    #[test]
    fn a_single_disjunct_implies_solution() {
        let any = parsed("(query :goal (p ?x) :expect (or (model (p A))))").expect("or");
        assert_eq!(any.verdict_name(), "Solution");
    }

    #[test]
    fn the_empty_model_is_not_false() {
        let empty = parsed("(query :goal (p ?x) :expect (model))").expect("model");
        assert_eq!(empty.models()[0].facts.len(), 0);
        assert_eq!(
            empty.verdict_name(),
            "Solution",
            "`(model)` expects a model with nothing in the relations it names — \
             which is no relations. `(false)` is the contradiction."
        );
    }

    #[test]
    fn a_variable_in_an_expectation_is_rejected() {
        let e = parsed("(query :goal (p ?x) :expect (model (p ?x)))").expect_err("a pattern");
        assert!(e.contains("?x"), "{e}");
        assert!(e.contains("not a pattern"), "{e}");
        let e = parsed("(query :goal (p ?x) :expect (model (p _)))").expect_err("a wildcard");
        assert!(e.contains("wildcard"), "{e}");
    }

    #[test]
    fn the_shape_errors_name_what_was_expected() {
        for src in [
            "(query :goal (p ?x) :expect all)",
            "(query :goal (p ?x) :expect none)",
            "(query :goal (p ?x) :expect false)",
            "(query :goal (p ?x) :expect (false A))",
            "(query :goal (p ?x) :expect (and (p A) (p B)))",
            "(query :goal (p ?x) :expect (models (p A)))",
            "(query :goal (p ?x) :expect (or (p A)))",
            "(query :goal (p ?x) :expect (model 3))",
            "(query :goal (p ?x) :expect (model (p (?q A))))",
        ] {
            let e = parsed(src).expect_err(src);
            assert!(e.starts_with(":expect"), "{src}: {e}");
        }
    }

    /// Two malformed shapes never reach [`parse`] at all, because the grammar
    /// refuses them first: `OrForm ::= '(' 'or' Value+ ')'` wants at least one
    /// disjunct, and `NotForm ::= '(' 'not' Value KwPair* ')'` exactly one
    /// fact. Both guards stay in `parse` — an empty `Any` would be an
    /// `Ambiguity` with nothing to be ambiguous between — but this is where it
    /// is written down that a *program* cannot get there.
    #[test]
    fn two_shapes_are_parse_errors_before_they_are_load_errors() {
        for src in [
            "(query :goal (p ?x) :expect (or))",
            "(query :goal (p ?x) :expect (model (not (p A) (p B))))",
        ] {
            let mut ast = Ast::new();
            assert!(parse_ein(&mut ast, src, None).is_err(), "{src}");
        }
    }

    /// ⊥ is `(false)`, the kernel's own word — and the two ways a reader is
    /// likely to reach for it instead each get a message that says so.
    #[test]
    fn bottom_is_spelled_with_the_kernels_own_word() {
        let e = parsed("(query :goal (p ?x) :expect false)").expect_err("unparenthesised");
        assert!(e.contains("`(false)`"), "{e}");
        assert!(
            e.contains(":assert"),
            "and says where that spelling comes from: {e}"
        );
        let e = parsed("(query :goal (p ?x) :expect (and (p A)))").expect_err("a connective");
        assert!(e.contains("complete* extent"), "{e}");
    }

    #[test]
    fn a_fact_splits_into_relation_polarity_and_text() {
        let (ast, node) = value("(query :goal (p ?x) :expect (model (p A 3) (not (q B))))");
        let Expectation::One(m) = parse(&ast, node).expect("model") else {
            panic!("one model");
        };
        let pos = fact(&ast, m.facts[0]).expect("a fact");
        assert!(!pos.negated);
        assert_eq!(pos.relation, "p");
        assert_eq!(pos.rendered, "(p A 3)");
        let neg = fact(&ast, m.facts[1]).expect("a fact");
        assert!(neg.negated);
        assert_eq!(
            neg.relation, "q",
            "a negative is *about* the inner relation"
        );
        assert_eq!(neg.rendered, "(not (q B))");
    }

    #[test]
    fn a_nullary_fact_renders_without_a_trailing_space() {
        let (ast, node) = value("(query :goal (p ?x) :expect (model (q)))");
        let Expectation::One(m) = parse(&ast, node).expect("model") else {
            panic!("one model");
        };
        assert_eq!(fact(&ast, m.facts[0]).expect("a fact").rendered, "(q)");
    }
}
