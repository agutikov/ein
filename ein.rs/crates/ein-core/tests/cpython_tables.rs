//! `pyrepr` and `pyfmt` against **frozen CPython answers** — S1a.1.2's
//! acceptance, after the interpreter left.
//!
//! Both modules re-implement behaviour nobody wrote down as a spec: `repr()`'s
//! quote choice and escape table, and `format()`'s sign/pad/align
//! interaction. A differential test was the only kind that could be wrong in
//! the direction that mattered, and
//! [S1a.10.2](../../../../docs/history/m1a_rust/README.md#s1a102--port-the-python-test-suite)
//! could not keep one. What it kept instead is the **corpus** — every
//! generator below is unchanged, and every one is deterministic — with
//! CPython's answers checked in beside it:
//!
//! | table | rows | what it covers |
//! |---|---:|---|
//! | `repr_values.txt` | 35 | every value shape reachable from a fact argument |
//! | `repr_escapes.txt` | 1 500+ | every code point where this implementation's printability classification turns over, plus all of U+0000–U+02FF and U+2000–U+20FF |
//! | `float_format.txt` | 2 584 | 136 seeded doubles × 19 format specs |
//!
//! This is the weakest of the substitutions the stage made and it is worth
//! being plain about why: a frozen table cannot follow CPython. If a future
//! Python changes `repr`'s escape set — as 3.0 did, and as a Unicode revision
//! could again — these tables keep the old answer and nothing notices. They
//! are a **regression** gate for ein.rs, not a parity gate against Python, and
//! that is [accepted loss L2](../../../../docs/history/m1a_rust/oracle_ledger.md#6-accepted-loss)
//! taking effect. What they still do is exactly what the sweeps did on every
//! ordinary day: notice when this engine's answer moves.
//!
//! ```text
//! EIN_BLESS=1 cargo test -p ein-core --test cpython_tables
//! ```

use ein_core::pyfmt::{Spec, format_float};
use ein_core::pyrepr::{PyValue, repr};
use ein_corpus::{golden, golden_path};

fn s(x: &str) -> PyValue {
    PyValue::Str(x.to_string())
}

fn i(x: &str) -> PyValue {
    PyValue::Int(x.to_string())
}

/// Every value shape reachable from a fact argument.
fn value_corpus() -> Vec<PyValue> {
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
    // Canonical decimal text only — `PyValue::Int` carries what
    // `canonical_int` produced, so `-0` is `0` before it ever gets here.
    let ints = [
        "0",
        "7",
        "-7",
        "123456789012345678901234567890",
        "-123456789012345678901234567890",
    ];

    let mut out: Vec<PyValue> = Vec::new();
    out.extend(strings.iter().map(|x| s(x)));
    out.extend(ints.iter().map(|x| i(x)));
    // Tuples of every arity that renders differently: `()`, `(a,)`, `(a, b)`.
    out.push(PyValue::Tuple(vec![]));
    out.push(PyValue::Tuple(vec![s("a")]));
    out.push(PyValue::Tuple(vec![i("-7")]));
    out.push(PyValue::Tuple(vec![s("a"), i("2"), s("it's")]));
    out.push(PyValue::Tuple(vec![
        PyValue::Tuple(vec![s("a")]),
        PyValue::Tuple(vec![]),
    ]));
    // Facts, including a nested one — the relational-node duality.
    let inner = PyValue::Fact {
        relation_name: "co-located".into(),
        args: vec![s("Norwegian"), s("House-2")],
    };
    out.push(inner.clone());
    out.push(PyValue::Fact {
        relation_name: "hypothesis".into(),
        args: vec![inner.clone()],
    });
    out.push(PyValue::Fact {
        relation_name: "arity-0".into(),
        args: vec![],
    });
    out.push(PyValue::Fact {
        relation_name: "quote'd".into(),
        args: vec![i("1"), inner, s("x")],
    });
    out.push(PyValue::Tuple(vec![PyValue::Fact {
        relation_name: "n".into(),
        args: vec![s("a")],
    }]));
    out
}

/// **`repr` of every reachable value shape.**
///
/// Quote choice (a `'` in the string flips CPython to `"`), the escape table,
/// the one-tuple's trailing comma, a nested `Fact`'s dataclass repr, an
/// arity-0 relation, and integers wider than an `i64` — 35 shapes, each with
/// the answer CPython gave.
#[test]
fn repr_matches_cpython_for_every_reachable_value_shape() {
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
/// The table is a Unicode-category question and CPython's answer depends on
/// *its* Unicode version, so the sweep is over every code point where this
/// implementation's classification turns over — the boundaries, where a
/// disagreement would live — plus all of U+0000–U+02FF and the
/// general-punctuation block, where it would be most visible.
///
/// One line per point, so the golden is a diff of the points that changed
/// rather than of a 1 500-character string.
#[test]
fn repr_escapes_the_same_code_points_cpython_escapes() {
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
            repr(&PyValue::Str(ch.to_string()))
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
    repr(&PyValue::Str(ch.to_string())).contains('\\')
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
fn float_formatting_matches_cpython() {
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
