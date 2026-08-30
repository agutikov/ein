//! The IR contract — what ein-lang *is*, asserted without an oracle.
//!
//! Stage **T1a.10.2.2** of
//! [S1a.10.2](../../../../docs/history/m1a_rust/README.md#s1a102--port-the-python-test-suite).
//! This file replaces the semantics half of three Python test files, which are
//! deleted with `ein.py`:
//!
//! | Python original | what it owned |
//! |---|---|
//! | `ein.py/tests/test_ir_parser.py` | the grammar's accept/reject surface |
//! | `ein.py/tests/test_ir_ast.py` | lowering + the `dump ∘ parse` round trip |
//! | `ein.py/tests/ir/test_macros.py` | expansion, and the names a macro may not take |
//!
//! Its sibling `parse_parity.rs` / `dump_parity.rs` asked ein.py the same
//! questions and diffed the answers; those go when the oracle goes. What is
//! written down here is the *answer*, blessed against ein.py while it was
//! still there — so the language survives the deletion of the implementation
//! that defined it.
//!
//! Nothing here reaches into a parser internal. Everything is stated as
//! "this text parses / does not", "the node at this position is *this* kind",
//! "the error is at this line and column", or "this is what it dumps back to"
//! — all four of which are things a puzzle author can observe.

use ein_core::Terms;
use ein_ir::macros::{collect_macros, expand_rule_clauses};
use ein_ir::{Ast, Node, NodeId, ParseError, dump_canonical, dump_compact, parse};

// ── Plumbing ───────────────────────────────────────────────────────

/// Every fixture is named `<demo>` so an asserted message is stable.
const FILE: &str = "<demo>";

fn parse_ok(ast: &mut Ast, text: &str) -> Vec<NodeId> {
    parse(ast, text, Some(FILE)).unwrap_or_else(|e| panic!("{text:?} should parse:\n{e}"))
}

/// The single top-level form `text` must lower to.
fn one_form(ast: &mut Ast, text: &str) -> NodeId {
    let forms = parse_ok(ast, text);
    assert_eq!(forms.len(), 1, "{text:?} is not one form");
    forms[0]
}

/// The error `text` must produce, or a panic naming what it parsed to.
fn rejected(text: &str) -> ParseError {
    let mut ast = Ast::new();
    match parse(&mut ast, text, Some(FILE)) {
        Err(e) => e,
        Ok(forms) => panic!(
            "{text:?} should be a parse error; it parsed as {:?}",
            forms
                .iter()
                .map(|f| dump_compact(&ast, *f))
                .collect::<Vec<_>>()
        ),
    }
}

fn accepts(text: &str) -> bool {
    parse(&mut Ast::new(), text, Some(FILE)).is_ok()
}

/// Every top-level form of `text`, one compact rendering per line — the
/// readable way to say "this is the reading the parser chose".
fn compact_all(text: &str) -> Result<String, ParseError> {
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, Some(FILE))?;
    Ok(forms
        .iter()
        .map(|f| dump_compact(&ast, *f))
        .collect::<Vec<_>>()
        .join("\n"))
}

/// The value of `form`'s `:key` kw-pair.
fn kw_value(ast: &Ast, form: NodeId, key: &str) -> Option<NodeId> {
    ast.form_args(form)
        .iter()
        .copied()
        .find_map(|a| match ast.node(a) {
            Node::KwPair { key: k, value } => match ast.node(k) {
                Node::Keyword(s) if ast.sym(s) == key => Some(value),
                _ => None,
            },
            _ => None,
        })
}

/// `(rule r () :match <clause> :assert (done))` with `decls` in front,
/// expanded the way the loader expands it, and the resulting `:match`.
///
/// Going through a rule rather than calling the expander on a hand-built node
/// is deliberate: `expand_rule_clauses` is the only entry point the loader
/// uses, so this exercises the path a puzzle actually takes.
fn expanded_match(decls: &str, clause: &str) -> (Ast, NodeId) {
    let src = format!("{decls}\n(rule r () :match {clause} :assert (done))");
    let mut ast = Ast::new();
    let forms = parse_ok(&mut ast, &src);
    let macros = collect_macros(&ast, &forms);
    let expanded = expand_rule_clauses(&mut ast, &forms, &macros).expect("expands");
    let rule = *expanded.last().expect("the rule is the last form");
    let m = kw_value(&ast, rule, "match").expect("the rule has a :match");
    (ast, m)
}

/// Load `text` and report either the macro names it registered or the loader's
/// accumulated message.
fn load_macro_names(text: &str) -> Result<Vec<String>, String> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse_ok(&mut ast, text);
    match ein_ir::load(&mut ast, &mut terms, &forms, None) {
        Ok(kb) => Ok(kb
            .program()
            .macros
            .keys()
            .map(|s| terms.sym(s).to_string())
            .collect()),
        Err(e) => Err(e.0),
    }
}

// ── Macros ─────────────────────────────────────────────────────────

/// `macro-param-in-head-position` — a parameter standing where a *relation
/// name* stands is substituted there too.
///
/// Substitution normally rewrites leaves, and a head is not a leaf: it is the
/// field that decides which relation a pattern is about. So the head arm is a
/// separate branch in `substitute`, and forgetting it produces a form still
/// headed `?R` that compiles into a pattern matching nothing — a silent
/// never-fires, not a crash. This is the relation-polymorphic shape the
/// `imply` / `converse` family needs (S1.8.A7); no stdlib macro uses it yet,
/// which is exactly why it needs a test of its own.
#[test]
fn a_macro_parameter_in_head_position_is_substituted() {
    let (ast, m) = expanded_match(
        "(macro apply (?R ?a ?b) (?R ?a ?b))",
        "(apply right-of A B)",
    );
    assert_eq!(dump_compact(&ast, m), "(right-of A B)");
    // Not merely "prints the same": the head is now an atom, so the compiler
    // sees a concrete relation rather than an unbound variable.
    assert_eq!(ast.head_name(m), Some("right-of"));
}

/// `macro-body-invokes-another-macro` — expansion is transitive, not one-shot.
///
/// A single pass would leave `(unknown (lives-in ?a ?b))` in the pattern, and
/// since an unexpanded invocation is a perfectly well-formed *pattern* the
/// rule would compile and then never match. Nothing downstream can notice,
/// so the property has to be pinned here. `maybe` is declared *before* the
/// `unknown` it calls, which also says the registry is collected before any
/// expansion runs rather than in source order.
#[test]
fn a_macro_body_may_invoke_another_macro() {
    let (ast, m) = expanded_match(
        "(macro maybe (?Q) (unknown ?Q))\n\
         (macro unknown (?P) (and (absent ?P) (absent (not ?P))))",
        "(maybe (lives-in ?a ?b))",
    );
    assert_eq!(
        dump_compact(&ast, m),
        "(and (absent (lives-in ?a ?b)) (absent (not (lives-in ?a ?b))))"
    );
}

/// `reserved-macro-names-beyond-absent` — the loader's shadowing check covers
/// the whole kernel vocabulary, not just `absent`.
///
/// [`ein_core::RESERVED`] is **nine** names, and four of them (`not` / `and` /
/// `or` / `neq`) cannot even be *written* as a macro name — the grammar stops
/// them first (see `a_reserved_keyword_cannot_be_written_as_a_macro_name`).
/// The other five are ordinary SYMBOLs, so nothing but this check stands
/// between a puzzle and a macro called `eq` silently shadowing the equality
/// predicate in every rule that mentions it. The second half of the test is
/// what makes the claim "exactly": `forall` and `unknown` are kernel *sugar*
/// and look just as reserved, but they are stdlib macros — a puzzle must be
/// free to define its own.
///
/// **`open` is the fifth, and it was missing here until M1e S1e.2.1.** M1d
/// S1d.2.3 made it the verdict atom and added it to `RESERVED`; this list, the
/// sentence above it (which still called `open` a stdlib macro) and
/// `imports.rs`'s second copy of `RESERVED` all stayed at eight. The last of
/// those three was [`CO-H2`], a shipped defect; the first two were only ever
/// this comment being wrong, which is the same drift one layer out.
///
/// [`CO-H2`]: ../../../../plans/m1e_review_processing/review/correctness/high.md
#[test]
fn the_kernel_names_a_macro_may_not_shadow() {
    for name in ["absent", "eq", "false", "open", "relation"] {
        let src = format!("(macro {name} (?p) (rel ?p))");
        let err = load_macro_names(&src).expect_err("reserved names are rejected at load");
        assert!(
            err.contains(&format!("macro '{name}' shadows a reserved kernel name")),
            "{name}: {err}"
        );
    }
    for name in ["forall", "unknown", "co"] {
        let src = format!("(macro {name} (?p) (rel ?p))");
        let names = load_macro_names(&src).expect("a non-reserved name loads");
        assert_eq!(names, vec![name.to_string()]);
    }
}

/// `macro-name-may-not-be-a-reserved-keyword` — `(macro not …)` dies at
/// *parse* time, one layer earlier than `(macro absent …)`.
///
/// The two failures look alike from a terminal and are not alike at all:
/// `not` / `and` / `or` / `neq` are excluded from `SYMBOL` by a negative
/// lookahead, so the `macro` production never gets a name and the form is not
/// ein-lang; `absent` is a perfectly good SYMBOL that the loader then refuses.
/// The contrast is the assertion — if the lookahead were dropped these would
/// merely become load errors, and every message about them would move.
#[test]
fn a_reserved_keyword_cannot_be_written_as_a_macro_name() {
    for name in ["not", "and", "or", "neq"] {
        let src = format!("(macro {name} (?p) (rel ?p))");
        let e = rejected(&src);
        assert_eq!((e.file.as_str(), e.line, e.col), (FILE, 1, 8), "{src}");
    }
    // …whereas the SYMBOL-legal reserved names get all the way to the loader.
    assert!(accepts("(macro absent (?p) (rel ?p))"));
}

// ── Lowering ───────────────────────────────────────────────────────

/// `equality-form-lowering` (and `eq-and-wildcard-heads-parse-in-a-pattern`,
/// the `=` half) — `(= X Y)` is its own production and is *not* the `eq`
/// predicate.
///
/// Both spellings exist and they mean different things: `=` builds a stored
/// equality fact, `eq` is a `:where` predicate the matcher evaluates. They
/// would be indistinguishable downstream if `=` lowered to an atom named
/// `eq`, so the test asserts the two lower to different heads as well as
/// asserting the shape of each. The first argument being allowed to be a
/// *form* (`(= (color House-1) Red)`) is the other half — that is the
/// function-application spelling of an attribute, and it is what would break
/// if `eq_fact` took two leaves.
#[test]
fn equality_lowers_to_an_eq_headed_form() {
    let mut ast = Ast::new();

    let f = one_form(&mut ast, "(= (color House-1) Red)");
    assert_eq!(ast.head_name(f), Some("="));
    let args = ast.form_args(f);
    assert_eq!(args.len(), 2, "= is binary");
    assert_eq!(
        ast.head_name(args[0]),
        Some("color"),
        "a form may be an arg"
    );
    assert_eq!(ast.atom_name(args[1]), Some("Red"));

    // Inside a `:match`, identically — `=` is a `list_head`, not only a fact
    // head. No corpus file contains this, which is why it is written down.
    let rule = one_form(&mut ast, "(rule eq-elim () :match (= ?a ?b) :assert ?a)");
    let m = kw_value(&ast, rule, "match").expect("a :match");
    assert_eq!(ast.head_name(m), Some("="));
    let m_args = ast.form_args(m);
    assert!(
        matches!(ast.node(m_args[0]), Node::Var(_)) && matches!(ast.node(m_args[1]), Node::Var(_)),
        "(= ?a ?b) lowers to two Vars"
    );

    // `eq` is a different head, and stays one.
    let p = one_form(&mut ast, "(eq a b)");
    assert_eq!(ast.head_name(p), Some("eq"));
    let e = one_form(&mut ast, "(= a b)");
    assert!(
        !ast.eq_nodes(e, p),
        "`=` must not lower to the `eq` predicate"
    );
}

/// `wildcard-and-var-head-lowering` (and the `_` half of
/// `eq-and-wildcard-heads-parse-in-a-pattern`) — `_` and `?r` in head position
/// stay a `Wildcard` and a `Var`.
///
/// The tempting lowering is an atom named `_` or `?r`, and it would even print
/// back the same. It is wrong where it matters: the compiler asks "is this
/// head concrete?" to decide whether a pattern watches one relation or all of
/// them, and an atom answers yes. So the assertion is on the node *kind*, and
/// the round trip is the second half — a head that survives lowering but not
/// `dump ∘ parse` breaks `--dump-states` and every trace that re-reads its own
/// output.
#[test]
fn a_wildcard_or_var_head_survives_lowering_and_a_round_trip() {
    for (src, wild) in [
        ("(rule any () :match (_ ?a ?b) :assert ?a)", true),
        ("(rule symmetric (?r) :match (?r ?a ?b) :assert ?a)", false),
    ] {
        let mut ast = Ast::new();
        let rule = one_form(&mut ast, src);
        let m = kw_value(&ast, rule, "match").expect("a :match");
        let Node::SForm { head, args } = ast.node(m) else {
            panic!("{src}: the :match is not a form")
        };
        if wild {
            assert!(
                matches!(ast.node(head), Node::Wildcard),
                "{src}: a `_` head must not lower to an atom named `_`"
            );
        } else {
            assert!(
                matches!(ast.node(head), Node::Var(_)),
                "{src}: a `?r` head must not lower to an atom named `?r`"
            );
        }
        let two_vars = ast
            .args(args)
            .iter()
            .all(|a| matches!(ast.node(*a), Node::Var(_)));
        assert!(two_vars && args.len == 2, "{src}: two Var arguments");

        // …and the same after `dump ∘ parse`.
        let text = dump_canonical(&ast, &[rule]);
        let again = one_form(&mut ast, &text);
        assert!(
            ast.eq_nodes(rule, again),
            "{src} did not round-trip: {text}"
        );
    }
}

/// `roundtrip-over-unreached-syntax` — `parse ∘ dump ∘ parse == parse`, and
/// `dump ∘ parse` is a text fixed point, over every shape the language admits.
///
/// `dump_parity.rs` already runs this property over the corpus, but the corpus
/// is a *puzzle collection*: it never writes `(= a b)`, never nests `(not …)`
/// twice, never uses a `0..0` cardinality or a negative `:priority`, and has
/// no reason to. Those are precisely the shapes where a dumper regression
/// hides, because nothing else would ever print one. The table below is the
/// forty-seven sources ein.py's `ROUNDTRIP_CASES` enumerated, unique — the
/// original listed `(= (color House-1) Red)` twice.
///
/// (Forty-nine since M1d S1d.2.3 added the two `open` forms.)
///
/// The property is stronger than "it prints something readable": `Loc` lives
/// in a side table specifically so structural equality cannot see a position,
/// and this is the assertion that makes that design load-bearing rather than
/// decorative.
#[test]
fn every_syntactic_shape_survives_dump_then_parse() {
    let cases: [&str; 49] = [
        "(type Person) (type Engineer Person)",
        "(= (color House-1) Red)",
        "(lives-in Norwegian House-1 :source \"condition (10)\")",
        "(symmetric co-located) (implies right-of next-to)",
        "(rule symmetric (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a) :why \"sym\")",
        "(rule t () :match (and (?r ?a ?b) (?r ?b ?c) :where (transitive ?r)) \
         :assert (?r ?a ?c) :why \"tr\")",
        "(query :goal (drinks Water ?h))",
        "(trace (step s1 :rule from-condition :using (c10) \
         :derives (lives-in Norwegian House-1)))",
        "(trace (branch-open s3 :on (lives-in Englishman ?h) :choices (a b c)))",
        // String escapes, the empty string, and non-ASCII.
        "(lives-in a b :source \"tab\\there\")",
        "(lives-in a b :source \"newline\\nhere\")",
        "(lives-in a b :source \"quote\\\"inside\")",
        "(lives-in a b :source \"back\\\\slash\")",
        "(lives-in a b :source \"\")",
        "(lives-in a b :source \"unicode é·»→\")",
        // Range edges, including the degenerate `0..0`.
        "(relation r A B :cardinality 0..0)",
        "(relation r A B :cardinality 0..*)",
        "(relation r A B :cardinality 9999..*)",
        // Four-deep `(and …)`: the width-driven line breaker's worst case.
        "(rule deep () :match (and (and (and (and (rel ?a ?b))))) :assert ?a :why \"d\")",
        // Variable / wildcard heads.
        "(rule var-head (?r) :match (?r ?a ?b) :assert ?a :why \"v\")",
        "(rule wild () :match (_ ?a ?b) :assert ?a :why \"w\")",
        "(rule mixed (?r ?s) :match (?r ?a (?s ?b ?c)) :assert ?a :why \"m\")",
        // Single forms, and the empty `(trace)`.
        "(trace)",
        "(type T)",
        "(rule x () :match a :assert b :why \"x\")",
        // One rule, three kw-pair orderings — order is data, not normalised.
        "(rule p () :match a :assert b :priority 1 :why \"p\")",
        "(rule p () :match a :priority 1 :assert b :why \"p\")",
        "(rule p () :why \"p\" :match a :assert b :priority 1)",
        // Equality, as a fact and as a pattern.
        "(= a b)",
        "(rule eq () :match (= ?a ?b) :assert ?a :why \"e\")",
        // Negation, including the double negation nothing writes.
        "(not (lives-in Spaniard Coffee))",
        "(not (not (lives-in a b)))",
        "(rule n () :match (rel ?a ?b) :assert (not (rel ?b ?a)) :why \"n\")",
        "(not (co-located N H) :rule type-exclusivity :using (s1))",
        "(rule mix () :match (and (or (rel ?a ?b) (rel ?b ?a)) (not (rel ?a ?a)) \
         :where (neq ?a ?b)) :assert ?a :why \"mx\")",
        "(co-located Blue House-2 :rule square-fwd :using (c10 c15))",
        // Integers: zero, negative, wide.
        "(relation r A B :priority 0)",
        "(relation r A B :priority -7)",
        "(relation r A B :priority 9999)",
        // A former declarator head is now an ordinary relation.
        "(instance Norwegian Nationality)",
        "(rule i () :match (instance ?a ?T) :assert ?a :why \"i\")",
        "(macro forall (?b ?G ?B) (absent (and ?G (absent ?B))))",
        "(macro unknown (?P) (and (absent ?P) (absent (not ?P))))",
        "(import std.macro)",
        "(import std.macro :as m)",
        "(import std.macro :symbols (forall unknown))",
        "(rel a.b c.d.e)",
        // The reserved verdict atom, both forms (M1d S1d.2.3). Nullary like
        // `(false)`, and the unary one names the incomplete relation.
        "(rule owes () :match (absent (r a b)) :assert (open) :why \"o\")",
        "(rule owed (?R ?isa) :match (and (relation ?R ?A ?B) (?isa ?a ?A) \
         (absent (and (?isa ?b ?B) (?R ?a ?b)))) :assert (open ?R) :why \"w\")",
    ];

    for src in cases {
        let mut ast = Ast::new();
        let once = parse_ok(&mut ast, src);
        let text = dump_canonical(&ast, &once);
        let twice = parse(&mut ast, &text, Some("<dumped>"))
            .unwrap_or_else(|e| panic!("{src:?} dumped to something unparseable:\n{text}\n{e}"));
        assert_eq!(once.len(), twice.len(), "{src:?}: form count moved");
        for (a, b) in once.iter().zip(&twice) {
            assert!(
                ast.eq_nodes(*a, *b),
                "{src:?} diverged:\n  once: {}\n  twice: {}",
                dump_compact(&ast, *a),
                dump_compact(&ast, *b)
            );
        }
        assert_eq!(
            text,
            dump_canonical(&ast, &twice),
            "{src:?}: the dump is not a text fixed point"
        );
    }
}

// ── What the top level admits ──────────────────────────────────────

/// `reserved-primitives-are-not-top-level-facts` — `and` / `or` / `neq` are
/// pattern combinators; `not` is also a fact.
///
/// This is a real asymmetry and not an oversight: `(not X)` is a *negative
/// assertion*, a thing a puzzle states, while a conjunction or a disequality
/// is only ever a question asked of a match. The three rejected ones are
/// SYMBOL-excluded, so they cannot even fall through to a generic fact — and
/// the caret lands on the reserved word itself, at column 2, which is the
/// diagnostic a puzzle author sees when they try to write a conjunction as a
/// fact.
#[test]
fn and_or_and_neq_are_not_top_level_facts_but_not_is() {
    for src in ["(and a b)", "(or a b)", "(neq a b)"] {
        let e = rejected(src);
        assert_eq!((e.file.as_str(), e.line, e.col), (FILE, 1, 2), "{src}");
        assert!(
            e.to_string().starts_with("<demo>:1:2: unexpected input"),
            "{src}: {e}"
        );
    }
    let mut ast = Ast::new();
    let f = one_form(&mut ast, "(not (co-located Spaniard Coffee))");
    assert_eq!(ast.head_name(f), Some("not"));
    assert_eq!(ast.form_args(f).len(), 1);
}

/// `reserved-primitive-arity-is-pinned` — `not` is unary and `neq` is binary,
/// by *grammar*.
///
/// Which layer rejects them is the point. A loader could check arity too, but
/// then `(not a b)` would parse, reach the compiler as a well-formed tree and
/// be refused with a message about a knowledge base rather than about syntax;
/// worse, a `(neq ?a)` buried in a `:where` would have to be found by a walk
/// nobody writes. Pinning it in the grammar makes both a position in the file.
/// `(not A :k 1)` is the boundary case that says the rule is about *values*,
/// not about argument count — trailing provenance kw-pairs are still allowed.
#[test]
fn not_is_unary_and_neq_is_binary_by_grammar() {
    for (src, line, col) in [
        ("(not a b)", 1, 8),
        ("(neq Norwegian)", 1, 2),
        (
            "(rule x () :match (and (?r ?a ?b) :where (neq ?a)) :assert ?a)",
            1,
            49,
        ),
    ] {
        let e = rejected(src);
        assert_eq!((e.file.as_str(), e.line, e.col), (FILE, line, col), "{src}");
    }
    assert!(accepts("(not a)"));
    assert!(accepts("(not A :k 1)"), "kw-pairs are not arguments");
    assert!(accepts(
        "(rule x () :match (and (?r ?a ?b) :where (neq ?a ?b)) :assert ?a)"
    ));
}

/// `only-a-parenthesised-form-is-top-level` — the file is a sequence of
/// `( … )` and nothing else.
///
/// A bare atom at top level is the mistake this rules out: a stray identifier
/// on its own line reads to a human as a declaration of something, and a
/// grammar that quietly accepted it would produce a nullary fact nobody meant.
/// The same goes for a keyword, a var, a literal, or a `)` that closes nothing.
/// All six report column 1 — the caret sits on the token itself, not at the
/// end of the file, which is what makes the message actionable.
#[test]
fn the_top_level_admits_only_parenthesised_forms() {
    for src in ["Norwegian", ":rule", "?house", "42", "\"hello\"", ")"] {
        let e = rejected(src);
        assert_eq!((e.file.as_str(), e.line, e.col), (FILE, 1, 1), "{src}");
    }
    // Wrapped in parens, the very same atom is a legal nullary fact.
    assert!(accepts("(Norwegian)"));
}

/// `a-keyword-is-never-a-value-or-a-head` — the `:key value` alternation is
/// strict in both directions.
///
/// `KEYWORD` is deliberately absent from the `?value` production, and that one
/// omission is what makes `(query :goal :solve)` an error rather than a query
/// whose goal is the keyword `:solve`. Without it a dropped argument would
/// silently swallow the *next* keyword and the form would still parse, one
/// kw-pair short — the worst kind of typo, because the file still loads.
#[test]
fn a_keyword_is_never_a_value_nor_a_head() {
    for src in [
        "(rule x () :match (?var :key :key) :assert ?var)",
        "(query :goal :solve)",
        "(rule x () :match (:foo :bar value) :assert _)",
    ] {
        rejected(src);
    }
    // The shape that *is* legal, for contrast.
    let mut ast = Ast::new();
    let f = one_form(&mut ast, "(foo :bar baz)");
    let Node::KwPair { key, value } = ast.node(ast.form_args(f)[0]) else {
        panic!("(foo :bar baz) should lower to one KwPair")
    };
    assert!(matches!(ast.node(key), Node::Keyword(s) if ast.sym(s) == "bar"));
    assert_eq!(ast.atom_name(value), Some("baz"));
}

// ── The declarators ────────────────────────────────────────────────

/// `macro-declarator-shape` — `(macro NAME (?p…) BODY)`, with `VAR+` params
/// and exactly one body.
///
/// A zero-parameter macro would be a plain alias, and the grammar refuses it
/// (`macro_params` is `VAR+`) rather than leaving a constant-folding path
/// nobody implements. The body is any `value`, a bare var included — which is
/// worth an accept case, because "one body" and "a *form* body" are different
/// rules and only the first is true. `macro` being SYMBOL-excluded is what
/// makes the four rejections *errors* instead of facts headed `macro`.
#[test]
fn the_macro_declarator_needs_one_parameter_and_one_body() {
    for src in [
        "(macro forall (?b ?G ?B) (absent (and ?G (absent ?B))))",
        "(macro unknown (?P) (and (absent ?P) (absent (not ?P))))",
        "(macro id (?x) ?x)",
    ] {
        assert!(accepts(src), "{src} should parse");
    }
    for src in [
        "(macro foo () (rel ?a))",
        "(macro foo (?p))",
        "(macro)",
        "(macro ?x ?y)",
    ] {
        rejected(src);
    }
}

/// `import-declarator-shape` — `(import MODULE …)` needs the module SYMBOL,
/// and a dotted module name is *one* atom.
///
/// `std.macro` looks like three tokens and is one: `.` is a SYMBOL character,
/// so nothing in the frontend ever splits a qualified name. That matters
/// because the resolver turns the atom into a path and the qualifier prefix
/// into part of every imported name — a lexer that split on `.` would make
/// both jobs guesswork.
#[test]
fn the_import_declarator_needs_a_module_symbol() {
    for src in [
        "(import std.macro)",
        "(import std.macro :as m)",
        "(import std.macro :symbols (forall unknown))",
    ] {
        assert!(accepts(src), "{src} should parse");
    }
    rejected("(import)");

    let mut ast = Ast::new();
    let f = one_form(&mut ast, "(import std.macro :as sg)");
    assert_eq!(
        ast.atom_name(ast.form_args(f)[0]),
        Some("std.macro"),
        "a dotted module name is one atom"
    );
}

/// `rule-and-query-declarator-shape` — a rule needs its parameter list *and*
/// at least one kw-pair; a query needs at least one kw-pair.
///
/// `(rule x ())` is the interesting rejection: it is a syntactically complete
/// declaration of a rule that does nothing, and accepting it would put a
/// no-op in the rule registry that fires on every pass. `kw_pair+` refuses it
/// at the grammar. The contrast case is `(config)`, whose kw-pairs are `*` —
/// an empty config block legitimately means "all defaults", so the two
/// declarators differ by exactly one character of grammar and that difference
/// is observable.
#[test]
fn a_rule_needs_params_and_a_query_needs_a_kw_pair() {
    for src in [
        "(rule x :match a :assert b)",
        "(rule x)",
        "(rule x ())",
        "(hrule x)",
        "(query)",
    ] {
        rejected(src);
    }
    for src in [
        "(rule x () :match a :assert b)",
        "(hrule x () :match a :assert b)",
        "(query :goal X)",
        "(config)",
    ] {
        assert!(accepts(src), "{src} should parse");
    }
}

/// `trace-is-shape-pinned-only-inside-itself` — `(trace …)` admits only trace
/// events; the same heads at top level are ordinary facts.
///
/// A trace is ein-lang the *engine* writes, so its interior is the one place
/// where a typo should be a parse error rather than an unknown relation: a
/// mis-spelled event in a re-read trace would otherwise become a fact and the
/// replay would diverge silently. Outside `(trace …)` no such guarantee is
/// wanted — `step` is a perfectly good relation name for a puzzle about
/// staircases — and the grammar reflects that by SYMBOL-excluding `trace` but
/// not its event heads.
#[test]
fn trace_events_are_shape_pinned_only_inside_a_trace() {
    rejected("(trace (foo s1 :rule x))");
    assert!(accepts("(trace)"));
    assert!(accepts(
        "(trace\n\
           (step s1 :rule from-condition :using (c10)\n\
                    :derives (lives-in Norwegian House-1))\n\
           (branch-open s3 :on (lives-in Englishman ?h) :choices (s3_1 s3_2))\n\
           (contradiction c-branch :using (s3_1) :assumption s3_1)\n\
           (branch-close s3 :choose s3_2)\n\
           (symmetry-class sc1 :over (House-1 House-2) :note \"numbering\"))"
    ));

    // The identical heads at top level are generic facts.
    let mut ast = Ast::new();
    for (src, head) in [
        ("(step s1 :rule x)", "step"),
        ("(branch-open s1 :on X)", "branch-open"),
    ] {
        let f = one_form(&mut ast, src);
        assert_eq!(ast.head_name(f), Some(head));
    }
}

/// `any-non-declarator-head-is-a-fact` — the declarator set is closed, and
/// everything outside it is a fact with unchecked arity.
///
/// "Detect facts by *not* being reserved" is the whole flat-top-level design,
/// and it is falsifiable in two directions. Outward: an unknown head, a former
/// wrapper (`ontology`), a former declarator (`a-priori`, `instance`) all
/// parse, at any arity — arity belongs to the loader, which can say which
/// relation was declared with what signature. Inward: each of the seven
/// SYMBOL-excluded words cannot be a fact head at all, so `(query a b c)` is a
/// syntax error rather than a fact named `query`. `relation` is the documented
/// exception — it stays a plain SYMBOL so rules can match `(relation ?R ?A ?B)`
/// — and it is the one word in the set that `(w a b c)` accepts.
#[test]
fn any_head_outside_the_declarator_set_is_a_generic_fact() {
    let mut ast = Ast::new();
    for (src, head, arity) in [
        ("(unknown-head a b c)", "unknown-head", 3),
        ("(ontology :foo bar)", "ontology", 1),
        ("(a-priori right-of House House)", "a-priori", 3),
        ("(instance Norwegian)", "instance", 1),
        ("(instance Norwegian Nationality Spaniard)", "instance", 3),
    ] {
        let f = one_form(&mut ast, src);
        assert_eq!(ast.head_name(f), Some(head), "{src}");
        assert_eq!(ast.form_args(f).len(), arity, "{src}: arity is unchecked");
    }
    for word in [
        "rule", "hrule", "query", "config", "macro", "import", "trace",
    ] {
        let src = format!("({word} a b c)");
        assert!(
            !accepts(&src),
            "{src} parsed — a declarator must not fall through to a generic fact"
        );
    }
    assert!(
        accepts("(relation a b c)"),
        "`relation` is the one declarator that is also a SYMBOL"
    );
}

// ── The ambiguities Lark resolved implicitly ───────────────────────

/// `the-documented-lark-ambiguities` — the seventy-eight cases where the
/// reading is not obvious, with the reading itself written down.
///
/// ein.py parsed ein-lang with Earley plus a dynamic lexer, which never
/// produced a token stream: at every position it offered every terminal that
/// matched and let the parser explore. `parse.rs` reproduces that with
/// backtracking, and this table is the difference between the two approaches
/// made visible — `(rulex (?a) :match X :assert Y)` is a rule named `x`
/// because the split reading is the only one that parses, while `(rulex A)` is
/// a fact named `rulex` because it is not. Eight more literals behave the same
/// way, and the boundary cases either side (`(rule-x A)`, `(rule_x …)`,
/// `(std.rule X)`) say where the effect stops.
///
/// Asserting the **compact dump** rather than mere acceptance is what makes
/// this a test of the *reading*: a parser that accepted `(notx)` as a fact
/// named `notx` would pass an accept/reject table and fail here.
///
/// Every expectation below was produced by ein.py through
/// `utils/ir_oracle.py` while it still existed; `parse_parity.rs` compared
/// them live and goes when the oracle goes.
#[test]
fn the_documented_ambiguities_resolve_the_way_lark_did() {
    // `None` = a parse error. `Some(s)` = accepted, dumping to `s`.
    let cases: [(&str, Option<&str>); 78] = [
        // A reserved word is a legal *prefix* of a SYMBOL, and the split
        // reading wins wherever it parses.
        (
            "(rulex (?a) :match X :assert Y)",
            Some("(rule x (?a) :match X :assert Y)"),
        ),
        ("(rulex A)", Some("(rulex A)")),
        (
            "(hrulex (?a) :match X :assert Y)",
            Some("(hrule x (?a) :match X :assert Y)"),
        ),
        ("(macrox (?a) B)", Some("(macro x (?a) B)")),
        ("(importx :as m)", Some("(import x :as m)")),
        ("(importfoo.bar :as m)", Some("(import foo.bar :as m)")),
        ("(notx)", Some("(not x)")),
        ("(notx A)", Some("(notx A)")),
        ("(a (notx))", Some("(a (not x))")),
        ("(a (andx A B))", Some("(a (and x A B))")),
        ("(a (orx A B))", Some("(a (or x A B))")),
        ("(a (neqx A B))", Some("(a (neqx A B))")),
        ("(relationx R A B)", Some("(relation x R A B)")),
        ("(relationx R (T1 T2))", Some("(relationx R (T1 T2))")),
        ("(queryx :goal Y)", Some("(queryx :goal Y)")),
        ("(configx)", Some("(configx)")),
        ("(tracex)", Some("(tracex)")),
        ("(trace (stepx :a b))", Some("(trace (step x :a b))")),
        (
            "(trace (branch-openx :a b))",
            Some("(trace (branch-open x :a b))"),
        ),
        ("(trace (branch-refx))", Some("(trace (branch-ref x))")),
        (
            "(trace (contradictionx :a b))",
            Some("(trace (contradiction x :a b))"),
        ),
        (
            "(trace (symmetry-classx :a b))",
            Some("(trace (symmetry-class x :a b))"),
        ),
        ("(a (stepx :a b))", Some("(a (stepx :a b))")),
        // …but only at a word boundary.
        ("(rule-x A)", None),
        ("(rule.x A)", None),
        (
            "(rule_x (?a) :match X :assert Y)",
            Some("(rule_x (?a) :match X :assert Y)"),
        ),
        ("(not_a X)", Some("(not_a X)")),
        ("(neq_test X)", Some("(neq_test X)")),
        ("(std.rule X)", Some("(std.rule X)")),
        ("(relation-x R)", Some("(relation-x R)")),
        // `relation` is the one declarator that is also a SYMBOL.
        ("(relation R A B)", Some("(relation R A B)")),
        ("(relation R (T1 T2))", Some("(relation R (T1 T2))")),
        ("(relation)", Some("(relation)")),
        ("(relation R)", Some("(relation R)")),
        // `value*` then `kw_pair*`, and the interleaving `list_item*` allows
        // *inside* a form but not at top level.
        ("(a 1 :k 2)", Some("(a 1 :k 2)")),
        ("(a :k 1 :k 2)", Some("(a :k 1 :k 2)")),
        ("(a :k 1 2)", None),
        ("(a 1 :k 2 3)", None),
        ("(x (a 1 :k 2 3))", Some("(x (a 1 :k 2 3))")),
        ("(x (a :k 2 3))", Some("(x (a :k 2 3))")),
        // Arity-pinned shapes — and the same shapes one level in, where
        // `generic_list` accepts what `eq_fact` refuses.
        ("(= a b)", Some("(= a b)")),
        ("(= a b c)", None),
        ("(= a b :k 1)", Some("(= a b :k 1)")),
        ("(x (= a b c))", Some("(x (= a b c))")),
        ("(not A B)", None),
        ("(not A :k 1)", Some("(not A :k 1)")),
        ("(query)", None),
        ("(config)", Some("(config)")),
        ("(trace)", Some("(trace)")),
        ("(rule r ())", None),
        ("(rule r () :match X)", Some("(rule r () :match X)")),
        ("(macro m () B)", None),
        ("(macro m (?a) B)", Some("(macro m (?a) B)")),
        // Terminals: `_` is the wildcard and nothing longer is; an integer
        // canonicalises; a RANGE beats an INT; a string keeps its body.
        ("(a __closed__)", Some("(a __closed__)")),
        ("(a _)", Some("(a _)")),
        ("(a _x)", None),
        ("(a 007)", Some("(a 7)")),
        ("(a -0)", Some("(a 0)")),
        ("(a 1..5)", Some("(a 1..5)")),
        ("(a 1..*)", Some("(a 1..*)")),
        ("(a 1..)", None),
        ("(a \"h\\di\")", Some("(a \"hdi\")")),
        ("(a \"a\nb\")", Some("(a \"a\\nb\")")),
        ("(a \"a\\\nb\")", None),
        ("(a \"unterminated)", None),
        ("(x (?p ?q))", Some("(x (?p ?q))")),
        ("(x (_ ?a))", Some("(x (_ ?a))")),
        // `()` is a *value*, not a top-level form — and it cannot head one.
        ("()", None),
        ("(x ())", Some("(x ())")),
        ("(x (()))", None),
        // Trivia, including the unterminated block comment that is not trivia.
        ("(x ; comment\n y)", Some("(x y)")),
        ("(x #| c |# y)", Some("(x y)")),
        ("(x #| never closed", None),
        ("", Some("")),
        ("   ", Some("")),
        (";; only a comment\n", Some("")),
        ("(a))", None),
        ("(a", None),
    ];

    let mut bad = Vec::new();
    for (src, want) in cases {
        match (compact_all(src), want) {
            (Ok(got), Some(want)) if got == want => {}
            (Err(_), None) => {}
            (Ok(got), Some(want)) => {
                bad.push(format!("{src:?}\n  want: {want:?}\n  got:  {got:?}"))
            }
            (Ok(got), None) => bad.push(format!("{src:?}\n  want: a parse error\n  got:  {got:?}")),
            (Err(e), Some(want)) => bad.push(format!("{src:?}\n  want: {want:?}\n  got:  {e}")),
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {} cases differ:\n{}",
        bad.len(),
        cases.len(),
        bad.join("\n")
    );
}
