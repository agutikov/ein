# Q-M1f.69 — A guide block that writes an ellipsis is an excerpt, and its whole forms load together

Every `lisp` block of the guide, `docs/guide/`, is read as its reader reads it, strings and comments as the lexer
reads them. A block that writes `…` outside a string is an **excerpt**; any other block is a **whole program**.

- A whole program loads under the default options, so a `:why` is required, and `ein solve FILE` answers it with
  exit 0: an `:expect` the engine refutes fails it.
- An excerpt's whole top-level forms load together, as one program, each where it stands in the block, so a
  position the loader prints is the page's. A top-level form with a `…` in it outside a string and a comment is
  elided, blanked character for character, as is a `…` between forms.

The prose around a block, a shell command outside a marked transcript, and whether an excerpt quotes the file it
names are not read.

**Why.** The `…` is what tells a reader that a block is cut, so a test that reads the block the same way needs no
marker kept in step with the text. That an excerpt's forms load together is what a reader who copies them gets.

**Held by.** `ein-cli/tests/guide_programs.rs`: `every_program_block_in_the_guide_runs`, and
`a_planted_refusal_in_a_copy_of_the_guide_fails_it`, which holds the reading the other way; `docs/guide/README.md`
says what the test does not read.
