# S1f.6.32 — gcd and lcm, as `std.math.gcd`

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day. **Done 2026-11-15** ([§ Record](#record-2026-11-15)).
**Depends on:** [S1f.6.31](s1f.6.31_the_stdlib_layout.md), the layout, since the module is born under
`std.math`; and [S1f.6.24](s1f.6.24_division_by_zero_at_the_call.md), since it calls division with a tag.

## The ruling

[Question 38](questions.md#q38): *Do gcd and lcm go into `std.arith`?*, with (a) *into `std.arith`, as
`(gcd X Y)` answered by `(gcd-of X Y G)`*. The user, 2026-11-12:

> (a) but not into std.arith; it's time to make a std.math with subfolders;

So they ship, in a module of their own: `std.math.gcd`, `stdlib/math/gcd.ein`, entry `gcd`
([round 2's layout](questions_round2.md#layout), confirmed).

## What it builds

- `(gcd X Y)` answered by `(gcd-of X Y G)` and `(lcm X Y)` by `(lcm-of X Y L)`, on natural numbers of
  `std.math.decimal-arith`, the operands repeated as written, an operand a number or a name a `let` states.
- **Euclid's chain** over the calculator's tagged division, a step a `div-call`, each step's tag carrying
  what is left; the lcm as the product divided by the gcd. Six rules offered both on the prototype
  (`questions.md` 38; [`beyond_arithmetic.md`](beyond_arithmetic.md)'s question 5 and its banked example
  under `examples/beyond_arith/`).
- **Under the rule of the theory:** no rule joins two facts on a number; no `absent`, no `not`; search-free.
- **The edge the rules must say:** gcd with zero (gcd(X, 0) is X, gcd(0, 0) is zero by the usual
  convention, which the stage states and the harness holds), and lcm with zero, which is zero and must not
  divide by a zero gcd. No request of this module derives a division by zero; the test holds that.
- The entry's closing rule closes the module's relations, names its rules, and names `decimal-arith`, so one
  name brings the theory under it (E-20's remedy).

**`std.rat` is not touched by this stage.** It computes a greatest common divisor inside itself today.
Under option (b) of [round 2, item 54](questions_round2.md#q54), *reduce only on request*, `rat-simplify`
calls this module and the reason gcd lives inside `std.rat` is gone; the user replied *"(b)"* there
(`dc88520b`), and that change is
[S1f.6.33](s1f.6.33_lowest_terms_on_request.md), which runs after this stage. Here the two chains are
measured side by side, firings on the same operands, as that stage's starting figures.

## Tasks

1. **T1f.6.32.1 — the module,** from the banked example's rules, rewritten to the tagged calls the interface
   names ([S1f.6.30](s1f.6.30_name_clashes_between_imported_theories.md)'s T1); its header with the names it
   occupies; the closing rule's lists.
2. **T1f.6.32.2 — the harness,** `ein-infer/tests/gcd_theory.rs`, a sibling of `arith_theory.rs`: every pair
   to a bound and sampled long operands against Rust's integers; zero in each place; equal operands;
   operands by name; exactly one answer a request, in normal form; the rule of the theory on the text.
3. **T1f.6.32.3 — the programs** under `tests/stdlib/`, one per rule or tight family, each with its
   `:expect`; `stdlib_coverage.rs`; `arith_search_free.rs` takes the module in; the mutation census with no
   unexplained survivor; the batch by hand.
4. **T1f.6.32.4 — the example:** the banked gcd example under `examples/beyond_arith/` imports the module in
   place of its own rules, with the same claim; `examples/arith/` gains one that uses both requests.
5. **T1f.6.32.5 — the comparison with `std.rat`'s chain,** firings on the same operands, in the record.
6. **T1f.6.32.6 — the pages:** [`stdlib/README.md`](../../../stdlib/README.md), a section for the module;
   `AGENTS.md`; the manifest.

## Acceptance

- Both requests answer one fact a request and agree with the harness's integers, zero included.
- Search-free, held by the test; every rule activated by a program under `tests/stdlib/`.
- `std.rat`, `std.int` and the decimal theory are unchanged, and no existing program's answer moves.

## What it must not do

- **Not edit `std.rat`:** that is S1f.6.33's.
- **Not add inverse requests** (`solve-add`, `solve-mul`): [question 37](questions.md#q37) is not ruled in,
  and round 2 says they are *not* added to the stdlib now.
- **Not add gcd of integers or rationals.** Natural numbers only, unless the user asks.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id (gcd(0, 0) is one, with its default).
Config never changes an answer. The gate is `./run_tests.sh`, then `./run_tests.sh --nightly` after the code
commit; `utils/ci_container.sh` is not needed.

## Record, 2026-11-15

**Status: built.** T1f.6.32.1 to .6, on S1f.6.16's tree (`f69e1b65`). Commits: `8ccf48e5` (T1 to T5: the module, the harness,
the programs, the examples), `4a0d3726` (T6, the pages), and this record's. **No existing program's
answer moved**: every exit and every rendering of the corpus as it was but for `examples/beyond_arith/gcd.ein`'s,
whose text T4 rewrote to the module with the same claims.

### T1f.6.32.1 — the module, `std.math.gcd`

`stdlib/math/gcd.ein`, 18 rules and the closing rule `gcd`, which has the request's name (round 2's layout; a rule
and a relation may share a name, as a probe showed before the module was written). The banked example's six rules
(`examples/beyond_arith/gcd.ein`), rewritten to the interface's calls:

| what | rules |
|---|---|
| the requests, operands looked up by `std.math.decimal-arith`'s `operand`; the tags hold the request, so one rule a step serves both | `gcd-ask`, `lcm-ask`, `gcd-ask-second`, `gcd-start`, `gcd-answer` |
| **the tagged call** `(gcd-call TAG X Y)`, answered by `(gcd-done TAG G)` ([Q-M1f.300](../open_questions.md#q-m1f300--stdmathgcds-tagged-call-an-interface), defaulted): each number put into normal form by an addition walk to zero ([Q-M1f.301](../open_questions.md#q-m1f301--what-gcd-call-does-before-euclids-chain), defaulted), then the chain | `gcd-call-first`, `gcd-call-second`, `gcd-call-chain` |
| **Euclid's chain**, a `div-call` a step, the tag carrying what is left; a divisor is `(n P D)` in normal form, so never zero | `gcd-step`, `gcd-next`, `gcd-end` |
| **the least common multiple**, the product divided by the gcd; `lcm-zero` where the gcd is zero, which divides nothing | `lcm-start`, `lcm-zero`, `lcm-product`, `lcm-divide`, `lcm-answer` |
| an operand's fault, from the module's two tags to the request, as `std.math.int`'s three | `gcd-bad-first`, `gcd-bad-second` |

**gcd(0, 0) is zero** ([Q-M1f.299](../open_questions.md#q-m1f299--gcd0-0), defaulted to the stage's *usual
convention*): Euclid's chain gives it with no rule of its own, and lcm(0, 0) is zero by `lcm-zero`. **No request of
the module divides by zero:** the chain divides only by `(n P D)` in normal form, `lcm-divide` only by a gcd of that
shape. Under the rule of the theory: every rule one premise, no `absent`, `not` or guard, no `(false)` of its own; the
closing rule closes the module's 16 relations and names its 18 rules and `decimal-arith`, `decimal` and `arith`, the
entries of the three modules it imports (the stage named `decimal-arith`; `std.math.int` and `std.math.rat` name all
of theirs). Natural numbers only: an integer or a rational where a number goes is answered by silence.

**Why the call normalises.** The banked example's chain, and `std.math.rat`'s, divide the operands as written: an
answer that is an operand's value comes back as written, gcd(006, 0012) = `006`, and a zero written with digits,
`(n z 0)`, has the shape `(n P D)`, so `(gcd X (n z 0))` divided by zero. Two walks put both into normal form first,
so zero is `z` there and the answer is in normal form (Q-M1f.301).

### T1f.6.32.2 — the harness, `ein-infer/tests/gcd_theory.rs`

Thirteen tests, every answer held to Rust's `u128` and Euclid's algorithm, **exactly one answer a request, in normal
form, and no `division-by-zero` and no `(false)` in any program** (`hold`):

| test | cases |
|---|---|
| `every_pair_to_a_bound_is_right` | both requests over every pair from 0 to 30, both ways round: 961 each, zero on either side and both, equal operands, every divisor pair |
| `long_operands_are_right` | one to eight digits a side in every combination of lengths; four pairs of nineteen digits; four with a common factor of twelve digits; F(11)/F(10), F(26)/F(25) and F(41)/F(40), the last 39 steps |
| `zero_in_each_place` | zero written `z`, `(n z 0)` and `(n (n z 0) 0)` beside six numbers with and without high zeros, in each place, and beside itself (Q-M1f.299) |
| `equal_operands` | eight numbers against themselves, written alike and with high zeros on either side |
| `high_zeros_are_read_by_value` | sixty sampled pairs with up to two high zeros a side, and four where one divides the other, the answer an operand's value |
| `named_operands_work_for_both_requests` | ten `let`s, zero among them, by two names and by a name and a number |
| `the_tagged_call_answers_under_its_tag` | six calls from a program's own tag, high zeros and zeros; no `gcd-of` from a call |
| `an_operands_fault_is_flagged_on_the_request` | a digit, `0` and an undeclared name, in each place of each request: the flag as written, `(false)`, no answer |
| `what_is_not_an_operand` | an integer and a rational where a number goes: silence, the request beside answered |
| `the_circuit_of_each_request` | eight requests pinned, firings and depth (below) |
| `every_rule_has_one_premise`, `the_theory_writes_no_negation_and_no_guard`, `the_closing_rule_names_every_rule_and_closes_every_relation` | the module's text |

`cargo test --profile gate -p ein-infer --test gcd_theory`: 13 of 13, 3.9 s (on AC, the whole machine). **The circuits** (firings · depth, the
closing rules' firings left out; the numerals' checks counted, as every circuit of the calculator since S1f.6.38):
gcd(1071, 462) 553 · 553; gcd(462, 1071) 742 · 742, the first step turning the two round; lcm(1071, 462) 866 · 866;
gcd(5, 6) 257; gcd(7, 0) 17; gcd(0, 0) and lcm(0, 0) 14 each; gcd(F(40), F(39)) 10 337, 38 divisions. A circuit is a
sequence: depth equals firings.

`arith_theory.rs` holds the call's shape: `TAGGED` gains `(gcd-call TAG X Y)` and `(gcd-done TAG G)`, the module is
read for its declarations and its header, `stdlib/README.md` writes both, and `stdlib/math/gcd.ein` is among the
callers read (the `add-walk` it starts writes `z` where the interface does). `arith_batch.rs` gains
`a_batch_of_gcd_requests_costs_in_proportion_to_its_count`: batches of 100, 400 and 1 200 requests over numbers of one
to three digits; `cargo test -p ein-infer --features counters --test arith_batch a_batch_of_gcd -- --nocapture`
(2026-11-15): a gcd 453.5, 461.8 and 465.7 firings and 5 992.3, 6 151.6 and 6 148.0 candidates a request, an lcm
634.3, 645.2, 651.7 and 7 629.2, 7 822.4, 7 822.4: **in proportion**, within the rationals' 15 % (the chain's length
varies from pair to pair).

### T1f.6.32.3 — the programs, the census

Twelve programs under `tests/stdlib/math/gcd/`, every claim computed by Python's `math.gcd` and `math.lcm` and held
by the engine; nine claim R = {∅}, three `(false)` (an undeclared name in each place, a digit, the digit 0): `01_gcd`,
`02_gcd_zero`, `03_high_zeros`, `04_named`, `05_lcm`, `06_lcm_zero`, `07_gcd_call` (a rule of the program's own asks
whether two numbers are coprime through the call), `08` and `09` (`gcd-bad-first`, `-second` on an undeclared name),
`10` and `11` (a digit and `0`), `12_what_stays_silent` (an integer where a number goes). Every rule of the module
activated by one of them; `stdlib_coverage.rs`: every rule of the stdlib activated, **501 of 501** (`find stdlib -name '*.ein' -exec cat {} + | grep -c '^(rule '`, 2026-11-15). `arith_search_free.rs` takes the module in: `ROOTS` gains
`tests/stdlib/math/gcd`, `RESULTS` `gcd-of`, `lcm-of` and `gcd-done` (the call's hand-over, read as `div-done` is),
`THEORIES` and `DECIMAL_BASE` the import, `READS` `(gcd ` and `(lcm `; every program search-free, the three dead
ones dead as a malformed program is.

**The mutation census** (`python3 utils/stdlib_mutants.py --jobs 16`, 6 min 45 s, 2026-11-15, on AC): **494 of 613
killed**, where S1f.6.16 left 475 of 594: the module's 19 mutants (a conclusion's or a premise's arguments
exchanged) all killed on the first run, and **no new survivor**: the 119 survivors are the banked set,
`tests/mutation_survivors.txt` unmoved.

The batch by hand, the whole of it (`cargo test -p ein-infer --features counters --test arith_batch -- --nocapture`,
2026-11-15): 8 of 8, every base and module in proportion; decimal's sum and product 27.0 and 486.9 firings and
223.0 and 9 645.5 candidates a request at 250, as S1f.6.16 left them; the rationals' sum and product at 100, 651.4
and 597.9 firings, 7 526.0 and 6 972.7 candidates (S1f.6.33's starting figures).

### T1f.6.32.4 — the examples

`examples/beyond_arith/gcd.ein` imports the module in place of its six rules, each pair now asked both ways, gcd and
lcm, **its two claims as they were**, held. `examples/arith/gcd.ein`, new, both requests: the largest square tile for
a floor of 1071 by 462, gcd 21, with how many go along each wall by two divisions a rule of the program's own asks (51
and 22), and two buses every 12 and 18 minutes together again after lcm 36; search-free (`arith_search_free.rs`
reads it with the directory).

### T1f.6.32.5 — the comparison with `std.math.rat`'s chain

The same operands to three chains, each alone in a program with its own closing rules, firings and depth (a sequence:
equal), the closing rules left out; `std.math.rat`'s chain asked directly, `(rat-gcd TAG A B)`, its circuit's own
relation (a temporary test, `ein.rs/crates/ein-infer/tests/zz_tmp_chain_compare.rs`, not kept; 2026-11-15):

| A, B | `std.math.gcd`'s call | `std.math.rat`'s chain | the banked example's | steps | the call's walks |
|---|---:|---:|---:|---:|---:|
| 1071, 462 | 547 | 524 | 524 | 3 | 23 |
| 462, 1071 | 736 | 713 | 713 | 4 | 23 |
| 5, 6 | 251 | 238 | 238 | 3 | 13 |
| 6, 5 | 173 | 160 | 160 | 2 | 13 |
| 51, 22 | 331 | 314 | 314 | 3 | 17 |
| 2, 4 | 170 | 157 | 157 | 2 | 13 |
| F(40), F(39) | 10 331 | 10 288 | 10 288 | 38 | 43 |

**`std.math.rat`'s chain is the banked example's to the firing**: the same three rules over the same `div-call`. The
module's call is the chain and its two walks to normal form, some two firings a digit and three for the call's own
steps; the chain is the same steps, rule for rule (`gcd-step` and `gcd-next` fire as often as `rat-gcd-step` and
`rat-gcd-next`). Nearly all of a chain is its divisions, each computing its divisor's nine multiples first. These are
[S1f.6.33](s1f.6.33_lowest_terms_on_request.md)'s starting figures: there `rat-simplify` calls this module, and pays
the walks; the operations call nothing.

### T1f.6.32.6 — the pages

`stdlib/README.md`: the layout's row, a section `std.math.gcd` (the protocol, zero, the call, the harness, the costs
above), the modules' table, the coverage count (501 of 501, `find stdlib -name '*.ein' -exec cat {} + | grep -c
'^(rule '`, 2026-11-15). `docs/kernel/ir/03-ein-lang/07_stdlib_api.md`: a section. `tests/README.md`: the row and a
section. `examples/README.md`: both examples. `AGENTS.md`: the module in the stdlib's paragraph, `gcd_theory.rs`
among the closing rules' lists. `stdlib/MANIFEST.sha256` (24 modules). `corpus/corpus.toml`: fourteen entries.

**The goldens**, named before blessing: `corpus_exits.txt` gained 70 cells, the 14 new files' (12 programs' five
runs, the module's four, the example's six), **none moved**; `corpus_shapes.md5` gained 604 renderings of the same 14
files, and **30 moved, exactly `examples/beyond_arith/gcd.ein`'s**, whose text T4 rewrote. Lists:
`dot_wellformed.rs`' `SKIPPING` gains `tests/stdlib/math/gcd`, whose programs hold the digit tables; `alive_invariant.rs`'
`SEEDED` gains `12_what_stays_silent.ein`, which imports `std.math.int` and meets `pos` and `neg` in its division's
rules, as every program that brings it does.

### Defaults, findings

- New, each defaulted, the user's to revise: [Q-M1f.299](../open_questions.md#q-m1f299--gcd0-0) (gcd(0, 0) = 0),
  [Q-M1f.300](../open_questions.md#q-m1f300--stdmathgcds-tagged-call-an-interface) (the tagged call an interface),
  [Q-M1f.301](../open_questions.md#q-m1f301--what-gcd-call-does-before-euclids-chain) (normal form first; no
  comparison first).
- No finding, and no wrong answer of the engine met.

### What it did not do

No inverse requests; no gcd of integers or rationals; `std.math.rat`, `std.math.int` and the decimal theory
untouched (S1f.6.33 is `std.math.rat`'s). The gate is the batch's, after S1f.6.33 (the user's cadence: one gate for
the batch; [S1f.6.33](s1f.6.33_lowest_terms_on_request.md)'s record has it).
