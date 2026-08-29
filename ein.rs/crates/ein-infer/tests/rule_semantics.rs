//! What a rule *means* when it fires — S1a.10.2 **T1a.10.2.2**.
//!
//! Replaces the semantics half of five files under `ein.py/tests/inference/`:
//! `test_forall.py`, `test_match.py`, `test_multi_assert.py`,
//! `test_predicates.py` and `test_rules.py`. The common subject is what one
//! rule application *is*: which bindings a premise admits (repeated variables,
//! nested patterns, stored `not`), which bindings a guard keeps (`eq` / `neq`,
//! over variables and over literals), how many facts one match concludes
//! (`:assert (and …)` × `:match (or …)`), what `forall` expands to, and when a
//! parametrised rule is allowed to run at all.
//!
//! **Every claim here is asserted as a derivation.** The Python originals
//! mostly could not be: they called `compile_rule` and `match.run` directly and
//! inspected plan opcodes (`isinstance(s, AbsentGuard)`, `plan.naf_guards`,
//! `outer.scope`) and raw binding dicts. Those are the *encoding* of a rule,
//! and the encoding is exactly what the port was free to change — ein.rs
//! resolves variables to registers and lifts `(absent …)` to a boundary guard,
//! so a translated opcode assertion would pin this build rather than the
//! language. What the language fixes is which facts a program derives, so each
//! test below saturates a fixture and reads the firings and the KB.

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
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, Some("<fixture>")).map_err(|e| e.to_string())?;
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).map_err(|e| e.to_string())?;
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
///
/// From `test_forall_empty_domain_is_vacuously_true`.
#[test]
fn a_forall_over_an_empty_domain_is_vacuously_true() {
    let tournament = |roster: &str| {
        format!(
            r#"
            (import std.macro :symbols (forall))
            (relation player T) (relation beats T T) (relation undefeated T)
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
/// else; and a rule that puts `?q` in its head is refused at fire time with a
/// message that names `?q` as unbound *while showing `?p` bound*. A leaked
/// binding would satisfy that head instead of failing.
///
/// From `test_forall_bound_does_not_escape`.
#[test]
fn the_forall_bound_variable_does_not_escape_the_guard() {
    let tournament = |head: &str| {
        format!(
            r#"
            (import std.macro :symbols (forall))
            (relation player T) (relation beats T T) (relation all-beaten T T)
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
        err.contains("unbound var ?q in :assert"),
        "the head's ?q should be reported unbound, got: {err}"
    );
    assert!(
        err.contains("'p': 'Alice'"),
        "the message should show the outer environment that *was* built, so \
         the reader can see ?p present and ?q missing, got: {err}"
    );
}

/// **A `forall` whose bound variable never appears in its guard still
/// compiles** — and this is a deliberate loss, not an oversight.
///
/// Before P1.8 S1.5.9 the desugaring lived in `compile.py` and refused such a
/// rule (`_var_in_ast`: "?bound must appear in the guard", because there is no
/// domain to enumerate). `forall` is now an ordinary `(macro …)`, and macro
/// expansion does no per-macro validation, so the rejection left with the
/// desugar. What is left is a rule that means something other than what its
/// author wrote, and the two cells below show it *runs*: with no `(whatever …)`
/// fact the expansion `(absent (and (player ?other) (absent (whatever ?v))))`
/// is false and nothing fires; add one and it fires. A test that only checked
/// "no error" would pass just as well against a rule silently dropped on the
/// floor, which is why both cells are here.
///
/// From `test_forall_bound_not_in_guard_no_longer_rejected`.
#[test]
fn a_forall_whose_bound_var_is_absent_from_its_guard_still_compiles() {
    let bad_rule = |extra: &str| {
        format!(
            r#"
            (import std.macro :symbols (forall))
            (relation player T) (relation whatever T) (relation oops T)
            (rule bad-rule ()
              :match  (forall ?v (player ?other) (whatever ?v))
              :assert (oops)
              :why    "?v is not bound by (player ?other)")
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
///
/// From `test_neq_guard_prunes_self_loops` (whose name is a leftover — the
/// rule it exercises carries no `neq`).
#[test]
fn a_repeated_variable_is_an_equality_constraint() {
    let run = saturate(
        r#"
        (relation r T T) (relation self-loop T)
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
///
/// From `test_not_premise_matches_stored_neg_fact` and
/// `test_not_premise_does_not_match_without_stored_neg`.
#[test]
fn not_in_a_premise_is_stored_negation_not_negation_as_failure() {
    let see_neg = |evidence: &str| {
        format!(
            r#"
            (relation r T T) (relation other T T) (relation saw-neg T T)
            (rule see-neg (?rl)
              :match  (and (?rl ?a ?b) (not (other ?a ?b)))
              :assert (saw-neg ?a ?b)
              :why    "a stored negative was seen")
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
/// From `test_nested_fact_pattern_unifies_against_relational_arg` (Q40), which
/// had to synthesise the nested `Fact` in Python; the surface language writes
/// it directly.
#[test]
fn a_nested_pattern_binds_inside_the_nested_fact() {
    let run = saturate(
        r#"
        (relation co-located T T) (relation hypothesis T) (relation caught T T)
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
///
/// From `test_generic_multi_assert_now_works`.
#[test]
fn a_parameterised_rule_may_multi_assert() {
    let run = saturate(
        r#"
        (rule place-and-exclude (?rel)
          :match  (and (?rel ?a ?x) (slot ?y) (neq ?x ?y))
          :assert (and (?rel ?a ?x) (not (?rel ?a ?y)))
          :why    "{?a} is {?x} via {?rel}, so not {?y}")
        (relation color-loc Color House) (relation slot T)
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
///
/// From `test_or_match_and_assert_one_rule`.
#[test]
fn every_disjunct_emits_every_template() {
    let run = saturate(
        r#"
        (relation a T) (relation b T) (relation p T) (relation q T)
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
///
/// From `test_eq_resolves_vars` and `test_neq_resolves_vars`, which called the
/// registry's callable with a hand-built binding dict.
#[test]
fn eq_admits_exactly_the_bindings_neq_rejects() {
    let run = saturate(
        r#"
        (relation p T) (relation q T) (relation same T T) (relation diff T T)
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
/// silently admit nothing, or everything. The fixture pins both leaf kinds an
/// `Atom` and an `Int` land as — a name and a number — and each rule is given a
/// non-matching companion fact so that "fires for the right one" and "fires for
/// everything" are distinguishable.
///
/// From `test_eq_resolves_literal_atom_and_int`.
#[test]
fn a_guard_argument_may_be_a_literal() {
    let run = saturate(
        r#"
        (relation color T) (relation is-red T)
        (relation n T) (relation is-five T)
        (rule red ()
          :match  (and (color ?x) (eq ?x Red))
          :assert (is-red ?x)
          :why    "{?x} is the atom Red")
        (rule five ()
          :match  (and (n ?k) (eq ?k 5))
          :assert (is-five ?k)
          :why    "{?k} is the integer 5")
        (color Red) (color Green)
        (n 5) (n 6)
        "#,
    );
    assert_eq!(
        run.derived(),
        BTreeSet::from(["(is-red Red)".to_string(), "(is-five 5)".to_string()]),
        "an atom literal compares against the bound name and an int literal \
         against the bound number — neither is resolved as a variable, and \
         neither matches the companion fact"
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
///
/// From `test_symmetric_negative_no_activator`,
/// `test_square_bwd_negative_no_activator` (the same claim) and
/// `test_implies_negative_wrong_relation`.
#[test]
fn a_parametrised_rule_with_no_activator_is_dormant() {
    let program = |activator: &str| {
        format!(
            r#"
            (rule implies (?p ?q)
              :match  (?p ?a ?b)
              :assert (?q ?a ?b)
              :why    "{{?p}} implies {{?q}}"
              :priority 100)
            (relation co-located T T) (relation next-to T T) (relation right-of T T)
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

// ── the activator's identity — Q-M1a.8 ─────────────────────────────────────
//
// `BindingKey` is `(rule, activator, values)` and there are **three** keys
// over one activator, not two:
//
// | key | what it keeps of the activator |
// |---|---|
// | [`PlanKey`] — the compile cache | every argument, stringified |
// | `BindingKey.activator` — an interned `plan.activator_args` | the **symbol** arguments |
// | `BindingKey.values` — the register file | every argument that **binds a parameter**: symbols *and* ints |
//
// `defined_behaviour.md` §3.2 read the middle row alone and concluded that two
// activators differing only in an `int` argument share an identity. They do
// not: `bind_activator` seeds a register for every argument that is not a
// nested `Fact` (`compile.rs`, `if a.as_fact().is_some() { continue; }`), so
// the int is in `values`. The arguments that reach **neither** half are
// exactly the nested `Fact`s. The three tests below are that correction, and
// they are three because the third shape is the one that costs a derivation.

/// **Two activators differing only in an `int` argument both fire** — which is
/// what `defined_behaviour.md` §3.2 said they could not do, and the probe that
/// refutes it (M1e [S1e.1.4](../../../../plans/m1e_review_processing/p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md),
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
        (relation edge   Node Node)
        (relation tagged Node Label)
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

/// **Two activators differing only in a nested `Fact` argument share one
/// binding key** — the collision §3.2 was reaching for, at the argument kind
/// that actually has it.
///
/// A nested `Fact` binds no parameter, so it reaches neither half of the key.
/// The same fixture is run four ways and only the *kind* of the second
/// argument varies:
///
/// | second argument | plans | firings |
/// |---|---:|---:|
/// | one nested `Fact` | 1 | 1 |
/// | two nested `Fact`s | 2 | **1** |
/// | two symbols | 2 | 2 |
/// | two ints | 2 | 2 |
///
/// **And nothing is lost by it.** `activator` reaches the compiler at exactly
/// one site — `Compiler::run` passes it to `bind_activator` and nowhere else —
/// and `bind_activator` skips a `Fact` argument outright, so the two plans are
/// equal in every field of [`Plan`]. The suppressed application is a
/// *duplicate*: it would have derived what the survivor derived, which is why
/// the derived set is the same in all four cells. The shape that does cost a
/// derivation is the next test.
#[test]
fn activators_differing_only_by_a_nested_fact_argument_share_one_binding_key() {
    let program = |activators: &str| {
        format!(
            r#"
            (relation edge  Node Node)
            (relation noted Node)
            (rule note (?R ?f)
              :match  (?R ?a ?b)
              :assert (noted ?a)
              :why    "note")
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
            1,
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
             included — a second plan whose binding key equals the first's is \
             never enqueued at all"
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

/// **An `int` in the position another activator gives a nested `Fact` loses a
/// derivation, silently** — the real latent bug behind `Q-M1a.8`, and the one
/// `defined_behaviour.md` §3.2 now states.
///
/// Both activators drop their second argument from `plan.activator_args`, so
/// the two plans share a `(rule, activator)` binding-key space — but the int
/// seeds a register and the `Fact` does not, so they disagree on their
/// *register layout*: `?f` is register 1 in one plan and register 3 in the
/// other. `BindingKey` then compares `(?R ?f ?a ?b)` against `(?R ?a ?b ?f)`
/// position by position, and `(edge 1 2 3)` is a legitimate match of both.
/// Whichever fires first suppresses the other, and here the losing
/// application is the only one that would have derived `(noted 1 3)`.
///
/// **Two profiles, one claim.** `Engine::check_layout` asserts exactly this
/// invariant — and only under `debug_assertions`, which is why the test
/// expects the assertion in a `cargo test` build and the wrong answer in a
/// release one. The premise its doc comment rested on (*"a shape no rule
/// application has"*) is this seventeen-line program.
///
/// The fix is filed, not taken here:
/// [Q-M1e.16](../../../../plans/m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one).
#[test]
#[cfg_attr(
    debug_assertions,
    should_panic(expected = "disagree on their register layout")
)]
fn an_int_beside_a_nested_fact_in_one_position_loses_a_derivation() {
    let program = |activators: &str| {
        format!(
            r#"
            (relation edge  Node Node)
            (relation holds Node)
            (relation noted Node Node)
            (rule note (?R ?f)
              :match  (and (?R ?a ?b) (holds ?f))
              :assert (noted ?a ?f)
              :why    "note")
            (edge 1 2) (edge 2 3)
            (holds 1)  (holds 3)
            {activators}
            "#
        )
    };
    // The control runs in both profiles: one plan, no layout to disagree with,
    // and the four conclusions the rule has.
    let whole = BTreeSet::from([
        "(noted 1 1)".to_string(),
        "(noted 1 3)".to_string(),
        "(noted 2 1)".to_string(),
        "(noted 2 3)".to_string(),
    ]);
    assert_eq!(
        saturate(&program("(note edge (src Y))")).derived(),
        whole,
        "the nested-fact activator alone derives every (noted ?a ?f)"
    );

    // Debug: `check_layout` fires here and the test ends at this line.
    let both = saturate(&program("(note edge 1) (note edge (src Y))"));

    // Release: adding an activator *removed* a conclusion.
    assert!(
        !both.derived().contains("(noted 1 3)"),
        "the collision is what this test is about — if (noted 1 3) is back, \
         Q-M1e.16 was fixed and this test should be turned into its regression"
    );
    assert_eq!(
        both.derived(),
        &whole - &BTreeSet::from(["(noted 1 3)".to_string()]),
        "the int activator's own two conclusions are a subset of the nested \
         one's, so the union of the two programs is `whole` — and running \
         them together derives one fact fewer than running either alone"
    );
}
