# Q-M1e2.21 — `refuted by` names the first recorded derivation the core supports

Beside a dead node's core, `refuted by` names, for each fact of the contradiction the core was taken over, the rule
of the last step of a derivation from the core: the fact's first recorded justification where the core derives it,
else the first alternative the store keeps that the core derives, fewest premises first, else the smallest-closure
arrival; `(given)` for a fact the core holds itself. One rule for a `(false)`, up to two for a pair. Where the core
derives a fact through two rules, the store's order picks one, so two programs whose rules are written in another
order may name two rules beside one core: `refuted by` is an explanation, as the core is
([Q-M1f.6](../answer/Q-M1f.6-a-core-is-an-explanation.md)). Under `:record-alternative-justifications false` it
moves only where the core moves ([Q-M1e2.11](../answer/Q-M1e2.11-a-core-is-the-smallest-over-every-derivation.md)).

**Why.** The rule named must be one the printed core's own derivation ends in, not merely the first the detector
met. Of the rules that qualify, the first recorded one keeps the alternatives flag's exemption to the core alone:
with the flag off the store keeps each fact's first justification, which is a derivation from the same core.

**Held by.** `ein_infer::explain`'s `Explanation::steps`, and `region::Node::refuted_by`.
`ein-infer/tests/corpus_cores.rs`: `every_core_names_its_commitment_and_derives_its_contradiction`, which holds every
printed core's rules to the first recorded step of each fact of its contradiction over the corpus.
`ein-cli/tests/lever_invariance.rs`: `the_core_exemption_is_one_flag_and_the_core_rows`. The row:
[read_out.md](../../kernel/inference/read_out.md) § 6.
