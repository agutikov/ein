//! What a `:match` compiles to, what it is refused for, and what a
//! `(config …)` block coerces to — T1a.10.2.2.
//!
//! Four subjects, all of them the *semantics* of the front half of the engine:
//!
//! - the in-`:match` kw_pair drop, and S1.22.0's four refusals;
//! - each refusal's message, against its `.expected` fixture;
//! - `SolverConfig` coercion from IR literals (the F-KER-4 regression);
//! - the arity-1 `(relation R)` membership channel and bare declarations.
//!
//! Where a *rendering* exists the assertion is on that rendering —
//! [`ein_infer::plan_shape`] is the compiler's only readable surface.
//! Everywhere else it is on the observable the structure existed for: the
//! facts a rule derives, the candidates the enumerator proposes, the message a
//! rule author reads.
//!
//! The `.expected`-file half is
//! [`every_compile_error_says_what_its_expected_file_says`] below, and the
//! corpus-wide `plan_shape` sweep is `corpus_shapes.md5`'s 78 `::plan` lines.

use std::collections::BTreeSet;
use std::ops::ControlFlow;
use std::path::PathBuf;

use ein_core::{FactId, Kb, SolverConfig, Terms};
use ein_infer::{Events, HypGenStats, Saturator, Session, SharedMemo, events::sexpr};
use ein_ir::{Ast, parse};

// ── Harness ────────────────────────────────────────────────────────

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

/// One loaded fixture, arenas and all. The three travel together because a
/// `Kb`'s symbols are indices into its `Terms` and its rule bodies into its
/// `Ast`: none of them means anything without the others.
struct Loaded {
    ast: Ast,
    terms: Terms,
    kb: Kb,
}

fn load_text(text: &str) -> Loaded {
    load_named(text, "<fixture>")
}

/// [`load_text`], under the file name the parser should stamp on every `Loc`.
///
/// Which matters since M1e S1e.2.1: a `CompileError` may now carry the
/// position of the form it refuses, and a position is only as real as the file
/// name it names. The `.expected` fixtures write it as `{FILE}` — the same
/// placeholder `examples/broken/load/` has used since S1a.0.1.
fn load_named(text: &str, file: &str) -> Loaded {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, Some(file)).expect("the fixture parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the fixture loads");
    Loaded { ast, terms, kb }
}

/// The load error, for the fixtures whose point is that they do not load.
fn load_error(text: &str) -> String {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, Some("<fixture>")).expect("the fixture parses");
    ein_ir::load(&mut ast, &mut terms, &forms, None)
        .expect_err("must not load")
        .0
}

impl Loaded {
    /// Every plan this KB compiles, as the deterministic text `plan_shape`
    /// renders — or the `CompileError` that stopped it.
    ///
    /// `filter` is `activators_for`'s S1.22.0 arity filter: `true` is what the
    /// engine does, `false` is what a direct caller of `compile_rule` is.
    fn shape(&mut self, filter: bool) -> Result<String, String> {
        ein_infer::plan_shape_with(&self.ast, &mut self.terms, &self.kb, filter).map_err(|e| e.0)
    }

    fn compile_error(&mut self, filter: bool) -> String {
        self.shape(filter).expect_err("must not compile")
    }

    /// Run the deductive closure. Nothing here needs the search: every claim
    /// below is about what a rule derives from a fixed KB.
    fn saturate(&mut self) {
        let mut events = Events::off();
        let mut s = Session {
            kb: &mut self.kb,
            terms: &mut self.terms,
            ast: &self.ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).expect("the fixture compiles");
        sat.saturate(&mut s, None, &mut |_| {})
            .expect("the fixture saturates");
    }

    /// Every fact under `head`, as s-expressions, sorted — the readable form
    /// of "what did this rule conclude".
    fn facts_under(&self, head: &str) -> Vec<String> {
        let mut out: Vec<String> = self
            .kb
            .facts()
            .filter(|&f| self.terms.sym(self.terms.facts.get(f).0) == head)
            .map(|f| sexpr(&self.terms, f))
            .collect();
        out.sort();
        out
    }

    fn all_facts(&self) -> Vec<String> {
        self.kb.facts().map(|f| sexpr(&self.terms, f)).collect()
    }
}

/// A `(config …)` block's coerced value, or the message that refused it.
///
/// Through `load`, not through the coercer directly: a flag nobody can set
/// from a file is exactly the bug F-KER-4 was, and it survived a suite that
/// built its configs in code.
fn config(body: &str) -> Result<SolverConfig, String> {
    let text = format!("(config {body})\n");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, Some("<config>")).expect("the block parses");
    match ein_ir::load(&mut ast, &mut terms, &forms, None) {
        Ok(kb) => Ok(kb.program().config.clone().expect("the block was ingested")),
        Err(e) => Err(e.0),
    }
}

fn cfg(body: &str) -> SolverConfig {
    config(body).expect("coerces")
}

// ── The in-`:match` keyword pair ───────────────────────────────────

/// `:where (neq ?a ?b)` inside a `:match` conjunction is **dropped**, and the
/// rule still compiles — work-list
/// `an-in-match-keyword-pair-is-dropped-not-compiled`.
///
/// Q32 moved guards from a `:where` clause to positional `(neq …)` premises
/// and chose tolerance over a loud failure, so the old spelling has to survive
/// the parser and then vanish. "Vanish" is the load-bearing half: lowered to
/// *anything* — a guard on an unresolvable predicate, an opaque slot that
/// never unifies — the rule would stop firing rather than fire unguarded, and
/// a file still carrying `:where` would silently lose its conclusions instead
/// of gaining a few. So the assertion is on the whole compiled program: one
/// `SCAN`, and no guard of any kind.
#[test]
fn a_where_keyword_pair_in_a_match_is_dropped_and_the_rule_still_compiles() {
    let mut l = load_text(
        r#"(relation r T T)
(rule old-where (?r)
  :match (and (?r ?a ?b) :where (neq ?a ?b))
  :assert (?r ?b ?a)
  :why "ow")
(old-where r)
"#,
    );
    let shape = l
        .shape(true)
        .expect("the rule compiles despite the kw_pair");
    let steps: Vec<&str> = shape
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with("SCAN ") || l.starts_with("JOIN "))
        .collect();
    assert_eq!(
        steps,
        vec!["SCAN r [?a ?b]"],
        "only the relational premise survives:\n{shape}"
    );
    assert!(
        !shape.contains("GUARD"),
        "the `:where` pair must emit no guard — neither inline nor lifted:\n{shape}"
    );
    assert!(
        shape.contains("D0 STEPS 1"),
        "…and no step of any other kind either:\n{shape}"
    );
}

// ── A string in a rule — M1e2 S1e2.2.18 ────────────────────────────

impl Loaded {
    /// Every fact under `head` **by value**: each fact's `repr`, which keeps a
    /// string's symbol quoted and an int bare, where [`Loaded::facts_under`]
    /// prints the two alike (P1e2.3's N5).
    fn values_under(&self, head: &str) -> BTreeSet<String> {
        self.kb
            .facts()
            .filter(|&f| self.terms.sym(self.terms.facts.get(f).0) == head)
            .map(|f| ein_core::repr::repr(&self.terms.repr_fact(f)))
            .collect()
    }
}

/// **A string in a rule is the symbol it names, in both clauses and at any
/// depth** — M1e2 S1e2.2.18, W18: what the loader holds a string written as a
/// fact's argument as (`01_grammar.md`'s `STRING` row), the compiler lowers a
/// string written in a rule to.
///
/// Until S1e2.2.18 it did not: in a `:match` a string was an opaque slot that
/// matched nothing, so a premise naming one never matched and an `(absent …)`
/// over one always held; a guard's was a raw node that equalled no value, so
/// `eq` never held and `neq` always did; and in an `:assert` the firing
/// stopped the run. Each rule below is one of those places, and each conclusion
/// is compared to the one the same program with the string written as an atom
/// derives, by value. `"1"` has no atom spelling, a bare `1` being the int, so
/// the program states both and the claim is that the string matches the
/// symbol's fact and not the int's.
#[test]
fn a_string_in_a_rule_is_the_symbol_it_names() {
    let program = |q: &str| {
        let s = |text: &str| format!("{q}{text}{q}");
        format!(
            r#"(relation holds Node) (relation wrap Node) (relation hit Node) (relation tag Node Node)
(holds "1") (holds 1) (holds "b") (wrap (box "1"))
(rule top    () :match (holds "1")              :assert (hit top))
(rule nested () :match (wrap (box "1"))         :assert (hit nested))
(rule guard  () :match (and (holds ?x) (eq ?x {b})) :assert (tag guard ?x))
(rule other  () :match (and (holds ?x) (neq ?x {b})) :assert (tag other ?x))
(rule naf    () :match (and (holds b) (absent (holds {b}))) :assert (hit naf))
(rule out    () :match (holds b)
  :assert (and (tag out {lbl}) (tag out (box {lbl})) (not (tag {lbl} {lbl}))))
"#,
            b = s("b"),
            lbl = s("lbl"),
        )
    };
    let mut strings = load_text(&program("\""));
    strings.saturate();
    let mut atoms = load_text(&program(""));
    atoms.saturate();
    for head in ["hit", "tag", "not"] {
        assert_eq!(
            strings.values_under(head),
            atoms.values_under(head),
            "{head}: a string in a rule concludes what the atom it names does"
        );
    }
    assert_eq!(
        strings.facts_under("hit"),
        ["(hit nested)", "(hit top)"],
        "the premises naming \"1\" match the symbol's facts, top-level and \
         nested, and the absent over (holds \"b\") does not hold"
    );
    assert_eq!(
        strings.facts_under("tag"),
        [
            "(tag guard b)",
            "(tag other 1)",
            "(tag other 1)",
            "(tag out (box lbl))",
            "(tag out lbl)"
        ],
        "eq holds for b alone, neq for both 1s, and the :assert stores the \
         symbol, top-level and nested"
    );
    assert_eq!(strings.facts_under("not"), ["(not (tag lbl lbl))"]);
    // The slots hold the symbol: `SCAN holds ['1']`, `ASSERT 0 (tag 'out'
    // 'lbl')`. A guard's argument prints as its IR node whatever it lowers to
    // (`shape::guard_arg_text`), so its lowering is held by `(tag guard b)`.
    let shape = strings.shape(true).expect("compiles");
    let slots: Vec<&str> = shape
        .lines()
        .filter(|l| !l.trim_start().starts_with("GUARD"))
        .collect();
    assert!(
        slots.iter().all(|l| !l.contains("String(")),
        "no slot holds a string as an opaque node:\n{shape}"
    );
    assert!(slots.contains(&"    SCAN holds ['1']"), "{shape}");
}

/// M1e2 S1e2.2.20 (W20, Q-M1e2.13 (a)) — **a wildcard in a rule's pattern
/// matches any value**, at the top of a premise, in a nested fact, under a
/// `not` and under an `absent`, two of them independently: the program
/// concludes what the same program with each `_` written as a fresh variable
/// concludes, by value. Until then the compiler lowered `_` as an opaque slot,
/// so every premise below that writes one never matched and `naf-no`'s
/// `absent` always held. The slots are `_`, never an opaque node.
#[test]
fn a_wildcard_in_a_rule_matches_what_a_fresh_variable_does() {
    let program = |wild: bool| {
        let mut n = 0;
        let mut w = || {
            n += 1;
            if wild {
                "_".to_string()
            } else {
                format!("?w{n}")
            }
        };
        format!(
            "(relation holds T) (relation pair T T) (relation wrap T) (relation hit T)
(holds a) (pair a b) (wrap (box a)) (not (pair b a))
(rule top    () :match (holds {})                 :assert (hit top))
(rule nested () :match (wrap (box {}))            :assert (hit nested))
(rule neg    () :match (not (pair {} ?y))         :assert (hit ?y))
(rule two    () :match (pair {} {})               :assert (hit two))
(rule naf-yes () :match (and (holds a) (absent (pair {} a))) :assert (hit naf-yes))
(rule naf-no  () :match (and (holds a) (absent (pair a {}))) :assert (hit naf-no))
",
            w(),
            w(),
            w(),
            w(),
            w(),
            w(),
            w()
        )
    };
    let mut wild = load_text(&program(true));
    wild.saturate();
    let mut fresh = load_text(&program(false));
    fresh.saturate();
    assert_eq!(
        wild.facts_under("hit"),
        fresh.facts_under("hit"),
        "`_` matches what a fresh variable matches"
    );
    assert_eq!(
        wild.facts_under("hit"),
        [
            "(hit a)",
            "(hit naf-yes)",
            "(hit nested)",
            "(hit top)",
            "(hit two)"
        ],
    );
    let shape = wild.shape(true).expect("compiles");
    assert!(
        !shape.contains("Wildcard"),
        "no slot holds `_` as an opaque node:\n{shape}"
    );
    assert!(shape.contains("    SCAN holds [_]"), "{shape}");
}

// ── S1.22.0's four refusals, and M1e's four ────────────────────────

/// Each fixture under `examples/broken/compile/` is refused with exactly the
/// message in its `.expected` file — work-list
/// `the-four-compile-errors-say-what-their-expected-file-says`.
///
/// These messages are a **surface**: a rule author reads them, and all but one
/// name the offending IR form by its `repr`. Checking the directory
/// listing as well is not tidiness — a fixture renamed away from its
/// `.expected`, or a refusal added without one, would leave a branch of the
/// compiler with nobody asserting its text.
///
/// **Eight since M1e [S1e.2.1], not four.** `CO-H1` was one process panic from
/// a well-formed program, `(eq ?x)`; the S1e.1.6 sweep turned it into a rule —
/// the grammar shape-pins four of the seven kernel meta-primitives, and every
/// cell that panicked or silently misbehaved was one of the other three. The
/// four new fixtures are that rule's whole surface: `eq` and `absent`, each
/// below and above its arity. Two of the four used to **fire and exit 0**,
/// which is why they are here rather than in a note.
///
/// [S1e.2.1]: ../../../../docs/history/m1e_review_processing/README.md#s1e21--correctness-the-panic-the-guard-the-traversal
///
/// **Six since M1e2 S1e2.2.24**: `empty_absent` and `unbound_head` moved to
/// `examples/broken/load/` when the loader began refusing a premise that is
/// not a form and a relation variable no parameter binds (W24), each
/// `.expected` its load line there.
///
/// The walk runs with the arity filter **off**, because `activator_arity`'s
/// error is unreachable with it on — see the next test but one.
#[test]
fn every_compile_error_says_what_its_expected_file_says() {
    let dir = repo_root().join("examples/broken/compile");
    let mut fixtures: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "ein"))
        .collect();
    fixtures.sort();
    let stems: Vec<String> = fixtures
        .iter()
        .map(|p| {
            p.file_stem()
                .expect("a stem")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(
        stems,
        [
            "absent_arity_high",
            "absent_arity_zero",
            "activator_arity",
            "eq_arity_high",
            "eq_arity_low",
            "nested_or"
        ],
        "one fixture per CompileError branch, in {}",
        dir.display()
    );
    for path in &fixtures {
        let name = path.file_name().expect("a file").to_string_lossy();
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{name}: {e}"));
        let expected = std::fs::read_to_string(path.with_extension("expected"))
            .unwrap_or_else(|e| panic!("{name}.expected: {e}"))
            .replace("{FILE}", path.to_str().expect("utf-8"));
        let mut l = load_named(&text, path.to_str().expect("utf-8"));
        assert_eq!(
            l.compile_error(false).trim_end(),
            expected.trim_end(),
            "{name}: the message and its .expected have drifted"
        );
    }
}

/// A `(or …)` that is not top-level is refused in **both** polarities —
/// work-list `a-nested-or-in-a-match-premise-is-refused`.
///
/// One message, two reachable call sites, and before S1.22.0 the two were
/// wrong in opposite directions: dropped from a positive premise the conjunct
/// made the match set *larger*, so the rule fired with neither disjunct in the
/// KB; dropped from inside an `(absent …)` it left the guard's sub-plan empty,
/// which matches vacuously — so the guard failed against every possible KB
/// and, being monotone, retired its candidates for good. Only the positive
/// site has a checked-in fixture, so the negative one is built here: a
/// compiler that guarded only the site it had a fixture for would look correct.
#[test]
fn a_nested_or_is_refused_in_a_premise_and_inside_an_absent() {
    let decls = "(relation a T) (relation p T) (relation q T) (relation out T)\n";
    for (site, match_) in [
        ("positive premise", "(and (a ?x) (or (p ?x) (q ?x)))"),
        (
            "inside an absent",
            "(and (a ?x) (absent (or (p ?x) (q ?x))))",
        ),
    ] {
        let mut l = load_text(&format!(
            "{decls}(rule r ()\n  :match {match_}\n  :assert (out ?x) :why \"d\")\n"
        ));
        let err = l.compile_error(true);
        assert!(
            err.starts_with("nested `(or …)` in a `:match` premise:"),
            "{site}: {err}"
        );
        assert!(
            err.contains("Only a TOP-LEVEL `(or …)` is supported"),
            "{site}: the message has to say what *is* supported, or the author \
             cannot act on it: {err}"
        );
    }
}

/// An `(absent …)` that compiles to no steps is refused — work-list
/// `an-absent-whose-sub-plan-is-empty-is-refused`.
///
/// An empty sub-plan is not "a query that finds nothing": the matcher reads an
/// empty step tuple as one *vacuous* match, so the negative query holds, the
/// premise fails, and — the guard being monotone — the candidate is retired
/// permanently. The failure is therefore silent and irreversible, which is why
/// this is a compile error and not a warning.
///
/// **The loader refuses the two shapes this test was written with since M1e2
/// S1e2.2.24** (W24, Q-M1e2.16 (1), Q-M1e2.18 (v)): `(absent ?x)`, a premise
/// that is not a form, and `(absent (?R ?x))`, a relation variable no
/// parameter binds, each with its line, before the compiler sees either; the
/// first's fixture, `empty_absent.ein`, moved to `examples/broken/load/`. One
/// program the loader accepts still reaches the empty sub-plan: a keyword pair
/// as an `absent`'s whole body, which the compiler drops (Q32), so the
/// refusal is checked through it.
#[test]
fn an_absent_that_compiles_to_no_steps_is_refused() {
    let mut empty = load_text(
        "(relation p T) (relation out T)\n\
         (rule vacuous ()\n  :match (and (p ?x) (absent :k v))\n  :assert (out ?x))\n",
    );
    let err = empty.compile_error(true);
    assert!(
        err.starts_with("`(absent …)` sub-plan compiled to no steps"),
        "{err}"
    );
    assert!(
        err.contains("retired permanently"),
        "the message has to say why an empty guard is fatal rather than merely \
         useless: {err}"
    );

    let not_a_form = load_error(
        "(relation p T) (relation out T)\n\
         (rule vacuous ()\n  :match (and (p ?x) (absent ?x))\n  :assert (out ?x))\n",
    );
    assert!(
        not_a_form.contains(
            "rule 'vacuous': its :match writes ?x as a premise of absent, and ?x is no premise"
        ),
        "a premise that is not a form is the loader's since S1e2.2.24: {not_a_form}"
    );
    let unbound = load_error(
        "(relation a T) (relation out T)\n\
         (rule free-head ()\n  :match (and (a ?x) (absent (?R ?x)))\n  :assert (out ?x))\n",
    );
    assert!(
        unbound.contains(
            "rule 'free-head': its :match writes (?R ?x) with ?R as its relation, which no \
             parameter binds"
        ),
        "a relation variable no parameter binds is the loader's since S1e2.2.24: {unbound}"
    );
}

/// An activator whose arity does not match the rule's parameter list is
/// refused by the compiler, **and** never reaches it through the engine —
/// work-list `an-arity-mismatched-activator-is-refused-by-a-direct-caller`.
///
/// S1.22.0 added two guards and the pair is the point. Without the filter the
/// mismatched pair left every parameter-headed premise with an unbound head
/// var; those premises were dropped; `steps` was then empty, which the matcher
/// accepts as one vacuous match — so the rule fired unconditionally. Without
/// the error, anything that builds the pair itself gets that same vacuous plan
/// back. So both halves are asserted against the one fixture: unfiltered it
/// raises, filtered it compiles *nothing at all* — which is the whole
/// assertion, `pairwise` being the file's only rule.
#[test]
fn an_arity_mismatched_activator_is_refused_and_never_reaches_the_compiler() {
    let path = repo_root().join("examples/broken/compile/activator_arity.ein");
    let text = std::fs::read_to_string(&path).expect("the fixture");
    let mut l = load_text(&text);

    let err = l.compile_error(false);
    assert!(
        err.contains("activator pairwise('r',) has 1 argument(s)"),
        "the message names the activator and its arity: {err}"
    );
    assert!(
        err.contains("rule `pairwise` declares 2 parameter(s) ('R', 'T')"),
        "…and the parameter list it cannot bind: {err}"
    );

    let shape = l.shape(true).expect("with the filter on, nothing compiles");
    assert!(
        shape.is_empty(),
        "the arity filter has to drop the activator before the compiler sees \
         it; any plan here is a vacuous one:\n{shape}"
    );
}

// ── A relation named by a number — M1e2 S1e2.2.4 ───────────────────

/// **A relation parameter bound to an integer is refused, wherever the
/// binding is made** — M1e2 S1e2.2.4, Q-M1e.21's residue
/// (`docs/kernel/defined_behaviour.md` § 4.6).
///
/// `mirror` names the relation it matches and asserts by its parameter, so
/// `(mirror 3)` would have the compiler intern `3` as a relation's name: a
/// name no rule's text predicts, which the candidates cannot hold. Written as
/// a fact it is the loader's (`examples/broken/load/relation_parameter_int.ein`);
/// derived through a variable, as `make` does here, only a run binds it, so
/// root's saturation refuses it when it compiles `mirror` for that activator.
/// Until then the program loaded, ran and answered, with a fact of relation
/// `3` in every state.
#[test]
fn a_relation_parameter_a_rule_binds_to_a_number_is_refused() {
    const MIRROR: &str = "(rule mirror (?R) :match (?R ?a ?b) :assert (?R ?b ?a))";
    let derived = format!(
        "(relation q T T)\n(q A B)\n(num 3)\n(rule make () :match (num ?n) :assert (mirror ?n))\n{MIRROR}"
    );
    let mut l = load_text(&derived);
    let mut events = Events::off();
    let mut s = Session {
        kb: &mut l.kb,
        terms: &mut l.terms,
        ast: &l.ast,
        events: &mut events,
        memo: SharedMemo::default(),
    };
    let mut sat = Saturator::new(&mut s).expect("nothing is refused before `make` fires");
    let err = sat
        .saturate(&mut s, None, &mut |_| {})
        .expect_err("`mirror` is refused for the derived (mirror 3)")
        .to_string();
    assert!(
        err.contains("the relation head ?R of (?R ?a ?b) at Loc(")
            && err.contains("is bound to the integer 3 — a relation is named by a symbol"),
        "the compiler names the head, the form and the integer: {err}"
    );

    // A nested fact, derived the same way, binds no parameter, so it names no
    // relation either — M1e2 S1e2.2.24 (W24, Q-M1e2.18 (iv)). Until then the
    // top-level premise stopped compilation with the unbound-head line in the
    // Python IR's spelling, and a pattern inside a pattern matched nothing.
    for pattern in ["(?R ?a ?b)", "(box (?R ?a ?b))"] {
        let rule = format!("(rule mirror (?R) :match {pattern} :assert (hit a))");
        let mut l = load_text(&format!(
            "(relation q T T) (relation hit T) (relation box T)\n(q A B) (src (q A B))\n\
             (rule make () :match (src ?f) :assert (mirror ?f))\n{rule}"
        ));
        let mut events = Events::off();
        let mut s = Session {
            kb: &mut l.kb,
            terms: &mut l.terms,
            ast: &l.ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).expect("nothing is refused before `make` fires");
        let err = sat
            .saturate(&mut s, None, &mut |_| {})
            .expect_err("`mirror` is refused for the derived (mirror (q A B))")
            .to_string();
        assert!(
            err.contains("the relation head ?R of (?R ?a ?b) at Loc(")
                && err.contains(
                    "is bound to the fact (q A B) — a relation is named by a symbol, not a fact."
                ),
            "{pattern}: the compiler names the head, the form and the fact: {err}"
        );
    }

    // Written down, in an `:assert` as in a fact, the loader sees it.
    let literal = format!(
        "(relation q T T)\n(q A B)\n(rule make () :match (q ?a ?b) :assert (mirror 3))\n{MIRROR}"
    );
    let e = load_error(&literal);
    assert!(
        e.contains("rule 'mirror': its parameter ?R names a relation, and (mirror 3) binds it to the integer 3 at Loc("),
        "{e}"
    );

    // A name is what the parameter is for.
    let mut ok = load_text(&format!("(relation q T T)\n(q A B)\n(mirror q)\n{MIRROR}"));
    ok.saturate();
    assert_eq!(ok.facts_under("q"), ["(q A B)", "(q B A)"]);
}

// ── `(config …)` coercion ──────────────────────────────────────────

/// The numeric and string flags are retired — M1e2 S1e2.1a.5: every flag a
/// program can set is a bool, and setting a retired one is a load error that
/// says why (`solve_interface.md` § 2.4). The coercions they reached stay the
/// container's (`ein-einb` round-trips every field), and
/// `ein-ir/tests/golden/config_coercion.txt` pins each refusal.
#[test]
fn the_retired_flags_are_refused_with_why() {
    for (flag, why) in [
        (":lattice-order-seed 7", "an order lever"),
        (":candidate-order-seed 5", "it was inert"),
        (":print-alive true", "it was inert"),
        (":lattice-order lex", "an order lever"),
        (":hypgen-rel-weight 2", "an order lever"),
        (":enable-singleton-writeback false", "it wrote into root"),
        (":enable-forced-positive false", "it wrote into root"),
    ] {
        let e = config(flag).expect_err("a retired flag is refused");
        assert!(
            e.contains("was retired at M1e2 S1e2.1a.5") && e.contains(why),
            "{flag}: {e}"
        );
    }
    // S1e2.1a.6: the one flag that changed R, retired on the user's ruling of
    // 2026-11-01. The mirror itself is always on.
    let e = config(":enable-symmetric-mirror false").expect_err("retired");
    assert!(
        e.contains("was retired at M1e2 S1e2.1a.6") && e.contains("always on"),
        "{e}"
    );
    assert_eq!(
        SolverConfig::default().lattice_order_seed,
        None,
        "an unset optional field is absent, not zero"
    );
}

/// The bool coercion reads its IR shapes — work-list
/// `bool-str-and-float-flags-coerce-from-their-ir-shapes`, whose string and
/// float halves retired with their flags.
#[test]
fn bool_flags_coerce_from_their_ir_shapes() {
    // `lattice-sanity-check` since M1e2 S1e2.2.5 retired `warn-derived-naf`.
    assert!(cfg(":lattice-sanity-check true").lattice_sanity_check);
    assert!(!cfg(":lattice-sanity-check false").lattice_sanity_check);
    // A quoted bool: `record-alternative-justifications` since M1e2 S1e2.1c.5
    // retired `enable-path-nogoods`, which carried it.
    assert!(
        !cfg(r#":record-alternative-justifications "false""#).record_alternative_justifications
    );
}

/// The two refusals name the flag and the shape they wanted — work-list
/// `the-three-config-rejections-say-what-they-say`, whose integer refusal
/// retired with the integer flags.
///
/// A `(config …)` block is hand-authored and its keys are easy to misspell, so
/// the unknown-flag message enumerates the accepted set rather than merely
/// saying no: that enumeration is the only discovery mechanism the surface
/// language offers for these. The type refusal matters for the opposite
/// reason — silently coercing `7` to `true` would hand a puzzle a
/// configuration its author did not write, and the run would still look
/// successful.
#[test]
fn the_config_rejections_name_the_flag_and_the_shape_they_wanted() {
    let unknown = config(":nope 1").expect_err("no such flag");
    assert!(
        unknown.contains("unknown config flag :nope"),
        "must name the offending key: {unknown}"
    );
    for (known, _) in ein_core::config::FLAGS {
        assert!(
            unknown.contains(known),
            "the accepted set must be enumerated, and {known} is missing: {unknown}"
        );
    }
    for (retired, _, _) in ein_core::config::RETIRED {
        assert!(
            !unknown.contains(retired),
            "a retired flag is not offered: {unknown}"
        );
    }

    let not_a_bool = config(":lattice-sanity-check 7").expect_err("not a boolean");
    assert!(
        not_a_bool.contains("config flag :lattice-sanity-check expects true/false"),
        "{not_a_bool}"
    );
}

// ── S1.22.4 — the arity-1 membership channel ───────────────────────

const DECLS: &str = "(relation adult Person)\n\
                     (relation likes Person Drink)\n\
                     (relation between Person Person Person)\n";

/// `:match (relation ?R)` sees every declaration whatever its arity, and the
/// arity-coupled patterns still see exactly what they saw before — work-list
/// `arity-independent-membership-match`.
///
/// This is the origin ask ("other rules could check if argument is name of
/// relation by `:match (relation ?R)`"), and before S1.22.4 it matched
/// *nothing*: the loader stored only the arity-N signature mirror, and
/// matching is arity-coupled, so `std.bijection` / `std.algebra` /
/// `std.typing` quietly saw only the binary declarations. The second half of
/// the assertion is the real hazard in a fix of this shape — a membership fact
/// that *replaced* the mirror, or a matcher made arity-tolerant, would satisfy
/// the ask and break every existing signature-reading rule at the same time.
/// All three probes therefore run against one KB.
#[test]
fn the_membership_channel_is_arity_independent_and_the_old_ones_are_unchanged() {
    let mut l = load_text(&format!(
        "{DECLS}\
         (rule sees-any    () :match (relation ?R)       :assert (is-rel ?R))\n\
         (rule sees-unary  () :match (relation ?R ?A)    :assert (is-unary ?R))\n\
         (rule sees-binary () :match (relation ?R ?A ?B) :assert (is-binary ?R))\n"
    ));
    l.saturate();
    assert_eq!(
        l.facts_under("is-rel"),
        ["(is-rel adult)", "(is-rel between)", "(is-rel likes)"],
        "the membership channel is arity-independent"
    );
    assert_eq!(
        l.facts_under("is-unary"),
        ["(is-unary adult)"],
        "the arity-1 signature mirror still means a unary declaration"
    );
    assert_eq!(
        l.facts_under("is-binary"),
        ["(is-binary likes)"],
        "…and the arity-2 one a binary declaration"
    );
}

/// Only *declared* relations get a membership fact — work-list
/// `membership-facts-are-declarations-only`.
///
/// A property tag such as `(symmetric likes)` auto-vivifies `symmetric` into
/// the relation registry, open-world, with nobody having declared it. If
/// registration were what emitted `(relation R)`, `(relation ?R)` would
/// enumerate the puzzle's whole *vocabulary* rather than its declarations, and
/// every stdlib rule that walks relations would start walking property tags
/// too. The claim is asserted from both sides for that reason: `symmetric` is
/// registered, and it is still not what `(relation ?R)` finds.
#[test]
fn only_declared_relations_get_a_membership_fact() {
    let mut l = load_text(
        "(relation likes Person Drink)\n\
         (symmetric likes)\n\
         (rule sees-any () :match (relation ?R) :assert (is-rel ?R))\n",
    );
    let symmetric = l.terms.syms.get("symmetric").expect("interned by the tag");
    assert!(
        l.kb.program().relations.contains(symmetric),
        "the tag auto-vivifies a relation — without that this test proves nothing"
    );
    l.saturate();
    assert_eq!(
        l.facts_under("is-rel"),
        ["(is-rel likes)"],
        "`(relation ?R)` means *declared*, not *registered*"
    );
}

/// A bare `(relation opaque)` is a declaration with an empty signature, and it
/// stores exactly one fact — work-list
/// `a-bare-declaration-is-declared-with-an-empty-signature`.
///
/// Every other declaration stores two facts: the arity-N signature mirror and
/// the arity-1 membership companion. For a bare one the two coincide, and the
/// loader has to notice — storing `(relation opaque)` twice would double every
/// rule that counts relations, and storing it only as a mirror would leave
/// `opaque` invisible to `(relation ?R)`. It is also still a *declaration*,
/// which is the half an empty signature makes newly ambiguous: the wrapped
/// argument form `(relation R (T1 T2))` now differs from a bare declaration
/// only by an explicit check, so it is asserted to still be refused.
#[test]
fn a_bare_declaration_is_declared_with_an_empty_signature_and_stores_one_fact() {
    let l = load_text("(relation opaque)\n");
    let opaque = l.terms.syms.get("opaque").expect("interned");
    let rel =
        l.kb.program()
            .relations
            .get(opaque)
            .expect("a bare declaration is still a declaration");
    assert!(rel.signature.is_empty(), "no signature was written");
    assert!(rel.declared, "…but it is declared, not open-world vivified");
    assert_eq!(
        l.all_facts(),
        ["(relation opaque)"],
        "the mirror *is* the membership fact; a second copy would double-count"
    );
    assert!(
        load_error("(relation R (T1 T2))\n").contains("malformed (relation)"),
        "a wrapped argument list must not slip through as a bare declaration"
    );
}

// ── S1.22.4 — what the blind enumerator fills ──────────────────────

/// The relation names the blind enumerator proposes hypotheses for.
fn guessed_relations(text: &str) -> BTreeSet<String> {
    let mut l = load_text(text);
    let mut ids: Vec<FactId> = Vec::new();
    {
        let mut events = Events::off();
        let mut stats = HypGenStats::new();
        let mut s = Session {
            kb: &mut l.kb,
            terms: &mut l.terms,
            ast: &l.ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        ein_infer::generate(&mut s, &mut stats, &mut |fact| {
            ids.push(fact);
            ControlFlow::Continue(())
        })
        .expect("the fixture compiles");
    }
    ids.iter()
        .map(|&f| l.terms.sym(l.terms.facts.get(f).0).to_string())
        .collect()
}

/// A relation with an empty signature is never a hypothesis target — work-list
/// `an-empty-signature-is-not-a-hypothesis-target`.
///
/// Signature *presence* is the kernel's only "declared domain relation"
/// signal, and a bare declaration deliberately fails it: with no signature
/// there is no type to draw fillers from, so every candidate the enumerator
/// could build would be a guess over the entire name set. The filter is one
/// `.filter(|r| !r.signature.is_empty())` in `hypgen`; no corpus file carries a
/// bare declaration, so deleting it would not move a single corpus run — which
/// is precisely why it needs a test of its own rather than the corpus.
#[test]
fn an_empty_signature_is_not_a_hypothesis_target() {
    let guessed = guessed_relations(
        "(relation opaque)\n\
         (relation likes Person Drink)\n\
         (is-a Jack Person) (is-a Jill Person)\n",
    );
    assert_eq!(
        guessed,
        BTreeSet::from(["likes".to_string()]),
        "the bare declaration is skipped and the signed one is not — asserting \
         the whole set is what keeps the skip from passing vacuously"
    );
}

/// Arity 3 and beyond is unenumerated — work-list `arity-three-is-unenumerated`.
///
/// The enumerator fills arity 1 (the candidate *is* `(R focal)`, no filler
/// loop) and arity 2 (a filler loop over the object set); a ternary
/// declaration yields no candidates at all. No corpus relation exceeds arity
/// 2, so nothing in the corpus can tell "skipped" from "never
/// encountered", and an enumerator that quietly emitted a truncated binary candidate
/// for a ternary relation would pass every run. The binary control is in the
/// fixture for the same reason as above.
#[test]
fn arity_three_is_unenumerated() {
    let guessed = guessed_relations(
        "(relation between Person Person Person)\n\
         (relation likes Person Drink)\n\
         (is-a Jack Person) (is-a Jill Person)\n",
    );
    assert_eq!(
        guessed,
        BTreeSet::from(["likes".to_string()]),
        "only the binary relation is filled"
    );
}
