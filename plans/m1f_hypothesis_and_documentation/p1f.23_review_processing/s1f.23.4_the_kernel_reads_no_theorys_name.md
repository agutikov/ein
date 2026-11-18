# S1f.23.4 — the kernel reads no theory's name, and the built-in symmetry goes

**Phase:** [P1f.23](README.md)
**Status:** done 2026-11-18 in the worktree `build/wt-s1f234` on the branch `s1f.23.4-no-theory-names`, beside
S1f.23.1 (master) and S1f.23.2 (its own worktree); the main session merges it, and one gate, `utils/ci_container.sh`
and one nightly hold the phase's stages on the merged tree. Its readings wait to be filed (§ Record).

## The ruling

The user, 2026-11-18, verbatim:

> In general: ein.rs must expect the minimal shape of the program: rules, relations, ... Not special names like
> `is-a` `type` `instance` `symmetric` `bijection` `slot`; All those names belong to some specific theory and must
> not pollute the kernel.

and, on the main session's census of what `ein.rs` reads by a theory's name (production code, a string literal
compared or looked up) and its proposal of this stage, with the question whether the built-in `__symmetric__` is this
stage's or P1f.21's:

> Yes, let's drop also built-in symmetry.

The census, at the ruling: `grid.rs` and `models.rs`' key (`is-a`, `T`, `slot-partition`, `functional`, `injective`,
`bijective`) go with S1f.23.2; `kb_dot.rs`' and `ir_dot.rs`' `is-a` and `constraints.rs`' `type`/`instance` go with
S1f.23.1; **left for this stage**: `ein-ir/src/from_ir.rs`' `program_relations`, which makes `is-a` every program's
relation beside `eq` and `neq`; and **`__symmetric__`**, the saturator's native mirror (`ein-core/src/terms.rs`'
`SYMMETRIC` and `Kernel::symmetric`, `ein-infer/src/saturator.rs`' `SYMMETRIC` and the mirror, `ein-narration`'s
`mirror`), written by no standard-library module (`std.algebra`'s `symmetric` is a rule), only by
`examples/features/06_symmetric_native.ein` and four witnesses under `examples/ein-bugs/wrong-answers/`
(`git grep -l __symmetric__`). `__closed__`, search control like `open` and `false`, is no theory's name and stays,
P1f.21's.

## What it does

1. **First, the invariance**: every program that writes `(__symmetric__ R)` (the corpus's, and the tests' if any)
   answers the same with that fact replaced by `std.algebra`'s `symmetric` (imported, `(symmetric R)`): R, every
   mode, the oracle, the verdicts of its claims. A program where they differ is shown before anything is removed, and
   is the user's.
2. **The built-in symmetry goes**: the mirror in the saturator and wherever it is read (the agenda, the matcher, the
   narration's `mirror`, the events, the read-out), `Kernel::symmetric`, `SYMMETRIC`; `__symmetric__` is no longer
   reserved (`ein_core::text`, the blind enumerator's exclusion, `dunder_objects.rs`), an ordinary name a program may
   write and which means nothing to the kernel. Its example and the four witnesses are rewritten to `std.algebra`'s
   rule where what they show still stands, or retired with their corpus entries where they show only the mirror, each
   said in the record; their rows in `corpus/regressions.toml` follow (a fixed wrong answer of a mechanism that is
   gone is retired with it, the row's history in git).
3. **`program_relations` names no theory**: `is-a` leaves its list; `eq` and `neq`, the kernel's computed relations,
   stay. A goal or claim over `is-a` in a program that never writes `is-a` is then refused as any unknown relation
   is; the corpus programs that do that, if any, are shown.
4. **A test holds the rule**: every corpus program with every name that is not the kernel's renamed (a fixed
   bijection onto fresh names, the program's text rewritten through the parser, imports and standard-library modules
   renamed alike) gives the same answer, the same claims' verdicts and the same drawings, apart from the names. Its
   tier is the gate's where it fits the gate's budget, else the nightly's, by measurement; a program it cannot hold is
   named with why. The kernel's names are `ein-core/src/terms.rs`' `Kernel` after this stage.
5. **The pages and decisions**: `AGENTS.md`'s standing rule on `__closed__` and `__symmetric__`;
   `03-reserved-names.md`, `ein.rs/07-reserved-engine-strings.md`, the saturation's pages and algorithms where they
   state the mirror; every decision that states it (`git grep -l '__symmetric__\|native mirror' -- docs`), rewritten
   to the answer that stands. P1f.21's `open_facts.md` § 8 lists `__symmetric__` among the names left: a line there
   says it went here.

## Readings and rulings to file

The main session files the user's two rulings above, and this stage's readings (each with its default, written in
its Record), in `open_questions.md` on merge; the stage does not edit that file.

## Acceptance

- Step 1's invariance holds on every program that wrote `(__symmetric__ R)`, or what differs is the user's.
- No other answer moves: the shape golden's lines move only for the programs rewritten or retired, and for what the
  narration of a mirror wrote.
- The crate tests it touches, `cargo test --profile gate --workspace` once in the worktree when no other cargo run
  is going, `utils/doc_audit.py --links --check --tracked`, `utils/check_doc_blocks.py`, `utils/check_generated.py`,
  `utils/check_release_voice.py`, `utils/wrong_answers.py`, green in the worktree; the gate, `utils/ci_container.sh`
  and the nightly once, on the merged tree, by the main session.

## Record

Run 2026-11-18 in the worktree `build/wt-s1f234`, on the branch `s1f.23.4-no-theory-names`, to its end.

### Step 1: the invariance

Each of the five programs that wrote `(__symmetric__ R)` (`git grep -l __symmetric__`; no `tests/` program did),
against itself with every `(__symmetric__ R)` replaced by `(symmetric R)` and `(import std.algebra :symbols
(symmetric))` added, through the binary built from the branch's base (the mirror still in it). Compared: the
`--json-summary` of a bare describe, `--describe-route states`, `--maximal`, `--minimal`, `--single`, `-n 2`,
`--count` and `--no-search`, and `ein oracle`'s: `asked`, `region` (every state, the lines, components, N, the
universe), `verdict`, `queries` (every class), `claim`, root's `dead`/`owes`/`owed`/`commit`, H (`hypgen.emitted`,
`outside`). Claims: none of the five states an `:expect` (`ein test` on each: *nothing to check*, both ways).

| program | k_s / k_c | describe | states route | maximal | minimal | single | -n 2 | count | no-search | oracle |
|---|---|---|---|---|---|---|---|---|---|---|
| `examples/features/06_symmetric_native.ein` | 1 / 1, Solution | same | same | same | same | same | same | same | same | same |
| `wrong-answers/symmetric-written-root.ein` | 1 / 1, Solution | same | same | same | same | same | same | same | same | same |
| `wrong-answers/symmetric-derived-root.ein` | 1 / 1, Solution | same | same | same | same | same | same | same | same | same |
| `wrong-answers/dunder-object-symmetric-by-rule.ein` | 4 / 4, Ambiguity | same | same | same | same | same | same | same | same | same |
| `wrong-answers/symmetric-derived-node.ein` | 1 / 1, Solution | same R † | same R † | same R † | same R † | same R † | same R † | same R † | same | same |

† R, every state, the verdict and every query class the same. **What differs is the static check's verdict, not the
answer**: the `std.algebra` spelling is flagged, *monotone no — 6 sites*, each an activator relation a commitment can
change (`std.algebra.compose`, `converse`, `join-l`, `join-r`, …: `(symmetric near)` derived under a commitment
derives `(converse near near)` and the rest through the module's lemmas), so the route is the cube (L7) where the
native spelling's was the state route, and the cube's `discharging` and `holes` rows appear. The native mark derived at
a node was invisible to the static check. The invariance holds on R, every mode and the oracle; nothing was stopped
for it, and the program is retired (below), so no corpus program carries the difference.

### What went

- `ein-core`: `terms::SYMMETRIC`, `MIRROR_A`, `MIRROR_B`, and `Kernel::symmetric`, `mirror_a`, `mirror_b`;
  `RESERVED` loses `__symmetric__` (10 names), `ENGINE` the three (5 names). `Terms::new` no longer interns `a` and
  `b` ahead of a program.
- `ein-infer`: the saturator's mirror (`SYMMETRIC`, `sym_rels`, `sym_n`, `sym_sym`, `mirror_queue`, `mirror_seeded`
  on `Saturator` and `Snapshot`, `symmetric_rels`, `enqueue_mirror_sources`, `has_pending_mirror`,
  `next_mirror_firing`, the `mirror` event), its unit test `the_mirror_seeds_its_relations_in_name_order`; the
  blind enumerator's exclusion of `__symmetric__` (`hypgen::candidate_objects` leaves out `__closed__` alone, and its
  test now holds `__symmetric__` an ordinary object); L9's mirror clause (`unheld.rs`' `Unwritten::every` and the
  `(__symmetric__ R)` read; the argument's step 4).
- `ein-narration`: the `mirror` kind of the derivation.
- `ein-ir`: `program_relations` names `eq` and `neq` only, `is-a` no longer (step 3).
- The retired flag `enable-symmetric-mirror` stays refused, its reason now *it switched the kernel's native mirror,
  and the kernel mirrors no relation …* (`config::RETIRED`; the grammar table's golden line re-blessed).
- Tests: the three mirror tests of `stdlib_semantics.rs` and the symmetry pair of `dunder_objects.rs` went; added
  `stdlib_semantics.rs`' `the_kernel_mirrors_no_relation_and_symmetric_is_a_rule` (a probe: a fact of
  `__symmetric__` derives nothing, `std.algebra`'s rule mirrors), `engine_names.rs`'
  `a_name_the_kernel_does_not_read_binds_as_any_name_does`, `goal_premises.rs`'
  `a_goal_over_is_a_in_a_program_that_writes_none_is_refused`; `interning.rs`, `alive_invariant.rs`, `ir_semantics.rs`,
  `kb_semantics.rs`, `grammar_pages.rs` held to the one dunder. `cli_semantics.rs`' event cover drops the `mirror`
  row; `h_space_view.rs`, `show_hyps.rs` and `event_cut_control.rs` read `examples/saturation/symmetric/couple.ein`
  where they read the retired `06` (an empty blind H; productive and redundant firings).
- Utils: `fuzz_engine.py` writes no marker; `nl_census.py`'s `KERNEL` and `rule_census.py`'s `ANY_STRUCT` lose the
  name.

### Programs retired, and why

None was rewritten: each showed only the mirror.

| program | why | corpus entry | regression row |
|---|---|---|---|
| `examples/features/06_symmetric_native.ein` | the native mirror and its seed's name order (hazard H1); a symmetric relation by `std.algebra`'s rule is `examples/saturation/symmetric/`'s three | gone | `symmetric-mirror-off` (a lever on the mirror) gone |
| `wrong-answers/symmetric-derived-root.ein` | a mark a rule derives at root, not mirrored by its saturation: the mirror's bug | gone | `derived-at-root` gone |
| `wrong-answers/symmetric-written-root.ein` | its control, the mark written | gone | — |
| `wrong-answers/symmetric-derived-node.ein` | a mark derived at a node, not mirrored: the mirror's bug | gone | `derived-at-node` gone |
| `wrong-answers/symmetric-rule-node.ein` | its control, writes no mark and exists only as the control | gone | — |
| `wrong-answers/dunder-object-symmetric-by-rule.ein` | `__symmetric__` as a kernel dunder among the blind enumerator's objects; it is no kernel name now, and as an ordinary relation only a rule concludes it is a guessable atom (Q-M1f.306's second bullet), so the program would answer the eight states the row calls wrong | gone | `symmetric-by-rule` gone |

`corpus/regressions.toml`: 333 rows → 329. `plans/wrong_answers/fixed_rows.tsv` loses the four rows' lines
(`W13/symmetric-mirror-off`, `W96/symmetric-by-rule`, `W98/derived-at-root`, `W98/derived-at-node`): the regressions
they named are gone. `examples/features/` keeps its numbers, `06` a gap.

### Step 3: `is-a`

No corpus program asks a goal, a claim or a `(solve …)` list about `is-a` without writing it: every corpus program
gives the same root read-out (`ein solve --no-search`, stdout and stderr) with the branch's binary as with the base's,
the six retired apart.

### The shape golden

No line moved but the retired programs': their 234 lines (six programs × 39 ops) went, and `EIN_BLESS=1` per commit
(the programs outside `tests/`) re-banked nothing. `EIN_CORPUS_SLOW=1 EIN_BLESS=1` over every program, `tests/` among them, each permuted: nothing moved either (92 s, eight workers).
The mirror's narration moved nothing, there being no program left that marks a relation; the kernel's three fewer
interned names moved no rendering (the id-order claim).

### Step 4: the renaming test

`ein-render/tests/renaming_invariance.rs`. Every corpus program, as written and with every `NAME` token that is not
the kernel's renamed `n` → `n!~`, in its text, in every local module it imports and in the whole standard library
(written once, renamed, and read through `Source::Override`); the kernel's names are what `Terms::new` interns, with
`:over`'s three values (`ein_ir::expect::Over::NAMES`, new); module names and aliases are kept. The suffix opens with
`!`, the least name character, so the bijection keeps every order by name (a test of it); no corpus name holds a `!`.
Compared on a load of each: the frontend's parse and compact dump, the KB's shape, root's event stream, the plans, the
match, the NAF map, both explanations, every DOT view, and the answer as documents (`corpus_ops::answer`, new: the
summary's blocks, the claims among them, `nogoods.json`, `nodes.json`); the renamed side's suffix taken out and its
scratch paths put back, then on both a hashed DOT id numbered by first appearance, a palette colour one colour, a
column of a `LINE:COL` masked and whitespace collapsed (a printer wraps at a width). The read-out's text is not
compared (it wraps; its content is the summary's). `ein_ir::grammar::name_spans` is new, the lexer's `NAME` spans.

**Tier**: per commit the programs outside `tests/` (615), as `id_order_invariance` and `dot_wellformed`; under
`EIN_CORPUS_SLOW=1` every program (888), a step of the nightly's `deep-corpus` job, unsharded, and of
`run_tests.sh --nightly` and `--dev-host`. **Cost**: the whole corpus 86 s on four P-core threads with four workers
(`utils/bench_env.sh --cores PT:4`, AC, load 12, another stage's `dot` running), 337 s of units, the heaviest
`beyond_arith/laws.ein` 10.8 s; outside `tests/` 156 s of units. Within the default budget: every run is at the sweep's
(`corpus_ops::SWEEP_BUDGET` = 2^14, under 2^20, Q-M1f.304).

**What it found**: 888 programs alike but for the drawings that read a theory's name, listed in the test's `READS`
(a listed view that moves on no program fails a whole run):

| views | the name, and where |
|---|---|
| `dot[ir]`, `ir-levi`, `ir-overlay`, `ir-trace-dag`, `ir-forms` | `is-a`: `ir_dot.rs` draws a fact of it as an edge into a type box |
| `dot[constraints]` | `type`, `instance`: `constraints.rs`' `KERNEL_ONTOLOGY_HEADS` |
| `dot[kb]`, `kb-origin`, `kb-none`, `kb-no-types`, `kb-no-instances`, `kb-since` | `is-a`: `kb_dot.rs` |
| `dot[rules]`, `rules-overlay` | **`forall`**: `rules.rs`' `head_name` reads `std.macro.forall` and `forall` and draws a call as a guard over a context — not in the census at the ruling |

The first three are S1f.23.1's; the fourth is a finding (below). Nothing else: no answer, claim, event stream,
plan or KB shape read a theory's name. On the way the test met `:over`'s values (`all`, `minimal`, `maximal`), names
the loader reads as a keyword's, now `Over::NAMES` and kept.

### Readings, each with its default

1. **The flagged spelling of `symmetric-derived-node`** (step 1 †): the invariance is of R, every mode and the oracle,
   which hold; the static check's verdict differs. *Default*: not the stop condition (the answer is the same), shown
   here; the program is retired.
2. **Retire, not rewrite**, all six (the four witnesses, `06`, and the control `symmetric-rule-node.ein`, which writes
   no mark but exists only as a retired row's control). *Default*: retired with their rows.
3. **`examples/features/` keeps its numbers**, a gap at `06`: the numbering rule is the documentation's (Q-M1f.367).
   *Default*: no renumbering.
4. **The retired flag `enable-symmetric-mirror` stays retired**, refused with a new reason, its field kept for the
   `.einb` layout. *Default*: as written.
5. **The `--events` protocol stays `ein-events/1`**: the `mirror` kind is gone and no other field moved; a reader of
   `/1` reads every stream the engine writes. *Default*: no version bump.
6. **Q-M1f.7's title** is now *The dunder is never bound, …*; its file keeps its slug
   (`Q-M1f.7-the-dunders-are-never-bound-and-the-writeback.md`), since `open_questions.md` links it and this stage
   does not edit that file. *Default*: the main session renames it on merge, with the link, or keeps the slug.
7. **No new decision** for the two rulings: Q-M1f.7 (the dunder) and the pages carry *the kernel reads no theory's
   name*; the rulings take their ids on merge. *Default*: the main session files them and, if it gives them a decision
   file, re-points Q-M1f.7's sentence.
8. **The renaming test keeps** the kernel's names, `:over`'s values, the grammar's words (no `NAME` token), module
   names and aliases (where a file is found), and does not compare the read-out's text. *Default*: as built.
9. **The renaming test's tier**: per commit outside `tests/`, the nightly whole (Q-M1f.290's split). *Default*: as
   built.
10. **The `forall` drawing** (`rules.rs`) is listed in `READS`, not changed: S1f.23.1 rewrites `rules.rs` on master.
    *Default*: S1f.23.1's, or a stage after; the list fails a whole run once it moves no program.

### Findings

- **The rules view reads `std.macro`'s `forall` by name** (`ein-render/src/rules.rs`' `head_name`, `is_guard`): a
  theory's name in the renderer, not in the census at the ruling. Listed in `READS`.
- **The link check fails on HEAD itself**: `utils/doc_audit.py --links --check --tracked` reports 2961 findings, every
  one in `plans/m1f_hypothesis_and_documentation/review/*/README.md`, links to the re-runs' ignored outputs; none
  outside the review bundle.
- **After the merge**: the renaming test's `READS` must shrink with S1f.23.1's drawing. A whole run fails on a listed
  view that moves on no program, so the main session drops each entry S1f.23.1's rewrite cleared, and an
  unlisted view that moves on the merged tree (a drawing S1f.23.1 made read a name) fails it too.

### What the checks printed

All in the worktree, on the branch's last code commit. Another stage's `dot_wellformed` was running the whole time,
hung on one Graphviz layout in its own scratch directory (over thirty minutes); the sweeps here ran beside it.

- `cargo test --profile gate --workspace --no-fail-fast`: 1 353 passed, 1 failed, 1 233 s: `test_cli.rs`'
  `a_query_with_no_expectation_is_never_solved` counted nine files of `examples/features/` with no claim, eight with
  `06` gone; fixed, and `test_cli` (43) and `events_reference` (3, `the_protocol_has_eighteen_kinds`, which an
  earlier run had failed at nineteen) green after. `renaming_invariance` per commit: 3 passed, 19.9 s.
- `id_order_invariance`, `EIN_BLESS=1`, per commit (34 s) and `EIN_CORPUS_SLOW=1` (92 s, eight workers): green, no
  line re-banked.
- `renaming_invariance`, `EIN_CORPUS_SLOW=1`, `bench_env.sh --cores PT:4`, `EIN_TEST_JOBS=4`: 3 passed, 86 s.
- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -D warnings` (one `single_element_loop` in
  `ir_semantics.rs`, fixed), `cargo doc --no-deps -D warnings`: clean.
- `utils/doc_audit.py --links --check --tracked`: *2961 finding(s)*, every one under
  `plans/m1f_hypothesis_and_documentation/review/` (links to the review runs' ignored outputs, untouched here);
  *decision ids: 0 finding(s)*.
- `utils/check_doc_blocks.py`: *doc blocks ok — 493 page(s), 1091 fenced block(s), 752 diagram(s) (700 `.dot`, 52
  `.puml`); rules 1, 2, 3, 4, 5 hold* (five diagrams drawn again by `utils/render_diagrams.py`).
- `utils/check_generated.py`: *9 generators, every file they write as written*.
- `utils/check_release_voice.py`: *release voice ok — 1770 file(s) read*.
- `utils/wrong_answers.py`: *16 rows, 16 runs, 16 regions, each still answered wrong; 0 defects*.
- `utils/check_gate_steps.py`: *the gate's 17 steps are per-commit.yml's, --nightly's 76 are nightly.yml's, and
  --dev-host's 81 are the nightly's and what left it*; `utils/check_tool_mirrors.py`: green.
