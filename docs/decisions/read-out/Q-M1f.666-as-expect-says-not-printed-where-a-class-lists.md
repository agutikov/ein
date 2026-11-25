# Q-M1f.666 — `as :expect` says *not printed* where a class lists an `open` fact

Where a query's class lists an `open` fact, one at whose head the kernel reads `open`, or the absence of one,
`(absent (open …))`, the read-out's `as :expect` row prints no claim. It says *not printed: these classes list an
open fact, which no claim lists*, the first of the reasons the row gives. Under an ordinary relation `open` is a
name, `(think Ann (open (q a)))`, and that class's claim is printed, and pasted into the query's `:expect` it holds.

**Why.** A claim lists facts of the program's relations, and `open` at the head of one a claim writes is refused at
the parse ([Q-M1f.88](../diagnostics/Q-M1f.88-a-misplaced-open-is-a-parse-error-with-a-note.md)): a claim printed
for such a class is one the loader refuses. The row says why it prints none, as it does of a class that is a lower
bound or over the states listed.

**Held by.** `ein_infer::read_out`'s `names_open`, which marks a query's classes, and `ein_render::read_out`, which
prints the row; `ein-cli/tests/read_out.rs`: `a_claim_no_query_can_state_is_not_printed`, under describe and each
mode, the printed claim read back by `ein test`. The page is
[`08-read-out.md` § 7](../../ein/user/04-reference/08-read-out.md#7-queries-and-claims).
