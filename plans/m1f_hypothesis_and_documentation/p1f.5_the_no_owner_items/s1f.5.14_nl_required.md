# S1f.5.14 — Every statement convertible to NL

**Phase:** [P1f.5](README.md) (The no-owner items, then NL and `:priority`)
**Was S1f.5.5**, until 2026-11-08, when P1e2.5's thirteen stages joined the phase ahead of it ([§ The ids](README.md#the-ids)).
**Estimate:** 4 days
**Status:** specified 2026-08-28; **T1f.5.14.6, the loader flag, done 2026-11-08, ahead of the phase** (the user:
*"yes, land the :why flag early"*; [its record](#t1f5146-record-2026-11-08)); **running since 2026-11-10**, the
phase's last stage, on default decisions recorded as they are taken (the user's instruction of 2026-11-09): T1, the
census, done ([its record](#t1f5141-record-2026-11-10)); T2, a fact in English from its first recorded justification,
in the state files and the trace at no measurable cost, done ([its record](#t1f5142-record-2026-11-10)); T3 and T4,
the three forms with nowhere to put a template decided and the page written, done
([T3](#t1f5143-record-2026-11-10), [T4](#t1f5144-record-2026-11-10)); T5, the two checks banked, the first with 66
findings, each fixed, done ([its record](#t1f5145-record-2026-11-10)); T7's part 1, every missing `:why` written,
done ([its record](#t1f5147-record-2026-11-10)). **Ruled and placed on 2026-11-08**
([P1f.5 § The order](README.md#the-order)). The user's principle that day, on the phase's grammar: *"collect all
requirement for the parser in this phase and implement grammar that will already meet all of them instead of making
wrong parser that meets our old wrong parser and then updating it step by step"*. NL `:why` was one of the two
items that would have edited the generated grammar later, and the user ruled it: *":why is optional in grammar and
loader has a flag that makes it required, start with default false so not break everything, then after other
stages finished swith to default true and fix remaining ein code"*. Asked whether the flag can change an answer:
*"yes, the flag only decides loading"*. It decides whether a program loads, never what a loaded program answers,
so it keeps the rule that config never changes the answer
([`configuration.md`](../../../docs/kernel/configuration.md) § 2.5). What that settles here:
- **The grammar part is done.** [S1f.5.5](s1f.5.5_the_grammar.md)'s T3 gave `(macro …)` an optional `:why`, once,
  after the body (`54715b5f`, `ein_ir::macros::Macro::why`), so in the generated grammar every form that can carry
  a template may: `:why` on a relation, a rule, an hrule and a macro, `:goal-text` on a query, `:source` on a fact.
  The hand-written parser refuses a macro's `:why` until S1f.5.5's swap. § T3's macro bullet is settled with it.
- **The flag**, default false, is this stage's first code task, [T1f.5.14.6](#task-t1f5146--the-loader-flag-default-false-first),
  ahead of T1's census. **Built 2026-11-08, early**, on the user's third ruling that day, *"yes, land the :why flag
  early"*: it landed as its own small commit while the rest of P1f.5 runs, and this stage keeps only the flip and
  the migration, last in the phase. `--require-why` on every command that loads a program,
  `ein_ir::LoadOptions::require_why` in the library ([§ T1f.5.14.6 record](#t1f5146-record-2026-11-08)).
- **Rung (d), *a load error*, is the target**, reached through the flag, where § T4 recommended (a) + (b) + (c)
  and refused (d) in writing. § T4 keeps that argument as the record of what the user overrode, and it stands as
  the risk the flip has to answer.
- **This stage runs last**, after every other stage of P1f.5 (*"after other stages finished"*). Its last task,
  [T1f.5.14.7](#task-t1f5147--the-flip-default-true-and-every-missing-why-written-last), turns the default to true
  and writes `:why` wherever the corpus, the stdlib and the Rust suites' inline programs still lack it.
**Depends on:** every other stage of P1f.5, which runs before it (the user's ruling of 2026-11-08); nothing else
to start. Its *definition* leans on
[Q-M1e.6](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
and [Q-M1e.7](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model),
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
**The non-NL half exists** since M1e2
[S1e2.1.4](../../m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.4_a_model_you_can_read.md):
`ein solve --models grid` renders a model as the table a person would draw,
and **the rule that picks the row type is written once**, in
[`ein-render/src/grid.rs`](../../../ein.rs/crates/ein-render/src/grid.rs)'s
module note (a `slot-partition`, or the type the most `functional` /
`injective` columns give one value to, never `T`). A sentence per row of that
grid is the shape this stage's NL read-out of a model would take, and it should
cite that rule rather than choose a second anchor.

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

`answer.rs`'s header states
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
- ~~**A scoped mandate, not a global one.** Whatever binds, binds the way
  [`stdlib_coverage.rs`](../../../ein.rs/crates/ein-infer/tests/stdlib_coverage.rs)
  binds: to the roots the repo controls (`stdlib/`, `tests/stdlib/`), never to
  every `.ein` anyone writes. T4 is where that is decided and written down.~~
  **Overruled 2026-11-08** (§ Status): the mandate binds every program the
  loader reads with the flag at its default, which is true after T1f.5.14.7.
- **The flag** (T1f.5.14.6), **met 2026-11-08**: with it on, a missing `:why` is a load error,
  one line naming the form; with it off, every program loads as before. It
  decides whether a program loads and nothing a loaded program answers, a test
  holds both, and [`configuration.md`](../../../docs/kernel/configuration.md)
  has its row ([§ T1f.5.14.6 record](#t1f5146-record-2026-11-08)).
- **The flip** (T1f.5.14.7): the default is true, and every program under
  `examples/`, `tests/`, `corpus/` and `stdlib/`, and every inline program of
  the Rust suites, loads with it. No R, N or exit code moved; every golden the
  written `:why`s move is named before the commit.
- No number in the page is one this stage counted by hand and nothing
  re-counts — [Q-M1e.4](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all).

## Tasks

*The order since the user's ruling of 2026-11-08 (§ Status): T6, the flag, first; then T1 to T5; then T7, the
flip, last. T6 and T7 keep the next free numbers, as S1f.5.5's T8 to T10 run before its T4. T6 ran on 2026-11-08,
ahead of the rest of P1f.5, on the user's ruling that day (*"yes, land the :why flag early"*); T1 to T5 and T7
stay last in the phase.*

### Task T1f.5.14.1 — The census, and the four registers

Write `utils/nl_census.py`. Per corpus entry under its declared runs: the
reported answer's fact set (`--json-summary`'s `verdict.solutions[i].facts`,
which is what [Q-M1e.7](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)
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
[TE-L4](../../../docs/history/m1e_review_processing/README.md#the-findings), a `--check` wired to no gate is a check
nobody runs, so the gate is T5's cargo test and this is the measurement it is
a yes/no of.

#### T1f.5.14.1 record, 2026-11-10

**The census is `utils/nl_census.py`**, the thirty-second script, in two halves, both running the binary
(`$EIN_BIN` or `--bin`). The fact set and the registers are
[Q-M1f.56](../open_questions.md#q-m1f56--what-the-nl-census-counts-which-fact-set-and-which-registers), taken by
default on the user's instruction of 2026-11-09: the stage file named `--json-summary`'s `verdict.solutions[i].facts`,
which M1e2's switch removed, so the census reads every fact of every state file `ein solve FILE --dump-answer DIR`
writes, the whole state the 444-fact number was about; and its registers are the stage's three and **undeclared**, a
relation written or derived and never declared, and no rule's, the small set § T3 says the census prints. Each
fact's provenance is read off its state-file comment and its templates off the program the file re-writes above it,
so the numbers are the same before T2 ships the English and after.

**The flag, re-taken** (`python3 utils/nl_census.py --flag-only`, 2026-11-10, at `b7108e3a`): of the 454 `.ein` files
under the four roots, 341 load and 113 do not load either way (`examples/broken/`, the parse-refused witnesses, the
fuzz findings refused at the parse); with a `:why` required, **238 are refused** (207 of `examples/`' 260 that load,
24 of `tests/`' 67, 7 of `corpus/`' 7) and none of `stdlib/`' 7, naming **726 relations, 166 rules and 64 hrules**,
and no macro. On 2026-11-08 it was 274 of 339, 806, 198 and 68: S1f.5.5's swap moved 25 refusal fixtures to
`examples/broken/`, where they do not load, and the lane merged since added files. Lexically (the T6 scan, every
top-level form, refused files included, over the four roots and `docs/`) 313 of 455 files lack one on 866 relations, 216 rules, 67 hrules and 5
macros; 53 of those files are refused at the parse, which [Q-M1f.57](../open_questions.md#q-m1f57--which-fixtures-get-a-why)
leaves as they are, and the rest, 259 files and 995 forms, are T7's.

**The answer, re-taken** (`python3 utils/nl_census.py --slow`, 2026-11-10, at `b7108e3a`): 405 entries with a `solve`
run, 211 of them answered with a state file (194 have none: a refusal, exit 1, on 131, and an empty or owing R on
63), 12 934 states and 239 359 facts. **0.9 % of the facts have a statement, 44.7 % a reason, 44.9 % render**; 20
entries render every fact, and the mean entry renders 33.3 %. The fallbacks by register:

| register | facts | fallback |
|---|---:|---:|
| puzzle | 146 941 | 130 650 |
| stdlib | 5 275 | 555 |
| kernel | 86 377 | 35 |
| undeclared | 766 | 540 |

The puzzle's fallbacks are its untemplated declarations, `is-a` (78 361) and `co-located` (30 703) the most, which a
`:why` on each declaration moves; the kernel's are few because a `not` is derived, and reads by its rule's reason;
the stdlib's and the undeclared ones are given activators, `(bijective color-loc)`, `(symmetric next-to)`, which no
declaration can reach (§ T3). On `examples/zebra2.ein`'s one state, 426 facts: **5.9 % have a statement** (the
stage's 5 %, re-taken), 88.5 % a reason, 88.7 % render, and its 48 fallbacks are 37 `is-a`, 5 `bijective`, 2
`includes` and one each of `symmetric`, `transitive`, `bijection-hierarchy` and `typecheck-hierarchy`. On
`examples/zebra.ein`'s, 606 facts: 20.5 %, 92.7 % and 99.0 %.

### Task T1f.5.14.2 — Fact-level rendering from provenance

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
  [Q2](../../../docs/history/m1e_review_processing/README.md#s1e13--what-the-core-promises-q2)'s
  neighbourhood — a fact whose narration depends on recording order is a
  narration that moves when a rule's `:priority` moves
  ([S1f.5.15](s1f.5.15_rule_priority.md) is the other end of that thread).
  Take *first recorded*, say so, and let S1e.1.3's answer refine it.
- **Determinism.** The rendered text is output, so it is under the same
  invariance the `id_order_invariance` and `jobs_invariance` suites hold the
  rest of the read-out to.
- **Cost.** A provenance lookup per rendered fact, on a path that today renders
  8 facts and would render 444. Measure it on the zebra family before shipping;
  the answer table is on every `ein solve`.

#### T1f.5.14.2 record, 2026-11-10

**Fact-level rendering ships, in the two surfaces that print a fact for a reader and are not Ein text a claim is
pasted from** ([Q-M1f.58](../open_questions.md#q-m1f58--where-a-fact-is-rendered-in-english-and-what-it-reads-from),
taken by default on the user's instruction of 2026-11-09). The stage file's surface, the read-out's *rendered* answer
table, went with the walk at M1e2 S1e2.1c.5; since then a relation's `:why` reached no surface at all, though
`01_grammar.md` still said the solve table printed it. The read-out's `(model …)` blocks are Ein text a claim is
pasted from (`as :expect`), so they stay s-expressions, as `ein solve`'s stdout and `answer.txt` do: **no read-out
byte moves**, and `read_out.md` and its examples page are unchanged.

- **`ein_render::sentence`**, a module of its own: `rule_sentence` fills a rule's (or hrule's) `:why` with a firing's
  bindings, lifted out of `trace/linearize.rs` and `slice.rs`, which both call it now, so the trace, the DOT slice
  and a state file fill a rule's `:why` one way; `statement` fills a relation's `:why` with a fact's arguments; and
  `fact_sentence` reads a fact's **first recorded justification** ([`Kb::primary`]) for its origin (`source`, `rule`,
  `hypothesis`, or a given) and its reason, beside its statement.
- **A `--dump-answer` state file** writes each fact's comment as `; rule r: <reason> — <statement>`, `; source "…" —
  <statement>`, `; hypothesis N — <statement>`, or `; <statement>` for a given with no sentence; a fact with no text
  keeps the comment it had. A line break in a text starts a comment line of its own (`;   `), so the text is carried
  character for character and the file reloads to its facts (`answer_round_trip.rs`, unchanged).
  `natural_language_text.rs`' new `a_state_file_writes_each_fact_in_english` holds both texts on Cyrillic, CJK,
  emoji and every escape.
- **`--trace`**'s step says what the derived fact says: *Derives `f` — <statement>* where its relation carries a
  `:why` (the same test).
- **First recorded**, as the stage file said to take, and not refined: a reason is an explanation, as a core is, and
  the user's ruling of 2026-11-08 (*"core is an explanation, F22 makes it canonical"*) covers it. Within a run it is
  deterministic: a state file's KB is its commitment's fixpoint, built once per state by `Nodes::with_fixpoint`, and
  the corpus sweep gains a **`states` op** (`ein-render/tests/corpus_ops`), the state files under describe compared
  exactly, which `id_order_invariance` and `jobs_invariance` now hold.
- **What moved, named before it was blessed** (`EIN_BLESS=1`): `corpus_shapes.md5` gains the `states` digest of
  every corpus file that answers with a state, 334, and the `region-trace` digest moves on the 52 files whose trace
  derives a fact of a relation with a `:why`, each by the statement after its *Derives* lines; of the six trace
  goldens one moves, `trace_owing.md`, by one line: *Derives `closed-here(r)` — r's extension was closed by
  infer-closure*. No R, N, H, claim, exit code or read-out byte moved.

**The cost** (`utils/bench_env.sh`, pinned to cpu4, 2026-11-10: each run best of 7, the binary built at `b7108e3a` against this
commit's, on the zebra family):

| program | `ein solve F` | `--dump-answer D` | `--trace T` |
|---|---:|---:|---:|
| `zebra.ein` | 884 → 882 ms (0.998) | 2727 → 2720 ms (0.998) | 2771 → 2764 ms (0.997) |
| `zebra2.ein` | 505 → 506 ms (1.001) | 6546 → 6540 ms (0.999) | 6565 → 6561 ms (0.999) |
| `zebra2-obligations.ein` | 505 → 505 ms (0.998) | 6546 → 6538 ms (0.999) | 6557 → 6553 ms (0.999) |
| `zebra2-minus-15.ein` | 8746 → 8766 ms (1.002) | 29704 → 29710 ms (1.000) | 29792 → 29749 ms (0.999) |
| `zebra2-minus-15-obligations.ein` | 8701 → 8716 ms (1.002) | 29620 → 29627 ms (1.000) | 29698 → 29678 ms (0.999) |
| `zebra2-hints.ein` | 105 → 105 ms (0.998) | 1289 → 1290 ms (1.001) | 1295 → 1295 ms (1.000) |

Every ratio is between 0.997 and 1.002: **no measurable cost**, so the rendering ships on the default path of the
two surfaces and no presentation option gates it. A bare `ein solve` renders no fact; a dump's and a trace's seconds
are the cores they take for N, which the English does not touch.

### Task T1f.5.14.3 — Where a template has nowhere to live

Three forms have no slot. Decide each, and write the decision down:

- **`(macro …)`** — 6 in the corpus, 0 templated. A macro is consumed at load
  time and nothing reads the registry afterwards, so a macro never appears in
  an answer. The likely answer is *no slot, and here is why*, which is a
  sentence in the page and not a change. **Settled 2026-11-08, the other
  way**: the generated grammar gives a macro an optional `:why` (S1f.5.5's T3,
  `54715b5f`), on the user's ruling that `:why` is optional in the grammar and
  a loader flag makes it required. What a macro's `:why` renders, when nothing
  reads the registry after load, is still this task's to write down.
- **An auto-vivified relation** — no declaration to annotate. The provenance
  route (T2) covers every *derived* one. What it does not cover is a
  load-time fact of an undeclared relation; that is a small, countable set and
  the census prints it.
- **A goal's conjuncts** — `:goal-text` is one string for the whole query,
  which is why the answer table renders the conjuncts through their
  *relations* instead. Whether a per-conjunct template is wanted is a language
  question; if the answer is yes it is a `Q-M1e.<n>`, not a quiet addition.

#### T1f.5.14.3 record, 2026-11-10

Each of the three decided by default on the user's instruction of 2026-11-09, and written where the definition lives,
§ 5 of the page T4 writes:

- **A macro's `:why`** ([Q-M1f.59](../open_questions.md#q-m1f59--what-a-macros-why-renders), (a)): required like the
  others since S1f.5.5's swap gave a macro one, and rendered nowhere. A macro is expanded at load and no fact's
  justification names one; the rule whose `:match` used it has a `:why` that says what the pattern meant there, and a
  state file re-writes the macro's with the program.
- **An auto-vivified relation** ([Q-M1f.60](../open_questions.md#q-m1f60--a-relation-nobody-declares), (a)): no slot.
  A fact of one that a rule derived reads by its reason, since T2; one the program states falls back to its
  s-expression, and the census counts it under `stdlib` (an activator of a `std.*` rule, `(bijective color-loc)`) or
  `undeclared`: 1 095 of them on the corpus before the migration, `zebra2`'s eleven (§ T1's record).
- **A goal's conjuncts** ([Q-M1f.61](../open_questions.md#q-m1f61--a-template-per-goal-conjunct), (a)): no
  per-conjunct template, which would be a language change; each conjunct's fact reads by its relation's `:why` where a
  surface prints it.

### Task T1f.5.14.4 — The mandate, and where it binds

Four rungs, and the stage picks one *and writes the argument*:

| rung | what it does | cost |
|---|---|---|
| **a. derived default** | the fallback ladder, made normative in the page | 0 — it is what ships |
| **b. census + `--check`** | a number, and a floor nobody is forced past | 1 script |
| **c. gate scoped to a root** | stdlib + `tests/stdlib/` must be 100 %; `examples/` need not | 1 cargo test; the `stdlib_coverage.rs` shape |
| **d. a load error** | `:why` mandatory in the grammar | breaks 18 rules, 247 relations, and every inline program in the Rust suites; re-blesses `corpus_shapes.md5`, which prints `why=` per rule and relation |

**Ruled 2026-11-08: (d), through a loader flag, not the grammar** (§ Status). The user: *":why is optional in
grammar and loader has a flag that makes it required, start with default false so not break everything, then
after other stages finished swith to default true and fix remaining ein code"*. `:why` stays optional in the
grammar, and a loader flag ([T1f.5.14.6](#task-t1f5146--the-loader-flag-default-false-first)) makes it required:
default false until [T1f.5.14.7](#task-t1f5147--the-flip-default-true-and-every-missing-why-written-last), the
stage's last task, turns it to true. So the target is (d) in two steps, and (d)'s cost row is the migration
T1f.5.14.7 does; its counts, taken at `9ba2349`, are re-taken by T1 and T6. The other rungs still serve. (a) and
(b), the fallback ladder and the census, cover what no flag can reach: a relation nobody declares has no form to
carry a `:why`, and the rendered-output reading (3) is measured by the census, not by keyword presence. (c), a
test that loads `stdlib/` and `tests/stdlib/` with the flag on, holds those roots between T6 and the flip, and
the default holds them after. T5's two checks are unchanged.

*The recommendation as it stood until the ruling, kept as the record of what the user overrode, and as the risk
T1f.5.14.7 has to answer:*

**Recommendation: (a) + (b) + (c), and (d) explicitly refused in writing.**
Rung (d) is where a mandate turns into a lie: 100 % coverage bought with
`:why "rule fired"` is *worse* than 39 % with honest s-expressions, because the
s-expression is true and the prose is noise. The repo already made this call
once, in `answer.rs`'s *never invented prose*, and a mandate that reverses it
should be a ruling and not a side effect.

If the user wants (d) anyway, it is a separate stage with a corpus-wide
migration commit, and this stage's page is its specification. *Since the
ruling it is this stage's T1f.5.14.7, not a separate stage.*

#### T1f.5.14.4 record, 2026-11-10

**Rung (d), through the loader flag**, as the user ruled on 2026-11-08; § T4's recommendation stays above as the record
of what the user overrode and of the risk T7 answers. The page is
[`docs/kernel/ir/03-ein-lang/09_natural_language.md`](../../../docs/kernel/ir/03-ein-lang/09_natural_language.md),
*current*, 46 kernel pages since (`git ls-files 'docs/kernel/*.md' | wc -l`, 2026-11-10): the four texts and where
each is printed (§ 1), the mandate and what an honest `:why` says per form (§ 2), a fact in English, the four
provenance kinds and the fallback ladder, with a state file's lines (§ 3), the four registers and the census's numbers
(§ 4), T3's three decisions (§ 5), and the line between the domain's words and the engine's (§ 6): every text a
program writes is the domain's, and the words around them the engine's, owned by `read_out.md` and
`reserved_engine_strings.md`, which gains a paragraph pointing at it. Linked from `01_grammar.md`'s `:why` paragraph
and its division of labour, from `ir/03-ein-lang/README.md`, from `docs/kernel/README.md`'s user row, and from
`utils/README.md`'s census row; `docs/kernel/README.md` and AGENTS.md count it. T5 pins it to a run.

### Task T1f.5.14.5 — Bank both checks

- `template_wellformedness` in `ein-ir`'s suite: parse each rule's `:why` for
  `{?x}` refs, assert each names a bound variable or a param; each relation's
  refs are positional and within arity. **Runs green today** — 0 of 544, and
  the check exists because `render_why` leaves an unbound ref verbatim, so the
  failure mode is a typo shipping into a user's answer as `{?huse}`.
- `no_placeholder_reaches_an_output` in the corpus sweep: no line of a rendered
  answer, trace or owe list contains `{?`.

Both are cheap and neither is a discovery. Say that in the commit message —
[Q-M1e.1](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)'s
standard cuts both ways, and a probe that comes back clean is still a result.

#### T1f.5.14.5 record, 2026-11-10

**Both checks are banked, and the first was not clean.** The stage file expected *0 findings over 544 templates*;
on 2026-11-10, at `5e4ecb93`, `ein-ir/tests/template_wellformedness.rs` read 893 templates and found 66:

- **58 literals `{N}` in 45 relation `:why`s over 13 files**, `"{1} is a {2}"` where `"{?1} is a {?2}"` was meant: a
  bare `{x}` is a literal (`01_grammar.md`), so each printed as written once T2 gave a relation's `:why` a surface.
  `examples/features/01`, `07`, `10`, `11`, `12`, `examples/ein-bugs/minimal-line-stated-obligation` and its control,
  and under `wrong-answers/` `alt-flag-refuted-by`, `alt-flag-two-derivations`, `guard-only-first`,
  `h-long-relation`, `or-one-key` and its twin. The check flags a literal that names a reference its form could fill
  (a relation's digit, a rule's bound variable), and leaves every other brace literal, as the grammar has it.
- **8 references in one rule's `:why` that name nothing it binds**: `knows-is-mutual` writes `"{?1} knows {?2}, so
  {?2} knows {?1}."` and binds `?a` and `?b`, in `examples/ein-bugs/wrong-answers/uncovered.ein` and its `-knows`
  twin; its `--trace` printed the braces.

Each is fixed in this commit ([Q-M1f.64](../open_questions.md#q-m1f64--existing-whys-that-are-placeholders-or-name-nothing),
defaulted): none moves an answer, so none is a register row. A rule's binding set is its parameters and the variables
of its `:match`'s positive premises with the macros expanded (`forall` and `unknown` are `absent`), and in a top-level
`(or …)` those every disjunct binds; a relation's references are positions within its signature.

**`no_placeholder_reaches_an_output`** (`ein-cli/tests/answer_round_trip.rs`): no line of `answer.txt` or of a state
file's facts carries `{?`, over every corpus program the per-commit tier dumps, and `corpus_cli.rs` holds every
`--trace` run's file to the same. It came back clean once the references were fixed, which is a result too: the
template check holds the cause, this the effect wherever a template is filled.

**And the page is pinned** (T4's): `ein-cli/tests/natural_language_page.rs` holds its § 3 block to the state file
`ein solve examples/zebra.ein --dump-answer` writes, its § 4 registers to the census's and the census's `kernel` to
`ein_core::RESERVED`, and every question it cites to a heading of `open_questions.md`.

**What moved**, named before it was blessed: `corpus_shapes.md5`, 109 digests over the 15 files the fix touched: the
five frontend ops and `load` of each (the template's text), `plan` on 2 (a plan carries its rule's `:why`),
`region-trace` on 8 and `states` on 9 (the sentences now filled). No R, N, H, claim or exit code moved.

### Task T1f.5.14.6 — the loader flag, default false (first)

The user's ruling of 2026-11-08 (§ Status), and the stage's first code task, ahead of T1, so that T1's census can
count per root what the flag would refuse with it on. A flag that makes a missing `:why` a load error, **default
false**, so that nothing moves when it lands: no answer, golden or exit code. **Done 2026-11-08**, ahead of the
phase ([its record](#t1f5146-record-2026-11-08)); what follows is the task as it was specified, and the record says
what was decided.

**What it requires** is not ruled. **Recommended:** a `:why` on every relation, rule, hrule and macro a program
writes, the four forms that take one. `:goal-text` and `:source` stay optional: a query's answer renders its
conjuncts through their relations (§ T3), and only a given has a sentence to quote. A relation nobody declares has
no form to require one on; T2's provenance route covers it, and no flag can.

**Its name and surface** are this task's design; the stage file named neither before the ruling. What constrains
them in the tree:
- **A `(config …)` field** would be a grammar edit: the generated grammar's `Config` takes exactly two flags,
  `:lattice-sanity-check` and `:record-alternative-justifications` (`ein.y`, S1f.5.5's T3), and the user's first
  ruling of 2026-11-08 has that grammar final. A program could also turn its own mandate off, and `SolverConfig` is
  rendered into the KB-shape digest ([S1f.5.15 § T1](s1f.5.15_rule_priority.md#task-t1f5151--the-census-and-the-control-experiment-properly)),
  so a field there would move every digest.
- **An `EIN_*` name**: the binary reads two, `config_reference.rs` pins them, and the repo's `EIN_*` levers have
  been measurement controls, retired once measured.
- **Recommended:** an option of the loader in `ein_ir`'s API (today `ein_ir::load` takes the AST, the terms, the
  forms and a base directory, `from_ir.rs`), and one CLI option on every command that loads a program, so that a
  program without `:why` can still be loaded once the default is true.

Wherever it lands, [`configuration.md`](../../../docs/kernel/configuration.md) gains a row, with *no: it decides
whether a program loads* in its *does it change the answer* column, and `config_reference.rs` pins it.

**Its test:** with the flag on, a program missing a `:why` on each of the forms it requires is refused with one
line naming the form; with it off, the default until T7, every corpus program loads as before.

#### T1f.5.14.6 record, 2026-11-08

Landed as its own commit, T1f.5.14.6's, on the user's rulings of 2026-11-08 (*":why is optional in grammar and
loader has a flag that makes it required, start with default false so not break everything"*; *"yes, the flag only
decides loading"*; *"yes, land the :why flag early"*). The coordinator decided the four open points the same day,
on the agent's report: what the flag requires on the shipped path, the stdlib, the surface and the API.

- **What it requires: a `:why "…"` on every `(relation …)`, `(rule …)` and `(hrule …)`** the load reads. A `:why`
  that is not a string counts as none, as the loader's strings-only guard reads it (the generated grammar refuses
  one at the parse). `:goal-text` and `:source` stay optional, as § T6 recommended.
- **A macro only where its parser reads one.** The shipped hand-written parser refuses a macro's `:why`
  (`parse.rs`' `alt_macro_decl` takes nothing after the body; `(macro mm (?x) (p ?x) :why "…")` is
  *`m.ein:1:23: unexpected input`*, exit 1, `40272dbd`), and the user's re-plan forbids editing it, so a flag that
  required one there would refuse a program for a `:why` it cannot write. The macro arm is under
  `#[cfg(feature = "generated-parser")]`, the feature that routes `ein_ir::parse` through the generated parser,
  whose `Macro::why` is filled: whichever parser a build has, the flag never asks for what that parser cannot read.
  [S1f.5.5's T4](s1f.5.5_the_grammar.md#task-t1f554--the-swap), the swap, deletes the `cfg` and in the same
  commit writes the stdlib's two macros their `:why` (`forall` and `unknown`, `stdlib/macro.ein`, with
  `MANIFEST.sha256`).
- **No stdlib exemption.** The check is uniform over every form the loader reads, imports included, and a `.einb`
  is refused where its `.ein` is (a container re-loads its program from the text it stores). On the shipped path
  the stdlib passes: its 77 rules carry their `:why` and it declares no relation and no hrule. In a
  `generated-parser` build with the flag on, its two macros are refused until T4, and so is every program that
  imports `std.macro` with them.
- **The surface.** `ein_ir::LoadOptions { require_why }`, default false, with `load_with`, `load_query_with` and
  `load_file_with` beside the unchanged `load`, `load_query` and `load_file`, and `ein_einb::OpenOptions::load`.
  On the CLI, `--require-why`, one shared builder in `cmdline.rs` as `--rule-mode` and `--events` are, on the
  seven commands that load a program: `solve`, `oracle`, `test`, `saturate`, `render kb`, `render lattice` and
  `kb save`. Not on `render program`, `rules`, `rule` or `constraints`, which parse and never load, nor on `render
  region`, which loads nothing. The CLI has 78 options where it had 71 (`help_shape.txt`; `config_reference.rs`
  reads the counts off it). The header leaves the option out, as it leaves `-y`, so a program it lets through
  prints the same bytes. Not a `(config …)` field and not an `EIN_*` name, for § T6's reasons.
- **The refusal**, one line per form, `; `-joined with the loader's other refusals in pass order, at the exit code
  the command gives every load error (1, `ein test`'s 2):
  `relation 'p' has no :why "…", which this load requires of every relation, rule and hrule` (`… rule and hrule`
  becomes `… rule, hrule and macro` in a `generated-parser` build).
  [`defined_behaviour.md` § 4.10](../../../docs/kernel/defined_behaviour.md#410-a-form-with-no-why-under---require-why--m1f-t1f5146)
  and [`configuration.md` § 3.4](../../../docs/kernel/configuration.md#34-the-option-that-decides-loading) state it.
- **The tests.** `ein-ir/tests/load_semantics.rs`: `require_why_refuses_each_form_without_one` (each kind refused
  with its line, and loaded without the flag; the lines in pass order beside another refusal),
  `a_macro_needs_a_why_only_where_its_parser_reads_one` (both arms: the gate runs it without the feature, and it
  ran on 2026-11-08 with `cargo test -p ein-ir --features generated-parser --test load_semantics`, which the gate
  does not run), and `require_why_decides_loading_only` (every corpus program: the flag either refuses it with
  `has no :why` lines alone, or loads it to the same KB shape; every stdlib module loads under it). And
  `ein-cli/tests/config_reference.rs`' `require_why_decides_loading_only`: the page's row, the seven commands,
  each command's refusal and exit code on an inline program missing three `:why`s, a `.einb`, and two corpus
  programs that carry every `:why`, one importing the stdlib, printing the same bytes with the flag and without.
  The `ein-ir` sweep's *both outcomes are reached* floor goes when T7 empties the refused side.
- **No answer, golden or exit code moved**, past the option's own seven lines in `help_shape.txt` and the counts
  that read them.

**The migration's size**, what T7 has to write. Lexically, every top-level form under `examples/`, `tests/` and
`corpus/` (2026-11-08, `3998a1c1`): **321 of 407 files** carry a form with no `:why`, **883 relations, 222 rules,
69 hrules and 5 macros**. The scan reads comments, strings and parentheses and nothing of Ein's grammar:

```sh
python3 - examples tests corpus <<'PY'
import pathlib, re, sys, collections
tok = re.compile(r';[^\n]*|"(?:\\.|[^"\\])*"|[()]|[^\s()";]+')
miss, files, n = collections.Counter(), set(), 0
for p in sorted(q for r in sys.argv[1:] for q in pathlib.Path(r).rglob('*.ein')):
    n += 1; d = 0; head = None; why = False
    for t in tok.findall(p.read_text()):
        if t[0] == ';': continue
        if t == '(':
            d += 1
            if d == 1: head, why = None, False
        elif t == ')':
            d -= 1
            if d == 0 and head in ('relation', 'rule', 'hrule', 'macro') and not why:
                miss[head] += 1; files.add(p)
        elif d == 1:
            if head is None: head = t
            elif t == ':why': why = True
print(n, 'files,', len(files), 'with a form missing :why:', dict(miss))
PY
```

By the loader, with the flag (2026-11-08, the T1f.5.14.6 build): of the 407 files, 68 do not load without it
(`examples/broken/` and the parse-error fixtures among them); of the 339 that do, **274 are refused** (241 of
`examples/`'s 331 files, 24 of `tests/`'s 66, 9 of `corpus/`'s 10) and 65 load. The refusals name **806
relations, 198 rules and 68 hrules**, and no macro on the shipped path; a program that imports a module of the
corpus is counted with that module's forms. The stdlib's 7 modules all load. Re-taken with

```sh
for f in $(find examples tests corpus -name '*.ein' | sort); do
  ein kb save "$f" /tmp/x.einb >/dev/null 2>&1 &&
  ein kb save --require-why "$f" /tmp/x.einb 2>&1 >/dev/null
done | sed 's/^kb load error: //; s/; /\n/g' |
  grep -oE "^(relation|rule|hrule|macro) '[^']*' has no :why" | cut -d' ' -f1 | sort | uniq -c
```

### Task T1f.5.14.7 — the flip: default true, and every missing `:why` written (last)

The stage's last task, after T1 to T5 and after every other stage of P1f.5, so that what the others write is in
the final grammar before its `:why` is written. The default turns to true, and a `:why` is written wherever the
corpus (`examples/`, `tests/`, `corpus/`), the stdlib and the Rust suites' inline programs lack one. The stdlib's
two macros, `forall` and `unknown` in `stdlib/macro.ein`, carry none (`grep -n '(macro ' stdlib/*.ein`,
2026-11-08); since T6 the flag asks a macro for its `:why` only once S1f.5.5's swap deletes its `cfg`, and that
commit writes those two and moves `MANIFEST.sha256` ([§ T6 record](#t1f5146-record-2026-11-08)), so they are not
this task's. Its size is T6's census, re-taken by T1. `corpus_shapes.md5` prints `why=`
per rule and relation and moves with what this task writes. Every golden it moves is named here before the commit. T1's census counts what is missing before, and nothing after.

**Each `:why` says what its form says.** None is a placeholder such as *"rule fired"*: that is § T4's argument
for refusing (d), which the user overrode, and it stands as this task's risk.

#### T1f.5.14.7 record, 2026-11-10

**Part 1, every missing `:why` written** (`T1f.5.14.7 — part 1`), with the requirement still off, so the commit moves
only what the texts move. Each `:why` was written by reading its program — its header, its facts, its rules, its
query — by helpers each given a disjoint set of files and the stage's rules (each says what its form says in English;
a relation's positional, a rule's over what it binds; a made-up name said structurally, `"p holds of {?1}"`, and given
no meaning it does not have; never a placeholder; strings natural language only), working on copies; every file they
touched was reviewed here, held to `whycheck` (the T5 check's rules, lexically) and loaded with `--require-why`, and
fixtures with a twin or a copy were given the same words.

| where | relation | rule | hrule | macro | files |
|---|---:|---:|---:|---:|---:|
| `examples/` | 673 | 154 | 58 | 3 | 228 |
| `tests/` | 62 | 10 | 6 | 0 | 24 |
| `corpus/` | 19 | 9 | 1 | 0 | 7 |
| the Rust suites' inline programs | 605 | 245 | 40 | 13 | 43 |
| the doc pages' code blocks | 32 forms | | | | 9 |
| **all** | 1 359 | 418 | 105 | 16 | |

- **Line-locked** ([Q-M1f.57](../open_questions.md#q-m1f57--which-fixtures-get-a-why)): under `examples/ein-bugs/` and
  `examples/broken/`, and every file refused at load, each `:why` sits on its form's last line, so no line or column a
  refusal or a register row prints moves; each refused fixture is refused with the line it was before, and with
  `--require-why` with no other. A file refused at the parse is left (53, 156 forms), and so are the records under
  `plans/` and `docs/history/`. In the Rust suites a program the parse refuses, or one only parsed, is left; a program
  a test expects refused for another reason got its `:why`s, so its refusal stays the only line.
- **Placeholders replaced** (Q-M1f.64): six in the corpus, `"a"`, `"b"` (`broken/load/hrule_duplicate_name`), `"w"`
  (`macro_arity_mismatch`), `"u"` (`unimported_std_macro`), `"forall"` (`syntax/rule-forall-and-not`), `"guarded"`
  (`syntax/rule-guards`); and 74 in the Rust suites' loaded programs, single letters and bare words (`"g"`, `"note"`,
  `"sib"`, `"fn"`, `"first"` …, `naf_semantics.rs` and `explain_semantics.rs` the most), with a relation's `{?a}` that
  should have been `{?1}` (`rule_semantics.rs`). Placeholders in programs nothing loads (the grammar's round-trip
  fixtures) are left. Terse but true ones (`"{?rel} sym"`) are left.
- **Comments made false and fixed**: `zebra2.ein`'s *relations with no `:why` render as their raw IR s-expression*,
  and its four generated variants (`examples/gen_zebra2_variants.py --check`, up to date).
- **Copies made the same**: `corpus/fuzz_findings/{d3-unsat-core,fork-delta-order,unsat-core-id-space,binding-key-layout*}`
  read as their `examples/ein-bugs/` siblings; `naf-nowb` as `naf-upward-closure`; the `kw-*` twins as their witnesses,
  but for the renamed name. The fuzz findings' notes (`corpus/fuzz_findings/*.md`) quote the program the fuzzer
  generated then, and are left as that record.
- **Generators** ([Q-M1f.63](../open_questions.md#q-m1f63--what-each-program-generator-does-about-why)):
  `utils/fuzz_engine.py` writes a `:why` on every form, a rule's its clauses in words (*"fire-3 concludes … from …"*),
  where it wrote *"fire-N fired"*, the placeholder the stage forbids, on 40 % of rules, keeping the random draw so a
  seed generates the program it did, and its minimiser no longer drops a conjunction inside a string;
  `utils/region_synth.py` writes its four relations' (its examples under `examples/lattice/synth/` regenerate
  byte for byte, `--check`); `utils/guard_split.py` its cells' relations, marks and candidate hrules, each cell it
  kept loading with `--require-why`; the Rust suites' generators (`detector.rs`, `compile_limits.rs`' wide rule,
  `match_alloc.rs`' chain, `imports_semantics.rs`' declaration) likewise. `fuzz_properties` keeps its draws.
- **Goldens moved, named before they were blessed** (`EIN_BLESS=1`): `corpus_shapes.md5` (the frontend ops, `load`,
  `plan`, `region-trace` and `states` digests of every migrated file); `zebra2.golden` (four relation lines);
  `fuzz_seeds.txt` (the six corpus fuzz findings' parses); four trace goldens, each *Derives* line of a templated
  relation gaining its statement (`trace_ambiguous`, `trace_one-hypothesis`, `trace_two-level`, `trace_unconditional`);
  one assertion, `explain_semantics.rs`' FIFO test's two parse columns (`4:56` → `4:85`, `5:56` → `5:86`), where a
  `:why` sits before the `:priority` the test refuses; and `model_set_report.rs`' fixture prefix, which read
  `branching/06`'s untemplated declaration. `ein-ir/tests/load_semantics.rs`' corpus sweep, whose floor asked that
  the flag both refuse a program and let one through, now asks that it refuse none, as the T6 record said it would.
  No R, N, H, claim, exit code or register row moved.
- **The census after** (`python3 utils/nl_census.py --slow`, 2026-11-10, on this commit's tree): with a `:why`
  required the loader refuses **none** of the 341 loadable programs. Over the same 211 answered entries and 239 359
  facts, **54.4 % have a statement, 45.6 % a reason, 94.4 % render**, where 0.9 %, 44.7 % and 44.9 % did; 104 of the
  203 entries with a fact render every one, where 20 did, and the mean entry 87.9 %, where 34.6 %. The fallbacks:
  puzzle 12 239 (11 677 of them `type-exclusivity`, a puzzle rule's activator the program states and cannot
  declare), stdlib 555, kernel 33, undeclared 540. **`zebra2`'s state**, 426 facts: **41.5 % statement, 88.5 %
  reason, 97.4 % rendered**, where 5.9 %, 88.5 % and 88.7 %: its eleven fallbacks are the given activators of
  `std.*` rules, `(bijective …)` five times, `(includes …)` twice, `(symmetric next-to)`, `(transitive is-a*)`,
  `(bijection-hierarchy …)` and `(typecheck-hierarchy …)`, which no form can carry a `:why` for (T3).

## Risks

- **A mandate that manufactures prose.** The whole risk of the stage, and the
  mitigation is that acceptance is measured on *rendered output* with a
  register split, not on keyword presence. A relation nobody can describe in
  English is a relation whose s-expression is the honest rendering. Since the
  user's ruling of 2026-11-08 every declared form must carry a `:why` once the
  flag is on by default, so this risk is T1f.5.14.7's to answer, one `:why` at
  a time.
- **The engine/domain boundary is not written down anywhere.** Eight English
  strings live in `ein-render`. If the page draws the line loosely, the next
  stage that adds a verdict word will not know which side it is on. The line
  belongs beside `reserved_engine_strings.md`'s, and probably *in* it.
- **Rendering 444 facts where 8 were rendered.** A per-fact provenance lookup
  on the default `ein solve` path. Measure before shipping; if it costs, the
  full rendering is a flag and the answer table stays as it is.
- **It touches the read-out three phases are already editing.**
  [SE-M1](../../../docs/history/m1e_review_processing/README.md#the-findings) and [AR-M2](../../../docs/history/m1e_review_processing/README.md#the-findings) are
  findings about read-out ownership being split across three crates, and T2
  adds a fourth caller unless it lifts the shared function that
  [S1e.3.4](../../../docs/history/m1e_review_processing/README.md#s1e34--architecture-medium) is already going to want.
  Sequence after it, or hand it the function. Since S1f.5.16 the read-out has one live reference,
  [`read_out.md`](../../../docs/kernel/inference/read_out.md), held to the printer by `read_out_reference.rs`: the
  NL answer table is a row or a form of it, written there in the commit that prints it.

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
- [S1f.5.15](s1f.5.15_rule_priority.md) — the other stage that turns on *which*
  of a fact's ≤ 32 justifications was recorded first.
