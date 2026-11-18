# S1f.23.10 — the user's answers on the language and three decisions' sentences, built

**Phase:** [P1f.23](README.md)
**Status:** started 2026-11-19 on the user's word (*"answered plans/m1f_hypothesis_and_documentation/review/04-language-and-decisions/ANSWERS.md - process"*),
built 2026-11-19 in a worktree (§ Record); merged by the main session, and one `utils/ci_container.sh` and one nightly hold it with R-42's fix.
**The answers:** the user's file as written, `b0e9c412`
(`git show b0e9c412:plans/m1f_hypothesis_and_documentation/review/04-language-and-decisions/ANSWERS.md`), and the
removed items' pages beside it in that commit (`…/04-language-and-decisions/NN-…/README.md`), with their programs.

## The answers

| item | the question | the user's answer |
|---|---|---|
| V-7 | a rule can conclude a fact of a relation the program does not count | *"What does mean "program does not count relation" ? relation is almost any atom if it is 1st in the tuple. Also for nested facts. Its enough to be the first item in the tuple to became a relation. (XXX) - is a relation fact with relation 'XXX'. Relation declaration `(relation = Formula Formula)` is optional fact that can be used to handle some random labels associated with the relation, even maybe not arguments types, not arguments count. It is just by convention is "relation declaration", that is used to reason about arguments domains. You easily can use `(rel-arg-types <relation_name> Type Type)`, `(rel-arg-types <relation_name> (list Type Type))`, `(defrel <relation_name> Type Type)` WHATEVER for the same purpose. But `relation` is already used by SOME OF stdlib modules. That is it."* |
| F21 N-15 | *everybody's thoughts are false* with `absent` comes out dead | F21's; filed there by the main session, nothing built here |
| Q-M1f.412 | the `is-a` row dropped: does a schema id move | a question, answered by the main session (the kernel reads `is-a` nowhere); the default (a) stands; one stale comment, task 1 |
| L-92 | Q-M1f.362's sentence on the fact index | *"(a)"* |
| L-93 | two stale sentences of the pages (a third found) | *"both, make docs consistent with the code and clear"* |
| L-94 | Q-M1f.119 misses a fact's argument | *"(a)"* |

## What it builds

### Task 1 — the sentences (L-92, L-93, L-94, and one comment of Q-M1f.412's)

Documentation and comments only; no answer moves. The text each item proposed is on its page in `b0e9c412`.

1. **L-92 (a)**: [Q-M1f.362](../../../docs/decisions/stdlib/math/Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md)'s
   **Why** re-worded to say both halves, the item's § 3 text, with one correction the main session found: zero is the
   name `z`, so the text says *a number other than `z`* (or *a numeral `(n …)`*) is a fact, not *a number is a fact*.
   With it, the item's § 7: `ein-core/src/kb.rs`' comment in `Kb::index_fact` (*"Only one level: the corpus's
   nesting is `(not (R …))` …"*) says the one level serves a stored negation and a context, as
   `03-data-model.md` § 6.1 does; `SlotKey`'s doc comment links § 6.1, the participation key, not § 5.
2. **L-93, both, *"consistent with the code and clear"***: the diagnostics page gains the item's **§ 1.7** (its
   option (b): the normative page of refusals states the parser's nesting limit, citing Q-M1f.185 and
   `generated_parser.rs`' `the_nesting_limit`), and `04-stdlib/08-natural-numbers.md` § How long a number may be
   links it; that paragraph's third sentence re-worded (the item's § 3.1: a saved state loads again). The
   reserved-strings page's `#nested` row re-worded (the item's § 3.2: the parse refuses every such form, so no program
   reaches it), and `NESTED_HEAD`'s doc comment (`ein-core/src/text.rs`) with it, which says the same stale thing.
   The constant stays (no code change).
3. **L-94 (a)**: [Q-M1f.119](../../../docs/decisions/language/Q-M1f.119-a-goal-and-a-claim-ask-only-about-relations.md)
   names *"writes as the relation of a fact that is an argument"* (the item's § 3.1, today's text with that clause
   added). With it, under L-93's *"make docs consistent with the code and clear"*, the item's § 3.3: `12-query.md` and
   `02-syntax.md` § Query list *"passed by an activator where a relation's name is held"*. **F23's D-23, the
   refusal's line, is not touched**: a code message, F23's, not ruled. If task 2 changes what a relation of the
   program is, these texts are written to task 2's notion, once.
4. **Q-M1f.412's comment**: `ein-ir/src/from_ir.rs` above `program_relations` (line 664) still lists `is-a` among a
   program's relations; Q-M1f.436 took it out. The comment says what the function does.

### Task 2 — V-7: a relation is any atom first in a tuple

**The ruling** (filed as **Q-M1f.522**, the next free id, a decision file of its own under `docs/decisions/language/`):
an atom is a relation where it is the first item of a tuple the program writes, at any depth: a stated fact, a
fact nested in another, a rule's, an hrule's or an obligation's pattern, a constant head or one an activator binds
for a parameter that heads a conclusion. A declaration, `(relation R …)`, is an optional fact, a convention some
stdlib modules use to reason about a relation's arguments; nothing makes a relation of the program depend on it.

**Built so:**

- **One notion, one function.** The engine's lists of the program's relations agree: the registry
  (`ein_core::Program::relations`, which decides what the blind rung takes for an object), the alive-set invariant's
  baseline (`invariant.rs`' `Universe::of_facts`), the goal's and claim's list (`from_ir.rs`' `program_relations`) and
  the reasoning's $\operatorname{rels}(P)$ ([`02-saturation.md`](../../../docs/ein/reasoning/02-kb-evolution/02-saturation.md)).
  Where they can be one computation they are; where one must stay apart, the stage says why.
- **"Almost any atom"**: the forms the kernel reads as forms today (`and`, `or`, `absent`, `eq`, `neq`, the macro
  calls, and whatever else `ein_core::terms::Kernel` holds) are not made relations by heading a tuple; the stage lists
  them and takes that as a reading with a default. P1f.21's direction (`false`, `not` and `relation` stdlib
  conventions) is P1f.21's and not built here.
- **What moves**: R moves where the blind rung guessed about such a name as an object (the item's `v7_blind.ein`, 8
  states today, 1 under the ruling). **A census before the build**: every corpus program's R and H by today's binary
  and by the stage's, each program that moves named with both answers; an `:expect` that states the old answer is
  brought to the ruling, as S1f.23.9 did. If a move is not the ruling's (an answer moves for another reason), stop
  and report it.
- **What does not move**: the argument-count check (Q-M1f.447, open, P1f.21's; the user's *"not arguments count"* is
  recorded there and decides nothing here), the declarations' own refusals, every answer the blind rung does not
  reach.
- **Pages**: the knowledge-base page and the user pages that say what a relation of the program is (the goal's
  refusal, `12-query.md`, `02-syntax.md`, `07-diagnostics.md` § 2.20 and § 2.21, Q-M1f.114 and Q-M1f.119 under task
  1), the read-out's `relation names` row ([`08-read-out.md`](../../../docs/ein/user/04-reference/08-read-out.md))
  where its words move, and the saturation page's $\operatorname{rels}(P)$.

## Acceptance

- Task 1: `utils/doc_audit.py --links --check --tracked`, `utils/check_doc_blocks.py`,
  `utils/check_release_voice.py`, `cargo doc` with `-D warnings`; no golden moves.
- Task 2: the census, every moved program named; `alive_invariant.rs`, `goal_premises.rs`, `solve_lists.rs`,
  `region.rs`, `explore.rs`, `corpus_cli`, `renaming_invariance.rs`, `stdlib_coverage.rs`; the shape golden re-banked
  (`EIN_BLESS=1 EIN_CORPUS_SLOW=1`), the kinds that move named.
- The full gate is the main session's, once on the merged tree: `utils/ci_container.sh` and the nightly.

## Record

Run 2026-11-19 in the worktree `build/wt-s1f2310`, branch `s1f.23.10-language`, on `f68f4adf`. Commits: `f88aae72`
(task 1's L-92 and L-93: one decision's and three pages' sentences, three doc comments; no code), `fdbb71ae` (task 2,
V-7 as Q-M1f.522, with L-94's texts and Q-M1f.412's comment written to its notion: the engine, the decision, the
pages, the tests, the shape golden) and this record (plans only). Not merged: the main session merges it.

### Task 1 — the sentences

- **L-92 (a)**: [Q-M1f.362](../../../docs/decisions/stdlib/math/Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md)'s
  **Why** is the item's § 3 text with the main session's correction: *"A number other than `z` is a fact, a numeral
  `(n …)` or `(b2 …)` nested to any depth"* (binary's numeral named beside decimal's, since the decision is the whole
  calculator's). `Kb::index_fact`'s comment says the one level serves a stored negation and a context, and what nests
  deeper is not keyed (`03-design/03-data-model.md` § 6.1); `SlotKey`'s doc links § 6.1.
- **L-93, both**: `07-diagnostics.md` § 1.7 (*A text nested past 1 000 parentheses is refused before it is parsed*),
  the item's option (b) text, citing Q-M1f.185 (a user page names no test: the test, `generated_parser.rs`'
  `the_nesting_limit`, stays named on `03-design/04-ir-frontend.md` § 3.4). Its exits re-taken with this branch's
  binary on the item's `nested-1001.ein`: `solve` 1, `saturate` 1, `kb save` 1, `test` 2. `08-natural-numbers.md`
  § How long a number may be links § 1.7, and its third sentence is the item's § 3.1 text. The reserved-strings
  page's `#nested` row and `NESTED_HEAD`'s doc comment (`ein-core/src/text.rs`) are the item's § 3.2 text; the
  constant stays.
- **L-94 (a)** and **Q-M1f.412's comment** are task 2's commit, written once to Q-M1f.522's notion (below).

### Task 2 — Q-M1f.522, a relation of the program is any atom it writes first in a tuple

**One notion, one computation.** `ein_ir::from_ir::relations_written` reads, once a program is loaded and its
activators' places are known, every atom the program writes first in a tuple outside its goals and `(solve …)`
lists: the relation of every stated fact and every fact inside one (`relation_heads`, at any depth, `:using` lists
included), of every form of every rule's, hrule's and obligation's `:match` and `:assert` as expanded, every declared
relation, and every name a fact or a conclusion passes where a relation's name is held (`places.passed`: `happy` of
`(tag happy)`, the inferred marker positions to their fixpoint). From it:

- **the registry**, `Program::relations`: `register_relations` adds each that is not a word of the kernel's
  (`ein_core::terms::heads_a_relation`), open-world, in the order of its text, after the declared relations and the
  stated facts' heads (`ingest_fact` registers a stated fact's head as before, less the kernel's words: a stated
  `(not (p a))` registered `not` and now registers `p`, a stated `(__closed__ r)` registered `__closed__` and now does
  not);
- **the goal's and claim's list**, `program_relations`: that set and `eq`, `neq`. It is the set the function computed
  before, kernel's words and all, so no goal, claim or `(solve …)` list is refused or let in that was not (the census:
  no refusal moved);
- **the alive-set invariant's baseline**, `Universe::of_facts`: the registry and the kernel's words; the loop that
  added the rules' constant `:assert` heads is gone, the registry holding them. `alive_invariant.rs`'
  `no_run_reaches_a_name_the_program_does_not_have` holds it over every state a bounded describe meets on the corpus;
- **the reasoning's** $\operatorname{rels}(P)$: [`02-saturation.md`](../../../docs/ein/reasoning/02-kb-evolution/02-saturation.md)'s
  clause 2 says every atom the program writes first in a tuple, at any depth, the atom an activator gives a parameter,
  and the declared ones; [`04-hypothesis-generation.md` § 5](../../../docs/ein/reasoning/02-kb-evolution/04-hypothesis-generation.md#5-the-blind-enumerator)'s
  $\operatorname{RelNames}(P)$ is $\operatorname{rels}(P)$ with the rules' names, and $\operatorname{guessable}(P)$
  is $\operatorname{names}(P)$ less the four kinds (the term $\operatorname{rels}(\mathrm{root})$ is gone from it).

**The blind rung.** `hypgen::candidate_objects` takes no registered relation (as before, by `Program::categorise`) and,
new, no atom that heads a fact root's fixpoint holds (`Kb::name_as_head`), so that a relation the static reading
cannot see, a parameter bound through a guard, `(rule pick () :match (eq ?y glad) :assert (tag ?y))`, is no object
either (the unit test `candidate_objects_hold_no_relation` holds both; no corpus program has such a rule: the census
reads the same with and without this line).

**The `relation names` row is gone**, with `ein_infer::relation_names`, `RootStats::relation_names`,
`--json-summary`'s `root.hypgen.relation_names_as_objects.*`, `region.json`'s `relation_names_as_objects`,
`region.md`'s *Relation names as objects* line and `ein-cli/tests/relation_names_row.rs`: no candidate has a relation
as an argument any more, so nothing could print it. The three keys were written only where non-empty, so no schema
id moves (Q-M1f.412's (a)); `08-read-out.md`'s row, `10-answer-dump.md`'s three rows and `11-region-dump.md`'s two
mentions are gone in the same commit; `09-read-out-examples.md` re-written by its generator (X15 alone moved, its
H 35 → 19, the table's `n` with it).

**Tests**: `from_ir`'s `the_registry_holds_every_atom_first_in_a_tuple` (a stated fact's, a nested fact's `=` and
`+`, a stored negation's `p`, a rule's `said` and `liquid`, an activator's `happy`; none of `not`, `__closed__`,
`and`, `false`); `hypgen`'s `candidate_objects_hold_no_relation`; `alive_invariant.rs`' `SEEDED` loses
`syntax/wrappers.ein`'s `r`, now a relation the names index holds through the registry; `answer_dump.rs` no longer
walks the gone keys.

**The decision**: [Q-M1f.522](../../../docs/decisions/language/Q-M1f.522-a-relation-of-the-program-is-any-atom-it-writes.md),
`docs/decisions/language/`. Re-worded to it: Q-M1f.119 (L-94 (a): *"writes as the relation of a fact that is an
argument"*, and the link to Q-M1f.522), Q-M1f.114, Q-M1f.306 (no relation is guessable). Pages: the knowledge base
(*the relations of a program*), solution semantics § 1.2 and § 1.4, `02-relations.md`, `05-rules.md` § Parameters and
activators, `09-hypotheses.md`, `12-query.md` and `02-syntax.md` § Query and § the solve lists (L-94's § 3.3: *"passed
by an activator where a relation's name is held, `(symmetric next-to)`"*), `07-diagnostics.md` § 2.20 and § 2.21,
`03-data-model.md` § 9.1, `01-architecture.md`, `08-reasoning-in-code/03-hypotheses-and-the-answer.md`. F23's D-23,
the refusal's line, is untouched. **Q-M1f.412's comment**: the doc block above `relation_heads` in `from_ir.rs` was
`refuse_solve_names`' own, misplaced; it is that function's doc now, with no `is-a`, its *relation of the program*
paragraph Q-M1f.522's.

### The census

Every corpus program (899 entries), `ein solve FILE` with `--json-summary` (R's states hashed) and `ein show hyps
FILE`, by the binary at `f68f4adf` and by this branch's: stdout, stderr and exit of both, `k_s`, `k_c`. The script
and its two runs are the stage's scratch, not kept. **Two programs moved, both the ruling's**, and nothing else (no
read-out line, no refusal, no H):

| program | before | after |
|---|---|---|
| `examples/features/01_not_and_absent.ein` | H 35 candidates, 16 of `is-a` and `likes` over `explicitly-dislikes`, the relation `see-stored-neg` concludes; flagged, past `--region-limit` (2^35): 64 655 states met, a lower bound, 43 projection classes | H 19; past the limit (2^19): 65 299 states met, a lower bound, 1 projection class |
| `examples/features/09_adjacent_via_same_house.ein` | H 82 candidates, 44 of `colour-loc` and `pet-loc` over `adjacent-via-fwd` and `adjacent-via-bwd`, all dead alone | H 38, all dead alone; R the same states |

The V-7 witnesses (not corpus programs), re-run: `v7_blind.ein` 8 states → 1 (H empty); `v7_blind-constant-head.ein`
8 → 1; `v7_blind-registered.ein` and `v7_blind-declared.ein` 1, as before; `v7_unpack.ein` {∅} as before, its
registry `think`, `unpack`, `+`, `=` where it was `think`, `unpack`; `v7_unpack-declared.ein` 4 096 as before;
`v7_tag-typo.ein` refused as before. **No `:expect` stated the old answer**: neither moved program states a claim
about R (`01_not_and_absent.ein` none, `09_adjacent…`'s query none), and `ein test examples tests stdlib` reads the
same before and after (749 held, the one FAILED the register's `goal-macro-expect-model.ein`, 2 not checked, 224
errors, the `broken/` and `ein-bugs/` refusals).

### The shape golden

Re-banked with `EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-render
--test id_order_invariance` (`EIN_TEST_JOBS=8`): 465 of the golden's lines moved, over 439 programs. **`load`**, the
KB's shape dump, moves on 439 programs: its registry lines (`REL`, `RELAPP`, `RULESREL`), every program that writes a
relation first in a tuple outside a stated fact's head, or a stated `(not …)` or `(__closed__ …)` fact. **Every other
kind** moves on the two programs whose H moved and nowhere else: `01_not_and_absent.ein` (`cores[…]` and
`read-out[…]` under the five questions, `oracle`, `region-lattice`, `region-trace`, `states`) and
`09_adjacent_via_same_house.ein` (the same less `region-lattice` and `states`).

### Checks

On `fdbb71ae`, in the worktree, with this branch's binary:

- `cargo test --profile gate -p ein-core -p ein-ir --tests` and `-p ein-infer --tests --no-fail-fast` (every
  integration test of the three, `alive_invariant.rs`, `region.rs`, `explore.rs`, `stdlib_coverage.rs`,
  `dunder_objects.rs` among them), and the four crates' unit tests: all pass (`EIN_TEST_JOBS=8`).
- `-p ein-cli`: `corpus_cli`, `goal_premises`, `solve_lists`, `answer_dump`, `read_out_reference`,
  `read_out_examples`, `read_out`, `events_reference`, `cli_semantics`, `show_hyps`, `summary_properties`,
  `region_dump`, `leftover_probe`, `einb_cli`, `render_doors`, `strata_diagnostic`, `regressions`,
  `config_reference`, `h_space_view`, `region_modes`, `saved_files`: all pass. `-p ein-render`:
  `renaming_invariance` passes; `id_order_invariance` re-banked (above).
- `ein test examples tests stdlib`: 749 held, 1 FAILED, 2 not checked, 224 errors, the same lines as at `f68f4adf`.
- `python3 utils/wrong_answers.py`: 16 open rows, each still answered wrong. `utils/check_generated.py`: 9
  generators, every file as written. `utils/doc_audit.py --links --check --tracked`: no finding outside
  `plans/…/review/` (whose ignored outputs a worktree lacks); 0 decision-id findings. `utils/check_doc_blocks.py`
  and `utils/check_release_voice.py` pass.
- `cargo fmt --check`; `cargo clippy -p ein-core -p ein-ir -p ein-infer -p ein-render -p ein-cli --all-targets -- -D
  warnings`; `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`: clean.
- Not run, the main session's on the merged tree: `./run_tests.sh`, the whole `cargo test --workspace`, the nightly,
  `utils/ci_container.sh`.

### Readings taken, each with its default (the main session files them)

- **The kernel's words** that heading a tuple does not make a relation of the program: `ein_core::terms::RESERVED`
  (`__closed__`, `absent`, `and`, `eq`, `false`, `neq`, `not`, `open`, `or`, `relation`) and the loader's `#` names.
  **Default**: that list, read by one function, `heads_a_relation`. `=`, which `Kernel` interns for the grammar's
  sake (`Kernel::equals`), is a relation: the kernel reads no form of it, the user's answer writes `(relation =
  Formula Formula)`, and `06_double_negation.ein` declares it. The other declarator words (`rule`, `query`, …) never
  head a tuple: the grammar reads them as no name.
- **The goal's and claim's list keeps the kernel's words a program writes**, the registry does not. **Default**: the
  list is the written set and `eq`, `neq`, unchanged from before, so a region claim listing a state's
  `(__closed__ r)` loads as it did; the registry, which decides what is an object, leaves them out.
- **A `(solve :hrules …)` activator's relation** (`att` of `(guess (att Color House))`) stays out of the registry and
  of the solve lists' set, as Q-M1f.114 says (*outside the three lists*), and stays a relation a goal and a claim may
  name. **Default**: as before.
- **A declared relation stays a relation of the program** with no tuple written: the blind rung's eligible relations
  are the declared ones, and whether a declaration should count once `relation` is a convention is P1f.21's
  (Q-M1f.447). **Default**: as before.
- **The relation of a fact root's fixpoint holds is no object** though no text writes it first in a tuple (a
  parameter a guard binds). **Default**: left out by `candidate_objects`; the registry, fixed at load, does not take
  it, so `alive_invariant.rs` would report it as a new relation where a describe meets one (no corpus program does).
- **The `relation names` row, its keys and its module are removed**, not kept for a case that cannot arise.
  **Default**: removed; no schema id moves (Q-M1f.412 (a)).

### Found, outside the stage

- **A claim listing a declaration is refused though Q-M1f.119 says the declarations are every program's**: `(query
  :goal (relation ?R ?T) :expect (model (relation person P)))` → `:expect names relation, and the program names no
  relation relation — …`; the goal alone loads. Not moved by this stage (the list is the one it was).
- **`walk_pattern` reads `=` as a connective** (`and`, `or`, `not`, `neq`, `eq`, `=`), so `Pattern::relation_names`
  and `pattern_relations` leave `=` out: `rules_by_relation` (the shape dump's `RULESREL`), `scope::outside`'s
  *asked*, and `ein test --json-report`'s `goal_relations` of a goal over `=`. Nothing the stage's four lists read;
  left as it is, since moving it moves a read-out row and a report, which the ruling does not ask.
