# S1f.6.37 — What a rule may conclude: `and` inside `and` (E-63)

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-14** ([§ Record](#record-2026-11-14)); planned 2026-11-14, on the user's word of that day;
the fix ruled the same day (flatten). W94 and W95 filed `open` (`eccab155`) and fixed (`0d144bef`, `fc9bd066`);
Q-M1f.271 defaulted; E-63 closed, E-65 to E-68 filed. **An engine stage**, small.
**Depends on:** nothing. Runs after [S1f.6.17](s1f.6.17_binary_logic.md)'s build (which holds `corpus/` and the
goldens while it runs) and **before [S1f.6.16](s1f.6.16_binary_arithmetic.md)'s build**, whose connecting call
(Q-M1f.266, (iii)) then sits in each base's closing rule rather than in a rule of its own.

## The instruction

E-63 was met by S1f.6.16's design ([§ The design](s1f.6.16_binary_arithmetic.md)): a macro call that expands to
an `(and …)`, written inside a closing rule's `(and …)` conclusion, activated nothing, in silence. The user,
2026-11-14:

> E-63 is a separate stage, The rule can assert some arbitrary complext fact built from simple facts with `and`
> and `or` - how engine handles this?

## What the engine does today

Probed 2026-11-14 on a release binary of that day (`ein saturate`, `ein solve -f`), one rule `:match ()` each,
over declared `p`, `q`, `r`, `h`:

| the rule's `:assert` | today |
|---|---|
| `(p a)` | stores the fact |
| `(not (q a))` | stores the negation (a stored negative, not closed) |
| `(and (p a) (not (q a)))` | stores each conjunct |
| **`(and (r a) (and (p a) (q a)))`** | **loads; stores `(r a)` and one fact headed `and`; `(p a)` and `(q a)` never** — E-63, a wrong answer: `(query :goal (p ?x) :expect (model (p a)))` fails (witness `build/s1f.6.16/design/and_in_and.ein`) |
| `(false)` | refutes the state |
| `(eq a a)`, `(eq a b)` | a check: the first holds and stores nothing; the second kills the state (root dead) |
| `(or (p a) (q a))`, `(and (p a) (or (q a) (r a)))` | refused at parse, *unexpected `or`*; the loader carries its own refusal too (*"an (or …) is no conclusion … write one rule per disjunct"*) |
| `(h (and (p a) (q a)))`, `(h (or …))`, `(not (and …))` | refused at parse: a value is a name or an atom (AGENTS.md § The language), so no fact holds a formula |
| `()`, `(open)`, `(open ?R)` | nothing; an obligation |

So a rule concludes **a conjunction of literals** (atoms and stored negations), `(false)`, a check, or an
obligation, and nothing else: no disjunction and no compound fact. **Why E-63 slips through:** the grammar's
`Conclusion ::= '(' 'and' Conclusions ')'` nests, and the loader's check of a conclusion
(`ein-ir/src/from_ir.rs`, the `conclusion` walk) recurses through every level; the compiler splits only the top
one (`assert_conjuncts`, `ein-infer/src/compile.rs`), so an inner `(and …)` becomes a fact template headed
`and`. Premises flatten at any depth (the same file's `premise`), conclusions do not. Macro expansion makes the
shape without anyone writing it: a macro whose body is an `(and …)`, called as one conjunct.

**Where "one of" lives instead.** A rule asserts no disjunction; *A or B* is said to the hypothesis layer, an
hrule's proposals or an obligation `(open R)` and the candidates that discharge it, and each alternative is a
state of R ([`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md)). A formula as an
object, `(And X Y)` written with ordinary names, is M1g's
([S1g.3.3](../../m1g_logic/p1g.3_proof_systems/s1g.3.3_natural_deduction.md)): to the engine it is an atom like
any other.

## Tasks

1. **T1f.6.37.1 — the register row first:** W-next `open` in `corpus/wrong_answers.toml`, the witness under
   `examples/ein-bugs/wrong-answers/`, its corpus entry, in a commit of its own.
2. **T1f.6.37.2 — the inventory of every reader of a conclusion** before the fix: the compiler's plan, the
   loader's checks (`from_ir.rs`), `ein_infer::forbid`, `ein_infer::mechanisms`, `invariant.rs`'s names, the
   obligation reader, the NL rendering of `:why`, the rule view and the IR printer (`ein-render`), `dump.rs`,
   `rule_census.py`. Each must read the same list of conjuncts.
3. **T1f.6.37.3 — the fix: flatten at any depth**, as premises are, through one function every reader of T2
   calls; the grammar unchanged. **Ruled by the user, 2026-11-14:** *"ok, flatten is fine, continue"* (the
   alternative, refusing a nested `and` at parse, is not taken). Flattening keeps the macro call of S1f.6.16
   legal inside a closing rule.
4. **T1f.6.37.4 — every row of the table pinned:** a test per shape (the grammar page's table for the parse
   refusals, `defined_behaviour.md` § 4 for the load refusals, a solve test for each stored shape), and the
   grammar page and `01_grammar.md` saying what a conclusion is in one sentence.
5. **T1f.6.37.5 — the sweep and the flip:** a census of the corpus's conclusions by shape (no tracked program
   writes a nested `and` today, so no answer may move: verified on every tracked program, old binary against
   new); W-next flipped `fixed`; the record and the phase README's row.

## Acceptance

- `(and (r a) (and (p a) (q a)))` stores `(r a)`, `(p a)` and `(q a)`, and no fact is ever headed `and`.
- Every reader of a conclusion reads the same conjuncts; a macro expanding to an `(and …)` inside an `(and …)`
  conclusion activates what it names.
- No tracked program's answer moves.

## What it must not do

- **Not add disjunctive conclusions** or compound facts: *one of* stays the hypothesis layer's.
- **Not change S1f.6.16's design** beyond letting its call sit in the closing rule.

## The standing rules

A wrong answer is a register row before its fix. Findings go to [`findings.md`](findings.md), questions to
[`open_questions.md`](../open_questions.md), each under the next free id. The gate runs once over the batch
(the user's cadence of 2026-11-13).

## Record, 2026-11-14

### T1f.6.37.1 — the register rows first, done (`eccab155`)

**W94**, two rows, each `now` the unchanged engine's: `and-in-and` (E-63's witness with a second query, over `q`)
and `and-in-and-macro` (S1f.6.16's trial `m1.ein`, the macro `both` writing `(and (p ?x) (q ?x))` called as one
conjunct of `(and (r a) (both a))`). R {∅}, each goal without a row, both claims FAILED, exit 1; `ein saturate
--dump` lists `(and (p a) (q a) :rule nested)` and `(r a :rule nested)`.

**W95**, two rows, met taking T2's inventory (E-65), **contested**: `hrule-and`, `(hrule h () :match () :assert
(and (p a) (q a)))`, whose H was `{(p a)}`, `(q a)` dropped in silence; `hrule-and-in-and`, the nested shape first in
an hrule, whose H was one candidate over the relation `and`. Their right answer is
[Q-M1f.271](../open_questions.md#q-m1f271--what-an-hrules-and--conclusion-proposes)'s default (a), a refusal, with
(b), each conjunct proposed, as the alternative.

The goldens were named before blessing and counted after: `corpus_exits.txt` 11 new cells, the two W94 solves at
exit 1; `corpus_shapes.md5` 164 new digests, 41 for each witness, none moved. The rows' own wrong answers were
named where a test met them, each list to be emptied by the fix: `answer_dump.rs`' and `answer_round_trip.rs`'
`DO_NOT_LOAD` (a state holding a fact headed `and` writes a line the parser refuses; the list now says how many
state files of a witness do not load, since `hrule-and-in-and`'s ∅ does), `claim_round_trip.rs`'
`DO_NOT_READ_BACK` (`hrule-and-in-and`'s region prints `(model (and (p a) (q a)))`, which no claim can write);
`alive_invariant.rs`' `SEEDED` gained the four programs' `a`, a name only a conclusion writes.

### T1f.6.37.2 — every reader of a conclusion, done

**The census first** (`cargo run --release -p ein-infer --example conclusion_census`, new, 2026-11-14, on
`9a922b96`'s tree before the witnesses were filed): 758 corpus
files and register witnesses, 537 loading (the 221 others are the negative fixtures and the refused witnesses), 2 173
distinct conclusions after macro expansion, a rule counted once by its full name and its conclusion's text. By shape:
rules 1 644 one fact, 46 one stored negation, 111 `(false)`, 4 a check, 1 `()`, and 312 an `(and …)` (79 of two facts,
1 of a fact and a stored negation, 5 of three, 36 of four, 191 of five or more); hrules 37 one fact, 1 one stored
negation, 1 a check; obligations 16 `(open …)`. **No conclusion writes an `and` inside an `and`** (depth 2 or more:
none), and **no hrule's conclusion is an `(and …)`**. As written, `utils/rule_census.py` over every tracked `.ein`
(1 629 rules, the refused witnesses among them) agrees: no `:assert` nor `:match` holds an `(and …)` directly inside
its top `(and …)`, and no hrule writes an `(and …)`. The census script did not parse at HEAD (E-67) and is fixed.

**The inventory.** One function reads a conclusion's conjuncts since T3, `ein_ir::Ast::conjuncts`: an `(and …)` at
any depth read as its members in the order written, any other node as one conjunct. Where a reader is Rust and splits
a conclusion, it calls it; where it walks every form at any depth with `and` transparent, it never split one and
reads the same either way.

| reader | what it read before | since T3 |
|---|---|---|
| `ein_infer::compile`'s `assert_conjuncts`, the plan's templates | the top `(and …)` split, an inner one a template headed `and` — **the defect** | `Ast::conjuncts` |
| `from_ir::refuse_no_fact_holds` (a relation no parameter binds, a conclusion no form) | the top `(and …)` split, an inner one read as a pattern headed `and` | `Ast::conjuncts` |
| `from_ir::refuse_malformed_primitives` (`Shapes::conclusion`, § 4.16) | recursed through every `and`, so the same conjuncts | `Ast::conjuncts`, then one conjunct each |
| `from_ir::refuse_unbound_conclusions` (§ 4.13) | `top_conjuncts`, each conjunct's variables at any depth: the same variables | `Ast::conjuncts`; `top_conjuncts` deleted |
| `from_ir::refuse_derived_relation_activators`: the sites a conclusion derives an activator at, and `refuse_eq_at_another_arity` (§ 4.9, § 4.14) | `top_conjuncts`: an inner `(and …)` read as a conclusion of the relation `and`, its members as its arguments, so a parameter heading a member was read as a value | `Ast::conjuncts` |
| `from_ir::refuse_negated_names`, its `(not ?x)` conclusions and what names a conclusion binds (§ 4.23) | `top_conjuncts`: a `(not ?g)` inside an inner `(and …)` not read | `Ast::conjuncts` |
| `from_ir::validate_open`, `obligations::open_argument` (the obligation reader) | `(open …)` a whole `:assert`, the grammar's `Assert` | unchanged: an obligation is never a conjunct |
| `from_ir`'s `wrong_counts` (§ 4.26), `walk_pattern` (a rule's `relation_names` and variables), `names_relation`, `arguments`, `sforms_under` (the derived-activator check), `intern_program_names` | every form at any depth, `and` a structural head | unchanged, depth-agnostic |
| `ein_ir::expect::goal_atoms` (a goal's atoms, flat at any depth since S1f.7.5) and `ein_infer::read_out`'s `conjuncts` | its own recursion, the same reading | `Ast::conjuncts` |
| `ein_infer::forbid`'s `heads` (what a rule writes, for *forbid*'s walk) | recursed through every `and` | `Ast::conjuncts` |
| `ein_infer::mechanisms`' `writes_name` (`__closed__` written), `invariant::rule_constants` and `Universe::of` (the alive-set invariant's names), `ein_core::kb`'s `build_rules_by_relation` | every node at any depth, `and` transparent (`STRUCTURAL`), or `relation_names` | unchanged, depth-agnostic |
| the plan's readers: `firing::fire`, the saturator, `lookahead`, `monotone`'s writes, `naf_deps::refuting_conclusion`, `shape::plan_shape`, the engine's counters | `plan.asserts`, the compiler's list | the same list, flat |
| the plan's **first** template, `plan.assert_template()`: `hrule`'s `proposes`, `compile::asserted_relation` and `negated_relation` (read by `closed`, `naf_deps`' producible sets, `hrule::asserted_relations`, `mechanisms`, *forbid*'s hrule remedy, `shape`) | the first of the compiler's list, an `and` where the first conjunct nested | the first of the same list, flat. For an hrule the only one since the refusal (E-65); for the S1.7.4 map, E-66, open |
| the natural-language reading (`09_natural_language.md`): a rule's `:why` filled from the firing's bindings (`ein_core::render_why`), a stored fact read through its relation's `:why` | no conclusion: `(and (p a) (q a))`, stored, had no relation's `:why` | each member stored, each read through its relation's |
| `ein-render`'s rule view (`rules.rs`' `panel`) | the top `(and …)` split, an inner one drawn by `clause_lines`' recursion, so the same lines for a conclusion | `Ast::conjuncts`, a premise's too (a guard inside an inner `(and …)` clustered as the compiler lifts it; no tracked program nests either clause directly, so no view moved) |
| `ein-render`'s IR view (`ir_dot.rs`), `ein-ir`'s printer (`dump.rs`), the trace (`trace::region::derives_line`, every fact a firing stored since S1f.6.25) | the text as written, or the stored facts | unchanged: the text of the rule as written is the one thing a nested and a flat conclusion differ in |
| `utils/rule_census.py` (`concl`), `utils/stdlib_mutants.py` (`conjuncts`), `utils/nl_census.py` (`heads`) | recursed through every `and` | unchanged, the same reading; `rule_census.py` fixed to parse (E-67) |
| `utils/stdlib_census.py`, and the writers `guard_split.py`, `region_synth.py`, `fuzz_engine.py`, the generators | the `:assert` as text, or flat conclusions written | unchanged |

E-65 and E-66 are the two readers of the first template; E-67 the census script.

### T1f.6.37.3 — the fix: flatten at any depth, done (`0d144bef`)

`ein_ir::Ast::conjuncts` is the one function: an `(and …)` at any depth read as its members in the order written,
any other node one conjunct, so `(and (r a) (and (p a) (q a)))` is `[(r a), (p a), (q a)]`, as `(and (r a) (p a) (q
a))` is. Every Rust reader of T2's table that splits a conclusion calls it: the compiler's `assert_conjuncts`, the
five loader checks (`top_conjuncts`, the loader's own top-level split, deleted), *forbid*'s `heads`, a goal's
`goal_atoms` (the same reading, its own recursion until now) and the rule view's `panel`. The grammar is unchanged:
`Conclusions` still nests, and a macro's call that leaves an `(and …)` as one conjunct of a closing rule's `(and
…)`, S1f.6.16's design's case, now concludes what it names.

W94's two rows flipped `fixed` in the fixing commit, `[case.fixed]` the null id there (this record's commit writes
the hash, `0d144bef`), both claims held, exit 0; `hrule-and-in-and`'s `now` restated, its hrule proposing `(p a)`
alone, the flat defect E-65 left. The test lists T1 filled were emptied the same commit.

**An hrule's conclusion is one fact** (E-65, W95), a commit of its own (T1f.6.37.3b, `fc9bd066`): the loader refuses an
hrule whose `:assert`, read flat, is more than one conjunct, one line placed at its first conjunct, *hrule 'h': its
:assert writes (and (p a) (q a)), 2 conclusions — an hrule proposes one fact per match, each a commitment of its own:
write one hrule per conclusion*, on every command (`defined_behaviour.md` § 4.29; Q-M1f.271's default (a)). W95's
rows flipped `fixed`, each `now` the refusal; their `[case.region]` dropped, a refused program having no R; the
two programs leave `alive_invariant.rs`' `SEEDED`, which no longer loads them.

### T1f.6.37.4 — every row of the table pinned, done

| the table's row | pinned by |
|---|---|
| `(p a)`, `(not (q a))`, `(and (p a) (not (q a)))`, `(and (r a) (and (p a) (q a)))`, `(false)`, `(eq a a)`, `(eq a b)`, `()`, `(open)`, and `(open ?R)` | `ein-cli/tests/conclusion_shapes.rs`' `every_shape_a_rule_concludes` and `an_obligation_concludes_nothing`: root's derived facts by `ein saturate --dump`, root alive, dead or owing and R by `ein solve`'s summary, and no fact headed `and` — fourteen conclusions, `(neq a b)`, `(not (eq a b))`, `(and (p a) (false))` and a check inside an inner `and` among them |
| an `and` inside an `and`, written and through a macro | `an_and_inside_an_and_is_read_flat` (five nestings against the flat conclusion), `a_macros_and_inside_an_and_is_read_flat` (a call inside an `and`, a macro whose body calls one, the expansion written out) and `the_witnesses_answer_as_their_flat_twins` (both W94 witnesses against their text with the conclusion written flat): the same derived facts in order, the same state after saturation, the same `ein solve` read-out and stderr, the same summary but its `work` — the user's rule of 2026-11-14, *verify by invariance*. Run against the old binary, four of the file's six tests fail and the two of unchanged shapes pass |
| `(or (p a) (q a))`, `(and (p a) (or (q a) (r a)))`, `(h (and …))`, `(h (or …))`, `(not (and …))` | refused at the parse: the grammar's table, rows 391 to 398 (`ein-ir/tests/grammar_pages.rs`, `grammar_table.txt`), each place a conclusion is written, an hrule's `(or …)` and an `(or …)` inside an inner `and` among them; and `no_disjunction_and_no_compound_fact_is_a_conclusion` through the binary |
| the same through a macro's expansion | refused at load, `defined_behaviour.md` § 4.16, already pinned by `ein-ir`'s `a_macros_expansion_is_held_to_the_grammars_shapes` |
| an hrule's `(and …)` | refused at load, § 4.29: `an_hrule_concludes_one_fact`, four conclusions written and through a macro's call, each on nine commands with its line, and one conjunct, bare or in an `(and …)` at any depth, loading |

The pages: `01_grammar.md` § Rules says what a rule concludes and that an hrule's conclusion is one fact, and its
§ Conclusion forms that an inner `(and …)` is read flat; `00_ebnf.md` says the same of `Conclusion` below the
generated grammar; `defined_behaviour.md` § 4.16 that a macro's `(and …)` inside a conclusion's is no malformed
primitive, and § 4.29 the hrule's refusal; `AGENTS.md` § The language one standing rule.

### T1f.6.37.5 — the sweep and the flip, done

**No tracked program writes the shape** (T2's census: no conclusion nests an `and`, as written or after expansion,
and no hrule's conclusion is an `(and …)`). **No tracked program's answer moved**: `build/s1f.6.37/sweep.py`, every
`corpus/corpus.toml` entry under `ein solve --json-summary` and `ein saturate --dump`, exit, stdout, stderr and the
summary with wall-clock numbers masked, the release binary of `9a922b96` against the fixed one of `0d144bef`'s tree:
762 programs, 5 runs differ, all on three of the stage's own witnesses (`and-in-and` and `and-in-and-macro`, both
runs; `hrule-and-in-and`, its solve).

**The goldens**, named before blessing and counted after:

| commit | named | moved |
|---|---|---|
| T1, `eccab155` | the four witnesses' cells and digests, new | `corpus_exits.txt` 11 new cells; `corpus_shapes.md5` 164 new, 41 a witness; nothing else |
| T3, `0d144bef` | W94's two solves 1 → 0; the three witnesses whose answer moves; the grammar's eight rows | `corpus_exits.txt` 2; `corpus_shapes.md5` 43 moved, 14 on `and-in-and`, 14 on `and-in-and-macro`, 15 on `hrule-and-in-and`; `grammar_table.txt` rows 391 to 398 new |
| T3b, `fc9bd066` | the two W95 witnesses, refused: their solve and saturate 0 → 1, their digests | `corpus_exits.txt` 4; `corpus_shapes.md5` on each witness 29 out and 7 in, the surfaces of a program refused at load; nothing else |

Each commit's tree ran `cargo test --release --workspace` green but for the goldens named, before blessing; the
last ran `cargo fmt --check`, clippy and rustdoc with `-D warnings`, `utils/doc_audit.py --links --check` and
`utils/check_hashmap_iteration.py` clean, and `utils/stdlib_mutants.py --check` (458 of 547 killed, the banked survivors, exit 0). No generator's input was
touched. `./run_tests.sh` was not run: the coordinator runs one gate over the batch.

W94 and W95 are `fixed`, every row; the register reads 322 rows fixed, 16 open and 1 not-wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-14). `[case.fixed]` names `0d144bef` for W94's
two and `fc9bd066` for W95's two, written by this record's commit.

### Defaults and findings

- **[Q-M1f.271](../open_questions.md#q-m1f271--what-an-hrules-and--conclusion-proposes)**, what an hrule's
  `(and …)` proposes: **defaulted (a)**, refused at load, the user's to revise; (b) each conjunct proposed, (c) as it
  was with a note.
- **E-63** closed (W94). **E-65**, an hrule's `(and …)` proposing its first conjunct alone, a wrong answer, W95,
  filed and fixed under Q-M1f.271's default. **E-66**, the static map of what rules derive (`asserted_relation`,
  `negated_relation`, the S1.7.4 map) reading a plan's first conclusion alone, so the stratification advisory and the
  auto-closure pass miss a relation concluded second; no command prints either; open, shown, no owner. **E-67**,
  `utils/rule_census.py` not parsing since S1f.6.31, fixed. **E-68**, a disjunction written as a conclusion refused at
  the parse with no word on why, where the loader says why for a macro's; open, shown, no owner.

### What the stage did not do

No disjunctive conclusion and no compound fact: *one of* stays the hypothesis layer's, and both stay refused, at the
parse and through a macro at load. S1f.6.16's design is unchanged; its connecting call may now stand in a base's
closing rule's `(and …)`, as Q-M1f.266's (iii) wanted once E-63 was fixed. The grammar is unchanged.
