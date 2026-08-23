# Open Questions — M1c (External validation)

Milestone-scoped questions. Ids are **sticky** — `Q-M1c.<n>`, in the style
[M1a](../../docs/history/m1a_rust/open_questions.md) uses for `Q-M1a.<n>` rather than the
global `Q<n>` sequence in [`plans/open_questions.md`](../open_questions.md),
so the namespaces cannot collide. A closed id is never reused.

**Q-M1c.1 and Q-M1c.2 arrived with [P1c.1](p1c.1_stdlib_conformance/README.md)
on 2026-08-21**, where they were Q-M1a.19 and Q-M1a.20. The text below is
theirs, unchanged apart from ids and paths; the M1a entries stay in place as
redirects, because a sticky id that silently disappears is worse than one that
points somewhere. **Q-M1c.3–5 left the same way on 2026-08-23**, with P1c.2
becoming [M10](../m10_external_benchmarks/README.md): their text lives there
as Q-M10.1–3 and the rows below say so.

## Index

| Q | title | status |
|---|---|---|
| [Q-M1c.1](#q-m1c1--how-does-a-program-state-what-it-expects) | How does a program state what it expects? | open — recommendation: **`:expect` on `query`, several queries per file**; [S1c.1.2](p1c.1_stdlib_conformance/s1c.1.2_test_form.md) decides *(was Q-M1a.19)* |
| [Q-M1c.2](#q-m1c2--what-may-an-expectation-say) | What may a `(test …)` expectation say? | open — recommendation: four keys, each demanded by a rule *(was Q-M1a.20)* |
| ~~Q-M1c.3~~ | What makes a benchmark encoding fair? | **moved 2026-08-23 with P1c.2 → [Q-M10.1](../m10_external_benchmarks/open_questions.md#q-m101--what-makes-an-encoding-fair)** |
| ~~Q-M1c.4~~ | Does a proof assistant belong in a timing table? | **moved 2026-08-23 with P1c.2 → [Q-M10.2](../m10_external_benchmarks/open_questions.md#q-m102--does-a-proof-assistant-belong-in-a-timing-table)** |
| ~~Q-M1c.5~~ | Where does the benchmark live, and is any of it a gate? | **moved 2026-08-23 with P1c.2 → [Q-M10.3](../m10_external_benchmarks/open_questions.md#q-m103--where-does-the-benchmark-live-and-is-any-of-it-a-gate)** |

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
