# Q-M1f.574 — A hypothesis first held above root makes its program not monotone

A state above root that holds a hypothesis root does not makes the program not monotone, found two ways: **by the
static check**, a site of its own kind, `hypothesis`, for a rule that concludes `(open F)` and reads a relation a
commitment can change at an even `absent` depth, where growing it adds a match; and **in flight**, when a run
builds such a state where it checks, where the pass that writes a state's `open` facts runs: a state printed or
saved, every state of a program a rule of which reads an `open` fact, and every state under `--in-flight`
([Q-M1f.656](../cli/Q-M1f.656-a-flag-runs-the-pass-at-every-state-and-checks.md)). Found in flight, the answer stands and nothing is claimed of it: the warning is said on stderr
at once, once, and the read-out's `lattice` row names it.

**Why.** A monotone program, state by state, proves or refutes hypotheses and adds none: its lattice is a cube over
root's. A hypothesis first held above root breaks that, so the program is not monotone, whether the static check saw
it or the run met it.

**Held by.** `ein_infer::monotone`'s `hypothesis` site; `ein_infer::open::Writers::first_above`, the check at each state where the pass runs;
`ein_infer::monotone::Monotonicity::in_flight`; `ein-infer/tests/open_facts.rs`:
`a_hypothesis_first_held_above_root_is_found`.
