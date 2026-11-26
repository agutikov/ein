//! **The kernel as EinAf reads it**: the one module of EinAf that calls Ein's
//! library, `ein-ir` to load, `ein-infer` to ask, `ein-render` to read what
//! a run says beside its answer.
//!
//! **One door to an answer, the bounded call** ([`call`]): a program and the questions
//! asked of it go in, and one [`Feedback`] for each question comes out,
//! whatever the program is. The program is loaded once, as `ein solve`
//! loads it: the text parsed and loaded against the program file's
//! directory, then the judgement of the hypothesis mechanisms,
//! `ein_infer::mechanisms::judge_loaded`, which refuses what the binary
//! refuses. Each question is `ein solve`'s ([`Question`]), asked of a copy of
//! the load. Every saturation the call computes, the judgement's among them,
//! is under the call's [`Bounds`]: the clock and the cap on one saturation's
//! firings, which the kernel reads inside a saturation, and the facts budget
//! and the cube's limit, which its routes read. A run a bound stopped is an
//! object whose outcome is *not decided*. A panic of the kernel's is caught
//! here and is the object's `fault`, never an outcome of the program.
//!
//! **Everything is read off values**: a refusal by its findings, the
//! kernel's diagnostics, each with its kind and its place; a run's error by
//! its kind (`SolveError::kind`); the answer by the read-out's fields. No
//! part of the object is read back from text the kernel printed.
//!
//! **And one reading of a program that asks it nothing** ([`forms()`]): its
//! top-level forms, each as `ein_ir::dump_canonical` prints it and by its
//! shape, and what its imports bring, parsed and loaded as `ein solve` loads
//! it and nothing saturated, so that no bound is needed. It is what the
//! contract's categories are computed from.
//!
//! **And a parse alone** ([`parse()`]): whether a text is Ein's syntax,
//! with every error the parser reports, each with its kind and its place,
//! and nothing loaded. The probe of a model's output reads it beside the
//! bounded call, which loads what parses.
//!
//! **And a program's vocabulary** ([`vocabulary()`]): what its text
//! declares, imports and writes, read off its parse, which a step's grammar
//! is rendered on; and the standard library's modules as texts
//! ([`library()`]), which the grammars' catalogue is read from.
//!
//! Nothing else in EinAf names an item of Ein's crates, and nothing here
//! calls them but through [`call`], [`forms()`], [`shapes()`], [`parse()`],
//! [`vocabulary()`] and [`library()`].

mod feedback;
mod forms;
mod outcome;
mod program;
mod vocab;

pub use feedback::{
    Asked, ClaimPart, ClassPart, Core, ExpectPart, Fault, Feedback, Finding, Flagged, KernelId,
    Owed, Owing, Place, ProgramId, QueryPart, Readings, SCHEMA, Site, StatePart, Stop, Word, Work,
};
pub use forms::{Form, Forms, Library, Shape, Written, forms, shapes};
pub use outcome::{Outcome, Reading};
pub use vocab::{Imported, Vocabulary, library, vocabulary};

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;
use std::time::{Duration, Instant};

use ein_infer::explore::{self, Ask, Mode, Which};
use serde::{Deserialize, Serialize};

/// The default facts budget of a question, `--region-budget`'s default:
/// 2²⁰ facts beyond root, summed over every fixpoint the run computes.
pub const DEFAULT_BUDGET: u64 = ein_infer::downset::DEFAULT_BUDGET;

/// The default limit of the cube, `--region-limit`'s default: the largest
/// count of candidates whose whole cube a program the static check flags
/// gets.
pub const DEFAULT_LIMIT: u32 = ein_infer::region::DEFAULT_LIMIT;

/// The default cap on the firings of one saturation, `--max-steps`' default.
pub const DEFAULT_MAX_STEPS: u64 = explore::DEFAULT_MAX_STEPS;

/// A part of R, as `ein solve` names it: all of it, or one of its two lines.
/// *One* and *several* are counted over a part, and the part is the
/// **reading** an outcome is taken under.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Part {
    /// Every state of R.
    All,
    /// The minimal states of R.
    Minimal,
    /// The maximal states of R.
    Maximal,
}

impl Part {
    pub const ALL: [Part; 3] = [Part::All, Part::Minimal, Part::Maximal];

    /// The reading, in words.
    pub fn words(self) -> &'static str {
        match self {
            Part::All => "over all of R",
            Part::Minimal => "over the minimal line",
            Part::Maximal => "over the maximal line",
        }
    }

    fn which(self) -> Which {
        match self {
            Part::All => Which::All,
            Part::Minimal => Which::Minimal,
            Part::Maximal => Which::Maximal,
        }
    }

    fn flag(self) -> &'static str {
        match self {
            Part::All => "",
            Part::Minimal => "--minimal",
            Part::Maximal => "--maximal",
        }
    }
}

/// A question about R, as `ein solve`'s flags ask it. The outcome is of a
/// question: each asks the kernel for different work and is decided by
/// different values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Question {
    /// `ein solve FILE`: describe all of R.
    Describe,
    /// `--single`, `--minimal --single`, `--maximal --single`: is the part
    /// one state?
    Single(Part),
    /// `--minimal`, `--maximal`: the line, every state of it.
    Line(Part),
    /// `--count`: R's two counts, by the description's route.
    Count,
    /// `--no-search`: root alone; R is not searched.
    NoSearch,
}

impl Question {
    /// Every question EinAf asks, in this order.
    pub const ALL: [Question; 8] = [
        Question::Describe,
        Question::Single(Part::All),
        Question::Single(Part::Minimal),
        Question::Single(Part::Maximal),
        Question::Line(Part::Minimal),
        Question::Line(Part::Maximal),
        Question::Count,
        Question::NoSearch,
    ];

    /// The question as `ein solve`'s flags, `describe` for the bare run.
    pub fn flags(self) -> String {
        match self {
            Question::Describe => "describe".to_string(),
            Question::Single(p) => [p.flag(), "--single"]
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" "),
            Question::Line(Part::All) => "describe".to_string(),
            Question::Line(p) => p.flag().to_string(),
            Question::Count => "--count".to_string(),
            Question::NoSearch => "--no-search".to_string(),
        }
    }

    /// The flags `ein solve` takes for this question, one an argument; none
    /// for a description.
    pub fn argv(self) -> Vec<&'static str> {
        match self {
            Question::Describe | Question::Line(Part::All) => Vec::new(),
            Question::Single(p) => [p.flag(), "--single"]
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect(),
            Question::Line(p) => vec![p.flag()],
            Question::Count => vec!["--count"],
            Question::NoSearch => vec!["--no-search"],
        }
    }

    /// **The readings this question's answer can be taken under**: the parts
    /// of R over which *one* and *several* can be counted from it. A
    /// description lists every state of R with its lines, so it gives all
    /// three; `--single` and a line give the part they ask about; `--count`
    /// counts all of R; `--no-search` counts nothing, and is read over all of
    /// R.
    pub fn readings(self) -> Vec<Part> {
        match self {
            Question::Describe => Part::ALL.to_vec(),
            Question::Single(p) | Question::Line(p) => vec![p],
            Question::Count | Question::NoSearch => vec![Part::All],
        }
    }

    /// **Whether the question needs the cube on a program the static check
    /// flags**: every question but `--single` over all of R or its minimal
    /// line, which stop when decided, and `--no-search`, which reads root
    /// alone. A description, `--count`, a line and `--maximal --single` read
    /// every node of a flagged program's cube up to the limit.
    pub fn needs_the_cube(self) -> bool {
        !matches!(
            self,
            Question::Single(Part::All | Part::Minimal) | Question::NoSearch
        )
    }

    fn mode(self) -> Mode {
        match self {
            Question::Describe | Question::NoSearch => Mode {
                which: Which::All,
                ask: Ask::Describe,
            },
            Question::Single(p) => Mode {
                which: p.which(),
                ask: Ask::Single,
            },
            Question::Line(p) => Mode {
                which: p.which(),
                ask: Ask::Describe,
            },
            Question::Count => Mode {
                which: Which::All,
                ask: Ask::Count,
            },
        }
    }
}

/// What may stop a call before it decides its question: `--region-budget`,
/// `--region-limit`, `--max-steps` and `-T`. None changes an answer; a run a
/// bound stops is *not decided*. All four are set on every call, and each is
/// a key of the object's `asked`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bounds {
    /// Facts beyond root, summed over every fixpoint the run computes.
    pub budget: u64,
    /// The largest count of candidates whose whole cube a program the static
    /// check flags gets; a monotone program never reads it.
    pub limit: u32,
    /// The firings one saturation may take, root's and the judgement's
    /// included; past it the run is *not decided*. `None` is no cap.
    pub max_steps: Option<u64>,
    /// The wall clock: the load's judgement from the moment the call starts,
    /// and each question from the moment it is asked. `None` is no clock.
    pub clock: Option<Duration>,
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds {
            budget: DEFAULT_BUDGET,
            limit: DEFAULT_LIMIT,
            max_steps: Some(DEFAULT_MAX_STEPS),
            clock: None,
        }
    }
}

/// A program handed to the kernel.
#[derive(Clone, Copy, Debug)]
pub enum Source<'a> {
    /// A file, read as `ein solve` reads it, its local imports found from
    /// its directory and its messages naming it as given.
    File(&'a Path),
    /// A text: `name` is what its messages name it by, and `base_dir` where
    /// its local imports are found.
    Text {
        text: &'a str,
        name: &'a str,
        base_dir: Option<&'a Path>,
    },
}

/// **The bounded call**: load `source` once, ask it each of `questions` on a
/// copy of the load, within `bounds`, and return one [`Feedback`] for each,
/// in their order.
///
/// It returns whatever the program is: a refused program is an object whose
/// outcome is *invalid* under every question; a run a bound stopped, *not
/// decided*; a panic of the kernel's, at the load or in a question, is the
/// object's `fault` with no outcome, and the other questions are still
/// asked.
pub fn call(source: Source<'_>, questions: &[Question], bounds: Bounds) -> Vec<Feedback> {
    let kernel = kernel_id();
    let t0 = Instant::now();
    let name = match source {
        Source::File(path) => path.display().to_string(),
        Source::Text { name, .. } => name.to_string(),
    };
    let loaded = catch_unwind(AssertUnwindSafe(|| match source {
        Source::File(path) => program::load_file(path, bounds, t0),
        Source::Text {
            text,
            name,
            base_dir,
        } => program::load_text(text, name, base_dir, bounds, t0),
    }));
    let load_us = t0.elapsed().as_micros() as u64;
    let loaded = match loaded {
        Ok(l) => l,
        Err(payload) => {
            let identity = program::Identity {
                digest: None,
                of: None,
            };
            let shared = feedback::Shared {
                identity: &identity,
                name: &name,
                kernel: &kernel,
                bounds,
                load_us,
                notes: &[],
            };
            let message = panic_message(payload.as_ref());
            return questions
                .iter()
                .map(|&q| feedback::faulted(&shared, q, 0, "panic", message.clone()))
                .collect();
        }
    };
    let notes: &[ein_core::Diagnostic] = match &loaded.program {
        Ok(p) => &p.notes,
        Err(_) => &[],
    };
    let shared = feedback::Shared {
        identity: &loaded.identity,
        name: &loaded.name,
        kernel: &kernel,
        bounds,
        load_us,
        notes,
    };
    questions
        .iter()
        .map(|&question| {
            let t = Instant::now();
            let asked = catch_unwind(AssertUnwindSafe(|| {
                let run = match &loaded.program {
                    Ok(p) => p.ask(question, bounds)?,
                    Err(r) => program::Run::refused(question, r.clone()),
                };
                Ok::<_, program::EngineError>(feedback::build(
                    &shared,
                    &run,
                    t.elapsed().as_micros() as u64,
                ))
            }));
            let ask_us = t.elapsed().as_micros() as u64;
            match asked {
                Ok(Ok(mut f)) => {
                    f.work.ask_us = ask_us;
                    f
                }
                Ok(Err(program::EngineError(m))) => {
                    feedback::faulted(&shared, question, ask_us, "engine", m)
                }
                Err(payload) => feedback::faulted(
                    &shared,
                    question,
                    ask_us,
                    "panic",
                    panic_message(payload.as_ref()),
                ),
            }
        })
        .collect()
}

/// **What the parser says of a text**, loading nothing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parsed {
    /// The count of top-level forms, where the text parses.
    pub forms: Option<usize>,
    /// Every error the parser reports, each a finding of its kind with its
    /// place, its notes and repairs in its message; `[]` where it parses.
    pub findings: Vec<Finding>,
    /// The parser's recovery stopped at its budget: errors past the last one
    /// listed are not reported.
    pub cut: bool,
}

/// **Parse `text` as Ein's syntax**, `ein_ir::parse`, and load nothing:
/// `name` is what its messages name it by. A panic of the parser's is a
/// finding of the kind `parse` saying so.
pub fn parse(text: &str, name: &str) -> Parsed {
    let parsed = catch_unwind(AssertUnwindSafe(|| {
        let mut ast = ein_ir::Ast::new();
        match ein_ir::parse(&mut ast, text, Some(name)) {
            Ok(forms) => Parsed {
                forms: Some(forms.len()),
                findings: Vec::new(),
                cut: false,
            },
            Err(e) => Parsed {
                forms: None,
                findings: e.diagnostics().iter().map(feedback::finding).collect(),
                cut: e.cut,
            },
        }
    }));
    parsed.unwrap_or_else(|payload| Parsed {
        forms: None,
        findings: vec![Finding {
            kind: ein_core::Kind::Parse.name().to_string(),
            severity: "error".to_string(),
            message: format!("the parser panicked: {}", panic_message(payload.as_ref())),
            place: None,
        }],
        cut: false,
    })
}

/// A panic's message, where its payload is text.
fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "a panic with no message".to_string()
    }
}

/// **Which kernel this is**: the three things `ein --version` prints that a
/// record needs, each taken as the binary takes it. The version is the
/// engine's workspace version, read from its manifest when EinAf is built:
/// no crate EinAf links says it. The commit is the record's to set.
fn kernel_id() -> KernelId {
    let manifest = include_str!("../../../ein.rs/Cargo.toml");
    let version = manifest
        .split("[workspace.package]")
        .nth(1)
        .and_then(|rest| {
            rest.lines()
                .take_while(|l| !l.starts_with('['))
                .find_map(|l| {
                    let l = l.trim();
                    let v = l.strip_prefix("version")?.trim().strip_prefix('=')?;
                    Some(v.trim().trim_matches('"').to_string())
                })
        })
        .unwrap_or_default();
    let stdlib = ein_ir::stdlib::resolve_default()
        .read(ein_ir::stdlib::MARKER)
        .map(|text| program::sha256(text.as_bytes()));
    KernelId {
        version,
        protocol: ein_infer::events::SCHEMA.to_string(),
        stdlib,
        commit: None,
    }
}
