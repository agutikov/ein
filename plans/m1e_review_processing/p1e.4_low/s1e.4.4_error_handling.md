# S1e.4.4 — Error handling (Low)

**Phase:** [P1e.4](README.md) (Low)
**Estimate:** 0.5 days
**Depends on:** [Q7](../p1e.1_open_questions/s1e.1.5_cli_semantics.md) — the
`-n 0` ruling is taken there; this stage carries it out.
**Findings:** [`EH-L1`](../review/error-handling/low.md),
[`EH-L2`](../review/error-handling/low.md).

## Context

> **`EH-L1` is done — 2026-08-29, in
> [S1e.1.5](../p1e.1_open_questions/s1e.1.5_cli_semantics.md) itself.** The
> ruling was *refuse*, and T1e.1.5.3 says the ruling stage carries it out: the
> validator, the test and the doc paragraph were one commit's worth and
> splitting them from the ruling would have left `configuration.md` stating
> the old behaviour for the length of two phases. What is left for this stage
> is `EH-L2` and the confirmation in T1e.4.4.1's last paragraph, which now
> waits only on [CO-H3](../p1e.2_high/s1e.2.1_correctness.md)(a).

**`EH-L1` — `-n 0`.** Accepted, with `stop_after: Some(0)` and no stated
meaning, twenty lines from a `--jobs 0` refusal whose message argues that a
flag with two readings should be refused
([`ein-cli/src/solve.rs:570-574`](../../../ein.rs/crates/ein-cli/src/solve.rs),
[`cmdline.rs:171-179`](../../../ein.rs/crates/ein-cli/src/cmdline.rs) against
`:20-47`). The ruling is
[S1e.1.5](../p1e.1_open_questions/s1e.1.5_cli_semantics.md)'s; the
implementation, the test and the doc line are this stage's.

**`EH-L2` — five bytes against eight.** A `--no-default-features` build
sniffs **5** magic bytes
([`common.rs:107`](../../../ein.rs/crates/ein-cli/src/common.rs)) where
`is_einb` requires **8**
([`header.rs:14, 164-166`](../../../ein.rs/crates/ein-einb/src/header.rs)). So
a file beginning `EINB\0xyz` is refused as *"a .einb container and this build
has no einb feature"* in one build and treated as (invalid-UTF-8 or
parse-error) text in the other — a behavioural divergence between the two
shipped feature sets, on garbage input.

Two findings, one theme: **the CLI's behaviour when the input or the argument
is not what it expects**, which is the surface a generated program or a
pipeline hits first.

## Acceptance

- ~~`-n 0` does what
  [S1e.1.5](../p1e.1_open_questions/s1e.1.5_cli_semantics.md) ruled, is pinned
  by a test, and is stated in
  [`defined_behaviour.md` § 4](../../../docs/kernel/defined_behaviour.md).~~
  ✅ **Done 2026-08-29 in S1e.1.5** — and in **§5**, not §4: §4's own text
  says the CLI-rejects row belongs there, and §5 is where `--max-set-size 0`
  was ruled the other way.
- The two magic-byte literals are **one constant**, so the two feature sets
  cannot diverge again.
- If the `-n 0` ruling is *refuse*, nothing in the corpus, `utils/` or
  `run_tests.sh` passes it — checked before the refusal ships.

## Tasks

### Task T1e.4.4.1 — `EH-L1`: carry out the `-n 0` ruling ✅

**Done 2026-08-29 in
[S1e.1.5](../p1e.1_open_questions/s1e.1.5_cli_semantics.md).** *Refuse* was
the ruling: `cmdline::solutions_spec` takes 1 or more, the message names both
readings (`expected 1 or more, or --exhaustive`), the fixture is
`cli_semantics::solutions_takes_a_count_of_one_or_more_and_nothing_else` —
a usage error has no `.ein` file — and the grep was run before it shipped:
every `-n` in `corpus.toml` and `utils/` is `-n 3`.

**The last paragraph's four cells are measured** (S1e.1.5 T1e.1.5.1): the
lattice makes `-m 0` a truncation and the tree ignores it, on both `-m 0` and
`-e -m 0`. So the confirmation this task asks for is *half* available now and
the other half is [CO-H3](../p1e.2_high/s1e.2.1_correctness.md)(a)'s to
deliver. What the measurement added is a third column the review did not
have: a **negative** `-m` is clamped onto 0 silently, and so is a negative
`-E` — [Q-M1e.17](../open_questions.md#q-m1e17--three-py_int-options-silently-reinterpret-a-negative).

The task as written:

Implement whichever
[S1e.1.5](../p1e.1_open_questions/s1e.1.5_cli_semantics.md) took:

- **Refuse** — validation in the same place `--jobs`' lives, a message in the
  same form (name the flag, state both readings, decline to guess), a fixture,
  and a row in `defined_behaviour.md` § 4's error table. Then grep
  `corpus.toml`, `utils/*.py` and `run_tests.sh` for `-n 0` / `--max-models 0`
  before shipping it.
- **Define** — the meaning in `--help`, in § 4, and a test that pins the
  resulting verdict, `k` and exit code for the three arms
  ([S1e.1.5](../p1e.1_open_questions/s1e.1.5_cli_semantics.md) T1 recorded
  them).

`-m 0` travels with it: the lattice honours it as a truncated no-op and the
tree ignores it, which is a divergence
[CO-H3](../p1e.2_high/s1e.2.1_correctness.md)(a) fixes. Confirm both flags
read the same way after both stages land — that confirmation is the actual
deliverable, since two flags and two traversals is four cells and the review
only checked three.

### Task T1e.4.4.2 — `EH-L2`: one magic constant

Export the magic bytes from `ein-einb`'s header module and have
`ein-cli/src/common.rs` use it — noting that the point of the 5-byte sniff is
presumably to work **without** the `einb` feature, so the constant may need to
live somewhere both builds can see it, or be duplicated with a test that they
agree. The second is acceptable here and is the case
[AR-M1](../p1e.3_medium/s1e.3.4_architecture.md)'s rule covers explicitly:
compared by a test, since a shared crate for four bytes is worse than the
problem.

Add the fixture: a file beginning with the magic bytes but otherwise garbage,
asserted to produce the same refusal in both feature configurations. The
`--no-default-features` build is already exercised by a release-workflow leg
that has never run ([TE-L5](s1e.4.5_tests.md)), so this is one of the few
places where the two feature sets are compared at all.

## Notes

Half a day covers both because the decisions are made elsewhere. If
[S1e.1.5](../p1e.1_open_questions/s1e.1.5_cli_semantics.md) has not run, this
stage does `EH-L2` and waits — implementing a ruling that has not been taken
is how a Low finding becomes a behaviour change nobody agreed to.

---

## ✅ Done 2026-09-01 — three sniffs, not two, and the third had no second arm

**`EH-L1` was done at [S1e.1.5](../p1e.1_open_questions/s1e.1.5_cli_semantics.md)**,
as this stage's Context already recorded. **`EH-L2`: fixed**, and it is larger
than reported.

### The finding, and the half it did not see

`common.rs` sniffed **five** magic bytes where `ein_einb::is_einb` requires
**eight**, so `EINB\0xyz` was *"a `.einb` container and this build has no
`einb` feature"* in a `--no-default-features` build and a parse error in the
default one. As reported.

**There is a third sniff**, at
[`solve.rs`](../../../ein.rs/crates/ein-cli/src/solve.rs), and it was
`#[cfg(feature = "einb")]` with **no `not(einb)` counterpart** — so a light
build's `ein solve` never reached the container refusal at all:

```
$ ein solve real.einb          # --no-default-features, before
UnicodeDecodeError: 'utf-8' codec can't decode bytes in '…/real.einb'
exit=1
$ ein solve real.einb          # after
kb load error: …/real.einb is a .einb container and this build has no `einb` feature
```

That is the promise `ein-cli/Cargo.toml`'s own feature comment makes — *"a
`.einb` argument is refused by the loader that would have opened it, which is
what a build with no container in it should say"* — and `ein solve` is the
subcommand most likely to be handed one.

### One predicate, one constant

`common::looks_like_einb` is the only sniff in the crate now, and
`common::EINB_MAGIC` the only literal. It is a **second copy** of
`ein_einb::header::MAGIC` on purpose: `ein-einb` is an optional dependency and
the light build must still recognise a container in order to refuse it, which
is exactly the state [`AR-M1`](../p1e.3_medium/s1e.3.4_architecture.md)'s rule
permits for a pair too small to share a crate — *compared by a test*.

Two more five-byte literals were in the **test** file
(`einb_cli.rs`), and its helper was called `ein_is_text` while returning *is
einb*. Both fixed; the helper is `looks_like_einb`.

### What holds it, and the asymmetry that is worth stating

| | where | which build |
|---|---|---|
| the constants agree | `einb_cli::the_two_magic_constants_agree` | the default one — it is the only build where both exist |
| the **behaviour** agrees | `cli_semantics::a_file_that_starts_like_a_container_but_is_not_one_is_text_in_either_build` | both — which is why it is not in `einb_cli.rs`, whose whole file is `#![cfg(feature = "einb")]` |

The fixture uses a **five-byte** prefix deliberately. A file with the full
eight-byte magic legitimately parts ways between the builds — one opens it, the
other says it cannot — and that divergence is the feature, not the defect.

**The acceptance line *"the two feature sets cannot diverge again"* is met by
construction, not by a test**, and saying so is part of the disposition: no
local runner and no per-commit CI step builds `--no-default-features`
([TE-L5](s1e.4.5_tests.md) is the same absence one level up). What stands in
its place is a run, dated: `cargo test --workspace --no-default-features`
against a scratch `CARGO_TARGET_DIR`, 2026-09-01 — **676 tests, 2 failures,
both artifacts of the out-of-tree target directory** (`imports_semantics`
records the stdlib's resolved path, which moves with the target dir). Both fail
identically with **default** features in the same directory, which is the
control. That out-of-tree requirement is itself why nothing runs this leg
locally: in-tree it clobbers the default build's artifacts.

### T1e.4.4.1's last paragraph — the four cells

`-n 0` and `-m 0`, lattice and tree, is what the task asked to confirm after
both stages landed. It is confirmed: `-n 0` is **refused** on every arm
(S1e.1.5, exit 2), `-m 0` is a truncation the lattice honours and the tree
**refuses** (S1e.2.1's `CO-H3`, exit 2 with a stderr `error:`) — so neither
flag has two readings any more, which is the argument `--jobs 0`'s message
made and the reason the review paired them.

**Gate:** `cargo test --workspace` — **810 tests, 0 failures**. No golden moved.
