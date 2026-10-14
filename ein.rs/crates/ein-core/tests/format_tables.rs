//! `repr` and `numfmt`, as tables — the specification of both formats.
//!
//! Both modules implement a format whose rules are a quote choice, an escape
//! table and a sign/pad/align interaction, and a rule stated only in prose is
//! one nobody notices changing. So every generator below is deterministic,
//! and its output is checked in beside it, one row per case:
//!
//! | table | rows | what it covers |
//! |---|---:|---|
//! | `repr_values.txt` | 35 | every value shape reachable from a fact argument |
//! | `repr_escapes.txt` | 1 500+ | every code point where the printability classification turns over, plus all of U+0000–U+02FF and U+2000–U+20FF |
//! | `float_format.txt` | 2 584 | 136 seeded doubles × 19 format specs |
//!
//! A diff here is a change to what the engine prints, and re-blessing one is
//! a decision to change the format:
//!
//! ```text
//! EIN_BLESS=1 cargo test -p ein-core --test format_tables
//! ```

use ein_core::numfmt::{Spec, format_float};
use ein_core::repr::{ReprValue, repr};
use ein_corpus::{golden, golden_path};

fn s(x: &str) -> ReprValue {
    ReprValue::Str(x.to_string())
}

fn i(x: &str) -> ReprValue {
    ReprValue::Int(x.to_string())
}

/// Every value shape reachable from a fact argument.
fn value_corpus() -> Vec<ReprValue> {
    let strings = [
        "",
        "a",
        "co-located",
        "it's",
        "say \"hi\"",
        "both ' and \"",
        "back\\slash",
        "tab\there",
        "nl\nhere",
        "cr\rhere",
        "\u{0}\u{1}\u{1f}\u{7f}",
        "\u{a0}nbsp",
        "\u{200b}zwsp",
        "\u{2028}\u{2029}",
        "Åsa",
        "中文",
        "😀",
        "\u{e000}private",
        "\u{378}unassigned",
        "mixed 'quote\" and \\ and \n",
        "House-1",
        "__closed__",
    ];
    // Canonical decimal text only — `ReprValue::Int` carries what
    // `canonical_int` produced, so `-0` is `0` before it ever gets here.
    let ints = [
        "0",
        "7",
        "-7",
        "123456789012345678901234567890",
        "-123456789012345678901234567890",
    ];

    let mut out: Vec<ReprValue> = Vec::new();
    out.extend(strings.iter().map(|x| s(x)));
    out.extend(ints.iter().map(|x| i(x)));
    // Tuples of every arity that renders differently: `()`, `(a,)`, `(a, b)`.
    out.push(ReprValue::Tuple(vec![]));
    out.push(ReprValue::Tuple(vec![s("a")]));
    out.push(ReprValue::Tuple(vec![i("-7")]));
    out.push(ReprValue::Tuple(vec![s("a"), i("2"), s("it's")]));
    out.push(ReprValue::Tuple(vec![
        ReprValue::Tuple(vec![s("a")]),
        ReprValue::Tuple(vec![]),
    ]));
    // Facts, including a nested one — the relational-node duality.
    let inner = ReprValue::Fact {
        relation_name: "co-located".into(),
        args: vec![s("Norwegian"), s("House-2")],
    };
    out.push(inner.clone());
    out.push(ReprValue::Fact {
        relation_name: "hypothesis".into(),
        args: vec![inner.clone()],
    });
    out.push(ReprValue::Fact {
        relation_name: "arity-0".into(),
        args: vec![],
    });
    out.push(ReprValue::Fact {
        relation_name: "quote'd".into(),
        args: vec![i("1"), inner, s("x")],
    });
    out.push(ReprValue::Tuple(vec![ReprValue::Fact {
        relation_name: "n".into(),
        args: vec![s("a")],
    }]));
    out
}

/// **`repr` of every reachable value shape.**
///
/// Quote choice (a `'` and no `"` in the string flips the quote to `"`), the
/// escape table, the one-tuple's trailing comma, a nested `Fact`, an arity-0
/// relation, and integers wider than an `i64` — 35 shapes, one row each.
#[test]
fn repr_of_every_reachable_value_shape() {
    let corpus = value_corpus();
    assert!(
        corpus.len() >= 30,
        "the value corpus shrank to {}",
        corpus.len()
    );
    let mut out = String::new();
    for v in &corpus {
        out.push_str(&format!("{}\n", repr(v)));
    }
    if let Some(msg) = golden(&golden_path("ein-core", "repr_values.txt"), &out) {
        panic!("{msg}");
    }
}

/// **`repr`'s escape table, code point by code point.**
///
/// The table is a Unicode-category question and its answer depends on the
/// Unicode version `printable.rs` was generated from, so the sweep is over
/// every code point where the classification turns over — the boundaries,
/// where a regeneration would move something — plus all of U+0000–U+02FF and
/// the general-punctuation block, where it would be most visible.
///
/// One line per point, so the golden is a diff of the points that changed
/// rather than of a 1 500-character string.
#[test]
fn repr_escape_table_code_point_by_code_point() {
    let mut points: Vec<u32> = (0..0x300).collect();
    points.extend(0x2000..0x2100);
    let mut prev = escapes(0x80);
    for cp in 0x80..0x11_0000u32 {
        let now = escapes(cp);
        if now != prev {
            points.extend([cp.saturating_sub(1), cp, cp + 1]);
            prev = now;
        }
    }
    points.retain(|&c| char::from_u32(c).is_some());
    points.sort_unstable();
    points.dedup();
    assert!(
        points.len() > 1500,
        "only {} code points swept",
        points.len()
    );

    let mut out = String::new();
    for &c in &points {
        let Some(ch) = char::from_u32(c) else {
            continue;
        };
        out.push_str(&format!(
            "U+{c:04X} {}\n",
            repr(&ReprValue::Str(ch.to_string()))
        ));
    }
    if let Some(msg) = golden(&golden_path("ein-core", "repr_escapes.txt"), &out) {
        panic!("{msg}");
    }
}

/// Does this code point survive into a `repr` verbatim? `printable` is a
/// private module, so ask the renderer, which is the only caller anyway.
fn escapes(cp: u32) -> bool {
    let Some(ch) = char::from_u32(cp) else {
        return false;
    };
    repr(&ReprValue::Str(ch.to_string())).contains('\\')
}

/// **`format_float` over 136 doubles × 19 specs.**
///
/// The seeded values are the ones a human would not think of — −0.0, ±inf,
/// NaN and its sign, 5e-324, `f64::MAX`, and the `0x3F747AE147AE147B` near-tie
/// — plus two hundred bit patterns from a fixed xorshift, so the corpus is not
/// only the cases somebody wrote down. Each row carries the value's **bits**,
/// because two doubles that print the same are not the same double and a diff
/// has to be able to tell which one moved.
#[test]
fn float_formatting_over_seeded_doubles_and_specs() {
    let mut values: Vec<f64> = vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        0.5,
        1.5,
        2.5,
        -2.5,
        0.05,
        1.0 / 3.0,
        100.0 / 3.0,
        2.0 / 3.0,
        0.1,
        1e-300,
        f64::MIN_POSITIVE,
        5e-324,
        1e15,
        1e16,
        1e17,
        1e300,
        f64::MAX,
        -f64::MAX,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        -f64::NAN,
        1234567.891,
        -1234567.891,
        99.995,
        // The classic "looks like a tie, is not" rounding case.
        f64::from_bits(0x3F747AE147AE147Bu64),
    ];
    // A deterministic spread of bit patterns, so the corpus is not only the
    // values a human thought of.
    let mut x = 0x2545_F491_4F6C_DD1Du64;
    for _ in 0..200 {
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        let v = f64::from_bits(x.wrapping_mul(0x2545_F491_4F6C_DD1D));
        if v.is_finite() && v.abs() < 1e30 {
            values.push(v);
        }
    }
    let specs = [
        "f", ".0f", ".1f", ".2f", ".6f", ".17f", "9.2f", ">5.1f", "<9.2f", "^9.2f", "+.1f", " .1f",
        "08.2f", "=9.2f", "_>9.2f", "020.10f", "-.3f", "0.2f", "*^12.3f",
    ];

    assert!(
        values.len() >= 130,
        "the float corpus shrank to {}",
        values.len()
    );
    let mut out = String::new();
    for &v in &values {
        for spec in specs {
            let parsed = Spec::parse(spec).unwrap_or_else(|| panic!("spec {spec:?}"));
            out.push_str(&format!(
                "{:016x} {spec:>8} {:?}\n",
                v.to_bits(),
                format_float(v, &parsed)
            ));
        }
    }
    if let Some(msg) = golden(&golden_path("ein-core", "float_format.txt"), &out) {
        panic!("{msg}");
    }
}
