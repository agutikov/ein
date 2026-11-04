//! The detector — M1e2
//! [S1e2.1b.5](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.5_the_detector.md).
//!
//! What a program the static check flags is warned of, held to the corpus:
//! the static half ([`ein_infer::detector::Static`]) on every corpus program,
//! with each cycle and each order finding pinned by name.

use std::path::Path;

use ein_core::{FactId, Kb, Prov, Terms};
use ein_corpus::{Scratch, corpus_files, repo_root};
use ein_infer::canon::state_key;
use ein_infer::compile::SharedMemo;
use ein_infer::detector::{self, AtStake, Concludes, StaleCheck, Static, Status, Warning};
use ein_infer::events::Events;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::forbid;
use ein_infer::monotone::Monotonicity;
use ein_infer::region::{self, Nodes, Options, Outcome};
use ein_infer::saturator::{Saturator, Session};
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

/// A flagged program whose root is dead, over 17 candidates: an obligation's
/// trigger is its site, and `dead` fires at root.
const ROOT_DEAD_PAST_THE_LIMIT: &str = "
     (relation thing T) (relation h T) (relation need T T)
     (thing A) (thing B) (thing C) (thing D) (thing E) (thing F) (thing G) (thing H)
     (thing I) (thing J) (thing K) (thing L) (thing M) (thing N) (thing O) (thing P)
     (thing Q)
     (need K1 A)
     (hrule guess () :match (thing ?x) :assert (h ?x))
     (rule dead () :match (thing A) :assert (false))
     (rule owe () :match (and (h ?x) (absent (and (need K1 ?y) (h ?y)))) :assert (open h))
     (query :goal (h ?x))";

/// The static check of `text` (read from `at`), through the oracle's own
/// preparation — the cube is refused past the limit and the check still runs.
/// Read past *forbid*'s refusal (M1e2 S1e2.2.5), which comes after the check:
/// the static half is the program's, refused or not.
fn checked(text: &str, at: &Path) -> Option<(Monotonicity, Terms)> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, at.to_str()).ok()?;
    let mut kb = load(&mut ast, &mut terms, &forms, at.parent()).ok()?;
    let opts = Options {
        limit: 0,
        ..Options::default()
    };
    let out = forbid::studying(|| region::compute(&mut kb, &mut terms, &ast, &opts, &mut |_| {}));
    let m = match out.ok()? {
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
    // 24 since M1e2 S1e2.2.5 (2026-11-04): `features/05` and `/12` close the
    // relation their `std.elim`-shaped refutations read, which *forbid* asked
    // of them, and are monotone; the programs it refuses keep their static
    // half, which `checked` reads before the refusal. 26 since S1e2.2.15
    // banked W14's `oblig-stdlib-total-owed` pair the same day, each an
    // `obligation-trigger` site on `is-a`. 27 since S1e2.2.12 promoted the
    // fuzz finding `d3-unsat-core` into the corpus the same day, which the
    // static check flags and *forbid* refuses (a `via` program). 30 since
    // S1e2.2.16 banked W16's second shape the same day, three programs with an
    // `absent` site and an `activator` site each: the symbol control and the
    // int and nested witnesses, all three refused by *forbid* since W16's
    // fix, which keep their static half as the refused programs above do. 38
    // since S1e2.2.17 banked W17's family the same day: its five witnesses,
    // its two controls (`forbid-chain-0`, refused, and `-no-absent`) and
    // `lattice/12_activator_chain_hole.ein`, each with an `activator` site.
    assert_eq!(flagged, 38, "the flagged corpus programs");
    assert_eq!(
        found_cycles,
        [
            "examples/ein-bugs/d3-unsat-core.ein odd: fire-1: (not r1) →¬ r1",
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
/// programs `node_census.md` § 5 found it on; `d3-unsat-core`, a fuzz
/// finding, is a corpus entry since M1e2 S1e2.2.12 promoted it (2026-11-04).
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
    let (m, _) = file("examples/ein-bugs/d3-unsat-core.ein");
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
/// `--region-limit`. Studied past *forbid*'s refusal (M1e2 S1e2.2.5): the
/// warnings these tests hold are on the shapes it refuses, the dead hole and
/// the stale fact, as a library-level fixture.
fn warned(rel: &str, mode: Mode, limit: u32) -> Warning {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("parses");
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("loads");
    let mut opts = explore::Options::new(mode);
    opts.limit = limit;
    let r = forbid::studying(|| ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts))
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
        // M1e2 S1e2.2.17: where S1e2.1b.3's rule names no site, the
        // derivation followed through each plan's activator. `lattice/12`'s
        // `total-owed` activator is derived by a plan of `serves`, which root
        // never compiled; W17's witnesses, studied past *forbid*, die through
        // `r2`, `r3` or `bad`, compiled at a node, none of them a site.
        (
            "examples/lattice/12_activator_chain_hole.ein",
            exercised(&["serves"], 1, 0, 1),
        ),
        (
            "examples/ein-bugs/wrong-answers/forbid-chain-1.ein",
            exercised(&["q"], 1, 1, 0),
        ),
        (
            "examples/ein-bugs/wrong-answers/forbid-chain-2.ein",
            exercised(&["q"], 1, 1, 0),
        ),
        (
            "examples/ein-bugs/wrong-answers/forbid-chain-node-absent.ein",
            exercised(&["q"], 1, 1, 0),
        ),
        (
            "examples/ein-bugs/wrong-answers/forbid-chain-param-absent.ein",
            exercised(&["watch"], 1, 1, 0),
        ),
        // ... and where that names none either, the row says so: `chk`'s death
        // rests on `(absent (x a))`, no site reads `x`, and no rule on the
        // derivation is an activator site.
        (
            "examples/ein-bugs/wrong-answers/forbid-chain-node-writes.ein",
            exercised(&[], 1, 1, 0),
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
    assert_eq!(
        ein_infer::detector::by_words(&[]),
        "by no site: the violation's derivation, each plan's activator followed, reaches none"
    );
    assert_eq!(
        ein_infer::detector::by_words(&["q".to_string(), "r".to_string()]),
        "by q, r"
    );
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
    // `features/12` was the corpus's one flagged program with a dead root past
    // the limit until M1e2 S1e2.2.5 closed its `instance`; a written one, an
    // obligation's trigger beside a refutation root fires, holds the status.
    let path = repo_root().join("written.ein");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, ROOT_DEAD_PAST_THE_LIMIT, path.to_str()).expect("parses");
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("loads");
    let mut opts = explore::Options::new(DESCRIBE);
    opts.limit = region::DEFAULT_LIMIT;
    let r = ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts).expect("it answers");
    assert_eq!(
        r.answer.warning.expect("a warning").status,
        Status::RootDead { n: 17, limit: 16 }
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
            "examples/ein-bugs/d3-unsat-core.ein",
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

// ── (c): robust, on the corpus ──────────────────────────────────────

/// The bound on `n` for the split sweep: every split `C1 ⊆ C` of every node
/// is `3ⁿ` saturations a program, so `n ≤ 10` is at most 59 049. Measured
/// 2026-11-03: 147 monotone corpus cubes, 1 941 nodes, 37 172 splits, 4.8 s in
/// a release build and 6.0 s in the gate's test profile. The eight monotone
/// cubes at `n = 11…16` are past it; `node_census -- --paths --control
/// --max-n 12` reads to 12 (`node_census.md` § 11.4).
const SPLIT_BOUND: usize = 10;

/// `sat(P ∪ C)` with `P` the program as loaded and not saturated: a node
/// built **from scratch**, `node_census`'s step from the base.
fn scratch(base: &mut Kb, terms: &mut Terms, ast: &Ast, facts: &[FactId]) -> Box<[FactId]> {
    let mut kb = base.fork();
    terms.provs.open_fork();
    for &h in facts {
        let (rel, args) = terms.facts.get(h);
        let args = args.to_vec();
        let prov = terms.provs.push(Prov::from_hypothesis(0, None));
        kb.add_and_index_fact(terms, rel, &args, Some(prov))
            .expect("room for a hypothesis");
    }
    {
        let mut events = Events::off();
        let mut s = Session {
            kb: &mut kb,
            terms,
            ast,
            events: &mut events,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).expect("compiles");
        sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
    }
    let key = state_key(&kb);
    terms.provs.discard_fork();
    key
}

/// What one program's sweep found.
#[derive(Default)]
struct Swept {
    nodes: u64,
    splits: u64,
    /// `node: fork ≠ split from C1`.
    split_differs: Vec<String>,
    /// `node` where scratch is not the fork.
    scratch_differs: Vec<String>,
    /// `node: fact`, a stale fact in a fork-built node.
    stale: Vec<String>,
}

/// Every node of a monotone program's cube built by fork, by every split
/// `sat(sat(root ∪ C1) ∪ (C \\ C1))` and from scratch, and read for stale
/// facts — `None` when the program is flagged, refused, or past `bound`.
fn sweep(text: &str, at: &Path, bound: usize) -> Option<Swept> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, at.to_str()).ok()?;
    let mut kb = load(&mut ast, &mut terms, &forms, at.parent()).ok()?;
    let mut base = kb.fork();
    let opts = Options {
        limit: bound as u32,
        ..Options::default()
    };
    let Ok(Outcome::Region(r)) = region::compute(&mut kb, &mut terms, &ast, &opts, &mut |_| {})
    else {
        return None;
    };
    if !r.monotonicity.is_monotone() {
        return None;
    }
    let n = r.n();
    let facts = |mask: u64| -> Vec<FactId> {
        (0..n)
            .filter(|b| mask >> b & 1 == 1)
            .map(|b| r.candidates[b].fact)
            .collect()
    };
    let mut out = Swept::default();
    let mut keys: Vec<Box<[FactId]>> = Vec::with_capacity(1 << n);
    {
        let mut nodes = Nodes::new(&mut kb, &mut terms, &ast, &r.candidates).ok()?;
        let check = StaleCheck::of(&nodes.plans());
        for mask in 0..1u64 << n {
            let (key, stale) = nodes
                .with_fixpoint(&facts(mask), |kb, terms| {
                    (
                        state_key(kb),
                        detector::stale_facts(kb, terms, &ast, &check),
                    )
                })
                .ok()?;
            out.nodes += 1;
            out.stale
                .extend(stale.into_iter().map(|f| format!("{}: {f}", r.name(mask))));
            let mut sub = mask;
            while sub != 0 {
                sub = (sub - 1) & mask;
                if sub == 0 {
                    break;
                }
                let k = nodes.by_split(&facts(sub), &facts(mask & !sub)).ok()?;
                out.splits += 1;
                if k != key {
                    out.split_differs
                        .push(format!("{} from {}", r.name(mask), r.name(sub)));
                }
            }
            keys.push(key);
        }
    }
    for mask in 0..1u64 << n {
        if scratch(&mut base, &mut terms, &ast, &facts(mask)) != keys[mask as usize] {
            out.scratch_differs.push(r.name(mask));
        }
    }
    Some(out)
}

/// **(c) 2 and 3: a *Monotone* verdict means one state per node, whatever
/// built it, and the dynamic detector has nothing to find.** On every
/// monotone corpus cube with `n ≤` [`SPLIT_BOUND`], every node built by fork
/// is the node built by every split from a fixpoint that holds root's, fact
/// for fact, and holds no stale fact — so no parent path can differ and the
/// stale check would read nothing, which is why the detector is gated on the
/// verdict (`detector.rs`'s module docs). **From scratch the verdict does not
/// reach**: on `examples/lattice/11_fifo_gap_static.ein` an equal-priority
/// race over a static read leaves `{(b one)}` a stale `(p one)` that the fork
/// never holds (`monotonicity.md` § 7.6), and it is the one corpus node
/// where scratch departs.
#[test]
fn a_monotone_verdict_is_one_state_per_node_by_fork_and_by_every_split() {
    let (mut programs, mut nodes, mut splits) = (0, 0, 0);
    let mut failures: Vec<String> = Vec::new();
    let mut scratch_differs: Vec<String> = Vec::new();
    for path in corpus_files() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .display()
            .to_string();
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Some(s) = sweep(&text, &path, SPLIT_BOUND) else {
            continue;
        };
        programs += 1;
        nodes += s.nodes;
        splits += s.splits;
        failures.extend(s.split_differs.iter().map(|d| format!("{rel}: split {d}")));
        failures.extend(s.stale.iter().map(|d| format!("{rel}: stale {d}")));
        scratch_differs.extend(s.scratch_differs.iter().map(|d| format!("{rel} {d}")));
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert_eq!(
        scratch_differs,
        ["examples/lattice/11_fifo_gap_static.ein {(b one)}"],
        "scratch departs from the fork on exactly the static FIFO race"
    );
    eprintln!("{programs} monotone cubes, {nodes} nodes, {splits} splits");
    assert!(programs > 140 && nodes > 1900, "the sweep is not vacuous");
}

/// **(c) 2 and 3 on random programs**: `utils/region_synth.py --random`
/// writes a program for a random region, as `region_dump.rs`'s round trip
/// draws them per commit (whose check holds (c) 1 on them: a *Monotone*
/// verdict beside a cube violation fails it). Here every one the static
/// check calls monotone is swept as the corpus is — by fork, by every split,
/// from scratch, and read for stale facts — and none departs: no random
/// program races two `absent` rules over a static read.
#[test]
fn a_monotone_verdict_is_one_state_per_node_on_random_programs() {
    let tmp = Scratch::new("ein-detector-random");
    let dir = tmp.join("d");
    let out = std::process::Command::new("python3")
        .arg("-I")
        .arg(repo_root().join("utils/region_synth.py"))
        .args(["--random", "200", "--seed", "5", "--max-n", "6", "--write"])
        .arg(&dir)
        .output()
        .expect("python3 runs utils/region_synth.py — the gate needs python3");
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let (mut programs, mut nodes) = (0, 0);
    let mut failures: Vec<String> = Vec::new();
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("written")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    files.sort();
    for path in &files {
        let text = std::fs::read_to_string(path).expect("written");
        let Some(s) = sweep(&text, path, 6) else {
            continue;
        };
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        programs += 1;
        nodes += s.nodes;
        failures.extend(s.split_differs.iter().map(|d| format!("{name}: split {d}")));
        failures.extend(s.stale.iter().map(|d| format!("{name}: stale {d}")));
        failures.extend(
            s.scratch_differs
                .iter()
                .map(|d| format!("{name}: scratch {d}")),
        );
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    eprintln!(
        "{programs} monotone random programs of {}, {nodes} nodes",
        files.len()
    );
    assert!(
        programs >= 50,
        "only {programs} random programs were monotone"
    );
}
