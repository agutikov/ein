//! The 64-byte header and the section table — [design/10
//! §2](../../../../docs/history/m1a_rust/design/10_binary_format.md).
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

/// Bumped on any layout change. A reader **refuses** a newer major.
pub const FORMAT_MAJOR: u16 = 1;

/// Bumped when a section kind is added. A reader of the same major **ignores**
/// section kinds it does not know, which is what makes a minor bump additive.
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
    Ints = 3,
    Facts = 4,
    Present = 5,
    Prov = 6,
    Program = 7,
    /// Never written — see [`crate`] § What is not a section.
    Indexes = 8,
    Nogoods = 9,
    Solutions = 10,
}

impl Kind {
    /// `None` for a kind this major version does not know: the reader skips
    /// it rather than failing, which is the forward-compatibility rule.
    pub fn from_u32(n: u32) -> Option<Kind> {
        Some(match n {
            1 => Kind::Meta,
            2 => Kind::Symbols,
            3 => Kind::Ints,
            4 => Kind::Facts,
            5 => Kind::Present,
            6 => Kind::Prov,
            7 => Kind::Program,
            8 => Kind::Indexes,
            9 => Kind::Nogoods,
            10 => Kind::Solutions,
            _ => return None,
        })
    }
}

/// One row of the section table.
#[derive(Clone, Copy, Debug)]
pub struct Entry {
    pub kind: u32,
    /// Reserved for per-section compression (design/10 §2). No writer sets it
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
        // The digest covers everything *after* the header (design/10 §2), so
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
/// The CLI dispatches on this rather than on the extension (T1a.8.1.7): a
/// `.ein` and a `.einb` are told apart by their first bytes, so a renamed file
/// is still read as what it is.
pub fn is_einb(bytes: &[u8]) -> bool {
    bytes.len() >= MAGIC.len() && bytes[..MAGIC.len()] == MAGIC
}
