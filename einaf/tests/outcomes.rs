//! **Each outcome, read off a program that has it**, through the bounded
//! call. One test an outcome, named in [the outcomes](../../docs/einaf/01-outcomes.md) beside the
//! outcome it holds; the corpus sweep, `tests/corpus.rs`, holds the reading
//! over every program.

use std::path::Path;
use std::time::Duration;

use einaf::kernel::{Bounds, Feedback, Outcome, Part, Question, Source, call};

/// The object of `rel` under `question` and `bounds`.
fn ask(rel: &str, question: Question, bounds: Bounds) -> Feedback {
    let path = ein_corpus::repo_root().join(rel);
    call(Source::File(&path), &[question], bounds)
        .pop()
        .expect("one object a question")
}

/// The outcome of `rel` under `question` and `bounds`, over `part`.
fn read(rel: &str, question: Question, part: Part, bounds: Bounds) -> Outcome {
    let f = ask(rel, question, bounds);
    assert!(f.fault.is_none(), "{rel}: {:?}", f.fault);
    f.outcome_over(part)
        .expect("a reading the question gives")
        .outcome
}

fn single(rel: &str) -> Outcome {
    read(
        rel,
        Question::Single(Part::All),
        Part::All,
        Bounds::default(),
    )
}

/// Every reading of every question, as a list, for a program whose outcome
/// is the same under all of them: one call, the program loaded once.
fn under_every_question(rel: &str, bounds: Bounds) -> Vec<Outcome> {
    let path = ein_corpus::repo_root().join(rel);
    call(Source::File(&path), &Question::ALL, bounds)
        .iter()
        .flat_map(|f| {
            assert!(f.fault.is_none(), "{rel}: {:?}", f.fault);
            f.outcome.clone().expect("an outcome")
        })
        .map(|r| r.outcome)
        .collect()
}

/// **invalid**: the load's refusal, read off its findings, the kernel's
/// diagnostics: at the parse with each finding's place, at the load with the
/// loader's finding and its kind, and the judgement of the hypothesis
/// mechanisms, the step every command of Ein takes after the load, among the
/// load's refusals, its findings of the kind `mechanisms`.
#[test]
fn invalid_is_read_off_the_refusal() {
    let kinds = |f: &Feedback| {
        let mut out: Vec<String> = Vec::new();
        for d in &f.diagnostics {
            if !out.contains(&d.kind) {
                out.push(d.kind.clone());
            }
        }
        out
    };
    let parse = ask(
        "examples/broken/unclosed_paren.ein",
        Question::Describe,
        Bounds::default(),
    );
    assert_eq!(kinds(&parse), ["parse"]);
    let place = parse.diagnostics[0]
        .place
        .as_ref()
        .expect("a parse finding's place");
    assert_eq!((place.line, place.col), (4, 10));
    assert!(
        parse.diagnostics[0]
            .message
            .contains(":4:10: unexpected `a`"),
        "{:?}",
        parse.diagnostics
    );
    assert_eq!(parse.program.of.as_deref(), Some("text"));
    let why = &parse.outcome_over(Part::All).expect("a reading").why;
    assert!(why.starts_with("refused at the parse (parse): "), "{why}");

    let load = ask(
        "examples/broken/load/rule_duplicate_name.ein",
        Question::Describe,
        Bounds::default(),
    );
    assert_eq!(kinds(&load), ["declaration"]);
    assert_eq!(
        load.diagnostics[0].message,
        "duplicate rule name 'r' at None"
    );
    assert!(
        load.diagnostics[0].place.is_none(),
        "{:?}",
        load.diagnostics
    );
    assert_eq!(load.program.of.as_deref(), Some("canonical-dump"));

    let mechanisms = ask(
        "examples/broken/load/no_hypothesis_beside_open.ein",
        Question::Describe,
        Bounds::default(),
    );
    assert_eq!(kinds(&mechanisms), ["mechanisms"]);
    assert!(
        mechanisms.diagnostics[0]
            .message
            .contains("its :no-hypothesis names likes"),
        "{:?}",
        mechanisms.diagnostics
    );

    let missing = ask(
        "examples/does-not-exist.ein",
        Question::Describe,
        Bounds::default(),
    );
    assert_eq!(kinds(&missing), ["unreadable"]);
    assert!(missing.program.digest.is_none());

    for rel in [
        "examples/broken/unclosed_paren.ein",
        "examples/broken/load/rule_duplicate_name.ein",
        "examples/broken/load/no_hypothesis_beside_open.ein",
    ] {
        let all = under_every_question(rel, Bounds::default());
        assert!(all.iter().all(|&o| o == Outcome::Invalid), "{rel}: {all:?}");
    }
}

/// **flagged**: read before any answer. The program answers `Ambiguity`
/// under `--single` and is flagged; past the cube's limit it is flagged and
/// not *not decided*. A question that needs the cube is not asked past the
/// static check: its object holds the sites and the warning, and no answer.
#[test]
fn flagged_is_read_before_any_answer() {
    let rel = "examples/lattice/04_obligation_trigger_hole.ein";
    let all = under_every_question(rel, Bounds::default());
    assert!(all.iter().all(|&o| o == Outcome::Flagged), "{all:?}");
    let past_the_limit = Bounds {
        limit: 0,
        ..Bounds::default()
    };
    let all = under_every_question(rel, past_the_limit);
    assert!(all.iter().all(|&o| o == Outcome::Flagged), "{all:?}");

    let described = ask(rel, Question::Describe, Bounds::default());
    let flagged = described.flagged.as_ref().expect("the sites");
    assert!(!flagged.sites.is_empty());
    assert!(described.states.is_none() && described.readings.is_none());
    assert_eq!(described.diagnostics[0].kind, "not-monotone");
    let single = ask(rel, Question::Single(Part::All), Bounds::default());
    assert_eq!(single.flagged, described.flagged);
    assert!(single.states.is_some(), "--single is asked, and recorded");
}

/// **not decided**: a bound stopped the run, the facts budget, the clock or
/// the cap on one saturation; or `--no-search` on a root that is alive. The
/// same program is *one* with no bound, so not decided is never a guess at
/// the answer.
#[test]
fn not_decided_is_a_run_a_bound_stopped() {
    let rel = "examples/zebra2.ein";
    let budget = Bounds {
        budget: 1,
        ..Bounds::default()
    };
    for part in Part::ALL {
        assert_eq!(
            read(rel, Question::Describe, part, budget),
            Outcome::NotDecided
        );
    }
    let f = ask(rel, Question::Single(Part::All), budget);
    assert_eq!(
        f.outcome_over(Part::All).expect("its reading").outcome,
        Outcome::NotDecided
    );
    assert_eq!(
        f.readings
            .as_ref()
            .and_then(|r| r.stopped.as_ref())
            .map(|s| s.why.as_str()),
        Some("budget")
    );
    let clock = Bounds {
        clock: Some(Duration::ZERO),
        ..Bounds::default()
    };
    let f = ask(rel, Question::Single(Part::All), clock);
    assert_eq!(
        f.outcome_over(Part::All).expect("its reading").outcome,
        Outcome::NotDecided
    );
    assert_eq!(f.diagnostics.len(), 1, "{:?}", f.diagnostics);
    assert_eq!(f.diagnostics[0].kind, "deadline");
    assert_eq!(
        f.diagnostics[0].message,
        "--max-time 0 s reached before the question was decided"
    );
    assert_eq!(f.asked.clock_us, Some(0));
    assert_eq!(
        read(rel, Question::NoSearch, Part::All, Bounds::default()),
        Outcome::NotDecided
    );
    let cap = Bounds {
        max_steps: Some(100),
        ..Bounds::default()
    };
    assert_eq!(
        read(rel, Question::Single(Part::All), Part::All, cap),
        Outcome::NotDecided
    );
    assert_eq!(single(rel), Outcome::One);
}

/// **not decided**, of a program whose saturation has no fixpoint: a rule
/// that nests its conclusion one deeper than its premise. The cap on
/// firings ends root's saturation, under every question, and so does the
/// clock; and where the program declares two hypothesis mechanisms, the
/// load's judgement, which saturates root, is under the same bounds.
#[test]
fn a_saturation_with_no_fixpoint_is_not_decided_at_the_kernels_bound() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cap = Bounds {
        max_steps: Some(10_000),
        ..Bounds::default()
    };
    let clock = Bounds {
        clock: Some(Duration::from_millis(50)),
        max_steps: None,
        ..Bounds::default()
    };
    for fixture in ["tests/fixtures/grow.ein", "tests/fixtures/grow_judged.ein"] {
        let path = here.join(fixture);
        for bounds in [cap, clock] {
            let all: Vec<Feedback> = call(Source::File(&path), &Question::ALL, bounds);
            for f in &all {
                assert!(f.fault.is_none(), "{fixture}: {:?}", f.fault);
                for r in f.outcome.as_ref().expect("an outcome") {
                    assert_eq!(r.outcome, Outcome::NotDecided, "{fixture}: {}", r.why);
                }
            }
            if bounds.clock.is_none() {
                let stop = all[0]
                    .readings
                    .as_ref()
                    .and_then(|r| r.stopped.clone())
                    .expect("what stopped it");
                assert_eq!(
                    (stop.why.as_str(), stop.saturation.as_deref()),
                    ("steps", Some("root"))
                );
                assert_eq!(all[0].diagnostics[0].kind, "partial");
            } else {
                assert_eq!(all[0].diagnostics[0].kind, "deadline");
            }
        }
    }
}

/// **A dead root's core search is under the call's clock, and taken only
/// where the question prints the core**: Ein's `dead_link.ein`, whose root
/// saturates in milliseconds and dies on a link several rules derive, every
/// derivation recorded, so that the search for its core runs far past any
/// clock set here. A description and `--no-search`, whose objects carry
/// root's core, are *not decided* at the clock, with the deadline's
/// diagnostic; every other question prints no core and takes none, and is
/// *inconsistent* in what root's saturation costs, with no deadline. The call
/// returns about when its clock says.
#[test]
fn a_dead_roots_core_search_is_not_decided_at_the_kernels_clock() {
    let path = ein_corpus::repo_root().join("ein.rs/crates/ein-cli/tests/bounded/dead_link.ein");
    let clock = Bounds {
        clock: Some(Duration::from_millis(200)),
        ..Bounds::default()
    };
    let t = std::time::Instant::now();
    let all: Vec<Feedback> = call(Source::File(&path), &Question::ALL, clock);
    assert!(t.elapsed() < Duration::from_secs(30), "{:?}", t.elapsed());
    assert_eq!(all.len(), Question::ALL.len());
    for (q, f) in Question::ALL.iter().zip(&all) {
        assert!(f.fault.is_none(), "{:?}", f.fault);
        let prints_core = matches!(q, Question::Describe | Question::NoSearch);
        let want = if prints_core {
            Outcome::NotDecided
        } else {
            Outcome::Inconsistent
        };
        for r in f.outcome.as_ref().expect("an outcome") {
            assert_eq!(r.outcome, want, "{}: {}", q.flags(), r.why);
        }
        let deadline = f.diagnostics.iter().any(|d| d.kind == "deadline");
        assert_eq!(deadline, prints_core, "{}: {:?}", q.flags(), f.diagnostics);
        assert!(f.core.is_none(), "{}", q.flags());
    }
}

/// **inconsistent**: R is empty and every node is dead, under `--single`, a
/// description and `--no-search` alike; a description carries root's core.
#[test]
fn inconsistent_is_an_empty_r_every_node_dead() {
    let rel = "examples/ein-bugs/zebra2-bad.ein";
    let all = under_every_question(rel, Bounds::default());
    assert!(all.iter().all(|&o| o == Outcome::Inconsistent), "{all:?}");
    let core = ask(rel, Question::Describe, Bounds::default())
        .core
        .expect("a dead root's core under a description");
    assert_eq!(core.facts.len(), core.sources.len());
    assert!(!core.refuted_by.is_empty());
    assert!(
        ask(rel, Question::Single(Part::All), Bounds::default())
            .core
            .is_none(),
        "a question that prints no core takes none"
    );
}

/// **incomplete**: R is empty and every alive node owes. `--no-search` does
/// not decide it: root is alive, and an extension of it might pay. A
/// description says what root owes.
#[test]
fn incomplete_is_an_empty_r_every_alive_node_owing() {
    let rel = "tests/region/04_owing.ein";
    assert_eq!(single(rel), Outcome::Incomplete);
    for part in Part::ALL {
        assert_eq!(
            read(rel, Question::Describe, part, Bounds::default()),
            Outcome::Incomplete
        );
    }
    assert_eq!(
        read(rel, Question::NoSearch, Part::All, Bounds::default()),
        Outcome::NotDecided
    );
    let owed = ask(rel, Question::Describe, Bounds::default())
        .owed
        .expect("what is owed");
    assert!(!owed.root.expect("root's debts").is_empty());
}

/// **several** and **one** are counted over a part of R, and one program is
/// both: two states over all of R, one over the maximal line. The reading is
/// returned with the outcome.
#[test]
fn several_and_one_are_read_over_a_part() {
    let rel = "examples/lattice/03_state_hash_collision.ein";
    assert_eq!(single(rel), Outcome::Several);
    assert_eq!(
        read(
            rel,
            Question::Single(Part::Maximal),
            Part::Maximal,
            Bounds::default()
        ),
        Outcome::One
    );
    assert_eq!(
        read(rel, Question::Describe, Part::All, Bounds::default()),
        Outcome::Several
    );
    assert_eq!(
        read(rel, Question::Describe, Part::Maximal, Bounds::default()),
        Outcome::One
    );
    let f = ask(rel, Question::Single(Part::All), Bounds::default());
    let readings = f.outcome.as_ref().expect("an outcome");
    assert_eq!(readings.len(), 1);
    assert_eq!(readings[0].part, Part::All);
    // `--single` over all of R is not read over the maximal line.
    assert!(f.outcome_over(Part::Maximal).is_none());
}

/// **one**: the part of R counted is one state, under every question that
/// decides it: the Zebra puzzle, whose answer query has one class.
#[test]
fn one_is_one_state() {
    let rel = "examples/zebra2.ein";
    for (q, p) in [
        (Question::Single(Part::All), Part::All),
        (Question::Single(Part::Maximal), Part::Maximal),
        (Question::Describe, Part::All),
        (Question::Describe, Part::Maximal),
        (Question::Line(Part::Maximal), Part::Maximal),
        (Question::Count, Part::All),
    ] {
        assert_eq!(
            read(rel, q, p, Bounds::default()),
            Outcome::One,
            "{}",
            q.flags()
        );
    }
    let f = ask(rel, Question::Single(Part::All), Bounds::default());
    let q = &f.queries.as_ref().expect("the queries")[0];
    assert!(q.agree && q.exact);
    assert_eq!(
        q.classes[0].lines,
        ["The Norwegian drinks water in House-1; the Japanese owns zebra in House-5"]
    );
}

/// A program handed over as text loads as its file does, its local imports
/// found from the directory it is given, and its digest is its file's.
#[test]
fn a_text_loads_as_its_file_does() {
    let root = ein_corpus::repo_root();
    let rel = "examples/zebra2.ein";
    let text = std::fs::read_to_string(root.join(rel)).expect("the file");
    let examples = root.join("examples");
    let from_text = call(
        Source::Text {
            text: &text,
            name: rel,
            base_dir: Some(&examples),
        },
        &[Question::Single(Part::All)],
        Bounds::default(),
    )
    .pop()
    .expect("an object");
    assert_eq!(
        from_text
            .outcome_over(Part::All)
            .expect("its reading")
            .outcome,
        Outcome::One
    );
    let from_file = ask(rel, Question::Single(Part::All), Bounds::default());
    assert_eq!(from_text.program.digest, from_file.program.digest);
    assert_eq!(from_text.states, from_file.states);

    let broken = call(
        Source::Text {
            text: "(relation p :why \"p\"",
            name: "<model>",
            base_dir: None,
        },
        &[Question::Describe],
        Bounds::default(),
    )
    .pop()
    .expect("an object");
    assert_eq!(
        broken.outcome_over(Part::All).expect("its reading").outcome,
        Outcome::Invalid
    );
    assert!(
        broken.diagnostics[0].message.starts_with("<model>:1:"),
        "{:?}",
        broken.diagnostics
    );
}
