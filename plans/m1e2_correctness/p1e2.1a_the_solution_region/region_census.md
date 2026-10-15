# The region census — 2026-10-16

**S1e2.1a.3's measurement**: the region R of every corpus program the oracle
reaches, put on the table beside the rulings of
[Q-M1e2.7](../open_questions.md#q-m1e27--the-answer-is-a-region-of-the-lattice).
The definition it measures is
[`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md),
rewritten by the same stage.

```sh
python3 utils/region_census.py --all --levers --json rows.json   # 191 s on 12 jobs
python3 utils/region_census.py --from rows.json                  # the tables again
python3 utils/region_census.py                                   # the acceptance's selection, 7 s
```

Taken 2026-10-16 at `9248236d`, release build, stdlib = the checkout. Run
twice; the two runs' tables are identical. Every number on this page is from
that output unless it names another command.

**What it reads.** `ein solve FILE --dump-region DIR`, the oracle's
`region.json` and `walk.json`
([`region_dump.md`](../../../docs/kernel/inference/region_dump.md)), and under
`--levers` also `--region-states` and `ein saturate --dump`. It saturates
nothing itself. What it adds is arithmetic over the node array: the count of
R in states (k_s) beside commitments (k_c), each state's minimal generators,
the shape the cube shows, and where the walk's `alive₀` is not the oracle's
universe.

**What it selects.** The acceptance's scope: every corpus entry that declares
a `solve` run, plus every file the wrong-answer register names (220 programs).
`--all` adds the 29 entries that declare no `solve` and reaches them through
`solve -m 1`, whose oracle is the same (249 programs). This page reports
`--all`, because it is a superset and because the entries `--all` adds hold
the largest regions in the corpus (`features/02_star_in_identifiers`,
`syntax/constraint-scopes`). The default selection's reach is in § 1.

**The first take.** An unbanked take of the same afternoon
([Q-M1e2.7 § Measured for these answers](../open_questions.md#measured-for-these-answers))
ran every `.ein` file under `examples/`, `tests/` and `corpus/`, so its counts
differ: 154 cubes against 159 here, and 14 regions with k_c ≠ k_s against 13.
This page replaces it.

---

## 1. Reach

| | `--all` | default |
|---|---:|---:|
| programs | 249 | 220 |
| region computed, n ≤ 16 | **159** | 142 |
| … monotone by the static check | 146 | 129 |
| … flagged non-monotone | 13 | 13 |
| refused by the limit | 35 | 23 |
| no region: the program does not load | 55 | 55 |

The 55 that do not load are the 52 negative fixtures (load, compile and parse)
and three `ein-bugs/` regression fixtures that fail to compile. Every program
that loads either has a region or is over the limit.

**Over the limit**, by n:

| n | programs | which |
|---|---:|---|
| 17–19 | 5 | `branching/10` and `11` (18), `lattice/06_friends_tables` and its `_everyone_sits` twin, `syntax/arg-kinds` (19) |
| 20–31 | 8 | the six `square-{fwd,bwd}` demos (20), `features/12_expect_false` (20), `branching/12` (25) |
| ≥ 32 | 22 | the zebra family (36–96), `branching/02`, `03`, `04`, `06`, `07`, `08`, the `square-unique` demos, `alt-cap-core` and its twin (70), `features/01`, `04`, `05`, `09` |

The five at 17–19 need `--region-limit` raised, and S1e2.1a.8 measured what
that costs: `branching/10` ran past 11 minutes on 8 jobs. The rest are the
region's *definition* only, and the phase claims nothing about them.

**93 of the 146 monotone cubes have no candidate at all** (n = 0). Their
lattice is one node, root, so R is {∅} or nothing. They are the saturation
fixtures and most of `tests/stdlib/`. Every table below gives them a column of
their own, because otherwise they make every count look better than the
programs that have a lattice.

## 2. R: empty or not, ∅, how many

| | monotone, n > 0 | monotone, n = 0 | flagged |
|---|---:|---:|---:|
| programs | 53 | 93 | 13 |
| R empty | 2 | 34 | 0 |
| … root dead (no alive node) | 0 | 21 | 0 |
| … every alive node owes | 2 | 13 | 0 |
| R non-empty | 51 | 59 | 13 |
| ∅ ∈ R | **41** | 59 | 11 |
| k_s = 1 | **12** | 59 | 2 |
| k_c ≠ k_s | 11 | 0 | 2 |
| one maximal node, several states | **25** | 0 | 9 |
| the lines meet (min = max) | 15 | 59 | 2 |

- **∅ ∈ R on 41 of the 51.** The other ten all owe at root: on a monotone
  program a dead root kills the whole cube, so a non-empty R without ∅ means ∅
  owes. Nine of the ten are register witnesses or `lattice/` fixtures written
  to owe (`owed-seats` and `uncovered` with their twins, `lattice/05`, `07`,
  three `synth/`); the tenth is `features/13_mixed_solution_and_open`.
- **Empty R with a candidate happens twice**, both W1's `owed-two-step` row
  (the witness and its control). Every dead root has n = 0.
- **k_s = 1 on 12 of the 51**, and **25 have one maximal node and several
  states**. That is ruling (a)'s consequence, stated in Q-M1e2.7's answer and
  now measured: on a program with restrictions only, *"is it the only
  solution?"* is a question about the maximal line, and k_s = |R| is large.
  S1e2.1a.9's *single* flag composes with *maximal* for that reason.

## 3. Shape, and the trivial lines (ruling (f))

The shape *the cube shows*: whether any node is dead (a restriction bites),
and whether any node owes (an obligation does). Monotone, n > 0:

| shape | programs | R non-empty | min = {∅} | max = {⊤} | flagged with this shape |
|---|---:|---:|---:|---:|---:|
| neither: R is the whole cube | 16 | 16 | 16 | 16 | 4 |
| restrictions only | 25 | 25 | **25** | 0 | 6 |
| obligations only | 5 | 5 | 0 | **5** | 1 |
| both | 7 | 5 | 0 | 0 | 2 |

**(f) holds on every case the corpus has**: restrictions only gives
min = {∅} on 25 of 25, and obligations only gives max = {⊤} on 5 of 5. The
16 programs where nothing bites have R = [∅ … ⊤], one sub-cube of dimension
n. `features/02` (n = 15, 32 768 commitments) and `syntax/constraint-scopes`
(n = 12) are among them.

## 4. Components and sub-cubes (ruling (i))

Non-empty R, n > 0, monotone and flagged:

| components | programs | every component a point |
|---:|---:|---:|
| 1 | 59 | 14 |
| 2 | 2 | 1 |
| 3 | 1 | 0 |
| 4 | 1 | 1 |
| 8 | 1 | 1 |

| largest sub-cube dimension | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 12 | 15 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| programs | 17 | 9 | 21 | 8 | 2 | 1 | 3 | 2 | 1 |

- **Several components only where both kinds of constraint bite**, as
  Q-M1e2.7 argued: `lattice/05` (2), `lattice/07` (8, one per seating),
  `synth/merged_states` (2), `synth/one_model_per_component` (4), and
  `synth/three_components` (3, flagged).
- **A component is not a model.** `lattice/07`'s eight components are its
  eight seatings, each a point. The user's grouping, *which table each pair
  took*, is two **projection classes** of four, which no surface computes yet
  (S1e2.1a.4).
- **17 programs are well-posed in the strict sense**: every component a point.
  14 of them have one component, so |R| = 1.

## 5. Holes, upward closure, the duality

| | monotone, n > 0 | monotone, n = 0 | flagged |
|---|---:|---:|---:|
| has a hole | 0 | 0 | **5** |
| dead not upward-closed | 0 | 0 | 4 |
| discharged not upward-closed | 0 | 0 | 2 |
| duality fails, either side | 0 | 0 | 4 |
| … without a hole | 0 | 0 | 0 |
| the local test ≠ the cube | 0 | 0 | 0 |

| program | holes | minimal holes, and why |
|---|---:|---|
| `ein-bugs/naf-upward-closure.ein` | 1 | `{(p A)}`: `(absent (q A))` |
| `ein-bugs/wrong-answers/naf-nowb.ein` | 1 | `{(p A)}`: `(absent (q A))` |
| `lattice/04_obligation_trigger_hole.ein` | 1 | `{(seats Ann C1)}`: the trigger, *meal owes Ann a Food* |
| `lattice/synth/three_components.ein` | 3 | `{(h A), (h F)}`: `(absent (h B))`; `{(h A), (h G)}`: `(absent (h D))` |
| `lattice/synth/trigger_hole.ein` | 7 | `{(h A), (h B)}`: a trigger; `{(h A), (h D)}`: `(absent (h B))`, `(absent (h E))` |

- **The static check's theorem holds on every monotone cube**: no hole, no
  violation of either upward closure, and a duality that gives both lines.
- **The duality fails only where there is a hole**, and the local test never
  disagrees with the cube in the corpus.
- **No new hole.** The stage file's note asked for a register row for any hole
  the census found. All five are known: the two W3 witnesses (register rows
  already), `lattice/04` (the obligation-trigger hole S1e2.1a.1 banked for
  P1e2.1b), and two programs S1e2.1a.8 wrote to have their holes. Whether
  today's answer on `lattice/04` is wrong depends on what R means on a
  non-monotone program, so its row waits for
  [P1e2.1b](../p1e2.1b_the_non_monotone_lattice/README.md)'s S1e2.1b.3, which
  now lists it.

## 6. Commitments against states (ruling (a) × (e))

| program | k_c | k_s | states merged | largest class | states with several minimal generators | guess depth |
|---|---:|---:|---:|---:|---:|---:|
| `features/02_star_in_identifiers.ein` | 32 768 | 728 | 688 | 64 | 168 | 9 |
| `features/03_forall.ein` | 32 | 21 | 9 | 4 | 0 | 3 |
| `lattice/synth/merged_states.ein` | 16 | 8 | 8 | 2 | 0 | 3 |
| `branching/05_mini_zebra.ein` | 8 | 5 | 1 | 4 | 1 | 2 |
| `lattice/03_state_hash_collision.ein` | 8 | 5 | 1 | 4 | 0 | 2 |
| `saturation/transitive/colocation-chain.ein`, `mealtimes.ein`, `taxonomy.ein` | 8 | 4 | 1 | 5 | 1 | 1 |
| `saturation/implies/org-chart.ein`, `parent-to-ancestor.ein`, `right-then-next.ein` | 4 | 3 | 1 | 2 | 0 | 1 |
| `branching/13_lookahead_naf_world.ein`, `14_lookahead_unjudgeable.ein` (flagged) | 4 | 3 | 1 | 2 | 0 | 1 |

*Guess depth* is, over the states of R, the largest of each state's smallest
generator: the most facts any solution needs guessed. *Several minimal
generators* is the case (e)'s `(merged (to from) …)` exists for: one state
reached from incomparable starting points.

**Both counts are needed** (the second answer): they differ on 13 regions,
and by 45× on `features/02`, where saturation derives most of what a
commitment would otherwise have to guess.

## 7. The universe (ruling (h))

The walk's `alive₀` against the oracle's config-free candidate list H, on the
194 programs whose walk was written:

| | programs |
|---|---:|
| alive₀ = H | 165 |
| alive₀ ≠ H | **29** |
| … a candidate promoted into root (forced positive) | 7 |
| … a candidate dropped before the search (lookahead, root negative) | 26 |
| … alive₀ holds a fact H does not | 0 |

- **A dropped candidate is a dead singleton** on every program whose region
  is computed: 66 of 66 (checked separately, the same day, by reading each
  dropped candidate's node in the region). On a monotone program that loses
  nothing, because every superset is dead too. On the W3 witnesses it loses
  `{(p A), (q A)}`, a node of R, because `{(p A)}` is a hole.
- **A promotion is what removes ∅.** The seven: `branching/09_hrule`,
  `domain_elim/b_branch`, `domain_elim/b_only`, `features/07_unstratifiable`,
  and three register witnesses (`forced-2` for W2, the two W3 witnesses).
- Both depend on a config switch (`:enable-pre-branch-lookahead`,
  `:enable-forced-positive`), which is (h)'s second half: the universe is
  defined without them.

## 8. Monotonicity: the static check against the cube

The detector flags **13** programs. The cube observes a violation on **5** of
them, the five with holes. On the other eight the check is conservative: the
site exists and the corpus program never exercises it (`d3-unsat-core`,
`branching/13`, `14`, `alive-empty-interlayer`, `features/07` and its
`f07-fpoff` twin, the two `stale-naf-fork` files). A monotone verdict beside an
observed violation would be a defect, and there is none.

## 9. The config principle

Every program with a region, run again under each of 24 levers, R compared on
the candidates, every node's status and state class, root's fixpoint, and the
facts of every state in R:

| lever | same R | R moved | no region |
|---|---:|---:|---:|
| `-L`, `-K`, `-o score-sum` | 159 | 0 | 0 |
| `-y` | 158 | 0 | 1 |
| `EIN_OBLIGATION_CHOICE=off`, `=fail-first`, `EIN_TRAVERSAL=tree` | 159 | 0 | 0 |
| fifteen of the seventeen `(config …)` flags | 159 | 0 | 0 |
| `lattice-sanity-check` | 158 | 0 | 1 |
| `enable-symmetric-mirror` | 158 | **1** | 0 |

- **`enable-symmetric-mirror` changes R**, on
  `features/06_symmetric_native.ein`: off, root's fixpoint loses three facts,
  `(knows Bob Ann)` the query's goal among them. The mirror is what
  `(__symmetric__ R)` means, so the flag switches off part of the program's
  theory. It is the one flag that violates the principle, and
  [`configuration.md` § 2.5](../../../docs/kernel/configuration.md#25-the-config-principle-read-against-every-row)
  names it.
- **`-y` and `lattice-sanity-check` abort `d3-unsat-core.ein`**, flagged
  non-monotone, with *"Saturation commutativity violated"*: a fork and its
  parent path reach different KBs. The probe works as designed, and which
  state a node of a non-monotone lattice is belongs to P1e2.1b's S1e2.1b.2.
- That **R** does not move is the half this stage can measure. Whether a lever
  moves what the **engine reports** is
  [S1e2.1a.6](s1e2.1a.6_engine_against_oracle.md)'s, against this R.

## 10. Route 1's reach, for S1e2.1a.9

[S1e2.1a.9](s1e2.1a.9_solve_describes_the_region.md)'s route 1 enumerates a
monotone program's alive nodes and never generates a superset of a dead one,
so a node is generated only when every one-smaller subset is alive and its
dead border is exactly N.

Over the 146 monotone cubes, alive + |N| = **49 302 of 268 137** nodes,
**18.4 %**. Counting every dead node with *some* alive parent instead gives
65 259, 24.3 % (checked separately, the same day). The first take's 32 %
counted a dead border over a different set of files, and is not comparable.

| program | n | alive | \|N\| | of 2ⁿ |
|---|---:|---:|---:|---:|
| `features/02_star_in_identifiers.ein` | 15 | 32 768 | 0 | 32 768 |
| `saturation/type-exclusivity/pets.ein` | 15 | 11 339 | 12 | 32 768 |
| `syntax/constraint-scopes.ein` | 12 | 4 096 | 0 | 4 096 |
| `lattice/07_two_pairs_two_tables.ein` | 16 | 113 | 64 | 65 536 |
| `saturation/type-exclusivity/colors.ein`, `nationalities.ein` | 8 | 169 | 4 | 256 |
| `branching/01_saturate_only.ein` | 16 | 1 | 16 | 65 536 |

The ratio is driven by the 16 programs where nothing bites, whose R is the
whole cube; where restrictions bite, route 1 is nearly free. T1e2.1a.9.1 can
re-take this table from the census's `--json` rows.

## 11. What each ruling meets in the census

The rulings came before the census (2026-10-16). The stage's job was to put
the census beside them and argue only where a ruling contradicts a measured
fact. **None does.**

| ruling | what the census shows |
|---|---|
| **general**: R is its node set | holes only where the static check flags a site (§ 5); everywhere else R is the union of its sub-cubes, which the oracle lists |
| **(a)**: `k` = \|R\| | k_s = 1 on 12 of 51; 25 have one maximal node and several states (§ 2). \|R\| is large on programs with restrictions only, as Q-M1e2.7's answer said, so *single* needs the *maximal* reading there |
| **(a)×(e)**: k_s and k_c | they differ on 13 regions, up to 45× (§ 6) |
| **(b)**: solve once, then ask | no file under `examples/`, `tests/`, `corpus/` or `stdlib/` has more than one `(query …)` outside comments (counted the same day), so no program defines two lattices today. 19 put `:hrules` on the query (counted 2026-10-16 for Q-M1e2.7, not re-taken) |
| **(e)**: `(region …)` with `(merged …)` | 13 regions have merged states; 5 have a state with several minimal generators (§ 6) |
| **(f)**: the trivial lines, monotone | 25 of 25 and 5 of 5 (§ 3) |
| **(g)**: `exhausted` means all of R | nothing to measure until S1e2.1a.9 reports R |
| **(h)**: the universe, config-free | alive₀ ≠ H on 29; every drop a dead singleton, every promotion a lost ∅ (§ 7) |
| **(i)**: components are sets of models | several components only where both kinds of constraint bite; `lattice/07`'s 8 against its 2 projection classes (§ 4) |
| **∅** | in R on 41 of 51, and every exception owes at root (§ 2) |
| **the config principle** | R moves under one flag, `enable-symmetric-mirror` (§ 9) |

## 12. What it does not say

- **Nothing past n = 16.** The zebra family, the slow entries and every
  program in § 1's lower rows have a region by definition and none by
  measurement. That is P1e2.1c's.
- **Nothing about the saturator or the generator.** The census reads the
  oracle, which saturates with the engine's code (W7) and takes its universe
  from the engine's generator (W4, W6).
- **Nothing about what the engine reports.** Missing and spurious nodes,
  per lever, are S1e2.1a.6's census, once S1e2.1a.5 has given the read-out R's
  vocabulary.
- **What R means on the 13 flagged programs.** The numbers for them are
  reported, in their own column, as the fork route computes them. P1e2.1b
  decides what they are.
