# Q-M1f.414 — a fact's English, which a state file used to write in a comment

## 1. The question

Until save/load was merged, `ein solve --dump-answer DIR` wrote each state of R as `DIR/models/NN.ein`: the program,
then **every fact of the state, each followed by a comment in English** — why it holds (its `:source`, or its rule's
`:why` filled) and what it says (its relation's `:why` filled). The saved file that replaced it (Q-M1f.400,
Q-M1f.401) holds the program and the choices made, `(commit …)`, and no fact of the state. So today **no file holds a
state's facts in English**. The question is where, if anywhere, a reader gets a state's facts in English now: (a) the
trace, as it is; (b) `ein saturate --dump` of a saved file, with a comment after each fact; (c) the `-p` listing of a
state, with a comment after each fact.

Recorded: [`open_questions.md` § Q-M1f.414](../../../open_questions.md#q-m1f414--a-facts-english-which-a-state-file-wrote-in-a-comment).
The decision it would amend: [`Q-M1f.58`](../../../../../docs/decisions/read-out/Q-M1f.58-a-fact-reads-in-english-in-the-trace.md).
Its sibling: [`02-L-102`](../02-L-102/README.md), the writer of those comments, which no command calls now.

The program used throughout is [`programs/walks.ein`](programs/walks.ein), written for this review: three facts
given with a `:source`, one background fact (`(dog Rex)`, no source), a rule `walk` that derives from the givens, an
hrule `guess` that supposes each person busy, a rule `tire` that derives from a commitment, and a refutation
`one-busy`. R is three states: ∅, {(busy Alice)}, {(busy Bob)} ([`runs/today.solve.txt`](runs/today.solve.txt)).

## 2. What is so today

**The earlier binary** (`ein-a8d595eb`, before save/load) wrote the state {(busy Alice)} as
[`runs/before-ein-a8d595eb.dump-answer/models/02.ein`](runs/before-ein-a8d595eb.dump-answer/models/02.ein): the
program, then this (output):

```text
;; ── the state ──────────────────────────────────────────────────────────

(busy Alice) ; hypothesis 0 — Alice is busy
(dog Rex) ; Rex is a dog
(owns Alice Rex) ; source "sentence 3" — Alice owns Rex
(person Alice) ; source "sentence 1" — Alice is a person
(person Bob) ; source "sentence 2" — Bob is a person
(tired Alice) ; rule tire: Alice is busy and walks Rex, so Alice is tired — Alice is tired
(walks Alice Rex) ; rule walk: Alice owns the dog Rex, so Alice walks Rex — Alice walks Rex
```

Every fact of the state but the program's `relation` facts, sorted, each with its English: 5, 7 and 6 lines over the
three states.

**Today's binary** (`ein-head-b5fb8d70`) writes the same state as
[`runs/today.dump-answer/models/02.ein`](runs/today.dump-answer/models/02.ein): the program as written inside
`(program …)`, then (output)

```text
)
(commit
  (busy Alice))
```

and no fact. The two directories side by side: [`runs/models.before-vs-today.diff`](runs/models.before-vs-today.diff).

**Where a fact reads in English today**, on the same program:

- **`--trace`** ([`runs/today.trace.md`](runs/today.trace.md)) writes each firing's reason in its step's quote and
  each fact it derives with its statement, and a given premise with its source (output):

  ```text
  ### Step 1 — `walk`

  > Alice owns the dog Rex, so Alice walks Rex

  Premises: `owns(Alice, Rex)`, `dog(Rex)` — from sentence 3

  Derives `walks(Alice, Rex)` — Alice walks Rex
  ```

  The earlier binary wrote the same trace byte for byte
  ([`runs/trace.before-vs-today.diff`](runs/trace.before-vs-today.diff) is empty): the trace did not change, the
  state file lost its comments. **What the trace does not write**: a fact no step derives or reads with a source, and
  every statement of a given — *Alice is a person*, *Bob is a person*, *Rex is a dog*, *Alice owns Rex* — and the
  commitment's own statement: a state says `Assuming **(busy Alice)**.`, an s-expression, never *Alice is busy*. Of
  the 18 commented lines the earlier state files wrote, the trace writes two facts' statements (`walks`, `tired`) and
  one source (*sentence 3*). Its two diagrams: [`renders/today.trace.01-root.svg`](renders/today.trace.01-root.svg),
  [`renders/today.trace.02-state-2-of-3.svg`](renders/today.trace.02-state-2-of-3.svg).
- **`ein render kb`** of the saved file labels a fact's arrow with its relation and its source or its rule
  (`person sentence 1`, `owns sentence 3`, `walks by walk`, `tired by tire`), and a commitment with its relation
  alone (`busy`): [`renders/today.render-kb.models-02.svg`](renders/today.render-kb.models-02.svg), and against root,
  the commitment and what it added drawn heavy:
  [`renders/today.render-kb.models-02-since-01.svg`](renders/today.render-kb.models-02-since-01.svg). A source and a
  rule's name, not a statement and not a reason.
- **`ein saturate --dump`** of the saved file prints each fact with its origin, `:source "sentence 3"`,
  `:rule tire`, `:hypothesis 0`, and no English
  ([`runs/today.saturate-dump.models-02.txt`](runs/today.saturate-dump.models-02.txt); the other two states,
  [`01`](runs/today.saturate-dump.models-01.txt), [`03`](runs/today.saturate-dump.models-03.txt); the program,
  [`runs/today.saturate-dump.program.txt`](runs/today.saturate-dump.program.txt)).
- **`ein solve -p`** lists each state's facts beyond the program as loaded (root's derivations, the commitment and
  what it derives), as s-expressions ([`runs/today.solve-p.txt`](runs/today.solve-p.txt)); with `--dump-answer` the
  same listing is [`runs/today.dump-answer-p/forms/print-final-state.txt`](runs/today.dump-answer-p/forms/print-final-state.txt).
  The givens are not in it.
- **No other surface** writes a fact's English: a grep for three of the texts over the read-out, `answer.txt`,
  `answer.json`, `index.json`, `--json-summary`, `--events`, `-p`, the dump and the KB view finds them in the trace
  alone ([`runs/today.where-the-english-is.grep.txt`](runs/today.where-the-english-is.grep.txt); the one hit in each
  saved file is the rule `walk`'s `:why` as the program writes it, a template, `{?p} owns the dog {?d} …`). The
  events as a table: [`runs/today.events.md`](runs/today.events.md).
- **The page**, `14-natural-language.md` § 3, showed a state file's lines and now shows a trace's
  ([`runs/14-natural-language.section-3.before-vs-today.diff`](runs/14-natural-language.section-3.before-vs-today.diff);
  the two sections whole: [`before`](runs/14-natural-language.section-3.before-a8d595eb.md),
  [`today`](runs/14-natural-language.section-3.today.md)). Its table's last column moved from *the state file
  writes* to *the trace writes*, and its hypothesis row from `; hypothesis N — <statement>` to `Assuming **(R a b)**.`.

## 3. The options

**(a) The English goes with the facts: the trace.** Nothing changes; what § 2 shows is the option. A reader who
wants a state's facts in English reads `--trace`, which writes the derivation, not the state: the givens' statements
and the commitment's are not written. **The default.**

**(b) `ein saturate --dump` writes each fact's English in a comment after it**, as the state file did. A code change
(the dump's printer calls the writer [`02-L-102`](../02-L-102/README.md) shows). Mock:
[`runs/today.saturate-dump.models-02.MOCK.option-b.txt`](runs/today.saturate-dump.models-02.MOCK.option-b.txt),
today's dump with each fact's comment appended, the comment copied verbatim from the earlier binary's state file of
the same state; the diff: [`…MOCK.option-b.diff`](runs/today.saturate-dump.models-02.MOCK.option-b.diff) (output):

```text
;; ── GIVEN (3 facts) ──
  (owns Alice Rex :source "sentence 3") ; source "sentence 3" — Alice owns Rex
;;   person (2)
  (person Alice :source "sentence 1") ; source "sentence 1" — Alice is a person
  (person Bob :source "sentence 2") ; source "sentence 2" — Bob is a person

;; ── DERIVED (3 facts) ──
  (busy Alice :hypothesis 0) ; hypothesis 0 — Alice is busy
  (tired Alice :rule tire) ; rule tire: Alice is busy and walks Rex, so Alice is tired — Alice is tired
  (walks Alice Rex :rule walk) ; rule walk: Alice owns the dog Rex, so Alice walks Rex — Alice walks Rex
```

What was edited: a comment after each fact line of the KB section, nothing else. The `relation` facts get none (the
kernel's `relation` has no `:why`, and the writer writes no comment for a fact with no text). The comment repeats the
origin the dump already prints (`:source "sentence 3"` and `; source "sentence 3"`; `:hypothesis 0` and
`; hypothesis 0`, the number [`30-L-104`](../30-L-104/README.md) is about). It covers every fact of the state,
givens included, as the state file did, on stdout and only where asked.

**(c) A listing of a state, `-p`'s, writes each fact's English.** A code change in the `-p` printer. Mock:
[`runs/today.solve-p.MOCK.option-c.txt`](runs/today.solve-p.MOCK.option-c.txt), diff
[`…MOCK.option-c.diff`](runs/today.solve-p.MOCK.option-c.diff) (output):

```text
── state 2 of 3 ──
final-state facts (beyond the program as loaded: root's derivations, the commitment and what it derives; 3 facts):
  (busy Alice) ; hypothesis 0 — Alice is busy
  (tired Alice) ; rule tire: Alice is busy and walks Rex, so Alice is tired — Alice is tired
  (walks Alice Rex) ; rule walk: Alice owns the dog Rex, so Alice walks Rex — Alice walks Rex
```

What was edited: a comment after each listed fact, copied from the earlier state file. `-p` lists only the facts
beyond the program as loaded, so the givens' statements are not in it either; it shares stdout's 100 lines with the
read-out, and `--dump-answer DIR` writes it whole as `forms/print-final-state.txt`, so with `-p` the English would be
in a file of the dump again.

## 4. The default

**(a)**, from the ledger: *the trace is where a fact reads in English, Q-M1f.58 says so, and
`14-natural-language.md` § 3 shows a trace's lines instead of a state file's.* Built: the page, the decision, and
`ein-cli/tests/natural_language_page.rs`' `the_pages_trace_lines_are_a_runs`. Under (a) the writer of the comments
has no caller in the binary ([`02-L-102`](../02-L-102/README.md)) and the census that read them counts nothing
([`03-L-105`](../03-L-105/README.md)).

## 5. Files

- [`programs/walks.ein`](programs/walks.ein) — the program, written for this review.
- [`runs/today.solve.txt`](runs/today.solve.txt) — today's `ein solve`, the read-out.
- [`runs/today.dump-answer/`](runs/today.dump-answer/) — today's `--dump-answer`: `models/01.ein` to `03.ein`, the saved files; `answer.txt`, `answer.json`, `index.json`, `nogoods.json`.
- [`runs/before-ein-a8d595eb.solve.txt`](runs/before-ein-a8d595eb.solve.txt) — the earlier binary's read-out.
- [`runs/before-ein-a8d595eb.dump-answer/`](runs/before-ein-a8d595eb.dump-answer/) — the earlier binary's `--dump-answer`: the state files with each fact's comment.
- [`runs/models.before-vs-today.diff`](runs/models.before-vs-today.diff) — the two `models/` directories, `diff -ru`.
- [`runs/today.trace.md`](runs/today.trace.md) — today's `--trace`.
- [`runs/before-ein-a8d595eb.trace.md`](runs/before-ein-a8d595eb.trace.md) — the earlier binary's `--trace`.
- [`runs/trace.before-vs-today.diff`](runs/trace.before-vs-today.diff) — their diff: empty, the same bytes.
- [`runs/today.saturate-dump.program.txt`](runs/today.saturate-dump.program.txt) — `ein saturate --dump` of the program.
- [`runs/today.saturate-dump.models-01.txt`](runs/today.saturate-dump.models-01.txt), [`-02`](runs/today.saturate-dump.models-02.txt), [`-03`](runs/today.saturate-dump.models-03.txt) — `ein saturate --dump` of each saved file.
- [`runs/today.saturate-dump.models-02.MOCK.option-b.txt`](runs/today.saturate-dump.models-02.MOCK.option-b.txt) and [`.diff`](runs/today.saturate-dump.models-02.MOCK.option-b.diff) — **MOCK** of (b).
- [`runs/today.solve-p.txt`](runs/today.solve-p.txt) — today's `ein solve -p`.
- [`runs/today.solve-p.MOCK.option-c.txt`](runs/today.solve-p.MOCK.option-c.txt) and [`.diff`](runs/today.solve-p.MOCK.option-c.diff) — **MOCK** of (c).
- [`runs/today.dump-answer-p/`](runs/today.dump-answer-p/) — `ein solve -p --dump-answer`: the same saved files and `forms/print-final-state.txt`.
- [`runs/today.json-summary.json`](runs/today.json-summary.json) — `--json-summary`.
- [`runs/today.events.jsonl`](runs/today.events.jsonl) and [`runs/today.events.md`](runs/today.events.md) — `--events`, raw and as a table.
- [`runs/today.where-the-english-is.grep.txt`](runs/today.where-the-english-is.grep.txt) — which surface holds three of the texts.
- [`runs/today.render-kb.models-02.stderr.txt`](runs/today.render-kb.models-02.stderr.txt) — `ein render kb`'s stderr (empty).
- [`runs/14-natural-language.section-3.before-a8d595eb.md`](runs/14-natural-language.section-3.before-a8d595eb.md), [`…today.md`](runs/14-natural-language.section-3.today.md), [`…before-vs-today.diff`](runs/14-natural-language.section-3.before-vs-today.diff) — the page's § 3 at the two commits.
- [`renders/today.render-kb.models-02.dot`](renders/today.render-kb.models-02.dot) / [`.svg`](renders/today.render-kb.models-02.svg) — `ein render kb` of the state {(busy Alice)} (fdp).
- [`renders/today.render-kb.models-02-since-01.dot`](renders/today.render-kb.models-02-since-01.dot) / [`.svg`](renders/today.render-kb.models-02-since-01.svg) — the same, `--since` root.
- [`renders/today.trace.01-root.dot`](renders/today.trace.01-root.dot) / [`.svg`](renders/today.trace.01-root.svg), [`renders/today.trace.02-state-2-of-3.dot`](renders/today.trace.02-state-2-of-3.dot) / [`.svg`](renders/today.trace.02-state-2-of-3.svg) — the trace's two inline diagrams, drawn.
- [`tools/mock01.py`](tools/mock01.py) — writes the two mocks from today's outputs and the earlier state files.
- [`tools/jsonl_table.py`](tools/jsonl_table.py) — the events table.

## 6. Re-run

From the repo root:

```sh
D=plans/m1f_hypothesis_and_documentation/review/01-save-and-load/01-Q-M1f.414
H=build/review-bin/ein-head-b5fb8d70
A=build/review-bin/ein-a8d595eb; export EA=$PWD/build/wt-rev-a8d/stdlib
EIN_STDLIB=$EA $A solve $D/programs/walks.ein --dump-answer $D/runs/before-ein-a8d595eb.dump-answer > $D/runs/before-ein-a8d595eb.solve.txt
EIN_STDLIB=$EA $A solve $D/programs/walks.ein --trace $D/runs/before-ein-a8d595eb.trace.md
$H solve $D/programs/walks.ein --dump-answer $D/runs/today.dump-answer > $D/runs/today.solve.txt
$H solve $D/programs/walks.ein --trace $D/runs/today.trace.md
$H solve $D/programs/walks.ein -p > $D/runs/today.solve-p.txt
$H solve $D/programs/walks.ein -p --dump-answer $D/runs/today.dump-answer-p
$H solve $D/programs/walks.ein --json-summary $D/runs/today.json-summary.json --events $D/runs/today.events.jsonl
$H saturate $D/programs/walks.ein --dump > $D/runs/today.saturate-dump.program.txt
for i in 01 02 03; do $H saturate $D/runs/today.dump-answer/models/$i.ein --dump > $D/runs/today.saturate-dump.models-$i.txt; done
$H render kb $D/runs/today.dump-answer/models/02.ein > $D/renders/today.render-kb.models-02.dot
$H render kb $D/runs/today.dump-answer/models/02.ein --since $D/runs/today.dump-answer/models/01.ein > $D/renders/today.render-kb.models-02-since-01.dot
for d in $D/renders/today.render-kb.*.dot; do fdp -Tsvg $d > ${d%.dot}.svg; done
for d in $D/renders/today.trace.*.dot; do dot -Tsvg $d > ${d%.dot}.svg; done
python3 -I $D/tools/mock01.py $D
```

## 7. Found while building

- **The trace's premises line puts the step's sources after all its premises**, not after the one each belongs to:
  `` Premises: `owns(Alice, Rex)`, `dog(Rex)` — from sentence 3 `` where only `owns` has a source
  (`ein-render/src/trace/render.rs`' `premises_line`, which joins `step.sources`). The page's table writes the
  one-premise case, `` `R(a, b)` — from <source> ``. Not in the ledger.

## 8. Your call

- **(a)** the trace is where a fact reads in English; no surface lists a state's facts in English — **the default**.
- **(b)** `ein saturate --dump` of a saved file writes each fact's English in a comment, every fact of the state.
- **(c)** `-p` writes each listed fact's English: the facts beyond the program, not the givens.
