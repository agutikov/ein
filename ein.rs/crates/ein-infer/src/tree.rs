//! The modes' routes on a monotone program — M1e2
//! [S1e2.1c.4](../../../../plans/m1e2_correctness/p1e2.1c_the_region_search/s1e2.1c.4_the_flags_at_scale.md),
//! the right-hand column of
//! [`solve_interface.md` § 3.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md#32-a-monotone-program-each-modes-route).
//!
//! Describe takes the state route ([`crate::states`]): it needs every state of
//! `R`, so it visits every alive state. The four other questions do not, and
//! each takes the least of the same lattice of states its question needs.
//! Every node is still [`crate::region`]'s, through [`crate::explore`]'s node
//! cache, so a mode and the oracle can visit different nodes and cannot read
//! one differently.
//!
//! ### The states, and three licences every route here uses
//!
//! A **state** is a closed set: the candidates `S` some fixpoint holds. On a
//! monotone program a commitment's state is its closure's (**L3**), so a route
//! moves from a state `S` to `cl(S ∪ {b})` by saturating the one commitment
//! `S ∪ {b}`, and reads it without a saturation when `S ∪ {b}` is already a
//! known state's closure. A dead set prunes every superset (**L1**): an
//! extension `S′ ∪ {b}` is dead without a visit when a dead `S ∪ {b}` with
//! `S ⊆ S′` is known. An alive extension of a state of `R` is in `R` (**L2**),
//! and the routes check it on every node they compute: a state of `R` with an
//! owing extension refuses the run, as the state route's does.
//!
//! ### L5, the obligation tree
//!
//! **The licence.** At an owing state `C` that owes the instance `I`, let
//! `A(I)` be the candidates that can witness `I`. When L5's premise holds of
//! `I`, every member of `R` above `C` holds one of them, so **branching on
//! `A(I)` alone loses no member of `R`**: the children `cl(C ∪ {a})`, `a ∈
//! A(I)`, cover `R ∩ ↑C`. And when every child is dead, `R ∩ ↑C` is empty —
//! the user's *owing meets dead*, which ends the branch.
//!
//! **The argument**, under
//! [`standard_of_proof.md`](../../../../docs/kernel/standard_of_proof.md)'s
//! rule 2. Let `D ⊇ C` be a member of `R`. The program is monotone, so `I`'s
//! trigger reads only relations no commitment changes ([`crate::monotone`]):
//! `I` is an instance at `D` too, and `D` is discharged, so a guard of `I`
//! holds at `D`. The premise says every read of that guard beside its witness
//! step is static, so the guard holds at `D` exactly when a witness fact `w`
//! is in `D`'s fixpoint, and `w` is one of the facts the guard admits at root.
//! The premise also says each of those is a candidate or has its negative
//! stored at root. The second kind is in no alive fixpoint, so `w` is a
//! candidate, `w ∈ cl(D)`, and `w ∉ cl(C)` since `C` still owes `I`. So
//! `cl(D) ⊇ C ∪ {w}` with `w ∈ A(I)`. If `C ∪ {w}` is dead, so is `cl(D)`
//! (L1), and so is `D`, whose state is `cl(D)`'s (L3): a contradiction. ∎
//!
//! **The premise, checked at load.** [`crate::oblgen::alternatives`] reads, at
//! root's fixpoint, every instance root owes and the facts that can witness
//! it, and licenses it only when the premise holds; the read-out's `route`
//! row names L5 only when a branch used it. **And on every node a route
//! computes**: the node function reports which of root's instances it owes
//! ([`crate::region::NodeOut::owed`]), and for every licensed instance the
//! route checks that it is owed exactly when none of its alternatives is in
//! the node's closure. A node that owes an instance root did not, or that
//! breaks that equivalence, refuses the run — the premise failing is an
//! engine defect, never an answer. The oracle holds the routes on every
//! monotone cube it enumerates (`ein-infer/tests/explore.rs`).
//!
//! **Which instance.** An owing state branches on the licensed instance it
//! owes with the fewest alternatives not already known dead, the first in
//! report order on a tie. A state that owes no licensed instance branches on
//! every extension, which is the state route's step and needs no licence: a
//! member of `R` above an owing `C` is a proper superset of it. The order is
//! the route's own and no lever reaches it (C4).
//!
//! **The hrule rung.** L5's premise is about `H` as a set, not about the rung
//! that proposed it, so it is checked the same way whatever the program
//! declares. On `zebra.ein` the instances `slot-owed-room` owes are licensed —
//! each attribute value's houses are hrule candidates — and those
//! `slot-owed-fill` owes are not: their witnesses are the symmetric
//! `(co-located House-n v)`, which the hrule never proposes. A state there
//! that owes only the second kind branches on every extension.
//!
//! ### The routes
//!
//! - **`-n N`**: the tree's leaves in its own order — owing states by size,
//!   then closure — then, when it runs out, the states of `R` above its
//!   leaves (L2), each extended once. When `∅ ∈ R`, root's state first and
//!   the states above it.
//! - **`--single`**: `∅ ∈ R`, root's extensions until one is alive. Otherwise
//!   the tree, stopping at its second state of `R`; with exactly one, `s`,
//!   `s`'s extensions until one is alive (L4).
//! - **`--minimal`**: each leaf of the tree, its commitment shrunk while it
//!   stays discharged — one pass suffices, by L2 — gives a state on the
//!   minimal line; then, when the question needs the whole line, every owing
//!   state extended once, and the line **by duality**: the minimal commitments
//!   of `R` are the alive minimal transversals of the complements of the
//!   maximal owing states, since a set is outside `R` and alive exactly when
//!   some owing state holds it (L2).
//! - **`--maximal`**: a first state of `R` (root's, or the tree's first leaf)
//!   climbed to a maximal state; then above each minimal state of `R`
//!   **dualise and advance** (Gunopulos et al., *PODS* 1997): every maximal
//!   state above a seed `σ` not found yet holds `σ` and a minimal transversal
//!   of the complements of those found, so each such transversal is saturated
//!   with `σ`, and an alive one is climbed to a new maximal state. When none
//!   is alive, the maximal states above `σ` are all found. A climb that
//!   reaches ⊤ ends the question (L1).

use std::collections::{BTreeSet, VecDeque};
use std::sync::Arc;

use rustc_hash::FxHashMap;

use crate::error::SolveError;
use crate::explore::{Ask, Found, Licence, Mode, Route, Seen, Space, State, Step, Verdict, Which};
use crate::oblgen::Alternatives;
use crate::transversal::Bits;

/// The largest step a route takes on one state's extensions, as
/// [`crate::explore`]'s.
const BATCH: usize = 1024;

/// One state a route met.
struct St {
    closure: Box<[u32]>,
    bits: Bits,
    /// The commitment that first reached it: the candidates the route added
    /// from root. Its closure is [`Self::closure`] (L3).
    path: Box<[u32]>,
    owes: u32,
    /// The instances of L5's table it owes.
    owed: Box<[u32]>,
    /// [`Seen::class`]: its fixpoint.
    class: u32,
    /// The candidates whose extension of it is decided.
    tried: Bits,
    /// The alive states one candidate up, among those decided.
    up: Vec<u32>,
    /// What an owing state branches on, once chosen: an instance of L5's
    /// table (`Some(i)`), or every extension (`None`).
    branch: Option<Option<u32>>,
    /// Its next step's size: 1, 2, 4, … up to [`BATCH`].
    step: usize,
}

impl St {
    fn in_r(&self) -> bool {
        self.owes == 0
    }
}

/// Which states a step extends, and by what.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    /// Owing states, each by its branch set: the tree.
    Tree,
    /// Owing states, each by every extension.
    Owing,
    /// States of `R`, each by every extension.
    Up,
}

/// The states a route met, and the work it has queued.
pub(crate) struct Search {
    n: usize,
    st: Vec<St>,
    by_closure: FxHashMap<Box<[u32]>, u32>,
    by_class: FxHashMap<u32, u32>,
    /// Per candidate `b`, the sets `S` whose `S ∪ {b}` is dead (L1).
    dead_under: Vec<Vec<Bits>>,
    table: Option<Arc<Alternatives>>,
    /// Owing states the tree has not finished branching, by size, closure.
    tree_q: BTreeSet<(usize, Box<[u32]>)>,
    /// Owing states with an extension not yet decided.
    owing_q: BTreeSet<(usize, Box<[u32]>)>,
    /// States of `R` with an extension not yet decided.
    up_q: BTreeSet<(usize, Box<[u32]>)>,
    /// States of `R` a tree step met, not yet handed out.
    leaves: VecDeque<u32>,
    /// Every state of `R` met, in the order met.
    r_met: Vec<u32>,
    /// Some branch took an instance's alternatives (L5).
    pub(crate) l5: bool,
}

/// A state's step, or why it refused: a node contradicted a licence.
fn refuse(what: &str) -> SolveError {
    SolveError::Unsupported(format!(
        "{what} — the static verdict's premise failed; run `ein oracle` on this program"
    ))
}

/// `set ∪ {b}`, ascending, for `b ∉ set`.
fn with(set: &[u32], b: u32) -> Box<[u32]> {
    let mut s = set.to_vec();
    let at = s.partition_point(|&x| x < b);
    s.insert(at, b);
    s.into_boxed_slice()
}

fn key(st: &St) -> (usize, Box<[u32]>) {
    (st.closure.len(), st.closure.clone())
}

impl Search {
    /// A search over `sp`'s program, with L5's table when it has obligations,
    /// starting from root's node, which `root` is.
    pub(crate) fn new(
        sp: &Space<'_>,
        table: Option<Arc<Alternatives>>,
        root: &Seen,
    ) -> Result<Search, SolveError> {
        let n = sp.n();
        let mut s = Search {
            n,
            st: Vec::new(),
            by_closure: FxHashMap::default(),
            by_class: FxHashMap::default(),
            dead_under: vec![Vec::new(); n],
            table,
            tree_q: BTreeSet::new(),
            owing_q: BTreeSet::new(),
            up_q: BTreeSet::new(),
            leaves: VecDeque::new(),
            r_met: Vec::new(),
            l5: false,
        };
        s.register(Box::default(), &[], root)?;
        Ok(s)
    }

    fn root(&self) -> u32 {
        0
    }

    /// L5's premise on one alive node: every licensed instance is owed
    /// exactly when none of its alternatives is in the closure, and no
    /// instance is owed that root did not owe.
    fn check_owed(&self, seen: &Seen, bits: &Bits) -> Result<(), SolveError> {
        let Some(t) = self.table.as_ref() else {
            return Ok(());
        };
        if seen.owed.contains(&u32::MAX) {
            return Err(refuse("a node owes an obligation instance root did not"));
        }
        for (i, inst) in t.instances.iter().enumerate() {
            let Ok(alts) = &inst.alternatives else {
                continue;
            };
            let witnessed = alts.iter().any(|&a| bits.contains(a));
            let owed = seen.owed.contains(&(i as u32));
            if witnessed == owed {
                return Err(refuse(&format!(
                    "L5: a node {} instance {i} {} a witness in its closure",
                    if owed { "owes" } else { "has paid" },
                    if witnessed { "with" } else { "without" }
                )));
            }
        }
        Ok(())
    }

    /// The state an alive node reached, met before or new.
    fn register(&mut self, path: Box<[u32]>, set: &[u32], seen: &Seen) -> Result<u32, SolveError> {
        let bits = Bits::of(self.n, &seen.closure);
        if !set.iter().all(|&b| bits.contains(b)) {
            return Err(refuse(
                "a fixpoint does not hold its commitment (a fork only adds)",
            ));
        }
        if let Some(&t) = self.by_closure.get(&seen.closure) {
            if self.st[t as usize].class != seen.class {
                return Err(refuse("two fixpoints on one closure (L3)"));
            }
            return Ok(t);
        }
        if self.by_class.contains_key(&seen.class) {
            return Err(refuse("one fixpoint on two closures (L3)"));
        }
        self.check_owed(seen, &bits)?;
        let t = self.st.len() as u32;
        let st = St {
            closure: seen.closure.clone(),
            bits,
            path,
            owes: seen.owes,
            owed: seen.owed.clone(),
            class: seen.class,
            tried: Bits::empty(self.n),
            up: Vec::new(),
            branch: None,
            step: 1,
        };
        let k = key(&st);
        if st.in_r() {
            self.r_met.push(t);
            self.up_q.insert(k);
        } else {
            self.tree_q.insert(k.clone());
            self.owing_q.insert(k);
        }
        self.by_closure.insert(st.closure.clone(), t);
        self.by_class.insert(st.class, t);
        self.st.push(st);
        Ok(t)
    }

    /// A known dead set below `x`: `S ∪ {b} ⊆ x` with `S ∪ {b}` dead (L1).
    fn pruned(&self, x: &Bits) -> bool {
        x.iter()
            .any(|b| self.dead_under[b as usize].iter().any(|s| s.is_subset(x)))
    }

    fn record_dead(&mut self, set: &[u32]) {
        if let Some((&b, rest)) = set.split_last() {
            self.dead_under[b as usize].push(Bits::of(self.n, rest));
        }
    }

    /// One commitment, by L1, L3 or a saturation: `None` when it is dead, its
    /// state otherwise. `path` is what the state remembers if it is new.
    fn eval_set(
        &mut self,
        sp: &mut Space<'_>,
        set: Box<[u32]>,
        path: Box<[u32]>,
    ) -> Step<Option<u32>> {
        if self.pruned(&Bits::of(self.n, &set)) {
            return Ok(Ok(None));
        }
        if let Some(&t) = self.by_closure.get(&set) {
            return Ok(Ok(Some(t)));
        }
        let seen = match sp.eval(std::slice::from_ref(&set))? {
            Ok(mut v) => v.pop().expect("one"),
            Err(stop) => return Ok(Err(stop)),
        };
        if seen.dead {
            self.record_dead(&set);
            return Ok(Ok(None));
        }
        Ok(Ok(Some(self.register(path, &set, &seen)?)))
    }

    /// Extend state `s` by each of `bs` (ascending, outside it), in one step:
    /// each extension pruned (L1), read off a known state (L3), or saturated.
    /// Its outcome per candidate: the state reached, or `None` when dead.
    fn extend(&mut self, sp: &mut Space<'_>, s: u32, bs: &[u32]) -> Step<Vec<Option<u32>>> {
        enum How {
            Dead,
            Known(u32),
            Fresh(usize),
        }
        let mut fresh: Vec<Box<[u32]>> = Vec::new();
        let mut how: Vec<How> = Vec::with_capacity(bs.len());
        {
            let st = &self.st[s as usize];
            for &b in bs {
                let pruned = self.dead_under[b as usize]
                    .iter()
                    .any(|d| d.is_subset(&st.bits));
                if pruned {
                    how.push(How::Dead);
                    continue;
                }
                let x = with(&st.closure, b);
                if let Some(&t) = self.by_closure.get(&x) {
                    how.push(How::Known(t));
                    continue;
                }
                how.push(How::Fresh(fresh.len()));
                fresh.push(x);
            }
        }
        let seen = match sp.eval(&fresh)? {
            Ok(v) => v,
            Err(stop) => return Ok(Err(stop)),
        };
        let mut out = Vec::with_capacity(bs.len());
        for (&b, h) in bs.iter().zip(how) {
            self.st[s as usize].tried.insert(b);
            let t = match h {
                How::Dead => None,
                How::Known(t) => Some(t),
                How::Fresh(i) => {
                    if seen[i].dead {
                        let bits = self.st[s as usize].bits.clone();
                        self.dead_under[b as usize].push(bits);
                        None
                    } else {
                        let path = with(&self.st[s as usize].path, b);
                        Some(self.register(path, &fresh[i], &seen[i])?)
                    }
                }
            };
            if let Some(t) = t {
                let from = &self.st[s as usize];
                if from.in_r() && !self.st[t as usize].in_r() {
                    return Err(refuse(
                        "a state of R has an owing extension (L2: discharged is upward-closed)",
                    ));
                }
                if !from.up.contains(&t) {
                    self.st[s as usize].up.push(t);
                }
            }
            out.push(t);
        }
        Ok(Ok(out))
    }

    /// The candidates outside `s` not yet decided for it, ascending.
    fn untried(&self, s: u32) -> Vec<u32> {
        let st = &self.st[s as usize];
        (0..self.n as u32)
            .filter(|&b| !st.bits.contains(b) && !st.tried.contains(b))
            .collect()
    }

    /// What the tree branches `s` on, choosing it the first time: the licensed
    /// instance it owes with the fewest alternatives not known dead (L5), or
    /// every extension. What is left of it, ascending.
    fn branch_rest(&mut self, s: u32) -> Vec<u32> {
        if self.st[s as usize].branch.is_none() {
            let choice = self.table.as_ref().and_then(|t| {
                let st = &self.st[s as usize];
                st.owed
                    .iter()
                    .filter_map(|&i| {
                        let alts = t.instances.get(i as usize)?.alternatives.as_ref().ok()?;
                        let open = alts
                            .iter()
                            .filter(|&&a| {
                                !self.dead_under[a as usize]
                                    .iter()
                                    .any(|d| d.is_subset(&st.bits))
                            })
                            .count();
                        Some((open, i))
                    })
                    .min()
                    .map(|(_, i)| i)
            });
            if choice.is_some() {
                self.l5 = true;
            }
            self.st[s as usize].branch = Some(choice);
        }
        let st = &self.st[s as usize];
        match st.branch.expect("chosen") {
            Some(i) => {
                let t = self.table.as_ref().expect("a table");
                let alts = t.instances[i as usize]
                    .alternatives
                    .as_ref()
                    .expect("licensed");
                alts.iter()
                    .copied()
                    .filter(|&a| !st.bits.contains(a) && !st.tried.contains(a))
                    .collect()
            }
            None => self.untried(s),
        }
    }

    /// One step of `phase`: the first state of its queue, by size and
    /// closure, extended by the next part of what the phase extends it by.
    /// `Ok(Ok(false))` when the phase has nothing left.
    fn step(&mut self, sp: &mut Space<'_>, phase: Phase) -> Step<bool> {
        loop {
            let q = match phase {
                Phase::Tree => &self.tree_q,
                Phase::Owing => &self.owing_q,
                Phase::Up => &self.up_q,
            };
            let Some(k) = q.iter().next().cloned() else {
                return Ok(Ok(false));
            };
            let s = self.by_closure[&k.1];
            let rest = match phase {
                Phase::Tree => self.branch_rest(s),
                Phase::Owing | Phase::Up => self.untried(s),
            };
            if rest.is_empty() {
                match phase {
                    Phase::Tree => self.tree_q.remove(&k),
                    Phase::Owing => self.owing_q.remove(&k),
                    Phase::Up => self.up_q.remove(&k),
                };
                continue;
            }
            // A full sweep takes a state's extensions at once; a phase a
            // question can stop takes them 1, 2, 4, … at a time.
            let take = if phase == Phase::Owing {
                rest.len()
            } else {
                let st = &mut self.st[s as usize];
                let take = st.step.min(rest.len());
                st.step = (st.step * 2).min(BATCH);
                take
            };
            let before = self.st.len() as u32;
            if let Err(stop) = self.extend(sp, s, &rest[..take])? {
                return Ok(Err(stop));
            }
            if phase != Phase::Up {
                // The states of R first met by branching an owing state are
                // the tree's leaves, in the order met.
                for t in before..self.st.len() as u32 {
                    if self.st[t as usize].in_r() {
                        self.leaves.push_back(t);
                    }
                }
            }
            return Ok(Ok(true));
        }
    }

    /// The tree's next leaf: a state of `R` met by branching an owing state,
    /// in the order met. `None` when the tree is exhausted.
    fn next_leaf(&mut self, sp: &mut Space<'_>) -> Step<Option<u32>> {
        loop {
            if let Some(l) = self.leaves.pop_front() {
                return Ok(Ok(Some(l)));
            }
            match self.step(sp, Phase::Tree)? {
                Ok(true) => {}
                Ok(false) => return Ok(Ok(None)),
                Err(stop) => return Ok(Err(stop)),
            }
        }
    }

    /// Run `phase` to its end.
    fn sweep(&mut self, sp: &mut Space<'_>, phase: Phase) -> Step<()> {
        loop {
            match self.step(sp, phase)? {
                Ok(true) => {}
                Ok(false) => return Ok(Ok(())),
                Err(stop) => return Ok(Err(stop)),
            }
        }
    }

    /// Climb from `s`, a state of `R`: add the lowest-index candidate whose
    /// extension is alive, until none is. Every alive extension of a state of
    /// `R` is in `R` (L2), and a candidate whose extension died below stays
    /// dead above (L1), so one ascending pass ends at a state whose every
    /// extension is dead — maximal, by L4.
    fn climb(&mut self, sp: &mut Space<'_>, s: u32) -> Step<u32> {
        let mut cur = s;
        for b in 0..self.n as u32 {
            if self.st[cur as usize].bits.contains(b) {
                continue;
            }
            match self.extend(sp, cur, &[b])? {
                Ok(v) => {
                    if let Some(t) = v[0] {
                        cur = t;
                    }
                }
                Err(stop) => return Ok(Err(stop)),
            }
        }
        // Every candidate outside the top died on a set under it.
        for b in self.untried(cur) {
            let pruned = self.dead_under[b as usize]
                .iter()
                .any(|d| d.is_subset(&self.st[cur as usize].bits));
            if !pruned {
                return Err(refuse("a climb's top has an extension not known dead (L1)"));
            }
            self.st[cur as usize].tried.insert(b);
        }
        Ok(Ok(cur))
    }

    /// The next maximal state above `seed` not among `tops`, by dualise and
    /// advance; `None` when every maximal state above it is in `tops`.
    fn next_top(&mut self, sp: &mut Space<'_>, seed: u32, tops: &[u32]) -> Step<Option<u32>> {
        let sb = self.st[seed as usize].bits.clone();
        let mine: Vec<u32> = tops
            .iter()
            .copied()
            .filter(|&t| sb.is_subset(&self.st[t as usize].bits))
            .collect();
        if mine.is_empty() {
            return self.climb(sp, seed).map(|r| r.map(Some));
        }
        let full = Bits::full(self.n);
        let edges: Vec<Bits> = mine
            .iter()
            .map(|&t| full.minus(&self.st[t as usize].bits))
            .collect();
        for t in crate::transversal::minimal_transversals(self.n, &edges) {
            let x = sb.or(&t);
            if self.pruned(&x) {
                continue;
            }
            let path: Vec<u32> = self.st[seed as usize]
                .path
                .iter()
                .copied()
                .chain(t.iter())
                .collect::<BTreeSet<u32>>()
                .into_iter()
                .collect();
            let c = match self.eval_set(sp, x.to_set(), path.into_boxed_slice())? {
                Ok(Some(c)) => c,
                Ok(None) => continue,
                Err(stop) => return Ok(Err(stop)),
            };
            if !self.st[c as usize].in_r() {
                return Err(refuse(
                    "an alive set above a state of R owes (L2: discharged is upward-closed)",
                ));
            }
            return self.climb(sp, c).map(|r| r.map(Some));
        }
        Ok(Ok(None))
    }

    /// Shrink `s`'s commitment while it stays in `R`: one pass, in candidate
    /// order. A subset of a commitment of `R` is alive (L1), and if dropping
    /// `x` failed once it fails on every smaller set (L2), so what is left is
    /// a minimal commitment of `R`. It, and the state it reaches.
    fn shrink(&mut self, sp: &mut Space<'_>, s: u32) -> Step<(Box<[u32]>, u32)> {
        let mut g: Vec<u32> = self.st[s as usize].path.to_vec();
        let mut at = s;
        for x in self.st[s as usize].path.clone().iter() {
            let y: Box<[u32]> = g.iter().copied().filter(|&b| b != *x).collect();
            match self.eval_set(sp, y.clone(), y.clone())? {
                Ok(Some(t)) => {
                    if self.st[t as usize].in_r() {
                        g = y.to_vec();
                        at = t;
                    }
                }
                Ok(None) => {
                    return Err(refuse("a subset of a commitment of R is dead (L1)"));
                }
                Err(stop) => return Ok(Err(stop)),
            }
        }
        Ok(Ok((g.into_boxed_slice(), at)))
    }

    /// The minimal line, once every owing state is extended: the alive minimal
    /// transversals of the maximal owing states' complements, each with the
    /// state it reaches — the least state of `R` met that holds it, which is
    /// its closure. By size, then by candidates.
    fn minimal_line(&self) -> Vec<(Box<[u32]>, u32)> {
        let full = Bits::full(self.n);
        let edges: Vec<Bits> = self
            .st
            .iter()
            .filter(|st| !st.in_r() && st.up.iter().all(|&u| self.st[u as usize].in_r()))
            .map(|st| full.minus(&st.bits))
            .collect();
        let mut out = Vec::new();
        for t in crate::transversal::minimal_transversals(self.n, &edges) {
            let reach = self
                .r_met
                .iter()
                .copied()
                .filter(|&r| t.is_subset(&self.st[r as usize].bits))
                .min_by_key(|&r| (self.st[r as usize].closure.len(), r));
            if let Some(r) = reach {
                out.push((t.to_set(), r));
            }
        }
        out
    }

    /// A state as the answer prints it: its commitment and its closure.
    fn state(&self, s: u32) -> State {
        let st = &self.st[s as usize];
        State {
            set: st.path.clone(),
            closure: st.closure.clone(),
        }
    }

    /// A maximal state as the answer prints it: the state is its own
    /// commitment.
    fn top(&self, s: u32) -> State {
        let st = &self.st[s as usize];
        State {
            set: st.closure.clone(),
            closure: st.closure.clone(),
        }
    }

    fn route(&self) -> Route {
        if self.l5 { Route::Tree } else { Route::States }
    }

    fn licences(&self, base: &[Licence]) -> Vec<Licence> {
        let mut v = base.to_vec();
        if self.l5 {
            v.push(Licence::Exhaustive);
        }
        v.sort();
        v
    }
}

/// Answer `mode` on a monotone program whose root is alive: the module's
/// routes. Describe is not asked here.
pub(crate) fn answer(sp: &mut Space<'_>, mode: Mode) -> Result<Found, SolveError> {
    // L5's table first: every node, root's included, names what it owes.
    let table = sp.alternatives()?;
    let root = match sp.eval(&[Box::default()])? {
        Ok(mut v) => v.pop().expect("one"),
        Err(stop) => {
            return Ok(Found {
                stopped: Some(stop),
                ..Found::default()
            });
        }
    };
    if root.dead {
        return Err(refuse("root's node is dead and root's fixpoint is not"));
    }
    let mut s = Search::new(sp, table, &root)?;
    let mut f = match (mode.which, mode.ask) {
        (Which::All, Ask::Find(want)) => find(sp, &mut s, want)?,
        (Which::All, Ask::Single) => single(sp, &mut s)?,
        (Which::Minimal, ask) => minimal(sp, &mut s, ask)?,
        (Which::Maximal, ask) => maximal(sp, &mut s, ask)?,
        (Which::All, Ask::Describe) => unreachable!("describe takes the state route"),
        (_, Ask::Count) => unreachable!("--count is describe's question, and takes its route"),
    };
    f.route = Some(s.route());
    f.licences = s.licences(&f.licences);
    Ok(f)
}

/// `-n N` over all of `R`: the tree's leaves, then the states above them.
fn find(sp: &mut Space<'_>, s: &mut Search, want: u64) -> Result<Found, SolveError> {
    let mut f = Found {
        licences: vec![
            Licence::DeadUpward,
            Licence::DischargedUpward,
            Licence::Closure,
        ],
        ..Found::default()
    };
    let mut shown = 0usize;
    // The tree first; root's state is the one seed when it is in R.
    let mut phase = Phase::Tree;
    loop {
        while shown < s.r_met.len() {
            f.states.push(s.state(s.r_met[shown]));
            shown += 1;
            if f.states.len() as u64 >= want {
                return Ok(f);
            }
        }
        match s.step(sp, phase)? {
            Ok(true) => {}
            Ok(false) if phase == Phase::Tree => phase = Phase::Up,
            Ok(false) => {
                f.exhausted = true;
                return Ok(f);
            }
            Err(stop) => {
                f.stopped = Some(stop);
                return Ok(f);
            }
        }
    }
}

/// `s`'s extensions, 1, 2, 4, … at a time, until one is alive: that state,
/// or `None` when every one is dead.
fn alive_above(sp: &mut Space<'_>, s: &mut Search, at: u32) -> Step<Option<u32>> {
    let rest = s.untried(at);
    let mut size = 1usize;
    let mut i = 0usize;
    while i < rest.len() {
        let end = (i + size).min(rest.len());
        match s.extend(sp, at, &rest[i..end])? {
            Ok(v) => {
                if let Some(t) = v.into_iter().flatten().next() {
                    return Ok(Ok(Some(t)));
                }
            }
            Err(stop) => return Ok(Err(stop)),
        }
        i = end;
        size = (size * 2).min(BATCH);
    }
    Ok(Ok(None))
}

/// `--single` over all of `R`.
fn single(sp: &mut Space<'_>, s: &mut Search) -> Result<Found, SolveError> {
    let root = s.root();
    if s.st[root as usize].in_r() {
        // ∅ ∈ R: every alive node is in R (L2), so one alive extension of
        // root's state is a second state.
        let mut f = Found {
            licences: vec![Licence::DeadUpward, Licence::DischargedUpward],
            states: vec![s.state(root)],
            ..Found::default()
        };
        match alive_above(sp, s, root)? {
            Ok(Some(t)) => {
                f.states.push(s.state(t));
                f.verdict = Some(Verdict::Ambiguity);
            }
            Ok(None) => {
                f.verdict = Some(Verdict::Solution);
                f.exhausted = true;
            }
            Err(stop) => f.stopped = Some(stop),
        }
        return Ok(f);
    }
    let mut f = Found {
        licences: vec![
            Licence::DeadUpward,
            Licence::DischargedUpward,
            Licence::Closure,
            Licence::LocalLines,
        ],
        ..Found::default()
    };
    let mut leaves: Vec<u32> = Vec::new();
    loop {
        match s.next_leaf(sp)? {
            Ok(Some(l)) => {
                leaves.push(l);
                if leaves.len() >= 2 {
                    f.states = leaves.iter().map(|&l| s.state(l)).collect();
                    f.verdict = Some(Verdict::Ambiguity);
                    return Ok(f);
                }
            }
            Ok(None) => break,
            Err(stop) => {
                f.states = leaves.iter().map(|&l| s.state(l)).collect();
                f.stopped = Some(stop);
                return Ok(f);
            }
        }
    }
    let Some(&one) = leaves.first() else {
        f.verdict = Some(Verdict::Open);
        f.exhausted = true;
        return Ok(f);
    };
    // Every member of R lies above a leaf, and every alive state above the
    // one leaf is a member (L2): it is all of R when nothing is alive above
    // it (L4).
    f.states = vec![s.state(one)];
    match alive_above(sp, s, one)? {
        Ok(Some(t)) => {
            f.states.push(s.state(t));
            f.verdict = Some(Verdict::Ambiguity);
        }
        Ok(None) => {
            f.verdict = Some(Verdict::Solution);
            f.exhausted = true;
        }
        Err(stop) => f.stopped = Some(stop),
    }
    Ok(f)
}

/// The minimal line: shrunk leaves first, then the line by duality when the
/// question needs it.
fn minimal(sp: &mut Space<'_>, s: &mut Search, ask: Ask) -> Result<Found, SolveError> {
    let root = s.root();
    if s.st[root as usize].in_r() {
        // ∅ ∈ R: the minimal line is {∅} (L2), one saturation.
        return Ok(Found {
            licences: vec![Licence::DeadUpward, Licence::DischargedUpward],
            states: vec![s.state(root)],
            minimal: vec![(Box::default(), 0)],
            exhausted: true,
            verdict: (ask == Ask::Single).then_some(Verdict::Solution),
            ..Found::default()
        });
    }
    let mut f = Found {
        licences: vec![
            Licence::DeadUpward,
            Licence::DischargedUpward,
            Licence::Closure,
        ],
        ..Found::default()
    };
    let enough = match ask {
        Ask::Find(n) => n as usize,
        Ask::Single => 2,
        Ask::Describe => usize::MAX,
        Ask::Count => unreachable!("--count is over all of R"),
    };
    // Each leaf, shrunk: a minimal commitment, and a state on the line.
    let mut found: Vec<(Box<[u32]>, u32)> = Vec::new();
    let stopped = loop {
        if found.len() >= enough {
            break None;
        }
        let leaf = match s.next_leaf(sp)? {
            Ok(Some(l)) => l,
            Ok(None) => break None,
            Err(stop) => break Some(stop),
        };
        match s.shrink(sp, leaf)? {
            Ok((g, at)) => {
                if !found.iter().any(|(_, t)| *t == at) {
                    found.push((g, at));
                }
            }
            Err(stop) => break Some(stop),
        }
    };
    let shown = |s: &Search, found: &[(Box<[u32]>, u32)], f: &mut Found| {
        f.states = found
            .iter()
            .map(|(g, at)| State {
                set: g.clone(),
                closure: s.st[*at as usize].closure.clone(),
            })
            .collect();
        f.minimal = found
            .iter()
            .enumerate()
            .map(|(i, (g, _))| (g.clone(), i))
            .collect();
    };
    if let Some(stop) = stopped {
        shown(s, &found, &mut f);
        f.stopped = Some(stop);
        return Ok(f);
    }
    if found.len() >= enough {
        shown(s, &found, &mut f);
        if ask == Ask::Single {
            f.verdict = Some(Verdict::Ambiguity);
        }
        return Ok(f);
    }
    // The whole line: every owing state extended once, then the duality.
    if let Err(stop) = s.sweep(sp, Phase::Owing)? {
        shown(s, &found, &mut f);
        f.stopped = Some(stop);
        return Ok(f);
    }
    let line = s.minimal_line();
    // Its states, by their least minimal commitment: the oracle's order.
    let mut order: Vec<u32> = Vec::new();
    for (_, at) in &line {
        if !order.contains(at) {
            order.push(*at);
        }
    }
    let k = order.len();
    let all = match ask {
        Ask::Describe => true,
        Ask::Find(n) => k as u64 <= n,
        Ask::Single => k <= 1,
        Ask::Count => unreachable!("--count is over all of R"),
    };
    let printed: Vec<u32> = if all {
        order
    } else {
        // More than asked: the leaves found first, then the line's order.
        let mut v: Vec<u32> = found.iter().map(|(_, t)| *t).collect();
        for t in order {
            if !v.contains(&t) {
                v.push(t);
            }
        }
        v.truncate(enough);
        v
    };
    f.exhausted = all;
    if ask == Ask::Single {
        f.verdict = Some(match k {
            0 => Verdict::Open,
            1 => Verdict::Solution,
            _ => Verdict::Ambiguity,
        });
    }
    f.states = printed
        .iter()
        .map(|&t| {
            let g = line
                .iter()
                .find(|(_, at)| *at == t)
                .map(|(g, _)| g.clone())
                .expect("a state of the line");
            State {
                set: g,
                closure: s.st[t as usize].closure.clone(),
            }
        })
        .collect();
    f.minimal = line
        .iter()
        .filter_map(|(g, at)| printed.iter().position(|t| t == at).map(|i| (g.clone(), i)))
        .collect();
    Ok(f)
}

/// The maximal line: a first state of `R` climbed, then dualise and advance
/// above each minimal state of `R`.
fn maximal(sp: &mut Space<'_>, s: &mut Search, ask: Ask) -> Result<Found, SolveError> {
    let mut f = Found {
        licences: vec![
            Licence::DeadUpward,
            Licence::DischargedUpward,
            Licence::Closure,
            Licence::LocalLines,
        ],
        ..Found::default()
    };
    let enough = match ask {
        Ask::Find(n) => n as usize,
        Ask::Single => 2,
        Ask::Describe => usize::MAX,
        Ask::Count => unreachable!("--count is over all of R"),
    };
    let root = s.root();
    let root_in_r = s.st[root as usize].in_r();
    let first = if root_in_r {
        root
    } else {
        match s.next_leaf(sp)? {
            Ok(Some(l)) => l,
            Ok(None) => {
                f.exhausted = true;
                if ask == Ask::Single {
                    f.verdict = Some(Verdict::Open);
                }
                return Ok(f);
            }
            Err(stop) => {
                f.stopped = Some(stop);
                return Ok(f);
            }
        }
    };
    let mut tops: Vec<u32> = Vec::new();
    let done = |s: &Search, tops: &[u32], f: &mut Found, exhausted: bool| {
        finish(s, tops, f, ask, exhausted)
    };
    match s.climb(sp, first)? {
        Ok(t) => tops.push(t),
        Err(stop) => {
            f.stopped = Some(stop);
            return Ok(f);
        }
    }
    if s.st[tops[0] as usize].closure.len() == s.n {
        // ⊤ is alive, so every node is (L1) and ⊤ is the one maximal state.
        done(s, &tops, &mut f, true);
        return Ok(f);
    }
    if tops.len() >= enough {
        done(s, &tops, &mut f, false);
        return Ok(f);
    }
    // The seeds: root's state, or every minimal leaf of the whole tree.
    let mut seeds: Vec<u32> = vec![first];
    let mut exhausted_seeds = root_in_r;
    let mut i = 0usize;
    loop {
        if i == seeds.len() {
            if exhausted_seeds {
                break;
            }
            // The rest of the tree: its leaves, the minimal ones as seeds.
            let mut leaves: Vec<u32> = vec![first];
            loop {
                match s.next_leaf(sp)? {
                    Ok(Some(l)) => leaves.push(l),
                    Ok(None) => break,
                    Err(stop) => {
                        done(s, &tops, &mut f, false);
                        f.stopped = Some(stop);
                        return Ok(f);
                    }
                }
            }
            exhausted_seeds = true;
            let minimal: Vec<u32> = leaves
                .iter()
                .copied()
                .filter(|&l| {
                    let lb = &s.st[l as usize].bits;
                    !leaves
                        .iter()
                        .any(|&m| m != l && s.st[m as usize].bits.is_subset(lb))
                })
                .collect();
            let mut minimal: Vec<u32> =
                minimal.into_iter().filter(|m| !seeds.contains(m)).collect();
            minimal.sort_by(|&a, &b| {
                let (x, y) = (&s.st[a as usize].closure, &s.st[b as usize].closure);
                (x.len(), x).cmp(&(y.len(), y))
            });
            seeds.extend(minimal);
            continue;
        }
        let seed = seeds[i];
        match s.next_top(sp, seed, &tops)? {
            Ok(Some(t)) => {
                if tops.contains(&t) {
                    return Err(refuse("dualise and advance met a maximal state twice"));
                }
                tops.push(t);
                if tops.len() >= enough {
                    done(s, &tops, &mut f, false);
                    return Ok(f);
                }
            }
            Ok(None) => i += 1,
            Err(stop) => {
                done(s, &tops, &mut f, false);
                f.stopped = Some(stop);
                return Ok(f);
            }
        }
    }
    done(s, &tops, &mut f, true);
    Ok(f)
}

/// The maximal line found, as the answer prints it. All of it is in the
/// oracle's order; `--single` judges it.
fn finish(s: &Search, tops: &[u32], f: &mut Found, ask: Ask, exhausted: bool) {
    let mut v = tops.to_vec();
    if exhausted && ask == Ask::Describe {
        v.sort_by(|&a, &b| {
            let (x, y) = (&s.st[a as usize].closure, &s.st[b as usize].closure);
            (x.len(), x).cmp(&(y.len(), y))
        });
    }
    f.states = v.iter().map(|&t| s.top(t)).collect();
    let two = tops.len() >= 2;
    f.exhausted = exhausted && !(ask == Ask::Single && two);
    if ask == Ask::Single {
        f.verdict = if two {
            Some(Verdict::Ambiguity)
        } else if exhausted {
            Some(if tops.is_empty() {
                Verdict::Open
            } else {
                Verdict::Solution
            })
        } else {
            None
        };
    }
}
