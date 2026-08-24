# The stdlib census — what 73 rules promise, and what 400 corpus runs activate

**Stage:** [S1c.1.1](s1c.1.1_what_the_stdlib_promises.md) — T1c.1.1.1 … T1c.1.1.4
**Taken:** 2026-08-23, commit `114835d`, `ein 0.1.0` (`ein-events/1`, stdlib
`sha256:a498c762…`)
**Instrument:** [`utils/stdlib_census.py`](../../../utils/stdlib_census.py) — 128
corpus entries, **400 inference runs**, `--events-level verbose`, 35.6 s wall
**Re-take:** `python3 utils/stdlib_census.py --json census.json`

> **Since the take, twice.** The tables below are the 2026-08-23 `114835d`
> take and are kept as the *before* column;
> [§11](#11-the-re-take--2026-08-24-and-the-zero-set-is-empty) is the re-take
> after [S1c.1.4](s1c.1.4_stdlib_corpus.md), where the zero-firing set is
> **0**. What follows in this callout is the smaller of the two moves.
>
> [S1c.1.2](s1c.1.2_test_form.md) added five fixtures to
> the corpus on the same day. Re-taken against them, exactly one cell moves:
> `std.algebra`'s `symmetric` goes from **7 entries to 8** and 1 084 → 1 092
> productive firings, because
> [`examples/features/10_expect.ein`](../../../examples/features/10_expect.ein)
> imports it. **The zero-firing set is unchanged at 38** and every module's
> covered/zero pair is identical, so nothing below is stale. The numbers in the
> tables are the 2026-08-23 `114835d` take, and the command above is what
> re-takes them.

| finding | number |
|---|---|
| stdlib rules declared | **73**, over six modules (`std.macro` declares two macros and no rule) |
| rules **no corpus run activates** | **38** — 52 % |
| of those, rules **no corpus entry even loads** | **33**; the other 5 are loaded and never satisfied |
| rules activated by **exactly one entry** | **23** |
| entries those 23 depend on | **three** — `examples/zebra.ein` carries **20**, `ein-bugs/zebra2-bad.ein` 2, `features/05_stdlib_domain_elim.ein` 1 |
| rules either untested or held up by one file | **61 of 73 — 84 %** |
| rules that **refute** (`:assert (false)`) | **18**, of which **12** never fire |
| rules that fire but **derive nothing** in any run | **3** — `functional`, `injective`, `slot-prune-bwd` |
| modules with **zero** coverage | **two** — `std.typing` (4 rules), `std.closure` (1) |
| modules with **full** coverage | **one** — `std.slots` (18 rules), every one of them by `examples/zebra.ein` alone |
| rules that read as zero at `--events-level normal` but fire at `verbose` | **3** — the elision trap, measured: 41 zeroes against 38 |
| declarations that are another declaration under a second name | **4 pairs** |
| rules whose promise needs more than one sentence | **6** |

**The one-line reading.** Fifty-two per cent of the standard library has never
been run, and of the half that has, most of it is held up by a single file. The
phase's premise was that the stdlib is *"exercised only as a side effect of
whatever the zebra corpus happens to need"* — that is not a characterisation,
it is the measurement: delete `examples/zebra.ein` and coverage falls from 35
rules to **15**.

---

## 1. The instrument, and the three ways it was wrong first

The census is two halves. The first parses `stdlib/*.ein` for `(rule …)` heads
— module, parameters, priority, whether the assert is `(false)`, and every
guard shape in the `:match`. The second runs every corpus entry under every
declared `solve` / `saturate` invocation with `--events`, and counts `fire` by
rule.

Three things stand between a `fire` event and a coverage number, and **the
first run of this instrument got two of them wrong**. They are recorded because
a coverage number nobody can reproduce the derivation of is a slogan.

**A fired name is not a stdlib rule.** Twenty-five files under `examples/`
declare their own `symmetric`, `transitive`, `functional`, `injective`,
`total`, `surjective` or `domain-elimination` — the inline copies
[`stdlib/README.md`](../../../stdlib/README.md) says are kept deliberately
(a variant `:why`, or a showcase demo that wants the rule visible in the file),
plus `zebra2-hints.ein`, which *imports* three `std.algebra` symbols and
declares five more of its own names alongside them. Crediting those firings to
the stdlib is how a census reports coverage nobody has. Before the filter,
`symmetric` read 112 271 productive firings over 22 entries; after it, **1 084
over 7**. `transitive` read 35 835 over 18; after, **1 868 over 6**. So a rule
counts for a module only when the file does not declare that name itself *and*
the module is in its import closure.

**`domain-elimination` is two rules and `typecheck-arg-0` is two rules.**
`std.elim` ships the positional formulation, `std.bijection` the
signature-driven one, and the corpus runs both. A `fire` event carries its
rule's `activator` — the parameter tuple — so the two `domain-elimination`s
separate on arity (4 against 2). The two `typecheck-arg-0`s do not: both take
`(?R ?isa ?Dom)`. They separate on the import closure — and the first run,
which had neither filter, left **291 `domain-elimination` firings attributed to
both modules at once**, most of them belonging to a third rule of that name
that `examples/domain_elim/ab.ein` declares for itself. With the filters in
place the run reports zero ambiguous attributions, and the instrument prints
the count rather than hiding it, because silent double-counting is how the
first version read 129 firings for a rule that fires twice.

**`normal` hides three rules** — the one the stage doc warned about, so it was
never taken, only measured. [events.md § Levels](../../../docs/kernel/inference/events.md)
says a redundant firing is counted but not emitted at `normal`, so a rule whose
every firing re-derives an existing fact reads as zero — the trap
[S1a.7.0](../../../docs/history/m1a_rust/README.md#s1a70--the-speculation-audit)'s
audit hit. It is real here and now has a number:

| level | rules reading zero | the difference |
|---|---:|---|
| `--events-level normal` | **41** | — |
| `--events-level verbose` | **38** | `functional`, `injective`, `slot-prune-bwd` |

Twenty-five rules report a different pair of counts at the two levels. The
census runs at `verbose` by default and `--level normal` exists to show this
row, not to be used.

---

## 2. The table

**Promise** is hand-written per T1c.1.1.1, one sentence each, checked against
the rule *body* rather than its `:why` — the `:why` is a message and several
of them describe the intent rather than the match. **⊥** marks a rule whose
`:assert` is `(false)`: it does not derive, it refutes. **p** / **r** are
productive and redundant firings summed over all 400 runs; **n** is how many
corpus entries activate it.

### `std.algebra` — 38 rules, 10 activated

| rule | pri | ⊥ | promise | guards | p | r | n |
|---|---:|:-:|---|---|---:|---:|---:|
| `converse` | 100 | | mirror every `R1` edge into `R2` with the endpoints swapped | — | 0 | 0 | 0 |
| `imply1` | 100 | | copy every 1-arg `R1` fact into `R2` — property → property | — | 0 | 0 | 0 |
| `imply2-fwd` | 100 | | copy every 2-arg `R1` edge into `R2`, orientation kept | — | 0 | 0 | 0 |
| `imply2-reverse` | 100 | | copy every `R1` edge into `R2` reversed — `converse` under a second name | — | 0 | 0 | 0 |
| `symmetric-is-self-converse` | 90 | | a `(symmetric R)` tag derives `(converse R R)` | — | 0 | 0 | 0 |
| `self-converse-is-symmetric` | 90 | | `(converse R R)` derives the `(symmetric R)` tag — the converse of the above | — | 0 | 0 | 0 |
| `converse-pair-symmetric` | 90 | | `(converse R1 R2)` derives `(converse R2 R1)` | — | 0 | 0 | 0 |
| `converse-illtyped-dom` | 110 | ⊥ | a converse pair whose `range(R1)` is neither `domain(R2)` nor a `?isR*`-subtype of it | absent×1 neq×1 | 0 | 0 | 0 |
| `converse-illtyped-ran` | 110 | ⊥ | the same on the other side — `domain(R1)` against `range(R2)` | absent×1 neq×1 | 0 | 0 | 0 |
| `compose` | 100 | | relative product: an `R1` edge into `?b` and an `R2` edge out of it derive the `R3` edge across | — | 0 | 0 | 0 |
| `identity` | 100 | | self-loop every `Dom` member — **extensive**, ranges over the `?isa` extent | — | 0 | 0 | 0 |
| `meet` | 100 | | a pair present in both operands lands in `R3` | — | 0 | 0 | 0 |
| `difference` | 100 | | a pair in `R1` and absent from `R2` lands in `R3` — closed-world | absent×1 | 0 | 0 | 0 |
| `derive-join` | 120 | | fan a `(join R1 R2 R3)` operand fact into the two per-operand copier activators | — | 0 | 0 | 0 |
| `join-l` | 100 | | copy `R1`'s edges into `R3` | — | 0 | 0 | 0 |
| `join-r` | 100 | | copy `R2`'s edges into `R3` | — | 0 | 0 | 0 |
| `empty` | 110 | ⊥ | any edge at all in a relation declared empty | — | 0 | 0 | 0 |
| `top` | 100 | | materialise the whole `Dom×Ran` rectangle — **extensive** | — | 0 | 0 | 0 |
| `complement` | 100 | | materialise every `Dom×Ran` pair *absent* from `R1` — **extensive**, and the op that most needs `R1` saturation-determined | absent×1 | 0 | 0 | 0 |
| `functional` | 250 | ⊥ | one `a` with two distinct images | neq×1 | 0 | 2000 | 1 |
| `injective` | 250 | ⊥ | one `b` reached from two distinct sources | neq×1 | 0 | 2000 | 1 |
| `bijective-properties` | 100 | | fan one `(bijective R)` into the four cardinality markers | — | 100 | 0 | 3 |
| `total` | 110 | ⊥ | some `a∈A` with *every* `b∈B` explicitly excluded | forall×1 not×1 | 79 | 96 | 3 |
| `surjective` | 110 | ⊥ | some `b∈B` with *every* `a∈A` explicitly excluded | forall×1 not×1 | 31 | 100 | 2 |
| `irreflexive` | 110 | ⊥ | a self-loop | — | 0 | 0 | 0 |
| `antisymmetric` | 110 | ⊥ | a mutual pair between *distinct* elements | neq×1 | 0 | 0 | 0 |
| `asymmetric` | 110 | ⊥ | any mutual pair, self-loops included | — | 0 | 0 | 0 |
| `connex` | 110 | ⊥ | two distinct `Dom` members with neither orientation present — **extensive** | absent×2 neq×1 | 0 | 0 | 0 |
| `difunctional` | 90 | | rows overlapping in a column agree on every column: `R(a,b)∧R(c,b)∧R(c,d) ⟹ R(a,d)` | — | 0 | 0 | 0 |
| `symmetric` | 100 | | mirror every edge of the tagged relation | — | 1084 | 1084 | 7 |
| `symmetric-negative-setup` | 100 | | lift a `(symmetric R)` tag to the `(symmetric-negative R)` activator, because a variable relation head inside `(not …)` is only scannable when it is a parameter | — | 8 | 0 | 1 |
| `symmetric-negative` | 100 | | mirror every *stored negative* of the tagged relation | not×1 | 4651 | 4651 | 1 |
| `transitive` | 200 | | transitive closure, `neq a c` keeping an irreflexive relation irreflexive | neq×1 | 1868 | 840 | 6 |
| `includes` | 100 | | lift every `p` edge into `q` — `imply2-fwd` under a second name | — | 1208 | 0 | 7 |
| `compose-negative-s` | 240 | | Schröder: a missing composite plus an `R` edge forces a missing `S` edge | not×1 | 0 | 0 | 0 |
| `compose-negative-r` | 240 | | Schröder, the other factor | not×1 | 0 | 0 | 0 |
| `compose-contravariant` | 90 | | `(A;B)° = B°;A°` — derive the converse composite from a declared one plus three converse facts | — | 0 | 0 | 0 |
| `join-converse` | 90 | | `(A∪B)° = A°∪B°`, same shape | — | 0 | 0 | 0 |

### `std.bijection` — 8 rules, 6 activated

| rule | pri | ⊥ | promise | guards | p | r | n |
|---|---:|:-:|---|---|---:|---:|---:|
| `bijective-setup` | 100 | | fan `(bijective R)` + `(bijection-hierarchy isa)` into six activators — totality, both eliminations, both negative completions | — | 100 | 0 | 3 |
| `typecheck-setup` | 100 | | fan a bijective relation's `(relation R A B)` signature + `(typecheck-hierarchy isa)` into the two arg-typecheck activators | — | 100 | 0 | 3 |
| `functional-negative` | 240 | | a positive `(R a b)` excludes every *other* `B`-member for `a` | neq×1 | 1644 | 10614 | 3 |
| `injective-negative` | 240 | | the dual — excludes every other `A`-member for `b` | neq×1 | 2932 | 9232 | 3 |
| `domain-elimination` | 400 | | functional ∧ total, every `B` but one excluded for `a` ⟹ force it | forall×1 neq×1 not×1 | 1008 | 1749 | 3 |
| `range-elimination` | 400 | | injective ∧ surjective, every `A` but one excluded for `b` ⟹ force it | forall×1 neq×1 not×1 | 203 | 2533 | 3 |
| `typecheck-arg-0` | 220 | ⊥ | a fact whose arg 0 is not an `?isa`-member of the declared domain | absent×1 | 0 | 0 | 0 |
| `typecheck-arg-1` | 220 | ⊥ | the same for arg 1 against the range | absent×1 | 0 | 0 | 0 |

### `std.elim` — 4 rules, 1 activated

The positional formulation: properties declared `(functional R 0 1)` /
`(total R 0)`, types passed as parameters rather than read off a signature.

| rule | pri | ⊥ | promise | guards | p | r | n |
|---|---:|:-:|---|---|---:|---:|---:|
| `domain-elimination` | 400 | | every `VT` but one excluded for `a` ⟹ force `(R a v)` | forall×1 neq×1 not×1 | 2 | 0 | 1 |
| `no-room-left` | 110 | ⊥ | every `VT` excluded for some `a` — nothing left to force | forall×1 not×1 | 0 | 0 | 0 |
| `typecheck-arg-0` | 110 | ⊥ | arg 0 is not an `?isa`-`Dom` | absent×1 | 0 | 0 | 0 |
| `typecheck-arg-1` | 110 | ⊥ | arg 1 is not an `?isa`-`Ran` | absent×1 | 0 | 0 | 0 |

### `std.closure` — 1 rule, 0 activated

| rule | pri | ⊥ | promise | guards | p | r | n |
|---|---:|:-:|---|---|---:|---:|---:|
| `infer-closure` | 90 | | functional ∧ total ⟹ `(__closed__ R)`: hypgen must never speculate an `R`-fact | — | 0 | 0 | 0 |

### `std.slots` — 18 rules, 18 activated, all by one file

| rule | pri | ⊥ | promise | guards | p | r | n |
|---|---:|:-:|---|---|---:|---:|---:|
| `slot-partition-setup` | 100 | | fan `(slot-partition …)` into the eight partition activators | — | 4 | 0 | 1 |
| `slot-spatial-setup` | 100 | | fan `(slot-spatial …)` into the eight directional ones | — | 8 | 0 | 1 |
| `slot-locate` | 200 | | index-anchored transitivity: `a` shares a slot with `b`, `b` is at `i` ⟹ `a` is at `i` | neq×1 | 329 | 312 | 1 |
| `slot-exclusive` | 240 | | two distinct members of one slot type can never share a slot | neq×1 | 240 | 240 | 1 |
| `slot-occupied` | 240 | | a slot's seat for `b`'s type is taken, so no other member of that type is in it | neq×1 | 1289 | 5945 | 1 |
| `slot-negative` | 240 | | the contrapositive of transitivity: a slot `b` cannot be in, `a` cannot be in either | not×1 | 515 | 9247 | 1 |
| `slot-elimination` | 400 | | every slot but `i` excluded for the value `a` ⟹ `a` is at `i` | forall×1 neq×2 not×1 | 271 | 442 | 1 |
| `slot-fill` | 400 | | every member of a type but `v` excluded for slot `i` ⟹ `v` is at `i` | forall×1 neq×2 not×1 | 31 | 643 | 1 |
| `slot-no-room` | 250 | ⊥ | a value with every slot excluded | forall×1 neq×1 not×1 | 46 | 0 | 1 |
| `slot-no-fill` | 250 | ⊥ | a slot with every member of some type excluded | forall×1 neq×1 not×1 | 5 | 0 | 1 |
| `slot-adjacent-fwd` | 200 | | `b` is at `p1`, whose *only* `S`-source is `p2` ⟹ `a` is at `p2` | absent×1 neq×1 | 54 | 117 | 1 |
| `slot-adjacent-bwd` | 200 | | `a` is at `p2`, whose *only* `S`-target is `p1` ⟹ `b` is at `p1` | absent×1 neq×1 | 50 | 118 | 1 |
| `slot-adjacent-fwd-neg` | 240 | | the contrapositive under the same uniqueness witness | absent×1 neq×1 not×1 | 64 | 410 | 1 |
| `slot-adjacent-bwd-neg` | 240 | | the backward contrapositive | absent×1 neq×1 not×1 | 11 | 463 | 1 |
| `slot-prune-fwd` | 250 | | `b` is at `p1`, so `a` is at some `S`-source of `p1` — exclude every position that is not one | absent×1 neq×1 | 9 | 597 | 1 |
| `slot-prune-bwd` | 250 | | the `S`-target reading, *not* the mirror when `S` is asymmetric | absent×1 neq×1 | 0 | 606 | 1 |
| `slot-endpoint-fwd` | 240 | | a position with no `S`-source cannot hold `b` — fires at d=0, no positive needed | absent×1 | 4 | 16 | 1 |
| `slot-endpoint-bwd` | 240 | | a position with no `S`-target cannot hold `a` | absent×1 | 4 | 16 | 1 |

### `std.typing` — 4 rules, 0 activated

| rule | pri | ⊥ | promise | guards | p | r | n |
|---|---:|:-:|---|---|---:|---:|---:|
| `type-hierarchy-converse` | 120 | | one `(type-hierarchy isR*)` knob derives both converse-typecheck activators for every declared converse pair | — | 0 | 0 | 0 |
| `derive-reflexive` | 120 | | fan `(reflexive R)` into the two per-position closers | — | 0 | 0 | 0 |
| `reflexive-dom` | 110 | | self-loop every element `R` touches in arg 0 | — | 0 | 0 | 0 |
| `reflexive-cod` | 110 | | self-loop every element `R` touches in arg 1 | — | 0 | 0 | 0 |

### `std.macro` — 2 macros, no rules

Macros expand at load and emit no `fire` event, so they are not in the census's
rule count. Their coverage is inherited from the rules that use them: `forall`
appears in **10** stdlib rule bodies, **9** of which fire somewhere in the
corpus (the tenth is `std.elim`'s `no-room-left`), so the expansion is
exercised. `open` appears in **no stdlib rule at all** — one file in the whole
repository uses it, [`examples/features/04_open.ein`](../../../examples/features/04_open.ein),
which the corpus marks as having no finite hypothesis space and runs under
`saturate` only.

---

## 3. The zero-firing set — 38 rules, two kinds

The kinds are different findings and want different fixtures.

### 3a. Never loaded — 33 rules

No corpus entry imports them, so nothing about them has ever been executed:
not the match, not the guard, not the assert, not the priority.

| module | rules |
|---|---|
| `std.algebra` (28) | `converse` `imply1` `imply2-fwd` `imply2-reverse` `symmetric-is-self-converse` `self-converse-is-symmetric` `converse-pair-symmetric` `converse-illtyped-dom` `converse-illtyped-ran` `compose` `identity` `meet` `difference` `derive-join` `join-l` `join-r` `empty` `top` `complement` `irreflexive` `antisymmetric` `asymmetric` `connex` `difunctional` `compose-negative-s` `compose-negative-r` `compose-contravariant` `join-converse` |
| `std.typing` (4) | `type-hierarchy-converse` `derive-reflexive` `reflexive-dom` `reflexive-cod` |
| `std.closure` (1) | `infer-closure` — and its only importer, `examples/broken/load/import_conflicting_definitions.ein`, is a **load-negative** fixture that never reaches the engine |

That is the *whole relative and Boolean layer* of the relation algebra —
composition, meet, join, complement, top, identity, difference — plus every
equational lemma, plus the entire converse-typecheck path in both its raw
(`std.algebra`) and knob-driven (`std.typing`) forms.

### 3b. Loaded and never satisfied — 5 rules

These are compiled, activated and waiting; the state that would fire them
never arises. **Every one asserts `(false)`.**

| rule | module | loaded by | why it never fires |
|---|---|---|---|
| `typecheck-arg-0` / `-1` | `std.bijection` | 4 entries | the zebra2 family is well-typed, and nothing in the corpus states an ill-typed fact under a bijective relation |
| `typecheck-arg-0` / `-1` | `std.elim` | 1 entry | same, for `features/05_stdlib_domain_elim.ein` |
| `no-room-left` | `std.elim` | 1 entry | `features/05` is satisfiable and never exhausts a domain |

A refutation rule that never fires in a corpus of solvable puzzles is
expected. That it has *also* never fired in a corpus of **broken** ones is the
finding: `examples/broken/` is **37** entries of parse and load errors and not one
of a program that loads, runs, and is ill-typed.

---

## 4. The low-firing set — 23 rules, three files

| entry | rules it is the sole activator of | which |
|---|---:|---|
| `examples/zebra.ein` | **20** | all 18 of `std.slots`, plus `std.algebra`'s `symmetric-negative-setup` and `symmetric-negative` |
| `examples/ein-bugs/zebra2-bad.ein` | **2** | `std.algebra`'s `functional` and `injective` — and see §5 |
| `examples/features/05_stdlib_domain_elim.ein` | **1** | `std.elim`'s `domain-elimination` |

This is the phase's premise restated as a dependency: **`std.slots` has 100 %
rule coverage and one test.** Drop `examples/zebra.ein` from the corpus and
the covered set falls from **35 rules to 15** — a third of the standard
library's tested surface is one file. If that file changed its encoding —
which is exactly what a second Zebra formulation is *for*, and
[`examples/zebra2.ein`](../../../examples/zebra2.ein) is the other one, using
`std.bijection` instead — eighteen rules would become untested silently, and
the census is the only thing in the repository that would say so.

The other two rows are worse in a different way. `std.algebra`'s two
cardinality checks are activated **only by a fixture whose name says it is
broken**, and `std.elim`'s `domain-elimination` fires **twice** in the entire
corpus — the positional twin of the rule
[whose guard was wrong for a year](../README.md#the-thesis).

---

## 5. Fires, derives nothing — 3 rules

A rule with productive firings 0 and redundant firings > 0 ran, matched, passed
its guards, and every fact it asserted was already there.

| rule | module | p | r | loaded by | fires in | reading |
|---|---|---:|---:|---:|---|---|
| `functional` | `std.algebra` | 0 | 2000 | 3 puzzles | `ein-bugs/zebra2-bad.ein` | never the first to refute |
| `injective` | `std.algebra` | 0 | 2000 | 3 puzzles | `ein-bugs/zebra2-bad.ein` | never the first to refute |
| `slot-prune-bwd` | `std.slots` | 0 | 606 | 1 | `examples/zebra.ein` | its forward twin gets there first, 9 times |

**Who actually kills a branch**, productive `(false)` firings per run:

| file | who refutes | count |
|---|---|---:|
| `zebra2.ein -e` | `total`, `surjective` | 32 + 10 |
| `zebra.ein -e` | `slot-no-room`, `slot-no-fill` | 43 + 5 |
| `zebra2-bad.ein` | `total` | **1**, then `surjective` ×25, `functional` ×500 and `injective` ×500 redundantly |

So the two cardinality *checks* of `std.algebra` — the rules that say "two
distinct images of one `a` is ⊥" — have never in this corpus produced a
contradiction that something else had not already produced, and outside one
deliberately-broken fixture they are never activated at all. Their `(neq ?b ?c)`
guard is the same shape as `disjunctive-prune`'s, and nothing would notice if
it were wrong.

They are not merely absent, either. `zebra2.ein` and `zebra2-minus-15.ein`
both **load and activate** them — auto-closure pulls the two rules in behind
`bijective-properties`, which fires 100 times deriving the markers — and in
neither does the violating state ever arise. In `zebra2-bad.ein` it arises 500
times each, and by then `total` at priority 110 has already put `(false)` in
the branch.

So the check is **dominated twice over**: by `functional-negative` at 240,
which excludes every other partner the moment a positive lands, and by the
totality checks at 110, which reach `(false)` first when a contradiction does
occur. That is a finding about the *rule set*, not about the corpus — and
S1c.1.4 should record it rather than paper over it with a fixture that pins
`(functional R)` and no negatives just to make the rule fire. The fixture worth
writing is the one that says what the corpus cannot: that the guard rejects
`b = c`.

`slot-prune-bwd` is a milder case, and one the census can only half explain.
`zebra.ein` declares two spatial relations — `(slot-spatial co-located right-of
instance House)` and the same for `next-to` — and in 606 firings the backward
reading never excluded a position the forward one had not already excluded.
Whether that is structural (both are functional on a row of five houses, and
`next-to` is symmetric, so the two guards coincide) or an artefact of the
scheduling at equal priority 250, this corpus cannot say: separating them needs
an `S` that is asymmetric *and* non-functional on positions, and there is no
such puzzle. That fixture is S1c.1.4's, and it is the one place in `std.slots`
where a test would be finding something rather than recording it.

---

## 6. What would activate a rule that nothing activates — T1c.1.1.3

Per zero-firing rule, the smallest thing that fires it. The 38 partition into
seven buckets, and none of them needs a puzzle:

| bucket | rules | n |
|---|---|---:|
| **copiers and products** — an activator fact plus two edges | `converse` `imply1` `imply2-fwd` `imply2-reverse` `compose` `meet` `difference` `join-l` `join-r` `difunctional` | 10 |
| **tag lemmas and fan-outs** — one operator fact, no edges at all | `symmetric-is-self-converse` `self-converse-is-symmetric` `converse-pair-symmetric` `derive-join` `compose-contravariant` `join-converse` `type-hierarchy-converse` `derive-reflexive` | 8 |
| **extensive ops** — need a type extent, and each carries the closed-world caveat | `identity` `top` `complement` `connex` (also ⊥) | 4 |
| **the reflexive closers** — activator plus one edge | `reflexive-dom` `reflexive-cod` | 2 |
| **stored-negative premises** — need an authored or derived `(not …)` | `compose-negative-s` `compose-negative-r` | 2 |
| **refutation, needs a deliberate violation** | `empty` `irreflexive` `antisymmetric` `asymmetric` `converse-illtyped-dom` `converse-illtyped-ran` `typecheck-arg-{0,1}` ×2 modules, `no-room-left` | 11 |
| **the caveat, exhibited** | `infer-closure` | 1 |

The first bucket is three lines each:

```lisp
(import std.algebra :symbols (compose))
(compose right-of right-of two-right)          ; the activator
(right-of House-2 House-1) (right-of House-3 House-2)
;; ⟹ (two-right House-3 House-1)
```

The second is cheaper still — a fan-out rule's whole promise is "this
declaration produced those activators", which is a claim about *facts* and so
is exactly what an `:expect` can say with nothing else in the file.

The third and the sixth are where the work is. The extensive ops materialise
pairs that are absent from the edge set, so each fixture is also the
demonstration of *why* they are opt-in; and the eleven refutation rules want
the **negative** direction written first (§7), because "fires and derives ⊥"
and "does not fire on the legal case" are two different fixtures and only the
second finds a guard bug. `infer-closure`'s fixture is the one place the corpus
would document a soundness caveat by exhibiting it: a puzzle that needs
branching, closed by import, failing to solve.

**No rule in the zero set is unreachable.** The stage doc anticipated finding
some — *"this rule is unreachable given the others' priorities, which is a
finding"* — and there are none. Every one of the 38 has a state that fires it;
what is missing is a file that reaches that state.

## 7. The negative shapes — T1c.1.1.4

Every guard is a case where firing is wrong. Counted over the 73 declarations:

| guard | occurrences | rules carrying it | what a negative test pins |
|---|---:|---:|---|
| `neq` | 27 | 25 | the *identical* case must not fire — the `disjunctive-prune` shape |
| `absent` (direct, not via `forall`) | 18 | 17 | firing when the fact is present is wrong; and NAF is re-checked at fire time, so "the guard passed at match and failed at fire" is a second case |
| `not` (a stored negative as a *premise*) | 16 | 16 | the rule must not read an *absent* fact as a negative one — the third state `open` names |
| `forall` | 10 | 10 | the quantifier must not fire on a *partially* excluded domain — the open-world-safety claim `total` / `surjective` / `slot-no-*` all make in their comments and nothing checks |

Counted from the parsed `:match` bodies, so the `(not …)` that **14** rules
*assert* — the whole negative-completion band — is not a guard here. Nine of
those fourteen are `std.slots`'.

**33 rules carry no guard at all** and so have no negative case in this sense:
the pure copiers (`converse`, `imply*`, `includes`, `symmetric`, `join-*`,
`meet`, `compose`, `identity`, `top`), the setup fan-outs, the equational
lemmas, and the four `std.typing` rules. For those the only "must not" is
*scope* — a copier activated for `R1→R2` must not touch `R3` — which is a
relation-closure claim and therefore exactly what
[S1c.1.2](s1c.1.2_test_form.md)'s rule 3 makes checkable, and *only* rule 3:
neither a per-fact assertion nor a whole-state golden states it.

**The `forall` row is the one to write first.** Ten rules quantify, every
one of them documents itself as open-world-safe — "fires only when the stored
negatives cover the whole extent, never on a merely-undecided state" — and that
claim is currently checked by nothing at all. It is also the claim whose failure
mode is silent: a `forall` that fires one negative early yields a *wrong model*,
not a crash.

---

## 8. Four declarations are two rules

Alpha-renaming each `(params, :match, :assert)` and grouping:

| the same body | difference |
|---|---|
| `std.algebra/converse` ≡ `std.algebra/imply2-reverse` | none but the name and the `:why`; the module calls the second an "ergonomic alias" |
| `std.algebra/imply2-fwd` ≡ `std.algebra/includes` | none but the name and the `:why`; the module calls them twins |
| `std.bijection/typecheck-arg-0` ≡ `std.elim/typecheck-arg-0` | **priority 220 against 110** |
| `std.bijection/typecheck-arg-1` ≡ `std.elim/typecheck-arg-1` | **priority 220 against 110** |

The first two say a fact-shaped expectation cannot distinguish the members of a
pair: one test covers both, and *which one fired* is the `route` residue
[Q-M1c.2](../open_questions.md#q-m1c2--what-may-an-expectation-say) parks. Fine
— that is what parking it means.

The last two are not an alias, they are a **divergence**. Two modules ship the
same rule under the same name with the same body and different priorities;
`std.bijection`'s 220 is documented ("so the NAF evaluates after the puzzle's
transitive closure at 200 has saturated") and `std.elim`'s 110 is not. A puzzle
cannot import both — a same-name differing body is a load conflict — so nothing
has ever had to reconcile them, and neither is tested. Whether 110 is a bug is
outside this stage; that the question exists is the census's to report.

---

## 9. Promises that do not fit in one sentence — 6

The stage asks for these to be flagged as findings about the rule.

1. **`symmetric-negative` + `symmetric-negative-setup`** — two rules because
   the matcher can scan a variable relation head inside `(not …)` only when the
   head is an activator parameter. The promise is one sentence; the *shape* is
   a workaround, and a reader cannot tell from the pair which part is the
   semantics.
2. **`slot-elimination` / `slot-fill`** — "not each other's mirror": `R`'s
   symmetry makes the conclusions interchangeable but the two quantify over
   different domains, so they consume different negatives and fire at different
   times. The Zebra opening needs one and the endgame the other. Any one-line
   promise loses the reason both exist.
3. **`slot-prune-fwd` / `-bwd`** — the difference is the operand order inside
   the `absent` guard, and it only matters when `S` is asymmetric. §5 shows the
   corpus never separates them.
4. **`total` / `surjective`** — the promise is not "R is total" but "R is
   *provably not* total **in the closed-world reading**, because the stored
   negatives cover the extent". The distinction is the whole open-world-safety
   argument and it is in a comment.
5. **`infer-closure`** — the promise has a caveat larger than itself: functional
   ∧ total is an *operational* witness, sound only when R needs no branching, and
   importing it into the zebra family breaks the search. A one-sentence promise
   here is actively misleading.
6. **`complement` / `difference` / `connex`** — each is sound only when its
   operand is saturation-determined. Same shape as 5: the sentence is easy, the
   condition under which it is true is not.

Every one of these is a case where [S1c.1.4](s1c.1.4_stdlib_corpus.md)'s
"header saying what the rule promises" is doing real work, and where the
expectation that matters is the **consequence at a distance** the phase README
demands rather than a restatement of the assert.

---

## 10. What this does to S1c.1.4

The stage doc asked for this explicitly: *"If the zero-firing set is large, say
so before committing to four days."*

**It is large: 38 of 73 never fire, and 23 more rest on a single corpus entry
— 20 of them the same one.** The four-day estimate was written against an
unknown. Against the measurement:

| bucket | rules | fixture cost |
|---|---:|---|
| copiers, products, tag lemmas and fan-outs (§6 rows 1, 2, 4) | 20 | cheap; several share one file |
| extensive ops — need a type extent, and each is also a caveat demo | 4 | medium |
| refutation rules — the negative direction first, then the ⊥ | 11 | medium, and the most valuable |
| stored-negative rules (`compose-negative-*`) | 2 | medium |
| `infer-closure` — the caveat exhibited | 1 | medium |
| `std.slots`' eighteen, currently one file | 18 | **the real cost**: a second, small slot puzzle |
| `forall` open-world-safety, across 10 rules | — | a fixture family of its own, and §7 says write it first |

The 38 zero-firing rules are mostly *cheap*, because they are small generic
rules that need three facts. The expensive item is not in the zero set at all:
it is `std.slots`, which is fully covered and entirely dependent on
`examples/zebra.ein`, and whose eighteen rules need a second activating program
small enough to read —
[`features/09_adjacent_via_same_house.ein`](../../../examples/features/09_adjacent_via_same_house.ein)-sized,
per the phase's own acceptance.

**Recommendation: S1c.1.4 goes from 4 days to 6**, and its task list gains one
that is not in the stage doc — a second `std.slots` program — while
T1c.1.4.1's `std.algebra` task absorbs the 28 never-loaded rules that make up
most of the zero set. The two implementation stages before it are unchanged:
nothing here argues against `:expect`, and §7 is an argument *for* it, since
relation-closure is the only one of the three candidate forms that can state
what a `forall` guard must not do.

---

## 11. The re-take — 2026-08-24, and the zero set is empty

**Taken:** 2026-08-24, `ein 0.1.0`, 180 corpus entries, **557 inference
runs**, `--events-level verbose`, 36.6 s wall. `python3
utils/stdlib_census.py --check` exits **0**.

[S1c.1.4](s1c.1.4_stdlib_corpus.md) shipped **45 programs** under
`tests/stdlib/`, and T1c.1.4.6 is this: the same instrument, re-run, so the
before and after are comparable by construction. One thing about the
instrument changed — `inference_runs` now sweeps `test` runs as well as `solve`
and `saturate`, because a fixture that declared only `test` would have been
invisible to the census. That is neutral on the corpus as it stood: the three
entries that declared `test` before this stage
(`examples/features/1{0,1,2}_expect*.ein`) reach only `symmetric`, which eight
others already do.

| | 2026-08-23 | 2026-08-24 |
|---|---:|---:|
| corpus entries | 128 | **180** |
| inference runs | 400 | **557** |
| rules declared | 73 | 73 |
| **rules no run activates** | **38** | **0** |
| rules that fire but derive nothing | 3 | **0** |
| modules with zero coverage | 2 | **0** |
| rules activated by exactly one entry | 23 | 31 |
| …and how many of those entries exist to be one | 3 of 23 | **31 of 31** |
| …and that entry is `examples/zebra.ein` | **20** | **0** |
| ambiguous attributions | 0 | 0 |

The *before* column is the 2026-08-23 take this document's tables are, not the
tree immediately before S1c.1.4: seven entries landed in between (S1c.1.2's
five `:expect` fixtures and two more), which took the corpus to 135 entries and
419 runs and moved no coverage cell. The 45 that follow are this stage's.

Per module, and the first column is unchanged because the stdlib is:

| module | rules | covered before | covered after |
|---|---:|---:|---:|
| `std.algebra` | 38 | 10 | **38** |
| `std.bijection` | 8 | 6 | **8** |
| `std.closure` | 1 | 0 | **1** |
| `std.elim` | 4 | 1 | **4** |
| `std.slots` | 18 | 18 | 18 |
| `std.typing` | 4 | 0 | **4** |

### Three rows that moved for a reason, not for a count

**§5's whole finding is gone, and it was three rules.** `functional`,
`injective` and `slot-prune-bwd` fired only ever redundantly — matched, passed
their guards, and derived nothing that was not already there. All three are
productive now, and each for the reason §5 predicted:

| rule | p before | p after | what did it |
|---|---:|---:|---|
| `functional` | 0 | 3 | `algebra/14_functional_violated.ein` — the rule **alone**, with no bijection stack to reach `(false)` first and no negative completion to make the violation unreachable |
| `injective` | 0 | 3 | `algebra/15_injective_violated.ein`, the same idea transposed |
| `slot-prune-bwd` | 0 | 3 | `slots/07_spatial_prune.ein` — the puzzle §5 said did not exist |

**And §5's open question is answered.** It asked whether `slot-prune-bwd`'s
sterility was *structural* — both of `examples/zebra.ein`'s spatial relations
being either functional on positions or symmetric, so the two guards coincide
— or an artefact of scheduling at equal priority 250. It is structural. Over a
four-seat diamond, where `above` is asymmetric and functional in **neither**
direction, the two rules derive different exclusions from the same clue pair:
forward excludes a seat that is no *source* of the anchor, backward one that is
no *target*. Exchanging the operands inside either `absent` turns that program
into a contradiction, which was verified by mutating a copy of the stdlib and
re-running it.

**`examples/zebra.ein` is no longer anybody's sole activator.** It carried 20
rules on 2026-08-23; the column is 0 now, and the 31 entries that are somebody's
only activator are all `tests/stdlib/` files whose whole purpose is to be one.
The fragility §4 named — *"delete `examples/zebra.ein` and coverage falls from
35 rules to 15"* — is what this stage was for, and it is worth being precise
about what replaced it: `std.slots`' eighteen rules now have a second
activating program (eight, in fact), so a change to that puzzle's encoding no
longer silently untests a module.

### What the numbers still do not say

The census counts **activation**, and this stage's acceptance is stated in
those terms, so 0/73 closes it. Three things it is not:

- **Not every rule is productive in the new corpus.** `slot-adjacent-fwd`
  fires 54 productive times in `examples/zebra.ein` and only redundantly in
  `slots/07_spatial_adjacent.ein`, because its productive case wants the lower
  end of a spatial clue placed before the upper one and a three-seat row has no
  slack for both orders. Coverage is a firing; a *productive* firing is a
  stronger claim the census reports but does not gate on.
- **Not every promise is pinned.** The seven fan-out rules assert into
  relations that exist only after saturation, and `:expect` may name only a
  relation the program text already makes — so their output is checked through
  what it activates rather than directly
  ([Q-M1c.7](../open_questions.md#q-m1c7--may-an-expectation-name-a-relation-that-only-saturation-creates)).
- **Not a mutation score.** "Every rule fires somewhere" and "every rule's
  guard is tested" are different claims, and only the first is what this
  instrument measures. The second was taken separately and by hand — 51
  deliberate defects, one per rule family, injected into a copy of `stdlib/`
  and run past `ein test tests/` — and it is **50 of 51**, with the survivor
  named in [`tests/README.md`](../../../tests/README.md). Seven fixtures were
  changed because of what it found, which is the difference between a coverage
  number and a test suite. Making either check a *gate* is
  [S1c.1.5](s1c.1.5_gate.md)'s.

---

## Cross-links

- [`utils/stdlib_census.py`](../../../utils/stdlib_census.py) — the instrument;
  `--check` exits 1 while any rule is at zero, which is
  [S1c.1.5](s1c.1.5_gate.md)'s coverage gate in script form
- [`stdlib/README.md`](../../../stdlib/README.md) — the module catalogue, and
  the `std.bijection` vs `std.slots` comparison §4 turns into a fragility
- [`docs/kernel/inference/events.md`](../../../docs/kernel/inference/events.md)
  § Levels — why the census runs at `verbose`
- [S1c.1.2](s1c.1.2_test_form.md) — the form these findings become
  expectations in; §7 and §8 are its two hardest cases
