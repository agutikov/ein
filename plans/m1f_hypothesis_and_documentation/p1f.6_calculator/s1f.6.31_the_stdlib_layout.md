# S1f.6.31 — The stdlib layout: `std.math.*` and `std.bool.*`

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12. Not started. **Mechanical, and done once.** It runs after the stages that are
independent of the layout ([`replies_2026-11-12.md` § The order](replies_2026-11-12.md#the-order)), which are
written against today's names, and before the stages that add modules.
**Depends on:** the user's reply to [round 2's layout item](questions_round2.md#layout)
([§ The reply](#the-reply)), and every stage that edits a module it moves having landed:
[S1f.6.22](s1f.6.22_integer_division.md), [S1f.6.12's build](s1f.6.12_malformed_numbers.md#the-build-as-ruled-2026-11-12)
and [S1f.6.24](s1f.6.24_division_by_zero_at_the_call.md).
**Blocks:** binary arithmetic ([S1f.6.16](s1f.6.16_binary_arithmetic.md#the-build-as-ruled-2026-11-12)),
binary logic ([S1f.6.17](s1f.6.17_binary_logic.md#the-build-as-ruled-2026-11-12)), gcd
([S1f.6.32](s1f.6.32_gcd_and_lcm.md)) and lowest terms on request ([S1f.6.33](s1f.6.33_lowest_terms_on_request.md)).

## The rulings

Four replies of 2026-11-12 in [`questions.md`](questions.md) ask for it between them:

- [16](questions.md#q16): *"yes, extract common rules into separate file the both import, btw rename arith
  to decimal-arith"*;
- [38](questions.md#q38): *"(a) but not into std.arith; it's time to make a std.math with subfolders;"*;
- [42](questions.md#q42): *"Yes, again not std.crypt, maybe std.math.arith.crypt"*;
- [52](questions.md#q52): *"already answered - many new modules inside std.math, rename arith to
  decimal-arith, may have internal structure if it make sense: std.math.decimal-arith.overflow"*.

Round 2 put one layout to the user, since *a rename moves every program, golden and page of the calculator,
so it should happen once*, with (a) as proposed, (b) one level deeper where a module has parts, (c) another,
and one more question: whether the decimal requests keep their bare names (`add`, `sub`, `sum-of`) while
binary's take `bin-`, *proposed: yes*.

### The reply

In `questions_round2.md`, committed as `dc88520b` (2026-11-12):

> There are many more suitable names, look:
> std.logic.gates:
>   Булева алгебра (Boolean algebra) — изучает логические операции AND, OR, NOT, XOR, NAND, NOR, их свойства, тождества и преобразования выражений.
>   Логика высказываний (Propositional logic) — изучает логические формулы, их истинность, выводимость и доказательства.
>   Теория переключательных функций (Switching theory) — изучает булевы функции и их представление, преобразование и минимизацию.
>   Проектирование цифровых схем (Digital logic design) — изучает реализацию логических операций посредством электронных вентилей (logic gates).
> I think std.bool suits better here than std.logic
> Confirm others

Read as: **`std.bool` in place of `std.logic`,** and the rest of the proposal confirmed, option (a). The
reply does not name the bare decimal requests apart; *"Confirm others"* is taken to cover them, as proposed.
Both readings are the plan's and are the user's to correct.

## The layout

Round 2's table, with the user's one change:

| module | file | what it is | entry rule |
|---|---|---|---|
| `std.math.digits` | `stdlib/math/digits.ein` | the decimal tables, today's `std.digits` | none, facts |
| `std.math.bits` | `stdlib/math/bits.ein` | the binary tables | none, facts |
| `std.math.positional` | `stdlib/math/positional.ein` | the rules both bases share (round 2, item 16) | none, imported by the two below |
| `std.math.decimal-arith` | `stdlib/math/decimal-arith.ein` | today's `std.arith`: `add`, `sum-of`, … unchanged | `decimal-arith` |
| `std.math.binary-arith` | `stdlib/math/binary-arith.ein` | `bin-add`, `bin-sum-of`, … (reply 14 (b)) | `binary-arith` |
| `std.math.convert` | `stdlib/math/convert.ein` | decimal ⇄ binary (reply 17 (b)) | `convert` |
| `std.math.int` | `stdlib/math/int.ein` | today's `std.int`, with the three divisions | `int-arith` |
| `std.math.rat` | `stdlib/math/rat.ein` | today's `std.rat` | `rat-arith` |
| `std.math.num9` | `stdlib/math/num9.ein` | today's `std.num9`, kept as the record of another representation (reply 9) | `arith9` |
| `std.math.gcd` | `stdlib/math/gcd.ein` | `gcd` and `lcm` (reply 38) | `gcd` |
| `std.math.crypt` | later | the cryptarithm theory (reply 42), after S1f.6.27 and P1f.10 | |
| `std.bool.gates`, `.props`, `.circuit`, `.inputs` | `stdlib/bool/` | round 2, item 23; **`std.bool`, the user's reply above** | |

`std.algebra`, `std.slots` and the other zebra modules stay where they are.

**This stage moves the five modules that exist:** `digits`, `arith` (to `decimal-arith`), `int`, `rat` and
`num9`. The others are created in place by the stages that build them.

## What moves, and what does not

- **Module names and file paths move.** `(import std.arith :symbols (arith))` becomes `(import
  std.math.decimal-arith :symbols (decimal-arith))`.
- **One rule name moves:** the entry `arith` becomes `decimal-arith`, as the table has it, and with it the
  fact the module writes of its closing rule's name (E-11's workaround) and the name `std.int`'s and
  `std.rat`'s closing rules write to bring the lower theory (E-20's remedy). The entries `int-arith`,
  `rat-arith` and `arith9` keep their names.
- **No relation is renamed.** `add`, `sum-of`, `n`, `z`, the tagged calls and the circuit's own relations
  are as they are (*decimal stays bare*). So every fact of every state is the same fact but for the entry
  rule's name.
- **No answer moves.** R, N, H, every claim and every exit code are the same on every program; what moves
  in the goldens is module names in paths and `:source`s, and the one rule name where a surface prints it.
- **No alias is kept.** `std.arith` stops resolving. A stale import is refused by the loader's ordinary
  *module not found*; the stage does not add a redirect, since nothing outside this tree imports the
  stdlib yet. If the user wants the old names kept for a time, that is a question, with this default.

## Tasks

1. **T1f.6.31.1 — the probe, before anything moves:** that a dotted module name is a directory under each of
   the three places the stdlib is resolved from, `$EIN_STDLIB`, the checkout, and the copy `ein-ir` embeds
   with `include_dir!` (round 2: *no engine change is expected*); that `stdlib/MANIFEST.sha256` and
   `utils/stdlib_manifest.py` take a subdirectory; that `ein --version`'s stdlib line and
   `docs/install.md`'s check of a binary's stdlib against the manifest still agree. One scratch module under
   `build/`, three resolutions. **If any of the three needs an engine change, the stage stops and shows
   it,** and the change is a task with its own sweep.
2. **T1f.6.31.2 — the inventory of what names a module or its path,** by grep, written into the record
   before the move so the move can be checked against it:
   - every `(import std.…)` of the five, in `stdlib/`, `examples/`, `tests/`, the doc pages' `ein` blocks,
     and the Rust suites' inline programs;
   - every reader of `stdlib/*.ein` by glob or by name: `stdlib_coverage.rs`, `arith_theory.rs` and its
     siblings, `arith_batch.rs`, `arith_search_free.rs`, `digit_tables.rs`, `compile_limits.rs`,
     `utils/stdlib_census.py`, `utils/stdlib_mutants.py` and `tests/mutation_survivors.txt`'s keys,
     `utils/nl_census.py`, `utils/arith_cost.py`;
   - the generators: `utils/gen_arith_tables.py`, `utils/gen_num9.py`, `utils/gen_arith_examples.py`, and
     `examples/binary_arithmetic/gen_binary.py`, which reads `stdlib/arith.ein`;
   - the commands pages quote, such as `cat stdlib/*.ein | grep -c '^(rule '`, which a subdirectory makes
     wrong in silence.
3. **T1f.6.31.3 — the move,** in one commit: the files with `git mv`, the module headers, the entry rule's
   rename, every import and reader of T2, the generators and their `--check`s, the manifest re-written
   (`utils/stdlib_manifest.py --write`).
4. **T1f.6.31.4 — `tests/stdlib/` follows:** `tests/stdlib/arith/`, `digits/`, `int/`, `rat/` and `num9/`
   move under `tests/stdlib/math/` so the suite mirrors the library, with their corpus entries, or stay,
   if T2 shows that the move buys nothing and costs every golden path twice. The record says which and why.
5. **T1f.6.31.5 — the sweep.** Old release binary and old tree against new, every tracked `.ein` file under
   `ein solve` and `ein saturate --dump`, compared after mapping the old module names and the one rule name
   to the new: **no other difference.** The goldens expected to move are named before blessing
   (`corpus_exits.txt`, `corpus_shapes.md5`, the read-out examples, `--dump-config`'s block if it names a
   module); anything else that moves is shown first.
6. **T1f.6.31.6 — the pages:** [`stdlib/README.md`](../../../stdlib/README.md), a section a module and the
   layout's table first; `tests/README.md`; `examples/README.md` and the examples' own READMEs;
   `docs/install.md` if T1 touched what it says; this phase's README and stage files get **one note at the
   top of the README** that maps old names to new, and are not rewritten (they are records); `AGENTS.md`'s
   § Where things live, `stdlib/` paragraph; `plans/` pages of M1g, P1g.10 and P1g.11 among them, that name `std.arith` as what
   they import.
7. **T1f.6.31.7 — the checks that keep it:** `stdlib_coverage.rs` reads every module at any depth, and a
   test fails on a `.ein` under `stdlib/` that the manifest or the coverage reader does not see.

## Acceptance

- The five modules resolve under their new names from the checkout, from `$EIN_STDLIB` and from the
  embedded copy; the old names do not.
- No answer moved (the sweep); every moved golden was named first.
- Every stdlib rule is still activated by a program under `tests/stdlib/`, the mutation census reads every
  module, and each generator's `--check` is green.
- No page quotes a command that silently skips a subdirectory.

## What it must not do

- **Not rename a relation,** and not the entries other than `arith`.
- **Not move the zebra modules.**
- **Not create a module that a later stage builds** (`bits`, `positional`, `binary-arith`, `convert`, `gcd`,
  `std.bool.*`).
- **Not depart from the reply.** If a later reply of the user's differs from the one
  quoted above, this file is restated to it first.
- **Not edit `ein.rs/vendor/`.**

## The standing rules

A wrong answer of the engine met on the way (a module resolved differently from two places is one) is a row
in `corpus/wrong_answers.toml`, filed `open` before its fix. Findings go to [`findings.md`](findings.md)
under the next free id; questions to [`open_questions.md`](../open_questions.md) under the next free id.
Config never changes an answer; exit non-zero only on error. The gate is `./run_tests.sh`, then
`./run_tests.sh --nightly` after the commit, and `./run_tests.sh --dev-host`'s `stdlib_census.py --check`
is worth one run here, since it is the census that reads every module by path. `utils/ci_container.sh` if
T1 changed what the release build embeds or what CI installs; otherwise not.
