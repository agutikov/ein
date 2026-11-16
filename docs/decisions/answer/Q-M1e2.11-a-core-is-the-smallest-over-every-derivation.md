# Q-M1e2.11 — A core is the smallest over every derivation the program records

A dead node's core is the smallest frontier over every derivation of its contradiction the program records. The
store keeps, per fact, at most `ein_core::kb::MAX_ALT_JUSTIFICATIONS` derivations, those with the fewest premises,
and beside them the arrival with the smallest closure (`Kb::closure`, `Kb::best`); a derivation it does not keep
can hide a smaller core. `(config :record-alternative-justifications false)` records each fact's first derivation
alone. It is the one option that may change a core, and with it which `refuted by` a refutation names; it changes
nothing else: R, every state, every count and every verdict word are the same either way.

**Why.** Recording every derivation of a fact derived many ways is unbounded, and a program such as
`examples/zebra.ein` solves at all only with the first derivation alone; a core is an explanation
([Q-M1f.6](Q-M1f.6-a-core-is-an-explanation.md)), so an option may choose among valid ones where it may not touch the answer.

**Held by.** `ein-cli/tests/lever_invariance.rs`: `no_config_flag_moves_a_line_of_any_mode`, every line of every
mode under every flag but the core's under this one, and `the_core_exemption_is_one_flag_and_the_core_rows`, which
holds the exemption to that flag and those rows on `examples/ein-bugs/alt-cap-core.ein`;
`ein-infer/tests/explain_semantics.rs`: `a_full_list_keeps_the_smallest_closure_and_the_pair_report_one_core`.
