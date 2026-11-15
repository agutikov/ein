//! The structure of the hypothesis set `H`, read from the program's own
//! lattice, with nobody declaring it.
//!
//! Every object here is a statement about nodes [`crate::region`] defines:
//! root's fixpoint with a commitment written in and saturated, by the node
//! function every route and the oracle share, reached through
//! [`crate::explore`]'s node cache (`Space::eval`). So each is true of the
//! lattice `R` is a region of, and none is a second semantics.
//!
//! ### The pair layer
//!
//! [`PairLayer`] is what the nodes of one candidate and of two decide at
//! root, over `H` as root builds it:
//!
//! | object | read from | definition |
//! |---|---|---|
//! | **dead alone** | the node `{a}` | it is dead |
//! | **implies** | the node `{a}`, alive | `b` is in its closure: committing `a` brings `b` |
//! | **equivalent** | two alive single nodes | one closure |
//! | **excludes** | the node `{a, b}`, both alive alone, neither implying the other | it is dead |
//! | **interacts** | the same node, alive | its fixpoint holds a fact that neither `{a}`'s nor `{b}`'s does |
//!
//! The excluded pairs are the members of `N`, the minimal dead sets, that
//! hold two candidates: a dead pair whose two members are each alive alone,
//! over an alive root. `ein-infer/tests/pair_layer.rs` holds the two to each
//! other on every corpus program whose describe exhausts, where `N` is read
//! off the maximal alive states by duality and shares no code with this
//! module past the node.
//!
//! ### What needs no saturation
//!
//! The singles are read first, all of them. On a program the static check
//! calls monotone two licences then decide a pair without its node:
//!
//! - **L1, dead is upward-closed.** A pair with a member dead alone is dead.
//!   It is not an exclusion: its dead member says all there is to say.
//! - **L3, a commitment's state is its closure's.** A pair one of whose
//!   members implies the other is the node of the one that implies: alive,
//!   and with no fact of its own.
//!
//! A pair is skipped by one of the two and by nothing else: not by the
//! relation of its members, not by their arguments. Under a dead root every
//! node is dead (L0, on any program), so every candidate is dead alone and
//! no node is read.
//!
//! ### A program the static check flags
//!
//! There neither licence holds, so **every pair is saturated**, and the layer
//! is still a set of true statements about root's nodes; nothing above root
//! follows from them ([`PairLayer::domain`]). Two more lists say where the
//! licences would have read a node wrongly: [`PairLayer::against_l1`], a pair
//! alive over a member dead alone, and [`PairLayer::against_l3`], a pair one
//! of whose members implies the other whose node is not that member's.
//!
//! ### What it costs, and where it stops
//!
//! Root's node, `n` singles, and the pairs the licences leave, each one
//! saturation; the budget is the routes', facts beyond root summed over the
//! fixpoints computed, checked before each step by the cache's own
//! projection. A stopped layer says what it read: the lists hold the pairs
//! saturated, [`Pairs::left`] counts the rest, and the singles not read are
//! [`PairLayer::singles_left`]. Nothing here is written anywhere a route
//! reads: the layer reads nodes and moves no answer.

use std::collections::BTreeMap;
use std::time::Instant;

use ein_core::{FactId, Kb, Terms};
use ein_ir::Ast;

use crate::downset;
use crate::error::SolveError;
use crate::events::Events;
use crate::explore::{self, Ask, Licence, Mode, Seen, Space, Stopped, Which};
use crate::region::{self, Candidate, Cores};

/// The largest step, as the routes'.
const BATCH: u64 = 1024;

/// What [`PairLayer::domain`] says of a program the static check flags.
pub const FLAGGED: &str = "the static check flags this program: every statement of the pair layer \
                           is true of root's nodes, and nothing above root follows from them";

/// How the layer is asked.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Facts beyond root, summed over every fixpoint the layer computes: the
    /// routes' budget, in the routes' unit.
    pub budget: u64,
    /// Threads the node function fans out over. The layer is the same at any
    /// count.
    pub jobs: usize,
    /// Checked before each step, as the budget is.
    pub deadline: Option<Instant>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            budget: downset::DEFAULT_BUDGET,
            jobs: 1,
            deadline: None,
        }
    }
}

/// The pairs of `H`, by what decided each.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pairs {
    /// `n (n − 1) / 2`.
    pub all: u64,
    /// Not saturated: a member is dead alone, so the pair is dead (L1; L0
    /// under a dead root).
    pub dead_member: u64,
    /// Not saturated: one member implies the other, so the pair is that
    /// member's node (L3).
    pub implied: u64,
    /// Read through the node function.
    pub saturated: u64,
    /// Neither licence decides them and the layer stopped before their
    /// nodes.
    pub left: u64,
}

/// The work the layer did, in the read-out's units.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    /// Fixpoints computed: root's node, the singles and the pairs saturated.
    pub saturations: u64,
    /// Facts beyond root's, summed over them.
    pub facts: u64,
}

/// What one candidate and two decide at root. Every candidate is its index
/// in [`Self::candidates`], the universe in rendered-text order, and every
/// pair `(a, b)` has `a < b` unless its field says otherwise; each list is
/// ascending, so the layer does not depend on the interner's ids or on the
/// job count.
#[derive(Clone, Debug)]
pub struct PairLayer {
    pub candidates: Vec<Candidate>,
    /// The static verdict: L1 and L3 hold, and decided the pairs they could.
    pub monotone: bool,
    /// Root's own fixpoint derives `(false)`: every candidate is dead alone
    /// without a visit, and no pair is read.
    pub root_dead: bool,
    /// Singles the layer stopped before. While it is not 0,
    /// [`Self::dead_alone`] holds the dead singles met and every list after
    /// it is empty.
    pub singles_left: u64,
    /// The candidates root's own fixpoint holds: committing one is
    /// committing nothing. They are in no implication and in no equivalence
    /// class below.
    pub at_root: Vec<u32>,
    /// The candidates whose own node is dead.
    pub dead_alone: Vec<u32>,
    /// `(a, b)`, in either order of index: `a` is alive alone and `b`, which
    /// is not `a` and not at root, is in the closure of `{a}`.
    pub implies: Vec<(u32, u32)>,
    /// The classes of two or more candidates alive alone with one closure,
    /// each ascending, in the order of their first members.
    pub equivalent: Vec<Vec<u32>>,
    /// The pairs that exclude: each member alive alone, neither implying the
    /// other, the pair dead.
    pub excluded: Vec<(u32, u32)>,
    /// The pairs that interact: as [`Self::excluded`]'s but alive, with a
    /// fact in the pair's fixpoint that neither member's own holds.
    pub interacting: Vec<(u32, u32)>,
    /// On a flagged program: the pairs alive over a member dead alone.
    pub against_l1: Vec<(u32, u32)>,
    /// On a flagged program: the pairs one of whose members implies the
    /// other and whose node is not that member's state, each with whether
    /// it is dead.
    pub against_l3: Vec<(u32, u32, bool)>,
    pub pairs: Pairs,
    /// The licences that decided a pair without its node: none on a flagged
    /// program.
    pub licences: Vec<Licence>,
    pub work: Work,
    /// Why the layer stopped before its last node, when it did.
    pub stopped: Option<Stopped>,
}

impl PairLayer {
    pub fn n(&self) -> usize {
        self.candidates.len()
    }

    /// Candidate `b`, as the program writes it.
    pub fn text(&self, b: u32) -> &str {
        &self.candidates[b as usize].text
    }

    /// Every single and every pair the licences leave was read.
    pub fn complete(&self) -> bool {
        self.stopped.is_none()
    }

    /// What the layer's statements are about, where that is not the whole
    /// lattice: [`FLAGGED`] on a program the static check flags.
    pub fn domain(&self) -> Option<&'static str> {
        (!self.monotone).then_some(FLAGGED)
    }

    /// How many candidates are in at least one excluded pair.
    pub fn in_an_excluded_pair(&self) -> usize {
        let mut seen = vec![false; self.n()];
        for &(a, b) in &self.excluded {
            seen[a as usize] = true;
            seen[b as usize] = true;
        }
        seen.into_iter().filter(|&x| x).count()
    }
}

/// The pair layer of `root`, a **freshly loaded, unsaturated** program
/// nothing else will use, as [`crate::explore::run`] asks. Root is saturated
/// once and never written after.
pub fn pair_layer(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
) -> Result<PairLayer, SolveError> {
    with_space(root, terms, ast, opts, pair_layer_in)
}

/// `read` over the prepared space of `root`, a freshly loaded program: the
/// routes' node cache, at the layer's budget, keeping no node's record.
fn with_space<R>(
    root: &mut Kb,
    terms: &mut Terms,
    ast: &Ast,
    opts: &Options,
    read: impl FnOnce(&mut Space<'_>) -> Result<R, SolveError>,
) -> Result<R, SolveError> {
    let prepared = region::prepare(root, terms, ast)?;
    let mut events = Events::off();
    let space = explore::Options {
        budget: opts.budget,
        limit: 0,
        jobs: opts.jobs,
        deadline: opts.deadline,
        cores: Cores::READ_OUT,
        ..explore::Options::new(Mode {
            which: Which::All,
            ask: Ask::Describe,
        })
    };
    let mut sp = Space::open(root, terms, ast, prepared, &space, &mut events);
    read(&mut sp)
}

/// One single's node, as the layer keeps it.
struct Single {
    dead: bool,
    /// Its state, as the cache numbers it.
    class: u32,
    closure: Box<[u32]>,
}

impl Single {
    fn holds(&self, b: u32) -> bool {
        self.closure.binary_search(&b).is_ok()
    }
}

/// The next step's size: the largest the budget affords, and one where it
/// affords none, so that the cache's own refusal is the stop reported.
fn step(sp: &Space<'_>, want: u64) -> usize {
    want.min(BATCH).min(sp.affordable().max(1)) as usize
}

/// A fact of `pair` that neither `a` nor `b` holds: each a state, its facts
/// beyond root's ascending.
fn has_own_fact(pair: &[FactId], a: &[FactId], b: &[FactId]) -> bool {
    let (mut i, mut j) = (0, 0);
    pair.iter().any(|f| {
        while i < a.len() && a[i] < *f {
            i += 1;
        }
        while j < b.len() && b[j] < *f {
            j += 1;
        }
        !(i < a.len() && a[i] == *f) && !(j < b.len() && b[j] == *f)
    })
}

/// How the licences read a pair before its node.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Before {
    /// A member is dead alone.
    DeadMember,
    /// One member implies the other.
    Implied,
    /// Neither.
    Open,
}

fn before(singles: &[Single], a: u32, b: u32) -> Before {
    let (x, y) = (&singles[a as usize], &singles[b as usize]);
    if x.dead || y.dead {
        Before::DeadMember
    } else if x.holds(b) || y.holds(a) {
        Before::Implied
    } else {
        Before::Open
    }
}

/// [`pair_layer`] over a program's prepared space, the routes' node cache:
/// a node the space has computed is read and not saturated again, and the
/// layer's nodes are the space's from then on. [`PairLayer::work`] is what
/// the layer added to the space's.
pub(crate) fn pair_layer_in(sp: &mut Space<'_>) -> Result<PairLayer, SolveError> {
    let licensed = sp.is_monotone();
    layer(sp, licensed)
}

/// The layer over `sp`, L1 and L3 deciding the pairs they can when
/// `licensed` and every pair saturated otherwise.
fn layer(sp: &mut Space<'_>, licensed: bool) -> Result<PairLayer, SolveError> {
    let n = sp.n();
    let monotone = sp.is_monotone();
    let (saturations, facts) = sp.work();
    let mut out = PairLayer {
        candidates: sp.candidates().to_vec(),
        monotone,
        root_dead: sp.root_is_dead(),
        singles_left: 0,
        at_root: Vec::new(),
        dead_alone: Vec::new(),
        implies: Vec::new(),
        equivalent: Vec::new(),
        excluded: Vec::new(),
        interacting: Vec::new(),
        against_l1: Vec::new(),
        against_l3: Vec::new(),
        pairs: Pairs {
            all: (n as u64) * (n as u64).saturating_sub(1) / 2,
            ..Pairs::default()
        },
        licences: Vec::new(),
        work: Work::default(),
        stopped: None,
    };
    read(sp, &mut out, licensed)?;
    let (s, f) = sp.work();
    out.work = Work {
        saturations: s - saturations,
        facts: f - facts,
    };
    Ok(out)
}

fn read(sp: &mut Space<'_>, out: &mut PairLayer, licensed: bool) -> Result<(), SolveError> {
    let n = out.n() as u32;
    if out.root_dead {
        // A fork only adds: every node holds root's contradiction.
        out.dead_alone = (0..n).collect();
        out.pairs.dead_member = out.pairs.all;
        out.licences = vec![Licence::ForkOnlyAdds];
        return Ok(());
    }

    // Root's node, for the candidates its own fixpoint holds.
    let root = match sp.eval(&[Box::default()])? {
        Ok(seen) => seen.into_iter().next().expect("one node"),
        Err(stop) => {
            out.stopped = Some(stop);
            out.singles_left = u64::from(n);
            out.pairs.left = out.pairs.all;
            return Ok(());
        }
    };
    out.at_root = root.closure.to_vec();

    // The singles, all of them, before any pair.
    let mut singles: Vec<Single> = Vec::with_capacity(n as usize);
    while (singles.len() as u32) < n {
        let from = singles.len() as u32;
        let take = step(sp, u64::from(n - from)) as u32;
        let sets: Vec<Box<[u32]>> = (from..from + take).map(|b| Box::from([b])).collect();
        match sp.eval(&sets)? {
            Ok(seen) => singles.extend(seen.into_iter().map(|s: Seen| Single {
                dead: s.dead,
                class: s.class,
                closure: s.closure,
            })),
            Err(stop) => {
                out.stopped = Some(stop);
                break;
            }
        }
    }
    out.dead_alone = (0..singles.len() as u32)
        .filter(|&b| singles[b as usize].dead)
        .collect();
    if (singles.len() as u32) < n {
        out.singles_left = u64::from(n) - singles.len() as u64;
        out.pairs.left = out.pairs.all;
        return Ok(());
    }

    // Implication and equivalence, off the singles' closures.
    let held = out.at_root.clone();
    let at_root = |b: u32| held.binary_search(&b).is_ok();
    let mut classes: BTreeMap<&[u32], Vec<u32>> = BTreeMap::new();
    for a in 0..n {
        let single = &singles[a as usize];
        if single.dead || at_root(a) {
            continue;
        }
        out.implies.extend(
            single
                .closure
                .iter()
                .filter(|&&b| b != a && !at_root(b))
                .map(|&b| (a, b)),
        );
        classes.entry(&single.closure).or_default().push(a);
    }
    out.equivalent = classes.into_values().filter(|c| c.len() > 1).collect();
    out.equivalent.sort();

    // The pairs, in the universe's order, a step at a time: the licences
    // first, then the node of each pair they leave.
    let mut pairs = (0..n).flat_map(|a| (a + 1..n).map(move |b| (a, b)));
    let mut skipped = [false; 2];
    loop {
        let want = step(sp, BATCH);
        let mut chunk: Vec<(u32, u32, Before)> = Vec::with_capacity(want);
        while chunk.len() < want {
            let Some((a, b)) = pairs.next() else {
                break;
            };
            let read = before(&singles, a, b);
            match read {
                Before::DeadMember if licensed => {
                    out.pairs.dead_member += 1;
                    skipped[0] = true;
                }
                Before::Implied if licensed => {
                    out.pairs.implied += 1;
                    skipped[1] = true;
                }
                _ => chunk.push((a, b, read)),
            }
        }
        if chunk.is_empty() {
            break;
        }
        let sets: Vec<Box<[u32]>> = chunk.iter().map(|&(a, b, _)| Box::from([a, b])).collect();
        let seen = match sp.eval(&sets)? {
            Ok(seen) => seen,
            Err(stop) => {
                out.stopped = Some(stop);
                out.pairs.left = chunk.len() as u64;
                // The licences still decide what they decide of the rest.
                for (a, b) in pairs.by_ref() {
                    match before(&singles, a, b) {
                        Before::DeadMember if licensed => out.pairs.dead_member += 1,
                        Before::Implied if licensed => out.pairs.implied += 1,
                        _ => out.pairs.left += 1,
                    }
                }
                break;
            }
        };
        out.pairs.saturated += chunk.len() as u64;
        let states = sp.states();
        for (&(a, b, read), node) in chunk.iter().zip(&seen) {
            let (x, y) = (&singles[a as usize], &singles[b as usize]);
            match read {
                // Unlicensed, read against what L1 would have said.
                Before::DeadMember => {
                    if !node.dead {
                        out.against_l1.push((a, b));
                    }
                }
                // And against L3: the node of whichever member implies.
                Before::Implied => {
                    let is = |s: &Single| !node.dead && node.class == s.class;
                    if (x.holds(b) && !is(x)) || (y.holds(a) && !is(y)) {
                        out.against_l3.push((a, b, node.dead));
                    }
                }
                Before::Open if node.dead => out.excluded.push((a, b)),
                Before::Open => {
                    if has_own_fact(
                        states[node.class as usize],
                        states[x.class as usize],
                        states[y.class as usize],
                    ) {
                        out.interacting.push((a, b));
                    }
                }
            }
        }
    }
    if skipped[0] {
        out.licences.push(Licence::DeadUpward);
    }
    if skipped[1] {
        out.licences.push(Licence::Closure);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_ir::{load, parse};

    fn layer_of(rel: &str, licensed: bool) -> PairLayer {
        let path = ein_corpus::repo_root().join(rel);
        let text = std::fs::read_to_string(&path).expect("the program");
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, &text, path.to_str()).expect("parses");
        let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).expect("loads");
        with_space(&mut kb, &mut terms, &ast, &Options::default(), |sp| {
            layer(sp, licensed)
        })
        .expect("the layer")
    }

    /// **The licences decide a pair as its node does.** A monotone program's
    /// layer read with no licence saturates every pair, finds none alive over
    /// a member dead alone and none that is not the node of the member that
    /// implies, and lists the excluded and the interacting pairs the licensed
    /// layer lists.
    #[test]
    fn the_licences_decide_a_pair_as_its_node_does() {
        for rel in [
            "examples/branching/02_one_dead_one_alive.ein",
            "examples/branching/08_hypothesis_relation_whitelist.ein",
            "examples/zebra2-hints.ein",
        ] {
            let with = layer_of(rel, true);
            let without = layer_of(rel, false);
            assert!(
                with.monotone && with.complete() && without.complete(),
                "{rel}"
            );
            assert!(
                with.pairs.dead_member > 0 && with.pairs.implied > 0,
                "{rel}"
            );
            assert_eq!(without.pairs.saturated, without.pairs.all, "{rel}");
            assert!(without.licences.is_empty(), "{rel}");
            assert_eq!(without.against_l1, [], "{rel}: against L1");
            assert_eq!(without.against_l3, [], "{rel}: against L3");
            assert_eq!(with.excluded, without.excluded, "{rel}");
            assert_eq!(with.interacting, without.interacting, "{rel}");
            assert_eq!(with.dead_alone, without.dead_alone, "{rel}");
            assert_eq!(with.implies, without.implies, "{rel}");
        }
    }
}
