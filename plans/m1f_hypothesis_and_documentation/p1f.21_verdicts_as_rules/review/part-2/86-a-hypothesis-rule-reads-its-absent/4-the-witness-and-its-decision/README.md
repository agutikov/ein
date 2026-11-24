# 86 · 4 — the witness and its decision (defaults 4 and 5, Q-M1f.688 and Q-M1f.689)

[← item 86](../README.md) · [← part 1](../1-the-absent-is-read/README.md) · [← part 2](../2-a-read-under-absent-flags-nothing/README.md) ·
[← part 3](../3-the-oracles-listing/README.md)

Two choices of bookkeeping, made while the wrong answer was filed and fixed by the register's rules
([`plans/wrong_answers/README.md`](../../../../../../wrong_answers/README.md)). Neither moves an answer.

<img src="runs/concept.svg" alt="found by S1f.21.19's runs; filed as a row of the register in 1f14f270, with its witness and no expectation; fixed by 21115a70, which moves the row to corpus/regressions.toml, the witness into the corpus with its :expect and a test run (default 5), writes the decision file Q-M1f.685 as a default before your ruling (default 4), and adds the test that holds H to the rule that concludes a fact; your ruling keeps the decision or rewrites it">

*Drawn by hand (`../tools/concept-4.dot` → `runs/concept.svg`).*

## Default 4: the decision file is written before you rule (Q-M1f.688)

Default 1's reading, Q-M1f.685, is a default, not yet your ruling. **The default:** it got its decision file in the
fixing commit, [`docs/decisions/hypotheses/Q-M1f.685-…`](../../../../../../../docs/decisions/hypotheses/Q-M1f.685-a-rule-that-concludes-a-hypothesis-reads-its.md),
as Q-M1f.590, also a default, has one. The regression row's `decisions`, a code comment and two pages cite it, and
the decisions' README says a decision taken by default and one ruled are alike. **The other reading:** it stays in
`open_questions.md` until you rule, and the project cites Q-M1f.129 and Q-M1f.567 alone.

Who cites it today (`runs/rows.txt`): `corpus/corpus.toml`, `corpus/regressions.toml`, the decision file and the
hypotheses' decisions README, `docs/ein/user/03-syntax-and-semantics/09-hypotheses.md`,
`docs/ein/user/04-reference/02-syntax.md`, `ein-infer/src/hypothesis_rules.rs`,
`ein-infer/tests/hypothesis_rule_absent.rs`, and the witness.

## Default 5: what the moved witness states (Q-M1f.689)

While the row was open its witness stated **no `:expect`**, so the register's `solve` run exited 0 on the wrong
answer, which the row held as data. **The default:** moved into the corpus, it states the right answer, the row's,
and the corpus runs it under `test`:

```lisp
(solve
  :expect (region
            (model)
            (model (r b))))
```

`corpus/corpus.toml`'s entry runs it under `solve`, `saturate`, `test` and `saturate --dump`, and
`corpus/regressions.toml`'s case `hypothesis-rule-absent` holds the binary to `k_s = 2`, `minimal [[]]`,
`maximal [["(r b)"]]` (`runs/rows.txt`). On the engine before the fix the `:expect` fails, and since it holds
(`runs/witness.pre.test.txt`, `runs/witness.new.test.txt`; quoted in [part 1](../1-the-absent-is-read/README.md#the-corpus-witness)).
**The other reading:** no `:expect`, the regression row alone holding the answer.
