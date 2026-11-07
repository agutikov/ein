# P1f.5 — The no-owner items, one plan per decision; then NL and `:priority`

**Milestone:** [M1f](../README.md)
**Estimate:** ~7 weeks — 17 stages, **37 days**: S1f.5.0, which files the known wrong answers as register
rows first, 1 day; [S1f.5.17](s1f.5.17_ci_green_again.md), which makes the CI green again on Ubuntu 26.04's
Graphviz and re-banks the mutant baseline, 1 day (added 2026-11-08); the thirteen no-owner stages, 25 days; then the two stages P1f.5 already had, 10 days ([S1f.5.14](s1f.5.14_nl_required.md) 4 d,
[S1f.5.15](s1f.5.15_rule_priority.md) 6 d). One more, S1f.5.16, is proposed and not specified.
**Status:** created in M1e2 as P1e2.5 on 2026-11-06, on the user's instruction: *"do not apply
modifications now, create new phase in m1e2 where put stages with plans to fix
every decided item; I will continue review and you will write down plans when
after my decisions"*. The review finished on 2026-11-08 with every item decided. **Moved to M1f the same
day**, on the user's instruction: *"I want to move P1e2.5 -> P1f.5 adding 2 stages of existing p1f.5 in
the end of phase"*. **Nothing here is built, and no stage starts until the user says so.**
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
| G-6 | a variable or a range written in a fact is a name (outside the 13; [the no-owner inventory](../../m1e2_correctness/inventory_2026-11-06_no_owner.md), § 3) | **ruled 2026-11-06**: `?x` in a fact is refused, and the range goes with S1f.5.3 | [S1f.5.2](s1f.5.2_strings_are_natural_language.md), default 1 |
| **F5** | **new: an unknown escape is dropped in silence** (below) | **ruled 2026-11-06**: strings are Unicode, with reasonable escapes, held by a test and shown by two examples | [S1f.5.2](s1f.5.2_strings_are_natural_language.md) |
| **F6** | **new: a range has no user** (below) | **ruled 2026-11-06**: no numbers and no ranges in Ein ([Q-M1e2.24](../../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)), and the token goes | [S1f.5.3](s1f.5.3_range_retired.md) |
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
[S1f.5.3](s1f.5.3_range_retired.md).

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
report on each of 5 000 corpus mutants over three runs, where the unpatched crate differed on 1 032.

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
| [S1f.5.1](s1f.5.1_bare_underscore_refused.md) | A bare `_` is reserved and refused, and `_` is valid anywhere in a name | 2 d | W-22, F3 | specified, not started |
| [S1f.5.2](s1f.5.2_strings_are_natural_language.md) | A string is natural language: only in `:why`, `:source` and `:goal-text` | 2.5 d | F4, F5, G-6 | specified, not started; runs after S1f.5.1 |
| [S1f.5.3](s1f.5.3_range_retired.md) | No ranges: the `RANGE` token retired | 1 d | F6, G-6's range half | specified, not started |
| [S1f.5.4](s1f.5.4_digits_are_names.md) | Digits are plain names, and a name is any run of name characters | 1.5 d | F7, F8, F2 | specified, not started |
| [S1f.5.5](s1f.5.5_the_grammar.md) | The grammar revisited: each position says what it holds, and the parser is generated from it | 8.5 d, re-taken at its step 1 | F9, F10, F11, F2, S-15, a question on G-14, and S1f.5.11's keyword lists as grammar | specified, not started; grmtools and recovery on ruled |
| [S1f.5.6](s1f.5.6_the_empty_state_decides_the_minimal_line.md) | Where ∅ is in R, every minimal-line question stops at root, and says only ∅ | 1.5 d | S-16, P-14, S-17, F1 | specified, not started; independent of the grammar stages |
| [S1f.5.7](s1f.5.7_eq_and_neq_are_relations.md) | `eq` and `neq` are relations: in a goal, a premise and a conclusion | 3 d | S-18, G-21; restates W24's guard rows | specified, not started; restates two W24 rows |
| [S1f.5.8](s1f.5.8_no_contradiction_no_core.md) | A node with no contradiction has no core: `null`, not `[]` | 0.5 d | D-14 | specified, not started |
| [S1f.5.9](s1f.5.9_an_empty_match_fires.md) | An empty `:match` fires | 1 d | F12 | specified, not started |
| [S1f.5.10](s1f.5.10_a_conclusion_variable_must_be_bound.md) | A conclusion's variable must be bound, and is refused at load where it is not | 0.5 d | F13; G-14's unbound part | specified, not started; after S1f.5.7, before S1f.5.9 |
| [S1f.5.11](s1f.5.11_closed_keywords.md) | Each form takes a fixed set of keywords, and `:where` is gone | 1.5 d | D-15, G-15, G-22 | specified, not started; on the hand-written parser, before S1f.5.5's swap |
| [S1f.5.12](s1f.5.12_cores_for_n_only.md) | A run takes a core only for the nodes a surface prints: N's members, after the walk | 1 d | P-15 | specified, not started; independent of the grammar stages |
| [S1f.5.13](s1f.5.13_large_dumps_to_the_slow_tier.md) | The round trip's four large dumps move to the slow tier, and the nightly runs it | 0.5 d | P-16 | specified, not started; independent |
| [S1f.5.14](s1f.5.14_nl_required.md) | Every statement convertible to NL | 4 d | was S1f.5.5 | specified, not started |
| [S1f.5.15](s1f.5.15_rule_priority.md) | Remove `:priority`; derive the order from the rules | 6 d | was S1f.5.6 | specified, not started |
| [S1f.5.17](s1f.5.17_ci_green_again.md) | CI green again: the per-commit job on Ubuntu 26.04's Graphviz, and the nightly's mutant baseline | 1 d | the red per-commit and nightly runs since `916d97ac` | specified, not started; **runs next after S1f.5.0** (the user, 2026-11-08: *"add new stage to this phase - fix failing CI jobs"*) |
| *S1f.5.16* | *The read-out reference* | *proposed* | was S1f.5.3 | *proposed*: one page for every number `ein solve` prints, and which of them counts *models* and which *what the search did*. [SE-M1](../../../docs/history/m1e_review_processing/README.md#the-findings) and [AR-M2](../../../docs/history/m1e_review_processing/README.md#the-findings) are findings about exactly this confusion. M1e2's [`read_out.md`](../../m1e2_correctness/p1e2.1a_the_solution_region/read_out.md) has since written much of it, and [P1f.20](../p1f.20_docs_refactor/README.md)'s S1f.20.2 gives that page a home |

**The order, if the user keeps it:** S1f.5.0 first, so that every known wrong answer is a register row
before a stage moves it; then S1f.5.17, so that every later stage boundary is held to a green CI as well as a
green gate; then S1f.5.5's census and paper grammar, since it frames the
others; then S1f.5.3, S1f.5.4, S1f.5.1 and S1f.5.2, in that order, on the hand-written parser, so that
each move of an answer is attributed to its ruling and no name is ever left unwritable, and S1f.5.11's
closed keywords beside them; then S1f.5.5's generated parser, run beside the old one and swapped in a
commit that moves no answer. **S1f.5.14 and S1f.5.15 come last**, as the user placed them. S1f.5.15
deletes `:priority`, to which S1f.5.5's grammar gives nothing new until then.

## Acceptance for the phase

- Every item of the table is either fixed, with the test that holds it, or has an
  owner the user accepted, or is ruled to stay as it is, with the ruling written
  where the definition lives.
- A wrong answer among them is a register row before it is fixed, as M1e2's
  acceptance asked when the phase was planned. The register,
  [`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml), outlives M1e2's plans.
- Every golden a stage moves is named in its file before it moves.
- `./run_tests.sh` is green at every stage boundary.
- Every page S1f.5.14 and S1f.5.15 add is **pinned by something that runs**, states its date and its commit,
  and links a `docs/history/` page rather than restating its subject. The rule is
  [Q-M1e.4](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all)'s:
  *every count a test pins is exactly right; every count only prose states has drifted.*
- `RUSTDOCFLAGS="-D warnings" cargo doc` is green.

## The ids

| until 2026-11-08 | since | stage |
|---|---|---|
| — | `S1f.5.0` | the known wrong answers filed, new on 2026-11-08 |
| — | `S1f.5.17` | CI green again, new on 2026-11-08; `S1f.5.16` stays the proposed read-out reference |
| `S1e2.5.1` … `S1e2.5.13`, in M1e2 | `S1f.5.1` … `S1f.5.13` | the no-owner items, in the same order |
| `S1f.5.5` | **`S1f.5.14`** | Every statement convertible to NL |
| `S1f.5.6` | **`S1f.5.15`** | Remove `:priority`; derive the order from the rules |
| `S1f.5.3`, proposed | `S1f.5.16`, proposed | The read-out reference |

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
[AR-M1](../../../docs/history/m1e_review_processing/README.md#the-findings) found. So a page S1f.5.14 adds
is generated, or diffed by a test. [`docs/api/rust.md`](../../../docs/api/rust.md)'s marker-guarded region
is the precedent that already works. The old phase's other two risks were about the configuration
reference, which shipped as S1e.5.1.
