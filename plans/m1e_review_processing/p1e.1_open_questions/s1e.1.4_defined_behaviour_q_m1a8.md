# S1e.1.4 — Q-M1a.8's real trigger: Q3

**Phase:** [P1e.1](README.md) (The ten questions)
**Estimate:** 1 day
**Depends on:** [S1e.1.1](s1e.1.1_search_soundness_probes/README.md) T1 — this stage
is the standard of proof's first real customer, because its likely outcome is
a **refutation** of a page the tree calls normative.
**Blocks:** [CD-H3](../p1e.2_high/s1e.2.2_code_doc_consistency.md) — the fix
there is a doc correction, an engine bug or a closure, and this stage decides
which.
**Answers:** [`review/open-questions.md`](../review/open-questions.md) Q3.

## Context

[`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md) exists
because `ein.py` was deleted: it is what *"whatever ein.py did"* used to
define, promoted to a normative page and checked by `cargo test`. §3.2 is the
one item the page itself flags as **a latent bug rather than a quirk**, filed
as `Q-M1a.8` and cited from the README's Known gaps. It promises that two
activators differing only in an integer argument can suppress each other's
firings, silently.

The review probed it directly — two activators `(walk edge 1)` and
`(walk edge 2)` — and got **both** firings, because `BindingKey.values`
includes the int-seeded register. So one of two things is true:

- the engine does not have the documented bug, and the page's one
  self-declared latent bug is **false**; or
- the collision exists in a narrower shape — nested-`Fact` activator
  arguments, which bind nothing and so stay out of both halves of the key —
  and every statement of it names the **wrong trigger**: *"integer
  argument"*, *"a puzzle whose rule parameters are integers can lose a
  firing"*.

Either way nothing pins the claimed suppression, in either direction, and
anyone triaging `Q-M1a.8` starts from a recipe that does not reproduce. There
is a second reason to care: `refresh_collision_risk`
([`saturator.rs:1182-1204`](../../../ein.rs/crates/ein-infer/src/saturator.rs))
spends real conservatism on the same asymmetry, so if the asymmetry is not
what §3.2 says it is, the cost is being paid against a mis-stated model.

## Acceptance

- **Both probe shapes are executed and banked as tests** — the int-argument
  shape (expected: both fire) and the nested-`Fact`-argument shape (expected:
  unknown, and that is the point). A non-reproduction that is not banked is
  not an answer.
- §3.2 is either **amended to the real trigger** with the reproducing program
  quoted, or **deleted** with the reason recorded, and `Q-M1a.8` is closed in
  the [M1a ledger](../../../docs/history/m1a_rust/open_questions.md) with a
  date and the probe's name.
- The README's Known gaps entry moves with it. Two pages state this claim and
  they do not get to disagree afterwards.
- If the nested-`Fact` shape *does* reproduce, the item stays a latent bug,
  §3.2 is rewritten to the shape that reproduces, and the fix — if any — is
  filed, not taken here. The page's job is to state behaviour correctly; it
  is not this stage's job to change the behaviour.

## Tasks

### Task T1e.1.4.1 — Read the key, then predict

Before running anything, write down what the two halves of the binding key
contain, from
[`firing.rs:219-224, 242-249`](../../../ein.rs/crates/ein-infer/src/firing.rs)
and [`compile.rs:94-101, 440-453`](../../../ein.rs/crates/ein-infer/src/compile.rs):
which activator argument kinds seed a register and therefore reach
`BindingKey.values`, and which bind nothing and therefore reach neither half.
The prediction is cheap and it is what makes the probes decisive rather than
exploratory — a probe designed after the answer is known proves less.

The expected reading, to be confirmed or corrected: a **symbol** or **int**
argument seeds a register and lands in `values`; a **nested `Fact`** pattern
in activator position binds nothing, so two activators differing only there
produce equal keys. If that is right, §3.2's mechanism is real and its
*trigger* is misnamed.

### Task T1e.1.4.2 — Two probes, both banked

1. **Int arguments** — the review's shape, re-run and turned into a test:
   two activators differing only in an int argument, both must fire, asserted
   on the `fire` event stream rather than on a rendered table (the stream is
   where the firing is, and `--events` is already the instrument
   `stdlib_census.py` uses).
2. **Nested-`Fact` arguments** — two activators differing only in a nested
   fact pattern. Assert what the reading predicts; if the prediction is
   wrong, the test records the real behaviour and the reading in T1e.1.4.1
   gets corrected, not the test.

Both go under `ein-infer/tests/` beside the firing tests, named for the
claim (`activators_differing_only_by_int_argument_both_fire`), not for the
question id — a test named `q_m1a8` is unreadable the day the question
closes.

### Task T1e.1.4.3 — Amend or delete §3.2, and close Q-M1a.8

Then the paperwork, which is the point of the stage:

| probe result | §3.2 becomes | `Q-M1a.8` |
|---|---|---|
| int fires both, nested collides | rewritten: the trigger is a nested-`Fact` activator argument, with the program that shows it | stays open, correctly stated, with a fixture |
| int fires both, nested also fires both | **deleted** — the page has no latent bug; the two probes become the tests that say so | closed as **not a bug**, dated, naming both probes |
| int collides (the review's probe was wrong) | unchanged in substance; the probe that failed to reproduce is recorded with why | stays open, and the review's finding is `refuted` |

Whichever row lands, `defined_behaviour.md`'s own count of what it states
moves with it, and the README's Known gaps sentence is edited in the same
commit. The milestone has a whole finding about counts that drift because two
copies of one claim were not edited together
([DO-M1](../README.md#the-findings)); this is the first chance not to do it
again.

## Notes

The third row is the one worth keeping in mind. The review reproduced its
probe against the release binary and the reproduction is credible — but this
milestone's rule cuts both ways, and a finding that a page is wrong is itself
a claim until a banked probe holds it. Running the int shape again is fifteen
minutes and it is the difference between *the review said so* and *the tree
says so*.
