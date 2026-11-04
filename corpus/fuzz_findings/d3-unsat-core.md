# d3-unsat-core

**2026-11-04**: refused since M1e2 S1e2.2.5 (*forbid* with F2's *via*: `fire-0`
concludes `(not …)` from `r1`, which `fire-1` derives under an `absent` a
commitment can lift), the refusal the same under eight permutations of the
interner (S1e2.2.12), and promoted by S1e2.2.12 to
[`examples/ein-bugs/d3-unsat-core.ein`](../../examples/ein-bugs/d3-unsat-core.ein).
*The id-space sweep is green on it* below was true at seed 1 only, of the
walk: at the third seed its core moved 4 → 6 (M1e2 W11). The note below is
the finding as it stood.

- found: 2026-08-20, `utils/fuzz_engine.py --seed 20260820`, mode `mixed`
- run: `solve --max-set-size 2 --max-enterings 300`
- minimised: 14 → 9 forms

A fork *resumes* root's saturation rather than re-deriving it, so two runs can
narrate different amounts of one derivation, and on this program the
difference reaches the printed unsat core: 4 facts on the resumed path, 6
under `EIN_FORK_DELTA=0`. Unlike its sibling `d3-goal-row-order`, this one **is**
the resumed fork saturator and not an ordering accident. The id-space sweep is
green on it.

```
;;; case 6710 (seed 20260820) — utils/fuzz_engine.py
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
