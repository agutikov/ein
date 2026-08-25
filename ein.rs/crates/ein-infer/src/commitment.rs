//! The commitment primitive — fork, write, detect, saturate, detect.
//!
//! `try_commitment_set(root.sealed(), C)` is the one operation the whole search layer
//! is built on, and it is **pure with respect to root**: every consequence
//! stays in the fork (P1.21 R2). That is not a nicety — it is the unit
//! [P1a.7](../../../../docs/history/m1a_rust/README.md#p1a7--parallelism)
//! parallelises, and the reason nothing here writes a no-good, a `(not h)`
//! writeback or a counter. Those are the caller's commit step
//! ([design/08](../../../../docs/history/m1a_rust/design/08_parallelism.md) §2).
//!
//! ### Fail-fast
//!
//! `enable_fail_fast_fork` stops a dying fork's saturation at the firing that
//! kills it instead of running to quiescence. It is the engine's one pure
//! speed lever — same verdict, same enterings, same deaths, same clauses —
//! worth 1.9–2.4× on an exhaustive `zebra2`, because ~88 % of a dying fork's
//! saturation happens after the clash.
//!
//! Its *off* case is not dead configuration. A dead fork's `firings` then is
//! the full run and its `kb` the complete dead state, which a DAG builder that
//! merges dead commitments by `state_key` needs: two orientations of a
//! symmetric dead commitment share a fixpoint without sharing a fail-fast
//! prefix.

use ein_core::{FactId, Kb, Prov, Terms};
use ein_ir::Ast;

use crate::compile::SharedMemo;
use crate::events::Events;
use crate::firing::Firing;
use crate::obligations::Owes;
use crate::saturator::{SaturateError, Saturator, Session, Snapshot};

/// How a commitment ended.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Alive,
    /// Contradictory as soon as the hypotheses were written — no saturation
    /// ran. This catches a negative that landed at root between the
    /// candidate's generation and this fork, including one a mid-layer
    /// singleton writeback produced.
    DeadPre,
    DeadPost,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Alive => "alive",
            Kind::DeadPre => "dead-pre",
            Kind::DeadPost => "dead-post",
        }
    }
}

/// One entering's outcome.
///
/// The `kb` is owned — an `Arc` base plus a delta layer — so handing it back
/// costs nothing, which is what lets the caller keep an alive fork without a
/// second saturation.
pub struct CommitmentSetResult {
    pub commitment: Vec<FactId>,
    pub kb: Kb,
    pub firings: Vec<Firing>,
    pub kind: Kind,
    pub unsat_core: Vec<FactId>,
    /// The `(h_i)` writes for `h_i ∈ commitment` — **not** the saturator's
    /// additions.
    pub hypothesis_facts: Vec<FactId>,
    /// What this node's fixpoint still **owes** — M1d S1d.2.4.
    ///
    /// The other per-node verdict that is not a fact, and it sits beside
    /// `kind` for that reason: an `open` conclusion may not be stored, because
    /// a fork inheriting one would carry the debt after paying it
    /// ([`crate::obligations`]).
    ///
    /// **Empty on a dead node, and that is not an omission.** The read-out is
    /// three states in one order — `(false)` first, then the tally — so a node
    /// with a contradiction never has its debts consulted, and computing them
    /// would be work no surface can observe. On an exhaustive `zebra2` that is
    /// 67 of 101 enterings.
    pub owes: Owes,
}

/// Branch root, write every hypothesis in `commitment`, saturate, detect.
///
/// **Root is `&`**, which is the seam
/// [P1a.7](../../../../docs/history/m1a_rust/README.md#p1a7--parallelism) runs on:
/// what the module note above claims — that this is pure with respect to root
/// — becomes the signature rather than a promise, and a layer's workers can
/// therefore hold one root at once. Sealing root's top layer is the caller's,
/// because it is the half of [`Kb::fork`] that mutates and a fanned-out layer
/// does it **once** where the sequential path does it per entering;
/// [`Kb::sealed`] is the one-call form.
///
/// `saturator_steps` caps the fork's firings; `None` runs to the fixpoint,
/// which terminates because the M1 rule set is monotone. It is `None` on the
/// shipping path and exists for tests.
///
/// Every call is independent: two calls on the same root return two results
/// whose forks share no mutable state. `memo` is the exception and is not
/// mutable state in the sense that matters — it is an append-only cache of a
/// pure function of `(rule, activator)`, so what a fork finds in it is what it
/// would have compiled ([design/06](../../../../docs/history/m1a_rust/design/06_saturation.md)
/// § Win A). The *order* plans enter an engine's list stays per-engine, which
/// is the part the trace can see.
///
/// `resume` is `None` on every shipping path: with it the fork **continues**
/// root's saturation from the delta instead of re-deriving root's fixpoint
/// ([`Snapshot`], S1a.6.9). It is reachable only from a `fork-delta` build,
/// because dropping those re-derivations changes what the engine narrates and
/// that is [Q-M1a.18](../../../../docs/history/m1a_rust/open_questions.md)'s to
/// decide, not this function's.
// The eighth argument is `resume`, and bundling it with `memo` into a "run
// state" struct would be the tidy fix for a parameter that may not survive
// Q-M1a.18. It stays a parameter until that is decided.
#[allow(clippy::too_many_arguments)]
pub fn try_commitment_set(
    root: &Kb,
    terms: &mut Terms,
    ast: &Ast,
    events: &mut Events,
    memo: &SharedMemo,
    commitment: &[FactId],
    saturator_steps: Option<usize>,
    resume: Option<&Snapshot>,
) -> Result<CommitmentSetResult, SaturateError> {
    let cfg = root.program().config.clone().unwrap_or_default();
    let mut fork = root.branch();
    let mut hypothesis_facts = Vec::with_capacity(commitment.len());
    for &h in commitment {
        let (rel, args) = terms.facts.get(h);
        let args = args.to_vec();
        // `branch=0` is not a placeholder to improve: the branch id is
        // per-commitment context the lattice search does not use, and changing
        // it changes provenance output.
        let prov = terms.provs.push(Prov::from_hypothesis(0, None));
        let added = fork
            .add_and_index_fact(terms, rel, &args, Some(prov))
            .expect("room for a hypothesis");
        hypothesis_facts.push(added.id());
    }

    let done = |fork: Kb, firings: Vec<Firing>, kind: Kind, core: Vec<FactId>, owes: Owes| {
        CommitmentSetResult {
            commitment: commitment.to_vec(),
            kb: fork,
            firings,
            kind,
            unsat_core: core,
            hypothesis_facts: hypothesis_facts.clone(),
            owes,
        }
    };

    if let Some(core) = dead(&fork, terms) {
        let result = done(fork, Vec::new(), Kind::DeadPre, core, Owes::default());
        #[cfg(feature = "fork-delta")]
        crate::fork_audit::record(terms, &result);
        return Ok(result);
    }

    let mut s = Session {
        kb: &mut fork,
        terms,
        ast,
        events,
        memo: memo.clone(),
    };
    let mut sat = match resume {
        // The delta is everything the fork has that the snapshot did not:
        // this commitment's hypotheses, and whatever landed at root since —
        // a forced positive, a singleton `(not h)` writeback, a lookahead
        // kill cache. Asking the *fork* covers both in one question, which is
        // the point of keeping the fact set on the snapshot.
        Some(snap) => {
            let delta = snap.new_facts_of(s.kb);
            Saturator::resume(&mut s, snap, delta)?
        }
        None => Saturator::new(&mut s)?,
    };
    let firings = if cfg.enable_fail_fast_fork {
        saturate_until_dead(&mut sat, &mut s, saturator_steps)?
    } else {
        let mut out = Vec::new();
        sat.saturate(&mut s, saturator_steps, &mut |f| out.push(f.clone()))?;
        out
    };

    let result = if let Some(core) = dead(&fork, terms) {
        done(fork, firings, Kind::DeadPost, core, Owes::default())
    } else {
        // The fixpoint is reached and the node is consistent, which is the one
        // state in which what it owes can be read — see `owes`.
        let owes = crate::obligations::tally(&fork, terms, ast, memo, events)?;
        done(fork, firings, Kind::Alive, Vec::new(), owes)
    };
    #[cfg(feature = "fork-delta")]
    crate::fork_audit::record(terms, &result);
    Ok(result)
}

/// The smallest source frontier of `kb`'s contradictions, or `None` when it
/// has none.
fn dead(kb: &Kb, terms: &Terms) -> Option<Vec<FactId>> {
    let witnesses: Vec<FactId> = crate::contradiction::detect(kb, terms)
        .iter()
        .map(|c| c.witness())
        .collect();
    if witnesses.is_empty() {
        return None;
    }
    Some(crate::explain::smallest_contradiction_frontier(
        kb,
        terms,
        Some(&witnesses),
    ))
}

/// Saturate, stopping at the firing that kills the fork.
///
/// Identical to a full `saturate` on a fork that survives; on one that dies it
/// returns the prefix up to and including the firing whose conclusion made the
/// KB inconsistent, and abandons the loop there.
///
/// **Sound because the KB is append-only**: a contradiction is *created* by an
/// insertion and can never be retracted, so a fork inconsistent at firing *n*
/// is inconsistent at the fixpoint too. The verdict is therefore unchanged;
/// only the amount of dead-branch work is.
fn saturate_until_dead(
    sat: &mut Saturator,
    s: &mut Session<'_>,
    max_steps: Option<usize>,
) -> Result<Vec<Firing>, SaturateError> {
    let mut firings: Vec<Firing> = Vec::new();
    loop {
        if max_steps.is_some_and(|m| firings.len() >= m) {
            return Err(SaturateError::StepLimit(format!(
                "saturator hit max_steps={} without reaching fixed point — \
                 last firing was {:?}; see Saturator::last_firing for the \
                 runaway candidate.",
                max_steps.expect("checked"),
                sat.last_firing()
            )));
        }
        let Some(firing) = sat.step(s)? else {
            return Ok(firings);
        };
        let redundant = firing.redundant;
        let derived = firing.derived.clone();
        firings.push(firing.clone());
        sat.set_last_firing(firing);
        if redundant {
            continue; // wrote nothing, so the KB cannot have changed
        }
        for d in derived.iter() {
            if crate::contradiction::contradicts(s.kb, s.terms, *d) {
                return Ok(firings);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_core::Value;
    use ein_ir::{from_ir::load, parse};

    fn kb_of(src: &str) -> (Ast, Terms, Kb) {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, src, Some("<test>")).expect("parses");
        let kb = load(&mut ast, &mut terms, &forms, None).expect("loads");
        (ast, terms, kb)
    }

    const SRC: &str = "(rule mirror (?rel)\n  :match (?rel ?a ?b)\n  :assert (?rel ?b ?a)\n\
                       \x20 :priority 100)\n\
                       (relation r T T)\n(mirror r)\n(r A B :source \"(1)\")";

    /// Two calls on the same root are independent, and mutating one fork does
    /// not reach the other — the property
    /// [P1a.7](../../../../docs/history/m1a_rust/README.md#p1a7--parallelism) runs on.
    ///
    /// The `REPEAT` line of the parity diff already says the two calls *agree*;
    /// what it cannot say is that they agree because they are isolated rather
    /// than because nothing happened to collide. This writes into the first
    /// fork between the calls and checks the second is untouched.
    #[test]
    fn two_enterings_share_no_mutable_state() {
        let (ast, mut terms, mut kb) = kb_of(SRC);
        let mut ev = crate::events::Events::off();
        let r = terms.syms.get("r").expect("interned");
        let c = terms
            .syms
            .get("C")
            .unwrap_or_else(|| terms.intern_text("C").expect("room"));
        let d = terms.intern_text("D").expect("room");
        let h = terms
            .intern_fact(r, &[Value::sym(c), Value::sym(d)])
            .expect("room");

        // One memo across both calls, deliberately: the claim is that two
        // enterings share no *mutable* state, and an append-only plan cache is
        // the one thing they do share.
        let memo = SharedMemo::default();
        let mut first = try_commitment_set(
            kb.sealed(),
            &mut terms,
            &ast,
            &mut ev,
            &memo,
            &[h],
            None,
            None,
        )
        .expect("enters");
        let root_facts = kb.n_facts();
        let first_facts = first.kb.n_facts();

        // Mutate the first fork out from under the second call.
        let junk = terms.intern_text("junk").expect("room");
        first
            .kb
            .add_and_index_fact(&mut terms, junk, &[], None)
            .expect("room");

        let second = try_commitment_set(
            kb.sealed(),
            &mut terms,
            &ast,
            &mut ev,
            &memo,
            &[h],
            None,
            None,
        )
        .expect("enters");
        assert_eq!(second.kb.n_facts(), first_facts, "the forks are not shared");
        assert_eq!(second.kind, first.kind);
        assert_eq!(kb.n_facts(), root_facts, "root was written to");
        assert!(!kb.contains(h), "the hypothesis leaked into root");
    }

    /// The pre-saturation detect is not redundant with the post one: it is
    /// what catches a negative that landed at root *after* the candidate was
    /// generated, and it reports `dead-pre` with **no** firings.
    #[test]
    fn a_negated_hypothesis_dies_before_saturation() {
        let (ast, mut terms, mut kb) =
            kb_of("(relation r T T)\n(r A B :source \"(1)\")\n(not (r C D) :source \"(2)\")");
        let mut ev = crate::events::Events::off();
        let r = terms.syms.get("r").expect("interned");
        let (c, d) = (
            terms.intern_text("C").expect("room"),
            terms.intern_text("D").expect("room"),
        );
        let h = terms
            .intern_fact(r, &[Value::sym(c), Value::sym(d)])
            .expect("room");
        let result = try_commitment_set(
            kb.sealed(),
            &mut terms,
            &ast,
            &mut ev,
            &SharedMemo::default(),
            &[h],
            None,
            None,
        )
        .expect("enters");
        assert_eq!(result.kind, Kind::DeadPre);
        assert!(result.firings.is_empty(), "saturation ran anyway");
        assert!(!result.unsat_core.is_empty(), "a dead entering has a core");
    }
}
