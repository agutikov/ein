# Q-M1f.590 — A hypothesis rule is matched as it is reached

The hypothesis rules are matched against root's fixpoint one after another, each as it is reached, and nothing is
written between two of them: a hypothesis rule whose `:match` reads `(not …)` sees the stored negations root holds,
and none another rule's candidate would bring. No look-ahead reads what a candidate of one rule would add before
another rule is matched.

**Why.** A candidate is supposed by a commitment and never written into root, so what one rule proposes is no fact
another rule can read at root; a look-ahead over a premise that reads `not` read a negation by its name, which the
kernel does not ([Q-M1f.436](../engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md)), and matching each rule as it
is reached answers alike.

**Held by.** `ein_infer::hrule::Hrules::candidates`; `ein-cli/tests/hrule_reads_not.rs`; the corpus sweeps, which
hold every program's H and R.
