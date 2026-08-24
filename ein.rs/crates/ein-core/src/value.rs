//! `Value` — a fact argument in 4 bytes, and the integer pool behind one of
//! its three shapes.
//!
//! ein.py's fact arguments are `str | int | Fact`, and 31.9 M `isinstance`
//! calls in an exhaustive `zebra2` solve are the type dispatch that unifying
//! them needs. Here the discriminant is two bits
//! ([design/03](../../../../docs/history/m1a_rust/design/03_data_model.md) §3):
//!
//! ```text
//! [tag:2][payload:30]
//!   Sym  → a Symbol   — every textual arg
//!   Int  → an IntId   — an entry in the int pool
//!   Fact → a FactId   — a nested relational node
//! ```
//!
//! **Ordering trap.** A `Value`'s numeric order is (tag, then assignment
//! order) — identity order, not semantic order — so this type deliberately has
//! no `Ord`. Identity containers ask for [`Value::cmp_identity`] by name;
//! observable sorts go through `Terms::cmp_semantic`, which reads the
//! interner's rank table and the int pool's values. Two comparators, two
//! names, and no way to reach for the wrong one by writing `.sort()`.

use crate::facts::FactId;
use crate::intern::{CAPACITY, Overflow, Symbol};
use crate::pyrepr::canonical_int;
use rustc_hash::FxHashMap;
use std::cmp::Ordering;

const PAYLOAD_BITS: u32 = 30;
const PAYLOAD_MASK: u32 = (1 << PAYLOAD_BITS) - 1;

/// Which of the three shapes a [`Value`] holds.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Tag {
    Sym = 0,
    Int = 1,
    Fact = 2,
}

/// A fact argument. 4 bytes, `Copy`, no `Ord` — see the module docs.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(u32);

impl Value {
    pub fn sym(sym: Symbol) -> Value {
        Value::pack(Tag::Sym, sym.0)
    }

    pub fn int(id: IntId) -> Value {
        Value::pack(Tag::Int, id.0)
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

    pub fn tag(self) -> Tag {
        match self.0 >> PAYLOAD_BITS {
            0 => Tag::Sym,
            1 => Tag::Int,
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
    pub fn from_bits(bits: u32) -> Value {
        Value(bits)
    }

    pub fn as_sym(self) -> Option<Symbol> {
        (self.tag() == Tag::Sym).then(|| Symbol(self.payload()))
    }

    pub fn as_int(self) -> Option<IntId> {
        (self.tag() == Tag::Int).then(|| IntId(self.payload()))
    }

    pub fn as_fact(self) -> Option<FactId> {
        (self.tag() == Tag::Fact).then(|| FactId(self.payload()))
    }

    /// The register file's "nothing bound here yet" sentinel — S1a.3.2.
    ///
    /// It is not a fourth shape: the two tag bits have four states and
    /// [`Tag`] uses three, so `0b11` is a bit pattern `Value::pack` can
    /// never produce. `regs[r] == Value::UNBOUND` is therefore one integer
    /// compare, and a real value cannot forge it.
    ///
    /// It is also what `resolve_leaf`'s lenient policy calls Python's `None`:
    /// an unbound `Var` in a predicate guard resolves to it, and two of them
    /// compare equal, exactly as `None == None`.
    pub const UNBOUND: Value = Value(u32::MAX);

    pub fn is_unbound(self) -> bool {
        self.0 == u32::MAX
    }

    /// A total order over *identity*, not over meaning.
    ///
    /// Correct wherever any total order would do — a `state_key`'s sorted
    /// vector, a no-good clause's canonical form
    /// ([design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md) §6)
    /// — and wrong everywhere a name or a number is what the reader sees.
    pub fn cmp_identity(self, other: Value) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}#{}", self.tag(), self.payload())
    }
}

/// An entry in the integer pool.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct IntId(pub u32);

/// Integer literals, canonicalised and pooled.
///
/// Python's `int` is unbounded and the grammar's `INT: /-?[0-9]+/` accepts any
/// width, so an integer cannot be inlined into a 30-bit payload and cannot be
/// parsed into an `i64` without rejecting inputs ein.py handles fine. The pool
/// stores the **canonical decimal form** — parse, then re-render, so `007` and
/// `7` and `-0`/`0` collapse exactly as `Int(value=int(tok))` does — plus an
/// `Option<i64>` fast field for the overwhelmingly common case.
///
/// Two integers are equal iff their pool ids are.
#[derive(Default, Debug)]
pub struct IntPool {
    texts: Vec<String>,
    fast: Vec<Option<i64>>,
    lookup: FxHashMap<Box<str>, IntId>,
}

impl IntPool {
    pub fn new() -> Self {
        Self::default()
    }

    /// Pool `text` — any decimal literal, canonical or not.
    pub fn intern(&mut self, text: &str) -> Result<IntId, Overflow> {
        let canonical = canonical_int(text);
        if let Some(&id) = self.lookup.get(canonical.as_str()) {
            return Ok(id);
        }
        if self.texts.len() as u32 >= CAPACITY {
            return Err(Overflow::Ints);
        }
        let id = IntId(self.texts.len() as u32);
        self.fast.push(canonical.parse::<i64>().ok());
        self.lookup.insert(canonical.as_str().into(), id);
        self.texts.push(canonical);
        Ok(id)
    }

    /// The [`IntId`] `text` already has, if any — [`IntPool::intern`] without
    /// the `&mut`, for a reader that may not grow the pool.
    pub fn get(&self, text: &str) -> Option<IntId> {
        self.lookup.get(canonical_int(text).as_str()).copied()
    }

    /// The canonical decimal text — what `str(v)` prints for this integer,
    /// which is what provenance bindings and the dumper's `_compact` render.
    pub fn text(&self, id: IntId) -> &str {
        &self.texts[id.0 as usize]
    }

    /// The value, when it fits an `i64`. `None` means "wider than 64 bits",
    /// not "not a number".
    pub fn value(&self, id: IntId) -> Option<i64> {
        self.fast[id.0 as usize]
    }

    pub fn len(&self) -> usize {
        self.texts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.texts.is_empty()
    }

    /// Numeric order over two pooled integers, at any width.
    pub fn cmp_value(&self, a: IntId, b: IntId) -> Ordering {
        match (self.value(a), self.value(b)) {
            (Some(x), Some(y)) => x.cmp(&y),
            _ => cmp_decimal(self.text(a), self.text(b)),
        }
    }
}

/// Compare two **canonical** decimal literals numerically.
///
/// Canonical means: no leading zeros, no `-0`, and a `-` only on a non-zero
/// value — so magnitude is digit count first, then a plain byte compare.
fn cmp_decimal(a: &str, b: &str) -> Ordering {
    let (a_neg, a_digits) = split_sign(a);
    let (b_neg, b_digits) = split_sign(b);
    match (a_neg, b_neg) {
        (false, true) => Ordering::Greater,
        (true, false) => Ordering::Less,
        (false, false) => a_digits
            .len()
            .cmp(&b_digits.len())
            .then_with(|| a_digits.cmp(b_digits)),
        // Both negative: the larger magnitude is the smaller number.
        (true, true) => b_digits
            .len()
            .cmp(&a_digits.len())
            .then_with(|| b_digits.cmp(a_digits)),
    }
}

fn split_sign(text: &str) -> (bool, &str) {
    match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
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
    fn the_three_shapes_round_trip_through_the_tag() {
        let s = Value::sym(Symbol(7));
        let i = Value::int(IntId(7));
        let f = Value::fact(FactId(7));
        assert_eq!(s.as_sym(), Some(Symbol(7)));
        assert_eq!(s.as_int(), None);
        assert_eq!(i.as_int(), Some(IntId(7)));
        assert_eq!(f.as_fact(), Some(FactId(7)));
        // Same payload, three different values — the tag is identity too.
        assert_ne!(s, i);
        assert_ne!(i, f);
    }

    #[test]
    fn the_unbound_sentinel_is_not_a_value() {
        // `pack` only ever emits tags 0..2, so the all-ones word is
        // unreachable from every (tag, payload) pair.
        for tag in [Tag::Sym, Tag::Int, Tag::Fact] {
            assert_ne!(Value::pack(tag, CAPACITY - 1), Value::UNBOUND);
        }
        assert!(Value::UNBOUND.is_unbound());
        assert!(!Value::sym(Symbol(0)).is_unbound());
    }

    #[test]
    fn the_top_payload_survives_packing() {
        let top = Symbol(CAPACITY - 1);
        let v = Value::sym(top);
        assert_eq!(v.as_sym(), Some(top));
        assert_eq!(v.tag(), Tag::Sym);
        assert_eq!(Value::fact(FactId(CAPACITY - 1)).tag(), Tag::Fact);
    }

    #[test]
    fn the_pool_canonicalises_exactly_as_int_then_str_does() {
        let mut p = IntPool::new();
        let seven = p.intern("7").expect("room");
        assert_eq!(p.intern("007").expect("room"), seven);
        assert_eq!(p.intern("0007").expect("room"), seven);
        let zero = p.intern("0").expect("room");
        assert_eq!(p.intern("-0").expect("room"), zero);
        assert_eq!(p.intern("-000").expect("room"), zero);
        assert_eq!(p.text(zero), "0");
        assert_ne!(p.intern("-7").expect("room"), seven);
        assert_eq!(p.len(), 3);
    }

    #[test]
    fn a_literal_wider_than_i64_pools_without_overflowing() {
        let mut p = IntPool::new();
        let big = p.intern("00123456789012345678901234567890").expect("room");
        assert_eq!(p.text(big), "123456789012345678901234567890");
        assert_eq!(p.value(big), None);
        assert_eq!(
            p.intern("123456789012345678901234567890").expect("room"),
            big
        );
        let small = p.intern("42").expect("room");
        assert_eq!(p.value(small), Some(42));
    }

    #[test]
    fn numeric_order_holds_at_any_width() {
        let mut p = IntPool::new();
        let mut ids: Vec<_> = [
            "0",
            "-1",
            "10",
            "9",
            "-10",
            "-9",
            "99999999999999999999999999",
            "-99999999999999999999999999",
        ]
        .iter()
        .map(|t| (*t, p.intern(t).expect("room")))
        .collect();
        ids.sort_by(|a, b| p.cmp_value(a.1, b.1));
        assert_eq!(
            ids.iter().map(|(t, _)| *t).collect::<Vec<_>>(),
            [
                "-99999999999999999999999999",
                "-10",
                "-9",
                "-1",
                "0",
                "9",
                "10",
                "99999999999999999999999999",
            ]
        );
    }
}
