# F23 D-24 — the note on a carried standard-library module when `$EIN_STDLIB` points nowhere

## 1. The question

A saved file carries its program and every module the program's imports read, each as written (Q-M1f.401), and loads
from what it carries, reading no other file. Where a carried `std.*` module differs from the one this binary would
read, the load says so on stderr: *`note: FILE: the std.math.decimal it carries is not this binary's standard
library's; the text it carries is loaded`*. When `$EIN_STDLIB` names something that is not a standard library, the
binary has no library to compare with, and the note still says, for **every** carried `std` module, that it is *not
this binary's standard library's* — which reads as a difference that nobody found. The file loads as carried and the
answer is the same. The call: a note naming why there is nothing to compare, or as it is.

Recorded: [`plans/followups/f23_diagnostics_in_eins_words.md`](../../../../followups/f23_diagnostics_in_eins_words.md#the-items),
row **D-24** (the user's call; F23 owns diagnostics that change no answer).

## 2. What is so today

**The saved file.** `tests/saved/03_the_standard_library.ein` ([`programs/`](programs/03_the_standard_library.ein))
imports `std.math.decimal-arith`, `std.math.decimal` and `std.math.arith`; R is {∅}.
`ein solve … --dump-answer` ([`runs/03.solve-dump-answer.stdout.txt`](runs/03.solve-dump-answer.stdout.txt), stderr
empty) writes [`runs/03.dump-answer/models/01.ein`](runs/03.dump-answer/models/01.ein), 134 654 bytes: `(program …)`
and five `(module …)` forms, `std.math.arith`, `std.math.decimal`, `std.math.decimal-arith`, `std.math.digits` and
`std.math.positional`, each the checkout's file as it stands.

**That file under each `$EIN_STDLIB`**, with `ein solve`, `ein saturate --dump` and `ein test`
([`runs/stderr-by-setting.md`](runs/stderr-by-setting.md), from the files `runs/models-01.COMMAND.EIN_STDLIB-SETTING.{stdout,stderr,exit}.txt`):

| `$EIN_STDLIB` | what the binary reads as its library (`ein --version`) | `note:` lines, each command | stdout against unset |
|---|---|---:|---|
| `/nonexistent` | *unreadable  $EIN_STDLIB /nonexistent* | **5**, every carried module | the same |
| a directory with no `MANIFEST.sha256` (`ein solve` only) | — | **5** | the same |
| unset (the binary in the checkout) | the checkout | 0 | — |
| unset, the binary copied outside any checkout | the embedded copy | 0 | the same |
| the checkout's `stdlib/` | the checkout | 0 | the same |
| the tree of `a8d595eb` | another library: one module differs, by comments | 1, `std.math.decimal-arith` | the same |
| the tree of `0a4a97a3` | another library: three modules differ, in rules | 3, `decimal`, `decimal-arith`, `positional` | the same |

Every run exits 0; `ein solve`'s stdout is the same bytes in all seven, `ein saturate --dump`'s KB section the same,
`ein test`'s report the same but for its time. The `--version` lines: [`runs/version.by-setting.txt`](runs/version.by-setting.txt).

**The witness**, `EIN_STDLIB=/nonexistent ein solve DIR/models/01.ein`
([`runs/models-01.solve.EIN_STDLIB-nonexistent.stderr.txt`](runs/models-01.solve.EIN_STDLIB-nonexistent.stderr.txt))
(output, the path shortened to `…/models/01.ein`):

```text
note: …/models/01.ein: the std.math.arith it carries is not this binary's standard library's; the text it carries is loaded
note: …/models/01.ein: the std.math.decimal it carries is not this binary's standard library's; the text it carries is loaded
note: …/models/01.ein: the std.math.decimal-arith it carries is not this binary's standard library's; the text it carries is loaded
note: …/models/01.ein: the std.math.digits it carries is not this binary's standard library's; the text it carries is loaded
note: …/models/01.ein: the std.math.positional it carries is not this binary's standard library's; the text it carries is loaded
```

The same five under `saturate` ([stderr](runs/models-01.saturate.EIN_STDLIB-nonexistent.stderr.txt)) and `test`
([stderr](runs/models-01.test.EIN_STDLIB-nonexistent.stderr.txt)). With the variable unset
([solve](runs/models-01.solve.EIN_STDLIB-unset.stderr.txt), [saturate](runs/models-01.saturate.EIN_STDLIB-unset.stderr.txt),
[test](runs/models-01.test.EIN_STDLIB-unset.stderr.txt)) and pointing at the checkout
([solve](runs/models-01.solve.EIN_STDLIB-checkout.stderr.txt), [saturate](runs/models-01.saturate.EIN_STDLIB-checkout.stderr.txt),
[test](runs/models-01.test.EIN_STDLIB-checkout.stderr.txt)), stderr is empty.

**The note where it is meant**: with `$EIN_STDLIB` at an older tree's library, the note names exactly the modules that
differ ([`runs/models-01.solve.EIN_STDLIB-0a4-tree.stderr.txt`](runs/models-01.solve.EIN_STDLIB-0a4-tree.stderr.txt):
`decimal`, `decimal-arith`, `positional`; the modules' diffs, [`runs/stdlib.carried-modules.0a4-tree-vs-checkout.diff`](runs/stdlib.carried-modules.0a4-tree-vs-checkout.diff),
and the one-module case, [`runs/stdlib.carried-modules.a8d-tree-vs-checkout.diff`](runs/stdlib.carried-modules.a8d-tree-vs-checkout.diff)).

**The program, not saved**, under the same `/nonexistent` is refused at load, and the refusal says why
([`runs/program-03.solve.EIN_STDLIB-nonexistent.stderr.txt`](runs/program-03.solve.EIN_STDLIB-nonexistent.stderr.txt), exit 1) (output):

```text
kb load error: (import std.math.decimal-arith) — $EIN_STDLIB names /nonexistent, which is not a directory (None)
```

and for a directory with no manifest, *"… which has no MANIFEST.sha256 — a directory is the stdlib only if it carries
the marker …"* ([stderr](runs/program-03.solve.EIN_STDLIB-a-directory-with-no-manifest.stderr.txt)). (The trailing
`(None)` is F23's D-22.)

**The code** (read, not run): `ein-ir/src/saved.rs`, `load_saved`, keeps a carried module `m` for a note where
`m.starts_with("std.") && ours.stdlib_text(m).as_deref() != Some(text)`; `ein-ir/src/imports.rs`' `stdlib_text`
returns `None` whenever the library has a problem (`stdlib::Source::problem`, `$EIN_STDLIB` not a directory or with no
marker), so every carried `std` module is unequal to `None`. The note's text is pinned by
`ein-cli/tests/saved_files.rs:417`.

## 3. The options

**(a) As it is**: one note per carried `std` module, the same words whether the binary's library differs or is
missing.

**(b) A note naming why there is nothing to compare.** A change in `load_saved`: where the binary's library has a
problem, the note says so instead of *not this binary's standard library's*. Two spellings, each a MOCK of the
witness's stderr, hand-written from today's five lines and the refusal's own words for the variable:

- one note for the file:
  [`runs/models-01.solve.EIN_STDLIB-nonexistent.stderr.MOCK.option-b-one-note.txt`](runs/models-01.solve.EIN_STDLIB-nonexistent.stderr.MOCK.option-b-one-note.txt)
  ([diff](runs/models-01.solve.EIN_STDLIB-nonexistent.stderr.MOCK.option-b-one-note.diff)) (output):

  ```text
  note: …/models/01.ein: it carries std.math.arith, std.math.decimal, std.math.decimal-arith, std.math.digits and std.math.positional, and this binary has no standard library to compare them with ($EIN_STDLIB names /nonexistent, which is not a directory); the text it carries is loaded
  ```

- one note a module, the reason in each:
  [`runs/models-01.solve.EIN_STDLIB-nonexistent.stderr.MOCK.option-b-per-module.txt`](runs/models-01.solve.EIN_STDLIB-nonexistent.stderr.MOCK.option-b-per-module.txt)
  ([diff](runs/models-01.solve.EIN_STDLIB-nonexistent.stderr.MOCK.option-b-per-module.diff)) (output, the first):

  ```text
  note: …/models/01.ein: the std.math.arith it carries is not compared: $EIN_STDLIB names /nonexistent, which is not a directory, so this binary has no standard library; the text it carries is loaded
  ```

Where the binary has a library that differs (the two older trees above), the note is unchanged under either. No
answer moves under any option.

## 4. The default

None: the row is **the user's call** (*a note naming why there is nothing to compare, or as it is*). Today it is (a).

## 5. Files

- [`rerun.sh`](rerun.sh) — every run of this page and the table.
- [`programs/03_the_standard_library.ein`](programs/03_the_standard_library.ein) — copied from `tests/saved/`.
- [`runs/03.solve-dump-answer.stdout.txt`](runs/03.solve-dump-answer.stdout.txt) / [`.stderr.txt`](runs/03.solve-dump-answer.stderr.txt) — `ein solve --dump-answer` of it, `$EIN_STDLIB` unset.
- [`runs/03.dump-answer/`](runs/03.dump-answer/) — the dump; `models/01.ein` is the saved file every run below reads.
- `runs/models-01.{solve,saturate,test}.EIN_STDLIB-{nonexistent,unset,unset-binary-outside-checkout,checkout,a8d-tree,0a4-tree}.{stdout,stderr,exit}.txt` — the saved file under each setting and command; and `models-01.solve.EIN_STDLIB-a-directory-with-no-manifest.*`.
- [`runs/stderr-by-setting.md`](runs/stderr-by-setting.md) — those runs as one table.
- [`runs/program-03.solve.EIN_STDLIB-nonexistent.stderr.txt`](runs/program-03.solve.EIN_STDLIB-nonexistent.stderr.txt), [`runs/program-03.solve.EIN_STDLIB-a-directory-with-no-manifest.stderr.txt`](runs/program-03.solve.EIN_STDLIB-a-directory-with-no-manifest.stderr.txt) (with their `stdout` and `exit`) — the program itself, refused.
- [`runs/version.by-setting.txt`](runs/version.by-setting.txt) — `ein --version`'s `stdlib` line under each setting.
- [`runs/stdlib.carried-modules.a8d-tree-vs-checkout.diff`](runs/stdlib.carried-modules.a8d-tree-vs-checkout.diff), [`runs/stdlib.carried-modules.0a4-tree-vs-checkout.diff`](runs/stdlib.carried-modules.0a4-tree-vs-checkout.diff) — the five carried modules, each older tree against the checkout.
- `runs/models-01.solve.EIN_STDLIB-nonexistent.stderr.MOCK.option-b-{one-note,per-module}.{txt,diff}` — **MOCKS** of (b).

## 6. Re-run

From the repo root, `bash plans/m1f_hypothesis_and_documentation/review/01-save-and-load/31-F23-D-24/rerun.sh
[SCRATCH_DIR]`: the dump, the saved file under each setting and command (stdout, stderr, exit), the program refused,
`--version` under each setting, the modules' diffs, the table [`runs/stderr-by-setting.md`](runs/stderr-by-setting.md)
and the mocks' diffs. `SCRATCH_DIR` (a fresh `mktemp -d` by default) holds the binary copied outside the checkout
and the directory with no `MANIFEST.sha256`. The core commands:

```sh
D=plans/m1f_hypothesis_and_documentation/review/01-save-and-load/31-F23-D-24
H=build/review-bin/ein-head-17085b27
F=$D/runs/03.dump-answer/models/01.ein
env -u EIN_STDLIB $H solve tests/saved/03_the_standard_library.ein --dump-answer $D/runs/03.dump-answer
for c in solve saturate test; do x=; [ $c = saturate ] && x=--dump
  EIN_STDLIB=/nonexistent $H $c $F $x
  env -u EIN_STDLIB $H $c $F $x
  EIN_STDLIB=$PWD/stdlib $H $c $F $x
  EIN_STDLIB=$PWD/build/wt-rev-a8d/stdlib $H $c $F $x
  EIN_STDLIB=$PWD/build/wt-rev-0a4/stdlib $H $c $F $x
done
EIN_STDLIB=/nonexistent $H solve $D/programs/03_the_standard_library.ein
cp $H /some/dir/outside/the/checkout/ein && env -u EIN_STDLIB /some/dir/outside/the/checkout/ein solve $F   # the embedded copy
```

## 7. Found while building

- **The file carries five modules, and its program's header says three.** `tests/saved/03_the_standard_library.ein`
  says *"the state is this program and the three modules its imports read, each as written"*; the saved file carries
  `std.math.digits` and `std.math.positional` too, which the three import, as Q-M1f.401 says (*every module it
  imports*). So the witness's *five `note:` lines* are five modules, not three imports and two of something else.
- The witness reproduces as the ledger states it (five notes, exit 0), under `saturate` and `test` as well.
- **Re-taken with `ein-head-17085b27`** (the first look was `ein-head-b5fb8d70`): nothing moved. The five notes under
  `/nonexistent` and under a directory with no manifest, none where the library is readable, one and three under the
  older trees, every exit 0 and every stdout the same; the binary's `stdlib` digest is the same,
  `sha256:2ae8fe21…` ([`runs/version.by-setting.txt`](runs/version.by-setting.txt)).

## 8. Your call

- **(a)** as it is: one note per carried `std` module, *not this binary's standard library's*, whether it differs
  or there is nothing to compare with. Today's state.
- **(b)** where the binary has no readable library, a note that says so and names why (`$EIN_STDLIB names …`): one
  note for the file, or one per module.
- No default.
