//! S1a.2.1's acceptance — the two places the data model has to agree with
//! CPython rather than with itself, checked without CPython.
//!
//! Both are re-implementations of behaviour that only exists as CPython
//! source: `sorted(names)` (which the interner's rank table replaces with a
//! `u32` sort) and `str(int(tok))` (which the int pool applies to every
//! literal). Unit tests can only check the cases somebody thought of; these
//! check a generated corpus against a **specification of the imitated
//! behaviour**, which is what
//! [S1a.10.2](../../../../docs/history/m1a_rust/README.md#s1a102--port-the-python-test-suite)
//! substituted for asking the interpreter:
//!
//! - `sorted()` on `str` is code-point order, and so is Rust's `Ord` on
//!   `String` — UTF-8 preserves code-point order — so the reference is
//!   `Vec<String>::sort`, computed here rather than fetched.
//! - `str(int(x))` is "strip the leading zeros, and `-0` is `0`", which is
//!   nine lines of Rust that no more shares an implementation with `IntPool`
//!   than CPython did.
//! - `repr` of a nested `Fact` has no such specification, so the six shapes
//!   the test builds are checked in as strings. Those six are ein.py's own
//!   output, captured while the differential test was green.
//!
//! The generated corpora stay the size they were — ~900 names and ~400
//! literals — because their job is to reach the neighbourhoods a hand-written
//! case would miss, and that job does not change when the reference does.

use ein_core::{IntPool, Interner, Symbol, Terms};

/// A deterministic xorshift, so the corpus is the same on every run and on
/// every machine — a differential test that fails only sometimes is not a
/// gate.
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
fn the_rank_table_orders_names_the_way_python_sorted_does() {
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

    // CPython's `sorted()` on `str` is code-point order; Rust's `Ord` on
    // `String` is byte order over UTF-8, and UTF-8 is order-preserving on code
    // points. So the reference is a sort, and the only thing that could make
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

#[test]
fn the_int_pool_canonicalises_exactly_as_int_then_str_does() {
    // Widths either side of i64, leading zeros, both signs, and the two
    // spellings of zero.
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

    let mut pool = IntPool::new();
    let mut bad = Vec::new();
    // `str(int(x)) == str(int(y))` iff the two literals are the same integer,
    // so CPython's answer also tells us which pool ids must coincide.
    let mut by_canonical: std::collections::HashMap<String, ein_core::IntId> =
        std::collections::HashMap::new();
    for literal in &literals {
        let id = pool.intern(literal).expect("room");
        let want = canonical(literal);
        if pool.text(id) != want {
            bad.push(format!(
                "{literal:?}: str(int(x)) is {want:?}, pool {:?}",
                pool.text(id)
            ));
        }
        match by_canonical.get(&want) {
            Some(&first) if first != id => {
                bad.push(format!("{literal:?} pooled as {id:?}, expected {first:?}"))
            }
            Some(_) => {}
            None => {
                by_canonical.insert(want, id);
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {} literals differ:\n{}",
        bad.len(),
        literals.len(),
        bad.join("\n")
    );
    assert_eq!(pool.len(), by_canonical.len());
}

/// `str(int(x))` for a decimal literal, written out rather than fetched.
///
/// The whole of CPython's behaviour on this input class: strip the sign,
/// strip the leading zeros, and a value of zero has no sign. Nine lines,
/// sharing no code with [`IntPool`] — which is what makes the comparison a
/// comparison.
fn canonical(literal: &str) -> String {
    let (negative, digits) = match literal.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, literal),
    };
    let stripped = digits.trim_start_matches('0');
    if stripped.is_empty() {
        return "0".to_string();
    }
    if negative {
        format!("-{stripped}")
    } else {
        stripped.to_string()
    }
}

/// **`Terms::display` renders a value the way CPython's `str` does** — for the
/// six shapes that reach a trace.
///
/// `display` is `str(v)`, which for a nested fact is its dataclass repr: the
/// string that lands in provenance bindings and in a `:why` template. There is
/// no short specification of that — it is `dataclasses`' generated `__repr__`
/// over a tuple of values, with `repr` on each — so the six answers are
/// checked in rather than re-derived. They are **ein.py's**, captured while
/// `a_facts_repr_matches_cpythons_for_every_value_shape` was green against a
/// live `utils/py_oracle.py` — the batch CPython oracle, deleted with the
/// second engine at S1a.10.4. These six answers are what is left of it here.
///
/// What each shape is for: an atom (no quotes, unlike `repr`), an atom with an
/// apostrophe (which flips CPython's quote choice to `"`), a leading-zero int,
/// a negative int wider than an `i64`, a nested fact, and a fact nested inside
/// a fact — the last being the only place the recursion is visible.
#[test]
fn a_facts_repr_matches_cpythons_for_every_value_shape() {
    let mut t = Terms::new();
    let co_located = t.intern_text("co-located").expect("room");
    let hypothesis = t.intern_text("hypothesis").expect("room");
    let norwegian = t.value_text("Norwegian").expect("room");
    let quote = t.value_text("it's").expect("room");
    let seven = t.value_int("007").expect("room");
    let big = t
        .value_int("-00123456789012345678901234567890")
        .expect("room");
    let inner = t.value_fact(co_located, &[norwegian, seven]).expect("room");
    let outer = t
        .value_fact(hypothesis, &[inner, quote, big])
        .expect("room");

    let mut bad = Vec::new();
    for (v, want) in [
        (norwegian, "Norwegian"),
        (quote, "it's"),
        (seven, "7"),
        (big, "-123456789012345678901234567890"),
        (
            inner,
            "Fact(relation_name='co-located', args=('Norwegian', 7))",
        ),
        (
            outer,
            "Fact(relation_name='hypothesis', args=(Fact(relation_name='co-located', \
             args=('Norwegian', 7)), \"it's\", -123456789012345678901234567890))",
        ),
    ] {
        let got = t.display(v);
        if got != want {
            bad.push(format!("{v:?}\n  cpython: {want}\n  ein.rs:  {got}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
