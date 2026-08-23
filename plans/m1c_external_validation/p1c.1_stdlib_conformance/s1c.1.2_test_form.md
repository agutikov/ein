# S1c.1.2 — How a program states what it expects

**Phase:** P1c.1 (stdlib conformance)
**Estimate:** 3 days
**Depends on:** [S1c.1.1](s1c.1.1_what_the_stdlib_promises.md)
**Decides:** [Q-M1c.1](../open_questions.md#q-m1c1--how-does-a-program-state-what-it-expects),
[Q-M1c.2](../open_questions.md#q-m1c2--what-may-an-expectation-say)

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
