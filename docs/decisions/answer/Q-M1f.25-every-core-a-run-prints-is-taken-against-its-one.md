# Q-M1f.25 — Every core a run prints is taken against its one snapshot of root

A run prepares root once. Every core it prints is taken through the node function against the snapshot every node
of that run is resumed from: a dead node's core as the route visits it, where a surface asks for those, and each
member of N's after the route, where `--dump-answer` or `--trace` asks for N's cores, past `--region-limit` too. A
member whose core the route already took keeps it. No surface prepares root a second time to take a core.

**Why.** A core is an explanation that may vary with the order a store keeps derivations in, and a second
preparation of root is a second store: one member of N could then be explained against two snapshots, depending on
whether the route happened to visit it. One preparation makes every core a run prints the core the run would take
eagerly.

**Held by.** `ein_infer::region::Cores`, the two fields `visited` and `nogoods`; `ein-cli/tests/answer_round_trip.rs`,
whose `eager_cores` holds each member of N's core in `nogoods.json` to the one an eager run writes in `nodes.json`,
over the corpus.
