# S1c.1.5 — In the gate

**Phase:** P1c.1 (stdlib conformance)
**Estimate:** 1 day
**Depends on:** [S1c.1.4](s1c.1.4_stdlib_corpus.md)

**Status: shipped 2026-08-24.** The coverage claim is a test, and scoping it to
the suite found the one rule the suite never ran.

| finding | number |
|---|---|
| the gate | **`ein-infer/tests/stdlib_coverage.rs`**, two tests, **0.04 s** — 73 rules, 45 programs, 796 firings |
| what it costs `cargo test --workspace` | nothing measurable: the gate is ~2 min, and 0.04 s of it is this |
| what the 45 entries cost the corpus sweep | **0.72 s** of its 5.08 s — 225 of 889 cells, and nearly all of that is process spawn |
| **rules activated by `tests/stdlib/` alone** | **73 of 73** — up from 72, and with no `examples/` entry contributing |
| the rule that was at 72 | `transitive`, whose fixture is a two-cycle where the `(neq ?a ?c)` guard refuses every match. [`21_transitive.ein`](../../../tests/stdlib/algebra/21_transitive.ein) grew a three-chain |
| negative checks run | **3** — revert the fixture (names `std.algebra/transitive`), append a rule to `stdlib/algebra.ein` (names `std.algebra/brand-new-and-untested`, 1 of **74**), delete an `:expect` (names the file — and the coverage test fails with it, since a query nobody solves activates nothing) |
| goldens moved | **1** — `corpus_shapes.md5`, 39 lines, every one of them `21_transitive.ein`'s |
| new language surface | **0**. New engine code: **0** |

## What the stage decided

**The gate asks the suite, not the corpus.** `utils/stdlib_census.py --check`
sweeps all 180 entries and exits 0 today, and it would go on exiting 0 for a
rule added tomorrow that happened to fire somewhere inside
`examples/zebra.ein` — with **no test written**. That is not a hypothetical: it
is the state [S1c.1.1](s1c.1.1_what_the_stdlib_promises.md) found 20 rules in.
So the test sweeps `tests/stdlib/` and nothing else, and the claim it holds up
is the phase's acceptance in its own words — *adding a rule to the stdlib
without a test fails the gate* — rather than the census's weaker one.

**And that is what found `transitive`.** Its fixture was a two-cycle: A→B, B→A,
where `R(a,b) ∧ R(b,c)` matches twice and `(neq ?a ?c)` refuses both. Zero
firings, deliberately, and the file's header argued for it — *"a rule that is
only ever tested where it fires has had its guard tested nowhere"*. The
argument is right about the guard and silent about the assertion, which was
resting on six puzzles whose hierarchies are acyclic. The file has a third
relation now, `chn`, A→B→C, where the rule fires once and derives A→C. Three
claims in one fixture: what it derives, what the guard refuses, and — through
`(compose cyc cyc cyc)` on the same two edges — that the difference between
them *is* the guard.

**In-process, not through the binary.** The census shells out 557 times and
takes 37 s. A check shaped like that runs when somebody remembers it. This one
loads `stdlib/*.ein` with the engine's own parser for its rule heads, solves
each program the way `ein test` does with an in-memory `Events` sink, and reads
`fire` off the stream — the same observable the script reads, so the two cannot
drift on what a firing *is*.

**The attribution rule is copied, and that is deliberate.** A local declaration
shadows a stdlib name outright; a module the file never imported cannot have
fired; arity splits `std.elim`'s four-parameter `domain-elimination` from
`std.bijection`'s two-parameter one. That is `stdlib_census.py`'s `resolve()`,
re-implemented in Rust, and the two have to stay one rule — which is said in
both places. (On today's suite the arity tiebreak never fires: the two modules
do not import each other, so the closure has already separated them.)

**One acceptance bullet was met by S1c.1.4 and needed only a number.** The 45
programs were registered as corpus entries when they landed, so the sweep
already ran each one's `test` cell against the banked exit code. What this
stage owed it was the measurement: **0.72 s**, 225 cells, against a budget the
bullet set as "a couple of seconds".

## What is deliberately not gated

- **The dual — "every program activates a stdlib rule".** Four fixtures would
  fail it and all four are right: `algebra/08_checks_satisfied.ein` and
  `18_totality_open_world.ein` exist to show a rule loaded, activated and
  *silent*, and the two `macro/` programs test expansion, which `std.macro`
  does with no rules at all. A rule not firing is the case S1c.1.4's notes call
  the one that finds bugs.
- **Sensitivity.** 50 of 51 mutants, taken by hand and recorded in
  [`tests/README.md`](../../../tests/README.md). A gate cannot hold a number
  that needs 51 copies of the stdlib to re-take, and a *stale* mutation score
  is worse than none.
- **Productivity.** The census reports productive vs redundant firings and the
  gate does not read the split. "Every rule fires" and "every rule derives
  something somewhere" are different claims; the second is
  [§11](stdlib_census.md#11-the-re-take--2026-08-24-and-the-zero-set-is-empty)'s
  to report.

## Context

A corpus nobody runs is documentation. This wires it into `cargo test` and
makes the coverage claim self-enforcing.

**What [S1c.1.4](s1c.1.4_stdlib_corpus.md) already handed it, 2026-08-24.** The
45 programs are corpus entries, so the sweep runs each one's `test` cell and
holds its exit code to the banked golden — the first acceptance bullet, met by
registration rather than by new code, and it cost the default sweep nothing
measurable (889 cells, still 5.3 s). The second and third bullets are the
stage's own work and neither exists yet: `utils/stdlib_census.py --check` exits
1 while any rule is at zero and exits **0** today, but it is a script that takes
36 s and shells out to a release binary, which is the shape the Notes below say
decays. The third bullet reads "with no `(test …)`" because it predates
[S1c.1.2](s1c.1.2_test_form.md); the form is `:expect` on a `(query …)`, and
what it asks for is that a file under `tests/` carrying none of them fails —
`ein test` already prints `nothing to check` and exits 2 in that case, so the
check has an implementation to call rather than to write.

## Acceptance

- The stdlib corpus runs in `cargo test --workspace`, and its runtime is
  reported. These are small programs; if the suite grows by more than a couple
  of seconds, something in the corpus is bigger than S1c.1.4 intended.
  **Met** — 225 cells, **0.72 s**, and the sweep as a whole is unmoved at
  5.08 s.
- **A stdlib rule with no activating program fails the gate.** The same
  completeness shape the corpus manifest already uses for `.ein` files, applied
  to rules — which requires the firing census to be a test rather than a
  script. **Met** — `every_stdlib_rule_is_activated_by_a_program`, verified by
  appending a rule to `stdlib/algebra.ein` and watching it name it.
- **A `.ein` file under the stdlib corpus with no `(test …)` fails the gate**,
  so the directory cannot accumulate programs that check nothing. **Met** —
  `every_program_states_an_expectation`, over all of `tests/`, verified by
  deleting one `:expect`.
- `CLAUDE.md` documents `ein test` and the stdlib corpus in the same breath as
  the other gates. **Met** — § Running the gate carries the two commands and
  what each of the two tests fails on.

## Tasks

### Task T1c.1.5.1 — The cargo test

**Done, by measurement rather than by code.** The registration S1c.1.4 did is
what puts the corpus in `cargo test`; what this task owed was the number, and
`corpus_cli`'s 889 cells include 225 of `tests/stdlib/` at 0.72 s.

### Task T1c.1.5.2 — The rule-coverage check

The census as an assertion. Needs a list of stdlib rules the check can
enumerate — parse the modules rather than hard-code the list, so adding a rule
without a test fails rather than being invisible.

**Done** — `ein-infer/tests/stdlib_coverage.rs`. The list is parsed with
`ein_ir::parse`, which is the engine's own reader rather than a scanner: a rule
head a scanner could not read would be a rule the gate silently stopped
requiring a test for. Two shapes needed care and both are in the stdlib — a
parameter list's head is a `Var`, so `head_name` is `None` there, and `()`
parses to the synthetic `@empty` head, which is **zero** parameters rather than
one.

Vacuity is guarded twice: the inventory must be non-empty, and every module
that is not `std.macro` must have parsed to at least one rule — so a parser
change that silently stopped reading rule heads fails here rather than turning
the gate green.

### Task T1c.1.5.3 — Docs

**Done** — `AGENTS.md` § Running the gate (the two commands, what each test
fails on, and why the scope is the suite), the `tests/` and `utils/` bullets,
[`tests/README.md`](../../../tests/README.md) § What holds this directory up,
[`utils/README.md`](../../../utils/README.md)'s census row (which no longer
claims to be the gate), and [`stdlib_census.md` §12](stdlib_census.md#12-the-suite-on-its-own--s1c15).

## Notes

- The coverage check is the part that decays if it is a script. As a test it
  fails the moment someone adds a rule, which is the only moment anyone will
  read it.
- **The negative checks are the stage's real evidence.** A coverage gate that
  has never been seen to fail is a coverage gate nobody has tested, and all
  three of this one's failure modes were exercised against the tree and then
  reverted. What each one prints is in the table at the top.
