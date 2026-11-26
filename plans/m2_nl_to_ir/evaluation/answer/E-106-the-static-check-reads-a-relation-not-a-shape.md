# E-106 — The static check reads a relation, not a shape

**Kind:** weakness
**Status:** open
**Filed:** by S2.2.0a, 2026-11-26, on `4bdbccaf`
**Owner:** none; the design ([Q-M1f.564](../../../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md):
the check is conservative, and a flagged program is answered with no claim)
**Since P2.2's close (2026-11-27):** the design ([Q-M1f.564](../../../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)); it weighs on [Q-M2.38](../../open_questions/formalizer/Q-M2.38-is-a-description-abbreviated-to-its-word-or-written-as-a-term.md)

## What

**The static check reads a relation, not a shape**: a relation is *dynamic* when any rule derives any fact of it from
what a commitment can extend, and every `absent` over a relation derived from a dynamic one is a site. A theory that
reads two kinds of sentence through one relation inherits the dynamism of either. In `zebra3.2`'s first two forms
the answer rule writes `(same ?man (the ?drink (is drunk by) Man))` from `same-house`, which a commitment extends,
so `same` is dynamic; the sentences of the row were `same` facts too, `(same (the Norwegian (lives in) House) (the
House (next to) (the Blue House)))`, read into `next-to`, which `std.slots`' `absent` guards read: sixteen sites, and
the program flagged. The facts of `same` the row's sentences are, `(same ?x (the House (next to) ?y))`, are never
derived by the answer rule (another constant in the second place), and the check cannot see it. The same happened
with a helper relation, `means`, shared by the two readings.

## Evidence

[`../zebra_formulations/probes/`](../../p2.2_formalizer/zebra_formulations/probes/): `ein solve zebra3.2-spatial-as-same.ein
--no-search` and `ein solve zebra3.2-read-through-means.ein --no-search` print the capitals warning and `not
monotone: 16 sites`; `--json-summary`'s `monotonicity.sites` names `std.slots.slot-adjacent-*`, `-prune-*` and
`-endpoint-*` over `next-to` and `right-of`, and `dynamic` holds `same` and `next-to`. `examples/zebra3.2.ein`
writes the three sentences of the row as facts of the row's relations between descriptions, and is monotone.

## What it costs the application

A literal vocabulary has fewer relations than a theory has uses for them, so a formalizer that reads two kinds of
sentence through one relation meets a flag it cannot explain from the shape it wrote; the way out is a vocabulary
choice (a relation a use), which the contract has to state. The cheaper reading of `zebra3.2` is out of reach for this reason: descriptions read through `means` take 36 222
facts for the description where descriptions placed in `same-house` take 60 194, both without the answer rule
(the drafts with their query and answer rule taken out, `ein solve FILE`'s `work` row).
