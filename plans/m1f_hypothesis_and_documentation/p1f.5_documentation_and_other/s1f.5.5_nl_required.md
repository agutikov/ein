# S1f.5.5 — Every statement convertible to NL

**Phase:** [P1f.5](README.md) (Documentation ein does not have)
**Estimate:** 4 days
**Depends on:** nothing to start. Its *definition* leans on
[Q-M1e.6](../../m1e_review_processing/open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
and [Q-M1e.7](../../m1e_review_processing/open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model),
which decide **which fact set** the claim quantifies over — the model or the
solution KB. Both are ratified.
**Blocks:** nothing in M1e. It is the IR→NL half of the round trip
[`zebra_walkthrough.md`](../../../docs/kernel/inference/zebra_walkthrough.md)
calls the M2 target, and the channel
[`EinAf.md`](../../m2_nl_to_ir/EinAf.md#f7--provenance)'s F6/F7 feedback grades
are written against.
**Source:** the user's note of 2026-08-28 — *"make `:why` or other →NL
conversion string/template mandatory, so every fact, match, every Ein statement
can be converted to NL."*

---

## Context

**Ein already renders NL, and it is the puzzle's own English, not the
engine's.** `ein solve examples/zebra.ein` ends:

```text
    query facts                       rendered
    (co-located Water House-1)        Water and House-1 are in the same house
    (instance House-1 House)          House-1 is an instance of House
    …
  result
    The Norwegian drinks water in House-1; the Japanese owns zebra in House-5
```

Three template sources feed one substitution engine
([`why.rs`](../../../ein.rs/crates/ein-core/src/why.rs) `render_why`, which
replaces `{?name}` from a binding list and **leaves an unbound reference
verbatim**):

| source | attached to | binds | drives |
|---|---|---|---|
| `:why "<tmpl>"` | `(rule …)`, `(hrule …)` | the firing's variables | the trace's step line, the DOT slice label, an obligation's owe line |
| `:why "<tmpl>"` | `(relation …)` | `{?1}`, `{?2}` — **positional** | the answer table's *rendered* column |
| `:goal-text "<tmpl>"` | `(query …)` | the goal's variables from the solution | the one-line NL result |

Beside them, `:source "<sentence>"` on a load-time fact carries the puzzle's
**original sentence** — 254 occurrences — and reaches the trace as the quoted
premise, never the answer.

### What the corpus actually carries

Measured at `9ba2349`, 2026-08-28, by parsing every top-level form under
`stdlib/`, `examples/`, `tests/` and `corpus/`:

| form | n | with a template | |
|---|---:|---:|---|
| `(rule …)` | 379 | **361** | 95 % |
| `(hrule …)` | 24 | **21** | 87 % |
| `(relation …)` | 409 | **162** | 39 % |
| `(macro …)` | 6 | **0** | the grammar has no slot |
| `(query …)` `:goal-text` | — | **13** | in 7 files; every other query prints *(query has no :goal-text template)* |

`stdlib/` is at **77 of 77** rules — the standard library already lives by the
law this stage proposes. The gap is relations, and it is not where the answer
table makes it look.

### The number that says why this stage exists

`examples/zebra2.ein`'s **unique model is 444 facts over 40 relations**, and
**25 of them — 5 % — belong to a relation that carries a `:why`.** The file
declares 13 relations, 5 of them templated; the other 27 relations in the model
are auto-vivified — stdlib-derived (`domain-elimination`, `typecheck-arg-0`,
`co-located-negative`, `total-owed`) or property heads (`symmetric`,
`bijective`, `relation`).

The answer table looks fully narrated because it renders the **8 goal facts**
and nothing else. Behind it, ninety-five percent of what the engine proved has
no English at all. *That* is the finding, and it is why the request cannot be
satisfied by a check that every `(rule …)` carries a `:why`: 95 % of them
already do.

### Why the missing 95 % has no place to put a template

An auto-vivified relation has **no declaration** — `declared: false`,
created on the fly for an open-world head
([`entities.rs`](../../../ein.rs/crates/ein-core/src/entities.rs)) — so there
is no form for `:why` to hang off. The stdlib cannot declare it either: a
stdlib rule is kernel-pure and generic, and the relation it asserts into is the
puzzle's.

But every one of those facts **already carries the thing needed**: a
provenance. [`prov.rs`](../../../ein.rs/crates/ein-core/src/prov.rs)'s
`ProvKind` has exactly four values — `Source`, `Rule`, `Hypothesis`,
`Rejected` — and they line up one-for-one with the four registers a fact-level
renderer needs:

| provenance | what renders it | coverage today |
|---|---|---|
| `Source` | the fact's `:source` sentence, else its relation's `:why` | 18 of 80 load facts in `zebra2` carry `:source` (22 %) |
| `Rule` | **the deriving rule's `:why`, under the firing's bindings** | 95 % of rules have one — and the trace already renders exactly this, per step |
| `Hypothesis` | the commitment's rendering | exists, in the trace's *Assuming …* line |
| `Rejected` | the reductio line | exists, `Assumed **X**; the branch derives ⊥.` |

The trace narrates a *firing*; the answer narrates a *relation*. Nothing
narrates a **fact** — and a fact's provenance is the join between the two.
That is the design this stage should take, and it costs no new keyword.

### What is deliberate and must stay

[`answer.rs`](../../../ein.rs/crates/ein-render/src/answer.rs)'s header states
the rule the whole surface is built on:

> **No hardcoded vocabulary.** … A relation without a `:why` renders as its raw
> IR s-expression `(R a b)` — never invented prose.

A mandate must not overturn that. There is engine English — `Solved in {n}
steps`, `Lifted no-good:`, `Outstanding obligations`, `Assumed **X**; the
branch derives ⊥.` — and it is the engine's to own, the way
[`reserved_engine_strings.md`](../../../docs/kernel/inference/reserved_engine_strings.md)
owns the engine's reserved *names*. The mandate is about **domain**
vocabulary. Nothing registers that boundary today.

## What "convertible to NL" has to mean before it can be checked

The request is a totality claim about the language, and a totality claim needs
a quantifier and a witness. Four candidate readings, in increasing strength:

1. **Every form may carry a template.** A grammar claim. False today for
   `(macro …)`, for an auto-vivified relation, and per-conjunct for a goal.
2. **Every form does carry one.** A corpus claim. Achievable only by writing
   247 relation templates, half of them for relations the *stdlib* derives and
   the puzzle never names.
3. **Every fact of a reported answer renders with no fallback.** An *output*
   claim — countable, per file, and the one this stage recommends: it is 5 % on
   `zebra2` today and the provenance design above is what moves it.
4. **The rendering round-trips** — the NL re-parses to the same IR. That is
   [M2](../../m2_nl_to_ir/README.md)'s, not this stage's, and saying so here is
   part of the result.

Reading (3) is the acceptance below. It is the only one that is a number
rather than an opinion, and it is the one that makes the answer *useful*:
`EinAf.md`'s F6/F7 grades hand the formalizer a dependency and a provenance,
and a provenance whose facts are s-expressions is a provenance an LLM has to
re-derive the meaning of.

## Acceptance

- **One census, re-takable**: `utils/nl_census.py`, the twenty-fourth script,
  reporting per corpus entry — facts in the reported answer, facts that render
  from a template, facts that fall back to an s-expression, and **the register
  each fallback is in** (puzzle / stdlib-derived / kernel). Its headline is one
  number per entry and one for the corpus.
- **One page**, `docs/kernel/ir/03-ein-lang/09_natural_language.md` (name
  provisional — P1e.2's triage may move the tree), stating the four registers,
  the four provenance kinds and what renders each, the fallback ladder, and
  **where the boundary between domain and engine vocabulary is**. Linked from
  [`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md)'s
  § `:why` and from `docs/kernel/README.md`.
- **Fact-level rendering from provenance ships**, or the stage records a
  written *no* with the reason. If it ships, `zebra2`'s model coverage moves
  from 5 % to a number the census prints, and that number is in the page.
- **Two mechanical checks, both in `cargo test`:**
  - *template well-formedness* — every `{?x}` in a rule's `:why` names a
    variable that rule binds; every `{?n}` in a relation's `:why` is positional
    and within arity. **Today this is 0 findings over 544 templates** — it
    ships as a regression guard, and the stage says so rather than dressing a
    green check as a discovery.
  - *no unsubstituted placeholder reaches an output* — a rendered line
    containing `{?` is a defect, and `render_why`'s documented
    leave-it-verbatim behaviour is what makes it reachable.
- **A scoped mandate, not a global one.** Whatever binds, binds the way
  [`stdlib_coverage.rs`](../../../ein.rs/crates/ein-infer/tests/stdlib_coverage.rs)
  binds: to the roots the repo controls (`stdlib/`, `tests/stdlib/`), never to
  every `.ein` anyone writes. T4 is where that is decided and written down.
- No number in the page is one this stage counted by hand and nothing
  re-counts — [Q-M1e.4](../../m1e_review_processing/open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all).

## Tasks

### Task T1f.5.5.1 — The census, and the four registers

Write `utils/nl_census.py`. Per corpus entry under its declared runs: the
reported answer's fact set (`--json-summary`'s `verdict.solutions[i].facts`,
which is what [Q-M1e.7](../../m1e_review_processing/open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)
is about — record which set was counted), each fact's relation, whether that
relation carries a `:why`, and the register:

- **puzzle** — declared in the file being solved;
- **stdlib-derived** — asserted by a `std.*` rule (join against
  `stdlib_census.py`'s rule-head parse, which already exists);
- **kernel** — `not`, `relation`, `is-a*`, `open`, and whatever else
  [`reserved_engine_strings.md`](../../../docs/kernel/inference/reserved_engine_strings.md)
  registers.

The register split is the deliverable, not the total: a 5 % total is not
actionable, *"27 of 40 relations are stdlib-derived and no puzzle can template
them"* is.

`--check` exits 1 below a floor — but per
[TE-L4](../../m1e_review_processing/README.md#the-findings), a `--check` wired to no gate is a check
nobody runs, so the gate is T5's cargo test and this is the measurement it is
a yes/no of.

### Task T1f.5.5.2 — Fact-level rendering from provenance

The change: when rendering a fact for a human, consult its provenance before
its relation.

```text
fact → prov.kind
  Source     → the fact's :source, else the relation's :why, else (R a b)
  Rule       → the deriving rule's :why under the firing's bindings,
               else the relation's :why, else (R a b)
  Hypothesis → "Suppose …" + the same ladder
  Rejected   → the reductio's line
```

The `Rule` arm is the one that moves the number, and it is not new code so
much as a **new caller**: `linearize.rs` already renders exactly this per step
([`linearize.rs:172`](../../../ein.rs/crates/ein-render/src/trace/linearize.rs)),
and `answer.rs` does not have it. Lift the rendering to a function both call.

Three things to establish before writing it:

- **A fact may have up to 32 recorded justifications**
  (`MAX_ALT_JUSTIFICATIONS`, [`kb.rs:49`](../../../ein.rs/crates/ein-core/src/kb.rs)).
  Rendering must pick one **deterministically**, and which one is
  [Q2](../../m1e_review_processing/p1e.1_open_questions/s1e.1.3_unsat_core_completeness.md)'s
  neighbourhood — a fact whose narration depends on recording order is a
  narration that moves when a rule's `:priority` moves
  ([S1f.5.6](s1f.5.6_rule_priority.md) is the other end of that thread).
  Take *first recorded*, say so, and let S1e.1.3's answer refine it.
- **Determinism.** The rendered text is output, so it is under the same
  invariance the `id_order_invariance` and `jobs_invariance` suites hold the
  rest of the read-out to.
- **Cost.** A provenance lookup per rendered fact, on a path that today renders
  8 facts and would render 444. Measure it on the zebra family before shipping;
  the answer table is on every `ein solve`.

### Task T1f.5.5.3 — Where a template has nowhere to live

Three forms have no slot. Decide each, and write the decision down:

- **`(macro …)`** — 6 in the corpus, 0 templated. A macro is consumed at load
  time and nothing reads the registry afterwards, so a macro never appears in
  an answer. The likely answer is *no slot, and here is why*, which is a
  sentence in the page and not a change.
- **An auto-vivified relation** — no declaration to annotate. The provenance
  route (T2) covers every *derived* one. What it does not cover is a
  load-time fact of an undeclared relation; that is a small, countable set and
  the census prints it.
- **A goal's conjuncts** — `:goal-text` is one string for the whole query,
  which is why the answer table renders the conjuncts through their
  *relations* instead. Whether a per-conjunct template is wanted is a language
  question; if the answer is yes it is a `Q-M1e.<n>`, not a quiet addition.

### Task T1f.5.5.4 — The mandate, and where it binds

Four rungs, and the stage picks one *and writes the argument*:

| rung | what it does | cost |
|---|---|---|
| **a. derived default** | the fallback ladder, made normative in the page | 0 — it is what ships |
| **b. census + `--check`** | a number, and a floor nobody is forced past | 1 script |
| **c. gate scoped to a root** | stdlib + `tests/stdlib/` must be 100 %; `examples/` need not | 1 cargo test; the `stdlib_coverage.rs` shape |
| **d. a load error** | `:why` mandatory in the grammar | breaks 18 rules, 247 relations, and every inline program in the Rust suites; re-blesses `corpus_shapes.md5`, which prints `why=` per rule and relation |

**Recommendation: (a) + (b) + (c), and (d) explicitly refused in writing.**
Rung (d) is where a mandate turns into a lie: 100 % coverage bought with
`:why "rule fired"` is *worse* than 39 % with honest s-expressions, because the
s-expression is true and the prose is noise. The repo already made this call
once, in `answer.rs`'s *never invented prose*, and a mandate that reverses it
should be a ruling and not a side effect.

If the user wants (d) anyway, it is a separate stage with a corpus-wide
migration commit, and this stage's page is its specification.

### Task T1f.5.5.5 — Bank both checks

- `template_wellformedness` in `ein-ir`'s suite: parse each rule's `:why` for
  `{?x}` refs, assert each names a bound variable or a param; each relation's
  refs are positional and within arity. **Runs green today** — 0 of 544, and
  the check exists because `render_why` leaves an unbound ref verbatim, so the
  failure mode is a typo shipping into a user's answer as `{?huse}`.
- `no_placeholder_reaches_an_output` in the corpus sweep: no line of a rendered
  answer, trace or owe list contains `{?`.

Both are cheap and neither is a discovery. Say that in the commit message —
[Q-M1e.1](../../m1e_review_processing/open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)'s
standard cuts both ways, and a probe that comes back clean is still a result.

## Risks

- **A mandate that manufactures prose.** The whole risk of the stage, and the
  mitigation is that acceptance is measured on *rendered output* with a
  register split, not on keyword presence. A relation nobody can describe in
  English is a relation whose s-expression is the honest rendering.
- **The engine/domain boundary is not written down anywhere.** Eight English
  strings live in `ein-render`. If the page draws the line loosely, the next
  stage that adds a verdict word will not know which side it is on. The line
  belongs beside `reserved_engine_strings.md`'s, and probably *in* it.
- **Rendering 444 facts where 8 were rendered.** A per-fact provenance lookup
  on the default `ein solve` path. Measure before shipping; if it costs, the
  full rendering is a flag and the answer table stays as it is.
- **It touches the read-out three phases are already editing.**
  [SE-M1](../../m1e_review_processing/README.md#the-findings) and [AR-M2](../../m1e_review_processing/README.md#the-findings) are
  findings about read-out ownership being split across three crates, and T2
  adds a fourth caller unless it lifts the shared function that
  [S1e.3.4](../../m1e_review_processing/p1e.3_medium/s1e.3.4_architecture.md) is already going to want.
  Sequence after it, or hand it the function.

## Notes

The strongest single sentence this stage can leave behind is the one the census
makes checkable: **an Ein answer is as explainable as its provenance, not as
its vocabulary.** The 5 % is not a documentation debt — it is a renderer that
asks the wrong object.

`:source` deserves a second look while the stage is in there. 22 % of
`zebra2`'s load facts carry the puzzle's own sentence, and that sentence is the
best NL any renderer will ever have for a given. It reaches the trace and not
the answer, which is backwards.

## Connections

- [`docs/kernel/ir/03-ein-lang/01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md)
  § `:why` render template and § the division of labour — the normative
  statement the new page extends rather than restates.
- [`docs/kernel/inference/zebra_walkthrough.md`](../../../docs/kernel/inference/zebra_walkthrough.md)
  — the NL ⇄ ein-rule table, and the target this stage is the return leg of.
- [`plans/ideas/08-human-style-deductive-trace.md`](../../ideas/08-human-style-deductive-trace.md)
  — the user's own framing, authoritative on intent.
- [`EinAf.md` § Stage F](../../m2_nl_to_ir/EinAf.md) — F6 dependency and F7
  provenance are the grades whose usefulness depends on whether a provenance
  can be read as English.
- [S1f.5.6](s1f.5.6_rule_priority.md) — the other stage that turns on *which*
  of a fact's ≤ 32 justifications was recorded first.
