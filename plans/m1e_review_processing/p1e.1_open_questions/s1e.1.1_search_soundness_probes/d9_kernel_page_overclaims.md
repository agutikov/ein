# D9 — Taken, and the row was not qualified but **withdrawn**

> **Done 2026-08-28.** The probe this file was waiting for ran, so the choice
> below is no longer between *qualify now* and *let the probe edit it*: the
> probe edited it. § 6's first table now carries **three** rows, the
> `complete(S) ⇒ solution(S)` direction reads **no**, and the sentence *"so the
> engine never records a false model"* is gone from
> [`docs/kernel/inference/solution_semantics.md`](../../../../docs/kernel/inference/solution_semantics.md)
> and named in its place as withdrawn, with the date. § 6 also gained a section
> the options below did not anticipate — **What is recorded is `S ∪ K`** — and
> § 6's closing premise now says C3 **wins**.
>
> The same claim was repeated inside
> [Q-M1e.6](../../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)'s
> own § 2 table, decided by the user on the same page; that copy is corrected
> too, with a ⚠ marking what it used to say.

**Touches:** [`docs/kernel/inference/solution_semantics.md`](../../../../docs/kernel/inference/solution_semantics.md)
§6, committed 2026-08-28 in `429777f` and rewritten the same day.

## The claim as written

§6's first table:

| | holds? | consequence |
|---|---|---|
| `complete(S) ⇒ solution(S)` | **yes** | every filter that can make `complete` true is a genuine refutation, so **the engine never records a false model** |
| `solution(S) ⇒ complete(S)` | **no** | a remaining hypothesis that needs *two* firings to die is still proposed, so a real solution goes unrecorded |

## Why the first row is not established

`solution(S)` is a **conjunction**:

```
solution(S) ≡ S saturated ∧ S consistent ∧ ( owes nothing ∨ maximal )
```

The argument given — *every filter that can make `complete` true is a genuine
refutation* — establishes the **maximality** conjunct and says nothing about
**consistency**. The page silently assumes consistency was established
elsewhere.

For a forked state that is fine: `try_commitment_set` returns `Alive` only
after its post-saturation detect came back empty, and `finish_entering` asks
`complete` on exactly that unmutated fork. For **root**, it is fine on
`phase1`'s path (`solve.rs:1091` checks, then `:1114` records) and on the
cascade's (`:2131`).

It is **not** established on the inter-layer path — `:1544` tests
`alive.is_empty()` and `:1550` records, with no `has_contradiction` between
them. Note that `alive == ∅` *is* `complete(root)`: both call the same
generator with the same filters. So the one place the engine records a state
whose consistency was not re-established is exactly the site
[D1](d1_q4_which_route_reaches_the_site.md) is about — and if Q4 lands
**fixed**, §6's first row is wrong as printed.

[D4](d4_q_m1e9_upward_closure.md) supplies a second reason independently: at
default configuration the probe's recorded state is `{(q A), (not (p A))}`,
which is **not maximal** — `(p A)` is consistent with `{(q A)}` — and it
passes `complete` only because the kill cache manufactured the negative that
makes it look maximal. So the row's *maximality* half needs a qualification
too, for a different mechanism.

## Why it matters more than a wording nit

`CLAUDE.md` says of this tree:

> **This tree is now the only statement of intent that is not also the
> implementation**, so it is load-bearing: a claim here is checked by
> `cargo test --workspace` and by nothing else.

An unqualified soundness claim that a scheduled probe may refute is precisely
[CD-H1](../../README.md#the-findings)'s defect — a kernel page asserting
something about an engine that may not be true — committed by the milestone
that exists to fix it.

## Options

| | what happens | consequence |
|---|---|---|
| **A — qualify now** | rewrite the row to state the conjunct it proves, and name the two open exposures with forward references to Q4 and Q-M1e.9 | one paragraph. The page becomes honest about what it does and does not know, which is the same shape §6 already has for `exhausted` |
| **B — let the probe edit it** | leave it; T1e.1.1.2 and D4's disposition rewrite it when they land | the page is wrong in the interval, and the interval is however long P1e.1 takes |
| **C — drop the row** | say only that the engine under-reports, and delete the soundness direction | loses real information — the direction *is* what makes the under-reporting safe, and a reader needs it |

**Taken: A, in the strong form.** What was proposed here — keep the row and
attach *"two exposures qualify this"* — was written when both exposures were
**forward references to probes that had not run**. They ran. `complete ⇒
solution` is not a qualified *yes*; it is a **no**, with three witnesses at
three of `record_node`'s four callers, and the honest edit was to split the row
into the conjunct that holds and the implication that does not.

The proposal as written, kept for the record:

> | `complete(S) ⇒ S is maximal` | **yes** | every filter that can make `complete` true is a genuine refutation of the candidate it dropped — so a recorded state has no live child it knows of, and the engine does not manufacture models the search did not reach. **Two exposures qualify this**: the kill cache can write a negative whose `absent` justification a later promotion invalidates ([Q-M1e.9](../../open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent)), and the inter-layer path records root without re-establishing consistency ([CO-M1](../../README.md#the-findings), open) |
> | `S is a solution ⇒ complete(S)` | **no** | a remaining hypothesis that needs *two* firings to die is still proposed, so a real solution goes unrecorded |

and delete the sentence *"so the engine never records a false model"*, which
is the part that is not yet earned.

## What the page says now

Three things, none of which existed when this file was written:

1. **The table has three rows.** `complete ⇒ S is maximal` **yes**;
   `solution ⇒ complete` **no** (the under-report); `complete ⇒ solution`
   **no**, with the withdrawn sentence named and dated so a reader who
   remembers it knows it went.
2. **A new section — *What is recorded is `S ∪ K`, and the criteria were
   checked against `S`*.** The two writers that run after the last consistency
   check (the singleton writeback, the kill cache), the four `record_node`
   callers and what each establishes, the three witnesses, and the pointer to
   [Q-M1e.7](../../open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)
   for which fix follows.
3. **§ 6's closing premise says C3 wins.** *"That premise is not new here"* now
   continues with the twenty-line program that breaks it, the three mechanisms
   that read it, and the qualification the maximality arm actually needs:
   *every program whose refutations do not pass through an `(absent …)` over a
   relation the search can still extend* — which is stated nowhere else in the
   engine, which is why it is there.

## Ordering — as it happened

This was a **doc** change to a page committed on this branch; it touched no
engine code and waited on nothing. It went in *after* the probes rather than
before, which is the better order and not the one this file planned: the page
now states what was measured instead of what was expected.
