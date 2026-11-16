# Q-M1f.108 — Route 1's stop at a layer is the library's, and no `ein` run prints it

The read-out has an arm for route 1 stopped by the budget, *stopped before layer L*, and no `ein` run reaches it.
Describe takes route 1 only on a flagged program whose root is dead: layer 0 is root, projected at one fact, which the
smallest `--region-budget` the command takes admits, and a dead root leaves layer 1 empty. The arm stays for
`ein_infer::explore::describe_by`, which takes route 1 on a monotone program and can stop it at the budget; no
option of `ein` chooses route 1.

**Why.** A library caller that asks for route 1 gets a read-out that says where it stopped; the page says when a row
is printed, so a row no command prints is stated, not hidden.

**Held by.** `ein-cli/tests/read_out_reference.rs`: `route_1_never_stops_on_an_ein_run`, the one shape describe takes
route 1 on exhausting at `--region-budget 1`. The row: [read_out.md](../../kernel/inference/read_out.md) § 2.
