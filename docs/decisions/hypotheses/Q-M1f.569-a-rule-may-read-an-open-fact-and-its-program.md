# Q-M1f.569 — A rule may read an `open` fact, and its program is then not monotone

A rule's `:match` may read an `open` fact, `(open ?R)`, `(open (p ?x))`, or the absence of one,
`(absent (open (p ?x)))`. The pass writes the state's `open` facts first, then such rules fire, and the two repeat
until neither adds anything. **A program in which a rule reads `open` is not monotone**, by a site of the static
check of its own kind, `open-read`: it loads, is warned of, and nothing is claimed of its answer
([Q-M1f.564](../answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)). A fork drops the `open` facts
and keeps what a rule concluded from them.

**Why.** What a rule concludes from an `open` fact at root stays in every state above it, while the `open` fact it
read is dropped where a commitment pays it: a state then holds a conclusion of a premise it no longer holds. Allowed,
so that programs can try it, and flagged, so that nothing is claimed.

**Held by.** `ein_infer::monotone`'s `open-read` site; `ein_infer::open`'s loop; `ein-infer/tests/open_facts.rs`:
`a_rule_that_reads_open_is_flagged`.
