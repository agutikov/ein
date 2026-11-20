# Q-M1f.656 — A flag runs the pass at every state and checks each in flight

`--in-flight`, on `ein solve`, `ein test`, `ein oracle` and `ein render lattice`, runs the pass that writes a
state's `open` facts at every state the run builds, where by default it runs at root and where a state is printed,
saved or reads them ([Q-M1f.655](../engine/Q-M1f.655-open-facts-are-written-at-root-and-where-a-state.md)). The
check in flight is made where the pass runs: at each such state the run checks whether it holds a hypothesis root
does not, which makes the program not monotone, found in flight
([Q-M1f.574](../answer/Q-M1f.574-a-hypothesis-first-held-above-root-makes-its.md)). By default that is a state printed
or saved, and every state of a program a rule of which reads an `open` fact; under the option, every state. What a
printed state finds is folded into what the run reports, the read-out, the summary and `ein oracle --dump`'s
`region.json` alike.

It changes no R and no printed state. What the check finds is what it can add: the read-out's `in flight` row, the
stderr line said at once, and the summary's `monotonicity.in_flight`, and, on a program the static check found no
site in, the warning that a finding makes of it. It costs the pass at every state.

**Why.** The user's ruling: *"also can have cmdline flag that enables firing H-set for every state to detect
non-monotonicity in-flight"*. The check in flight needs the hypotheses each state's writers give, which is the work
the default leaves out where nothing prints or reads it.

**Held by.** `ein-cli`'s `cmdline::in_flight`, and `ein_infer::open::at_every_state`, which each route reads as it
builds its axes; `ein-cli/tests/lever_invariance.rs`: `no_config_flag_moves_a_line_of_any_mode`, every corpus
program whose R is known asked every question with the option and without, one read-out but for what the check
finds; `ein-infer/tests/open_facts.rs`: `a_hypothesis_first_held_above_root_is_found`. The row:
[06-config.md](../../ein/user/04-reference/06-config.md#33-what-an-option-can-do-to-the-answer).
