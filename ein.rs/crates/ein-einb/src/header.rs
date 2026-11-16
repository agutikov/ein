//! The 64-byte header and the section table — [design/binary-format.md](../../../../docs/ein/ein.rs/design/binary-format.md).
//!
//! Everything here is little-endian and fixed-width, which is checked rather
//! than assumed: a big-endian host reading a file written on a little-endian
//! one would otherwise read plausible garbage, and the format has no consumer
//! that wants byte-swapping.

use crate::wire::{Reader, Writer};
use crate::{EinbError, Result};

/// `EINB\0` plus three pad bytes — eight, so the two `u16`s that follow are
/// aligned and the header stays castable.
pub const MAGIC: [u8; 8] = *b"EINB\0\0\0\0";

/// Bumped on any layout change. A reader **refuses** any other major.
///
/// `2`: `META` holds no time, which would make two saves of one program
/// differ (Q-M1f.110), and there is no integer table (Q-M1f.10). A file of
/// another major is refused with the version line, and is a cache to save
/// again.
pub const FORMAT_MAJOR: u16 = 2;

/// Bumped when a section kind is added. A reader of the same major **ignores**
/// section kinds it does not know, which is what makes a minor bump additive.
///
/// `0`: every section kind below, [`Kind::Closure`] and [`Kind::Universal`]
/// among them, is 2.0's.
pub const FORMAT_MINOR: u16 = 0;

pub const HEADER_LEN: usize = 64;
pub const ENTRY_LEN: usize = 32;

/// Sections are padded to this, so a cast of a section's bytes to `&[u32]` or
/// `&[RawRow]` never fails on alignment ([`crate::cast`]).
pub const ALIGN: usize = 8;

/// What a section holds. The numbers are on disk: reorder them and every file
/// written before the reorder reads as something else, which is what
/// [`FORMAT_MAJOR`] is for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Kind {
    Meta = 1,
    Symbols = 2,
    // 3 is no kind: it numbered an integer table, which Ein has no use for
    // (Q-M1f.10), and is never reused, so a section of kind 3 is an unknown
    // one and skipped.
    Facts = 4,
    Present = 5,
    Prov = 6,
    Program = 7,
    /// Never written — see [`crate`] § What is not a section.
    Indexes = 8,
    // 9 is no kind: it numbered learned clauses, which no route keeps, and
    // no 2.x writer writes it. Never reused: a section of kind 9 is an
    // unknown one and skipped.
    Solutions = 10,
    /// Each derived fact's closure and smallest-closure arrival
    /// ([`ein_core::Kb::closures`]); written when the KB has
    /// any, which a saturated one does.
    Closure = 11,
    /// The facts each derivation record's universals read
    /// ([`ein_core::Prov::universal`]); written when any
    /// record has some.
    Universal = 12,
}

impl Kind {
    /// `None` for a kind this major version does not know: the reader skips
    /// it rather than failing, which is the forward-compatibility rule.
    pub fn from_u32(n: u32) -> Option<Kind> {
        Some(match n {
            1 => Kind::Meta,
            2 => Kind::Symbols,
            4 => Kind::Facts,
            5 => Kind::Present,
            6 => Kind::Prov,
            7 => Kind::Program,
            8 => Kind::Indexes,
            10 => Kind::Solutions,
            11 => Kind::Closure,
            12 => Kind::Universal,
            _ => return None,
        })
    }
}

/// One row of the section table.
#[derive(Clone, Copy, Debug)]
pub struct Entry {
    pub kind: u32,
    /// Reserved for per-section compression. No writer sets it
    /// and a reader refuses a non-zero value rather than guessing, so the
    /// field cannot become a silent divergence while it is unused.
    pub flags: u32,
    pub off: u64,
    pub len: u64,
    /// The uncompressed length. Equal to `len` while `flags` is zero.
    pub raw_len: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct Header {
    pub major: u16,
    pub minor: u16,
    pub flags: u32,
    pub n_sections: u32,
    pub digest: [u8; 32],
}

impl Header {
    pub fn write(&self, w: &mut Writer) {
        w.bytes(&MAGIC);
        w.u16(self.major);
        w.u16(self.minor);
        w.u32(self.flags);
        w.u32(self.n_sections);
        w.u32(0); // reserved
        w.bytes(&self.digest);
        w.bytes(&[0u8; 8]); // reserved
        debug_assert_eq!(w.len(), HEADER_LEN);
    }

    pub fn read(r: &mut Reader<'_>) -> Result<Header> {
        let magic = r.array::<8>()?;
        if magic != MAGIC {
            return Err(EinbError::NotEinb);
        }
        let (major, minor, flags, n_sections) = (r.u16()?, r.u16()?, r.u32()?, r.u32()?);
        // The digest covers everything *after* the header
        // (binary-format.md § 2.1), so
        // the header is the one place a flipped bit has no hash over it. Every
        // field but these two either changes what the file claims to be
        // (`magic`, `major`, `flags`, `n_sections`) or is legitimately allowed
        // to differ (`minor` — a later minor is a file this reader is supposed
        // to accept). The reserved words are neither, so requiring them zero is
        // what closes the gap rather than leaving eight quiet bytes.
        let reserved = r.u32()?;
        let header = Header {
            major,
            minor,
            flags,
            n_sections,
            digest: r.array::<32>()?,
        };
        if reserved != 0 || r.array::<8>()? != [0u8; 8] {
            return Err(EinbError::Malformed("reserved header bytes are not zero"));
        }
        if header.major != FORMAT_MAJOR {
            return Err(EinbError::Version {
                major: header.major,
                minor: header.minor,
            });
        }
        Ok(header)
    }
}

impl Entry {
    pub fn write(&self, w: &mut Writer) {
        w.u32(self.kind);
        w.u32(self.flags);
        w.u64(self.off);
        w.u64(self.len);
        w.u64(self.raw_len);
    }

    pub fn read(r: &mut Reader<'_>) -> Result<Entry> {
        Ok(Entry {
            kind: r.u32()?,
            flags: r.u32()?,
            off: r.u64()?,
            len: r.u64()?,
            raw_len: r.u64()?,
        })
    }
}

/// The magic sniff, for a caller deciding what it was handed.
///
/// The CLI dispatches on this rather than on the extension: a
/// `.ein` and a `.einb` are told apart by their first bytes, so a renamed file
/// is still read as what it is.
pub fn is_einb(bytes: &[u8]) -> bool {
    bytes.len() >= MAGIC.len() && bytes[..MAGIC.len()] == MAGIC
}
