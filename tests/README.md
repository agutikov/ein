# tests/

**Ein programs whose job is to fail when the engine is wrong.**

Everything here is an `.ein` file carrying an `:expect` (a query's, or since
M1e2 S1e2.1a.4 the region claim on `(solve …)`), so it states its own answer
and `ein test` turns the directory into a status code:

```sh
ein test tests/                       # 0 if every claim holds, 1 if one is false
ein test tests/stdlib/slots/ -v       # one line per claim, with the question asked and what it found
```

**This is not `examples/`.** That directory is a set of things to read and run
— two Zebra encodings, per-feature demonstrations, the negative fixtures the
diagnostics are pinned against — and a reader goes there to find out what
ein-lang looks like. These are tests: each one exists to break, most of them
are three declarations and two facts, and nobody would learn the language from
them. The split is the reason `tests/` is a root of its own rather than
`examples/stdlib/`.

They are corpus entries all the same. `corpus/corpus.toml` has one row per file
(group `stdlib`, five runs each) and `ein-corpus`'s completeness check walks
this directory beside the other two, so a test file with no entry fails the
gate — the sweep would otherwise never run it, which reads as coverage nobody
has.

## `region/` — the region claim, one program per form

M1e2 [S1e2.1a.4](../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.4_the_language.md).
Each program states its expected region on `(solve … :expect …)`, and `ein
test` checks it against R as `ein solve` describes it. That is route 1, or the
cube on a flagged program, and never the walk. The claims were rendered from
each program's computed R, the way
[`region_examples.md`](../plans/m1e2_correctness/p1e2.1a_the_solution_region/region_examples.md)'s
were.

| file | the form |
|---|---|
| `01_states.ein` | states, ∅ among them as `(model)` |
| `02_merged_single.ein` | `(merged …)` on every state, one `(model …)` from each |
| `03_merged_or.ein` | `(merged …)` with an `(or …)` of two commitments of different sizes |
| `04_owing.ein` | `(owing …)`: R empty, root owes |
| `05_empty.ein` | `(region)`: R empty, either kind |
| `06_false.ein` | `(false)`: every node dead |
| `07_cube.ein` | a flagged program, checked against the cube |

Every claim here holds. Their false twins, and the claims a route cannot
check, are `ein-infer/tests/claim_semantics.rs`. Corpus group `positive`,
runs `test` and `saturate`.

## `text/` — natural-language text

M1f [S1f.5.2](../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.2_strings_are_natural_language.md)
(T1f.5.2.2). A string is natural-language text, written only after `:why`,
`:source` and `:goal-text`, and it is Unicode with reasonable escapes (the
user, 2026-11-06). `01_unicode.ein` writes each of the three with Cyrillic,
CJK, an emoji and every escape a string reads, `\n \t \r \" \\` and `\u{…}`,
and one `:source` with a raw newline; its claim is about facts, which the text
does not move. `ein-cli/tests/natural_language_text.rs` holds the text byte for
byte on every surface that carries it: the read-out and `answer.txt`,
`--trace`, `ein saturate --dump`, a `--dump-answer` state file, `ein render
program` and a `.einb`. Corpus group `positive`, runs `test`, `solve` and
`saturate`.

## `imports/` — a theory built on a theory

M1f [S1f.6.25](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.25_the_section_j_fixes.md)
(T1f.6.25.4, E-20 of P1f.6's findings). Five files: two theories, `lower.ein`
and `upper.ein` over it, each with an entry rule; `upper_without_entry.ein`,
the upper one with the lower theory's entry name left out of its own; and two
programs that import the upper theory by its entry. `01_nested_import_whole`
gets both theories and its request answered. `02_nested_import_left_behind`
was the witness of E-20: until M1f S1f.6.35 a selection's closure followed
names, so it got only the *declarations* of the lower theory's relations, its
request was never answered, and the loader printed a `note:` line. Since,
every import brings its module whole, the upper module's own import of the
lower one among them, and it is answered as `01` is, `(got a aa)`; nothing is
left behind and no line is printed (`ein-cli/tests/imports_by_full_name.rs`).
Corpus group `positive`.

## `stdlib/` — the stdlib conformance corpus

M1c [S1c.1.4](../docs/history/m1c_external_validation/README.md#s1c14--the-stdlib-corpus).
One program per stdlib rule or tight family, each the smallest thing that
activates it and each stating what it should and should not derive. **The
directories mirror the library**: a module `std.x` has its programs under
`stdlib/x/`, and `std.math.x`, the calculator's, under `stdlib/math/x/` since
M1f S1f.6.31, which moved `arith/` to `math/decimal-arith/` and `digits/`,
`int/`, `rat/` and `num9/` under `math/` with their modules; `std.bool.x`'s
are under `stdlib/bool/x/` since M1f S1f.6.17.

**Why they exist.** Every other check in this repository is *relative*: the
goldens compare ein.rs to its own past. The stdlib was where that gap was
widest —
[the census](../docs/history/m1c_external_validation/stdlib_census.md)
measured **38 of 73 rules never firing** on 2026-08-23 over 400 corpus runs,
33 of them never even loaded, and 20 more held up by `examples/zebra.ein`
alone.

| dir | rules | what the programs do |
|---|---:|---|
| [`stdlib/algebra/`](stdlib/algebra/) | 40 | the copiers and the relative product; the Boolean lattice; the extensive operators and their closed-world caveat; the tag lemmas and the two Tarski equations; Schröder's negative propagation; then the seven property **checks** — once satisfied, once violated apiece — the totality pair's `forall` in both directions, and (M1d S1d.2.4) the totality pair's **obligation** duals, each once owing and once satisfied |
| [`stdlib/bijection/`](stdlib/bijection/) | 8 | the two setup fan-outs and the negative completion; domain- and range-elimination, each productive in its own file and redundant in its sibling's; the two arg typechecks violated; and `06_blind_enumeration.ein`, the one program here that leaves the enumerator **on** — since M1d S1d.2.5 by making the obligations rung *decline*, which is the only way to reach it from a program that declares one |
| [`stdlib/elim/`](stdlib/elim/) | 4 | the same inference in its *positional* formulation, plus `no-room-left` — the pair that brackets a quantifier: one exclusion short of full forces a value, exactly full refutes |
| [`stdlib/closure/`](stdlib/closure/) | 1 | `infer-closure`, and its soundness caveat **exhibited**: a program the import takes from fifty-four models to one; plus `02`, closed and satisfied; its twin `03`, closed and **owing**, the one place in the suite where one fact decided a verdict word from M1d S1d.2.6, is `examples/ein-bugs/closed-and-owing.ein` since M1f S1f.6.34, which refuses an obligation on a closed relation, and `02` dropped its obligation |
| [`stdlib/slots/`](stdlib/slots/) | 20 | a second activating puzzle for the module that had exactly one — the partition chain; `slot-fill` and `slot-elimination` as a matched pair, each productive where the other cannot fire; both violation duals; and the spatial family over two different position structures, one of which exists nowhere else and is what tells `slot-prune-fwd` from `slot-prune-bwd` |
| [`stdlib/typing/`](stdlib/typing/) | 4 | the reflexive-closure knob, and the `(type-hierarchy …)` knob in both directions |
| [`stdlib/math/decimal-arith/`](stdlib/math/decimal-arith/) | 113 | `std.math.decimal-arith`, school arithmetic computed by saturation alone (M1f S1f.6.3, addition; S1f.6.4, subtraction and comparison; S1f.6.5, multiplication; S1f.6.6, division; S1f.6.12, malformed numbers and undeclared names; S1f.6.38, the numeral's check moved to `std.math.decimal`, a `let` that names a digit, and an operand's fault through each of the rules from a tag to a request): one program per rule or tight family, each claiming its result relation's whole extent and R = {∅}, but the two that divide by zero and the fourteen that state something the theory flags, which claim `(false)`; see [below](#stdlibmathdecimal-arith-a-theory-whose-answers-a-program-cannot-check) |
| [`stdlib/math/decimal/`](stdlib/math/decimal/) | 6 | `std.math.decimal`, the numeral and its check where it is written (M1f S1f.6.38): five programs: a well-formed numeral in every place a file writes one; a malformed one in a `let` nobody asks about, in a fact of the program's own and in a rule's conclusion, each claiming `(false)`; and what stays silent. `std.math.arith`, whose two rules the division-by-zero programs activate, has none of its own; see [below](#stdlibmathdecimal-a-numeral-checked-where-it-is-written) |
| [`stdlib/math/num9/`](stdlib/math/num9/) | 47 | `std.math.num9`, school arithmetic on nine digits in one fact by wide rules (M1f S1f.6.13): a program a rule or tight family, thirteen in all: a sum in one firing and a sum that overflows; subtraction's two rules; the one rule of comparison; a product, its zero digit and its three overflows; a quotient, each of the ten step rules, and a division by zero through `std.math.decimal-arith`'s rule; the conversions at every count of digits; both modules in one program; and a chain. See [below](#stdlibmathnum9-wide-rules-and-a-generated-module) |
| [`stdlib/math/int/`](stdlib/math/int/) | 82 | `std.math.int`, integers as a positive and a negative part over `std.math.decimal-arith` (M1f S1f.6.7, and S1f.6.22 for division): fourteen programs: the normal form; negation; addition, subtraction, multiplication and comparison, each in every combination of signs; an operand that is not simplified answering as its normal form does; operands by name with two rules of the program's own; division under each of the three conventions, every combination of signs with a remainder and without; and a division by zero under each, claiming `(false)`; and (S1f.6.38) twenty-two more, E-38's probes banked and the integer's own shape: a malformed numeral in a part, an undeclared name and a digit as an operand, a digit or a named constant as a part, each claiming `(false)`, and a natural number for an integer, silent. See [below](#stdlibmathint-a-theory-over-a-theory) |
| [`stdlib/math/rat/`](stdlib/math/rat/) | 67 | `std.math.rat`, rational numbers in lowest terms over `std.math.int` and `std.math.decimal-arith` (M1f S1f.6.8): thirteen programs: lowest terms and Euclid's chain; negation; addition, subtraction, multiplication, division and comparison; an operand not in lowest terms answering as its normal form does; operands by name and a chain; and four that claim `(false)`, a division by a rational that is zero and a denominator of zero in each place an operand can stand; and (S1f.6.38) seventeen more, E-38's probes banked and the rational's own shape, as `std.math.int`'s. See [below](#stdlibmathrat-lowest-terms-and-four-ways-to-divide-by-zero) |
| [`stdlib/math/digits/`](stdlib/math/digits/) | — | `std.math.digits` is a library of **facts** (M1f S1f.6.2) and has no rule to activate; see [below](#a-module-of-facts-has-no-rule-to-activate). Two programs: every table imported flat with a row of each claimed and R = {∅}; and a refutation through a lookup, whose core names the row |
| [`stdlib/bool/`](stdlib/bool/) | 55 | `std.bool.gates`, `.props`, `.circuit` and `.inputs`, the sixteen Boolean functions of two inputs and a circuit (M1f S1f.6.17): twelve programs, eight of them written by `utils/gen_gates.py`, each claiming relations whole; a circuit simulated and one refuted; inputs as hypotheses, a truth table and a requirement. See [below](#stdlibbool-the-sixteen-functions-by-rules-and-a-circuit) |
| [`stdlib/macro/`](stdlib/macro/) | — | `forall` checked against its own expansion written out by hand, and `unknown`'s three states |

### The obligation pairs, and the check that is not an `:expect`

M1d [S1d.2.4](../docs/history/m1d_satisfiability/README.md#s1d24--obligations-in-the-saturator)
added four rules that assert the verdict atom `open` — `total-owed`,
`surjective-owed`, `slot-owed-room`, `slot-owed-fill` — and eight programs,
one firing and one satisfied per rule. They are written the way everything
else here is, with one exception that is worth knowing before writing a
ninth.

**`:expect` cannot state an owe claim, and will not learn to.** Its three
forms are all assertions about *facts*, and an `open` conclusion is by
construction never a fact: it is a tally on the search-lattice node, because a
stored one would survive its own discharge in a fork that paid it. So each of
the eight carries an ordinary `:expect (model …)` — which is what satisfies
the coverage gate and what pins the *state* the debt is read from — and the
count itself is asserted in
[`ein-infer/tests/obligation_reports.rs`](../ein.rs/crates/ein-infer/tests/obligation_reports.rs),
in-process, the same shape S1c.1.5 used. Growing the grammar a word for an
open verdict is a verdict-vocabulary change and is S1d.2.6's to take.

**The satisfied half is the half that matters.** The coverage gate is happy
with the firing file alone. What it cannot show is that the report ever
*stops*: discharge **is** the guard — there is no second query that could
disagree with the rule's own `absent` — so "this file reports nothing" is the
whole claim that the guard is right.

**`06_blind_enumeration.ein` is not a rule's test.** It exists because the two
new activator facts per declaration are *stored* facts, and the question "can
a blind run start guessing about them" had to be a fixture rather than an
argument. It found a bug: an obligation rule's name was categorising as an
*object*, so the enumerator proposed `(seats total-owed C1)` — the name of a
rule as a puzzle value. It is the only program in this directory that leaves
the enumerator on, and it names only the hierarchy relations in
`:no-hypothesis`, because a file that scoped the activators out would be
testing its own scoping line.

**And since M1d S1d.2.5 it has to *earn* the enumerator**, which is worth
knowing before editing it. Hypothesis generation is a ladder now — the puzzle's
hrules, else what the state owes, else the blind enumeration — so a program
declaring `(bijective seats)` no longer reaches the third rung at all, and this
file's whole check went vacuous the day the ladder landed. It gets it back with
a second obligation, `sorted-somewhere`, a bare `(open)`, which names no
relation to branch on, so the rung declines it, and a declined obligation hands
the whole call to the blind enumerator. (It owed `is-a` until M1f S1f.6.34,
declined by the domain contract's C4, since `is-a` is what `(total-owed seats
is-a)`'s guard scans; beside a `:no-hypothesis` naming `is-a` that is an
obligation the program never lets be guessed, which the stage refuses.) So the
file is also the corpus's one `mode=declined` fixture, and the obligation it
added is *satisfied* on purpose: the decline must not depend on the KB
happening to owe something.

### A module of facts has no rule to activate

M1f [S1f.6.2](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.2_digits_and_tables.md).
`std.math.digits` holds the calculator's seven digit tables as plain facts, 1 664
rows, and no rule. The coverage gate asks that every stdlib *rule* be
activated by a program here, so it has nothing to ask of this module, and
`stdlib_coverage.rs` names it as rule-free, beside `std.macro`, rather than
inferring that a module with no rule is fine. The mutation sweep likewise
mutates rules and has none here.

**What holds the rows is not a program here.** A wrong row is a wrong answer
in every sum that reads it, and no `.ein` program can say what 7 + 5 is
without the table it would be checking. So the rows are held from outside the
language: [`ein-ir/tests/digit_tables.rs`](../ein.rs/crates/ein-ir/tests/digit_tables.rs)
computes every table from Rust's integers and compares, none missing and none
twice, and runs the generator's `--check`.

The two programs under [`stdlib/math/digits/`](stdlib/math/digits/) hold what only a
program can show:

| file | what it holds |
|---|---|
| `01_tables.ein` | an import brings a table's rows (S1f.6.11's fix, W88, seen from the stdlib), and since S1f.6.35 the module whole; each goal is a lookup, constants where the table reads and variables where it gives, and its claim closes the goal, so a second row for one reading fails the file; `nz` and `adj` are claimed whole; and the module's own closing facts, which every import brings since S1f.6.35 (until then only with `__closed__` listed), leave **no candidate**, R = {∅}, claimed on the `(solve …)` form with no `:hypothesis-relations` list to help |
| `02_a_core_names_its_rows.ein` | a refutation through a lookup: root is dead and the unsat core is `(column 7 5 3)` and `(fa 7 5 0 2 1)`, the claim and the row it was checked against. The agreeing `(column 7 5 2)` is there so the rule is matched and silent once (*a refutation rule gets two files*, here two facts) |

### `stdlib/math/decimal-arith/`: a theory whose answers a program cannot check

M1f [S1f.6.3](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.3_addition.md).
`std.math.decimal-arith` is school arithmetic written in Ein over `std.math.digits`' tables,
computed by saturation alone. Its programs are written like the others here,
one per rule or tight family, each claiming `sum-of`'s whole extent, with two
differences worth knowing before adding one.

**Every program claims R = {∅}** on its `(solve …)` form, with no
`:hypothesis-relations` list. The theory closes its own relations, and that
claim is what fails if a relation added to the module is left out of the
closing rule. `ein-cli/tests/arith_search_free.rs` reads the same off every
program in this directory and in `examples/arith/`: no candidate, one state,
one saturation, and the same result facts under `ein solve`, `ein saturate`
and `ein test`. A new program joins by being here.

**Right is held from outside the language.** A program here can claim a
handful of sums, written out by hand. Whether the theory adds *correctly* is
`ein-infer/tests/arith_theory.rs`, against Rust's integers: every pair up to
120, every pair of lengths up to 30 digits on a fixed-seed sample, a carry
through every length, and operands padded with high zeros. The same file
holds the rule of the theory on the module's text, and `arith_batch.rs` holds
it on the matcher's count of its work, under `--features counters`, which no
tier builds since M1f S1f.6.21: `cargo test -p ein-infer --features counters
--test arith_batch` is run by hand by a stage that changes a rule of the
calculator. Every operation is a row of that
file's `OPS` and is asked the same cases: subtraction and comparison since
S1f.6.4, with equal operands and operands one apart at every length, a borrow
through every length, and requests by name.

| file | the rules it is there for | what would fail it |
|---|---|---|
| `01_add.ein` | the whole chain once, 123078 + 4567: `add-ask`, `add-ask-second`, `add-start`, `operand-number`, `add-column`, `add-column-first`, `add-end`, `strip-top`, `turn-step`, `turn-end`, `add-answer` | a wrong digit, a second answer, an answer filed under the operands exchanged |
| `02_carry_out.ein` | `add-carry-out`: 999999999 + 1 has ten digits, and 5 + 5 two | the carry lost at the top |
| `03_lengths.ein` | `add-column-second` beside `add-column-first`, each pair asked both ways round, with a carry running through the longer number | the two column rules confused |
| `04_zero.ein` | `operand-zero`, `strip-none`: zero is `z`, asked, added and answered | a zero answered as `(n z 0)` |
| `05_high_zeros.ein` | `strip-zero`: operands written with high zeros, read by value; a zero inside a number stays | an answer that is not in normal form |
| `06_named.ein` | `let-named` and `operand-name`: `(let NAME NUMBER)`, a request by name, by a name and a number, and the same sum by numbers as a request of its own | a request answered under another spelling of its operands |
| `07_only_what_is_asked.ein` | nothing fires for a `let` nobody asks about: `done`'s whole extent is one call. (Until S1f.6.12 it also asked for a sum naming a name no `let` states, which is flagged since: `29`) | a theory that computes what was not asked |
| `08_sub.ein` | subtraction's whole chain, 123078 − 4567 and the reverse: `sub-ask`, `sub-ask-second`, `sub-start`, `sub-column`, `sub-column-first`, `sub-column-second`, `sub-end`, `sub-borrow-out`, `sub-answer`, `sub-answer-short`; `short`'s whole extent is claimed too | a wrong digit, the two numbers of the answer exchanged, a walk that ended with a borrow answering anyway |
| `09_sub_borrow_and_zeros.ein` | a borrow through every column, high zeros dropped, equal numbers, operands with high zeros, zero on either side | an answer with a high zero, an equal pair answered twice |
| `10_sub_named.ein` | `sub` by `(let NAME NUMBER)`, and a rule of the program's own that matches the shortfall `(diff-of X Y z (n ?p ?a))` | a request answered under the numbers its names stand for; a shortfall reported for an equal pair |
| `11_cmp.ein` | comparison's whole chain: `cmp-ask`, `cmp-ask-second`, `cmp-start`, `cmp-column`, `cmp-end`, `cmp-answer`; `lt`, `eq`, `gt`, and a higher column overruling the order below | the order of the units answered as the order of the whole, a request answered with two orders |
| `12_cmp_lengths_and_names.ein` | `cmp-column-first`, `cmp-column-second`: different lengths, high zeros, zero, named operands | a longer spelling read as a greater number |
| `13_mul.ein` | multiplication's whole chain, 123 × 456 and the operands exchanged: `mul-ask`, `mul-ask-second`, `mul-start`, `mul-turn`, `top-first-step`, `top-first-end`, `mul-begin`, `mul-row`, `mul-column`, `mul-carry-out`, `mul-walk-end`, `mul-first-row`, `mul-add-row`, `mul-next`, `mul-end`, `mul-answer` | a row added without the shift, the multiplier's digits taken from the wrong end, a product filed under the operands exchanged |
| `14_mul_zeros_and_carries.ein` | zero on either side, rows of zeros inside the multiplier, high zeros, one, a carry out of every row and none | a product that is not in normal form |
| `15_mul_long_and_named.ein` | 123078 × 123078, eleven digits; `mul` by `(let NAME NUMBER)`; `done`'s extent for the public products | an answer handed over twice, a request answered under another spelling |
| `16_div.ein` | division's whole chain, 127645 ÷ 25 = 5105 remainder 20: `div-ask`, `div-ask-second`, `div-start`, `div-multiples`, `div-multiple-2` to `-9`, `div-turn`, `div-begin`, `div-bring-first`, `div-bring`, `div-try-5`, `-7`, `-6`, `-2`, `-1`, `div-fits`, `div-too-big`, `div-digit-found`, `div-next`, `div-next-zero`, `div-next-none`, `div-end`, `div-answer` | a wrong digit, a remainder not less than the divisor, a high zero in the quotient |
| `17_div_every_digit.ein` | a quotient with every digit, 1234567890: all of `div-try-1` to `div-try-9`; and a divisor of three digits | a trial rule that reads the wrong multiple |
| `18_div_small_and_zeros.ein` | `div-bring-zero`; a dividend smaller than the divisor, equal numbers, a divisor of one, zero divided, high zeros on both sides, zeros inside the quotient | a quotient or a remainder that is not in normal form |
| `19_div_named_and_chained.ein` | `div` by `(let NAME NUMBER)`, and a `div` request a rule of the program derives from a `product-of` | a derived request left unanswered |
| `20_div_by_zero.ein` | `div-by-zero` and `division-by-zero-is-false`, firing: the divisor `z`; `(solve :expect (false))` | a division by zero answered, or passed over in silence |
| `21_div_by_zero_spelt_and_named.ein` | the same two, the zero written 00 and named by a `let` | a zero that is not seen because it is not written `z` |
| `22_malformed_digit.ein` | `std.math.decimal`'s `numeral-digit` and `not-decimal-is-false` since S1f.6.38 (`check-step`, `check-digit`, `bad-add-first`, `bad-request-malformed`, `malformed-is-false` until then): a name that is no digit in a digit's place, one place up; `(solve :expect (false))` | a malformed number passed over in silence |
| `23_malformed_fact_as_digit.ein` | `numeral-digit` on a fact (`check-digit` and `bad-add-second` until S1f.6.38) | a guard that holds of names only |
| `24_malformed_ends_in_digit.ein` | `numeral-prefix-digit` (`check-prefix-digit` and `bad-sub-first` until S1f.6.38): `(n 1 2)` | a number that ends in a digit read as a number |
| `25_malformed_ends_in_zero.ein` | `numeral-prefix-zero` (`check-prefix-zero` and `bad-sub-second` until S1f.6.38): `(n 0 2)` | the digit 0, which `nz` has no row for, missed |
| `26_malformed_ends_in_a_name.ein` | `numeral-prefix-named` (`check-prefix-named` and `bad-cmp-first` until S1f.6.38): `(n price 1)` | a named constant inside a number |
| `27_malformed_digit_as_number.ein` | `operand-digit` and `bad-cmp-second`: `(cmp X 7)` | a digit read as an operand that is merely never found |
| `28_malformed_zero_digit_as_number.ein` | `operand-digit-zero` and `bad-mul-first`: `(mul 0 X)` | the digit 0 as an operand |
| `29_undeclared_name.ein` | `operand-undeclared`, `bad-mul-second`, `bad-request-undeclared`, `undeclared-is-false`: a name no `let` states | an undeclared name passed over in silence |
| `30_declared_name.ein` | the same rules loaded and not firing: `29` with the `let` stated; the product answered, and `undeclared`'s and `malformed`'s extents empty | a flag left behind once the `let` is stated; a flag on a well-formed request |
| `31_malformed_through_a_name.ein` | `numeral-digit` on a `let`'s value, a request naming it and one nobody asks about, each flagged where it is written (until S1f.6.38 `operand-name` handed the first to the check, `bad-div-first`, and the second was not checked) | a malformed number that hides behind a name |
| `32_malformed_name_for_a_name.ein` | `let-to-name` (`check-prefix-named` on what a `let` gave, and `bad-div-second`, until S1f.6.38): `(let alias price)` | a name that stands for a name read as a number |
| `33_let_twice.ein` | `let-twice`: a name stated twice with two numbers | two answers to one request |
| `34_let_twice_the_same.ein` | `let-twice` loaded and not firing: the same `let` written twice is one fact | a flag on a fact stated twice |
| `35_let_zero.ein` | `let-zero`: `(let z N)` | `z` standing for itself and for a number |
| `36_let_digit.ein` | `let-digit`: `(let 7 N)`, with no request that uses it | a `let` flagged only when asked about |
| `37_let_zero_digit.ein` | `let-digit-zero`: `(let 0 N)` | the digit 0, which `nz` has no row for, missed |
| `38_what_stays_silent.ein` | no rule: what the theory does **not** flag, a number that ends in a name that is neither a digit nor a constant, a fact of another relation as an operand and inside a number, the accumulator's shape; beside an operand with high zeros, a foreign fact of `n` and `z` in a relation of the program's own, which are fine | a rule that starts flagging one of them, which makes the module's page wrong; a check that fires on a fact nobody asked about |
| `42_let_to_a_digit.ein` | `let-to-digit`: `(let seven 7)`, with no request that uses it (S1f.6.38) | a digit a `let` names read as a number, or flagged only when asked about |
| `43_let_to_the_digit_zero.ein` | `let-to-digit-zero`: `(let none 0)` | the digit 0, which `nz` has no row for, missed |
| `44_digit_as_the_first_operand_of_add.ein` to `50_digit_as_the_second_operand_of_div.ein` | `operand-digit` and, one a program, `bad-add-first`, `bad-add-second`, `bad-sub-first`, `bad-sub-second`, `bad-cmp-first`, `bad-div-first`, `bad-div-second` (S1f.6.38: the malformed numerals of `22` to `32` reached them until the numeral's check became `std.math.decimal`'s) | a rule from a tag to its request that reads the wrong tag |

**A division by zero gets the refutation idiom**, *one file where it fires and
one where it is loaded and satisfied*: `20` and `21` fire the two rules, and
`16` to `19` load them on divisors that are not zero and claim an R of one
state. A program that divides by zero can claim only `(false)`: root is dead,
so no query of it has an answer. `arith_search_free.rs` holds such a program
to an empty R decided by root alone, one saturation, the request in the core,
and `refuted by` naming the second rule; `arith_theory.rs` holds the flag to
the requests that divide by zero and to no other.

**A malformed request gets the same idiom** (M1f
[S1f.6.12](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.12_malformed_numbers.md)):
`22` to `29`, `31` to `33`, `35` to `37` and `42` to `50` each fire one fault,
`27` to `29` and `44` to `50` through one of the ten rules from an operand's
tag to its request, so that a mutant of any of them is seen, and claim
`(false)` (since S1f.6.38 `22` to `26` and `31` reach `std.math.decimal`'s
check of a numeral instead, where it is written); `30`, `34` and `38` load the same
rules where none fires and claim an R of one state with the flags' extents
empty. **One fault a program, and one malformed request**, because a dead
program's one claim cannot say which rule killed it: two faults in a file
would let either rule be deleted unseen. `arith_search_free.rs` holds such a
program as it holds a division by zero, with `refuted by` naming
`malformed-is-false` or `undeclared-is-false`, and holds that the flagged
request has no result fact; `arith_theory.rs` holds every kind of fault
under every operation and either operand to exactly one flag, and the list
of what stays silent.

`decimal-arith`, the closing rule, fires once at root in each. **The mutation sweep
kills every mutant of the module**, 21 of 21: the sweep reads 188 of 238 where
it read 167 of 217, and the bank's 50 survivors are the ones it had
(`python3 utils/stdlib_mutants.py --check`, 2026-11-11). `03_lengths.ein` is
written both ways round because of it: `(add ?x ?y)` with its operands
exchanged survives any program whose requests are symmetric.

**S1f.6.12's rules have twelve survivors, and each is banked with its
reason**: 58 of their 70 mutants are killed, 437 of 499 over the stdlib
(`python3 utils/stdlib_mutants.py`, 2026-11-13). A program the theory flags
is dead, and a dead program's one claim, `(false)`, cannot say *which* flag
killed it or how the flag is written. So `ein test` cannot see the ten
mutants that drop one of `operand-undeclared`'s `(neq ?x DIGIT)` guards,
under which a digit as an operand is flagged twice, `malformed` and
`undeclared`, and is dead either way; nor `bad-request-undeclared` with its
flag's two arguments exchanged; and `undeclared-is-false` with its premise's
two variables exchanged is the same rule. `arith_theory.rs` holds all of
them from outside the language: its harness compares the flags' text, and
asks that a digit as an operand has no `undeclared` flag.

**S1f.6.24's rule has one mutant, killed, and costs the bank one line**: 437
of 500 over the stdlib (`python3 utils/stdlib_mutants.py`, 2026-11-13).
`39_div_call_by_zero.ein` and `40_div_call_by_zero_spelt.ein` are a division
by zero that a rule of the program's own makes through the tagged call,
`(div-call (mine A B) A B)`, with no `div` request: they kill
`div-call-by-zero` with its premise's arguments exchanged, and
`41_div_call_not_zero.ein` is the same rules loaded and silent. The line the
bank gained is `div-by-zero` with its premise exchanged, which
`20_div_by_zero.ein` killed until now: a public division by zero has two
flags since the stage, the request's and its call's, either of which is
`(false)`, and a dead program's one claim cannot say which flag killed it.
`arith_search_free.rs` holds it from outside the language: a dead program
whose core names a `(div X Y)` has exactly two flags in root's state, each by
its own rule, and `arith_theory.rs` holds the two for every public request.

### `stdlib/math/decimal/`: a numeral checked where it is written

M1f [S1f.6.38](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.38_a_number_is_well_formed_from_the_start.md).
`std.math.decimal` declares the numeral `n` and checks every numeral a file
writes, off the `(written T)` facts the loader states, wherever it sits and
with no request reading it. Its five rules are activated by `22` to `26` of
`stdlib/math/decimal-arith/`, whose numerals are a request's operands; the
programs here are for **where it is written**, each one place, one fault:

| file | the rules it is there for | what would fail it |
|---|---|---|
| `01_well_formed_everywhere.ein` | the check loaded and silent: a well-formed numeral as an operand, in an unused `let`, in a fact of the program's own, in an integer and a rational, and in a rule's conclusion; R is {∅}, the sum answered and `not-decimal`'s extent empty | a well-formed numeral flagged; a place whose numerals are never stated |
| `02_in_a_let_nobody_asks_about.ein` | `numeral-digit` on a `let` no request names | a numeral checked only where a calculation reads it |
| `03_in_a_fact_of_the_programs_own.ein` | `numeral-prefix-digit` on a fact of the program's own relation, `(price apple (n 1 2))` | a numeral checked only in the calculator's own requests |
| `04_in_a_rules_conclusion.ein` | `numeral-prefix-zero` on a numeral written whole in a rule's conclusion, a rule that never fires: core ∅ (Q-M1f.275 (a)) | the loader stating facts but not the ground terms of a conclusion |
| `05_what_stays_silent.ein` | no rule: `(n end 1)`, `(n (pair 1 2) 3)`, a numeral a rule assembles from a variable, and a top-level fact of `n`, each silent beside a sum answered | a rule that starts flagging one of them, which makes the module's page wrong |

`std.math.arith` has two rules, `division-by-zero-is-false`, which
`20_div_by_zero.ein` and the other divisions by zero activate, and its closing
rule, which every program here fires: it has no program of its own.

**S1f.6.38's rules have 22 survivors, each banked with its reason**: 461 of
572 mutants killed over the stdlib, the 89 banked before and these
(`python3 utils/stdlib_mutants.py --check`, 2026-11-14). Every program a check
of a representation flags is dead, and a dead program's one claim, `(false)`,
cannot say how its flag is written or which of two flags killed it: so the
mutants that exchange a flag's arguments survive, in each of
`std.math.decimal`'s four rules, `std.math.int`'s six and `std.math.rat`'s six;
the three `…-is-false` rules with their premise's variables exchanged are the
same rule; `numeral-prefix-named` without its `(neq ?m z)` flags more, in a
program a `let` of `z` already kills; and `int-bad-div` either way, the
division's tag's own flag killing the program beside it. `arith_theory.rs`,
`int_theory.rs` and `rat_theory.rs` hold every flag's text from outside the
language.

### `stdlib/math/num9/`: wide rules, and a generated module

M1f [S1f.6.13](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.13_num9.md).
`std.math.num9` is the calculator's second module: nine digit names in one fact,
`(num9 0 0 0 1 2 3 0 7 8)`, and rules with a premise a column, so a sum is
one firing. What `stdlib/math/decimal-arith/` says of its programs holds of these: each
claims R = {∅} on its `(solve …)` form, `arith_search_free.rs` reads the
search-free claim off every one, and right is held from outside the language,
by `ein-infer/tests/num9_theory.rs` against Rust's integers, **exhaustively**
at width 2 on the generator's own output and on a sample at width 9. The
module is written by `utils/gen_num9.py`; a rule is changed there, and
`--check` in that test file fails on an edit to `stdlib/math/num9.ein` itself.

| file | the rules it is there for | what would fail it |
|---|---|---|
| `01_add.ein` | `add9-fits`: 123078 + 4567, and 99999999 + 1, a carry through eight columns | a column read out of order; the operands exchanged in the answer |
| `02_add_overflow.ein` | `add9-overflows`: 999999999 + 1 is a flag and no `sum9-of`; the sum beside it is answered | a truncated number; a flag that kills the program |
| `03_sub.ein` | `sub9-fits`, `sub9-short`: a difference, a shortfall, and equal operands, where both rules conclude one fact | a request answered with the other's fact; an equal pair answered twice |
| `04_cmp.ein` | `cmp9-columns`: `lt`, `gt`, `eq`, and a higher column overruling the units | the order of the lowest differing column |
| `05_mul.ein` | `mul9-start`, `mul9-step`, `mul9-step-zero`, `mul9-done`: 123 × 456, the largest square that fits, a multiplier with a zero inside; and no overflow at all | a zero digit that adds a row; a flag on a product that fits |
| `06_mul_overflow.ein` | `mul9-overflows-shift`, `-row`, `-add`, each by a product that falls off the top in that place and no other; the product beside them answered | an overflow missed, or a number stored for it |
| `07_div.ein` | `div9-start`, `div9-step-0`, `-1`, `-5`, `div9-done`: 127645 ÷ 25 = 5105 remainder 20 | a step that reads the wrong multiple |
| `08_div_every_digit.ein` | `div9-step-1` to `div9-step-9`, a quotient of 123456789; and a divisor of nine digits whose ninth multiple has ten | a multiple truncated to nine digits |
| `09_div_by_zero.ein` | `div9-by-zero`, and `std.math.decimal-arith`'s `division-by-zero-is-false` through this module's import: `(solve :expect (false))` | a division by zero answered, or passed over in silence |
| `10_to9.ein` | `to9-zero`, `to9-1` to `to9-9`, `to9-big`: `as9`'s and `big`'s whole extents | a number padded on the wrong side; ten digits read as nine |
| `11_from9.ein` | `from9-zero`, `from9-1` to `from9-9`: `of9`'s whole extent, a zero inside the number kept | an answer with a high zero |
| `12_both_modules.ein` | both modules flat in one program: one sum by each and the answers compared by `std.math.decimal-arith`, `eq` | a name of one module that is a name of the other |
| `13_chain.ein` | a `div9` request a rule of the program derives from a `product9-of` | a derived request left unanswered |

**No request here is another the other way round where it matters**, and
`03_sub.ein` says why in its header: the mutation sweep's first run left
`(sub9 X Y)` read with its operands exchanged alive in both subtractors,
since a program that asks X − Y and Y − X gets each one's fact from the
other's request and the two together look right. Two other mutants survived
that run because they changed nothing: a step counter's premise in two of the
overflow rules, which a digit that is not zero already implies. The premise
is gone from the rules. **The sweep kills every mutant of the module**, 41 of
41, and the bank is unmoved (`python3 utils/stdlib_mutants.py --check`,
2026-11-11).

### `stdlib/math/int/`: a theory over a theory

M1f [S1f.6.7](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.7_negative_numbers.md).
`std.math.int` is the calculator's first extension: an integer is `(int P N)`, two
natural numbers of `std.math.decimal-arith`, and every rule of the module has one premise
and starts one of `std.math.decimal-arith`'s walks. What `stdlib/math/decimal-arith/` says of its
programs holds of these: each claims R = {∅} on its `(solve …)` form,
`arith_search_free.rs` reads the search-free claim off every one, and right is
held from outside the language, by `ein-infer/tests/int_theory.rs` against
Rust's signed integers, every pair of integers written with one-digit parts
under every operation. Each claim here was computed by Python's integers when
the program was drafted.

| file | the rules it is there for | what would fail it |
|---|---|---|
| `01_simplify.ein` | `int-operand`, `int-simplify-ask`, `-start`, `-answer`, `int-norm-walks`, `int-norm-plus`, `int-norm-minus`: nine operands, 5 less 3, 3 less 5, equal parts, high zeros | the parts exchanged; a zero that is not `(int z z)`; an answer with a high zero |
| `02_neg.ein` | `int-neg-ask`, `-start`, `-answer`: seven operands, simplified and not | a negation that only exchanges the parts and does not simplify |
| `03_add.ein` | `int-operand-second`, `int-add-ask`, `-start`, `-answer`, `int-add-plus`, `-minus`, `-norm`: eleven sums in every combination of signs | a part added to the wrong side; the operands exchanged in the answer |
| `04_sub.ein` | `int-sub-ask`, `-start`, `-answer`: ten differences, each one integer | the second operand's parts not exchanged, which is a sum |
| `05_mul.ein` | `int-mul-ask`, `-start`, `-answer`, `int-mul-first` to `-fourth`, `-plus`, `-minus`, `-norm`: every combination of signs, zero on either side, operands not simplified | a cross product on the wrong side: the sign of every mixed product |
| `06_cmp.ein` | `int-cmp-ask`, `-start`, `-answer`, `int-cmp-left`, `-right`, `-sides`: twelve comparisons, two pairs written differently that are `eq` | the sides added straight and not across |
| `07_unsimplified.ein` | the stage's own case: 5 less 3 and 2 less 0 under each of the four operations | an operand read by its spelling |
| `08_named_and_chained.ein` | names through `std.math.decimal-arith`'s `let`; two rules of the program's own that read a negative difference by its shape, one deriving an `int-neg` request | a named integer left unanswered; a derived request left unanswered |

**No request here is another the other way round,** `stdlib/math/num9/`'s lesson,
kept from the first draft: `03_add.ein`'s header says why. **The mutation
sweep kills every mutant of the module**, 42 of 42, on its first run, and the
bank is unmoved (`python3 utils/stdlib_mutants.py --check`, 2026-11-12); 60
of 60 with S1f.6.22's division, 379 of 429 over the stdlib (`python3
utils/stdlib_mutants.py`, 2026-11-12).
**Integer division, under three conventions** (M1f
[S1f.6.22](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.22_integer_division.md)).
The three programs ask the same eighteen requests, so the three answers to
one request read side by side, and each claims its result relation's whole
extent; the three that divide by zero claim `(solve :expect (false))`, the one
claim such a program can make, and each writes its zero another way.

| file | the rules it is there for | what would fail it |
|---|---|---|
| `09_div_euclid.ein` | the shared `int-div-start`, `-divisor`, `-pp`, `-pn`, `-np`, `-nn`, `-rest`; `int-div-euclid-ask`, `-pp`, `-pn`, `-np-exact`, `-nn-exact`, `-np-more`, `-nn-more`, `-neg`, `-pos`, `-answer` | a remainder answered as an integer; a negative dividend's quotient not taken one further; the remainder left as the sizes' and not \|Y\| less it |
| `10_div_trunc.ein` | `int-div-trunc-ask`, `-pp`, `-pn`, `-np`, `-nn`, `-answer` | a sign put on the wrong one of the two; a quotient rounded down |
| `11_div_floor.ein` | `int-div-floor-ask`, `-pp`, `-nn`, `-pn-exact`, `-np-exact`, `-pn-more`, `-np-more`, `-neg`, `-pos`, `-answer` | a remainder with the dividend's sign; an exact division adjusted |
| `12_div_euclid_by_zero.ein` | `int-div-by-zero`, the divisor `(int z z)` | a division by zero answered with a number |
| `13_div_trunc_by_zero.ein` | `int-div-by-zero`, the divisor 3 less 3 | a zero read by its spelling |
| `14_div_floor_by_zero.ein` | `int-div-by-zero`, the divisor a name for 0 less 00 | a flag that does not repeat the request as written |

**What is malformed, and whose it is** (M1f
[S1f.6.38](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.38_a_number_is_well_formed_from_the_start.md),
E-38's 26 probes banked here and in `stdlib/math/rat/`). Each program is one
fault and claims `(false)`, but the silent one, which claims R = {∅} and
answers a well-formed sum beside its request:

| file | the rules it is there for | what would fail it |
|---|---|---|
| `15` to `19`, `24`, `30` | `std.math.decimal`'s check of a numeral in a part, in either part, either operand, behind a name and in a division: the numeral's fault is decimal's, flagged where it is written | a numeral inside an integer never checked |
| `20_undeclared_first_operand.ein`, `21_undeclared_second_operand.ein`, `36_undeclared_only_operand.ein` | `int-bad-first`, `int-bad-second`, `int-bad-only`: an undeclared name, flagged on the request | an operand's fault seen under the module's tag and not flagged |
| `22_digit_as_an_operand.ein` | `int-bad-first` on a digit as the operand | a digit as an integer operand passed over |
| `35_undeclared_divisor.ein` | `int-bad-first` and `int-bad-div`: a division's tag holds `(int-div-of REQUEST)`, and the request as written is flagged beside it | a division's flag that names no request |
| `23`, `29`, `31` to `34` | `int-p-digit`, `-named`, `int-n-digit`, `int-p-zero`, `int-n-zero`, `int-n-named`, and `not-integer-is-false`: a digit or a named constant where a part goes (Q-M1f.276 (a)) | an integer's own malformation read as an integer |
| `25` to `27` | `let-twice`, `let-zero`, `let-digit` on a `let` of an integer, as before S1f.6.38 | a `let` of an integer not read |
| `28_natural_where_an_integer_goes.ein` | nothing: a natural number for an integer is silent (Q-M1f.277 (b)), and the request is not answered | a rule that starts flagging another type, which makes the module's page wrong |

### `stdlib/math/rat/`: lowest terms, and four ways to divide by zero

M1f [S1f.6.8](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.8_rational_numbers.md).
`std.math.rat` is the calculator's second extension: a rational is `(rat Z D)`, an
integer of `std.math.int` over a natural number, answered in lowest terms. What
`stdlib/math/int/` says of its programs holds of these: every rule of the module
has one premise, nine of the thirteen claim R = {∅}, and right is held from
outside the language, by `ein-infer/tests/rat_theory.rs` against exact
fractions. Each claim here was computed by Python's fractions when the
program was drafted.

| file | the rules it is there for | what would fail it |
|---|---|---|
| `01_simplify.ein` | `rat-operand`, `rat-operand-check`, `-found`, `rat-simplify-ask`, `-start`, `-answer`, `rat-norm-numerator`, `-reduce`, `rat-reduce-pos`, `-neg`, `rat-gcd-step`, `-next`, `-end`, `rat-lowest-numerator`, `-denominator`, `-pos`, `-neg`: eleven operands, 1071/462 = 51/22 by a chain of three divisions | a common divisor that is not the greatest; a zero that is not 0/1; a sign lost in the reduction |
| `02_neg.ein` | `rat-neg-ask`, `-start`, `-answer` | a negation not put in lowest terms |
| `03_add.ein` | `rat-operand-second`, `rat-add-ask`, `-start`, `-answer`, the seven rules of the cross products, `rat-sum-denominator`, `rat-sum-norm`: eight sums in every combination of signs | a cross product with the wrong denominator; a part on the wrong side |
| `04_sub.ein` | `rat-sub-ask`, `-start`, `-answer`: seven differences | the second numerator's parts not exchanged, which is a sum |
| `05_mul.ein` | `rat-mul-ask`, `-start`, `-answer`, `rat-mul-numerator`, `-denominator`, `-reduce`: seven products | a product not reduced; a sign |
| `06_div.ein` | `rat-div-ask`, `-start`, `-answer`, `rat-div-pos`, `rat-div-neg`: six quotients | the divisor not turned over; its sign lost |
| `07_cmp.ein` | `rat-cmp-ask`, `-start`, `-answer`, `rat-cmp-sides`: ten comparisons, 22/7 against 355/113 | the denominators ignored; the order of a negative pair |
| `08_unsimplified.ein` | the stage's own case: 2/4 and 1/2 under each of the five operations | an operand read by its spelling |
| `09_named_and_chained.ein` | names through `std.math.decimal-arith`'s `let`; two rules of the program's own, each deriving a request from an answer | a named rational left unanswered; a derived request left unanswered |
| `10_div_by_zero_rational.ein` | `rat-div-zero`: a divisor that is 0 over 5; `(solve :expect (false))` | a division by zero answered, or passed over in silence |
| `11_zero_denominator_first.ein` | `rat-operand-zero`, `rat-zero-first` | a rational over zero computed with |
| `12_zero_denominator_second.ein` | `rat-zero-second`: the operand named, the zero written `(n (n z 0) 0)` | a zero read by its spelling |
| `13_zero_denominator_one_operand.ein` | `rat-zero-only`: `(rat-neg X)` with X over nothing | a request of one operand that is not checked |

**Four programs are dead on purpose**, one a rule that derives the flag. Each
states a sum beside the request that kills it, and claims only `(solve
:expect (false))`: root's own fixpoint derives `(false)`, R is empty, and
`arith_search_free.rs` holds each to that, the request named in the core and
`std.math.decimal-arith`'s rule under `refuted by`. A core names the request, a `let`
where the operand is named, and the rows of `cl` a denominator was compared
with zero through.

**The mutation sweep kills every mutant of the module**, 44 of 44, on its first
run, and the bank is unmoved (`python3 utils/stdlib_mutants.py --check`,
2026-11-12). No request here is another the other way round where both would
be answered alike, `stdlib/math/num9/`'s lesson.

**What is malformed, and whose it is** (M1f S1f.6.38), as in `stdlib/math/int/`:
`14` to `16` and `20` a malformed numeral, `std.math.decimal`'s; `17`, `18` and
`30` an undeclared name, `19` a digit, through `rat-bad-first`, `-second` and
`-only`; `21` a `let` stated twice; `24` to `29` the rational's own shape,
`rat-z-digit`, `-zero`, `-named`, `rat-d-digit`, `-zero`, `-named` and
`not-rational-is-false`; and `22`, `23`, an integer and a natural number for a
rational, silent (Q-M1f.277 (b)).

### `stdlib/bool/`: the sixteen functions, by rules, and a circuit

M1f [S1f.6.17](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.17_binary_logic.md).
Four modules, a directory each. **Eight of the twelve programs are
generated**, by `utils/gen_gates.py` from the sixteen columns, whose
`--check` is in the gate: `gates/01` claims each of the sixteen relations of
the second layout whole, and `props/01` to `07` claim every relation of
`std.bool.props`, a family a program, each relation's whole extent computed in
Python. A library with no refutation rule has its negative case in that
idiom alone: a claim that names a relation closes it, so a rule that derived
one fact too many, or one too few, fails the file. Right is held from outside
the language too, by `ein-infer/tests/bool_theory.rs`, against each
property's definition in Rust. A generated program's R is {∅}; the four
written by hand state theirs.

| file | the rules it is there for | what would fail it |
|---|---|---|
| `gates/01_a_relation_a_function.ein` | the sixteen `Zero-rows` to `One-rows`, and `gates` | a relation with a row its function does not have, or without one it has |
| `props/01_column.ein` | `column-of` | a column read in the wrong order |
| `props/02_between_two.ein` | `swap`, `complement`, `dual`, `left-neg`, `right-neg` | a function paired with the wrong one; a dual written as its complement, the stage file's hand count |
| `props/03_commutative_associative_idempotent.ein` | `commutative-r`, `assoc-point`, `associative-r`, `idempotent-r` | a function associative at seven points of eight |
| `props/04_reads_and_ignores.ein` | `reads-first-r`, `reads-second-r`, `ignores-first-r`, `ignores-second-r` | an input exchanged for the other |
| `props/05_identity_and_absorbing.ein` | the one-sided and two-sided identity and absorbing rules, six | a side exchanged: `Andn` has a right identity and no left one |
| `props/06_post_inside.ein` | `preserves-zero-r`, `preserves-one-r`, `monotone-r`, `affine-r`, `self-dual-r` | an order of the bits read backwards |
| `props/07_post_outside.ein` | the five `escapes-` rules and `complete-alone-r` | a witness pair read backwards |
| `circuit/01_propagate.ein` | `propagate`, `circuit`; `one-value` loaded and silent | a gate's inputs exchanged (`Rorn`'s are read); a gate with an input unset computing |
| `circuit/02_one_value.ein` | `one-value`: a gate's output against a written value, `(solve :expect (false))` | two values on a wire passed over |
| `inputs/01_in_drives.ein` | `in-drives`, `inputs`, and `std.algebra`'s `total-owed` and `functional`; `required` loaded and silent | R not the four assignments; the truth table's rows |
| `inputs/02_required.ein` | `required`: two requirements, one assignment of four left | a requirement read the wrong way round |

**The mutation sweep kills 27 of the modules' 45 mutants, and the 18 that
survive are equivalent to their rules**, each deriving the very facts the rule
does, so no claim made of facts can tell them apart and none is a hole
(`python3 utils/stdlib_mutants.py`, 2026-11-14, banked in
[`mutation_survivors.txt`](mutation_survivors.txt)):

- **thirteen exchange the arguments of an `(inv …)` premise**, in `complement`,
  `dual`, `reads-first-r`, `reads-second-r`, `escapes-affine-r` and
  `escapes-self-dual-r`: `inv` is its own converse, `(inv 0 1) (inv 1 0)`, so the
  premise matches the same pairs either way round;
- **five exchange the arguments of the conclusion** of `swap`, `complement`,
  `dual`, `left-neg` and `right-neg`: each relation is an involution, G is F's
  swap exactly when F is G's, so the rule's matches come in mirrored pairs and
  the conclusion written the other way round writes the same facts.

Every mutant of `std.bool.circuit` and `std.bool.inputs` is killed, as is every
one that exchanges an `le`, a one-sided relation or a value of `val`.

### Five things about how they are written

**Naming a relation closes it.** An expectation lists a relation's *complete*
extent, so a rule that derived one fact too many fails the file. That is what
makes a "must not fire" case checkable at all for the 33 rules that carry no
guard: their only *must not* is scope, and scope is a claim about a whole
relation. Where a rule needs a relation to stay untouched, the fixture gives it
one authored edge and closes it there —
[Q-M1c.6](../docs/history/m1c_external_validation/open_questions.md#q-m1c6--how-does-an-expectation-say-a-relation-is-empty)
is why an *empty* relation cannot be named.

**`(unknown …)` is how a fixture says a negative was *not* invented.** Stored
negatives are deliberately not closed by an expectation, so listing the
exclusions that exist says nothing about the ones that do not. Ten programs
carry a four-line `probe-undecided` rule whose body is `std.macro`'s `(unknown
P)` — true exactly when a fact is in neither store — turning the absence into
an ordinary positive fact the expectation *can* close. It reads under an
`absent`, so the boundary judges it once the closure is at its fixpoint, which
is what a probe that reads the world needs: to be the last thing to read it.
(Until M1f S1f.5.15 removed `:priority` it also carried priority 500, above
everything the stdlib declared.)

**A refutation rule gets two files.** "It fires and the answer is ⊥" is one
test and it is not the one that finds bugs; the other is a program where the
rule is loaded, activated and *satisfied*, so that a guard admitting too much
turns an ordinary model into a contradiction. `algebra/08_checks_satisfied.ein`
is that file for seven rules at once, and each of the seven has a `_violated`
sibling.

**Where two rules reach one verdict, separate them by activation.** An
expectation is made of facts and cannot say which rule produced them — the
`route` residue
[Q-M1c.2](../docs/history/m1c_external_validation/open_questions.md#q-m1c2--what-may-an-expectation-say)
parks. On a fully excluded row `no-room-left` refutes and `domain-elimination`
forces a value against a stored negative, and both end in ⊥; the fixture picks
one by declaring only its activator. Five files do this — `elim/02`,
`elim/03`, `elim/04`, `slots/05`, `slots/06` — and each says so in its header,
because a reader who does not know why `slot-partition-setup` is absent will
put it back.

### What a mutation sweep says about them

**A verdict word is not what an expectation claims.** All three `:expect`
forms are assertions about **facts** (`01_grammar.md` § Query), so no fixture
here says `Solution`, `Open` or `Contradiction` — it says what the answer's
fact set is. M1d S1d.2.6 is the proof that this was the right shape: twelve of
these programs changed verdict word that day, from `Solution` to `Open`, and
**not one `(model …)` claim moved**, because the facts an open state reached
are the facts it reached. What a fixture cannot state it asserts in-process
instead — the owe counts in
[`obligation_reports.rs`](../ein.rs/crates/ein-infer/tests/obligation_reports.rs),
the rung modes in `obligation_rung.rs` — which is S1c.1.5's shape and the
reason growing `:expect` a word for the verdict was deferred rather than
needed. **Since M1e2's switch one form can say that a state owes**, though not
how much: `(solve … :expect (region (owing (model …))))` names an owing state,
which R, the states that are alive *and discharged*, does not hold
([`solution_semantics.md`](../docs/kernel/inference/solution_semantics.md)).
The owe counts are still `obligation_reports.rs`'s. Eight mutants died of that
word (below). **A list may not switch an obligation's generating role off**
since M1f S1f.6.34 (*"filtered obligations are nonsense by definition"*, the
user, 2026-11-13): a `:no-hypothesis` naming a relation an obligation owes is
refused at load, and so is a closure of one. **To judge root alone, write
`(solve :no-search)`** (M1f S1f.6.36, the user's *"some explicit flag in (solve
) ? instead of abusing bugs"*): the lattice is root alone, nothing is proposed
whatever the program declares, and the obligations judge root, so R is root
where it owes nothing and empty, root the owing node it stops at, where it
owes. That is what the fixtures whose question is root's verdict or root's
saturation say: `algebra/23` to `26`, `bijection/01` to `03`, `slots/01` to
`04`, `07` to `13`, `region/04_owing` and `05_empty`. Between S1f.6.34 and
S1f.6.36 they said it with workarounds, an hrule `as-stated` proposing only what
root holds, `slot-partition`'s fan-out written out without its obligations,
negatives that made a debt unpayable or R one state, or another question
(`algebra/23`, `25` let the obligation propose); `:no-search` replaced every one,
and `utils/stdlib_mutants.py`'s survivors are the banked set with it. An hrule
or a list beside `:no-search` proposes or filters nothing, and says so in a
`note:`; `ein test --no-search`, a flag of the runner, is another thing, a run
that declines to search R and checks only what root decides.

**157 of 217 on 2026-08-31, and it is an instrument now** —
[`utils/stdlib_mutants.py`](../utils/stdlib_mutants.py), M1e S1e.3.6 T6, the
review's `TE-M6`. The sweep began as **50 of 51**, hand-taken: one deliberate
defect per rule family — a dropped `neq`, an exchanged pair of `absent`
operands, a `forall` over the wrong type, a fan-out missing an activator —
injected into a copy of `stdlib/` and run past `ein test tests/`. Every one of
the five separations above was added *because* a mutant survived without it,
so the sweep is not decoration: it is what built this directory.

The two numbers are different measurements. The script is **exhaustive** over
four mechanical families where the hand sweep was one per family, and it runs
in 7 s, so it can be re-taken rather than remembered. The survivors are banked
as a *set* in [`mutation_survivors.txt`](mutation_survivors.txt) — a score
decays while the thing it guards grows, which is the same finding as `TE-M2`'s
floors — and `--check` fails on a new one **and** on a banked one that has since
been killed.

**The recorded survivor is dead.** `slot-adjacent-bwd-neg` with its two
structure operands exchanged survived from M1c to M1e: on the three-seat row of
`slots/08_spatial_adjacent.ein` the exclusion it should derive is reachable
from the other clue's chain a pass later.
[`slots/13_adjacent_bwd_neg_direction.ein`](stdlib/slots/13_adjacent_bwd_neg_direction.ein)
is the four-seat row that isolates it, and it killed two of its siblings on the
way. What it needed was the observation that the exchanged rule is **sound and
strictly weaker** — its guard becomes satisfiable only where the conclusion is
already true for another reason — so no contradiction fixture could ever have
caught it, and only a program that *needs* the derivation can.

**165 of 217 on 2026-11-03: the region claim killed eight**
(`python3 utils/stdlib_mutants.py`, M1e2
[S1e2.3.6](../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.6_the_nightly_workflow.md)).
The nightly found them two weeks after they died: its `--check` failed on
2026-11-02 and 11-03 with eight banked survivors now killed and none new, and
from 10-17 to 11-01 a step before it had failed first. All eight are mutants of
the four obligation rules, and they had survived for the reason the first
paragraph of this section gives: no fixture could say what a state owes, so a
mutant that changed only *whether* a debt is owed left every fact where it was.
The region claim says it. A mutant that stops a debt being owed puts the state
in R where the claim says R is empty, and one that invents a debt takes a
discharged state out of R where the claim lists it. The claim's fixtures came
in two commits, and the kills with them:

| survivor until 2026-10-16 | killed by | what the run says | since |
|---|---|---|---|
| `algebra.ein::total-owed::swap-premise::(?isa ?a ?A)` | `region/04_owing`, `region/05_empty`, `region/07_cube`; since the switch also `stdlib/algebra/23_total_owed` and `stdlib/closure/03_closed_and_owing` | *the claim says R is empty, and R has 1 state*; on `07_cube`, *the claim lists 3 states and R has 4* | `0f9a6656`, T1e2.1a.4.2: the region claim, with its fixtures under `region/` |
| `algebra.ein::surjective-owed::swap-premise::(?isa ?b ?B)` | `stdlib/algebra/25_surjective_owed` | *the claim says R is empty, and R has 1 state* | `fbcae3b2`, S1e2.1a.5's switch, which moved every fixture's claim about a state to `(solve … :expect (region …))` |
| `slots.ein::slot-owed-room::swap-premise::(?isa ?a ?Ta)` | `stdlib/slots/09_owed_room` | the same | `fbcae3b2` |
| `slots.ein::slot-owed-room::swap-premise::(?sub ?Ta ?super)` | `stdlib/slots/09_owed_room` | the same | `fbcae3b2` |
| `slots.ein::slot-owed-fill::swap-premise::(?isa ?i ?index)` | `stdlib/slots/11_owed_fill` | the same | `fbcae3b2` |
| `slots.ein::slot-owed-room::drop-neq::(neq ?Ta ?index)` | `stdlib/slots/01_partition`, `08_spatial_adjacent`, `10_owed_room_satisfied` | *the claim lists 1 state and R is empty*: a seat now owes a seat | `fbcae3b2` |
| `slots.ein::slot-owed-fill::drop-neq::(neq ?Tv ?index)` | `stdlib/slots/01_partition`, `08_spatial_adjacent`, `12_owed_fill_satisfied` | the same, on the fill side | `fbcae3b2` |
| `slots.ein::slot-owed-fill::swap-premise::(?sub ?Tv ?super)` | the same three | *the claim lists 1 state and R is empty* | `fbcae3b2` |

The *since* column is measured, not read off the history: each commit's own
`ein`, `tests/` and `stdlib/`, the mutant applied, `ein test tests/` run. All
eight survive at `9c2815e0`, the commit before the region claim; at
`0f9a6656` and at `d09ba6a7`, the switch's parent, `total-owed`'s alone dies;
at `fbcae3b2` all eight do, killed by the files above (S1e2.3.6, 2026-11-03).
`stdlib/` has not changed since the last green nightly (`git diff 3fde4ef1
HEAD -- stdlib/` is empty), so the kills are the suite's and the engine's.
**The bank's diff is those eight lines, removed, and nothing added**: 60 → 52,
re-banked by `--bless` once each line was explained here.

**167 of 217 on 2026-11-08: *forbid* killed two**
(`python3 utils/stdlib_mutants.py`, M1f
[S1f.5.17](../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.17_ci_green_again.md)).
The nightly's `--check` failed on them on 2026-11-06 and 11-07, its first two
runs after they died, and on this machine too, so the platform is not what
killed them:

| survivor until 2026-11-04 | killed by | what the run says | since |
|---|---|---|---|
| `bijection.ein::bijective-setup::swap-assert::(total ?R ?isa)` | `stdlib/bijection/06_blind_enumeration` | refused at load, exit 2: *rule 'total' [activator is-a seats]: concludes (false) from (absent …) over seats, a relation a commitment can still extend* | `597b2b7e`, T1e2.2.5.2: *forbid*'s refusal |
| `bijection.ein::bijective-setup::swap-assert::(surjective ?R ?isa)` | the same | the same, of *rule 'surjective'* | `597b2b7e` |

The swap turns the activator `(total ?R ?isa)` into `(total is-a seats)`, so
`std.algebra`'s `total` reads `seats` under `absent`. `06_blind_enumeration` is
the one program in `stdlib/bijection/` that leaves the blind enumerator on, so
`seats` is a relation a commitment can extend, and *forbid* refuses the
program. A refusal is a kill: the script counts any non-zero exit of `ein test
tests/`. The *since* column is measured as the table above measures it: `git
bisect run` over `3ee7796a..916d97ac` names `597b2b7e`. At its parent
`c887a35b` both mutants pass (66 held, exit 0), and at `597b2b7e` both are
refused. `stdlib/` and `tests/` are unchanged since `3ee7796a`, the last green
nightly's commit (S1f.5.17, 2026-11-08). **The bank's diff is those two lines,
removed, and nothing added**: 52 → 50.

**188 of 238 on 2026-11-11: `std.math.decimal-arith` added 21 mutants and the suite kills
every one** (`python3 utils/stdlib_mutants.py`, M1f
[S1f.6.3](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.3_addition.md)).
The module's rules have no `neq` and no `absent`, so its mutants are the two
exchange families: a premise's two arguments, a tag and a number, and a
conclusion's. Exchanged, a step hands its answer to nobody, the chain stops,
and the program's `sum-of` claim fails for want of a row; the one mutant
that still answers, `(add ?x ?y)` read as `(add ?y ?x)`, files the sum under
the operands exchanged, which `arith/01_add.ein` refuses. `std.math.digits` has no
rule and adds none. **The bank is unmoved**, 50 lines.

**203 of 253 on 2026-11-11: subtraction and comparison added 15 mutants and
the suite kills every one** (`python3 utils/stdlib_mutants.py`, M1f
[S1f.6.4](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.4_subtraction.md)).
They are the exchange families again. The ones that still answer file a
difference or an order under the operands exchanged, which `arith/08_sub.ein`
and `11_cmp.ein` refuse, each asking an unequal pair one way round or both
with different answers. **The bank is unmoved**, 50 lines.

**216 of 266 on 2026-11-11: multiplication added 13 mutants and the suite
kills every one** (`python3 utils/stdlib_mutants.py`, M1f
[S1f.6.5](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.5_multiplication.md)).
A product is symmetric in its operands, so `13_mul.ein` asks 123 × 456 both
ways round and claims each under its own operands: `(mul ?x ?y)` read as
`(mul ?y ?x)` files both under the wrong ones. **The bank is unmoved**, 50
lines.

**234 of 284 on 2026-11-11: division added 18 mutants and the suite kills
every one** (`python3 utils/stdlib_mutants.py`, M1f
[S1f.6.6](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.6_division.md)).
**The bank is unmoved**, 50 lines: `std.math.decimal-arith`'s 89 rules have 67 mutants and
no survivor.

**275 of 325 on 2026-11-11: `std.math.num9` added 41 mutants and the suite kills
every one** (`python3 utils/stdlib_mutants.py`, M1f
[S1f.6.13](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.13_num9.md)).
Its rules write no guard, so its mutants are the exchange families: a
request's two operands, the two arguments of a step counter's row, a
conversion's two. Its first run read 273 of 327, four survivors: two were
equivalent mutants of a premise two rules did not need, which the generator
no longer writes, and two were `(sub9 X Y)` with its operands exchanged, which
`num9/03_sub.ein` kills since it stopped asking a pair both ways round. **The
bank is unmoved**, 50 lines.

**317 of 367 on 2026-11-12: `std.math.int` added 42 mutants and the suite kills
every one**, on its first run (`python3 utils/stdlib_mutants.py`, M1f
[S1f.6.7](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.7_negative_numbers.md)).
Every rule of the module has one premise and no guard, so its mutants are the
exchange families: a request's two operands, a hand-over's tag and answer.
**The bank is unmoved**, 50 lines.

**361 of 411 on 2026-11-12: `std.math.rat` added 44 mutants and the suite kills
every one**, on its first run (`python3 utils/stdlib_mutants.py`, M1f
[S1f.6.8](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.8_rational_numbers.md)).
As `std.math.int`'s, they are the exchange families. **The bank is unmoved**, 50
lines.

**431 of 502 on 2026-11-14: one killed, eleven masked by whole modules**
(`python3 utils/stdlib_mutants.py --bless`, M1f
[S1f.6.35](../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.35_imports_by_full_name.md),
which banked them). **Killed**: `converse-illtyped-dom` without its `neq`, by
`stdlib/algebra/27_converse_welltyped_either_way.ein`, W93's fixture, which its
two commits added without re-banking (*the claim lists 1 state and R is
empty*; the binary and stdlib of `dc94b947` kill it alike). **New, eleven,
each a mutant whose loss another rule of the same module makes good**: since
S1f.6.35 every import brings its module whole, so a program that lists
`symmetric` or `converse` holds `std.algebra`'s lemmas too, and with them
`(converse R1 R2)` holds both ways (`converse-pair-symmetric`) and a symmetric
relation is its own converse (`symmetric-is-self-converse`). A `(converse …)`
premise read the other way round — `converse-illtyped-dom`, `-ran`,
`compose-contravariant`'s three, `join-converse`'s three, `std.typing`'s
`type-hierarchy-converse` — matches the mirror the lemma derives, and
`symmetric` with its premise or its conclusion exchanged leaves the mirror to
`converse`, through the self-converse lemma. The facts are those of the
unmutated module, so no claim made of facts can tell them apart, and no
program can now load one of those rules without its lemma: they are banked,
71 lines, and E-56 of P1f.6's findings says so.

**Where the remaining 50 are.** Forty-two in `slots.ein`, which has the most
parameterised, direction-sensitive rules and the fewest programs per rule; the
rest split between `algebra` (4), `bijection` (2) and `elim` (2)
(`grep -c '^slots' tests/mutation_survivors.txt`, 2026-11-08). That distribution is the
useful part of the number, and it is what a score alone never said.

### What holds this directory up

Two claims about it are **in `cargo test`**, since M1c
[S1c.1.5](../docs/history/m1c_external_validation/README.md#s1c15--in-the-gate)
— `ein-infer/tests/stdlib_coverage.rs`, 0.04 s for the whole sweep:

| the claim | what fails without it |
|---|---|
| **every stdlib rule is activated by a program *here*** | a rule added to `stdlib/` with no test. It is named, with its module, so the failure says which directory the missing file goes in |
| **every program here states an expectation** | a fixture whose `:expect` was lost in a refactor, which would then load, run and pass forever |

The first is deliberately scoped to this directory and not to the corpus. The
corpus-wide version is weaker in exactly the way that matters: a rule that
happened to fire somewhere inside `examples/zebra.ein` would pass it with **no
test written**, which is the state
[the census](../docs/history/m1c_external_validation/stdlib_census.md)
found 20 rules in. Scoping it also found the one rule this suite did not run —
`transitive`, whose fixture was a two-cycle where the `(neq ?a ?c)` guard
refuses every match. `algebra/21_transitive.ein` grew a three-chain, and the
suite has stood on its own since — **73 of 73 with no `examples/` entry
contributing** when S1c.1.4 closed on 2026-08-24, and *all of them* today,
which is not a number this page has to keep because
`stdlib_coverage.rs::every_stdlib_rule_is_activated_by_a_program` fails
the moment it stops being true.

The third file every entry here is swept by is `corpus/corpus.toml` — five runs
each but one, so this directory's share of the sweep is
`grep -c 'path *= *"tests/stdlib' corpus/corpus.toml` entries out of
`grep -c '^\[\[entry\]\]' corpus/corpus.toml`, and `wc -l
ein.rs/crates/ein-cli/tests/golden/corpus_exits.txt` is the whole cell count.
It cost **0.72 s** of the sweep's 5.1 s on 2026-08-24.

### Re-measuring the coverage

The gate is a yes/no. The *numbers* — firings per rule, productive vs
redundant, who the sole activator is — stay with the instrument:

```sh
python3 utils/stdlib_census.py                     # the table, every corpus entry
python3 utils/stdlib_census.py -k tests/stdlib     # this directory's own contribution
python3 utils/stdlib_census.py --check             # exit 1 while any rule is at zero
```

It parses every `.ein` under `stdlib/`, at any depth (`stdlib/math/` since
M1f S1f.6.31), for rule heads, then sweeps every corpus entry under
every declared `solve` / `saturate` / `test` run with `--events` and counts
`fire` by rule. A firing counts for a module only when the file does not
declare that name itself and the module is in its import closure, so a puzzle's
inline `symmetric` is never credited to `std.algebra`'s. That attribution rule
is the one the cargo test re-implements, and the two have to stay one rule.

## What may be claimed here — M1d S1d.4.3

**A claim that cannot be checked at the runner's own budget does not go in.**
`ein test` asks the question a file's claims are, at the default
`--region-budget` since M1e2 S1e2.1a.5 (the walk's `--max-set-size 5` before).
Since M1f S1f.6.27 that is the least question that can decide them, `--single`
or `-n k+1` for queries' claims over one part of R and the description for a
region claim, and then the description for whatever that left undecided
([`01_grammar.md` § Query](../docs/kernel/ir/03-ein-lang/01_grammar.md#query)).
So a query's claim on a search too large to describe can go in where the
question decides it, which is what `examples/beyond_arith/send_more_money.ein`
is; a region claim on such a program still cannot. Where both questions stop
short, `:expect` reports
[`NOT CHECKED`](../docs/kernel/defined_behaviour.md) — which is honest, is not
a pass, and **takes a failing exit code**. So the policy needs no mechanism: a
claim that does not check cannot be added without turning the gate red. It is
the failure mode of adding one, not a convention to remember.

Its cost today is **zero entries** — 59 corpus claims, 59 held, all 59 under a
search that exhausted, and 58 of them entering no commitment at all
([`closure_census.md`](../docs/history/m1d_satisfiability/closure_census.md)).
Its forward cost was the **ten** corpus entries whose claim *would* have come
back unchecked under the walk — `saturation/type-exclusivity/pets.ein` most
sharply, which the walk called `Contradiction k = 0` at `-m 5` and which has
**35 models** at `-m 10`. Under R the set is the entries a describe does not
exhaust at the default budget — the zebra family among them, and `pets.ein`
is not: its R, 11 339 states, is exhausted. Making those affordable is
P1e2.1c's speed, not this directory's.

**The corollary for a new fixture**: keep it small enough that its answer is
reached by saturation or by a couple of layers. That is what every program here
already is, and it is why the whole suite is 0.04 s.
