# The kernel's verdicts as rules (P1f.21)

**Moved** 2026-11-18 from the milestone's review bundle (`review/05-verdicts-as-rules/`) into this phase, on the user's
word (*"plans/m1f_hypothesis_and_documentation/review/05-verdicts-as-rules - move to p1f.21"*): the two questions
are this phase's. Pages and programs committed, outputs ignored, as in the bundle.

[← the phase](../README.md)

P1f.21's questions, **none ruled**; the phase is not started. Item 34 was added 2026-11-19 with the user's fifth
instruction, its runs taken with the gate build at `e4a5f8fb`; [the overview](../overview.md) is the page to read
first. The material is
`../../p1f.21_verdicts_as_rules/` (`design.md`, `open_facts.md` with the user's second and third instructions).
Nothing was built: the options are runs of programs that emulate them today, and `.MOCK.` files. Re-taken with
`build/review-bin/ein-head-17085b27`, every P1f.23 stage merged (`tools/run.sh` in each): no run moved since the first
look; the drawings and the KB mocks are in the canonical form, and the census of the kernel's names lost the native
symmetry and every read of `is-a` (Q-M1f.436).

| # | item | the question | state |
|---|---|---|---|
| 32 | [Q-M1f.390](32-Q-M1f.390/README.md) | what an obligation emits: the verdict, with its justification, or as a fact | not ruled; `design.md` recommends (b); the user's `(open F J)` beside (d) |
| 33 | [Q-M1f.399](33-Q-M1f.399/README.md) | the names of what controls the search, `false` and `open` | **ruled 2026-11-19**, the word with it (Q-M1f.550, `inconsistent`): no prefix, `false` kept, a rule's key `:ein inconsistent`. The item leaves the bundle when S1f.21.6 is in: that stage is editing a page that links it |
| 34 | [Q-M1f.549](34-Q-M1f.549/README.md) | hypotheses as facts: held, in which reading, and how far the search goes into rules | not ruled; recommended (b), held as *asked* |

## Found while building

- **32**: every count of the ledger re-taken and held; the user's form holds 4 `(open F J)` facts at
  {(seats Ann C1)}, 8 at seats' root, 112 at zebra2-obligations' root. Option (c) runs today as ordinary rules (the
  unpaid requirement equals the kernel's debt at 16 of 16 nodes, same R); a debt derived as a fact is stale in 15 of
  16 nodes. The two KB mocks are drawn again in the canonical form: each fact of `open` a red, bold octagon whose
  arrow `1` is at the box `open`. `design.md` § 4.4 and § 4.5 assume a state file that lists facts; a saved file today lists none, so under
  (d) or the user's form a saved file need not change. `--dump-answer` writes no file for an owing node below a
  non-empty R.
- **33**: `(false)` jumps the agenda and stops the node; `(think Ann (false))` leaves it alive. The kernel's 24 names
  with their 70 reading lines, each marked as `open_facts.md` § 8 has it (27 and 79 at the first look:
  `__symmetric__` and the mirror's `a`, `b` went with the native symmetry, and the KB view's read of `not` with the
  drawing); `is-a` is spelt nowhere outside the tests, and `__symmetric__` in no program. Today `(ein.kill)` loads as an ordinary
  fact and kills nothing; `(open (locks ?k ?d))` is refused at the parse; a module of kill, contradiction and
  double-negation rules loads. Emulated: since the agenda reads `(false)` by its name, a kill module loses the
  fail-first place and *refuted by* names the module's rule, not the program's; nothing reserves the `ein.` prefix.
  § 8.4's c6 gives 16 hits, not 18 (the claim holds).
- **34**: *zebra2* and the seats program answer the same with their candidates held as ordinary facts and one generic
  hrule over them, the read-out byte for byte; root holds 125 such facts on *zebra2* and $H$ is 56 of them. A rule
  that stores *asked and still undecided* gets the program flagged and a stale fact in 15 of 16 nodes. The lattice of
  a two-candidate program written as rules gives the oracle's $R$ in one saturation, and its rule over a second
  absence answers wrong, three strata printed and not followed (V-8).
