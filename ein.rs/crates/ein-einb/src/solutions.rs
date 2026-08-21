//! `SOLUTIONS` — what a search found, as a delta against the KB in the same
//! file (T1a.8.1.6, [design/10
//! §5](../../../../plans/m1a_rust/design/10_binary_format.md#5-the-solution-store)).
//!
//! A model is a `Kb`, and a `Kb` is a stack of layers over a base. Storing one
//! whole would store the base again per solution; storing the **delta** — the
//! facts the branch added — stores what makes it a different model, and
//! reconstituting it is `base.fork()` plus that delta, which is the structure
//! the engine already runs on ([design/03
//! §5](../../../../plans/m1a_rust/design/03_data_model.md)).
//!
//! ## The measurement hazard, and what is done about it
//!
//! [F9](../../../../plans/followups/f9_e_catalog.md) says it about this
//! section's ancestor: **a stored answer memoises the puzzle rather than
//! improving the reasoner.** The mitigation here is structural rather than
//! advisory — nothing in the CLI's solve path reads this section, so no
//! benchmark can accidentally time a lookup, and a reader that wants a stored
//! model has to ask for it by name. What a file carries and what a run
//! *computes* stay separable, which is the property a measurement needs.

use ein_core::facts::FactId;
use ein_core::intern::Symbol;
use ein_core::value::Value;
use ein_core::{Kb, Terms};
use ein_infer::solve::{BaseStats, MonotonicStats};
use ein_infer::verdict::{Answer, Verdict};
use ein_ir::Ast;

use crate::tables::Maps;
use crate::wire::{Reader, Writer};
use crate::{EinbError, Result};

/// The verdict, without the `Kb`s hanging off it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VerdictKind {
    /// `k = 1`.
    Solution,
    /// `k > 1` — a genuine gap.
    Ambiguity,
    /// `k = 0`.
    Contradiction,
    /// A budget was spent. Not a verdict; recorded so a reader is not told one.
    Aborted,
}

/// One solution node.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SolutionNode {
    /// The canonical identity of the state — `canon::state_key`, the sorted
    /// fact list, which is what the search deduped nodes by.
    pub state_key: Box<[FactId]>,
    /// The facts this model has and the file's KB does not, in the model's own
    /// insertion order.
    pub delta: Box<[FactId]>,
    /// `(?var, value)` per goal solution, in bind order.
    pub bindings: Vec<Vec<(Symbol, Value)>>,
}

impl SolutionNode {
    /// `base.fork()` plus the delta — the model, believed again.
    ///
    /// The provenance of a delta fact is whatever the arena already holds for
    /// it; the fork records none of its own, because a stored model is a
    /// *state*, not a re-derivation, and inventing a justification for it
    /// would put a record in the trace that no rule ever fired.
    pub fn reconstitute(&self, base: &mut Kb) -> Kb {
        let mut kb = base.fork();
        for &f in &self.delta {
            if !kb.contains(f) {
                kb.restore_fact(f, None);
            }
        }
        kb
    }
}

/// The whole section, in memory.
#[derive(Clone, PartialEq, Debug)]
pub struct Solutions {
    pub verdict: VerdictKind,
    /// Non-empty only for [`VerdictKind::Contradiction`].
    pub unsat_core: Vec<FactId>,
    pub nodes: Vec<SolutionNode>,
    pub stats: MonotonicStats,
}

impl Solutions {
    /// Read a finished solve into the stored shape.
    ///
    /// `base` is the KB the file will hold — the delta of every node is taken
    /// against it, so the two have to be saved together or the deltas name
    /// nothing.
    pub fn of(
        base: &Kb,
        terms: &mut Terms,
        ast: &Ast,
        answer: &Answer,
        stats: MonotonicStats,
    ) -> Solutions {
        let models: Vec<&Kb> = match answer {
            Answer::Verdict(Verdict::Solution(s)) => vec![&s.kb],
            Answer::Verdict(Verdict::Ambiguity(all)) => all.iter().map(|s| &s.kb).collect(),
            _ => Vec::new(),
        };
        let nodes = models
            .into_iter()
            .map(|kb| SolutionNode {
                state_key: ein_infer::state_key(kb),
                delta: kb.facts().filter(|f| !base.contains(*f)).collect(),
                bindings: ein_infer::goal_bindings(ast, terms, kb, None),
            })
            .collect();
        Solutions {
            verdict: match answer {
                Answer::Verdict(Verdict::Solution(_)) => VerdictKind::Solution,
                Answer::Verdict(Verdict::Ambiguity(_)) => VerdictKind::Ambiguity,
                Answer::Verdict(Verdict::Contradiction { .. }) => VerdictKind::Contradiction,
                Answer::Aborted { .. } => VerdictKind::Aborted,
            },
            unsat_core: match answer {
                Answer::Verdict(Verdict::Contradiction { unsat_core }) => unsat_core.clone(),
                _ => Vec::new(),
            },
            nodes,
            stats,
        }
    }
}

pub fn write(s: &Solutions) -> Vec<u8> {
    let mut w = Writer::new();
    w.u8(match s.verdict {
        VerdictKind::Solution => 0,
        VerdictKind::Ambiguity => 1,
        VerdictKind::Contradiction => 2,
        VerdictKind::Aborted => 3,
    });
    write_ids(&mut w, &s.unsat_core);
    w.u32(s.nodes.len() as u32);
    for n in &s.nodes {
        write_ids(&mut w, &n.state_key);
        write_ids(&mut w, &n.delta);
        w.u32(n.bindings.len() as u32);
        for set in &n.bindings {
            w.u32(set.len() as u32);
            for (name, value) in set {
                w.u32(name.0);
                w.u32(value.bits());
            }
        }
    }
    write_stats(&mut w, &s.stats);
    w.align(crate::header::ALIGN);
    w.into_vec()
}

pub fn read(body: &[u8], maps: &Maps) -> Result<Solutions> {
    let mut r = Reader::new(body);
    let verdict = match r.u8()? {
        0 => VerdictKind::Solution,
        1 => VerdictKind::Ambiguity,
        2 => VerdictKind::Contradiction,
        3 => VerdictKind::Aborted,
        _ => return Err(EinbError::Malformed("unknown verdict")),
    };
    let unsat_core = read_ids(&mut r, maps)?;
    let n = r.count(12)?;
    let mut nodes = Vec::with_capacity(n);
    for _ in 0..n {
        let state_key = read_ids(&mut r, maps)?.into_boxed_slice();
        let delta = read_ids(&mut r, maps)?.into_boxed_slice();
        let n_sets = r.count(4)?;
        let mut bindings = Vec::with_capacity(n_sets);
        for _ in 0..n_sets {
            let n_pairs = r.count(8)?;
            let mut set = Vec::with_capacity(n_pairs);
            for _ in 0..n_pairs {
                let name = maps.symbol(r.u32()?)?;
                set.push((name, maps.value(r.u32()?)?));
            }
            bindings.push(set);
        }
        nodes.push(SolutionNode {
            state_key,
            delta,
            bindings,
        });
    }
    Ok(Solutions {
        verdict,
        unsat_core,
        nodes,
        stats: read_stats(&mut r)?,
    })
}

fn write_ids(w: &mut Writer, ids: &[FactId]) {
    w.u32(ids.len() as u32);
    for f in ids {
        w.u32(f.0);
    }
}

fn read_ids(r: &mut Reader<'_>, maps: &Maps) -> Result<Vec<FactId>> {
    let n = r.count(4)?;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(maps.fact(r.u32()?)?);
    }
    Ok(out)
}

fn write_stats(w: &mut Writer, s: &MonotonicStats) {
    for v in [
        s.base.enterings_total,
        s.base.enterings_alive,
        s.base.enterings_dead_pre,
        s.base.enterings_dead_post,
        s.base.facts_merged,
        s.base.forced_positives,
        s.base.saturate_count,
        s.base.layers_explored,
        s.base.nogoods_emitted,
        s.base.nogoods_subsumed,
        s.solution_nodes,
    ] {
        w.u64(v);
    }
    w.u8(u8::from(s.exhausted));
}

fn read_stats(r: &mut Reader<'_>) -> Result<MonotonicStats> {
    let base = BaseStats {
        enterings_total: r.u64()?,
        enterings_alive: r.u64()?,
        enterings_dead_pre: r.u64()?,
        enterings_dead_post: r.u64()?,
        facts_merged: r.u64()?,
        forced_positives: r.u64()?,
        saturate_count: r.u64()?,
        layers_explored: r.u64()?,
        nogoods_emitted: r.u64()?,
        nogoods_subsumed: r.u64()?,
    };
    let solution_nodes = r.u64()?;
    let exhausted = match r.u8()? {
        0 => false,
        1 => true,
        _ => return Err(EinbError::Malformed("exhausted flag is not 0 or 1")),
    };
    Ok(MonotonicStats {
        base,
        solution_nodes,
        exhausted,
    })
}
