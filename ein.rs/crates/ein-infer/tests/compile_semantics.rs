//! What a `:match` compiles to, what it is refused for, and what a
//! `(config …)` block coerces to — T1a.10.2.2.
//!
//! Replaces four Python files, all of which asserted the *semantics* of the
//! front half of the engine rather than any Python object:
//!
//! | Python original | subject |
//! |---|---|
//! | `ein.py/tests/inference/test_compile.py` | the in-`:match` kw_pair drop, and S1.22.0's four refusals |
//! | `ein.py/tests/inference/test_compile_negative.py` | each refusal's message, against its `.expected` fixture |
//! | `ein.py/tests/inference/test_config.py` | `SolverConfig` coercion from IR literals (the F-KER-4 regression) |
//! | `ein.py/tests/inference/test_relation_arity.py` | the arity-1 `(relation R)` membership channel and bare declarations |
//!
//! The Python tests reached into `plan.steps[0]`, `kb._facts_by_relation` and
//! `SolverConfig.from_kw_pairs`. Where the two implementations agree on a
//! *rendering* the assertion is on that rendering — [`ein_infer::plan_shape`]
//! is the compiler's only readable surface and is the text ein.py's oracle
//! produced too. Everywhere else it is on the observable the structure existed
//! for: the facts a rule derives, the candidates the enumerator proposes, the
//! message a rule author reads.
//!
//! `compile_parity.rs` covers the same four messages, but against the live
//! oracle: once `ein.py/` is deleted `Oracle::start` returns `None` and that
//! test skips forever. The `.expected`-file half is re-asserted here **with no
//! oracle**, which is the form that outlives the delete.

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
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, Some("<fixture>")).expect("the fixture parses");
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

// ── S1.22.0 — the four refusals ────────────────────────────────────

/// Each fixture under `examples/broken/compile/` is refused with exactly the
/// message in its `.expected` file — work-list
/// `the-four-compile-errors-say-what-their-expected-file-says`.
///
/// These messages are a **surface**: a rule author reads them, and three of the
/// four name the offending IR form in Python's `repr`, which is the only reason
/// two implementations could ever be compared on them. Checking the directory
/// listing as well is not tidiness — a fixture renamed away from its
/// `.expected`, or a fifth refusal added without one, would leave a branch of
/// the compiler with nobody asserting its text.
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
            "activator_arity",
            "empty_absent",
            "nested_or",
            "unbound_head"
        ],
        "one fixture per CompileError branch, in {}",
        dir.display()
    );
    for path in &fixtures {
        let name = path.file_name().expect("a file").to_string_lossy();
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{name}: {e}"));
        let expected = std::fs::read_to_string(path.with_extension("expected"))
            .unwrap_or_else(|e| panic!("{name}.expected: {e}"));
        let mut l = load_text(&text);
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
/// site has a checked-in fixture, so the negative one is built here: a port
/// that guarded only the site it had a fixture for would look correct.
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
/// Both routes into the branch are checked. `(absent ?x)` reaches the empty
/// sub-plan itself; `(absent (?R ?x))` is the shape an author is far likelier
/// to write, and it stops *earlier*, at the unbound-head refusal. That
/// ordering is what the two `.expected` files record, and a compiler that
/// checked emptiness before heads would swap the messages while still
/// refusing both.
#[test]
fn an_absent_that_compiles_to_no_steps_is_refused() {
    let mut empty = load_text(
        "(relation p T) (relation out T)\n\
         (rule vacuous ()\n  :match (and (p ?x) (absent ?x))\n  :assert (out ?x))\n",
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

    let mut unbound = load_text(
        "(relation a T) (relation out T)\n\
         (rule free-head ()\n  :match (and (a ?x) (absent (?R ?x)))\n  :assert (out ?x))\n",
    );
    let err = unbound.compile_error(true);
    assert!(
        err.starts_with("unbound relation head ?R"),
        "an unbound head inside an `(absent …)` reaches the head refusal, not \
         the empty-sub-plan one: {err}"
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

// ── `(config …)` coercion ──────────────────────────────────────────

/// Numeric flags take their value from an IR integer literal — work-list
/// `numeric-config-flags-load-through-the-ir`.
///
/// F-KER-4: for a while *every* numeric flag was unsettable from a
/// `(config …)` block, and the suite did not notice because it built its
/// configs in code and only the IR path was broken. Both numeric shapes are
/// here because they failed for unrelated reasons — an `int | None` field was
/// rejected by a type comparison that could not see through the annotation, a
/// plain `int` field by unwrapping the IR node through the wrong attribute —
/// so a port could easily fix one and reintroduce the other. The unset default
/// is the third: `None` is what makes `lattice-order-seed` mean "no shuffle"
/// rather than "seed 0", and the two orders differ.
#[test]
fn numeric_config_flags_load_through_the_ir() {
    assert_eq!(cfg(":lattice-order-seed 7").lattice_order_seed, Some(7));
    assert_eq!(cfg(":candidate-order-seed 5").candidate_order_seed, 5);
    assert_eq!(
        SolverConfig::default().lattice_order_seed,
        None,
        "an unset optional flag is absent, not zero"
    );
}

/// The other three coercions read their IR shapes — work-list
/// `bool-str-and-float-flags-coerce-from-their-ir-shapes`.
///
/// The float cases are the ones worth writing down: **the ein-lang grammar has
/// no float token**, so a float flag can only ever arrive as an integer
/// literal or as a quoted string, and both have to work or the flag is
/// unreachable from a file at all. `2` has to become `2.0` rather than be
/// refused for not being a float, and `"1.5"` has to be parsed rather than
/// kept as text — the string route is the *only* way to write a non-integral
/// weight.
#[test]
fn bool_str_and_float_flags_coerce_from_their_ir_shapes() {
    assert!(cfg(":print-alive true").print_alive);
    assert!(!cfg(":print-alive false").print_alive);
    assert_eq!(cfg(":lattice-order lex").lattice_order, "lex");
    assert_eq!(cfg(":hypgen-rel-weight 2").hypgen_rel_weight, 2.0);
    assert_eq!(cfg(r#":hypgen-rel-weight "1.5""#).hypgen_rel_weight, 1.5);
}

/// The three refusals name the flag and the shape they wanted — work-list
/// `the-three-config-rejections-say-what-they-say`.
///
/// A `(config …)` block is hand-authored and its keys are easy to misspell, so
/// the unknown-flag message enumerates the accepted set rather than merely
/// saying no: that enumeration is the only discovery mechanism the surface
/// language offers for these. The type refusals matter for the opposite
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
    for known in ["candidate-order-seed", "lattice-order-seed", "print-alive"] {
        assert!(
            unknown.contains(known),
            "the accepted set must be enumerated, and {known} is missing: {unknown}"
        );
    }

    let not_an_int = config(":candidate-order-seed foo").expect_err("not an integer");
    assert!(
        not_an_int.contains("config flag :candidate-order-seed expects an integer"),
        "{not_an_int}"
    );

    let not_a_bool = config(":print-alive 7").expect_err("not a boolean");
    assert!(
        not_a_bool.contains("config flag :print-alive expects true/false"),
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
/// bare declaration, so deleting it would not move a single parity run — which
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
/// 2, so nothing in the parity corpus can tell "skipped" from "never
/// encountered", and a port that quietly emitted a truncated binary candidate
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
