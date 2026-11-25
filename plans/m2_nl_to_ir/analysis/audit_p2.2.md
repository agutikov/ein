# M2 audit, part P2.2 "the formalizer", against the repository at HEAD `458bf90b`

Repository root: `/home/user/work/ein` (every path below without a leading `/` is under it).
Plan files audited, each read whole: `plans/m2_nl_to_ir/p2.2_formalizer/README.md` and
`s2.2.1_contract.md`, `s2.2.2_llama_server_and_client.md`, `s2.2.3_gbnf.md`, `s2.2.4_passes.md`,
`s2.2.5_from_text.md`. Short names below: `README` is the phase README, `s2.2.N` the stage file.
Context read: `plans/m2_nl_to_ir/README.md`, `open_questions.md` (Q9, Q23, Q24, Q25, Q-M2.2), `EinAf.md` Stage B.

Runs: `(scratch, not kept: re-run the commands) p2.2_runs/`
holds every snippet (`sNN_*.ein`) and its output (`sNN_*.out`, `*.nowhy.out` under `--no-require-why`,
`*.single.out` under `--single`), and `cli_flags_and_commands.out`. Binary: `ein.rs/target/release/ein`
(`ein 0.1.0`, stdlib `sha256:292894be…`, checkout). Nothing in the repository was written; `git status` is clean.

The plan was written 2026-08-23 and its files were last touched by link re-aiming (2026-11-17): **every one of
its 64 distinct link targets, anchors included, resolves today**. What is stale is in the prose, the snippets and
the names written in backticks, which no link check reads.

---

## Snippet runs (the table sections A to C cite)

| file | what it is | `ein solve` | result |
|---|---|---|---|
| `s01_readme_theory_as_written` | README:42, `(import std.bijection) (bijective pet-loc) (transitive left-of)` | exit 0 | loads. `(bijective pet-loc)` fires `std.algebra.bijective-properties`; **`(transitive left-of)` activates nothing**: `ein saturate --dump` lists `std.algebra.transitive  :params (?rel)  no plan compiled` |
| `s02_contract_snippet_as_written` | s2.2.1:73-83, the contract's example, as written | exit 1 | `kb load error: (import std.bijection :symbols …) — not provided by the module: bijective-properties at None` |
| `s03_contract_snippet_import_fixed` | the same with zebra2.ein's present import list | exit 1 | `kb load error: …:5:20: query: its :goal writes (drink-loc Water ?h_water), and the program names no relation drink-loc — …` (the fragment's own incompleteness, not a language change) |
| `s04_contract_snippet_completed` | the same, `drink-loc` declared and one fact of it added | exit 0 | loads. No hypothesis mechanism is activated, so `ein show hyps` says `H, from blind: 10 candidates` (`(drink-loc House-1 Norwegian)` among them); R has 160 states; `--single`: `Ambiguity` |
| `s05_relation_no_why` | s2.2.1:74 "the rendering template is optional"; s2.2.1:95 `(relation adjacent House House)` | exit 1 | `kb load error: relation 'adjacent' has no :why "…", which this load requires of every relation, rule and macro`; exit 0 under `--no-require-why` |
| `s06_origin_keyword_on_fact` | s2.2.1:103, `(color-loc Red House-3 :origin generated)` | exit 1 | parse error: ``unexpected `:origin`: a keyword this form does not take`` |
| `s07_hrule_form` | s2.2.1:96, s2.2.3:103, an `(hrule …)` form | exit 1 | parse errors at `?R`, `:match`, `:assert`, `:why` (`hrule` is a name, so the form is read as a fact) |
| `s08_hrules_keyword` | `(solve :hrules (guess))` | exit 1 | ``unexpected `:hrules`: a keyword this form does not take`` |
| `s09_greeting_string_argument` | s2.2.2:160, what `greeting.gbnf` emits, `(greeting "hello")` | exit 1 | ``unexpected `"hello"`: a string`` with the note *a string is natural-language text, written only after :why, :source or :goal-text* |
| `s10_fact_without_source` | s2.2.1:97, `(color-loc Red House-3)` with its declaration | exit 0 | loads (a background fact) |
| `s11_theory_pass_acceptance` | s2.2.4:57-61, `(import std.bijection :symbols (…))`, `(bijective …)`, `(symmetric next-to)`, `(includes right-of next-to)` | exit 0 | loads, **and `symmetric` and `includes` activate nothing**: the goal `(next-to ?a ?b)` has no row. No message |
| `s12_theory_pass_with_algebra_symbols` | the same plus `(import std.algebra :symbols (symmetric transitive includes))` | exit 0 | `next-to` derived both ways (2 rows) |
| `s19_full_name_activators`, `s20_alias_activators` | a bare import with `(std.algebra.symmetric next-to)`; `:as alg` with `(alg.symmetric next-to)` | exit 0 | both derive the 2 rows |
| `s13_query_form` | s2.2.4:136-139, a query with `:goal`, `:goal-text`, `?who_water`, `?h_zebra` | exit 0 | loads, answers |
| `s14_needs_marker_comment` | s2.2.4:107, the marker `;; needs: <property> for <relation>` | exit 0 | a comment; loads |
| `s15_undeclared_arity_type` | s2.2.3:31-32, an undeclared relation, a wrong count of arguments, a `Pet` where the label says `Color` | exit 0 | **all four facts load, in silence** |
| `s16_cyrillic_atom` | s2.2.1:28, RU input: a Cyrillic atom | exit 1 | `'Н' (U+041D) is not ASCII: Unicode is allowed only inside a string literal (after :why, :source or :goal-text) or in a comment` |
| `s17_cyrillic_source_only` | a Cyrillic `:source` and `:goal-text`, ASCII atoms | exit 0 | loads |
| `s18_numbers_are_names` | `(age Ann 7) (age Bob 07)`, goal `(age ?p 7)` | exit 0 | one row, `Ann`: `7` and `07` are two names |
| `s23_wrapper_forms`, `s24_wrapper_with_source` | `(ontology …)`, `(facts …)`, `(hypothesis …)`, `(meta …)` as heads | exit 0 / 1 | each is a fact of a relation so named; a `:source` inside the wrapped fact is a parse error |
| `s25_written_open_and_expect` | a written `(open F)`, `(solve :expect (region …))`, `(query … :over minimal :expect (model))` | exit 0 | loads; `ein test`: 2 expectations held |
| `s21_zebra_slots_no_guess` | `examples/zebra.ein` with its one `rule` form (`guess`) and five `(guess …)` activators deleted | exit 0 | `H, from obligations: 112 candidates`; R is 1 state; the canonical answer |
| `s22_zebra_slots_four_parts_only` | the same with `(config …)` deleted too: only `import`, `relation`, facts and one `query` | exit 0 | `--single`: `Solution — R is one state`; `?who_water = Norwegian · ?h_water = House-1 · ?who_zebra = Japanese · ?h_zebra = House-5` |
| `s26_zebra_slots_four_parts_reversed` | s22's 69 forms in reverse order | exit 0 | the same verdict and bindings |
| `examples/zebra2.ein`, `zebra2-obligations.ein`, `zebra.ein` | as shipped | exit 0 | `Solution`; Norwegian / water / House-1, Japanese / zebra / House-5 |

`cli_flags_and_commands.out`: `ein solve -e` → `error: -e/--exhaustive is retired …`, exit 2; `--feedback` →
`error: unexpected argument '--feedback' found`, exit 2; `--max-set-size 3` → `error: -m/--max-set-size is retired:
a depth cap changed answers …`, exit 2; `ein solve -` → `FileNotFoundError: … '-'`, exit 1; `ein solve /dev/stdin`
reads the program, exit 0; `ein test` on a parse error, exit 2; `ein from-text` → `unrecognized subcommand`.

---

## A. Stale claims

Each row: where the plan says it, what it says, what is true today, the evidence.

### A.1 The language

| # | plan | quote | today | evidence |
|---|---|---|---|---|
| 1 | README:40; s2.2.4:53; s2.2.3:102 | "relations with their signatures; the types and instances"; "the six types"; "facts that declare types and members" | Ein has no types. A declaration's slot names are labels, ordinary atoms no form is checked against; a relation is overloaded by its count of arguments; membership is a fact of a relation the program chooses | `docs/decisions/language/Q23-a-premise-says-what-a-variable-ranges-over.md`; `Q-M1f.372`; `Q-M1f.447`; `docs/ein/user/04-reference/02-syntax.md:152-163`; run s15 |
| 2 | README:42 | `(import std.bijection)  (bijective pet-loc)  (transitive left-of)` | Loads; `(transitive left-of)` is the file's own name `transitive` and activates nothing. A parameterised library rule is activated only by its **full** name, an alias's, or a short name the file lists in `:symbols` from the module that declares it (no re-export) | run s01; s11 against s12, s19, s20; `Q-M1f.236`, `Q-M1f.232`; `02-syntax.md:821-830`; `stdlib/README.md:120-130` |
| 3 | README:65-66; s2.2.1:93 | "a fact with `:source` is layered as a *fact* by the engine's own provenance rule" | The three words are **given** (`:source`), **derived** (`:rule`/`:using`) and **background** (neither), and the distinction is "presentation only" | `02-syntax.md:126-137`; `docs/ein/user/03-syntax-and-semantics/03-facts.md:181-182`; `ein saturate --dump` prints `;; ── BACKGROUND (2 facts) ──` (s01) |
| 4 | README:67-70; s2.2.1:7-9, 103-106 | the marking is decided "with the census's answer on whether the parser accepts an unknown keyword on a fact"; the `:origin` alternative "only if … the parser already accepts an unknown keyword" | Answered by the language: each form takes a closed list of keywords, and a fact's are `:source`, `:rule`, `:using`. Any other is a parse error | `Q-M1e2.28`; `ein.rs/crates/ein-ir/src/ein.y:196-200` (`FactClause`); run s06 |
| 5 | s2.2.1:74 | "the rendering template is optional" | `:why` is required on every relation, rule and macro; `--no-require-why` loads a program without | `02-syntax.md:120-122`; `14-natural-language.md:95-99`; `07-diagnostics.md` § 2.10; run s05 |
| 6 | s2.2.1:75 | `:why "the {?1} lives in {?2}"` | In a relation's `:why`, `{?1}` is the atom `relation`, `{?2}` the relation's name, `{?3}` its first label. The shipped line is `"the {?3} lives in {?4}"`. The snippet's text loads and reads *the relation lives in nation-loc* | `14-natural-language.md:78`; `02-syntax.md:180-183`; `examples/zebra2.ein:371` |
| 7 | s2.2.1:77 | `(import std.bijection :symbols (bijective-properties bijective-setup typecheck-setup))` | Refused at load: `bijective-properties` is `std.algebra`'s, and a file lists a name from the module that declares it. The shipped line is `(import std.bijection :symbols (bijective-setup typecheck-setup))`. The snippet was zebra2.ein's line on the plan's date (`git show ccf78ce9:examples/zebra2.ein`, line 104) | run s02; `examples/zebra2.ein:113`; `Q-M1f.232` |
| 8 | s2.2.1:95 | `(relation adjacent House House)` | Refused without `:why` | run s05 |
| 9 | s2.2.1:96; s2.2.3:103 | "a `rule` / `hrule` form" | There is no `hrule` form and no `:hrules` keyword. A hypothesis rule is a `rule` that concludes `(open F)`, activated as every rule is | `Q-M1f.567`; `02-syntax.md:312-316, 443-449`; `07-diagnostics.md` § 2.31; runs s07, s08 |
| 10 | s2.2.3:31-32, 147-148 | a grammar "guarantees nothing the loader checks: an undeclared relation, a wrong arity, a type mismatch"; "cannot check arity or types; the loader does, at F1" | None of the three is a load refusal. A fact of an undeclared relation loads (every activator is one); a form at another count loads, "in silence, a fact no pattern of the other count matches"; no argument is held to a label. A *type* check exists only as a rule of a theory (`std.bijection`'s `typecheck-arg-0/-1`, which conclude `(false)`: a dead state, not a diagnostic), and only where the program names its membership relation | run s15; `01-grammar.md:364-368`; `Q-M1f.447`; `stdlib/bijection.ein` (`typecheck-setup`, `typecheck-arg-0`) |
| 11 | s2.2.4:93-95 | "the unified model (`04-ontology.md` — no `type` / `instance` keywords, `is-a` all the way up to `T`)" | The page says "There are no types" and "`is-a` and `T` are names this program writes: neither is reserved, and the engine gives neither a meaning". `examples/zebra.ein` writes its membership as two relations it declares, `type` and `instance`; the library takes the membership relation as a parameter | `docs/ein/user/03-syntax-and-semantics/04-ontology.md:3-8, 36-38`; `examples/zebra.ein` (`(slot-partition co-located instance type Attribute House)`); `Q-M1f.436` |
| 12 | s2.2.2:160 | `root ::= "(greeting \"" [a-z]+ "\")"`, "the smallest grammar in `grammars/`" | What it emits is not Ein: a string is written only after `:why`, `:source` or `:goal-text` | run s09; `Q-M1e2.23` |
| 13 | s2.2.3:81 | the old skeleton's `string`, `number` | No number token: `7` is a `NAME`. (The plan marks this paragraph "as it stood before".) | `01-grammar.md:151-153`; `Q-M1e2.24`; run s18 |
| 14 | s2.2.1:28-29; s2.2.4:163-164 | "Input: a problem statement, EN or RU"; "the passes do not translate" | A name is ASCII. A Cyrillic word can be a `:source`, `:why` or `:goal-text` text or a comment, never an atom | runs s16, s17; `01-grammar.md:170-185`; `Q-M1f.65` |
| 15 | s2.2.4:19-23 | a program with relations and facts and no activators "would have loaded … and sat idle" | It does not sit idle. Where root holds no `open` fact and no obligation is activated, the blind enumerator proposes a fact for every pair of guessable atoms of each declared relation of one or two labels, so such a program has many states | run s04 (10 candidates from `blind`, 160 states, `--single`: `Ambiguity`); `02-syntax.md:245-258`; `Q-M1f.570` |

### A.2 The grammar page, the corpus, the tests

| # | plan | quote | today | evidence |
|---|---|---|---|---|
| 16 | s2.2.3:21-27; README:86 | "`01-grammar.md` is the source of truth for what parses, checked against the recursive-descent parser by a 78-case decision table"; "The generator reads the EBNF blocks of that page" | The parser is generated by grmtools (LR(1)) from `ein.l` and `ein.y`. The page holds **no grammar text**: its two generated regions are images of railroad diagrams and one table (§ 1.5). The EBNF, in ISO 14977 notation, is the text of 20 files, `docs/ein/user/04-reference/diagrams/01-grammar-*.puml`, translated rule by rule from the two files by `ein-ir/tests/grammar_pages.rs`. The decision table has 77 cases and holds the parser's readings | `01-grammar.md:3-11, 49-104, 192-324`; `Q-M1f.378`; `ein.rs/crates/ein-ir/tests/grammar_decisions.rs:203` (`assert_eq!(cases.len(), 77, …)`) |
| 17 | s2.2.3:45-46, 62, 72-73 | "`00_ebnf.md`" (four times) | No such file. The page is `docs/ein/user/04-reference/01-grammar.md` | `git log` `3e98f70a` moved it |
| 18 | s2.2.3:61-66 | "`00_ebnf.md`'s two blocks are those files as `grammar_pages.rs` writes them, lrlex rules with their start conditions and Yacc productions, not W3C EBNF; the page's § 3 is gone …; there is no `A - B`" | The regions are no longer the files' own syntax: they are EBNF (ISO 14977), a `.puml` per group, with character classes as special sequences in words (`? one of: A-Z a-z 0-9 _ . - … ?`) and start conditions as a table. The page has a § 3 again, with other content ("What the grammar leaves to the loader"). A set difference is written `one of: …, but none of: …`, in `BAD` and `BADTEXT` only, tokens no production takes. True still: "the lexer being a scanner pass, longest match" | `01-grammar.md:28-39, 82-104, 349`; `diagrams/01-grammar-lexical-names-variables-keywords.puml`; `grep -n 'but none of' docs/ein/user/04-reference/diagrams/01-grammar-*.puml` |
| 19 | s2.2.3:47-49 | "every `.ein` in the corpus (128 entries, `positive` / `stdlib` / `regression` groups and the `load-` / `compile-negative` ones …)" | 933 entries: `positive` 216, `stdlib` 316, `regression` 338, `load-negative` 36, `parse-negative` 27. `compile-negative` has no entry. (133 on the plan's date.) The corpus now holds saved files too (`(program …)`, `(module …)`, `(commit …)` at the top) | `grep -c '^\[\[entry\]\]' corpus/corpus.toml`; `corpus/README.md:144`; `ein.y:463-467` |
| 20 | s2.2.3:120-121; s2.2.4:90-91, 148; s2.2.1:100-102 | "canonical dump (`ein` has one; the harness calls the binary to produce it)"; "parsed by calling the binary to dump it canonically"; "dumped canonically by the binary" | The canonical printer is a **library function**, `ein_ir::dump_canonical`. No command prints a program's canonical text: `ein show` has one subcommand, `hyps`; `--dump-states` is retired | `ein.rs/crates/ein-ir/src/dump.rs:170`; `stdlib/README.md:145-155` ("No command prints it"); `ein show --help`; `ein.rs/crates/ein-cli/src/cmdline.rs:897` |

### A.3 The CLI, the machine outputs, the verdict words

| # | plan | quote | today | evidence |
|---|---|---|---|---|
| 21 | s2.2.5:7, 34, 43-44, 70 | "`ein solve --feedback`"; "`ein solve -e --feedback`" | `-e` is retired and is an error (exit 2). `--feedback` is not an option; `ein-feedback/1` is P2.1's plan and exists nowhere in `ein.rs/` | `cli_flags_and_commands.out`; `grep -rn 'ein-feedback' ein.rs/crates` is empty |
| 22 | s2.2.5:44-46; s2.2.4:65 | "the feedback object says `unique`, `search_complete = true`"; "no `invalid` outcome" | Not words of any shipped output. `ein solve --single` answers `Solution`, `Ambiguity`, `Contradiction` or `Open`; the summary (`ein-summary/2`) carries `region.k_s`, `region.k_c`, `region.exhausted`, `region.root`, and a `verdict` that is `null` when R is described and `{"word": "Solution", "which": "all"}` under `--single` | `ein solve --help`; `p2.2_runs/zebra2.summary.json`; `p2.2_runs/zebra2.single.summary.json` |
| 23 | s2.2.5:79-81 | "The `--max-set-size` / `--max-time` budgets are passed through" | `--max-set-size` is retired ("a depth cap changed answers"). `-T/--max-time` exists and is an abort, exit 2, "never an answer". The budget is `--region-budget FACTS` (default 2^20), which bounds and never changes an answer | `cli_flags_and_commands.out`; `ein solve --help` |
| 24 | s2.2.1:156-157; s2.2.5:76-79 | "`ein_commit` is whatever `ein --version` of that binary says" | `ein --version` prints five lines (true) and none is a commit: version, `protocol ein-events/1`, `container einb/2.2`, `features`, `stdlib sha256:… <where>` | `ein --version` (in `cli_flags_and_commands.out`); `docs/ein/user/01-install.md:68-92` |
| 25 | s2.2.5:89-90 | "`summary.json`" | The schema is `ein-summary/2` (the M2 README, line 307, says `ein-summary/1`) | `p2.2_runs/zebra2.summary.json`, key `schema` |
| 26 | s2.2.5:129-131 | "if `ein` ever reads stdin; today it does not" | `-` is not read (true). `/dev/stdin` as the path is: `… | ein solve /dev/stdin` answers | `cli_flags_and_commands.out` |

### A.4 Paths, pages, other plans

| # | plan | quote | today | evidence |
|---|---|---|---|---|
| 27 | README:92-93; s2.2.5:107-109 | "(no `.txt` exists today)"; "`examples/zebra.txt` is checked in — … beside the two hand encodings" | True of `examples/`. But the puzzle's text is in the tree since 2026-11-13: `nl2smt/puzzles/zebra.txt`, the fifteen clues and the two questions, a line a sentence, with six edits of it (`zebra-minus-15.txt`, `zebra-contradiction.txt`, `-reversed`, `-shuffled`, `-swapped`, `new-sentences.txt`) | `nl2smt/puzzles/`; `nl2smt/README.md:68-70` |
| 28 | s2.2.1:22-24 | "three `docs/decisions/` documents that were never written" | `docs/decisions/` exists now, as the project's decisions (one file a decision). No `M2-*` file is in it | `docs/decisions/README.md` |
| 29 | s2.2.1:190-192 | "in the sense `AGENTS.md` gives `docs/guide/` — they reference `docs/kernel/`" | Neither directory exists. The guide is `docs/ein/user/02-guide/`; the documentation is `docs/ein/` in three perspectives | `ls docs/` → `decisions demo ein lib` |
| 30 | s2.2.1:117-123 | "four pages at a named commit: `01-grammar.md` (what parses), `02-syntax.md` …, `02-syntax.md#3-patterns` …, `04-stdlib/`" | Three pages and a folder of 23. `01-grammar.md` states what parses only as pictures (A.2 row 16). Sizes: `wc -w` gives 3 702 words for `01-grammar.md`, 11 347 for `02-syntax.md`, 84 647 for `04-stdlib/*.md` | `wc -w docs/ein/user/04-reference/01-grammar.md docs/ein/user/04-reference/02-syntax.md`; `cat docs/ein/user/04-reference/04-stdlib/*.md \| wc -w` |
| 31 | s2.2.3:61 | "since M1f S1f.5.5 (2026-11-09)" | M1f is closed and its folder deleted; the stage record is in `git log` only | `git log --diff-filter=D -- plans/m1f_hypothesis_and_documentation` |
| 32 | s2.2.4:57-61; README:94-95; s2.2.1:70-71 | the theory pass yields the import, a `(bijective …)` per relation, the two spatial properties "and **no `rule` form**"; zebra2.ein "is the hand-written program the formalizer is meant to reproduce" | `examples/zebra2.ein` has 15 `rule` forms of its own: 14 puzzle rules (`co-located`, `adjacent-via-*`, `disjunctive-prune-*`, …), none of them in `stdlib/`, and the hypothesis rule `guess`; 12 of its conditions are activators of those rules. It also writes `(bijection-hierarchy is-a)`, `(typecheck-hierarchy is-a*)`, `(includes is-a is-a*)`, `(transitive is-a*)`, without which `(bijective R)` activates only the four cardinality checks. This was so on the plan's date too (15 `rule`/`hrule` forms then): not a change, a premise that never held of that file | `grep -c '^(rule ' examples/zebra2.ein`; `examples/zebra2.ein:118-119`; `stdlib/bijection.ein` (`bijective-setup`); `grep -rn '^(rule \(co-located\|adjacent-via\)' stdlib` is empty |

Seen in the context files, not in mine: `plans/m2_nl_to_ir/README.md:27-30, 281-283` cite a `README.md § EinAf`
with "thirteen components"; the root `README.md` has no such section and does not contain the word (`grep -ni einaf
README.md` is empty). `open_questions.md:80-83` calls `hrule` "the engine's existing" form. `open_questions.md:6-8`
is in section C.

### Claims checked and found true: 46

1. The 64 link targets of the six files resolve, anchors included (one script over the six files).
2. EinAf Stage B is B1 to B4 as the phase restates them (`EinAf.md:224-304`).
3. F12 says *select a theory, do not invent properties* (`plans/followups/f12_rules_and_relations/ideas.md:291`).
4. F13 names the three neural actions (`f13_puzzles_beyond_zebra/ideas.md:632-634`).
5. F7 B puts activators on M2's critical path and names S2.2.4 (`f7_rule_induction.md:236-256`).
6. `:source` is a keyword of a fact (`ein.y:197`).
7. `stdlib/README.md` and `04-stdlib/README.md` exist and describe the library.
8. No `.txt` under `examples/`.
9. The canonical answer is Norwegian / Water / House-1, Japanese / Zebra / House-5 (three runs).
10. `examples/zebra.ein` and `examples/zebra2.ein` are the two hand encodings.
11. The acva compose file exists on this host, with a `llama` service on `ghcr.io/ggml-org/llama.cpp:server-cuda`,
    the model directory mounted `:ro` and named from `.env` (`/home/user/work/acva/packaging/compose/docker-compose.yml:27-35`).
12. `(bijective nation-loc)` is a fact that loads and selects a library rule (s04, s01).
13. A fact with `:source "<a sentence>"` loads (s04, s17).
14. A `(query :goal (and …) :goal-text "…")` with named variables loads (s13); it is zebra2.ein's own shape.
15. The order of a program's forms did not matter on the one program tried (s26). One instance, not a proof.
16. The hand encodings' `:source "condition (2)"` is a citation, not a span (`examples/zebra2.ein:457`, `zebra.ein:289`).
17. `02-syntax.md#3-patterns` exists.
18. `stdlib/MANIFEST.sha256` exists and its SHA-256 is what `ein --version` prints (`sha256sum stdlib/MANIFEST.sha256`).
19. `ein --version` prints five lines.
20. `docs/lib/01`, `10`, `12` exist; F16 cites Wu et al. 2022 (25.3 %), *Process-Driven Autoformalization in Lean 4*,
    Lean Workbook, the 2025 survey and the AAAI 2026 framework (`f16_autoformalization/ideas.md:11, 76, 103, 261`).
21. Q25's entry records that `ollama` is not an alternative (`open_questions.md:227-233`).
22. Q23's working answer names Qwen3-30B-Instruct and Phi-4 14B (`open_questions.md:162-163`).
23. The parser is generated by grmtools from `ein.l` and `ein.y`, and `grammar_pages.rs` writes the page's regions.
24. The lexer is a separate pass, longest match, the earlier rule on a tie (`01-grammar.md:37-44`).
25. The grammar page cannot drift from the parser: `the_pages_grammar_blocks_are_the_parsers`.
26. `corpus/corpus.toml`'s schema is `ein-corpus/2` (line 29).
27. The groups `positive`, `stdlib`, `regression`, `load-negative`, `parse-negative` exist.
28. `bijective`, `transitive`, `converse` are names of library rules or activators (`stdlib/README.md:66-67`).
29. `utils/bench_env.sh` exists and prints the machine state.
30. zebra2.ein has the six kinds, five `*-loc` relations and `right-of`, `next-to`, as `is-a` facts and `relation` forms.
31. `(symmetric next-to)` and `(includes right-of next-to)` are zebra2.ein's lines.
32. `std.bijection` and `std.slots` give the same closed-world inference, and both Zebra encodings are held to the
    published answer (`stdlib/README.md:186-201`, `Q-M1e.25`).
33. `ein solve` exits 0 on every answer; a load error exits 1, 2 under `ein test` (`07-diagnostics.md:203-209`).
34. `04-ontology.md` has no `type` or `instance` keyword.
35. The binary links `snmalloc` and embeds the stdlib (`features` line; `stdlib/README.md:22`).
36. `$EIN_BIN` is `utils/`' convention (`utils/README.md:27`).
37. `--events`, `--trace` and `--json-summary` are options of `ein solve`.
38. `ein solve -` does not read stdin.
39. The kernel has no LLM client, no GBNF and no HTTP: `git ls-files | grep -iE 'gbnf|llama'` finds only plan files.
40. S2.1.1's T2.1.1.2, item 2, is the unknown-keyword question (`s2.1.1_census.md:111`).
41. S2.1.3's recommendation is a Python harness under `einaf/` that drives the binary (`s2.1.3_boundary.md:101-110`).
42. F2 keeps the grammar in the loop (`f2_self_modifying_language.md:8, 30`).
43. Idea 04 has the pipeline sketch with a `source` sentence on each extracted fact (`plans/ideas/04-…md:75-99`).
44. `(import std.bijection …)` brings `std.algebra`, `std.macro` and `std.contradiction` with it (`stdlib/bijection.ein:53-55`).
45. No `.gitmodules`, no submodule (`git submodule status` is empty).
46. `-T/--max-time` exists.

Not checked, being claims about other software: that `ollama`'s API exposes only a JSON-schema `format`; that
llama.cpp ships a `gbnf-validator`; the `grammar` field of `/v1/chat/completions`; that the models named exist
under those names; the image tags `server-cuda` and `server`.

---

## B. What each stage needs from the project, and its state today

### B.0 The four passes, in today's language

A program's top-level forms are `relation`, `rule`, `query`, `solve`, `config`, `macro`, `import`, `trace`, a fact
(`(R …)`, `(not …)`, `(open …)`), and a saved file's three wrappers (`ein.y:44-48, 463-467`). The plan's four
parts cover four of them:

| pass | what it emits today | what changed under it |
|---|---|---|
| **ontology** | `(relation NAME LABEL* :why "…")`, and membership facts of a relation the program chooses (`is-a`; or `type` and `instance`, as zebra.ein) | `:why` is required, a template over the declaration's arrows (`{?3}` the first label). A label is no type. Atoms are ASCII. Nothing reads `is-a` but the rules handed it as a parameter |
| **theory** | `(import std.X :symbols (NAME+))` and activator facts | An activator works only under a name the file can spell: listed in `:symbols` from the module that **declares** it, the full name, or an alias. An unlisted one is inert, with no message (s11). The library's entry points are of two kinds: parameterised rules, activated by a fact of the rule's name (`(symmetric R)`, `(transitive R)`, `(includes A B)`), and facts of relations no module declares that a rule with no parameter matches (`(bijective R)`, `(bijection-hierarchy ISA)`, `(typecheck-hierarchy ISA*)`, `(slot-partition R ISA SUB Super Index)`, `(slot-spatial R S ISA PT)`). Several take more than a relation: the membership relation, kinds. `std.contradiction` comes with `std.algebra`, `std.bijection` and `std.slots`; `std.core` with every program |
| *(no pass)* **hypotheses** | nothing, a `rule` that concludes `(open F)` with its activators, a written `(open F)`, or `(solve :hypothesis-relations … / :no-hypothesis … / :no-search)` | The plan's four parts name no hypothesis mechanism, and the answer depends on which one a program has. Three cases, each run: (a) the library's obligations, which `(bijective R)` with `(bijection-hierarchy …)` and `(slot-partition …)` activate, propose what would pay what root owes: no form to write (s21, s22, `zebra2-obligations.ein`); (b) a hypothesis rule, as both shipped Zebra files write (`guess`); (c) neither: the blind enumerator guesses over every declared relation of one or two labels (s04: 160 states, `Ambiguity`) |
| **instance** | `(R a b :source "…")`, `(not (R a b) :source "…")` | `:source` is the one keyword a stated fact takes; the text is free, any Unicode. An argument is a name or a nested fact; a number is a name, and a quantity that is computed is a numeral of `std.math` (`(n z 5)`) |
| **query** | `(query :goal PREMISE [:goal-text "…"] [:over all\|minimal\|maximal] [:expect CLAIM])`, several a file | A goal is held to a rule's `:match`, and every relation it names is one the program writes (s03's refusal). `:expect` states the answer, and `(solve :expect (region …))` the whole region: the forms a gold answer is written in (`Q-M1c.1`; s25) |

**The acceptance's target shape exists in the tree, in the `std.slots` formulation.** `examples/zebra.ein` less its
one rule, its five activators and its `(config …)` is: two imports with `:symbols`, five `relation` forms with
`:why`, six library activators, the `type` and `instance` facts, 18 condition facts each with `:source`, one
`query`. It has no `rule` form, and `ein solve --single` answers `Solution` with the canonical bindings (s22). Its
condition facts are facts of declared relations over declared instances (`(co-located Englishman Red)`,
`(right-of Green Ivory)`, `(next-to Chesterfields Fox)`), which is the shape s2.2.3:104's facts grammar describes.
The `std.bijection` formulation, the one the contract's example and s2.2.1:70 name as the target, is not of that
shape: 12 of zebra2.ein's 18 condition facts are activators of rules the file writes, with relation names among
their arguments (`(co-located nation-loc Englishman color-loc Red)`).

### B.1 S2.2.1, the contract

| needs | state today |
|---|---|
| the census's answer on an unknown keyword (S2.1.1 T2.1.1.2) | **Answered by the language**, without the census: refused at the parse (A row 4). The `:origin` alternative would be a change to `ein.l`/`ein.y` and to `Q-M1e2.28`; the sidecar is what the present language allows |
| "a sidecar keyed by the form's canonical text" | the canonical text of a form is `ein_ir::dump_canonical`'s, **a library function only** (A row 20). A harness that drives the binary has no command that prints it |
| the four spec pages, cut into excerpts | exist, in another shape: `02-syntax.md` is the page that says each form in text; `01-grammar.md` holds pictures; the EBNF text is the 20 `.puml` sources (8.9 KB together); `04-stdlib/` is 23 pages. There is also `03-reserved-names.md`, with a "Quick reference — from intent to word" (line 24), and the concept pages `03-syntax-and-semantics/01` to `14`, which the plan does not list |
| the stdlib catalogue, "generated from `stdlib/` by a script", with "the manifest SHA at the top" | **does not exist as a file a prompt can take; its parts exist.** One row a module: `stdlib/README.md:63-92` (28 modules: name, file, what it is, entry), hand-written. One page a family: `04-stdlib/README.md`'s table. One region a rule, generated: `<!-- rule MODULE NAME -->` … `<!-- /rule -->` on the `04-stdlib/` rules pages. One sentence a rule: its own `:why`, which every rule carries (571 `rule` forms: `cat stdlib/*.ein stdlib/*/*.ein \| grep -c '^(rule '`). Each rule's parameter list from the binary: `ein saturate --dump FILE` prints `;;   std.algebra.transitive  :params (?rel)` for every rule a program brings. The version: `stdlib/MANIFEST.sha256`, whose SHA-256 is `ein --version`'s `stdlib` line. A census of rules against what activates them is `utils/stdlib_census.py` and `utils/censuses/stdlib-census.md`. What none of these states is **how a property is spelled to be active** (which names go in `:symbols`, which knob facts a module needs beside the activator): that is prose in `04-stdlib/02`, `04`, `05` |
| few-shot: "the Zebra smoke test (text → program)" and "hand-written micro-examples … each three to five sentences" | **partly shipped.** The text: `nl2smt/puzzles/zebra.txt`. The sentence beside its Ein form, for zebra2: `docs/ein/user/02-guide/06-zebra-walkthrough.md` § Condition cross-reference (15 rows). Sixteen small problem programs whose every stated fact carries the sentence it states as its `:source`, each under the problem's text as a comment: `examples/word_problems/*.ein` (`bob1.ein`: *Bob has 5 apples. He gave 2 to Alice. How many apples does Bob have?*), with the guide `02-guide/08-word-problems.md`. Their theory is four modules of that folder (`words`, `gaps`, `readings`, `listed_is_all`), not the stdlib (`Q-M1f.334`). They are one family, arithmetic |
| "the five categories partition the forms of every seed-set gold program" | the seed set is P2.3's and does not exist. Of today's forms, the categories as worded have no place for a `(solve …)`, a `(config …)`, a hypothesis rule's activator, or a `:why` text (which a model would write on every relation and rule it emits) |
| the record fields (`ein_commit`, `ein_version`, `stdlib_manifest_sha256`) | `ein --version` gives the version, the protocol, the container format, the features and the stdlib digest. **It gives no commit** (A row 24) |
| `docs/einaf/contract.md`, `docs/einaf/reading.md` | do not exist; where they may live is section C |

### B.2 S2.2.2, `llama-server` and the client

Nothing of it exists in the tree and nothing of the project has moved under it: no compose file, no `.env`, no
`packaging/`, `scripts/`, `models/`, `grammars/`, `einaf/` or `experiments/` directory, no Python package file
(`pyproject.toml`, `requirements.txt`), no `.gitmodules`. The repository's Python is `utils/` (standard library)
and `nl2smt/` (`nltk`, and `rich` where installed). The template the stage lifts from is outside the repository
and present on this host. The one thing of the project the stage touches is its smoke grammar's output, which is
not Ein (A row 12). T2.2.2.6's columns "parse rate, load rate, answer accuracy" lean on S2.2.4 and on P2.3's seed
set; its RU column leans on A row 14.

### B.3 S2.2.3, GBNF

| needs | state today |
|---|---|
| a source to generate the grammar from | **exists in two forms, neither the one the stage names.** (1) `ein.rs/crates/ein-ir/src/ein.l` and `ein.y`, read through grmtools' readers: Rust, as `grammar_pages.rs` does it (`YaccGrammar`, the lexer's regex syntax trees); 101 rules in `ein.y`. (2) The 20 generated `.puml` files, ISO 14977 EBNF as text, 247 lines, every production of `ein.y` and every rule of `ein.l`, test-held to the two files. The phrase part is mechanical from either. The lexical part is not: the character classes are words in a `? … ?`, and what EBNF "cannot state" is a table and prose (start conditions; longest match; the earlier rule on a tie). So what s2.2.3:66-69 says is written by hand still is: how the 10 reserved words stay out of a `NAME`, how the 15 position words are names in some places and not in others (`Name`, `HeadName`, `DeclName`, `FactHead`, `ValueHead`, `OrdinaryHead` in `ein.y:57-123`), and how a token ends at a separator. The page itself is not a source |
| the corpus as the acceptance set | exists, seven times larger (933 entries), with saved files among it, `stdlib/math/num9.ein` at 47 KB, and 27 `parse-negative` fixtures. Two refusals of the parser are not context-free and no grammar can reproduce them: a text nested past 1 000 parentheses (`07-diagnostics.md` § 1.7) and a byte that is not UTF-8 |
| per-pass grammars restricting "to one set of top-level heads" | the heads are `Form`'s alternatives in `ein.y:44-48`. Three things the table at s2.2.3:100-105 does not have: the theory grammar must put the selected names into the import's `:symbols` list (or write full names), and its activators take other arguments than one relation (B.0); the ontology grammar's `relation` form needs its `:why`; `hrule` is gone, and a synthesised hypothesis rule is a `rule` with `(open …)` in its `:assert` |
| "the loader's" share, "reported … as feedback level F1" | smaller than the stage says: a wrong count, an undeclared relation and a label mismatch are not refusals (A row 10), and an activator that activates nothing is silent by two rulings: a short name the file lists from no import is its own, "and nothing warns" (`Q-M1f.236`), and one that binds a relation parameter to a fact "loads in silence" (`07-diagnostics.md:215-217`; `Q-M1f.1`). What the loader does refuse is `07-diagnostics.md` § 2.1 to § 2.31; those a generated program can reach include § 2.9, § 2.10, § 2.13, § 2.20, § 2.21, § 2.24, § 2.26, § 2.27 |
| the echo round-trip's "canonical dump" from the binary | no command (A row 20). `ein_ir::parse` and `dump_canonical` are three lines of Rust (`stdlib/README.md:151-155`); `ein-ir/tests/dump_goldens.rs` holds the printer |
| `utils/bench_env.sh` | exists |

### B.4 S2.2.4, the passes

| needs | state today |
|---|---|
| a theory selectable "from `std.*` and not written out" for Zebra | **shipped in one formulation** (`std.slots`, s22), not in the other (`std.bijection`, A row 32). The stage's acceptance accepts either; its example, its catalogue lines and the contract's snippet are the second |
| the theory pass's output being enough to solve | with `std.slots`, the six activators are enough: the obligations supply the hypotheses (M1d and M1f shipped that). With `std.bijection`, `(bijective R)` needs the hierarchy facts beside it, and the conditions need rules. With no activated mechanism the blind enumerator answers, and the verdict is its (s04) |
| "the families the stdlib has no theory for (object tracking, temporal)" | the stdlib has grown theories the plan did not know: `std.math.*` (natural numbers, integers, rationals, gcd, binary, conversion, nine-digit numbers), `std.bool.*` (gates, circuits, inputs as hypotheses), `std.negation`, `std.contradiction`, `std.debts`, `std.typing`, `std.closure`, `std.elim` (`stdlib/README.md:63-92`). None is temporal or tracks objects. An order is a declared relation closed by `(transitive R)` (`docs/decisions/stdlib/Q25-…`) |
| `:source` spans, sentence by sentence | the language carries them (s17); `examples/word_problems/` has 16 problem programs written that way |
| the assembled program "loads: `ein solve` exits 0" | exit 0 means it loaded **and** no claim it states was refuted; exit 1 is a load error or a refuted `:expect`, which the stream tells apart (`kb load error:` or a parse line) |
| a reply "parsed by calling the binary to dump it canonically"; assembly "dumped canonically by the binary" | no command (A row 20) |
| `--ontology PATH` (Q9's escape hatch) | the language has the mechanism: a file beside the program is a module, `(import onto :symbols (…))` brings it whole (`02-syntax.md:813-819`) |
| "tested on every gate run without a model" | the gate is `./run_tests.sh`, whose steps `utils/check_gate_steps.py` holds equal to the CI workflow's; it runs no Python test suite of a package today |

### B.5 S2.2.5, `einaf from-text`

| needs | state today |
|---|---|
| `ein solve -e --feedback` | neither exists (A row 21). What ships: `ein solve FILE` describes R; `--single` gives one of four words; `--json-summary FILE` (`ein-summary/2`), `--events FILE` (`ein-events/1`), `--trace FILE.md`, `--dump-answer DIR` (the read-out uncut, `answer.json`, `index.json`, one saved file a state, each carrying the program as written and every module it imports), `--dump-lattice DIR` (`ein-lattice/2`); `ein test FILE --json-report F` (`ein-test-report/6`). The feedback object is P2.1's and not built |
| the budgets passed through | `--region-budget`, `--region-limit`, `-T` (A row 23) |
| `--gold PATH`, "a structural diff of two programs as multisets of canonical forms" | no program-against-program diff exists, and no canonical text from the binary. What ships compares **answers**: a gold program states `:expect`, and `ein test FILE --diff full` reports each claim's models against what the run found, as match, near (one fact off), missing, extra |
| `examples/zebra.txt` | not there; the text exists at `nl2smt/puzzles/zebra.txt`, with an ambiguous edit (`zebra-minus-15.txt`, 32 solutions by `nl2smt/README.md:507-509`) and an unsatisfiable one (`zebra-contradiction.txt`) beside it. `nl2smt/` is external to the project and to the process |
| "the gold trace's named rule firings present in the run's `--trace`" | `--trace` writes markdown, a step a firing, each rule by the name the main file spells or its full name (`14-natural-language.md:143-173`); `--events` has the firings as JSON lines |
| a non-monotone program's answer | a generated program whose rule reads an `absent` another rule can fill is **flagged**: it loads, the read-out's first row, stderr's first line and the summary's `monotonicity.warning` say that nothing is claimed of the answer (`Q-M1f.564`). The plan's outcome classes have no place for it |
| `einaf report`, `experiments/` | P2.5's; nothing exists |

---

## C. Conflicts with standing rules and decisions

1. **Where `einaf`, its prompts, grammars, records and pages may live** (`AGENTS.md` § Edits that go together,
   "The project and the process are two things"; `docs/decisions/docs/Q-M1f.345`). The rule's lists are closed:
   the project is `README.md`, `ein.rs/`, `stdlib/`, `examples/`, `tests/`, `corpus/`, `docs/` but `docs/lib/`; the
   process is `plans/`, `utils/`, the gate, CI and `AGENTS.md`; `c/`, `nl2smt/`, `docs/lib/` are neither. The phase
   creates `einaf/`, `grammars/`, `packaging/compose/`, `scripts/`, `models/`, `experiments/`, `.env.example`
   (in none of the three), and `docs/einaf/contract.md`, `docs/einaf/reading.md` and `examples/zebra.txt` (inside
   the project). No rule says which of the three a new root is. What follows from the rules as they are written:
   - `utils/check_release_voice.py` treats every tracked `.md` and `.ein` outside its `NOT_PROJECT` tuple
     (`plans/`, `utils/`, `.github/`, `run_tests.sh`, `AGENTS.md`, `CLAUDE.md`, `c/`, `nl2smt/`, `docs/lib/`,
     `ein.rs/vendor/`) as the project's prose. So a page under `docs/einaf/`, `grammars/README.md`, a `.md` under
     `einaf/` or `experiments/`, and the comments of a recorded `program.ein`, fail the gate on `M2`, `P2.4`,
     `S2.2.4`, `T2.2.1.3`, an ISO date, a path into `plans/` or `utils/`, and on **`F0` to `F8`**, the research
     plan's feedback levels, which match the followup pattern `F\d{1,2}` (`check_release_voice.py:63-66, 81`);
     s2.2.3:36 writes "feedback level F1". The check reads `.md` and `.ein` only: a `.tmpl`, a `.py`, a `.gbnf`
     or a `.json` outside `ein.rs/` is not read.
   - `Q-M1f.341` (the project documents only its current state): a contract "written down … before a prompt
     exists" (s2.2.1:19-20) is a statement of intent, and a reading list "two hours, stopped on the clock" a step
     of the work; under `docs/` both are project pages.
   - `Q-M1f.349` (a page of the project states no measurement): tokens/s "measured and recorded" (s2.2.3:54-55),
     and the seed-set numbers, cannot stand on a project page; "a measurement lives beside the script that takes it".
   - `Q-M1f.348` and `AGENTS.md`'s generated-files rule: a generated project file "says it is generated and names
     no tool", every generator has a `--check` that `utils/check_generated.py` runs, and the script is written
     down in `AGENTS.md`'s list and `utils/README.md`. s2.2.3:140 puts the "regeneration command at the top" of
     `grammars/README.md`, and the generator in `einaf/grammar/`.
   - The repository's scripts are `utils/`, catalogued in `utils/README.md`; `scripts/get-model.sh` would be a
     second place.
   - `utils/check_doc_blocks.py` reads `docs/ein/` and `docs/decisions/` only, so the numbering rule
     (`Q-M1f.367`) and the diagram rule do not reach `docs/einaf/` as the check stands.
2. **The ids Q23, Q24, Q25.** `AGENTS.md`: "An id is kept and never reused", and a bare id resolves "to exactly
   one file". `open_questions.md:6-8` calls Q23 to Q25 "sticky ids from the cross-milestone pool". In the root
   ledger those three ids are other questions (`plans/open_questions.md:31-33`: a type slot, `:where`, cardinality
   and ordinality), and two have decision files: `docs/decisions/language/Q23-a-premise-says-what-a-variable-ranges-over.md`
   and `docs/decisions/stdlib/Q25-ein-has-no-ordinal-form-an-order-is-a-relation.md`. The phase "decides" Q23
   (the model), Q24 (one GBNF or many) and leans on Q25 (the frontend's language) under those same ids. Q9 has no
   such collision (`plans/open_questions.md:24` gives it to M2).
3. **`:why` is required** (`AGENTS.md` § The language; `Q-M1f.62`). s2.2.1:74 says optional; s2.2.1:95's
   example has none; the ontology grammar and the rule-synthesis path do not mention it. A program without one
   loads only under `--no-require-why`, which "decides whether a program loads, never what a loaded one answers".
4. **A string is `:why`, `:source` or `:goal-text` text and nothing else** (`Q-M1e2.23`): the smoke grammar's
   `(greeting "…")` (s2.2.2:160).
5. **Names are ASCII** (`01-grammar.md` fact 6; `Q-M1f.65`): RU input with no translating pass (s2.2.1:28;
   s2.2.4:163-164) cannot name its things in Cyrillic.
6. **Ein has no numbers** (`Q-M1e2.24`): only the old skeleton's `number` (s2.2.3:81), which the plan already
   marks as past.
7. **Ein has no types, and the kernel reads no theory's name** (`Q23`, `Q-M1f.372`, `Q-M1f.436`): "types and
   instances" as the ontology (README:40), "a type mismatch" as the loader's (s2.2.3:32), `is-a` "all the way up
   to `T`" as the model (s2.2.4:95), "arguments are declared instances" as something a declaration says
   (s2.2.3:104). A grammar parameterised on the atoms the ontology pass listed is the harness's own closure of the
   vocabulary; nothing in the kernel holds a fact to it.
8. **A hypothesis rule is a rule that concludes `(open F)`** (`Q-M1f.567`): `hrule` (s2.2.1:96; s2.2.3:103).
9. **Each form takes a closed list of keywords** (`Q-M1e2.28`): the `:origin` alternative (s2.2.1:103-106).
10. **Imports** (`Q-M1f.232`, `Q-M1f.236`, `Q-M1f.247`): a name is listed from the module that declares it
    (s2.2.1:77 lists `std.algebra`'s name from `std.bijection`: refused); a short name the file lists from no
    import is its own (README:42 and s2.2.4:60: inert).
11. **The answer is a region R; no flag changes R; every answer exits 0** (`Q-M1e2.7`; `AGENTS.md` § The answer):
    `-e`, `--max-set-size` and a verdict that "depends on" a budget (s2.2.5:79-81). `--max-set-size` was retired
    because "a depth cap changed answers".
12. **A non-monotone program loads and nothing is claimed of its answer** (`Q-M1f.564`): the phase reports
    "verdict distribution and answer accuracy" for programs a model wrote, rules among them, and names no flagged
    class.
13. **No binding for another language; `pip install` is not a channel** (`Q-M1a.23`; `docs/ein/ein.rs/04-api.md`
    § 7): **no conflict found.** A Python program that runs the binary and reads `--json-summary` and `--events` is
    what the decision describes, and the phase proposes no package of Ein. Two things the decision's route does
    not give such a harness: the canonical text of a program (A row 20) and the loader's refusals as data, which
    the decision says a caller gets by linking `ein-ir`.
14. **The gate** (`AGENTS.md` § Running the gate: "The tests read only the project"; the local and the CI step
    lists are held equal): "tested on every gate run" (s2.2.4:158-159) is a new step in `run_tests.sh` and in the
    workflow, with `httpx` and a test runner where CI runs.
15. **`AGENTS.md` § Where the work is** lists what may start after M1f (M1g, M1h, M12, M15, M1k, M1n, M1p) and
    does not name M2.

---

## D. Open questions and decisions the phase defers

| what | who the plan says decides | anything since |
|---|---|---|
| **Q9**, ontology provenance | S2.2.4, "reframed as theory *selection*"; `--ontology PATH` as the escape hatch | No types (`Q23`'s decision file, `Q-M1f.372`); the library takes the membership relation as a parameter (`Q-M1f.436`); a local module is importable by path (`Q-M1f.235`). Not answered |
| **Q23**, the local model | S2.2.2 T2.2.2.6, by a measurement on the seed set; "the reading written before the run" | Nothing in the project bears on it. The id is taken (section C, 2). The seed set is P2.3's, so the measurement waits on P2.3 and on S2.2.4's passes |
| **Q24**, one GBNF or many | S2.2.3 T2.2.3.5, by tokens/s; working answer *many* | Nothing. The id is the root ledger's `:where` question |
| **Q25**, the frontend's language | S2.1.3 (P2.1); this phase "assumes its recommendation", Python | `Q-M1a.23` is now a decision file and `04-api.md` § 7 a page: both describe driving the binary. The canonical printer and the loader's errors as values are on the Rust side only. Not decided |
| **Q-M2.2**, must the model be local | S2.2.2, "taken as written": local primary, a hosted arm once | Nothing |
| how four of the five categories are marked: sidecar or `:origin` | S2.2.1 T2.2.1.2, on S2.1.1's census | The census's question is answered: refused (`Q-M1e2.28`). The sidecar's key, canonical text, has no command behind it |
| what a span is: "the sentence, or a character range into the input" | S2.2.1 | `:source` is free text; the word problems write the sentence |
| whether an intermediate representation beside the four parts helps | G8, P2.6 | Nothing |
| whether a synthesised rule is allowed, and when | S2.2.4, by the missing-property marker | `hrule` gone; `:why` required on the rule; a rule that reads an `absent` may flag the program (`Q-M1f.564`) |
| the acceptor: llama.cpp's `gbnf-validator` or "a small Python one" | S2.2.3 T2.2.3.2 | Nothing |
| the third candidate model | S2.2.2, "from the current llama.cpp leaderboards" | Nothing |
| whether `ambiguity.gbnf` exists | S2.4.5 | `hrule`, which Q8 names as the existing form, is gone |
| whether RU works | S2.2.2, a measured column | Atoms are ASCII (s16), so RU input implies naming its things in ASCII |
| `ein` reading stdin | "if `ein` ever reads stdin" | `/dev/stdin` as a path works |
| the hosted arm's parse rate "the first thing GBNF is shown to have been worth" | S2.2.3's note | Nothing |
| **not asked by the plan, raised by today's project**: which hypothesis mechanism a formalized program has; whether the harness passes `--no-require-why` or the model writes every `:why`; which of the two Zebra formulations is the gold; where `einaf` and its pages live; under which ids the phase's verdicts are filed | nobody | each is a fact of section A, B or C |

---

## E. Dependencies

| on | what the phase takes from it | exists | still says it |
|---|---|---|---|
| P2.1 S2.1.1, T2.1.1.2 | the unknown-keyword answer | plan file, not run | yes; the language has since answered it |
| P2.1 S2.1.2 | `ein-feedback/1`, `--feedback`, the outcome words | plan file, not run; nothing built | yes. Its mapping reads the summary's `exhausted` and a verdict type; the summary is `ein-summary/2` now. P2.1's auditor has the rest |
| P2.1 S2.1.3 | the language of the client and the passes; `einaf/` at the root | plan file, not run | yes, as a recommendation; Q25 is open |
| P2.1 README § The sixth outcome, today | which budgets move the outcome | anchor resolves | names `--max-set-size`, retired |
| P2.3 S2.3.1, S2.3.3, S2.3.4 | the seed set, the external adapters' goal shape, Zebra out of every split | plan files, not run | yes. Four of this phase's acceptance bullets need the seed set, which is P2.3's work: a cross-dependency with a phase this one "blocks" |
| P2.4 S2.4.3, S2.4.4, S2.4.5 | the feedback levels, the faithfulness judge, the alternatives wrapper | plan files, not run | yes |
| P2.5 S2.5.1, S2.5.2, S2.5.3, README § Order | the record layout ("written *before* this stage runs"), baseline B2, the metrics | plan files, not run; anchor resolves | yes |
| P2.6 README, S2.6.4 | G8, G9, the link-grammar arm | plan files | yes |
| P2.10 README | the second `docs/einaf/` page | plan file | yes |
| M5 | the paper's § 3 | `plans/m5_presentation/README.md` | not read further |
| M1f S1f.5.5 (and the M2 README's S1f.5.14, S1f.6.20) | the generated parser; `:why`; the word problems | **deleted** with M1f's folder | what they shipped is in the tree: `ein.l`/`ein.y`, `14-natural-language.md`, `examples/word_problems/` |
| F2 | the grammar in the loop stays there | `plans/followups/f2_self_modifying_language.md` | yes, and its first line now says it was pulled into M12 (2026-11-12), the file staying; its trigger still reads "M2 (LLM infra + GBNF for IR) is complete and stable" |
| F4 Q38 | the LLM as extractor | `f4_cross_cutting.md:461` | yes |
| F7 B | activators on the critical path | `f7_rule_induction.md:236-256` | yes; it names S2.2.4 back |
| F12, F13, F16 | select do not invent; three neural actions; the references | `ideas.md` each | yes |
| ideas 01, 04 | GBNF as the firewall; the pipeline | `plans/ideas/` | yes |
| `docs/lib/01`, `10`, `12` | the reading list's groups | exist | not read further |
| acva's compose file | the template | `/home/user/work/acva/packaging/compose/docker-compose.yml`, on this host, outside the repository | yes |
| `utils/bench_env.sh`, `$EIN_BIN` | the measurement wrapper, the binary's name | exist | yes |

---

## F. Estimates, dates used as status, counts

- **Estimates**: README:3 ("3 weeks — 5 stages, 15 days") and the *est.* column of its table (2, 3, 3, 4, 3 d);
  the `**Estimate:**` line of each stage file (2, 3, 3, 4, 3 days); s2.2.1:62 and :162 ("two hours, stopped on the
  clock").
- **Dates as status**: s2.2.3:61 ("since M1f S1f.5.5 (2026-11-09)"); s2.2.5:24 ("the 2026-05 plan"); README:92-93
  ("no `.txt` exists today"); s2.2.5:131 ("today it does not"); "the old plan", "the old S2.1.1", "the old
  T2.3.1.4", "the old P2.2 / P2.3 / S2.4.1 / S2.4.4 / S2.5.2 / S2.6.1" throughout, which name files that are not in
  the tree.
- **Counts**: "128 entries" (s2.2.3:47; 933); "78-case decision table" (s2.2.3:24; 77); "four pages"
  (s2.2.1:117; three and a folder of 23); "five" categories; "six types", "five `*-loc` relations", "≥ 15 facts —
  one per condition" (s2.2.4:53-62; zebra2.ein has 18 condition facts, four of them condition 1's); "the fifteen
  conditions and the two questions" (s2.2.5:108; true of `nl2smt/puzzles/zebra.txt`); "the six outcomes"
  (s2.2.5:122; P2.1's); "four passes … four prompts … four grammars"; "three candidates", "three seeds", "ten
  seeds", "three attempts"; "≥ 30 tok/s", "100-token emission", "`/health` … within 30 s", `--ctx-size 8192`,
  `timeout 120.0`, `max_tokens 2048`, port 9080; "three to five sentences".
