# Decisions: the calculator

Decisions about the calculator as a whole: its rule, its tables, `std.math.arith`, `std.math.positional`,
`std.math.convert`, the tagged calls and what is flagged; each module's own are in [`decimal/`](decimal/README.md),
[`int/`](int/README.md), [`rat/`](rat/README.md), [`gcd/`](gcd/README.md), [`binary/`](binary/README.md) and
[`num9/`](num9/README.md).

- [Q-M1f.146](Q-M1f.146-each-digit-table-is-closed-by-a-written-fact.md) — Each digit table is closed by a written fact
- [Q-M1f.148](Q-M1f.148-the-seven-digit-tables-are-one-module.md) — The seven digit tables are one module
- [Q-M1f.177](Q-M1f.177-a-module-over-the-decimal-arithmetic-tags-each.md) — A module over the decimal arithmetic tags each call with its own request
- [Q-M1f.189](Q-M1f.189-the-decimal-arithmetics-tagged-calls-are.md) — The decimal arithmetic's tagged calls are an interface
- [Q-M1f.192](Q-M1f.192-a-program-that-reads-the-binary-tables-imports.md) — A program that reads the binary tables imports `std.math.bits`
- [Q-M1f.210](Q-M1f.210-a-malformed-number-is-its-representations-fault.md) — A malformed number is its representation's fault
- [Q-M1f.228](Q-M1f.228-the-interface-of-the-tagged-calls-is-what-their.md) — The interface of the tagged calls is what their callers write
- [Q-M1f.266](Q-M1f.266-a-base-is-connected-to-the-shared-rules-by-one.md) — A base is connected to the shared rules by one call of a macro
- [Q-M1f.269](Q-M1f.269-division-by-zero-is-a-contradiction.md) — Division by zero is a contradiction of arithmetic itself, in every base
- [Q-M1f.270](Q-M1f.270-the-connecting-macros-body-is-written-by-hand.md) — The connecting macro's body is written by hand
- [Q-M1f.276](Q-M1f.276-an-integers-and-a-rationals-own-shape-is-checked.md) — An integer's and a rational's own shape is checked by its own module
- [Q-M1f.277](Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md) — An operand of another kind of number is answered by silence
- [Q-M1f.283](Q-M1f.283-zero-is-a-relation-of-no-argument-and-each.md) — Zero is a relation of no argument, and each closing rule names the entries below it
- [Q-M1f.298](Q-M1f.298-a-shared-rules-text-writes-the-bases-own-numeral.md) — A shared rule's text writes the base's own numeral
- [Q-M1f.316](Q-M1f.316-the-calculator-flags-each-malformed-request-it.md) — The calculator flags each malformed request it can detect, and the flag is a contradiction
- [Q-M1f.318](Q-M1f.318-a-flag-means-malformed-and-no-flag-does-not-mean.md) — A flag means malformed, and no flag does not mean well-formed
- [Q-M1f.322](Q-M1f.322-zero-is-one-z-in-every-base.md) — Zero is one `z` in every base
- [Q-M1f.323](Q-M1f.323-conversion-between-the-bases-is-a-module.md) — Conversion between the bases is a module of its own
- [Q-M1f.362](Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md) — No rule of the calculator joins two facts on a number
- [Q-M1f.363](Q-M1f.363-a-calculation-is-a-request-fact-answered.md) — A calculation is a request fact, answered by a result fact that repeats its operands
- [Q-M1f.365](Q-M1f.365-the-calculator-closes-the-held-extents.md) — The calculator closes the held extents of its own relations, so arithmetic searches nothing
- [Q-M1f.366](Q-M1f.366-the-calculators-tables-are-generated-facts-every.md) — The calculator's tables are generated facts, every row held to the integers
- [Q-M1f.404](Q-M1f.404-a-numeral-is-checked-where-an-operation-takes-it.md) — A numeral is checked where an operation takes it
- [Q-M1f.518](Q-M1f.518-the-calculators-own-constants-are-not-checked.md) — The calculator's own constants are not checked
- [Q-M1f.520](Q-M1f.520-a-shared-rules-words-are-one-text-under-every.md) — A shared rule's words are one text under every base
