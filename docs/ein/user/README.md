# Ein, from the outside

**Problem → [solve as a black box] → result.** These pages are for someone who writes an Ein program and reads
what `ein` answers: what a program says, how to run it, what the answer means and every row it prints. They name
no crate, module or algorithm. How the answer is computed and why it is right, stated so that any implementation
is held to it, is the reasoning perspective's subject, and how `ein.rs` computes it is the implementation's: the
[tree's index](../README.md) has both.

## The four parts

| part | what it is for |
|---|---|
| [`install.md`](install.md) | getting the `ein` binary, checking which build and which standard library it carries |
| [`guide/`](guide/README.md) | the tutorial: *Learn Ein by solving the Zebra puzzle*, then worked examples and word problems |
| [`syntax-and-semantics/`](syntax-and-semantics/README.md) | one page a concept — objects, relations, facts, rules, constraints, obligations, hypotheses, theories, macros, queries, solutions — each stated four ways: in Ein, as a graph, in words and in mathematics |
| [`reference/`](reference/README.md) | the exact surface: the grammar, every form and reserved name, the standard library symbol by symbol, every command and option, every diagnostic, every row of the read-out and every file of a dump |

Beside them, [`../standard-of-proof.md`](../standard-of-proof.md) says what it takes to call a claim on any of these
pages settled, and [`../../decisions/`](../../decisions/README.md) holds every decision the pages cite, one file
each.

## Reading paths

- **Never seen Ein.** [`install.md`](install.md), then the [guide](guide/README.md) from chapter 1 to the Zebra
  walkthrough. Each chapter links the concept pages it leans on.
- **Writing a program.** [`syntax-and-semantics/`](syntax-and-semantics/README.md) in its order — objects,
  relations, facts, rules, constraints — then [`theory.md`](syntax-and-semantics/theory.md) for how rules work
  together, [`query.md`](syntax-and-semantics/query.md) for what a program asks, and
  [`reference/stdlib.md`](reference/stdlib.md) for what the standard library already states. Keep
  [`reference/syntax.md`](reference/syntax.md) and [`reference/diagnostics.md`](reference/diagnostics.md) open
  beside it: the first says what each form takes, the second what each refusal means.
- **Reading an answer.** [`solutions-and-models.md`](syntax-and-semantics/solutions-and-models.md) for what the
  region R is, then [`reference/read-out.md`](reference/read-out.md), every row `ein solve` prints and what it
  counts, with [`reference/read-out-examples.md`](reference/read-out-examples.md) beside it.
- **Driving `ein` from another program.** [`reference/cli.md`](reference/cli.md) for the commands and their exit
  codes, `--json-summary`'s blocks in [`reference/read-out.md`](reference/read-out.md), the files of
  [`reference/answer-dump.md`](reference/answer-dump.md) and [`reference/region-dump.md`](reference/region-dump.md),
  and [`reference/config.md`](reference/config.md) for which options change only how much is printed. No option
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
