# Q-M1f.655 — Open facts are written at root and where a state is used

The pass that writes a state's `open` facts, what it owes as `(open R)` and its hypotheses as `(open F)`, runs at
root, where those facts make $H$. Above root it runs only at a state that is printed or saved — the read-out's
states and their forms, `--dump-answer`'s files, `ein oracle --states`, a trace, a saved file's node — and at a
state that uses them, which is every state the search builds where a rule of the program reads an `open` fact. At every other state of a search the state's verdict is read as at every state, what
it owes by the obligations' tally, and nothing is written: no surface reads what it would write. A state printed only
as a count or a verdict, a node of `--dump-lattice` or an `enter` line of `--events`, writes none.

What a state holds under `open` is the same wherever it is read: a state the pass runs at holds what the pass at
every state would write there, and the empty commitment's state is root's, the `open` facts the program wrote among
them. No answer, no printed state and no count moves: the counts of a node's work leave its `open` facts out.

**Why.** The user's ruling: *"per-state open pass - we can make it optional, by default run only on root to make an
H-set, then only on states that will be printed or saved or used eather way."* The writing matches every rule that
concludes a hypothesis and reads every activated obligation's guard at the state, and at a node of a search that
nothing prints or reads it was the largest share of what the pass added to the search.

**Held by.** `ein_infer::open::Pass` and `ein_infer::region`'s node function, which chooses it; `ein_infer::region::Nodes`,
every reader of a state, writes them; `ein-cli/tests/lever_invariance.rs`:
`no_config_flag_moves_a_line_of_any_mode`, which holds every corpus program whose R is known to one read-out with
the pass at every state and without; `ein-infer/tests/open_facts.rs`. The command line's option that runs it at every
state is [Q-M1f.656](../cli/Q-M1f.656-a-flag-runs-the-pass-at-every-state-and-checks.md).
