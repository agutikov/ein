# S1e.5.1 — The configuration reference

**Phase:** [P1e.5](README.md) (Documentation ein does not have)
**Estimate:** 3 days
**Depends on:** nothing to start. Its *content* is affected by
[P1e.1](../p1e.1_open_questions/README.md) — Q5 may reclassify
`enable-pre-branch-lookahead` and Q7 defines or refuses `-n 0` — so the page's
two most interesting rows are written last.
**Blocks:** nothing.

## Context

There is no configuration reference. The surface a user can set is:

| surface | count | where it is stated today |
|---|---:|---|
| `(config …)` flags | **17** | `FIELDS` in [`config.rs`](../../../ein.rs/crates/ein-core/src/config.rs), plus twelve Rust doc comments of one line each and four fields with none. `(config …)`'s *shape* is in [`01_grammar.md:58`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md); its flags are nowhere |
| CLI options | **50** across four subcommands (`solve` 33, `test` 10, `saturate` 6, `render` 1) | `--help`, and nothing else |
| `EIN_*` environment | **~14 live**, of ~24 greppable names | scattered; six are listed in [`AGENTS.md`](../../../AGENTS.md)'s gate block, `$EIN_STDLIB` in [`docs/install.md`](../../../docs/install.md), and the rest in the source that reads them |

Eight of the fifty CLI options **shadow** a config flag (`-L`, `-K`, `-o`,
`-y`, `-z`, `-d`, plus `-p`/`-s` presentation pairs), and the precedence
between the three surfaces is stated in exactly one place — `config.rs`'s
module doc — as *"an explicit `solve(config=…)` argument, then `kb.config`
from the IR, then these defaults"*, which names a Python API that no longer
exists.

## Acceptance

- **One page**, `docs/kernel/configuration.md`, linked from
  [`docs/kernel/README.md`](../../../docs/kernel/README.md), covering all
  three surfaces and the precedence between them.
- **Every row carries four columns beyond the name:** default, *what it
  changes*, **does it change the answer**, and **stability**. The third is the
  page's reason to exist and the fourth is what keeps it from blessing a
  probe.
- **The flag table is pinned by a test.** A flag added to `FIELDS` without a
  row here fails `cargo test`, the way a stdlib rule without a `tests/stdlib/`
  program fails `stdlib_coverage.rs` and a `.ein` without a corpus entry fails
  the completeness check. Mechanism in T3.
- **The live `EIN_*` set is enumerated and classified** — read by the shipped
  binary / read by the test harness / read by `utils/` scripts / not an
  environment variable at all. The last class is not empty and naming it is
  part of the result.
- `AGENTS.md`'s gate block and `docs/install.md` are **reconciled** with the
  page in the same commit, or the page states where they disagree and why.
- No claim in the page is a number this stage counted by hand and nothing
  re-counts.

## Tasks

### Task T1e.5.1.1 — Enumerate all three surfaces, from the source of truth

- **Flags:** `FIELDS` is the list, in declaration order, and `rendered_fields`
  is its already-diffed twin. `ein --dump-config` prints them; use that as the
  extraction path rather than parsing Rust.
- **CLI:** `--help` per subcommand. Note that `ein-cli`'s help shape is
  already golden-pinned
  ([`golden/help_shape.txt`](../../../ein.rs/crates/ein-cli/tests/golden/help_shape.txt)),
  so the option list has an owner and the page should cite it rather than
  compete with it.
- **Environment:** grep, then **verify each name by reading the site**. The
  known trap is that `EIN_RS` appears in two `utils/` scripts as a Python
  local (`EIN_RS = Path(os.environ.get("EIN_BIN", …))`) and is not an
  environment variable at all; a grep-only census would list it.

### Task T1e.5.1.2 — The two judgement columns

**Does it change the answer?** Three values, and each needs its evidence
named:

- **no** — cost only. The claim is checkable and mostly already checked: the
  corpus lever sweeps (`levers = ["-L", "-K", "-y", "-o score-sum"]` in
  [`corpus.toml`](../../../corpus/corpus.toml)) run entries both ways.
- **yes** — `enable-pre-branch-lookahead`, on the review's Q5 evidence: it
  decides which states are `complete`, and `lattice/02` goes from
  `Ambiguity k=3` to an **exhausted** `Contradiction k=0` under `-L`. Whatever
  P1e.1 rules, this row says *yes* and links the ruling.
- **presentation only** — `print-alive`, `--models`, `-p`/`-s`.

There is a fourth value the stage may need and should not invent early:
`enable-lookahead-kill-cache` does not change `k` or the verdict, but it
**does** change the recorded fact set — with `-K` the models of `lattice/02`
lose their `(not (c-prop X))` facts, because the cache is what writes them.
Under [Q-M1e.6](../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)'s
definition a *model* is the positive part, so the **model** is unchanged and
the **solution KB** is not. That distinction is the page's, and it is the
clearest single argument for why the page needs the column at all.

**Stability.** At least: *contract* (documented, tested, safe to depend on),
*probe* (exists to test invariance — `lattice-order-seed`,
`candidate-order-seed`, `lattice-sanity-check`), *measurement lever* (exists
for an A/B — `EIN_OBLIGATION_CHOICE`, `-L`, `-K`), *experimental*
(`EIN_TRAVERSAL`, and [Q-M1e.5](../open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface)
is the question of what that word licenses).

### Task T1e.5.1.3 — Pin it, so it cannot rot

The design constraint is the phase's: **generate or diff, never a third
hand-maintained copy.** Three mechanisms, in preference order:

1. **A marked region diffed by a test** — the
   [`docs/api/rust.md`](../../../docs/api/rust.md) precedent, which works and
   is already trusted: the page's block is the marked region of a test file
   and a test diffs the two. Here the generated half is the flag/default
   table, and the prose columns live outside the markers. *Edit the generator,
   run it, paste; never edit the block by hand.*
2. **A name-set test** — weaker and much cheaper: parse the page's table for
   flag names, assert the set equals `FIELDS`'s. It catches an added flag and
   a renamed one, and not a wrong default.
3. **A `utils/` check** — the census shape, `--check` exiting 1 on drift. But
   `stdlib_census.py --check` is [TE-L4](../README.md#the-findings) — *wired
   to no gate or workflow* — so this is the option that has already failed
   once in this repo.

Take (1) for the table and (2) as its cheap guard; do not take (3) alone.

### Task T1e.5.1.4 — Reconcile the three places that already say some of this

- `AGENTS.md` § Running the gate lists six `EIN_*` names in a code block as if
  they were the set; either it becomes a pointer to the new page, or it says
  *the six an agent needs* explicitly.
- `docs/install.md` documents `$EIN_STDLIB` and its resolution order; the new
  page links rather than restates.
- `config.rs`'s module doc names a Python resolution path (`solve(config=…)`)
  that has not existed since M1a P1a.10. That is
  [MA-M2](../README.md#the-findings)'s class — *stale rustdoc contradicting
  the code it documents* — and it is fixed here because this stage is the one
  reading it.

## Notes

The page should be a **reference**, not a tutorial: one row per knob, no
narrative. The narrative homes already exist —
[`features.md`](../../../docs/kernel/inference/features.md) for what the
levers *do* to the search and what they were measured at, and
[`docs/guide/`](../../../docs/guide/README.md) for the newcomer. A reference
that explains twice is a reference that will disagree with itself by M2.

Four flags carry no doc comment at all in `config.rs`
(`print-alive`, `hypgen-rel-weight`, `hypgen-obj-weight`,
`lattice-sanity-check` has one line). Writing their rows means reading their
call sites, and that is where this stage will find whatever it finds — the
same way [S1e.1.1](../p1e.1_open_questions/s1e.1.1_search_soundness_probes.md)
found that `most-constrained` returns `0.0`. Anything found gets a
`Q-M1e.<n>`, not a quiet fix in a doc stage.
