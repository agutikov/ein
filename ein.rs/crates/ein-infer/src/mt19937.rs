//! CPython's `random.Random`, enough of it for `--shuffle`.
//!
//! `solve` shuffles each layer's candidates through one `random.Random(seed)`
//! whose state **advances across layers**, so reproducing the traversal means
//! reproducing the generator bit for bit: the Mersenne Twister, its
//! `init_by_array` seeding, `getrandbits`, `_randbelow_with_getrandbits` and
//! `shuffle`'s exact Fisher–Yates direction.
//!
//! This is [Q-M1a.5](../../../../plans/m1a_rust/open_questions.md) resolved as
//! its option (a). The alternative was to declare shuffled runs T0-only, and
//! the argument against it is what `--shuffle` is *for*: it exists to probe
//! whether the verdict depends on traversal order, so a silent ordering
//! difference there is exactly the one easiest to dismiss as "well, it is
//! shuffled".
//!
//! Ported from CPython's `_randommodule.c` — the reference implementation the
//! standard library ships, unchanged since 2.4 — and checked against it by
//! table rather than by reading: `utils/ir_oracle.py`'s `shuffle` op
//! returned what `random.Random(seed).shuffle(list(range(n)))` produces, and
//! the answers it gave are the tables in this file's own tests. The op left
//! with the script at S1a.10.4; the tables are what remain of it.

const N: usize = 624;
const M: usize = 397;
const MATRIX_A: u32 = 0x9908_b0df;
const UPPER_MASK: u32 = 0x8000_0000;
const LOWER_MASK: u32 = 0x7fff_ffff;

pub struct Mt19937 {
    mt: [u32; N],
    index: usize,
}

impl Mt19937 {
    /// `random.Random(seed)` for an integer seed.
    ///
    /// CPython takes the **absolute value** and splits it into 32-bit words,
    /// little-endian, then seeds by array — so `Random(-5)` and `Random(5)`
    /// are the same generator, and a zero seed still seeds with one word.
    pub fn seeded(seed: i64) -> Mt19937 {
        let n = seed.unsigned_abs();
        let mut key: Vec<u32> = Vec::new();
        let mut rest = n;
        loop {
            key.push((rest & 0xffff_ffff) as u32);
            rest >>= 32;
            if rest == 0 {
                break;
            }
        }
        let mut r = Mt19937 {
            mt: [0; N],
            index: N,
        };
        r.init_by_array(&key);
        r
    }

    fn init_genrand(&mut self, s: u32) {
        self.mt[0] = s;
        for i in 1..N {
            let prev = self.mt[i - 1];
            self.mt[i] = 1_812_433_253u32
                .wrapping_mul(prev ^ (prev >> 30))
                .wrapping_add(i as u32);
        }
        self.index = N;
    }

    fn init_by_array(&mut self, key: &[u32]) {
        self.init_genrand(19_650_218);
        let (mut i, mut j) = (1usize, 0usize);
        for _ in 0..N.max(key.len()) {
            let prev = self.mt[i - 1];
            self.mt[i] = (self.mt[i] ^ (prev ^ (prev >> 30)).wrapping_mul(1_664_525))
                .wrapping_add(key[j])
                .wrapping_add(j as u32);
            i += 1;
            j += 1;
            if i >= N {
                self.mt[0] = self.mt[N - 1];
                i = 1;
            }
            if j >= key.len() {
                j = 0;
            }
        }
        for _ in 0..N - 1 {
            let prev = self.mt[i - 1];
            self.mt[i] = (self.mt[i] ^ (prev ^ (prev >> 30)).wrapping_mul(1_566_083_941))
                .wrapping_sub(i as u32);
            i += 1;
            if i >= N {
                self.mt[0] = self.mt[N - 1];
                i = 1;
            }
        }
        // The MSB-only value is what makes the state non-zero.
        self.mt[0] = 0x8000_0000;
    }

    fn genrand_uint32(&mut self) -> u32 {
        if self.index >= N {
            for kk in 0..N - M {
                let y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
                self.mt[kk] = self.mt[kk + M] ^ (y >> 1) ^ if y & 1 == 0 { 0 } else { MATRIX_A };
            }
            for kk in N - M..N - 1 {
                let y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
                self.mt[kk] =
                    self.mt[kk + M - N] ^ (y >> 1) ^ if y & 1 == 0 { 0 } else { MATRIX_A };
            }
            let y = (self.mt[N - 1] & UPPER_MASK) | (self.mt[0] & LOWER_MASK);
            self.mt[N - 1] = self.mt[M - 1] ^ (y >> 1) ^ if y & 1 == 0 { 0 } else { MATRIX_A };
            self.index = 0;
        }
        let mut y = self.mt[self.index];
        self.index += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^= y >> 18;
        y
    }

    /// `getrandbits(k)`, for `k` up to 64.
    ///
    /// A single draw for `k <= 32`; above that CPython draws one word per 32
    /// bits, **least significant first**, and shifts only the last one down.
    /// The word count is what matters: it decides how far the generator
    /// advances, and a shortcut that produced the same number from one draw
    /// would desynchronise every later call.
    fn getrandbits(&mut self, k: u32) -> u64 {
        if k == 0 {
            return 0;
        }
        if k <= 32 {
            return (self.genrand_uint32() >> (32 - k)) as u64;
        }
        let words = k.div_ceil(32);
        let mut out: u64 = 0;
        let mut left = k;
        for i in 0..words {
            let mut r = self.genrand_uint32();
            if left < 32 {
                r >>= 32 - left;
            }
            out |= (r as u64) << (32 * i);
            left = left.saturating_sub(32);
        }
        out
    }

    /// `_randbelow_with_getrandbits` — rejection sampling on the bit length,
    /// which is what makes the result uniform *and* what makes the number of
    /// draws data-dependent, so it has to be reproduced exactly.
    fn randbelow(&mut self, n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        let k = usize::BITS - n.leading_zeros();
        loop {
            let r = self.getrandbits(k) as usize;
            if r < n {
                return r;
            }
        }
    }

    /// `random.shuffle` — Fisher–Yates **downwards**, `for i in
    /// reversed(range(1, len(x)))`. The direction is not a detail: reversing
    /// it consumes the same numbers in a different order and produces a
    /// different permutation from the same seed.
    pub fn shuffle<T>(&mut self, x: &mut [T]) {
        for i in (1..x.len()).rev() {
            let j = self.randbelow(i + 1);
            x.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The first words `random.Random(seed).getrandbits(32)` produces,
    /// captured from CPython 3.14 — including a seed wider than one word and
    /// a negative one, because those are the two places the seeding differs
    /// from the textbook `init_genrand`.
    #[test]
    fn the_first_draws_match_cpython() {
        for (seed, want) in [
            (0i64, [3_626_764_237u32, 1_654_615_998, 3_255_389_356]),
            (42, [2_746_317_213, 478_163_327, 107_420_369]),
            (-7, [1_390_851_128, 4_071_050_724, 647_892_279]),
            (
                12_345_678_901_234,
                [107_551_873, 2_403_399_316, 624_174_316],
            ),
        ] {
            let mut r = Mt19937::seeded(seed);
            let got = [r.genrand_uint32(), r.genrand_uint32(), r.genrand_uint32()];
            assert_eq!(got, want, "seed {seed}");
        }
    }

    /// `shuffle` itself, and — the part `solve` depends on — that **one**
    /// generator carries its state across two shuffles, so the second list's
    /// permutation is not the one a fresh `Random(seed)` would give it.
    #[test]
    fn shuffle_matches_cpython_and_carries_state() {
        let mut r = Mt19937::seeded(42);
        let mut x: Vec<u32> = (0..10).collect();
        r.shuffle(&mut x);
        assert_eq!(x, [7, 3, 2, 8, 5, 6, 9, 4, 0, 1]);
        let mut y: Vec<u32> = (0..5).collect();
        r.shuffle(&mut y);
        assert_eq!(y, [1, 2, 0, 3, 4]);
    }
}
