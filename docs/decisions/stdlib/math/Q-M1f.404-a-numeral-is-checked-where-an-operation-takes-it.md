# Q-M1f.404 — A numeral is checked where an operation takes it

A numeral an operation takes from the program is checked once by its representation's rules, and labelled valid or
flagged. A number an operation makes is labelled valid as it is made, and no operation checks it. A numeral no
operation takes is not checked: one in a `let` nobody asks about, in a fact of the program's own relation, or in a
rule's conclusion that never fires.

The look-up of an operand, `std.math.positional`'s `operand-check`, asks `(check N N)` of `std.math.decimal`, or
`(bin-check N N)` of `std.math.binary`, for the numeral it finds, written as the operand or as a `let`'s value;
`std.math.convert` and `std.math.num9` ask it of the number they convert. `std.math.int` asks `(int-check I)` of the
integer an operand stands for and `std.math.rat` `(rat-check Q)` of the rational, each of which checks its own shape
([Q-M1f.276](Q-M1f.276-an-integers-and-a-rationals-own-shape-is-checked.md)) and asks the check of its parts. The
check walks the numeral a place a firing from the units, `(check N REST)`: the walk that reaches `z` labels it
`(valid N)`, and a malformed place stops the walk and derives the representation's flag, `(not-decimal NUMERAL
THING)`, from which `(false)` follows. The ask and the label are facts of the numeral, so a numeral is checked once
whatever reads it. The turn that ends a walk of the arithmetic, and the division's end, label the number they make,
so every number an operation answers with, a tagged call's included, is valid already; `z` is valid by its
representation's closing rule. The loader states nothing of what a file nests.

**Why.** A malformed number is found by the operation that would compute with it, and only there, so the check
reads what an operation takes and needs no statement of the program's text: a check of what a file writes needed
the loader to state a context of its own for every fact a file nests, which no rule could state and which a saved
state could not carry. A number the arithmetic makes is a number by construction. No rule of the calculator joins
two facts on a number ([Q-M1f.362](Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md)): the walk carries
the numeral it checks, so its end labels it with no second premise, and no operation waits for the label. A number
that a program's own rule hands to a second request is that request's operand and is walked again, deriving the
label it has: only the label tells it from a numeral the program wrote, and reading the label beside the operand
would be such a join.

**Held by.** `ein-infer/tests/arith_theory.rs`: `a_malformed_request_is_flagged_once_and_never_answered`, a numeral
no request takes left unchecked among them, and `no_rule_joins_two_facts_on_a_number`; `int_theory.rs`:
`a_malformed_integer_and_an_operands_fault_are_flagged`; `rat_theory.rs`: `a_malformed_rational_is_flagged`, each
with a malformed term no operation takes; under `tests/stdlib/math/decimal/`: `01_well_formed_everywhere.ein`, the
labels of what an operation takes and makes and of nothing else; `02_in_a_let_nobody_asks_about.ein`,
`03_in_a_fact_of_the_programs_own.ein` and `04_in_a_rules_conclusion.ein`, a malformed numeral nobody takes, and
`07_in_a_rules_conclusion_that_fires.ein`, one a rule hands to a request; binary's like under
`tests/stdlib/math/binary/`.
