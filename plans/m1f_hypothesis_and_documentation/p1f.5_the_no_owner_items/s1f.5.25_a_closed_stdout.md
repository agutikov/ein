# S1f.5.25 — A closed stdout: `ein … | head`

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 5
([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). T1f.5.25.1, the investigation
the ruling asks for, was taken by the coordinator the same day ([its record](#t1-record-2026-11-10)), and the fix
was put to the user as [Q-M1f.68](../open_questions.md#q-m1f68--what-ein-does-when-its-reader-closes-stdout), and
**ruled the same day, (b)**: *"why not handle sigpipe properly?"* ([§ The ruling on Q-M1f.68](#the-ruling-on-q-m1f68)).
**Done 2026-11-10**, in one working tree for the coordinator to commit: T1f.5.25.2 ([its record](#t2-record-2026-11-10)),
the census of seventy-two runs, every subcommand and every option that writes to stdout, each with its reader closed
before the run, with `head -1`'s and read in full, fifty-five panicking, exit 101, and `clap`'s sixteen helps exiting 0
with their output lost; W75, eighteen rows, filed `open` under the register's schema `/21` and held to the unchanged
engine; the fix ([its record](#t3-record-2026-11-10)), `main.rs`'s `restore_sigpipe`, one audited `libc::signal(SIGPIPE,
SIG_DFL)` as `main`'s first statement, so every run of the census ends by `SIGPIPE` with nothing on stderr, the eighteen
rows `fixed`, and the pin, `ein-cli/tests/closed_stdout.rs`; T1f.5.25.3 ([its record](#t4-record-2026-11-10)), the pages;
the acceptance re-taken ([§ Acceptance, re-taken](#acceptance-re-taken-2026-11-10)). Five questions taken by default,
Q-M1f.97 to Q-M1f.101, two of them shown at the close (Q-M1f.100, Q-M1f.101). The register reads 249 fixed, 13 open
and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-10). No wrong answer was found
outside the stage.
**Depends on:** nothing since Q-M1f.68's ruling. It touches the binary, `ein-cli/src/main.rs`, and the register's
readers (`ein-corpus`'s `register`, `wrong_answers.rs`, the review bundle).

## The ruling

The close review's finding 5, as written on 2026-11-10:

> **New today: `ein saturate … | head` panics.** It prints `failed printing to stdout: Broken pipe (os error 32)`
> and exits 101. `ein solve … | head` exits 0.

The user, 2026-11-10: *"new stage, investigate and exaplain the root cause, propose fix, then I will decide"*.

## T1 record, 2026-11-10

Taken by the coordinator on the release binary at `21c24c07`.

**The root cause.** Rust's runtime sets `SIGPIPE` to *ignored* before `main` runs. A write to a pipe whose reader
has exited then fails with `EPIPE` (*Broken pipe*, os error 32) instead of killing the process, and `print!` /
`println!` panic on any write error: *failed printing to stdout*. The panic exits 101. Nothing in `ein-cli`
handles the error: every stdout write is a `print!` or a `println!` (`saturate.rs` 67, `modes.rs` 28, `test.rs` 12,
`oracle.rs` 9, `render.rs` 9, and the rest; `grep -c 'println!' ein-cli/src/*.rs`, 2026-11-10). No module holds a
`Stdout` handle or names `BrokenPipe`.

**Why `ein solve` exits 0.** By luck of size, not by handling. The read-out is built in memory and written by one
`print!("{}", text.stdout)` (`modes.rs` lines 529 and 1335). It is at most 100 lines, well inside a pipe's 64 KiB
buffer, so the write completes before `head` exits, and nothing is written after. A `solve` that writes more than
once panics too:

| run | exit |
|---|---:|
| `ein saturate examples/zebra.ein \| head -2` | **101** |
| `ein solve examples/zebra.ein --dump-config \| head -1` | **101** |
| `ein solve examples/zebra.ein \| head -2` | 0 |
| `ein solve --maximal examples/zebra2-minus-15.ein -n 32 \| head -1` | 0 (117 lines, one write) |
| `ein render kb examples/zebra.ein \| head -1` | 0 (one write) |

Whether a command panics depends on whether it writes again after the reader has gone, which is a race. So every
command with more than one write can exit 101 under `| head`, `| less` quit early, or a closed socket.

**The fixes, for Q-M1f.68.**
- **(a) A quiet stop, exit 0.** Every stdout write goes through one writer in `ein-cli`. When a write fails with
  `BrokenPipe`, the command stops writing and exits 0 with nothing on stderr. Any other write error, such as a full
  disk, is an error: one line in Ein's words and exit 1. This is ripgrep's convention and what `ein solve` does
  today by luck. It fits the user's ruling *exit non-zero only on error*: the reader stopping is not Ein's error.
  It touches every `print!` site, mechanically. A test pipes each command into a reader that closes after one line
  and asserts exit 0 and an empty stderr.
- **(b) Die by the signal, as `cat` does.** Reset `SIGPIPE` to its default before `main`'s work, so the kernel
  ends the process and the shell sees 141 (128 + 13), with nothing printed. It is one call, but `ein-cli` is
  `#![forbid(unsafe_code)]` and `libc::signal` is unsafe, so it needs a dependency or an audited `unsafe` module.
  Today only `ein-einb`'s `cast.rs` is one (design/12 § 2). And 141 is a non-zero exit for a run that was not an
  error.
- **(c) An error line, exit 1.** As (a), but a closed stdout is reported as `error: stdout was closed before the
  output was written` and exits 1.

The coordinator recommended (a).

## The ruling on Q-M1f.68

The user, 2026-11-10: *"why not handle sigpipe properly?"*, read as (b). The coordinator's two reasons against it
do not hold:
- **The `unsafe`.** Which disposition `SIGPIPE` has is a process-wide choice. It belongs on the binary, as the
  global allocator does (`ein-cli/src/main.rs`'s `snmalloc`), and never in a library an embedder links.
  `main.rs` is its own crate root, not under `lib.rs`'s `#![forbid(unsafe_code)]`. Rust 1.97.1, the pinned
  toolchain, has no stable option for it (`rustc -C help`, 2026-11-10). So it is one `libc::signal(libc::SIGPIPE,
  libc::SIG_DFL)` call at the top of `main`, with a `// SAFETY:` comment: it runs before any thread exists and
  installs the default disposition. `libc` becomes a direct dependency of the binary (it is in `Cargo.lock`
  already). It is a no-op on targets that are not Unix. It is the second audited `unsafe` site beside `ein-einb`'s
  `cast.rs`, which [design/12](../../../docs/history/m1a_rust/design/12_toolchain_and_layout.md) § 2 permits. design/12
  is a record and is not edited. The live pages that state the policy name the binary's call: AGENTS.md's
  `ein-einb` line, and `docs/api/rust.md` or `docs/kernel/architecture.md` where they state it.
- **The 141.** A process the kernel ends by `SIGPIPE` does not choose an exit code. It behaves as `cat`, `grep`
  and `head` do, so a pipeline under `set -o pipefail` reports what it reports for them. *Exit non-zero only on
  error* is about the codes Ein returns, and with the default disposition Ein never reaches a return.

## Defaults, as taken (2026-11-10)

Five choices the stage leaves open, each taken by default and recorded as a row of
[`open_questions.md`](../open_questions.md), marked *defaulted 2026-11-10*:

- **How a test closes a run's stdout**: before the run starts, in the register and the pin's second reader, so every
  write meets no reader whatever the timing; and `head -1`'s reader in the pin's third, on a one-page pipe, strict
  where a run's stdout is more than its first line and the page
  ([Q-M1f.97](../open_questions.md#q-m1f97--how-a-test-closes-a-runs-stdout)).
- **Which moves are register rows**: W75's eighteen, one per command and per `solve` question and `solve -c`, on two
  corpus programs; every other option, `--version`, `render region` and the artefacts on `/dev/stdout` the pin's; a
  move from exit 0 to the signal no row
  ([Q-M1f.98](../open_questions.md#q-m1f98--which-of-the-closed-stdouts-moves-are-register-rows)).
- **What the register records**: schema `/21`, `stdout = "closed"`, `signal`, `panicked`
  ([Q-M1f.99](../open_questions.md#q-m1f99--what-the-register-records-of-a-run-a-closed-stdout-ended)).
- **An artefact flag writing to a pipe whose reader has gone** ends the process by `SIGPIPE` too, the disposition
  being process-wide, which narrows `defined_behaviour.md` § 4.4; shown at the close
  ([Q-M1f.100](../open_questions.md#q-m1f100--an-artefact-flag-that-writes-to-a-pipe-whose-reader-has-gone)).
- **Windows**, which has no `SIGPIPE`, is left as it is, `restore_sigpipe` a no-op there; shown at the close
  ([Q-M1f.101](../open_questions.md#q-m1f101--a-closed-stdout-on-windows)).

## Tasks

### Task T1f.5.25.1 — the investigation (done, [above](#t1-record-2026-11-10))

### Task T1f.5.25.2 — the fix, as ruled (done, [its records](#t2-record-2026-11-10))

The default disposition restored on the binary, as above, and every in-process path checked. The library's tests
and an embedder are not affected, since the call is on `main.rs` only. Run every command and every option that
writes to stdout (`--help`, `--version`, `--dump-config`, `render`, `test`, `kb`, `--events -`) into a reader
that closes after one line: each must die by `SIGPIPE`, with nothing on stderr and never 101. A register row for
each command run whose exit moves (schema `/16`; the register was at `/20`, and the stage moves it to `/21`, which can
state a signal, [T2](#t2-record-2026-11-10)), filed before the fix. Bless no golden that is not named first.
`cargo deny` stays green (`libc` is MIT OR Apache-2.0).

### Task T1f.5.25.3 — the pages (done, [its record](#t4-record-2026-11-10))

[`defined_behaviour.md` § 4](../../../docs/kernel/defined_behaviour.md#4-errors-and-exit-codes) (exit codes), `configuration.md` if an exit code is listed there, the stage's records.

## Acceptance

- Every command, its stdout read by a reader that closes after its first line, ends by `SIGPIPE` (the test reads
  the child's `ExitStatus::signal()`, 13) with an empty stderr and never 101, held by a test over every
  subcommand and every option that writes to stdout.
- Every live page that states the `unsafe` policy names the binary's call.
- `./run_tests.sh` green.

## T2 record, 2026-11-10

Taken on the release binary at `0d0e529e` (`./build.sh --engine`), before any engine edit. The script is the
session's scratch (`census.py`); the rows read two corpus programs, and every run is the pin's (T3).

**The census.** Seventy-two runs: `ein --version` and `-V`; the help, `--help` and `-h`, of `ein` and of each
subcommand and view (sixteen); `saturate` bare, `--dump`, `--max-steps 1`, `--progress-every 1` and `--events
/dev/stdout`, and `--dump` on `examples/zebra.ein`; every `solve` question (describe, `--minimal`, `--maximal`,
`--single`, `--count`, `--monotone`, `-n 2`), every option that adds a write (`-c`, `-H`, `-t`, `-s`, `-y`, `--models
key` and `grid`, `--diff-models`, `-p`, `-P`, `-f`), the three artefacts on `/dev/stdout` (`--events`, `--json-summary`,
`--trace`), and `-c -H -t -s` on `zebra.ein`; `oracle` bare, `-s`, `--models key` and `grid`, `-p`, `-P` and
`--json-summary /dev/stdout`; `test` bare, `-v`, `-q`, its three artefacts on `/dev/stdout`, and `tests/`; every view,
`rules`, `rule --name functional`, `constraints`, `program` and `--levi`, `kb`, `lattice` and `--view full`, `region`
and `--view table` on the witness's `oracle --dump`, and `rules` on `zebra.ein`; and `kb save`, which writes no stdout.
The witness is `examples/branching/05_mini_zebra.ein` (no claim, so no question prints a `note:`), `ein test`'s
`examples/features/11_expect_ambiguity.ein` (a claim that holds). An artefact reaches stdout by naming `/dev/stdout`:
`--events -` writes a file named `-`. Each run three ways: read in full (its exit, its bytes, and its `write(2)` calls
on fd 1 under `strace`); with its stdout a pipe whose reader closed before the run (R0); and as `head -1`, read up to
the first newline and closed, five times (R1).

| runs | read in full | R0 at `0d0e529e` | R1 (×5) at `0d0e529e` |
|---|---|---|---|
| the sixteen helps | exit 0, one write | exit **0**, nothing on stderr: `clap` prints the help with the write's error ignored, so the help is lost and the run reports success | 0 |
| `--version`, `-V` | exit 0, one write | **101**: `thread 'main' (…) panicked at library/std/src/io/stdio.rs:1166:9:`, *failed printing to stdout: Broken pipe (os error 32)* | 0 |
| the other fifty-three that write to stdout | exit 0, nothing on stderr; one write (a question's read-out, a view's DOT) to 766 (`saturate --dump` on `zebra.ein`) | **101**, the same panic; `test --json-summary /dev/stdout` prints § 4.4's `error: --json-summary /dev/stdout: Broken pipe (os error 32)` first | **101 every time** on 18: every `saturate`, `solve -c`, `-p`, `-P`, `--models grid`, `--diff-models`, `--events /dev/stdout` and `-c -H -t -s` on `zebra.ein`, `oracle --models key`, `test --events /dev/stdout`, `test tests/`, and `render region`'s two single writes of 77 762 and 286 187 bytes, past the 64 KiB pipe; 101 or 0 on 3 (`solve -f`, `oracle -p`, `-P`); 0 with § 4.4's line on stderr on 3 (`solve --trace`, `--json-summary` and `oracle --json-summary` on `/dev/stdout`); 0 on the rest |
| `kb save` | exit 0, no stdout | 0 | 0 |

So R0 panics on every run that writes through a `print!`, at its first write, 55 of the 72; `clap`'s help exits 0
having printed nothing. Under `head -1` it is T1's race: a run that writes again after `head` exits panics, and one
whose writes all land first exits 0, `solve`'s one-write read-out and every view but `region` among them.

**Register rows**, filed `open` by this task (Q-M1f.98), each `owner` S1f.5.25, `right` `signal = 13`, its `now` and its
`wrong` the panic, its run with the reader closed before it starts (Q-M1f.97), held to the unchanged engine by
`wrong_answers.rs` (`cargo test -p ein-cli --test wrong_answers`, green; `ein-corpus`'s register tests and
`ein-infer`'s `register_region` green; a row's message changed by one digit failed it): **W75**, eighteen rows, one per
command and per `solve` question, and `solve -c`, whose write comes before the read-out: `saturate`, `saturate --dump`,
`solve`, `--minimal`, `--maximal`, `--single`, `--count`, `-n 2`, `--monotone`, `-c`, `oracle`, `render rules`, `rule
--name functional`, `constraints`, `program`, `kb` and `lattice` on `05_mini_zebra.ein`, and `test` on
`11_expect_ambiguity.ein`. The register then read 231 fixed, 31 open and 1 not wrong.

**Schema `/21`**, since the register recorded exit codes and a process `SIGPIPE` ends has none (Q-M1f.99): `[case.now]`
gains `stdout = "closed"`, a run whose stdout is a pipe whose reader closed before it started, a command's run whatever
its subcommand (`Now::is_command`), and `signal` (13, nothing on stderr) or `panicked` (exit 101, the panic's message,
the line after `panicked at`), exactly one; `[case.right]` gains `signal`; a recorded answer gains `stdout` and
`panicked`. Its readers: `ein-corpus`'s `register` (the fields, `Now::against`, the well-formedness rules and a unit
test, `a_closed_stdout_ends_a_run_by_a_signal_or_a_panic`), `ein-cli/tests/wrong_answers.rs` (the run with its reader
dropped before the spawn, `ExitStatus::signal()`, and the panic's message), and the review bundle (`util.run_closed`,
`command.py`'s comparison and words, `case.py`'s cells), whose `--smoke` reads every row per commit and builds a
`W75/solve` case since. `ein-infer/tests/register_region.rs` needed no change: a closed row's witness answers, so the
row states its region.

**What will move**, listed before the one bless and the fix:
- `corpus/wrong_answers.toml`: the eighteen rows, schema `/21`, and `register_region`'s eighteen `[case.region]`
  blocks, which its unblessed run named, 18 of 263 rows each with *no [case.region]* and no other (`cargo test -p
  ein-infer --test register_region`, then with `EIN_BLESS=1`); `git diff` shows no other line of the register moved but
  the schema's and the file's last line, which gained its newline.
- `ein.rs/Cargo.lock`: `libc` in `ein-cli`'s dependencies; it is in every build already, through `lrpar`'s `filetime`
  (`cargo tree -i libc`).
- No golden: `corpus_exits.txt`, `corpus_shapes.md5`, `help_shape.txt`, `grammar_table.txt`, `fuzz_seeds.txt`, every
  summary, read-out and `.expected` fixture are unmoved, no corpus entry being added and no run of theirs reading a
  closed stdout.

No wrong answer was found outside the stage. Windows, a release target, has no `SIGPIPE`; whether a closed pipe still
panics there no host here can show (Q-M1f.101).

## T3 record, 2026-11-10

**The fix.** `ein-cli/src/main.rs`'s `restore_sigpipe`, called as `main`'s first statement: one
`libc::signal(libc::SIGPIPE, libc::SIG_DFL)` in an `unsafe` block with its `// SAFETY:` comment (called once, before
any thread or handler exists; `SIG_DFL` runs no code of ours; the previous disposition, Rust's `SIG_IGN`, not needed),
`#[cfg(unix)]`, and a no-op elsewhere. `main.rs` is its own crate root, outside `lib.rs`'s `#![forbid(unsafe_code)]`,
and the library `ein_cli::run` is unchanged, so the bench and an embedder keep their disposition. `libc = "0.2"` is a
workspace dependency with its line of justification (`ein.rs/Cargo.toml`) and `ein-cli`'s on Unix only
(`[target.'cfg(unix)'.dependencies]`); MIT OR Apache-2.0, which `deny.toml` allows (`cargo deny` is run by no gate
step and is not installed here). No gate step audits `unsafe` sites, so none needed telling; the live pages that state
the policy name the call (T4).

**The census, re-taken on the build** (release, the working tree, 2026-11-10): R0 ends 71 of the 72 runs by signal 13
with nothing on stderr, the sixteen helps and `--version` among them, and `kb save` exits 0 as before; R1 ends 17 runs by
signal 13 every time, 6 by signal 13 or exit 0, and 49 exit 0, never 101 and never a word on stderr. `ein saturate
examples/zebra.ein | head -2` reports 141 in the shell's pipe status, and `ein solve examples/zebra.ein | head -1` 0, its
one write landing before `head` exits, as `cat`'s would.

**The register**: the eighteen rows `fixed`, each `now` `signal = 13`, `[case.fixed]` the null id, the note naming the
fix, and the header's W75 paragraph and the `/21` fields; it reads **249 fixed, 13 open and 1 not wrong** (`grep
'^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-10). The bundle built `W75/solve` and `W75/test` on the
build, each **matches** its `now`.

**The pin**, `ein-cli/tests/closed_stdout.rs`: sixty-eight runs, the census's but the three on `zebra.ein`, `kb save`,
and `tests/` (a directory of the suite, `tests/stdlib/closure`, in its place), `kb`'s two helps under the `einb`
feature; `every_run_listed_writes_stdout_and_nothing_on_stderr` (read in full: exit 0, stdout, an empty stderr, so the
readers below hold a closed stdout's doing alone); `a_reader_closed_before_the_run_ends_every_run_by_sigpipe`
(`ExitStatus::signal()` 13 and an empty stderr, every run); `a_reader_that_closes_after_the_first_line_ends_a_run_that_writes_again`
(`head -1`'s reader on a pipe shrunk to one page with `F_SETPIPE_SZ`, its size read back: a run whose stdout read in
full is more than the bytes read and the page must end by signal 13, and one that fits by signal 13 or exit 0, so the
test is no race; never 101, never stderr; 21 runs strict and 47 either way, and the strict arm held non-empty on Linux);
and `a_command_that_writes_nothing_to_stdout_is_not_ended_by_a_closed_one` (`kb save`, the control). **Held both ways**
(`cargo test -p ein-cli --test closed_stdout`, 2026-11-10): with `main.rs` as it was at `0d0e529e`, two of the four
fail, every one of the 68 runs under the closed reader (52 at exit 101, the 16 helps at exit 0) and the `head -1` reader
on its 101s; the precondition and the control pass; with the stage's tree, all four pass.

## T4 record, 2026-11-10

**The pages.**
- [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md): a new
  [§ 4.19](../../../docs/kernel/defined_behaviour.md#419-a-closed-stdout-ends-ein-by-sigpipe--m1f-s1f525), what a closed
  stdout does since and until, the race under `| head`, why it is no exit code, where the disposition lives and what an
  embedder keeps; § 4.4's table, a row for a pipe whose reader has gone (Q-M1f.100); § 5, a paragraph under the exit
  codes' table.
- [`configuration.md`](../../../docs/kernel/configuration.md) § 3.1, beside *every answer exits 0*: a closed stdout ends a
  run by `SIGPIPE`, which no option changes.
- The `unsafe` policy, on every live page that states it: [`docs/api/rust.md`](../../../docs/api/rust.md) (every library
  `forbid` but `ein-einb`, the binary's one call, what an embedder keeps), the repository's
  [`README.md`](../../../README.md) layout table, and AGENTS.md's `ein-einb` line. design/12 is a record and is not
  edited.
- [`utils/README.md`](../../../utils/README.md): the smoke's `W75/solve` case.
- The register's header (W75, schema `/21`); [`open_questions.md`](../open_questions.md): Q-M1f.97 to Q-M1f.101, and
  Q-M1f.68's row *built*; the P1f.5 README's row.

## Acceptance, re-taken (2026-11-10)

Each item, and what holds it:

- *Every command, its stdout read by a reader that closes after its first line, ends by `SIGPIPE` with an empty stderr
  and never 101, held by a test over every subcommand and every option that writes to stdout*: **met**, read as
  Q-M1f.97 reads it. `closed_stdout.rs` holds sixty-eight runs: with the reader closed before the run every one ends by
  signal 13 with an empty stderr; with `head -1`'s, every run that must write after the reader has gone ends by signal
  13, and one whose stdout fits in what the reader took and the pipe holds ends by signal 13 or exits 0, as `cat` does,
  never 101 and never with stderr. The register's eighteen W75 rows hold each command and `solve` question. Held both
  ways (T3).
- *Every live page that states the `unsafe` policy names the binary's call*: **met**, `docs/api/rust.md`, `README.md` and
  AGENTS.md (T4); `grep -rn 'forbid(unsafe_code)' docs/kernel docs/api docs/guide README.md AGENTS.md` lists no other.
- *`./run_tests.sh` green*: **met**, `build/gate-s1f.5.25.log` (green, 17 min 23 s, `./run_tests.sh`, 2026-11-10, exit 0:
  the six static checks, `cargo test --workspace` with `closed_stdout`'s four tests and `wrong_answers`' 263 rows, the
  scratch check, the review bundle's smoke, its register pass reading 263 rows and 0 failing and its `W75/solve` case
  ending by signal 13, and the bench smoke). The stage changes the review bundle, so by Q-M1f.77 `utils/ci_container.sh`
  is part of its done; it runs on HEAD as committed, so the coordinator runs it on the stage's commit.
