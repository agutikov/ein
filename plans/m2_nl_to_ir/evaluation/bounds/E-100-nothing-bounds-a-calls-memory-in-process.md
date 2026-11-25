# E-100 — Nothing bounds a call's memory in process

**Kind:** gap
**Status:** open
**Filed:** by S2.1.3, 2026-11-25, on `0ab1aef0`
**Owner:** none

## What is missing

**Nothing bounds a call's memory in process**: the cap on firings bounds a runaway root's, at about 0.7 GB at its default, and no bound of the kernel's reads memory; the loop asks for a memory bound as a parameter of the run ([S2.4.1](../../p2.4_loop/s2.4.1_state_machine.md))

## Evidence

[`one_rule.ein`](../../analysis/probes/p2.1/one_rule.ein): `einaf outcome one_rule.ein --single`, *not decided* at `--max-steps 1048576` after 1.3 s, peak 727 MB; at `--max-steps 1000`, 0.01 s and 14 MB

## What it costs the application

a memory bound needs the kernel in a process of its own, under an rlimit (E-61: the binary does not start under `ulimit -v`); the worker process stays unbuilt until a case needs it
