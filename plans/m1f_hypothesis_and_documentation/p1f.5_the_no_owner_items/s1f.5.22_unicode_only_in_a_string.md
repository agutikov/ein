# S1f.5.22 — Unicode only inside a string literal, and the refusal says so

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 2
([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). T1f.5.22.1, the census, was
taken by the coordinator the same day ([its record](#t1-record-2026-11-10)), and its question on comments,
[Q-M1f.65](../open_questions.md#q-m1f65--may-a-comment-hold-unicode), was put to the user and **ruled the same
day, (b): a comment may hold Unicode**, so no file is rewritten and T1f.5.22.3 is the lexer's refusal alone.
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

## Tasks

### Task T1f.5.22.1 — the census (done, [above](#t1-record-2026-11-10))

### Task T1f.5.22.2 — the import read

`imports.rs` reads through `common::read_program`'s text half, moved into `ein-ir` if the dependency direction
needs it. Witnesses: a module that is not UTF-8, one that is a directory, and one that cannot be read, each on
every command that reads a program. Register rows under schema `/16` for each command run whose line or exit
moves, filed before the fix.

### Task T1f.5.22.3 — the refusal, and the comments if ruled

The lexer rule and its line; the grammar's table; `00_ebnf.md` re-blessed. If Q-M1f.65 is (a), the comment
rewrite, mechanical, one commit, before the lexer refuses it.

### Task T1f.5.22.4 — the pages

[`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md) (what a program's text may hold),
`defined_behaviour.md` § 1 (the lexer's lines), and the stage's records.

## Acceptance

- A character that is not ASCII outside a string literal (and outside a comment, if Q-M1f.65 rules (b)) is refused with the line
  above on every command that reads a program, held by a test with one witness per position.
- An imported module that is not UTF-8 is refused as the main file is, with its own path; no message carries
  `None`.
- No corpus answer moved.
- `./run_tests.sh` green.
