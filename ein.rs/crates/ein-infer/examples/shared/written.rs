//! Programs written for the region tests and P1e2.1b's censuses — M1e2
//! [S1e2.1b.3](../../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.3_holes.md).
//!
//! One text each, read by `ein-infer/tests/region.rs` and by the
//! `hole_census` and `taint_census` examples, so that a census and a test
//! measure the same program. None is a corpus file: each shows a cause no
//! corpus program within the oracle's limit shows, and a corpus file would be
//! a fixture of a semantics nobody has ruled on (F21).

/// An activator compiling a **refutation** at a node: `{(p A)}` derives
/// `(watch q)`, which compiles `watch` for `q`, and `(r A)` with `(q A)`
/// absent is then a contradiction. No plan at root reads `q` under an
/// `absent`, so the static check's one site is the activator, and a check
/// that runs on root's plans cannot see the refutation (S1e2.1b.6).
pub const ACTIVATOR_REFUTES: &str =
    "(relation p T :why \"p holds of {?1}\") (relation q T :why \"q holds of {?1}\") (relation r T :why \"r holds of {?1}\") (relation watch T :why \"rule watch is switched on for relation {?1}\")
     (r A)
     (hrule g1 () :match (r ?x) :assert (p ?x) :why \"r holds of {?x}, so guess that p holds of {?x}\")
     (hrule g2 () :match (r ?x) :assert (q ?x) :why \"r holds of {?x}, so guess that q holds of {?x}\")
     (rule watch (?R) :match (and (r ?x) (absent (?R ?x))) :assert (false) :why \"r holds of {?x} and {?R} does not, which cannot be\")
     (rule act () :match (p ?x) :assert (watch q) :why \"p holds of {?x}, so rule watch is switched on for q\")";

/// An activator **derived under an `absent`**, compiling a refutation that
/// reads none (S1e2.1b.3). `act` writes `(watch r)` while `(q ?x)` is absent,
/// which compiles `watch` for `r`, and `(r A)` then refutes. So `{(p A), (r
/// A)}` is dead and `{(p A), (q A), (r A)}` is alive: a dead hole. Its
/// refutation's derivation, `(false)` ← `watch` ← `(r A)`, records no `absent`
/// premise, because a derivation does not record the activator its plan was
/// compiled from. The `absent` the death rests on is the activator's.
pub const ACTIVATOR_UNDER_ABSENT: &str =
    "(relation thing T :why \"{?1} is a thing\") (relation p T :why \"p holds of {?1}\") (relation q T :why \"q holds of {?1}\") (relation r T :why \"r holds of {?1}\") (relation watch T :why \"rule watch is switched on for relation {?1}\")
     (thing A)
     (hrule g1 () :match (thing ?x) :assert (p ?x) :why \"{?x} is a thing, so guess that p holds of {?x}\")
     (hrule g2 () :match (thing ?x) :assert (q ?x) :why \"{?x} is a thing, so guess that q holds of {?x}\")
     (hrule g3 () :match (thing ?x) :assert (r ?x) :why \"{?x} is a thing, so guess that r holds of {?x}\")
     (rule act () :match (and (p ?x) (absent (q ?x))) :assert (watch r) :why \"p holds of {?x} and q does not, so rule watch is switched on for r\")
     (rule watch (?R) :match (?R ?x) :assert (false) :why \"{?R} holds of {?x}, which cannot be\")";

/// Every program here, by the name a census prints.
pub const WRITTEN: &[(&str, &str)] = &[
    ("activator_refutes", ACTIVATOR_REFUTES),
    ("activator_under_absent", ACTIVATOR_UNDER_ABSENT),
];
