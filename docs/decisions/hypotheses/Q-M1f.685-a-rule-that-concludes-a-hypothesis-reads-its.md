# Q-M1f.685 — A rule that concludes a hypothesis reads its `absent` premises

A rule that concludes `(open F)` is matched as every rule is: a match is one whose `(absent …)` premises hold, each
judged against the state the rule is read at, its fixpoint, as the saturator judges a rule's that concludes a fact. So
one `:match` under a rule that concludes a fact and under one that concludes a hypothesis concludes of the same
matches, and a rule that concludes both gives both of the same matches; at root its hypotheses are H's, and at a state
above root the `open` facts the state holds.

```lisp
(p a) (p b)
(q a)
(rule guess () :match (and (p ?x) (absent (q ?x))) :assert (open (r ?x)) :why "suppose {?x} is r")
(rule plain () :match (and (p ?x) (absent (q ?x))) :assert (s ?x)        :why "{?x} is s")
```

`plain` concludes `(s b)` and `guess` proposes `(r b)`: H is `(r b)`, and R is ∅ and {(r b)}.

Where the `absent` reads a relation a commitment can change, a state above root proposes fewer hypotheses than root
and none root does not, so no hypothesis is first held above root and the static check names no site for it
([Q-M1f.574](../answer/Q-M1f.574-a-hypothesis-first-held-above-root-makes-its.md)): R is over root's H, and what a
state above root proposes is part of what it holds, not of what is committed.

**Why.** A hypothesis rule is a rule that concludes `(open F)` and nothing apart
([Q-M1f.567](Q-M1f.567-a-hypothesis-rule-is-a-rule-that-concludes-open.md)), and its `:match` reads the state it is
asked about ([Q-M1f.129](../engine/Q-M1f.129-a-rule-that-concludes-a-hypothesis-and-reads-not.md)).

**Held by.** `ein_infer::HypothesisRules::candidates`, through which every reader of a rule's `(open F)` reads it;
`ein-infer/tests/hypothesis_rule_absent.rs`, which holds H at root and the `open` facts at every node of the oracle's
cube to the facts the rule that concludes a fact over the same match derives; the regression
`hypothesis-rule-absent` in [`corpus/regressions.toml`](../../../corpus/regressions.toml).
