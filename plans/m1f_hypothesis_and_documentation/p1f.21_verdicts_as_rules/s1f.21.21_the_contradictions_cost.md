# S1f.21.21 — the contradiction's cost: a pair matched by a probe, an index over contexts

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-20 by the coordinator on the user's word, *"plan to build everything recommended, then measure again and get final table with 3 cols: before, after, improved after"*
([Q-M1f.657](../open_questions.md#q-m1f657--the-cost-of-the-contradiction-as-rules-what-is-built)); **not
started**.
**Waits on:** [S1f.21.20](s1f.21.20_what_the_first_uses_of_open_found.md), which writes the same files of the
saturator and whose task 7 is the first of the four things recommended.
**Writes:** `ein-infer/src/` (`saturator.rs`, `contradiction.rs`, `compile.rs`, `plan.rs`, `match_.rs`),
`ein-core/src/kb.rs`, their tests; the engine's design pages that state the indexes
(`docs/ein/ein.rs/03-design/05-matcher.md`, `03-data-model.md`, `02-algorithms.md`); the goldens' lines that list
a step's probes.

## What it is for

[The cost page](costs/README.md#why-the-contradiction-as-rules-costs) finds what the contradiction as rules added
in three parts, and [its analysis of an index over contexts](costs/README.md#an-index-over-contexts-analysed)
ends on four things recommended. The user rules all four built, and the cost measured again.

| # | recommended | built by |
|---|---|---|
| 1 | the agenda's tables kept with the snapshot, not built again at every state | [S1f.21.20](s1f.21.20_what_the_first_uses_of_open_found.md), its task 7 |
| 2 | a rule of the pair's shape matched by one probe for each new fact | task 1 |
| 3 | the relation of a nested fact a key of the indexes: (context, relation) | task 2 |
| 4 | one instance of the pair's rule a relation in place of two | task 3: **not as worded**, below |

## Tasks

1. **A pair matched by a probe.** A rule of the pair's shape has two premises, a fact $F$ and a fact holding $F$
   as its one argument, and nothing else in its match; `contradiction::Pairs` tells such a plan by its shape, and
   by no name. Today each new fact is handed to the plan through the matcher, twice: after each step by
   `match_ends`, and by the pending pass.
   - **The probe.** For a new fact $f$ of relation $R$ and a pair's plan over $R$ with wrapper $W$, of $f$'s
     count of arguments: *does the state hold $(W\ f)$?* For a new fact $(W\ g)$: *does the state hold $g$?* One
     lookup by the fact's id, with no term built through the registers: the fact store's own probe of
     $(W, [f])$ with `Kb::contains`, or a set of the facts a state holds under a wrapper, as `Kb::is_negated` is
     for one of them, whichever measures better. **Keyed by the wrapper the rule's shape names, never by a name
     the kernel knows**: the renaming invariance holds it.
   - **Queued only where the probe hits**, as the match would have queued it: the same plan, the same bindings,
     the same premises in the same order, the same key, so that what was seen stays seen and the candidate takes
     the same place. The rule fires as a rule: its firing's record, its `:why`, the explanation that reads
     through it.
   - **Such a plan is seeded by neither path**: not by `match_ends`, not by the pending pass. A plan compiled at a
     state, which is matched whole once, still is.
   - **Every other rule that ends a state** keeps the path it has: `std.core`'s over `(false)`, a program's own
     that states `:ein inconsistent`, and a rule that refutes a pair with anything more in its match.
   - **A debug check** holds the probe to the matcher on the gate's build: what the probe queues is what a seeded
     match of the same fact would queue. It is bounded, a check a fact and not a walk of the state, which is
     what V-65 found costly.
2. **An index over contexts: (context, relation).** The participation index keys a name at an argument, and one
   level in, a name at an argument of the fact there. It does not key that fact's relation.
   - **The key.** `Kb`'s indexing of a fact adds, for an argument that is a fact, the relation of that fact: a
     position of `SlotKey::inner` of its own.
   - **The matcher's use.** A nested pattern whose relation is known, a constant or a bound register, offers that
     key as a probe where no argument of it is bound, so `(W (R ?a ?b))` with nothing bound walks the facts held
     under $W$ of $R$ and not every fact of $W$. A bound argument's bucket stays the first choice: it is the
     narrower. The scan the debug build compares a probe with (`candidates_scan`) is taught the same order.
   - **The saturator's use.** `pos_index` and the index of the rules that end a state are keyed by the premise's
     relation and, where the premise holds a nested pattern of a known relation, by that relation too. A new
     fact $(W\ g)$ is handed to the plans keyed by $(W, R_g)$ and to those whose premise leaves the nested
     relation open, in the plans' order, which is the order today's list gives the same plans.
   - Every source yields facts in the extent's order, so the choice of a source changes how many are offered and
     not the order of the matches.
3. **The instance of no argument (recommended as *one instance a relation in place of two*).** What the
   recommendation did not say: the second instance is no accident of the library. Every declaration stores the
   membership fact `(relation R)` beside `(relation R LABEL…)`, which the syntax reference states, and
   `clash-of-0` matches it, so a relation declared with labels has the pair's rule for its fact of no argument
   too. Taking that instance away is one of: the membership fact not stored (the language changes); `clash-of-0`
   guarded by an `absent` over the declarations of every other count (the library gains an `absent` over
   `relation`, and a program whose rule concludes a declaration is flagged); or the instance not activated, and
   `(R)` beside `(not (R))` no contradiction for a relation declared with labels (what a program means changes).
   **Default taken (Q-M1f.658): the instance stays, and is made free.** After task 1 it is matched by a compare
   of the fact's count of arguments with the plan's and no probe, and after S1f.21.20's task 7 it is one row of
   a table built once. The task: measure what it still costs, by the same programs with `clash-of-0` taken out
   of a copy of the library, and record the figure with the three ways above for the user.
4. **Measured.** Instructions executed, user space, on the five programs of the cost page, against A and D; the
   page's script on its nineteen entries with the built engine a point of its own; the gate's build of
   `equation_two_unknowns.ein` against release. Under `utils/bench_env.sh --cores PT`, with the load said.

## What must hold

- **Every corpus answer as before, and the order of every firing.** No core, no trace, no explanation, no
  digest of a derivation moves. `ein-infer/tests/agenda.rs` holds the agenda,
  `ein-infer/tests/contradiction_rules.rs` the firing of the pair's rules.
- **What a golden may show**: a step's list of probes, where a dump prints it, gains the nested relation's key.
  Nothing else. The goldens are banked only after a diff that shows those lines alone.
- **The kernel reads no theory's name** (Q-M1f.436): the probe and the index are of a wrapper a rule's shape
  names. `ein-render/tests/renaming_invariance.rs`.
- No config option, lever or flag changes $R$; a flagged program is claimed nothing of.
- The gate on the last commit, and the two slow render sweeps, since the engine moves.

## After it: the coordinator's part

**The final table**, the user's word: three columns, *before*, *after*, *improved after*.

- *before*: `edbe6c3b`, the engine before the contradiction became rules.
- *after*: `1655f19d`, the engine after the phase's builds and before any of the four.
- *improved after*: the engine with S1f.21.20 and this stage in it.

On the cost page's nineteen entries in milliseconds, each engine on the corpus of its own commit, and on its
five programs in instructions executed; on an idle machine, on AC, no agent running. It goes on
[the cost page](costs/README.md). Then the corpus's costs are taken again and the manifest's flags and recorded
costs move once.

## Record

Run 2026-11-20 in a worktree fast-forwarded to `master` at `0a6e6e3b`, then rebased onto `master` at its tip. The
machine was shared the whole time (another stage building, load average 14 to 24): every figure below that is a
time says its load; the instruction counts do not move with it. Scratch named `build/s21/…` is the worktree's.

### Commits

| commit | task | what |
|---|---|---|
| `d28618c3` | 1, 2 | a pair matched by a probe; the nested fact's relation a key of the participation index and of the plans a fact seeds; the scan for a never-matched plan from a watermark |
| `ddce3113` | 1 | `Kb::is_negated` / `Kb::negated` and their set go, with the shape dump's `NEG` rows (V-36); 52 `load` lines of the shape golden banked again |
| `4d05d590` | | an engine's plan list shared with its clones until one compiles a plan |
| this record | | plans only |

### 1. A pair matched by a probe

**Built.** `contradiction::PairShape { wrapper, rel, count, held }`, read by `Pairs::of` off a plan that refutes a
pair and whose match is that pair and nothing else, one disjunct (`Pairs::shape_at`); by shape, no name.
`PairShape::each_match` finds, for a new fact, the premises of each match it takes part in, in step order: at the
held step a fact of `rel` and `count` arguments whose `(W f)` the fact store's own probe finds (`FactStore::probe`,
then `Kb::contains`), at the other a `(W g)` with `g` of `rel` and `count` and held. A fact of another relation or
count is turned away by a compare. The saturator hands a new fact to such a plan through `Saturator::offer` →
`pair_match` in place of `seed_match`, after a step (`match_ends`) and in the pass alike, at the plan's place in
the plans' order; only a hit takes the plan and the matcher, and `Matcher::run_pair` builds the match a seeded
match emits (the seed's registers, the held fact's variables in slot order, the premises in step order), which
`enqueue_binding` keys, dedups and queues as any. A plan compiled at a state is matched whole once, as every plan.
`std.core`'s rule over `(false)`, a rule that refutes a pair with anything more in its match, and one whose `(or …)`
holds a pair in one of several disjuncts keep the seeded path.

**The debug check**: on a build with debug assertions (the gate's), `pair_match` runs `run_seeded` of the same plan
with the same fact and asserts the probe queued exactly its matches (disjunct, registers, bindings' order,
premises), on every fact offered, a hit or not. One seeded match a fact and a pair plan: what the release path did
before, and no walk of the state.

**The probe, decided by measurement** (Q-M1f.659). Instructions, one `ein solve --jobs 1` pinned to the P-core
threads, the build with the watermarks below:

| program | the fact store's probe | the set of a state's negations | difference |
|---|---:|---:|---:|
| `beyond_arith/equation_two_unknowns.ein` | 3 549 M | 3 535 M | 0.4 % |
| `saturation/type-exclusivity/pets.ein` | 3 610 M | 3 597 M | 0.4 % |
| `features/01_not_and_absent.ein` | 24 482 M | 24 431 M | 0.2 % |
| `beyond_arith/send_more_money.ein` | 11 836 M | 11 824 M | 0.1 % |
| `zebra2.ein` | 1 124 M | 1 124 M | 0.0 % |

The set measured is the cheapest form of one: the existing `negated` bit set, specific to the kernel's `not`; one
by wrapper relation adds a map lookup a layer to each probe and an upkeep for every fact of one argument that holds
a fact. The upkeep of the existing set measured nothing (24 476 M against 24 482 M on `01_not_and_absent.ein`). So
the fact store's probe, which reads no name, and `Kb::is_negated`, `Kb::negated`, the per-layer set and the dump's
`NEG` rows go (V-36): the two measurement examples and `from_ir`'s unit test ask the fact store. **The shape golden
moved on exactly the 52 programs that hold a stored negation at load, each on its `load` line alone, each text
shorter by 1 to 11 lines** (its `NEG` rows); no `plan`, `match`, `saturate`, `naf` or other line moved.

### 2. An index over contexts

**Built.** `SlotKey::RELATION` (`u16::MAX − 1`) and `SlotKey::relation(rel, slot, of)`: `Kb::index_fact` and
`rebuild_layer`, through one function now (`participation_keys`), post a fact holding a fact at `slot` under the
nested fact's relation as well. The compiler lists, last in a step's probes, the relation of its first nested
pattern, a constant, where no constant came before it (top level or inside the nested fact): a bound argument's
bucket stays the first choice. `candidates_scan` returns the same key where no slot is usable. The saturator's
`pos_index` and the ends' index are one structure, `PlanIndex`: relation → the plans with a premise of it that
leaves the nested relation open, and `(relation, slot, RELATION, nested relation)` → the plans every premise of that
relation of which holds a nested fact there; `plans_for` merges the lists of a fact in the plans' order, each plan
once, with one map lookup where a relation has no keyed plan. Both are behind an `Arc`, so a snapshot and every
state resumed from it share them, where each copied a map of vectors before. The shape dump prints the `DIRECT`
keys only, and no dump prints a step's probes, so no golden line moved for this task.

Tests: `kb.rs`' participation test reads the relation's key; `compile.rs`' probe-list tests (the relation last, a
constant ending the list); `match_.rs`' `narrowing_never_changes_the_match_sequence` with negations of a second
relation beside the first, through both indexings; `saturator.rs`' `a_fact_holding_a_fact_seeds_the_plans_of_its_relation`;
`contradiction.rs`' three tests of the shape and the probe, a wrapper of its own relation among them.

### Also built: what a plan costs at every state (Q-M1f.662)

Profiles of the build of tasks 1 and 2 put two loops over every plan first in `enqueue_pass` and `end_step`: the
search for a plan never matched, at every pass and at every step of a stopped saturation, over 338 plans on
`equation_two_unknowns.ein`. It starts at a watermark below which every plan is matched (`matched_upto`,
`ends_matched`), 0 where a saturation is built. And a resumed state cloned its engine's plan list, keys and
activator lists, an allocation or two a plan, and dropped them: the list is one value behind an `Arc`, copied
where an engine compiles a plan (`Arc::make_mut`). Both are costs in the count of plans, which the contradiction's
instances doubled.

### No firing moved

`build/s21/diffbins.sh`: for every `.ein` under `examples/`, `tests/`, `stdlib/` and `corpus/` (942 files), `ein solve
FILE -p` (stdout, stderr, exit code) and the md5 of `ein solve FILE --events E`, whose `enqueue` lines carry every
candidate's tiebreaker and whose `fire` lines every firing in order: **byte for byte the same** as `0a6e6e3b`'s on
the build of tasks 1 and 2, on the one with the watermarks, on the final one, and on the gate profile's build of
tasks 1 and 2, whose debug check of every probe never fired. `cargo test --profile gate -p ein-core -p ein-infer`:
652 tests, none failed; the shape golden as above.

### 3. The instance of no argument (Q-M1f.658)

**Not removed.** With task 1 a fact of another count than the plan's is turned away by a compare and no probe, and
since S1f.21.20 the instance is a row of tables read once. Measured on the final build by the same five programs
with `clash-of-0` taken out of a copy of the library (`build/s21/stdlib-no-clash0`, `$EIN_STDLIB` naming it),
instructions, user space, pinned, `--jobs 1`:

| program | root's plans, with and without | with | without | the instances cost |
|---|---:|---:|---:|---:|
| `beyond_arith/equation_two_unknowns.ein` | 338, 244 | 3 404 M | 3 113 M | 291 M, 8.6 % |
| `saturation/type-exclusivity/pets.ein` | 13, 10 | 3 530 M | 3 400 M | 130 M, 3.7 % |
| `features/01_not_and_absent.ein` | 14, 11 | 23 991 M | 23 300 M | 691 M, 2.9 % |
| `beyond_arith/send_more_money.ein` | 63, 44 | 11 511 M | 11 305 M | 206 M, 1.8 % |
| `zebra2.ein` | 160, 150 | 1 117 M | 1 110 M | 6 M, 0.5 % |

So the instance is not free: what it costs now is not its matching but its being a plan, at every state (V-79):
the tables and the index read again where a state compiles a plan, the activator and the firing at root that each
state's `fired` and `seen` copy. Without `clash-of-0` (`build/s21/answers.py`, every corpus file's
`--json-summary` under both libraries) **no corpus answer moves** — R, the verdict, every claim and query — but
the plan counts the monotonicity block reports, and the two programs that ask about `clash-0`'s activators
themselves (`tests/stdlib/contradiction/09_clash_of_declared.ein`, `10_undeclared_not_reached.ein`) no longer load,
their goal naming a relation nothing writes.

**The three ways to take it away**, for the user:

1. **The membership fact not stored.** A declaration with labels stores only `(relation R LABEL…)`; `clash-of-0`
   then activates `clash-0` for a relation declared bare alone. The language changes: `02-syntax.md` documents
   `(relation R)` as the pattern *is `?R` a declared relation, at any count*, and every program or module that
   matches `(relation ?R)` stops seeing relations declared with labels (in the library, `clash-of-0` alone reads it
   today). The KB holds one fact less a declared relation.
2. **`clash-of-0` guarded by `absent` over the declarations of every other count**: `(and (relation ?R) (absent
   (relation ?R ?A)) … (absent (relation ?R ?A ?B ?C ?D ?E ?F)))`. Nothing in the language changes, but the
   library gains six `absent`s over `relation`, judged at root's boundary; in a program whose rule concludes a
   declaration, a refutation of no argument would rest on such an `absent`, which the static check reads as it
   reads any (not measured here); and a relation declared both bare and with labels loses its `clash-0`.
3. **The instance not activated for a relation declared with labels**, by any means: then `(R)` beside `(not (R))`
   is no contradiction for such a relation, where it is today. What a program means changes, for a fact of no
   argument of a relation declared with labels, which loads in silence (Q-M1f.447); no corpus program holds one
   beside its negation, by the comparison above.

### 4. Measured

**Instructions executed, user space**: `perf stat -x, -e instructions:u <ein> solve FILE --jobs 1`, pinned with
`taskset -c 0-15` (the P-core threads; the E-core counter then counts nothing), each engine on the corpus and the
standard library of its own commit (`git archive` of each into `build/s21/tree-<commit>`, `$EIN_STDLIB` naming its
`stdlib/`); three runs of the final build agree to within 0.01 %, one run each of the others. The script is
`build/s21/instr.sh`. Millions:

| program | A `edbe6c3b` | B `ddf09492` | C `08ed0ce7` | D `1655f19d` | before, `0a6e6e3b` | after, `4d05d590` | after / before | after / A |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `beyond_arith/equation_two_unknowns.ein` | 3 143 | 4 847 | 5 011 | 5 036 | 4 221 | 3 404 | 0.81 | 1.08 |
| `saturation/type-exclusivity/pets.ein` | 2 629 | 3 562 | 4 411 | 4 432 | 3 808 | 3 530 | 0.93 | 1.34 |
| `features/01_not_and_absent.ein` | 17 118 | 22 257 | 29 326 | 29 391 | 25 297 | 23 991 | 0.95 | 1.40 |
| `beyond_arith/send_more_money.ein` | 10 665 | 13 734 | 19 697 | 19 841 | 12 208 | 11 511 | 0.94 | 1.08 |
| `zebra2.ein` | 1 117 | 1 250 | 1 341 | 1 342 | 1 245 | 1 117 | 0.90 | 1.00 |

Each step of this stage, the same way (one run each; `0a6e6e3b` first):

| program | before | tasks 1, 2 | + the watermark | + the shared plan list, the set gone: after |
|---|---:|---:|---:|---:|
| `equation_two_unknowns.ein` | 4 221 | 3 980 | 3 549 | 3 404 |
| `pets.ein` | 3 808 | 3 613 | 3 610 | 3 530 |
| `01_not_and_absent.ein` | 25 297 | 24 481 | 24 482 | 23 991 |
| `send_more_money.ein` | 12 208 | 11 882 | 11 836 | 11 511 |
| `zebra2.ein` | 1 245 | 1 134 | 1 124 | 1 117 |

The contradiction as rules added, from A to B, 1.7 G on `equation_two_unknowns.ein`, 0.9 G on `pets.ein`, 5.1 G on
`01_not_and_absent.ein`, 3.1 G on `send_more_money.ein` and 0.13 G on `zebra2.ein`; B to C added the `open` facts,
which S1f.21.20 took mostly back. After this stage `zebra2.ein` executes what A did, and `equation_two_unknowns.ein`
and `send_more_money.ein` 8 % more; on `pets.ein` and `01_not_and_absent.ein` the rest is mostly not the
contradiction's (the read-out and the pass that writes `open` facts lead their profiles), and § 3's figure is
what the instances of no argument still cost.

**The cost page's script** (`four_engines.py` copied to `build/s21/`, its points A, D, `0a6e6e3b` and this stage's
build, each on its own tree), `utils/bench_env.sh --cores PT`, on AC, **load average 14.0, 21.9, 21.1 at the
start and 15.9, 17.7, 18.5 at the end**: another stage was building, so these times are not the final table's,
and on the long entries A itself reads up to 1.6 times its page figure (`zebra2-minus-15.ein` 16 977 ms against
10 985). Milliseconds, each entry's declared runs summed; *work* is the same saturations at every point as on the
page:

| entry | A | D | `0a6e6e3b` | after | after / `0a6e6e3b` | after / A |
|---|---:|---:|---:|---:|---:|---:|
| `zebra.ein` | 370 | 892 | 907 | 771 | 0.85× | 2.08× |
| `features/04_open.ein` | 562 | 1651 | 652 | 720 | 1.10× | 1.28× |
| `beyond_arith/equation_two_unknowns.ein` | 638 | 842 | 778 | 706 | 0.91× | 1.11× |
| `beyond_arith/send_more_money.ein` | 2963 | 6053 | 3720 | 3784 | 1.02× | 1.28× |
| `beyond_arith/send_more_money_written.ein` | 2597 | 5373 | 2021 | 1511 | 0.75× | 0.58× |
| `features/01_not_and_absent.ein` | 2458 | 3969 | 3264 | 3016 | 0.92× | 1.23× |
| `lattice/06_friends_tables.ein` | 4911 | 7318 | 5232 | 5034 | 0.96× | 1.02× |
| `lattice/07_two_pairs_two_tables.ein` | 4171 | 5102 | 6699 | 4681 | 0.70× | 1.12× |
| `lattice/14_seats_nothing_proposes.ein` | 246 | 5534 | 5860 | 4334 | 0.74× | 17.65× |
| `zebra2.ein` | 1833 | 2172 | 2426 | 2504 | 1.03× | 1.37× |
| `zebra2-obligations.ein` | 2648 | 2152 | 1807 | 1688 | 0.93× | 0.64× |
| `zebra2-hints.ein` | 491 | 504 | 506 | 497 | 0.98× | 1.01× |
| `saturation/square-fwd/meetings.ein` | 1992 | 2152 | 1622 | 1586 | 0.98× | 0.80× |
| `saturation/square-fwd/houses.ein` | 1610 | 1851 | 1576 | 1559 | 0.99× | 0.97× |
| `branching/06_lookahead_on.ein` | 29 | 42 | 34 | 33 | 0.99× | 1.14× |
| `branching/07_lookahead_off.ein` | 29 | 41 | 35 | 34 | 0.96× | 1.16× |
| `arith/rat.ein` | 656 | 852 | 1133 | 1037 | 0.92× | 1.58× |
| `saturation/type-exclusivity/pets.ein` | 653 | 1248 | 941 | 963 | 1.02× | 1.48× |
| `zebra2-minus-15.ein` | 16977 | 19004 | 17346 | 11821 | 0.68× | 0.70× |

Its rows are `build/s21/four_engines.json`. Under this load a ratio below about 1.3 says nothing either way; the
instruction counts above are the stage's evidence.

**The gate's build against release**, `equation_two_unknowns.ein`, one `ein solve --jobs 1` pinned to one P-core
thread (`taskset -c 2`), five runs each, task-clock, load average 15.6 to 14.9 over the runs; the gate build of
`0a6e6e3b` built from its own tree:

| build | `0a6e6e3b` | after | instructions, `0a6e6e3b` → after |
|---|---:|---:|---:|
| release | 315 to 334 ms | 252 to 254 ms | 4 221 M → 3 404 M |
| gate | 441 to 448 ms | 415 to 421 ms | 7 084 M → 6 585 M |
| gate / release | 1.39× | 1.66× | |

The gate build is faster than before; its ratio to release grows because its check runs a seeded match for every
fact a pair's plan is offered, what the release path did before.

### Defaults taken (Q-M1f.658 to Q-M1f.663)

| id | question | default | the other reading |
|---|---|---|---|
| Q-M1f.658 | `clash-0`'s instance for a relation declared with labels | stays, made free; measured, the three ways shown (§ 3) | one of the three ways |
| Q-M1f.659 | how a pair's probe asks whether the state holds the other side | the fact store's own probe of `(W, [f])`, then `Kb::contains`; `Kb::is_negated`, its set and the dump's `NEG` rows go | a set of the facts a state holds under a wrapper, by wrapper relation: measured 0.1 to 0.4 % cheaper in the form specific to `not`, which reads a kernel name |
| Q-M1f.660 | which plans take the probe | a plan whose whole match is one pair, one disjunct of two premises, and whose firing ends a state, by stating the key or concluding what a rule that does matches alone: `clash-N` and a program's own pair rule either way; a rule that states `:ein inconsistent` over anything but a pair, `std.core`'s, and a pair inside one of several disjuncts stay seeded | probe the pair's disjunct of a plan of several and seed the rest |
| Q-M1f.661 | the nested relation's key | `(context, argument, RELATION, relation)`, the argument part of the key; a plan keyed by its premise's first nested pattern; the probe last, after every argument that can be bound, and not listed after a constant | `(context, relation)` without the argument; or the relation's key before a later argument |
| Q-M1f.662 | the costs of a plan at a state beside the two tasks | built: the watermark for the plan never matched, and the shared plan list | leave them for a stage of their own |
| Q-M1f.663 | a position inside a nested fact that is `RELATION`'s number (the last of a fact of 65 535 arguments) | not keyed, by the scan, the compiled list and the index alike (`SlotKey::keys_inner`), so a position's key and a relation's never share a bucket | a field of its own in `SlotKey`, past its 12 bytes |

### Findings outside the stage (V-78 to V-80)

- **V-78 — a resumed state walks every fact it holds to find its delta.** `Snapshot::new_facts_of`, which
  `region::saturated` asks at every node, walks the node's whole fact list across its layers and asks the
  snapshot's set of root's facts for each. A call-graph profile (`perf record --call-graph lbr`, one worker) puts it
  at about 17 % of `send_more_money.ein`'s instructions (8.8 % the walk of the layers, 8.2 % the collection) and
  about 5 % of `equation_two_unknowns.ein`'s, on the build of tasks 1 and 2 with the watermark. No part of the contradiction's cost, and no file
  of this stage's beyond the function itself; a delta read off the layers above the snapshot's would make it the
  size of the delta.
- **V-79 — what a plan still costs at a state.** Where a state compiles a plan (the calculator's derived
  activators), it reads every agenda table, the plan index and `Pairs` again over every plan (`refresh_first`,
  `FirstTables::of`, `Pairs::of` twice, `PlanIndex::of`): about 5 % of `equation_two_unknowns.ein`, in the profile
  of V-78. Every state also
  copies root's `fired` and `seen` sets, an allocation a `BindingKey`: about 1.3 % each on that program and on
  `send_more_money.ein`. These, with the plans' own firings and activators at root, are what § 3's figure is made
  of: a cost in the count of plans that this stage did not make free.
- **V-80 — the cost page's instruction counts and these are two conditions.** The page's A to D counts were taken
  unpinned at the default job count; pinned to the P-core threads with `--jobs 1`, A executes 3 143 M on
  `equation_two_unknowns.ein` against the page's 3 610 M, and B 4 847 M against 6 670 M. A final table that sets
  the page's counts beside counts taken the other way compares two machines; the counts in § 4 are all taken one
  way, each engine on the corpus and library of its own commit.

### The gate

GATE

### What the user must see

USERSEE
