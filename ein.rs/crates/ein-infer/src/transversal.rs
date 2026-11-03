//! Minimal transversals of a hypergraph, and the count of the sets that hit
//! every edge — the state route's arithmetic, M1e2
//! [S1e2.1c.2](../../../../plans/m1e2_correctness/p1e2.1c_the_region_search/s1e2.1c.2_the_lines_by_duality.md).
//!
//! A **transversal** of a family of sets (the *edges*) is a set that meets
//! every edge. The state route asks for the minimal ones twice
//! ([`crate::states`]):
//!
//! - **a state's minimal generators** are the minimal transversals of its
//!   complements against the states below it: `C` reaches the state `S` exactly
//!   when `C ⊆ S` and `C` lies under no closed proper subset of `S`;
//! - **`N`**, the minimal dead sets, are the minimal transversals of the
//!   complements of the maximal alive states: a set is dead exactly when it lies
//!   under no maximal alive one. That is the duality of
//!   [Q-M1e2.7](../../../../plans/m1e2_correctness/open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice),
//!   read from the alive side.
//!
//! ### The algorithm, a decision taken without a ruling
//!
//! **MMCS** (Murakami & Uno, *Discrete Applied Mathematics* 2014): a
//! depth-first search that adds one vertex of an uncovered edge at a time and
//! keeps a vertex only while some edge has it as its sole member in the set
//! (its *critical* edges), so every set it reaches is minimal and every
//! minimal transversal is reached once. It is chosen over Berge's sequential
//! product, whose intermediate families can be far larger than the answer,
//! and over Fredman & Khachiyan's quasi-polynomial dualisation, whose bound
//! matters on inputs far larger than the corpus's. Its cost is measured, per
//! program, on `alive_set_reach.md`'s second half, and the oracle holds its
//! output on every monotone cube it enumerates
//! (`ein-infer/tests/region.rs`), which is what lets the argument stand: the
//! premise is enforced by a test that fails when it stops being true.

/// A set of small integers as a fixed-width bit vector.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct Bits(Box<[u64]>);

impl Bits {
    /// The empty set over `0..n`.
    pub fn empty(n: usize) -> Bits {
        Bits(vec![0u64; n.div_ceil(64).max(1)].into_boxed_slice())
    }

    /// `items`, over `0..n`.
    pub fn of(n: usize, items: &[u32]) -> Bits {
        let mut b = Bits::empty(n);
        for &i in items {
            b.insert(i);
        }
        b
    }

    /// Every member of `0..n`.
    pub fn full(n: usize) -> Bits {
        let mut b = Bits::empty(n);
        for i in 0..n as u32 {
            b.insert(i);
        }
        b
    }

    pub fn insert(&mut self, i: u32) {
        self.0[(i / 64) as usize] |= 1 << (i % 64);
    }

    pub fn remove(&mut self, i: u32) {
        self.0[(i / 64) as usize] &= !(1 << (i % 64));
    }

    pub fn contains(&self, i: u32) -> bool {
        self.0[(i / 64) as usize] >> (i % 64) & 1 == 1
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|&w| w == 0)
    }

    pub fn len(&self) -> u32 {
        self.0.iter().map(|w| w.count_ones()).sum()
    }

    /// `self ⊆ other`.
    pub fn is_subset(&self, other: &Bits) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a & !b == 0)
    }

    pub fn intersects(&self, other: &Bits) -> bool {
        self.0.iter().zip(other.0.iter()).any(|(a, b)| a & b != 0)
    }

    /// `self ∩ other`.
    pub fn and(&self, other: &Bits) -> Bits {
        Bits(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| a & b)
                .collect(),
        )
    }

    /// `self \ other`.
    pub fn minus(&self, other: &Bits) -> Bits {
        Bits(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| a & !b)
                .collect(),
        )
    }

    /// `self ∪ other`.
    pub fn or(&self, other: &Bits) -> Bits {
        Bits(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(a, b)| a | b)
                .collect(),
        )
    }

    /// The members, ascending.
    pub fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.0.iter().enumerate().flat_map(|(w, &word)| {
            let mut x = word;
            std::iter::from_fn(move || {
                if x == 0 {
                    return None;
                }
                let b = x.trailing_zeros();
                x &= x - 1;
                Some(w as u32 * 64 + b)
            })
        })
    }

    /// The members, ascending, as the candidate-index sets the routes use.
    pub fn to_set(&self) -> Box<[u32]> {
        self.iter().collect()
    }
}

/// Every edge that has no proper subset among `edges`, each once.
pub fn minimal_edges(edges: &[Bits]) -> Vec<Bits> {
    let mut v: Vec<Bits> = edges.to_vec();
    v.sort_by(|a, b| (a.len(), a).cmp(&(b.len(), b)));
    v.dedup();
    let mut out: Vec<Bits> = Vec::new();
    for e in v {
        if !out.iter().any(|f| f.is_subset(&e)) {
            out.push(e);
        }
    }
    out
}

/// Every minimal transversal of `edges` over the vertices `0..n`, by MMCS,
/// sorted by size and then by members. No edge at all gives `{∅}`; an empty
/// edge gives none, since nothing meets it.
pub fn minimal_transversals(n: usize, edges: &[Bits]) -> Vec<Bits> {
    let edges = minimal_edges(edges);
    if edges.iter().any(Bits::is_empty) {
        return Vec::new();
    }
    let m = edges.len();
    let mut occ: Vec<Vec<u32>> = vec![Vec::new(); n];
    for (i, e) in edges.iter().enumerate() {
        for v in e.iter() {
            occ[v as usize].push(i as u32);
        }
    }
    let mut uncov = Bits::empty(m);
    for i in 0..m as u32 {
        uncov.insert(i);
    }
    let mut cand = Bits::empty(n);
    for e in &edges {
        cand = cand.or(e);
    }
    let mut search = Mmcs {
        n,
        edges: &edges,
        occ: &occ,
        set: Vec::new(),
        crit: Vec::new(),
        uncov,
        out: Vec::new(),
    };
    search.rec(&mut cand);
    let mut out = search.out;
    out.sort_by_key(|t| (t.len(), t.to_set()));
    out
}

struct Mmcs<'a> {
    n: usize,
    edges: &'a [Bits],
    occ: &'a [Vec<u32>],
    /// The current set, in the order its vertices were added.
    set: Vec<u32>,
    /// Per member of `set`, the edges it alone meets.
    crit: Vec<Bits>,
    /// The edges `set` does not meet.
    uncov: Bits,
    out: Vec<Bits>,
}

impl Mmcs<'_> {
    fn rec(&mut self, cand: &mut Bits) {
        if self.uncov.is_empty() {
            self.out.push(Bits::of(self.n, &self.set));
            return;
        }
        // The uncovered edge with the fewest candidates left.
        let f = self
            .uncov
            .iter()
            .min_by_key(|&e| self.edges[e as usize].and(cand).len())
            .expect("an uncovered edge");
        let c = self.edges[f as usize].and(cand);
        *cand = cand.minus(&c);
        for v in c.iter() {
            let saved_uncov = self.uncov.clone();
            let saved_crit = self.crit.clone();
            let mut mine = Bits::empty(self.edges.len());
            for &e in &self.occ[v as usize] {
                if self.uncov.contains(e) {
                    self.uncov.remove(e);
                    mine.insert(e);
                } else {
                    for c in self.crit.iter_mut() {
                        c.remove(e);
                    }
                }
            }
            if self.crit.iter().all(|c| !c.is_empty()) {
                self.set.push(v);
                self.crit.push(mine);
                self.rec(cand);
                self.set.pop();
                self.crit.pop();
            }
            self.uncov = saved_uncov;
            self.crit = saved_crit;
            cand.insert(v);
        }
    }
}

/// How many subsets of `vars` meet every one of `edges`, each a subset of
/// `vars` — or `None` past `limit` branches, for the caller to count another
/// way. A branch-and-count over the vertices, most-shared first: a vertex in
/// the set removes the edges it meets, a vertex out of it shrinks them, and an
/// edge shrunk to nothing ends the branch with zero.
pub fn count_hitting(vars: &Bits, edges: &[Bits], limit: u64) -> Option<u128> {
    let mut calls = 0u64;
    count(vars.clone(), minimal_edges(edges), limit, &mut calls)
}

fn count(vars: Bits, edges: Vec<Bits>, limit: u64, calls: &mut u64) -> Option<u128> {
    *calls += 1;
    if *calls > limit {
        return None;
    }
    if edges.is_empty() {
        return pow2(vars.len());
    }
    if edges.iter().any(Bits::is_empty) {
        return Some(0);
    }
    // A one-vertex edge forces its vertex in.
    if let Some(unit) = edges.iter().find(|e| e.len() == 1) {
        let v = unit.iter().next().expect("one member");
        let mut rest = vars;
        rest.remove(v);
        let kept: Vec<Bits> = edges.into_iter().filter(|e| !e.contains(v)).collect();
        return count(rest, kept, limit, calls);
    }
    let mut tally: Vec<u32> = vec![0; vars.0.len() * 64];
    for e in &edges {
        for v in e.iter() {
            tally[v as usize] += 1;
        }
    }
    let v = (0..tally.len() as u32)
        .max_by_key(|&v| (tally[v as usize], std::cmp::Reverse(v)))
        .expect("a vertex");
    let mut rest = vars;
    rest.remove(v);
    let taken: Vec<Bits> = edges.iter().filter(|e| !e.contains(v)).cloned().collect();
    let a = count(rest.clone(), minimal_edges(&taken), limit, calls)?;
    let shrunk: Vec<Bits> = edges
        .iter()
        .map(|e| {
            let mut e = e.clone();
            e.remove(v);
            e
        })
        .collect();
    let b = count(rest, minimal_edges(&shrunk), limit, calls)?;
    a.checked_add(b)
}

/// `2^k`, or `None` past `u128`.
pub fn pow2(k: u32) -> Option<u128> {
    (k < 128).then(|| 1u128 << k)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tiny deterministic generator, so the cases are the same every run.
    fn lcg(seed: &mut u64) -> u64 {
        *seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *seed >> 33
    }

    fn brute(n: usize, edges: &[Bits]) -> (Vec<Bits>, u128) {
        let hits = |m: u64| edges.iter().all(|e| e.iter().any(|v| m >> v & 1 == 1));
        let all: Vec<u64> = (0..1u64 << n).filter(|&m| hits(m)).collect();
        let minimal: Vec<u64> = all
            .iter()
            .copied()
            .filter(|&m| (0..n).all(|b| m >> b & 1 == 0 || !hits(m & !(1 << b))))
            .collect();
        let as_bits = |m: u64| {
            let items: Vec<u32> = (0..n as u32).filter(|&b| m >> b & 1 == 1).collect();
            Bits::of(n, &items)
        };
        let mut t: Vec<Bits> = minimal.into_iter().map(as_bits).collect();
        t.sort_by_key(|t| (t.len(), t.to_set()));
        (t, all.len() as u128)
    }

    /// MMCS and the count agree with brute force on 2 000 random hypergraphs.
    #[test]
    fn transversals_and_counts_are_brute_force() {
        let mut seed = 7u64;
        for case in 0..2000 {
            let n = 1 + (lcg(&mut seed) % 9) as usize;
            let m = (lcg(&mut seed) % 7) as usize;
            let edges: Vec<Bits> = (0..m)
                .map(|_| {
                    let items: Vec<u32> = (0..n as u32)
                        .filter(|_| lcg(&mut seed).is_multiple_of(3))
                        .collect();
                    Bits::of(n, &items)
                })
                .collect();
            let (want, count) = brute(n, &edges);
            assert_eq!(minimal_transversals(n, &edges), want, "case {case}");
            assert_eq!(
                count_hitting(&Bits::full(n), &edges, u64::MAX),
                Some(count),
                "case {case}"
            );
        }
    }

    #[test]
    fn the_edge_cases() {
        assert_eq!(minimal_transversals(3, &[]), vec![Bits::empty(3)]);
        assert!(minimal_transversals(3, &[Bits::empty(3)]).is_empty());
        assert_eq!(count_hitting(&Bits::full(3), &[], 10), Some(8));
        assert_eq!(
            count_hitting(&Bits::full(3), &[Bits::empty(3)], 10),
            Some(0)
        );
        let wide = Bits::full(130);
        assert_eq!(wide.len(), 130);
        assert_eq!(wide.iter().last(), Some(129));
    }
}
