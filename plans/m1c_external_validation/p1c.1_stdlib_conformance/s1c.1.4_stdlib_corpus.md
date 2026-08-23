# S1c.1.4 — The stdlib corpus

**Phase:** P1c.1 (stdlib conformance)
**Estimate:** ~~4 days~~ **6 days** — re-estimated 2026-08-23 against
[S1c.1.1](s1c.1.1_what_the_stdlib_promises.md)'s census, which the stage doc
asked to decide this ("if the zero-firing set is large, say so before
committing to four days"). It is large — **38 of 73 rules never fire** — but
that is not where the two extra days go: the zero set is cheap, and the
expensive item is `std.slots`, at 100 % rule coverage and *one* activating
file. [`stdlib_census.md` §10](stdlib_census.md#10-what-this-does-to-s1c14).
**Depends on:** [S1c.1.3](s1c.1.3_test_subcommand.md)

## Context

The programs themselves — one per rule or tight family, each the smallest
thing that activates it, each stating what it should and should not derive.
[S1c.1.1](s1c.1.1_what_the_stdlib_promises.md)'s table is the work list, its
zero-firing set is the priority, and
[`stdlib_census.md` §6](stdlib_census.md#6-what-would-activate-a-rule-that-nothing-activates--t1c113)
already carries the smallest activating program for each of the 38.

## Acceptance

- **Every stdlib rule is activated by at least one program**, re-measured by
  the same firing census S1c.1.1 used. The census, not a reading of the
  directory, is what closes this.
- Every program is **small enough to read** — the fixture that came out of this
  session's bug, [`features/09_adjacent_via_same_house.ein`](../../../examples/features/09_adjacent_via_same_house.ein),
  is three houses and two attribute kinds, and that is the right size. A
  stdlib test that needs a zebra puzzle to activate its rule is testing the
  zebra puzzle.
- Each carries **both directions** where the rule has a guard: what it derives,
  and what it must not derive when the guard says no.
- Each carries a **header** in the corpus's style: what the rule promises,
  what this program does to it, and what the expected result *means*.
  `examples/features/` is the model.
- Registered in `corpus/corpus.toml` (or its successor from
  [S1a.10.3](../../../docs/history/m1a_rust/README.md#s1a103--the-corpus-without-a-second-engine))
  and catalogued in the examples README, like every other fixture.

## Tasks

### Task T1c.1.4.1 — `std.algebra` — `symmetric`, `transitive`, `includes`, the cardinality properties
### Task T1c.1.4.2 — `std.bijection` — the setup glue, the negatives, the eliminations, the typechecks

The biggest module and the one whose rules chain: `bijective-setup` fans out
into activators, the negatives feed the eliminations' `forall`, and the
priorities (240 negatives, 250 checks, 400 eliminations) are what make the
order work. A test per rule *and* at least one that pins the chain, because
the priorities are a promise nothing currently states.

### Task T1c.1.4.3 — `std.elim` — `domain-elimination`, `no-room-left`

The two this session's bug ran through. `domain-elimination` asserting a
positive from accumulated negatives is the mechanism that turned a mid-layer
writeback into a refutation, and the fact that nothing tested it directly is
why the guard survived.

### Task T1c.1.4.4 — `std.closure`, `std.slots`, `std.typing`
### Task T1c.1.4.5 — `std.macro` — `forall`, `open`

Macro expansion, so the expectations are about the *expanded* program. Whether
`(test …)` sees pre- or post-expansion state is a decision
[S1c.1.2](s1c.1.2_test_form.md) has to have made.

### Task T1c.1.4.7 — A second `std.slots` program — **added 2026-08-23**

Not in the original task list, and the census's main finding for this stage:
all eighteen `std.slots` rules are activated by `examples/zebra.ein` and by
nothing else, so the module's whole test is one puzzle's accident. The program
wants a slot structure whose spatial relation is **asymmetric and not
functional on positions** — which is also the only state that separates
`slot-prune-fwd` from `slot-prune-bwd`, the pair the corpus currently cannot
tell apart ([§5](stdlib_census.md#5-fires-derives-nothing--3-rules)).

### Task T1c.1.4.6 — The census, re-run

The acceptance number, and it is
[`utils/stdlib_census.py`](../../../utils/stdlib_census.py) — the same
instrument S1c.1.1 used, so the before and after are comparable by
construction. Report per module: rules, covered, still zero.

## Notes

- Write the negative case first where a rule has a guard. It is the case that
  finds bugs, and it is the one that gets skipped when the positive already
  passes.
