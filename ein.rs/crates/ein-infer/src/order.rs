//! **The order of the obligation tree**: which owed instance an owing state
//! branches on, and which owing state is branched next.
//!
//! An order decides the work a question takes and never its answer: under
//! every order here the tree's leaves cover `R` (L5 holds of whichever
//! licensed instance a state branches on), so the verdict, `R`, `N` and
//! what a budget-stopped run claims are one. Which members of `R` a
//! question that stops early lists first is the order's.
//!
//! [`Order::of`] is the order the routes take, a function of the question
//! and of nothing else. The others exist so that the tree can be **held to
//! one answer under every order** (`ein-infer/tests/order.rs`,
//! `explore.rs`) and so that an order can be measured against the others on
//! the same binary. [`with`] chooses one for the calling thread while a
//! closure runs, as [`crate::known_dead::from_deaths_alone`] chooses what
//! the memory learns from. It is no lever: no `(config …)` field, no option
//! of `ein` and no `EIN_*` name reaches it.
//!
//! ### The order the routes take
//!
//! **Which instance.** An owing state branches on one licensed instance it
//! owes, chosen once, by three keys in turn:
//!
//! 1. **the closest to closing a coupling**, once the couplings are read
//!    (below): over the edges that hold the instance, the fewest questions
//!    the state has not paid, the instance's own among them. An edge is a
//!    set of owed instances some conflict or derivation needs a member of
//!    each of, so an instance one short of an edge is one whose every
//!    alternative lets a check fire that could not before;
//! 2. **the fewest alternatives not known dead**, by the run's one memory
//!    ([`crate::known_dead`]), a death or a denial;
//! 3. **the first by rendered text**: the instances in the order of their
//!    alternatives, which are candidates in rendered-text order. Under
//!    `-n N`, of all of `R` or of a line, the first the obligation pass
//!    reports instead, which is the order the program states its facts in.
//!
//! **The couplings** are [`crate::structure::NodeCoupling`]'s edges, read
//! off the whole node, one saturation the run pays for and counts. They are
//! read **when a choice is left tied by the count, the run has saturated
//! [`READ_AFTER`] nodes, and the node costs at most one [`READ_SHARE`]-th of
//! the facts the run has derived**: the node is saturated within that
//! bound; where it needs more it is stopped there, what it derived is
//! counted, and it is tried again when the run has derived four times as
//! much. So a question a few nodes answer reads no structure, and one that
//! reads it has spent twenty times its price already.
//!
//! **When the couplings are read the choices made before are made again**,
//! each by the counts its state had when it first chose, so the couplings
//! move a choice and what the run happened to learn meanwhile does not; and
//! where one falls otherwise the tree starts again from root, keeping every
//! node it computed. Until then key 1 says nothing and keys 2 and 3 decide.
//!
//! **Which state next.** The owing state of the smallest closure, then by
//! closure: every state of `k` candidates before any of `k + 1`.
//!
//! No key reads the interner's ids, and but for `-n N`'s last key none
//! reads the order a program states its facts in.

use std::cell::RefCell;
use std::sync::Arc;

use crate::explore::{Ask, Mode};

/// The saturations a run has made before the couplings are read: one more
/// is then a twentieth of them at most.
pub const READ_AFTER: u64 = 20;

/// The share of the facts a run has derived that the whole node may cost:
/// one twentieth.
pub const READ_SHARE: u64 = 20;

/// How an owing state chooses the instance it branches on, among the
/// licensed instances it owes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Choice {
    /// Fewest alternatives not known dead; among equals the first the
    /// obligation pass reports, which is the order the program states its
    /// facts in. With `denials` off the count reads what a saturation found
    /// dead and not what a state denies.
    Written { denials: bool },
    /// [`Choice::Written`] with the last reported among equals.
    Reversed,
    /// Fewest alternatives not known dead; among equals the first by its
    /// alternatives' rendered text.
    Text,
    /// A fixed pseudo-random permutation of the instances by `seed`, the
    /// count not read: any instance a state owes may come first.
    Shuffled(u64),
    /// Fewest alternatives not known dead, then the most coupled to what
    /// the state has paid, then text.
    OpenCoupled,
    /// The most coupled to what the state has paid, then fewest
    /// alternatives not known dead, then text.
    CoupledOpen,
    /// **By the couplings, once they are read.** Until then fewest
    /// alternatives not known dead; from then on the closest to closing a
    /// coupling, then fewest alternatives not known dead. A state that had
    /// chosen chooses again by the counts it had then, so the couplings
    /// move a choice and what the run learned meanwhile does not. Among
    /// equals the first by text, or with `written` the first the obligation
    /// pass reports.
    Coupled { written: bool },
    /// Fewest alternatives not known dead over one more than the instance's
    /// weight, the deaths it was blamed for; then text. It reads no
    /// structure.
    Learned(Blame),
}

/// Which instances a death under a branch is counted against
/// ([`Choice::Learned`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blame {
    /// The instance branched on.
    Branch,
    /// The instance branched on and every licensed instance the state has
    /// paid.
    Path,
}

/// Which owing state the tree branches next.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Queue {
    /// The smallest closure first, then by closure: every state of `k`
    /// candidates before any of `k + 1`.
    BySize,
    /// The largest closure first, then by closure: a branch is followed
    /// down before its siblings are opened.
    Deepest,
}

/// Edges given from outside the run, over candidates: each question its
/// members ascending, each edge its questions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Given {
    pub questions: Vec<Vec<u32>>,
    pub edges: Vec<Vec<u32>>,
}

/// What a coupled choice reads its edges from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Structure {
    /// Nothing: every instance is as coupled as another.
    None,
    /// The whole node alone ([`crate::structure::NodeCoupling`]), one
    /// saturation, read the first time a choice is left tied once the run
    /// has saturated `after` nodes; the tree then starts again from root
    /// with what it knows. `one_step` takes the edges one application back.
    ///
    /// With a `share` other than 0 the node is read only within one
    /// `share`-th of the facts the run has derived: a node that needs more
    /// is stopped there, what it derived is counted, and it is tried again
    /// when the run has done twice the work.
    Node {
        after: u64,
        one_step: bool,
        share: u64,
    },
    /// Edges the caller read before the run, whose cost is the caller's to
    /// count.
    Given(Arc<Given>),
}

/// One order of the tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Order {
    pub choice: Choice,
    pub queue: Queue,
    pub structure: Structure,
    /// Start the tree again from root, keeping what the run knows, each
    /// time its saturations double from 32: what lets a learned weight
    /// reach the choices made before it was learned.
    pub restarts: bool,
}

impl Order {
    /// The tree as it was before any order was derived: fewest alternatives
    /// not known dead, the first reported among equals, by size.
    pub const WRITTEN: Order = Order {
        choice: Choice::Written { denials: true },
        queue: Queue::BySize,
        structure: Structure::None,
        restarts: false,
    };
}

impl Order {
    /// **The order the routes take for `mode`** (the module's § The order
    /// the routes take).
    pub fn of(mode: Mode) -> Order {
        Order {
            choice: Choice::Coupled {
                written: matches!(mode.ask, Ask::Find(_)),
            },
            queue: Queue::BySize,
            structure: Structure::Node {
                after: READ_AFTER,
                one_step: false,
                share: READ_SHARE,
            },
            restarts: false,
        }
    }
}

thread_local! {
    static ORDER: RefCell<Option<Order>> = const { RefCell::new(None) };
}

/// Run `f` with every tree opened on this thread taking `order`. Restored
/// when `f` returns or unwinds.
pub fn with<T>(order: Order, f: impl FnOnce() -> T) -> T {
    struct Restore(Option<Order>);
    impl Drop for Restore {
        fn drop(&mut self) {
            ORDER.with(|s| *s.borrow_mut() = self.0.take());
        }
    }
    let _restore = Restore(ORDER.with(|s| s.borrow_mut().replace(order)));
    f()
}

/// The order [`with`] chose for trees opened on this thread, where one is
/// running; `None` where the routes take their own ([`Order::of`]).
pub fn chosen() -> Option<Order> {
    ORDER.with(|s| s.borrow().clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::explore::Which;

    #[test]
    fn with_chooses_an_order_only_while_it_runs() {
        assert_eq!(chosen(), None);
        let other = Order {
            choice: Choice::Text,
            queue: Queue::Deepest,
            structure: Structure::None,
            restarts: false,
        };
        let seen = with(other.clone(), || {
            let inner = with(Order::WRITTEN, chosen);
            assert_eq!(inner, Some(Order::WRITTEN));
            chosen()
        });
        assert_eq!(seen, Some(other));
        assert_eq!(chosen(), None);
    }

    #[test]
    fn the_routes_order_is_a_function_of_the_question() {
        let mode = |which, ask| Mode { which, ask };
        let coupled = |written| Choice::Coupled { written };
        let single = Order::of(mode(Which::All, Ask::Single));
        assert_eq!(single.choice, coupled(false));
        assert_eq!(single.queue, Queue::BySize);
        assert_eq!(
            single.structure,
            Structure::Node {
                after: READ_AFTER,
                one_step: false,
                share: READ_SHARE
            }
        );
        assert!(!single.restarts);
        // `-n N`, of all of R or of a line, keeps the reported order among
        // equals, and nothing else differs; every other question is
        // `--single`'s order.
        for which in [Which::All, Which::Minimal, Which::Maximal] {
            let find = Order::of(mode(which, Ask::Find(1)));
            assert_eq!(find.choice, coupled(true));
            assert_eq!(
                (find.queue, find.structure.clone()),
                (single.queue, single.structure.clone())
            );
            for ask in [Ask::Describe, Ask::Single] {
                if which != Which::All || ask == Ask::Single {
                    assert_eq!(Order::of(mode(which, ask)), single);
                }
            }
        }
    }
}
