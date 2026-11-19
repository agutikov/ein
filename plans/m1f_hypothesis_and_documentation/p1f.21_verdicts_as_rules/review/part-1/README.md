# Part 1 — the verdict *dead*: `false`, `not`, `relation`, the key

[← the bundle](../README.md) · [← the phase](../../README.md) · [your answers](ANSWERS.md)

**No question of this part is open.** You answered its six items on 2026-11-19; each answer is filed in your words
under its question in [the ledger](../../../open_questions.md) and in
[the phase's README](../../README.md#the-rulings-of-the-answers-file), and the items are out of the bundle.

## Read off your answers, and said back

Three things your answers do not say in so many words. Each is built only after you have seen it, and a word from
you changes it.

| # | your words | read as | where it waits |
|---|---|---|---|
| 1 | item 38: *"forbid becomes a warning, fix :hrulse activators syntax"* | the first half is about **that item's refusal**, which is not *forbid*: the loader refuses a program in which a rule *could* derive an activator that puts a fact where a relation's name goes. Read as: it becomes a warning and the program loads, which reverses your *"Q1 - b"* of 2026-11-08 | [S1f.21.17](../../s1f.21.17_the_loader_warns_and_std_core_by_import.md)'s task 1, not built before your word |
| 2 | item 35: *"… then it should also import std.contradiction"*; item 40: *"as built"* | the two differ on one fact: as built, a program writes `(clash-of-declared)` beside the import to ask for the rules. Read as: the import alone is the switch, and that fact goes. Also: the library's modules that conclude a negation import the module, and each corpus program that relies on the death gains the import, so no corpus answer moves | [S1f.21.11](../../s1f.21.11_kill_and_three_conventions.md)'s task 1.1, shown on the corpus first |
| 3 | item 36: *"… relations overloading … different number of args"* | a relation may be **declared** once a count; today a second declaration of a name is refused as a duplicate | S1f.21.11's task 1.4, shown first |

## What was ruled

| item | question | ruled |
|---|---|---|
| 35 | Q-M1f.557 | (d): the contradiction is a theory a program imports; without `std.contradiction` a program has none, or its own |
| 36 | Q-M1f.447 | (c): the argument-count check is dropped; a relation at several counts is overloading by count |
| 38 | Q-M1f.559 | the `:hrules` clause lists activators whole (Q-M1f.565); the rest as row 1 above; the elimination stays a module a program imports |
| 40 | Q-M1f.552 to .556 | as built |
| 41 | Q-M1f.562, Q-M1f.563 | `(import std.core)` works as a duplicated import; `$EIN_STDLIB` may replace `std.core`, and a library there without one is an error that says so; no backward compatibility until you ask |
| 42 | the findings | each disposition as proposed |

## Out of the bundle

Each item's page and programs are in `git log`: items 35, 36, 38, 40, 41 and 42, last at `bedbfbd4`; item
33, the names (Q-M1f.399, Q-M1f.550), last at `984137bf`; item 37, *forbid* once the contradiction is rules
(Q-M1f.558), ruled against both its options by Q-M1f.564; item 43, one way to say *must* and one to say *may*
(Q-M1f.561), declined, its analysis and programs in [`one_way_each.md`](../../one_way_each.md).
