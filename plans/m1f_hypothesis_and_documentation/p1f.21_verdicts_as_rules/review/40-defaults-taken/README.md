# 40 · The defaults taken and built: Q-M1f.552 to Q-M1f.556

[← the topic](../README.md) · [← the phase](../../README.md)

**Part 1. Each is built, on a default; each is yours to revise.** Five readings the two finished stages took as
they ran. One was not taken as its default read (the fourth). Nothing is asked unless you want one changed.

| | question | what was taken, and built | the other reading | where it shows |
|---|---|---|---|---|
| 1 | [Q-M1f.552](../../../open_questions.md#q-m1f552--the-saturated-dumps-sections-for-the-rules-no-agenda-holds): the saturated dump's sections for the rules no agenda holds | a section each, `;; Obligation rules (N)` and `;; Hypothesis rules (N)`, the first heading `;; Rules — the saturation agenda's (N)`; and, past the default's words, the count of a rule's activators beside it, `no activator` where nothing activates it | the heading's words alone | `ein saturate --dump examples/zebra2.ein`; `8d8327ce` |
| 2 | [Q-M1f.553](../../../open_questions.md#q-m1f553--the-words-for-a-state-dead-by-a-fact-beside-its-negation): the words for a dead state | *holds a contradiction*, in the read-out's `empty` and `∅` rows, the region's legend, the h-space's sentence, the pages that quote them, eight region drawings | left as *derives (false)* until it is literally true; or, since the key's word was ruled, *is inconsistent* (item [41](../41-S1f.21.10-readings/README.md), reading 7) | `ein solve` of any program whose root is dead; `c9ea9597` |
| 3 | [Q-M1f.554](../../../open_questions.md#q-m1f554--a-saved-file-for-an-owing-node-below-a-non-empty-r): a saved file for an owing node below a non-empty $R$ | **not built**; the case is [V-9](../../findings.md#v-9--an-owing-node-below-a-non-empty-r-has-no-saved-file) | `--dump-answer` writes the owing frontier whether $R$ is empty or not | `ein solve examples/lattice/05_owed_seats_bounded.ein --dump-answer D` writes two models and nothing for `{(seats Ann C1)}`, which owes 2 |
| 4 | [Q-M1f.555](../../../open_questions.md#q-m1f555--where-the-rules-of-a-stored-negation-live): where the rules of a stored negation live | **not the default**: two modules, `std.negation` (the elimination) and `std.contradiction`, since the elimination is refused beside four corpus programs and an import is whole (item [38](../38-Q-M1f.559/README.md) is that refusal); `clash-of-N` fire only where a program writes `(clash-of-declared)`, since a rule with no parameter has no activator; neither module declares a relation | one module for both, the default as planned; `std.logic`, kept free for M1g | [the rules' page](../../../../../docs/ein/user/04-reference/04-stdlib/20-negation-and-contradiction.md); `b0ee99f4` |
| 5 | [Q-M1f.556](../../../open_questions.md#q-m1f556--the-counts-of-arguments-the-negations-rules-are-written-for): the counts of arguments the rules are written for | 0 to 6, for the contradiction and the elimination | what the corpus needs and no more: it negates facts of 0, 1 and 2 arguments at root | [`stdlib/contradiction.ein`](../../../../../stdlib/contradiction.ein); `b0ee99f4` |

What the stages checked is in their records: [S1f.21.6](../../s1f.21.6_surfaces.md#record) (no answer moved over the
899 corpus files, old binary against new) and
[S1f.21.8](../../s1f.21.8_rules_the_standard_library_states.md#record) (the two invariances with the modules
imported). The batch's gate ran in the runner's image on the merge, `ca98dec2`, and is green.
