# S1c.1.2 — How a program states what it expects

**Phase:** P1c.1 (stdlib conformance)
**Estimate:** 3 days
**Depends on:** [S1c.1.1](s1c.1.1_what_the_stdlib_promises.md)
**Decides:** [Q-M1c.1](../open_questions.md#q-m1c1--how-does-a-program-state-what-it-expects),
[Q-M1c.2](../open_questions.md#q-m1c2--what-may-an-expectation-say)

**Status: shipped 2026-08-23.** (c) as recommended, with one change the
recommendation could not have known about and two costs it did not price.

| finding | number |
|---|---|
| the form | **`:expect (model …)` / `(or (model …) …)` / `(false)`** — one keyword, as (c) promised |
| why `(model …)` and not the proposed bare list | `ListHead ::= SYMBOL \| VAR \| WILDCARD \| EQ` — **a list head does not parse**, and widening it would change the grammar of every form |
| grammar productions added | **0** — the shape is loader-checked, not parsed |
| call sites touched by `Program.query` → `queries` | **14** |
| corpus files that relied on the last-query-wins discard | **0 of 128** — the trap was real and had never been sprung |
| goldens that moved | **0** — 188 golden lines added, all of them cells for the new fixtures, none changed |
| load-time refusals added | **5**, each with a `broken/load/` fixture: unknown keyword, malformed shape, unknown relation, omits the goal, not ground |
| …and they are the **first ein.rs-only diagnostics** | which decided a question [`defined_behaviour.md` §4](../../../docs/kernel/defined_behaviour.md) left open: a message with no Python counterpart names **no exception class** |
| the cost nobody predicted | an artefact flag names **one** path, so `--events` / `--trace` / `--json-summary` / `--dump-states` are refused (exit 2) on a file that asks more than one question |
| tests added | **31** — 8 shape, 7 loader, 12 comparison, 7 CLI (minus the two grammar-level cases that never reach the loader) |
| opened | [Q-M1c.6](../open_questions.md#q-m1c6--how-does-an-expectation-say-a-relation-is-empty) — closure cannot say a relation is *empty*, and rule 1 makes that reachable |
| …and handed on | [P1d.4](../../m1d_satisfiability/p1d.4_model_set_closure/README.md) / [Q-M1d.7](../../m1d_satisfiability/open_questions.md#q-m1d7--may-a-program-require-its-own-model-count) — `(or …)` states *these are all the models*, which no `:match` can require and only an exhausted search can verify |

**Corrected 2026-08-24, the day after it shipped.** Two things, both found by
the user reading the form back:

- **⊥ is `(false)`, not `none`.** `false` is already ein's contradiction — one
  of the five `STRUCTURAL` names, and what every refutation rule in the stdlib
  asserts. Shipping an invented word for it was the mistake, and the
  recommendation that suggested `none` had been right only about rejecting
  `()`.
- **A non-exhausted search cannot confirm a verdict**, and the checker was
  calling it a pass. `:expect` names a `k`; a stopped run establishes a lower
  bound on `k`; the two happening to agree is not evidence. There is now a
  third outcome — `Outcome::NotChecked`, exit 1, `NOT CHECKED` on the line —
  and it bites only where more searching could have changed the answer: finding
  *more* models than claimed, or a model that disagrees with the expectation it
  matched, stays a plain failure. Three fixtures now carry the three verdicts,
  and `11_expect_ambiguity.ein` declares `solve -e` and no plain `solve`
  because a k>1 claim is not this corpus's to check at `-n 1`.

**Where the spec of record went.** This stage doc and the phase README both say
the form must land in `grammar.lark`. That file left with `ein.py` at M1a
S1a.10.5 and its successor is
[`docs/kernel/ir/03-ein-lang/00_ebnf.md`](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md),
which is what carries it — and it carries it as **§4, what the grammar
deliberately does not enforce**, because `KwPair ::= KEYWORD Value` already
admits every shape `:expect` uses. The cross-milestone edit M2's GBNF lift
reads is therefore smaller than the stage expected: no new production, one
paragraph about what the loader checks.

## Context

Three shapes are on the table. The third is the user's, added 2026-08-20 after
the first two were written, and it is the recommendation.

### (a) A sidecar

Expectations in `corpus.toml`, or a `.expect` file beside the `.ein`. No
grammar change at all. Rejected on one argument: an expectation keyed by path
is a second thing to keep in step with the program, and this milestone has
just spent a phase ([P1a.10](../../../docs/history/m1a_rust/README.md#p1a10--one-implementation))
removing the last one.

### (b) A `(test …)` head replacing `(query …)`

A file is either a puzzle to solve or a program to check. Costs a parser case,
a dumper case, a macro pass, every AST walker, a `grammar.lark` change and a
SYMBOL exclusion for `test` — and needs its own vocabulary of assertion keys
(`:derives`, `:absent`, `:verdict`, `:k`) because the form has no other shape
to borrow.

### (c) `:expect` on `query`, and several queries per file — **recommended**

Keep `query`. Add one keyword:

```lisp
(query
  :goal   (pet-loc Zebra ?h)
  :expect ((pet-loc  Zebra House-5) (pet-loc Fox House-1) …
           (nation-loc Japanese House-5) …))
```

and allow **several `query` heads in one file**, each an independent check
over the same ontology and rules.

Three things make this better than (b) rather than merely cheaper:

1. **The expectation is the engine's own output shape.** A model. Writing a
   test is "run it, read the answer, review it"; reading a test is reading a
   model rather than parsing a list of assertions.
2. **The verdict is implied, not asserted.** One solution means `Solution`;
   `(or S1 S2 …)` means `Ambiguity` with `k` equal to the number of disjuncts;
   the empty expectation means `Contradiction`. Four keys collapse into one,
   and a whole class of test — "says `:verdict Solution` but lists two models"
   — becomes unwriteable.
3. **Several checks per file.** The stdlib's rules are small and share
   ontologies; one program with four queries beats four programs with one.
   `:hypothesis-relations` is already per-query, so two queries over one KB can
   be genuinely different searches — which is a *testing* capability, not an
   accident.

## The semantics of `:expect`, as the user specified it

> a solution is in turn **at least** the relations from the query's `:goal`,
> and may contain additional facts for verification, but requires checking
> **all** relations in the final solution

Read precisely, that is three rules, and the third is the one that gives the
form its teeth:

1. **The goal's relations are mandatory.** An expectation that does not pin
   what the query asked is not an expectation.
2. **More is allowed.** Any fact may be listed, so a test can pin the
   consequence at a distance that
   [S1c.1.4](s1c.1.4_stdlib_corpus.md) says is the only kind worth writing.
3. **Naming a relation closes it.** If `:expect` mentions `pet-loc` at all,
   the listed `pet-loc` facts are the model's *complete* `pet-loc` extent —
   not a subset. Relations the expectation never mentions are unconstrained.

Rule 3 is the design. It sits exactly between the two useless extremes: a
per-fact assertion cannot catch a *surplus* fact, and a whole-state golden
pins 250 facts of `is-a*` and activator noise that no test means to assert.
Relation-closure is exact on what the test is about and silent on the rest.

**And it would have caught this morning's bug.** The 23 spurious models of
`zebra2-minus-15` were surplus: they placed Chesterfields and the Fox in one
house. A per-fact `:derives` would have passed on every one of them. An
`:expect` naming `smoke-loc` and `pet-loc` fails on all 23.

## What (c) does not cover

**Route.** `:fires R` / `:does-not-fire R` — "the right fact by the wrong
rule" — has no home in a form whose vocabulary is facts. For the stdlib that
matters: `domain-elimination` and `range-elimination` can derive the same
positive from opposite directions, and a test that cannot tell them apart is
not testing either.

**Recommendation: leave it out of the first cut**, and let
[S1c.1.1](s1c.1.1_what_the_stdlib_promises.md)'s table say whether any rule
actually needs it. If one does, `:fires` is a second keyword on the same query
and costs nothing structurally — which is another way (c) beats (b): its
vocabulary can grow one key at a time instead of arriving whole.

## The loader change is real, and it has a trap

**Today the last `query` silently wins.** `from_ir.rs`'s "Last one wins, for
both blocks", pinned in both engines by a test named
`the_last_query_and_the_last_config_win`. So a file with two queries loads,
runs one, and says nothing — which is precisely the failure mode a *test* file
must not have. A test program whose second check is silently discarded is
worse than no test program.

So `Program.query` becomes plural, and every consumer of it has to say what it
does with N: `solve` (one query is the run; N is either an error or N runs),
`render`, `--json-summary`, the trace, and `shape.rs`'s parity views. The
existing "last wins" behaviour for `config` is untouched and should stay —
config is a *setting*, queries are *content*, and the two want opposite rules.

## Acceptance

- `:expect` parses, dumps and round-trips like every other query keyword, and
  `grammar.lark` — the spec of record — carries it. M2's GBNF lift reads that
  file, so this is a **cross-milestone edit**, reviewed as one.
- **Several `query` heads load, and all of them are reachable.** The silent
  last-wins discard is gone, and a file that relied on it is either an error or
  is enumerated. The corpus says how many such files exist.
- Rule 3 (relation-closure) is implemented and **tested in the failing
  direction**: a model with a surplus fact in a named relation fails, and the
  message says which fact was unexpected.
- `(or …)` compares model **sets**, not sequences — the order the search
  happens to find models in is exactly what
  [S1a.7.0](../../../docs/history/m1a_rust/README.md#s1a70--the-speculation-audit)'s invariance
  tests assert is not observable.
- The `Contradiction` spelling is decided and is not `:expect ()` if that could
  be read as "expect the empty model". `:expect none` reads better; the stage
  picks one and documents it.
- A query carrying `:expect` under plain `solve` either checks it or errors —
  **never ignores it**. Same reasoning as the last-wins trap.
- The form is documented in `docs/kernel/ir/03-ein-lang/` next to `query`.

## Tasks

### Task T1c.1.2.1 — Settle Q-M1c.1 and Q-M1c.2 in writing
### Task T1c.1.2.2 — `Program.query` becomes plural

The trap above, and the widest-reaching part of the stage: every reader of
`program().query` in both the engine and the renderers.

### Task T1c.1.2.3 — Grammar, AST, dumper, round-trip
### Task T1c.1.2.4 — The comparison

Relation-closure, `(or …)` as set equality, and the `Contradiction` case.
Facts compare by content — rendered s-expressions — not by `FactId`, for
[`fork_audit`](../../../ein.rs/crates/ein-infer/src/fork_audit.rs)'s reason:
two runs do not share an interner.

### Task T1c.1.2.5 — Validation at load

An `:expect` naming an undeclared relation is a load error. An `:expect` that
omits the goal's relations is a load error (rule 1). A test that silently
checks nothing is the one outcome this phase cannot afford.

## Notes

- Most `std.*` rules fire during **saturation**, not search —
  `functional-negative`, `domain-elimination`, `typecheck-arg-*`, `symmetric`,
  `transitive`, `includes`. That is not an objection to a query-shaped
  expectation: with no hypotheses the search degenerates to "`alive` is empty,
  so root *is* the unique model", and the solution the expectation compares
  against is the saturated root. It does mean every stdlib test pays a solve,
  which for these programs is microseconds.
- Rule 3 has one ambiguity worth nailing down: does closing a relation include
  its stored negatives, `(not (R a b))`? **Recommendation: no** — the positive
  extent only, with `(not …)` listable as an ordinary fact when a test means
  to pin one. Otherwise every expectation drags in the negative-completion
  rules' entire output, which is most of a model.
