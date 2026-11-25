# Region · examples/lattice/03_state_hash_collision.ein

The lattice is over **3 root candidates** — the generator's list at root's fixpoint with no config switch applied (`docs/ein/user/04-reference/11-region-dump.md`).

**Monotone by construction**: no read a commitment can undo (3 saturation and 0 obligation plans read; 0 reads inside an `absent`, 0 of them over a relation a commitment changes, all at monotone polarity).

| bit | candidate |
|---:|---|
| 0 | `(h1 X)` |
| 1 | `(h2 X)` |
| 2 | `(h3 X)` |

## R — alive and discharged

- **|R| = 8** of 2^3 = 8 nodes
- **minimal line**: `∅`
- **maximal line**: `{(h1 X), (h2 X), (h3 X)}`
- **holes**: none
- **components**: 1 (sizes 8)
- **maximal sub-cubes**: [∅ … {(h1 X), (h2 X), (h3 X)}] dim 3
- **N**, minimal dead sets: none
- **E**, minimal discharging sets: `∅`
- **duality**: minimal line from E holds; maximal line from N holds
- **upward closure**: dead nodes with an alive superset: none; discharged nodes with an owing superset: none
- **edges**: R→R 12

## Every node

| layer | node | status | why |
|---:|---|---|---|
| 0 | `∅` | R |  |
| 1 | `{(h1 X)}` | R |  |
| 1 | `{(h2 X)}` | R |  |
| 1 | `{(h3 X)}` | R |  |
| 2 | `{(h1 X), (h2 X)}` | R | same state as {(h2 X)} |
| 2 | `{(h1 X), (h3 X)}` | R |  |
| 2 | `{(h2 X), (h3 X)}` | R | same state as {(h2 X)} |
| 3 | `{(h1 X), (h2 X), (h3 X)}` | R | same state as {(h2 X)} |

## Cost

root 0.118 ms · 8 nodes 0.044 ms (5.5 µs each) · topology 0.007 ms. These are the only lines of this directory that differ between two runs.
