# Save and load

[← the bundle](../README.md)

A saved file is the program unchanged, every module it read, and the choices made, `(commit …)`; it loads as that
node of its program's lattice (Q-M1f.392, Q-M1f.400 to .403). These items are what that build left to rule, and what
went with the old state files: each fact written with its English in a comment.

| # | item | the question | default |
|---|---|---|---|
| 07 | [L-106](07-L-106/README.md) | the round trip's nightly shards near five minutes | as it is |
| 17 | [Q-M1f.413](17-Q-M1f.413/README.md) | `ein kb save` of a saved file | (a) a root's stored, a node's refused |
| 18 | [Q-M1f.415](18-Q-M1f.415/README.md) | `ein show hyps` / `ein render h-space` of a saved file | (a) the program's H |
| 19 | [Q-M1f.416](19-Q-M1f.416/README.md) | what `ein test` asks of a saved node | (a) root alone, NOT CHECKED, exit 0 |
| 20 | [Q-M1f.417](20-Q-M1f.417/README.md) | the `commit` row, `root.commit`, `ein-answer/3` | (a) |
| 21 | [Q-M1f.418](21-Q-M1f.418/README.md) | the nesting limit and a saved file's wrappers | (a) one level more |
| 30 | [L-104](30-L-104/README.md) | `saturate --dump` prints `:hypothesis 0` for every choice | as is |
| 31 | [F23 D-24](31-F23-D-24/README.md) | the carried-module note when `$EIN_STDLIB` points nowhere | as is |

## Found while building

What the ledger does not say, each on its item's page under *Found while building*.

- **07**: shard 4 holds three heavy programs side by side, so spreading a program's files over workers alone does not
  move it; dealing the heavy programs one to a shard does (about 98 to 139 s a shard). The third heaviest is
  `cul-de-sac` (208 s), not `04_open`.
- **17**: a container stored under `EIN_STDLIB=/nonexistent` records that library's digest, so later runs warn
  `StaleStdlib`; a carried module that differs from the binary's is noted at `kb save` only, and the container is
  read silently afterwards.
- **18**: no surface marks which candidate is the node's own choice: `show hyps` lists 3, `solve -H` 2, for one file.
- **19**: on a node, the query claim's verbose header reads `(--no-search: all of R, …)` with no `--no-search` given,
  and that row says `over: all`, `exact: false`, over the node alone.
- **20**: on a node's run, the work row and the region budget count the node's own choice once per saturation (9 facts
  beyond root against 5 from the earlier binary, for the same R).
- **21**: a nesting refusal inside a carried text is placed at the saved file's own line and column, against
  `10-answer-dump.md`'s *every position in a carried text is the text's own*. Q-M1f.185's rewrite follows from the
  format, not from the extra level: option (b) loses only programs and modules at exactly 1 000.
- **30**: the ledger's mechanism is not what runs: `region.rs` writes the branch `0` (three places) and the dump reads
  `saturate.rs`' `branch(p)`, not `sentence.rs`; a refuted claim prints the same `0` (`hypothesised in branch 0`).
- **31**: `tests/saved/03`'s header says three modules are carried; the file carries five.
