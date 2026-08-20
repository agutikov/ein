# S1c.1.5 — In the gate

**Phase:** P1c.1 (stdlib conformance)
**Estimate:** 1 day
**Depends on:** [S1c.1.4](s1c.1.4_stdlib_corpus.md)

## Context

A corpus nobody runs is documentation. This wires it into `cargo test` and
makes the coverage claim self-enforcing.

## Acceptance

- The stdlib corpus runs in `cargo test --workspace`, and its runtime is
  reported. These are small programs; if the suite grows by more than a couple
  of seconds, something in the corpus is bigger than S1c.1.4 intended.
- **A stdlib rule with no activating program fails the gate.** The same
  completeness shape the corpus manifest already uses for `.ein` files, applied
  to rules — which requires the firing census to be a test rather than a
  script.
- **A `.ein` file under the stdlib corpus with no `(test …)` fails the gate**,
  so the directory cannot accumulate programs that check nothing.
- `CLAUDE.md` documents `ein test` and the stdlib corpus in the same breath as
  the other gates.

## Tasks

### Task T1c.1.5.1 — The cargo test
### Task T1c.1.5.2 — The rule-coverage check

The census as an assertion. Needs a list of stdlib rules the check can
enumerate — parse the modules rather than hard-code the list, so adding a rule
without a test fails rather than being invisible.

### Task T1c.1.5.3 — Docs

## Notes

- The coverage check is the part that decays if it is a script. As a test it
  fails the moment someone adds a rule, which is the only moment anyone will
  read it.
