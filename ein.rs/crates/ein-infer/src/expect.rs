//! What a `:expect` check reports: its [`Outcome`] and its [`Report`], and a
//! fact's provenance as a claim's diagnostic prints it.
//!
//! The comparison is [`crate::claim`]'s since M1e2: a query's `:expect` lists
//! its projection classes over the states its `:over` names, and a program's
//! `(solve … :expect (region …))` is checked against the `R` the run
//! described ([`crate::claim::check_described`]). The walk's comparison of a
//! `:expect` against its verdict, `check`, which M1c
//! [S1c.1.2](../../../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)
//! T1c.1.2.4 wrote here with its relation-closure rule (*naming a relation
//! asserts its complete extent*, now the claim's), had no caller since M1e2
//! S1e2.1a.5's switch, and went with the walk at S1e2.1c.5 (2026-11-03).

use ein_core::{FactId, Kb, ProvKind, Terms};

use crate::events::sexpr;

/// Three outcomes, not two.
///
/// **An expectation is a claim about the *exhausted* answer**, because the
/// verdict it names is: `Solution` means one model and no other, `(or …)` with
/// k disjuncts means k and no k+1-th, `(false)` means every branch died. A
/// search that stopped early establishes a *lower bound* on k, which confirms
/// none of those — so "the counts happen to agree" is not a pass, and calling
/// it one would be a green result for a claim nobody checked.
///
/// It only bites where more searching could have changed the answer.
/// `found > claimed` is a genuine failure whether or not the lattice was
/// exhausted, and so is a model that disagrees with the expectation it was
/// matched to: no amount of further search unfinds them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
    /// The claim holds, and the search that established it was exhausted.
    Held,
    /// The claim is false, and more searching would not rescue it.
    Failed,
    /// The search stopped early; the claim was neither confirmed nor refuted.
    NotChecked,
}

/// What the check found. `lines` is empty exactly when [`Outcome::Held`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub outcome: Outcome,
    /// One line per disagreement, in the order they were found — the loader's
    /// convention, and what a person debugging a rule reads.
    pub lines: Vec<String>,
}

impl Report {
    /// Held — and *only* held. A caller that treats `NotChecked` as success is
    /// the failure mode this enum exists to prevent, so the question has one
    /// spelling.
    pub fn passed(&self) -> bool {
        self.outcome == Outcome::Held
    }
}

/// Why this fact is in the model — one line, because "and where did *that*
/// come from" is `--trace`'s question and not this report's.
///
/// A surplus fact is the case [S1c.1.2](../../../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)
/// built relation-closure for, and the *next* thing its reader wants is the
/// rule that put it there — the `disjunctive-prune` guard bug this milestone
/// is written around was found exactly one step past "there is an extra fact
/// here". One level of premises, not a walk: the primary justification is what
/// the engine chose, and a reader who needs the rest reaches for `--trace`.
pub(crate) fn provenance(terms: &Terms, kb: &Kb, id: FactId) -> Option<String> {
    let prov = terms.provs.get(kb.primary(id)?);
    match prov.kind {
        ProvKind::Rule => {
            let rule = terms.sym(prov.rule?);
            if prov.premises.is_empty() {
                // A rule record with no premises is a synthetic engine
                // writeback, whose contract is that walks stop on it.
                return Some(format!("written back by {rule}"));
            }
            let premises: Vec<String> = prov.premises.iter().map(|&p| sexpr(terms, p)).collect();
            Some(format!("derived by {rule} from {}", premises.join(" ")))
        }
        ProvKind::Source => Some(match prov.source {
            Some(s) => format!("given by {}", terms.sym(s)),
            None => "in the program's own text".to_string(),
        }),
        ProvKind::Hypothesis => Some(match prov.branch {
            Some(b) => format!("hypothesised in branch {b}"),
            None => "hypothesised".to_string(),
        }),
        // Not reachable from a model — a rejected fact is not in one — and
        // named rather than swallowed, so it reads as a bug if it ever is.
        ProvKind::Rejected => Some("recorded as rejected".to_string()),
    }
}
