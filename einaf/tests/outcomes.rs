//! **Each outcome, read off a program that has it.** One test an outcome,
//! named in [the outcomes](../../docs/einaf/01-outcomes.md) beside the outcome it holds; the
//! corpus sweep, `tests/corpus.rs`, holds the reading over every program.

use std::path::Path;
use std::time::Duration;

use einaf::kernel::{Bounds, Outcome, Part, Program, Question, Run, Stage, outcome};

fn load(rel: &str) -> Program {
    let path = ein_corpus::repo_root().join(rel);
    Program::load_file(&path).unwrap_or_else(|r| panic!("{rel}: refused: {}", r.message))
}

/// The outcome of `rel` under `question` and `bounds`, over `part`.
fn read(rel: &str, question: Question, part: Part, bounds: Bounds) -> Outcome {
    let path = ein_corpus::repo_root().join(rel);
    let run = match Program::load_file(&path) {
        Ok(p) => p.ask(question, bounds).expect("no engine error"),
        Err(r) => Run::refused(question, r),
    };
    outcome(&run, part)
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
/// is the same under all of them.
fn under_every_question(rel: &str, bounds: Bounds) -> Vec<Outcome> {
    let questions = [
        Question::Describe,
        Question::Single(Part::All),
        Question::Single(Part::Minimal),
        Question::Single(Part::Maximal),
        Question::Line(Part::Minimal),
        Question::Line(Part::Maximal),
        Question::Count,
        Question::NoSearch,
    ];
    questions
        .iter()
        .flat_map(|&q| q.readings().into_iter().map(move |p| (q, p)))
        .map(|(q, p)| read(rel, q, p, bounds))
        .collect()
}

/// **invalid**: the load's refusal, read off its findings, the kernel's
/// diagnostics: at the parse with the parser's value and each finding's
/// place, at the load with the loader's finding and its kind, and the
/// judgement of the hypothesis mechanisms, the step every command of Ein
/// takes after the load, among the load's refusals, its findings of the kind
/// `mechanisms`.
#[test]
fn invalid_is_read_off_the_refusal() {
    let root = ein_corpus::repo_root();
    let parse = Program::load_file(&root.join("examples/broken/unclosed_paren.ein"))
        .err()
        .expect("refused");
    assert_eq!(parse.stage, Stage::Parse);
    let value = parse.parse.as_ref().expect("the parse error as a value");
    assert_eq!(value.position(), (4, 10));
    assert!(
        parse.message.contains(":4:10: unexpected `a`"),
        "{}",
        parse.message
    );
    assert_eq!(parse.kinds(), [ein_core::Kind::Parse]);
    let place = parse.findings[0]
        .place
        .as_ref()
        .expect("a parse finding's place");
    assert_eq!((place.line, place.col), (4, 10));

    let load = Program::load_file(&root.join("examples/broken/load/rule_duplicate_name.ein"))
        .err()
        .expect("refused");
    assert_eq!(load.stage, Stage::Load);
    assert_eq!(
        load.message,
        "kb load error: duplicate rule name 'r' at None"
    );
    assert_eq!(load.kinds(), [ein_core::Kind::Declaration]);
    assert!(load.findings[0].place.is_none(), "{:?}", load.findings);

    let mechanisms =
        Program::load_file(&root.join("examples/broken/load/no_hypothesis_beside_open.ein"))
            .err()
            .expect("refused");
    assert_eq!(mechanisms.stage, Stage::Load);
    assert!(
        mechanisms
            .message
            .contains("its :no-hypothesis names likes"),
        "{}",
        mechanisms.message
    );
    assert_eq!(mechanisms.kinds(), [ein_core::Kind::Mechanisms]);

    let missing = Program::load_file(&root.join("examples/does-not-exist.ein"))
        .err()
        .expect("refused");
    assert_eq!(missing.stage, Stage::Read);
    assert_eq!(missing.kinds(), [ein_core::Kind::Unreadable]);

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
/// not *not decided*.
#[test]
fn flagged_is_read_before_any_answer() {
    let rel = "examples/lattice/04_obligation_trigger_hole.ein";
    let p = load(rel);
    assert!(p.statically_flagged().expect("the static check"));
    let all = under_every_question(rel, Bounds::default());
    assert!(all.iter().all(|&o| o == Outcome::Flagged), "{all:?}");
    let past_the_limit = Bounds {
        limit: 0,
        ..Bounds::default()
    };
    let all = under_every_question(rel, past_the_limit);
    assert!(all.iter().all(|&o| o == Outcome::Flagged), "{all:?}");
}

/// **not decided**: a bound stopped the run, the facts budget or the clock;
/// or `--no-search` on a root that is alive. The same program is *one* with
/// no bound, so not decided is never a guess at the answer.
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
    assert_eq!(
        read(rel, Question::Single(Part::All), Part::All, budget),
        Outcome::NotDecided
    );
    let clock = Bounds {
        clock: Some(Duration::ZERO),
        ..Bounds::default()
    };
    assert_eq!(
        read(rel, Question::Single(Part::All), Part::All, clock),
        Outcome::NotDecided
    );
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
/// firings ends root's saturation, under every question.
#[test]
fn a_saturation_with_no_fixpoint_is_not_decided_at_the_cap() {
    let rel = "ein.rs/crates/ein-cli/tests/bounded/grow.ein";
    let cap = Bounds {
        max_steps: Some(10_000),
        ..Bounds::default()
    };
    let all = under_every_question(rel, cap);
    assert!(all.iter().all(|&o| o == Outcome::NotDecided), "{all:?}");
}

/// **inconsistent**: R is empty and every node is dead, under `--single`, a
/// description and `--no-search` alike.
#[test]
fn inconsistent_is_an_empty_r_every_node_dead() {
    let rel = "examples/ein-bugs/zebra2-bad.ein";
    let all = under_every_question(rel, Bounds::default());
    assert!(all.iter().all(|&o| o == Outcome::Inconsistent), "{all:?}");
}

/// **incomplete**: R is empty and every alive node owes. `--no-search` does
/// not decide it: root is alive, and an extension of it might pay.
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
    let run = load(rel)
        .ask(Question::Single(Part::All), Bounds::default())
        .expect("no engine error");
    let r = outcome(&run, Part::All).expect("its reading");
    assert_eq!(
        (r.question, r.part),
        (Question::Single(Part::All), Part::All)
    );
    // `--single` over all of R is not read over the maximal line.
    assert!(outcome(&run, Part::Maximal).is_none());
}

/// **one**: the part of R counted is one state, under every question that
/// decides it: the Zebra puzzle.
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
}

/// A program handed over as text loads as its file does, its local imports
/// found from the directory it is given.
#[test]
fn a_text_loads_as_its_file_does() {
    let root = ein_corpus::repo_root();
    let rel = "examples/zebra2.ein";
    let text = std::fs::read_to_string(root.join(rel)).expect("the file");
    let p = Program::load_text(&text, rel, Some(Path::new(&root.join("examples"))))
        .unwrap_or_else(|r| panic!("refused: {}", r.message));
    let run = p
        .ask(Question::Single(Part::All), Bounds::default())
        .expect("no engine error");
    assert_eq!(
        outcome(&run, Part::All).expect("its reading").outcome,
        Outcome::One
    );

    let broken = Program::load_text("(relation p :why \"p\"", "<model>", None)
        .err()
        .expect("refused");
    assert_eq!(broken.stage, Stage::Parse);
    assert!(
        broken.message.starts_with("<model>:1:"),
        "{}",
        broken.message
    );
}
