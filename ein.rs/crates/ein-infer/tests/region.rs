//! The oracle and the static check — M1e2
//! [S1e2.1a.1](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.1_the_oracle.md).
//!
//! Every number here is Q-M1e2.7's tables, which were taken by a different
//! route (`utils/region.py`: one `ein saturate` per subset); the comparison
//! node for node is `ein-cli/tests/region_dump.rs`'s, because that route runs
//! the binary.

use std::path::Path;

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::monotone::{Monotonicity, SiteKind};
use ein_infer::region::{self, Options, Outcome, Region};
use ein_ir::{Ast, load, parse};

fn compute(rel: &str) -> (Outcome, Terms) {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("parses");
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("loads");
    let out = region::compute(&mut kb, &mut terms, &ast, &Options::default(), &mut |_| {})
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

/// **`--jobs` changes nothing.** The same record with one thread and with
/// four, node for node and reason for reason — including the core, whose
/// choice depends on interner order (W11), because a worker never numbers a
/// fact and every new number is the calling thread's, in layer order.
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
            match region::compute(&mut kb, &mut terms, &ast, &opts, &mut |_| {}).unwrap() {
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
