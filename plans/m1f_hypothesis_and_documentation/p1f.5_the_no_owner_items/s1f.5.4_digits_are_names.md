# S1f.5.4 — Digits are plain names

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.4**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).
**Estimate:** 1.5 days.
- Half a day to re-take the moves on a prototype and show real programs.
- Half a day for the lexer, and to retire the int kind.
- Half a day for the register, the fixtures and the pages.

**Status:** specified 2026-11-06 on the user's ruling of that day, and extended 2026-11-07 on the
user's rulings on F8 and on what a name is
([Q-M1e2.24](../../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)).
**Not started, and not to start until the user says so.** No engine code, test, golden or
fixture has moved.
**Depends on:** [S1f.5.3](s1f.5.3_range_retired.md), so that `1..5` is gone first.
[S1f.5.5](s1f.5.5_the_grammar.md) frames it.

## The ruling

The user, 2026-11-06, on F7: *"YES, digits are not numbers, digits are plain names, and refuse
?x in facts; (p 1) and (p 01) are VERY DIFFERENT facts, (p "1") has to be refused in
plans/m1e2_correctness/p1e2.5_the_no_owner_items/s1e2.5.2_strings_are_natural_language.md"*.

The user, 2026-11-07, on F8 and on what a name is: *"yes, `0`, `1` - any alphanumeric plus connectors
signs (`_`,`-`) plus decorators signs (`$`, `%`, `^`, `*`, ...) string without space considered a NAME,
except variables that marked with `?` that sits in front of the name"*. The rule itself, and what the
agent read it to mean, are in [S1f.5.5](s1f.5.5_the_grammar.md#the-name-rule).

## The case

Taken on 2026-11-06 at `916d97ac`, on the release binary.

No operation reads a value as a quantity: the built-in predicates are `eq` and `neq`. But the
language has `INT ::= '-'? [0-9]+`, a value kind (`Tag::Int`, `IntId`, a pool and a `.einb`
section), and a numeric reading: `Terms::cmp_value` sorts by value, and `(p 1) (p 01)` is one
fact. Programs write a digit as `:priority 110` (475, in 136 files) and as a plain value (217, in
34 files; the agent's scan), a label in every case.

| written | today | after |
|---|---|---|
| `(p 1) (p 01)` | **one fact**, `(p 1)` | **two facts** (the ruling) |
| `(p 1st)` | `(p 1 st)`, split in silence (F2) | the name `1st` |
| `1_a`, `1-2`, `1.5` | refused | a name |
| `(1 a)` | refused | a fact of a relation named `1`, as any name |
| `-7` | an int | **a name**, so `(p -7)` and `(p 7)` are two facts; only `ein-bugs/int-goal-binding.ein` writes one |
| `1..5` | a range | a name, with no range reading ([S1f.5.3](s1f.5.3_range_retired.md)) |
| `(functional R 0 1)` | ints | names, and `std.elim`'s text does not change |
| `:priority 110` | a keyword's value | unchanged, and **not a name** (below) |

## What moves

**Measured without a prototype** (2026-11-06): every program that writes a digit or imports
`std.elim`, with each plain digit `1` renamed `d1` in a throwaway copy, including the stdlib's,
and `ein solve --json-summary` compared on today's binary. A digit is no candidate today and a
name is one (Q-M1e2.9 (a)), so the blind enumerator's universe grows. 36 programs: 17 answer as
they do, and 19 move.

| programs | today → with digits as names |
|---|---|
| `features/05_stdlib_domain_elim` | k_s 32, k_c 32 unmoved; **N 21 → 47** |
| `branching/12_typed_blind_solve` | k_s 3, k_c 7 unmoved; **N 25 → 51** |
| the `(edge 1 2)` family: `binding-key-*`, `activator-text-*` and four `corpus/fuzz_findings/` | k_s 1 or 2 → past 56 000, stopped at the budget; one exits 1 |
| `str-in-assert`, `str-in-match`, `classes-int-name` | k_s doubles or more; these go with [S1f.5.2](s1f.5.2_strings_are_natural_language.md) |
| `syntax/arg-kinds` | stopped either way, with larger counts |
| `broken/load/relation_parameter_int` | refused → loads, so the fixture's point is gone |

The two real programs keep R, and the extra members of N are dead sets over the new candidates.
**A label is a candidate like any other name: the user ruled F8 on 2026-11-07, as recommended**, so these
moves are accepted. A signature that names its types keeps a label out of H.
The `(edge 1 2)` family are W7's and W16's witnesses, whose point is a value kind that no longer
exists. Step 1 says which retire and which are kept, scoped with `:no-hypothesis`.

## What it changes

**The lexer.** `SYMBOL` becomes [the name rule](s1f.5.5_the_grammar.md#the-name-rule): a run of name
characters up to a separator, any of them first. `INT` goes as a value, and `EQ` with it, since `=` is a
name. On the hand-written parser this is the stage that lands the rule, and so most of F2.

**`:priority` is the trap.** `invariant.rs`' name walk reads `Node::Atom` even under a keyword
(line 501), so `:priority 110`, lexed as an atom, would make `110` a name of the program and a
candidate on every rule that has one, which is 136 files. It has to stay out: the walk skips a
keyword's value, or the loader reads it as a count and stores no atom. **Step 1 stops if H moves
on any program that writes no plain digit.** `:priority` itself is
[S1f.5.15](s1f.5.15_rule_priority.md)'s
to delete (the user, 2026-11-07), so this stage keeps it working and gives it nothing new: its value is a
run of digits, as every corpus use is. `ein_core::parse_int` is for CLI options, and stays.

**What only an int used.** `Tag::Int`, `IntId`, the pool and `cmp_value` in `value.rs` and
`terms.rs`; `Node::Int` in `parse.rs`, `ast.rs`, `from_ir.rs`, `expect.rs`, `compile.rs`, `dump.rs`,
`ir_dot.rs`, `dot_util.rs`, `read_out.rs` and `trace/ast.rs`; the arms in `saturator.rs`,
`obligations.rs`, `region.rs`, `invariant.rs`, `detector.rs` and `saturate.rs`; and `.einb`'s int
section, with its format version bumped (`einb/1.2`).

**Order.** Names sort by interned rank, so `(edge 10 …)` may print before `(edge 2 …)`. Goldens on
digit programs move, and step 1 lists them.

**The register.** W7's, W16's and W28's rows are about an int beside another value, and each `right`
is restated or retired on step 1's list. The fixes behind W16 and W28 compare by value, which is
still right.

**What it closes.** F7, F8, and F2: `a?x`, `?x?y` and `"a"b` are refused once a token runs to a
separator. S-15 shrinks: `(p 01)` against a stored `(p 1)` is
now a true difference, and a whitespace difference is what is left of it (`(p  a)` gets *0 matched*
today), which [S1f.5.5](s1f.5.5_the_grammar.md#task-t1f556----diff-models-compares-by-value-s-15) fixes.

## The method

### Task T1f.5.4.1 — every move listed, real programs shown

Prototype on a `git archive` snapshot under `build/`, on top of S1f.5.3. Sweep every `.ein` under
`examples/`, `tests/`, `stdlib/`, `corpus/` and `plans/`, run `cargo test --workspace`, and list
every move against the table above. **Stop and show the user real programs before any edit** (the
standing rule on syntax): `features/05`, `std.elim`, `zebra2.ein` unmoved, and `(p 1) (p 01)`.

### Task T1f.5.4.2 — digits are names

The lexer, the `:priority` guard, the fixtures and the register.

**What [S1f.5.3](s1f.5.3_range_retired.md) left for this task** (2026-11-08, [§ Between this stage and
S1f.5.4](s1f.5.3_range_retired.md#between-this-stage-and-s1f54)):

- **Empty `ein_corpus::REFUSED_UNTIL_S1F_5_4`** and delete `corpus_cli`'s
  `the_files_refused_until_digits_are_names_are_refused`, which holds the list both ways and fails in this
  commit, once `syntax/arg-kinds` and `wrong-answers/range-in-match` parse. With the list empty, the round
  trip's witnesses, `alive_invariant`'s `SEEDED`, the dump fixed point and `every_positive_entry_answers…`
  read both files again with no other edit; `W20/range-in-match` loads, a pattern over the name `1..3`, and
  its `right` is restated.
- **`cli_semantics`' `a_range_is_no_token` becomes its plain-name counterpart**: `1..5`, `1..*` and `2..*`
  in a fact, a keyword's value, a declaration, a rule's `:match` and `:assert` and a `(config …)` each read
  as the name it spells, which is S1f.5.3's acceptance, *a test holds each*. `:card 1..*` then loads with
  `:card` dropped until [S1f.5.11](s1f.5.11_closed_keywords.md) refuses the keyword, and the row says so.

### Task T1f.5.4.3 — what only an int used

Retired as a separate commit, so that no answer moves in it.

### Task T1f.5.4.4 — the pages

`00_ebnf.md`, `01_grammar.md` (the `INT` row), `02_patterns.md`, `defined_behaviour.md`, `glossary.md`,
`examples/README.md`. Q-M1e2.20 says what overrode it.

## Acceptance

- `(p 1) (p 01)` is two facts, each printing as written and reloading to itself.
- `1st`, `1_a`, `7`, `-7`, `1..5`, `$x`, `%`, `a^b`, `a/b`, `<=`, `!x`, `@x`, `~`, `'name'`, `f'`, `{}` and
  `[a]` load as names, and each prints as written and reloads to itself. `'name'` and `name` are two names.
  `?a.b` is a variable.
- A name holds only the characters [the list](s1f.5.5_the_grammar.md#the-name-rule) allows, so `a?x`,
  `?x?y`, `??x`, `a:b`, `a,b`, `a#b`, `a|b`, `a\b`, `Жук` and `"a"b` are refused, and no
  token splits in silence. `Жук` in a rule's `:why` string loads.
- `:priority` takes a run of digits and nothing else, until S1f.5.15 deletes it.
- `:priority 110` puts no name in the program, and H is unmoved on every program that writes no
  plain digit.
- `Tag::Int` and `Node::Int` are gone, and that commit moved no golden.
- `./run_tests.sh` is green.
