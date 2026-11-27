//! **A text's analysis by trained parsers, with no parser and no model**:
//! the recorded analyses of the six texts with steps by hand, committed
//! under `einaf/tests/fixtures/analysis/`, read and written in each form; the
//! facts loaded by the kernel with no refusal and drawn by `ein render kb`;
//! step 1 given an analysis, its request the plain one with the analysis
//! after the text, and the run's record naming it; a run given none, its
//! step 1 the request a recorded run of the model made; and the flags of
//! `einaf steps run` and `einaf from-text` end to end, on calls a scripted
//! run recorded here.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;

use einaf::analysis::{self, Choice, Form, Given, penman};
use einaf::catalogue::Catalogue;
use einaf::formalize::{self, Input, Mode, Options, Run};
use einaf::from_text::{self, Config};
use einaf::kernel::{self, Bounds, Outcome, Part, Question, Source};
use einaf::model::replay::Replay;
use einaf::model::{Failure, Generation, Message, Prompt, Recorded, Request, Runtime, sha256_hex};
use einaf::prompts;

const TEXTS: [&str; 6] = ["apples", "more", "seating", "together", "youngest", "zebra"];
const PARSERS: [&str; 3] = ["spring", "xfm", "ibm"];

fn package() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(package().join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

fn file(text: &str) -> PathBuf {
    package()
        .join("tests/fixtures/analysis")
        .join(format!("{text}.json"))
}

fn text_of(text: &str) -> String {
    read(&format!("contract/{text}/text.txt"))
}

fn given(text: &str, parser: &str, form: Form, entities: bool) -> Given {
    let choice = Choice {
        parser: parser.to_string(),
        form,
        entities,
    };
    Given::read(&file(text), choice).unwrap_or_else(|e| panic!("{text}: {e}"))
}

fn bounds() -> Bounds {
    Bounds {
        clock: Some(std::time::Duration::from_secs(60)),
        ..Bounds::default()
    }
}

/// A directory of the test's own under the temporary directory, removed
/// when dropped.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let d = std::env::temp_dir().join(format!("einaf-analysis-{name}-{}", std::process::id()));
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

/// **Every recorded analysis is of its text, and every graph reads**: each
/// of the three parsers a graph a sentence, in the sentences EinAf splits the
/// text into; a learned aligner's every node aligned; and the counts the
/// check took of the three parsers' nodes and aligned nodes, over the six
/// texts.
#[test]
fn every_recorded_analysis_is_of_its_text_and_every_graph_reads() {
    let mut totals: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for t in TEXTS {
        for p in PARSERS {
            let g = given(t, p, Form::Penman, true);
            g.is_of(&text_of(t))
                .unwrap_or_else(|e| panic!("{t}/{p}: {e}"));
            for r in g.graphs() {
                assert!(r.graph.is_ok(), "{t}/{p}: sentence {} reads no graph", r.n);
            }
            let c = g.counts();
            assert_eq!(c.graphs, c.sentences, "{t}/{p}");
            if g.parse().unwrap().aligner.kind == "learned" {
                assert_eq!(c.aligned, c.nodes, "{t}/{p}: a learned aligner left a node");
            }
            let e = totals.entry(p).or_default();
            e.0 += c.nodes;
            e.1 += c.aligned;
        }
    }
    assert_eq!(totals["spring"], (181, 95));
    assert_eq!(totals["xfm"], (183, 97));
    assert_eq!(totals["ibm"], (182, 182));
}

/// **The text forms hold every sentence, and no aligner's marker**: PENMAN
/// a graph under each `# ::snt`, as written less its markers; a node a
/// line under each sentence, the words of a learned aligner in brackets
/// and a rule-based one's nowhere; the named entities beside where asked.
#[test]
fn the_text_forms_hold_every_sentence_and_no_marker() {
    for t in TEXTS {
        for p in PARSERS {
            let g = given(t, p, Form::Penman, false);
            let form = g.form();
            for (i, s) in g.analysis.sentences.iter().enumerate() {
                assert!(form.contains(&format!("# ::snt {s}\n")), "{t}/{p}: {s}");
                let written = g.parse().unwrap().graphs[i].penman.as_deref().unwrap();
                assert!(
                    form.contains(&penman::without_alignments(written).unwrap()),
                    "{t}/{p}: sentence {} is not as the parser printed it",
                    i + 1
                );
            }
            assert!(!form.contains('~'), "{t}/{p}: a marker is left");
            assert!(!form.contains("named entities"), "{t}/{p}");

            let lines = given(t, p, Form::Lines, true);
            let form = lines.form();
            for (i, s) in lines.analysis.sentences.iter().enumerate() {
                assert!(form.contains(&format!("Sentence {}: {s}\n", i + 1)));
            }
            let bracketed = form.lines().filter(|l| l.contains(" [")).count();
            if lines.words_shown() {
                assert_eq!(bracketed, lines.counts().nodes, "{t}/{p}");
            } else {
                assert_eq!(
                    bracketed, 0,
                    "{t}/{p}: a rule-based aligner's words are shown"
                );
            }
            for e in lines
                .analysis
                .entities
                .iter()
                .flat_map(|e| &e.sentences)
                .flatten()
            {
                assert!(
                    form.contains(&format!("{} ({})", e.text, e.label)),
                    "{t}: {e:?}"
                );
            }
        }
    }
}

/// **The facts form loads with no refusal and is judged at root alone**:
/// every text under every parser, the named entities beside, through the
/// bounded call: *not decided* under `--no-search` (root alive, R not
/// searched), *one* under `--single`; the writer of H `no-search`, no
/// candidate; each fact's `:source` a sentence of the text; the count of
/// facts the converter gives, the facts the program writes.
#[test]
fn the_facts_form_loads_with_no_refusal_and_is_judged_at_root_alone() {
    for t in TEXTS {
        let sentences = einaf::contract::sentences(&text_of(t));
        for p in PARSERS {
            let g = given(t, p, Form::Facts, true);
            let program = analysis::facts::program(&g);
            let name = format!("{t}.{p}.facts.ein");
            let src = Source::Text {
                text: &program.text,
                name: &name,
                base_dir: None,
            };
            let f = kernel::call(
                src,
                &[Question::NoSearch, Question::Single(Part::All)],
                bounds(),
            );
            let word = |i: usize| f[i].outcome.as_ref().unwrap()[0].outcome;
            assert_eq!(
                word(0),
                Outcome::NotDecided,
                "{name}: {:?}",
                f[0].diagnostics
            );
            assert_eq!(word(1), Outcome::One, "{name}");
            assert!(
                f[0].diagnostics.is_empty(),
                "{name}: {:?}",
                f[0].diagnostics
            );
            let root = kernel::root(src, bounds());
            assert_eq!(root.writer.as_deref(), Some("no-search"), "{name}");
            assert_eq!(root.candidates, 0, "{name}");
            let facts: Vec<&str> = program
                .text
                .lines()
                .filter(|l| l.starts_with("(amr-") || l.starts_with("(named-entity"))
                .collect();
            assert_eq!(facts.len(), program.facts, "{name}");
            for l in facts {
                let source = l
                    .split(":source \"")
                    .nth(1)
                    .and_then(|s| s.strip_suffix("\")"))
                    .unwrap_or_else(|| panic!("{name}: no :source: {l}"));
                assert!(sentences.iter().any(|s| s == source), "{name}: {l}");
            }
        }
    }
}

fn ein_bin() -> PathBuf {
    let bin = std::env::var_os("EIN_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| ein_corpus::repo_root().join("ein.rs/target/gate/ein"));
    assert!(
        bin.is_file(),
        "{}: no `ein` binary to draw with; `cargo build --manifest-path ein.rs/Cargo.toml \
         --profile gate -p ein-cli` builds it, and `$EIN_BIN` names another",
        bin.display()
    );
    bin
}

/// **`ein render kb` draws the facts**: the SPRING facts of each text with
/// the entities beside, an arrow for each argument of each fact and more.
#[test]
fn ein_render_kb_draws_the_facts() {
    let scratch = Scratch::new("kb");
    for t in TEXTS {
        let g = given(t, "spring", Form::Facts, true);
        let program = analysis::facts::program(&g);
        let path = scratch.0.join(format!("{t}.ein"));
        std::fs::write(&path, &program.text).expect("written");
        let out = Command::new(ein_bin())
            .args(["render", "kb"])
            .arg(&path)
            .output()
            .expect("ein runs");
        assert!(
            out.status.success(),
            "{t}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let dot = String::from_utf8(out.stdout).expect("UTF-8");
        assert!(dot.starts_with("digraph"), "{t}");
        assert!(dot.matches("->").count() > 2 * program.facts, "{t}");
    }
}

/// **A runtime that answers each request with the next of its texts**, in a
/// fenced block, with the envelope of a recorded call; every request kept.
struct Scripted {
    answers: Vec<String>,
    envelope: Generation,
    asked: Vec<Request>,
}

impl Scripted {
    fn hand(text: &str) -> Scripted {
        let f = |n: &str| read(&format!("contract/{text}/{n}"));
        let line = read("tests/fixtures/model/calls.jsonl")
            .lines()
            .next()
            .expect("a recorded call")
            .to_string();
        let r: Recorded = serde_json::from_str(&line).expect("a record");
        Scripted {
            answers: formalize::hand_parts(&f("step1.ein"), &f("step2.ein"), &f("step3.ein"))
                .to_vec(),
            envelope: r.generation.expect("a generation"),
            asked: Vec::new(),
        }
    }
}

impl Runtime for Scripted {
    fn generate(&mut self, request: &Request) -> Result<Generation, Failure> {
        self.asked.push(request.clone());
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

fn input(text: &str) -> Input {
    Input {
        id: text.to_string(),
        text: text_of(text),
        answer_line: prompts::answer_line("generic").expect("an answer line"),
    }
}

fn user(r: &Request) -> &str {
    let Prompt::Chat { messages, .. } = &r.prompt else {
        panic!("a chat")
    };
    let m: &Message = messages.last().expect("a user message");
    &m.content
}

fn run(text: &str, analysis: Option<&Given>) -> (Run, Scripted, Vec<Recorded>) {
    let mut rt = Scripted::hand(text);
    let mut calls = Vec::new();
    let options = Options::default();
    let run = formalize::run_given(
        &mut rt,
        &input(text),
        options,
        Catalogue::new(options.catalogue),
        analysis,
        &mut calls,
    )
    .expect("runs");
    (run, rt, calls)
}

/// **Step 1 given an analysis holds it after the text, and the record names
/// it**: on the Zebra puzzle's steps by hand, a generation a step, in each
/// form, step 1's request is the plain one with the block after the text
/// and nothing else, under `steps/step1-analysis.v1.md`; every later request
/// is the plain run's; the program is the plain run's; the record keeps the
/// file's digest, the parser, the form and the block's digest, and a run
/// given none writes no `analysis` key.
#[test]
fn step_1_given_an_analysis_holds_it_after_the_text_and_the_record_names_it() {
    let (plain, plain_rt, _) = run("zebra", None);
    assert_eq!(plain.steps[0].template, prompts::STEP1.file);
    assert!(plain.analysis.is_none());
    let json = serde_json::to_value(&plain).expect("JSON");
    assert!(
        json.get("analysis").is_none(),
        "a run given none writes the key"
    );
    for (form, entities) in [
        (Form::Penman, false),
        (Form::Lines, true),
        (Form::Facts, true),
    ] {
        let g = given("zebra", "spring", form, entities);
        let (with, rt, _) = run("zebra", Some(&g));
        assert_eq!(with.steps[0].template, prompts::STEP1_ANALYSIS.file);
        assert_eq!(
            with.steps[0].template_sha256,
            prompts::STEP1_ANALYSIS.sha256()
        );
        let block = g.block();
        let asked = user(&rt.asked[0]);
        assert!(asked.contains(block.trim_end()), "{form:?}");
        assert_eq!(
            asked.replacen(&format!("{}\n\n", block.trim_end()), "", 1),
            user(&plain_rt.asked[0]),
            "{form:?}: step 1's request differs from the plain one by more than the analysis"
        );
        assert_eq!(rt.asked.len(), plain_rt.asked.len());
        for (a, b) in rt.asked.iter().zip(&plain_rt.asked).skip(1) {
            assert_eq!(
                a.digest(),
                b.digest(),
                "{form:?}: a later step's request moved"
            );
        }
        assert_eq!(with.program, plain.program);
        let a = with
            .analysis
            .as_ref()
            .expect("the record names the analysis");
        assert_eq!(
            a.sha256,
            sha256_hex(read("tests/fixtures/analysis/zebra.json").as_bytes())
        );
        assert_eq!((a.parser.as_str(), a.form), ("spring", form));
        assert_eq!(a.block_sha256, sha256_hex(block.as_bytes()));
        assert_eq!(a.template_sha256, form.template().sha256());
        assert_eq!(a.entities.is_some(), entities);
        assert_eq!(a.vocabulary_sha256.is_some(), form == Form::Facts);
        assert!(!a.words, "SPRING's aligner is rule-based: no word is shown");
        let back: Run = serde_json::from_value(serde_json::to_value(&with).unwrap()).unwrap();
        assert_eq!(back.analysis, with.analysis);
    }
    let ibm = given("zebra", "ibm", Form::Lines, false);
    let (with, _, _) = run("zebra", Some(&ibm));
    assert!(
        with.analysis.unwrap().words,
        "IBM's aligner is learned: its words are shown"
    );
}

/// **A run given no analysis sends the requests it sent**: step 1 of each
/// recorded run of the model, replayed, finds its call by its digest, under
/// `steps/step1.v1.md` and the digest the record names.
#[test]
fn a_run_given_no_analysis_sends_the_step_1_request_it_sent() {
    let dir = package().join("tests/fixtures/steps/bob1");
    let recorded: Run = serde_json::from_str(&read("tests/fixtures/steps/bob1/run.json")).unwrap();
    let mut replay = Replay::from_file(&dir.join("calls.jsonl")).expect("the calls");
    let mut calls = Vec::new();
    let again = formalize::run(
        &mut replay,
        &Input {
            id: recorded.id.clone(),
            text: read("tests/fixtures/steps/bob1/text.txt"),
            answer_line: prompts::answer_line(
                recorded
                    .answer_line
                    .trim_start_matches("answer-lines/")
                    .split('.')
                    .next()
                    .unwrap_or("generic"),
            )
            .unwrap(),
        },
        recorded.options,
        &mut calls,
    );
    assert_eq!(again.steps[0].template, recorded.steps[0].template);
    assert_eq!(
        again.steps[0].template_sha256,
        recorded.steps[0].template_sha256
    );
    assert_eq!(again.steps[0].digest, recorded.steps[0].digest);
    assert!(calls[0].failure.is_none(), "step 1's request found no call");

    let generations = read("tests/fixtures/from_text/zebra-contradiction/generations.jsonl");
    let mut replay = Replay::from_jsonl(&generations).expect("the calls");
    let mut calls = Vec::new();
    let mut config = Config::default();
    config.options.clock_s = 120;
    let ft = from_text::run(
        &mut replay,
        &Input {
            id: "zebra-contradiction".to_string(),
            text: read("tests/fixtures/from_text/zebra-contradiction/text.txt"),
            answer_line: prompts::answer_line("zebra").unwrap(),
        },
        &config,
        &mut calls,
    )
    .expect("runs");
    assert!(calls[0].failure.is_none(), "step 1's request found no call");
    assert_eq!(
        ft.run.steps[0].program.as_deref(),
        Some(read("tests/fixtures/from_text/zebra-contradiction/step1.ein").as_str())
    );
}

/// **An analysis of other sentences, or given to the whole program in one
/// call, is refused before any call**; so is a parser the file does not
/// hold, and named entities a file does not hold.
#[test]
fn an_analysis_that_does_not_fit_the_run_is_refused_before_any_call() {
    let g = given("zebra", "spring", Form::Penman, false);
    let mut rt = Scripted::hand("apples");
    let mut calls = Vec::new();
    let options = Options::default();
    let e = formalize::run_given(
        &mut rt,
        &input("apples"),
        options,
        Catalogue::new(options.catalogue),
        Some(&g),
        &mut calls,
    )
    .unwrap_err();
    assert!(e.contains("other sentences"), "{e}");
    assert!(rt.asked.is_empty() && calls.is_empty());

    let whole = Options {
        mode: Mode::Whole,
        ..Options::default()
    };
    let e = formalize::run_given(
        &mut rt,
        &input("zebra"),
        whole,
        Catalogue::new(whole.catalogue),
        Some(&g),
        &mut calls,
    )
    .unwrap_err();
    assert!(e.contains("whole program"), "{e}");
    assert!(rt.asked.is_empty() && calls.is_empty());

    let bad = Choice {
        parser: "amrbart".to_string(),
        ..Choice::default()
    };
    let e = Given::read(&file("zebra"), bad).unwrap_err();
    assert!(e.contains("spring, xfm, ibm"), "{e}");
    let mut a: analysis::Analysis =
        serde_json::from_str(&read("tests/fixtures/analysis/zebra.json")).unwrap();
    a.entities = None;
    let entities = Choice {
        entities: true,
        ..Choice::default()
    };
    let e = Given::of(&serde_json::to_string(&a).unwrap(), "none.json", entities).unwrap_err();
    assert!(e.contains("no named entities"), "{e}");
}

/// **Every template of the analysis renders with its slots and no other**,
/// and step 1's with the analysis is step 1's with the block after the text.
#[test]
fn every_template_of_the_analysis_renders_with_its_slots_and_no_other() {
    let fill = |names: &[&'static str]| -> BTreeMap<&'static str, String> {
        names.iter().map(|n| (*n, format!("<{n}>"))).collect()
    };
    let cases: [(prompts::Template, &[&'static str]); 5] = [
        (
            prompts::STEP1_ANALYSIS,
            &["examples", "text", "answer_line", "analysis"],
        ),
        (analysis::PENMAN, &["analysis", "entities"]),
        (analysis::LINES, &["analysis", "entities"]),
        (analysis::FACTS, &["analysis", "entities"]),
        (analysis::ENTITIES, &[]),
    ];
    for (t, names) in cases {
        let text = t.render(&fill(names)).expect("renders");
        assert!(!text.contains("%%") && !text.contains("{{"), "{}", t.file);
        let mut more = fill(names);
        more.insert("unknown", String::new());
        assert!(t.render(&more).is_err(), "{}", t.file);
        if let Some(first) = names.first() {
            let mut fewer = fill(names);
            fewer.remove(first);
            assert!(t.render(&fewer).is_err(), "{}", t.file);
        }
    }
    let plain = prompts::STEP1
        .render(&fill(&["examples", "text", "answer_line"]))
        .unwrap();
    let with = prompts::STEP1_ANALYSIS
        .render(&fill(&["examples", "text", "answer_line", "analysis"]))
        .unwrap();
    assert_eq!(with.replacen("<analysis>\n\n", "", 1), plain);
    let vocabulary = kernel::parse(analysis::VOCABULARY.text, "vocabulary");
    assert_eq!(vocabulary.forms, Some(8), "{:?}", vocabulary.findings);
    assert!(
        !analysis::VOCABULARY.text.contains(['`', '<', '>', '|']),
        "the vocabulary holds a character no grammar lets the model write"
    );
}

fn einaf(args: &[&std::ffi::OsStr]) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_einaf"))
        .args(args)
        .current_dir(ein_corpus::repo_root())
        .output()
        .expect("einaf runs");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8(out.stdout).expect("UTF-8"),
        String::from_utf8(out.stderr).expect("UTF-8"),
    )
}

/// **`einaf steps run` and `einaf from-text` take an analysis, and replay
/// it**: the calls of a scripted run of the Zebra puzzle's steps by hand
/// given the analysis, replayed through each command with `--analysis`,
/// every request found, step 1 under its template, the record naming the
/// analysis; an option of the analysis with no file, and an analysis of
/// another text, refused with the usage's exit before any call; `einaf
/// analysis show` and `counts`.
#[test]
fn einaf_steps_run_and_from_text_take_an_analysis_and_replay_it() {
    let scratch = Scratch::new("cli");
    let g = given("zebra", "spring", Form::Facts, true);
    let (scripted, _, calls) = run("zebra", Some(&g));
    let record = scratch.0.join("calls.jsonl");
    let lines: Vec<String> = calls
        .iter()
        .map(|c| serde_json::to_string(c).expect("JSON"))
        .collect();
    std::fs::write(&record, lines.join("\n") + "\n").expect("written");
    let text = package().join("contract/zebra/text.txt");
    let analysis = file("zebra");
    let o = |s: &str| std::ffi::OsString::from(s);
    let given_args = [
        o("--analysis"),
        analysis.clone().into(),
        o("--analysis-form"),
        o("facts"),
        o("--analysis-entities"),
    ];

    let out = scratch.0.join("steps");
    let mut args = vec![
        o("steps"),
        o("run"),
        text.clone().into(),
        o("--out"),
        out.clone().into(),
    ];
    args.extend([o("--replay"), record.clone().into()]);
    args.extend(given_args.iter().cloned());
    let refs: Vec<&std::ffi::OsStr> = args.iter().map(|a| a.as_os_str()).collect();
    let (code, stdout, stderr) = einaf(&refs);
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("step 1 given the analysis"), "{stdout}");
    let again: Run =
        serde_json::from_str(&std::fs::read_to_string(out.join("run.json")).unwrap()).unwrap();
    assert!(
        again.steps.iter().all(|s| s.failure.is_none()),
        "a request found no call"
    );
    assert_eq!(again.steps[0].template, prompts::STEP1_ANALYSIS.file);
    assert_eq!(again.program, scripted.program);
    assert_eq!(again.analysis.as_ref().map(|a| a.form), Some(Form::Facts));

    let dir = scratch.0.join("from-text");
    let mut args = vec![
        o("from-text"),
        text.clone().into(),
        o("--record"),
        dir.clone().into(),
    ];
    args.extend([o("--replay"), record.clone().into()]);
    args.extend(given_args.iter().cloned());
    let refs: Vec<&std::ffi::OsStr> = args.iter().map(|a| a.as_os_str()).collect();
    let (code, _, stderr) = einaf(&refs);
    assert_eq!(code, 0, "{stderr}");
    assert!(stderr.contains("step 1 given the analysis"), "{stderr}");
    let config: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("config.json")).unwrap()).unwrap();
    assert_eq!(config["analysis"]["form"], "facts");
    assert_eq!(config["analysis"]["parser"], "spring");
    let ft: from_text::FromText =
        serde_json::from_str(&std::fs::read_to_string(dir.join("run.json")).unwrap()).unwrap();
    assert!(ft.run.steps.iter().all(|s| s.failure.is_none()));
    assert_eq!(ft.run.program, scripted.program);

    // Refused before any call: an option of the analysis's with no file, an
    // analysis of another text.
    let no_file = [
        o("steps"),
        o("run"),
        text.clone().into(),
        o("--out"),
        out.clone().into(),
        o("--replay"),
        record.clone().into(),
        o("--analysis-form"),
        o("lines"),
    ];
    let refs: Vec<&std::ffi::OsStr> = no_file.iter().map(|a| a.as_os_str()).collect();
    let (code, _, stderr) = einaf(&refs);
    assert_eq!(code, 2, "{stderr}");
    assert!(
        stderr.contains("--analysis-form takes --analysis FILE"),
        "{stderr}"
    );
    let other = [
        o("from-text"),
        package().join("contract/apples/text.txt").into(),
        o("--replay"),
        record.clone().into(),
        o("--analysis"),
        analysis.clone().into(),
    ];
    let refs: Vec<&std::ffi::OsStr> = other.iter().map(|a| a.as_os_str()).collect();
    let (code, _, stderr) = einaf(&refs);
    assert_eq!(code, 2, "{stderr}");
    assert!(stderr.contains("other sentences"), "{stderr}");

    let show = [
        o("analysis"),
        o("show"),
        analysis.clone().into(),
        o("--form"),
        o("facts"),
    ];
    let refs: Vec<&std::ffi::OsStr> = show.iter().map(|a| a.as_os_str()).collect();
    let (code, stdout, _) = einaf(&refs);
    assert_eq!(code, 0);
    assert_eq!(
        stdout,
        analysis::facts::program(&given("zebra", "spring", Form::Facts, false)).text
    );
    let counts = [o("analysis"), o("counts"), analysis.into()];
    let refs: Vec<&std::ffi::OsStr> = counts.iter().map(|a| a.as_os_str()).collect();
    let (code, stdout, _) = einaf(&refs);
    assert_eq!(code, 0);
    assert!(
        stdout.contains("zebra\tibm\tlearned\t17/17\t94\t94\t0\t"),
        "{stdout}"
    );
}
