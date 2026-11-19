# Q-M1e2.12 — A dead node's core follows an activator only a node derives

A rule with parameters fires through the plan compiled from its activator, and every firing records that
activator (`Prov::activator`). Where a node derived or committed the activator, a dead node's core follows it: the
core names what the node added to compile the plan its refutation rests on. Root's activators, stated or derived,
are the program's, as the plans root compiles from them are its rules, and no core follows them.

**Why.** A commitment that reaches a refutation only by activating a rule is part of why the node is dead, and a
core without it names root's facts for a contradiction root does not have. Root's plans are what the program says,
and a core explains a node against the program, not the program against itself.

**Held by.** `ein_core::walks::node_activator`, which a core's walk follows. `ein-infer/tests/region.rs`:
`a_dead_nodes_core_follows_no_activator_of_roots`, the two programs whose activator is root's, stated
(`core-activator-given.ein`) or derived (`core-activator-root-derived.ein`). The other half is held by no test: a
program in which a node derives or commits an activator has an activator site, the static check flags it, and
nothing is claimed of its answer
([Q-M1f.564](Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)).
