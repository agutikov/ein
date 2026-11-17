# Ein, from the outside

**A problem goes in, `ein` solves it as a black box, and a result comes out.** These pages are for someone who writes an Ein program and reads
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
| [`04-reference/`](04-reference/README.md) | the exact surface: the grammar as railroad diagrams, every form and reserved name, the standard library module by module, every command and option, every diagnostic, every row of the read-out and every file of a dump |

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
  [`04-reference/04-stdlib/`](04-reference/04-stdlib/README.md) for what the standard library already states. Keep
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

- **Ein is the language of these pages**: a program a reader can run, in a ```` ```lisp ```` block, and what `ein`
  prints for it, in a ```` ```text ```` block. A graph shows what the Ein says, words say it, and a formula is added
  where it explains ([Q-M1f.370](../../decisions/docs/Q-M1f.370-each-perspective-of-the-documentation-is-written.md)).
- A drawing is an image. Its source is a file in the `diagrams/` folder beside the page, a `.dot` that Graphviz
  draws or, for the grammar's diagrams, a `.puml` that PlantUML draws, and the page shows the `.svg` drawn from it;
  none is drawn in characters
  ([Q-M1f.368](../../decisions/docs/Q-M1f.368-a-diagram-is-a-file-and-a-page-shows-its-picture.md)). A drawing of a
  program is the one `ein render` writes for it, where a page names the command.
- A formula is TeX, between `$` in a line and between two `$$` lines when displayed
  ([Q-M1f.369](../../decisions/docs/Q-M1f.369-a-formula-is-written-in-tex-displayed-or-inline.md)). A symbol `ein`
  prints, the read-out's `∅`, is quoted as printed.
- A shell session is in a ```` ```sh ```` block, and a form's shape, written with `<placeholders>`, in
  ```` ```text ````. A `lisp` block that writes `…` is an excerpt. In
  the guide every other `lisp` block is a whole program that `ein solve` loads, and the test suite holds that; on the
  other pages a block may also be a fragment the text names (a claim, a query of a program shown before, a module's
  importer), or a program shown beside the refusal it gets.
- Output shown on a page is the binary's, pasted, not written by hand; where a page is held to the binary by the
  test suite, it says so.
- **Ein has no types.** Where a page says *type*, it is a program's word for a name it groups its objects under, or
  a word `ein` itself prints; a declaration's slots carry *labels*, which are ordinary names.
- A decision is cited as a link to its file under [`docs/decisions/`](../../decisions/README.md).
