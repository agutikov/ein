# d3-unsat-core

- found: 2026-08-20, `utils/fuzz_ein.py --seed 20260820`, mode `mixed`, as a
  cross-engine **diff** at T3
- run: `solve --max-set-size 2 --max-enterings 300`
- minimised: 14 → 9 forms

Unlike its sibling `d3-goal-row-order`, this one **is** D3: `EIN_FORK_DELTA=0`
reproduces ein.py's six-fact core exactly, so the difference is the resumed
fork saturator and not an ordering accident. The id-space sweep is green on
it.

The diff below is the retired harness's output, kept verbatim as the record of
what the second engine said — it is the last thing that can say it.

```
;;; case 6710 (seed 20260820) — utils/fuzz_ein.py
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

## The harness's diff, on the minimum (2026-08-20)

```
  conformance/out/fuzz/cases/x-1787188217-005.ein :: solve --max-set-size 2 --max-enterings 300
      stdout:6: "  unsat core (6 facts)" vs "  unsat core (4 facts)"
      summary.json:9: "      \"(is-a o2 T)\"," vs "      \"(is-a o3 T)\","
  conformance/out/fuzz/cases/x-1787188217-005.ein :: solve -e --max-set-size 2 --max-enterings 300
      stdout:6: "  unsat core (6 facts)" vs "  unsat core (4 facts)"
      summary.json:9: "      \"(is-a o2 T)\"," vs "      \"(is-a o3 T)\","
[2/4] DIFF conformance/out/fuzz/cases/x-1787188217-005.ein :: solve --max-set-size 2 --max-enterings 300
        stdout:6: "  unsat core (6 facts)" vs "  unsat core (4 facts)"
        summary.json:9: "      \"(is-a o2 T)\"," vs "      \"(is-a o3 T)\","
[4/4] DIFF conformance/out/fuzz/cases/x-1787188217-005.ein :: solve -e --max-set-size 2 --max-enterings 300
        stdout:6: "  unsat core (6 facts)" vs "  unsat core (4 facts)"
        summary.json:9: "      \"(is-a o2 T)\"," vs "      \"(is-a o3 T)\","
```
