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

**Status: shipped 2026-08-24.** 45 programs, and the acceptance number is
**0**.

| finding | number |
|---|---|
| programs | **45**, over seven directories — one per module, `std.macro` included |
| where they live | **`tests/stdlib/`**, not `examples/`. They are a suite, not a set of things to read, and mixing them into the catalogue would have tripled it with files nobody would learn the language from. `tests/` is a third corpus root beside `examples/` and `stdlib/` |
| **rules no corpus run activates** | **38 → 0**, re-measured by the same instrument ([§11](stdlib_census.md#11-the-re-take--2026-08-24-and-the-zero-set-is-empty)) |
| rules that fire but derive nothing | **3 → 0** — `functional`, `injective` and `slot-prune-bwd` are productive for the first time |
| rules whose sole activator is `examples/zebra.ein` | **20 → 0** |
| modules with zero coverage | **2 → 0** |
| lines | 2 628, of which **1 499 are header** — the ratio is the stage's, not an accident: 854 lines of program, and the smallest file is 8 |
| the whole suite under `ein test` | **0.03 s**, 45 expectations |
| corpus cells added | **225** — 45 entries × `test` / `solve` / `saturate` / `render rules` / `render constraints`; 0 existing cells moved |
| local rules the fixtures declare | **13**, ten of them the same four-line `probe-undecided` |
| …and that probe is the stage's one invention | `(open P)` turns "neither asserted nor denied" into a positive fact an expectation *can* close, which is the only way to say a rule did **not** derive a negative — stored negatives are deliberately not closed |
| mutants caught | **50 of 51** — one deliberate defect per stdlib rule family, injected into a copy of `stdlib/` and run past `ein test tests/`. The one miss is named below |
| the census's §5 question, answered | `slot-prune-bwd`'s sterility is **structural**, not a scheduling artefact; verified by exchanging the guard's operands in a copy of the stdlib, which turns `slots/07_spatial_prune.ein` into a contradiction |
| instrument change | one: `utils/stdlib_census.py` sweeps `test` runs too, which is neutral on the corpus as it stood |
| opened | [Q-M1c.7](../open_questions.md#q-m1c7--may-an-expectation-name-a-relation-that-only-saturation-creates) — `:expect` cannot name a relation only saturation creates, so the seven fan-out rules are pinned at one remove |

**The mutation sweep, and the five fixtures it rewrote.** The census's number
is *activation*, and activation is not sensitivity: a program can fire a rule
and still pass with the rule broken. So the suite was measured against 51
deliberate defects — one per rule family, injected into a copy of `stdlib/`
and run past `ein test tests/`: a dropped `neq`, exchanged `absent` operands,
a `forall` over the wrong type, a fan-out short one activator, a copier that
forgets to swap.

The first run caught **44**, and the seven survivors were one finding wearing
seven hats: **where two rules can reach the same verdict, an expectation made
of facts cannot say which one did**, so the fixture has to pick one by
declaring only its activator. `no-room-left` and `domain-elimination` both end
a fully excluded row in ⊥; `slot-no-room` and `slot-elimination` likewise;
`typecheck-arg-0` and `-1` both refute a fact that is ill-typed on either side;
`infer-closure` read `(injective R)` instead of `(functional R) ∧ (total R)`
and still fired, because `bijective-properties` had put all four markers in the
file. Five fixtures were narrowed to one activator each, two were rewritten and one
was added (`slots/04_elimination.ein`, so that `slot-elimination` has a state
`slot-fill` cannot reach), and the second run catches **50 of 51**.

The survivor is named in [`tests/README.md`](../../../tests/README.md) rather
than hidden: `slot-adjacent-bwd-neg` with its structure operands exchanged, on
a three-seat row where the exclusion it should derive is reachable from the
other clue's chain. Isolating it wants a fifth seat, which is larger than the
acceptance allows a fixture to be.

This is the `route` residue
([Q-M1c.2](../open_questions.md#q-m1c2--what-may-an-expectation-say)) arriving
as a measurement rather than as an argument. It does not unpark the question:
separation-by-activation costs five header paragraphs and no language surface,
which is a better trade than a `:fires` keyword.

**What the census predicted and got wrong.** §6 called the fan-out rules the
cheapest bucket — *"a claim about facts, and so exactly what an `:expect` can
say with nothing else in the file"*. It is not: `:expect` validates its
relation names at **load**, and `(domain-elimination R isa)` names a relation
that exists only after saturation. All seven fan-out rules are in that
position. What they get instead is a test through the rules they
activate, which is weaker in one specific way — it cannot catch a *surplus*
activator, because closure is what catches surplus. Q-M1c.7 carries the fix
and the reason it was not made here.

**What it predicted and got right.** §10's cost split. The 38 never-loaded
rules were cheap — 20 of them are covered by five files — and the expensive
item was the module at 100 % coverage: `std.slots` needed four programs and a
position structure that does not exist in any puzzle, and it is where two of
the six days went.


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
  **Met, with the catalogue moved**: they are registered in `corpus.toml`
  (group `stdlib`, 220 cells) and catalogued in
  [`tests/README.md`](../../../tests/README.md), because they are not examples
  — see the banner. `examples/README.md` carries the pointer, and
  `ein-corpus`'s completeness check walks `tests/` as a third root so a file
  with no entry still fails the gate.

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

**Done** — [`stdlib_census.md` §11](stdlib_census.md#11-the-re-take--2026-08-24-and-the-zero-set-is-empty).
180 entries, 557 inference runs, **73 of 73 activated**. The instrument gained
one thing, `test` in its run filter, because a fixture declaring only `test`
would have been invisible to it; that is neutral on the corpus as it stood, so
"comparable by construction" survives.

## Notes

- Write the negative case first where a rule has a guard. It is the case that
  finds bugs, and it is the one that gets skipped when the positive already
  passes.
