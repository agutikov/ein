# S1f.6.24 — Division by zero is flagged at the call, whoever made it

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-13**, design (ii), the default: [the record](#record-2026-11-13). Planned
2026-11-12, on the user's reply of that day and round 2's reading of it, which the user agreed to in round 2.
**Depends on:** [S1f.6.6](s1f.6.6_division.md) (the flag as built), [S1f.6.8](s1f.6.8_rational_numbers.md)
(`std.rat`'s own rule). It edits `stdlib/arith.ein`, so it runs after
[S1f.6.12's build](s1f.6.12_malformed_numbers.md#the-build-as-ruled-2026-11-12). Independent of the stdlib
layout.

## The ruling

[Question 48](questions.md#q48) asked the user to confirm two defaults: that only the public request is
flagged, `(division-by-zero (div X Y))`, so that *a theory that calls division internally with a zero divisor
gets nothing and writes a rule of its own*; and that `std.rat` uses the same flag for any zero denominator.
The user, 2026-11-12:

> I don't understand the question: (div ?x 0) must emit division-by-zero no matter where it happens (except
> theories where it is allowed - like wheels, meadows, transreal arithmetic, etc...).

[Round 2, item 48](questions_round2.md#q48) reads it as two decisions, to which the user replied there,
committed as `dc88520b`: *"OK, I agree"*. The two:

1. **A division called from inside another theory,** `(div-call TAG X z)`, derives nothing today; by the
   reply this is wrong. *The flag moves to the division itself, so any call with a zero divisor derives
   `(division-by-zero CALL)` and `(false)`, whoever made it.*
2. **A rational written over zero,** used in a product or a comparison, is flagged as built, *and stays*. A
   `let` that names such a rational and that no request uses is not flagged.

And: *a theory in which division by zero is allowed (a wheel, a meadow) would be a module of its own that
does not import the flag's rule.* That is recorded and not built.

## What it changes

Today (S1f.6.6): `div-by-zero` derives `(division-by-zero (div X Y))`, the request as written, for a divisor
that is zero by value, and `division-by-zero-is-false` derives `(false)` from the flag, the one rule of the
theory that concludes it. `(div-call TAG X Y)` with a zero divisor derives nothing, and each theory above
writes a rule of its own; `std.rat` does (Q-M1f.163, Q-M1f.182).

After: the detection is at `div-call`. Every division, a program's `(div X Y)` and every theory's tagged
call, passes through it, so one rule flags all of them.

**What the flag names is the stage's design question,** shown on real programs before anything ships:

- **(i) the call,** `(division-by-zero (div-call TAG X Y))`, as round 2's sentence has it. One rule. The tag
  names the caller, so the flag of a public request reads `(division-by-zero (div-call (…the request's tag…) X
  Y))` where it read `(division-by-zero (div X Y))`: every surface S1f.6.6's T2 recorded moves (the unsat
  core, `refuted by`, `--trace`, `ein saturate --dump`), and the programs that claim on the flag.
- **(ii) the call, and the public request as today.** The public flag keeps its shape, derived from the
  call's; two flags for one public division.
- **(iii) the caller's request, through the tag.** Each theory states which request a tag belongs to;
  `std.int` and `std.rat` already use the request as the tag, so there the call's flag *is* about the
  request.

**Default: (ii),** because it moves no existing answer's text: a program that divides by zero in public
reads as it does today, and a theory's internal division by zero is flagged where it was silent. The record
shows the three on the same three programs, and the user may pick another.

**`std.rat`'s own rule, reconsidered.** `std.rat` derives the flag itself for a request with a
zero-denominator operand and for a `rat-div` by a rational that is zero. With the flag at the call, the task
reads which of those rules a division at the call would now also flag, and which never reach a division at
all (a product or a comparison with 3/0 performs none). The second kind stays as built, by item 2. The first
kind is removed only if the flag a program sees is the same fact as before; otherwise it stays and the
record says why two rules flag one request.

## Tasks

1. **T1f.6.24.1 — the three designs on real programs,** on a copy of the stdlib under `build/s1f.6.24/` and
   the release binary: a public `(div X z)`; a rule of a program's own that calls `(div-call (mine) X z)`;
   a `std.int` division by zero (after S1f.6.22, or the prototype's if that stage has not run); a `rat-div`
   by zero; a product with 3/0. For each design and program: the flag facts, the core, `refuted by`, the
   trace's steps. The table is the record's first section. A zero written with digits, `(n z 0)`, is zero by
   value and is among the cases.
2. **T1f.6.24.2 — the witness of today's silence,** as a test program before the change: a tagged call with
   a zero divisor that derives nothing and exits 0. **It is not a register row by default:** the register
   holds the engine's wrong answers, and here the engine answers the program as the stdlib's rules say; what
   is wrong is a rule of a theory. If the user wants a theory's wrong answers in the register, that is a
   question under the next free id, and the row is filed `open` before the fix.
3. **T1f.6.24.3 — the rule moved** in `stdlib/arith.ein`, the closing rule's lists, and `std.rat`'s rules as
   T1 decided.
4. **T1f.6.24.4 — the tests.** `arith_theory.rs`: a tagged call with a zero divisor derives the flag and
   `(false)`, under every tag shape the modules use. `arith_search_free.rs`: its three lists, `DIVIDES`,
   `RESTS_ON` and `FLAGGERS`, restated to where the flag now comes from. `rat_theory.rs` and `int_theory.rs`:
   every division by zero still one flag a request, or two under (ii), pinned. A program per changed rule
   under `tests/stdlib/`; the mutation census.
5. **T1f.6.24.5 — the callers swept.** Every program that writes `div-call` (`std.int`, `std.rat`,
   `examples/binary_arithmetic/conv.ein`, the eight of `examples/beyond_arith/` that compute through the
   tagged calls, the word problems): none divides by zero today unless it claims `(false)`; the sweep, old
   stdlib against new over every tracked `.ein` file, shows it. Every golden expected to move is named
   before blessing.
6. **T1f.6.24.6 — the pages:** the module's header and [`stdlib/README.md`](../../../stdlib/README.md)
   (*a division by zero is flagged wherever it is called*; a theory that allows it is a module that does not
   import the rule); S1f.6.6's record gets a pointer, not a rewrite; `AGENTS.md`; Q-M1f.163 marked overruled
   and Q-M1f.182 confirmed, with the user's words.

## Acceptance

- `(div-call TAG X Y)` with a divisor that is zero by value derives a `division-by-zero` flag and `(false)`,
  whoever asserted the call.
- A rational over zero in a request is flagged as before; a `let` nobody asks about is not.
- A public division by zero reads as it did on every surface, or the user chose a design under which it does
  not and the moved goldens were named first.
- No program that does not divide by zero moves.

## What it must not do

- **Not build a wheel, a meadow or transreal arithmetic.** The reply names them as theories where the flag
  does not apply; none is planned.
- **Not change what `(false)` from the flag means:** root is dead, R is empty, every command exits 0.
- **Not touch lowest terms in `std.rat`,** which waits on round 2's item 54.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id (which fact carries the flag is one, with
its default). Config never changes an answer. The gate is `./run_tests.sh`, then `./run_tests.sh --nightly`
after the code commit; `utils/ci_container.sh` is not needed.

## Record, 2026-11-13

**Built: design (ii), the default.** `std.arith` flags a division by zero at the tagged call, whoever made
it, and a public request as it always did. One rule more, `div-call-by-zero`; 115 rules and the closing
rule (`grep -c '^(rule ' stdlib/arith.ein`, less one, 2026-11-13). `std.int`, `std.rat` and `std.num9` are
unchanged but for two header paragraphs. Q-M1f.211 to .213 defaulted, the user's to revise; E-39 filed. The
register is unmoved. Every command below was run on 2026-11-13 from the repo root; the scratch is
`build/s1f.6.24/`, gitignored.

### T1: the three designs on nine programs

`python3 build/s1f.6.24/proto.py` writes four copies of the stdlib of `326319f5`, one a design, and
`sh build/s1f.6.24/t1.sh > build/s1f.6.24/t1.txt` runs the release binary of `326319f5` on each program
under each: `ein saturate --dump` for the flag facts, `ein solve --trace` for the core, `refuted by` and
the trace's steps. All three designs carry the divisor as the call wrote it in the first multiple's tag,
`(div-m1 TAG X Y)` where `(div-m1 TAG X)`, since the flag of a call repeats the call. (ii) was tried two
ways: the request's flag derived from the divisor's normal form, as today, with the call's rule written
after it; and (ii-b), the request's flag derived *from* the call's.

The programs (`build/s1f.6.24/p/`): `pub_z`, `(div (n z 7) z)`; `pub_spelt`, `(div (n z 7) none)` with
`(let none (n (n z 0) 0))`; `own_z` and `own_spelt`, a rule of the program's own, `share-ask`, that asserts
`(div-call (mine A B) A B)` from `(share A B)`, with the divisor `z` and `(n (n z 0) 0)`; `own_fine`, the
same with the divisor 2; `int`, `(int-div-trunc 7 (int 3 3))`; `rat_div`, a `rat-div` by 0/5; `rat_mul`, a
product with 3/0; `rat_let`, a `let` naming 3/0 that no request uses.

| program | today (`326319f5`) | (i) the call | (ii) the call, and the request | (iii) the caller's request |
|---|---|---|---|---|
| `pub_z` | flag `(division-by-zero (div (n z 7) z))`; core `(div (n z 7) z)`; `refuted by division-by-zero-is-false`; 25 trace steps, the last two the flag and `(false)` | flag `(division-by-zero (div-call (quotient (n z 7) z) (n z 7) z))` alone; core and `refuted by` as today; 25 steps, the last two respelt | **both flags**, the request's first; core and `refuted by` as today; 27 steps: the request's flag, the call's, `(false)` from each | a marker `(div-zero (quotient …) (n z 7) z)`, then the request's flag; core and `refuted by` as today; 26 steps |
| `pub_spelt` | flag names `(div (n z 7) none)`; core the request, the `let` and `(md 0 1 0 0 0)`; 32 steps | the call's flag, the divisor as written, `(n (n z 0) 0)`; core as today; 32 steps | both; core as today; 34 steps | marker and the request's flag; core as today; 33 steps |
| `own_z` | **no flag, root alive, exit 0**; with `(solve :expect (false))` the claim FAILED, exit 1 | flag `(division-by-zero (div-call (mine (n z 7) z) (n z 7) z))`, `(false)`; core `(share (n z 7) z)`; `refuted by division-by-zero-is-false`; 20 steps | the same as (i) | **the marker and nothing else: root alive, exit 0**, the claim FAILED |
| `own_spelt` | no flag, root alive | the call's flag, the divisor as written; core the `share` fact and `(md 0 1 0 0 0)` | the same as (i) | the marker, root alive |
| `own_fine` | answered, 101 steps | the same | the same | the same |
| `int` | flag `(division-by-zero (int-div-trunc …))` by `int-div-by-zero`; 58 steps | the same on every surface | the same | the same |
| `rat_div` | flag `(division-by-zero (rat-div …))` by `rat-div-zero`; 25 steps | the same | the same | the same |
| `rat_mul` | flag `(division-by-zero (rat-mul …))` by `rat-zero-first`; 10 steps | the same | the same | the same |
| `rat_let` | no flag, R {∅} | the same | the same | the same |

What the table says:

- **`ein solve` reads the same under all three for a public division by zero**: the core is written
  facts, and the request is the written fact under every design. What moves is the flag's text in
  `--trace` and `ein saturate --dump`: respelt under (i), one fact and two steps more under (ii), a marker
  and one step more under (iii).
- **(iii) leaves a caller that writes no rule alive and silent**, which is what the ruling overruled, so
  it is not the default whatever else it has.
- **(ii) against (ii-b):** the same facts; (ii) derives the request's flag first, so a trace reads the
  request's flag, the call's, then the two contradictions, and the first justification of `(false)` is
  the request's, as today. (ii) is what is built.
- **`std.int` and `std.rat` never call the division with a zero divisor**, so no design touches them:
  `std.int` sees the zero by the divisor's shape, `(int z z)` after simplifying, before its four calling
  rules, each of which matches a divisor `(n ?b ?c)` in normal form; `std.rat` compares each denominator
  with zero as the operand is looked up, sees a zero divisor of `rat-div` at its numerator, and its own
  three calls (Euclid's chain, `rat-gcd-step`, and lowest terms' two) divide by a remainder or a common
  divisor that is not zero. **So none of `std.rat`'s four flagging rules is of the first kind the stage
  file names, the kind a division at the call would now also flag, and all four stay as built**
  (`rat-zero-only`, `-first`, `-second`, `rat-div-zero`); a request has one flag there, as before, and
  `rat_theory.rs` and `int_theory.rs` hold the set of flags exactly.

The default stands as [Q-M1f.211](../open_questions.md#q-m1f211--which-fact-carries-the-flag-of-a-division-by-zero-at-the-call).

### T2: the witness of today's silence

`build/s1f.6.24/p/own_silent.ein`, `own_z` without its claim, on the stdlib and binary of `326319f5`:
`ein solve` prints `states (k_s) 1`, *∅ in R — root is alive and owes nothing*, exit 0, and `ein saturate
--dump` holds no `division-by-zero` fact. The share among nobody has no `share-of` fact and nothing says
so. The same program is banked with its claim as
[`tests/stdlib/arith/39_div_call_by_zero.ein`](../../../tests/stdlib/arith/39_div_call_by_zero.ein), which
fails on the old stdlib (`(solve :expect) FAILED — (false) says every node is dead, and R has 1 state`) and
holds on the new. **No register row**, the stage file's default:
[Q-M1f.212](../open_questions.md#q-m1f212--a-theorys-wrong-answer-and-the-register).

### T3: the rule, as built

In [`stdlib/arith.ein`](../../../stdlib/arith.ein):

- `(relation div-m1 Tag Number Number)`, the divisor as the call wrote it beside the dividend;
  `div-multiples` writes it and `div-multiple-2` reads past it. The facts of the first multiple's walk are
  respelt in every division's state; none is added and no firing (the circuit pins of a division that is
  not by zero are unmoved, `(228, 220)` for 127645 ÷ 25).
- `div-by-zero`, the request's flag, unchanged but for the tag's third place.
- **`div-call-by-zero`**, new, written after it: `(done (div-m1 ?k ?u ?v) z)` gives
  `(division-by-zero (div-call ?k ?u ?v))`. One premise, no guard, no `absent`.
- `division-by-zero-is-false` unchanged: it fires once a flag, so a public division by zero derives
  `(false)` twice, the second a redundant firing. 127645 ÷ 0 is 18 firings where 16, no deeper.
- The closing rule names the new rule; no relation was added.

`std.rat` and `std.int`: no rule changed, as T1 decided. `std.num9` divides by a rule of its own that
derives `std.arith`'s flag and never calls `div-call`; unchanged. **The binary example theory** is
generated from this file and has a tagged division of its own, `div-call2`: the generator carries the rule
over, `div-call-by-zero2`, 68 rules where 67
([Q-M1f.213](../open_questions.md#q-m1f213--the-binary-example-theory-and-the-flag-at-the-call);
`python3 examples/binary_arithmetic/gen_binary.py --check`, exit 0).

**What a program sees for a division by zero, by who made it:**

| who | flags in root's state | `ein solve` |
|---|---|---|
| a request, `(div X Y)` | `(division-by-zero (div X Y))` and `(division-by-zero (div-call (quotient X Y) U V))`, U and V the numbers the operands stand for | core the request (and the `let`s and table rows the zero rests on), `refuted by division-by-zero-is-false`; as before |
| `std.int`, `std.rat`, `std.num9` | the request's, one, by the module's own rule; as before | as before |
| a rule of a program's own, `(div-call TAG X Y)` | `(division-by-zero (div-call TAG X Y))`, the call as asserted | root dead, R empty, exit 0; the core is the program's own fact the call came from; **until the stage: nothing, root alive** |

### T4: the tests

- `arith_theory.rs`: **`a_tagged_call_by_zero_is_flagged_whoever_made_it`**, new: calls written as facts
  under seven shapes of tag (a bare name, `std.arith`'s `(quotient …)`, `std.int`'s `(int-div-m …)`,
  `std.rat`'s `(rat-gcd-rem …)`, `(rat-q-num …)` and `(rat-q-den …)`, and an example's `(tr-div …)`), three
  dividends by three spellings of zero each: exactly one flag a call, the call as asserted, `(false)`, no
  `div-done`; the calls by 3 beside them answered and not flagged. `a_division_by_zero_is_flagged_and_false`
  now asks for exactly two flags a public request. The circuit pin of 127645 ÷ 0 is `(18, 10)` where
  `(16, 10)`. The static tests needed no change and pass: no join on a number, one `absent`, three rules
  that conclude `(false)`, the closing rule's lists.
- `arith_search_free.rs`, its three lists restated: `DIVIDES` gains `(share `, the fact a program's own
  rule makes its call from; `FLAGGERS` gains `("(division-by-zero (div-call ", "div-call-by-zero")`;
  `RESTS_ON` is unmoved. The dead-program check now reads **every** flag in root's state and holds each to
  its rule, and holds the count: two for a public `div`, one a call for a program's own, one and no call
  for a module's. `RESULTS` gains `div-done`, the tagged division's hand-over, which is the answer `41`'s
  rule reads.
- `int_theory.rs` and `rat_theory.rs`: both already compare the *set* of flags with the requests', so one
  flag a request is pinned as it was; each test's comment now says why no call's flag stands beside it.
  Both pass unchanged.
- Three programs under `tests/stdlib/arith/`, each a corpus entry: `39_div_call_by_zero.ein` (a program's
  own rule, the divisor `z`), `40_div_call_by_zero_spelt.ein` (the divisor 00, the `md` row in the core),
  `41_div_call_not_zero.ein` (the same rules, no zero, one divisor with a high zero and one dividend zero:
  the rule loaded and silent, each call answered). `stdlib_coverage.rs` passes.
- The batch, by hand: `cargo test --manifest-path ein.rs/Cargo.toml -p ein-infer --features counters --test
  arith_batch -- --nocapture`, 5 passed (`build/s1f.6.24/arith_batch.txt`): `div` 6 033.98, 6 030.25 and
  6 030.94 candidates a request at 250, 1 000 and 4 000, [S1f.6.6's](s1f.6.6_division.md#t3-record-2026-11-11)
  numbers to the last digit.

### T5: the callers swept, and the goldens named before blessing

**The goldens expected to move, named here before `EIN_BLESS=1`** (from `cargo test --workspace
--no-fail-fast`, `build/s1f.6.24/ws1.log`, which failed in three tests and no other):

- `ein-cli/tests/golden/corpus_exits.txt`: 15 new cells, the five runs of each of `39`, `40` and `41`,
  every one exit 0. No existing cell moves.
- `ein-render/tests/golden/corpus_shapes.md5`: the renderings of every program that holds `std.arith`'s
  rules, by the one rule more and its `(arith-rule div-call-by-zero)` fact, and of the three new programs.
- `ein-infer/tests/alive_invariant.rs`'s list, not a blessed golden: one entry, `40`, whose `z` is only
  inside a number.

**Blessed** (`EIN_BLESS=1 cargo test -p ein-cli --test corpus_cli` and `-p ein-render --test
corpus_shapes`), and what moved is what was named: `corpus_exits.txt` gained its 15 cells and lost none;
`corpus_shapes.md5` moved in 2 308 renderings of 112 files and gained 126, the three new programs', none
gone. Every one of the 112 files imports the theory, `std.arith`, a module over it or an example theory
that does; the kinds are the renderings that hold the rules or root's facts (`load`, `ir`, `plan`,
`match`, `saturate`, `explain`, the `dot` views, and `states`, `oracle` and `region-trace`, which print
the state's facts, the new `(arith-rule div-call-by-zero)` among them).

**The sweep**, the release binary and stdlib of `326319f5` against the tree's, `ein saturate --dump` and
`ein solve` over the 834 `.ein` files `326319f5` tracks (`python3 build/s1f.6.24/sweep.py --diff`,
`build/s1f.6.24/sweep.txt`):

- **`ein solve` moves on one file, and not by an answer:** `examples/broken/load/import_module_not_found.ein`,
  a refusal that prints the stdlib's path, which differs between the two runs by construction.
- **`ein saturate --dump` moves on 106:** that file again, by the path; **102 programs that hold
  `std.arith`'s rules and do not divide by zero**, by one fact more, `(arith-rule div-call-by-zero)`, the
  rule's line in the strata list, and the counts in the statistics; those of them that divide show the
  facts of the first multiple's walk respelt, the divisor now in its tag, as many gone as came, and no
  other fact gone or come; and **the three that divide by zero in public**, `examples/arith/div_by_zero.ein`,
  `tests/stdlib/arith/20_div_by_zero.ein` and `21_div_by_zero_spelt_and_named.ein`, by those and the
  call's flag, `(division-by-zero (div-call (quotient …) …))`, with one more redundant firing of
  `division-by-zero-is-false`.
- **No caller divides by zero unless it claims `(false)`:** no program under `examples/beyond_arith/`,
  `examples/word_problems/` or `examples/arith/` other than `div_by_zero.ein` gained a flag, and none
  changed its exit or its `ein solve`.
- **The binary examples** read `bin.ein` beside them, which the tree regenerated, so the sweep above
  compares them with themselves. Compared apart, the tree of `326319f5` against the working tree
  (`bash build/s1f.6.24/binsweep.sh`, `build/s1f.6.24/binsweep.txt`): `ein solve` moves on none of the
  files under `examples/binary_arithmetic/` and `examples/binary_logic/`; `ein saturate --dump` moves on
  seven, all under `binary_arithmetic/`, by `(arith-rule2 div-call-by-zero2)`, and on `div_by_zero.ein`
  by the call's flag too.

**The mutation sweep** (`python3 utils/stdlib_mutants.py`, `build/s1f.6.24/mutants.txt`): 437 of 500
where 437 of 499. The new rule has one mutant, its premise's two arguments exchanged, and `39` and `40`
kill it. **One survivor is new and banked by hand in `tests/mutation_survivors.txt`:** `div-by-zero` with
its premise exchanged, which `20_div_by_zero.ein` killed until now. Under (ii) a public division by zero
has two flags, either of which is `(false)`, and a dead program's one claim cannot say which killed it:
the cost of the design, of the same kind as S1f.6.12's twelve. `arith_search_free.rs` and
`arith_theory.rs` hold the request's flag from outside the language (T4). `tests/README.md` says so.

### T6: the pages

`stdlib/arith.ein`'s header and the comment over the two rules; `stdlib/README.md` (the division-by-zero
row: *flagged wherever it is called*, and a theory that allows it is a module that does not import the
flag's rules; the rule count; the index row); `int.ein`'s and `rat.ein`'s headers, why each keeps its own
rule and has one flag; `examples/arith/div_by_zero.ein`'s header and its line in `examples/README.md`;
the binary example's generator, README and corpus note, 68 rules; `tests/README.md`; S1f.6.6's T2 record,
a pointer; Q-M1f.163's row, *built*, and Q-M1f.182's, *left as built*; the phase README's row.
**`AGENTS.md` is not edited:** none of its standing rules, locations or commands became untrue.
`stdlib/README.md` § What a program pays was **not re-taken**: its one edited number is the count of
rules, 116 with the closing rule, and its timings are S1f.6.12's of the same day.

### Found

- **E-39** of the phase's [`findings.md`](findings.md): a tagged call written under `std.arith`'s own tag,
  `(div-call (quotient A B) U V)`, is taken for a request nobody stated, answered or flagged as `(div A B)`.
  Before the stage and after alike; not a wrong answer of the engine.
- **Not touched, and still open:** [Q-M1f.210](../open_questions.md#q-m1f210--stdint-and-stdrat-and-the-flags)
  and E-38. A malformed or undeclared operand under a module's tag is as silent as it was; this stage
  flags a *division by zero* under any tag and nothing else, and `std.int` and `std.rat` still have no
  rule from their tags to a request. A tagged call whose numbers are malformed is not checked either:
  the check is at `std.arith`'s operand, and a call goes straight to the walks.
- **A wheel, a meadow, transreal arithmetic:** recorded as the user named them, not built. `std.num9`
  shows the shape such a module would have from the other side: it imports the flag and its rule by
  name, `(import std.arith :symbols (n division-by-zero division-by-zero-is-false))`.

### Acceptance re-taken

- *`(div-call TAG X Y)` with a divisor that is zero by value derives a flag and `(false)`, whoever
  asserted the call:* `a_tagged_call_by_zero_is_flagged_whoever_made_it`, and `39` and `40` on the binary.
- *A rational over zero in a request is flagged as before; a `let` nobody asks about is not:* T1's
  `rat_mul` and `rat_let`, `rat_theory.rs` unchanged and passing.
- *A public division by zero reads as it did on every surface, or the user chose otherwise:* `ein solve`
  reads as it did, to the byte (the sweep). `--trace` and `ein saturate --dump` gain the call's flag
  under the default design, (ii), which is the stage file's default and the user's to revise; the moved
  goldens were named first.
- *No program that does not divide by zero moves:* none in `ein solve`; in `ein saturate --dump` by the
  rule's name and a walk's tag, no fact gone.

### The gate

`./run_tests.sh`, green, exit 0 (`build/gate-s1f.6.24.log`, 2026-11-13), on the tree as committed, after
one red run on `cargo fmt` alone. `python3 utils/stdlib_mutants.py --check` exits 0 on the bank as edited
by hand (`build/s1f.6.24/mutants-check2.txt`). `utils/ci_container.sh` was not run: no DOT writer, review
bundle or CI install changed.

### After the commit: the nightly

`./run_tests.sh --nightly` on `eddff460`, the build's commit: green in every step, exit 0
(`build/nightly-s1f.6.24.log`, 2026-11-13), `utils/stdlib_mutants.py --check` among them, 437 of 500 on
the tree as committed, so the survivor bank edited by hand is held. `utils/ci_container.sh` was not run.
