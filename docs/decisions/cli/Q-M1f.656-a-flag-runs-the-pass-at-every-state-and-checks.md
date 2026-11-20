# Q-M1f.656 — A flag runs the pass at every state and checks each in flight

`--in-flight`, on `ein solve`, `ein test`, `ein oracle` and `ein render lattice`, runs the pass that writes a
state's `open` facts at every state the run builds, where by default it runs at root and where a state is printed,
saved or reads them ([Q-M1f.655](../engine/Q-M1f.655-open-facts-are-written-at-root-and-where-a-state.md)). At each
state it runs at for the search, under the option or on a program a rule of which reads an `open` fact, the run checks
whether the state holds a hypothesis root does not, which makes the program not monotone, found in flight
([Q-M1f.574](../answer/Q-M1f.574-a-hypothesis-first-held-above-root-makes-its.md)). A state the pass runs at only to be
printed or saved is not checked, so every surface of one run reports one finding.

It changes no R and no printed state. What the check finds is what it can add: the read-out's `in flight` row, the
stderr line said at once, and the summary's `monotonicity.in_flight`. It costs the pass at every state.

**Why.** The user's ruling: *"also can have cmdline flag that enables firing H-set for every state to detect
non-monotonicity in-flight"*. The check in flight needs the hypotheses each state's writers give, which is the work
the default leaves out where nothing prints or reads it.

**Held by.** `ein-cli`'s `cmdline::in_flight`, and `ein_infer::open::at_every_state`, which each route reads as it
builds its axes; `ein-cli/tests/lever_invariance.rs`: `no_config_flag_moves_a_line_of_any_mode`, every corpus
program whose R is known asked every question with the option and without, one read-out but for what the check
finds; `ein-infer/tests/open_facts.rs`: `a_hypothesis_first_held_above_root_is_found`. The row:
[06-config.md](../../ein/user/04-reference/06-config.md#33-what-an-option-can-do-to-the-answer).
