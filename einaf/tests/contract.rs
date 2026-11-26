//! **The contract's examples, run**: [the formalization contract](../../docs/einaf/04-contract.md)'s
//! worked example in its forms, and the texts worked through the three steps
//! by hand under `einaf/contract/`, each step's program asked through the
//! bounded call and held to what the page's tables say; and the categories:
//! every top-level form of every reference program in exactly one row, with
//! the step that wrote it.
//!
//! H's writer and its candidates are read off `ein show hyps`, from the
//! binary the gate built (`$EIN_BIN`, or `ein.rs/target/gate/ein`), and
//! `einaf::kernel::root`, which a run of the steps reads the writer off, is
//! held to it on every program here.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use einaf::contract::{self, Category, Judged, Row};
use einaf::kernel::{self, Bounds, Feedback, Forms, Outcome, Part, Question, Shape, Source, call};

/// `einaf/contract/`, where the page's examples are.
fn contract_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("contract")
}

/// A step's program: `einaf/contract/DIR/stepN.ein`.
fn step(dir: &str, n: u8) -> PathBuf {
    contract_dir().join(dir).join(format!("step{n}.ein"))
}

fn ein_bin() -> PathBuf {
    let bin = std::env::var_os("EIN_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| ein_corpus::repo_root().join("ein.rs/target/gate/ein"));
    assert!(
        bin.is_file(),
        "{}: no `ein` binary to read H with; the gate builds it with \
         `cargo test --manifest-path ein.rs/Cargo.toml --profile gate --workspace`, \
         or `cargo build --manifest-path ein.rs/Cargo.toml --profile gate -p ein-cli`, \
         and `$EIN_BIN` names another",
        bin.display()
    );
    bin
}

/// **H, as `ein show hyps` says it**: its writer, and its candidates, one a
/// line.
fn hypotheses(path: &Path) -> (String, Vec<String>) {
    let out = Command::new(ein_bin())
        .args(["show", "hyps"])
        .arg(path)
        .output()
        .expect("ein runs");
    assert!(
        out.status.success(),
        "{}: ein show hyps failed",
        path.display()
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let note = stderr
        .lines()
        .find(|l| l.starts_with("note: "))
        .expect("a note that says who wrote H");
    let writer = note
        .split('`')
        .nth(1)
        .expect("the writer, in backquotes")
        .to_string();
    let candidates = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_string)
        .collect();
    (writer, candidates)
}

/// The text a directory of `einaf/contract/` holds.
fn text(dir: &str) -> String {
    std::fs::read_to_string(contract_dir().join(dir).join("text.txt")).expect("the text")
}

fn read(path: &Path) -> Forms {
    kernel::forms(Source::File(path)).unwrap_or_else(|f| panic!("{}: {f:?}", path.display()))
}

/// The objects of `path` under `questions`, the program loaded once.
fn ask(path: &Path, questions: &[Question]) -> Vec<Feedback> {
    let all = call(Source::File(path), questions, Bounds::default());
    for f in &all {
        assert!(f.fault.is_none(), "{}: {:?}", path.display(), f.fault);
    }
    all
}

fn one(path: &Path, question: Question) -> Feedback {
    ask(path, &[question]).pop().expect("one object a question")
}

/// The outcome over all of R.
fn outcome(f: &Feedback) -> Outcome {
    f.outcome_over(Part::All)
        .expect("a reading over all of R")
        .outcome
}

/// **The answer**: the lines of the answer query, the one query that carries
/// a `:goal-text`, a list a class; `None` where no answer came back.
fn answer(f: &Feedback) -> Option<Vec<Vec<String>>> {
    let queries = f.queries.as_ref()?;
    let q = queries.iter().find(|q| q.goal_text.is_some())?;
    Some(q.classes.iter().map(|c| c.lines.clone()).collect())
}

/// The rows of a directory's three steps: each form of step 3's program, with
/// the step that wrote it.
fn rows_by_steps(dir: &str) -> Vec<Row> {
    let steps: Vec<Forms> = (1..=3).map(|n| read(&step(dir, n))).collect();
    let refs: Vec<&Forms> = steps.iter().collect();
    contract::categories_by_steps(&refs, &text(dir))
}

/// How many rows are in each (step, category), and of each event and note.
type Tally = (
    BTreeMap<(u8, Category), usize>,
    BTreeMap<&'static str, usize>,
);

fn tally(rows: &[Row]) -> Tally {
    let mut counts = BTreeMap::new();
    let mut judged = BTreeMap::new();
    for r in rows {
        *counts.entry((r.step, r.category)).or_default() += 1;
        if let Some(j) = r.judged {
            *judged.entry(j.name()).or_default() += 1;
        }
    }
    (counts, judged)
}

fn events(rows: &[Row]) -> Vec<&str> {
    rows.iter()
        .filter(|r| r.judged.is_some_and(Judged::is_event))
        .map(|r| r.canonical.as_str())
        .collect()
}

const DIRS: [&str; 6] = ["youngest", "zebra", "seating", "apples", "together", "more"];

const ZEBRA_ANSWER: &str = "The Norwegian drinks water. The Japanese owns the zebra.";

/// **The page's worked example, in the three forms its mechanism line takes**,
/// and the five forms of a formalizer with no theory: the answer is right,
/// unclaimed or wrong by the mechanism line alone, and nothing the kernel
/// returns says that a program stated no mechanism.
#[test]
fn the_worked_example_turns_on_its_mechanism_line() {
    let dir = contract_dir().join("youngest");
    let complete = dir.join("step3.ein");
    assert_eq!(hypotheses(&complete), ("no-search".to_string(), vec![]));
    let f = one(&complete, Question::Single(Part::All));
    assert_eq!(outcome(&f), Outcome::One);
    assert!(f.flagged.is_none());
    assert_eq!(
        answer(&f),
        Some(vec![vec!["Cy is the youngest".to_string()]])
    );

    let deleted = dir.join("no-mechanism.ein");
    assert_eq!(hypotheses(&deleted).0, "blind");
    assert_eq!(hypotheses(&deleted).1.len(), 5);
    let f = one(&deleted, Question::Single(Part::All));
    assert_eq!(outcome(&f), Outcome::Flagged);
    let sites = &f.flagged.as_ref().expect("the static check's sites").sites;
    assert_eq!(sites.len(), 1);
    assert_eq!(sites[0].relation, "older");

    let closed = dir.join("closed.ein");
    assert_eq!(
        hypotheses(&closed),
        (
            "blind".to_string(),
            vec!["(youngest Ann)".to_string(), "(youngest Bob)".to_string()]
        )
    );
    let f = one(&closed, Question::Single(Part::All));
    assert_eq!(outcome(&f), Outcome::Several);
    assert!(f.flagged.is_none());
    let lines = answer(&f).expect("the two states' classes");
    assert!(lines.contains(&vec!["Cy is the youngest".to_string()]));
    assert!(lines.contains(&vec![
        "Ann is the youngest".to_string(),
        "Cy is the youngest".to_string()
    ]));
    let f = one(&closed, Question::Describe);
    assert_eq!(f.readings.as_ref().and_then(|r| r.k_s), Some(4));

    let none = dir.join("no-theory.ein");
    assert_eq!(hypotheses(&none).0, "blind");
    assert_eq!(hypotheses(&none).1.len(), 7);
    let all = ask(&none, &[Question::Describe, Question::Single(Part::All)]);
    assert!(all[0].flagged.is_none());
    assert_eq!(all[0].readings.as_ref().and_then(|r| r.k_s), Some(128));
    assert_eq!(outcome(&all[1]), Outcome::Several);
    for f in &all {
        assert!(
            f.diagnostics
                .iter()
                .all(|d| !d.message.contains("mechanism")),
            "the kernel says nothing of a mechanism not stated: {:?}",
            f.diagnostics
        );
    }

    // The page's examples follow the contract: the mechanical judge finds no
    // event in any.
    for file in [
        "step1.ein",
        "step2.ein",
        "step3.ein",
        "no-mechanism.ein",
        "closed.ein",
        "no-theory.ein",
    ] {
        let rows = contract::categories(&read(&dir.join(file)), &text("youngest"));
        assert_eq!(events(&rows), Vec::<&str>::new(), "{file}");
    }
}

/// One step's program, as the page's table of the steps has it: its
/// directory and step, H's writer and size, and under `--single` the outcome
/// and the answer query's lines, a list a class (none for a program whose H
/// is too large to search).
type StepRow = (
    &'static str,
    u8,
    &'static str,
    usize,
    Option<(Outcome, &'static [&'static [&'static str]])>,
);

const NO_ROW: &[&[&str]] = &[&[]];

/// **Each step's program, judged**: at root, as the contract's steps are, and
/// under `--single`, as the page's table says; the Zebra puzzle's step 1,
/// whose H is the blind enumerator's ten thousand candidates, at root alone.
#[test]
fn each_steps_program_is_judged_as_the_page_says() {
    use Outcome::*;
    let table: [StepRow; 18] = [
        ("youngest", 1, "blind", 7, Some((Several, NO_ROW))),
        ("youngest", 2, "blind", 6, Some((Several, NO_ROW))),
        (
            "youngest",
            3,
            "no-search",
            0,
            Some((One, &[&["Cy is the youngest"]])),
        ),
        ("zebra", 1, "blind", 9_896, None),
        ("zebra", 2, "obligations", 112, Some((One, NO_ROW))),
        (
            "zebra",
            3,
            "obligations",
            112,
            Some((One, &[&[ZEBRA_ANSWER]])),
        ),
        ("seating", 1, "blind", 89, Some((Several, NO_ROW))),
        ("seating", 2, "obligations", 18, Some((Several, NO_ROW))),
        (
            "seating",
            3,
            "obligations",
            0,
            Some((One, &[&["Bob sits in the first chair."]])),
        ),
        ("apples", 1, "blind", 0, Some((One, NO_ROW))),
        ("apples", 2, "blind", 0, Some((One, NO_ROW))),
        (
            "apples",
            3,
            "no-search",
            0,
            Some((One, &[&["Bob has (n z 3) apples."]])),
        ),
        ("together", 1, "blind", 0, Some((One, NO_ROW))),
        ("together", 2, "blind", 0, Some((One, NO_ROW))),
        ("together", 3, "no-search", 0, Some((One, NO_ROW))),
        ("more", 1, "blind", 0, Some((One, NO_ROW))),
        ("more", 2, "blind", 0, Some((One, NO_ROW))),
        (
            "more",
            3,
            "no-search",
            0,
            Some((One, &[&["Alice has (n z 7) more apples than Bob."]])),
        ),
    ];
    for (dir, n, h_writer, h_size, single) in table {
        let path = step(dir, n);
        let name = format!("{dir}/step{n}");
        let (writer, candidates) = hypotheses(&path);
        assert_eq!(
            (writer.as_str(), candidates.len()),
            (h_writer, h_size),
            "{name}: H"
        );
        let mut questions = vec![Question::NoSearch];
        if single.is_some() {
            questions.push(Question::Single(Part::All));
        }
        let all = ask(&path, &questions);
        // At root: it loads, it is not flagged, and root is alive; R is not
        // searched.
        assert_eq!(outcome(&all[0]), Outcome::NotDecided, "{name}: at root");
        assert!(all[0].flagged.is_none(), "{name}: flagged");
        if let Some((expected, lines)) = single {
            assert_eq!(outcome(&all[1]), expected, "{name}: --single");
            let lines: Vec<Vec<String>> = lines
                .iter()
                .map(|c| c.iter().map(|l| l.to_string()).collect())
                .collect();
            let got = answer(&all[1]).expect("an answer query");
            if expected == Outcome::One {
                assert_eq!(got, lines, "{name}: the answer");
            } else {
                assert!(got.iter().all(|c| c.is_empty()), "{name}: {got:?}");
            }
        }
    }
}

/// **Each complete program answers its question under every question**: *one*
/// under each reading of each question that searches, *not decided* under
/// `--no-search`, and its answer the same sentence under each.
#[test]
fn each_complete_program_is_one_under_every_question() {
    for dir in DIRS {
        let path = step(dir, 3);
        let all = ask(&path, &Question::ALL);
        let mut answers = Vec::new();
        for f in &all {
            for r in f.outcome.as_ref().expect("an outcome") {
                let expected = if f.asked.question == "--no-search" {
                    Outcome::NotDecided
                } else {
                    Outcome::One
                };
                assert_eq!(
                    r.outcome, expected,
                    "{dir}: {} {:?}",
                    f.asked.question, r.part
                );
            }
            if f.asked.question != "--no-search" && f.asked.question != "--count" {
                answers.push(answer(f));
            }
        }
        answers.dedup();
        assert_eq!(answers.len(), 1, "{dir}: {answers:?}");
    }
}

/// **The Zebra puzzle's steps end at the target shape**: step 3 is
/// `examples/zebra3.1.ein` form for form and in its order, less its
/// `(config …)`, each form's `:source` aside (step 1 quotes, in each
/// membership of a thing a sentence names, that sentence); and the two say
/// the same under every question: the outcomes, the states, H and the
/// answer.
#[test]
fn the_zebra_steps_end_at_the_target_shape() {
    let target = ein_corpus::repo_root().join("examples/zebra3.1.ein");
    let complete = step("zebra", 3);
    let unsourced = |forms: &Forms| -> Vec<String> {
        forms
            .forms
            .iter()
            .filter(|f| f.shape != Shape::Config)
            .map(|f| {
                // The form on one line, its `:source "…"` taken out.
                let folded = f.canonical.split_whitespace().collect::<Vec<_>>().join(" ");
                match folded.find(" :source \"") {
                    Some(i) => {
                        let end = i + " :source \"".len();
                        let close = folded[end..].find('"').expect("the text's end") + end + 1;
                        format!("{}{}", &folded[..i], &folded[close..])
                    }
                    None => folded,
                }
            })
            .collect()
    };
    let (a, b) = (read(&complete), read(&target));
    assert_eq!(unsourced(&a), unsourced(&b));
    let sourced = |forms: &Forms| {
        forms
            .forms
            .iter()
            .filter(|f| {
                matches!(
                    &f.shape,
                    Shape::Fact {
                        source: Some(_),
                        ..
                    }
                )
            })
            .count()
    };
    assert_eq!((sourced(&a), sourced(&b)), (44, 19));
    assert_eq!(hypotheses(&complete), hypotheses(&target));
    let (a, b) = (ask(&complete, &Question::ALL), ask(&target, &Question::ALL));
    for (x, y) in a.iter().zip(&b) {
        assert_eq!(x.outcome, y.outcome, "{}", x.asked.question);
        assert_eq!(x.states, y.states, "{}", x.asked.question);
        assert_eq!(x.queries, y.queries, "{}", x.asked.question);
    }
}

/// **A later step rewrites no statement**: every form step 1 wrote is in step
/// 2's program and step 3's as it was, but the partial ontology, which step 2
/// completes; and **a step writes no form of a later step's part**.
#[test]
fn a_later_step_keeps_the_statements_and_writes_its_own_part() {
    for dir in DIRS {
        let steps: Vec<Forms> = (1..=3).map(|n| read(&step(dir, n))).collect();
        for later in &steps[1..] {
            let held: Vec<&str> = later.forms.iter().map(|f| f.canonical.as_str()).collect();
            for f in &steps[0].forms {
                if matches!(
                    f.shape,
                    Shape::Fact {
                        source: Some(_),
                        ..
                    } | Shape::Query { .. }
                ) {
                    assert!(
                        held.contains(&f.canonical.as_str()),
                        "{dir}: {}",
                        f.canonical
                    );
                }
            }
        }
        let by_part = contract::categories(&steps[2], &text(dir));
        let by_step = rows_by_steps(dir);
        for ((part, actual), form) in by_part.iter().zip(&by_step).zip(&steps[2].forms) {
            assert_eq!(part.canonical, actual.canonical);
            assert!(
                actual.step >= contract::part_step(part.category, &form.shape),
                "{dir}: step {} writes a form of a later step's part: {}",
                actual.step,
                actual.canonical
            );
        }
    }
}

/// The sixteen word problems of `examples/word_problems/`, each with its text
/// as its first comment states it.
const WORD_PROBLEMS: [(&str, &str); 16] = [
    (
        "bob1",
        "Bob has 5 apples. He gave 2 to Alice. How many apples does Bob have?",
    ),
    (
        "gap",
        "Bob has 5 apples. He gave 2 to Alice. How many apples do they have together?",
    ),
    (
        "gap_fact",
        "Bob has 5 apples. Alice has 4 apples. He gave 2 to Alice. How many apples does Bob have? How many does Alice have? How many apples do they have together?",
    ),
    (
        "gap_assume",
        "Bob has 5 apples. He gave 2 to Alice. How many apples do they have together?",
    ),
    (
        "contra",
        "Bob has 5 apples. He gave 7 to Alice. How many apples does Bob have?",
    ),
    (
        "kinds",
        "Bob has 5 apples, Alice has 3 pears. How many apples do they have together?",
    ),
    (
        "kinds_assume",
        "Bob has 5 apples, Alice has 3 pears. How many apples do they have together?",
    ),
    (
        "kinds_listed",
        "Bob has 5 apples, Alice has 3 pears. How many apples do they have together?",
    ),
    (
        "kinds_full",
        "Bob has 5 apples and 2 pears. Alice has 3 apples and 4 pears. How many apples do they have together?",
    ),
    (
        "more",
        "Bob has 5 apples. Alice has 12 apples. Who has more apples, and by how many?",
    ),
    (
        "pencils",
        "A school has 12 boxes of pencils, 144 pencils in each. 250 pencils go to the office. The rest is shared equally among 31 classes. How many pencils does each class get, and how many are left over?",
    ),
    (
        "boxes12",
        "Bob has 5 apples, Alice has 7. A box holds 3. How many boxes do they need?",
    ),
    (
        "boxes13",
        "Bob has 5 apples, Alice has 8. A box holds 3. How many boxes do they need?",
    ),
    (
        "unused",
        "Bob has 5 apples. Alice has 7 apples. Carol has 9 apples and a box that holds 3. Bob gave 2 apples to Alice. How many apples does Bob have?",
    ),
    (
        "bystander",
        "Bob has 5 apples. Carol has 9 apples. Bob gave 2 apples to Alice. How many apples does Carol have?",
    ),
    (
        "before",
        "Bob gave Alice 2 apples. Now Bob has 5. How many apples did Bob have before?",
    ),
];

/// **The categories partition the forms**: every top-level form of every
/// reference program is in exactly one row of the contract's table, with the
/// step that wrote it; the counts of the page's table are these, and the
/// events. `--nocapture` prints each program's counts.
#[test]
fn the_categories_partition_every_form_of_the_reference_programs() {
    use Category::*;
    let mut programs: Vec<(String, Vec<Row>, usize)> = Vec::new();
    let root = ein_corpus::repo_root();
    let zebra = read(&root.join("examples/zebra3.1.ein"));
    programs.push((
        "examples/zebra3.1.ein".into(),
        contract::categories(&zebra, &text("zebra")),
        zebra.forms.len(),
    ));
    for (file, problem) in WORD_PROBLEMS {
        let forms = read(&root.join(format!("examples/word_problems/{file}.ein")));
        programs.push((
            format!("examples/word_problems/{file}.ein"),
            contract::categories(&forms, problem),
            forms.forms.len(),
        ));
    }
    for file in ["no-mechanism", "closed", "no-theory"] {
        let forms = read(&contract_dir().join("youngest").join(format!("{file}.ein")));
        programs.push((
            format!("youngest/{file}.ein"),
            contract::categories(&forms, &text("youngest")),
            forms.forms.len(),
        ));
    }
    for dir in DIRS {
        let steps: Vec<Forms> = (1..=3).map(|n| read(&step(dir, n))).collect();
        for n in 1..=3 {
            let refs: Vec<&Forms> = steps[..n].iter().collect();
            programs.push((
                format!("{dir}/step{n}.ein"),
                contract::categories_by_steps(&refs, &text(dir)),
                steps[n - 1].forms.len(),
            ));
        }
    }
    for (name, rows, forms) in &programs {
        assert_eq!(rows.len(), *forms, "{name}: a form with no row");
        assert!(
            rows.iter()
                .all(|r| Category::ALL.contains(&r.category) && (1..=3).contains(&r.step)),
            "{name}"
        );
        let (counts, judged) = tally(rows);
        println!("{name}: {forms} forms");
        for ((step, c), n) in &counts {
            println!("  step {step}  {:<30} {n}", c.words());
        }
        for (j, n) in &judged {
            println!("  {j}: {n}");
        }
    }
    let of = |name: &str| -> Tally {
        let (_, rows, _) = programs
            .iter()
            .find(|(n, _, _)| n == name)
            .expect("a program");
        tally(rows)
    };
    let counts =
        |pairs: &[((u8, Category), usize)]| pairs.iter().cloned().collect::<BTreeMap<_, _>>();
    let judged =
        |pairs: &[(&'static str, usize)]| pairs.iter().cloned().collect::<BTreeMap<_, _>>();

    // The page's table of the categories of the steps' complete programs.
    assert_eq!(
        of("youngest/step3.ein"),
        (
            counts(&[
                ((1, SourceDerived), 5),
                ((1, Frame), 4),
                ((2, GenericTheory), 2),
                ((3, GeneratedRule), 1),
                ((3, Frame), 1)
            ]),
            judged(&[("generated_rule", 1)])
        )
    );
    assert_eq!(
        of("zebra/step3.ein"),
        (
            counts(&[
                ((1, SourceDerived), 44),
                ((1, Frame), 11),
                ((2, GenericTheory), 16),
                ((2, GeneratedRelation), 12),
                ((2, GeneratedAssumption), 11),
                ((3, GenericTheory), 18),
            ]),
            judged(&[("generated_assumption", 11), ("generated_relation", 12)])
        )
    );
    assert_eq!(
        of("seating/step3.ein"),
        (
            counts(&[
                ((1, SourceDerived), 9),
                ((1, Frame), 4),
                ((2, GenericTheory), 8),
                ((2, GeneratedRelation), 4),
                ((2, GeneratedAssumption), 5),
                ((3, GenericTheory), 2),
                ((3, GeneratedRule), 1),
            ]),
            judged(&[
                ("generated_assumption", 5),
                ("generated_relation", 4),
                ("generated_rule", 1)
            ])
        )
    );
    for (dir, rules, frame) in [("apples", 2, 3), ("together", 6, 4), ("more", 2, 3)] {
        assert_eq!(
            of(&format!("{dir}/step3.ein")),
            (
                counts(&[
                    ((1, SourceDerived), 2),
                    ((1, Frame), frame),
                    ((2, GenericTheory), 3),
                    ((3, GeneratedRule), rules),
                    ((3, Frame), 1)
                ]),
                judged(&[("generated_rule", rules)])
            ),
            "{dir}"
        );
    }
    // The target shape as written: its twenty-five memberships of a named
    // thing quote no sentence, and are events beside the kinds and the row.
    assert_eq!(
        of("examples/zebra3.1.ein"),
        (
            counts(&[
                ((1, SourceDerived), 19),
                ((1, Frame), 11),
                ((2, GenericTheory), 34),
                ((2, GeneratedRelation), 12),
                ((2, GeneratedAssumption), 36),
                ((3, Frame), 1),
            ]),
            judged(&[("generated_assumption", 36), ("generated_relation", 12)])
        )
    );
    // The word problems quote a clause where a sentence states two facts,
    // and an assumption by a text no sentence has: each an `unfound_source`.
    let unfound: Vec<(&str, usize)> = programs
        .iter()
        .filter(|(n, _, _)| n.starts_with("examples/word_problems/"))
        .map(|(n, rows, _)| {
            (
                n.trim_start_matches("examples/word_problems/")
                    .trim_end_matches(".ein"),
                rows.iter()
                    .filter(|r| r.judged == Some(Judged::UnfoundSource))
                    .count(),
            )
        })
        .filter(|(_, k)| *k > 0)
        .collect();
    println!("unfound_source: {unfound:?}");
    assert_eq!(
        unfound,
        [
            ("gap_assume", 1),
            ("kinds", 2),
            ("kinds_assume", 3),
            ("kinds_listed", 2),
            ("kinds_full", 4),
            ("pencils", 4),
            ("boxes12", 2),
            ("boxes13", 2),
            ("unused", 2),
        ]
    );
    let other_events: usize = programs
        .iter()
        .filter(|(n, _, _)| n.starts_with("examples/word_problems/"))
        .flat_map(|(_, rows, _)| rows)
        .filter(|r| {
            r.judged
                .is_some_and(|j| j.is_event() && j != Judged::UnfoundSource)
        })
        .count();
    assert_eq!(other_events, 0);
}

/// **The order of the forms changes no answer**: each complete program,
/// written in synthesis order, answers as its forms written the other way
/// round do, the theory first and the statements last: the outcome, the
/// states and the answer query's classes, under a description and under
/// `--single`.
#[test]
fn each_complete_program_answers_alike_with_its_forms_reversed() {
    let questions = [Question::Describe, Question::Single(Part::All)];
    for dir in DIRS {
        let path = step(dir, 3);
        let reversed: String = read(&path)
            .forms
            .iter()
            .rev()
            .map(|f| format!("{}\n", f.canonical))
            .collect();
        let a = ask(&path, &questions);
        let b = call(
            Source::Text {
                text: &reversed,
                name: "reversed",
                base_dir: path.parent(),
            },
            &questions,
            Bounds::default(),
        );
        for (x, y) in a.iter().zip(&b) {
            assert!(y.fault.is_none(), "{dir}: {:?}", y.fault);
            assert_eq!(x.outcome, y.outcome, "{dir}: {}", x.asked.question);
            assert_eq!(x.states, y.states, "{dir}: {}", x.asked.question);
            assert_eq!(x.queries, y.queries, "{dir}: {}", x.asked.question);
            assert_ne!(x.program.digest, y.program.digest, "{dir}: the same text");
        }
    }
}

/// **The writer of H `einaf::kernel::root` reads is `ein show hyps`'**, and
/// so is H's size, on every program of `einaf/contract/`.
#[test]
fn the_writer_of_h_the_kernel_module_reads_is_ein_show_hyps() {
    let mut seen = 0;
    for entry in std::fs::read_dir(contract_dir()).expect("einaf/contract/") {
        let dir = entry.expect("an entry").path();
        for file in std::fs::read_dir(&dir).expect("a text's directory") {
            let path = file.expect("a file").path();
            if path.extension().is_none_or(|e| e != "ein") {
                continue;
            }
            let (writer, candidates) = hypotheses(&path);
            let root = kernel::root(Source::File(&path), Bounds::default());
            assert_eq!(
                (root.writer.as_deref(), root.candidates),
                (Some(writer.as_str()), candidates.len()),
                "{}",
                path.display()
            );
            seen += 1;
        }
    }
    assert!(seen >= 21, "the contract's programs: {seen}");
}
