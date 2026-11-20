# Q-M1d.3 — An obligation needs no closed domain

Ein closes no domain for an obligation. Stating one needs no closure: the guard's own scan is its domain, and
`(open ?R)` names the relation rather than restating it. Paying one needs none: a witness is a positive fact, and a
positive check is monotone. Refuting stays with the `forall` scans over stored negations, `std.algebra`'s `total`
beside `total-owed`, which are safe in an open world. The facts that would pay an instance are the guard's scan with
its witness step skipped, read at a state's fixpoint and written as hypotheses, `(open F)`, root's in H:
nothing restates the domain. A choice among those facts is exhaustive only while the set cannot grow beneath it, so
where it can, the obligation writes what root's guard gives as any obligation does, and the program is one the
static check flags ([Q-M1e2.10](Q-M1e2.10-an-obligation-whose-domain-can-grow-writes.md)).

**Why.** A domain the program never closed cannot carry a requirement that quantifies over it; an obligation that
named a domain of its own would bring a type system into the kernel, and the stdlib takes its hierarchy relation as an
activator's parameter instead.

**Held by.** `ein_infer::oblgen`, through `Matcher::scan_without`; `ein-infer/tests/obligation_rung.rs`:
`the_candidates_are_what_the_guard_scans`, `the_theory_finds_what_the_hypothesis_rule_finds`; the `total` and `total-owed`
pairs of `stdlib/algebra.ein`.
