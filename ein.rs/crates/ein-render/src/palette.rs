//! The shared relation-colour palette.
//!
//! One deterministic `relation name → colour` map, so the same relation is
//! drawn in the same colour in every view and the eye groups by relation
//! across diagrams.
//!
//! `hash_color` hashes with a stable digest, **SHA-1**, never with a salted
//! hash, so a relation's colour is the same in every process and every build.

use sha1::{Digest, Sha1};

/// d3's `schemeCategory10`, with two swaps for legibility on print.
pub const PALETTE: [&str; 10] = [
    "#1f77b4", // blue
    "#ff7f0e", // orange
    "#2ca02c", // green
    "#d62728", // red
    "#9467bd", // purple
    "#8c564b", // brown
    "#e377c2", // pink
    "#7f7f7f", // gray
    "#bcbd22", // olive
    "#17becf", // cyan
];

/// A stable colour per relation name.
///
/// `PALETTE[sha1(name) mod 10]`, the digest read as one big-endian 160-bit
/// integer. There is no 160-bit integer here and none is
/// needed: the digest is big-endian, so folding it byte by byte under the
/// modulus gives the same residue.
pub fn hash_color(name: &str) -> &'static str {
    let digest = Sha1::digest(name.as_bytes());
    let mut acc: u32 = 0;
    for byte in digest.iter() {
        acc = (acc * 256 + u32::from(*byte)) % PALETTE.len() as u32;
    }
    PALETTE[acc as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The colours the checked-in goldens carry, so the fold is pinned to the
    /// whole-digest residue rather than to itself.
    #[test]
    fn the_palette_index_is_the_whole_digest_mod_ten() {
        assert_eq!(hash_color("r"), "#ff7f0e");
        assert_eq!(hash_color("tern"), "#bcbd22");
        assert_eq!(hash_color("co-located"), "#bcbd22");
        assert_eq!(hash_color("symmetric"), "#7f7f7f");
        assert_eq!(hash_color("t"), "#d62728");
        assert_eq!(hash_color(""), "#8c564b");
    }
}
