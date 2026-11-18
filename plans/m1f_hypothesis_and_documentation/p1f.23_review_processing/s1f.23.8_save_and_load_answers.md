# S1f.23.8 — the user's answers on save and load, built

**Phase:** [P1f.23](README.md)
**Status:** started 2026-11-18 on the user's word (*"process plans/m1f_hypothesis_and_documentation/review/01-save-and-load/ANSWERS.md"*),
in a worktree beside S1f.23.6; **built** the same day (`86d46410`, the ledger after it), on the branch
`s1f.23.8-save-load`; merged after S1f.23.6, and one gate, `utils/ci_container.sh` and one nightly hold both.
**The answers:** the user's file as written, `35719115`
(`git show 35719115:plans/m1f_hypothesis_and_documentation/review/01-save-and-load/ANSWERS.md`); the topic left the
bundle with every item answered (`eaabeada`).

## The answers

| item | the question | the user's answer |
|---|---|---|
| L-106 | `answer_round_trip`'s nightly shards near five minutes | (a), as it is |
| Q-M1f.413 | `ein kb save` of a saved file | **(c)**: *give the container a section for the commitment, so a node's container opens as that node* |
| Q-M1f.415 | `ein show hyps` / `ein render h-space` of a saved file | (a), the program's H |
| Q-M1f.416 | what `ein test` asks of a saved node | **(b)**: *no run at all: each claim NOT CHECKED, no run's data in `--json-report`* |
| Q-M1f.417 | the `commit` row, `root.commit`, `ein-answer/3` | (a) |
| Q-M1f.418 | the nesting limit and a saved file's wrappers | (a) |
| L-104 | `:hypothesis 0` in `ein saturate --dump` | (a), as it is |
| F23 D-24 | the note on a carried standard-library module | *"Drop the note entirely, since the file is self-contained and comparing libraries isn't the loader's business"* |

## What it builds

1. **Q-M1f.413 (c)**: `ein kb save` of a saved file stores its `(commit …)` in a section of the container, and a
   node's container opens as that node: `ein solve x.einb` of a node's container answers what `ein solve` of the
   saved file answers, byte for byte but the path (the container's contract). A root's container stays as it is. The
   container's version moves if its format does (with R-30, the 24 bytes S1f.23.4 took out at the same `einb/2.0`):
   the stage says what the version is and why.
2. **Q-M1f.416 (b)**: `ein test` of a saved file with a `(commit …)` runs nothing: each claim is NOT CHECKED, no exit
   code moves, and `--json-report` carries no run's data for it.
3. **D-24**: the `note:` a saved file's load writes for a carried `std.*` module that differs from the binary's
   library goes, with its code, its tests and its pages; a saved file loads from what it carries and compares nothing.
4. **The ledger**: each answer filed against its question as ruled, the user's words verbatim (`open_questions.md`,
   the milestone's `findings.md` for L-104 and L-106, F23's D-24 row); the decisions Q-M1f.413 and .416 amend
   (Q-M1f.403's file, `ein kb save`'s page, `10-answer-dump.md`, `05-cli.md`, `07-diagnostics.md`) brought to the
   rulings.

## Acceptance

- No answer moves; a saved node's container answers as its saved file, held by a test over the corpus's saved
  nodes (`answer_round_trip`'s or `einb_cli`'s); the claims of a saved node NOT CHECKED with no run (a test).
- The gate, `utils/ci_container.sh` and the nightly, once, with S1f.23.6, on the merged tree.

## Record

### What was built (`86d46410`)

- **Q-M1f.413 (c).** `ein kb save` of a saved file stores the program it carries, its imports from the modules it
  carries (`ein_einb::sections::Carried`, as before), and a node's `(commit …)` in a new section, **`COMMIT`, kind
  13** (`sections::write_commit` / `read_commit`): a count, then each candidate's text as the file writes it, in its
  order. `ein_einb::save_to_vec` writes it from `kb.program().commit`, so a container saved again keeps it; an open
  reads it right after the stored program is loaded and sets `Program::commit`, before `PRESENT` takes the program's
  `Arc`, so the believed KB and a dropped-derived one alike carry it, and every command reads the node as it reads
  the saved file (`saturate`, `render kb`, `oracle`, `solve` in every mode; `show hyps` and `render h-space` set it
  aside by `program_of_saved`). `kb.rs`' refusal of a node is gone. A root writes no `COMMIT`.
- **The version: `einb/2.1`** (`FORMAT_MINOR = 1`, `FORMAT_MAJOR` 2). By `09-binary-format.md` § 4.1 and Q-M1f.10's
  *why*, a section kind added moves the minor, and a reader of the same major skips a kind it does not know
  (`corruption.rs`' `an_unknown_section_kind_is_ignored_rather_than_refused`); the layout of every existing section
  is unchanged. **R-30's 24 bytes move nothing**: `SYMBOLS` is the interner in id order, whatever the kernel interns
  before a program's names, so S1f.23.4's kernel writes a shorter `SYMBOLS` of the same layout, and an earlier
  container opens through § 3's remap to the same answer (the main session's check on R-30). The page says so in
  § 4.1.
- **Q-M1f.416 (b).** `ein test` of a file with a `(commit …)` (a saved node, or a node's container) runs nothing,
  not root's saturation either (`test.rs`' `NotRun`, before the question is chosen): each claim NOT CHECKED with
  Q-M1f.403's line, counted in `Tally::saved`, so no exit code moves. Before and after, `ein test -v
  tests/saved/04_a_saved_node.ein`:

  <!-- output -->
  ```text
  before: NOT CHECKED  tests/saved/04_a_saved_node.ein
            solve · :expect (region …) — NOT CHECKED (a saved file's node: root alone)
              the claim is the program's, …: the program checks it
              outside the universe: flagged — no candidate: no hrule proposes it. …
            :goal (flagged ?x) — NOT CHECKED (--no-search: all of R, 1 class over 1 states)
              the claim is the program's, …: the program checks it
              outside the universe: flagged — …
  after:  NOT CHECKED  tests/saved/04_a_saved_node.ein
            solve · :expect (region …) — NOT CHECKED
              the claim is the program's, about the program's R, and this saved file is a node of its lattice: the program checks it
            :goal (flagged ?x) — NOT CHECKED
              the claim is the program's, about the program's R, and this saved file is a node of its lattice: the program checks it
          1 file, 2 expectations: 0 held, 0 FAILED, 2 not checked, 0 errors, exit 0, both
  ```

  The report's rows: `outcome` `not-checked`; `answer`, `outside`, `route`, `fell_back_from`, `diff` `null`; the
  solve row no `region` (before: `region.route` `"root alone"`, `route` `"--no-search"`, `outside` the list). The
  id stays `ein-test-report/6`.
- **D-24.** The note, its comparison in `ein_ir::saved::load_saved`, `Resolver::stdlib_text` (its one caller),
  `ein_ir::saved::Loaded::notes`, `common.rs`' call, its test (`saved_files.rs`) and its pages
  (`07-diagnostics.md` § 2.30, `10-answer-dump.md`) are gone. `saved_files.rs`' `what_a_saved_file_is_refused_for`
  now holds the absence: a carried `std.math.arith` moved, and the file under `EIN_STDLIB=/nonexistent`, print
  nothing on stderr, exit 0, under `solve`, `saturate` and `test`.
- **Tests.** `einb_cli.rs`' `a_nodes_container_opens_as_the_node` (02's node: `solve` under seven flags byte for
  byte but the path, `saturate --dump`, `test`, `show hyps`, saved again the same bytes, a root's container's
  sections the program's, no `COMMIT`); `roundtrip.rs`' `a_nodes_commitment_is_stored_and_a_roots_is_not` (the
  library: into an empty and a crowded interner, saved again); **`answer_round_trip.rs`' `by_its_container`**, each
  program's last saved node with a `(commit …)` stored and solved against its file, stdout, stderr and exit: 164
  nodes per commit, every shard; `saved_files.rs` and `test_cli.rs` (the row-rebuilt header, the scope lines) for
  (b).
- **The ledger.** `open_questions.md`: Q-M1f.413, .415, .416, .417, .418 entries and rows as ruled, the user's
  words; Q-M1f.401's point 2 amended by D-24; Q-M1f.403's row. `findings.md`: L-104, L-106. F23: D-24 fixed. The
  phase's R-30 decided. Decisions: [Q-M1f.413](../../../docs/decisions/engine/Q-M1f.413-a-nodes-container-opens-as-the-node.md)
  new (engine/); Q-M1f.401, Q-M1f.403 and Q-M1f.10 (its version line) brought to the rulings.

### Readings, each with a default

- **A 2.0 reader opens a node's container as its program's root.** It skips `COMMIT` as any unknown kind, and so
  answers the program's R where the container is a node: a binary built before this stage, handed a node's container
  this one wrote. No 2.0 writer wrote a node's container (`kb save` refused one), so a root's container is read right
  both ways. **Default: the minor, as the page's rule says.** The other way is 3.0: every 2.x container refused with
  the version line, a cache to save again, and no older binary ever opens a node. The user's, if the gap matters.
- **"A root's container is unchanged"** read as: the same sections, byte for byte, and no `COMMIT`; its header's
  minor reads 1. **Default.**
- **The container keeps the saved file's words.** The review's mock of (c) wrote *container* for *saved file* in the
  `commit` row and the NOT CHECKED lines; the container's contract is its file's output byte for byte but the path
  (Q-M1a.22), so the words are the saved file's. **Default.**
- **A load note's position in a container is the stored program's** (Q-M1f.15; F23's D-18), so `by_its_container`
  compares a position as `<position>`; one corpus node printed one, `features/13_mixed_solution_and_open.ein`'s
  mechanism note. The pages say it beside the byte-for-byte promise. **Default.**
- **A saved file's container still records the environment's stdlib digest**, as every container does, so a later run
  under another library warns `… was written under different inputs (StaleStdlib)`, which the saved file, comparing
  nothing since D-24, does not; the answer is the same (a loaded container drops nothing). The review's 17-Q-M1f.413
  § 7 found it. **Default: as is**, the container's freshness being the cache's own and a root's container left
  unchanged; the other way is a saved file's container recording no digest, compared with nothing. The user's.
- **`ein kb save` checks no commitment against H**: it saturates nothing, so a node whose choice is no candidate is
  stored, and refused by every command that reads the node, as the file is. **Default: as is.**
- **`kb save --saturate` of a node banks root's fixpoint and the commitment**, not the node's state; the run commits
  the node on open, as for the file. **Default: as is.**
- **`ein test` refuses no bad commitment now.** With no run, H is never built, so a `(commit …)` naming no
  candidate, or one twice, is NOT CHECKED with exit 0 under `ein test` (it was an ERROR, exit 2, under (a));
  `solve`, `oracle`, `saturate` and `render kb` still refuse it, and `07-diagnostics.md` § 2.30 says `ein test`
  refuses neither. **Default: as built**, the ruling being *no run at all*.
- **`--json-summary` and `--events` under `ein test` of a node write nothing**, as for a file that states no claim:
  the option (b) left them unsaid. **Default.**
- **The `-v` header names no question** (the mock's *(a saved file's node: not run)* not taken): the parenthesis
  names the question an outcome was read off, and the report row the header is rebuilt from has no field to tell a
  node's row from a goal-less query's. **Default.**
- **The report: `outside` `null`, not the mock's `[]`**, and a solve row with no `region` key, not `region: null`:
  both are the shape of a row no run checked today (`[]` would say H leaves nothing out, which only a run knows), so
  no key moves and the id stays `ein-test-report/6`, as Q-M1f.259 keeps an id. `06-config.md` is unchanged.
  **Default.**
- **`claim::check`, the library's, keeps its saved-file branch**: `ein test` does not call it, and a library caller
  checking a node's claim gets NOT CHECKED as `ein solve` does. **Default: unchanged.**
- **No decision file for Q-M1f.415, .416, .417, .418**: .416's answer is Q-M1f.403's file, .415's and .417's row
  Q-M1f.402's, .418's Q-M1f.185's; .413 has its own (engine/). **Default.**
- **`answer_round_trip`'s cost**: one `kb save` and two `ein solve`s per program on its last node. Not measured
  against the nightly's five minutes (L-106's (a) is today's shards); here, under the other worktree's load (about
  35), the per-commit run took 99 s for eight shards against 83 s for a run that failed early. **Default: kept in
  `answer_round_trip`**, beside the dump it reads; the merged tree's nightly re-takes the shards.
- **Not run**: `EIN_CORPUS_SLOW=1 answer_round_trip` (the nightly's), so the heavy and slow programs' last nodes are
  not yet held by their containers; the merged tree's nightly runs it.

### Checks

`cargo test --profile gate --workspace --no-fail-fast` on `86d46410`: 1 351 passed, 1 failed, 162 binaries. The
one is `roundtrip.rs`' open-time ceiling, 21.5 ms against the gate profile's 20 under the other worktree's load
(about 35 when the run started); alone, three times, 5.3 ms, and the saturated zebra2 is 73 624 bytes, its
sections as before (a root writes no `COMMIT`). **The shape golden did not move**: `id_order_invariance` green, no
line re-banked. Before it, `ein-einb`, `ein-ir`, and `ein-cli`'s `einb_cli`, `saved_files`, `test_cli`,
`answer_round_trip` alone: green. `cargo fmt --check`, `cargo clippy --workspace --all-targets -D warnings`, `cargo doc -D warnings`:
clean. `utils/check_doc_blocks.py`, `check_release_voice.py`, `check_gate_steps.py`, `wrong_answers.py` (16 open
rows), `check_generated.py --bin target/gate/ein`: green. `utils/doc_audit.py --links --check --tracked`: 1 862
findings, every one *file missing* under a `review/` folder (the outputs the worktree lacks), 0 decision ids.
