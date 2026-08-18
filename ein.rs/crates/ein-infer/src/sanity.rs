//! S1.5b.27 — the saturation-commutativity sanity check.
//!
//! Deferred out of [S1a.4.5](../../../../plans/m1a_rust/p1a.4_search_layer/s1a.4.5_solve_loop.md)
//! ("moves to P1a.5") and landed here: it is off by default, costs `k+1`
//! saturations per checked commitment, and has no bearing on a shipping
//! verdict — but `ein solve -y` turns it on, and a flag whose *effect* is
//! absent is not a drop-in replacement. It is invisible at T0/T1/T3, which is
//! why the T2 event trace is what found it missing.
//!
//! The claim: for an alive size-`k` commitment `C`, every `(k−1)`-subset
//! parent path saturates to the same KB as the direct path — compared by
//! exact [`state_key`] equality, so a digest collision can never mask a real
//! violation (P1.21 R1). The digests in the message are display only.

use ein_core::prov::Prov;
use ein_core::{FactId, Kb, Terms};
use ein_ir::Ast;

use crate::apriori::CanonicalSetId;
use crate::canon::{state_digest, state_key};
use crate::commitment::{Kind, try_commitment_set};
use crate::events::Events;
use crate::saturator::{SaturateError, Saturator, Session};

/// Two lattice paths to the same commitment produced KBs with distinct state
/// keys.
#[derive(Clone, Debug)]
pub struct SanityError {
    pub commitment: CanonicalSetId,
    pub direct_state_key: Box<[FactId]>,
    /// Only the *mismatching* parents, each with the key its path reached.
    pub parent_state_keys: Vec<(CanonicalSetId, Box<[FactId]>)>,
    /// Rendered at construction: `__str__` reprs the commitments, and the
    /// repr needs a `Terms` the error cannot borrow.
    rendered: String,
}

impl std::fmt::Display for SanityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.rendered)
    }
}

/// `SanityError.__str__` — the digests are `state_digest` in Python's `#x`
/// form, which is `0x…` lowercase hex.
fn render(
    terms: &Terms,
    commitment: &[FactId],
    direct: &[FactId],
    mismatches: &[(CanonicalSetId, Box<[FactId]>)],
) -> String {
    let set_repr = |ids: &[FactId]| {
        let items: Vec<String> = ids
            .iter()
            .map(|&f| {
                let (rel, args) = terms.facts.get(f);
                let args: Vec<String> = args
                    .iter()
                    .map(|a| terms.py_value(*a))
                    .map(|v| ein_core::pyrepr::repr(&v))
                    .collect();
                format!(
                    "({}, ({}{}))",
                    ein_core::pyrepr::repr_str(terms.sym(rel)),
                    args.join(", "),
                    if args.len() == 1 { "," } else { "" }
                )
            })
            .collect();
        format!(
            "({}{})",
            items.join(", "),
            if items.len() == 1 { "," } else { "" }
        )
    };
    let parent_lines: Vec<String> = mismatches
        .iter()
        .map(|(p, k)| format!("    {} -> {:#x}", set_repr(p), state_digest(k)))
        .collect();
    format!(
        "Saturation commutativity violated for {}\n  direct state_key digest = {:#x}\n  parent paths:\n{}",
        set_repr(commitment),
        state_digest(direct),
        parent_lines.join("\n")
    )
}

/// Verify saturation commutativity for one `commitment` against `root`.
///
/// No-op below size 2 — a singleton has no parents and the claim is trivially
/// satisfied. A `dead-pre` direct path is skipped too: there is no saturated
/// fork to compare against, and the pre-saturation contradiction depends only
/// on `root + C` set-union fact equality, which is deterministic on its own.
pub fn check_commutativity(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    events: &mut Events,
    commitment: &[FactId],
) -> Result<Option<SanityError>, SaturateError> {
    if commitment.len() < 2 {
        return Ok(None);
    }
    let direct = try_commitment_set(root, terms, ast, events, commitment, None)?;
    if direct.kind == Kind::DeadPre {
        return Ok(None);
    }
    let direct_key = state_key(&direct.kb);

    let mut mismatches: Vec<(CanonicalSetId, Box<[FactId]>)> = Vec::new();
    for i in 0..commitment.len() {
        let parent: CanonicalSetId = commitment
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, &f)| f)
            .collect();
        let missing = commitment[i];
        let parent_result = try_commitment_set(root, terms, ast, events, &parent, None)?;
        if parent_result.kind != Kind::Alive {
            // A dead parent means the lattice path through it does not exist;
            // skip rather than fail.
            continue;
        }
        let mut fork = parent_result.kb;
        let (rel, args) = terms.facts.get(missing);
        let args = args.to_vec();
        let prov = terms.provs.push(Prov::from_hypothesis(0, None));
        let _ = fork.add_and_index_fact(terms, rel, &args, Some(prov));
        {
            let mut s = Session {
                kb: &mut fork,
                terms,
                ast,
                events,
            };
            let mut sat = Saturator::new(&mut s)?;
            sat.saturate(&mut s, None, &mut |_| {})?;
        }
        let parent_key = state_key(&fork);
        if parent_key != direct_key {
            mismatches.push((parent, parent_key));
        }
    }

    if mismatches.is_empty() {
        return Ok(None);
    }
    let rendered = render(terms, commitment, &direct_key, &mismatches);
    Ok(Some(SanityError {
        commitment: commitment.to_vec(),
        direct_state_key: direct_key,
        parent_state_keys: mismatches,
        rendered,
    }))
}
