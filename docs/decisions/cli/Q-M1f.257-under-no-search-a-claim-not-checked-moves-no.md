# Q-M1f.257 — Under `--no-search`, a claim not checked moves no exit code

`ein test --no-search` decides what root decides. A claim root does not decide is NOT CHECKED, says that
`--no-search` reads root alone, and moves no exit code: the run exits 0 unless a claim root decides is FAILED (1), or
a file is refused, aborted or states nothing to check (2). Without the flag, a claim not checked exits 1.

**Why.** Every answer exits 0, and only a refuted claim exits 1. A claim left undecided because the run was asked not
to search has not been refuted; it has not been asked.

**Held by.** `ein-cli/tests/no_search.rs`: `test_decides_what_root_decides_and_a_claim_not_checked_exits_0`. The
codes: [defined_behaviour.md](../../kernel/defined_behaviour.md) § 5. Which commands take the flag:
[Q-M1f.258](Q-M1f.258-ein-solve-and-ein-test-take-no-search-and-ein.md).
