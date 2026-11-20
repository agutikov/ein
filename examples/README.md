# Ein examples

Encoded puzzles and focused fixtures, in [ein-lang](../docs/ein/user/04-reference/). Run any with `ein solve <file>`
(or `ein saturate <file>` for the saturation demos); [`docs/ein/ein.rs/04-api.md`](../docs/ein/ein.rs/04-api.md) drives
them from another Rust program. The human Zebra walkthrough, every clue paired with its fact and every deduction
with its rule firing, is [`06-zebra-walkthrough.md`](../docs/ein/user/02-guide/06-zebra-walkthrough.md).

> **The stdlib's test suite is not here.** One program per `std.*` rule or tight family, each stating what it
> should and should not derive, lives in [`tests/stdlib/`](../tests/README.md): those programs exist to break, and
> nobody would learn ein-lang from them. What *is* here is `features/10` to `13`, which show the form they are
> written in.

## Zebra puzzle — two ontologies, one puzzle

`zebra.ein` and `zebra2.ein` encode the *same* Zebra puzzle over deliberately different vocabularies, both solve to
the same model, and the pair is kept because the comparison is the only way to see which of the engine's reasoning
power is general and which is an artefact of one encoding.

**What pins that they agree cell by cell**
([Q-M1e.25](../docs/decisions/corpus/Q-M1e.25-both-zebra-encodings-are-held-to-the-published.md)):

| test | what it holds |
|---|---|
| `ein-infer/tests/acceptance.rs::the_generic_link_encoding_is_the_unique_solution` | `zebra.ein`'s model places all 25 `GRID` cells through `co-located`, as the one state of R |
| `…::both_ontologies_reach_the_same_model` | `zebra2.ein`'s model is **exactly** those 25 cells, read through the five `*-loc` projections |

`GRID` is the *published* answer, vocabulary-independent, and both encodings are compared against **it** rather
than against each other, which is the stronger claim of the two, because two encodings can agree on a wrong model.

| | [`zebra.ein`](zebra.ein) | [`zebra2.ein`](zebra2.ein) |
|---|---|---|
| attribute link | **one generic** `co-located` equivalence over all 30 atoms, whose classes *are* the houses | **five** projections, a relation an attribute (`color-loc : Color → House`, …) |
| membership | split: `(instance V T)` + `(type Sub Super)` | unified `is-a` |
| a cross-attribute clue | an ordinary fact: `(co-located Englishman Red)` | a 4-ary activator: `(co-located nation-loc Englishman color-loc Red)` |
| a spatial clue | an ordinary fact: `(right-of Green Ivory)` | a 5-ary activator: `(adjacent-via right-of color-loc Ivory color-loc Green)` |
| the property that drives it | scoped by membership: `(slot-partition co-located instance type Attribute House)` + one `(slot-spatial …)` per spatial relation — `std.slots` | per-relation: `(bijective color-loc)` ×5 — `std.bijection` |
| rules defined in the file | 0 (all imported) | `grep -c '^(rule ' <file>` |

`zebra2.ein` is the reference encoding, and the one its variants are generated from; `zebra.ein` is the independent
second reading.

| file | description |
|------|-------------|
| [`zebra2-hints.ein`](zebra2-hints.ein) | `zebra2` with ten hint facts from the published solution: comment hints out to make the search branch more |
| [`zebra2-minus-15.ein`](zebra2-minus-15.ein) | `zebra2` with condition (15) removed: a reduced, under-determined variant |
| [`zebra2-obligations.ein`](zebra2-obligations.ein) | `zebra2` with its hypothesis rule `guess` and the rule's five activators **removed**: the theory alone drives the search, through the facts that would pay what its obligations owe |
| [`zebra2-minus-15-obligations.ein`](zebra2-minus-15-obligations.ein) | both at once: under-determined *and* with no hypothesis rule |

### What an edit to these two files fans out into

`zebra.ein` and `zebra2.ein` are **world anchors**: tests across the workspace hard-code facts about them (counts,
model cells, the answer) and their docs say so. This is the list a reviewer changing either file needs. **Both
puzzle files point at this section** (`world_anchors.rs` holds that they do); nothing else has to be kept in sync by
hand.

Four rings, outermost first:

1. **Four generated files.** `zebra2.ein` is copied whole into `zebra2-minus-15.ein`, `ein-bugs/zebra2-bad.ein`,
   `zebra2-obligations.ein` and `zebra2-minus-15-obligations.ein`, each generated and not edited by hand. **An edit
   to `zebra2.ein` that is not regenerated fails the gate**, an edit that is only a comment included, and
   `cli_semantics` holds the variants to `zebra2.ein`'s relations, rules and facts.
2. **Test files that name one of these paths**, re-derived by
   `world_anchors::the_anchor_list_is_the_greps_own_answer` and banked below. Edit the test, run it, paste — or
   `EIN_BLESS=1` writes it.

<!-- generated: grep -rl 'examples/zebra' ein.rs/crates/*/tests/*.rs ein.rs/crates/*/benches/*.rs -->
```text
ein-cli/tests/acceptance_cli.rs
ein-cli/tests/answer_dump.rs
ein-cli/tests/cli_semantics.rs
ein-cli/tests/corpus_cli.rs
ein-cli/tests/einb_cli.rs
ein-cli/tests/embedding.rs
ein-cli/tests/h_space_view.rs
ein-cli/tests/lever_invariance.rs
ein-cli/tests/region_dump.rs
ein-cli/tests/render_doors.rs
ein-cli/tests/show_hyps.rs
ein-corpus/benches/engine.rs
ein-einb/tests/corruption.rs
ein-einb/tests/invalidation.rs
ein-einb/tests/open_budget.rs
ein-einb/tests/roundtrip.rs
ein-infer/tests/acceptance.rs
ein-infer/tests/agenda.rs
ein-infer/tests/coupling.rs
ein-infer/tests/cover.rs
ein-infer/tests/debts_rules.rs
ein-infer/tests/exactly_one.rs
ein-infer/tests/explain_semantics.rs
ein-infer/tests/first_contradiction.rs
ein-infer/tests/known_dead.rs
ein-infer/tests/naf_semantics.rs
ein-infer/tests/node_coupling.rs
ein-infer/tests/obligation_reports.rs
ein-infer/tests/obligation_rung.rs
ein-infer/tests/order.rs
ein-infer/tests/pair_layer.rs
ein-infer/tests/price_list.rs
ein-infer/tests/region.rs
ein-infer/tests/search_semantics.rs
ein-infer/tests/stdlib_coverage.rs
ein-infer/tests/structure_invariance.rs
ein-infer/tests/theory_share.rs
ein-infer/tests/worker_view.rs
ein-ir/tests/dump_goldens.rs
ein-ir/tests/imports_semantics.rs
ein-ir/tests/kb_semantics.rs
ein-render/tests/dot_wellformed.rs
ein-render/tests/golden_dot.rs
ein-render/tests/idea08_acceptance.rs
ein-render/tests/presentation_semantics.rs
```
<!-- /generated -->

3. **Outside the crates**, which no test can re-derive: `docs/ein/ein.rs/04-api.md` (through `embedding.rs`'s marked
   region) and `corpus/corpus.toml`.
4. **Goldens.** `ein-ir/tests/golden/zebra.golden` and `zebra2.golden`, and
   `ein-render/tests/golden/kb_zebra_unified.dot`, are byte goldens of the two puzzles. **An edit to `zebra.ein` or
   `zebra2.ein` that changes the *parse* moves them**, and `EIN_BLESS=1` re-banks them like any other golden; read
   the diff first. A `;`-comment does not: comments are lexer trivia and reach no golden.

## Feature fixtures (per engine capability)

| dir | what it exercises |
|-----|-------------------|
| [`features/`](features/) | language features: `not` and `absent` in a `:match`, `*` in identifiers, `forall`, `unknown`, the stdlib's domain elimination (`05` keeps `is-a` from being guessed and `12` keeps `instance`, `(solve :no-hypothesis …)`, so neither refutation rests on an `absent` a commitment can fill), the unstratifiable `p ← absent q; q ← absent p` (which pins that the NAF boundary admits **one** candidate per round: a batch would derive both), two `(or …)` disjuncts with different guards and one binding key, and an `adjacent-via` constraint that is not satisfied from the *same* house. **`10_expect.ein` to `13` state their own answers** in an `:expect`, which makes `ein solve` on them a test rather than a demonstration ([claims](../docs/ein/user/03-syntax-and-semantics/12-query.md#claims)): `10` one state, `11_expect_ambiguity.ein` several, `12_expect_false.ein` none, and `13_mixed_solution_and_open.ein` one discharged state beside one that owes, so R is the one. **`ein test examples/features/` checks every file that states a claim** and asks nothing of the others ([`ein test`](../docs/ein/user/04-reference/05-cli.md#23-ein-test)). **`14_unicode_text.ein` and `15_unicode_escapes.ein` show natural-language text**: a string is written only after `:why`, `:source` and `:goal-text`, and it is Unicode with the escapes `\n \t \r \" \\` and `\u{…}`, while a name stays ASCII; `14` is a three-house puzzle in Russian, solved by saturation, and `15` the escapes and text in other scripts. Each states its claim, which the text does not move. **`16_eq_neq_goal.ein` asks about `eq` and `neq` in a goal**: they are relations, computed and never stored, a variable nothing else binds ranging over the program's names, and each of its nine queries states its class; **`17_eq_neq_premise.ein`** writes them in a rule's premise: a guard before the premise that binds its variable, a fact as a guard's argument and a guard whose variable no premise binds, four claims about root. **`18_ein_inconsistent.ein` states `:ein inconsistent`**, a word to the engine where another rule states an `:assert`: the state that holds two kings is dead by the rule's firing, which writes no fact, and R is the other three. **`19_derived_activator.ein` and `20_derived_activator_never.ein` load under a warning**: a rule could derive an activator that binds a relation parameter to a fact, which activates nothing; `19`'s root derives one, and the run says so too, `20`'s never does. The static check flags both, and nothing is claimed of their answers. **`21_import_of_core.ein` imports `std.core`**, the module every program gets, as any module is imported: no second copy, the module last, and the answer the program has without the line. **`22_declared_twice.ein` declares one relation twice and at two counts**: a declaration holds no form to its count, and each count's facts are its own. **`23_a_rule_reads_open.ein` reads an `open` fact**: a rule matches the obligation's `(open ?R)` at root, and a fork drops the `open` fact and keeps what the rule concluded from it, so the static check flags the program by a site of its own, `open-read`; it loads under the warning, and nothing is claimed of its answer. **`24_written_hypotheses.ein` writes its hypotheses as facts**, `(open (s a))` and `(open (s b))`: root holds them and no fork does, the empty commitment's state is root's, and R is ∅ and the two states that hold one |
| [`arith/`](arith/) | **the calculator**: programs over `std.math.*`, arithmetic written in Ein over digit tables and computed by saturation alone, with no search: the universe has no candidate, R is {∅}, and `ein solve`, `ein saturate` and `ein test` read the same answer (`ein-cli/tests/arith_search_free.rs` holds it of every file here). A number is `z` or `(n PREFIX DIGIT)`, in reading order; a request fact `(add X Y)` is answered by `(sum-of X Y S)`. `add.ein`, `sub.ein`, `mul.ein` and `div.ein` are the four operations of `std.math.decimal-arith`, each with a rule of the program's own that reads an answer or chains a request from it; `ein solve examples/arith/add.ein --trace T.md` writes the addition's steps as sentences, the check of its operands among them (a request the theory cannot read, `(add (n 1 2) X)` or `(add foo X)` with no `(let foo …)`, is flagged, `(malformed REQUEST)` or `(undeclared NAME REQUEST)`, and the flag derives `(false)`: [`stdlib/README.md`](../stdlib/README.md) says what is flagged and where). `div_by_zero.ein` is a division by zero: the request and its tagged call each derive the flag `(division-by-zero …)`, the flag derives `(false)`, root is dead, and `ein solve` names the request and its two `let`s as the core; it is the one file here whose R is not {∅}. **Three files are generated**, each a different way a program can be long: `long_numbers.ein` (few requests on operands of up to 100 digits), `many_calculations.ein` (100 requests on short numbers) and `factorial.ein` (a chain from one written fact, 25! by three rules of the program's own, which stops by itself where the comparison answers `eq`); `ein saturate` prints the time of each phase. `num9.ein`, `num9_overflow.ein` and `num9_both_ways.ein` are `std.math.num9`'s, nine digits in one fact computed by wide rules, an overflow flagged `(overflow REQUEST)`, and both modules in one program through the conversion requests. `int.ein` is `std.math.int`'s, an integer `(int P N)` with its value P − N, the three division conventions side by side; `rat.ein` is `std.math.rat`'s, a rational `(rat Z D)`, an answer unreduced and `rat-simplify` asked for lowest terms; `gcd.ein` is `std.math.gcd`'s, the largest square tile for a floor and two buses that leave together again |
| [`beyond_arith/`](beyond_arith/) | **what the calculator's tables do besides arithmetic** ([`08-word-problems.md`](../docs/ein/user/02-guide/08-word-problems.md) says what each shows). Eighteen programs, each with a claim, beside `arith/` and not in it because five of them **search**. **Tables read backwards, by saturation alone:** `inverse_addition.ein` (x + 4567 = 127645 gives x = 123078 with nothing guessed), `tables_backwards.ein` (a column with two answers, the next column ending the wrong one), `equation_refuted.ein` (x × 7 = 862 refuted at root, whose core is the proof that it has no solution). **Equations by search,** the unknown's digits as hypotheses under `total-owed` and `functional`: `equation_by_search.ein`, `equation_two_unknowns.ein` (3x + 5y = 47, three states), `equation_no_solution.ein` (R empty because every alive node owes, its claim the owing frontier), `cryptarithm.ein` (I + BB = ILL), `send_more_money.ein` (SEND + MORE = MONEY, its claim held by `ein test`, which asks `--single`'s question) and `send_more_money_written.ein`, **generated** from it and not edited by hand, the same program with its letters declared as the puzzle is written: one universe and one answer, and `--single` doing the same work on both, since the obligation tree orders its branches by the columns it reads off the program and not by the declaration. **Bounded theorem checking:** `laws.ein` (commutativity, associativity, distributivity and the division identity on every instance over 0 to 3) and `column_lemma.ein` (each of the full adder's 200 rows against addition derived by counting). **Number theory by saturation,** each a chain of `std.math.decimal-arith`'s tagged calls: `primes.ein`, `gcd.ein` (importing `std.math.gcd`), `casting_out_nines.ein`, `perfect.ein`, `collatz.ein` (27 stopped at its bound), `pythagorean.ein` and `fibonacci.ein` |
| [`beliefs/`](beliefs/) | **a fact as another fact's argument, and when its content is true**: `01_common_belief.ein`, beliefs as `(believes PERSON FACT)`, the content an object no state holds, and one rule per shape of content saying that what every person believes is true (`forall` over a closed `person`; `(solve :no-search)`), two claims; `02_packing.ein`, the opposite: a rule packs a fact the state holds into another fact's argument, `(knows Ann (likes Dan Soup))`, no new name made, and the fact a rule packed joins the same fact a file writes under an arrow, three claims. Beside each of the two, its saturated KB drawn by `ein render kb` (`NN_….kb.dot`, `.kb.svg`; regenerate with `ein render kb examples/beliefs/NN_….ein > NN_….kb.dot && dot -Tsvg NN_….kb.dot -o NN_….kb.svg`). **The negation of a negation** (`(not (not g))` is a fact): `03_liar_denial.ein`, a liar's rule `(not ?f)` over a negative statement builds one, and nothing concludes the fact under it; `04_liar_denial_contradicted.ein`, the same beside the denial, dead; `05_liar_denial_beside_the_fact.ein`, beside the fact, alive; `06_double_negation.ein`, the elimination as a rule: `std.negation`'s, one rule a count with the relation a parameter, activated for one relation, and the same rule wrapped in a context, written by the program. **A fact in the global context and in a fact's**: `07_three_cases.ein`, one fact held, in a context, or both, and the two questions; `08_global_and_wrapped.ein`, a global rule beside the same rule wrapped by a macro, neither reading the other's facts; `09_false_in_a_context.ein`, the context a parameter, `(false)` concluded in a context with root alive, and `10_everybody_contradicts.ein`, where a `forall` over the contexts' `(false)` refutes root; `11_nothing_acts_in_a_context.ein`, a declaration, an activator, `(false)` and a denial under an arrow doing nothing |
| [`binary_arithmetic/`](binary_arithmetic/) | **arithmetic on binary numbers** ([README](binary_arithmetic/README.md)): five search-free programs over `std.math.binary-arith` and `std.math.convert`, their claims computed independently: `first.ein`, 101 + 111 = 1100; `five_operations.ein`, decimal's example operands in binary; `div_by_zero.ein`, `std.math.arith`'s flag and `(false)`; `both_bases.ein`, both bases in one program; `conversions.ein`, to 255 and from 1000 |
| [`binary_logic/`](binary_logic/) | **things to do with a circuit** ([README](binary_logic/README.md)), each a program over the stdlib's `std.bool` modules ([`stdlib/README.md`](../stdlib/README.md)): `full_adder.ein`, five gates, a module the programs import; `simulation.ein`; `truth_table.ein`, R the eight assignments and the query's classes the rows; `adder_against_table.ein`, the gate-built adder against `std.math.bits`' `bin-fa` on eight test vectors, no search, which `ein-infer/tests/bool_theory.rs` holds too; `satisfiable.ein`, one state; `unsatisfiable.ein` and `equivalent.ein`, **R empty because every alive node owes**, their claims the owing frontier (`Open` under `--single`); `not_equivalent.ein`, two counterexamples; `knights.ein`, two states and one answer for C; `synthesis.ein`, `Or` and `Xor` fit the missing gate; `simplify.ein`, three laws read from `std.bool.props`; `post.ein`, Post's criterion on seven sets. Their claims are computed independently, by enumeration |
| [`word_problems/`](word_problems/) | **elementary-school word problems, the calculator among other relations**: examples and not a stdlib module, theory modules beside the problems that import them by file name; [`word_problems/README.md`](word_problems/README.md) has the vocabulary and the table of answers, and [`08-word-problems.md`](../docs/ein/user/02-guide/08-word-problems.md) the guide. `words.ein` is what *has*, *gave*, *together*, *more*, *holds*, *of*, *shared* and *need* mean, as rules: a plan backwards from the question, `(wanted Q)` and `(needs Q Q')`, then a request to `std.math.decimal-arith` a word and its answer read back as `(value Q N)`, so nothing is computed that no question needs. `gaps.ein` names a missing quantity, by `absent`; `readings.ein` holds the two readings of *need* as hypotheses, a reading always owed; `listed_is_all.ein` is *what is not listed is zero*, an assumption a problem adopts for `has` by importing it. `boxes12.ein` is *Bob has 5 apples, Alice has 7, a box holds 3, how many boxes do they need?*: R is two states, a reading each, and one answer; `boxes13.ein` is the same with 13 apples, two states and two answers. `gap.ein` names the gap a question leaves, and `gap_fact.ein` and `gap_assume.ein` close it by a sentence and by a marked assumption; `contra.ein` gives away 7 of 5 and is refuted, the core its two sentences; `kinds*.ein`, `pencils.ein`, `more.ein`, `unused.ein` and `bystander.ein` are the other shapes. `before.ein` asks *how many did Bob have before he gave 2?*: the right answer is 7, and the theory, which reads *gave* one way in time, names the quantity as not stated |
| [`branching/`](branching/) | the hypothesis layer: saturate-only, dead and alive branches, two levels, a mini Zebra, a hypothesis-relation list, a hypothesis rule, a blind solve over declared relations. `06`/`07`, `10`/`11` and `15`/`16` are three programs each written twice, under the names of a lever that is not in the language; each pair gives one answer. `13` and `14` each conclude `(false)` from an `absent` over a relation a commitment can still extend, so neither is monotone: each loads, is flagged and answered with the warning, and nothing is claimed of its answer |
| [`saturation/`](saturation/) | per-rule saturation demos by family: symmetric, transitive, `implies`, square forward, backward and unique, type-exclusivity, hypothesis-contradiction ([`saturation/README.md`](saturation/README.md)) |
| [`lattice/`](lattice/) | **the commitment lattice and the region R**: subset pruning, a genuine 3-set death, a state-hash collision; two region fixtures, `04_obligation_trigger_hole.ein`, a hole in R with no `absent` in any saturation rule (an obligation's trigger grows with the commitment), and `05_owed_seats_bounded.ein`, `owed-seats` with both upper bounds, whose region is two isolated points; seating problems, `06_friends_tables.ein` (a group that does not fit goes elsewhere; the obligation is the group's, so the program is not monotone), its monotone twin `06_friends_tables_everyone_sits.ein` and `07_two_pairs_two_tables.ein`; one fixture per **cause** of a non-monotone program: `08_absent_witness_hole.ein` (an `absent` that takes a witness away), `04` (an obligation's trigger, the `obligation-trigger` site, beside the `hypothesis` sites of its two hypothesis rules and the obligation's `obligation-guard` site, each over the same `is-a`), `09_activator_hole.ein` (a rule derives an activator, so a commitment compiles a plan root never had) and `12_activator_chain_hole.ein` (`09`'s cube one activator further from root, where the status line names the relation that derived the activator), each of `08`, `09` and `12` the program's only site; and `17_flagged_region_claim.ein` (`04`'s program stating a region claim, which `ein test FILE` checks for whoever asks and no suite does); `10_fifo_gap.ein` and `11_fifo_gap_static.ein`, where a node built at once and one built by way of a smaller commitment hold different facts ([monotonicity § 8.3](../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md#83-the-fifo-gap)); `13_witness_of_two.ein`, a debt one candidate pays alone and two others pay together; `14_seats_nothing_proposes.ein`, a debt some of whose witnesses the hypothesis rule's guard leaves out, which the obligation writes beside the rule's, so H is their union, 16 candidates by `ein show hyps` ([Q-M1f.570](../docs/decisions/hypotheses/Q-M1f.570-h-is-the-union-of-the-hypotheses-root-holds.md)); `15_union_label_dead.ein` and `16_pair_through_closure.ein`, extensions that die without closing a coupling edge through their last candidate (`ein-infer/tests/no_edge.rs`). **`18_two_candidates.ein` and `18_two_candidates_as_rules.ein` are one lattice twice**: two candidates, a clash and a requirement, searched by the engine, its claims held by `ein test` and the oracle; and the same lattice written as rules, every state a world `(in W F)`, the whole lattice one saturation under `(solve :no-search)`, each verdict a fact about a world, whose claims, held by `ein test`, state the dead world, the one that owes and R's two as the oracle answers the first. **`synth/`** holds programs **written for a chosen region**, each carrying its target in its header, which `ein-infer/tests/region.rs` holds the oracle to node for node: generated, regenerate, never edit. `three_components.ein` and `trigger_hole.ein` are kept as they were written, their dead holes resting on an `absent` the hypothesis rule can fill: neither is monotone |
| [`domain_elim/`](domain_elim/) | one exactly-one puzzle written three ways, the answer reached by elimination at root or by a commitment ([`domain_elim/README.md`](domain_elim/README.md)) |
| [`syntax/`](syntax/) | seventeen **node-kind probes**, one per shape the renderers draw and no puzzle contains: the two `=` arities, an arity-0 relation, every argument kind (a name, digits, decorators, a nested fact and a stored negation), stored negation over four shapes, the `is-a` subject positions, `(relation …)` at every arity as a nested fact, a full `(query …)`, a chained `(trace …)` and an empty one, `(config …)`, wrapper heads as ordinary facts, five rule shapes (nested `absent`, `forall`/`not`/`eq`, top-level `or`, disjoint guard sets), and the constraint-scope markers. The eighteenth, the two half-specified declarators, is `broken/load/rule_half_declarators.ein`, because it is a load-negative as well as a probe. As corpus entries the manifest digests them under **every** view and every op |

## Diagnostics & negative fixtures

| dir | what it holds |
|-----|---------------|
| [`ein-bugs/`](ein-bugs/) | **regressions**: programs on which the engine once answered wrongly or failed, each stating the right answer in an `:expect` or kept as the witness of a refusal, and the programs that are not monotone, of whose answer nothing is claimed. `zebra2-bad.ein`, an injected-fact contradiction; `closed-and-owing.ein`, an obligation on a relation `infer-closure` closes, refused at load; `mixed-type-hypothesis.ein` and `nested-fact-hypothesis.ein`, candidates `Value`'s total order compares across shapes; `unbound-relation-head.ein` and `unbound-assert-var.ein`, `(?R ?x)` with `?R` unbound and an `(or …)` whose arms bind different variables, both refused; `query-goal-free-head.ein` and its `-unsat` twin, a goal whose relation is a variable; `int-goal-binding.ein` and `fact-goal-binding.ein`, a binding that is a name of digits and one that is a nested fact; `alive-empty-phase1.ein`, `alive-empty-interlayer.ein` and `complete-records-stale.ein`, states a program's own rules refute, each `:expect` the right answer; `alt-cap-core.ein` and `alt-cap-core-reordered.ein`, one rule's written place apart, which report one core, the smallest over every derivation ([Q-M1e2.11](../docs/decisions/answer/Q-M1e2.11-a-core-is-the-smallest-over-every-derivation.md)); `naf-upward-closure.ein`, on which `dead` is not upward-closed under `absent`, so the static check flags it ([Q-M1f.564](../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)); `alive-set-fresh-name.ein` and its `-declared` twin, one fact apart, which describe one R, since a rule's constant is a name of the program; `d3-goal-row-order.ein`, a model that satisfies the goal twice, its rows printed sorted; `nested-core.ein`, a core holding a nested fact; the `binding-key-*` family, a rule application's identity being its activator's every argument; `unsat-core-id-space.ein`, `d3-unsat-core.ein` and `fork-delta-order.ein`, cores that do not move with the interner; the `activator-text-*` pair, refused at the parse, a string being no value; `activator-all-int.ein`, `activator-all-nested.ein` and `activator-all-symbol.ein`, activators whose every argument is a name of digits, a nested fact or a symbol, which the static check reads alike; `minimal-line-stated-obligation.ein` and its `-control`, an obligation root discharges and none: the minimal line is ∅ on both, and its row says nothing of an obligation |
| [`ein-bugs/wrong-answers/`](ein-bugs/wrong-answers/) | **witnesses of answers once wrong**, each a program of a few lines: what each answers and what it should answer is a row of [`corpus/regressions.toml`](../corpus/regressions.toml), not of this page |
| [`broken/`](broken/) | **parse-failure** fixtures; each carries its report in a `.expected` beside it, every error recovery reaches with its line, column, notes and ranked repairs: a bare top-level name, a keyword where an object goes, a rule missing its parameters, an unclosed paren, `eq` and `absent` at a wrong arity, the config and query keyword refusals, the reserved declarator names, `(open …)`'s shapes, a malformed relation, a top-level `(region …)`, an empty `absent`, the `expect_*` shapes, and a keyword pair below a form (`kwpair-below-the-filter.ein`) |
| [`broken/load/`](broken/load/) | **load-failure** fixtures: files that parse and then fail the loader (`ein_ir::load`); each carries the exact message in a `.expected` beside it ([`broken/load/README.md`](broken/load/README.md)) |
| [`broken/compile/`](broken/compile/) | **compile-failure** fixtures ([`broken/compile/README.md`](broken/compile/README.md)). No program reaches a compile refusal: the parser and the loader refuse every shape that would. `activator_arity.ein` is the one file, and the odd one: its error is unreachable through the engine by design, so the file solves and derives nothing, which is what it pins |
