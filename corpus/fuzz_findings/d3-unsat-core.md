# d3-unsat-core

A generated program, minimised, whose printed unsat core could differ between two runs that narrate different
amounts of one derivation. It is not monotone: `fire-0` concludes `(not …)` from `r1`, which
`fire-1` derives resting on an `absent` over `(not r1)`, a relation a commitment can still extend, so the program
can call a state that has not yet decided `(not r1)` a contradiction. It loads, the static check flags it, and
nothing is claimed of its answer. Its fixture is
[`examples/ein-bugs/d3-unsat-core.ein`](../../examples/ein-bugs/d3-unsat-core.ein); this note and the `.ein` beside
it stay as a frontend-fuzzer seed and the program below.

- run: `solve`
- minimised: 14 → 9 forms

```
;;; case 6710 (seed 20260820) — a generated program, minimised
(relation r0 T T)
(relation r1 T T)
(relation is-a T T)
(is-a o2 T)
(is-a o3 T)
(r0 o2 o3)
(rule fire-0 (?P)
  :match  (and (r1 ?v0 ?v1) (r0 ?v1 ?v0))
  :assert (not (r1 ?v0 ?v0)))
(fire-0 T)
(rule fire-1 ()
  :match  (and (is-a ?v0 T) (r0 ?v0 ?v1) (r1 ?v2 ?v3) (absent (not (r1 o3 ?v1))))
  :assert (r1 ?v2 ?v2))
```

## What it guards

A fork *resumes* root's saturation rather than re-deriving it, so two runs can narrate different amounts of one
derivation; on a program whose refutation rests on an `absent` a commitment can lift, that difference could reach
the printed core. The id-order sweep permutes the fixture and compares each run with the unpermuted one.

```sh
ein solve corpus/fuzz_findings/d3-unsat-core.ein   # the warning, and the site fire-1 reads its absent at
```
