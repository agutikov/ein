# S1f.20.3 — `docs/ein/user/` — the black box

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Estimate:** 5 days.
**Status:** **done 2026-11-17** ([§ Record](#record-2026-11-17)). It was `T1f.5.20.3` until 2026-11-08.
**Depends on:** [S1f.20.2](s1f.20.2_the_disposition_table.md)'s table, reviewed.

## What it writes

*"problem → [solve as a black box] → result"*: every page the table sends to `docs/ein/user/`, rewritten
and not copied.

The cheapest and most valuable third. `docs/guide/` moves nearly verbatim (**1** history line in 693, on
2026-08-28), and the reference pages are assembly rather than writing.

Two things are genuinely new, and they are the reason the perspective split is worth doing:

- **`syntax-and-semantics/` states each concept in four representations**: ein-lang, graph, NL and math, as
  the user's outline asks. That is a *format* decision applied uniformly, and it is the thing no current page
  does. The grammar states the syntax, `01-ein-graph/` states the graph, nothing states the maths, and the NL
  is S1f.5.14's subject. Pick the format once, in
  `objects.md`, and hold it.
- **`theory.md`**: a rule set as a *theory*, and how rules interact. Nothing in the tree says this today,
  and it is the concept [M2](../../m2_nl_to_ir/README.md)'s theory selection and theory synthesis are about.
  It may be the single most load-bearing new page in the phase.

## How it moves a page

Each page moves with its inbound links, in one commit ([P1f.20 § Two rules](README.md#two-rules-across-the-stages)).
A page's release-voice count falls to 0 in the same commit, and the baseline is lowered with it.

## The method

### Task T1f.20.3.1 — the format, in `objects.md`

The four representations, settled on one page, and shown to the user before the other pages take it.

### Task T1f.20.3.2 — `syntax-and-semantics/`, `guide/` and `reference/`

Each page of the table's `user/` rows, with its links.

### Task T1f.20.3.3 — `theory.md`

The new page.

## Acceptance

- Every row the table sends to `user/` is written there, and its source page is gone or carries the table's
  disposition.
- Every concept page states its concept in the four representations.
- The link check is green, and the release-voice baseline has no line for a page under `docs/ein/user/`.
- `./run_tests.sh` is green.

## Record 2026-11-17

### How it ran

The coordinator fixed the shared rules once (the release voice and the boundary, decisions cited by file, links to a
sibling page without an anchor, the concept-page format, the anchor map each writer hands back) and wrote
`user/README.md`; nine forks in this worktree wrote the pages, each returning an anchor map (old `path#anchor` → new
`path#anchor`, a default line for a source wholly theirs): read-out and dumps; configuration and the command line;
diagnostics; the language reference; the core concepts; the solving concepts; the standard library; the guide,
install and the standard of proof; word problems. The API's usage limit stopped six of them once mid-work; each was
resumed where it stopped, nothing lost. The pages were committed first (`7a80f8c8`, `861a8cd7`), with the test-held
pages' path constants; then each group's sources were deleted and every reference re-aimed, a commit a group, by one
script over the maps: links in every tracked `.md` (decisions, the kernel pages still standing, the reasoning and
ein.rs trees, `plans/`, `docs/history/`), Rust comment links, `.ein` and `corpus.toml` bare paths, the repo-relative
paths written as text (help strings, test messages, workflows, scripts), a link label's file name and its `§` number
where the new heading carries one, and the short names written as text where the section cited is the new page's.
Every `.ein` file kept its line count. master was merged twice (S1f.20.4, then S1f.20.5 and S1f.20.7): the
modify/delete conflicts taken as deletions, every content conflict taken from master and this stage's re-aims applied
again over it by the same maps, and this stage's pages re-aimed into the pages the other two moved.

### The pages

`docs/ein/user/` (40 pages) and `docs/ein/standard-of-proof.md`.

| page | rewritten from |
|---|---|
| `README.md` | new: the four parts, the reading paths of `kernel/README.md`'s audience table (user side), the conventions |
| `install.md` | `docs/install.md` |
| `guide/README.md`, `01`–`04` | `docs/guide/` near verbatim (its transcripts byte for byte) |
| `guide/05_jack_drinks_coffee.md` | `01-ein-graph/04_jack_drinks_coffee.md`, its sketch made a program that runs |
| `guide/06_zebra_walkthrough.md` | `inference/zebra_walkthrough.md`, recast against today's derivations (root, a supposition dead alone, *therefore* in the one state) |
| `guide/examples.md` | `03-ein-lang/03_examples.md` |
| `guide/word-problems.md` | P1f.6's `word_problems.md` and `beyond_arithmetic.md`, results only (the user's ruling 3) |
| `syntax-and-semantics/README.md` | `01-ein-graph/README.md`; states the four representations once |
| `objects.md`, `relations.md`, `facts.md` | `01_kb.md` §§ 1–4, 6, 7, 9 · `03_ein_model.md` §§ 4, 5.1 |
| `ontology.md` | `03_ein_model.md`, its resolved seams stated as what holds |
| `rules.md` | `02_rules.md` §§ 1, 4, 5, 6.1, 6.2 · `02_patterns.md` (what a pattern means) |
| `constraints.md` | `02_rules.md` § 3 |
| `properties-and-operators.md` | `07_stdlib_api.md` (`std.algebra`) · `02_rules.md` § 3.2 · `08_self_describing.md` § 3 |
| `obligations.md` | m1d `obligation_forms.md` (form G) · `domain_contract.md` (C1–C4) · m1d README §§ S1d.2.3, S1d.2.4 |
| `hypotheses.md` | `01_grammar.md` (hrule, the solve lists, `:no-search`, `__closed__`) · `defined_behaviour.md` § 4.27 |
| `theory.md` | new: a rule set as a theory, how its rules interact, grounded in the stdlib's theories |
| `macros.md` | `01_grammar.md` (macro, import) |
| `query.md` | `01_grammar.md` (solve, query, `:expect`, config) · M1e2 `region_language.md` §§ 3–5, `region_examples.md` · m1c README § S1c.1.2 |
| `solutions-and-models.md` | `solution_semantics.md` § 5 and what §§ 1–4 mean to a reader (the definition is the reasoning tree's) |
| `natural-language.md` | `03-ein-lang/09_natural_language.md` (test-held; its numbered sections kept) |
| `reference/README.md` | `03-ein-lang/README.md` |
| `reference/grammar.md` | `00_ebnf.md` (two generated blocks; old § 4 is § 3) |
| `reference/syntax.md` | `01_grammar.md` · `02_patterns.md` (its § 3) |
| `reference/reserved-names.md` | `06_reserved_names.md` |
| `reference/stdlib.md` | `07_stdlib_api.md` · P1f.6 `design.md`, `binary_arithmetic.md`, `binary_logic.md`, `malformed_numbers.md`; every module (L-5) |
| `reference/cli.md` | `solve_interface.md` §§ 1, 2, 4 · `defined_behaviour.md` § 5 · m1c README § S1c.1.3 |
| `reference/config.md` | `configuration.md` (test-held; numbered sections and the defaults block kept) |
| `reference/diagnostics.md` | `defined_behaviour.md` opening, § 1, § 4 renumbered § 2 (4.N → 2.N) |
| `reference/read-out.md`, `read-out-examples.md` | `inference/read_out.md` (numbered sections kept), `read_out_examples.md` (generated) |
| `reference/answer-dump.md`, `region-dump.md` | `inference/answer_dump.md`, `region_dump.md` (`walk.json` dropped: no writer) |
| `reference/rendering.md` | `04_dot_rendering.md` (§ Reverse parse, the DPO mode and the trace views dropped: no command offers them) |
| `reference/glossary.md` | `kernel/glossary.md` (the process terms and the walk's terms dropped) |
| `standard-of-proof.md` | `kernel/standard_of_proof.md`; Q-M1e.1, Q-M1e.2, Q-M1e.4 cited as decision files |

Every concept page has `## In Ein`, `## As a graph`, `## In words`, `## In mathematics` in that order. No page names a
crate, module, test file or process file (the install commands name the `ein.rs/` paths they need), states a measured
number (the two `ein test` transcripts' clock is masked), or has a release-voice line.

### The table's rows, by status

**Done, the source deleted with every reference re-aimed**: `configuration.md`, `glossary.md`, `standard_of_proof.md`,
`inference/{answer_dump,read_out,read_out_examples,region_dump,zebra_walkthrough}.md`,
`01-ein-graph/{03_ein_model,04_jack_drinks_coffee,README}.md`,
`03-ein-lang/{00_ebnf,01_grammar,02_patterns,03_examples,04_dot_rendering,06_reserved_names,07_stdlib_api,09_natural_language,README}.md`
(the folder is empty; a link to it names `reference/`), `docs/guide/` (five pages), `docs/install.md`, m1d
`domain_contract.md` and `obligation_forms.md`, M1e2 `region_language.md` and `region_examples.md`, P1f.6
`design.md`, `binary_arithmetic.md`, `binary_logic.md`, `word_problems.md`, `beyond_arithmetic.md`. `01_grammar.md`
was named as split in the brief, but the table sends all of it to `user/`, so it is deleted.

**Done, the source kept as a ledger**: P1f.6 `malformed_numbers.md`, rewritten into `stdlib.md` with every reference
re-aimed, restored (`c66da914`) because `utils/decision_census.py` reads its questions and `--check` failed without
it; a note at its head says so. It goes with P1f.6's folder (L-17).

**Split and left** for S1f.20.6, the references to the sections taken re-aimed: `defined_behaviour.md` (opening, § 1,
§ 4 → `diagnostics.md` § 1 and § 2; § 5 → `cli.md` § 5; § 6 not carried), `solution_semantics.md` (§ 5 →
`solutions-and-models.md`; §§ 1–4 are the reasoning tree's), `01_kb.md` (§§ 1–4, 6, 7, 9; § 8 not carried; § 5 the
reasoning tree's), `02_rules.md` (§§ 1, 3, 3.2, 4, 5, 6.1, 6.2), `solve_interface.md` (§§ 1, 2, 4; § 2.4 → `config.md`
§ 2.4; §§ 0, 7 → their decisions; a link naming no section → `cli.md`). For S1f.20.9: the m1c README (S1c.1.2,
S1c.1.3) and the m1d README (S1d.2.3, S1d.2.4).

**Not done**: none. Not this stage's: `hypothesis_space.md`'s `h-space.json` section stayed with S1f.20.4.

**Citations written as text and left** (no link, so no anchor to re-aim), for S1f.20.6: `defined_behaviour.md § N`
(read 1.N as `diagnostics.md` § 1.N, 4.N as § 2.N, 5 as `cli.md` § 5); the M1e2 record's `read_out.md` § 6.2, 8.x, 9,
13 in the crates (L-7), and the string `region_answer.rs:725` writes into every state file (`read_out.md § 8.2`, a
section of that record); `kernel/inference/README.md`'s file tree; `01_kb.md`'s `03_ein_model.md §5`; the register's
rows and two witnesses' `malformed_numbers.md`.

### Findings closed or taken

L-5 (`stdlib.md` covers every module), L-8 (the generator's links), L-10's line 340 (the release note links
`docs/ein/user/01-install.md`; the comments are S1f.20.6's), L-13 (the exit table was right: `-T` aborts with exit 2,
nothing on stdout, no summary, an empty `--dump-answer` directory; the `exhausted` row and `region.stopped.why`'s
`deadline` were wrong), L-15 for every page this stage wrote, and S1f.20.4's finding on `(relation ?R)` (`syntax.md`
and `relations.md`: a rule may conclude the membership fact, so the pattern matches every declared relation and every
name a rule concluded it of).

### Tests and generated files changed

Path constants: `read_out_reference.rs`, `read_out_examples.rs`, `answer_dump.rs`, `region_dump.rs`,
`nightly_steps.rs`, `config_reference.rs`, `test_cli.rs` (the schema check reads `cli.md` and `config.md`),
`grammar_pages.rs`, `natural_language_page.rs`, `guide_programs.rs`, `guide_transcripts.rs` (its list adds 05, 06,
`examples.md`, `word-problems.md`). `utils/read_out_examples.py` writes the new page, its prose in the release voice.
`grammar_pages.rs`' generated block headers name no tool (re-blessed, a two-header diff). `ein-render`'s
`dump/region.rs` writes `region-dump.md`'s path into every oracle dump, so `corpus_shapes.md5` was re-banked with the
slow set: 647 lines, every one `::oracle`. The help strings and `help_shape.txt` for `--dump-answer`, `oracle --dump`
and `render region`. The release-voice baseline re-banked once for seven counts that fell (`52daae7e`).

### Verification, on the branch

The tracked link check: 0 findings, every decision id resolving. The release-voice check: passing, no line under
`docs/ein/user/`. The process grep over the pages. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`. On
the gate profile: `read_out_reference`, `read_out_examples`, `answer_dump`, `region_dump`, `nightly_steps`,
`region_modes`, `help_surface`, `config_reference`, `test_cli`'s schema check, `grammar_pages`, `fuzz_properties`,
`natural_language_page`, `template_wellformedness`, `guide_programs`, `guide_transcripts`, and `id_order_invariance`
(the bless). `utils/decision_census.py --check`. Every `lisp` block outside the guide loaded as the guide's test loads
one: fifteen form shapes became `text` blocks, and the eight left are fragments or refused programs the text names.
No gate, by the brief.

### Findings outside the stage

1. **The binary prints the development** beyond L-12: help texts carry `(P1a.8)`, `(S1.6.1)`, `(M1e2)`, `(M1c)`,
   `(M1f T1f.5.14.7)` and `ein saturate`'s *"Benchmark + state dump for the Saturator"* (`ein --help`, the golden
   `help_shape.txt`); a `(query …)` keyword note says *"since M1e2 S1e2.1a.4"* (`ein-ir/src/grammar.rs:487`); the
   forbid refusal *"forbidden since M1e2 S1e2.2.5"* (`ein solve examples/ein-bugs/naf-upward-closure.ein`); the
   `$EIN_STDLIB` refusal names `utils/stdlib_manifest.py --write` (`EIN_STDLIB=/tmp/x ein solve examples/zebra2.ein`);
   `config_reference.rs:610` asserts *"retired at M1e2 {stage}"*. Owner by default: L-12's (S1f.20.9).
2. **`config_reference.rs` requires the test harness's `EIN_*` names on the user page** (`EIN_BLESS`,
   `EIN_TEST_JOBS`, …): `config.md` § 4 keeps them in a table naming no file; `EIN_BIN` and `EIN_RULE_MODE`, read only
   by scripts, are dropped. Default: S1f.20.9, with the other tests that read the process.
3. **`natural_language_page.rs` reads `plans/…/open_questions.md` and `utils/nl_census.py`** (its lines 69–70,
   98–100): a project test reading the process. Default: S1f.20.9.
4. **`stdlib/README.md` describes the old import model** (*Auto-closure*, *tree-shaking*, `derive-join join-l join-r`
   listed) and states measured times, and no table row covers it: `grep -n 'Auto-closure\|tree-shaking' stdlib/README.md`.
5. **The SEND + MORE headers and Q-M1f.187 say a bare `ein solve` stops at the region budget**; today it describes R
   whole, one state, exhausted: `ein solve examples/beyond_arith/send_more_money.ein`.
6. **The old pages disagreed with the binary** where the new ones were checked, each fixed in the new page and left in
   the split source for S1f.20.6 to delete: a load refusal *":expect does not name …"* that does not exist (the real
   one: `examples/broken/load/expect_omits_the_goal.ein`); `(eq ?x)`, `(neq ?x)`, `(absent a b)` and `(false ?x)` in a
   rule are parse errors; the *forbid* refusal also stops `ein show hyps` and `ein render h-space`; `and`/`or` take
   one argument or more; no DPO rule mode; chapter 3 credited `domain-elimination` with what
   `std.bijection.range-elimination` derives.
7. **Rendering**: `ein render program` draws a written `(not …)` fact as a dashed arrow with no `¬`, as it draws a
   hypothetical edge, and a ternary declaration's schema between its first two types only; `ein render constraints`
   draws an import as a «import» node. `--trace` prints a re-derivation of a given fact as a *Derives* step.
8. `(import nosuch)`'s refusal ends `(None)`: `kb load error: … module not found at nosuch.ein (None)`.

### Defaults taken, each the user's to revise

1. The four representations' format and order (In Ein, As a graph, In words, In mathematics), fixed by the
   coordinator rather than shown to the user before the other pages took it (T1f.20.3.1), since the stage ran
   autonomously.
2. `diagnostics.md` renumbers `defined_behaviour.md` § 4 as § 2 (4.N → 2.N, same order).
3. `cli.md` numbers its sections as `solve_interface.md` did (§§ 1, 2.1, 2.2, 4.x, 5), with § 3 the exit codes.
4. Patterns are `syntax.md` § 3, not a page; `grammar.md` drops the old § 3 (*gone*) and renumbers.
5. The glossary drops the walk's terms rather than marking them retired.
6. One running example (two people, two chairs) across obligations, hypotheses, query and solutions.
7. `word-problems.md` carries a cut-down copy of a word-problem theory and one example program whole, so that its
   blocks run under the guide's test; they can drift from the files they copy.
8. `malformed_numbers.md` kept as a ledger rather than the census changed.
