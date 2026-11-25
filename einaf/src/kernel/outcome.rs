//! **The seven outcomes**: what the kernel says of a program under one
//! question, read off the values the library hands back.
//!
//! The order of the reading is the order of [`Outcome`]'s variants, and it
//! is the point: a refusal is read before anything, a program the engine
//! flags as not monotone before any answer, a run that stopped before any
//! word a stopped run might still carry, and only then the answer's word.
//! *Not decided* is never read as one of the four answers after it.
//!
//! [The outcomes](../../../docs/einaf/01-outcomes.md) is this module's page: each outcome, what it
//! is read from, and the test that holds the reading.

use ein_infer::explore::{Stopped, Verdict};
use ein_infer::read_out::ReadOut;

use super::{Answered, Part, Question, Run};

/// What the kernel says of a program under one question.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Outcome {
    /// Refused at the read, the parse, the load, or at a firing: no answer
    /// exists.
    Invalid,
    /// Not monotone, by the engine's static check or found in flight: an
    /// answer is printed and nothing is claimed of it.
    Flagged,
    /// A bound, the facts budget, the cube's limit or the clock, stopped the
    /// run before the question was decided; or the question does not decide
    /// it (`--no-search` on a root that is alive).
    NotDecided,
    /// R is empty and every node is dead.
    Inconsistent,
    /// R is empty and every alive node owes an obligation the program
    /// states.
    Incomplete,
    /// The part of R the reading counts holds two states or more.
    Several,
    /// The part of R the reading counts is one state.
    One,
}

impl Outcome {
    pub const ALL: [Outcome; 7] = [
        Outcome::Invalid,
        Outcome::Flagged,
        Outcome::NotDecided,
        Outcome::Inconsistent,
        Outcome::Incomplete,
        Outcome::Several,
        Outcome::One,
    ];

    /// The outcome's name, as EinAf's pages write it.
    pub fn word(self) -> &'static str {
        match self {
            Outcome::Invalid => "invalid",
            Outcome::Flagged => "flagged",
            Outcome::NotDecided => "not decided",
            Outcome::Inconsistent => "inconsistent",
            Outcome::Incomplete => "incomplete",
            Outcome::Several => "several",
            Outcome::One => "one",
        }
    }

    /// One of the four the kernel's answer decides: R empty of either kind,
    /// or the part counted one state or several.
    pub fn is_answer(self) -> bool {
        matches!(
            self,
            Outcome::Inconsistent | Outcome::Incomplete | Outcome::Several | Outcome::One
        )
    }
}

/// An outcome, with the question it is of and the reading it was taken
/// under, and the kernel's words for why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reading {
    pub question: Question,
    /// The part of R *one* and *several* were counted over.
    pub part: Part,
    pub outcome: Outcome,
    /// What the outcome was read from, in a line.
    pub why: String,
}

/// **The outcome of `run` under the reading `part`**, or `None` where the
/// run's question cannot be read over that part ([`Question::readings`]).
pub fn outcome(run: &Run, part: Part) -> Option<Reading> {
    if !run.question.readings().contains(&part) {
        return None;
    }
    let (outcome, why) = read(run, part);
    Some(Reading {
        question: run.question,
        part,
        outcome,
        why,
    })
}

fn read(run: &Run, part: Part) -> (Outcome, String) {
    let r: &ReadOut = match &run.answered {
        Answered::Refused(refusal) => {
            let first = refusal.message.lines().next().unwrap_or("").to_string();
            return (
                Outcome::Invalid,
                format!("refused {}: {first}", refusal.stage.words()),
            );
        }
        Answered::Clock => {
            return (
                Outcome::NotDecided,
                "the clock ran out before the question was decided".to_string(),
            );
        }
        Answered::Read(r) => r,
    };
    let a = &r.answer;
    // Flagged, before any answer: the static check's sites, or a hypothesis
    // a state above root held that root does not, found in flight.
    if !a.monotonicity.is_monotone() || a.warning.is_some() {
        let why = match &a.monotonicity.in_flight {
            Some(f) if a.monotonicity.sites.is_empty() => {
                format!("not monotone: found in flight, {f}")
            }
            _ => format!(
                "not monotone: {} site{} of the static check",
                a.monotonicity.sites.len(),
                if a.monotonicity.sites.len() == 1 {
                    ""
                } else {
                    "s"
                }
            ),
        };
        return (Outcome::Flagged, why);
    }
    // A bound stopped the run first, whatever the question.
    match &a.stopped {
        Some(Stopped::Budget { spent, projected }) => {
            return (
                Outcome::NotDecided,
                format!(
                    "stopped at the facts budget: {spent} spent, the next step projected at \
                     {projected}"
                ),
            );
        }
        Some(Stopped::Limit { n, limit }) => {
            return (
                Outcome::NotDecided,
                format!("stopped at the cube's limit: {n} candidates, the limit {limit}"),
            );
        }
        Some(Stopped::Deadline) => {
            return (
                Outcome::NotDecided,
                "the clock ran out before the question was decided".to_string(),
            );
        }
        Some(Stopped::NoSearch) | None => {}
    }
    match run.question {
        Question::NoSearch => {
            if a.root_dead {
                (
                    Outcome::Inconsistent,
                    "root is dead, so every node is".to_string(),
                )
            } else {
                (
                    Outcome::NotDecided,
                    "--no-search: root is alive, and R was not searched".to_string(),
                )
            }
        }
        Question::Single(_) => match a.verdict {
            Some(Verdict::Solution) => (Outcome::One, "Solution: one state".to_string()),
            Some(Verdict::Ambiguity) => (
                Outcome::Several,
                "Ambiguity: two states found, which differ".to_string(),
            ),
            Some(Verdict::Contradiction) => (
                Outcome::Inconsistent,
                "Contradiction: R is empty and every node is dead".to_string(),
            ),
            Some(Verdict::Open) => (
                Outcome::Incomplete,
                "Open: R is empty and every alive node owes".to_string(),
            ),
            None => (Outcome::NotDecided, "no verdict word".to_string()),
        },
        Question::Line(_) => {
            if !a.exhausted {
                return (
                    Outcome::NotDecided,
                    "the line was not listed whole".to_string(),
                );
            }
            counted(a.states.len() as u64, || a.root_dead)
        }
        Question::Describe | Question::Count => {
            let Some(d) = a.described.as_deref() else {
                return (Outcome::NotDecided, "no description".to_string());
            };
            if !d.exhausted {
                return (
                    Outcome::NotDecided,
                    "the description is partial".to_string(),
                );
            }
            let k = match part {
                Part::All => d.k_s(),
                Part::Minimal => d.states.iter().filter(|s| s.minimal).count() as u64,
                Part::Maximal => d.states.iter().filter(|s| s.maximal).count() as u64,
            };
            counted(k, || !d.any_alive)
        }
    }
}

/// An exhausted part of R, by how many states it holds: none of the dead
/// kind or of the owing kind, one, or several.
fn counted(k: u64, every_node_dead: impl Fn() -> bool) -> (Outcome, String) {
    match k {
        0 if every_node_dead() => (
            Outcome::Inconsistent,
            "R is empty and every node is dead".to_string(),
        ),
        0 => (
            Outcome::Incomplete,
            "R is empty and every alive node owes".to_string(),
        ),
        1 => (Outcome::One, "one state".to_string()),
        k => (Outcome::Several, format!("{k} states")),
    }
}
