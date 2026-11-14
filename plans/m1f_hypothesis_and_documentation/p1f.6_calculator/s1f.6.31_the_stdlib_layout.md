# S1f.6.31 — The stdlib layout: `std.math.*` and `std.bool.*`

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-14** ([§ Record](#record-2026-11-14)); planned 2026-11-12. The restatement after
S1f.6.35, T1 and T2 recorded before the move (`56941a77`); T3 to T7 in one commit (`ac620be0`), the sweep finding no difference
but the names. Defaults Q-M1f.261 and .262, the user's to revise; E-58 and E-59 found and fixed. **Mechanical, and
done once.** It runs after the stages that are
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

## Record, 2026-11-14

Run on the coordinator's word of 2026-11-14, after S1f.6.34, S1f.6.35 and S1f.6.36, on master at `bfdce135`.

### The replies, re-read

No later reply of the user's changes the layout. [`questions.md`](questions.md), [`questions_round2.md`](questions_round2.md)
and [`replies_2026-11-12.md`](replies_2026-11-12.md) hold the four replies and round 2's *"Confirm others"* quoted
above, and nothing after them about the layout. [`open_questions.md`](../open_questions.md) holds two later mentions,
both in the user's words about imports and not about the layout: on 2026-11-13, *"let's call symbols full names
`std.math.arith.decimal...`"* ([`import_clashes.md` § 7](import_clashes.md#7-the-ruling-2026-11-13-h-full-names-and-short-names)),
and on 2026-11-14, *"when import std.math.arith and use std.math.arith.sum must work identical to sum from import
std.math.arith :symbols (sum)"* ([Q-M1f.247](../open_questions.md#q-m1f247--what-the-closure-ii-follows-a-relation-of-any-kind-or-a-declared-one)).
Each names the module in passing, beside `sum`, which is no name of the module either; neither is read as a reply
that differs from replies 16 and 52 (*"rename arith to decimal-arith"*) and round 2's confirmed table, so this
file is not restated on them. The name is [Q-M1f.261](../open_questions.md#q-m1f261--the-decimal-theorys-module-name-after-the-user-wrote-stdmatharith-in-passing),
**defaulted to `std.math.decimal-arith`, the user's to revise**; another name would be the same mechanical move again.

### What moves, restated after S1f.6.35

The stage was planned before [S1f.6.35](s1f.6.35_imports_by_full_name.md) (`d4332cd3`, `27197ea3`): since the user's
ruling of 2026-11-14 every import brings its module whole, with the modules it imports, one copy per program, every
declared name under its **full name**, `std.arith.add`; `:symbols (…)` binds the short names a file writes and
brings nothing a bare import does not; `:as a` binds `a.`; a state prints the main file's spelling, else the full
name; E-20 is closed. *What moves, and what does not* above therefore reads:

- **Module names, file paths, and every full name.** A full name is its module's canonical name and the short name,
  so it moves with the module: `std.arith.add` is `std.math.decimal-arith.add`, `std.digits.fa`
  `std.math.digits.fa`, `std.int.int-sum-of` `std.math.int.int-sum-of`, `std.rat.rat` `std.math.rat.rat`,
  `std.num9.add9` `std.math.num9.add9`. **Wherever a surface prints a full name it moves**: a name the main file does
  not list, in a trace, a core, a `refuted by` line, a read-out row, `ein saturate --dump`, a DOT view, the events,
  and the program a `--dump-answer` state file or a `.einb` writes, which declares full names. A name the main file
  lists prints short and does not move; nor does an `:as` spelling (`a.add`, Q-M1f.253). A location's path moves,
  `stdlib/arith.ein:1256:1` to `stdlib/math/decimal-arith.ein:1256:1`; the line does not, since no module's text
  gains or loses a line in the move.
- **One rule name moves, `arith` to `decimal-arith`**, full name `std.math.decimal-arith.decimal-arith`. With it:
  the fact the module writes of its own closing rule, `(arith-rule arith)`, E-11's workaround, kept since S1f.6.25
  because removing it would move every state, becomes `(arith-rule decimal-arith)`; and `std.int`'s and `std.rat`'s
  closing rules still write the lower theory's entry, `(int-rule arith)` and `(rat-rule arith)` with `arith` in their
  import's list, which become `decimal-arith`. **E-20's remedy is gone and the text stays**: until S1f.6.35 those
  names were what brought the lower theory with a selection; since then every import brings it whole, and the two
  modules keep the names *"for a program and a trace to read"* (their own comments, S1f.6.35). The comments move
  with the name; nothing is added or taken out. `std.num9` names no entry of `std.arith`'s. **The relation
  `arith-rule` keeps its name**, since no relation is renamed (above); so do `int-rule`, `rat-rule`, `num9-rule` and
  the entries `int-arith`, `rat-arith` and `arith9`.
- **How `(import std.math.decimal-arith :symbols (…))` reads.** It brings `std.math.decimal-arith` whole, and
  `std.math.digits` with it, one copy however many files import them; the list binds, for the file that writes it,
  the short names that file writes: `(import std.math.decimal-arith :symbols (decimal-arith add n sum-of))` writes
  `decimal-arith` only where the file writes the entry, and listing it brings nothing more. The plan's
  `(import std.math.decimal-arith :symbols (decimal-arith))` is now a program that writes the entry and no other
  name of the module short. A bare `(import std.math.decimal-arith)` binds none and the file writes
  `std.math.decimal-arith.add`; `:as da` writes `da.add`. A full name splits at its last dot, since a declared name
  holds none (Q-M1f.241), and `-` is a name's character, so `decimal-arith` is one segment (T1 below).
- **No relation is renamed and no answer moves**, as above. What a printed text may also do, beside the names: a
  list printed in the order of its names' text sorts the new names where the old ones sorted, except against
  `std.bijection`, `std.closure`, `std.elim` and `std.macro` (`std.arith` sorted before them, `std.math.` sorts
  after) and where `decimal-arith` meets a short name between `arith` and `decimal-arith`; S1f.6.35 saw seven
  read-outs reorder on the same ground. A column padded to its longest name widens. The sweep (T5) compares a file
  as a set of lines where its order moved, and names each such file.
- **No alias is kept**: `(import std.arith …)` is refused by the loader's *module not found at
  &lt;stdlib&gt;/arith.ein*.

### T1f.6.31.1 — the probe, done: no engine change

A scratch module, `std.math.probe` (`stdlib/math/probe.ein`: one relation, one fact, a closing rule), in a
detached worktree of `bfdce135` under `build/s1f.6.31/probe/wt`, its line written into that tree's
`MANIFEST.sha256` by hand; three programs, its import as a selection (`p.ein`), bare with full names
(`p_full.ein`) and under `:as` (`p_as.ein`), each with a claim of the module's fact. `ein test`, 2026-11-14:

| resolution | binary | the three claims |
|---|---|---|
| `$EIN_STDLIB` = the worktree's `stdlib/` | the release binary of `bfdce135` | held, 3 of 3 |
| the checkout, by the walk from the binary's directory (`wt/bin/ein`) | the same, copied | held, 3 of 3 |
| the embedded copy (`ein --version`: `embedded`; the binary copied outside any checkout) | a release build of the worktree, `include_dir!` over `stdlib/math/` | held, 3 of 3 |

`include_dir`'s `Dir::get_file("math/probe.ein")` searches subdirectories, and `ein_ir::imports`' `Resolver::read`
joins a module's segments as directories under each root, so a dotted name resolves from each of the three places
as it is: **no engine change.** A segment with a `-` resolves (`std.math.decimal-probe`, under `$EIN_STDLIB`, both
a bare import's full names and a selection), `(import std.math)` is refused, *module not found at
…/math.ein*, and the old name of a module that moved is refused, *module not found at …/probe.ein*.
`ein --version`'s stdlib line on the checkout and on the embedded build is
`sha256:48e3244e…138b`, which is `sha256sum stdlib/MANIFEST.sha256` of the worktree, so
[`docs/install.md`](../../../docs/install.md)'s check holds as written; `sha256sum -c MANIFEST.sha256` from inside
`stdlib/` reads `math/probe.ein: OK`.

**What does not see a subdirectory**, each a check rather than the resolution, and each fixed with the move (T3,
T7):

- `utils/stdlib_manifest.py` globs `stdlib/*.ein` and writes a file's name: on the worktree it reports *math/probe.ein:
  in the manifest, not on disk*, and `--write` would drop the line, so a module under `math/` would be embedded and
  in no manifest.
- `ein_ir::stdlib::Source::modules()` lists one directory: `the_embedded_copy_has_no_extra_modules` fails on the
  worktree (`cargo test --release -p ein-ir --lib stdlib`, the manifest's 13 names against the 12 it lists), and
  passes in silence on a manifest that `--write` wrote without the line. It is read by two tests and by no command.
- the stdlib's other readers by `glob("*.ein")` or one `read_dir` (T2 below).

`utils/ci_container.sh` is not needed on this ground: the release build embeds what it embedded, a tree, and what
CI installs does not change.

### T1f.6.31.2 — the inventory, done

By `git grep` on `bfdce135` (`build/s1f.6.31/inv_ein_imports.txt` holds the first list):

- **Imports of the five in `.ein` files**: 202 `(import std.arith|digits|int|rat|num9 …)` lines, some in header
  comments, in 142 files: `stdlib/` 5 (`arith`, `digits`'s header, `int`, `rat`, `num9`), `tests/stdlib/` 83
  (`arith/` 41, `int/` 14, `num9/` 13, `rat/` 13, `digits/` 2), `examples/word_problems/` 18 (its local modules
  among them), `examples/beyond_arith/` 17, `examples/arith/` 13, `examples/binary_arithmetic/` 3 and
  `examples/ein-bugs/wrong-answers/` 3 (`arity-*`, W89's witnesses). Of 856 tracked `.ein` files.
- **Doc pages' `ein` blocks and prose that import one**: `AGENTS.md` 1, `docs/kernel/defined_behaviour.md` 1,
  `docs/kernel/ir/03-ein-lang/07_stdlib_api.md` 5, `stdlib/README.md` 10, and M1g's
  `p1g.10_school_algebra/s1g.10.1` to `.4` and `p1g.11_school_geometry/s1g.11.4`, one each; the module's name
  without an import in `docs/kernel/inference/answer_dump.md` and `ir/03-ein-lang/01_grammar.md`, and in
  `examples/README.md`, `examples/word_problems/README.md`, `examples/binary_arithmetic/README.md`,
  `tests/README.md`, `utils/README.md`; P1f.6's own records (42 files) are records.
- **The Rust suites' inline programs and the module's names**: `ein-cli`'s `arith_search_free.rs`,
  `declared_counts.rs`, `imports_by_full_name.rs`, `mechanism_conflicts.rs`, `import_invariance.rs`,
  `answer_round_trip.rs`; `ein-infer`'s `arith_theory.rs`, `int_theory.rs`, `rat_theory.rs`, `num9_theory.rs`,
  `arith_batch.rs`, `region.rs`, `refutation_under_absent.rs`, `alive_invariant.rs` (86 lines, the seeded names and
  the suite's paths), `compile_limits.rs`, `stdlib_coverage.rs`; `ein-ir`'s `digit_tables.rs`,
  `imports_full_names.rs`; `ein-render`'s `dot_wellformed.rs` (`SKIPPING`'s five `tests/stdlib/` directories);
  `ein-corpus`'s `lib.rs`, the helpers S1f.6.35 added (`import_lines`, `listed`, `spellings`, `unprefixed`); and
  doc comments in `ein-ir/src/imports.rs`, `from_ir.rs` and `ein-infer/examples/shared/fact_grain.rs`.
- **Every reader of `stdlib/` by glob or by one directory**: `ein-ir/src/stdlib.rs`' `Source::modules()`;
  `utils/stdlib_manifest.py` (`glob("*.ein")`, `p.name`); `stdlib_coverage.rs`' `inventory()` (`read_dir`, module
  `std.<stem>`, `RULE_FREE_MODULES` naming `std.digits`); `arith_theory.rs` (`stdlib/{module}.ein`) and its three
  siblings, `arith_batch.rs`, `arith_search_free.rs`, `digit_tables.rs`, `compile_limits.rs` (each by a path);
  `ein-corpus`'s `spellings` (`read_dir`), `unprefixed` (a module prefix of one segment) and `import_lines`/`listed`
  (`std.x.y` to `x/y.ein`, already deep); `ein-corpus/benches/engine.rs` (`read_dir`, the bench set);
  `ein-infer/examples/shared/census.rs` (`read_dir`); `ein-cli`'s `einb_cli.rs` (copies `stdlib/`'s files one level
  into a scratch stdlib; counts the modules), `import_invariance.rs` (`read_dir`), `cli_semantics.rs` and
  `version.rs` (the directory, not its files); `ein-ir`'s `kb_semantics.rs` and `load_semantics.rs` (the directory);
  `utils/stdlib_census.py` (`glob("*.ein")`, module `std.<stem>`), `utils/stdlib_mutants.py` (`glob("*.ein")`, the
  survivors keyed `arith.ein::…`, so `tests/mutation_survivors.txt`'s 15 `arith.ein` keys), `utils/nl_census.py`,
  `utils/guard_split.py`, `utils/arith_cost.py`, `utils/rule_census.py` (and the page it writes,
  `plans/m12_rules_as_data/rule_census.md`); the corpus's completeness walk (`ein-corpus/src/manifest.rs`) and
  `utils/fuzz_engine.py` walk every depth already.
- **The generators**: `utils/gen_arith_tables.py` (writes `stdlib/digits.ein`), `utils/gen_num9.py` (writes
  `stdlib/num9.ein`), `utils/gen_arith_examples.py` (three `examples/arith/` programs), and, by hand,
  `examples/binary_arithmetic/gen_binary.py`, which reads `stdlib/arith.ein` and derives the binary theory from it.
- **Paths and keys**: `corpus/corpus.toml` (the five modules' entries and 83 lines naming `tests/stdlib/{arith,
  digits,int,rat,num9}/`), `ein-cli/tests/golden/corpus_exits.txt` (415 such lines),
  `ein-render/tests/golden/corpus_shapes.md5` (3 403); markdown links to a moved file: 11 to `stdlib/*.ein` (5 in
  current pages and plans, 6 in P1f.6's records) and 6 to `tests/stdlib/*/` (3 and 3).
- **Commands pages quote that a subdirectory makes wrong in silence**: `cat stdlib/*.ein | grep -c '^(rule '` in
  `stdlib/README.md` and in `utils/rule_census.py` (so in `rule_census.md`); `tests/README.md`'s and
  `utils/README.md`'s *"parses `stdlib/*.ein`"*, which describe the coverage test, the census and the mutants.

### T1f.6.31.3 — the move, done

Committed with T4 to T7 as `ac620be0`; this record, the findings and the questions in the commit after it.

`git mv` of the five modules, `stdlib/arith.ein` to `stdlib/math/decimal-arith.ein` and `digits`, `int`, `rat`,
`num9` to `stdlib/math/<name>.ein`; then one mechanical rewrite, `build/s1f.6.31/move.py`, over every current file
the inventory named (203 files; P1f.6's records, `docs/history/`, the register's filing notes and the generated
files left out), and nothing else of it by hand but what the script lists for a reader:

- `std.arith`, `std.digits`, `std.int`, `std.rat`, `std.num9` to `std.math.decimal-arith`, `std.math.digits`,
  `std.math.int`, `std.math.rat`, `std.math.num9`, as a module's name and as a full name's prefix;
  `stdlib/<m>.ein` to `stdlib/math/<new>.ein`; `tests/stdlib/<m>` to `tests/stdlib/math/<new>`;
- the entry where it is that rule: `(rule arith ()`, its full name, an import list of the decimal module, and
  the four facts that name it, `(arith-rule arith)` in the module, `(int-rule arith)` and `(rat-rule arith)`
  in the two over it, `(words-uses arith)` in `examples/word_problems/words.ein`; and, by hand, the 30 lines of
  prose that name the entry in backticks (the script's `--prose` list). `examples/binary_arithmetic/conv.ein`
  writes `(arith-rule decimal-arith)` beside `(arith-rule2 bin)`.

No module's text gained or lost a line (the script checks it), so every location into a module keeps its line.
One generated example did: `many_calculations.ein`'s import list wraps to two lines at the generator's 100
columns, and its forms sit one line lower.

**The generators**, edited and re-run, never their outputs: `utils/gen_arith_tables.py` and `utils/gen_num9.py`
write `stdlib/math/`, their headers name `std.math.*` (num9's `:why` of `num9-rule` too, `"{?1} is a rule of
std.math.num9"`, as `arith-rule`'s and `int-rule`'s move with the mapping); `utils/gen_arith_examples.py` reads
`stdlib/math/decimal-arith.ein` and lists `decimal-arith`; `examples/binary_arithmetic/gen_binary.py` reads the
moved module and drops its closing rule by its new name, and writes `bin.ein` with its rules **byte for byte as
before** (the two header lines that name the source move): the binary theory derived from the decimal one is the
same theory. Every `--check` is green; `utils/stdlib_manifest.py --write` re-took the manifest.

**The readers** (T2's list), each now reading every depth or the new name: `utils/stdlib_manifest.py`
(`rglob`, the manifest's names paths from `stdlib/`, sorted as text, `math/decimal-arith.ein`);
`ein_ir::stdlib::Source::modules()` (both the directory and the embedded tree walked; read by tests, no command);
`stdlib_coverage.rs`' inventory (`ein_files_under`, a module's name a dot a directory); `ein-corpus`'s `spellings`
(every depth) and `unprefixed` (a prefix of any number of segments, `std.math.decimal-arith.`);
`ein-corpus/benches/engine.rs` and `ein-infer/examples/shared/census.rs`; `ein-cli`'s `einb_cli.rs` (the scratch
stdlib copied at every depth, the size bound summed over every module) and `import_invariance.rs` (the prefixes it
takes off); `arith_theory.rs`' interface table and closing-rule checks, `int_theory.rs`, `rat_theory.rs` (the entry
by its new name); `utils/stdlib_census.py`, `stdlib_mutants.py` (a survivor keyed by its path,
`math/decimal-arith.ein::…`), `nl_census.py`, `guard_split.py`, `rule_census.py` (a test file's group is its
directory, `tests/stdlib/math/int`; the command it prints reads every depth); `utils/arith_cost.py`, broken since
S1f.6.35 and fixed (E-58). `tests/mutation_survivors.txt`'s 13 `arith.ein::` keys re-keyed
`math/decimal-arith.ein::`, nothing else of it.

**No alias.** `(import std.arith :symbols (add))`: *kb load error: (import std.arith) — module not found at
…/stdlib/arith.ein*, exit 1.

### T1f.6.31.4 — `tests/stdlib/` follows: moved

`tests/stdlib/arith/` is `tests/stdlib/math/decimal-arith/`, and `digits/`, `int/`, `rat/`, `num9/` are under
`tests/stdlib/math/`: 83 programs, with their corpus entries. **Moved, because the suite then mirrors the
library** (`std.math.x`'s programs in `tests/stdlib/math/x/`, which is where `stdlib_coverage.rs`'s message sends
a reader for a rule with no program) and because the cost T2 counted is paid once either way: every golden key of
those files is a path, and the digests of the same files move with the full names anyway (T5). What it costs
besides: P1f.6's records cite `tests/stdlib/arith/NN_…` and its siblings in backticks on some 160 lines; the phase README's note
maps the directories, and the three links among them are re-pointed (Q-M1f.262). Later modules are born where the
mirror puts them, `tests/stdlib/math/gcd/` and `tests/stdlib/bool/…`.

### T1f.6.31.5 — the sweep, done: no other difference

`build/s1f.6.31/sweep.py`, 2026-11-14: **old**, the release binary of `bfdce135` in a detached worktree of that
commit, resolving its own stdlib by the checkout walk; **new**, this stage's release binary on the working tree.
Every tracked `.ein` (856 on each side, the old paths mapped to the new one to one), each under `ein solve` and
`ein saturate --dump`, stdout, stderr and the exit code. The old side's text mapped, and nothing else: the old
tree's path to the new; `std.<m>` to `std.math.<new>`, a module's name and a full name's prefix alike;
`stdlib/<m>.ein` and `tests/stdlib/<m>/` to their new paths; the entry `arith` as a name, and
`std.math.decimal-arith.arith` to `…decimal-arith.decimal-arith`. On both, a timing masked (`ms`, `µs`, `s`), the
main file's character count masked (`ein saturate`'s `chars` line: its import lines are longer), and runs of
blanks read as one (a column padded to a longer name).

| | count |
|---|---:|
| tracked `.ein` files | 856 |
| runs (`solve` and `saturate --dump`) | 1 712 |
| exit code differs | **0** |
| equal line for line after the mapping | **1 712** |
| equal only as a set of lines | 0 |
| differ | **0** |
| streams that differ before the mapping (so the comparison is not vacuous) | 241, in 146 files |
| lines compared, the new side | 580 163 |

**No answer moved**, and no line moved but by the map: R, N, H, every claim's `holds`, every exit code, and every
state's every fact. No line reordered either: no surface the two commands print sorts the calculator's names
against the zebra modules' in one list. `ein test examples tests stdlib` on both sides: the same 1 292 lines once
mapped, as a set (the walk reaches the moved directories in another order), exit 2 on both (the refused fixtures').
The sweep was re-run on the new side after `gen_num9.py` learned the module's name (its `:why` and header), with the
same result.

### T1f.6.31.6 — the pages, done

- [`stdlib/README.md`](../../../stdlib/README.md): **§ The layout** first, the five modules' table with their files,
  sections and entries beside the zebra modules, the old names and *no alias*; every module's section under its new
  name (the anchors move with the headings, and the two pages that link into them follow); the modules table's
  links into `math/`; the rule count re-taken, 368, with the command that reads every depth (E-59).
- [`tests/README.md`](../../../tests/README.md): the suite mirrors the library; the five directories' rows, links
  and sections under `stdlib/math/`; the census *"parses every `.ein` under `stdlib/`, at any depth"*.
- `utils/README.md`: the census and the mutants read every depth, a survivor keyed by its path.
- `examples/README.md`, `examples/word_problems/README.md`, `examples/binary_arithmetic/README.md`: the names.
- `docs/kernel/`: `07_stdlib_api.md` (the five sections, their imports, the entry, and the layout in the import
  recap), `01_grammar.md`, `defined_behaviour.md` and `answer_dump.md` (their examples' full names);
  `docs/install.md` unchanged, since T1 changed nothing it says.
- This phase: one note at the top of [`README.md`](README.md) maps the old names to the new; the stage files are
  records and are not rewritten, but for ten links in six of them whose target moved, re-pointed with their text
  as it was ([Q-M1f.262](../open_questions.md#q-m1f262--a-records-link-to-a-file-the-layout-moved)).
- `AGENTS.md`: § Where things live, `stdlib/`'s calculator sentence (the modules under `stdlib/math/`, their
  programs under `tests/stdlib/math/`), and the standing rules that named the modules or their paths.
- Later plans: M1g's (its README, P1g.5, P1g.8, P1g.10 and its stage files, P1g.11 and S1g.11.4, S1g.1.3) and
  M1h's, and M12's README, under the new names; the *Names* notes that said *"read `std.arith` as
  `std.math.decimal-arith` once that stage has run"* say it ran. Left as they were: P1g.10's `findings.md`, a
  record, and three lines quoting what S1f.6.20's filter printed (*"116 steps of `std.arith`"*).
  `plans/m12_rules_as_data/rule_census.md` is a generated page that was already 1 091 lines behind its generator
  before this stage (`rule_census.py --page` on the tree at `bfdce135`), so it is not re-taken here: its tables are
  a measurement of their day, and its generator now groups `tests/stdlib/math/<module>` and prints a command that
  reads every depth.

### T1f.6.31.7 — the checks that keep it, done

- `ein-infer/tests/stdlib_coverage.rs`: the inventory reads every module at any depth (`module_files`,
  `module_name`), and **`the_inventory_reads_every_module_the_manifest_lists`** fails on a `.ein` under `stdlib/`
  that the inventory or the manifest does not see, and on a tree with no module under a subdirectory.
- `ein-ir/src/stdlib.rs`: `Source::modules()` lists every depth of the directory and of the embedded tree, so
  `the_embedded_copy_has_no_extra_modules` compares the whole embedded copy with the manifest, and
  **`the_checkout_has_no_module_the_manifest_does_not_list`** compares the checkout's.
- `utils/stdlib_manifest.py` writes and checks every depth; `utils/stdlib_census.py` and `utils/stdlib_mutants.py`
  read every module.

### The goldens, named before the bless and counted after

- **`ein-cli/tests/golden/corpus_exits.txt`**, named: its keys are paths, so the map re-paths 435 of its lines
  (the 88 moved files' cells), and no cell may move. It is `bfdce135`'s file with the map applied, byte for byte,
  and `corpus_cli` was green on it before any bless: **no exit code of any declared run moved.**
- **`ein-render/tests/golden/corpus_shapes.md5`**, named: the map re-paths 3 608 of its keys, and digests move
  on the files whose text, path or printed output the move changed, 153 by the sweep (`predicted_moved_files.txt`).
  Blessed (`EIN_BLESS=1 cargo test -p ein-render --test corpus_shapes`) and counted against the re-pathed file
  (`build/s1f.6.31/shapes_moved.py`): **4 787 of 23 921 renderings moved, on 144 files, every one among the 153**;
  none added and none gone. By view: `dot` 1 690, `ir` 566, `cores` 455, `read-out` 442, `load` 139, `hyp` and
  `hyp+closed` 135 each, `oracle` 133, `saturate`, `plan`, `match` and `region-trace` 130 each, `naf` 129,
  `explain` and `explain-noalts` 124 each, `states` 107, `region-lattice` 88. 299 of them changed their count of
  lines, **every one an `ir` rendering**: the IR printer breaks a form that passes 80 columns
  (`ein_ir::dump::DEFAULT_WIDTH`), and a full name eleven characters longer breaks more of them
  (`examples/arith/add.ein`'s resolved IR, 4 666 lines to 4 722). That is the only thing besides the names, and it
  is a line break.
- **Unmoved, as named**: `docs/kernel/inference/read_out_examples.md` (`utils/read_out_examples.py` re-run, no
  diff), `configuration.md`'s defaults block (`config_reference.rs`), `docs/api/rust.md`'s block
  (`embedding.rs`), the dump goldens (`zebra*.golden`, `module_paths.txt`), the register (`wrong_answers.rs`).
- **`tests/mutation_survivors.txt`**: its 13 `arith.ein::` keys re-keyed `math/decimal-arith.ein::`, and
  `utils/stdlib_mutants.py --check` holds the re-keyed set: 431 of 502 mutants killed, 71 survivors, S1f.6.35's
  numbers.
- Not a golden, a list: `dot_wellformed.rs`' `SKIPPING` names `stdlib/math` where it named `stdlib`, so a view
  past the bound in a zebra module now fails the sweep where it was skipped in silence.

### The checks, on the working tree before the commit

`cargo test --release --workspace` (TMPDIR under `build/`, nothing left in it): every target green but
`corpus_shapes` and `dot_wellformed`, each re-run green after the bless and `SKIPPING` above; `cargo fmt --check`,
`cargo clippy --workspace --all-targets -D warnings`, `cargo doc --no-deps` with `-D warnings`;
`utils/check_hashmap_iteration.py`; `utils/doc_audit.py --links --check` over the gate's pages, no finding;
`utils/stdlib_manifest.py`; `utils/stdlib_census.py --check` (368 of 368 rules activated, by module under the new
names); `utils/stdlib_mutants.py --check` (above); every generator's `--check` (`gen_arith_tables.py`,
`gen_num9.py`, `gen_arith_examples.py`, `examples/gen_zebra2_variants.py`, and, by hand,
`examples/binary_arithmetic/gen_binary.py` and `examples/binary_logic/gen_gates.py`); by hand,
`cargo test -p ein-infer --features counters --test arith_batch`, 5 of 5; `cargo bench --bench engine -- --test`
(the bench set reads the stdlib at every depth). `./run_tests.sh` and the nightly are the coordinator's, once over
the batch.

### Acceptance

- **The five resolve under their new names from the checkout, from `$EIN_STDLIB` and from the embedded copy, and
  the old names do not**: T1's probe for the mechanism, and the sweep, `corpus_cli` and `ein test tests/` for the
  checkout (`ein test tests/stdlib/math`, 83 files, 175 claims held); `$EIN_STDLIB` is how `stdlib_mutants.py`
  runs every mutant, on a copy of the moved tree; the embedded copy: this stage's release binary copied outside any
  checkout (`ein --version`: `embedded`, the manifest's digest), nine programs of `tests/stdlib/math/` that import
  each of the five, 25 claims held, and held to the manifest at every depth by
  `the_embedded_copy_matches_the_manifest` and `the_embedded_copy_has_no_extra_modules`. The old names:
  `(import std.arith …)` is *module not found at …/stdlib/arith.ein* from the checkout and *at
  `<embedded>/arith.ein`* from the embedded copy, `std.digits` likewise.
- **No answer moved**: the sweep, 1 712 of 1 712 runs equal but for the names; every moved golden named first.
- **Every stdlib rule is still activated** (`stdlib_coverage.rs`, and the census, 368 of 368); the mutation census
  reads every module; every generator's `--check` is green.
- **No page quotes a command that silently skips a subdirectory**: `stdlib/README.md`'s `cat stdlib/*.ein` is
  `find stdlib -name '*.ein' -exec cat {} +`; `rule_census.py` prints the same; `tests/README.md` and
  `utils/README.md` say *at any depth*. `rule_census.md`, generated and already behind, keeps its day's text.
- **What it did not do**: no relation renamed, no entry but `arith`; the zebra modules where they were; no module of
  a later stage created (`bits`, `positional`, `binary-arith`, `convert`, `gcd`, `std.bool.*`); `ein.rs/vendor/`
  untouched; no engine change: of the engine's sources, `Source::modules()`, which no command calls, and two doc
  comments.

### Defaults, findings, and what is the user's

- **[Q-M1f.261](../open_questions.md#q-m1f261--the-decimal-theorys-module-name-after-the-user-wrote-stdmatharith-in-passing)**,
  defaulted: `std.math.decimal-arith`, the layout as replied and confirmed, against `std.math.arith`, which the user
  wrote in passing on 2026-11-14. Another name is this stage again, mechanically, with its sweep.
- **[Q-M1f.262](../open_questions.md#q-m1f262--a-records-link-to-a-file-the-layout-moved)**, defaulted: a record's
  link to a moved file is re-pointed, its text kept.
- **T4's choice**, the suite moved to mirror the library, is the stage file's to make and is recorded above.
- **[E-58](findings.md)**, fixed: `utils/arith_cost.py` had not run since S1f.6.35. **[E-59](findings.md)**, fixed:
  `stdlib/README.md`'s rule count was 27 behind and its pin a test that does not read it.
- No wrong answer was met: the old and new layouts answer alike on every tracked program, and no module resolves
  differently from two places. The register is unmoved.
- **`utils/ci_container.sh` is not needed** on the stage file's ground: T1 changed neither what the release build
  embeds (a tree, as it was) nor what CI installs. The DOT writers are unchanged; what they draw moved by names
  only, and the DOT sweep is green on Graphviz here.
