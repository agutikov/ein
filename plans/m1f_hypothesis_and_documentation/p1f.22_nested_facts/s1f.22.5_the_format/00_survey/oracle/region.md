# Region · examples/domain_elim/b_branch.ein

The lattice is over **7 root candidates** — the generator's list at root's fixpoint with no config switch applied (`docs/ein/user/04-reference/11-region-dump.md`).

**Monotone by construction**: no read a commitment can undo (3 saturation and 0 obligation plans read; 0 reads inside an `absent`, 0 of them over a relation a commitment changes, all at monotone polarity).

| bit | candidate |
|---:|---|
| 0 | `(color-loc Blue H1)` |
| 1 | `(color-loc Blue H2)` |
| 2 | `(color-loc Blue H3)` |
| 3 | `(color-loc Green H1)` |
| 4 | `(color-loc Green H2)` |
| 5 | `(color-loc Red H1)` |
| 6 | `(color-loc Red H3)` |

## R — alive and discharged

- **|R| = 2** of 2^7 = 128 nodes
- **minimal line**: `∅`
- **maximal line**: `{(color-loc Blue H1)}`
- **holes**: none
- **components**: 1 (sizes 2)
- **maximal sub-cubes**: [∅ … {(color-loc Blue H1)}] dim 1
- **N**, minimal dead sets: `{(color-loc Blue H2)}`, `{(color-loc Blue H3)}`, `{(color-loc Green H1)}`, `{(color-loc Green H2)}`, `{(color-loc Red H1)}`, `{(color-loc Red H3)}`
- **E**, minimal discharging sets: `∅`
- **duality**: minimal line from E holds; maximal line from N holds
- **upward closure**: dead nodes with an alive superset: none; discharged nodes with an owing superset: none
- **edges**: R→R 1, R→dead 12, dead→dead 435

Every node is in `region.json` (`region.nodes`, 128 records).

## Cost

root 0.115 ms · 128 nodes 2.533 ms (19.8 µs each) · topology 0.021 ms. These are the only lines of this directory that differ between two runs.
