//! Little-endian primitives, and a cursor that cannot walk off the end.
//!
//! Every read is checked and returns [`EinbError::Truncated`] rather than
//! panicking, because the acceptance for this stage is that *arbitrary bytes*
//! are rejected and never mis-parsed: a reader that indexes a slice is one
//! fuzz case away from a panic, and a panic in a library is a denial of
//! service in whatever embeds it.

use crate::{EinbError, Result};

#[derive(Default)]
pub struct Writer {
    buf: Vec<u8>,
}

impl Writer {
    pub fn new() -> Writer {
        Writer::default()
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.buf
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }

    pub fn bytes(&mut self, b: &[u8]) {
        self.buf.extend_from_slice(b);
    }

    pub fn u8(&mut self, v: u8) {
        self.buf.push(v);
    }

    pub fn u16(&mut self, v: u16) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn u32(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn u64(&mut self, v: u64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn i64(&mut self, v: i64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    /// A float by its bits — `to_le_bytes` on the IEEE-754 pattern, so a NaN
    /// payload survives the round trip and nothing depends on a formatter.
    pub fn f64(&mut self, v: f64) {
        self.buf.extend_from_slice(&v.to_bits().to_le_bytes());
    }

    /// A length-prefixed string. UTF-8 in, UTF-8 out — the reader validates.
    pub fn str(&mut self, s: &str) {
        self.u32(s.len() as u32);
        self.bytes(s.as_bytes());
    }

    pub fn opt_u32(&mut self, v: Option<u32>) {
        match v {
            Some(v) => {
                self.u8(1);
                self.u32(v);
            }
            None => {
                self.u8(0);
                self.u32(0);
            }
        }
    }

    /// Pad to a multiple of `n` with zeros — what keeps a section's arrays
    /// castable ([`crate::cast`]).
    pub fn align(&mut self, n: usize) {
        while !self.buf.len().is_multiple_of(n) {
            self.buf.push(0);
        }
    }
}

pub struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Reader<'a> {
        Reader { bytes, at: 0 }
    }

    pub fn at(&self) -> usize {
        self.at
    }

    pub fn remaining(&self) -> usize {
        self.bytes.len() - self.at
    }

    pub fn is_done(&self) -> bool {
        self.at == self.bytes.len()
    }

    pub fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        let end = self.at.checked_add(n).ok_or(EinbError::Truncated)?;
        let out = self.bytes.get(self.at..end).ok_or(EinbError::Truncated)?;
        self.at = end;
        Ok(out)
    }

    pub fn array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut out = [0u8; N];
        out.copy_from_slice(self.take(N)?);
        Ok(out)
    }

    pub fn u8(&mut self) -> Result<u8> {
        Ok(self.array::<1>()?[0])
    }

    pub fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.array()?))
    }

    pub fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.array()?))
    }

    pub fn u64(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.array()?))
    }

    pub fn i64(&mut self) -> Result<i64> {
        Ok(i64::from_le_bytes(self.array()?))
    }

    pub fn f64(&mut self) -> Result<f64> {
        Ok(f64::from_bits(u64::from_le_bytes(self.array()?)))
    }

    pub fn str(&mut self) -> Result<&'a str> {
        let n = self.u32()? as usize;
        std::str::from_utf8(self.take(n)?).map_err(|_| EinbError::Malformed("string is not UTF-8"))
    }

    pub fn opt_u32(&mut self) -> Result<Option<u32>> {
        let present = self.u8()?;
        let v = self.u32()?;
        match present {
            0 => Ok(None),
            1 => Ok(Some(v)),
            _ => Err(EinbError::Malformed("optional flag is not 0 or 1")),
        }
    }

    /// A count, checked against what is left before anything is allocated.
    ///
    /// `Vec::with_capacity(n)` on an attacker-chosen `n` is the other way a
    /// reader of arbitrary bytes falls over, and it is not a panic — it is a
    /// 4 GB allocation. `each` is the smallest number of bytes one element can
    /// occupy, so a count that could not possibly be backed by the remaining
    /// input is refused up front.
    pub fn count(&mut self, each: usize) -> Result<usize> {
        let n = self.u32()? as usize;
        if each > 0 && n > self.remaining() / each {
            return Err(EinbError::Truncated);
        }
        Ok(n)
    }
}
