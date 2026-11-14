# S1f.6.38 — A number is well-formed from the start: the check is its representation's

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-14, on the user's ruling of that day. **T1 is a design shown to the user
before anything is built** (the user's standing rule: real programs with computed answers first). **T1 done
2026-11-14, stopped for the user:** [§ T1 record](#t1-record-2026-11-14-the-design-on-real-programs). No rule-level
way is *right away*; the design is an engine stage first (the loader states every nested fact a program writes),
decimal's check in `std.math.decimal-arith`'s own rules over it, int and rat with the (b) part. Nothing is built;
its open choices and findings are under [§ To file](#to-file), for the coordinator. **Ruled 2026-11-14** ([§ The build, as ruled](#the-build-as-ruled-2026-11-14)): E1, a separate `std.math.decimal`, the other defaults.
**Depends on:** [S1f.6.12](s1f.6.12_malformed_numbers.md)'s build, whose checks it moves;
[S1f.6.31](s1f.6.31_the_stdlib_layout.md)'s layout. **Runs before [S1f.6.16](s1f.6.16_binary_arithmetic.md)'s
build**, whose design ([§ The design](s1f.6.16_binary_arithmetic.md)) put the parameterised checks in
`std.math.positional`, the shared arithmetic, which this ruling says they are not: the build follows T1's design
once the user has seen it.

## The ruling

[Q-M1f.210](../open_questions.md#q-m1f210--stdint-and-stdrat-and-the-flags) asked what `std.math.int` and
`std.math.rat` do with a malformed request (E-38): (a) silent, as today; (b) three rules a module from its tags to
`(bad-request REQUEST FAULT)`, flagging an undeclared name and a digit as an operand; (c) (b), and each part of an
integer and a rational handed to the decimal check. The user, 2026-11-14:

> Now Q-M1f.210: yes (c), but have to notice that malformed decimal numbers is a problem of decimal module,
> malformed binary numbers - of binary module, not arithmetic, integer or rationals or whatever else. If there is
> a malformed number, I expect it has to be detected right away, without need to start calculations with it.
> Thing about it like written numbers on paper, if decimal number contains letters, signs, whitespaces and
> hieroglyphs - it is not a decimal number from the beginning. It is a string, cypher, whatever else but not a
> decimal number.

Read as three requirements:

1. **Whose check.** Whether `(n (n z 1) x)` is a decimal number is the decimal representation's question, and a
   binary numeral's is binary's. Not the shared arithmetic's (`std.math.positional`), not `std.math.int`'s or
   `std.math.rat`'s, nor any module that only carries numbers inside its own terms.
2. **When.** As soon as the number is written, by what it is, **without a calculation started with it**: not a
   pass at a request's operand (S1f.6.12's placement, `check` handed from `operand-number`), not a walk that finds
   it on the way.
3. **Where it is written does not matter.** A numeral written as a request's operand, inside an integer, inside a
   rational, as a `let`'s value, is the same numeral and is checked the same way, once.

What (c) keeps of its own: the faults that are **not** of a numeral, which S1f.6.12's table mixes in, belong to
the module that reads the operand, and int and rat flag them as decimal does (the (b) part): a name no `let`
states, a digit where a number goes, a `let` that cannot be read.

## Today

S1f.6.12's table ([§ What the theory flags now](s1f.6.12_malformed_numbers.md#what-the-theory-flags-now)):
`operand-number` and `operand-name` hand each number they find to `(check TAG NUMBER)`, `check-step` walks it a
place a firing, a fault is `(bad-operand TAG FAULT)`, ten rules by operation lift it to `(bad-request REQUEST
FAULT)`, and the flags `(malformed REQUEST)` / `(undeclared NAME REQUEST)` derive `(false)`. So the check runs
only where a request asks for an operand; int and rat reach `operand` for a named operand and nothing more (E-38);
a malformed part of an integer or a rational is never seen.

## Tasks

1. **T1f.6.38.1 — the design, on real programs, shown before anything is built; the stage stops after it.** On a
   copy of the stdlib under `build/s1f.6.38/` and a release binary of its own:
   - **Sort every fault** of S1f.6.12's table into *a numeral's* (a digit's place holding no digit, a number
     ending in a digit or a name where `z` goes, …) and *an operand's or a program's* (a name no `let` states, a
     digit where a number goes, a `let` stated twice or of `z` or a digit), and say whose module each is under
     the ruling.
   - **How a representation module sees every numeral a program writes, right away,** whatever term it sits in.
     What Ein has, each tried with a real program and its computed answer, its cost against S1f.6.12's pass,
     and what a trace, a core and `refuted by` read: rules whose premises have a variable head over what the
     program writes (a walk of written terms, not of requests); a check over root's written facts only, numbers
     a rule derives being well-formed by construction; and, if neither can, the engine change it would take (a
     declared constructor's argument positions checked at load, say), described and not built. Say which is
     *right away* in the user's sense and which only looks like it.
   - **The layout it gives,** module by module, with S1f.6.16's design beside it: where the decimal numeral's
     check lives (`std.math.digits`, which is facts today, a new `std.math.decimal`, or `decimal-arith` as
     decimal's combining module), where binary's, what leaves `std.math.positional`, what `std.math.int` and
     `std.math.rat` keep (the (b) part only), and what `std.math.num9` does (silent by reply 9).
   - **What a program sees:** the flag a malformed numeral derives (is it still `(malformed REQUEST)`, or a flag
     on the numeral itself, and `(false)` either way), on the probes of S1f.6.12 and E-38's 26
     (`build/s1f.6.12/build/t9/`), written as integers and rationals too; and that a well-formed program's
     answers and firings are what they were, or by how much they move.
   Recorded as the record's first section, every open choice with a proposed default (filed as Q-M1f rows,
   defaulted, the user's to revise); committed, records only; **then the stage stops for the user.**
2. **T1f.6.38.2 onward — the build, after the user has seen T1:** written then, with S1f.6.16's build, which it
   shapes. Its acceptance will hold at least: a malformed numeral is flagged and `(false)` wherever it is written,
   with no request reading it; int and rat flag an undeclared name and a digit as an operand (E-38 closed); no
   module but the representation's names a numeral's fault; no well-formed program's answer moves.

## What it must not do

- **Not leave a numeral's check in the arithmetic** (`positional`, the operations), in int or in rat.
- **Not make a malformed program a load error** without the user's word: today it loads, is dead and claims
  `(solve :expect (false))` (AGENTS.md § Edits that go together); a load refusal is a proposal in T1, not a
  default.
- **Not build before the user has seen T1.**

## The standing rules

A wrong answer met on the way is a register row before its fix. Findings go to [`findings.md`](findings.md),
questions to [`open_questions.md`](../open_questions.md), each under the next free id. The gate runs once over the
batch (the user's cadence of 2026-11-13).

## T1 record, 2026-11-14: the design, on real programs

Taken on a copy of HEAD's stdlib (`727627e7`) and a release binary of the stage's own, built from a detached
worktree at that commit (`build/s1f.6.38/ein`, sha256 `118dc168b60837bd…`), each variant written from the copy by a
script, so each is reproducible. Everything is under `build/s1f.6.38/` (gitignored): `gen_probes.py` (the probes),
`mk_variants.py` (the stdlib variants), `emulate_written.py` (the engine change E1, emulated), `run_probes.py`,
`order.py`, `cost.py`, `sweep.py` and `cmp_sweep.py`, their output under `out/`. Nothing under `stdlib/`, `tests/`,
`examples/`, `corpus/` or `ein.rs/` moved.

**In one paragraph.** Today's check runs where a request asks for an operand, so a numeral inside an integer, a
rational, a `let` nobody asks about or a program's own fact is never read (E-38). A representation module can see
every numeral a program writes only by a walk of every fact the program holds (rules whose head is a parameter,
which Ein has), and that walk is not *right away*: it costs two to three times the firings and three to seven times
the time of the computation it sits beside, buries the flag in a trace of 2 081 steps, and, with the closing rule naming its
rules as every closing rule must, makes a program that also imports `std.algebra` a load error (E-69 below). Restricted
to what a program states, it needs an `absent`, which Ein decides after the whole positive closure, so the flag comes
after every calculation (E-70). **What is right away is an engine change, E1: the loader states each nested fact a
program writes, `(written T)`, as it states each declaration's mirror today.** Decimal's five rules over it flag a
malformed numeral as the first or second firing of root, wherever it is written, with no request reading it, at
fewer firings than today; emulated, it moves no well-formed program's answer in the corpus. What it leaves silent is
named below, with the one stricter option, E2, a load refusal, which is the user's to take.

### 1. The faults, sorted

Every row of S1f.6.12's table ([§ What the theory flags now](s1f.6.12_malformed_numbers.md#what-the-theory-flags-now)),
and the faults it lists as silent, by whose they are under the ruling. *A numeral's* is a fault of the term headed
by the representation's constructor, decimal's `n` (full name `std.math.decimal-arith.n`, so a term claims to be a
decimal number exactly when its head says so; an `n` a file writes without listing it is the file's own, S1f.6.35,
and is no decimal number at all). *An operand's or a program's* is a fault of what stands where the module that
reads it wants a number, or of a `let`.

| what a program wrote | today's rule (HEAD) | whose fault | whose module, under the ruling |
|---|---|---|---|
| a digit's place holding a name that is no digit, `(n (n z 1) x)`, a sign, `(n z -)`, or a fact, `(n z (n z 1))` | `check-digit` | **the numeral's** | decimal's (`b2`'s: binary's) |
| a number ending in a digit where `z` goes, `(n 1 2)`, `(n 0 2)` | `check-prefix-digit`, `check-prefix-zero` | **the numeral's** | decimal's |
| a number ending in a named constant, `(n price 1)` | `check-prefix-named` | **the numeral's** (a name where `z` goes) | decimal's; the rule reads `named`, the `let`'s, so it stays in the module that declares both (§ 3) |
| a number ending in another name, `(n end 1)` | silent (saying *a name* costs the program's names, E-36) | the numeral's | decimal's; silent unless E-36's remedy, a kernel test *is a name* |
| a number ending in a fact of another relation, `(n (pair 1 2) 3)` | silent (no rule can say *not of this shape*) | the numeral's | decimal's; silent |
| a digit where a number goes, `(add 7 X)` | `operand-digit`, `operand-digit-zero` | an operand's | the module that reads the operand: decimal-arith (S1f.6.16's `positional`), int, rat |
| a name no `let` states, `(add foo X)` | `operand-undeclared`, the one `absent` | an operand's | the same |
| a name standing for a name, `(let a b)` with `(let b …)` | `check-prefix-named`, through the check of the `let`'s value | **a `let`'s** (it was found by the numeral's check only because the check read the value) | the module that declares `let`: one rule, `let-to-name` (§ 4) |
| a name stated twice with two numbers; `z` or a digit as a `let`'s name | `let-twice`, `let-zero`, `let-digit`, `let-digit-zero` | a program's (the `let`'s) | the module that declares `let` |
| a fact of another relation where a number goes, `(add (pair 1 2) X)`, `(add (b 1 z) X)` | silent | an operand's | silent, as today |
| a natural where an integer goes, an integer where a rational goes | silent | an operand's | int's, rat's: [§ 6, Q5 and Q6](#6-the-open-choices-each-with-a-proposed-default) |
| a digit or a named constant inside an integer, `(int 7 z)`, `(int price z)` | silent | **the integer's** (int's own term) | int's (rat's for `rat`): Q5 |
| a wrong count, `(n 1)`, `(n z 1 2)` (*whitespace* inside a numeral) | refused at load (S1f.6.23, [`defined_behaviour.md` § 4.26](../../../docs/kernel/defined_behaviour.md#426-a-form-of-a-declared-relation-has-the-declared-count-of-arguments--m1f-s1f623)) | the loader's | already right away |
| a character that is not ASCII (*hieroglyphs*), a string where a value goes | refused at the parse (M1f S1f.5.22, S1f.5.2) | the parser's | already right away |
| every malformation of `std.math.num9` | silent | num9's | silent, reply 9 (b) |

So of the user's list, *whitespace* and *hieroglyphs* are refused before anything runs already; *letters* and
*signs* are names (`[A-Za-z0-9_.\-$%^*+=<>!&@~/'…]+`, `ein.l`) and are this stage's.

### 2. How a representation module can see every numeral a program writes

**What Ein has** ([`01_grammar.md` § Rules](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#rules)): a premise's
head may be a variable only as a parameter its activator binds; the loader stores each declaration as a fact,
`(relation R T1 … Tk)` ([§ Relation declarator](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#relation-declarator)),
so activators can be derived from the declarations; **no premise tells a written fact from a derived one**; and an
`(absent …)` premise is decided at the closure/world boundary, after the positive closure stalls
([`absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md#evaluation-points)).

Five placements, each run on the same probes (§ 4) and batches (§ 5):

- **H, today's pass** (S1f.6.12): `operand-number` and `operand-name` hand the operand they find to `(check TAG
  NUMBER)`, walked a place a firing.
- **W1, a walk of every fact**, rule-level, no engine change (`stdlib-w1`): `walk-K (?R) :match (?R ?a1 … ?aK)
  :assert (and (term ?a1) …)` and `into-K (?F) :match (term (?F ?a1 … ?aK))`, K from 1 to 10, each activated by a
  rule over the mirror `(relation ?R ?t1 … ?tK)`; decimal's check over `(term (n P A))`. 30 rules for the walk.
- **W2, the walk restricted to what a program states** (`stdlib-w2`): W1, its activators reading `(absent (walk-skip
  ?R))` over a written list of 149 relations: the 141 of the calculator's three modules that a program does not
  state, the seven tables and `term`. The nearest a rule
  comes to *a check over root's written facts only*: a rule cannot read where a fact came from, so the restriction
  is by relation.
- **E1, the loader states every nested fact a program writes** (`stdlib-e1`; the engine change, **emulated**:
  `emulate_written.py` appends `(written-term T)` for each nested fact of each fact a file writes, the facts the
  loader would add; the core reads that fact instead of the written one, below). Decimal's check over `(written-term
  (n P A))`: five rules and no walk, every place of a numeral being a nested fact of its own.
- **E2, a constructor's argument domain checked at load**, described and not built (§ 2.3).

Every variant but H also carries the (b) part (three rules each in int and rat, § 3), and in every one the numeral's
check is the same five rules ending in `(not-decimal NUMERAL THING)` and `(false)`; only how they reach the numeral
differs.

| | H, today | W1 | W2 | E1 (emulated) |
|---|---|---|---|---|
| sees a numeral as an operand, inside an integer, a rational's numerator and denominator, a `let` nobody asks about, a program's own fact | the operand only | all six | all six | all six |
| sees one a rule writes in its conclusion (`rule/r1`, `r2`, `r3`) | r1 and r2, at the request that reads them | all three | all three | **none** (facts only; § 2.2, item 4) |
| when the flag comes: of `ein saturate --events`' firings, the first that flags, on `(mul (n z x) (n z 2))` | #11, after 5 of the calculator's | #249, after 3 of the calculator's; the 245 others before it the walk's and the closing rules' | #316, **after all** the calculator could do (10) | **#2**, after 1 (the request's first step, written first) |
| the same on `(rat-add (rat (int (n (n z 1) x) z) (n z 2)) …)` | never | #482 | #496, after 19 of 19 | **#1** |
| firings, one sum of 123078 + 4567 | 32 | 2 143 | 338 | **22** |
| firings a request, 1 000 sums / products (§ 5) | 45.0 / 504.0 | 115.4 / 1 506.1 | 41.0 / 499.9 | **27.0 / 486.0** |
| saturation, 1 000 sums / products | 63.1 / 812.5 ms | 413.4 / 5 781.6 ms | 65.0 / 831.4 ms | **39.5 / 808.7 ms** |
| `--trace` of `(add (n (n z 1) x) (n z 5))`, root | 13 steps, the flag at 12 | 2 081 steps (16 674 lines) | — | **8 steps, the flag at 2** |
| `unsat core` · `refuted by` | the request · `malformed-is-false` | the request · `not-decimal-is-false` | the same | the written fact holding the numeral (emulated: the stated fact) · `not-decimal-is-false` |
| the corpus (§ 5) | — | `equation_two_unknowns.ein` refused at load (E-69); firings +369.6 % | refused at load (E-69); +41.5 % | R unmoved on all 114; firings −2.2 % |

**Which is right away, and which only looks like it.**

- **H is not.** It needs a request to ask for the operand: the integer's parts, the rational's, an unused `let` and
  a program's own fact are never read (E-38), and where it does read one the calculation started first.
- **W1 only looks like it.** No request is needed, but it is a walk of *every* fact the program holds, the
  calculator's own steps and the 1 664 table rows included: some 2 000 firings on a program of one request, a
  flag interleaved with the computation it was meant to precede, and a trace nobody can read. It is the only
  rule-level way that sees a numeral a rule builds, and it pays for that by walking everything.
- **W2 looks like it and is the opposite.** The restriction to what a program states is an `absent`, and Ein
  decides an `absent` once the positive closure has stalled: the walk, and so every flag, comes after every
  calculation the program can run (19 of the rational's 19 firings before its flag). And it is refused at load beside
  `std.algebra`, as W1 is.
- **E1 is right away in the user's sense:** the numeral is checked by what it is, from the program as written,
  whatever it sits in, once; no request, walk or calculation is needed for it, and its flag is the first or second
  firing of root. It is not *before* saturation: the check is five rules, which fire in root's first layer.
- **E2 is right away in the strictest sense,** before anything runs, and a refusal.

#### 2.1 Why W cannot be kept

Beyond the cost: S1f.5.18's check refuses an activator a rule could derive binding a relation parameter to a fact,
and reads every name written as an argument as a candidate. The closing rule names every rule of the module,
`(arith-rule walk-1)` (held by `arith_theory.rs`), so `std.algebra`'s `imply1`, whose conclusion is `(?R2 ?a)`, can
be read as activating `walk-1` with a fact: `examples/beyond_arith/equation_two_unknowns.ein` is a `kb load error`
of twenty such lines under W1 and W2. With the walk's names left out of the closing rule it loads and answers its 3
states, at 91 768 facts beyond root where HEAD has 75 423 (`stdlib-w1n`). So a rule-level walk in the stdlib is an
exception to the closing rule's test, or a load error for every program that imports `std.algebra` beside it (E-69).

#### 2.2 E1, the engine change, described and not built

1. **What the loader states.** For every fact a file writes (a program's, a local module's, a stdlib module's),
   each fact nested among its arguments at any depth, once a value, as `(written T)`; for a stored negation `(not
   P)`, P's arguments. Where: beside the declarations' mirror facts in `ein_ir::from_ir` (`kb.add_fact(…,
   relation, …)`), from the walk S1f.6.23's count check already makes of every nested form. Measured by the
   emulation: 12 669 facts for 1 000 nine-digit sums, which cost the store 42 ms (`kb load` 73 ms where 31; the
   emulation's parse, 415 ms where 75, is its own text, which a loader stating them from its walk does not read).
2. **What it reads as.** A kernel relation like `relation`: closed, never a hypothesis target, decided at root, so
   no commitment can extend it and nothing that reads it can be stale. The proposed name is `written`; the
   emulation used `written-term`.
3. **The core.** A stated fact is derived from the fact it was read in, a step of the loader, so the core names
   the fact as written, `(add (n (n (n z 1) x) 7) (n z 5))`, and not the stated one (the emulation, writing them as
   facts, names `(written-term (n (n z 1) x))`).
4. **Rule conclusions.** A numeral a rule writes is no written fact. Stated from facts alone, E1 loses two
   detections H has at the request (`rule/r1-ground`, `(rule ask () … :assert (add (n z x) (n z 5)))`, and
   `rule/r2-pattern`, `(add (n ?p x) (n z 5))`): both dead under H, both alive under E1. Stating also the ground
   nested facts of every rule's and hrule's conclusion catches r1; r2, a numeral a rule assembles from a variable
   and a constant that is no digit, stays silent short of E2. Q4.
5. **What it changes elsewhere,** for the build: every program that writes a nested fact holds that many root
   facts more (the KB views and `corpus_shapes.md5` move for each); a `--dump-answer` state file re-loaded is stated
   again (the round trip's to hold); `.einb` carries them; `predicates::domain` gains no name (a nested fact is no
   name).

#### 2.3 E2, the alternative, described and not built

A module declares what a constructor's arguments may be, `n`'s second a digit and its first `z` or an `n`, and the
loader refuses, as S1f.6.23 refuses a wrong count, a written term outside it: in a fact at any depth and in a rule's
or a query's text where the argument is a constant, so `(n ?p x)` in a conclusion (r2) is refused too. It is the
paper analogy taken literally (*not a decimal number from the beginning*), and costs nothing at run time. Against it:
**a malformed program becomes a load error**, which the plan says is not a default without the user's word, and
today's claim `(solve :expect (false))` of every malformed test becomes a refusal; it is a type check in the kernel,
which has none ([§ What the signature means](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#what-the-signature-means--userspace-types-kernel-structure));
and *what a decimal digit is* moves from the module's rules into a declaration the engine reads.

### 3. The layout, beside S1f.6.16's design

The target once both builds are done; [S1f.6.16's § The modules](s1f.6.16_binary_arithmetic.md#2-the-modules-the-bases-own-module-is-the-file-that-combines)
is the left column. This stage's build runs first, on today's `std.math.decimal-arith`, so `positional` never holds a
numeral's check.

| module | S1f.6.16's design | this design (the defaults) |
|---|---|---|
| engine | — | E1: the loader states `(written T)` (Q1) |
| `std.math.positional` | 78 shared rules, the check among them: `check-step`, `check-prefix-digit`, `-zero`, `-named`; `operand-number` and `operand-name` hand the operand to the parameter `?CHECK`; the macro of 63 parameters | the four leave; `operand-number` and `operand-name` conclude `operand-is` alone; the macro loses `check`. It keeps the operand's and the `let`'s faults: `operand-digit`, `operand-digit-zero`, the four `let-*` rules and **`let-to-name`**, the ten routes to `bad-request`, the two flags and their `(false)` |
| `std.math.digits` | the tables, facts | unchanged |
| `std.math.decimal-arith` | decimal's own 37 rules: `check-digit` and `operand-undeclared` among them; the connecting call | **decimal's numeral check, its own rules**, five over `(written (n P A))`: `numeral-digit` (ten `neq`), `numeral-prefix-digit` (`nz`), `numeral-prefix-zero`, `numeral-prefix-named` (`named`, beside `let`), `not-decimal-is-false`; the relation `not-decimal`. `check-digit` leaves (its job is `numeral-digit`'s); `operand-undeclared` stays (an operand's). Where it lives: Q2 |
| `std.math.bits`, `std.math.convert` | as designed | unchanged |
| `std.math.binary-arith` | its 16 own rules, `bin-check-digit` among them | binary's numeral check, four over `(written (b2 P A))`: two `neq`, the prefix a bit (`bin-nz`), the prefix `0`, `not-binary-is-false`; `bin-check-digit` leaves |
| `std.math.int` | unchanged | **the (b) part**: `int-bad-only`, `-first`, `-second`, from its three tags to `(bad-request REQUEST FAULT)`, so `std.math.decimal-arith`'s flags and `(false)` follow; Q5's own shape and Q6 beside it |
| `std.math.rat` | unchanged | the (b) part: `rat-bad-only`, `-first`, `-second`, from `(rat-of …)`'s three tags; Q5, Q6 |
| `std.math.num9` | unchanged | unchanged and silent (reply 9) |

No module but the representation's names a numeral's fault: `not-decimal` is decimal's, `not-binary` binary's;
`positional`, int and rat name only an operand's (`malformed REQUEST`, `undeclared NAME REQUEST`) and a `let`'s.
Two details for the build: `numeral-prefix-named` wants `(neq ?m z)`, or a program that writes `(let z …)` (already
flagged) also flags every numeral, `(not-decimal (n z 1) z)` (`nat/o11-let-zero`, `int/i6b-let-zero`); and a
module's rule that writes another module's name without listing it reads its own undeclared relation and matches
nothing, in silence (E-62's shape, met twice building the prototypes: `bad-operand` in rat, `nz` in int), which
`stdlib_coverage.rs` will catch as a rule no test activates.

### 4. What a program sees

`out/e1x.txt`, the proposed design's prototype (`stdlib-e1x`: E1 emulated, the (b) part, Q5's own shape for int, Q6
for int, `let-to-name`), against HEAD (`out/head.txt`); `python3 run_probes.py` per variant, 2026-11-14. **Every
numeral fault the check knows is flagged in all six places it can be written**, where HEAD flags it only as an
operand:

| fault | as an operand | an integer's part | a rational's numerator | its denominator | a `let` nobody asks about | a program's own fact |
|---|---|---|---|---|---|---|
| `(n (n z 1) x)`, a letter | H: `(malformed REQUEST)` | H: silent | H: silent | H: silent | H: silent | H: silent |
| the same, proposed | `(not-decimal (n (n z 1) x) x)` | the same | the same | the same | the same | the same |
| `(n (n z 1) -)`, a sign · `(n z (n z 1))`, a fact · `(n 1 2)` · `(n 0 2)` · `(n price 1)` | as above, each its own `(not-decimal NUMERAL THING)` | | | | | |
| `(n end 1)` · `(n (pair 1 2) 3)` | silent under both | | | | | |

Each flag names the smallest `n` term at fault and what stands in it, `(n (n (n z 1) x) 7)` flagging `(n (n z 1) x)`;
`(false)` follows by `not-decimal-is-false`; `ein solve` reads `states (k_s) 0`, *every node is dead — root's own
fixpoint derives (false)*, `refuted by std.math.decimal-arith.not-decimal-is-false`. E-38's 26, as integers and
rationals:

| probe | HEAD | proposed |
|---|---|---|
| `i1`, `i1b`, `i1c`, `i2`, `i2b`, `i5`, `i9`; `r1`, `r1b`, `r2`, `r5`: a malformed numeral in a part, written or behind a name | alive, silent | dead, `(not-decimal …)` |
| `i3`, `i3b`, `r3`, `r3b`: an undeclared name | alive; `bad-operand` in root's state | dead, `(undeclared foo (int-add foo …))` (the (b) part) |
| `i4`, `r4`: a digit as the operand | alive; `bad-operand` | dead, `(malformed (int-add 7 …))` (the (b) part) |
| `i4b`, `i8`: `(int 7 z)`, `(int price z)` | alive | dead, `(not-integer (int 7 z) 7)` (Q5, default) |
| `i7`: a natural for an integer | alive | dead, `(malformed (int-add (n z 5) …))` under Q6 (a); alive under its default |
| `r7`, `r7b`: an integer, a natural, for a rational | alive | alive (rat's Q6 not prototyped; the shape is int's) |
| `i6`, `i6b`, `i6c`, `r6`: a `let` that cannot be read | dead, `(malformed (let …))` | the same |
| `m4c`, `(let a b)` beside `(let b (n z 1))` | dead, through the request's check | dead, `(malformed (let a b))` by `let-to-name`; without it, alive (`tests/stdlib/math/decimal-arith/32` moves to alive under plain E1) |
| `rule/r1`, `r2`: a malformed numeral in a rule's conclusion | dead, at the request | alive: the emulation states facts only; Q4's default (a) would catch r1, and r2 stays silent |

**The well-formed probes, firings:** one sum 22 where 32, a quotient 221 where 229, a named operand 15 where 18, a
program's own fact beside a sum 12 where 14, an integer sum 35 and a rational sum 495 as before.

### 5. Well-formed programs: answers and cost

**The corpus.** All 114 programs that import `std.math.decimal-arith`, `int` or `rat` (from the HEAD worktree;
`python3 sweep.py head b e1 w2 w1 e1x`, `python3 cmp_sweep.py …`, 2026-11-14): `ein solve`'s read-out without its
`work` row, and `ein saturate`'s firings.

- **The (b) part alone moves nothing**: every read-out and every firing count equal.
- **E1, and the proposed prototype e1x, move no well-formed program's R.** The read-outs that move are the seven
  malformed tests' (`tests/stdlib/math/decimal-arith/22` to `26`, `31`, `32`): dead before and after, their core
  and `refuted by` naming the numeral's flag; under plain E1 `32` turns alive (the `let`'s case), and `let-to-name`
  restores it. Firings −2.2 % in all (108 193 where 110 625), equal on 52 programs; two `work` rows move,
  `equation_two_unknowns.ein` 60 223 facts beyond root where 75 423 (the check's facts gone, the figure before
  S1f.6.12) and `boxes13.ein` 37 where 45.
- **W1 and W2** move the same seven, and refuse `equation_two_unknowns.ein` at load (E-69); firings +369.6 % and
  +41.5 %.

**The batches,** S1f.6.12's (nine-digit operands, a four-digit divisor, its seed): firings a request and the
saturator's best of three, one P-core (`utils/bench_env.sh --core 2 python3 build/s1f.6.38/cost.py 3 head w1 w2 e1
-- 250 1000`, i9-14900HX, `powersave`, 2026-11-14):

| operation | requests | HEAD: firings, ms | W1: firings, ms | W2: firings, ms | E1: firings, ms |
|---|---:|---:|---:|---:|---:|
| add | 1 000 | 45.0, 63.1 | 115.4, 413.4 | 41.0, 65.0 | 27.0, 39.5 |
| sub | 1 000 | 55.0, 79.4 | 148.0, 568.6 | 51.0, 82.3 | 37.0, 56.9 |
| cmp | 1 000 | 34.0, 55.4 | 66.4, 185.1 | 30.0, 58.0 | 16.0, 31.0 |
| mul | 1 000 | 504.0, 812.5 | 1 506.1, 5 781.6 | 499.9, 831.4 | 486.0, 808.7 |
| div | 1 000 | 460.3, 811.7 | 1 110.1, 3 385.9 | 456.3, 772.2 | 447.3, 749.6 |

E1's firings are the theory's before S1f.6.12 to the digit (27, 37, 16, 486, 447): a well-formed numeral costs the
check no firing, only its ten `neq` and two probes a place. Its load pays the stated facts, 42 ms at 1 000 sums
(§ 2.2). The 250-request rows are in `out/cost.md`; each is in proportion.

### 6. The open choices, each with a proposed default

Filed defaulted, the user's to revise (ids under [§ To file](#to-file)):

- **Q1, the mechanism.** **(a) E1, an engine stage first**: the loader states every nested fact a program writes
  as `(written T)`, the representation's rules read it; (b) E2, a constructor's argument domain checked at load, a
  refusal (a malformed program a load error: the user's word, not a default); (c) W1, the rule-level walk (§ 2.1: two to
  three times the firings and three to seven times the time, the trace unreadable, an exception to the closing
  rule's test); (d) none: today's pass, which the ruling rules out.
- **Q2, where decimal's numeral check lives.** **(a) `std.math.decimal-arith`'s own rules,** decimal's own module in
  S1f.6.16's design, beside `n` and `let`: no program changes, and `numeral-prefix-named` can read `named`; (b) a new
  `std.math.decimal` that declares `n` and holds the check, `decimal-arith` importing it: the representation apart
  from its arithmetic, as reply 14 frames it, but every program lists `n` from `std.math.decimal` (S1f.6.35, no
  re-export: the 114 programs' import lines and the pages), and the module below `let` cannot say `(n price 1)` but
  by E-36's cost; (c) `std.math.digits`: no, it is generated facts.
- **Q3, the flag.** **(a) `(not-decimal NUMERAL THING)`**, on the smallest `n` term at fault, `(false)` from it by
  `not-decimal-is-false`; binary's `(not-binary NUMERAL THING)`; (b) on the numeral as written, the outermost `n`,
  which takes a walk down it again, a firing a place; (c) `(malformed NUMERAL)`, the request flag's relation reused.
- **Q4, what E1 states.** **(a) the nested facts of every written fact and the ground ones of every rule's and
  hrule's conclusion**: a malformed numeral written in a rule's text as a whole is caught (r1), and one a rule
  assembles from a variable and a non-digit (r2) is silent and named so; (b) written facts only (r1 and r2 silent:
  two detections HEAD has, lost); (c) (a) and E2's static check of rule texts for r2.
- **Q5, an integer's and a rational's own shape.** **(a) int and rat flag a digit or a named constant where their
  own term's part goes**, `(int 7 z)`, `(int price z)`, `(rat (int …) 7)`, read off `written`: `(not-integer …)`,
  `(not-rational …)`, six rules and a flag each (int prototyped, `stdlib-e1x`): it is the (b) part's *a digit where
  a number goes*, in their own term; (b) silent, named on the page.
- **Q6, an operand of another type.** (a) int and rat flag a natural where an integer goes, and an integer or a
  natural where a rational goes, at their operand (`int-natural-first`, `-second`, prototyped: `i7` dead); **(b)
  silent, as S1f.6.12 left it and named on the page**: it is not in the ruling's list.

`num9` stays silent (reply 9); binary flags as far as decimal does (Q-M1f.268's default, unchanged); `let-to-name`
is no choice: the move needs it, or `32_malformed_name_for_a_name.ein` turns alive.

### 7. Findings

No wrong answer of the engine was met: the two below are a ruled refusal and the documented semantics of `absent`.

- **A relation-parameterised rule a closing rule names makes every program that imports `std.algebra` beside it a
  load error** (E-69 to file). S1f.5.18's check reads `(arith-rule walk-1)` as a name `imply1`'s `?R2` could bind,
  so `(?R2 ?a)` could activate `walk-1` with a fact: `equation_two_unknowns.ein` refused under `stdlib-w1` and
  `-w2`, twenty lines; loaded, 3 states, under `stdlib-w1n`, the same walk unnamed. It bears on S1f.6.16's design
  too, whose `positional` is 78 parameterised rules: its closing rules name none of them, and its trial loaded all
  143 programs, `equation_two_unknowns.ein` among them; a build that named them in a closing rule would meet this.
- **A check that reads an `absent` comes after every calculation** (E-70 to file). An `absent` is decided at the
  closure/world boundary, so a rule with one fires only when the positive closure has stalled: today's
  `operand-undeclared` (`(undeclared …)` is the last thing root derives, after every walk), and W2's walk, whose
  flag comes after all 19 of a rational sum's firings. A *right away* check cannot read an `absent`.

### To file

**Filed 2026-11-14 by the coordinator**, after S1f.6.37's commits: Q-M1f.272 to .277 under these ids, each marked
ruled or accepted ([§ The build, as ruled](#the-build-as-ruled-2026-11-14)); Q-M1f.210 marked ruled; the two
findings as [E-69 and E-70](findings.md), since S1f.6.37 took E-68 (this record named them E-68 and E-69, the ids
that were free when it was written, and `6cca4406`'s message still does).

`open_questions.md` and `findings.md` were being edited by S1f.6.37's agent when this record was written, so the
rows are here, under the ids that were the next free ones then (Q-M1f.271 and E-67 the last), for the coordinator
to file (and renumber if taken):

- **Q-M1f.272 — how a representation module sees a numeral, right away** (Q1): (a) E1, default; (b) E2; (c) W1;
  (d) today's pass.
- **Q-M1f.273 — where decimal's numeral check lives** (Q2): (a) `std.math.decimal-arith`'s own rules, default;
  (b) a new `std.math.decimal`; (c) `std.math.digits`.
- **Q-M1f.274 — the numeral's flag** (Q3): (a) `(not-decimal NUMERAL THING)` on the smallest term at fault,
  default; (b) on the outermost; (c) `(malformed NUMERAL)`.
- **Q-M1f.275 — what the loader states** (Q4): (a) written facts and conclusions' ground terms, default; (b) facts
  only; (c) (a) and E2 for rule texts.
- **Q-M1f.276 — an integer's and a rational's own shape** (Q5): (a) flagged, default; (b) silent.
- **Q-M1f.277 — an operand of another type in int and rat** (Q6): (a) flagged; (b) silent, default.
- **E-69** and **E-70** (filed under those ids; this list said E-68 and E-69 until S1f.6.37 took E-68), as § 7
  states them; neither a wrong answer.

### What the build then does (T1f.6.38.2 onward, on the defaults)

An engine stage first (E1: the loader's statement, its provenance for the core, the reserved name, its pages and a
test per surface), then the stdlib: `std.math.decimal-arith`'s check moved as § 3 says, `let-to-name`, the (b) part
and Q5 in int and rat, one program per new rule under `tests/stdlib/math/`, the malformed tests' claims unchanged
(`(solve :expect (false))`), the mutants, the batch by hand, the sweep old stdlib against new with every golden that
moves named first; then S1f.6.16's build on the moved theory. Not started: the stage stops here for the user.

## The build, as ruled (2026-11-14)

The user, 2026-11-14, on T1: *"E1 yes, separate std.math.decimal module, other defaults fine"*. So Q1 (a), **Q2
(b)** against the proposed default, Q3 (a), Q4 (a), Q5 (a), Q6 (b) (filed as Q-M1f.272 to .277 by the coordinator,
each marked ruled or accepted). By the same ruling's symmetry, **binary's numeral is `std.math.binary`'s**, built by
[S1f.6.16](s1f.6.16_binary_arithmetic.md)'s build on this stage's pattern (a default, the user's to revise).

**Status:** ruled, not started; Q-M1f.272 to .277, E-69 and E-70 filed (2026-11-14). Runs after
[S1f.6.37](s1f.6.37_conclusion_shapes.md) (done 2026-11-14), which edits the same loader and compiler, and before
S1f.6.16's build.

### Tasks

2. **T1f.6.38.2 — E1, the engine.** The loader states `(written T)` for every fact nested at any depth in a fact a
   file writes (a program, a local module, a stdlib module; a stored negation's arguments too), and for the ground
   nested facts of every rule's and hrule's conclusion (Q4 (a)). `written` is a kernel relation as the
   declarations' mirror facts are: reserved, closed, never a hypothesis target, decided at root; how it shows (or
   does not) in a state's listing, `--dump-answer`, `.einb`, the KB views and the read-out follows what the mirror
   facts do, and every surface that moves is named first. A core names the fact as written, not the stated one
   (§ 2.2, item 3). Pages: `defined_behaviour.md`, the reserved names page, the grammar's or the data model's
   statement of what root holds, AGENTS.md § The language. **Verified by invariance:** every tracked program
   answers alike under the old binary and the new (no rule reads `written` yet), and a dumped state reloaded
   states the same `written` facts.
3. **T1f.6.38.3 — `std.math.decimal`, the representation.** A new module, `stdlib/math/decimal.ein`: it declares
   `n` and holds decimal's numeral check over `(written (n P A))` (§ 3: `numeral-digit`, the prefix rules,
   `not-decimal`, `not-decimal-is-false`; `check-digit` leaves `decimal-arith`). `std.math.decimal-arith` imports
   it, as do `int`, `rat` and `num9` through it. **`z` is `std.math.arith`'s** (the user, 2026-11-14: *"z is shared
   by both bases, put it in std.math.arith"*): this build creates `stdlib/math/arith.ein`, arithmetic itself, with
   `z` and, by the user's ruling of the same day on S1f.6.16 (*"division by zero is false in binary arithmetic too,
   so it belongs to arithmetic itself"*), the relation `division-by-zero` and the rule `division-by-zero-is-false`,
   moved out of `decimal-arith`, so every program's imports are rewritten once; `std.math.decimal` imports it, and
   so will `std.math.binary`. Open in the build, a default to file: how a prefix that is a named constant,
   `(n price 1)`, is flagged from a module below `let` (§ 6, Q2 (b): by E-36's cost, or another way the build
   measures). Every program and generator that writes a numeral lists `n` from `std.math.decimal` (no re-export,
   S1f.6.35): a mechanical rewrite, counted.
4. **T1f.6.38.4 — the operand's faults and int and rat.** `let-to-name` in `decimal-arith`; the (b) part in int
   and rat, `int-bad-only`/`-first`/`-second` and `rat-…`, to `(bad-request REQUEST FAULT)`; Q5 (a), `(not-integer
   …)` and `(not-rational …)` for a digit or a named constant in their own terms; Q6 (b), another type silent and
   named on the pages. E-38 closed.
5. **T1f.6.38.5 — the tests, the sweep and the pages.** One program per new rule under `tests/stdlib/math/`, the
   malformed tests' claims unchanged (`(solve :expect (false))`), E-38's 26 probes banked as programs with their
   claims; the mutation census; the batch by hand; the sweep, old stdlib against new on every tracked program,
   **no well-formed program's answer moves** and every malformed one is dead as before, every golden that moves
   named first; `stdlib/README.md`, `tests/README.md`, AGENTS.md's calculator lines; Q-M1f.210 closed; the
   record and the phase README's row.

### Acceptance

- A malformed decimal numeral is flagged `(not-decimal …)` and `(false)` wherever a file writes it, an operand,
  inside an integer or a rational, in a `let`, in a program's own fact, before any request reads it.
- No module but `std.math.decimal` names a numeral's fault; int and rat flag an undeclared name and a digit as an
  operand.
- No well-formed program's answer moves; the firings of a well-formed request are no more than today's.

