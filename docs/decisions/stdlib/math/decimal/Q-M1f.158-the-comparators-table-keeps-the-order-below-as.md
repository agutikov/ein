# Q-M1f.158 — The comparator's table keeps the order below as its first column

A row of `cl` is `(cl BELOW A B ORDER)`: the order of the columns already passed, then two digits, then the order
they make.

**Why.** The matcher keys a premise on its first bound argument, so a comparison's column offers it more candidates
than a sum's; the cost is a constant a column, and a batch still costs in proportion to its count.

**Held by.** `stdlib/math/digits.ein`; `ein-ir/tests/digit_tables.rs`: `every_row_agrees_with_the_integers`.
