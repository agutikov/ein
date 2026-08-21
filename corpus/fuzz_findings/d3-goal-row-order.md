# d3-goal-row-order

- found: 2026-08-20, `utils/fuzz_ein.py --seed 20260820`, mode `mixed`, as a
  cross-engine **diff** at T3 — `?x = B` in ein.py, `?x = A` in ein.rs
- property: **id-order** — the same answer under a permuted interner
- re-found: 2026-08-21, by the rewritten fuzzer
  ([S1a.10.4](../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md)),
  from a different seed and a different mutation source
  (`examples/branching/13_lookahead_naf_world.ein`), minimised to **the same
  seven forms**
- minimised: 11 → 7 forms

```
(relation ok      T)
(relation blessed T)
(relation cand    T)
(rule promote ()
  :match  (and (ok ?x) (blessed ?x))
  :assert (cand ?x))
(ok      B)
(blessed A)
(query
  :goal (cand ?x))
```

## What happens

The goal `(cand ?x)` is satisfied **twice** in one model — the blind
enumerator hypothesises `(cand A)` and `(cand B)` and both survive, which
`--print-final-state` shows. The solve table prints **one** row, and which one
is not determined by the data:

```
  query bindings
    ?x  = A
```

## The re-find narrows what this file means

It was filed as a
**[D3](../../plans/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)**
consequence, and that reading was too narrow. The cross-engine difference was
real and D3 did cause it — a resumed fork puts facts into the KB in a
different order, and the row is `rows[0]` of an unsorted match. But **D3 is
one thing that perturbs the row, not what makes it perturbable.** With D3 held
fixed — one engine, one build, `fork-delta` off in both runs — the row still
moves when the interner assigns ids in a different order:

```sh
mkdir -p /tmp/goal-row && cp corpus/fuzz_findings/d3-goal-row-order.ein /tmp/goal-row/
EIN_ID_FILES=/tmp/goal-row cargo test --manifest-path ein.rs/Cargo.toml \
    -p ein-render --test id_order_invariance
```

```
1 of 45 (file, op) pairs move when the ids do:
  d3-goal-row-order.ein [trace[answer]] seed 1
    line 11
      plain:        ?x  = A
      permuted:     ?x  = B
```

So the cross-engine difference was a *symptom* of an under-determined
rendering, and the two engines were each reporting a legitimate row. Filed
against D3 the finding is a divergence to accept; filed against the renderer
it is the class
[design/02](../../plans/m1a_rust/design/02_determinism_and_order.md) forbids —
an observable that depends on the order ids were assigned in — and one of the
first two the id-space sweep has found that the corpus does not reach. The
second framing is the one that survives losing the second engine, which is
the practical difference: after
[P1a.10](../../plans/m1a_rust/p1a.10_single_implementation/README.md) nothing
can re-run the first.

`summary.json` is already right: `goal_bindings` carries **both** rows,
sorted. The table is the surface that picks one. Nobody has decided whether it
should print every row, print the first in `Interner::rank` order, or say that
there is more than one — which is why this is still a finding and not a fix.
