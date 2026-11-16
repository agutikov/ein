# Q-M1f.221 — `ein test` asks a file the least question that decides its claims

`ein test` reads the shapes of the claims a file states and asks the file one question, the least that can decide
them all: where the queries' claims are all over one part of R and the most classes one lists is k, that part is
asked for k + 1 states, `--single` where k is at most one; a `(solve … :expect …)`, or claims over more than one
part, ask describe at once. What that question leaves undecided is asked the describing question, of the program
loaded again: two runs a file at most. No option chooses the question, and a query with no `:expect` is not solved.

**Why.** Each mode does the least work its question needs, and a file's claims state the question. One run that
decides every claim is cheaper than a run a claim, and the fall-back leaves no claim unchecked that describe
decides.

**Held by.** `ein-infer/tests/claim_routes.rs`: `the_least_question_is_read_off_the_claims_shapes` and
`every_claim_is_decided_as_the_oracle_decides_it`; `ein-cli/tests/test_cli.rs`:
`a_claim_is_asked_the_least_question_and_the_surfaces_say_which` and
`an_undecided_claim_falls_back_to_the_description`.
