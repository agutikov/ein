# Reference

The pages to look things up in: the language exactly, every command and what it prints, every refusal. Each page
states one surface completely. What a concept means, in four representations, is
[`../03-syntax-and-semantics/`](../03-syntax-and-semantics/README.md); learning Ein from a first program is
[`../02-guide/`](../02-guide/README.md).

## The language

- [`01-grammar.md`](01-grammar.md): **the grammar**, every rule of the lexer and every production of the parser as
  EBNF, drawn as railroad diagrams and generated from the two grammar files, which the page links; what parses,
  exactly, and what the grammar leaves to the loader.
- [`02-syntax.md`](02-syntax.md): the lexical rules, every top-level form, the keywords each takes and what the loader holds
  it to, and the pattern language of a rule's `:match` and `:assert`.
- [`03-reserved-names.md`](03-reserved-names.md): the names a program writes and does not redefine, an *intent to word*
  card, the verdict `open`, and the hypothesis mechanisms and how each pair reads.
- [`04-stdlib/`](04-stdlib/README.md): the standard library, `std.*`, a page a family of modules: what a program
  imports and writes, what each symbol produces, and the module's rules apart.

## Running Ein

- [`05-cli.md`](05-cli.md): every command and option, the exit codes, and `ein test`.
- [`06-config.md`](06-config.md): the `(config …)` flags, the options and the `EIN_*` names, each with whether it changes
  the answer.
- [`07-diagnostics.md`](07-diagnostics.md): every parse and load diagnostic, one section per refusal.

## What Ein prints

- [`08-read-out.md`](08-read-out.md): every row `ein solve` prints and what it counts.
- [`09-read-out-examples.md`](09-read-out-examples.md): the read-out on real programs, generated from the binary.
- [`10-answer-dump.md`](10-answer-dump.md): `ein solve --dump-answer DIR`, the answer as files.
- [`11-region-dump.md`](11-region-dump.md): `ein oracle --dump DIR`, the region by brute force.
- [`12-rendering.md`](12-rendering.md): every view `ein render` draws, and the trace.

## Words

- [`13-glossary.md`](13-glossary.md): the terms Ein uses with a meaning of its own.

## Conventions

- `;` line comments, `#| block |#` block comments, not nesting.
- Hyphenated lower case for relations and rules (`has-color`, `triangle-composition`); PascalCase or `Foo-N` for the
  names of things and of the groups a program puts them in (`Person`, `House-1`).
- A block of Ein is fenced ```` ```lisp ````, a command ```` ```sh ```` and printed output ```` ```text ````; a
  formula is TeX between `$`. A drawing is an image, and its source is a file of the same name in the
  [`diagrams/`](diagrams/) folder beside the page: a `.dot`, or a `.puml` for a diagram of the grammar
  ([Q-M1f.368](../../../decisions/docs/Q-M1f.368-a-diagram-is-a-file-and-a-page-shows-its-picture.md)).
