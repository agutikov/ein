//! `:expect` — what a `(query …)` says its own answer is.
//!
//! The query's claim
//! ([`12-query.md` § Claims](../../../../docs/ein/user/03-syntax-and-semantics/12-query.md#claims)),
//! and the one part of it that is *shape* rather than comparison. The
//! comparison is `ein_infer::expect`, which needs solutions; this module reads
//! the claim the loader validates, so a program that checks nothing is
//! rejected before anything runs. The shape itself is the grammar's
//! (`ein.y`'s `Claim`), and what the loader checks is what a grammar cannot
//! see: that a listed relation is the program's, and that a listed fact is one
//! the query's goal matches.
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
//! **Why a `(model …)` head.** A bare list of facts —
//! `:expect ((p A H1) (p B H2))` — would need a form whose head is a list, and
//! admitting one would change the grammar of *every* form to buy one keyword's
//! ergonomics, where a name head costs nothing. `model` is a word a position
//! gives a meaning to, and still a name wherever a name goes, so a relation of
//! that name is unaffected ([`01-grammar.md`](../../../../docs/ein/user/04-reference/01-grammar.md)).
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
//! already says.

use crate::ast::{Ast, Node, NodeId};

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

/// A query's `:over` — which states of R its classes are over
/// ([`12-query.md`](../../../../docs/ein/user/03-syntax-and-semantics/12-query.md#query--one-question-about-r)).
/// All of R by default.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Over {
    #[default]
    All,
    Minimal,
    Maximal,
}

impl Over {
    /// The names `:over` takes, in [`Over`]'s order: the language's words for
    /// a part of R, read in that one place, as a keyword is.
    pub const NAMES: [&'static str; 3] = ["all", "minimal", "maximal"];

    /// Read `:over`'s value, a name (`ein.y`'s `QueryClause`), or say what
    /// is wrong with it: which name, the grammar does not say. A value that is
    /// no name the grammar refuses.
    pub fn parse(ast: &Ast, node: NodeId) -> Result<Over, String> {
        match ast
            .atom_name(node)
            .expect(":over takes a name (ein.y's QueryClause)")
        {
            n if n == Self::NAMES[0] => Ok(Over::All),
            n if n == Self::NAMES[1] => Ok(Over::Minimal),
            n if n == Self::NAMES[2] => Ok(Over::Maximal),
            other => Err(format!(
                "(query …): :over {other} — one of {}",
                Self::NAMES.join(", ")
            )),
        }
    }

    /// `all of R`, `the minimal line`, `the maximal line`.
    pub fn words(self) -> &'static str {
        match self {
            Over::All => "all of R",
            Over::Minimal => "the minimal line",
            Over::Maximal => "the maximal line",
        }
    }
}

/// The goal's atoms: its conjuncts, `(and …)` flattened at any depth
/// ([`Ast::conjuncts`], which a conclusion is read by too), and of an `(or
/// …)` goal every disjunct's ([`goal_disjuncts`]). A class is these atoms
/// under its rows, an `(or …)` goal's the atoms of the disjuncts that hold
/// (Q-M1f.532), so a query's `:expect` may list only facts one of them
/// matches.
pub fn goal_atoms(ast: &Ast, goal: NodeId) -> Vec<NodeId> {
    goal_disjuncts(ast, goal).into_iter().flatten().collect()
}

/// The goal's disjuncts, each as its conjuncts: an `(or d1 … dm)` goal's
/// `d1` to `dm`, in the order written, which is the order the compiler
/// numbers them in; any other goal is one disjunct, its conjuncts. An `(or …)`
/// is a whole goal or none (`refuse_nested_disjunctions`), so no disjunct
/// holds another.
pub fn goal_disjuncts(ast: &Ast, goal: NodeId) -> Vec<Vec<NodeId>> {
    if matches!(ast.node(goal), Node::SForm { .. }) && ast.head_name(goal) == Some("or") {
        return ast
            .form_args(goal)
            .iter()
            .map(|&d| ast.conjuncts(d))
            .collect();
    }
    vec![ast.conjuncts(goal)]
}

/// Whether the ground `fact` is an instance of the goal atom `pattern`: same
/// heads and arity, a variable binding one value throughout, `_` anything.
pub fn matches(ast: &Ast, fact: NodeId, pattern: NodeId) -> bool {
    fn go<'a>(ast: &'a Ast, f: NodeId, p: NodeId, b: &mut Vec<(&'a str, String)>) -> bool {
        match ast.node(p) {
            Node::Var(s) => {
                let name = ast.sym(s);
                let value = render(ast, f);
                match b.iter().find(|(k, _)| *k == name) {
                    Some((_, v)) => *v == value,
                    None => {
                        b.push((name, value));
                        true
                    }
                }
            }
            Node::SForm { head, args } => {
                let Node::SForm {
                    head: fh,
                    args: fargs,
                } = ast.node(f)
                else {
                    return false;
                };
                let (pa, fa) = (ast.args(args), ast.args(fargs));
                pa.len() == fa.len()
                    && go(ast, fh, head, b)
                    && pa.iter().zip(fa).all(|(&x, &y)| go(ast, y, x, b))
            }
            _ => render(ast, f) == render(ast, p),
        }
    }
    go(ast, fact, pattern, &mut Vec::new())
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

/// Read a `:expect` value.
///
/// **Its shape is the grammar's** (`ein.y`'s `Claim`, `Model` and
/// `ClaimFact`): `(false)`, one `(model …)`, or an `(or …)` of one or more,
/// each listing ground facts, stored negations and `neq` facts. Every other
/// shape (a bare atom, `(false …)` with arguments, `(or)`, a disjunct that is
/// not a `(model …)`, `(and …)`, a listed fact that is no fact, a variable or
/// `_` in one) the parse refuses first, at its token, and the loader never
/// reads one.
pub fn parse(ast: &Ast, node: NodeId) -> Expectation {
    let (name, items) = form(ast, node).expect("a claim is a form (ein.y's Claim)");
    match name {
        "false" => Expectation::Contradiction,
        "model" => Expectation::One(Model {
            facts: items.to_vec(),
        }),
        "or" => Expectation::Any(
            items
                .iter()
                .map(|&m| Model {
                    facts: form(ast, m)
                        .expect("a disjunct is a (model …) (ein.y's Models)")
                        .1
                        .to_vec(),
                })
                .collect(),
        ),
        other => unreachable!("a claim is (false), (model …) or (or …), not ({other} …)"),
    }
}

/// A form's head name and its arguments.
fn form(ast: &Ast, node: NodeId) -> Option<(&str, &[NodeId])> {
    let Node::SForm { head, args } = ast.node(node) else {
        return None;
    };
    Some((ast.atom_name(head)?, ast.args(args)))
}

/// Split a listed fact: a ground fact form, `(not F)` of one, or a `neq` fact
/// (`ein.y`'s `ClaimFact`).
///
/// Ground is the whole of the rule: an expectation is an answer, and an answer
/// with a `?var` in it is a pattern. A variable here would silently match
/// anything, which is the one outcome this form exists to prevent; the grammar
/// refuses one, a claim's fact holding names and facts.
pub fn fact<'a>(ast: &'a Ast, node: NodeId) -> ExpectFact<'a> {
    let (negated, inner) = match form(ast, node) {
        Some(("not", [one])) => (true, *one),
        _ => (false, node),
    };
    let (relation, _) = form(ast, inner).expect("a listed fact is a form (ein.y's ClaimFact)");
    ExpectFact {
        negated,
        relation,
        rendered: render(ast, node),
    }
}

/// An expectation fact as its canonical s-expression.
///
/// **This must agree with `ein_infer::events::sexpr` byte for byte**, since
/// that is what the comparison holds it against — a fact is compared by
/// *content*, never by `FactId`, because two runs do not share an interner.
/// `ein-infer`'s `rendering_agrees_with_the_fact_dump`
/// is what keeps the two honest.
pub fn render(ast: &Ast, node: NodeId) -> String {
    match ast.node(node) {
        // An atom is the name it spells once loaded, so it prints as the one
        // printer prints that name: bare where it lexes as itself, else
        // quoted, which is what `Terms::display` prints for the interned
        // value. The grammar refuses a string in a claim.
        Node::Atom(s) => ein_core::text::symbol(ast.sym(s)).into_owned(),
        Node::Str(_) => unreachable!("a claim's fact holds no string (ein.y's ClaimFact)"),
        Node::Var(s) => format!("?{}", ast.sym(s)),
        Node::Keyword(s) => format!(":{}", ast.sym(s)),
        Node::KwPair { key, value } => format!("{} {}", render(ast, key), render(ast, value)),
        Node::SForm { head, args } => {
            // `(q)`, no trailing space — `events::sexpr`'s nullary case, and
            // a head as the one printer prints a fact's ([`ein_core::text::form`]).
            let inner: Vec<String> = ast.args(args).iter().map(|&a| render(ast, a)).collect();
            match ast.node(head) {
                Node::Atom(s) => ein_core::text::form(ast.sym(s), &inner),
                _ if inner.is_empty() => format!("({})", render(ast, head)),
                _ => format!("({} {})", render(ast, head), inner.join(" ")),
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

    fn parsed(src: &str) -> Expectation {
        let (ast, node) = value(src);
        parse(&ast, node)
    }

    // The shapes a claim may not take are rows of the grammar's table
    // (`ein-ir/tests/grammar_pages.rs`): a variable or `_` in a fact, a bare
    // `false`, `(false A)`, `(and …)`, `(models …)`, `(or)`, a disjunct that
    // is no `(model …)`, and `(not …)` of two facts.

    #[test]
    fn the_three_shapes() {
        let bottom = parsed("(query :goal (p ?x) :expect (false))");
        assert!(matches!(bottom, Expectation::Contradiction));
        assert_eq!(bottom.verdict_name(), "Contradiction");

        let one = parsed("(query :goal (p ?x) :expect (model (p A) (not (p B))))");
        assert_eq!(one.models().len(), 1);
        assert_eq!(one.models()[0].facts.len(), 2);
        assert_eq!(one.verdict_name(), "Solution");

        let any = parsed("(query :goal (p ?x) :expect (or (model (p A)) (model (p B))))");
        assert_eq!(any.models().len(), 2);
        assert_eq!(any.verdict_name(), "Ambiguity");
    }

    /// `(or …)` with one disjunct is a `Solution`, not an `Ambiguity` — `k` is
    /// the number of disjuncts and 1 is a legal number of them.
    #[test]
    fn a_single_disjunct_implies_solution() {
        let any = parsed("(query :goal (p ?x) :expect (or (model (p A))))");
        assert_eq!(any.verdict_name(), "Solution");
    }

    #[test]
    fn the_empty_model_is_not_false() {
        let empty = parsed("(query :goal (p ?x) :expect (model))");
        assert_eq!(empty.models()[0].facts.len(), 0);
        assert_eq!(
            empty.verdict_name(),
            "Solution",
            "`(model)` expects a model with nothing in the relations it names — \
             which is no relations. `(false)` is the contradiction."
        );
    }

    #[test]
    fn a_fact_splits_into_relation_polarity_and_text() {
        let (ast, node) = value("(query :goal (p ?x) :expect (model (p A 3) (not (q B))))");
        let Expectation::One(m) = parse(&ast, node) else {
            panic!("one model");
        };
        let pos = fact(&ast, m.facts[0]);
        assert!(!pos.negated);
        assert_eq!(pos.relation, "p");
        assert_eq!(pos.rendered, "(p A 3)");
        let neg = fact(&ast, m.facts[1]);
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
        let Expectation::One(m) = parse(&ast, node) else {
            panic!("one model");
        };
        assert_eq!(fact(&ast, m.facts[0]).rendered, "(q)");
    }
}
