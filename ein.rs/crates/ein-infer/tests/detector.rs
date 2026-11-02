//! The detector — M1e2
//! [S1e2.1b.5](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.5_the_detector.md).
//!
//! What a program the static check flags is warned of, held to the corpus:
//! the static half ([`ein_infer::detector::Static`]) on every corpus program,
//! with each cycle and each order finding pinned by name.

use std::path::Path;

use ein_core::Terms;
use ein_corpus::{corpus_files, repo_root};
use ein_infer::detector::{AtStake, Concludes, Static};
use ein_infer::monotone::Monotonicity;
use ein_infer::region::{self, Options, Outcome};
use ein_ir::{Ast, load, parse};

/// `stratified_stale`, `node_census`'s written program (`node_census.md`
/// § 2.3): stratified (r < q < p), one priority, `derive-p` written first, so
/// root holds a stale `(p one)` beside `(q one)` from scratch, and swapping
/// the two rules' text changes the universe (§ 7.1 there).
const STRATIFIED_STALE: &str = "(relation thing T) (relation p T) (relation q T) (relation r T)
     (thing one)
     (rule derive-p () :match (and (thing ?x) (absent (q ?x))) :assert (p ?x) :priority 200)
     (rule derive-q () :match (and (thing ?x) (absent (r ?x))) :assert (q ?x) :priority 200)
     (query :goal (r ?x))";

/// `lattice/10_fifo_gap.ein` with the reader `r2` at priority 100: it is
/// admitted before `r1` in every written order, so no equal-priority pair
/// exists for a reorder to move, and the order check still reports it (the
/// proposal's § 7, item 4).
const FIFO_GAP_PRIORITY: &str = "(relation thing T) (relation a T) (relation b T)
     (relation p T) (relation q T) (relation z T)
     (thing one)
     (hrule guess-a () :match (thing ?x) :assert (a ?x))
     (hrule guess-b () :match (thing ?x) :assert (b ?x))
     (rule r2 () :match (and (a ?x) (b ?x) (absent (q ?x))) :assert (p ?x) :priority 100)
     (rule r1 () :match (and (a ?x) (absent (z ?x))) :assert (q ?x) :priority 200)
     (query :goal (p ?x))";

/// The static check of `text` (read from `at`), through the oracle's own
/// preparation — the cube is refused past the limit and the check still runs.
fn checked(text: &str, at: &Path) -> Option<(Monotonicity, Terms)> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, at.to_str()).ok()?;
    let mut kb = load(&mut ast, &mut terms, &forms, at.parent()).ok()?;
    let opts = Options {
        limit: 0,
        ..Options::default()
    };
    let m = match region::compute(&mut kb, &mut terms, &ast, &opts, &mut |_| {}).ok()? {
        Outcome::Region(r) => r.monotonicity,
        Outcome::Refused { monotonicity, .. } => monotonicity,
    };
    Some((m, terms))
}

fn file(rel: &str) -> (Monotonicity, Terms) {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    checked(&text, &path).unwrap_or_else(|| panic!("{rel}: does not load"))
}

fn written(text: &str) -> (Monotonicity, Terms) {
    checked(text, &repo_root().join("written.ein")).expect("loads")
}

/// `reader ← writer @ p/q: relations`, one per order finding.
fn races(s: &Static) -> Vec<String> {
    s.order
        .iter()
        .map(|r| {
            format!(
                "{} ← {} @ {}/{}: {}",
                r.reader,
                r.writer,
                r.reader_priority,
                r.writer_priority,
                r.relations.join(" ")
            )
        })
        .collect()
}

/// `odd|even: negative edges`, one per cycle.
fn cycles(s: &Static) -> Vec<String> {
    s.cycles
        .iter()
        .map(|c| {
            format!(
                "{}: {}",
                if c.odd { "odd" } else { "even" },
                c.negative.join(" · ")
            )
        })
        .collect()
}

/// **The gate, on the corpus.** A program the static check calls monotone
/// gets an empty static half, and a flagged one a stake per site. Every cycle
/// through an `absent` over `D` and every order finding on the corpus is named
/// here, so a new one fails until it is read.
#[test]
fn the_static_half_is_empty_on_a_monotone_program_and_named_on_a_flagged_one() {
    let mut found_cycles: Vec<String> = Vec::new();
    let mut found_races: Vec<String> = Vec::new();
    let mut flagged = 0;
    for path in corpus_files() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .display()
            .to_string();
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Some((m, _)) = checked(&text, &path) else {
            continue;
        };
        if m.is_monotone() {
            assert_eq!(*m.detector, Static::default(), "{rel}");
            continue;
        }
        flagged += 1;
        assert_eq!(m.detector.stakes.len(), m.sites.len(), "{rel}");
        for c in cycles(&m.detector) {
            found_cycles.push(format!("{rel} {c}"));
        }
        for r in races(&m.detector) {
            found_races.push(format!("{rel} {r}"));
        }
    }
    assert_eq!(flagged, 26, "the flagged corpus programs");
    assert_eq!(
        found_cycles,
        [
            "examples/ein-bugs/wrong-answers/f07-fpoff.ein even: derive-p: q →¬ p · derive-q: p →¬ q",
            "examples/features/07_unstratifiable.ein even: derive-p: q →¬ p · derive-q: p →¬ q",
        ]
    );
    assert_eq!(
        found_races,
        [
            "examples/ein-bugs/wrong-answers/f07-fpoff.ein derive-p ← derive-q @ 200/200: q",
            "examples/ein-bugs/wrong-answers/f07-fpoff.ein derive-q ← derive-p @ 200/200: p",
            "examples/features/07_unstratifiable.ein derive-p ← derive-q @ 200/200: q",
            "examples/features/07_unstratifiable.ein derive-q ← derive-p @ 200/200: p",
            "examples/lattice/10_fifo_gap.ein r2 ← r1 @ 200/200: q",
        ]
    );
}

/// The cycle through an `absent` over `D`, with its parity, on the three
/// programs `node_census.md` § 5 found it on; `d3-unsat-core` is a fuzz
/// finding with no corpus entry.
#[test]
fn a_cycle_through_an_absent_is_found_with_its_parity() {
    for rel in [
        "examples/features/07_unstratifiable.ein",
        "examples/ein-bugs/wrong-answers/f07-fpoff.ein",
    ] {
        let (m, _) = file(rel);
        assert_eq!(
            cycles(&m.detector),
            ["even: derive-p: q →¬ p · derive-q: p →¬ q"],
            "{rel}"
        );
    }
    let (m, _) = file("corpus/fuzz_findings/d3-unsat-core.ein");
    assert_eq!(cycles(&m.detector), ["odd: fire-1: (not r1) →¬ r1"]);
}

/// **Order sensitivity has no false negative** on the programs whose state or
/// universe moves with the written order of two `absent` rules
/// (`node_census.md` § 7.1: `features/07`, `f07-fpoff`, the written
/// `stratified_stale`), on the FIFO-gap fixture (§ 12.1), and on the reader at
/// an earlier priority, which no reorder moves.
#[test]
fn the_order_check_finds_every_fifo_race() {
    for rel in [
        "examples/features/07_unstratifiable.ein",
        "examples/ein-bugs/wrong-answers/f07-fpoff.ein",
    ] {
        let (m, _) = file(rel);
        assert_eq!(
            races(&m.detector),
            [
                "derive-p ← derive-q @ 200/200: q",
                "derive-q ← derive-p @ 200/200: p"
            ],
            "{rel}"
        );
    }
    let (m, _) = file("examples/lattice/10_fifo_gap.ein");
    assert_eq!(races(&m.detector), ["r2 ← r1 @ 200/200: q"]);
    assert!(m.detector.order[0].at_one_priority());
    let (m, _) = written(STRATIFIED_STALE);
    assert_eq!(races(&m.detector), ["derive-p ← derive-q @ 200/200: q"]);
    let (m, _) = written(FIFO_GAP_PRIORITY);
    assert_eq!(races(&m.detector), ["r2 ← r1 @ 100/200: q"]);
    assert!(!m.detector.order[0].at_one_priority());
    // The static variant: `q` is outside D, so the program is monotone and
    // the gated detector reads nothing (monotonicity.md § 7.6).
    let (m, _) = file("examples/lattice/11_fifo_gap_static.ein");
    assert!(m.is_monotone());
    assert_eq!(*m.detector, Static::default());
}

/// What each kind of site concludes and puts at stake, one fixture per cause
/// (`monotonicity.md` § 7.9's c2, c4, c5, c6).
#[test]
fn each_site_names_what_it_concludes_and_puts_at_stake() {
    let cases: &[(&str, Concludes, AtStake)] = &[
        (
            "examples/ein-bugs/naf-upward-closure.ein",
            Concludes::False,
            AtStake::StateAndDead,
        ),
        (
            "examples/lattice/08_absent_witness_hole.ein",
            Concludes::Positive,
            AtStake::State,
        ),
        (
            "examples/lattice/04_obligation_trigger_hole.ein",
            Concludes::Open,
            AtStake::Owes,
        ),
        (
            "examples/lattice/09_activator_hole.ein",
            Concludes::NoRootPlan,
            AtStake::OwesThroughPlan,
        ),
    ];
    for (rel, concludes, at_stake) in cases {
        let (m, t) = file(rel);
        let sites: Vec<String> = m.sites.iter().map(|s| s.render(&t)).collect();
        assert_eq!(m.detector.stakes.len(), 1, "{rel}: {sites:?}");
        assert_eq!(m.detector.stakes[0].concludes, *concludes, "{rel}");
        assert_eq!(m.detector.stakes[0].at_stake, *at_stake, "{rel}");
    }
}
