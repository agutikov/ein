//! The detector
//! ([08-monotonicity.md § 10](../../../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md#10-the-detector)).
//!
//! What a program the static check flags is warned of, held to the corpus:
//! the static half ([`ein_infer::detector::Static`]) on every corpus program,
//! with each cycle and each order finding pinned by name. The dynamic half,
//! what the cube of a flagged program shows (its status, its stale facts,
//! its parent paths), is an answer of a flagged program and is held to
//! nothing (Q-M1f.564); on a monotone program it has nothing to find, which
//! the sweeps at the end hold.

use std::path::Path;

use ein_core::{FactId, Kb, Prov, Terms};
use ein_corpus::{corpus_files, repo_root};
use ein_infer::canon::state_key;
use ein_infer::compile::SharedMemo;
use ein_infer::detector::{self, AtStake, Concludes, StaleCheck, Static};
use ein_infer::events::Events;
use ein_infer::monotone::Monotonicity;
use ein_infer::region::{self, Nodes, Options, Outcome};
use ein_infer::saturator::{Saturator, Session};
use ein_ir::{Ast, load, parse};

/// `stratified_stale`, a program the `node_census` example writes too:
/// stratified (r < q < p), `derive-p` written first, so root holds a stale
/// `(p one)` beside `(q one)` from scratch, and swapping the two rules' text
/// changes the universe.
const STRATIFIED_STALE: &str = "(relation thing T :why \"{?3} is a thing\") (relation p T :why \"p holds of {?3}\") (relation q T :why \"q holds of {?3}\") (relation r T :why \"r holds of {?3}\")
     (thing one)
     (rule derive-p () :match (and (thing ?x) (absent (q ?x))) :assert (p ?x) :why \"{?x} is a thing and q does not hold of it, so p holds of {?x}\")
     (rule derive-q () :match (and (thing ?x) (absent (r ?x))) :assert (q ?x) :why \"{?x} is a thing and r does not hold of it, so q holds of {?x}\")
     (query :goal (r ?x))";

/// `lattice/10_fifo_gap.ein`'s program, written here with its own `:why`s:
/// `r2` written first, and its race is the written order's, as the file's is.
const FIFO_GAP_WRITTEN: &str = "(relation thing T :why \"{?3} is a thing\") (relation a T :why \"a holds of {?3}\") (relation b T :why \"b holds of {?3}\")
     (relation p T :why \"p holds of {?3}\") (relation q T :why \"q holds of {?3}\") (relation z T :why \"z holds of {?3}\")
     (thing one)
     (hrule guess-a () :match (thing ?x) :assert (a ?x) :why \"guess that a holds of {?x}, a thing\")
     (hrule guess-b () :match (thing ?x) :assert (b ?x) :why \"guess that b holds of {?x}, a thing\")
     (rule r2 () :match (and (a ?x) (b ?x) (absent (q ?x))) :assert (p ?x) :why \"a and b hold of {?x} and q does not, so p holds of {?x}\")
     (rule r1 () :match (and (a ?x) (absent (z ?x))) :assert (q ?x) :why \"a holds of {?x} and z does not, so q holds of {?x}\")
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
    let out = region::compute(&mut kb, &mut terms, &ast, &opts, &mut |_| {});
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

/// `reader ← writer: relations`, one per order finding. No rule has a
/// priority, so every race is the written order's.
fn races(s: &Static) -> Vec<String> {
    s.order
        .iter()
        .map(|r| format!("{} ← {}: {}", r.reader, r.writer, r.relations.join(" ")))
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
    // Every corpus program the static check flags. A program the check flags
    // moves this count when it joins the corpus, and one it no longer flags
    // when it changes.
    assert_eq!(flagged, 47, "the flagged corpus programs");
    assert_eq!(
        found_cycles,
        [
            "examples/ein-bugs/d3-unsat-core.ein odd: fire-1: (not r1) →¬ r1",
            "examples/ein-bugs/wrong-answers/f07-fpoff.ein even: derive-p: q →¬ p · derive-q: p →¬ q",
            "examples/ein-bugs/wrong-answers/f07-rules-swapped.ein even: derive-p: q →¬ p · derive-q: p →¬ q",
            "examples/features/07_unstratifiable.ein even: derive-p: q →¬ p · derive-q: p →¬ q",
        ]
    );
    assert_eq!(
        found_races,
        [
            "examples/ein-bugs/wrong-answers/f07-fpoff.ein derive-p ← derive-q: q",
            "examples/ein-bugs/wrong-answers/f07-fpoff.ein derive-q ← derive-p: p",
            "examples/ein-bugs/wrong-answers/f07-rules-swapped.ein derive-p ← derive-q: q",
            "examples/ein-bugs/wrong-answers/f07-rules-swapped.ein derive-q ← derive-p: p",
            "examples/features/07_unstratifiable.ein derive-p ← derive-q: q",
            "examples/features/07_unstratifiable.ein derive-q ← derive-p: p",
            "examples/lattice/10_fifo_gap.ein r2 ← r1: q",
        ]
    );
}

/// The cycle through an `absent` over `D`, with its parity, on the three
/// programs that have one; `d3-unsat-core`, a fuzz finding, is a corpus entry.
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
/// (`features/07`, `f07-fpoff`, the written `stratified_stale`), on the
/// FIFO-gap fixture, and on the reader written first, which is one race more.
#[test]
fn the_order_check_finds_every_fifo_race() {
    for rel in [
        "examples/features/07_unstratifiable.ein",
        "examples/ein-bugs/wrong-answers/f07-fpoff.ein",
    ] {
        let (m, _) = file(rel);
        assert_eq!(
            races(&m.detector),
            ["derive-p ← derive-q: q", "derive-q ← derive-p: p"],
            "{rel}"
        );
    }
    let (m, _) = file("examples/lattice/10_fifo_gap.ein");
    assert_eq!(races(&m.detector), ["r2 ← r1: q"]);
    let (m, _) = written(STRATIFIED_STALE);
    assert_eq!(races(&m.detector), ["derive-p ← derive-q: q"]);
    // The reader written first: a race like any other, the written order's.
    let (m, _) = written(FIFO_GAP_WRITTEN);
    assert_eq!(races(&m.detector), ["r2 ← r1: q"]);
    // The static variant: `q` is outside D, so the program is monotone and
    // the gated detector reads nothing (08-monotonicity.md § 3.4).
    let (m, _) = file("examples/lattice/11_fifo_gap_static.ein");
    assert!(m.is_monotone());
    assert_eq!(*m.detector, Static::default());
}

/// What each kind of site concludes and puts at stake, one fixture per cause
/// (`08-monotonicity.md` § 9's c2, c4, c5, c6).
#[test]
fn each_site_names_what_it_concludes_and_puts_at_stake() {
    // What the first fixture's rule concludes is `(false)`, a fact a rule
    // that states `:ein inconsistent` matches; the others, by the word.
    let cases: &[(&str, Option<Concludes>, AtStake)] = &[
        (
            "examples/ein-bugs/naf-upward-closure.ein",
            None,
            AtStake::StateAndDead,
        ),
        (
            "examples/lattice/08_absent_witness_hole.ein",
            Some(Concludes::Positive),
            AtStake::State,
        ),
        (
            "examples/lattice/04_obligation_trigger_hole.ein",
            Some(Concludes::Open),
            AtStake::Owes,
        ),
        (
            "examples/lattice/09_activator_hole.ein",
            Some(Concludes::NoRootPlan),
            AtStake::OwesThroughPlan,
        ),
    ];
    for (rel, concludes, at_stake) in cases {
        let (m, t) = file(rel);
        let sites: Vec<String> = m.sites.iter().map(|s| s.render(&t)).collect();
        assert_eq!(m.detector.stakes.len(), 1, "{rel}: {sites:?}");
        let stake = &m.detector.stakes[0];
        match concludes {
            Some(c) => assert_eq!(stake.concludes, *c, "{rel}"),
            None => {
                assert!(
                    matches!(
                        stake.concludes,
                        Concludes::Refutes(ein_infer::monotone::Refutes::Leads {
                            nullary: true,
                            ..
                        })
                    ),
                    "{rel}: {:?}",
                    stake.concludes
                );
                assert_eq!(stake.concluded, "(false)", "{rel}");
            }
        }
        assert_eq!(m.detector.stakes[0].at_stake, *at_stake, "{rel}");
    }
}

// ── (c): robust, on the corpus ──────────────────────────────────────

/// The bound on `n` for the split sweep: every split `C1 ⊆ C` of every node
/// is `3ⁿ` saturations a program, so `n ≤ 10` is at most 59 049; the test
/// prints how many cubes, nodes and splits it swept. The monotone cubes past
/// it are read by `cargo run --release -p ein-infer --example node_census --
/// --paths --control --max-n 12`, to 12.
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
    // A saved file's nodes are its program's above its node (Q-M1f.402),
    // which a node from scratch, the program and a set written at once, does
    // not start from: its program's sweep reads them.
    if kb.program().commit.is_some() {
        return None;
    }
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

/// **A *Monotone* verdict means one state per node, whatever
/// built it, and the dynamic detector has nothing to find.** On every
/// monotone corpus cube with `n ≤` [`SPLIT_BOUND`], every node built by fork
/// is the node built by every split from a fixpoint that holds root's, fact
/// for fact, and holds no stale fact — so no parent path can differ and the
/// stale check would read nothing, which is why the detector is gated on the
/// verdict (`detector.rs`'s module docs). **From scratch the verdict does not
/// reach**: on `examples/lattice/11_fifo_gap_static.ein` a FIFO race over a
/// static read leaves `{(b one)}` a stale `(p one)` that the fork
/// never holds (`08-monotonicity.md` § 3.4), and it is the one corpus node
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

/// **The same on random programs**: two hundred programs, each written for a
/// random region over at most six candidates, banked as one generated
/// fixture, `golden/random_programs.txt`. Every one the static check calls
/// monotone is swept as the corpus is — by fork, by every split, from
/// scratch, and read for stale facts — and none departs: no random program
/// races two `absent` rules over a static read.
#[test]
fn a_monotone_verdict_is_one_state_per_node_on_random_programs() {
    let fixture = ein_corpus::golden_path("ein-infer", "random_programs.txt");
    let text =
        std::fs::read_to_string(&fixture).unwrap_or_else(|e| panic!("{}: {e}", fixture.display()));
    let mut files: Vec<(String, String)> = Vec::new();
    for line in text.lines() {
        if let Some(name) = line.strip_prefix(";;; program ") {
            files.push((name.to_string(), String::new()));
        } else if let Some((_, body)) = files.last_mut() {
            body.push_str(line);
            body.push('\n');
        }
    }
    assert_eq!(files.len(), 200, "the fixture holds two hundred programs");
    let (mut programs, mut nodes) = (0, 0);
    let mut failures: Vec<String> = Vec::new();
    for (name, program) in &files {
        let path = fixture.with_file_name(format!("{name}.ein"));
        let Some(s) = sweep(program, &path, 6) else {
            continue;
        };
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
