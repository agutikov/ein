# Q-M1e2.35 — The default region budget is 2^20 facts

A run that searches stops past 2²⁰ facts beyond root (1 048 576), in every mode, unless `--region-budget FACTS` says
otherwise. A run the budget stops reports what it established as a part of R, says it was stopped and where, writes
one `note:` and exits 0 ([Q-M1e2.33](Q-M1e2.33-every-answer-exits-0.md)); a claim it cannot decide is not checked.

**Why.** It is large enough to certify `examples/zebra.ein`'s `--single`, `--maximal` and `--minimal`, and small
enough that a run over every corpus program fits in memory; no test runs past it. A budget only stops work, so its value changes how
much of R a run discovers and never which R
([Q-M1e2.30](../answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)).

**Held by.** `ein_infer::downset::DEFAULT_BUDGET`, the one constant the option's default and its help text read
(`ein-cli`'s `cmdline.rs`); `docs/kernel/configuration.md`, the budgets; `docs/ein/user/reference/read-out-examples.md`,
which the binary writes and `ein-cli/tests/read_out_reference.rs` holds, a describe stopped at this default.
