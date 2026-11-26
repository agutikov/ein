//! **The feedback object**, `einaf-feedback/1`: what the kernel found of one
//! program under one question, as one value, built from the library's values
//! by [`build`] and nothing else, and written as JSON for the record.
//!
//! **Every key on every object, `null` where it does not apply**, as the
//! engine's summary keeps its keys
//! ([Q-M1e2.27](../../../docs/decisions/read-out/Q-M1e2.27-in-eins-json-is-the-empty-set-and-null-what-does.md)):
//! `[]` is a list known to be empty, `null` a part that does not apply to
//! this question or this outcome, or that the run did not establish. So a
//! renderer reads a part by its key and renders nothing for `null`.
//!
//! [The feedback object](../../../docs/einaf/03-feedback.md) is this
//! module's page: the object key by key.

use std::collections::BTreeMap;

use ein_infer::explore::{Ask, Stopped, Which};
use ein_infer::read_out::{Listing, ReadOut};
use serde::{Deserialize, Serialize};

use super::outcome::{self, Reading};
use super::program::{Answered, Identity, Program, Run, sha256};
use super::{Bounds, Part, Question};

/// The object's schema: its number moves when a part changes its meaning.
pub const SCHEMA: &str = "einaf-feedback/1";

/// **What the kernel found of one program under one question.** The
/// parts are the work order's: what was asked, the outcome under each
/// reading, the readings of *one answer* the run established, the
/// diagnostics, the static check's sites, the queries and the claim, the
/// states, what is owed, the core, the work, and two lists left open; and a
/// fault of the kernel's, which is no outcome of the program.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Feedback {
    /// [`SCHEMA`].
    pub schema: String,
    pub program: ProgramId,
    pub kernel: KernelId,
    pub asked: Asked,
    /// The outcome under each reading the question gives, in
    /// [`Question::readings`]' order; `null` where the kernel faulted.
    pub outcome: Option<Vec<Reading>>,
    /// What the run established of *one answer*, each reading the question
    /// asks for; `null` where no answer came back.
    pub readings: Option<Readings>,
    /// What the kernel said, in the order `ein solve` prints it on stderr:
    /// a refusal's findings, the clock's abort, the static check's warning,
    /// a partial run's note, each claim not checked or refuted, and the
    /// load's notes last.
    pub diagnostics: Vec<Finding>,
    /// The static check's sites, or the hypothesis found in flight, where the
    /// program is not monotone.
    pub flagged: Option<Flagged>,
    /// Each `(query …)` with a goal, answered; `null` where no answer came
    /// back.
    pub queries: Option<Vec<QueryPart>>,
    /// The program's `(solve … :expect …)`; `null` where it states none.
    pub claim: Option<ClaimPart>,
    /// The states the run reached; `null` where it read none.
    pub states: Option<Vec<StatePart>>,
    /// What root, or the frontier of an empty R, still owes.
    pub owed: Option<Owed>,
    /// A dead root's core, under a description only.
    pub core: Option<Core>,
    pub work: Work,
    /// **Left open**: what the program's facts depend on, the ladder's F6.
    /// The ladder's stage adds it; `null` on every object until then.
    pub dependencies: Option<Vec<serde_json::Value>>,
    /// **Left open**: how a fact was derived, the ladder's F7. `null` on
    /// every object until the ladder's stage adds it.
    pub provenance: Option<Vec<serde_json::Value>>,
    /// The kernel failed at this boundary: a panic, or an engine's error no
    /// program should reach. Never an outcome of the program: `outcome` is
    /// `null` beside it.
    pub fault: Option<Fault>,
}

/// What identifies the program.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgramId {
    /// The name its messages give it: the path as given, or a text's name.
    pub name: String,
    /// `sha256:` and the hex digest; `null` where nothing was read.
    pub digest: Option<String>,
    /// What the digest is of: `canonical-dump`, the program's forms as
    /// `ein_ir::dump_canonical` prints them; `text`, the bytes of a text
    /// that did not parse; `bytes`, a file not read as text.
    pub of: Option<String>,
}

/// Which kernel answered.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelId {
    /// Ein's version, as `ein --version`'s first line has it.
    pub version: String,
    /// The `--events` protocol, as `ein --version` names it.
    pub protocol: String,
    /// `sha256:` of the standard library's manifest as this process resolved
    /// it, as `ein --version` prints it.
    pub stdlib: Option<String>,
    /// The commit of Ein: `null` unless the caller that records the object
    /// sets it, from git.
    pub commit: Option<String>,
}

/// What was asked, and the bounds it was asked under.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asked {
    /// The question as `ein solve`'s flags, `describe` for the bare run.
    pub question: String,
    /// The part of R it is about: `all`, `minimal` or `maximal`.
    pub which: Part,
    /// `describe`, `single`, `count` or `no-search`.
    pub ask: String,
    /// `--region-budget`, in facts.
    pub budget: u64,
    /// `--region-limit`, in candidates.
    pub limit: u32,
    /// `--max-steps`, in firings; `null` for no cap.
    pub max_steps: Option<u64>,
    /// `-T`, in microseconds; `null` for no clock.
    pub clock_us: Option<u64>,
}

/// What the run established of *one answer*: each reading the question
/// asks for, and none it does not; a reading not asked is `null`, never
/// guessed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Readings {
    /// `--single`'s word over the part asked: `Solution`, `Ambiguity`,
    /// `Contradiction` or `Open`; `null` under any other question, and where
    /// the run stopped first.
    pub word: Option<Word>,
    /// A description's or `--count`'s count of R's states, exact where
    /// `exhausted`.
    pub k_s: Option<u64>,
    /// Their count of R's commitments.
    pub k_c: Option<u64>,
    /// The states of the minimal line: a description's, `--minimal`'s.
    pub minimal: Option<u64>,
    /// The states of the maximal line: a description's, `--maximal`'s.
    pub maximal: Option<u64>,
    /// Whether the part asked about was listed whole.
    pub exhausted: Option<bool>,
    /// What stopped the run before its question was decided.
    pub stopped: Option<Stop>,
}

/// `--single`'s word, and the part it is over.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Word {
    pub part: Part,
    pub word: String,
}

/// What stopped a run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stop {
    /// `budget`, `limit`, `steps`, `clock` or `asked` (`--no-search`).
    pub why: String,
    /// Under `steps`: `root` or `node`, the saturation that was stopped.
    pub saturation: Option<String>,
}

/// One diagnostic of the kernel's, as the summary's `diagnostics` writes
/// it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    /// The kind, by its name in Ein's reference.
    pub kind: String,
    /// `error`, `warning` or `note`.
    pub severity: String,
    /// The line, without its severity's word.
    pub message: String,
    pub place: Option<Place>,
}

/// Where a line names a position.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Place {
    pub file: String,
    pub line: u32,
    pub col: u32,
}

/// Why the program is not monotone.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Flagged {
    /// The static check's sites, as the summary's `monotonicity.sites`.
    pub sites: Vec<Site>,
    /// The hypothesis a state above root held that root does not, where the
    /// run found the program not monotone in flight.
    pub in_flight: Option<String>,
}

/// One site of the static check.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Site {
    pub rule: String,
    pub activator: Vec<String>,
    pub kind: String,
    pub relation: String,
    pub depth: u32,
    /// What the site's rule concludes, as the detector names it.
    pub concludes: Option<String>,
    /// What the site puts at stake.
    pub at_stake: Option<String>,
}

/// One `(query …)`, answered.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryPart {
    /// The goal, as written.
    pub goal: String,
    /// Its `:goal-text`, where it has one.
    pub goal_text: Option<String>,
    /// What its classes are over: the part its `:over` names, or `listed`,
    /// the states the run listed where it asked another.
    pub over: String,
    /// The classes are over a complete set, not a lower bound.
    pub exact: bool,
    /// Every state gives one answer: exactly one class.
    pub agree: bool,
    pub classes: Vec<ClassPart>,
    /// Its `:expect`; `null` where it states none.
    pub expect: Option<ExpectPart>,
}

/// One projection class of a query.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassPart {
    /// The goal's facts under its rows, sorted.
    pub facts: Vec<String>,
    /// Each binding row, a map from each goal variable to its value.
    pub rows: Vec<BTreeMap<String, String>>,
    /// Each row filled into the query's `:goal-text`, in the rows' order.
    pub lines: Vec<String>,
    /// The states that give it, 1-based into `states`.
    pub states: Vec<usize>,
}

/// A query's `:expect`: what it claims, and the kernel's check of it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectPart {
    /// How many classes the claim lists, `(false)` listing none.
    pub classes: usize,
    /// `held`, `failed` or `not-checked`; `null` where the run checked
    /// nothing.
    pub outcome: Option<String>,
    /// Why, a line each.
    pub lines: Vec<String>,
}

/// The program's `(solve … :expect …)`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimPart {
    /// `false` (every node dead), `region` (R's states one to one), `owing`
    /// (R empty, these alive nodes owing), or `unread` (a claim the library
    /// could not read).
    pub shape: String,
    /// The states it lists, under `region` and `owing`.
    pub states: Option<usize>,
    /// `held`, `failed` or `not-checked`; `null` where the run checked
    /// nothing.
    pub outcome: Option<String>,
    pub lines: Vec<String>,
}

/// One state the run reached.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatePart {
    /// Its facts over the candidates' relations, root's included, sorted:
    /// what the library gives of a state, and what tells R's states apart.
    pub facts: Vec<String>,
    /// `sha256:` of `facts`, one a line.
    pub digest: String,
    /// What the digest is of: `model`, the facts above, never a whole
    /// state.
    pub of: String,
    /// `minimal`, `maximal`: the lines it is on, as far as the run
    /// certified them.
    pub lines: Vec<String>,
    /// Printed in `ein solve`'s listing.
    pub listed: bool,
}

/// What is owed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Owed {
    /// Each instance root owes, through its rule's `:why`, under a
    /// description; `null` under any other question.
    pub root: Option<Vec<String>>,
    /// An empty R's owing frontier, where the run computed it.
    pub frontier: Option<Vec<Owing>>,
}

/// An alive node of an empty R's frontier, owing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Owing {
    pub facts: Vec<String>,
    pub owed: Vec<String>,
}

/// A dead root's core.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Core {
    /// The facts its contradiction rests on; `[]` where it rests on none.
    pub facts: Vec<String>,
    /// Each fact's `:source`, in the core's order, `null` for a fact with
    /// none.
    pub sources: Vec<Option<String>>,
    /// The core's search: `within-budget`, or `cut-` and its caps.
    pub status: String,
    /// The rules whose firing produced the contradiction.
    pub refuted_by: Vec<String>,
}

/// What the call cost.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Work {
    /// Fixpoints the run computed, as the summary's `work`.
    pub saturations: Option<u64>,
    /// Facts beyond root, summed over them.
    pub facts: Option<u64>,
    /// Route 1's completed layers.
    pub layers: Option<u64>,
    /// The run's wall clock as the library measured it, in microseconds.
    pub run_us: Option<u64>,
    /// The load and the judgement of the mechanisms, measured by EinAf;
    /// shared by every question of one call.
    pub load_us: u64,
    /// The question, asked and read, measured by EinAf.
    pub ask_us: u64,
}

/// The kernel failed at the bounded call.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fault {
    /// `panic`, or `engine`: an error of the engine's that names no kind.
    pub kind: String,
    pub message: String,
}

impl Feedback {
    /// The object as JSON, pretty-printed, keys in the type's order.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("a feedback object serialises")
    }

    /// Read back what [`Feedback::to_json`] wrote; an object of another
    /// schema is refused.
    pub fn from_json(text: &str) -> Result<Feedback, String> {
        let f: Feedback = serde_json::from_str(text).map_err(|e| e.to_string())?;
        if f.schema != SCHEMA {
            return Err(format!("schema {}, not {SCHEMA}", f.schema));
        }
        Ok(f)
    }

    /// The outcome under `part`, where the question gives that reading.
    pub fn outcome_over(&self, part: Part) -> Option<&Reading> {
        self.outcome.as_ref()?.iter().find(|r| r.part == part)
    }
}

/// What every object of one call shares.
pub(crate) struct Shared<'a> {
    pub(crate) identity: &'a Identity,
    pub(crate) name: &'a str,
    pub(crate) kernel: &'a KernelId,
    pub(crate) bounds: Bounds,
    pub(crate) load_us: u64,
    /// The load's notes, which `ein solve` prints last.
    pub(crate) notes: &'a [ein_core::Diagnostic],
}

/// The parts every object has, whatever came back.
fn shell(shared: &Shared<'_>, question: Question, ask_us: u64) -> Feedback {
    let mode = question.mode();
    Feedback {
        schema: SCHEMA.to_string(),
        program: ProgramId {
            name: shared.name.to_string(),
            digest: shared.identity.digest.clone(),
            of: shared.identity.of.map(str::to_string),
        },
        kernel: shared.kernel.clone(),
        asked: Asked {
            question: question.flags(),
            which: match mode.which {
                Which::All => Part::All,
                Which::Minimal => Part::Minimal,
                Which::Maximal => Part::Maximal,
            },
            ask: match (question, mode.ask) {
                (Question::NoSearch, _) => "no-search",
                (_, Ask::Single) => "single",
                (_, Ask::Count) => "count",
                (_, Ask::Find(_)) => "find",
                (_, Ask::Describe) => "describe",
            }
            .to_string(),
            budget: shared.bounds.budget,
            limit: shared.bounds.limit,
            max_steps: shared.bounds.max_steps,
            clock_us: shared.bounds.clock.map(|c| c.as_micros() as u64),
        },
        outcome: None,
        readings: None,
        diagnostics: Vec::new(),
        flagged: None,
        queries: None,
        claim: None,
        states: None,
        owed: None,
        core: None,
        work: Work {
            saturations: None,
            facts: None,
            layers: None,
            run_us: None,
            load_us: shared.load_us,
            ask_us,
        },
        dependencies: None,
        provenance: None,
        fault: None,
    }
}

/// The object of a call the kernel faulted on.
pub(crate) fn faulted(
    shared: &Shared<'_>,
    question: Question,
    ask_us: u64,
    kind: &str,
    message: String,
) -> Feedback {
    let mut f = shell(shared, question, ask_us);
    f.fault = Some(Fault {
        kind: kind.to_string(),
        message,
    });
    f
}

pub(crate) fn finding(d: &ein_core::Diagnostic) -> Finding {
    Finding {
        kind: d.kind.name().to_string(),
        severity: d.severity.name().to_string(),
        message: d.message.clone(),
        place: d.place.as_ref().map(|p| Place {
            file: p.file.clone(),
            line: p.line,
            col: p.col,
        }),
    }
}

/// **The object of one run**, from the library's values: the load's
/// result, the read-out, and what the read-out says beside the answer, each
/// line of `ein solve`'s stderr, as `ein_render` gives it as values.
pub(crate) fn build(shared: &Shared<'_>, run: &Run, ask_us: u64) -> Feedback {
    let mut f = shell(shared, run.question, ask_us);
    f.outcome = Some(outcome::readings(run));
    match &run.answered {
        Answered::Refused(refusal) => {
            f.diagnostics = refusal.findings.iter().map(finding).collect();
        }
        Answered::Clock => {
            // The clock's abort, as `ein solve` says it: the line is the
            // binary's, which the library does not hand over.
            let secs = shared.bounds.clock.map_or(0.0, |c| c.as_secs_f64());
            f.diagnostics.push(finding(&ein_core::Diagnostic::error(
                ein_core::Kind::Deadline,
                format!("--max-time {secs} s reached before the question was decided"),
            )));
            f.diagnostics.extend(shared.notes.iter().map(finding));
        }
        Answered::Read(r) => {
            let program = run.program.as_deref().expect("a run's copy");
            read(&mut f, shared, run, r, program);
        }
    }
    f
}

fn read(f: &mut Feedback, shared: &Shared<'_>, run: &Run, r: &ReadOut, p: &Program) {
    let a = &r.answer;
    let cut = matches!(a.stopped, Some(Stopped::Steps { .. }));
    let (saturations, facts, layers) = match r.described() {
        Some(d) => (d.work.saturations, d.work.facts, d.work.layers),
        None => (a.saturations, a.facts, None),
    };
    f.work.saturations = Some(saturations);
    f.work.facts = Some(facts);
    f.work.layers = layers.map(|l| l as u64);
    f.work.run_us = Some(a.wall.as_micros() as u64);

    // The static check, read before any answer: a run a saturation stopped
    // at the cap read none.
    if !cut && !a.monotonicity.is_monotone() {
        let m = &a.monotonicity;
        f.flagged = Some(Flagged {
            sites: m
                .sites
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let stake = m.detector.stakes.get(i);
                    Site {
                        rule: p.terms.sym(s.rule).to_string(),
                        activator: s.activator.iter().map(|&v| p.terms.display(v)).collect(),
                        kind: s.kind.as_str().to_string(),
                        relation: s.relation.label(&p.terms),
                        depth: s.depth,
                        concludes: stake.map(|x| x.concluded.clone()),
                        at_stake: stake.map(|x| x.at_stake.key().to_string()),
                    }
                })
                .collect(),
            in_flight: m.in_flight.clone(),
        });
    }

    // A question that needs the cube, asked of a program that is not
    // monotone, ran under a limit of 0: the flag is the object, and nothing
    // of the run is an answer to the question asked.
    if run.cube_skipped {
        f.diagnostics
            .push(finding(&ein_infer::detector::warning(shared.name)));
        f.diagnostics.extend(shared.notes.iter().map(finding));
        return;
    }

    // What `ein solve` prints on stderr: the in-flight warning, said when it
    // was found and before the read-out, then the read-out's lines, then the
    // load's notes.
    if a.monotonicity.sites.is_empty() && a.monotonicity.in_flight.is_some() {
        f.diagnostics
            .push(finding(&ein_infer::detector::warning(shared.name)));
    }
    let argv = match run.question {
        Question::Describe => String::new(),
        q => q.flags(),
    };
    let text = ein_render::read_out::render(
        r,
        &ein_render::read_out::Options {
            command: "solve",
            argv: &argv,
            file: shared.name,
            budget: shared.bounds.budget,
            uncut: false,
        },
    );
    f.diagnostics.extend(text.diagnostics.iter().map(finding));
    f.diagnostics.extend(shared.notes.iter().map(finding));

    f.readings = Some(readings_of(run.question, r));
    f.states = (a.mode.ask != Ask::Count && !cut).then(|| {
        r.states
            .iter()
            .map(|s| {
                let mut facts = s.model.clone();
                facts.sort();
                let mut lines = Vec::new();
                if s.minimal {
                    lines.push("minimal".to_string());
                }
                if s.maximal {
                    lines.push("maximal".to_string());
                }
                StatePart {
                    digest: sha256(
                        facts
                            .iter()
                            .map(|x| format!("{x}\n"))
                            .collect::<String>()
                            .as_bytes(),
                    ),
                    facts,
                    of: "model".to_string(),
                    lines,
                    listed: s.listed,
                }
            })
            .collect()
    });
    f.queries = Some(queries_of(r, p));
    f.claim = claim_of(r, p);

    let d = r.described();
    let frontier = match &r.listing {
        Listing::Owing(fr) if !d.is_some_and(|d| d.frontier_skipped) => Some(
            fr.iter()
                .map(|o| Owing {
                    facts: o.model.clone(),
                    owed: o.node.owed.clone(),
                })
                .collect(),
        ),
        _ => None,
    };
    let root = d.map(|d| d.root.owed.clone());
    if root.is_some() || frontier.is_some() {
        f.owed = Some(Owed { root, frontier });
    }
    if let (Listing::False, Some(d)) = (&r.listing, d)
        && !d.root.alive
        && let Some(status) = d.root.core_status
    {
        f.core = Some(Core {
            facts: d.root.core.clone(),
            sources: d.root.core_sources.clone(),
            status: status.word(),
            refuted_by: d.root.refuted_by.clone(),
        });
    }
}

/// The readings of *one answer* the run established, the question's own and
/// no other.
fn readings_of(question: Question, r: &ReadOut) -> Readings {
    let a = &r.answer;
    let part_of = |w: Which| match w {
        Which::All => Part::All,
        Which::Minimal => Part::Minimal,
        Which::Maximal => Part::Maximal,
    };
    let mut out = Readings {
        word: None,
        k_s: None,
        k_c: None,
        minimal: None,
        maximal: None,
        exhausted: Some(a.exhausted),
        stopped: a.stopped.as_ref().map(|s| match s {
            Stopped::Budget { .. } => Stop {
                why: "budget".to_string(),
                saturation: None,
            },
            Stopped::Limit { .. } => Stop {
                why: "limit".to_string(),
                saturation: None,
            },
            Stopped::Deadline => Stop {
                why: "clock".to_string(),
                saturation: None,
            },
            Stopped::NoSearch => Stop {
                why: "asked".to_string(),
                saturation: None,
            },
            Stopped::Steps { root, .. } => Stop {
                why: "steps".to_string(),
                saturation: Some(if *root { "root" } else { "node" }.to_string()),
            },
        }),
    };
    match question {
        Question::Single(_) => {
            out.word = a.verdict.map(|v| Word {
                part: part_of(a.mode.which),
                word: v.word().to_string(),
            });
        }
        Question::Line(Part::Minimal) => out.minimal = Some(a.states.len() as u64),
        Question::Line(Part::Maximal) => out.maximal = Some(a.states.len() as u64),
        Question::Line(Part::All) | Question::Describe | Question::Count => {
            if let Some(d) = r.described() {
                out.k_s = Some(d.k_s());
                out.k_c = Some(d.k_c);
                out.minimal = Some(d.states.iter().filter(|s| s.minimal).count() as u64);
                out.maximal = Some(d.states.iter().filter(|s| s.maximal).count() as u64);
                out.exhausted = Some(d.exhausted);
            }
        }
        Question::NoSearch => {}
    }
    out
}

/// The program's `(query …)` forms with a goal, in the order the read-out
/// answers them: each one's `:goal-text` and `:expect`'s count of classes.
fn asked_queries(p: &Program) -> Vec<(Option<String>, Option<usize>)> {
    p.kb.program()
        .queries
        .iter()
        .filter(|q| ein_infer::query_value(&p.ast, q, "goal").is_some())
        .map(|q| {
            let text =
                ein_infer::query_value(&p.ast, q, "goal-text").and_then(|n| match p.ast.node(n) {
                    ein_ir::Node::Str(s) => Some(p.ast.sym(s).to_string()),
                    _ => None,
                });
            let classes = ein_infer::query_value(&p.ast, q, "expect")
                .map(|e| ein_ir::expect::parse(&p.ast, e).models().len());
            (text, classes)
        })
        .collect()
}

fn checked_word(o: ein_infer::Outcome) -> String {
    match o {
        ein_infer::Outcome::Held => "held",
        ein_infer::Outcome::Failed => "failed",
        ein_infer::Outcome::NotChecked => "not-checked",
    }
    .to_string()
}

fn queries_of(r: &ReadOut, p: &Program) -> Vec<QueryPart> {
    let asked = asked_queries(p);
    r.queries
        .iter()
        .enumerate()
        .map(|(i, q)| {
            let (goal_text, claimed) = asked.get(i).cloned().unwrap_or((None, None));
            QueryPart {
                goal: q.goal.clone(),
                goal_text,
                over: match (q.as_asked, q.over) {
                    (false, _) => "listed",
                    (true, ein_ir::expect::Over::All) => "all",
                    (true, ein_ir::expect::Over::Minimal) => "minimal",
                    (true, ein_ir::expect::Over::Maximal) => "maximal",
                }
                .to_string(),
                exact: q.exact,
                agree: q.classes.len() == 1,
                classes: q
                    .classes
                    .iter()
                    .map(|c| ClassPart {
                        facts: c.facts.clone(),
                        rows: c
                            .rows
                            .iter()
                            .map(|row| row.iter().cloned().collect())
                            .collect(),
                        lines: c.rendered.clone(),
                        states: c.states.iter().map(|&s| s + 1).collect(),
                    })
                    .collect(),
                expect: match (claimed, &q.claim) {
                    (None, None) => None,
                    (classes, report) => Some(ExpectPart {
                        classes: classes.unwrap_or(0),
                        outcome: report.as_ref().map(|c| checked_word(c.outcome)),
                        lines: report.as_ref().map_or_else(Vec::new, |c| c.lines.clone()),
                    }),
                },
            }
        })
        .collect()
}

fn claim_of(r: &ReadOut, p: &Program) -> Option<ClaimPart> {
    let program = p.kb.program();
    let solve = program.solve()?;
    let node = ein_infer::solve_value(&p.ast, solve, "expect")?;
    let (shape, states) = match ein_ir::claim::parse(&p.ast, node) {
        Ok(ein_ir::claim::Claim::False) => ("false", None),
        Ok(ein_ir::claim::Claim::Region(reg)) if !reg.owing.is_empty() => {
            ("owing", Some(reg.owing.len()))
        }
        Ok(ein_ir::claim::Claim::Region(reg)) => ("region", Some(reg.models.len())),
        Err(_) => ("unread", None),
    };
    Some(ClaimPart {
        shape: shape.to_string(),
        states,
        outcome: r.claim.as_ref().map(|c| checked_word(c.outcome)),
        lines: r.claim.as_ref().map_or_else(Vec::new, |c| c.lines.clone()),
    })
}
