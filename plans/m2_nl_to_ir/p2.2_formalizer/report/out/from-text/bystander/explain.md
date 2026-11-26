# Region trace

```text
  asked             is R one state? (--single)
  verdict           Solution — R is one state
  decided           yes — R holds no other state
  lattice           monotone (the static check) — R is convex: no hole
  universe          0 candidates
  route             the state route, each state at most once — L1 dead is upward-closed · L2 discharged is upward-closed
  work              1 saturation · 0 facts beyond root
```

## Root — 61 steps

### Step 1 — `std.math.arith.arith`

> arithmetic is computed and never guessed: its flag is closed

Premises: —

Derives `__closed__(std.math.arith.division-by-zero)`; `__closed__(std.math.arith.arith-module-rule)`; `std.math.arith.arith-module-rule(std.math.arith.division-by-zero-is-false)`.

### Step 2 — `std.math.decimal.decimal`

> a numeral is written, and never guessed: every relation of the module is closed

Premises: —

Derives `__closed__(std.math.digits.nz)`; `__closed__(n)`; `__closed__(std.math.decimal.named)`; `__closed__(std.math.decimal.check)`; `__closed__(std.math.decimal.valid)`; `__closed__(std.math.decimal.not-decimal)`; `__closed__(std.math.decimal.decimal-rule)`; `std.math.decimal.valid(z)`; `std.math.decimal.decimal-rule(std.math.arith.arith)`; `std.math.decimal.decimal-rule(std.math.decimal.numeral-digit)`; `std.math.decimal.decimal-rule(std.math.decimal.numeral-prefix-digit)`; `std.math.decimal.decimal-rule(std.math.decimal.numeral-prefix-zero)`; `std.math.decimal.decimal-rule(std.math.decimal.numeral-prefix-named)`; `std.math.decimal.decimal-rule(std.math.decimal.not-decimal-is-false)`.

### Step 3 — `std.math.decimal-arith.decimal-arith`

> arithmetic is computed and never guessed: every relation of the theory is closed

Premises: —

Derives `__closed__(std.math.digits.fa)`; `__closed__(std.math.digits.fs)`; `__closed__(std.math.digits.md)`; `__closed__(std.math.digits.cl)`; `__closed__(std.math.digits.nz)`; `__closed__(std.math.digits.mid)`; `__closed__(std.math.digits.adj)`; `__closed__(std.math.decimal-arith.b)`; `__closed__(std.math.decimal-arith.let)`; `__closed__(std.math.decimal-arith.add)`; `__closed__(std.math.decimal-arith.sum-of)`; `__closed__(sub)`; `__closed__(diff-of)`; `__closed__(std.math.decimal-arith.cmp)`; `__closed__(std.math.decimal-arith.cmp-of)`; `__closed__(std.math.decimal-arith.mul)`; `__closed__(std.math.decimal-arith.product-of)`; `__closed__(std.math.decimal-arith.div)`; `__closed__(std.math.decimal-arith.quot-of)`; `__closed__(std.math.decimal-arith.malformed)`; `__closed__(std.math.decimal-arith.undeclared)`; `__closed__(std.math.decimal-arith.bad-operand)`; `__closed__(std.math.decimal-arith.bad-request)`; `__closed__(std.math.decimal-arith.not-a-number)`; `__closed__(std.math.decimal-arith.no-let)`; `__closed__(std.math.decimal-arith.operand)`; `__closed__(std.math.decimal-arith.operand-is)`; `__closed__(std.math.decimal-arith.add-walk)`; `__closed__(std.math.decimal-arith.add-run)`; `__closed__(std.math.decimal-arith.strip)`; `__closed__(std.math.decimal-arith.turn)`; `__closed__(std.math.decimal-arith.done)`; `__closed__(std.math.decimal-arith.sub-walk)`; `__closed__(std.math.decimal-arith.sub-run)`; `__closed__(std.math.decimal-arith.short)`; `__closed__(std.math.decimal-arith.cmp-walk)`; `__closed__(std.math.decimal-arith.cmp-run)`; `__closed__(std.math.decimal-arith.order-is)`; `__closed__(std.math.decimal-arith.top-first)`; `__closed__(std.math.decimal-arith.top-first-is)`; `__closed__(std.math.decimal-arith.mul-walk)`; `__closed__(std.math.decimal-arith.mul-call)`; `__closed__(std.math.decimal-arith.mul-run)`; `__closed__(std.math.decimal-arith.mul-step)`; `__closed__(std.math.decimal-arith.div-call)`; `__closed__(std.math.decimal-arith.div-run)`; `__closed__(std.math.decimal-arith.div-done)`; `__closed__(std.math.decimal-arith.multiples)`; `__closed__(std.math.decimal-arith.div-step)`; `__closed__(std.math.decimal-arith.div-choose)`; `__closed__(std.math.decimal-arith.div-digit)`; `__closed__(std.math.decimal-arith.sum)`; `__closed__(std.math.decimal-arith.add-first)`; `__closed__(std.math.decimal-arith.add-second)`; `__closed__(std.math.decimal-arith.difference)`; `__closed__(std.math.decimal-arith.shortfall)`; `__closed__(std.math.decimal-arith.sub-first)`; `__closed__(std.math.decimal-arith.sub-second)`; `__closed__(std.math.decimal-arith.order)`; `__closed__(std.math.decimal-arith.cmp-first)`; `__closed__(std.math.decimal-arith.cmp-second)`; `__closed__(std.math.decimal-arith.product)`; `__closed__(std.math.decimal-arith.mul-first)`; `__closed__(std.math.decimal-arith.mul-second)`; `__closed__(std.math.decimal-arith.mul-digits)`; `__closed__(std.math.decimal-arith.mul-row-of)`; `__closed__(std.math.decimal-arith.mul-sum)`; `__closed__(std.math.decimal-arith.quotient)`; `__closed__(std.math.decimal-arith.div-first)`; `__closed__(std.math.decimal-arith.div-second)`; `__closed__(std.math.decimal-arith.div-m1)`; `__closed__(std.math.decimal-arith.div-m2)`; `__closed__(std.math.decimal-arith.div-m3)`; `__closed__(std.math.decimal-arith.div-m4)`; `__closed__(std.math.decimal-arith.div-m5)`; `__closed__(std.math.decimal-arith.div-m6)`; `__closed__(std.math.decimal-arith.div-m7)`; `__closed__(std.math.decimal-arith.div-m8)`; `__closed__(std.math.decimal-arith.div-m9)`; `__closed__(std.math.decimal-arith.div-digits)`; `__closed__(std.math.decimal-arith.div-trial)`; `__closed__(std.math.decimal-arith.arith-rule)`; `std.math.decimal-arith.arith-rule(std.math.arith.arith)`; `std.math.decimal-arith.arith-rule(std.math.decimal.decimal)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.mul-column)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.mul-walk-end)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.mul-carry-out)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.mul-row)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.mul-first-row)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.mul-add-row)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-multiples)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-multiple-2)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-multiple-3)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-multiple-4)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-multiple-5)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-multiple-6)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-multiple-7)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-multiple-8)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-multiple-9)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-turn)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-bring)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-bring-first)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-bring-zero)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-try-1)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-try-2)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-try-3)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-try-4)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-try-5)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-try-6)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-try-7)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-try-8)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-try-9)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-fits)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-too-big)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-digit-found)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-next)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-next-zero)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.div-next-none)`; `std.math.decimal-arith.arith-rule(std.math.decimal-arith.operand-undeclared)`; `std.math.positional.operand-zero(std.math.decimal-arith.operand, std.math.decimal-arith.operand-is)`; `std.math.positional.operand-number(std.math.decimal-arith.operand, n, std.math.decimal-arith.operand-is)`; `std.math.positional.let-named(std.math.decimal-arith.let, std.math.decimal.named)`; `std.math.positional.operand-name(std.math.decimal-arith.operand, std.math.decimal.named, std.math.decimal-arith.let, std.math.decimal-arith.operand-is)`; `std.math.positional.operand-check(std.math.decimal-arith.operand-is, n, std.math.decimal.check)`; `std.math.positional.numeral-step(std.math.decimal.check, n, std.math.digits.nz)`; `std.math.positional.numeral-step-zero(std.math.decimal.check, n)`; `std.math.positional.numeral-valid(std.math.decimal.check, std.math.decimal.valid)`; `std.math.positional.add-walk-runs(std.math.decimal-arith.add-walk, std.math.decimal-arith.add-run)`; `std.math.positional.add-walk-check-x(std.math.decimal-arith.add-walk, n, std.math.decimal.check)`; `std.math.positional.add-walk-check-y(std.math.decimal-arith.add-walk, n, std.math.decimal.check)`; `std.math.positional.sub-walk-runs(std.math.decimal-arith.sub-walk, std.math.decimal-arith.sub-run)`; `std.math.positional.sub-walk-check-x(std.math.decimal-arith.sub-walk, n, std.math.decimal.check)`; `std.math.positional.sub-walk-check-y(std.math.decimal-arith.sub-walk, n, std.math.decimal.check)`; `std.math.positional.cmp-walk-runs(std.math.decimal-arith.cmp-walk, std.math.decimal-arith.cmp-run)`; `std.math.positional.cmp-walk-check-x(std.math.decimal-arith.cmp-walk, n, std.math.decimal.check)`; `std.math.positional.cmp-walk-check-y(std.math.decimal-arith.cmp-walk, n, std.math.decimal.check)`; `std.math.positional.mul-call-runs(std.math.decimal-arith.mul-call, std.math.decimal-arith.mul-run)`; `std.math.positional.mul-call-check-x(std.math.decimal-arith.mul-call, n, std.math.decimal.check)`; `std.math.positional.mul-call-check-y(std.math.decimal-arith.mul-call, n, std.math.decimal.check)`; `std.math.positional.div-call-runs(std.math.decimal-arith.div-call, std.math.decimal-arith.div-run)`; `std.math.positional.div-call-check-x(std.math.decimal-arith.div-call, n, std.math.decimal.check)`; `std.math.positional.div-call-check-y(std.math.decimal-arith.div-call, n, std.math.decimal.check)`; `std.math.positional.add-column(std.math.decimal-arith.add-run, n, std.math.digits.fa, std.math.decimal-arith.b)`; `std.math.positional.add-column-first(std.math.decimal-arith.add-run, n, std.math.digits.fa, std.math.decimal-arith.b)`; `std.math.positional.add-column-second(std.math.decimal-arith.add-run, n, std.math.digits.fa, std.math.decimal-arith.b)`; `std.math.positional.add-end(std.math.decimal-arith.add-run, std.math.decimal-arith.strip)`; `std.math.positional.add-carry-out(std.math.decimal-arith.add-run, std.math.decimal-arith.turn, std.math.decimal-arith.b)`; `std.math.positional.strip-zero(std.math.decimal-arith.strip, std.math.decimal-arith.b)`; `std.math.positional.strip-top(std.math.decimal-arith.strip, std.math.decimal-arith.b, std.math.digits.nz, std.math.decimal-arith.turn)`; `std.math.positional.strip-none(std.math.decimal-arith.strip, std.math.decimal-arith.done)`; `std.math.positional.turn-step(std.math.decimal-arith.turn, std.math.decimal-arith.b, n)`; `std.math.positional.turn-end(std.math.decimal-arith.turn, std.math.decimal-arith.done, std.math.decimal.valid)`; `std.math.positional.add-ask(std.math.decimal-arith.add, std.math.decimal-arith.operand, std.math.decimal-arith.add-first)`; `std.math.positional.add-ask-second(std.math.decimal-arith.operand-is, std.math.decimal-arith.add-first, std.math.decimal-arith.operand, std.math.decimal-arith.add-second)`; `std.math.positional.add-start(std.math.decimal-arith.operand-is, std.math.decimal-arith.add-second, std.math.decimal-arith.add-run, std.math.decimal-arith.sum)`; `std.math.positional.add-answer(std.math.decimal-arith.done, std.math.decimal-arith.sum, std.math.decimal-arith.sum-of)`; `std.math.positional.sub-column(std.math.decimal-arith.sub-run, n, std.math.digits.fs, std.math.decimal-arith.b)`; `std.math.positional.sub-column-first(std.math.decimal-arith.sub-run, n, std.math.digits.fs, std.math.decimal-arith.b)`; `std.math.positional.sub-column-second(std.math.decimal-arith.sub-run, n, std.math.digits.fs, std.math.decimal-arith.b)`; `std.math.positional.sub-end(std.math.decimal-arith.sub-run, std.math.decimal-arith.strip)`; `std.math.positional.sub-borrow-out(std.math.decimal-arith.sub-run, std.math.decimal-arith.short)`; `std.math.positional.sub-ask(sub, std.math.decimal-arith.operand, std.math.decimal-arith.sub-first)`; `std.math.positional.sub-ask-second(std.math.decimal-arith.operand-is, std.math.decimal-arith.sub-first, std.math.decimal-arith.operand, std.math.decimal-arith.sub-second)`; `std.math.positional.sub-start(std.math.decimal-arith.operand-is, std.math.decimal-arith.sub-second, std.math.decimal-arith.sub-run, std.math.decimal-arith.difference, std.math.decimal-arith.shortfall)`; `std.math.positional.sub-answer(std.math.decimal-arith.done, std.math.decimal-arith.difference, diff-of)`; `std.math.positional.sub-answer-short(std.math.decimal-arith.done, std.math.decimal-arith.shortfall, diff-of)`; `std.math.positional.cmp-column(std.math.decimal-arith.cmp-run, n, std.math.digits.cl)`; `std.math.positional.cmp-column-first(std.math.decimal-arith.cmp-run, n, std.math.digits.cl)`; `std.math.positional.cmp-column-second(std.math.decimal-arith.cmp-run, n, std.math.digits.cl)`; `std.math.positional.cmp-end(std.math.decimal-arith.cmp-run, std.math.decimal-arith.order-is)`; `std.math.positional.cmp-ask(std.math.decimal-arith.cmp, std.math.decimal-arith.operand, std.math.decimal-arith.cmp-first)`; `std.math.positional.cmp-ask-second(std.math.decimal-arith.operand-is, std.math.decimal-arith.cmp-first, std.math.decimal-arith.operand, std.math.decimal-arith.cmp-second)`; `std.math.positional.cmp-start(std.math.decimal-arith.operand-is, std.math.decimal-arith.cmp-second, std.math.decimal-arith.cmp-run, std.math.decimal-arith.order)`; `std.math.positional.cmp-answer(std.math.decimal-arith.order-is, std.math.decimal-arith.order, std.math.decimal-arith.cmp-of)`; `std.math.positional.top-first-step(std.math.decimal-arith.top-first, n, std.math.decimal-arith.b)`; `std.math.positional.top-first-end(std.math.decimal-arith.top-first, std.math.decimal-arith.top-first-is)`; `std.math.positional.mul-turn(std.math.decimal-arith.mul-run, std.math.decimal-arith.top-first, std.math.decimal-arith.mul-digits)`; `std.math.positional.mul-begin(std.math.decimal-arith.top-first-is, std.math.decimal-arith.mul-digits, std.math.decimal-arith.mul-step)`; `std.math.positional.mul-next(std.math.decimal-arith.done, std.math.decimal-arith.mul-sum, std.math.decimal-arith.mul-step)`; `std.math.positional.mul-end(std.math.decimal-arith.mul-step, std.math.decimal-arith.done)`; `std.math.positional.mul-ask(std.math.decimal-arith.mul, std.math.decimal-arith.operand, std.math.decimal-arith.mul-first)`; `std.math.positional.mul-ask-second(std.math.decimal-arith.operand-is, std.math.decimal-arith.mul-first, std.math.decimal-arith.operand, std.math.decimal-arith.mul-second)`; `std.math.positional.mul-start(std.math.decimal-arith.operand-is, std.math.decimal-arith.mul-second, std.math.decimal-arith.mul-run, std.math.decimal-arith.product)`; `std.math.positional.mul-answer(std.math.decimal-arith.done, std.math.decimal-arith.product, std.math.decimal-arith.product-of)`; `std.math.positional.div-begin(std.math.decimal-arith.top-first-is, std.math.decimal-arith.div-digits, std.math.decimal-arith.div-step)`; `std.math.positional.div-end(std.math.decimal-arith.div-step, std.math.decimal-arith.div-done, std.math.decimal.valid)`; `std.math.positional.div-ask(std.math.decimal-arith.div, std.math.decimal-arith.operand, std.math.decimal-arith.div-first)`; `std.math.positional.div-ask-second(std.math.decimal-arith.operand-is, std.math.decimal-arith.div-first, std.math.decimal-arith.operand, std.math.decimal-arith.div-second)`; `std.math.positional.div-start(std.math.decimal-arith.operand-is, std.math.decimal-arith.div-second, std.math.decimal-arith.div-call, std.math.decimal-arith.quotient)`; `std.math.positional.div-answer(std.math.decimal-arith.div-done, std.math.decimal-arith.quotient, std.math.decimal-arith.quot-of)`; `std.math.positional.div-by-zero(std.math.decimal-arith.done, std.math.decimal-arith.div-m1, std.math.decimal-arith.quotient, std.math.decimal-arith.div)`; `std.math.positional.div-call-by-zero(std.math.decimal-arith.done, std.math.decimal-arith.div-m1, std.math.decimal-arith.div-call)`; `std.math.positional.operand-digit(std.math.decimal-arith.operand, std.math.digits.nz, std.math.decimal-arith.bad-operand, std.math.decimal-arith.not-a-number, n)`; `std.math.positional.operand-digit-zero(std.math.decimal-arith.operand, std.math.decimal-arith.bad-operand, std.math.decimal-arith.not-a-number)`; `std.math.positional.let-twice(std.math.decimal-arith.let, std.math.decimal-arith.malformed)`; `std.math.positional.let-zero(std.math.decimal-arith.let, std.math.decimal-arith.malformed)`; `std.math.positional.let-digit(std.math.decimal-arith.let, std.math.digits.nz, std.math.decimal-arith.malformed)`; `std.math.positional.let-digit-zero(std.math.decimal-arith.let, std.math.decimal-arith.malformed)`; `std.math.positional.let-to-name(std.math.decimal-arith.let, std.math.decimal.named, std.math.decimal-arith.malformed)`; `std.math.positional.let-to-digit(std.math.decimal-arith.let, std.math.digits.nz, std.math.decimal-arith.malformed, n)`; `std.math.positional.let-to-digit-zero(std.math.decimal-arith.let, std.math.decimal-arith.malformed)`; `std.math.positional.bad-add-first(std.math.decimal-arith.bad-operand, std.math.decimal-arith.add-first, std.math.decimal-arith.bad-request, std.math.decimal-arith.add)`; `std.math.positional.bad-add-second(std.math.decimal-arith.bad-operand, std.math.decimal-arith.add-second, std.math.decimal-arith.bad-request, std.math.decimal-arith.add)`; `std.math.positional.bad-sub-first(std.math.decimal-arith.bad-operand, std.math.decimal-arith.sub-first, std.math.decimal-arith.bad-request, sub)`; `std.math.positional.bad-sub-second(std.math.decimal-arith.bad-operand, std.math.decimal-arith.sub-second, std.math.decimal-arith.bad-request, sub)`; `std.math.positional.bad-cmp-first(std.math.decimal-arith.bad-operand, std.math.decimal-arith.cmp-first, std.math.decimal-arith.bad-request, std.math.decimal-arith.cmp)`; `std.math.positional.bad-cmp-second(std.math.decimal-arith.bad-operand, std.math.decimal-arith.cmp-second, std.math.decimal-arith.bad-request, std.math.decimal-arith.cmp)`; `std.math.positional.bad-mul-first(std.math.decimal-arith.bad-operand, std.math.decimal-arith.mul-first, std.math.decimal-arith.bad-request, std.math.decimal-arith.mul)`; `std.math.positional.bad-mul-second(std.math.decimal-arith.bad-operand, std.math.decimal-arith.mul-second, std.math.decimal-arith.bad-request, std.math.decimal-arith.mul)`; `std.math.positional.bad-div-first(std.math.decimal-arith.bad-operand, std.math.decimal-arith.div-first, std.math.decimal-arith.bad-request, std.math.decimal-arith.div)`; `std.math.positional.bad-div-second(std.math.decimal-arith.bad-operand, std.math.decimal-arith.div-second, std.math.decimal-arith.bad-request, std.math.decimal-arith.div)`; `std.math.positional.bad-request-malformed(std.math.decimal-arith.bad-request, std.math.decimal-arith.not-a-number, std.math.decimal-arith.malformed)`; `std.math.positional.bad-request-undeclared(std.math.decimal-arith.bad-request, std.math.decimal-arith.no-let, std.math.decimal-arith.undeclared)`; `std.math.positional.malformed-is-false(std.math.decimal-arith.malformed)`; `std.math.positional.undeclared-is-false(std.math.decimal-arith.undeclared)`.

### Step 4 — `giving-asks-the-difference`

> Bob had (n z 5) and gave (n z 2): ask (n z 5) less (n z 2)

Premises: `gave(Bob, n(z, 2), Apple, Alice)`, `has(Bob, n(z, 5), Apple)` — from Bob gave 2 apples to Alice., Bob has 5 apples.

Derives `sub(n(z, 5), n(z, 2))`.

### Step 5 — `std.math.positional.sub-ask`

> (n z 5) - (n z 2) is asked: first, the number (n z 5) stands for

Premises: `sub(n(z, 5), n(z, 2))`

Derives `std.math.decimal-arith.operand(std.math.decimal-arith.sub-first(n(z, 5), n(z, 2)), n(z, 5))`.

### Step 6 — `std.math.positional.operand-number`

> a number stands for itself

Premises: `std.math.decimal-arith.operand(std.math.decimal-arith.sub-first(n(z, 5), n(z, 2)), n(z, 5))`

Derives `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-first(n(z, 5), n(z, 2)), n(z, 5))`.

### Step 7 — `std.math.positional.operand-check`

> the operand found for (std.math.decimal-arith.sub-first (n z 5) (n z 2)) is (n z 5): it is checked, once, as a number

Premises: `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-first(n(z, 5), n(z, 2)), n(z, 5))`

Derives `std.math.decimal.check(n(z, 5), n(z, 5))`.

### Step 8 — `std.math.positional.sub-ask-second`

> the first operand is (n z 5): now the number (n z 2) stands for

Premises: `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-first(n(z, 5), n(z, 2)), n(z, 5))`

Derives `std.math.decimal-arith.operand(std.math.decimal-arith.sub-second(n(z, 5), n(z, 2), n(z, 5)), n(z, 2))`.

### Step 9 — `std.math.positional.numeral-step`

> the digit 5 is one: z is still to check

Premises: `std.math.decimal.check(n(z, 5), n(z, 5))`, `std.math.digits.nz(5)`

Derives `std.math.decimal.check(n(z, 5), z)`.

### Step 10 — `std.math.positional.operand-number`

> a number stands for itself

Premises: `std.math.decimal-arith.operand(std.math.decimal-arith.sub-second(n(z, 5), n(z, 2), n(z, 5)), n(z, 2))`

Derives `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-second(n(z, 5), n(z, 2), n(z, 5)), n(z, 2))`.

### Step 11 — `std.math.positional.numeral-valid`

> every place of (n z 5) holds a digit, and it ends in z: it is a number

Premises: `std.math.decimal.check(n(z, 5), z)`

Derives `std.math.decimal.valid(n(z, 5))`.

### Step 12 — `std.math.positional.operand-check`

> the operand found for (std.math.decimal-arith.sub-second (n z 5) (n z 2) (n z 5)) is (n z 2): it is checked, once, as a number

Premises: `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-second(n(z, 5), n(z, 2), n(z, 5)), n(z, 2))`

Derives `std.math.decimal.check(n(z, 2), n(z, 2))`.

### Step 13 — `std.math.positional.sub-start`

> take (n z 2) from (n z 5), and (n z 5) from (n z 2), from the units, with no borrow in: one of the two has an answer

Premises: `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-second(n(z, 5), n(z, 2), n(z, 5)), n(z, 2))`

Derives `std.math.decimal-arith.sub-run(std.math.decimal-arith.difference(n(z, 5), n(z, 2)), n(z, 5), n(z, 2), 0, z)`; `std.math.decimal-arith.sub-run(std.math.decimal-arith.shortfall(n(z, 5), n(z, 2)), n(z, 2), n(z, 5), 0, z)`.

### Step 14 — `std.math.positional.numeral-step`

> the digit 2 is one: z is still to check

Premises: `std.math.decimal.check(n(z, 2), n(z, 2))`, `std.math.digits.nz(2)`

Derives `std.math.decimal.check(n(z, 2), z)`.

### Step 15 — `std.math.positional.sub-column`

> 5 - 2 - borrow 0 is 3, borrow 0: 3 goes in front of the digits so far

Premises: `std.math.decimal-arith.sub-run(std.math.decimal-arith.difference(n(z, 5), n(z, 2)), n(z, 5), n(z, 2), 0, z)`, `std.math.digits.fs(5, 2, 0, 3, 0)`

Derives `std.math.decimal-arith.sub-run(std.math.decimal-arith.difference(n(z, 5), n(z, 2)), z, z, 0, std.math.decimal-arith.b(3, z))`.

### Step 16 — `std.math.positional.sub-column`

> 2 - 5 - borrow 0 is 7, borrow 1: 7 goes in front of the digits so far

Premises: `std.math.decimal-arith.sub-run(std.math.decimal-arith.shortfall(n(z, 5), n(z, 2)), n(z, 2), n(z, 5), 0, z)`, `std.math.digits.fs(2, 5, 0, 7, 1)`

Derives `std.math.decimal-arith.sub-run(std.math.decimal-arith.shortfall(n(z, 5), n(z, 2)), z, z, 1, std.math.decimal-arith.b(7, z))`.

### Step 17 — `std.math.positional.numeral-valid`

> every place of (n z 2) holds a digit, and it ends in z: it is a number

Premises: `std.math.decimal.check(n(z, 2), z)`

Derives `std.math.decimal.valid(n(z, 2))`.

### Step 18 — `std.math.positional.sub-end`

> no column is left and no borrow: the digits are (std.math.decimal-arith.b 3 z)

Premises: `std.math.decimal-arith.sub-run(std.math.decimal-arith.difference(n(z, 5), n(z, 2)), z, z, 0, std.math.decimal-arith.b(3, z))`

Derives `std.math.decimal-arith.strip(std.math.decimal-arith.difference(n(z, 5), n(z, 2)), std.math.decimal-arith.b(3, z))`.

### Step 19 — `std.math.positional.sub-borrow-out`

> no column is left and a borrow is: the number taken from is the smaller

Premises: `std.math.decimal-arith.sub-run(std.math.decimal-arith.shortfall(n(z, 5), n(z, 2)), z, z, 1, std.math.decimal-arith.b(7, z))`

Derives `std.math.decimal-arith.short(std.math.decimal-arith.shortfall(n(z, 5), n(z, 2)))`.

### Step 20 — `std.math.positional.strip-top`

> the top digit 3 is not zero: no high zero is left

Premises: `std.math.decimal-arith.strip(std.math.decimal-arith.difference(n(z, 5), n(z, 2)), std.math.decimal-arith.b(3, z))`, `std.math.digits.nz(3)`

Derives `std.math.decimal-arith.turn(std.math.decimal-arith.difference(n(z, 5), n(z, 2)), std.math.decimal-arith.b(3, z), z)`.

### Step 21 — `std.math.positional.turn-step`

> the digit 3 goes after z

Premises: `std.math.decimal-arith.turn(std.math.decimal-arith.difference(n(z, 5), n(z, 2)), std.math.decimal-arith.b(3, z), z)`

Derives `std.math.decimal-arith.turn(std.math.decimal-arith.difference(n(z, 5), n(z, 2)), z, n(z, 3))`.

### Step 22 — `std.math.positional.turn-end`

> turned round: the answer is (n z 3), a number as it is made

Premises: `std.math.decimal-arith.turn(std.math.decimal-arith.difference(n(z, 5), n(z, 2)), z, n(z, 3))`

Derives `std.math.decimal-arith.done(std.math.decimal-arith.difference(n(z, 5), n(z, 2)), n(z, 3))`; `std.math.decimal.valid(n(z, 3))`.

### Step 23 — `std.math.positional.sub-answer`

> (n z 5) - (n z 2) = (n z 3), and nothing is short

Premises: `std.math.decimal-arith.done(std.math.decimal-arith.difference(n(z, 5), n(z, 2)), n(z, 3))`

Derives `diff-of(n(z, 5), n(z, 2), n(z, 3), z)`.

### Step 24 — `giving-is-the-difference`

> (n z 5) less (n z 2) is (n z 3), so Bob has (n z 3) left

Premises: `gave(Bob, n(z, 2), Apple, Alice)`, `has(Bob, n(z, 5), Apple)`, `diff-of(n(z, 5), n(z, 2), n(z, 3), z)` — from Bob gave 2 apples to Alice., Bob has 5 apples.

Derives `has(Bob, n(z, 3), Apple)`.

### Step 25 — `giving-asks-the-difference`

> Bob had (n z 3) and gave (n z 2): ask (n z 3) less (n z 2)

Premises: `gave(Bob, n(z, 2), Apple, Alice)`, `has(Bob, n(z, 3), Apple)` — from Bob gave 2 apples to Alice.

Derives `sub(n(z, 3), n(z, 2))`.

### Step 26 — `std.math.positional.sub-ask`

> (n z 3) - (n z 2) is asked: first, the number (n z 3) stands for

Premises: `sub(n(z, 3), n(z, 2))`

Derives `std.math.decimal-arith.operand(std.math.decimal-arith.sub-first(n(z, 3), n(z, 2)), n(z, 3))`.

### Step 27 — `std.math.positional.operand-number`

> a number stands for itself

Premises: `std.math.decimal-arith.operand(std.math.decimal-arith.sub-first(n(z, 3), n(z, 2)), n(z, 3))`

Derives `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-first(n(z, 3), n(z, 2)), n(z, 3))`.

### Step 28 — `std.math.positional.operand-check`

> the operand found for (std.math.decimal-arith.sub-first (n z 3) (n z 2)) is (n z 3): it is checked, once, as a number

Premises: `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-first(n(z, 3), n(z, 2)), n(z, 3))`

Derives `std.math.decimal.check(n(z, 3), n(z, 3))`.

### Step 29 — `std.math.positional.sub-ask-second`

> the first operand is (n z 3): now the number (n z 2) stands for

Premises: `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-first(n(z, 3), n(z, 2)), n(z, 3))`

Derives `std.math.decimal-arith.operand(std.math.decimal-arith.sub-second(n(z, 3), n(z, 2), n(z, 3)), n(z, 2))`.

### Step 30 — `std.math.positional.numeral-step`

> the digit 3 is one: z is still to check

Premises: `std.math.decimal.check(n(z, 3), n(z, 3))`, `std.math.digits.nz(3)`

Derives `std.math.decimal.check(n(z, 3), z)`.

### Step 31 — `std.math.positional.operand-number`

> a number stands for itself

Premises: `std.math.decimal-arith.operand(std.math.decimal-arith.sub-second(n(z, 3), n(z, 2), n(z, 3)), n(z, 2))`

Derives `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-second(n(z, 3), n(z, 2), n(z, 3)), n(z, 2))`.

### Step 32 — `std.math.positional.numeral-valid`

> every place of (n z 3) holds a digit, and it ends in z: it is a number

Premises: `std.math.decimal.check(n(z, 3), z)`

Derives `std.math.decimal.valid(n(z, 3))`.

### Step 33 — `std.math.positional.operand-check`

> the operand found for (std.math.decimal-arith.sub-second (n z 3) (n z 2) (n z 3)) is (n z 2): it is checked, once, as a number

Premises: `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-second(n(z, 3), n(z, 2), n(z, 3)), n(z, 2))`

Derives `std.math.decimal.check(n(z, 2), n(z, 2))`.

### Step 34 — `std.math.positional.sub-start`

> take (n z 2) from (n z 3), and (n z 3) from (n z 2), from the units, with no borrow in: one of the two has an answer

Premises: `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-second(n(z, 3), n(z, 2), n(z, 3)), n(z, 2))`

Derives `std.math.decimal-arith.sub-run(std.math.decimal-arith.difference(n(z, 3), n(z, 2)), n(z, 3), n(z, 2), 0, z)`; `std.math.decimal-arith.sub-run(std.math.decimal-arith.shortfall(n(z, 3), n(z, 2)), n(z, 2), n(z, 3), 0, z)`.

### Step 35 — `std.math.positional.sub-column`

> 3 - 2 - borrow 0 is 1, borrow 0: 1 goes in front of the digits so far

Premises: `std.math.decimal-arith.sub-run(std.math.decimal-arith.difference(n(z, 3), n(z, 2)), n(z, 3), n(z, 2), 0, z)`, `std.math.digits.fs(3, 2, 0, 1, 0)`

Derives `std.math.decimal-arith.sub-run(std.math.decimal-arith.difference(n(z, 3), n(z, 2)), z, z, 0, std.math.decimal-arith.b(1, z))`.

### Step 36 — `std.math.positional.sub-column`

> 2 - 3 - borrow 0 is 9, borrow 1: 9 goes in front of the digits so far

Premises: `std.math.decimal-arith.sub-run(std.math.decimal-arith.shortfall(n(z, 3), n(z, 2)), n(z, 2), n(z, 3), 0, z)`, `std.math.digits.fs(2, 3, 0, 9, 1)`

Derives `std.math.decimal-arith.sub-run(std.math.decimal-arith.shortfall(n(z, 3), n(z, 2)), z, z, 1, std.math.decimal-arith.b(9, z))`.

### Step 37 — `std.math.positional.sub-end`

> no column is left and no borrow: the digits are (std.math.decimal-arith.b 1 z)

Premises: `std.math.decimal-arith.sub-run(std.math.decimal-arith.difference(n(z, 3), n(z, 2)), z, z, 0, std.math.decimal-arith.b(1, z))`

Derives `std.math.decimal-arith.strip(std.math.decimal-arith.difference(n(z, 3), n(z, 2)), std.math.decimal-arith.b(1, z))`.

### Step 38 — `std.math.positional.sub-borrow-out`

> no column is left and a borrow is: the number taken from is the smaller

Premises: `std.math.decimal-arith.sub-run(std.math.decimal-arith.shortfall(n(z, 3), n(z, 2)), z, z, 1, std.math.decimal-arith.b(9, z))`

Derives `std.math.decimal-arith.short(std.math.decimal-arith.shortfall(n(z, 3), n(z, 2)))`.

### Step 39 — `std.math.positional.strip-top`

> the top digit 1 is not zero: no high zero is left

Premises: `std.math.decimal-arith.strip(std.math.decimal-arith.difference(n(z, 3), n(z, 2)), std.math.decimal-arith.b(1, z))`, `std.math.digits.nz(1)`

Derives `std.math.decimal-arith.turn(std.math.decimal-arith.difference(n(z, 3), n(z, 2)), std.math.decimal-arith.b(1, z), z)`.

### Step 40 — `std.math.positional.turn-step`

> the digit 1 goes after z

Premises: `std.math.decimal-arith.turn(std.math.decimal-arith.difference(n(z, 3), n(z, 2)), std.math.decimal-arith.b(1, z), z)`

Derives `std.math.decimal-arith.turn(std.math.decimal-arith.difference(n(z, 3), n(z, 2)), z, n(z, 1))`.

### Step 41 — `std.math.positional.turn-end`

> turned round: the answer is (n z 1), a number as it is made

Premises: `std.math.decimal-arith.turn(std.math.decimal-arith.difference(n(z, 3), n(z, 2)), z, n(z, 1))`

Derives `std.math.decimal-arith.done(std.math.decimal-arith.difference(n(z, 3), n(z, 2)), n(z, 1))`; `std.math.decimal.valid(n(z, 1))`.

### Step 42 — `std.math.positional.sub-answer`

> (n z 3) - (n z 2) = (n z 1), and nothing is short

Premises: `std.math.decimal-arith.done(std.math.decimal-arith.difference(n(z, 3), n(z, 2)), n(z, 1))`

Derives `diff-of(n(z, 3), n(z, 2), n(z, 1), z)`.

### Step 43 — `giving-is-the-difference`

> (n z 3) less (n z 2) is (n z 1), so Bob has (n z 1) left

Premises: `gave(Bob, n(z, 2), Apple, Alice)`, `has(Bob, n(z, 3), Apple)`, `diff-of(n(z, 3), n(z, 2), n(z, 1), z)` — from Bob gave 2 apples to Alice.

Derives `has(Bob, n(z, 1), Apple)`.

### Step 44 — `giving-asks-the-difference`

> Bob had (n z 1) and gave (n z 2): ask (n z 1) less (n z 2)

Premises: `gave(Bob, n(z, 2), Apple, Alice)`, `has(Bob, n(z, 1), Apple)` — from Bob gave 2 apples to Alice.

Derives `sub(n(z, 1), n(z, 2))`.

### Step 45 — `std.math.positional.sub-ask`

> (n z 1) - (n z 2) is asked: first, the number (n z 1) stands for

Premises: `sub(n(z, 1), n(z, 2))`

Derives `std.math.decimal-arith.operand(std.math.decimal-arith.sub-first(n(z, 1), n(z, 2)), n(z, 1))`.

### Step 46 — `std.math.positional.operand-number`

> a number stands for itself

Premises: `std.math.decimal-arith.operand(std.math.decimal-arith.sub-first(n(z, 1), n(z, 2)), n(z, 1))`

Derives `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-first(n(z, 1), n(z, 2)), n(z, 1))`.

### Step 47 — `std.math.positional.operand-check`

> the operand found for (std.math.decimal-arith.sub-first (n z 1) (n z 2)) is (n z 1): it is checked, once, as a number

Premises: `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-first(n(z, 1), n(z, 2)), n(z, 1))`

Derives `std.math.decimal.check(n(z, 1), n(z, 1))`.

### Step 48 — `std.math.positional.sub-ask-second`

> the first operand is (n z 1): now the number (n z 2) stands for

Premises: `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-first(n(z, 1), n(z, 2)), n(z, 1))`

Derives `std.math.decimal-arith.operand(std.math.decimal-arith.sub-second(n(z, 1), n(z, 2), n(z, 1)), n(z, 2))`.

### Step 49 — `std.math.positional.numeral-step`

> the digit 1 is one: z is still to check

Premises: `std.math.decimal.check(n(z, 1), n(z, 1))`, `std.math.digits.nz(1)`

Derives `std.math.decimal.check(n(z, 1), z)`.

### Step 50 — `std.math.positional.operand-number`

> a number stands for itself

Premises: `std.math.decimal-arith.operand(std.math.decimal-arith.sub-second(n(z, 1), n(z, 2), n(z, 1)), n(z, 2))`

Derives `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-second(n(z, 1), n(z, 2), n(z, 1)), n(z, 2))`.

### Step 51 — `std.math.positional.numeral-valid`

> every place of (n z 1) holds a digit, and it ends in z: it is a number

Premises: `std.math.decimal.check(n(z, 1), z)`

Derives `std.math.decimal.valid(n(z, 1))`.

### Step 52 — `std.math.positional.operand-check`

> the operand found for (std.math.decimal-arith.sub-second (n z 1) (n z 2) (n z 1)) is (n z 2): it is checked, once, as a number

Premises: `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-second(n(z, 1), n(z, 2), n(z, 1)), n(z, 2))`

Derives `std.math.decimal.check(n(z, 2), n(z, 2))`.

### Step 53 — `std.math.positional.sub-start`

> take (n z 2) from (n z 1), and (n z 1) from (n z 2), from the units, with no borrow in: one of the two has an answer

Premises: `std.math.decimal-arith.operand-is(std.math.decimal-arith.sub-second(n(z, 1), n(z, 2), n(z, 1)), n(z, 2))`

Derives `std.math.decimal-arith.sub-run(std.math.decimal-arith.difference(n(z, 1), n(z, 2)), n(z, 1), n(z, 2), 0, z)`; `std.math.decimal-arith.sub-run(std.math.decimal-arith.shortfall(n(z, 1), n(z, 2)), n(z, 2), n(z, 1), 0, z)`.

### Step 54 — `std.math.positional.sub-column`

> 1 - 2 - borrow 0 is 9, borrow 1: 9 goes in front of the digits so far

Premises: `std.math.decimal-arith.sub-run(std.math.decimal-arith.difference(n(z, 1), n(z, 2)), n(z, 1), n(z, 2), 0, z)`, `std.math.digits.fs(1, 2, 0, 9, 1)`

Derives `std.math.decimal-arith.sub-run(std.math.decimal-arith.difference(n(z, 1), n(z, 2)), z, z, 1, std.math.decimal-arith.b(9, z))`.

### Step 55 — `std.math.positional.sub-column`

> 2 - 1 - borrow 0 is 1, borrow 0: 1 goes in front of the digits so far

Premises: `std.math.decimal-arith.sub-run(std.math.decimal-arith.shortfall(n(z, 1), n(z, 2)), n(z, 2), n(z, 1), 0, z)`, `std.math.digits.fs(2, 1, 0, 1, 0)`

Derives `std.math.decimal-arith.sub-run(std.math.decimal-arith.shortfall(n(z, 1), n(z, 2)), z, z, 0, std.math.decimal-arith.b(1, z))`.

### Step 56 — `std.math.positional.sub-borrow-out`

> no column is left and a borrow is: the number taken from is the smaller

Premises: `std.math.decimal-arith.sub-run(std.math.decimal-arith.difference(n(z, 1), n(z, 2)), z, z, 1, std.math.decimal-arith.b(9, z))`

Derives `std.math.decimal-arith.short(std.math.decimal-arith.difference(n(z, 1), n(z, 2)))`.

### Step 57 — `std.math.positional.sub-end`

> no column is left and no borrow: the digits are (std.math.decimal-arith.b 1 z)

Premises: `std.math.decimal-arith.sub-run(std.math.decimal-arith.shortfall(n(z, 1), n(z, 2)), z, z, 0, std.math.decimal-arith.b(1, z))`

Derives `std.math.decimal-arith.strip(std.math.decimal-arith.shortfall(n(z, 1), n(z, 2)), std.math.decimal-arith.b(1, z))`.

### Step 58 — `std.math.positional.strip-top`

> the top digit 1 is not zero: no high zero is left

Premises: `std.math.decimal-arith.strip(std.math.decimal-arith.shortfall(n(z, 1), n(z, 2)), std.math.decimal-arith.b(1, z))`, `std.math.digits.nz(1)`

Derives `std.math.decimal-arith.turn(std.math.decimal-arith.shortfall(n(z, 1), n(z, 2)), std.math.decimal-arith.b(1, z), z)`.

### Step 59 — `std.math.positional.turn-step`

> the digit 1 goes after z

Premises: `std.math.decimal-arith.turn(std.math.decimal-arith.shortfall(n(z, 1), n(z, 2)), std.math.decimal-arith.b(1, z), z)`

Derives `std.math.decimal-arith.turn(std.math.decimal-arith.shortfall(n(z, 1), n(z, 2)), z, n(z, 1))`.

### Step 60 — `std.math.positional.turn-end`

> turned round: the answer is (n z 1), a number as it is made

Premises: `std.math.decimal-arith.turn(std.math.decimal-arith.shortfall(n(z, 1), n(z, 2)), z, n(z, 1))`

Derives `std.math.decimal-arith.done(std.math.decimal-arith.shortfall(n(z, 1), n(z, 2)), n(z, 1))`; `std.math.decimal.valid(n(z, 1))`.

### Step 61 — `std.math.positional.sub-answer-short`

> (n z 1) is short of (n z 2) by (n z 1): the difference is zero

Premises: `std.math.decimal-arith.done(std.math.decimal-arith.shortfall(n(z, 1), n(z, 2)), n(z, 1))`

Derives `diff-of(n(z, 1), n(z, 2), z, n(z, 1))`.

## State 1 of 1 — ∅, root itself — 0 steps

_Nothing beyond root and the commitment._
