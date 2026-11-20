# Q-M1f.549 — A hypothesis is a fact `(open F)`, whoever writes it

Every writer of H writes each hypothesis as a fact `(open F)` the knowledge base holds: a rule that concludes one,
an obligation activated at root, which writes the facts that would pay what root owes, the blind enumerator, and the
program, which may write one ([Q-M1f.566](Q-M1f.566-what-a-state-owes-is-open-r-a-fact-and-open-f.md),
[Q-M1f.570](Q-M1f.570-h-is-the-union-of-the-hypotheses-root-holds.md)). `(open …)` is the context of every
hypothesis, and not only of an obligation's: an obligation says what a state must hold, a hypothesis what it may.
H is the hypotheses root holds, less the facts root holds or denies; one first held above root is held there and
proposed nowhere ([Q-M1f.573](Q-M1f.573-under-open-a-relation-is-owed-and-a-fact.md)), and its program is not
monotone ([Q-M1f.574](../answer/Q-M1f.574-a-hypothesis-first-held-above-root-makes-its.md)).

**Why.** The kernel has access to every fact anyway, and a hypothesis held as a fact is listed, drawn and read like
the rest of a state, as a stored negation is: one context more, and no list kept beside the knowledge base. A
hypothesis proposed above root would make the space of commitments a lattice that is no cube, which is a
non-monotone program by definition.

**Held by.** `ein_infer::hypgen`, every writer through one pipeline into root's `open` facts; `ein_infer::open`;
`ein-infer/tests/open_facts.rs`: `h_is_the_union_of_the_writers`,
`a_hypothesis_first_held_above_root_is_found`; [hypotheses](../../ein/user/03-syntax-and-semantics/09-hypotheses.md).
