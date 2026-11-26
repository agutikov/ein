# E-56 — A row's value reaches a caller of the library as text

**Kind:** gap
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none; [M1p](../../../m1p_ein_to_natural_language/README.md)'s ground

## What is missing

**A row's value reaches a caller of the library as text**, in the main file's spelling, and the query's `:goal-text` template is not a field of the read-out's query: a binding row is pairs of strings

## Evidence

`ein_infer::read_out`'s `Row` and `Query`


Confirmed by S2.2.5, 2026-11-26 (`8c2037b0`): the answer's formatters, `einaf::format`, read each value's text back
as a term and resolve its names to their full names from the program's imports (`einaf::format::Names`), so that the
decimal formatter matches `std.math.decimal.n` whether the file prints `(n z 3)`, `(imp3.n imp4.z 3)` or the full
names; held on the sixteen word problems' own programs under the three forms of an import
(`einaf/tests/from_text.rs`, `the_word_problems_print_no_numeral_as_a_term_under_every_import_form`). The template is
read off the program's parse by `einaf::kernel` (the feedback object's `queries[].goal_text`), not off the read-out.

## What it costs the application

a formatter *over the library's values* parses `(n z 3)` again
