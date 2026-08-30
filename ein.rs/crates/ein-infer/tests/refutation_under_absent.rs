//! **A refutation resting on an `absent` the search can still fill** — M1e
//! [S1e.2.3](../../../../plans/m1e_review_processing/p1e.2_high/s1e.2.3_naf_refutation_diagnostic.md),
//! the containment for
//! [Q-M1e.9](../../../../plans/m1e_review_processing/open_questions.md).
//!
//! ## What is unsound, and why a warning is the right size of answer
//!
//! `dead` is **not** upward-closed under `absent`. The premise
//! [design/08](../../../../docs/history/m1a_rust/design/08_parallelism.md)
//! states — *`X ⊆ Y ∧ dead(X) ⇒ dead(Y)`, because the KB is append-only and
//! nothing retracts* — establishes that `sat` is **inflationary**, not that it
//! is monotone in its input, and `(absent P)` is exactly what separates the
//! two. A twenty-line probe has `{(p A)}` dead and `{(p A), (q A)}` alive, and
//! **five of the six shipped configurations answer it wrongly**, every one of
//! them reporting `exhausted = true`. Three mechanisms read the false premise
//! and each is sufficient alone: the lookahead kill cache, the singleton
//! writeback, and the width-1 no-good clause.
//!
//! [D4](../../../../plans/m1e_review_processing/p1e.1_open_questions/s1e.1.1_search_soundness_probes/d4_q_m1e9_upward_closure.md)
//! ruled **B**: narrow the claim and diagnose it, rather than make the three
//! consumers world-aware (that is `F18`) or refuse the shape at load (that is
//! S1f.10.8's, and a refusal today would refuse `std.algebra`'s `connex`
//! before anyone has decided whether that rule should be rewritten).
//!
//! ## This file is the census, and the census decided the default
//!
//! The stage's rule was: measure first, then set the default — *a warning
//! nobody sees is not containment, and a warning that fires on `zebra2` is one
//! that gets disabled*. The corpus is **not** silent, so the warning ships
//! **on** but the seven exposed entries are named here rather than left to be
//! rediscovered. They are the input
//! [S1f.10.8](../../../../plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md)
//! T1 starts from.
//!
//! The claim is deliberately **exact** — the set, not a bound — because both
//! directions are findings. A new entry appearing means a program was written
//! with the shape; one disappearing means a rule was rewritten, and S1f.10.8
//! wants to know which.

use std::collections::BTreeSet;
use std::path::Path;

use ein_core::Terms;
use ein_corpus::{corpus_files, repo_root};
use ein_infer::events::Events;
use ein_infer::naf_deps::{REFUTATION_UNDER_ABSENT, compute_naf_map, naf_warnings};
use ein_ir::{Ast, load, parse};

/// D4's probe, as a **corpus** fixture.
///
/// The original is `plans/m1e_review_processing/…/probes/naf_upward_closure.ein`
/// and it will be deleted with the milestone's plan tree, the way M1a's, M1c's
/// and M1d's were. A probe that decides a soundness question belongs where the
/// gate sweeps, so it is banked under `examples/ein-bugs/` with the three
/// sibling fixtures that also state today's answer and are meant to break.
const PROBE: &str = "examples/ein-bugs/naf-upward-closure.ein";

/// One exposed `(rule, activator)`, rendered for the golden set below.
///
/// `path::rule[activator] concludes ← watched` — the four things S1f.10.8 has
/// to look at, and nothing else. The activator is in the key because the
/// exposure is **per activator**: a T2 rule's watched relation is baked to a
/// literal per activation, so `(connex likes)` can be exposed while
/// `(connex is-a)` is not.
fn exposure(path: &Path) -> Vec<String> {
    let rel = path
        .strip_prefix(repo_root())
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let Ok(forms) = parse(&mut ast, &text, path.to_str()) else {
        // A `parse-negative` fixture states nothing about rules.
        return Vec::new();
    };
    let Ok(mut kb) = load(&mut ast, &mut terms, &forms, path.parent()) else {
        // Likewise `load-negative`: 41 of the 212 entries exist to be refused.
        return Vec::new();
    };
    let mut events = Events::off();
    let mut s = ein_infer::saturator::Session {
        kb: &mut kb,
        terms: &mut terms,
        ast: &ast,
        events: &mut events,
        memo: Default::default(),
    };
    let Ok(mut sat) = ein_infer::saturator::Saturator::new(&mut s) else {
        return Vec::new();
    };
    // Root saturation, because the map is only complete on a saturated cache:
    // most NAF-bearing rules are activated by facts a rule derives, so their
    // plan does not exist at load. A `compile-negative` fixture fails here.
    if sat.saturate(&mut s, None, &mut |_| {}).is_err() {
        return Vec::new();
    }
    let Ok(eligible) = ein_infer::hypgen::eligible_relations(&mut s) else {
        return Vec::new();
    };
    compute_naf_map(&sat.engine, s.terms, &eligible)
        .into_iter()
        .filter_map(|d| {
            let r = d.refutation?;
            let act: Vec<&str> = d.activator.iter().map(|&a| s.terms.sym(a)).collect();
            Some(format!(
                "{rel}::{}[{}] {} ← {}",
                s.terms.sym(d.rule),
                act.join(" "),
                r.concludes,
                r.watching.join(", ")
            ))
        })
        .collect()
}

/// **The corpus's exposed set, exactly** — nine rules over seven entries,
/// measured 2026-08-30 over all 213 corpus files.
///
/// Three of the seven are **probes**: `branching/13` and `14` are S1e.1.1's
/// own lookahead-NAF fixtures, and `ein-bugs/naf-upward-closure.ein` is
/// [D4]'s probe itself, banked here. `ein-bugs/alive-empty-interlayer.ein`
/// is a recorded bug fixture whose `totality` rule has the probe's exact
/// shape over a relation its own `(hrule guess …)` proposes.
/// `syntax/rule-forall-and-not.ein` is the one `(not …)` row, and it is there
/// because [`refuting_conclusion`](../src/naf_deps.rs) walks **every**
/// conclusion: its `:assert` is `(and (r ?b ?a) (not (r ?a ?a)))`, so reading
/// `assert_template()` alone — which is what `asserted_relation` does — would
/// have missed it.
///
/// **Four rows are stdlib rules, and they are not the ones [D4] predicted.**
/// D4 named `std.algebra`'s `connex` as the one stdlib rule with the exposed
/// shape, reasoning that the `std.slots` prune / endpoint / adjacency family
/// is safe because its `absent` reads the position structure. The first half
/// holds — none of those 60 rules is here. The second does not: `connex` is
/// activated **twice** in the corpus (`tests/stdlib/algebra/08` and `12`) and
/// is exposed **neither** time, because both fixtures write
/// `:no-hypothesis (instance lt)` and so exclude the subject relation from
/// generation. What *is* exposed is `std.elim`'s `typecheck-arg-0` /
/// `typecheck-arg-1` / `no-room-left` / `no-room`, whose guards read the
/// **membership** relation — `is-a` on `features/05`, `instance` on
/// `features/12` — which nothing in either file closes or excludes, so the
/// blind enumerator can propose one.
///
/// That is the census's real result, and it is a result about *programs*
/// rather than about rules: the same `connex` is exposed or not depending on
/// what its file says about hypotheses, and the discipline that saves it is
/// the query keyword rather than the rule's shape. It is also why the
/// eligibility half had to be a program property — a syntactic census of the
/// same tree finds **60** rules with an `absent` and a refuting conclusion,
/// and nine of them matter.
///
/// [D4]: ../../../../plans/m1e_review_processing/p1e.1_open_questions/s1e.1.1_search_soundness_probes/d4_q_m1e9_upward_closure.md
const EXPOSED: &str = "\
examples/branching/13_lookahead_naf_world.ein::self-block[] (false) \u{2190} cand
examples/branching/14_lookahead_unjudgeable.ein::premature-signoff[] (false) \u{2190} needs-review, pending, signed
examples/ein-bugs/alive-empty-interlayer.ein::totality[] (false) \u{2190} p
examples/ein-bugs/naf-upward-closure.ein::bad[] (false) \u{2190} q
examples/features/05_stdlib_domain_elim.ein::no-room-left[color-of is-a House Color] (false) \u{2190} is-a
examples/features/05_stdlib_domain_elim.ein::typecheck-arg-0[color-of is-a House] (false) \u{2190} is-a
examples/features/05_stdlib_domain_elim.ein::typecheck-arg-1[color-of is-a Color] (false) \u{2190} is-a
examples/features/12_expect_false.ein::no-room[in instance] (false) \u{2190} instance
examples/syntax/rule-forall-and-not.ein::f[] (not \u{2026}) \u{2190} r
";

#[test]
fn the_corpus_exposed_set_is_exactly_these() {
    let mut got: BTreeSet<String> = BTreeSet::new();
    for path in corpus_files() {
        got.extend(exposure(&path));
    }
    let want: BTreeSet<String> = EXPOSED.lines().map(str::to_string).collect();
    let missing: Vec<&String> = want.difference(&got).collect();
    let extra: Vec<&String> = got.difference(&want).collect();
    assert!(
        missing.is_empty() && extra.is_empty(),
        "the exposed set moved.\n  no longer exposed (a rule was rewritten — tell S1f.10.8): {missing:#?}\
         \n  newly exposed (a program was written with the shape): {extra:#?}"
    );
}

/// D4's probe, banked: the warning fires, and it names both replacements.
///
/// The probe is a **mis-encoded obligation** — it says *a world with `p` and
/// without `q` is false* and never says *`q` is required*, which is what
/// `(open ?R)` is for. That reading is the user's, 2026-08-28, and it is why
/// the message has to say *instead* and not only *don't*.
#[test]
fn the_probe_warns_and_the_message_names_both_replacements() {
    let path = repo_root().join(PROBE);
    let text = std::fs::read_to_string(&path).expect("the probe is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("it parses");
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("it loads");
    let mut events = Events::off();
    let mut s = ein_infer::saturator::Session {
        kb: &mut kb,
        terms: &mut terms,
        ast: &ast,
        events: &mut events,
        memo: Default::default(),
    };
    let mut sat = ein_infer::saturator::Saturator::new(&mut s).expect("a saturator");
    sat.saturate(&mut s, None, &mut |_| {})
        .expect("it saturates");
    let eligible = ein_infer::hypgen::eligible_relations(&mut s).expect("an eligible set");

    let ws = naf_warnings(&sat.engine, s.terms, &eligible);
    assert_eq!(ws.len(), 1, "one warning, on one rule: {ws:#?}");
    assert_eq!(ws[0].category, REFUTATION_UNDER_ABSENT);
    let m = &ws[0].text;
    for want in [
        "rule 'bad'",
        "concludes (false)",
        "over q",
        "can still propose",
        // The two replacements the acceptance asks for by name.
        "STORED negative",
        "`total`",
        "(open ?R)",
    ] {
        assert!(m.contains(want), "the message does not say {want:?}:\n{m}");
    }
}

/// The warning is part of the fixture's **recorded output**, not only of a
/// unit test — which is what makes the containment a fixture rather than a
/// claim. `solve_shape` filters the event log to `enter` / `nogood` /
/// `writeback` / `warn`, and the fixture's own `(config :warn-derived-naf
/// true)` is what turns the line on, since `solve` falls back to the KB's
/// config when the caller passes none.
#[test]
fn the_fixtures_solve_shape_carries_the_warning() {
    let path = repo_root().join(PROBE);
    let text = std::fs::read_to_string(&path).expect("the probe is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("it parses");
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("it loads");
    let shape = ein_infer::solve_shape(&ast, &mut terms, &mut kb, "default", 1)
        .expect("the fixture solves");
    assert!(
        shape.contains(REFUTATION_UNDER_ABSENT),
        "the corpus shape does not carry the warning:\n{shape}"
    );
}

/// The one existing hazard signal stays silent here, and that is the finding
/// it was filed as: `warn_derived_naf` watches an `(absent …)` over a
/// **rule-derived** relation, and `q` is only ever proposed by the generator.
/// Two adjacent hazards, and until S1e.2.3 the engine had a warning for one.
#[test]
fn the_stratification_warning_does_not_cover_this() {
    let path = repo_root().join(PROBE);
    let text = std::fs::read_to_string(&path).expect("the probe is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("it parses");
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("it loads");
    let mut events = Events::off();
    let mut s = ein_infer::saturator::Session {
        kb: &mut kb,
        terms: &mut terms,
        ast: &ast,
        events: &mut events,
        memo: Default::default(),
    };
    let mut sat = ein_infer::saturator::Saturator::new(&mut s).expect("a saturator");
    sat.saturate(&mut s, None, &mut |_| {})
        .expect("it saturates");
    assert!(
        ein_infer::naf_deps::derived_naf_warnings(&sat.engine, s.terms).is_empty(),
        "the stratification warning fired on a hypothesis-eligible watch"
    );
}
