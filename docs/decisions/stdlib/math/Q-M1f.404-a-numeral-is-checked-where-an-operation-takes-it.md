# Q-M1f.404 — A numeral is checked where an operation takes it

A number an operation makes from valid numbers is valid as it is made, and every other numeral is checked,
whatever its source: one the program writes in a request, one a rule of the program makes up, and one a program or
a module over the calculator hands straight to a tagged call. A numeral is checked once by its representation's
rules, and labelled valid or flagged. A numeral no operation takes is not checked: one in a `let` nobody asks
about, in a fact of the program's own relation, or in a rule's conclusion that never fires.

**Who asks.** The look-up of an operand, `std.math.positional`'s `operand-check`, asks `(check N N)` of
`std.math.decimal`, or `(bin-check N N)` of `std.math.binary`, for the numeral it finds, written as the operand or
as a `let`'s value. A tagged call asks it of each numeral it is handed, whoever made the call: the five of each
base (`add-walk-check-x` and the rest, `std.math.positional`'s) and `std.math.gcd`'s `gcd-call`; then the call is
handed on to the calculator's own walk, `add-run`, `sub-run`, `cmp-run`, `mul-run`, `div-run` or `gcd-run`, which
checks nothing. The calculator calls those walks itself, with numbers it checked or made, so a number an operation
makes is not walked again. `std.math.convert` and `std.math.num9` ask the check of the number they convert.

**An integer and a rational.** `std.math.int` asks `(int-check I)` of the integer an operand stands for and of each
pair of parts one of its calls is handed, `int-norm` and the rest, and `std.math.rat` `(rat-check Q)` of the
rational and of the rationals its calls' parts make; each checks its own shape
([Q-M1f.276](Q-M1f.276-an-integers-and-a-rationals-own-shape-is-checked.md)) and asks the check of its parts, a
rule a shape, so that a check is asked only of what it is for. An integer built from an invalid decimal is
invalid: each part that is a numeral is asked `std.math.decimal`'s check. A rational built from invalid integers is
invalid: an integer numerator is asked `std.math.int`'s check, and a denominator that is a numeral
`std.math.decimal`'s. A rational built from something else verifies it as an operation verifies its operands: a
numerator that is a natural number is asked `std.math.decimal`'s check, though the operations answer such a
rational by silence ([Q-M1f.277](Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md)). An integer and a
rational have no label of their own: their numerals' labels are theirs.

**The walk.** The check walks the numeral a place a firing from the units, `(check N REST)`: the walk that reaches
`z` labels it `(valid N)`, and a malformed place stops the walk and derives the representation's flag on the first
place at fault from the units, `(not-decimal NUMERAL THING)`
([Q-M1f.274](decimal/Q-M1f.274-a-malformed-numeral-is-flagged-on-its-smallest.md)), from which `(false)` follows.
The walk knows no base and is `std.math.positional`'s, `numeral-step`, `numeral-step-zero` and `numeral-valid`,
which each base activates with its representation's relations; the relations, the label and the flags are the
representation's ([Q-M1f.210](Q-M1f.210-a-malformed-number-is-its-representations-fault.md)). The ask and the label
are facts of the numeral, so a numeral is checked once whatever reads it. The turn that ends a walk of the
arithmetic, and the division's end, label the number they make, so every number an operation answers with, a
tagged call's included, is valid already; `z` is valid by its representation's closing rule. The loader states
nothing of what a file nests.

**Why.** A malformed number is found by the operation that would compute with it, and only there, so the check
reads what an operation takes and needs no statement of the program's text: a check of what a file writes needed
the loader to state a context of its own for every fact a file nests, which no rule could state and which a saved
state could not carry. A number the arithmetic makes is a number by construction, so the calculator hands its own
walks the numbers it makes and walks them no more; anything else, a call's numerals among them, has no such
warrant. No rule of the calculator joins two facts on a number
([Q-M1f.362](Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md)): the walk carries the numeral it checks,
so its end labels it with no second premise, and no operation waits for the label; the calls and the walks are two
relations, so that what is checked is told by where it is written and not by a join. A number that a program's own
rule hands to a second request or to a tagged call is checked as any numeral it hands, and is walked again,
deriving the label it has: only the label tells it from a numeral the program wrote, and reading the label beside
it would be such a join.

**Held by.** `ein-infer/tests/arith_theory.rs`: `a_malformed_request_is_flagged_once_and_never_answered`, a numeral
no request takes left unchecked among them, `no_rule_joins_two_facts_on_a_number`,
`one_call_of_the_macro_connects_each_base` (the walk and the calls' rules among `std.math.positional`'s) and
`the_tagged_calls_keep_their_shapes` (each call and the walk it is handed on to); `int_theory.rs`:
`a_malformed_integer_and_an_operands_fault_are_flagged`; `rat_theory.rs`: `a_malformed_rational_is_flagged`, each
with a malformed term no operation takes; under `tests/stdlib/math/decimal/`: `01_well_formed_everywhere.ein`, the
labels of what an operation takes and makes and of nothing else; `02_in_a_let_nobody_asks_about.ein`,
`03_in_a_fact_of_the_programs_own.ein` and `04_in_a_rules_conclusion.ein`, a malformed numeral nobody takes, and
`07_in_a_rules_conclusion_that_fires.ein`, one a rule hands to a request; under `decimal-arith/`:
`51_tagged_calls_checked.ein`, the asks of the five calls exactly the numerals they are handed, and
`52_tagged_call_malformed.ein`, a malformed numeral handed to a call; binary's like under `binary/` and
`binary-arith/` (`23`, `24`); `int/37_calls_checked.ein` and `38_call_malformed_part.ein`;
`rat/32_calls_checked.ein`, `33_call_malformed_part.ein`, `34_natural_numerator.ein` and
`35_natural_numerator_malformed.ein`; `gcd/07_gcd_call.ein` and `13_call_malformed.ein`.
