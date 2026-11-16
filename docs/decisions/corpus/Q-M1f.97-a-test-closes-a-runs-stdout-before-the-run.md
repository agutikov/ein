# Q-M1f.97 — A test closes a run's stdout before the run starts

A test that holds what `ein` does when its stdout is closed closes the pipe's read end before it starts the run, so
every write the run makes meets no reader whatever the timing, and the run ends by `SIGPIPE` at its first write,
with nothing on stderr. A reader that closes after the first line, as `head -1` does, is a third check, on a pipe
shrunk to one page where the kernel allows it: a run whose stdout is more than its first line and what the pipe
holds must write after the reader has gone, and ends by `SIGPIPE`; one whose stdout fits may finish first and exit
0, as `cat` does. Either way never exit 101 and never a word on stderr.

**Why.** Whether a reader that closes after a line meets a write is a race: a run whose writes all land in the pipe
before the reader closes has met no closed pipe. Closing the pipe before the run decides every run by construction,
with no lever in the binary to slow a run down.

**Held by.** `ein-cli/tests/closed_stdout.rs`: `a_reader_closed_before_the_run_ends_every_run_by_sigpipe`,
`a_reader_that_closes_after_the_first_line_ends_a_run_that_writes_again`,
`every_run_listed_writes_stdout_and_nothing_on_stderr`. What a closed stdout does to `ein`:
[Q-M1f.68](../cli/Q-M1f.68-a-closed-stdout-ends-ein-by-sigpipe.md).
