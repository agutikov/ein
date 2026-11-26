//! **A program as one load left it, and a question asked of a copy**: the
//! library's calls, made as `ein solve` makes them. Private to
//! [`crate::kernel`]: every call reaches them through the bounded call,
//! [`super::call`], which sets the bounds and catches a panic.

use std::path::Path;
use std::time::Instant;

use ein_core::{Diagnostic, Kb, Kind, Terms};
use ein_infer::explore;
use ein_infer::read_out::ReadOut;
use ein_ir::Ast;
use sha2::{Digest, Sha256};

use super::{Bounds, Question};

/// Where a program was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Stage {
    /// The file could not be read as text.
    Read,
    /// The parse.
    Parse,
    /// The load, the judgement of the hypothesis mechanisms among it.
    Load,
    /// The run, at a firing whose conclusion no fact can hold.
    Run,
}

impl Stage {
    pub(crate) fn words(self) -> &'static str {
        match self {
            Stage::Read => "at the read",
            Stage::Parse => "at the parse",
            Stage::Load => "at the load",
            Stage::Run => "at the run",
        }
    }
}

/// **A program the kernel refused**: its findings, as the kernel's values,
/// and the message it refused it with, as `ein solve` prints it on stderr.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Refusal {
    pub(crate) stage: Stage,
    pub(crate) message: String,
    /// The refusal's findings, each with its kind, the refusal of Ein's
    /// reference it is, its place where the line names one, and its line.
    /// Empty only for a run's error the kernel names no kind for.
    pub(crate) findings: Vec<Diagnostic>,
}

impl Refusal {
    /// The kinds of its findings, in order, each once.
    pub(crate) fn kinds(&self) -> Vec<Kind> {
        let mut out: Vec<Kind> = Vec::new();
        for d in &self.findings {
            if !out.contains(&d.kind) {
                out.push(d.kind);
            }
        }
        out
    }
}

/// A failure of the engine that is none of the outcomes: a question the
/// engine says it cannot answer, or a check of its own that failed. No
/// program should reach one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EngineError(pub(crate) String);

/// **What identifies a program**: the SHA-256 of what it is, and over what.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Identity {
    /// `sha256:` and the hex digest; `None` where nothing was read.
    pub(crate) digest: Option<String>,
    /// `canonical-dump` (the program parsed, its forms as
    /// `ein_ir::dump_canonical` prints them), `text` (a text that did not
    /// parse, its bytes) or `bytes` (a file EinAf does not read as text).
    pub(crate) of: Option<&'static str>,
}

/// SHA-256 of `bytes`, as `sha256:` and the hex digest.
pub(crate) fn sha256(bytes: &[u8]) -> String {
    let mut s = String::from("sha256:");
    for b in Sha256::digest(bytes) {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

/// A program as one load left it. Every question is asked of a copy, so the
/// load is made once.
#[derive(Clone)]
pub(crate) struct Program {
    pub(crate) ast: Ast,
    pub(crate) terms: Terms,
    pub(crate) kb: Kb,
    /// What the judgement of the mechanisms noted and did not refuse.
    pub(crate) notes: Vec<Diagnostic>,
}

/// A load's result, and what identifies the program whatever it was.
pub(crate) struct Loaded {
    pub(crate) program: Result<Program, Refusal>,
    pub(crate) identity: Identity,
    /// The name the program's messages give it.
    pub(crate) name: String,
}

/// Read and load the program at `path`, as `ein solve` does: a module the
/// program imports by a local name is found from the file's directory.
pub(crate) fn load_file(path: &Path, bounds: Bounds, t0: Instant) -> Loaded {
    let name = path.display().to_string();
    let text = match ein_ir::read::read_program(path) {
        Ok(ein_ir::read::Held::Text(text)) => text,
        Ok(ein_ir::read::Held::Container(bytes)) => {
            let message = format!("{name} is a .einb container: EinAf reads a program as text");
            return Loaded {
                program: Err(Refusal {
                    stage: Stage::Read,
                    findings: vec![Diagnostic::error(Kind::Unreadable, message.clone())],
                    message,
                }),
                identity: Identity {
                    digest: Some(sha256(&bytes)),
                    of: Some("bytes"),
                },
                name,
            };
        }
        Err(e) => {
            return Loaded {
                program: Err(Refusal {
                    stage: Stage::Read,
                    findings: vec![e.diagnostic()],
                    message: e.line,
                }),
                identity: Identity {
                    digest: None,
                    of: None,
                },
                name,
            };
        }
    };
    load_text(&text, &name, path.parent(), bounds, t0)
}

/// Load a program from its text: `name` is what a message names it by, and
/// `base_dir` where its local imports are found. The judgement of the
/// hypothesis mechanisms, which saturates root, runs under `bounds`, its
/// clock from `t0`: a judgement the bound stops judges nothing, and the
/// question asked next reports the stop.
pub(crate) fn load_text(
    text: &str,
    name: &str,
    base_dir: Option<&Path>,
    bounds: Bounds,
    t0: Instant,
) -> Loaded {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let loaded = ein_ir::saved::load_text(
        &mut ast,
        &mut terms,
        text,
        Some(name),
        base_dir,
        0,
        &ein_ir::LoadOptions::default(),
    );
    let loaded = match loaded {
        Ok(l) => l,
        Err(e) => {
            let identity = match &e {
                // A text that did not parse has no forms: its bytes.
                ein_ir::saved::TextError::Parse(_) => Identity {
                    digest: Some(sha256(text.as_bytes())),
                    of: Some("text"),
                },
                // A text that parsed and did not load: its forms, parsed
                // again into an arena of their own.
                ein_ir::saved::TextError::Load(_) => {
                    let mut again = Ast::new();
                    match ein_ir::parse(&mut again, text, Some(name)) {
                        Ok(forms) => Identity {
                            digest: Some(sha256(ein_ir::dump_canonical(&again, &forms).as_bytes())),
                            of: Some("canonical-dump"),
                        },
                        Err(_) => Identity {
                            digest: Some(sha256(text.as_bytes())),
                            of: Some("text"),
                        },
                    }
                }
            };
            return Loaded {
                program: Err(refusal_of_load(e)),
                identity,
                name: name.to_string(),
            };
        }
    };
    let identity = Identity {
        digest: Some(sha256(
            ein_ir::dump_canonical(&ast, &loaded.forms).as_bytes(),
        )),
        of: Some("canonical-dump"),
    };
    let mut program = Program {
        ast,
        terms,
        kb: loaded.kb,
        notes: Vec::new(),
    };
    let bound = ein_infer::saturator::Bound {
        deadline: bounds.clock.map(|c| t0 + c),
        max_steps: bounds.max_steps,
    };
    match ein_infer::saturator::bounded(bound, || program.judge_mechanisms()) {
        Ok(notes) => program.notes = notes,
        Err(refusal) => {
            return Loaded {
                program: Err(refusal),
                identity,
                name: name.to_string(),
            };
        }
    }
    Loaded {
        program: Ok(program),
        identity,
        name: name.to_string(),
    }
}

/// **What a refused load says**, from the load's error, as values: its
/// findings, each with its kind and its place.
fn refusal_of_load(e: ein_ir::saved::TextError) -> Refusal {
    let findings = e.findings();
    match e {
        ein_ir::saved::TextError::Parse(p) => Refusal {
            stage: Stage::Parse,
            message: p.to_string(),
            findings,
        },
        ein_ir::saved::TextError::Load(k) => Refusal {
            stage: Stage::Load,
            message: format!("kb load error: {k}"),
            findings,
        },
    }
}

impl Program {
    /// **The hypothesis mechanisms judged on root's fixpoint**, the step
    /// after the load every command of Ein takes,
    /// `ein_infer::mechanisms::judge_loaded`, on a copy of the load: its
    /// conflicts refuse the program, each a finding of the kind
    /// `mechanisms`, and its notes are kept.
    fn judge_mechanisms(&self) -> Result<Vec<Diagnostic>, Refusal> {
        ein_infer::mechanisms::judge_loaded(&self.ast, &self.terms, &self.kb, |ast, terms| {
            *ast = self.ast.clone();
            *terms = self.terms.clone();
            Some(self.kb.clone())
        })
        .map_err(|conflicts| Refusal {
            stage: Stage::Load,
            message: conflicts
                .iter()
                .map(|d| format!("kb load error: {d}"))
                .collect::<Vec<_>>()
                .join("\n"),
            findings: conflicts,
        })
    }

    /// Ask `question` of a copy of this program, within `bounds`, the clock
    /// from the moment it is asked.
    ///
    /// **A question that needs the cube is asked under a limit of 0**
    /// ([`Question::needs_the_cube`]): the limit is read by a program the
    /// static check flags and by no other, so on every other program the run
    /// is the one asked, and on a flagged one the run reads the static check
    /// at root and stops at once, where the limit asked would have it compute
    /// a cube of up to 2¹⁶ nodes whose answer nothing claims.
    pub(crate) fn ask(&self, question: Question, bounds: Bounds) -> Result<Run, EngineError> {
        let mut copy = self.clone();
        let mut opts = explore::Options::new(question.mode());
        opts.budget = bounds.budget;
        opts.limit = if question.needs_the_cube() {
            0
        } else {
            bounds.limit
        };
        opts.no_search = question == Question::NoSearch;
        // The program's own `(config :lattice-sanity-check true)`, which
        // `ein solve` reads as `-y`.
        opts.sanity = copy
            .kb
            .program()
            .config
            .as_ref()
            .is_some_and(|c| c.lattice_sanity_check);
        opts.max_steps = bounds.max_steps;
        opts.deadline = bounds.clock.map(|c| Instant::now() + c);
        let answered =
            match ein_infer::read_out::run(&mut copy.kb, &mut copy.terms, &copy.ast, &opts) {
                Ok(r) => Answered::Read(Box::new(r)),
                Err(e) => answered_of(e)?,
            };
        let cube_skipped = question.needs_the_cube()
            && matches!(&answered, Answered::Read(r) if !r.answer.monotonicity.is_monotone());
        Ok(Run {
            question,
            answered,
            cube_skipped,
            program: Some(Box::new(copy)),
        })
    }
}

impl Program {
    /// **The kernel's trace of the answer to `question`**, as `ein solve
    /// --trace FILE.md --no-diagrams` writes it: root's derivation, then each
    /// state the read-out lists, derived from its first minimal commitment,
    /// each step a firing with its rule's `:why` filled. Asked of a copy,
    /// under `bounds` as [`Program::ask`] asks; a run that hands back no
    /// read-out is the line saying why.
    pub(crate) fn trace(
        &self,
        question: Question,
        bounds: Bounds,
        name: &str,
    ) -> Result<String, String> {
        let mut copy = self.clone();
        let mut opts = explore::Options::new(question.mode());
        opts.budget = bounds.budget;
        opts.limit = if question.needs_the_cube() {
            0
        } else {
            bounds.limit
        };
        opts.no_search = question == Question::NoSearch;
        opts.max_steps = bounds.max_steps;
        opts.deadline = bounds.clock.map(|c| Instant::now() + c);
        let bound = ein_infer::saturator::Bound {
            deadline: opts.deadline,
            max_steps: bounds.max_steps,
        };
        let firings = match ein_infer::saturator::bounded(bound, || {
            ein_infer::region::root_derivation(&mut copy.kb, &mut copy.terms, &copy.ast)
        }) {
            Ok(f) => f,
            // Root past the cap: the run says so, and the trace tells no
            // derivation, as `ein solve --trace` does.
            Err(ein_infer::SolveError::Saturate(ein_infer::saturator::SaturateError::Steps(_))) => {
                Vec::new()
            }
            Err(e) => return Err(e.to_string()),
        };
        let r = ein_infer::read_out::run(&mut copy.kb, &mut copy.terms, &copy.ast, &opts)
            .map_err(|e| e.to_string())?;
        let argv = match question {
            Question::Describe => String::new(),
            q => q.flags(),
        };
        ein_render::trace::region::of_read_out(
            &r,
            &mut copy.kb,
            &mut copy.terms,
            &copy.ast,
            &firings,
            &ein_render::read_out::Options {
                command: "solve",
                argv: &argv,
                file: name,
                budget: bounds.budget,
                uncut: false,
            },
            ein_render::trace::region::Shape {
                diagrams: false,
                relevant: false,
                reorder: false,
                full_kb: false,
                fold: false,
            },
        )
    }
}

/// A run's error, as one of the outcomes' inputs or as the engine's: read
/// by its kind, `SolveError::kind`, where the kernel names one, the clock's
/// (`deadline`) and a saved file's choice that names no node (`saved-file`,
/// the load's exit under `ein solve`); and by its variant where it names
/// none.
fn answered_of(e: ein_infer::SolveError) -> Result<Answered, EngineError> {
    use ein_infer::SaturateError as S;
    use ein_infer::SolveError as E;
    match e.kind() {
        Some(Kind::Deadline) => return Ok(Answered::Clock),
        Some(kind) => {
            let message = e.to_string();
            return Ok(Answered::Refused(Refusal {
                stage: Stage::Load,
                findings: vec![Diagnostic::error(kind, message.clone())],
                message,
            }));
        }
        None => {}
    }
    match e {
        // A conclusion no fact can hold, met at a firing, and a rule that
        // does not compile: the program refused at its run. The kernel names
        // no kind for either, which no program it loads should reach.
        E::Saturate(S::Fire(ein_infer::FireError::NotAFact(m))) => Ok(Answered::Refused(Refusal {
            stage: Stage::Run,
            message: m,
            findings: Vec::new(),
        })),
        E::Saturate(S::Compile(c)) | E::Compile(c) => Ok(Answered::Refused(Refusal {
            stage: Stage::Run,
            message: c.to_string(),
            findings: Vec::new(),
        })),
        other => Err(EngineError(other.to_string())),
    }
}

/// What the kernel handed back for one question.
#[derive(Debug)]
pub(crate) enum Answered {
    /// The program was refused.
    Refused(Refusal),
    /// The clock ran out inside the run, which hands back nothing.
    Clock,
    /// The read-out: the answer, its states and each query's classes.
    Read(Box<ReadOut>),
}

/// One question asked of one program, and what came back.
pub(crate) struct Run {
    pub(crate) question: Question,
    pub(crate) answered: Answered,
    /// A question that needs the cube, asked of a program that is not
    /// monotone: its run stopped at once, under a limit of 0, and what it
    /// established is not an answer to the question asked.
    pub(crate) cube_skipped: bool,
    /// The copy the question was asked of, as the run left it: its names
    /// render the answer's values. `None` for a refused program.
    pub(crate) program: Option<Box<Program>>,
}

impl Run {
    /// A question asked of a program the kernel refused: there is no answer,
    /// and every question's outcome is *invalid*.
    pub(crate) fn refused(question: Question, refusal: Refusal) -> Run {
        Run {
            question,
            answered: Answered::Refused(refusal),
            cube_skipped: false,
            program: None,
        }
    }
}
