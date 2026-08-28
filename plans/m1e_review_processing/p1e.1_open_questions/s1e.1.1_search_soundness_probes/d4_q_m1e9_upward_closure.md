# D4 — Q-M1e.9 reproduced: `dead` is **not** upward-closed under `absent`

> **This one is no longer a question.** It was filed as *two kernel pages
> appear to disagree*; the probe was run on 2026-08-28 against `7731848` and
> the disagreement is real. What is left to decide is **who owns the fix**, not
> whether there is one.

**Probe:** [`probes/naf_upward_closure.ein`](probes/naf_upward_closure.ein)
**Bears on:** the no-good store, the singleton writeback and the lookahead
kill cache — all three **ship**, and all three read the same premise.

## The premise, as the repo states it

[design/08 § The objects](../../../../docs/history/m1a_rust/design/08_parallelism.md):

> `dead(X)` — `X` holds a contradiction. **Monotone**: `X ⊆ Y ∧ dead(X) ⇒
> dead(Y)`, because the KB is append-only and nothing retracts.

and its claim (1):

> **A writeback prunes; it does not decide.** If `sat(B ∪ {h})` is dead then
> for every `c ⊇ {h}`, `sat(B ∪ c) ⊇ sat(B ∪ {h})` by monotonicity, so `c`
> dies whether or not `¬h` is at root.

Against [`absent_semantics.md` C3](../../../../docs/kernel/inference/absent_semantics.md):

> Removing a fact can flip an absent and **fabricate** a contradiction the
> full KB never had.

*Append-only, nothing retracts* establishes that `sat` is **inflationary**.
It does not establish that `sat` is **monotone in its input**, and `absent` is
exactly what separates the two.

## The probe

Twenty lines. `bad` fires only while `q` is missing:

```lisp
(relation is-a T T)
(relation p T)
(relation q T)

(is-a A T)

(rule bad ()
  :match  (and (p ?x) (absent (q ?x)))
  :assert (false)
  :why    "{?x} has p without q"
  :priority 200)

(query :goal (p ?x))
```

By hand, under [Q-M1e.6](../../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model):

- `alive₀ = {(p A), (q A)}`
- `{(p A)}` is **dead** — `bad` fires
- `{(p A), (q A)}` is **alive** — the guard's witness is present
- so `dead` is not upward-closed, and the single solution is `{(p A), (q A)}`
  — maximal, nothing left to add.

## What the engine answers

Every run below is `-e`, and **every one reports `k = 1` (or 0) with
`exhausted = true`**. Nothing in the read-out signals a disagreement; only the
recorded fact set differs, which is [Q-M1e.7](../../open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)'s
point arriving as a diagnostic.

| configuration | enterings | recorded | verdict |
|---|---:|---|---|
| **default** | 0 | `(q A)`, `(not (p A))` | **wrong** |
| `-K` | 0 | `(p A)`, `(q A)` | right |
| `-L` | 2 | `(q A)`, `(not (p A))` | **wrong** |
| `-L -K` | 2 | `(q A)`, `(not (p A))` | **wrong** |
| `(config :enable-lookahead-kill-cache false)` | 0 | `(p A)`, `(q A)` | right |
| `(config :enable-singleton-writeback false)` | 0 | `(q A)`, `(not (p A))` | **wrong** |
| `(config :enable-singleton-writeback false)` + `-L` | 2 | — | **`k = 0`, Contradiction** |

## Three consumers, one false premise

The matrix separates them, and each is sufficient on its own:

1. **The lookahead kill cache** — the default's route, at **zero** enterings.
   `(p A)` dies in one firing against root-without-`q`, and `write_negated`
   caches that as a `(not (p A))` fact with provenance
   `<lookahead-dies-immediately>` and **no premises**
   ([`hypgen.rs:464`](../../../../ein.rs/crates/ein-infer/src/hypgen.rs)). The
   forced-positive cascade then promotes `(q A)` into root — and the cached
   negative, whose justification no longer holds, is never revisited. This is
   [C6](../../../../docs/kernel/inference/absent_semantics.md) — *"an `absent`
   query answered in one world is meaningless in every other"* — violated by a
   cache.
2. **The singleton writeback** — the `-L` route. `{(p A)}` is entered, dies,
   and `handle_dead` writes `(not (p A))` at root because `|c| == 1`. Same
   permanent negative, reached by forking rather than by simulating. This is
   design/08 claim (1) failing on its own terms.
3. **The no-good store and apriori's filter** — the last row. With both writes
   off, `{(p A)}`'s death still emits the width-1 clause `¬(p A)`, which
   subsumes `{(p A), (q A)}`, so the real solution is **never generated** and
   `k = 0`.

`-K` gets the right answer **by accident**, not by being correct: the
lookahead filters `(p A)` out of `alive` without recording it, the cascade
promotes `(q A)`, and the recomputation *after* the promotion finds `(p A)` no
longer doomed. The cascade repairs what the filter did.

## The one existing hazard signal does not fire

`(config :warn-derived-naf true)` emits nothing on this program.
`derived_naf_warnings` watches an `(absent …)` guard over a **rule-derived**
relation; `q` here is only ever proposed by the hypothesis generator, which is
a different origin. So the engine has a warning for the adjacent hazard and
none for this one.

## Why the corpus never saw it

The shape needs an `(absent …)` guard whose watched relation is
**hypothesis-eligible**. The corpus's `(false)`-deriving rules are the
algebraic scans — `functional`, `injective`, `total`, `no-room-left` — whose
guards read the relation's *extent*, not its absence at a point; and the
stdlib's `absent` guards read `is-a`-shaped membership, which no generator
proposes. One program outside that pattern is enough, and this is it.

## What has to be decided

Not *whether* — *who*, and *how far*.

| | scope | consequence |
|---|---|---|
| **A — record and hand on** | Q-M1e.9 becomes a finding with the probe banked as a `broken/`-style fixture; owner is a new milestone | honest, cheap, and leaves three shipped mechanisms unsound on a shape a user can write today |
| **B — narrow the claim, keep the machinery** | state that the engine's search is sound **only for programs whose `(false)` derivations do not pass an `absent` guard over a hypothesis-eligible relation**, enforce it with a **load-time check** (the compiler already knows every guard's watched relations and every hypothesis-eligible relation), and refuse or warn | the smallest change that makes the tree honest. Turns a silent wrong answer into a diagnostic, which is the repo's usual move |
| **C — fix the machinery** | make the three consumers world-aware: no kill-cache write for a lookahead whose firing used an `absent`; no singleton writeback for such a death; a no-good clause tagged with its `absent` premises and not applied where they no longer hold | correct, and large. `Prov::absent` already **records** the negative premises (C2, S1.21.8) — *"the dependence is visible … but no walk yet interprets it"*. This would be the first walk that does |
| **D — declare it out of scope for M1e** | it is not one of the 63 findings, and M1e processes a review | defensible on scope, and it means the milestone found a soundness defect and shipped without saying what happens to it |

**Recommended: B now, C filed.** B is a load-time check the compiler has the
information for, it converts a wrong answer into a refusal, and it does not
require reshaping the no-good store mid-milestone. C is the real fix and
wants its own stage, with `Prov::absent` as its starting point.

## Related

- [D3](d3_q_m1e8_file_or_take.md) — the maximality test would read the same
  layer results, so it inherits this premise. It does not *add* the exposure.
- [D9](d9_kernel_page_overclaims.md) — the kernel page says *the engine never
  records a false model*. Under this probe the default configuration records
  `(q A)` and calls it the model, which is a **non-maximal** state, so the
  claim needs the same qualification for a second reason.
- [S1e.1.6](../s1e.1.6_coverage_gaps.md) — Q9's unswept surface is *no
  dedicated pass over algorithmic pathology or invariants*. This is what one
  hour of that pass found.
