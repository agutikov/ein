# Q-M1f.378 — The grammar page is the two grammar files as EBNF, translated and never typed

The grammar on `docs/ein/user/04-reference/01-grammar.md` is the lexer's file, `ein.l`, and the parser's, `ein.y`,
written as EBNF in the notation of ISO 14977, which PlantUML draws as railroad diagrams. One diagram holds a group
of rules. Its source is a file, `01-grammar-lexical-<group>.puml` or `01-grammar-phrase-<group>.puml` in the
`diagrams/` folder beside the page, the rules between `@startebnf` and `@endebnf`; the page shows the picture drawn
from it, and links the source, which is the EBNF as text, and the grammar file it is read from
([Q-M1f.368](Q-M1f.368-a-diagram-is-a-file-and-a-page-shows-its-picture.md)). Every rule of a diagram is translated
from its file by the test that writes the sources and the page; none is typed.

- **A production of `ein.y`** is a rule whose alternatives are its productions, a token named for its text the
  terminal and a class of tokens its name. A list, written as a rule that names itself first, `R: B | R X`, is the
  repetition it derives: `{ X }` where `B` is empty, `{ X }-` where `B` is `X`, and `B , { X }` otherwise. Any other
  recursion stands as written, the rule naming itself.
- **A rule of `ein.l`** is a regular expression, and its EBNF is that expression node for node of the syntax tree
  the lexer's own regular-expression library reads it as: a run of characters a terminal, a concatenation a
  sequence, `|` an alternation, `?` an option, `*` and `+` the two repetitions, `{m,n}` a count. A class of a few
  printing characters is their alternation; any other class is a special sequence, `? … ?`, that lists its members
  in words, a negated class as `any character but:` and a difference of classes as `one of: …, but none of: …`,
  since the drawing of an exception takes no group on its right.
- **The tokens that are their own text** have one rule each in `ein.l` and are listed on the page as four rules, by
  kind, the words a name may also be spelt as being those `ein.y`'s `Name` takes. Those four names are the page's.
- **What EBNF cannot state** is a table, generated with the diagrams: the start conditions a lexer rule is tried in,
  the one it enters, and the name a parse error gives its token. That the lexer takes the longest match, and of two
  the rule written first, is prose; the diagrams keep the file's order, and the test fails if the tokens that are
  their own text are not written before the classes.
- **Which rule starts a diagram, its title and its file's name,** is the one thing written by hand, in two tables of
  the test. A rule added to a grammar file is drawn in the diagram of the rule it is written after.

A construct of either file that has no translation fails the test, so the page never states less than the grammar.
No line of a diagram starts with `'`, which PlantUML reads as a comment and drops: a terminal quoted that way is
written at the end of the line before it.

**Why.** A grammar typed onto a page is a second grammar, and the two differ the day one of them changes. A railroad
diagram is what a reader of a grammar wants to look at, and EBNF is the notation with a drawing. Translating the
lexer's rules through the library that compiles them means the page reads an expression exactly as the lexer does,
escapes, classes and counts alike.

**Held by.** `ein-ir/tests/grammar_pages.rs`: `the_pages_grammar_blocks_are_the_parsers`, which fails where a
region of the page or a diagram's source is not what the two files translate to, or where a source of the grammar's
is left that no group writes, and is what writes the regions and the sources;
`the_ebnf_translation_reads_each_shape`, the translation on rules written in the test; and
`every_token_the_phrase_block_writes_the_lexical_block_defines`.
