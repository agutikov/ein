# D1 — Q4: which route reaches the unguarded `record_node`?

**Blocks:** [T1e.1.1.2](README.md#task-t1e112--q4-construct-the-alive-path).
The task has no shape until this is answered.
**Decides:** whether [CO-M1](../../README.md#the-findings) is a bug at shipped
defaults, a guard for a shape only a `(config …)` line reaches, or `accepted`.

## What is undecided

The review says `phase2` records root as a model without re-checking
`has_contradiction`. It does — but reaching that line is much harder than the
finding implies, and *how* the fixture gets there is the whole disposition.

The relevant window is fifteen lines of
[`solve.rs`](../../../../ein.rs/crates/ein-infer/src/solve.rs):

```rust
if a_layer.is_empty() { break; }                       // :1528  survivors gone → never reaches below
alive = self.compute_alive(root, terms, ast, events)?; // :1534  re-read after this layer's (not h)
if self.cfg.enable_forced_positive {
    let (next, term) = self.promote_forced_positives(…)?; // :1536  ← re-saturates AND re-checks
    …
}
if alive.is_empty() {                                  // :1544
    let owes = crate::obligations::tally(…)?;          // :1548
    self.record_node(root, terms, Vec::new(), …);      // :1550  ← the unguarded site
    break;
}
```

`promote_forced_positives` (`:2086`) is `while alive.len() == 1 { promote;
re-saturate; if has_contradiction { return terminal } ; alive = recompute }`.
So **every path through a singleton is checked.** The site is reachable
un-re-saturated only when `compute_alive` returns ∅ *directly* — from ≥ 2, or
from 0.

## The two routes

### Route A — turn the cascade off

```lisp
(config :enable-forced-positive false)
```

skips `:1536` entirely, so `alive` can arrive at `:1544` by any path. Cheap,
certain, and it proves the *branch* is unguarded rather than that the shipped
engine reaches it.

Sketch of the rest, which is the review's own recipe — totality as a
saturation refutation rather than as an obligation, so the `:1548` tally is
silent:

```lisp
(config :enable-forced-positive false)
(relation is-a T T) (relation p T)
(is-a A T) (is-a B T)

;; "every object must have p" — as (false), NOT as (open ?R)
(rule needs-p ()
  :match  (and (is-a ?x T) (absent (p ?x)))
  :assert (false)
  :priority 400)

(query :goal (p ?x))
```

with whatever makes a layer-1 singleton die so root grows a `(not …)` before
`compute_alive` is re-read.

### Route B — the lookahead empties `alive`

No lever. `alive` shrinks between layers for two reasons, and only one of them
is a writeback: the pipeline's third filter drops a candidate that **provably
dies in one firing against the grown root**
([`hypgen.rs:440`](../../../../ein.rs/crates/ein-infer/src/hypgen.rs)). A layer
whose deaths add enough `(not h)` to root can make every remaining candidate
doomed at once — `alive` goes from ≥ 2 to 0 with no singleton in between, and
`:1536` never runs.

If Route B is constructible, **Q4 and Q5 are one mechanism seen from two
places**, and CO-M1 is a defect at stock configuration.

## Options

| | what T1e.1.1.2 does | consequence |
|---|---|---|
| **A** | Route B first, timeboxed to a day; Route A as the fallback, with the config line explained in the fixture header | most work. The only option that can find the stock-config defect, and the only one that surfaces the Q4/Q5 coupling |
| **B** | Route A only | decisive in ~2 h. `CO-M1` becomes **fixed** or **refuted** on a program no default run reaches; a reader may fairly say the guard exists for a shape nobody ships |
| **C** | Stock config, and `accepted` if it does not appear | cheapest. Closes a *risk* finding with *"I could not build it"*, which [Q-M1e.1](../../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)'s third rule says is not a refutation — see also [D5](d5_does_t1_ratify_q_m1e2.md) |

**Recommended: A**, falling back to B rather than C.

## What each outcome means for `CO-M1`

Unchanged from the stage's table, and repeated here because this decision is
what selects the row:

| outcome | disposition |
|---|---|
| root recorded with a derivable `(false)` | **fixed** — add the re-check at `:1544`; the fixture is the regression test |
| root correctly not recorded | **refuted** — bank the fixture, write the reason at the site |
| unreachable from any `.ein` program | **accepted** — argument at `:1544`, noting the premise is unchecked because the branch is unreached |

## Related

- [D9](d9_kernel_page_overclaims.md) — the kernel page currently claims *the
  engine never records a false model*. That claim is about **completeness**;
  this decision is about the **consistency** conjunct on the one path that
  does not re-check it. If Q4 lands "fixed", D9's row was wrong.
- [D4](d4_q_m1e9_upward_closure.md) — the reproduced `absent` defect is a
  third way root can carry a negative it should not, which is adjacent to
  this path but not the same one.
