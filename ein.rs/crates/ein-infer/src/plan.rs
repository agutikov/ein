//! The compiled form of a `(rule, activator)` pair — plan bytecode.
//!
//! ein.py's `JoinPlan` (`ein/inference/compile.py`) was a
//! frozen dataclass holding tuples of `Scan` / `Join` / `Guard` /
//! `AbsentGuard` opcodes whose slots are **raw IR nodes**, dispatched by
//! `isinstance` at every unification. This is the same shape with the encoding
//! changed ([design/05](../../../../plans/m1a_rust/design/05_matcher.md) §2):
//! flat arenas indexed by [`Span`], a slot that is one of four `u32`-sized
//! things, and every variable resolved to a **register** at compile time.
//!
//! ### Register spaces
//!
//! A plan has **one** register space, shared by every disjunct, laid out as:
//!
//! ```text
//! [0 .. n_seed)   the activator binding, pre-bound at match start
//! [n_seed .. )    every free variable, in first-encounter order
//! ```
//!
//! Sharing one space across disjuncts is safe because a match never spans
//! disjuncts and the trail is fully unwound between them; it is *useful*
//! because the `:assert` templates are compiled once per plan and have to
//! resolve against whichever disjunct produced the match.
//!
//! A [`NafGuard`]'s sub-plan gets a space of its own, because it is not
//! evaluated in the parent's environment: the boundary runs it under
//! `project(bindings, scope)` — the parent's bindings **restricted to the
//! guard's scope** — and a variable outside that scope must be free even when
//! the parent has since bound it. [`NafGuard::scope_of`] is that projection,
//! resolved to register pairs. Guards nested inside a guard share their
//! enclosing sub-plan's space, because `_run_steps` passes them the same
//! binding dict.
//!
//! ### Registers are not an order
//!
//! `Provenance.bindings` is CPython dict-insertion order, i.e. the order the
//! matcher first bound each variable — which the matcher's trail records
//! directly. Register *numbers* are therefore free to be assigned in whatever
//! order the compiler encounters them, and nothing observable reads them.

use ein_core::{Symbol, Value};
use ein_ir::NodeId;

use crate::predicates::Pred;

/// A slice of one of a [`Plan`]'s arenas.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Span {
    pub start: u32,
    pub len: u32,
}

impl Span {
    pub const EMPTY: Span = Span { start: 0, len: 0 };

    pub fn range(self) -> std::ops::Range<usize> {
        self.start as usize..(self.start + self.len) as usize
    }

    pub fn is_empty(self) -> bool {
        self.len == 0
    }

    pub fn len(self) -> usize {
        self.len as usize
    }
}

/// A variable's slot in the register file.
pub type Reg = u16;

/// The register-file ceiling.
///
/// A fixed-size array is what keeps the inner loop allocation-free, so the
/// count has to be bounded somewhere. 256 distinct variables in one `:match`
/// is two orders of magnitude past anything in the corpus (the widest rule in
/// `stdlib/` binds seven), and overflowing it is a [`CompileError`] rather
/// than a panic — ein.py has no such limit, so this is the port's one
/// compile-time bound and it says so out loud.
pub const MAX_REGS: usize = 256;

/// A compiled argument position.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Slot {
    /// A free variable: bind on first encounter, compare afterwards.
    Reg(Reg),
    /// An `Atom` / `Int` literal, or a parameter the activator bound.
    Const(Value),
    /// A sub-pattern matched against a `Fact`-valued argument (Q40 option A).
    Nested { rel: Symbol, slots: Span },
    /// An IR node `_slot` returned unchanged — a `String`, a `Wildcard`, a
    /// `Range`, a `KwPair`, or an `SForm` whose head is neither an atom nor a
    /// bound parameter.
    ///
    /// ein.py's unifier falls through to `slot == arg`, and no IR node is ever
    /// equal to a `str` / `int` / `Fact`, so an opaque slot **never matches**.
    /// It is kept (rather than collapsed to a "never" marker) because it is
    /// what the plan-shape diff prints, and a port that silently agreed on
    /// "never" would agree for the wrong reason.
    Opaque(NodeId),
}

/// One step of a disjunct's positive program.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    /// A relation lookup — ein.py's `Scan` (no shared variables) or `Join`.
    Rel(RelStep),
    /// A built-in predicate evaluated against the current bindings.
    Guard { pred: Pred, args: Span },
    /// A **nested** `(absent …)`: what a `forall` desugars to. Top-level ones
    /// are lifted to [`NafGuard`]s by `split_naf` and never appear here.
    Absent { sub: Span },
}

/// A `Scan` / `Join` — identical to the unifier, split for the trace.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RelStep {
    /// `true` for a `Join`: some slot names a variable an earlier premise
    /// already mentioned. Informational in ein.py too — the unifier treats
    /// both identically — but it is in the compiled shape, so it is compared.
    pub join: bool,
    pub rel: Symbol,
    pub slots: Span,
    /// The ordered candidate probes for this step — see [`Probe`].
    pub probe: Span,
    /// The shared variables, sorted by name. ein.py keeps a `frozenset`, whose
    /// order is not reproducible; every reader of it sorts.
    pub shared: Span,
}

/// A slot the participation index *could* be keyed on at this step.
///
/// ein.py's `_candidates` walks `arg_slots` left to right and narrows on the
/// first slot whose value is known, skipping two shapes: a `NestedPattern`
/// (never keyed) and a variable bound to a nested `Fact` (not keyed).
///
/// The first skip is static and is applied here — an opaque or nested slot
/// never produces a `Probe`. The other two conditions are **not** static, and
/// [design/05](../../../../plans/m1a_rust/design/05_matcher.md) §2's claim that
/// they are is the one thing in that section this stage had to correct:
///
/// - whether a register is bound at this step depends on the *entry point* —
///   `run_seeded` removes a step from the sequence and binds it first, so a
///   register unbound at step 3 in a full run is bound there in a run seeded
///   at step 5;
/// - whether a bound register holds a `Fact` depends on the data — a variable
///   in a `(not ?x)` premise binds to a nested fact on some rows and to a name
///   on others.
///
/// So the compile-time win is *narrowing the scan*, not eliminating it: the
/// runtime walks this pre-filtered list — no type dispatch, no `arg_slots`
/// re-walk — and takes the first entry that is a constant or a bound non-fact
/// register. That is what `_candidates` computes, by construction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Probe {
    /// The argument position this probe keys on.
    pub slot: u16,
    /// The position *inside* the nested fact at `slot`, or
    /// [`SlotKey::DIRECT`](ein_core::SlotKey::DIRECT) when the probe keys on
    /// the argument itself.
    ///
    /// T1a.6.3.0. A `Nested` slot contributes no direct key — there is no
    /// single value at that position — but its own slots do, and the index
    /// has held them since the same task. This is the compile-time half:
    /// `(not (?R ?b ?i))` with `?b` already bound probes
    /// `(not, slot 0, inner 0) = ?b` instead of walking `not`'s extent.
    pub inner: u16,
    pub src: ProbeSrc,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProbeSrc {
    Const(Value),
    Reg(Reg),
}

/// A guard argument, pre-resolved against the register space.
///
/// A `Guard`'s args are **raw IR nodes** in ein.py — `_compile_premise` emits
/// `Guard(predicate, args=node.args)` without running `_slot` — so they
/// resolve against the *runtime* environment, seeds included, rather than
/// against compile-time substitution. That asymmetry is load-bearing: it is
/// why `split_naf` seeds every guard's scope with the rule's parameters
/// (`compile.split_naf`'s docstring), and getting it wrong makes an
/// `(eq ?y ?PARAM)` inside an `(absent …)` resolve `?PARAM` to nothing.
///
/// Resolving the node to a register *is* that runtime lookup, moved to
/// compile time without changing when it is answered.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GuardArg {
    pub kind: GuardArgKind,
    /// The node it came from — what the plan-shape diff prints, because ein.py
    /// stores the node itself.
    pub node: NodeId,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GuardArgKind {
    /// A `Var`: its bound value, or "unbound" — which `resolve_leaf`'s lenient
    /// policy renders as Python's `None`, equal only to another `None`.
    Reg(Reg),
    /// An `Atom` (resolving to its **name**) or an `Int` (to its value).
    Const(Value),
    /// Anything else — a `String`, a `Wildcard`, an `SForm` — which
    /// `resolve_leaf` returns as-is, so it compares by IR-node equality.
    Node,
}

/// A top-level `(absent …)` lifted out of a disjunct — S1.21.8.
///
/// The closure runs purely positive plans; the guard is judged on the
/// closure/world boundary instead, against a positive fixpoint.
#[derive(Clone, Debug)]
pub struct NafGuard {
    /// The sub-plan's steps, in the plan's step arena.
    pub sub: Span,
    /// The size of the sub-plan's own register space.
    pub n_regs: u16,
    /// Relation steps in the query, nested guards included — see
    /// [`Disjunct::n_slots`].
    pub n_slots: u16,
    /// `reg_names[r]` for that space — the sub-plan's variables are *not* the
    /// parent's, so it carries its own table.
    pub reg_names: Box<[Symbol]>,
    /// `scope_of[r]` is the **parent** register a sub-plan register is
    /// projected from, or `None` for a variable local to the query.
    ///
    /// This is `world.project(bindings, scope)` resolved: the guard was
    /// written where its scope was in force, and lifting it must not silently
    /// gain the bindings of premises that followed it — or
    /// `(and (absent (P ?x)) (Q ?x))` would quietly become
    /// `(and (Q ?x) (absent (P ?x)))`.
    pub scope_of: Box<[Option<Reg>]>,
    /// The variables bound before the guard, **seeded with the rule's
    /// parameters** — ein.py's `NafGuard.scope`, sorted, kept for the shape
    /// diff. `scope_of` is the executable form.
    pub scope: Box<[Symbol]>,
    /// Every relation the negative query reads, nested guards included —
    /// the boundary's invalidation key. Sorted: ein.py keeps a `frozenset`
    /// and both readers (`_watch_stamp`, the `park`/`retire` events) sort it.
    pub watched: Box<[Symbol]>,
    /// The query is purely positive, so its match set only grows: once it
    /// finds a match it finds one forever, and a candidate it rejects is dead
    /// rather than waiting. False for a *nested* absent, which can flip from
    /// failing to passing as the KB grows.
    pub monotone: bool,
}

/// One `(or …)` branch: a positive program plus the guards lifted out of it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Disjunct {
    pub steps: Span,
    pub guards: Span,
    /// Top-level relation steps — the length of the `premises` tuple a match
    /// of this disjunct reports.
    pub n_premises: u16,
    /// A **structural** encoding of this disjunct's guards, into
    /// [`Plan::guard_keys`].
    ///
    /// The saturator's `_seen` set is keyed on `(binding_key, guards)`, and in
    /// ein.py `guards` is a tuple of frozen dataclasses — compared **by
    /// value**. So two `(or …)` disjuncts whose guards happen to be the same
    /// query really do collapse there, and a port that keyed on the disjunct
    /// index would enqueue one candidate twice.
    ///
    /// Keying on the guards rather than on the disjunct is also the S1.22.0
    /// fix in the other direction: without it, two disjuncts with equal
    /// bindings but *different* guards collide, only the first is ever
    /// admitted, and — because a failing monotone guard retires its candidate
    /// — a disjunct whose guards would have passed is masked permanently.
    /// `(or …)` became order-dependent.
    pub guard_key: Span,
    /// Relation steps including those inside nested `(absent …)` queries — the
    /// size of the premise-slot array a run needs. A nested query writes into
    /// the slots the enclosing walk has not reached yet and every one of them
    /// is overwritten before the emit reads it, so the array is sized for the
    /// deepest write rather than the reported length.
    pub n_slots: u16,
}

/// The compiled `(rule, activator)` pair.
#[derive(Clone, Debug)]
pub struct Plan {
    pub rule: Symbol,
    /// `plan.activator_args` — the activator's **string** arguments only.
    /// The cache key stringifies *all* of them, and that asymmetry is
    /// Q-M1a.8: two activators differing only in an `int` argument share a
    /// binding key. Reproduced, not fixed.
    pub activator_args: Box<[Symbol]>,
    /// The activator binding, materialised: register `i` holds `seed[i].1`
    /// at match start, and the name is what provenance renders.
    pub seed: Box<[(Symbol, Value)]>,
    pub disjuncts: Box<[Disjunct]>,
    /// One `:assert` conclusion template per top-level conjunct (A13
    /// multi-assert). Slots into [`Plan::slots`].
    pub asserts: Box<[Slot]>,
    pub why: Option<Symbol>,
    /// The plan's register-space size, seeds included.
    pub n_regs: u16,
    /// `reg_names[r]` — for `Provenance.bindings` and the unbound-var error.
    pub reg_names: Box<[Symbol]>,

    // ── Arenas ────────────────────────────────────────────────────
    pub steps: Box<[Step]>,
    pub slots: Box<[Slot]>,
    pub guards: Box<[NafGuard]>,
    pub probes: Box<[Probe]>,
    pub shared: Box<[Symbol]>,
    pub guard_args: Box<[GuardArg]>,
    /// Flat structural encodings of the disjuncts' guard tuples — see
    /// [`Disjunct::guard_key`].
    pub guard_keys: Box<[u32]>,
}

impl Plan {
    pub fn steps(&self, span: Span) -> &[Step] {
        &self.steps[span.range()]
    }

    pub fn slots(&self, span: Span) -> &[Slot] {
        &self.slots[span.range()]
    }

    pub fn guards(&self, span: Span) -> &[NafGuard] {
        &self.guards[span.range()]
    }

    pub fn probes(&self, span: Span) -> &[Probe] {
        &self.probes[span.range()]
    }

    pub fn shared(&self, span: Span) -> &[Symbol] {
        &self.shared[span.range()]
    }

    pub fn guard_args(&self, span: Span) -> &[GuardArg] {
        &self.guard_args[span.range()]
    }

    pub fn guard_key(&self, span: Span) -> &[u32] {
        &self.guard_keys[span.range()]
    }

    /// The first conclusion template — ein.py's `assert_template`, the
    /// back-compat reader for the single-assert consumers (closure/NAF
    /// analysis, hrule).
    pub fn assert_template(&self) -> Option<Slot> {
        self.asserts.first().copied()
    }

    /// True iff any disjunct carries an `(absent …)` guard.
    pub fn has_naf(&self) -> bool {
        self.disjuncts.iter().any(|d| !d.guards.is_empty())
    }
}

/// An index into a [`PlanMemo`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PlanId(pub u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_span_is_a_range() {
        let s = Span { start: 3, len: 2 };
        assert_eq!(s.range(), 3..5);
        assert!(Span::EMPTY.is_empty());
    }

    #[test]
    fn a_register_starts_unbound() {
        let regs = [Value::UNBOUND; 4];
        assert!(regs.iter().all(|v| v.is_unbound()));
    }
}
