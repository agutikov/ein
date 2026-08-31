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

---

## ✅ Done 2026-09-01 — four comments, and two of them were wrong about ein.py rather than about the code

`MA-L5` was closed at S1e.2.1, as this stage's *Depends on* line said it would
be. The other four, and **three of them are larger or different than reported**.

### `MA-L1` — fixed, and the review's sub-claim is refuted

The direction is settled by the comparator, not by reading: the agenda is a
min-heap on `(priority, tiebreaker)` and the parked set a `BTreeSet` walked
ascending, so **lower fires earlier** and 1000 fires **last**. Confirmed
empirically — a four-rule program declared 1001 / 900 / 999 / none fires
`hyp, lo, unbanded, hi`.

Three things the review did not have:

- **The 900 band exists.** It says *"no 900 band exists"*; three
  `hypothesis-contradiction` rules under `examples/saturation/` declare it,
  continuously since `d94b7d9`. Only the **stdlib** stops at 500 — a fix
  written from the review's text would have put a second false claim at the
  site.
- **The comment is a verbatim port of `ein.py`'s**, whose `heapq` was also a
  min-heap. The defect predates the port.
- **A second consumer**: `obligations::priority_of` returns this constant to
  order the **report** of what a state owes, which nothing had written down.

**The value is not merely unpinned, it is unobservable**: 36 corpus rules carry
no `:priority` and not one shares a file with a banded rule, so every golden
and digest is identical for any default above 900 — which is how the comment
drifted for a milestone. `explain_semantics::the_default_priority_is_1000_and_fires_last`
**sandwiches** it between 999 and 1001, so it fails for any change to the
number rather than merely to the band.

**A cross-plan disagreement, resolved.** M1f's
[S1f.5.6](../../m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/s1f.5.6_rule_priority.md)
claimed `MA-L1` as *"fixed by removal, and S1e.4.8 is told so"*. Under
`standard_of_proof.md` Rule 2 the premise *the constant is leaving* is enforced
by nothing — that stage may end in a written refusal and has not started — and
a deferral needs a note **at the site** anyway, which costs the same edit as
the truth. M1f's line is corrected rather than left false.

### `MA-L2` — fixed and pinned, and **smaller** than reported

22 literal spaces, and the arithmetic confirms the reflow story: the sibling
`Ambiguity` arm continues at 21 spaces of indent, and 1 + 21 = 22.

But *"it reaches output through `render_answer` on every non-exhausted `k = 0`
run"* is **false**: `ein-cli` never calls `render_answer`. Every CLI `k = 0`
run renders `render_solution_table`, whose Contradiction arm is the untouched
single-spaced wording. Where it lands is the `corpus_shapes.md5` digest on
**31** entries, and the `ein-render` public API — which `docs/api/rust.md`
documents as the *explain* crate, so it is a surface an embedder is pointed at.
That correction is written here because a reader who believes the finding will
look for the spaces in `ein solve` output and not find them.

The pinning gap is exactly as reported and is the finding's real content:
**nothing asserted either `Contradiction` headline.**
`a_truncated_contradiction_headline_is_one_sentence` sits beside
`an_unexhausted_ambiguity_says_the_count_is_a_lower_bound` — the `k = 0` and
`k > 1` counterparts of the same claim, in one file, in one style.

**The re-bless, predicted and then measured: 31 of 9 015, all `trace[answer]`,
no line count changed.** That equality is the check that the substitution is
intra-line.

**And the sibling sweep is not empty.** Every string literal in `ein.rs/crates`
with a run of 3+ spaces between two non-space characters: 35 sites, all but one
deliberate column alignment (`printers.rs`, `saturate.rs`, `hypgen.rs`,
`shape.rs`). The one that is prose is `saturator.rs`'s
`the_resumed_run_narrated…` assertion message — **14 spaces**, the same lost
continuation, in text only a failing test prints. Fixed with it.

### `MA-L3` — accepted, and the question **is** settleable — from git, not from the goldens

T1e.4.8.3 predicted the answer might be *we cannot know*, because neither
`from_ein_py/` directory holds a summary golden. It does not — and the evidence
was never the goldens. `git show 4c1a5b3^:ein.py/src/ein/cli/_summary.py` line
208 is literally `json.dumps(summary, indent=2, ensure_ascii=False)`.

So the review's disjunction — *"one of the two is wrong about the parity
target"* — is **false**. Both are wrong, differently, and only one was
reported:

| comment | wrong about |
|---|---|
| `summary.rs`'s `write()` | **the code it documents** — `dumps_indent` escapes everything non-ASCII. It is a *true* statement about ein.py |
| `dump/json.rs`'s *"no caller overrides it"* | **ein.py** — two call sites did, and this port implements one of them |

**The port knew what the flag meant, implemented it once, and missed it once**:
`ein-infer`'s event writer reproduces `_events.py`'s override exactly, so the
*same run* emits `—` in `--json-summary` and a literal em dash in
`--events` — verified on `tests/stdlib/slots/09_owed_room.ein`. A latent parity
bug that went live at M1d S1d.2.4, four days after the thing it diverged from
was deleted.

**Accepted**, because there is nothing left to be byte-identical to, every
reader in the tree parses the summary (so the two encodings give identical
values), and an ASCII-only artefact survives a pipeline and a locale that an em
dash does not. The premise the acceptance rests on is what
`cli_semantics::the_summary_escapes_non_ascii_where_the_event_stream_does_not`
holds, asserting **encoding** and not wording. The alternative, and its one
trap — a blanket change to the writer would move `--dump-states` and
`--json-report`, whose CPython default is *correct* — is named at the site.

### `MA-L4` — split: the memo is fixed, the **cause** is refuted, and the documented cost was wrong

- **The memo: fixed.** A plan is a pure function of `(ast, rule, activator)`,
  the direct path six lines up already shares it, and there is no design
  statement anywhere for the fresh one — a threading miss. Measured on
  `zebra2 -e -y`: **66.9 → 63.1 ms**.
- **"…polluting the live event stream": refuted as to cause, and the
  refutation is at the site.** The `compile` event fires on an **engine** miss,
  never a memo miss — `engine.rs` says so in a comment three lines above the
  emitter — so sharing the memo removes **zero** events. The stage's own
  instruction (*"pass the run's shared memo and both go away"*) is false on its
  second half, and a reader who is not told will re-file this finding.
- **What `-y` does to the stream is real, and it is four kinds, not one.**
  Measured on `examples/branching/04_two_levels.ein -e`: `compile` 96 → 336,
  `enqueue` 168 → 1 296, `fire` 87 → 143, `quiesce` 21 → 81. `fire` is the one
  that matters — derivation lines no entering produced. It is inherent to
  running the extra saturations through a narrating `Events`, so it is a
  **documented property of the flag**, stated where `-y` is documented.
- **NEW — the cost is wrong in three places.** `k+1` is inherited from ein.py's
  docstring, which counted the direct path and each parent's
  `try_commitment_set` and forgot each *alive* parent's re-saturation. The code
  says `1 + k + (alive parents)` — up to **`2k+1`**, five per size-2
  commitment where all three documents said three. Corrected in `sanity.rs`,
  in `--help` (one `help_shape.txt` line, blessed) and in `configuration.md`.
  It is a `MA-L1` sibling: a number wrong in the direction that matters, on the
  one flag whose whole documented justification is its cost.

### One defect found on the way

`events_reference.rs` banked a **file:line** — `saturator.rs:1586` — for the
single `emit` whose kind is not a literal, and `MA-L1`'s doc comment moved it
to :1623 thirty lines earlier in the file. A test failing for a reason
unrelated to what it checks. It banks the **file** now, which is what the check
actually needs: *there is exactly one, and it is the known one*.

**Gate:** `./run_tests.sh` — exit 0, **821 tests**. Two goldens moved, both
named in advance: 31 `trace[answer]` digests (`MA-L2`) and one `help_shape.txt`
line (`MA-L4`).
