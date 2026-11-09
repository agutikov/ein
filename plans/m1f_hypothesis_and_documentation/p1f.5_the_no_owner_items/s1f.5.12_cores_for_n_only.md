# S1f.5.12 — A run takes a core only for the nodes a surface prints: N's members, after the walk

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.12**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).

**Status:** specified 2026-11-08 on the user's ruling of that day on P-15. **Running since 2026-11-09**, on the
user's go-ahead for P1f.5 of 2026-11-08 and the instruction of 2026-11-09 (*"just continue with default decisions
till the end of the phase"*), beside the second lane's S1f.5.10: T1f.5.12.1, every ask, writer and move listed (`87b7cb5a`, [its
record](#t1-record-2026-11-09)); T1f.5.12.2, the two asks and N's cores after the walk ([its record](#t2-record-2026-11-09)). The coordinator handed the stage a second item on 2026-11-09, `--dump-answer` on a
`.einb` ([§ Task T1f.5.12.4](#task-t1f5124--the-answer-directory-of-a-einb-w43)).
**Depends on:** nothing. It is independent of the grammar stages. [S1f.5.13](s1f.5.13_large_dumps_to_the_slow_tier.md)'s
four dumps each get cheaper with it, but neither stage waits for the other.

## The ruling

The agent put P-15 with three options, after explaining it. A run past `--region-limit` under `--dump-answer`
searches for a core at every dead node it visits. It then keeps each node as its two bits, so `nogoods.json`
writes every member of N `not-kept`.
- (a) Keep N's cores.
- (b) Take none.
- (c) Both: no search during the walk, and after it, a core for each member of N only.

The agent recommended (c), as what the ruling of 2026-11-05 already says (S1e2.2.21, *minimum work per
question*): a core is taken only where a surface prints one. The user, 2026-11-08: *"P-15 - c"*.

## The case

| program | the run | today |
|---|---|---|
| `lattice/06_friends_tables` | `ein solve FILE --dump-answer D` | 65 022 core searches, 0.58 s of 2.55 s, no surface prints them; `nogoods.json`: 68 of 68 `not-kept` ([S1e2.3.9 § Notes](../../m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.9_a_core_the_run_did_not_take.md#notes), 2026-11-06; not re-taken) |
| `ein-bugs/wrong-answers/past-limit-lines` | the same | stops at the limit (`n` 17, `limit` 16), 3 state files, 1.23 s; `nogoods.json`: 15 of 15 `not-kept` (2026-11-08, `916d97ac`, release) |

**Where.**
- [`modes.rs`](../../../ein.rs/crates/ein-cli/src/modes.rs), line 232: `opts.cores` is one switch. It is on
  for `--dump-answer`, `--dump-lattice`, `--events` and `--trace`.
- [`region.rs`](../../../ein.rs/crates/ein-infer/src/region.rs), line 1367: with it on, the node function
  searches at every dead node it visits.
- [`explore.rs`](../../../ein.rs/crates/ein-infer/src/explore.rs), lines 1020–1028: past the limit, every
  member of N met is written with an empty core and `CoreStatus::NotKept`.

**The four surfaces do not ask for the same cores:**

| surface | prints a core for |
|---|---|
| `--dump-lattice`'s `nodes.json` | every node visited |
| `--events`' `enter` lines | every node visited |
| `--dump-answer`'s `nogoods.json` | N's members |
| `--trace`'s *N — minimal dead sets* | N's members ([`trace/region.rs`](../../../ein.rs/crates/ein-render/src/trace/region.rs), line 190) |

So the last two pay for a search at every dead node and print a few. The pattern that fixes it is already in
`region.rs` (lines 521–539). Where no surface asks for cores, a run takes the minimal holes' cores after the
walk, *each through the node function, against the same snapshot, so each is the core an eager run takes*.

## What it changes

- **Two asks where there is one.** `opts.cores` splits:
  - *every node's core*, asked for by `--dump-lattice` and `--events`, which is today's behaviour;
  - *N's cores*, asked for by `--dump-answer` and `--trace`.
- **N's cores are taken after the walk.** Under the second ask, the walk takes none. After it, each member of
  N met is run through the node function against the same snapshot, with cores on, as the minimal holes are.
  This holds within the limit and past it.
- **Past the limit, every member gets its core.** `nogoods.json` writes each member's core, status and
  rules: 68 on `lattice/06`, 15 on `past-limit-lines`. `--trace` prints them.
- **`not-kept` retires** (default). `explore.rs` line 1026 is its one producer, and after this stage nothing
  writes it. The status goes with it: `CoreStatus::NotKept`, its word, `trace/region.rs`' line and the
  `not-kept` branch of `answer_round_trip.rs`' `members_of_n`. Step 1 checks there is no other writer. If one
  is left, the status stays for it alone.
- **The cost.** The walk loses one search at every dead node that is not a member of N: 65 022 − 68 on
  `lattice/06`. It gains one saturation and one search per member. Step 1 takes the cost before and after,
  under `utils/bench_env.sh`, on every program past the limit and on the corpus's ten slowest
  `--dump-answer` runs.
- **What does not move.** No answer, count or register row. Within the limit, N's cores are the cores an
  eager run takes, by the node function's construction. A test holds that, below.

## The method

### Task T1f.5.12.1 — every ask and every writer listed

Each of the four options, and the nodes whose cores it prints. Every writer of `not-kept`, and every golden
that holds one. The cost before, as above.

### T1 record, 2026-11-09

**Every ask**, read off the code at `114e8f26`: one switch, `explore::Options::cores` (`region::Options::cores` and
`downset::Options::cores` beneath it), on for any of four options, and each surface prints the cores of other nodes:

| surface | prints a core for | taken |
|---|---|---|
| the read-out, `--json-summary` and `--dump-answer`'s `answer.txt` and `answer.json` | root, and on the cube each minimal hole (`unsat core`, `minimal hole`, `region.core`, `region.holes.minimal[].core`) | on every run, the switch on or off |
| `--dump-answer`'s `nogoods.json` | each member of N describe holds | at every dead node the walk visits, the switch on |
| `--trace`'s *N — minimal dead sets* | the same | the same |
| `--dump-lattice`'s `nodes.json` | every node the walk visits | the same |
| `--events`' `enter` lines | every node the walk visits | the same |
| `ein oracle --dump`'s `region.json` | every node of the cube | `region::Options::cores`, on with `--dump` |
| `ein test --events` | every node the walk visits | `explore::Options::cores`, on with `--events` |

Only describe prints N (`ReadOut::described` is describe's alone), so under `--minimal`, `--maximal`, `--single` or
`-n N`, `--dump-answer` and `--trace` print no member's core and took one at every dead node all the same.

**Every writer of `not-kept`: one**, `Space::partial_cube` in [`explore.rs`](../../../ein.rs/crates/ein-infer/src/explore.rs),
describe past `--region-limit` on a flagged program. Its readers: `region_answer::nogoods` (a `null` core and rules),
`trace::region`'s *Core: not kept: the run stopped at `--region-limit`.*, `explain::CoreStatus`'s `word`, `from_word`
and `not_kept`, and `region_doc`'s reading of an oracle dump through `from_word`, which never holds it. The tests
that hold it: `cli_semantics`' `a_core_the_run_did_not_keep_says_so_and_no_member_of_n_has_an_empty_core`,
`answer_round_trip`'s `members_of_n`, `corpus_cores`' `not_kept` count (held at one or more) and `explain_semantics`'
`a_core_status_is_one_word_read_back_as_written`. No checked-in golden holds the word; `corpus_shapes.md5` holds it in
four digests (below). The register records none: its core statuses leave it out (`ein_corpus::register`).

**And a writer the spec did not name: the state route's N went through a second preparation of root.** The state
route reads N off its maximal alive states and saturates few of its members. For each other member,
`region_answer::nogoods` and `trace::region` called `region::Nodes::new`, which prepares the already-saturated root
again, then `Nodes::refutation`. That is the mechanism `W10/second-preparation`'s `open_because` names. A scratch probe
(`build/s1f.5.12/s1f512_probe.rs`, gitignored, run as an `ein-infer` example) takes every member of N three ways: the
node function against the walk's snapshot after the walk, `refutation` over a fresh first preparation, and
`refutation` over the second. The walk's snapshot gives the first preparation's core, status and rules on every
member, 366 of 366 on `zebra2.ein` and on `zebra2-obligations.ein` and 73 of 73 on `zebra2-hints.ein`, and the second
preparation differs on 19, 19 and 28 of them (2026-11-09).

**The case, re-taken** by `build/s1f.5.12/sweep.py` (gitignored, kept with its logs): every `.ein` under `examples/`,
`tests/`, `stdlib/` and `corpus/`, 426 files, under a bare `ein solve`, `--maximal`, `--minimal` and `--single`, each
with `--json-summary`, `--dump-answer` and `--trace`, and describe at `--region-limit 1` and `2` on every program the
static check flags: 1 762 runs at `114e8f26`, release.
- `lattice/06_friends_tables`: 68 of 68 members `not-kept`, as the case says; `past-limit-lines`: 15 of 15.
- Over the sweep, 92 members are `not-kept`, on 11 runs: those two at the default limit, and nine runs at the lowered
  limits, one member each, on seven programs (`core-activator-guard-only`, `forbid-chain-no-absent`,
  `guard-only-first`, `guard-only-second`, `oblig-stdlib-total-owed` and its `-declared` twin at limits 1 and 2, and
  `past-limit-lines` at 2). Five more programs stop at the default limit with no member of N met.

**The cost before**, `ein solve FILE --dump-answer DIR` (`utils/bench_env.sh python3 build/s1f.5.12/cost.py`, one
pinned P-core, best of three, 2026-11-09), on every program past the limit and the sweep's ten slowest describes:

| program | past the limit | best of three | spread | nodes the walk visits | dead, each a core search | members of N printed | `not-kept` |
|---|---|---:|---:|---:|---:|---:|---:|
| `ein-bugs/wrong-answers/past-limit-lines` | yes | 1.19 s | 0.04 s | 65 536 | 65 532 | 15 | 15 |
| `features/01_not_and_absent` | yes | 3.03 s | 0.04 s | 64 655 | 0 | 0 | 0 |
| `features/04_open` | yes | 3.20 s | 0.04 s | 64 761 | 0 | 0 | 0 |
| `lattice/06_friends_tables` | yes | 2.51 s | 0.01 s | 65 299 | 65 022 | 68 | 68 |
| `saturation/square-unique/corner-house` | yes | 2.58 s | 0.17 s | 65 389 | 0 | 0 | 0 |
| `saturation/square-unique/cul-de-sac` | yes | 2.87 s | 0.01 s | 64 923 | 0 | 0 | 0 |
| `saturation/square-unique/terminus` | yes | 3.34 s | 0.09 s | 65 029 | 0 | 0 | 0 |
| `zebra2-minus-15` | no | 31.28 s | 0.41 s | 8 391 | 1 995 | 1 495 | 0 |
| `zebra2-minus-15-obligations` | no | 31.18 s | 0.05 s | 8 391 | 1 995 | 1 495 | 0 |
| `zebra2` | no | 8.88 s | 1.00 s | 374 | 190 | 366 | 0 |
| `zebra2-obligations` | no | 8.85 s | 0.00 s | 374 | 190 | 366 | 0 |
| `zebra` | no | 2.68 s | 0.00 s | 445 | 225 | 458 | 0 |
| `saturation/square-fwd/houses` | no | 2.27 s | 0.03 s | 85 574 | 0 | 0 | 0 |

The dead nodes are `--dump-lattice`'s count of the same walk (its `steps`). On `lattice/06` the walk takes 65 022 core
searches and prints none of them, S1e2.3.9's count, and on the state route's programs it takes one at each dead
extension it saturates (190 on `zebra2`) and the dump takes each other member's through the second preparation. The
five programs past the limit with no member of N met take none, their walks meeting no dead node.

**Every move, on a prototype of T2** (`python3 build/s1f.5.12/diff.py build/s1f.5.12/base.jsonl
build/s1f.5.12/proto.jsonl`, 2026-11-09): 14 runs move, every one in a class, **0 findings**. No exit code, stdout,
stderr, summary or answer file moves but `nogoods.json`, and no trace moves but in its *N*. No line mode moves.

- **`not-kept`: 92 members on 11 runs** each written with its core, `within-budget` and its rules, in `nogoods.json`
  and in `--trace`'s *N*, where they printed `not-kept` and *Core: not kept*.
- **The second preparation: 58 members on 3 programs**, each now taken against the walk's snapshot. `zebra2.ein` and
  `zebra2-obligations.ein`, 19 of 366 each: on 6 the core and the rules move, each core one fact larger; on 12 the
  rules alone; on 1, `{(pet-loc Dog House-4), (smoke-loc Chesterfields House-5)}`, the status, `cut-environments` to
  `within-budget` beside the same five facts. `zebra2-hints.ein`, 20 of 73: the core and the rules, 16 cores smaller
  by one or two facts and 4 of the same size. Each is the first preparation's, by the probe above.
  `W10/second-preparation`'s member is not among them: the walk's snapshot gives it the same 18 facts.

The first prototype also moved `k_c` on the 37 runs past the limit: a name in `partial_cube` shadowed the members of
R it counts. The diff caught it, and the sweep above is the prototype with it fixed.

**The goldens and tests that move** (`cargo test --workspace --release --no-fail-fast` on the prototype, 2026-11-09):
- `ein-render/tests/golden/corpus_shapes.md5`: 5 of 13 801 renderings, `cores[describe]` and `region-trace` of
  `past-limit-lines` and `lattice/06_friends_tables` (past the sweep's limit of 10, each member's core where it printed
  `not-kept`), and `cores[describe]` of `zebra2-hints` (the second preparation);
- the four tests that hold `not-kept`, rewritten on the prototype as T2 writes them, and green;
- nothing else fails: `wrong_answers` (`W10/second-preparation`'s 18 facts are what the walk's snapshot gives),
  `corpus_cli`, `read_out_reference`, `read_out_examples` and every other test pass unchanged. No answer, count or
  register row moves, and the live read-out reference has no row whose words move: stdout prints no member of N's
  core.

**Two choices, taken by default** on the user's instruction of 2026-11-09:
[Q-M1f.25](../open_questions.md#q-m1f25--the-state-routes-members-of-n-it-did-not-saturate-whose-preparation-takes-their-cores),
the state route's members of N it did not saturate are taken against the walk's snapshot, (a), the 58 moves above;
and [Q-M1f.26](../open_questions.md#q-m1f26--not-kept-retires-and-what-a-member-with-no-core-prints), `not-kept`
retires with no other writer left, (a), and a renderer handed a member with no core refuses it, which the binary
never does.

### Task T1f.5.12.2 — the split, and N's cores after the walk

The two asks. N's cores are taken after the walk, and `not-kept` retires. A test in
`ein-cli/tests/answer_round_trip.rs`, over every corpus program within the limit: `nogoods.json`'s core,
status and rules for each member of N equal what one run with `--dump-lattice` writes for the same set in
`nodes.json`, the eager core. Its `members_of_n` then holds that every member has a core, past the limit
too.

### T2 record, 2026-11-09

**The two asks** are `region::Cores`, a field each of `explore::Options`, `region::Options` and `downset::Options`
where `cores: bool` was:
- `visited`: every dead node the walk visits takes its core as the walk visits it, today's behaviour;
- `nogoods`: each member of N a description holds gets its core **after the walk**, through `region::cores_after`, the
  node function's own fork of root resumed from the walk's snapshot with the walk's plans, fanned out as the walk is,
  narrating nothing. A member the walk took a core for and kept keeps it.

`Cores::ALL` is the library's default and `Cores::READ_OUT` takes root's and the minimal holes' alone, which every run
takes. `ein solve` asks `visited` under `--dump-lattice` or `--events`, and `nogoods` under `--dump-answer` or
`--trace` on describe alone, since only describe prints N: a line mode and `--count` take no member's core where they
took one at every dead node. `ein oracle --dump` asks `Cores::ALL`, `ein test --events` `visited`, and a claim's check
`READ_OUT`, as before.

**Where each route takes N's cores after its walk:**
- the cube (`region::compute_prepared`): with the minimal holes', one fan-out of the masks in the walk's order, where
  the walk took none;
- route 1 (`downset::compute_prepared`): each dead point, every one a member of N;
- the state route: `states::compute_resumable` hands back the walk's pool, snapshot, plans and axes
  (`region::AfterWalk`), and `explore::describe_states` takes each member with no core once `states::describe` has
  read N off the maximal alive states, the members the route never visited among them ([Q-M1f.25](../open_questions.md#q-m1f25--the-state-routes-members-of-n-it-did-not-saturate-whose-preparation-takes-their-cores));
- past the limit (`Space::partial_cube`): each member met, the walk keeping a node as its two bits.

**`not-kept` retires** ([Q-M1f.26](../open_questions.md#q-m1f26--not-kept-retires-and-what-a-member-with-no-core-prints)):
`CoreStatus::NotKept`, its word, `NOT_KEPT` and `not_kept()` go, and with them `trace::region`'s two sentences, *Core:
not kept: the run stopped at `--region-limit`.* and *Core: not kept by this route.* `region_answer::nogoods` and
`trace::region` print each member's core as the run took it and refuse a member with none, and neither prepares root
again: the second preparation is gone from every surface.

**What moved** is T1's list and nothing else. `corpus_shapes.md5`'s 5 renderings (`EIN_BLESS=1`, the manifest's diff
read before it was kept: those 5 lines and no other); `nogoods.json` and `--trace`'s *N* on T1's 14 runs; and the
register's `W10/second-preparation`, whose `now` is unmoved and whose `open_because` named the second preparation as
what decided its member's core. It is restated: the walk's snapshot gives the same 18 facts at `114e8f26`, as a fresh
first preparation does, and with the alternatives cap raised to 1 000 000 both give the 17 its `right` states (a
scratch build, `build/s1f.5.12/s1f512_member.rs`, 2026-11-09), so the cap decides it all and the row stays open, F22's.
The kernel pages that said `not-kept` say what replaced it in the same commit:
[`answer_dump.md`](../../../docs/kernel/inference/answer_dump.md)'s `nogoods.json` paragraph and its file list,
[`glossary.md`](../../../docs/kernel/glossary.md)'s core statuses and
[`region_dump.md`](../../../docs/kernel/inference/region_dump.md)'s `core_status` row. The live read-out reference has
no row whose words move.

**The tests** (each run on the commit's build, release, 2026-11-09):
- `ein-cli/tests/answer_round_trip.rs`: `members_of_n` holds every member to a core, its search's status and its rules,
  past the limit too; and `eager_cores`, on every program the run describes within the limit, runs `ein solve FILE
  --dump-lattice` and holds each member that run visits to the core, status and rules `nodes.json` writes for it: 917
  members over the eight shards, each shard held to at least one.
- `ein-infer/tests/explore.rs`'s `a_run_that_prints_no_core_takes_none_and_answers_alike` asks describe a third time,
  with N's cores alone (`Cores { visited: false, nogoods: true }`): the same answer, every member with a core and the
  core, status and rules the run that takes every core gives it, 1 228 members (held to at least 100).
- `ein-cli/tests/cli_semantics.rs`' `every_member_of_n_has_its_core_past_the_limit_too`, where
  `a_core_the_run_did_not_keep_says_so_and_no_member_of_n_has_an_empty_core` was: `lattice/06` 68 of 68 and
  `past-limit-lines` 15 of 15 each with a core past the limit, in `nogoods.json` and `--trace`; the same 15 with the same
  cores where `--region-limit 17` exhausts; and each the core `--dump-lattice` writes at that limit.
- `ein-infer/tests/corpus_cores.rs` reads N as the run gives it, every member held against a fresh preparation, a
  member with no core a failure, and counts the members past the limit where it counted `not-kept` (15 per commit).
- `ein-infer/tests/explain_semantics.rs` holds `not-kept` read back as no status.

### Task T1f.5.12.3 — the cost after, and the pages

The cost after, beside step 1's. The pages:
- [`answer_dump.md`](../../../docs/kernel/inference/answer_dump.md)'s `nogoods.json` row: each member's core
  is taken after the walk, and is the core an eager run takes;
- the live read-out reference, [`read_out.md`](../../../docs/kernel/inference/read_out.md) (S1f.5.16), where a row's words move, and
  [`answer_dump.md`](../../../docs/kernel/inference/answer_dump.md)'s list of core statuses; M1e2's `read_out.md`
  § 6.2 and § 8.2 are a record, and are not edited;
- [`glossary.md`](../../../docs/kernel/glossary.md)'s list of the core statuses;
- [`region_dump.md`](../../../docs/kernel/inference/region_dump.md)'s note that the oracle never writes
  `not-kept`;
- `CLAUDE.md`'s S1e2.3.9 paragraph;
- the inventory's P-15, which says what fixed it.

### Task T1f.5.12.4 — the answer directory of a `.einb` (W43)

**Handed to the stage by the coordinator on 2026-11-09**, from S1f.5.5's T1f.5.5.13, which found it and left it: `ein
solve X.einb --dump-answer DIR` exits 2, refused before the run with `error: --dump-answer DIR: a .einb container
carries no program source, and every file the answer directory holds starts with it — run it on the .ein`. A `.einb`
stores its program, resolved and import-flattened: since T1f.5.5.13 the `render` views and `saturate` read it through
`ein-cli`'s `common::read_program`, its positions naming `<program stored in x.einb>` (Q-M1f.15). AGENTS.md says every
command that takes a `.ein` takes a `.einb`, and the one-tool rule says two paths may not disagree. It touches the
dump code T1f.5.12.2 touches.

A refusal where a run should answer is a wrong answer, W40's six rows the precedent. So, in two commits: **T1f.5.12.4
files it** as a register row from the stage's block, W43, `open` and owned by this stage, its `now` the refusal; and
**T1f.5.12.5 fixes it**: the dump writes every file from the program the container stores, unless something makes that
impossible, in which case the record says why and the refusal's line stays, in Ein's words.

## Acceptance

- With `--dump-answer` or `--trace`, and neither `--dump-lattice` nor `--events`, a run takes no core at a
  dead node that is not a member of N.
- `nogoods.json` and `--trace` give every member of N its core, status and rules, past the limit too.
- Every such core equals the core an eager run takes, held by a test over the corpus.
- `not-kept` is written nowhere, or only where step 1 found it still meant.
- No answer moves, and `./run_tests.sh` is green.
