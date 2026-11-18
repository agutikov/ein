# Q-M1f.412 — the read-out's `is-a` row is dropped: does a schema id move?

## 1. The question

The read-out had a `types` row under `universe`: the atoms on the right of an `is-a` fact that the blind rung's
candidates have as arguments. It read the relation `is-a` by its spelling and decided nothing. On the user's ruling
(*"V-6 (a), drop the is-a row"*) it was dropped, on every surface: the row on every question, `region.md`'s line,
the key `types_as_objects` in `region.json`, in `--json-summary`'s `root.hypgen` and in each row of
`ein test --json-report`, and `ein test -v`'s line beside a claim. What is **not ruled** is a reading taken with it:
**no schema id moves** — `ein-summary/2`, `ein-region/1` and `ein-test-report/5` lose the key and keep their ids
(the default), or the report's id moved, as it moved on each change of its row before. Since the first look the
report's id has moved for another reason, `ein-test-report/6` for `ein test --diff`'s key (Q-M1f.462), so the
reading's (b) now reads `ein-test-report/7`. The user's earlier question, *why does the kernel read `is-a` at all*, is
answered since by the ruling that the kernel reads no theory's name (Q-M1f.436, Q-M1f.421): § 2.4 shows what is left.

Recorded at [`open_questions.md` § Q-M1f.412](../../../open_questions.md#q-m1f412--the-read-outs-is-a-row), with
[`findings.md` § V-6](../../../p1f.21_verdicts_as_rules/findings.md#v-6--the-read-out-reads-is-a-by-its-name).

## 2. What is so today

Two binaries: **before**, `build/review-bin/ein-0a4a97a3` (with `EIN_STDLIB=build/wt-rev-0a4/stdlib`), which still
prints the row; **today**, `build/review-bin/ein-head-17085b27`, every P1f.23 stage merged. [`rerun.sh`](rerun.sh)
makes every file below. The program is
[`programs/likes.ein`](programs/likes.ein), the claim program of today's test
`is_a_reads_as_any_other_relation_on_every_surface` (`ein-cli/tests/read_out.rs`): `(is-a Ann Person)`, a
declared `likes T T`, so the blind rung proposes `(likes Ann Person)` and `(likes Person Ann)` (4 states); and a
claim on the maximal line. Every command runs from `programs/`, so the paths the outputs echo are the same for both.

### 2.1 Before and today, every surface

[`runs/before-vs-today.diff`](runs/before-vs-today.diff) is `diff -ru` of
[`runs/before-0a4a97a3/`](runs/before-0a4a97a3/) and [`runs/today-17085b27/`](runs/today-17085b27/). What the row
was, on each surface:

| surface | before (0a4a97a3) | today |
|---|---|---|
| the read-out, all 7 questions (describe, `--maximal`, `--minimal`, `--single`, `-n 1`, `--count`, `--monotone`) and `ein oracle` | `    types           Person — on the right of an is-a, and 2 candidates of likes name it as an object: the blind enumerator is untyped, and no signature names it` | no line ([`runs/before-0a4a97a3/likes.describe.readout.txt`](runs/before-0a4a97a3/likes.describe.readout.txt) against [`runs/today-17085b27/likes.describe.readout.txt`](runs/today-17085b27/likes.describe.readout.txt)) |
| `--json-summary` (`ein-summary/2`), and `--dump-answer`'s `answer.json`, the same document | `root.hypgen.types_as_objects = {"objects": ["Person"], "relations": ["likes"], "candidates": 2}` | no key |
| `ein oracle --dump`'s `region.json` (`ein-region/1`) | `types_as_objects`, the same object | no key |
| `ein oracle --dump`'s `region.md` | `**Types as objects**: Person — on the right of an is-a, …` | no line |
| `ein test -v` | `    a type as an object: Person — on the right of an is-a, …` under the claim | no line |
| `ein test --json-report` | each row's `types_as_objects`, `ein-test-report/5` | no key; `ein-test-report/6`, each row with `diff` (`null` without `--diff`), the key `ein test --diff` added (Q-M1f.462) |

The summary's and the region dump's ids are the same in both (`ein-summary/2`, `ein-region/1`); the report's moved,
`/5` to `/6`, for the diff and not for the dropped key: at the first look, with `ein-head-b5fb8d70`, it was `/5` in
both. On a program with no `is-a`, the before binary wrote the key **empty** (`{"objects": [], "relations": [], "candidates": 0}`), so a summary
always had it: [`runs/before-0a4a97a3-is-b/likes.describe.summary.json`](runs/before-0a4a97a3-is-b/likes.describe.summary.json).

**What else the diff holds** (not this question's; between the two binaries the save/load work was merged,
Q-M1f.400 to .403, and `ein test --diff`, Q-M1f.454 to .462): `--json-summary` gains `root.commit` with its id kept
(`ein-summary/2`, a key added, Q-M1f.259's reading); `--dump-answer`'s `index.json` moves from `ein-answer/2` to
`ein-answer/3` and gains `commit` per state, and its state files change format; the report gains `diff` on each row,
`ein-test-report/6`.

The same drop on [`programs/pets.ein`](programs/pets.ein) (a copy of
`examples/saturation/type-exclusivity/pets.ein`, 11 339 states): [`runs/pets.before-vs-today.readout.diff`](runs/pets.before-vs-today.readout.diff),
one line, `types Pet — … 9 candidates of co-located, is-a name it as an object …`.

### 2.2 Today, `is-a` reads as any other relation

[`programs/likes-is-b.ein`](programs/likes-is-b.ein) is `likes.ein` with `is-a` spelt `is-b` everywhere (`sed`, the
comment included). Each binary ran on it from a directory of its own under the name `likes.ein`; its outputs, `is-b`
spelt back `is-a`, are diffed against `likes.ein`'s:

- **today**: [`runs/today-17085b27.is-a-vs-is-b.diff`](runs/today-17085b27.is-a-vs-is-b.diff) is **empty** — every
  read-out, summary, region dump, test line and report is the same name for name (wall-clock lines ignored);
- **before**: [`runs/before-0a4a97a3.is-a-vs-is-b.diff`](runs/before-0a4a97a3.is-a-vs-is-b.diff), 111 lines in 21
  files: the row and the key's contents, present for `is-a` alone.

### 2.3 What a reader keyed on the dropped key sees

[`reader/types_reader.py`](reader/types_reader.py) is a reader written against the three ids as they were: it
checks the id it knows, then reads `types_as_objects` where each document put it. Its output
([`reader/types_reader.out.txt`](reader/types_reader.out.txt)):

<!-- output -->
```text
## STRICT, today-17085b27
today-17085b27/likes.describe.summary.json: schema ein-summary/2 — an id this reader knows
    KeyError: 'types_as_objects' — the id is one this reader knows, and the key it was written for is not there
today-17085b27/likes.oracle-dump/region.json: schema ein-region/1 — an id this reader knows
    KeyError: 'types_as_objects' — …
today-17085b27/likes.test-report.json: schema ein-test-report/6 — UNKNOWN id: refused
## LENIENT, today-17085b27
today-17085b27/likes.describe.summary.json: schema ein-summary/2 — an id this reader knows
    types_as_objects.objects = [], candidates = 0
```

A strict reader fails on a document whose id it accepts; a lenient one (`.get` with a default) reads *no type bound
as an object*, which is what the before binary wrote for a program with no `is-a` and is false for `likes.ein`. The
report it refuses by its id today, `/6`, which it was not written for: the diff's move, not this question's, protects
this one reader of the report from the dropped key. `jq`
gives `null`, and `jq -r '….objects[]'` exits 5, *Cannot iterate over null*
([`reader/jq.out.txt`](reader/jq.out.txt)). Before, both readers print `['Person'], candidates = 2`.

### 2.4 What still reads `is-a` by its name: nothing

At the first look five places of the engine read `is-a` by its spelling outside `ein-infer`: the KB view's type boxes,
instance ovals and dashed empty-headed edge, the program view's same edge, `--models grid`'s rows, `program_relations`
(a goal over `is-a` loaded in a program that names it nowhere), and two refusals that suggested `(is-a ?y T)`. Each
went with the phase's merged work: the drawings with Q-M1f.421 (no relation drawn otherwise than another, every atom
a box), `--models` whole with S1f.23.2 (`grid.rs` deleted), `program_relations` and the refusals' text with
S1f.23.4 (Q-M1f.436). [`runs/is-a-readers.txt`](runs/is-a-readers.txt), the same `grep` today: what names `is-a` is
doc comments, unit tests' programs and a doc link, and no reader.

| where it read `is-a` at the first look | today | shown by |
|---|---|---|
| `ein render kb`: a type box, an instance oval, a dashed edge with an empty head | a fact of `is-a` is an octagon whose arrow `1` is at the box `is-a`, coloured as any relation; spelt `is-b`, the same drawing name for name | [`runs/pets.kb.is-a-vs-is-b.diff`](runs/pets.kb.is-a-vs-is-b.diff), [`runs/likes.kb.is-a-vs-is-b.diff`](runs/likes.kb.is-a-vs-is-b.diff), [`runs/derived-is-a.kb.is-a-vs-is-b.diff`](runs/derived-is-a.kb.is-a-vs-is-b.diff): **empty**, a fact's node id (a hash of its text) and a relation's colour (picked by its name) put aside |
| `ein render program`'s ontology digraph: the same edge | no ontology digraph; the program's `kb` drawn as the KB view draws one | [`renders/pets.program.01-kb.svg`](renders/pets.program.01-kb.svg), [`renders/pets-is-b.program.01-kb.svg`](renders/pets-is-b.program.01-kb.svg) |
| `ein solve --models grid`: rows read off `is-a` | `--models` is removed (Q-M1f.448, Q-M1f.449): clap refuses it, exit 2, so the item's two `--models grid` runs and their programs `grid-rows.ein` and `grid-rows-is-b.ein` are dropped | — |
| `program_relations`: a goal over `is-a` loads in a program that names none | refused, as `is-b` is | [`runs/goal-is-a-unnamed.solve.txt`](runs/goal-is-a-unnamed.solve.txt) and [`runs/goal-is-b-unnamed.solve.txt`](runs/goal-is-b-unnamed.solve.txt), both `exit 1`; the before binary loaded the `is-a` one ([`runs/goal-is-a-unnamed.before-0a4a97a3.solve.txt`](runs/goal-is-a-unnamed.before-0a4a97a3.solve.txt)) |
| two refusals suggesting `(is-a ?y T)` | they suggest a premise of any relation, `(R ?y)`, or `(eq ?y ?y)` | read, not run (P1f.23's R-2) |
| `ein render kb --no-types` | removed with the convention (Q-M1f.421) | — |

<!-- output -->
```text
kb load error: programs/goal-is-a-unnamed.ein:8:15: query: its :goal writes (is-a ?x ?y), and the program names no relation is-a — a relation of the program is one it declares, states a fact of, or writes in a rule or an hrule, so no state holds a fact of is-a
exit 1
```

**The drawing** of `examples/saturation/type-exclusivity/pets.ein`'s root, today
([`renders/pets.kb.dot`](renders/pets.kb.dot)): every fact of `is-a` an octagon, `Pet` and `Dog` boxes like every atom.

![ein render kb of pets.ein: every fact of is-a an octagon whose arrow 1 is at the box is-a, no box or oval of its own](renders/pets.kb.svg)

A derived `is-a` is drawn as any derived fact, dashed with `by dogs-are-pets` beside it
([`renders/derived-is-a.kb.svg`](renders/derived-is-a.kb.svg)):

![ein render kb of derived-is-a.ein: the derived (is-a Rex Pet) dashed, by dogs-are-pets, as the derived (likes Rex Ball) is](renders/derived-is-a.kb.svg)

## 3. The options

- **(a) No schema id moves for the dropped key** (today's build). A key dropped moves no other key; Q-M1f.259 keeps
  an id where keys are added. What a reader sees is § 2.3: a strict reader of `ein-summary/2` or `ein-region/1`
  fails on the missing key; a lenient one reads an empty value. The report is `ein-test-report/6` today for the
  diff's key, which a reader written against `/5` refuses by its id.
- **(b) The report's id moves for it too, `ein-test-report/7`** (the ledger's second reading, `/6` when it was
  written; P1f.23's R-10): the report's id has moved on each change of its row before — it became `/4` when every
  row gained `outside` and `types_as_objects` (commit `c239754c`), `/5` when rows gained `route` and
  `fell_back_from`, `/6` when they gained `diff`. The summary and the region dump keep theirs. A **mock**:
  [`runs/mocks/likes.test-report.MOCK.b-report-7.json`](runs/mocks/likes.test-report.MOCK.b-report-7.json), today's
  report with its `schema` line edited and nothing else ([`runs/mocks/mocks-vs-today.diff`](runs/mocks/mocks-vs-today.diff)).
  The reader on it refuses the report by its id, as it refuses today's, and still fails on the summary and the
  region ([`reader/types_reader.on-mocks.out.txt`](reader/types_reader.on-mocks.out.txt)).
- **(c) Every id bumped**: `ein-summary/3`, `ein-region/2`, `ein-test-report/7`. **Mocks**, the `schema` line
  edited: [`runs/mocks/likes.describe.summary.MOCK.c-summary-3.json`](runs/mocks/likes.describe.summary.MOCK.c-summary-3.json),
  [`runs/mocks/region.MOCK.c-region-2.json`](runs/mocks/region.MOCK.c-region-2.json),
  [`runs/mocks/likes.test-report.MOCK.c-report-7.json`](runs/mocks/likes.test-report.MOCK.c-report-7.json). The reader
  refuses all three by id, before reading a key. The pages name each id
  (`05-cli.md`, `06-config.md`, `08-read-out.md` § 11, `10-answer-dump.md`, `11-region-dump.md`), and tests pin them
  (`test_cli.rs`' `the_pages_name_the_reports_schema` for the report; `summary_properties.rs` and `region_answer.rs` name
  `ein-summary/2`, `corpus_cli.rs` `ein-region/1`; read, not run), so (b) or (c) is a change to each.

## 4. The default

**(a)**, from the ledger: *"no schema id moves … as Q-M1f.259 keeps an id where keys are added and no other key
moves"*. Built in `5a3f2979`. The ledger's Q-M1f.462 notes that master's report was still `/5` when S1f.23.3 merged,
so `/6` is the diff's alone, and a revision of this reading to (b) would make it `/7`.

## 5. Files

- [`rerun.sh`](rerun.sh) — every run, diff, mock, reader output and drawing below.

Programs:
- [`programs/likes.ein`](programs/likes.ein) — the test's claim program, with a comment; the one every surface runs on.
- [`programs/likes-is-b.ein`](programs/likes-is-b.ein) — `likes.ein` with `is-a` spelt `is-b`, comment included.
- [`programs/pets.ein`](programs/pets.ein) — copy of `examples/saturation/type-exclusivity/pets.ein`; [`programs/pets-is-b.ein`](programs/pets-is-b.ein), spelt `is-b`.
- [`programs/derived-is-a.ein`](programs/derived-is-a.ein), [`programs/derived-is-b.ein`](programs/derived-is-b.ein) — a written and a derived `is-a` fact.
- [`programs/goal-is-a-unnamed.ein`](programs/goal-is-a-unnamed.ein), [`programs/goal-is-b-unnamed.ein`](programs/goal-is-b-unnamed.ein) — a goal over a relation the program names nowhere.

Runs:
- [`runs/before-0a4a97a3/`](runs/before-0a4a97a3/), [`runs/today-17085b27/`](runs/today-17085b27/) — `likes.ein` under each binary: `likes.QUESTION.readout.txt` and `likes.QUESTION.summary.json` for `describe`, `q-maximal`, `q-minimal`, `q-single`, `q-n1`, `q-count`, `q-monotone`; `likes.dump-answer/` (`--dump-answer`); `likes.oracle.txt` and `likes.oracle-dump/` (`region.json`, `region.md`); `likes.test.txt` (`ein test -v`) and `likes.test-report.json` (`--json-report`).
- [`runs/before-0a4a97a3-is-b/`](runs/before-0a4a97a3-is-b/), [`runs/today-17085b27-is-b/`](runs/today-17085b27-is-b/) — the same for `likes-is-b.ein`.
- [`runs/before-vs-today.diff`](runs/before-vs-today.diff) — the two binaries on `likes.ein`, every file.
- [`runs/before-0a4a97a3.is-a-vs-is-b.diff`](runs/before-0a4a97a3.is-a-vs-is-b.diff), [`runs/today-17085b27.is-a-vs-is-b.diff`](runs/today-17085b27.is-a-vs-is-b.diff) (empty) — the rename, per binary.
- [`runs/pets.before-0a4a97a3.readout.txt`](runs/pets.before-0a4a97a3.readout.txt), [`runs/pets.today-17085b27.readout.txt`](runs/pets.today-17085b27.readout.txt), [`runs/pets.before-vs-today.readout.diff`](runs/pets.before-vs-today.readout.diff).
- [`runs/mocks/`](runs/mocks/) — the schema-id mocks and [`mocks-vs-today.diff`](runs/mocks/mocks-vs-today.diff).
- [`runs/is-a-readers.txt`](runs/is-a-readers.txt) — every place outside `ein-infer` that names `is-a`, with its line.
- [`runs/goal-is-a-unnamed.solve.txt`](runs/goal-is-a-unnamed.solve.txt), [`runs/goal-is-b-unnamed.solve.txt`](runs/goal-is-b-unnamed.solve.txt), and the before binary's [`goal-is-a-unnamed`](runs/goal-is-a-unnamed.before-0a4a97a3.solve.txt) and [`goal-is-b-unnamed`](runs/goal-is-b-unnamed.before-0a4a97a3.solve.txt).
- [`runs/pets.kb.is-a-vs-is-b.diff`](runs/pets.kb.is-a-vs-is-b.diff), [`runs/likes.kb.is-a-vs-is-b.diff`](runs/likes.kb.is-a-vs-is-b.diff), [`runs/derived-is-a.kb.is-a-vs-is-b.diff`](runs/derived-is-a.kb.is-a-vs-is-b.diff) — the KB views, `is-a` against `is-b`, node ids and colours put aside: empty.
- [`runs/draw-times.tsv`](runs/draw-times.tsv) — each drawing's layout and seconds.

Reader: [`reader/types_reader.py`](reader/types_reader.py), its output on today and before
([`types_reader.out.txt`](reader/types_reader.out.txt)) and on the mocks ([`types_reader.on-mocks.out.txt`](reader/types_reader.on-mocks.out.txt)),
and `jq`'s ([`jq.out.txt`](reader/jq.out.txt)).

Renders (today's binary; `.dot` as written, `.svg` drawn with the layout the DOT suggests, `fdp` for a knowledge base;
a program view's digraphs one file each):
[`pets.kb`](renders/pets.kb.svg), [`pets-is-b.kb`](renders/pets-is-b.kb.svg),
[`pets.program` kb](renders/pets.program.01-kb.svg), [rule](renders/pets.program.02-rule_type_exclusivity_lhs_rhs.svg) and [query](renders/pets.program.03-query.svg),
[`pets-is-b.program` kb](renders/pets-is-b.program.01-kb.svg),
[`likes.kb`](renders/likes.kb.svg), [`likes-is-b.kb`](renders/likes-is-b.kb.svg),
[`likes.program` kb](renders/likes.program.01-kb.svg), [`likes-is-b.program` kb](renders/likes-is-b.program.01-kb.svg),
[`derived-is-a.kb`](renders/derived-is-a.kb.svg), [`derived-is-b.kb`](renders/derived-is-b.kb.svg).

## 6. Re-run

```sh
bash plans/m1f_hypothesis_and_documentation/review/04-language-and-decisions/16-Q-M1f.412/rerun.sh
```

It runs every surface of `likes.ein` and `likes-is-b.ein` under both binaries from `programs/` (the `is-b` program
copied as `likes.ein` into a directory of its own), the diffs, `pets.ein`'s read-out under both, the two goal
programs, the `grep` of `is-a`, the reader and `jq`, the mocks (today's documents, the `schema` line edited), and the
KB and program views, drawn with [`../../03-drawings/tools/draw.sh`](../../03-drawings/tools/draw.sh).

## 7. Found while building

- **Re-taken with `ein-head-17085b27`** against the first look's `ein-head-b5fb8d70`: on `likes.ein` every read-out,
  summary, dump and test line is the same; the report reads `ein-test-report/6` and each row a `diff` key, `null`
  (Q-M1f.462). The question stands: whether dropping a key moves an id is still the user's, the report's `/6` being
  the diff's. **The page's second half is answered**: nothing reads `is-a` by its name (§ 2.4), and the two
  findings of the first look on it are gone with it: the KB view draws a derived `is-a` dashed with its rule, as
  any derived fact; `--models grid`'s rows are gone with `--models`.
- **A reader keyed on the ids now refuses the report** by `/6`, a move made for another key: under (a) a reader
  meets the dropped key only in the summary and the region dump.
- **`region.json` comes from `ein oracle --dump`, not `--dump-answer`**: `--dump-answer` writes `answer.json` (the
  summary) and `index.json`; both carried the key in the summary's place. The diff covers both commands.

## 8. Your call

- **(a)** no id moves for the dropped key: `ein-summary/2`, `ein-region/1` kept, the report `ein-test-report/6` for
  the diff alone — **default**, built.
- **(b)** the report's id moves for it too, `ein-test-report/7`; the summary and the region dump keep theirs.
- **(c)** every id moves: `ein-summary/3`, `ein-region/2`, `ein-test-report/7`.
