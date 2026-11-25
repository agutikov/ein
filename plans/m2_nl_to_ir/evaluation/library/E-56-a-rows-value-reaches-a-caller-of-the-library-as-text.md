# E-56 — A row's value reaches a caller of the library as text

**Kind:** gap
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none; [M1p](../../../m1p_ein_to_natural_language/README.md)'s ground

## What is missing

**A row's value reaches a caller of the library as text**, in the main file's spelling, and the query's `:goal-text` template is not a field of the read-out's query: a binding row is pairs of strings

## Evidence

`ein_infer::read_out`'s `Row` and `Query`

## What it costs the application

a formatter *over the library's values* parses `(n z 3)` again
