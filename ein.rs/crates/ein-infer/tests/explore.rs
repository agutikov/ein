//! Every mode of `ein solve` against the oracle — M1e2
//! [S1e2.1a.9](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
//! T1e2.1a.9.4, and the contract of
//! [`solve_interface.md` § 1](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md#1-the-contract):
//!
//! - **C1** every state a run prints is in the oracle's chosen part of `R`;
//! - **C2** a run that says it exhausted that part printed all of it, and a
//!   verdict or a count it gives is the oracle's;
//! - **C3** the same at a budget small enough to stop most runs;
//! - **C4** the same answer at one job and at four.
//!
//! Over every corpus file and register witness whose cube the oracle
//! enumerates, monotone or flagged.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::explore::{self, Answer, Ask, Mode, Verdict, Which};
use ein_infer::region::{self, Outcome, Region};
use ein_ir::{Ast, load, parse};

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

fn mask(set: &[u32]) -> u64 {
    set.iter().fold(0, |m, &b| m | 1 << b)
}

const WHICH: [Which; 3] = [Which::All, Which::Minimal, Which::Maximal];

/// The oracle's reading of one part of `R`.
struct Part {
    /// The part's commitments.
    masks: BTreeSet<u64>,
    /// Their states, as the oracle's class (the smallest mask of the state).
    classes: BTreeSet<u64>,
}

fn part(r: &Region, which: Which) -> Part {
    let masks: BTreeSet<u64> = match which {
        Which::All => (0..1u64 << r.n()).filter(|&m| r.in_r(m)).collect(),
        Which::Minimal => r.topology.minimal.iter().copied().collect(),
        Which::Maximal => r.topology.maximal.iter().copied().collect(),
    };
    let classes = masks.iter().map(|&m| r.nodes[m as usize].class).collect();
    Part { masks, classes }
}

fn oracle_verdict(r: &Region, p: &Part) -> Verdict {
    match p.classes.len() {
        1 => Verdict::Solution,
        0 if r.status.iter().any(|s| s & region::DEAD == 0) => Verdict::Open,
        0 => Verdict::Contradiction,
        _ => Verdict::Ambiguity,
    }
}

/// Every way `a` breaks C1 or C2 against the oracle, as lines.
fn against(rel: &str, r: &Region, a: &Answer, budgeted: bool) -> Vec<String> {
    let mut bad = Vec::new();
    let tag = format!(
        "{rel} {:?} {:?}{}",
        a.mode.which,
        a.mode.ask,
        if budgeted { " (budget)" } else { "" }
    );
    let p = part(r, a.mode.which);
    if !budgeted && a.stopped.is_some() {
        bad.push(format!(
            "{tag}: stopped ({:?}) inside the oracle's limit",
            a.stopped
        ));
    }
    // C1: every printed state is a member of the part, and they are distinct.
    let mut printed: Vec<u64> = Vec::new();
    for s in &a.states {
        let m = mask(&s.set);
        if !r.in_r(m) {
            bad.push(format!("{tag}: printed {} is not in R", a.name(&s.set)));
            continue;
        }
        let class = r.nodes[m as usize].class;
        if !p.classes.contains(&class) {
            bad.push(format!(
                "{tag}: printed {} is not on the line",
                a.name(&s.set)
            ));
        }
        if mask(&s.closure) != r.nodes[m as usize].closure {
            bad.push(format!(
                "{tag}: {}'s closure differs from the oracle's",
                a.name(&s.set)
            ));
        }
        if a.mode.which == Which::Maximal && r.monotonicity.is_monotone() {
            // The climb's top is a maximal commitment itself.
            if !p.masks.contains(&mask(&s.closure)) {
                bad.push(format!(
                    "{tag}: top {} is not a maximal commitment",
                    a.name(&s.closure)
                ));
            }
        }
        printed.push(class);
    }
    let distinct: BTreeSet<u64> = printed.iter().copied().collect();
    if distinct.len() != printed.len() {
        bad.push(format!("{tag}: a state printed twice"));
    }
    for (set, at) in &a.minimal {
        if !p.masks.contains(&mask(set)) {
            bad.push(format!(
                "{tag}: {} printed as minimal and is not",
                a.name(set)
            ));
        }
        if *at >= a.states.len() || r.nodes[mask(set) as usize].class != printed[*at] {
            bad.push(format!("{tag}: {} points at the wrong state", a.name(set)));
        }
    }
    // C2: an exhausted part is all of the part.
    if a.exhausted {
        match a.mode.ask {
            Ask::Describe | Ask::Find(_) => {
                if distinct != p.classes {
                    bad.push(format!(
                        "{tag}: exhausted with {} states where the oracle has {}",
                        distinct.len(),
                        p.classes.len()
                    ));
                }
            }
            Ask::Single => {}
        }
        if a.mode.which == Which::Minimal && a.mode.ask != Ask::Single {
            let got: BTreeSet<u64> = a.minimal.iter().map(|(s, _)| mask(s)).collect();
            if got != p.masks {
                bad.push(format!("{tag}: exhausted minimal commitments differ"));
            }
        }
    }
    if let Ask::Find(want) = a.mode.ask
        && a.stopped.is_none()
    {
        let expect = (want as usize).min(p.classes.len());
        if printed.len() != expect {
            bad.push(format!("{tag}: printed {} of {expect}", printed.len()));
        }
    }
    if a.mode.ask == Ask::Single {
        let want = oracle_verdict(r, &p);
        match a.verdict {
            Some(v) if v != want => {
                bad.push(format!("{tag}: {v:?} where the oracle says {want:?}"))
            }
            Some(Verdict::Ambiguity) if distinct.len() != 2 => bad.push(format!(
                "{tag}: Ambiguity with {} states printed",
                distinct.len()
            )),
            None if a.stopped.is_none() => bad.push(format!("{tag}: no verdict and not stopped")),
            _ => {}
        }
    }
    if a.mode.ask == Ask::Describe && a.mode.which == Which::All && a.exhausted {
        let c = a.counts.expect("describe counts");
        let k_s = p.classes.len() as u64;
        if (c.k_c, c.k_s) != (r.topology.size_r, k_s) {
            bad.push(format!(
                "{tag}: k_c, k_s = {}, {} where the oracle has {}, {k_s}",
                c.k_c, c.k_s, r.topology.size_r
            ));
        }
        let shape = a.shape.as_ref().expect("an exhausted describe has a shape");
        let set = |v: &[Box<[u32]>]| v.iter().map(|s| mask(s)).collect::<BTreeSet<u64>>();
        let t = &r.topology;
        if set(&shape.minimal) != t.minimal.iter().copied().collect()
            || set(&shape.maximal) != t.maximal.iter().copied().collect()
        {
            bad.push(format!("{tag}: the lines differ"));
        }
        if shape.empty_in_r != r.in_r(0)
            || shape.components != t.components.len()
            || shape.nogoods != t.nogoods.len()
        {
            bad.push(format!("{tag}: ∅, the components or N differ"));
        }
    }
    bad
}

fn cells(k: usize) -> Vec<Mode> {
    let mut v = Vec::new();
    for which in WHICH {
        for ask in [
            Ask::Describe,
            Ask::Find(1),
            Ask::Find(2),
            Ask::Find(k as u64 + 1),
            Ask::Single,
        ] {
            v.push(Mode { which, ask });
        }
    }
    v
}

fn jobs() -> usize {
    std::thread::available_parallelism().map_or(1, |n| n.get().min(8))
}

fn ask(path: &Path, opts: &explore::Options) -> (Answer, BTreeSet<String>) {
    let (mut kb, mut terms, ast) = loaded(path).expect("loaded once already");
    let a = explore::run(&mut kb, &mut terms, &ast, opts)
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    (a, facts_text(&kb, &terms))
}

/// **Every mode is the oracle** — C1, C2 and C3 of the contract, every cell
/// of `ein solve`'s two axes, on every program whose cube the oracle
/// enumerates. And root afterwards is `sat(program)`: no mode writes it.
#[test]
fn every_mode_is_the_oracle() {
    let mut failures: Vec<String> = Vec::new();
    let mut counted: BTreeMap<&str, usize> = BTreeMap::new();
    let mut stopped = 0usize;
    let oracle = region::Options {
        jobs: jobs(),
        ..region::Options::default()
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
        let root = facts_text(&kb, &terms);
        let monotone = r.monotonicity.is_monotone();
        *counted
            .entry(if monotone { "monotone" } else { "flagged" })
            .or_default() += 1;
        let k_s = part(&r, Which::All).classes.len();
        for mode in cells(k_s) {
            let mut opts = explore::Options::new(mode);
            opts.jobs = jobs();
            opts.budget = u64::MAX;
            let (a, after) = ask(&path, &opts);
            if after != root {
                failures.push(format!(
                    "{rel} {mode:?}: root afterwards is not sat(program)"
                ));
            }
            if a.route == explore::Route::DownSet && !monotone {
                failures.push(format!("{rel} {mode:?}: route 1 on a flagged program"));
            }
            failures.extend(against(&rel, &r, &a, false));
            // C3: a budget that stops many runs on the way. The cube is
            // bounded by the limit and not by the budget, so a flagged
            // program answers as above.
            if monotone {
                opts.budget = 8;
                let (b, _) = ask(&path, &opts);
                stopped += usize::from(b.stopped.is_some());
                failures.extend(against(&rel, &r, &b, true));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    // C3 binds only where the budget stopped a run; a budget nothing reached
    // would pass the loop above without testing it.
    // 247 on 2026-10-16.
    assert!(stopped >= 200, "the budget stopped {stopped} runs");
    assert!(
        counted.get("monotone").copied().unwrap_or(0) >= 140
            && counted.get("flagged").copied().unwrap_or(0) >= 10,
        "{counted:?}"
    );
}

/// **C4**: one job and four give the same answer, the same work and the same
/// order, on programs where each mode does real work.
#[test]
fn every_mode_at_four_jobs_is_every_mode_at_one() {
    for rel in [
        "examples/lattice/02_genuine_3set_death.ein",
        "examples/lattice/06_friends_tables_everyone_sits.ein",
        "examples/lattice/07_two_pairs_two_tables.ein",
        "examples/saturation/type-exclusivity/pets.ein",
        "examples/ein-bugs/wrong-answers/owed-seats.ein",
        "examples/ein-bugs/naf-upward-closure.ein",
    ] {
        let path = repo_root().join(rel);
        for mode in cells(3) {
            let run = |jobs: usize| {
                let mut opts = explore::Options::new(mode);
                opts.jobs = jobs;
                ask(&path, &opts).0
            };
            let (a, b) = (run(1), run(4));
            assert_eq!(
                (
                    &a.states,
                    &a.minimal,
                    a.exhausted,
                    a.verdict,
                    a.counts,
                    &a.shape,
                    &a.stopped,
                    a.saturations,
                    a.facts
                ),
                (
                    &b.states,
                    &b.minimal,
                    b.exhausted,
                    b.verdict,
                    b.counts,
                    &b.shape,
                    &b.stopped,
                    b.saturations,
                    b.facts
                ),
                "{rel} {mode:?}"
            );
        }
    }
}

/// The cheap questions are cheap: the costs `solve_interface.md` § 3.4
/// derived for the `square-*` cube, where every commitment is alive and in
/// `R` and today's walk says *no model found*.
#[test]
fn a_square_cube_answers_its_first_questions_in_a_handful_of_saturations() {
    let path = repo_root().join("examples/saturation/square-bwd/floors.ein");
    let run = |which, ask| {
        let opts = explore::Options::new(Mode { which, ask });
        self::ask(&path, &opts).0
    };
    let any = run(Which::All, Ask::Find(1));
    assert_eq!(
        (any.saturations, any.states.len()),
        (1, 1),
        "root is a solution"
    );
    let single = run(Which::All, Ask::Single);
    assert_eq!(single.verdict, Some(Verdict::Ambiguity));
    assert_eq!(
        single.saturations, 2,
        "root, then the first alive singleton"
    );
    for ask in [Ask::Find(1), Ask::Single, Ask::Describe] {
        let max = run(Which::Maximal, ask);
        assert!(max.saturations <= 21, "{ask:?}: {}", max.saturations);
        assert_eq!(max.states.len(), 1, "{ask:?}");
        assert_eq!(
            max.states[0].closure.len(),
            max.n(),
            "{ask:?}: ⊤ holds every candidate"
        );
        assert!(max.exhausted || ask == Ask::Find(1), "{ask:?}");
    }
    let min = run(Which::Minimal, Ask::Describe);
    assert_eq!((min.saturations, min.minimal.len()), (1, 1), "∅ ∈ R");
}
