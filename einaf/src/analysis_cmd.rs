//! `einaf analysis`: a text's recorded analysis written in one of its forms,
//! as step 1's prompt holds it or alone; its counts. And the options by which
//! `einaf steps run` and `einaf from-text` give step 1 an analysis.

use std::path::PathBuf;
use std::process::ExitCode;

use einaf::analysis::{self, Choice, Form, Given};

use crate::usage_error;

/// **The options that give a run an analysis**: `--analysis FILE`,
/// `--analysis-parser P`, `--analysis-form F`, `--analysis-entities`.
pub struct AnalysisArgs {
    file: Option<PathBuf>,
    choice: Choice,
    /// An option of the analysis's other than its file was given.
    other: Option<&'static str>,
}

impl AnalysisArgs {
    pub fn new() -> AnalysisArgs {
        AnalysisArgs {
            file: None,
            choice: Choice::default(),
            other: None,
        }
    }

    /// Take `a` where it is an option of the analysis: `Ok(true)`, its value
    /// read from `it`.
    pub fn take(&mut self, a: &str, it: &mut std::slice::Iter<'_, String>) -> Result<bool, String> {
        let mut value = |name: &str| it.next().cloned().ok_or(format!("{name} takes a value"));
        match a {
            "--analysis" => self.file = Some(value("--analysis")?.into()),
            "--analysis-parser" => {
                self.choice.parser = value("--analysis-parser")?;
                self.other = Some("--analysis-parser");
            }
            "--analysis-form" => {
                let v = value("--analysis-form")?;
                self.choice.form = Form::parse(&v)
                    .ok_or(format!("--analysis-form {v}: penman, lines or facts"))?;
                self.other = Some("--analysis-form");
            }
            "--analysis-entities" => {
                self.choice.entities = true;
                self.other = Some("--analysis-entities");
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    /// The analysis given, read and checked, or none.
    pub fn open(self) -> Result<Option<Given>, String> {
        match (self.file, self.other) {
            (Some(f), _) => Given::read(&f, self.choice).map(Some),
            (None, Some(o)) => Err(format!("{o} takes --analysis FILE beside it")),
            (None, None) => Ok(None),
        }
    }
}

/// A run's analysis in a line, for a run's summary.
pub fn line(a: &analysis::AnalysisOf) -> String {
    format!(
        "step 1 given the analysis {} ({}): {}'s graphs as {}{}{}",
        a.file,
        a.sha256,
        a.parser,
        a.form.word(),
        if a.words { ", a node's words" } else { "" },
        if a.entities.is_some() {
            ", the named entities beside"
        } else {
            ""
        }
    )
}

/// `einaf analysis show|counts`.
pub fn analysis(args: &[String]) -> ExitCode {
    match args.first().map(String::as_str) {
        Some("show") => show(&args[1..]),
        Some("counts") => counts(&args[1..]),
        _ => usage_error("analysis takes `show` or `counts`"),
    }
}

/// `einaf analysis show FILE [--parser P] [--form F] [--entities] [--block]`.
fn show(args: &[String]) -> ExitCode {
    let mut choice = Choice::default();
    let (mut file, mut block) = (None::<PathBuf>, false);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let r: Result<(), String> = (|| {
            match a.as_str() {
                "--parser" => choice.parser = it.next().cloned().ok_or("--parser takes a value")?,
                "--form" => {
                    let v = it.next().ok_or("--form takes a value")?;
                    choice.form =
                        Form::parse(v).ok_or(format!("--form {v}: penman, lines or facts"))?;
                }
                "--entities" => choice.entities = true,
                "--block" => block = true,
                s if s.starts_with('-') => return Err(format!("unknown option {s}")),
                s if file.is_none() => file = Some(s.into()),
                s => return Err(format!("one file only: {s}")),
            }
            Ok(())
        })();
        if let Err(e) = r {
            return usage_error(&e);
        }
    }
    let Some(file) = file else {
        return usage_error("analysis show takes a FILE");
    };
    match Given::read(&file, choice) {
        Ok(g) => {
            print!("{}", if block { g.block() } else { g.form() });
            ExitCode::SUCCESS
        }
        Err(e) => usage_error(&e),
    }
}

/// `einaf analysis counts FILE...`: for each file and each parser, its
/// graphs, its nodes, the nodes its aligner gave a word, and the facts its
/// facts form holds.
fn counts(args: &[String]) -> ExitCode {
    if args.is_empty() {
        return usage_error("analysis counts takes one FILE or more");
    }
    println!("text\tparser\taligner\tgraphs\tnodes\taligned\tno word\tfacts");
    for f in args {
        let first = match Given::read(std::path::Path::new(f), Choice::default()) {
            Ok(g) => g,
            Err(e) => return usage_error(&e),
        };
        for p in &first.analysis.amr {
            let g = Given {
                choice: Choice {
                    parser: p.parser.clone(),
                    form: Form::Facts,
                    entities: false,
                },
                ..first.clone()
            };
            let c = g.counts();
            println!(
                "{}\t{}\t{}\t{}/{}\t{}\t{}\t{}\t{}",
                g.analysis.text,
                p.parser,
                p.aligner.kind,
                c.graphs,
                c.sentences,
                c.nodes,
                c.aligned,
                c.nodes - c.aligned,
                analysis::facts::program(&g).facts
            );
        }
    }
    ExitCode::SUCCESS
}
