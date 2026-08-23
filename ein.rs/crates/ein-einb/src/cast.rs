//! The zero-copy casts — **the only `unsafe` in the repository**
//! (T1a.8.1.4, [design/12
//! §2](../../../../docs/history/m1a_rust/design/12_toolchain_and_layout.md#2-dependency-policy)).
//!
//! A `.einb` section is an array of fixed-width little-endian records, and
//! reading one element at a time through [`crate::wire::Reader`] costs a bounds
//! check and a `from_le_bytes` per field. The point of the layout is that on a
//! little-endian host those bytes **already are** the array, so the reader
//! borrows it instead ([design/10
//! §2](../../../../docs/history/m1a_rust/design/10_binary_format.md#2-container)).
//!
//! Three things make that sound, and all three are checked here rather than
//! assumed:
//!
//! 1. **Alignment.** Sections are padded to [`crate::header::ALIGN`] and the
//!    whole file is read into a `u64`-backed buffer ([`Aligned`]), so a cast
//!    is aligned by construction — but [`slice_of`] verifies it anyway and
//!    returns `None` instead, because "by construction" stops being true the
//!    first time someone passes a subslice.
//! 2. **Length.** The byte length must be an exact multiple of the element
//!    size; a trailing partial record is a refusal, not a truncation.
//! 3. **Bit patterns.** [`Pod`] is `unsafe` to implement and is implemented
//!    only for `u32`, `u64` and `#[repr(C)]` structs of them, none of which
//!    has an invalid pattern. A cast to a type with a niche — `bool`, an
//!    `enum`, a `NonZeroU32` — would be undefined behaviour for a file that
//!    contains the wrong byte, which is exactly the file this reader must
//!    survive, so no such type is `Pod`.
//!
//! Little-endianness is the format's, not the host's: [`slice_of`] is
//! compiled out on a big-endian target and the reader falls back to the
//! decoding path, so the bytes mean the same thing everywhere.

#![allow(unsafe_code)]

use std::mem::{align_of, size_of};

/// A type whose every bit pattern is a valid value, and whose layout is the
/// file's.
///
/// # Safety
///
/// Implementors must be `#[repr(C)]` or `#[repr(transparent)]`, contain no
/// padding, and have no invalid bit patterns — no `bool`, no `char`, no
/// fieldless `enum`, no `NonZero*`, no reference, no pointer.
pub unsafe trait Pod: Copy {}

// SAFETY: an integer has no invalid bit pattern and no padding.
unsafe impl Pod for u32 {}
// SAFETY: as above.
unsafe impl Pod for u64 {}

/// One fact row as the file holds it — `(rel, args_at, arity)`.
///
/// Deliberately **not** [`ein_core::facts`]'s `Row`. That one carries two
/// inline arguments because the matcher's dependency chain wanted them
/// (T1a.6.2.6), and `INLINE_ARGS` is a measurement that may move; a file
/// written before it moved would then be a file of a different shape. The
/// on-disk row keeps every argument in the arena, so the format is not
/// hostage to an in-memory tuning decision.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct RawRow {
    pub rel: u32,
    pub args_at: u32,
    pub arity: u32,
}

// SAFETY: `#[repr(C)]`, three `u32`s, no padding (size 12 = 3 × 4), and every
// bit pattern of each field is a valid `u32`.
unsafe impl Pod for RawRow {}

/// Borrow `bytes` as `&[T]`, or `None` when it is not exactly an array of them.
pub fn slice_of<T: Pod>(bytes: &[u8]) -> Option<&[T]> {
    if cfg!(target_endian = "big") {
        return None;
    }
    let size = size_of::<T>();
    if size == 0 || !bytes.len().is_multiple_of(size) {
        return None;
    }
    if bytes.as_ptr().align_offset(align_of::<T>()) != 0 {
        return None;
    }
    // SAFETY: the pointer is aligned for `T` (checked above) and the region
    // holds exactly `bytes.len() / size` whole `T`s within one allocation
    // (`bytes` is a live slice). `T: Pod` promises no invalid bit pattern, so
    // every one of those elements is a valid value. The lifetime of the result
    // is tied to `bytes` by the signature.
    Some(unsafe { std::slice::from_raw_parts(bytes.as_ptr().cast::<T>(), bytes.len() / size) })
}

/// The bytes behind a `&[T]` — the write direction, and always sound.
pub fn bytes_of<T: Pod>(items: &[T]) -> &[u8] {
    // SAFETY: `T: Pod` is `#[repr(C)]` with no padding, so its representation
    // is `size_of::<T>()` initialised bytes; `u8` has alignment 1, which every
    // pointer satisfies; the region is one live allocation.
    unsafe { std::slice::from_raw_parts(items.as_ptr().cast::<u8>(), size_of_val(items)) }
}

/// A byte buffer that is 8-byte aligned, so [`slice_of`] never has to refuse
/// one of *our* sections.
///
/// `Vec<u8>` gives alignment 1 and happens to be 8- or 16-aligned in practice;
/// "happens to" is not a property to build a fast path on, and a file read
/// into an under-aligned buffer would silently take the slow path on some
/// allocators and not others. Backing the buffer with `Vec<u64>` makes the
/// guarantee the allocator's.
pub struct Aligned {
    words: Vec<u64>,
    len: usize,
}

impl Aligned {
    /// Read a whole file into an 8-byte-aligned buffer.
    pub fn read(path: &std::path::Path) -> std::io::Result<Aligned> {
        use std::io::Read;
        let mut file = std::fs::File::open(path)?;
        let hint = file.metadata().map(|m| m.len() as usize).unwrap_or(0);
        let mut words: Vec<u64> = vec![0; hint / 8 + 1];
        let mut len = 0usize;
        loop {
            if len == words.len() * 8 {
                words.resize(words.len() * 2 + 1, 0);
            }
            let n = {
                // SAFETY: `words` is a live allocation of `words.len() * 8`
                // bytes; `u8` is `Pod` with alignment 1, so a mutable byte view
                // of it is valid for the borrow, and `len` is within it.
                let buf: &mut [u8] = unsafe {
                    std::slice::from_raw_parts_mut(words.as_mut_ptr().cast::<u8>(), words.len() * 8)
                };
                file.read(&mut buf[len..])?
            };
            if n == 0 {
                break;
            }
            len += n;
        }
        Ok(Aligned { words, len })
    }

    /// Copy an existing buffer into an aligned one — the in-memory path, and
    /// what a test uses to open bytes it just wrote.
    pub fn from_bytes(bytes: &[u8]) -> Aligned {
        let mut words: Vec<u64> = vec![0; bytes.len() / 8 + 1];
        // SAFETY: as in `read` — a byte view of a live `[u64]` allocation.
        let buf: &mut [u8] = unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast::<u8>(), words.len() * 8)
        };
        buf[..bytes.len()].copy_from_slice(bytes);
        Aligned {
            words,
            len: bytes.len(),
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &bytes_of(&self.words)[..self.len]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_row_is_twelve_bytes_with_no_padding() {
        assert_eq!(size_of::<RawRow>(), 12);
        assert_eq!(align_of::<RawRow>(), 4);
    }

    #[test]
    fn a_cast_refuses_a_partial_record_and_a_misaligned_start() {
        let a = Aligned::from_bytes(&[0u8; 24]);
        assert_eq!(
            slice_of::<RawRow>(a.as_slice()).map(<[RawRow]>::len),
            Some(2)
        );
        // One byte short of three rows.
        assert!(slice_of::<RawRow>(&a.as_slice()[..23]).is_none());
        // Offset by one: the length divides, the address does not.
        let mut wide = Aligned::from_bytes(&[0u8; 32]);
        wide.len = 25;
        assert!(slice_of::<RawRow>(&wide.as_slice()[1..25]).is_none());
    }

    #[test]
    fn the_round_trip_through_bytes_is_the_identity() {
        let rows = [
            RawRow {
                rel: 1,
                args_at: 2,
                arity: 3,
            },
            RawRow {
                rel: u32::MAX,
                args_at: 0,
                arity: 7,
            },
        ];
        let a = Aligned::from_bytes(bytes_of(&rows));
        assert_eq!(slice_of::<RawRow>(a.as_slice()).expect("aligned"), &rows);
    }

    #[test]
    fn an_aligned_buffer_is_aligned() {
        for n in [0usize, 1, 7, 8, 9, 4096] {
            let a = Aligned::from_bytes(&vec![7u8; n]);
            assert_eq!(a.as_slice().len(), n);
            assert_eq!(a.as_slice().as_ptr().align_offset(8), 0);
            assert!(a.as_slice().iter().all(|&b| b == 7));
        }
    }
}
