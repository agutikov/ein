# 41 · S1f.21.10 as built: the key's readings, and what a module every program gets brings

[← the part](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.10_the_kernels_prefix.md) ·
the ledger: [Q-M1f.562](../../../../open_questions.md#q-m1f562--the-keys-readings-as-built),
[Q-M1f.563](../../../../open_questions.md#q-m1f563--what-a-module-every-program-gets-brings)

**Part 1. Built 2026-11-19 (`68c3d5d0`); every point below is a default the stage took, yours to revise.** The two
rulings it built are not here: the word, `inconsistent` (Q-M1f.550), and `std.core` always loaded (Q-M1f.551). No
answer moved: old binary against new over the 913 corpus files, the same exit code, read-out and stderr in each.

## What it looks like

The gate build of master at `946ee788` (`build/review-bin/ein-head-946ee788`, with that commit's `stdlib/`), on
[`programs/`](programs/); `tools/run.sh` re-takes them into `runs/` (both ignored), the read-out, the events, the
trace, the saturated dump and the program's drawing of each.

`std.core` is one rule, [`stdlib/core.ein`](../../../../../../stdlib/core.ein):

```lisp
(rule inconsistent ()
  :match (false)
  :ein   inconsistent
  :why   "a state that derives a contradiction is inconsistent")
```

| program | what it writes | what `ein` says |
|---|---|---|
| [`1-false-kills.ein`](programs/1-false-kills.ein) | a rule concludes `(false)` | dead; `refuted by not-both`; and the firing that ended the state, of the module's rule: `{"e": "fire", "rule": "std.core.inconsistent", "premises": ["(false)"], "derived": [], "ein": "inconsistent"}` |
| [`2-own-key.ein`](programs/2-own-key.ein) | the same rule states `:ein inconsistent` itself | dead, the same core, `refuted by not-both`; no `(false)` is ever held |
| [`3-false-in-a-context.ein`](programs/3-false-in-a-context.ein) | `(think Ann (false))` | alive: `∅ in R`, and the query answers `?who = Ann` |
| [`4-another-word.ein`](programs/4-another-word.ein) | `:ein dead` | refused: *rule 'r': :ein dead — dead is no word of the engine's (the engine's words: inconsistent)* |
| [`5-both-clauses.ein`](programs/5-both-clauses.ein) | `:assert` and `:ein` on one rule | refused: *states :ein inconsistent beside an :assert — a rule states what it concludes under :assert, or a word to the engine under :ein, never both: write one rule for each* |
| [`6-import-core.ein`](programs/6-import-core.ein) | `(import std.core)` | refused: *std.core is the module every program gets, with no import line: no import names it* |

`ein saturate --dump` lists the module's rule among the agenda's, `std.core.inconsistent  :params ()  :ein
inconsistent`, in every program.

## The key's readings (Q-M1f.562)

| | reading | built | the other reading |
|---|---|---|---|
| 1 | a rule's conclusion | `:assert` **or** `:ein`, never both (row 5); a word that is not the engine's is refused (row 4) | a rule may state both |
| 3 | who may state the key | any rule, a program's own among them (row 2); *forbid* reads it as it reads a rule that concludes `(false)` | only the standard library's rule |
| 4 | what `refuted by` names | the program's rules: the module's rule is read through, so row 1 says `not-both` as it did before | the rule that ended the state, `std.core.inconsistent`, on every dead node |
| 5 | when a rule with the key fires | before anything else the agenda holds; a node stopped at its first contradiction stops where it did, one firing later | with the others that lead to a death |
| 7 | the surfaces' words for a dead state | *holds a contradiction*, as S1f.21.6 put them | *is inconsistent*, the key's word |

Reading 6, the word `false` in the grammar and `:expect (false)`, is S1f.21.11's and untouched.

**`std.core` declares nothing**, though it may (your correction of the stage's brief): the one declaration that is
natural there, of `false`, cannot be written while `false` is a reserved word of the grammar. It becomes writable
with S1f.21.11's task on the reserved words, and is put there.

## What a module every program gets brings (Q-M1f.563)

Four consequences of *always loaded*, each taken by the stage as the only way it found to keep a program and its
container the same, and each something you may want otherwise.

| | consequence | why it was taken | what it shows as |
|---|---|---|---|
| a | **an import of `std.core` is refused**, in a program or a module | the module is in one place, last, under full names, so a program and its container list the same rules in the same order | row 6 above |
| b | **a saved file written before this stage no longer loads** | a saved file carries every module it needs and reads no other file; one written before carries no `std.core` | *std.core, the module every program gets, cannot be read: … 01.ein carries no module std.core, and a saved file reads no other file*, on a state file dumped the day before. The one in the corpus, `tests/saved/04_a_saved_node.ein`, was written again from its program |
| c | **an `$EIN_STDLIB` that names no standard library refuses every program**, where it refused only one that imported from the library | every program now reads the library | `runs/no-stdlib.solve.txt`: *… $EIN_STDLIB names /tmp, which has no MANIFEST.sha256 …* |
| d | **the container's format is `einb/2.2`**, with a section for the firings that ended a saved state | without it a saturated dead knowledge base did not come back dead | `ein --version`: `container  einb/2.2` |

## What moved, and what did not

- **No answer**: of 668 summaries, no `region`, `queries`, `work`, `claim`, `verdict` or count of root's facts.
- **One firing more a dead node**: the events, the traces, `ein saturate`'s firing totals on a dead root, `ein render
  h-space`'s applications of a dead node; **one rule more a program**: the dumps' and the drawings' lists.
- The shape golden: 4 266 of 29 987 lines, all of the kinds `load`, `saturate`, `plan`, `match`, `naf`, `oracle` and
  `region-trace`; no read-out, core, state, lattice or drawing line.
- The gate in the runner's image and the nightly are the coordinator's, on `946ee788`.
