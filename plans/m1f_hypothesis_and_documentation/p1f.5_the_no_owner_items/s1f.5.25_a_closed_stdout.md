# S1f.5.25 — A closed stdout: `ein … | head`

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 5
([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). T1f.5.25.1, the investigation
the ruling asks for, was taken by the coordinator the same day ([its record](#t1-record-2026-11-10)), and the fix
was put to the user as [Q-M1f.68](../open_questions.md#q-m1f68--what-ein-does-when-its-reader-closes-stdout).
**Depends on:** Q-M1f.68's ruling. It touches only `ein-cli`'s output.

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

The coordinator recommends (a).

## Tasks

### Task T1f.5.25.1 — the investigation (done, [above](#t1-record-2026-11-10))

### Task T1f.5.25.2 — the fix, as ruled

One stdout writer in `ein-cli` (or the signal, under (b)). Every command, every output path, including `--help`,
`--version`, `--dump-config`, `render`, `test`, `kb`, and `--events -` where the event stream goes to stdout. A
register row for each command run whose exit moves (schema `/16`), filed before the fix.

### Task T1f.5.25.3 — the pages

[`defined_behaviour.md` § 4](../../../docs/kernel/defined_behaviour.md#4-errors-and-exit-codes) (exit codes), `configuration.md` if an exit code is listed there, the stage's records.

## Acceptance

- Every command, its stdout read by a reader that closes after its first line, exits as Q-M1f.68 rules and never
  101, held by a test over every subcommand and every option that writes to stdout.
- `./run_tests.sh` green.
