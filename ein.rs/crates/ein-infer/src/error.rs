//! [`SolveError`] — why a question about `R` was not answered.
//!
//! Every route ([`crate::explore`], [`crate::region`], [`crate::states`],
//! [`crate::downset`]) and the read-out ([`crate::read_out`]) return it. A
//! budget that stops a route is no error: the route reports an answer that
//! says it is partial. `-y` is the detector's spot check unbounded, and its
//! one failure is [`SolveError::SpotCheck`].

use crate::compile::CompileError;
use crate::saturator::SaturateError;

#[derive(Debug)]
pub enum SolveError {
    Saturate(SaturateError),
    Compile(CompileError),
    /// A question the engine does not answer, or a claim it cannot read: the
    /// message says which.
    Unsupported(String),
    /// `-T`: the wall clock ran out inside the run. A
    /// machine-dependent limit is never an answer.
    Deadline,
    /// `-y` on a program the static check calls monotone found a parent path
    /// that reaches another state: the static check's theorem
    /// ([monotonicity.md § 3.2](../../../../docs/ein/reasoning/kb-evolution/monotonicity.md#32-the-theorem))
    /// says none can, so this is an engine error, never an answer. The message
    /// names the state and the parent.
    SpotCheck(String),
    /// *Forbid* (Q-M1e2.5): a refutation rests on an `absent` over a
    /// relation a commitment can still extend, so the program is refused
    /// before any question about a commitment is answered. One line per
    /// refusing rule ([`crate::forbid`]); a load error's exit code.
    Refused(String),
}

impl std::fmt::Display for SolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolveError::Saturate(e) => write!(f, "{e}"),
            SolveError::Compile(e) => write!(f, "{e}"),
            SolveError::Unsupported(m) => f.write_str(m),
            SolveError::Deadline => f.write_str("--max-time reached inside the cube"),
            SolveError::SpotCheck(m) => f.write_str(m),
            SolveError::Refused(m) => f.write_str(m),
        }
    }
}

impl From<SaturateError> for SolveError {
    fn from(e: SaturateError) -> Self {
        SolveError::Saturate(e)
    }
}

impl From<CompileError> for SolveError {
    fn from(e: CompileError) -> Self {
        SolveError::Compile(e)
    }
}
