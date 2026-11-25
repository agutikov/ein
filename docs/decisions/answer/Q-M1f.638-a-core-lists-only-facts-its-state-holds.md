# Q-M1f.638 — A core lists only facts its state holds, a written `(open F)` in root's alone

A dead node's core is the frontier of its contradiction's derivation over the facts the node's state holds: a
premise the state does not hold is neither walked nor listed. A written `(open F)` is held at root alone, and a fork
drops it ([Q-M1f.566](../hypotheses/Q-M1f.566-what-a-state-owes-is-open-r-a-fact-and-open-f.md)), so where a rule
read one at root, the core of a dead node above root does not list it, though the derivation the core explains read
it; at the empty commitment, whose state is root's
([Q-M1f.655](../engine/Q-M1f.655-open-facts-are-written-at-root-and-where-a-state.md)), the core lists it.

```lisp
(open (s a)) (open (s b))
(q b)
(rule r () :match (open (s ?x)) :assert (t ?x) :why "{?x} is t")
(rule k () :match (and (s ?x) (t ?x) (q ?x)) :assert (false) :why "{?x} clashes")
```

The dead `{(s b)}` has the core `(q b) (s b)`, and no `(open (s b))`.

**Why.** A core explains a state by what the state holds
([Q-M1f.6](Q-M1f.6-a-core-is-an-explanation.md)), and a fact the state does not hold is no part of it, as nothing
else a node does not hold is; listed, it would name in a node's explanation a fact no state above root has.

**Held by.** `ein_core::walks::walk_premises`, which resolves each premise against the state it walks and follows
none the state does not hold; `ein-infer/tests/open_facts.rs`:
`a_fork_drops_every_open_fact_a_written_one_among_them` and `the_empty_commitments_state_is_roots`. No test holds
the core itself: a rule that reads an `open` fact flags its program
([Q-M1f.569](../hypotheses/Q-M1f.569-a-rule-may-read-an-open-fact-and-its-program.md)), and nothing is claimed of
its answer ([Q-M1f.564](Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)).
