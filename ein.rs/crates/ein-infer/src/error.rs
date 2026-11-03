//! [`SolveError`] — why a question about `R` was not answered.
//!
//! It was the walk's (`solve.rs`) until M1e2
//! [S1e2.1c.5](../../../../plans/m1e2_correctness/p1e2.1c_the_region_search/s1e2.1c.5_wrong_answers_re_scoped.md)
//! deleted the walk on the user's ruling of 2026-11-03, and every route
//! ([`crate::explore`], [`crate::region`], [`crate::states`],
//! [`crate::downset`]) and the read-out ([`crate::read_out`]) return it. The
//! walk's two arms went with it: a budget that stopped the walk, which a route
//! reports as an answer that says it is partial, and `-y`'s commutativity
//! failure, which the walk's sanity probe raised.

use crate::compile::CompileError;
use crate::saturator::SaturateError;

#[derive(Debug)]
pub enum SolveError {
    Saturate(SaturateError),
    Compile(CompileError),
    /// A question the engine does not answer, or a claim it cannot read: the
    /// message says which.
    Unsupported(String),
    /// `-T`: the wall clock ran out inside the run — M1e2 T1e2.1a.5.3. A
    /// machine-dependent limit is never an answer.
    Deadline,
}

impl std::fmt::Display for SolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolveError::Saturate(e) => write!(f, "{e}"),
            SolveError::Compile(e) => write!(f, "{e}"),
            SolveError::Unsupported(m) => f.write_str(m),
            SolveError::Deadline => f.write_str("--max-time reached inside the cube"),
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
