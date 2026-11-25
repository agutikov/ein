# E-66 — The seven outcomes are read off values, and eight questions agree on every corpus program

**Kind:** strength
**Status:** standing (a strength)
**Filed:** by S2.1.1, 2026-11-25, on `0effa7b0`
**Owner:** none

## What

**The seven outcomes are read off values, and eight questions agree on every corpus program.** No outcome but *invalid* reads a line of text: the verdict word, `stopped`, `exhausted`, the description's states and lines, whether a node is alive, the monotonicity and the warning are fields of `ReadOut`. Over the 917 corpus programs that are not `slow`, every question that reads one part of R agrees with every other where both decided it, `--no-search` says *inconsistent* exactly where the others do, and every claim the kernel holds (827) allows the outcome over its part

## Evidence

`cargo test --manifest-path einaf/Cargo.toml --profile gate --test corpus -- --nocapture` ([`corpus.rs`](../../../../einaf/tests/corpus.rs)), 3.6 s on 16 workers

## What it gives the application

the loop reads the kernel through one function over values; a program is loaded once and asked any number of questions on copies (`Ast`, `Terms` and `Kb` are `Clone`)
