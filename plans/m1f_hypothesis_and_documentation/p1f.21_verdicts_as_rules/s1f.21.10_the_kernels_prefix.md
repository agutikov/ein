# S1f.21.10 — the engine's key, `:ein inconsistent`

**Phase:** [P1f.21](README.md), part 1 ([the order](README.md#the-order-two-parts))
**Status:** **done 2026-11-19** ([§ Record](#record)). Planned that day as *the kernel's prefix*, re-planned on the
user's two rulings and unblocked on the user's word (*"Yes, :ein inconsistent"*, *"unblock S1f.21.10"*). The file keeps its first name; a stage's id and file are not moved.
**Group:** [F](modifications.md#f-q-m1f399-a-word-of-the-language-renamed), in its ruled form, with items 24 and 25
of [G](modifications.md#g-the-verdict-as-a-rules-conclusion-einkill-and-three-names-as-conventions), which the
rulings answer.
**Waits on:** no ruling. [Q-M1f.399](../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open)
and [Q-M1f.550](../open_questions.md#q-m1f550--the-word-a-rule-gives-the-engine-to-end-a-state-the-argument-of-ein)
are ruled. **In order, it runs after [S1f.21.6](s1f.21.6_surfaces.md) and
[S1f.21.8](s1f.21.8_rules_the_standard_library_states.md) are merged**: it moves the read-out's and the dump's
goldens the first re-banks, and the standard library's manifest the second rewrites.
**Writes:** the grammar (`ein-ir/src/ein.l`, `ein.y`, and the grammar page's generated regions) and the loader;
`ein-core`'s rule; `ein-infer/src/` (`saturator.rs`, `contradiction.rs`, `compile.rs`, `explain.rs`, `events.rs`,
`forbid.rs`, `monotone.rs`); `stdlib/`, one module; the pages that state a rule's form and a state's death;
`AGENTS.md` § The language; the decisions of the two rulings; the goldens.

## The rulings it builds

The user, 2026-11-19, on Q-M1f.399: *"no need to reserver `ein.` prefix; make (false) an stdlib convention; add
rules detecting contradictions infereing (false) and then - we can make another key, not :assert, but special
engine control key `:ein false` which argument is not a fact, it can remain even the same word `false` but see
plans/m1g_logic/p1g.40_proof_theory/ideas2.md and maybe propose better name here"*; and on the word proposed,
Q-M1f.550: *"Yes, :ein inconsistent"*.

So: **a rule states what it concludes under `:assert`, facts, or under `:ein`, a word addressed to the engine.**
The word is no fact: no knowledge base holds it, no rule matches it, it stands under no arrow. There is one word,
`inconsistent`: where a rule that states it fires, the saturation ends and the state is dead.

```lisp
(rule inconsistent ()
  :match (false)
  :ein   inconsistent
  :why   "a state that derives a contradiction is inconsistent")
```

`(false)` is then an ordinary proposition of the standard library's, inferred by rules, held or under an arrow,
and that one rule is what makes a state that holds it dead. No word of the language is renamed and no prefix is
reserved (items 21 to 23 are not built).

## What it builds

1. **The key.** The grammar's rule takes `:ein` *word* where it takes `:assert`. The loader holds the word to the
   engine's words, of which there is one, and refuses another with a line that lists them, so a misspelt word does
   not load in silence.
2. **What the engine does with it.** A firing of a rule that states `:ein inconsistent` ends the node's saturation
   and makes the state dead, and writes no fact. The state's core and its `refuted by` are taken from that firing's
   premises.
3. **The rule from `(false)`**, in a module of the standard library every program gets. After it the kernel reads
   `(false)` by its name nowhere to judge a state: `contradiction.rs`' direct shape goes. Its other shape, a fact
   beside its stored negation, stays the kernel's until [S1f.21.11](s1f.21.11_kill_and_three_conventions.md).
4. **What read *leads to `(false)`* reads *leads to a rule with the key***: the agenda's first half
   ([Q-M1f.313](../../../docs/decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)), *forbid*
   and the static check, each through the module's rule, so that each decides what it decides today.
5. **The surfaces**: `ein saturate --dump` lists the module's rule as any rule; a `fire` event of a rule with the
   key says so; the trace shows the firing that ended the state.

## The readings, each with the default the stage takes

The syntax is one key on one rule of the standard library, shown above and accepted; no corpus program is
rewritten. So the stage does not stop for an examples look: it takes these defaults, records each, and they are
shown at its end; the second is the user's ruling and no default. **It stops only where a default cannot be built, or an answer or a count below moves.**

1. **`:assert` or `:ein`, never both** on one rule: the answer says *"another key, not :assert"*.
2. **The module every program gets**: a module of the standard library, `std.core`, loaded into every program with
   no import line, its rule shown and counted as any rule. **Ruled 2026-11-19 by the user**, verbatim: *"std.core
   always loaded - ok now, later will decide how it could be customized"*
   ([Q-M1f.551](../open_questions.md#q-m1f551--the-module-every-program-gets-stdcore-always-loaded)). It adds a notion to the language, a module no file
   imports; the other reading, a program imports the convention, leaves every program that writes `(false)` and
   imports nothing unable to refute: `(false)` is on 691 lines of 284 files. **How a program customises it is not
   this stage's**: the stage builds no way to leave the module out, to replace it or to add to it, and builds
   nothing that would have to be undone for one. **It may declare** (the user, the same day, correcting the
   stage's brief: *"there is no requirement "std.core must declare nothing" - it may declare"*); as built it
   declares nothing, the one natural declaration, of `false`, being unwritable while `false` is reserved.
3. **A program's own rule may state the key**, from whatever it matches, with no `(false)` between; *forbid* reads
   such a rule as it reads one that concludes `(false)` today.
4. **`refuted by` names the program's rules**: the module's rule is left out of an explanation, so no read-out row
   moves. A core is the frontier of the firing's premises, which is the frontier of `(false)` as today.
5. **A rule with the key fires before any other the agenda holds**, so a route's node stops where it stops today:
   after the firing that concluded `(false)`, with one firing more, the key's.
6. **`:expect (false)`**, a claim's word for an empty $R$, and the grammar's reading of `false` are not this
   stage's: the reserved words are S1f.21.11's (item 29).
7. **The surfaces' words for a dead state stay *holds a contradiction*** (S1f.21.6 put them there the same day,
   [Q-M1f.553](../open_questions.md#q-m1f553--the-words-for-a-state-dead-by-a-fact-beside-its-negation)): the
   sentence is true of a state the rule calls inconsistent, and a read-out row that moved would move 147 dead-root
   programs' goldens again. The other reading, *is inconsistent* on the surfaces, is shown at the stage's end.

## What must hold

- **Every answer**: $R$ of every corpus program by the corpus sweep, each mode against the oracle
  (`ein-infer/tests/explore.rs`), `ein-cli/tests/lever_invariance.rs`, `import_invariance.rs`,
  `ein-infer/tests/agenda.rs`, `first_contradiction.rs`.
- **Every count a read-out prints**: `work`'s saturations and facts beyond root, the cores, `refuted by`. A
  read-out line of the shape golden that moves is a finding, shown, not re-banked in silence. What may move: the
  firing streams (one firing more a dead node), the saturated dump's rule lists, the events.
- **The register**: every open row answers as it does (`python3 utils/wrong_answers.py`), and
  `corpus/regressions.toml` stays right.
- **A state dead by a pair** answers as today, untouched.
- The grammar page's regions and its diagrams are written by their generator (`EIN_BLESS=1`), the new shape and
  the two refusals have their rows in its table (`ein-ir/tests/grammar_pages.rs`) and their sections of
  [`07-diagnostics.md`](../../../docs/ein/user/04-reference/07-diagnostics.md) § 2.
- The module is a file of `stdlib/`: its program under `tests/stdlib/`, its region on a rules page, the manifest.
- The standing rules it rewrites, in place and in the same commits: `AGENTS.md` § The language (*a rule concludes
  a conjunction of literals*, *the agenda fires first what leads to `(false)`*), Q-M1f.313's decision; and the two
  rulings as decisions under `docs/decisions/`.
- The `.einb` container holds a rule's key as it holds its conclusion; `ein solve x.einb` is `ein solve x.ein`.
- A large change to the engine: the gate and the nightly on its last commit; `utils/ci_container.sh` if a DOT
  writer moves (a rule's drawing gains the key).

## Acceptance

`grep` finds no read of `false` by name in a decision about a state in `ein-infer/src/`; the readers that are
left are named (the grammar, a claim, the pair's other half); a file whose rule states `:ein inconsistent` over its
own premises is dead where they hold and alive where they do not; a file that writes `:ein dead` is refused with
the engine's words listed.

## Record

Built 2026-11-19 in the main working copy, in one commit, `68c3d5d0` (the code, the standard library, the corpus,
the goldens, the pages, the decisions Q-M1f.399, Q-M1f.550 and Q-M1f.551, Q-M1f.313 and Q-M1e2.5 amended in place,
and `AGENTS.md`), and this record's.

### What was built

- **The key.** `ein.l` has the token `:ein`, `ein.y`'s `RuleClause` takes `":ein" Name`. The loader
  (`from_ir::engine_word`) holds the name to `ein_core::EngineWord` (one word, `inconsistent`) and refuses, each
  line ending with the engine's words: a word that is none of them, `:ein` beside an `:assert`, `:ein` on an hrule.
  A rule that states it is registered with `assert_: None` and `ein: Some(Inconsistent)`; the loader's passes read
  its conclusion as `()`.
- **What the engine does.** `Plan::ends`. `Saturator::apply` on such a plan calls `firing::end`: a `Prov` (rule,
  premises, absences, universals, activator) is pushed and recorded on the KB, `Kb::end`, a list `Kb::ended` beside
  the facts that a fork inherits; no fact is written, `Firing::derived` is empty and `Firing::ends` true; a KB that
  already holds the record of the same application keeps one and the firing is redundant.
  `contradiction::detect` is the firings that ended the state, then the pairs; its `Direct` shape is gone, and
  `contradicts` reads the pair alone. `Contradiction` is an enum, `Ended(ProvId)` or `Pair`.
- **The agenda.** A candidate of such a rule carries no band bit and is before every other, under the agenda and
  under the written order alone (`ORDINARY`, `Saturator::band`). The first half is what
  `monotone::leads_to_an_end` reads off the plans, a rule that concludes a fact such a rule matches, with the
  negations and pair sides as before. A saturation stopped at its first contradiction stops at a firing that ends
  the state, and before it returns, at a pair too, fires what the facts written so far give such rules
  (`Saturator::end_step`, which matches those plans alone and leaves the pending pass as it was), so a stopped
  state holds every contradiction its facts make.
- **The core and `refuted by`.** `explain` takes a contradiction as a group: a pair's two sides, or what the
  firing rests on, as a derivation does (premises, the activator a node derived, the universals' reads at their
  smallest). The step of a firing that ended the state is the firing, its rule the one named, unless the rule is
  `Rule::implicit`, of the module every program gets: then the steps are those of what it matched, which is the
  `(false)`'s as before.
- **`std.core`.** `stdlib/core.ein`, one rule. `Resolver::resolve_program`, which the loader calls, is
  `resolve_imports` and after it the module's forms, read as a bare import of it written last would be;
  `resolve_imports` itself brings no form of it, so what `ein_ir::resolve_imports` and a container's `PROGRAM`
  hold loads to what the program loads to. A program with no import of its own keeps its forms untouched, as
  before. `Resolver::implicit_names` marks `Rule::implicit`. The word after `:ein` is no name: no import renames it.
- **What reads *leads to a rule with the key***: the agenda; *forbid* (`monotone::refutes`, one classifier for
  *forbid*, `naf_deps` and the detector: the rule states the key, or concludes a fact such a rule matches, or a
  stored negation; such a fact is not followed into N, as `(false)` was not); the static check reads reads, and
  needed no change.
- **The surfaces.** `ein saturate --dump` lists `std.core.inconsistent  :params ()  :ein inconsistent`; a `fire`
  event of such a rule carries `"ein": "inconsistent"` and an empty `derived`; the trace's step reads *States
  `:ein inconsistent`: the state is inconsistent, and the step derives no fact.*; a rule's drawing has an `ein`
  panel with the word, and a step's slice an arrow to it.
- **The container.** Format `einb/2.2`: `ENDED`, kind 14, the firings that ended a saved state, since no fact
  stands for one; a 2.1 reader skips it and a saturation fires the rule again.

### The readings

1. **`:assert` or `:ein`, never both**: taken; refused at load.
2. **`std.core`, loaded into every program**: the user's ruling, built. **Two things taken with it, to show**:
   an import of `std.core` is refused, in a program or a module (one module in one place, last, under full names,
   so a program and its container list the same rules in the same order); and since every load reads the module,
   an `$EIN_STDLIB` that names no standard library now refuses every program, where it refused only a program that
   imported from the library.
3. **A program's own rule may state the key**: built; `examples/features/18_ein_inconsistent.ein`. *Forbid* reads
   it as a refutation.
4. **`refuted by` names the program's rules**: the module's rule is read through; a program's own rule that
   states the key is named, as a rule that concluded `(false)` was.
5. **A rule with the key fires before any other**: built, under the written order alone too.
6. **`:expect (false)` and the grammar's reading of `false`**: untouched. So `false` cannot be declared in this
   stage: no declarator binds a reserved word. **On the coordinator's correction that `std.core` may declare
   relations**: it declares none because the one natural declaration, `false`, is not writable until the reserved
   words are S1f.21.11's; no declaration was tried and dropped, and no claim was touched.
7. **The surfaces' words stay *holds a contradiction***: untouched.

### Old against new, over the corpus

The binary of the tree before the stage, kept aside, against the stage's, `ein solve FILE --json-summary` for
every `.ein` under `examples/`, `tests/` and `stdlib/` as the tree had them, 913 files: exit code, read-out
(stdout) and stderr the same for all 913; of the 668 summaries, no `region`, `queries`, `work`, `claim` or `verdict`
moved and no root fact count (`root.facts`, `facts_by_relation`, `dead`, `owes`). What moved, in the summary alone:
`monotonicity.plans.saturation` and `root.plans`, one plan more, in 668; `monotonicity.reads.static` in 571 and
`reads.dynamic_monotone` in 97, the module's rule's read of `(false)`. No read-out prints either.

One file needed writing again before that: `tests/saved/04_a_saved_node.ein`, a saved file, which the stage's binary
refused as it stood (*carries no module std.core*). It is byte for byte what its program's dump writes, so it was
written again by that dump and carries `(module std.core …)`; its read-out is the one it had. **A saved file
written before this stage does not load.**

The script and the comparison are not in the tree (a sweep of `ein solve` with `--json-summary`, and a diff of the
three outputs and the summary's blocks). The register: `python3 utils/wrong_answers.py`, 16 open rows, each answered
as it was.

### The goldens that moved

- **The shape golden** (`corpus_shapes.md5`, re-banked with `EIN_BLESS=1 EIN_CORPUS_SLOW=1`), 29 987 lines before:
  **4 266 moved**, by kind `load` 689, `saturate` 689, `plan` 689, `match` 689, `naf` 689 (one rule and one plan more a
  program), `oracle` 669 (the dump's plan and read counts) and `region-trace` 152 (a dead root's or a dead node's
  trace, one step more). **25 721 did not move**: every `read-out[…]` (3 445), every `cores[…]` (3 445), `states`,
  `region-lattice`, `explain`, `explain-noalts` (689 each), every `ir[…]` (3 327) and every `dot[…]` (12 075), 537
  `region-trace`, 20 `oracle`, 116 `load`. 240 lines added for the 8 new files; none gone.
- `trace_dead.md`: root's trace, one step more, the module's.
- `corpus_exits.txt`: 34 cells added for the new files, none changed.
- `ein-ir`'s: `grammar_table.txt` (5 rows added for `:ein`; two rows' ranked repairs gain *Insert `:ein`*),
  `grammar_decisions.txt` and `recovery_cut.txt` (the parser's ranked repairs on malformed input, which may now
  shift or insert at `:ein`), `config_coercion.txt` (the rule in each load's listing).
- The grammar's two diagram sources and their pictures (`01-grammar-lexical-own-text`, `01-grammar-phrase-rule`),
  drawn here with PlantUML; the standard library's rule page region and drawing for `std.core`.
- Tests that pinned a count the module's firing moves, each by one: the division by zero's circuit
  (`arith_theory.rs` (22, 11) to (23, 12), `binary_theory.rs` (33, 21) to (34, 21)), the whole node's applications
  of `coupling.rs`' dead node (1 to 2), the firings `contradiction_rules.rs` counts (it now counts those that derive
  a fact). **So `ein render h-space`'s applications of a dead node and `ein saturate`'s firing totals of a dead root
  are one more**, the firing that ends the state; `ein solve`'s rows are not.
- Tests that pinned a rule list or a module list: `saturate_dump_rules.rs`, `strata.rs`, `saved_files.rs`,
  `compile_semantics.rs`; `test_cli.rs`' count of the examples' claims, one more for the new example.

### Checks

On the stage's tree, the gate profile:

- `cargo test --manifest-path ein.rs/Cargo.toml --profile gate --workspace --no-fail-fast`: every target passed but
  `ein-cli`'s `saturate_dump_rules`, one expectation of a rule list not yet extended; extended, and the target run
  again, passed. The sweeps among them: `explore.rs` (every mode against the oracle), `agenda.rs`,
  `first_contradiction.rs`, `corpus_cores.rs`, `region.rs`, `stdlib_coverage.rs`, `import_invariance.rs`,
  `lever_invariance.rs`, `answer_round_trip.rs`, `renaming_invariance.rs`, `dot_wellformed.rs`,
  `read_out_reference.rs` (the read-out examples page did not move), `ein-einb`'s round trips.
- `EIN_CORPUS_SLOW=1`: `id_order_invariance` (after its re-bank), `renaming_invariance`, `corpus_cli`,
  `answer_round_trip` and `lever_invariance`: each passed.
- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`: clean.
- `utils/doc_audit.py --links --check --tracked` (0 findings, links and decision ids), `check_release_voice.py`,
  `check_doc_blocks.py`, `render_diagrams.py --check`, `check_generated.py --bin`, `stdlib_manifest.py`,
  `wrong_answers.py` (16 open rows as they were), `check_tool_mirrors.py`: each passed.
- **Not run**, the coordinator's: `./run_tests.sh`, `--nightly`, `utils/ci_container.sh`. A DOT writer moved (a
  rule's drawing gains the `ein` panel, a step's slice the word), so the container run is part of done.
- After the sweeps one loop of `Saturator::match_ends` was rewritten to index a list where it cloned it, no
  behaviour in it; `ein-infer`'s unit tests, `first_contradiction.rs`, `agenda.rs` and the slow sweeps ran on the
  code as committed.

### Not done, and left

- **The surfaces' other reading** (reading 7): *is inconsistent* in the place of *holds a contradiction* was not
  built; the words are as S1f.21.6 left them.
- **The pair** stays the kernel's check, and `std.contradiction` is loaded into no program.
- **The readers of `false` by its name that are left** in `ein-infer/src/`: `firing.rs`' `build_conclusion` and
  `monotone.rs`' `writes_of` and `refutes`, a false `eq` or `neq` conclusion, which is `(false)` (it writes the
  fact, and judges no state); `hypgen.rs`' `candidate_objects`, the grammar's words the blind rung never guesses as
  objects. Outside it: the grammar and the loader's shapes of `false`, a claim's `(false)`, the printer. `unheld.rs`
  no longer counts `false` among the kernel's relations: a rule that concludes it is read off the plans as any is.
- **The measurement examples** under `ein-infer/examples/` (`shared/census.rs`' `refutes`, `refutation_census.rs`)
  still read a refutation as *writes `(false)` or a negation*: they compile and were not brought to the new
  reading, which differs from theirs only for a rule that states the key itself.
- **Pages**: every sentence found false was changed (the syntax, rules, grammar, reserved names, diagnostics
  §§ 2.31 and 2.32, events, read-out, answer dump, glossary, the knowledge base, cores, saturation, the design
  pages, install, the binary format, the standard library's index and a page for `std.core`). The definitions of
  *dead* were amended where they stand, by a sentence each; the teaching of the key and of *dead* is the pages'
  stage's.

### For the plan's other files

- `README.md` § Stages and `overview.md`: S1f.21.10 done; S1f.21.11 inherits a kernel that reads no `(false)`, a
  flag `Rule::implicit` that an explanation reads through (its contradiction rules in `std.core` would be read
  through with no more code where they conclude `(false)`), and `monotone::leads_to_an_end` for what leads to an
  end.
- `open_questions.md`: the readings 1 and 3 to 7 as defaults taken; under Q-M1f.551, the three consequences to
  show: an import of `std.core` is refused; a saved file written before does not load; an `$EIN_STDLIB` that names
  no library refuses every program. And that `std.core` declares nothing, with why (reading 6).
- `findings.md`: the container's format moved to `einb/2.2` (`ENDED`); `ein saturate`'s firing totals and
  `ein render h-space`'s applications of a dead node are one more; the measurement examples above.
- `open_facts.md` § 8.1's first row and `design.md` § 1.1's table are of the kernel before this stage.

### The coordinator's runs, after the stage

**2026-11-19.** `utils/ci_container.sh` on `946ee788`: exit 0, 165 test binaries, no failure, every step of the
per-commit tier. `utils/ci_container.sh --nightly` on `901f9fa3`, the same code under commits to the plans alone:
exit 0, its 82 steps, no failure. Both in the runner's image, since the stage moves a DOT writer and is a large
change to the engine.

