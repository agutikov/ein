# Q-M1f.214 — A firing's *Derives* line names every conclusion, in the rule's order

A step of `--trace` names every conclusion its firing stored on one *Derives* line, in the order the rule writes
them, `; ` between two, each as written, *Derives `q(a)`; `flag(a)`; `r(a, a)`.*, since a fact has no `:why` of
its own; a conclusion that is a declaration the program wrote is followed by its statement
([Q-M1f.58](Q-M1f.58-a-fact-reads-in-english-in-the-trace.md)). A rule of one conclusion prints one conclusion.
Under `--reorder` the step's arrow lists the same conclusions, `, ` between two.

**Why.** A conclusion stored and read by later steps must be named where it was derived; one line per firing keeps a
count of *Derives* lines a count of steps.

**Held by.** `ein_render::trace::region`'s `derives_line`; `ein-cli/tests/region_modes.rs`:
`a_trace_step_names_every_conclusion_of_its_firing`.
