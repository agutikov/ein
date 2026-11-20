# S1f.21.18 — the `hrule` form and the keyword `:hrules` removed

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-20 on the user's ruling of [Q-M1f.567](../open_questions.md#q-m1f567--the-hrule-form-dropped-for-a-rule-that-concludes-open-f): *"The special form (hrule ...) is not required
anymore so we remove it and keyword :hrules."* **Done 2026-11-20**, on the user's word to run the phase to its end (the record below).
**Waits on:** [S1f.21.12](s1f.21.12_hypotheses_as_facts.md), which makes a rule that concludes `(open F)` a
hypothesis rule and reads the form as one.
**In place of:** [S1f.21.16](s1f.21.16_hrules_lists_activators.md), which the user dropped (*"not start S1f.21.16,
will drop it after :hrules removed"*): its stage file is deleted with this stage's last commit.
**Writes:** every program, page and test that writes the form or the clause; the grammar and the loader;
`ein-infer/src/hrule.rs` and `mechanisms.rs`; `utils/gen_zebra2_variants.py` and the other scripts that name it.

## What it does

A hypothesis rule is written as a rule, and its activators as facts:

```lisp
;; before                                         ;; after
(hrule guess (?R ?T)                              (rule guess (?R ?T)
  :match  (is-a ?x ?T)                              :match  (is-a ?x ?T)
  :assert (?R ?x)                                   :assert (open (?R ?x))
  :why    "guess: is {?x} {?R}?")                   :why    "guess: is {?x} {?R}?")
(solve :hrules (guess (chosen Drink)              (guess chosen Drink)
                      (served Drink)))            (guess served Drink)
```

An activator is a fact the program writes, anywhere in it (the user: *"by "Activate anywhere" I meant in program,
not necessary in :hrules list"*). Then the form `hrule` and the keyword `:hrules` are deleted: no second spelling
is kept (the phase's rule, no backward compatibility).

## What it touches, counted today

| what | count | the command |
|---|---:|---|
| `.ein` files under `examples/`, `tests/` and `stdlib/` that write `(hrule` | 119 | `grep -c '(hrule ' -r examples tests stdlib --include='*.ein' \| awk -F: '$2>0' \| wc -l` |
| of them, files that write the `:hrules` clause | 53 | `grep -rl ':hrules' examples tests stdlib --include='*.ein' \| wc -l` |
| pages under `docs/` that name `hrule` | 93 | `grep -rl 'hrule' docs --include='*.md' \| wc -l` |
| crate sources and tests that name it | 112 | `grep -rl 'hrule' ein.rs/crates --include='*.rs' \| wc -l` |
| scripts under `utils/` | 16 | `grep -rl 'hrule' utils \| wc -l` |

The four generated zebra files are `utils/gen_zebra2_variants.py`'s, and two of them are *`zebra2` with the
`(hrule guess …)` and the `:hrules` clause deleted and nothing else*: the generator and that sentence of
`AGENTS.md` are written again for the new spelling.

## Tasks

1. **The rewrite, by a script**, every program at once; then **the invariance**: each program's `ein show hyps`
   and its summary's `region` before and after, the same, since S1f.21.12 reads the form as the rule it becomes.
   A program that differs is shown, not banked.
2. **A hypothesis rule that is declared and not activated** (a clause that names none of it) becomes a rule with
   no activator: nothing to write. A program that activates one hypothesis rule from several declared keeps the
   others unactivated.
3. **The form and the keyword deleted**: the grammar, the loader, `hrule.rs`, the conflicts of `mechanisms.rs` that
   name a hypothesis rule, the events' and `ein show hyps`' rung name; the refusals that go and the ones that
   stay, each with its row of the diagnostics page.
4. **The pages**, with [S1f.21.14](s1f.21.14_the_pages_of_open.md): the reference, the guide, the reasoning pages'
   ladder; the decisions the ruling supersedes, rewritten to the id that carries it.
5. **The scripts and the generators**, each with its `--check`.

## What must hold

$R$ of every corpus program as S1f.21.12 left it; every mode against the oracle; `ein-cli/tests/import_invariance.rs`;
no `hrule` left in a project file but where a page says the form is gone, by `grep`; `check_generated.py`.

## Acceptance

`grep -rn 'hrule' ein.rs/crates/ein-ir/src/ein.y` prints nothing; a program that writes `(hrule …)` or `:hrules` is
refused with the parse's own message; the gate, the nightly and the container gate.

## Record

**Done 2026-11-20**, in a subagent of its own on the branch of its worktree from `bbf64c39`. Commits: `766514f6` (the language, the engine, the corpus and the tests), `5f04ade4` (the goldens), `5cba9872` (the pages and the decisions), `86f5f8d7` (the scripts, the editor grammar, AGENTS.md), `e44e1402` (a test's doc), `6a14f656` (the shape golden on the rebase), and this record; rebased onto `b9e803fa` (S1f.21.13's merge: two examples written in the new spelling, whose comments name no `hrule`; the conflicts were `examples/README.md`, one line, and the shape golden, banked again). Three helpers of the stage's own, forks of it, reworded the corpus's
prose, the decisions and the pages of `docs/ein/`; each change is in the commits above, read and checked here.

### Task 1, the rewrite and the invariance

**The "before"**: with a release build of the branch point (`build/s18/before-bin/ein`), every corpus program's `ein
show hyps` (stdout, stderr, exit) and its `ein solve FILE --json-summary` (exit, stderr, the summary's `region`), 934
programs, banked twice: the two banks agree byte for byte, stderr included (the engine is deterministic on them).

**The rewrite, by a script** (`build/s18/tools/rewrite.py`, on `einsexp.py`, a reader of Ein that keeps positions and
comments; both in the worktree's `build/`, not in the tree), every program at once:

- `(hrule N P … :assert X …)` becomes `(rule N P … :assert (open X) …)`: the whole conclusion under `open`, so a
  conclusion that was no fact stays no fact and is refused, now at the parse (Q-M1f.622).
- `(solve … :hrules (H I1 I2 …) …)`: each item that is a list, `(a b)`, is the activator `(H a b)`; a bare name
  activates nothing and goes (the engine read it so: `hrule::hrule_activators` took list items alone; a bare name
  named a parameterless hrule, which needed no activator); `()` activates nothing. The activators stand where the
  solve stood when it emptied, with the comment lines that stood before the clause inside it, else just before the
  solve; a solve left with no keyword goes (Q-M1f.621).
- A saved file's inner forms are read too, inside `(program …)` and `(module …)` (V-41): `tests/saved/02` and `04`.

| what | before | after |
|---|---:|---:|
| `.ein` under `examples/`, `tests/`, `stdlib/`, `corpus/` that write `(hrule` | 120 (142 hrules) | 0 |
| of them, writing `:hrules` | 53 (48 clauses, 81 activators written) | 0 |
| `(solve …)` forms removed, left empty | — | 39 |
| files rewritten by the script | — | 120 |

Then the comments: `build/s18/tools/prose.py` reworded *hrule* in comment lines, and a fork read every changed file's
prose (40 files: 30 `.ein` headers, `corpus/corpus.toml`'s notes, 19 titles of `corpus/regressions.toml`, the
catalogues and READMEs), with no `.ein` header growing or shrinking a line. Identifiers stay (Q-M1f.625).

**Inline programs**: the crates' tests and the docs' blocks, by `rust_hrules.py` (100 forms in 41 Rust files) and
`rust_clauses.py` (30 clauses in 14), then read and fixed by hand where a test's subject was the form (below); the
pages by the docs fork, every command re-run.

**The invariance, with the same build** (`before-bin`, which still read both spellings), on the rewritten corpus: 925
of 934 the same (`show hyps`' stdout and exit, the region, `solve`'s exit); 9 differed, each explained:

| program | before (old spelling) | the same build, new spelling | what became of it |
|---|---|---|---|
| `broken/load/ein_on_hrule.ein` | refused: `:ein` on an hrule | loads | the refusal goes with the form; fixture deleted (Q-M1f.624) |
| `wrong-answers/eq-arity-hrule.ein` | refused at load | **panic** in `compile.rs` | S1f.21.12's loader read no hypothesis under `open` for `eq` at another arity: fixed (Q-M1f.627); refused as before |
| `wrong-answers/hrule-assert-var.ein` | refused at load | **panic** in `firing.rs` | the conclusion-variable check skipped every `(open …)`: fixed; refused as before |
| `wrong-answers/hrule-eq.ein` | loads, H empty, R {∅} | `(open (eq …))`, refused at the parse | rewritten by hand through a parameter, `(h eq)` (Q-M1f.629); the same build then proposed the four `eq` facts (a wrong answer of S1f.21.12's build), fixed: H empty, R {∅} as before |
| `wrong-answers/macro-bare-conclusion-hrule.ein` | refused: a bare `?x` | loads, `(open ?x)` a form | the place is gone; witness and row deleted (Q-M1f.624) |
| `wrong-answers/solve-hrules-activator-arity.ein`, `-no-such-hrule.ein`, `-no-such-relation.ein` | refused by the clause's checks | load, activators as facts | the checks go with the clause (Q-M1f.623); witnesses and rows deleted |
| `syntax/query-full.ein` | loads, R {∅} | refused: the goal names `drinks` | `relations_written` read `hrules` and not the hypothesis registry: fixed; loads, R {∅} |

**The invariance with the final build** (`build/s18/final-bin/ein`, the tip), every program left in the corpus
against the "before": **928 of 928 the same** (master's two examples, merged after the bank and written in the new spelling, have no "before"; their goldens were banked on the rebase) — `show hyps`' stdout and exit, the summary's region, `solve`'s exit.
Stderr: 741 runs differ only in the note's words (`H, from \`…\`` for `H, built by the rung \`…\``) and the read-out's
*no hypothesis rule proposes it*; 48 runs of 24 programs differ in a refusal's words, every one still refused with
the same exit: the parse's positions where `(hrule` became `(rule`, `rule 'h'` for `hrule 'h'`, *writes in a rule*,
*duplicate rule name*, and the shapes now refused at the parse (`(open (and …))`, `(open (absent …))`,
`(open (not ?g))`). The regression rows' `refused` strings were rewritten from the binary (18 rows).

### Task 2, a hypothesis rule declared and not activated

A rule with parameters and no activator compiles no plan (`compile::activators_for`), so nothing is written: the
programs where a clause named none of a rule's activators kept no activator. A parameterless rule needs none, as the
parameterless hrule did. No program in the corpus declared a parameterised hrule no clause activated.

### Task 3, the form and the keyword deleted

- **The grammar**: `ein.y` loses the `hrule` alternative of `Rule`, the `:hrules` clause, `Hrules`, `HruleItem`,
  `HruleItems`; `ein.l` the two tokens. `hrule` is a name; `(hrule guess (?R ?T) …)` is a fact refused at the parse,
  `unexpected \`?R\`: a variable`, and `:hrules` an unknown keyword. The reserved words are ten
  (`grammar::RESERVED_WORDS`, `ein_core::text::LEXER_KEYWORDS`); `:hrules` is no lattice keyword of a query's.
- **The loader** (`from_ir.rs`): no `hrule` head; no hrule registry (`Program::hrules`, `add_hrule` gone;
  `Program::every_rule()` reads every rule once, whole); the refusals that went: `:ein` on an hrule, an hrule's
  conclusion of more than one fact (Q-M1f.271, now the parse's `(open (and …))`), an hrule concluding `(open …)`
  (Q-M1f.620), the `:hrules` list's three checks, the duplicate's word (`duplicate rule name`). **Every check that
  read an hrule reads a hypothesis rule** (Q-M1f.627): interning, the relations written, the unbound conclusion, the
  relation places, `eq`/`absent`/`false` at another arity through a parameter, the negated names, the unimported
  macro, the alive-set invariant's constants, the names index (`Kb`'s registry names, which left the hypothesis rules
  out). A hypothesis `(open F)` is read as `F` where a check reads what a state can hold (`from_ir::stated`).
- **The engine**: `ein-infer/src/hrule.rs` deleted; `hypothesis_rules.rs`, `HypothesisRules`, compiles one plan per
  activator the state holds of each rule that concludes a hypothesis, and drops an `eq`/`neq` instance (Q-M1f.32).
  `mechanisms.rs`' `hypothesis_rules` and its notes (*rule 'r' proposes no candidate*); `unheld.rs`, `open.rs`,
  `invariant.rs`, `scope.rs` (`Why::NotProposed`, key `not-proposed`), `structure.rs`, the read-out (*no hypothesis
  rule proposes it*).
- **The words (V-53)**: the mode `hrules` is `hypothesis-rules`; `ein show hyps`' note *H, from \`MODE\`: N
  candidates*; the h-space read-out's H row *· from \`MODE\`*; h-space.json's `mechanisms.hypothesis_rules`,
  `hypothesis_plans`, a row's `a_hypothesis_rule_proposes_it`; the `load` event's `hypothesis_rules`; `ein saturate
  --dump`'s *;; Hypothesis rules (N)* lists the hypothesis registry with the activators the knowledge base holds
  (Q-M1f.626, Q-M1f.630).
- **The tests**: the inline programs rewritten; the tests whose subject was the form or the clause rewritten to the
  new subject or dropped: `solve_lists.rs` keeps the two lists; `conclusion_shapes.rs`' `a_hypothesis_is_one_fact`;
  `negated_names.rs` and `from_ir`'s cases that wrote `(open (not ?x))`, now unwritable (V-56), dropped;
  `macro_bodies.rs`' hypothesis site reads a `:match` alone; `grammar_pages.rs`' rows *in an hrule* (20) dropped and
  two rows added, *the removed hrule form* and *the removed :hrules*; `grammar_decisions.rs`' counts.

### Task 4, the pages and the decisions

The docs fork: 49 pages and 10 diagram sources (each picture redrawn), the diagnostics page rewritten section by
section with no section deleted or renumbered (§ 2.20 the two lists; § 2.23's and § 2.28's headings, whose anchors no
link used; § 2.30 and § 2.2 lose their hrule rows), every command a touched page shows re-run. The decisions fork:
Q-M1f.271 (*a hypothesis is one fact*), Q-M1f.32, Q-M1f.129 and Q-M1f.114 rewritten in place, slugs kept; 40
decisions reworded. Q-M1f.115 and Q-M1f.116 rewritten in place here, since `open_questions.md` links them
(Q-M1f.628). The ruling is filed: `docs/decisions/hypotheses/Q-M1f.567-a-hypothesis-rule-is-a-rule-that-concludes-open.md`.
Generated, by their generators: `01-grammar.md`'s regions and its three changed `.puml` (and pictures),
`09-read-out-examples.md`, the stdlib pages' regions, `04-api.md`'s block (89 facts: zebra2 writes its five
activators). `AGENTS.md`: the zebra variants' sentence, the stdlib bullet, the `:why`, `:ein` and library-names
bullets, and a rule *a hypothesis rule is a rule that concludes `(open F)`*; its pointer is the coordinator's.

### Task 5, the scripts and the generators

`gen_zebra2_variants.py` (the variants are `zebra2` with the rule `guess` and its five activators deleted and nothing
else), `region_synth.py` (writes `(rule guess () … :assert (open (h ?x)))`; the detector's two hundred random
programs written again from it), `fuzz_engine.py`, `guard_split.py`, `rule_census.py` (a rule that concludes a
hypothesis is its own kind), `nl_census.py`, `stdlib_census.py`, `gen_arith_examples.py`, `profile_ein_rs.py`, the
editor grammar and its README (`check_tool_mirrors.py` clean), `utils/README.md`, the nightly's comment. Every
generator's `--check` passes (`check_generated.py`: 9 generators).

### What the search for `hrule` still finds, and why

A case-blind search for `hrule` in the content of every tracked file outside `plans/`, on the rebased tip: 58 files (the same set as before the rebase; master's two new examples name no `hrule`). Each line that names it is one of:

- **An identifier, kept** (Q-M1f.625): a witness's file name or regression id (`hrule-eq.ein`, `kw-or-in-hrule`,
  `macro-absent-hrule`, `hrule-reads-not`, …, 22 witnesses under `examples/ein-bugs/wrong-answers/`, their
  `corpus.toml` entries and `regressions.toml` rows), `examples/branching/09_hrule.ein`,
  `examples/broken/load/hrule_duplicate_name.ein`, `corpus/fuzz_findings/hrule-reads-not.{ein,md}`,
  `ein-cli/tests/hrule_reads_not.rs` and its two test names, and the slugs of Q-M1f.32, Q-M1f.115, Q-M1f.129 and
  Q-M1f.271 in their links (plan files link the witnesses and the decisions; V-55's precedent keeps a slug).
- **A sentence that says the form is gone**: Q-M1f.567, `09-hypotheses.md`, `02-syntax.md` (three),
  `03-reserved-names.md`, `07-diagnostics.md`, AGENTS.md's new rule; the grammar's table's two rows of the removed
  shapes (`grammar_pages.rs`, `grammar_table.txt`), and its row `(solve :hrule (h))`, a misspelt keyword.
- **A deleted refusal line quoted as it was**: `grammar_pages.rs`' `DELETED` list (`solve-keyword`,
  `query-lattice-keyword`), whose sentences are the lines' text when they were deleted.
- **A finding's dedup key**: `corpus/fuzz_findings/hrule-reads-not.md`'s panic signature (`hrule.rs:130`,
  `Hrules::candidates`), which a later find is matched against verbatim.
- **A negative check**: `forms_cut.rs`' list of words no fact dump's header writes, `:hrules` among them.
- **Process data**: `utils/censuses/{hole,node,region}-census.md` (`naf_open_hrules`, a probe's name, and two counts
  taken before), re-taken by their tools and not edited; AGENTS.md's pointer, the coordinator's.

`grep -rn 'hrule' ein.rs/crates/ein-ir/src/ein.y` prints nothing (the acceptance).

### Defaults taken (Q-M1f.621 to Q-M1f.630), for the coordinator to file

None is cited by a project file but as the decisions name them; each is what the stage does until the user rules
otherwise.

| id | question | default the stage takes |
|---|---|---|
| Q-M1f.621 | where a removed `:hrules` clause's activators are written | where the `(solve …)` stood when it empties (the empty solve goes), else just before it, with the comment lines that stood before the clause inside it; a bare name in the clause activates nothing and goes |
| Q-M1f.622 | how an hrule's `:assert X` is written | `(open X)`, the whole conclusion: what was no fact stays no fact, and is refused, now at the parse: `(open (and …))`, `(open (absent …))`, `(open (not ?g))`, `(open (eq …))` |
| Q-M1f.623 | the `:hrules` list's own checks (a listed name is a declared hrule, an activator's count, an argument at a relation parameter a relation of the program) | they go with the clause: an activator is a fact as every rule's (Q-M1f.567's *a rule like any other*); a wrong count activates nothing, a misspelt one is a fact of another relation; their three witnesses (`solve-hrules-*`) and rows deleted. No refusal is added for rules |
| Q-M1f.624 | a fixture or witness whose subject is the form itself | deleted with its entry and row: `ein_on_hrule` (its refusal goes), `hrule_reserved_name` (it became `rule_reserved_name`'s program), `macro-bare-conclusion-hrule` (`(open (id ?x))` expands to `(open ?x)`, a form, and loads) |
| Q-M1f.625 | file names, test names and regression ids that contain `hrule` | kept as identifiers: plan files the stage may not edit link them, and decision slugs are kept (V-55) |
| Q-M1f.626 | the words for the writer (V-53) | the mode `hypothesis-rules`; the note *H, from \`MODE\`: …*; the h-space H row *· from \`MODE\`*; JSON `hypothesis_rules`, `hypothesis_plans`, `a_hypothesis_rule_proposes_it`; the `load` event's `hypothesis_rules`; the outside reason `not-proposed`, *no hypothesis rule proposes it*. The `rung` event and `Mode` keep their names |
| Q-M1f.627 | what reads a rule that concludes a hypothesis where a check read an hrule | every loader check, `F` under `open` read as `F` where a check reads what a state can hold, and the names index; an `eq`/`neq` instance of a hypothesis template is none (Q-M1f.32) |
| Q-M1f.628 | Q-M1f.115 and Q-M1f.116, whose questions the clause asked | rewritten in place under their ids for activator facts, not deleted: `open_questions.md` links both |
| Q-M1f.629 | `hrule-eq.ein`, whose shape `(open (eq …))` the grammar refuses | written through a parameter, `(h eq)`, so the witness keeps its subject and its answer, H empty and R {∅} |
| Q-M1f.630 | `ein saturate --dump`'s hypothesis rules | the hypothesis registry under *;; Hypothesis rules (N)*, each with the activators the knowledge base holds; a rule that concludes facts beside its hypotheses stands in the agenda's section too |

### Findings outside the stage (V-56 to V-60)

- **V-56.** `(open (not ?x))` is a parse error: a hypothesis that is the stored negation of a fact a variable holds
  cannot be written, where an hrule could propose `(not ?x)` (Q-M1f.127 refused it only where the variable could
  hold a name). No corpus program wrote one that loaded; three test cases that did are dropped
  (`negated_names.rs`, `from_ir`'s), and `not-name-hrule-concluded.ein` is refused at the parse.
- **V-57.** `(open ?x)` with `?x` bound by the `:match` to a name writes nothing, in silence, where `(not ?x)` of a name
  is refused at load (Q-M1f.127): what `macro-bare-conclusion-hrule.ein` became (it loaded, and the blind enumerator
  wrote H, three candidates).
- **V-58.** An activator of another count, or a misspelt one, loads in silence, where the `:hrules` list refused both:
  with no other writer the blind enumerator writes H (`(guess at Colour House)`: six candidates on a probe). The
  diagnostics page § 2.20 says so.
- **V-59.** The folded drawing of a rule that concludes `(open F)` draws a dashed self-loop labelled `open` on the
  candidate's octagon (`ein render rule --fold`), which may break the drawing rule *no arrow to or from an arrow*; the
  pages show the binary's output.
- **V-60.** The per-commit `corpus_cli` check of the slow flag failed twice during the stage on
  `examples/beyond_arith/equation_two_unknowns.ein` (4 756 and 5 031 ms of cells against 1 000), and once on the two
  `send_more_money` programs, while another worktree's suite loaded the machine (load average about 10); one
  `ein solve` of that program, old spelling on the old build against new on the new, ran alike. Costs not re-taken,
  as the brief says.

### The gate

`./run_tests.sh` runs on this record's commit; what it gives is written in the commit after it.

`s1f.21.16_hrules_lists_activators.md` is not deleted here, as the brief says: other plan files link it, and the
coordinator removes it.

### What the user must see

- **S1f.21.12's build had three faults on the new spelling**, which the old spelling hid and the invariance found:
  two panics (`eq-arity-hrule.ein`'s shape and `hrule-assert-var.ein`'s, the loader reading nothing under `open`),
  a refusal of a legal goal (`query-full.ein`), and a wrong answer: a rule that concludes `(open (?R ?x ?y))` with
  `?R` bound to `eq` proposed four `eq` facts, R sixteen states where {∅} is right. All fixed here; with the final
  build every corpus program answers as before.
- **The `:hrules` list's checks are gone, not moved** (Q-M1f.623, V-58): a misspelt or short activator now loads.
- V-56, a hypothesis `(not ?x)` cannot be written; V-57; V-59.
- The `20-negation-and-contradiction.md` example of a program `std.contradiction` warns of no longer warned in the new
  spelling (the warning came from the `:hrules` clause writing `guess` as an argument); the docs fork added
  `(relation offered Rule …)` and `(offered guess)` to keep it, the warning line and k_s as before.
