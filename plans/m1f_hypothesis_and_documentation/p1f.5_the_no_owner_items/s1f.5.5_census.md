# S1f.5.5 step 1 — what each position holds today (T1f.5.5.1)

**Stage:** [S1f.5.5](s1f.5.5_the_grammar.md). **Taken** 2026-11-08 on a snapshot of `ec004a48`
(`git archive HEAD` into `build/s1f.5.5/snapshot`), with a release `ein` built from it and the same
snapshot built a second time with the final grammar behind `EIN_PARSER=grm`
([`s1f.5.5_paper_grammar.md`](s1f.5.5_paper_grammar.md) is step 2, and says how). Nothing under
`ein.rs/`, `corpus/`, `examples/`, `tests/`, `stdlib/` or `docs/` moved. The scripts and their
outputs are in [`s1f.5.5_parser_probe/census/`](s1f.5.5_parser_probe/census/), and
[§ Re-taking it](#re-taking-it) has the commands. *Since the user's re-plan of 2026-11-08 the moves this page
puts before the swap, by S1f.5.4's ruling, land in it instead, each credited to its ruling
([S1f.5.5 § The order](s1f.5.5_the_grammar.md#the-order-re-planned-2026-11-08)), and this page's
[estimate](#the-estimate-re-taken) is re-taken again in the stage file.*

## What it found

1. **Every puzzle, every stdlib module and every `tests/` program parses under the final grammar as it
   is written, to the tree today's parser builds, `Loc` for `Loc`**, but for digits, which are a name.
   Of the 390 files under `examples/`, `tests/`, `stdlib/` and `corpus/`, 312 build today's tree and 19
   more differ from it only where digits are written, and in one a range, now the name `1..3`
   (`c_ein corpus`, and with `C_EIN_DIGITS=same` to read past a digit;
   [`grammar_vs_today.txt`](s1f.5.5_parser_probe/census/grammar_vs_today.txt)). Run through
   `solve` and `saturate --dump` with both parsers, 309 of the 312 answer byte for byte alike; the
   other three are load-negative fixtures that import `broken/load/reserved_open_direct.ein`, one of
   the 55 below, so the text of their refusal moves. Every answer that moves with exit 0 is one of the
   19, by S1f.5.3's and S1f.5.4's rulings, and none at the swap, which comes after them
   ([`sweep.txt`](s1f.5.5_parser_probe/census/sweep.txt)). The four files both parsers refuse are the
   parse-negative fixtures, whose text moves.
2. **The final grammar refuses 55 files today's parser accepts; the probe's draft refused 34**, re-taken
   at `ec004a48` with the probe's own harness (`harness corpus`: *hand accepts, grammar refuses: 34*).
   The 21 more are 20 refusal fixtures under `examples/broken/` whose refusal moves from the loader or
   the compiler into the grammar, and `syntax/unary-and-nary`. None is a puzzle, a stdlib module or a
   `tests/` program. Which stay refusal fixtures and which are rewritten is a question for the user,
   in [`s1f.5.5_paper_grammar.md` § The 55 files](s1f.5.5_paper_grammar.md#q2--the-55-files-the-grammar-refuses-that-todays-parser-accepts).
3. **At 44 places today's parser and loader accept a shape the final grammar refuses**, and read it as
   nothing, as something else, or as a name nobody meant. That is of 723 probes: 18 shapes at 40
   positions, and an unknown keyword at 21 places
   ([`matrix.md`](s1f.5.5_parser_probe/census/matrix.md)). 205 probes load today and are a parse
   error after; 94 are a load error today and a parse error after; 14 stop the run today and are a
   parse error after; 55 are a parse error today and parse after, every one a name under the name rule.
   [§ The positions](#the-positions) has them.
4. **67 refusal sites move from the loader, the compiler and the firing into the grammar**: 27 of
   `from_ir.rs`' 52, 3 of `compile.rs`' 10, and 37 in `expect.rs`, `claim.rs`, `imports.rs`,
   `macros.rs` and `firing.rs`, which the stage did not count. Three more of `from_ir.rs`' are dead
   today. [§ The refusal lines](#the-refusal-lines-classified) lists each. This is most of why the
   estimate grows.
5. **Three wrong answers that are not the grammar's, with no register row** (§ [Findings outside the
   grammar](#findings-outside-the-grammar)): an activator that binds a value parameter to a nested fact
   leaves it unbound, so the rule fires for every value; `:assert ()` stores a fact with the synthetic
   name `@empty`, which then joins H as a candidate object, where S1f.5.9's case table says it adds
   nothing; and `(absent ())` holds, where F12's reading makes it never hold.
6. **The trace sub-language has a writer and a reader nobody calls.** `ein_render::trace::trace_to_ir`
   writes `:bind` and `:why` on a step, which S1f.5.11's ruled list for `step` does not take, and
   neither it nor `parse_trace_steps` has a caller or a test (`grep -rn trace_to_ir ein.rs/crates`,
   2026-11-08). The five other event words have no exerciser, as `00_ebnf.md` § 5 says.

**The estimate re-taken: 10 days**, from 8.5 ([§ The estimate](#the-estimate-re-taken)).

## The positions

What the corpus writes at each position, from a plain S-expression reader over the 390 files
(`positions.py`, [`positions.txt`](s1f.5.5_parser_probe/census/positions.txt)); what today's parser and
loader do with each of 18 shapes written there, and what the final grammar does
(`matrix.py`, [`matrix.md`](s1f.5.5_parser_probe/census/matrix.md), one table per position). A shape
the loader *drops* loads and reads as if it were not written, or reads something else; the matrix shows
what each stores or derives.

| position | the corpus writes (390 files) | today, besides what it means | the final grammar |
|---|---|---|---|
| a fact's argument | a name 4 543, digits 105, a nested fact 35, a string 18, `(relation …)` 5, a range 2, `_` 2, `?x` 2, a negative int 1, `(model …)` 1 | `_` and `:k v` dropped, `(p _)` stores `(p)`; `?x` stored as the name `"?x"`; `a?x` and `1a` split into two; `()`, `(and …)`, `(neq …)` stored as nested facts of relations `@empty`, `and`, `neq` | a name or a fact, or `(not F)`; the rest a parse error |
| a nested fact's argument | a name 56, a fact 2, digits 1; a keyword pair 19, in three files | the same drops; a keyword pair dropped | the same; a nested fact takes no keyword |
| a fact's relation | a name 2 720 | `1a`, digits, `$x` a parse error; `a?x` splits | any name but `relation`, `region`, `merged`, `owing` |
| a stored negation's argument | a fact 45, a name 1 (`syntax/negation`) | any value stored: `(not plain)`, `(not 7)`, `(not "?x")`, `(not)` | a fact |
| a fact's `:source`, `:rule`, `:using` | a string 243; a name 3; a list 3 | a non-string `:source` and a non-name `:rule` dropped; `:using` reads the list's arguments that are facts and drops its head and every name, so `:using (x)` names no premise | `:source` a string, `:rule` a name, `:using` a list of facts |
| an activator's argument | (as a fact's) | `_`, `:k v`, a string, `?x` read as nothing: the rule does not fire. A nested fact, `()` or `(and …)` leaves the parameter unbound: the rule fires for every value ([finding](#findings-outside-the-grammar)) | as a fact's |
| a relation's name, signature | a name 1 026; a name 1 541, a list 1 | a non-name *malformed (relation)*; `:k v` in the signature dropped | a name, not a reserved one; names |
| `:why`, `:goal-text`, `:source` | a string 201 + 484 + 36, 7, 243 | a non-string dropped in silence (`:why`), or a load error | a string |
| a rule's `:priority` | digits 475 | any other value dropped in silence; `-5` read as an int | digits |
| a rule's parameter | `?x` 718 | anything else a parse error | unchanged |
| a `:match`, a conjunct, `absent`'s argument | 1 880 premises: a pattern 1 109 (69 over `relation`), `and` 407, `absent` 164, `neq` 98, `not` 36, `forall` 30, `eq` 12, `unknown` 12, `or` 7, and five witnesses' shapes | a name, digits, a string, `_`, `?x` refused at load since W24; `()` read as a pattern that matches nothing; `(absent ())` holds; `(absent :k v)` a compile error in the Python IR's spelling; `(and)` a parse error | a premise; `()` only as the whole `:match`; `(and)` and `(absent)` a parse error |
| a premise's relation | a name 741, `?x` 371, `_` 1 | `_` refused at load; digits a parse error | a name or a variable, not `eq`, `absent` or `open` |
| a premise's argument | `?x` 1 830, a name 128, digits 13, a pattern 2, a range 1, a string 1, `_` 1 | `_` matches any value (W20); a string the name it spells; `:k v` dropped; a range refused at load | a name, a variable, a pattern, or a connective, which a macro such as `forall` takes |
| a guard's argument, at a `:match`'s top | `?x` 204, a name 7, `_` 4, a pattern 2, a string 2; arity 2 107, 1 2, 3 1 | `_`, a form, a keyword pair refused at load (W24); arity a compile error at the first plan | a name, a variable or a pattern; arity 2 in the grammar |
| an `:assert` | a pattern 458, `(false)` 178, `not` 95, `and` 37, `(open …)` 28 | a name, digits, a string, `_`, `?x` stop the run at the first firing, and load where the rule never fires (G-14); `()` stores the fact `@empty` ([finding](#findings-outside-the-grammar)); `(open …)` beside a conjunct refused at load | a conclusion, `()`, or `(open)` / `(open ?R)` alone |
| a conclusion's argument | `?x` 895, a name 93, a pattern 2, a string 2, digits 1 | `_` and `:k v` stop the run at the first firing; `(and …)`, `(neq …)` stored as nested facts | a name, a variable, a pattern, `(not P)` |
| an hrule's conclusion | (as a rule's) | `_` and `:k v` propose nothing, in silence (D-8) | as a rule's |
| a query's `:goal` | a pattern 236, `and` 11, `not` 6, a free head 3, `:kw` 1 | a name, digits, a string, `_`, `?x` read as a goal that *holds: it has no variable*; `()` answers `(model)` | a premise |
| a query's `:over` | a name 3 | any other value refused at load | a name, checked by the loader |
| a claim's facts | names; digits 96, a string 10, nested facts 15, `(not …)` 15 | `_`, `?x`, a range, `:k v` refused at load | ground facts, `(not F)`, `(neq a b)` |
| `:no-hypothesis`, `:hypothesis-relations` | a name 242 | `_`, `?x`, `a?x` read as names | names |
| `:hrules` | `(h …)` 22: a name 43, a list 70 | a string, `_`, `?x`, `()`, `:k v` read as nothing | a name, or a list of names |
| `(config …)` | 5 forms: the two flags, a retired one, an unknown one | a value that is not `true` / `false` refused at load | `true` or `false` |
| `(import …)` | `:symbols` 154, `:as` 3 | `:symbols ()` and `:symbols (_)` refused at load as *empty*; `:as "m"` refused at load | a name; `:symbols` one name or more, `:as` a name not reserved |
| `(macro …)`'s body | 9 bodies, `std.macro`'s two among them | a name, digits, a string, `_` load as a body | a template: any position a parameter |
| `(trace …)` | `step` 4, `:rule` 2, `:using` 3, `:derives` 3 | a name, digits, `_`, `?x`, `(and …)` accepted as `:derives` | a fact, or `(not F)` |

**A keyword the form does not take** (21 places, the matrix's last table): today a `relation`, a `rule`,
an `hrule`, a fact, a nested fact, a stored negation, an `import`, a trace `step`, a pattern in a
`:match` or a `:goal`, and an `and` in either clause drop it in silence, `:where` and `:cardinality`
included; a pattern in an `:assert` stops the run at the first firing; a `query`, a `solve` and a
`config` refuse it at load. The final grammar refuses each at the parse, as [S1f.5.11 § The
lists](s1f.5.11_closed_keywords.md#the-lists) rules.

**What the final grammar accepts that today's parser refuses** (the matrix's other direction, 55
probes): the name rule's names wherever a name goes, `$x`, `1a`, `-7`, `1..5` and digits, including as
a relation's name, a rule's name and an import's module.

**40 000 seeded mutants of the corpus** (`c_ein mutate`, one to three edits each, one core, 5 min 31 s,
2026-11-08; [`mutants.txt`](s1f.5.5_parser_probe/census/mutants.txt)): 21 531 build today's tree and
1 415 differ only in digits; 37 more differ where today's lexer splits a word the name rule reads
whole, `01x`, `=typecheck-arg-0`, `(relation 1y4 T)` (today a fact of `relation`, then a declaration);
9 904 both refuse; 6 537 today accepts and the grammar refuses, by the token it refuses at, a string
1 414 times and `_` 561; and 576 today refuses and the grammar accepts, 556 of them holding a name only
the name rule makes, the first of the other 20 writing `=` as an argument, `(symmetric = next-to)`,
which the rule makes a name too.

## The refusal lines classified

*The grammar's*: a shape a context-free grammar over the token stream states, so the final grammar
refuses it at the parse and the line goes. *The loader's*: what a grammar cannot see, so the line
stays. Line numbers are at `ec004a48`.

### [`from_ir.rs`](../../../ein.rs/crates/ein-ir/src/from_ir.rs): 52 sites, 27 the grammar's

| line | refusal | whose |
|---|---|---|
| 104 | *unexpected top-level form* | grammar |
| 123 | *a top-level (region …) — a region claim belongs under (solve … :expect …)* | grammar: a top-level fact's relation is not `region`, `merged` or `owing` |
| 344, 429 | *(solve …)* / *(query …): unknown keyword* | grammar |
| 425 | *(query …): :hrules chooses the lattice and belongs on (solve …)* | grammar; the sentence's hint is a default of step 2 |
| 672, 685 | *(relation) needs a name*, *malformed (relation)* | grammar |
| 693, 766 | *… shadows a reserved kernel name* (a relation, a rule, an hrule) | grammar: a declarator's name is not `relation`, `eq`, `absent`, `false` or `open` |
| 752, 759 | *({head}) needs name + params*, *malformed ({head} …)* | grammar |
| 1808 | *fact with non-atom head* | grammar |
| 1880, 1900 | *(config …) body expects kw_pairs*, *unknown config flag* | grammar |
| 1893 | *config flag :… was retired at M1e2 …* | grammar; its sentence is a default of step 2 |
| 1959 | *config flag … expects true/false* | grammar |
| 1987, 1997, 2008 | an int, a number, a string flag | dead today: both flags left are booleans, so the arms go |
| 1143–1210, `Found::` | a range (`Range`), `_` as a relation (`WildHead`), a keyword pair inside a pattern (`NestedKw`), a premise that is not a form (`NotAForm`), `_` in a guard (`WildGuard`), a keyword pair as a guard's argument (`GuardNode`) | grammar, six of the eight |
| 1200 | a form as a guard's argument (`GuardNode`) | goes with S1f.5.7, ruled answered |
| 1206 | a guard's variable no premise before it binds (`GuardFree`) | goes with S1f.5.7 |
| 1159 | a relation written as a variable no parameter binds (`FreeHead`) | loader |
| 1532, 1552, 1564, 1575, 1589 | `(open …)` in a `:match`, inside a conclusion, beside another conclusion, with two arguments, with a non-name argument | grammar |
| 1600, 1640, 1648, 1678, 1683 | `(open ?R)` with `?R` no parameter; the `absent` it projects from: none, two, no witness, a compound witness | loader |
| 1302 | an activator binds a relation parameter to a fact | loader |
| 1314 | … to an integer | goes with S1f.5.4: digits are a name |
| 57, 114, 162, 176, 211, 384, 468, 491, 506, 700, 777, 784, 798 | a file, an internal error, two `(solve …)`, the config wrapper, a derivation cycle, a claim naming no relation or a class twice, duplicates, a missing `:match` / `:assert`, a macro's expansion | loader |

### [`compile.rs`](../../../ein.rs/crates/ein-infer/src/compile.rs): 10 messages, 3 the grammar's

| line | refusal | whose |
|---|---|---|
| 585 | *`(absent …)` takes exactly 1 argument* | grammar |
| 640 | *`(eq …)` takes exactly 2 arguments* | grammar |
| 826 | *`(absent …)` sub-plan compiled to no steps* | grammar: `absent` takes a premise |
| 617 | *nested `(or …)` in a `:match` premise* | compiler: an engine limit (no DNF), not a rule of the language, so the grammar does not encode it |
| 278, 435, 792, 877, 894 | 64 variables; an activator's arity; a relation parameter bound to a fact; a relation variable no activator binds, two spellings | compiler |
| 801 | a relation parameter bound to an integer | goes with S1f.5.4 |
| 546, 558, 1037 (drops, not refusals) | an in-`:match` keyword pair, a premise that is not a form, an opaque slot | unreachable: the grammar refuses each shape |

### The rest of the loader, not in the stage's count: 37 sites the grammar's

| file | the grammar's | stays |
|---|---|---|
| `expect.rs` | 17: a claim's shape (`(model …)`, `(or …)`, `(false)`, `(not …)` of one fact, ground, no `_`, no keyword) | `:over`'s three words, 2 |
| `claim.rs` | 11: the region's shape (`(region …)`, `(merged …)`, `(owing …)` of models) | 4: one `owing`, `owing` beside states, a `merged` state the region lists |
| `imports.rs` | 4: a missing module name, `:as` not a name, `:symbols` not a list, an empty list | not found, not provided, `:as` with `:symbols` |
| `macros.rs` | 3: name, parameters and body, malformed, a reserved name | a duplicate |
| `firing.rs` | 2 of `NotAFact`'s 3: a conclusion that is not a fact, an argument no fact can hold (G-14) | a relation parameter its activator binds to a non-name; `UnboundVar` goes with S1f.5.10 |

## Findings outside the grammar

Wrong answers the census met that the grammar does not fix and no register row records. Each program
is in `build/s1f.5.5/probes/` and its output below is today's binary at `ec004a48`.

- **An activator that binds a value parameter to a nested fact leaves it unbound.** `(rule act (?y)
  :match (p ?y) :assert (hit ?y))` with `(act (f a))` over `(p a) (p b) (p (f a))` derives `(hit a)`,
  `(hit b)` and `(hit (f a))`, where it should derive `(hit (f a))` only. `compile.rs` skips a nested
  fact when it seeds the parameters (*a nested Fact does not bind*), which W24 made a refusal only where
  the parameter names a relation. The same holds of `()` and `(and …)` as the activator's argument,
  which the final grammar refuses. **Fixed 2026-11-09** by
  [S1f.5.18](s1f.5.18_an_activator_binds_a_nested_fact.md)'s T1f.5.18.2 (W33, filed by S1f.5.0): an activator
  binds a parameter to a nested fact as to a name, so the program derives `(hit (f a))` alone. *Default:* filed as
  a register row by the stage that owns
  activators; not this stage's.
- **`:assert ()` stores a fact named `@empty`, which then joins H.** `(rule r () :match (q ?x) :assert
  ())` over `(q a)`: `saturate --dump` lists the derived `( :rule r)`, and `solve` answers two states,
  the second `(model (q "@empty") (q a))`, the synthetic head having become one of the program's names.
  [S1f.5.9's case table](s1f.5.9_an_empty_match_fires.md#the-case) says it *loads, adds nothing*. The
  final grammar keeps `:assert ()` as the empty conclusion, so the fix is the engine's. **Fixed 2026-11-09** by
  [S1f.5.9](s1f.5.9_an_empty_match_fires.md)'s T1f.5.9.2 (W34, filed by S1f.5.0): an empty conclusion compiles to no
  template and asserts nothing, so the program answers R {∅} and its state files load.
- **`(absent ())` holds.** `(rule rr () :match (and (p ?x) (absent ())) :assert (hit ?x))` derives
  `(hit a)` and `(hit b)`. Under F12's reading `()` is true and its absence never holds. The final
  grammar refuses `()` anywhere but a whole `:match`, so this one goes with the grammar.

## The estimate, re-taken

| | the stage | re-taken | why |
|---|---:|---:|---|
| the census (T1) | 1 | 1 | done |
| the grammar on paper and the real programs (T2) | 1 | 1 | done, with the builder that makes today's AST, which T3 had |
| the generated parser beside the old one, and the swap (T3, T4) | 2 | 2 | the builder is written and holds on 331 files, `Loc` for `Loc` (−0.5); the swap moves 31 `.expected` files, the 78-case `grammar_decisions.txt`, the 53 460-parse keyword test, which tests the rule the name rule overrides, and six fixtures' digests (+0.5) |
| recovery, deterministic (F11) | 1 | 1.25 | the probe's patch cannot yet tell a cut from *no repair*, and the printer's *three repairs, then a count* and the two hint sentences are new |
| the grammar-pages test, the loader's refusals, the error text (T5) | 1 | 2 | 67 refusal sites in seven files, each with its test, against the 46 lines and 19 `Found::` sites the stage counted in one; and one loader line to add, a connective left as a pattern's argument once macros have expanded |
| the nesting limit and the tests that move | 1 | 1 | |
| `--diff-models` (S-15, T6) | 0.5 | 0.5 | |
| the pages (T7) | 1 | 1.25 | `06_reserved_names.md` also moves: nine words become grammar tokens that stay names |
| **total** | **8.5** | **10** | |

## Re-taking it

From the repository root, with the snapshot and the two binaries in `build/s1f.5.5/` as
[`s1f.5.5_paper_grammar.md` § Re-taking it](s1f.5.5_paper_grammar.md#re-taking-it) builds them:

```sh
C=plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/census
python3 $C/positions.py build/s1f.5.5/snapshot > positions.txt          # what the corpus writes
python3 $C/matrix.py matrix.json && python3 $C/matrix_md.py matrix.json > matrix.md   # 723 probes, 6 s
build/s1f.5.5/c_ein corpus build/s1f.5.5/snapshot                      # both parsers, every file: the 55
build/s1f.5.5/c_ein mutate build/s1f.5.5/snapshot 40000                # mutants, one core
(cd build/s1f.5.5/snapshot && find examples tests stdlib corpus -name '*.ein' | sort) \
  | taskset -c 24-31 xargs -P 6 -n 1 $C/sweep.sh                       # solve and saturate, both parsers, 37 s
python3 $C/refusals.py ein.rs/crates/ein-ir/src/from_ir.rs ein.rs/crates/ein-infer/src/compile.rs
```

The draft's 34: the probe's `harness corpus`, built from the snapshot's own copy of the probe, whose
corpus root is the snapshot.
