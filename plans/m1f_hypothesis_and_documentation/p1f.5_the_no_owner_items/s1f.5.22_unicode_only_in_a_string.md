# S1f.5.22 — Unicode only inside a string literal, and the refusal says so

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 2
([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). T1f.5.22.1, the census, was
taken by the coordinator the same day ([its record](#t1-record-2026-11-10)), and its question on comments,
[Q-M1f.65](../open_questions.md#q-m1f65--may-a-comment-hold-unicode), was put to the user and **ruled the same
day, (b): a comment may hold Unicode**, so no file is rewritten and T1f.5.22.3 is the lexer's refusal alone.
**Done 2026-11-10**, in one working tree for the coordinator to commit: T1f.5.22.2 ([its record](#t2-record-2026-11-10)),
the census of thirty-four places a token can stand and twelve characters, each on every command, and of four ways a
module can fail to be read; four Unicode spaces loaded as separators in thirty-one places (W73, five rows), and every
way a module that exists can fail to be read was `module not found … (None)` on every command (W74, nine rows on the
one way git can keep, a directory), the fourteen rows filed `open` and held to the unchanged engine; the import read,
an imported module read through `ein_ir::read`, the read every command makes of the program it is given, moved there
from `ein-cli`'s `common`; T1f.5.22.3 ([its record](#t3-record-2026-11-10)), the lexer's refusal, `ein.l`'s catch-alls
and whitespace ASCII, so a character that is not ASCII outside a string and a comment is refused at the lexer, at the
character, by every command, with a line naming it, its code point and where Unicode is allowed, the fourteen rows
`fixed`, and the pin; T1f.5.22.4 ([its record](#t4-record-2026-11-10)), the pages; the acceptance re-taken
([§ Acceptance, re-taken](#acceptance-re-taken-2026-11-10)). Seven questions taken by default, Q-M1f.90 to Q-M1f.96.
The register reads 231 fixed, 13 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`,
2026-11-10). No wrong answer was found outside the stage.
**Depends on:** nothing since Q-M1f.65's ruling. It edits `ein.l` and the grammar's table, so it does
not run beside [S1f.5.23](s1f.5.23_open_is_reserved.md), which may too.

## The ruling

The close review's finding 2, as written on 2026-11-10:

> **A non-UTF-8 imported module is reported as missing.** With `(import badmod :symbols (q))` and a `badmod.ein`
> holding a `0xff` byte, `ein solve` prints `kb load error: (import badmod) — module not found at …/badmod.ein
> (None)`, so a Rust `None` leaks into the message. `ein solve badmod.ein` on the same file prints the right line,
> `badmod.ein:3:4: byte 0xff is not UTF-8: …`. The cause is `imports.rs`'s `read_to_string(…).ok()`.

The user, 2026-11-10: *"new stage - Unicode is not allowed anywhere else than inside string literals, and the
error must say it explicitly"*.

So the stage does two things: an imported module is read as the main file is read, with the same refusal; and a
character that is not ASCII is refused everywhere but inside a string literal (`:why`, `:source`, `:goal-text`
since S1f.5.2), by a line that says exactly that.

## The case

Re-taken on 2026-11-10, release binary at `21c24c07`:

| program | `ein solve`'s stderr | exit |
|---|---|---:|
| `main.ein`: `(import badmod :symbols (q))`, and `badmod.ein` holding a `0xff` byte on its line 3 | `kb load error: (import badmod) — module not found at badmod.ein (None)` | 1 |
| `badmod.ein` itself | `badmod.ein:3:5: byte 0xff is not UTF-8: a program is UTF-8 text, or a .einb container` | 1 |
| `(p café)` | `` nonascii.ein:2:4: unexpected `café`: a token that is not a name ``, with a repair | 1 |
| a `;;` comment holding `café` and `→` | none: the program loads and answers | 0 |

The cause of the first row is [`imports.rs`](../../../ein.rs/crates/ein-ir/src/imports.rs) line 159,
`.then(|| std::fs::read_to_string(&path).ok())`: an error reading a file that exists turns into *not found*, and
the `(None)` is the `Option` the message formats. S1f.5.5's T13 gave every command one read path for the main file
(`ein-cli`'s `common::read_program`); the import path is the one read it did not reach.

## T1 record, 2026-11-10

Taken by the coordinator with a scan of every `.ein` under `examples/`, `tests/`, `stdlib/` and `corpus/` (454
files), classifying each character that is not ASCII by where it stands: inside a string literal, inside a `;`
comment, or anywhere else.

| where | characters | files |
|---|---:|---:|
| inside a string literal | 606 | — |
| inside a comment | 35 590 | 429 of 454 |
| anywhere else | **0** | 0 |

So refusing a character that is not ASCII outside strings and comments moves **no** corpus program: only the
wording of the line changes, from *a token that is not a name* to a line that names the character and says where
Unicode may stand. In the comments, 88 % of the characters are the box-drawing rule `─` (31 377). After it come
`—` 1 868, `…` 393, `═` 332, `∅` 161, `·` 156, `⟹` 155, `→` 153, `§` 116, `∧` 83, `¬` 81 and `⊥` 76, then 40 more kinds.
51 pages under `docs/kernel/` and `docs/guide/` carry Unicode, some of it in the Ein code blocks a test loads.

Whether a comment may hold Unicode is the one thing the ruling's words leave to read: *"anywhere else than inside
string literals"* includes comments if read literally, and then 429 files' comments are rewritten in ASCII.
Put to the user as [Q-M1f.65](../open_questions.md#q-m1f65--may-a-comment-hold-unicode), and ruled the same day:
the user chose *"Comments may hold Unicode"*, (b).

## What it changes

- **The import read.** An imported module is read through the same function as the main file, so a module that is
  not UTF-8 is refused with the main file's line, its own path and position, and an unreadable one with the OS's
  reason in Ein's words. No `None`, no *not found* for a file that exists. Each command that reads imports gets it.
- **The refusal.** A character that is not ASCII outside a string literal is refused at the lexer with one line,
  `<file>:<line>:<col>: '<char>' (U+XXXX) is not ASCII: Unicode is allowed only inside a string literal, after
  :why, :source or :goal-text`, which adds *or in a comment* if Q-M1f.65 rules (b). The grammar's table gets a row per
  place, `00_ebnf.md` is re-blessed from `ein.l`, and the `byte 0x.. is not UTF-8` line stays as it is.
- **If Q-M1f.65 rules (a)**, every comment under `examples/`, `tests/`, `stdlib/` and `corpus/`, in the Rust suites'
  inline programs and in the doc pages' Ein code blocks is rewritten in ASCII. T1f.5.22.2 writes the mapping
  before the edit (`─` → `-`, `→` → `->`, `⟹` → `=>`, `§` → `S`, …). Positions must not move where a
  refusal or a register row prints one, which is Q-M1f.57's line lock. The stdlib's `MANIFEST.sha256` is
  re-taken. No answer may move, held by the corpus digests.

## Defaults, as taken (2026-11-10)

Seven choices the stage leaves open, each taken by default and recorded as a row of
[`open_questions.md`](../open_questions.md), marked *defaulted 2026-11-10*:

- **Which moves are register rows, and where their witnesses live**: W73, a Unicode space that loaded, one row per
  space the census found and one for a no-break space inside a name; W74, a module that exists and cannot be read,
  nine rows on the one way git can keep, a directory, one per command run the register can run; a module that is not
  UTF-8, one that cannot be read and a `.einb` container are the pin's, written at run time
  ([Q-M1f.90](../open_questions.md#q-m1f90--which-of-the-import-reads-moves-are-register-rows-and-where-their-witnesses-live)).
- **A module that is a `.einb` container** is refused with a line of its own, an import reading Ein text
  ([Q-M1f.91](../open_questions.md#q-m1f91--a-module-that-is-a-einb-container)).
- **A character a reader cannot see** is quoted as the escape a string writes it with, `'\u{a0}' (U+00A0)`
  ([Q-M1f.92](../open_questions.md#q-m1f92--how-the-line-quotes-a-character-a-reader-cannot-see)).
- **A module's read line names the path the import resolves to**, the one `module not found at …` names
  ([Q-M1f.93](../open_questions.md#q-m1f93--which-path-a-modules-read-line-names)).
- **A directory's and an unreadable module's line is the program's**, Python's spelling, F23's D-1 for both; a missing
  module keeps `module not found … (None)`, its `(None)` F23's D-2
  ([Q-M1f.94](../open_questions.md#q-m1f94--a-directorys-and-an-unreadable-modules-line-and-a-missing-ones-none)).
- **The refusal names the first such character**, where the lexer stops
  ([Q-M1f.95](../open_questions.md#q-m1f95--how-many-characters-the-refusal-names)).
- **A byte-order mark** is refused as any other character that is not ASCII
  ([Q-M1f.96](../open_questions.md#q-m1f96--a-byte-order-mark)).

## Tasks

### Task T1f.5.22.1 — the census (done, [above](#t1-record-2026-11-10))

### Task T1f.5.22.2 — the import read (done, [its record](#t2-record-2026-11-10))

`imports.rs` reads through `common::read_program`'s text half, moved into `ein-ir` if the dependency direction
needs it. Witnesses: a module that is not UTF-8, one that is a directory, and one that cannot be read, each on
every command that reads a program. Register rows under schema `/16` for each command run whose line or exit
moves, filed before the fix (the register is at schema `/20` since, which the stage leaves as it is).

### Task T1f.5.22.3 — the refusal, and the comments if ruled (done, [its record](#t3-record-2026-11-10))

The lexer rule and its line; the grammar's table; `00_ebnf.md` re-blessed. If Q-M1f.65 is (a), the comment
rewrite, mechanical, one commit, before the lexer refuses it.

### Task T1f.5.22.4 — the pages (done, [its record](#t4-record-2026-11-10))

[`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md) (what a program's text may hold),
`defined_behaviour.md` § 1 (the lexer's lines), and the stage's records.

## Acceptance

- A character that is not ASCII outside a string literal (and outside a comment, if Q-M1f.65 rules (b)) is refused with the line
  above on every command that reads a program, held by a test with one witness per position.
- An imported module that is not UTF-8 is refused as the main file is, with its own path; no message carries
  `None`.
- No corpus answer moved.
- `./run_tests.sh` green.

## T2 record, 2026-11-10

Taken on the release binary at `724c11e8` (`./build.sh --engine`), before any engine edit. The scripts are the
session's scratch (`census.py`, the places, the characters and every command; `lexcost.py`, the lexing cost); every
program a register row reads is banked under `examples/ein-bugs/wrong-answers/`, and every place is the pin's (T3).

**A character that is not ASCII, outside a string and a comment**: thirty-four places a token can stand, each a
program of six declarations and the place (and a claim `ein test` checks where the place is no query), with each of
twelve characters, `é`, `→`, `─`, a zero-width space U+200B, a no-break space U+00A0, a fullwidth paren U+FF08, 🦓
U+1F993 (outside the BMP), an ideographic space U+3000, a line separator U+2028, a byte-order mark U+FEFF, a next-line
control U+0085 and a right-to-left override U+202E, each run on the seventeen commands that read a program
(`saturate`, `saturate --dump`, `solve` (describe), `--minimal`, `--maximal`, `--single`, `--count`, `-n 1`, `oracle`,
`test`, `render lattice`, `render kb`, `render program`, `render rules`, `render rule --name r`, `render constraints`,
`kb save`): 6 936 runs. The places: a fact's relation, its value glued inside, alone and glued after; a declared
relation's name and type; an `is-a`'s object; a nested fact's relation; a stored negation; after a keyword; a rule's
name, a variable, a premise's relation, a conclusion's value and a guard's value; an hrule's conclusion; a macro's
name and body; a query's goal, a claim's fact and `:over`; a region claim's fact; a `(solve …)` list; a `(config …)`
flag's value and its keyword; a trace step's name; an import's module; between two forms; a line's first character;
between a paren and a word; glued after a `:why` string; after a `#`; in place of a paren; and the file's first
character.

| characters | at `724c11e8` |
|---|---|
| `é`, `→`, `─`, U+200B, U+FF08, 🦓, U+FEFF, U+202E | refused in every place on every command (8 × 578 runs), the parse error at the token the character is glued to, *`` unexpected `café`: a token that is not a name ``* and a repair, or at a `#` before it, *`` no token starts with `#` ``* |
| U+00A0, U+3000, U+2028, U+0085, Unicode's whitespace | **a separator**, `ein.l`'s skip rule reading `\s`, Unicode's: in thirty-one of the thirty-four places the program **loads** and every command answers it as if the character were a space (468 of each one's 578 runs exit 0; `render rules` and `render rule` exit 1 where the place writes no rule, a claim a split value refutes exits 1, and the other places are refused by lines of their own: `:over a` with the space after it, a `#` before it, the space in place of a paren) |

Every character inside a `:why`, a `:source` and a `:goal-text` string and in a `;` and a `#| … |#` comment loads and
is answered, as it is since (the census's program declares no rule, so `render rules` and `render rule` have nothing
to draw there, and its `:goal-text` query no claim for `ein test`; the pin's program has both).

**An imported module that cannot be read**: seven modules a file-relative `(import m :symbols (q))` names, each on the
same seventeen commands (119 runs): one that is not UTF-8 (a `0xff` byte on its line 3), a directory, one whose mode
is `000` (the session's user is not root), a `.einb` container (`ein kb save` of a module, named `m-box.ein`), one that
does not exist, one holding `ké`, and one holding a no-break space between two tokens.

| module | at `724c11e8`, on every command (`ein test`: exit 2, the rest 1) |
|---|---|
| not UTF-8, a directory, unreadable, a container | `kb load error: (import m) — module not found at m.ein (None)`: a path that exists reported as missing, with a Rust `None` (the close review's finding 2); the same file given to `ein solve` as the program is refused at its byte, `m.ein:3:4: byte 0xff is not UTF-8: …` |
| missing | the same line, which is right but for its `(None)` |
| holding `ké` | `kb load error: <its canonical path>:2:4: unexpected `ké`: a token that is not a name`, the module's parse error |
| holding a no-break space | **loads**, every command answers (the parse-only views read imports too) |

The cause is `imports.rs`'s `Resolver::read`, `path.is_file().then(|| std::fs::read_to_string(&path).ok())`, so any
error reading a module that exists was *not found*, and the `(None)` is the import's position, which a top-level form
does not carry (`build_ast::named` builds one with none), the `loc_repr` every import line ends with.

**Register rows**, filed `open` by this task (Q-M1f.90, Q-M1f.36's reading), each `owner` S1f.5.22, `right` the
refusal, witness under `examples/ein-bugs/wrong-answers/` with a corpus entry (`regression`, `solve` and `saturate`),
held to the unchanged engine by `wrong_answers.rs` (`cargo test -p ein-cli --test wrong_answers`, 2026-11-10, green;
`ein-corpus`'s register tests and `ein-infer`'s `register_region`, which banked the five W73 regions, green; the
register then read 217 fixed, 27 open and 1 not wrong):

- **W73**, a Unicode space outside a string and a comment loads, read as a space, five rows, each `now` `ein saturate
  --dump`, which answered: `nbsp` (U+00A0 between a relation and its value, R four states), `nbsp-in-a-name` (U+00A0
  inside what is written as one value, read as two, `(p k j)`, R eight states), `ideographic-space` (U+3000),
  `line-separator` (U+2028 between two facts, which an editor may show as a line break and Ein's positions do not
  count) and `next-line` (U+0085). The other eight characters were refused already, so only their line moves, which is
  no row (Q-M1f.36).
- **W74**, an imported module that exists and cannot be read as text is reported as missing, nine rows on one
  witness, `import-module-directory.ein`, whose module `import-module-directory-module.ein` is a directory holding a
  README (git keeps no empty directory; no `.ein` is under it, so it is no corpus file): one per command run the
  register can run, each `now` the command's refusal, `saturate --dump`, `oracle`, `render lattice`, `render kb`,
  `render program`, `render rules`, `render rule --name r`, `render constraints` and `kb save {out}/again.einb`.
  `register_region.rs` reads a refusal's paths relative to the repository's root, as the binary prints them when run
  from there (its `refusal` strips the root the library is handed).

Fourteen rows, the register's schema `/20` unchanged. A module that is not UTF-8, the finding's own way, one that
cannot be read and a container cannot be a corpus file, and are the pin's (Q-M1f.90).

**The corpus**: the census T1 took (454 files) found no character that is not ASCII outside a string or a comment, and
none of the 601 `.ein` files `git ls-files` lists is not UTF-8 (`iconv -f utf-8`, 2026-11-10), so the refusal moves no
corpus program but the stage's witnesses.

**What it costs a program that loads**: nothing measurable. `ein saturate` on the two zebras, the HEAD binary and the
stage's interleaved, 80 runs each (`utils/bench_env.sh python3 lexcost.py`, pinned to a P-core, 2026-11-10): the
`parse:` time saturate prints, median, 1.99 ms at HEAD and 1.91 ms with the stage on `examples/zebra.ein`, 2.73 and
2.66 ms on `examples/zebra2.ein`; the wall 9.7 and 9.7 ms, 9.5 and 9.4 ms. The ASCII classes take no more of
`regex`'s time than `\s` and the negated classes did.

**What will move**, listed before any golden was re-blessed, from the census and the workspace's tests run on the
change unblessed (`cargo test --workspace --no-fail-fast`, 2026-11-10: six tests failed, each on one of these):

- `corpus/wrong_answers.toml`: the fourteen rows `fixed`, each `[case.fixed]` the null id; W73's `now` the lexer's
  line, `<witness>:<line>:<col>: '\u{a0}' (U+00A0) is not ASCII: Unicode is allowed only inside a string literal (after
  :why, :source or :goal-text) or in a comment`, and its five `[case.region]` blocks out, a refused row stating none;
  W74's `now` `kb load error: IsADirectoryError: [Errno 21] Is a directory:
  'examples/ein-bugs/wrong-answers/import-module-directory-module.ein'`; schema `/20` unchanged (`wrong_answers.rs`,
  `register_region`).
- *`ein.rs/crates/ein-cli/tests/golden/corpus_exits.txt`*: twelve cells in, each witness's `solve` and `saturate`,
  exit 1; no cell out.
- *`ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5`*: twenty-five lines in, the five W73 witnesses' `ir[parse]`,
  each refused at the lexer, and W74's witness's nineteen renderings and its `load`, refused at its module's read; no
  line out and no other program's rendering moved, its read-out among them.
- *`ein.rs/crates/ein-ir/tests/golden/grammar_table.txt`*: twenty-two rows in, one per place, each refused at the lexer
  with the line; and one row that moves, `(relation p T) (p Жук)` (*a fact's argument*, S1f.5.4's), from *`` unexpected
  `Жук`: a token that is not a name ``* and its repair to the line at the `Ж` (`grammar_pages`' `TABLE`).
- `docs/kernel/ir/03-ein-lang/00_ebnf.md`'s lexical block: `BADTEXT`, `BAD` and the skip rule, ASCII; the phrase block
  is unmoved (`grammar_pages`' `the_pages_grammar_blocks_are_the_parsers`).
- *Tests* restated, not goldens: `generated_parser`'s copy of the skip rule (`SKIP_RULE`, `THREE_SKIP_RULES`), so the
  two lexers its token test compares read one whitespace class; `register_region`'s `refusal`, a module's path relative
  to the root; `einb_cli::the_two_magic_constants_agree`, the reader's constant compared directly since it is public;
  `cli_semantics`' and `einb_cli`'s comments naming where the constant lives.

No other golden: `fuzz_seeds.txt` (no seed writes a character that is not ASCII outside a string or a comment),
`grammar_decisions.txt`, `help_shape.txt`, every summary, read-out and plan-shape golden are unmoved, and no
`.expected` fixture moved.

## T3 record, 2026-11-10

T1f.5.22.2's fix and T1f.5.22.3's, one working tree.

**The read** (`ein-ir/src/read.rs`, new): `ein-cli`'s `common::read_program` moved to `ein-ir`, which every reader
of a program can reach, `ein-cli` standing above it: `read_program` reads the bytes, tells a container by its first
eight (`EINB_MAGIC` and `looks_like_einb` moved with it) and decodes UTF-8, and a failure is a `ReadError`, the line a
command prints and the OS's reason, `not_found()` the one case the resolver words itself. `ein-cli`'s `read_program` is
a wrapper and `Held` a re-export, so no command's line moved. **The import** (`ein-ir/src/imports.rs`):
`Resolver::read` reads a module on disk through it, `Unread::Missing` where no file is there (`module not found at …`,
unchanged) and `Unread::Refused` with the read's line otherwise, naming the path the import resolves to (Q-M1f.93); a
container is refused with `read::container_module_line` (Q-M1f.91). A directory's and an unreadable module's line is
the program's, Python's spelling, F23's D-1 (Q-M1f.94). The embedded stdlib is read as it was.

**The lexer** (`ein-ir/src/ein.l`): `BAD`, `BADTEXT`'s tail and the skip rule's whitespace are ASCII,
`[\x00-\x7F--[…]]` and `[\x09-\x0D\x20\x1C-\x1F]` where they were negated classes and `\s`, Unicode's; `TEXT`,
`BADESCAPE` and the comments take any character, as they did. So no rule matches a character that is not ASCII
outside a string and a comment, in either start condition, and `lrpar` returns that one lexing error, the lexer having
run before the parse (Q-M1f.95). The tokens of a file that is ASCII outside its strings and comments are the same,
the classes differing only on characters that are not ASCII: no corpus program's `ir[parse]` digest moved
(`corpus_shapes.md5`), and `generated_parser::the_lexer_gives_the_tokens_t4s_did`, which compares the shipped lexer
with its T4 form, holds the two to one whitespace class and passes. **The line** (`ein-ir/src/grammar.rs`): a lexing error at a character that is not ASCII is `not_ascii(c)`,
*`'<c>' (U+XXXX) is not ASCII: Unicode is allowed only inside a string literal (after :why, :source or :goal-text) or in
a comment`* (`UNICODE_ALLOWED`), the user's sentence with Q-M1f.65 (b)'s comment, the character quoted as its `\u{…}`
escape where a reader cannot see it (`shown_char`, Q-M1f.92); one at a `#` that a character that is not ASCII follows is
placed at the character. Any other lexing error is *`` no token starts with … ``*, as it was.

**The census, re-taken on the build** (`census.py`, release build of the working tree, 2026-11-10): each of the 6 936
runs is refused, every command but `ein test` exit 1 (`ein test` 2), with one first line, the same on each of a
program's seventeen commands, the line at the character (the pin holds that `kb save` writes no container); every
character in a string and a comment loads and is answered, as before. The imports: the four ways that exist are each refused
with the program's line behind `kb load error: `, naming the module's path, on every command; the missing module keeps
its line; the module holding `ké` is refused at the `é`, and the one holding a no-break space at the space. No line
carries `None` but the missing module's.

**What moved**, T2's list, re-blessed with `EIN_BLESS=1` test by test:

- `corpus/wrong_answers.toml`: the fourteen rows `fixed`, each `now` its first line, `[case.fixed]` the null id, W73's
  five regions out, and the register's header; it reads **231 fixed, 13 open and 1 not wrong** (`grep '^status'
  corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-10).
- `corpus_exits.txt` (`cargo test -p ein-cli --test corpus_cli`): 12 lines in and none out (`git diff --numstat`,
  2026-11-10).
- `corpus_shapes.md5` (`cargo test -p ein-render --test corpus_shapes`): 25 lines in and none out (`git diff
  --numstat`, 2026-11-10), the six witnesses' lines; no other program moved.
- `grammar_table.txt` (`cargo test -p ein-ir --test grammar_pages`): 91 lines in and 2 out, twenty-two rows in, 335 rows
  to 357 (`grep -c '^## '`), and `(p Жук)`'s report, its line at the `Ж` and its repair gone. `00_ebnf.md`'s lexical
  block, the same run: three rules.

**The tests.** New: `ein-cli/tests/unicode_only_in_a_string.rs`, the pin:
`every_character_in_every_place_is_refused_by_the_parser` (the thirty-four places, each with the twelve characters,
through `ein_ir::parse`: one error, the line, at the character);
`a_character_that_is_not_ascii_is_refused_on_every_command` (each place, its character in turn, and each character in
one place, on twenty commands, `--events` and `kb save --saturate` among them: the one first line, no container, nothing
fired); `the_registers_witnesses_are_refused_on_every_command` (W73's five and W74's, the same);
`a_string_literal_and_a_comment_hold_any_character` (each character in the three strings and the two comments, every
command answering, the container `kb save` writes answered by `saturate`, `solve`, `oracle` and `test`, and `saturate
--dump` writing a `:source` back byte for byte); and `an_imported_module_is_read_as_the_program_is` (the seven modules
on twenty commands, each line held to the one the module's path gives as a program, no `None` but the missing one's;
the unreadable case left out, and said so, where the run can read a file of mode `000`). The grammar's table
(twenty-two rows). Restated: `generated_parser`'s skip rule, `register_region`'s `refusal`,
`einb_cli::the_two_magic_constants_agree`. `natural_language_text.rs` and `tests/text/01_unicode.ein` hold every
string surface byte for byte, unchanged.

**Held both ways** (`cargo test -p ein-cli --test unicode_only_in_a_string`, 2026-11-10): with `ein.l` put back as it was at `724c11e8`, four of the pin's five tests fail, the strings' and comments' holding; with `Resolver::read`'s `read_to_string(…).ok()` behind `is_file()` put back, two fail, the import test and the register's witnesses (W74's); with the stage's tree, all five pass.

## T4 record, 2026-11-10

**The pages.**
- [`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md): a new § *What a program's text may hold*,
  the line and where Unicode may stand, a separator ASCII's whitespace, what it was until the stage, and an imported
  module read as the program is; the `NAME` row's `Жук`.
- [`00_ebnf.md`](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md): the lexical block re-blessed; § 1's preamble, fact 5
  (`Жук` no `BAD` token since) and a new fact 6, the refusal; § 4's note on what a parse error's message looks like; § 5's
  table, the pin.
- [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md): § 1's opening (the line beside *no token starts
  with*), a new [§ 1.6](../../../docs/kernel/defined_behaviour.md#16-a-character-that-is-not-ascii-is-written-only-in-a-string-or-a-comment--m1f-s1f522),
  § 4's table (a row for an imported module that cannot be read) and its paragraph on the one read, and a new
  [§ 4.18](../../../docs/kernel/defined_behaviour.md#418-an-imported-module-is-read-as-the-program-is--m1f-s1f522); § 1.5's
  count of `at None` fixtures is unmoved, no fixture under `examples/broken/load/` having moved.
- [`03_implementation.md`](../../../docs/kernel/ir/02-data-model/03_implementation.md): `read.rs` in the module map,
  and the lexer's thirteen words (it said twelve, from before S1f.5.23).
- `examples/README.md`'s catalogue (224 entries under `ein-bugs/wrong-answers/`, `ls examples/ein-bugs/wrong-answers |
  wc -l`, 2026-11-10, the directory among them; the `unicode-space-*` family and `import-module-directory`), the
  witnesses' headers and the module directory's README, the register's header; `corpus/README.md` holds no count that
  moved. Two scripts that walk `rglob("*.ein")` skip a directory since, `utils/nl_census.py` and
  `utils/fuzz_engine.py`'s seed corpus, W74's module being one.
- [`open_questions.md`](../open_questions.md): Q-M1f.90 to Q-M1f.96; the P1f.5 README's row.

`utils/doc_audit.py --links --check` over the gate's scope: 409 pages, 0 findings (2026-11-10).

## Acceptance, re-taken (2026-11-10)

Each item, and what holds it:

- *A character that is not ASCII outside a string literal (and outside a comment, if Q-M1f.65 rules (b)) is refused
  with the line above on every command that reads a program, held by a test with one witness per position*: **met**,
  Q-M1f.65 having ruled (b). The line is the user's sentence with the comment, *`<file>:<line>:<col>: '<char>' (U+XXXX)
  is not ASCII: Unicode is allowed only inside a string literal (after :why, :source or :goal-text) or in a comment`*;
  `unicode_only_in_a_string.rs` holds thirty-four places on twenty commands each and twelve characters in every place,
  and `grammar_pages`' table a row per place; held both ways (T3).
- *An imported module that is not UTF-8 is refused as the main file is, with its own path; no message carries `None`*:
  **met** for every module that exists: not UTF-8, unreadable, a directory and a container, each on twenty commands,
  the line the module's path gives as a program, behind `kb load error: `
  (`an_imported_module_is_read_as_the_program_is`; W74's nine rows). A module that does not exist keeps `module not found
  at … (None)`, whose `(None)` is every import line's `at None`, F23's D-2 (Q-M1f.94).
- *No corpus answer moved*: **met**. No corpus file writes a character that is not ASCII outside a string or a comment
  (T1, 454 files) and none is not UTF-8 (601); `corpus_exits.txt` and `corpus_shapes.md5` moved on the witnesses alone,
  no line out, and the one grammar-table row that moved is a refusal whose line moved.
- *`./run_tests.sh` green*: **met**, `build/gate-s1f.5.22.log` (green, 17 min 18 s, `./run_tests.sh`, 2026-11-10, exit 0: the six static checks, `cargo test --workspace`, the scratch check, the review bundle's smoke, 245 register rows read and 0 failed, and the bench smoke). The stage changes no DOT writer, bundle or CI install, so `utils/ci_container.sh` is not part of its done (Q-M1f.77).
