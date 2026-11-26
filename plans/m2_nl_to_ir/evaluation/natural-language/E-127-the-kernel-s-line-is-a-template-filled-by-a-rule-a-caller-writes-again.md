# E-127 — The kernel's line is a template filled by a rule a caller writes again, byte for byte

**Kind:** strength
**Status:** standing (a strength)
**Filed:** by S2.2.5, 2026-11-26, on `8c2037b0`
**Owner:** —
**Since P2.2's close (2026-11-27):** — ; [M1p](../../../m1p_ein_to_natural_language/README.md) starts from it

## What

**The kernel's line of an answer is its `:goal-text` filled per row by a rule of four sentences** (`ein_core::render_why`:
`{?`, a name's characters up to the first `}`, `}`, replaced where the row binds it and left as written where it does
not), **and a caller that writes the rule again gets the kernel's line, byte for byte**. EinAf's filler,
`einaf::format::fill`, twenty lines of its own over the row the feedback object hands it, gives the kernel's line on
every row of every query with a `:goal-text` of every corpus program that is not `slow`, under every question: 5 187
lines of 183 programs. So a formatted answer is the template filled again with a value written otherwise, and never
an edit of the kernel's line, inside which a value's text is not unique (`boxes13.ein`'s line holds `(n z 5)` and a
bare `z` together).

## Evidence

`cargo test --manifest-path einaf/Cargo.toml --profile gate --test corpus -- the_answer_line --nocapture`
(*the answer line with no formatter: 5187 lines of 183 programs with a :goal-text*).

## What it serves

The answer's formatters, which M1p starts from: the row keeps each value apart under its variable, and the rule is
small enough to hold a second writer to it. What it does not carry is
[E-56](../library/E-56-a-rows-value-reaches-a-caller-of-the-library-as-text.md)'s: the template is not a
field of the read-out's query, and a value is its text.
