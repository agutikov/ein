//! `Value` — a fact argument in 4 bytes.
//!
//! A fact argument is a name or a nested fact, and unifying two of them
//! starts by asking which. Here the discriminant is two bits
//! ([design/03](../../../../docs/history/m1a_rust/design/03_data_model.md) §3):
//!
//! ```text
//! [tag:2][payload:30]
//!   Sym  → a Symbol   — every textual arg (0b00)
//!   Fact → a FactId   — a nested relational node (0b10)
//! ```
//!
//! **`0b01` is no value.** It was the tag of an int, an entry in an integer
//! pool, until M1f S1f.5.4 retired the kind: since S1f.5.5's swap a run of
//! digits is a name (Q-M1e2.24, the user's ruling of 2026-11-06: *"digits are
//! not numbers, digits are plain names"*), so nothing built one. The two other
//! tags keep their bits, so a `.einb` stores a value as it did, and the one
//! reader of stored bits, [`Value::from_bits`], refuses `0b01` rather than
//! reading it as something else. `0b11` is [`Value::UNBOUND`]'s.
//!
//! **Ordering trap.** A `Value`'s numeric order is (tag, then assignment
//! order) — identity order, not semantic order — so this type deliberately has
//! no `Ord`. Identity containers ask for [`Value::cmp_identity`] by name;
//! observable sorts go through `Terms::cmp_semantic`, which reads the
//! interner's rank table. Two comparators, two names, and no way to reach for
//! the wrong one by writing `.sort()`.

use crate::facts::FactId;
use crate::intern::{CAPACITY, Symbol};
use std::cmp::Ordering;

const PAYLOAD_BITS: u32 = 30;
const PAYLOAD_MASK: u32 = (1 << PAYLOAD_BITS) - 1;

/// Which of the two shapes a [`Value`] holds. The numbers are the tag bits,
/// and `1`, an int's until M1f S1f.5.4, is no shape (the module docs).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Tag {
    Sym = 0,
    Fact = 2,
}

/// The tag bits an int had, which no value has had since M1f S1f.5.4.
const RETIRED_INT: u32 = 1;

/// A fact argument. 4 bytes, `Copy`, no `Ord` — see the module docs.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(u32);

impl Value {
    pub fn sym(sym: Symbol) -> Value {
        Value::pack(Tag::Sym, sym.0)
    }

    /// A nested relational node — `(not (color-loc Red House-1))` is a fact
    /// whose one argument is this. It is what turns the negation index into a
    /// bitset over `FactId` and `contradicts()` into a bit test.
    pub fn fact(id: FactId) -> Value {
        Value::pack(Tag::Fact, id.0)
    }

    fn pack(tag: Tag, payload: u32) -> Value {
        debug_assert!(payload < CAPACITY, "an id was assigned past CAPACITY");
        Value((tag as u32) << PAYLOAD_BITS | (payload & PAYLOAD_MASK))
    }

    /// The shape this value names.
    ///
    /// **Not defined on `UNBOUND`**, which is why the sentinel is checked
    /// rather than named: [`Tag`] has two members and the sentinel is a bit
    /// pattern neither has, so any answer here would be a lie. Ask
    /// [`Value::is_unbound`] first — every caller that can see a register
    /// value does, and the assertion is what keeps that true (M1e `CO-M3`).
    pub fn tag(self) -> Tag {
        debug_assert!(
            !self.is_unbound(),
            "Value::tag on the UNBOUND sentinel — ask is_unbound() first"
        );
        debug_assert!(
            self.0 >> PAYLOAD_BITS != RETIRED_INT,
            "a value with an int's tag bits, which no constructor builds and from_bits refuses"
        );
        match self.0 >> PAYLOAD_BITS {
            0 => Tag::Sym,
            _ => Tag::Fact,
        }
    }

    pub fn payload(self) -> u32 {
        self.0 & PAYLOAD_MASK
    }

    /// The whole 32-bit word — for hashing, for `.einb`, and for nothing else.
    pub fn bits(self) -> u32 {
        self.0
    }

    /// The inverse of [`Value::bits`], for the one caller that has a stored
    /// word and no other way to say what it meant: `.einb`'s remap
    /// ([design/10 §3](../../../../docs/history/m1a_rust/design/10_binary_format.md)),
    /// which reads the tag to decide *which* table the payload moves through
    /// and re-packs the result. Not a general constructor — a value assembled
    /// from a number nobody interned names an id that may not exist, which is
    /// why the container checks every payload against its table before it
    /// trusts one.
    ///
    /// **`None` for an int's tag bits** (M1f S1f.5.4): a `.einb` written
    /// before the int kind was retired may hold one, and there is no value it
    /// could be read as, so the container refuses the file instead.
    pub fn from_bits(bits: u32) -> Option<Value> {
        (bits >> PAYLOAD_BITS != RETIRED_INT).then_some(Value(bits))
    }

    pub fn as_sym(self) -> Option<Symbol> {
        (!self.is_unbound() && self.tag() == Tag::Sym).then(|| Symbol(self.payload()))
    }

    /// The `FactId` this value names — **`None` for the sentinel**.
    ///
    /// M1e `CO-M3`. `UNBOUND` is `u32::MAX`, whose two tag bits are `0b11` and
    /// whose payload is `CAPACITY - 1` — an id [`crate::facts`] will
    /// legitimately assign, since it refuses only at `>= CAPACITY`. So
    /// `tag()`'s `_ => Tag::Fact` fallthrough made this hand back a **phantom
    /// fact**, and the type's own test proved only that `pack` cannot
    /// *produce* the sentinel, which is a different claim: the safety rested
    /// on every caller asking `is_unbound()` first.
    ///
    /// They all did — 29 `as_fact` sites at the audit, and the three that can
    /// see a register value (`match_.rs`'s two guard readers and
    /// `saturator.rs`'s boundary scan) test it explicitly. That is the reason
    /// this is a hardening and not a bug fix, and the reason it is worth
    /// doing anyway: the discipline was a convention, and the next caller is
    /// the one it fails.
    ///
    /// `as_sym` is guarded for the same reason, though the sentinel could
    /// never have passed its tag test: it would have asked [`Value::tag`],
    /// which is not defined on it.
    pub fn as_fact(self) -> Option<FactId> {
        (!self.is_unbound() && self.tag() == Tag::Fact).then(|| FactId(self.payload()))
    }

    /// The register file's "nothing bound here yet" sentinel — S1a.3.2.
    ///
    /// It is not a third shape: the two tag bits have four states and
    /// [`Tag`] uses two, so `0b11` is a bit pattern `Value::pack` can
    /// never produce. `regs[r] == Value::UNBOUND` is therefore one integer
    /// compare, and a real value cannot forge it.
    ///
    /// It is also what a predicate guard's lenient policy treats as a null:
    /// an unbound `Var` in a guard resolves to it, and two of them compare
    /// equal to each other and to nothing else.
    pub const UNBOUND: Value = Value(u32::MAX);

    pub fn is_unbound(self) -> bool {
        self.0 == u32::MAX
    }

    /// A total order over *identity*, not over meaning.
    ///
    /// Correct wherever any total order would do — a `state_key`'s sorted
    /// vector, a no-good clause's canonical form
    /// ([design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md) §6)
    /// — and wrong everywhere a name is what the reader sees.
    pub fn cmp_identity(self, other: Value) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}#{}", self.tag(), self.payload())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_value_is_four_bytes() {
        assert_eq!(size_of::<Value>(), 4);
        assert_eq!(size_of::<Option<Value>>(), 8);
    }

    #[test]
    fn the_two_shapes_round_trip_through_the_tag() {
        let s = Value::sym(Symbol(7));
        let f = Value::fact(FactId(7));
        assert_eq!(s.as_sym(), Some(Symbol(7)));
        assert_eq!(s.as_fact(), None);
        assert_eq!(f.as_fact(), Some(FactId(7)));
        assert_eq!(f.as_sym(), None);
        // Same payload, two different values — the tag is identity too.
        assert_ne!(s, f);
    }

    /// **An int's tag bits are no value** — M1f S1f.5.4. `from_bits` is the
    /// one way in for stored bits, and it reads every word `bits` gives back
    /// and refuses a word tagged `0b01`, an int's until the kind was retired,
    /// whatever its payload: a `.einb` written before may hold one.
    #[test]
    fn an_ints_tag_bits_are_no_value() {
        for v in [
            Value::sym(Symbol(0)),
            Value::sym(Symbol(CAPACITY - 1)),
            Value::fact(FactId(0)),
            Value::fact(FactId(CAPACITY - 1)),
            Value::UNBOUND,
        ] {
            assert_eq!(Value::from_bits(v.bits()), Some(v));
        }
        for payload in [0, 1, 7, CAPACITY - 1] {
            assert_eq!(
                Value::from_bits(RETIRED_INT << PAYLOAD_BITS | payload),
                None
            );
        }
    }

    #[test]
    fn the_unbound_sentinel_is_not_a_value() {
        // `pack` only ever emits tags 0 and 2, so the all-ones word is
        // unreachable from every (tag, payload) pair.
        for tag in [Tag::Sym, Tag::Fact] {
            assert_ne!(Value::pack(tag, CAPACITY - 1), Value::UNBOUND);
        }
        assert!(Value::UNBOUND.is_unbound());
        assert!(!Value::sym(Symbol(0)).is_unbound());
    }

    /// **The sentinel names no fact** — M1e `CO-M3`.
    ///
    /// The test above proves `pack` cannot *produce* `UNBOUND`, which is a
    /// weaker claim than it reads as: the sentinel's payload is
    /// `CAPACITY - 1`, an id the fact store will assign, and `tag()`'s
    /// `_ => Tag::Fact` fallthrough used to hand it back as
    /// `Some(FactId(CAPACITY - 1))`. So a consumer that called `as_fact()`
    /// without asking `is_unbound()` first got a **phantom fact** — a real id
    /// naming somebody else's row. This assertion failed before S1e.3.1.
    #[test]
    fn the_sentinel_names_no_fact() {
        assert_eq!(Value::UNBOUND.as_fact(), None);
        assert_eq!(Value::UNBOUND.as_sym(), None);
        // …and the real value with that payload still does.
        let top = FactId(CAPACITY - 1);
        assert_eq!(Value::fact(top).as_fact(), Some(top));
        assert_ne!(Value::fact(top), Value::UNBOUND);
    }

    #[test]
    fn the_top_payload_survives_packing() {
        let top = Symbol(CAPACITY - 1);
        let v = Value::sym(top);
        assert_eq!(v.as_sym(), Some(top));
        assert_eq!(v.tag(), Tag::Sym);
        assert_eq!(Value::fact(FactId(CAPACITY - 1)).tag(), Tag::Fact);
    }
}
