//! The border route — describe on a monotone program by the borders of the
//! predicates its description reads, and not by every alive state.
//!
//! The state route ([`crate::states`]) visits every alive state once and
//! reads every row of the description off what it met. Every quantity it
//! reports is a **border** of a monotone predicate over the cube, or a count
//! over one, and this route finds the borders instead:
//!
//! | what the description reports | the border | how it is found |
//! |---|---|---|
//! | `R`'s states | *in R*, up-closed among the alive | the obligation tree exhausted (L5, L9), then every state of `R` met extended by every candidate (L2, L3): `-n N`'s route with no `N` (`crate::tree`) |
//! | per state `s`, its class `{C ⊆ s : cl(C) = s}`: its commitments and minimal generators | *generates s*, an up-set inside `2^s`: its minimal elements are the generators, the maximal sets outside it the maximal closed proper subsets `m_j` of `s` | dualise and advance inside `2^s`, `cl` the oracle (`class`) |
//! | the minimal line | `R`'s minimal commitments | the minimal sets among every state's generators |
//! | the maximal line | `R`'s maximal states | a state of `R` every extension of which is dead (L4) |
//! | `N`, and an empty `R`'s frontier | the minimal dead sets, and their dual, the maximal alive sets | dualise and advance between the two (`global`) |
//! | the candidates dead alone | `N`'s singletons | `N`'s, or root's one-up extensions where `N` is not taken |
//!
//! ### Dualise and advance
//!
//! Gunopulos et al. (*PODS* 1997), as `--maximal`'s route takes it above a
//! seed. For a monotone predicate with a known family `M` of maximal sets
//! outside it, every minimal transversal `T` of the complements `{U ∖ m : m ∈
//! M}` is a candidate for a minimal set inside it: each proper subset of `T`
//! misses some complement, so lies inside some `m`, so is outside. `T` is
//! asked; inside, it is a minimal member; outside, it is climbed to a new
//! maximal set outside, and the transversals are taken again. When every
//! transversal is inside, `M` holds every maximal set outside: a maximal one
//! not in `M` meets every complement (it lies inside no other), so some
//! minimal transversal lies inside it and would have been asked and found
//! outside. So the two borders are found together, each member asked once.
//!
//! - **Inside a state `s`** the predicate is *generates s*, the universe is
//!   `s` and the oracle `cl`, read by saturating subsets of `s` only (each
//!   alive, since `s` is). The seed `M` is the maximal closed proper subsets
//!   of `s` among the states the run met. Where every state met inside `s`
//!   was extended by every candidate, every closed subset of `s` was met
//!   (each has a chain of one-candidate extensions from root's state, every
//!   link alive and closed inside `s`), so the seed is the border and
//!   nothing is asked: that is the state route's reading, and it is what a
//!   program with `∅ ∈ R` gets.
//! - **Globally** the predicate is *dead*, the universe `H` and the oracle the
//!   node function. The seed is the maximal alive states the run met, each
//!   extended by every candidate with none alive. Where every state met was so
//!   extended, the seed is every maximal alive state and `N` is its dual with
//!   nothing asked: again the state route's reading. A transversal is asked
//!   of the memory of what is known dead first, closed under what the run
//!   knows: its node holds the closure of every state whose first
//!   commitment it holds (L3), so where that closure holds a set known dead,
//!   by a death or a denial (L1, L8), the transversal is dead with no
//!   saturation.
//!
//! ### What it never does
//!
//! It never decides a node another route could read otherwise: every node is
//! [`crate::region`]'s, through [`crate::explore`]'s node cache, and every
//! skip is a licence the read-out names. It is not taken on a program the
//! static check flags, nor where root is dead.
//!
//! ### Where it stops
//!
//! Each step is afforded against the budget as every tree step is. The
//! borders are found in order, `R`'s states, then the candidates dead alone
//! (whose closures every later step reads), then each state's class, then
//! `N`, so a run the budget stops has finished every border before the one
//! it stopped in, and says which (`Border`).
//!
//! ### Measured, and taken by no question
//!
//! No question of `ein solve` takes this route: describe and `--count` take
//! the state route. [`crate::explore::describe_by`] and the price list's
//! `--borders` call it, which measure it against the state route, and
//! `ein-infer/tests/borders.rs` holds its description to the state route's
//! and to the oracle's. What it costs is two borders the state route reads
//! for nothing once it has met every alive state: where root owes, `N` with
//! its dual the maximal alive sets, each transversal of the dualisation a
//! node unless the memory decides it; and a state reached by many minimal
//! commitments, each of which the class's dualisation asks.

use rustc_hash::FxHashSet;

use crate::count::Count;
use crate::error::SolveError;
use crate::explore::{Found, Mode, Space, Step, Stopped};
use crate::transversal::{Bits, count_hitting, minimal_transversals};
use crate::tree::Search;

/// Branches a class's count may take before it is counted from the states
/// below it: the state route's own bound ([`crate::states`]).
const COUNT_LIMIT: u64 = 1 << 16;

/// The largest step the singles take, as the tree's.
const BATCH: usize = 1024;

/// Which border a run was finding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Border {
    /// `R`'s states: the tree, then every state of `R` extended.
    States,
    /// Each state's class.
    Classes,
    /// Root's one-up extensions, for the candidates dead alone.
    Singles,
    /// `N` and the maximal alive sets.
    Global,
}

/// One state's class: the commitments that reach it.
#[derive(Clone, Debug)]
pub(crate) struct Classed {
    /// Its maximal closed proper subsets, as complements against it: the
    /// edges every commitment of the class meets. Minimal, sorted.
    pub(crate) edges: Vec<Bits>,
    /// Its minimal generators, by size and then by candidates.
    pub(crate) generators: Vec<Box<[u32]>>,
    /// How many commitments reach it.
    pub(crate) count: Count,
    /// Found with no node asked: every state met inside it was extended.
    pub(crate) read: bool,
}

/// The global border: `N` and the maximal alive sets.
#[derive(Clone, Debug, Default)]
pub(crate) struct Global {
    /// The maximal alive states, as indices of the search's states, by size
    /// and then by candidates.
    pub(crate) maximal_alive: Vec<u32>,
    /// `N`, by size and then by candidates.
    pub(crate) nogoods: Vec<Box<[u32]>>,
    /// Found with no node asked: every state met was extended.
    pub(crate) read: bool,
    /// Transversals the memory decided dead, with no saturation.
    pub(crate) known: u64,
    /// Rounds of dualise and advance.
    pub(crate) rounds: u64,
}

/// How far the global border's dualisation had got: what a run the budget
/// stopped in it had established.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Met {
    /// Maximal alive sets found.
    pub(crate) maximal_alive: u64,
    /// Transversals found dead: each a member of `N`.
    pub(crate) dead: u64,
    /// Transversals in the round it stopped in.
    pub(crate) transversals: u64,
    /// Rounds begun.
    pub(crate) rounds: u64,
}

/// What the border route found, and what each border cost.
pub(crate) struct Borders {
    pub(crate) search: Search,
    /// The tree's answer to `-n` with no `N`: its licences, `exhausted`,
    /// `stopped`.
    pub(crate) found: Found,
    /// `R`'s states met, as indices of the search's states, by size and then
    /// by candidates.
    pub(crate) r: Vec<u32>,
    /// Per state of [`Self::r`], its class, where it was found.
    pub(crate) classes: Vec<Option<Classed>>,
    /// The candidates dead alone, where they were found.
    pub(crate) dead_alone: Option<Vec<u32>>,
    /// `N` and the maximal alive sets, where they were found.
    pub(crate) global: Option<Global>,
    /// Why the run stopped, and in which border.
    pub(crate) stopped: Option<(Border, Stopped)>,
    /// How far the global border had got, where the run stopped in it.
    pub(crate) met: Met,
    /// Per border, the work it took: saturations and facts beyond root.
    pub(crate) work: [(u64, u64); 4],
}

impl Borders {
    /// Every border the question needs was found.
    pub(crate) fn exhausted(&self) -> bool {
        self.stopped.is_none()
    }
}

fn refuse(what: &str) -> SolveError {
    SolveError::Unsupported(format!(
        "{what} — the static verdict's premise failed; run `ein oracle` on this program"
    ))
}

/// `(|x|, x)` as a key: by size, then by members.
fn key(x: &Bits) -> (u32, Box<[u32]>) {
    (x.len(), x.to_set())
}

/// The maximal sets of `sets`, each once, by size and then by members.
fn maximal(sets: Vec<Bits>) -> Vec<Bits> {
    let mut v = sets;
    v.sort_by_key(|x| std::cmp::Reverse(key(x)));
    v.dedup();
    let mut out: Vec<Bits> = Vec::new();
    for x in v {
        if !out.iter().any(|m| x.is_subset(m)) {
            out.push(x);
        }
    }
    out.sort_by_key(key);
    out
}

/// **Describe by borders**: `mode` is describe's, for the tree's order;
/// `nogoods` whether `N` is taken. Root must be alive and the program
/// monotone, which the caller has checked.
pub(crate) fn describe(
    sp: &mut Space<'_>,
    mode: Mode,
    nogoods: bool,
) -> Result<Borders, SolveError> {
    let table = sp.route_alternatives()?;
    let root = match sp.eval(&[Box::default()])? {
        Ok(mut v) => v.pop().expect("one"),
        Err(stop) => {
            return Err(SolveError::Unsupported(format!(
                "the border route stopped before root's node: {stop:?}"
            )));
        }
    };
    if root.dead {
        return Err(refuse("root's node is dead and root's fixpoint is not"));
    }
    let search = Search::new(sp, table, &root, mode)?;
    let mut b = Borders {
        search,
        found: Found::default(),
        r: Vec::new(),
        classes: Vec::new(),
        dead_alone: None,
        global: None,
        stopped: None,
        met: Met::default(),
        work: [(0, 0); 4],
    };
    let mut at = sp.work();
    let mut charge = |b: &mut Borders, border: Border, sp: &Space<'_>| {
        let now = sp.work();
        let w = &mut b.work[border as usize];
        w.0 += now.0 - at.0;
        w.1 += now.1 - at.1;
        at = now;
    };

    // R's states: the tree, then every state of R extended.
    let found = crate::tree::find(sp, &mut b.search, u64::MAX)?;
    charge(&mut b, Border::States, sp);
    let s = &b.search;
    let mut r = s.r_met.clone();
    r.sort_by(|&x, &y| {
        let (a, c) = (&s.st[x as usize].closure, &s.st[y as usize].closure);
        (a.len(), a).cmp(&(c.len(), c))
    });
    b.r = r;
    let stop = found.stopped.clone();
    b.found = found;
    if let Some(stop) = stop {
        b.stopped = Some((Border::States, stop));
        return Ok(b);
    }
    let met_r = b.search.r_met.len();

    // The candidates dead alone: root's state extended by every candidate.
    match singles(sp, &mut b.search)? {
        Ok(d) => b.dead_alone = Some(d),
        Err(stop) => {
            charge(&mut b, Border::Singles, sp);
            b.stopped = Some((Border::Singles, stop));
            return Ok(b);
        }
    }
    charge(&mut b, Border::Singles, sp);

    // Each state's class.
    for t in b.r.clone() {
        match class(sp, &mut b.search, t)? {
            Ok(c) => b.classes.push(Some(c)),
            Err(stop) => {
                charge(&mut b, Border::Classes, sp);
                b.stopped = Some((Border::Classes, stop));
                return Ok(b);
            }
        }
    }
    charge(&mut b, Border::Classes, sp);

    // N and the maximal alive sets, where N is asked or R is empty and its
    // frontier is the maximal alive sets.
    if nogoods || b.r.is_empty() {
        match global(sp, &mut b.search, &mut b.met)? {
            Ok(g) => {
                let singles: Vec<u32> = g
                    .nogoods
                    .iter()
                    .filter(|x| x.len() == 1)
                    .map(|x| x[0])
                    .collect();
                if b.dead_alone.as_ref() != Some(&singles) {
                    return Err(refuse(
                        "N's singletons are not the candidates dead alone (L1: dead is \
                         upward-closed)",
                    ));
                }
                b.global = Some(g);
            }
            Err(stop) => {
                charge(&mut b, Border::Global, sp);
                b.stopped = Some((Border::Global, stop));
                return Ok(b);
            }
        }
        charge(&mut b, Border::Global, sp);
    }
    if b.search.r_met.len() != met_r {
        return Err(refuse(
            "a state of R met after the tree was exhausted (L5: the tree's leaves cover R)",
        ));
    }
    Ok(b)
}

/// **State `t`'s class**, by dualise and advance inside `2^t` (the module's
/// § Dualise and advance): its maximal closed proper subsets and its minimal
/// generators, and how many commitments reach it.
fn class(sp: &mut Space<'_>, s: &mut Search, t: u32) -> Step<Classed> {
    let n = s.n;
    let sb = s.st[t as usize].bits.clone();
    // The closed proper subsets of `t` the run met, and whether every one of
    // them was extended by every candidate.
    let inside: Vec<u32> = (0..s.st.len() as u32)
        .filter(|&u| u != t && s.st[u as usize].bits.is_subset(&sb))
        .collect();
    let complete = inside.iter().all(|&u| s.untried(u).is_empty());
    let mut below: Vec<Bits> = maximal(
        inside
            .iter()
            .map(|&u| s.st[u as usize].bits.clone())
            .collect(),
    );
    let mut generates: FxHashSet<Bits> = FxHashSet::default();
    let gens: Vec<Bits> = loop {
        let edges: Vec<Bits> = below.iter().map(|m| sb.minus(m)).collect();
        let ts = minimal_transversals(n, &edges);
        if complete {
            break ts;
        }
        let mut fresh: Vec<Bits> = Vec::new();
        for tr in &ts {
            if generates.contains(tr) || fresh.iter().any(|m| tr.is_subset(m)) {
                continue;
            }
            if known_closure(s, tr) == sb {
                // Its node holds every state whose first commitment it
                // holds (L3), and those make up the state: it generates it.
                generates.insert(tr.clone());
                continue;
            }
            let set = tr.to_set();
            let u = match s.eval_set(sp, set.clone(), set)? {
                Ok(Some(u)) => u,
                Ok(None) => return Err(refuse("a subset of an alive state is dead (L1)")),
                Err(stop) => return Ok(Err(stop)),
            };
            if s.st[u as usize].bits == sb {
                generates.insert(tr.clone());
                continue;
            }
            // Outside: climbed inside `t` to a maximal closed proper subset.
            let mut c = u;
            for x in sb.minus(&s.st[u as usize].bits).iter() {
                if s.st[c as usize].bits.contains(x) {
                    continue;
                }
                let v = match s.extend(sp, c, &[x])? {
                    Ok(v) => v[0],
                    Err(stop) => return Ok(Err(stop)),
                };
                match v {
                    Some(v) if s.st[v as usize].bits != sb => c = v,
                    Some(_) => {}
                    None => return Err(refuse("a subset of an alive state is dead (L1)")),
                }
            }
            fresh.push(s.st[c as usize].bits.clone());
        }
        if fresh.is_empty() {
            break ts;
        }
        below.extend(fresh);
        below = maximal(below);
    };
    let edges: Vec<Bits> = below.iter().map(|m| sb.minus(m)).collect();
    let count = match count_hitting(&sb, &edges, COUNT_LIMIT) {
        Some(v) => Count::of(v),
        None => count_below(&sb, &below),
    };
    Ok(Ok(Classed {
        edges: crate::transversal::minimal_edges(&edges),
        generators: gens.iter().map(Bits::to_set).collect(),
        count,
        read: complete,
    }))
}

/// `|{C ⊆ s : C ⊄ m for every m}|`, the sets of `2^s` inside no `m`, where
/// the branching count passed its bound: `2^|s|` less the union of the
/// `2^m`, by inclusion and exclusion over the intersections, merged by set.
/// Exact; its work grows with the number of distinct intersections.
fn count_below(s: &Bits, below: &[Bits]) -> Count {
    // The intersections of every non-empty subfamily, with their sign,
    // merged by set: a sum over distinct sets.
    let mut terms: Vec<(Bits, i64)> = Vec::new();
    for m in below {
        let mut next: Vec<(Bits, i64)> = vec![(m.clone(), 1)];
        for (x, c) in &terms {
            next.push((x.and(m), -c));
        }
        for (x, c) in next {
            match terms.iter_mut().find(|(y, _)| *y == x) {
                Some((_, d)) => *d += c,
                None => terms.push((x, c)),
            }
        }
        terms.retain(|(_, c)| *c != 0);
    }
    let mut plus = Count::pow2(s.len());
    let mut minus = Count::zero();
    for (x, c) in &terms {
        let p = Count::pow2(x.len());
        for _ in 0..c.unsigned_abs() {
            if *c > 0 {
                minus.add(&p);
            } else {
                plus.add(&p);
            }
        }
    }
    plus.checked_sub(&minus)
        .expect("a class holds no fewer commitments than none")
}

/// **The candidates dead alone**: root's state extended by every candidate
/// it was not extended by, each step afforded.
fn singles(sp: &mut Space<'_>, s: &mut Search) -> Step<Vec<u32>> {
    let root = s.root();
    loop {
        let rest = s.untried(root);
        if rest.is_empty() {
            break;
        }
        let take = rest.len().min(BATCH);
        if let Err(stop) = s.extend(sp, root, &rest[..take])? {
            return Ok(Err(stop));
        }
    }
    let rb = s.st[root as usize].bits.clone();
    Ok(Ok((0..s.n as u32)
        .filter(|&b| !rb.contains(b) && s.dead.knows_extension(&rb, b))
        .collect()))
}

/// **What `x`'s node is known to hold** with no saturation: `x`, root's
/// state, and the closure of every state met whose first commitment it
/// holds, until nothing more is added. The node of `x` holds the fixpoint of
/// every commitment inside it (a monotone state), and a commitment's state is
/// its closure's (L3), so each closure added is in `cl(x)`: the result lies
/// between `x` and `cl(x)`, and its node is `x`'s.
fn known_closure(s: &Search, x: &Bits) -> Bits {
    let mut closed = x.or(&s.st[s.root() as usize].bits);
    loop {
        let mut grew = false;
        for st in &s.st {
            if !st.bits.is_subset(&closed) && st.path_bits.is_subset(&closed) {
                closed = closed.or(&st.bits);
                grew = true;
            }
        }
        if !grew {
            return closed;
        }
    }
}

/// Whether `x` is dead by what the run knows: what its node is known to hold
/// ([`known_closure`]) holds a set known dead (L1, L8).
fn known_dead(s: &mut Search, x: &Bits) -> bool {
    let closed = known_closure(s, x);
    s.pruned(&closed)
}

/// **`N` and the maximal alive sets**, by dualise and advance (the module's
/// § Dualise and advance).
fn global(sp: &mut Space<'_>, s: &mut Search, met_so_far: &mut Met) -> Step<Global> {
    let n = s.n;
    let full = Bits::full(n);
    let met = s.st.len() as u32;
    let extended = |s: &Search, u: u32| s.untried(u).is_empty();
    let complete = (0..met).all(|u| extended(s, u));
    let mut g = Global {
        read: complete,
        ..Global::default()
    };
    // The seed: the maximal alive states met, every extension of each dead.
    let mut tops: Vec<u32> = (0..met)
        .filter(|&u| extended(s, u) && s.st[u as usize].up.iter().all(|&v| v == u))
        .collect();
    if tops.is_empty() {
        let root = s.root();
        match s.climb(sp, root)? {
            Ok(t) => tops.push(t),
            Err(stop) => return Ok(Err(stop)),
        }
    }
    let mut dead: FxHashSet<Bits> = FxHashSet::default();
    let ts = loop {
        g.rounds += 1;
        let edges: Vec<Bits> = tops
            .iter()
            .map(|&u| full.minus(&s.st[u as usize].bits))
            .collect();
        let ts = minimal_transversals(n, &edges);
        if complete {
            break ts;
        }
        let mut fresh: Vec<u32> = Vec::new();
        for t in &ts {
            *met_so_far = Met {
                maximal_alive: (tops.len() + fresh.len()) as u64,
                dead: dead.len() as u64,
                transversals: ts.len() as u64,
                rounds: g.rounds,
            };
            if dead.contains(t) || fresh.iter().any(|&u| t.is_subset(&s.st[u as usize].bits)) {
                continue;
            }
            if known_dead(s, t) {
                g.known += 1;
                dead.insert(t.clone());
                continue;
            }
            let set = t.to_set();
            let u = match s.eval_set(sp, set.clone(), set)? {
                Ok(Some(u)) => u,
                Ok(None) => {
                    dead.insert(t.clone());
                    continue;
                }
                Err(stop) => return Ok(Err(stop)),
            };
            let top = match s.climb(sp, u)? {
                Ok(top) => top,
                Err(stop) => return Ok(Err(stop)),
            };
            if tops.contains(&top) || fresh.contains(&top) {
                return Err(refuse(
                    "dualise and advance met a maximal alive state twice",
                ));
            }
            fresh.push(top);
        }
        if fresh.is_empty() {
            break ts;
        }
        tops.extend(fresh);
    };
    tops.sort_by(|&a, &b| {
        let (x, y) = (&s.st[a as usize].closure, &s.st[b as usize].closure);
        (x.len(), x).cmp(&(y.len(), y))
    });
    g.maximal_alive = tops;
    g.nogoods = ts.iter().map(Bits::to_set).collect();
    Ok(Ok(g))
}
