//! S1a.2.1's acceptance — one ordering and one rendering the data model has
//! to get exactly right, each checked against a statement of it written out
//! separately.
//!
//! The interner's rank table replaces every sort by name with a `u32` sort.
//! Unit tests can only check the cases somebody thought of; this checks a
//! generated corpus against a **specification of the behaviour**:
//!
//! - names sort in code-point order, and so does Rust's `Ord` on `String` —
//!   UTF-8 preserves code-point order — so the reference is
//!   `Vec<String>::sort`, computed here rather than fetched.
//! - `display` of a nested `Fact` has no such short specification, so the six
//!   shapes the test builds are checked in as strings.
//!
//! The generated corpus is ~900 names, because its job is to reach the
//! neighbourhoods a hand-written case would miss. A third, ~400 integer
//! literals each canonicalised by the int pool against nine lines of Rust that
//! shared no code with it, went with the pool when M1f S1f.5.4 retired the int
//! kind: since S1f.5.5's swap a run of digits is a name, kept as written, so
//! `7`, `07` and `007` are three names
//! ([`a_run_of_digits_is_a_name_as_written`]).

use ein_core::{Interner, Symbol, Terms};

/// A deterministic xorshift, so the corpus is the same on every run and on
/// every machine — a test that fails only sometimes is not a gate.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// Distinct strings spanning the code-point neighbourhoods where a byte
/// ordering and a code-point ordering could plausibly disagree.
fn unicode_corpus() -> Vec<String> {
    // (start, len) of each sampled range: ASCII, Latin-1 supplement, Greek +
    // Cyrillic, CJK, and non-BMP emoji — plus the C0 controls, which sort
    // below everything and are legal inside a `String` literal.
    const RANGES: [(u32, u32); 6] = [
        (0x00, 0x20),
        (0x20, 0x5f),
        (0xa0, 0x60),
        (0x370, 0x190),
        (0x4e00, 0x50),
        (0x1f600, 0x40),
    ];
    let mut rng = Rng(0x5eed_1a2b_3c4d_5e6f);
    let mut out: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    // Every single character from every range, so the pairwise orderings are
    // all exercised, then random strings for the multi-character cases
    // (prefixes, lengths, and mixed scripts).
    for (start, len) in RANGES {
        for cp in start..start + len {
            if let Some(c) = char::from_u32(cp) {
                out.insert(c.to_string());
            }
        }
    }
    while out.len() < 900 {
        let n = 1 + rng.below(8);
        let mut s = String::new();
        for _ in 0..n {
            let (start, len) = RANGES[rng.below(RANGES.len())];
            let cp = start + rng.below(len as usize) as u32;
            if let Some(c) = char::from_u32(cp) {
                s.push(c);
            }
        }
        out.insert(s);
    }
    // Names an actual puzzle uses, so the corpus is not purely synthetic.
    for name in [
        "House-1",
        "House-10",
        "House-2",
        "co-located",
        "colocated",
        "Co-located",
        "is-a",
        "__closed__",
        "?x",
        "1..5",
        "1..*",
    ] {
        out.insert(name.to_string());
    }
    out.into_iter().collect()
}

#[test]
fn the_rank_table_orders_names_by_code_point() {
    let corpus = unicode_corpus();
    assert!(corpus.len() >= 900, "corpus is {} strings", corpus.len());

    // Intern in a scrambled order, so a rank table that had quietly become
    // assignment order would be caught.
    let mut interner = Interner::new();
    let mut scrambled: Vec<usize> = (0..corpus.len()).collect();
    let mut rng = Rng(0x0123_4567_89ab_cdef);
    for i in (1..scrambled.len()).rev() {
        scrambled.swap(i, rng.below(i + 1));
    }
    let mut syms: Vec<Option<Symbol>> = vec![None; corpus.len()];
    for &i in &scrambled {
        syms[i] = Some(interner.intern(&corpus[i]).expect("room"));
    }

    // Names sort in code-point order; Rust's `Ord` on `String` is byte order
    // over UTF-8, and UTF-8 is order-preserving on code points. So the reference is a sort, and the only thing that could make
    // it disagree is the rank table itself.
    let mut want: Vec<usize> = (0..corpus.len()).collect();
    want.sort_by(|&a, &b| corpus[a].cmp(&corpus[b]));

    let mut bad = Vec::new();
    for (position, &index) in want.iter().enumerate() {
        let got = interner.rank(syms[index].expect("interned"));
        if got as usize != position {
            bad.push(format!(
                "{:?}: code-point position {position}, rank table {got}",
                corpus[index]
            ));
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {} names rank differently:\n{}",
        bad.len(),
        corpus.len(),
        bad.join("\n")
    );
}

/// **A run of digits is a name, as written** — M1f S1f.5.4, on the user's
/// ruling of 2026-11-06 (*"digits are not numbers, digits are plain names
/// … (p 1) and (p 01) are VERY DIFFERENT facts"*).
///
/// Every literal the int pool's test canonicalised until the kind was retired
/// — widths either side of an `i64`, leading zeros, both signs, the two
/// spellings of zero — is a name here: interned as its own text, distinct
/// from every other spelling, and printed as written.
#[test]
fn a_run_of_digits_is_a_name_as_written() {
    let mut literals: Vec<String> = Vec::new();
    let mut rng = Rng(0xfeed_face_dead_beef);
    for digits in [1usize, 2, 5, 18, 19, 20, 39, 80] {
        for _ in 0..12 {
            let mut s = String::new();
            for _ in 0..digits {
                s.push(char::from(b'0' + rng.below(10) as u8));
            }
            literals.push(s.clone());
            literals.push(format!("-{s}"));
            literals.push(format!("000{s}"));
            literals.push(format!("-000{s}"));
        }
    }
    for fixed in [
        "0",
        "-0",
        "007",
        "-007",
        "-000",
        "9223372036854775807",
        "9223372036854775808",
    ] {
        literals.push(fixed.to_string());
    }
    literals.sort();
    literals.dedup();
    assert_eq!(
        literals.len(),
        361,
        "the generator's 391 literals, as distinct texts"
    );

    let mut t = Terms::new();
    let values: Vec<_> = literals
        .iter()
        .map(|l| t.value_text(l).expect("room"))
        .collect();
    let mut bad = Vec::new();
    for (literal, v) in literals.iter().zip(&values) {
        if t.display(*v) != *literal {
            bad.push(format!("{literal:?} prints as {:?}", t.display(*v)));
        }
    }
    let distinct: std::collections::HashSet<_> = values.iter().collect();
    if distinct.len() != literals.len() {
        bad.push(format!(
            "{} spellings are {} names",
            literals.len(),
            distinct.len()
        ));
    }
    assert!(
        bad.is_empty(),
        "{} of {} literals differ:\n{}",
        bad.len(),
        literals.len(),
        bad.join("\n")
    );
}

/// **`Terms::display` renders each of the six value shapes that reach a
/// trace** — as the one printer prints them (M1e2 S1e2.3.8): Ein text the
/// loader reads back to the same value.
///
/// Until S1e2.3.8 `display` of a nested fact was its `repr`,
/// `Fact(relation_name=…, args=(…))`, and a name its raw text, so `it's`
/// printed bare and could not be read back.
///
/// What each shape is for: an atom (bare), an atom with an apostrophe (bare
/// since S1f.5.5's swap), a name of digits with leading zeros and a negative
/// one wider than an `i64` (each an int until M1f S1f.5.4, printed in its
/// canonical decimal; a name since, printed as written), a nested fact, and a
/// fact nested inside a fact — the last being the only place the recursion is
/// visible.
#[test]
fn display_renders_every_value_shape() {
    let mut t = Terms::new();
    let co_located = t.intern_text("co-located").expect("room");
    let hypothesis = t.intern_text("hypothesis").expect("room");
    let norwegian = t.value_text("Norwegian").expect("room");
    let quote = t.value_text("it's").expect("room");
    let seven = t.value_text("007").expect("room");
    let big = t
        .value_text("-00123456789012345678901234567890")
        .expect("room");
    let inner = t.value_fact(co_located, &[norwegian, seven]).expect("room");
    let outer = t
        .value_fact(hypothesis, &[inner, quote, big])
        .expect("room");

    let mut bad = Vec::new();
    for (v, want) in [
        (norwegian, "Norwegian"),
        // `'` is a name character since S1f.5.5's swap (the name rule), so
        // `it's` prints bare and reads back as itself.
        (quote, "it's"),
        (seven, "007"),
        (big, "-00123456789012345678901234567890"),
        (inner, "(co-located Norwegian 007)"),
        (
            outer,
            "(hypothesis (co-located Norwegian 007) it's -00123456789012345678901234567890)",
        ),
    ] {
        let got = t.display(v);
        if got != want {
            bad.push(format!("{v:?}\n  want: {want}\n  got:  {got}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
