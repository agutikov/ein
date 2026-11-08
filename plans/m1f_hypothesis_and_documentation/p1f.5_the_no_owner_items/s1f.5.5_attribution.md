# S1f.5.5 T9 — every difference between the two parsers, attributed (T1f.5.5.9)

**Stage:** [S1f.5.5](s1f.5.5_the_grammar.md), [§ T9 record](s1f.5.5_the_grammar.md#t9-record-2026-11-08). **Taken**
2026-11-08 on `git archive` snapshots of `efd2cc74` (HEAD, with T3, S1f.5.15 and T8 landed), beside
[the census](s1f.5.5_census.md) and [the paper grammar](s1f.5.5_paper_grammar.md). Nothing under `ein.rs/`,
`corpus/`, `examples/`, `tests/`, `stdlib/` or `docs/` moved. **Three release binaries**, each answering on
`EIN_STDLIB` the checkout's:

| | what it is | where (gitignored) |
|---|---|---|
| **today** | the snapshot as committed: the hand-written parser | `build/s1f.5.5/t9/base` |
| **the generated build** | the swap prototyped, three files: the `generated-parser` feature on by default, so every command reads its program through the generated parser; `ein_core::text::is_symbol` the name rule; every name a JSON string in `summary.json`'s bindings (Q4). Its diff is T4's starting point | `build/s1f.5.5/t9/proto`, `build/s1f.5.5/t9/swap-prototype.patch` |
| **the name-rule build** | [S1f.5.4's step-1 prototype](s1f.5.4_digits_are_names.md#step-1--every-move-listed-2026-11-08) ported to the snapshot (the name rule in the hand-written lexer, its printer; the `:priority` guard dropped, there being no `:priority`) and Q4's writer: **the name rule alone**, the counterfactual S1f.5.4's credit is read off | `build/s1f.5.5/t9/namerule`, `build/s1f.5.5/t9/namerule.patch` |

The scripts are `build/s1f.5.5/t9/scripts/` ([§ Re-taking it](#re-taking-it)), S1f.5.4's step-1 scripts extended
to three binaries and to `ein test`. **The review material for the user's stop is `build/s1f.5.5/review/index.md`**
(gitignored; 207 programs, every artefact both binaries write, raw and rendered), and its § 0 is the stop's
questions.

## What it found

1. **Every difference is attributed; none stops the swap.** Of the **513** `.ein` files under `examples/`,
   `tests/`, `stdlib/`, `corpus/` and `plans/` (`find … -name '*.ein' | wc -l`, 2026-11-08), 346 answer as they
   did on every command and **167 move**; each is credited to one ruling or more, and **0 to none**:

   | ruling | files | what it moves |
   |---|---:|---|
   | S1f.5.4, the name rule | 49 | the 45 the name-rule build moves, and 4 whose tree alone moves |
   | the paper grammar's own | 48 | refusals that move up from the loader and the compiler, the 11 files both parsers refuse (the error text), defaults 2, 3 and 7, G-14 (Q-M1f.1 (b)), and three fixtures that import a refused one |
   | S1f.5.15, `:priority` gone | 43 | records under `plans/` that S1f.5.15's T5 left as written |
   | S1f.5.2, strings are natural language | 28 | a string as a value, an escape outside the set, a variable in a fact |
   | S1f.5.11, closed keywords | 28 | a keyword its form does not take, and any inside a term or a nested fact |
   | S1f.5.1, a bare `_` refused | 24 | `_` and `?_` in every position |

   The generated parser refuses 142 of the 167 (131 that today's accepts) and accepts 25 (4 that today's refuses).
   Exit codes that move: `saturate` 0 → 1 on 91 files and 1 → 0 on 4, `solve` 0 → 1 on 83 and 1 → 0 on 3, `test`
   0 → 2 on 23, 1 → 2 on 4 and 0 → 1 on 4 (the `binding-key-*` witnesses, whose `:expect` the name rule refutes).
2. **Cross-check 1 holds as meant, and read literally fails on one file.** The name-rule build moves **45 of
   S1f.5.4's 46**, and nothing outside them. The 46th, `ein-bugs/wrong-answers/assert-empty.ein`, moved in step 1
   only because its candidate printed `(q @empty)` bare, which was step 1's finding; T8 renamed the name `#empty`
   (Q1 (a), `d7df861d`), which neither printer prints bare, so it moves under neither build. It is put to the user
   at the stop.
3. **The generated parser's name rule is the prototype's, measured.** On the 20 of the 45 that the generated parser
   accepts, the generated build answers **byte for byte as the name-rule build** on every command; the other 25 are
   refused for a string, a `:priority` or a keyword, the combined grammar's answer, credited to both rulings.
4. **Cross-check 2 holds.** The generated parser refuses **all of the census's 55**; the 14 other corpus files it
   refuses that today's parser accepts are witnesses filed after the census, each for its row's shape: W29's six
   (S1f.5.1), W32's seven (S1f.5.11) and W35 (default 7).
5. **The parse itself** (both parsers and the name-rule parser on every file and on 40 000 census mutants,
   [§ The mutants](#the-mutants)): on files, 345 the same tree `Loc` for `Loc`, 22 the same but for digits, 131
   refused by the generated parser alone, 4 by today's alone, 11 by both; the name-rule parser against the
   generated one, **0 trees that differ**. On mutants the generated parser accepts only what the name-rule parser
   accepts, with its tree, on every one.
6. **`cargo test --workspace` on the snapshot** (2026-11-08, 21 min 19 s): **834 passed, 109 failed, in 39 of 107
   test binaries**, each the move of a ruling ([§ The tests T4 moves](#the-tests-t4-moves)) but one, the generator's
   own cost: a `.einb` cold open re-parses its program, and `zebra2` opens in 3.7 ms on the generated build against
   0.76 to 1.0 ms today (release), past design/10 § 6's millisecond. **47 are `ein-ir`'s**, T8's 47, named. Blessed, the
   goldens that move are [§ The goldens T4 moves](#the-goldens-t4-moves).
7. **The register: 37 of 139 rows move, exactly the stage's list**: 14 `open` rows flip to `fixed` (W29's six,
   W32's seven, W35) and 23 `fixed` rows are restated (W7, W15 ×2, W16 ×3, W18 ×5, W20 ×5, W21, W24 ×3, W28 ×3).
8. **Found here, each a report and not a stop**: a `:why` or `:goal-text` template cannot name a variable whose name
   holds a character other than a letter, a digit, `_` or `-` (`{?isR*}` unfilled in three stdlib templates
   today, and every variable the name rule adds after the swap), **a wrong answer with no register row**;
   `utils/region_synth.py`, `guard_split.py` and `fuzz_engine.py` still write `:priority`, which two gate tests
   read through the first; W34's printed region no longer reads back as a claim; `(solve :hypothesis-relations
   co-located)` without its list, which a test pins and no corpus program writes, is refused; and `value_label`'s
   `KwPair` arm is reached by no corpus file; and a `.einb` opens past its millisecond. Each is in the stage's [§ T9 record](s1f.5.5_the_grammar.md#t9-record-2026-11-08)
   with its recommendation.

## How a file is credited

A ruling is credited to a file when **that ruling alone moves it** (`build/s1f.5.5/t9/scripts/attribute.py`):

- **S1f.5.4**, when the name-rule build moves it: the counterfactual, measured rather than inferred; or, where no
  answer moves, when the generated parser's tree differs from today's by digits read as names.
- **S1f.5.1, S1f.5.2, S1f.5.11, S1f.5.15**, when the generated parser refuses a token that ruling refuses: `_` or
  `?_`; a string, an escape outside the set, or a variable in a fact; a keyword its form does not take; a
  `:priority`. Recovery reports every error, so every such token is seen; an error recovery's own repair causes
  later in the file is not credited (a *cascade*, named in [`attribution.tsv`'s notes](#every-file)).
- **The paper grammar's own**, for a shape the grammar refuses by position (the first error), for the error text of
  a file both parsers refuse, for default 2's `:using`, and for a fixture that imports a refused one.
- **C1, C2** read as written: `_x`, `__`, `?_x` are the name rule's (no corpus file writes one), the bare `_` and `?_`
  S1f.5.1's; a keyword error is never S1f.5.4's. **Where two rulings meet, the combined grammar's answer is
  attributed to both**: `activator-text-int-and-string` moves under the name rule for its `1` and is refused for its
  `"1"`, so it is S1f.5.4's and S1f.5.2's, and the refusal is its answer.

A variable the generated parser refuses is S1f.5.2's where it stands in a fact (G-6) and the paper grammar's where
it stands where no variable goes (a claim, `absent`'s argument, a whole conclusion); the 9 files that write one are
read by hand (`attribute.py`'s `VAR`).

## The mutants

`ein_corpus::mutants`, the census's generator, seed 0, over today's corpus files under 6 000 bytes, read by the
three parsers, with today's loader run wherever today's parser accepts (`build/s1f.5.5/t9/scripts/mutants.py`, 13 s
for the three):

```text
40000 mutants (ein_corpus::mutants, seed 0, the census's state) of the corpus files under 6 000 bytes
  21301  A no difference: all three accept, one tree
   1191  B1 S1f.5.4: digits are a name, the same tree otherwise
     32  B2 S1f.5.4: a word today splits is one name
    616  B3 S1f.5.4: today refuses, the name rule and gen accept one tree
    746  C S1f.5.4: the name rule refuses it, and gen
   6446  D gen alone refuses it
    381  E S1f.5.4 makes it parse, gen refuses it
   9287  F all three refuse: the error text (own)

C, D and E by the generated parser's first error, and the ruling it is credited to:
    678  C S1f.5.4   a token that is not a name                       e.g. examples/saturation/square-unique/cul-de-sac.ein: 34:41 unexpected `neq?y`: a token that is not a name |                (absent (and (?R ?x ?z) (neq?y ?z))))
     16  C S1f.5.11  a keyword no form takes                          e.g. examples/ein-bugs/wrong-answers/closure-shared-given-control.ein: 20:14 unexpected `:k`: a keyword this form does not take | (rule x-uv ():k v:match (and (u ?k) (v ?k)) :assert (x ?k))
     15  C S1f.5.4   a string glued to what follows                   e.g. corpus/fuzz_findings/binding-key-layout-swapped.ein: 5:11 unexpected `"a"dge`: a string glued to what follows | (relation "a"dge  Node Node)
     11  C S1f.5.11  a keyword this form does not take                e.g. examples/broken/load/import_as_and_symbols.ein: 3:20 unexpected `:as` | (i-mport std.macro :as m :symbols (foral:k vl))
      8  C S1f.5.2   a string                                         e.g. examples/ein-bugs/activator-text-string-then-int.ein: 25:19 unexpected `"1"`: a string | (holds 1)  (holds "1")
      7  C own       `absent`, a word a position gives a meaning to   e.g. examples/broken/load/hrule_reserved_name.ein: 3:8 unexpected `absent` | (hrule absent () :match (x?a) :assert (y ?a) :why "w")
      7  C own       a parenthesis a position does not take           e.g. examples/saturation/symmetric/friends.ein: 21:15 unexpected `(` | (relation frie()d-of T T)
      2  C own       a name a position does not take                  e.g. examples/saturation/type-exclusivity/colors.ein: 24:1 unexpected `1`: a name | 1               (neq ?a ?b))
      1  C S1f.5.1   _ (reserved)                                     e.g. examples/ein-bugs/wrong-answers/underscore-core-two-ways.ein: 34:26 unexpected `_`: _ (reserved) | (rule w () :match (holds _) :assert (hit w))
      1  C S1f.5.2   a variable in a fact (G-6)                       e.g. examples/saturation/implies/right-then-next.ein: 21:15 unexpected `?p`: a variable | (rul implies (?p ?q)
   1580  D S1f.5.11  a keyword no form takes                          e.g. examples/broken/load/expect_unknown_keyword.ein: 8:23 unexpected `:expct`: a keyword this form does not take | (query :goal (p A ?h) :expct (model (p A H1)))
   1207  D S1f.5.2   a string                                         e.g. corpus/fuzz_findings/activator-text-key-swapped.ein: 14:19 unexpected `"1"`: a string | (holds 1)  (holds "1")
   1018  D S1f.5.1   _ (reserved)                                     e.g. examples/ein-bugs/wrong-answers/wild-in-guard.ein: 35:38 unexpected `_`: _ (reserved) | (rule e () :match (and (p ?x) (eq ?x _)) :assert (hit-eq ?x))
    792  D own       a parenthesis a position does not take           e.g. examples/ein-bugs/wrong-answers/match-only.ein: 54:12 unexpected `)` | (rue no-b ()
    527  D own       a name a position does not take                  e.g. examples/ein-bugs/binding-key-swapped.ein: 42:11 unexpected `01`: a name |           01  (model (holds 1) (holds 3) (holds Y)
    490  D S1f.5.11  a keyword this form does not take                e.g. corpus/fuzz_findings/kwpair-below-the-filter.ein: 1:18 unexpected `:rule` | (tracex (step s1 :rule from-condition :using (c10) :derives (lives-in N H1))
    186  D S1f.5.2   a variable in a fact (G-6)                       e.g. examples/broken/load/macro_duplicate.ein: 3:10 unexpected `?a`: a variable | (macr m (?a) (rel ?a))
    181  D own       `open`, a word a position gives a meaning to     e.g. examples/broken/load/reserved_open_direct.ein: 16:8 unexpected `open` | (macro open (?x) (p ?x))
    154  D own       a variable in a name list, a claim or an absent  e.g. examples/broken/load/empty_absent.ein: 17:31 unexpected `?x`: a variable |   :match  (and (p ?x) (absent ?x))
    150  D own       `absent`, a word a position gives a meaning to   e.g. examples/broken/load/hrule_reserved_name.ein: 3:8 unexpected `absent` | (hrule absent () :match (x ?a) :assert (y ?a) :why "w")
     79  D own       `region`, a word a position gives a meaning to   e.g. examples/broken/load/region_top_level.ein: 11:2 unexpected `region` | (region (model (p A)))
     67  D own       `eq`, a word a position gives a meaning to       e.g. examples/broken/load/relation_reserved_name.ein: 4:11 unexpected `eq` | (relation eq)
     15  D S1f.5.2   a string with an unknown escape                  e.g. stdlib/typing.ein: 36:11 unexpected `"typecheck converse {?R1}/{?R2} unde\ hi…`: a st |   :why    "typecheck converse {?R1}/{?R2} unde\ hierarchy {?isR*}.")
    108  E S1f.5.2   a string                                         e.g. examples/syntax/arg-kinds.ein: 13:6 unexpected `"a string"`: a string | (r 7 "a string")
    100  E own       a name a position does not take                  e.g. tests/region/02_merged_single.ein: 66:10 unexpected `$`: a name |          $   (merged (model (h A) (h B) (h D) (h E))
     47  E S1f.5.11  a keyword this form does not take                e.g. tests/stdlib/algebra/22_symmetric_negative.ein: 65:3 unexpected `:goal` |   :goal (sits-with ?x ?y))
     42  E own       a parenthesis a position does not take           e.g. examples/ein-bugs/d3-unsat-core.ein: 35:16 unexpected `)` | (_rule fire-1 ()
     31  E S1f.5.2   a variable in a fact (G-6)                       e.g. examples/ein-bugs/wrong-answers/closure-shared-given-control.ein: 13:76 unexpected `?_23`: a variable | (c C19) (d C19) (c C20) (d C20) (c C21) (d C21) (c C22) (d C22) (c C23) (d ?_23) (c C24) (
     24  E S1f.5.11  a keyword no form takes                          e.g. examples/lattice/09_activator_hole.ein: 35:3 unexpected `:wh`: a keyword this form does not take |   :wh    "{?k} is open, so every guest owes a meal")
     12  E own       a variable in a name list, a claim or an absent  e.g. examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed-declared.ein: 64:27 unexpected `?xAnn`: a variable |             (model (likes ?xAnn Z) (likes Soup Ann) (likes Soup Z) (likes Z Ann))
      7  E S1f.5.1   _ (reserved)                                     e.g. examples/ein-bugs/wrong-answers/wild-in-guard.ein: 35:38 unexpected `_`: _ (reserved) | (rule e () :match (and (p ?x) (eq ?x _)) :assert (hit-eq ?x))
      7  E own       `open`, a word a position gives a meaning to     e.g. examples/broken/load/open_in_match.ein: 6:49 unexpected `open` | (rule bad (?R) :match (and (relation ?R ?A ?B) (open ?R)) :assert (r a ..))
      3  E own       `absent`, a word a position gives a meaning to   e.g. examples/broken/load/macro_reserved_name.ein: 3:8 unexpected `absent` | (macro absent (?p) (rel -p))

D and E where today's parser accepts, by what today's loader does with it:
    739  S1f.5.1   today loads it
    279  S1f.5.1   today's loader refuses it
   1113  S1f.5.11  today loads it
    957  S1f.5.11  today's loader refuses it
   1277  S1f.5.2   today loads it
    131  S1f.5.2   today's loader refuses it
    728  own       today loads it
   1222  own       today's loader refuses it
```

The name rule's share (B, C and the first half of E) is exact, being where today's parser and the name-rule parser
differ. Of the 728 that today's parser and loader accept and the generated parser refuses for a shape, 282 hold `()`
as a fact's argument (default 7), 84 `:using (x)` (default 2), 77 `(eq ?x)` and 68 `(eq ?x A B)`, 75 `(absent)` and
64 `(absent P Q)` (the compiler's arity refusals, moved up), 71 `(not plain)` (default 3) and five single ones, each
a position the paper grammar states.

## The tests T4 moves

`cargo test --workspace --no-fail-fast` on the snapshot with the swap prototyped (`build/s1f.5.5/t9/proto-test.log`,
2026-11-08): 834 passed and 109 failed, in 39 of 107 test binaries. One row a failing test, with the ruling that moves it
and what T4 does with it. **The 47 `ein-ir` rows are T8's 47**: the same count, each a suite in today's language
read through the generated grammar.

| crate | target | test | ruling | why it moves | what T4 does |
|---|---|---|---|---|---|
| ein-cli | `answer_dump` | `every_recorded_state_is_a_fixpoint_of_its_own_program` | S1f.5.2 | the four `activator-text-*` witnesses write `"1"` and are refused | the fixtures as § 0 item 2 settles; the sweep reads what loads |
| ein-cli | `answer_round_trip` | `the_witnesses_reload_to_their_states` | S1f.5.2, S1f.5.4 | four of its five witnesses write a string as a value, the fifth is `arg-kinds` | witnesses restated to programs in the final grammar whose names exercise the printer (`7` beside `07`, decorators, brackets) |
| ein-cli | `cli_semantics` | `a_conclusion_no_fact_can_hold_is_one_line_in_eins_spelling` | own (Q-M1f.1 (b)), S1f.5.11 | the conclusions no fact can hold are parse errors, fired or not | restated to the parse error |
| ein-cli | `cli_semantics` | `a_range_is_no_token` | S1f.5.4 | `(r 1..5 a)` loads: `1..5` is a name | its plain-name counterpart, as the stage says |
| ein-cli | `cli_semantics` | `a_rule_shape_that_reads_nothing_is_refused_at_load` | S1f.5.1, S1f.5.11, own | W24's shapes are parse errors | restated to the parse error; the loader lines go in T5 |
| ein-cli | `cli_semantics` | `a_load_failure_is_one_diagnostic_line_and_a_non_zero_exit` | own (default 2, the error text) | `derivation_cycle` is refused at the parse, with every error | T10 rewrites the fixture (`:using ((r c d))`), and its loader line comes back |
| ein-cli | `cli_semantics` | `an_integer_goal_binding_stays_a_json_number` | S1f.5.4 (Q4) | every name is a JSON string | restated: `?y = "8"` |
| ein-cli | `cli_semantics` | `a_wildcard_relation_in_a_rule_is_refused_at_load` | S1f.5.1 | `(_ 2)` is a parse error | restated to the parse error |
| ein-cli | `cli_semantics` | `render_views_the_ir_and_never_reports_a_load_error` | own (default 2) | `derivation_cycle` does not parse | T10's rewrite |
| ein-cli | `config_reference` | `the_retired_flags_are_refused_with_why` | S1f.5.11 (default 5) | a retired `config` flag is a parse error | restated; default 5's note names the stage that retired it |
| ein-cli | `config_reference` | `require_why_decides_loading_only` | the T3 ruling (a macro takes a `:why`) | the stdlib's two macros carry no `:why`, which `--require-why` asks of a macro on the generated parser | the stdlib's `forall` and `unknown` get their `:why`, `MANIFEST.sha256` with them, and the `cfg` arms go (the stage's T4 bullet) |
| ein-cli | `corpus_cli` | `every_refusal_carries_a_diagnostic` | S1f.5.4 | the four `binding-key-*` witnesses' `test` cells exit 1 with an empty stderr (C3) | the witnesses scoped (S1f.5.4's proposal A), so each `:expect` holds |
| ein-cli | `corpus_cli` | `the_files_refused_until_digits_are_names_are_refused` | S1f.5.4 | `range-in-match` parses | deleted with `REFUSED_UNTIL_S1F_5_4` |
| ein-cli | `corpus_cli` | `no_cell_crashes` | S1f.5.2 | a `test` cell on a file now refused exits 2 | the cells follow the fixtures (§ 0 item 2) |
| ein-cli | `corpus_cli` | `every_positive_entry_answers_under_at_least_one_run` | own, S1f.5.11, S1f.5.2, S1f.5.1 | `is-a-shapes`, `negation`, `unary-and-nary`, `wrappers` are refused as written | their rewrites (T10 where no answer moves, else T4) |
| ein-cli | `corpus_cli` | `the_slow_flag_still_describes_the_sweep` | S1f.5.4 | `binding-key-both`, `-swapped`, `-nested-only` cost 4.4 to 5.1 s of cells | scoped (proposal A); the flags are re-read |
| ein-cli | `corpus_cli` | `every_cell_reproduces_its_exit_code` | all | `corpus_exits.txt`: 87 of 1 306 cells | re-banked (the golden it reads, § 12) |
| ein-cli | `expect_cli` | `the_load_negatives_are_refused_with_their_banked_message` | S1f.5.11 | `expect_unknown_keyword`'s line is the parse error | restated |
| ein-cli | `primitive_arity` | `a_refused_arity_names_its_position` | own | `(eq ?x)`'s arity is the grammar's | restated to the parse error |
| ein-cli | `primitive_arity` | `the_unpinned_primitives_are_refused_at_a_wrong_arity` | own | `(eq)` is a `ParseError`, not a compile refusal | restated; the compiler lines go in T5 |
| ein-cli | `region_dump` | `a_random_region_written_as_a_program_comes_back` | S1f.5.15 (a leftover) | `utils/region_synth.py` writes `:priority 200/300/500` | **not in the stage's list**: the three clauses go from `region_synth.py` (and from `guard_split.py` and `fuzz_engine.py`, which write it too) |
| ein-cli | `render_doors` | `render_program_draws_what_the_loader_refuses` | own (default 2) | `derivation_cycle` does not parse | T10's rewrite |
| ein-cli | `wrong_answers` | `every_row_answers_what_the_register_records` | all | 37 of 139 register runs | § 8 |
| ein-core | `lib` | `terms::tests::display_of_each_of_the_three_shapes` | S1f.5.4 | the name `1` prints bare | restated (the printer) |
| ein-core | `lib` | `text::tests::a_name_is_bare_only_where_it_lexes_as_itself` | S1f.5.4 | `1` prints bare | restated, and held to `is_one_name` |
| ein-core | `values_semantics` | `display_renders_every_value_shape` | S1f.5.4 | `it's` prints bare (`'` is a name character) | restated |
| ein-einb | `roundtrip` | `a_saturated_zebra2_is_small_and_opens_cold_in_under_a_millisecond` | the generator's cost (grmtools, the user's ruling of 2026-11-07) | a `.einb` cold open re-parses its PROGRAM section: `zebra2` opens in 3.7 ms on the generated build against 0.76 to 1.0 today (release, three runs each), past design/10 § 6's 1 ms; 5.4 against 1.0 in the dev profile, past its 5 | **not in the stage's list**: § 0 item 12 (restate the budget, or keep the parsed tree in the container) |
| ein-infer | `lib` | `compile::tests::a_string_in_a_pattern_is_the_symbol_it_names` | S1f.5.2 | a string in a pattern is a parse error | restated to the refusal; the value arm goes in T1f.5.2.3 |
| ein-infer | `lib` | `compile::tests::a_nested_pattern_with_no_keyable_slot_has_no_probe` | S1f.5.1 | its fixture writes `_` | rewritten with a variable used once |
| ein-infer | `lib` | `compile::tests::plan_key_is_the_activators_values` | S1f.5.2, S1f.5.4 | its activators `"7"` and `"seven"` are strings | the int-against-string case restated, as the stage lists |
| ein-infer | `lib` | `compile::tests::a_wildcard_matches_any_value_and_binds_nothing` | S1f.5.1 | `_` is refused | goes with `Slot::Any` (T1f.5.1.3), or restated to the refusal |
| ein-infer | `alive_invariant` | `every_name_of_the_program_is_a_candidate_object` | S1f.5.4, S1f.5.1, S1f.5.11 | `SEEDED` moves: digits are names; `underscore-nested-fact-arg`, `kwpair-in-pattern`, `kwpair-in-assert` no longer load | `SEEDED`'s lines, as the stage lists |
| ein-infer | `claim_round_trip` | `every_printed_region_holds_as_its_own_claim` | S1f.5.2 (on W34) | W34's printed region `(model (q "#empty") (q a))` holds a string, which a claim may no longer write | **not in the stage's list**: W34's witness joins the round trip's exceptions, as `DO_NOT_LOAD` holds it out of the state-file round trip, until S1f.5.9 fixes W34 |
| ein-infer | `claim_semantics` | `the_loader_refuses_what_is_not_a_claim` | own | a top-level `(region …)` is a parse error | restated |
| ein-infer | `compile_semantics` | `an_absent_that_compiles_to_no_steps_is_refused` | S1f.5.11 | `(absent :k v)` is a parse error | restated |
| ein-infer | `compile_semantics` | `the_retired_flags_are_refused_with_why` | S1f.5.11 (default 5) | a retired `config` flag is a parse error | restated; default 5's note names the stage that retired it |
| ein-infer | `compile_semantics` | `the_config_rejections_name_the_flag_and_the_shape_they_wanted` | S1f.5.11, S1f.5.2 | an unknown flag and a string value are parse errors | restated; T5 moves the loader lines |
| ein-infer | `compile_semantics` | `a_where_keyword_pair_in_a_match_is_dropped_and_the_rule_still_compiles` | S1f.5.11 | `:where` is refused | becomes a refusal test (S1f.5.11 names it) |
| ein-infer | `compile_semantics` | `bool_flags_coerce_from_their_ir_shapes` | S1f.5.2 | `"false"` is a string | restated: `true` and `false` only |
| ein-infer | `compile_semantics` | `a_bare_declaration_is_declared_with_an_empty_signature_and_stores_one_fact` | own | `(relation R (T1 T2))` is a parse error | its malformed case restated to the parse error |
| ein-infer | `compile_semantics` | `a_relation_parameter_a_rule_binds_to_a_number_is_refused` | S1f.5.4 (Q2) | `(mirror 3)` names the relation `3` | retired with `relation_parameter_int` |
| ein-infer | `compile_semantics` | `a_wildcard_in_a_rule_matches_what_a_fresh_variable_does` | S1f.5.1 | `_` is refused | goes with `Slot::Any`, or restated |
| ein-infer | `compile_semantics` | `a_string_in_a_rule_is_the_symbol_it_names` | S1f.5.2 | a string in a rule is a parse error | restated to the refusal |
| ein-infer | `compile_semantics` | `every_compile_error_says_what_its_expected_file_says` | own | the four `broken/compile/` fixtures' arity refusals are the grammar's | the fixtures move to `examples/broken/` (Q-M1f.2 (a)) with the new report |
| ein-infer | `detector` | `a_monotone_verdict_is_one_state_per_node_on_random_programs` | S1f.5.15 (a leftover) | `region_synth.py --random` writes `:priority`, so 5 of its programs load | as `a_random_region_written_as_a_program_comes_back` |
| ein-infer | `detector` | `the_static_half_is_empty_on_a_monotone_program_and_named_on_a_flagged_one` | S1f.5.11 | `kwpair-in-macro` leaves the flagged programs: 49 → 48 | `detector.rs`' count, as the stage lists |
| ein-infer | `explain_semantics` | `the_agenda_is_fifo_in_the_written_order` | S1f.5.15 | its fixture writes `:priority 1001`, to show it decides nothing | restated: the clause is refused, so the fixture drops it |
| ein-infer | `explore` | `a_wildcard_answers_what_a_fresh_variable_does` | S1f.5.1 | `_` is refused | goes with `Slot::Any`, or restated |
| ein-infer | `naf_semantics` | `the_stratification_advisory_is_the_librarys_and_the_flag_is_retired` | S1f.5.11 (default 5) | `:warn-derived-naf` is a parse error | restated; the note names S1e2.2.5 |
| ein-infer | `refutation_under_absent` | `the_refused_set_is_exactly_these` | S1f.5.4 | `activator-all-int`'s line gains `[activator 1]` | restated, as the stage lists |
| ein-infer | `register_region` | `every_row_states_its_witness_s_region` | S1f.5.2, S1f.5.1, S1f.5.11 | a `[case.region]` row's witness is refused | the rows as § 8 restates them |
| ein-infer | `rule_semantics` | `an_int_and_a_string_with_one_text_are_two_activators` | S1f.5.2, S1f.5.4 | W16's case cannot be written | restated to the refusal (§ 0 item 2) |
| ein-infer | `rule_semantics` | `a_string_in_a_rule_means_what_it_means_in_a_fact` | S1f.5.2 | W18's case: a string in a rule is refused | restated to the refusal |
| ein-infer | `rule_semantics` | `a_wildcard_in_a_pattern_matches_any_value_and_binds_nothing` | S1f.5.1 | W20's case: `_` is refused | restated to the refusal |
| ein-infer | `search_semantics` | `lattice_relation_keyword_accepts_a_bare_symbol` | own | `:hypothesis-relations co-located` without parentheses: the grammar takes a list of names | **not in the stage's list**: restated to the refusal, the list the one form (no corpus file writes the bare form) |
| ein-ir | `lib` | `expect::tests::a_variable_in_an_expectation_is_rejected` | own | a claim's facts are ground: the parse refuses `?x` | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `expect::tests::bottom_is_spelled_with_the_kernels_own_word` | own | `:expect false` is a parse error; the loader line naming `(false)` goes (T5), or becomes a note | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `from_ir::tests::the_obsolete_mode_keyword_still_loads` | S1f.5.11 | `:mode` is refused (S1f.5.11's default) | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `from_ir::tests::a_nested_form_becomes_a_relational_node` | S1f.5.2 | `(p (?q a))`: a variable in a fact (G-6) | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `from_ir::tests::a_rule_shape_that_reads_nothing_is_refused_at_load` | S1f.5.1, S1f.5.11 | W24's shapes are parse errors | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `from_ir::tests::the_verdict_atom_refuses_every_shape_it_cannot_resolve` | own | `(open …)`'s shapes are the grammar's | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `from_ir::tests::every_pass_accumulates_rather_than_stopping_at_the_first_problem` | own | its shapes are parse errors, so the loader never sees them | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `dump::tests::the_width_budget_counts_characters_not_bytes` | S1f.5.2 | its fixture writes a string as a value | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `from_ir::tests::a_wildcard_relation_in_a_rule_is_refused_at_load` | S1f.5.1 | `(_ a)` is a parse error | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `from_ir::tests::a_present_but_unusable_annotation_reads_as_absent` | own | `:source` takes a string: `:source sentence` is refused | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `lex::tests::the_printers_symbol_is_the_lexers` | S1f.5.4 | the printer is the name rule; goes with `lex.rs`, its successor holds `is_symbol` to `is_one_name` | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `from_ir::tests::every_query_is_kept_and_the_last_config_still_wins` | S1f.5.11 | `:mode` and a retired flag are refused | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `from_ir::tests::a_malformed_expect_is_a_load_error` | own | a claim's shape is the grammar's | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `expect::tests::the_shape_errors_name_what_was_expected` | own | a claim's shape is the grammar's | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `from_ir::tests::a_later_query_is_reachable_by_index` | S1f.5.11 | `:mode` | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `from_ir::tests::provenance_comes_from_source_or_from_rule_and_using` | own (default 2) | `:using (p (r c d))` | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `macros::tests::arity_is_counted_over_positional_args_only` | S1f.5.11 | a keyword pair inside a macro call | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `from_ir::tests::an_unknown_query_keyword_is_a_load_error` | S1f.5.11 | the unknown keyword is a parse error | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `lib` | `dump::tests::synthetic_heads_print_without_one` | own | `:match X`: a name is no premise | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `dump_goldens` | `dump_then_parse_is_a_fixed_point` | own, S1f.5.2, S1f.5.1, S1f.5.11 | the corpus holds files the grammar refuses; reads what parses | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `fuzz_properties` | `every_seed_and_finding_parses_the_way_it_was_recorded` | all | `fuzz_seeds.txt` re-banked | re-banked (the golden it reads, § 12) |
| ein-ir | `grammar_decisions` | `the_parse_negative_fixtures_reproduce_their_message` | own (the error text) | the four parse-negatives' `.expected`, the new report | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `grammar_decisions` | `the_documented_ambiguities_keep_their_decisions` | all | `grammar_decisions.txt` re-banked | re-banked (the golden it reads, § 12) |
| ein-ir | `grammar_decisions` | `a_keyword_is_a_whole_word_in_every_place` | S1f.5.4 | its 435 `keyword-1` parses read the name `keyword-1`, overriding Q-M1e2.17 (i)(a): restated | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `imports_semantics` | `the_import_and_macro_failures_are_byte_identical` | S1f.5.2, own | `import_alias_not_a_name`'s `"m"` and the reserved names are parse errors | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `imports_semantics` | `reserved_names_are_reserved_through_every_import_route` | own | `(rule open …)` is a parse error, not the loader's reserved-name line | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `imports_semantics` | `every_ein_core_reserved_name_is_unbindable_through_a_qualified_import` | own | the names the lexer refuses as a declarator are nine, not four | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `ir_semantics` | `not_is_unary_and_neq_is_binary_by_grammar` | own (the error text) | the error's column | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `ir_semantics` | `the_kernel_names_a_macro_may_not_shadow` | own | `(macro absent …)` is a parse error | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `ir_semantics` | `a_wildcard_or_var_head_survives_lowering_and_a_round_trip` | S1f.5.1, own | `_` and `:assert ?a` are refused | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `ir_semantics` | `any_head_outside_the_declarator_set_is_a_generic_fact` | S1f.5.11 | a fact takes no `:foo` | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `ir_semantics` | `equality_lowers_to_an_eq_headed_form` | own | `:assert ?a` is no conclusion | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `ir_semantics` | `a_reserved_keyword_cannot_be_written_as_a_macro_name` | own | refused at the parse | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `ir_semantics` | `every_syntactic_shape_survives_dump_then_parse` | S1f.5.11, S1f.5.1, S1f.5.2 | its shapes include `:where`, `_` and strings | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `ir_semantics` | `trace_events_are_shape_pinned_only_inside_a_trace` | own (default 8) | a trace takes `step` alone | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `ir_semantics` | `a_rule_needs_params_and_a_query_needs_a_kw_pair` | own | `(rule x ())` parses; the loader asks for `:match` and `:assert` | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `ir_semantics` | `a_keyword_is_never_a_value_nor_a_head` | S1f.5.11 | a fact takes no `:bar` | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `ir_semantics` | `the_documented_ambiguities_resolve_as_recorded` | all | 34 of its 78 cases | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `kb_semantics` | `a_facts_origin_is_a_three_way_split_over_its_provenance` | own (default 2) | `:using (via …)` | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `kb_semantics` | `a_snapshot_still_resolves_its_premises_after_the_source_moves_on` | own (default 2) | `:using (via …)` | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `kb_semantics` | `the_unsat_core_frontier_is_the_derivation_dags_sources` | own (default 2) | `:using (via …)` | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `kb_semantics` | `keep_decides_membership_without_stopping_the_walk` | own (default 2) | `:using (via …)` | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `kb_semantics` | `the_reserved_name_guard_is_on_declarators_and_matches_whole_names` | own | `(relation eq …)` is a parse error | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `load_semantics` | `load_errors_accumulate_in_pass_order` | own (the error text) | `load_error_order.txt`: the parse error comes first | re-banked (the golden it reads, § 12) |
| ein-ir | `load_semantics` | `the_config_coercion_table_is_stable` | S1f.5.4, S1f.5.2, S1f.5.11 | `config_coercion.txt` | re-banked (the golden it reads, § 12) |
| ein-ir | `load_semantics` | `the_load_negative_fixtures_are_byte_identical` | own | the refusal fixtures the grammar takes over (Q-M1f.2 (a)) | restated in the final grammar, or moved to the grammar's table (T5) |
| ein-ir | `load_semantics` | `require_why_decides_loading_only` | the T3 ruling (a macro takes a `:why`) | the stdlib's two macros carry no `:why`, which `--require-why` asks of a macro on the generated parser | the stdlib's `forall` and `unknown` get their `:why`, `MANIFEST.sha256` with them, and the `cfg` arms go (the stage's T4 bullet) |
| ein-render | `lib` | `dot_util::tests::a_keyword_pair_below_a_form_is_labelled_key_value` | S1f.5.11 | its witness is refused; the arm is not reached (§ 5) | restated |
| ein-render | `corpus_shapes` | `every_corpus_rendering_reproduces_its_digest` | all | `corpus_shapes.md5`: 2 442 of 15 104 renderings | re-banked (the golden it reads, § 12) |
| ein-render | `golden_dot` | `the_per_form_ir_goldens_reproduce` | own (default 2) | a fixture writes `:using (c10)` | restated |
| ein-render | `golden_dot` | `the_query_and_trace_goldens_reproduce` | own (default 8) | a fixture writes the trace events the grammar drops | restated |
| ein-render | `golden_trace` | `every_real_solve_reproduces_its_golden` | S1f.5.4 | `branching/12`: N and the universe, 25 → 51 | re-banked (the golden it reads, § 12) |
| ein-render | `presentation_semantics` | `only_a_step_event_is_a_box_the_rest_are_ellipses` | own (default 8) | `branch-open` and the rest | restated |
| ein-render | `presentation_semantics` | `a_wildcard_or_variable_pattern_head_still_labels_its_edge` | S1f.5.1, own | `_` and `:assert ?a` | restated |

## The goldens T4 moves

`EIN_BLESS=1 cargo test --workspace --no-fail-fast` on the same snapshot (`build/s1f.5.5/t9/proto-bless.log`,
2026-11-08, 23 min 49 s): 841 passed and 102 failed, the seven tests that read a golden settled and nothing new
failing. **Eight goldens move**, each against an untouched snapshot (`diff -r base proto`): S1f.5.4's step-1 list,
and `load_error_order.txt`.

| golden | lines out, in | what moves | ruling |
|---|---:|---|---|
| `ein-cli/tests/golden/corpus_exits.txt` | 87, 87 of 1 390 | 87 cells: 36 `saturate`, 31 `solve`, 8 `test`, 6 `render rules`, 4 `saturate --dump`, one `render constraints` and one `render program`; a refusal fixture's `render` cell exits 1 now that it is refused at the parse | all |
| `ein-render/tests/golden/corpus_shapes.md5` | 2 381, 409 of 15 104 | 89 files: the 86 the sweep moves, and the three whose tree alone moves (`relation-decls`, W24's `form-in-guard` and `guard-before-binder`); a refused file keeps one row, its parse | all |
| `ein-ir/tests/golden/grammar_decisions.txt` | 86, 192 | the keyword table under the name rule (`rule-x`, `=`, `_x`, `1..5` parse), and every refusal a report with its repairs | S1f.5.4, own |
| `ein-ir/tests/golden/fuzz_seeds.txt` | 21, 73 | the seeds the grammar refuses, each with its report; `(a 1..5 :k 1..* …)` among them | S1f.5.4, S1f.5.1, S1f.5.2, S1f.5.11 |
| `ein-ir/tests/golden/config_coercion.txt` | 35, 98 | `(config :lattice-sanity-check 1)`'s `1` a name; a string, an unknown or a retired flag a parse error | S1f.5.4, S1f.5.2, S1f.5.11 |
| `ein-ir/tests/golden/load_error_order.txt` | 4, 40 | the shapes it accumulates in the loader's pass order are parse errors now, reported first, all of them | own |
| `ein-render/tests/golden/trace_ambiguous.md` | 4, 238 | `branching/12`: the universe and N, 25 → 51 | S1f.5.4 |
| `ein-render/tests/golden/trace_one-hypothesis.md` | 2, 2 | the same | S1f.5.4 |

`corpus/wrong_answers.toml` is not blessed: its 37 rows are restated by hand (§ The register in the stage's
record), and so are the 102 assertions the bless leaves failing.

## Every file

One row a file, from `build/s1f.5.5/t9/attribution.tsv`; a file that moves on no command has no ruling, unless its
tree moved alone. *What moves* is today against the generated build, on `ein solve`, `ein saturate --dump` and `ein
test`; *refused* is a parse refusal on the generated build, whose report the review material shows whole.

| file | what moves, today → generated build | ruling | why |
|---|---|---|---|
| `corpus/fuzz_findings/activator-text-key.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: R counts; H; saturate facts \| S1f.5.2: `"1"` a string |
| `corpus/fuzz_findings/activator-text-key-swapped.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: R counts; H; saturate facts \| S1f.5.2: `"1"` a string |
| `corpus/fuzz_findings/binding-key-layout.ein` | R counts; H; k_s 2, k_c 2, N 0, H 1 → k_s ≥56401, k_c 2097152, N 0, H 21 | S1f.5.4 | S1f.5.4: the name rule alone moves it: R counts; H |
| `corpus/fuzz_findings/binding-key-layout-swapped.ein` | R counts; H; k_s 2, k_c 2, N 0, H 1 → k_s ≥56401, k_c 2097152, N 0, H 21 | S1f.5.4 | S1f.5.4: the name rule alone moves it: R counts; H |
| `corpus/fuzz_findings/d3-goal-row-order.ein` | — | — | |
| `corpus/fuzz_findings/d3-unsat-core.ein` | — | — | |
| `corpus/fuzz_findings/fork-delta-order.ein` | — | — | |
| `corpus/fuzz_findings/hrule-reads-not.ein` | — | — | |
| `corpus/fuzz_findings/kwpair-below-the-filter.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.11 | S1f.5.11: `:rule`: a keyword this form does not take |
| `corpus/fuzz_findings/unsat-core-id-space.ein` | — | — | |
| `examples/branching/01_saturate_only.ein` | — | — | |
| `examples/branching/02_one_dead_one_alive.ein` | — | — | |
| `examples/branching/03_five_hyps_one_alive.ein` | — | — | |
| `examples/branching/04_two_levels.ein` | — | — | |
| `examples/branching/05_mini_zebra.ein` | — | — | |
| `examples/branching/06_lookahead_on.ein` | — | — | |
| `examples/branching/07_lookahead_off.ein` | — | — | |
| `examples/branching/08_hypothesis_relation_whitelist.ein` | — | — | |
| `examples/branching/09_hrule.ein` | — | — | |
| `examples/branching/10_kill_cache_on.ein` | — | — | |
| `examples/branching/11_kill_cache_off.ein` | — | — | |
| `examples/branching/12_typed_blind_solve.ein` | N; H; k_s 3, k_c 7, N 25, H 25 → k_s 3, k_c 7, N 51, H 51 | S1f.5.4 | S1f.5.4: the name rule alone moves it: N; H |
| `examples/branching/13_lookahead_naf_world.ein` | — | — | |
| `examples/branching/14_lookahead_unjudgeable.ein` | — | — | |
| `examples/branching/15_lookahead_two_step_on.ein` | — | — | |
| `examples/branching/16_lookahead_two_step_off.ein` | — | — | |
| `examples/broken/bare_top_level_atom.ein` | text only | own | own: `Norwegian` a name, both parsers refuse: the error text and every error recovery reaches |
| `examples/broken/compile/absent_arity_high.ein` | text only | own | own: `(`, absent takes one premise: the refusal moves up from the compiler — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/compile/absent_arity_zero.ein` | text only | own | own: `)`, absent takes one premise: the refusal moves up from the compiler — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/compile/activator_arity.ein` | — | — | |
| `examples/broken/compile/eq_arity_high.ein` | text only | own | own: `B` a name, eq takes two terms: the refusal moves up from the compiler — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/compile/eq_arity_low.ein` | text only | own | own: `)`, eq takes two terms: the refusal moves up from the compiler — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/compile/nested_or.ein` | — | — | |
| `examples/broken/keyword_as_value.ein` | text only | own | own: `:solve`: a keyword where the goal belongs, (query :goal :solve), both parsers refuse: the error text and every error recovery reaches |
| `examples/broken/load/config_bad_value.ein` | text only | own | own: `maybe` a name — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/config_retired_flag.ein` | text only | S1f.5.11 | S1f.5.11: `:lattice-order`: a keyword this form does not take — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/config_unknown_flag.ein` | text only | S1f.5.11 | S1f.5.11: `:nope`: a keyword this form does not take — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/derivation_cycle.ein` | text only | own | own: `p` a name — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/empty_absent.ein` | text only | own | own: `?x`: a variable where the position takes none — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/expect_is_a_pattern.ein` | text only | own | own: `?h`: a variable where the position takes none — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/expect_omits_the_goal.ein` | — | — | |
| `examples/broken/load/expect_unknown_keyword.ein` | text only | S1f.5.11 | S1f.5.11: `:expct`: a keyword this form does not take — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/expect_unknown_relation.ein` | — | — | |
| `examples/broken/load/hrule_duplicate_name.ein` | — | — | |
| `examples/broken/load/hrule_reserved_name.ein` | text only | own | own: `absent` — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/import_alias_not_a_name.ein` | text only | S1f.5.2 | S1f.5.2: `"m"` a string — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/import_as_and_symbols.ein` | — | — | |
| `examples/broken/load/import_bare_std.ein` | — | — | |
| `examples/broken/load/import_conflicting_definitions.ein` | — | — | |
| `examples/broken/load/import_cycle_b.ein` | — | — | |
| `examples/broken/load/import_cycle.ein` | — | — | |
| `examples/broken/load/import_module_not_found.ein` | — | — | |
| `examples/broken/load/import_relative_not_found.ein` | — | — | |
| `examples/broken/load/import_symbols_empty.ein` | text only | own | own: `)` — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/import_symbols_not_a_list.ein` | text only | own | own: `forall` a name — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/import_symbols_not_provided.ein` | — | — | |
| `examples/broken/load/macro_arity_mismatch.ein` | — | — | |
| `examples/broken/load/macro_duplicate.ein` | — | — | |
| `examples/broken/load/macro_reserved_name.ein` | text only | own | own: `absent` — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/open_arity.ein` | text only | own | own: `(` — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/open_compound_witness.ein` | — | — | |
| `examples/broken/load/open_in_match.ein` | text only | own | own: `open` — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/open_mixed_assert.ein` | text only | own | own: `open` — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/query_class_twice.ein` | — | — | |
| `examples/broken/load/query_lattice_keyword.ein` | text only | S1f.5.11 | S1f.5.11: `:no-hypothesis`: a keyword this form does not take — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/query_over_unknown.ein` | — | — | |
| `examples/broken/load/region_merged_unlisted.ein` | — | — | |
| `examples/broken/load/region_owing_with_states.ein` | — | — | |
| `examples/broken/load/region_top_level.ein` | text only | own | own: `region` — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/relation_duplicate.ein` | — | — | |
| `examples/broken/load/relation_malformed.ein` | text only | own | own: `(` — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/relation_needs_a_name.ein` | text only | own | own: `)` — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/relation_parameter_int.ein` | solve exit 1→0; saturate exit 1→0; refused → k_s 32, k_c 32, N 0, H 5 | S1f.5.4 | S1f.5.4: the name rule alone moves it: solve exit 1→0; saturate exit 1→0 |
| `examples/broken/load/relation_reserved_name.ein` | text only | own | own: `eq` — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/reserved_open_aliased.ein` | text only | own | own: it imports reserved_open_direct.ein by path, which the grammar refuses: the line its refusal prints moves |
| `examples/broken/load/reserved_open_direct.ein` | text only | own | own: `open` — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/reserved_open_qualified.ein` | text only | own | own: it imports reserved_open_direct.ein by path, which the grammar refuses: the line its refusal prints moves |
| `examples/broken/load/reserved_open_symbols.ein` | text only | own | own: it imports reserved_open_direct.ein by path, which the grammar refuses: the line its refusal prints moves |
| `examples/broken/load/rule_duplicate_name.ein` | — | — | |
| `examples/broken/load/rule_half_declarators.ein` | — | — | |
| `examples/broken/load/rule_missing_assert.ein` | — | — | |
| `examples/broken/load/rule_missing_match.ein` | — | — | |
| `examples/broken/load/rule_reserved_name.ein` | text only | own | own: `absent` — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/solve_twice.ein` | — | — | |
| `examples/broken/load/solve_unknown_keyword.ein` | text only | S1f.5.11 | S1f.5.11: `:no-hypothesys`: a keyword this form does not take — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/broken/load/unbound_head.ein` | — | — | |
| `examples/broken/load/unimported_std_macro.ein` | — | — | |
| `examples/broken/rule_missing_params.ein` | text only | own | own: `:match`: a keyword where the parameter list belongs, both parsers refuse: the error text and every error recovery reaches |
| `examples/broken/unclosed_paren.ein` | text only | own | own: `a` a name, both parsers refuse: the error text and every error recovery reaches |
| `examples/domain_elim/ab.ein` | — | — | |
| `examples/domain_elim/b_branch.ein` | — | — | |
| `examples/domain_elim/b_only.ein` | — | — | |
| `examples/ein-bugs/activator-all-int.ein` | text only | S1f.5.4 | S1f.5.4: the name rule alone moves it: text only |
| `examples/ein-bugs/activator-all-nested.ein` | — | — | |
| `examples/ein-bugs/activator-all-symbol.ein` | — | — | |
| `examples/ein-bugs/activator-text-int-and-string.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: solve exit 0→1; test exit 0→1; R counts; H; saturate facts \| S1f.5.2: `"1"` a string |
| `examples/ein-bugs/activator-text-int-only.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: solve exit 0→1; test exit 0→1; R counts; H; saturate facts \| S1f.5.2: `"1"` a string |
| `examples/ein-bugs/activator-text-string-only.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: solve exit 0→1; test exit 0→1; R counts; H; saturate facts \| S1f.5.2: `"1"` a string |
| `examples/ein-bugs/activator-text-string-then-int.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: solve exit 0→1; test exit 0→1; R counts; H; saturate facts \| S1f.5.2: `"1"` a string |
| `examples/ein-bugs/alive-empty-interlayer.ein` | — | — | |
| `examples/ein-bugs/alive-empty-phase1.ein` | — | — | |
| `examples/ein-bugs/alive-set-fresh-name-declared.ein` | — | — | |
| `examples/ein-bugs/alive-set-fresh-name.ein` | — | — | |
| `examples/ein-bugs/alt-cap-core.ein` | — | — | |
| `examples/ein-bugs/alt-cap-core-reordered.ein` | — | — | |
| `examples/ein-bugs/binding-key-both.ein` | test exit 0→1; R counts; H; k_s 2, k_c 2, N 0, H 1 → k_s ≥56401, k_c 2097152, N 0, H 21 | S1f.5.4 | S1f.5.4: the name rule alone moves it: test exit 0→1; R counts; H |
| `examples/ein-bugs/binding-key-int-only.ein` | solve exit 0→1; test exit 0→1; R counts; H; k_s 1, k_c 1, N 0, H 0 → k_s 640, k_c 1024, N 0, H 10 | S1f.5.4 | S1f.5.4: the name rule alone moves it: solve exit 0→1; test exit 0→1; R counts; H |
| `examples/ein-bugs/binding-key-nested-only.ein` | test exit 0→1; R counts; H; k_s 2, k_c 2, N 0, H 1 → k_s ≥56401, k_c 2097152, N 0, H 21 | S1f.5.4 | S1f.5.4: the name rule alone moves it: test exit 0→1; R counts; H |
| `examples/ein-bugs/binding-key-swapped.ein` | test exit 0→1; R counts; H; k_s 2, k_c 2, N 0, H 1 → k_s ≥56401, k_c 2097152, N 0, H 21 | S1f.5.4 | S1f.5.4: the name rule alone moves it: test exit 0→1; R counts; H |
| `examples/ein-bugs/complete-records-stale.ein` | — | — | |
| `examples/ein-bugs/d3-goal-row-order.ein` | — | — | |
| `examples/ein-bugs/d3-unsat-core.ein` | — | — | |
| `examples/ein-bugs/fact-goal-binding.ein` | — | — | |
| `examples/ein-bugs/fork-delta-order.ein` | — | — | |
| `examples/ein-bugs/int-goal-binding.ein` | text only | S1f.5.4 | S1f.5.4: the name rule alone moves it: text only |
| `examples/ein-bugs/kwpair-below-the-filter.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.11 | S1f.5.11: `:rule`: a keyword this form does not take |
| `examples/ein-bugs/mixed-type-hypothesis.ein` | text only | S1f.5.4 | S1f.5.4: the name rule alone moves it: text only |
| `examples/ein-bugs/naf-upward-closure.ein` | — | — | |
| `examples/ein-bugs/nested-core.ein` | — | — | |
| `examples/ein-bugs/nested-fact-hypothesis.ein` | — | — | |
| `examples/ein-bugs/query-goal-free-head.ein` | — | — | |
| `examples/ein-bugs/query-goal-free-head-unsat.ein` | — | — | |
| `examples/ein-bugs/unbound-assert-var.ein` | — | — | |
| `examples/ein-bugs/unbound-relation-head.ein` | — | — | |
| `examples/ein-bugs/unsat-core-id-space.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/absent-empty.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | own | own: `)`, default 7: (absent ()) refused, W35 |
| `examples/ein-bugs/wrong-answers/activator-nested-value.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/alt-cap-shared.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/alt-flag-refuted-by.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/alt-flag-two-derivations.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/arity3.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/assert-empty.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/assert-eq-x-w.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/assert-neq-w-x.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/assert-neq-x-x.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/bare-premise.ein` | text only | S1f.5.1 | S1f.5.1: `_` reserved — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/ein-bugs/wrong-answers/claim-int-name.ein` | saturate exit 0→1; test exit 1→2; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: solve exit 1→0; test exit 1→0; saturate facts \| S1f.5.2: `"1"` a string |
| `examples/ein-bugs/wrong-answers/claim-int-name-solve.ein` | saturate exit 0→1; test exit 1→2; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: solve exit 1→0; test exit 1→0; saturate facts \| S1f.5.2: `"1"` a string |
| `examples/ein-bugs/wrong-answers/classes-int-name.ein` | solve exit 0→1; saturate exit 0→1; k_s 8, k_c 8, N 0, H 3 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: R states; H \| S1f.5.2: `"1"` a string |
| `examples/ein-bugs/wrong-answers/closure-shared-given-control.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/closure-shared-given.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/core-activator-committed.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/core-activator-given.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/core-activator-guard-only.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/core-activator-root-derived.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/core-search-cut-5.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/core-search-cut-control.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/core-search-cut.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/empty-match-activator.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/empty-match-fact.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/empty-match-false.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/empty-match-hrule.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/f07-fpoff.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/f07-rules-swapped.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/forall-core-elim.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/forall-core-false.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/forbid-chain-0.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/forbid-chain-1.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/forbid-chain-2.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/forbid-chain-no-absent.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/forbid-chain-node-absent.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/forbid-chain-node-writes.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/forbid-chain-param-absent.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/forced-1.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/forced-1-two.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/forced-2.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/forced-2-off.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/forced-2-two.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/form-in-guard.ein` | unmoved | S1f.5.4 | S1f.5.4: the tree only: digits are a name, no answer moves |
| `examples/ein-bugs/wrong-answers/fresh-name-default.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/fresh-name-nospawn.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/guard-before-binder.ein` | unmoved | S1f.5.4 | S1f.5.4: the tree only: digits are a name, no answer moves |
| `examples/ein-bugs/wrong-answers/guard-only-first.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/guard-only-second.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-and-in-assert.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-and-in-assert-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-and-in-expect.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-and-in-expect-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-and-in-match.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-and-in-match-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-neq-in-absent.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-neq-in-absent-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-neq-in-assert.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-neq-in-assert-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-neq-in-fact.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-neq-in-fact-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-neq-in-goal.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-neq-in-goal-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-neq-in-match.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-neq-in-match-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-not-fact.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-not-fact-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-or-in-assert.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-or-in-assert-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-or-in-hrule.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-or-in-hrule-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-or-in-obligation.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-or-in-obligation-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-or-in-solve.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-or-in-solve-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kwpair-in-assert.ein` | solve exit 0→1; saturate exit 0→1; k_s 24, k_c 32, N 0, H 5 → refused | S1f.5.11 | S1f.5.11: `:k`: a keyword this form does not take |
| `examples/ein-bugs/wrong-answers/kwpair-in-goal.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 16, k_c 16, N 0, H 4 → refused | S1f.5.11 | S1f.5.11: `:zz`: a keyword this form does not take |
| `examples/ein-bugs/wrong-answers/kwpair-in-guard.ein` | text only | S1f.5.11 | S1f.5.11: `:k`: a keyword this form does not take — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/ein-bugs/wrong-answers/kwpair-in-macro.ein` | solve exit 0→1; saturate exit 0→1; k_s 4, k_c 4, N 0, H 2 → refused | S1f.5.11 | S1f.5.11: `:k`: a keyword this form does not take |
| `examples/ein-bugs/wrong-answers/kwpair-in-pattern.ein` | solve exit 0→1; saturate exit 0→1; k_s 24, k_c 32, N 0, H 5 → refused | S1f.5.11 | S1f.5.11: `:k`: a keyword this form does not take |
| `examples/ein-bugs/wrong-answers/kw-relation-fact.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/kw-relation-fact-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/match-only-declared.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/match-only.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/naf-nowb.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/negative-only-declared.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/negative-only.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/nested-fact-activator.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/nested-unbound-head.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/oblig-derived-activator-declared.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/oblig-derived-activator.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/oblig-derived-domain-declared.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/oblig-derived-domain.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/oblig-fresh-name-declared.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/oblig-fresh-name.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/oblig-fresh-name-two-declared.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/oblig-fresh-name-two.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed-declared.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/owed-seats.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/owed-seats-renamed.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/owed-two-step.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/pair-core-root-derived.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/pair-core-root-given.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/pair-core-root-negative.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/past-limit-lines.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/range-in-match.ein` | solve exit 1→0; saturate exit 1→0; refused → k_s 8, k_c 8, N 0, H 3 | S1f.5.4 | S1f.5.4: the name rule alone moves it: solve exit 1→0; saturate exit 1→0 |
| `examples/ein-bugs/wrong-answers/stale-naf-fork.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/stale-naf-fork-warn.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/str-in-absent.ein` | solve exit 0→1; saturate exit 0→1; k_s 4, k_c 4, N 0, H 2 → refused | S1f.5.2 | S1f.5.2: `"b"` a string |
| `examples/ein-bugs/wrong-answers/str-in-assert.ein` | solve exit 0→1; saturate exit 0→1; k_s 4, k_c 4, N 0, H 2 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: R counts; H \| S1f.5.2: `"lbl"` a string |
| `examples/ein-bugs/wrong-answers/str-in-guard.ein` | solve exit 0→1; saturate exit 0→1; k_s 4, k_c 4, N 0, H 2 → refused | S1f.5.2 | S1f.5.2: `"b"` a string |
| `examples/ein-bugs/wrong-answers/str-in-hrule.ein` | solve exit 0→1; saturate exit 0→1; k_s 4, k_c 4, N 0, H 2 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: text only \| S1f.5.2: `"lbl"` a string |
| `examples/ein-bugs/wrong-answers/str-in-match.ein` | solve exit 0→1; saturate exit 0→1; k_s 8, k_c 8, N 0, H 3 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: R states; H; saturate facts \| S1f.5.2: `"1"` a string, `"b"` a string |
| `examples/ein-bugs/wrong-answers/tree-dead.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/uncovered.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/uncovered-knows.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/underscore-activator-arg.ein` | solve exit 0→1; saturate exit 0→1; k_s 2, k_c 2, N 0, H 1 → refused | S1f.5.1 | S1f.5.1: `_` reserved |
| `examples/ein-bugs/wrong-answers/underscore-core-two-ways.ein` | solve exit 0→1; saturate exit 0→1; k_s 0, k_c 0, N 1, H 14 → refused | S1f.5.1 | S1f.5.1: `_` reserved |
| `examples/ein-bugs/wrong-answers/underscore-core-two-ways-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/underscore-fact-arg.ein` | solve exit 0→1; saturate exit 0→1; k_s 2, k_c 2, N 0, H 1 → refused | S1f.5.1 | S1f.5.1: `_` reserved |
| `examples/ein-bugs/wrong-answers/underscore-goal-arg.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.1 | S1f.5.1: `_` reserved |
| `examples/ein-bugs/wrong-answers/underscore-is-a-arg.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.1 | S1f.5.1: `_` reserved |
| `examples/ein-bugs/wrong-answers/underscore-nested-fact-arg.ein` | solve exit 0→1; saturate exit 0→1; k_s 4, k_c 4, N 0, H 2 → refused | S1f.5.1 | S1f.5.1: `_` reserved |
| `examples/ein-bugs/wrong-answers/where-in-and.ein` | solve exit 0→1; saturate exit 0→1; k_s 4, k_c 4, N 0, H 2 → refused | S1f.5.11 | S1f.5.11: `:where`: a keyword this form does not take |
| `examples/ein-bugs/wrong-answers/where-in-pattern.ein` | solve exit 0→1; saturate exit 0→1; k_s 4, k_c 4, N 0, H 2 → refused | S1f.5.11 | S1f.5.11: `:where`: a keyword this form does not take |
| `examples/ein-bugs/wrong-answers/where-on-rule.ein` | solve exit 0→1; saturate exit 0→1; k_s 4, k_c 4, N 0, H 2 → refused | S1f.5.11 | S1f.5.11: `:where`: a keyword this form does not take |
| `examples/ein-bugs/wrong-answers/wild-in-absent.ein` | solve exit 0→1; saturate exit 0→1; k_s 2, k_c 2, N 0, H 1 → refused | S1f.5.1 | S1f.5.1: `_` reserved |
| `examples/ein-bugs/wrong-answers/wild-in-absent-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/wild-in-guard.ein` | text only | S1f.5.1 | S1f.5.1: `_` reserved — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/ein-bugs/wrong-answers/wild-in-head.ein` | text only | S1f.5.1 | S1f.5.1: `_` reserved — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `examples/ein-bugs/wrong-answers/wild-in-head-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/wild-in-hrule.ein` | solve exit 0→1; saturate exit 0→1; k_s 2, k_c 2, N 0, H 1 → refused | S1f.5.1 | S1f.5.1: `_` reserved |
| `examples/ein-bugs/wrong-answers/wild-in-hrule-twin.ein` | — | — | |
| `examples/ein-bugs/wrong-answers/wild-in-match.ein` | solve exit 0→1; saturate exit 0→1; k_s 2, k_c 2, N 0, H 1 → refused | S1f.5.1 | S1f.5.1: `_` reserved |
| `examples/ein-bugs/wrong-answers/wild-in-match-twin.ein` | — | — | |
| `examples/ein-bugs/zebra2-bad.ein` | — | — | |
| `examples/ein-bugs/zebra2-hints.ein` | — | — | |
| `examples/features/01_not_and_absent.ein` | — | — | |
| `examples/features/02_star_in_identifiers.ein` | — | — | |
| `examples/features/03_forall.ein` | — | — | |
| `examples/features/04_open.ein` | — | — | |
| `examples/features/05_stdlib_domain_elim.ein` | N; H; k_s 32, k_c 32, N 21, H 26 → k_s 32, k_c 32, N 47, H 52 | S1f.5.4 | S1f.5.4: the name rule alone moves it: N; H |
| `examples/features/06_symmetric_native.ein` | — | — | |
| `examples/features/07_unstratifiable.ein` | — | — | |
| `examples/features/08_disjunct_guard_sets.ein` | — | — | |
| `examples/features/09_adjacent_via_same_house.ein` | — | — | |
| `examples/features/10_expect.ein` | — | — | |
| `examples/features/11_expect_ambiguity.ein` | — | — | |
| `examples/features/12_expect_false.ein` | — | — | |
| `examples/features/13_mixed_solution_and_open.ein` | — | — | |
| `examples/lattice/01_subset_pruned.ein` | — | — | |
| `examples/lattice/02_genuine_3set_death.ein` | — | — | |
| `examples/lattice/03_state_hash_collision.ein` | — | — | |
| `examples/lattice/04_obligation_trigger_hole.ein` | — | — | |
| `examples/lattice/05_owed_seats_bounded.ein` | — | — | |
| `examples/lattice/06_friends_tables.ein` | — | — | |
| `examples/lattice/06_friends_tables_everyone_sits.ein` | — | — | |
| `examples/lattice/07_two_pairs_two_tables.ein` | — | — | |
| `examples/lattice/08_absent_witness_hole.ein` | — | — | |
| `examples/lattice/09_activator_hole.ein` | — | — | |
| `examples/lattice/10_fifo_gap.ein` | — | — | |
| `examples/lattice/11_fifo_gap_static.ein` | — | — | |
| `examples/lattice/12_activator_chain_hole.ein` | — | — | |
| `examples/lattice/synth/local_not_global.ein` | — | — | |
| `examples/lattice/synth/maximal_not_top.ein` | — | — | |
| `examples/lattice/synth/merged_states.ein` | — | — | |
| `examples/lattice/synth/one_model_per_component.ein` | — | — | |
| `examples/lattice/synth/three_components.ein` | — | — | |
| `examples/lattice/synth/trigger_hole.ein` | — | — | |
| `examples/lattice/synth/trivial_line.ein` | — | — | |
| `examples/saturation/hypothesis-contradiction/coloc-disproved.ein` | — | — | |
| `examples/saturation/hypothesis-contradiction/next-to-disproved.ein` | — | — | |
| `examples/saturation/hypothesis-contradiction/right-of-disproved.ein` | — | — | |
| `examples/saturation/implies/org-chart.ein` | — | — | |
| `examples/saturation/implies/parent-to-ancestor.ein` | — | — | |
| `examples/saturation/implies/right-then-next.ein` | — | — | |
| `examples/saturation/square-bwd/floors.ein` | — | — | |
| `examples/saturation/square-bwd/houses.ein` | — | — | |
| `examples/saturation/square-bwd/meetings.ein` | — | — | |
| `examples/saturation/square-fwd/floors.ein` | — | — | |
| `examples/saturation/square-fwd/houses.ein` | — | — | |
| `examples/saturation/square-fwd/meetings.ein` | — | — | |
| `examples/saturation/square-unique/corner-house.ein` | — | — | |
| `examples/saturation/square-unique/cul-de-sac.ein` | — | — | |
| `examples/saturation/square-unique/terminus.ein` | — | — | |
| `examples/saturation/symmetric/couple.ein` | — | — | |
| `examples/saturation/symmetric/friends.ein` | — | — | |
| `examples/saturation/symmetric/neighbours.ein` | — | — | |
| `examples/saturation/transitive/colocation-chain.ein` | — | — | |
| `examples/saturation/transitive/mealtimes.ein` | — | — | |
| `examples/saturation/transitive/taxonomy.ein` | — | — | |
| `examples/saturation/type-exclusivity/colors.ein` | — | — | |
| `examples/saturation/type-exclusivity/nationalities.ein` | — | — | |
| `examples/saturation/type-exclusivity/pets.ein` | — | — | |
| `examples/syntax/arg-kinds.ein` | text only | S1f.5.4, S1f.5.2, S1f.5.1, own | S1f.5.4: the name rule alone moves it: solve exit 1→0; saturate exit 1→0 \| S1f.5.2: `"a string"` a string \| S1f.5.1: `_` reserved \| own: both parsers refuse: the error text and every error recovery reaches |
| `examples/syntax/config.ein` | — | — | |
| `examples/syntax/constraint-scopes.ein` | — | — | |
| `examples/syntax/equality.ein` | — | — | |
| `examples/syntax/is-a-shapes.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.4, S1f.5.2, S1f.5.1 | S1f.5.4: the name rule alone moves it: text only \| S1f.5.2: `?x`: a variable in a fact (G-6) \| S1f.5.1: `_` reserved |
| `examples/syntax/negation.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | own | own: `plain` a name, default 3: (not plain) refused; (not (plain)) the nullary negation |
| `examples/syntax/nullary.ein` | — | — | |
| `examples/syntax/query-full.ein` | — | — | |
| `examples/syntax/relation-decls.ein` | unmoved | S1f.5.4 | S1f.5.4: the tree only: digits are a name, no answer moves |
| `examples/syntax/rule-forall-and-not.ein` | — | — | |
| `examples/syntax/rule-guards.ein` | — | — | |
| `examples/syntax/rule-nary-and-or.ein` | — | — | |
| `examples/syntax/rule-shared-guard-var.ein` | — | — | |
| `examples/syntax/trace-empty.ein` | — | — | |
| `examples/syntax/trace-steps.ein` | — | — | |
| `examples/syntax/unary-and-nary.ein` | solve exit 0→1; saturate exit 0→1; k_s 8, k_c 8, N 0, H 3 → refused | own | own: `x` a name, default 2: :using (x) names no fact |
| `examples/syntax/wrappers.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.11 | S1f.5.11: `:source`: a keyword this form does not take |
| `examples/zebra2.ein` | — | — | |
| `examples/zebra2-hints.ein` | — | — | |
| `examples/zebra2-minus-15.ein` | — | — | |
| `examples/zebra2-minus-15-obligations.ein` | — | — | |
| `examples/zebra2-obligations.ein` | — | — | |
| `examples/zebra.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-05/findings/F1b_or_prefix.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-05/findings/F1_keyword_prefix_split.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-05/findings/F2_core_cut_5.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-05/findings/F2_core_cut_control.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-05/findings/F2_core_cut.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/bare_premise.ein` | text only | S1f.5.1 | S1f.5.1: `_` reserved — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/empty_parens.ein` | text only | own | own: `)`, both parsers refuse: the error text and every error recovery reaches |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/f07_as_written.ein` | solve exit 0→1; saturate exit 0→1; k_s 2, k_c 2, N 0, H 1 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/f07_rules_swapped.ein` | solve exit 0→1; saturate exit 0→1; k_s 2, k_c 2, N 0, H 1 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/F2_core_cut_5.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/F2_core_cut_control.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/F2_core_cut.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/guard_only_first.ein` | solve exit 0→1; saturate exit 0→1; k_s 2, k_c 2, N 1, H 2 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/guard_only_second.ein` | solve exit 0→1; saturate exit 0→1; k_s 2, k_c 2, N 1, H 2 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/hrule_no_fact.ein` | solve exit 0→1; saturate exit 0→1; k_s 2, k_c 2, N 0, H 1 → refused | own | own: `foo` a name, G-14 (Q-M1f.1 (b)): a conclusion no fact can hold |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/kwpair_in_pattern.ein` | text only | S1f.5.4, S1f.5.11, own | S1f.5.4: the name rule alone moves it: solve exit 1→0; saturate exit 1→0 \| S1f.5.11: `:where`: a keyword this form does not take \| own: both parsers refuse: the error text and every error recovery reaches |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/n5_int_vs_string.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: saturate facts \| S1f.5.2: `"1"` a string, `"x y"` a string |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/n5_string_space.ein` | solve exit 0→1; saturate exit 0→1; k_s 40, k_c 64, N 0, H 6 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: R states; H \| S1f.5.2: `"x y"` a string, `"1"` a string |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/nested_unbound_head.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/refuted_region_claim.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-05/probes/wild_in_guard.ein` | text only | S1f.5.1 | S1f.5.1: `_` reserved — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `plans/m1e2_correctness/inventory_2026-11-06/findings/F1_trivial_line_stated_obligation_control.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-06/findings/F1_trivial_line_stated_obligation.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-06/probes/assert_and_kw.ein` | solve exit 0→1; saturate exit 0→1; k_s 12, k_c 16, N 0, H 4 → refused | S1f.5.11 | S1f.5.11: `:k`: a keyword this form does not take |
| `plans/m1e2_correctness/inventory_2026-11-06/probes/assert_neq.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-06/probes/assert_never_fires.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.1, own | S1f.5.1: `_` reserved \| own: G-14 (Q-M1f.1 (b)): a conclusion no fact can hold, fired or not |
| `plans/m1e2_correctness/inventory_2026-11-06/probes/diff_models_text.ein` | solve exit 0→1; saturate exit 0→1; k_s 4, k_c 4, N 0, H 2 → refused | S1f.5.2 | S1f.5.2: `"x y"` a string |
| `plans/m1e2_correctness/inventory_2026-11-06/probes/empty_arg.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | own | own: `)`, default 7: () as a fact argument refused |
| `plans/m1e2_correctness/inventory_2026-11-06/probes/fact_wild_arg.ein` | solve exit 0→1; saturate exit 0→1; k_s 32, k_c 32, N 0, H 5 → refused | S1f.5.1 | S1f.5.1: `_` reserved |
| `plans/m1e2_correctness/inventory_2026-11-06/probes/goal_guard_claim.ein` | text only | own | own: `?y`: a variable where the position takes none — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `plans/m1e2_correctness/inventory_2026-11-06/probes/goal_guard.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-06/probes/unused_param.ein` | — | — | |
| `plans/m1e2_correctness/inventory_2026-11-06/probes/var_in_fact.ein` | solve exit 0→1; saturate exit 0→1; k_s 2, k_c 2, N 0, H 1 → refused | S1f.5.2 | S1f.5.2: `?a`: a variable in a fact (G-6) |
| `plans/m1e2_correctness/inventory_2026-11-06/probes/where_t3_example.ein` | text only | S1f.5.11, S1f.5.15 | S1f.5.11: `:where`: a keyword this form does not take \| S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/detector_proposal/activator_under_absent.ein` | — | — | |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/detector_proposal/fifo_gap.ein` | solve exit 0→1; saturate exit 0→1; k_s 4, k_c 4, N 0, H 2 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/detector_proposal/fifo_gap_priority.ein` | solve exit 0→1; saturate exit 0→1; k_s 4, k_c 4, N 0, H 2 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/detector_proposal/stratified_stale.ein` | solve exit 0→1; saturate exit 0→1; k_s 2, k_c 2, N 0, H 1 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_probes/d3_unsat_core_positive_read.ein` | — | — | |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_probes/features01_closed.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_probes/features04_closed.ein` | solve exit 0→1; saturate exit 0→1; k_s ≥317684, k_c 9007199254740992, N 0, H 53 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_probes/features07_choice.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 2, k_c 2, N 1, H 2 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_probes/fifo_gap_without_r2.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 4, k_c 4, N 0, H 2 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_probes/lattice08_dual_rail.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 5, k_c 5, N 1, H 4 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_probes/square_unique_corner_house_closed.ein` | saturate exit 0→1 | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_probes/square_unique_cul_de_sac_closed.ein` | saturate exit 0→1 | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_probes/square_unique_terminus_closed.ein` | saturate exit 0→1 | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_probes/stale_naf_fork_closed.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.7_probes/stale_naf_fork_dual_rail.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 2, k_c 2, N 1, H 2 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.10_probes/two-rules.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 0, k_c 0, N 1, H 0 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.10_probes/two-rules-node.ein` | solve exit 0→1; saturate exit 0→1; k_s 3, k_c 3, N 1, H 2 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.10_probes/two-rules-reordered.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 0, k_c 0, N 1, H 0 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_probes/claim-int-name.ein` | saturate exit 0→1; test exit 1→2; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: solve exit 1→0; test exit 1→0; saturate facts \| S1f.5.2: `"1"` a string |
| `plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_probes/claim-int-name-solve.ein` | saturate exit 0→1; test exit 1→2; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: solve exit 1→0; test exit 1→0; saturate facts \| S1f.5.2: `"1"` a string |
| `plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_probes/classes-int-name.ein` | solve exit 0→1; saturate exit 0→1; k_s 48, k_c 64, N 0, H 6 → refused | S1f.5.4, S1f.5.2 | S1f.5.4: the name rule alone moves it: R counts; H \| S1f.5.2: `"1"` a string |
| `plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_probes/grid.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 3, k_c 7, N 25, H 25 → refused | S1f.5.4, S1f.5.15, S1f.5.2 | S1f.5.4: the name rule alone moves it: R states; N; H; saturate facts \| S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) \| S1f.5.2: `"H 2"` a string, `"x y"` a string, `"1"` a string |
| `plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_probes/heads.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.2, S1f.5.1, S1f.5.11, own | S1f.5.2: `?x`: a variable in a fact (G-6) \| S1f.5.1: `_` reserved \| S1f.5.11: `:kw`: a keyword this form does not take \| own: (and …), (or …), (neq …) as a fact argument: a fact holds names and facts |
| `plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_probes/values.ein` | text only | S1f.5.4, S1f.5.2, own | S1f.5.4: the name rule alone moves it: solve exit 1→0; saturate exit 1→0 \| S1f.5.2: `"1"` a string, `"-1"` a string, `"007"` a string, `"x y"` a string, `""` a string, `"a\"b"` a string, `"back\\slash"` a string, `"line\nbreak"` a string, `"tab\there"` a string, `"Åsa"` a string, `"(paren"` a string, `";semi"` a string, `"?x"` a string, `"_"` a string, `"_x"` a string, `":kw"` a string, `"1..5"` a string, `"1..*"` a string, `"="` a string, `"not"` a string, `"and"` a string, `"or"` a string, `"neq"` a string, `"rule"` a string, `"hrule"` a string, `"query"` a string, `"solve"` a string, `"config"` a string, `"trace"` a string, `"macro"` a string, `"import"` a string, `"relation"` a string, `"open"` a string, `"false"` a string, `"model"` a string, `"not-red"` a string, `"rule.x"` a string, `"neq-1"` a string, `"#\|"` a string, `"x\|#"` a string \| own: both parsers refuse: the error text and every error recovery reaches |
| `plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/b13_without_self_block.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 3, k_c 4, N 0, H 2 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/b14_without_signoff.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 3, k_c 4, N 0, H 2 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/closed_q.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 1, H 1 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/dual_rail.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 3, k_c 3, N 2, H 3 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/features05_scoped.ein` | N; H; k_s 32, k_c 32, N 21, H 26 → k_s 32, k_c 32, N 47, H 52 | S1f.5.4 | S1f.5.4: the name rule alone moves it: N; H |
| `plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/features12_scoped.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 0, k_c 0, N 1, H 0 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/interlayer_without_totality.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 4, k_c 4, N 1, H 3 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/naf_open.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 0, H 0 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/naf_open_hrules.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 3, k_c 3, N 0, H 2 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/naf_stored_negative.ein` | solve exit 0→1; saturate exit 0→1; k_s 4, k_c 4, N 0, H 2 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/not_conclusion.ein` | saturate exit 0→1 | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/three_components_owe.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 13, k_c 13, N 8, H 7 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/trigger_hole_owe.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 9, k_c 9, N 0, H 5 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_probes/via_hole.ein` | saturate exit 0→1 | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/probes/alt-cap-below.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 0, k_c 0, N 1, H 8 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/probes/alt-cap-below-noalts.ein` | solve exit 0→1; saturate exit 0→1; test exit 0→2; k_s 0, k_c 0, N 1, H 8 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/probes/binding-key-int-only.ein` | R counts; H; k_s 1, k_c 1, N 0, H 0 → k_s 640, k_c 1024, N 0, H 10 | S1f.5.4 | S1f.5.4: the name rule alone moves it: R counts; H |
| `plans/m1e2_correctness/probes/binding-key-nested-only.ein` | R counts; H; k_s 2, k_c 2, N 0, H 1 → k_s ≥56401, k_c 2097152, N 0, H 21 | S1f.5.4 | S1f.5.4: the name rule alone moves it: R counts; H |
| `plans/m1e2_correctness/probes/cfgtest.ein` | text only | S1f.5.11, S1f.5.15 | S1f.5.11: `:enable-pre-branch-lookahead`: a keyword this form does not take, `:enable-forced-positive`: a keyword this form does not take \| S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `plans/m1e2_correctness/probes/f07-warn.ein` | text only | S1f.5.11, S1f.5.15 | S1f.5.11: `:warn-derived-naf`: a keyword this form does not take \| S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `plans/m1e2_correctness/probes/fresh-name-declared-default.ein` | text only | S1f.5.11 | S1f.5.11: `:no-hypothesis`: a keyword this form does not take — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `plans/m1e2_correctness/probes/php-a.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 6, H 6 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/probes/php-b.ein` | solve exit 0→1; saturate exit 0→1; k_s 1, k_c 1, N 6, H 6 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1e2_correctness/probes/ucid-perm1.ein` | text only | S1f.5.11 | S1f.5.11: `:no-hypothesis`: a keyword this form does not take — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `plans/m1e2_correctness/probes/uncovered-renamed.ein` | solve exit 0→1; saturate exit 0→1; k_s 7, k_c 7, N 0, H 4 → refused | S1f.5.15 | S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/four_mistakes.ein` | text only | S1f.5.4, S1f.5.11, S1f.5.1, own | S1f.5.4: the name rule alone moves it: text only, `a?x`: a token that is not a name \| S1f.5.11: `:bogus`: a keyword this form does not take \| S1f.5.1: `_` reserved \| own: both parsers refuse: the error text and every error recovery reaches |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/programs/arg-kinds.ein` | saturate exit 1→0 | S1f.5.4 | S1f.5.4: the name rule alone moves it: saturate exit 1→0 |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/programs/derivation_cycle.ein` | text only | own | own: default 2: :using ((r c d)) a list of facts; the loader's cycle line, now reached, default 2 (today's parser refuses :using ((…))) |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/programs/g14/derived-activator.ein` | — | — | |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/programs/g14/derived-activator-never.ein` | — | — | |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/programs/g14/fires.ein` | text only | S1f.5.11, S1f.5.1, own | S1f.5.11: `:k`: a keyword this form does not take \| S1f.5.1: `_` reserved \| own: `?x`: a variable where the position takes none, G-14 (Q-M1f.1 (b)): ?x as a whole conclusion — today's loader or compiler refuses it too: the refusal moves up (Q-M1f.2 (a)) |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/programs/g14/never-fires.ein` | solve exit 0→1; saturate exit 0→1; k_s 8, k_c 8, N 0, H 3 → refused | S1f.5.11, S1f.5.1, own | S1f.5.11: `:k`: a keyword this form does not take \| S1f.5.1: `_` reserved \| own: `?x`: a variable where the position takes none, G-14 (Q-M1f.1 (b)): ?x as a whole conclusion, never fired |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/programs/is-a-shapes.ein` | text only | S1f.5.4 | S1f.5.4: the name rule alone moves it: text only |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/programs/negation.ein` | — | — | |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/programs/refused/fact-args.ein` | solve exit 0→1; saturate exit 0→1; k_s ≥124315, k_c 4398046511104, N 0, H 42 → refused | S1f.5.4, S1f.5.2, S1f.5.1, own | S1f.5.4: the name rule alone moves it: R counts; H \| S1f.5.2: `"a string"` a string, `?x`: a variable in a fact (G-6) \| S1f.5.1: `_` reserved \| own: defaults 2 and 3: (not plain), :using (x) |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/programs/refused/names.ein` | text only | S1f.5.4, S1f.5.1, own | S1f.5.4: the name rule alone moves it: text only, `a?x`: a token that is not a name, `?x?y`: a token that is not a name, `a:b`: a token that is not a name, `a,b`: a token that is not a name, `a#b`: a token that is not a name, `a\|b`: a token that is not a name, `a\b`: a token that is not a name, `Жук`: a token that is not a name, `"a"b`: a string glued to what follows \| S1f.5.1: `?_` reserved, `_` reserved \| own: both parsers refuse: the error text and every error recovery reaches |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/programs/refused/rules.ein` | text only | own, S1f.5.11, S1f.5.15 | own: `)`, both parsers refuse: the error text and every error recovery reaches \| S1f.5.11: `:where`: a keyword this form does not take, `:enable-path-nogoods`: a keyword this form does not take \| S1f.5.15: :priority, which no rule takes since S1f.5.15 (a record under plans/ that kept it) |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/programs/unary-and-nary.ein` | solve exit 1→0; saturate exit 1→0; refused → k_s 8, k_c 8, N 0, H 3 | own | own: default 2: :using ((u a)) a list of facts, which today's parser refuses, default 2 (today's parser refuses :using ((…))) |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/programs/wrappers.ein` | — | — | |
| `stdlib/algebra.ein` | — | — | |
| `stdlib/bijection.ein` | — | — | |
| `stdlib/closure.ein` | — | — | |
| `stdlib/elim.ein` | unmoved | S1f.5.4 | S1f.5.4: the tree only: digits are a name, no answer moves |
| `stdlib/macro.ein` | — | — | |
| `stdlib/slots.ein` | — | — | |
| `stdlib/typing.ein` | — | — | |
| `tests/region/01_states.ein` | — | — | |
| `tests/region/02_merged_single.ein` | — | — | |
| `tests/region/03_merged_or.ein` | — | — | |
| `tests/region/04_owing.ein` | — | — | |
| `tests/region/05_empty.ein` | — | — | |
| `tests/region/06_false.ein` | — | — | |
| `tests/region/07_cube.ein` | — | — | |
| `tests/region/08_query_classes.ein` | — | — | |
| `tests/region/09_query_over_maximal.ein` | — | — | |
| `tests/stdlib/algebra/01_copiers.ein` | — | — | |
| `tests/stdlib/algebra/02_compose.ein` | — | — | |
| `tests/stdlib/algebra/03_boolean.ein` | — | — | |
| `tests/stdlib/algebra/04_extensive.ein` | — | — | |
| `tests/stdlib/algebra/05_tag_lemmas.ein` | — | — | |
| `tests/stdlib/algebra/06_equational.ein` | — | — | |
| `tests/stdlib/algebra/07_schroder.ein` | — | — | |
| `tests/stdlib/algebra/08_checks_satisfied.ein` | — | — | |
| `tests/stdlib/algebra/09_irreflexive_violated.ein` | — | — | |
| `tests/stdlib/algebra/10_antisymmetric_violated.ein` | — | — | |
| `tests/stdlib/algebra/11_asymmetric_violated.ein` | — | — | |
| `tests/stdlib/algebra/12_connex_violated.ein` | — | — | |
| `tests/stdlib/algebra/13_empty_violated.ein` | — | — | |
| `tests/stdlib/algebra/14_functional_violated.ein` | — | — | |
| `tests/stdlib/algebra/15_injective_violated.ein` | — | — | |
| `tests/stdlib/algebra/16_converse_illtyped_dom.ein` | — | — | |
| `tests/stdlib/algebra/17_converse_illtyped_ran.ein` | — | — | |
| `tests/stdlib/algebra/18_totality_open_world.ein` | — | — | |
| `tests/stdlib/algebra/19_total_violated.ein` | — | — | |
| `tests/stdlib/algebra/20_surjective_violated.ein` | — | — | |
| `tests/stdlib/algebra/21_transitive.ein` | — | — | |
| `tests/stdlib/algebra/22_symmetric_negative.ein` | — | — | |
| `tests/stdlib/algebra/23_total_owed.ein` | — | — | |
| `tests/stdlib/algebra/24_total_owed_satisfied.ein` | — | — | |
| `tests/stdlib/algebra/25_surjective_owed.ein` | — | — | |
| `tests/stdlib/algebra/26_surjective_owed_satisfied.ein` | — | — | |
| `tests/stdlib/bijection/01_setup_and_negatives.ein` | — | — | |
| `tests/stdlib/bijection/02_domain_elimination.ein` | — | — | |
| `tests/stdlib/bijection/03_range_elimination.ein` | — | — | |
| `tests/stdlib/bijection/04_typecheck_arg0_violated.ein` | — | — | |
| `tests/stdlib/bijection/05_typecheck_arg1_violated.ein` | — | — | |
| `tests/stdlib/bijection/06_blind_enumeration.ein` | — | — | |
| `tests/stdlib/closure/01_infer_closure.ein` | — | — | |
| `tests/stdlib/closure/02_closed_and_satisfied.ein` | — | — | |
| `tests/stdlib/closure/03_closed_and_owing.ein` | — | — | |
| `tests/stdlib/elim/01_domain_elimination.ein` | text only | S1f.5.4 | S1f.5.4: the name rule alone moves it: text only |
| `tests/stdlib/elim/02_no_room_left.ein` | text only | S1f.5.4 | S1f.5.4: the name rule alone moves it: text only |
| `tests/stdlib/elim/03_typecheck_arg0_violated.ein` | — | — | |
| `tests/stdlib/elim/04_typecheck_arg1_violated.ein` | — | — | |
| `tests/stdlib/macro/01_forall.ein` | — | — | |
| `tests/stdlib/macro/02_unknown.ein` | — | — | |
| `tests/stdlib/slots/01_partition.ein` | — | — | |
| `tests/stdlib/slots/02_negative.ein` | — | — | |
| `tests/stdlib/slots/03_fill.ein` | — | — | |
| `tests/stdlib/slots/04_elimination.ein` | — | — | |
| `tests/stdlib/slots/05_no_room.ein` | — | — | |
| `tests/stdlib/slots/06_no_fill.ein` | — | — | |
| `tests/stdlib/slots/07_spatial_prune.ein` | — | — | |
| `tests/stdlib/slots/08_spatial_adjacent.ein` | — | — | |
| `tests/stdlib/slots/09_owed_room.ein` | — | — | |
| `tests/stdlib/slots/10_owed_room_satisfied.ein` | — | — | |
| `tests/stdlib/slots/11_owed_fill.ein` | — | — | |
| `tests/stdlib/slots/12_owed_fill_satisfied.ein` | — | — | |
| `tests/stdlib/slots/13_adjacent_bwd_neg_direction.ein` | — | — | |
| `tests/stdlib/typing/01_reflexive.ein` | — | — | |
| `tests/stdlib/typing/02_type_hierarchy_converse.ein` | — | — | |
| `tests/stdlib/typing/03_type_hierarchy_welltyped.ein` | — | — | |

## Re-taking it

```sh
# from the repository root; everything lands under build/s1f.5.5/t9/ (gitignored)
T=build/s1f.5.5/t9
for s in base proto namerule; do mkdir -p $T/$s && git archive efd2cc74 | tar -x -C $T/$s; done
(cd $T/proto && patch -p1 < ../swap-prototype.patch)         # the swap prototyped: 3 files
(cd $T/namerule && patch -p1 < ../namerule.patch)            # S1f.5.4's step-1 prototype on HEAD, and Q4
for s in base proto namerule; do (cd $T/$s/ein.rs && CARGO_TARGET_DIR=$PWD/target cargo build --release -p ein-cli); done
# the CLI sweep, 513 files through three binaries (115 s), then the per-file credit
$T/scripts/run_sweep.sh && python3 $T/scripts/analyze.py && python3 $T/scripts/attribute.py
# the parse itself: both parsers on every file and on 40 000 census mutants (13 s)
for s in base namerule; do cp $T/scripts/t9_parse.rs $T/$s/ein.rs/crates/ein-cli/examples/; \
  (cd $T/$s/ein.rs && CARGO_TARGET_DIR=$PWD/target cargo build --release -p ein-cli --example t9_parse); done
$T/scripts/run_parse.sh && python3 $T/scripts/mutants.py
# cargo test on the snapshot with the swap prototyped, then blessed
(cd $T/proto/ein.rs && CARGO_TARGET_DIR=$PWD/target cargo test --workspace --no-fail-fast) > $T/proto-test.log 2>&1
(cd $T/proto/ein.rs && EIN_BLESS=1 CARGO_TARGET_DIR=$PWD/target cargo test --workspace --no-fail-fast) > $T/proto-bless.log 2>&1
diff -r -q -x target $T/base $T/proto                      # the files the bless moved
# this review
python3 $T/scripts/review_build.py && python3 $T/scripts/register_md.py && python3 $T/scripts/index_build.py
```
