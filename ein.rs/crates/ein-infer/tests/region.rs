//! The oracle and the static check — M1e2
//! [S1e2.1a.1](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.1_the_oracle.md).
//!
//! Every number here is Q-M1e2.7's tables, which were taken by a different
//! route (`utils/region.py`: one `ein saturate` per subset); the comparison
//! node for node is `ein-cli/tests/region_dump.rs`'s, because that route runs
//! the binary.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::downset::{self, Declined, DownSet};
use ein_infer::forbid;
use ein_infer::monotone::{Monotonicity, SiteKind};
use ein_infer::region::{self, Options, Outcome, Region};
use ein_infer::states;
use ein_ir::{Ast, load, parse};

// The fact-grain prototype and the loader it shares with `site_census`
// (M1e2 S1e2.1b.1). Neither ships: they are examples' code, held to the cube
// here.
#[allow(dead_code)]
#[path = "../examples/shared/census.rs"]
mod census;
#[allow(dead_code)]
#[path = "../examples/shared/fact_grain.rs"]
mod fact_grain;
// The programs written for this file and for P1e2.1b's censuses (S1e2.1b.3).
#[allow(dead_code)]
#[path = "../examples/shared/written.rs"]
mod written;
// The taint walk `taint_census` reads (S1e2.1b.4).
#[allow(dead_code)]
#[path = "../examples/shared/taint.rs"]
mod taint;

fn compute(rel: &str) -> (Outcome, Terms) {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("parses");
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("loads");
    // A named fixture is read for its region, which is what these tests
    // study, including the shapes *forbid* refuses (M1e2 S1e2.2.5): the dead
    // holes of `naf-upward-closure` and the two `lattice/synth/` witnesses.
    let out = forbid::studying(|| {
        region::compute(&mut kb, &mut terms, &ast, &Options::default(), &mut |_| {})
    })
    .unwrap_or_else(|e| panic!("{rel}: {e}"));
    (out, terms)
}

fn region_of(rel: &str) -> (Region, Terms) {
    match compute(rel) {
        (Outcome::Region(r), t) => (*r, t),
        (Outcome::Refused { n, .. }, _) => panic!("{rel}: refused at n = {n}"),
    }
}

fn names(r: &Region, masks: &[u64]) -> Vec<String> {
    masks.iter().map(|&m| r.name(m)).collect()
}

/// `region.py`'s first lines, so a failure reads like the table.
fn summary(r: &Region) -> String {
    let t = &r.topology;
    let j = |ms: &[u64]| {
        let mut v = names(r, ms);
        v.sort();
        if v.is_empty() {
            "—".to_string()
        } else {
            v.join(", ")
        }
    };
    format!(
        "|R| {}; minimal {}; maximal {}; holes {}; components {}; nogoods {}",
        t.size_r,
        j(&t.minimal),
        j(&t.maximal),
        j(&t.holes),
        t.components.len(),
        j(&t.nogoods)
    )
}

fn monotone(m: &Monotonicity) -> bool {
    m.is_monotone()
}

/// **Q-M1e2.7's first table, row by row.**
#[test]
fn the_seven_programs_have_the_tables_regions() {
    let rows: &[(&str, &str)] = &[
        (
            "examples/branching/15_lookahead_two_step_on.ein",
            "|R| 5; minimal ∅; maximal {(p A)}, {(p B), (p C)}; holes —; components 1; nogoods {(p A), (p B)}, {(p A), (p C)}",
        ),
        (
            "examples/lattice/02_genuine_3set_death.ein",
            "|R| 7; minimal ∅; maximal {(a-prop X), (b-prop X)}, {(a-prop X), (c-prop X)}, {(b-prop X), (c-prop X)}; holes —; components 1; nogoods {(a-prop X), (b-prop X), (c-prop X)}",
        ),
        (
            "examples/ein-bugs/wrong-answers/forced-2-two.ein",
            "|R| 1; minimal ∅; maximal ∅; holes —; components 1; nogoods {(q one)}, {(q two)}",
        ),
        (
            "examples/ein-bugs/wrong-answers/owed-two-step.ein",
            "|R| 0; minimal —; maximal —; holes —; components 0; nogoods {(likes Ann Soup)}, {(likes Ann Stew)}",
        ),
        (
            "examples/ein-bugs/wrong-answers/owed-seats.ein",
            "|R| 7; minimal {(seats Ann C1), (seats Bob C2)}, {(seats Ann C2), (seats Bob C1)}; maximal {(seats Ann C1), (seats Ann C2), (seats Bob C1), (seats Bob C2)}; holes —; components 1; nogoods —",
        ),
        (
            "examples/lattice/05_owed_seats_bounded.ein",
            "|R| 2; minimal {(seats Ann C1), (seats Bob C2)}, {(seats Ann C2), (seats Bob C1)}; maximal {(seats Ann C1), (seats Bob C2)}, {(seats Ann C2), (seats Bob C1)}; holes —; components 2; nogoods {(seats Ann C1), (seats Ann C2)}, {(seats Ann C1), (seats Bob C1)}, {(seats Ann C2), (seats Bob C2)}, {(seats Bob C1), (seats Bob C2)}",
        ),
        (
            "examples/ein-bugs/naf-upward-closure.ein",
            "|R| 3; minimal ∅; maximal {(p A), (q A)}; holes {(p A)}; components 1; nogoods {(p A)}",
        ),
    ];
    let mut wrong = Vec::new();
    for (rel, want) in rows {
        let (r, _) = region_of(rel);
        let got = summary(&r);
        if got != *want {
            wrong.push(format!("{rel}\n  want {want}\n  got  {got}"));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// The witness for *saturation alone*: through a solve, the forced positive
/// had already seated Bob and `{(seats Ann C1)}` owed nothing.
#[test]
fn a_commitment_owes_what_its_fixpoint_owes_and_nothing_the_search_wrote() {
    let (r, _) = region_of("examples/lattice/05_owed_seats_bounded.ein");
    let mask = (0..r.n())
        .find(|&i| r.candidates[i].text == "(seats Ann C1)")
        .map(|i| 1u64 << i)
        .expect("a candidate");
    assert_eq!(
        r.nodes[mask as usize].owes, 2,
        "{:?}",
        r.nodes[mask as usize].owed
    );
}

/// The owing bit is read on a dead node too — E needs it.
#[test]
fn a_dead_node_still_says_what_it_owes() {
    let (r, _) = region_of("examples/ein-bugs/wrong-answers/owed-two-step.ein");
    // ∅ owes Ann a food; every non-empty commitment is dead.
    assert_eq!(r.status[0], region::OWING);
    let both = (1u64 << r.n()) - 1;
    assert!(r.status[both as usize] & region::DEAD != 0);
    assert_eq!(
        r.nodes[both as usize].owes, 0,
        "a food is liked, so nothing is owed"
    );
}

/// Without holes the two lines are N's and E's; with W3's hole the N side
/// fails, which is the cross-check failing where it should.
#[test]
fn the_duality_holds_without_holes_and_fails_on_the_hole() {
    for rel in [
        "examples/branching/15_lookahead_two_step_on.ein",
        "examples/lattice/02_genuine_3set_death.ein",
        "examples/ein-bugs/wrong-answers/forced-2-two.ein",
        "examples/ein-bugs/wrong-answers/owed-two-step.ein",
        "examples/ein-bugs/wrong-answers/owed-seats.ein",
        "examples/lattice/05_owed_seats_bounded.ein",
    ] {
        let (r, _) = region_of(rel);
        let d = &r.topology.duality;
        assert!(r.topology.holes.is_empty(), "{rel}");
        assert!(d.minimal_holds && d.maximal_holds, "{rel}: {d:?}");
    }
    let (r, _) = region_of("examples/ein-bugs/naf-upward-closure.ein");
    assert!(!r.topology.duality.maximal_holds);
    assert_eq!(names(&r, &r.topology.duality.maximal_from_n), ["{(q A)}"]);
}

/// The maximal sub-cubes, Q-M1e2.7's second table.
#[test]
fn the_sub_cubes_are_the_second_tables() {
    let rows: &[(&str, &[(&str, &str)])] = &[
        (
            "examples/branching/15_lookahead_two_step_on.ein",
            &[("∅", "{(p A)}"), ("∅", "{(p B), (p C)}")],
        ),
        (
            "examples/ein-bugs/naf-upward-closure.ein",
            &[("∅", "{(q A)}"), ("{(q A)}", "{(p A), (q A)}")],
        ),
    ];
    for (rel, want) in rows {
        let (r, _) = region_of(rel);
        let mut got: Vec<(String, String)> = r
            .topology
            .subcubes
            .as_ref()
            .expect("computed")
            .iter()
            .map(|&(a, b)| (r.name(a), r.name(b)))
            .collect();
        got.sort();
        let mut want: Vec<(String, String)> = want
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        want.sort();
        assert_eq!(got, want, "{rel}");
    }
}

/// **The theorem, checked on the cube**: a program the static check calls
/// monotone has no observed violation of either upward closure. And the
/// three programs known to be non-monotone are flagged, each for its reason.
#[test]
fn monotone_programs_show_no_violation_and_the_non_monotone_three_are_flagged() {
    for rel in [
        "examples/branching/15_lookahead_two_step_on.ein",
        "examples/lattice/02_genuine_3set_death.ein",
        "examples/ein-bugs/wrong-answers/forced-2-two.ein",
        "examples/ein-bugs/wrong-answers/owed-two-step.ein",
        "examples/ein-bugs/wrong-answers/owed-seats.ein",
        "examples/lattice/05_owed_seats_bounded.ein",
    ] {
        let (r, t) = region_of(rel);
        let sites: Vec<String> = r.monotonicity.sites.iter().map(|s| s.render(&t)).collect();
        assert!(monotone(&r.monotonicity), "{rel}: {sites:?}");
        assert_eq!(r.topology.dead_not_upward_closed.count, 0, "{rel}");
        assert_eq!(r.topology.discharged_not_upward_closed.count, 0, "{rel}");
    }
    let flagged: &[(&str, SiteKind)] = &[
        ("examples/ein-bugs/naf-upward-closure.ein", SiteKind::Absent),
        (
            "examples/ein-bugs/wrong-answers/stale-naf-fork.ein",
            SiteKind::Absent,
        ),
        (
            "examples/lattice/04_obligation_trigger_hole.ein",
            SiteKind::ObligationTrigger,
        ),
    ];
    for (rel, kind) in flagged {
        let (r, t) = region_of(rel);
        let sites: Vec<String> = r.monotonicity.sites.iter().map(|s| s.render(&t)).collect();
        assert!(
            r.monotonicity.sites.iter().any(|s| s.kind == *kind),
            "{rel}: {sites:?}"
        );
    }
    // The trigger hole is a hole, and the cube sees the violation.
    let (r, _) = region_of("examples/lattice/04_obligation_trigger_hole.ein");
    assert_eq!(names(&r, &r.topology.holes), ["{(seats Ann C1)}"]);
    assert!(r.topology.discharged_not_upward_closed.count > 0);
}

// An activator compiling a **refutation** at a node (`written.rs` has the text
// and why): its one site is the activator, and a check that runs on root's
// plans cannot see the refutation. The text lives beside the censuses that
// read it too (M1e2 S1e2.1b.3), so a census and this test read one program.
use written::ACTIVATOR_REFUTES;

/// **One fixture per cause** — S1e2.1b.6, the comparison's *what causes it*
/// row. Each program's only site is of its cause, and its cube shows the hole
/// that cause makes: an `absent` that takes a witness away
/// (`lattice/08`), an obligation's trigger (`lattice/04`), and an activator
/// (`lattice/09`, the same cube as `lattice/04` by another mechanism). None
/// of the three concludes a refutation, so S1e2.4.1's *forbid* reaches none of
/// them. [`ACTIVATOR_REFUTES`] is the third cause carrying a refutation: a
/// dead hole, with the activator still the only site.
#[test]
fn each_cause_has_a_fixture_whose_cube_shows_its_hole() {
    let cases: &[(&str, SiteKind, &[&str])] = &[
        (
            "examples/lattice/08_absent_witness_hole.ein",
            SiteKind::Absent,
            &["{(allergic Ann), (seats Ann C1)}"],
        ),
        (
            "examples/lattice/04_obligation_trigger_hole.ein",
            SiteKind::ObligationTrigger,
            &["{(seats Ann C1)}"],
        ),
        (
            "examples/lattice/09_activator_hole.ein",
            SiteKind::Activator,
            &["{(opens K1)}"],
        ),
    ];
    for &(rel, kind, holes) in cases {
        let (r, t) = region_of(rel);
        let sites: Vec<String> = r.monotonicity.sites.iter().map(|s| s.render(&t)).collect();
        assert!(
            !sites.is_empty() && r.monotonicity.sites.iter().all(|s| s.kind == kind),
            "{rel}: {sites:?}"
        );
        assert_eq!(names(&r, &r.topology.holes), holes, "{rel}");
        assert_eq!(r.topology.dead_not_upward_closed.count, 0, "{rel}");
        assert!(r.topology.discharged_not_upward_closed.count > 0, "{rel}");
    }

    let (mut kb, mut terms, ast) = loaded_text("activator-refutes", ACTIVATOR_REFUTES);
    // Refused since M1e2 S1e2.2.5 (F3's read of a rule with parameters), and
    // studied here for the hole it would have.
    let Outcome::Region(r) = forbid::studying(|| {
        region::compute(&mut kb, &mut terms, &ast, &Options::default(), &mut |_| {})
    })
    .expect("computes") else {
        panic!("activator-refutes: within the limit");
    };
    let kinds: Vec<SiteKind> = r.monotonicity.sites.iter().map(|s| s.kind).collect();
    assert_eq!(kinds, [SiteKind::Activator]);
    assert_eq!(names(&r, &r.topology.holes), ["{(p A)}"]);
    assert_eq!(r.nodes[1].refuted_by, ["watch"]);
}

/// **Across a hole the one-fact test misreads the lines** — S1e2.1b.6, the
/// comparison's *lines* row. On a monotone cube a node of R is maximal exactly
/// when every edge up leaves R, and the corpus never disagrees. On
/// `synth/local_not_global`, *A and B together or not at all*, ∅ and {A,B}
/// are in R with a hole on each side: the one-fact test calls ∅ maximal and
/// {A,B} minimal, and the cube says neither is.
#[test]
fn across_a_hole_the_one_fact_test_misreads_the_lines() {
    let (r, _) = region_of("examples/lattice/synth/local_not_global.ein");
    assert!(!r.monotonicity.is_monotone());
    assert_eq!(names(&r, &r.topology.holes), ["{(h A)}", "{(h B)}"]);
    assert_eq!(names(&r, &r.topology.maximal), ["{(h A), (h B)}"]);
    assert_eq!(names(&r, &r.topology.local_max), ["∅"]);
    assert_eq!(names(&r, &r.topology.local_min), ["{(h A), (h B)}"]);
}

// ── Fact grain — S1e2.1b.1 ──────────────────────────────────────────

/// Programs written to test the fact-grain prototype, each with what it is
/// for: `(name, program, leaves the set at fact grain, its closure complete,
/// its cube violated)`. Every one is flagged at relation grain. The corpus
/// has no program within the oracle's limit that leaves the set, so without
/// these the claim below would be vacuous where it matters.
const FACT_GRAIN_PROGRAMS: &[(&str, &str, bool, bool, bool)] = &[
    // A derived fact matches the read: `tag` writes `(q A Tagged)` from a
    // candidate, so `bad`'s absence can be filled.
    (
        "derived",
        "(relation p T :why \"p holds of {?1}\") (relation s T :why \"s holds of {?1}\") (relation q T T :why \"q holds of {?1} and {?2}\") (relation r T :why \"r holds of {?1}\") (r A)
         (hrule g1 () :match (r ?x) :assert (p ?x) :why \"r holds of {?x}, so guess that p holds of {?x}\")
         (hrule g2 () :match (r ?x) :assert (s ?x) :why \"r holds of {?x}, so guess that s holds of {?x}\")
         (rule tag () :match (s ?x) :assert (q ?x Tagged) :why \"s holds of {?x}, so q holds of {?x} and Tagged\")
         (rule bad () :match (and (p ?x) (absent (q ?x Tagged))) :assert (false) :why \"p holds of {?x} and q does not hold of {?x} and Tagged, which cannot be\")",
        false,
        true,
        true,
    ),
    // The same, reading `(q ?x Other)`, which nothing a commitment adds has.
    (
        "static",
        "(relation p T :why \"p holds of {?1}\") (relation s T :why \"s holds of {?1}\") (relation q T T :why \"q holds of {?1} and {?2}\") (relation r T :why \"r holds of {?1}\") (r A)
         (hrule g1 () :match (r ?x) :assert (p ?x) :why \"r holds of {?x}, so guess that p holds of {?x}\")
         (hrule g2 () :match (r ?x) :assert (s ?x) :why \"r holds of {?x}, so guess that s holds of {?x}\")
         (rule tag () :match (s ?x) :assert (q ?x Tagged) :why \"s holds of {?x}, so q holds of {?x} and Tagged\")
         (rule bad () :match (and (p ?x) (absent (q ?x Other))) :assert (false) :why \"p holds of {?x} and q does not hold of {?x} and Other, which cannot be\")",
        true,
        true,
        false,
    ),
    // `done` is derived through a read at depth 2, a `forall` written out:
    // a closure that followed only top-level reads would call `bad`'s
    // `(absent (done W))` static, and the cube shows the hole it makes.
    (
        "even-depth",
        "(relation p T :why \"p holds of {?1}\") (relation s T :why \"s holds of {?1}\") (relation done T :why \"{?1} is done\") (relation world T :why \"{?1} is a world\")
         (relation slot T :why \"{?1} is a slot\") (world W) (s A) (slot A) (slot B)
         (hrule g () :match (slot ?x) :assert (p ?x) :why \"{?x} is a slot, so guess that p holds of {?x}\")
         (rule all-p () :match (and (world ?w) (absent (and (s ?x) (absent (p ?x)))))
                        :assert (done ?w) :why \"p holds of everything s holds of, so world {?w} is done\")
         (rule bad () :match (and (p B) (absent (done W))) :assert (false) :why \"p holds of B while W is not done, which cannot be\")",
        false,
        true,
        true,
    ),
    // A stored negative a commitment derives fills the read.
    (
        "negative",
        "(relation p T :why \"p holds of {?1}\") (relation t T :why \"t holds of {?1}\") (relation q T :why \"q holds of {?1}\") (relation r T :why \"r holds of {?1}\") (r A)
         (hrule g1 () :match (r ?x) :assert (p ?x) :why \"r holds of {?x}, so guess that p holds of {?x}\")
         (hrule g2 () :match (r ?x) :assert (t ?x) :why \"r holds of {?x}, so guess that t holds of {?x}\")
         (rule neg () :match (p ?x) :assert (not (q ?x)) :why \"p holds of {?x}, so q does not hold of {?x}\")
         (rule bad () :match (and (t ?x) (absent (not (q ?x)))) :assert (false) :why \"t holds of {?x} and no stored fact says q does not hold of {?x}, which cannot be\")",
        false,
        true,
        true,
    ),
    // A commitment derives an activator, `(watch q)`: the site stays, and
    // the closure says it cannot see the plan that would compile.
    (
        "activator",
        "(relation p T :why \"p holds of {?1}\") (relation t T :why \"t holds of {?1}\") (relation q T :why \"q holds of {?1}\") (relation r T :why \"r holds of {?1}\") (r A)
         (hrule g1 () :match (r ?x) :assert (p ?x) :why \"r holds of {?x}, so guess that p holds of {?x}\")
         (hrule g2 () :match (r ?x) :assert (t ?x) :why \"r holds of {?x}, so guess that t holds of {?x}\")
         (rule watch (?R) :match (and (t ?x) (absent (?R ?x))) :assert (false) :why \"t holds of {?x} and {?R} does not, which cannot be\")
         (rule act () :match (p ?x) :assert (watch q) :why \"p holds of {?x}, so rule watch is switched on for q\")",
        false,
        false,
        false,
    ),
    // `features/05`'s shape within the limit: membership between objects is
    // proposed, and the read asks for membership in a type.
    (
        "type-constant",
        "(relation is-a T T :why \"{?1} is a {?2}\") (relation owns T T :why \"{?1} owns {?2}\")
         (is-a A Thing) (is-a B Thing) (is-a C Box) (owns A C)
         (hrule g () :match (and (is-a ?x Thing) (is-a ?y Thing) (neq ?x ?y))
                     :assert (is-a ?x ?y) :why \"{?x} and {?y} are two different Things, so guess that {?x} is a {?y}\")
         (rule check () :match (and (owns ?x ?y) (absent (is-a ?x Thing))) :assert (false) :why \"{?x} owns {?y} but is not a Thing, which cannot be\")",
        true,
        true,
        false,
    ),
];

fn loaded_text(name: &str, text: &str) -> (Kb, Terms, Ast) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, Some(name)).unwrap_or_else(|e| panic!("{name}: {e}"));
    let kb = load(&mut ast, &mut terms, &forms, None).unwrap_or_else(|e| panic!("{name}: {e}"));
    (kb, terms, ast)
}

/// The fact-grain verdict on one load: `(relation grain flags it, fact grain
/// calls it monotone, its closure is complete, its closure gave up)`.
fn fact_grain_of(kb: &mut Kb, terms: &mut Terms, ast: &Ast) -> Option<(bool, bool, bool, bool)> {
    let (root, ()) = census::root_of(kb, terms, ast, |_, _| Some(()))?;
    let mut cands = root.proposed.clone();
    cands.sort_by_key(|&f| ein_infer::events::sexpr(terms, f));
    cands.dedup();
    let fg = fact_grain::classify(kb, terms, &root.plans, &cands, &root.monotonicity);
    Some((
        !root.monotonicity.is_monotone(),
        fg.is_monotone(),
        fg.complete,
        fg.gave_up,
    ))
}

fn violated(r: &Region) -> bool {
    let t = &r.topology;
    !t.holes.is_empty()
        || t.dead_not_upward_closed.count > 0
        || t.discharged_not_upward_closed.count > 0
}

/// **The fact-grain prototype against the cube** — S1e2.1b.1 T1e2.1b.1.2.
/// `examples/shared/fact_grain.rs` reads a site at fact grain: dynamic only
/// when a candidate, or a fact a saturation plan derives from candidates, can
/// match its pattern. Its claim is the theorem at that grain: **on no cube the
/// oracle enumerates is its verdict monotone while the cube observes a
/// violation** (a hole, a dead node with an alive superset, a discharged node
/// with an owing superset). Checked on every corpus file and register witness
/// within the limit, and on [`FACT_GRAIN_PROGRAMS`]. No route reads the
/// prototype; the shipped check reads at relation grain until the user rules.
///
/// **The closure runs only where a site reads it**, and a program on which
/// it gives up fails here (M1f S1f.6.28, E-33): its verdict would be the safe
/// one, *not monotone*, and this test reads no cube of such a program, so a
/// give-up would narrow what the test checks without saying so.
#[test]
fn fact_grain_never_calls_a_violated_cube_monotone() {
    let oracle = Options {
        jobs: std::thread::available_parallelism().map_or(1, |n| n.get().min(8)),
        ..Options::default()
    };
    let mut failures: Vec<String> = Vec::new();
    let mut cubes = 0usize;
    let mut check = |name: &str, load: &dyn Fn() -> Option<(Kb, Terms, Ast)>| {
        let Some((mut kb, mut terms, ast)) = load() else {
            return;
        };
        let Some((flagged, monotone, _, gave_up)) = fact_grain_of(&mut kb, &mut terms, &ast) else {
            return;
        };
        if gave_up {
            failures.push(format!(
                "{name}: the fact-grain closure gave up past {} shapes (E-33), so its verdict \
                 is the safe one and no cube of it is read here",
                fact_grain::MAX_SHAPES
            ));
            return;
        }
        if !monotone {
            return;
        }
        let (mut kb, mut terms, ast) = load().expect("loaded once already");
        let Ok(Outcome::Region(r)) =
            region::compute(&mut kb, &mut terms, &ast, &oracle, &mut |_| {})
        else {
            return;
        };
        cubes += 1;
        // The shipped check reads at relation grain, which flags a superset
        // of fact grain's programs, so every cube it calls monotone is read
        // here too: **a Monotone verdict implies no cube violation** — M1e2
        // S1e2.1b.5's test (c) 1, on every corpus file and register witness
        // within the limit (`region_synth.py --random`'s check holds it on
        // random programs).
        if !flagged && violated(&r) {
            failures.push(format!(
                "{name}: the static check says monotone, and the cube has a violation"
            ));
        }
        if violated(&r) {
            let t = &r.topology;
            failures.push(format!(
                "{name}: fact grain says monotone, and the cube has {} holes, {} dead nodes with an \
                 alive superset, {} discharged nodes with an owing superset",
                t.holes.len(),
                t.dead_not_upward_closed.count,
                t.discharged_not_upward_closed.count
            ));
        }
    };
    for path in corpus_and_register() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        check(&rel, &|| loaded(&path));
    }
    for (name, text, ..) in FACT_GRAIN_PROGRAMS {
        check(name, &|| Some(loaded_text(name, text)));
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    // 146 monotone cubes on 2026-10-16, all of them monotone at fact grain
    // too. A sweep that silently found none would pass the loop above.
    assert!(cubes >= 140, "{cubes} cubes checked");
}

/// The programs written for the prototype get the verdicts they were written
/// for, and their cubes show what each was written to show — so the claim
/// above is not vacuous on a program that leaves the set.
#[test]
fn fact_grain_on_programs_written_to_test_it() {
    for &(name, text, leaves, complete, want_violated) in FACT_GRAIN_PROGRAMS {
        let (mut kb, mut terms, ast) = loaded_text(name, text);
        let (flagged, monotone, whole, gave_up) =
            fact_grain_of(&mut kb, &mut terms, &ast).expect("saturates");
        assert!(!gave_up, "{name}: the closure ends");
        assert!(flagged, "{name}: relation grain flags it");
        assert_eq!(monotone, leaves, "{name}: leaves the set at fact grain");
        assert_eq!(whole, complete, "{name}: the closure is complete");
        let (mut kb, mut terms, ast) = loaded_text(name, text);
        let Outcome::Region(r) = forbid::studying(|| {
            region::compute(&mut kb, &mut terms, &ast, &Options::default(), &mut |_| {})
        })
        .expect("computes") else {
            panic!("{name}: within the limit");
        };
        assert_eq!(
            violated(&r),
            want_violated,
            "{name}: what the cube observes"
        );
    }
}

/// **Where the closure runs, and where it gives up** (M1f S1f.6.28, E-33).
/// Two programs on which a supposed fact concludes a request to `std.math.decimal-arith`
/// whose dividend the commitment does not bind, `(div _ (n z 3))`, which sends
/// the closure through every shape of the division's circuits. Without a site
/// the verdict reads nothing of `Δ̂`, which is not computed, and the cube is
/// read as on every unflagged program. With one, an `absent` over the
/// quotient, the closure runs and gives up at [`fact_grain::MAX_SHAPES`]: every
/// site is one, the verdict the safe *not monotone*.
#[test]
fn fact_grain_closure_runs_where_a_site_reads_it_and_gives_up_on_the_safe_side() {
    const READING: &str = "(import std.math.decimal-arith :symbols (decimal-arith div n quot-of))
         (relation reading T :why \"{?1} is a reading\") (relation read-as T :why \"read as {?1}\")
         (relation amount T T :why \"there are {?2} {?1}\") (relation answer T T :why \"the answer for {?1} is {?2}\")
         (relation counted T :why \"{?1} is counted\")
         (reading one) (amount apples (n (n z 1) 3))
         (hrule suppose () :match (reading ?m) :assert (read-as ?m) :why \"suppose the reading {?m}\")
         (rule under () :match (and (read-as ?m) (amount ?k ?v)) :assert (answer ?k ?v) :why \"under {?m}, the answer for {?k} is {?v}\")
         (rule divide () :match (answer ?k ?v) :assert (div ?v (n z 3)) :why \"divide {?v} by 3\")";
    const SITE: &str = "(rule site () :match (and (reading ?m) (absent (quot-of ?a ?b ?q ?r))) :assert (counted ?m)
         :why \"{?m} is counted while nothing is divided\")";
    let verdict = |text: &str| {
        let (mut kb, mut terms, ast) = loaded_text("reading", text);
        let (root, ()) =
            census::root_of(&mut kb, &mut terms, &ast, |_, _| Some(())).expect("saturates");
        let mut cands = root.proposed.clone();
        cands.sort_by_key(|&f| ein_infer::events::sexpr(&terms, f));
        cands.dedup();
        assert_eq!(cands.len(), 1, "one reading is supposed");
        let fg = fact_grain::classify(&kb, &terms, &root.plans, &cands, &root.monotonicity);
        (root.monotonicity.is_monotone(), fg)
    };

    let (unflagged, fg) = verdict(READING);
    assert!(unflagged, "no site at relation grain");
    assert!(
        fg.shapes.is_none(),
        "no site reads Δ̂, so it is not computed"
    );
    assert!(fg.is_monotone() && fg.complete && !fg.gave_up);
    let (mut kb, mut terms, ast) = loaded_text("reading", READING);
    let Outcome::Region(r) =
        region::compute(&mut kb, &mut terms, &ast, &Options::default(), &mut |_| {})
            .expect("computes")
    else {
        panic!("within the limit");
    };
    assert!(
        !violated(&r),
        "the cube of the unflagged program has no violation"
    );

    let flagged_text = format!("{READING}\n{SITE}");
    let (unflagged, fg) = verdict(&flagged_text);
    assert!(!unflagged, "the absent over quot-of is a site");
    assert!(fg.gave_up, "the closure gives up");
    assert!(fg.shapes.is_none());
    assert_eq!(
        fg.sites,
        vec![Some("(quot-of _ _ _ _)".to_string())],
        "every site is one, its witness the relation it reads"
    );
    assert!(!fg.is_monotone(), "the safe verdict");
    assert!(fg.complete, "no activator site");
}

/// **`--jobs` changes nothing.** The same record with one thread and with
/// four, node for node and reason for reason — including the core, whose
/// choice depends on interner order (W11), because a worker never numbers a
/// fact and every new number is the calling thread's, in layer order.
/// **Saturation is a move inside the cube** (S1e2.1a.2). Every node's
/// closure — the candidates its fixpoint holds — contains its own mask, and an
/// alive node and its closure are one state: committing to what saturation
/// derived changes nothing. The second is what lets `ein render region` draw
/// `C ⇒ cl(C)` as an arrow rather than as a node of its own; it holds on every
/// corpus cube (`census_for_inspection` counts the exceptions), and these two
/// are the programs where it is not vacuous at n ≤ 9.
#[test]
fn a_node_and_its_closure_are_one_state() {
    for rel in [
        "examples/saturation/transitive/taxonomy.ein",
        "examples/branching/05_mini_zebra.ein",
    ] {
        let (r, _) = region_of(rel);
        let mut moved = 0;
        for (m, node) in r.nodes.iter().enumerate() {
            let m = m as u64;
            assert_eq!(node.closure & m, m, "{rel}: {} ⊄ its closure", r.name(m));
            assert_eq!(node.excluded & node.closure & !m, 0, "{rel}: {}", r.name(m));
            if r.status[m as usize] & region::DEAD != 0 || node.closure == m {
                continue;
            }
            moved += 1;
            assert_eq!(
                r.nodes[node.closure as usize].class,
                node.class,
                "{rel}: {} and {} are different states",
                r.name(m),
                r.name(node.closure)
            );
        }
        assert!(moved > 0, "{rel}: nothing moved");
    }
}

/// Where `cl` breaks a closure-operator law on one cube, as lines: `C ⊆
/// cl(C)` (extensive), `cl(cl(C)) = cl(C)` (idempotent), and `C ⊆ D ⇒ cl(C)
/// ⊆ cl(D)` (monotone), the last on every edge `C → C + h`, which implies it
/// for every pair. Over every node, dead ones included: the oracle runs a dead
/// node's saturation to quiescence too.
fn closure_operator_breaks(r: &Region) -> Vec<String> {
    let cl = |m: u64| r.nodes[m as usize].closure;
    let mut bad = Vec::new();
    for m in 0..r.nodes.len() as u64 {
        if cl(m) & m != m {
            bad.push(format!("not extensive at {}", r.name(m)));
        }
        if cl(cl(m)) != cl(m) {
            bad.push(format!(
                "not idempotent at {}: cl = {}, cl(cl) = {}",
                r.name(m),
                r.name(cl(m)),
                r.name(cl(cl(m)))
            ));
        }
        for b in 0..r.n() {
            let up = m | 1 << b;
            if up != m && cl(m) & cl(up) != cl(m) {
                bad.push(format!(
                    "not monotone on {} ⊂ {}: cl = {} ⊄ {}",
                    r.name(m),
                    r.name(up),
                    r.name(cl(m)),
                    r.name(cl(up))
                ));
            }
        }
    }
    bad
}

/// A program the static check flags on which `cl` is **not** monotone, so the
/// law below is the verdict's and not the oracle's: `{(p A)}` derives `(s A)`
/// while `(q A)` is absent, and `{(p A), (q A)}` does not.
const CL_NOT_MONOTONE: &str = "(relation p T :why \"p holds of {?1}\") (relation q T :why \"q holds of {?1}\") (relation s T :why \"s holds of {?1}\") (relation r T :why \"r holds of {?1}\") (r A)
     (hrule g1 () :match (r ?x) :assert (p ?x) :why \"r holds of {?x}, so guess that p holds of {?x}\")
     (hrule g2 () :match (r ?x) :assert (q ?x) :why \"r holds of {?x}, so guess that q holds of {?x}\")
     (hrule g3 () :match (r ?x) :assert (s ?x) :why \"r holds of {?x}, so guess that s holds of {?x}\")
     (rule derive-s () :match (and (p ?x) (absent (q ?x))) :assert (s ?x) :why \"p holds of {?x} and q does not, so s holds of {?x}\")";

/// **A *Monotone* verdict makes saturation a closure operator on the cube** —
/// S1e2.1b.6, the comparison's *saturation on the cube* row. `cl(C)` is the
/// candidates `C`'s fixpoint holds (`Node::closure`). On every corpus and
/// register cube within the limit that the static check calls monotone, `cl`
/// is extensive, idempotent and monotone on every node (`closure_operator_breaks`).
/// That is what makes R's states the closed sets of a closure system, each
/// with its minimal generators. The law is not vacuous: saturation moves a
/// node on 15 of these cubes. And it is the verdict's: on
/// [`CL_NOT_MONOTONE`], which the check flags, `cl` is not monotone.
#[test]
fn a_monotone_verdict_makes_saturation_a_closure_operator() {
    let oracle = Options {
        jobs: std::thread::available_parallelism().map_or(1, |n| n.get().min(8)),
        ..Options::default()
    };
    let mut failures: Vec<String> = Vec::new();
    let (mut cubes, mut moving) = (0usize, 0usize);
    for path in corpus_and_register() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        let Some((mut kb, mut terms, ast)) = loaded(&path) else {
            continue;
        };
        let Ok(Outcome::Region(r)) =
            region::compute(&mut kb, &mut terms, &ast, &oracle, &mut |_| {})
        else {
            continue;
        };
        if !r.monotonicity.is_monotone() {
            continue;
        }
        cubes += 1;
        if (0..r.nodes.len() as u64).any(|m| r.nodes[m as usize].closure != m) {
            moving += 1;
        }
        failures.extend(
            closure_operator_breaks(&r)
                .into_iter()
                .take(3)
                .map(|b| format!("{rel}: {b}")),
        );
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    eprintln!("{cubes} monotone cubes; saturation moves a node on {moving}");
    // 172 monotone cubes in a debug build on 2026-11-04, saturation moving a
    // node on 16 of them; 153 and 15 on 2026-11-02, when a debug build
    // skipped W7's two witnesses, before M1e2 S1e2.2.7 fixed W7 and the
    // stages since banked their witnesses. A sweep that silently found none
    // would pass.
    assert!(cubes >= 150, "{cubes} monotone cubes checked");
    assert!(moving >= 10, "saturation moves a node on {moving} of them");

    let (mut kb, mut terms, ast) = loaded_text("cl-not-monotone", CL_NOT_MONOTONE);
    let Outcome::Region(r) =
        region::compute(&mut kb, &mut terms, &ast, &Options::default(), &mut |_| {})
            .expect("computes")
    else {
        panic!("cl-not-monotone: within the limit");
    };
    assert!(!r.monotonicity.is_monotone(), "the check flags it");
    let breaks = closure_operator_breaks(&r);
    assert!(
        breaks.iter().any(|b| b.starts_with("not monotone")),
        "cl-not-monotone: {breaks:?}"
    );
}

/// **The opposite direction** (S1e2.1a.8). Every program under
/// `examples/lattice/synth/` was written by `utils/region_synth.py` for a
/// region it carries in its header (`;;; region-status:`, one class per node
/// in mask order). The oracle has to find exactly that region, node for node,
/// and the static check has to call the program monotone exactly when the
/// target is: no dead node below an alive one, no owing node above one in R.
#[test]
fn every_synthetic_program_has_the_region_it_was_written_for() {
    let dir = repo_root().join("examples/lattice/synth");
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("examples/lattice/synth")
        .map(|e| e.expect("entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "ein"))
        .collect();
    files.sort();
    assert!(files.len() >= 5, "{} synthetic programs", files.len());
    for path in files {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap()
            .to_string_lossy()
            .to_string();
        let text = std::fs::read_to_string(&path).unwrap();
        let want: Vec<u8> = text
            .lines()
            .find_map(|l| l.strip_prefix(";;; region-status: "))
            .unwrap_or_else(|| panic!("{rel}: no region-status header"))
            .trim()
            .bytes()
            .collect();
        let (r, _) = region_of(&rel);
        assert_eq!(want.len(), r.status.len(), "{rel}: the header's cube");
        let got: Vec<u8> = r
            .status
            .iter()
            .map(|&s| {
                if s & region::DEAD != 0 {
                    b'x'
                } else if s & region::OWING != 0 {
                    b'o'
                } else {
                    b'R'
                }
            })
            .collect();
        let differ: Vec<String> = (0..want.len())
            .filter(|&m| want[m] != got[m])
            .map(|m| {
                format!(
                    "{}: want {}, oracle {}",
                    r.name(m as u64),
                    want[m] as char,
                    got[m] as char
                )
            })
            .collect();
        assert!(differ.is_empty(), "{rel}:\n  {}", differ.join("\n  "));
        let n = r.n();
        let monotone = (0..want.len()).all(|m| {
            (0..n).all(|b| {
                let up = m | 1 << b;
                (want[m] != b'x' || want[up] == b'x') && (want[m] != b'R' || want[up] != b'o')
            })
        });
        assert_eq!(
            monotone,
            r.monotonicity.is_monotone(),
            "{rel}: the target is{} monotone",
            if monotone { "" } else { " not" }
        );
    }
}

#[test]
fn jobs_change_nothing() {
    for rel in [
        "examples/branching/05_mini_zebra.ein",
        "examples/saturation/type-exclusivity/pets.ein",
        "examples/ein-bugs/naf-upward-closure.ein",
        "examples/lattice/05_owed_seats_bounded.ein",
    ] {
        let run = |jobs: usize| {
            let path = repo_root().join(rel);
            let text = std::fs::read_to_string(&path).unwrap();
            let mut ast = Ast::new();
            let mut terms = Terms::new();
            let forms = parse(&mut ast, &text, path.to_str()).unwrap();
            let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).unwrap();
            let opts = Options {
                jobs,
                keep_states: true,
                ..Options::default()
            };
            let out =
                forbid::studying(|| region::compute(&mut kb, &mut terms, &ast, &opts, &mut |_| {}));
            match out.unwrap() {
                Outcome::Region(r) => *r,
                Outcome::Refused { n, .. } => panic!("{rel}: n = {n}"),
            }
        };
        let (a, b) = (run(1), run(4));
        assert_eq!(a.status, b.status, "{rel}: status");
        assert_eq!(a.states, b.states, "{rel}: states");
        for (m, (x, y)) in a.nodes.iter().zip(&b.nodes).enumerate() {
            assert_eq!(
                (&x.owed, &x.core, &x.refuted_by, &x.absent_premises, x.class),
                (&y.owed, &y.core, &y.refuted_by, &y.absent_premises, y.class),
                "{rel}: node {m}"
            );
        }
        assert_eq!(a.topology.minimal, b.topology.minimal, "{rel}");
        assert_eq!(a.topology.maximal, b.topology.maximal, "{rel}");
    }
}

/// zebra2's `absent`s read the position structure, which no commitment
/// changes — static, so the static check calls it monotone even though the
/// cube is far past the limit.
#[test]
fn zebra2_is_monotone_by_construction() {
    let (out, terms) = compute("examples/zebra2.ein");
    let Outcome::Refused {
        n, monotonicity, ..
    } = out
    else {
        panic!("zebra2's cube is not enumerable");
    };
    let sites: Vec<String> = monotonicity
        .sites
        .iter()
        .map(|s| s.render(&terms))
        .collect();
    assert!(n > 16);
    assert!(monotonicity.is_monotone(), "{sites:#?}");
}

#[test]
fn print_for_inspection() {
    if std::env::var_os("EIN_REGION_PRINT").is_none() {
        return;
    }
    for rel in [
        "examples/branching/15_lookahead_two_step_on.ein",
        "examples/lattice/02_genuine_3set_death.ein",
        "examples/ein-bugs/wrong-answers/forced-2-two.ein",
        "examples/ein-bugs/wrong-answers/owed-two-step.ein",
        "examples/ein-bugs/wrong-answers/owed-seats.ein",
        "examples/lattice/05_owed_seats_bounded.ein",
        "examples/ein-bugs/naf-upward-closure.ein",
        "examples/ein-bugs/wrong-answers/stale-naf-fork.ein",
        "examples/lattice/04_obligation_trigger_hole.ein",
    ] {
        let (r, t) = region_of(rel);
        eprintln!("{rel}\n  {}", summary(&r));
        for s in &r.monotonicity.sites {
            eprintln!("  site: {}", s.render(&t));
        }
        eprintln!("  cost {:?}", r.cost);
        let _ = Path::new(rel);
    }
}

/// The corpus census behind `region_dump.md § Monotonicity`'s numbers — off
/// unless `EIN_REGION_CENSUS` is set, because it is a measurement and not a
/// claim. Re-take: `EIN_REGION_CENSUS=1 cargo test --release -p ein-infer
/// --test region census_for_inspection -- --nocapture`.
#[test]
fn census_for_inspection() {
    if std::env::var_os("EIN_REGION_CENSUS").is_none() {
        return;
    }
    let mut total = 0;
    let mut enumerable = 0;
    let mut flagged: Vec<String> = Vec::new();
    let mut by_kind: std::collections::BTreeMap<(String, String), usize> = Default::default();
    let mut costs: Vec<(f64, usize, String)> = Vec::new();
    let mut ns: Vec<usize> = Vec::new();
    // Alive nodes saturation moves, and those whose closure is another state.
    let mut moves = 0usize;
    let mut move_breaks: Vec<String> = Vec::new();
    let t_all = std::time::Instant::now();
    for path in ein_corpus::corpus_files() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap()
            .to_string_lossy()
            .to_string();
        let text = std::fs::read_to_string(&path).unwrap();
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Ok(forms) = parse(&mut ast, &text, path.to_str()) else {
            continue;
        };
        let Ok(mut kb) = load(&mut ast, &mut terms, &forms, path.parent()) else {
            continue;
        };
        let t = std::time::Instant::now();
        let Ok(out) = region::compute(&mut kb, &mut terms, &ast, &Options::default(), &mut |_| {})
        else {
            continue;
        };
        total += 1;
        let (m, n, holes) = match &out {
            Outcome::Region(r) => {
                enumerable += 1;
                costs.push((t.elapsed().as_secs_f64(), r.n(), rel.clone()));
                for (m, node) in r.nodes.iter().enumerate() {
                    if r.status[m] & region::DEAD != 0 || node.closure == m as u64 {
                        continue;
                    }
                    moves += 1;
                    if r.nodes[node.closure as usize].class != node.class {
                        move_breaks.push(format!("{rel}: {}", r.name(m as u64)));
                    }
                }
                (&r.monotonicity, r.n(), r.topology.holes.len())
            }
            Outcome::Refused {
                n, monotonicity, ..
            } => (monotonicity, *n, 0),
        };
        ns.push(n);
        if !m.is_monotone() {
            flagged.push(format!(
                "{rel}: n={n} sites={} holes={holes}",
                m.sites.len()
            ));
            for s in &m.sites {
                *by_kind
                    .entry((terms.sym(s.rule).to_string(), s.kind.as_str().to_string()))
                    .or_default() += 1;
            }
        }
    }
    eprintln!(
        "alive nodes saturation moves: {moves}; whose closure is another state: {}",
        move_breaks.len()
    );
    for b in move_breaks.iter().take(20) {
        eprintln!("  {b}");
    }
    costs.sort_by(|a, b| b.0.total_cmp(&a.0));
    eprintln!(
        "loaded and saturated {total}; enumerable at n <= 16: {enumerable}; flagged {}",
        flagged.len()
    );
    for f in &flagged {
        eprintln!("  {f}");
    }
    eprintln!("sites by (rule, kind):");
    for ((r, k), c) in &by_kind {
        eprintln!("  {r:<24} {k:<20} {c}");
    }
    eprintln!("slowest enumerable:");
    for (secs, n, rel) in costs.iter().take(6) {
        eprintln!("  {secs:8.3} s  n={n:<3} {rel}");
    }
    let sum: f64 = costs.iter().map(|c| c.0).sum();
    eprintln!(
        "all enumerable: {sum:.2} s; whole census {:.2} s",
        t_all.elapsed().as_secs_f64()
    );
}

// ── A refutation's taint — S1e2.1b.4 ────────────────────────────────

/// Every dead node of `r`, as `(mask, untainted, a proper superset is
/// alive)`, its taint read by `walk` off the node's own fixpoint, a premise
/// tainted when its relation's label is in `extendable`. `kb` is the load `r`
/// was computed on.
fn deaths(
    kb: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    r: &Region,
    extendable: &[String],
    walk: taint::Walk,
) -> Vec<(u64, bool, bool)> {
    let root: rustc_hash::FxHashSet<ein_core::FactId> = kb.facts().collect();
    let d = taint::Dynamic::of(extendable.iter().cloned());
    let size = r.status.len() as u64;
    let alive_above = |m: u64| {
        (0..size).any(|x| x != m && x & m == m && r.status[x as usize] & region::DEAD == 0)
    };
    // The deaths of a program *forbid* refuses are what this reads (M1e2
    // S1e2.2.5): studied, past the refusal.
    let mut nodes =
        forbid::studying(|| region::Nodes::new(kb, terms, ast, &r.candidates)).expect("prepares");
    let mut out = Vec::new();
    for m in 0..size {
        if r.status[m as usize] & region::DEAD == 0 {
            continue;
        }
        let facts: Vec<_> = (0..r.n())
            .filter(|b| m >> b & 1 == 1)
            .map(|b| r.candidates[b].fact)
            .collect();
        let untainted = nodes
            .with_fixpoint(&facts, |kb, terms| {
                taint::of(kb, terms, &root, &d, walk).untainted()
            })
            .expect("a node");
        out.push((m, untainted, alive_above(m)));
    }
    out
}

/// **Option (b) of F21's D6, held to the oracle** — S1e2.1b.4. (b) would let
/// a route learn from a dead node whose refutation rests on no `absent` over
/// `D`, so it is sound only if every such death is upward-closed: **no
/// untainted dead node has an alive superset**, and so every dead hole is
/// tainted. Checked on every flagged corpus and register cube within the
/// limit and on the programs in `written.rs`, with the taint read by
/// `examples/shared/taint.rs`: every contradiction, both sides of a pair, and
/// the activator of each plan a fact was derived by.
///
/// **`D` is read as *forbid* reads it** since M1e2 S1e2.2.17: `D⁺`, the
/// static check's `D` followed through the rules root has no plan of
/// (`forbid::extendable`). Read as `D`, W17's `forbid-chain-node-writes` is a
/// counterexample: its death rests on `(absent (x a))`, `x` outside `D`, and
/// a plan only a node compiles writes `x` at the alive superset.
///
/// The activator half is not optional. On `ACTIVATOR_UNDER_ABSENT` the dead
/// hole `{(p A), (r A)}` is refuted by a plan compiled from an activator `act`
/// derived under `(absent (q ?x))`, and a derivation does not record its
/// plan's activator: read off provenance alone, the death is untainted and
/// `{(p A), (q A), (r A)}` above it is alive. The test pins that too, so it
/// fails if the provenance-only reading stops being the counterexample it is.
#[test]
fn an_untainted_death_is_dead_in_every_superset() {
    let oracle = Options {
        jobs: std::thread::available_parallelism().map_or(1, |n| n.get().min(8)),
        ..Options::default()
    };
    let full = taint::Walk { activators: true };
    let mut failures: Vec<String> = Vec::new();
    let (mut cubes, mut dead, mut untainted) = (0usize, 0usize, 0usize);
    let mut check = |name: &str, load: &dyn Fn() -> Option<(Kb, Terms, Ast)>| {
        // The static check first, on its own load, so a monotone cube is
        // never enumerated; and what *forbid* calls extendable, `D⁺`, the
        // static check's `D` followed through the rules root has no plan of.
        // Until M1e2 S1e2.2.17 this read `D`, and W17's
        // `forbid-chain-node-writes` was a counterexample to that reading and
        // not to the claim: its dead hole {(p a)} rests on `(absent (x a))`,
        // and only a plan a node compiles writes `x`, at the alive superset.
        let Some((mut kb, mut terms, ast)) = load() else {
            return;
        };
        let extendable: Vec<String> =
            match census::root_of(&mut kb, &mut terms, &ast, |_, _| Some(())) {
                Some((root, ())) if !root.monotonicity.is_monotone() => {
                    forbid::extendable(&kb, &terms, &ast, &root.plans, &root.monotonicity)
                        .iter()
                        .map(|r| r.label(&terms))
                        .collect()
                }
                _ => return,
            };
        let (mut kb, mut terms, ast) = load().expect("loaded once already");
        // A study of the shape: the programs *forbid* refuses are read too
        // (M1e2 S1e2.2.5), since every dead hole is on one of them.
        let Ok(Outcome::Region(r)) =
            forbid::studying(|| region::compute(&mut kb, &mut terms, &ast, &oracle, &mut |_| {}))
        else {
            return;
        };
        cubes += 1;
        let holes: BTreeSet<u64> = r.topology.holes.iter().copied().collect();
        for (m, clean, alive_above) in deaths(&mut kb, &mut terms, &ast, &r, &extendable, full) {
            dead += 1;
            if clean {
                untainted += 1;
            }
            if clean && alive_above {
                failures.push(format!(
                    "{name}: {} is dead on no `absent` over D, and a superset is alive",
                    r.name(m)
                ));
            }
            if holes.contains(&m) && clean {
                failures.push(format!("{name}: the dead hole {} is untainted", r.name(m)));
            }
        }
    };
    for path in corpus_and_register() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        check(&rel, &|| loaded(&path));
    }
    for (name, text) in written::WRITTEN {
        check(name, &|| Some(loaded_text(name, text)));
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    eprintln!("{cubes} flagged cubes, {dead} dead nodes, {untainted} untainted");
    // 2026-11-02: 20 flagged cubes (18 corpus and register, 2 written), 124
    // dead nodes, 100 of them untainted. A sweep that found none would pass.
    assert!(cubes >= 20, "{cubes} flagged cubes");
    assert!(
        untainted >= 90 && dead > untainted,
        "{dead} dead, {untainted} untainted"
    );

    // Read off provenance alone, `ACTIVATOR_UNDER_ABSENT`'s dead hole is
    // untainted with an alive superset.
    let (mut kb, mut terms, ast) =
        loaded_text("activator_under_absent", written::ACTIVATOR_UNDER_ABSENT);
    let Outcome::Region(r) = forbid::studying(|| {
        region::compute(&mut kb, &mut terms, &ast, &Options::default(), &mut |_| {})
    })
    .expect("computes") else {
        panic!("activator_under_absent: within the limit");
    };
    assert_eq!(names(&r, &r.topology.holes), ["{(p A), (r A)}"]);
    let hole = r.topology.holes[0];
    assert!(r.nodes[hole as usize].absent_premises.is_empty());
    let d: Vec<String> = r
        .monotonicity
        .dynamic
        .iter()
        .map(|x| x.label(&terms))
        .collect();
    let bare = deaths(
        &mut kb,
        &mut terms,
        &ast,
        &r,
        &d,
        taint::Walk { activators: false },
    );
    assert!(
        bare.iter()
            .any(|&(m, clean, alive)| m == hole && clean && alive),
        "{bare:?}"
    );
}

// ── Route 1 — S1e2.1a.9 ─────────────────────────────────────────────

/// A program's own load, or `None` for a file that does not load (the
/// negative fixtures).
fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

/// Every corpus file and every register witness, once. A debug build left
/// out W7's two witnesses, which it aborted on, until M1e2 S1e2.2.7 fixed W7,
/// and W16's order pair until M1e2 S1e2.2.16 fixed W16.
fn corpus_and_register() -> Vec<PathBuf> {
    let mut files = ein_corpus::corpus_files();
    for row in ein_corpus::register().case {
        for (_, a) in row.answers() {
            files.push(repo_root().join(row.witness_of(a)));
        }
    }
    files.sort();
    files.dedup();
    files
}

fn facts_text(kb: &Kb, terms: &Terms) -> BTreeSet<String> {
    kb.facts()
        .map(|f| ein_infer::events::sexpr(terms, f))
        .collect()
}

/// Route 1 against the oracle on one program: every disagreement, as lines.
fn route_1_against_oracle(rel: &str, r: &Region, d: &DownSet) -> Vec<String> {
    let mut bad = Vec::new();
    let texts = |c: &[region::Candidate]| c.iter().map(|c| c.text.clone()).collect::<Vec<_>>();
    if texts(&r.candidates) != texts(&d.candidates) {
        return vec![format!("{rel}: the two universes differ")];
    }
    if let Some(why) = &d.declined {
        return vec![format!(
            "{rel}: route 1 declined ({why:?}) on a monotone program"
        )];
    }
    let n = r.n();
    let size = 1usize << n;
    let t = &r.topology;
    // Visited is exactly alive ∪ N, and every other node is dead in the cube.
    let visited: BTreeSet<u64> = (0..d.points.len() as u32).map(|i| d.mask(i)).collect();
    let mut want: BTreeSet<u64> = (0..size as u64)
        .filter(|&m| r.status[m as usize] & region::DEAD == 0)
        .collect();
    want.extend(t.nogoods.iter().copied());
    if visited != want {
        let extra: Vec<String> = visited.difference(&want).map(|&m| r.name(m)).collect();
        let missing: Vec<String> = want.difference(&visited).map(|&m| r.name(m)).collect();
        bad.push(format!(
            "{rel}: visited ≠ alive ∪ N — extra {extra:?}, missing {missing:?}"
        ));
    }
    for (i, p) in d.points.iter().enumerate() {
        let m = d.mask(i as u32);
        if p.status != r.status[m as usize] {
            bad.push(format!(
                "{rel}: {} has status {} on route 1 and {} in the cube",
                r.name(m),
                p.status,
                r.status[m as usize]
            ));
        }
    }
    let desc = d
        .description
        .as_ref()
        .expect("a description when not declined");
    let masks = |v: &[u32]| v.iter().map(|&i| d.mask(i)).collect::<BTreeSet<u64>>();
    let set = |v: &[u64]| v.iter().copied().collect::<BTreeSet<u64>>();
    for (what, got, want) in [
        ("minimal", masks(&desc.minimal), set(&t.minimal)),
        ("maximal", masks(&desc.maximal), set(&t.maximal)),
        ("N", masks(&desc.nogoods), set(&t.nogoods)),
    ] {
        if got != want {
            bad.push(format!("{rel}: {what} differs"));
        }
    }
    if desc.k_c != t.size_r {
        bad.push(format!("{rel}: k_c {} against |R| {}", desc.k_c, t.size_r));
    }
    let oracle_ks = (0..size)
        .filter(|&m| r.in_r(m as u64))
        .map(|m| r.nodes[m].class)
        .collect::<BTreeSet<u64>>()
        .len() as u64;
    if desc.k_s != oracle_ks {
        bad.push(format!("{rel}: k_s {} against {oracle_ks}", desc.k_s));
    }
    let mut sizes: Vec<u64> = desc.components.iter().map(|c| c.size).collect();
    let mut want_sizes: Vec<u64> = t.components.iter().map(|c| c.size).collect();
    sizes.sort_unstable();
    want_sizes.sort_unstable();
    if sizes != want_sizes {
        bad.push(format!(
            "{rel}: components {sizes:?} against {want_sizes:?}"
        ));
    }
    let cubes: BTreeSet<(u64, u64)> = desc
        .subcubes
        .iter()
        .map(|&(a, b)| (d.mask(a), d.mask(b)))
        .collect();
    let want_cubes: BTreeSet<(u64, u64)> = t.subcubes.iter().flatten().copied().collect();
    if cubes != want_cubes {
        bad.push(format!("{rel}: sub-cubes differ"));
    }
    // One state partition: route 1's class and the oracle's are each a
    // function of the other on the alive points.
    let mut fwd: BTreeMap<u32, u64> = BTreeMap::new();
    let mut back: BTreeMap<u64, u32> = BTreeMap::new();
    for (i, p) in d.points.iter().enumerate() {
        if p.status & region::DEAD != 0 {
            continue;
        }
        let oc = r.nodes[d.mask(i as u32) as usize].class;
        if *fwd.entry(p.class).or_insert(oc) != oc || *back.entry(oc).or_insert(p.class) != p.class
        {
            bad.push(format!(
                "{rel}: {} is in another state on route 1",
                r.name(d.mask(i as u32))
            ));
        }
        if d.mask(p.class) != oc && r.nodes[d.mask(p.class) as usize].class != oc {
            bad.push(format!(
                "{rel}: {}'s class representative",
                r.name(d.mask(i as u32))
            ));
        }
        let closure: u64 = p.closure.iter().fold(0, |m, &b| m | 1 << b);
        if closure != r.nodes[d.mask(i as u32) as usize].closure {
            bad.push(format!("{rel}: {}'s closure", r.name(d.mask(i as u32))));
        }
    }
    // The merges: per state of R with several commitments, its minimal
    // generators by the local test over the cube.
    let mut by_class: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    for m in 0..size as u64 {
        if r.in_r(m) {
            by_class
                .entry(r.nodes[m as usize].class)
                .or_default()
                .push(m);
        }
    }
    let want_merged: BTreeSet<(BTreeSet<u64>, u64, u64)> = by_class
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .map(|(&c, v)| {
            let gens = v
                .iter()
                .copied()
                .filter(|&m| {
                    (0..n).all(|b| {
                        m >> b & 1 == 0
                            || r.nodes[(m ^ 1 << b) as usize].class != c
                            || !r.in_r(m ^ 1 << b)
                    })
                })
                .collect();
            (gens, v.len() as u64, r.nodes[c as usize].closure)
        })
        .collect();
    let got_merged: BTreeSet<(BTreeSet<u64>, u64, u64)> = desc
        .merged
        .iter()
        .map(|g| {
            (
                masks(&g.generators),
                g.size,
                g.to.iter().fold(0, |m, &b| m | 1 << b),
            )
        })
        .collect();
    if got_merged != want_merged {
        bad.push(format!("{rel}: the merged classes differ"));
    }
    bad
}

/// **Route 1 equals the oracle** — S1e2.1a.9's acceptance, the empirical half
/// of `monotone.rs`'s theorem made a test. On every corpus file and register
/// witness whose cube the oracle enumerates and whose static verdict is
/// monotone, route 1 visits exactly the alive nodes and `N`, agrees with the
/// cube on both bits of every node it visits, and gives the oracle's `R`: its
/// lines, `k_c`, `k_s`, components, sub-cubes, `N`, states and merges. On
/// every flagged program whose root is alive it declines, and where root is
/// dead it reads root alone and is the oracle's. Its root is the oracle's — `sat(program)`
/// — so the route writes nothing into root.
#[test]
fn route_1_is_the_oracle_on_every_monotone_program_it_reaches() {
    let mut failures: Vec<String> = Vec::new();
    let (mut monotone, mut flagged, mut saved) = (0usize, 0usize, (0u64, 0u64));
    // The oracle's record is the same at any `jobs` (`jobs_change_nothing`),
    // and its four `n = 16` cubes are most of this test's wall clock.
    let oracle = Options {
        jobs: std::thread::available_parallelism().map_or(1, |n| n.get().min(8)),
        ..Options::default()
    };
    for path in corpus_and_register() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        let Some((mut kb, mut terms, ast)) = loaded(&path) else {
            continue;
        };
        let t0 = std::time::Instant::now();
        let Ok(Outcome::Region(r)) =
            region::compute(&mut kb, &mut terms, &ast, &oracle, &mut |_| {})
        else {
            continue;
        };
        let (mut kb1, mut terms1, ast1) = loaded(&path).expect("loaded once already");
        let opts = downset::Options::unbounded(1);
        let d = downset::compute(&mut kb1, &mut terms1, &ast1, &opts)
            .unwrap_or_else(|e| panic!("{rel}: {e}"));
        if facts_text(&kb1, &terms1) != facts_text(&kb, &terms) {
            failures.push(format!("{rel}: route 1's root is not sat(program)"));
        }
        if !r.monotonicity.is_monotone() {
            flagged += 1;
            // A dead root decides a flagged program too (L0, a fork only
            // adds: `downset::compute` since the switch), and route 1 reads
            // root alone. No program the oracle answers has the shape: W16's
            // second-shape witnesses had it from their banking until W16's
            // fix the same day (M1e2 S1e2.2.16), and *forbid* refuses them
            // since. This expected a decline there until that banking.
            if d.root_dead {
                failures.extend(route_1_against_oracle(&rel, &r, &d));
            } else if d.declined != Some(Declined::NotMonotone) {
                failures.push(format!("{rel}: flagged, and route 1 did not decline"));
            }
            continue;
        }
        monotone += 1;
        saved.0 += d.points.len() as u64;
        saved.1 += 1u64 << r.n();
        failures.extend(route_1_against_oracle(&rel, &r, &d));
        if std::env::var_os("EIN_ROUTE1_TIMES").is_some() {
            eprintln!("{:.3} {rel}", t0.elapsed().as_secs_f64());
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    // The census of 2026-10-16 (`region_census.md`): 146 monotone cubes and
    // 13 flagged. A sweep that silently found none would pass the loop above.
    // 2026-11-04: 158 monotone and 9 flagged, since *forbid* (M1e2 S1e2.2.5)
    // refuses nine of the flagged cubes, which no route answers.
    assert!(
        monotone >= 140 && flagged >= 8,
        "{monotone} monotone, {flagged} flagged"
    );
    assert!(
        saved.0 < saved.1,
        "route 1 visited {} of {} nodes",
        saved.0,
        saved.1
    );
}

/// Route 1 at four jobs is route 1 at one, point for point.
#[test]
fn route_1_jobs_change_nothing() {
    for rel in [
        "examples/branching/05_mini_zebra.ein",
        "examples/saturation/type-exclusivity/pets.ein",
        "examples/lattice/05_owed_seats_bounded.ein",
        "examples/lattice/07_two_pairs_two_tables.ein",
    ] {
        let run = |jobs: usize| {
            let (mut kb, mut terms, ast) = loaded(&repo_root().join(rel)).expect(rel);
            let opts = downset::Options::unbounded(jobs);
            downset::compute(&mut kb, &mut terms, &ast, &opts).unwrap()
        };
        let (a, b) = (run(1), run(4));
        assert_eq!(a.points.len(), b.points.len(), "{rel}");
        for (x, y) in a.points.iter().zip(&b.points) {
            assert_eq!(
                (&x.set, x.status, &x.owed, &x.core, x.class, &x.closure),
                (&y.set, y.status, &y.owed, &y.core, y.class, &y.closure),
                "{rel}"
            );
        }
        assert_eq!(a.layers, b.layers, "{rel}");
    }
}

/// Past the oracle's limit: route 1 describes `zebra2.ein`'s neighbours or
/// declines at its budget, and says which — never a silent partial answer.
/// In visits, as the measurement examples ask.
#[test]
fn route_1_declines_at_its_budget_with_whole_layers() {
    let (mut kb, mut terms, ast) =
        loaded(&repo_root().join("examples/branching/10_kill_cache_on.ein")).unwrap();
    let opts = downset::Options {
        max_visits: 3,
        ..downset::Options::unbounded(1)
    };
    let d = downset::compute(&mut kb, &mut terms, &ast, &opts).unwrap();
    let Some(Declined::Budget { layer, next, .. }) = d.declined else {
        panic!("{:?}", d.declined);
    };
    assert_eq!(layer, 1, "∅ fits; layer 1's {next} do not");
    assert_eq!(next, d.n() as u64);
    assert_eq!(d.points.len(), 1);
    assert!(d.description.is_none());
}

/// The ruled unit, facts beyond root: the budget is checked before each
/// layer against the layer's size times the facts per node of the layer
/// before, so a run declines on a projection and never mid-layer, and what it
/// spent is at most the budget.
#[test]
fn route_1_declines_at_a_facts_budget_before_the_layer_that_would_pass_it() {
    let (mut kb, mut terms, ast) = loaded(&repo_root().join("examples/zebra2-hints.ein")).unwrap();
    let budget = 1 << 14;
    let opts = downset::Options {
        budget,
        ..downset::Options::unbounded(1)
    };
    let d = downset::compute(&mut kb, &mut terms, &ast, &opts).unwrap();
    let Some(Declined::Budget {
        layer,
        next,
        spent,
        projected,
    }) = d.declined
    else {
        panic!("{:?}", d.declined);
    };
    assert_eq!(d.layers.len(), layer, "whole layers 0 … {layer} − 1");
    assert_eq!(spent, d.layers.iter().map(|l| l.facts).sum::<u64>());
    assert!(spent <= budget, "{spent} spent of {budget}");
    assert!(spent + projected > budget);
    let before = d.layers.last().expect("root ran");
    assert_eq!(
        projected,
        next * (before.facts / before.visited).max(1),
        "the layer's size times the facts per node of the layer before"
    );
    assert!(d.description.is_none());
}

// ── The state route — S1e2.1c.2 ─────────────────────────────────────────

/// The state route against the oracle on one program: every disagreement, as
/// lines.
fn state_route_against_oracle(rel: &str, r: &Region, s: &states::StateRoute) -> Vec<String> {
    let mut bad = Vec::new();
    let texts = |c: &[region::Candidate]| c.iter().map(|c| c.text.clone()).collect::<Vec<_>>();
    if texts(&r.candidates) != texts(&s.candidates) {
        return vec![format!("{rel}: the two universes differ")];
    }
    if let Some(why) = &s.declined {
        return vec![format!("{rel}: the state route declined ({why:?})")];
    }
    let n = r.n();
    let size = 1usize << n;
    let t = &r.topology;
    let mask = |set: &[u32]| set.iter().fold(0u64, |m, &b| m | 1 << b);
    let alive = |m: u64| r.status[m as usize] & region::DEAD == 0;
    // The alive states are exactly the oracle's closures of alive nodes, each
    // with the oracle's status.
    let want: BTreeSet<u64> = (0..size as u64)
        .filter(|&m| alive(m))
        .map(|m| r.nodes[m as usize].closure)
        .collect();
    let got: BTreeSet<u64> = s.states.iter().map(|x| mask(&x.closure)).collect();
    if got != want || got.len() != s.states.len() {
        bad.push(format!(
            "{rel}: {} alive states where the oracle has {}",
            s.states.len(),
            want.len()
        ));
        return bad;
    }
    for x in &s.states {
        let m = mask(&x.closure);
        if r.status[m as usize] != if x.owes > 0 { region::OWING } else { 0 } {
            bad.push(format!("{rel}: {} has another status", r.name(m)));
        }
        if !x.extended {
            bad.push(format!("{rel}: {} was not extended", r.name(m)));
        }
    }
    let d = match states::describe(s) {
        Ok(d) => d,
        Err(e) => return vec![format!("{rel}: {e}")],
    };
    // Per state of R: its commitments and its minimal generators.
    let mut by_closure: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    for m in 0..size as u64 {
        if r.in_r(m) {
            by_closure
                .entry(r.nodes[m as usize].closure)
                .or_default()
                .push(m);
        }
    }
    if d.r.len() != by_closure.len() {
        bad.push(format!(
            "{rel}: k_s {} against {}",
            d.r.len(),
            by_closure.len()
        ));
    }
    for &i in &d.r {
        let x = &s.states[i as usize];
        let members = by_closure
            .get(&mask(&x.closure))
            .cloned()
            .unwrap_or_default();
        if d.commitments[i as usize] != Some(members.len() as u64) {
            bad.push(format!(
                "{rel}: {} has {:?} commitments, the oracle {}",
                r.name(mask(&x.closure)),
                d.commitments[i as usize],
                members.len()
            ));
        }
        let gens: BTreeSet<u64> = members
            .iter()
            .copied()
            .filter(|&m| (0..n).all(|b| m >> b & 1 == 0 || !members.contains(&(m & !(1 << b)))))
            .collect();
        let got: BTreeSet<u64> = d.generators[i as usize].iter().map(|g| mask(g)).collect();
        if got != gens {
            bad.push(format!(
                "{rel}: {}'s minimal generators differ",
                r.name(mask(&x.closure))
            ));
        }
    }
    if d.k_c != t.size_r {
        bad.push(format!("{rel}: k_c {} against |R| {}", d.k_c, t.size_r));
    }
    let set = |v: &[u64]| v.iter().copied().collect::<BTreeSet<u64>>();
    let minimal: BTreeSet<u64> = d.minimal.iter().map(|(g, _)| mask(g)).collect();
    let maximal: BTreeSet<u64> = d
        .maximal
        .iter()
        .map(|&i| mask(&s.states[i as usize].closure))
        .collect();
    let nogoods: BTreeSet<u64> = d.nogoods.iter().flatten().map(|g| mask(g)).collect();
    for (what, got, want) in [
        ("minimal", &minimal, set(&t.minimal)),
        (
            "E's alive members",
            &minimal,
            set(&t.duality.minimal_from_e),
        ),
        ("maximal", &maximal, set(&t.maximal)),
        ("maximal from N", &maximal, set(&t.duality.maximal_from_n)),
        ("N", &nogoods, set(&t.nogoods)),
    ] {
        if *got != want {
            bad.push(format!("{rel}: {what} differs"));
        }
    }
    let mut comps = d.components.clone().unwrap_or_default();
    let mut want_comps: Vec<u64> = t.components.iter().map(|c| c.size).collect();
    comps.sort_unstable();
    want_comps.sort_unstable();
    if comps != want_comps {
        bad.push(format!(
            "{rel}: components {comps:?} against {want_comps:?}"
        ));
    }
    let cubes: BTreeSet<(u64, u64)> = d
        .subcubes
        .iter()
        .flatten()
        .map(|(a, b)| (mask(a), mask(b)))
        .collect();
    let want_cubes: BTreeSet<(u64, u64)> = t.subcubes.iter().flatten().copied().collect();
    if cubes != want_cubes {
        bad.push(format!("{rel}: sub-cubes differ"));
    }
    let frontier: BTreeSet<u64> = d
        .frontier
        .iter()
        .map(|&i| mask(&s.states[i as usize].closure))
        .collect();
    let want_frontier: BTreeSet<u64> = if by_closure.is_empty() {
        (0..size as u64)
            .filter(|&m| alive(m) && r.status[m as usize] & region::OWING != 0)
            .filter(|&m| !(0..n).any(|b| m >> b & 1 == 0 && alive(m | 1 << b)))
            .collect()
    } else {
        BTreeSet::new()
    };
    if frontier != want_frontier {
        bad.push(format!("{rel}: the owing frontier differs"));
    }
    bad
}

/// **The state route equals the oracle** — S1e2.1c.2's acceptance. On every
/// corpus file and register witness whose cube the oracle enumerates and whose
/// static verdict is monotone, the route meets exactly the alive states, each
/// with the cube's status, extends every one, and reads off them the oracle's
/// `R`: `k_s`, `k_c`, each state's commitments (L3) and minimal generators,
/// both lines (and each again by duality, from `E`'s alive members and from
/// `N`), `N` itself, the components, the sub-cubes and the owing frontier. It
/// never saturates more than route 1 does. On every flagged program it
/// declines.
#[test]
fn the_state_route_is_the_oracle_on_every_monotone_program_it_reaches() {
    let mut failures: Vec<String> = Vec::new();
    let (mut monotone, mut flagged, mut fewer) = (0usize, 0usize, 0usize);
    let oracle = Options {
        jobs: std::thread::available_parallelism().map_or(1, |n| n.get().min(8)),
        ..Options::default()
    };
    for path in corpus_and_register() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        let Some((mut kb, mut terms, ast)) = loaded(&path) else {
            continue;
        };
        let Ok(Outcome::Region(r)) =
            region::compute(&mut kb, &mut terms, &ast, &oracle, &mut |_| {})
        else {
            continue;
        };
        let (mut kb1, mut terms1, ast1) = loaded(&path).expect("loaded once already");
        let s = states::compute(
            &mut kb1,
            &mut terms1,
            &ast1,
            &downset::Options::unbounded(1),
        )
        .unwrap_or_else(|e| panic!("{rel}: {e}"));
        if facts_text(&kb1, &terms1) != facts_text(&kb, &terms) {
            failures.push(format!("{rel}: the state route's root is not sat(program)"));
        }
        if !r.monotonicity.is_monotone() {
            flagged += 1;
            if s.declined != Some(Declined::NotMonotone) {
                failures.push(format!(
                    "{rel}: flagged, and the state route did not decline"
                ));
            }
            continue;
        }
        monotone += 1;
        failures.extend(state_route_against_oracle(&rel, &r, &s));
        let (mut kb2, mut terms2, ast2) = loaded(&path).expect("loaded once already");
        let d = downset::compute(
            &mut kb2,
            &mut terms2,
            &ast2,
            &downset::Options::unbounded(1),
        )
        .unwrap_or_else(|e| panic!("{rel}: {e}"));
        let route_1: u64 = d.layers.iter().map(|l| l.visited).sum();
        if s.saturations() > route_1 {
            failures.push(format!(
                "{rel}: the state route saturated {} where route 1 saturates {route_1}",
                s.saturations()
            ));
        }
        fewer += usize::from(s.saturations() < route_1);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    // 2026-11-04: 158 monotone and 9 flagged, as route 1's sweep above.
    assert!(
        monotone >= 140 && flagged >= 8,
        "{monotone} monotone, {flagged} flagged"
    );
    eprintln!("{monotone} monotone, {flagged} flagged; fewer saturations on {fewer}");
    // 2026-11-03: fewer saturations than route 1 on 13 of the monotone cubes,
    // and the same number on the rest.
    assert!(fewer >= 10, "fewer saturations on {fewer} programs");
}

/// R's commitments of each size `0 ..= k`, counted from the state route's
/// description: every subset of a state of R that holds one of its minimal
/// generators. Route 1 counts the same column layer by layer.
fn r_by_size(s: &states::StateRoute, d: &states::Description, k: usize) -> Vec<u64> {
    let mut out = vec![0u64; k + 1];
    for &i in &d.r {
        let st = &s.states[i as usize];
        let at = |b: u32| st.closure.binary_search(&b).expect("in S");
        let gens: Vec<u128> = d.generators[i as usize]
            .iter()
            .map(|g| g.iter().fold(0u128, |m, &b| m | 1 << at(b)))
            .collect();
        let m = st.closure.len();
        // Every subset of S of at most k members, by a stack of positions.
        let mut stack: Vec<(usize, u128, usize)> = vec![(0, 0, 0)];
        while let Some((from, mask, size)) = stack.pop() {
            if gens.iter().any(|&g| g & !mask == 0) {
                out[size] += 1;
            }
            if size < k {
                for j in from..m {
                    stack.push((j + 1, mask | 1 << j, size + 1));
                }
            }
        }
    }
    out
}

/// **Past the oracle's limit, the state route is route 1 where route 1
/// finishes** — on `zebra2-minus-15`, which route 1 runs to layer 6 under a
/// 2²² ceiling and no further (`alive_set_reach.md` § 2.3, 2026-11-03, 495 s at
/// `-j 16`). Its R per size and its N per size, through layer 6, are route
/// 1's, banked here from that run; and the state route finishes it, with the
/// 32 states M1d's tree found. `EIN_CORPUS_SLOW=1` only: about 10 s at
/// `-j 16` in a release build. The CI nightly runs it since M1f S1f.5.27
/// (Q-M1f.51), where no tier had.
#[test]
fn the_state_route_is_route_1_on_zebra2_minus_15_through_layer_6() {
    if std::env::var_os("EIN_CORPUS_SLOW").is_none() {
        return;
    }
    // Route 1's layers 0–6: in R, and dead (each a member of N).
    const IN_R: [u64; 7] = [0, 0, 221, 10_270, 107_963, 585_989, 2_135_864];
    const DEAD: [u64; 7] = [0, 0, 1_428, 13_269, 11_799, 1_300, 8];
    let path = repo_root().join("examples/zebra2-minus-15.ein");
    let (mut kb, mut terms, ast) = loaded(&path).expect("loads");
    let jobs = std::thread::available_parallelism().map_or(1, |n| n.get());
    let s = states::compute(
        &mut kb,
        &mut terms,
        &ast,
        &downset::Options::unbounded(jobs),
    )
    .expect("the state route runs");
    assert!(s.declined.is_none(), "{:?}", s.declined);
    let d = states::describe(&s).expect("described");
    assert_eq!(r_by_size(&s, &d, 6), IN_R);
    let mut dead = [0u64; 7];
    let nogoods = d.nogoods.as_ref().expect("N, exhausted");
    for g in nogoods {
        if g.len() <= 6 {
            dead[g.len()] += 1;
        }
    }
    assert_eq!(dead, DEAD);
    // Nothing past layer 6 is dead: route 1 found N nearly stopped there.
    assert_eq!(nogoods.len() as u64, DEAD.iter().sum::<u64>());
    assert_eq!(d.r.len(), 32, "k_s");
    assert_eq!(d.k_c, 241_462_963);
}

// ── L5 — S1e2.1c.4 ──────────────────────────────────────────────────

/// **L5 is the oracle's**: an owed instance's alternatives are exhaustive.
/// On every monotone cube the oracle enumerates whose program states an
/// obligation, read at every alive state, for every instance L5's premise
/// licenses (`oblgen::alternatives`, checked at load):
///
/// - the instance is owed at the state exactly when none of its alternatives
///   is in the state — the premise, read against the state's own fixpoint,
///   whose debts `obligations::tally` counts as every route's node does;
/// - **the licence**: every member of R above a state that owes it holds one
///   of its alternatives, so an owing state whose every alternative is dead
///   has no member of R above it.
///
/// And no state owes an instance root did not. The modes' obligation tree
/// (`crate::tree`) rests on these three and checks the first and the third on
/// every node it computes; this holds the second, before any mode relies on
/// it.
#[test]
fn l5_is_the_oracle_on_every_monotone_program_it_reaches() {
    use ein_infer::events::Events;
    let mut failures: Vec<String> = Vec::new();
    let (mut programs, mut licensed, mut checked, mut owing) = (0usize, 0usize, 0usize, 0usize);
    let oracle = Options {
        jobs: std::thread::available_parallelism().map_or(1, |n| n.get().min(8)),
        ..Options::default()
    };
    for path in corpus_and_register() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        let Some((mut kb, mut terms, ast)) = loaded(&path) else {
            continue;
        };
        let Ok(Outcome::Region(r)) =
            region::compute(&mut kb, &mut terms, &ast, &oracle, &mut |_| {})
        else {
            continue;
        };
        if !r.monotonicity.is_monotone() {
            continue;
        }
        let (mut kb1, mut terms1, ast1) = loaded(&path).expect("loaded once already");
        let Some(table) = ein_infer::oblgen::alternatives_of(&mut kb1, &mut terms1, &ast1)
            .unwrap_or_else(|e| panic!("{rel}: {e}"))
        else {
            continue;
        };
        programs += 1;
        let lic: Vec<(usize, u64)> = table
            .instances
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                let alts = x.alternatives.as_ref().ok()?;
                Some((i, alts.iter().fold(0u64, |m, &b| m | 1 << b)))
            })
            .collect();
        licensed += usize::from(!lic.is_empty());
        let n = r.n();
        let alive = |m: u64| r.status[m as usize] & region::DEAD == 0;
        let in_r: Vec<u64> = (0..1u64 << n).filter(|&m| r.in_r(m)).collect();
        // Every alive state: an alive mask that is its own closure.
        let states: Vec<u64> = (0..1u64 << n)
            .filter(|&m| alive(m) && r.nodes[m as usize].closure == m)
            .collect();
        let memo = ein_infer::SharedMemo::default();
        let mut nodes = region::Nodes::new(&mut kb1, &mut terms1, &ast1, &r.candidates)
            .unwrap_or_else(|e| panic!("{rel}: {e}"));
        for m in states {
            let facts: Vec<_> = (0..n)
                .filter(|b| m >> b & 1 == 1)
                .map(|b| r.candidates[b].fact)
                .collect();
            let owed: Vec<Option<u32>> = nodes
                .with_fixpoint(&facts, |kb, terms| {
                    ein_infer::obligations::tally(kb, terms, &ast1, &memo, &mut Events::off())
                        .expect("the tally compiles")
                        .instances()
                        .iter()
                        .map(|o| {
                            let key = (o.rule, o.activator.clone(), o.bindings.clone());
                            table.index.get(&key).copied()
                        })
                        .collect()
                })
                .unwrap_or_else(|e| panic!("{rel}: {e}"));
            checked += 1;
            if owed.contains(&None) {
                failures.push(format!(
                    "{rel}: {} owes an instance root did not",
                    r.name(m)
                ));
            }
            for &(i, alts) in &lic {
                let witnessed = m & alts != 0;
                let is_owed = owed.contains(&Some(i as u32));
                if witnessed == is_owed {
                    failures.push(format!(
                        "{rel}: {} {} instance {i}, with {} alternative in it",
                        r.name(m),
                        if is_owed { "owes" } else { "has paid" },
                        if witnessed { "an" } else { "no" }
                    ));
                }
                if !is_owed {
                    continue;
                }
                owing += 1;
                for &d in &in_r {
                    if d & m == m && r.nodes[d as usize].closure & alts == 0 {
                        failures.push(format!(
                            "{rel}: {} is in R above {}, which owes instance {i}, and holds \
                             none of its alternatives",
                            r.name(d),
                            r.name(m)
                        ));
                    }
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    // Not vacuous: the licence binds on real programs and real debts — 40
    // programs, 12 with a licensed instance, 284 states and 299 owed licensed
    // instances on 2026-11-03.
    assert!(
        programs >= 35 && licensed >= 10 && checked >= 250 && owing >= 250,
        "{programs} programs, {licensed} with a licensed instance, {checked} states, \
         {owing} owed licensed instances"
    );
    eprintln!(
        "L5: {programs} programs, {licensed} with a licensed instance, {checked} states, \
         {owing} owed licensed instances"
    );
}

/// L5's premise on the zebra family, past the oracle's limit: what the table
/// licenses, and why it licenses no more. On `zebra.ein` the hrule proposes
/// `(co-located v House-n)` only, so each `slot-owed-room` debt is licensed
/// and no `slot-owed-fill` one is: its witnesses `(co-located House-n v)` are
/// derived by symmetry, never proposed.
#[test]
fn l5_licenses_what_its_premise_holds_of_on_the_zebra_family() {
    for (file, instances, licensed) in [
        ("examples/zebra.ein", 36, 18),
        ("examples/zebra2.ein", 36, 36),
        ("examples/zebra2-obligations.ein", 36, 36),
        ("examples/zebra2-minus-15.ein", 46, 46),
    ] {
        let (mut kb, mut terms, ast) = loaded(&repo_root().join(file)).expect("loads");
        let t = ein_infer::oblgen::alternatives_of(&mut kb, &mut terms, &ast)
            .expect("prepares")
            .expect("a monotone program with obligations");
        assert_eq!(
            (t.instances.len(), t.licensed()),
            (instances, licensed),
            "{file}"
        );
        for i in &t.instances {
            // The program writes no `slot-owed-fill`: its full name since M1f
            // S1f.6.35.
            let rule = terms.sym(i.key.0).to_string();
            let short = rule.strip_prefix("std.slots.").unwrap_or(&rule);
            match (&i.alternatives, short) {
                (Ok(a), _) => assert!(!a.is_empty(), "{file}: {rule} licensed, no alternative"),
                (Err(why), "slot-owed-fill") => {
                    assert!(why.contains("no candidate proposes"), "{file}: {why}")
                }
                (Err(why), _) => panic!("{file}: {rule} not licensed: {why}"),
            }
        }
    }
}

/// **A dead node's core follows the activator its refutation rests on, where
/// a node derived or committed it** — M1e2 S1e2.2.19, W19, on the user's
/// ruling of 2026-11-04 on Q-M1e2.12, (c).
///
/// A plan of a rule with parameters is compiled from an activator, and until
/// S1e2.2.19 a derivation recorded the premises the plan matched and not the
/// activator, so a commitment that reaches a refutation only through one was
/// in no core: `forbid-chain-no-absent`'s dead `{(p a)}` reported `(seed a)`,
/// a fact of root's while root is alive, `core-activator-committed`'s
/// `{(r2 a)}` the same, and `core-activator-guard-only`'s `{(p a)}` nothing.
/// Root's activators, stated (`core-activator-given`) or derived
/// (`core-activator-root-derived`), are the program's, as the plans root
/// compiles from them are its rules, and are followed into no core. W17's
/// five, studied past *forbid*'s refusal, report the core that names the
/// commitment too, but `-node-writes`, whose refutation matches it.
#[test]
fn a_dead_nodes_core_follows_an_activator_only_a_node_derives() {
    let dir = "examples/ein-bugs/wrong-answers";
    let cases: [(&str, &[&str]); 11] = [
        ("forbid-chain-no-absent", &["(p a)", "(seed a)"]),
        ("core-activator-committed", &["(r2 a)", "(seed a)"]),
        ("core-activator-guard-only", &["(p a)"]),
        ("core-activator-given", &["(p a)"]),
        ("core-activator-root-derived", &["(p a)"]),
        ("forbid-chain-0", &["(p a)", "(seed a)"]),
        ("forbid-chain-1", &["(p a)", "(seed a)"]),
        ("forbid-chain-2", &["(p a)", "(seed a)"]),
        ("forbid-chain-node-absent", &["(p a)", "(seed a)"]),
        ("forbid-chain-param-absent", &["(p a)", "(seed a)"]),
        ("forbid-chain-node-writes", &["(p a)"]),
    ];
    let mut bad = Vec::new();
    for (name, want) in cases {
        let rel = format!("{dir}/{name}.ein");
        let (r, _) = region_of(&rel);
        let mut dead = 0;
        for (mask, node) in r.nodes.iter().enumerate() {
            if r.status[mask] & region::DEAD == 0 {
                continue;
            }
            dead += 1;
            if node.core != want {
                bad.push(format!(
                    "{name}: dead {} reports core {:?}, where it rests on {want:?}",
                    r.name(mask as u64),
                    node.core
                ));
            }
        }
        if dead == 0 {
            bad.push(format!("{name}: no dead node"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **A contradiction's core covers what it rests on: both sides of a pair, and
/// the facts a `forall` read** — M1e2 S1e2.2.21, W21, on the user's ruling of
/// 2026-11-05 on Q-M1e2.15, (b).
///
/// Until S1e2.2.21 the core search explained a contradiction by its witness
/// alone, a pair's positive `X`, and a firing recorded none of the facts its
/// universal read: `pair-core-root-given`'s dead `{(q a)}` reported `(p a)`,
/// which the program states, `pair-core-root-derived`'s `(seed a)`,
/// `forall-core-false`'s `{(tag s1), (tag s2)}` `(item a)` and
/// `forall-core-elim`'s `{(tag s1)}` `(item a) (slot s1)`, each a set of facts
/// root holds while root is alive; and `pair-core-root-negative`'s `(q a)`,
/// which named the commitment and left out the stated `(not (p a))`. Every
/// dead node of each cube reports the core below, which names its commitment
/// and, with root, derives the contradiction.
#[test]
fn a_contradictions_core_covers_both_sides_and_what_a_forall_read() {
    let dir = "examples/ein-bugs/wrong-answers";
    let cases: [(&str, &[&str]); 5] = [
        ("pair-core-root-given", &["(p a)", "(q a)"]),
        ("pair-core-root-derived", &["(q a)", "(seed a)"]),
        ("pair-core-root-negative", &["(not (p a))", "(q a)"]),
        ("forall-core-false", &["(item a)", "(tag s1)", "(tag s2)"]),
        (
            "forall-core-elim",
            &["(item a)", "(not (in a s2))", "(slot s1)", "(tag s1)"],
        ),
    ];
    let mut bad = Vec::new();
    for (name, want) in cases {
        let rel = format!("{dir}/{name}.ein");
        let (r, _) = region_of(&rel);
        let mut dead = 0;
        for (mask, node) in r.nodes.iter().enumerate() {
            if r.status[mask] & region::DEAD == 0 {
                continue;
            }
            dead += 1;
            if node.core != want {
                bad.push(format!(
                    "{name}: dead {} reports core {:?}, where it rests on {want:?}",
                    r.name(mask as u64),
                    node.core
                ));
            }
        }
        if dead == 0 {
            bad.push(format!("{name}: no dead node"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
