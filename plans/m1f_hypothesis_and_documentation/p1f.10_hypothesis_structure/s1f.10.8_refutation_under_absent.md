# S1f.10.8 — ~~May a refutation rest on an `absent`?~~

**Phase:** [P1f.10](README.md)
**Status:** **closed.** Ruled and built in M1e2; recorded as closed at the re-plan of 2026-11-15. Nothing of it
is left for this phase to run.

## What it asked

Planned 2026-08-28 on the user's note of that day (*"maybe emitting `(false)` or `(not …)` under `absent` is
not a good idea"*): whether a rule may conclude `(false)` or a stored negative from an `(absent P)` over a
relation the search can still extend. It was this phase's because an exclusion computed on a smaller KB and
spent on larger ones is false exactly when such a rule exists: a smaller KB passes an `absent` that a larger
one fails.

## What happened to it

| | |
|---|---|
| **2026-11-01** | the ruling moved to M1e2's S1e2.4.1 (Q-M1e2.5) |
| **2026-11-02** | the user ruled ***forbid*** (S1e2.4.1 § 5) |
| **2026-11-04** | built by M1e2's S1e2.2.5, with the user's *via, per activator*; followup F18, world-aware negatives, closed without being done |

## Where it stands in the engine

- **The refusal**: [`forbid.rs`](../../../ein.rs/crates/ein-infer/src/forbid.rs). A program whose refutation
  could rest on an `absent` over a relation a commitment can still extend is refused, on every command that
  asks about a commitment.
- **The pages**: the corollary in
  [`absent_semantics.md`](../../../docs/ein/reasoning/kb-evolution/absent-and-naf.md), and
  [`diagnostics.md`](../../../docs/ein/user/reference/diagnostics.md#27-a-refutation-resting-on-an-absent-a-commitment-can-fill) § 2.7.
- **The census** it planned as its first task: `ein-infer/examples/refutation_census.rs`.
- **`connex`**, the stdlib's one rule of the shape (`std.algebra`): unchanged, and judged per activator like
  any other rule. Over a relation no commitment extends it is closed-world reasoning and legal; over one a
  commitment extends, the program that activates it is refused: `(connex lt instance Node)` beside an hrule
  that proposes `lt` is a load error naming the rule, the activator and the hrule (`ein solve`, the release
  binary at `dbedc535`, 2026-11-15).

## What this phase takes from it

The premise its own first rung needed: on a program that loads, **no death rests on an `absent` that a
commitment can lift**, so on a monotone program a pair dead at root is dead above it
([README § The domain](README.md#the-domain)). [S1f.10.10](s1f.10.10_the_theorys_share.md)'s small KB is the
one place the old worry can still appear, a KB with *stated* facts removed, and its first task stops on a
disagreement for that reason.

The stage file as planned is in `git log -- plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md`.
