# S1f.23.9 — the user's answers on the numerals' check, built

**Phase:** [P1f.23](README.md)
**Status:** **built 2026-11-18 on the branch `s1f.23.9-numerals`** (`90e88122`, `f4378b23`; the Record below), not merged; started 2026-11-18 on the user's word (*"process plans/m1f_hypothesis_and_documentation/review/02-numerals/ANSWERS.md"*),
in a worktree beside S1f.23.6 and S1f.23.8; merged after them, and one gate, `utils/ci_container.sh` and one nightly
hold the three. **A stage that changes a rule of the calculator runs the batch by hand** (`AGENTS.md`).
**The answers:** the user's file as written, `93f30a91`
(`git show 93f30a91:plans/m1f_hypothesis_and_documentation/review/02-numerals/ANSWERS.md`), and the removed items'
pages beside it in that commit (`…/02-numerals/NN-…/README.md`), with their runs' programs; the topic left the bundle
(`d21d05ec`).

## The answers

| item | the question | the user's answer |
|---|---|---|
| Q-M1f.405 | which numerals are *taken from the input program*, and *once* | *"(a) the difference is between numerals produced by operation from valid numerals, and all other numerals from any source"* |
| Q-M1f.409 | an integer and a rational: their check and their label | (a); and, asked how L-101's rule reads a rational whose part is a natural numeral, **"Verify the natural"**: a rational built from a natural numeral checks it as a numeral, as an operation checks its operands, so `(rat (n z x) (n z 2))` is dead again (the option (d) for that case) |
| Q-M1f.406, .407, .410, .411 | the label; an untaken numeral; no join on a number; two faults one flag | (a) each |
| Q-M1f.408 | where the check's rules live | **(b)**: *the walk in `std.math.positional`, the representation keeping the flags* |
| L-101 | `int-check-parts` / `rat-check-parts` ask non-numerals | *"integer built from invalid decimal is invalid, rational built from invalid integers is invalid, if you want to build rational from something else it has to provide verification, the same way as addition, multiplication, comparison etc."* |

## What it builds

1. **The rule of Q-M1f.405**: a numeral an operation produces from valid numerals is valid as made, and **every other
   numeral, from any source, is checked**: one the program writes in a request, one it hands straight to a tagged
   call (Q-M1f.405's case (e), dead again), one a rule of the program makes up. What an operation produces is not
   walked again. Within the calculator's laws: no rule joins two facts on a number (`AGENTS.md`); where the rule
   cannot be had without one, the stage says so and stops for the user.
2. **Q-M1f.408 (b)**: the numeral's walk moves into `std.math.positional`, its relations parameters as the module's
   are, each base connecting it by its macro's call; `std.math.decimal` and `std.math.binary` keep their flags
   (`malformed`, `not-decimal`, …). The closing rules' lists, the macro's body and parameters, every base's call.
3. **L-101 and the natural part**: validity composes: an integer is valid where its parts are valid decimals, a
   rational where its parts are valid integers; a rational whose part is a natural numeral checks the numeral, as an
   operation checks its operands; the check is asked only of the shapes it is for (a rule a shape), so no fact like
   `(check 7 7)` is left that no rule reads.
4. **The ledger**: each answer filed as ruled with the user's words (`open_questions.md`, Q-M1f.405 to .411; the
   milestone's `findings.md`, L-101); the decisions that state the check (Q-M1f.404 and those the build amends)
   brought to the rulings; the pages (`04-stdlib/` calculator pages, regenerated rule regions and drawings,
   `stdlib/README.md`).

## Acceptance

- Every corpus program's answer as before, but those the rulings move, each named with today's answer and the new
  one: Q-M1f.405's case (e) and Q-M1f.409's case (d) dead again, and any other the census finds; their `:expect`
  brought to the ruling.
- `cargo test -p ein-infer --features counters --test arith_batch`, by hand; `arith_theory.rs`, `int_theory.rs`,
  `rat_theory.rs` and the rest of the calculator's tests; `tests/stdlib/math/` (`ein test`); `stdlib_coverage.rs`.
- The gate, `utils/ci_container.sh` and the nightly, once, with the phase's other stages, on the merged tree.

## Record

Run 2026-11-18 in the worktree `build/wt-s1f239`, branch `s1f.23.9-numerals`, on `263ae0df`. Commits: `90e88122` (the
build: the standard library, its tests and programs, the pages, the decisions, the shape golden), `f4378b23` (two
programs' comments naming a path the link check reads) and this record (plans only). Not merged: the main session merges it after S1f.23.6 and S1f.23.8.

### The rulings, as built

**Q-M1f.405, with the user's distinction.** The calculator cannot tell a number an operation made from one a program
wrote by reading it: only its label tells them apart, and a rule that read the label beside the number would join two
facts on a number. So it is told by **where the number is written**: each tagged call has a walk of its own behind
it, and the call is the only way in from outside the calculator.

- Every tagged call a program or a module over the calculator writes asks the check of what it is handed and is
  handed on to a relation of the calculator's own, which checks nothing. Decimal and binary, the five each:
  `add-walk` → `add-run`, `sub-walk` → `sub-run`, `cmp-walk` → `cmp-run`, `mul-call` → `mul-run`, `div-call` →
  `div-run` (`bin-` twins), by `std.math.positional`'s fifteen new rules, three a call: `add-walk-runs` hands the
  call on, `add-walk-check-x` and `add-walk-check-y` ask `(?CHECK N N)` of a numeral in either place (a rule a place,
  `z` asked nothing). `std.math.int`'s four calls (`int-norm` → `int-norm-run`, `int-add-call` → `int-add-run`,
  `int-mul-call` → `int-mul-run`, `int-cmp-call` → `int-cmp-run`) by one rule each, `int-norm-runs` and the rest,
  which hands the call on and asks `(int-check (int P N))` of each pair of parts; `std.math.rat`'s three
  (`rat-norm-run`, `rat-mul-run`, `rat-cross-run`) the same with `(rat-check (rat (int P N) D))`; `std.math.gcd`'s
  `gcd-call` → `gcd-run` by `gcd-call-runs`, `gcd-call-check-x` and `gcd-call-check-y`.
- Every caller inside the calculator writes the walks: `std.math.positional`'s request starts (`add-start` →
  `?ADD-RUN`, and so on), `std.math.decimal-arith`'s and `std.math.binary-arith`'s products and divisions,
  `std.math.int`, `std.math.rat`, `std.math.gcd` and `std.math.convert`. The numbers they hand on are operands they
  checked or numbers a walk made: nothing is walked again.
- **One exception, the public division**: `div-start` writes the call, `(?DIV-CALL (?QUOTIENT X Y) U V)`, which is
  handed on, so that the call's division-by-zero flag, `(division-by-zero (div-call (quotient X Y) U V))`, names a
  fact the state holds, as Q-M1f.211 says; its two asks find their facts held (the look-up asked them).
- Rules before → after, `std.math.positional`: 78 rules and 63 parameters → 96 and 68 (the walk's three, the calls'
  fifteen); every walk rule's `?ADD-WALK` (and its four siblings) is `?ADD-RUN`; the macro's body and list, and both
  bases' calls, written again in the order a reader meets them. `std.math.decimal-arith`: 57 relations → 62; binary's
  57 → 62; `std.math.int` +4 relations, +4 rules (and L-101's); `std.math.rat` +3 and +3; `std.math.gcd` +1 and +3.
- **What moved**: Q-M1f.405's case (e), `(add-walk (mine a) (n z x) (n z 1) 0 z)`, alive before, is dead
  (`tests/stdlib/math/decimal-arith/52_tagged_call_malformed.ein`); likewise a malformed numeral handed to a
  binary call (`binary-arith/24`), to an integer's call as a part (`int/38`), to a rational's call as a denominator
  (`rat/33`) and to `gcd-call` (`gcd/13`). No corpus program hands a call a malformed numeral, and no corpus answer
  moved.

**Q-M1f.408 (b).** `numeral-step`, `numeral-step-zero` and `numeral-valid` are `std.math.positional`'s,
`(?CHECK ?N ?NZ)`, `(?CHECK ?N)` and `(?CHECK ?VALID)`, after `operand-check`; `std.math.decimal` and
`std.math.binary` lose their three walk rules each (`bin-numeral-step` and the rest) and keep `check`, `valid`
(`bin-`), the four flags and `not-…-is-false`, which read the walk. A program that loads a representation without
an operation loads no walk, and nothing asks it.

**L-101 and the natural part.** `int-check-parts` (`(check ?p ?p) (check ?n ?n)`, whatever the part) is two rules,
`int-check-p` and `int-check-n`, each matching a part `(n ?q ?a)`; `rat-check-parts` (`(int-check ?i) (check ?d
?d)`) is three, `rat-check-z` (an `(int ?p ?n)` numerator → `int-check`), `rat-check-z-natural` (an `(n ?q ?a)`
numerator → `check`: *"Verify the natural"*) and `rat-check-d` (an `(n ?q ?a)` denominator → `check`). No `(check
7 7)`, no `(check z z)` and no `(int-check (n …))` is derived. Q-M1f.409's case (d), `(rat-add (rat (n z x) (n z 2))
…)`, alive before, is dead (`rat/35_natural_numerator_malformed.ein`); a well-formed natural numerator is checked,
labelled valid and answered by silence as before (`rat/34`).

**Programs** (eleven, each with its corpus entry and its row in `tests/README.md`): `decimal-arith/51` and
`binary-arith/23` (the five calls from a program's rule, every answer under its tag, and the asks exactly the two
numerals handed: what the calculator makes and hands itself is not asked); `int/37`, `rat/32` (the calls of each);
`decimal-arith/52`, `binary-arith/24`, `int/38`, `rat/33`, `gcd/13` (a malformed numeral handed to a call, `(false)`);
`rat/34`, `rat/35` (the natural numerator; `34` answers a sum beside its silent request, so that it has an answer
for `arith_search_free.rs` to read). `stdlib_coverage` holds every new rule activated.

**Tests brought along**: `arith_theory.rs` (`one_call_of_the_macro_connects_each_base`'s 78 → 96 rules;
`the_tagged_calls_keep_their_shapes` holds the walks, `RUNS`, each declared with its call's types and started at
its call's values; the division's circuit pins); `int_theory.rs`, `rat_theory.rs`, `binary_theory.rs` (the
circuits' pins); `arith_search_free.rs` (`RESULTS` +8, the calls' hand-overs a program reads: `done`, `order-is`,
`bin-done`, `bin-order-is`, `bin-div-done`, `int-done`, `rat-done`, `rat-cross-is`); `alive_invariant.rs`
(`SEEDED`: the eleven programs' names, and `std.math.positional`'s `0` now met first in `numeral-step-zero`);
`corpus_exits.txt` (the eleven programs' 55 cells, re-banked).

### Checks

| check | result |
|---|---|
| `ein test tests/stdlib/math` (the gate binary of this tree) | 195 files, 371 expectations, all held |
| `ein test examples tests stdlib`, every verdict line against the same binary on `263ae0df`'s `stdlib/` (`EIN_STDLIB`) | the eleven new programs alone differ (each failing before, as banked); every other file's verdict the same |
| `cargo test -p ein-infer --features counters --test arith_batch`, before (`EIN_STDLIB` at `263ae0df`'s) and after | 8 tests pass both; the counts below |
| the calculator's theory tests (`arith_theory`, `int_theory`, `rat_theory`, `gcd_theory`, `binary_theory`, `num9_theory`), `stdlib_coverage` | all pass after the circuits' pins were taken again (below) |
| `utils/stdlib_census.py -k tests/ --check` | 536 rules over 21 modules, every one activated by `tests/` |
| `utils/stdlib_mutants.py -k numeral-step`, `-k numeral-valid` | 5 of 5 killed (the walk's mutants, now `std.math.positional`'s); the new rules make no mutant: a call's ask swapped is itself |
| `cargo test --profile gate --workspace --no-fail-fast`, on `90e88122` | 161 test binaries passed, 1 338 tests; `int_theory` was killed by the kernel's OOM killer (the machine's tmpfs held 15 GiB of other sessions' files, 7.6 GiB left): run alone at `EIN_TEST_JOBS=4`, 12 passed, peak 6.86 GB, against 6.73 GB for the same binary on `263ae0df`'s `stdlib/`. A first run, before the fixes, failed four binaries (`arith_search_free`, `corpus_cli`, `alive_invariant`, `ein-ir`'s manifest test after a comment edit), each fixed and run again green. Nothing left in the run's `TMPDIR` |
| shape golden, `EIN_BLESS=1 EIN_CORPUS_SLOW=1` | below |
| `doc_audit.py --links --check --tracked` | findings only under `review/` (ignored outputs); every decision id resolves |
| `check_doc_blocks.py`, `render_diagrams.py --check`, `check_generated.py` (9 generators, `EIN_BIN` the gate binary), `check_release_voice.py`, `wrong_answers.py` (16 open rows, each still answered wrong), `check_gate_steps.py`, `check_tool_mirrors.py`, `cargo fmt --check` | green |

**The batch, before → after**, per request at every batch size (the proportion holds in both): decimal `add`,
`sub`, `cmp`, `mul` the same firings and candidates; `div` +3 firings (462.66 → 465.66 at 4 000: the hand-on and
the two asks found held), candidates the same; every decimal and binary operation +8 guard values a request (757.93
→ 765.93 for `add`), `operand-undeclared` comparing each operand with the program's names, two names more;
`bin-add`, `bin-sub`, `bin-cmp`, `bin-mul` the same firings, `bin-div` +3; `int-*` +0.7 firings (84.26 → 84.92 for
`int-add`), candidates a little fewer (L-101: no `(check z z)`); `rat-*` +0.1 to +1.1 firings; `add9` to `div9`,
`gcd`, `lcm` the same.

**The circuits' pins, taken again**: decimal 127645 ÷ 25 (232, 220) → (235, 221), and the division's four others +3
and +1 deep, 127645 ÷ 0 (20, 10) → (22, 11); binary 127645 ÷ 25 (271, 245) → (274, 246), ÷ 0 (31, 21) → (33, 21);
`int_theory`'s fourteen ±1 or +2 firings, no depth moved; `rat_theory`'s eight +1, none deeper; `gcd_theory`'s and
the decimal and binary other operations' unchanged. The pages that state them follow.

**A program's own theory costs more**: each numeral it hands a call is checked, a number a walk made among them
(Q-M1f.410 (a)): root's firings, `ein saturate`, before → after: `fibonacci` 702 → 1 167, `pythagorean` 2 099 →
2 784, `collatz` 4 762 → 5 470, `laws` 14 303 → 17 587, `casting_out_nines` 8 364 → 9 131, `perfect` 7 048 → 7 870,
`primes` 6 210 → 6 810. `send_more_money`, the slow ones, write no call: the same.

**The shape golden**, re-banked whole (`EIN_BLESS=1 EIN_CORPUS_SLOW=1`): 29 027 lines → 29 467, the eleven new
programs' 440 added; 4 068 lines moved, of 254 programs, all the calculator's and its users': `ir[resolve]` and
`ir[expand]` 243 each, `load` 240, `saturate`, `plan`, `match`, `naf`, `explain`, `explain-noalts`, `region-trace`,
`oracle` 239 each (the oracle's page counts the plans it read), `dot[kb…]` 237 to 239 each, `states` 143, and the
nine programs whose own file writes a call or a walk 9 each in `ir[parse]`, `ir[dump-compact]`, `dot[ir…]` and
`dot[rules…]`, 5 in `dot[constraints…]`. **No `read-out`, `cores` or `region-lattice` line moved**: every answer is
as it was.

### Readings, each with its default

- **What is checked is told by the relation it is written in.** A call and the walk behind it are two relations;
  the calculator writes the walks, anything outside it the calls. *Default: as built.* The other way within the
  laws, one relation and every start asking the check (Q-M1f.405's (e)), walks again every number the calculator
  hands itself, which the ruling excludes.
- **The interface keeps its names**: the calls stay `add-walk` … `gcd-call` (Q-M1f.189, Q-M1f.228, a program's text
  unchanged) and the walks are the new names. *Default: as built.* The other way renames every call a program
  writes.
- **The public division is a call** (above), the other requests' starts writing the walk. *Default: as built*,
  Q-M1f.211's *why* kept true; the cost is the division's +3 firings. The other way: `div-start` writes `div-run`,
  and the call's flag names a `(div-call …)` no state holds.
- **The division-by-zero flag names the call**, `(division-by-zero (div-call …))`, whatever made the division; the
  calculator's own divide by nothing that is zero. *Default: as built*, every flag's text as before.
- **An integer's or a rational's call is checked as an operand of its kind**, `(int-check (int P N))` and
  `(rat-check (rat (int P N) D))`, so a digit or a name a call is handed as a part is the shape rules' flag
  (`not-integer`, `not-rational`), where it was silent. *Default: as built.* The other way, a rule a part asking
  `std.math.decimal`'s check only, is fourteen rules for `std.math.int` and fifteen for `std.math.rat` and leaves a
  digit silent.
- **The calculator's own constants are not checked**: `(n z 1)` in `std.math.int`'s adjustments, `(n z 2)` in
  `std.math.convert`'s halving, written by its own rules into its own walks. *Default: not checked.*
- **The decisions**: Q-M1f.404's file states the rulings on Q-M1f.405 to .411 and L-101, and Q-M1f.163, .210,
  .228, .276 and .277 are brought to them; no decision file of their own, the code citing Q-M1f.404. *Default: as
  built.* The other way: a file each for Q-M1f.405, .408, .409.
- **The walk's words are the base's**: `numeral-step`'s `:why` says *the digit {?a} is one* in binary too, as
  `std.math.positional`'s other rules do. *Default: as built.*
- **The tagged-call test holds the walks** (`arith_theory.rs`' `RUNS`): each walk declared with its call's types, and
  a caller that starts one writes the call's start values; the walks started somewhere are now `add-run`,
  `sub-run`, `cmp-run`, `bin-add-run`, `bin-sub-run`, and `add-walk` and `cmp-walk` from `examples/beyond_arith/`.
  *Default: as built.*
- **`AGENTS.md` and the phase's README are not edited here**: the calculator's lines of `AGENTS.md` stay true
  (*a numeral … flagged where an operation takes it*), and the phase's row is the main session's on the merge.
