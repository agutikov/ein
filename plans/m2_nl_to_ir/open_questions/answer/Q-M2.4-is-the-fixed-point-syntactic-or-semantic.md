# Q-M2.4 — Is the fixed point syntactic or semantic?

**Status:** open

**Arrived 2026-08-23 with the reshape.** The research plan's E4 terminates
on *repeated identical theory* and detects cycles *by canonicalized program
hashes* — syntactic. The user's own note says the fixed point should be
*semantic stabilisation*, `Cl(Tᵢ₊₁, Pᵢ₊₁) = Cl(Tᵢ, Pᵢ)`
([F13](../../../followups/f13_puzzles_beyond_zebra/ideas.md)), because an LLM can
rewrite equivalent rules forever without the closure moving.

**The engine can do both, today.** `ein-ir` has a canonical dump, which is the
syntactic hash; and [`06-solution-semantics.md` § 1.5, the state class](../../../../docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md#15-commitment-node-state)
pins that *canonical state identity is the sorted fact list itself*, which
makes the saturated state's digest — or the `.einb` of a saturated KB — the
semantic one, and the summary's sorted `solutions` the model-set one. So the
question is not which is computable but which the loop *stops on*.
**Recommendation, to be decided in [S2.4.1](../../p2.4_loop/s2.4.1_state_machine.md):**
the syntactic hash is the cheap first check and catches the literal
resubmission; the semantic digest is the termination criterion; and both are
logged per transition, because the *difference* between them — a theory that
changed its text and not its closure — is itself a measurement of how much
repair is cosmetic.

**Since (the re-plan, 2026-11-25).** Three things this entry leans on are not as it says ([`analysis/audit_p2.4.md`](../../analysis/audit_p2.4.md)): the reasoning page says a digest never decides a state's
identity, the summary has no `solutions` key, and `.einb` is a private cache, never something to compare by. EinAf
links the library and has a program's canonical dump and a state's facts as values, so the two identities are
compared as texts and lists. [S2.4.1](../../p2.4_loop/s2.4.1_state_machine.md) keeps the question and gives *semantic*
three candidates: root's fixpoint, the answer, and the answer as printed; its default is the second.
