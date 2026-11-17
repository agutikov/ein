# tests/

**Ein programs whose job is to fail when the engine is wrong.**

Everything here is an `.ein` file carrying an `:expect` (a query's, or the
region claim on `(solve …)`), so it states its own answer and `ein test` turns
the directory into a status code:

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
(the programs under `stdlib/` in group `stdlib`, five runs each but one) and
`ein-corpus`'s completeness check walks this directory beside the other two,
so a test file with no entry fails `cargo test` — the sweep would otherwise
never run it, which reads as coverage nobody has.

## `region/` — the region claim, one program per form

Each program states its expected region on `(solve … :expect …)`, or a query's
classes over a part of R, and `ein test` checks it against R as `ein solve`
describes it: by the routes on a monotone program, by the cube on one the
static check flags ([finding R](../docs/ein/reasoning/kb-evolution/finding-r.md)).
The claims were rendered from each program's computed R, the way
[`query.md`](../docs/ein/user/syntax-and-semantics/query.md#claims)'s
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
| `08_query_classes.ein` | a query's claim: the goal's classes over all of R, `(model)` among them, on `01_states.ein`'s program |
| `09_query_over_maximal.ein` | a query's claim `:over maximal`: the seatings nothing can be added to, where over all of R the same goal has a class per state |
| `10_denied_extensions.ein` | a query's classes over R, on a bijection whose rules store what a seating excludes: the fixture of *what a state denies is not saturated* (`ein-infer/tests/known_dead.rs`) |

Every claim here holds. Their false twins, and the claims a route cannot
check, are `ein-infer/tests/claim_semantics.rs`. Corpus group `positive`,
runs `test` and `saturate`.

## `text/` — natural-language text

A string is natural-language text, written only after `:why`, `:source` and
`:goal-text`: any Unicode text, with the escapes `\n \t \r \" \\` and `\u{…}`
([Q-M1e2.23](../docs/decisions/language/Q-M1e2.23-a-string-is-natural-language-text-never-a-value.md)).
`01_unicode.ein` writes each of the three with Cyrillic, CJK, an emoji and
every escape a string reads, and one `:source` with a raw newline; its claim is
about facts, which the text does not move. `ein-cli/tests/natural_language_text.rs`
holds the text byte for byte on every surface that carries it: the read-out and
`answer.txt`, `--trace`, `ein saturate --dump`, a `--dump-answer` state file,
`ein render program` and a `.einb`. Corpus group `positive`, runs `test`,
`solve` and `saturate`.

## `imports/` — a theory built on a theory

Five files: two theories, `lower.ein` and `upper.ein` over it, each with an
entry rule; `upper_without_entry.ein`, the upper one with the lower theory's
entry name left out of its own; and two programs that import the upper theory
by its entry. `01_nested_import_whole` gets both theories and its request
answered. `02_nested_import_left_behind` imports `upper_without_entry`, whose
entry does not name the lower theory's, and is answered as `01` is,
`(got a aa)`: every import brings its module whole, the upper module's own
import of the lower one among them, so nothing is left behind and no line is
printed
([Q-M1f.247](../docs/decisions/imports/Q-M1f.247-every-import-brings-its-module-whole.md);
`ein-cli/tests/imports_by_full_name.rs`). Corpus group `positive`.

## `stdlib/` — the stdlib conformance corpus

One program per stdlib rule or tight family, each the smallest thing that
activates it and each stating what it should and should not derive. **The
directories mirror the library**: a module `std.x` has its programs under
`stdlib/x/`, `std.math.x`, the calculator's, under `stdlib/math/x/`, and
`std.bool.x`'s under `stdlib/bool/x/`.

**Why they exist.** Every other check in this repository is *relative*: the
goldens compare ein.rs to its own past. A rule that fires only inside
`examples/zebra.ein`, or in no corpus program at all, has no test of its own;
these programs give each rule one.

| dir | what the programs do |
|---|---|
| [`stdlib/algebra/`](stdlib/algebra/) | the copiers and the relative product; the Boolean lattice; the extensive operators and their closed-world caveat; the tag lemmas and the two Tarski equations; Schröder's negative propagation; then the seven property **checks** — once satisfied, once violated apiece — the totality pair's `forall` in both directions, and the totality pair's **obligation** duals, each once owing and once satisfied |
| [`stdlib/bijection/`](stdlib/bijection/) | the two setup fan-outs and the negative completion; domain- and range-elimination, each productive in its own file and redundant in its sibling's; the two arg typechecks violated; and `06_blind_enumeration.ein`, the one program here that leaves the enumerator **on**, by an obligation the rung declines, which is the only way to reach it from a program that declares one |
| [`stdlib/elim/`](stdlib/elim/) | the same inference in its *positional* formulation, plus `no-room-left` — the pair that brackets a quantifier: one exclusion short of full forces a value, exactly full refutes |
| [`stdlib/closure/`](stdlib/closure/) | `infer-closure`, and its soundness caveat **exhibited**: a program the import takes from many models to one; plus `02`, closed and satisfied. An obligation on a closed relation is refused at load ([Q-M1f.244](../docs/decisions/hypotheses/Q-M1f.244-an-obligated-relation-is-not-closed-whatever.md)), so the closed and owing case is `examples/ein-bugs/closed-and-owing.ein`, a refused program |
| [`stdlib/slots/`](stdlib/slots/) | a second activating puzzle for the module beside `examples/zebra.ein` — the partition chain; `slot-fill` and `slot-elimination` as a matched pair, each productive where the other cannot fire; both violation duals; and the spatial family over two different position structures, one of which exists nowhere else and is what tells `slot-prune-fwd` from `slot-prune-bwd` |
| [`stdlib/typing/`](stdlib/typing/) | the reflexive-closure knob, and the `(type-hierarchy …)` knob in both directions |
| [`stdlib/math/decimal-arith/`](stdlib/math/decimal-arith/) | `std.math.decimal-arith`, school arithmetic computed by saturation alone: addition, subtraction and comparison, multiplication, division, and the faults of an operand and a `let`: one program per rule or tight family, each claiming its result relation's whole extent and R = {∅}, but those that divide by zero and those that state something the theory flags, which claim `(false)`; see [below](#stdlibmathdecimal-arith-a-theory-whose-answers-a-program-cannot-check) |
| [`stdlib/math/decimal/`](stdlib/math/decimal/) | `std.math.decimal`, the numeral and its check where it is written: a well-formed numeral in every place a file writes one; a malformed one in a `let` nobody asks about, in a fact of the program's own and in a rule's conclusion, each claiming `(false)`; what stays silent; and a program's own `(named z)`. `std.math.arith`, whose two rules the division-by-zero programs activate, has none of its own; see [below](#stdlibmathdecimal-a-numeral-checked-where-it-is-written) |
| [`stdlib/math/num9/`](stdlib/math/num9/) | `std.math.num9`, school arithmetic on nine digits in one fact by wide rules: a program a rule or tight family: a sum in one firing and a sum that overflows; subtraction's two rules; the one rule of comparison; a product, its zero digit and its three overflows; a quotient, each of the ten step rules, and a division by zero through `std.math.decimal-arith`'s rule; the conversions at every count of digits; both modules in one program; and a chain. See [below](#stdlibmathnum9-wide-rules-and-a-generated-module) |
| [`stdlib/math/int/`](stdlib/math/int/) | `std.math.int`, integers as a positive and a negative part over `std.math.decimal-arith`: the normal form; negation; addition, subtraction, multiplication and comparison, each in every combination of signs; an operand that is not simplified answering as its normal form does; operands by name with two rules of the program's own; division under each of the three conventions, every combination of signs with a remainder and without; a division by zero under each, claiming `(false)`; and the integer's faults and its own shape: a malformed numeral in a part, an undeclared name and a digit as an operand, a digit or a named constant as a part, each claiming `(false)`, and a natural number for an integer, silent. See [below](#stdlibmathint-a-theory-over-a-theory) |
| [`stdlib/math/rat/`](stdlib/math/rat/) | `std.math.rat`, rational numbers over `std.math.int` and `std.math.decimal-arith`, reduced only by `rat-simplify` over `std.math.gcd`: lowest terms by `std.math.gcd`'s call; negation; addition, subtraction, multiplication, division and comparison; an operand not in lowest terms answering as its normal form does; operands by name and a chain; four that claim `(false)`, a division by a rational that is zero and a denominator of zero in each place an operand can stand; the rational's faults and its own shape, as `std.math.int`'s; and two answers of one value, two facts, `rat-cmp` saying `eq`. See [below](#stdlibmathrat-lowest-terms-on-request-and-four-ways-to-divide-by-zero) |
| [`stdlib/math/gcd/`](stdlib/math/gcd/) | `std.math.gcd`, the greatest common divisor and the least common multiple of two natural numbers: Euclid's chain; zero in each place and gcd(0, 0); high zeros; names; the least common multiple, and with zero; the tagged call from a rule of the program's own; an undeclared name and a digit in each place, each claiming `(false)`; and an integer where a number goes, silent. See [below](#stdlibmathgcd-euclids-chain-and-zero-in-each-place) |
| [`stdlib/math/digits/`](stdlib/math/digits/) | `std.math.digits` is a library of **facts** and has no rule to activate; see [below](#a-module-of-facts-has-no-rule-to-activate). Two programs: every table imported flat with a row of each claimed and R = {∅}; and a refutation through a lookup, whose core names the row |
| [`stdlib/bool/`](stdlib/bool/) | `std.bool.gates`, `.props`, `.circuit` and `.inputs`, the sixteen Boolean functions of two inputs and a circuit: the programs under `gates/` and `props/` generated, each claiming relations whole; a circuit simulated and one refuted; inputs as hypotheses, a truth table and a requirement. See [below](#stdlibbool-the-sixteen-functions-by-rules-and-a-circuit) |
| [`stdlib/macro/`](stdlib/macro/) | `forall` checked against its own expansion written out by hand, and `unknown`'s three states |

### The obligation pairs, and the check that is not an `:expect`

Four rules assert the verdict atom `open` — `total-owed`, `surjective-owed`,
`slot-owed-room`, `slot-owed-fill`
([when an obligation is read](../docs/ein/user/syntax-and-semantics/obligations.md#when-an-obligation-is-read))
— and eight programs exercise them, one owing and one satisfied per rule. They
are written the way everything else here is, with one exception that is worth
knowing before writing a ninth.

**A claim says *that* a state owes, never how much.** `:expect`'s forms are
all assertions about *facts*, and an `open` conclusion is by construction never
a fact: it is a tally on the search-lattice node, because a stored one would
survive its own discharge in a fork that paid it. So the owing file of each
pair names its state under `(owing (model …))` on its `(solve …)` form, the
satisfied one names its state in R — which is what satisfies the coverage gate
and what pins the *state* the debt is read from — and the count itself is
asserted in
[`ein-infer/tests/obligation_reports.rs`](../ein.rs/crates/ein-infer/tests/obligation_reports.rs),
in-process.

**The satisfied half is the half that matters.** The coverage gate is happy
with the firing file alone. What it cannot show is that the report ever
*stops*: discharge **is** the guard — there is no second query that could
disagree with the rule's own `absent` — so "this file reports nothing" is the
whole claim that the guard is right.

**`06_blind_enumeration.ein` is not a rule's test.** It exists because the
activator facts a declaration derives are *stored* facts, and the question "can
a blind run start guessing about them" has to be a fixture rather than an
argument. An obligation's name is no candidate object
([Q-M1f.226](../docs/decisions/hypotheses/Q-M1f.226-an-obligations-name-is-no-candidate-object.md)):
were it one, the enumerator would propose `(seats total-owed C1)`, the name of
a rule as a puzzle value. It is the only program in this directory that leaves
the enumerator on, and it names only the hierarchy relations and `sort` in
`:no-hypothesis`, because a file that scoped the activators out would be
testing its own scoping line.

**And it has to *earn* the enumerator**, which is worth knowing before editing
it. Hypothesis generation is a ladder — the puzzle's hrules, else what root
owes, else the blind enumeration — so a program declaring `(bijective seats)`
does not reach the third rung on its own. It gets there with a second
obligation, `sorted-somewhere`, a bare `(open)`, which names no relation to
branch on, so the rung declines it, and a declined obligation hands the whole
call to the blind enumerator
([the ladder § 4.3](../docs/ein/reasoning/kb-evolution/hypothesis-generation.md#43-when-it-declines)).
So the file is also the corpus's one `mode=declined` fixture, and the
obligation it adds is *satisfied* on purpose: the decline must not depend on
the KB happening to owe something.

### A module of facts has no rule to activate

`std.math.digits` holds the calculator's digit tables as plain facts and no
rule; `std.math.bits` holds binary's. The coverage gate asks that every stdlib
*rule* be activated by a program here, so it has nothing to ask of these
modules, and `stdlib_coverage.rs` names them as rule-free, beside `std.macro`,
rather than inferring that a module with no rule is fine. The mutation sweep
likewise mutates rules and has none here.

**What holds the rows is not a program here.** A wrong row is a wrong answer
in every sum that reads it, and no `.ein` program can say what 7 + 5 is
without the table it would be checking. So the rows are held from outside the
language: [`ein-ir/tests/digit_tables.rs`](../ein.rs/crates/ein-ir/tests/digit_tables.rs)
computes every table from Rust's integers and compares, none missing and none
twice.

The two programs under [`stdlib/math/digits/`](stdlib/math/digits/) hold what only a
program can show:

| file | what it holds |
|---|---|
| `01_tables.ein` | an import brings the module whole, every table's rows (the `import-facts-*` regressions, seen from the stdlib); each goal is a lookup, constants where the table reads and variables where it gives, and its claim closes the goal, so a second row for one reading fails the file; `nz` and `adj` are claimed whole; and the module's own closing facts, which every import brings, leave **no candidate**, R = {∅}, claimed on the `(solve …)` form with no `:hypothesis-relations` list to help |
| `02_a_core_names_its_rows.ein` | a refutation through a lookup: root is dead and the unsat core is `(column 7 5 3)` and `(fa 7 5 0 2 1)`, the claim and the row it was checked against. The agreeing `(column 7 5 2)` is there so the rule is matched and silent once (*a refutation rule gets two files*, here two facts) |

### `stdlib/math/decimal-arith/`: a theory whose answers a program cannot check

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
it on the matcher's count of its work; it builds only under `--features
counters` (`cargo test -p ein-infer --features counters --test arith_batch`).
Every operation is a row of that file's `OPS` and is asked the same cases, and
subtraction and comparison besides with equal operands and operands one apart
at every length, a borrow through every length, and requests by name.

| file | the rules it is there for | what would fail it |
|---|---|---|
| `01_add.ein` | the whole chain once, 123078 + 4567: `add-ask`, `add-ask-second`, `add-start`, `operand-number`, `add-column`, `add-column-first`, `add-end`, `strip-top`, `turn-step`, `turn-end`, `add-answer` | a wrong digit, a second answer, an answer filed under the operands exchanged |
| `02_carry_out.ein` | `add-carry-out`: 999999999 + 1 has ten digits, and 5 + 5 two | the carry lost at the top |
| `03_lengths.ein` | `add-column-second` beside `add-column-first`, each pair asked both ways round, with a carry running through the longer number | the two column rules confused |
| `04_zero.ein` | `operand-zero`, `strip-none`: zero is `z`, asked, added and answered | a zero answered as `(n z 0)` |
| `05_high_zeros.ein` | `strip-zero`: operands written with high zeros, read by value; a zero inside a number stays | an answer that is not in normal form |
| `06_named.ein` | `let-named` and `operand-name`: `(let NAME NUMBER)`, a request by name, by a name and a number, and the same sum by numbers as a request of its own | a request answered under another spelling of its operands |
| `07_only_what_is_asked.ein` | nothing fires for a `let` nobody asks about: `done`'s whole extent is one call | a theory that computes what was not asked |
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
| `22_malformed_digit.ein` | `std.math.decimal`'s `numeral-digit` and `not-decimal-is-false`: a name that is no digit in a digit's place, one place up; `(solve :expect (false))` | a malformed number passed over in silence |
| `23_malformed_fact_as_digit.ein` | `numeral-digit` on a fact | a guard that holds of names only |
| `24_malformed_ends_in_digit.ein` | `numeral-prefix-digit`: `(n 1 2)` | a number that ends in a digit read as a number |
| `25_malformed_ends_in_zero.ein` | `numeral-prefix-zero`: `(n 0 2)` | the digit 0, which `nz` has no row for, missed |
| `26_malformed_ends_in_a_name.ein` | `numeral-prefix-named`: `(n price 1)` | a named constant inside a number |
| `27_malformed_digit_as_number.ein` | `operand-digit` and `bad-cmp-second`: `(cmp X 7)` | a digit read as an operand that is merely never found |
| `28_malformed_zero_digit_as_number.ein` | `operand-digit-zero` and `bad-mul-first`: `(mul 0 X)` | the digit 0 as an operand |
| `29_undeclared_name.ein` | `operand-undeclared`, `bad-mul-second`, `bad-request-undeclared`, `undeclared-is-false`: a name no `let` states | an undeclared name passed over in silence |
| `30_declared_name.ein` | the same rules loaded and not firing: `29` with the `let` stated; the product answered, and `undeclared`'s and `malformed`'s extents empty | a flag left behind once the `let` is stated; a flag on a well-formed request |
| `31_malformed_through_a_name.ein` | `numeral-digit` on a `let`'s value, a request naming it and one nobody asks about, each flagged where it is written | a malformed number that hides behind a name |
| `32_malformed_name_for_a_name.ein` | `let-to-name`: `(let alias price)` | a name that stands for a name read as a number |
| `33_let_twice.ein` | `let-twice`: a name stated twice with two numbers | two answers to one request |
| `34_let_twice_the_same.ein` | `let-twice` loaded and not firing: the same `let` written twice is one fact | a flag on a fact stated twice |
| `35_let_zero.ein` | `let-zero`: `(let z N)` | `z` standing for itself and for a number |
| `36_let_digit.ein` | `let-digit`: `(let 7 N)`, with no request that uses it | a `let` flagged only when asked about |
| `37_let_zero_digit.ein` | `let-digit-zero`: `(let 0 N)` | the digit 0, which `nz` has no row for, missed |
| `38_what_stays_silent.ein` | no rule: what the theory does **not** flag, a number that ends in a name that is neither a digit nor a constant, a fact of another relation as an operand and inside a number, the accumulator's shape; beside an operand with high zeros, a foreign fact of `n` and `z` in a relation of the program's own, which are fine | a rule that starts flagging one of them, which makes the module's page wrong; a check that fires on a fact nobody asked about |
| `42_let_to_a_digit.ein` | `let-to-digit`: `(let seven 7)`, with no request that uses it | a digit a `let` names read as a number, or flagged only when asked about |
| `43_let_to_the_digit_zero.ein` | `let-to-digit-zero`: `(let none 0)` | the digit 0, which `nz` has no row for, missed |
| `44_digit_as_the_first_operand_of_add.ein` to `50_digit_as_the_second_operand_of_div.ein` | `operand-digit` and, one a program, `bad-add-first`, `bad-add-second`, `bad-sub-first`, `bad-sub-second`, `bad-cmp-first`, `bad-div-first`, `bad-div-second`: a malformed numeral is `std.math.decimal`'s, flagged where it is written, so a digit as the operand is what reaches each of these rules | a rule from a tag to its request that reads the wrong tag |

**A division by zero gets the refutation idiom**, *one file where it fires and
one where it is loaded and satisfied*: `20` and `21` fire the two rules, and
`16` to `19` load them on divisors that are not zero and claim an R of one
state. A program that divides by zero can claim only `(false)`: root is dead,
so no query of it has an answer. `arith_search_free.rs` holds such a program
to an empty R decided by root alone, one saturation, the request in the core,
and `refuted by` naming the second rule; `arith_theory.rs` holds the flag to
the requests that divide by zero and to no other.

**A malformed request gets the same idiom**:
`22` to `29`, `31` to `33`, `35` to `37` and `42` to `50` each fire one fault,
`27` to `29` and `44` to `50` through one of the ten rules from an operand's
tag to its request, so that a mutant of any of them is seen, and claim
`(false)` (`22` to `26` and `31` reach `std.math.decimal`'s check of a numeral,
where it is written); `30`, `34` and `38` load the same
rules where none fires and claim an R of one state with the flags' extents
empty. **One fault a program, and one malformed request**, because a dead
program's one claim cannot say which rule killed it: two faults in a file
would let either rule be deleted unseen. `arith_search_free.rs` holds such a
program as it holds a division by zero, with `refuted by` naming
`malformed-is-false` or `undeclared-is-false`, and holds that the flagged
request has no result fact; `arith_theory.rs` holds every kind of fault
under every operation and either operand to exactly one flag, and the list
of what stays silent.

`decimal-arith`, the closing rule, fires once at root in each. Every mutant of
the module's walks is killed. `03_lengths.ein` is written both ways round
because of it: `(add ?x ?y)` with its operands exchanged survives any program
whose requests are symmetric.

**The fault rules' survivors are banked, each with its reason.** A program the
theory flags is dead, and a dead program's one claim, `(false)`, cannot say
*which* flag killed it or how the flag is written. So `ein test` cannot see
the ten mutants that drop one of `operand-undeclared`'s `(neq ?x DIGIT)`
guards, under which a digit as an operand is flagged twice, `malformed` and
`undeclared`, and is dead either way; nor `bad-request-undeclared` with its
flag's two arguments exchanged; and `undeclared-is-false` with its premise's
two variables exchanged is the same rule (the last two are
`std.math.positional`'s, the rules both bases share). `arith_theory.rs` holds
all of them from outside the language: its harness compares the flags' text,
and asks that a digit as an operand has no `undeclared` flag.

**A division by zero through the tagged call.**
`39_div_call_by_zero.ein` and `40_div_call_by_zero_spelt.ein` are a division
by zero that a rule of the program's own makes through the tagged call,
`(div-call (mine A B) A B)`, with no `div` request: they kill
`div-call-by-zero` with its premise's arguments exchanged, and
`41_div_call_not_zero.ein` is the same rules loaded and silent. A public
division by zero has two flags, the request's and its call's, either of which
is `(false)`, and a dead program's one claim cannot say which flag killed it,
so `div-by-zero` with its premise exchanged survives, banked.
`arith_search_free.rs` holds it from outside the language: a dead program
whose core names a `(div X Y)` has exactly two flags in root's state, each by
its own rule, and `arith_theory.rs` holds the two for every public request.

### `stdlib/math/decimal/`: a numeral checked where it is written

`std.math.decimal` declares the numeral `n` and checks every numeral a file
writes, off the `(written T)` facts the loader states
([Q-M1f.275](../docs/decisions/language/Q-M1f.275-written-states-every-fact-a-file-nests.md)),
wherever it sits and with no request reading it. Its rules are activated by
`22` to `26` of `stdlib/math/decimal-arith/`, whose numerals are a request's
operands; the programs here are for **where it is written**, each one place,
one fault:

| file | the rules it is there for | what would fail it |
|---|---|---|
| `01_well_formed_everywhere.ein` | the check loaded and silent: a well-formed numeral as an operand, in an unused `let`, in a fact of the program's own, in an integer and a rational, and in a rule's conclusion; R is {∅}, the sum answered and `not-decimal`'s extent empty | a well-formed numeral flagged; a place whose numerals are never stated |
| `02_in_a_let_nobody_asks_about.ein` | `numeral-digit` on a `let` no request names | a numeral checked only where a calculation reads it |
| `03_in_a_fact_of_the_programs_own.ein` | `numeral-prefix-digit` on a fact of the program's own relation, `(price apple (n 1 2))` | a numeral checked only in the calculator's own requests |
| `04_in_a_rules_conclusion.ein` | `numeral-prefix-zero` on a numeral written whole in a rule's conclusion, a rule that never fires: core ∅ | the loader stating facts but not the ground terms of a conclusion |
| `05_what_stays_silent.ein` | no rule: `(n end 1)`, `(n (pair 1 2) 3)`, a numeral a rule assembles from a variable, and a top-level fact of `n`, each silent beside a sum answered | a rule that starts flagging one of them, which makes the module's page wrong |
| `06_a_programs_own_named_z.ein` | `numeral-prefix-named`'s `(neq ?m z)`: the program lists `named` and writes `(named z)` itself beside `(price apple (n z 3))`, and R is {∅}, a sum answered | the guard dropped, which flags every numeral, all of which end in `z` |

`std.math.arith` has two rules, `division-by-zero-is-false`, which
`20_div_by_zero.ein` and the other divisions by zero activate, and its closing
rule, which every program here fires: it has no program of its own.

**The checks' survivors are banked, each with its reason.** Every program a
check of a representation flags is dead, and a dead program's one claim,
`(false)`, cannot say how its flag is written or which of two flags killed it:
so the mutants that exchange a flag's arguments survive, in each of
`std.math.decimal`'s four rules, `std.math.int`'s six and `std.math.rat`'s six;
the three `…-is-false` rules with their premise's variables exchanged are the
same rule; and `int-bad-div` either way, the division's tag's own flag killing
the program beside it. `arith_theory.rs`, `int_theory.rs` and `rat_theory.rs`
hold every flag's text from outside the language. `numeral-prefix-named`
without its `(neq ?m z)` is not among them: a program may write `(named z)`
itself, and there the guard is the difference between R = {∅} and an empty R;
`06_a_programs_own_named_z.ein` is that program, and it kills the mutant.

### `stdlib/math/binary-arith/`, `binary/`, `bits/` and `convert/`: the second base

Binary arithmetic is `std.math.binary-arith`'s rules of its own over
`std.math.bits`' tables, and `std.math.positional`'s, the rules both bases
share, activated by one call in its closing rule; `std.math.binary` is the
numeral and its check, on `std.math.decimal`'s pattern; `std.math.convert`
converts. **A shared rule is one rule however many bases activate it**: the
coverage gate counts `std.math.positional.add-column` once, and decimal's
programs above already activate every one of them. The programs here activate
them again with binary's relations, so that a rule read wrong for one base and
right for the other is seen, and activate each of binary's own:

| directory | what they are there for |
|---|---|
| `binary-arith/` | `01` to `07` the five operations, zero, lengths, high zeros and `bin-let`, every shared rule of the walks and the protocol and binary's product and division rules (`07` reaches `bin-div-bring-zero-none` with a dividend written `0101`); `08` and `09` a division by zero, the request's and a call a rule of the program's own makes; `10` a bit as either operand of every request, the ten routes from a tag to its request once each; `11` the bit `0` as an operand; `12` and `13` an undeclared name, `7` among them, and the same names stated (`bin-operand-undeclared` fired and satisfied); `14` to `20` each `let` rule; `21` both bases in one program, each answering its own; `22` what stays silent |
| `binary/` | `01` a well-formed numeral in every place, `not-binary-is-false` loaded and satisfied; `02` to `05` each of the four rules of the check, one fault each; `06` what stays silent, a decimal digit as a prefix among it; `07` `(bin-named z)` written by the program beside `(b2 z 1)`, R {∅} and a sum answered, which `bin-numeral-prefix-named`'s `(neq ?m z)` holds |
| `bits/` | the tables seen from a program, as `digits/01_tables.ein`; the rows are `ein-ir/tests/digit_tables.rs`' |
| `convert/` | each way, and a round trip a rule of the program asks back |

**What holds the answers is outside the language**, as for decimal:
[`ein-infer/tests/binary_theory.rs`](../ein.rs/crates/ein-infer/tests/binary_theory.rs)
holds the five operations to Rust's integers, every conversion to 300 and a sample
to 2^40 both ways, and the two bases in one program against each other;
`arith_theory.rs` holds the rule of the calculator on binary's text and on
`std.math.positional` read as each base activates it, and the macro's body and
each base's call. Every claim here was computed from Python's integers.

**Binary's survivors are decimal's twins**, banked for the same reasons: every
program `std.math.binary` flags is dead, so its four flags with their arguments
exchanged and `not-binary-is-false` with its premise's variables exchanged are
not seen by a claim made of facts; and `bin-operand-undeclared` without
`(neq ?x 0)` or `(neq ?x 1)` flags a bit as an operand undeclared beside the
`(bin-malformed …)` that kills the program, as decimal's ten digits' do.
`binary_theory.rs` holds every flag's text from outside the language.
`bin-numeral-prefix-named` without its `(neq ?m z)` is killed by
`binary/07_a_programs_own_named_z.ein`, as decimal's twin is by `decimal/06`.

### `stdlib/math/num9/`: wide rules, and a generated module

`std.math.num9` is the calculator's second module: nine digit names in one fact,
`(num9 0 0 0 1 2 3 0 7 8)`, and rules with a premise a column, so a sum is
one firing. What `stdlib/math/decimal-arith/` says of its programs holds of these: each
claims R = {∅} on its `(solve …)` form, `arith_search_free.rs` reads the
search-free claim off every one, and right is held from outside the language,
by `ein-infer/tests/num9_theory.rs` against Rust's integers, **exhaustively**
at width 2 and on a sample at width 9. The module is generated and is not
edited by hand: a rule is changed in its generator.

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
`03_sub.ein` says why in its header: `(sub9 X Y)` read with its operands
exchanged survives in both subtractors any program that asks X − Y and
Y − X, since each gets its fact from the other's request and the two together
look right. **Every mutant of the module is killed.**

### `stdlib/math/int/`: a theory over a theory

`std.math.int` is the calculator's first extension: an integer is `(int P N)`, two
natural numbers of `std.math.decimal-arith`, and every rule of the module has one premise
and starts one of `std.math.decimal-arith`'s walks. What `stdlib/math/decimal-arith/` says of its
programs holds of these: each claims R = {∅} on its `(solve …)` form,
`arith_search_free.rs` reads the search-free claim off every one, and right is
held from outside the language, by `ein-infer/tests/int_theory.rs` against
Rust's signed integers, every pair of integers written with one-digit parts
under every operation. Every claim here was computed from Python's integers.

| file | the rules it is there for | what would fail it |
|---|---|---|
| `01_simplify.ein` | `int-operand`, `int-simplify-ask`, `-start`, `-answer`, `int-norm-walks`, `int-norm-plus`, `int-norm-minus`: nine operands, 5 less 3, 3 less 5, equal parts, high zeros | the parts exchanged; a zero that is not `(int z z)`; an answer with a high zero |
| `02_neg.ein` | `int-neg-ask`, `-start`, `-answer`: seven operands, simplified and not | a negation that only exchanges the parts and does not simplify |
| `03_add.ein` | `int-operand-second`, `int-add-ask`, `-start`, `-answer`, `int-add-plus`, `-minus`, `-norm`: eleven sums in every combination of signs | a part added to the wrong side; the operands exchanged in the answer |
| `04_sub.ein` | `int-sub-ask`, `-start`, `-answer`: ten differences, each one integer | the second operand's parts not exchanged, which is a sum |
| `05_mul.ein` | `int-mul-ask`, `-start`, `-answer`, `int-mul-first` to `-fourth`, `-plus`, `-minus`, `-norm`: every combination of signs, zero on either side, operands not simplified | a cross product on the wrong side: the sign of every mixed product |
| `06_cmp.ein` | `int-cmp-ask`, `-start`, `-answer`, `int-cmp-left`, `-right`, `-sides`: twelve comparisons, two pairs written differently that are `eq` | the sides added straight and not across |
| `07_unsimplified.ein` | an unsimplified operand: 5 less 3 and 2 less 0 under each of the four operations | an operand read by its spelling |
| `08_named_and_chained.ein` | names through `std.math.decimal-arith`'s `let`; two rules of the program's own that read a negative difference by its shape, one deriving an `int-neg` request | a named integer left unanswered; a derived request left unanswered |

**No request here is another the other way round,** `stdlib/math/num9/`'s
lesson: `03_add.ein`'s header says why. **Every mutant of the module's
operations is killed**; its checks' survivors are
[above](#stdlibmathdecimal-a-numeral-checked-where-it-is-written).

**Integer division, under three conventions.**
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

**What is malformed, and whose it is.** Each program is one fault and claims
`(false)`, but the silent one, which claims R = {∅} and answers a well-formed
sum beside its request:

| file | the rules it is there for | what would fail it |
|---|---|---|
| `15` to `19`, `24`, `30` | `std.math.decimal`'s check of a numeral in a part, in either part, either operand, behind a name and in a division: the numeral's fault is decimal's, flagged where it is written | a numeral inside an integer never checked |
| `20_undeclared_first_operand.ein`, `21_undeclared_second_operand.ein`, `36_undeclared_only_operand.ein` | `int-bad-first`, `int-bad-second`, `int-bad-only`: an undeclared name, flagged on the request | an operand's fault seen under the module's tag and not flagged |
| `22_digit_as_an_operand.ein` | `int-bad-first` on a digit as the operand | a digit as an integer operand passed over |
| `35_undeclared_divisor.ein` | `int-bad-first` and `int-bad-div`: a division's tag holds `(int-div-of REQUEST)`, and the request as written is flagged beside it | a division's flag that names no request |
| `23`, `29`, `31` to `34` | `int-p-digit`, `-named`, `int-n-digit`, `int-p-zero`, `int-n-zero`, `int-n-named`, and `not-integer-is-false`: a digit or a named constant where a part goes ([Q-M1f.276](../docs/decisions/stdlib/math/Q-M1f.276-an-integers-and-a-rationals-own-shape-is-checked.md)) | an integer's own malformation read as an integer |
| `25` to `27` | `let-twice`, `let-zero`, `let-digit` on a `let` of an integer | a `let` of an integer not read |
| `28_natural_where_an_integer_goes.ein` | nothing: a natural number for an integer is silent ([Q-M1f.277](../docs/decisions/stdlib/math/Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md)), and the request is not answered | a rule that starts flagging another type, which makes the module's page wrong |

### `stdlib/math/rat/`: lowest terms on request, and four ways to divide by zero

`std.math.rat` is the calculator's second extension: a rational is `(rat Z D)`, an
integer of `std.math.int` over a natural number; an operation answers
unreduced, and `rat-simplify` alone answers in lowest terms, by
`std.math.gcd`. What `stdlib/math/int/` says of its programs holds of these:
every rule of the module has one premise, `01` to `09` and `31` claim
R = {∅}, and right is held from outside the language, by
`ein-infer/tests/rat_theory.rs` against exact fractions, by value. Every claim
here was computed from Python's fractions: a claim names the unreduced answer
an operation gives, and a program whose point is lowest terms asks
`rat-simplify` for it (`08`, and `examples/arith/rat.ein`).

| file | the rules it is there for | what would fail it |
|---|---|---|
| `01_simplify.ein` | `rat-operand`, `rat-operand-check`, `-found`, `rat-simplify-ask`, `-start`, `-pos`, `-neg`, `-answer`, `rat-lowest-numerator`, `-denominator`, `-pos`, `-neg`, and `std.math.gcd`'s call: eleven operands, 1071/462 = 51/22 by a chain of three divisions | a common divisor that is not the greatest; a zero that is not 0/1; a sign lost in the reduction |
| `02_neg.ein` | `rat-neg-ask`, `-start`, `-answer`, `rat-norm-numerator`, `-denominator`, `-done`: −(2/4) = −2/4, −(0/5) = 0/5 | a negation that reduces; a denominator left with a high zero |
| `03_add.ein` | `rat-operand-second`, `rat-add-ask`, `-start`, `-answer`, the seven rules of the cross products, `rat-sum-denominator`, `rat-sum-norm`: eight sums in every combination of signs, 1/6 + 1/3 = 9/18 | a cross product with the wrong denominator; a part on the wrong side; a sum reduced |
| `04_sub.ein` | `rat-sub-ask`, `-start`, `-answer`: seven differences | the second numerator's parts not exchanged, which is a sum |
| `05_mul.ein` | `rat-mul-ask`, `-start`, `-answer`, `rat-mul-numerator`, `-denominator`, `-done`: seven products, 2/3 × 3/4 = 6/12 | a product reduced; a sign |
| `06_div.ein` | `rat-div-ask`, `-start`, `-answer`, `rat-div-pos`, `rat-div-neg`: six quotients | the divisor not turned over; its sign lost |
| `07_cmp.ein` | `rat-cmp-ask`, `-start`, `-answer`, `rat-cmp-sides`: ten comparisons, 22/7 against 355/113 | the denominators ignored; the order of a negative pair |
| `08_unsimplified.ein` | an operand not in lowest terms: 2/4 and 1/2 under each of the five operations; four rules of the program's own ask `rat-simplify` of each answer, one lowest terms for the two spellings | an operand read by its spelling |
| `09_named_and_chained.ein` | names through `std.math.decimal-arith`'s `let`; two rules of the program's own, each deriving a request from an answer, the chain carrying 15/24 | a named rational left unanswered; a derived request left unanswered |
| `10_div_by_zero_rational.ein` | `rat-div-zero`: a divisor that is 0 over 5; `(solve :expect (false))` | a division by zero answered, or passed over in silence |
| `11_zero_denominator_first.ein` | `rat-operand-zero`, `rat-zero-first` | a rational over zero computed with |
| `12_zero_denominator_second.ein` | `rat-zero-second`: the operand named, the zero written `(n (n z 0) 0)` | a zero read by its spelling |
| `13_zero_denominator_one_operand.ein` | `rat-zero-only`: `(rat-neg X)` with X over nothing | a request of one operand that is not checked |
| `31_equal_values_two_facts.ein` | one value, three facts: 4/4, 16/16 and 6/6; `rat-cmp` against one says `eq` of each, and `rat-simplify` gives each 1/1 | an operation that reduces, which makes the three one fact; an equality read off the spelling |

**Four programs are dead on purpose**, one a rule that derives the flag. Each
states a sum beside the request that kills it, and claims only `(solve
:expect (false))`: root's own fixpoint derives `(false)`, R is empty, and
`arith_search_free.rs` holds each to that, the request named in the core and
`std.math.decimal-arith`'s rule under `refuted by`. A core names the request, a `let`
where the operand is named, and the rows of `cl` a denominator was compared
with zero through.

**Every mutant of the module's operations is killed**, `rat-simplify`'s and
the normal form's among them; its checks' survivors are
[above](#stdlibmathdecimal-a-numeral-checked-where-it-is-written). No request
here is another the other way round where both would be answered alike,
`stdlib/math/num9/`'s lesson.

**What is malformed, and whose it is**, as in `stdlib/math/int/`:
`14` to `16` and `20` a malformed numeral, `std.math.decimal`'s; `17`, `18` and
`30` an undeclared name, `19` a digit, through `rat-bad-first`, `-second` and
`-only`; `21` a `let` stated twice; `24` to `29` the rational's own shape,
`rat-z-digit`, `-zero`, `-named`, `rat-d-digit`, `-zero`, `-named` and
`not-rational-is-false`; and `22`, `23`, an integer and a natural number for a
rational, silent
([Q-M1f.277](../docs/decisions/stdlib/math/Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md)).

### `stdlib/math/gcd/`: Euclid's chain, and zero in each place

`std.math.gcd` is a theory over `std.math.decimal-arith`: every rule of the
module has one premise and makes one of that theory's tagged calls, and right is
held from outside the language, by `ein-infer/tests/gcd_theory.rs` against
Rust's integers, every pair from 0 to 30 under both requests. Every claim here
was computed from Python's `math.gcd` and `math.lcm`; `08` to `11` claim
`(false)`, and the others R = {∅}.

| file | the rules it is there for | what would fail it |
|---|---|---|
| `01_gcd.ein` | `gcd-ask`, `gcd-ask-second`, `gcd-start`, `gcd-answer`; the call's `gcd-call-first`, `-second`, `-chain`; the chain's `gcd-step`, `gcd-next`, `gcd-end`: seven requests, 1071 and 462 both ways round | a remainder that does not go on; the chain stopped a step early; the operands exchanged in the answer |
| `02_gcd_zero.ein` | the chain at its end at once, and after one step: zero in each place, written `z` and `(n z 0)`, gcd(0, 0) = 0 ([Q-M1f.299](../docs/decisions/stdlib/math/gcd/Q-M1f.299-the-greatest-common-divisor-of-zero-and-zero-is.md)) | a zero written with digits divided by; gcd(0, 0) unanswered |
| `03_high_zeros.ein` | the call's two walks to normal form, where the answer is an operand's value | an answer with a high zero, an operand as written |
| `04_named.ein` | names through `std.math.decimal-arith`'s `let`, in both requests | a named operand left unanswered |
| `05_lcm.ein` | `lcm-ask`, `lcm-start`, `lcm-product`, `lcm-divide`, `lcm-answer`: six requests | the product not divided; the operands as written multiplied |
| `06_lcm_zero.ein` | `lcm-zero`, and a zero beside a number through the product | a least common multiple of zeros divided by zero |
| `07_gcd_call.ein` | the tagged call `(gcd-call TAG X Y)` from a rule of the program's own, answered under its tag ([Q-M1f.300](../docs/decisions/stdlib/math/gcd/Q-M1f.300-the-greatest-common-divisors-tagged-call-is.md)) | an answer not under the caller's tag; a call read as a request |
| `08_undeclared_first_operand.ein`, `09_undeclared_second_operand.ein` | `gcd-bad-first`, `gcd-bad-second`: an undeclared name, flagged on the request, a gcd and an lcm | an operand's fault seen under the module's tag and not flagged |
| `10_digit_as_an_operand.ein`, `11_digit_zero_as_the_second_operand.ein` | the same two rules on a digit, and on the digit 0 | a digit as an operand passed over, or read as a zero to divide by |
| `12_what_stays_silent.ein` | nothing: an integer where a number goes is silent (natural numbers only), and the request beside it answered | a rule that starts reading another type |

**Every mutant of the module is killed.**

### `stdlib/bool/`: the sixteen functions, by rules, and a circuit

Four modules, a directory each. **The programs under `gates/` and `props/` are
generated** from the sixteen columns and are not edited by hand: `gates/01`
claims each of the sixteen relations of the second layout whole, and
`props/01` to `07` claim every relation of `std.bool.props`, a family a
program, each relation's whole extent computed in Python. A library with no
refutation rule has its negative case in that idiom alone: a claim that names a
relation closes it, so a rule that derived one fact too many, or one too few,
fails the file. Right is held from outside the language too, by
`ein-infer/tests/bool_theory.rs`, against each property's definition in Rust.
A generated program's R is {∅}; the four written by hand state theirs.

| file | the rules it is there for | what would fail it |
|---|---|---|
| `gates/01_a_relation_a_function.ein` | the sixteen `Zero-rows` to `One-rows`, and `gates` | a relation with a row its function does not have, or without one it has |
| `props/01_column.ein` | `column-of` | a column read in the wrong order |
| `props/02_between_two.ein` | `swap`, `complement`, `dual`, `left-neg`, `right-neg` | a function paired with the wrong one; a dual written as its complement |
| `props/03_commutative_associative_idempotent.ein` | `commutative-r`, `assoc-point`, `associative-r`, `idempotent-r` | a function associative at seven points of eight |
| `props/04_reads_and_ignores.ein` | `reads-first-r`, `reads-second-r`, `ignores-first-r`, `ignores-second-r` | an input exchanged for the other |
| `props/05_identity_and_absorbing.ein` | the one-sided and two-sided identity and absorbing rules, six | a side exchanged: `Andn` has a right identity and no left one |
| `props/06_post_inside.ein` | `preserves-zero-r`, `preserves-one-r`, `monotone-r`, `affine-r`, `self-dual-r` | an order of the bits read backwards |
| `props/07_post_outside.ein` | the five `escapes-` rules and `complete-alone-r` | a witness pair read backwards |
| `circuit/01_propagate.ein` | `propagate`, `circuit`; `one-value` loaded and silent | a gate's inputs exchanged (`Rorn`'s are read); a gate with an input unset computing |
| `circuit/02_one_value.ein` | `one-value`: a gate's output against a written value, `(solve :expect (false))` | two values on a wire passed over |
| `inputs/01_in_drives.ein` | `in-drives`, `inputs`, and `std.algebra`'s `total-owed` and `functional`; `required` loaded and silent | R not the four assignments; the truth table's rows |
| `inputs/02_required.ein` | `required`: two requirements, one assignment of four left | a requirement read the wrong way round |

**The mutants of `std.bool.props` that survive are equivalent to their
rules**, each deriving the very facts the rule does, so no claim made of facts
can tell them apart and none is a hole; they are banked in
[`mutation_survivors.txt`](mutation_survivors.txt):

- **those that exchange the arguments of an `(inv …)` premise**, in
  `complement`, `dual`, `reads-first-r`, `reads-second-r`, `escapes-affine-r`
  and `escapes-self-dual-r`: `inv` is its own converse, `(inv 0 1) (inv 1 0)`,
  so the premise matches the same pairs either way round;
- **those that exchange the arguments of the conclusion** of `swap`,
  `complement`, `dual`, `left-neg` and `right-neg`: each relation is an
  involution, G is F's swap exactly when F is G's, so the rule's matches come
  in mirrored pairs and the conclusion written the other way round writes the
  same facts.

Every mutant of `std.bool.circuit` and `std.bool.inputs` is killed, as is every
one that exchanges an `le`, a one-sided relation or a value of `val`.

### Five things about how they are written

**Naming a relation closes it.** An expectation lists a relation's *complete*
extent, so a rule that derived one fact too many fails the file. That is what
makes a "must not fire" case checkable at all for a rule that carries no
guard: its only *must not* is scope, and scope is a claim about a whole
relation. Where a rule needs a relation to stay untouched, the fixture gives it
one authored edge and closes it there. A relation with no fact at all is
said empty another way: a query over it claims `(model)`, the class of a state
where its goal has no row, or a region claim names it in another state
([Q-M1c.6](../docs/decisions/answer/Q-M1c.6-a-claim-can-say-that-a-relation-is-empty.md)).

**`(unknown …)` is how a fixture says a negative was *not* invented.** Stored
negatives are deliberately not closed by an expectation, so listing the
exclusions that exist says nothing about the ones that do not. The programs
that carry a four-line `probe-undecided` rule (`grep -rl probe-undecided
tests/`) read it through `std.macro`'s `(unknown P)` — true exactly when a
fact is in neither store — turning the absence into an ordinary positive fact
the expectation *can* close. It reads under an `absent`, so the boundary judges
it once the closure is at its fixpoint, which is what a probe that reads the
world needs: to be the last thing to read it.

**A refutation rule gets two files.** "It fires and the answer is ⊥" is one
test and it is not the one that finds bugs; the other is a program where the
rule is loaded, activated and *satisfied*, so that a guard admitting too much
turns an ordinary model into a contradiction. `algebra/08_checks_satisfied.ein`
is that file for seven rules at once, and each of the seven has a `_violated`
sibling.

**Where two rules reach one verdict, separate them by activation.** An
expectation is made of facts and cannot say which rule produced them
([Q-M1c.2](../docs/decisions/language/Q-M1c.2-an-expectation-is-ground-and-naming-a-relation.md)).
On a fully excluded row `no-room-left` refutes and `domain-elimination`
forces a value against a stored negative, and both end in ⊥; the fixture picks
one by declaring only its activator. Five files do this — `elim/02`,
`elim/03`, `elim/04`, `slots/05`, `slots/06` — and each says so in its header,
because a reader who does not know why `slot-partition-setup` is absent will
put it back.

**A verdict word is not what an expectation claims.** All `:expect` forms are
assertions about **facts** ([`syntax.md` § Query](../docs/ein/user/reference/syntax.md#query)),
so no fixture here says `Solution`, `Open` or `Contradiction` — it says what
the answer's fact set is, and the facts an owing state reached are the facts it
reached. What a fixture cannot state it asserts in-process instead — the owe
counts in
[`obligation_reports.rs`](../ein.rs/crates/ein-infer/tests/obligation_reports.rs),
the rung modes in `obligation_rung.rs`. One form says that a state owes, though
not how much: `(solve … :expect (region (owing (model …))))` names an owing
state, which R, the states that are alive *and discharged*, does not hold
([solution semantics](../docs/ein/reasoning/kb-evolution/solution-semantics.md#2-the-answer-the-region-r)).
**A list may not switch an obligation's generating role off**: a
`:no-hypothesis` naming a relation an obligation owes is refused at load, and
so is a closure of one
([Q-M1f.244](../docs/decisions/hypotheses/Q-M1f.244-an-obligated-relation-is-not-closed-whatever.md)).
**To judge root alone, write `(solve :no-search)`**: the lattice is root alone,
nothing is proposed whatever the program declares, and the obligations judge
root, so R is root where it owes nothing and empty, root the owing node it
stops at, where it owes. That is what the fixtures whose question is root's
verdict or root's saturation say: `algebra/23` to `26`, `bijection/01` to `03`,
`slots/01` to `04`, `07` to `13`, `region/04_owing` and `05_empty`. An hrule or
a list beside `:no-search` proposes or filters nothing, and says so in a
`note:`
([Q-M1f.254](../docs/decisions/hypotheses/Q-M1f.254-solve-no-search-relaxes-no-conflict.md));
`ein test --no-search`, a flag of the runner, is another thing, a run that
declines to search R and checks only what root decides.

### What a mutation sweep says about them

**The suite's sensitivity is measured by mutating the stdlib.** One mechanical
defect at a time — a dropped `neq`, a dropped `absent`, the two arguments of a
premise or of the conclusion exchanged — is made in a copy of `stdlib/` and run
past `ein test tests/`; any non-zero exit is a kill, a refusal at load among
them. Every one of the separations above is there because a mutant survives
without it, so the sweep is not decoration: it is what built this directory.
The survivors are banked as a *set* in
[`mutation_survivors.txt`](mutation_survivors.txt) — a score decays while the
thing it guards grows — so a new survivor and a banked one that is killed both
fail the comparison, and each banked line has its reason on this page. Where
they are is
`grep -v '^#' tests/mutation_survivors.txt | cut -d: -f1 | sort | uniq -c`:
most in `slots.ein`, which has the most parameterised, direction-sensitive
rules and the fewest programs per rule. That distribution is the useful part
of the bank, and it is what a score alone never says.

**`slots/13_adjacent_bwd_neg_direction.ein` exists for one mutant.**
`slot-adjacent-bwd-neg` with its two structure operands exchanged survives on
the three-seat row of `slots/08_spatial_adjacent.ein`: the exclusion it should
derive is reachable from the other clue's chain a pass later.
[`slots/13_adjacent_bwd_neg_direction.ein`](stdlib/slots/13_adjacent_bwd_neg_direction.ein)
is the four-seat row that isolates it, and it kills two of its siblings too.
The exchanged rule is **sound and strictly weaker** — its guard becomes
satisfiable only where the conclusion is already true for another reason — so
no contradiction fixture can catch it, and only a program that *needs* the
derivation can.

**The region claim kills the obligation rules' mutants.** A mutant that
changes only *whether* a debt is owed leaves every fact where it was, so no
claim made of facts sees it. The region claim does: a mutant that stops a debt
being owed puts the state in R where the claim says R is empty, and one that
invents a debt takes a discharged state out of R where the claim lists it.

| mutant | killed by |
|---|---|
| `algebra.ein::total-owed::swap-premise::(?isa ?a ?A)` | `region/04_owing`, `region/05_empty`, `region/07_cube`, `stdlib/algebra/23_total_owed` |
| `algebra.ein::surjective-owed::swap-premise::(?isa ?b ?B)` | `stdlib/algebra/25_surjective_owed` |
| `slots.ein::slot-owed-room::swap-premise::(?isa ?a ?Ta)` | `stdlib/slots/09_owed_room` |
| `slots.ein::slot-owed-room::swap-premise::(?sub ?Ta ?super)` | `stdlib/slots/09_owed_room` |
| `slots.ein::slot-owed-fill::swap-premise::(?isa ?i ?index)` | `stdlib/slots/11_owed_fill` |
| `slots.ein::slot-owed-room::drop-neq::(neq ?Ta ?index)` | `stdlib/slots/01_partition`, `08_spatial_adjacent`, `10_owed_room_satisfied`: a seat now owes a seat |
| `slots.ein::slot-owed-fill::drop-neq::(neq ?Tv ?index)` | `stdlib/slots/01_partition`, `08_spatial_adjacent`, `12_owed_fill_satisfied`: the same, on the fill side |
| `slots.ein::slot-owed-fill::swap-premise::(?sub ?Tv ?super)` | the same three |

***Forbid* kills two of `bijective-setup`'s.** Exchanging the arguments of
`(total ?R ?isa)` or `(surjective ?R ?isa)` in its conclusion turns the
activator into `(total is-a seats)`, so `std.algebra`'s `total` (or
`surjective`) reads `seats` under `absent`. `bijection/06_blind_enumeration`
is the one program in `stdlib/bijection/` that leaves the blind enumerator on,
so `seats` is a relation a commitment can extend, and *forbid* refuses the
program at load, exit 2
([diagnostics § 2.7](../docs/ein/user/reference/diagnostics.md#27-a-refutation-resting-on-an-absent-a-commitment-can-fill)):
a refusal is a kill.

**The calculator's walks.** The modules' rules are mostly the two exchange
families: a premise's two arguments, a tag and a number, and a conclusion's.
Exchanged, a step hands its answer to nobody, the chain stops, and the
program's result claim fails for want of a row; a mutant that still answers
files its result under the operands exchanged, which a program asking an
unequal pair one way round, or both with different answers, refuses
(`decimal-arith/01_add.ein`, `08_sub.ein`, `11_cmp.ein`; a product is
symmetric in its operands, so `13_mul.ein` asks 123 × 456 both ways round and
claims each under its own operands). Every mutant of the walks is killed; what
survives in the calculator is its flags', each with its reason above.

**Some mutants are masked by whole modules.** Every import brings its module
whole, so a program that lists `symmetric` or `converse` holds `std.algebra`'s
lemmas too, and with them `(converse R1 R2)` holds both ways
(`converse-pair-symmetric`) and a symmetric relation is its own converse
(`symmetric-is-self-converse`). A `(converse …)` premise read the other way
round — `converse-illtyped-dom`, `-ran`, `compose-contravariant`'s three,
`join-converse`'s three, `std.typing`'s `type-hierarchy-converse` — matches
the mirror the lemma derives, and `symmetric` with its premise or its
conclusion exchanged leaves the mirror to `converse`, through the
self-converse lemma. The facts are those of the unmutated module, so no claim
made of facts can tell them apart, and no program can load one of those rules
without its lemma: they are banked. `converse-illtyped-dom` without its `neq`
is killed, by `stdlib/algebra/27_converse_welltyped_either_way.ein`, the
fixture of the regression `converse-lemma-subtype`.

### What holds this directory up

Two claims about it are **in `cargo test`**, by
`ein-infer/tests/stdlib_coverage.rs`:

| the claim | what fails without it |
|---|---|
| **every stdlib rule is activated by a program *here*** | a rule added to `stdlib/` with no test. It is named, with its module, so the failure says which directory the missing file goes in |
| **every program here states an expectation** | a fixture whose `:expect` was lost in a refactor, which would then load, run and pass forever |

The first is deliberately scoped to this directory and not to the corpus. The
corpus-wide version is weaker in exactly the way that matters: a rule that
happened to fire somewhere inside `examples/zebra.ein` would pass it with **no
test written**. Scoping it is also what tells a guard's test from a rule's:
`transitive`'s two-cycle, where the `(neq ?a ?c)` guard refuses every match,
tests the guard, and `algebra/21_transitive.ein` has a three-chain beside it,
where the rule fires. The suite stands on its own, which is not a number this
page has to keep, because
`stdlib_coverage.rs::every_stdlib_rule_is_activated_by_a_program` fails
the moment it stops being true.

**The rendering and determinism sweeps read them only under
`EIN_CORPUS_SLOW=1`**: the golden of every rendering
(`ein-render/tests/id_order_invariance.rs`), the job count's and the id
order's sweeps and `dot_wellformed` read every corpus program outside
`tests/` by default, and every corpus program with the variable set. By
default a program here is held by its claims, `ein test` (`corpus_cli`'s and
`test_cli`'s cells) and the coverage gate above. A program added or changed
here re-banks its golden lines with `EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test
-p ein-render --test id_order_invariance`; a program with no line at all
fails.

The third file every entry here is swept by is `corpus/corpus.toml` — five runs
each but one, so this directory's share of the sweep is
`grep -c 'path *= *"tests/stdlib' corpus/corpus.toml` entries out of
`grep -c '^\[\[entry\]\]' corpus/corpus.toml`, and `wc -l
ein.rs/crates/ein-cli/tests/golden/corpus_exits.txt` is the whole cell count.

### Counting the firings

The coverage gate is a yes/no. The numbers — firings per rule, productive
against redundant, which program is a rule's sole activator — are read off the
`fire` events a run writes (`ein test PROGRAM --events FILE`;
[the `--events` protocol](../docs/ein/ein.rs/events.md)). A firing counts for a
module only when the file does not declare that name itself and the module is
in its import closure, so a puzzle's inline `symmetric` is never credited to
`std.algebra`'s; and the arity of the activator tells `std.elim`'s
four-parameter `domain-elimination` from `std.bijection`'s two-parameter one.
That is the attribution `stdlib_coverage.rs` applies.

## What may be claimed here

**A claim that cannot be checked at the runner's own budget does not go in.**
`ein test` asks the least question that can decide a file's claims, at the
default `--region-budget`: `--single` or `-n k+1` for queries' claims over one
part of R and the description for a region claim, and then the description
for whatever that left undecided
([`syntax.md` § Query](../docs/ein/user/reference/syntax.md#query)).
So a query's claim can go in where its question decides it, on a program whose
R the description does not exhaust within the budget; a region claim on such a
program cannot. Where both questions stop short, `:expect` reports
[`NOT CHECKED`](../docs/ein/user/reference/diagnostics.md#21-a-querys-a-solves-and-a-claims-refusals)
— which is honest, is not a pass, and under `ein test` **takes a failing exit
code**. So the policy needs no mechanism: a claim that does not check cannot
be added without turning `ein test tests/` red. It is the failure mode of
adding one, not a convention to remember.

**The corollary for a new fixture**: keep it small enough that its answer is
reached by saturation or by a couple of layers. That is what every program here
already is.
