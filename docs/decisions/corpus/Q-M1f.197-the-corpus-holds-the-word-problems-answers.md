# Q-M1f.197 — The corpus holds the word problems' answers

Each word problem states its answers as claims, and the corpus holds them: every problem declares a `test` run, so
its claims are checked and its exit code banked; `boxes12.ein` and `boxes13.ein` declare a `--trace` run as well;
and every file is under the sweeps every corpus program is under: the renderings' digests, the answer dump's round
trip, the cores, the levers. The claims state every answer the problems give, the gap problems' `unanswered` and
`rests-on` facts among them.

**Why.** A claim is the program's own statement of its answer, so the problem and what holds it are one file, and
the sweeps that hold every corpus program hold these too. What no claim can state, the order in which two requests
are made and the steps of a trace, is the trace run's.

**Held by.** The problems' entries in `corpus/corpus.toml`; `ein-cli/tests/corpus_cli.rs`:
`every_cell_reproduces_its_exit_code`; `ein test examples/word_problems/` as a status code.
