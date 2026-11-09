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

    /// The line the closure stops with, for the fixtures whose point is that
    /// a firing stops it.
    fn saturate_error(&mut self) -> String {
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
            .expect_err("the closure must stop")
            .to_string()
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

/// The parser's report on `text`, which must refuse it: since M1f S1f.5.5's
/// swap the grammar refuses a shape the loader or the compiler refused until
/// then, each error at its place, `<fixture>:line:col: …`.
fn refused(text: &str) -> ein_ir::ParseError {
    parse(&mut Ast::new(), text, Some("<fixture>")).expect_err("the parser refuses it")
}

// ── The in-`:match` keyword pair ───────────────────────────────────

/// **A guard is a positional premise, and it compiles** — the test of Q32's
/// drop, restated by M1f S1f.5.11's T1f.5.11.2, which retired the drop.
///
/// Until M1f S1f.5.5's swap `:where (neq ?a ?b)` inside a `:match` conjunction
/// was dropped (work-list `an-in-match-keyword-pair-is-dropped-not-compiled`):
/// Q32 moved guards from a `:where` clause to positional `(neq …)` premises
/// and chose tolerance over a loud failure, so the old spelling survived the
/// parser and vanished, and the rule compiled to one `SCAN` and no guard,
/// firing unguarded; this test held the drop. Since the swap the pair is
/// refused at the parse, at the keyword: each form takes a fixed set of
/// keywords, and no keyword pair parses inside a term (S1f.5.11's ruling,
/// D-15; Q-M1e2.28). The refusal of this program is a row of the grammar's
/// table (`ein-ir/tests/grammar_pages.rs`, *a :match's and*), where every
/// place in a term is held, and the compiler's drop is gone; what stays here
/// is the compiler's half, that the positional guard is what to write.
#[test]
fn a_where_guard_written_as_a_positional_premise_compiles() {
    let mut l = load_text(
        r#"(relation r T T :why "r holds of {?1} and {?2}")
(rule old-where (?r)
  :match (and (?r ?a ?b) (neq ?a ?b))
  :assert (?r ?b ?a)
  :why "{?r} holds of {?a} and {?b}, which differ, so {?r} holds of {?b} and {?a}")
(old-where r)
"#,
    );
    let shape = l.shape(true).expect("the positional guard compiles");
    assert!(shape.contains("GUARD"), "the guard is compiled:\n{shape}");
}

// ── A string in a rule — M1e2 S1e2.2.18 ────────────────────────────

/// **A string in a rule is refused at the parse** — M1f S1f.5.5's swap, on
/// S1f.5.2's ruling: a string is natural-language text, written only after
/// `:why`, `:source` or `:goal-text`, so a string as a value is refused in
/// both clauses, at any depth, and as a fact's argument, each with S1f.5.2's
/// note; the name written bare is what to write.
///
/// Until M1e2 S1e2.2.18 a string in a `:match` was an opaque slot that matched
/// nothing, a guard's a raw node that equalled no value, and in an `:assert`
/// the firing stopped the run (W18); from S1e2.2.18 to the swap a string in a
/// rule was the symbol it names, what the loader held a string in a fact as.
/// The same program with every string written bare as the name it spelled
/// derives what the string program derived then, `"1"` and the int `1` being
/// one name since S1f.5.4's ruling.
#[test]
fn a_string_in_a_rule_is_the_symbol_it_names() {
    let program = |q: &str| {
        let s = |text: &str| format!("{q}{text}{q}");
        format!(
            r#"(relation holds Node :why "{{?1}} holds") (relation wrap Node :why "wrap holds of {{?1}}") (relation hit Node :why "hit holds of {{?1}}") (relation tag Node Node :why "{{?1}} is tagged {{?2}}")
(holds {one}) (holds 1) (holds {b}) (wrap (box {one}))
(rule top    () :match (holds {one})              :assert (hit top) :why "1 holds, so hit holds of top")
(rule nested () :match (wrap (box {one}))         :assert (hit nested) :why "wrap holds of (box 1), so hit holds of nested")
(rule guard  () :match (and (holds ?x) (eq ?x {b})) :assert (tag guard ?x) :why "{{?x}} holds and is b, so guard is tagged {{?x}}")
(rule other  () :match (and (holds ?x) (neq ?x {b})) :assert (tag other ?x) :why "{{?x}} holds and is not b, so other is tagged {{?x}}")
(rule naf    () :match (and (holds b) (absent (holds {b}))) :assert (hit naf) :why "b both holds and does not, so hit holds of naf")
(rule out    () :match (holds b)
  :assert (and (tag out {lbl}) (tag out (box {lbl})) (not (tag {lbl} {lbl}))) :why "b holds, so out is tagged lbl and (box lbl), and lbl is not tagged lbl")
"#,
            one = s("1"),
            b = s("b"),
            lbl = s("lbl"),
        )
    };
    let r = refused(&program("\""));
    // Twelve strings, eleven errors: recovery reaches each but the last, whose
    // error is its neighbour's, the repair deleting both.
    assert_eq!(r.errors.len(), 11, "an error at each string:\n{r}");
    for e in &r.errors {
        assert!(
            e.what.ends_with(": a string")
                && e.notes.len() == 1
                && e.notes[0].ends_with(
                    "is a string, and a string is natural-language text, written only after \
                     :why, :source or :goal-text: write a name bare"
                ),
            "{r}"
        );
    }
    let mut names = load_text(&program(""));
    names.saturate();
    assert_eq!(names.facts_under("hit"), ["(hit nested)", "(hit top)"]);
    assert_eq!(
        names.facts_under("tag"),
        [
            "(tag guard b)",
            "(tag other 1)",
            "(tag out (box lbl))",
            "(tag out lbl)"
        ],
        "eq holds for b alone and neq for 1, one name since S1f.5.4"
    );
    assert_eq!(names.facts_under("not"), ["(not (tag lbl lbl))"]);
    let shape = names.shape(true).expect("compiles");
    assert!(shape.contains("    SCAN holds ['1']"), "{shape}");
}

/// M1e2 S1e2.2.20 (W20, Q-M1e2.13 (a)) — **a wildcard in a rule's pattern
/// matched any value**, at the top of a premise, in a nested fact, under a
/// `not` and under an `absent`, two of them independently: the program
/// concluded what the same program with each `_` written as a fresh variable
/// concludes, by value (until then the compiler lowered `_` as an opaque slot
/// that matched nothing). **Since M1f S1f.5.5's swap a bare `_` is refused at
/// the parse, in every position** (S1f.5.1's ruling), each with its note, and
/// the fresh variable is what to write: it derives what the wildcard did.
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
            "(relation holds T :why \"{{?1}} holds\") (relation pair T T :why \"{{?1}} and {{?2}} are a pair\") (relation wrap T :why \"wrap holds of {{?1}}\") (relation hit T :why \"hit holds of {{?1}}\")
(holds a) (pair a b) (wrap (box a)) (not (pair b a))
(rule top    () :match (holds {})                 :assert (hit top) :why \"some value holds, so hit holds of top\")
(rule nested () :match (wrap (box {}))            :assert (hit nested) :why \"wrap holds of some (box …), so hit holds of nested\")
(rule neg    () :match (not (pair {} ?y))         :assert (hit ?y) :why \"some value and {{?y}} are stored as not a pair, so hit holds of {{?y}}\")
(rule two    () :match (pair {} {})               :assert (hit two) :why \"some two values are a pair, so hit holds of two\")
(rule naf-yes () :match (and (holds a) (absent (pair {} a))) :assert (hit naf-yes) :why \"a holds and no pair ends in a, so hit holds of naf-yes\")
(rule naf-no  () :match (and (holds a) (absent (pair a {}))) :assert (hit naf-no) :why \"a holds and no pair starts with a, so hit holds of naf-no\")
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
    let r = refused(&program(true));
    assert!(
        !r.errors.is_empty()
            && r.errors
                .iter()
                .all(|e| e.what == "unexpected `_`: _ (reserved)"
                    && e.notes.len() == 1
                    && e.notes[0].starts_with("_ is reserved and names nothing")),
        "{r}"
    );
    let mut fresh = load_text(&program(false));
    fresh.saturate();
    assert_eq!(
        fresh.facts_under("hit"),
        [
            "(hit a)",
            "(hit naf-yes)",
            "(hit nested)",
            "(hit top)",
            "(hit two)"
        ],
    );
}

// ── S1.22.0's four refusals, and M1e's four ────────────────────────

/// Each fixture under `examples/broken/compile/` is refused with exactly the
/// message in its `.expected` file — work-list
/// `the-four-compile-errors-say-what-their-expected-file-says`.
///
/// **Two since M1f S1f.5.5's swap**: the grammar states `eq`'s and `absent`'s
/// shapes, so the four M1e fixtures below are parse errors, and they moved to
/// `examples/broken/` beside the parse-negatives (Q-M1f.2 (a)), each
/// `.expected` the parse report, which
/// `ein-ir/tests/grammar_decisions.rs` holds.
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
        ["activator_arity", "nested_or"],
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
    let decls = "(relation a T :why \"a holds of {?1}\") (relation p T :why \"p holds of {?1}\") (relation q T :why \"q holds of {?1}\") (relation out T :why \"out holds of {?1}\")\n";
    for (site, match_) in [
        ("positive premise", "(and (a ?x) (or (p ?x) (q ?x)))"),
        (
            "inside an absent",
            "(and (a ?x) (absent (or (p ?x) (q ?x))))",
        ),
    ] {
        let mut l = load_text(&format!(
            "{decls}(rule r ()\n  :match {match_}\n  :assert (out ?x) :why \"a holds of {{?x}} and its premise over p or q holds, so out holds of {{?x}}\")\n"
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

/// **The compiler's and the firing's lines a macro's body still reaches** —
/// M1f S1f.5.5, T1f.5.5.5.
///
/// An `(absent …)` that compiled to no steps was refused here until T5 (work-list
/// `an-absent-whose-sub-plan-is-empty-is-refused`): an empty sub-plan is not "a
/// query that finds nothing", the matcher reads an empty step tuple as one
/// *vacuous* match, so the guard fails and, being monotone, retires its
/// candidate for good, in silence. No program reaches it since the swap: an
/// `absent` takes one premise (`ein.y`'s `Connective`), the two programs that
/// did, `(absent :k v)` and `(absent ?x)`, are parse errors (rows of `ein-ir`'s
/// grammar table), and a premise that is no form, which only a macro's body
/// still writes, is the loader's refusal (`Found::NotAForm`). So T5 made it an
/// assertion, and deleted the line.
///
/// The census counted three more lines the grammar's, and each is still
/// reached, through a macro: a macro's body is a template whose head may be
/// any name (`ein.y`'s `Template`), `absent` and `eq` among them, at any
/// arity, so the compiler's arity checks stay; and a body that is a bare
/// parameter, called in an `:assert`, leaves a variable as the whole
/// conclusion, which the firing refuses. So do a relation variable no parameter
/// binds under an `absent`, the loader's since S1e2.2.24.
#[test]
fn the_compile_and_firing_lines_a_macro_still_reaches() {
    let mut l = load_text(
        "(macro two (?a ?b) (absent ?a ?b) :why \"an absent of both {?a} and {?b}, two premises where it takes one\")\n(relation p T :why \"p holds of {?1}\") (relation hit T :why \"hit holds of {?1}\")\n(p a)\n\
         (rule r () :match (and (p ?x) (two (p ?x) (p ?x))) :assert (hit ?x) :why \"p holds of {?x} and two's absent holds, so hit holds of {?x}\")\n",
    );
    let err = l.compile_error(true);
    assert!(
        err.starts_with("`(absent …)` takes exactly 1 argument and was given 2:"),
        "{err}"
    );
    let mut l = load_text(
        "(macro three (?a ?b ?c) (eq ?a ?b ?c) :why \"{?a}, {?b} and {?c} are equal, three arguments where eq takes two\")\n(relation p T :why \"p holds of {?1}\") (relation hit T :why \"hit holds of {?1}\")\n(p a)\n\
         (rule r () :match (and (p ?x) (three ?x ?x ?x)) :assert (hit ?x) :why \"p holds of {?x}, which three calls equal to itself, so hit holds of {?x}\")\n",
    );
    let err = l.compile_error(true);
    assert!(
        err.starts_with("`(eq …)` takes exactly 2 arguments and was given 3:"),
        "{err}"
    );
    let mut l = load_text(
        "(macro id (?x) ?x :why \"{?x} itself, unchanged\")\n(relation p T :why \"p holds of {?1}\") (relation hit T :why \"hit holds of {?1}\")\n(p a)\n\
         (rule r () :match (p ?y) :assert (id ?y) :why \"p holds of {?y}, so the conclusion is {?y} itself\")\n",
    );
    assert_eq!(
        l.saturate_error(),
        "rule 'r': its :assert concludes ?y, which is not a fact — a conclusion is a fact \
         (relation argument …), or an (and …) of facts"
    );
    let unbound = load_error(
        "(relation a T :why \"a holds of {?1}\") (relation out T :why \"out holds of {?1}\")\n\
         (rule free-head ()\n  :match (and (a ?x) (absent (?R ?x)))\n  :assert (out ?x) :why \"a holds of {?x} and no relation holds of {?x}, so out holds of {?x}\")\n",
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

// ── A relation named by a fact — M1e2 S1e2.2.4, S1e2.2.24 ──────────

/// **A relation parameter bound to a nested fact is refused, wherever the
/// binding is made** — M1e2 S1e2.2.24 (W24, Q-M1e2.18 (iv)); where a rule
/// derives the binding, at load since M1f S1f.5.18 (Q-M1f.1 (b)), and by the
/// compiler behind it.
///
/// `mirror` names the relation it matches and asserts by its parameter. Until
/// M1f S1f.5.5's swap this test held the integer case too, S1e2.2.4's (Q-M1e.21's
/// residue): `(mirror 3)` had the compiler intern `3` as a relation's name, a
/// name no rule's text predicts, and the loader (written as a fact, the fixture
/// `examples/broken/load/relation_parameter_int.ein`) or the compiler (derived
/// through a variable) refused it. Since the swap a run of digits is a name
/// (S1f.5.4's ruling), so `(mirror 3)` names the relation `3` as `(mirror q)`
/// names `q`, and the fixture and that half retired with it (S1f.5.4's Q2 (a),
/// decided 2026-11-08).
#[test]
fn a_relation_parameter_a_rule_binds_to_a_nested_fact_is_refused() {
    const MIRROR: &str = "(rule mirror (?R) :match (?R ?a ?b) :assert (?R ?b ?a) :why \"{?R} holds of {?a} and {?b}, so it holds of {?b} and {?a}\")";
    // Digits are a name: `3` is the relation `mirror` matches, which holds
    // `(3 A B)`, so the rule concludes `(3 B A)`.
    let mut digits = load_text(&format!(
        "(relation q T T :why \"q holds of {{?1}} and {{?2}}\")\n(q A B)\n(3 A B)\n(num 3)\n\
         (rule make () :match (num ?n) :assert (mirror ?n) :why \"num holds of {{?n}}, so mirror runs over the relation {{?n}}\")\n{MIRROR}"
    ));
    digits.saturate();
    assert_eq!(digits.facts_under("3"), ["(3 A B)", "(3 B A)"]);

    // A nested fact, derived the same way, names no relation either — M1e2
    // S1e2.2.24 (W24, Q-M1e2.18 (iv)). Until then the top-level premise
    // stopped compilation with the unbound-head line in the Python IR's
    // spelling, and a pattern inside a pattern matched nothing. Since M1f
    // S1f.5.18 the loader refuses the program (Q-M1f.1 (b)): `src` is a marker
    // whose argument `make` passes to `mirror`'s relation parameter, so the fact
    // `(src (q A B))` writes a fact where a relation is named (Q-M1f.41). Until
    // then it loaded and the run stopped where `mirror`'s plan was compiled.
    for pattern in ["(?R ?a ?b)", "(box (?R ?a ?b))"] {
        let rule = format!(
            "(rule mirror (?R) :match {pattern} :assert (hit a) :why \"a fact of {{?R}} matches the pattern, so hit holds of a\")"
        );
        let err = load_error(&format!(
            "(relation q T T :why \"q holds of {{?1}} and {{?2}}\") (relation hit T :why \"hit holds of {{?1}}\") (relation box T :why \"box holds of {{?1}}\")\n(q A B) (src (q A B))\n\
             (rule make () :match (src ?f) :assert (mirror ?f) :why \"src holds of {{?f}}, so mirror runs over the relation {{?f}}\")\n{rule}"
        ));
        assert_eq!(
            err,
            "the fact (src (q A B)) writes (q A B) as ?f of (src ?f), which rule 'make' passes \
             to rule 'mirror''s parameter ?R, which names a relation — a relation is named by a \
             symbol, not a fact",
            "{pattern}"
        );
    }
    // The compiler's refusal stays behind the loader's, for a caller that hands
    // it such an activator: no program the loader accepts reaches it.
    for pattern in ["(?R ?a ?b)", "(box (?R ?a ?b))"] {
        let mut l = load_text(&format!(
            "(relation q T T :why \"q holds of {{?1}} and {{?2}}\") (relation hit T :why \"hit holds of {{?1}}\") (relation box T :why \"box holds of {{?1}}\")\n(q A B)\n\
             (rule mirror (?R) :match {pattern} :assert (hit a) :why \"a fact of {{?R}} matches the pattern, so hit holds of a\")"
        ));
        let rule =
            l.kb.program()
                .rules
                .values()
                .next()
                .expect("mirror")
                .clone();
        let qab =
            l.kb.facts()
                .find(|&f| sexpr(&l.terms, f) == "(q A B)")
                .expect("(q A B)");
        let activator = l
            .terms
            .intern_fact(rule.name, &[ein_core::Value::fact(qab)])
            .expect("room");
        let err = ein_infer::compile_rule(&l.ast, &mut l.terms, &rule, Some(activator))
            .expect_err("a relation head bound to a fact")
            .0;
        assert!(
            err.contains("the relation head ?R of ")
                && err.contains(
                    "is bound to the fact (q A B) — a relation is named by a symbol, not a fact."
                ),
            "{pattern}: the compiler names the head, the form and the fact: {err}"
        );
    }

    // A name is what the parameter is for.
    let mut ok = load_text(&format!(
        "(relation q T T :why \"q holds of {{?1}} and {{?2}}\")\n(q A B)\n(mirror q)\n{MIRROR}"
    ));
    ok.saturate();
    assert_eq!(ok.facts_under("q"), ["(q A B)", "(q B A)"]);
}

/// **A value parameter binds a nested fact** — M1f S1f.5.18, W33 and W58, on
/// the user's ruling of 2026-11-08 on S1f.5.5's census (*"Q3 - as
/// recommended"*): a nested fact is a value, so an activator binds a
/// parameter to it as to a name, whether the activator is written down or a
/// rule derives it.
///
/// Until S1f.5.18 `compile::Compiler::bind_activator` skipped a nested fact
/// (*a nested Fact does not bind*): the parameter was a free variable of the
/// plan, so `(act (f a))` fired `(p ?y)` on every `p` and concluded `(hit a)`
/// and `(hit b)` beside `(hit (f a))` (`W33/activator-nested-value`), and a
/// conclusion that alone read the parameter stopped the run with
/// `FireError::UnboundVar`'s *unbound var ?y in :assert* (`W58`). The test
/// beside it holds the relation half, which stays refused.
#[test]
fn a_value_parameter_an_activator_binds_to_a_nested_fact_is_bound() {
    // Written down, read by the premise: W33's witness.
    let mut matched = load_text(
        "(relation p T :why \"p holds of {?1}\") (relation hit T :why \"hit holds of {?1}\")\n(p a) (p b) (p (f a))\n\
         (rule act (?y) :match (p ?y) :assert (hit ?y) :why \"act names {?y} and p holds of it, so hit holds of {?y}\")\n(act (f a))\n",
    );
    matched.saturate();
    assert_eq!(matched.facts_under("hit"), ["(hit (f a))"]);

    // Written down, read by the conclusion alone: W58's witness.
    let mut concluded = load_text(
        "(relation p T :why \"p holds of {?1}\") (relation hit T :why \"hit holds of {?1}\")\n(p a)\n\
         (rule act (?y) :match (p ?x) :assert (hit ?y) :why \"act names {?y} and p holds of {?x}, so hit holds of {?y}\")\n(act (f a))\n",
    );
    concluded.saturate();
    assert_eq!(concluded.facts_under("hit"), ["(hit (f a))"]);

    // Derived by a rule through a variable, and read by both clauses: the
    // activator `mk` derives binds `?y` to the fact `src` holds.
    let mut derived = load_text(
        "(relation p T :why \"p holds of {?1}\") (relation hit T :why \"hit holds of {?1}\") (relation src T :why \"src holds of {?1}\")\n\
         (p a) (p b) (p (f a)) (src (f a))\n\
         (rule mk () :match (src ?s) :assert (act ?s) :why \"src holds of {?s}, so act runs for {?s}\")\n\
         (rule act (?y) :match (p ?y) :assert (and (hit ?y) (hit (got ?y))) :why \"act names {?y} and p holds of it, so hit holds of {?y} and of (got {?y})\")\n",
    );
    derived.saturate();
    assert_eq!(
        derived.facts_under("hit"),
        ["(hit (f a))", "(hit (got (f a)))"]
    );

    // Two activators that differ only in a nested fact bind two values, and
    // each plan matches its own.
    let mut two = load_text(
        "(relation p T :why \"p holds of {?1}\") (relation hit T :why \"hit holds of {?1}\")\n(p (f a)) (p (f b)) (p c)\n\
         (rule act (?y) :match (p ?y) :assert (hit ?y) :why \"act names {?y} and p holds of it, so hit holds of {?y}\")\n(act (f a)) (act (f b))\n",
    );
    two.saturate();
    assert_eq!(two.facts_under("hit"), ["(hit (f a))", "(hit (f b))"]);
}

// ── `(config …)` coercion ──────────────────────────────────────────

/// The bool coercion reads its IR shapes — work-list
/// `bool-str-and-float-flags-coerce-from-their-ir-shapes`, whose string and
/// float halves retired with their flags (M1e2 S1e2.1a.5: every flag a program
/// can set is a bool). Since M1f S1f.5.5's swap a flag takes `true` or `false`
/// and nothing else, and every other shape a `(config …)` was refused for is the
/// grammar's: the quoted `"false"` this test coerced until then (a string,
/// S1f.5.2), an unknown flag, a value no bool, and a flag retired at M1e2, with
/// its *was retired at* sentence as a note (the paper grammar's default 5).
/// Each is a row of `ein-ir`'s grammar table since T1f.5.5.5 deleted the
/// loader's lines and moved their tests there (`the_retired_flags_are_refused_with_why`
/// and `the_config_rejections_name_the_flag_and_the_shape_they_wanted` until
/// then). The coercions the retired flags reached stay the container's
/// (`ein-einb` round-trips every field).
#[test]
fn bool_flags_coerce_from_their_ir_shapes() {
    // `lattice-sanity-check` since M1e2 S1e2.2.5 retired `warn-derived-naf`.
    assert!(cfg(":lattice-sanity-check true").lattice_sanity_check);
    assert!(!cfg(":lattice-sanity-check false").lattice_sanity_check);
    // `record-alternative-justifications` since M1e2 S1e2.1c.5 retired
    // `enable-path-nogoods`, which carried it.
    assert!(!cfg(":record-alternative-justifications false").record_alternative_justifications);
    // The last of a repeated flag wins.
    assert!(cfg(":lattice-sanity-check false :lattice-sanity-check true").lattice_sanity_check);
    assert_eq!(
        SolverConfig::default().lattice_order_seed,
        None,
        "an unset optional field is absent, not zero"
    );
}

// ── S1.22.4 — the arity-1 membership channel ───────────────────────

const DECLS: &str = "(relation adult Person :why \"{?1} is an adult\")\n\
                     (relation likes Person Drink :why \"{?1} likes {?2}\")\n\
                     (relation between Person Person Person :why \"{?1} is between {?2} and {?3}\")\n";

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
         (rule sees-any    () :match (relation ?R)       :assert (is-rel ?R) :why \"{{?R}} is a declared relation, so is-rel holds of it\")\n\
         (rule sees-unary  () :match (relation ?R ?A)    :assert (is-unary ?R) :why \"{{?R}} is declared over {{?A}} alone, so it is unary\")\n\
         (rule sees-binary () :match (relation ?R ?A ?B) :assert (is-binary ?R) :why \"{{?R}} is declared over {{?A}} and {{?B}}, so it is binary\")\n"
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
        "(relation likes Person Drink :why \"{?1} likes {?2}\")\n\
         (symmetric likes)\n\
         (rule sees-any () :match (relation ?R) :assert (is-rel ?R) :why \"{?R} is a declared relation, so is-rel holds of it\")\n",
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
    let l = load_text("(relation opaque :why \"opaque holds\")\n");
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
    // Since M1f S1f.5.5's swap a declaration's signature is names, so the
    // wrapped list is a parse error at its `(`, where the loader refused it as
    // `malformed (relation)` until then: a row of `ein-ir`'s grammar table
    // since T1f.5.5.5 deleted the line.
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
        "(relation opaque :why \"opaque holds\")\n\
         (relation likes Person Drink :why \"{?1} likes {?2}\")\n\
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
        "(relation between Person Person Person :why \"{?1} is between {?2} and {?3}\")\n\
         (relation likes Person Drink :why \"{?1} likes {?2}\")\n\
         (is-a Jack Person) (is-a Jill Person)\n",
    );
    assert_eq!(
        guessed,
        BTreeSet::from(["likes".to_string()]),
        "only the binary relation is filled"
    );
}
