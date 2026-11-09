//! The IR contract — what ein-lang *is*, written down as answers.
//!
//! Stage **T1a.10.2.2** of
//! [S1a.10.2](../../../../docs/history/m1a_rust/README.md#s1a102--the-behaviour-suite).
//! Three surfaces: the grammar (what is accepted and what is rejected),
//! lowering (the node a text becomes, and the `dump ∘ parse` round trip), and
//! macros (expansion, and the names a macro may not take).
//!
//! What is written down here is the *answer*, so the language is stated as
//! data rather than as whatever the parser happens to do.
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
    let macros = collect_macros(&ast, &forms).expect("the fixture declares no bad macro");
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
/// [`ein_core::RESERVED`] is **eleven** names, and four of them (`not` / `and`
/// / `or` / `neq`) cannot even be *written* as a macro name — the grammar stops
/// them first (see `a_reserved_keyword_cannot_be_written_as_a_macro_name`).
/// The other seven are ordinary SYMBOLs, so nothing but this check stands
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
/// **The kernel's two dunders are the sixth and the seventh since M1f
/// S1f.5.5's T1f.5.5.11** (Q-M1f.7 (b)): `__closed__` and `__symmetric__` are
/// used, never bound.
///
/// **Since M1f S1f.5.5's swap (T1f.5.5.4) only the two dunders reach this
/// check**: the other five are words a position gives a meaning to, tokens of
/// their own that no declarator takes (the paper grammar's default 1), so a
/// macro named one is refused at the parse, at the name, as `not` is (the next
/// test).
///
/// [`CO-H2`]: ../../../../docs/history/m1e_review_processing/review/correctness/high.md
#[test]
fn the_kernel_names_a_macro_may_not_shadow() {
    for name in ["absent", "eq", "false", "open", "relation"] {
        let src = format!("(macro {name} (?p) (rel ?p))");
        let e = rejected(&src);
        assert_eq!(
            (e.position(), e.errors[0].what.as_str()),
            ((1, 8), format!("unexpected `{name}`").as_str()),
            "{src}"
        );
    }
    for name in ["__closed__", "__symmetric__"] {
        let src = format!("(macro {name} (?p) (rel ?p) :why \"rel holds of {{?p}}\")");
        let err = load_macro_names(&src).expect_err("reserved names are rejected at load");
        assert!(
            err.contains(&format!("macro '{name}' shadows a reserved kernel name")),
            "{name}: {err}"
        );
    }
    for name in ["forall", "unknown", "co"] {
        let src = format!("(macro {name} (?p) (rel ?p) :why \"rel holds of {{?p}}\")");
        let names = load_macro_names(&src).expect("a non-reserved name loads");
        assert_eq!(names, vec![name.to_string()]);
    }
}

/// `macro-name-may-not-be-a-reserved-keyword` — `(macro not …)` dies at
/// *parse* time, one layer earlier than `(macro __closed__ …)`.
///
/// The two failures look alike from a terminal and are not alike at all:
/// `not` / `and` / `or` / `neq` are never a name, so the `macro` production
/// never gets one and the form is not ein-lang; `__closed__` is a perfectly
/// good name that the loader then refuses. Until M1f S1f.5.5's swap
/// (T1f.5.5.4) the contrast was `absent`, which the hand-written lexer read as
/// a `SYMBOL`; since, `absent` is a token of its own that no declarator takes,
/// refused at the parse too (the test above), and the contrast is a dunder.
#[test]
fn a_reserved_keyword_cannot_be_written_as_a_macro_name() {
    for name in ["not", "and", "or", "neq"] {
        let src = format!("(macro {name} (?p) (rel ?p))");
        let e = rejected(&src);
        assert_eq!(
            (e.file.as_str(), e.position().0, e.position().1),
            (FILE, 1, 8),
            "{src}"
        );
    }
    // …whereas a name the grammar reads gets all the way to the loader.
    assert!(accepts("(macro __closed__ (?p) (rel ?p))"));
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
    // Its conclusion is `(q ?a)` since M1f S1f.5.5's swap, which refuses a
    // variable as a whole conclusion (G-14, Q-M1f.1 (b)); it was `?a`.
    let rule = one_form(
        &mut ast,
        "(rule eq-elim () :match (= ?a ?b) :assert (q ?a))",
    );
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
/// `eq-and-wildcard-heads-parse-in-a-pattern`) — `?r` in head position stays
/// a `Var`, and until M1f S1f.5.5's swap `_` stayed a `Wildcard`.
///
/// The tempting lowering is an atom named `?r`, and it would even print back
/// the same. It is wrong where it matters: the compiler asks "is this head
/// concrete?" to decide whether a pattern watches one relation or all of
/// them, and an atom answers yes. So the assertion is on the node *kind*, and
/// the round trip is the second half — a head that survives lowering but not
/// `dump ∘ parse` breaks `--dump-states` and every trace that re-reads its own
/// output. **Since the swap (T1f.5.5.4) a bare `_` is refused at the parse in
/// every position** (S1f.5.1's ruling), so its half is the refusal, at the
/// `_`; and the conclusion is `(q ?a)`, a variable being no whole conclusion
/// (G-14, Q-M1f.1 (b)).
#[test]
fn a_wildcard_or_var_head_survives_lowering_and_a_round_trip() {
    let e = rejected("(rule any () :match (_ ?a ?b) :assert (q ?a))");
    assert_eq!(e.position(), (1, 22), "{e}");
    assert_eq!(e.errors[0].what, "unexpected `_`: _ (reserved)");
    let src = "(rule symmetric (?r) :match (?r ?a ?b) :assert (q ?a))";
    let mut ast = Ast::new();
    let rule = one_form(&mut ast, src);
    let m = kw_value(&ast, rule, "match").expect("a :match");
    let Node::SForm { head, args } = ast.node(m) else {
        panic!("{src}: the :match is not a form")
    };
    assert!(
        matches!(ast.node(head), Node::Var(_)),
        "{src}: a `?r` head must not lower to an atom named `?r`"
    );
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

/// `roundtrip-over-unreached-syntax` — `parse ∘ dump ∘ parse == parse`, and
/// `dump ∘ parse` is a text fixed point, over every shape the language admits.
///
/// `dump_goldens.rs` already runs this property over the corpus, but the corpus
/// is a *puzzle collection*: it never writes `(= a b)`, never writes a
/// decorated name, and has no reason to. Those are precisely the shapes where
/// a dumper regression hides, because nothing else would ever print one.
///
/// **Restated in the final grammar at M1f S1f.5.5's swap (T1f.5.5.4)**: each
/// case the grammar refuses was rewritten to the shape it stands for, so the
/// table still covers what the language admits. A `:where` guard is a
/// positional premise (S1f.5.11); a conclusion is a fact, never `?a` or a name
/// (G-14); a trace takes `step` alone (default 8); `_` is gone (S1f.5.1); a
/// fact's `:using` lists facts (default 2); `:priority` is gone (S1f.5.15), so
/// the three kw-pair orderings are `:match`, `:assert` and `:why`, and its
/// integers are names of digits; `(not (not …))`, which nothing wrote, is
/// refused (`not` negates a fact); a rule named `eq` is refused (`eq` is no
/// declarator's name); and the name rule's decorators and a macro's `:why` are
/// added.
///
/// The property is stronger than "it prints something readable": `Loc` lives
/// in a side table specifically so structural equality cannot see a position,
/// and this is the assertion that makes that design load-bearing rather than
/// decorative.
#[test]
fn every_syntactic_shape_survives_dump_then_parse() {
    let cases: [&str; 46] = [
        "(type Person) (type Engineer Person)",
        "(= (color House-1) Red)",
        "(lives-in Norwegian House-1 :source \"condition (10)\")",
        "(symmetric co-located) (implies right-of next-to)",
        "(rule symmetric (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a) :why \"sym\")",
        "(rule t () :match (and (?r ?a ?b) (?r ?b ?c) (transitive ?r)) \
         :assert (?r ?a ?c) :why \"tr\")",
        "(query :goal (drinks Water ?h))",
        "(trace (step s1 :rule from-condition :using (c10) \
         :derives (lives-in Norwegian House-1)))",
        "(trace (step s3 :rule guess :derives (lives-in Englishman House-3)))",
        // String escapes, the empty string, and non-ASCII.
        "(lives-in a b :source \"tab\\there\")",
        "(lives-in a b :source \"newline\\nhere\")",
        "(lives-in a b :source \"quote\\\"inside\")",
        "(lives-in a b :source \"back\\\\slash\")",
        "(lives-in a b :source \"\")",
        "(lives-in a b :source \"unicode é·»→\")",
        // Four-deep `(and …)`: the width-driven line breaker's worst case.
        "(rule deep () :match (and (and (and (and (rel ?a ?b))))) :assert (q ?a) :why \"d\")",
        // Variable heads.
        "(rule var-head (?r) :match (?r ?a ?b) :assert (q ?a) :why \"v\")",
        "(rule mixed (?r ?s) :match (?r ?a (?s ?b ?c)) :assert (q ?a) :why \"m\")",
        // Single forms, and the empty `(trace)`.
        "(trace)",
        "(type T)",
        "(rule x () :match (a) :assert (b) :why \"x\")",
        // One rule, three kw-pair orderings — order is data, not normalised.
        "(rule p () :match (a) :assert (b) :why \"p\")",
        "(rule p () :match (a) :why \"p\" :assert (b))",
        "(rule p () :why \"p\" :match (a) :assert (b))",
        // Equality, as a fact and as a pattern.
        "(= a b)",
        "(rule equal () :match (= ?a ?b) :assert (q ?a) :why \"e\")",
        // Negation, a nullary one, and one with a sentence.
        "(not (lives-in Spaniard Coffee))",
        "(not (plain))",
        "(not (lives-in a b) :source \"s\")",
        "(rule n () :match (rel ?a ?b) :assert (not (rel ?b ?a)) :why \"n\")",
        "(not (co-located N H) :rule type-exclusivity :using ((s1)))",
        "(rule mix () :match (and (or (rel ?a ?b) (rel ?b ?a)) (not (rel ?a ?a)) \
         (neq ?a ?b)) :assert (q ?a) :why \"mx\")",
        "(co-located Blue House-2 :rule square-fwd :using ((c10) (c15)))",
        // Names of digits: zero, negative, wide, a leading zero.
        "(r 0 -0 -7 9999 007 7)",
        // A former declarator head is now an ordinary relation.
        "(instance Norwegian Nationality)",
        "(rule i () :match (instance ?a ?T) :assert (q ?a) :why \"i\")",
        "(macro forall (?b ?G ?B) (absent (and ?G (absent ?B))))",
        "(macro unknown (?P) (and (absent ?P) (absent (not ?P))) :why \"{?P} is unknown\")",
        "(import std.macro)",
        "(import std.macro :as m)",
        "(import std.macro :symbols (forall unknown))",
        "(rel a.b c.d.e)",
        // The name rule's decorators and brackets.
        "(rel $x [y] {z} it's a/b <= !x @x ~ %)",
        // The empty conclusion.
        "(rule e () :match () :assert () :why \"e\")",
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
/// never a name, so they cannot even fall through to a generic fact — and the
/// error lands on the reserved word itself, at column 2, which is the
/// diagnostic a puzzle author sees when they try to write a conjunction as a
/// fact. Since M1f S1f.5.5's swap (T1f.5.5.4) it names the word, where it said
/// `unexpected input`, and its one repair deletes it.
#[test]
fn and_or_and_neq_are_not_top_level_facts_but_not_is() {
    for (src, word) in [
        ("(and a b)", "and"),
        ("(or a b)", "or"),
        ("(neq a b)", "neq"),
    ] {
        let e = rejected(src);
        assert_eq!(
            (e.file.as_str(), e.position().0, e.position().1),
            (FILE, 1, 2),
            "{src}"
        );
        assert_eq!(
            e.to_string(),
            format!("<demo>:1:2: unexpected `{word}`\n  repair: Delete `{word}`"),
            "{src}"
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
/// worse, a `(neq ?a)` buried in a `:match` would have to be found by a walk
/// nobody writes. Pinning it in the grammar makes both a position in the file.
///
/// Since M1f S1f.5.5's swap (T1f.5.5.4) `not` negates a fact, so `(not a b)`
/// is refused at the bare `a` (the paper grammar's default 3; until then at
/// the `b`, column 8), and a fact's keyword pairs are its provenance, so
/// `(not A :k 1)`, which this test accepted as the boundary case, is refused
/// at the `A`; the boundary is `(not (a) :source "s")`. `(neq Norwegian)` at
/// the top level is refused at `neq`, which no fact takes, and the guard in a
/// `:match` is positional, its `:where` gone (S1f.5.11).
#[test]
fn not_is_unary_and_neq_is_binary_by_grammar() {
    for (src, line, col) in [
        ("(not a b)", 1, 6),
        ("(not A :k 1)", 1, 6),
        ("(neq Norwegian)", 1, 2),
        (
            "(rule x () :match (and (?r ?a ?b) (neq ?a)) :assert (q ?a))",
            1,
            42,
        ),
    ] {
        let e = rejected(src);
        assert_eq!(
            (e.file.as_str(), e.position().0, e.position().1),
            (FILE, line, col),
            "{src}"
        );
    }
    assert!(accepts("(not (a))"));
    assert!(
        accepts("(not (a) :source \"s\")"),
        "kw-pairs are not arguments"
    );
    assert!(accepts(
        "(rule x () :match (and (?r ?a ?b) (neq ?a ?b)) :assert (q ?a))"
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
        assert_eq!(
            (e.file.as_str(), e.position().0, e.position().1),
            (FILE, 1, 1),
            "{src}"
        );
    }
    // Wrapped in parens, the very same atom is a legal nullary fact.
    assert!(accepts("(Norwegian)"));
}

/// `a-keyword-is-never-a-value-or-a-head` — the `:key value` alternation is
/// strict in both directions.
///
/// `KEYWORD` is deliberately absent from every value's production, and that
/// one omission is what makes `(query :goal :solve)` an error rather than a
/// query whose goal is the keyword `:solve`. Without it a dropped argument
/// would silently swallow the *next* keyword and the form would still parse,
/// one kw-pair short — the worst kind of typo, because the file still loads.
///
/// Since M1f S1f.5.5's swap (T1f.5.5.4) each form takes a fixed set of
/// keywords (S1f.5.11), so the legal contrast is a fact's provenance pair,
/// `(foo bar :source "baz")`, where until then it was `(foo :bar baz)`, which
/// is refused at `:bar`; and the third rejection's conclusion is `(q a)`.
#[test]
fn a_keyword_is_never_a_value_nor_a_head() {
    for src in [
        "(rule x () :match (?var :key :key) :assert (q ?var))",
        "(query :goal :solve)",
        "(rule x () :match (:foo :bar value) :assert (q a))",
        "(foo :bar baz)",
    ] {
        rejected(src);
    }
    // The shape that *is* legal, for contrast.
    let mut ast = Ast::new();
    let f = one_form(&mut ast, "(foo bar :source \"baz\")");
    let Node::KwPair { key, value } = ast.node(ast.form_args(f)[1]) else {
        panic!("(foo bar :source \"baz\") should lower to an atom and one KwPair")
    };
    assert!(matches!(ast.node(key), Node::Keyword(s) if ast.sym(s) == "source"));
    assert!(matches!(ast.node(value), Node::Str(s) if ast.sym(s) == "baz"));
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

/// `rule-and-query-declarator-shape` — a rule needs its parameter list, and a
/// query needs at least one kw-pair.
///
/// `(rule x ())` is the interesting case: it is a syntactically complete
/// declaration of a rule that does nothing. Until M1f S1f.5.5's swap
/// (T1f.5.5.4) `kw_pair+` refused it at the grammar; since, the grammar takes
/// a rule's clauses as a list a form may leave empty, and the loader, which
/// says which clause is missing, refuses it (T9's attribution: *the loader
/// asks for `:match` and `:assert`*). The contrast case is `(config)`, whose
/// kw-pairs are `*` — an empty config block legitimately means "all
/// defaults". A name is no premise or conclusion since the swap, so the
/// accepted rules write `(a)` and `(b)`.
#[test]
fn a_rule_needs_params_and_a_query_needs_a_kw_pair() {
    for src in [
        "(rule x :match (a) :assert (b))",
        "(rule x)",
        "(hrule x)",
        "(query)",
    ] {
        rejected(src);
    }
    for src in [
        "(rule x () :match (a) :assert (b))",
        "(hrule x () :match (a) :assert (b))",
        "(query :goal (x))",
        "(config)",
        "(rule x ())",
    ] {
        assert!(accepts(src), "{src} should parse");
    }
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse_ok(
        &mut ast,
        "(rule x () :why \"rule x says nothing: it has neither a :match nor an :assert\")",
    );
    let e = ein_ir::load(&mut ast, &mut terms, &forms, None).expect_err("an empty rule");
    assert_eq!(e.0, "(rule x) missing :match or :assert at None");
}

/// `trace-is-shape-pinned-only-inside-itself` — `(trace …)` admits only trace
/// events; the same heads at top level are ordinary facts.
///
/// A trace is ein-lang the *engine* writes, so its interior is the one place
/// where a typo should be a parse error rather than an unknown relation: a
/// mis-spelled event in a re-read trace would otherwise become a fact and the
/// replay would diverge silently. Outside `(trace …)` no such guarantee is
/// wanted — `step` is a perfectly good relation name for a puzzle about
/// staircases.
///
/// Since M1f S1f.5.5's swap (T1f.5.5.4) a trace takes `step` alone, with its
/// five keywords (the paper grammar's default 8): `branch-open`,
/// `contradiction`, `branch-close` and `symmetry-class`, which no engine
/// output has written since the walk went, are names, refused as an event,
/// and their keyword pairs at the top level as any keyword a fact does not
/// take (S1f.5.11).
#[test]
fn trace_events_are_shape_pinned_only_inside_a_trace() {
    rejected("(trace (foo s1 :rule x))");
    rejected("(trace (branch-open s3 :on (lives-in Englishman ?h) :choices (s3_1 s3_2)))");
    assert!(accepts("(trace)"));
    assert!(accepts(
        "(trace\n\
           (step s1 :rule from-condition :using (c10)\n\
                    :derives (lives-in Norwegian House-1))\n\
           (step s2 :rule adjacent :derives (color-loc Blue House-2)))"
    ));

    // The identical heads at top level are generic facts.
    let mut ast = Ast::new();
    for (src, head) in [
        ("(step s1 :rule x)", "step"),
        ("(branch-open s1 X)", "branch-open"),
    ] {
        let f = one_form(&mut ast, src);
        assert_eq!(ast.head_name(f), Some(head));
    }
    rejected("(branch-open s1 :on X)");
}

/// `any-non-declarator-head-is-a-fact` — the declarator set is closed, and
/// everything outside it is a fact with unchecked arity.
///
/// "Detect facts by *not* being reserved" is the whole flat-top-level design,
/// and it is falsifiable in two directions. Outward: an unknown head, a former
/// wrapper (`ontology`), a former declarator (`a-priori`, `instance`) all
/// parse, at any arity — arity belongs to the loader, which can say which
/// relation was declared with what signature. Inward: each of the seven
/// words that are never a name cannot be a fact head at all, so `(query a b
/// c)` is a syntax error rather than a fact named `query`. `relation` is the
/// documented exception — it is a name wherever a name goes, so rules can
/// match `(relation ?R ?A ?B)` — and it is the one word in the set that `(w a
/// b c)` accepts. Since M1f S1f.5.5's swap (T1f.5.5.4) a fact takes no keyword
/// but its provenance's (S1f.5.11), so the former wrapper's case is
/// `(ontology bar)`, where it was `(ontology :foo bar)`, refused at `:foo`.
#[test]
fn any_head_outside_the_declarator_set_is_a_generic_fact() {
    let mut ast = Ast::new();
    for (src, head, arity) in [
        ("(unknown-head a b c)", "unknown-head", 3),
        ("(ontology bar)", "ontology", 1),
        ("(a-priori right-of House House)", "a-priori", 3),
        ("(instance Norwegian)", "instance", 1),
        ("(instance Norwegian Nationality Spaniard)", "instance", 3),
    ] {
        let f = one_form(&mut ast, src);
        assert_eq!(ast.head_name(f), Some(head), "{src}");
        assert_eq!(ast.form_args(f).len(), arity, "{src}: arity is unchecked");
    }
    assert_eq!(
        rejected("(ontology :foo bar)").errors[0].what,
        "unexpected `:foo`: a keyword this form does not take"
    );
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
        "`relation` is the one declarator that is also a name"
    );
}

// ── The ambiguities the grammar resolves implicitly ────────────────

/// `the-documented-ambiguities` — the seventy-eight cases where the reading
/// is not obvious, with the reading itself written down.
///
/// Until M1f S1f.5.5's swap (T1f.5.5.4) the hand-written parser read these by
/// backtracking, every terminal on offer at every position. Since, the parser
/// is generated from S1f.5.5's grammar and the table records what it reads,
/// case for case: the cases stay, as the record of where the reading is not
/// obvious, and each answer is the one the rulings give. A reserved word is a
/// whole token, so `(rulex A)`, `(rule-x A)` and `(rule.x A)` are facts named
/// as written (the name rule overrides Q-M1e2.17 (i)(a), where the word before
/// `-` or `.` was the keyword); a variable, a string or `_` in a fact, a
/// keyword pair a fact does not take, and `()` or a parameter list where a
/// fact's argument goes are refused (S1f.5.2, S1f.5.11, S1f.5.1, the paper
/// grammar's default 7); a run of digits, a range and `_x` are names, printed
/// as written (S1f.5.4); `=` is a name, so `(= a b c)` is a fact; `not` negates
/// a fact; a trace takes `step` alone (default 8); a name is no premise and
/// no macro body; and a block comment never closed is no token.
///
/// Asserting the **compact dump** rather than mere acceptance is what makes
/// this a test of the *reading*: a parser that accepted `(notx)` as a fact
/// named `notx` would pass an accept/reject table and fail here.
#[test]
fn the_documented_ambiguities_resolve_as_recorded() {
    // `None` = a parse error. `Some(s)` = accepted, dumping to `s`.
    let cases: [(&str, Option<&str>); 78] = [
        // unexpected `?a`: a variable
        ("(rulex (?a) :match X :assert Y)", None),
        ("(rulex A)", Some("(rulex A)")),
        // unexpected `?a`: a variable
        ("(hrulex (?a) :match X :assert Y)", None),
        // unexpected `?a`: a variable
        ("(macrox (?a) B)", None),
        // unexpected `:as`
        ("(importx :as m)", None),
        // unexpected `:as`
        ("(importfoo.bar :as m)", None),
        ("(notx)", Some("(notx)")),
        ("(notx A)", Some("(notx A)")),
        ("(a (notx))", Some("(a (notx))")),
        ("(a (andx A B))", Some("(a (andx A B))")),
        ("(a (orx A B))", Some("(a (orx A B))")),
        ("(a (neqx A B))", Some("(a (neqx A B))")),
        ("(relationx R A B)", Some("(relationx R A B)")),
        ("(relationx R (T1 T2))", Some("(relationx R (T1 T2))")),
        // unexpected `:goal`
        ("(queryx :goal Y)", None),
        ("(configx)", Some("(configx)")),
        ("(tracex)", Some("(tracex)")),
        // unexpected `stepx`: a name
        ("(trace (stepx :a b))", None),
        // unexpected `branch-openx`: a name
        ("(trace (branch-openx :a b))", None),
        // unexpected `branch-refx`: a name
        ("(trace (branch-refx))", None),
        // unexpected `contradictionx`: a name
        ("(trace (contradictionx :a b))", None),
        // unexpected `symmetry-classx`: a name
        ("(trace (symmetry-classx :a b))", None),
        // unexpected `:a`: a keyword this form does not take
        ("(a (stepx :a b))", None),
        ("(rule-x A)", Some("(rule-x A)")),
        ("(rule.x A)", Some("(rule.x A)")),
        // unexpected `?a`: a variable
        ("(rule_x (?a) :match X :assert Y)", None),
        ("(not_a X)", Some("(not_a X)")),
        ("(neq_test X)", Some("(neq_test X)")),
        ("(std.rule X)", Some("(std.rule X)")),
        ("(relation-x R)", Some("(relation-x R)")),
        ("(relation R A B)", Some("(relation R A B)")),
        // unexpected `(`
        ("(relation R (T1 T2))", None),
        // unexpected `)`
        ("(relation)", None),
        ("(relation R)", Some("(relation R)")),
        // unexpected `:k`: a keyword this form does not take
        ("(a 1 :k 2)", None),
        // unexpected `:k`: a keyword this form does not take
        ("(a :k 1 :k 2)", None),
        // unexpected `:k`: a keyword this form does not take
        ("(a :k 1 2)", None),
        // unexpected `:k`: a keyword this form does not take
        ("(a 1 :k 2 3)", None),
        // unexpected `:k`: a keyword this form does not take
        ("(x (a 1 :k 2 3))", None),
        // unexpected `:k`: a keyword this form does not take
        ("(x (a :k 2 3))", None),
        ("(= a b)", Some("(= a b)")),
        ("(= a b c)", Some("(= a b c)")),
        // unexpected `:k`: a keyword this form does not take
        ("(= a b :k 1)", None),
        ("(x (= a b c))", Some("(x (= a b c))")),
        // unexpected `A`: a name
        ("(not A B)", None),
        // unexpected `A`: a name
        ("(not A :k 1)", None),
        // unexpected `)`
        ("(query)", None),
        ("(config)", Some("(config)")),
        ("(trace)", Some("(trace)")),
        ("(rule r ())", Some("(rule r ())")),
        // unexpected `X`: a name
        ("(rule r () :match X)", None),
        // unexpected `)`
        ("(macro m () B)", None),
        // unexpected `B`: a name
        ("(macro m (?a) B)", None),
        ("(a __closed__)", Some("(a __closed__)")),
        // unexpected `_`: _ (reserved)
        ("(a _)", None),
        ("(a _x)", Some("(a _x)")),
        ("(a 007)", Some("(a 007)")),
        ("(a -0)", Some("(a -0)")),
        ("(a 1..5)", Some("(a 1..5)")),
        ("(a 1..*)", Some("(a 1..*)")),
        ("(a 1..)", Some("(a 1..)")),
        // unexpected `"h\di"`: a string with an unknown escape
        ("(a \"h\\di\")", None),
        // unexpected `"a\nb"`: a string
        ("(a \"a\nb\")", None),
        // unexpected `"a\`: a token that is not a name
        ("(a \"a\\\nb\")", None),
        // unexpected `"unterminated`: a token that is not a name
        ("(a \"unterminated)", None),
        // unexpected `?p`: a variable
        ("(x (?p ?q))", None),
        // unexpected `_`: _ (reserved)
        ("(x (_ ?a))", None),
        // unexpected `)`
        ("()", None),
        // unexpected `)`
        ("(x ())", None),
        // unexpected `(`
        ("(x (()))", None),
        ("(x ; comment\n y)", Some("(x y)")),
        ("(x #| c |# y)", Some("(x y)")),
        // no token starts with `#`
        ("(x #| never closed", None),
        ("", Some("")),
        ("   ", Some("")),
        (";; only a comment\n", Some("")),
        // unexpected `)`
        ("(a))", None),
        // unexpected end of input
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
