//! **From a text to a short answer, one shot**: the formalization in steps
//! ([`crate::formalize`]), each step's program judged at root and kept; the
//! complete program asked the least questions its answer needs through the
//! bounded call; and the answer the program prints, its values formatted,
//! or EinAf's fixed text ([`crate::answer`]). No repair, and no step
//! generated a second time. `einaf from-text` is its command for one text,
//! and the loop's GENERATE and the harness's run call the same function.
//!
//! **The questions** are the least the answer needs: `--no-search` and
//! `--single`, which the formalization asks of the complete program, and a
//! description of R only where `--single`'s two states leave the answer
//! query's one class a lower bound ([`crate::answer::needs_description`]).
//! Each is one bounded call under the run's bounds.
//!
//! **The record** ([`write_record`]) is a directory: the configuration, every
//! call of the model slim ([`crate::model::Slim`]: the request's digest and
//! what came back, no request body), the program with its sidecar (the
//! categories and the check's findings by step), each step's program, every
//! feedback object, the answer, and the run whole. Replay gives the same
//! program and the same answer from the calls alone.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::answer::{self, Answer, Scored};
use crate::catalogue::Catalogue;
use crate::check::Finding as CheckFinding;
use crate::contract::Row;
use crate::formalize::{self, Input, OntologyChange, Options, Run};
use crate::format::Formatters;
use crate::kernel::{self, Bounds, Feedback, Question, Source};
use crate::model::{Recorded, Runtime, sha256_hex};
use crate::prompts;

/// The schema of a run's record of the one shot.
pub const SCHEMA: &str = "einaf-from-text/1";

/// **What a one shot is run with**: the formalization's options, a
/// directory of modules given beside the library, and the formatters on or
/// off.
#[derive(Clone, Debug)]
pub struct Config {
    pub options: Options,
    /// A theory given, the escape hatch of the ontology's provenance: a
    /// directory of theory modules the program may import; `None` in the
    /// main arms.
    pub module_dir: Option<PathBuf>,
    /// The answer's formatters; `false`, the kernel's lines as they stand.
    pub format: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            options: Options {
                cut: formalize::Cut::KeepForms,
                ..Options::default()
            },
            module_dir: None,
            format: true,
        }
    }
}

impl Config {
    /// The bounds of every kernel call of the run: the kernel's defaults,
    /// the clock the options set.
    pub fn bounds(&self) -> Bounds {
        Bounds {
            clock: Some(Duration::from_secs(self.options.clock_s)),
            ..Bounds::default()
        }
    }
}

/// **One shot, whole**: the formalization, the questions asked of the
/// complete program and their objects, and the answer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FromText {
    pub schema: String,
    /// The formalization: each call, each step's program and judgement, the
    /// complete program, its judgement and categories.
    pub run: Run,
    /// The questions asked of the complete program for its answer, each
    /// object in the order asked: `--no-search`, `--single`, and a
    /// description where the answer needed one.
    pub asked: Vec<Feedback>,
    pub answer: Answer,
    /// The bounds of every kernel call.
    pub bounds: BoundsOf,
}

/// The bounds a run's kernel calls were under, as the record keeps them.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundsOf {
    /// `--region-budget`, the kernel's facts budget.
    pub budget: u64,
    /// `--region-limit`, the kernel's cube limit.
    pub limit: u32,
    /// EinAf's cap on one saturation's firings, `--max-steps`.
    pub max_steps: Option<u64>,
    /// EinAf's clock on each call, `-T`, in seconds.
    pub clock_s: Option<u64>,
}

impl BoundsOf {
    fn of(b: Bounds) -> BoundsOf {
        BoundsOf {
            budget: b.budget,
            limit: b.limit,
            max_steps: b.max_steps,
            clock_s: b.clock.map(|c| c.as_secs()),
        }
    }
}

/// **Run the one shot on `input`**, each request answered by `runtime`;
/// every call is appended to `calls`. A module directory that cannot be read
/// is an error, before any call.
pub fn run(
    runtime: &mut dyn Runtime,
    input: &Input,
    config: &Config,
    calls: &mut Vec<Recorded>,
) -> Result<FromText, String> {
    let mut catalogue = Catalogue::new(config.options.catalogue);
    if let Some(dir) = &config.module_dir {
        catalogue = catalogue.with_modules(dir)?;
    }
    let run = formalize::run_with(runtime, input, config.options, catalogue, calls);
    Ok(answer_of(run, config))
}

/// **The answer of a formalization**: the questions its complete program's
/// answer needs past those the formalization asked, and the answer read off
/// their objects.
pub fn answer_of(run: Run, config: &Config) -> FromText {
    let bounds = config.bounds();
    let mut asked: Vec<Feedback> = run
        .complete
        .as_ref()
        .map(|c| c.feedback.clone())
        .unwrap_or_default();
    let formatters = if config.format {
        Formatters::standard()
    } else {
        Formatters::none()
    };
    let names = run
        .program
        .as_deref()
        .and_then(|p| kernel::vocabulary(p, &run.id).ok())
        .map(|v| answer::names(&v))
        .unwrap_or_default();
    if let (Some(program), Some(single)) = (
        run.program.as_deref(),
        asked.iter().find(|f| f.asked.ask == "single"),
    ) {
        let more = answer::more_questions(single);
        if !more.is_empty() {
            let name = format!("{}.ein", run.id);
            asked.extend(kernel::call(
                Source::Text {
                    text: program,
                    name: &name,
                    base_dir: config.module_dir.as_deref(),
                },
                &more,
                bounds,
            ));
        }
    }
    let mut answer = answer::read(&asked, &names, &formatters);
    if run.program.is_none() {
        answer.beneath.push(
            run.stopped
                .clone()
                .unwrap_or_else(|| "no program was written".to_string()),
        );
        if let Some(p) = run.steps.last().and_then(|s| s.parsed.as_ref()) {
            answer.beneath.extend(
                p.findings
                    .iter()
                    .map(|f| format!("{}: {}", f.kind, f.message)),
            );
        }
    }
    FromText {
        schema: SCHEMA.to_string(),
        run,
        asked,
        answer,
        bounds: BoundsOf::of(bounds),
    }
}

/// **The kernel's trace of the answer**, markdown: the deciding question
/// asked again of the complete program ([`kernel::explain`]); `None` where
/// there is no program.
pub fn explain(ft: &FromText, config: &Config) -> Option<Result<String, String>> {
    let program = ft.run.program.as_deref()?;
    let question = match ft.answer.question.as_deref() {
        Some("describe") => Question::Describe,
        Some("--no-search") => Question::NoSearch,
        _ => Question::Single(kernel::Part::All),
    };
    let name = format!("{}.ein", ft.run.id);
    Some(kernel::explain(
        Source::Text {
            text: program,
            name: &name,
            base_dir: config.module_dir.as_deref(),
        },
        question,
        config.bounds(),
    ))
}

/// **The sidecar of a program**: each form's category with the step that
/// wrote it, and the contract's check's findings of each step's program.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sidecar {
    pub schema: String,
    pub categories: Vec<Row>,
    /// Each step's findings, the complete program's last, under `complete`.
    pub findings: Vec<StepFindings>,
    pub ontology_change: Option<OntologyChange>,
}

/// The check's findings of one step's program.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepFindings {
    pub step: String,
    pub findings: Vec<CheckFinding>,
    /// The writer of H at that step, as the kernel reported it.
    pub writer: Option<String>,
}

/// The sidecar of a run.
pub fn sidecar(run: &Run) -> Sidecar {
    let mut findings: Vec<StepFindings> = run
        .steps
        .iter()
        .filter_map(|s| {
            s.judged.as_ref().map(|j| StepFindings {
                step: s.step.clone(),
                findings: j.check.findings.clone(),
                writer: j.root.writer.clone(),
            })
        })
        .collect();
    if let Some(c) = &run.complete {
        findings.push(StepFindings {
            step: "complete".to_string(),
            findings: c.check.findings.clone(),
            writer: c.root.writer.clone(),
        });
    }
    Sidecar {
        schema: "einaf-sidecar/1".to_string(),
        categories: run.categories.clone(),
        findings,
        ontology_change: run.ontology_change.clone(),
    }
}

/// **A run's configuration, as its record names it**.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigOf {
    pub schema: String,
    pub id: String,
    pub text_sha256: String,
    pub answer_line: String,
    pub answer_line_sha256: String,
    pub options: Options,
    pub module_dir: Option<String>,
    pub format: bool,
    pub bounds: BoundsOf,
    /// The questions asked of the complete program, as `ein solve`'s flags.
    pub questions: Vec<String>,
    pub kernel: Option<kernel::KernelId>,
    /// The model, its runtime, its context and its chat template, from the
    /// first call's envelope; `null` where no call came back.
    pub model: Option<serde_json::Value>,
    pub excerpt_sha256: String,
    pub catalogue_sha256: String,
    /// Each template the run rendered, and its digest.
    pub templates: Vec<(String, String)>,
    pub quoted: Vec<String>,
}

/// The configuration of a run.
pub fn config_of(ft: &FromText, config: &Config, calls: &[Recorded]) -> ConfigOf {
    let r = &ft.run;
    let model = calls.iter().find_map(|c| c.generation.as_ref()).map(|g| {
        serde_json::json!({
            "model": g.envelope.model,
            "runtime": g.envelope.runtime,
            "context": g.envelope.context,
            "template": g.envelope.template,
            "sampler": g.envelope.sampler,
        })
    });
    let mut templates: Vec<(String, String)> = r
        .steps
        .iter()
        .map(|s| (s.template.clone(), s.template_sha256.clone()))
        .collect();
    templates.push((prompts::SYSTEM.file.to_string(), prompts::SYSTEM.sha256()));
    templates.sort();
    templates.dedup();
    ConfigOf {
        schema: "einaf-from-text-config/1".to_string(),
        id: r.id.clone(),
        text_sha256: r.text_sha256.clone(),
        answer_line: r.answer_line.clone(),
        answer_line_sha256: r.answer_line_sha256.clone(),
        options: r.options,
        module_dir: r.module_dir.clone(),
        format: config.format,
        bounds: ft.bounds.clone(),
        questions: ft.asked.iter().map(|f| f.asked.question.clone()).collect(),
        kernel: ft.asked.first().map(|f| f.kernel.clone()),
        model,
        excerpt_sha256: r.excerpt_sha256.clone(),
        catalogue_sha256: r.catalogue_sha256.clone(),
        templates,
        quoted: r.quoted.clone(),
    }
}

/// One line of `kernel.jsonl`: whose call it was, and its object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KernelLine {
    /// The step whose program was judged, `complete` for the complete
    /// program's judgement, `answer` for a question the answer needed.
    pub step: String,
    pub feedback: Feedback,
}

/// **Write a run's record into `dir`**, which is made where it is missing:
/// `config.json`, `generations.jsonl` (every call slim), `program.ein` and
/// `program.sidecar.json`, `steps/stepN.ein` (each step's program),
/// `kernel.jsonl` (every feedback object), `answer.json` (the answer, and
/// its score where a gold answer was given) and `run.json` (the run whole).
pub fn write_record(
    dir: &Path,
    ft: &FromText,
    config: &Config,
    calls: &[Recorded],
    scored: Option<&Scored>,
) -> std::io::Result<()> {
    std::fs::create_dir_all(dir.join("steps"))?;
    let json = |v: &dyn erased::Json| v.pretty() + "\n";
    std::fs::write(dir.join("config.json"), json(&config_of(ft, config, calls)))?;
    let mut f = std::fs::File::create(dir.join("generations.jsonl"))?;
    for c in calls {
        writeln!(
            f,
            "{}",
            serde_json::to_string(&c.slim()).expect("a call is JSON")
        )?;
    }
    if let Some(p) = &ft.run.program {
        std::fs::write(dir.join("program.ein"), p)?;
    }
    std::fs::write(dir.join("program.sidecar.json"), json(&sidecar(&ft.run)))?;
    for s in &ft.run.steps {
        if let Some(p) = &s.program {
            std::fs::write(dir.join("steps").join(format!("step{}.ein", s.step)), p)?;
        }
    }
    let mut k = std::fs::File::create(dir.join("kernel.jsonl"))?;
    let mut line = |step: &str, f: &Feedback| -> std::io::Result<()> {
        writeln!(
            k,
            "{}",
            serde_json::to_string(&KernelLine {
                step: step.to_string(),
                feedback: f.clone(),
            })
            .expect("a line is JSON")
        )
    };
    for s in &ft.run.steps {
        if s.program.is_some() && ft.run.program.as_ref() != s.program.as_ref() {
            for f in s.judged.iter().flat_map(|j| &j.feedback) {
                line(&s.step, f)?;
            }
        }
    }
    for (i, f) in ft.asked.iter().enumerate() {
        let complete = ft.run.complete.as_ref().map_or(0, |c| c.feedback.len());
        line(if i < complete { "complete" } else { "answer" }, f)?;
    }
    let answer = serde_json::json!({
        "schema": "einaf-answer/1",
        "answer": ft.answer,
        "text": ft.answer.text(),
        "kernel": ft.answer.kernel_text(),
        "scored": scored,
    });
    std::fs::write(
        dir.join("answer.json"),
        serde_json::to_string_pretty(&answer).expect("JSON") + "\n",
    )?;
    std::fs::write(dir.join("run.json"), json(ft))?;
    Ok(())
}

/// The digest of a text, as a record names one.
pub fn digest(text: &str) -> String {
    sha256_hex(text.as_bytes())
}

mod erased {
    /// A value written as pretty JSON.
    pub trait Json {
        fn pretty(&self) -> String;
    }

    impl<T: serde::Serialize> Json for T {
        fn pretty(&self) -> String {
            serde_json::to_string_pretty(self).expect("a record's part is JSON")
        }
    }
}
