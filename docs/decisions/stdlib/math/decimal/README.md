# Decisions: `std.math.decimal` and `std.math.decimal-arith`

Decisions about `std.math.decimal` and `std.math.decimal-arith`: the decimal numeral, its check, and its
arithmetic.

- [Q-M1f.149](Q-M1f.149-a-request-may-name-an-operand-by-a-let.md) — A request may name an operand by a `let`
- [Q-M1f.151](Q-M1f.151-the-circuits-own-relations-are-named-with-words.md) — The circuit's own relations are named with words
- [Q-M1f.152](Q-M1f.152-the-decimal-arithmetics-rules-are-written.md) — The decimal arithmetic's rules are written by hand
- [Q-M1f.155](Q-M1f.155-equal-operands-of-a-subtraction-are-answered.md) — Equal operands of a subtraction are answered by both walks, with one fact
- [Q-M1f.156](Q-M1f.156-comparison-is-called-with-a-tag.md) — Comparison is called with a tag
- [Q-M1f.157](Q-M1f.157-each-request-looks-its-operands-up-by-rules.md) — Each request looks its operands up by rules of its own
- [Q-M1f.158](Q-M1f.158-the-comparators-table-keeps-the-order-below-as.md) — The comparator's table keeps the order below as its first column
- [Q-M1f.159](Q-M1f.159-multiplication-is-called-with-a-tag.md) — Multiplication is called with a tag
- [Q-M1f.160](Q-M1f.160-a-zero-digit-of-the-multiplier-is-a-row-like-any.md) — A zero digit of the multiplier is a row like any other
- [Q-M1f.163](Q-M1f.163-a-division-by-zero-is-flagged-wherever-division.md) — A division by zero is flagged wherever division is called
- [Q-M1f.164](Q-M1f.164-divisions-families-of-nine-rules-are-written.md) — Division's families of nine rules are written by hand, and a test holds them alike
- [Q-M1f.165](Q-M1f.165-a-program-that-divides-by-zero-is-decided.md) — A program that divides by zero is decided by root alone
- [Q-M1f.206](Q-M1f.206-an-operands-fault-travels-with-its-tag.md) — An operand's fault travels with its tag to the request
- [Q-M1f.207](Q-M1f.207-a-let-that-cannot-be-read-is-flagged-whether.md) — A `let` that cannot be read is flagged, whether or not a request uses it
- [Q-M1f.208](Q-M1f.208-two-guards-say-that-an-operand-is-a-name.md) — Two guards say that an operand is a name
- [Q-M1f.211](Q-M1f.211-a-public-division-by-zero-holds-two-flags.md) — A public division by zero holds two flags, the request's and its call's
- [Q-M1f.261](Q-M1f.261-the-decimal-arithmetic-is-std-math-decimal-arith.md) — The decimal arithmetic is `std.math.decimal-arith`
- [Q-M1f.273](Q-M1f.273-the-decimal-numeral-is-std-math-decimal-s-which.md) — The decimal numeral is `std.math.decimal`'s, which checks it
- [Q-M1f.274](Q-M1f.274-a-malformed-numeral-is-flagged-on-its-smallest.md) — A malformed numeral is flagged on its smallest term at fault
- [Q-M1f.278](Q-M1f.278-a-named-constant-is-told-by-named-the-numeral.md) — A named constant is told by `named`, the numeral module's relation
- [Q-M1f.281](Q-M1f.281-a-let-that-names-a-digit-is-flagged.md) — A `let` that names a digit is flagged
- [Q-M1f.317](Q-M1f.317-a-name-no-let-states-is-flagged-as-undeclared.md) — A name no `let` states is flagged as undeclared
- [Q-M1f.324](Q-M1f.324-decimal-division-is-long-division-each-digit.md) — Decimal division is long division, each digit found by bisection
- [Q-M1f.361](Q-M1f.361-a-natural-number-is-a-numeral-of-digit-names.md) — A natural number is a numeral of digit names, in reading order
- [Q-M1f.364](Q-M1f.364-a-subtraction-answers-the-difference.md) — A subtraction answers the difference and the shortfall
