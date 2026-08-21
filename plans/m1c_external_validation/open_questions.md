# Open Questions — M1c (External validation)

Milestone-scoped questions. Ids are **sticky** — `Q-M1c.<n>`, in the style
[M1a](../m1a_rust/open_questions.md) uses for `Q-M1a.<n>` rather than the
global `Q<n>` sequence in [`plans/open_questions.md`](../open_questions.md),
so the namespaces cannot collide. A closed id is never reused.

**Q-M1c.1 and Q-M1c.2 arrived with [P1c.1](p1c.1_stdlib_conformance/README.md)
on 2026-08-21**, where they were Q-M1a.19 and Q-M1a.20. The text below is
theirs, unchanged apart from ids and paths; the M1a entries stay in place as
redirects, because a sticky id that silently disappears is worse than one that
points somewhere.

## Index

| Q | title | status |
|---|---|---|
| [Q-M1c.1](#q-m1c1--how-does-a-program-state-what-it-expects) | How does a program state what it expects? | open — recommendation: **`:expect` on `query`, several queries per file**; [S1c.1.2](p1c.1_stdlib_conformance/s1c.1.2_test_form.md) decides *(was Q-M1a.19)* |
| [Q-M1c.2](#q-m1c2--what-may-an-expectation-say) | What may a `(test …)` expectation say? | open — recommendation: four keys, each demanded by a rule *(was Q-M1a.20)* |
| [Q-M1c.3](#q-m1c3--what-makes-an-encoding-fair) | What makes a benchmark encoding fair? | open — recommendation: published where one exists, provenance where it does not, and no tuning against the clock |
| [Q-M1c.4](#q-m1c4--does-a-proof-assistant-belong-in-a-timing-table) | Does a proof assistant belong in a timing table? | open — recommendation: **(b) keep Lean, drop its time column** |
| [Q-M1c.5](#q-m1c5--where-does-the-benchmark-live-and-is-any-of-it-a-gate) | Where does the benchmark live, and is any of it a gate? | open — recommendation: a crate, and only the answer half runs unattended |

---

## Q-M1c.1 — How does a program state what it expects?

[P1c.1](p1c.1_stdlib_conformance/README.md) needs somewhere to say what a
stdlib rule should derive. Three shapes; the third is the user's, proposed
2026-08-20 after the first two, and it is the recommendation.

| | (a) sidecar | (b) `(test …)` head | (c) `:expect` on `query` |
|---|---|---|---|
| grammar cost | none | new head, SYMBOL exclusion, every AST walker | **one keyword** |
| travels with the program | no | yes | yes |
| several checks per file | yes | needs a rule | **yes — one per query** |
| the expectation's shape | assertions | assertions | **the engine's own output** |
| verdict / `k` | separate keys | separate keys | **implied by the shape** |
| exactness | per fact | per fact | **relation-closed** |
| route (`:fires R`) | expressible | expressible | not expressible |
| loader change | none | a new form | **`query` becomes plural** |

**Recommendation: (c).** Not because it is cheapest — though it is — but
because of the fourth and sixth rows. An expectation shaped like a *model* is
written by running the program and reviewing the answer, and read as an
answer; and **relation-closure** ("naming a relation asserts its complete
extent, and says nothing about relations it does not name") sits exactly
between the two useless extremes. A per-fact assertion cannot catch a
*surplus* fact; a whole-state golden pins 250 facts of `is-a*` and activator
noise no test means to assert.

The concrete argument is this morning's bug: the 23 spurious models of
`zebra2-minus-15` were surplus — Chesterfields and the Fox in one house. A
per-fact `:derives` passes on every one. An `:expect` naming `smoke-loc` and
`pet-loc` fails on all 23.

**The cost is a loader change with a trap in it.** Today the last `query`
silently wins (`from_ir.rs`, "Last one wins, for both blocks", pinned in both
engines by `the_last_query_and_the_last_config_win`). A *test* file whose
second check is silently discarded is worse than no test file, so
`Program.query` becomes plural and every consumer says what it does with N.
`config`'s last-wins stays: a config is a setting, a query is content, and the
two want opposite rules.

Decided in [S1c.1.2](p1c.1_stdlib_conformance/s1c.1.2_test_form.md).

## Q-M1c.2 — What may an expectation say?

Under (c) the question narrows sharply, because the *shape* is fixed — an
expectation is a solution — and what is left is three semantic rules and one
residue.

The rules, as the user specified them: an expectation is **at least** the
relations named by the query's `:goal`; it **may** carry further facts for
verification; and **naming a relation closes it** — the listed facts are that
relation's complete extent in the model. `(or S1 S2 …)` is the ambiguous case
and compares model *sets*, with `k` implied by the count.

Two sub-questions the stage has to settle:

- **Do stored negatives count?** Does closing `pet-loc` also assert the
  extent of `(not (pet-loc …))`? **Recommendation: no** — the positive extent
  only, with a `(not …)` listable as an ordinary fact when a test means to pin
  one. Otherwise every expectation drags in the negative-completion rules'
  whole output, which is most of a model.
- **How is `Contradiction` spelled?** Not `:expect ()` if that reads as "the
  empty model". `:expect none` is clearer.

**The residue is route.** `:fires R` / `:does-not-fire R` — "the right fact by
the wrong rule" — has no home in a vocabulary of facts, and it matters for the
stdlib: `domain-elimination` and `range-elimination` can derive the same
positive from opposite directions. **Recommendation: leave it out of the first
cut** and let [S1c.1.1](p1c.1_stdlib_conformance/s1c.1.1_what_the_stdlib_promises.md)'s
table say whether a rule needs it. Under (c) it arrives as one more keyword on
the same query, which is the other way (c) beats (b): the vocabulary grows a
key at a time instead of arriving whole.

## Q-M1c.3 — What makes an encoding fair?

The benchmark's entire validity rests on this and nothing else. Whoever writes
six encodings of the same puzzle knows one of the six systems far better than
the other five, and a clumsy CLP(FD) program is indistinguishable in the table
from a slow Prolog.

- **(a) Published encodings only.** Cite it or drop the cell. Maximum
  credibility, and it fails immediately: there is no published ein-lang
  n-queens, and there never will be.
- **(b) Idiomatic-per-system, written here, with provenance.** Each file
  records who wrote it, from what, and what was changed. Honest, and it puts
  the reader in a position to discount it.
- **(c) An encoding budget.** The same wall-clock effort per system, recorded.
  Sounds fair, measures the author's fluency, and cannot be audited by a
  reader.

**Recommendation: (a) where a published encoding exists, (b) with provenance
where it does not**, plus [S1c.2.1](p1c.2_external_benchmarks/s1c.2.1_problem_corpus.md)'s
rule 3 — the first working idiomatic version is the one that is timed, and a
later faster one is added rather than substituted.

**The residue is n-queens in ein-lang**, which has no published prior art by
construction and whose `attacks` relation is generated because the kernel has
no arithmetic. The line the stage draws — *the generator may compute the
board, never the solution* — is the part a reviewer should attack first, and
it is written down so that they can.

## Q-M1c.4 — Does a proof assistant belong in a timing table?

Lean 4 is in the user's list, and it is not a solver. `decide` /
`native_decide` over a finite domain is brute force through kernel reduction;
a hand-written proof measures the author's afternoon. Either number next to
Z3's is a category error.

- **(a) Drop Lean.** Clean, and loses the one column where something is said
  that no solver can say.
- **(b) Keep Lean, no time column.** Its cell reports the *artefact*: what had
  to be stated, and what was proved.
- **(c) Keep Lean with a time column and a warning.** Warnings do not survive
  being quoted.

**Recommendation: (b).** And the reason is not diplomacy — a Lean development
can prove that the model is the *only* model, which is exactly the guarantee
the word `exhausted` claims in Ein's own verdict
([Q-M1d.1](../m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)).
Having one column in the corpus where that guarantee is machine-checked is
worth more than a number, and it is the corpus's only link to
[M1d](../m1d_satisfiability/README.md)'s subject.

## Q-M1c.5 — Where does the benchmark live, and is any of it a gate?

- **(a) `ein.rs/crates/ein-bench` + `bench/` for data**, mirroring
  `ein-conformance` + `conformance/`: a crate that shells out and links
  nothing.
- **(b) A `utils/` script**, like the M1a measurement set (`bench_env.sh`,
  `e2e_baseline.py`, `profile_ein_rs.py`).
- **(c) In-tree tests**, run by `cargo test --workspace`.

**Recommendation: (a) for the code, and (c) for the answer half only.** After
[P1a.10](../m1a_rust/p1a.10_single_implementation/README.md) `cargo test
--workspace` is the whole gate and a shell script is where a check goes to
die — but a per-commit gate that depends on six external programs fails for
reasons that have nothing to do with the commit. So: the harness is a crate;
the answer-parity subset runs in `nightly.yml` where the systems are
installable and reports `missing` where they are not; every number with a
clock in it is taken by a person on a quiet machine.

The sub-question the stage still owes: **do the `bench/` corpus files count as
corpus for the completeness check** that fails when an `.ein` file has no
entry? They are `.ein` files under a new directory, and the answer decides
whether adding a benchmark problem also means adding a conformance entry.
