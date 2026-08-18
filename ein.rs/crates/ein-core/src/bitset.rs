//! A dense set of `u32` ids.
//!
//! Three of the KB's indexes are membership tests over `FactId` — belief,
//! the negated-inner set, and (from
//! [P1a.4](../../../../plans/m1a_rust/p1a.4_search_layer/README.md)) the alive
//! set. ein.py spells them `set[tuple[str, tuple]]`; here an id is a dense
//! `u32`, so a bitset answers in one shift and one mask
//! ([design/03](../../../../plans/m1a_rust/design/03_data_model.md) §6).
//!
//! Hand-rolled rather than `bitvec`, per
//! [design/12](../../../../plans/m1a_rust/design/12_toolchain_and_layout.md) §2:
//! the whole surface is four operations.

/// Equality is over *contents*: a set that has grown and one that has not
/// compare equal when they hold the same ids, which is what the
/// flatten-versus-rebuild assertion needs.
#[derive(Clone, Default, Debug)]
pub struct BitSet {
    words: Vec<u64>,
}

impl BitSet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add `id`; `true` if it was not already present.
    pub fn insert(&mut self, id: u32) -> bool {
        let (word, bit) = (id as usize / 64, id as usize % 64);
        if word >= self.words.len() {
            self.words.resize(word + 1, 0);
        }
        let was = self.words[word] & (1 << bit) != 0;
        self.words[word] |= 1 << bit;
        !was
    }

    pub fn contains(&self, id: u32) -> bool {
        let (word, bit) = (id as usize / 64, id as usize % 64);
        self.words.get(word).is_some_and(|w| w & (1 << bit) != 0)
    }

    pub fn len(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.words.iter().all(|&w| w == 0)
    }

    /// The ids, ascending.
    pub fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.words.iter().enumerate().flat_map(|(i, &w)| {
            (0..64).filter_map(move |b| (w & (1 << b) != 0).then_some((i * 64 + b) as u32))
        })
    }
}

impl PartialEq for BitSet {
    fn eq(&self, other: &Self) -> bool {
        let n = self.words.len().max(other.words.len());
        (0..n).all(|i| {
            self.words.get(i).copied().unwrap_or(0) == other.words.get(i).copied().unwrap_or(0)
        })
    }
}

impl Eq for BitSet {}

impl FromIterator<u32> for BitSet {
    fn from_iter<I: IntoIterator<Item = u32>>(ids: I) -> Self {
        let mut set = BitSet::new();
        for id in ids {
            set.insert(id);
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn membership_and_iteration_agree() {
        let mut s = BitSet::new();
        assert!(s.is_empty());
        for id in [0u32, 1, 63, 64, 65, 4095] {
            assert!(s.insert(id));
            assert!(!s.insert(id));
        }
        assert_eq!(s.iter().collect::<Vec<_>>(), vec![0, 1, 63, 64, 65, 4095]);
        assert_eq!(s.len(), 6);
        assert!(s.contains(4095));
        assert!(!s.contains(4094));
        assert!(!s.contains(1 << 20));
    }

    #[test]
    fn equality_ignores_how_far_the_backing_grew() {
        let mut grown = BitSet::new();
        grown.insert(4095);
        grown.insert(1);
        let mut small: BitSet = [1u32].into_iter().collect();
        assert_ne!(grown, small);
        small.insert(4095);
        assert_eq!(grown, small);
    }
}
