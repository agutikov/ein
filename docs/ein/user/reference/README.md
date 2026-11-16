# Reference

The pages to look things up in: the language exactly, every command and what it prints, every refusal. Each page
states one surface completely. What a concept means, in four representations, is
[`../syntax-and-semantics/`](../syntax-and-semantics/README.md); learning Ein from a first program is
[`../guide/`](../guide/README.md).

## The language

- [`grammar.md`](grammar.md): **the grammar**, the lexer's rules and the parser's productions, generated from the
  grammar files; what parses, exactly, and what the grammar leaves to the loader.
- [`syntax.md`](syntax.md): the lexical rules, every top-level form, the keywords each takes and what the loader holds
  it to, and the pattern language of a rule's `:match` and `:assert`.
- [`reserved-names.md`](reserved-names.md): the names a program writes and does not redefine, an *intent → word*
  card, the verdict atom `open`, `written`, and the hypothesis mechanisms and how each pair reads.
- [`stdlib.md`](stdlib.md): every module of the standard library, `std.*`, and what each symbol does.

## Running Ein

- [`cli.md`](cli.md): every command and option, the exit codes, and `ein test`.
- [`config.md`](config.md): the `(config …)` flags, the options and the `EIN_*` names, each with whether it changes
  the answer.
- [`diagnostics.md`](diagnostics.md): every parse and load diagnostic, one section per refusal.

## What Ein prints

- [`read-out.md`](read-out.md): every row `ein solve` prints and what it counts.
- [`read-out-examples.md`](read-out-examples.md): the read-out on real programs, generated from the binary.
- [`answer-dump.md`](answer-dump.md): `ein solve --dump-answer DIR`, the answer as files.
- [`region-dump.md`](region-dump.md): `ein oracle --dump DIR`, the region by brute force.
- [`rendering.md`](rendering.md): every view `ein render` draws, and the trace.

## Words

- [`glossary.md`](glossary.md): the terms Ein uses with a meaning of its own.

## Conventions

- `;` line comments, `#| block |#` block comments, not nesting.
- Hyphenated lower case for relations and rules (`has-color`, `triangle-composition`); PascalCase or `Foo-N` for types
  and instances (`Person`, `House-1`).
- A block of Ein is fenced ```` ```lisp ````, a drawing ```` ```dot ````, a command ```` ```sh ```` and printed output
  ```` ```text ````.
