# Q-M1f.224 — A run takes N only where it prints N or its count

N, the minimal dead sets, is searched for after the route, as the minimal transversals of the maximal alive states'
complements, and its count is how many there are. A bare `ein solve` on the state route prints the `N` row with its
count and pays for that search. A run whose surfaces print neither N nor its count, `ein solve --count` and `ein test`
without an artefact, takes none, and prints and claims the same as if it had; where it writes an artefact that
carries the count, `--json-summary`, `--events` or `--dump-answer`, it takes N. The tree and the classes compute no
N, and the row says so.

**Why.** Each run does the work its question needs: a count of R's states needs no N, and a row that prints N needs
all of it.

**Held by.** `ein-cli/tests/test_cli.rs`: `a_run_that_prints_no_n_takes_none_and_says_the_same`;
`ein_infer::transversal`, the search; the `N` row: [08-read-out.md](../../ein/user/04-reference/08-read-out.md#2-describe-a-bare-ein-solve---count-and-ein-oracle) § 2.
