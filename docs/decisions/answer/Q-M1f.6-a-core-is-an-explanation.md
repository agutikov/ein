# Q-M1f.6 — A core is an explanation

A dead node's core, and the `refuted by` beside it, is an explanation of why the node is dead, not part of the
answer. It may vary with the rules' written order and with the agenda's, as long as it is a valid core: it names
the node's commitment where the refutation needs it and, with root, derives the contradiction. Which core is
reported is not canonical; R, N as sets of nodes, every state and every count do not depend on it.

**Why.** A dead node usually has many refutations, and which one a search meets first is a matter of order. Making
the core a function of the program alone would tie the answer to an order that decides nothing else; keeping it an
explanation leaves every order free to be chosen for speed
([Q-M1f.313](../engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)).

**Held by.** `ein-infer/tests/corpus_cores.rs`: `every_core_names_its_commitment_and_derives_its_contradiction`;
`ein-infer/tests/agenda.rs`: `every_question_and_claim_answers_alike_under_the_agenda_and_the_written_order`, which
holds every field of every answer alike under two orders, a core being held as a valid explanation.
