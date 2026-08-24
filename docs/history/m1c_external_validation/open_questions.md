# Open Questions — M1c (External validation)

Milestone-scoped questions. Ids are **sticky** — `Q-M1c.<n>`, in the style
[M1a](../m1a_rust/open_questions.md) uses for `Q-M1a.<n>` rather than the
global `Q<n>` sequence in [`plans/open_questions.md`](../../../plans/open_questions.md),
so the namespaces cannot collide. A closed id is never reused.

**Q-M1c.1 and Q-M1c.2 arrived with [P1c.1](README.md#p1c1--stdlib-conformance)
on 2026-08-21**, where they were Q-M1a.19 and Q-M1a.20. The text below is
theirs, unchanged apart from ids and paths; the M1a entries stay in place as
redirects, because a sticky id that silently disappears is worse than one that
points somewhere. **Q-M1c.3–5 left the same way on 2026-08-23**, with P1c.2
becoming [M10](../../../plans/m10_external_benchmarks/README.md): their text lives there
as Q-M10.1–3 and the rows below say so.

## Index

| Q | title | status |
|---|---|---|
| [Q-M1c.1](#q-m1c1--how-does-a-program-state-what-it-expects) | How does a program state what it expects? | **closed 2026-08-23 → (c)**, `:expect` on `query`, several queries per file *(was Q-M1a.19)* |
| [Q-M1c.2](#q-m1c2--what-may-an-expectation-say) | What may an expectation say? | **closed 2026-08-23** — a `(model …)` of ground facts, relation-closed; `(or …)` a set; `(false)` for ⊥; route parked *(was Q-M1a.20)* |
| [Q-M1c.6](#q-m1c6--how-does-an-expectation-say-a-relation-is-empty) | How does an expectation say a relation is *empty*? | **open**, and found by building S1c.1.2 |
| [Q-M1c.7](#q-m1c7--may-an-expectation-name-a-relation-that-only-saturation-creates) | May an expectation name a relation that only saturation creates? | **open**, and found by building S1c.1.4 |
| ~~Q-M1c.3~~ | What makes a benchmark encoding fair? | **moved 2026-08-23 with P1c.2 → [Q-M10.1](../../../plans/m10_external_benchmarks/open_questions.md#q-m101--what-makes-an-encoding-fair)** |
| ~~Q-M1c.4~~ | Does a proof assistant belong in a timing table? | **moved 2026-08-23 with P1c.2 → [Q-M10.2](../../../plans/m10_external_benchmarks/open_questions.md#q-m102--does-a-proof-assistant-belong-in-a-timing-table)** |
| ~~Q-M1c.5~~ | Where does the benchmark live, and is any of it a gate? | **moved 2026-08-23 with P1c.2 → [Q-M10.3](../../../plans/m10_external_benchmarks/open_questions.md#q-m103--where-does-the-benchmark-live-and-is-any-of-it-a-gate)** |

---

## Q-M1c.1 — How does a program state what it expects?

[P1c.1](README.md#p1c1--stdlib-conformance) needs somewhere to say what a
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

### Closed 2026-08-23 — (c), as recommended

Built in [S1c.1.2](README.md#s1c12--how-a-program-states-what-it-expects). What the
build changed about the recommendation, and what it cost:

- **The value needs a head.** The proposed shape — a bare list of facts,
  `:expect ((p A H1) (q B H2))` — **does not parse**, and the reason is
  structural rather than incidental: `ListHead ::= SYMBOL | VAR | WILDCARD |
  EQ` and never a list
  ([`00_ebnf.md` §2](../../kernel/ir/03-ein-lang/00_ebnf.md)). Widening
  `ListHead` would change the grammar of *every* form to buy one keyword its
  ergonomics. The shape is therefore `(model <fact>*)`, whose `SYMBOL` head
  costs nothing: it parses, dumps and round-trips today, `(or (model …)
  (model …))` falls out of `OrForm` with no further work, and `model` needs no
  reservation because it is read structurally under `:expect` and nowhere else.
  **The keyword count is still one**, which is what the row that decided this
  was about.
- **`Program.query` became `Program.queries` + an active index**, and the CLI
  loads once per query rather than sharing a `Kb` — not a concession but the
  honest shape, since `:hypothesis-relations` and `:hrules` are per-query and
  two queries over one KB are two genuinely different searches. Fourteen call
  sites; the goldens did not move, because **no file in the 128-entry corpus
  had a second `(query …)` to discard**. The trap was real and had never been
  sprung.
- **One thing the recommendation did not anticipate**: an artefact flag names
  *one* path. `--events`, `--trace`, `--json-summary` and `--dump-states` are
  refused (exit 2) on a file that asks more than one question, because
  overwriting quietly is the last-wins discard again under another name.
- **And the keyword vocabulary became an allow-list**, which row 1 of the table
  above did not price. It is the same argument one level up: a mistyped
  `:expect` that parsed, loaded and checked nothing would be exactly the
  failure the keyword exists to prevent. Six keywords plus the obsolete
  `:mode`; anything else is a load error.

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
cut** and let [S1c.1.1](README.md#s1c11--what-the-stdlib-promises-and-what-is-exercised)'s
table say whether a rule needs it. Under (c) it arrives as one more keyword on
the same query, which is the other way (c) beats (b): the vocabulary grows a
key at a time instead of arriving whole.

### Closed 2026-08-23

Both sub-questions went the way they were recommended, and the census settled
the residue:

- **Stored negatives do not count.** Closing `pet-loc` asserts its *positive*
  extent only. A `(not …)` is listable as an ordinary fact and is then checked
  for presence — so a test can pin one deliberately without dragging in the
  negative-completion rules' whole output, which on a Zebra puzzle is most of
  the model.
- **`Contradiction` is spelled `:expect (false)`** — corrected 2026-08-24, a
  day after it shipped as `:expect none`. The recommendation said "not
  `:expect ()` if that reads as the empty model, and `none` is clearer", and it
  was right about the first half and reaching for an invented word in the
  second: **`false` is already ein's ⊥** — one of the five `STRUCTURAL` names,
  and what every refutation rule in the stdlib asserts. `(model)` remains a
  legal expectation meaning *the empty model*, which is why `()` was never the
  answer; the pair is `(false)` and `(model)`.
- **Route stays out**, and
  [`stdlib_census.md` §8](stdlib_census.md#8-four-declarations-are-two-rules)
  is why: the pairs that raise the question — `converse` ≡ `imply2-reverse`,
  `imply2-fwd` ≡ `includes` — are alpha-identical bodies under two names, so
  one test covers both and there is nothing a `:fires` would distinguish.
  `domain-elimination` and `range-elimination`, the case that motivated the
  residue, have *different* bodies and an `:expect` naming the relation tells
  their results apart.

What the build added to the vocabulary beyond the recommendation: nothing. What
it added to the *rules*: expectations are **ground** — a `?var` or `_` in an
expectation is a load error, because an expectation is an answer and a pattern
there would match whatever the engine happened to derive, which is a test that
cannot fail.

## Q-M1c.6 — How does an expectation say a relation is *empty*?

**Opened 2026-08-23**, by building [S1c.1.2](README.md#s1c12--how-a-program-states-what-it-expects).

Relation-closure has one state it cannot express. Naming a relation closes it,
and the only way to name one is to list a fact in it — so **"relation `r` is
empty in this model" is unsayable**. Usually that does not matter, because a
relation nobody mentions is unconstrained and an empty one is normally
uninteresting. It matters in exactly one place: rule 1 makes the goal's
relations mandatory, so a query whose goal relation is *legitimately* empty in
a `Solution` cannot be given a valid `:expect` at all — the loader refuses it.

Whether such a query exists in practice is the open part. `:expect (false)`
covers the `Contradiction` case, which is the usual reason a goal projects
nothing. Three shapes if one turns up, cheapest first: allow a bare relation
name as a model item (`(model p (q A))` — "`p` is empty, `q` is exactly
`(q A)`"); relax rule 1 to "names, or the model is `(model)`"; or a keyword.
**Deliberately not decided while no fixture needs it** — the residue-parking
discipline Q-M1c.2 used for route.

---

## Q-M1c.7 — May an expectation name a relation that only saturation creates?

**Opened 2026-08-24**, by building [S1c.1.4](README.md#s1c14--the-stdlib-corpus).

`:expect` is validated at **load**, and one of its five refusals is that a
relation it names must be one `kb.program().relations` already has — a name no
declaration and no fact makes would close a relation that does not exist, and
pass vacuously for ever. That is the right check for a typo. It is the wrong
check for a **derived** relation.

The stdlib's fan-out rules are exactly that case, and there are seven of them:
`std.bijection`'s `bijective-setup` and `typecheck-setup`, `std.slots`'
`slot-partition-setup` and `slot-spatial-setup`, `std.typing`'s
`derive-reflexive`, and `std.algebra`'s `derive-join` and
`bijective-properties`. Their entire output is activator facts —
`(domain-elimination R isa)`, `(slot-locate R isa Index)`,
`(typecheck-arg-0 R isa A)` — in relations that exist *only after saturation*.
So the one thing each of those rules promises is the one thing an expectation
cannot state, and
[`stdlib_census.md` §6](stdlib_census.md#6-what-would-activate-a-rule-that-nothing-activates--t1c113)'s
prediction that the fan-outs would be the cheapest bucket — *"a claim about
facts, and so exactly what an `:expect` can say with nothing else in the
file"* — is false as the loader stands.

What S1c.1.4 did instead is pin the fan-outs **at one remove**: the rules they
activate have no other activator, so a setup rule that dropped an operand
leaves the file with fewer negatives, or with none. That works, and it is
weaker than the direct claim in a specific way — it cannot catch a fan-out
that produces a *surplus* activator, because closure is what catches surplus
and closure is what is unavailable.

The shape of the fix is small and the risk is that it is too small to be
right: widen the check from "some declaration or fact makes this a relation"
to "…**or some rule can assert it**", computed from the compiled plans'
ground assert heads (`ein_infer::compile::asserted_relation` already does this
for the closed-relation pass, one crate above `ein-ir`, which is why it is not
a two-line change). That keeps the anti-typo property — a misspelling is
neither declared nor asserted — and admits every derived relation. The
question is whether a relation asserted only through a **variable** head
(`(?R ?a ?b)`, which is most of the stdlib) should count, and it cannot: the
head is not known until the activator is. So the widening admits the fan-outs
and still refuses, say, `(model (co-loc Ann S1))` in a file where `co-loc` is
declared nowhere — which is the case that matters.

**Deliberately not decided here**, for the reason S1c.1.4 is a corpus stage
and not a language one: the fixtures it owed are written and they are
checkable without it. Same residue-parking discipline as
[Q-M1c.2](#q-m1c2--what-may-an-expectation-say) used for route and
[Q-M1c.6](#q-m1c6--how-does-an-expectation-say-a-relation-is-empty) for the
empty extent — and the three are the same question asked three ways, which is
**what may appear on the left of a closure claim**. If a fourth arrives they
should be answered together.
