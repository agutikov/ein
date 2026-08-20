//! The `(trace …)` IR round-trip — the half of S1a.5.2's acceptance that was
//! never about a second engine.
//!
//! `trace_parity` compared three rendering modes per corpus entry against
//! ein.py, and rode a property along inside the comparison: every rendered
//! trace ends with a `--- round-trip` block that reports whether
//! `trace_to_ir(parse_trace_steps(trace_to_ir(steps)))` reproduced its input.
//! That property needs no oracle — it is a claim about one engine's two
//! directions agreeing — and
//! [S1a.10.2](../../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.2_port_the_suite.md)
//! keeps it while the byte comparison goes to `corpus_shapes.md5` (107 files ×
//! `trace[trace]` / `trace[answer]` / `trace[no-proof]`) and to
//! `golden_trace.rs`'s five whole documents.
//!
//! It is worth keeping *separately* from the digests because it is the only
//! thing here that is not a self-golden. A digest says the rendering is what
//! it was; this says the rendering can be **read back**, which is the property
//! [idea 08](../../../../plans/ideas/08-human-style-deductive-trace.md) needs
//! and the one a renderer breaks by emitting something prettier.

use ein_core::Terms;
use ein_corpus::{corpus_files, repo_root};
use ein_ir::{Ast, parse};
use ein_render::shape::trace_shape;

/// **Every trace the corpus produces round-trips through the IR.**
///
/// The `--- round-trip` line is `trace_shape`'s own report, so what this sweep
/// adds is coverage and a floor: the property has to hold on *every* file that
/// renders a trace, and at least fifty have to render one. Fifty is
/// `trace_parity`'s own floor, carried over — a sweep that quietly stopped
/// finding traces would otherwise pass by asserting nothing about nothing.
#[test]
fn every_rendered_trace_round_trips_through_the_ir() {
    let (mut round_trips, mut rendered, mut bad) = (0usize, 0usize, Vec::new());
    for path in &corpus_files() {
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Ok(forms) = parse(&mut ast, &text, path.to_str()) else {
            continue;
        };
        let Ok(shape) = trace_shape(&mut ast, &mut terms, &forms, path.parent(), "trace") else {
            continue;
        };
        rendered += 1;
        let name = path.strip_prefix(repo_root()).unwrap_or(path).display();
        if shape.contains("--- round-trip ok") {
            round_trips += 1;
        } else if shape.contains("--- round-trip") {
            bad.push(format!("{name}: the trace IR does not round-trip"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    assert!(
        rendered >= 50 && round_trips >= 50,
        "only {round_trips} of {rendered} rendered traces round-tripped"
    );
}
