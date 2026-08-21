//! The remap — [design/10
//! §3](../../../../plans/m1a_rust/design/10_binary_format.md#3-ids-across-the-boundary),
//! task T1a.8.1.3.
//!
//! `Symbol`, `IntId`, `FactId` and `ProvId` are process-local integers, so a
//! file must be loadable into a process whose tables already hold other
//! content. Each id space is stored **in id order**, so re-interning entry *i*
//! yields the live id for the file's *i*, and every other section is remapped
//! through the resulting `Vec` in one linear pass.
//!
//! **The fast path is the normal one.** A fresh process opening one file
//! re-interns into empty tables and gets its own ids back, so every table is
//! the identity and the pass is skipped — that is the `mmap`-and-go case, and
//! [`Maps::identity`] is what a reader tests instead of assuming it.

use ein_core::facts::FactId;
use ein_core::intern::Symbol;
use ein_core::prov::ProvId;
use ein_core::value::{IntId, Tag, Value};

use crate::{EinbError, Result};

#[derive(Default, Debug)]
pub struct Maps {
    pub sym: Vec<Symbol>,
    pub int: Vec<IntId>,
    pub fact: Vec<FactId>,
    /// Provenance is **never** the identity: the reader rebuilds the program
    /// before it reads `PROV`, and rebuilding pushes the loader's own records
    /// first, so the file's records land at an offset. A linear remap, which
    /// is what the design costs the whole boundary at.
    pub prov: Vec<ProvId>,
    /// `Loc.file` — an index into the reconstructed [`ein_ir::Ast`]'s file
    /// table, which the reader primes in the file's own order so this is the
    /// identity in practice and correct when it is not.
    pub file: Vec<u32>,
}

impl Maps {
    /// Are the three interned spaces the identity? Asked once, so the hot
    /// loops can skip the indirection entirely.
    pub fn identity(&self) -> bool {
        is_identity(&self.sym, |s| s.0)
            && is_identity(&self.int, |i| i.0)
            && is_identity(&self.fact, |f| f.0)
    }

    pub fn symbol(&self, i: u32) -> Result<Symbol> {
        self.sym.get(i as usize).copied().ok_or(EinbError::BadId {
            what: "symbol",
            id: i,
        })
    }

    pub fn int(&self, i: u32) -> Result<IntId> {
        self.int.get(i as usize).copied().ok_or(EinbError::BadId {
            what: "integer",
            id: i,
        })
    }

    pub fn fact(&self, i: u32) -> Result<FactId> {
        self.fact.get(i as usize).copied().ok_or(EinbError::BadId {
            what: "fact",
            id: i,
        })
    }

    pub fn prov(&self, i: u32) -> Result<ProvId> {
        self.prov.get(i as usize).copied().ok_or(EinbError::BadId {
            what: "provenance record",
            id: i,
        })
    }

    pub fn loc_file(&self, i: u32) -> Result<u32> {
        self.file.get(i as usize).copied().ok_or(EinbError::BadId {
            what: "source file",
            id: i,
        })
    }

    /// A stored [`Value`]'s 32 bits, translated.
    ///
    /// The tag survives untouched and only the payload moves, which is what
    /// makes the pass linear. [`Value::UNBOUND`] is special-cased rather than
    /// decoded: its bit pattern is the fourth tag state, so reading it as a
    /// tag would call it a fact and remap a payload it does not have.
    pub fn value(&self, bits: u32) -> Result<Value> {
        if bits == Value::UNBOUND.bits() {
            return Ok(Value::UNBOUND);
        }
        let raw = Value::from_bits(bits);
        Ok(match raw.tag() {
            Tag::Sym => Value::sym(self.symbol(raw.payload())?),
            Tag::Int => Value::int(self.int(raw.payload())?),
            Tag::Fact => Value::fact(self.fact(raw.payload())?),
        })
    }
}

fn is_identity<T: Copy>(table: &[T], id: impl Fn(T) -> u32) -> bool {
    table.iter().enumerate().all(|(i, &v)| id(v) == i as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fresh_table_is_the_identity_and_a_permuted_one_is_not() {
        let mut m = Maps {
            sym: (0..4).map(Symbol).collect(),
            int: (0..2).map(IntId).collect(),
            fact: (0..3).map(FactId).collect(),
            ..Maps::default()
        };
        assert!(m.identity());
        m.sym.swap(0, 1);
        assert!(!m.identity());
    }

    #[test]
    fn a_value_keeps_its_tag_and_moves_its_payload() {
        let m = Maps {
            sym: vec![Symbol(7), Symbol(8)],
            int: vec![IntId(3)],
            fact: vec![FactId(5)],
            ..Maps::default()
        };
        assert_eq!(
            m.value(Value::sym(Symbol(1)).bits()).unwrap(),
            Value::sym(Symbol(8))
        );
        assert_eq!(
            m.value(Value::int(IntId(0)).bits()).unwrap(),
            Value::int(IntId(3))
        );
        assert_eq!(
            m.value(Value::fact(FactId(0)).bits()).unwrap(),
            Value::fact(FactId(5))
        );
        assert_eq!(m.value(Value::UNBOUND.bits()).unwrap(), Value::UNBOUND);
        assert!(m.value(Value::sym(Symbol(9)).bits()).is_err());
    }
}
