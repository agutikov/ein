//! The state route — a monotone program's region over its **states**, not its
//! commitments. M1e2
//! [S1e2.1c.2](../../../../plans/m1e2_correctness/p1e2.1c_the_region_search/s1e2.1c.2_the_lines_by_duality.md),
//! the user's *"(B) as the first step of P1e2.1c"* (2026-10-16).
//!
//! Route 1 ([`crate::downset`]) visits every alive commitment. On a monotone
//! program a commitment's state is its closure's (L3), so the alive
//! commitments fall into far fewer states — `zebra.ein` has 270 454 alive
//! commitments and 27 alive states
//! ([`alive_set_reach.md` § 3](../../../../plans/m1e2_correctness/p1e2.1c_the_region_search/alive_set_reach.md#3-states-beside-commitments)).
//! This route visits each alive state once and saturates its one-up
//! extensions, and nothing else.
//!
//! ### The walk
//!
//! A **state** is a closed set: the candidates `S` some fixpoint holds, with
//! `cl(S) = S`. From root's state `S₀ = cl(∅)` the route takes the alive states
//! **by size**, a layer per size, and for each `S` and each candidate `b ∉ S`
//! saturates the commitment `S ∪ {b}` — the node [`crate::region`] computes,
//! with nothing written into root. The fixpoint is dead, or it is the state
//! `cl(S ∪ {b})`, met before or new. Every alive state is met: an alive `T`
//! has a chain `S₀ ⊂ … ⊂ T` of closed sets, each the closure of the one below
//! plus a candidate of `T`, and each alive because it lies under `T` (dead is
//! upward-closed).
//!
//! Three things keep it from saturating what it already knows:
//!
//! - **A dead extension prunes its supersets (L1).** When `S ∪ {b}` is dead,
//!   every later `S' ∪ {b}` with `S ⊆ S'` is dead without a visit. States are
//!   taken by size, so every state under `S'` has been extended before `S'`
//!   is, and the dead extensions it would inherit are all known.
//! - **A commitment that is already a state is not saturated again.** When
//!   `S ∪ {b}` is the closure of a state met before, `cl(S ∪ {b})` is that
//!   state (L3).
//! - **One set, one saturation.** Two states of one size can share an
//!   extension; it is saturated once and recorded under both.
//!
//! ### What it checks of the theorem on the way
//!
//! The route rests on L1–L3, so it declines ([`Declined::Contradicted`]) when
//! a node it computes contradicts them: a state of `R` with an alive, owing
//! extension (discharged is not upward-closed), a fixpoint that does not hold
//! its parent's candidates (a fork only adds), or two states with one closure
//! (L3). The oracle holds the route on every monotone cube it enumerates
//! (`ein-infer/tests/region.rs`).
//!
//! ### Where N and E come from — a decision taken without a ruling
//!
//! [`describe`] reads both lines, `N` and `E`'s alive members off the states
//! this route met, and enumerates neither separately:
//!
//! - **the maximal line** is the states of `R` whose every one-up extension
//!   is dead (L4), and the maximal alive states are the same test without
//!   *of `R`*;
//! - **`N`** is the minimal transversals of the maximal alive states'
//!   complements: the duality of Q-M1e2.7 read from the alive side, since a
//!   set is dead exactly when no maximal alive set holds it (L1);
//! - **`E`'s alive members**, the minimal line, are the minimal sets among the
//!   minimal generators of `R`'s states.
//!
//! The alternative is a MARCO-style enumeration of `N` and the maximal alive
//! sets that never meets the alive set, which saturates at least once per
//! member of each (Liffiton et al., *Constraints* 2016). Describe cannot use it
//! instead of this route, because `k_s`, `k_c` and the merged states need
//! every state of `R`. Beside this route it would only add saturations, and
//! read here, `N` and `E` cost none. The argument holds while describe needs
//! the states, which is what *describe* asks (`solution_semantics.md` § 2.3),
//! and while this route meets every alive state, which
//! `ein-infer/tests/region.rs` holds to the oracle. A mode that asks for a line
//! alone is S1e2.1c.4's, and that is where an enumeration without the states
//! can win.
//!
//! ### Where it stops
//!
//! At the budget route 1 uses, facts beyond root, checked before each layer
//! against the layer's extensions times the facts per node so far. A stopped
//! run has extended every state of size up to its last layer's, and nothing
//! past it; it then saturates ⊤ once, as route 1 does.

use std::time::{Duration, Instant};

use ein_core::{FactId, Kb, Terms};
use ein_ir::Ast;
use rustc_hash::FxHashMap;

use crate::downset::{Declined, Layer, Options};
use crate::error::SolveError;
use crate::events::Events;
use crate::monotone::Monotonicity;
use crate::region::{Axes, Candidate, Cost, NodeOut, Prepared, fan_out, pool_of, prepare};
use crate::transversal::Bits;

/// Nodes per fan-out, as route 1's.
const BATCH: usize = 1024;

/// One alive state.
#[derive(Clone, Debug)]
pub struct State {
    /// The candidates its fixpoint holds, ascending — the state itself.
    pub closure: Box<[u32]>,
    pub bits: Bits,
    /// The commitment whose saturation first met it: `∅` for root's state.
    pub from: Box<[u32]>,
    /// The obligation instances its fixpoint leaves undischarged.
    pub owes: u32,
    pub owed: Vec<String>,
    /// Facts beyond root's that its fixpoint holds.
    pub facts: u64,
    /// The alive states one candidate up — `cl(S ∪ {b})` for each `b` whose
    /// extension is alive — each once, in the order met.
    pub children: Vec<u32>,
    /// Its extensions were taken: [`Self::children`] is complete.
    pub extended: bool,
    /// The distinct commitments the route saturated or read that reached it
    /// — a partial run's count of its commitments met.
    pub met: u64,
    /// Its closure was read as an extension's (L3), and counted in
    /// [`Self::met`].
    pub closure_met: bool,
}

impl State {
    pub fn in_r(&self) -> bool {
        self.owes == 0
    }

    /// Extended, and every one-up extension dead: a maximal alive set.
    pub fn maximal(&self) -> bool {
        self.extended && self.children.is_empty()
    }
}

/// A dead node the route saturated: root, or an extension `S ∪ {b}`.
#[derive(Clone, Debug)]
pub struct Dead {
    pub set: Box<[u32]>,
    pub owes: u32,
    pub core: Vec<String>,
    pub refuted_by: Vec<String>,
    pub sources: Vec<String>,
}

/// Root's node.
#[derive(Clone, Debug, Default)]
pub struct Root {
    pub dead: bool,
    pub owes: u32,
    pub owed: Vec<String>,
    pub core: Vec<String>,
    pub refuted_by: Vec<String>,
    pub sources: Vec<String>,
}

/// ⊤, every candidate, saturated once when the budget stopped the route.
#[derive(Clone, Debug)]
pub struct Top {
    pub dead: bool,
    pub owes: u32,
    pub facts: u64,
}

/// What the route did.
#[derive(Clone, Debug)]
pub struct StateRoute {
    pub candidates: Vec<Candidate>,
    pub monotonicity: Monotonicity,
    pub root: Root,
    /// Every alive state met, in the order met. Root's is the first.
    pub states: Vec<State>,
    /// Every dead node saturated, in the order saturated.
    pub dead: Vec<Dead>,
    /// Per layer: root's, then one per state size extended. `visited` counts
    /// the saturations, so an extension pruned (L1) or already a state (L3)
    /// is not in it.
    pub layers: Vec<Layer>,
    /// The state size each layer extended — `None` for root's.
    pub layer_size: Vec<Option<usize>>,
    pub layer_wall: Vec<Duration>,
    /// Extensions known dead without a visit (L1), summed.
    pub pruned: u64,
    /// Extensions that were a state's closure already (L3), summed.
    pub known: u64,
    /// `None` when every alive state was extended.
    pub declined: Option<Declined>,
    /// The largest state size extended — every alive state of at most this
    /// many candidates is in [`Self::states`] and extended.
    pub complete_up_to: Option<usize>,
    /// On a run its budget stopped: the state size it did not extend.
    pub stopped_size: Option<usize>,
    /// Every commitment whose state the run knows, with that state: root,
    /// each alive extension saturated, and each state's closure (L3). What a
    /// partial run certifies about the commitments it met is read off it.
    pub reached: FxHashMap<Box<[u32]>, u32>,
    pub top: Option<Top>,
    pub cost: Cost,
}

impl StateRoute {
    pub fn n(&self) -> usize {
        self.candidates.len()
    }

    /// Fixpoints computed: root, the extensions, and ⊤ when it was.
    pub fn saturations(&self) -> u64 {
        self.layers.iter().map(|l| l.visited).sum::<u64>() + u64::from(self.top.is_some())
    }

    /// Facts beyond root, summed over every fixpoint computed.
    pub fn facts(&self) -> u64 {
        self.layers.iter().map(|l| l.facts).sum::<u64>() + self.top.as_ref().map_or(0, |t| t.facts)
    }

    /// `{a, b}` or `∅`, in candidate order.
    pub fn name(&self, set: &[u32]) -> String {
        if set.is_empty() {
            return "∅".to_string();
        }
        let parts: Vec<&str> = set
            .iter()
            .map(|&b| self.candidates[b as usize].text.as_str())
            .collect();
        format!("{{{}}}", parts.join(", "))
    }
}

/// The state route on `root`, a **freshly loaded, unsaturated** program
/// nothing else will use, as [`crate::region::compute`] asks.
pub fn compute(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
) -> Result<StateRoute, SolveError> {
    let t_root = Instant::now();
    let prepared = prepare(root, terms, ast)?;
    compute_prepared(root, terms, ast, prepared, t_root, opts, &mut Events::off())
}

/// `set ∪ {b}`, ascending, for `b ∉ set`.
fn with(set: &[u32], b: u32) -> Box<[u32]> {
    let mut s = set.to_vec();
    let at = s.partition_point(|&x| x < b);
    s.insert(at, b);
    s.into_boxed_slice()
}

/// [`compute`] after [`prepare`], for [`crate::explore`]: `events` hears every
/// node saturated, in order, and one `layer` line per layer.
pub(crate) fn compute_prepared(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    prepared: Prepared,
    t_root: Instant,
    opts: &Options,
    events: &mut Events,
) -> Result<StateRoute, SolveError> {
    let Prepared {
        snapshot,
        root_dead,
        candidates,
        hypothesised,
        monotonicity,
        memo,
        plans: _,
        root: _,
    } = prepared;
    let n = candidates.len();
    let mut out = StateRoute {
        candidates,
        monotonicity,
        root: Root {
            dead: root_dead,
            ..Root::default()
        },
        states: Vec::new(),
        dead: Vec::new(),
        layers: Vec::new(),
        layer_size: Vec::new(),
        layer_wall: Vec::new(),
        pruned: 0,
        known: 0,
        declined: None,
        complete_up_to: None,
        stopped_size: None,
        reached: FxHashMap::default(),
        top: None,
        cost: Cost::default(),
    };
    if !out.monotonicity.is_monotone() {
        out.declined = Some(Declined::NotMonotone);
        out.cost.root = t_root.elapsed();
        return Ok(out);
    }
    let axes = Axes::new(root, &out.candidates, hypothesised, false).narrating(events.narration());
    let facts: Vec<FactId> = out.candidates.iter().map(|c| c.fact).collect();
    let root = root.sealed();
    out.cost.root = t_root.elapsed();

    let t_nodes = Instant::now();
    let pool = pool_of(opts.jobs);
    // A state by its fixpoint, and by its closure: on a monotone program the
    // two are one key (L3), and a second state on one closure declines.
    let mut by_state: FxHashMap<Box<[FactId]>, u32> = FxHashMap::default();
    let mut by_closure: FxHashMap<Box<[u32]>, u32> = FxHashMap::default();
    // Per candidate `b`, the states `S` whose extension `S ∪ {b}` is dead.
    let mut dead_under: Vec<Vec<Bits>> = vec![Vec::new(); n];
    // States met and not yet extended, by size.
    let mut pending: std::collections::BTreeMap<usize, Vec<u32>> = Default::default();

    let narrate = |events: &mut Events, cands: &[Candidate], o: &mut NodeOut, set: &[u32]| {
        if events.on() || o.events.is_some() || events.watching() {
            let text = |bits: &[u32]| {
                let mut c: Vec<String> = bits
                    .iter()
                    .map(|&b| cands[b as usize].text.clone())
                    .collect();
                c.sort();
                c
            };
            let closure = if events.watching() {
                text(&o.closure)
            } else {
                Vec::new()
            };
            crate::region::narrate_node(events, o, text(set), || closure);
        }
    };

    // Layer 0: root.
    {
        let t_layer = Instant::now();
        let mut o = fan_out(
            pool.as_ref(),
            root,
            &snapshot,
            terms,
            ast,
            &memo,
            &axes,
            &[Vec::new()],
        )?
        .into_iter()
        .next()
        .expect("one node");
        narrate(events, &out.candidates, &mut o, &[]);
        let mut stats = Layer {
            visited: 1,
            facts: o.state.len() as u64,
            ..Layer::default()
        };
        out.root = Root {
            dead: o.dead,
            owes: o.owes,
            owed: o.node.owed.clone(),
            core: o.node.core.clone(),
            refuted_by: o.node.refuted_by.clone(),
            sources: o.node.sources.clone(),
        };
        if o.dead {
            stats.dead = 1;
            out.dead.push(Dead {
                set: Box::default(),
                owes: o.owes,
                core: o.node.core,
                refuted_by: o.node.refuted_by,
                sources: o.node.sources,
            });
        } else {
            if o.owes > 0 {
                stats.owing = 1;
            } else {
                stats.r = 1;
            }
            let closure: Box<[u32]> = o.closure.into_boxed_slice();
            by_state.insert(o.state, 0);
            by_closure.insert(closure.clone(), 0);
            out.reached.insert(Box::default(), 0);
            out.reached.insert(closure.clone(), 0);
            pending.entry(closure.len()).or_default().push(0);
            out.states.push(State {
                bits: Bits::of(n, &closure),
                closure,
                from: Box::default(),
                owes: o.owes,
                owed: o.node.owed,
                facts: stats.facts,
                children: Vec::new(),
                extended: false,
                met: 1,
                closure_met: false,
            });
        }
        events.watch_step(&crate::events::Stepped {
            layer: Some(0),
            visited: stats.visited,
            dead: stats.dead,
            owing: stats.owing,
            r: stats.r,
            facts: stats.facts,
        });
        emit_layer(events, 0, &stats);
        out.layers.push(stats);
        out.layer_size.push(None);
        out.layer_wall.push(t_layer.elapsed());
    }

    while let Some((&size, _)) = pending.iter().next() {
        if opts.deadline.is_some_and(|d| Instant::now() >= d) {
            return Err(SolveError::Deadline);
        }
        let mut layer_states = pending.remove(&size).expect("a size");
        layer_states.sort_by(|&a, &b| {
            out.states[a as usize]
                .closure
                .cmp(&out.states[b as usize].closure)
        });
        let layer = out.layers.len();

        // The layer's extensions: pruned by L1, read off a known state by
        // L3, or saturated once per distinct set.
        let mut fresh: Vec<Box<[u32]>> = Vec::new();
        let mut owners: Vec<Vec<(u32, u32)>> = Vec::new();
        let mut at: FxHashMap<Box<[u32]>, usize> = FxHashMap::default();
        let mut known: Vec<(u32, u32)> = Vec::new();
        let mut pruned = 0u64;
        for &s in &layer_states {
            let st = &out.states[s as usize];
            for b in 0..n as u32 {
                if st.bits.contains(b) {
                    continue;
                }
                if dead_under[b as usize].iter().any(|d| d.is_subset(&st.bits)) {
                    pruned += 1;
                    continue;
                }
                let set = with(&st.closure, b);
                if let Some(&t) = by_closure.get(&set) {
                    known.push((s, t));
                    continue;
                }
                match at.get(&set) {
                    Some(&i) => owners[i].push((s, b)),
                    None => {
                        at.insert(set.clone(), fresh.len());
                        fresh.push(set);
                        owners.push(vec![(s, b)]);
                    }
                }
            }
        }

        let visited: u64 = out.layers.iter().map(|l| l.visited).sum();
        let spent: u64 = out.layers.iter().map(|l| l.facts).sum();
        let per_node = (spent / visited.max(1)).max(1);
        let next = fresh.len() as u64;
        let projected = next.saturating_mul(per_node);
        if visited.saturating_add(next) > opts.max_visits
            || spent.saturating_add(projected) > opts.budget
        {
            out.declined = Some(Declined::Budget {
                layer,
                next,
                spent,
                projected,
            });
            pending.insert(size, layer_states);
            out.stopped_size = Some(size);
            break;
        }

        let t_layer = Instant::now();
        let mut stats = Layer::default();
        let mut read: Vec<u32> = known.iter().map(|&(_, t)| t).collect();
        read.sort_unstable();
        read.dedup();
        for t in read {
            // The closure itself, read and not saturated; once.
            if !out.states[t as usize].closure_met {
                out.states[t as usize].closure_met = true;
                out.states[t as usize].met += 1;
            }
        }
        for &(s, t) in &known {
            if out.states[s as usize].in_r() && !out.states[t as usize].in_r() {
                out.declined = Some(Declined::Contradicted { from: s, to: t });
            }
            push_child(&mut out.states[s as usize], t);
        }
        for (chunk, chunk_owners) in fresh.chunks(BATCH).zip(owners.chunks(BATCH)) {
            let sets: Vec<Vec<FactId>> = chunk
                .iter()
                .map(|s| s.iter().map(|&b| facts[b as usize]).collect())
                .collect();
            let outs = fan_out(
                pool.as_ref(),
                root,
                &snapshot,
                terms,
                ast,
                &memo,
                &axes,
                &sets,
            )?;
            for ((mut o, set), pairs) in outs.into_iter().zip(chunk).zip(chunk_owners) {
                narrate(events, &out.candidates, &mut o, set);
                stats.visited += 1;
                stats.facts += o.state.len() as u64;
                if o.dead {
                    stats.dead += 1;
                    for &(s, b) in pairs {
                        dead_under[b as usize].push(out.states[s as usize].bits.clone());
                    }
                    out.dead.push(Dead {
                        set: set.clone(),
                        owes: o.owes,
                        core: o.node.core,
                        refuted_by: o.node.refuted_by,
                        sources: o.node.sources,
                    });
                    continue;
                }
                if o.owes > 0 {
                    stats.owing += 1;
                } else {
                    stats.r += 1;
                }
                let node_facts = o.state.len() as u64;
                let closure: Box<[u32]> = o.closure.into_boxed_slice();
                let bits = Bits::of(n, &closure);
                // A fork only adds: the fixpoint holds its commitment.
                if !Bits::of(n, set).is_subset(&bits) {
                    let s = pairs[0].0;
                    out.declined = Some(Declined::Contradicted { from: s, to: s });
                    break;
                }
                let t = match by_state.get(&o.state) {
                    Some(&t) => {
                        if out.states[t as usize].closure != closure {
                            out.declined = Some(Declined::Contradicted { from: t, to: t });
                            break;
                        }
                        t
                    }
                    None => {
                        let t = out.states.len() as u32;
                        if let Some(&u) = by_closure.get(&closure) {
                            // Two fixpoints on one closure: L3 fails.
                            out.declined = Some(Declined::Contradicted { from: u, to: t });
                            break;
                        }
                        by_state.insert(o.state, t);
                        by_closure.insert(closure.clone(), t);
                        out.reached.insert(closure.clone(), t);
                        pending.entry(closure.len()).or_default().push(t);
                        out.states.push(State {
                            bits,
                            closure,
                            from: set.clone(),
                            owes: o.owes,
                            owed: o.node.owed,
                            facts: node_facts,
                            children: Vec::new(),
                            extended: false,
                            met: 0,
                            closure_met: false,
                        });
                        t
                    }
                };
                out.states[t as usize].met += 1;
                out.reached.insert(set.clone(), t);
                for &(s, _) in pairs {
                    // Discharged is upward-closed: a state of R has no owing
                    // state above it.
                    if out.states[s as usize].in_r() && !out.states[t as usize].in_r() {
                        out.declined = Some(Declined::Contradicted { from: s, to: t });
                    }
                    push_child(&mut out.states[s as usize], t);
                }
            }
            if out.declined.is_some() {
                break;
            }
        }
        for &s in &layer_states {
            out.states[s as usize].extended = true;
        }
        out.pruned += pruned;
        out.known += known.len() as u64;
        events.watch_step(&crate::events::Stepped {
            layer: Some(layer),
            visited: stats.visited,
            dead: stats.dead,
            owing: stats.owing,
            r: stats.r,
            facts: stats.facts,
        });
        emit_layer(events, layer, &stats);
        out.layers.push(stats);
        out.layer_size.push(Some(size));
        out.layer_wall.push(t_layer.elapsed());
        if out.declined.is_some() {
            break;
        }
        out.complete_up_to = Some(size);
    }
    if out.declined.is_none() && out.root.dead {
        out.complete_up_to = None;
    }

    // ⊤ once, when the budget stopped the route and ⊤ is not a state met.
    if matches!(out.declined, Some(Declined::Budget { .. }))
        && n > 0
        && !by_closure.contains_key(&(0..n as u32).collect::<Box<[u32]>>())
    {
        let all: Vec<u32> = (0..n as u32).collect();
        let mut o = fan_out(
            pool.as_ref(),
            root,
            &snapshot,
            terms,
            ast,
            &memo,
            &axes,
            std::slice::from_ref(&facts),
        )?
        .into_iter()
        .next()
        .expect("one node");
        narrate(events, &out.candidates, &mut o, &all);
        out.top = Some(Top {
            dead: o.dead,
            owes: o.owes,
            facts: o.state.len() as u64,
        });
    }
    out.cost.nodes = t_nodes.elapsed();
    Ok(out)
}

fn push_child(s: &mut State, t: u32) {
    if !s.children.contains(&t) {
        s.children.push(t);
    }
}

fn emit_layer(events: &mut Events, layer: usize, stats: &Layer) {
    events.emit("layer", |l| {
        l.num("layer", layer as i64);
        l.num("visited", stats.visited as i64);
        l.num("dead", stats.dead as i64);
        l.num("owing", stats.owing as i64);
        l.num("r", stats.r as i64);
        l.num("facts", stats.facts as i64);
    });
}

// ── R, read off the states ──────────────────────────────────────────────

/// A sub-cube `[a … b]`: its bottom, a minimal commitment, and its top, a
/// maximal state's closure.
pub type SubCube = (Box<[u32]>, Box<[u32]>);

/// Branches a state's count may take ([`crate::transversal::count_hitting`])
/// before it is counted from the states below it instead.
const COUNT_LIMIT: u64 = 1 << 16;

/// `R`, read off the states the route met: all of it when the route extended
/// every alive state, and on a partial run what it certified.
#[derive(Clone, Debug, Default)]
pub struct Description {
    /// Every alive state was extended: what follows is all of `R`.
    pub exhausted: bool,
    /// The states of `R` met, as indices into [`StateRoute::states`], in the
    /// order met.
    pub r: Vec<u32>,
    /// `|R|` in commitments. On a partial run: every commitment of an
    /// extended state of `R`, and those met of the rest — a lower bound,
    /// unless [`Self::cube`].
    pub k_c: u64,
    /// Per state, the commitments that reach it, where the run counted them
    /// exactly: every extended state (L3).
    pub commitments: Vec<Option<u64>>,
    /// Per state of `R` that was extended, its minimal generators, by size and
    /// then by candidates. Empty where the run does not know them.
    pub generators: Vec<Vec<Box<[u32]>>>,
    /// The minimal line as commitments, by size and then by candidates, each
    /// with the state it reaches. On a partial run, those certified.
    pub minimal: Vec<(Box<[u32]>, u32)>,
    /// The states of `R` on the maximal line, by size and then by candidates:
    /// every one-up extension dead (L4).
    pub maximal: Vec<u32>,
    /// `N`, by size and then by candidates — `None` where the run does not
    /// know it.
    pub nogoods: Option<Vec<Box<[u32]>>>,
    /// On a partial run that does not know `N`: the members of it the run
    /// met and certified, each a dead set it saturated whose every one-down
    /// subset is a commitment it knows alive.
    pub nogoods_met: Vec<Box<[u32]>>,
    /// The components' sizes in commitments, in the order of their least
    /// minimal commitment — `None` on a partial run.
    pub components: Option<Vec<u64>>,
    /// The maximal sub-cubes `[a … b]`, `a` on the minimal line and `b` on the
    /// maximal one — `None` on a partial run.
    pub subcubes: Option<Vec<SubCube>>,
    /// When `R` is empty and root is alive: the maximal alive states, every
    /// one owing, by size and then by candidates.
    pub frontier: Vec<u32>,
    /// ∅ and ⊤ are both in `R`, so `R` is the whole cube (L1, L2) — decided by
    /// two saturations even on a partial run.
    pub cube: bool,
}

/// `R` from what `s` met. `s` must not have declined for any reason but its
/// budget.
pub fn describe(s: &StateRoute) -> Result<Description, SolveError> {
    let n = s.n();
    let ns = s.states.len();
    let exhausted = s.declined.is_none();
    let mut parents: Vec<Vec<u32>> = vec![Vec::new(); ns];
    for (i, st) in s.states.iter().enumerate() {
        for &c in &st.children {
            parents[c as usize].push(i as u32);
        }
    }
    let edges_of = |i: usize| -> Vec<Bits> {
        let st = &s.states[i];
        parents[i]
            .iter()
            .map(|&p| st.bits.minus(&s.states[p as usize].bits))
            .collect()
    };

    // How many commitments reach each extended state: the sets under it that
    // lie under no state below it (L3).
    let mut f: Vec<Option<u128>> = vec![None; ns];
    let mut order: Vec<u32> = (0..ns as u32)
        .filter(|&i| s.states[i as usize].extended)
        .collect();
    order.sort_by_key(|&i| s.states[i as usize].closure.len());
    let overflow = || {
        SolveError::Unsupported(
            "a state's commitments pass 2^128: the state route cannot count them".to_string(),
        )
    };
    let r_wanted: Vec<bool> = s.states.iter().map(|st| st.extended && st.in_r()).collect();
    // A state whose every edge down is one candidate is the common case
    // where commitments do not collapse (`syntax/arg-kinds`: every one of its
    // 2¹⁹ states): its one minimal generator is those candidates, and its
    // commitments are the subsets of S that hold them. No search is needed.
    let mut forced: Vec<Option<Bits>> = vec![None; ns];
    for &i in &order {
        let i = i as usize;
        if !r_wanted[i] {
            continue;
        }
        let edges = crate::transversal::minimal_edges(&edges_of(i));
        if edges.iter().all(|e| e.len() == 1) {
            let mut g = Bits::empty(n);
            for e in &edges {
                g = g.or(e);
            }
            let free = s.states[i].closure.len() as u32 - g.len();
            f[i] = Some(crate::transversal::pow2(free).ok_or_else(overflow)?);
            forced[i] = Some(g);
            continue;
        }
        if f[i].is_none() {
            f[i] = Some(count_of(s, &parents, &edges_of, &mut f, i).ok_or_else(overflow)?);
        }
    }
    let commitments: Vec<Option<u64>> = f
        .iter()
        .map(|x| x.and_then(|v| u64::try_from(v).ok()))
        .collect();

    let r: Vec<u32> = (0..ns as u32)
        .filter(|&i| s.states[i as usize].in_r())
        .collect();
    let mut generators: Vec<Vec<Box<[u32]>>> = vec![Vec::new(); ns];
    for &i in &r {
        let i = i as usize;
        if !s.states[i].extended {
            continue;
        }
        generators[i] = match forced[i].take() {
            Some(g) => vec![g.to_set()],
            None => crate::transversal::minimal_transversals(n, &edges_of(i))
                .iter()
                .map(Bits::to_set)
                .collect(),
        };
    }

    // The minimal line: ∅ when root's state is in R; otherwise the minimal
    // sets among the generators of R's extended states.
    let root_in_r = !s.root.dead && s.root.owes == 0;
    // A partial run knows the generators of the states it extended. Of a
    // state it met and did not extend, it certifies a commitment it met when
    // every one-down subset is a commitment whose state it knows and that
    // state owes: the commitment is then minimal in R (L2), and so a minimal
    // generator of its state.
    let mut met: Vec<(&Box<[u32]>, u32)> = if exhausted {
        Vec::new()
    } else {
        // determinism-ok: sorted by size and then candidates below.
        s.reached.iter().map(|(c, &t)| (c, t)).collect()
    };
    met.sort_by(|a, b| (a.0.len(), a.0).cmp(&(b.0.len(), b.0)));
    let mut probe: Vec<u32> = Vec::new();
    for (c, t) in met {
        let st = &s.states[t as usize];
        if st.extended || !st.in_r() {
            continue;
        }
        let certified = (0..c.len()).all(|k| {
            crate::downset::without(c, k, &mut probe);
            s.reached
                .get(&probe[..])
                .is_some_and(|&u| !s.states[u as usize].in_r())
        });
        if certified {
            generators[t as usize].push(c.clone());
        }
    }
    let minimal: Vec<(Box<[u32]>, u32)> = if root_in_r {
        vec![(Box::default(), 0)]
    } else {
        let mut all: Vec<(Box<[u32]>, u32)> = r
            .iter()
            .flat_map(|&i| generators[i as usize].iter().map(move |g| (g.clone(), i)))
            .collect();
        all.sort_by(|a, b| (a.0.len(), &a.0).cmp(&(b.0.len(), &b.0)));
        let mut kept: Vec<(Box<[u32]>, u32)> = Vec::new();
        let mut kept_bits: Vec<Bits> = Vec::new();
        for (g, i) in all {
            let gb = Bits::of(n, &g);
            if !kept_bits.iter().any(|k| k.is_subset(&gb)) {
                kept_bits.push(gb);
                kept.push((g, i));
            }
        }
        kept
    };
    let by_size = |v: &mut Vec<u32>| {
        v.sort_by(|&a, &b| {
            let (x, y) = (&s.states[a as usize].closure, &s.states[b as usize].closure);
            (x.len(), x).cmp(&(y.len(), y))
        })
    };
    let mut maximal: Vec<u32> = r
        .iter()
        .copied()
        .filter(|&i| s.states[i as usize].maximal())
        .collect();
    by_size(&mut maximal);

    let top_alive =
        s.top.as_ref().is_some_and(|t| !t.dead) || s.states.iter().any(|st| st.closure.len() == n);
    let top_in_r = s.top.as_ref().is_some_and(|t| !t.dead && t.owes == 0)
        || s.states.iter().any(|st| st.closure.len() == n && st.in_r());
    let cube = root_in_r && top_in_r;

    // N, by duality: the minimal transversals of the maximal alive states'
    // complements — a set is dead exactly when no maximal alive state holds
    // it (L1). Where N comes from is a decision taken without a ruling; the
    // module's § "Where N and E come from" gives its argument.
    let nogoods = if exhausted {
        let full = Bits::full(n);
        let edges: Vec<Bits> = s
            .states
            .iter()
            .filter(|st| st.maximal())
            .map(|st| full.minus(&st.bits))
            .collect();
        Some(
            crate::transversal::minimal_transversals(n, &edges)
                .iter()
                .map(Bits::to_set)
                .collect(),
        )
    } else if top_alive {
        // ⊤ alive: nothing is dead (L1).
        Some(Vec::new())
    } else {
        None
    };
    let mut nogoods_met: Vec<Box<[u32]>> = Vec::new();
    if nogoods.is_none() {
        for x in &s.dead {
            let alive_below = x.set.is_empty()
                || (0..x.set.len()).all(|k| {
                    crate::downset::without(&x.set, k, &mut probe);
                    s.reached.contains_key(&probe[..])
                });
            if alive_below {
                nogoods_met.push(x.set.clone());
            }
        }
        nogoods_met.sort_by(|a, b| (a.len(), a).cmp(&(b.len(), b)));
    }

    let k_c: u64 = if exhausted {
        let mut sum = 0u128;
        for &i in &r {
            sum = sum
                .checked_add(f[i as usize].expect("counted"))
                .ok_or_else(overflow)?;
        }
        u64::try_from(sum).map_err(|_| overflow())?
    } else if cube {
        u64::try_from(crate::transversal::pow2(n as u32).ok_or_else(overflow)?)
            .map_err(|_| overflow())?
    } else {
        r.iter()
            .map(|&i| {
                let st = &s.states[i as usize];
                commitments[i as usize].unwrap_or(st.met)
            })
            .sum()
    };

    let (components, subcubes) = if exhausted {
        // Union-find over R's states, joined by their one-up edges inside R.
        let mut parent: Vec<u32> = (0..ns as u32).collect();
        fn find(p: &mut [u32], x: u32) -> u32 {
            let mut root = x;
            while p[root as usize] != root {
                root = p[root as usize];
            }
            let mut y = x;
            while p[y as usize] != root {
                let next = p[y as usize];
                p[y as usize] = root;
                y = next;
            }
            root
        }
        for &i in &r {
            for &c in &s.states[i as usize].children {
                if s.states[c as usize].in_r() {
                    let (a, b) = (find(&mut parent, i), find(&mut parent, c));
                    if a != b {
                        parent[a.max(b) as usize] = a.min(b);
                    }
                }
            }
        }
        let mut size: FxHashMap<u32, u128> = FxHashMap::default();
        for &i in &r {
            let c = find(&mut parent, i);
            *size.entry(c).or_default() += f[i as usize].expect("counted");
        }
        let mut comps: Vec<u64> = Vec::new();
        let mut seen: Vec<u32> = Vec::new();
        for (_, i) in &minimal {
            let c = find(&mut parent, *i);
            if !seen.contains(&c) {
                seen.push(c);
                comps.push(u64::try_from(size[&c]).map_err(|_| overflow())?);
            }
        }
        let mut cubes: Vec<SubCube> = Vec::new();
        for (a, _) in &minimal {
            let ab = Bits::of(n, a);
            for &m in &maximal {
                let st = &s.states[m as usize];
                if ab.is_subset(&st.bits) {
                    cubes.push((a.clone(), st.closure.clone()));
                }
            }
        }
        (Some(comps), Some(cubes))
    } else if cube {
        (
            Some(vec![k_c]),
            Some(vec![(Box::default(), (0..n as u32).collect())]),
        )
    } else {
        (None, None)
    };

    let mut frontier: Vec<u32> = if exhausted && r.is_empty() && !s.root.dead {
        (0..ns as u32)
            .filter(|&i| s.states[i as usize].maximal())
            .collect()
    } else {
        Vec::new()
    };
    by_size(&mut frontier);

    Ok(Description {
        exhausted,
        r,
        k_c,
        commitments,
        generators,
        minimal,
        maximal,
        nogoods,
        nogoods_met,
        components,
        subcubes,
        frontier,
        cube,
    })
}

/// The commitments that reach state `i`: the subsets of its closure that meet
/// every complement against a state one edge below it, counted by branching;
/// past [`COUNT_LIMIT`], `2^|S|` less every state below it, each counted the
/// same way. `None` past `u128`.
fn count_of(
    s: &StateRoute,
    parents: &[Vec<u32>],
    edges_of: &dyn Fn(usize) -> Vec<Bits>,
    f: &mut [Option<u128>],
    i: usize,
) -> Option<u128> {
    if let Some(v) = f[i] {
        return Some(v);
    }
    let st = &s.states[i];
    if let Some(v) = crate::transversal::count_hitting(&st.bits, &edges_of(i), COUNT_LIMIT) {
        f[i] = Some(v);
        return Some(v);
    }
    // Every closed set under S is alive, extended, and reached through the
    // one-up edges: S's down-set in the state graph.
    let mut below: Vec<u32> = Vec::new();
    let mut seen = vec![false; s.states.len()];
    let mut stack: Vec<u32> = parents[i].clone();
    while let Some(t) = stack.pop() {
        if std::mem::replace(&mut seen[t as usize], true) {
            continue;
        }
        below.push(t);
        stack.extend(parents[t as usize].iter().copied());
    }
    below.sort_by_key(|&t| s.states[t as usize].closure.len());
    let mut v = crate::transversal::pow2(st.closure.len() as u32)?;
    for t in below {
        let ft = count_of(s, parents, edges_of, f, t as usize)?;
        v = v.checked_sub(ft)?;
    }
    f[i] = Some(v);
    Some(v)
}
