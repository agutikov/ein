# Q-M1e.7 — A state is what the engine judges, and a model is a projection of it

The object the engine records, compares and judges is a node's **state**, `sat(root + C)`: dead, alive, owing and
discharged are each read on the whole state, its negations included. No route writes into a state anything its own
fixpoint does not derive, so every fact a rule reads there is a consequence of root and the commitment. A **model**
([Q-M1e.6](Q-M1e.6-a-model-is-what-a-solutions-state-adds.md)) is computed from the state where a surface prints
one, and decides nothing.

**Why.** A negation a state holds is entailed by it. Judging only the projection would hide a contradiction the state
entails, and keeping a fact in the state unread would record a state its own rules refute.

**Held by.** [Solution semantics](../../ein/reasoning/02-kb-evolution/06-solution-semantics.md) § 1.5, the state and the four
words read on it, and § 3, the model; `ein oracle`, which computes every state whole, and
`ein-infer/tests/explore.rs`: `every_mode_is_the_oracle`, which holds every route's answer to it.
