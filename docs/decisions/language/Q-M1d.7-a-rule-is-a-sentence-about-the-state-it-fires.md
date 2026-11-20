# Q-M1d.7 — A rule is a sentence about the state it fires in

A rule reads and concludes facts of the one state it fires in. No form of the language reads another state, the
number of states, or the search's own progress: a program cannot require its own model count, that there is exactly
one answer, or that it has the answers of another file. A claim about the set of states is stated at the meta
level, by a claim (`:expect`), which the engine checks against the region; a claim the run did not decide is
reported as not checked, and is never read as held.

**Why.** Another model is another state, a fact in no state's store, so no guard can test it, and what a rule
concludes under `open`, what a state owes and what it supposes, is written by one pass at its own state's fixpoint.
A rule that read the search would make what is derived depend on how far the search went, so a budget would change
what is provable, and no option of the engine changes the answer.

**Held by.** The grammar, `ein-ir/src/ein.y`: a rule's `:assert` is `()` or a conclusion, `(open …)` among them,
or the rule states a word to the engine under `:ein`, and no premise names a state.
`ein-cli/tests/lever_invariance.rs`: `every_declared_lever_cell_moves_no_line_of_any_mode`;
`ein-infer/tests/explore.rs`: `every_mode_is_the_oracle`; `ein-infer/tests/expect_semantics.rs`:
`a_claim_the_run_did_not_decide_is_not_checked`.
