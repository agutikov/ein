//! A `(solve … :expect …)` claim against R — M1e2
//! [S1e2.1a.4](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.4_the_language.md).
//!
//! `tests/region/` holds one program per form, each a claim that holds. This
//! file holds the other half: every way a claim can be false is reported as
//! false, the two ways it can go unchecked are reported as unchecked, and the
//! loader refuses what is not a claim. Each program is small enough that its R
//! can be read off the comment beside it.
//!
//! ```text
//! cargo test -p ein-infer --test claim_semantics
//! ```

use ein_core::{Kb, Terms};
use ein_infer::claim::{self, Options};
use ein_infer::expect::Outcome;
use ein_ir::{Ast, load, parse};

/// Three properties of one item, the triple dead: R is ∅, three singletons and
/// three pairs, seven states, each reached by one commitment.
const TRIPLE: &str = "
(rule three-conflict () :match (and (a-prop ?x) (b-prop ?y) (c-prop ?z)) :assert (false) :why \"a-prop holds of {?x}, b-prop of {?y} and c-prop of {?z}, which cannot all be\")
(hrule guess (?R ?T) :match (is-a ?x ?T) :assert (?R ?x) :why \"guess that {?R} holds of {?x}, a {?T}\")
(relation is-a T T :why \"{?1} is a {?2}\") (relation a-prop T :why \"a-prop holds of {?1}\") (relation b-prop T :why \"b-prop holds of {?1}\") (relation c-prop T :why \"c-prop holds of {?1}\")
(is-a Item T) (is-a X Item)
";
const TRIPLE_HRULES: &str = ":hrules (guess (a-prop Item) guess (b-prop Item) guess (c-prop Item))";
const TRIPLE_R: &str = "(model) (model (a-prop X)) (model (b-prop X)) (model (c-prop X)) \
     (model (a-prop X) (b-prop X)) (model (a-prop X) (c-prop X)) (model (b-prop X) (c-prop X))";

/// A chain under transitivity, the blind rung proposing the reversed pairs.
/// R has four states; the top one is reached by five commitments, two of them
/// minimal: {(co-located C A)} and {(co-located B A), (co-located C B)}.
const CHAIN: &str = "
(rule transitive (?rel) :match (and (?rel ?a ?b) (?rel ?b ?c) (neq ?a ?c)) :assert (?rel ?a ?c) :why \"{?rel} relates {?a} to {?b} and {?b} to {?c}, and is transitive, so it relates {?a} to {?c}\")
(relation co-located T T :why \"{?1} is co-located with {?2}\") (transitive co-located)
(co-located A B) (co-located B C)
";
const CHAIN_ROOT: &str = "(co-located A B) (co-located A C) (co-located B C)";
const CHAIN_TOP: &str = "(co-located A B) (co-located A C) (co-located B A) (co-located B C) \
     (co-located C A) (co-located C B)";

fn chain_r(merged: &str) -> String {
    format!(
        "(model {CHAIN_ROOT}) \
         (model {CHAIN_ROOT} (co-located B A)) \
         (model {CHAIN_ROOT} (co-located C B)) \
         (model {CHAIN_TOP}) {merged}"
    )
}

/// Ann likes nothing and owes a liking; nothing is guessed. R is empty, and
/// root is the one alive node that owes.
const OWING: &str = "
(import std.algebra :symbols (total-owed))
(relation is-a T T :why \"{?1} is a {?2}\") (relation likes Person Food :why \"{?1} likes {?2}\")
(is-a Ann Person) (is-a Bob Person) (is-a Soup Food) (is-a Stew Food)
(total-owed likes is-a)
(likes Bob Stew)
";

/// A self-loop `irreflexive` refutes; nothing is guessed. Every node is dead.
const DEAD: &str = "
(import std.algebra :symbols (irreflexive))
(relation loops Node Node :why \"{?1} loops to {?2}\") (irreflexive loops)
(loops A A)
";

/// One candidate, and a rule that writes a stored negative beside it: R is ∅
/// and {(p A)}, whose state holds `(not (q A))`.
const NEGATIVE: &str = "
(relation is-a T T :why \"{?1} is a {?2}\") (relation p T :why \"p holds of {?1}\") (relation q T :why \"q holds of {?1}\")
(is-a A T)
(hrule guess () :match (is-a ?x T) :assert (p ?x) :why \"guess that p holds of {?x}\")
(rule writes-a-negative () :match (p ?x) :assert (not (q ?x)) :why \"p holds of {?x}, so q does not hold of {?x}\")
";

/// Seating Ann makes her a diner, and a diner owes a meal: flagged by the
/// static check, so R is the cube's. ∅, {meal}, {meal, seat}.
const FLAGGED: &str = "
(import std.algebra :symbols (total-owed))
(relation is-a T T :why \"{?1} is a {?2}\") (relation seats Guest Chair :why \"{?1} is seated on {?2}\") (relation meal Diner Food :why \"{?1} has {?2} for a meal\")
(is-a Ann Guest) (is-a C1 Chair) (is-a Soup Food)
(rule seated-dines () :match (seats ?g ?c) :assert (is-a ?g Diner) :why \"{?g} is seated on {?c}, so {?g} is a Diner\")
(total-owed meal is-a)
(hrule seat () :match (and (is-a ?g Guest) (is-a ?c Chair)) :assert (seats ?g ?c) :why \"guess that guest {?g} is seated on chair {?c}\")
(hrule dine () :match (and (is-a ?g Guest) (is-a ?f Food)) :assert (meal ?g ?f) :why \"guess that guest {?g} has {?f} for a meal\")
";

fn loaded(src: &str) -> Result<(Kb, Terms, Ast), String> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, src, None).map_err(|e| format!("{e:?}"))?;
    let kb = load(&mut ast, &mut terms, &forms, None).map_err(|e| e.to_string())?;
    Ok((kb, terms, ast))
}

/// The outcome and the report's lines for `body` + `(solve <kws> :expect <claim>)`.
fn check_with(body: &str, kws: &str, claim_src: &str, opts: &Options) -> (Outcome, Vec<String>) {
    let src = format!("{body}\n(solve {kws} :expect {claim_src})\n");
    let (mut kb, mut terms, ast) = loaded(&src).unwrap_or_else(|e| panic!("{e}\n{src}"));
    let node = kb
        .program()
        .solve()
        .and_then(|s| ein_infer::solve_value(&ast, s, "expect"))
        .expect("an :expect");
    let c = ein_ir::claim::parse(&ast, node).expect("the loader accepted it");
    let checked = claim::check(&mut kb, &mut terms, &ast, &c, opts).expect("checks");
    (checked.report.outcome, checked.report.lines)
}

fn check(body: &str, kws: &str, claim_src: &str) -> (Outcome, Vec<String>) {
    check_with(body, kws, claim_src, &Options::default())
}

fn holds(body: &str, kws: &str, claim_src: &str) {
    let (o, lines) = check(body, kws, claim_src);
    assert_eq!(o, Outcome::Held, "{claim_src}\n{lines:#?}");
}

fn fails(body: &str, kws: &str, claim_src: &str, says: &str) {
    let (o, lines) = check(body, kws, claim_src);
    assert_eq!(o, Outcome::Failed, "{claim_src}\n{lines:#?}");
    assert!(
        lines.iter().any(|l| l.contains(says)),
        "{claim_src}: wanted {says:?} in {lines:#?}"
    );
}

fn refused(src: &str, says: &str) {
    match loaded(src) {
        Ok(_) => panic!("loaded: {src}"),
        Err(e) => assert!(e.contains(says), "wanted {says:?} in {e:?}"),
    }
}

// ── States ─────────────────────────────────────────────────────────────

#[test]
fn every_state_listed_holds_in_any_order() {
    holds(TRIPLE, TRIPLE_HRULES, &format!("(region {TRIPLE_R})"));
    let reversed = "(model (b-prop X) (c-prop X)) (model (a-prop X) (c-prop X)) \
         (model (a-prop X) (b-prop X)) (model (c-prop X)) (model (b-prop X)) \
         (model (a-prop X)) (model)";
    holds(TRIPLE, TRIPLE_HRULES, &format!("(region {reversed})"));
}

#[test]
fn a_missing_state_fails_and_is_named() {
    let six = TRIPLE_R.replace("(model (b-prop X) (c-prop X))", "");
    fails(
        TRIPLE,
        TRIPLE_HRULES,
        &format!("(region {six})"),
        "the claim lists 6 states and R has 7",
    );
    fails(
        TRIPLE,
        TRIPLE_HRULES,
        &format!("(region {six})"),
        "a state of R no model lists: (model (b-prop X) (c-prop X))",
    );
}

#[test]
fn a_state_that_is_not_in_r_fails() {
    let wrong = TRIPLE_R.replace(
        "(model (b-prop X) (c-prop X))",
        "(model (a-prop X) (b-prop X) (c-prop X))",
    );
    fails(
        TRIPLE,
        TRIPLE_HRULES,
        &format!("(region {wrong})"),
        "a listed model no state of R is",
    );
}

/// `(model)` is a state with *no* fact of any relation the claim names, so
/// leaving it out is a missing state — the empty model is not a wildcard.
#[test]
fn the_empty_model_is_root_and_not_a_wildcard() {
    let no_root = TRIPLE_R.replacen("(model) ", "", 1);
    fails(
        TRIPLE,
        TRIPLE_HRULES,
        &format!("(region {no_root})"),
        "a state of R no model lists: (model)",
    );
}

/// Closure is across the whole claim: a relation one model names is closed in
/// every model, so `(model (a-prop X))` beside models naming `b-prop` says
/// there is no `b-prop` fact in that state.
#[test]
fn naming_a_relation_closes_it_in_every_model() {
    // The same seven states, with `c-prop` named nowhere: then the three
    // states that differ only in `c-prop` collapse onto others, and the
    // matching cannot be one to one.
    let blind_to_c = "(model) (model (a-prop X)) (model (b-prop X)) (model) \
         (model (a-prop X) (b-prop X)) (model (a-prop X)) (model (b-prop X))";
    holds(TRIPLE, TRIPLE_HRULES, &format!("(region {blind_to_c})"));
}

/// A listed `(not …)` is checked for presence, and closes nothing.
#[test]
fn a_listed_negative_is_checked_for_presence() {
    holds(NEGATIVE, "", "(region (model) (model (p A) (not (q A))))");
    fails(
        NEGATIVE,
        "",
        "(region (model) (model (p A) (not (q B))))",
        "a listed model no state of R is",
    );
}

// ── (merged …) ─────────────────────────────────────────────────────────

#[test]
fn merged_with_every_minimal_commitment_holds() {
    let merged = format!(
        "(merged (model {CHAIN_TOP}) (or (model (co-located B A) (co-located C B)) \
                                         (model (co-located C A))))"
    );
    holds(CHAIN, "", &format!("(region {})", chain_r(&merged)));
    // …and with no (merged …) at all the claim is about states only.
    holds(CHAIN, "", &format!("(region {})", chain_r("")));
}

#[test]
fn merged_with_a_minimal_commitment_missing_fails() {
    let merged = format!("(merged (model {CHAIN_TOP}) (model (co-located C A)))");
    fails(
        CHAIN,
        "",
        &format!("(region {})", chain_r(&merged)),
        "its minimal ones are not the listed ones",
    );
}

#[test]
fn merged_with_a_commitment_that_is_not_minimal_fails() {
    let merged = format!(
        "(merged (model {CHAIN_TOP}) (or (model (co-located B A) (co-located C A)) \
                                         (model (co-located C A))))"
    );
    fails(
        CHAIN,
        "",
        &format!("(region {})", chain_r(&merged)),
        "its minimal ones are not the listed ones",
    );
}

#[test]
fn merged_on_a_state_one_commitment_reaches_fails() {
    let merged = format!(
        "(merged (model {CHAIN_TOP}) (or (model (co-located B A) (co-located C B)) \
                                         (model (co-located C A)))) \
         (merged (model {CHAIN_ROOT} (co-located B A)) (model (co-located B A)))"
    );
    fails(
        CHAIN,
        "",
        &format!("(region {})", chain_r(&merged)),
        "which one commitment alone reaches",
    );
}

/// A `(merged …)` names a state several commitments reach: on a state one
/// commitment reaches, it over-states the class.
#[test]
fn merged_needs_a_state_several_commitments_reach() {
    let src = "
(relation h T :why \"h holds of {?1}\") (relation slot T :why \"{?1} is a slot\")
(slot A) (slot C)
(hrule guess () :match (slot ?x) :assert (h ?x) :why \"guess that h holds of slot {?x}\")
(rule imp-A () :match (h A) :assert (h B) :why \"h holds of A, so h holds of B\")
(rule imp-C () :match (h C) :assert (h D) :why \"h holds of C, so h holds of D\")
";
    // R: ∅, {A,B} from {A}, {C,D} from {C}, {A,B,C,D} from {A,C}.
    let states = "(model) (model (h A) (h B)) (model (h C) (h D)) (model (h A) (h B) (h C) (h D))";
    holds(src, "", &format!("(region {states})"));
    let all = format!(
        "(region {states} \
           (merged (model (h A) (h B)) (model (h A))) \
           (merged (model (h C) (h D)) (model (h C))) \
           (merged (model (h A) (h B) (h C) (h D)) (model (h A) (h C))))"
    );
    let (o, lines) = check(src, "", &all);
    // `B` is not a candidate (there is no `(slot B)`), so only {A} reaches
    // {A,B}, and the claim above over-states that class. It says which.
    assert_eq!(o, Outcome::Failed, "{lines:#?}");
    assert!(
        lines
            .iter()
            .any(|l| l.contains("which one commitment alone reaches")),
        "{lines:#?}"
    );
}

/// All or nothing: with one `(merged …)`, every state reached by more than one
/// commitment has one.
#[test]
fn merged_is_all_or_nothing() {
    let src = "
(relation h T :why \"h holds of {?1}\") (relation slot T :why \"{?1} is a slot\")
(slot A) (slot B) (slot C)
(hrule guess () :match (slot ?x) :assert (h ?x) :why \"guess that h holds of slot {?x}\")
(rule imp-A-B () :match (h A) :assert (h B) :why \"h holds of A, so h holds of B\")
";
    // {A,B} is reached by {A} and {A,B}, and {A,B,C} by {A,C} and {A,B,C}.
    let states = "(model) (model (h B)) (model (h C)) (model (h B) (h C)) \
         (model (h A) (h B)) (model (h A) (h B) (h C))";
    let both = format!(
        "(region {states} \
           (merged (model (h A) (h B)) (model (h A))) \
           (merged (model (h A) (h B) (h C)) (model (h A) (h C))))"
    );
    holds(src, "", &both);
    let one = format!("(region {states} (merged (model (h A) (h B)) (model (h A))))");
    fails(src, "", &one, "no (merged …) lists it");
}

#[test]
fn merged_with_the_wrong_generator_fails() {
    // examples/lattice/synth/merged_states.ein's shape, with B and D
    // candidates too: every state is reached by two commitments.
    let src = "
(relation h T :why \"h holds of {?1}\") (relation slot T :why \"{?1} is a slot\")
(slot A) (slot B)
(hrule guess () :match (slot ?x) :assert (h ?x) :why \"guess that h holds of slot {?x}\")
(rule imp-A-B () :match (h A) :assert (h B) :why \"h holds of A, so h holds of B\")
";
    // R: ∅, {B}, and {A,B} reached by {A} and {A,B}.
    let one = "(region (model) (model (h B)) (model (h A) (h B)) \
                 (merged (model (h A) (h B)) (model (h A))))";
    holds(src, "", one);
    let wrong = "(region (model) (model (h B)) (model (h A) (h B)) \
                   (merged (model (h A) (h B)) (model (h A) (h B))))";
    fails(src, "", wrong, "its minimal ones are not the listed ones");
}

// ── Empty R, and every node dead ───────────────────────────────────────

#[test]
fn owing_states_the_owing_frontier() {
    let kws = ":no-hypothesis (is-a likes)";
    holds(OWING, kws, "(region (owing (model (likes Bob Stew))))");
    fails(
        OWING,
        kws,
        "(region (owing (model (likes Bob Soup))))",
        "an owing state no model lists",
    );
    // Listed as a state of R, the same facts are false: R is empty.
    fails(
        OWING,
        kws,
        "(region (model (likes Bob Stew)))",
        "the claim lists 1 state and R is empty",
    );
    holds(OWING, kws, "(region)");
    fails(OWING, kws, "(false)", "that is `(region (owing …))`");
}

#[test]
fn false_is_every_node_dead() {
    let kws = ":no-hypothesis (loops)";
    holds(DEAD, kws, "(false)");
    holds(DEAD, kws, "(region)");
    fails(
        DEAD,
        kws,
        "(region (owing (model (loops A A))))",
        "that is `(false)`",
    );
    fails(
        TRIPLE,
        TRIPLE_HRULES,
        "(false)",
        "(false) says every node is dead, and R has 7 states",
    );
    fails(
        TRIPLE,
        TRIPLE_HRULES,
        "(region)",
        "the claim says R is empty, and R has 7 states",
    );
}

// ── The cube, and what goes unchecked ──────────────────────────────────

#[test]
fn a_flagged_program_is_checked_against_the_cube() {
    holds(
        FLAGGED,
        "",
        "(region (model) (model (meal Ann Soup)) (model (meal Ann Soup) (seats Ann C1)))",
    );
    // {(seats Ann C1)} alone owes Ann's meal: a hole, not a state.
    fails(
        FLAGGED,
        "",
        "(region (model) (model (meal Ann Soup)) (model (seats Ann C1)) \
                 (model (meal Ann Soup) (seats Ann C1)))",
        "the claim lists 4 states and R has 3",
    );
}

#[test]
fn a_route_that_stops_leaves_the_claim_unchecked() {
    let (o, lines) = check_with(
        TRIPLE,
        TRIPLE_HRULES,
        &format!("(region {TRIPLE_R})"),
        &Options {
            budget: 1,
            ..Options::default()
        },
    );
    assert_eq!(o, Outcome::NotChecked, "{lines:#?}");
    assert!(
        lines[0].contains("the state route stopped at its budget"),
        "{lines:#?}"
    );

    let (o, lines) = check_with(
        FLAGGED,
        "",
        "(region (model))",
        &Options {
            limit: 1,
            ..Options::default()
        },
    );
    assert_eq!(o, Outcome::NotChecked, "{lines:#?}");
    assert!(lines[0].contains("past the oracle's limit"), "{lines:#?}");
}

// ── What the loader refuses ────────────────────────────────────────────

/// What is not a claim is refused. Until M1f S1f.5.5's swap the loader
/// refused every shape a claim may not take with a line of its own; since, the
/// grammar states a claim's shapes, so a top-level `(region …)` or `(owing
/// …)`, a claim that is no `(region …)`, a variable in a claim's fact, a
/// `(merged …)` with no `<from>`, an empty `(owing)` and a fact where a
/// `(model …)` goes are parse errors at their place, rows of `ein-ir`'s
/// grammar table since T1f.5.5.5 deleted the loader's lines for them
/// (`ein-ir/tests/grammar_pages.rs`). The loader keeps what a context-free
/// grammar does not see: an `(owing …)` beside listed states, a merged state
/// the region does not list, and a relation no declaration or fact makes.
#[test]
fn the_loader_refuses_what_is_not_a_claim() {
    let body = "(relation p T :why \"p holds of {?1}\") (p A)";
    refused(
        &format!("{body} (solve :expect (region (model (p A)) (owing (model))))"),
        "`(owing …)` says R is empty, and the region lists states",
    );
    refused(
        &format!("{body} (solve :expect (region (model (p A)) (merged (model) (model (p A)))))"),
        "its state is not one the region lists",
    );
    refused(
        &format!("{body} (solve :expect (region (model (nosuch A))))"),
        "names nosuch, which no declaration or fact makes a relation",
    );
    refused(
        &format!("{body} (solve :expect (region (owing (model)) (owing (model))))"),
        "one `(owing …)`, listing every owing state",
    );
    refused(
        &format!("{body} (solve :expect (region (merged (model) (model (p A)))))"),
        "a `(merged …)` names a state, and the region lists none",
    );
}
