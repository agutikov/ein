//! A count of commitments, exact at any size, and the bound a run reports it
//! to — M1f S1f.6.25, E-22 of P1f.6's `findings.md`, W92.
//!
//! One state can be reached by more commitments than any machine integer
//! holds: a state of 130 candidates that any one of them reaches is reached by
//! 2¹³⁰ − 1. Until that stage the state route counted in `u128` and summed
//! into `u64`, and where either overflowed the whole description was an
//! error, *a state's commitments pass 2^128: the state route cannot count
//! them*, exit 1 with no read-out, though the run had classified every
//! commitment. It was no error and its sentence was not true either: a sum
//! past 2⁶⁴ raised it too.
//!
//! [`Count`] is exact, so the counting itself never fails: `2^|S|` less the
//! states below is a subtraction of exact numbers whatever their size. What a
//! run *reports* is a `u64` no greater than [`CAP`], 2⁶³ − 1, which every
//! surface can carry, JSON and TOML among them; **a reported count equal to
//! [`CAP`] reads *this many or more***.

/// The greatest count a run reports, 2⁶³ − 1: a reported count equal to it
/// reads *this many or more*. It is `i64::MAX`, so a JSON writer and a TOML
/// reader that hold a signed 64-bit integer carry every reported count.
pub const CAP: u64 = i64::MAX as u64;

/// `a + b` as a reported count: no greater than [`CAP`].
pub fn capped_add(a: u64, b: u64) -> u64 {
    a.saturating_add(b).min(CAP)
}

/// A natural number of any size: little-endian 64-bit limbs, no trailing zero
/// limb, zero the empty vector.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Count(Vec<u64>);

impl Count {
    pub fn zero() -> Count {
        Count(Vec::new())
    }

    pub fn of(v: u128) -> Count {
        let mut c = Count(vec![v as u64, (v >> 64) as u64]);
        c.trim();
        c
    }

    /// `2^k`.
    pub fn pow2(k: u32) -> Count {
        let mut limbs = vec![0u64; (k / 64) as usize + 1];
        limbs[(k / 64) as usize] = 1u64 << (k % 64);
        Count(limbs)
    }

    fn trim(&mut self) {
        while self.0.last() == Some(&0) {
            self.0.pop();
        }
    }

    pub fn add(&mut self, other: &Count) {
        if self.0.len() < other.0.len() {
            self.0.resize(other.0.len(), 0);
        }
        let mut carry = false;
        for (i, limb) in self.0.iter_mut().enumerate() {
            let o = other.0.get(i).copied().unwrap_or(0);
            let (s, c1) = limb.overflowing_add(o);
            let (s, c2) = s.overflowing_add(carry as u64);
            *limb = s;
            carry = c1 || c2;
        }
        if carry {
            self.0.push(1);
        }
    }

    /// `self − other`, or `None` where `other` is the greater.
    pub fn checked_sub(&self, other: &Count) -> Option<Count> {
        if other.0.len() > self.0.len() {
            return None;
        }
        let mut out = self.0.clone();
        let mut borrow = false;
        for (i, limb) in out.iter_mut().enumerate() {
            let o = other.0.get(i).copied().unwrap_or(0);
            let (d, b1) = limb.overflowing_sub(o);
            let (d, b2) = d.overflowing_sub(borrow as u64);
            *limb = d;
            borrow = b1 || b2;
        }
        if borrow {
            return None;
        }
        let mut c = Count(out);
        c.trim();
        Some(c)
    }

    /// The count as a run reports it: itself, or [`CAP`] where it is that or
    /// more.
    pub fn reported(&self) -> u64 {
        match self.0.as_slice() {
            [] => 0,
            [v] => (*v).min(CAP),
            _ => CAP,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_count_is_exact_past_every_machine_integer() {
        // 2¹³⁰ − 1, the commitments that reach a state of 130 candidates any
        // one of which reaches it: every subset but the empty one.
        let all = Count::pow2(130);
        let reach = all.checked_sub(&Count::of(1)).expect("2^130 > 1");
        assert_eq!(reach.0, vec![u64::MAX, u64::MAX, 3]);
        assert_eq!(reach.reported(), CAP);
        // …and with ∅'s one, the whole cube again.
        let mut sum = reach.clone();
        sum.add(&Count::of(1));
        assert_eq!(sum, all);
        // Subtraction of the greater is refused, and of itself is zero.
        assert_eq!(Count::of(3).checked_sub(&Count::of(4)), None);
        assert_eq!(all.checked_sub(&all), Some(Count::zero()));
        assert_eq!(Count::of(u128::MAX).0, vec![u64::MAX, u64::MAX]);
        let mut carry = Count::of(u128::MAX);
        carry.add(&Count::of(1));
        assert_eq!(carry, Count::pow2(128));
    }

    #[test]
    fn a_reported_count_stops_at_the_cap() {
        assert_eq!(Count::zero().reported(), 0);
        assert_eq!(Count::of(7).reported(), 7);
        assert_eq!(Count::of(CAP as u128 - 1).reported(), CAP - 1);
        // The cap itself reads *this many or more*, which is true of it.
        assert_eq!(Count::of(CAP as u128).reported(), CAP);
        assert_eq!(Count::pow2(63).reported(), CAP);
        assert_eq!(Count::pow2(64).reported(), CAP);
        assert_eq!(Count::pow2(792).reported(), CAP);
        assert_eq!(capped_add(CAP - 1, 1), CAP);
        assert_eq!(capped_add(CAP, CAP), CAP);
        assert_eq!(capped_add(u64::MAX, 1), CAP);
        assert_eq!(capped_add(2, 3), 5);
    }
}
