# S1f.6.3 — Addition, and the machinery every circuit shares

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-11**: `stdlib/arith.ein`, 19 rules, addition over nested numbers with named
constants, held to Rust's integers, search-free, and the rule of the theory held on the text and on a counter;
six decisions taken by default, Q-M1f.149 to .154, and one finding, E-11 ([§ T1 record](#t1-record-2026-11-11)
onward). Planned
2026-11-08; re-planned 2026-11-11 to [the rulings](design.md#the-rulings-2026-11-11).
**Depends on:** [S1f.6.2](s1f.6.2_digits_and_tables.md).

## What it builds

`stdlib/arith.ein`, imported as `std.arith`, begins here: the number, the protocol, the machinery, and the first
operation.

**The number** (the user: *"reading order"*). A natural number is `z`, zero, or `(n PREFIX DIGIT)`: ten times
the prefix, plus the digit. 123078 is `(n (n (n (n (n (n z 1) 2) 3) 0) 7) 8)`. The normal form has no high
zero. Any length; no width and no overflow.

**The protocol** (the user: *"ok, result facts like (sum-of X Y S) can be reused for other purposes, the goal
then has to be (sum-of x y ?s) with a variable on the result place"*). A request fact, `(add X Y)`, is answered
by a result fact that repeats its operands, `(sum-of X Y S)`. A program reads the answer with a goal that has a
variable in the result's place.

**The machinery,** design K of
[design.md § Round 3](design.md#k-an-accumulator-in-the-other-order-and-a-continuation-on-every-call):
- **One walk from the units,** each column through the table, its digit put in front of an accumulator. The
  accumulator holds the answer most significant digit outermost, a second shape, `(b DIGIT REST)`, that only the
  theory writes.
- **High zeros are dropped there,** where they are outermost.
- **Then the answer is turned round** into the public shape and handed over as `(done TAG ANSWER)`.
- **Every call carries its continuation** as a tag, a nested fact holding what the caller needs next.

**The rule of the theory: no rule joins two facts on a number.** A rule has one premise that holds numbers; its
other premises are table rows looked up by digits. This is what makes a batch of requests cost in proportion to
their count, where an ordinary join on a nested value scans its relation (E-3 of [`findings.md`](findings.md)).
It is stated in the module's header and held by a test (task 4).

**Addition:** `(add X Y)` asks for the walk with the tag `(sum X Y)`, and `(done (sum X Y) S)` is
`(sum-of X Y S)`. Operands with high zeros are read by value; the answer is in normal form.

## Tasks

1. **The module's start:** `n`, `z`, the accumulator shape, the strip, the turn, the walk for addition, the
   import of `std.digits`, and one closing rule that closes every relation of the theory and names every rule,
   so `(import std.arith :symbols (arith))` brings the theory.
2. **Named constants** (the user: *"(let NAME NUMBER) is ok"*). `(let price N)` names a number, and a request
   may name its operand: `(add price qty)` is answered by `(sum-of price qty S)`. The stage's default for what
   that means is in [the rulings § what is open](design.md#what-is-open-after-the-rulings), item 2, and it is the
   user's to overrule before this task is written.
3. **The tests.**
   - One program per rule under `tests/stdlib/arith/`, each with its `:expect`.
   - A harness test against the integers: random operands of every length up to a stated bound, operands with
     high zeros, and every answer in normal form.
   - **The search-free test,** which every later operation joins: H empty, k_s 1, R {∅}, and the same answer
     under `ein solve`, `ein saturate` and `ein test`.
4. **The test that holds the rule of the theory.** Two parts.
   - Static: every rule of `std.arith` has at most one premise whose relation is not a table of `std.digits`.
   - Measured: a batch of 250 and of 1 000 requests, and a deterministic count of the matcher's work per request
     (a counter of `ein_core::counters`, which the stage picks and names, not the wall clock) that does not grow
     with the batch.
5. **The circuit's measure:** firings and depth of one addition, with the command that re-takes them.
6. **The example:** `examples/arith/` gains the addition program of `design.md`, respelt, with its corpus entry.

## Acceptance

- 123078 + 4567 is 127645, and 999999999 + 1 has an answer of ten digits.
- The harness test, the search-free test and the test of the rule of the theory pass.
- No `absent`, no `not`, one stratum.

## T1 record, 2026-11-11

`stdlib/arith.ein`, written by hand
([Q-M1f.152](../open_questions.md#q-m1f152--are-stdariths-rules-written-by-hand-or-generated), defaulted): 16
relations and 19 rules (`grep -c '^(relation' stdlib/arith.ein`, `grep -c '^(rule' stdlib/arith.ein`,
2026-11-11), one stratum, no `absent`, `not`, `or` or guard.

| what | relations | rules |
|---|---|---|
| the number and the accumulator | `n`, `b`; `z` is a name | — |
| the protocol | `let`, `add`, `sum-of` | — |
| the operands | `named`, `operand`, `operand-is` | `operand-zero`, `operand-number`, `let-named`, `operand-name` |
| the walk every adding circuit shares | `add-walk` | `add-column`, `add-column-first`, `add-column-second`, `add-end`, `add-carry-out` |
| the normal form and the turn | `strip`, `turn`, `done` | `strip-zero`, `strip-top`, `strip-none`, `turn-step`, `turn-end` |
| addition | the tags `sum`, `add-first`, `add-second` | `add-ask`, `add-ask-second`, `add-start`, `add-answer` |
| the entry | `arith-rule` | `arith`, the closing rule |

The design is K's, in the ruled spelling: where the prototype wrote `(d DIGIT NUMBER)` the module writes
`(n PREFIX DIGIT)`, and the units are outermost in both, so the walk peels a column a firing. Three things
differ from `build/s1f.6/prototype/r3/k.ein`:

- **the relations have words for names**, `add-walk`, `strip`, `turn`, where the prototype had `ak`, `sb`, `bk`
  ([Q-M1f.151](../open_questions.md#q-m1f151--the-names-of-the-circuits-own-relations), defaulted); `done`,
  `sum` and `b` are the stage file's;
- **a carry out of the top goes straight to the turn**: the top digit is a 1, so there is no high zero to look
  for, and the firing the prototype spent there is not spent;
- **each operand is looked up before the walk**, which is task 2's, below.

**How H is kept empty.** `arith` has an empty `:match` and asserts `(__closed__ R)` for each of the module's 16
relations and for `fa` and `nz`, the two tables it imports, and `(arith-rule NAME)` for each of the other 18
rules. A selection's closure follows the names a kept declaration writes, so `(import std.arith :symbols
(arith))` brings every rule, every relation, and the two tables with their rows; and a `(__closed__ R)` a rule
derives at root closes as a written one does. `arith_theory.rs`'s
`the_closing_rule_names_every_rule_and_closes_every_relation` holds the two lists to the module, exactly, and
every program under `tests/stdlib/arith/` claims R = {∅} with no `:hypothesis-relations` list.

**The module writes one fact, `(arith-rule arith)`,** the closing rule's own name, and it is there for E-11 of
[`findings.md`](findings.md). As first written the module had none, and `ein-cli`'s `answer_round_trip.rs`
failed on every program of the theory: the program a `--dump-answer` state file re-writes goes through the
tree-shake, which keeps a rule that a name or a premise reaches, and nothing reaches a rule that matches on
nothing and is named only by the importer's `:symbols`. The state file held every fact of the state and neither
`(rule arith …)` nor `(relation arith-rule …)`. A selection brings the written fact, since `arith` writes
`arith-rule`, and the tree-shake follows the name in it. `arith_theory.rs` holds that the module writes that
fact and no other.

## T2 record, 2026-11-11

**What `(let NAME NUMBER)` does**
([Q-M1f.149](../open_questions.md#q-m1f149--what-let-name-number-does-as-built), defaulted, the reading of
[the rulings § what is open](design.md#what-is-open-after-the-rulings), item 2). A request's operand is a number
or a name. `add-ask` asks for the first operand's number, `(operand (add-first X Y) X)`, and three rules answer
with `(operand-is TAG NUMBER)`: `z` and `(n …)` stand for themselves, and a name for the number its `let`
states. The second operand the same way, and then the walk starts with the two numbers and the tag `(sum X Y)`,
which holds the operands **as written**, so `(add price qty)` is answered by `(sum-of price qty S)` and the same
sum by numbers is another request.

- **A name no `let` states** is an operand never found: no walk, no answer, no word
  (`tests/stdlib/arith/07_only_what_is_asked.ein`). Whether that is flagged is S1f.6.12's.
- **A name stands for a number**, not for another name: `(let a b)` answers `(operand-is TAG b)`, and the walk
  matches nothing on it.
- **It costs four firings a request**, numbers too: 21 for 123078 + 4567 where the prototype's K took 17.
- **It is the one stated exception to the rule of the theory,** and its premises are ordered so that it is not
  one in cost. `operand-name` is `(and (operand ?k ?x) (named ?x) (let ?x ?v))`. Written without `(named ?x)`,
  as it first was, an operand that is a number seeded the rule and walked every `let` fact, since the index
  does not key the nested fact it was asked with: 500 requests beside 200 `let`s cost 200 000 candidates more
  than beside none, the product (`cargo test -p ein-infer --features counters --test arith_batch --
  --nocapture`, 2026-11-11, on that spelling). `(named ?x)` has every argument bound, and **the engine answers a
  premise with every slot bound by one interned lookup, whatever the arguments are**; a number stops there.
  With it the same requests cost 90 694 candidates beside no `let` and 90 894 beside 200, the 200 being the
  `let`s' own firings. `let-named` derives `(named NAME)` from each `let`.

That last finding qualifies E-3 of [`findings.md`](findings.md), and is noted there: a join on a number scans
only where the joined premise has an unbound slot. A second premise that repeats a whole fact costs one lookup.
The rule of the theory stays as ruled and as the static test reads it, one premise that holds numbers; the
measurement is recorded for the stages that follow.

## T3 record, 2026-11-11

- **Seven programs** under `tests/stdlib/arith/`, each a corpus entry of group `stdlib`:
  `01_add` (123078 + 4567 = 127645), `02_carry_out` (999999999 + 1, ten digits), `03_lengths` (different
  lengths, both ways round), `04_zero`, `05_high_zeros`, `06_named`, `07_only_what_is_asked`.
  `stdlib_coverage.rs` holds that the 19 rules are each activated by one of them: 96 of 96 over the stdlib
  (`cat stdlib/*.ein | grep -c '^(rule '`, 2026-11-11).
- **The harness against the integers,** `ein-infer/tests/arith_theory.rs`
  ([Q-M1f.154](../open_questions.md#q-m1f154--the-bounds-of-the-harness-against-the-integers), defaulted). The
  oracle is `u128`. Every pair from 0 to 120, 14 641 sums; every pair of lengths from 0 to 30 digits, twice,
  1 922 sums drawn by a fixed-seed SplitMix64; 99…9 + 1, 1 + 99…9 and 99…9 + 99…9 at every length to 30; and
  1 500 sums of operands of up to 12 digits padded with up to three high zeros, with zero written as zeros.
  Each request has **exactly one** answer, and it is the normal form of the integers' sum.
- **The search-free test,** `ein-cli/tests/arith_search_free.rs`, which a later operation joins by adding its
  result relation to one list: every `.ein` under `tests/stdlib/arith/` and `examples/arith/`, on the binary.
  `--json-summary`: no candidate, nothing outside the universe, k_s 1, k_c 1, root in R, one state and it is ∅,
  one saturation, monotone, no `absent` read. `ein saturate --dump`: *stratified, 1 stratum*. The `sum-of` facts
  of the solve's one state file and of the saturation's dump are the same set. `ein test`: every claim held.
- **The mutation sweep** kills every mutant of the module, 21 of 21: 188 of 238 where 167 of 217, the bank's 50
  survivors unmoved (`python3 utils/stdlib_mutants.py --check`, 2026-11-11).

## T4 record, 2026-11-11

**Static,** `arith_theory.rs`'s `no_rule_joins_two_facts_on_a_number`, on the module's text: every rule has at
most one premise that is not a row of `std.digits`, and a row's arguments are names or variables, never a form.
`operand-name` is named as the exception and held to its three premises in their order.
`the_theory_writes_no_negation` holds that no rule writes `absent`, `not`, `or`, `neq` or `false`. Tried: a
second premise `(add ?x ?y)` in `add-answer` fails the first, naming the rule.

**Measured,** `ein-infer/tests/arith_batch.rs`
([Q-M1f.150](../open_questions.md#q-m1f150--the-counter-and-how-the-measured-test-is-in-the-gate), defaulted).
The counter is **`candidates`** of `ein_core::counters`: the facts the matcher offered to a premise and unified
against. Batches of 250, 1 000 and 4 000 requests over two nine-digit numbers:

| requests | firings | a request | candidates | a request |
|---:|---:|---:|---:|---:|
| 250 | 6 751 | 27.00 | 45 346 | 181.38 |
| 1 000 | 27 001 | 27.00 | 181 384 | 181.38 |
| 4 000 | 108 001 | 27.00 | 725 577 | 181.39 |

(`cargo test -p ein-infer --features counters --test arith_batch -- --nocapture`, 2026-11-11.) The test holds
that the count a request does not grow by more than a twentieth from 250 to 1 000 and to 4 000. Tried: with
`(operand-is (add-first ?x ?y) ?u)` as a second premise of `add-answer`, a join that leaves a slot unbound, the
three rows read 681, 2 181 and 8 181 candidates a request and the test fails, every answer unchanged.

**The counters are compiled out of every build but one,** so the test is in the gate twice. Under `cargo test
--workspace` it is built without them and holds what it can, the firings a request. The step after it,
`cargo test -p ein-infer --features counters --test arith_batch`, new in `run_tests.sh` and `per-commit.yml`
and held to each other by `gate_steps.rs`, builds `ein-infer` once more with the feature and holds the count.
It took 15 s the first time and takes 2 s warm here (2026-11-11).

## T5 record, 2026-11-11

`arith_theory.rs`'s `the_circuit_of_one_addition` pins the measure and is the command that re-takes it (`cargo
test -p ein-infer --test arith_theory the_circuit`):

| request | firings | depth | what they are |
|---|---:|---:|---|
| 123078 + 4567 | 21 | 21 | 5 to ask and find the operands, 6 columns, the end of the walk, the check for a high zero, 6 digits turned, the hand-over, the answer |
| 999999999 + 1 | 27 | 27 | 5, 9 columns, the carry out, 10 digits turned, the hand-over, the answer |
| the first, by two names | 23 | 21 | the same, and a firing for each `let` |

Depth is the longest chain of firings, a fact being one deeper than the deepest premise of the firing that
derived it. A circuit of design K is a sequence, so its depth is its size. The request adds 22 facts to the
store, itself included.

On the release binary (`python3 build/s1f.6.3/batch.py`, 2026-11-11), random sums of two numbers of up to nine
digits in one program:

| | the saturator | firings | the process |
|---|---:|---:|---:|
| 250 sums | 6.2 ms | 5 335 | 35 ms |
| 1 000 sums | 24.4 ms | 20 805 | 86 ms |
| 4 000 sums | 109.6 ms | 82 093 | 307 ms |
| 4 000 sums beside 4 000 `let`s | 112.6 ms | 86 687 | 404 ms |

The prototype's K took 6.9, 28 and 115 ms for the same batches.

**What a program pays to load.** One that does not import the theory, nothing. One that does parses
`std.digits`, all of it (Q-M1f.148): `ein test tests/stdlib/arith/01_add.ein` takes 20.0 ms where a `std.macro`
test takes 5.2 ms. `ein test tests/` takes 276 ms where it took 171 ms before the seven programs, and
`stdlib_coverage.rs` half a second where a quarter (the same script, and the gate's logs).

## T6 record, 2026-11-11

`examples/arith/add.ein`: the addition program of `design.md`, respelt, with a second request by two names,
each read by a goal with a variable in the result's place. Corpus group `positive`, runs `solve`, `test`,
`saturate`, `saturate --dump` and `solve --trace`.

**The DOT sweep's bound moved to 1 000 edges**
([Q-M1f.153](../open_questions.md#q-m1f153--the-dot-sweeps-bound-under-the-smallest-table), defaulted). The KB
view of a program that imports `std.arith` holds `fa`'s 200 rows, some 1 200 edges; `dot` draws one in 0.55 s
and each program has six. Under S1f.6.2's bound of 1 500 the eight programs would have added 26 s to the sweep.
`dot_wellformed.rs` takes 180 s (2026-11-11). S1f.6.2's commit was run in the runner's image before this stage:
`utils/ci_container.sh --tests-only -p ein-render --test dot_wellformed` on `3bd448a5`, green, 126.8 s.

**The goldens that move,** named before `EIN_BLESS=1`: `ein-cli/tests/golden/corpus_exits.txt`, 44 new cells
(the module's four, the seven programs' five each, `render rules` exiting 1 on a program that writes no rule of
its own as on the 49 others of `tests/stdlib/`, and the example's five), and
`ein-render/tests/golden/corpus_shapes.md5`, the nine new files' lines alone.

**The sweep.** `python3 build/s1f.6.3/sweep.py` (2026-11-11), the release binary built at `ed398cc5` against
the tree's, `ein saturate --dump` and `ein solve` over the tree's 681 `.ein` files: 0 files moved. No engine source
changed in either stage.

Pages: `stdlib/README.md` (§ `std.arith`, new), `tests/README.md` (§ `stdlib/arith/`, new),
`07_stdlib_api.md` (§ `std.arith`, new), `examples/README.md`, `run_tests.sh` and `per-commit.yml`,
`AGENTS.md`, [`findings.md`](findings.md) (E-11, and the note on E-3), the questions Q-M1f.149 to .154, the phase README
and this record.

## Acceptance re-taken, 2026-11-11

- **123078 + 4567 is 127645, and 999999999 + 1 has an answer of ten digits:** `tests/stdlib/arith/01_add.ein`
  and `02_carry_out.ein`, each claim held by `ein test`.
- **The harness test, the search-free test and the test of the rule of the theory pass:** `arith_theory.rs`,
  `arith_search_free.rs`, and `arith_theory.rs` with `arith_batch.rs` under the counters.
- **No `absent`, no `not`, one stratum:** `the_theory_writes_no_negation`, and `arith_search_free.rs`'s read of
  `;; Strata — stratified, 1 stratum` on every program.

## For the stages that follow

- **A library whose entry is a rule with an empty `:match` writes that rule's name in a fact** (E-11), as
  `(arith-rule arith)` does; `std.num9` will need the same.
- A new operation adds its rules and relations to `arith.ein`, **each relation to the closing rule's
  `(__closed__ …)` list and each rule to its `(arith-rule …)` list**, and each table it reads to the import and
  the closing list; `arith_theory.rs` fails on any of the three forgotten.
- It adds its result relation to `RESULTS` in `arith_search_free.rs`, its request to `REQUESTS` in
  `arith_batch.rs`, and its cases to `arith_theory.rs`; its programs go under `tests/stdlib/arith/`.
- An operand goes through `operand` and `operand-is`, as `add-ask` to `add-start` do, so a request may name it.
- A call inside the theory starts a walk directly with a tag of its own and reads `(done TAG ANSWER)`.
- A program that imports `md` has a KB view of some 4 500 edges, past the DOT sweep's bound.
