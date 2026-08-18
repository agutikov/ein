//! S1a.1.2 acceptance — `pyrepr` and `pyfmt` are checked against CPython
//! itself, over a generated corpus, not against remembered strings.
//!
//! Both modules re-implement behaviour nobody wrote down as a spec: `repr()`'s
//! quote choice and escape table, and `format()`'s sign/pad/align interaction.
//! A differential test is the only kind that can be wrong in the direction
//! that matters.

use ein_core::pyfmt::{Spec, format_float};
use ein_core::pyrepr::{PyValue, repr};
use ein_oracle::{Oracle, PY_ORACLE, skip};
use serde_json::json;

fn encode(v: &PyValue) -> serde_json::Value {
    match v {
        PyValue::Str(s) => json!({ "s": s }),
        PyValue::Int(i) => json!({ "i": i }),
        PyValue::Tuple(xs) => json!({ "t": xs.iter().map(encode).collect::<Vec<_>>() }),
        PyValue::Fact {
            relation_name,
            args,
        } => {
            json!({ "f": [relation_name, args.iter().map(encode).collect::<Vec<_>>()] })
        }
    }
}

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

#[test]
fn repr_matches_cpython_for_every_reachable_value_shape() {
    let Some(mut py) = Oracle::start(PY_ORACLE) else {
        return skip("repr_matches_cpython_for_every_reachable_value_shape");
    };
    let mut bad = Vec::new();
    for v in value_corpus() {
        let got = repr(&v);
        let want = py.ask(json!({"op": "repr", "v": encode(&v)}));
        if got != want.unwrap() {
            bad.push(format!(
                "{v:?}\n  cpython: {}\n  ein.rs:  {got}",
                want.unwrap()
            ));
        }
    }
    assert!(
        bad.is_empty(),
        "{} value(s) differ:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// `repr`'s escape table is a Unicode-category question, and the table is
/// generated from *CPython's* Unicode version — so sweep every code point
/// where this implementation's classification changes, plus all of Latin-1
/// and the general-punctuation block.
#[test]
fn repr_escapes_the_same_code_points_cpython_escapes() {
    let Some(mut py) = Oracle::start(PY_ORACLE) else {
        return skip("repr_escapes_the_same_code_points_cpython_escapes");
    };
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

    // Batched 32 at a time — 1 500 round trips would not be a test, it would
    // be a nightly job. A failing chunk is re-checked one code point at a time
    // so the message names the offender.
    let mut bad = Vec::new();
    for chunk in points.chunks(32) {
        let text: String = chunk.iter().filter_map(|&c| char::from_u32(c)).collect();
        let v = PyValue::Str(text);
        let want = py.ask(json!({"op": "repr", "v": encode(&v)}));
        if repr(&v) == want.unwrap() {
            continue;
        }
        for &c in chunk {
            let Some(ch) = char::from_u32(c) else {
                continue;
            };
            let one = PyValue::Str(ch.to_string());
            let want = py.ask(json!({"op": "repr", "v": encode(&one)}));
            if repr(&one) != want.unwrap() {
                bad.push(format!(
                    "U+{c:04X}: cpython {} · ein.rs {}",
                    want.unwrap(),
                    repr(&one)
                ));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} code point(s) differ:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// Does this code point survive into a `repr` verbatim? `printable` is a
/// private module, so ask the renderer, which is the only caller anyway.
fn escapes(cp: u32) -> bool {
    let Some(ch) = char::from_u32(cp) else {
        return false;
    };
    repr(&PyValue::Str(ch.to_string())).contains('\\')
}

#[test]
fn float_formatting_matches_cpython() {
    let Some(mut py) = Oracle::start(PY_ORACLE) else {
        return skip("float_formatting_matches_cpython");
    };
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

    let mut bad = Vec::new();
    for &v in &values {
        for spec in specs {
            let parsed = Spec::parse(spec).unwrap_or_else(|| panic!("spec {spec:?}"));
            let got = format_float(v, &parsed);
            let want = py.ask(json!({
                "op": "format",
                "v": format!("{:016x}", v.to_bits()),
                "spec": spec,
            }));
            if got != want.unwrap() {
                bad.push(format!(
                    "{v:?} (bits {:016x}) as {spec:?}\n  cpython: {:?}\n  ein.rs:  {got:?}",
                    v.to_bits(),
                    want.unwrap()
                ));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {} formattings differ:\n{}",
        bad.len(),
        values.len() * specs.len(),
        bad.join("\n")
    );
}
