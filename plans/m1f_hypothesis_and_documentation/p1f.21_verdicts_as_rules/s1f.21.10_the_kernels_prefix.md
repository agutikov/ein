# S1f.21.10 — the kernel's prefix

**Phase:** [P1f.21](README.md)
**Status:** planned 2026-11-19; **not run: ruled away the same day.** The user's answer to
[Q-M1f.399](../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open), verbatim:
*"no need to reserver `ein.` prefix; make (false) an stdlib convention; add rules detecting contradictions infereing
(false) and then - we can make another key, not :assert, but special engine control key `:ein false` which argument
is not a fact, it can remain even the same word `false` but see plans/m1g_logic/p1g.40_proof_theory/ideas2.md and
maybe propose better name here"*.
**Group:** [F](modifications.md#f-q-m1f399-a-word-of-the-language-renamed), items 21 to 23.

## What became of its items

| item | what | now |
|---|---|---|
| 21, 22 | a new word for `(false)` | **not built**: `false` is not renamed, and is a convention of the standard library |
| 23 | the prefix `ein.` reserved for the kernel's surface | **not built**: no prefix is reserved, and no fact is the kernel's. A file that writes `(ein.kill)` stays what it is today, a program with an ordinary fact of an undeclared relation |

What the ruling puts in the prefix's place, **a rule's key `:ein` whose argument is a word of the engine's and no
fact**, is built by [S1f.21.11](s1f.21.11_kill_and_three_conventions.md), its first build step, where the fact
`(ein.kill)` was planned.

## The names this stage was to put, and where each is now

| what | where |
|---|---|
| what ends a saturation | the argument of `:ein`: [Q-M1f.550](../open_questions.md#q-m1f550--the-word-a-rule-gives-the-engine-to-end-a-state-the-argument-of-ein), open; proposed `inconsistent` |
| a candidate, under [Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules) (b) | [S1f.21.12](s1f.21.12_hypotheses_as_facts.md)'s own question; with no prefix it is a word of the standard library's or of the key's |
| an obligation's verdict, `(open ?R)` | as today; [Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact)'s |
| a relation no commitment extends, `__closed__` | as today; S1f.21.12's |

## Record

**2026-11-19.** Not run. The ruling is filed under Q-M1f.399 in the ledger; nothing was built or changed in the
project for this stage.
