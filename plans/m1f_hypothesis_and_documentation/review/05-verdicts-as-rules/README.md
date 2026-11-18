# The kernel's verdicts as rules (P1f.21)

[← the bundle](../README.md)

P1f.21's two questions, **neither ruled**; the phase is not started. The material is
`../../p1f.21_verdicts_as_rules/` (`design.md`, `open_facts.md` with the user's second and third instructions).
Nothing was built: the options are runs of programs that emulate them today, and `.MOCK.` files.

| # | item | the question | state |
|---|---|---|---|
| 32 | [Q-M1f.390](32-Q-M1f.390/README.md) | what an obligation emits: the verdict, with its justification, or as a fact | not ruled; `design.md` recommends (b); the user's `(open F J)` beside (d) |
| 33 | [Q-M1f.399](33-Q-M1f.399/README.md) | the names of what controls the search, `false` and `open` | not ruled; leaning `(ein.kill)` |

## Found while building

- **32**: every count of the ledger re-taken and held; the user's form holds 4 `(open F J)` facts at
  {(seats Ann C1)}, 8 at seats' root, 112 at zebra2-obligations' root. Option (c) runs today as ordinary rules (the
  unpaid requirement equals the kernel's debt at 16 of 16 nodes, same R); a debt derived as a fact is stale in 15 of
  16 nodes. `design.md` § 4.4 and § 4.5 assume a state file that lists facts; a saved file today lists none, so under
  (d) or the user's form a saved file need not change. `--dump-answer` writes no file for an owing node below a
  non-empty R.
- **33**: `(false)` jumps the agenda and stops the node; `(think Ann (false))` leaves it alive. The kernel's 27 names
  with their 79 reading lines, each marked as `open_facts.md` § 8 has it. Today `(ein.kill)` loads as an ordinary
  fact and kills nothing; `(open (locks ?k ?d))` is refused at the parse; a module of kill, contradiction and
  double-negation rules loads. Emulated: since the agenda reads `(false)` by its name, a kill module loses the
  fail-first place and *refuted by* names the module's rule, not the program's; nothing reserves the `ein.` prefix.
  § 8.4's c6 gives 16 hits, not 18 (the claim holds).
