# E-132 — The whole node's core search reads no clock

**Kind:** bug
**Status:** open; **postponed** on the user's word at P2.2e's close, 2026-11-27: *"postpone E-132"*
**Filed:** by [S2.2e.7](../../p2.2e_kernel_fixes/s2.2e.7_two_more_cores_only_where_printed.md), 2026-11-27, on `87d5db47`, measured
**Owner:** none; the iteration of Ein's, or a stage of M2 that needs it, the user's word deciding (a fix leaves Ein's
syntax and semantics as they are and moves no printed byte: the run's deadline handed to the whole node's core
search, as S2.2e.3 handed it to root's)

## What

**The whole node's core is searched with no clock, where every other dead node's is searched under the run's.** The
obligation tree reads the whole node, every candidate together, for the coupling of its choices (`tree.rs`,
`read_structure`), and where that node is dead and a surface that prints every visited node's core reads the run
(`--events`, `--dump-lattice`), `structure::coupling::node` takes its core through `region::core_of_fixpoint`, which
calls `explain::contradiction_frontier`: the search with no deadline (`contradiction_frontier_by(…, None)`). The node
function's own search reads the run's clock (`snapshot.bound().deadline`), and the API page's `deadline` row says the
clock is read *inside every dead node's core search (`explain::contradiction_frontier_by`)*
([`04-api.md`](../../../../docs/ein/ein.rs/04-api.md)), which is not true of this one. So where the whole node dies
on a link several rules derive, the search runs to its end past `-T`, and the run is the clock's abort at the clock's
next read, after the search.

Since S2.2e.7 the whole node's core is taken only where such a surface reads the run; before it, `-v` and
`--layer-progress` took it too ([E-131](../../p2.2e_kernel_fixes/evaluation/bounds/E-131-two-more-cores-a-question-takes-and-prints-nowhere.md)), so the same
overrun was theirs.

## Evidence

The probe [`probes/two-more-cores-only-where-printed/whole-node-clock.ein`](../../p2.2e_kernel_fixes/probes/two-more-cores-only-where-printed/whole-node-clock.ein),
a monotone program whose whole node dies on `dead_link.ein`'s link, on `87d5db47`'s release binary, pinned to a P-core,
on AC:

| question | clock | exit | wall (s) | what it says |
|---|---|---|---|---|
| `ein solve --single` | `-T 5` | 0 | 0.62 | *Ambiguity*: the tree reads the whole node, dead, and takes no core |
| `ein solve --single -v` | `-T 5` | 0 | 0.62 | the same: a watch alone takes no core since S2.2e.7 |
| `ein solve --single --events E` | `-T 5` | 2 | 18.47 | `** aborted: --max-time 5 s reached before the question was decided **`, after the search |
| `ein solve --single --dump-lattice D` | `-T 5` | 2 | 18.41 | the same |

Each wall includes `utils/bench_env.sh`'s fingerprint, some 0.1 s. The probe is
`ein.rs/crates/ein-cli/tests/bounded/dead_link_whole.ein` with its far rule on Ann in two houses, so that no node the tree commits holds both and the whole
node is the one dead node the run visits (the event stream's one dead `enter` line is the whole node's, at layer 25,
its core `cut-environments`). On `dead_link_whole.ein` itself the run under `--events` meets dead nodes the node
function computes before the whole node, whose searches read the clock, and ends at 5.09 s.

## What it costs

A run asked under a clock with `--events` or `--dump-lattice`, on a program whose whole node the tree reads and whose
core's search is long, ends past its clock by what the search costs. EinAf's bounded call narrates no region run to a
surface that prints a core, and takes no whole node's core; nothing of EinAf is reached.

## A fix

Not made: the stage's question is which cores a question takes, and this is the clock's reach, which
[S2.2e.3](../../p2.2e_kernel_fixes/s2.2e.3_clock_and_a_dead_root.md) set. `region::core_of_fixpoint` would take the run's deadline and call
`contradiction_frontier_by`, and `structure::coupling::node` would carry its `Deadline` out of the reader as the node
function does (`SolveError::Deadline`). No answer moves: a run the clock does not reach takes the same core, and one it
reaches is the clock's abort, as it is now, at the clock rather than after the search.
