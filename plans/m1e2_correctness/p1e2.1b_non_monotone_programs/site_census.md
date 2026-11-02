# The site census — 2026-11-02

**S1e2.1b.1's measurement** ([the stage](s1e2.1b.1_the_site_census.md)), for
the rest of [P1e2.1b](README.md). It lists every site of the static check per
site, not per program. It says what the cube observes on every flagged
program, and what route 2 certifies past the limit. It prototypes the check at
fact grain and holds it to the cube. And it re-derives the four older counts
of the flagged set from its own rows.

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example site_census                  # the census, 6 s
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example site_census -- --json > sites.jsonl
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example site_census -- \
    plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/*.ein                                # § 6
cargo test --release --manifest-path ein.rs/Cargo.toml -p ein-infer --test region fact_grain               # § 5.2
```

Taken 2026-11-02 at `4b822a8d`, release build, stdlib = the checkout,
`--jobs 8` (the default) on a 32-thread machine. Every number on this page is
from that output unless it names another command. Two runs gave the same rows
apart from wall-clock times.

**What it selects.** `corpus_files()` and the register's witnesses, which is
[`refutation_census`](../../../ein.rs/crates/ein-infer/examples/refutation_census.rs)'s
selection. That is 267 files: 263 under `examples/`, `stdlib/` and `tests/`, and
4 register witnesses under `corpus/fuzz_findings/`. 213 load and **204** load
and saturate. The 9 that load and do not saturate are 7 `examples/broken/compile/`
fixtures and 2 `ein-bugs/` regression fixtures, all compile errors.

**What it reads.** [`site_census`](../../../ein.rs/crates/ein-infer/examples/site_census.rs)
and `refutation_census` share one loader,
[`examples/shared/census.rs`](../../../ein.rs/crates/ein-infer/examples/shared/census.rs).
It loads the program, builds root's fixpoint as `region::prepare` does, and
calls `monotone::classify`. The census copies none of that logic. The cube is
`region::compute` on a second load of the same program, and the census asserts
that the two loads give one verdict and one universe. Everything at fact grain
comes from
[`examples/shared/fact_grain.rs`](../../../ein.rs/crates/ein-infer/examples/shared/fact_grain.rs),
the prototype, which no route reads.

---

## 1. The headline

| | count |
|---|---:|
| files that load and saturate | 204 of 267 |
| flagged by the static check | **22** |
| … within `--region-limit 16` | 14: **6 exercised**, **8 conservative** |
| … past it | 8 |
| sites | **32**: 27 `absent`, 4 obligation triggers, 1 activator |
| … written in the program | 26 |
| … written in the stdlib | 6: `std.elim` 4 (one program, `features/05`), `std.algebra`'s `total-owed` 2 |
| … whose rule concludes `(false)` | 13, all `absent` sites. This is S1e2.4.1's `direct` set, 13 rules over 9 entries |
| … whose rule fires at root's fixpoint | 13: 4 within the limit, 9 past it |
| minimal holes and violation witnesses | 15, each attributed to at least one site |
| monotone cubes with a violation | 0 of 155 |
| flagged programs that leave the set at fact grain | **2**: `features/05`, `features/12`. Both are past the limit |

## 2. The sites

One row per site, in the census's order. The columns:

- **reads @d** is the relation and its `absent` depth.
- **concludes** is the site's rule's `:assert`: `(false)`, `(open …)` for an
  obligation, or the positive relations it writes.
- **at root** counts the rule's firings that derived a fact in root's
  saturation, or, for an obligation, the instances root owes. A non-zero
  count is where root holds a fact derived under the site, which is the
  fork-or-scratch question ([S1e2.1b.2](s1e2.1b.2_fork_or_scratch.md)).
- **fact grain** is the prototype's verdict on the site (§ 5). Where the site
  is dynamic, it shows one fact a commitment can add that its pattern matches.

| program | n | rule [activator] | kind | reads @d | concludes | at root | written | fact grain |
|---|---:|---|---|---|---|---:|---|---|
| [`corpus/fuzz_findings/d3-unsat-core.ein`](../../../corpus/fuzz_findings/d3-unsat-core.ein) | 5 | `fire-1` | absent | `(not r1)` @1 | `r1` | 0 | program :12 | dynamic, `(not (r1 _ _))` |
| [`branching/13_lookahead_naf_world.ein`](../../../examples/branching/13_lookahead_naf_world.ein) | 2 | `self-block` | absent | `cand` @1 | `(false)` | 0 | program :50 | dynamic, `(cand B)` |
| [`branching/14_lookahead_unjudgeable.ein`](../../../examples/branching/14_lookahead_unjudgeable.ein) | 2 | `premature-signoff` | absent | `signed` @3 | `(false)` | 0 | program :62 | dynamic, `(signed B)` |
| [`ein-bugs/alive-empty-interlayer.ein`](../../../examples/ein-bugs/alive-empty-interlayer.ein) | 3 | `totality` | absent | `p` @1 | `(false)` | 0 | program :56 | dynamic, `(p A)` |
| [`ein-bugs/naf-upward-closure.ein`](../../../examples/ein-bugs/naf-upward-closure.ein) | 2 | `bad` | absent | `q` @1 | `(false)` | 0 | program :70 | dynamic, `(q A)` |
| [`ein-bugs/wrong-answers/f07-fpoff.ein`](../../../examples/ein-bugs/wrong-answers/f07-fpoff.ein) | 1 | `derive-p` | absent | `q` @1 | `p` | 1 | program :32 | dynamic, `(q one)` |
| ″ |  | `derive-q` | absent | `p` @1 | `q` | 0 | program :38 | static |
| [`ein-bugs/wrong-answers/naf-nowb.ein`](../../../examples/ein-bugs/wrong-answers/naf-nowb.ein) | 2 | `bad` | absent | `q` @1 | `(false)` | 0 | program :50 | dynamic, `(q A)` |
| [`ein-bugs/wrong-answers/stale-naf-fork-warn.ein`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork-warn.ein) | 1 | `derive-p` | absent | `q` @1 | `p` | 1 | program :19 | dynamic, `(q one)` |
| [`ein-bugs/wrong-answers/stale-naf-fork.ein`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein) | 1 | `derive-p` | absent | `q` @1 | `p` | 1 | program :39 | dynamic, `(q one)` |
| [`features/01_not_and_absent.ein`](../../../examples/features/01_not_and_absent.ein) | 35 | `unknown-feeling` | activator | `unknown-feeling` @0 | `unknown-feeling` | 4 | program :33 | static |
| ″ |  | `unknown-feeling` [Person] | absent | `likes` @1 | `unknown-feeling` | 4 | program :33 | dynamic, `(likes Alice Person)` |
| [`features/04_open.ein`](../../../examples/features/04_open.ein) | 81 | `open-candidate` | absent | `house-color` @1 | `open-slot` | 7 | program :36 | dynamic, `(house-color Blue House-1)` |
| [`features/05_stdlib_domain_elim.ein`](../../../examples/features/05_stdlib_domain_elim.ein) | 56 | `domain-elimination` [color-of is-a House Color] | absent | `is-a` @1 | `color-of` | 1 | [`std.elim`](../../../stdlib/elim.ein) :55 | static |
| ″ |  | `no-room-left` [color-of is-a House Color] | absent | `is-a` @1 | `(false)` | 0 | [`std.elim`](../../../stdlib/elim.ein) :67 | static |
| ″ |  | `typecheck-arg-0` [color-of is-a House] | absent | `is-a` @1 | `(false)` | 0 | [`std.elim`](../../../stdlib/elim.ein) :41 | static |
| ″ |  | `typecheck-arg-1` [color-of is-a Color] | absent | `is-a` @1 | `(false)` | 0 | [`std.elim`](../../../stdlib/elim.ein) :48 | static |
| [`features/07_unstratifiable.ein`](../../../examples/features/07_unstratifiable.ein) | 1 | `derive-p` | absent | `q` @1 | `p` | 1 | program :23 | dynamic, `(q one)` |
| ″ |  | `derive-q` | absent | `p` @1 | `q` | 0 | program :29 | static |
| [`features/12_expect_false.ein`](../../../examples/features/12_expect_false.ein) | 20 | `forced` [in instance] | absent | `instance` @1 | `in` | 5 | program :67 | static |
| ″ |  | `no-room` [in instance] | absent | `instance` @1 | `(false)` | 1 | program :78 | static |
| [`lattice/04_obligation_trigger_hole.ein`](../../../examples/lattice/04_obligation_trigger_hole.ein) | 2 | `total-owed` [meal is-a] | obligation-trigger | `is-a` @0 | `(open …)` | 0 | [`std.algebra`](../../../stdlib/algebra.ein) :336 | dynamic, `(is-a Ann Diner)` |
| [`lattice/06_friends_tables.ein`](../../../examples/lattice/06_friends_tables.ein) | 19 | `group-seated-owed` | obligation-trigger | `sits` @0 | `(open …)` | 0 | program :67 | dynamic, `(sits Ann B1)` |
| [`lattice/synth/three_components.ein`](../../../examples/lattice/synth/three_components.ein) | 7 | `dead-3` | absent | `h` @1 | `(false)` | 0 | program :48 | dynamic, `(h B)` |
| ″ |  | `dead-7` | absent | `h` @1 | `(false)` | 0 | program :72 | dynamic, `(h D)` |
| [`lattice/synth/trigger_hole.ein`](../../../examples/lattice/synth/trigger_hole.ein) | 5 | `dead-1` | absent | `h` @1 | `(false)` | 0 | program :35 | dynamic, `(h B)` |
| ″ |  | `dead-2` | absent | `h` @1 | `(false)` | 0 | program :41 | dynamic, `(h E)` |
| ″ |  | `owe-2` | obligation-trigger | `h` @0 | `(open …)` | 0 | program :53 | dynamic, `(h B)` |
| [`saturation/square-unique/corner-house.ein`](../../../examples/saturation/square-unique/corner-house.ein) | 118 | `square-unique` [next-to House] | absent | `next-to` @1 | `co-located` | 1 | program :29 | dynamic, `(next-to Blue Color)` |
| [`saturation/square-unique/cul-de-sac.ein`](../../../examples/saturation/square-unique/cul-de-sac.ein) | 114 | `square-unique` [next-to Lot] | absent | `next-to` @1 | `co-located` | 1 | program :29 | dynamic, `(next-to Alice Lot)` |
| [`saturation/square-unique/terminus.ein`](../../../examples/saturation/square-unique/terminus.ein) | 153 | `square-unique` [next-to Station] | absent | `next-to` @1 | `co-located` | 1 | program :31 | dynamic, `(next-to Bridge Central)` |
| [`tests/region/07_cube.ein`](../../../tests/region/07_cube.ein) | 2 | `total-owed` [meal is-a] | obligation-trigger | `is-a` @0 | `(open …)` | 0 | [`std.algebra`](../../../stdlib/algebra.ein) :336 | dynamic, `(is-a Ann Diner)` |

**By kind.** The 27 `absent` sites sit on 19 programs. The 4 triggers are
`total-owed` twice (`lattice/04` and `tests/region/07_cube`, one program
written twice), `group-seated-owed` (`lattice/06`) and `owe-2`
(`synth/trigger_hole`). **The one activator site** is `features/01`'s
`unknown-feeling`, and it is never exercised: the program is past the limit
(n = 35), and at fact grain the site is static. The rule writes
`unknown-feeling` at arity 2, its activator has arity 1, and
`compile::activators_for` filters by arity. **So the corpus has no exercised
activator site**, and [S1e2.1b.6](s1e2.1b.6_monotone_and_non_monotone_programs_compared.md)'s
synthetic fixture for that cause is still owed. This stage writes none, as its
notes say.

**By where written.** The 6 stdlib sites come from two programs' imports. The
4 `std.elim` sites are all on `features/05`, with `is-a` open. The 2
`total-owed` sites are on `lattice/04` and its copy under `tests/region/`. No
stdlib rule is a site in any other program.

**Fires at root.** Within the limit, only W8's family fires at root:
`derive-p` on `f07-fpoff`, `stale-naf-fork`, its `-warn` twin and
`features/07`. Only on those four does root hold a fact derived under a site.
So they are the cubes within the limit where the fork node and the scratch
node can differ. That is an argument: S1e2.1b.2 compares the two routes over
all 14. Past the
limit, 9 sites fire at root, on `features/01`, `/04`, `/05`, `/12` and the three
`square-unique` demos.

**The `-y` abort** on `corpus/fuzz_findings/d3-unsat-core.ein` (2026-10-16,
before the switch, [region census § 9](../p1e2.1a_the_solution_region/region_census.md#9-the-config-principle))
comes from that program's one site: `fire-1`, which reads `(not r1)` at depth
1, concludes `(r1 …)` and does not fire at root. Its cube is conservative.
S1e2.1b.2 owns the abort.

## 3. The flagged programs within the limit

| program | n | \|R\| | holes (minimal) | dead, alive superset | discharged, owing superset | verdict |
|---|---:|---:|---:|---:|---:|---|
| [`corpus/fuzz_findings/d3-unsat-core.ein`](../../../corpus/fuzz_findings/d3-unsat-core.ein) | 5 | 20 | 0 (0) | 0 | 0 | conservative |
| [`branching/13_lookahead_naf_world.ein`](../../../examples/branching/13_lookahead_naf_world.ein) | 2 | 4 | 0 (0) | 0 | 0 | conservative |
| [`branching/14_lookahead_unjudgeable.ein`](../../../examples/branching/14_lookahead_unjudgeable.ein) | 2 | 4 | 0 (0) | 0 | 0 | conservative |
| [`ein-bugs/alive-empty-interlayer.ein`](../../../examples/ein-bugs/alive-empty-interlayer.ein) | 3 | 4 | 0 (0) | 0 | 0 | conservative |
| [`ein-bugs/naf-upward-closure.ein`](../../../examples/ein-bugs/naf-upward-closure.ein) | 2 | 3 | 1 (1) | 1 | 0 | exercised |
| [`ein-bugs/wrong-answers/f07-fpoff.ein`](../../../examples/ein-bugs/wrong-answers/f07-fpoff.ein) | 1 | 2 | 0 (0) | 0 | 0 | conservative |
| [`ein-bugs/wrong-answers/naf-nowb.ein`](../../../examples/ein-bugs/wrong-answers/naf-nowb.ein) | 2 | 3 | 1 (1) | 1 | 0 | exercised |
| [`ein-bugs/wrong-answers/stale-naf-fork-warn.ein`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork-warn.ein) | 1 | 1 | 0 (0) | 0 | 0 | conservative |
| [`ein-bugs/wrong-answers/stale-naf-fork.ein`](../../../examples/ein-bugs/wrong-answers/stale-naf-fork.ein) | 1 | 1 | 0 (0) | 0 | 0 | conservative |
| [`features/07_unstratifiable.ein`](../../../examples/features/07_unstratifiable.ein) | 1 | 2 | 0 (0) | 0 | 0 | conservative |
| [`lattice/04_obligation_trigger_hole.ein`](../../../examples/lattice/04_obligation_trigger_hole.ein) | 2 | 3 | 1 (1) | 0 | 1 | exercised |
| [`lattice/synth/three_components.ein`](../../../examples/lattice/synth/three_components.ein) | 7 | 13 | 3 (2) | 3 | 0 | exercised |
| [`lattice/synth/trigger_hole.ein`](../../../examples/lattice/synth/trigger_hole.ein) | 5 | 9 | 7 (2) | 3 | 4 | exercised |
| [`tests/region/07_cube.ein`](../../../tests/region/07_cube.ein) | 2 | 3 | 1 (1) | 0 | 1 | exercised |

**Exercised: 6.** These are W3's four witnesses (two of them written as
probes, two by `region_synth.py`), `lattice/04`, and `tests/region/07_cube`,
which is `lattice/04`'s program banked as an `ein test` fixture.
**Conservative: 8.** These are [region census § 8](../p1e2.1a_the_solution_region/region_census.md#8-monotonicity-the-static-check-against-the-cube)'s
eight, unchanged. Every one is flagged over a relation the program means to
decide (`q`, `cand`, `signed`, `p`, `r1`). Each site there is a real
non-monotone read, and the corpus program never makes it bite.

**Each minimal hole, and each violation's smallest witness, attributed.** A
dead node names the `absent` sites over the relation of each of its `absent`
premises (`region::Node::absent_premises`). Where one of those sites' rules
refuted the node (`refuted_by`), only that site is named. An owing node names
the trigger sites of the instances it owes that the node of R below it does
not (a fresh `obligations::tally` on each fixpoint, through `region::Nodes`).

| program | what | node | kind | why | site |
|---|---|---|---|---|---|
| `ein-bugs/naf-upward-closure.ein` | minimal hole | `{(p A)}` | dead | `(absent (q A))`; refuted by `bad` | `bad` |
| `ein-bugs/naf-upward-closure.ein` | dead ↑ witness | `{(p A)}` ⊂ `{(p A), (q A)}` | dead | `(absent (q A))`; refuted by `bad` | `bad` |
| `ein-bugs/wrong-answers/naf-nowb.ein` | minimal hole | `{(p A)}` | dead | `(absent (q A))`; refuted by `bad` | `bad` |
| `ein-bugs/wrong-answers/naf-nowb.ein` | dead ↑ witness | `{(p A)}` ⊂ `{(p A), (q A)}` | dead | `(absent (q A))`; refuted by `bad` | `bad` |
| `lattice/04_obligation_trigger_hole.ein` | minimal hole | `{(seats Ann C1)}` | owing | owes: *meal owes Ann a Food.* | `total-owed [activator meal is-a]` |
| `lattice/04_obligation_trigger_hole.ein` | discharged ↑ witness | `∅` ⊂ `{(seats Ann C1)}` | R | owes: *meal owes Ann a Food.* | `total-owed [activator meal is-a]` |
| `lattice/synth/three_components.ein` | minimal hole | `{(h A), (h F)}` | dead | `(absent (h B))`; refuted by `dead-3` | `dead-3` |
| `lattice/synth/three_components.ein` | minimal hole | `{(h A), (h G)}` | dead | `(absent (h D))`; refuted by `dead-7` | `dead-7` |
| `lattice/synth/three_components.ein` | dead ↑ witness | `{(h A), (h F)}` ⊂ `{(h A), (h B), (h F)}` | dead | `(absent (h B))`; refuted by `dead-3` | `dead-3` |
| `lattice/synth/trigger_hole.ein` | minimal hole | `{(h A), (h B)}` | owing | owes: *with B: one of C is required* | `owe-2` |
| `lattice/synth/trigger_hole.ein` | minimal hole | `{(h A), (h D)}` | dead | `(absent (h B))`, `(absent (h E))`; refuted by `dead-1` | `dead-1` |
| `lattice/synth/trigger_hole.ein` | dead ↑ witness | `{(h A), (h D)}` ⊂ `{(h A), (h B), (h D)}` | dead | `(absent (h B))`, `(absent (h E))`; refuted by `dead-1` | `dead-1` |
| `lattice/synth/trigger_hole.ein` | discharged ↑ witness | `{(h A)}` ⊂ `{(h A), (h B)}` | R | owes: *with B: one of C is required* | `owe-2` |
| `tests/region/07_cube.ein` | minimal hole | `{(seats Ann C1)}` | owing | owes: *meal owes Ann a Food.* | `total-owed [activator meal is-a]` |
| `tests/region/07_cube.ein` | discharged ↑ witness | `∅` ⊂ `{(seats Ann C1)}` | R | owes: *meal owes Ann a Food.* | `total-owed [activator meal is-a]` |

Every item names a site, and the one it names is the one the program was
written to show. The dead holes are named by their refuting rule's `absent`,
and the owing holes by their trigger.

**Since S1e2.1b.3 (2026-11-02)** the rule is
[`examples/shared/attribution.rs`](../../../ein.rs/crates/ein-infer/examples/shared/attribution.rs),
which `hole_census` reads too, and each item carries a cause beside its sites.
It grew a clause for each cause the fixtures banked after this page found
uncovered (`lattice/08`, `lattice/09`), and one for a refutation a node
compiles. Its re-take at `4562bf70` names a site for all 23 items of the 18
flagged cubes within the limit
([hole census § 8](hole_census.md#8-the-attribution-gap-closed)).

## 4. Past the limit

What a bare `ein solve` certifies today on each flagged program past
`--region-limit 16` ([`read_out.md` § 6.2](../p1e2.1a_the_solution_region/read_out.md#62-a-partial-run-c3)).
The census runs `explore::run` with that mode. "Whole layers" counts the
layers in which every commitment was visited.

| program | n | rung | route today | saturations | whole layers | members of R met | minimal certified | ∅ in R |
|---|---:|---|---|---:|---:|---:|---:|---|
| [`features/01_not_and_absent.ein`](../../../examples/features/01_not_and_absent.ein) | 35 | blind | 2, L7, partial | 64 655 | 0–4 | ≥ 64 655 | 1 | yes |
| [`features/04_open.ein`](../../../examples/features/04_open.ein) | 81 | blind | 2, L7, partial | 64 761 | 0–2 | ≥ 64 761 | 1 | yes |
| [`features/05_stdlib_domain_elim.ein`](../../../examples/features/05_stdlib_domain_elim.ein) | 56 | blind | 2, L7, partial | 65 156 | 0–3 | ≥ 7 176 | 1 | yes |
| [`features/12_expect_false.ein`](../../../examples/features/12_expect_false.ein) | 20 | blind | 1, L0 | 1 | all | 0 | 0 | no — R is empty |
| [`lattice/06_friends_tables.ein`](../../../examples/lattice/06_friends_tables.ein) | 19 | hrules | 2, L7, partial | 65 299 | 0–6 | ≥ 21 | 1 | yes |
| [`saturation/square-unique/corner-house.ein`](../../../examples/saturation/square-unique/corner-house.ein) | 118 | blind | 2, L7, partial | 65 389 | 0–2 | ≥ 65 389 | 1 | yes |
| [`saturation/square-unique/cul-de-sac.ein`](../../../examples/saturation/square-unique/cul-de-sac.ein) | 114 | blind | 2, L7, partial | 64 923 | 0–2 | ≥ 64 923 | 1 | yes |
| [`saturation/square-unique/terminus.ein`](../../../examples/saturation/square-unique/terminus.ein) | 153 | blind | 2, L7, partial | 65 029 | 0–2 | ≥ 65 029 | 1 | yes |

- **`features/12` is already exact.** Root is dead, so L0 decides every node:
  R = ∅, exhausted, in one saturation. Whatever grain the check reads at, its
  answer cannot move.
- **The others certify ∅ as their one minimal state and nothing else**: ∅ is
  in R, so the minimal line is exactly {∅}. The maximal line needs the whole
  cube, and the row says so. `lattice/06` met 21 members, which is all of R
  as [S1e2.1b.6](s1e2.1b.6_monotone_and_non_monotone_programs_compared.md) states it. Nothing
  here certifies that.
- **The oracle's cost at 17–19.** Only `lattice/06_friends_tables` is in that
  band among the flagged programs. Its projection (`region::Projection`, 64
  nodes probed across the cube, at one job) is 524 288 nodes × 88 µs ≈
  **46 s** at `--region-limit 19`. The census reads the projection with a
  deadline of now, so the cube stops before its first batch. The other seven
  are at n = 20 (`features/12`) or n ≥ 35.

## 5. The grain

### 5.1 The prototype

Relation grain calls a read **dynamic** when its relation is in D. The
prototype calls it dynamic only when its **pattern** can match a fact a
commitment can add. It builds Δ̂, an over-approximation of every fact some node
holds beyond root, as a closure over **shapes**: facts with some arguments
*any*. The seeds are the candidates. A saturation plan adds its conclusions
when a read at **even** `absent` depth unifies with a shape in Δ̂. A depth-0
read passes its bindings to the conclusions. Every other variable becomes
*any*.

Only even depth counts, because a fork only adds. A firing a node makes and
root did not had a match that was false at root's fixpoint and true later, and
growth makes a match true only through an even-depth read. The module docs
give the argument and its one gap. When a commitment can derive an **activator**,
a node can compile a plan root never had. The prototype then marks its closure
incomplete, and the program stays flagged.

A site at fact grain is a relation-grain site one of whose reads matches Δ̂.
Polarity is read exactly as the shipped check reads it.

### 5.2 Its test

Two tests in `ein-infer/tests/region.rs` hold the prototype to the cube. They
include the prototype with `#[path]` and read no file under `plans/`.

- **`fact_grain_never_calls_a_violated_cube_monotone`.** On no cube within the
  limit may the fact-grain verdict be *monotone* while the cube observes a
  hole, a dead node with an alive superset, or a discharged node with an
  owing superset. It **holds** on every corpus and register cube within the
  limit that fact grain calls monotone: 155 in the release rows. **None of the
  155 is flagged at relation grain**, so on the corpus this checks the
  relation-grain theorem again and nothing more. No corpus program within the
  limit leaves the set.
- **`fact_grain_on_programs_written_to_test_it`.** Six programs written into
  the test give the claim cases that bite. Each is flagged at relation grain.

  | program | what it tests | fact grain | its cube |
  |---|---|---|---|
  | `derived` | a fact `tag` derives from a candidate fills `bad`'s `(absent (q ?x Tagged))` | flagged | a dead hole |
  | `static` | the same, reading `(q ?x Other)`, which nothing a commitment adds has | **leaves** | no violation |
  | `even-depth` | `done` is derived through a read at depth 2 (a `forall` written out) | flagged | a dead hole |
  | `negative` | a stored negative a commitment derives fills the read | flagged | a dead hole |
  | `activator` | a commitment derives `(watch q)`, an activator | flagged, closure incomplete | no violation |
  | `type-constant` | `features/05`'s shape within the limit: candidates are memberships between objects, the read asks for membership in a type | **leaves** | no violation |

  **The test has teeth.** With the closure following only top-level reads,
  the prototype calls `even-depth` monotone beside its dead hole. That
  mutation was run on the census on 2026-11-02, and the census marked the
  program `UNSOUND`.

### 5.3 What leaves the set

| program | n | sites | why each goes static at fact grain |
|---|---:|---|---|
| [`features/05_stdlib_domain_elim.ein`](../../../examples/features/05_stdlib_domain_elim.ein) | 56 | `std.elim`'s `typecheck-arg-0`, `typecheck-arg-1`, `no-room-left`, `domain-elimination`, all reading `is-a` at depth 1 | the reads ask for membership in a type, `(is-a ?a House)`. The 30 `is-a` candidates are memberships between objects, such as `(is-a Red House-1)`, and no rule derives `is-a` |
| [`features/12_expect_false.ein`](../../../examples/features/12_expect_false.ein) | 20 | `forced`, `no-room`, reading `instance` at depth 1 | the same, with `(instance ?h Hole)` against 20 candidates between objects |

The other 20 stay flagged. On 17 of them the read is over a relation of H, and
a candidate matches it. On the other 3 only a fact a plan derives from a
candidate matches it. On `lattice/04` and its copy `tests/region/07_cube`,
`seated-dines` derives `(is-a Ann Diner)` from a seat. On `d3-unsat-core`,
a rule derives `(not (r1 _ _))`.

A site can go static while its program stays flagged. There are two cases.
One is `derive-q` on `features/07` and `f07-fpoff`: it reads `p`, which root
derives and a fork never loses. The other is `features/01`'s activator site.
Those per-site verdicts hold for the fork node only (`fact_grain.rs` § Which
node).

**What (b) would change.** Within the limit, nothing: no program leaves.
`features/12`'s answer is L0's and cannot move. Only its lattice row and its
`monotonicity` block would. `features/05` would move from route 2's partial
answer (7 176 members of R met in 65 156 saturations, the maximal line not
known) to route 1, with the licences. How far route 1 gets on its 56
candidates is **not measured**, because measuring it means routing on the
prototype. That is (b)'s build, not evidence for it. The goldens (b) would
move are `corpus_shapes.md5`'s renderings of `features/05`, which carry the
`monotonicity` event (S1e2.1a.1 named them when the event was added). The
stage file names the rest: both programs' `--json-summary` `monotonicity`
blocks and their stdout's `lattice` row.

### 5.4 What C1 would reach

C1 is [S1e2.2.10](../p1e2.2_wrong_answers/s1e2.2.10_types_as_objects.md), and
its three readings are [Q-M1e.12](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e12--the-blind-rung-is-untyped-and-a-model-binds-a-type-as-an-object)'s.
The census asks the shipped check what each would do to the flag.

- **(c), `candidate_objects` excludes types.** S1e2.2.10 defines a type as an
  object on the right of an `is-a`. The census drops every candidate naming
  one and classifies again. That **unflags none of the 22**. The candidates
  the five structural programs' sites read name no type: `(is-a Red House-1)`
  and `(next-to Bridge Central)` stay, while 61–86 candidates per
  `square-unique` demo go.
- **(b), the read-out.** A presentation change: no verdict moves.
- **(a), the program is under-specified.** That is an edit to the program,
  and the census says which edit unflags each one. Taking one relation out of
  H leaves no site on 18 of the 22. On 6 of the 18 that relation is one the
  program states by given facts and no rule derives:

  | program | relation | facts of it at root | reached by fact grain |
  |---|---|---:|---|
  | `features/05` | `is-a` | 8 | yes |
  | `features/12` | `instance` | 5 | yes |
  | `square-unique/corner-house` | `next-to` | 2 | no |
  | `square-unique/cul-de-sac` | `next-to` | 5 | no |
  | `square-unique/terminus` | `next-to` | 7 | no |
  | `features/04_open` | `house-color` | 1 | no |

  The first five are S1e2.1a.1's five. S1e2.4.1 made the edit on two of them
  and both became monotone: `features05_scoped` and `features12_scoped` in
  [`s1e2.4.1_probes/`](../p1e2.4_what_the_switch_left/s1e2.4.1_probes/).
  `features05_scoped` has an exact R, 32 states over 26 candidates.
  `features/04`'s `house-color` is the puzzle's own relation with one clue
  given. A mechanical rule, *the blind rung does not propose a relation the
  program only states*, would reach it too, which is why "structural" is a
  reading and not a test.

So **C1 reaches none of the conservative eight**. Each is flagged over a
relation the program decides and states no fact of. It reaches the five only
by editing them. Fact grain reaches two of the five with no edit.
It does not reach `square-unique`: the blind rung proposes `next-to` between
any two objects, so `(next-to ?x ?z)` matches a candidate.

### 5.5 The evidence, against the question

| option | what the census says it does |
|---|---|
| **(a) relation grain** | 22 flagged. The 8 past the limit keep route 2's partial answer, and `features/12` keeps its exact L0 answer |
| **(b) fact grain** | 20 flagged. No program within the limit leaves, so no R the cube computes can move. Past it, `features/05` gets route 1, at a reach not measured, and `features/12`'s answer stays L0's. The test holds on every cube, but on the corpus it distinguishes nothing; the cases where the grains differ are the six written programs. No unsound case was found. Building it is moving the prototype into `monotone.rs`, restating the theorem at four places and moving the goldens of § 5.3: about 1.5 days, in this stage, if ruled |
| **(c) C1's reading first** | (c) and (b) of Q-M1e.12 unflag nothing. (a) is five program edits, two of them banked as probes. It is a reorder of P1e2.2's S1e2.2.10, and the user's |

**Two things the grain also touches.**
[S1e2.2.5](../p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md)'s F1
(S1e2.4.1 § 4) lists the same reading as its option (c) for what *extendable*
means. Under it the refusal would not reach `std.elim`, and F5 would be moot
for `features/05`. The prototype is that reading, with a test. And the
program-level verdict should hold whichever node S1e2.1b.2 rules. That is an
argument (`fact_grain.rs` § Which node) and is not tested here.

**A recommendation, mine, now that the prototype has run: (a) for now.** Keep
the prototype and its tests as they are, as the evidence S1e2.2.5's F1 (c)
will want. Fact grain changes no answer the cube can check. Past the limit it
changes one program's route, to a reach nobody has measured. And its evidence
where the grains differ is six programs written for the test. If the user
would rather `features/05` and `features/12` stop being called non-monotone
now, (b) is safe as far as the cube can tell, and the table above prices it.

**Ruled 2026-11-02: (a)** ([the stage](s1e2.1b.1_the_site_census.md#the-question-it-puts-to-the-user)).
The check stays at relation grain, and the prototype and its tests stay as
evidence.

## 6. The S1e2.4.1 probes

A second invocation over the 14 files in
[`s1e2.4.1_probes/`](../p1e2.4_what_the_switch_left/s1e2.4.1_probes/). It
gets its own run, because `plans/` is deleted at the milestone's close.

| | |
|---|---|
| files | 14, all load and saturate |
| flagged | 5, all within the limit |
| exercised | 4: `naf_open_hrules`, `three_components_owe`, `trigger_hole_owe` (owing holes, each named by its trigger), `via_hole` (a dead hole) |
| conservative | 1: `not_conclusion` |
| sites | 8: 2 `absent`, 6 triggers, all written in the probe |
| leave at fact grain | 0 |

**`via_hole.ein`'s dead hole** `{(r one), (s one)}` is refuted by `clash`,
which reads no `absent`. Its premise `(absent (q one))` belongs to the rule
that derived `p`, so the census names `derive-p`, the `absent` site over `q`,
by relation. That is the derived class S1e2.4.1 found, seen from the cube.
`features05_scoped`, `features12_scoped`, `dual_rail` and the other six are
monotone.

## 7. The four counts, reconciled

Each count is re-derived from this census's rows under its own selection. The
census prints all four at the foot of its text output. The selection at each
old commit is `git ls-tree -r --name-only COMMIT examples stdlib tests` (the
`.ein` files) plus `git show COMMIT:corpus/wrong_answers.toml`'s witnesses.

| count | taken | its selection | then | today | every difference |
|---|---|---|---|---|---|
| S1e2.1a.1's `census_for_inspection` ([§ Outcome](../p1e2.1a_the_solution_region/s1e2.1a.1_the_oracle.md#outcome-2026-10-15)) | 2026-10-15, `e24ae3ea` | `corpus_files()`, `region::compute` succeeds | 183 · 150 at n ≤ 16 · 17 flagged · a hole on 3 | 200 · 165 · 21 · 6 | **26 files added since.** 9 are `examples/broken/load/` fixtures and do not load. The other 17 add 17 programs, 15 of them at n ≤ 16 (all but the two `lattice/06_friends_tables*` at n = 19), 4 flagged (`lattice/06_friends_tables`, `synth/three_components`, `synth/trigger_hole`, `tests/region/07_cube`) and 3 holes (the last three) |
| `region_census.py --all` ([§ 1](../p1e2.1a_the_solution_region/region_census.md#1-reach), [§ 8](../p1e2.1a_the_solution_region/region_census.md#8-monotonicity-the-static-check-against-the-cube)) | 2026-10-16, `9248236d` | corpus + register, through the CLI | 159 regions at n ≤ 16 · 13 flagged · a violation on 5 | 169 · 14 · 6 | **18 files added since**: the 9 load fixtures and `tests/region/01`–`09`, which add 9 regions, 1 of them flagged and violated (`07_cube`). **The tenth region is the route, not a file.** [`ein-bugs/query-goal-free-head.ein`](../../../examples/ein-bugs/query-goal-free-head.ein) loads and saturates in process (n = 0). `ein solve` refuses its `:goal` at compile, so `region_census.py`, which runs the binary, got no `region.json` and counted it among the 55 that "do not load". This was re-run with the `3fde4ef1` binary on the 195 programs on 2026-11-02: it is the only one with no region. Today's binary still refuses it |
| `route1_reach` ([§ 1](../p1e2.1a_the_solution_region/route1_reach.md#1-the-three-routes-over-the-corpus)) | 2026-10-16, `3fde4ef1` | corpus + register, load and compile | 195 · 13 flagged at n ≤ 16 · 8 past it | 204 · 14 · 8 | the 9 `tests/region/` files: +9 programs and +1 flagged within the limit (`07_cube`). The same 8 past it. Its page says the one program it has beyond the region census is one *"the census's selection does not hold"*. It is `query-goal-free-head`, which the census selected and the CLI refused, as the row above says |
| `refutation_census` ([S1e2.4.1 § 1](../p1e2.4_what_the_switch_left/s1e2.4.1_refutation_under_absent.md#1-the-exposed-set-re-counted)) | 2026-11-02, `6e7dfc49` | corpus + register | 267 files · 204 load and saturate · 22 flagged | 267 · 204 · 22 | none: no file added since |

Every difference is a file added since, apart from one route difference. No
count was wrong, and route1_reach.md gives the wrong reason for its one row.
The 17 flagged files of 2026-10-15 are today's 21 in the corpus less the 4
added since. The 13 of 2026-10-16 are today's 14 within the limit less
`07_cube`.

## 8. What the other stages take from it

| stage | takes |
|---|---|
| [S1e2.1b.2](s1e2.1b.2_fork_or_scratch.md), fork or scratch | the 14 flagged cubes within the limit (232 nodes in all, n ≤ 7) and the 5 flagged probes. The four on which a site's rule fires at root (W8's family, § 2) are where fork and scratch can differ. The `-y` abort's site |
| [S1e2.1b.3](s1e2.1b.3_holes.md), holes | the 6 corpus programs with a hole and the 4 probes, each minimal hole with its kind and its site (§ 3, § 6). `lattice/06` at `--region-limit 19`, projected at 46 s |
| [S1e2.1b.4](s1e2.1b.4_refutation_resting_on_absent.md), a refutation on an `absent` | the 13 sites that conclude `(false)` (S1e2.4.1's `direct` set), the dead holes and dead witnesses with their `absent` premises named, and `via_hole`'s dead hole named through the deriving rule |
| [S1e2.1b.5](s1e2.1b.5_the_detector.md), the handling | the 22 programs that carry the stub today, and the two that leave it if the grain ruling is (b) |
| [S1e2.1b.6](s1e2.1b.6_monotone_and_non_monotone_programs_compared.md), the comparison | the *detection* row: 22 of 204 programs flagged, 6 exercised and 8 conservative within the limit, 0 violations on 155 monotone cubes, and the corpus's one activator site, never exercised |
| [S1e2.1c.3](../p1e2.1c_the_region_search/s1e2.1c.3_the_non_monotone_search.md) | the 8 flagged programs past the limit and what route 2 certifies on each (§ 4) |

## 9. The re-estimate

S1e2.1b.2–.5 were specified on 2026-11-02 before this census
([phase README § Stages](README.md#stages)). Re-estimated from it the same
day:

| stage | was | now | why |
|---|---:|---:|---|
| S1e2.1b.2 | 3 d | **2.5 d** | its comparison is 14 cubes of at most 128 nodes and 5 probes, so `utils/region.py` is fast enough and no library prototype is needed. The census names the four programs where fork and scratch can differ. The stratification check is still new |
| S1e2.1b.3 | 2 d | **1.5 d** | every corpus hole's kind, source and site is here. What is left is the repair edges, the sub-cubes, `lattice/06` at 19 (46 s), the drafts and the ruling |
| S1e2.1b.4 | 1.5 d | **1 d** | the refuting sites, and the dead holes with their `absent` premises named, are here. The taint census classifies those premises, and its test is small |
| S1e2.1b.5 | 3 d | **3 d** | unchanged. It still waits on .2–.4's rulings. The census names its programs: 22 with the stub, and 2 that leave if (b) |

The phase goes from 15.5 to **14 days**, and the milestone from 102.5 to 101.
Building fact grain, if the user rules (b), is about 1.5 days in this stage
and is not in either total.
