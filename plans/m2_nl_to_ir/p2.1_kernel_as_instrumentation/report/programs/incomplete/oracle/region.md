# Region · tests/region/04_owing.ein

The lattice is over **0 root candidates** — the generator's list at root's fixpoint with no config switch applied (`docs/ein/user/04-reference/11-region-dump.md`).

**Monotone by construction**: no read a commitment can undo (20 saturation and 1 obligation plans read; 2 reads inside an `absent`, 0 of them over a relation a commitment changes, all at monotone polarity).

| bit | candidate |
|---:|---|

## R — alive and discharged

- **|R| = 0** of 2^0 = 1 nodes
- **minimal line**: none
- **maximal line**: none
- **holes**: none
- **components**: 0 (sizes —)
- **maximal sub-cubes**: none
- **N**, minimal dead sets: none
- **E**, minimal discharging sets: none
- **duality**: minimal line from E holds; maximal line from N holds
- **upward closure**: dead nodes with an alive superset: none; discharged nodes with an owing superset: none
- **edges**: 

## Every node

| layer | node | status | why |
|---:|---|---|---|
| 0 | `∅` | owing | owes likes owes Ann a Food. |

## Cost

root 0.335 ms · 1 nodes 0.020 ms (20.0 µs each) · topology 0.004 ms. These are the only lines of this directory that differ between two runs.
