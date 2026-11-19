//! What a run knows dead without asking the node function: **the one memory
//! every route on a monotone program asks before it saturates a set.**
//!
//! The state route ([`crate::states`]) and the modes' routes
//! (`crate::tree`) both extend a state `S` by a candidate `b`, and the
//! maximal line's dualisation asks about sets of any shape. Each asks here
//! first, and each writes here what it learns. Neither keeps a copy.
//!
//! ### What is in it
//!
//! Per candidate `b`, the states `S` for which `S ∪ {b}` is known dead, from
//! two sources:
//!
//! - **a death**: a route saturated `S ∪ {b}` and its fixpoint derived a
//!   contradiction (`KnownDead::died`);
//! - **a denial**: `S` is an alive state whose own fixpoint holds a fact a
//!   rule refutes `b` beside alone, `(not b)` where the program imports
//!   `std.contradiction` ([`crate::contradiction::Pairs`]), which the node
//!   function reports beside its closure (`crate::region::NodeOut::denied`,
//!   `KnownDead::denies`). No saturation is spent on `S ∪ {b}`.
//!
//! Whatever its source, a set known dead prunes every set that holds it
//! (**L1**, *dead is upward-closed*): `KnownDead::extension` and
//! `KnownDead::inside` answer for `S′ ∪ {b}` and for any `x` with
//! `S ∪ {b} ⊆ x`.
//!
//! ### L8, what a state denies is dead above it
//!
//! **The licence.** On a program the static check calls monotone, let `S` be
//! an alive state whose fixpoint holds `(N b)` for a candidate `b`, where a
//! compiled rule refutes a fact of `b`'s relation and count beside the fact
//! `N` holds it in, alone: `(not b)`, under `std.contradiction`'s `clash-N`.
//! Then the node of `S ∪ {b}` is dead, and by L1 so is every node above it.
//!
//! **The argument**, under Q-M1e.2. Three facts, none of them about a closure
//! or how `(N b)` came to be held:
//!
//! 1. *A node's fixpoint holds its commitment.* The node function writes the
//!    commitment's facts into root's fork before it saturates
//!    ([`crate::region`], § What a node is), so `b` is in the fixpoint of
//!    `S ∪ {b}`.
//! 2. *The state is monotone.* A program with no site has a monotone state:
//!    each node's fixpoint contains its subsets' ([`crate::monotone`], § The
//!    theorem). `(N b)` is a fact of the fixpoint like any other, so the
//!    fixpoint of `S ∪ {b}` holds it.
//! 3. *The rule fires on the two alone*: its match is `b` and `(N b)` and
//!    nothing else, and its firing ends the state, by stating
//!    `:ein inconsistent` or by concluding what a rule that states it matches
//!    alone, `(false)` under `std.core`'s ([`crate::contradiction::Pairs`]).
//!
//! So the fixpoint of `S ∪ {b}` holds `b` and `(N b)`, and the rule ends it:
//! the node is dead. ∎ Where no such rule is compiled, a state denies
//! nothing, and a fact beside its negation is alive.
//!
//! **It is not L1.** L1 needs a dead node below the set, and there is none
//! here: `S` is alive and `{b}` may be. L8 is a second corollary of the same
//! theorem L1 and L3 rest on, the monotone state, and it gives L1 a dead set
//! it then closes upward. **It holds exactly where L1 holds**, so a program
//! the static check flags gets nothing: its routes are the cube's, which ask
//! nothing here.
//!
//! **Through a closure.** The node of `S ∪ {b}` holds more than `S` and `b`:
//! it holds the fixpoint of every commitment inside the set, so every state
//! `U = cl(P ∪ {b})` the run has met for a state `P` under `S` (the monotone
//! state again; L3 names it for a commitment and its closure). So the set is
//! dead as soon as two of the states its node holds meet in a candidate one
//! holds and the other denies: `S` denies a candidate of such a `U`, a `U`
//! denies a candidate of `S`, or one `U` a candidate of another. A route
//! finds that from the states one edge under `S` and tells the memory
//! (`KnownDead::denied_through`); it is L8 on the closed set, and is
//! counted and named as L8. Without it a puzzle whose rules derive one
//! placement from another keeps saturating what its own states exclude:
//! the denial sits one derivation away from the candidate asked about.
//!
//! **What it does not read.** A rule that concludes `(false)` from two facts
//! of which neither holds the other denies nothing, so the same exclusion is
//! spent on one program and not on another according to how its rules are
//! written.
//!
//! ### The premise, enforced
//!
//! - The oracle: on every monotone cube it enumerates, every node above an
//!   alive node and a candidate that node denies is dead
//!   (`ein-infer/tests/region.rs`), and every mode is the oracle with the
//!   denials read (`ein-infer/tests/explore.rs`).
//! - On every run: an alive state that holds a set known dead refuses the run
//!   (`KnownDead::inside_of_alive`, asked by both routes of every state
//!   they meet), and so does a denied extension that is a state already met
//!   alive.
//! - With and without: every question on every corpus program that searches
//!   answers the same with the denials read and with deaths alone
//!   (`ein-infer/tests/known_dead.rs`).
//!
//! ### The switch
//!
//! [`from_deaths_alone`] makes every memory opened on the calling thread
//! while it runs learn from deaths alone: for the test that holds the
//! two to one answer, and for measuring what the denials save. It is no
//! lever: no `(config …)` field, no option of `ein` and no `EIN_*` name
//! reaches it.

use std::cell::Cell;

use rustc_hash::FxHashSet;

use crate::transversal::Bits;

thread_local! {
    static DEATHS_ALONE: Cell<bool> = const { Cell::new(false) };
}

/// Run `f` with every `KnownDead` opened on this thread learning from
/// deaths alone: what a state denies is not read (the module's § The switch).
/// Restored when `f` returns or unwinds.
pub fn from_deaths_alone<T>(f: impl FnOnce() -> T) -> T {
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            DEATHS_ALONE.with(|s| s.set(self.0));
        }
    }
    let _restore = Restore(DEATHS_ALONE.with(|s| s.replace(true)));
    f()
}

/// Whether [`from_deaths_alone`] is running on this thread.
pub fn is_from_deaths_alone() -> bool {
    DEATHS_ALONE.with(Cell::get)
}

/// One set known dead, `state ∪ {b}`, kept under its `b`.
#[derive(Clone, Debug)]
struct Entry {
    state: Bits,
    /// Known by a denial (L8); otherwise by a saturation.
    denial: bool,
    /// Some answer rested on it.
    spent: bool,
}

/// The sets a run knows dead (the module's docs).
#[derive(Clone, Debug)]
pub(crate) struct KnownDead {
    /// Per candidate `b`, the states `S` with `S ∪ {b}` known dead, in the
    /// order learned.
    under: Vec<Vec<Entry>>,
    /// Denials are read: off under [`from_deaths_alone`].
    denials: bool,
    /// The sets `S ∪ {b}` of the denials an answer rested on.
    spent: FxHashSet<Bits>,
}

impl KnownDead {
    /// An empty memory over `n` candidates, reading denials unless
    /// [`from_deaths_alone`] is running on this thread.
    pub(crate) fn new(n: usize) -> KnownDead {
        KnownDead {
            under: vec![Vec::new(); n],
            denials: !is_from_deaths_alone(),
            spent: FxHashSet::default(),
        }
    }

    /// A saturation found `state ∪ {b}` dead.
    pub(crate) fn died(&mut self, state: Bits, b: u32) {
        self.under[b as usize].push(Entry {
            state,
            denial: false,
            spent: false,
        });
    }

    /// A saturation found `set` dead, a set of any shape: kept under its
    /// last candidate. Nothing is kept of ∅, whose death is root's and ends
    /// every route before it asks.
    pub(crate) fn died_set(&mut self, n: usize, set: &[u32]) {
        if let Some((&b, rest)) = set.split_last() {
            self.died(Bits::of(n, rest), b);
        }
    }

    /// The alive state `state` denies `b`: its own fixpoint holds `(not b)`,
    /// beside which a rule refutes `b`,
    /// so `state ∪ {b}` is dead (L8). `false`, and nothing is learned, where
    /// denials are not read or a set already known dead lies inside
    /// `state ∪ {b}`.
    pub(crate) fn denies(&mut self, state: &Bits, b: u32) -> bool {
        if !self.denials || self.knows_extension(state, b) {
            return false;
        }
        self.under[b as usize].push(Entry {
            state: state.clone(),
            denial: true,
            spent: false,
        });
        true
    }

    /// `state ∪ {b}` is a set a denial already said is dead, reached from
    /// another of its states: kept under this one too, as a death a route
    /// saturated is kept under every state it was reached from. Nothing where
    /// denials are not read.
    pub(crate) fn denied_again(&mut self, state: &Bits, b: u32) {
        if self.denials {
            self.under[b as usize].push(Entry {
                state: state.clone(),
                denial: true,
                spent: false,
            });
        }
    }

    /// `state ∪ {b}` is dead by a denial of a state its node holds, which a
    /// route found by closing the set under what it knows (L8, through L3):
    /// learned as a denial and spent as the answer it is. `false`, and
    /// nothing is learned, where denials are not read.
    pub(crate) fn denied_through(&mut self, state: &Bits, b: u32) -> bool {
        if !self.denials {
            return false;
        }
        self.denied_again(state, b);
        let i = self.under[b as usize].len() - 1;
        self.spend(b, i);
        true
    }

    /// `state ∪ {b}` holds a set known dead, read without resting an answer
    /// on it: what an order may count and a check may ask.
    pub(crate) fn knows_extension(&self, state: &Bits, b: u32) -> bool {
        self.under[b as usize]
            .iter()
            .any(|e| e.state.is_subset(state))
    }

    /// [`Self::knows_extension`], counting only what a saturation found
    /// dead: what an order that does not read the denials counts.
    pub(crate) fn knows_extension_by_death(&self, state: &Bits, b: u32) -> bool {
        self.under[b as usize]
            .iter()
            .any(|e| !e.denial && e.state.is_subset(state))
    }

    /// `state ∪ {b}` holds a set known dead, so it is dead and is not
    /// saturated (L1, over a death or over L8's set). A death answers before
    /// a denial does, and a denial that answers is spent
    /// ([`Self::denials_spent`]).
    pub(crate) fn extension(&mut self, state: &Bits, b: u32) -> bool {
        let mut denial: Option<usize> = None;
        for (i, e) in self.under[b as usize].iter().enumerate() {
            if !e.state.is_subset(state) {
                continue;
            }
            if !e.denial {
                return true;
            }
            denial.get_or_insert(i);
        }
        let Some(i) = denial else {
            return false;
        };
        self.spend(b, i);
        true
    }

    /// The denial at `under[b][i]` answered: its set is counted once.
    fn spend(&mut self, b: u32, i: usize) {
        let e = &mut self.under[b as usize][i];
        if !std::mem::replace(&mut e.spent, true) {
            let mut set = e.state.clone();
            set.insert(b);
            self.spent.insert(set);
        }
    }

    /// `x` holds a set known dead: some `S ∪ {b} ⊆ x`. As
    /// [`Self::extension`], for a set of any shape.
    pub(crate) fn inside(&mut self, x: &Bits) -> bool {
        let mut denial: Option<(u32, usize)> = None;
        for b in x.iter() {
            for (i, e) in self.under[b as usize].iter().enumerate() {
                if !e.state.is_subset(x) {
                    continue;
                }
                if !e.denial {
                    return true;
                }
                denial.get_or_insert((b, i));
            }
        }
        let Some((b, i)) = denial else {
            return false;
        };
        self.spend(b, i);
        true
    }

    /// An **alive** state's closure `x` holds a set known dead, which no
    /// monotone program allows (L1, L8): the premise a route refuses its run
    /// over. Nothing is spent by asking.
    pub(crate) fn inside_of_alive(&self, x: &Bits) -> bool {
        x.iter()
            .any(|b| self.under[b as usize].iter().any(|e| e.state.is_subset(x)))
    }

    /// Nothing is known dead: no saturation found a set dead and no state
    /// met denies a candidate.
    pub(crate) fn is_empty(&self) -> bool {
        self.under.iter().all(Vec::is_empty)
    }

    /// How many sets an answer took for dead on a denial: the distinct sets
    /// `S ∪ {b}`, `S` a state that denies `b`, that a route was told are dead,
    /// themselves or inside a larger set, where no death said so. A set two
    /// of its states deny is one. L8 is named in a run's licences exactly
    /// when this is not zero.
    pub(crate) fn denials_spent(&self) -> u64 {
        self.spent.len() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits(items: &[u32]) -> Bits {
        Bits::of(8, items)
    }

    #[test]
    fn a_death_and_a_denial_both_prune_what_holds_them() {
        let mut k = KnownDead::new(8);
        k.died(bits(&[0]), 3);
        assert!(k.denies(&bits(&[1]), 4));
        // The sets themselves, and sets above them.
        assert!(k.extension(&bits(&[0]), 3));
        assert!(k.extension(&bits(&[0, 2]), 3));
        assert!(k.extension(&bits(&[1, 2]), 4));
        assert!(k.inside(&bits(&[0, 3, 5])));
        assert!(k.inside(&bits(&[1, 4, 5])));
        // Not a set that misses the state, or the candidate.
        assert!(!k.extension(&bits(&[2]), 3));
        assert!(!k.inside(&bits(&[0, 1, 5])));
        assert!(k.inside_of_alive(&bits(&[1, 4])));
        assert!(!k.inside_of_alive(&bits(&[1, 3])));
    }

    #[test]
    fn a_denial_is_spent_once_and_only_where_no_death_answers() {
        let mut k = KnownDead::new(8);
        assert!(k.denies(&bits(&[1]), 4));
        assert_eq!(k.denials_spent(), 0, "learned, and nothing asked");
        assert!(k.knows_extension(&bits(&[1]), 4));
        assert_eq!(k.denials_spent(), 0, "an order's count rests no answer");
        assert!(k.extension(&bits(&[1]), 4));
        assert!(k.inside(&bits(&[1, 2, 4])));
        assert_eq!(k.denials_spent(), 1, "one denial, asked twice");
        // The same set under its other state is the same set.
        k.denied_again(&bits(&[4]), 1);
        assert!(k.extension(&bits(&[4]), 1));
        assert_eq!(k.denials_spent(), 1, "one set, two of its states");
        // A set a death already covers learns no denial and spends none.
        k.died(bits(&[0]), 3);
        assert!(!k.denies(&bits(&[0, 2]), 3));
        assert!(k.extension(&bits(&[0, 2]), 3));
        assert_eq!(k.denials_spent(), 1);
        // A death answers before a denial under the same candidate.
        assert!(k.denies(&bits(&[5]), 6));
        k.died(bits(&[5]), 6);
        assert!(k.extension(&bits(&[5]), 6));
        assert_eq!(k.denials_spent(), 1);
    }

    #[test]
    fn from_deaths_alone_reads_no_denial_and_only_while_it_runs() {
        let k = from_deaths_alone(|| {
            assert!(is_from_deaths_alone());
            let mut k = KnownDead::new(8);
            assert!(!k.denies(&bits(&[1]), 4));
            assert!(!k.extension(&bits(&[1]), 4));
            k.died(bits(&[0]), 3);
            assert!(k.extension(&bits(&[0]), 3));
            k
        });
        assert!(!is_from_deaths_alone());
        assert_eq!(k.denials_spent(), 0);
        // A memory keeps what it was opened as.
        let mut k = k;
        assert!(!k.denies(&bits(&[1]), 4));
    }
}
