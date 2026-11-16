//! **The order of the obligation tree**: which owed instance an owing state
//! branches on, and which owing state is branched next.
//!
//! An order decides the work a question takes and never its answer: under
//! every order here the tree's leaves cover `R` (L5 holds of whichever
//! licensed instance a state branches on), so the verdict, `R`, `N` and
//! what a budget-stopped run claims are one. Which members of `R` a
//! question that stops early lists first is the order's.
//!
//! [`Order::default`] is the order the routes take. The others exist so
//! that the tree can be **held to one answer under every order**
//! (`ein-infer/tests/order.rs`) and so that an order can be measured against
//! the others on the same binary. [`with`] chooses one for the calling
//! thread while a closure runs, as [`crate::known_dead::from_deaths_alone`]
//! chooses what the memory learns from. It is no lever: no `(config …)`
//! field, no option of `ein` and no `EIN_*` name reaches it.

use std::cell::RefCell;
use std::sync::Arc;

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

impl Default for Order {
    /// The order the routes take.
    fn default() -> Self {
        Order::WRITTEN
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

/// The order a tree opened on this thread takes: [`with`]'s, or the
/// default.
pub fn current() -> Order {
    ORDER.with(|s| s.borrow().clone()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_chooses_an_order_only_while_it_runs() {
        assert_eq!(current(), Order::default());
        let other = Order {
            choice: Choice::Text,
            queue: Queue::Deepest,
            structure: Structure::None,
            restarts: false,
        };
        let seen = with(other.clone(), || {
            let inner = with(Order::WRITTEN, current);
            assert_eq!(inner, Order::WRITTEN);
            current()
        });
        assert_eq!(seen, other);
        assert_eq!(current(), Order::default());
    }
}
