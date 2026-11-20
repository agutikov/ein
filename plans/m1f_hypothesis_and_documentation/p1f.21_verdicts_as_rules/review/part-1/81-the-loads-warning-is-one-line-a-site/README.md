# 81 · The load's warning is one line a site

[← part 1](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the finding](../../../findings.md#v-32--the-loads-warning-is-one-line-a-site)

A finding of the phase that is **yours to call**. It was on the findings page alone; it is here so that everything
that waits for you is in one place. Nothing waits on it.

## What it is

The refusal for an activator a rule could derive printed one line and stopped. As the warning you ruled it to be,
it prints every site: a program that names a rule of a copier's count beside `std.algebra` loads with seventeen
lines, one for each generic rule of the module (`converse`, `imply2-fwd`, …). Each line is true.

## The question

- **(a)** As built: one line a site, all of them. **Default.**
- **(b)** A module's lines are gathered into one, which names the module and counts its rules.

## On the built engine

**How it was taken.** The final binary, `build/review-bin/ein-final-7b72bd84`, the release build of `7b72bd84`, with
the checkout's `stdlib/`; its `--version` is `runs/version.txt`. `tools/run.sh` re-takes every run into `runs/`
(both ignored by git). For each program: the read-out (`X.solve.txt`), `-p`, `--monotone`, `ein show hyps`,
`ein saturate --dump`, `ein test`, the oracle with every node's state, the events and the summary, the answer and the
visited lattice as files, and the knowledge base, the lattice (both views) and the region drawn (`X.kb.svg`,
`X.lattice.svg`, `X.lattice-full.svg`, `X.region.svg`, each beside its `.dot`). Then **stderr alone**, under six
commands, `ein solve`, `ein saturate`, `ein show hyps`, `ein oracle`, `ein render kb` and `ein test`
(`runs/stderr/X.<command>.txt`), and its `warning:` lines counted (`runs/warning-counts.txt`).

**The programs.** The finding's program is the one the reference page of `std.negation` shows warning once
([*A program the import warns of*](../../../../../../docs/ein/user/04-reference/04-stdlib/20-negation-and-contradiction.md#a-program-the-import-warns-of)),
with `std.algebra` imported in its place. Its rule `guess` has two parameters, the first standing first in a
pattern, so it names a relation; `(offered guess)` writes the rule's name as an argument; and a copier of
`std.algebra` concludes a fact of two arguments whose relation is a parameter, which the loader reads as writing into
every rule of that count whose name the program writes as an argument.

- [`algebra.ein`](programs/algebra.ein): the page's program, `(import std.algebra)` in the place of
  `(import std.negation)`. The finding's case.
- [`negation.ein`](programs/negation.ein): the page's program as the page writes it. The smallest that warns:
  once.
- [`both.ein`](programs/both.ein): both imports. Two modules' lines on one load, which option (b) would make two.

### The lines: the finding's figure, and today's

| program | the finding's figure | **`warning:` lines on stderr, the built engine** | the run |
|---|---|---|---|
| `algebra.ein` | seventeen, one a generic rule of `std.algebra` | **17**, under each of the six commands | `runs/warning-counts.txt`, `runs/stderr/algebra.*.txt` |
| `negation.ein` | one (the `std.negation` page) | **1**, under each of the six | `runs/warning-counts.txt` |
| `both.ein` | — | **18**, under each of the six: `std.negation`'s one, then `std.algebra`'s 17 | `runs/warning-counts.txt`, `runs/stderr/both.solve.txt` |

Stderr holds nothing else under `ein solve`, `ein saturate`, `ein oracle` and `ein render kb`; under `ein show hyps` it
has one line more, the `note:` that names H's writer (``H, from `hypothesis-rules`: 4 candidates``), and under
`ein test` one more, *error: nothing to check — no (query …) in the selection carries an :expect*, exit 2, since no
query states an `:expect` (`runs/X.test.txt`). Every other command exits 0 (`runs/X.solve.txt`, `X.saturate-dump.txt`,
`X.show-hyps.txt`, `X.oracle.txt`).

The seventeen rules `algebra.ein`'s lines name, in the order printed (`runs/algebra.warned-rules.txt`):
`converse`, `imply2-fwd`, `imply2-reverse`, `compose`, `identity`, `meet`, `difference`, `join-l`, `join-r`, `top`,
`complement`, `difunctional`, `symmetric`, `symmetric-negative`, `includes`, `compose-negative-s`,
`compose-negative-r`, each `std.algebra.`-prefixed. The first line, whole:

<!-- output -->
```text
warning: programs/algebra.ein: rule 'std.algebra.converse': its :assert writes (?R2 ?b ?a), whose ?b is rule 'guess''s parameter ?R, which names a relation, and (?R1 ?a ?b) can bind ?b to a fact, as an argument of the relation ?R1 names — a relation is named by a symbol, not a fact, so such an activator activates nothing
```

### What the program answers

The three answer alike, and none is flagged: `states (k_s) 16`, `lattice monotone (the static check) — R is convex:
no hole`, `universe 4 candidates, 0 dead alone` (`runs/X.solve.txt`); `--monotone` says `yes — no site`
(`runs/X.monotone.txt`); `ein show hyps` lists the four candidates `(chosen Milk)`, `(chosen Tea)`, `(served Milk)`,
`(served Tea)` (`runs/X.show-hyps.txt`); and the oracle's cube agrees, `states (k_s) 16` and *the static verdict
holds* (`runs/X.oracle.txt`). Nothing activates a copier with `guess`, so no state derives the activator the lines
warn of, and no run line follows the load's lines. The `std.negation` page's figure for its program,
`states (k_s) 16`, is the same today.
