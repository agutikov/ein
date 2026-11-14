//! What a rule *means* when it fires — S1a.10.2 **T1a.10.2.2**.
//!
//! The common subject is what one rule application *is*: which bindings a
//! premise admits (repeated variables, nested patterns, stored `not`), which
//! bindings a guard keeps (`eq` / `neq`, over variables and over literals), how
//! many facts one match concludes (`:assert (and …)` × `:match (or …)`), what
//! `forall` expands to, and when a parametrised rule is allowed to run at all.
//!
//! **Every claim here is asserted as a derivation.** Plan opcodes and raw
//! bindings are the *encoding* of a rule — ein.rs resolves variables to
//! registers and lifts `(absent …)` to a boundary guard — so an opcode
//! assertion would pin this build rather than the language. What the language
//! fixes is which facts a program derives, so each test below saturates a
//! fixture and reads the firings and the KB.

use ein_core::{Kb, Terms};
use ein_infer::events::{binding_pairs, sexpr};
use ein_infer::saturator::{Saturator, Session};
use ein_infer::{Events, Firing, SharedMemo};
use ein_ir::{Ast, parse};
use std::collections::BTreeSet;

/// A saturated fixture: the KB at the fixpoint and every rule application that
/// got it there.
struct Run {
    kb: Kb,
    terms: Terms,
    firings: Vec<Firing>,
}

/// Parse, load and saturate `text`, surfacing any refusal as its message.
///
/// The whole pipeline is behind one `Result` on purpose: two of the claims
/// below are about *where* a program is refused (a `forall` whose bound
/// variable escapes into the head is refused; one whose bound variable never
/// appears in its guard is not), and a helper that unwrapped each phase
/// separately would have to guess which phase to unwrap in.
fn try_saturate(text: &str) -> Result<Run, String> {
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, Some("<fixture>")).map_err(|e| e.to_string())?;
    saturate_forms(ast, &forms)
}

/// Load and saturate the forms a parser built into `ast`.
fn saturate_forms(mut ast: Ast, forms: &[ein_ir::NodeId]) -> Result<Run, String> {
    let mut terms = Terms::new();
    let mut kb = ein_ir::load(&mut ast, &mut terms, forms, None).map_err(|e| e.to_string())?;
    let mut firings = Vec::new();
    {
        let mut events = Events::off();
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).map_err(|e| e.to_string())?;
        // A budget, not a limit: every fixture here is a handful of facts, so
        // hitting it means a rule is re-deriving forever and the test should
        // say so instead of hanging.
        sat.saturate(&mut s, Some(2_000), &mut |f| firings.push(f.clone()))
            .map_err(|e| e.to_string())?;
    }
    Ok(Run { kb, terms, firings })
}

fn saturate(text: &str) -> Run {
    try_saturate(text).unwrap_or_else(|e| panic!("the fixture should saturate: {e}"))
}

impl Run {
    /// What saturation *added*, as `(rel arg …)` — the conclusions of the
    /// productive firings. A redundant firing re-derives what the file already
    /// stated, so counting it here would let a fixture's own facts pass as
    /// derivations.
    fn derived(&self) -> BTreeSet<String> {
        self.firings
            .iter()
            .filter(|f| !f.redundant)
            .flat_map(|f| f.derived.iter().map(|&d| sexpr(&self.terms, d)))
            .collect()
    }

    /// The same conclusions **by value**: each fact's `repr`, which keeps a
    /// string quoted and an int bare. `derived` prints a fact as the trace
    /// does, and that printer drops a string's quotes (P1e2.3's N5), so the
    /// int `1` and the string `"1"` would be one line there.
    fn derived_values(&self) -> BTreeSet<String> {
        self.firings
            .iter()
            .filter(|f| !f.redundant)
            .flat_map(|f| {
                f.derived
                    .iter()
                    .map(|&d| ein_core::repr::repr(&self.terms.repr_fact(d)))
            })
            .collect()
    }

    /// Every fact the KB holds at the fixpoint, including the file's own and
    /// the `(relation …)` declarations.
    fn facts(&self) -> BTreeSet<String> {
        self.kb.facts().map(|f| sexpr(&self.terms, f)).collect()
    }

    /// The productive firings — one entry per rule application.
    fn productive(&self) -> Vec<&Firing> {
        self.firings.iter().filter(|f| !f.redundant).collect()
    }

    /// The rule name a firing reports, which is what `--events` prints as
    /// `fire rule=…` and what a trace shows the reader.
    fn rule_of(&self, f: &Firing) -> &str {
        self.terms.sym(f.rule)
    }

    /// The variables a firing bound, as the trace renders them.
    fn bindings_of(&self, f: &Firing) -> Vec<(String, String)> {
        binding_pairs(&self.terms, &f.bindings)
    }

    /// One firing's conclusions, in template order.
    fn conclusions_of(&self, f: &Firing) -> Vec<String> {
        f.derived.iter().map(|&d| sexpr(&self.terms, d)).collect()
    }
}

// ── `forall` ───────────────────────────────────────────────────────────────
//
// Since P1.8 S1.5.9 `forall` is not kernel vocabulary: it is a `(macro …)` in
// `stdlib/macro.ein`, expanded by the loader to `(absent (and G (absent B)))`
// before the compiler sees anything. Every fixture below therefore imports it,
// and what is under test is the classical ¬∃b. G(b) ∧ ¬B(b) reduction rather
// than any engine support for universals.

/// **A `forall` over an empty domain is vacuously true.**
///
/// This is the case a reader distrusts, because "Alice beat everyone" sounds
/// false when Alice played nobody — and it is the case an implementation gets
/// wrong by enumerating the guard and requiring a witness. The two fixtures
/// differ by *one player*, not by one `beats` fact: with an opponent present
/// and unbeaten the rule must stay silent, and with the opponent removed the
/// same rule must fire. Nothing about Alice changed between them, which is
/// what makes the firing attributable to the empty domain.
///
/// The `examples/features/03_forall.ein` trailing comment describes exactly
/// this variation ("delete ALL beats facts AND remove Bob and Carol"); it is
/// built inline here because the corpus copy would need a `corpus.toml` entry.
#[test]
fn a_forall_over_an_empty_domain_is_vacuously_true() {
    let tournament = |roster: &str| {
        format!(
            r#"
            (import std.macro :symbols (forall))
            (relation player T :why "{{?1}} is a player") (relation beats T T :why "{{?1}} beats {{?2}}") (relation undefeated T :why "{{?1}} is undefeated")
            (rule lone-undefeated ()
              :match  (and (player ?p)
                           (forall ?q (and (player ?q) (neq ?p ?q))
                                      (beats ?p ?q)))
              :assert (undefeated ?p)
              :why    "{{?p}} beats every other player")
            {roster}
            "#
        )
    };

    let with_an_opponent = saturate(&tournament("(player Alice) (player Bob)"));
    assert!(
        with_an_opponent.derived().is_empty(),
        "Bob is unbeaten, so the guard has a witness and nobody is undefeated — \
         got {:?}",
        with_an_opponent.derived()
    );

    let alone = saturate(&tournament("(player Alice)"));
    assert_eq!(
        alone.derived(),
        BTreeSet::from(["(undefeated Alice)".to_string()]),
        "with no other player the guard yields no bindings, so the forall is \
         vacuously true and the rule must fire"
    );
}

/// **The `forall`-bound variable is local to the guard.**
///
/// `?q` is quantified by the macro's expansion, so it lives inside the
/// `(absent …)` sub-query and the outer environment never sees it. That is
/// invisible in the happy case — the rule concludes `(all-beaten Alice)`
/// whether or not `?q` leaked — so the claim is pinned from both sides: the
/// firing's bindings, which `--events` prints verbatim, name `?p` and nothing
/// else; and a rule that puts `?q` in its head is refused, naming `?q` as the
/// variable no premise binds. A leaked binding would satisfy that head
/// instead. **Since M1f S1f.5.10 the refusal is the loader's** (F13): until
/// then the rule loaded and its first firing stopped the run with a message
/// that named `?q` unbound while showing `?p` bound. `all-beaten` is declared
/// by nothing here: the fixture concludes it with one argument and with two,
/// and a declared relation has one count (M1f S1f.6.23).
#[test]
fn the_forall_bound_variable_does_not_escape_the_guard() {
    let tournament = |head: &str| {
        format!(
            r#"
            (import std.macro :symbols (forall))
            (relation player T :why "{{?1}} is a player") (relation beats T T :why "{{?1}} beats {{?2}}")
            (rule all-beaten ()
              :match  (and (player ?p)
                           (forall ?q (and (player ?q) (neq ?p ?q))
                                      (beats ?p ?q)))
              :assert {head}
              :why    "{{?p}} beat them all")
            (player Alice) (player Bob)
            (beats Alice Bob)
            "#
        )
    };

    let run = saturate(&tournament("(all-beaten ?p)"));
    let [firing] = run.productive()[..] else {
        panic!("expected exactly one firing, got {:?}", run.derived());
    };
    assert_eq!(
        run.bindings_of(firing),
        vec![("p".to_string(), "Alice".to_string())],
        "the surviving match binds only the outer ?p — the guard's ?q must not \
         reach the environment the conclusion is built from"
    );

    let err = match try_saturate(&tournament("(all-beaten ?p ?q)")) {
        Err(e) => e,
        Ok(leaked) => panic!(
            "?q escaped the guard: the head was satisfied and derived {:?}",
            leaked.derived()
        ),
    };
    assert!(
        err.contains(
            "rule 'all-beaten': its :assert writes ?q, which no premise or parameter binds"
        ),
        "the head's ?q should be refused at load as a variable no premise binds, got: {err}"
    );
    assert!(
        !err.contains("writes ?p"),
        "?p is bound by the outer premise, so only ?q is named, got: {err}"
    );
}

/// **A `forall` whose bound variable never appears in its guard still
/// compiles** — and this is a deliberate loss, not an oversight.
///
/// Before P1.8 S1.5.9 the compiler desugared `forall` itself and refused such
/// a rule ("?bound must appear in the guard", because there is no domain to
/// enumerate). `forall` is now an ordinary `(macro …)`, and macro
/// expansion does no per-macro validation, so the rejection left with the
/// desugar. What is left is a rule that means something other than what its
/// author wrote, and the two cells below show it *runs*: with no `(whatever …)`
/// fact the expansion `(absent (and (player ?other) (absent (whatever ?v))))`
/// is false and nothing fires; add one and it fires. A test that only checked
/// "no error" would pass just as well against a rule silently dropped on the
/// floor, which is why both cells are here.
#[test]
fn a_forall_whose_bound_var_is_absent_from_its_guard_still_compiles() {
    let bad_rule = |extra: &str| {
        format!(
            r#"
            (import std.macro :symbols (forall))
            (relation player T :why "{{?1}} is a player") (relation whatever T :why "whatever holds of {{?1}}") (relation oops :why "oops: bad-rule's malformed forall held")
            (rule bad-rule ()
              :match  (forall ?v (player ?other) (whatever ?v))
              :assert (oops)
              :why    "its malformed forall over player and whatever holds, so oops holds")
            (player Alice)
            {extra}
            "#
        )
    };

    let silent = try_saturate(&bad_rule("")).expect("a malformed forall is not rejected");
    assert!(
        silent.derived().is_empty(),
        "with no (whatever …) the inner absent holds, so the outer one fails — \
         got {:?}",
        silent.derived()
    );

    let fires = try_saturate(&bad_rule("(whatever X)")).expect("still not rejected");
    assert_eq!(
        fires.derived(),
        BTreeSet::from(["(oops)".to_string()]),
        "the expansion really is a compiled, running plan: one (whatever …) \
         fact flips it — which is the point, because it means the rule now \
         asks a question its author did not write"
    );
}

// ── premises ───────────────────────────────────────────────────────────────

/// **A repeated variable in one premise is an equality constraint.**
///
/// `(?rel ?a ?a)` is not two independent slots that happen to share a name:
/// the second occurrence must unify with what the first bound. So `(r X X)`
/// matches and `(r Y Z)` does not, and the fixture holds both facts so that a
/// matcher which forgot the constraint would visibly derive `(self-loop Y)`
/// as well.
#[test]
fn a_repeated_variable_is_an_equality_constraint() {
    let run = saturate(
        r#"
        (relation r T T :why "r holds of {?1} and {?2}") (relation self-loop T :why "{?1} has a self-loop")
        (rule self (?rel)
          :match  (?rel ?a ?a)
          :assert (self-loop ?a)
          :why    "{?a} points at itself")
        (self r)
        (r X X)
        (r Y Z)
        "#,
    );
    assert_eq!(
        run.derived(),
        BTreeSet::from(["(self-loop X)".to_string()]),
        "only the fact whose two arguments are equal may match (?rel ?a ?a)"
    );
    let [firing] = run.productive()[..] else {
        panic!("expected one firing, got {:?}", run.derived());
    };
    assert!(
        run.bindings_of(firing).contains(&("a".into(), "X".into())),
        "the two unified slots leave one binding, ?a = X, not two"
    );
}

/// **`(not P)` in a premise is stored negation, never negation-as-failure.**
///
/// K-Δ.1 (S1.5.8c): `not` is a relation head like any other, so `(not P)`
/// matches a fact that was *written down* as `(not P)` and nothing else. The
/// tempting reading is the opposite one — "P is not derivable" — and the two
/// fixtures separate them exactly: they differ only in whether the KB carries
/// `(not (other X Y))` or `(other X Y)`. Under NAF the second would match
/// nothing *because P holds*; under stored negation it matches nothing because
/// no `(not …)` fact exists. So the first fixture, where the stored negative is
/// present and the positive absent, is the one that tells them apart: NAF would
/// match there too, but so would this — and the pair together only admits the
/// stored reading. `(absent P)` is the explicit NAF operator, and it is a
/// different premise.
#[test]
fn not_in_a_premise_is_stored_negation_not_negation_as_failure() {
    let see_neg = |evidence: &str| {
        format!(
            r#"
            (relation r T T :why "r holds of {{?1}} and {{?2}}") (relation other T T :why "other holds of {{?1}} and {{?2}}") (relation saw-neg T T :why "a stored negation of other was seen for {{?1}} and {{?2}}")
            (rule see-neg (?rl)
              :match  (and (?rl ?a ?b) (not (other ?a ?b)))
              :assert (saw-neg ?a ?b)
              :why    "{{?rl}} holds of {{?a}} and {{?b}} and other is stored false of them, so a negation was seen")
            (see-neg r)
            (r X Y)
            {evidence}
            "#
        )
    };

    let stored = saturate(&see_neg("(not (other X Y))"));
    assert_eq!(
        stored.derived(),
        BTreeSet::from(["(saw-neg X Y)".to_string()]),
        "a written-down (not (other X Y)) is a fact, and a (not …) premise \
         matches it the way any premise matches its head's storage"
    );

    let positive_only = saturate(&see_neg("(other X Y)"));
    assert!(
        positive_only.derived().is_empty(),
        "with only the positive fact and no stored negative there is nothing \
         for the premise to match — got {:?}",
        positive_only.derived()
    );
}

/// **A nested pattern binds inside the nested fact.**
///
/// `not` is the nesting most files use, which makes it easy to believe nesting
/// is special-cased for negation. It is not: any relation may take a fact as an
/// argument, and a premise written against it unifies structurally and binds
/// the *inner* variables. The fixture uses `hypothesis`, an ordinary declared
/// relation, so a special case for `not` would not save it — and the KB is
/// checked to still hold the nested fact as a nested fact, because a loader
/// that flattened the argument to a name would let the rule match for the
/// wrong reason.
///
/// Q40; the surface language writes the nested fact directly.
#[test]
fn a_nested_pattern_binds_inside_the_nested_fact() {
    let run = saturate(
        r#"
        (relation co-located T T :why "{?1} and {?2} are co-located") (relation hypothesis T :why "{?1} is a hypothesis") (relation caught T T :why "the trap caught {?1} and {?2}")
        (rule trap ()
          :match  (hypothesis (co-located ?a ?b))
          :assert (caught ?a ?b)
          :why    "the hypothesis names {?a} and {?b}")
        (hypothesis (co-located Norwegian House-2))
        "#,
    );
    assert!(
        run.facts()
            .contains("(hypothesis (co-located Norwegian House-2))"),
        "the KB must hold a fact whose argument is itself a fact — otherwise \
         the match below proves nothing about nesting"
    );
    assert_eq!(
        run.derived(),
        BTreeSet::from(["(caught Norwegian House-2)".to_string()]),
        "?a and ?b are bound one level down, inside the nested (co-located …)"
    );
}

// ── one match, several conclusions ─────────────────────────────────────────
//
// P1.8 S1.8.A13: `:assert (and c1 … ck)` and `:match (or d1 … dm)` are lowered
// to several assert templates and several match plans on **one** rule. They
// used to SPLIT the rule into `__and<j>` / `__or<i>` clones, and the two tests
// below are the two things that split broke.

/// **A parameterised rule may multi-assert.**
///
/// This is the case the clone-based lowering had to reject outright (S1.8.A11):
/// a generic rule is reached by an activator fact that names it, and splitting
/// `place-and-exclude` into `place-and-exclude__and0/1` left no name for
/// `(place-and-exclude color-loc)` to resolve to. So the assertion that matters
/// is not "two facts appeared" but "two facts appeared *from a rule the
/// activator could reach*" — hence the firing's own rule name is checked
/// against the name in the source, which is what `--events` and a trace print.
#[test]
fn a_parameterised_rule_may_multi_assert() {
    let run = saturate(
        r#"
        (rule place-and-exclude (?rel)
          :match  (and (?rel ?a ?x) (slot ?y) (neq ?x ?y))
          :assert (and (?rel ?a ?x) (not (?rel ?a ?y)))
          :why    "{?a} is {?x} via {?rel}, so not {?y}")
        (relation color-loc Color House :why "{?2} is painted {?1}") (relation slot T :why "{?1} is a slot")
        (place-and-exclude color-loc)
        (color-loc Red House-1)
        (slot House-2)
        "#,
    );
    let [firing] = run.productive()[..] else {
        panic!(
            "expected one application of the generic rule, got {:?}",
            run.derived()
        );
    };
    assert_eq!(
        run.rule_of(firing),
        "place-and-exclude",
        "the rule must keep the name its activator names — a clone would fire \
         under a name no `(place-and-exclude …)` fact could ever reach"
    );
    assert_eq!(
        run.conclusions_of(firing),
        vec![
            "(color-loc Red House-1)".to_string(),
            "(not (color-loc Red House-2))".to_string(),
        ],
        "both templates are instantiated by the one match, in source order, \
         with ?rel resolved to the activator's relation in each"
    );
}

/// **Every disjunct emits every template.**
///
/// `(or d1 d2)` × `(and c1 c2)` is 2 × 2 = 4 facts, and the shape of the
/// derivation is the claim: *two* applications of one rule, each concluding
/// both templates — not four applications, and not a `__or<i>__and<j>` grid of
/// four rules. The fact set alone cannot tell those apart, so the test counts
/// firings, reads each one's rule name, and checks that the two conclusions of
/// an application **share a provenance**: one justification is what makes them
/// one derivation step for the explainer and the trace.
#[test]
fn every_disjunct_emits_every_template() {
    let run = saturate(
        r#"
        (relation a T :why "a holds of {?1}") (relation b T :why "b holds of {?1}") (relation p T :why "p holds of {?1}") (relation q T :why "q holds of {?1}")
        (rule x ()
          :match  (or (a ?n) (b ?n))
          :assert (and (p ?n) (q ?n))
          :why    "{?n} came in through one of the disjuncts")
        (a A)
        (b B)
        "#,
    );
    assert_eq!(
        run.derived(),
        BTreeSet::from([
            "(p A)".to_string(),
            "(q A)".to_string(),
            "(p B)".to_string(),
            "(q B)".to_string(),
        ]),
        "each disjunct's match must fire the whole template list"
    );
    let firings = run.productive();
    assert_eq!(
        firings.len(),
        2,
        "one application per disjunct match, each concluding both facts — not \
         one per (disjunct, template) pair"
    );
    for f in &firings {
        assert_eq!(
            run.rule_of(f),
            "x",
            "the rule keeps its single name; no __or/__and clones exist to fire"
        );
        assert_eq!(run.conclusions_of(f).len(), 2, "both templates, one match");
        let provs: Vec<_> = f.derived.iter().map(|&d| run.kb.primary(d)).collect();
        assert_eq!(
            provs[0], provs[1],
            "the two conclusions of one application share one justification — \
             that is what makes them one step rather than two"
        );
    }
}

// ── guards ─────────────────────────────────────────────────────────────────

/// **`eq` admits exactly the bindings `neq` rejects.**
///
/// The registry ships two predicates and they are meant to be complements, but
/// only `neq` is exercised by the corpus — every shipping rule that guards at
/// all guards on difference — so `eq` could be wrong in either direction and
/// nothing would notice. The fixture makes all four `(p ?a) × (q ?b)` pairs
/// available and runs both predicates over them, so the claim can be stated as
/// a partition: the two derived sets are disjoint and together cover every
/// candidate binding. An `eq` that always failed, or one that ignored an
/// argument, breaks that identity rather than merely shrinking a set.
#[test]
fn eq_admits_exactly_the_bindings_neq_rejects() {
    let run = saturate(
        r#"
        (relation p T :why "p holds of {?1}") (relation q T :why "q holds of {?1}") (relation same T T :why "{?1} and {?2} are the same") (relation diff T T :why "{?1} and {?2} differ")
        (rule same-pair ()
          :match  (and (p ?a) (q ?b) (eq ?a ?b))
          :assert (same ?a ?b)
          :why    "{?a} and {?b} resolve equal")
        (rule diff-pair ()
          :match  (and (p ?a) (q ?b) (neq ?a ?b))
          :assert (diff ?a ?b)
          :why    "{?a} and {?b} resolve unequal")
        (p X) (p Y)
        (q X) (q Y)
        "#,
    );
    let pairs = |rel: &str| -> BTreeSet<String> {
        run.derived()
            .iter()
            .filter_map(|f| f.strip_prefix(&format!("({rel} ")))
            .map(|rest| rest.trim_end_matches(')').to_string())
            .collect()
    };
    let (equal, unequal) = (pairs("same"), pairs("diff"));
    assert_eq!(
        equal,
        BTreeSet::from(["X X".to_string(), "Y Y".to_string()]),
        "eq holds exactly when both arguments resolve to the same value"
    );
    assert!(
        equal.is_disjoint(&unequal),
        "no binding may satisfy both guards: {equal:?} ∩ {unequal:?}"
    );
    assert_eq!(
        equal.union(&unequal).count(),
        4,
        "and between them they must admit all four (p ?a) × (q ?b) bindings — \
         a guard that silently dropped one would leave a gap here"
    );
}

/// **A guard argument may be a literal, compared against the bound value.**
///
/// A guard's arguments are raw IR nodes resolved against the runtime
/// environment, so the question "is `Red` a name to look up, or a value?" has a
/// real answer, and getting it wrong is not loud: an implementation that
/// resolved `Red` as an unbound *variable* would compare against "nothing" and
/// silently admit nothing, or everything. The fixture pins an `Atom` of
/// letters and one of digits, each a name (`5` was an `Int`, a number, until
/// M1f S1f.5.4 retired the kind: since S1f.5.5's swap a run of digits is a
/// name), and each rule is given a non-matching companion fact so that "fires
/// for the right one" and "fires for everything" are distinguishable.
#[test]
fn a_guard_argument_may_be_a_literal() {
    let run = saturate(
        r#"
        (relation color T :why "{?1} is a colour") (relation is-red T :why "{?1} is red")
        (relation n T :why "n holds of {?1}") (relation is-five T :why "{?1} is five")
        (rule red ()
          :match  (and (color ?x) (eq ?x Red))
          :assert (is-red ?x)
          :why    "{?x} is the atom Red")
        (rule five ()
          :match  (and (n ?k) (eq ?k 5))
          :assert (is-five ?k)
          :why    "{?k} is the name 5")
        (color Red) (color Green)
        (n 5) (n 6)
        "#,
    );
    assert_eq!(
        run.derived(),
        BTreeSet::from(["(is-red Red)".to_string(), "(is-five 5)".to_string()]),
        "a literal of letters and one of digits each compare against the bound \
         name — neither is resolved as a variable, and neither matches the \
         companion fact"
    );
}

// ── activation ─────────────────────────────────────────────────────────────

/// **A rule that declares parameters is dormant until a fact names it.**
///
/// A parametrised rule is a *schema*: `(rule implies (?p ?q) :match (?p ?a ?b)
/// …)` has no relation to scan until `(implies co-located next-to)` says which.
/// Dormancy is therefore not "it matched nothing" — a plan compiled with `?p`
/// left unbound would scan every relation in the KB and conclude `(?q A B)`
/// with `?q` unbound, which is an error, not silence. So the fixture keeps a
/// `(right-of A B)` fact on hand in all three cells: with no activator, and
/// with an activator naming a *different* source relation, the rule must derive
/// nothing at all; the third cell supplies the matching activator and shows the
/// rule is otherwise perfectly able to fire, which is what stops the first two
/// from passing for some unrelated reason.
#[test]
fn a_parametrised_rule_with_no_activator_is_dormant() {
    let program = |activator: &str| {
        format!(
            r#"
            (rule implies (?p ?q)
              :match  (?p ?a ?b)
              :assert (?q ?a ?b)
              :why    "{{?p}} implies {{?q}}"
)
            (relation co-located T T :why "{{?1}} and {{?2}} are co-located") (relation next-to T T :why "{{?1}} is next to {{?2}}") (relation right-of T T :why "{{?1}} is right of {{?2}}")
            {activator}
            (right-of A B)
            "#
        )
    };

    let dormant = saturate(&program(""));
    assert!(
        dormant.firings.is_empty(),
        "no activator, no plan, no firing — not even a redundant one: {:?}",
        dormant.derived()
    );

    let wrong = saturate(&program("(implies co-located next-to)"));
    assert!(
        wrong.firings.is_empty(),
        "an activator authorises the rule over the relation it names and no \
         other, so a (right-of …) fact must not wake it: {:?}",
        wrong.derived()
    );

    let armed = saturate(&program("(implies right-of next-to)"));
    assert_eq!(
        armed.derived(),
        BTreeSet::from(["(next-to A B)".to_string()]),
        "with the matching activator the very same rule and the very same fact \
         derive — so the two silences above are the activator's doing"
    );
}

// ── the activator's identity — Q-M1a.8, Q-M1e.16 ───────────────────────────
//
// `BindingKey` is `(rule, activator, values)`, and three keys read one
// activator:
//
// | key | what it keeps of the activator |
// |---|---|
// | [`PlanKey`] — the compile cache | every argument, stringified |
// | `BindingKey.activator` — an interned `PlanKey.activator` | the same list: every argument |
// | `BindingKey.values` — the register file | every argument that **binds a parameter**: symbols *and* ints |
//
// Until M1e2 S1e2.2.7 the middle row was an interned `plan.activator_args`,
// the **symbol** arguments. `defined_behaviour.md` §3.2 read it alone and
// concluded that two activators differing only in an `int` argument share an
// identity. They did not: `bind_activator` seeds a register for every
// argument that is not a nested `Fact` (`compile.rs`, `if a.as_fact().is_some()
// { continue; }`), so the int is in `values` (M1e S1e.1.4). The arguments that
// reached **neither** half were exactly the nested `Fact`s, and an int beside
// a nested `Fact` in one position put two register layouts in one space and
// cost a derivation (Q-M1e.16, M1e2's W7). Since S1e2.2.7 the middle row is
// the compile key's list, so every plan has a space of its own. The three
// tests below are the three shapes, held under the key as it is now.
//
// Since M1f S1f.5.4 retired the int kind (a run of digits is a name since
// S1f.5.5's swap), the `int` the tests below name is a name of digits: their
// programs are unchanged, and a name binds a parameter and seeds a register
// as the int did, so each holds its case for names, the register row's
// "ints" being symbols.

/// **Two activators differing only in an `int` argument both fire** — which is
/// what `defined_behaviour.md` §3.2 said they could not do, and the probe that
/// refutes it (M1e [S1e.1.4](../../../../docs/history/m1e_review_processing/README.md#s1e14--q-m1a8s-real-trigger-q3),
/// the review's `Q3`).
///
/// The conclusions carry the int, so a suppressed firing is visible as a
/// missing *fact* rather than only as a missing event: `(tag edge 1)` and
/// `(tag edge 2)` authorise the same rule over the same edge and must derive
/// two different tags. Under §3.2 as written, one of them would be gone with
/// no diagnostic.
#[test]
fn activators_differing_only_by_an_int_argument_both_fire() {
    let run = saturate(
        r#"
        (relation edge   Node Node :why "there is an edge from {?1} to {?2}")
        (relation tagged Node Label :why "{?1} is tagged {?2}")
        (rule tag (?R ?n)
          :match  (?R ?a ?b)
          :assert (tagged ?a ?n)
          :why    "tag {?n}")
        (edge A B)
        (tag edge 1)
        (tag edge 2)
        "#,
    );
    assert_eq!(
        run.derived(),
        BTreeSet::from(["(tagged A 1)".to_string(), "(tagged A 2)".to_string()]),
        "an int activator argument seeds a register and so reaches \
         `BindingKey.values`; the two applications have different keys and \
         neither suppresses the other"
    );
    let bound: Vec<Vec<(String, String)>> = run
        .productive()
        .iter()
        .map(|f| run.bindings_of(f))
        .collect();
    assert_eq!(bound.len(), 2, "two applications, not one: {bound:?}");
    for (i, want) in ["1", "2"].iter().enumerate() {
        assert!(
            bound[i].contains(&("n".to_string(), want.to_string())),
            "firing {i} should bind ?n to {want}: {bound:?}"
        );
    }
}

/// **Two activators differing only in a nested `Fact` argument have two
/// binding keys** since M1e2 S1e2.2.7, and the second application is a
/// redundant firing — where until then they shared one key and it was never
/// enqueued, the collision §3.2 was reaching for.
///
/// Since M1f S1f.5.18 (W33) a nested `Fact` binds its parameter as a name
/// does, so it reaches `values` as well as the activator half, which is the
/// compile key's every argument. The same fixture is run four ways and only
/// the *kind* of the second argument varies:
///
/// | second argument | plans | firings | productive |
/// |---|---:|---:|---:|
/// | one nested `Fact` | 1 | 1 | 1 |
/// | two nested `Fact`s | 2 | **2** (1 until S1e2.2.7) | 1 |
/// | two symbols | 2 | 2 | 1 |
/// | two ints | 2 | 2 | 1 |
///
/// **And nothing changes but a duplicate.** The rule reads `?f` nowhere, so
/// whatever value each plan's seed binds it to, the second application
/// derives what the first derived, which is why the derived set is the same
/// in all four cells and one firing in each is productive. Until M1f S1f.5.18
/// `bind_activator` skipped a `Fact` argument outright, so the two plans were
/// equal in every field of [`Plan`] but their activator list; since, each
/// seed holds its own fact (W33), and the cells count as they did. Q-M1e.16's second fix, the layout in the key, would have kept
/// this collapse; the first, which S1e2.2.7 took, gives each plan its space.
/// The shape the collapse's key cost a derivation on is the next test.
#[test]
fn activators_differing_only_by_a_nested_fact_argument_both_fire() {
    let program = |activators: &str| {
        format!(
            r#"
            (relation edge  Node Node :why "there is an edge from {{?1}} to {{?2}}")
            (relation noted Node :why "{{?1}} is noted")
            (rule note (?R ?f)
              :match  (?R ?a ?b)
              :assert (noted ?a)
              :why    "{{?R}} holds of {{?a}} and {{?b}}, so {{?a}} is noted")
            (edge A B)
            {activators}
            "#
        )
    };
    let cells = [
        ("one nested fact", "(note edge (src X))", 1),
        (
            "two nested facts",
            "(note edge (src X)) (note edge (src Y))",
            2,
        ),
        ("two symbols", "(note edge sx) (note edge sy)", 2),
        ("two ints", "(note edge 1) (note edge 2)", 2),
    ];
    for (what, activators, firings) in cells {
        let run = saturate(&program(activators));
        assert_eq!(
            run.firings.len(),
            firings,
            "{what}: expected {firings} rule application(s), redundant ones \
             included — each plan has a binding-key space of its own, so a \
             second plan's application fires and finds its conclusion known"
        );
        assert_eq!(
            run.productive().len(),
            1,
            "{what}: one application concludes, and every other is redundant"
        );
        assert_eq!(
            run.derived(),
            BTreeSet::from(["(noted A)".to_string()]),
            "{what}: the conclusion cannot depend on an argument that binds \
             nothing, so every cell derives the same one fact and the \
             collision costs a duplicate"
        );
    }
}

/// **An `int` in the position another activator gives a nested `Fact` loses
/// no derivation, in either order and in either build** — M1e2 S1e2.2.7's
/// regression for the latent bug behind `Q-M1a.8`, which until then this test
/// pinned as a defect. The `int` is the name `1` since M1f S1f.5.4, and since
/// M1f S1f.5.18 (W33) the nested `Fact` binds `?f` too, so each plan derives
/// its own `(noted ?a ?f)`, and the two together their union.
///
/// Both activators drop their second argument from `plan.activator_args`, and
/// until S1e2.2.7 that list was the binding key's activator half, so the two
/// plans shared a `(rule, activator)` space. The int seeds a register and the
/// `Fact` does not, so they disagree on their *register layout*: `?f` is
/// register 1 in one plan and register 3 in the other, and `BindingKey`
/// compared `(?R ?f ?a ?b)` against `(?R ?a ?b ?f)` position by position.
/// `(edge 1 2 3)` is a legitimate match of both, so whichever fired first
/// suppressed the other, and with the int first the lost application was the
/// only one that derives `(noted 1 3)`. `Engine::check_layout` asserted the
/// invariant under `debug_assertions`, so a debug build panicked and a release
/// one answered wrongly.
///
/// The fix,
/// [Q-M1e.16](../../../../docs/history/m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one)'s
/// first: the activator half is the compile key's list, every argument, so
/// the two plans have two spaces and both fire. `check_layout` stays, and
/// cannot fire unless the two keys part again. The corpus family is
/// `examples/ein-bugs/binding-key-{both,swapped,int-only,nested-only}.ein`.
#[test]
fn an_int_beside_a_nested_fact_in_one_position_loses_no_derivation() {
    let program = |activators: &str| {
        format!(
            r#"
            (relation edge  Node Node :why "there is an edge from {{?1}} to {{?2}}")
            (relation holds Node :why "{{?1}} holds")
            (relation noted Node Node :why "{{?1}} is noted with {{?2}}")
            (rule note (?R ?f)
              :match  (and (?R ?a ?b) (holds ?f))
              :assert (noted ?a ?f)
              :why    "{{?R}} holds of {{?a}} and {{?b}} and {{?f}} holds, so {{?a}} is noted with {{?f}}")
            (edge 1 2) (edge 2 3)
            (holds 1)  (holds 3)  (holds (src Y))
            {activators}
            "#
        )
    };
    // The controls: one plan each, no layout to disagree with. The nested-fact
    // activator derives the two with `?f` = `(src Y)`, the name one the two
    // with `?f` = 1. Until M1f S1f.5.18 the nested fact bound nothing, `?f`
    // was `(holds ?f)`'s to bind, and the nested-fact activator derived every
    // `(noted ?a ?f)`, which this control stated (W33).
    let nested = BTreeSet::from([
        "(noted 1 (src Y))".to_string(),
        "(noted 2 (src Y))".to_string(),
    ]);
    let name = BTreeSet::from(["(noted 1 1)".to_string(), "(noted 2 1)".to_string()]);
    assert_eq!(
        saturate(&program("(note edge (src Y))")).derived(),
        nested,
        "the nested-fact activator alone derives the two with ?f = (src Y)"
    );
    assert_eq!(
        saturate(&program("(note edge 1)")).derived(),
        name,
        "the name activator alone derives the two with ?f = 1"
    );
    let whole: BTreeSet<String> = nested.union(&name).cloned().collect();

    // Both, in both orders: a rule set with one more activator concludes the
    // union, and the order two facts are written in changes nothing.
    for order in [
        "(note edge 1) (note edge (src Y))",
        "(note edge (src Y)) (note edge 1)",
    ] {
        assert_eq!(
            saturate(&program(order)).derived(),
            whole,
            "{order}: the two plans' bindings are compared within their own \
             spaces, so neither suppresses the other and nothing is lost"
        );
    }
}

/// **Two activators of one rule are two plans, in either order and in either
/// build** — M1e2 S1e2.2.16's regression for W16, restated at M1f S1f.5.5's
/// swap.
///
/// Until the swap the loader held `1` and `"1"` as two values, an `Int` and a
/// `Sym` whose text is `1`, so the program had two `holds` facts and each
/// activator alone derived its own two `noted` facts; until S1e2.2.16 the
/// compile key rendered a non-symbol argument and interned its text, so the two
/// activators keyed one plan and the second's conclusions were lost (a debug
/// build aborted). Since the swap neither value can be written: a run of
/// digits is a name (S1f.5.4's ruling) and a string is refused at the parse
/// (S1f.5.2's), so the program W16's witnesses write is refused at its `"1"`s
/// (the user's ruling of 2026-11-08 on the review stop, item 2). What the key
/// must still keep apart is two names of digits, `1` and `01`, which it does by
/// value: each activator derives its own two conclusions, and both derive the
/// four, in either order. The corpus witnesses are
/// `examples/ein-bugs/activator-text-{int-and-string,string-then-int}.ein`.
#[test]
fn an_int_and_a_string_with_one_text_are_two_activators() {
    let program = |holds: &str, activators: &str| {
        format!(
            r#"
            (relation edge  Node Node :why "there is an edge from {{?1}} to {{?2}}")
            (relation holds Node :why "{{?1}} holds")
            (relation noted Node Node :why "{{?1}} is noted with {{?2}}")
            (rule note (?R ?f)
              :match  (and (?R ?a ?b) (holds ?f))
              :assert (noted ?a ?f)
              :why    "{{?R}} holds of {{?a}} and {{?b}} and {{?f}} holds, so {{?a}} is noted with {{?f}}")
            (edge 1 2) (edge 2 3)
            (holds 1)  {holds}
            {activators}
            "#
        )
    };
    let refused = try_saturate(&program(
        r#"(holds "1")"#,
        r#"(note edge 1) (note edge "1")"#,
    ))
    .err()
    .expect("a string as a value is refused at the parse");
    assert!(
        refused.contains(r#"<fixture>:10:31: unexpected `"1"`: a string"#)
            && refused.contains(r#"<fixture>:11:38: unexpected `"1"`: a string"#),
        "{refused}"
    );
    // The controls: one activator, one plan, two conclusions each.
    let one = saturate(&program("(holds 01)", "(note edge 1)"));
    let zero_one = saturate(&program("(holds 01)", "(note edge 01)"));
    let (one, zero_one) = (one.derived_values(), zero_one.derived_values());
    assert_eq!(one.len(), 2, "the activator `1` derives two: {one:?}");
    assert_eq!(
        zero_one.len(),
        2,
        "the activator `01` derives two: {zero_one:?}"
    );
    assert!(
        one.is_disjoint(&zero_one),
        "two names: {one:?} {zero_one:?}"
    );
    let union: BTreeSet<String> = one.union(&zero_one).cloned().collect();
    // Both, in both orders: one more activator cannot conclude less, and the
    // order two facts are written in changes nothing.
    for order in [
        "(note edge 1) (note edge 01)",
        "(note edge 01) (note edge 1)",
    ] {
        let run = saturate(&program("(holds 01)", order));
        assert_eq!(
            run.derived_values(),
            union,
            "{order}: the two activators key two plans, so neither one's \
             conclusions are lost"
        );
        assert_eq!(run.productive().len(), 4, "{order}: four applications");
    }
}

// ── a string in a rule — M1e2 S1e2.2.18 ────────────────────────────────────

/// **A string in a rule is refused at the parse, as in a fact** — M1e2
/// S1e2.2.18's regression for W18, restated at M1f S1f.5.5's swap.
///
/// From S1e2.2.18 to the swap a string in a rule meant what it meant in a
/// fact, the symbol it names: it matched the fact it named, and an `:assert`
/// stored it (until S1e2.2.18 a string in a `:match` was an opaque slot that
/// matched nothing, so `s` never fired, and in an `:assert` the run stopped).
/// Since the swap a string is natural-language text, written only after
/// `:why`, `:source` or `:goal-text` (S1f.5.2's ruling), so every string
/// below, in a fact, a `:match` and an `:assert`, top-level and nested, is
/// refused at the parse, each with S1f.5.2's note (the review stop's item 3),
/// and the name written bare is what to write: the program so written
/// derives what the string program derived. The corpus witnesses are
/// `examples/ein-bugs/wrong-answers/str-in-*.ein`.
#[test]
fn a_string_in_a_rule_means_what_it_means_in_a_fact() {
    let program = |q: &str| {
        let s = |text: &str| format!("{q}{text}{q}");
        format!(
            r#"
            (relation holds Node :why "{{?1}} holds") (relation hit Node :why "hit holds of {{?1}}") (relation tag Node Node :why "{{?1}} is tagged {{?2}}")
            (holds 1) (holds a) (holds {b})
            (rule s () :match (holds {one}) :assert (hit s) :why "1 holds, so hit holds of s")
            (rule t () :match (holds b)   :assert (hit t) :why "b holds, so hit holds of t")
            (rule l () :match (holds ?x) :assert (and (tag ?x {lbl}) (tag ?x (hit {lbl}))) :why "{{?x}} holds, so {{?x}} is tagged lbl and (hit lbl)")
            "#,
            b = s("b"),
            one = s("1"),
            lbl = s("lbl"),
        )
    };
    let refused = try_saturate(&program("\""))
        .err()
        .expect("a string is refused at the parse");
    assert_eq!(
        refused
            .matches(" is a string, and a string is natural-language text")
            .count(),
        4,
        "one error and its note a string:\n{refused}"
    );
    let names = saturate(&program(""));
    assert_eq!(
        names
            .derived()
            .into_iter()
            .filter(|f| f.starts_with("(hit"))
            .collect::<Vec<_>>(),
        ["(hit s)", "(hit t)"],
        "`s` fires as `t` does"
    );
    assert_eq!(
        names.derived_values().len(),
        8,
        "two hits, and two conclusions of `l` for each of the three values: {:?}",
        names.derived_values()
    );
}

/// M1e2 S1e2.2.20 (W20, Q-M1e2.13 (a)) — **a wildcard in a pattern matched
/// any value and bound nothing**, in either build (until S1e2.2.20 the rule
/// compiler lowered `_` as an opaque slot, so `w` never fired on `(holds a)`).
/// **Since M1f S1f.5.5's swap a bare `_` is refused at the parse, in every
/// position** (S1f.5.1's ruling), each with its note, and a variable used
/// once is what to write: it concludes what the wildcard did, one application
/// per match where two matches under `_` alone were one. The corpus witnesses
/// are `examples/ein-bugs/wrong-answers/wild-in-*.ein`.
#[test]
fn a_wildcard_in_a_pattern_matches_any_value_and_binds_nothing() {
    let refused = try_saturate(
        "(relation holds T) (relation pair T T) (relation hit T)
         (holds a) (pair a b) (pair c d)
         (rule w () :match (holds _) :assert (hit w))
         (rule v () :match (pair _ _) :assert (hit v))",
    )
    .err()
    .expect("a bare `_` is refused at the parse");
    assert!(
        refused.contains("<fixture>:3:35: unexpected `_`: _ (reserved)")
            && refused.contains("<fixture>:4:34: unexpected `_`: _ (reserved)")
            && refused.contains(
                "note: _ is reserved and names nothing: where any value will do, write a \
                 variable used once"
            ),
        "{refused}"
    );
    let fresh = saturate(
        "(relation holds T :why \"{?1} holds\") (relation pair T T :why \"{?1} and {?2} are a pair\") (relation hit T :why \"hit holds of {?1}\")
         (holds a) (pair a b) (pair c d)
         (rule w () :match (holds ?x) :assert (hit w) :why \"{?x} holds, so hit holds of w\")
         (rule v () :match (pair ?x ?y) :assert (hit v) :why \"{?x} and {?y} are a pair, so hit holds of v\")",
    );
    assert_eq!(
        fresh.derived(),
        BTreeSet::from(["(hit v)", "(hit w)"].map(String::from)),
        "`w` and `v` fire"
    );
    assert_eq!(
        fresh
            .firings
            .iter()
            .filter(|f| fresh.rule_of(f) == "v")
            .count(),
        2,
        "a fresh variable makes one application per match"
    );
}

/// **An empty conclusion and a conclusion a program names `@empty` are two
/// facts** (M1f S1f.5.5's T1f.5.5.8), through the generated parser, which reads
/// `@empty` as a name (S1f.5.4's ruling) and `()` only as a whole `:match` or
/// `:assert`.
///
/// `:assert ()` asserts nothing since M1f S1f.5.9's T1f.5.9.2 (2026-11-09,
/// W34): `empty-form` matches and derives nothing, and is no firing. Until
/// then it stored a fact under the parser's synthetic head for `()`, whose
/// name joined H; until T1f.5.5.8 that head was `@empty`, so `at-empty`'s
/// `(@empty)` was the fact `empty-form` had already stored, and its firing
/// re-derived it. `at-empty` stores its own fact either way. (The rule was
/// named `written` until M1f S1f.6.38 reserved the name.)
#[test]
fn an_empty_conclusion_and_one_named_at_empty_are_two_facts() {
    let text = "(relation q Node :why \"q holds of {?1}\")\n(q a)\n\
         (rule empty-form () :match (q ?x) :assert () :why \"q holds of {?x}, and the empty conclusion asserts nothing\")\n\
         (rule at-empty () :match (q ?x) :assert (@empty) :why \"q holds of {?x}, so @empty holds\")\n";
    let mut ast = Ast::new();
    let forms =
        ein_ir::grammar::parse(&mut ast, text, Some("<fixture>")).expect("the generated parser");
    let run = saturate_forms(ast, &forms).expect("it saturates");
    let by = |rule: &str| -> Vec<ein_core::FactId> {
        run.productive()
            .into_iter()
            .filter(|f| run.rule_of(f) == rule)
            .flat_map(|f| f.derived.iter().copied())
            .collect()
    };
    let written = by("at-empty");
    assert_eq!(
        written.len(),
        1,
        "`at-empty` re-derived a fact it did not store"
    );
    let (rel, args) = run.terms.facts.get(written[0]);
    assert_eq!((run.terms.sym(rel), args.len()), ("@empty", 0));
    assert!(run.kb.contains(written[0]));
    assert!(
        !by("empty-form").contains(&written[0]),
        "`:assert ()` and `:assert (@empty)` stored one fact"
    );
    assert!(
        run.firings.iter().all(|f| run.rule_of(f) != "empty-form"),
        "`:assert ()` concludes nothing, so it is no firing"
    );
    assert_eq!(
        run.facts().iter().filter(|f| f.contains("#empty")).count(),
        0,
        "`:assert ()` stored a fact under the synthetic head"
    );
}

/// **An empty `:match` fires once, or once per activator fact** — M1f
/// S1f.5.9, on the user's ruling of 2026-11-07 on P1f.5's F12 (*"1 fire; 2
/// same"*). `()` is the empty conjunction, which is true, so a rule whose
/// `:match` is `()` fires once at root with nothing bound, and a rule with
/// parameters once per activator, its parameters alone bound. Until S1f.5.9
/// `()` was a premise over the parser's synthetic `#empty`, which no fact
/// holds, so neither fired, in silence (W31).
#[test]
fn an_empty_match_fires_once_or_once_per_activator() {
    let run = saturate(
        "(relation q Node :why \"q holds of {?1}\") (relation hit Node :why \"hit holds of {?1}\")\n(q a)\n\
         (rule once () :match () :assert (q b) :why \"with nothing to match, q holds of b\")\n\
         (rule each (?x) :match () :assert (hit ?x) :why \"each names {?x}, so hit holds of {?x}\")\n(each a) (each c)\n",
    );
    let fired = |rule: &str| {
        run.productive()
            .into_iter()
            .filter(|f| run.rule_of(f) == rule)
            .count()
    };
    assert_eq!(fired("once"), 1, "the empty conjunction holds once");
    assert_eq!(fired("each"), 2, "once per activator fact");
    assert_eq!(
        run.derived(),
        BTreeSet::from([
            "(q b)".to_string(),
            "(hit a)".to_string(),
            "(hit c)".to_string()
        ])
    );
    let each = run
        .productive()
        .into_iter()
        .find(|f| run.rule_of(f) == "each")
        .expect("fired");
    assert_eq!(
        run.bindings_of(each).len(),
        1,
        "only the parameter is bound: {:?}",
        run.bindings_of(each)
    );
}

// ── `eq` and `neq` are relations (M1f S1f.5.7) ─────────────────────────────
//
// The user's ruling of 2026-11-07 on Q-M1e2.26: `eq` and `neq` are the
// relations of equality and of non-equality, computed and never stored, and a
// premise follows the reading a goal does.

/// **A premise means the same in any order of its conjuncts.** A guard written
/// before the relation premise that binds its variable waits for it, so each
/// pair of rules below derives one set of facts. Until S1f.5.7 the loader
/// refused the first of each pair (W24, Q-M1e2.18 (ii)(a)), and before M1e2
/// S1e2.2.24 the guard compared the unbound variable as a null, so `eq-first`
/// never fired and `neq-first` fired for `a` too.
#[test]
fn a_premise_means_the_same_in_any_order_of_its_conjuncts() {
    let run = saturate(
        r#"
        (relation p T :why "p holds of {?1}") (relation e1 T :why "e1 holds of {?1}") (relation e2 T :why "e2 holds of {?1}") (relation n1 T :why "n1 holds of {?1}") (relation n2 T :why "n2 holds of {?1}")
        (p a) (p b) (p (f a))
        (rule eq-first () :match (and (eq ?x a) (p ?x)) :assert (e1 ?x) :why "{?x} is a and p holds of it, so e1 holds of {?x}")
        (rule eq-last () :match (and (p ?x) (eq ?x a)) :assert (e2 ?x) :why "p holds of {?x}, which is a, so e2 holds of {?x}")
        (rule neq-first () :match (and (neq ?x a) (p ?x)) :assert (n1 ?x) :why "{?x} is not a and p holds of it, so n1 holds of {?x}")
        (rule neq-last () :match (and (p ?x) (neq ?x a)) :assert (n2 ?x) :why "p holds of {?x}, which is not a, so n2 holds of {?x}")
        "#,
    );
    let of = |rel: &str| -> BTreeSet<String> {
        run.derived()
            .iter()
            .filter_map(|f| f.strip_prefix(&format!("({rel} ")))
            .map(|rest| rest[..rest.len() - 1].to_string())
            .collect()
    };
    assert_eq!(of("e1"), BTreeSet::from(["a".to_string()]));
    assert_eq!(of("e1"), of("e2"), "eq, either order");
    assert_eq!(
        of("n1"),
        BTreeSet::from(["(f a)".to_string(), "b".to_string()])
    );
    assert_eq!(of("n1"), of("n2"), "neq, either order");
}

/// **A guard's variable nothing binds ranges over the program's names**: every
/// constant a fact writes in an argument, at any depth, and every constant of
/// a rule's `:match` or `:assert` (Q-M1e2.9 (a)), `(relation …)`'s names among
/// them. `eq` binds it to the other side, and `neq` to every name but it.
#[test]
fn a_guards_free_variable_ranges_over_the_programs_names() {
    let run = saturate(
        r#"
        (relation q T :why "q holds of {?1}") (relation same T T :why "{?1} and {?2} are the same") (relation apart T T :why "{?1} and {?2} are apart")
        (q b) (q (box c))
        (rule pair-same () :match (and (q ?y) (eq ?x ?y)) :assert (same ?x ?y) :why "q holds of {?y} and {?x} equals it, so they are the same")
        (rule pair-apart () :match (and (q ?y) (neq ?x ?y)) :assert (apart ?x ?y) :why "q holds of {?y} and {?x} is any other name, so they are apart")
        "#,
    );
    let derived = run.derived();
    let of = |rel: &str| -> BTreeSet<String> {
        derived
            .iter()
            .filter(|f| f.starts_with(&format!("({rel} ")))
            .cloned()
            .collect()
    };
    // `eq` binds `?x` to `?y`'s value, a name or a nested fact.
    assert_eq!(
        of("same"),
        BTreeSet::from([
            "(same (box c) (box c))".to_string(),
            "(same b b)".to_string()
        ])
    );
    // `neq` ranges over the names: `T`, `apart`, `b`, `c`, `q` and `same`
    // (each `(relation …)` is a stored fact), not the nested fact `(box c)`,
    // and not a rule's name.
    let names = ["T", "apart", "b", "c", "q", "same"];
    let mut want: BTreeSet<String> = BTreeSet::new();
    for x in names {
        if x != "b" {
            want.insert(format!("(apart {x} b)"));
        }
        want.insert(format!("(apart {x} (box c))"));
    }
    assert_eq!(of("apart"), want);
}

/// **A guard that binds its variable binds it where it is written**, as a
/// relation premise in its place would (Q-M1f.34, taken by default on the
/// user's instruction of 2026-11-09): an `(absent …)` after it reads the
/// variable, and one before it does not, which is the order rule an `(absent
/// …)` already has (S1.21.8). Here `(neq ?x a)` before `(absent (q ?x))` is
/// *every name but `a` with no `q`*, and after it *every name but `a`, if there
/// is no `q` at all*, which there is.
#[test]
fn a_binding_guard_binds_where_it_is_written() {
    let run = saturate(
        r#"
        (relation p T :why "p holds of {?1}") (relation q T :why "q holds of {?1}") (relation first T :why "first holds of {?1}") (relation last T :why "last holds of {?1}")
        (p a) (p b) (q b)
        (rule guard-first () :match (and (neq ?x a) (absent (q ?x))) :assert (first ?x) :why "{?x} is not a and q does not hold of it, so first holds of {?x}")
        (rule guard-last () :match (and (absent (q ?x)) (neq ?x a)) :assert (last ?x) :why "q holds of nothing and {?x} is not a, so last holds of {?x}")
        "#,
    );
    let derived = run.derived();
    let first: BTreeSet<&str> = derived
        .iter()
        .filter_map(|f| f.strip_prefix("(first "))
        .map(|r| r.trim_end_matches(')'))
        .collect();
    assert_eq!(
        first,
        BTreeSet::from(["T", "first", "last", "p", "q"]),
        "every name of the program but `a`, and `b`, which has a `q`"
    );
    assert!(
        !derived.iter().any(|f| f.starts_with("(last ")),
        "`(absent (q ?x))` with nothing bound asks whether there is no `q` at all: {derived:?}"
    );
}

/// **`(not (eq ?x ?y))` is `(neq ?x ?y)`, and a relation its activator binds
/// to `eq` is `eq`** (Q-M1f.31, taken by default on the user's instruction of
/// 2026-11-09; Q-M1e2.26). Until S1f.5.7 each was a pattern over stored
/// facts, which no state holds, and neither rule fired (W51, W52).
#[test]
fn not_eq_and_a_relation_bound_to_eq_are_the_guards() {
    let run = saturate(
        r#"
        (relation p T :why "p holds of {?1}") (relation apart T T :why "{?1} and {?2} are apart") (relation in T T T :why "{?1} holds of {?2} and {?3}")
        (p a) (p b)
        (rule not-eq () :match (and (p ?x) (p ?y) (not (eq ?x ?y))) :assert (apart ?x ?y) :why "p holds of {?x} and of {?y}, which are not equal, so they are apart")
        (rule within (?R) :match (and (p ?x) (p ?y) (?R ?x ?y)) :assert (in ?R ?x ?y) :why "p holds of {?x} and of {?y}, and {?R} holds of them, so in records it")
        (within eq)
        "#,
    );
    let derived = run.derived();
    assert!(
        derived.contains("(apart a b)") && derived.contains("(apart b a)"),
        "{derived:?}"
    );
    assert!(
        !derived
            .iter()
            .any(|f| f == "(apart a a)" || f == "(apart b b)")
    );
    let within: BTreeSet<&String> = derived.iter().filter(|f| f.starts_with("(in ")).collect();
    assert_eq!(
        within,
        BTreeSet::from([&"(in eq a a)".to_string(), &"(in eq b b)".to_string()])
    );
}

/// **A rule's `eq` or `neq` conclusion is checked, never stored** (the user's
/// G-21 of 2026-11-07, M1f S1f.5.7): one that holds adds nothing, and one that
/// does not is `(false)`, derived by the rule from the premises it matched.
/// Until S1f.5.7 each was stored as an ordinary fact (W30), so a false one sat
/// in the KB while a guard computed the opposite.
#[test]
fn an_eq_or_neq_conclusion_is_checked_never_stored() {
    let holds = saturate(
        r#"
        (relation p T :why "p holds of {?1}") (relation hit T :why "hit holds of {?1}")
        (p a) (p b)
        (rule apart () :match (and (p ?x) (p ?y) (neq ?x ?y)) :assert (and (neq ?x ?y) (hit ?x)) :why "p holds of {?x} and of {?y}, which differ, so hit holds of {?x}")
        (rule same () :match (p ?x) :assert (eq ?x ?x) :why "p holds of {?x}, so {?x} equals itself")
        "#,
    );
    let facts = holds.facts();
    assert!(
        !facts
            .iter()
            .any(|f| f.starts_with("(eq ") || f.starts_with("(neq ")),
        "a true conclusion adds nothing: {facts:?}"
    );
    assert!(facts.contains("(hit a)") && facts.contains("(hit b)"));
    assert!(!facts.contains("(false)"));
    // A firing whose only conclusion holds derives nothing, and is no firing.
    assert!(
        !holds.firings.iter().any(|f| holds.rule_of(f) == "same"),
        "{:?}",
        holds
            .firings
            .iter()
            .map(|f| holds.rule_of(f))
            .collect::<Vec<_>>()
    );

    for (conclusion, premise) in [
        ("(neq ?x ?x)", "(p a)"),
        ("(eq ?x w)", "(p a)"),
        ("(not (eq ?x ?x))", "(p a)"),
    ] {
        let run = saturate(&format!(
            "(relation p T :why \"p holds of {{?1}}\")\n(p a)\n(rule r () :match (p ?x) :assert {conclusion} :why \"p holds of {{?x}}, so {conclusion} with ?x = {{?x}}, which cannot hold\")\n"
        ));
        let falses: Vec<&Firing> = run
            .productive()
            .into_iter()
            .filter(|f| run.conclusions_of(f) == ["(false)"])
            .collect();
        assert_eq!(falses.len(), 1, "{conclusion}: one firing derives (false)");
        assert_eq!(run.rule_of(falses[0]), "r");
        let premises: Vec<String> = falses[0]
            .premises
            .iter()
            .map(|&p| sexpr(&run.terms, p))
            .collect();
        assert_eq!(
            premises,
            [premise],
            "{conclusion}: its premises are the match's"
        );
        assert!(
            !run.facts().iter().any(|f| f.starts_with("(eq ")
                || f.starts_with("(neq ")
                || f.starts_with("(not (eq ")),
            "{conclusion}: nothing of eq is stored"
        );
    }
}

/// **A written fact of `eq`, or the negation of one, is a conclusion with no
/// premise** (Q-M1f.33 and Q-M1f.31, taken by default on the user's
/// instruction of 2026-11-09): a true one is not stored, and a false one is
/// `(false)`, written where it stands. Until S1f.5.7 each was stored (W50,
/// W51).
#[test]
fn a_written_eq_fact_is_checked_never_stored() {
    for (fact, dead) in [
        ("(eq a a)", false),
        ("(not (eq a b))", false),
        ("(eq a w)", true),
        ("(not (eq a a))", true),
    ] {
        let run = saturate(&format!(
            "(relation p T :why \"p holds of {{?1}}\")\n(p a)\n{fact}\n"
        ));
        let facts = run.facts();
        assert!(
            !facts
                .iter()
                .any(|f| f.starts_with("(eq ") || f.starts_with("(not (eq ")),
            "{fact}: not stored: {facts:?}"
        );
        assert_eq!(facts.contains("(false)"), dead, "{fact}: {facts:?}");
    }
}
