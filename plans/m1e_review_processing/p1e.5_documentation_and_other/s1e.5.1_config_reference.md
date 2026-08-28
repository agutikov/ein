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
same way [S1e.1.1](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md)
found that `most-constrained` returns `0.0`. Anything found gets a
`Q-M1e.<n>`, not a quiet fix in a doc stage.

---

## What shipped — 2026-08-28, at `4a47aa3`

[`docs/kernel/configuration.md`](../../../docs/kernel/configuration.md), six
sections, linked from
[`docs/kernel/README.md`](../../../docs/kernel/README.md) twice (the audience
table and § Cross-references), pinned by
[`ein-cli/tests/config_reference.rs`](../../../ein.rs/crates/ein-cli/tests/config_reference.rs)
— six tests, 0.03 s, in `cargo test --workspace`.

**The gate**: `./run_tests.sh` exit 0, **744 tests**, five static checks clean
(`cargo fmt`, `clippy -D warnings`, `RUSTDOCFLAGS=-D warnings cargo doc`,
`stdlib_manifest.py`, `check_hashmap_iteration.py`). The review's baseline was
**738** and the six new tests are the whole difference — quoted from the run,
as [M1e's acceptance](../README.md#acceptance-for-the-milestone) asks. **The
corpus is unchanged**: the behavioural pins write their fixtures to a temp
directory, so no `.ein` was added and the completeness check has nothing new
to see.

### Acceptance, line by line

| bullet | how |
|---|---|
| one page, linked from the kernel README | `docs/kernel/configuration.md`; both link sites above |
| four columns beyond the name | § 2.2 carries **five**: type, default, what it changes, answer, stability. Type and default are diffed against `FIELDS` and against the generated block |
| the flag table pinned by a test | T3 mechanism **(1)** *and* **(2)**, both taken. (1) is `the_defaults_block_is_the_binarys_own_dump_config`: the marked region **is** `ein solve --dump-config`'s output on a config-less program, with `EIN_BLESS=1` rewriting it in place — better than the `docs/api/rust.md` precedent's *edit, run, paste*, because there is nothing to paste. (2) is `every_flag_has_a_row_and_no_row_is_orphaned` |
| the live `EIN_*` set enumerated and classified | § 4, four classes, and **the last class is nine names** |
| `AGENTS.md` and `docs/install.md` reconciled | both, in this commit — below |
| no hand-counted number | the flag count and defaults are the binary's; the CLI counts, the parser count and the total are the golden's, asserted row by row; the *read by the shipped binary* class is scanned out of the shipping crates in both directions; and § 1's summary table is diffed against all three, because a summary two hundred lines from what it summarises is a parallel copy. **Two** counts are not pinned and say so where they stand: § 2.4's one-lever sweep and § 4's other three environment classes, both dated 2026-08-28. Neither could be a check — the sweep is a measurement, and the fourth class would fail the day someone *proposes* a new variable in a plan file |

### What the enumeration found

Five of the stage's own reconnaissance numbers were wrong, in the direction
that makes the page worth having.

- **52 CLI options, not 50**, and across **eleven** parsers rather than four
  subcommands: `solve` 32, `test` 9, `saturate` 5, `render rule` 2,
  `render lattice` 2, `render rules` 1, `kb save` 1. The stage's 33/10/6/1
  counted `-h` per parser and collapsed `render`'s four sub-parsers into one.
  The page reads the numbers off `golden/help_shape.txt` and the test asserts
  every row against it, so this particular mistake cannot recur.
- **Six options shadow a flag, not eight**, and they reach **five** flags:
  `-L`, `-K`, `-y`, `-o`, `-z`, `-d` — the last two both writing
  `lattice-order-seed`, and `-d` **inert without `-z`**, which is verifiable in
  one command (`ein solve --dump-config x.ein -d 9` prints
  `lattice-order-seed None`). `-p` / `-s` shadow nothing; they are
  presentation.
- **29 greppable `EIN_*` names, not ~24**, of which 8 are read by the shipped
  binary (3 only in a non-default build), 10 by the test harness, 2 by `utils/`
  and the shell — and **9 are not environment variables at all**. The stage
  predicted `EIN_RS`; the class also holds `EIN_CMD` (a shell array),
  `EIN_C_PROBLEM_H` (a C include guard), `EIN_PY` and `EIN_RUN_SLOW` (named in
  comments in the past tense), `EIN_MUTANT` (a frozen measurement's, read by a
  harness deleted at S1a.10.3), `EIN_PRIORITY` and `EIN_NOGOOD_INJECT`
  (*proposed* by two M1e stage files), and **`EIN_RENDER_LEVI`** — documented
  in `04_dot_rendering.md` and never implemented, which is
  [CD-H1](../README.md#the-findings)'s and now has a second statement.
- **Two flags are inert, not probes.** The phase README predicted four probes
  among the seventeen (`candidate-order-seed`, `lattice-order-seed`,
  `lattice-sanity-check`, `print-alive`). Two of them are:
  `lattice-order-seed` moves the traversal on 6 of 29 fixtures and
  `lattice-sanity-check` can abort a run. The other two are read by **no code
  path** — [Q-M1e.10](../open_questions.md#q-m1e10--two-config--flags-are-inert),
  raised rather than fixed, per this stage's own Notes. The probe is banked as
  `the_two_inert_flags_are_still_inert` and the argument is written at the
  site, in `config.rs`'s two new field doc comments.
- **`enable-symmetric-mirror` changes the answer**, and
  [`features.md`](../../../docs/kernel/inference/features.md)'s `1.0×` is a
  different claim. That page measures `zebra2`, where the mirror has a
  transparent rule fallback. `examples/features/06_symmetric_native.ein` is
  the only fixture in the tree that reaches the mirror, and with the flag off
  it derives **0 facts instead of 3** and stops reaching its query goal. This
  is not a `features.md` defect — it says *"inert on this puzzle by
  construction"* — but it is exactly the row a *does it change the answer*
  column exists to get right, and no page had one.

A sixth, small: this file's Context table says the flags are *"twelve Rust doc
comments of one line each and four fields with none"* — sixteen, for
seventeen flags. It was **nine** documented and **eight** not
(`hypgen-rel-weight`, `hypgen-obj-weight`, `print-alive`,
`lattice-sanity-check`, `enable-path-nogoods`, `enable-symmetric-mirror`,
`enable-singleton-writeback`, `enable-forced-positive`). The stage did **not**
write eight new doc comments: the page is the reference now, and a per-field
paragraph in `config.rs` would be the third hand-maintained copy this phase's
first risk names. Two fields got one anyway — `print_alive` and
`candidate_order_seed` — because [Q-M1e.2](../open_questions.md#q-m1e2--may-a-review-finding-be-closed-by-a-comment)'s
rule is that an accepted state is argued **beside the code**, and *this knob
does nothing* is exactly such a state.

### T2's judgement columns, as taken

The stage proposed three answer values and warned against inventing a fourth
early. The reading forced **six**, each with an occupant and a witness:
**yes** (2 flags), **KB only** (1), **order only** (5), **explanation only**
(1), **no** (6), **inert** (2). *Presentation only* turned out to belong to the
CLI section, not to any flag. Stability took five values — the four the stage
named minus *experimental*, which no flag is (`EIN_TRAVERSAL` is the only
experimental surface in the system, and
[Q-M1e.5](../open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface)
is what the word licenses), plus **partly wired** for `hypgen-scoring` and
**inert** for the two.

Evidence, in three tiers: the four flags with a CLI lever are run both ways
over eleven corpus entries on every `cargo test`; the ten-cell matrix on
`zebra2` is `features.md`'s; and a **one-lever sweep of 2026-08-28** — every
flag flipped, over the 29 `examples/` fixtures that state no `(config …)` head
of their own, `solve -e -p` compared byte for byte — is what the page's *no*
rows rest on. § 2.3's dagger records the one place the whole column is scoped:
[Q-M1e.9](../open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent)'s
counterexample moves three rows, because three shipped mechanisms read the
premise it refuted.

### T4 — the three reconciliations

- **`AGENTS.md` § Running the gate** now says in as many words that its seven
  `EIN_*` names are *the ones an agent needs, not the set*, and points at the
  page. Its `docs/kernel/` bullet gained the page, the inertness finding and
  the `EIN_BLESS` re-bank command.
- **`docs/install.md`** — the sentence *"The directory must contain
  `MANIFEST.sha256`; a directory without it is not the stdlib"* sat directly
  under the `$EIN_STDLIB` paragraph and read as a rule about the override. It
  is a rule about the **walk**. The override is unvalidated, which is
  [EH-M2](../README.md#the-findings) and stays EH-M2's to fix; the page and
  install.md now both state the behaviour, with the observable
  (`stdlib     unreadable  $EIN_STDLIB <path>`, then a load error) quoted from
  a run.
- **`config.rs`'s module doc** — the `solve(config=…)` sentence
  ([MA-M2](../README.md#the-findings)'s class) is replaced by the two orders
  that exist: the engine's (`SolveOptions::config` → `kb.config` → defaults)
  and the binary's (CLI shadow → the program's last `(config …)` head, whole →
  defaults), with the note that `ein-cli`'s `resolved_config` computes the
  second and hands it to the first, which is why they never disagree.

### Left for its owner

**`help_surface.rs`'s doc comment says *"50 options across 9 parsers"* and
*"39 of them are ein.py's; the other eleven"*, where its own constants sum to
52 across 11** — M1d's `solve --layer-progress` and `test --json-report` were
added to the arrays and not to the prose above them. That is
[MA-M4](../README.md#the-findings) (*numeric drift across load-bearing in-code
comments*) at a site [S1e.3.9](../p1e.3_medium/s1e.3.9_maintainability.md)
owns, and it is recorded here rather than patched, because a doc stage editing
another stage's file is how two stages come to disagree about one number.
Nothing on a path out of the new page reaches it: the page cites
`help_shape.txt`, and the test compares against the golden, not against the
prose.

Also noted, and too small to file: `utils/feature_matrix.py`'s docstring says
the float-valued knobs are skipped *"which is also why no puzzle can set
one"*. A puzzle can — `(config :hypgen-rel-weight 2)` resolves to `2.0`; what
it cannot write is a non-integral value, because the lexer has no float
literal. The page states the rule correctly.
