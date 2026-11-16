# Q-M1f.214 — A firing's *Derives* line names every conclusion, in the rule's order

A step of `--trace` names every conclusion its firing stored on one *Derives* line, in the order the rule writes
them, `; ` between two, each followed by its own statement where its relation has a `:why`:
*Derives `q(a)` — a is q; `flag(a)`; `r(a, a)` — a is related to a*. A rule of one conclusion prints one conclusion.
Under `--reorder` the step's arrow lists the same conclusions, `, ` between two.

**Why.** A conclusion stored and read by later steps must be named where it was derived; one line per firing keeps a
count of *Derives* lines a count of steps.

**Held by.** `ein_render::trace::region`'s `derives_line`; `ein-cli/tests/region_modes.rs`:
`a_trace_step_names_every_conclusion_of_its_firing`.
