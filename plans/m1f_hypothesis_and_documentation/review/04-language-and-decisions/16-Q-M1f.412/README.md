# Q-M1f.412 — the read-out's `is-a` row is dropped: does a schema id move?

## 1. The question

The read-out had a `types` row under `universe`: the atoms on the right of an `is-a` fact that the blind rung's
candidates have as arguments. It read the relation `is-a` by its spelling and decided nothing. On the user's ruling
(*"V-6 (a), drop the is-a row"*) it was dropped, on every surface: the row on every question, `region.md`'s line,
the key `types_as_objects` in `region.json`, in `--json-summary`'s `root.hypgen` and in each row of
`ein test --json-report`, and `ein test -v`'s line beside a claim. What is **not ruled** is a reading taken with it:
**no schema id moves** — `ein-summary/2`, `ein-region/1` and `ein-test-report/5` lose the key and keep their ids
(the default), or `ein-test-report/6`, as the report's id moved on each change of its row before. This page also
lists what still reads `is-a` by its name, the rest of the user's earlier question *why does the kernel read `is-a`
at all*.

Recorded at [`open_questions.md` § Q-M1f.412](../../../open_questions.md#q-m1f412--the-read-outs-is-a-row), with
[`findings.md` § V-6](../../../p1f.21_verdicts_as_rules/findings.md#v-6--the-read-out-reads-is-a-by-its-name).

## 2. What is so today

Two binaries: **before**, `build/review-bin/ein-0a4a97a3` (with `EIN_STDLIB=build/wt-rev-0a4/stdlib`), which still
prints the row; **today**, `build/review-bin/ein-head-b5fb8d70`. The program is
[`programs/likes.ein`](programs/likes.ein), the claim program of today's test
`is_a_reads_as_any_other_relation_on_every_surface` (`ein-cli/tests/read_out.rs`): `(is-a Ann Person)`, a
declared `likes T T`, so the blind rung proposes `(likes Ann Person)` and `(likes Person Ann)` (4 states); and a
claim on the maximal line. Every command runs from `programs/`, so the paths the outputs echo are the same for both.

### 2.1 Before and today, every surface

[`runs/before-vs-today.diff`](runs/before-vs-today.diff) is `diff -ru` of
[`runs/before-0a4a97a3/`](runs/before-0a4a97a3/) and [`runs/today-b5fb8d70/`](runs/today-b5fb8d70/). What the row
was, on each surface:

| surface | before (0a4a97a3) | today |
|---|---|---|
| the read-out, all 7 questions (describe, `--maximal`, `--minimal`, `--single`, `-n 1`, `--count`, `--monotone`) and `ein oracle` | `    types           Person — on the right of an is-a, and 2 candidates of likes name it as an object: the blind enumerator is untyped, and no signature names it` | no line ([`runs/before-0a4a97a3/likes.describe.readout.txt`](runs/before-0a4a97a3/likes.describe.readout.txt) against [`runs/today-b5fb8d70/likes.describe.readout.txt`](runs/today-b5fb8d70/likes.describe.readout.txt)) |
| `--json-summary` (`ein-summary/2`), and `--dump-answer`'s `answer.json`, the same document | `root.hypgen.types_as_objects = {"objects": ["Person"], "relations": ["likes"], "candidates": 2}` | no key |
| `ein oracle --dump`'s `region.json` (`ein-region/1`) | `types_as_objects`, the same object | no key |
| `ein oracle --dump`'s `region.md` | `**Types as objects**: Person — on the right of an is-a, …` | no line |
| `ein test -v` | `    a type as an object: Person — on the right of an is-a, …` under the claim | no line |
| `ein test --json-report` (`ein-test-report/5`) | each row's `types_as_objects` | no key |

The schema ids are the same in both (`ein-summary/2`, `ein-region/1`, `ein-test-report/5`). On a program with no
`is-a`, the before binary wrote the key **empty** (`{"objects": [], "relations": [], "candidates": 0}`), so a summary
always had it: [`runs/before-0a4a97a3-is-b/likes.describe.summary.json`](runs/before-0a4a97a3-is-b/likes.describe.summary.json).

**What else the diff holds** (not this question's; between the two binaries the save/load work was merged,
Q-M1f.400 to .403): `--json-summary` gains `root.commit` with its id kept (`ein-summary/2`, a key added, Q-M1f.259's
reading); `--dump-answer`'s `index.json` moves from `ein-answer/2` to `ein-answer/3` and gains `commit` per state,
and its state files change format; `region.md`'s *Cost* line is wall-clock.

The same drop on [`programs/pets.ein`](programs/pets.ein) (a copy of
`examples/saturation/type-exclusivity/pets.ein`, 11 339 states): [`runs/pets.before-vs-today.readout.diff`](runs/pets.before-vs-today.readout.diff),
one line, `types Pet — … 9 candidates of co-located, is-a name it as an object …`.

### 2.2 Today, `is-a` reads as any other relation

[`programs/likes-is-b.ein`](programs/likes-is-b.ein) is `likes.ein` with `is-a` spelt `is-b` everywhere (`sed`, the
comment included). Each binary ran on it from a directory of its own under the name `likes.ein`; its outputs, `is-b`
spelt back `is-a`, are diffed against `likes.ein`'s:

- **today**: [`runs/today-b5fb8d70.is-a-vs-is-b.diff`](runs/today-b5fb8d70.is-a-vs-is-b.diff) is **empty** — every
  read-out, summary, region dump, test line and report is the same name for name (wall-clock lines ignored);
- **before**: [`runs/before-0a4a97a3.is-a-vs-is-b.diff`](runs/before-0a4a97a3.is-a-vs-is-b.diff), 111 lines in 21
  files: the row and the key's contents, present for `is-a` alone.

### 2.3 What a reader keyed on the dropped key sees

[`reader/types_reader.py`](reader/types_reader.py) is a reader written against the three ids as they were: it
checks the id it knows, then reads `types_as_objects` where each document put it. Its output
([`reader/types_reader.out.txt`](reader/types_reader.out.txt)):

<!-- output -->
```text
## STRICT, today-b5fb8d70
today-b5fb8d70/likes.describe.summary.json: schema ein-summary/2 — an id this reader knows
    KeyError: 'types_as_objects' — the id is one this reader knows, and the key it was written for is not there
today-b5fb8d70/likes.oracle-dump/region.json: schema ein-region/1 — an id this reader knows
    KeyError: 'types_as_objects' — …
today-b5fb8d70/likes.test-report.json: schema ein-test-report/5 — an id this reader knows
    KeyError: 'types_as_objects' — …
## LENIENT, today-b5fb8d70
today-b5fb8d70/likes.describe.summary.json: schema ein-summary/2 — an id this reader knows
    types_as_objects.objects = [], candidates = 0
```

A strict reader fails on a document whose id it accepts; a lenient one (`.get` with a default) reads *no type bound
as an object*, which is what the before binary wrote for a program with no `is-a` and is false for `likes.ein`. `jq`
gives `null`, and `jq -r '….objects[]'` exits 5, *Cannot iterate over null*
([`reader/jq.out.txt`](reader/jq.out.txt)). Before, both readers print `['Person'], candidates = 2`.

### 2.4 What still reads `is-a` by its name

[`runs/is-a-readers.txt`](runs/is-a-readers.txt) has every hit with its lines (`grep -rn 'is-a' ein.rs/crates/*/src`,
each read). No module of `ein-infer` reads it (its `src` names it in test fixtures and doc comments only). Outside it:

| where | what it does with `is-a` | shown by |
|---|---|---|
| `ein-render/src/kb_dot.rs:104–150, :346` (`schema_nodes`, `emit_is_a_edge`) | `ein render kb`: the right side of an `is-a` fact is a **type box**, a left side that is no type an **instance oval**, and the fact a **dashed edge with an empty arrowhead**, no colour and no rule | [`renders/pets.kb.svg`](renders/pets.kb.svg) against [`renders/pets-is-b.kb.svg`](renders/pets-is-b.kb.svg) |
| `ein-render/src/ir_dot.rs:354–372` | `ein render program`: a membership fact as the same dashed edge, with box and oval | [`renders/pets.program.2-ontology.svg`](renders/pets.program.2-ontology.svg) against [`renders/pets-is-b.program.2-ontology.svg`](renders/pets-is-b.program.2-ontology.svg) |
| `ein-render/src/grid.rs:34–41, :317–321` | `--models grid`: the anchor type's rows include every `(is-a X Anchor)` | [`programs/grid-rows.ein`](programs/grid-rows.ein): rows Ann, Bob, Cy ([run](runs/grid-rows.models-grid.txt)); spelt `is-b`, Ann alone ([run](runs/grid-rows-is-b.models-grid.txt)) |
| `ein-ir/src/from_ir.rs:716–717` (`program_relations`) | `is-a` is every program's relation: a goal or claim over it loads in a program that names it nowhere | [`programs/goal-is-a-unnamed.ein`](programs/goal-is-a-unnamed.ein) loads ([run](runs/goal-is-a-unnamed.solve.txt)); `is-b` is refused ([program](programs/goal-is-b-unnamed.ein), [run](runs/goal-is-b-unnamed.solve.txt)) |
| `ein-ir/src/from_ir.rs:1795, :1902` | a refusal's message suggests `(is-a ?y T)` as a premise that binds a variable | a message, read nothing |
| `ein-cli/src/cmdline.rs:313` | `ein render kb --no-types`' help: *"leave out the type boxes and their is-a edges"* | [`renders/pets.kb-no-types.svg`](renders/pets.kb-no-types.svg) |
| `ein-core/src/entities.rs:9`, `ein-render/src/constraints.rs:9` | doc comments | — |

**The drawing** ([`renders/pets.kb.dot`](renders/pets.kb.dot), drawn [`renders/pets.kb.svg`](renders/pets.kb.svg),
`fdp`):

<!-- output -->
```text
  // types
  "Pet" [shape=box, label="Pet"];
  // instances
  "Bird" [shape=oval, label="Bird"];
  …
  "Dog" -> "Pet" [style=dashed, arrowhead=empty, label="is-a"];
```

and with `is-b` ([`renders/pets-is-b.kb.dot`](renders/pets-is-b.kb.dot)) the three facts are ordinary coloured edges,
`"Dog" -> "Pet" [label="is-b", color="#bcbd22", …, style=solid]`, and no box or oval. The smaller pair
[`renders/likes.kb.svg`](renders/likes.kb.svg) / [`renders/likes-is-b.kb.svg`](renders/likes-is-b.kb.svg) shows the same.

## 3. The options

- **(a) No schema id moves** (today's build). A key dropped moves no other key; Q-M1f.259 keeps an id where keys are
  added. What a reader sees is § 2.3: a strict reader of `ein-summary/2`, `ein-region/1` or `ein-test-report/5`
  fails on the missing key; a lenient one reads an empty value.
- **(b) `ein-test-report/6`** (the ledger's second reading): the report's id has moved on each change of its row
  before — it became `/4` when every row gained `outside` and `types_as_objects` (commit `c239754c`), and `/5` when
  rows gained `route` and `fell_back_from`. The summary and the region dump keep theirs. A **mock**:
  [`runs/mocks/likes.test-report.MOCK.b-report-6.json`](runs/mocks/likes.test-report.MOCK.b-report-6.json), today's
  report with its `schema` line edited and nothing else ([`runs/mocks/mocks-vs-today.diff`](runs/mocks/mocks-vs-today.diff)).
  The reader on it refuses the report by its id and still fails on the summary and the region
  ([`reader/types_reader.on-mocks.out.txt`](reader/types_reader.on-mocks.out.txt)).
- **(c) Every id bumped**: `ein-summary/3`, `ein-region/2`, `ein-test-report/6`. **Mocks**, the `schema` line
  edited: [`runs/mocks/likes.describe.summary.MOCK.c-summary-3.json`](runs/mocks/likes.describe.summary.MOCK.c-summary-3.json),
  [`runs/mocks/region.MOCK.c-region-2.json`](runs/mocks/region.MOCK.c-region-2.json),
  [`runs/mocks/likes.test-report.MOCK.c-report-6.json`](runs/mocks/likes.test-report.MOCK.c-report-6.json). The reader
  refuses all three by id, before reading a key. The pages name each id
  (`05-cli.md`, `06-config.md`, `08-read-out.md` § 11, `10-answer-dump.md`, `11-region-dump.md`), and tests pin them
  (`test_cli.rs`' `the_pages_name_the_reports_schema` for the report; `summary_properties.rs` and `region_answer.rs` name
  `ein-summary/2`, `corpus_cli.rs` `ein-region/1`; read, not run), so (b) or (c) is a change to each.

## 4. The default

**(a)**, from the ledger: *"no schema id moves … as Q-M1f.259 keeps an id where keys are added and no other key
moves"*. Built in `5a3f2979`.

## 5. Files

Programs:
- [`programs/likes.ein`](programs/likes.ein) — the test's claim program, with a comment; the one every surface runs on.
- [`programs/likes-is-b.ein`](programs/likes-is-b.ein) — `likes.ein` with `is-a` spelt `is-b`, comment included.
- [`programs/pets.ein`](programs/pets.ein) — copy of `examples/saturation/type-exclusivity/pets.ein`; [`programs/pets-is-b.ein`](programs/pets-is-b.ein), spelt `is-b`.
- [`programs/derived-is-a.ein`](programs/derived-is-a.ein), [`programs/derived-is-b.ein`](programs/derived-is-b.ein) — a written and a derived `is-a` fact, for § 7.
- [`programs/grid-rows.ein`](programs/grid-rows.ein), [`programs/grid-rows-is-b.ein`](programs/grid-rows-is-b.ein) — `--models grid`'s rows.
- [`programs/goal-is-a-unnamed.ein`](programs/goal-is-a-unnamed.ein), [`programs/goal-is-b-unnamed.ein`](programs/goal-is-b-unnamed.ein) — a goal over a relation the program names nowhere.

Runs:
- [`runs/before-0a4a97a3/`](runs/before-0a4a97a3/), [`runs/today-b5fb8d70/`](runs/today-b5fb8d70/) — `likes.ein` under each binary: `likes.QUESTION.readout.txt` and `likes.QUESTION.summary.json` for `describe`, `q-maximal`, `q-minimal`, `q-single`, `q-n1`, `q-count`, `q-monotone`; `likes.dump-answer/` (`--dump-answer`); `likes.oracle.txt` and `likes.oracle-dump/` (`region.json`, `region.md`); `likes.test.txt` (`ein test -v`) and `likes.test-report.json` (`--json-report`).
- [`runs/before-0a4a97a3-is-b/`](runs/before-0a4a97a3-is-b/), [`runs/today-b5fb8d70-is-b/`](runs/today-b5fb8d70-is-b/) — the same for `likes-is-b.ein`.
- [`runs/before-vs-today.diff`](runs/before-vs-today.diff) — the two binaries on `likes.ein`, every file.
- [`runs/before-0a4a97a3.is-a-vs-is-b.diff`](runs/before-0a4a97a3.is-a-vs-is-b.diff), [`runs/today-b5fb8d70.is-a-vs-is-b.diff`](runs/today-b5fb8d70.is-a-vs-is-b.diff) (empty) — the rename, per binary.
- [`runs/pets.before-0a4a97a3.readout.txt`](runs/pets.before-0a4a97a3.readout.txt), [`runs/pets.today-b5fb8d70.readout.txt`](runs/pets.today-b5fb8d70.readout.txt), [`runs/pets.before-vs-today.readout.diff`](runs/pets.before-vs-today.readout.diff).
- [`runs/mocks/`](runs/mocks/) — the schema-id mocks and [`mocks-vs-today.diff`](runs/mocks/mocks-vs-today.diff).
- [`runs/is-a-readers.txt`](runs/is-a-readers.txt) — every place outside `ein-infer` that names `is-a`, with its lines.
- [`runs/grid-rows.models-grid.txt`](runs/grid-rows.models-grid.txt), [`runs/grid-rows-is-b.models-grid.txt`](runs/grid-rows-is-b.models-grid.txt), [`runs/goal-is-a-unnamed.solve.txt`](runs/goal-is-a-unnamed.solve.txt), [`runs/goal-is-b-unnamed.solve.txt`](runs/goal-is-b-unnamed.solve.txt).

Reader: [`reader/types_reader.py`](reader/types_reader.py), its output on today and before
([`types_reader.out.txt`](reader/types_reader.out.txt)) and on the mocks ([`types_reader.on-mocks.out.txt`](reader/types_reader.on-mocks.out.txt)),
and `jq`'s ([`jq.out.txt`](reader/jq.out.txt)).

Renders (today's binary; `.dot` as written, `.svg` drawn, the KB views with `fdp`):
[`pets.kb`](renders/pets.kb.svg), [`pets-is-b.kb`](renders/pets-is-b.kb.svg), [`pets.kb-no-types`](renders/pets.kb-no-types.svg),
[`pets.program` ontology](renders/pets.program.2-ontology.svg) and [rule](renders/pets.program.1-rule_type_exclusivity_lhs_rhs.svg),
[`pets-is-b.program` ontology](renders/pets-is-b.program.2-ontology.svg),
[`likes.kb`](renders/likes.kb.svg), [`likes-is-b.kb`](renders/likes-is-b.kb.svg),
[`likes.program` ontology](renders/likes.program.1-ontology.svg), [`likes-is-b.program` ontology](renders/likes-is-b.program.1-ontology.svg),
[`derived-is-a.kb`](renders/derived-is-a.kb.svg), [`derived-is-b.kb`](renders/derived-is-b.kb.svg).

## 6. Re-run

```sh
OLD=build/review-bin/ein-0a4a97a3; NEW=build/review-bin/ein-head-b5fb8d70; D=plans/m1f_hypothesis_and_documentation/review/04-language-and-decisions/16-Q-M1f.412
cd $D/programs
for q in "" --maximal --minimal --single "-n 1" --count --monotone; do
  EIN_STDLIB=/home/user/work/ein/build/wt-rev-0a4/stdlib ../../../../../$OLD solve likes.ein $q --json-summary /tmp/old.json
  ../../../../../$NEW solve likes.ein $q --json-summary /tmp/new.json
done
EIN_STDLIB=/home/user/work/ein/build/wt-rev-0a4/stdlib ../../../../../$OLD oracle likes.ein --dump /tmp/old-cube
EIN_STDLIB=/home/user/work/ein/build/wt-rev-0a4/stdlib ../../../../../$OLD test -v likes.ein --json-report /tmp/old-report.json
../../../../../$NEW render kb pets.ein; ../../../../../$NEW render kb pets-is-b.ein
../../../../../$NEW solve grid-rows.ein --models grid
cd ..; python3 reader/types_reader.py STRICT $PWD/runs/today-b5fb8d70/likes.describe.summary.json $PWD/runs/today-b5fb8d70/likes.oracle-dump/region.json $PWD/runs/today-b5fb8d70/likes.test-report.json
```

## 7. Found while building

- **`region.json` comes from `ein oracle --dump`, not `--dump-answer`**: `--dump-answer` writes `answer.json` (the
  summary) and `index.json`; both carried the key in the summary's place. The diff covers both commands.
- **The KB view draws a derived `is-a` as a written one.** [`programs/derived-is-a.ein`](programs/derived-is-a.ein)
  writes `(is-a Rex Dog)` and derives `(is-a Rex Pet)` and `(likes Rex Ball)`: both `is-a` edges are
  `[style=dashed, arrowhead=empty, label="is-a"]`, with no *by dogs-are-pets* and no colour, while the derived
  `likes` edge reads `likes by dogs-are-pets` ([`renders/derived-is-a.kb.dot`](renders/derived-is-a.kb.dot)). Spelt
  `is-b`, the derived edge says its rule ([`renders/derived-is-b.kb.dot`](renders/derived-is-b.kb.dot)). Dashed means
  *derived* for every other relation in the same view.
- `--models grid` lists different rows for `is-a` and `is-b` (§ 2.4): a presentational read of the name, outside the
  rename test (which covers the read-out, the summary, the oracle and its dump, and `ein test`).

## 8. Your call

- **(a)** keep `ein-summary/2`, `ein-region/1`, `ein-test-report/5` (a key dropped moves no id) — **default**, built.
- **(b)** `ein-test-report/6`; the summary and the region dump keep theirs.
- **(c)** every id moves: `ein-summary/3`, `ein-region/2`, `ein-test-report/6`.
- And, separately, whether the renderers' `is-a` conventions (§ 2.4) stay: they decide no answer, and the user asked
  why anything reads `is-a`.
