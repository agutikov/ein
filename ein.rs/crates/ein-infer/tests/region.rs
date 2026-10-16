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

/// Every corpus file and every register witness, once — less the witnesses
/// a debug build aborts on (W7's two), in a debug build.
fn corpus_and_register() -> Vec<PathBuf> {
    let mut files = ein_corpus::corpus_files();
    for row in ein_corpus::register().case {
        for (_, a) in row.answers() {
            if cfg!(debug_assertions) && a.debug_panic.is_some() {
                continue;
            }
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
/// every flagged program it declines. Its root is the oracle's — `sat(program)`
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
            if d.declined != Some(Declined::NotMonotone) {
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
    assert!(
        monotone >= 140 && flagged >= 10,
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
