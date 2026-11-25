//! **The kernel as EinAf reads it**: the one module of EinAf that calls Ein's
//! library, `ein-ir` to load, `ein-infer` to ask, `ein-render` to print.
//!
//! A program is loaded once ([`Program::load_file`], [`Program::load_text`])
//! and asked any number of questions about its answer, the region R
//! ([`Program::ask`]), each on a copy of the load, as `ein solve` asks one
//! question of a fresh load. A load the kernel refuses is a [`Refusal`], and
//! [`Run::refused`] makes it a run like any other, so that every question has
//! an [`outcome`] whatever the program is.
//!
//! The load is the one `ein solve` makes: the text parsed and loaded against
//! the program file's directory, then the judgement of the hypothesis
//! mechanisms on a copy of root, which refuses what the binary refuses. The
//! question is `ein solve`'s ([`Question`]), at its default bounds unless told
//! otherwise ([`Bounds`]).

mod outcome;

pub use outcome::{Outcome, Reading, outcome};

use std::path::Path;
use std::time::{Duration, Instant};

use ein_core::{Kb, Terms};
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::read_out::ReadOut;
use ein_ir::Ast;

/// The default facts budget of a question, `--region-budget`'s default:
/// 2²⁰ facts beyond root, summed over every fixpoint the run computes.
pub const DEFAULT_BUDGET: u64 = ein_infer::downset::DEFAULT_BUDGET;

/// A part of R, as `ein solve` names it: all of it, or one of its two lines.
/// *One* and *several* are counted over a part, and the part is the
/// **reading** an outcome is taken under.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

/// The default limit of the cube, `--region-limit`'s default: the largest
/// count of candidates whose whole cube a program the static check flags
/// gets.
pub const DEFAULT_LIMIT: u32 = ein_infer::region::DEFAULT_LIMIT;

/// The default cap on the firings of one saturation, `--max-steps`' default.
pub const DEFAULT_MAX_STEPS: u64 = explore::DEFAULT_MAX_STEPS;

/// What may stop a run before it decides its question: `--region-budget`,
/// `--region-limit`, `--max-steps` and `-T`. None changes an answer; a run a bound stops is
/// *not decided*.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bounds {
    /// Facts beyond root, summed over every fixpoint the run computes.
    pub budget: u64,
    /// The largest count of candidates whose whole cube a program the static
    /// check flags gets; a monotone program never reads it.
    pub limit: u32,
    /// The firings one saturation may take, root's included; past it the run
    /// is *not decided*. `None` is no cap.
    pub max_steps: Option<u64>,
    /// The wall clock, from the moment the question is asked.
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

/// Where a program was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Stage {
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
    pub fn words(self) -> &'static str {
        match self {
            Stage::Read => "at the read",
            Stage::Parse => "at the parse",
            Stage::Load => "at the load",
            Stage::Run => "at the run",
        }
    }
}

/// **A program the kernel refused**, with the message it refused it with,
/// as `ein solve` prints it on stderr.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub stage: Stage,
    pub message: String,
    /// The parse error as the parser's value, every error with its place,
    /// notes and repairs, where the refusal was the parse's.
    pub parse: Option<ein_ir::ParseError>,
}

/// A failure of the engine that is none of the outcomes: a question the
/// engine says it cannot answer, or a check of its own that failed. No
/// program should reach one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EngineError(pub String);

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// A program as one load left it. Every question is asked of a copy, so the
/// load is made once.
#[derive(Clone)]
pub struct Program {
    ast: Ast,
    terms: Terms,
    kb: Kb,
}

impl Program {
    /// Read and load the program at `path`, as `ein solve` does: a module
    /// the program imports by a local name is found from the file's
    /// directory.
    pub fn load_file(path: &Path) -> Result<Program, Refusal> {
        let text = match ein_ir::read::read_program(path) {
            Ok(ein_ir::read::Held::Text(text)) => text,
            Ok(ein_ir::read::Held::Container(_)) => {
                return Err(Refusal {
                    stage: Stage::Read,
                    message: format!(
                        "{} is a .einb container: EinAf reads a program as text",
                        path.display()
                    ),
                    parse: None,
                });
            }
            Err(e) => {
                return Err(Refusal {
                    stage: Stage::Read,
                    message: e.line,
                    parse: None,
                });
            }
        };
        let name = path.display().to_string();
        Program::load_text(&text, &name, path.parent())
    }

    /// Load a program from its text: `name` is what a message names it by,
    /// and `base_dir` where its local imports are found.
    pub fn load_text(text: &str, name: &str, base_dir: Option<&Path>) -> Result<Program, Refusal> {
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
        )
        .map_err(|e| refusal_of_load(e, text, name))?;
        let program = Program {
            ast,
            terms,
            kb: loaded.kb,
        };
        program.judge_mechanisms()?;
        Ok(program)
    }

    /// **The hypothesis mechanisms judged on root's fixpoint**, which `ein
    /// solve` does after the load and refuses a conflict of at the load's
    /// exit: the library's load does not, so EinAf asks for it as the binary
    /// does, on a copy of root.
    fn judge_mechanisms(&self) -> Result<(), Refusal> {
        if !ein_infer::mechanisms::may_conflict(&self.kb, &self.terms, &self.ast) {
            return Ok(());
        }
        let mut copy = self.clone();
        let Some(report) = ein_infer::mechanisms::check(&copy.ast, &mut copy.terms, &mut copy.kb)
        else {
            return Ok(());
        };
        if report.conflicts.is_empty() {
            return Ok(());
        }
        Err(Refusal {
            stage: Stage::Load,
            message: report
                .conflicts
                .iter()
                .map(|l| format!("kb load error: {l}"))
                .collect::<Vec<_>>()
                .join("\n"),
            parse: None,
        })
    }

    /// Ask `question` of a copy of this program, within `bounds`.
    pub fn ask(&self, question: Question, bounds: Bounds) -> Result<Run, EngineError> {
        let mut copy = self.clone();
        let mut opts = explore::Options::new(question.mode());
        opts.budget = bounds.budget;
        opts.limit = bounds.limit;
        opts.no_search = question == Question::NoSearch;
        opts.max_steps = bounds.max_steps;
        opts.deadline = bounds.clock.map(|c| Instant::now() + c);
        let answered =
            match ein_infer::read_out::run(&mut copy.kb, &mut copy.terms, &copy.ast, &opts) {
                Ok(r) => Answered::Read(Box::new(r)),
                Err(e) => answered_of(e)?,
            };
        Ok(Run { question, answered })
    }

    /// **Whether the engine's static check flags the program as not
    /// monotone**: `ein solve --monotone`, root's fixpoint and the check, on
    /// a copy. A run can find a program not monotone too, in flight; this is
    /// the check alone.
    pub fn statically_flagged(&self) -> Result<bool, EngineError> {
        let mut copy = self.clone();
        let checked = ein_infer::explore::check_monotone(
            &mut copy.kb,
            &mut copy.terms,
            &copy.ast,
            &mut ein_infer::Events::off(),
        )
        .map_err(|e| EngineError(e.to_string()))?;
        Ok(!checked.monotonicity.is_monotone())
    }

    /// The claims the program states, by their shapes.
    pub fn claims(&self) -> Claims {
        let program = self.kb.program();
        let solve = program.solve().and_then(|s| {
            let node = ein_infer::solve_value(&self.ast, s, "expect")?;
            Some(match ein_ir::claim::parse(&self.ast, node) {
                Ok(ein_ir::claim::Claim::False) => SolveClaim::False,
                Ok(ein_ir::claim::Claim::Region(r)) => SolveClaim::Region {
                    states: r.models.len(),
                    owing: !r.owing.is_empty(),
                },
                Err(_) => SolveClaim::Unread,
            })
        });
        // The queries a run answers, which are those with a goal, in the
        // order the program writes them: what `ReadOut::queries` holds.
        let queries = program
            .queries
            .iter()
            .filter(|q| ein_infer::query_value(&self.ast, q, "goal").is_some())
            .map(|q| {
                let expect = ein_infer::query_value(&self.ast, q, "expect")?;
                let over = ein_infer::query_value(&self.ast, q, "over")
                    .and_then(|n| ein_ir::expect::Over::parse(&self.ast, n).ok())
                    .unwrap_or_default();
                Some(QueryClaim {
                    over: match over {
                        ein_ir::expect::Over::All => Part::All,
                        ein_ir::expect::Over::Minimal => Part::Minimal,
                        ein_ir::expect::Over::Maximal => Part::Maximal,
                    },
                    classes: ein_ir::expect::parse(&self.ast, expect).models().len(),
                })
            })
            .collect();
        Claims { solve, queries }
    }
}

/// **What a refused load says**, from the load's error: the parse's value
/// where the parse refused, and otherwise the load's message, which the
/// library hands back as text. The one place EinAf reads a refusal; the
/// kernel's diagnostics as values replace what it reads here.
fn refusal_of_load(e: ein_ir::saved::TextError, text: &str, name: &str) -> Refusal {
    match e {
        ein_ir::saved::TextError::Parse(message) => Refusal {
            stage: Stage::Parse,
            // The load hands the parse's error back as its text: the parser
            // is asked again for its value, which the text was printed from.
            parse: ein_ir::parse(&mut Ast::new(), text, Some(name)).err(),
            message,
        },
        load @ ein_ir::saved::TextError::Load(_) => Refusal {
            stage: Stage::Load,
            message: load.to_string(),
            parse: None,
        },
    }
}

/// A run's error, as one of the outcomes' inputs or as the engine's.
fn answered_of(e: ein_infer::SolveError) -> Result<Answered, EngineError> {
    use ein_infer::SaturateError as S;
    use ein_infer::SolveError as E;
    match e {
        E::Deadline => Ok(Answered::Clock),
        // A conclusion no fact can hold, met at a firing, and a rule that
        // does not compile: the program refused at its run.
        E::Saturate(S::Fire(ein_infer::FireError::NotAFact(m))) => Ok(Answered::Refused(Refusal {
            stage: Stage::Run,
            message: m,
            parse: None,
        })),
        E::Saturate(S::Compile(c)) | E::Compile(c) => Ok(Answered::Refused(Refusal {
            stage: Stage::Run,
            message: c.to_string(),
            parse: None,
        })),
        // A saved file's commitment that names no node of its lattice: the
        // load's exit under `ein solve`.
        E::Refused(m) => Ok(Answered::Refused(Refusal {
            stage: Stage::Load,
            message: m,
            parse: None,
        })),
        other => Err(EngineError(other.to_string())),
    }
}

/// The claims a program states, by their shapes: what each says of R.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Claims {
    /// The `(solve … :expect …)`.
    pub solve: Option<SolveClaim>,
    /// Each query's `:expect`, one entry a query that has a goal, in the
    /// order the program writes them, `None` for a query that states none.
    pub queries: Vec<Option<QueryClaim>>,
}

/// What the kernel's own check of a claim found on a run, `:expect`'s three
/// outcomes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Checked {
    /// The claim holds, on a run that exhausted what it is about.
    Held,
    /// The claim is false.
    Failed,
    /// The run did not decide it.
    NotChecked,
}

impl Checked {
    fn of(report: &ein_infer::Report) -> Checked {
        match report.outcome {
            ein_infer::Outcome::Held => Checked::Held,
            ein_infer::Outcome::Failed => Checked::Failed,
            ein_infer::Outcome::NotChecked => Checked::NotChecked,
        }
    }
}

/// The kernel's check of each claim a program states, on one run: aligned
/// with [`Claims`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ClaimsChecked {
    pub solve: Option<Checked>,
    pub queries: Vec<Option<Checked>>,
}

/// A `(solve … :expect …)`, by its shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolveClaim {
    /// `(false)`: every node is dead.
    False,
    /// `(region …)`: R's states one to one, or, with `(owing …)`, R empty
    /// with these alive nodes owing.
    Region { states: usize, owing: bool },
    /// A claim the library could not read.
    Unread,
}

/// A query's `:expect`, by its shape: the part of R its classes are over,
/// and how many it lists, `(false)` listing none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QueryClaim {
    pub over: Part,
    pub classes: usize,
}

/// What the kernel handed back for one question.
#[derive(Debug)]
pub enum Answered {
    /// The program was refused.
    Refused(Refusal),
    /// The clock ran out inside the run, which hands back nothing.
    Clock,
    /// The read-out: the answer, its states and each query's classes.
    Read(Box<ReadOut>),
}

/// One question asked of one program, and what came back.
#[derive(Debug)]
pub struct Run {
    pub question: Question,
    pub answered: Answered,
}

impl Run {
    /// A question asked of a program the kernel refused: there is no answer,
    /// and every question's outcome is *invalid*.
    pub fn refused(question: Question, refusal: Refusal) -> Run {
        Run {
            question,
            answered: Answered::Refused(refusal),
        }
    }

    /// The kernel's own check of each claim the program states, on this run;
    /// `None` where the run has no read-out.
    pub fn claims_checked(&self) -> Option<ClaimsChecked> {
        let Answered::Read(r) = &self.answered else {
            return None;
        };
        Some(ClaimsChecked {
            solve: r.claim.as_ref().map(Checked::of),
            queries: r
                .queries
                .iter()
                .map(|q| q.claim.as_ref().map(Checked::of))
                .collect(),
        })
    }

    /// The program's file and run, read in full: `ein solve`'s read-out as
    /// it prints it to stdout, for a person reading beside the outcome.
    pub fn read_out_text(&self, file: &str, budget: u64) -> Option<String> {
        let Answered::Read(r) = &self.answered else {
            return None;
        };
        let argv = match self.question {
            Question::Describe => String::new(),
            q => q.flags(),
        };
        let text = ein_render::read_out::render(
            r,
            &ein_render::read_out::Options {
                command: "solve",
                argv: &argv,
                file,
                budget,
                uncut: false,
            },
        );
        Some(text.stdout)
    }
}
