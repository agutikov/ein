//! **The pair layer is held to `N`**: two computations, one answer.
//!
//! [`ein_infer::structure::pair_layer`] reads what one candidate and two
//! decide at root: root's node, every single, and every pair two licences
//! leave. A describe that exhausts states `N`, the minimal dead sets, which
//! the state route reads off its maximal alive states by duality
//! (`ein_infer::states`), the cube off its status array and a dead root off
//! root alone. The two share the node function and nothing past it, and they
//! must agree:
//!
//! 1. **the excluded pairs are exactly `N`'s members of two candidates**;
//! 2. **the candidates dead alone are the read-out's `dead alone`**,
//!    `Described::dead_alone_set`.
//!
//! On every corpus file and register witness the loader accepts, *forbid*
//! does not refuse and whose describe exhausts at the default
//! `--region-budget`, the layer read at the same budget. A program the static
//! check flags is among them where its cube is within `--region-limit`: there
//! the layer saturates every pair, and `N`'s pairs are its excluded pairs and
//! the dead pairs it lists against L3.
//!
//! The corpus's `slow` entries are read under `EIN_CORPUS_SLOW=1`, as the
//! other corpus sweeps read them, but for the instances the tests below name,
//! which every run reads. One test reads the flag, the sweep, and the nightly
//! runs it by name.
//!
//! ```text
//! cargo test -p ein-infer --profile gate --test pair_layer
//! EIN_CORPUS_SLOW=1 cargo test -p ein-infer --profile gate --test pair_layer -- the_excluded_pairs_are_the_pairs_of_n
//! ```

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::region::Cores;
use ein_infer::structure::{self, PairLayer};
use ein_ir::{Ast, load, parse};

fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

fn rel_of(p: &Path) -> String {
    p.strip_prefix(repo_root())
        .unwrap_or(p)
        .display()
        .to_string()
}

/// The instances every run reads, `slow` or not: the declared control, its
/// one-relation twin, and the negative control for pairs.
const INSTANCES: [&str; 3] = [
    "examples/zebra.ein",
    "examples/zebra2.ein",
    "examples/lattice/02_genuine_3set_death.ein",
];

/// Every corpus file and register witness, once; the `slow` entries that are
/// not [`INSTANCES`] where `slow_too` only.
fn files(slow_too: bool) -> Vec<PathBuf> {
    let slow: BTreeSet<PathBuf> = ein_corpus::corpus()
        .entry
        .iter()
        .filter(|e| e.slow && !INSTANCES.contains(&e.path.as_str()))
        .map(|e| repo_root().join(&e.path))
        .collect();
    let mut files = ein_corpus::corpus_files();
    for row in ein_corpus::register().case {
        for (_, a) in row.answers() {
            files.push(repo_root().join(row.witness_of(a)));
        }
    }
    files.sort();
    files.dedup();
    files.retain(|f| slow_too || !slow.contains(f));
    files
}

/// The layer of the program at `path`, at the default budget.
fn layer_of(path: &Path, opts: &structure::Options) -> PairLayer {
    let (mut kb, mut terms, ast) = loaded(path).expect("the program loads");
    structure::pair_layer(&mut kb, &mut terms, &ast, opts).expect("the layer")
}

fn instance(rel: &str) -> PairLayer {
    layer_of(&repo_root().join(rel), &structure::Options::default())
}

/// A pair, as its two candidates' texts in order.
type Pair = (String, String);

fn pairs_of(l: &PairLayer, pairs: impl Iterator<Item = (u32, u32)>) -> BTreeSet<Pair> {
    pairs
        .map(|(a, b)| {
            let (x, y) = (l.text(a).to_string(), l.text(b).to_string());
            if x <= y { (x, y) } else { (y, x) }
        })
        .collect()
}

/// What one program's two computations said.
#[derive(Default)]
struct Held {
    failures: Vec<String>,
    /// The layer and an exhausted describe were both read.
    compared: bool,
    flagged: bool,
    root_dead: bool,
    excluded: usize,
    dead_alone: usize,
    /// The layer's budget stopped it where the describe's did not.
    stopped: bool,
}

/// Both claims for `path`; `None` where the program does not load, is
/// refused, or its describe does not exhaust.
fn held(path: &Path) -> Option<Held> {
    let rel = rel_of(path);
    let (mut kb, mut terms, ast) = loaded(path)?;
    let opts = explore::Options {
        cores: Cores::READ_OUT,
        ..explore::Options::new(Mode {
            which: Which::All,
            ask: Ask::Describe,
        })
    };
    let answer = explore::run(&mut kb, &mut terms, &ast, &opts).ok()?;
    let d = answer.described.as_deref()?;
    if !d.exhausted || d.nogoods_skipped {
        return None;
    }
    drop((kb, terms));
    let mut out = Held {
        compared: true,
        ..Held::default()
    };
    let (mut kb, mut terms, ast) = loaded(path)?;
    let l = match structure::pair_layer(&mut kb, &mut terms, &ast, &structure::Options::default()) {
        Ok(l) => l,
        Err(e) => {
            out.failures.push(format!(
                "{rel}: a describe answers and the layer does not: {e}"
            ));
            return Some(out);
        }
    };
    out.flagged = !l.monotone;
    out.root_dead = l.root_dead;
    out.excluded = l.excluded.len();
    out.dead_alone = l.dead_alone.len();
    if l.n() != d.n()
        || l.candidates
            .iter()
            .zip(&d.candidates)
            .any(|(a, b)| a.text != b.text)
    {
        out.failures.push(format!(
            "{rel}: the layer and the describe read two universes"
        ));
        return Some(out);
    }

    // N's members of two candidates, and the layer's dead pairs whose
    // members are each alive alone.
    let nogoods: BTreeSet<Pair> = pairs_of(
        &l,
        d.dead
            .iter()
            .filter(|x| x.set.len() == 2)
            .map(|x| (x.set[0], x.set[1])),
    );
    let mut dead_pairs = pairs_of(&l, l.excluded.iter().copied());
    dead_pairs.extend(pairs_of(
        &l,
        l.against_l3.iter().filter(|x| x.2).map(|&(a, b, _)| (a, b)),
    ));
    if !l.complete() {
        // A stopped layer is held to what it read: no pair it calls
        // excluded is outside N.
        out.stopped = true;
        for p in dead_pairs.difference(&nogoods) {
            out.failures.push(format!(
                "{rel}: the stopped layer excludes {{{}, {}}}, which is not in N",
                p.0, p.1
            ));
        }
        if l.singles_left > 0 {
            return Some(out);
        }
    } else {
        for p in dead_pairs.difference(&nogoods) {
            out.failures.push(format!(
                "{rel}: the layer excludes {{{}, {}}}, which is not in N",
                p.0, p.1
            ));
        }
        for p in nogoods.difference(&dead_pairs) {
            out.failures.push(format!(
                "{rel}: N holds {{{}, {}}}, which the layer does not exclude",
                p.0, p.1
            ));
        }
    }
    let alone: Vec<&str> = l.dead_alone.iter().map(|&b| l.text(b)).collect();
    let read_out: Vec<&str> = d.dead_alone_set.iter().map(|&b| l.text(b)).collect();
    if alone != read_out {
        out.failures.push(format!(
            "{rel}: the layer has {} dead alone, {alone:?}, and the read-out {}, {read_out:?}",
            alone.len(),
            read_out.len()
        ));
    }
    Some(out)
}

/// **The excluded pairs are `N`'s members of two candidates, and the
/// candidates dead alone are the read-out's**, on every corpus program whose
/// describe exhausts at the default budget.
#[test]
fn the_excluded_pairs_are_the_pairs_of_n() {
    // The corpus's `slow` entries too, where the run asks for them.
    let slow_too = std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1");
    let files = files(slow_too);
    let results = ein_corpus::pool::map(
        "pair_layer::held",
        &files,
        |p| (rel_of(p), String::new()),
        |p| held(p),
    );
    let mut failures: Vec<String> = Vec::new();
    let (mut compared, mut with_pairs, mut pairs, mut flagged, mut dead_roots, mut alone) =
        (0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
    let mut stopped: Vec<String> = Vec::new();
    for (file, got) in files.iter().zip(results) {
        let Some(h) = got else { continue };
        failures.extend(h.failures);
        compared += usize::from(h.compared);
        with_pairs += usize::from(h.excluded > 0);
        pairs += h.excluded;
        flagged += usize::from(h.flagged);
        dead_roots += usize::from(h.root_dead);
        alone += usize::from(h.dead_alone > 0 && !h.root_dead);
        if h.stopped {
            stopped.push(rel_of(file));
        }
    }
    assert!(
        failures.is_empty(),
        "{} disagreement(s) between the pair layer and N:\n{}",
        failures.len(),
        failures.join("\n")
    );
    // The describe exhausted at the default budget and the layer did not: a
    // program the layer costs more than its describe, named.
    assert!(
        stopped.is_empty(),
        "the layer's budget stopped it where the describe's did not: {stopped:?}"
    );
    eprintln!(
        "pair_layer: {compared} programs compared, {with_pairs} with an excluded pair \
         ({pairs} pairs), {alone} with a candidate dead alone over an alive root, \
         {flagged} flagged, {dead_roots} dead roots"
    );
    // The sweep is not vacuous: it compares programs of each kind.
    assert!(compared >= 300, "only {compared} programs compared");
    assert!(
        with_pairs >= 20,
        "only {with_pairs} programs with an excluded pair"
    );
    assert!(
        alone >= 20,
        "only {alone} programs with a candidate dead alone"
    );
    assert!(flagged >= 10, "only {flagged} flagged programs compared");
    assert!(dead_roots >= 10, "only {dead_roots} dead roots compared");
}

/// **The negative control for pairs**: `02_genuine_3set_death`'s `(false)`
/// needs all three of its candidates, so each is alive alone, none implies
/// another, no pair excludes, and the one member of `N` holds three.
#[test]
fn a_conflict_of_three_shows_no_excluded_pair() {
    let rel = "examples/lattice/02_genuine_3set_death.ein";
    let l = instance(rel);
    assert!(l.monotone && l.complete());
    assert_eq!(l.n(), 3);
    assert!(
        l.dead_alone.is_empty(),
        "three alive singles: {:?}",
        l.dead_alone
    );
    assert!(l.implies.is_empty(), "no implication: {:?}", l.implies);
    assert!(l.equivalent.is_empty());
    assert!(l.excluded.is_empty(), "no excluded pair: {:?}", l.excluded);
    assert_eq!(l.pairs.saturated, 3, "every pair is read");

    let (mut kb, mut terms, ast) = loaded(&repo_root().join(rel)).expect("loads");
    let opts = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    });
    let answer = explore::run(&mut kb, &mut terms, &ast, &opts).expect("a describe");
    let d = answer.described.expect("described");
    let n: Vec<&[u32]> = d.dead.iter().map(|x| &x.set[..]).collect();
    assert_eq!(n, [&[0u32, 1, 2][..]], "N is the triple");
}

/// A `(rel value house)` candidate's three names.
fn parts(text: &str) -> Vec<&str> {
    text.trim_matches(|c| c == '(' || c == ')')
        .split_whitespace()
        .collect()
}

/// **The declared control**: `zebra2.ein` declares its five `*-loc` relations
/// bijective, so two candidates of one relation with the same value or the
/// same house cannot hold together. Every such pair is dead at root: the
/// layer excludes it where its two members are each alive alone, and where
/// one is dead alone the pair is dead with it (L1) and is no exclusion. The
/// clues exclude more, across relations.
#[test]
fn a_declared_bijection_excludes_along_its_rows_and_columns() {
    let l = instance("examples/zebra2.ein");
    assert!(l.monotone && l.complete());
    let excluded: BTreeSet<(u32, u32)> = l.excluded.iter().copied().collect();
    let dead: BTreeSet<u32> = l.dead_alone.iter().copied().collect();
    let n = l.n() as u32;
    let (mut lines, mut with_dead) = (BTreeSet::new(), 0usize);
    let mut relations: BTreeSet<&str> = BTreeSet::new();
    for a in 0..n {
        for b in a + 1..n {
            let (x, y) = (parts(l.text(a)), parts(l.text(b)));
            assert!(x.len() == 3 && x[0].ends_with("-loc"), "{}", l.text(a));
            // One relation, and the same value or the same house, not both.
            if x[0] != y[0] || (x[1] == y[1]) == (x[2] == y[2]) {
                continue;
            }
            if dead.contains(&a) || dead.contains(&b) {
                with_dead += 1;
                assert!(!excluded.contains(&(a, b)));
                continue;
            }
            assert!(
                excluded.contains(&(a, b)),
                "{{{}, {}}} share a row or a column of {} and do not exclude",
                l.text(a),
                l.text(b),
                x[0]
            );
            lines.insert((a, b));
            relations.insert(x[0]);
        }
    }
    assert_eq!(
        relations.len(),
        5,
        "every relation has such a pair: {relations:?}"
    );
    assert!(with_dead > 0, "root leaves candidates that are dead alone");
    let more = excluded.difference(&lines).count();
    assert!(more > 0, "the clues exclude pairs no bijection does");
    eprintln!(
        "zebra2: {} excluded pairs, {} of them along a row or a column, {more} more; {with_dead} \
         row or column pairs dead with a member dead alone",
        excluded.len(),
        lines.len()
    );
}

/// What a layer says, for a comparison.
fn said(l: &PairLayer) -> String {
    format!(
        "{:?}",
        (
            (&l.at_root, &l.dead_alone, &l.implies, &l.equivalent),
            (&l.excluded, &l.interacting, &l.against_l1, &l.against_l3),
            (l.pairs, &l.licences, l.work, &l.stopped, l.singles_left),
        )
    )
}

/// **The job count moves nothing**: the layer of a monotone program and of a
/// flagged one, at one thread and at four.
#[test]
fn the_layer_is_the_same_at_any_job_count() {
    for rel in [
        "examples/zebra2-hints.ein",
        "examples/branching/08_hypothesis_relation_whitelist.ein",
        "examples/saturation/square-unique/cul-de-sac.ein",
    ] {
        let path = repo_root().join(rel);
        let one = layer_of(&path, &structure::Options::default());
        let four = layer_of(
            &path,
            &structure::Options {
                jobs: 4,
                ..structure::Options::default()
            },
        );
        assert!(one.pairs.saturated > 0, "{rel}: the layer reads pairs");
        assert_eq!(said(&one), said(&four), "{rel}");
    }
}

/// **A flagged program's every pair is saturated**, by no licence, and the
/// layer says what its statements are about.
#[test]
fn a_flagged_program_gets_no_licence() {
    let l = instance("examples/saturation/square-unique/cul-de-sac.ein");
    assert!(!l.monotone && l.complete());
    assert_eq!(l.domain(), Some(structure::FLAGGED));
    assert!(l.licences.is_empty());
    assert_eq!(l.pairs.saturated, l.pairs.all);
    assert_eq!(
        (l.pairs.dead_member, l.pairs.implied, l.pairs.left),
        (0, 0, 0)
    );
    assert!(
        !l.implies.is_empty(),
        "it has implications, and reads their pairs all the same"
    );
    assert_eq!(l.work.saturations, 1 + l.n() as u64 + l.pairs.all);

    assert_eq!(instance("examples/zebra2-hints.ein").domain(), None);
}

/// **A dead root decides the layer without a node** (L0): every candidate is
/// dead alone, as the read-out counts them, and no pair is an exclusion.
#[test]
fn a_dead_root_leaves_every_candidate_dead_alone() {
    let l = instance("examples/ein-bugs/alt-cap-core.ein");
    assert!(l.root_dead && l.complete());
    assert!(l.n() > 0);
    assert_eq!(l.dead_alone.len(), l.n());
    assert_eq!(l.pairs.dead_member, l.pairs.all);
    assert!(l.excluded.is_empty() && l.implies.is_empty());
    assert_eq!(l.work, structure::Work::default());
    assert_eq!(l.licences, [explore::Licence::ForkOnlyAdds]);
}

/// **A stopped layer says what it read and counts what it left**: at a
/// budget that reaches the singles and not every pair, each pair is decided
/// by a licence, saturated or left; what it excludes the whole layer
/// excludes; and at a budget that does not reach the singles it lists no
/// pair at all.
#[test]
fn a_stopped_layer_says_what_it_read() {
    let path = repo_root().join("examples/zebra2-hints.ein");
    let whole = layer_of(&path, &structure::Options::default());
    assert!(whole.complete() && whole.pairs.saturated > 0);
    let part = layer_of(
        &path,
        &structure::Options {
            budget: whole.work.facts / 2,
            ..structure::Options::default()
        },
    );
    assert!(
        matches!(part.stopped, Some(explore::Stopped::Budget { .. })),
        "{:?}",
        part.stopped
    );
    assert_eq!(part.singles_left, 0, "half the budget reaches every single");
    assert!(part.pairs.left > 0 && part.pairs.saturated > 0);
    assert_eq!(
        part.pairs.dead_member + part.pairs.implied + part.pairs.saturated + part.pairs.left,
        part.pairs.all
    );
    assert_eq!(
        part.pairs.saturated + part.pairs.left,
        whole.pairs.saturated
    );
    assert!(part.work.facts < whole.work.facts);
    assert_eq!(part.dead_alone, whole.dead_alone);
    assert_eq!(part.implies, whole.implies);
    let all: BTreeSet<(u32, u32)> = whole.excluded.iter().copied().collect();
    assert!(part.excluded.iter().all(|p| all.contains(p)));
    assert!(
        part.excluded.len() < whole.excluded.len()
            || part.interacting.len() < whole.interacting.len()
    );

    let none = layer_of(
        &path,
        &structure::Options {
            budget: 1,
            ..structure::Options::default()
        },
    );
    assert!(none.singles_left > 0 && !none.complete());
    assert_eq!(none.pairs.left, none.pairs.all);
    assert!(none.implies.is_empty() && none.excluded.is_empty());
}
