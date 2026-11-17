# Ein, from the outside

**Problem → [solve as a black box] → result.** These pages are for someone who writes an Ein program and reads
what `ein` answers: what a program says, how to run it, what the answer means and every row it prints. They name
no crate, module or algorithm. How the answer is computed and why it is right, stated so that any implementation
is held to it, is the reasoning perspective's subject, and how `ein.rs` computes it is the implementation's: the
[tree's index](../README.md) has both.

## The four parts

| part | what it is for |
|---|---|
| [`01-install.md`](01-install.md) | getting the `ein` binary, checking which build and which standard library it carries |
| [`02-guide/`](02-guide/README.md) | the tutorial: *Learn Ein by solving the Zebra puzzle*, then worked examples and word problems |
| [`03-syntax-and-semantics/`](03-syntax-and-semantics/README.md) | one page a concept — objects, relations, facts, rules, constraints, obligations, hypotheses, theories, macros, queries, solutions — each stated four ways: in Ein, as a graph, in words and in mathematics |
| [`04-reference/`](04-reference/README.md) | the exact surface: the grammar, every form and reserved name, the standard library symbol by symbol, every command and option, every diagnostic, every row of the read-out and every file of a dump |

Beside them, [`../../decisions/`](../../decisions/README.md) holds every decision the pages cite, one file each;
[Q-M1e.1](../../decisions/docs/Q-M1e.1-a-behaviour-is-refuted-by-a-banked-probe.md) and
[Q-M1e.2](../../decisions/docs/Q-M1e.2-an-argument-is-enough-only-when-its-premise-is.md) say what it takes to call a
claim on any of these pages settled.

## Reading paths

- **Never seen Ein.** [`01-install.md`](01-install.md), then the [guide](02-guide/README.md) from chapter 1 to the Zebra
  walkthrough. Each chapter links the concept pages it leans on.
- **Writing a program.** [`03-syntax-and-semantics/`](03-syntax-and-semantics/README.md) in its order — objects,
  relations, facts, rules, constraints — then [`10-theory.md`](03-syntax-and-semantics/10-theory.md) for how rules work
  together, [`12-query.md`](03-syntax-and-semantics/12-query.md) for what a program asks, and
  [`04-reference/04-stdlib.md`](04-reference/04-stdlib.md) for what the standard library already states. Keep
  [`04-reference/02-syntax.md`](04-reference/02-syntax.md) and [`04-reference/07-diagnostics.md`](04-reference/07-diagnostics.md) open
  beside it: the first says what each form takes, the second what each refusal means.
- **Reading an answer.** [`13-solutions-and-models.md`](03-syntax-and-semantics/13-solutions-and-models.md) for what the
  region R is, then [`04-reference/08-read-out.md`](04-reference/08-read-out.md), every row `ein solve` prints and what it
  counts, with [`04-reference/09-read-out-examples.md`](04-reference/09-read-out-examples.md) beside it.
- **Driving `ein` from another program.** [`04-reference/05-cli.md`](04-reference/05-cli.md) for the commands and their exit
  codes, `--json-summary`'s blocks in [`04-reference/08-read-out.md`](04-reference/08-read-out.md), the files of
  [`04-reference/10-answer-dump.md`](04-reference/10-answer-dump.md) and [`04-reference/11-region-dump.md`](04-reference/11-region-dump.md),
  and [`04-reference/06-config.md`](04-reference/06-config.md) for which options change only how much is printed. No option
  changes the answer.

## Conventions

- Ein code is in ```` ```lisp ```` blocks, Graphviz in ```` ```dot ````, a shell session in ```` ```sh ````, and a
  form's shape, written with `<placeholders>`, in ```` ```text ````. A `lisp` block that writes `…` is an excerpt. In
  the guide every other `lisp` block is a whole program that `ein solve` loads, and the test suite holds that; on the
  other pages a block may also be a fragment the text names (a claim, a query of a program shown before, a module's
  importer), or a program shown beside the refusal it gets.
- Output shown on a page is the binary's, pasted, not written by hand; where a page is held to the binary by the
  test suite, it says so.
- A decision is cited as a link to its file under [`docs/decisions/`](../../decisions/README.md).
