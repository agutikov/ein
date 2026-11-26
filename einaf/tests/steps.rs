//! **The formalization in steps, with no model**: the prompts' examples
//! answer as their texts ask, under their steps' grammars; S2.2.1's steps by
//! hand replay through the steps as one generation a step (the prompts
//! rendered, each part parsed, the program assembled, each step's program
//! judged at root with the writer of H, the contract's check, the
//! categories); a recorded run of the model replays to the run it
//! recorded; each template's digest is the record's; and the check finds
//! what the kernel loads in silence.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

use einaf::catalogue::{self, Catalogue, Size};
use einaf::check::{self, Kind};
use einaf::formalize::{self, Input, Options, Run};
use einaf::grammar::gbnf::Compiled;
use einaf::grammar::steps::{self as gsteps, Params, Step};
use einaf::kernel::{self, Bounds, Outcome, Part, Question, Source};
use einaf::model::replay::Replay;
use einaf::model::{Failure, Generation, Recorded, Request, Runtime};
use einaf::prompts::{self, EXAMPLES};

fn package() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(package().join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

fn bounds() -> Bounds {
    Bounds {
        clock: Some(Duration::from_secs(120)),
        ..Bounds::default()
    }
}

/// The answer query's lines of a program under `--single`, and its outcome.
fn single(program: &str, name: &str) -> (Outcome, Vec<String>) {
    let f = kernel::call(
        Source::Text {
            text: program,
            name,
            base_dir: None,
        },
        &[Question::Single(Part::All)],
        bounds(),
    )
    .pop()
    .expect("one object");
    let outcome = f.outcome.as_ref().expect("an outcome")[0].outcome;
    let lines = f
        .queries
        .iter()
        .flatten()
        .filter(|q| q.goal_text.is_some())
        .flat_map(|q| q.classes.iter().flat_map(|c| c.lines.clone()))
        .collect();
    (outcome, lines)
}

#[test]
fn the_examples_answer_as_their_texts_ask_under_their_steps_grammars() {
    let expected = [
        ("shops", "Ann sells bread. Ben has the red stall."),
        ("buying", "Ann pays (n (n z 1) 2) coins."),
        ("ages", "His sister is (n z 7) years old."),
    ];
    let narrow = Catalogue::new(Size::Whole).grammar();
    for (e, (id, line)) in EXAMPLES.iter().zip(expected) {
        assert_eq!(e.id, id);
        let program = e.program();
        assert_eq!(
            single(&program, e.id),
            (Outcome::One, vec![line.to_string()]),
            "{id}"
        );
        for step in e.steps {
            assert!(
                !step.contains(['`', '<', '>', '|']),
                "{id}: an example holds a character no grammar lets the model write"
            );
        }
        // Each step's output under its grammar, rendered as a run renders it.
        let sentences = einaf::contract::sentences(e.text);
        let program_of = |k: usize| -> String {
            let (statements, _) = formalize::split_step1(e.steps[0]);
            match k {
                1 => e.steps[0].to_string(),
                _ => formalize::assemble_parts(&statements, &e.steps[1..k]),
            }
        };
        for (k, step) in [(1, Step::One), (2, Step::Two), (3, Step::Three)] {
            let mut p = Params {
                sentences: Some(sentences.clone()),
                ..Params::default()
            };
            if k > 1 {
                p.catalogue = narrow.clone();
                p.program = kernel::vocabulary(&program_of(k - 1), id).expect("parses");
            }
            let g = gsteps::render(step, &p);
            let c = Compiled::parse(&g.text).expect("a step's grammar reads");
            let body = e.steps[k - 1].trim_end();
            assert!(
                c.accepts(&g.root, body).unwrap().accepted(),
                "{id}: step {k}'s output is refused by its grammar"
            );
        }
    }
}

/// **A runtime that answers each request with the next of its texts**, in a
/// fenced block, and an envelope from a recorded call.
struct Scripted {
    answers: Vec<String>,
    envelope: Generation,
}

impl Runtime for Scripted {
    fn generate(&mut self, _: &Request) -> Result<Generation, Failure> {
        if self.answers.is_empty() {
            return Err(Failure::NotRecorded {
                digest: "none left".to_string(),
            });
        }
        let mut g = self.envelope.clone();
        g.answer = format!("```ein\n{}\n```", self.answers.remove(0).trim_end());
        Ok(g)
    }
}

fn envelope() -> Generation {
    let line = read("tests/fixtures/model/calls.jsonl")
        .lines()
        .next()
        .expect("a recorded call")
        .to_string();
    let r: Recorded = serde_json::from_str(&line).expect("a record");
    r.generation.expect("a generation")
}

fn hand_run(dir: &str) -> Run {
    let file = |n: &str| read(&format!("contract/{dir}/{n}"));
    let parts = formalize::hand_parts(&file("step1.ein"), &file("step2.ein"), &file("step3.ein"));
    let mut rt = Scripted {
        answers: parts.to_vec(),
        envelope: envelope(),
    };
    let input = Input {
        id: dir.to_string(),
        text: file("text.txt"),
        answer_line: prompts::answer_line("generic").expect("an answer line"),
    };
    let mut calls = Vec::new();
    formalize::run(&mut rt, &input, Options::default(), &mut calls)
}

fn writer(run: &Run, step: &str) -> String {
    let s = run
        .steps
        .iter()
        .find(|s| s.step == step)
        .expect("the step ran");
    s.judged
        .as_ref()
        .and_then(|j| j.root.writer.clone())
        .unwrap_or_default()
}

fn kinds(run: &Run) -> BTreeMap<&'static str, usize> {
    let mut m = BTreeMap::new();
    for f in &run.complete.as_ref().expect("judged").check.findings {
        *m.entry(f.kind.name()).or_insert(0) += 1;
    }
    m
}

#[test]
fn the_steps_by_hand_replay_through_the_steps_as_one_generation_a_step() {
    // The text, each step's writer of H, and the answer: S2.2.1's table.
    let table: [(&str, [&str; 3], &[&str]); 6] = [
        (
            "youngest",
            ["blind", "blind", "no-search"],
            &["Cy is the youngest"],
        ),
        (
            "zebra",
            ["blind", "obligations", "obligations"],
            &["The Norwegian drinks water. The Japanese owns the zebra."],
        ),
        (
            "seating",
            ["blind", "obligations", "obligations"],
            &["Bob sits in the first chair."],
        ),
        (
            "apples",
            ["blind", "blind", "no-search"],
            &["Bob has (n z 3) apples."],
        ),
        ("together", ["blind", "blind", "no-search"], &[]),
        (
            "more",
            ["blind", "blind", "no-search"],
            &["Alice has (n z 7) more apples than Bob."],
        ),
    ];
    for (dir, writers, answer) in table {
        let run = hand_run(dir);
        assert_eq!(run.stopped, None, "{dir}");
        assert_eq!(
            run.steps
                .iter()
                .map(|s| s.step.as_str())
                .collect::<Vec<_>>(),
            ["1", "2", "3"],
            "{dir}"
        );
        for (step, w) in ["1", "2", "3"].iter().zip(writers) {
            assert_eq!(writer(&run, step), w, "{dir}: step {step}'s writer of H");
        }
        // Each step before the last judged at root: not decided, root alive.
        for s in &run.steps[..2] {
            let f = &s.judged.as_ref().expect("judged").feedback[0];
            assert_eq!(
                f.outcome.as_ref().expect("an outcome")[0].outcome,
                Outcome::NotDecided,
                "{dir}: step {} at root",
                s.step
            );
        }
        let program = run.program.as_deref().expect("a program");
        let (outcome, lines) = single(program, dir);
        assert_eq!(outcome, Outcome::One, "{dir}");
        assert_eq!(lines, answer, "{dir}: the answer");
        // The complete program in synthesis order is the hand's step 3, form
        // for form.
        let canon = |t: &str| -> std::collections::BTreeSet<String> {
            kernel::shapes(t, dir)
                .expect("parses")
                .into_iter()
                .map(|f| f.canonical)
                .collect()
        };
        assert_eq!(
            canon(program),
            canon(&read(&format!("contract/{dir}/step3.ein"))),
            "{dir}: the assembled program is the hand's"
        );
        // The categories: every form in one row, each with its step.
        assert_eq!(
            run.categories.len(),
            kernel::shapes(program, dir).unwrap().len(),
            "{dir}"
        );
        let found = kinds(&run);
        println!("{dir}: the check finds {found:?}");
        // The hand's programs: no source that is no sentence, no selection
        // misspelt, no mechanism missing, no relation undeclared, no number
        // as a name, the answer query once, nothing rewritten.
        for k in [
            "source-not-a-sentence",
            "selection-not-listed",
            "selection-not-in-catalogue",
            "mechanism-not-stated",
            "mechanism-not-the-one-stated",
            "undeclared",
            "wrong-count",
            "number-as-name",
            "no-answer-query",
            "several-answer-queries",
            "unbound-reference",
            "answer-in-template",
            "rewritten",
        ] {
            assert!(!found.contains_key(k), "{dir}: {k}");
        }
    }
}

#[test]
fn the_check_finds_what_the_kernel_loads_in_silence() {
    let text = "Ann is older than Bob. Who is the youngest?";
    let program = r#"
(older Ann Bob :source "Ann is older than Bob.")
(older Bob Cy :source "Bob is older than Cy.")
(older Ann 5)
(person Ann)
(query :goal (older ?who Bob) :goal-text "{?who} is older than {?z}")
(query :goal (older ?x Cy) :goal-text "Cy is the youngest")
(relation older Person Person :why "a {?3} is older than a {?4}")
(import std.algebra :symbols (symmetric))
(transitive older)
(symmetric older)
(compose older older older)
"#;
    let catalogue = Catalogue::new(Size::Whole);
    let generated = catalogue::generated();
    let root = kernel::root(
        Source::Text {
            text: program,
            name: "silent",
            base_dir: None,
        },
        bounds(),
    );
    let step1 = check::Step1 {
        statements: vec![(
            "(older Ann Bob :source \"Ann is older than Bob.\")".to_string(),
            Some("Ann is older than Bob.".to_string()),
        )],
        memberships: Default::default(),
    };
    let checked = check::check(&check::Input {
        program,
        name: "silent",
        step: 3,
        text,
        catalogue: &catalogue,
        generated: &generated,
        step1: Some(&step1),
        root: &root,
    });
    let found: Vec<Kind> = checked.findings.iter().map(|f| f.kind).collect();
    for k in [
        Kind::SourceNotASentence,
        Kind::SelectionNotListed,
        Kind::MechanismNotStated,
        Kind::Undeclared,
        Kind::NumberAsName,
        Kind::SeveralAnswerQueries,
        Kind::UnboundReference,
    ] {
        assert!(
            found.contains(&k),
            "{k:?} not found: {:#?}",
            checked.findings
        );
    }
    // `older` at two places, `(older Ann 5)` at two: no count wrong; the
    // undeclared relation is `person`.
    assert!(!found.contains(&Kind::WrongCount));
    assert_eq!(checked.mechanism.writer.as_deref(), Some("blind"));
    // At step 1 only the findings that apply from it.
    let at1 = check::check(&check::Input {
        program,
        name: "silent",
        step: 1,
        text,
        catalogue: &catalogue,
        generated: &generated,
        step1: None,
        root: &root,
    });
    assert!(
        at1.findings.iter().all(|f| f.from_step == 1),
        "{:#?}",
        at1.findings
    );
    assert!(
        !at1.findings
            .iter()
            .any(|f| f.kind == Kind::MechanismNotStated)
    );

    // A wrong count, an inactive selection and a rewritten statement.
    let program = r#"
(older Ann Bob :source "Ann is older than Bob.")
(older Bob Ann :source "Ann is older than Bob.")
(relation older Person Person :why "a {?3} is older than a {?4}")
(relation youngest Person :why "a {?3} is the youngest")
(query :goal (youngest ?w ?z) :goal-text "{?w} is younger than Ann")
(import std.algebra :symbols (compose))
(compose older youngest older)
(solve :no-search)
"#;
    let root = kernel::root(
        Source::Text {
            text: program,
            name: "silent2",
            base_dir: None,
        },
        bounds(),
    );
    let checked = check::check(&check::Input {
        program,
        name: "silent2",
        step: 3,
        text,
        catalogue: &catalogue,
        generated: &generated,
        step1: Some(&step1),
        root: &root,
    });
    let found: Vec<Kind> = checked.findings.iter().map(|f| f.kind).collect();
    for k in [
        Kind::WrongCount,
        Kind::SelectionInactive,
        Kind::Rewritten,
        Kind::AnswerInTemplate,
    ] {
        assert!(
            found.contains(&k),
            "{k:?} not found: {:#?}",
            checked.findings
        );
    }
    assert!(!found.contains(&Kind::MechanismNotStated));

    // `(solve :no-search)` beside a selected obligation: the writer is
    // `no-search`, and the obligations' candidates are gone.
    let entry = catalogue::ENTRIES
        .iter()
        .find(|e| e.id == "slots.partition")
        .expect("the entry");
    let program = format!("{}\n(solve :no-search)\n", entry.program);
    let root = kernel::root(
        Source::Text {
            text: &program,
            name: "silent3",
            base_dir: None,
        },
        bounds(),
    );
    let checked = check::check(&check::Input {
        program: &program,
        name: "silent3",
        step: 3,
        text: "",
        catalogue: &catalogue,
        generated: &generated,
        step1: None,
        root: &root,
    });
    assert_eq!(checked.mechanism.writer.as_deref(), Some("no-search"));
    assert_eq!(checked.mechanism.stated, ["no-search", "obligations"]);
    assert!(
        checked
            .findings
            .iter()
            .any(|f| f.kind == Kind::MechanismNotTheOneStated),
        "{:#?}",
        checked.findings
    );
}

/// **The steps by hand are what a run's grammars take**: each step's part,
/// as one generation, accepted by its step's grammar rendered as a run
/// renders it, on the catalogue's entries.
#[test]
fn each_hand_written_step_is_accepted_by_the_grammar_a_run_renders() {
    let narrow = Catalogue::new(Size::Whole).grammar();
    for dir in ["youngest", "zebra", "seating", "apples", "together", "more"] {
        let file = |n: &str| read(&format!("contract/{dir}/{n}"));
        let parts =
            formalize::hand_parts(&file("step1.ein"), &file("step2.ein"), &file("step3.ein"));
        let sentences = einaf::contract::sentences(&file("text.txt"));
        let (statements, _) = formalize::split_step1(&parts[0]);
        let programs = [
            None,
            Some(parts[0].clone()),
            Some(formalize::assemble_parts(&statements, &[&parts[1]])),
        ];
        for (k, step) in [Step::One, Step::Two, Step::Three].into_iter().enumerate() {
            let mut p = Params {
                sentences: Some(sentences.clone()),
                ..Params::default()
            };
            if let Some(program) = &programs[k] {
                p.catalogue = narrow.clone();
                p.program = kernel::vocabulary(program, dir).expect("parses");
            }
            let g = gsteps::render(step, &p);
            let c = Compiled::parse(&g.text).expect("a step's grammar reads");
            // The hand's comments quote code in backticks, which no grammar
            // a model writes under allows: they are emptied to `;;`; and its
            // runs of blank lines are one, the most a step's layout allows.
            let mut body = parts[k]
                .trim()
                .lines()
                .map(|l| {
                    if l.trim_start().starts_with(';') && !l.contains(gsteps::MARKER) {
                        ";;".to_string()
                    } else {
                        l.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            while body.contains("\n\n\n") {
                body = body.replace("\n\n\n", "\n\n");
            }
            assert!(
                c.accepts(&g.root, &body).unwrap().accepted(),
                "{dir}: step {}'s part is refused by its grammar:\n{body}",
                k + 1
            );
        }
    }
}

/// **The recorded runs of the model replay**: each fixture's calls, replayed,
/// give the run it recorded, its programs and its findings; and its
/// templates' digests are today's.
#[test]
fn a_recorded_run_replays_to_the_run_it_recorded() {
    let dir = package().join("tests/fixtures/steps");
    let mut seen = 0;
    for entry in std::fs::read_dir(&dir).expect("the fixtures") {
        let path = entry.expect("an entry").path();
        if !path.join("run.json").exists() {
            continue;
        }
        let recorded: Run = serde_json::from_str(
            &std::fs::read_to_string(path.join("run.json")).expect("run.json"),
        )
        .expect("a run");
        for s in &recorded.steps {
            let t = prompts::TEMPLATES
                .iter()
                .find(|t| t.file == s.template)
                .expect("a template of the record");
            assert_eq!(
                s.template_sha256,
                t.sha256(),
                "{}: {} changed since the record: a prompt edit is a new experiment, record again",
                path.display(),
                s.template
            );
        }
        let mut replay = Replay::from_file(&path.join("calls.jsonl")).expect("the calls");
        let text = std::fs::read_to_string(path.join("text.txt")).expect("text.txt");
        let input = Input {
            id: recorded.id.clone(),
            text,
            answer_line: prompts::answer_line(
                recorded
                    .answer_line
                    .trim_start_matches("answer-lines/")
                    .split('.')
                    .next()
                    .unwrap_or("generic"),
            )
            .expect("an answer line"),
        };
        let mut calls = Vec::new();
        let again = formalize::run(&mut replay, &input, recorded.options, &mut calls);
        assert_eq!(again.stopped, recorded.stopped);
        assert_eq!(again.program, recorded.program, "{}", path.display());
        assert_eq!(again.steps.len(), recorded.steps.len());
        for (a, b) in again.steps.iter().zip(&recorded.steps) {
            assert_eq!(a.digest, b.digest, "{}: step {}", path.display(), a.step);
            assert_eq!(a.output, b.output);
            assert_eq!(
                a.judged.as_ref().map(|j| &j.check),
                b.judged.as_ref().map(|j| &j.check),
                "{}: step {}",
                path.display(),
                a.step
            );
            assert_eq!(
                a.judged.as_ref().map(|j| &j.root),
                b.judged.as_ref().map(|j| &j.root)
            );
        }
        assert_eq!(again.categories, recorded.categories);
        seen += 1;
    }
    assert!(seen > 0, "no recorded run under {}", dir.display());
}
