# S1f.5.11 — Each form takes a fixed set of keywords, and `:where` is gone

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.11**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).
**Estimate:** 1 day, re-taken on 2026-11-08 on the user's re-plan ([P1f.5 § The order](README.md#the-order)); it
was 1.5.
- A quarter of a day for the drops that go, in the compiler and the macro expander (T1f.5.11.2).
- A quarter of a day for a keyword held in every form and place, and the fuzzer's property (T1f.5.11.2).
- Half a day for the pages: six current kernel pages describe `:where` (T1f.5.11.3).
- Re-taking the rows [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md) filed, the sweep and every move (a
  quarter), and the lists and the check inside terms (half a day, in the loader) are
  [S1f.5.5](s1f.5.5_the_grammar.md)'s since the re-plan: its attribution sweep and review stop (T1f.5.5.9) and
  its swap (T1f.5.5.4), where each keyword is a token of its form and no loader list is written.

**Status:** specified 2026-11-07 on the user's rulings of that day on D-15, G-15 and G-22
([Q-M1e2.28](../../m1e2_correctness/open_questions.md#q-m1e228--which-keywords-does-each-form-take)).
**Not started** until the swap, which moved its refusals, fixtures and rows (below). **Running since 2026-11-09**:
T1f.5.11.2 done ([its record](#t1f5112-record-2026-11-09)): the compiler's drop and its filters, the expander's arity
filter, the loader validator's skips and the renderers' arms retired, each shown unreachable first, 33 rows of the
grammar's table and a fuzz property holding the lists, no answer and no golden but the table's moving; T1f.5.11.3,
the pages, next.
**Re-planned on 2026-11-08, on the user's ruling** ([P1f.5 § The order](README.md#the-order)): *"It is the same
work done twice, why not switch to grmtools first and remove hand-written parser and then verify requirement met
(strings, names, etc...) ?"* Until then this stage was to land the lists in the loader, on the hand-written
parser, before S1f.5.5's swap, and S1f.5.5's T5 was to move them into the grammar. Since then the lists are
written once, as the grammar's tokens ([the paper grammar](s1f.5.5_paper_grammar.md#the-grammar-by-position)), and
nothing edits the hand-written parser or the loader for them. This stage's step 1 is done inside S1f.5.5's
attribution sweep and its one review stop
([T1f.5.5.9](s1f.5.5_the_grammar.md#task-t1f559--every-difference-attributed-and-one-review-stop)), and the
refusals and **W32's seven rows land in S1f.5.5's swap** ([T1f.5.5.4](s1f.5.5_the_grammar.md#task-t1f554--the-swap)),
credited to this stage's ruling: each `right` is a refusal the grammar makes at the parse, so none waits for a
task here. What is left, T1f.5.11.2 and T1f.5.11.3, runs after the swap, on the generated parser.
**Depends on:** S1f.5.5's swap, which makes each keyword a token of its form. [S1f.5.3](s1f.5.3_range_retired.md),
done, refused `:cardinality 1..1` for its value; the swap refuses it for its keyword.

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
   into the grammar, where it is a parse error. *Since the re-plan of 2026-11-08 there is no loader check in
   between: the parser sees it, from the swap.*

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
| `(relation s Node :cardinality 1..1)`, `(relation s Node :zz 1)` | loads; ignored (the first a parse error at `1..1` from S1f.5.3, 2026-11-08, until S1f.5.4 makes it a name) | refused |
| `(q c :zz 1)`, `(q c :why "text")` | loads; ignored | refused |
| `(query :goal (p ?x) :mode x)` | loads; ignored, as an obsolete keyword | refused |
| `(import std.macro :zz 1)`, `(trace (step s1 :rule r :derives (q a) :zz 1))` | loads; ignored | refused |
| `(facts (r a b :source "(1)"))`: a pair in a nested fact | loads, and stores `(facts (r a b))` | refused |
| **Closed already** | | |
| `(query … :zz 1)`, `(solve :zz 1)`, `(config :zz 1)` | refused, with the form's list | the same |
| `:expect (model (p a) (p b) :zz 1)` | refused: *`:zz 1` is not a fact* | the same |

The first seven are wrong answers of W24's kind: a written condition or conclusion read as nothing, in
silence. [S1f.5.0](s1f.5.0_the_known_wrong_answers_filed.md) filed them on 2026-11-08 as W32, `open` and owned by this stage, each
witness under `examples/ein-bugs/wrong-answers/` and a corpus entry, and each row's `right` is the refusal.
Each witness is the case's own program, above. Three rows come from the inventory's probes, which were not
copied as written: `inventory_2026-11-06/probes/where_t3_example.ein` (D-15) is refused today for its
relation variable `?r` (W24's refusal), so it cannot show the drop;
`inventory_2026-11-05/probes/kwpair_in_pattern.ein` (G-15) writes the range `1..3`, which S1f.5.3 retires;
and `inventory_2026-11-06/probes/assert_and_kw.ein` (G-22) is the same shape over other relations. The rest
are refusals with no answer to move, so they get no row.

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
    move. *Landed 2026-11-08, ahead of the swap, by S1f.5.5's
    [T1f.5.5.10](s1f.5.5_the_grammar.md#t10-record-2026-11-08): the same stored facts, R, N and exit codes on
    today's binary; its five parse-view digests moved.*
  - `examples/ein-bugs/kwpair-below-the-filter.ein` (since S1f.5.5's swap
    [`examples/broken/kwpair-below-the-filter.ein`](../../../examples/broken/kwpair-below-the-filter.ein),
    refused at the parse) and its fuzz find, `corpus/fuzz_findings/kwpair-below-the-filter.ein`: `(tracex (step s1 :rule … :using …
    :derives …) …)`. It was banked for a render crash, *not a value node: KwPair*, which
    [S1e2.3.2](../../m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.2_a_keyword_pair_panics_two_views.md) fixed
    by labelling a pair `:key value`. Once no pair loads below a form, the crash's path is closed at load.
    **Default:** the fixture moves to `examples/broken/load/` as a refusal with its `.expected`, and the fuzz
    find's note says why. Step 1 says whether `value_label`'s `KwPair` arm is still reached, by a form's own
    pairs, and keeps it if so.
- **[`kwpair-in-guard.ein`](../../../examples/ein-bugs/wrong-answers/kwpair-in-guard.ein)**, W24's
  `(eq ?x :k a)`: refused today and after. Its row's `right` is restated if the line's text changes.

## What it changes

*Since the re-plan of 2026-11-08 the first two bullets are the grammar's, at S1f.5.5's swap, where a keyword its
form does not take is a parse error and no loader list is written; the rest is this stage's, after the swap.*

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
| [`ir/03-ein-lang/01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md) | line 22's `KEYWORD` row, whose examples are `:rule`, `:where` and `:source` (`:cardinality` went with S1f.5.3, 2026-11-08, as did the relation's `[:cardinality <RANGE>]`); line 256; the query's `:mode` |
| [`glossary.md`](../../../docs/kernel/glossary.md), [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md) | glossary line 56; `defined_behaviour.md` § 4.8, *a keyword's value inside a clause, `(n ?x :where v)`* (`1..3` until S1f.5.3), and its Q32 lines |

Each page says what the form takes, that any other keyword is refused, and that a guard is a premise,
`(neq ?a ?c)`. `01_grammar.md` gains the table of [§ The lists](#the-lists). In
[`plans/open_questions.md`](../../open_questions.md), Q32 gains a line: the pair is refused since this stage.
`docs/history/` is a record, and does not change.

## The method

### Task T1f.5.11.1 — the rows, every move listed

**Done inside S1f.5.5 since the re-plan of 2026-11-08**: its attribution sweep credits every keyword difference to
this ruling, and its one review stop shows the refusal lines and `wrappers.ein` as rewritten
([T1f.5.5.9](s1f.5.5_the_grammar.md#task-t1f559--every-difference-attributed-and-one-review-stop)). `wrappers`'
rewrite lands before the swap if it moves no answer (T1f.5.5.10), and `kwpair-below-the-filter` moves to
`examples/broken/` at the swap, a parse error and not a load one. The task as it stood is that sweep's brief for
this ruling:

It starts from W32's seven rows, which S1f.5.0 filed `open` on 2026-11-08, each `now` taken at
`ec004a48` (the engine `916d97ac`'s), release and debug alike, every one the case's answer above:

| row | witness | `now`, today |
|---|---|---|
| `W32/where-in-and` | `where-in-and.ein` | `solve --minimal`: loads; root holds `(q a)` and `(q b)` |
| `W32/where-in-pattern` | `where-in-pattern.ein` | the same |
| `W32/where-on-rule` | `where-on-rule.ein` | the same |
| `W32/kwpair-in-pattern` | `kwpair-in-pattern.ein` | `solve --minimal`: `(model (p a) (p b) (q a) (q b))`, `v` a name |
| `W32/kwpair-in-assert` | `kwpair-in-assert.ein` | the same |
| `W32/kwpair-in-goal` | `kwpair-in-goal.ein` | `solve`: one class, the claim `held` |
| `W32/kwpair-in-macro` | `kwpair-in-macro.ein` | `solve --minimal`: loads, flagged; root holds `(hit a)` and `(hit b)` |

`kwpair-in-goal`'s `now` records its query's class and claim beside the refusal, the register's schema
`/13`, and the fix drops both, which a refused `now` may not record. Three lines outside the register move
with the fix: `ein-infer/tests/alive_invariant.rs`' `SEEDED` names `v` on `kwpair-in-pattern` and
`kwpair-in-assert`, and `detector.rs` counts `kwpair-in-macro` among the flagged corpus programs (49 since
S1f.5.0); a refused program loads in neither.

Each re-taken. The lists prototyped, and the corpus and
`corpus/fuzz_findings/` swept with every command that loads. `cargo test --workspace` is run on the
prototype. Every move must be one of the case's rows, the three files above, or a fixture whose line's text
changes. **Stop and show the user** the refusal lines and `wrappers.ein` as rewritten, computed on the
prototype.

### Task T1f.5.11.2 — the drops gone, and a keyword held in every place

The lists and the refusals, and the case's rows fixed, **moved to S1f.5.5's swap on 2026-11-08**
([T1f.5.5.4](s1f.5.5_the_grammar.md#task-t1f554--the-swap)); the loader's lines for the three forms already closed,
and `QUERY_KEYWORDS`, go in S1f.5.5's T5. What is left, after both: the drops go (the compiler's in-`:match` pair
and its `KwPair` filters, the macro expander's arity filter, `from_ir.rs`' *not refused* note), and the tests that
named them move. A test holds one parse error for each form and for each place in a term, as rows of the grammar's
table: a `:match`'s `and`, a pattern, an `:assert`, a `:goal`, an `absent`, a guard, a macro call and a nested
fact. `fuzz_properties` gains the property that no accepted program carries a keyword its form does not list.

### T1f.5.11.2 record, 2026-11-09

**One commit, on `366877cc`** (S1f.5.1's T1f.5.1.4). Nothing in the compiler or the macro expander drops a keyword
pair, and the code that did is gone, each path shown unreachable first; the grammar's table holds a parse error for
each form and each place in a term, and the fuzzer holds the lists. **No answer moved, and no golden but the
grammar's table**, whose new rows are named first.

**The grammar's table** (`ein-ir/tests/grammar_pages.rs`, 256 → 289 rows; `cargo test -p ein-ir --test
grammar_pages`, 2026-11-09). Its golden, `grammar_table.txt`, gains the 33 rows' reports, 224 lines (`EIN_BLESS=1`),
each refused at the parse at its keyword, and none names a deleted line: each is the ruling's.
- **One for each form**, a keyword it does not take: a relation's `:cardinality 1..1`, a rule's `:where (neq ?x a)`,
  an hrule's `:zz 1`, a fact's `:zz 1`, a stored negation's `:k v`, a query's `:G 1`, a solve's `:notes (p)`, a
  config's `:zz true`, an import's `:zz 1`, a macro's `:k v`, a trace's `:k v` and a trace step's `:zz 1`, each
  *unexpected `:k`: a keyword this form does not take* at the keyword. T5's rows already held a query's `:mode` and
  an unknown keyword on a query, a solve and a config, and S1f.5.2's a fact's `:why`, a relation's `:source` and a
  query's `:why`.
- **One for each place in a term**: a `:match`'s `and`, twice (W32's `where-in-and` shape, and
  `compile_semantics`' old-where program, below), its `or`, a pattern in a rule and in an hrule (W32's
  `kwpair-in-pattern`), an `absent`, a premise's stored negation, a guard, an `:assert`'s `and` (W32's
  `kwpair-in-assert`, G-22), a conclusion, a conclusion's stored negation, a `:goal`'s `and` (W32's
  `kwpair-in-goal`) and its pattern, a macro call in a `:match` (W32's `kwpair-in-macro`) and in an `:assert`, a
  macro's body, a nested fact (the case's `(facts (r a b :source "(1)"))`) and one in a fact's `:using`, a claim's
  model, a region claim and a trace step's `:derives`. Each is refused at its keyword with the same line, but the
  two nested facts, whose `:source` is a keyword a fact takes, `unexpected :source`. A pair inside a pattern inside
  a pattern, a guard's argument that is a pair, a premise's `:where` and an `absent` of a pair alone were T5's rows.

**The fuzzer's property** (`ein-ir/tests/fuzz_properties.rs`' fifth,
`no_accepted_program_carries_a_keyword_its_form_does_not_list`): every keyword pair an accepted input of the
stream holds is a form's own clause and on its form's list (§ The lists, without `:priority`, which S1f.5.15
removed, and with a macro's `:why`, the user's of 2026-11-08), a trace's steps carrying a step's, and no term or
nested fact holds a keyword. It counts the pairs and fails below one for four accepted inputs: the per-commit
stream holds 261 in 442 (`cargo test -p ein-ir --test fuzz_properties`, 2026-11-09). The generator splices
`:source`, `:where`, `:cardinality`, `:mode` and `:k` beside `:why` since, so the stream writes a pair inside a
term as well as beside a form's own, which moves the nightly's stream: its fuzz job on this tree, both seeds at
1 000 000 iterations, ran after this commit, and its outcome is in T1f.5.11.3's record, below.

**How unreachable was shown** (T1f.5.1.3's harness, `build/s1f.5.11/t2/`). A copy of `366877cc` (`git archive`,
`probe/`) with a panic, `S11-PROBE <id>`, at 23 sites (`probe_patch.py`, by text), every arm that drops, skips,
walks into or labels a keyword pair in a function only ever handed a term: the compiler's three drops (the
disjunct and conjunct lists' filter, the premise's arm, the relation step's skip), its slot and guard lowering's
catch-alls met with a pair, and its guard key's pair code; the invariant's constant walk; the macro expander's
arity filter and its `expand` and `substitute` arms; the loader's validator (`refuse_no_fact_holds`: the pattern's
skip, the premise's arm, the conclusion's skip) and its term walkers (`names_relation`, `open_forms`, `pos_args`,
`vars_under`, `collect_vars_outside`, `walk_pattern`); `expect::render` and `node_repr`; `value_label` and the rules
view's `positional`. Each probe logs its id, the thread and argv.
- `cargo test --workspace --no-fail-fast` on it (`probe-tests.log`, 16 min 26 s): **949 passed and 1 failed, the
  one probe**,
  `render/value_label-KwPair` in `dot_util::tests::a_keyword_pair_below_a_form_is_labelled_key_value`, the unit
  test that handed `value_label` a fact's own `:rule` and `:using` pairs directly, which no view does (each filters
  a form's pairs before it labels its arguments); restated below.
- Every corpus file under 6 000 bytes (404) and 40 000 seeded mutants of them (`ein_corpus::mutants`, seed 0), then
  200 000 more (seed 987654321), each parsed, drawn (`ein render program`'s, `rules`' and `constraints`'
  functions), dumped, loaded, saturated and printed (`s11_sweep.rs`, T1f.5.1.3's `s1_sweep`): 16 317 refused at the
  parse, 2 646 at load, 64 at compile, 21 294 saturated and 83 stopped saturating, and 81 974, 12 729, 340,
  104 851 and 510, **no probe and no other panic** (`sweep.txt`, `sweep2.txt`), the outcomes T1f.5.1.3's on the
  same inputs.

And by construction: `ein.y` builds a `KwPair` only as a direct child of a top-level form and of a trace's `step`
(its `pair` calls), no production of a term or a nested fact takes a keyword, and the macro expander and the
import resolver copy only what a program wrote.

**Removed**, the drops and what only a pair inside a term reached:
- `ein-infer` `compile.rs`: `positional_args`, the filter the `(or …)` disjuncts and the `(and …)` conjuncts were
  read through; the premise's `KwPair` arm, Q32's drop, now an `unreachable!` naming `ein.y`'s `Premise`; the
  relation step's skip of a pair at a pattern's top level, with an `unreachable!` in `slot` naming `Arg` and `Term`
  where a pair would otherwise have become an opaque slot.
- `ein-ir` `macros.rs`: the arity filter, so a call's arity is its every argument (`(forall G B :k v)` expanded as
  `(forall G B)` until the swap).
- `ein-ir` `from_ir.rs`: the validator's three skips (a pattern's pair, a premise that is a pair, an `:assert`
  conjunct that is a pair; a pair met as a premise is now refused as one that is no form), `pos_args` and its
  filter, and the *not refused* note, restated as what was not refused until the swap and what retired it.
- `ein-render`: `value_label`'s `KwPair` arm (the review stop's item 11), an `unreachable!` naming `FactArg`, `Arg`
  and `Term`, and the rules view's filter in `positional`.
- Every comment that described a drop as live says what it was and when it went: `slot`'s and `guard_arg`'s,
  `plan.rs`' `Slot::Opaque` and `GuardArgKind::Node`, the invariant's, `value_label`'s, and the kwpair fuzz
  finding's note and corpus entry.

**Kept, and why.** The structural walkers that descend into a pair's value as into a form's arguments: the
expander's `expand` and `substitute`, the loader's `names_relation`, `open_forms`, `vars_under`,
`collect_vars_outside` and `walk_pattern`, the invariant's `walk_constants`, the guard key's code 7,
`expect::render` and `node_repr`. Each is handed a clause's value and so meets no pair (the probes above), but
none drops one: each is total over `Node`, as the dumper, the import resolver and `eq_nodes`, which do meet a
form's own pairs, are. Making them partial would turn a correct walk into a panic for a library caller that
hands one a whole form, and retire nothing the ruling names. The loader's name walk, the fact reader's filter
(`fact_args`), the relation-parameter check's filter, `constraints.rs`' and `ir_dot`'s `positional` filters and
`models.rs`' `first_loc` meet a top-level form's own pairs and stay as they are.
([Q-M1f.14](../open_questions.md#q-m1f14--which-keyword-pair-arms-go-once-no-term-holds-one), defaulted.)

**The tests that named the drops.** `compile_semantics`' `a_where_keyword_pair_in_a_match_is_dropped_and_the_rule_still_compiles`
is `a_where_guard_written_as_a_positional_premise_compiles`, its refusal half a row of the grammar's table (its
program, verbatim) and its compile half kept, with its now unused `errors` helper gone; `macros.rs`'
`arity_is_counted_over_positional_args_only` is `arity_is_counted_over_every_argument`; and `dot_util`'s
`a_keyword_pair_below_a_form_is_labelled_key_value` is `no_keyword_pair_reaches_a_values_label`: the fuzz witness
refused at the parse, and a fact carrying its own `:rule` and `:using` drawn by both views that drew the witness,
its arguments labelled. The stage's other tests (`ir_semantics`' `:where` rules and `from_ir`'s `:mode` queries)
moved at the swap.

**Nothing moved.** Every `.ein` under the five roots (517, `find examples tests stdlib corpus plans -name '*.ein' |
wc -l`, 2026-11-09) through the release builds of `366877cc` and this commit (`build/s1f.5.11/t2/xcheck/`,
T1f.5.1.3's scripts, 156 s): `solve --json-summary -H --dump-answer`, `saturate --dump` and `test`, timings masked,
**517 of 517 the same on every command**; T12's 2 000 mutants through `saturate`, the error text included, the same
(1 047 accepted, 767 refused with repairs, 186 without); and the container both ways: on the 344 that load, the four
reads agree with each other and with `solve` of the `.ein` but `g14/derived-activator.ein`, whose four reads agree
with each other and place its compile refusal at `<string>`, as T1f.5.1.3 found.

**Taken by default on the user's instruction of 2026-11-09** (*"just continue with default decisions till the end
of the phase"*):
- **Which arms go** (above): the drops, the validator's skips and the renderers' arms, and not the walkers;
  [Q-M1f.14](../open_questions.md#q-m1f14--which-keyword-pair-arms-go-once-no-term-holds-one).
- **A retired drop that needed an arm** is an `unreachable!` naming `ein.y`'s production, as Q-M1f.9 (a) has a
  deleted loader line: the premise's, `slot`'s and `value_label`'s. Where no arm is needed the filter went with
  nothing in its place: the expander counts every argument, and the validator refuses a pair met as a premise.
- **The rows** are one per form and one per place in a term, the case's own programs where it had one, so each W32
  witness's shape is a row; the generator gains five keyword splices, so the property reads a stream that writes
  pairs in terms.

**The gate.** On this commit, 16 min 11 s (`build/gate-s1f.5.11-T2.log`), green: 951 tests, T1f.5.1.3 step 3's 950 and
the fuzz property, and every static check.

### Task T1f.5.11.3 — the pages

After the swap, beside S1f.5.5's T7.

The six pages, Q32's line, and the inventory's D-15 and G-22, each saying what fixed it.

## Acceptance

**Since the re-plan of 2026-11-08 the items the grammar settles are
[S1f.5.5's acceptance](s1f.5.5_the_grammar.md#acceptance)**, held at its swap: each form takes exactly the
keywords its row lists and any other is refused on every command, `:where`, `:cardinality`, `:mode`, `:k`, `:G`
and `:notes` among them, no keyword pair parses inside a term or a nested fact, the case's rows are `fixed`, and
no corpus answer moves outside the three files listed. A refusal is the parse error, naming the file, line and
column and the keywords the form takes, where this page had the loader's line. This stage's own, after the swap:

- A test holds one parse error for each form and each place in a term, and the fuzzer holds the property.
- Nothing in the compiler or the macro expander drops a keyword pair, and the code that did is gone.
- No current kernel page describes `:where`, and `03_examples.md`'s `transitive` writes `(neq ?a ?c)`.
- `./run_tests.sh` is green.
