# Q-M1f.68 — A closed stdout ends `ein` by `SIGPIPE`

A write to a pipe whose reader has gone, `ein saturate x.ein | head` or a pager quit early, ends the process by
`SIGPIPE` at that write, as it ends `cat` and `grep`: nothing is printed, and a shell reports 141. That holds for
every command and every option that writes to stdout, the help and `--version` among them. A run whose writes all
landed before its reader closed exits as it would have. It is no exit code of Ein's: a process the kernel ends
returns none, so *every answer exits 0* is not engaged.

The disposition is the binary's. Rust's runtime ignores `SIGPIPE` before `main`; the binary restores the default as
`main`'s first statement, one audited `libc::signal(SIGPIPE, SIG_DFL)` call on Unix. A program that embeds the engine
links the library, not the binary, and keeps its own disposition.

**Why.** Ignoring `SIGPIPE` makes a failed write a panic, exit 101 with a message on stderr, and whether a command
met it depended on a race between its writes and its reader's exit. Dying by the signal is the convention every Unix
filter follows, which `set -o pipefail` and every shell already read.

**Held by.** `ein-cli/src/main.rs`'s `restore_sigpipe`; `ein-cli/tests/closed_stdout.rs`:
`a_reader_closed_before_the_run_ends_every_run_by_sigpipe`,
`a_reader_that_closes_after_the_first_line_ends_a_run_that_writes_again`,
`a_command_that_writes_nothing_to_stdout_is_not_ended_by_a_closed_one`. The page:
[defined_behaviour.md](../../kernel/defined_behaviour.md) § 4.19. An artefact's pipe:
[Q-M1f.100](Q-M1f.100-an-artefact-written-to-a-closed-pipe-ends.md); off Unix:
[Q-M1f.101](Q-M1f.101-off-unix-a-closed-stdout-is-left-to-rusts.md).
