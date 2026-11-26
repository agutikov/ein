//! **From a text to a short answer, with no model**: the formatters' module
//! built and tested alone; every outcome, and an answer query with no row,
//! printed by `einaf from-text --replay` as the answer's table says, from a
//! record of one call written slim; the word problems' own programs, under
//! each of the three forms of an import, with no numeral left a term; the
//! reference word problem over the calculator scored right; the Zebra
//! puzzle's steps by hand through the one shot to its answer; and a recorded
//! run of the model, slim, replayed step by step to its program and answer.

use std::path::{Path, PathBuf};
use std::process::Command;

use einaf::answer::{self, Case};
use einaf::formalize::{self, Input, Mode, Options, Run};
use einaf::format::{self, Formatters};
use einaf::from_text::{self, Config, FromText};
use einaf::kernel::{self, Bounds, Part, Question, Source};

use einaf::model::{Failure, Generation, Recorded, Request, Runtime};
use einaf::prompts;

fn package() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(package().join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

/// A directory of the test's own under the temporary directory, removed
/// when dropped.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let d = std::env::temp_dir().join(format!("einaf-from-text-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("a scratch directory");
        Scratch(d)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// **A runtime that answers each request with the next of its texts**, in a
/// fenced block, with the envelope of a recorded call.
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

/// The one shot in one call, each kernel call under a clock of 5 s: the
/// fixture with no fixpoint costs the check's narration of root its clock.
fn whole() -> Config {
    let mut c = Config::default();
    c.options.mode = Mode::Whole;
    c.options.clock_s = 5;
    c
}

/// **The formatters' module is built and tested alone**: `rustc` on its
/// directory, with no crate beside the standard library, and its own
/// tests run; and no file of it names EinAf's other modules or Ein's
/// crates.
#[test]
fn the_formatters_module_builds_and_is_tested_alone() {
    let dir = package().join("src/format");
    for entry in walk(&dir) {
        let text = std::fs::read_to_string(&entry).expect("a source");
        for word in ["crate::", "ein_", "einaf::", "serde"] {
            assert!(
                !text.contains(word),
                "{}: names {word}: the formatters know nothing of EinAf or Ein",
                entry.display()
            );
        }
    }
    let scratch = Scratch::new("format");
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let bin = scratch.0.join("format-tests");
    let built = Command::new(&rustc)
        .args([
            "--edition",
            "2024",
            "--test",
            "--crate-name",
            "einaf_format",
        ])
        .arg(dir.join("mod.rs"))
        .arg("-o")
        .arg(&bin)
        .output()
        .expect("rustc runs");
    assert!(
        built.status.success(),
        "the formatters' module does not build alone:\n{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let ran = Command::new(&bin).output().expect("its tests run");
    let out = String::from_utf8_lossy(&ran.stdout);
    assert!(ran.status.success(), "{out}");
    assert!(out.contains("test result: ok"), "{out}");
}

fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(dir).expect("a directory") {
        let p = e.expect("an entry").path();
        if p.is_dir() {
            out.extend(walk(&p));
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
    out
}

/// One shot in one call, the program its answer, and the record written.
fn one_shot(program: &str, text: &str, config: &Config, dir: Option<&Path>) -> FromText {
    let mut rt = Scripted {
        answers: vec![program.to_string()],
        envelope: envelope(),
    };
    let input = Input {
        id: "item".to_string(),
        text: text.to_string(),
        answer_line: prompts::answer_line("generic").expect("an answer line"),
    };
    let mut calls = Vec::new();
    let ft = from_text::run(&mut rt, &input, config, &mut calls).expect("runs");
    if let Some(d) = dir {
        from_text::write_record(d, &ft, config, &calls, None).expect("the record is written");
    }
    ft
}

/// **Every outcome has its line**, and an answer query with no row: each
/// fixture the one shot's program, its record written slim, and
/// `einaf from-text --replay` on the record prints the answer's table's line.
#[test]
fn every_outcome_has_its_line_and_einaf_from_text_replay_prints_it() {
    let text = "The hall is lit.\n";
    // The fixture, its case, and the first line printed.
    let table: [(&str, Case, &str); 10] = [
        ("answer/one.ein", Case::Lines, "Bob has 3 apples"),
        ("answer/described.ein", Case::Lines, "The Hall is lit."),
        ("answer/several.ein", Case::Several, answer::UNDETERMINED),
        ("answer/norow.ein", Case::NoRow, answer::UNDETERMINED),
        (
            "answer/inconsistent.ein",
            Case::Inconsistent,
            answer::CONTRADICT,
        ),
        (
            "answer/incomplete.ein",
            Case::Incomplete,
            answer::INCOMPLETE,
        ),
        ("grow.ein", Case::NotDecided, answer::NOT_DECIDED),
        ("answer/flagged.ein", Case::Flagged, answer::FLAGGED),
        ("answer/invalid.ein", Case::Invalid, answer::REFUSED),
        ("answer/noquery.ein", Case::NoQuery, answer::NO_QUERY),
    ];
    let scratch = Scratch::new("outcomes");
    let config = whole();
    for (fixture, case, first) in table {
        let t = std::time::Instant::now();
        let program = read(&format!("tests/fixtures/{fixture}"));
        let name = fixture.replace(['/', '.'], "-");
        let dir = scratch.0.join(&name);
        let ft = one_shot(&program, text, &config, Some(&dir));
        assert_eq!(ft.answer.case, case, "{fixture}: {:#?}", ft.answer);
        assert_eq!(ft.answer.case.fixed().is_none(), case == Case::Lines);
        let text_file = dir.join("text.txt");
        std::fs::write(&text_file, text).expect("the text");
        let out = Command::new(env!("CARGO_BIN_EXE_einaf"))
            .arg("from-text")
            .arg(&text_file)
            .args(["--id", "item", "--mode", "whole", "-T", "5", "--replay"])
            .arg(dir.join("generations.jsonl"))
            .output()
            .expect("einaf runs");
        let stdout = String::from_utf8(out.stdout).expect("UTF-8");
        assert_eq!(out.status.code(), Some(0), "{fixture}: {stdout}");
        assert_eq!(
            stdout.lines().next(),
            Some(first),
            "{fixture}: einaf from-text printed\n{stdout}"
        );
        assert_eq!(
            stdout,
            ft.answer.print(),
            "{fixture}: the replay is the run"
        );
        println!(
            "{fixture}: {:.1} s, the run and its replay",
            t.elapsed().as_secs_f64()
        );
        match case {
            // `--single`'s two states, two classes: the text is decided, and
            // no description is asked for the alternatives.
            Case::Several => {
                assert_eq!(ft.answer.alternatives.len(), 2, "{fixture}");
                assert!(!ft.answer.alternatives_complete, "{fixture}");
            }
            Case::Incomplete => assert_eq!(
                ft.answer.beneath,
                ["likes owes Ann a Food."],
                "{fixture}: what is owed, in the kernel's sentence"
            ),
            Case::NotDecided => assert!(
                ft.answer.beneath.iter().any(|b| b.contains("steps")),
                "{fixture}: what stopped it: {:?}",
                ft.answer.beneath
            ),
            Case::Flagged => assert!(
                ft.answer
                    .beneath
                    .iter()
                    .any(|b| b.contains("NOT MONOTONE") || b.contains("not monotone")),
                "{fixture}: the kernel's warning: {:?}",
                ft.answer.beneath
            ),
            Case::Invalid => assert!(
                ft.answer
                    .beneath
                    .iter()
                    .any(|b| b.starts_with("missing-why")),
                "{fixture}: the diagnostic with its kind: {:?}",
                ft.answer.beneath
            ),
            _ => {}
        }
        // The questions asked: --no-search and --single, and a description
        // only where the answer query's one class was a lower bound.
        let asked: Vec<&str> = ft.asked.iter().map(|f| f.asked.question.as_str()).collect();
        let expected: &[&str] = if fixture == "answer/described.ein" {
            &["--no-search", "--single", "describe"]
        } else {
            &["--no-search", "--single"]
        };
        assert_eq!(asked, expected, "{fixture}");
    }
}

/// **The reference word problem over the calculator** prints a sentence
/// whose number reads 3, which the stand-in reader scores right; with no
/// formatter, the kernel's line.
#[test]
fn the_reference_word_problem_prints_a_sentence_whose_number_reads_3() {
    let program = read("tests/fixtures/answer/one.ein");
    let text = "Bob has 5 apples. He gave 2 to Alice. How many apples does Bob have?\n";
    let ft = one_shot(&program, text, &whole(), None);
    assert_eq!(ft.answer.text(), "Bob has 3 apples");
    assert_eq!(ft.answer.kernel_text(), "Bob has (n z 3) apples");
    assert_eq!(answer::score(&ft.answer, "3").verdict, "right");
    let mut plain = whole();
    plain.format = false;
    let ft = one_shot(&program, text, &plain, None);
    assert_eq!(ft.answer.text(), "Bob has (n z 3) apples");
}

// ── The word problems under the three forms of an import ────────────

/// The sixteen word problems of `examples/word_problems/`.
const WORD_PROBLEMS: [&str; 16] = [
    "bob1",
    "gap",
    "gap_fact",
    "gap_assume",
    "contra",
    "kinds",
    "kinds_assume",
    "kinds_listed",
    "kinds_full",
    "more",
    "pencils",
    "boxes12",
    "boxes13",
    "unused",
    "bystander",
    "before",
];

/// **A program with its selective imports in another form**: `whole`, every
/// such import `(import M)` and each name it listed written in full; `alias`,
/// `(import M :as impK)` and each name `impK.name`. The names are rewritten
/// where the program writes them as names, outside strings and comments and
/// never after `?` or `:`.
fn import_form(text: &str, form: &str) -> String {
    let forms = kernel::shapes(text, "program").expect("parses");
    let vocab = kernel::vocabulary(text, "program").expect("parses");
    let mut renames: Vec<(String, String)> = Vec::new();
    let mut imports: Vec<String> = Vec::new();
    for (k, imp) in vocab.imports.iter().enumerate() {
        let prefix = match form {
            "whole" => imp.module.clone(),
            _ => format!("imp{}", k + 1),
        };
        for s in imp.symbols.iter().flatten() {
            renames.push((s.clone(), format!("{prefix}.{s}")));
        }
        imports.push(match form {
            "whole" => format!("(import {})", imp.module),
            _ => format!("(import {} :as {prefix})", imp.module),
        });
    }
    // The program's forms but its imports, each rewritten, after the new
    // imports.
    let spans = formalize::top_level_spans(text);
    let mut out = imports.join("\n") + "\n";
    for ((s, e), f) in spans.iter().zip(&forms) {
        if matches!(f.shape, kernel::Shape::Import { .. }) {
            continue;
        }
        out.push_str(&rename(&text[*s..*e], &renames));
        out.push('\n');
    }
    out
}

fn rename(form: &str, renames: &[(String, String)]) -> String {
    let c: Vec<char> = form.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < c.len() {
        match c[i] {
            '"' => {
                out.push('"');
                i += 1;
                while i < c.len() && c[i] != '"' {
                    if c[i] == '\\' {
                        out.push(c[i]);
                        i += 1;
                    }
                    out.push(c[i]);
                    i += 1;
                }
                if i < c.len() {
                    out.push('"');
                    i += 1;
                }
            }
            ';' => {
                while i < c.len() && c[i] != '\n' {
                    out.push(c[i]);
                    i += 1;
                }
            }
            ch if format::is_name_char(ch) => {
                let start = i;
                while i < c.len() && format::is_name_char(c[i]) {
                    i += 1;
                }
                let name: String = c[start..i].iter().collect();
                let after_marker = start > 0 && matches!(c[start - 1], '?' | ':');
                match renames.iter().find(|(s, _)| *s == name) {
                    Some((_, full)) if !after_marker => out.push_str(full),
                    _ => out.push_str(&name),
                }
            }
            ch => {
                out.push(ch);
                i += 1;
            }
        }
    }
    out
}

/// **No answer line shows a decimal numeral as a term**, under each of the
/// three forms of an import: on the sixteen word problems' own programs,
/// every value of every query with a `:goal-text` that is a numeral prints
/// as its digits, every quantity in its words, and the line the kernel
/// filled is the one the formatters fill with none.
#[test]
fn the_word_problems_print_no_numeral_as_a_term_under_every_import_form() {
    let dir = ein_corpus::repo_root().join("examples/word_problems");
    let bounds = Bounds {
        clock: Some(std::time::Duration::from_secs(60)),
        ..Bounds::default()
    };
    let numeral = |s: &str| {
        s.contains("(n ")
            || s.contains(".n ")
            || s.contains("std.math.decimal.n")
            || s.split(|c: char| !format::is_name_char(c))
                .any(|w| w == "z" || w.ends_with(".z"))
    };
    let mut lines = 0;
    for p in WORD_PROBLEMS {
        let written = std::fs::read_to_string(dir.join(format!("{p}.ein"))).expect("the problem");
        for form in ["selective", "whole", "alias"] {
            let text = match form {
                "selective" => written.clone(),
                f => import_form(&written, f),
            };
            let name = format!("{p}.{form}.ein");
            let f = kernel::call(
                Source::Text {
                    text: &text,
                    name: &name,
                    base_dir: Some(&dir),
                },
                &[Question::Describe],
                bounds,
            )
            .pop()
            .expect("one object");
            let outcome = f.outcome_over(Part::All).map(|r| r.outcome);
            assert!(
                f.fault.is_none() && outcome != Some(kernel::Outcome::Invalid),
                "{name}: {:?}\n{text}",
                f.diagnostics
            );
            let names = answer::names(&kernel::vocabulary(&text, &name).expect("parses"));
            for q in f.queries.iter().flatten() {
                let Some(t) = &q.goal_text else { continue };
                for c in &q.classes {
                    for (i, row) in c.rows.iter().enumerate() {
                        let pairs: Vec<(String, String)> =
                            row.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                        assert_eq!(
                            format::fill(t, &pairs),
                            c.lines[i],
                            "{name}: the line filled again is the kernel's"
                        );
                        let (line, values) = Formatters::standard().fill(t, &pairs, &names);
                        for v in &values {
                            assert!(
                                !numeral(&v.text),
                                "{name}: {} prints as {}",
                                v.written,
                                v.text
                            );
                        }
                        assert!(!line.contains("(n "), "{name}: {line}");
                        lines += 1;
                    }
                }
            }
        }
    }
    assert!(lines > 48, "the problems' lines were read: {lines}");
}

// ── The Zebra puzzle ───────────────────────────────────────────────

/// **The Zebra puzzle's steps by hand, through the one shot**: each step's
/// program judged at root, the complete program the target shape, its
/// answer the program's own sentence, which the reader scores right; the
/// library's rules root fired, by name.
#[test]
fn the_zebra_puzzles_steps_by_hand_go_through_the_one_shot_to_its_answer() {
    let file = |n: &str| read(&format!("contract/zebra/{n}"));
    let parts = formalize::hand_parts(&file("step1.ein"), &file("step2.ein"), &file("step3.ein"));
    let mut rt = Scripted {
        answers: parts.to_vec(),
        envelope: envelope(),
    };
    let input = Input {
        id: "zebra".to_string(),
        text: file("text.txt"),
        answer_line: prompts::answer_line("zebra").expect("an answer line"),
    };
    let mut config = Config::default();
    config.options.clock_s = 120;
    let ft = from_text::run(&mut rt, &input, &config, &mut Vec::new()).expect("runs");
    let steps: Vec<&str> = ft.run.steps.iter().map(|s| s.step.as_str()).collect();
    assert_eq!(steps, ["1", "2", "3"]);
    for s in &ft.run.steps[..2] {
        let at_root = &s.judged.as_ref().expect("judged").feedback[0];
        assert_eq!(
            at_root.outcome_over(Part::All).map(|r| r.outcome),
            Some(kernel::Outcome::NotDecided),
            "step {} at root",
            s.step
        );
    }
    assert_eq!(ft.answer.case, Case::Lines);
    assert_eq!(
        ft.answer.text(),
        "The Norwegian drinks water. The Japanese owns the zebra."
    );
    assert_eq!(
        ft.answer.text(),
        ft.answer.kernel_text(),
        "names need no formatter"
    );
    assert_eq!(
        answer::score(&ft.answer, "Norwegian water|Japanese zebra").verdict,
        "right"
    );
    let fired = &ft.run.complete.as_ref().expect("judged").root.fired;
    println!("the reference's library rules fired at root: {fired:?}");
    assert!(fired.keys().any(|r| r.contains("slot")), "{fired:?}");
}

/// The recorded run's options are the default's: the one shot replays it as
/// recorded, with nothing but the calls.
#[test]
fn the_one_shots_defaults_keep_the_forms_an_output_the_budget_cut() {
    let o = Config::default().options;
    assert_eq!(o.cut, formalize::Cut::KeepForms);
    assert_eq!(
        o,
        Options {
            cut: formalize::Cut::KeepForms,
            ..Options::default()
        }
    );
    let (kept, off) = formalize::keep_forms("(p a)\n;; c\n(q b)\n(q b)\n(q b)\n(r (c");
    assert_eq!(kept, "(p a)\n;; c\n(q b)\n");
    assert_eq!((off.kept, off.repeats), (2, 2));
    let _: Option<Run> = None;
}

/// **A directory of modules given beside the library** (`--module-dir`):
/// its modules, and no problem's file, shown by their forms, with no comment
/// of theirs; offered to step 2's grammar, which accepts their imports; and
/// found from the directory when the program is loaded, so that a word
/// problem's own program answers, its quantity in the word theory's words.
#[test]
fn a_directory_of_modules_is_shown_offered_to_the_grammars_and_loaded() {
    let dir = ein_corpus::repo_root().join("examples/word_problems");
    let c = einaf::catalogue::Catalogue::new(einaf::catalogue::Size::Whole)
        .with_modules(&dir)
        .expect("the directory's modules");
    let given = c.given.as_ref().expect("given");
    let names: Vec<&str> = given.modules.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(names, ["gaps", "listed_is_all", "readings", "words"]);
    let text = c.text();
    assert!(text.contains("# Modules given beside the library"));
    assert!(
        text.contains("(relation has Who Kind Number Moment"),
        "the declarations"
    );
    assert!(
        !text.contains("Bob has 5 apples"),
        "no problem and no comment is shown"
    );
    let g = einaf::grammar::steps::render(
        einaf::grammar::steps::Step::Two,
        &einaf::grammar::steps::Params {
            sentences: None,
            catalogue: c.grammar(),
            program: kernel::Vocabulary::default(),
        },
    );
    let compiled = einaf::grammar::gbnf::Compiled::parse(&g.text).expect("reads");
    let imports = "(import words :symbols (words has gave asked amount value))\n\
                   (import std.math.decimal-arith)\n\
                   (import std.math.decimal :symbols (n))\n\
                   (import std.math.arith :symbols (z))";
    assert!(
        compiled.accepts(&g.root, imports).unwrap().accepted(),
        "step 2's grammar accepts the given modules' imports"
    );
    let mut config = whole();
    config.module_dir = Some(dir.clone());
    let program = std::fs::read_to_string(dir.join("bob1.ein")).expect("bob1");
    let ft = one_shot(
        &program,
        "Bob has 5 apples. He gave 2 to Alice. How many apples does Bob have?\n",
        &config,
        None,
    );
    assert_eq!(
        ft.run.module_dir.as_deref(),
        Some(dir.display().to_string().as_str())
    );
    assert_eq!(ft.answer.kernel_text(), "(amount Bob apples t1): (n z 3)");
    assert_eq!(ft.answer.text(), "how many apples Bob has at t1: 3");
    assert_eq!(answer::score(&ft.answer, "3").verdict, "right");
}
