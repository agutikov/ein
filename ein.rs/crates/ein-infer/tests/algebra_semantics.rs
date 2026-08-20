//! `std.algebra` — the relation-algebra library as *behaviour* (T1a.10.2.2).
//!
//! Replaces three Python files, which die with `ein.py/`:
//!
//! - `ein.py/tests/inference/test_algebra.py` — the A7 seed: `converse`, the
//!   `imply` family, and the reflective `symmetric ⇄ (converse R R)` lemmas.
//! - `ein.py/tests/inference/test_converse_typecheck.py` — A10: the
//!   `converse-illtyped-{dom,ran}` signature check, `std.typing`'s one-knob
//!   `(type-hierarchy …)` driver, and `(reflexive R)`.
//! - `ein.py/tests/inference/test_relation_algebra.py` — A12: composition, the
//!   Boolean lattice (`meet` / `join` / `difference` / `complement` / `top` /
//!   `empty`), the identity, the single-relation property checks, and the
//!   Tarski/Maddux equational lemmas (Schröder B10, contravariance B7,
//!   converse-over-join B8).
//!
//! The common subject is that **every rule in `stdlib/algebra.ein` is generic**
//! — parametrised over the operand relations — so the library on its own
//! derives nothing at all. What makes a rule run is a puzzle-declared
//! *activator fact* naming the operands: `(converse right-of left-of)`,
//! `(compose R S T)`, `(empty R)`. Each test therefore carries its own
//! three-line program, written inline rather than checked into `examples/`, so
//! that the activator sits next to the claim it switches on; most of these
//! programs are one relation and two edges and would say less as corpus files
//! than they do here.
//!
//! What is asserted is the **fixpoint**, never a plan or a firing's internals:
//! the facts the KB holds once saturation is done, and — where the claim is
//! about running away rather than about a conclusion — how many firings it took
//! to get there. Where the Python original asked `X in derived`, the port often
//! asks for the *whole extent* of the derived relation instead: "meet is
//! intersection" is a statement about which pairs are absent as much as about
//! which are present, and a membership check cannot fail on an over-derivation.

use std::collections::BTreeSet;

use ein_core::{FactId, Terms};
use ein_infer::events::sexpr;
use ein_infer::{Events, SaturateError, Saturator, Session, SharedMemo};
use ein_ir::{Ast, parse};

/// One saturation, reduced to what a claim about `std.algebra` can be about.
struct Run {
    /// Every fact in the KB at the fixpoint, rendered `(rel arg …)`.
    facts: BTreeSet<String>,
    /// The conclusions of the non-redundant firings — ein.py's `_derived`.
    ///
    /// A subset of `facts`, and the sharper of the two when the claim is that a
    /// *rule* produced something rather than that the fact is present: a given
    /// fact is in `facts` and never in `derived`.
    derived: BTreeSet<String>,
    /// How many firings the fixpoint took. Only the termination tests read it.
    firings: usize,
}

impl Run {
    fn has(&self, fact: &str) -> bool {
        self.facts.contains(fact)
    }

    fn was_derived(&self, fact: &str) -> bool {
        self.derived.contains(fact)
    }

    /// Every fact whose head is `rel`, sorted — the relation's whole extent.
    fn extent(&self, rel: &str) -> Vec<String> {
        let head = format!("({rel} ");
        self.facts
            .iter()
            .filter(|f| f.starts_with(&head))
            .cloned()
            .collect()
    }

    /// Did anything assert ⊥? `(false)` is the "direct" contradiction shape,
    /// and it is what every check rule in `std.algebra` fires.
    fn is_false(&self) -> bool {
        self.has("(false)")
    }
}

/// The expected extent, sorted the way [`Run::extent`] returns one.
fn extent(facts: &[&str]) -> Vec<String> {
    let mut v: Vec<String> = facts.iter().map(|s| (*s).to_string()).collect();
    v.sort();
    v
}

/// `(import std.algebra :symbols (…))` — the flat, selective import every
/// consumer of the library uses.
fn algebra(symbols: &str) -> String {
    format!("(import std.algebra :symbols ({symbols}))\n")
}

/// `(import std.typing :symbols (…))`.
fn typing(symbols: &str) -> String {
    format!("(import std.typing :symbols ({symbols}))\n")
}

fn try_run(src: &str, max_steps: usize) -> Result<Run, SaturateError> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, src, Some("<algebra>")).expect("the program parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the program loads");
    let mut events = Events::off();
    let mut derived_ids: Vec<FactId> = Vec::new();
    let outcome = {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).expect("the program compiles");
        sat.saturate(&mut s, Some(max_steps), &mut |f| {
            if !f.redundant {
                derived_ids.extend(f.derived.iter().copied());
            }
        })
    };
    let firings = outcome?;
    Ok(Run {
        facts: kb.facts().map(|f| sexpr(&terms, f)).collect(),
        derived: derived_ids.iter().map(|&f| sexpr(&terms, f)).collect(),
        firings,
    })
}

/// Saturate to the fixpoint. The budget here is a runaway guard, not a claim —
/// the tests that *are* about the budget call [`try_run`] and read the error.
fn run(src: &str) -> Run {
    try_run(src, 4000).unwrap_or_else(|e| panic!("saturation did not converge: {e}"))
}

// ── the relative layer: converse and the imply family ──────────────

/// `converse-mirrors-each-edge` — `(converse r1 r2)` turns every `r1` edge into
/// the reversed `r2` edge.
///
/// The activator is what makes this run at all: `stdlib/algebra.ein` declares
/// `(rule converse (?R1 ?R2) …)` with the relation *heads* as parameters, so
/// until a fact names two relations there is no scan to do and the library is
/// inert. The mirror is one-way here — `r1` is read, `r2` is written — which is
/// what separates `converse` from the `symmetric` tag the lemmas below relate
/// it to.
#[test]
fn converse_mirrors_each_edge_into_the_paired_relation() {
    let r = run(
        &(algebra("converse")
            + "(relation r1 T T) (relation r2 T T)
               (converse r1 r2)
               (r1 A B :source \"(1)\")"),
    );
    assert!(r.was_derived("(r2 B A)"), "the mirror: {:?}", r.derived);
    assert_eq!(r.extent("r2"), extent(&["(r2 B A)"]), "and nothing else");
}

/// `imply1-lifts-a-one-arg-property-marker` — a 1-arg implication can produce a
/// *kernel* trigger, which is `imply1`'s headline use.
///
/// The interesting part is not that `(r1 a) ⇒ (r2 a)`; it is that `?R1` and
/// `?R2` may be bound to property markers rather than to edge relations, so
/// `(imply1 functional __closed__)` re-tags every functional relation as closed
/// without needing a new rule shape. `__closed__` is a dunder the engine itself
/// reads, so this is userspace reaching a kernel switch.
#[test]
fn imply1_lifts_a_property_marker_onto_a_kernel_trigger() {
    let r = run(
        &(algebra("imply1")
            + "(relation foo T T)
               (imply1 functional __closed__)
               (functional foo)"),
    );
    assert!(
        r.was_derived("(__closed__ foo)"),
        "the marker did not lift: {:?}",
        r.derived
    );
}

/// `the-two-arg-implications-copy-forward-and-swap` — `imply2-fwd` preserves
/// argument order, `imply2-reverse` inverts it.
///
/// Both live in one program because the pair is only meaningful as a contrast:
/// they have the same premise and differ in one register of the conclusion, so
/// a test of either alone would still pass if the two rule bodies had been
/// swapped. `imply2-reverse` is a deliberate alias of `converse` — identical
/// body, ergonomic name — so this also pins that the alias did not quietly
/// become the forward copy.
#[test]
fn the_two_arg_implications_copy_forward_and_swap() {
    let r = run(
        &(algebra("imply2-fwd imply2-reverse")
            + "(relation r1 T T) (relation r2 T T) (relation r3 T T)
               (imply2-fwd r1 r2)
               (imply2-reverse r1 r3)
               (r1 A B :source \"(1)\")"),
    );
    assert_eq!(r.extent("r2"), extent(&["(r2 A B)"]), "fwd copies");
    assert_eq!(r.extent("r3"), extent(&["(r3 B A)"]), "reverse swaps");
}

// ── the reflective lemmas ──────────────────────────────────────────

const LEMMAS: &str = "converse symmetric-is-self-converse self-converse-is-symmetric \
                      converse-pair-symmetric";

/// `symmetric-and-self-converse-are-interderivable-and-reflective` — the two
/// tags mean the same thing, and each *derived* tag then works as an activator.
///
/// This is the A9 reflective idiom, and it is why the claim needs a test rather
/// than a comment: a derived `(converse knows knows)` is an ordinary fact, and
/// what turns it into behaviour is that the saturator re-reads the activator
/// table on a later pass. So the observable is not the derived tag at all — it
/// is `(knows B A)`, the mirror the derived tag went on to produce. Both
/// directions are asserted because either rule could go missing without the
/// other noticing.
#[test]
fn symmetric_and_self_converse_are_interderivable_and_reflective() {
    let from_symmetric = run(
        &(algebra(LEMMAS)
            + "(relation knows T T)
               (symmetric knows)
               (knows A B :source \"(1)\")"),
    );
    assert!(
        from_symmetric.was_derived("(converse knows knows)"),
        "symmetric ⟹ self-converse: {:?}",
        from_symmetric.derived
    );
    assert!(
        from_symmetric.was_derived("(knows B A)"),
        "the derived tag never fired the mirror"
    );

    let from_converse = run(
        &(algebra(LEMMAS)
            + "(relation knows T T)
               (converse knows knows)
               (knows A B :source \"(1)\")"),
    );
    assert!(
        from_converse.was_derived("(symmetric knows)"),
        "self-converse ⟹ symmetric: {:?}",
        from_converse.derived
    );
    assert!(
        from_converse.was_derived("(knows B A)"),
        "the derived tag never fired the mirror"
    );
}

/// `converse-is-symmetric-on-its-pair` — `(converse r1 r2)` derives
/// `(converse r2 r1)`, and the derived pair mirrors in that direction too.
///
/// A puzzle declares one direction; the second is what makes `left-of` usable
/// as a premise after only `(converse right-of left-of)` was written. The
/// `(r2 C D)` edge is in the program purely so the *derived* pair has something
/// to bite on: without it the back-edge would be an unobservable fact, and the
/// test would pass on a `converse-pair-symmetric` whose output the activator
/// table never picked up.
#[test]
fn converse_is_symmetric_on_its_pair_and_the_back_edge_mirrors() {
    let r = run(
        &(algebra(LEMMAS)
            + "(relation r1 T T) (relation r2 T T)
               (converse r1 r2)
               (r1 A B :source \"(1)\") (r2 C D :source \"(2)\")"),
    );
    assert!(
        r.was_derived("(converse r2 r1)"),
        "the swapped pair: {:?}",
        r.derived
    );
    assert!(r.was_derived("(r2 B A)"), "r1 → r2, the declared direction");
    assert!(
        r.was_derived("(r1 D C)"),
        "r2 → r1, which only the derived pair can do"
    );
}

/// `the-algebra-lemma-loop-terminates` — the `symmetric ⇄ (converse R R) ⇄
/// converse-pair-symmetric` cycle converges.
///
/// Each of those rules derives an activator that switches the others back on,
/// so the loop is genuinely circular and termination is a property of the
/// saturator's dedup rather than of the rules: every back-derivation is a fact
/// that already exists, so it fires once and is redundant forever after. The
/// budget is the falsifier — a rule producing a *fresh* fact each time round
/// would exhaust it — and the count bound catches a dedup that still works but
/// has stopped being cheap.
#[test]
fn the_algebra_lemma_loop_terminates() {
    let src = algebra(LEMMAS)
        + "(relation knows T T)
           (symmetric knows)
           (knows A B :source \"(1)\")";
    let r = try_run(&src, 5000).unwrap_or_else(|e| panic!("the lemma loop ran away: {e}"));
    assert!(
        r.firings < 0,
        "converged, but in {} firings — the loop is no longer cheap",
        r.firings
    );
}

// ── the converse type-check ────────────────────────────────────────

const TYPECHECK: &str = "converse-illtyped-dom converse-illtyped-ran";

/// `converse-with-incompatible-signatures-derives-false` — a reverse-
/// incompatible pairing is ⊥.
///
/// The check reads the signatures straight off the `(relation …)` declarations,
/// which is the design point: a declaration is an ordinary matchable fact, so
/// type-checking a generic rule needs no compiler support at all. `house-color
/// : (House Color)` cannot be the converse of `pet-dog : (Person Pet)`, because
/// `Color` is neither `Person` nor a subtype of it.
#[test]
fn converse_with_incompatible_signatures_derives_false() {
    let r = run(
        &(algebra(TYPECHECK)
            + "(relation house-color House Color)
               (relation pet-dog Person Pet)
               (converse house-color pet-dog)
               (converse-illtyped-dom house-color pet-dog is-a*)
               (converse-illtyped-ran house-color pet-dog is-a*)"),
    );
    assert!(r.is_false(), "the ill-typed pairing was accepted");
}

/// `converse-with-exactly-reversed-signatures-is-silent` — identical types pass
/// with no hierarchy fact and no reflexive closure.
///
/// `right-of` and `left-of` are both `(House House)`, so `range(R1) =
/// domain(R2)` holds by *equality* rather than by subtyping. That case is
/// absorbed by the rules' `(neq …)` guard, and the reason it matters is zebra2:
/// its `is-a*` is transitive but irreflexive, so a pure `(absent (is-a* X X))`
/// reading would reject every exact-type converse in the corpus. This is the
/// test that goes red if the guard is ever dropped in favour of that reading.
#[test]
fn converse_with_exactly_reversed_signatures_is_silent() {
    let r = run(
        &(algebra(TYPECHECK)
            + "(relation right-of House House)
               (relation left-of House House)
               (converse right-of left-of)
               (converse-illtyped-dom right-of left-of is-a*)
               (converse-illtyped-ran right-of left-of is-a*)"),
    );
    assert!(!r.is_false(), "an exact reverse was rejected");
}

/// `the-hierarchy-relation-is-genuinely-consulted` — the same converse pair is
/// accepted with `(is-a* Pet Animal)` stated and rejected without it.
///
/// One program would not prove this. A check that ignored `?isR*` and merely
/// compared the two signatures would be silent on both halves; one that always
/// fired would reject both. Only the *pair* isolates the hierarchy lookup, so
/// both halves live in one test — they are one claim, and separating them would
/// let half of it pass alone.
#[test]
fn the_hierarchy_relation_is_genuinely_consulted() {
    let program = |hierarchy: &str| {
        algebra(TYPECHECK)
            + "(relation owns Person Pet)
               (relation owned-by Animal Person)
               (converse owns owned-by)
               (converse-illtyped-dom owns owned-by is-a*)
               (converse-illtyped-ran owns owned-by is-a*)
            "
            + hierarchy
    };
    assert!(
        !run(&program("(is-a* Pet Animal :source \"(sub)\")")).is_false(),
        "Pet <: Animal is stated, so range(owns) <: domain(owned-by) holds"
    );
    assert!(
        run(&program("")).is_false(),
        "without the subtype edge Pet is neither = nor <: Animal"
    );
}

/// `the-type-hierarchy-knob-derives-the-per-pair-activators` — one
/// `(type-hierarchy is-a*)` type-checks every converse pair in the program.
///
/// The reflective idiom again, used for ergonomics rather than for a lemma:
/// `std.typing` derives the `(converse-illtyped-dom R1 R2 is-a*)` facts that
/// the two tests above wrote by hand, and those derived facts are then read as
/// activators on a later pass. So the derived activator is asserted as well as
/// the ⊥ — the verdict alone would also appear if the check had been wired up
/// some other way, and it is the derivation that is the claim. The well-typed
/// pair is in the same test because "the knob fires" is only interesting
/// alongside "the knob does not fire indiscriminately".
#[test]
fn the_type_hierarchy_knob_derives_the_per_pair_activators() {
    let bad = run(
        &(algebra(TYPECHECK)
            + &typing("type-hierarchy-converse")
            + "(relation house-color House Color)
               (relation pet-dog Person Pet)
               (type-hierarchy is-a*)
               (converse house-color pet-dog)"),
    );
    assert!(
        bad.was_derived("(converse-illtyped-dom house-color pet-dog is-a*)"),
        "the knob derived no activator: {:?}",
        bad.derived
    );
    assert!(bad.is_false(), "the derived activator never fired the check");

    let good = run(
        &(algebra(TYPECHECK)
            + &typing("type-hierarchy-converse")
            + "(relation right-of House House)
               (relation left-of House House)
               (type-hierarchy is-a*)
               (converse right-of left-of)"),
    );
    assert!(!good.is_false(), "a well-typed pair was rejected");
}

/// `reflexive-closes-both-argument-positions` — every node the relation touches
/// gets a self-loop, from *either* argument position.
///
/// One edge `(is-a* House-1 House)` has to produce two loops, and the two come
/// from different rules — `reflexive-dom` reads the left argument,
/// `reflexive-cod` the right — fanned out from a single `(reflexive is-a*)`
/// declaration. Asserting the whole extent is what makes the test able to fail:
/// checking only `(is-a* House House)` would pass on a closure that had lost
/// the domain side entirely.
#[test]
fn reflexive_closes_both_argument_positions() {
    let r = run(
        &(typing("derive-reflexive reflexive-dom reflexive-cod")
            + "(relation is-a* T T)
               (reflexive is-a*)
               (is-a* House-1 House :source \"(e)\")"),
    );
    assert_eq!(
        r.extent("is-a*"),
        extent(&[
            "(is-a* House-1 House)",
            "(is-a* House House)",
            "(is-a* House-1 House-1)",
        ]),
        "both self-loops and no more"
    );
}

// ── the relative (composition) layer ───────────────────────────────

/// `compose-chains-two-relations` — `(compose R S T)` is the relative product.
///
/// The shared `?b` is what makes it a join rather than two independent scans,
/// so the extent is asserted whole: a rule that had lost the shared variable
/// would still derive `(two-right A C)` here, and would derive
/// `(two-right A B)` and the rest of the cross product alongside it.
#[test]
fn compose_chains_two_relations() {
    let r = run(
        &(algebra("compose")
            + "(compose right-of right-of two-right)
               (right-of A B :source \"(1)\")
               (right-of B C :source \"(2)\")"),
    );
    assert_eq!(
        r.extent("two-right"),
        extent(&["(two-right A C)"]),
        "exactly the composite, not the cross product"
    );
}

/// `compose-into-self-is-transitive-closure` — `(compose R R R)` on a chain is
/// exactly the transitive closure, and it reaches a fixpoint.
///
/// Composing a relation into itself feeds the rule its own conclusions, so this
/// is the first program here where termination is not obvious: every derived
/// edge is a new premise. It converges because the closure of a finite acyclic
/// chain is finite, and the extent is asserted exactly — over a 4-node chain
/// the closure is the six ordered pairs and *not* the self-loops a cyclic input
/// would add.
#[test]
fn compose_into_self_is_the_transitive_closure() {
    let r = run(
        &(algebra("compose")
            + "(compose lt lt lt)
               (lt A B :source \"(1)\") (lt B C :source \"(2)\") (lt C D :source \"(3)\")"),
    );
    assert_eq!(
        r.extent("lt"),
        extent(&[
            "(lt A B)", "(lt B C)", "(lt C D)", "(lt A C)", "(lt B D)", "(lt A D)",
        ]),
        "the closure of the chain, and no self-loop"
    );
}

/// `identity-materialises-the-diagonal` — `(identity R isa Dom)` self-loops the
/// `Dom` extent and derives no off-diagonal pair.
///
/// `identity` is one of the *extensive* operations: it materialises pairs that
/// are absent from every edge set, so it cannot read edges and instead ranges
/// over the puzzle's instance-type relation, handed in as the `?isa` parameter.
/// Off-diagonal absence is the whole content of "diagonal", and it is asserted
/// by comparing the extent rather than by spot-checking one missing pair.
#[test]
fn identity_materialises_the_diagonal_of_the_extent() {
    let r = run(
        &(algebra("identity")
            + "(is-a H1 House) (is-a H2 House)
               (identity same is-a House)"),
    );
    assert_eq!(
        r.extent("same"),
        extent(&["(same H1 H1)", "(same H2 H2)"]),
        "the diagonal and only the diagonal"
    );
}

// ── the Boolean (lattice) layer ────────────────────────────────────

/// `meet-is-intersection` — `(meet R S T)` derives exactly the pairs present in
/// both operands.
///
/// Each operand is given a pair the other does not have, because that is where
/// an intersection can go wrong: a rule that had dropped one of its two
/// premises would still derive the shared pair, and would additionally derive
/// the private one — which only an extent comparison notices.
#[test]
fn meet_is_intersection() {
    let r = run(
        &(algebra("meet")
            + "(meet owns rents both)
               (owns A B :source \"(1)\") (owns A C :source \"(2)\")
               (rents A B :source \"(3)\") (rents X Y :source \"(4)\")"),
    );
    assert_eq!(
        r.extent("both"),
        extent(&["(both A B)"]),
        "the shared pair, neither private one"
    );
}

/// `difference-is-set-minus` — `(difference R S T)` keeps the pairs of `R` that
/// `S` does not have.
///
/// The exclusion runs through negation as failure, which makes this the one
/// lattice operation whose reading is closed-world: it is sound exactly when
/// `S` is saturation-determined, which is why `stdlib/algebra.ein` documents
/// the caveat and makes the operation opt-in per use. What the test pins is the
/// semantics — `(A B)` is in both operands and must not survive the
/// subtraction.
#[test]
fn difference_is_set_minus() {
    let r = run(
        &(algebra("difference")
            + "(difference owns rents only-owns)
               (owns A B :source \"(1)\") (owns A C :source \"(2)\")
               (rents A B :source \"(3)\")"),
    );
    assert_eq!(
        r.extent("only-owns"),
        extent(&["(only-owns A C)"]),
        "the pair rents does not have, and not the shared one"
    );
}

/// `join-fans-out-then-unions` — `(join R S U)` works in two stages, and the
/// second is driven by an activator the first *derived*.
///
/// `join` cannot be one generic rule with an `(or …)` premise: the loader
/// splits a disjunctive match into `join__or0` / `join__or1` siblings, and an
/// activator fact is registered against a rule's exact name, so the `(join …)`
/// fact could never reach either half. The library works around that with a
/// non-generic fan-out rule that multi-asserts two per-operand copier
/// activators. Both stages are asserted, because the derived activators are the
/// mechanism and the union is only its consequence.
#[test]
fn join_fans_out_into_copiers_that_then_union() {
    let r = run(
        &(algebra("derive-join join-l join-r")
            + "(join owns rents has)
               (owns A B :source \"(1)\")
               (rents C D :source \"(2)\")"),
    );
    assert!(
        r.was_derived("(join-l owns rents has)") && r.was_derived("(join-r owns rents has)"),
        "the fan-out did not happen: {:?}",
        r.derived
    );
    assert_eq!(
        r.extent("has"),
        extent(&["(has A B)", "(has C D)"]),
        "both operands, once each"
    );
}

/// `empty-check-fires-on-any-edge` — `(empty R)` is a check: one edge in `R` is
/// ⊥, and an edge in some other relation is not.
///
/// The second half is what makes the first half mean anything. `empty`'s
/// premise is a bare `(?R ?a ?b)` whose head is the activator parameter, so it
/// compiles to a scan of one named relation; a compilation that had lost the
/// binding and scanned everything would fire on `(bar A B)` too, and only the
/// silent program can see that.
#[test]
fn empty_fires_on_any_edge_of_its_own_relation_only() {
    let tripped = run(&(algebra("empty") + "(empty foo)\n(foo A B :source \"(1)\")"));
    assert!(tripped.is_false(), "a non-empty relation was accepted");

    let untouched = run(&(algebra("empty") + "(empty foo)\n(bar A B :source \"(1)\")"));
    assert!(
        !untouched.is_false(),
        "an edge on another relation tripped the check"
    );
}

/// `top-fills-the-rectangle` — `(top R isa Dom Ran)` materialises the whole
/// `Dom × Ran` product.
///
/// Extensive again, and asymmetric on purpose: two `Dom` elements against one
/// `Ran` element, so a rule that had confused its two type arguments would
/// produce a differently-shaped rectangle rather than the same one. The extent
/// is the claim — "every pair" is not observable from one pair.
#[test]
fn top_fills_the_rectangle() {
    let r = run(
        &(algebra("top")
            + "(is-a A1 Dom) (is-a A2 Dom)
               (is-a B1 Ran)
               (top all is-a Dom Ran)"),
    );
    assert_eq!(
        r.extent("all"),
        extent(&["(all A1 B1)", "(all A2 B1)"]),
        "the full 2×1 rectangle"
    );
}

/// `complement-materialises-the-absent-pairs` — the one operation that reads
/// the *absence* of an edge and writes a fact about it.
///
/// Over a 2×2 universe with a single positive edge the complement is the other
/// three pairs, and that positive edge is the falsifier: a `complement` that
/// had ignored its `(absent …)` guard would derive all four, which is what
/// `top` does. The reading inherits the closed-world caveat squarely — it is
/// sound only while `R` is saturation-determined.
#[test]
fn complement_materialises_exactly_the_absent_pairs() {
    let r = run(
        &(algebra("complement")
            + "(is-a A1 Dom) (is-a A2 Dom)
               (is-a B1 Ran) (is-a B2 Ran)
               (r A1 B1 :source \"(1)\")
               (complement r co-r is-a Dom Ran)"),
    );
    assert_eq!(
        r.extent("co-r"),
        extent(&["(co-r A1 B2)", "(co-r A2 B1)", "(co-r A2 B2)"]),
        "the three absent pairs; the present edge shrinks the complement"
    );
}

// ── the single-relation property checks ────────────────────────────

/// `irreflexive-rejects-a-self-loop` — `(R a a)` is ⊥, `(R a b)` is not.
///
/// The rule's premise is `(?R ?a ?a)`: one variable in two argument positions.
/// That repetition is an equality constraint the matcher enforces while
/// binding, not a filter applied afterwards, and the silent half of this test
/// is what proves it — if the second occurrence bound freely, the check would
/// fire on every edge.
#[test]
fn irreflexive_rejects_a_self_loop_and_passes_a_plain_edge() {
    assert!(
        run(&(algebra("irreflexive") + "(irreflexive r)\n(r A A :source \"(1)\")")).is_false(),
        "a self-loop was accepted"
    );
    assert!(
        !run(&(algebra("irreflexive") + "(irreflexive r)\n(r A B :source \"(1)\")")).is_false(),
        "a plain edge was rejected"
    );
}

/// `antisymmetric-rejects-a-mutual-pair-but-allows-a-self-loop` — the `neq`
/// guard is the entire difference between antisymmetry and asymmetry.
///
/// `R(a,b) ∧ R(b,a) ⟹ a = b` permits the diagonal, so `(r A A)` is a mutual
/// pair that must *not* fire. Both halves belong in one test because the
/// distinguishing content is the contrast: drop the guard and the first half
/// still passes while the second goes red.
#[test]
fn antisymmetric_rejects_a_distinct_mutual_pair_but_allows_a_self_loop() {
    let mutual = run(
        &(algebra("antisymmetric")
            + "(antisymmetric r)
               (r A B :source \"(1)\") (r B A :source \"(2)\")"),
    );
    assert!(mutual.is_false(), "a distinct mutual pair was accepted");

    let loop_only = run(&(algebra("antisymmetric") + "(antisymmetric r)\n(r A A :source \"(1)\")"));
    assert!(
        !loop_only.is_false(),
        "antisymmetry rejected the diagonal — that is asymmetry"
    );
}

/// `asymmetric-rejects-a-self-loop` — asymmetry is strictly stronger, and the
/// self-loop is exactly where the two differ.
///
/// `R ∩ R° = 0` has no `neq` escape, so `(r A A)` matches both premises with
/// the same fact and is ⊥. Pinning the case the previous test says is *allowed*
/// under antisymmetry is what makes "strictly stronger" a falsifiable claim
/// rather than a remark in the library.
#[test]
fn asymmetric_rejects_the_self_loop_antisymmetry_allows() {
    assert!(
        run(&(algebra("asymmetric") + "(asymmetric r)\n(r A A :source \"(1)\")")).is_false(),
        "asymmetry accepted a self-loop"
    );
}

/// `connex-rejects-an-incomparable-pair` — a linear order has no incomparable
/// pair, and one orientation is enough to make a pair comparable.
///
/// Extensive and closed-world: the rule quantifies over the `Dom` extent and
/// fires on the *absence* of both orientations, so it needs the instance-type
/// relation rather than an edge set — there are no `r` edges at all in the
/// first program and it still fires. The silent half establishes that one
/// direction suffices; a check demanding both would reject every strict order,
/// which is the opposite of what connexity means.
#[test]
fn connex_rejects_an_incomparable_pair_and_accepts_one_orientation() {
    let incomparable = run(
        &(algebra("connex")
            + "(is-a A Dom) (is-a B Dom)
               (connex r is-a Dom)"),
    );
    assert!(incomparable.is_false(), "an incomparable pair was accepted");

    let comparable = run(
        &(algebra("connex")
            + "(is-a A Dom) (is-a B Dom)
               (connex r is-a Dom)
               (r A B :source \"(1)\")"),
    );
    assert!(
        !comparable.is_false(),
        "one orientation should already make the pair comparable"
    );
}

/// `difunctional-closes-overlapping-rows` — rows that share a column agree on
/// every column.
///
/// `R;R°;R ⊆ R` is a *closure*, not a check, so it writes into the relation it
/// reads and could in principle run away; it terminates because the conclusion
/// only ever pairs nodes that already occur. The fixpoint is asserted whole
/// because the closure's effect is symmetric — deriving `(r A D)` makes the two
/// rows equal, and the extent is the only way to say that.
#[test]
fn difunctional_closes_overlapping_rows() {
    let r = run(
        &(algebra("difunctional")
            + "(difunctional r)
               (r A B :source \"(1)\") (r C B :source \"(2)\") (r C D :source \"(3)\")"),
    );
    assert_eq!(
        r.extent("r"),
        extent(&["(r A B)", "(r A D)", "(r C B)", "(r C D)"]),
        "the two overlapping rows became equal"
    );
}

// ── the equational lemmas ──────────────────────────────────────────

/// `schroder-negative-s` — a missing composite forces a missing right factor.
///
/// Schröder's cycle law read operationally: for a *closed* `T = R;S`,
/// `¬T(a,c) ∧ R(a,b) ⟹ ¬S(b,c)`. Two things make it worth pinning. The premise
/// is a stored `(not (T a c))` — a negative fact matched positively, not a NAF
/// guard — and the conclusion is another stored negative, which is how a
/// negative propagates at all in an append-only KB. The closed-`T` obligation
/// is stricter than `compose`'s, which is why the activator is a separate
/// declaration rather than something `(compose …)` implies.
#[test]
fn schroder_reads_a_missing_composite_back_into_the_right_factor() {
    let r = run(
        &(algebra("compose-negative-s")
            + "(compose-negative-s right-of right-of two-right)
               (right-of A B :source \"(1)\")
               (not (two-right A C) :source \"(2)\")"),
    );
    assert!(
        r.was_derived("(not (right-of B C))"),
        "¬T(A,C) ∧ R(A,B) should give ¬S(B,C): {:?}",
        r.derived
    );
}

/// `schroder-negative-r` — the dual: a missing composite forces a missing left
/// factor.
///
/// Kept separate from the right-factor rule rather than merged with it: these
/// are two rules with two activators, their premises differ in which factor is
/// known, and one test asserting both would not say which of them broke. The
/// contrast is also the point — the same `¬T(A,C)` combines with an edge on the
/// *other* side of the composition and yields a negative about the other
/// factor.
#[test]
fn schroder_reads_a_missing_composite_back_into_the_left_factor() {
    let r = run(
        &(algebra("compose-negative-r")
            + "(compose-negative-r right-of right-of two-right)
               (right-of B C :source \"(1)\")
               (not (two-right A C) :source \"(2)\")"),
    );
    assert!(
        r.was_derived("(not (right-of A B))"),
        "¬T(A,C) ∧ S(B,C) should give ¬R(A,B): {:?}",
        r.derived
    );
}

/// `contravariance-derives-the-converse-composite` — B7, `(A;B)° = B°;A°`, as a
/// rule that writes a new *operator* fact.
///
/// This is the sharpest form of the reflective idiom in the library: the
/// conclusion `(compose sc rc tc)` is not an edge but an activator, so the
/// engine goes on to compile and run a composition nobody declared. The order
/// reversal is the content — `Sc` before `Rc` — which is why the covariant
/// spelling is asserted absent rather than left unmentioned.
#[test]
fn contravariance_derives_the_converse_composite_as_a_new_activator() {
    let r = run(
        &(algebra("compose converse compose-contravariant")
            + "(compose r s t)
               (converse r rc) (converse s sc) (converse t tc)"),
    );
    assert!(
        r.was_derived("(compose sc rc tc)"),
        "B7 should reverse the operands: {:?}",
        r.derived
    );
    assert!(
        !r.has("(compose rc sc tc)"),
        "contravariance is not covariance"
    );
}

/// `join-converse-derives-the-converse-union` — B8, `(A∨B)° = A°∨B°`.
///
/// Same shape as B7, and here for the same reason: union is *covariant* under
/// converse where composition is contravariant, so the two lemmas differ in
/// exactly the operand order of their conclusions, and pinning one without the
/// other would leave that difference untested.
#[test]
fn join_converse_derives_the_converse_union() {
    let r = run(
        &(algebra("derive-join join-l join-r converse join-converse")
            + "(join r s u)
               (converse r rc) (converse s sc) (converse u uc)"),
    );
    assert!(
        r.was_derived("(join rc sc uc)"),
        "B8 should keep the operand order: {:?}",
        r.derived
    );
}

/// `the-reflective-lemma-set-terminates` — `compose` + `converse` +
/// `converse-pair-symmetric` + `compose-contravariant` reaches a fixpoint.
///
/// These lemmas derive the activators that switch each other on, so the set is
/// mutually recursive over *operator* facts as well as over edges: B7 applied
/// to its own output, with `converse-pair-symmetric` supplying the back-edges,
/// re-derives the original `(compose r s t)`. It terminates because that fact
/// already exists — nothing about the rules themselves bounds the loop. The
/// 8 000-step budget is the runaway detector; the firing count is the
/// regression detector.
#[test]
fn the_reflective_lemma_set_terminates() {
    let src = algebra("compose converse converse-pair-symmetric compose-contravariant")
        + "(compose r s t)
           (converse r rc) (converse s sc) (converse t tc)
           (r A B :source \"(1)\") (s B C :source \"(2)\")";
    let r = try_run(&src, 8000).unwrap_or_else(|e| panic!("the lemma set ran away: {e}"));
    assert!(
        r.firings < 0,
        "converged, but in {} firings — the lemma set is no longer cheap",
        r.firings
    );
}
