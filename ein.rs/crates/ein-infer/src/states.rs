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
use crate::events::Events;
use crate::monotone::Monotonicity;
use crate::region::{Axes, Candidate, Cost, NodeOut, Prepared, fan_out, pool_of, prepare};
use crate::solve::SolveError;
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
