# S1f.5.11 — Each form takes a fixed set of keywords, and `:where` is gone

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.11**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).
**Estimate:** 1.5 days.
- A quarter of a day to file the rows, sweep the corpus and list every move.
- Half a day for the loader's lists and the check inside terms.
- A quarter of a day for the drops that go, in the compiler and the macro expander, and the tests.
- Half a day for the pages: six current kernel pages describe `:where`.

**Status:** specified 2026-11-07 on the user's rulings of that day on D-15, G-15 and G-22
([Q-M1e2.28](../../m1e2_correctness/open_questions.md#q-m1e228--which-keywords-does-each-form-take)).
**Not started, and not to start until the user says so.** No engine code, test, golden or fixture has
moved.
**Depends on:** nothing. It lands on the hand-written parser before [S1f.5.5](s1f.5.5_the_grammar.md)'s
swap, as S1f.5.1 to S1f.5.4 do, so each move is attributed to this ruling. S1f.5.5's grammar then makes
each keyword a token of its own form, and this stage's loader lines move into the grammar
([S1f.5.5 § What it changes](s1f.5.5_the_grammar.md#what-it-changes), *Refusals move up*).
[S1f.5.3](s1f.5.3_range_retired.md) and this stage each refuse `:cardinality 1..1`, for its value and for
its keyword, so either can land first.

## The ruling

The agent explained D-15, `02_patterns.md`'s `:where` rows. The user, 2026-11-07: *"I don't remember we had
`:where` kvarg; we must have only fixed defined kwargs, list me all with description"*.

The agent listed every keyword, form by form. Only `query`, `solve` and `config` refuse a keyword they do not
know; every other form, and every term, drops one in silence. The agent then proposed a closed list per form
and four defaults. The user: *"yes, plan to remove `:where`"*. And on G-22, the same day: *"G-22 already
answered also - there are fixed set of allowed kwargs, if parser sees :k or :G :notes or whatever else unknown
kwarg - it is an error"*.

1. **Each form takes a fixed set of keywords**, the table below. Any other keyword is an error, refused
   before anything runs, on every command.
2. **A term and a nested fact take no keyword.** That covers a `:match`, an `:assert`, a `:goal`, an
   `:expect`, a pattern, an `and`, `or`, `not` or `absent`, a guard and a macro call. `:where` is on no list,
   so it is refused wherever it is written. This is G-15 and G-22, and it supersedes the half of
   [Q32](../../open_questions.md#q32--where-semantics) (2026-05-20) that left a `:where` pair to be dropped.
   Q32 removed `:where` and kept dropping its pair in silence.
3. **The four defaults stand:**
   - `:cardinality` is retired: it was never read, and its value is a range ([Q-M1e2.24](../../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)).
   - `:mode` on a query is retired: it is obsolete, accepted and ignored.
   - `:rule` and `:using` stay on a fact.
   - `:why` on a fact is refused.
4. **"If parser sees".** Today a keyword is one token for every form, so the check is the loader's. It runs
   before any run, and names the keyword's file, line and column, as a parse error does. S1f.5.5 moves it
   into the grammar, where it is a parse error.

## The lists

| form | keyword | holds | required |
|---|---|---|---|
| `relation` | `:why` | a string: the template that renders the relation's facts | no |
| `rule`, `hrule` | `:match` | a term: the premises | yes |
| | `:assert` | a term: the conclusion | yes |
| | `:why` | a string: the template that explains a firing | no |
| | `:priority` | digits: the rule's order, until [S1f.5.15](s1f.5.15_rule_priority.md) deletes it | no |
| a fact, negated or not | `:source` | a string: the fact is given, and where it is stated | no |
| | `:rule`, `:using` | a rule's name and its premises: the fact is derived | no |
| `query` | `:goal` | a term: the pattern asked about | yes |
| | `:goal-text` | a string: the headline template | no |
| | `:over` | `all`, `minimal` or `maximal`: which states | no |
| | `:expect` | a claim: the answer stated | no |
| `solve` | `:hrules`, `:hypothesis-relations`, `:no-hypothesis` | what may be guessed: the lattice | no |
| | `:expect` | a claim about the region | no |
| `config` | `:lattice-sanity-check`, `:record-alternative-justifications` | the flags ([`configuration.md`](../../../docs/kernel/configuration.md)) | no |
| `import` | `:symbols` | the names imported | no |
| | `:as` | an alias | no |
| `macro` | none | | |
| `trace`'s `step` | `:rule`, `:using`, `:derives`, `:source`, `:assumes` | one derivation step | no |
| a term, a nested fact | none | | |

Already closed today: `query`, `solve` and `config`. Their lines and their refusal fixtures stay as they are,
including a query's line naming `(solve …)` as the home of the three lattice keywords. A `config` flag the
milestone retired keeps its line naming the stage that retired it.

What stays the loader's: which keywords a form requires, and that each appears once.

## The case

Taken on 2026-11-07 at `916d97ac`, on the release binary, with `ein saturate FILE --dump` and `ein test FILE`.
Each program is `(relation p Node) (relation q Node) (relation hit Node) (p a) (p b)` and the line shown.

| written | today | after |
|---|---|---|
| **A condition or conclusion read as nothing** | | |
| `:match (and (p ?x) :where (neq ?x a))` | loads, derives `(q a)` and `(q b)`: the guard is dropped (Q32) | refused |
| `:match (p ?x :where (neq ?x a))` | the same | refused |
| `(rule r () :match (p ?x) :assert (q ?x) :where (neq ?x a))` | the same | refused |
| `:match (p ?x :k v)` (G-15) | loads, derives both | refused |
| `:assert (and (q ?x) :k v)` (G-22) | loads, derives `(q a)` and `(q b)`; `:k v` is gone | refused |
| `(query :goal (and (p ?x) :zz 1) :expect (model (p a) (p b)))` | loads; `ein test` says the claim held | refused |
| `(forall ?y (q ?y) (p ?y) :k v)` in a `:match`, `std.macro` imported | loads, derives `(hit a)` and `(hit b)`; the expander drops the pair | refused |
| **Refused today, but late or in the wrong spelling** | | |
| `:assert (q ?x :k v)` | loads; the run stops at the first firing, exit 1: *its :assert writes :k v as an argument of q, and no fact can hold it* | refused at load |
| `:match (and (p ?x) (absent (q ?x) :k v))` | exit 1 at compile, in the Python implementation's spelling: `ein.inference.compile.CompileError: (absent …) takes exactly 1 argument and was given 2: SForm(head=Atom(name='absent'), …` | refused at load, in Ein's spelling |
| `(eq ?x :k a)` | refused at load since W24 | refused, with this stage's line |
| **An annotation dropped** | | |
| `(rule … :zz 1)`, `(hrule … :zz 1)` | loads; the keyword is ignored | refused |
| `(relation s Node :cardinality 1..1)`, `(relation s Node :zz 1)` | loads; ignored | refused |
| `(q c :zz 1)`, `(q c :why "text")` | loads; ignored | refused |
| `(query :goal (p ?x) :mode x)` | loads; ignored, as an obsolete keyword | refused |
| `(import std.macro :zz 1)`, `(trace (step s1 :rule r :derives (q a) :zz 1))` | loads; ignored | refused |
| `(facts (r a b :source "(1)"))`: a pair in a nested fact | loads, and stores `(facts (r a b))` | refused |
| **Closed already** | | |
| `(query … :zz 1)`, `(solve :zz 1)`, `(config :zz 1)` | refused, with the form's list | the same |
| `:expect (model (p a) (p b) :zz 1)` | refused: *`:zz 1` is not a fact* | the same |

The first seven are wrong answers of W24's kind: a written condition or conclusion read as nothing, in
silence. Step 1 files them as a new register case, `open` and owned by this stage. Each witness goes under
`examples/ein-bugs/wrong-answers/`, and each row's `right` is the refusal. Three of them are the inventory's
probes: `inventory_2026-11-05/probes/kwpair_in_pattern.ein` (G-15),
`inventory_2026-11-06/probes/where_t3_example.ein` (D-15) and `inventory_2026-11-06/probes/assert_and_kw.ein`
(G-22). The rest are refusals with no answer to move, so they get no row.

## What moves in the corpus

A census of 2026-11-07: a throwaway S-expression reader over `examples/`, `tests/`, `stdlib/` and `corpus/`,
with strings and comments stripped, checking each keyword against its form's list and each term and nested
fact for a pair.

- **No program writes `:where`, `:cardinality` or `:mode`.** The two `.ein` files that write `:where` are the
  inventory's probes, under `plans/`.
- **Every keyword at the top of a form is on its list**, except in five refusal fixtures, which are refused
  today and after: `examples/broken/load/config_retired_flag`, `config_unknown_flag`,
  `expect_unknown_keyword`, `query_lattice_keyword` and `solve_unknown_keyword`. Their lines do not change.
- **Three files write a pair inside a nested fact**, and are refused after:
  - [`examples/syntax/wrappers.ein`](../../../examples/syntax/wrappers.ein): `(facts (r a b :source "(1)"))`
    and `(reasoning (r b c :rule sym :using (x)))`. It is a syntax fixture for the wrapper heads, whose inner
    pairs were never read. **Default:** the pairs go, the fixture keeps its wrapper shapes, and its digests
    move.
  - [`examples/ein-bugs/kwpair-below-the-filter.ein`](../../../examples/ein-bugs/kwpair-below-the-filter.ein)
    and its fuzz find, `corpus/fuzz_findings/kwpair-below-the-filter.ein`: `(tracex (step s1 :rule … :using …
    :derives …) …)`. It was banked for a render crash, *not a value node: KwPair*, which
    [S1e2.3.2](../../m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.2_a_keyword_pair_panics_two_views.md) fixed
    by labelling a pair `:key value`. Once no pair loads below a form, the crash's path is closed at load.
    **Default:** the fixture moves to `examples/broken/load/` as a refusal with its `.expected`, and the fuzz
    find's note says why. Step 1 says whether `value_label`'s `KwPair` arm is still reached, by a form's own
    pairs, and keeps it if so.
- **[`kwpair-in-guard.ein`](../../../examples/ein-bugs/wrong-answers/kwpair-in-guard.ein)**, W24's
  `(eq ?x :k a)`: refused today and after. Its row's `right` is restated if the line's text changes.

## What it changes

- **The loader** ([`from_ir.rs`](../../../ein.rs/crates/ein-ir/src/from_ir.rs)): one list per form, beside
  `QUERY_KEYWORDS`. Any other keyword is refused with one line per keyword, in the shape the query's line
  already has: *rule 'r': unknown keyword :where — one of :match :assert :why :priority*. `QUERY_KEYWORDS`
  loses `:mode`.
- **Inside a term or a nested fact**, any keyword pair is refused at load. The line names the form and the
  clause, as W24's lines do: *rule 'r': its :match writes :where (neq ?x a) inside a pattern, and a premise
  takes no keyword*.
- **No hint for a retired keyword.** `:where`, `:cardinality` and `:mode` are unknown keywords like any
  other, as the user ruled (*"whatever else unknown kwarg - it is an error"*). The pages record what went and
  why.
- **What goes**, since nothing can reach it once the loader refuses:
  - the compiler's drop of an in-`:match` pair ([`compile.rs`](../../../ein.rs/crates/ein-infer/src/compile.rs)
    line 546) and its `KwPair` filters (lines 491–497 and 910);
  - the macro expander's arity filter ([`macros.rs`](../../../ein.rs/crates/ein-ir/src/macros.rs) lines
    163–168);
  - `from_ir.rs`' *not refused* note (lines 921–922).
- **Tests that move.** Step 1 lists each:
  - `compile_semantics.rs`' drop test (lines 143–183) becomes a refusal test, and its empty-`absent` note
    (line 476) is restated;
  - `ir_semantics.rs`' four rules that write `:where` (lines 341, 381, 470 and 481) are rewritten with a
    positional guard or turned into refusal cases;
  - `from_ir.rs`' tests at lines 2209 and 2385–2386.
- **What does not move.** No corpus answer outside the three files above, and no register row but the new
  case. `:priority` stays until S1f.5.15. The shape of each keyword's value is S1f.5.5's grammar by position.
  So is a finding of this census, that a fact's `:using` reads a list and ignores its first element:
  `:using (x)` is no premise, and `:using (p (r c d))` is the one premise `(r c d)` (`from_ir.rs`,
  `ingest_fact`).

## The pages

The inventory's D-15 named `02_patterns.md`. The sweep (`grep -rn ':where' docs/`, outside `docs/history/`,
2026-11-07) finds six current kernel pages:

| page | what describes `:where` |
|---|---|
| [`ir/03-ein-lang/02_patterns.md`](../../../docs/kernel/ir/03-ein-lang/02_patterns.md) | line 14, *+ `:where` filters*; the construct table's two rows (52, 53); line 61, Q32's drop; line 151; § *As a T3 structural rule* (181–204), whose example has not loaded since `3ab9a231` |
| [`ir/01-ein-graph/02_rules.md`](../../../docs/kernel/ir/01-ein-graph/02_rules.md) | § 4, *The :where clause — predicate guards* (389–410); the figures' `:where` labels (29, 134, 470); the examples at 110, 170 and 320; step 2 of the matching algorithm (529) |
| [`ir/01-ein-graph/README.md`](../../../docs/kernel/ir/01-ein-graph/README.md) | line 14, *rule (LHS / `:where` / RHS)* |
| [`ir/03-ein-lang/03_examples.md`](../../../docs/kernel/ir/03-ein-lang/03_examples.md) | line 27: the `transitive` rule written `:where (neq ?a ?c)`, which loads today with its guard dropped. `zebra.ein` has written `(neq ?a ?c)` since Q32, and no test reads the page |
| [`ir/03-ein-lang/01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md) | line 22's `KEYWORD` row, whose examples are `:rule`, `:where` and `:cardinality`; line 256; the relation's `[:cardinality <RANGE>]`; the query's `:mode` |
| [`glossary.md`](../../../docs/kernel/glossary.md), [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md) | glossary line 56; `defined_behaviour.md` line 883, *a keyword's value inside a clause, `(n ?x :where 1..3)`*, and its Q32 lines |

Each page says what the form takes, that any other keyword is refused, and that a guard is a premise,
`(neq ?a ?c)`. `01_grammar.md` gains the table of [§ The lists](#the-lists). In
[`plans/open_questions.md`](../../open_questions.md), Q32 gains a line: the pair is refused since this stage.
`docs/history/` is a record, and does not change.

## The method

### Task T1f.5.11.1 — the rows, every move listed

The register case filed with its seven witnesses. The lists prototyped, and the corpus and
`corpus/fuzz_findings/` swept with every command that loads. `cargo test --workspace` is run on the
prototype. Every move must be one of the case's rows, the three files above, or a fixture whose line's text
changes. **Stop and show the user** the refusal lines and `wrappers.ein` as rewritten, computed on the
prototype.

### Task T1f.5.11.2 — the lists and the refusals

The loader's lists and the check inside terms. The drops go, and the tests move. A test in `ein-ir` holds one
line for each form and for each place in a term: a `:match`'s `and`, a pattern, an `:assert`, a `:goal`, an
`absent`, a guard, a macro call and a nested fact. `fuzz_properties` gains the property that no accepted
program carries a keyword its form does not list. The case's rows are fixed.

### Task T1f.5.11.3 — the pages

The six pages, Q32's line, and the inventory's D-15 and G-22, each saying what fixed it.

## Acceptance

- Each form takes exactly the keywords its row lists. Any other keyword is refused at load, on every command,
  one line per keyword naming the form and its list; that includes `:where`, `:cardinality`, `:mode`, `:k`,
  `:G` and `:notes`.
- No keyword pair loads inside a term or a nested fact.
- Nothing in the compiler or the macro expander drops a keyword pair, and the code that did is gone.
- No current kernel page describes `:where`, and `03_examples.md`'s `transitive` writes `(neq ?a ?c)`.
- The case's rows are `fixed`, and no corpus answer moves outside the three files listed.
- `./run_tests.sh` is green.
