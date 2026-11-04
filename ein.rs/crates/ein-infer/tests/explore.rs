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
//! enumerates, monotone or flagged. Where the oracle refuses the program
//! (*forbid*, M1e2 S1e2.2.5), every mode refuses it with the same message.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::described::Described;
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
            Ask::Describe | Ask::Find(_) | Ask::Count => {
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
    // `--count` is held where describe is: its counts are R's.
    let describes = matches!(a.mode.ask, Ask::Describe | Ask::Count) && a.mode.which == Which::All;
    if describes {
        match &a.described {
            Some(d) => bad.extend(described_against(&tag, r, d)),
            None => bad.push(format!("{tag}: describe carries no description")),
        }
    }
    if describes && a.exhausted {
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

/// `g ⊆ m`, as masks.
fn below(g: u64, m: u64) -> bool {
    g & m == g
}

/// One state of `R` as the oracle has it, keyed by its closure mask.
struct OracleState {
    commitments: u64,
    generators: BTreeSet<u64>,
    minimal: bool,
    maximal: bool,
}

/// Every way a description breaks C1 or C2 against the oracle: exactly the
/// oracle's on an exhausted run; on a partial one, nothing it does not hold,
/// and a line flag only where the oracle's line has the state.
fn described_against(tag: &str, r: &Region, d: &Described) -> Vec<String> {
    let mut bad = Vec::new();
    let n = r.n();
    let t = &r.topology;
    let line =
        |v: &[u64]| -> BTreeSet<u64> { v.iter().map(|&m| r.nodes[m as usize].class).collect() };
    let (min_line, max_line) = (line(&t.minimal), line(&t.maximal));
    let mut oracle: BTreeMap<u64, OracleState> = BTreeMap::new();
    let mut members: Vec<u64> = (0..1u64 << n).filter(|&m| r.in_r(m)).collect();
    members.sort_by_key(|m| m.count_ones());
    for m in members {
        let node = &r.nodes[m as usize];
        let o = oracle.entry(node.closure).or_insert(OracleState {
            commitments: 0,
            generators: BTreeSet::new(),
            minimal: min_line.contains(&node.class),
            maximal: max_line.contains(&node.class),
        });
        o.commitments += 1;
        if !o.generators.iter().any(|&g| below(g, m)) {
            o.generators.insert(m);
        }
    }
    let mut seen = BTreeSet::new();
    let mut last: Option<(usize, Vec<u32>)> = None;
    for s in &d.states {
        let key = (s.closure.len(), s.closure.to_vec());
        if last.as_ref().is_some_and(|l| *l >= key) {
            bad.push(format!("{tag}: states not in canonical order"));
        }
        last = Some(key);
        let c = mask(&s.closure);
        if !seen.insert(c) {
            bad.push(format!("{tag}: {} described twice", d.name(&s.closure)));
        }
        let Some(o) = oracle.get(&c) else {
            bad.push(format!("{tag}: {} is not a state of R", d.name(&s.closure)));
            continue;
        };
        let gens: BTreeSet<u64> = s.generators.iter().map(|g| mask(g)).collect();
        if !r.in_r(mask(&s.rep)) || r.nodes[mask(&s.rep) as usize].closure != c {
            bad.push(format!(
                "{tag}: {}'s rep does not reach it",
                d.name(&s.closure)
            ));
        }
        let flags_ok = if d.exhausted {
            (s.minimal, s.maximal) == (o.minimal, o.maximal)
        } else {
            (!s.minimal || o.minimal) && (!s.maximal || o.maximal)
        };
        let counts_ok = if d.exhausted {
            s.commitments == o.commitments && gens == o.generators
        } else {
            s.commitments <= o.commitments && gens.is_subset(&o.generators)
        };
        if !flags_ok || !counts_ok {
            bad.push(format!(
                "{tag}: {}: lines {:?}, {} commitments, generators {gens:?} — the oracle has \
                 {:?}, {}, {:?}",
                d.name(&s.closure),
                (s.minimal, s.maximal),
                s.commitments,
                (o.minimal, o.maximal),
                o.commitments,
                o.generators
            ));
        }
    }
    let line_masks: BTreeSet<u64> = t.minimal.iter().copied().collect();
    let got: BTreeSet<u64> = d.minimal_commitments.iter().map(|s| mask(s)).collect();
    let alive = |m: u64| r.status[m as usize] & region::DEAD == 0;
    if d.exhausted {
        if seen.len() != oracle.len() || d.k_c != t.size_r || got != line_masks {
            bad.push(format!(
                "{tag}: k_s, k_c, minimal line = {}, {}, {} where the oracle has {}, {}, {}",
                seen.len(),
                d.k_c,
                got.len(),
                oracle.len(),
                t.size_r,
                line_masks.len()
            ));
        }
        let dead: BTreeSet<u64> = d.dead.iter().map(|x| mask(&x.set)).collect();
        if d.nogoods != t.nogoods.len() as u64 || dead != t.nogoods.iter().copied().collect() {
            bad.push(format!(
                "{tag}: N = {} ({} sets) where the oracle has {}",
                d.nogoods,
                dead.len(),
                t.nogoods.len()
            ));
        }
        let shape = d.shape.as_ref();
        let mut comps: Vec<u64> = shape.map_or(Vec::new(), |s| s.components.clone());
        let mut want: Vec<u64> = t.components.iter().map(|c| c.size).collect();
        comps.sort_unstable();
        want.sort_unstable();
        if comps != want {
            bad.push(format!(
                "{tag}: components {comps:?} where the oracle has {want:?}"
            ));
        }
        if let (Some(mut dims), Some(cubes)) =
            (shape.and_then(|s| s.subcubes.clone()), t.subcubes.as_ref())
        {
            let mut want: Vec<usize> = cubes
                .iter()
                .map(|&(lo, hi)| (hi.count_ones() - lo.count_ones()) as usize)
                .collect();
            dims.sort_unstable();
            want.sort_unstable();
            if dims != want {
                bad.push(format!(
                    "{tag}: sub-cubes {dims:?} where the oracle has {want:?}"
                ));
            }
        }
        let frontier: BTreeSet<u64> = d.frontier.iter().map(|o| mask(&o.set)).collect();
        let want: BTreeSet<u64> = if oracle.is_empty() {
            (0..1u64 << n)
                .filter(|&m| alive(m) && r.status[m as usize] & region::OWING != 0)
                .filter(|&m| !(0..n).any(|b| m >> b & 1 == 0 && alive(m | 1 << b)))
                .collect()
        } else {
            BTreeSet::new()
        };
        if frontier != want {
            bad.push(format!(
                "{tag}: the owing frontier differs from the oracle's"
            ));
        }
    } else if !got.is_subset(&line_masks) || d.k_c > t.size_r {
        bad.push(format!(
            "{tag}: a partial description claims more than R holds"
        ));
    }
    let dead_alone = (0..n).filter(|&b| !alive(1 << b)).count() as u64;
    if d.exhausted && d.dead_alone != dead_alone {
        bad.push(format!(
            "{tag}: {} dead alone where the oracle has {dead_alone}",
            d.dead_alone
        ));
    }
    let root = &r.nodes[0];
    if d.root.alive != alive(0)
        || (alive(0) && d.root.owes != root.owes)
        || d.any_alive != (0..1u64 << n).any(alive)
    {
        bad.push(format!(
            "{tag}: root or any-alive differs from the oracle's"
        ));
    }
    bad
}

/// Every question of `ein solve`'s two axes: three parts of `R` × describe,
/// `-n 1`, `-n 2`, `-n k+1`, `--single`; and `--count`, over all of `R`
/// only (M1e2 T1e2.1c.4.6).
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
    v.push(Mode {
        which: Which::All,
        ask: Ask::Count,
    });
    v
}

/// What an answer says, less the question it answers: `--count`'s must be
/// describe's in every field, since it takes describe's route.
fn said(a: &Answer) -> String {
    format!(
        "{:?} {:?} {:?} {:?} {} {:?} {:?} {:?} {:?} {} {} {:?} {:?}",
        a.states,
        a.minimal,
        a.counts,
        a.shape,
        a.exhausted,
        a.verdict,
        a.stopped,
        a.route,
        a.licences,
        a.saturations,
        a.facts,
        a.described,
        a.warning
    )
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
    let mut refused = 0usize;
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
        let out = region::compute(&mut kb, &mut terms, &ast, &oracle, &mut |_| {});
        // A refusal is the whole answer, and every path gives it alike.
        if let Err(ein_infer::SolveError::Refused(m)) = &out {
            refused += 1;
            for mode in cells(1) {
                let (mut kb, mut terms, ast) = loaded(&path).expect("loaded once already");
                match explore::run(&mut kb, &mut terms, &ast, &explore::Options::new(mode)) {
                    Err(ein_infer::SolveError::Refused(n)) if n == *m => {}
                    other => failures.push(format!(
                        "{rel} {mode:?}: the oracle refuses and the mode answers {:?}",
                        other.map(|a| a.route)
                    )),
                }
            }
            continue;
        }
        let Ok(Outcome::Region(r)) = out else {
            continue;
        };
        let root = facts_text(&kb, &terms);
        let monotone = r.monotonicity.is_monotone();
        // The cube's own description, on every program: a flagged one prints
        // it, and the claim check reads it.
        failures.extend(described_against(
            &format!("{rel} the cube"),
            &r,
            &ein_infer::described::of_cube(&r),
        ));
        *counted
            .entry(if monotone { "monotone" } else { "flagged" })
            .or_default() += 1;
        let k_s = part(&r, Which::All).classes.len();
        // Describe's answer, unbudgeted and at the C3 budget, for `--count`'s
        // to be held to.
        let mut described: [Option<String>; 2] = [None, None];
        for mode in cells(k_s) {
            let mut opts = explore::Options::new(mode);
            opts.jobs = jobs();
            opts.budget = u64::MAX;
            let (a, after) = ask(&path, &opts);
            match mode.ask {
                Ask::Describe if mode.which == Which::All => described[0] = Some(said(&a)),
                Ask::Count if described[0].as_deref() != Some(said(&a).as_str()) => {
                    failures.push(format!("{rel} --count: not describe's answer"));
                }
                _ => {}
            }
            if after != root {
                failures.push(format!(
                    "{rel} {mode:?}: root afterwards is not sat(program)"
                ));
            }
            if a.route == explore::Route::DownSet && !monotone && !a.root_dead {
                failures.push(format!("{rel} {mode:?}: route 1 on a flagged program"));
            }
            // Each mode's route (S1e2.1c.4): a flagged program keeps the cube
            // and its warning, but where its root is dead, which decides it
            // (L0, a fork only adds: `explore::run` since the switch), and
            // route 1 reads root alone, with the warning; a monotone one whose
            // root is alive takes the state route for describe and the modes'
            // routes over states otherwise, naming L5 exactly when the
            // obligation tree used it. No program the oracle answers has a
            // flagged dead root: W16's second-shape witnesses had one from
            // their banking until W16's fix the same day (M1e2 S1e2.2.16), and
            // *forbid* refuses them since. This expected the cube there until
            // that banking.
            let describe =
                mode.which == Which::All && matches!(mode.ask, Ask::Describe | Ask::Count);
            let route_ok = if !monotone && a.root_dead {
                a.route == explore::Route::DownSet && a.warning.is_some()
            } else if !monotone {
                a.route == explore::Route::Cube && a.warning.is_some()
            } else if describe {
                a.route == explore::Route::States
            } else if a.root_dead {
                a.route == explore::Route::DownSet
            } else {
                matches!(a.route, explore::Route::States | explore::Route::Tree)
            };
            if !route_ok {
                failures.push(format!(
                    "{rel} {mode:?}: route {:?} (warning {}) on a {} program",
                    a.route,
                    a.warning.is_some(),
                    if monotone { "monotone" } else { "flagged" }
                ));
            }
            let l5 = a.licences.contains(&explore::Licence::Exhaustive);
            if l5 != (a.route == explore::Route::Tree) {
                failures.push(format!(
                    "{rel} {mode:?}: route {:?} with licences {:?}",
                    a.route, a.licences
                ));
            }
            *counted
                .entry(if l5 { "tree" } else { "other" })
                .or_default() += 1;
            failures.extend(against(&rel, &r, &a, false));
            // C3: a budget that stops many runs on the way. The cube is
            // bounded by the limit and not by the budget, so a flagged
            // program answers as above.
            if monotone {
                opts.budget = 8;
                let (b, _) = ask(&path, &opts);
                stopped += usize::from(b.stopped.is_some());
                failures.extend(against(&rel, &r, &b, true));
                match mode.ask {
                    Ask::Describe if mode.which == Which::All => described[1] = Some(said(&b)),
                    Ask::Count if described[1].as_deref() != Some(said(&b).as_str()) => {
                        failures.push(format!("{rel} --count at budget 8: not describe's answer"));
                    }
                    _ => {}
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    // C3 binds only where the budget stopped a run; a budget nothing reached
    // would pass the loop above without testing it.
    // 247 on 2026-10-16.
    assert!(stopped >= 200, "the budget stopped {stopped} runs");
    // 2026-11-04: 9 flagged, since *forbid* (M1e2 S1e2.2.5) refuses ten
    // programs of the corpus and the register, each refused alike by every
    // mode.
    assert!(
        counted.get("monotone").copied().unwrap_or(0) >= 140
            && counted.get("flagged").copied().unwrap_or(0) >= 8,
        "{counted:?}"
    );
    assert!(refused >= 10, "{refused} refused");
    // L5 is used, not only licensed: the obligation tree answered real cells,
    // 168 of them on 2026-11-03.
    assert!(
        counted.get("tree").copied().unwrap_or(0) >= 100,
        "{counted:?}"
    );
    eprintln!("{counted:?}");
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
            // `naf-upward-closure`'s dead hole, studied past *forbid*'s
            // refusal (M1e2 S1e2.2.5): C4 is about the machinery.
            let run = |jobs: usize| {
                let mut opts = explore::Options::new(mode);
                opts.jobs = jobs;
                ein_infer::forbid::studying(|| ask(&path, &opts).0)
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
