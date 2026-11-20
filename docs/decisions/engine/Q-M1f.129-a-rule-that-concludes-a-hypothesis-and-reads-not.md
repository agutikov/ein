# Q-M1f.129 — A rule that concludes a hypothesis and reads `not` reads the state it was asked about

A rule that concludes a hypothesis, `(open F)`, is matched against the state it was asked about, its `(absent …)`
premises judged against that state
([Q-M1f.685](../hypotheses/Q-M1f.685-a-rule-that-concludes-a-hypothesis-reads-its.md)): each of its plans'
matches is collected before the first of its candidates is handed back, and no caller writes the state between two
candidates ([Q-M1f.590](../hypotheses/Q-M1f.590-a-hypothesis-rule-is-matched-as-it-is-reached.md)). So a `:match` that
reads a negation, a stored `(not …)`, an `(absent (not …))` or the stdlib's `unknown`, sees the negations that state
holds and none written mid-enumeration. A debug build and a release build answer such a program alike, and as `ein
oracle` does.

**Why.** A candidate is a hypothesis about the state the generator was asked about; an enumeration that could see a
negation written mid-enumeration would propose about a state nobody asked about.

**Held by.** `ein_infer::hypothesis_rules::HypothesisRules::candidates`; `ein-cli/tests/hrule_reads_not.rs`, every
program it reads held to `ein oracle`'s answer; the witnesses `examples/ein-bugs/wrong-answers/hrule-reads-not.ein`,
`hrule-reads-absent-not.ein` and `hrule-reads-unknown.ein`.
