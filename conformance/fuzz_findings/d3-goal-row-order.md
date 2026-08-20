# d-1787188217-002

- found: 2026-08-20 05:16:50
- kind: **diff**
- seed: 20260820, mode: mixed, tier: T3
- run: `solve --max-set-size 2 --max-enterings 300`
- minimised: 11 → 7 forms
- from: `conformance/out/fuzz/cases/c003567.ein`

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

## The harness's diff, on the minimum

```
  conformance/out/fuzz/cases/x-1787188217-002.ein :: solve --max-set-size 2 --max-enterings 300
      stdout:7: "    ?x  = B" vs "    ?x  = A"
  conformance/out/fuzz/cases/x-1787188217-002.ein :: solve -e --max-set-size 2 --max-enterings 300
      stdout:7: "    ?x  = B" vs "    ?x  = A"
[2/4] DIFF conformance/out/fuzz/cases/x-1787188217-002.ein :: solve --max-set-size 2 --max-enterings 300
        stdout:7: "    ?x  = B" vs "    ?x  = A"
[4/4] DIFF conformance/out/fuzz/cases/x-1787188217-002.ein :: solve -e --max-set-size 2 --max-enterings 300
        stdout:7: "    ?x  = B" vs "    ?x  = A"
```
