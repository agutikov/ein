# P1p.1 — custom formatting

**Milestone:** [M1p](../README.md)
**Status:** created 2026-11-20 on the user's instruction; **not planned into stages, not started**.

## The instruction, its part

> (2) add custom fast formatting, convertion to strings, of variables, e.g. decimal numbers.

## What it is

Where a text names a variable, `{?x}` in a `:why`, the value the variable holds is written as Ein writes it. For an
atom that is its name. For a term it is the term: a decimal number of the calculator is a nested term of digits,
and reads as one where a reader wants `12`. This phase gives a value a way to say how it is written as text, said
by whoever owns the value, the calculator for its numerals, and makes applying it cheap.

## Open, for its planning

Nothing here is ruled; each is a question the phase's first stage puts, on real programs.

1. **Who says how a value is written.** The kernel reads no theory's name, so it cannot know a numeral. Is a format
   a thing a program or a module declares, in Ein, that a surface applies; and in what form?
2. **What *fast* asks.** A text is made when a line is printed. Is the format applied by the surface, with no
   saturation behind it, and what does it cost on the calculator's own traces, where every step names numbers?
3. **What a format may read**: the term alone, or facts about it too (a numeral that is labelled valid, a name a
   program gave an object).
4. **Where it is applied**: every `{?x}` of a `:why`, a `:source` and a `:goal-text`; a query's answers; the
   read-out's rows that print a fact for a reader. And where it is not: a claim and a `(model …)` block are Ein.
5. **Its other direction.** Whether the text a format writes is one a reader can turn back into the value, which is
   M2's side of the round trip.
6. **The first formats**: decimal numbers; then binary numerals, integers, rationals, and what else the library
   builds as terms.
