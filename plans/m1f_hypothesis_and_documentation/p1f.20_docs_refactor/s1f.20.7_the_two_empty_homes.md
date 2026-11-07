# S1f.20.7 — The two empty homes

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Estimate:** 1 day.
**Status:** not started. It was `T1f.5.20.7` until 2026-11-08.
**Depends on:** [S1f.20.2](s1f.20.2_the_disposition_table.md)'s table, reviewed, since it moves
`05_inspirations.md`. It can run any time after that.

## What it builds

`docs/ein/overview/README.md` and `docs/demo/README.md`: an index, the axes, and a first line naming the
milestone that fills it.
- **`overview/`** is [M5](../../m5_presentation/README.md) Track A's: the comparison against z3, CVC4, Lean,
  K, OpenCog, Prolog and Certora.
- **`demo/`** is M5's: *"From Zebra puzzle to Autoformalization"*. The user's note already gives its
  five-beat structure:
  1. enumeration with hardcoded predicates in C;
  2. link grammar and SMT;
  3. IR as a reasoning substrate;
  4. Ein;
  5. theory selection, construction and neural-guided synthesis.

`ir/03-ein-lang/05_inspirations.md` moves under `overview/` as its first real content, with its inbound
links. `demo/` links [`c/README.md`](../../../c/README.md): the three C baselines and the 3 668 465× table
*are* beat one of that article, already written and already measured.

One day. Writing more would be writing M5's milestone for it.

## Acceptance

- `docs/ein/overview/` and `docs/demo/` exist as indexes, each naming its owning milestone in its first line.
- `inspirations.md` is under `overview/`, with every link to it rewritten.
- `./run_tests.sh` is green.
