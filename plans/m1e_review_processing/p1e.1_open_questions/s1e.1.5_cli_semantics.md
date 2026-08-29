# S1e.1.5 — `-n 0`: Q7

**Phase:** [P1e.1](README.md) (The ten questions)
**Estimate:** 0.5 days
**Depends on:** nothing.
**Blocks:** [EH-L1](../p1e.4_low/s1e.4.4_error_handling.md) — the same
question at Low severity; this stage takes the ruling, that stage carries out
whatever the ruling says and pins it.
**Answers:** [`review/open-questions.md`](../review/open-questions.md) Q7.

## Context

`ein solve -n 0` is accepted. `py_int` allows zero, `stop_after` becomes
`Some(0)`
([`solve.rs:570-574`](../../../ein.rs/crates/ein-cli/src/solve.rs)), and what
the engine then does is whatever `SolveOptions { stop_after: Some(0) }`
happens to do. Nothing states a meaning: no test, no doc, no comment.

The reason this is a question rather than a shrug is sitting twenty lines
away. `--jobs 0` is **refused**, with a message that argues precisely that a
flag with two readings should be refused
([`cmdline.rs:171-179`](../../../ein.rs/crates/ein-cli/src/cmdline.rs) against
`:20-47`). `-n 0` has the same two readings — *stop before recording
anything* and *no limit* — and gets neither the refusal nor the definition.
One CLI, one argument, applied in one place and not the other.

It is half a day because the ruling is small and the precedent is already
written; what makes it worth a stage is that it is the milestone's cheapest
demonstration of the disposition discipline, and it feeds a Low finding that
should not be decided independently.

## Acceptance

- A ruling, recorded in
  [`defined_behaviour.md` § 4](../../../docs/kernel/defined_behaviour.md)
  where the CLI's error table lives: **refuse** with a message in the
  `jobs_spec` form, or **define** (`-n 0` means *no limit* / *record nothing*)
  and say so in `--help`.
- The ruling is pinned by a test either way, and the test is named for the
  behaviour.
- Whether `ein.py` accepted `-n 0`, and what it did, is established from the
  goldens under `tests/golden/from_ein_py/` or recorded as unknowable — the
  likely origin of the current behaviour is parity, and a parity behaviour
  nobody can confirm is parity is just behaviour.
- `-m 0` is checked in the same breath: the lattice honours it as a truncated
  no-op ([`solve.rs:1152-1159`](../../../ein.rs/crates/ein-infer/src/solve.rs))
  and the tree ignores it entirely
  ([CO-H3](../p1e.2_high/s1e.2.1_correctness.md)(a)), so the zero-argument
  question is really two flags wide.

## Tasks

### Task T1e.1.5.1 — Find out what it does today ✅

**Done 2026-08-29. `-n 0` is `-n 1`** — not one of the two readings the
question offered, and the same on every arm. One fixture per verdict, and the
`=` spelling for the negatives because `-n -1` with a space is `clap`'s own
*unexpected argument*:

| fixture | verdict | `-n 0` | `-n 1` | `-n 2` | `-e` |
|---|---|---|---|---|---|
| `examples/zebra2.ein` | one model | k=1 · 11 enterings | **k=1 · 11** | k=1 · 101 | k=1 · 101 |
| `saturation/type-exclusivity/colors.ein` | nine † | k=1 · 144 | **k=1 · 144** | k=2 · 157 | k=5 · 168 |
| `examples/ein-bugs/zebra2-bad.ein` | none | k=0 · 0 | **k=0 · 0** | k=0 · 0 | k=0 · 0 |

† nine at `-e -m 6`; the `-e` column is the default `-m 5`, where the answer is
**5 and `exhausted = false`** — the M1d S1d.3.3 lower-bound case, and not
this stage's business.

`exhausted`, `stats.solution_nodes` and the exit code agree cell for cell, and
`diff` of the whole of stdout between `-n 0` and `-n 1` is **empty** on both
solvable fixtures. `--solutions=-1` and `--solutions=-7` are the same run
again.

**Intentional, incidental or inconsistent: incidental, and consistent.** Two
lines make it:

- `stop_after` is read once, at
  [`solve.rs`](../../../ein.rs/crates/ein-infer/src/solve.rs)'s
  `commit_entering`, as `self.lstate.nodes.len() as u64 >= n` — **after**
  `record_node`. So every `n ≤ 1` cuts at the first model, and `Some(0)` is
  `Some(1)`'s behaviour rather than a case of its own.
- the CLI reaches `Some(0)` from below too: `map(|n| (*n).max(0) as u64)`
  clamps every negative onto it silently.

No arm disagrees, so it does not promote past Low.

**`-m 0` in the same breath, and it is four cells rather than two** — the
review checked three:

| | `-m 0` | `-m=-3` | `-e -m 0` |
|---|---|---|---|
| lattice | `Contradiction k=0 exhausted=false`, 0 enterings | **the same** — clamped | `Contradiction k=0` |
| tree (`EIN_TRAVERSAL=tree`) | `Solution k=1`, 9 enterings — **the cap is ignored** | the same | `Solution k=1` |

The lattice's zero is *defined*: M1d T1d.10.5.0 made it a truncation and
[`defined_behaviour.md` §5](../../../docs/kernel/defined_behaviour.md) states
it. The tree's ignoring it is
[CO-H3](../p1e.2_high/s1e.2.1_correctness.md)(a), and the silent clamp on the
negative is new — it is [Q-M1e.17](../open_questions.md#q-m1e17--three-py_int-options-silently-reinterpret-a-negative).

The task as written:

Run it: `ein solve -n 0` on a single-model entry, an ambiguous one, and a
contradiction, with `--json-summary`. Record `verdict.type`, `k`,
`exhausted`, `stats.solution_nodes` and the exit code for each. Then read
`stop_after`'s consumers and say whether the observed behaviour is
*intentional*, *incidental* or *inconsistent across arms* — the third would
promote this from a Low finding to something else.

Do the same for `-m 0` on both traversals, since the review found the two
traversals disagree about it.

### Task T1e.1.5.2 — Check the parity origin ✅

**Done 2026-08-29. No golden pins it — and the origin is establishable
anyway**, from a source the task did not consider.

`tests/golden/from_ein_py/` holds **nineteen** files and **not one is a
`solve` invocation**: two are `dump_canonical(parse(f))` on the zebra pair
(`ein-ir/tests/golden/from_ein_py/`), seventeen are DOT and trace renderings
(`ein-render/…`). There is no cell to run with `-n 0`, so nothing in the
repo's last independent provenance is at risk and **refuse is available**.

That leaves the parity story a guess only if the guess cannot be checked. It
can: ein.py is in git at `4c1a5b3^`, the parent of *S1a.10.5: ein.py leaves the
tree*, and the repo's own convention already points there
(`git log --diff-filter=D`). Two lines settle it:

```
ein.py/src/ein/cli/solve.py:388   stop.add_argument("-n", "--solutions", type=int, default=1, …)
ein.py/src/ein/inference/monotonic/solver.py:379-380
                                  stop_after is not None
                                  and len(lstate.solution_nodes) >= stop_after
```

`type=int` with no bound, and the same `>=` after the record — the *only*
comparison of `stop_after` in the file. So ein.py accepted `-n 0` and behaved
as `-n 1` too, and ein.rs reproduced a behaviour rather than inventing one.
It also did not clamp, which is why `-n -7` reached the same place there by a
different route.

So the ruling is a **deliberate divergence from parity, on a behaviour parity
never pinned** — which is exactly the class
[P1a.10](../../../docs/history/m1a_rust/README.md#p1a10--one-implementation)
freed: with no oracle, "improve it" and "diverge from it" stopped being the
same act.

The task as written:

Grep `tests/golden/from_ein_py/` — the last independent provenance in the
repo — for any cell run with `-n 0`. If one exists, the current behaviour is
pinned parity and the ruling is *define and document*, because changing it
would break the one thing in the tree that is not the engine's own opinion.
If none exists, the parity story is a guess, and *refuse* is available.

### Task T1e.1.5.3 — Rule, and write it where the CLI's rules live ✅

**Ruled 2026-08-29: refuse.** `--solutions` takes a count of **one or more**;
zero and every negative are a usage error, exit 2:

```text
$ ein solve x.ein -n=0
error: invalid value '0' for '--solutions <N>': invalid solution count: '0' (expected 1 or more, or --exhaustive)
```

Three things about the shape, each of which was a choice:

- **`solutions_spec` wraps `py_int` rather than replacing it**, so a
  *non-integer* keeps `invalid int value: 'x'` —
  [`defined_behaviour.md` §4](../../../docs/kernel/defined_behaviour.md)'s
  error table quotes that line verbatim and uses `--solutions` as its example.
  The validator refuses the **range** and leaves the **type** alone; the test
  asserts both halves, because the second is the one that would rot quietly.
- **The message names the other reading**, the way `jobs_spec` names `auto`:
  *no limit* is `--exhaustive`, and that is what a user who typed `-n 0`
  probably meant.
- **`--help` is untouched**, which is also `--jobs`' precedent — its help
  string says nothing about the refusal either. So `help_surface.rs`'s
  structural golden and the 52-option table in
  [`configuration.md`](../../../docs/kernel/configuration.md) keep their
  shape, and the only doc that moves is the sentence that stated the old
  behaviour.

Written where the CLI's rules live: **§5**, not §4. §4's own text says the
CLI-rejects row *"belongs to §5 rather than here"*, and §5 is where
`--max-set-size 0` was ruled the other way — so the two zero-argument rulings
are one paragraph apart and the asymmetry between them is argued rather than
left to be noticed.

The fixture is the test: a usage error has no `.ein` file, and
`examples/broken/` is for programs the loader refuses.

The task as written:

Recommended, subject to what T1e.1.5.2 finds: **refuse**, with the message
built the way `jobs_spec`'s is — name the flag, state both readings, say
which the tool declines to guess between. It costs one line of validation and
it is the same argument the CLI already makes about `--jobs`, which is
exactly the kind of consistency this milestone is about.

Then record it in `defined_behaviour.md` § 4 next to the other CLI refusals,
add the fixture, and mark [EH-L1](../README.md#the-findings) `fixed` with
this stage named as where the ruling was taken.

## Notes

If the ruling is *refuse*, check whether any corpus entry, script or doc
passes `-n 0` before shipping it — `corpus.toml`'s `runs` columns,
`utils/*.py`, `run_tests.sh`. A refusal that breaks the corpus is a refusal
discovered in CI rather than in the stage that took it.

The Notes' check was run before the refusal shipped and nothing passes a
non-positive `-n`: every `-n` in `corpus.toml` is `-n 3` (eight entries),
`utils/fork_delta_verify.py`'s `SOLVE_RUNS` is the same `solve -n 3`,
`run_tests.sh` passes none, and the only `-n 0` strings in the tree are in
this milestone's own documents.

## What landed

| | |
|---|---|
| the ruling | **refuse** — `--solutions` takes 1 or more; `-n=0` and every negative are exit 2 with *`invalid solution count: '0' (expected 1 or more, or --exhaustive)`* |
| the code | `cmdline::solutions_spec`, which wraps `py_int` so the **type** error keeps §4's documented wording and only the **range** is new; one comment at `solve.rs`'s now-unreachable `max(0)` |
| the test | `ein-cli/tests/cli_semantics.rs::solutions_takes_a_count_of_one_or_more_and_nothing_else`, beside `jobs_takes_a_count_or_auto_and_nothing_else` — the refusal, its exit code, its two named readings, **and** that a non-integer still gets `py_int`'s message |
| the doc | [`defined_behaviour.md` §5](../../../docs/kernel/defined_behaviour.md), one paragraph after `--max-set-size 0`'s opposite ruling, with the asymmetry argued; [`configuration.md` §3.3](../../../docs/kernel/configuration.md)'s budget bullet, which had stated the old behaviour and cited this question as open |
| the parity finding | ein.py did the same thing, from `4c1a5b3^`'s `solve.py:388` + `solver.py:379` — so this is a **deliberate divergence on a behaviour parity never pinned**, and no `from_ein_py/` golden is a `solve` run at all |
| filed | [Q-M1e.17](../open_questions.md#q-m1e17--three-py_int-options-silently-reinterpret-a-negative) — `-m` and `-E` still clamp a negative onto zero, and `-E`'s abort line **prints the clamped number** |
| carried out | [EH-L1](../README.md#the-findings) → **fixed**, here rather than in [S1e.4.4](../p1e.4_low/s1e.4.4_error_handling.md), per T1e.1.5.3 |
| not changed | `--help`, the 52-option table's shape, `defined_behaviour.md` §4, and no golden |

**No golden moves.** `help_surface.rs` renders `{short, metavar, arity,
default, choices, group, help}` and a `value_parser` is in none of them; the
corpus sweep passes `-n 3`; and stdout for every accepted invocation is
byte-identical to what it was.

**What this stage did not do, and left where a reader will find it.** The
zero-argument question is genuinely two flags wide and the second flag's answer
is *not* the same: `-m 0` is a defined truncation and refusing it would delete
an M1d behaviour. So `-m` keeps its zero, its negative is
[Q-M1e.17](../open_questions.md#q-m1e17--three-py_int-options-silently-reinterpret-a-negative),
and the tree ignoring the cap altogether stays
[CO-H3](../p1e.2_high/s1e.2.1_correctness.md)(a). All four cells of that
two-by-two are measured in T1e.1.5.1 above, which is the confirmation
[T1e.4.4.1](../p1e.4_low/s1e.4.4_error_handling.md) asked for and could not
make before both rulings existed.
