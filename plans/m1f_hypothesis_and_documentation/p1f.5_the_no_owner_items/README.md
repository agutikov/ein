# P1f.5 — The no-owner items, one plan per decision; then NL and `:priority`

**Milestone:** [M1f](../README.md)
**Estimate:** ~8 weeks — 18 stages, **40.25 days**: S1f.5.0, which files the known wrong answers as register
rows first, 1 day; [S1f.5.17](s1f.5.17_ci_green_again.md), which makes the CI green again on Ubuntu 26.04's
Graphviz and re-banks the mutant baseline, 1 day (added 2026-11-08); the thirteen no-owner stages, 26.75 days,
S1f.5.5 re-taken at its step 1 from 8.5 days to 10; [S1f.5.18](s1f.5.18_an_activator_binds_a_nested_fact.md),
W33 and G-14's load-time check, 1.5 days (added 2026-11-08, on Q-M1f.1 and Q-M1f.3); then the two stages P1f.5 already had, 10 days ([S1f.5.14](s1f.5.14_nl_required.md) 4 d,
[S1f.5.15](s1f.5.15_rule_priority.md) 6 d). One more, [S1f.5.16](s1f.5.16_the_read_out_reference.md), is specified since 2026-11-08 (*"Ok, schedule
S1f.5.16 also"*), with no estimate. **It was 40
days until the user's re-plan of 2026-11-08** ([§ The order](#the-order)): S1f.5.5 took the grammar parts of
S1f.5.4, S1f.5.1, S1f.5.2 and S1f.5.11 and went from 10 days to 12.75, and the four went down by 2.5 days
together (S1f.5.4 1.5 → 1.25, S1f.5.1 2 → 1.25, S1f.5.2 2.5 → 1.5, S1f.5.11 1.5 → 1), a net quarter of a day.
**Status:** created in M1e2 as P1e2.5 on 2026-11-06, on the user's instruction: *"do not apply
modifications now, create new phase in m1e2 where put stages with plans to fix
every decided item; I will continue review and you will write down plans when
after my decisions"*. The review finished on 2026-11-08 with every item decided. **Moved to M1f the same
day**, on the user's instruction: *"I want to move P1e2.5 -> P1f.5 adding 2 stages of existing p1f.5 in
the end of phase"*. **Running since 2026-11-08**, on the user's go-ahead of that day (*"run
plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items, each stage in separate subagent, …, ask if
decision required"*): S1f.5.0, S1f.5.17 and S1f.5.3 are done, and so are S1f.5.5's steps 1 and 2 and
S1f.5.4's step 1. **Re-planned the same day on the user's ruling**: the generated parser first
([§ The order](#the-order)), and S1f.5.5's T3, the generated parser beside the old one, is done the same day.
**Four more rulings the same day** ([§ The order](#the-order)): the generated grammar is the phase's final one
from the start, so S1f.5.15 runs after T3 and before the swap and the grammar never has `:priority`; `:why` is
optional in the grammar and a loader flag makes it required, which S1f.5.14, now last, turns on; and the vendored
`lrpar` patch is not sent upstream ([F11](#f11-grmtools-recovery-is-not-deterministic)). S1f.5.15's T1 ran the
same day and stopped before any removal, on [Q-M1f.4](../open_questions.md#q-m1f4--removing-priority-moves-the-facts-of-cores-on-seven-corpus-programs)
and [Q-M1f.5](../open_questions.md#q-m1f5--the-alternatives-flag-moves-refuted-by-beside-an-unmoved-core).
On a fifth, *"yes, land the :why flag early"*, S1f.5.14's flag, `--require-why`, landed the same day
([T1f.5.14.6](s1f.5.14_nl_required.md#t1f5146-record-2026-11-08)); the stage keeps only the flip, last.
**Was P1e2.5**, in [M1e2](../../m1e2_correctness/README.md), until 2026-11-08. No stage had run. Each kept
its number, `S1e2.5.<n>` → `S1f.5.<n>`, and the two stages P1f.5 had follow them
([§ The ids](#the-ids)). The questions the review raised keep their ids, `Q-M1e2.22` to `.28`, as M1f's
rule is: *the question ids stay where they were raised*.
**Depends on:** the user's review of
[the no-owner inventory](../../m1e2_correctness/inventory_2026-11-06_no_owner.md), done 2026-11-08, whose
evidence is in `build/review/` (gitignored; rebuilt by the driver kept beside it). S1f.5.15 depends hard on
[S1e.1.3](../../../docs/history/m1e_review_processing/README.md#s1e13--what-the-core-promises-q2), done
2026-08-29.
**Blocks:** nothing. It no longer blocks M1e2's close: each item now has an owner, this phase.
[P1f.20](../p1f.20_docs_refactor/README.md) runs after it, since several stages here edit kernel pages that P1f.20 moves.

---

## How this phase works

1. The user reads an item and decides.
2. A ruling on the language or on an answer is recorded in
   [`open_questions.md`](../../m1e2_correctness/open_questions.md), with the user's words.
3. The agent writes the item's plan here as a stage file, and changes no engine
   code, test, golden or fixture to do it. A prototype, a sweep or a banked
   witness is a stage's step 1, and waits with the rest.
4. When the review is done, or sooner if the user asks, the user says which
   stages run and in what order.

A stage is written so that it can run later without this conversation: each one
states its case, the ruling verbatim, what it changes, its defaults, and what
has to be shown to the user before an edit.

## The decisions

The 13 items are the [no-owner inventory](../../m1e2_correctness/inventory_2026-11-06_no_owner.md)'s.
The two findings below them came out of the review itself.

| item | what it is | decision | plan |
|---|---|---|---|
| **W-22** | `_` as a fact's argument is dropped at load | **ruled 2026-11-06**: a bare `_` is reserved and refused, and `_` is valid anywhere in a name ([Q-M1e2.22](../../m1e2_correctness/open_questions.md#q-m1e222--does-ein-need-a-wildcard)) | [S1f.5.1](s1f.5.1_bare_underscore_refused.md) |
| **S-15** | `--diff-models` compares facts as text | **ruled 2026-11-07, as recommended**: the file's facts are read through the parser and compared by value; S1f.5.2 and S1f.5.4 take the value cases, and S1f.5.5 the rest: spacing, a trailing comment, and a fact that does not parse | [S1f.5.5](s1f.5.5_the_grammar.md#task-t1f556----diff-models-compares-by-value-s-15) |
| **S-16** | past the limit, two line questions say less than describe | **ruled 2026-11-07, (b)**: where ∅ is in R the minimal line is {∅}, and every minimal question says so after one saturation | [S1f.5.6](s1f.5.6_the_empty_state_decides_the_minimal_line.md) |
| **S-17, F1** | the minimal-line row's *no obligation is stated*; one fix for both | **ruled 2026-11-07, (a)**: the clause goes, the row reads `1 state — ∅`, and `minimal.trivial` reads *∅ is in R* | [S1f.5.6](s1f.5.6_the_empty_state_decides_the_minimal_line.md#s-17-and-f1-the-clause-goes) |
| **S-18** | a guard as a query's goal | **ruled 2026-11-07** ([Q-M1e2.26](../../m1e2_correctness/open_questions.md#q-m1e226--are-eq-and-neq-relations)): `eq` and `neq` are relations, and a query over them is answered, a free variable over the program's names | [S1f.5.7](s1f.5.7_eq_and_neq_are_relations.md) |
| **D-14** | an owing hole written `core: []` | **ruled 2026-11-07** ([Q-M1e2.27](../../m1e2_correctness/open_questions.md#q-m1e227--what-do--and-null-mean-in-eins-json)): `[]` is the empty set, `null` an object that does not exist, so a node that is not dead has `core: null`, on every JSON surface | [S1f.5.8](s1f.5.8_no_contradiction_no_core.md) |
| **D-15** | `02_patterns.md` and `:where` | **ruled 2026-11-07** ([Q-M1e2.28](../../m1e2_correctness/open_questions.md#q-m1e228--which-keywords-does-each-form-take)): each form takes a fixed set of keywords and any other is an error, so `:where` is refused wherever written; six kernel pages lose it, and G-15 is settled with it | [S1f.5.11](s1f.5.11_closed_keywords.md) |
| **G-21** | may a rule conclude `neq`? | **ruled 2026-11-07**: yes, with no restriction; a true conclusion adds nothing, a false one is a contradiction, and none is stored | [S1f.5.7](s1f.5.7_eq_and_neq_are_relations.md#conclusions-g-21) |
| **G-22** | a keyword pair beside an `:assert`'s conjuncts | **ruled 2026-11-07**, with D-15: *"if parser sees :k or :G :notes or whatever else unknown kwarg - it is an error"*; a term takes no keyword | [S1f.5.11](s1f.5.11_closed_keywords.md) |
| **P-14** | past the limit, work the question does not need | **closed by S-16's ruling**: the same rule stops each minimal question at root | [S1f.5.6](s1f.5.6_the_empty_state_decides_the_minimal_line.md) |
| **P-15** | N's cores past the limit: keep them, or take none | **ruled 2026-11-08, (c)**: no core during the walk; after it, a core for each member of N, which `nogoods.json` and `--trace` print; `not-kept` retires | [S1f.5.12](s1f.5.12_cores_for_n_only.md) |
| **P-16** | four dumps in one test | **ruled 2026-11-08**: *"move to slow tier"*; a manifest field by state-file count, held both ways, and a nightly step, since the test's slow tier runs in no CI tier today | [S1f.5.13](s1f.5.13_large_dumps_to_the_slow_tier.md) |
| **F2** | **new: a token needs no separator, so a word can split in silence** (below) | **ruled 2026-11-07**: a token runs to a separator, and a name is spelled from a list of allowed ASCII characters, forever; `?` and `:` only in front of a variable and a keyword, so `a?x` and `?x?y` are refused | [S1f.5.4](s1f.5.4_digits_are_names.md), [S1f.5.5](s1f.5.5_the_grammar.md#the-name-rule) |
| **F3** | **new: `_` can report a larger core than a variable used once** | closed by W-22's ruling: `_` no longer reaches a rule | [S1f.5.1](s1f.5.1_bare_underscore_refused.md) |
| **F4** | **new: strings as names were never ruled** (below) | **ruled 2026-11-06**: a string is natural language, only in `:why`, `:source` and `:goal-text` ([Q-M1e2.23](../../m1e2_correctness/open_questions.md#q-m1e223--what-is-a-string-for)) | [S1f.5.2](s1f.5.2_strings_are_natural_language.md) |
| G-6 | a variable or a range written in a fact is a name (outside the 13; [the no-owner inventory](../../m1e2_correctness/inventory_2026-11-06_no_owner.md), § 3) | **ruled 2026-11-06**: `?x` in a fact is refused, and the range goes with S1f.5.3 (**done 2026-11-08**, `65d1ad85`) | [S1f.5.2](s1f.5.2_strings_are_natural_language.md), default 1 |
| **F5** | **new: an unknown escape is dropped in silence** (below) | **ruled 2026-11-06**: strings are Unicode, with reasonable escapes, held by a test and shown by two examples | [S1f.5.2](s1f.5.2_strings_are_natural_language.md) |
| **F6** | **new: a range has no user** (below) | **ruled 2026-11-06**: no numbers and no ranges in Ein ([Q-M1e2.24](../../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)), and the token goes; **built 2026-11-08** by S1f.5.3 (`65d1ad85`) | [S1f.5.3](s1f.5.3_range_retired.md) |
| **F7** | **new: digits in a value** (below) | **ruled 2026-11-06**: digits are plain names, and `(p 1)` and `(p 01)` are different facts | [S1f.5.4](s1f.5.4_digits_are_names.md) |
| **F8** | **new: a digit label becomes a candidate** (below) | **ruled 2026-11-07, as recommended**: a label is a candidate like any other name | [S1f.5.4](s1f.5.4_digits_are_names.md) |
| **F9** | **new: the grammar is too permissive** ([Q-M1e2.25](../../m1e2_correctness/open_questions.md#q-m1e225--how-strict-is-the-grammar-and-what-holds-the-parser-to-it)) | **the user's instructions, 2026-11-06 and -07**: revisit it, and generate the parser; **ruled 2026-11-07: grmtools**, for its features | [S1f.5.5](s1f.5.5_the_grammar.md) |
| **F10** | **new: deep nesting aborts** (below) | goes with the grammar: a stated limit | [S1f.5.5](s1f.5.5_the_grammar.md) |
| **F11** | **new: grmtools' recovery is not deterministic** (below) | **ruled 2026-11-07: recovery on**; the stage makes it deterministic with a patch to `lrpar`, vendored | [S1f.5.5](s1f.5.5_the_grammar.md#f11-grmtools-recovery-is-not-deterministic-and-this-stage-makes-it-so) |
| **F12** | **new: an empty `:match` never fires** (below) | **filed and ruled 2026-11-07**: it fires, in rules and hrules alike, and `(and)` stays refused; a conclusion's variable nothing binds is F13 | [S1f.5.9](s1f.5.9_an_empty_match_fires.md) |
| **F13** | **new: a conclusion's variable that nothing binds stops the run** (below) | **filed and ruled 2026-11-07**: refused at load, naming the two explicit spellings | [S1f.5.10](s1f.5.10_a_conclusion_variable_must_be_bound.md) |

### F2: a token needs no separator

Found 2026-11-06 while planning S1f.5.1. The lexer takes the longest token at
the cursor and the next token may start directly after it, so these load with no
line (`ein saturate FILE --dump`, on `(relation r T)` and the fact shown):

| written | stored |
|---|---|
| `(r 1a)` | `(r 1 a)` |
| `(r 1..5a)` | `(r "1..5" a)` |
| `(r "a"b)` | `(r a b)` |
| `(r ?x?y)` | `(r "?x" "?y")` |
| `(r a?x)` | `(r a "?x")` |
| `(r 1_)` | `(r 1)` |
| `(r -1_)` | `(r -1)` |
| `(r _-1)` | `(r -1)` |

Each stores two arguments against a one-argument signature, or loses a `_`. It
is the class of [W22](../../m1e2_correctness/open_questions.md#q-m1e217--where-does-a-keyword-end), a
word read as two tokens, at a place W22's fix does not reach. The last three go
with S1f.5.1. The question for the user was the first five: must a number, a
string or a variable end at a separator, so that each is refused? **Ruled
2026-11-07 with what a name is:** a token runs to a separator, and no corpus
program writes one of these
([S1f.5.5](s1f.5.5_the_grammar.md#the-name-rule)). `1a`, `1_` and `-1_` become
names, and `a?x`, `?x?y` and `"a"b` are refused, on the agent's reading.

### F4: strings as names were never ruled

Found 2026-11-06, reading S-15. The first grammar gave `STRING` to *source-sentence
provenance + `:why` templates only* (`c44ccfb5`, 2026-05-18). The Rust loader held a
fact's string as the symbol it names from its first commit (`687b91bf`, 2026-08-18,
earlier history not traced), and `01_grammar.md` said so only at S1e2.2.16
(2026-11-04). S1e2.2.18 extended it to rules the same day, as a default the stage
says the user may override (`259c16e4`). The agent found no ruling. Four defects
came of it: W16, W18, W28 and S-15. Outside keyword values, 13 files write a string
as a value, 12 of them those defects' witnesses and the other the fixture
`syntax/arg-kinds` (the agent's scan of `examples/`, `tests/` and `stdlib/`,
2026-11-06). It is also the only way to write a name holding a space or a
non-ASCII letter: `(p "Жук")` loads, and bare `Жук` does not.

### F5: an unknown escape is dropped in silence

Found 2026-11-06. `\n`, `\t`, `\r`, `\"` and `\\` are escapes. For any other
character the backslash is dropped with no line (`ein saturate FILE --dump`):
`"a\x41"` stores the name `ax41`, `"\u0416"` stores `u0416`, and `"a\0b"` stores
`a0b`. This is the documented behaviour (`00_ebnf.md`, fact 4: *there is no `\xNN`,
no `\uNNNN`, no octal*), so no page is wrong. Ruled the same day
([Q-M1e2.23](../../m1e2_correctness/open_questions.md#q-m1e223--what-is-a-string-for)): strings are Unicode, with
reasonable escapes.

### F6: a range has no user

Found 2026-11-06, when the user asked what `1..5` is. `RANGE` (`1..5`, `0..1`, `1..*`) is
in the first grammar as *UML-style cardinality*, for `(relation has-color (House Color)
:cardinality 1..1)` (`c44ccfb5`, 2026-05-18). **It was never built.** No `.ein` has written
`:cardinality` in the repository's history (`git log -G':cardinality' -- '*.ein'` is empty,
2026-11-06), and no code reads it, though `01_grammar.md` still advertises it (lines 22 and
91).

What the token does today (`ein saturate FILE --dump`, 2026-11-06):

| written | result |
|---|---|
| `(r 1..5 a)` in a fact | loads, and stores a name printed `"1..5"` |
| `(r a b :card 1..*)`, `(relation r T T :card 1..*)` | loads, and the range is dropped |
| `(relation r T T 1..* 1..1)` | refused: *malformed (relation)* |
| `(n 1..3)` in a rule's `:match` | refused at load since W20, on Q-M1e2.13 (a) |

Outside comments, four programs write one: the fixture `syntax/arg-kinds`
(2026-08-20), W20's witness `range-in-match`, and two probes under M1e2's plans. **Ruled
the same day: the token goes**, which also settles the range half of G-6. The plan is
[S1f.5.3](s1f.5.3_range_retired.md), **done 2026-11-08**: no terminal reads a range, and `1..5` is a parse
error until S1f.5.4 makes it one name. Its step 1 found a fifth program, S1f.5.5's rewrite of `arg-kinds`
under `plans/`, banked after this section was written.

### F7: digits in a value

Found 2026-11-06, on the user's *"we don't have native numbers in Ein"*
([Q-M1e2.24](../../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)). As a matter of meaning
that holds: the only built-in predicates are `eq` and `neq`, and no rule can add, compare or
order two values as quantities. As a matter of the language it is not yet so:

| what exists | where |
|---|---|
| a token, `INT ::= '-'? [0-9]+` | `00_ebnf.md` line 82, since the first grammar (`c44ccfb5`, 2026-05-18) |
| a value kind and a pool, `Tag::Int`, `IntId` | `ein-core/src/value.rs`, and a section of the `.einb` format |
| numeric reading: `(p 1) (p 01)` is one fact, and sorting compares by value | `value.rs`, `Terms::cmp_value` |
| a value kind apart from a name: `(p 1) (p "1")` is two facts | what W16 and W28 fixed |

Where programs write one (the agent's scan of `examples/`, `stdlib/`, `tests/` and `corpus/`,
2026-11-06):

| use | count | what it is |
|---|---:|---|
| `:priority 110` | 475, in 136 files | a keyword's value, read as a rule's order |
| a plain value | 217, in 34 files | **a label in every case**: `std.elim`'s argument positions, `(functional R 0 1)`, and bug witnesses such as `(edge 1 2)`, none of which does arithmetic |

The stdlib's use is real: `std.elim` reads `(functional ?R 0 1)`, and `features/05`,
`branching/12` and `tests/stdlib/elim/01` and `/02` write it. `(p 1st)` also splits into
`(p 1 st)` (F2).

**Ruled the same day:** *"YES, digits are not numbers, digits are plain names, and refuse ?x in
facts; (p 1) and (p 01) are VERY DIFFERENT facts"*. The plan is
[S1f.5.4](s1f.5.4_digits_are_names.md).

### F8: a digit label becomes a candidate

Found 2026-11-06, measuring F7. A digit is no candidate today. A name is one (Q-M1e2.9 (a)), so
under the blind enumerator a label such as `std.elim`'s `(functional R 0 1)` joins H. On a
simulation over every program that writes a digit or imports `std.elim` (each digit renamed in a
throwaway copy, on today's binary), `features/05` keeps k_s 32 and k_c 32 with N 21 → 47, and
`branching/12` keeps k_s 3 and k_c 7 with N 25 → 51. R is unmoved on both, and the extra members of
N are dead sets over the new candidates. The question is whether that is acceptable or a label
should stay out of H. The agent recommends accepting it: it is what the ruling on names says, a
signature that names its types keeps a label out, and an exemption would be a second kind of name
again. **Ruled 2026-11-07, as recommended**, with what a name is:
[S1f.5.5](s1f.5.5_the_grammar.md#the-name-rule).

### F10: deep nesting aborts

Found 2026-11-07, comparing parsers. `ein solve` on a fact nested 100 000 deep exits 134 with *has overflowed
its stack*, and no line says why; 10 000 deep exits 0. The corpus's deepest program nests 8. An abort is not a
refusal. A generated LR parser survives any depth, but macro expansion, the loader and the dumper recurse, so
[S1f.5.5](s1f.5.5_the_grammar.md) states a limit.

### F11: grmtools' recovery is not deterministic

Found 2026-11-07, while the generator was being chosen. grmtools' error recovery, CPCT+, can report differently
on two runs of one file: `lrpar` 0.15 ranks equal-length repairs in a `HashSet`'s order, and stops recovering
at a 500 ms wall-clock budget. Over 20 runs on the probe's `four_mistakes.ein` it printed two outputs. The
first error is unaffected, since no recovery code runs before it. **The user ruled the same day that recovery
is on**, so [S1f.5.5](s1f.5.5_the_grammar.md#f11-grmtools-recovery-is-not-deterministic-and-this-stage-makes-it-so) vendors `lrpar` with a
patch: a stable order for equal repairs, and a step budget in place of the clock. On the probe it gave one
report on each of 5 000 corpus mutants over three runs, where the unpatched crate differed on 1 032. **The patch
is not sent upstream**: the user, 2026-11-08, *"don't send the lrpar patch upstream either"*, as on the Graphviz
bug ([S1f.5.17 § Notes](s1f.5.17_ci_green_again.md#notes)). It stays vendored, `ein.rs/vendor/lrpar/ein.patch`
(T3, `54715b5f`).

### F12: an empty `:match` never fires

Found 2026-11-07, while checking whether a core can be the empty set for D-14. A rule whose `:match` is empty
loads and never fires. An empty conjunction is true, so logically the rule fires once, or once per
activator, and concludes what it asserts. No corpus program writes one (`grep` over `examples/`, `tests/`,
`stdlib/` and `corpus/`). Taken at `916d97ac`, with `ein saturate FILE --dump` and `ein solve FILE`, over
`(relation q Node) (q a)`:

| the rule | today | logically |
|---|---|---|
| `(rule r () :match () :assert (false))` | loads, never fires: ∅ in R | root is dead, so R is empty |
| `(rule r () :match () :assert (q b))` | loads, never fires: no `(q b)` | root holds `(q b)` |
| `(rule r (?x) :match () :assert (hit ?x))` with `(r a)` | loads, never fires: no `(hit a)` | root holds `(hit a)`, one firing per activator |
| `(rule r () :match (and) :assert (false))` | refused at parse: *unexpected input* | `(and)` is the empty conjunction, the same as `()` |
| `(rule r () :match (q ?x) :assert ())` | loads, and stores a fact named `@empty`, which joins H: R two states (S1f.5.5's [census](s1f.5.5_census.md#findings-outside-the-grammar), 2026-11-08; this row read *adds nothing* until then) | an empty conclusion asserts nothing: W34, filed by S1f.5.0 and owned by [S1f.5.9](s1f.5.9_an_empty_match_fires.md) |

The first three are wrong answers of the kind W24 refused: a shape that reads nothing, in silence. **The
user's instruction** (*"file new F12 into p1e2.5"*) files it here, and a ruling decides between answering
it, so that an empty `:match` fires, and refusing it at load. The user's *"logical correctness first"*
(Q-M1e2.26) points to the first.

**Ruled the same day:** *"1 fire; 2 same; 3 reject (and) while it is the same as () and `and` should have
arguments"*. An empty `:match` fires, an hrule's too, and `(and)` stays refused, `()` being the one spelling
of the empty conjunction. The plan is [S1f.5.9](s1f.5.9_an_empty_match_fires.md). A fourth question, a
conclusion's variable that nothing binds (`(rule r () :match (p ?x) :assert (q ?y))` stops the run at its
first firing with `unbound var ?y in :assert — bindings: {'x': 'a'}`), is filed as F13.

### F13: a conclusion's variable that nothing binds

Found 2026-11-07, while asking F12's fourth question. `(rule r () :match (p ?x) :assert (q ?y))` loads, and the
run stops at the first node that fires it, exit 1, with `unbound var ?y in :assert — bindings: {'x': 'a'}`, the
Python implementation's spelling. `examples/ein-bugs/unbound-assert-var.ein`, the fuzzer's find of
2026-08-20, reaches it through an `(or …)` arm. The user asked what the shape is worth. The agent: almost
nothing, since "every name" holds relation and type names, and the meaning stays writable with a typed premise
or, after S1f.5.7, `(eq ?y ?y)`. **Ruled the same day:** *"yes, file F13 with the refusal"*. The plan is
[S1f.5.10](s1f.5.10_a_conclusion_variable_must_be_bound.md).

## Stages

| ID | title | est. | item | state |
|---|---|---:|---|---|
| [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md) | The known wrong answers filed: W29 to W32, before any stage moves them | 1 d | the twenty rows of S1f.5.1, .7, .9 and .11, and W33 to W35 | **done 2026-11-08**: the twenty rows (`0525444e`, `da58dda6`, `3332246e`) and W33 to W35 from S1f.5.5's census (T1f.5.0.4), twenty-three rows each `open` and owned by its stage: the register at 103 fixed, **33 open** and 1 not-wrong (`grep '^status' corpus/wrong_answers.toml \| sort \| uniq -c`, 2026-11-08); one right, `W30/assert-neq-w-x`'s, put to the user (the user, 2026-11-08: *"plan adding rows in p1f.5"*) |
| [S1f.5.1](s1f.5.1_bare_underscore_refused.md) | A bare `_` is reserved and refused, and `_` is valid anywhere in a name | 1.25 d, re-planned 2026-11-08 (was 2) | W-22, F3 | specified, not started; **re-planned 2026-11-08**: its step 1 is S1f.5.5's attribution sweep and review stop (T1f.5.5.9), its refusal, register rows (W29's six) and fixtures land at S1f.5.5's swap (T1f.5.5.4), credited to this stage; after the swap, F3 traced, `_` held in every position, `Slot::Any` and `Node::Wildcard` retired, and the pages |
| [S1f.5.2](s1f.5.2_strings_are_natural_language.md) | A string is natural language: only in `:why`, `:source` and `:goal-text` | 1.5 d, re-planned 2026-11-08 (was 2.5) | F4, F5, G-6 | specified, not started; **re-planned 2026-11-08**: its step 1 is S1f.5.5's T1f.5.5.9, its refusal, escapes and restated rows (W16, W18, W28) land at the swap, credited to this stage; after the swap, the string's value arms retired, a string held in every position, the Unicode test and two examples, and the pages |
| [S1f.5.3](s1f.5.3_range_retired.md) | No ranges: the `RANGE` token retired | 1 d | F6, G-6's range half | **done 2026-11-08** (`b1ac2178`, `65d1ad85`, T1f.5.3.3): no terminal reads a range, `Node::Range` and `Terms::value_range` are gone, and `1..5` is a parse error until S1f.5.4 makes it one name; five programs moved, four the case's and S1f.5.5's `arg-kinds` rewrite under `plans/`. The two corpus files that write one, `syntax/arg-kinds` and `range-in-match`, keep their programs and are named in `ein_corpus::REFUSED_UNTIL_S1F_5_4`, held both ways, which S1f.5.5's swap empties since the re-plan of 2026-11-08 (S1f.5.4's T2 until then); `W20/range-in-match` stays fixed, its `now` the parser's line; `a_range_is_no_token` holds each place a range was written. Five goldens moved, named at step 1 |
| [S1f.5.4](s1f.5.4_digits_are_names.md) | Digits are plain names, and a name is any run of name characters | 1.25 d, re-planned 2026-11-08 (was 1.5) | F7, F8, F2 | **step 1 done 2026-11-08** (T1f.5.4.1, `8b12ae07`, `638b1062`): 46 of 511 programs move on the prototype, each by the ruling, and H moved on one program with no plain digit, which is the finding (`@empty` and `<nested>`, the loader's synthetic names, become spellable); its four questions **decided by the coordinator the same day** as the step recommended, open to the user's review at S1f.5.5's stop ([§ Step 1](s1f.5.4_digits_are_names.md#step-1--every-move-listed-2026-11-08)). **Re-planned 2026-11-08**: the name rule, the register's 12 rows and the fixtures land at S1f.5.5's swap, credited to this stage, and the 46 are the attribution's first cross-check; after the swap, the int kind retired, and the pages. Its `:priority` guard has nothing to guard once S1f.5.15 has removed the keyword before the swap (the user's ruling of 2026-11-08), unless S1f.5.15 ends in a written refusal. **Owns `W37/template-var-star` since 2026-11-08** (S1f.5.5's T9 review, item 7, filed by its T10): a template reference reads the name rule's characters, [T1f.5.4.5](s1f.5.4_digits_are_names.md#task-t1f545--a-template-names-every-variable-w37), after the swap |
| [S1f.5.5](s1f.5.5_the_grammar.md) | The grammar revisited: each position says what it holds, and the parser is generated from it | 12.75 d, re-planned 2026-11-08 (10 at its step 1, 8.5 before) | F9, F10, F11, F2, S-15, G-14, W35, and since 2026-11-08 the grammar parts of S1f.5.4, .1, .2 and .11 | **steps 1 and 2 done** (`49e9fed6`); the paper grammar shown and stands, [Q-M1f.1](../open_questions.md#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused) (b) and [Q-M1f.2](../open_questions.md#q-m1f2--the-55-files-the-grammar-refuses-that-todays-parser-accepts) (a) ruled 2026-11-08. **T3 done 2026-11-08** ([§ T3 record](s1f.5.5_the_grammar.md#t3-record-2026-11-08)): the generated parser in `ein-ir` beside the hand-written one, behind the `generated-parser` cargo feature, off in every shipped build; the builder holds on 340 of 414 corpus files with `:priority` taken out (204 as written, 133 differing only by it), 0 trees differing; the grammar has no `:priority` (S1f.5.15 before the swap) and a macro takes a `:why`, the user's rulings of 2026-11-08; `lrpar` 0.15.0 vendored and patched, recovery deterministic at a budget of 30 000 steps, chosen from 40 000 corpus mutants; nesting limit 1 000; no answer, golden or exit code moved. **Re-planned 2026-11-08**: T3 (the generated parser beside the old one) ran first and waited for no other stage, then S1f.5.15, which removes `:priority` (the user's ruling of 2026-11-08: *"S1f.5.15 before swap"*), then T8 (the loader's names out of the name space; **done 2026-11-08**, [§ T8 record](s1f.5.5_the_grammar.md#t8-record-2026-11-08)), T9 (every difference attributed to a ruling, one review stop; **done 2026-11-08**, [§ T9 record](s1f.5.5_the_grammar.md#t9-record-2026-11-08), [the attribution](s1f.5.5_attribution.md): every one of the 167 files the swap moves credited to a ruling, the census's 55 refused, 45 of S1f.5.4's 46, the 46th Q1's witness; **stopped for the user's review**, ruled the same day, [§ The review stop ruled](s1f.5.5_the_grammar.md#the-review-stop-ruled-2026-11-08)), T10 (the fixture moves that move no answer; **done 2026-11-08**, [§ T10 record](s1f.5.5_the_grammar.md#t10-record-2026-11-08): W37 filed, `open` and S1f.5.4's, and `syntax/wrappers` moved, the other two candidates T4's), T11 (the engine's five names reserved, the user's ruling; stopped 2026-11-08 before any edit on [Q-M1f.7](../open_questions.md#q-m1f7--where-may-a-program-write-the-kernels-dunders), which the user ruled (b), *"Never bound"*; **done the same day**, [§ T11 record](s1f.5.5_the_grammar.md#t11-record-2026-11-08)) and T4, the swap; T5 to T7 after it |
| [S1f.5.6](s1f.5.6_the_empty_state_decides_the_minimal_line.md) | Where ∅ is in R, every minimal-line question stops at root, and says only ∅ | 1.5 d | S-16, P-14, S-17, F1 | specified, not started; independent of the grammar stages, and runs after S1f.5.5's swap ([§ The order](#the-order)) |
| [S1f.5.7](s1f.5.7_eq_and_neq_are_relations.md) | `eq` and `neq` are relations: in a goal, a premise and a conclusion | 3 d | S-18, G-21; restates W24's guard rows | specified, not started; restates two W24 rows |
| [S1f.5.8](s1f.5.8_no_contradiction_no_core.md) | A node with no contradiction has no core: `null`, not `[]` | 0.5 d | D-14 | specified, not started |
| [S1f.5.9](s1f.5.9_an_empty_match_fires.md) | An empty `:match` fires | 1 d | F12 | specified, not started |
| [S1f.5.10](s1f.5.10_a_conclusion_variable_must_be_bound.md) | A conclusion's variable must be bound, and is refused at load where it is not | 0.5 d | F13; G-14's unbound part | specified, not started; after S1f.5.7, before S1f.5.9 |
| [S1f.5.11](s1f.5.11_closed_keywords.md) | Each form takes a fixed set of keywords, and `:where` is gone | 1 d, re-planned 2026-11-08 (was 1.5) | D-15, G-15, G-22 | specified, not started; **re-planned 2026-11-08**: the lists are the grammar's, and its step 1, its refusals and W32's seven rows land in S1f.5.5's T9 and swap, credited to this stage; after the swap, the compiler's and the expander's drops go, a keyword held in every form and place, and the pages |
| [S1f.5.12](s1f.5.12_cores_for_n_only.md) | A run takes a core only for the nodes a surface prints: N's members, after the walk | 1 d | P-15 | specified, not started; independent of the grammar stages, and runs after S1f.5.5's swap ([§ The order](#the-order)) |
| [S1f.5.13](s1f.5.13_large_dumps_to_the_slow_tier.md) | The round trip's four large dumps move to the slow tier, and the nightly runs it | 0.5 d | P-16 | specified, not started; independent, and runs after S1f.5.5's swap ([§ The order](#the-order)) |
| [S1f.5.14](s1f.5.14_nl_required.md) | Every statement convertible to NL | 4 d | was S1f.5.5 | **T1f.5.14.6, the flag, done 2026-11-08, early** ([its record](s1f.5.14_nl_required.md#t1f5146-record-2026-11-08); the user: *"yes, land the :why flag early"*): `--require-why` on the seven commands that load a program, default false, refuses a relation, rule or hrule with no `:why`, and a macro once S1f.5.5's swap deletes its `cfg`; 274 of the corpus's 339 loadable files are refused with it on (2026-11-08). The rest **last**, after every other stage (the user's ruling of 2026-11-08): `:why` optional in the grammar, which T3 gave a macro too (`54715b5f`), and the flag turned to true as the stage's last task, once every program carries one; rung (d), *a load error*, reached through the flag |
| [S1f.5.15](s1f.5.15_rule_priority.md) | Remove `:priority`; derive the order from the rules | 6 d | was S1f.5.6 | **T1 done 2026-11-08, and stopped there, before any removal** ([§ T1 record](s1f.5.15_rule_priority.md#t1-record-2026-11-08)): every `:priority` taken out of the corpus and the stdlib moves no R, N, H, claim, exit code or stored fact on the 511 programs or the 1 378 declared runs, and moves the facts of cores on seven corpus programs (`zebra.ein`, the zebra2 family, W25's control), each through the alternatives cap, `:record-alternative-justifications false` or W25; and the stripped corpus shows a config-principle breach that exists today, `features/12`'s `refuted by` under the alternatives flag. [Q-M1f.4](../open_questions.md#q-m1f4--removing-priority-moves-the-facts-of-cores-on-seven-corpus-programs) and [Q-M1f.5](../open_questions.md#q-m1f5--the-alternatives-flag-moves-refuted-by-beside-an-unmoved-core) were put to the user, who **ruled both the same day** (`3998a1c1`): (b), derive the order first, and (a), W36, which T1f.5.15.7 filed, `open` and owned by F22, under the register's schema `/14`. **T2 and T3 done the same day, and the stage stopped again before T4** ([§ T3 record](s1f.5.15_rule_priority.md#t3-record-2026-11-08)): the graph, the strata and a band per rule from its shape ship in `ein_infer::strata` (275 of 471 declared rules derive the band their priority falls in, 62 of the stdlib's 77), the band orders the agenda in a `derived-order` build only, and the re-taken sweep moves the facts of cores on the programs T1 found and on W29's witness; the derived agenda holds every core of the zebra family and the FIFO boundary moves them, which is [Q-M1f.6](../open_questions.md#q-m1f6--the-derived-agenda-holds-the-cores-the-fifo-boundary-does-not). **Done 2026-11-08** on the user's ruling of Q-M1f.6, *"plain FIFO, core is an explanation, F22 makes it canonical"*: T4 the static non-stratifiability diagnostic (`ccc4e8ca`), T5 the removal, the agenda and the boundary FIFO and every core it moves T1's and F22's (`a3cb1753`), T6 the docs ([§ T5 record](s1f.5.15_rule_priority.md#t5-record-2026-11-08)). **Moved 2026-11-08 to run after S1f.5.5's T3 and before T8**, on the user's ruling (*"S1f.5.15 before swap"*), so the generated grammar never has `:priority`: T3 built it without (`54715b5f`), and the 133 corpus files that differ between the two parsers only by their `:priority` are attributed to this stage, pending. **T1f.5.15.9, 2026-11-08**, on the user's ruling on S1f.5.5's review stop (item 8): `utils/region_synth.py`, `guard_split.py` and `fuzz_engine.py` write no `:priority`, moving no answer ([§ T9 record](s1f.5.15_rule_priority.md#t9-record-2026-11-08)); the 43 records under `plans/` stay as written |
| [S1f.5.17](s1f.5.17_ci_green_again.md) | CI green again: the per-commit job on Ubuntu 26.04's Graphviz, and the nightly's mutant baseline | 1 d | the red per-commit and nightly runs since `916d97ac` | **done 2026-11-08** (`3e6e40f8`, `76ede241`, T1f.5.17.3): the per-commit red was the runner's **amd64v3** build of Graphviz 14.1.2, whose `libpathplan` contracts `ccw` into an FMA; fixed view-side, `ranksep=0.55` on the overlay and Levi digraphs (`corpus_shapes.md5` 1 198 lines). The nightly's two mutants were killed by `597b2b7e`, *forbid*'s refusal, and re-banked: 167 of 217 killed. Both tiers green in a container built like the runner, `utils/ci_container.sh` (the user, 2026-11-08: *"add new stage to this phase - fix failing CI jobs"*) |
| [S1f.5.18](s1f.5.18_an_activator_binds_a_nested_fact.md) | An activator's nested fact: a value parameter binds it, and a relation parameter is refused at load | 1.5 d | W33; G-14's derived activator ([Q-M1f.1](../open_questions.md#q-m1f1--g-14-is-a-bad-conclusion-that-never-fires-refused) (b)) | specified, not started; after S1f.5.7 and S1f.5.10, before S1f.5.9 |
| [S1f.5.16](s1f.5.16_the_read_out_reference.md) | The read-out reference: every row and number `ein solve` prints, on one live page the printer is held to | — | was S1f.5.3; [SE-M1](../../../docs/history/m1e_review_processing/README.md#the-findings), [AR-M2](../../../docs/history/m1e_review_processing/README.md#the-findings) and AR-M1, and [Q-M1e.7](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)'s read-out half | specified 2026-11-08 on the user's instruction (*"Ok, schedule S1f.5.16 also"*), not started. It runs after S1f.5.5's swap and its non-grammar halves, and before S1f.5.7 ([§ The order](#the-order), item 9), so that every later stage that changes the read-out writes the live page. The page is `docs/kernel/inference/read_out.md`: each row and number, what it counts (R, what lies around R, the run, or the listing), and the summary field that holds it, together with `--json-summary`'s seven answer keys. The read-out's golden page is generated beside it, and `ein-cli/tests/read_out_reference.rs` holds both to the printer. M1e2's [`read_out.md`](../../m1e2_correctness/p1e2.1a_the_solution_region/read_out.md) and `read_out_examples.md` stay the record, with one line each. [P1f.20](../p1f.20_docs_refactor/README.md)'s S1f.20.2 places the two pages. One question, Q-M1e.7's read-out half, has the default (a): document it |

### The order

**Re-planned by the user on 2026-11-08: the generated parser first.** Until then S1f.5.3, S1f.5.4, S1f.5.1 and
S1f.5.2 were to land one by one on the hand-written parser, with S1f.5.11's closed keywords beside them, and
S1f.5.5's generated parser was to follow in a swap that moved no answer. The coordinator told the user that the
generated parser waited for four language stages built on today's parser. The user: *"It is the same work done
twice, why not switch to grmtools first and remove hand-written parser and then verify requirement met (strings,
names, etc...) ?"* The coordinator proposed the variant below, and the user: *"yes, re-plan it that way"*.

**From then on nothing edits the hand-written lexer or parser for S1f.5.4, S1f.5.1, S1f.5.2 or S1f.5.11.** No
stage id is renumbered: each of the four keeps its id and is cut to its non-grammar part, and its grammar part,
its step-1 sweep and its review are [S1f.5.5](s1f.5.5_the_grammar.md)'s tasks, credited to it by name
([S1f.5.5 § The order](s1f.5.5_the_grammar.md#the-order-re-planned-2026-11-08)).

**The grammar is final from the start: the user's rulings of 2026-11-08, after the re-plan.** The user: *"collect
all requirement for the parser in this phase and implement grammar that will already meet all of them instead of
making wrong parser that meets our old wrong parser and then updating it step by step"*. The coordinator checked
[the paper grammar](s1f.5.5_paper_grammar.md#the-grammar-by-position) against every stage of the phase. It already
meets S1f.5.1, .2, .3, .4 and .11, S1f.5.7 (`eq` and `neq` in a goal, a premise and a conclusion, since
`Conclusion` takes a `Guard`) and S1f.5.9 (`:match ()`, and `(and)` refused). S1f.5.10's and S1f.5.18's checks,
arity, and a claim's closed relations are binding or type checks that no context-free grammar can make, so they
stay load-time checks by design. S1f.5.6, .8, .12 and .13 have no grammar part. Only two items would have edited
the grammar later, and the user ruled both:

- **`:priority`.** The user chose *"S1f.5.15 before swap"*: [S1f.5.15](s1f.5.15_rule_priority.md) runs after
  S1f.5.5's T3 and before T8, T9, T10 and the swap, so the generated grammar never has `:priority`. T3 built it
  without (`54715b5f`): a rule's `:priority` is refused like any keyword a rule does not take, and the 133 corpus
  files that differ between the two parsers only by their `:priority` are attributed to S1f.5.15, pending
  ([S1f.5.5 § T3 record](s1f.5.5_the_grammar.md#t3-record-2026-11-08)). S1f.5.15's acceptance allows a written
  refusal; if it ends in one, the production comes back then, before the swap.
- **NL `:why`.** The user: *":why is optional in grammar and loader has a flag that makes it required, start with
  default false so not break everything, then after other stages finished swith to default true and fix remaining
  ein code"*. Asked whether the flag can change an answer: *"yes, the flag only decides loading"*. It decides
  whether a program loads, never what a loaded program answers, so it does not breach the rule that config never
  changes the answer. T3 gave `(macro …)` an optional `:why` (`ein_ir::macros::Macro::why`), so every form that
  can carry a template may. The flag, default false, is [S1f.5.14](s1f.5.14_nl_required.md)'s first code task,
  and S1f.5.14 runs last: it turns the default to true and writes `:why` wherever the corpus, the stdlib and the
  Rust suites' inline programs still lack it. **The flag landed early, on 2026-11-08** (the user: *"yes, land the
  :why flag early"*; [T1f.5.14.6's record](s1f.5.14_nl_required.md#t1f5146-record-2026-11-08)): `--require-why`,
  which asks a macro for its `:why` only in a `generated-parser` build until S1f.5.5's swap deletes that `cfg` and
  writes the stdlib's two macros theirs.

A fourth ruling of the same day is not the grammar's: the vendored `lrpar` patch stays in the tree and is not
sent upstream ([F11](#f11-grmtools-recovery-is-not-deterministic)).

The order, as those rulings leave it:

1. **Done:** S1f.5.0, so that every known wrong answer is a register row before a stage moves it; S1f.5.17, so
   that every later stage boundary is held to a green CI as well as a green gate; S1f.5.5's census and paper
   grammar; S1f.5.3, the range token retired on the hand-written parser, the one language stage that ran
   before the re-plan; and S1f.5.4's step 1 (46 of 511 programs move on its prototype), which stands.
2. **S1f.5.5's T3, done 2026-11-08** ([its record](s1f.5.5_the_grammar.md#t3-record-2026-11-08)): the generated parser beside the old one, with the full paper grammar, the patched `lrpar`
   that makes recovery deterministic (F11) and the nesting limit (F10). Nothing in the shipped path moves.
3. **S1f.5.15, before the swap** (the user's ruling of 2026-11-08): `:priority` removed from every program the
   two parsers read, and the order derived from the rule graph. **Its T1 stopped on 2026-11-08 before any
   removal**, on Q-M1f.4 and Q-M1f.5 (the stage's [§ T1 record](s1f.5.15_rule_priority.md#t1-record-2026-11-08)),
   which the user ruled the same day; **its T3 stopped again before any removal**, on Q-M1f.6 (the stage's [§ T3
   record](s1f.5.15_rule_priority.md#t3-record-2026-11-08)). **Done 2026-11-08** on the user's ruling of Q-M1f.6,
   plain FIFO: no program writes `:priority`, so S1f.5.5's T9 has no `:priority` difference to attribute
   ([§ T5 record](s1f.5.15_rule_priority.md#t5-record-2026-11-08)).
   What the hand-written parser does with a `:priority` between this stage and the swap was S1f.5.15's to
   decide: the coordinator decided on 2026-11-08, on the user's ruling against double work, that no loader line
   refuses it, since S1f.5.5's T5 would delete one; until the swap it is a rule keyword the loader drops (D-15),
   and a test holds that no corpus program writes it.
4. **T8, the swap's prerequisite, done 2026-11-08:** the loader's four internal names renamed out of the name
   space (S1f.5.4's Q1, decided 2026-11-08), with the generated parser's fifth, `@list`: `#empty`, `#nested`,
   `#params`, `#query` and `#list`, `#` being no name character in either lexer. Held by
   `generated_parser::no_name_the_loader_makes_up_is_a_name` and two twins; no answer moved, only W34's
   candidate's spelling and the renderings that print a renamed head
   ([S1f.5.5 § T8 record](s1f.5.5_the_grammar.md#t8-record-2026-11-08)).
5. **T9, every difference attributed:** both parsers over the corpus and the fuzzer, the old one the reference,
   and each difference credited to S1f.5.4's name rule, S1f.5.1's bare `_`, S1f.5.2's strings, S1f.5.11's
   closed keywords or the paper grammar's own list (step 2's defaults, W35); S1f.5.4's 46 are the first
   cross-check. With S1f.5.15 landed no program writes `:priority`, so no difference is its. **An unattributed
   difference stops the swap**, and the comparison happens while the old parser, the only reference, still
   exists. **One review stop** shows the user real programs for every ruling at once. *Done 2026-11-08*
   ([§ T9 record](s1f.5.5_the_grammar.md#t9-record-2026-11-08)): nothing unattributed; 43 records under `plans/`
   that S1f.5.15 left as written are refused at their `:priority`, credited to it. The user ruled the stop the same
   day ([§ The review stop ruled](s1f.5.5_the_grammar.md#the-review-stop-ruled-2026-11-08)).
6. **T10:** whatever of [Q-M1f.2](../open_questions.md#q-m1f2--the-55-files-the-grammar-refuses-that-todays-parser-accepts)
   (a)'s fixture moves and rewrites can land without moving an answer; with it, T11, the engine's writeback names
   and dunders reserved (the user's ruling of 2026-11-08, *"they are also reserved names"*).
   *T10 done 2026-11-08* ([§ T10 record](s1f.5.5_the_grammar.md#t10-record-2026-11-08)): W37 filed (item 7),
   `syntax/wrappers` moved, and `derivation_cycle` and `unary-and-nary` left for T4, today's parser refusing their
   rewrites at the parse; item 8 landed as S1f.5.15's T1f.5.15.9. *T11 stopped before any edit*
   ([§ T11 record](s1f.5.5_the_grammar.md#t11-record-2026-11-08)): read as the coordinator read it, the ruling
   refuses `(imply1 functional __closed__)`, a stdlib idiom a test holds, so where a program may write a dunder is
   [Q-M1f.7](../open_questions.md#q-m1f7--where-may-a-program-write-the-kernels-dunders), put to the user. *Ruled
   (b) the same day, "Never bound", and T11 done*: a dunder is refused where a program binds a name (a declared
   relation, a rule's, an hrule's or a macro's name, an import's `:as`), through `ein_core::RESERVED`, and legal
   wherever one is used; the three writeback names are the generated grammar's `WRITEBACK` token, refused wherever
   written. T4 is next.
7. **T4, the swap, one commit:** the old `lex.rs` and `parse.rs` go and the printer reads names as the new
   lexer does; no program writes `:priority` and the grammar refuses it; the register rows the grammar fixes flip
   (W29's six, W32's seven, W35) and the rows the rulings restate are restated; the commit body lists the moved
   answers grouped by ruling.
8. **After the swap, the non-grammar halves**, one small stage at a time on the generated parser: S1f.5.5's T5
   (the loader's refusals the grammar now covers, and the grammar pages test); S1f.5.4's int kind and S1f.5.2's
   string value arms retired, moving no answer, with S1f.5.1's wildcard and S1f.5.11's keyword drops, each
   ruling held in every position by a test, and S1f.5.2's Unicode test and two examples; S1f.5.5's T6
   (`--diff-models`); then the pages, S1f.5.5's T7 and each stage's.
9. **[S1f.5.16](s1f.5.16_the_read_out_reference.md), the read-out reference, after the swap's non-grammar halves
   and before item 10's stages** (the user, 2026-11-08: *"Ok, schedule S1f.5.16 also"*). It writes one live
   page, `docs/kernel/inference/read_out.md`, for every row and number `ein solve` prints and what each counts.
   It generates the read-out's golden page beside it, and a test holds both to the printer. So S1f.5.7, S1f.5.6,
   S1f.5.8 and S1f.5.12 below, and S1f.5.14 last, each change the live page with the output, and no stage edits
   M1e2's `read_out.md` again. Its census and its examples are taken in the final grammar, which is why it waits
   for the swap. It moves no answer.
10. **The stages the grammar does not frame, after the swap and S1f.5.16**, so that their fixtures and pages are
   written in the final grammar, and what they change in the read-out is written on S1f.5.16's live page: S1f.5.7,
   S1f.5.10 and S1f.5.18, then S1f.5.6, S1f.5.12, S1f.5.13, S1f.5.8 and S1f.5.9.
   No stage file gives a reason for one to run earlier, and none needs the old parser: S1f.5.7's goals and
   claims over `eq` and `neq` already parse under [the paper grammar](s1f.5.5_paper_grammar.md#the-grammar-by-position),
   S1f.5.9's `(and)` is the grammar's refusal and its `:assert ()` the grammar's empty conclusion, and the rest
   touch no syntax. Each still runs one at a time, in that order, as their files say.
11. **S1f.5.14 last**, after every other stage (the user's ruling of 2026-11-08). Its first code task built the
    loader flag that makes `:why` required, default false, **early, on 2026-11-08** (*"yes, land the :why flag
    early"*), so what stays here is its last: the default turned to true, once every program the corpus, the
    stdlib and the Rust suites load carries one. Until these rulings this item read
    *S1f.5.14 and S1f.5.15 last, as the user placed them*; S1f.5.15 is item 3 since.

It costs about what the old order did, a quarter of a day more on paper (the estimate above): the four hand-written
lexer edits and the loader lines S1f.5.11 would have added for T5 to delete are not written, and the attribution
sweep, the prerequisite and a larger swap take their place. What it saves is a parser change written twice, and
a review taken four times.

## Acceptance for the phase

- Every item of the table is either fixed, with the test that holds it, or has an
  owner the user accepted, or is ruled to stay as it is, with the ruling written
  where the definition lives.
- A wrong answer among them is a register row before it is fixed, as M1e2's
  acceptance asked when the phase was planned. The register,
  [`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml), outlives M1e2's plans.
- Every golden a stage moves is named in its file before it moves.
- The language's grammar changes once, at S1f.5.5's swap, and every answer the swap moves is attributed to a
  ruling before the old parser goes (the re-plan of 2026-11-08, [§ The order](#the-order)). The generated
  grammar is the phase's final one from T3 on (the user's rulings of 2026-11-08): `:priority` leaves every program
  before the swap (S1f.5.15), and the `:why` S1f.5.14 requires is a loader flag's, not a grammar edit.
- `./run_tests.sh` is green at every stage boundary.
- Every page S1f.5.14, S1f.5.15 and S1f.5.16 add is **pinned by something that runs**, states its date and its commit,
  and links a `docs/history/` page rather than restating its subject. The rule is
  [Q-M1e.4](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)'s:
  *every count a test pins is exactly right; every count only prose states has drifted.*
- `RUSTDOCFLAGS="-D warnings" cargo doc` is green.

## The ids

| until 2026-11-08 | since | stage |
|---|---|---|
| — | `S1f.5.0` | the known wrong answers filed, new on 2026-11-08 |
| — | `S1f.5.17` | CI green again, new on 2026-11-08; `S1f.5.16` stayed the read-out reference, proposed then and specified the same day |
| `S1e2.5.1` … `S1e2.5.13`, in M1e2 | `S1f.5.1` … `S1f.5.13` | the no-owner items, in the same order |
| `S1f.5.5` | **`S1f.5.14`** | Every statement convertible to NL |
| `S1f.5.6` | **`S1f.5.15`** | Remove `:priority`; derive the order from the rules |
| `S1f.5.3`, proposed | `S1f.5.16`, specified 2026-11-08 | The read-out reference |

**`S1f.5.5` and `S1f.5.6` now name other stages.** A citation written before 2026-11-08 means the old
ones. M1e's record cites `S1f.5.6` twice as the `:priority` stage
([`docs/history/m1e_review_processing/README.md`](../../../docs/history/m1e_review_processing/README.md),
lines 160 and 489), and a record is not edited, so this table is how it reads. Every citation outside
`docs/history/` was renumbered on 2026-11-08, along with the task ids (`T1e2.5.<n>.<m>` → `T1f.5.<n>.<m>`,
`T1f.5.5.<m>` → `T1f.5.14.<m>`, `T1f.5.6.<m>` → `T1f.5.15.<m>`).

## The two stages that were P1f.5's

P1f.5 was *Documentation, and other* from 2026-08-29 to 2026-11-08, in
`p1f.5_documentation_and_other/`. **It was P1e.5**, in
[M1e](../../../docs/history/m1e_review_processing/README.md), until 2026-08-29. Two of its six stages
had already shipped and **stayed there** with the ids they shipped under; what
moved is the four that had not.

**Source:** the user's instruction of 2026-08-28 — *"we need a comprehensive
config options reference document in `docs/kernel/` — make it a first stage in
the new phase p1e.5 for ein documentation only."* Three further notes of the
same day added what are now S1f.5.14, S1f.5.15 and [P1f.20](../p1f.20_docs_refactor/README.md) (S1f.5.20
until 2026-11-08). The directory had been renamed `…_documentation_and_other` because S1f.5.15 is engine
work: it removes a language keyword and replaces it with a static analysis.

**The two that shipped stayed in M1e.** `S1e.5.1` (the configuration reference) and `S1e.5.2` (what a
solution is, and what a model is) landed on 2026-08-28 under M1e's numbering, and the tree cites `S1e.5.1`
as the stage that shipped [`configuration.md`](../../../docs/kernel/configuration.md).
[M1e's P1e.5](../../../docs/history/m1e_review_processing/README.md#p1e5--documentation-and-other) keeps
them. S1e.5.1 corrected five of the old phase's reconnaissance numbers; the record keeps that correction
beside what it corrected.

**What S1f.5.14 and S1f.5.15 are.** S1f.5.14 makes every statement convertible to NL: `zebra2`'s unique
model was **444 facts and 25 of them (5 %) had a template**. S1f.5.15 removes `:priority` and derives the
order from the rule dependency graph, on a control sweep where **137 of 139 corpus entries were identical**
with every `:priority` stripped. Each stage file carries its own case, dated.

**The risk the old phase named that still holds: a reference page is another hand-maintained copy.** A
prose table beside a list the code holds is a copy with no diff, which is what
[AR-M1](../../../docs/history/m1e_review_processing/README.md#the-findings) found. So a page S1f.5.14 adds is generated, or diffed by a test, and so is
[S1f.5.16](s1f.5.16_the_read_out_reference.md)'s, whose subject is the page this risk was first written
about. [`docs/api/rust.md`](../../../docs/api/rust.md)'s marker-guarded region
is the precedent that already works. The old phase's other two risks were about the configuration
reference, which shipped as S1e.5.1.
