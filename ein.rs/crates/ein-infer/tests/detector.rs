//! The detector — M1e2
//! [S1e2.1b.5](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.5_the_detector.md).
//!
//! What a program the static check flags is warned of, held to the corpus:
//! the static half ([`ein_infer::detector::Static`]) on every corpus program,
//! with each cycle and each order finding pinned by name.

use std::path::Path;

use ein_core::Terms;
use ein_corpus::{corpus_files, repo_root};
use ein_infer::detector::{AtStake, Concludes, Static, Status, Warning};
use ein_infer::explore::{self, Ask, Mode, Which};
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

/// The warning a run of `mode` gives on `rel`, read-out and all — `limit` is
/// `--region-limit`.
fn warned(rel: &str, mode: Mode, limit: u32) -> Warning {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("parses");
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("loads");
    let mut opts = explore::Options::new(mode);
    opts.limit = limit;
    let r = ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts)
        .unwrap_or_else(|e| panic!("{rel}: {e}"));
    *r.answer
        .warning
        .unwrap_or_else(|| panic!("{rel}: no warning"))
}

const DESCRIBE: Mode = Mode {
    which: Which::All,
    ask: Ask::Describe,
};

fn stale(w: &Warning) -> Vec<String> {
    w.state
        .stale
        .iter()
        .map(|s| format!("{} {}: {}", s.node, s.class.as_str(), s.facts.join(" · ")))
        .collect()
}

/// **The stale check reads every node the run computes** — the proposal's
/// § 4.1, approved: on W8's own witness the stale fact is in a dead node no
/// run lists. One per kind the proposal's § 4.2 shows: dead, in R, and the
/// FIFO gap's node of R; none where the state never breaks.
#[test]
fn the_stale_check_reads_every_node_the_run_computes() {
    let cases: &[(&str, &[&str])] = &[
        (
            "examples/ein-bugs/wrong-answers/stale-naf-fork.ein",
            &["{(q one)} dead: (p one) ← derive-p (absent (q one))"],
        ),
        (
            "examples/features/07_unstratifiable.ein",
            &["{(q one)} R: (p one) ← derive-p (absent (q one))"],
        ),
        (
            "examples/lattice/10_fifo_gap.ein",
            &["{(a one), (b one)} R: (p one) ← r2 (absent (q one))"],
        ),
        ("examples/ein-bugs/naf-upward-closure.ein", &[]),
        ("examples/lattice/08_absent_witness_hole.ein", &[]),
    ];
    for (rel, want) in cases {
        let w = warned(rel, DESCRIBE, region::DEFAULT_LIMIT);
        assert!(w.state.cube, "{rel}");
        assert_eq!(stale(&w), *want, "{rel}");
    }
    // A question decided early reads what it computed, and says how many.
    let w = warned(
        "examples/ein-bugs/wrong-answers/stale-naf-fork.ein",
        Mode {
            which: Which::All,
            ask: Ask::Find(1),
        },
        region::DEFAULT_LIMIT,
    );
    assert_eq!(w.state.nodes, 1);
    assert!(!w.state.cube);
    assert_eq!(w.state.stale, []);
}

/// **The status, attributed** — the cube's half, with the sites its
/// violations are attributed to by S1e2.1b.3's rule; and *not known* where
/// the run did not read the cube, said three ways.
#[test]
fn the_status_names_the_sites_its_violations_are_attributed_to() {
    let exercised = |by: &[&str], holes, dead, discharged| Status::Exercised {
        by: by.iter().map(|s| s.to_string()).collect(),
        holes,
        dead_not_upward_closed: dead,
        discharged_not_upward_closed: discharged,
    };
    let cases: &[(&str, Status)] = &[
        (
            "examples/ein-bugs/naf-upward-closure.ein",
            exercised(&["bad"], 1, 1, 0),
        ),
        (
            "examples/lattice/08_absent_witness_hole.ein",
            exercised(&["soup-unless-allergic"], 1, 0, 1),
        ),
        (
            "examples/lattice/04_obligation_trigger_hole.ein",
            exercised(&["total-owed [activator meal is-a]"], 1, 0, 1),
        ),
        (
            "examples/lattice/09_activator_hole.ein",
            exercised(&["total-owed"], 1, 0, 1),
        ),
        (
            "examples/lattice/synth/trigger_hole.ein",
            exercised(&["dead-1", "owe-2"], 7, 3, 4),
        ),
        (
            "examples/branching/13_lookahead_naf_world.ein",
            Status::Conservative,
        ),
        ("examples/lattice/10_fifo_gap.ein", Status::Conservative),
    ];
    for (rel, want) in cases {
        assert_eq!(
            warned(rel, DESCRIBE, region::DEFAULT_LIMIT).status,
            *want,
            "{rel}"
        );
    }
    let rel = "examples/ein-bugs/naf-upward-closure.ein";
    assert_eq!(
        warned(rel, DESCRIBE, 1).status,
        Status::PastLimit { n: 2, limit: 1 }
    );
    // `-n 1` stops at ∅, a node of R, so it read one node of four.
    let first = Mode {
        which: Which::All,
        ask: Ask::Find(1),
    };
    assert_eq!(
        warned(rel, first, region::DEFAULT_LIMIT).status,
        Status::NotRead { n: 2 }
    );
    // `--single` on `stale-naf-fork` reads both of its nodes: what the cube
    // says, from a question decided early.
    let single = Mode {
        which: Which::All,
        ask: Ask::Single,
    };
    let w = warned(
        "examples/ein-bugs/wrong-answers/stale-naf-fork.ein",
        single,
        region::DEFAULT_LIMIT,
    );
    assert_eq!(w.status, Status::Conservative);
    assert!(w.state.cube);
    // A dead root kills every node (L0), and the route reads root alone.
    assert_eq!(
        warned(
            "examples/features/12_expect_false.ein",
            DESCRIBE,
            region::DEFAULT_LIMIT
        )
        .status,
        Status::RootDead { n: 20, limit: 16 }
    );
}

/// **The parent-path spot check**, at ≤ 3 parents of each printed state of
/// two commitments or more — the proposal's § 5 table, every row of it on
/// the corpus. A dead parent is skipped, as `-y` skipped one.
#[test]
fn the_spot_check_reads_three_parents_of_each_printed_state() {
    // (states, parents, dead parents, saturations, the paths that differ)
    type Row<'a> = (&'a str, u64, u64, u64, u64, &'a [&'a str]);
    let cases: &[Row] = &[
        ("examples/ein-bugs/naf-upward-closure.ein", 1, 1, 1, 3, &[]),
        (
            "examples/lattice/08_absent_witness_hole.ein",
            3,
            7,
            0,
            14,
            &[
                "{(allergic Ann), (meal Ann Bread), (seats Ann C1)} by way of its parent without (allergic Ann)",
            ],
        ),
        (
            "examples/lattice/04_obligation_trigger_hole.ein",
            1,
            2,
            0,
            4,
            &[],
        ),
        ("examples/lattice/09_activator_hole.ein", 1, 2, 0, 4, &[]),
        (
            "corpus/fuzz_findings/d3-unsat-core.ein",
            14,
            34,
            0,
            68,
            &[
                "{(r1 o2 o3), (r1 o3 o2)} by way of its parent without (r1 o3 o2)",
                "{(is-a o2 o3), (r1 o2 o3), (r1 o3 o2)} by way of its parent without (r1 o3 o2)",
                "{(is-a o3 o2), (r1 o2 o3), (r1 o3 o2)} by way of its parent without (r1 o3 o2)",
            ],
        ),
        (
            "examples/lattice/10_fifo_gap.ein",
            1,
            2,
            0,
            4,
            &["{(a one), (b one)} by way of its parent without (b one)"],
        ),
        ("examples/lattice/synth/trigger_hole.ein", 8, 21, 1, 43, &[]),
        (
            "examples/ein-bugs/wrong-answers/stale-naf-fork.ein",
            0,
            0,
            0,
            0,
            &[],
        ),
        (
            "examples/branching/13_lookahead_naf_world.ein",
            0,
            0,
            0,
            0,
            &[],
        ),
    ];
    for &(rel, states, parents, dead, saturations, differ) in cases {
        let p = warned(rel, DESCRIBE, region::DEFAULT_LIMIT)
            .paths
            .unwrap_or_else(|| panic!("{rel}: no spot check"));
        assert_eq!(
            (p.states, p.parents, p.dead, p.saturations),
            (states, parents, dead, saturations),
            "{rel}"
        );
        assert_eq!(p.differ, differ, "{rel}");
        assert_eq!(p.unprinted, None, "{rel}");
    }
}
