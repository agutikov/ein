# P1f.10 — what the phase found outside its stages

**Phase:** [P1f.10](README.md). Opened 2026-11-15.

What a stage of this phase finds outside its own scope: kept in the phase and shown to the user at its stop,
not fixed by the stage that found it. A wrong answer among them is a row of
[`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml) before anything fixes it; none below is one.

Track H's stages keep theirs under a heading each, below the table: H9-n, H6-n, H15-n.

## From the stages of tracks S and R

| id | what | witness, and the command | found by | today | state |
|---|---|---|---|---|---|
| K-1 | **What *a program searches* means is written twice.** `corpus_ops::searches` is in `ein-render`'s test tree, which no other crate can read, so the pair layer's census, which T1f.10.1.3 scopes by it, restates its budget, its cube limit and its call in the example. Nothing fails when one copy moves | `grep -nE 'SWEEP_(BUDGET\|LIMIT)' ein.rs/crates/ein-render/tests/corpus_ops/mod.rs ein.rs/crates/ein-infer/examples/pair_layer.rs` | [S1f.10.1](s1f.10.1_exclusion_census.md), 2026-11-15 | one definition: `ein-render/tests/corpus_ops/searches.rs` holds `SWEEP_BUDGET`, `SWEEP_LIMIT` and `searches`, `corpus_ops` declares it as its own module, and `ein-infer/examples/pair_layer.rs` and `ein-infer/tests/structure_invariance.rs` include the same file by `#[path]` (`grep -rn 'corpus_ops/searches.rs' ein.rs/crates`, 2026-11-15). 188 of the 625 programs the census reads search | **closed** by [S1f.10.2](s1f.10.2_groups.md#record-2026-11-15-the-cover-under-the-theorys-exclusions), 2026-11-15, with K-4's sharing: not `ein-corpus`, whose library depends on no engine crate, but the file itself, included by each binary that reads it. A copy that moved now moves all three |
| K-2 | **The slow half of `ein-infer/tests/pair_layer.rs` is in no tier.** Per commit the test reads every corpus program but the `slow` entries, and three instances named in it whatever their flag (`zebra.ein`, `zebra2.ein`, `lattice/02_genuine_3set_death.ein`). Under `EIN_CORPUS_SLOW=1` it reads the rest, and two more programs reach the comparison, `zebra2-obligations.ein` and `lattice/07_two_pairs_two_tables.ein`; the other slow entries' describes do not exhaust, so nothing is compared on them | `EIN_CORPUS_SLOW=1 cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test pair_layer -- --nocapture`: 609 programs compared in 19.4 s, against 607 in 4.1 s without it (2026-11-15, 16 workers, on AC) | S1f.10.1, 2026-11-15 | a nightly step, and the dev host's: the one test that reads the flag, by name, 609 programs in 41.7 s on four P-core threads with four workers, on AC (`EIN_CORPUS_SLOW=1 EIN_TEST_JOBS=4 utils/bench_env.sh --cores PT:4 --require-ac cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test pair_layer -- the_excluded_pairs_are_the_pairs_of_n --nocapture`, 2026-11-15) | **closed by a default**, the coordinator's of 2026-11-15 and the user's to revise ([S1f.10.2](s1f.10.2_groups.md#defaults-taken), 2): a nightly step, as `obligation_rung`'s and `region`'s slow halves are. *By hand* was not open: `ein-cli/tests/nightly_steps.rs`'s `every_slow_half_runs_on_a_tier` fails on a target that reads `EIN_CORPUS_SLOW` and runs on no tier, and failed at `706f9b47` (`cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-cli --test nightly_steps`, 2026-11-15) |
| K-3 | **`utils/README.md` counts its measurement set wrongly.** § The M1a measurement set opens *Eleven scripts*; its table had nine rows before S1f.10.1 and has ten with `pair_layer` | `sed -n '/^## The M1a measurement set/,/^Results/p' utils/README.md \| grep -c '^\| \['` | S1f.10.1, 2026-11-15 | the sentence stands | open. P1f.20 sweeps the page |
| K-4 | **The pair layer's two invariances of the phase's acceptance are tests nowhere**: *the structure of a program is the same under every import form and under a permutation of the interner's ids*. `ein-infer/tests/pair_layer.rs` holds the layer to the job count alone. Each needs a helper that is a private function of another crate's test binary: `permuted`, the `Terms` whose id space is a shuffle of a finished run's (`ein-render/tests/id_order_invariance.rs`; it uses `ein_core` and `ein_infer::mt19937` only), and `form`, a program rewritten with its imports whole or `:as` (`ein-cli/tests/import_invariance.rs`, with `strip` and `module_prefixes`; they use `ein_ir` and `ein_corpus`). `ein-corpus`, the home K-1 names, serves neither: its library depends on no engine crate, by the comment in its `Cargo.toml` | `grep -n 'fn permuted\|fn form\|fn strip\|fn module_prefixes' ein.rs/crates/ein-render/tests/id_order_invariance.rs ein.rs/crates/ein-cli/tests/import_invariance.rs`; `grep -n -A3 '^\[dependencies\]' ein.rs/crates/ein-corpus/Cargo.toml` | [S1f.10.2](s1f.10.2_groups.md), 2026-11-15, which read the helpers and stopped at its control before building either test | both are tests, of root's pair layer, the theory's share and the cover together: `ein-infer/tests/structure_invariance.rs`, over every corpus program that searches, 175 per commit and all 188 under `EIN_CORPUS_SLOW=1`, 47 and 53 of them importing; no program moves under either (`cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test structure_invariance -- --nocapture`, with and without the flag, 2026-11-15) | **closed** by [S1f.10.2](s1f.10.2_groups.md#record-2026-11-15-the-cover-under-the-theorys-exclusions), 2026-11-15, built as read: `permuted` and `moved` moved as they were to `ein-render/tests/id_permutation/mod.rs`, and `form`, `strip` and `module_prefixes` to `ein-cli/tests/import_forms/mod.rs` with the scratch directory of links cut out of `import_invariance.rs`'s `check` beside them (`beside_its_modules`); each is a module of its own test binary and is included by `#[path]` from `structure_invariance.rs`. The id comparison is of every list as it is, the work and the enumeration's steps among them; the import comparison of every list as a set of texts with the module prefixes and aliases taken off. Its slow half is a nightly step and the dev host's, as K-2's is |
| K-5 | **No loaded fact says which file wrote it.** A fact's record carries a place, `Prov::loc`, and it is `None` for every fact a file writes: the frontend builds a top-level form without a position ([`defined_behaviour.md` § 1.5](../../../docs/kernel/defined_behaviour.md#15-loader-messages-about-a-top-level-form-end-in-at-none), `ein-core/src/entities.rs`'s `Loc`). So nothing in the engine can tell a fact an imported module states from one the program's own file states. Two things in S1f.10.10 follow from it: the small KB's rule cannot keep a module's table, and drops `std.math.digits`' rows over the digits a letter may be with the clues; and `pair_layer` writes `theory.ein` by matching each top-level form's text to a dropped fact's, and cannot delete a fact a module writes | `grep -n 'almost every one of these' ein.rs/crates/ein-core/src/entities.rs`; the count of programs it touches is `theory.ein: … drops a module's fact` in [S1f.10.10's headline](s1f.10.10_the_theorys_share.md#the-commands) | [S1f.10.10](s1f.10.10_the_theorys_share.md), 2026-11-15 | 16 of the 625 programs read drop a fact a module writes; on 11 the program that keeps the module's facts has the small KB's layer, and on 5, `binary_logic`'s over `full_adder.ein`, it has not | open. A fact that knew its file would let the rule say *a module's facts are its theory's*, which is not obviously right either: those five programs keep their puzzle in a module |
| K-6 | **Nothing records what an `absent` rests on, so a core or a support put back alone is another program.** A firing that passed an `absent` records the query, `Prov::absent`, and no walk follows it (`ein-core/src/prov.rs`: *recorded, not yet interpreted*). A core, and S1f.10.10's support, name the facts a derivation matched. The absence of a fact rests on the whole of what the program states of that relation, and a program over the named facts alone states part of it: `zebra2.ein` with three of its four `right-of` facts deleted has a dead root, its rules concluding from the houses that are missing, so a program over fewer statements derives `(false)` where the program does not. A check over the small KB and a support's statements alone is such a program. S1f.10.10's check therefore puts back every dropped statement of a relation a rule reads the absence of, at relation grain, which on `zebra.ein` brings four clues with the row | `grep -v '(right-of House-[235] ' examples/zebra2.ein > build/k6.ein && ein solve --no-search build/k6.ein` prints *∅ dead — root's own fixpoint derives (false)* (2026-11-15); `structure::Stated::read_absent` is what the check puts back, and `jq '.theory.supports.check' build/structure/census.jsonl` has `put_back_in_every_check` per program, 4 on `zebra2.ein` and 8 on `zebra.ein` | S1f.10.10, 2026-11-15 | every one of the corpus's 5 565 checks is dead with its root alive and its node inside the program's, with the statements put back | open. It bears on whoever reads a core as a program: [F22](../../followups/f22_exact_core_past_the_cap.md)'s exact core, and [S1f.10.13](s1f.10.13_the_price_list.md) if it prices a nogood learned from a death. A fact-grain reading, the facts a guard's `absent` could have matched, is what would keep `zebra.ein`'s clues out of its checks |
| K-7 | **A support costs 25 ms a pair where a core costs a few**, on `zebra2.ein`: 327 supports are 8 of the 10 s the example takes at `--check`. `explain.rs`'s search builds the AND/OR graph of the pair's contradiction from nothing each time, and nearly all of it is root's own derivation, the same for every pair; with root's activators followed it is also wider. A label for root's facts computed once would serve every pair of one program | the walls' table in [S1f.10.10 § What it costs](s1f.10.10_the_theorys_share.md#what-it-costs) | S1f.10.10, 2026-11-15 | I-Z2M's 1 428 supports take 26 s of its 35 | open. [S1f.10.13](s1f.10.13_the_price_list.md) prices the support as one way of saying why a pair is dead; nothing in track S needs it faster |
| K-8 | **The corpus's import forms respell a candidate and hardly ever reorder a universe.** A candidate's text changes with the import form only where the program proposes a fact of an imported relation, and the universe's order only where it also proposes facts of another relation that the new spelling crosses. Over the 53 corpus programs that search and import, each in its two other forms: a candidate is written otherwise in 22 of the 106 forms, and the universe's order is another in 2, no program with two groups or more among them. So a sweep of the corpus under the import forms holds a structure to the *spelling* of a name and says next to nothing of one that follows the universe's *order*: a cover cut to its first three groups in that order passes it (tried and reverted) | `EIN_CORPUS_SLOW=1 cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test structure_invariance -- --nocapture the_structure_is_the_same_under_every_import_form` prints the two counts (2026-11-15) | [S1f.10.2](s1f.10.2_groups.md#the-two-invariances), 2026-11-15 | `structure_invariance.rs` writes one program of its own that does reorder, `a_form_that_reorders_the_universe_moves_no_set`, a module's relation proposed beside the program's own, and holds its four groups to the three forms; the cut cover fails there | open, for the corpus: no corpus program with groups is reordered by a form. It bears on `ein-cli/tests/import_invariance.rs` the same way: an answer that followed the universe's order would show there only on a program whose order a form moves, and the corpus has those two forms. A corpus program that proposes facts of an imported relation beside its own, with groups, would give both sweeps the case |
| K-9 | **Under the blind enumerator a cover can be all of candidates nobody would propose.** On six searching programs every group's every member is dead alone at root, so *at most one* is vacuous there: `branching/03_five_hyps_one_alive.ein`, `06_lookahead_on.ein` and `07_lookahead_off.ein` (ten groups of nine each, an object's nine `is-a` candidates, `(is-a Blue Green)` with `(is-a Blue H1)`), `01_saturate_only.ein`, `02_one_dead_one_alive.ein` and `04_two_levels.ein`. The candidates those programs are about are in no group: on `06_lookahead_on.ein` the 42 `co-located` candidates, the two alive alone among them, since the small KB drops the `is-a` facts that tell a colour from a house (S1f.10.10's clause (a), where `is-a` is a candidates' relation because the blind enumerator proposes facts of it). The groups are true and say nothing of the search | `jq -r 'select(has("error") \| not) \| select(.searches) \| .theory.cover as $c \| select($c.groups > 0 and ($c.open \| all(.[0] == 0))) \| "\(.path) \($c.groups) \(.theory.rung)"' build/structure/census.jsonl` (2026-11-15) | S1f.10.2, 2026-11-15 | 79 of the corpus's 496 groups are on the ten programs with a group that the blind enumerator built, 55 groups have no open member | open, and not this stage's: what the blind enumerator takes for an object is [Q-M1f.306](../open_questions.md#q-m1f306--what-the-blind-enumerator-takes-for-an-object), and a ruling that takes a type's name out of the objects re-takes these covers by the census's command. It is the class [S1f.10.10](s1f.10.10_the_theorys_share.md#the-rule-as-the-corpus-left-it) says its rule does not fit, seen from the cover |
| K-10 | **L5's reason for an instance it does not license depends on what the interner holds, not on the program.** `oblgen::witnesses` says *"a `R` fact can witness it that no candidate proposes"* where nothing has interned the witness, and *"`(R a b)` can witness it and is not a candidate"* where something has. A node interns what it derives, so one program read before its first node and after it gives the two sentences, and so does one program loaded into two id spaces one of which holds the facts of a finished run. Whether the instance is licensed does not move, and no route reads the sentence: `tree.rs` asks only whether there are alternatives | `structure::Owed` carrying L5's words as they come fails `the_structure_is_the_same_under_a_permutation_of_the_ids` on `examples/zebra.ein` (18 instances of `std.slots.slot-owed-fill`) and `examples/features/13_mixed_solution_and_open.ein` (1): tried and reverted, 2026-11-15. And `jq -r '.theory.cover.owed.unlicensed_why' build/structure/s4/census_before.jsonl`, the table read before the nodes, against the same read after them, `"… can witness it and is not a candidate": 18` on `zebra.ein` | [S1f.10.4](s1f.10.4_bounded_groups.md), 2026-11-15 | the owner states one reason for both, `structure::UNPROPOSED`, from the witnesses it reads itself, and reads the table before any node as a route does; `oblgen.rs` is as it was. `grep -rn 'can witness it' ein.rs/crates --include=*.rs` finds the two sentences and no reader of either (2026-11-15) | open. A reason that named the witness by its text whether or not it is interned would be one sentence; it is `oblgen.rs`'s to change, and nothing prints it today. It bears on [S1f.10.12](s1f.10.12_the_structure_shown.md) if a surface ever shows why an instance is not branched on |
| K-11 | **On a program of the third rung, the obligation that would state a *must* moves the ladder, and with it H.** T3's third route, *the group is what an obligation rule would owe, were it stated*, was tried on I-B06, `branching/06_lookahead_on.ein`, whose ten groups are an object's nine other types: the lines add an obligation rule that root pays, every object's type being stated. Root owes nothing, the ladder is then on its second rung, H is empty and R is `{∅}`, where the program's own R is 2 states and 4 commitments over 132 candidates. The *must* changed nothing and the rule's presence changed the answer | `bash build/structure/s4/probes.sh`, then `build/structure/s4/out/examples_branching_06_lookahead_on.count.txt` against `…/build_structure_s4_p_b06_typed_owed.count.txt` (2026-11-15); the lines are `build/structure/s4/p/typed_owed.lines`, verbatim in [S1f.10.4's record](s1f.10.4_bounded_groups.md#t3-where-the-must-is-missing) | S1f.10.4, 2026-11-15 | it is E-51 as [S1f.10.15](s1f.10.15_the_rung_a_loaded_rule_chooses.md) has it, H15-5's *from the second rung, the only way to the blind enumerator is an obligation nothing owes*, met from the other side: a program cannot state a debt without choosing a rung | open, and [Q-M1f.308](../open_questions.md#q-m1f308--the-rung-a-loaded-obligation-rule-chooses)'s: the probe's rule is loaded and activated and root owes it nothing, so of that entry's three options only the third, *by a debt owed at root*, would leave the probe the blind enumerator's H; the comparison is re-taken by the same command |

## From S1f.10.9

Met by [S1f.10.9](s1f.10.9_what_the_blind_enumerator_takes_for_an_object.md#record-2026-11-15-tasks-1-to-3)'s
tasks 1 to 3, 2026-11-15, on the release binary of `d7050ccf`. The witnesses are under the untracked
`build/s1f.10.9/h9/` and verbatim below; `build/s1f.10.9/run.sh`, which the stage's record carries, runs them.
None is fixed, and no register row is filed: the stage was told to file none.

### H9-1 — A reserved word a program writes as a value is a name of the program and never an object

The reverse of E-1: a name the blind enumerator does **not** take. `hypgen::candidate_objects` drops seven
reserved words, `not false and or absent eq neq`, and every name `Program::categorise` calls a relation, the
kernel's `relation`, `rule` and `written` among them, whether or not the program writes the word as a value.
Five of them a program can write as a value today, and each loads: `eq`, `absent`, `false`, `relation`,
`written` (`(order eq)` and its four twins, probed; `neq`, `not`, `and`, `or` and `open` are refused at the
parse). The shipped stdlib writes one: `std.math.digits`'s table `cl` holds the orders `lt`, `eq` and `gt` as
values, `(cl eq 0 0 eq)`.

`h9/eq_as_value.ein`:

```
;; S1f.10.9, H9-1: a reserved word a program writes as a value is a name of
;; the program and never an object. std.math.digits writes `eq` so, in the
;; rows of its table `cl`.
(relation order Order :why "{?1} is an order")
(order lt) (order eq) (order gt)
(relation pick Thing :why "{?1} is picked")
(solve :no-hypothesis (order))
(query :goal (pick ?x))
```

`h9/eq_as_value_written.ein`:

```
;; S1f.10.9, H9-1's twin: eq_as_value.ein with the fact the enumerator does
;; not propose written by the program. It loads and root is alive.
(relation order Order :why "{?1} is an order")
(order lt) (order eq) (order gt)
(relation pick Thing :why "{?1} is picked")
(pick eq)
(solve :no-hypothesis (order))
(query :goal (pick ?x))
```

`ein oracle` on the first: H is `(pick gt)` and `(pick lt)`, 2 candidates, R 4 states, none holding `(pick
eq)`. On the twin: the same 2 candidates, R 4 states, every one holding `(pick eq)`, so the state the first
program never reaches is alive and owes nothing. The same on the shipped stdlib: the stage's witness
`e1/shipped_decimal_arith.ein` proposes `(total lt)` and `(total gt)` and not `(total eq)`.

**How it reads against the rulings.** `eq` in `(order eq)` is a constant in an argument position of a fact, so
it is a name of the program by Q-M1e2.9 (a), and `invariant::program_names` lists it.
[`solution_semantics.md` § 1.2](../../../docs/kernel/inference/solution_semantics.md) says *every name is in
the candidate list* and *a list that left one out was an engine bug* (W4, W15). Read so, this is a wrong
answer of that family. Read the other way, a reserved word is never an object, as a type a signature declares
is not, and § 1.2 has an exception it does not state. **Not filed as a register row**: which reading holds is
the user's, and it is the same question as Q-M1f.306's option (b) seen from the other side, whether a kernel
name a program writes as a value is an object. Not counted over the corpus.

**Today:** open, to be shown with Q-M1f.306. No owner.

### H9-2 — A written dunder fact registers the dunder as an auto-vivified relation of the program

The loader makes a registry entry for the head of every fact a file writes that no `(relation …)` declares
(`ein-ir/src/from_ir.rs`, *Undeclared relations are auto-created open-world*), and leaves a built-in predicate
out, *so vivifying one would put a phantom entry in the registry*. It does not leave the kernel's dunders out:
`ein saturate FILE --dump` on a program that writes `(__closed__ digit)` prints *Relations — auto-vivified
(1)*, `__closed__`, and on the same program with the closure derived by a rule prints none (the stage's
`e1/closed_by_fact.ein` and `e1/closed_by_rule.ein`). So whether the registry holds `__closed__` or
`__symmetric__` depends on how a closure or a symmetry came to hold, and every reader of the registry sees a
kernel name as a relation of the program in one case and not in the other.

The blind enumerator is one such reader, and what it does with the difference is E-1 and Q-M1f.306. **The
other readers were not examined**: the `relation` facts the loader states, `scope.rs`'s *outside* list,
`invariant::Universe`, the render views and the `--dump` listing itself. No wrong answer is known from them.

**Today:** open. It bears on where a fix under Q-M1f.306 (b) is made: one in the enumerator leaves this
difference in place.

### H9-3 — E-1 and E-7 are absent from the corpus's answers by the rung twelve programs take

The census of S1f.10.9 finds no corpus program with a dunder among its candidates and one with a relation's
name. On the 174 programs whose H an hrule, the obligations rung or `(solve :no-search)` builds, the blind
enumerator is not asked; were it (`hypgen::generate_blind` at root, which is not H and decides nothing), it
would name `__closed__` on 8 programs, 458 candidates, and a derived relation's name on 4, 200 candidates
(`cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example object_census`, the `alone`
columns, 2026-11-15):

| programs | rung today | the blind enumerator alone would name |
|---|---|---|
| `examples/binary_logic/{equivalent,knights,not_equivalent,satisfiable,unsatisfiable}.ein`, `stdlib/bool/inputs.ein`, `tests/stdlib/bool/inputs/{01_in_drives,02_required}.ein` | obligations | `__closed__` |
| `examples/domain_elim/{ab,b_branch,b_only}.ein` | an hrule | `surjective`, `total` |
| `examples/features/09_adjacent_via_same_house.ein` | obligations | `adjacent-via-bwd`, `adjacent-via-fwd` |

**Today:** not a wrong answer and nothing to fix. It is a dependency between two of the phase's questions:
[S1f.10.15](s1f.10.15_the_rung_a_loaded_rule_chooses.md) asks which rung a program takes, and a ruling there
that sends a program to the blind enumerator makes Q-M1f.306's (a) to (c) bear on programs they do not bear on
today. The census is re-taken by the same command after such a ruling.

## From S1f.10.6

Met by [S1f.10.6](s1f.10.6_obligations_under_hypothesis.md#record-2026-11-15-tasks-1-to-3)'s tasks 1 to 3,
2026-11-15, on the release binary of `14063474`. The programs are under the untracked `build/s1f.10.6/` and
verbatim below and in the stage's record; `build/s1f.10.6/run.sh`, which the record carries, runs them. None is
fixed, and no register row is filed: the stage was told to file none.

### H6-1 — An hrule that reads what a commitment can change proposes at a node what it did not at root, and nothing flags it

Q-M1e.11's loss without an obligation derived at a node. H is the generator's list at root's fixpoint, and an
hrule is matched there and nowhere else. Where an hrule's `:match` reads a relation another hrule proposes, the
facts it would propose once a commitment holds one are never candidates.

`hr/reads_h.ein`:

```
;; S1f.10.6, T1, the hrule rung with no derived activator: both debts are
;; stated, there is no rule `arm`, and the hrule that proposes a drink reads
;; `likes`, which the other hrule proposes.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(total-owed drinks is-a)
(hrule guess-likes  () :match (and (is-a ?a Person) (is-a ?b Food))  :assert (likes ?a ?b)  :why "guess: (likes {?a} {?b})?")
(hrule guess-drinks () :match (and (likes ?p ?f) (is-a ?d Drink)) :assert (drinks ?p ?d) :why "{?p} likes {?f}; guess: (drinks {?p} {?d})?")
```

`hr/reads_h_stated.ein`:

```
;; S1f.10.6, T1: hr/reads_h.ein with the fact its second hrule reads stated.
(import std.algebra :symbols (total-owed))
(relation is-a   T T :why "{?1} is a {?2}")
(relation likes  Person Food :why "{?1} likes {?2}")
(relation drinks Person Drink :why "{?1} drinks {?2}")
(is-a Ann Person) (is-a Soup Food) (is-a Tea Drink)
(total-owed likes is-a)
(total-owed drinks is-a)
(likes Ann Soup)
(hrule guess-likes  () :match (and (is-a ?a Person) (is-a ?b Food))  :assert (likes ?a ?b)  :why "guess: (likes {?a} {?b})?")
(hrule guess-drinks () :match (and (likes ?p ?f) (is-a ?d Drink)) :assert (drinks ?p ?d) :why "{?p} likes {?f}; guess: (drinks {?p} {?d})?")
```

`ein solve` and `ein oracle` on the first: H is `(likes Ann Soup)`, one candidate; the node that holds it owes
*drinks owes Ann a Drink*; **R is empty, every alive node owing**. On the second, which states the fact the
second hrule reads: H is `(drinks Ann Tea)` and R one state. `owed_census --iterate` on the first: the
generator at `{(likes Ann Soup)}` proposes `(drinks Ann Tea)`, and R over the two candidates is one state,
`{(drinks Ann Tea) (likes Ann Soup)}`, which `ein oracle` answers on the copy that states both
(`price/reads_h_r2.ein`).

**Nothing detects it.** No rule concludes an activator and the static check does not read an hrule's plan, so
the program is *monotone*: `ein solve` takes the state route and `--single` the obligation tree with L5. Root
owes both instances, so `check_owed` has nothing to refuse. The read-out's `outside` row is silent: no
saturation rule writes `drinks`, no query reads it, and an hrule asserts it.

**The corpus** (`owed_census`, 2026-11-15, the command in the stage's record). Four programs have an active
hrule whose `:match` reads a relation in `D`: `examples/ein-bugs/wrong-answers/hrule-reads-unknown.ein`
(`paid`), and `examples/ein-bugs/wrong-answers/past-limit-lines.ein`,
`examples/lattice/04_obligation_trigger_hole.ein` and `tests/region/07_cube.ein` (`is-a`). On none does a node
propose a fact outside H: three by the cube at the oracle's default limit, the second at `--limit 17`. Of the
88 programs on the hrule rung the cube read 61, and **no corpus answer is known to be concerned**.

**How it reads against the rulings.**
[`solution_semantics.md` § 1.4](../../../docs/kernel/inference/solution_semantics.md) defines a candidate as
*a fact the generator proposes at root*, so R is right by the definition. The same section holds that *H must
contain every fact a model needs* (ruling (h)), and that where it does not *either the program is wrong … or
the engine is*: which of the two an hrule conditioned on a guess is, is the user's. **Not filed as a register
row.**

**What it bears on.**

- **Q-M1f.307.** Answer (A) read over nodes (the record's reading 2) gives the twin's model here. Read
  statically from activators (reading 1) it does not reach this program, and (B), (C) and (D) as worded do
  not either: no activator is derived, and the program is monotone.
- **[README § The domain](README.md#the-domain)'s second paragraph**, *H does not grow*, is enforced for
  debts on a monotone program and for nothing an hrule proposes. Track S reads H as root builds it either way;
  what is not enforced is that no node would propose more.

**Today:** open, to be shown with Q-M1f.307. No owner.

### H6-2 — Under (B), Q-M1f.306 bears on two more programs, and marks nothing on them

[H9-3](#h9-3--e-1-and-e-7-are-absent-from-the-corpuss-answers-by-the-rung-twelve-programs-take)'s dependency,
met from this side. Q-M1f.307's answer (B) sends `oblig-derived-activator.ein` and its `-declared` control from
the obligations rung to the blind enumerator, so Q-M1f.306's options (a) to (c) would bear on them. The
stage's stand-ins for that H, `build/s1f.10.6/blind/derived.ein` and `blind/declared.ein`, hold twelve
candidates each, `likes` and `drinks` over `Ann`, `Soup` and `Tea`, and `object_census` marks none of them: no
kernel dunder, no relation's name, no name only an import writes (`cargo run --release --manifest-path
ein.rs/Cargo.toml -p ein-infer --example object_census -- build/s1f.10.6/blind/derived.ein
build/s1f.10.6/blind/declared.ein`, 2026-11-15).

**Today:** not a wrong answer and nothing to fix. A ruling of (B) re-takes S1f.10.9's census by its own
command, as H9-3 says of S1f.10.15's.

## From S1f.10.15

Met by [S1f.10.15](s1f.10.15_the_rung_a_loaded_rule_chooses.md#record-2026-11-15-tasks-1-to-3)'s tasks 1 to
3, 2026-11-15, on the release binary of `c6251db3`. The programs are under the untracked `build/s1f.10.15/`
and verbatim in the stage's record; `build/s1f.10.15/run.sh`, which the record carries, runs them. None is
fixed, and no register row is filed: the stage was told to file none.

### H15-1 — Under (b), Q-M1f.306 bears on one more corpus program, and under (c) on two

[H9-3](#h9-3--e-1-and-e-7-are-absent-from-the-corpuss-answers-by-the-rung-twelve-programs-take)'s dependency,
measured. Q-M1f.308's options (b) and (c) send programs from the obligations rung to the blind enumerator,
and Q-M1f.306's question is then asked of them. Of H9-3's twelve programs (b) moves one and (c) two; the
other ten keep their rung under every option, seven owing at root and three under an hrule
(`cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example rung_census -- --jobs 8
--check`, the columns `K1` to `any`, and `object_census` on the two files, 2026-11-15):

| program | moved by | new candidates | marked | what the marks name |
|---|---|---:|---:|---|
| `examples/features/09_adjacent_via_same_house.ein` | (b), (c) | 82 | 44, `R1` | `adjacent-via-bwd`, `adjacent-via-fwd`, a derived relation's name: E-7 |
| `stdlib/bool/inputs.ein` | (c) | 342 | 340: 36 `K1`, 336 `IN` | `__closed__`: E-1; the sixteen functions `std.bool.gates` writes: E-54 |

On the first, root is dead, so every candidate is dead alone and R is empty with the marked ones or without.
On the second, R goes from {∅} to an unexhausted region over the 342. No other corpus program an option moves
gains a candidate.

**Today:** not a wrong answer and nothing to fix. A ruling of (b) or (c) re-takes S1f.10.9's census by its
own command, as H9-3 says.

### H15-2 — Under (b), a program that loads today is refused: *forbid*'s premise depends on the rung

*Forbid* refuses a program whose refutation could rest on an `absent` over a relation a commitment can still
extend, and which relations those are is read off H. So what a loaded and unactivated obligation rule takes
off the blind enumerator, it also makes legal to read closed-world. `w/connex_lt_alive.ein`, the re-plan's
witness with its third pair stated:

```
;; S1f.10.15, T1: the re-plan's witness with the third pair stated, so root
;; is alive.
(import std.algebra :symbols (connex))
(relation lt Node Node :why "{?1} is less than {?2}")
(instance A Node) (instance B Node) (instance C Node)
(connex lt instance Node)
(lt A B) (lt B C) (lt A C)
(query :goal (lt ?x ?y))
```

Today: it loads, H is empty, R is {∅}, and `connex` is read over `lt` as stated. With the rung chosen by an
obligation activated, the blind enumerator proposes `(lt B A)`, `(lt C A)` and `(lt C B)`, and the program is
refused at load, exit 1: *rule 'connex' [activator lt instance Node]: concludes (false) from (absent …) over
lt, a relation a commitment can still extend … close the relation, (solve :no-hypothesis (lt))* (`ein solve`
on the stage's copy `b/connex_lt_alive.ein`, and `rung_census`'s `refusals` column, which reads
`forbid::check` over the blind list in process). The witness as described, with two pairs stated and a dead
root, is refused the same way.

**The corpus**: none. Its two programs that activate `connex`,
`tests/stdlib/algebra/08_checks_satisfied.ein` and `12_connex_violated.ein`, name `lt` in a `:no-hypothesis`
list; `rung_census` finds no program (b) or (c) moves that a load would then refuse.

**How it reads against the rulings.** Neither answer is wrong: today no commitment extends `lt`, so the
closed-world reading is the legal one by *forbid*'s own rule, and under (b) the refusal is that rule applied
to another H. What it shows is that an import decides today whether a program's `absent` is closed-world.

**Today:** open, to be shown with Q-M1f.308. No owner. Under (b), task 4's list of moved answers has to hold
programs that stop loading, of which the corpus has none.

### H15-3 — The corpus was written for the third rung: one header and 35 lists say so

`examples/features/09_adjacent_via_same_house.ein` imports `std.algebra` for `symmetric`, `transitive` and
`includes` and `std.bijection` for its two typechecks, activates no obligation, and has been on the second
rung, with no candidate, since every import brings its module whole. Its header still describes the third:
*"Argument typing, so the blind hypothesis generator stays inside the declared signatures"*, and of its
`:hypothesis-relations` list, *"if the guard comes back, root is consistent and the engine goes looking …
Four candidates is enough to report a verdict and stop."* Its corpus note says *with the guard back it
answers Ambiguity k=4*. With the guard put back (`w/features09_guard_back.ein`, the stage's `sed` over the
tracked file): today H is empty and R is {∅}, one state; on the third rung H is 82 candidates, 78 dead alone
and the four the header counts alive, and R is 9 states. The fixture still fails on the regression, a dead
root turned alive, and no longer shows what it says it would.

And 35 programs an unactivated rule holds on the second rung, 32 of them under `tests/stdlib/`, carry a
`:no-hypothesis` list that names every relation the blind enumerator would fill. On the second rung with
nothing owed the list filters nothing, and nothing notes it: the `note:` for a list that filters nothing is
written for a list beside an hrule.

**Today:** not a wrong answer. Under Q-M1f.308 (b) the header and the lists are true again as written; under
(a) the header and the corpus note are stale and task 4 restates them.

### H15-4 — The ladder is stated in five places, and three of them read as option (c)

Two say what the engine does.
[`06_reserved_names.md` § Hypothesis / query control](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md#hypothesis--query-control),
since S1f.6.34 corrected it: *a loaded obligation rule that owes nothing at root proposes nothing, and the
blind enumerator does not run*. And `oblgen.rs`'s header: *the middle row is about what the program declares,
not about what this state happens to owe*.

Three do not. [`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md) § 1.4 and
`AGENTS.md` say *the program's own `(hrule …)` if it declares one, else the facts that would discharge what
root owes, else the blind combinatorial enumerator*, and `hypgen.rs`'s header *obligation-driven while the
state owes something it may branch on, otherwise blind combinatorial*. Read as written, they send a program
that owes nothing to the blind enumerator, which is Q-M1f.308's option (c) and what the engine does not do:
`w/o_satisfied.ein` owes nothing and has no candidate.

**Today:** a documentation finding, no answer concerned. Whatever Q-M1f.308 rules, the sentence is restated
in all five; task 4 names `06_reserved_names.md` and the two module headers.

### H15-5 — From the second rung, the only way to the blind enumerator is an obligation nothing owes

`(solve :no-search)` is how a program says *judge root alone*. A program that imports `std.algebra`,
`std.bijection`, `std.closure`, `std.typing`, `std.slots` or `std.bool.inputs` and wants its own open
relations guessed about has no declaration for that: an hrule is the first rung and not the third, and the
two lists only take relations out. What reaches the third rung is the second rung declining, which a program
can cause by writing an obligation rule with a bare `(open)` that no state owes.
[`tests/stdlib/bijection/06_blind_enumeration.ein`](../../../tests/stdlib/bijection/06_blind_enumeration.ein)
does so on purpose, by `sorted-somewhere`, *"the file has to earn it back"*, and the stage's copies for
options (b) and (c) are made the same way, with one rule:

```
(rule s15-off () :match (and (s15-on) (absent (s15-paid))) :assert (open) :why "never owed: a bare (open) takes the obligations rung to the blind enumerator")
```

On each of the stage's programs the copy's H is the blind enumerator's list at the original's root, and its R
is the cube `rung_census` computes over that list in process.

**Today:** not a wrong answer. It bears on Q-M1f.308 (a): kept as it is, the ladder has a declared way down
to root alone and none across to the third rung.
