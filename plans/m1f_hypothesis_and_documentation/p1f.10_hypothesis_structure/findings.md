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
| K-12 | **`domain_elim_vs_hypothesis.md` says in the present tense what the engine no longer does.** The page's banner scopes its *tables* as a period record, and three things outside the tables are stated as today's and are not. (1) *"all three have a model, so all three are **Solution** (`k=1`)"*: `ein solve --single` answers *Ambiguity* on `b_only.ein` and on `b_branch.ein`, whose R is two states, ∅ and `{(color-loc Blue H1)}`, since nothing there owes Blue a house; only `ab.ein` is *Solution*. (2) *"on this puzzle the engine has **three** elimination mechanisms"*, the rule, forced-positive promotion and the pre-branch lookahead: the last two are retired fields (`enable-forced-positive`, `enable-pre-branch-lookahead` in `ein-core/src/config.rs`'s retired list, *"no route does"*). (3) § Cross-links gives *"The lookahead lever: `SolverConfig.enable_pre_branch_lookahead`"* as a lever. Today's numbers on the three fixtures are in [S1f.10.13's record](s1f.10.13_the_price_list.md#why-not-a-saturation-rule-the-comparison-restated) | `for f in ab b_only b_branch; do ein.rs/target/release/ein solve examples/domain_elim/$f.ein --single \| head -12; done`; `grep -n 'enable-pre-branch-lookahead' -A4 ein.rs/crates/ein-core/src/config.rs` | [S1f.10.13](s1f.10.13_the_price_list.md), 2026-11-16 | the page stands as it is: the stage was told not to edit it | open. P1f.20's, which sorts each page into *current*, *bannered* or *moved*: the page's §§ B to D and its cross-links are the walk's, and its one claim that survives, *a rule that decides a candidate at root takes it out of the search*, is restated in the record with today's numbers |
| K-13 | **No route reads what a state denies.** An alive node's fixpoint may store `(not b)` for a candidate `b` outside its closure. Both routes then extend the state by `b`, saturate `S ∪ {b}` and watch it die, though on a monotone program that node holds `b` and `(not b)` whatever else it derives. The node function reports a node's closure and not its denials (`region::NodeOut`), and neither `tree.rs`'s `extend` nor `states.rs`'s step asks. On the zebras every dead node that holds a pair of the theory is one of these, since their rules store the negation: on `zebra2.ein` 38 of `--single`'s 45 saturations and 111 of describe's 374, on `zebra2-minus-15.ein --maximal` 2 246 of 2 346. On `send_more_money.ein` none is: `std.algebra`'s `functional` and `injective` conclude `(false)` from the two facts and store nothing, so the same exclusion is announced by one program's rules and not by another's. Reading the denials costs no saturation | `ein.rs/target/release/examples/price_list examples/zebra2.ein`, then `jq '.cells.single.ways.denials' build/structure/examples/zebra2/prices.json`; the corpus's table is [S1f.10.13's record](s1f.10.13_the_price_list.md#t3-the-verdicts) | S1f.10.13, 2026-11-16 | the routes as they are | open, and [S1f.10.3](s1f.10.3_the_restricted_join.md)'s to take first: it is *at most one* spent with no pair layer, on the programs whose rules store negations, and the record's `denials` way prices it on every corpus program. Whether `std.algebra`'s `functional` and `injective` should also store the negation is the same question from the stdlib's side and is nobody's yet |
| K-14 | **A question's name is no identity, and `coupling.md` names an edge's questions by it.** A question is named by the pattern of its members' texts, `(co-located * House-2)`, and two questions of one program can share one: on `zebra.ein` the three kinds of attribute at `House-2` are three groups under that one text, 17 of its 36 questions share their text with another (four texts, a house's three to five kinds), and 300 of its 386 edges and 335 of its 424 one application back hold one of them. Twelve programs of the 625 read have such a pair, nine of them searching (`zebra.ein`, `branching/05_mini_zebra.ein`, `10_kill_cache_on.ein`, `11_kill_cache_off.ein`, `15_` and `16_lookahead_two_step`, `lattice/06_friends_tables.ein` and its `_everyone_sits` twin, `ein-bugs/unsat-core-id-space.ein`). `coupling.json` named an edge's and a debt's questions by text alone, so on those programs the file did not say which questions an edge is of; only `coupling.dot`, which writes `q<place>`, did, and it holds no edge one application back | `jq '[.questions[].name] \| length - (unique \| length)' build/structure/examples/zebra/coupling.json`: 13, the questions beyond the first of each text; `jq -r '[.questions[].name] \| group_by(.) \| map(select(length > 1) \| "\(.[0]) ×\(length)") \| join(", ")' …` (2026-11-16) | [S1f.10.12](s1f.10.12_the_structure_shown.md), 2026-11-16 | `coupling.json` carries `question_ids` beside `questions`, on every edge, every edge one application back and every debt: the one field S1f.10.12 added to `pair_layer`, every other byte of every file the census writes being as it was. `coupling.md`'s tables still name a question by its text alone | open, for the Markdown twin and for any surface: a view that draws the hypergraph has to name a question by more than its pattern (T1f.10.12.3) |
| K-15 | **Graphviz 16.0.0 does not lay one DOT out the same way twice under `fdp`**, the layout `kb_dot` suggests for a KB view (`ein-render/src/kb_dot.rs`, *suggested layout: fdp*), nor under `neato` with `overlap=false` on a few graphs: where no prism library is linked the fallback that removes overlaps moves nodes from run to run, and `fdp` with `splines=true` also moves an edge's label. The picture is well-formed each time, which is all `dot_wellformed.rs` asks; an SVG is not a function of its DOT | `for i in 1 2 3 4 5 6; do dot -Kfdp -Tsvg build/structure/examples/zebra/coupling.dot \| sha256sum; done \| sort -u \| wc -l`: 2 (2026-11-16, Graphviz 16.0.0); with `-Kneato -Gsplines=line -Goverlap=scale`, 1 | S1f.10.12, 2026-11-16 | `utils/structure_bundle.py` draws with `dot`, and with `neato -Gsplines=line -Goverlap=scale` past 60 edges, and three runs of it, two from an empty directory, write one digest. Nothing else in the repo holds a picture to its bytes. Nothing goes upstream (the user's ruling of 2026-11-08) | open; it bears on T1f.10.12.3 if a view's SVG is ever a golden, and on nothing today |
| K-16 | **A census line's `wall` is not the pair layer's where the theory's share is asked.** `pair_layer.rs`'s header lists `wall` among the layer's names, *root's saturation included*, and that is what it is with no flag: S1f.10.1's table has 1.01 s for `zebra2.ein`. Under `--theory`, `--supports` or `--check` the same clock runs over the small KB's layer, the supports and their check, the cover, *exactly one*, the files written and, with `--coupling` or `--readings`, the coupling under every reading asked: 23.8 s on the same program in the census the later stages left. No field of the line holds the layer's own wall there | `jq -r 'select(.path == "examples/zebra2.ein") \| .wall' build/structure/census.jsonl`: 23.82 (2026-11-16); `grep -n 'let wall = t.elapsed' ein.rs/crates/ein-infer/examples/pair_layer.rs` | S1f.10.12, 2026-11-16 | the bundle calls it *the census line's whole reading* and gives the layer no wall but the price list's node-by-node one (`prices.json`, `structure.root_layer.wall`) | open. A sentence of the example's header, or a field of its own; nobody needs the number today |
| K-17 | **A pair layer read whole can leave the witnesses' holders unread, the unlicensed debts drop out of the coupling with them, and `Coupling::complete()` stays true.** Which candidates' own nodes hold each witness of an owed instance is read after root's layer, through its singles (`structure.rs`' `holders`, by `Space::eval`). Every single is cached by then, so the step holds no new node, and `explore.rs`' `afford(0)` still refuses it where `spent > budget`. A layer's last step is afforded by a projection and may carry `spent` past the budget, so the layer ends whole, `stopped: None`, and the reading after it is refused: `Owed::singles_read` is false. `Coupling::read` then takes no payers for an instance L5 does not license, such an instance is no edge, and nothing on `Coupling` says so: `pairs_read` is true, `node_stopped` is `None`, `cut` is empty. The same holds of the small KB's layer and `Owed::theory_read`, where no unlicensed instance is then *about* a group. Where a debt is what joins two of the conflicts' components, the components would come out finer than R with `complete()` true; on the witness they are one component either way | `ein.rs/target/release/ein render structure examples/zebra.ein --view json --region-budget 181087 \| jq '{layer: .pairs.complete, stopped: .pairs.stopped, left: .pairs.pairs.left, singles_read: .exactly_one.singles_read, coupling_complete: .coupling.complete, debts: (.coupling.debts \| length), edges: (.coupling.edges \| length), whole}'`: the layer complete, nothing left, `singles_read` false, the coupling complete, 0 debts and 375 edges, where `--region-budget 181088`, what root's layer spends (`.pairs.work.facts`), gives 18 debts and 386 edges (2026-11-16). The same at 181 000, 180 500, 180 000, 179 000, 175 000 and 150 000; below 150 000 it was not looked for where the layer stops being whole, and no other program was probed. The small KB's side on the same program: at `--region-budget 67243`, one under what its layer spends (`.theory.small_kb_layer.work.facts`, 67 244), that layer is complete with no pair left, `.exactly_one.theory_read` is false, and of the 18 instances L5 does not license none has `pay_by_the_theory_alone` and none an `about_group`, where at 67 244 all 18 have both (2026-11-16) | [S1f.10.12](s1f.10.12_the_structure_shown.md), 2026-11-16, writing the sentence `ein render structure` prints for `singles_read: false`: the first wording blamed a stopped layer, and the run beside it said the layer had left no single and no pair | **both are done** (2026-11-16). The holders are read off the cache and ask no budget (`Space::cached`, `structure.rs`' `holders`): a layer that read its singles has them read, whatever it spent. And where a layer stopped before its last single, which is the one place they stay unread, `Coupling::payers_read` is false wherever the table holds an unlicensed instance with witnesses, and `Coupling::complete()` with it. The witness's command is today `ein.rs/target/release/ein render h-space examples/zebra.ein --view json --region-budget 181087 \| jq '{layer: .pairs.complete, singles_read: .exactly_one.singles_read, payers_read: .coupling.payers_read, coupling_complete: .coupling.complete, debts: (.coupling.debts \| length), edges: (.coupling.edges \| length), whole}'`: the layer complete, the holders and the payers read, the coupling complete, 18 debts and 386 edges, whole, as at the default budget; at `--region-budget 67243` the small KB's layer is complete, `theory_read` is true and all 18 unlicensed instances have `about_group` and `pay_by_the_theory_alone`; at `--region-budget 1` both layers stop among their singles, `singles_read`, `theory_read`, `payers_read` and `coupling.complete` are false, and the view says each in a sentence (2026-11-16). Held by `ein-infer/tests/coupling.rs`' `a_layer_that_read_its_singles_has_its_holders_read` and `holders_unread_leave_the_coupling_incomplete`, and by `ein-cli/tests/h_space_view.rs`' `a_layer_read_whole_has_its_holders_read` and `holders_a_stopped_layer_left_unread_are_said`, each budget taken off the program's own spend. `explore.rs`' `afford` is as it was, so where a budget-stopped run stops did not move | **closed** 2026-11-16, by the commit *K-17: a layer that read its singles has its witnesses' holders read, and a coupling without its payers is not complete* ([S1f.10.12's record](s1f.10.12_the_structure_shown.md#record-2026-11-16-the-view-renamed-h-dumped-plain-k-17)). The same refusal in another reader is [K-19](findings.md), open |
| K-18 | **`examples/README.md`'s anchor list was not the grep's answer on `master`, and `world_anchors` was red.** The list of test files that name a zebra puzzle is a generated block `ein-cli/tests/world_anchors.rs` diffs. The merge of the structure's surface added `ein-cli/tests/structure_view.rs`, which names the two puzzles, and four mentions of `examples/zebra2-minus-15` to `ein-render/tests/dot_wellformed.rs`, and neither file was in the list: the stage ran the tests it named and not the workspace's | `cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-cli --test world_anchors` at `2050a886`: `the_anchor_list_is_the_greps_own_answer` FAILED, the grep's answer holding `ein-cli/tests/structure_view.rs` and `ein-render/tests/dot_wellformed.rs` where the page held neither (2026-11-16) | [S1f.10.12](s1f.10.12_the_structure_shown.md), 2026-11-16, renaming that test file | re-banked with `EIN_BLESS=1` by the two commits that moved the list anyway: the rename, which puts `h_space_view.rs` and `dot_wellformed.rs` in it, and `ein show hyps`, whose `show_hyps.rs` names a puzzle too. Green on the stage's last tree | closed by those commits. What it shows stands: a stage that adds a test file runs `world_anchors`, or the gate, before it is merged |
| K-19 | **A support's check is dropped without a word where root's layer has spent its budget.** `theory_in` reads each excluded pair's own node for the check through `Space::eval`, *each a node the layer read, so the cache answers*, and `explore.rs`' `afford(0)` refuses that step once `spent` is past the budget, as it refused K-17's holders. The refusal is taken for no nodes (`Err(_) => Vec::new()`), the check is skipped because the two lists differ in length, every `Support::check` stays `None`, and the census counts 0 checked and 0 unread beside 414 supports read | `ein.rs/target/gate/examples/pair_layer --check --facts 181087 --out /tmp/k19 examples/zebra.ein \| jq -c '.theory.supports.check'`: `dead` 0, `unread` 0, where `--facts 1048576` gives `dead` 414; and `jq '[.root_excluded[] \| .support.check \| select(. != null)] \| length' /tmp/k19/examples/zebra/theory.json`: 0 against 414 (2026-11-16, one program, one budget, the one under what root's layer spends) | [S1f.10.12](s1f.10.12_the_structure_shown.md), 2026-11-16, fixing K-17 | as it was. Only the census example asks a check (`TheoryOptions::check`); `ein render h-space` never does, and no corpus program is in that state at the default budget. `Space::cached`, which K-17's fix added, reads those nodes without asking the budget | open. One line would close it, the check's nodes read by `Space::cached`; and a refusal there should be an `unread` of each check and not an empty list |
| K-20 | **`ein-infer/tests/obligation_rung.rs` states two counts in a doc comment that are no longer the routes'.** Above `the_theory_finds_what_the_hrule_finds` it says the obligation tree reaches `zebra2`'s one state *in 45 saturations* and that `zebra2-minus-15`'s 32 *cost 2 346 a side*. Since a route reads what a state denies they are 9 and 104. Nothing checks the sentence | `ein solve examples/zebra2.ein --maximal` and `ein solve examples/zebra2-minus-15.ein --maximal`, the `work` row (2026-11-16); `grep -n '45 saturations' ein.rs/crates/ein-infer/tests/obligation_rung.rs` | [S1f.10.3](s1f.10.3_the_restricted_join.md), 2026-11-16 | the comment stands: the file is the obligation rung's, which another stage of the batch was editing | open. One sentence, for whoever next edits the file |
| K-21 | **A describe its budget stops saturates ⊤ though its own memory says ⊤ is dead.** Where the budget stops the state route it saturates the commitment of every candidate once, as route 1 does, to say whether ⊤ is in R (`states.rs`, *⊤ once*). It does so whatever it knows: on `send_more_money.ein` the stopped describe holds thousands of dead sets, each inside ⊤, and still pays the node of 80 candidates; on a monotone program L1 decides it. What the node gives beyond its death is its debts and its facts, which `Top` reports | `ein solve examples/beyond_arith/send_more_money.ein`, whose last saturation is ⊤'s; `grep -n 'when the budget stopped the route and' ein.rs/crates/ein-infer/src/states.rs` | S1f.10.3, 2026-11-16 | as it was: one saturation of a stopped cell, and S1f.10.13's `singles` way was *go* under describe by this node alone | open. Asking the memory first is one line; it changes what a stopped read-out says of ⊤ where ⊤ owes, so it wants its own look |
| K-22 | **On no flagged cube of the corpus does a state's denial fail to kill.** L8 is spent on monotone programs only, and has to be: a rule `(and (p A) (absent (p B)))` that concludes `(not (p B))` gives an alive node above another's denial (`a_flagged_program_spends_no_denial`). But over every flagged corpus program and register witness within `--region-limit`, read past *forbid*, no alive node lies above an alive node and a candidate it denies: the corpus's flagged programs store no negation their `absent`s can take back | `cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer --test known_dead what_an_alive_node_denies -- --nocapture`, whose line ends *flagged cubes where a denial does not kill: []* (2026-11-16; 561 monotone cubes, 8 486 denials checked) | S1f.10.3, 2026-11-16 | a flagged program is told nothing, on the argument and not on a witness of the corpus's | open. It bears on [F21](../../followups/f21_non_monotone_programs_decided.md)'s D6, what a route may learn on a flagged program: a census of which flagged programs a denial is sound on would start from this test |
| K-23 | **The obligation tree's work follows the universe's order, which is rendered text, and so a program's spelling.** Three order points read a candidate's place in the universe: a branch's alternatives are taken ascending (`tree.rs`, `step`), a climb adds candidates ascending, and the last key of the choice of instance is the instances' alternatives as rendered text ([S1f.10.5](s1f.10.5_ordering.md#the-order-built)). None reads an interner's id or the order a program states its facts in, and a renaming of a program's objects, or an import form that spells an imported name otherwise, is neither: it reorders the universe. **The first two are older than the order built, and one corpus program shows them**: `binary_logic/synthesis.ein` lists three of `std.bool.gates`' sixteen functions short, as written they sort before the thirteen spelled in full, the two that fit are among them, and `--single` has its two states after 4 saturations; with every name in full it takes 16. The third is this stage's, and moves the first tree a run walks before it reads the couplings | the forms: `sed -E` the three imports of `examples/binary_logic/synthesis.ein` to `(import M)` and `Xor`, `And`, `Or`, `gate2`, `column-name`, `bin-fa`, `total-owed` and `functional` to their full names, then `ein solve FILE --single`: *Ambiguity* in 16 saturations and 148 facts, where the file as written is 4 and 28, **on the binary built before the order as on the one after**. The renaming: `sed -E '/^;/! { s/\bS\b/A/g; s/\bE\b/B/g; s/\bN\b/C/g; s/\bD\b/F/g; s/\bM\b/G/g; s/\bO\b/H/g; s/\bR\b/J/g; s/\bY\b/K/g }' examples/beyond_arith/send_more_money.ein > /tmp/renamed.ein`, the eight letters spelled so that their text sorts as the puzzle is written, then `ein solve /tmp/renamed.ein --single`: *Solution* in 19 433 saturations and 229 361 facts, the couplings read at saturation 1 872, where the file as spelled takes 18 030 and 221 803 and reads them at 712 | S1f.10.5, 2026-11-16 | the answer is the same and the work is not. `ein-cli/tests/import_invariance.rs` holds every corpus program's `--single` to one verdict and one work under its three import forms but `synthesis.ein`, which it names with why | open. The universe is in the order of the main file's spelling (`region::order_key`). An order by full names, were the routes given them, would hold under the import forms and still move under a renaming; what is equal in coupling and in count is, to the tree, the same instance |
| K-24 | **Counting an alternative a state denies as dead makes the tree choose worse on the zebra2 family.** The choice of instance counts the alternatives *not known dead*, and since S1f.10.3 a denial is known dead. With the count held to what a saturation found dead and the memory left as it is, `zebra2.ein --single` is 7 saturations where it was 9, `zebra2-minus-15.ein --single` 30 where it was 40 and `--maximal` 96 where it was 104, and no other program of the corpus moves but one cell the other way, `zebra2.ein -n 1`, 5 · 302 where it is 5 · 164. So S1f.10.3's 40 against a simulated 29 was the choice and not the memory | `ein.rs/target/release/examples/price_list --orders --only today,deaths-count`, then `--orders-report`; [S1f.10.5 § T3](s1f.10.5_ordering.md#t3-measured), *What the choice reading the denials did* | S1f.10.5, 2026-11-16 | the order built keeps the count reading the memory, and under its last key `zebra2.ein --single` is 7 either way | open, not looked into past the table: four programs, one puzzle. That an instance with one alternative left is a step that costs a saturation and decides nothing is a guess nobody traced |
| K-25 | **Branching the largest owing state first is a quarter less work under `-n 1` and is sunk by three cells of five saturations.** With the tree's queue taking the largest closure first and the choice as it was, `-n 1` over the corpus's cells that finish both ways is 13 296 saturations where it is 17 208, 20 cells lower, `send_more_money.ein` 12 662 where it is 15 854, and `zebra2-minus-15.ein --single`, an *Ambiguity*, 10 where it is 40. Three cells are higher by more than a tenth: `zebra.ein -n 1` 5 · 328 → 7 · 1 114, `zebra2.ein` and its `-obligations` twin 5 · 164 → 7 · 528. Under `--minimal` and `--maximal` it loses more: `ein-bugs/wrong-answers/oblig-fresh-name-two.ein` 39 → 150, `lattice/06_friends_tables_everyone_sits.ein` 213 → 273 | the same harness, `--only today,today/deepest,coupled/20/deepest`; [S1f.10.5 § T3](s1f.10.5_ordering.md#t3-measured) | S1f.10.5, 2026-11-16 | not built: *no* for all four questions by the rule's letter | open. A queue for the questions that stop at a first state and another for those that exhaust the tree was not measured, and the tenth on a cell of five nodes is the user's (K-26) |
| K-26 | **The rule's tenth is decided by cells of five to nine saturations, where one node or a hundred facts is more than a tenth.** 161 of the 188 `--single` cells are under ten saturations. Under `-n 1` every order with a canonical last key is *no* by one such cell, `zebra2.ein` 5 · 164 → 5 · 302, the same five saturations on other nodes; under `--maximal` the order built is *go* at a share of a twentieth and *no* at a tenth and at a fortieth, by one program whose cell moves by a third with the moment the node is read (`zebra2-minus-15.ein`, 108, 150 and 150 where it was 104); the largest state first is sunk under `--single` by `owed-seats.ein` 6 · 8 → 5 · 9, one node fewer and one fact more | [S1f.10.5 § T3](s1f.10.5_ordering.md#t3-measured), every table's *by more than a tenth* | S1f.10.5, 2026-11-16 | the rule was applied by its letter: `-n N` keeps the reported order among equals for it | open, the user's: the tenth is S1f.10.13's number, *the user's to revise at the stop*. A floor under which a cell is not read for the tenth, or the tenth read in saturations alone, would make `-n N` canonical and the queue a question again |
| K-27 | **The whole node can cost more than the search it would order.** `beyond_arith/equation_two_unknowns.ein`: the node of every candidate together is 17 620 facts, every pair of digits through the calculator, where `--single` is 524 saturations and 24 571 facts. Read unconditionally it takes that cell to 42 191 facts for no change of order | `ein.rs/target/release/examples/price_list --orders --only today,node@100 examples/beyond_arith/equation_two_unknowns.ein` | S1f.10.5, 2026-11-16 | the tree reads the node only within a twentieth of the facts the run has derived, stops it there and counts what it derived; on this program it is never read. `ein render h-space` reads it whole, within its layer's budget | open as a fact about the structure's cost: one saturation is not a bound on a node's facts, and S1f.10.13's *one saturation a program* priced the node in the unit that hides this |
| K-28 | **`ein-corpus`'s `the_slow_set_is_exactly_these_fifteen` carries a count in its name.** I-SENDW joined the slow set when it was banked, stopped at the budget, and is still in it with the order built: its two runs are 1.59 s together on an idle machine, 816 ms and 776 ms. So the test lists sixteen entries under a name that says fifteen | `grep -n 'fn the_slow_set' ein.rs/crates/ein-corpus/src/manifest.rs`; `grep -c '^slow' corpus/corpus.toml` | S1f.10.5, 2026-11-16 | the name is as it was, with a comment that says so; the flag was re-taken on an idle machine and stands | open. A name with no count in it would not need renaming when the set moves; it is cited by name in `AGENTS.md`, `corpus/README.md`, `corpus.toml` and `corpus_cli.rs` |
| K-29 | **`ein_core::Kb`'s learned no-good store is the walk's, and its doc says it is the shipping engine's.** `Nogoods`' doc tables "every access in the shipping engine": `nogoods::emit_nogood`, `nogoods::subsumed`, `Run::generate_layer`, `Run::proof`, all deleted with the walk, and argues from `Run::fan_out` and `apriori::filter_reason`. No route writes a clause: every route reads N, the minimal dead sets, by construction. The store, `Kb::freeze_nogoods` and the `.einb` container's `NOGOODS` section (kind 9) remain | `git grep -n 'emit_nogood\|fn generate_layer\|filter_reason' -- ein.rs/crates` (doc comments of `kb.rs` and `ein-einb/src/sections.rs` alone); `git grep -n 'nogoods()' -- ein.rs/crates` (`ein-einb`'s section and `kb.rs`'s own tests) | [S1f.10.16](s1f.10.16_what_the_walk_left.md), 2026-11-16, T1 | as found: outside the generator, so outside the stage's five items | open. Deleting the store is a `.einb` layout change (`FORMAT_MAJOR`), as the retired config fields' would be |
| K-30 | **`hypgen::is_solution_node` and `consistent` are the walk's definition of an answer, and one test reads them.** `is_solution_node` is *`consistent ∧ complete` — P1.7a's domain-agnostic definition of an answer*; the answer is R. Re-exported from `ein-infer`'s `lib.rs` with `consistent`; read by `search_semantics.rs`'s `solution_node_needs_both_halves` alone, and named by `architecture.md`'s *complete model* row and `implementation.md`'s `hypgen.rs` row. `complete`, which it calls, is also read by the shape golden's `hyp` ops (`complete_counted`) | `git grep -n 'is_solution_node\|ein_infer::consistent' -- ein.rs docs/kernel` | [S1f.10.16](s1f.10.16_what_the_walk_left.md), 2026-11-16, T1 | as found: reached through `hypgen::generate` as item 3's lookahead is, but not one of the stage's five items | open. By the stage's rule it would be deleted with its test; `complete` would stay with the `hyp` ops ([Q-M1f.312](../open_questions.md#q-m1f312--what-the-shape-goldens-hyp-and-hypclosed-ops-digest)) |
| K-51 | **`explore.rs`' `charge_whole_node` doc says the cache's nodes stop at their first contradiction; they run to quiescence.** *"Its facts are those of the node saturated to quiescence, where the cache's nodes stop at their first contradiction."* `Space::eval` → `region::fan_out` → `node_in_region` → `fixpoint_firing` → `Saturator::saturate(…, None, …)`, which returns only at the two-phase fixpoint, as `region.rs`' header says (*"never stopped at the first contradiction"*). The comparison the sentence makes, the whole node's facts against a cached node's, has no difference to draw | `ein.rs/target/release/examples/early_stop --out build/early_stop/final --jobs 8 --alive-orders --cores`: every one of the 251 733 distinct dead nodes the routes visit on the corpus is handed to the watch with its FIFO fixpoint's facts (the check `dead node's watched facts ≠ its FIFO fixpoint's` never fires), and 95 429 of them hold more facts at quiescence than at their first contradiction | [S1f.10.19](s1f.10.19_a_dead_node_stopped_early.md#t1-the-inventory), 2026-11-16 | the sentence stands; `region.rs` is right | open. [S1f.10.19](s1f.10.19_a_dead_node_stopped_early.md)'s T4 edits both, since a built stop makes the routes' dead nodes stop there and the oracle's not |
| K-52 | **`region.rs`' header names a field that does not exist and a value it does not have**: *"`owes.root` reports 0 on a dead root whatever it owes."* No name `owes.root` is in the workspace; the summary's `region.root_owes` is `RootStats.owes`, documented *"dead or not"*, filled by `region::prepare_as` from root's own tally whether root is dead or not, and written unfiltered by `ein-render`'s `region_answer.rs` | `grep -rn 'owes\.root' ein.rs/crates` (the header alone); `region.rs`' `RootStats::owes` and `prepare_as`; `ein-render/src/dump/region_answer.rs`' `root_owes` (2026-11-16). What a dead root's read-out prints was not run | [S1f.10.19](s1f.10.19_a_dead_node_stopped_early.md#t1-the-inventory)'s inventory, 2026-11-16 | the sentence stands | open. The header is a page S1f.10.19's T4 edits |

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

### H9-4 — A `(__symmetric__ R)` a rule derives is not mirrored by the saturation that derives it

Met by S1f.10.9's task 4, 2026-11-16, at `3e59b90a`, the gate build, when the register's `__symmetric__` witness
of W96 was put in the corpus. The saturator closes a symmetric relation under arg-swap natively, and it reads
which relations are symmetric where it is built: a `(__symmetric__ R)` fact that a rule derives during the run
is not read by that run. Two programs, the symmetry stated one way in each. `sym_rule.ein`, the witness:

```
(relation near Thing Thing :why "{?1} is near {?2}")
(near a b)
(rule near-symmetric () :match () :assert (__symmetric__ near) :why "near holds both ways round")
(query :goal (near ?x ?y))
```

`sym_fact.ein`, its control:

```
(relation near Thing Thing :why "{?1} is near {?2}")
(near a b)
(__symmetric__ near)
(query :goal (near ?x ?y))
```

The command, on each: `ein saturate FILE --dump | grep -E '^  \((near|__sym)'`, the facts of root's fixpoint
headed `near` or `__symmetric__`.

| the program | what it prints |
|---|---|
| `sym_fact.ein` | `(__symmetric__ near)`, `(near a b)`, `(near b a :rule __symmetric__)` |
| `sym_rule.ein` | `(near a b)`, `(__symmetric__ near :rule near-symmetric)`, and no `(near b a)` |

So root's fixpoint, as `ein saturate` prints it, is not closed under the symmetry it holds. A second saturation
of the same KB, whose saturator is built with the derived fact present, writes `(near b a)`; a node, which
resumes root's saturation, holds it too (`ein solve` on the second program answers the query `a b` and `b a` at
∅). **One symmetry stated two ways gives two roots.** Read against `solution_semantics.md` § 1.3, *root is
`sat(program)`, saturated to quiescence*, this is a wrong answer of `ein saturate`'s; no register row is filed
here, the finding being outside the stage's ruling.

**What it broke on the way**, with the first form of the witness (`(near a b)` alone, the rule, an open
`total`), each a sweep over the corpus that no program had tripped, since no corpus program derived a
`(__symmetric__ R)` by a rule:

- `ein-infer/tests/detector.rs`, `a_monotone_verdict_is_one_state_per_node_by_fork_and_by_every_split`:
  *scratch departs from the fork* on all eight nodes of the witness, ∅ among them, where the test holds that it
  does on one corpus node, `lattice/11_fifo_gap_static.ein`'s.
- `ein-infer/tests/claim_round_trip.rs`: `ein_infer::claim::check`, as the test calls it (`jobs: 4`, a budget
  of 2¹⁶), panics, *Kb::branch on an unsealed KB — call Kb::seal_top first* (`ein-core/src/kb.rs`), from
  `region::fan_out` → `node_in_region` → `fixpoint_firing`'s `root.branch()`. `claim.rs`'s `fixpoints` calls
  `region::prepare` on the root the description already saturated and hands it to `fan_out` with no
  `root.sealed()`, where `region::compute_prepared` and `explore.rs` seal root after their `prepare`; the
  second saturation writes the mirrored fact, and root's top layer is open. A probe of four programs, written
  for it and deleted after the run, panicked on each of the three that derive the symmetry by a rule (an empty
  `:match` and `:match (near a b)`; a claim that held and one that did not) and *Held* on the one that writes
  the fact. The binary did not panic on the same spliced program: `ein solve FILE`, `ein solve --jobs 4 FILE`
  and `ein test FILE` each printed *(solve :expect) holds*. Why its path differs was not examined.

**The witness was rewritten so as not to rest on this**:
`examples/ein-bugs/wrong-answers/dunder-object-symmetric-by-rule.ein` writes `near` both ways round, `(near a
b) (near b a)`, so the mirror has nothing to add, root's first saturation is a fixpoint, and both sweeps hold
on it. E-1's defect, the dunder among the objects, is the same on either form. A corpus program that derives a
symmetry by a rule over a relation not already closed under it fails both sweeps today.

**Today:** open, not fixed, no register row, no owner. It bears on any ruling that sends a stdlib module to
deriving `__symmetric__` by a closing rule, as `std.math.arith` derives its `__closed__`.

### H9-5 — The shape golden's `hyp+closed` op digests a pass no command calls

Met by S1f.10.9's task 4, 2026-11-16, when the fix of W96 moved 185 of its lines.
`ein-render/tests/golden/corpus_shapes.md5` holds two lines of the enumerator's narration for a corpus program:
`hyp`, the generator at root as every command runs it, and `hyp+closed`, the same after
`ein_infer::closed::emit_closed` has written a `(__closed__ R)` fact for every declared relation no rule can
conclude. `closed.rs`'s own header says where the pass is called from: *it is no command*, its one caller being
`shape::hyp_shape_with` for this op, kept when the walk was deleted. So the line pins the behaviour of a code
path no `ein` run reaches, on every corpus program, and a change to the enumerator re-banks it for programs
whose answers, read-outs and summaries do not move.

It is also how the corpus held E-1 in its goldens while no corpus program's H held a dunder: the pass writes
its facts after the program is sealed, so the registry never holds `__closed__`, the names index holds it as a
head, and under the op the blind enumerator proposed facts about the dunder on every program where the pass
closed something and a relation stayed open (`examples/branching/03_five_hyps_one_alive.ein`: 271 lines before
the fix, 228 after). The census of S1f.10.9's task 2 read H as the commands build it and did not read this op.

**Today:** open, and put to the user as
[Q-M1f.312](../open_questions.md#q-m1f312--what-the-shape-goldens-hyp-and-hypclosed-ops-digest), not ruled.
[S1f.10.16](s1f.10.16_what_the_walk_left.md#t1-the-readers) read both ops on 2026-11-16: `hyp` too runs a
generator no command runs, the walk's, with the one-step lookahead and its kill cache on, whose candidate stream
is H's pass's on all 659 programs and whose lines carry the lookahead's `compile` events on 568, its verdict on
59 and the kill cache's repeat verdict on 29; `emit_closed` closes a relation on 612. By the stage's rule both
ops keep what they call, `emit_closed` and the lookahead among it, the dependence written in `closed.rs`,
`lookahead.rs`, `hypgen.rs` and `shape.rs`; the ops themselves are the user's. The question's default is (b):
`hyp` re-pointed at H's pass, `hyp+closed` removed.

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
command, as H9-3 says of S1f.10.15's. **The ruling was (A)** (2026-11-16), so neither program changes rung and
this finding has nothing left to bear on.

**H6-3 and H6-4 were met by the stage's task 4**
([its record](s1f.10.6_obligations_under_hypothesis.md#record-2026-11-16-task-4)), 2026-11-16, on the release
binary of the fixing commit. Neither is fixed, and no register row is filed.

### H6-3 — The mechanisms check reads root's activators, so a list that takes out what a later debt owes is accepted

`mechanisms::check` refuses a program in which an obligation owes a relation that `:no-hypothesis`,
`:hypothesis-relations` or a closure takes out (O + NH, O + HR, O + C), and reads an obligation as *a plan
root's activators compile*. An obligation whose activator only a commitment derives is not read, so the same
list beside it loads.

The witness with `drinks` in its list, `(solve :no-hypothesis (is-a drinks))`: it loads, H is
`{(likes Ann Soup)}`, and R is empty, every alive node owing; the node that likes Soup owes a drink no commitment
may be about. With `(total-owed drinks is-a)` also stated the program is refused at load: *solve: its
:no-hypothesis names drinks, and obligation 'total-owed', activated by (total-owed drinks is-a) … owes drinks — an
obligation is paid by facts of the relation it owes, so a state that owes drinks and may not be given one is
stuck: drop drinks from the list*. Both runs are the same before task 4 and after it (`ein solve` on the two
programs, the binary of `a3c719bf` and the fixing commit's).

Since task 4 the obligations rung knows that debt: it reads the activator off `arm`, compiles its plan, and
scopes it out by the list, as it scopes out a plan of root's, without the `hypskip` line or the *stuck* report a
debt root owes gets. So the engine now holds, at load, the fact the check would need. **Today:** an empty R of
the owing kind where the twin that states the activator gets a load error that says why. Not a wrong answer by
the definitions: the program says no commitment is about `drinks`. A fix would have the check read the rung's
later debts, which is one more reader of `forbid::derivable_activators`; whether a debt a commitment *can*
bring refuses a program at load, at relation grain and so on the *over* case too, is a ruling.

### H6-4 — `hypgen::eligible_relations` does not name a later debt's relation

`eligible_relations` answers *which relations can this program's generator still propose*, for
`naf_deps`' refutation advisory and for `structure.rs`, and on the obligations rung it is the `(open ?R)`
arguments of the plans root's activators compile. Since task 4 the rung proposes facts of a later debt's
relation too, and that relation is not in the set (the function's doc says so). Reading it there would mean
running the rung's scans, which the advisory's one caller does before root has a hypothesis.

**Today:** no corpus program differs: the one program with a later debt, the witness, has no `absent` for the
advisory to read, and *forbid*, which is what refuses a refutation resting on an `absent` over a relation a
commitment extends, reads `D` from H's own relations and so does see it. `scope::outside`, the read-out's
`outside` row, was given the later relations in task 4 and is right.

## From S1f.10.15

Met by [S1f.10.15](s1f.10.15_the_rung_a_loaded_rule_chooses.md#record-2026-11-15-tasks-1-to-3)'s tasks 1 to
3, 2026-11-15, on the release binary of `c6251db3`. The programs are under the untracked `build/s1f.10.15/`
and verbatim in the stage's record; `build/s1f.10.15/run.sh`, which the record carries, runs them. None is
fixed, and no register row is filed: the stage was told to file none. Task 4, on the ruling of Q-M1f.308, settled
H15-1 to H15-4 and narrowed H15-5, each so marked below, and met H15-6.

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

**2026-11-16, Q-M1f.308 ruled (b) and built ([S1f.10.15's task 4](s1f.10.15_the_rung_a_loaded_rule_chooses.md#record-2026-11-16-task-4)): settled as measured.** `features/09` is on the
third rung, H 82 candidates, all dead alone with a dead root, R empty as before; its read-out prints the `relation
names` row S1f.10.9 built for E-7, *adjacent-via-bwd, adjacent-via-fwd … 44 candidates of colour-loc, pet-loc name
one as an object*, the 44 this finding counted. `stdlib/bool/inputs.ein` keeps its rung, as (b) has it. S1f.10.9's
census was not re-taken whole: of H9-3's twelve, (b) moved only `features/09`, and the read-out names its marks.

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

**2026-11-16, closed by [S1f.10.15's task 4](s1f.10.15_the_rung_a_loaded_rule_chooses.md#record-2026-11-16-task-4)**, on the ruling of Q-M1f.308, (b): the probe is the register's
`W97/connex-import`, `examples/ein-bugs/wrong-answers/rung-connex-import.ein` (the dead-root form, the re-plan's
own), filed `open` with a refusal as its right answer and fixed by the switch: *forbid* refuses `connex`, exit 1,
naming `(solve :no-hypothesis (lt))`, and `ein-infer/tests/refutation_under_absent.rs`'s exact set holds the line.
A program that loaded until then; the commit and the record name it as a change of behaviour. No other corpus
program stopped loading. `connex_lt_alive.ein`, the alive form, is refused the same way and is not banked.

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

**2026-11-16, settled by (b), as built by [S1f.10.15's task 4](s1f.10.15_the_rung_a_loaded_rule_chooses.md#record-2026-11-16-task-4).** The 35 lists filter again: on 32 programs the
summary's `root.hypgen.pre_candidate` now counts the relations they take out, and nothing else of theirs moved.
`features/09`'s header describes its hypothesis space again, and nothing was restated. Its corpus note's *with
the guard back it answers Ambiguity k=4* is the walk's verdict word, not R's count (on the third rung the guard
put back gives 9 states, 4 of them maximal); it is left as written.

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

**2026-11-16, closed by [S1f.10.15's task 4](s1f.10.15_the_rung_a_loaded_rule_chooses.md#record-2026-11-16-task-4)**: the ladder as (b) has it is stated in all five, `06_reserved_names.md`
§ Hypothesis / query control (its prose and its O and B rows), `solution_semantics.md` § 1.4, `AGENTS.md`'s
sentence, `hypgen.rs`'s and `oblgen.rs`'s headers, and in three more places the stage's `grep` found:
[`events.md`](../../../docs/kernel/inference/events.md)'s `rung` section, which said the line is emitted for a
program that declares an obligation rule, `oblgen::Mode::Blind`'s doc and `structure::Stated`'s
`declared`. The switch is one function, `hypgen::rung`, which `generate_with` and `eligible_relations` read.

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

**2026-11-16, narrowed by (b), as built by [S1f.10.15's task 4](s1f.10.15_the_rung_a_loaded_rule_chooses.md#record-2026-11-16-task-4).** An import no longer takes a program off the
third rung, so the case this finding was written for, a module imported for another of its names, needs no way
across. What is left is a program that **activates** an obligation and wants its own open relations guessed
about: it is on the second rung, and the only way to the third is still an obligation nothing owes that declines
it, as `tests/stdlib/bijection/06_blind_enumeration.ein` does; K-11 is that case. Open, no owner.

### H15-6 — `register_region`'s bless writes a region block into a row's quoted text

Met by [S1f.10.15's task 4](s1f.10.15_the_rung_a_loaded_rule_chooses.md#record-2026-11-16-task-4), 2026-11-16,
filing W97. Where a register row states no `[case.region]`, `EIN_BLESS=1 cargo test -p ein-infer --test
register_region` writes the block before the first occurrence of the text `[case.right]` in the row
(`register_region.rs`'s `bless`, a `str::find` over the row's text), strings included. `W97/unactivated`'s note
as first written ended *`[case.control]` and `[case.right].derived` carry the counts*: the block went into the
middle of the note, the register stopped parsing, and every test that reads it failed at its TOML
(`id_order_invariance`'s `the_register_s_id_order_rows_answer_what_they_record` named the line). The note was
reworded and the block placed by hand; the tracked register never held the broken text. The search for an
existing `[case.region]` asks for the header and its newline and is not reached the same way; the search for
`[case.right]` asks for the text alone. Finding the table's header line, a line that is exactly `[case.right]`,
would not be reached either.

**Today:** not a wrong answer: a defect of a re-banking tool, met only by a note that names a table in
backticks. Open, no owner.
