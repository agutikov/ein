# E-97 — A run's diagnostics are a value only of the rendered read-out, and their order is the binary's

**Kind:** gap
**Status:** open
**Filed:** by S2.1.3, 2026-11-25, on `0ab1aef0`
**Owner:** none

## What is missing

**A run's diagnostics are a value only of the rendered read-out, and their order is the binary's**: the partial notes and each claim not checked or refuted are `ein_render::read_out::Text::diagnostics`, which a caller has only by rendering the read-out it does not print; the in-flight warning is said by a hook the binary sets (`ein_infer::open::set_in_flight_hook`) and the load's notes are printed last by `ein-cli`; a query's `:goal-text` template is in no `read_out::Query`; and the program's `(config :lattice-sanity-check true)` reaches a run only where the caller copies it into `Options::sanity`, as `ein-cli`'s `resolved_config` does

## Evidence

the feedback object's `diagnostics`, `queries[].goal_text` and the run's options ([`einaf/src/kernel/`](../../../../einaf/src/kernel/mod.rs)); held equal to the summary's by E-94's test

## What it costs the application

the application re-does the binary's assembly of what a run says, and a caller that does not is told less than `ein solve` prints
