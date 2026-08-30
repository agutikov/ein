# S1e.4.8 — Maintainability (Low)

**Phase:** [P1e.4](README.md) (Low)
**Estimate:** 1.5 days
**Depends on:** [CO-H2](../p1e.2_high/s1e.2.1_correctness.md) for `MA-L5` —
the comment goes when the duplication it explains goes.
**Findings:** [`MA-L1`](../review/maintainability/low.md) …
[`MA-L5`](../review/maintainability/low.md).

## Context

Five findings, four of them comments that are wrong and one that is a string
with a lost line-continuation. Grouped, they are the tail of the same class as
[MA-M1..M4](../p1e.3_medium/s1e.3.9_maintainability.md): *what is written next
to the code contradicts the code*.

**`MA-L1` — `DEFAULT_PRIORITY`'s arithmetic.**
[`saturator.rs:42-45`](../../../ein.rs/crates/ein-infer/src/saturator.rs)
says *"Rules with no `:priority` sit between the eliminate band (300) and the
hypothesis band (900)"* — for a constant of **1000**, which sorts after both;
and the stdlib's real bands span **90–500**, with no 900 band at all. The only
in-code statement of where undeclared-priority rules schedule is wrong **in
the direction that matters**: they fire last, not mid-band.

**`MA-L2` — ~22 literal spaces in a headline.**
[`answer.rs:232`](../../../ein.rs/crates/ein-render/src/answer.rs), against
the single-space wording at `:507`. Every sibling string in the file uses `\`
continuations, so this looks like a lost line-continuation from a reflow. It
reaches output through `render_answer` on **every non-exhausted `k = 0` run**,
and **no test or golden pins it** — the corpus banks an md5 digest, which
cannot reveal an oddity, only a change.

**`MA-L3` — `write()`'s doc comment against the JSON writer.**
[`summary.rs:634-638`](../../../ein.rs/crates/ein-cli/src/summary.rs) claims
`json.dumps(summary, indent=2, ensure_ascii=False)`; the writer escapes all
non-ASCII and its own test asserts *"`ensure_ascii=True` is CPython's default
and no caller overrides it"*
([`dump/json.rs:153-184, 219-225`](../../../ein.rs/crates/ein-render/src/dump/json.rs)).
One of the two is wrong about the parity target — and if `ein.py` really did
pass `ensure_ascii=False` for the summary, the ported bytes differ for any
non-ASCII `:why` content.

**`MA-L4` — `sanity -y` pollutes the event stream.**
[`sanity.rs:137-152`](../../../ein.rs/crates/ein-infer/src/sanity.rs)'s
`check_commutativity` builds the parent-path `Session` with
`SharedMemo::default()` while the direct path uses the run's shared memo, so
every checked commitment recompiles all plans per parent — a cost — and the
recompiles narrate `compile` events into the run's **live stream**, so a `-y`
run's stream differs from a plain run's by more than the check itself.

**`MA-L5` — a comment predicting a refactor that never happened.** ✅ **Done
2026-08-29 by [S1e.2.1](../p1e.2_high/s1e.2.1_correctness.md) T2**, which is
what this stage's *Depends on* line said would happen: the comment went with
the duplication it explained. Nothing is left for this stage to do — the
`RESERVED_NAMES` array and its four-line prediction are deleted, `qualify()`
filters against `ein_core::is_reserved`, and that function's own doc comment
now records what the drift *was* rather than what P1a.3 was going to do about
it. The finding as reported:

> [`imports.rs:42-51`](../../../ein.rs/crates/ein-ir/src/imports.rs): *"P1a.3
> brings the registries over and this becomes a query against them"* — it never
> did, and the hand copy below it is the one that missed `open`. The comment now
> actively misleads: it explains why the duplication is temporary, and it is
> not.

## Acceptance

- No comment among the five states something the code contradicts.
- `MA-L2`'s headline is fixed **and pinned** — the reason it survived is that
  nothing pins the string, so fixing it without a test leaves the same hole.
- `MA-L3` is settled against evidence (the goldens from `ein.py`), not against
  whichever comment reads more confidently.
- `MA-L5` is deleted as part of, or immediately after,
  [CO-H2](../p1e.2_high/s1e.2.1_correctness.md)'s unification.

## Tasks

### Task T1e.4.8.1 — `MA-L1`: state the real bands

Read the stdlib's actual priorities, write the real range (90–500 as observed,
confirmed by a grep rather than by the review), and say plainly what 1000
means: **undeclared-priority rules fire last**. That is the operationally
important fact and it is the one the current comment denies.

Worth one extra sentence while there: whether *firing last* is the intended
default. It is a defensible choice (a rule nobody prioritised should not
preempt one somebody did) and it is currently stated nowhere, which is how the
comment came to describe a mid-band that does not exist.

### Task T1e.4.8.2 — `MA-L2`: fix the string, then pin it

Fix the spacing to match `:507`'s wording. Then add the pin, because its
absence is the finding's real content: the corpus banks an md5 of rendered
output, which detects a change but cannot show a reader that the current bytes
are odd. A short assertion on the rendered headline for one non-exhausted
`k = 0` entry — of which
[`saturation/type-exclusivity/pets.ein`](../../../examples/saturation/type-exclusivity/pets.ein)
is the standing example — is enough.

Check the digest goldens before and after: this **does** change rendered
output, so it is a named re-bless, and it is the only one this phase expects.

### Task T1e.4.8.3 — `MA-L3`: settle the `ensure_ascii` question

The evidence is in the tree, in two places:
[`ein-render/tests/golden/from_ein_py/`](../../../ein.rs/crates/ein-render/tests/golden/from_ein_py/README.md)
and
[`ein-ir/tests/golden/from_ein_py/`](../../../ein.rs/crates/ein-ir/tests/golden/from_ein_py/README.md)
— the last independent provenance the repo has. Look for banked output with
non-ASCII content — a `:why` template with a non-ASCII character is the likely
carrier — and read whether it is escaped. That settles which comment is
wrong. Note that neither directory holds a *summary* golden (`.dot`, a trace,
and two KB dumps), so the answer may be that `ein.py`'s summary bytes were
never banked at all.

If no such golden exists, the question cannot be settled from evidence and the
honest fix is to say so: the comment states the *current* behaviour
(`ensure_ascii=True`, escaped) and notes that the parity target for non-ASCII
summary content was never exercised. That is a better comment than either of
today's, and it hands the next person the actual state of knowledge.

### Task T1e.4.8.4 — `MA-L4`: don't narrate the sanity check's recompiles

Two effects, one cause. Pass the run's shared memo to the parent-path
`Session` and both go away: the recompiles stop (cost) and the `compile`
events stop appearing in the live stream (observability). Check first that the
shared memo is *correct* here — the fresh memo may have been chosen to keep
the commutativity check independent of the run's compile state, which would be
a real reason — and if it was, say so at the site and instead suppress the
events for the check's sub-session.

Either way `-y`'s effect on the event stream should be stated where `-y` is
documented: a diagnostic flag that changes the stream by more than its own
check is a surprise for anyone diffing streams, which is a thing this repo
does routinely.

### Task T1e.4.8.5 — `MA-L5`: delete the prediction

When [CO-H2](../p1e.2_high/s1e.2.1_correctness.md) unifies the reserved-name
lists, `imports.rs:42-51` loses its subject. Delete the whole comment block,
including the *"P1a.3 brings the registries over"* sentence — a prediction
that did not come true is worse than no comment, because it tells the next
reader the duplication is being handled.

If any part of it is still true after the unification (the note about which
names are SYMBOL-excluded, for instance), keep that part and only that part,
and check it against
[SE-L2](s1e.4.2_semantics.md)'s renaming so the two do not disagree the day
after they are both fixed.

## Notes

`MA-L2` is the one to do carefully and `MA-L3` is the one that may end in *we
cannot know*. The other three are edits. If the phase is compressed, `MA-L1`,
`MA-L4` and `MA-L5` are one commit and half an hour; `MA-L2` needs the
re-bless named; `MA-L3` needs the golden hunt or the honest non-answer.
