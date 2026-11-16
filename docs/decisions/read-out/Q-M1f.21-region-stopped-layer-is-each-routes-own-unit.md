# Q-M1f.21 — `region.stopped.layer` is each route's own unit

When a budget stops a description, the summary's `region.stopped.layer` is the stopping route's own layer: on route 1
the commitment size it stopped before (*stopped before layer L*); on the state route its layer, a state's size plus
one (*the states of k candidates*, k one less). `region.complete_up_to` is route 1's last complete commitment size,
and `null` on the state route, whose *complete up to* no field holds.

**Why.** Each field reports what its route counts; the rows on stdout say the same in words, and no answer depends
on the unit.

**Held by.** `ein_render::dump::region_answer`, the summary's writer;
[read_out.md](../../kernel/inference/read_out.md) § 11, held to the printer by `ein-cli/tests/read_out_reference.rs`.
