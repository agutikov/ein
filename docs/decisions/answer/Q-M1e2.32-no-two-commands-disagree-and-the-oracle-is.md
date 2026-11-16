# Q-M1e2.32 — No two commands disagree, and the oracle is the reference

Ein has one answer to a program, R, and every command and every path through a command that speaks of it states the
same R: a run that describes R describes the oracle's region; a line a mode reports is a part of it; a state any run
reports is a member of it. This holds of a monotone program and of one the static check flags alike. `ein oracle`,
R by brute force over every node, is the reference every mode is held to. A faster route is taken only where a
licence the static check proves makes it sound; elsewhere a run reports a part of R, never a different R. Every
command reads a program by one path, `.ein` or `.einb` alike, so a program one command refuses no other answers.

**Why.** A fast path that can disagree with the reference makes every answer untrustworthy, the fast ones included,
since a reader cannot tell which path printed it. One definition and one reference make every route, mode and view
trustworthy alike; a behaviour that disagrees is retired, not kept beside the right one. A surface called
experimental is held to it too ([Q-M1e.5](Q-M1e.5-no-surface-disagrees-with-the-oracle.md)).

**Held by.** `ein-infer/tests/explore.rs`: `every_mode_is_the_oracle`, `every_order_of_the_tree_is_the_oracle`;
`ein-infer/tests/claim_routes.rs`: `every_claim_is_decided_as_the_oracle_decides_it`;
`ein-cli/tests/route_invariance.rs`: `the_state_route_and_route_1_print_one_read_out`; `ein-cli`'s
`common::read_program`, the one read path.
